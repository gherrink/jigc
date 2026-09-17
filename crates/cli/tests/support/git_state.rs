//! The **git-in-progress fixture builder** — one construction per git operation a
//! user can leave un-concluded, built **only by driving the real git**
//! ([pinning.md](../../../../implementation/pinning.md) §4; M52 Increment 2).
//!
//! # Why it is a builder and not a hand-written marker
//!
//! The posture family answers *"is an operation in progress?"* by reading the
//! entries git writes into the worktree's own git dir (`MERGE_HEAD`,
//! `rebase-apply/`, `sequencer/`, …). A fixture that *wrote those entries itself*
//! would prove the product against a state git would never produce — the fixture and
//! the code under test would share one author's belief about git, which is the one
//! thing the family cannot afford to assume. So every state below is reached by
//! running the command a user runs, and the marker set is **asserted**, never
//! arranged: every construction ends by reading itself back through
//! [`assert_state`]. That assertion lives in the **builder**, not only in the suite
//! that sweeps the axis, because a consumer building one member on a git whose
//! on-disk contract moved must redden at the build rather than be handed a
//! different state to drive a door against.
//!
//! The M51 per-axis review's posture auditor built eight of these by hand, in shell,
//! once ([baseline-posture.md](../../../../completions/artifacts/M52/baseline-posture.md)
//! §1). This module is that census turned into a substrate, with the twelfth member
//! (`bisect`, attached) derived rather than observed — without it the family's
//! acceptance cannot iterate the enum it claims to iterate.
//!
//! # The git it was proven on
//!
//! **git 2.54.0 (Apple Git-157).** Every fact in [`EXPECTATIONS`] — `REVERT_HEAD`,
//! `rebase-apply/applying` vs `rebase-apply/onto`, the clean squash merge writing
//! `SQUASH_MSG` and *no* `MERGE_MSG`, each `git ls-files -u` count — is that git's
//! on-disk contract. It is **declared, not fenced**: [decisions-pending.md](../../../../implementation/decisions-pending.md)
//! → *The rc.16 wave (M52)*, deferral **(a)**, whose trigger is *the first CI or
//! adopter report on another git major/minor where a marker cell diverges*. A red
//! here on another git is that report arriving, and the datum it carries is the cell
//! name in the assertion message.
//!
//! # Two entry points, one construction
//!
//! * [`GitStateRepo::build`] — a repo built from scratch, carrying a hand-made
//!   `.jigc/config/` project-layer marker and **no `jigc setup`**. This is the only
//!   shape [`GitState::Unborn`] can take: driven, `jigc setup` *births* HEAD on an
//!   unborn repository (0 → 1 commits, exit 0), so a corpus that has been set up has
//!   a commit by construction.
//! * [`overlay`] — the same construction driven over an already-built
//!   [`TrialCorpus`], so the state carries jigc's own installed `pre-commit` hook and
//!   whatever managed docs the corpus state holds.
//!
//! **The overlay is entered LAST** — after the corpus state and after any `jigc
//! start` — and it enforces that rather than asking for it: it refuses a dirty
//! worktree, because a rebase cannot even begin over one and a `git stash push`
//! would swallow the caller's edits. The reason it must be last is that Increment 3
//! makes every acting door *refuse* under these postures, so a builder that ran a
//! jigc door afterwards would be building against a door that has stopped answering.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use super::trial_corpus::{TrialCorpus, unique_root};

