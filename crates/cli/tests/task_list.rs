//! Integration test for `jigc task list` — the active-task discovery verb (M26
//! post-completion shakedown: with one or more active tasks there was no in-tool way
//! to find a task id; `ls .jigc/tasks/` was the only route). Drives the built `jigc`
//! binary against throwaway temp git repos with 0, 1, and 2 active tasks, asserting
//! the ids render in the default human surface, the empty roster is a clean message
//! at exit 0, and `--format json` emits a structured array.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-task-list-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// so the cascade resolves (`jigc start` reads HEAD).
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

/// `jigc start --workflow single-task "<intent>"` to mint one active task.
fn start(repo: &Path, home: &Path, intent: &str) {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", intent])
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start {intent}` must mint a task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Run `jigc task <args>` with `cwd = repo`.
fn run_task(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("task")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run jigc task")
}

#[test]
fn task_list_empty_is_a_clean_message_at_exit_zero() {
    let repo = TempDir::new("empty");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = run_task(repo.path(), home.path(), &["list"]);
    assert!(
        out.status.success(),
        "`jigc task list` with no active tasks must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains("no active tasks"),
        "the empty roster must read as a clean `no active tasks` message; got:\n{stdout}",
    );
}

#[test]
fn task_list_one_active_task_shows_its_id() {
    let repo = TempDir::new("one");
    let home = TempDir::new("home");
    init_repo(repo.path());
    start(repo.path(), home.path(), "add rate limiter");

    let out = run_task(repo.path(), home.path(), &["list"]);
    assert!(
        out.status.success(),
        "`jigc task list` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains("add-rate-limiter"),
        "the roster must show the active task id; got:\n{stdout}",
    );
}

#[test]
fn task_list_two_active_tasks_show_both_ids() {
    let repo = TempDir::new("two");
    let home = TempDir::new("home");
    init_repo(repo.path());
    start(repo.path(), home.path(), "add rate limiter");
    start(repo.path(), home.path(), "cache sessions");

    let out = run_task(repo.path(), home.path(), &["list"]);
    assert!(
        out.status.success(),
        "`jigc task list` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains("add-rate-limiter") && stdout.contains("cache-sessions"),
        "the roster must show both active task ids; got:\n{stdout}",
    );
}

#[test]
fn task_list_json_is_a_structured_array_of_ids() {
    let repo = TempDir::new("json");
    let home = TempDir::new("home");
    init_repo(repo.path());
    start(repo.path(), home.path(), "add rate limiter");
    start(repo.path(), home.path(), "cache sessions");

    let out = run_task(repo.path(), home.path(), &["--format", "json", "list"]);
    assert!(
        out.status.success(),
        "`jigc task list --format json` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("`--format json` must emit valid JSON ({e}); got:\n{stdout}"));
    let rows = parsed.as_array().expect("json output is an array");
    let ids: Vec<&str> = rows
        .iter()
        .filter_map(|row| row.get("id").and_then(|v| v.as_str()))
        .collect();
    assert!(
        ids.contains(&"add-rate-limiter") && ids.contains(&"cache-sessions"),
        "the json array must carry both task ids; got ids: {ids:?}",
    );
}
