//! Integration test for the staged snapshot at the `jigc milestone create`
//! task-minting door (M43 T1 — `design/surface-contract.md` → The carryover
//! gate): creating a milestone snapshots the shared checkout's pre-milestone
//! staged state (entries + staged deletions) into the milestone area
//! (`.jigc/milestones/<id>/staged-snapshot.json`), so the milestone finalize
//! can later refuse to let a foreign pre-staged change ride the aggregate
//! commit. Drives the built `jigc` binary against a throwaway temp git repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-staged-snapshot-{tag}-{}-{:?}",
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

/// Run `git <args>` in `root`, asserting success, returning trimmed stdout.
fn git(root: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// The M43 T1 milestone arm: `jigc milestone create` persists a staged snapshot
/// into the milestone area — a pre-staged add lands as a `(path, staged blob)`
/// entry and a pre-staged `git rm` lands in the deletion set, read straight off
/// the on-disk `staged-snapshot.json` the finalize preflight will consume.
#[test]
fn milestone_create_persists_staged_snapshot_into_milestone_area() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    fs::write(repo.path().join("del.txt"), "doomed\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    // The foreign pre-staged change-set: an add and a delete, staged BEFORE create.
    fs::write(
        repo.path().join("stray.txt"),
        "staged before the milestone\n",
    )
    .expect("write stray.txt");
    git(repo.path(), &["add", "stray.txt"]);
    git(repo.path(), &["rm", "-q", "del.txt"]);
    let stray_blob = git(repo.path(), &["rev-parse", ":stray.txt"]);

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

    let snapshot_path = repo
        .path()
        .join(".jigc")
        .join("milestones")
        .join("cache-rework")
        .join("staged-snapshot.json");
    let body = fs::read_to_string(&snapshot_path)
        .expect("`milestone create` must persist staged-snapshot.json into the milestone area");
    let snapshot: serde_json::Value = serde_json::from_str(&body).expect("snapshot parses");
    assert_eq!(
        snapshot["entries"]["stray.txt"],
        serde_json::Value::String(stray_blob),
        "the pre-staged add reads back as a (path, staged blob) entry; got:\n{body}"
    );
    assert_eq!(
        snapshot["deletions"],
        serde_json::json!(["del.txt"]),
        "the pre-staged `git rm` reads back in the deletion set; got:\n{body}"
    );
}
