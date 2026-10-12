//! The route catalog's cases for the gate — the family the list of refusal sites calls
//! `task finalize`: the refusals `jigc task finalize` raises, with the ones its preview
//! (`jigc task validate`), the `jigc doc` write doors and `jigc milestone finalize` raise
//! under the same codes. The family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here** is not the family: it is the sites of it that the fix pass since
//! the previous release touched, as the human ruled on 2026-10-11 (`DECISIONS.md` →
//! *Which families the route catalog takes in*) — the rest are rows of the scheduled list.
//! Sixteen — seven with cases first: nothing staged over a dirty tree; git refusing to
//! stage jigc's own three paths; a path that was staged before the work unit existed, at the three boundaries
//! that ask; a promoting doc in a task whose commit is an amend, at the write door and at
//! the finalize; an amend whose `HEAD` moved; and the two of a doc that promotes in a
//! checkout that commits code only — a `jigc doc` write or a finalize asked from a linked
//! worktree, and a mint whose only product is a doc. Then the reconciliation and the task
//! area's own files: a committed doc or the milestone's record changed out of band under
//! a staged copy, where git cannot say, and into a shape the schema refuses; the task's
//! provenance manifest unread; and an entry of a working area that cannot be read or made.
//! Each of those twelve has a row of [`READINGS`]; two more have a reason and no case
//! ([`REASONS`]) — a refusal printed at exit 0, and one an earlier read always answers.
//! **And the last two**: the clobber guard — a doc a work unit minted whose home is taken,
//! by a file, by what git holds where the disk shows nothing, by a file a sub-task staged,
//! by another doc of the same plan; seven arms, at the task's door and at the milestone's
//! boundary — and the empty commit, a site the caller reading had left for the port and
//! the orchestrating session's reading of the ruling takes in: the lines that compute
//! whether there is anything to commit changed, one level out from the refusal.
//!
//! **A route of the clobber guard prints the rename with the title left to its reader** —
//! `jigc doc rename <address> --to "<title>" --task <id>`, the id and the address concrete.
//! The quoted placeholder is a word a shell hands on as it stands, so the line is run as
//! printed and the doc's title is `<title>`: no word of a case's goes into it, as none
//! does into `jigc start "<intent>"` above.
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
use crate::support::route_case::{
    Act, Case, Check, Cited, Operand, Place, Promise, Says, Step,
};

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
const CLOBBER: &str =
    "crates/engine/src/finalize.rs::clobber_finding::finalize.promote-clobber#1";
const EMPTY_COMMIT: &str =
    "crates/engine/src/finalize.rs::empty_commit_finding::finalize.empty-commit#1";

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
/// from the worktree, and the route says which command names where it lands — a line cited
/// for what it prints, and held to its words (the human's ruling of 2026-10-13): the path
/// the doc promotes to, and the checkout that is — the main one, by its place. The cited
/// command is itself refused, under this refusal's own code, and says it there.
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
        acts: &[Act::RunCited(Cited {
            line: 3,
            promised: &[Promise::Line(&[
                Says::Text("`docs/decisions-log.md`"),
                Says::Text("the main checkout at"),
                Says::Path(Place::Repo("")),
            ])],
        })],
        ..FINALIZE
    },
];

/// The same door from a worktree whose `HEAD` is detached: the task *commits nothing from
/// here until it is on a branch*, and the route cites the finalize, which *names that
/// route* — held to its words: a finding over the detached `HEAD` whose own route names
/// the way onto a branch. It is another refusal's, `repo.head-detached`, and it is the
/// route the citing one promised.
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
        acts: &[Act::RunCited(Cited {
            line: 3,
            promised: &[Promise::Route(&[
                Says::Text("HEAD is detached"),
                Says::Text("`git switch <branch>`"),
            ])],
        })],
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

// ---------------------------------------------------------------------------
// `clobber_finding` · `finalize.promote-clobber` — a doc a work unit minted, whose home is
// taken: by a file, by what git holds there, by a sub-task's staged file, by another doc
// ---------------------------------------------------------------------------

/// The home of an ADR titled *Use tabs*, and the home it has once the printed rename is
/// run as printed — `--to "<title>"` slugs to `title`.
const THE_ADRS_HOME: &str = "docs/decisions/use-tabs.md";
const THE_RENAMED_ADRS_HOME: &str = "docs/decisions/title.md";

/// A task that may create an ADR — its id slugs from the intent, `pick-tabs` — and the ADR
/// authored whole in one payload, by whichever work unit.
const AN_ADR_TASK: Step = Step::Jigc(&["start", "--workflow", "record-decision", "pick tabs"]);
const ADR_AREA: &str = ".jigc/tasks/pick-tabs";
const AN_ADR_PAYLOAD: Step = Step::File(
    Place::Outside("adr.yaml"),
    "title: \"Use tabs\"\nsections:\n  - id: context\n    set:\n      context: |-\n        \
     <<Because.>>\n  - id: decision\n    set:\n      decision: |-\n        <<Tabs.>>\n  - id: \
     consequences\n    set:\n      consequences: |-\n        <<None.>>\n",
);
macro_rules! the_adr_authored_by {
    ($task:literal) => {
        Step::Jigc(&[
            "doc",
            "author",
            "adr",
            "--from-file",
            "../outside/adr.yaml",
            "--task",
            $task,
        ])
    };
}
const THE_ADRS_COMMIT_DOC: [Step; 3] = commit_doc!("pick-tabs");

/// A file somebody wrote by hand at a doc's home; the same committed, staged, and taken
/// out of the working tree again — a shell line a `git` alias runs, since a state has no
/// step that removes.
macro_rules! by_hand_at {
    ($home:expr) => {
        Step::File(Place::Repo($home), "by hand\n")
    };
}
macro_rules! staged_at {
    ($home:expr) => {
        Step::Git(&["add", $home])
    };
}
const COMMITTED: Step = Step::Git(&["commit", "-q", "-m", "a file by hand"]);
const THE_ADRS_HOME_EMPTIED: Step =
    Step::Git(&["-c", "alias.gone=!rm docs/decisions/use-tabs.md", "gone"]);
const THE_LOGS_HOME_EMPTIED: Step =
    Step::Git(&["-c", "alias.gone=!rm docs/decisions-log.md", "gone"]);

/// **An install, what the case plants before the task exists, a task with an ADR authored
/// and its commit doc complete, and what the case plants afterwards.**
macro_rules! an_adr_task_after {
    ([$($before:expr),* $(,)?] $(, $plant:expr)* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            $($before,)*
            AN_ADR_TASK,
            AN_ADR_PAYLOAD,
            the_adr_authored_by!("pick-tabs"),
            THE_ADRS_COMMIT_DOC[0],
            THE_ADRS_COMMIT_DOC[1],
            THE_ADRS_COMMIT_DOC[2],
            $($plant,)*
        ]
    };
}

/// The finalize of the task that minted the ADR.
const THE_ADR_TASKS_FINALIZE: &[&str] = &["task", "finalize", "pick-tabs"];

/// The two readings of a printed line none of whose placeholders the refusal lists a value
/// for.
const WHERE_AN_UNLISTED_VALUE_COMES_FROM: &str = "*`jigc doc create <doctype> --title <title> \
    --slug <slug> --task <id>`* · *`jigc migrate <file> --as <doctype>`* · *`jigc task discard \
    <id> --force`* · *`jigc task finalize <id>`*: the surface contract reads a backticked line \
    with a word in angle brackets as a shape the reader fills, and the frame fills one with a \
    value the refusal's own output lists. This refusal lists the destination and nothing \
    else — no task id, no doctype, no slug — so no placeholder of these lines can be filled \
    from what was printed. (a) A value comes from the refusal's own output, as every other \
    value a route needs: then the route owes them and is red as it stands; (b) from somewhere \
    else the reader knows of: then the frame has no case, a value of the case's own being no \
    operand. The case runs the lines as printed, which no shell splits. It is what the \
    pilot's first question left open, over another route; the contract's nearest line is its \
    reason for `<workflow-id>` — *the set it must be filled from is named in the message \
    instead* — and it declares `<task-id>` not derivable from a finding's target";

/// **An ordinary task's ADR, and a file that turned up at its home after the mint** — the
/// arm the guard was built for. Every way the route offers ends in a line with a
/// placeholder the refusal lists no value for, or names an act and prints no line.
const A_FILE_AT_THE_HOME_OF_A_TASKS_DOC: &str = "an ordinary task minted a doc whose id is \
    its author's to choose, and a file stands at the doc's home that was not there at the \
    mint — written by hand, in no commit";
