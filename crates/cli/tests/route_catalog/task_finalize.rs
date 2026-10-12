//! The route catalog's cases for the gate — the family the list of refusal sites calls
//! `task finalize`: the refusals `jigc task finalize` raises, with the ones its preview
//! (`jigc task validate`), the `jigc doc` write doors and `jigc milestone finalize` raise
//! under the same codes. The family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here** is not the family: it is the sites of it that the fix pass since
//! the previous release touched, as the human ruled on 2026-10-11 (`DECISIONS.md` →
//! *Which families the route catalog takes in*) — the rest are rows of the scheduled list.
//! So far fourteen — seven with cases first: nothing staged over a dirty tree; git refusing to stage jigc's own three
//! paths; a path that was staged before the work unit existed, at the three boundaries
//! that ask; a promoting doc in a task whose commit is an amend, at the write door and at
//! the finalize; an amend whose `HEAD` moved; and the two of a doc that promotes in a
//! checkout that commits code only — a `jigc doc` write or a finalize asked from a linked
//! worktree, and a mint whose only product is a doc. Then the reconciliation and the task
//! area's own files: a committed doc or the milestone's record changed out of band under
//! a staged copy, where git cannot say, and into a shape the schema refuses; the task's
//! provenance manifest unread; and an entry of a working area that cannot be read or made.
//! Each of those twelve has a row of [`READINGS`]; two more have a reason and no case
//! ([`REASONS`]) — a refusal printed at exit 0, and one an earlier read always answers.
//!
//! **Every case starts from an install and an open task whose commit doc is complete**
//! ([`ready!`]) — made by the binary under test: `jigc setup`, `jigc start --workflow
//! dev-task`, and the two writes the `commit` doc requires. A refusal of the gate is then
//! about the one thing the case's state adds, and a route that is followed can end in a
//! commit.
//!
//! **A route here often names the command to run again and not its line** — *re-run the
//! finalize with `--carry-staged`*, *`git add` your changes, then re-run `jigc task
//! finalize`*. Where it prints a flag and no command line, the runner runs the refusing
//! command again with the printed flag ([`Act::RerunWith`]); where it prints the product's
//! command without the id the command requires, the case runs the line as printed and is
//! marked `doubt` ([`WHICH_LINE`]). **And a route of the linked-worktree guard sends its
//! reader to the main checkout** — *`cd <there>`, then `jigc …`* — so the runner goes
//! there before the next line ([`Act::Cd`]): such a case stands in the worktree, and its
//! task was minted in the main checkout, which is all the guard asks of a state.

use super::{Reading, Reason, Why};
use crate::support::route_case::{Act, Case, Check, Operand, Place, Step};

// The sites, as the committed list of refusal sites names them.
const NOTHING_STAGED: &str =
    "crates/cli/src/task.rs::nothing_staged_finding::finalize.nothing-staged#1";
const STAGE_FAILED: &str = "crates/cli/src/task.rs::stage_failed_finding::finalize.stage-failed#1";
const CARRIED_STAGED: &str =
    "crates/engine/src/finalize.rs::carried_staged_finding::finalize.carried-staged#1";
const AMEND_STAGED_DOC: &str =
    "crates/cli/src/task.rs::amend_staged_doc_finding::finalize.amend-staged-doc#1";
const AMEND_BASE_MISMATCH: &str =
    "crates/engine/src/finalize.rs::amend_base_mismatch_finding::finalize.base-mismatch#1";
const LINKED_DOC: &str =
    "crates/cli/src/task.rs::linked_worktree_doc_finding::finalize.linked-worktree-doc#1";
const LINKED_MINT: &str =
    "crates/cli/src/task.rs::linked_worktree_mint_finding::finalize.linked-worktree-doc#1";
const UNANSWERED_PIN: &str =
    "crates/engine/src/file_state.rs::unanswered_pin_finding::reconciliation.conflict-block#1";
const CONFLICT_BLOCK: &str =
    "crates/engine/src/file_state.rs::conflict_block_finding::reconciliation.conflict-block#1";
const CONFORMANCE_BLOCK: &str = "crates/engine/src/file_state.rs::conformance_block_finding::reconciliation.conformance-block#1";
const DRIFT_STORE: &str =
    "crates/engine/src/file_state.rs::drift_store_finding::file-state.hash-matches#1";
const PROVENANCE_IO: &str =
    "crates/engine/src/finalize.rs::provenance_io_finding::finalize.provenance-io#1";
const SOURCE_PATH_IO: &str =
    "crates/engine/src/finalize.rs::source_path_io_finding::finalize.source-path-io#1";
const WORKING_AREA_IO: &str = "crates/engine/src/state.rs::io_finding::task.working-area-io#1";

/// The open task's working area, and the amend task's, from the repository root — gone
/// once the task has landed or is discarded.
const TASK_AREA: &str = ".jigc/tasks/write-a-note";
const AMEND_AREA: &str = ".jigc/tasks/fix-the-message";

/// The committed decisions log, from the repository root.
const LOG: &str = "docs/decisions-log.md";

/// An open task of the `dev-task` workflow — its id slugs from the intent, `write-a-note`.
const A_TASK: Step = Step::Jigc(&["start", "--workflow", "dev-task", "write a note"]);
/// The same intent under the workflow that may create the decisions log.
const A_DECIDED_TASK: Step = Step::Jigc(&["start", "--workflow", "decided-task", "write a note"]);

/// The decisions log, created in the open task: the one doc the `decided-task` workflow
/// may create, and one that promotes — to `docs/decisions-log.md`.
const THE_LOG_CREATED: Step = Step::Jigc(&[
    "doc",
    "create",
    "decisions-log",
    "--title",
    "Decisions Log",
    "--task",
    "write-a-note",
]);

/// The two writes a `commit` doc requires, through the binary under test, for the task
/// `$id` — its type, and its summary from a file outside the repository.
macro_rules! commit_doc {
    ($id:literal) => {
        [
            Step::Jigc(&[
                "doc",
                "set-field",
                concat!("commit:", $id, "#header/type"),
                "--task",
                $id,
                "--value",
                "docs",
            ]),
            Step::File(Place::Outside("summary"), "write a note\n"),
            Step::Jigc(&[
                "doc",
                "set-slot",
                concat!("commit:", $id, "#summary"),
                "--task",
                $id,
                "--from-file",
                "../outside/summary",
            ]),
        ]
    };
}
const ITS_COMMIT_DOC: [Step; 3] = commit_doc!("write-a-note");
const THE_AMENDS_COMMIT_DOC: [Step; 3] = commit_doc!("fix-the-message");

/// **An install, then what the case plants before any task exists, then an open task with
/// its commit doc complete, then what the case plants in it.**
macro_rules! ready_after {
    ([$($before:expr),* $(,)?] $(, $plant:expr)* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            $($before,)*
            A_TASK,
            ITS_COMMIT_DOC[0],
            ITS_COMMIT_DOC[1],
            ITS_COMMIT_DOC[2],
            $($plant,)*
        ]
    };
}

/// [`ready_after!`] nothing: an install and an open task with its commit doc complete.
macro_rules! ready {
    ($($plant:expr),* $(,)?) => {
        ready_after!([] $(, $plant)*)
    };
}

/// A change of the task's own, staged.
const ITS_CHANGE: [Step; 2] = [
    Step::File(Place::Repo("mine.txt"), "mine\n"),
    Step::Git(&["add", "mine.txt"]),
];

/// What every case of the gate shares: the refusing command is the finalize of the open
/// task, at the repository's root.
const FINALIZE: Case = Case {
    name: "",
    site: "",
    cause: "",
    state: &[],
    refusing: &["task", "finalize", "write-a-note"],
    from: Place::Repo(""),
    env: &[],
    acts: &[],
    holds: &[],
    witnesses: &[],
    untouched: &[],
    doubt: None,
    over: None,
};

// ---------------------------------------------------------------------------
// `nothing_staged_finding` · `finalize.nothing-staged`
// ---------------------------------------------------------------------------

/// The two readings of a product command printed without the id it requires.
const WHICH_LINE: &str = "*`git add` your changes, then re-run `jigc task finalize`*: (a) the \
    second backticked span is a command line to run as printed, like every `jigc` line a \
    route prints — it has no task id, and the command requires one; (b) it names the command \
    the reader has just run, to be run again as they ran it — then the route prints no line \
    for it, and the frame has no act: it runs the refusing command again only where a route \
    prints no command line of the product. The case follows (a). The surface contract says \
    of a `git` span with no operand that it *names a command rather than a file* (the \
    printed-path fence), and of `jigc` spans nothing of the kind; `jigc task finalize` over \
    the neighbouring refusal, `finalize.empty-commit`, is printed with the id";

/// **The tree has changes and the index none of them**, by the three kinds of change git
/// reports: the route is one, and it is followed as printed — `git add`, which adds
/// nothing without an operand, and `jigc task finalize`, which is refused without an id.
macro_rules! nothing_staged {
    ($name:literal, $cause:literal, $dirt:expr) => {
        [Case {
            name: concat!("finalize-nothing-staged-", $name, "-added-and-finalized-as-printed"),
            site: NOTHING_STAGED,
            cause: $cause,
            state: ready![$dirt],
            acts: &[Act::Run(0), Act::Run(1)],
            holds: &[Check::Absent(Place::Repo(TASK_AREA))],
            doubt: Some(WHICH_LINE),
            ..FINALIZE
        }]
    };
}
const A_TRACKED_FILE_MODIFIED: [Case; 1] = nothing_staged!(
    "a-tracked-file-modified",
    "a tracked file is modified in the working tree and nothing is staged",
    Step::File(Place::Repo("README.md"), "changed\n")
);
const A_NEW_FILE_NO_INDEX_HAS: [Case; 1] = nothing_staged!(
    "a-new-file",
    "the working tree holds a file git does not track and nothing is staged",
    Step::File(Place::Repo("new.txt"), "new\n")
);
const A_TRACKED_FILE_DELETED: [Case; 1] = nothing_staged!(
    "a-tracked-file-deleted",
    "a tracked file is deleted from the working tree and the deletion is not staged",
    Step::Git(&["-c", "alias.gone=!rm README.md", "gone"])
);

// ---------------------------------------------------------------------------
// `stage_failed_finding` · `finalize.stage-failed`
// ---------------------------------------------------------------------------

/// A lock nobody holds stands in git's directory: the route names it, and with it removed
/// the printed re-run lands the commit.
const A_STALE_INDEX_LOCK: [Case; 1] = [Case {
    name: "finalize-stage-failed-a-stale-index-lock-removed",
    site: STAGE_FAILED,
    cause: "a stale `.git/index.lock` stands, so git cannot stage the three paths jigc \
            commits with the work",
    state: ready![
        ITS_CHANGE[0],
        ITS_CHANGE[1],
        Step::File(Place::Repo(".git/index.lock"), ""),
    ],
    acts: &[Act::Remove(Operand::Span(1)), Act::Run(0)],
    holds: &[
        Check::Absent(Place::Repo(".git/index.lock")),
        Check::Absent(Place::Repo(TASK_AREA)),
    ],
    ..FINALIZE
}];

