//! End-to-end integration test for `jigc start --task <id>` resume — the inc-5
//! `as:` role-binding round-trip (`DECISIONS.md` 2026-05-31 → inc-5 `as:` role
//! binding at create; `worked-examples.md` → Superseding decision: the agent
//! re-composes to pick up context now that the ADR is bound to `task.decision`).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo: an intent
//! mints a task and composes. `jigc doc create adr --title "…"` binds the created
//! ADR to `task.decision` (records `.jigc/tasks/<id>/roles.json`). On resume,
//! `jigc start --task <id>` reads that binding back — the observable proof being
//! the persisted `roles.json` mapping `decision -> adr:<slug>`.
//!
//! The `superseded-context` step's `{{@task.decision.supersedes#decision}}` resolves
//! over the committed store + edge overlay: with no `supersedes` edge set on the
//! created ADR, that placeholder is **empty** (the absent-value contract —
//! `worked-examples.md` → Task 2: "in any task that creates no superseding edge…
//! the placeholder resolved to empty text"). The full slice-of-a-prior-committed-ADR
//! path is proven end-to-end in `superseding_decision.rs`.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init`, and a self-cleaning
//! `TempDir` keeps the test off the developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-start-resume-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves (composition mints, which reads HEAD).
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `git` command in `root`, asserting success.
fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Commit `path` with `body` — the intervening-commit primitive the base-pin arms
/// below advance HEAD with.
fn commit_file(root: &Path, path: &str, body: &str, message: &str) {
    let full = root.join(path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(&full, body).expect("write file");
    git(root, &["add", path]);
    git(root, &["commit", "-q", "-m", message]);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn resume_re_composes_with_the_created_adr_bound_to_task_decision() {
    let repo = TempDir::new("bind");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // 1. Mint + compose. `task.decision` is declared-but-unbound, so the
    //    superseded-context placeholder resolves to empty text — the address is
    //    nowhere in the composed view.
    let first = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "move the cache to redis",
        ],
    );
    assert!(
        first.status.success(),
        "the initial mint+compose must succeed; stderr:\n{}",
        String::from_utf8_lossy(&first.stderr),
    );
    let first_out = String::from_utf8(first.stdout).expect("utf-8 stdout");
    let slug = "move-the-cache-to-redis";
    assert!(
        !first_out.contains("> adr:"),
        "before any ADR is created, task.decision is unbound — no `> adr:` slice; got:\n{first_out}",
    );

    // 2. Create the ADR — the create-gate binds it to `task.decision`
    //    (roles.json records `decision -> adr:<slug>`).
    let create = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Shared Redis session cache",
        ],
    );
    assert!(
        create.status.success(),
        "`jigc doc create adr` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&create.stderr),
    );
    let adr_addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr_addr, "adr:shared-redis-session-cache");

    // The bind landed on disk: .jigc/tasks/<id>/roles.json maps decision -> the ADR.
    let roles_path = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(slug)
        .join("roles.json");
    let roles = fs::read_to_string(&roles_path).expect("roles.json written by the create-gate");
    assert!(
        roles.contains("\"decision\": \"adr:shared-redis-session-cache\""),
        "roles.json must bind decision -> the created ADR; got:\n{roles}",
    );

    // 3. Resume: `jigc start --task <id>` reads roles.json back. The created ADR is
    //    bound to `task.decision`, but it supersedes nothing — so the
    //    `superseded-context` slice (which dereferences `.supersedes`, not the bound
    //    role itself) resolves to empty text over the committed store + edge overlay.
    //    No `> adr:` blockquote: the bind is real (roles.json above), the edge is not.
    //    The full slice-of-a-committed-ADR path is proven in `superseding_decision.rs`.
    let resume = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        resume.status.success(),
        "`jigc start --task <id>` resume must succeed; stderr:\n{}",
        String::from_utf8_lossy(&resume.stderr),
    );
    let resume_out = String::from_utf8(resume.stdout).expect("utf-8 stdout");
    assert!(
        !resume_out.contains("> adr:"),
        "the bound ADR supersedes nothing — the superseded-context slice is empty (the \
         absent-value contract), not the bound role's own address; got:\n{resume_out}",
    );
}

