//! M43 Inc 1 T5 — the three divergent wrong-task-id messages converge on `jigc task
//! list` via one shared helper (`DECISIONS.md` 2026-07-16 M43 Settle, cross-cutting:
//! wrong-id routes converge — the old `start --task` route "list live tasks with
//! `jigc start`" was factually false, gap-pass verified on the real binary).
//!
//! Three surfaces reject a task id that names no live working area:
//!
//! - `jigc task <verb> <id>` (`task.rs` → `TaskArea::resolve`),
//! - `jigc start --task <id>` (`start.rs` → `resume_in_repo`),
//! - `jigc doc <verb> … --task <id>` (`doc.rs` → `ActiveTask::resolve`).
//!
//! Each test asserts the **emitted stderr bytes** carry the one converged message —
//! byte-identical across all three, the observable of the shared helper — and then
//! extracts the backticked route from those emitted bytes and **runs it verbatim**
//! (the route is the contract; a reconstructed command could mask a broken emission).
//!
//! Out of scope (a different state, its start-route correct): the **no-active-task**
//! reject (`no active task — start one with `jigc start``) when no `--task` is given
//! and no task exists.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! dev's repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-nosuchtask-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// The one converged no-such-task message every wrong-id surface emits.
const CONVERGED: &str = "no task `nonexistent` — list live tasks with `jigc task list`\n";

/// Assert `out` is the converged wrong-id reject, then extract the backticked route
/// from the **emitted** stderr and run it verbatim in `repo` — the emitted bytes are
/// the contract, so the route the agent would copy must actually run (exit 0).
fn assert_converged_and_route_runs(out: &std::process::Output, repo: &Path, home: &Path) {
    assert_eq!(
        out.status.code(),
        Some(1),
        "a wrong task id is an operational error (exit 1); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert_eq!(
        stderr, CONVERGED,
        "every wrong-task-id surface emits the one converged message (byte-identical \
         across the three — the shared-helper observable)",
    );
    assert!(
        !stderr.contains("list live tasks with `jigc start`"),
        "the factually-false `jigc start` listing claim must be gone; got:\n{stderr}",
    );

    // Extract the route from the emitted bytes: the backticked span after "with ".
    let (_, tail) = stderr
        .split_once("with `")
        .unwrap_or_else(|| panic!("no backticked route after `with ` in:\n{stderr}"));
    let (route, _) = tail
        .split_once('`')
        .unwrap_or_else(|| panic!("unterminated backticked route in:\n{stderr}"));
    let argv: Vec<&str> = route.split_whitespace().collect();
    assert_eq!(
        argv.first(),
        Some(&"jigc"),
        "the emitted route must lead with the binary name; got `{route}`",
    );
    let run = jigc(repo, home, &argv[1..]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "the emitted route `{route}` must run verbatim; stderr:\n{}",
        String::from_utf8_lossy(&run.stderr),
    );
}

/// `jigc task <verb> <id>` with an unknown id (`task.rs` → `TaskArea::resolve`) — the
/// old "start one with `jigc start \"<intent>\"`" reject converges on `jigc task list`.
#[test]
fn task_verb_with_unknown_id_routes_to_task_list() {
    let repo = TempDir::new("task-verb");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "nonexistent"],
    );
    assert_converged_and_route_runs(&out, repo.path(), home.path());
}

/// `jigc start --task <id>` with an unknown id (`start.rs` → `resume_in_repo`) — the
/// old route claimed `jigc start` lists live tasks, which it never did; the reject
/// converges on `jigc task list`.
#[test]
fn start_resume_with_unknown_id_routes_to_task_list() {
    let repo = TempDir::new("start-resume");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--task", "nonexistent"],
    );
    assert_converged_and_route_runs(&out, repo.path(), home.path());
}

/// `jigc doc <verb> … --task <id>` with an unknown explicit id (`doc.rs` →
/// `ActiveTask::resolve`) — already the right route; pinned onto the shared message.
#[test]
fn doc_verb_with_unknown_explicit_task_routes_to_task_list() {
    let repo = TempDir::new("doc-task");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:nope#type",
            "--value",
            "feat",
            "--task",
            "nonexistent",
        ],
    );
    assert_converged_and_route_runs(&out, repo.path(), home.path());
}