/// The root `.gitignore` holds one rule, over the version stamp, and no index has the
/// stamp: git will not add it. **The rule dropped is, here, the file removed** — it holds
/// nothing else, and the frame has no act that takes one line out of a file.
const THE_STAMP_IGNORED: [Case; 1] = [Case {
    name: "finalize-stage-failed-the-stamp-ignored-and-in-no-index-ignore-file-removed",
    site: STAGE_FAILED,
    cause: "the root `.gitignore` covers `.jigc/version`, one of the three paths jigc \
            commits with the work, and no index has it: git will not add a path it ignores",
    state: &[
        Step::Jigc(&["setup"]),
        Step::File(Place::Repo(".gitignore"), ".jigc/version\n"),
        Step::Git(&["rm", "-q", "--cached", ".jigc/version"]),
        Step::Git(&["add", ".gitignore"]),
        Step::Git(&["commit", "-q", "-m", "ignore the stamp"]),
        A_TASK,
        ITS_COMMIT_DOC[0],
        ITS_COMMIT_DOC[1],
        ITS_COMMIT_DOC[2],
        ITS_CHANGE[0],
        ITS_CHANGE[1],
    ],
    acts: &[Act::Remove(Operand::Span(2)), Act::Run(0)],
    holds: &[Check::Absent(Place::Repo(TASK_AREA))],
    ..FINALIZE
}];

/// The root `.gitignore` covers the directory a doc promotes to: git refuses the same
/// `git add`, over a path that is none of the three the route names. No lock stands and no
/// rule covers any of the three, so nothing the route names is there to act on: what is
/// left of it is the printed re-run.
const THE_DOCS_HOME_IGNORED: [Case; 1] = [Case {
    name: "finalize-stage-failed-the-docs-home-ignored-re-run-as-printed",
    site: STAGE_FAILED,
    cause: "the root `.gitignore` covers `docs/`, where the task's new doc promotes to: git \
            will not add the promoted file",
    state: &[
        Step::Jigc(&["setup"]),
        Step::File(Place::Repo(".gitignore"), "docs/\n"),
        Step::Git(&["add", ".gitignore"]),
        Step::Git(&["commit", "-q", "-m", "ignore docs"]),
        A_DECIDED_TASK,
        ITS_COMMIT_DOC[0],
        ITS_COMMIT_DOC[1],
        ITS_COMMIT_DOC[2],
        THE_LOG_CREATED,
    ],
    acts: &[Act::Run(0)],
    holds: &[Check::Absent(Place::Repo(TASK_AREA))],
    ..FINALIZE
}];

// ---------------------------------------------------------------------------
// `carried_staged_finding` · `finalize.carried-staged`
// ---------------------------------------------------------------------------

/// A file staged before the work unit exists, and a tracked file staged for deletion then —
/// its bytes still in the working tree.
const STAGED_BEFORE: [Step; 2] = [
    Step::File(Place::Repo("wip.txt"), "wip\n"),
    Step::Git(&["add", "wip.txt"]),
];
const STAGED_FOR_DELETION_BEFORE: Step = Step::Git(&["rm", "-q", "--cached", "README.md"]);

/// A fan-out with one sub-task whose worktree holds one staged file — a milestone that
/// finalizes clean, made by the binary under test.
const A_FAN_OUT_WITH_WORK: [Step; 5] = [
    Step::Jigc(&["milestone", "create", "Fan out"]),
    Step::Jigc(&["milestone", "add-task", "fan-out", "do a thing"]),
    Step::Jigc(&["milestone", "provision", "fan-out"]),
    Step::File(Place::Repo(".jigc/worktrees/do-a-thing/work.txt"), "work\n"),
    Step::Git(&["-C", ".jigc/worktrees/do-a-thing", "add", "work.txt"]),
];

/// **A path staged before the work unit existed, followed the two ways its route offers**:
/// unstaged by the printed line and the boundary run again — which the route does not
/// print as a line — and the boundary run again with the printed flag.
macro_rules! carried {
    (
        $prefix:literal, $cause:literal, $path:literal,
        state: $state:expr, refusing: $refusing:expr $(,)?
    ) => {
        [
            Case {
                name: concat!("finalize-carried-", $prefix, "-unstaged-as-printed"),
                site: CARRIED_STAGED,
                cause: $cause,
                state: $state,
                refusing: $refusing,
                acts: &[Act::Run(0), Act::Rerun],
                holds: &[Check::RegularFile(Place::Repo($path))],
                untouched: &[$path],
                ..FINALIZE
            },
            Case {
                name: concat!("finalize-carried-", $prefix, "-declared-by-the-flag"),
                site: CARRIED_STAGED,
                cause: $cause,
                state: $state,
                refusing: $refusing,
                acts: &[Act::RerunWith(1)],
                holds: &[Check::RegularFile(Place::Repo($path))],
                untouched: &[$path],
                ..FINALIZE
            },
        ]
    };
}

const A_CHANGE_STAGED_BEFORE_THE_TASK: [Case; 2] = carried!(
    "a-change-at-the-finalize",
    "a file was staged before the task was minted, and the task's finalize is asked",
    "wip.txt",
    state: ready_after!([STAGED_BEFORE[0], STAGED_BEFORE[1]], ITS_CHANGE[0], ITS_CHANGE[1]),
    refusing: &["task", "finalize", "write-a-note"],
);
const A_DELETION_STAGED_BEFORE_THE_TASK: [Case; 2] = carried!(
    "a-deletion-at-the-finalize",
    "a tracked file was staged for deletion before the task was minted, and the task's \
     finalize is asked",
    "README.md",
    state: ready_after!([STAGED_FOR_DELETION_BEFORE], ITS_CHANGE[0], ITS_CHANGE[1]),
    refusing: &["task", "finalize", "write-a-note"],
);
const A_CHANGE_AT_THE_PREVIEW: [Case; 2] = carried!(
    "a-change-at-the-preview",
    "a file was staged before the task was minted, and the task's preview is asked",
    "wip.txt",
    state: ready_after!([STAGED_BEFORE[0], STAGED_BEFORE[1]], ITS_CHANGE[0], ITS_CHANGE[1]),
    refusing: &["task", "validate", "write-a-note"],
);
const A_DELETION_AT_THE_PREVIEW: [Case; 2] = carried!(
    "a-deletion-at-the-preview",
    "a tracked file was staged for deletion before the task was minted, and the task's \
     preview is asked",
    "README.md",
    state: ready_after!([STAGED_FOR_DELETION_BEFORE], ITS_CHANGE[0], ITS_CHANGE[1]),
    refusing: &["task", "validate", "write-a-note"],
);
const A_CHANGE_AT_THE_MILESTONES_BOUNDARY: [Case; 2] = carried!(
    "a-change-at-the-milestones-finalize",
    "a file was staged in the main checkout before the milestone was created, and the \
     milestone's finalize is asked",
    "wip.txt",
    state: &[
        Step::Jigc(&["setup"]),
        STAGED_BEFORE[0],
        STAGED_BEFORE[1],
        A_FAN_OUT_WITH_WORK[0],
        A_FAN_OUT_WITH_WORK[1],
        A_FAN_OUT_WITH_WORK[2],
        A_FAN_OUT_WITH_WORK[3],
        A_FAN_OUT_WITH_WORK[4],
    ],
    refusing: &["milestone", "finalize", "fan-out"],
);
const A_DELETION_AT_THE_MILESTONES_BOUNDARY: [Case; 2] = carried!(
    "a-deletion-at-the-milestones-finalize",
    "a tracked file was staged for deletion in the main checkout before the milestone was \
     created, and the milestone's finalize is asked",
    "README.md",
    state: &[
        Step::Jigc(&["setup"]),
        STAGED_FOR_DELETION_BEFORE,
        A_FAN_OUT_WITH_WORK[0],
        A_FAN_OUT_WITH_WORK[1],
        A_FAN_OUT_WITH_WORK[2],
        A_FAN_OUT_WITH_WORK[3],
        A_FAN_OUT_WITH_WORK[4],
    ],
    refusing: &["milestone", "finalize", "fan-out"],
);

// ---------------------------------------------------------------------------
// `amend_staged_doc_finding` · `finalize.amend-staged-doc`
// ---------------------------------------------------------------------------

/// **A committed decisions log, and an amend task over the commit that landed it**: a task
/// that may create the log creates it and is finalized — by the binary under test — and
/// `jigc task amend` is then minted against that commit. The steps a case adds follow.
macro_rules! amending_over_a_committed_log {
    ($($plant:expr),* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            A_DECIDED_TASK,
            ITS_COMMIT_DOC[0],
            ITS_COMMIT_DOC[1],
            ITS_COMMIT_DOC[2],
            THE_LOG_CREATED,
            Step::Jigc(&["task", "finalize", "write-a-note"]),
            Step::Jigc(&["task", "amend", "fix the message"]),
            $($plant,)*
        ]
    };
}

/// The two readings of a route that says a line mints a task.
const WHAT_THE_FRONT_DOOR_DOES: &str = "*`jigc start \"<intent>\"` mints an ordinary task, \
    whose finalize commits the promoted doc*: (a) the printed line, run with an intent, \
    mints a task — the case runs it and holds that a task's working area is there; (b) the \
    line is the front door, where the reader picks a workflow and mints with a second line \
    the door prints — then the route prints no line that mints, and the frame has no case. \
    The case follows (a), with the placeholder's own bytes for the intent: the refusal \
    lists none and the intent is the reader's own words. The surface contract makes a \
    quoted `<placeholder>` in a `jigc` span legal and the reader's to fill; it does not say \
    what a line must do that a route says *mints*. The linked-worktree refusal's route \
    words the same line as *the front door there, which mints a task or presents the \
    workflows to pick one from*";

/// The staged form of the log, hand-placed in the amend task's area: a file of the name a
/// staged `decisions-log` has, with the committed doc's bytes.
const THE_LOG_HAND_PLACED: Step = Step::File(
    Place::Repo(".jigc/tasks/fix-the-message/docs/decisions-log:decisions-log.md"),
    "---\nschema-version: 1\n---\n\n# Decisions Log\n\n## Entries\n",
);

