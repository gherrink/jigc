//! **`jigc task validate` refuses the posture `jigc task finalize` refuses** (M52
//! Increment 3 / T6; `completions/artifacts/M52/settle-record.md` → D2.6 as amended by
//! Review amendments §4).
//!
//! # What was broken
//!
//! `cli::gate_coverage::Door::Previewed` promises *"same check, same severity, same exit
//! code"*, and `jigc task validate` is the door a driver runs to learn whether the
//! commit boundary will refuse. Through M52 Increment 2 it was silent about the one
//! pre-commit phase a caller can resolve **before** finalizing: under a merge, a rebase,
//! a half-finished pick — any operation git has left un-concluded — `task validate`
//! reported the task's content findings and exited 0 or 3, while `task finalize` refused
//! outright at exit 1. The preview whose whole job is *tell me what this finalize will
//! do* was green over a state the door it forecasts would not act in.
//!
//! # The claim, and why it is one byte-identity rather than three assertions
//!
//! The Settle's words are *"renders the identical `repo.operation-in-progress` /
//! `repo.head-detached` finding, and exits 1 exactly as `finalize` would — same check,
//! same severity, same exit"*. So the fence is **byte-identity across the three doors**
//! — `task validate`, `task finalize --dry-run` and the committing `task finalize` —
//! rather than a restatement here of what the finding should say. A test that rebuilt
//! the expected message would pass while the bytes an agent reads diverged, and the
//! identity is the contract: one producer (`cli::cli`'s posture guard), asked by the
//! preview with the **finalize** door's own `ActsOnBehalf` row, so an exemption added to
//! that row reaches the preview without an edit here.
//!
//! **This suite is also the posture member's cell in two registries.** It is what
//! `flow49_acceptance.rs`'s arm-5 `preview_cell` cites for the `posture` member of
//! `cli::gate_coverage::Tier::Previewed`, and it is where
//! `dry_run_findings_equal_set.rs` states the three-door equality for the one previewed
//! member whose check is invoked **separately at the door** rather than inside
//! `TaskArea::preview_gates` (`cli::gate_coverage::Invocation::SeparatelyAtDoor`) — that
//! suite reads the envelope off **stdout**, and a refusal's envelope rides **stderr**.
//!
//! # The axis
//!
//! Every member of [`GitState::ALL`] that can be overlaid on a built corpus — the
//! operations git can leave un-concluded plus the detached HEAD — not the merge the
//! done-criterion names. One state is an instance; the class is *any posture the commit
//! boundary refuses under*, and a preview that covered one member of it and not the
//! others would be the incomplete sweep this wave exists to stop shipping.
//!
//! **The thirteenth cell is covered elsewhere, not skipped.** `GitState::Unborn` cannot
//! be overlaid on a built corpus — `jigc setup` births HEAD on an unborn repository, so
//! the state is unreachable from any corpus that has a live task to address
//! (`GitState::overlay_refusal`) — and the one fixture in the suite set that does hold a
//! live task on an unborn HEAD is
//! `file_state_history_gate::unborn_head_keeps_the_conservative_weak_deletion_block`,
//! which asserts the preview's `repo.head-unborn` refusal there rather than leaving the
//! member to a loop that cannot build it.
//!
//! # And the refusal acts on nothing
//!
//! Each cell re-asserts the fixture's own git state after all three doors have run
//! (`git_state::assert_state`): the markers git wrote are still there, HEAD is unmoved.
//! A preview that concluded the user's merge in order to report on it would be worse
//! than the silence it replaces.

use crate::support::git_state::{self, GitState};
use crate::support::trial_corpus::{State, TrialCorpus};

use std::process::Output;

/// The three doors the identity holds across, in the order a driver meets them.
const DOORS: &[(&str, &[&str])] = &[
    ("jigc task validate", &["task", "validate"]),
    ("jigc task finalize --dry-run", &["task", "finalize"]),
    ("jigc task finalize", &["task", "finalize"]),
];

/// The argv for one door over `task`, with `--dry-run` where the row needs it.
fn argv(index: usize, args: &[&str], task: &str, format: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = format.iter().map(|s| (*s).to_string()).collect();
    out.extend(args.iter().map(|s| (*s).to_string()));
    out.push(task.to_string());
    if index == 1 {
        out.push("--dry-run".to_string());
    }
    out
}

