//! The route catalog's cases for the family the list of refusal sites calls `start`:
//! composition — the refusals of `jigc start` and `jigc workflow`. The family's table,
//! declared by `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here** is not the family: it is the one site of it that the fix pass
//! since the previous release touched, as the human ruled on 2026-10-11 (`DECISIONS.md` →
//! *Which families the route catalog takes in*) — a task re-entered under another workflow
//! than the one it was minted with. The rest are rows of the scheduled list, two of them
//! with the change of a caller that was read and why it is off the path.

use super::{Reading, Reason, Why};
use crate::support::route_case::{Act, Case, Place, Step};

// The site, as the committed list of refusal sites names it.
const WORKFLOW_MISMATCH: &str =
    "crates/cli/src/start.rs::reenter_in_repo::workflow-refs.workflow-mismatch#1";

const SETUP: Step = Step::Jigc(&["setup"]);
/// A task minted under `record-decision` — its id slugs from the intent, `pick-tabs`.
const A_DECISION_TASK: Step = Step::Jigc(&["start", "--workflow", "record-decision", "pick tabs"]);

/// What every case of the family shares: the task is re-entered under `dev-task`, a
/// workflow it was not minted with, and the printed re-entry is run as printed.
const S: Case = Case {
    name: "",
    site: WORKFLOW_MISMATCH,
    cause: "",
    state: &[],
    refusing: &["workflow", "dev-task", "--task", "pick-tabs"],
    from: Place::Repo(""),
    env: &[],
    acts: &[Act::Run(0)],
    holds: &[],
    witnesses: &[],
    untouched: &[],
    doubt: None,
    over: None,
};

const AN_ORDINARY_TASK: [Case; 1] = [Case {
    name: "workflow-mismatch-an-ordinary-task-re-entered-as-printed",
    cause: "an ordinary task is re-entered, from the main checkout, under another workflow \
            than the one it was minted with",
    state: &[SETUP, A_DECISION_TASK],
    ..S
}];
const AN_ORDINARY_TASK_FROM_A_LINKED_WORKTREE: [Case; 1] = [Case {
    name: "workflow-mismatch-an-ordinary-task-from-a-linked-worktree-re-entered-as-printed",
    cause: "an ordinary task is re-entered, from a linked worktree its user made, under \
            another workflow than the one it was minted with",
    state: &[
        SETUP,
        A_DECISION_TASK,
        Step::Git(&[
            "worktree",
            "add",
            "-q",
            "../outside/elsewhere",
            "-b",
            "side",
        ]),
    ],
    from: Place::Outside("elsewhere"),
    ..S
}];
const A_SUB_TASK_FROM_ITS_WORKTREE: [Case; 1] = [Case {
    name: "workflow-mismatch-a-sub-task-from-its-worktree-re-entered-as-printed",
    cause: "a milestone's sub-task is re-entered, from its own worktree, under another \
            workflow than the one it was added with",
    state: &[
        SETUP,
        Step::Jigc(&["milestone", "create", "Fan out"]),
        Step::Jigc(&["milestone", "add-task", "fan-out", "do a thing"]),
        Step::Jigc(&["milestone", "provision", "fan-out"]),
    ],
    refusing: &["workflow", "dev-task", "--task", "do-a-thing"],
    from: Place::Repo(".jigc/worktrees/do-a-thing"),
    ..S
}];

/// The cases of the family, a group a cause.
pub const CASES: &[&[Case]] = &[
    &AN_ORDINARY_TASK,
    &AN_ORDINARY_TASK_FROM_A_LINKED_WORKTREE,
    &A_SUB_TASK_FROM_ITS_WORKTREE,
];

/// The sites of the family that have no case, each with why: not the one.
pub const REASONS: &[Reason] = &[];

/// **What was read for the site, and how many causes it holds** — a hand count, with its
/// derivation, and the way out of its route that is not followed.
pub const READINGS: &[Reading] = &[Reading {
    site: WORKFLOW_MISMATCH,
    read: &[
        "crates/cli/src/start.rs::reenter_in_repo — the refusal, behind the id's shape, the task's area, the base pin and the recorded workflow's read",
        "crates/cli/src/start.rs::compose_task_workflow — the callee that changed since the previous release: one field more on what it composes, which checkout the compose stands in; it is called behind this refusal",
        "crates/cli/src/cli.rs — `jigc workflow <W> --task <id>`, the one door that reaches it",
    ],
    causes: 3,
    derivation: "one predicate — the workflow named is not the one the task recorded — \
                 behind a base pin that must equal the `HEAD` of the checkout the command \
                 stands in. So the states differ by whose task it is and where it is \
                 asked from: an ordinary task from the main checkout; the same from a \
                 linked worktree of its user's, whose `HEAD` is the pin while no commit \
                 was made; a milestone's sub-task from its own worktree — from the main \
                 checkout the pin's own refusal answers first (driven), since adding the \
                 sub-task commits the record",
    without_case: &[],
    not_followed: &[(
        "*or re-seed the sub-task with the intended `--workflow`*",
        Why::NotConstructible(
            "the route prints a flag and no line: re-seeding is a discard and a new \
             `jigc milestone add-task … --workflow …` of the reader's own, and for an \
             ordinary task — which the message calls a sub-task too — there is no \
             re-seeding at all",
        ),
    )],
}];