const A_PROMOTING_DOC_IN_AN_AMEND: [Case; 3] = [
    Case {
        name: "finalize-amend-staged-doc-an-edit-refused-and-the-front-door-run-as-printed",
        site: AMEND_STAGED_DOC,
        cause: "a `jigc doc` edit of a committed doc that promotes is asked in a task whose \
                commit model is an amend — the write door, before anything is staged",
        state: amending_over_a_committed_log![],
        refusing: &[
            "doc",
            "add-item",
            "decisions-log:decisions-log#entries",
            "--task",
            "fix-the-message",
            "--title",
            "A decision",
        ],
        acts: &[Act::Run(0)],
        holds: &[
            Check::Directory(Place::Repo(".jigc/tasks/intent")),
            Check::Directory(Place::Repo(AMEND_AREA)),
        ],
        untouched: &[LOG],
        doubt: Some(WHAT_THE_FRONT_DOOR_DOES),
        ..FINALIZE
    },
    Case {
        name: "finalize-amend-staged-doc-hand-placed-task-discarded-and-begun-again-as-printed",
        site: AMEND_STAGED_DOC,
        cause: "a doc that promotes stands staged in an amend task's area — placed there by \
                hand, since the write door refuses every edit — and the finalize is asked",
        state: amending_over_a_committed_log![
            THE_AMENDS_COMMIT_DOC[0],
            THE_AMENDS_COMMIT_DOC[1],
            THE_AMENDS_COMMIT_DOC[2],
            THE_LOG_HAND_PLACED,
        ],
        refusing: &["task", "finalize", "fix-the-message"],
        acts: &[Act::Run(0), Act::Run(1), Act::Run(2)],
        holds: &[Check::Absent(Place::Repo(AMEND_AREA))],
        untouched: &[LOG],
        ..FINALIZE
    },
    Case {
        name: "finalize-amend-staged-doc-hand-placed-previewed-discarded-and-begun-again",
        site: AMEND_STAGED_DOC,
        cause: "a doc that promotes stands staged in an amend task's area, and the task's \
                preview is asked",
        state: amending_over_a_committed_log![
            THE_AMENDS_COMMIT_DOC[0],
            THE_AMENDS_COMMIT_DOC[1],
            THE_AMENDS_COMMIT_DOC[2],
            THE_LOG_HAND_PLACED,
        ],
        refusing: &["task", "validate", "fix-the-message"],
        acts: &[Act::Run(0), Act::Run(1), Act::Run(2)],
        holds: &[Check::Absent(Place::Repo(AMEND_AREA))],
        untouched: &[LOG],
        ..FINALIZE
    },
];

// ---------------------------------------------------------------------------
// `amend_base_mismatch_finding` · `finalize.base-mismatch`
// ---------------------------------------------------------------------------

/// **An amend task with its commit doc complete, and then `HEAD` is another commit**: the
/// route's first way is two printed lines — the task discarded, a fresh amend minted
/// against the commit that is there now.
macro_rules! head_moved {
    ($name:literal, $cause:literal $(, $moved:expr)+ $(,)?) => {
        [Case {
            name: concat!("finalize-amend-base-", $name, "-discarded-and-minted-again-as-printed"),
            site: AMEND_BASE_MISMATCH,
            cause: $cause,
            state: &[
                Step::Jigc(&["setup"]),
                Step::Jigc(&["task", "amend", "fix the message"]),
                THE_AMENDS_COMMIT_DOC[0],
                THE_AMENDS_COMMIT_DOC[1],
                THE_AMENDS_COMMIT_DOC[2],
                $($moved,)+
            ],
            refusing: &["task", "finalize", "fix-the-message"],
            acts: &[Act::Run(0), Act::Run(1)],
            holds: &[Check::Absent(Place::Repo(AMEND_AREA))],
            ..FINALIZE
        }]
    };
}
const A_COMMIT_ON_TOP: [Case; 1] = head_moved!(
    "a-commit-on-top",
    "a commit was made on top of the commit the amend was minted against",
    Step::File(Place::Repo("another.txt"), "another\n"),
    Step::Git(&["add", "another.txt"]),
    Step::Git(&["commit", "-q", "-m", "another"]),
);
const THE_PINNED_COMMIT_REWRITTEN: [Case; 1] = head_moved!(
    "the-pinned-commit-rewritten",
    "the commit the amend was minted against was rewritten by hand — `git commit --amend` — \
     so `HEAD` is a commit of another id",
    Step::Git(&["commit", "--amend", "-q", "-m", "reworded by hand"]),
);

// ---------------------------------------------------------------------------
// `linked_worktree_doc_finding` · `finalize.linked-worktree-doc` — a doc that promotes,
// in a checkout that commits code only
// ---------------------------------------------------------------------------

/// A linked worktree of the user's own, beside the repository, on a branch of its own —
/// and the same with its `HEAD` detached.
const A_LINKED_WORKTREE: Step = Step::Git(&["worktree", "add", "-q", "../outside/wt", "-b", "side"]);
const A_DETACHED_WORKTREE: Step = Step::Git(&["worktree", "add", "-q", "--detach", "../outside/wt"]);
/// Where a case stands that is asked from that worktree.
const THERE: Place = Place::Outside("wt");
/// Code staged in that worktree.
const CODE_STAGED_THERE: [Step; 2] = [
    Step::File(Place::Outside("wt/code.txt"), "code\n"),
    Step::Git(&["-C", "../outside/wt", "add", "code.txt"]),
];
/// jigc's own fan-out worktree, from the repository root — where a sub-task's case stands.
const THE_SUB_TASKS_WORKTREE: Place = Place::Repo(".jigc/worktrees/do-a-thing");

/// The create the write door refuses — the one doc the open task may create.
const THE_CREATE: &[&str] = &[
    "doc",
    "create",
    "decisions-log",
    "--title",
    "Decisions Log",
    "--task",
    "write-a-note",
];

/// **An install, an open task minted in the main checkout that may create the decisions
/// log, its commit doc complete — and then what the case plants**: a worktree, and what
/// stands in it or in the main checkout.
macro_rules! decided {
    ($($plant:expr),* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            A_DECIDED_TASK,
            ITS_COMMIT_DOC[0],
            ITS_COMMIT_DOC[1],
            ITS_COMMIT_DOC[2],
            $($plant,)*
        ]
    };
}

/// The two readings of a printed command a route sends its reader to for its words.
const WHAT_A_POINTER_OWES: &str = "*`jigc task validate <task>` names where it lands* · \
    *`jigc task finalize <task>` names that route*: (a) the backticked line is a step of \
    the route, held to exit 0 like every printed command — it is refused, under this \
    refusal's own code or another; (b) the line is there for what it prints — the next \
    route — and its exit owes nothing. The case follows (a). The surface contract holds a \
    backticked `jigc` span to shell-safe tokens and says nothing of what a line owes that a \
    route cites for its output; the pilot's fourth question is the same one, over a `git` \
    line";

/// The write door, asked from a linked worktree whose task commits code there — nothing
/// staged in the worktree.
const A_WRITE_FROM_A_LINKED_WORKTREE: [Case; 1] = [Case {
    name: "linked-doc-write-refused-the-front-door-there-as-printed",
    site: LINKED_DOC,
    cause: "a `jigc doc` write of a doc that promotes is asked from a linked worktree, in a \
            task that stages no managed doc; the worktree's index is empty",
    state: decided![A_LINKED_WORKTREE],
    refusing: THE_CREATE,
    from: THERE,
    acts: &[Act::Cd(0), Act::Run(1)],
    ..FINALIZE
}];

/// The same door with code staged in the worktree: the route grows the one exit a change
/// to both has — the task moved to the main checkout, line by line.
const A_WRITE_BESIDE_STAGED_CODE: &str = "a `jigc doc` write of a doc that promotes is asked \
    from a linked worktree whose index holds staged code";
const A_WRITE_FROM_A_LINKED_WORKTREE_WITH_CODE_STAGED: [Case; 3] = [
    Case {
        name: "linked-doc-write-refused-beside-staged-code-the-front-door-there-as-printed",
        site: LINKED_DOC,
        cause: A_WRITE_BESIDE_STAGED_CODE,
        state: decided![A_LINKED_WORKTREE, CODE_STAGED_THERE[0], CODE_STAGED_THERE[1]],
        refusing: THE_CREATE,
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        ..FINALIZE
    },
    Case {
        name: "linked-doc-write-refused-beside-staged-code-task-finalized-there-for-its-code",
        site: LINKED_DOC,
        cause: A_WRITE_BESIDE_STAGED_CODE,
        state: decided![A_LINKED_WORKTREE, CODE_STAGED_THERE[0], CODE_STAGED_THERE[1]],
        refusing: THE_CREATE,
        from: THERE,
        acts: &[Act::Run(3)],
        holds: &[
            Check::Absent(Place::Repo(TASK_AREA)),
            Check::RegularFile(Place::Outside("wt/code.txt")),
        ],
        ..FINALIZE
    },
    Case {
        name: "linked-doc-write-refused-beside-staged-code-change-moved-to-the-main-checkout",
        site: LINKED_DOC,
        cause: A_WRITE_BESIDE_STAGED_CODE,
        state: decided![A_LINKED_WORKTREE, CODE_STAGED_THERE[0], CODE_STAGED_THERE[1]],
        refusing: THE_CREATE,
        from: THERE,
        acts: &[
            Act::Run(6),
            Act::Run(7),
            Act::Cd(8),
            Act::Run(9),
            Act::Run(10),
            Act::Run(11),
        ],
        holds: &[
            Check::Absent(Place::Repo(TASK_AREA)),
            Check::RegularFile(Place::Repo("code.txt")),
            Check::Directory(Place::Repo(".jigc/tasks/intent")),
        ],
        ..FINALIZE
    },
];

/// The same door where the task already stages a doc that promotes: it does not finalize
/// from the worktree, and the route says which command names where it lands.
const A_SECOND_WRITE: &str = "a `jigc doc` write is asked from a linked worktree in a task \
    that already stages a doc that promotes — staged from the main checkout";
const AN_EDIT_OF_THE_STAGED_LOG: &[&str] = &[
    "doc",
    "add-item",
    "decisions-log:decisions-log#entries",
    "--title",
    "Use tabs",
    "--task",
    "write-a-note",
];
const A_WRITE_WHERE_THE_TASK_STAGES_A_DOC: [Case; 2] = [
    Case {
        name: "linked-doc-second-write-refused-the-front-door-there-as-printed",
        site: LINKED_DOC,
        cause: A_SECOND_WRITE,
        state: decided![THE_LOG_CREATED, A_LINKED_WORKTREE],
        refusing: AN_EDIT_OF_THE_STAGED_LOG,
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        ..FINALIZE
    },
    Case {
        name: "linked-doc-second-write-refused-the-preview-asked-where-it-lands",
        site: LINKED_DOC,
        cause: A_SECOND_WRITE,
        state: decided![THE_LOG_CREATED, A_LINKED_WORKTREE],
        refusing: AN_EDIT_OF_THE_STAGED_LOG,
        from: THERE,
        acts: &[Act::Run(3)],
        doubt: Some(WHAT_A_POINTER_OWES),
        ..FINALIZE
    },
];

/// The same door from a worktree whose `HEAD` is detached.
const A_WRITE_FROM_A_DETACHED_HEAD: &str = "a `jigc doc` write of a doc that promotes is \
    asked from a linked worktree whose `HEAD` is detached";
