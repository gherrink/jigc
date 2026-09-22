//! M53 completion audit, fix 2 — **a pin-less milestone area claims no task**
//! (`completions/artifacts/M53/settle-record.md` → D3; `implementation/roadmap.md` →
//! Milestone 53 Increment 3).
//!
//! **The reader Increment 3 missed.** D3's rule is *a directory under `.jigc/tasks/` or
//! `.jigc/milestones/` that holds no base pin is a work unit at NO door*. The increment
//! converted the enumerators, the four by-id task seams and `require_milestone_area`'s doors
//! — and left `engine::milestone::owning_milestone` asking `entry.file_type().is_dir()` and
//! then `read_task_list`. So *milestone-ness* rested on the file `engine::state::unwind_area`
//! removes **first** at every other door, while *membership in a milestone* rested on the
//! file it removes **last**, and the two disagreed over exactly the window between them.
//!
//! **What that cost, driven.** A live task named by a leftover's surviving `tasks.json` is
//! told at `jigc task finalize` that its only commit boundary is `jigc milestone finalize
//! <leftover>` — which answers `milestone.unknown`'s residual sibling, because the leftover
//! is a milestone at no *other* door. A task with staged prose, no reachable commit boundary,
//! and one exit: `jigc task discard --force`, which drops the work.
//!
//! **The axis is the provenance of the leftover, not the shape of it.** Two cells, and the
//! first is the one that matters:
//!
//! 1. [`a_faulted_teardowns_leftover_claims_no_task`] — the state reached **through the
//!    binary**, with no hand plant: a landed `jigc milestone finalize` whose area teardown
//!    faults on a member *after* the pin. `MILESTONE_AREA_FILES` is removed in row order and
//!    `base.json` is row 0, so any such fault leaves the pin gone and `tasks.json` standing.
//!    The fault is the cell `engine::state::unwind_area`'s own doc-comment names — a
//!    `pre-commit` hook that leaves the area unreadable during the commit — and the whole arc
//!    is jigc's: jigc mints the area, jigc writes the `tasks.json`, jigc's own teardown stops
//!    where it stops, at **exit 0** with the commit landed.
//! 2. [`a_bare_leftover_claims_no_task`] — the cheap control: one `mkdir` and a `tasks.json`,
//!    the shape a hand or an interrupted script leaves.
//!
//! And the control the fix must not move: [`a_live_milestone_still_claims_its_sub_task`] —
//! the predicate decides what is a *milestone*, and it must decide nothing about a real one's
//! membership. A fix that quietly stopped every milestone claiming its sub-tasks would turn
//! this suite green while deleting the sub-task finalize refusal outright.

use crate::support;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use support::trial_corpus::{State, TrialCorpus};

/// The `adr` body a sub-task contributes, so the milestone boundary is a real join and not a
/// `milestone.zero-contribution` refusal. Staged the way a fanned-out sub-agent leaves it:
/// the body plus its provenance bit.
fn stage_a_contribution(repo: &Path, sub_id: &str, slug: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub_id).join("docs");
    fs::create_dir_all(&docs).expect("mk the contributing sub-task's docs/");
    fs::write(
        docs.join(format!("adr:{slug}.md")),
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# Policy\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n",
    )
    .expect("stage the contributed doc");
    fs::write(
        docs.join("provenance.json"),
        format!("{{\"docs\":{{\"adr:{slug}\":\"created\"}}}}"),
    )
    .expect("stage the provenance bit");
}

/// `jigc milestone add-task <milestone> "<intent>"`, returning the **minted** sub-task id
/// read off the ack rather than re-slugged in test code (the slug rule is the binary's).
fn add_sub_task(corpus: &TrialCorpus, milestone: &str, intent: &str) -> String {
    let ack = corpus.jigc_ok(&["milestone", "add-task", milestone, intent]);
    let (_, rest) = ack
        .split_once("added task:")
        .unwrap_or_else(|| panic!("`milestone add-task` names the task it minted; got:\n{ack}"));
    rest.split_whitespace()
        .next()
        .expect("the ack's task id is one token")
        .to_string()
}

/// Every surface a live task's commit boundary reaches, asked of a corpus whose only
/// milestone area is a leftover: none of them may route the task at a milestone door.
///
/// The task is then **landed**, because "does not say it is a sub-task" and "has a commit
/// boundary that works" are different claims and only the second one is the defect.
fn assert_the_task_has_its_own_boundary(
    corpus: &TrialCorpus,
    task: &str,
    leftover: &str,
    cell: &str,
) {
    let orientation = corpus.jigc_ok(&["start"]);
    assert!(
        orientation.contains(&format!("Active task: {task}")),
        "[{cell}] the task is live, so orientation names it; got:\n{orientation}",
    );
    assert!(
        !orientation.contains(&format!("jigc milestone finalize {leftover}")),
        "[{cell}] orientation routed a live task's commit boundary at a leftover directory; \
         got:\n{orientation}",
    );

    // Read without `jigc_ok`: the commit doc is still unauthored at this point, so the
    // preview legitimately exits 3 on its own content findings. What is under test is
    // whether it names a *milestone* as this task's boundary, which is orthogonal.
    let previewed = corpus.jigc(&["task", "validate", task]);
    let preview = format!(
        "{}{}",
        String::from_utf8_lossy(&previewed.stdout),
        String::from_utf8_lossy(&previewed.stderr),
    );
    assert!(
        !preview.contains("finalize.milestone-sub-task"),
        "[{cell}] the finalize preview claimed a leftover owns this task; got:\n{preview}",
    );

    // Something for the commit to carry — staged *after* the mint, so the carryover gate
    // has nothing to say about it. Without a diff the door refuses on `finalize.empty-commit`
    // and the landing claim below would be about the wrong thing.
    let change = format!("{task}-note.txt");
    fs::write(corpus.repo().join(&change), "A line of work.\n").expect("write the task's change");
    corpus.git(&["add", &change]);

    let landed = corpus.finalize(task, "core", "Land the task", false);
    assert!(
        landed.contains("finalized"),
        "[{cell}] the task's own finalize must land — a leftover is a milestone at no door, so \
         it cannot take a live task's commit boundary away; got:\n{landed}",
    );
}

