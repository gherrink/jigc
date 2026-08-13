//! `jigc milestone provision` stops destroying what it cannot prove is junk (M48
//! Increment 1, T1; `DECISIONS.md` 2026-08-13 → the Settle, F3).
//!
//! `provision_worktrees` used to `remove_dir_all` **any** directory sitting at a sub-task's
//! `.jigc/worktrees/<sub-task-id>` path that was not a *registered* worktree of this repo —
//! at exit 0, with no probe and no consent. The registered set is the wrong subject: the
//! ordinary trigger is a `cp -R` or `mv` of the whole repo (how every RC trial corpus is
//! made), and a copy's worktree admin record names the **source's** path, so **no** path
//! under the copy's own `.jigc/worktrees/` is registered there, `git worktree prune` removes
//! nothing, and the copy's live, uncommitted sub-agent work dies silently. This was M47's
//! declared undischarged bound on `provision_worktrees`; the pre-1.0.0 trial confirmed it as
//! a live data-loss defect.
//!
//! The subject is now **the path the call is about to remove**, classified fail-closed by
//! [`cli::milestone::LeftoverVerdict`] — and this suite **iterates that code-side axis**
//! rather than pinning three hand-written repros: a fourth verdict cannot be minted without
//! this file failing to compile. Per verdict the fixture is the real shape that produces it:
//!
//! - [`LeftoverVerdict::NoOwnLinkage`] — a plain non-empty directory at the worktree path
//!   (`rev-parse --show-toplevel` walks up and prints the **main** repo root).
//! - [`LeftoverVerdict::OwnWorktree`] — a `cp -R` copy of the repo whose unregistered-but-live
//!   worktree holds an untracked file (`rev-parse` prints the path itself; `git status` reads
//!   its dirt correctly).
//! - [`LeftoverVerdict::Unverifiable`] — that same copy with the **source moved away**, so
//!   `rev-parse` exits 128 and git can say nothing at all.
//!
//! Each must refuse with the door-scoped blocking code, name the path, exit non-zero, and
//! leave the planted bytes **byte-intact**. The negative controls keep the guard from being
//! its own defect: an **empty** leftover still clears and provisions at exit 0, a second
//! `provision` is still idempotent (a registered worktree is reused untouched), and `--force`
//! clears and provisions in all three refusing arms.
//!
//! Drives the REAL binary — the emitted refusal is the contract, not a reconstructed one.

use cli::milestone::{LEFTOVER_VERDICTS, LeftoverVerdict, PROVISION_DOOR};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The bytes planted in every leftover — a refusal must leave them exactly this.
const PRECIOUS: &str = "precious, uncommitted, in no object DB\n";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-provision-leftover-{tag}-{}-{:?}",
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
        // Linked worktrees inside the tree are ordinary directories to `remove_dir_all`;
        // a moved-away source is simply already gone.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run `git <args>` in `cwd`, asserting success.