/// What a door printed, as the pair a refusal is judged on.
fn printed(out: &Output) -> (i32, String, String) {
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

/// Drive the three doors over one corpus and return their `(exit, stdout, stderr)`.
fn three_doors(corpus: &TrialCorpus, task: &str, format: &[&str]) -> Vec<(i32, String, String)> {
    DOORS
        .iter()
        .enumerate()
        .map(|(index, (_, args))| {
            let owned = argv(index, args, task, format);
            let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
            printed(&corpus.jigc(&borrowed))
        })
        .collect()
}

/// **The fence.** In every git state a user can leave behind, `jigc task validate`
/// refuses byte-identically to the two `finalize` doors, at the same exit code, and
/// concludes nothing.
#[test]
fn validate_refuses_every_posture_finalize_refuses_and_concludes_nothing() {
    let built = TrialCorpus::build(State::RefsPostHoc);
    let task = built
        .live_task()
        .expect("`refs-post-hoc` leaves a live task")
        .to_string();

    let mut covered = 0usize;
    for state in GitState::ALL {
        if state.overlay_refusal().is_some() {
            // Unborn cannot be overlaid on a corpus `jigc setup` has committed into
            // (module header): its cell lives in `file_state_history_gate.rs`.
            continue;
        }
        let corpus = built.copy_state();
        git_state::overlay(&corpus, *state).expect("overlay a reachable git state");

        let outs = three_doors(&corpus, &task, &[]);
        let (validate_code, validate_out, validate_err) = &outs[0];
        for ((door, _), (code, out, err)) in DOORS.iter().zip(outs.iter()) {
            assert_eq!(
                *code,
                1,
                "`{door}` must refuse in the `{}` state at exit 1; stdout:\n{out}\nstderr:\n{err}",
                state.name(),
            );
            assert_eq!(
                err,
                validate_err,
                "`{door}` and `jigc task validate` must print the SAME refusal in the \
                 `{}` state — one producer, so the preview cannot diverge from the door \
                 it forecasts",
                state.name(),
            );
            assert_eq!(
                out,
                validate_out,
                "`{door}` must print the same (empty) stdout as the preview in the `{}` \
                 state",
                state.name(),
            );
        }
        assert_eq!(
            *validate_code, 1,
            "the preview's own exit is finalize's, not a validation verdict",
        );
        assert!(
            validate_err.contains("repo."),
            "the refusal names a posture-family code in the `{}` state; got:\n{validate_err}",
            state.name(),
        );

        // Nothing was concluded: the markers git wrote are still there and HEAD has
        // not moved, after all three doors ran.
        git_state::assert_state(&corpus.repo(), &corpus.home(), *state);
        covered += 1;
    }
    assert_eq!(
        covered,
        GitState::ALL
            .iter()
            .filter(|state| state.overlay_refusal().is_none())
            .count(),
        "every overlayable git state is a cell; a skipped one is a posture nobody previewed",
    );
}

/// **The machine surface, at the state the done-criterion names.** The refusal document
/// is identical at all three doors and names the operation's own finding code and route.
///
/// The posture guard's `--format json` arm is the **flattened** `{"error": …}` one — the
/// row `cli::invocation_log::ENVELOPE_ARMS` declares for it — so this asserts the code and
/// the route inside that string rather than a `findings[]` array the door does not emit.
/// Which arm a refusal takes is that registry's call and not this suite's; what *is* this
/// suite's is that the preview takes the same one, byte for byte, as the door it forecasts.
#[test]
fn the_refusal_envelope_is_identical_at_the_three_doors() {
    let built = TrialCorpus::build(State::RefsPostHoc);
    let task = built
        .live_task()
        .expect("`refs-post-hoc` leaves a live task")
        .to_string();
    let corpus = built.copy_state();
    git_state::overlay(&corpus, GitState::Merge).expect("overlay a merge");

    let outs = three_doors(&corpus, &task, &["--format", "json"]);
    let (_, _, envelope) = &outs[0];
    let parsed: serde_json::Value = serde_json::from_str(envelope)
        .unwrap_or_else(|err| panic!("a JSON envelope ({err}); got:\n{envelope}"));
    let error = parsed["error"]
        .as_str()
        .unwrap_or_else(|| panic!("the refusal's declared arm is `error`; got:\n{parsed:#}"));
    assert!(
        error.contains("repo.operation-in-progress"),
        "the preview's refusal names the posture code; got:\n{error}",
    );
    assert!(
        error.contains("git merge --abort"),
        "and routes at the operation's own git command; got:\n{error}",
    );
    for ((door, _), (code, _, err)) in DOORS.iter().zip(outs.iter()) {
        assert_eq!(*code, 1, "`{door}` refuses at exit 1");
        assert_eq!(
            err, envelope,
            "`{door}`'s refusal envelope must be the preview's, byte for byte",
        );
    }
}

/// **The omitting context.** Over the same corpus with **no** operation left
/// un-concluded, the preview is untouched: it renders its validation report on stdout
/// and exits on the report's own verdict, never on a posture.
#[test]
fn a_committable_posture_leaves_the_preview_exactly_as_it_was() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let task = corpus.live_task().expect("a live task").to_string();

    let out = corpus.jigc(&["task", "validate", &task]);
    let (code, stdout, stderr) = printed(&out);
    assert!(
        code == 0 || code == 3,
        "a committable posture leaves the preview's own verdict; got exit {code}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("repo.operation-in-progress") && !stderr.contains("repo.head-detached"),
        "no posture member fires over a clean checkout; got:\n{stderr}",
    );
    assert!(
        !stdout.is_empty(),
        "the preview still renders its report on stdout",
    );
}