/// **Cell 1 — the leftover a real, landed boundary left behind.**
///
/// No directory is planted and no file is hand-written into the milestone area: jigc mints
/// it, jigc writes its `tasks.json`, and jigc's own post-commit teardown stops partway. The
/// stop is induced the way `engine::state::unwind_area`'s doc-comment says it is reached — a
/// `pre-commit` hook that leaves a member of the area unreadable while the commit runs — and
/// because `MILESTONE_AREA_FILES` is walked in row order with `base.json` at row 0, what
/// survives is precisely a pin-less directory holding `tasks.json`.
///
/// The milestone's sub-task id is then re-minted as an ordinary task, which is the ordinary
/// thing to do: the boundary landed, so the intent is done and the operator starts the next
/// piece of work under the same name.
#[test]
fn a_faulted_teardowns_leftover_claims_no_task() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let repo = corpus.repo();
    corpus.jigc_ok(&["milestone", "create", "Third wave"]);
    let sub_id = add_sub_task(&corpus, "third-wave", "Trim the log");
    stage_a_contribution(&repo, &sub_id, "trim-policy");

    // Installed only now, so it fires on the **boundary** commit and not on the record
    // commits above: those run before `merged/` exists, and an unreadable `merged/docs`
    // waiting there refuses the join instead of faulting its teardown.
    let area = repo.join(".jigc").join("milestones").join("third-wave");
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nmkdir -p '{docs}'\nchmod 000 '{docs}'\nexit 0\n",
            docs = area.join("merged").join("docs").display(),
        ),
    )
    .expect("install the probe hook");
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("make the hook runnable");

    let landed = corpus.jigc_ok(&["milestone", "finalize", "third-wave"]);
    assert!(
        landed.contains("finalized"),
        "the boundary must land — the defect lives *after* a successful commit; got:\n{landed}",
    );
    fs::remove_file(&hook).expect("retire the probe hook");

    // The state, asserted against the registry rather than a literal: the pin is the member
    // the unwind takes first, so a fault on any later one leaves exactly this.
    assert!(
        !area.join("base.json").exists(),
        "the teardown removes the base pin first — if it is still here the fault landed on \
         row 0 and this fixture is the *other* cell (`settle-record.md` → §7)",
    );
    assert!(
        area.join("tasks.json").is_file(),
        "and it stopped before the task list, which is what makes the leftover claim anybody",
    );
    // Put the mode back so the corpus can be torn down; the pin stays gone, which is the
    // state under test.
    fs::set_permissions(
        area.join("merged").join("docs"),
        fs::Permissions::from_mode(0o755),
    )
    .expect("restore the probe directory's mode");

    let task = corpus.start_workflow("quick-fix", "Trim the log");
    assert_eq!(
        task, sub_id,
        "the re-minted task must wear the id the leftover's task list names, or the cell is \
         not exercised",
    );
    assert_the_task_has_its_own_boundary(&corpus, &task, "third-wave", "faulted teardown");
}

/// **Cell 2 — the bare leftover**: one `mkdir` plus a `tasks.json`, the shape a hand, a
/// half-finished script or a restored backup leaves. The cheap control for cell 1's arc.
#[test]
fn a_bare_leftover_claims_no_task() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Add a thing");

    let area = corpus
        .repo()
        .join(".jigc")
        .join("milestones")
        .join("stray-mile");
    fs::create_dir_all(&area).expect("plant the leftover milestone area");
    fs::write(
        area.join("tasks.json"),
        format!("{{\"tasks\":[\"{task}\"]}}"),
    )
    .expect("plant the surviving task list");
    assert!(
        !area.join("base.json").exists(),
        "a planted leftover must carry no base pin, or this cell proves nothing",
    );

    assert_the_task_has_its_own_boundary(&corpus, &task, "stray-mile", "bare leftover");
}

/// **The control the fix must not move.** A milestone that carries its pin still owns its
/// sub-tasks: `jigc task finalize` refuses, naming the milestone, and routes at the one door
/// that is the sub-task's commit boundary.
///
/// Without this, the fix has a trivially green sibling — stop claiming everything — that
/// deletes the sub-task finalize refusal and strands a fan-out's work outside its boundary.
#[test]
fn a_live_milestone_still_claims_its_sub_task() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    corpus.jigc_ok(&["milestone", "create", "Fourth wave"]);
    let sub_id = add_sub_task(&corpus, "fourth-wave", "Shard the index");
    assert!(
        corpus
            .repo()
            .join(".jigc")
            .join("milestones")
            .join("fourth-wave")
            .join("base.json")
            .is_file(),
        "a minted milestone carries its pin — that is what makes it a milestone at every door",
    );

    let refused = corpus.jigc(&["task", "finalize", &sub_id]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
    assert!(
        !refused.status.success(),
        "a sub-task's per-task finalize is refused; got:\n{text}",
    );
    assert!(
        text.contains("finalize.milestone-sub-task")
            && text.contains("jigc milestone finalize fourth-wave"),
        "and the refusal names the milestone and routes at its boundary; got:\n{text}",
    );

    let orientation = corpus.jigc_ok(&["start"]);
    assert!(
        orientation.contains("jigc milestone finalize fourth-wave"),
        "orientation routes the sub-task at the same door; got:\n{orientation}",
    );
}