fn git_ok(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Initialize a real git repo with one commit (the milestone mint reads HEAD).
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    git_ok(root, &["config", "commit.gpgsign", "false"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Mint `milestone:cache-rework` with two sub-tasks — id-sorted `[area-low, area-zed]`, so
/// the leftover planted at `area-low` is the first path the provisioning loop reaches.
fn mint_milestone(repo: &Path, home: &Path) {
    assert!(
        run_milestone(repo, home, &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in ["Area zed", "Area low"] {
        assert!(
            run_milestone(repo, home, &["add-task", "cache-rework", intent])
                .status
                .success(),
            "add-task `{intent}` must exit 0",
        );
    }
}

/// `cp -R <from> <to>` — the ordinary way a corpus copy is made (the trigger the charter did
/// not know about), so the fixture reproduces it verbatim rather than simulating it.
fn copy_repo(from: &Path, to: &Path) {
    let out = Command::new("cp")
        .arg("-R")
        .arg(from)
        .arg(to)
        .output()
        .expect("run cp");
    assert!(
        out.status.success(),
        "cp -R {from:?} {to:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A planted leftover in the shape that produces one [`LeftoverVerdict`].
struct Fixture {
    /// The source repo — kept alive even when the run happens in the copy.
    _source: TempDir,
    /// The parent of the `cp -R` copy (the two copy-shaped verdicts).
    _copies: Option<TempDir>,
    /// Where a moved-away source lands, so it is still cleaned up.
    _attic: Option<TempDir>,
    /// `$HOME` for the runs.
    home: TempDir,
    /// The repo the `provision` runs in.
    repo: PathBuf,
    /// The leftover directory the guard must refuse to delete.
    leftover: PathBuf,
    /// The planted file inside it that must survive byte-intact.
    planted: PathBuf,
}

/// Build the real shape that yields `verdict`, with `PRECIOUS` planted inside the leftover at
/// the `area-low` worktree path.
fn plant(verdict: LeftoverVerdict) -> Fixture {
    let source = TempDir::new("source");
    init_repo(source.path());
    let home = TempDir::new("home");
    mint_milestone(source.path(), home.path());

    match verdict {
        LeftoverVerdict::NoOwnLinkage => {
            // A plain directory with no `.git` of its own: git walks up and answers for the
            // enclosing repo, so nothing there vouches for these bytes.
            let leftover = source
                .path()
                .join(".jigc")
                .join("worktrees")
                .join("area-low");
            fs::create_dir_all(&leftover).expect("mk leftover dir");
            let planted = leftover.join("precious.txt");
            fs::write(&planted, PRECIOUS).expect("plant precious.txt");
            let repo = source.path().to_path_buf();
            Fixture {
                _source: source,
                _copies: None,
                _attic: None,
                home,
                repo,
                leftover,
                planted,
            }
        }
        LeftoverVerdict::OwnWorktree | LeftoverVerdict::Unverifiable => {
            // Provision REAL worktrees in the source, then `cp -R` the whole repo: the copy's
            // admin records name the source's paths, so nothing under the copy's own
            // `.jigc/worktrees/` is registered there and `prune` removes nothing.
            let provisioned =
                run_milestone(source.path(), home.path(), &["provision", "cache-rework"]);
            assert!(
                provisioned.status.success(),
                "the source must provision cleanly first; stderr:\n{}",
                String::from_utf8_lossy(&provisioned.stderr),
            );
            let copies = TempDir::new("copies");
            let repo = copies.path().join("copy");
            copy_repo(source.path(), &repo);

            let leftover = repo.join(".jigc").join("worktrees").join("area-low");
            let planted = leftover.join("precious.txt");
            fs::write(&planted, PRECIOUS).expect("plant precious.txt");

            // The `Unverifiable` half: move the source away, so the copy's worktree points at
            // an admin directory that no longer exists and `rev-parse` exits 128.
            let attic = if verdict == LeftoverVerdict::Unverifiable {
                let attic = TempDir::new("attic");
                fs::rename(source.path(), attic.path().join("moved-source"))
                    .expect("move the source repo away");
                Some(attic)
            } else {
                None
            };

            Fixture {
                _source: source,
                _copies: Some(copies),
                _attic: attic,
                home,
                repo,
                leftover,
                planted,
            }
        }
    }
}

#[test]
fn every_verdict_refuses_a_non_empty_leftover_and_leaves_the_planted_bytes_intact() {
    for verdict in LEFTOVER_VERDICTS {
        let f = plant(verdict);
        let refused = run_milestone(&f.repo, f.home.path(), &["provision", "cache-rework"]);
        let stderr = String::from_utf8_lossy(&refused.stderr).into_owned();

        assert!(
            !refused.status.success(),
            "[{verdict:?}] provision must REFUSE a non-empty leftover; got {:?}\nstdout:\n{}\nstderr:\n{stderr}",
            refused.status,
            String::from_utf8_lossy(&refused.stdout),
        );
        assert!(
            stderr.contains(PROVISION_DOOR.code),
            "[{verdict:?}] the refusal must carry the door-scoped code `{}`; got:\n{stderr}",
            PROVISION_DOOR.code,
        );
        assert!(
            stderr.contains(&f.leftover.display().to_string()),
            "[{verdict:?}] the refusal must name the leftover path `{}`; got:\n{stderr}",
            f.leftover.display(),
        );
        assert!(
            stderr.contains("precious.txt"),
            "[{verdict:?}] the refusal must name what would be deleted; got:\n{stderr}",
        );
        assert!(
            stderr.contains("--force"),
            "[{verdict:?}] the refusal must route at the consent flag; got:\n{stderr}",
        );

        // The whole point: the bytes are still there, unchanged.
        assert_eq!(
            fs::read_to_string(&f.planted).expect("the planted file must survive the refusal"),
            PRECIOUS,
            "[{verdict:?}] the planted bytes must survive byte-intact",
        );
    }
}

#[test]
fn every_verdict_clears_and_provisions_under_force() {
    for verdict in LEFTOVER_VERDICTS {
        let f = plant(verdict);
        let forced = run_milestone(
            &f.repo,
            f.home.path(),
            &["provision", "cache-rework", "--force"],
        );
        assert!(
            forced.status.success(),
            "[{verdict:?}] `--force` is the consent to delete — provision must exit 0; got {:?}\nstderr:\n{}",
            forced.status,
            String::from_utf8_lossy(&forced.stderr),
        );
        assert!(
            !f.planted.exists(),
            "[{verdict:?}] `--force` means the leftover really is deleted",
        );
        assert!(
            f.leftover.join(".git").is_file(),
            "[{verdict:?}] a fresh linked worktree must stand at the cleared path",
        );
    }
}

#[test]
fn an_empty_leftover_still_clears_and_provisions() {
    // The negative control that keeps the guard from being its own defect: an empty
    // directory holds nothing, so it clears without consent — the idempotency a crashed
    // run's remnant depends on.
    let repo = TempDir::new("empty-leftover");
    init_repo(repo.path());
    let home = TempDir::new("home");
    mint_milestone(repo.path(), home.path());

    let leftover = repo.path().join(".jigc").join("worktrees").join("area-low");
    fs::create_dir_all(&leftover).expect("mk empty leftover dir");

    let provisioned = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "an EMPTY leftover must still clear and provision at exit 0; got {:?}\nstderr:\n{}",
        provisioned.status,
        String::from_utf8_lossy(&provisioned.stderr),
    );
    assert!(
        leftover.join(".git").is_file(),
        "the cleared path must carry a fresh linked worktree",
    );
}

#[test]
fn a_second_provision_reuses_the_registered_worktree_untouched() {
    // The other negative control: the guard must not fire on jigc's OWN live worktrees, and
    // the reuse must leave a sub-agent's in-flight work alone — this is the state M47 Inc 3
    // made normal and promised safe.
    let repo = TempDir::new("idempotent");
    init_repo(repo.path());
    let home = TempDir::new("home");
    mint_milestone(repo.path(), home.path());

    assert!(
        run_milestone(repo.path(), home.path(), &["provision", "cache-rework"])
            .status
            .success(),
        "the first provision must exit 0",
    );
    let wip = repo
        .path()
        .join(".jigc")
        .join("worktrees")
        .join("area-low")
        .join("wip.txt");
    fs::write(&wip, PRECIOUS).expect("plant sub-agent WIP in the live worktree");

    let again = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        again.status.success(),
        "a second provision must still be idempotent (exit 0); got {:?}\nstderr:\n{}",
        again.status,
        String::from_utf8_lossy(&again.stderr),
    );
    assert_eq!(
        fs::read_to_string(&wip).expect("the reused worktree's WIP must survive"),
        PRECIOUS,
        "a registered worktree is reused UNTOUCHED",
    );
}
