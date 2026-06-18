//! Integration test for the `--task <id>` selector on the `jigc doc` write verbs
//! (M8 Increment 3, T1 — `design/write-commands.md` → The write-time
//! `--task`-scoped barrier: active-task resolution, explicit wins; else single;
//! else reject).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo holding a
//! milestone with **two** write-ready sub-task areas, asserting the selector
//! routes a staging write to the *named* sub-area and leaves the sibling
//! untouched, and that the ambiguous (two areas, no `--task`) and unknown-id
//! cases reject. This is pure selection + resolution — the barrier (T2) and
//! copy-in (T4) land later; here a write `--task <subA>` must hit subA's staged
//! commit doc and only subA's.

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
            "jigc-doc-task-scope-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>` with `cwd = repo`, piping `stdin`.
fn run_doc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn the jigc binary");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
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

/// A sub-task's staged commit-doc path: `.jigc/tasks/<sub>/docs/commit:<sub>.md`.
fn commit_doc(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(sub)
        .join("docs")
        .join(format!("commit:{sub}.md"))
}

/// Stand up a milestone with two write-ready sub-task areas (each minted
/// `--workflow single-task`, then re-entered once so its commit doc is
/// provisioned). Returns `(repo, home, subA, subB)`.
fn milestone_with_two_subtasks() -> (TempDir, TempDir, &'static str, &'static str) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let created = run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert!(
        created.status.success(),
        "`milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );

    let sub_a = "move-cache-to-redis";
    let sub_b = "evict-stale-keys";
    for (intent, _) in [("Move cache to redis", sub_a), ("Evict stale keys", sub_b)] {
        let added = run(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-rework",
                intent,
                "--workflow",
                "single-task",
            ],
        );
        assert!(
            added.status.success(),
            "`add-task {intent}` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&added.stderr),
        );
    }

    // Provision each sub-area's commit doc (deferred to first re-entry), so the
    // `set-slot` write below has a staged instance to splice.
    for sub in [sub_a, sub_b] {
        let entered = run(
            repo.path(),
            home.path(),
            &["workflow", "single-task", "--task", sub],
        );
        assert!(
            entered.status.success(),
            "first re-entry of `{sub}` must provision its commit doc; stderr:\n{}",
            String::from_utf8_lossy(&entered.stderr),
        );
        assert!(
            commit_doc(repo.path(), sub).is_file(),
            "`{sub}` must have a provisioned commit doc after re-entry",
        );
    }

    (repo, home, sub_a, sub_b)
}

#[test]
fn task_selector_routes_the_write_to_the_named_sub_area() {
    let (repo, home, sub_a, sub_b) = milestone_with_two_subtasks();

    // `--task <subA>` edits subA's staged doc; subB is untouched.
    let prose_a = b"Move the cache backend to redis.\n";
    let out = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{sub_a}#summary"),
            "--from-file",
            "-",
            "--task",
            sub_a,
        ],
        prose_a,
    );
    assert!(
        out.status.success(),
        "`set-slot --task {sub_a}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let staged_a = fs::read_to_string(commit_doc(repo.path(), sub_a)).expect("read subA staged");
    assert!(
        staged_a.contains("Move the cache backend to redis."),
        "subA's staged doc must carry the piped prose; got:\n{staged_a}",
    );
    let staged_b = fs::read_to_string(commit_doc(repo.path(), sub_b)).expect("read subB staged");
    assert!(
        !staged_b.contains("Move the cache backend to redis."),
        "subB must be untouched by a `--task {sub_a}` write; got:\n{staged_b}",
    );

    // `--task <subB>` targets subB.
    let prose_b = b"Evict stale keys on a TTL sweep.\n";
    let out = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{sub_b}#summary"),
            "--from-file",
            "-",
            "--task",
            sub_b,
        ],
        prose_b,
    );
    assert!(
        out.status.success(),
        "`set-slot --task {sub_b}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let staged_b = fs::read_to_string(commit_doc(repo.path(), sub_b)).expect("read subB staged");
    assert!(
        staged_b.contains("Evict stale keys on a TTL sweep."),
        "subB's staged doc must carry its own prose; got:\n{staged_b}",
    );
}

#[test]
fn two_active_tasks_with_no_task_selector_reject() {
    let (repo, home, sub_a, sub_b) = milestone_with_two_subtasks();

    let out = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{sub_a}#summary"),
            "--from-file",
            "-",
        ],
        b"ambiguous\n",
    );
    assert!(
        !out.status.success(),
        "two active tasks with no `--task` must reject (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("more than one active task"),
        "the rejection must say `more than one active task`; got:\n{stderr}",
    );
    // The error must enumerate the live ids so the user can copy one into `--task`
    // (M26 shakedown: don't ask for an id the user has no way to discover).
    assert!(
        stderr.contains(sub_a) && stderr.contains(sub_b),
        "the rejection must name both active task ids ({sub_a}, {sub_b}); got:\n{stderr}",
    );
}

#[test]
fn an_unknown_task_id_rejects() {
    let (repo, home, sub_a, _sub_b) = milestone_with_two_subtasks();

    let out = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{sub_a}#summary"),
            "--from-file",
            "-",
            "--task",
            "no-such-sub",
        ],
        b"unknown\n",
    );
    assert!(
        !out.status.success(),
        "`--task <unknown>` must reject (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no task") && stderr.contains("no-such-sub"),
        "the rejection must say `no task` and name the id; got:\n{stderr}",
    );
}
