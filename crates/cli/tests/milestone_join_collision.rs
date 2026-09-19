//! M45 Increment 6 / T2 — **the join surfaces cross-worktree code collisions**: `jigc
//! milestone join` reads the provisioned fan-out worktrees' staged sets and **blocks**
//! naming the colliding path when two sub-tasks stage the SAME file, instead of exiting 0
//! blind to the clash and surfacing it only at finalize (`design/storage.md` → the join's
//! never-blind-merge discipline; `DECISIONS.md` → 2026-07-23 M45 Settle, Decision 2;
//! roadmap Milestone 45 Increment 6).
//!
//! Two arms drive the cargo-built `jigc` binary (`CARGO_BIN_EXE_jigc`) against throwaway
//! git repos with the **embedded** dev pack (the milestone_boundary_gate idiom): two
//! worktrees staging one shared path make `join` block at a non-zero exit naming the path;
//! a disjoint pair keeps `join` clean at exit 0.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-join-collision-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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

/// Run `git <args>` in `dir`, asserting success and returning trimmed stdout.
fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {dir:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 git stdout")
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README.md");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(root);
}

/// Run `jigc milestone <args>` with `cwd = repo`, `$HOME = home`, the embedded pack.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("milestone").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation succeeded, surfacing both streams on failure.
fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Write + `git add` a code file IN a provisioned fan-out worktree
/// (`.jigc/worktrees/<sub>/<rel>`) — the staged code a fanned-out sub-agent produces in
/// its isolated worktree (the milestone_boundary_gate idiom).
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    git(&wt, &["add", rel]);
}

/// Stand up a two-sub-task fan-out milestone (`cache-rework`) with sub-tasks `doc-area` +
/// `code-area`, then `provision` (so the fan-out worktrees exist for the caller to stage
/// code into).
fn create_add_provision(repo: &Path, home: &Path) {
    expect_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "milestone create",
    );
    for intent in ["Doc area", "Code area"] {
        expect_ok(
            &run_milestone(repo, home, &["add-task", "cache-rework", intent]),
            "milestone add-task",
        );
    }
    expect_ok(
        &run_milestone(repo, home, &["provision", "cache-rework"]),
        "milestone provision",
    );
}

/// **Two worktrees staging the SAME path make `join` block naming the colliding path.**
/// Red today: `join` exits 0 blind to the collision, surfacing it only at finalize.
#[test]
fn colliding_worktrees_block_join_naming_the_path() {
    let repo = TempDir::new("collision");
    let home = TempDir::new("collision-home");
    init_repo(repo.path());
    create_add_provision(repo.path(), home.path());

    // Both sub-tasks stage the SAME code path — the cross-worktree collision.
    stage_worktree_code(
        repo.path(),
        "doc-area",
        "src/shared.rs",
        "pub fn from_doc_area() {}\n",
    );
    stage_worktree_code(
        repo.path(),
        "code-area",
        "src/shared.rs",
        "pub fn from_code_area() {}\n",
    );

    let out = run_milestone(repo.path(), home.path(), &["join", "cache-rework"]);
    let output =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a cross-worktree collision must block `join` at a non-zero exit; got {:?}\n{output}",
        out.status.code(),
    );
    assert!(
        output.contains("src/shared.rs"),
        "the block names the colliding path; got:\n{output}",
    );
    assert!(
        output.contains("code collision"),
        "the block names the collision class; got:\n{output}",
    );
}

/// **A disjoint pair keeps `join` clean at exit 0.** Two worktrees stage distinct paths —
/// no collision, so the join is inert and exits 0.
#[test]
fn disjoint_worktrees_keep_join_clean() {
    let repo = TempDir::new("disjoint");
    let home = TempDir::new("disjoint-home");
    init_repo(repo.path());
    create_add_provision(repo.path(), home.path());

    stage_worktree_code(
        repo.path(),
        "doc-area",
        "src/alpha.rs",
        "pub fn alpha() {}\n",
    );
    stage_worktree_code(
        repo.path(),
        "code-area",
        "src/beta.rs",
        "pub fn beta() {}\n",
    );

    let out = run_milestone(repo.path(), home.path(), &["join", "cache-rework"]);
    expect_ok(&out, "a disjoint fan-out join");
}
