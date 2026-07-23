//! On-disk round-trip for the `commit` doctype's **trailers**, driven purely through
//! the write verbs (M45 Increment 4 T1 — `pinning.md` §2; `finalize.md` → Commit-doc
//! rendering / trailers).
//!
//! The `commit` schema's `trailers` block is `id-from: key`, so an item authored via
//! `jigc doc add-item …#trailers --title <Key>` carries the key as its **heading**,
//! never as a `- key:` field bullet (`store.rs` → `resolve_leaf`: the `id-from` leaf
//! resolves to the item heading). The commit renderer's `trailer_lines` therefore has
//! to read the key from `item.title`; reading a `key` field returns `None` for every
//! CLI-authored trailer, so the trailer silently never reaches the git message.
//!
//! This is the first test to author a `commit` trailer **through the verbs** (not a
//! hand-built `Instance` that pre-sets both a `title` and a synthetic `key` field) and
//! assert the finalized commit's `%(trailers)`. It closes the round-trip hole the
//! `commit` doctype was exempted from ("no on-disk round-trip to fence" — false: the
//! git message *is* the on-disk artifact, and the verbs are the only way to reach it).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the repo.

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
            "jigc-trailer-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// then `jigc start --workflow single-task "<intent>"`. Returns the repo + `$HOME`.
fn started_repo(intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
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

/// Set a commit-doc field/slot through the verbs, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    let out = run_doc(repo, home, &["set-field", addr, "--value", value], None);
    assert!(
        out.status.success(),
        "set-field {addr}={value} must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
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
}

/// Author a `commit` doc with a body and one trailer purely through the verbs, run
/// `finalize`, and assert the finalized commit's `%(trailers)` carries the authored
/// `<Key>: <value>` — the trailer authored via `add-item …#trailers --title <Key>`
/// (heading = key) + `set-field …/value` reaches the git message.
#[test]
fn a_trailer_authored_through_the_verbs_reaches_the_git_message() {
    let (repo, home) = started_repo("add a per-client rate limiter");
    let task = "add-a-per-client-rate";

    // A code change in the working tree (the task's work).
    fs::write(repo.path().join("limiter.rs"), "// rate limiter\n").expect("write code change");
    git(repo.path(), &["add", "limiter.rs"]);

    // The header + subject + body, through the verbs.
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#type"),
        "feat",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#scope"),
        "gateway",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#summary"),
        b"add a per-client rate limiter\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#body"),
        b"Throttle abusive clients at the gateway.\n",
    );

    // The trailer: `add-item …#trailers --title Refs` mints the item (heading = key),
    // then `set-field <emitted-addr>/value` fills the value. The emitted item address
    // is captured and fed verbatim (the emitted bytes are the contract).
    let add = run_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            &format!("commit:{task}#trailers"),
            "--title",
            "Refs",
        ],
        None,
    );
    assert!(
        add.status.success(),
        "`jigc doc add-item …#trailers --title Refs` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&add.stderr)
    );
    let item_addr = String::from_utf8(add.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned();
    assert!(
        item_addr.starts_with(&format!("commit:{task}#trailers/")),
        "`add-item` emits the minted trailer item address; got {item_addr:?}"
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("{item_addr}/value"),
        "#1242",
    );

    let log_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    let out = run_task(repo.path(), home.path(), &["finalize", task]);
    assert!(
        out.status.success(),
        "`jigc task finalize` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let log_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        log_after,
        log_before + 1,
        "finalize makes exactly ONE commit"
    );

    // The finalized commit's trailers, as git itself parses them — the round-trip
    // contract: an `add-item …#trailers` authored trailer is in `%(trailers)`.
    let trailers = git(repo.path(), &["log", "-1", "--format=%(trailers)"]);
    assert!(
        trailers.contains("Refs: #1242"),
        "the trailer authored through the verbs must reach the git message's %(trailers); \
         got trailers:\n{trailers}\nfull message:\n{}",
        git(repo.path(), &["log", "-1", "--format=%B"])
    );
}