/// One git state a user can leave behind — the axis the posture family answers over.
///
/// The twelve are *distinguishable on disk*: two states that git leaves with the same
/// marker set, the same HEAD shape and the same index are one member here, not two
/// (which is why the detached-bisect cell is stated in [`GitState::Bisect`]'s
/// documentation rather than built).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitState {
    /// A conflicting `git merge` — `MERGE_HEAD` + `MERGE_MSG`, HEAD attached.
    Merge,
    /// A **clean** `git merge --squash` — `SQUASH_MSG` and nothing else. The state
    /// with a message to destroy and no `MERGE_HEAD` to notice it by; the conflicting
    /// form adds `MERGE_MSG` and unmerged entries, so the clean one is the harder
    /// cell and the one built here.
    SquashMerge,
    /// A conflicting `git rebase` on the **merge** backend (git's default) —
    /// `rebase-merge/`, HEAD detached.
    RebaseMerge,
    /// A conflicting `git rebase --apply` on the **apply** backend —
    /// `rebase-apply/` carrying `onto` and **not** `applying`, HEAD detached.
    RebaseApply,
    /// A failing `git am` — `rebase-apply/` carrying `applying` and **not** `onto`,
    /// HEAD **attached**. The sibling of [`GitState::RebaseApply`] git itself
    /// discriminates (it is how `git rebase --abort` can answer *"It looks like 'git
    /// am' is in progress"*), and the reason a builder that reached only one of the
    /// two would leave the family's ambiguity untested.
    Am,
    /// A conflicting single-commit `git cherry-pick` — `CHERRY_PICK_HEAD` +
    /// `MERGE_MSG`, **no** `sequencer/`.
    CherryPick,
    /// A conflicting **multi-commit** `git cherry-pick` — the same markers **plus**
    /// `sequencer/`, which is the whole point: the remaining picks are queued on
    /// disk, so concluding the operation is not one command's worth of work.
    Sequencer,
    /// A conflicting `git revert` — `REVERT_HEAD` + `MERGE_MSG`.
    Revert,
    /// A conflicted `git stash pop`: **unmerged index entries and no marker at all**.
    /// The state no marker-set widening can ever reach, and therefore the control
    /// that keeps *"detect the operation"* honest about its ceiling.
    UnmergedIndex,
    /// `git bisect start`, before any `good`/`bad` — `BISECT_LOG` + `BISECT_START`
    /// with HEAD still **attached**.
    ///
    /// The detached-bisect cell (`start; bad; good`) is **stated, not built**: it
    /// carries the identical marker set and differs only in HEAD, which
    /// [`GitState::RebaseMerge`] already supplies as an operation-under-a-detached-HEAD
    /// cell — and reaching it needs a history of at least four commits or git
    /// concludes the bisect immediately (driven).
    Bisect,
    /// `git switch --detach` — no operation, no markers, HEAD detached.
    Detached,
    /// `git init` with no commit — HEAD points at a branch that does not exist.
    /// Buildable **only** from scratch ([`GitState::overlay_refusal`]).
    Unborn,
}

impl GitState {
    /// Every state, in declaration order. A suite that iterates this picks up a new
    /// member with no edit — and a member added without an [`EXPECTATIONS`] row
    /// reddens rather than being skipped.
    pub const ALL: &'static [GitState] = &[
        GitState::Merge,
        GitState::SquashMerge,
        GitState::RebaseMerge,
        GitState::RebaseApply,
        GitState::Am,
        GitState::CherryPick,
        GitState::Sequencer,
        GitState::Revert,
        GitState::UnmergedIndex,
        GitState::Bisect,
        GitState::Detached,
        GitState::Unborn,
    ];

    /// The state's name, as a suite label and the rig spell it.
    pub fn name(self) -> &'static str {
        match self {
            GitState::Merge => "merge",
            GitState::SquashMerge => "squash-merge",
            GitState::RebaseMerge => "rebase-merge",
            GitState::RebaseApply => "rebase-apply",
            GitState::Am => "am",
            GitState::CherryPick => "cherry-pick",
            GitState::Sequencer => "sequencer",
            GitState::Revert => "revert",
            GitState::UnmergedIndex => "unmerged-index",
            GitState::Bisect => "bisect",
            GitState::Detached => "detached",
            GitState::Unborn => "unborn",
        }
    }

    /// Why this state cannot be overlaid on a built [`TrialCorpus`], when it cannot —
    /// the refusal's one home, so the suite asserts the *reason* rather than the fact
    /// that something went wrong.
    pub fn overlay_refusal(self) -> Option<&'static str> {
        match self {
            GitState::Unborn => Some(
                "`unborn` cannot be overlaid on a built corpus: every corpus this \
                 builder can overlay has been through `jigc setup`, which BIRTHS HEAD \
                 on an unborn repository (driven: 0 -> 1 commits, exit 0) — and \
                 deleting that commit afterwards would not make HEAD unborn again. \
                 Build it from scratch with `GitStateRepo::build(GitState::Unborn)`.",
            ),
            _ => None,
        }
    }
}