const A_WRITE_FROM_A_DETACHED_WORKTREE: [Case; 2] = [
    Case {
        name: "linked-doc-write-refused-on-a-detached-head-the-front-door-there-as-printed",
        site: LINKED_DOC,
        cause: A_WRITE_FROM_A_DETACHED_HEAD,
        state: decided![A_DETACHED_WORKTREE],
        refusing: THE_CREATE,
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        ..FINALIZE
    },
    Case {
        name: "linked-doc-write-refused-on-a-detached-head-the-finalize-asked-for-its-route",
        site: LINKED_DOC,
        cause: A_WRITE_FROM_A_DETACHED_HEAD,
        state: decided![A_DETACHED_WORKTREE],
        refusing: THE_CREATE,
        from: THERE,
        acts: &[Act::Run(3)],
        doubt: Some(WHAT_A_POINTER_OWES),
        ..FINALIZE
    },
];

/// The two other standings of the task with code staged beside them: the route is the
/// standing's, grown by the same clause.
const A_WRITE_BESIDE_STAGED_CODE_IN_THE_OTHER_STANDINGS: [Case; 2] = [
    Case {
        name: "linked-doc-second-write-refused-beside-staged-code-the-front-door-there-as-printed",
        site: LINKED_DOC,
        cause: "a `jigc doc` write is asked from a linked worktree whose index holds staged \
                code, in a task that already stages a doc that promotes",
        state: decided![
            THE_LOG_CREATED,
            A_LINKED_WORKTREE,
            CODE_STAGED_THERE[0],
            CODE_STAGED_THERE[1],
        ],
        refusing: AN_EDIT_OF_THE_STAGED_LOG,
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        ..FINALIZE
    },
    Case {
        name: "linked-doc-write-refused-on-a-detached-head-beside-staged-code-the-front-door-there",
        site: LINKED_DOC,
        cause: "a `jigc doc` write of a doc that promotes is asked from a linked worktree \
                whose `HEAD` is detached and whose index holds staged code",
        state: decided![A_DETACHED_WORKTREE, CODE_STAGED_THERE[0], CODE_STAGED_THERE[1]],
        refusing: THE_CREATE,
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        ..FINALIZE
    },
];

/// The same door from jigc's own fan-out worktree, in a task that is not the sub-task's:
/// one route, the milestone first.
const A_WRITE_FROM_A_FAN_OUT_WORKTREE: [Case; 1] = [Case {
    name: "linked-doc-write-refused-in-a-fan-out-worktree-milestone-landed-first-as-printed",
    site: LINKED_DOC,
    cause: "a `jigc doc` write of a doc that promotes is asked from the worktree jigc \
            provisioned for a milestone's sub-task, in a task that is not that sub-task",
    state: decided![
        A_FAN_OUT_WITH_WORK[0],
        A_FAN_OUT_WITH_WORK[1],
        A_FAN_OUT_WITH_WORK[2],
        A_FAN_OUT_WITH_WORK[3],
        A_FAN_OUT_WITH_WORK[4],
    ],
    refusing: THE_CREATE,
    from: THE_SUB_TASKS_WORKTREE,
    acts: &[Act::Run(2), Act::Cd(5), Act::Run(6)],
    holds: &[Check::RegularFile(Place::Repo("work.txt"))],
    ..FINALIZE
}];

/// **The backstop: the task stages a doc that promotes — staged from the main checkout —
/// and its finalize, or its preview, is asked from the linked worktree.** The route turns
/// on what the finalize would do in the main checkout.
const STAGED_AND_FINALIZED_FROM_THERE: &str = "a task stages a doc that promotes, staged \
    from the main checkout, and its finalize is asked from a linked worktree; the main \
    checkout would take it";
const A_STAGED_DOC_FINALIZED_FROM_A_LINKED_WORKTREE: [Case; 2] = [
    Case {
        name: "linked-doc-staged-finalize-refused-finalized-from-the-main-checkout-as-printed",
        site: LINKED_DOC,
        cause: STAGED_AND_FINALIZED_FROM_THERE,
        state: decided![THE_LOG_CREATED, A_LINKED_WORKTREE],
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        holds: &[
            Check::RegularFile(Place::Repo(LOG)),
            Check::Absent(Place::Repo(TASK_AREA)),
        ],
        ..FINALIZE
    },
    Case {
        name: "linked-doc-staged-preview-refused-finalized-from-the-main-checkout-as-printed",
        site: LINKED_DOC,
        cause: "a task stages a doc that promotes, and its preview is asked from a linked \
                worktree",
        state: decided![THE_LOG_CREATED, A_LINKED_WORKTREE],
        refusing: &["task", "validate", "write-a-note"],
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        holds: &[
            Check::RegularFile(Place::Repo(LOG)),
            Check::Absent(Place::Repo(TASK_AREA)),
        ],
        ..FINALIZE
    },
];
const THE_MAIN_CHECKOUT_MOVED_OVER_THE_DOC: [Case; 1] = [Case {
    name: "linked-doc-staged-finalize-refused-base-moved-read-back-begun-again-and-dropped",
    site: LINKED_DOC,
    cause: "a task stages a doc that promotes and its finalize is asked from a linked \
            worktree, where the main checkout has since committed a file at the doc's home: \
            that door would refuse the task's base",
    state: decided![
        THE_LOG_CREATED,
        A_LINKED_WORKTREE,
        Step::File(Place::Repo(LOG), "by hand\n"),
        Step::Git(&["add", LOG]),
        Step::Git(&["commit", "-q", "-m", "a log by hand"]),
    ],
    from: THERE,
    acts: &[Act::Run(0), Act::Cd(1), Act::Run(2), Act::Run(3)],
    holds: &[Check::Absent(Place::Repo(TASK_AREA))],
    untouched: &[LOG],
    ..FINALIZE
}];
const THE_MAIN_INDEX_HOLDS_WHAT_NO_SNAPSHOT_COVERS: [Case; 1] = [Case {
    name: "linked-doc-staged-finalize-refused-main-index-uncovered-read-back-begun-again-and-dropped",
    site: LINKED_DOC,
    cause: "a task stages a doc that promotes and its finalize is asked from a linked \
            worktree, where the main checkout's index holds a staged file the task's \
            pre-task snapshot does not cover",
    state: decided![
        THE_LOG_CREATED,
        A_LINKED_WORKTREE,
        STAGED_BEFORE[0],
        STAGED_BEFORE[1],
    ],
    from: THERE,
    acts: &[
        Act::Run(0),
        Act::Run(1),
        Act::Cd(2),
        Act::Run(3),
        Act::Run(4),
    ],
    holds: &[Check::Absent(Place::Repo(TASK_AREA))],
    untouched: &["wip.txt"],
    ..FINALIZE
}];
// ---------------------------------------------------------------------------
// `linked_worktree_mint_finding` · `finalize.linked-worktree-doc` — a mint whose only
// product is a doc, from a checkout that commits code only
// ---------------------------------------------------------------------------

/// A foreign file the repository holds committed — a source `jigc migrate` can be asked
/// about from any checkout made afterwards.
const A_FOREIGN_FILE_COMMITTED: [Step; 3] = [
    Step::File(Place::Repo("NOTES.md"), "# Notes\n\nsome\n"),
    Step::Git(&["add", "NOTES.md"]),
    Step::Git(&["commit", "-q", "-m", "notes"]),
];
const A_MIGRATION: &[&str] = &["migrate", "NOTES.md", "--as", "research"];
/// A workflow whose finalize commits its docs and nothing else.
const A_DOC_ONLY_MINT: &[&str] = &["start", "--workflow", "report-inconsistency", "note a thing"];

const A_MINT_FROM_A_LINKED_WORKTREE: [Case; 2] = [
    Case {
        name: "linked-mint-migrate-refused-run-from-the-main-checkout-as-printed",
        site: LINKED_MINT,
        cause: "`jigc migrate` is asked from a linked worktree, over a source the main \
                checkout holds too",
        state: &[
            Step::Jigc(&["setup"]),
            A_FOREIGN_FILE_COMMITTED[0],
            A_FOREIGN_FILE_COMMITTED[1],
            A_FOREIGN_FILE_COMMITTED[2],
            A_LINKED_WORKTREE,
        ],
        refusing: A_MIGRATION,
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        holds: &[Check::Directory(Place::Repo(".jigc/tasks"))],
        untouched: &["NOTES.md"],
        ..FINALIZE
    },
    Case {
        name: "linked-mint-doc-only-workflow-refused-run-from-the-main-checkout-as-printed",
        site: LINKED_MINT,
        cause: "a workflow whose finalize commits docs only is minted from a linked worktree",
        state: &[Step::Jigc(&["setup"]), A_LINKED_WORKTREE],
        refusing: A_DOC_ONLY_MINT,
        from: THERE,
        acts: &[Act::Cd(0), Act::Run(1)],
        holds: &[Check::Directory(Place::Repo(".jigc/tasks/intent"))],
        ..FINALIZE
    },
];
const A_MINT_FROM_A_FAN_OUT_WORKTREE: [Case; 2] = [
    Case {
        name: "linked-mint-migrate-refused-in-a-fan-out-worktree-milestone-landed-first",
        site: LINKED_MINT,
        cause: "`jigc migrate` is asked from the worktree jigc provisioned for a milestone's \
                sub-task",
        state: &[
            Step::Jigc(&["setup"]),
            A_FOREIGN_FILE_COMMITTED[0],
            A_FOREIGN_FILE_COMMITTED[1],
            A_FOREIGN_FILE_COMMITTED[2],
            A_FAN_OUT_WITH_WORK[0],
            A_FAN_OUT_WITH_WORK[1],
            A_FAN_OUT_WITH_WORK[2],
            A_FAN_OUT_WITH_WORK[3],
            A_FAN_OUT_WITH_WORK[4],
        ],
        refusing: A_MIGRATION,
        from: THE_SUB_TASKS_WORKTREE,
        acts: &[Act::Run(2), Act::Cd(5), Act::Run(6)],
        holds: &[Check::RegularFile(Place::Repo("work.txt"))],
        untouched: &["NOTES.md"],
        ..FINALIZE
    },
    Case {
        name: "linked-mint-doc-only-workflow-refused-in-a-fan-out-worktree-milestone-landed-first",
        site: LINKED_MINT,
        cause: "a workflow whose finalize commits docs only is minted from the worktree jigc \
                provisioned for a milestone's sub-task",
        state: &[
            Step::Jigc(&["setup"]),
            A_FAN_OUT_WITH_WORK[0],
            A_FAN_OUT_WITH_WORK[1],
            A_FAN_OUT_WITH_WORK[2],
            A_FAN_OUT_WITH_WORK[3],
            A_FAN_OUT_WITH_WORK[4],
        ],
        refusing: A_DOC_ONLY_MINT,
        from: THE_SUB_TASKS_WORKTREE,
        acts: &[Act::Run(2), Act::Cd(5), Act::Run(6)],
        holds: &[
            Check::RegularFile(Place::Repo("work.txt")),
            Check::Directory(Place::Repo(".jigc/tasks/intent")),
        ],
        ..FINALIZE
    },
];

// ---------------------------------------------------------------------------
// The reconciliation — a committed doc, or the milestone's record, changed out of band
// ---------------------------------------------------------------------------

