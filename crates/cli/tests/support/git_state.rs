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
//! §1). This module is that census turned into a substrate, with two members derived
//! rather than observed — `bisect` (attached) and `dangling-sequencer` — because
//! without them the family's acceptance cannot iterate the enum it claims to iterate:
//! `dangling-sequencer` is the only cell in which [`cli::repo::InProgress::Sequencer`]
//! is the operation rather than a property of a live pick or revert.
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

use cli::repo::InProgress;

use super::trial_corpus::{TrialCorpus, unique_root};

/// One git state a user can leave behind — the axis the posture family answers over.
///
/// The members are *distinguishable on disk*: two states that git leaves with the same
/// marker set, the same HEAD shape and the same index are one member here, not two
/// (which is why the detached-bisect cell is stated in [`GitState::Bisect`]'s
/// documentation rather than built).
///
/// **One deliberate exception, stated rather than left to be noticed** (M53 Increment
/// 4): [`GitState::UncommittedPick`] and [`GitState::UncommittedPickRange`] leave the
/// *identical* marker set, HEAD shape and index, and are two members anyway. What the
/// range cell carries is a claim about the **construction** rather than about the disk
/// — a multi-commit `cherry-pick --no-commit` queues **no `sequencer/`** — and the only
/// way that fact stays true is to build the range and assert the absence. Collapsing it
/// into the one-commit cell would delete an assertion, not deduplicate a fixture.
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
    /// The same multi-commit pick with its **first** pick concluded by hand:
    /// `CHERRY_PICK_HEAD` and `MERGE_MSG` are consumed by that commit and
    /// `sequencer/` is left holding the picks git never ran.
    ///
    /// It is the state the M52 baseline caught **jigc's own exit-0 finalize
    /// producing** ([baseline-posture.md](../../../../completions/artifacts/M52/baseline-posture.md)
    /// §2.7 / L5), and the only cell in which [`cli::repo::InProgress::Sequencer`]
    /// answers at all: while `CHERRY_PICK_HEAD` or `REVERT_HEAD` is still there the
    /// pick or the revert is the operation, and the queue is one of its properties.
    DanglingSequencer,
    /// A **clean** one-commit `git cherry-pick --no-commit` — `MERGE_MSG` and nothing
    /// else, HEAD attached, the index clean of conflicts.
    ///
    /// The cell with no conflict anywhere, so a conflict-shaped route would be a lie in
    /// it; `--no-commit` writes **no `CHERRY_PICK_HEAD`**, which is why every
    /// marker-keyed member of the family misses the state (M53 Increment 4).
    UncommittedPick,
    /// The same pick over a **range** — several commits applied in one `--no-commit`
    /// pick, and **no `sequencer/`**: nothing on disk distinguishes it from the
    /// single-commit cell, which is the datum that makes it a cell rather than a
    /// variation (driven).
    UncommittedPickRange,
    /// A **conflicting** `git cherry-pick --no-commit`, left as git left it —
    /// `MERGE_MSG` with three unmerged index entries and still no `CHERRY_PICK_HEAD`.
    ///
    /// The cell the family answered as a bare [`cli::repo::InProgress::UnmergedIndex`]
    /// before the tenth member existed, and routed at `git reset --merge` — the one
    /// command that throws the picked bytes away.
    UncommittedPickConflicted,
    /// …and the same pick after the user resolved it with `git add`: the index is clean
    /// again while `MERGE_MSG` still stands, so **nothing but that file** says the pick
    /// is un-concluded — the cell no index probe can reach.
    UncommittedPickResolved,
    /// A conflicting `git revert` — `REVERT_HEAD` + `MERGE_MSG`.
    ///
    /// Driven on git 2.54.0: a `git revert --no-commit` writes `REVERT_HEAD` **clean or
    /// conflicting**, unlike `cherry-pick --no-commit`, so an uncommitted revert is this
    /// member and never the uncommitted-pick one above.
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
        GitState::DanglingSequencer,
        GitState::UncommittedPick,
        GitState::UncommittedPickRange,
        GitState::UncommittedPickConflicted,
        GitState::UncommittedPickResolved,
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
            GitState::DanglingSequencer => "dangling-sequencer",
            GitState::UncommittedPick => "uncommitted-pick",
            GitState::UncommittedPickRange => "uncommitted-pick-range",
            GitState::UncommittedPickConflicted => "uncommitted-pick-conflicted",
            GitState::UncommittedPickResolved => "uncommitted-pick-resolved",
            GitState::Revert => "revert",
            GitState::UnmergedIndex => "unmerged-index",
            GitState::Bisect => "bisect",
            GitState::Detached => "detached",
            GitState::Unborn => "unborn",
        }
    }

    /// **The operation the posture family must name in this state** — `None` for the
    /// two members that are *postures* rather than operations
    /// ([`cli::repo::PostureMember::HeadDetached`] / `HeadUnborn`: there is nothing to
    /// conclude and no git command to route at).
    ///
    /// The map is **declared, not derived**: it is the fixture's claim about what the
    /// probe owes in the state the fixture just built, so a suite that derived it from
    /// the probe would assert the probe against itself. Every cell was driven on git
    /// 2.54.0 with the same census that produced [`EXPECTATIONS`].
    ///
    /// Two cells carry the discriminations the family exists for, and both are the
    /// probe's **order** rather than its predicates:
    ///
    ///   * [`GitState::Sequencer`] answers `CherryPick`, not `Sequencer` — a
    ///     multi-commit pick is a pick *and* a queue, and `git cherry-pick --abort`
    ///     (driven, exit 0) clears both. `Sequencer` is what is left when the pick
    ///     itself is gone, which is [`GitState::DanglingSequencer`].
    ///   * [`GitState::Merge`], `CherryPick`, `Revert`, `RebaseMerge` and
    ///     `RebaseApply` all leave **unmerged index entries** too, and each answers
    ///     its own operation rather than `UnmergedIndex`: that member is the probe's
    ///     last question, asked only where no operation marker answered — the
    ///     conflicted `git stash pop` no marker-set widening can reach.
    ///   * the **four** uncommitted-pick cells answer one member
    ///     ([`cli::repo::InProgress::UncommittedCherryPick`]) and are four states
    ///     rather than one because what a route *claims* differs across them, not
    ///     because the disk does: a clean pick has no conflicts to resolve, so the
    ///     shared *"once its conflicts are resolved"* would be false there, and the
    ///     conflicted one must not be told `git reset --merge`, which is where it fell
    ///     before the member existed. Two of the four differ on disk only in the `git
    ///     ls-files -u` count, and two not at all — see the enum's stated exception.
    pub fn in_progress(self) -> Option<InProgress> {
        match self {
            GitState::Merge => Some(InProgress::Merge),
            GitState::SquashMerge => Some(InProgress::SquashMerge),
            GitState::RebaseMerge | GitState::RebaseApply => Some(InProgress::Rebase),
            GitState::Am => Some(InProgress::Am),
            GitState::CherryPick | GitState::Sequencer => Some(InProgress::CherryPick),
            GitState::DanglingSequencer => Some(InProgress::Sequencer),
            GitState::UncommittedPick
            | GitState::UncommittedPickRange
            | GitState::UncommittedPickConflicted
            | GitState::UncommittedPickResolved => Some(InProgress::UncommittedCherryPick),
            GitState::Revert => Some(InProgress::Revert),
            GitState::UnmergedIndex => Some(InProgress::UnmergedIndex),
            GitState::Bisect => Some(InProgress::Bisect),
            GitState::Detached | GitState::Unborn => None,
        }
    }

    /// Why this state cannot be built inside a **provisioned fan-out worktree**, when it
    /// cannot — [`overlay_worktree`]'s refusal, stated per member rather than left as a
    /// silent skip.
    pub fn worktree_refusal(self) -> Option<&'static str> {
        match self {
            GitState::Unborn => Some(
                "`unborn` cannot occur in a fan-out worktree: `git worktree add --detach \
                 <base>` creates the checkout AT a commit, so its HEAD resolves from the \
                 moment it exists. `cli::repo::PostureSubject::adjudicates` states the \
                 same fact on the production side.",
            ),
            _ => None,
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
/// for a member with no row, so a state added to [`GitState::ALL`] and not
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
        GitState::DanglingSequencer,
        // The hand-made commit consumes `CHERRY_PICK_HEAD` **and** `MERGE_MSG` and
        // resolves the index, leaving the queue alone: the one state whose whole
        // evidence is `sequencer/`, which is why it is the cell that proves
        // `InProgress::Sequencer` answers anything at all.
        Expectation {
            markers: &["sequencer", "sequencer/todo"],
            head: Head::Attached,
            unmerged: 0,
        },
    ),
    // The four uncommitted-pick cells. `--no-commit` leaves `MERGE_MSG` and nothing
    // else this module looks for: `CHERRY_PICK_HEAD` is **not** written, which is the
    // fact the tenth member exists for, and it is asserted here by the complement over
    // `MARKER_UNIVERSE` rather than by a "must not" list. `AUTO_MERGE` is written too
    // and is deliberately outside that universe — a clean `git stash apply` and a
    // concluded rebase leave it as well, so it discriminates nothing.
    (
        GitState::UncommittedPick,
        Expectation {
            markers: &["MERGE_MSG"],
            head: Head::Attached,
            unmerged: 0,
        },
    ),
    (
        GitState::UncommittedPickRange,
        // Driven: a multi-commit `--no-commit` pick writes **no `sequencer/`** — git
        // applies the range in one go and has no queue to park, so this cell is
        // byte-indistinguishable on disk from the one above.
        Expectation {
            markers: &["MERGE_MSG"],
            head: Head::Attached,
            unmerged: 0,
        },
    ),
    (
        GitState::UncommittedPickConflicted,
        Expectation {
            markers: &["MERGE_MSG"],
            head: Head::Attached,
            unmerged: 3,
        },
    ),
    (
        GitState::UncommittedPickResolved,
        // The user's `git add` clears the index and leaves `MERGE_MSG` standing: the
        // cell where the only evidence of an un-concluded pick is that one file.
        Expectation {
            markers: &["MERGE_MSG"],
            head: Head::Attached,
            unmerged: 0,
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

/// Queue **two** picks whose first conflicts, so the second stays in `sequencer/todo`
/// rather than being replayed — the shared prelude of [`GitState::Sequencer`] and
/// [`GitState::DanglingSequencer`], which differ only in what the user does next.
fn queue_two_picks(driver: &Driver) {
    let branch = seed(driver);
    driver.ok(&["checkout", "-q", "-b", THEIRS_BRANCH]);
    driver.write(SHARED, "theirs\n");
    driver.commit(&[SHARED], "posture fixture: theirs one");
    driver.write(SECOND, "second\n");
    driver.commit(&[SECOND], "posture fixture: theirs two");
    driver.ok(&["checkout", "-q", &branch]);
    driver.write(SHARED, "ours\n");
    driver.commit(&[SHARED], "posture fixture: ours");
    driver.must_conflict(&["cherry-pick", &format!("{THEIRS_BRANCH}~1"), THEIRS_BRANCH]);
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
        GitState::Sequencer => queue_two_picks(driver),
        GitState::DanglingSequencer => {
            queue_two_picks(driver);
            // The user concludes the FIRST pick by hand. git consumes
            // `CHERRY_PICK_HEAD` and `MERGE_MSG` with that commit and leaves
            // `sequencer/` holding the pick it never ran — the exact residue jigc's
            // own exit-0 `task finalize` left at the M52 baseline (§2.7 / L5), where
            // the recovery `git status` then advertises destroys jigc's commit.
            driver.write(SHARED, "resolved\n");
            driver.commit(&[SHARED], "posture fixture: the pick, concluded by hand");
        }
        // The two clean cells pick a commit that touches a file nothing on this branch
        // has, so `git cherry-pick -n` exits 0 and goes through the asserting runner.
        GitState::UncommittedPick => {
            let branch = seed(driver);
            driver.ok(&["checkout", "-q", "-b", CLEAN_BRANCH]);
            driver.write(OTHER, "other\n");
            driver.commit(&[OTHER], "posture fixture: the picked commit");
            driver.ok(&["checkout", "-q", &branch]);
            driver.ok(&["cherry-pick", "-n", CLEAN_BRANCH]);
        }
        GitState::UncommittedPickRange => {
            let branch = seed(driver);
            driver.ok(&["checkout", "-q", "-b", CLEAN_BRANCH]);
            driver.write(OTHER, "other\n");
            driver.commit(&[OTHER], "posture fixture: the first picked commit");
            driver.write(SECOND, "second\n");
            driver.commit(&[SECOND], "posture fixture: the second picked commit");
            driver.ok(&["checkout", "-q", &branch]);
            // Two commits in one pick — and, driven, no `sequencer/`: the range is
            // applied in one go rather than queued.
            driver.ok(&["cherry-pick", "-n", &format!("{branch}..{CLEAN_BRANCH}")]);
        }
        GitState::UncommittedPickConflicted => {
            let branch = seed(driver);
            diverge(driver, &branch);
            driver.must_conflict(&["cherry-pick", "-n", THEIRS_BRANCH]);
        }
        GitState::UncommittedPickResolved => {
            let branch = seed(driver);
            diverge(driver, &branch);
            driver.must_conflict(&["cherry-pick", "-n", THEIRS_BRANCH]);
            // The user resolves the conflict and stages it — and stops there. git
            // clears the index and leaves `MERGE_MSG`, so the pick is still
            // un-concluded with nothing but that file to say so.
            driver.write(SHARED, "resolved\n");
            driver.ok(&["add", "--", SHARED]);
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
        // `-b main` for the same reason as the identity two lines down: no part of a
        // posture fixture may come from ambient git config. A bare `git init` names its
        // branch from `init.defaultBranch` — set on a developer machine, unset on a CI
        // runner — and `Unborn` is *defined* by the branch HEAD points at not existing,
        // so that name must not vary by machine either.
        driver.ok(&["init", "-q", "-b", "main", "."]);
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
/// Drive a **provisioned fan-out worktree** into `state` — the third entry point onto the
/// one construction (M53 post-review fix, 2026-09-22; the M53 per-axis review, axis 2
/// `DEFECT 1`).
///
/// The posture family's subject is a path, and `jigc milestone finalize` commits from a
/// path the caller is not standing in: each sub-task's `.jigc/worktrees/<id>` checkout,
/// whose index the boundary reads with `git diff --cached`. Proving the door refuses there
/// needs the states built **inside** that checkout, which neither existing entry point can
/// do — [`GitStateRepo::build`] makes its own repository and [`overlay`] drives a
/// [`TrialCorpus`]'s main checkout.
///
/// **It attaches HEAD first, and that is the one adaptation.** `git worktree add --detach`
/// leaves HEAD on a commit, and every construction in [`drive`] moves between branches by
/// name (`seed` reads the current branch back and `diverge` returns to it), so a detached
/// start would send them to a commit rather than to the branch they left. The attach is
/// inert with respect to what is under test: [`cli::repo::InProgress`] is decided by what
/// git wrote into the worktree's own git dir and by `git ls-files -u`, neither of which
/// reads HEAD's shape, and `cli::repo::posture_subject` classifies a worktree by its
/// `.git` file, its path and the sub-task registry — never by HEAD. The one member HEAD's
/// shape *does* decide, [`cli::repo::PostureMember::HeadDetached`], is the member a
/// dedicated worktree is exempt from anyway.
///
/// **One worktree per repository at a time — asserted, not only stated** (the independent
/// review of `3c71da87`, LOW 3). [`drive`] names its branches from module constants
/// ([`THEIRS_BRANCH`], [`CLEAN_BRANCH`]), and git refuses to check one branch out in two
/// worktrees of the same repository — so a fixture driving two sub-task worktrees into
/// conflicting states at once would fail **at git**, several frames from the cause. The
/// precondition is now a named refusal at the door: no branch this construction is about to
/// claim may already be checked out in another worktree of this repository. A fixture that
/// needs two driven worktrees must parameterize those constants first, and the refusal says
/// so.
///
/// Returns `Err` for a state that cannot occur in a worktree at all
/// ([`GitState::worktree_refusal`]), and **panics** on a dirty worktree for the same
/// reason [`overlay`] does.
pub fn overlay_worktree(worktree: &Path, home: &Path, state: GitState) -> Result<(), String> {
    if let Some(reason) = state.worktree_refusal() {
        return Err(reason.to_string());
    }
    let driver = Driver {
        repo: worktree.to_path_buf(),
        home: home.to_path_buf(),
    };
    let dirty = driver.ok(&["status", "--porcelain"]);
    assert!(
        dirty.is_empty(),
        "refusing to drive the fan-out worktree {worktree:?} into the `{}` git state over \
         a DIRTY tree — the state is entered LAST, before the sub-task's own work is \
         staged. Outstanding:\n{dirty}",
        state.name(),
    );
    // A branch of this worktree's own, named after it so two worktrees of one repository
    // never ask git to check the same branch out twice.
    let branch = format!(
        "posture-wt-{}",
        worktree
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a fan-out worktree path ends in its sub-task id"),
    );
    refuse_if_a_construction_branch_is_taken(&driver, worktree, state);
    driver.ok(&["switch", "-q", "-c", &branch]);
    drive(&driver, state);
    assert_state(worktree, home, state);
    Ok(())
}

/// The one-worktree-per-repository precondition [`overlay_worktree`] documents, as a check.
///
/// [`drive`] names its branches from module constants, and the constructions that use them
/// **create** them ([`diverge`]'s `git checkout -b posture-theirs`, the squash merge's
/// `posture-clean`). A second fixture worktree driven into a branch-using state in the same
/// repository therefore dies inside `drive` at *"a branch named 'posture-theirs' already
/// exists"* — a message about branches, several frames from the fixture that caused it.
/// Asked here instead, naming the branch and the edit that lifts the bound.
///
/// **Existence, not checked-out-ness.** The prose this replaces said git refuses one branch
/// in two worktrees, and that is true but is not the failure: `diverge` returns to the
/// branch it started on, so the constant is left *existing and unchecked-out*, and the next
/// worktree trips on the create rather than on the checkout.
fn refuse_if_a_construction_branch_is_taken(driver: &Driver, worktree: &Path, state: GitState) {
    for branch in [THEIRS_BRANCH, CLEAN_BRANCH] {
        let exists = driver
            .run(&[
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("refs/heads/{branch}"),
            ])
            .status
            .success();
        assert!(
            !exists,
            "refusing to drive {worktree:?} into `{}`: this repository already carries the \
             construction branch `{branch}`, so something here has been driven into a git \
             state already. `drive` names its branches from module constants, so exactly one \
             worktree per repository can be driven at a time — parameterize `THEIRS_BRANCH` \
             and `CLEAN_BRANCH` before a fixture drives two.",
            state.name(),
        );
    }
}

/// [`overlay_worktree`], **leaving HEAD exactly as jigc provisioned it — detached** (the
/// independent review of `3c71da87`, LOW 2).
///
/// Its sibling attaches a branch first and states that as its one adaptation, on the ground
/// that HEAD's shape decides no [`cli::repo::InProgress`] member. That is true of what the
/// probe **detects** and false of what it **returns**: `cli::repo::posture` yields every
/// breach it finds, so a production fan-out worktree — always `--detach`ed — answers with
/// **two**, `OperationInProgress` and `HeadDetached`, and which one a door refuses with is
/// decided by probe order plus the `Dedicated` exemption. An attached fixture probes a
/// single-breach subject and cannot see that ordering at all, so every refusing cell of the
/// axis was one breach short of production.
///
/// **It accepts only a state `drive` can build with no branch to move between** — today
/// [`GitState::Bisect`], whose construction is one commit and `git bisect start`; every
/// other construction moves between branches by name (`seed` reads the current branch back,
/// `diverge` returns to it), and on a detached HEAD those go to a commit rather than to the
/// branch they left. The rest return `Err` with that reason rather than being driven into a
/// fixture that fails at git.
pub fn overlay_worktree_detached(
    worktree: &Path,
    home: &Path,
    state: GitState,
) -> Result<(), String> {
    if let Some(reason) = state.worktree_refusal() {
        return Err(reason.to_string());
    }
    if state != GitState::Bisect {
        return Err(format!(
            "`{}` cannot be built on a detached HEAD: every construction but `bisect` moves \
             between branches by name, and on a detached HEAD those land on a commit \
             rather than on the branch they left",
            state.name(),
        ));
    }
    let driver = Driver {
        repo: worktree.to_path_buf(),
        home: home.to_path_buf(),
    };
    assert!(
        !driver.run(&["symbolic-ref", "-q", "HEAD"]).status.success(),
        "refusing to build the detached-HEAD cell in {worktree:?}: its HEAD is ATTACHED, so \
         the fixture would prove the opposite of what it exists for. jigc provisions \
         every fan-out worktree `--detach`; pass the worktree as `provision` left it.",
    );
    let dirty = driver.ok(&["status", "--porcelain"]);
    assert!(
        dirty.is_empty(),
        "refusing to drive the fan-out worktree {worktree:?} into the `{}` git state over \
         a DIRTY tree — the state is entered LAST, before the sub-task's own work is \
         staged. Outstanding:\n{dirty}",
        state.name(),
    );
    refuse_if_a_construction_branch_is_taken(&driver, worktree, state);
    drive(&driver, state);
    assert_state_with_head(worktree, home, state, Head::Detached);
    Ok(())
}

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
    assert_state_with_head(repo, home, state, expectation(state).head);
}

/// [`assert_state`], with the HEAD shape supplied rather than read off the state's
/// [`Expectation`] — the one fact an entry point may legitimately change.
///
/// [`overlay_worktree_detached`] is the caller that needs it: its whole subject is a state
/// built **without** the branch attach its sibling performs, so the marker set is the
/// state's and the HEAD shape is the entry point's.
pub fn assert_state_with_head(repo: &Path, home: &Path, state: GitState, head: Head) {
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
        observed, head,
        "the `{label}` fixture must leave HEAD {head:?}",
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
