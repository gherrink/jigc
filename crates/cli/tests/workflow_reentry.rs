//! End-to-end integration test for `jigc workflow <W> --task <id>` — the M8
//! Increment 2 sub-agent re-entry verb (`design/write-commands.md` → Sub-agent
//! re-entry). A fanned sub-agent re-enters its milestone sub-task by the
//! explicitly-named sub-workflow `<W>`; the CLI **asserts `<W>` equals the
//! sub-task's recorded mint workflow** before composing, so a stale launch
//! template that names the wrong workflow fails loudly rather than silently
//! composing the wrong thing.
//!
//! This task (T2) is the **read/compose** half: compose `<W>` for the sub-task
//! with the W-equality guard. Provisioning of the write-ready area is T3.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo: a milestone
//! with two sub-tasks (one minted `--workflow single-task`, one defaulting to
//! `sub-task`). The four asserted cases:
//!   (a) `jigc workflow single-task --task <single>` composes the single-task view
//!       (its create-gate `jigc doc create adr` line emitted) and exits 0.
//!   (b) `jigc workflow router --task <single>` (`<W>` != recorded) is rejected with
//!       a blocking finding naming the mismatch and exits non-zero.
//!   (c) `jigc workflow single-task --task <unknown>` rejects `no task`.
//!   (d) `jigc workflow sub-task --task <default>` surfaces the existing
//!       `workflow-refs.unknown-workflow` block (the `sub-task` pack workflow
//!       ships in increment 5 — fail-loud until then), exits non-zero.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-workflow-reentry-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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
/// layer so the cascade resolves (milestone mint reads HEAD).
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
fn workflow_reentry_composes_w_with_the_equality_guard() {
    let repo = TempDir::new("guard");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A milestone with two sub-tasks: one minted `--workflow single-task` (so its
    // recorded mint workflow is single-task), one defaulting to `sub-task`.
    let created = run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
    let single = run(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Move cache to redis",
            "--workflow",
            "single-task",
        ],
    );
    assert!(
        single.status.success(),
        "`add-task --workflow single-task` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&single.stderr),
    );
    let single_sub = "move-cache-to-redis";
    let default = run(
        repo.path(),
        home.path(),
        &["milestone", "add-task", "cache-rework", "Evict stale keys"],
    );
    assert!(
        default.status.success(),
        "`add-task` (default workflow) must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&default.stderr),
    );
    let default_sub = "evict-stale-keys";

    // (a) `jigc workflow single-task --task <single>` composes the single-task view —
    //     `<W>` == the recorded mint workflow, so the guard passes. The
    //     single-task-distinctive line is its create-gate `jigc doc create adr`
    //     affordance (from `allows-create: [{type: adr, as: decision}]`).
    let composed = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", single_sub],
    );
    assert!(
        composed.status.success(),
        "`jigc workflow single-task --task <single>` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&composed.stderr),
    );
    let composed_out = String::from_utf8(composed.stdout).expect("utf-8 stdout");
    assert!(
        composed_out.contains("jigc doc create adr"),
        "the composed single-task view must carry its create-gate ADR affordance; got:\n{composed_out}",
    );

    // (b) `jigc workflow router --task <single>` — `<W>` (router) != the recorded
    //     mint workflow (single-task), so the equality guard rejects it with a
    //     blocking finding naming both the requested and the recorded workflow.
    let mismatch = run(
        repo.path(),
        home.path(),
        &["workflow", "router", "--task", single_sub],
    );
    assert!(
        !mismatch.status.success(),
        "a `<W>` != recorded-workflow mismatch must exit non-zero",
    );
    let mismatch_err = String::from_utf8_lossy(&mismatch.stderr);
    assert!(
        mismatch_err.contains("router")
            && mismatch_err.contains("single-task")
            && mismatch_err.contains(single_sub),
        "the mismatch finding must name the requested `<W>`, the recorded workflow, and \
         the sub-task; got:\n{mismatch_err}",
    );

    // (c) `jigc workflow single-task --task <unknown>` rejects `no task`.
    let unknown = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", "no-such-sub"],
    );
    assert!(
        !unknown.status.success(),
        "an unknown sub-task must exit non-zero",
    );
    let unknown_err = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        unknown_err.contains("no task") && unknown_err.contains("no-such-sub"),
        "the unknown-task rejection must say `no task` and name the id; got:\n{unknown_err}",
    );

    // (d) `jigc workflow sub-task --task <default>` — `<W>` (sub-task) == the recorded
    //     mint workflow, so the equality guard passes; but the `sub-task` pack
    //     workflow ships only in increment 5, so the compose surfaces the existing
    //     `workflow-refs.unknown-workflow` block (fail-loud, exits non-zero).
    let absent = run(
        repo.path(),
        home.path(),
        &["workflow", "sub-task", "--task", default_sub],
    );
    assert!(
        !absent.status.success(),
        "composing the not-yet-shipped `sub-task` pack workflow must exit non-zero",
    );
    let absent_err = String::from_utf8_lossy(&absent.stderr);
    assert!(
        absent_err.contains("sub-task"),
        "the unknown-workflow block must name the missing `sub-task` workflow; got:\n{absent_err}",
    );
}