/// **A committed decisions log — created in a task and landed by the binary under test —
/// and then what the case plants**: a second task, an edit by hand, a milestone.
macro_rules! a_committed_log {
    ($($plant:expr),* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            A_DECIDED_TASK,
            ITS_COMMIT_DOC[0],
            ITS_COMMIT_DOC[1],
            ITS_COMMIT_DOC[2],
            THE_LOG_CREATED,
            Step::Jigc(&["task", "finalize", "write-a-note"]),
            $($plant,)*
        ]
    };
}

/// A second task, minted once the log is committed, with its commit doc complete; and its
/// own edit of the log — an entry, with the prose the entry requires — which copies the
/// committed doc in.
const A_SECOND_TASK: Step = Step::Jigc(&["start", "--workflow", "decided-task", "add more"]);
const THE_SECOND_TASKS_COMMIT_DOC: [Step; 3] = commit_doc!("add-more");
const SECOND_AREA: &str = ".jigc/tasks/add-more";
macro_rules! an_entry_by {
    ($task:literal) => {
        [
            Step::Jigc(&[
                "doc",
                "add-item",
                "decisions-log:decisions-log#entries",
                "--title",
                "Use spaces",
                "--task",
                $task,
            ]),
            Step::File(Place::Outside("why"), "Spaces align.\n"),
            Step::Jigc(&[
                "doc",
                "set-slot",
                "decisions-log:decisions-log#entries/use-spaces/why",
                "--task",
                $task,
                "--from-file",
                "../outside/why",
            ]),
        ]
    };
}
const ITS_EDIT_OF_THE_LOG: [Step; 3] = an_entry_by!("add-more");
const THE_SUB_TASKS_EDIT_OF_THE_LOG: [Step; 3] = an_entry_by!("do-a-thing");

/// An edit by hand, in no commit: a line appended to the committed log, and one to the
/// milestone's record — a shell line a `git` alias runs at the repository's root, since a
/// state's file step writes a whole file and these bytes are the product's.
const A_HAND_LINE_IN_THE_LOG: Step = Step::Git(&[
    "-c",
    "alias.byhand=!printf '\\nA hand line.\\n' >> docs/decisions-log.md",
    "byhand",
]);
const THE_RECORD: &str = "docs/milestone-records/fan-out.md";
const A_HAND_NOTE_IN_THE_RECORD: Step = Step::Git(&[
    "-c",
    "alias.byhand=!printf '\\nA hand note.\\n' >> docs/milestone-records/fan-out.md",
    "byhand",
]);
/// The committed log with its one section's heading gone — an edit by hand that leaves the
/// doc no longer what its schema asks for.
const THE_LOG_WITHOUT_ITS_SECTION: Step = Step::File(
    Place::Repo(LOG),
    "---\nschema-version: 1\n---\n\n# Decisions Log\n",
);

/// A migration of the committed log in place, and of a foreign file elsewhere: the ids
/// `jigc migrate` mints for the two sources.
const THE_LOGS_MIGRATION: &str = "migrate-decisions-log-docs-decisions-log-b31ca3264f78";
const THE_NOTES_MIGRATION: &str = "migrate-decisions-log-notes-old-log-2173a5301d0e";

// `conflict_block_finding` · `reconciliation.conflict-block`

const A_HAND_EDIT_UNDER_A_STAGED_COPY: &str = "a committed doc is edited by hand while an \
    open task holds it copied in with writes of its own, and the task's finalize is asked";
const A_CONFLICT_AT_THE_TASKS_DOOR: [Case; 2] = [
    Case {
        name: "conflict-hand-line-under-a-tasks-staged-copy-task-discarded-as-printed",
        site: CONFLICT_BLOCK,
        cause: A_HAND_EDIT_UNDER_A_STAGED_COPY,
        state: a_committed_log![
            A_SECOND_TASK,
            THE_SECOND_TASKS_COMMIT_DOC[0],
            THE_SECOND_TASKS_COMMIT_DOC[1],
            THE_SECOND_TASKS_COMMIT_DOC[2],
            ITS_EDIT_OF_THE_LOG[0],
            ITS_EDIT_OF_THE_LOG[1],
            ITS_EDIT_OF_THE_LOG[2],
            A_HAND_LINE_IN_THE_LOG,
        ],
        refusing: &["task", "finalize", "add-more"],
        acts: &[Act::Run(0)],
        holds: &[
            Check::Absent(Place::Repo(SECOND_AREA)),
            Check::Contains(Place::Repo(LOG), "A hand line."),
        ],
        ..FINALIZE
    },
    Case {
        name: "conflict-hand-line-under-a-tasks-staged-copy-edit-reverted",
        site: CONFLICT_BLOCK,
        cause: A_HAND_EDIT_UNDER_A_STAGED_COPY,
        state: a_committed_log![
            A_SECOND_TASK,
            THE_SECOND_TASKS_COMMIT_DOC[0],
            THE_SECOND_TASKS_COMMIT_DOC[1],
            THE_SECOND_TASKS_COMMIT_DOC[2],
            ITS_EDIT_OF_THE_LOG[0],
            ITS_EDIT_OF_THE_LOG[1],
            ITS_EDIT_OF_THE_LOG[2],
            A_HAND_LINE_IN_THE_LOG,
        ],
        refusing: &["task", "finalize", "add-more"],
        acts: &[Act::Revert(Operand::Unprinted(LOG))],
        holds: &[
            Check::Lacks(Place::Repo(LOG), "A hand line."),
            Check::Directory(Place::Repo(SECOND_AREA)),
        ],
        ..FINALIZE
    },
];

/// A migration's rewrite, authored through the binary under test: one entry, with its prose.
const A_REWRITE: &str = "title: Decisions Log\nsections:\n  - id: entries\n    items:\n      \
    - title: \"Use tabs\"\n        set:\n          why: |-\n            <<Tabs are narrower.>>\n";
const THE_MIGRATIONS_COMMIT_DOC: [Step; 3] =
    commit_doc!("migrate-decisions-log-docs-decisions-log-b31ca3264f78");
const A_MIGRATION_OVER_A_STALE_BASELINE: [Case; 1] = [Case {
    name: "conflict-migration-in-place-over-a-stale-baseline-migration-retired-as-printed",
    site: CONFLICT_BLOCK,
    cause: "a committed doc is broken by hand and then migrated in place: the migration's \
            staged rewrite and the edit both changed the path, whose recorded baseline is \
            the committed doc's",
    state: a_committed_log![
        THE_LOG_WITHOUT_ITS_SECTION,
        Step::Jigc(&["migrate", LOG, "--as", "decisions-log"]),
        Step::File(Place::Outside("rewrite.yaml"), A_REWRITE),
        Step::Jigc(&[
            "doc",
            "author",
            "decisions-log",
            "--from-file",
            "../outside/rewrite.yaml",
            "--task",
            THE_LOGS_MIGRATION,
        ]),
        THE_MIGRATIONS_COMMIT_DOC[0],
        THE_MIGRATIONS_COMMIT_DOC[1],
        THE_MIGRATIONS_COMMIT_DOC[2],
    ],
    refusing: &["task", "finalize", THE_LOGS_MIGRATION],
    acts: &[Act::Run(2)],
    holds: &[Check::Absent(Place::Repo(
        ".jigc/tasks/migrate-decisions-log-docs-decisions-log-b31ca3264f78",
    ))],
    untouched: &[LOG],
    ..FINALIZE
}];
const A_MIGRATION_SOURCE_EDITED_AFTER_THE_MINT: [Case; 1] = [Case {
    name: "conflict-migration-source-edited-after-the-mint-retired-and-migrated-again-as-printed",
    site: CONFLICT_BLOCK,
    cause: "the foreign file a migration task was minted over is edited by hand after the \
            mint",
    state: &[
        Step::Jigc(&["setup"]),
        Step::File(Place::Repo("notes/old-log.md"), "# Old decisions\n\n- We use tabs.\n"),
        Step::Git(&["add", "notes/old-log.md"]),
        Step::Git(&["commit", "-q", "-m", "an old log"]),
        Step::Jigc(&["migrate", "notes/old-log.md", "--as", "decisions-log"]),
        Step::Git(&[
            "-c",
            "alias.byhand=!printf -- '- A line added after the mint.\\n' >> notes/old-log.md",
            "byhand",
        ]),
    ],
    refusing: &["task", "finalize", THE_NOTES_MIGRATION],
    acts: &[Act::Run(0), Act::Run(1)],
    holds: &[Check::Directory(Place::Repo(
        ".jigc/tasks/migrate-decisions-log-notes-old-log-2173a5301d0e",
    ))],
    untouched: &["notes/old-log.md"],
    ..FINALIZE
}];
const THE_RECORD_EDITED_BY_HAND: [Case; 2] = [
    Case {
        name: "conflict-milestone-record-edited-by-hand-checked-out-as-printed",
        site: CONFLICT_BLOCK,
        cause: "the milestone's record is edited by hand, in no commit, and a command that \
                writes the record is asked; jigc holds the hash of what it last wrote",
        state: &[
            Step::Jigc(&["setup"]),
            Step::Jigc(&["milestone", "create", "Fan out"]),
            A_HAND_NOTE_IN_THE_RECORD,
        ],
        refusing: &["milestone", "add-task", "fan-out", "do a thing"],
        acts: &[Act::Run(1)],
        holds: &[Check::Lacks(Place::Repo(THE_RECORD), "A hand note.")],
        ..FINALIZE
    },
    Case {
        name: "conflict-milestone-record-edited-by-hand-no-hash-held-checked-out-from-head-as-printed",
        site: CONFLICT_BLOCK,
        cause: "the milestone's record is edited by hand where jigc holds no hash for it — \
                dropped by `jigc unmanage` — so `HEAD` is what it is compared with",
        state: &[
            Step::Jigc(&["setup"]),
            Step::Jigc(&["milestone", "create", "Fan out"]),
            Step::Jigc(&["milestone", "add-task", "fan-out", "do a thing"]),
            Step::Jigc(&["unmanage", THE_RECORD]),
            A_HAND_NOTE_IN_THE_RECORD,
        ],
        refusing: &["milestone", "add-task", "fan-out", "second thing"],
        acts: &[Act::Run(2), Act::Rerun],
        holds: &[Check::Lacks(Place::Repo(THE_RECORD), "A hand note.")],
        ..FINALIZE
    },
];
const A_CONFLICT_AT_THE_JOIN: [Case; 1] = [Case {
    name: "conflict-hand-line-under-a-sub-tasks-staged-copy-edit-reverted",
    site: CONFLICT_BLOCK,
    cause: "a committed doc is edited by hand while a milestone's sub-task holds it copied \
            in with writes of its own, and the milestone's finalize is asked",
    state: a_committed_log![
        A_FAN_OUT_WITH_WORK[0],
        A_FAN_OUT_WITH_WORK[1],
        A_FAN_OUT_WITH_WORK[2],
        A_FAN_OUT_WITH_WORK[3],
        A_FAN_OUT_WITH_WORK[4],
        THE_SUB_TASKS_EDIT_OF_THE_LOG[0],
        THE_SUB_TASKS_EDIT_OF_THE_LOG[1],
        THE_SUB_TASKS_EDIT_OF_THE_LOG[2],
        A_HAND_LINE_IN_THE_LOG,
    ],
    refusing: &["milestone", "finalize", "fan-out"],
    acts: &[Act::Revert(Operand::Unprinted(LOG))],
    holds: &[Check::Lacks(Place::Repo(LOG), "A hand line.")],
    ..FINALIZE
}];

