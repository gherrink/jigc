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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the `[dev ▸ methodology]` compose-marker path the sub-task arm
/// below needs requires it ABSENT — `pack.rs`: an explicit pack dir supersedes the marker).
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
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

/// Write the `[dev ▸ methodology]` compose marker — the composed cascade then resolves
/// the methodology-pack `milestone-record` schema, so `milestone create` / `add-task` land
/// the **committed** record a route printed on this milestone must not contradict.
fn write_compose_marker(repo: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
}

/// Run `git <args>` in `root`, returning stdout — the **committed** read (`git show
/// HEAD:<path>`), so the record assertion is over what is committed, never the worktree copy.
fn git_out(root: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Every backticked `` `jigc …` `` span in `text`, in emission order. A route is proven
/// by running the bytes the surface emitted; a command rebuilt in test code can pass while
/// the emitted span is unrunnable.
fn jigc_spans(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with("jigc "))
        .map(str::to_string)
        .collect()
}

/// Set up one milestone with one sub-task (`do-the-thing`) plus one top-level task
/// (`top-task`) pinned to the same HEAD — the two unit kinds the per-task doors must
/// treat differently. `milestone create` commits the record, so both pin to the
/// post-record HEAD.
fn milestone_with_a_sub_task_and_a_top_level_task(repo: &Path, home: &Path) {
    let created = run(repo, home, &["milestone", "create", "Rework"]);
    assert!(
        created.status.success(),
        "`milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
    let added = run(
        repo,
        home,
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
        repo,
        home,
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
}

/// (d) The **declared non-goal**, pinned rather than left to inference — and pinned over
/// its whole axis: **every** per-task read door of a milestone sub-task keeps the blanket
/// refusal, not just the one door the original arm happened to drive. A sub-task's commit
/// boundary is the consciously-strict `plan_milestone_finalize` (`jigc task finalize
/// <sub>` refuses outright), so a read door that re-pins over moved history would be
/// **looser** than its own commit door — the inverse of the defect this task fixes.
///
/// The axis is *door × unit kind*: `jigc start --task <id>` (resume) and `jigc workflow
/// <W> --task <id>` (sub-agent re-entry) are both iterated for the sub-task, against the
/// top-level control in the same repo state. The first face of M47 Inc 8's finding was
/// exactly the un-iterated member — the resume half drove `top-task`, so relaxing resume
/// unconditionally left the suite green while `jigc start --task do-the-thing` (the
/// sub-task's own advertised resume door) had become looser than the milestone finalize.
///
/// **Revised at M46 Inc 8 (B2-2), not doubled** ([pinning.md](../../../implementation/pinning.md)
/// §3 addendum, which names this test as the hazard): the *verdict* is unchanged and still
/// iterated over both doors, while the assertion over the emitted **route** now says what
/// the route must be rather than what it was. The refusal used to route at `jigc task
/// discard <sub>`, which exits 0 while the milestone's committed record still calls the
/// sub-task active — so the route is lifted out of the emitted bytes and **run verbatim**,
/// and the record, the task list and the working area are read back afterwards: a route
/// that contradicts the record it leaves standing cannot pass. Both provisioning states are
/// driven, since the route is offered in both — the trial's unprovisioned one and the
/// provisioned one M48's leftover classifier guards.
#[test]
fn sub_task_read_doors_keep_the_blanket_base_pin_refusal() {
    let repo = TempDir::new("reentry");
    init_repo(repo.path());
    // `[dev ▸ methodology]`, so the milestone's record is **committed** — the state the
    // route must not contradict, and the state the trial ran in.
    write_compose_marker(repo.path());
    let home = TempDir::new("home");
    milestone_with_a_sub_task_and_a_top_level_task(repo.path(), home.path());

    // One disjoint intervening commit moves HEAD under the top-level task; under the
    // sub-task the milestone's own record commits already moved it — the trial's state,
    // where *seeding* the sub-task is what took it off its pin.
    commit_file(repo.path(), "unrelated.md", "unrelated\n", "unrelated");

    // The top-level control: its commit door re-pins over disjoint history, so its read
    // door does too.
    let resume = run(repo.path(), home.path(), &["start", "--task", "top-task"]);
    assert!(
        resume.status.success(),
        "the top-level resume door re-pins over disjoint history; stderr:\n{}",
        String::from_utf8_lossy(&resume.stderr),
    );

    // Phase 1 — the trial's own state: the milestone is not provisioned yet, and the
    // emitted span is what provisions it.
    both_sub_task_doors_refuse_and_route_at_the_worktree(repo.path(), home.path(), "unprovisioned");

    // Phase 2 — the already-provisioned state, which M48's leftover classifier guards at
    // the same door. Live work planted in the worktree fences the route's own claim that a
    // second run leaves an existing worktree untouched: a route that destroyed it would be
    // a worse dead end than the one it replaced.
    let wip = repo
        .path()
        .join(".jigc")
        .join("worktrees")
        .join("do-the-thing")
        .join("wip.txt");
    fs::write(&wip, "live sub-agent work\n").expect("plant work in the provisioned worktree");
    both_sub_task_doors_refuse_and_route_at_the_worktree(repo.path(), home.path(), "provisioned");
    assert_eq!(
        fs::read_to_string(&wip).ok().as_deref(),
        Some("live sub-agent work\n"),
        "the route ran again over a provisioned worktree and must leave its work untouched",
    );
}

/// Both sub-task read doors in one provisioning `state`: each refuses, the refusal routes
/// at neither `jigc task discard <sub>` (which exits 0 while the committed record still
/// calls the sub-task active) nor at anything but the worktree its work happens in, and
/// **every emitted `jigc …` span runs verbatim from the state that raised it** and leaves
/// the committed record, the milestone's task list and the sub-task's working area
/// exactly as it found them.
fn both_sub_task_doors_refuse_and_route_at_the_worktree(repo: &Path, home: &Path, state: &str) {
    for argv in [
        ["start", "--task", "do-the-thing"].as_slice(),
        ["workflow", "single-task", "--task", "do-the-thing"].as_slice(),
    ] {
        let blocked = run(repo, home, argv);
        assert!(
            !blocked.status.success(),
            "`jigc {}` on a sub-task must keep the blanket refusal ({state}); stdout:\n{}",
            argv.join(" "),
            String::from_utf8_lossy(&blocked.stdout),
        );
        let err = String::from_utf8(blocked.stderr).expect("utf-8 stderr");
        assert!(
            err.contains("is pinned to base"),
            "`jigc {}` keeps the pinned-to-base prose ({state}); got:\n{err}",
            argv.join(" "),
        );
        assert!(
            !err.contains("jigc task discard do-the-thing"),
            "`jigc {}` must not route at the discard that exits 0 while the committed record \
             still names the sub-task ({state}); got:\n{err}",
            argv.join(" "),
        );
        assert!(
            err.contains(".jigc/worktrees/do-the-thing"),
            "`jigc {}` must name the worktree the sub-task's work actually happens in \
             ({state}); got:\n{err}",
            argv.join(" "),
        );

        // The emitted bytes are the contract: lift each span and run it as printed.
        //
        // **What the refusal must offer is a followable act, and which act depends on
        // the state** (M53 — the cwd census, C2-08). Until then one route covered both
        // provisioning states, on the ground that `jigc milestone provision` is idempotent;
        // driven, that meant an agent whose worktree was already cut was told to run a
        // command that does nothing before the step that moves it. So the provisioned arm
        // offers the `cd` alone and carries no `jigc` span at all — and this loop is itself
        // the demonstration: the FIRST argv's refusal names the provisioning door, the
        // helper runs it verbatim as it must, and by the second argv the worktree exists,
        // so the second refusal is the provisioned one. Both arms end on the absolute `cd`,
        // which is what the `.jigc/worktrees/do-the-thing` assertion above reads.
        let spans = jigc_spans(&err);
        assert!(
            !spans.is_empty() || err.contains("`cd /"),
            "the refusal must offer a followable act — a runnable `jigc` span where \
             provisioning is what resolves it, or an absolute `cd` where the worktree is \
             already cut ({state}); got:\n{err}",
        );
        for span in &spans {
            let span_argv: Vec<&str> = span.split_whitespace().skip(1).collect();
            let ran = run(repo, home, &span_argv);
            assert!(
                ran.status.success(),
                "the emitted span `{span}` must run verbatim from the {state} state that \
                 raised it; stdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&ran.stdout),
                String::from_utf8_lossy(&ran.stderr),
            );
        }

        // …and having run it, nothing it did contradicts what the record says. This is the
        // whole defect: the discarding route exited 0 and left the committed record reading
        // `status: active` over a working area it had just removed.
        let record = git_out(repo, &["show", "HEAD:docs/milestone-records/rework.md"]);
        assert!(
            record.contains("do-the-thing") && record.contains("status: active"),
            "the committed record still names the sub-task as active after the route ran \
             ({state}); got:\n{record}",
        );
        let listed = run(repo, home, &["milestone", "list-tasks", "rework"]);
        let listed_out = String::from_utf8_lossy(&listed.stdout).into_owned();
        assert!(
            listed.status.success() && listed_out.contains("do-the-thing"),
            "`milestone list-tasks` still lists the sub-task after the route ran ({state}); \
             got:\n{listed_out}",
        );
        assert!(
            repo.join(".jigc")
                .join("tasks")
                .join("do-the-thing")
                .is_dir(),
            "the working area the record calls active must survive the route ({state})",
        );
    }
}

/// (e) The **second face** of the same top-level-task assumption: the composed
/// `task scope:` footer. Its parallel claim ends "resuming or finalizing here blocks and
/// names the overlapping paths" — true of a top-level task, a law-1 lie on a milestone
/// sub-task, whose `jigc task finalize` refuses outright and whose only commit boundary
/// blocks on *any* moved history, not on overlap. The claim is scoped per unit kind: a
/// sub-task compose names the milestone finalize instead.
#[test]
fn the_task_scope_footer_scopes_its_parallel_claim_per_unit_kind() {
    let repo = TempDir::new("footer");
    init_repo(repo.path());
    let home = TempDir::new("home");
    milestone_with_a_sub_task_and_a_top_level_task(repo.path(), home.path());

    // Both tasks sit at their pin, so both doors compose.
    let top = run(repo.path(), home.path(), &["start", "--task", "top-task"]);
    assert!(
        top.status.success(),
        "the top-level resume must compose; stderr:\n{}",
        String::from_utf8_lossy(&top.stderr),
    );
    let top_out = String::from_utf8(top.stdout).expect("utf-8 stdout");
    assert!(
        top_out.contains(
            "once a sibling task commits a path this one also touches, resuming or finalizing \
             here blocks and names the overlapping paths"
        ),
        "a top-level compose keeps the overlap claim verbatim; got:\n{top_out}",
    );

    let sub = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", "do-the-thing"],
    );
    assert!(
        sub.status.success(),
        "the sub-task re-entry must compose at its pin; stderr:\n{}",
        String::from_utf8_lossy(&sub.stderr),
    );
    let sub_out = String::from_utf8(sub.stdout).expect("utf-8 stdout");
    assert!(
        !sub_out.contains("resuming or finalizing here blocks and names the overlapping paths"),
        "a sub-task compose must not claim the top-level overlap behaviour; got:\n{sub_out}",
    );
    assert!(
        sub_out.contains(
            "this task is a sub-task of milestone `rework`, whose `jigc milestone finalize \
             rework` is its only commit boundary — every door here stays pinned to the \
             milestone's base"
        ),
        "a sub-task compose names the milestone commit boundary instead; got:\n{sub_out}",
    );
}