const A_TASKS_DOC_OVER_A_FILE: [Case; 2] = [
    Case {
        name: "clobber-task-file-at-the-home-re-created-with-a-slug-as-printed",
        site: CLOBBER,
        cause: A_FILE_AT_THE_HOME_OF_A_TASKS_DOC,
        state: an_adr_task_after!([], by_hand_at!(THE_ADRS_HOME)),
        refusing: THE_ADR_TASKS_FINALIZE,
        acts: &[Act::Run(2), Act::Run(4)],
        holds: &[Check::Absent(Place::Repo(ADR_AREA))],
        untouched: &[THE_ADRS_HOME],
        doubt: Some(WHERE_AN_UNLISTED_VALUE_COMES_FROM),
        ..FINALIZE
    },
    Case {
        name: "clobber-task-file-at-the-home-occupant-migrated-as-printed",
        site: CLOBBER,
        cause: A_FILE_AT_THE_HOME_OF_A_TASKS_DOC,
        state: an_adr_task_after!([], by_hand_at!(THE_ADRS_HOME)),
        refusing: THE_ADR_TASKS_FINALIZE,
        acts: &[Act::Run(3), Act::Run(4)],
        holds: &[Check::Absent(Place::Repo(ADR_AREA))],
        untouched: &[THE_ADRS_HOME],
        doubt: Some(WHERE_AN_UNLISTED_VALUE_COMES_FROM),
        ..FINALIZE
    },
];

/// A foreign file the repository holds committed, a migration of it to an ADR — the id
/// `jigc migrate` mints for that source — and its rewrite authored.
const A_FOREIGN_ADR_COMMITTED: [Step; 3] = [
    Step::File(Place::Repo("notes/old.md"), "# Use tabs\n\nWe use tabs.\n"),
    Step::Git(&["add", "notes/old.md"]),
    Step::Git(&["commit", "-q", "-m", "an old decision"]),
];
const THE_ADRS_MIGRATION: &str = "migrate-adr-notes-old-e1a7d84cafc2";
const THE_ADRS_MIGRATION_AREA: &str = ".jigc/tasks/migrate-adr-notes-old-e1a7d84cafc2";
const THE_ADRS_MIGRATION_MINTED: Step = Step::Jigc(&["migrate", "notes/old.md", "--as", "adr"]);
const THE_ADRS_MIGRATIONS_COMMIT_DOC: [Step; 3] = commit_doc!("migrate-adr-notes-old-e1a7d84cafc2");

/// **An install, a foreign decision committed, what the case plants before the migration
/// exists, the migration minted with its rewrite authored and its commit doc complete, and
/// what the case plants afterwards.**
macro_rules! an_adr_migration_after {
    ([$($before:expr),* $(,)?] $(, $plant:expr)* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            A_FOREIGN_ADR_COMMITTED[0],
            A_FOREIGN_ADR_COMMITTED[1],
            A_FOREIGN_ADR_COMMITTED[2],
            $($before,)*
            THE_ADRS_MIGRATION_MINTED,
            AN_ADR_PAYLOAD,
            the_adr_authored_by!("migrate-adr-notes-old-e1a7d84cafc2"),
            THE_ADRS_MIGRATIONS_COMMIT_DOC[0],
            THE_ADRS_MIGRATIONS_COMMIT_DOC[1],
            THE_ADRS_MIGRATIONS_COMMIT_DOC[2],
            $($plant,)*
        ]
    };
}
const THE_ADRS_MIGRATIONS_FINALIZE: &[&str] = &["task", "finalize", THE_ADRS_MIGRATION];

const A_FILE_AT_A_MIGRATIONS_DESTINATION: &str = "a migration's doc promotes to a path that \
    is not its recorded source, and a file stands there that was not there when the rewrite \
    was authored";
const A_MIGRATIONS_DOC_OVER_A_FILE: [Case; 2] = [
    Case {
        name: "clobber-migration-file-at-the-destination-discarded-and-minted-again-as-printed",
        site: CLOBBER,
        cause: A_FILE_AT_A_MIGRATIONS_DESTINATION,
        state: an_adr_migration_after!([], by_hand_at!(THE_ADRS_HOME)),
        refusing: THE_ADRS_MIGRATIONS_FINALIZE,
        acts: &[Act::Run(2), Act::Run(3), Act::Run(5), Act::Run(6)],
        holds: &[Check::Absent(Place::Repo(THE_ADRS_MIGRATION_AREA))],
        untouched: &[THE_ADRS_HOME, "notes/old.md"],
        doubt: Some(WHERE_AN_UNLISTED_VALUE_COMES_FROM),
        ..FINALIZE
    },
    Case {
        name: "clobber-migration-file-at-the-destination-finalized-and-approved-as-printed",
        site: CLOBBER,
        cause: A_FILE_AT_A_MIGRATIONS_DESTINATION,
        state: an_adr_migration_after!([], by_hand_at!(THE_ADRS_HOME)),
        refusing: THE_ADRS_MIGRATIONS_FINALIZE,
        acts: &[Act::Run(5), Act::Run(6)],
        holds: &[Check::Absent(Place::Repo(THE_ADRS_MIGRATION_AREA))],
        untouched: &[THE_ADRS_HOME, "notes/old.md"],
        doubt: Some(WHERE_AN_UNLISTED_VALUE_COMES_FROM),
        ..FINALIZE
    },
];

// The fixed-identity arm at the task's door: the decisions log, whose home is one path.

/// The two readings of a `git` verb a route names with no operand.
const WHAT_GIT_MV_IS: &str = "*move the file out of the doc's home … — with `git mv` if git \
    has it staged, since the index entry has to move with it*: (a) the backticked span is a \
    command line to run as printed, like every `git` line a route prints — it has no \
    operand, and git answers with its usage; (b) it is the name of a git verb, to be run on \
    the file's path with a destination of the reader's own — then the route prints no line \
    for the move, and the frame's move is a file act that leaves the index entry behind. \
    The case follows (a). The surface contract says of a `git` span with no operand that it \
    *names a command rather than a file* (the printed-path fence); it is the pilot's sixth \
    question, over another verb";

const A_FILE_AT_THE_LOGS_HOME: &str = "an ordinary task minted the decisions log — a doc \
    whose home is one fixed path — and a file in no commit stands at that home, written \
    after the mint";
const A_FIXED_DOC_OVER_A_FILE: [Case; 2] = [
    Case {
        name: "clobber-fixed-file-at-the-logs-home-moved-out-and-finalized-as-printed",
        site: CLOBBER,
        cause: A_FILE_AT_THE_LOGS_HOME,
        state: decided![THE_LOG_CREATED, by_hand_at!(LOG)],
        acts: &[Act::MoveOut(Operand::Span(0)), Act::Run(2)],
        holds: &[
            Check::Contains(Place::Repo(LOG), "# Decisions Log"),
            Check::Contains(Place::Moved(LOG), "by hand"),
            Check::Absent(Place::Repo(TASK_AREA)),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-fixed-file-at-the-logs-home-task-read-dropped-and-the-file-adopted-as-printed",
        site: CLOBBER,
        cause: A_FILE_AT_THE_LOGS_HOME,
        state: decided![THE_LOG_CREATED, by_hand_at!(LOG)],
        acts: &[Act::Run(4), Act::Run(3), Act::Run(5), Act::Run(6)],
        holds: &[
            Check::Absent(Place::Repo(TASK_AREA)),
            Check::Directory(Place::Repo(
                ".jigc/tasks/migrate-decisions-log-docs-decisions-log-b31ca3264f78",
            )),
        ],
        untouched: &[LOG],
        ..FINALIZE
    },
];
const A_FIXED_DOC_OVER_A_STAGED_FILE: [Case; 1] = [Case {
    name: "clobber-fixed-file-staged-at-the-logs-home-moved-with-git-mv-as-printed",
    site: CLOBBER,
    cause: "an ordinary task minted the decisions log, and a file stands at its home that \
            was written and staged after the mint — in git's index, in no commit",
    state: decided![THE_LOG_CREATED, by_hand_at!(LOG), staged_at!(LOG)],
    acts: &[Act::Run(1), Act::Run(2)],
    holds: &[
        Check::Contains(Place::Repo(LOG), "# Decisions Log"),
        Check::Absent(Place::Repo(TASK_AREA)),
    ],
    doubt: Some(WHAT_GIT_MV_IS),
    ..FINALIZE
}];

/// **An install, a file committed by hand at the log's home and taken out of the working
/// tree, then a task that mints the log — the create meets a free home — and what the case
/// plants afterwards.**
macro_rules! a_log_minted_over_a_committed_file {
    ($($plant:expr),* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            by_hand_at!(LOG),
            staged_at!(LOG),
            COMMITTED,
            THE_LOGS_HOME_EMPTIED,
            A_DECIDED_TASK,
            ITS_COMMIT_DOC[0],
            ITS_COMMIT_DOC[1],
            ITS_COMMIT_DOC[2],
            THE_LOG_CREATED,
            $($plant,)*
        ]
    };
}
const A_FIXED_DOC_OVER_A_COMMITTED_FILE: [Case; 1] = [Case {
    name: "clobber-fixed-committed-file-at-the-logs-home-task-read-and-dropped-as-printed",
    site: CLOBBER,
    cause: "an ordinary task minted the decisions log while the committed file at its home \
            was missing from the working tree, and the file is back: `HEAD` holds it and so \
            does the disk",
    state: a_log_minted_over_a_committed_file![Step::Git(&["checkout", "HEAD", "--", LOG])],
    acts: &[Act::Run(2), Act::Run(1)],
    holds: &[Check::Absent(Place::Repo(TASK_AREA))],
    untouched: &[LOG],
    ..FINALIZE
}];