// `unanswered_pin_finding` · `reconciliation.conflict-block` — git could not say

/// git cannot compare the record with `HEAD`: a clean filter the repository requires for
/// the record's path fails. The route names no act and no object — *resolve the git
/// failure quoted above* — so the object is the case's: the attributes file that asks for
/// the filter, removed.
const GIT_CANNOT_COMPARE_THE_RECORD: [Case; 1] = [Case {
    name: "pin-unanswered-a-required-clean-filter-fails-attributes-removed",
    site: UNANSWERED_PIN,
    cause: "jigc holds no hash for the milestone's record and asks git whether the file \
            differs from `HEAD`; `git diff` fails, over a required clean filter that exits 1",
    state: &[
        Step::Jigc(&["setup"]),
        Step::Jigc(&["milestone", "create", "Fan out"]),
        Step::Jigc(&["unmanage", THE_RECORD]),
        Step::Git(&["config", "filter.bad.clean", "false"]),
        Step::Git(&["config", "filter.bad.required", "true"]),
        Step::File(
            Place::Repo(".gitattributes"),
            "docs/milestone-records/*.md filter=bad\n",
        ),
        Step::Git(&[
            "-c",
            "alias.later=!sleep 1; touch docs/milestone-records/fan-out.md",
            "later",
        ]),
    ],
    refusing: &["milestone", "add-task", "fan-out", "do a thing"],
    acts: &[Act::Remove(Operand::Unprinted(".gitattributes")), Act::Rerun],
    holds: &[Check::Absent(Place::Repo(".gitattributes"))],
    untouched: &[],
    ..FINALIZE
}];

// `conformance_block_finding` · `reconciliation.conformance-block`

const AN_EDIT_THAT_BREAKS_THE_DOC: &str = "a committed doc is edited by hand into a shape \
    its schema refuses — its one section's heading removed — and a task that did not touch \
    it is asked";
const A_NONCONFORMANT_EDIT_AT_THE_TASKS_DOOR: [Case; 2] = [
    Case {
        name: "conformance-hand-broken-doc-at-the-finalize-edit-reverted",
        site: CONFORMANCE_BLOCK,
        cause: AN_EDIT_THAT_BREAKS_THE_DOC,
        state: a_committed_log![
            A_SECOND_TASK,
            THE_SECOND_TASKS_COMMIT_DOC[0],
            THE_SECOND_TASKS_COMMIT_DOC[1],
            THE_SECOND_TASKS_COMMIT_DOC[2],
            ITS_CHANGE[0],
            ITS_CHANGE[1],
            THE_LOG_WITHOUT_ITS_SECTION,
        ],
        refusing: &["task", "finalize", "add-more"],
        acts: &[Act::Revert(Operand::Unprinted(LOG)), Act::Rerun],
        holds: &[
            Check::Absent(Place::Repo(SECOND_AREA)),
            Check::Contains(Place::Repo(LOG), "## Entries"),
        ],
        ..FINALIZE
    },
    Case {
        name: "conformance-hand-broken-doc-at-the-preview-edit-reverted",
        site: CONFORMANCE_BLOCK,
        cause: "a committed doc is edited by hand into a shape its schema refuses, and the \
                preview of a task that did not touch it is asked",
        state: a_committed_log![
            A_SECOND_TASK,
            THE_SECOND_TASKS_COMMIT_DOC[0],
            THE_SECOND_TASKS_COMMIT_DOC[1],
            THE_SECOND_TASKS_COMMIT_DOC[2],
            ITS_CHANGE[0],
            ITS_CHANGE[1],
            THE_LOG_WITHOUT_ITS_SECTION,
        ],
        refusing: &["task", "validate", "add-more"],
        acts: &[Act::Revert(Operand::Unprinted(LOG)), Act::Rerun],
        holds: &[Check::Contains(Place::Repo(LOG), "## Entries")],
        ..FINALIZE
    },
];
const A_NONCONFORMANT_EDIT_AT_THE_JOIN: [Case; 1] = [Case {
    name: "conformance-hand-broken-doc-at-the-milestones-finalize-edit-reverted",
    site: CONFORMANCE_BLOCK,
    cause: "a committed doc is edited by hand into a shape its schema refuses, and the \
            finalize of a milestone none of whose sub-tasks touched it is asked",
    state: a_committed_log![
        A_FAN_OUT_WITH_WORK[0],
        A_FAN_OUT_WITH_WORK[1],
        A_FAN_OUT_WITH_WORK[2],
        A_FAN_OUT_WITH_WORK[3],
        A_FAN_OUT_WITH_WORK[4],
        THE_LOG_WITHOUT_ITS_SECTION,
    ],
    refusing: &["milestone", "finalize", "fan-out"],
    acts: &[Act::Revert(Operand::Unprinted(LOG)), Act::Rerun],
    holds: &[
        Check::RegularFile(Place::Repo("work.txt")),
        Check::Contains(Place::Repo(LOG), "## Entries"),
    ],
    ..FINALIZE
}];
const A_MIGRATION_MINTED_OVER_THE_BROKEN_DOC: [Case; 1] = [Case {
    name: "conformance-hand-broken-doc-migrated-in-place-nothing-authored-edit-reverted",
    site: CONFORMANCE_BLOCK,
    cause: "a committed doc is broken by hand and a migration task is minted over it; \
            nothing is authored yet, and the migration's finalize is asked",
    state: a_committed_log![
        THE_LOG_WITHOUT_ITS_SECTION,
        Step::Jigc(&["migrate", LOG, "--as", "decisions-log"]),
    ],
    refusing: &["task", "finalize", THE_LOGS_MIGRATION],
    acts: &[Act::Revert(Operand::Unprinted(LOG)), Act::Rerun],
    holds: &[Check::Contains(Place::Repo(LOG), "## Entries")],
    ..FINALIZE
}];

// ---------------------------------------------------------------------------
// `provenance_io_finding` · `finalize.provenance-io` and `io_finding` ·
// `task.working-area-io` — a file of the task's area that cannot be read as what it is
// ---------------------------------------------------------------------------

/// Put a directory where a file of the task's area stands — a shell line a `git` alias
/// runs, since a state's step makes a directory only where nothing is.
macro_rules! a_directory_in_place_of {
    ($file:literal) => {
        Step::Git(&[
            "-c",
            concat!("alias.swap=!rm ", $file, " && mkdir ", $file),
            "swap",
        ])
    };
}

/// **The task's provenance manifest is not what jigc wrote**, and the finalize is asked.
/// The route names one entry — the task's area — and one kind of fault, *a disk or
/// permissions problem*: the case makes that entry readable, which it was, and runs the
/// finalize again.
macro_rules! manifest_unread {
    ($name:literal, $cause:literal, $damage:expr) => {
        [Case {
            name: concat!("finalize-provenance-", $name, "-area-made-readable"),
            site: PROVENANCE_IO,
            cause: $cause,
            state: ready![ITS_CHANGE[0], ITS_CHANGE[1], $damage],
            acts: &[Act::MakeReadable(Operand::Span(0)), Act::Rerun],
            holds: &[Check::Absent(Place::Repo(TASK_AREA))],
            ..FINALIZE
        }]
    };
}
const THE_MANIFEST_NOT_JSON: [Case; 1] = manifest_unread!(
    "manifest-not-json",
    "the task's `docs/provenance.json` holds text that is no JSON",
    Step::File(
        Place::Repo(".jigc/tasks/write-a-note/docs/provenance.json"),
        "not json\n"
    )
);
const THE_MANIFEST_A_DIRECTORY: [Case; 1] = manifest_unread!(
    "manifest-a-directory",
    "a directory stands where the task's `docs/provenance.json` was",
    a_directory_in_place_of!(".jigc/tasks/write-a-note/docs/provenance.json")
);
const THE_MANIFEST_A_LINK_TO_ITSELF: [Case; 1] = manifest_unread!(
    "manifest-a-link-to-itself",
    "the task's `docs/provenance.json` is a symbolic link to itself",
    Step::Git(&[
        "-c",
        "alias.loop=!ln -sf provenance.json .jigc/tasks/write-a-note/docs/provenance.json",
        "loop",
    ])
);

/// **A `jigc` command cannot read or make an entry of a task's working area**, over a
/// state that is no permission: the route names `.jigc/` and *a disk or permissions
/// problem*; the case makes `.jigc/` writable, which it was, and runs the command again.
/// Where the command is a create that stages a doc, what must hold once it has gone
/// through is that the task's manifest is a file again — the create records the doc there.
macro_rules! area_io {
    (
        $name:literal, $cause:literal, state: $state:expr, refusing: $refusing:expr
        $(, holds: $holds:expr)? $(,)?
    ) => {
        [Case {
            name: concat!("working-area-io-", $name, "-workbench-made-writable"),
            site: WORKING_AREA_IO,
            cause: $cause,
            state: $state,
            refusing: $refusing,
            acts: &[Act::MakeWritable(Operand::Span(0)), Act::Rerun],
            $(holds: $holds,)?
            ..FINALIZE
        }]
    };
}
const THE_TASKS_DIRECTORY_A_FILE: [Case; 1] = area_io!(
    "tasks-directory-a-file",
    "a regular file stands at `.jigc/tasks`, and a task is minted: its working area cannot \
     be opened",
    state: &[
        Step::Jigc(&["setup"]),
        Step::File(Place::Repo(".jigc/tasks"), "x\n"),
    ],
    refusing: &["start", "--workflow", "decided-task", "write a note"],
);
const THE_ROLES_RECORD_NOT_JSON: [Case; 1] = area_io!(
    "roles-record-not-json",
    "the task's `roles.json` holds text that is no JSON, and a doc is created in the task: \
     the bound roles cannot be read",
    state: decided![Step::File(
        Place::Repo(".jigc/tasks/write-a-note/roles.json"),
        "not json\n"
    )],
    refusing: THE_CREATE,
);
const THE_MANIFEST_A_DIRECTORY_AT_A_CREATE: [Case; 1] = area_io!(
    "manifest-a-directory-at-a-create",
    "a directory stands where the task's `docs/provenance.json` was, and a doc is created \
     in the task: the instance cannot be provisioned",
    state: decided![a_directory_in_place_of!(
        ".jigc/tasks/write-a-note/docs/provenance.json"
    )],
    refusing: THE_CREATE,
    holds: &[Check::RegularFile(Place::Repo(
        ".jigc/tasks/write-a-note/docs/provenance.json"
    ))],
);
const A_DIRECTORY_AT_THE_STAGED_DOCS_PATH: [Case; 1] = area_io!(
    "a-directory-at-the-staged-docs-path",
    "a directory stands at the path the task stages the doc under, and the doc is created: \
     the staged instance cannot be read",
    state: decided![Step::Dir(Place::Repo(
        ".jigc/tasks/write-a-note/docs/decisions-log:decisions-log.md"
    ))],
    refusing: THE_CREATE,
);
const THE_MANIFEST_A_DIRECTORY_AT_A_COPY_IN: [Case; 1] = area_io!(
    "manifest-a-directory-at-a-copy-in",
    "a directory stands where a second task's `docs/provenance.json` was, and the committed \
     doc is created again in that task: the committed instance cannot be copied in",
    state: a_committed_log![
        A_SECOND_TASK,
        a_directory_in_place_of!(".jigc/tasks/add-more/docs/provenance.json"),
    ],
    refusing: &[
        "doc",
        "create",
        "decisions-log",
        "--title",
        "Decisions Log",
        "--task",
        "add-more",
    ],
    holds: &[Check::RegularFile(Place::Repo(
        ".jigc/tasks/add-more/docs/provenance.json"
    ))],
);

