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
    let git = |args: &[&str]| {
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
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
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
