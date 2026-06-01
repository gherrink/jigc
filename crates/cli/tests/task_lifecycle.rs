//! End-to-end integration test for the `jigc task <verb> <id>` lifecycle surface.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo with a started
//! task, exercising the three preview/abandon verbs
//! (`design/write-commands.md` → Lifecycle: `diff` · `validate` · `discard`):
//!
//! - `task diff <id>` — after staging a write, exits 0 and surfaces the staged
//!   change (`design/write-commands.md` → Lifecycle: see the working changeset).
//! - `task validate <id>` — runs `validate(task)` and renders findings; the exit
//!   code tracks blocking (`design/validation.md` → How it gates `finalize`:
//!   validate previews what finalize blocks on). A conformance-broken instance
//!   exits non-zero listing the `schema-conformance.*` finding; a clean one exits 0.
//! - `task discard <id>` — removes `.jigc/tasks/<id>/` and exits 0
//!   (`design/write-commands.md` → Lifecycle: abandon).
//!
//! No external test crates: the binary path comes from Cargo's `CARGO_BIN_EXE_jigc`,
//! the temp repo is a real `git init` (`jigc start` reads HEAD), and a self-cleaning
//! `TempDir` keeps the test off the developer's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-task-lifecycle-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// so the cascade resolves, then `jigc start --workflow single-task "<intent>"` to
/// mint a task and provision its commit doc (post-flip the cascade default is the
/// `router`, so minting goes through Form D). Returns the repo + a `$HOME` temp dir.
fn started_repo(intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
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
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", intent])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (repo, home)
}

/// Run `jigc doc <args>` with `cwd = repo`, optionally piping `stdin`.
fn run_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc");
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
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

/// Fill every author-required field/slot of the provisioned commit doc so a
/// `task validate` over it is clean (no `schema-conformance.*` blocker): the `type`
/// + `scope` header fields and the `summary` + `body` slots.
fn make_commit_conformant(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = run_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert!(
            out.status.success(),
            "set-field {addr}={value} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "gateway");
    set_slot(
        &format!("commit:{task}#summary"),
        b"add a per-client rate limiter\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Throttle abusive clients at the gateway.\n",
    );
}

#[test]
fn task_diff_shows_the_staged_change_and_exits_zero() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // Stage a write into the working area.
    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        None,
    );
    assert!(out.status.success(), "set-field must stage");

    let out = run_task(repo.path(), home.path(), &["diff", task]);
    assert!(
        out.status.success(),
        "`jigc task diff` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains("type: feat"),
        "`task diff` must surface the staged change; got:\n{stdout}"
    );
}

#[test]
fn task_validate_exit_code_tracks_blocking_findings() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // The freshly-provisioned commit doc has empty required slots/fields → blocking
    // `schema-conformance.*`. validate must exit non-zero and list a finding.
    let out = run_task(repo.path(), home.path(), &["validate", task]);
    assert!(
        !out.status.success(),
        "a conformance-broken instance must exit non-zero"
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        rendered.contains("schema-conformance."),
        "validate must list the `schema-conformance.*` finding; got:\n{rendered}"
    );

    // Fill every required field/slot → validate is clean and exits 0.
    make_commit_conformant(repo.path(), home.path(), task);
    let out = run_task(repo.path(), home.path(), &["validate", task]);
    assert!(
        out.status.success(),
        "a conformant instance must exit 0; output:\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn task_discard_removes_the_working_area_and_exits_zero() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(area.is_dir(), "the started task working area must exist");

    let out = run_task(repo.path(), home.path(), &["discard", task]);
    assert!(
        out.status.success(),
        "`jigc task discard` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !area.exists(),
        "discard must remove `.jigc/tasks/<id>/`; it still exists at {area:?}"
    );
}