// The held arm at the task's door: nothing on disk at the home, and git holds a file there.

const AN_ADR_MINTED_OVER_A_FILE_IN_HEAD: &str = "a committed file is missing from the \
    working tree — deleted and the deletion not committed — and an ordinary task minted a \
    doc, under an id its author chooses, whose home is that path";
const A_TASKS_DOC_OVER_A_FILE_IN_HEAD: [Case; 3] = [
    Case {
        name: "clobber-held-in-head-doc-renamed-and-finalized-as-printed",
        site: CLOBBER,
        cause: AN_ADR_MINTED_OVER_A_FILE_IN_HEAD,
        state: an_adr_task_after!([
            by_hand_at!(THE_ADRS_HOME),
            staged_at!(THE_ADRS_HOME),
            COMMITTED,
            THE_ADRS_HOME_EMPTIED,
        ]),
        refusing: THE_ADR_TASKS_FINALIZE,
        acts: &[Act::Run(0), Act::Run(1)],
        holds: &[
            Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
            Check::Absent(Place::Repo(ADR_AREA)),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-held-in-head-file-brought-back-doc-renamed-and-finalized-as-printed",
        site: CLOBBER,
        cause: AN_ADR_MINTED_OVER_A_FILE_IN_HEAD,
        state: an_adr_task_after!([
            by_hand_at!(THE_ADRS_HOME),
            staged_at!(THE_ADRS_HOME),
            COMMITTED,
            THE_ADRS_HOME_EMPTIED,
        ]),
        refusing: THE_ADR_TASKS_FINALIZE,
        acts: &[Act::Run(3), Act::Run(0), Act::Run(1)],
        holds: &[
            Check::Contains(Place::Repo(THE_ADRS_HOME), "by hand"),
            Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
            Check::Absent(Place::Repo(ADR_AREA)),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-held-in-head-deletion-committed-task-read-and-dropped-as-printed",
        site: CLOBBER,
        cause: AN_ADR_MINTED_OVER_A_FILE_IN_HEAD,
        state: an_adr_task_after!([
            by_hand_at!(THE_ADRS_HOME),
            staged_at!(THE_ADRS_HOME),
            COMMITTED,
            THE_ADRS_HOME_EMPTIED,
        ]),
        refusing: THE_ADR_TASKS_FINALIZE,
        acts: &[Act::Commit(Operand::Span(4)), Act::Run(6), Act::Run(5)],
        holds: &[
            Check::Absent(Place::Repo(THE_ADRS_HOME)),
            Check::Absent(Place::Repo(ADR_AREA)),
        ],
        ..FINALIZE
    },
];

const AN_ADR_MINTED_OVER_A_FILE_IN_THE_INDEX: &str = "a file was staged at a path and taken \
    out of the working tree — in git's index, in no commit — and an ordinary task minted a \
    doc, under an id its author chooses, whose home is that path";
const A_TASKS_DOC_OVER_A_FILE_IN_THE_INDEX: [Case; 2] = [
    Case {
        name: "clobber-held-in-the-index-doc-renamed-and-finalized-as-printed",
        site: CLOBBER,
        cause: AN_ADR_MINTED_OVER_A_FILE_IN_THE_INDEX,
        state: an_adr_task_after!(
            [],
            by_hand_at!(THE_ADRS_HOME),
            staged_at!(THE_ADRS_HOME),
            THE_ADRS_HOME_EMPTIED,
        ),
        refusing: THE_ADR_TASKS_FINALIZE,
        acts: &[Act::Run(0), Act::Run(1)],
        holds: &[
            Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
            Check::Absent(Place::Repo(ADR_AREA)),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-held-in-the-index-file-brought-back-doc-renamed-and-finalized-as-printed",
        site: CLOBBER,
        cause: AN_ADR_MINTED_OVER_A_FILE_IN_THE_INDEX,
        state: an_adr_task_after!(
            [],
            by_hand_at!(THE_ADRS_HOME),
            staged_at!(THE_ADRS_HOME),
            THE_ADRS_HOME_EMPTIED,
        ),
        refusing: THE_ADR_TASKS_FINALIZE,
        acts: &[Act::Run(3), Act::Run(0), Act::Run(1)],
        holds: &[
            Check::Contains(Place::Repo(THE_ADRS_HOME), "by hand"),
            Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
            Check::Absent(Place::Repo(ADR_AREA)),
        ],
        ..FINALIZE
    },
];

const THE_LOG_MINTED_OVER_A_FILE_GIT_HOLDS: [Case; 2] = [
    Case {
        name: "clobber-held-in-head-fixed-task-read-dropped-and-the-file-brought-back-as-printed",
        site: CLOBBER,
        cause: "a committed file is missing from the working tree, and an ordinary task \
                minted the decisions log, whose one fixed home is that path",
        state: a_log_minted_over_a_committed_file![],
        acts: &[Act::Run(1), Act::Run(0), Act::Run(3)],
        holds: &[
            Check::Absent(Place::Repo(TASK_AREA)),
            Check::Contains(Place::Repo(LOG), "by hand"),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-held-in-the-index-fixed-task-read-dropped-and-the-file-brought-back-as-printed",
        site: CLOBBER,
        cause: "a file was staged and taken out of the working tree, and an ordinary task \
                minted the decisions log, whose one fixed home is that path",
        state: decided![
            THE_LOG_CREATED,
            by_hand_at!(LOG),
            staged_at!(LOG),
            THE_LOGS_HOME_EMPTIED,
        ],
        acts: &[Act::Run(1), Act::Run(0), Act::Run(3)],
        holds: &[
            Check::Absent(Place::Repo(TASK_AREA)),
            Check::Contains(Place::Repo(LOG), "by hand"),
        ],
        ..FINALIZE
    },
];

// The held arm in a migration: the rename's exit ends at the migration's own boundary.

/// What the held arm's route promises of the review it cites in a migration — *re-run
/// `jigc task finalize <id>` to review the fidelity diff*: the diff itself, the foreign
/// source's own line against the rewrite's, under the home the rename gave the doc. A
/// migration's plain finalize exits 4 by design, and a cited line is held to its words.
const THE_FIDELITY_DIFF_OF_THE_RENAMED_ADR: &[Promise] = &[
    Promise::Line(&[Says::Text(
        "+++ canonical rewrite → docs/decisions/title.md",
    )]),
    Promise::Line(&[Says::Text("- We use tabs.")]),
    Promise::Line(&[Says::Text("+ Because.")]),
];

/// **A migration's doc over a path git holds and the disk does not, followed two ways**:
/// every printed line in the route's order, the review among them — cited for the diff it
/// prints, the approval behind it held to exit 0; and the rename and the approval alone.
macro_rules! a_migrations_doc_held {
    ($holder:literal, $cause:literal, state: $state:expr $(,)?) => {
        [
            Case {
                name: concat!(
                    "clobber-held-in-", $holder, "-migration-doc-renamed-reviewed-and-approved-as-printed"
                ),
                site: CLOBBER,
                cause: $cause,
                state: $state,
                refusing: THE_ADRS_MIGRATIONS_FINALIZE,
                acts: &[
                    Act::Run(0),
                    Act::RunCited(Cited {
                        line: 1,
                        promised: THE_FIDELITY_DIFF_OF_THE_RENAMED_ADR,
                    }),
                    Act::Run(2),
                ],
                holds: &[
                    Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
                    Check::Absent(Place::Repo(THE_ADRS_MIGRATION_AREA)),
                    Check::Absent(Place::Repo("notes/old.md")),
                ],
                ..FINALIZE
            },
            Case {
                name: concat!(
                    "clobber-held-in-", $holder, "-migration-doc-renamed-and-approved-as-printed"
                ),
                site: CLOBBER,
                cause: $cause,
                state: $state,
                refusing: THE_ADRS_MIGRATIONS_FINALIZE,
                acts: &[Act::Run(0), Act::Run(2)],
                holds: &[
                    Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
                    Check::Absent(Place::Repo(THE_ADRS_MIGRATION_AREA)),
                    Check::Absent(Place::Repo("notes/old.md")),
                ],
                ..FINALIZE
            },
        ]
    };
}
const A_MIGRATIONS_DOC_OVER_A_FILE_IN_HEAD: [Case; 2] = a_migrations_doc_held!(
    "head",
    "a committed file is missing from the working tree, and a migration's doc — under an \
     id its author chooses — promotes to that path, which is not the migration's source",
    state: an_adr_migration_after!([
        by_hand_at!(THE_ADRS_HOME),
        staged_at!(THE_ADRS_HOME),
        COMMITTED,
        THE_ADRS_HOME_EMPTIED,
    ]),
);
const A_MIGRATIONS_DOC_OVER_A_FILE_IN_THE_INDEX: [Case; 2] = a_migrations_doc_held!(
    "the-index",
    "a file was staged and taken out of the working tree, and a migration's doc — under \
     an id its author chooses — promotes to that path, which is not the migration's source",
    state: an_adr_migration_after!(
        [],
        by_hand_at!(THE_ADRS_HOME),
        staged_at!(THE_ADRS_HOME),
        THE_ADRS_HOME_EMPTIED,
    ),
);

/// A foreign log elsewhere, a migration of it to the decisions log, and its rewrite
/// authored — the log's home is one fixed path, which is not the migration's source.
const A_FOREIGN_LOG_COMMITTED: [Step; 3] = [
    Step::File(Place::Repo("notes/old-log.md"), "# Old decisions\n\n- We use tabs.\n"),
    Step::Git(&["add", "notes/old-log.md"]),
    Step::Git(&["commit", "-q", "-m", "an old log"]),
];
const THE_NOTES_MIGRATIONS_COMMIT_DOC: [Step; 3] =
    commit_doc!("migrate-decisions-log-notes-old-log-2173a5301d0e");
const THE_NOTES_MIGRATION_AREA: &str =
    ".jigc/tasks/migrate-decisions-log-notes-old-log-2173a5301d0e";
macro_rules! a_log_migration_after {
    ([$($before:expr),* $(,)?] $(, $plant:expr)* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            A_FOREIGN_LOG_COMMITTED[0],
            A_FOREIGN_LOG_COMMITTED[1],
            A_FOREIGN_LOG_COMMITTED[2],
            $($before,)*
            Step::Jigc(&["migrate", "notes/old-log.md", "--as", "decisions-log"]),
            Step::File(Place::Outside("rewrite.yaml"), A_REWRITE),
            Step::Jigc(&[
                "doc",
                "author",
                "decisions-log",
                "--from-file",
                "../outside/rewrite.yaml",
                "--task",
                THE_NOTES_MIGRATION,
            ]),
            THE_NOTES_MIGRATIONS_COMMIT_DOC[0],
            THE_NOTES_MIGRATIONS_COMMIT_DOC[1],
            THE_NOTES_MIGRATIONS_COMMIT_DOC[2],
            $($plant,)*
        ]
    };
}
const A_MIGRATED_LOG_OVER_A_FILE_GIT_HOLDS: [Case; 2] = [
    Case {
        name: "clobber-held-in-head-fixed-migration-read-dropped-and-the-file-brought-back-as-printed",
        site: CLOBBER,
        cause: "a committed file is missing from the working tree, and a migration's doc \
                promotes to that path — the decisions log's one fixed home, which is not \
                the migration's source",
        state: a_log_migration_after!([
            by_hand_at!(LOG),
            staged_at!(LOG),
            COMMITTED,
            THE_LOGS_HOME_EMPTIED,
        ]),
        refusing: &["task", "finalize", THE_NOTES_MIGRATION],
        acts: &[Act::Run(1), Act::Run(0), Act::Run(3)],
        holds: &[
            Check::Absent(Place::Repo(THE_NOTES_MIGRATION_AREA)),
            Check::Contains(Place::Repo(LOG), "by hand"),
        ],
        untouched: &["notes/old-log.md"],
        ..FINALIZE
    },
    Case {
        name: "clobber-held-in-the-index-fixed-migration-read-dropped-and-the-file-brought-back-as-printed",
        site: CLOBBER,
        cause: "a file was staged and taken out of the working tree, and a migration's doc \
                promotes to that path — the decisions log's one fixed home, which is not \
                the migration's source",
        state: a_log_migration_after!(
            [],
            by_hand_at!(LOG),
            staged_at!(LOG),
            THE_LOGS_HOME_EMPTIED,
        ),
        refusing: &["task", "finalize", THE_NOTES_MIGRATION],
        acts: &[Act::Run(1), Act::Run(0), Act::Run(3)],
        holds: &[
            Check::Absent(Place::Repo(THE_NOTES_MIGRATION_AREA)),
            Check::Contains(Place::Repo(LOG), "by hand"),
        ],
        untouched: &["notes/old-log.md"],
        ..FINALIZE
    },
];

// The same guard at the milestone's boundary: a sub-task's doc, and the homes of all of them.

/// A milestone and its sub-tasks — under the default workflow, which may create an ADR, or
/// under one that may create the decisions log — provisioned, with work staged in the
/// first one's worktree so that something lands whichever doc is dropped.
const A_MILESTONE: Step = Step::Jigc(&["milestone", "create", "Fan out"]);
const A_FIRST_SUB_TASK: Step = Step::Jigc(&["milestone", "add-task", "fan-out", "do a thing"]);
const A_SECOND_SUB_TASK: Step = Step::Jigc(&["milestone", "add-task", "fan-out", "second thing"]);
const A_THIRD_SUB_TASK: Step = Step::Jigc(&["milestone", "add-task", "fan-out", "third thing"]);
const A_FIRST_SUB_TASK_THAT_DECIDES: Step = Step::Jigc(&[
    "milestone",
    "add-task",
    "fan-out",
    "do a thing",
    "--workflow",
    "decided-task",
]);
const A_SECOND_SUB_TASK_THAT_DECIDES: Step = Step::Jigc(&[
    "milestone",
    "add-task",
    "fan-out",
    "second thing",
    "--workflow",
    "decided-task",
]);
const PROVISIONED: Step = Step::Jigc(&["milestone", "provision", "fan-out"]);
const WORK_IN_THE_FIRST: [Step; 2] = [A_FAN_OUT_WITH_WORK[3], A_FAN_OUT_WITH_WORK[4]];
const THE_MILESTONES_FINALIZE: &[&str] = &["milestone", "finalize", "fan-out"];
macro_rules! the_log_created_by {
    ($task:literal) => {
        Step::Jigc(&[
            "doc",
            "create",
            "decisions-log",
            "--title",
            "Decisions Log",
            "--task",
            $task,
        ])
    };
}
/// A file at a path in a sub-task's worktree, staged there.
macro_rules! staged_in_the_worktree_of {
    ($sub:literal, $path:literal) => {
        [
            Step::File(
                Place::Repo(concat!(".jigc/worktrees/", $sub, "/", $path)),
                "by hand\n",
            ),
            Step::Git(&["-C", concat!(".jigc/worktrees/", $sub), "add", $path]),
        ]
    };
}

const A_SUB_TASKS_DOC_OVER_A_FILE: [Case; 1] = [Case {
    name: "clobber-sub-task-file-at-the-home-doc-renamed-and-milestone-finalized-as-printed",
    site: CLOBBER,
    cause: "one sub-task of a milestone minted a doc under an id its author chooses, and a \
            file in no commit stands at the doc's home in the main checkout",
    state: &[
        Step::Jigc(&["setup"]),
        A_MILESTONE,
        A_FIRST_SUB_TASK,
        PROVISIONED,
        WORK_IN_THE_FIRST[0],
        WORK_IN_THE_FIRST[1],
        AN_ADR_PAYLOAD,
        the_adr_authored_by!("do-a-thing"),
        by_hand_at!(THE_ADRS_HOME),
    ],
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(0), Act::Run(1)],
    holds: &[
        Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
        Check::RegularFile(Place::Repo("work.txt")),
    ],
    untouched: &[THE_ADRS_HOME],
    ..FINALIZE
}];
const THE_SECOND_OF_TWO_SUFFIXED_ONTO_A_FILE: [Case; 1] = [Case {
    name: "clobber-sub-task-suffixed-onto-a-file-doc-renamed-and-milestone-finalized-as-printed",
    site: CLOBBER,
    cause: "two sub-tasks minted a doc of one id, the join suffixes the second's, and a \
            file stands at the home of the suffixed id",
    state: &[
        Step::Jigc(&["setup"]),
        A_MILESTONE,
        A_FIRST_SUB_TASK,
        A_SECOND_SUB_TASK,
        PROVISIONED,
        WORK_IN_THE_FIRST[0],
        WORK_IN_THE_FIRST[1],
        AN_ADR_PAYLOAD,
        the_adr_authored_by!("do-a-thing"),
        the_adr_authored_by!("second-thing"),
        by_hand_at!("docs/decisions/use-tabs-2.md"),
    ],
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(0), Act::Run(1)],
    holds: &[
        Check::Contains(Place::Repo(THE_ADRS_HOME), "Because."),
        Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
    ],
    untouched: &["docs/decisions/use-tabs-2.md"],
    ..FINALIZE
}];
const THE_FIRST_OF_TWO_OVER_A_FILE: [Case; 1] = [Case {
    name: "clobber-sub-task-first-of-two-over-a-file-both-docs-renamed-and-milestone-finalized",
    site: CLOBBER,
    cause: "two sub-tasks minted a doc of one id, and a file stands at the home of the \
            first's — the bare id: the route names a rename for each sub-task of the group",
    state: &[
        Step::Jigc(&["setup"]),
        A_MILESTONE,
        A_FIRST_SUB_TASK,
        A_SECOND_SUB_TASK,
        PROVISIONED,
        WORK_IN_THE_FIRST[0],
        WORK_IN_THE_FIRST[1],
        AN_ADR_PAYLOAD,
        the_adr_authored_by!("do-a-thing"),
        the_adr_authored_by!("second-thing"),
        by_hand_at!(THE_ADRS_HOME),
    ],
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(1), Act::Run(2), Act::Run(3)],
    holds: &[
        Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
        Check::Contains(Place::Repo("docs/decisions/title-2.md"), "Because."),
    ],
    untouched: &[THE_ADRS_HOME],
    ..FINALIZE
}];
const THE_SECOND_OF_THREE_SUFFIXED_ONTO_A_FILE: [Case; 1] = [Case {
    name: "clobber-sub-task-second-of-three-suffixed-onto-a-file-both-later-docs-renamed",
    site: CLOBBER,
    cause: "three sub-tasks minted a doc of one id, and a file stands at the home of the \
            second's suffixed id: the route names a rename for the second and the third",
    state: &[
        Step::Jigc(&["setup"]),
        A_MILESTONE,
        A_FIRST_SUB_TASK,
        A_SECOND_SUB_TASK,
        A_THIRD_SUB_TASK,
        PROVISIONED,
        WORK_IN_THE_FIRST[0],
        WORK_IN_THE_FIRST[1],
        AN_ADR_PAYLOAD,
        the_adr_authored_by!("do-a-thing"),
        the_adr_authored_by!("second-thing"),
        the_adr_authored_by!("third-thing"),
        by_hand_at!("docs/decisions/use-tabs-2.md"),
    ],
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(1), Act::Run(2), Act::Run(3)],
    holds: &[
        Check::Contains(Place::Repo(THE_ADRS_HOME), "Because."),
        Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
        Check::Contains(Place::Repo("docs/decisions/title-2.md"), "Because."),
    ],
    untouched: &["docs/decisions/use-tabs-2.md"],
    ..FINALIZE
}];