/// What HEAD looks like — the three shapes `git symbolic-ref` and `git rev-parse
/// --verify HEAD` jointly distinguish.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Head {
    /// On a branch that exists.
    Attached,
    /// No branch; a commit.
    Detached,
    /// On a branch that does not exist yet — no commit.
    Unborn,
}

/// Every marker this module knows how to look for, in the worktree's own git dir.
///
/// It is a **universe**, not a checklist: [`assert_state`] asserts every member a
/// state's [`Expectation`] does not name is **absent**, so the discriminating sibling
/// of each pair (`rebase-apply/onto` against `rebase-apply/applying`; `SQUASH_MSG`
/// against `MERGE_HEAD`; `sequencer/` against its absence on the one-commit pick) is
/// checked by the complement rather than by a hand-written "must not" list that can
/// silently fall behind a new member.
pub const MARKER_UNIVERSE: &[&str] = &[
    "MERGE_HEAD",
    "MERGE_MSG",
    "SQUASH_MSG",
    "CHERRY_PICK_HEAD",
    "REVERT_HEAD",
    "BISECT_LOG",
    "BISECT_START",
    "sequencer",
    "sequencer/todo",
    "rebase-merge",
    "rebase-apply",
    "rebase-apply/onto",
    "rebase-apply/applying",
];

/// What one built state must look like on disk — every field driven on git 2.54.0.
#[derive(Clone, Copy, Debug)]
pub struct Expectation {
    /// The entries that must be **present** in the worktree's git dir. Every other
    /// member of [`MARKER_UNIVERSE`] must be absent.
    pub markers: &'static [&'static str],
    /// HEAD's shape after the construction.
    pub head: Head,
    /// The exact number of `git ls-files -u` lines — three per conflicting path in
    /// these fixtures (stages 1/2/3: the seeded file exists in the merge base), and
    /// zero for the states that leave the index alone.
    pub unmerged: usize,
}

/// The driven expectation for every state — one row per [`GitState::ALL`] member.
///
/// A table rather than a `match` **on purpose**: the lookup ([`expectation`]) panics
/// for a member with no row, so a thirteenth state added to [`GitState::ALL`] and not
/// to this table fails loudly at the one place that could otherwise have skipped it.
/// A `match` would be compiler-checked but would also make *"a member carrying no
/// expectation"* unrepresentable — and the failure mode this guards is a member added
/// to the enum by one task and expected by another.
pub const EXPECTATIONS: &[(GitState, Expectation)] = &[
    (
        GitState::Merge,
        Expectation {
            markers: &["MERGE_HEAD", "MERGE_MSG"],
            head: Head::Attached,
            unmerged: 3,
        },
    ),
    (
        GitState::SquashMerge,
        // The driven correction to baseline-posture.md §1.1 row 2: a CLEAN
        // `git merge --squash` leaves `SQUASH_MSG` **alone**. The row's `MERGE_MSG`
        // belongs to the conflicting form.
        Expectation {
            markers: &["SQUASH_MSG"],
            head: Head::Attached,
            unmerged: 0,
        },
    ),
    (
        GitState::RebaseMerge,
        Expectation {
            markers: &["rebase-merge", "MERGE_MSG"],
            head: Head::Detached,
            unmerged: 3,
        },
    ),
    (
        GitState::RebaseApply,
        Expectation {
            markers: &["rebase-apply", "rebase-apply/onto"],
            head: Head::Detached,
            unmerged: 3,
        },
    ),
    (
        GitState::Am,
        // `git am` stops on the first patch that does not apply, and — without
        // `--3way` — applies nothing, so the index is clean while the operation is
        // very much in progress: the cell that refutes "unmerged entries" as a proxy
        // for "an operation is running".
        Expectation {
            markers: &["rebase-apply", "rebase-apply/applying"],
            head: Head::Attached,
            unmerged: 0,
        },
    ),
    (
        GitState::CherryPick,
        Expectation {
            markers: &["CHERRY_PICK_HEAD", "MERGE_MSG"],
            head: Head::Attached,
            unmerged: 3,
        },
    ),
    (
        GitState::Sequencer,
        Expectation {
            markers: &[
                "CHERRY_PICK_HEAD",
                "MERGE_MSG",
                "sequencer",
                "sequencer/todo",
            ],
            head: Head::Attached,
            unmerged: 3,
        },
    ),
    (
        GitState::Revert,
        Expectation {
            markers: &["REVERT_HEAD", "MERGE_MSG"],
            head: Head::Attached,
            unmerged: 3,
        },
    ),
    (
        GitState::UnmergedIndex,
        Expectation {
            markers: &[],
            head: Head::Attached,
            unmerged: 3,
        },
    ),
    (
        GitState::Bisect,
        Expectation {
            markers: &["BISECT_LOG", "BISECT_START"],
            head: Head::Attached,
            unmerged: 0,
        },
    ),
    (
        GitState::Detached,
        Expectation {
            markers: &[],
            head: Head::Detached,
            unmerged: 0,
        },
    ),
    (
        GitState::Unborn,
        Expectation {
            markers: &[],
            head: Head::Unborn,
            unmerged: 0,
        },
    ),
];

