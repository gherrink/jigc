//! M30 Increment 1 — per-task `finalize` change-set scoping (the dogfood repro).
//!
//! A per-task non-migration `finalize` commits exactly the agent's **existing git
//! index** + jigc's promoted docs + the first-commit config layer — never the ambient
//! dirty tree (`design/finalize.md` → Dirty-tree policy, revised M30; `DECISIONS.md`
//! 2026-06-20 M30 planning, G1/G2/G6). The two cases the keystone proves:
//!
//! - **Narrowing:** a finalize with a *staged* task edit alongside an unrelated
//!   *untracked* file and an unrelated *unstaged-modified tracked* file commits ONLY the
//!   staged edit (and jigc's promoted/config files); both unrelated files REMAIN
//!   uncommitted in the working tree (RED under the old `git add --all` sweep, GREEN
//!   after the narrowing).
//!
//! - **Block-on-empty (G2):** a task that stages NOTHING on a dirty tree BLOCKS with the
//!   "you staged nothing — `git add` your changes" guidance (the no-empty-commits
//!   invariant on the narrowed set).
//!
//! The assertions inspect the **landed git commit** (`git show --name-only HEAD`) and
//! the post-commit `git status --porcelain`, never a reconstruction.

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
            "jigc-finalize-index-scoping-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
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
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit (a tracked `README.md`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`,
/// optionally piping `stdin`, capturing output.
fn run_jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", dev_pack())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
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

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = run_jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = run_jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        );
        assert!(out.status.success(), "set-field {addr} must succeed");
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"scope the change set\n");
    set_slot(&format!("commit:{task}#body"), b"An M30 change.\n");
}

/// The narrowing keystone: a finalize with a staged task edit + an unrelated untracked
/// file + an unrelated unstaged-modified tracked file commits ONLY the staged edit (plus
/// jigc's own promoted/config files), leaving both unrelated files uncommitted.
#[test]
fn per_task_finalize_commits_only_the_staged_index_not_the_dirty_tree() {
    let repo = TempDir::new("narrow");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let task = "scope-the-set";
    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "scope the set"],
        "jigc start",
    );

    // The agent's own task edit — written AND staged (the new G5 contract).
    fs::write(repo.path().join("feature.rs"), "pub fn feature() {}\n").expect("write feature.rs");
    git(repo.path(), &["add", "feature.rs"]);

    // An unrelated untracked file — must NOT ride the commit.
    fs::write(repo.path().join("scratch.txt"), "private WIP\n").expect("write scratch.txt");
    // An unrelated unstaged-modified TRACKED file — its edit must NOT ride the commit.
    fs::write(repo.path().join("README.md"), "hello\nlocal edit\n").expect("modify README.md");

    fill_commit(repo.path(), home.path(), task);

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", task],
        "jigc task finalize",
    );

    // The landed commit carries the staged edit.
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    let landed: Vec<&str> = committed.lines().collect();
    assert!(
        landed.contains(&"feature.rs"),
        "the staged task edit lands in the commit; files:\n{committed}",
    );
    // The unrelated files did NOT ride the commit.
    assert!(
        !landed.contains(&"scratch.txt"),
        "the unrelated untracked file must NOT ride the commit; files:\n{committed}",
    );
    assert!(
        !landed.contains(&"README.md"),
        "the unrelated unstaged-modified tracked file must NOT ride the commit; files:\n{committed}",
    );

    // Both unrelated changes REMAIN uncommitted in the working tree post-commit.
    let status = git(repo.path(), &["status", "--porcelain"]);
    assert!(
        status.lines().any(|l| l == "?? scratch.txt"),
        "the untracked file stays untracked after the commit; status:\n{status}",
    );
    assert!(
        status
            .lines()
            .any(|l| l.trim_start().starts_with("M") && l.ends_with("README.md")),
        "the unstaged-modified tracked file stays modified after the commit; status:\n{status}",
    );
    // HEAD's README.md is still the unedited baseline (the local edit never landed).
    let head_readme = git(repo.path(), &["show", "HEAD:README.md"]);
    assert_eq!(
        head_readme, "hello",
        "the unrelated local edit must not be in the committed README.md; got:\n{head_readme}",
    );
}

/// G2 — a task that stages NOTHING on a dirty tree blocks with the git-add guidance
/// (the no-empty-commits invariant on the narrowed set). Run after a first finalize so
/// the config layer is already committed (so its first-commit term does not mask the
/// empty narrowed set).
#[test]
fn per_task_finalize_blocks_when_nothing_staged_on_a_dirty_tree() {
    let repo = TempDir::new("block");
    let home = TempDir::new("home");
    init_repo(repo.path());

    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // ── a first task lands a real staged edit (commits the config layer too) ──
    let first = "land-the-baseline";
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "land the baseline"],
        "jigc start (first)",
    );
    fs::write(repo.path().join("baseline.rs"), "pub fn baseline() {}\n")
        .expect("write baseline.rs");
    git(repo.path(), &["add", "baseline.rs"]);
    fill_commit(repo.path(), home.path(), first);
    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", first],
        "jigc task finalize (first)",
    );

    // ── a second task stages nothing, leaves a dirty tree → blocks ──
    let second = "stage-nothing";
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "stage nothing"],
        "jigc start (second)",
    );
    // A dirty working tree, but nothing is `git add`ed.
    fs::write(repo.path().join("README.md"), "hello\nanother local edit\n").expect("modify README");
    fill_commit(repo.path(), home.path(), second);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", second],
        None,
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a dirty-tree-nothing-staged finalize must block; got:\n{rendered}",
    );
    assert!(
        rendered.contains("you staged nothing"),
        "the block names the empty-index condition; got:\n{rendered}",
    );
    assert!(
        rendered.contains("git add"),
        "the block points at `git add`; got:\n{rendered}",
    );
}