const A_SUB_TASKS_DOC_OVER_A_FILE_GIT_HOLDS: [Case; 2] = [
    Case {
        name: "clobber-held-in-head-sub-task-doc-renamed-and-milestone-finalized-as-printed",
        site: CLOBBER,
        cause: "a committed file is missing from the main checkout's working tree, and a \
                sub-task minted a doc, under an id its author chooses, whose home is that \
                path",
        state: &[
            Step::Jigc(&["setup"]),
            by_hand_at!(THE_ADRS_HOME),
            staged_at!(THE_ADRS_HOME),
            COMMITTED,
            THE_ADRS_HOME_EMPTIED,
            A_MILESTONE,
            A_FIRST_SUB_TASK,
            PROVISIONED,
            WORK_IN_THE_FIRST[0],
            WORK_IN_THE_FIRST[1],
            AN_ADR_PAYLOAD,
            the_adr_authored_by!("do-a-thing"),
        ],
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[Act::Run(0), Act::Run(1)],
        holds: &[
            Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
            Check::RegularFile(Place::Repo("work.txt")),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-held-in-the-index-sub-task-doc-renamed-and-milestone-finalized-as-printed",
        site: CLOBBER,
        cause: "a file was staged in the main checkout and taken out of its working tree, \
                and a sub-task minted a doc, under an id its author chooses, whose home is \
                that path",
        state: &[
            Step::Jigc(&["setup"]),
            A_MILESTONE,
            A_FIRST_SUB_TASK,
            PROVISIONED,
            WORK_IN_THE_FIRST[0],
            WORK_IN_THE_FIRST[1],
            AN_ADR_PAYLOAD,
            the_adr_authored_by!("do-a-thing"),
            by_hand_at!(THE_ADRS_HOME),
            staged_at!(THE_ADRS_HOME),
            THE_ADRS_HOME_EMPTIED,
        ],
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[Act::Run(0), Act::Run(1)],
        holds: &[
            Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
            Check::RegularFile(Place::Repo("work.txt")),
        ],
        ..FINALIZE
    },
];

/// **An install, what the case plants before the milestone exists, a milestone of two
/// sub-tasks — the first with staged work, the second having minted the decisions log —
/// and what the case plants afterwards.**
macro_rules! a_sub_task_minted_the_log_after {
    ([$($before:expr),* $(,)?] $(, $plant:expr)* $(,)?) => {
        &[
            Step::Jigc(&["setup"]),
            $($before,)*
            A_MILESTONE,
            A_FIRST_SUB_TASK,
            A_SECOND_SUB_TASK_THAT_DECIDES,
            PROVISIONED,
            WORK_IN_THE_FIRST[0],
            WORK_IN_THE_FIRST[1],
            the_log_created_by!("second-thing"),
            $($plant,)*
        ]
    };
}
const A_SUB_TASKS_LOG_OVER_A_FILE_IN_HEAD: &str = "a committed file is missing from the main \
    checkout's working tree, and a sub-task minted the decisions log, whose one fixed home \
    is that path";
const A_SUB_TASKS_LOG_OVER_A_FILE_GIT_HOLDS: [Case; 3] = [
    Case {
        name: "clobber-held-in-head-fixed-only-sub-task-read-dropped-and-milestone-finalized-as-printed",
        site: CLOBBER,
        cause: A_SUB_TASKS_LOG_OVER_A_FILE_IN_HEAD,
        state: &[
            Step::Jigc(&["setup"]),
            by_hand_at!(LOG),
            staged_at!(LOG),
            COMMITTED,
            THE_LOGS_HOME_EMPTIED,
            A_MILESTONE,
            A_FIRST_SUB_TASK_THAT_DECIDES,
            PROVISIONED,
            the_log_created_by!("do-a-thing"),
        ],
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[Act::Run(2), Act::Run(1), Act::Run(3)],
        holds: &[Check::Absent(Place::Repo(".jigc/tasks/do-a-thing"))],
        ..FINALIZE
    },
    Case {
        name: "clobber-held-in-head-fixed-sub-task-read-dropped-and-milestone-finalized-as-printed",
        site: CLOBBER,
        cause: A_SUB_TASKS_LOG_OVER_A_FILE_IN_HEAD,
        state: a_sub_task_minted_the_log_after!([
            by_hand_at!(LOG),
            staged_at!(LOG),
            COMMITTED,
            THE_LOGS_HOME_EMPTIED,
        ]),
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[Act::Run(2), Act::Run(1), Act::Run(3)],
        holds: &[
            Check::RegularFile(Place::Repo("work.txt")),
            Check::Absent(Place::Repo(".jigc/tasks/second-thing")),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-held-in-the-index-fixed-sub-task-read-dropped-and-milestone-finalized-as-printed",
        site: CLOBBER,
        cause: "a file was staged in the main checkout and taken out of its working tree, \
                and a sub-task minted the decisions log, whose one fixed home is that path",
        state: a_sub_task_minted_the_log_after!(
            [],
            by_hand_at!(LOG),
            staged_at!(LOG),
            THE_LOGS_HOME_EMPTIED,
        ),
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[Act::Run(2), Act::Run(1), Act::Run(3)],
        holds: &[
            Check::RegularFile(Place::Repo("work.txt")),
            Check::Absent(Place::Repo(".jigc/tasks/second-thing")),
        ],
        ..FINALIZE
    },
];

const A_FILE_AT_THE_HOME_OF_A_SUB_TASKS_LOG: &str = "a sub-task minted the decisions log, \
    and a file in no commit stands at the log's one fixed home in the main checkout";
const A_SUB_TASKS_LOG_OVER_A_FILE: [Case; 2] = [
    Case {
        name: "clobber-fixed-sub-task-file-at-the-logs-home-moved-out-and-milestone-finalized",
        site: CLOBBER,
        cause: A_FILE_AT_THE_HOME_OF_A_SUB_TASKS_LOG,
        state: a_sub_task_minted_the_log_after!([], by_hand_at!(LOG)),
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[Act::MoveOut(Operand::Span(0)), Act::Run(3)],
        holds: &[
            Check::Contains(Place::Repo(LOG), "# Decisions Log"),
            Check::Contains(Place::Moved(LOG), "by hand"),
            Check::RegularFile(Place::Repo("work.txt")),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-fixed-sub-task-file-at-the-logs-home-sub-task-dropped-milestone-landed-file-adopted",
        site: CLOBBER,
        cause: A_FILE_AT_THE_HOME_OF_A_SUB_TASKS_LOG,
        state: a_sub_task_minted_the_log_after!([], by_hand_at!(LOG)),
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[
            Act::Run(6),
            Act::Run(5),
            Act::Run(7),
            Act::Run(8),
            Act::Run(9),
        ],
        holds: &[
            Check::RegularFile(Place::Repo("work.txt")),
            Check::Directory(Place::Repo(
                ".jigc/tasks/migrate-decisions-log-docs-decisions-log-b31ca3264f78",
            )),
        ],
        untouched: &[LOG],
        ..FINALIZE
    },
];
const A_SUB_TASKS_LOG_OVER_A_STAGED_FILE_IN_THE_MAIN_CHECKOUT: [Case; 1] = [Case {
    name: "clobber-fixed-sub-task-file-staged-at-the-logs-home-moved-with-git-mv-as-printed",
    site: CLOBBER,
    cause: "a sub-task minted the decisions log, and a file stands at the log's home in the \
            main checkout that was written and staged there after the milestone was \
            created — in git's index, in no commit",
    state: a_sub_task_minted_the_log_after!([], by_hand_at!(LOG), staged_at!(LOG)),
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(1), Act::Run(3)],
    holds: &[
        Check::Contains(Place::Repo(LOG), "# Decisions Log"),
        Check::RegularFile(Place::Repo("work.txt")),
    ],
    doubt: Some(WHAT_GIT_MV_IS),
    ..FINALIZE
}];
const A_SUB_TASKS_LOG_OVER_A_COMMITTED_FILE: [Case; 1] = [Case {
    name: "clobber-fixed-sub-task-committed-file-at-the-logs-home-sub-task-read-dropped-milestone-finalized",
    site: CLOBBER,
    cause: "a sub-task minted the decisions log while the committed file at its home was \
            missing from the main checkout's working tree, and the file is back",
    state: a_sub_task_minted_the_log_after!(
        [
            by_hand_at!(LOG),
            staged_at!(LOG),
            COMMITTED,
            THE_LOGS_HOME_EMPTIED,
        ],
        Step::Git(&["checkout", "HEAD", "--", LOG]),
    ),
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(3), Act::Run(2), Act::Run(4)],
    holds: &[
        Check::RegularFile(Place::Repo("work.txt")),
        Check::Absent(Place::Repo(".jigc/tasks/second-thing")),
    ],
    untouched: &[LOG],
    ..FINALIZE
}];

const TWO_SUB_TASKS_MINTED_THE_LOG: [Case; 1] = [Case {
    name: "clobber-contested-two-sub-tasks-minted-the-log-second-read-dropped-milestone-finalized",
    site: CLOBBER,
    cause: "two sub-tasks of one milestone each minted the decisions log: the join gives \
            the second a second id, and both promote to the log's one fixed home",
    state: &[
        Step::Jigc(&["setup"]),
        A_MILESTONE,
        A_FIRST_SUB_TASK_THAT_DECIDES,
        A_SECOND_SUB_TASK_THAT_DECIDES,
        PROVISIONED,
        WORK_IN_THE_FIRST[0],
        WORK_IN_THE_FIRST[1],
        the_log_created_by!("do-a-thing"),
        the_log_created_by!("second-thing"),
    ],
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(5), Act::Run(4), Act::Run(6)],
    holds: &[
        Check::Contains(Place::Repo(LOG), "# Decisions Log"),
        Check::RegularFile(Place::Repo("work.txt")),
        Check::Absent(Place::Repo(".jigc/tasks/second-thing")),
    ],
    ..FINALIZE
}];