/// The expectation for `state` — a **hard panic**, never a skip, for a member that
/// carries none.
pub fn expectation(state: GitState) -> &'static Expectation {
    EXPECTATIONS
        .iter()
        .find(|(member, _)| *member == state)
        .map(|(_, expectation)| expectation)
        .unwrap_or_else(|| {
            panic!(
                "`GitState::{state:?}` ({}) carries no EXPECTATIONS row: a git state \
                 this builder can produce and no suite can assert is a state that \
                 ships unasserted. Add its driven row (markers, HEAD, `git ls-files \
                 -u` count) beside the others — do not skip it.",
                state.name(),
            )
        })
}

/// The repo-relative path every conflicting construction fights over.
const SHARED: &str = "posture-shared.txt";

/// The second commit's file on the two-commit pick — present so the `sequencer/`
/// queue has a *remaining* pick after the first one conflicts.
const SECOND: &str = "posture-second.txt";

/// The clean branch's file — the squash merge's payload, which must **not** collide
/// with anything on the current branch or the merge stops being clean.
const OTHER: &str = "posture-other.txt";

/// The branch every conflicting construction merges, rebases onto or picks from.
const THEIRS_BRANCH: &str = "posture-theirs";

/// The branch the clean squash merge takes.
const CLEAN_BRANCH: &str = "posture-clean";

/// A git runner bound to one repo and one `$HOME` — the seam every construction and
/// every assertion goes through, so no construction can read the developer's global
/// git config by accident.
struct Driver {
    repo: PathBuf,
    home: PathBuf,
}

impl Driver {
    fn run(&self, args: &[&str]) -> Output {
        Command::new("git")
            .args(args)
            .current_dir(&self.repo)
            .env("HOME", &self.home)
            .output()
            .expect("spawn git")
    }

