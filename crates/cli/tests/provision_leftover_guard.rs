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
//! **The leftover's position in the ordered path set is the second axis** (M48 completion
//! audit). The refusal's recorded contract is that it fires *before any removal or add*, so a
//! refused provision leaves **every** sub-task path as it found it — and the probe used to be
//! asked **inside** the provisioning loop, which made that false for every position but the
//! first: a leftover at path *k* refused only after paths `0..k` had been cleared and freshly
//! `git worktree add`-ed. Both fixtures planted at the first path, so the claim was never
//! exercised. The fixtures now plant at the **last** path, and
//! [`a_refusal_leaves_every_path_unprovisioned_wherever_the_leftover_sits`] **iterates the
//! position** over an order it reads back from the tool — so a change to the walk order
//! re-derives the axis instead of silently re-masking it.
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

/// Mint `milestone:cache-rework` with one sub-task per intent.
fn mint_milestone_with(repo: &Path, home: &Path, intents: &[&str]) {
    assert!(
        run_milestone(repo, home, &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in intents {
        assert!(
            run_milestone(repo, home, &["add-task", "cache-rework", intent])
                .status
                .success(),
            "add-task `{intent}` must exit 0",
        );
    }
}

/// Mint `milestone:cache-rework` with two sub-tasks — id-sorted `[area-low, area-zed]`, so the
/// leftover planted at `area-zed` is the **last** path the provisioning loop reaches and the
/// first path is something a door that mutates as it walks would already have provisioned.
fn mint_milestone(repo: &Path, home: &Path) {
    mint_milestone_with(repo, home, &["Area zed", "Area low"]);
}

/// The milestone's sub-task ids **in the order the provisioning loop walks them**, read back
/// from the tool (`list-tasks` prints the id-sorted enumeration) rather than hand-written — so
/// the position axis below is derived from the shipped order, and a change to that order
/// re-derives the axis instead of silently re-masking it.
fn ordered_sub_ids(repo: &Path, home: &Path) -> Vec<String> {
    let out = run_milestone(repo, home, &["list-tasks", "cache-rework"]);
    assert!(
        out.status.success(),
        "list-tasks must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let line = stdout
        .lines()
        .find(|line| line.contains("): "))
        .unwrap_or_else(|| panic!("list-tasks must print the enumeration; got:\n{stdout}"));
    let (_, listed) = line
        .split_once("): ")
        .unwrap_or_else(|| panic!("list-tasks must print `(<n>): <ids>`; got:\n{stdout}"));
    listed
        .split(", ")
        .map(|id| id.trim().to_string())
        .filter(|id| !id.is_empty())
        .collect()
}

/// The worktree paths git has registered **under this repo's own `.jigc/worktrees/`** — empty
/// until a `provision` lands one here, since a `cp -R` copy's admin records name the *source's*
/// paths (the very reason the guard's subject is the path, not the registered set). A refused
/// provision must leave this empty: anything in it was added before the refusal.
fn own_registered_worktrees(repo: &Path) -> Vec<String> {
    let out = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(repo)
        .output()
        .expect("run git worktree list");
    let root = repo
        .canonicalize()
        .unwrap_or_else(|_| repo.to_path_buf())
        .join(".jigc")
        .join("worktrees");
    let prefix = root.display().to_string();
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .filter(|path| path.starts_with(&prefix))
        .map(str::to_string)
        .collect()
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

/// Plant `PRECIOUS` at the **last** sub-task worktree path (`area-zed`) and leave the first
/// (`area-low`) absent, returning `(leftover, planted)`.
///
/// The position is the point: with the single leftover at the *end* of the walk, the first path
/// is a path a door that probes as it goes would already have cleared and provisioned before it
/// ever reached the refusal. A copy fixture arrives carrying the source's provisioned worktrees,
/// so the first one is dropped here — the set must hold exactly **one** leftover, or the refusal
/// is not attributable to the planted path.
fn plant_precious(repo: &Path) -> (PathBuf, PathBuf) {
    let root = repo.join(".jigc").join("worktrees");
    let _ = fs::remove_dir_all(root.join("area-low"));
    let leftover = root.join("area-zed");
    fs::create_dir_all(&leftover).expect("mk leftover dir");
    let planted = leftover.join("precious.txt");
    fs::write(&planted, PRECIOUS).expect("plant precious.txt");
    (leftover, planted)
}

/// Build the real shape that yields `verdict`, with `PRECIOUS` planted inside the leftover at
/// the `area-zed` worktree path ([`plant_precious`]).
fn plant(verdict: LeftoverVerdict) -> Fixture {
    let source = TempDir::new("source");
    init_repo(source.path());
    let home = TempDir::new("home");
    mint_milestone(source.path(), home.path());

    match verdict {
        LeftoverVerdict::NoOwnLinkage => {
            // A plain directory with no `.git` of its own: git walks up and answers for the
            // enclosing repo, so nothing there vouches for these bytes.
            let (leftover, planted) = plant_precious(source.path());
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

            let (leftover, planted) = plant_precious(&repo);

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
        // And the refusal is transactional: the leftover sits at the LAST path, so anything
        // provisioned under this repo's own worktrees root was landed on the way there.
        assert!(
            own_registered_worktrees(&f.repo).is_empty(),
            "[{verdict:?}] a refused provision must add NO worktree — it fires before any \
             removal or add, so no path is half-provisioned; got: {:?}",
            own_registered_worktrees(&f.repo),
        );
    }
}

#[test]
fn a_refusal_leaves_every_path_unprovisioned_wherever_the_leftover_sits() {
    // The position axis. The probe used to be asked INSIDE the provisioning loop, so a
    // leftover at any but the FIRST path refused only after the earlier paths had already
    // been cleared and freshly `git worktree add`-ed — the half-provisioned set the refusal's
    // own contract says cannot happen. Three sub-tasks, one run per position, and the walk
    // order is read back from the tool rather than assumed.
    let intents = ["Area zed", "Area low", "Area mid"];
    for position in 0..intents.len() {
        let repo = TempDir::new(&format!("position-{position}"));
        init_repo(repo.path());
        let home = TempDir::new("home");
        mint_milestone_with(repo.path(), home.path(), &intents);

        let ids = ordered_sub_ids(repo.path(), home.path());
        assert_eq!(
            ids.len(),
            intents.len(),
            "every minted sub-task must be enumerated; got {ids:?}",
        );
        let worktrees = repo.path().join(".jigc").join("worktrees");
        let leftover = worktrees.join(&ids[position]);
        fs::create_dir_all(&leftover).expect("mk leftover dir");
        let planted = leftover.join("precious.txt");
        fs::write(&planted, PRECIOUS).expect("plant precious.txt");

        let refused = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
        let stderr = String::from_utf8_lossy(&refused.stderr).into_owned();
        assert!(
            !refused.status.success(),
            "[position {position} of {ids:?}] provision must REFUSE; got {:?}\nstderr:\n{stderr}",
            refused.status,
        );
        assert!(
            stderr.contains(PROVISION_DOOR.code)
                && stderr.contains(&leftover.display().to_string()),
            "[position {position} of {ids:?}] the refusal must carry `{}` and name `{}`; got:\n{stderr}",
            PROVISION_DOOR.code,
            leftover.display(),
        );
        assert_eq!(
            fs::read_to_string(&planted).expect("the planted file must survive the refusal"),
            PRECIOUS,
            "[position {position} of {ids:?}] the planted bytes must survive byte-intact",
        );

        // "leaves every sub-task path exactly as it found it" — asserted over the whole set,
        // not just the refusing one: no worktree registered anywhere under this repo's
        // worktrees root, and every other path still absent.
        assert!(
            own_registered_worktrees(repo.path()).is_empty(),
            "[position {position} of {ids:?}] a refused provision must add NO worktree at ANY \
             path; got: {:?}",
            own_registered_worktrees(repo.path()),
        );
        for id in &ids {
            if id == &ids[position] {
                continue;
            }
            assert!(
                !worktrees.join(id).exists(),
                "[position {position} of {ids:?}] `{id}` was untouched before the run and must \
                 still be untouched after it",
            );
        }
        assert!(
            !leftover.join(".git").exists(),
            "[position {position} of {ids:?}] the refusing path itself must not have been \
             cleared and re-provisioned",
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