// The staged arm: the occupant is a file a sub-task staged in its worktree.

const THE_ADRS_HOME_STAGED_IN_THE_FIRST: [Step; 2] =
    staged_in_the_worktree_of!("do-a-thing", "docs/decisions/use-tabs.md");
const THE_ADRS_HOME_STAGED_IN_THE_SECOND: [Step; 2] =
    staged_in_the_worktree_of!("second-thing", "docs/decisions/use-tabs.md");
const THE_LOGS_HOME_STAGED_IN_THE_FIRST: [Step; 2] =
    staged_in_the_worktree_of!("do-a-thing", "docs/decisions-log.md");

const A_STAGED_FILE_AT_THE_HOME_OF_A_SUB_TASKS_DOC: &str = "a sub-task minted a doc under an \
    id its author chooses, and one sub-task's worktree has a file staged at the doc's home";
const A_SUB_TASKS_DOC_OVER_A_STAGED_FILE: [Case; 2] = [
    Case {
        name: "clobber-staged-file-at-the-home-doc-renamed-and-milestone-finalized-as-printed",
        site: CLOBBER,
        cause: A_STAGED_FILE_AT_THE_HOME_OF_A_SUB_TASKS_DOC,
        state: &[
            Step::Jigc(&["setup"]),
            A_MILESTONE,
            A_FIRST_SUB_TASK,
            PROVISIONED,
            AN_ADR_PAYLOAD,
            the_adr_authored_by!("do-a-thing"),
            THE_ADRS_HOME_STAGED_IN_THE_FIRST[0],
            THE_ADRS_HOME_STAGED_IN_THE_FIRST[1],
        ],
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[Act::Run(0), Act::Run(3)],
        holds: &[
            Check::Contains(Place::Repo(THE_ADRS_HOME), "by hand"),
            Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
        ],
        ..FINALIZE
    },
    Case {
        name: "clobber-staged-file-at-the-home-stashed-and-milestone-finalized-as-printed",
        site: CLOBBER,
        cause: A_STAGED_FILE_AT_THE_HOME_OF_A_SUB_TASKS_DOC,
        state: &[
            Step::Jigc(&["setup"]),
            A_MILESTONE,
            A_FIRST_SUB_TASK,
            PROVISIONED,
            AN_ADR_PAYLOAD,
            the_adr_authored_by!("do-a-thing"),
            THE_ADRS_HOME_STAGED_IN_THE_FIRST[0],
            THE_ADRS_HOME_STAGED_IN_THE_FIRST[1],
        ],
        refusing: THE_MILESTONES_FINALIZE,
        acts: &[Act::Run(1), Act::Run(2), Act::Run(3)],
        holds: &[Check::Contains(Place::Repo(THE_ADRS_HOME), "Because.")],
        ..FINALIZE
    },
];

