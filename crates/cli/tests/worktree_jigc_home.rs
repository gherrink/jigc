//! Integration test for the M31 Inc-2 `repo_root` / `jigc_home` split: from inside a
//! linked `git worktree`, the committed doc-store and `.jigc/` resolve against
//! **jigc_home** (the main checkout) while code/HEAD resolve against the worktree.
//!
//! The previously-broken path is `reenter_in_repo` (`jigc workflow <W> --task <id>`): a
//! fanned sub-agent re-enters from its worktree, but the milestone's `.jigc/tasks/<sub>/`
//! area + `.jigc/config/` cascade live only in the main checkout (the worktree carries
//! neither — `.jigc/tasks` is runtime state, `.jigc/config` here is untracked, so a
//! `git worktree add` checkout copies neither). Before the rebind, re-entry resolved
//! `.jigc` against the worktree and failed; after, it resolves against jigc_home.
//!
//! The base-pin HEAD routing is the sharp seam: the main checkout's HEAD is advanced
//! **past** the milestone base pin *after* the worktree is added, so the worktree HEAD
//! (still == the pin) diverges from jigc_home's HEAD. Re-entry must read the **worktree**
//! HEAD (== the pin) for the base-pin guard — reading jigc_home's HEAD would block the
//! guard with a base mismatch.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop (cleans up worktrees too —
/// `remove_dir_all` flattens the tree).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-worktree-home-{tag}-{}-{:?}",
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

/// Run `git <args>` in `dir`, asserting success.
fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// `git rev-parse HEAD` in `dir`.
fn head_sha(dir: &Path) -> String {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .expect("run git rev-parse");
    assert!(out.status.success(), "rev-parse HEAD failed");
    String::from_utf8(out.stdout)
        .expect("utf-8 sha")
        .trim()
        .to_owned()
}

/// Run `jigc <args>` with `cwd = dir` and `$HOME = home`.
fn jigc(dir: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(dir)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn reentry_from_a_worktree_resolves_jigc_home_and_reads_the_worktree_head() {
    let main = TempDir::new("main");
    let home = TempDir::new("home");

    // C0 — the milestone base: a real git repo with one commit + the (untracked)
    // `.jigc/config/` cascade layer so the milestone verbs resolve.
    git(main.path(), &["init", "-q"]);
    git(main.path(), &["config", "user.email", "t@t"]);
    git(main.path(), &["config", "user.name", "t"]);
    fs::write(main.path().join("README.md"), "hello\n").expect("write README");
    git(main.path(), &["add", "README.md"]);
    git(main.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(main.path().join(".jigc").join("config")).expect("create project layer");
    let pin = head_sha(main.path());

    // A milestone (pinned to C0) with a single-task sub-task. The sub-area lives under
    // the main checkout's `.jigc/tasks/` only (untracked runtime state).
    let created = jigc(
        main.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert!(
        created.status.success(),
        "milestone create must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
    let sub = "move-cache-to-redis";
    let added = jigc(
        main.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Move cache to redis",
            "--workflow",
            "single-task",
        ],
    );
    assert!(
        added.status.success(),
        "add-task must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&added.stderr),
    );

    // The linked worktree, checked out at C0 (== the pin) on its own branch.
    let wt = main.path().join("wt");
    git(
        main.path(),
        &["worktree", "add", "-q", wt.to_str().unwrap()],
    );
    assert!(
        wt.join(".git").is_file(),
        "a linked worktree's .git must be a file",
    );
    assert_eq!(
        head_sha(&wt),
        pin,
        "the worktree HEAD must start at the pin (C0)"
    );

    // C1 — advance the MAIN checkout's HEAD past the pin, *after* the worktree was added.
    // Now jigc_home's HEAD (C1) != the worktree HEAD (C0 == the pin): re-entry's base-pin
    // guard must read the WORKTREE HEAD, not jigc_home's, or it would block.
    fs::write(main.path().join("other.txt"), "more\n").expect("write other.txt");
    git(main.path(), &["add", "other.txt"]);
    git(main.path(), &["commit", "-q", "-m", "advance main"]);
    assert_ne!(
        head_sha(main.path()),
        pin,
        "the main checkout HEAD must have advanced past the pin",
    );
    assert_eq!(
        head_sha(&wt),
        pin,
        "the worktree HEAD must still be at the pin"
    );

    // The previously-broken path: re-enter the sub-task from INSIDE the worktree. The
    // `.jigc/config/` cascade + the `.jigc/tasks/<sub>/` area exist only in the main
    // checkout, so this resolves only if `.jigc` binds to jigc_home; the base-pin guard
    // passes only if HEAD is read from the worktree (C0 == pin), not jigc_home (C1).
    let composed = jigc(
        &wt,
        home.path(),
        &["workflow", "single-task", "--task", sub],
    );
    assert!(
        composed.status.success(),
        "re-entry from a worktree must exit 0 (jigc_home `.jigc` + worktree HEAD); stderr:\n{}",
        String::from_utf8_lossy(&composed.stderr),
    );
    let out = String::from_utf8(composed.stdout).expect("utf-8 stdout");
    assert!(
        out.contains("jigc doc create adr"),
        "the composed single-task view (read from jigc_home) must carry its create-gate \
         ADR affordance; got:\n{out}",
    );

    // The `.jigc` write target is jigc_home: first-entry provisioning lands the commit
    // doc under the MAIN checkout's `.jigc/tasks/<sub>/docs/`, never the worktree's.
    let provisioned = main
        .path()
        .join(".jigc")
        .join("tasks")
        .join(sub)
        .join("docs")
        .join(format!("commit:{sub}.md"));
    assert!(
        provisioned.is_file(),
        "first-entry provisioning must write into the main checkout's `.jigc/tasks/<sub>/docs/`",
    );
    assert!(
        !wt.join(".jigc").join("tasks").join(sub).exists(),
        "no task area may be created under the worktree's `.jigc/`",
    );
}