/// The T3a done-criterion (`DECISIONS.md` 2026-06-01 → M2 Increment 3 re-cut):
/// resume composes the task's **own** minting workflow, not the cascade default.
/// A task minted via Form D `--workflow quick-fix` must re-compose **quick-fix**
/// on `jigc start --task <id>` — even though the cascade default is now the
/// `router` — proven by the absence of single-task's ADR/create affordance and
/// supersedes line in the resumed view. A working area whose recorded workflow id
/// is gone errors clearly rather than silently composing the default.
#[test]
fn resume_composes_the_tasks_own_minting_workflow_not_the_default() {
    let repo = TempDir::new("own-workflow");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Mint via Form D over quick-fix — NOT the cascade default (single-task).
    let mint = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "quick-fix", "fix the typo"],
    );
    assert!(
        mint.status.success(),
        "the Form-D quick-fix mint must succeed; stderr:\n{}",
        String::from_utf8_lossy(&mint.stderr),
    );
    let slug = "fix-the-typo";

    // Resume: it must re-compose quick-fix (the task's own workflow), so the view
    // carries neither single-task's `jigc doc create adr` affordance nor its
    // supersedes line — both of which a single-task default would emit.
    let resume = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        resume.status.success(),
        "resume of a quick-fix task must succeed; stderr:\n{}",
        String::from_utf8_lossy(&resume.stderr),
    );
    let resume_out = String::from_utf8(resume.stdout).expect("utf-8 stdout");
    assert!(
        !resume_out.contains("jigc doc create adr"),
        "resume must compose quick-fix (no ADR create affordance), not the single-task \
         default; got:\n{resume_out}",
    );
    assert!(
        !resume_out.contains("supersedes"),
        "resume must compose quick-fix (no supersedes line), not the single-task default; \
         got:\n{resume_out}",
    );

    // A working area whose recorded workflow id is gone errors clearly — never a
    // silent fall-through to the cascade default.
    fs::remove_file(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(slug)
            .join("workflow"),
    )
    .expect("remove the recorded workflow id");
    let orphan = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        !orphan.status.success(),
        "a task with no recorded workflow id must fail, not compose the default",
    );
    let orphan_err = String::from_utf8_lossy(&orphan.stderr);
    assert!(
        orphan_err.contains("workflow") && orphan_err.contains(slug),
        "the error must name the task and its missing workflow; got:\n{orphan_err}",
    );
}

// ── the base pin, overlap-aware (M47 Inc 8 T1 / N7) ─────────────────────────────────
//
// `jigc start --task <id>` used to blanket-refuse any `base != HEAD`, so the
// **read-only** resume door was stricter than the **commit** door, whose phase-1 guard
// has been overlap-aware since M17 (`design/finalize.md` → Parallel hand-editing, the
// 2026-06-12 amendment; `design/storage.md` → The per-task working area). Resume now
// makes the identical `engine::finalize::decide_base_repin` decision, so disjoint moved
// history re-pins in memory (`base.json` is never rewritten) and only an **overlap**
// with the task's footprint — dirty working-tree paths ∪ promote destinations — blocks.
//
// The change is **un-refusing**: every state it turns into a success previously refused,
// and no state that succeeded begins to refuse (pinned by the control arm below).

/// (a) **Disjoint** — an agent that commits work in a separate task and re-composes is
/// carried through instead of dead-ended, and the just-committed doc is **in view**:
/// resume exits 0, re-composes the task's own `implement-from-spec` workflow, and the
/// spec committed by the intervening commit is listed by `{{ store.specs }}` (which was
/// empty at mint — the committed store feeds from the **actual** HEAD, not the pin).
#[test]
fn resume_re_pins_over_disjoint_moved_history_and_sees_the_new_commit() {
    let repo = TempDir::new("disjoint");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let slug = "wire-the-cache";
    let mint = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "--slug",
            slug,
            "wire the cache",
        ],
    );
    assert!(
        mint.status.success(),
        "the mint must succeed; stderr:\n{}",
        String::from_utf8_lossy(&mint.stderr),
    );
    let mint_out = String::from_utf8(mint.stdout).expect("utf-8 stdout");
    assert!(
        !mint_out.contains("> spec:cache-layer"),
        "no spec is committed at mint — `store.specs` must be empty; got:\n{mint_out}",
    );

    // The intervening commit: a spec committed by other work. It touches nothing in the
    // task's footprint (the tree is clean; the task stages no doc), so the moved history
    // is **disjoint**.
    commit_file(
        repo.path(),
        "docs/specs/cache-layer.md",
        "# Cache layer\n",
        "spec: the cache layer",
    );

    let resume = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        resume.status.success(),
        "disjoint moved history must re-pin and resume, not dead-end; stderr:\n{}",
        String::from_utf8_lossy(&resume.stderr),
    );
    let resume_out = String::from_utf8(resume.stdout).expect("utf-8 stdout");
    assert!(
        resume_out.contains("> spec:cache-layer"),
        "the doc committed by the intervening commit must be in view; got:\n{resume_out}",
    );
    assert!(
        resume_out.contains("jigc task bind spec"),
        "resume must re-compose the task's own `implement-from-spec` workflow; \
         got:\n{resume_out}",
    );

    // The re-pin is in-memory only: `base.json` still records the ORIGINAL base (a
    // landed finalize is the only writer of durable task state).
    let pin = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(slug)
            .join("base.json"),
    )
    .expect("base.json readable");
    let head_short = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .current_dir(repo.path())
            .output()
            .expect("run git")
            .stdout,
    )
    .expect("utf-8");
    assert!(
        !pin.contains(head_short.trim()),
        "the re-pin is in-memory only — `base.json` must still record the original base; \
         got:\n{pin}",
    );
}