/// The cases of the gate, a group a cause — and, within a group, a case a way out of the
/// cause's route.
pub const CASES: &[&[Case]] = &[
    &A_TRACKED_FILE_MODIFIED,
    &A_NEW_FILE_NO_INDEX_HAS,
    &A_TRACKED_FILE_DELETED,
    &A_STALE_INDEX_LOCK,
    &THE_STAMP_IGNORED,
    &THE_DOCS_HOME_IGNORED,
    &A_CHANGE_STAGED_BEFORE_THE_TASK,
    &A_DELETION_STAGED_BEFORE_THE_TASK,
    &A_CHANGE_AT_THE_PREVIEW,
    &A_DELETION_AT_THE_PREVIEW,
    &A_CHANGE_AT_THE_MILESTONES_BOUNDARY,
    &A_DELETION_AT_THE_MILESTONES_BOUNDARY,
    &A_PROMOTING_DOC_IN_AN_AMEND,
    &A_COMMIT_ON_TOP,
    &THE_PINNED_COMMIT_REWRITTEN,
    &A_WRITE_FROM_A_LINKED_WORKTREE,
    &A_WRITE_FROM_A_LINKED_WORKTREE_WITH_CODE_STAGED,
    &A_WRITE_WHERE_THE_TASK_STAGES_A_DOC,
    &A_WRITE_FROM_A_DETACHED_WORKTREE,
    &A_WRITE_BESIDE_STAGED_CODE_IN_THE_OTHER_STANDINGS,
    &A_WRITE_FROM_A_FAN_OUT_WORKTREE,
    &A_STAGED_DOC_FINALIZED_FROM_A_LINKED_WORKTREE,
    &THE_MAIN_CHECKOUT_MOVED_OVER_THE_DOC,
    &THE_MAIN_INDEX_HOLDS_WHAT_NO_SNAPSHOT_COVERS,
    &A_MINT_FROM_A_LINKED_WORKTREE,
    &A_MINT_FROM_A_FAN_OUT_WORKTREE,
    &A_CONFLICT_AT_THE_TASKS_DOOR,
    &A_MIGRATION_OVER_A_STALE_BASELINE,
    &A_MIGRATION_SOURCE_EDITED_AFTER_THE_MINT,
    &THE_RECORD_EDITED_BY_HAND,
    &A_CONFLICT_AT_THE_JOIN,
    &GIT_CANNOT_COMPARE_THE_RECORD,
    &A_NONCONFORMANT_EDIT_AT_THE_TASKS_DOOR,
    &A_NONCONFORMANT_EDIT_AT_THE_JOIN,
    &A_MIGRATION_MINTED_OVER_THE_BROKEN_DOC,
    &THE_MANIFEST_NOT_JSON,
    &THE_MANIFEST_A_DIRECTORY,
    &THE_MANIFEST_A_LINK_TO_ITSELF,
    &THE_TASKS_DIRECTORY_A_FILE,
    &THE_ROLES_RECORD_NOT_JSON,
    &THE_MANIFEST_A_DIRECTORY_AT_A_CREATE,
    &A_DIRECTORY_AT_THE_STAGED_DOCS_PATH,
    &THE_MANIFEST_A_DIRECTORY_AT_A_COPY_IN,
];

/// The sites of the family that have no case, each with why: none of the ones taken in so
/// far.
pub const REASONS: &[Reason] = &[
    Reason {
        site: DRIFT_STORE,
        why: Why::NotConstructible(
            "the refusal is printed at exit 0. `jigc validate` is its one door — \
             `detect_committed_store` has no other caller — and that command flips its exit \
             only for the codes of `render::STORE_EXIT_FLIPS`, which `file-state.hash-matches` \
             is not among. Driven: a committed doc edited by hand, `jigc validate` prints \
             the finding as *blocking (gates at finalize)* and exits 0. The frame reads a \
             refusal off a non-zero exit, so no state provokes one here",
        ),
    },
    Reason {
        site: SOURCE_PATH_IO,
        why: Why::NotConstructible(
            "both places that raise it read `<task>/source-path` a second time: the same \
             finalize's own validation reads the file first and answers a fault of it with \
             a plain error and no code. Driven: a directory at that path, and `jigc task \
             finalize` exits 1 with *could not read the source path for task*. Only a file \
             that changes between two reads of one process reaches this site",
        ),
    },
];