const TWO_STAGED_FILES_AT_THE_HOME_OF_A_SUB_TASKS_DOC: &str = "a sub-task minted a doc under \
    an id its author chooses, and two other sub-tasks' worktrees each have the same file \
    staged at the doc's home";
macro_rules! a_doc_over_two_staged_files {
    ($name:literal, acts: $acts:expr, holds: $holds:expr $(,)?) => {
        Case {
            name: $name,
            site: CLOBBER,
            cause: TWO_STAGED_FILES_AT_THE_HOME_OF_A_SUB_TASKS_DOC,
            state: &[
                Step::Jigc(&["setup"]),
                A_MILESTONE,
                A_FIRST_SUB_TASK,
                A_SECOND_SUB_TASK,
                A_THIRD_SUB_TASK,
                PROVISIONED,
                AN_ADR_PAYLOAD,
                the_adr_authored_by!("third-thing"),
                THE_ADRS_HOME_STAGED_IN_THE_FIRST[0],
                THE_ADRS_HOME_STAGED_IN_THE_FIRST[1],
                THE_ADRS_HOME_STAGED_IN_THE_SECOND[0],
                THE_ADRS_HOME_STAGED_IN_THE_SECOND[1],
            ],
            refusing: THE_MILESTONES_FINALIZE,
            acts: $acts,
            holds: $holds,
            ..FINALIZE
        }
    };
}
const A_SUB_TASKS_DOC_OVER_TWO_STAGED_FILES: [Case; 2] = [
    a_doc_over_two_staged_files!(
        "clobber-staged-twice-at-the-home-doc-renamed-and-milestone-finalized-as-printed",
        acts: &[Act::Run(0), Act::Run(4)],
        holds: &[
            Check::Contains(Place::Repo(THE_ADRS_HOME), "by hand"),
            Check::Contains(Place::Repo(THE_RENAMED_ADRS_HOME), "Because."),
        ],
    ),
    a_doc_over_two_staged_files!(
        "clobber-staged-twice-at-the-home-both-stashed-and-milestone-finalized-as-printed",
        acts: &[Act::Run(1), Act::Run(2), Act::Run(3), Act::Run(4)],
        holds: &[Check::Contains(Place::Repo(THE_ADRS_HOME), "Because.")],
    ),
];