/// (b) **Overlap** — the moved history touches a path in the task's footprint (a dirty
/// working-tree path), so resume keeps the block, and the block is the engine's
/// `finalize.base-mismatch` finding: it **names the overlapping path** and routes at
/// resolving the overlap first, discarding only as the alternative (the old refusal
/// offered destructive exits only).
#[test]
fn resume_blocks_when_the_moved_history_overlaps_the_tasks_work() {
    let repo = TempDir::new("overlap");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let slug = "my-task";
    let mint = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "--slug",
            slug,
            "do the thing",
        ],
    );
    assert!(
        mint.status.success(),
        "the mint must succeed; stderr:\n{}",
        String::from_utf8_lossy(&mint.stderr),
    );

    // The moved history touches `notes.md`; the task's working tree carries an
    // uncommitted edit to the same path — the parallel-hand-editing case the pin exists
    // to catch.
    commit_file(repo.path(), "notes.md", "theirs\n", "second");
    fs::write(repo.path().join("notes.md"), "mine\n").expect("write file");

    let resume = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        !resume.status.success(),
        "overlapping moved history must keep the block; stdout:\n{}",
        String::from_utf8_lossy(&resume.stdout),
    );
    let err = String::from_utf8(resume.stderr).expect("utf-8 stderr");
    assert!(
        err.contains("the moved history overlaps the task's work on `notes.md`"),
        "the block must name the overlapping path; got:\n{err}",
    );
    assert!(
        err.contains("route: resolve the overlap on `notes.md` against the new history"),
        "the route must lead with the non-destructive repair; got:\n{err}",
    );
}

/// (c) The **un-refusing control**: `base == HEAD` still resumes exactly as before, and
/// the overlapping state — the only one that still blocks — is a state that **already
/// refused** under the blanket rule (it refuses for every `base != HEAD`). So the rule
/// change is provably only ever refusal→success: no state that succeeds today begins to
/// refuse.
#[test]
fn resume_at_its_pin_still_succeeds_and_only_refusals_become_successes() {
    let repo = TempDir::new("control");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let slug = "pin-holds";
    let mint = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "--slug",
            slug,
            "hold the pin",
        ],
    );
    assert!(
        mint.status.success(),
        "the mint must succeed; stderr:\n{}",
        String::from_utf8_lossy(&mint.stderr),
    );

    // `base == HEAD`: the pin holds — unchanged by the rule change.
    let at_pin = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        at_pin.status.success(),
        "a task at its pin must resume exactly as before; stderr:\n{}",
        String::from_utf8_lossy(&at_pin.stderr),
    );

    // The same repo, advanced onto an overlapping path: still refused — as the blanket
    // rule refused it, since `base != HEAD`.
    commit_file(repo.path(), "notes.md", "theirs\n", "second");
    fs::write(repo.path().join("notes.md"), "mine\n").expect("write file");
    let overlapping = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        !overlapping.status.success(),
        "the overlapping state must still refuse — it refused under the blanket rule too; \
         stdout:\n{}",
        String::from_utf8_lossy(&overlapping.stdout),
    );
}

/// (d) The **declared non-goal**, pinned rather than left to inference: the milestone
/// sub-agent re-entry door (`jigc workflow <W> --task <sub>`) keeps the blanket refusal.
/// Its commit boundary is the consciously-strict `plan_milestone_finalize` (whose base
/// equality `decide_base_repin`'s own doc comment calls "consciously unchanged"), so
/// relaxing it would make a read door **looser** than its commit door — the inverse of
/// the defect this task fixes. One repo state, two doors, opposite verdicts.
#[test]
fn sub_task_re_entry_keeps_the_blanket_base_pin_refusal() {
    let repo = TempDir::new("reentry");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A milestone + one sub-task (`milestone create` commits the record, so the sub-task
    // pins to the post-record HEAD), then a top-level task pinned to that same HEAD.
    let created = run(repo.path(), home.path(), &["milestone", "create", "Rework"]);
    assert!(
        created.status.success(),
        "`milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
    let added = run(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "rework",
            "Do the thing",
            "--workflow",
            "single-task",
        ],
    );
    assert!(
        added.status.success(),
        "`add-task` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&added.stderr),
    );
    let top = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "--slug",
            "top-task",
            "do the top thing",
        ],
    );
    assert!(
        top.status.success(),
        "the top-level mint must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&top.stderr),
    );

    // One disjoint intervening commit moves HEAD under both tasks.
    commit_file(repo.path(), "unrelated.md", "unrelated\n", "unrelated");

    let resume = run(repo.path(), home.path(), &["start", "--task", "top-task"]);
    assert!(
        resume.status.success(),
        "the top-level resume door re-pins over disjoint history; stderr:\n{}",
        String::from_utf8_lossy(&resume.stderr),
    );

    let reentry = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", "do-the-thing"],
    );
    assert!(
        !reentry.status.success(),
        "the sub-agent re-entry door must keep the blanket refusal; stdout:\n{}",
        String::from_utf8_lossy(&reentry.stdout),
    );
    let err = String::from_utf8(reentry.stderr).expect("utf-8 stderr");
    assert!(
        err.contains("is pinned to base") && err.contains("jigc task discard do-the-thing"),
        "the re-entry refusal keeps its pinned-to-base prose and discard route; got:\n{err}",
    );
}
