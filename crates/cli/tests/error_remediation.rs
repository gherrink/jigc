//! Integration test for **error remediation** (M36 Increment 4, T1 —
//! `implementation/roadmap.md` → M36 Inc-4 bullet 1: "dead-end errors become
//! self-correcting"). Drives the built `jigc` binary against a throwaway temp git
//! repo and asserts on the **emitted stderr** of four bad-verb loci — the contract is
//! the bytes an agent actually reads, never a reconstructed equivalent.
//!
//! The four loci (mirroring the proven-good `migrate --as <bad>` model — name the
//! valid set + carry a route to a real recovery, drop the leaked `os error 2` tail):
//!
//! - (a) `jigc doc create <unknown>` AND `jigc doc author <unknown>` — stderr names a
//!   valid doctype (`adr`) and routes to `jigc describe`, never the nonexistent
//!   `jigc doc types` micro-verb;
//! - (b) `jigc migrate <missing> --as <ty>` — stderr carries a route, no `os error`;
//! - (c) `jigc rename <valid:nonexistent> --to X` — stderr carries a recovery route;
//! - (d) `jigc doc add-item <bad-addr>` — stderr keeps `no staged instance` + the
//!   address, no `os error`.

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
            "jigc-error-remediation-{tag}-{}-{:?}",
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

/// Run `git` in `repo`, asserting success.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>` piping `stdin` — the payload verbs (`doc author`) read handoff.
fn run_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
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

/// Initialize a git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Stand up a project layer + one write-ready sub-task area (`single-task`, re-entered
/// once so its commit doc is provisioned). Returns `(repo, home, sub-task-id)`.
fn repo_with_task() -> (TempDir, TempDir, &'static str) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let created = run(repo.path(), home.path(), &["milestone", "create", "Rework"]);
    assert!(
        created.status.success(),
        "`milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );

    let sub = "do-the-thing";
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

    (repo, home, sub)
}

/// (a) `jigc doc create <unknown>` — stderr names a valid doctype and routes to
/// `jigc describe`, never the nonexistent `jigc doc types`.
#[test]
fn doc_create_unknown_doctype_names_the_set_and_routes_to_describe() {
    let (repo, home, sub) = repo_with_task();
    let out = run(
        repo.path(),
        home.path(),
        &[
            "doc", "create", "wormhole", "--title", "Whatever", "--task", sub,
        ],
    );
    assert!(
        !out.status.success(),
        "an unknown doctype must reject (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("adr"),
        "the reject must name a valid doctype (e.g. `adr`); got:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc describe"),
        "the reject must route to `jigc describe`; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("jigc doc types"),
        "the reject must NOT route to the nonexistent `jigc doc types`; got:\n{stderr}",
    );
}

/// (a) `jigc doc author <unknown>` — same remediation as `doc create`.
#[test]
fn doc_author_unknown_doctype_names_the_set_and_routes_to_describe() {
    let (repo, home, sub) = repo_with_task();
    let out = run_stdin(
        repo.path(),
        home.path(),
        &[
            "doc",
            "author",
            "wormhole",
            "--from-file",
            "-",
            "--task",
            sub,
        ],
        b"title: Whatever\n",
    );
    assert!(
        !out.status.success(),
        "an unknown doctype must reject (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("adr"),
        "the reject must name a valid doctype (e.g. `adr`); got:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc describe"),
        "the reject must route to `jigc describe`; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("jigc doc types"),
        "the reject must NOT route to the nonexistent `jigc doc types`; got:\n{stderr}",
    );
}

/// (b) `jigc migrate <missing> --as <ty>` — stderr carries a route, no `os error` tail.
#[test]
fn migrate_missing_source_routes_without_os_error_tail() {
    let (repo, home, _sub) = repo_with_task();
    let out = run(
        repo.path(),
        home.path(),
        &["migrate", "NOPE.md", "--as", "changelog"],
    );
    assert!(
        !out.status.success(),
        "a missing migration source must reject (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("route"),
        "the missing-source reject must carry a route; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("os error"),
        "the missing-source reject must NOT leak the `os error` tail; got:\n{stderr}",
    );
}

/// (c) `jigc rename <valid:nonexistent> --to X` — stderr carries a recovery route.
#[test]
fn rename_nonexistent_doc_carries_a_recovery_route() {
    let (repo, home, _sub) = repo_with_task();
    let out = run(
        repo.path(),
        home.path(),
        &["rename", "adr:ghosttown", "--to", "New title"],
    );
    assert!(
        !out.status.success(),
        "renaming a nonexistent doc must reject (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no managed doc"),
        "the reject must say `no managed doc`; got:\n{stderr}",
    );
    assert!(
        stderr.contains("route"),
        "the nonexistent-doc reject must carry a recovery route; got:\n{stderr}",
    );
}

/// (d) `jigc doc add-item <bad-addr>` — stderr keeps `no staged instance` + the
/// address, no `os error` tail.
#[test]
fn add_item_absent_instance_keeps_message_without_os_error_tail() {
    let (repo, home, sub) = repo_with_task();
    let out = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            "adr:ghosttown#options",
            "--title",
            "An option",
            "--task",
            sub,
        ],
    );
    assert!(
        !out.status.success(),
        "adding an item to an absent instance must reject (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no staged instance") && stderr.contains("adr:ghosttown"),
        "the reject must say `no staged instance` and name the address; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("os error"),
        "the absent-instance reject must NOT leak the `os error` tail; got:\n{stderr}",
    );
}