/// **What was read for each site, and how many causes it holds** — a hand count, with its
/// derivation; the causes that have no case, each with why; and the ways out of its route
/// that are not followed, each with why.
pub const READINGS: &[Reading] = &[
    Reading {
        site: NOTHING_STAGED,
        read: &[
            "crates/cli/src/task.rs::nothing_staged_finding",
            "crates/cli/src/task.rs::finalize — the arm that turns the planner's `finalize.empty-commit` into this refusal, and what it counts as a diff",
            "crates/cli/src/task.rs::git_dirty_paths — what counts as a change in the tree",
        ],
        causes: 3,
        derivation: "the planner answered `finalize.empty-commit` — no staged code, no staged \
                     doc whose promotion changes `HEAD`, no pending path under `.jigc/config`, \
                     no owner artifact — the task is not doc-only, and `git status`, asked \
                     with untracked files listed, is not empty. Three kinds of line it can \
                     hold: a tracked file modified, a file no index has, a tracked file \
                     deleted. One route for all three",
        without_case: &[],
        not_followed: &[(
            "the reader's own re-run of the finalize, with the task's id",
            Why::NotConstructible(
                "the route prints `jigc task finalize`, a command line of the product, and \
                 where a route prints one the command is the route's to give: the frame \
                 runs the refusing command again only where no such line is printed. \
                 Driven by hand — the change added by its path, `jigc task finalize \
                 write-a-note` — it exits 0 with one commit",
            ),
        )],
    },
    Reading {
        site: STAGE_FAILED,
        read: &[
            "crates/cli/src/task.rs::stage_failed_finding",
            "crates/cli/src/task.rs::finalize — the arm that answers a failed stage phase",
            "crates/cli/src/task.rs::stage_index_honoring, stage_migration and stage_doc_only — the three `git add` calls whose failure this is, and what each names",
        ],
        causes: 4,
        derivation: "three arms stage — the ordinary one, the migration's, the doc-only one \
                     — and each hands a failed `git add` to this one constructor, under one \
                     route: the arm is no cause. The causes are what git refuses the add \
                     over: a lock in its directory; an ignore rule over one of the three \
                     paths jigc commits with the work, where no index has the path (one \
                     cause, three paths — followed over the version stamp); an ignore rule \
                     over the home a doc promotes to, which the same `git add` names; and \
                     any other failure of it",
        without_case: &[(
            "git fails the `git add` some other way — an index it cannot write, an object \
             it cannot store",
            Why::NotConstructible(
                "an index or an object store git cannot write is a permission bit changed \
                 by hand, a declared bound; and an index git cannot read refuses the \
                 finalize's earlier questions to git before the stage phase is reached",
            ),
        )],
        not_followed: &[(
            "the rule dropped from a `.gitignore` that holds other rules too",
            Why::NotConstructible(
                "the frame has no act that takes one line out of a file: the case's \
                 `.gitignore` holds the one rule, and the file removed is the rule dropped",
            ),
        )],
    },
    Reading {
        site: CARRIED_STAGED,
        read: &[
            "crates/engine/src/finalize.rs::carried_staged_finding — one message and route a boundary: the task's finalize, its preview, the milestone's finalize; the setup boundary returns another site's finding",
            "crates/engine/src/finalize.rs::carried_staged_findings — a path is carried where the index still holds the entry, or the deletion, the pre-task snapshot recorded",
        ],
        causes: 6,
        derivation: "the snapshot taken when the work unit was minted holds two kinds of \
                     entry, a staged blob and a staged deletion, and the finding is worded \
                     by the boundary that asks: the task's finalize, the task's preview, \
                     the milestone's finalize. Three boundaries by two kinds. (The fourth \
                     boundary, `jigc setup`, returns `setup.dirty-install-path` — a site of \
                     its own, with its cases in `setup.rs`.)",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: AMEND_STAGED_DOC,
        read: &[
            "crates/cli/src/task.rs::amend_staged_doc_finding — two doors, two routes",
            "crates/cli/src/doc.rs::read_or_copy_in — the write door: the one place the six edit verbs copy a committed doc in",
            "crates/cli/src/task.rs::amend_staged_doc_findings, and its two callers in `finalize` and in the preview",
        ],
        causes: 3,
        derivation: "the write door answers an edit of a committed doc that promotes, in an \
                     amend task, before anything is staged — one place for the six edit \
                     verbs, followed through `add-item`; a create is refused earlier, by \
                     the amend workflow's own gate, under another code. Behind the door a \
                     promoting doc can stand staged in an amend task only if it was put \
                     there by hand or by an older binary: the finalize answers it, and the \
                     preview does, with the finalize's route",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: AMEND_BASE_MISMATCH,
        read: &[
            "crates/engine/src/finalize.rs::amend_base_mismatch_finding",
            "crates/cli/src/task.rs::finalize — the amend arm: the pinned commit is compared with `HEAD`, behind the index gate and the staged-doc gate",
        ],
        causes: 2,
        derivation: "one comparison — the commit the amend was minted against is not \
                     `HEAD` — reached two ways a case can tell apart: a commit on top of \
                     it, and the pinned commit itself rewritten, so that no ref holds it",
        without_case: &[],
        not_followed: &[(
            "*or return `HEAD` to the pinned commit first*",
            Why::NotConstructible(
                "the route names where `HEAD` must be and no act that puts it there: which \
                 of git's ways to move a branch the reader takes is theirs, and the frame \
                 has none; and the finalize that would follow is the refusing command, \
                 which the frame does not run again where the route prints command lines \
                 of the product",
            ),
        )],
    },
    Reading {
        site: LINKED_DOC,
        read: &[
            "crates/cli/src/task.rs::linked_worktree_doc_finding — one message; the route by door, by what the task can still do here, by staged code, by what the main checkout would do with the task, and one route from jigc's own fan-out worktree whatever the door",
            "crates/cli/src/task.rs::LinkedDocDoor, TaskHere and HomeLanding — the arms, as declared",
            "crates/cli/src/task.rs::linked_worktree_doc_findings — the backstop: one finding a staged doc that promotes",
            "crates/cli/src/task.rs::finalize — where the backstop stands: ahead of the base pin and of the amend arm",
            "crates/cli/src/doc.rs::read_or_copy_in — the write door's call, ahead of the staged read",
        ],
        causes: 12,
        derivation: "the write door, from a linked worktree of the user's: three standings \
                     of the task there — it commits code, it already stages a doc, the \
                     worktree's `HEAD` is detached — by whether the worktree's index holds \
                     staged code, six. The backstop over a doc already staged, by what the \
                     main checkout would do with the task: take it, refuse its base, commit \
                     staged entries no snapshot of the task covers — three at the finalize, \
                     and the preview raises the same finding, a fourth. And from jigc's own \
                     fan-out worktree there is one route whatever the door: the write door \
                     there, and the backstop there — two",
        without_case: &[(
            "the backstop from jigc's own fan-out worktree",
            Why::NotConstructible(
                "a fan-out worktree's `HEAD` is detached by construction, and the posture \
                 family answers first: `jigc task finalize` and `jigc task validate` of a \
                 task that stages a doc both exit 1 there with `repo.head-detached` and \
                 never reach this refusal — driven at both",
            ),
        )],
        not_followed: &[(
            "the write itself, made in the task the front door mints — *write it in a task \
             started from the main checkout*, *make this write and finalize there*",
            Why::NotConstructible(
                "the route sends its reader to the front door, `jigc start \"<intent>\"`, \
                 which under the shipped default presents the workflows: the line that \
                 mints, and the write that follows it, are printed by that door and by no \
                 route. The case that moves the change follows every printed line up to \
                 there",
            ),
        )],
    },
    Reading {
        site: LINKED_MINT,
        read: &[
            "crates/cli/src/task.rs::linked_worktree_mint_finding — one route, and the fan-out worktree's in front of it",
            "crates/cli/src/start.rs::refuse_doc_only_mint_from_a_code_only_checkout — the mint of a workflow whose finalize commits docs only",
            "crates/cli/src/migrate.rs::migrate_in_repo — the guard ahead of the migration's mint, and the re-run it words",
        ],
        causes: 4,
        derivation: "two doors mint a task whose only product is a doc — `jigc migrate`, \
                     and `jigc start --workflow` over a workflow that composes the \
                     doc-only finalize step — and each is asked from a linked worktree of \
                     the user's or from jigc's own fan-out worktree, where the route puts \
                     the milestone first",
        without_case: &[],
        not_followed: &[(
            "the branch merged into the main checkout first, where only the branch carries \
             the source — *(merge the branch there)*",
            Why::NotConstructible(
                "the route names the act and prints no line for it, and the frame has no \
                 act that merges: the case's source is one both checkouts hold",
            ),
        )],
    },
    Reading {
        site: CONFLICT_BLOCK,
        read: &[
            "crates/engine/src/file_state.rs::conflict_block_finding and ConflictBlock — six presentations, a message and a route each",
            "crates/engine/src/file_state.rs::reconcile_committed and reconcile_held — the three arms that raise it for a doc a work unit touched",
            "crates/engine/src/file_state.rs::ConflictBlock::finding_at, and its one caller in crates/cli/src/task.rs — a migration's source edited after the mint",
            "crates/cli/src/milestone.rs::reconcile_record_preflight and record_conflict_block — the record's doors",
        ],
        causes: 6,
        derivation: "a presentation is a cause — the message and the route differ by it: a \
                     task that holds the doc copied in; a migration whose source sits on a \
                     stale baseline; a migration whose source was edited after its mint; the \
                     milestone's record where jigc holds the hash of its last write; the \
                     record where it holds none and `HEAD` is the witness; and the join, \
                     over a sub-task's staged copy. Within the first, three arms of the \
                     sweep word it alike — the copy-in witness says *edited*, no witness \
                     and the file drifted, no hash and git says *modified* — and one is \
                     followed",
        without_case: &[],
        not_followed: &[
            (
                "the refused command run again behind the act, where the route prints a \
                 `jigc` line for another way — *and re-run this command*, *run this \
                 finalize again*, *then re-run the join*",
                Why::NotConstructible(
                    "the frame runs the refusing command again only where the route prints \
                     no command line of the product; these routes print one for their \
                     other way. The case ends on the act and holds what the act leaves",
                ),
            ),
            (
                "the record's hash dropped with `jigc unmanage`, where nobody edited the \
                 record — a checkout that converts line endings",
                Why::NotConstructible(
                    "the way is for a state in which the record's bytes changed and no \
                     edit was made; its last step is the refused command again, which the \
                     frame does not run beside a printed `jigc` line",
                ),
            ),
            (
                "*else revert the commit that changed it* · *discard the sub-task that \
                 staged this doc* · *undo the edit made since the mint*",
                Why::NotConstructible(
                    "each names an act and prints no line for it, on an object the route \
                     does not print: a commit, a sub-task, an edit",
                ),
            ),
        ],
    },
    Reading {
        site: UNANSWERED_PIN,
        read: &[
            "crates/engine/src/file_state.rs::unanswered_pin_finding, and the arm of reconcile_committed that raises it",
            "crates/cli/src/task.rs::git_against_rev — the four places git's answer is *unanswered*",
            "crates/cli/src/milestone.rs::reconcile_record_preflight — the door that reaches the arm with no hash and no copy-in witness",
        ],
        causes: 4,
        derivation: "the arm needs a doc a work unit touched, no hash recorded for it and \
                     no copy-in witness — which a doc staged by this binary always has, so \
                     the door that reaches it is the milestone record's, after `jigc \
                     unmanage` — and a git that does not answer. Four ways it does not: \
                     `ls-tree` of the pin fails; `ls-files` fails; `diff --quiet` exits \
                     neither 0 nor 1; the comparison through a scratch index fails",
        without_case: &[
            (
                "`git ls-files` fails — git's index cannot be read",
                Why::NotConstructible(
                    "provoked by hand at the record's door, with the index's bytes \
                     replaced; the one way out is that index put back, and the frame has \
                     no act that restores a file to bytes of the state's own. With it put \
                     back by hand the re-run exits 0",
                ),
            ),
            (
                "`git ls-tree` of the pin fails",
                Why::NotConstructible(
                    "no state found: a branch that names a missing object reads as an \
                     unborn `HEAD`, which is another arm",
                ),
            ),
            (
                "the comparison through a scratch index fails",
                Why::NotConstructible("no state found that makes it fail"),
            ),
        ],
        not_followed: &[],
    },
    Reading {
        site: CONFORMANCE_BLOCK,
        read: &[
            "crates/engine/src/file_state.rs::conformance_block_finding, and the arm of reconcile_committed that raises it: a recorded doc that drifted, untouched by the work unit that sweeps, failing the conformance gate",
            "crates/cli/src/task.rs::validate and the milestone's merged gate — the doors that sweep",
        ],
        causes: 4,
        derivation: "one arm and one route; the cause is the door that sweeps a doc it did \
                     not touch: a task's finalize, a task's preview, a milestone's \
                     finalize — and a migration task minted over the broken doc, before \
                     anything is authored in it. What the gate's first error is — a section \
                     missing, a required slot empty, a field twice — changes the message's \
                     tail and nothing of the route; the cases remove a section's heading",
        without_case: &[],
        not_followed: &[(
            "*fix the file to restore conformance*",
            Why::NotConstructible(
                "the reader's own edit of the doc's prose and headings: the frame has no \
                 act that writes a doc into the shape its schema asks for. Driven by hand \
                 at the task's door: with the prose put back the finalize exits 0 and \
                 absorbs the edit",
            ),
        )],
    },
    Reading {
        site: PROVENANCE_IO,
        read: &[
            "crates/engine/src/finalize.rs::provenance_io_finding, and plan_clobber_guard, which reads the manifest",
            "crates/engine/src/state.rs::ProvenanceRecord::load — what fails a load, and what reads as empty",
        ],
        causes: 4,
        derivation: "the finalize's planner loads `<task>/docs/provenance.json`; a file \
                     that is absent, and a link that leads nowhere, read as empty. A load \
                     fails where the bytes are no JSON, where a directory stands there, \
                     where the entry is a link that leads to itself — and where the file \
                     may not be read",
        without_case: &[(
            "the manifest may not be read",
            Why::NotConstructible(
                "a permission bit changed by hand — a declared bound of the catalog, not \
                 constructed",
            ),
        )],
        not_followed: &[(
            "the manifest itself put right",
            Why::NotConstructible(
                "the route prints the task's area and no act on the manifest; where a \
                 route prints a path, the frame takes no other object from a case. What \
                 the manifest held is jigc's own record, and no command prints it back",
            ),
        )],
    },
    Reading {
        site: WORKING_AREA_IO,
        read: &[
            "crates/engine/src/state.rs::io_finding — one route, whatever was being done",
            "crates/engine/src/state.rs::mint_task, stage_minted, create_gated, create_incumbent and copy_in — every call that hands it an error, by the words it is handed",
            "crates/cli/src/doc.rs::is_migration — the read that answers a fault of `source-path` first",
        ],
        causes: 10,
        derivation: "a cause is what was being done when the I/O call failed, as the \
                     message says it: open the working area; write the base pin; write \
                     the task intent; write the task workflow id; read the migration \
                     source path; copy in the committed instance; provision the instance; \
                     read the staged instance; read the bound roles; record the bound role. \
                     Each followed one is a state that is no permission: a file where a \
                     directory is opened, a directory where a file is read, text that is \
                     no JSON",
        without_case: &[
            (
                "write the base pin",
                Why::NotConstructible(
                    "the directory was made by the same process a moment earlier: only a \
                     permission bit changed by hand — a declared bound — or a full disk \
                     fails the write",
                ),
            ),
            (
                "write the task intent",
                Why::NotConstructible(
                    "as the base pin: a file written into a directory the process has \
                     just made",
                ),
            ),
            (
                "write the task workflow id",
                Why::NotConstructible(
                    "as the base pin: a file written into a directory the process has \
                     just made",
                ),
            ),
            (
                "read the migration source path",
                Why::NotConstructible(
                    "`doc.rs::is_migration` reads `<task>/source-path` first and answers \
                     a fault of it with a plain error and no code — driven, with a \
                     directory there",
                ),
            ),
            (
                "record the bound role",
                Why::NotConstructible(
                    "the record is written by replacement: a link that leads nowhere at \
                     `roles.json` is replaced and the create exits 0 — driven",
                ),
            ),
        ],
        not_followed: &[],
    },
];