const A_SUB_TASKS_EDIT_OVER_A_STAGED_FILE: [Case; 1] = [Case {
    name: "clobber-staged-change-at-the-home-of-an-edited-doc-stashed-and-milestone-finalized",
    site: CLOBBER,
    cause: "a sub-task edited a committed doc — copied in, not minted — and another \
            sub-task's worktree has a change staged at the doc's home",
    state: &[
        Step::Jigc(&["setup"]),
        AN_ADR_TASK,
        AN_ADR_PAYLOAD,
        the_adr_authored_by!("pick-tabs"),
        THE_ADRS_COMMIT_DOC[0],
        THE_ADRS_COMMIT_DOC[1],
        THE_ADRS_COMMIT_DOC[2],
        Step::Jigc(&["task", "finalize", "pick-tabs"]),
        A_MILESTONE,
        A_FIRST_SUB_TASK,
        A_SECOND_SUB_TASK,
        PROVISIONED,
        WORK_IN_THE_FIRST[0],
        WORK_IN_THE_FIRST[1],
        Step::File(Place::Outside("more"), "More.\n"),
        Step::Jigc(&[
            "doc",
            "set-slot",
            "adr:use-tabs#context",
            "--task",
            "do-a-thing",
            "--from-file",
            "../outside/more",
        ]),
        THE_ADRS_HOME_STAGED_IN_THE_SECOND[0],
        THE_ADRS_HOME_STAGED_IN_THE_SECOND[1],
    ],
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(0), Act::Run(1), Act::Run(2)],
    holds: &[
        Check::Contains(Place::Repo(THE_ADRS_HOME), "More."),
        Check::RegularFile(Place::Repo("work.txt")),
    ],
    ..FINALIZE
}];
const A_SUB_TASKS_LOG_OVER_A_STAGED_FILE: [Case; 1] = [Case {
    name: "clobber-staged-file-at-the-logs-home-stashed-and-milestone-finalized-as-printed",
    site: CLOBBER,
    cause: "a sub-task minted the decisions log, whose home is one fixed path, and its \
            worktree has a file staged at that path",
    state: &[
        Step::Jigc(&["setup"]),
        A_MILESTONE,
        A_FIRST_SUB_TASK_THAT_DECIDES,
        PROVISIONED,
        the_log_created_by!("do-a-thing"),
        THE_LOGS_HOME_STAGED_IN_THE_FIRST[0],
        THE_LOGS_HOME_STAGED_IN_THE_FIRST[1],
    ],
    refusing: THE_MILESTONES_FINALIZE,
    acts: &[Act::Run(0), Act::Run(1), Act::Run(2)],
    holds: &[Check::Contains(Place::Repo(LOG), "# Decisions Log")],
    ..FINALIZE
}];

// ---------------------------------------------------------------------------
// `empty_commit_finding` · `finalize.empty-commit` — a work unit that validated and has
// nothing to commit
// ---------------------------------------------------------------------------

