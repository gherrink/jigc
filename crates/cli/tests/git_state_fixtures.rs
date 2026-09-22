//! M52 Increment 2 / T1 — **the git-in-progress fixture builder**, and the fence that
//! makes it a measurement.
//!
//! The posture family (M52 Increment 3) refuses under every git operation a user has
//! not concluded, and its acceptance iterates an enum. This suite is what lets that
//! enum be trusted: every member of [`GitState::ALL`] is **built by driving real
//! git** and then **read back from the git dir git wrote**, so the fixture's claim
//! ("this is a rebase on the apply backend") is asserted rather than asserted-by-name.
//!
//! Three arms:
//!
//!   (1) **Every member carries a driven expectation** — the table lookup is a hard
//!       panic, never a skip, so a state this builder can produce and no suite can
//!       assert cannot ship.
//!   (2) **From scratch** — the entry point that carries a hand-made `.jigc/config/`
//!       project-layer marker and no `jigc setup`, which is the only shape
//!       [`GitState::Unborn`] can take.
//!   (3) **Overlaid on a built [`TrialCorpus`]** — the same constructions over
//!       a corpus carrying jigc's own installed `pre-commit` hook, with
//!       [`GitState::Unborn`] **refusing** and saying why. Every fixture commit here
//!       runs that hook, so the arm also answers whether jigc's own backstop tolerates
//!       the constructions.
//!
//! The marker facts are git 2.54.0's on-disk contract — declared, not fenced
//! ([decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.16
//! wave (M52)*, deferral **(a)**). A red on another git is the report that deferral
//! names its trigger on, and the assertion message carries the diverging cell.

use std::collections::BTreeSet;

use crate::support::git_state::{
    EXPECTATIONS, GitState, GitStateRepo, MARKER_UNIVERSE, assert_state, expectation, overlay,
};
use crate::support::trial_corpus::{State, TrialCorpus};

/// (1) Every member of the enum carries a row, every row names a member, and every
/// marker a row expects is one the absence sweep knows about.
///
/// The third clause is the one that keeps the *discriminating sibling* check honest:
/// [`assert_state`] asserts the complement of a row's markers against
/// [`MARKER_UNIVERSE`], so a marker named in a row but missing from the universe
/// would be checked for presence and never for absence anywhere else — a typo would
/// weaken the sweep silently instead of reddening.
#[test]
fn every_git_state_carries_a_driven_expectation() {
    let universe: BTreeSet<&str> = MARKER_UNIVERSE.iter().copied().collect();
    assert_eq!(
        universe.len(),
        MARKER_UNIVERSE.len(),
        "MARKER_UNIVERSE must not repeat a marker",
    );

    for state in GitState::ALL {
        // A hard panic for a member with no row — the lookup's whole point.
        let expected = expectation(*state);
        for marker in expected.markers {
            assert!(
                universe.contains(marker),
                "the `{}` row expects `{marker}`, which is not in MARKER_UNIVERSE — so \
                 no other state asserts it ABSENT, and the sweep that checks \
                 discriminating siblings has a hole exactly here",
                state.name(),
            );
        }
    }

    let members: BTreeSet<&str> = GitState::ALL.iter().map(|state| state.name()).collect();
    assert_eq!(
        members.len(),
        GitState::ALL.len(),
        "GitState::ALL must not repeat a member",
    );
    let rows: BTreeSet<&str> = EXPECTATIONS.iter().map(|(state, _)| state.name()).collect();
    assert_eq!(
        rows.len(),
        EXPECTATIONS.len(),
        "EXPECTATIONS must not carry two rows for one state — the lookup takes the \
         first, so the second would be dead and unreadable",
    );
    assert_eq!(
        rows, members,
        "EXPECTATIONS and GitState::ALL must name the same set of states",
    );
}

/// (2) The from-scratch entry point builds every declared member, and each one really
/// is the state it names.
///
/// It also asserts what the entry point is *for*: a project-layer marker that exists
/// without `jigc setup` having run — the only shape `unborn` can take, since driven,
/// `setup` births HEAD on an unborn repository (0 → 1 commits, exit 0).
#[test]
fn from_scratch_builds_every_git_state() {
    for state in GitState::ALL {
        let fixture = GitStateRepo::build(*state);
        assert_state(&fixture.repo(), &fixture.home(), *state);

        assert!(
            fixture.repo().join(".jigc").join("config").is_dir(),
            "the `{}` from-scratch fixture must carry the hand-made `.jigc/config/` \
             project-layer marker",
            state.name(),
        );
        assert!(
            !fixture
                .repo()
                .join(".git")
                .join("hooks")
                .join("pre-commit")
                .exists(),
            "the `{}` from-scratch fixture must NOT have been through `jigc setup` — \
             setup installs the `pre-commit` hook AND commits, and a commit is exactly \
             what `unborn` cannot survive",
            state.name(),
        );
    }
}

/// (3) The overlay carries every overlay-capable state onto a built corpus that
/// carries jigc's own `pre-commit` hook — and refuses `unborn` with its reason.
///
/// One corpus is built and **copied** per member (`State::Fresh` provisions no
/// worktrees, so copying is sound): the constructions commit, and a shared corpus
/// would carry one member's branches into the next.
#[test]
fn overlay_builds_every_overlay_capable_state_over_a_trial_corpus() {
    let corpus = TrialCorpus::build(State::Fresh);
    let hook = corpus.repo().join(".git").join("hooks").join("pre-commit");
    let hook_body =
        std::fs::read_to_string(&hook).expect("`jigc setup` installed a pre-commit hook");
    assert!(
        hook_body.contains("jigc"),
        "the overlay arm is only worth running over jigc's OWN hook; got:\n{hook_body}",
    );

    for state in GitState::ALL {
        match state.overlay_refusal() {
            Some(reason) => {
                let refused = overlay(&corpus, *state).expect_err(
                    "`unborn` must refuse the overlay rather than build something else",
                );
                assert_eq!(
                    refused, reason,
                    "the refusal must carry the stated reason, not a generic failure",
                );
                // The refusal is inert: the corpus still has a commit, which is the
                // very thing that makes `unborn` unreachable here.
                corpus.git(&["rev-parse", "--verify", "HEAD"]);
            }
            None => {
                let arm = corpus.copy_state();
                assert!(
                    arm.repo()
                        .join(".git")
                        .join("hooks")
                        .join("pre-commit")
                        .exists(),
                    "the copied corpus must still carry jigc's pre-commit hook — every \
                     fixture commit below runs it",
                );
                overlay(&arm, *state).expect("an overlay-capable state builds over a corpus");
                assert_state(&arm.repo(), &arm.home(), *state);
            }
        }
    }
}
