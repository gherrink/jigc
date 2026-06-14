//! Flow-A cold-start: the literal first commands of a new project must run on a
//! **fresh `git init` with ZERO commits** (`design/project-setup.md` → Flow 2
//! hardening — zero-commit; `implementation/roadmap.md` → M21 Increment 2).
//!
//! Today `jigc start` (and finalize) shell out to `git rev-parse HEAD`, which a repo
//! with no commits cannot resolve — the mint crashes before opening the working area.
//! The zero-commit sentinel detects an **unborn** HEAD distinctly (`git rev-parse
//! --verify -q HEAD` exits 1, vs 128 for a genuinely broken/missing git) and pins the
//! task to the canonical **empty-tree** SHA, so every `creates-task` workflow runs
//! pre-first-commit and the first finalize diffs against the empty tree and commits
//! cleanly as the repo's first commit.
//!
//! Three real-binary (`CARGO_BIN_EXE_jigc`) assertions over a throwaway `git init`:
//!   (i)   `jigc setup` then `jigc start --workflow single-task "<intent>"` mints
//!         (exit 0, the working area + a `base.json` pinning the empty-tree sentinel
//!         SHA appear) where today it crashes;
//!   (ii)  after a conformant commit-doc fill + a code change, `jigc task finalize`
//!         produces exactly ONE commit (the repo's first) carrying the code change +
//!         promoted docs, exit 0 — no empty-commit-guard false-abort;
//!   (iii) an existing repo WITH a seed commit still pins/diffs against the real HEAD
//!         SHA — the sentinel never fires when HEAD resolves.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The canonical git empty-tree SHA — the sentinel base an unborn HEAD pins to.
const EMPTY_TREE_SHA: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-coldstart-{tag}-{}-{:?}",
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

/// A real `git init` repo with identity configured but **no seed commit** (the
/// zero-commit precondition) — the `finalize_to_git.rs` harness minus its
/// `git commit -q -m initial`. Returns the repo + a `$HOME` temp dir.
fn fresh_unborn_repo() -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    (repo, home)
}

/// Run `jigc <args>` with `cwd = repo` and a temp `$HOME`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run jigc")
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

/// Fill every author-required field/slot of the provisioned commit doc so a `finalize`
/// over it validates clean (the `finalize_to_git.rs` idiom).
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

/// (i) + (ii): a fresh zero-commit repo mints against the empty-tree sentinel and the
/// first finalize lands the repo's first commit cleanly.
#[test]
fn zero_commit_repo_mints_against_the_sentinel_and_first_finalize_lands_one_commit() {
    let (repo, home) = fresh_unborn_repo();
    let task = "add-rate-limiter";

    // Sanity: the repo genuinely has no commits (unborn HEAD).
    assert!(
        !Command::new("git")
            .args(["rev-parse", "--verify", "-q", "HEAD"])
            .current_dir(repo.path())
            .output()
            .expect("run git")
            .status
            .success(),
        "the repo must start with an unborn HEAD (no commits)"
    );

    // `jigc setup` installs the adapter — must run pre-first-commit.
    let setup = run_jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` must run on a zero-commit repo; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr)
    );

    // (i) `jigc start --workflow single-task` must mint where today it crashes on
    // `git rev-parse HEAD`.
    let start = run_jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert!(
        start.status.success(),
        "`jigc start` must mint on a zero-commit repo (no `git rev-parse HEAD` crash); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&start.stdout),
        String::from_utf8_lossy(&start.stderr)
    );

    // The working area appeared, and base.json pins the empty-tree sentinel SHA.
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(area.exists(), "mint must open the working area at {area:?}");
    let base_json = fs::read_to_string(area.join("base.json")).expect("base pin written");
    assert!(
        base_json.contains(EMPTY_TREE_SHA),
        "the base pin must record the empty-tree sentinel SHA on a zero-commit repo; got:\n{base_json}"
    );

    // (ii) A conformant commit-doc fill + a code change → the first finalize lands ONE
    // commit (the repo's first), exit 0, no empty-commit-guard false-abort.
    fs::write(
        repo.path().join("limiter.rs"),
        "// a per-client rate limiter\n",
    )
    .expect("write code change");
    make_commit_conformant(repo.path(), home.path(), task);

    let finalize = run_jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        finalize.status.success(),
        "the first `jigc task finalize` must exit 0 on a zero-commit repo; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&finalize.stdout),
        String::from_utf8_lossy(&finalize.stderr)
    );

    // Exactly ONE commit exists — the repo's first.
    let count: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        count, 1,
        "the first finalize must produce exactly ONE commit (the repo's first)"
    );

    // The code change rode in that first commit.
    let files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.lines().any(|l| l == "limiter.rs"),
        "the code change must land in the first commit; files:\n{files}"
    );

    // The working area is gone (phase 7).
    assert!(
        !area.exists(),
        "finalize must remove the working area at {area:?}"
    );
}

/// (iii) The hardening #5 omitting-context guard: an existing repo WITH a seed commit
/// must still pin against the **real** HEAD SHA — the sentinel must never fire when
/// HEAD resolves.
#[test]
fn repo_with_a_seed_commit_pins_against_the_real_head_not_the_sentinel() {
    let (repo, home) = fresh_unborn_repo();
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    let head_sha = git(repo.path(), &["rev-parse", "HEAD"]);

    let start = run_jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert!(
        start.status.success(),
        "`jigc start` must mint on a seeded repo; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr)
    );

    let base_json = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter")
            .join("base.json"),
    )
    .expect("base pin written");
    assert!(
        base_json.contains(&head_sha),
        "the base pin must record the REAL HEAD SHA when HEAD resolves; got:\n{base_json}"
    );
    assert!(
        !base_json.contains(EMPTY_TREE_SHA),
        "the sentinel must never fire when HEAD resolves; got:\n{base_json}"
    );
}