/// **Nothing to commit, followed the one way the route prints a line for**: the work unit
/// abandoned. The other way — *make a change, then re-run* — names the reader's own work
/// and is no act of the frame's ([`READINGS`]).
const NOTHING_TO_COMMIT_IN_A_TASK: [Case; 1] = [Case {
    name: "finalize-empty-commit-nothing-staged-and-a-clean-tree-task-discarded-as-printed",
    site: EMPTY_COMMIT,
    cause: "an ordinary task's commit doc is complete, nothing is staged, no doc of the \
            task's promotes, and the working tree is clean",
    state: ready![],
    acts: &[Act::Run(1)],
    holds: &[Check::Absent(Place::Repo(TASK_AREA))],
    ..FINALIZE
}];
const A_DOC_WRITTEN_BACK_UNCHANGED: [Case; 1] = [Case {
    name: "finalize-empty-commit-a-committed-doc-written-back-unchanged-task-discarded-as-printed",
    site: EMPTY_COMMIT,
    cause: "an ordinary task copied a committed doc in and wrote it back with the bytes it \
            had: the doc is staged, its promotion changes no byte of `HEAD`, and nothing \
            else is staged",
    state: &[
        Step::Jigc(&["setup"]),
        AN_ADR_TASK,
        AN_ADR_PAYLOAD,
        the_adr_authored_by!("pick-tabs"),
        THE_ADRS_COMMIT_DOC[0],
        THE_ADRS_COMMIT_DOC[1],
        THE_ADRS_COMMIT_DOC[2],
        Step::Jigc(&["task", "finalize", "pick-tabs"]),
        Step::Jigc(&["start", "--workflow", "record-decision", "same again"]),
        Step::File(Place::Outside("same"), "Because.\n"),
        Step::Jigc(&[
            "doc",
            "set-slot",
            "adr:use-tabs#context",
            "--task",
            "same-again",
            "--from-file",
            "../outside/same",
        ]),
        THE_SAME_AGAIN_COMMIT_DOC[0],
        THE_SAME_AGAIN_COMMIT_DOC[1],
        THE_SAME_AGAIN_COMMIT_DOC[2],
    ],
    refusing: &["task", "finalize", "same-again"],
    acts: &[Act::Run(1)],
    holds: &[Check::Absent(Place::Repo(".jigc/tasks/same-again"))],
    untouched: &[THE_ADRS_HOME],
    ..FINALIZE
}];
const THE_SAME_AGAIN_COMMIT_DOC: [Step; 3] = commit_doc!("same-again");
const THE_NOTES_COMMIT_DOC: [Step; 3] = commit_doc!("note-a-thing");
const NOTHING_TO_COMMIT_IN_A_DOC_ONLY_TASK: [Case; 1] = [Case {
    name: "finalize-empty-commit-doc-only-task-with-no-doc-task-discarded-as-printed",
    site: EMPTY_COMMIT,
    cause: "a task whose finalize commits its docs and nothing else has minted none: its \
            path set is empty, whatever the working tree holds",
    state: &[
        Step::Jigc(&["setup"]),
        Step::Jigc(A_DOC_ONLY_MINT),
        THE_NOTES_COMMIT_DOC[0],
        THE_NOTES_COMMIT_DOC[1],
        THE_NOTES_COMMIT_DOC[2],
        Step::File(Place::Repo("unrelated.txt"), "unrelated\n"),
    ],
    refusing: &["task", "finalize", "note-a-thing"],
    acts: &[Act::Run(1)],
    holds: &[Check::Absent(Place::Repo(".jigc/tasks/note-a-thing"))],
    untouched: &["unrelated.txt"],
    ..FINALIZE
}];
/// A project that composes the dev pack alone resolves no `milestone-record` doctype, so a
/// milestone's finalize has no record to flip: with no sub-task's doc and no staged code
/// it is the planner's own empty commit. (With the record, the same milestone is refused
/// one step on, under `milestone.zero-contribution` — a site of the milestone's file.)
const NOTHING_TO_COMMIT_IN_A_MILESTONE: [Case; 1] = [Case {
    name: "finalize-empty-commit-milestone-without-a-record-doctype-milestone-discarded-as-printed",
    site: EMPTY_COMMIT,
    cause: "a milestone's finalize is asked in a project whose packs resolve no milestone \
            record — the dev pack alone — where no sub-task holds a doc and no worktree \
            holds staged code",
    state: &[
        Step::Jigc(&["setup"]),
        Step::File(
            Place::Repo(".jigc/config/packs.yaml"),
            "compose-embedded-methodology: false\n",
        ),
        Step::Git(&["add", ".jigc/config/packs.yaml"]),
        Step::Git(&["commit", "-q", "-m", "the dev pack alone"]),
        A_FAN_OUT_WITH_WORK[0],
        A_FAN_OUT_WITH_WORK[1],
        A_FAN_OUT_WITH_WORK[2],
    ],
    refusing: &["milestone", "finalize", "fan-out"],
    acts: &[Act::Run(1)],
    holds: &[
        Check::Absent(Place::Repo(".jigc/worktrees/do-a-thing")),
        Check::Absent(Place::Repo(".jigc/tasks/do-a-thing")),
    ],
    ..FINALIZE
}];

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
    &NOTHING_TO_COMMIT_IN_A_TASK,
    &A_DOC_WRITTEN_BACK_UNCHANGED,
    &NOTHING_TO_COMMIT_IN_A_DOC_ONLY_TASK,
    &NOTHING_TO_COMMIT_IN_A_MILESTONE,
    &A_TASKS_DOC_OVER_A_FILE,
    &A_MIGRATIONS_DOC_OVER_A_FILE,
    &A_FIXED_DOC_OVER_A_FILE,
    &A_FIXED_DOC_OVER_A_STAGED_FILE,
    &A_FIXED_DOC_OVER_A_COMMITTED_FILE,
    &A_TASKS_DOC_OVER_A_FILE_IN_HEAD,
    &A_TASKS_DOC_OVER_A_FILE_IN_THE_INDEX,
    &THE_LOG_MINTED_OVER_A_FILE_GIT_HOLDS,
    &A_MIGRATIONS_DOC_OVER_A_FILE_IN_HEAD,
    &A_MIGRATIONS_DOC_OVER_A_FILE_IN_THE_INDEX,
    &A_MIGRATED_LOG_OVER_A_FILE_GIT_HOLDS,
    &A_SUB_TASKS_DOC_OVER_A_FILE,
    &THE_SECOND_OF_TWO_SUFFIXED_ONTO_A_FILE,
    &THE_FIRST_OF_TWO_OVER_A_FILE,
    &THE_SECOND_OF_THREE_SUFFIXED_ONTO_A_FILE,
    &A_SUB_TASKS_DOC_OVER_A_FILE_GIT_HOLDS,
    &A_SUB_TASKS_LOG_OVER_A_FILE_GIT_HOLDS,
    &A_SUB_TASKS_LOG_OVER_A_FILE,
    &A_SUB_TASKS_LOG_OVER_A_STAGED_FILE_IN_THE_MAIN_CHECKOUT,
    &A_SUB_TASKS_LOG_OVER_A_COMMITTED_FILE,
    &TWO_SUB_TASKS_MINTED_THE_LOG,
    &A_SUB_TASKS_DOC_OVER_A_STAGED_FILE,
    &A_SUB_TASKS_DOC_OVER_TWO_STAGED_FILES,
    &A_SUB_TASKS_EDIT_OVER_A_STAGED_FILE,
    &A_SUB_TASKS_LOG_OVER_A_STAGED_FILE,
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
    Reading {
        site: CLOBBER,
        read: &[
            "crates/engine/src/finalize.rs::clobber_finding and ClobberedBy — seven arms, a message and a route each",
            "crates/engine/src/finalize.rs::plan_clobber_guard and plan_milestone_clobber_guard — the two doors, and which arm each hands a refused promotion to",
            "crates/engine/src/finalize.rs::refused_promotions, Occupant and Holder — what a promotion would land on: a file, what git holds where the disk shows nothing, a file a sub-task staged",
            "crates/engine/src/finalize.rs::split_contested and Contender — two promotions of one plan to one path",
            "crates/engine/src/finalize.rs::ShapeExit and ShapeUnit — the second and third axes: whether the doc's id is its author's to choose, and whose doc it is",
            "crates/engine/src/finalize.rs::sub_task_clobber_text and sub_task_renames, staged_clobber_text, contested_clobber_text, fixed_clobber_text, held_clobber_text — every clause each arm's route is put together from",
            "crates/engine/src/schema.rs::fixed_title, and the five `placement:` schemas of the shipped packs — which doctypes have one fixed home",
        ],
        causes: 32,
        derivation: "a cause is an arm by the axes its message and route are worded from. \
                     The task arm — an ordinary task's doc under an id its author chooses, \
                     over a file: 1. The migration arm, over a file that is not its source: \
                     1. The fixed-identity arm, over a file: at a task's door the file in no \
                     index, the file staged — the route has a clause for it — and the file \
                     `HEAD` holds too; the same three at a milestone's boundary — 6, one of \
                     them not followed apart from its sibling: the committed file is one \
                     case a door. The held arm, where the disk shows nothing: where git \
                     holds the file (`HEAD`, the index) by whose doc it is (a task's, a \
                     migration's, a sub-task's) by whether its id can change — 12. The \
                     sub-task arm, over a file: whether the join suffixed the doc's id by \
                     whether the route names one rename or one for each later sub-task of \
                     the group — 4. The staged arm, at the milestone's boundary only: a \
                     minted doc whose id can change over one staged file and over two, a doc \
                     copied in and edited, a minted doc under a fixed identity — 4. The \
                     contested arm: the door by whether a contender is under a fixed \
                     identity — 4. Two more arms of the text functions are reached by no \
                     state and are no cause: a held promotion of a body no unit staged — the \
                     held arm is asked only of a doc an origin says was minted — and a \
                     staged file at a task's door, which folds in no worktree",
        without_case: &[
            (
                "two docs of one task promote to one path, one of them under a fixed \
                 identity",
                Why::NotConstructible(
                    "a task's two docs share a home only where two doctypes do, and no two \
                     doctypes of the shipped packs share one: it takes a pack of the \
                     reader's own with two schemas on one `location:`, which no state step \
                     makes",
                ),
            ),
            (
                "two docs of one task promote to one path, both under ids their authors \
                 choose",
                Why::NotConstructible(
                    "as above: two doctypes on one `location:`, in a pack of the reader's own",
                ),
            ),
            (
                "two sub-tasks' docs promote to one path, both under ids their authors \
                 choose",
                Why::NotConstructible(
                    "the join's suffix gives two docs of one id two homes wherever the home \
                     carries the slug, and the five doctypes of the shipped packs whose \
                     home does not — `changelog`, `vision`, `roadmap`, `decisions-log`, \
                     `deferral-ledger` — are each a singleton, under a fixed identity. \
                     Again a pack of the reader's own",
                ),
            ),
        ],
        not_followed: &[
            (
                "*retitle this task's doc so it slugs differently* — the task arm's first way",
                Why::NotConstructible(
                    "the route names the act and prints no line for it; the product's line \
                     is `jigc doc rename`, which this arm does not print and every other \
                     arm does. Driven by hand: `jigc doc rename adr:use-tabs --to \"Use \
                     spaces\" --task pick-tabs`, then the finalize with its id — exit 0, \
                     one commit, the file at the old home left out",
                ),
            ),
            (
                "*re-author with a title that slugs differently* · *adopt it through its \
                 own `jigc migrate` task first* — the migration arm's first and third way",
                Why::NotConstructible(
                    "each names an act and prints no line for it — the one backticked span \
                     is `jigc migrate`, with no operand — and both end at the two finalize \
                     lines, whose `<id>` the refusal lists no value for: that tail is \
                     followed as printed by a case of its own",
                ),
            ),
            (
                "*and start the work again* · *commit the deletion of …* · *bring the file \
                 back* where no line is printed for it · *carry what you want kept into the \
                 doc that stays*",
                Why::NotConstructible(
                    "each names an act of the reader's own and prints no command for it: \
                     the cases end at the last printed line — the task or the sub-task \
                     dropped, the boundary run again — and hold what that leaves. Where the \
                     route prints the path of the deletion to commit, the case commits it \
                     by the runner's own git",
                ),
            ),
            (
                "the committed file at a fixed home, staged or not: one case a door",
                Why::NotConstructible(
                    "where `HEAD` holds the file the route offers one way, the drop, \
                     whatever the index holds — the clause about `git mv` is the \
                     uncommitted file's",
                ),
            ),
        ],
    },
    Reading {
        site: EMPTY_COMMIT,
        read: &[
            "crates/engine/src/finalize.rs::empty_commit_finding — two units, a message and a route each",
            "crates/engine/src/finalize.rs::plan_finalize and plan_milestone_finalize — the guard, behind the validation and the base pin, on a `has_diff` the door hands in",
            "crates/cli/src/task.rs::finalize — `has_diff`, arm by arm: an amend, a staged migration, a doc-only task, an ordinary one; and the arm that turns this refusal into `finalize.nothing-staged` where the tree is dirty",
            "crates/cli/src/task.rs::finalize against the previous release (`git show 91834b5e:crates/cli/src/task.rs`) — the lines of `has_diff` that changed: what counts as a pending `.jigc/config` path",
            "crates/cli/src/milestone.rs::run_milestone_finalize — the boundary's `has_diff`: a merged doc, staged code in a worktree, the record's flip; and the zero-contribution guard behind the planner",
        ],
        causes: 4,
        derivation: "the task's arm is reached where `has_diff` is false and the refusal is \
                     not recoloured. An amend's is always true; a staged migration's is true \
                     once a doc is staged, and with none staged another refusal answers \
                     first (`finalize.migration-no-replacement`, driven). An ordinary \
                     task's is false two ways — nothing staged and no doc of its own, on a \
                     clean tree (a dirty one is `finalize.nothing-staged`, a site with its \
                     own cases); and a staged doc whose promotion changes no byte of \
                     `HEAD`, beside nothing else. A doc-only task's is false where its path \
                     set is empty, whatever the tree holds: followed with no doc minted (a \
                     doc written back unchanged is the same term). The milestone's arm \
                     needs no merged doc, no staged code and no record flip, and with the \
                     shipped composition the record always flips — so it is reached only \
                     where the packs resolve no milestone record: 1. **The changed lines \
                     remove a cause and add none**: under the user's \
                     `status.showUntrackedFiles=no`, a task whose whole change is a new \
                     file under `.jigc/config` now has a diff and lands (driven: exit 0, \
                     the file committed); the previous release's binary refuses that state \
                     under `finalize.nothing-staged` (driven)",
        without_case: &[],
        not_followed: &[(
            "*make a change, then re-run `jigc task finalize <id>`* · *make a change in a \
             sub-task, then re-run `jigc milestone finalize <id>`*",
            Why::NotConstructible(
                "the change is the reader's own work, and the frame has no act that writes \
                 or stages a file of a case's own. Driven by hand at the task's door: a \
                 new file written and not staged, then the printed re-run — exit 3 under \
                 `finalize.nothing-staged`, whose route says to `git add` it",
            ),
        )],
    },
];