    /// Run git and require success — the arm for a command whose failure would leave
    /// a *different* state than the one being built.
    fn ok(&self, args: &[&str]) -> String {
        let out = self.run(args);
        assert!(
            out.status.success(),
            "git {args:?} failed ({}) while building a git-state fixture:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    /// Run git and require **failure** — the arm for the command whose *conflict* is
    /// the fixture. A merge that succeeded would leave a clean tree and the suite
    /// would then assert markers that were never written; asserting the non-zero exit
    /// here names the real cause at the point it happens.
    fn must_conflict(&self, args: &[&str]) {
        let out = self.run(args);
        assert!(
            !out.status.success(),
            "git {args:?} SUCCEEDED while building a git-state fixture — the \
             construction must leave a conflict un-concluded, and a clean run leaves \
             no operation in progress at all.\n--- stdout ---\n{}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }

    fn write(&self, rel: &str, body: &str) {
        fs::write(self.repo.join(rel), body).unwrap_or_else(|e| panic!("write {rel}: {e}"));
    }

    fn commit(&self, paths: &[&str], message: &str) {
        for path in paths {
            self.ok(&["add", "--", path]);
        }
        self.ok(&["commit", "-q", "-m", message]);
    }

    /// The **worktree's own** git dir, absolute — where git writes the markers (a
    /// linked worktree's is `.git/worktrees/<name>/`, which is exactly why this asks
    /// git rather than joining `.git`).
    fn git_dir(&self) -> PathBuf {
        let raw = self.ok(&["rev-parse", "--git-dir"]);
        let path = PathBuf::from(&raw);
        if path.is_absolute() {
            path
        } else {
            self.repo.join(path)
        }
    }

    fn branch(&self) -> String {
        self.ok(&["rev-parse", "--abbrev-ref", "HEAD"])
    }
}

/// Commit the file the conflicting constructions fight over, and return the branch
/// HEAD is on.
fn seed(driver: &Driver) -> String {
    let branch = driver.branch();
    driver.write(SHARED, "alpha\n");
    driver.commit(&[SHARED], "posture fixture: seed");
    branch
}

/// Put a conflicting commit on [`THEIRS_BRANCH`] and a different one on `branch`, so
/// any of merge / rebase / pick / am over the pair conflicts.
fn diverge(driver: &Driver, branch: &str) {
    driver.ok(&["checkout", "-q", "-b", THEIRS_BRANCH]);
    driver.write(SHARED, "theirs\n");
    driver.commit(&[SHARED], "posture fixture: theirs");
    driver.ok(&["checkout", "-q", branch]);
    driver.write(SHARED, "ours\n");
    driver.commit(&[SHARED], "posture fixture: ours");
}

/// Drive `repo` into `state`. Every branch runs the command a user runs; nothing
/// writes a marker directly.
fn drive(driver: &Driver, state: GitState) {
    match state {
        GitState::Merge => {
            let branch = seed(driver);
            diverge(driver, &branch);
            driver.must_conflict(&["merge", THEIRS_BRANCH]);
        }
        GitState::SquashMerge => {
            let branch = seed(driver);
            driver.ok(&["checkout", "-q", "-b", CLEAN_BRANCH]);
            driver.write(OTHER, "other\n");
            driver.commit(&[OTHER], "posture fixture: the squashed branch");
            driver.ok(&["checkout", "-q", &branch]);
            // Clean by construction — `--squash` never updates HEAD, so the message
            // it parks in SQUASH_MSG is the byte at risk.
            driver.ok(&["merge", "--squash", CLEAN_BRANCH]);
        }
        GitState::RebaseMerge => {
            let branch = seed(driver);
            diverge(driver, &branch);
            driver.must_conflict(&["rebase", THEIRS_BRANCH]);
        }
        GitState::RebaseApply => {
            let branch = seed(driver);
            diverge(driver, &branch);
            driver.must_conflict(&["rebase", "--apply", THEIRS_BRANCH]);
        }
        GitState::Am => {
            let branch = seed(driver);
            diverge(driver, &branch);
            // The patch is written **inside the git dir**, not the worktree: a
            // fixture that left patch files in the tree would change what every
            // door under test sees as untracked.
            let out_dir = driver.git_dir().join("posture-patches");
            let printed = driver.ok(&[
                "format-patch",
                "-1",
                THEIRS_BRANCH,
                "-o",
                out_dir.to_str().expect("utf-8 fixture path"),
            ]);
            let patch = printed
                .lines()
                .next_back()
                .unwrap_or_else(|| panic!("`git format-patch` must print the file it wrote"))
                .trim()
                .to_string();
            driver.must_conflict(&["am", &patch]);
        }
        GitState::CherryPick => {
            let branch = seed(driver);
            diverge(driver, &branch);
            driver.must_conflict(&["cherry-pick", THEIRS_BRANCH]);
        }
        GitState::Sequencer => {
            let branch = seed(driver);
            driver.ok(&["checkout", "-q", "-b", THEIRS_BRANCH]);
            driver.write(SHARED, "theirs\n");
            driver.commit(&[SHARED], "posture fixture: theirs one");
            driver.write(SECOND, "second\n");
            driver.commit(&[SECOND], "posture fixture: theirs two");
            driver.ok(&["checkout", "-q", &branch]);
            driver.write(SHARED, "ours\n");
            driver.commit(&[SHARED], "posture fixture: ours");
            // Two picks, the FIRST of which conflicts — so the second stays queued
            // in `sequencer/todo` rather than being replayed.
            driver.must_conflict(&["cherry-pick", &format!("{THEIRS_BRANCH}~1"), THEIRS_BRANCH]);
        }
        GitState::Revert => {
            seed(driver);
            driver.write(SHARED, "beta\n");
            driver.commit(&[SHARED], "posture fixture: the revert target");
            let target = driver.ok(&["rev-parse", "HEAD"]);
            driver.write(SHARED, "gamma\n");
            driver.commit(&[SHARED], "posture fixture: past the revert target");
            driver.must_conflict(&["revert", "--no-edit", &target]);
        }
        GitState::UnmergedIndex => {
            seed(driver);
            driver.write(SHARED, "local\n");
            driver.ok(&["stash", "push", "-q", "-m", "posture fixture: a local edit"]);
            driver.write(SHARED, "committed\n");
            driver.commit(&[SHARED], "posture fixture: moved under the stash");
            driver.must_conflict(&["stash", "pop"]);
        }
        GitState::Bisect => {
            seed(driver);
            driver.ok(&["bisect", "start"]);
        }
        GitState::Detached => {
            seed(driver);
            driver.ok(&["switch", "-q", "--detach", "HEAD"]);
        }
        GitState::Unborn => panic!(
            "`GitState::Unborn` carries no construction: it IS the absence of one. \
             The from-scratch entry point reaches it by not committing; `overlay` \
             refuses it ({}).",
            GitState::Unborn
                .overlay_refusal()
                .expect("Unborn declares its overlay refusal"),
        ),
    }
}

/// A from-scratch repo driven into one [`GitState`], in a throwaway directory that
/// removes itself on drop.
///
/// It holds a `repo/` (the git repo under test) and a `home/` (the `$HOME` every
/// child sees), on [`TrialCorpus`]'s shape — so a caller can point a `jigc`
/// invocation at the pair the same way.
///
/// **It is deliberately *not* set up.** `.jigc/config/` is created by hand, because
/// that directory's existence is the whole of what `cli::locate` reads as the project
/// cascade layer — and running `jigc setup` to obtain it would commit, which
/// [`GitState::Unborn`] cannot survive. A door that needs a *real* workbench takes the
/// [`overlay`] entry point instead.
pub struct GitStateRepo {
    root: PathBuf,
    state: GitState,
}

impl GitStateRepo {
    /// Build `state` from scratch.
    pub fn build(state: GitState) -> Self {
        let root = unique_root(&format!("git-state-{}", state.name()));
        let repo = root.join("repo");
        let home = root.join("home");
        fs::create_dir_all(&repo).expect("create the fixture repo dir");
        fs::create_dir_all(&home).expect("create the fixture home dir");

        let driver = Driver {
            repo: repo.clone(),
            home,
        };
        driver.ok(&["init", "-q"]);
        driver.ok(&["config", "user.email", "posture@example.com"]);
        driver.ok(&["config", "user.name", "Posture Fixture"]);
        // The project-layer marker, by hand: `cli::locate` reads the presence of
        // `.jigc/config/` and nothing else, and `jigc setup` is unavailable here.
        fs::create_dir_all(repo.join(".jigc").join("config"))
            .expect("create the hand-made project-layer marker");

        if state != GitState::Unborn {
            fs::write(repo.join("README.md"), "git-state fixture\n").expect("write README.md");
            driver.commit(&["README.md"], "initial");
            drive(&driver, state);
        }

        // The construction asserts itself, here rather than only in the suite that
        // sweeps the axis: a consumer building one member on a git whose on-disk
        // contract moved must redden AT THE BUILD, not be handed a different state
        // to drive a door against.
        assert_state(&repo, &driver.home, state);
        GitStateRepo { root, state }
    }

    /// The git repo under test.
    pub fn repo(&self) -> PathBuf {
        self.root.join("repo")
    }

    /// The `$HOME` every child process sees.
    pub fn home(&self) -> PathBuf {
        self.root.join("home")
    }

    /// The state this repo was built as.
    pub fn state(&self) -> GitState {
        self.state
    }
}

impl Drop for GitStateRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// Drive an already-built [`TrialCorpus`] into `state` — the entry point that carries
/// jigc's own installed `pre-commit` hook and the corpus's managed docs into the
/// posture.
///
/// **Enter it last.** It returns `Err` for a state that cannot be reached from a
/// set-up corpus (today: [`GitState::Unborn`], with [`GitState::overlay_refusal`]'s
/// reason), and it **panics** on a dirty worktree, which is a builder error rather
/// than a state: `git rebase` refuses to start over one, and `git stash push` would
/// swallow the caller's uncommitted work.
pub fn overlay(corpus: &TrialCorpus, state: GitState) -> Result<(), String> {
    if let Some(reason) = state.overlay_refusal() {
        return Err(reason.to_string());
    }
    let driver = Driver {
        repo: corpus.repo(),
        home: corpus.home(),
    };
    let dirty = driver.ok(&["status", "--porcelain"]);
    assert!(
        dirty.is_empty(),
        "refusing to overlay the `{}` git state on a DIRTY worktree — a git state is \
         entered LAST, after the corpus state and any `jigc start`. `git rebase` \
         refuses to begin over uncommitted changes and `git stash push` would swallow \
         them. Outstanding:\n{dirty}",
        state.name(),
    );
    drive(&driver, state);
    assert_state(&driver.repo, &driver.home, state);
    Ok(())
}

/// Assert that the repo at `repo` really is in `state` — the half that makes a built
/// fixture a *measurement* rather than an intention.
///
/// Four facts per state, all read back from git or from the git dir git wrote: the
/// expected markers are present, **every other member of [`MARKER_UNIVERSE`] is
/// absent** (which is where each discriminating sibling is checked), HEAD's shape,
/// and the `git ls-files -u` count.
pub fn assert_state(repo: &Path, home: &Path, state: GitState) {
    let driver = Driver {
        repo: repo.to_path_buf(),
        home: home.to_path_buf(),
    };
    let expectation = expectation(state);
    let git_dir = driver.git_dir();
    let label = state.name();

    for marker in expectation.markers {
        assert!(
            git_dir.join(marker).exists(),
            "the `{label}` fixture must carry `{marker}` in {} — git 2.54.0 writes it \
             for this state (deferral (a): another git that does not is the report \
             that deferral is waiting for)",
            git_dir.display(),
        );
    }
    for marker in MARKER_UNIVERSE
        .iter()
        .filter(|marker| !expectation.markers.contains(marker))
    {
        assert!(
            !git_dir.join(marker).exists(),
            "the `{label}` fixture must NOT carry `{marker}`: it is the discriminating \
             sibling of this state's marker set, and a fixture carrying both proves \
             nothing about a door that keys on either",
        );
    }

    let symbolic = driver.run(&["symbolic-ref", "-q", "HEAD"]).status.success();
    let has_commit = driver
        .run(&["rev-parse", "--verify", "--quiet", "HEAD"])
        .status
        .success();
    let observed = match (symbolic, has_commit) {
        (true, true) => Head::Attached,
        (false, true) => Head::Detached,
        (true, false) => Head::Unborn,
        (false, false) => panic!(
            "the `{label}` fixture left HEAD both unresolvable and unborn — git wrote \
             a shape this module has no name for"
        ),
    };
    assert_eq!(
        observed, expectation.head,
        "the `{label}` fixture must leave HEAD {:?}",
        expectation.head,
    );

    let unmerged = driver
        .ok(&["ls-files", "-u"])
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    assert_eq!(
        unmerged, expectation.unmerged,
        "the `{label}` fixture must leave {} unmerged index entries",
        expectation.unmerged,
    );
}
