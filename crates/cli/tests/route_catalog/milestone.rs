//! The route catalog's cases for the family the list of refusal sites calls `milestone`:
//! the refusals of the `jigc milestone` verbs. The family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here** is not the family: it is the eight sites of it that the fix pass
//! since the previous release touched, as the human ruled on 2026-10-11 (`DECISIONS.md` →
//! *Which families the route catalog takes in*) — the rest are rows of the scheduled list.
//! Three of the eight are one question asked at three doors: *what does this sub-task's
//! worktree path still hold?* — `jigc milestone provision` before it clears a path it will
//! not reuse, `jigc milestone discard` before it tears the workbench down, and `jigc
//! milestone finalize` before it lands without it. Then the id a record already owns, a
//! provision that stopped half-way, a boundary with nothing to land, two worktrees that
//! staged one path, and a file of the milestone's area that cannot be read as what it is.
//!
//! **Every case starts from an install and a milestone made by the binary under test** —
//! `jigc setup`, `jigc milestone create "Fan out"`, `add-task`, `provision` — and what a
//! sub-agent or a hand then left in a worktree is planted by file acts and by git, run in
//! that worktree.
//!
//! **A route here often sends its reader to the refusal's own listing** — *run the command
//! on that line* — where each held path has a line and the command that keeps or clears
//! what it holds. The case runs that command by its place among the listing's backticked
//! spans ([`Act::RunListed`]). **And where the line's command is a shell line** — the
//! recipe that brings a checkout back is three commands joined by `&&`, one of them a
//! redirect — no case runs it: the frame runs one command of git's or jigc's a line, and
//! the way has its reason in [`READINGS`].

use super::{Reading, Reason, Why};
use crate::support::route_case::{
    Act, Case, Check, Cited, Named, Operand, Place, Promise, Says, Step, Word,
};

// The sites, as the committed list of refusal sites names them.
const COLLISION: &str = "crates/cli/src/combine.rs::collision_finding::combine.code-collision#1";
const DIRTY: &str =
    "crates/cli/src/milestone.rs::dirty_worktree_finding::milestone.dirty-worktree#1";
const LEFTOVER: &str =
    "crates/cli/src/milestone.rs::leftover_finding::milestone.leftover-holds-work#1";
const PROVISION_FAILED: &str =
    "crates/cli/src/milestone.rs::provision_failed_finding::milestone.provision-failed#1";
const UNLANDED: &str =
    "crates/cli/src/milestone.rs::unlanded_work_finding::milestone.unlanded-work#1";
const ZERO: &str =
    "crates/cli/src/milestone.rs::zero_contribution_finding::milestone.zero-contribution#1";
const AREA_IO: &str = "crates/engine/src/milestone.rs::io_finding::milestone.area-io#1";
const RECORD_EXISTS: &str =
    "crates/engine/src/milestone.rs::record_home_refusal::milestone.record-exists#1";

/// The milestone's workbench, its committed record, and the two sub-tasks' worktrees and
/// working areas, from the repository root.
const AREA: &str = ".jigc/milestones/fan-out";
const RECORD: &str = "docs/milestone-records/fan-out.md";
const WT: &str = ".jigc/worktrees/do-a-thing";
const WT2: &str = ".jigc/worktrees/second-thing";

const SETUP: Step = Step::Jigc(&["setup"]);
const A_MILESTONE: Step = Step::Jigc(&["milestone", "create", "Fan out"]);
const FIRST: Step = Step::Jigc(&["milestone", "add-task", "fan-out", "do a thing"]);
const SECOND: Step = Step::Jigc(&["milestone", "add-task", "fan-out", "second thing"]);
const PROVISIONED: Step = Step::Jigc(&["milestone", "provision", "fan-out"]);

/// **An install, a milestone of one sub-task, its worktree provisioned — and what the case
/// plants.** [`fan2!`] is the same with a second sub-task; [`bare!`] the same with no
/// worktree provisioned.
macro_rules! fan {
    ($($plant:expr),* $(,)?) => {
        &[SETUP, A_MILESTONE, FIRST, PROVISIONED, $($plant,)*]
    };
}
macro_rules! fan2 {
    ($($plant:expr),* $(,)?) => {
        &[SETUP, A_MILESTONE, FIRST, SECOND, PROVISIONED, $($plant,)*]
    };
}
macro_rules! bare {
    ($($plant:expr),* $(,)?) => {
        &[SETUP, A_MILESTONE, FIRST, $($plant,)*]
    };
}

/// A sub-agent's work in the first worktree: a file written and staged there, and the same
/// committed there — a commit no ref reaches, a worktree's `HEAD` being detached.
const WORK: Step = Step::File(Place::Repo(".jigc/worktrees/do-a-thing/work.txt"), "work\n");
const STAGED: Step = Step::Git(&["-C", WT, "add", "work.txt"]);
const COMMITTED: Step = Step::Git(&["-C", WT, "commit", "-q", "-m", "the sub-task's work"]);
/// A file written in the first worktree and left alone.
const LOOSE: Step = Step::File(
    Place::Repo(".jigc/worktrees/do-a-thing/loose.txt"),
    "loose\n",
);
/// A shell line a `git` alias runs at the repository's root — a state has no step that
/// removes or moves.
macro_rules! sh {
    ($line:literal) => {
        Step::Git(&["-c", concat!("alias.plant=!", $line), "plant"])
    };
}
/// The first worktree's directory gone, git's registration of it left behind.
const THE_WORKTREE_GONE: Step = sh!("rm -rf .jigc/worktrees/do-a-thing");
/// The first worktree's directory moved out of the repository.
const THE_WORKTREE_MOVED_AWAY: Step = sh!("mv .jigc/worktrees/do-a-thing ../outside/moved-wt");
/// The index of git's registration of the first worktree, overwritten with bytes that are
/// no index.
const ITS_INDEX_GARBAGE: Step =
    Step::File(Place::Repo(".git/worktrees/do-a-thing/index"), "garbage\n");

/// A sub-task's commit doc, composed and authored through the binary under test: the
/// sub-task re-entered — from its own worktree, the one place the product re-enters it
/// from ([`Step::JigcAt`]) — then the two writes the doc requires.
macro_rules! sub_tasks_commit_doc {
    ($id:literal) => {
        [
            Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $id)),
                &["workflow", "sub-task", "--task", $id],
            ),
            Step::Jigc(&[
                "doc",
                "set-field",
                concat!("commit:", $id, "#header/type"),
                "--task",
                $id,
                "--value",
                "docs",
            ]),
            Step::File(Place::Outside("summary"), "do a thing\n"),
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
const THE_FIRSTS_COMMIT_DOC: [Step; 4] = sub_tasks_commit_doc!("do-a-thing");
const THE_SECONDS_COMMIT_DOC: [Step; 4] = sub_tasks_commit_doc!("second-thing");
/// The first sub-task with staged code and its commit doc authored — a milestone whose
/// join has a doc to merge.
macro_rules! with_a_doc {
    ($($plant:expr),* $(,)?) => {
        fan![
            WORK,
            STAGED,
            THE_FIRSTS_COMMIT_DOC[0],
            THE_FIRSTS_COMMIT_DOC[1],
            THE_FIRSTS_COMMIT_DOC[2],
            THE_FIRSTS_COMMIT_DOC[3],
            $($plant,)*
        ]
    };
}

/// What every case of the family shares: it stands at the repository's root.
const M: Case = Case {
    name: "",
    site: "",
    cause: "",
    state: &[],
    refusing: &["milestone", "finalize", "fan-out"],
    from: Place::Repo(""),
    env: &[],
    acts: &[],
    holds: &[],
    witnesses: &[],
    untouched: &[],
    doubt: None,
    over: None,
};
const PROVISION: &[&str] = &["milestone", "provision", "fan-out"];
const DISCARD: &[&str] = &["milestone", "discard", "fan-out"];
const FINALIZE: &[&str] = &["milestone", "finalize", "fan-out"];
const CREATE: &[&str] = &["milestone", "create", "Fan out"];

// ---------------------------------------------------------------------------
// `record_home_refusal` · `milestone.record-exists` — `jigc milestone create` over an id a
// record already owns
// ---------------------------------------------------------------------------

/// A live record: the milestone is continued by the printed line, run as printed — its
/// quoted `<intent>` is the sub-task's intent, and slugs to the id `intent`.
const A_LIVE_RECORD: &str = "a milestone of the same title was created before and is not \
    settled: its record stands at the record's home";
const THE_ID_HAS_A_LIVE_RECORD: [Case; 2] = [
    Case {
        name: "record-exists-live-continued-with-a-sub-task-as-printed",
        site: RECORD_EXISTS,
        cause: A_LIVE_RECORD,
        state: &[SETUP, A_MILESTONE],
        refusing: CREATE,
        acts: &[Act::Run(0)],
        holds: &[Check::Directory(Place::Repo(".jigc/tasks/intent"))],
        ..M
    },
    Case {
        name: "record-exists-live-no-workbench-continued-with-a-sub-task-as-printed",
        site: RECORD_EXISTS,
        cause: A_LIVE_RECORD,
        state: &[SETUP, A_MILESTONE, sh!("rm -rf .jigc/milestones/fan-out")],
        refusing: CREATE,
        acts: &[Act::Run(0)],
        holds: &[
            Check::Directory(Place::Repo(".jigc/tasks/intent")),
            Check::Directory(Place::Repo(AREA)),
        ],
        ..M
    },
];
const A_SETTLED_RECORD: &str = "a milestone of the same title is over: its record is settled";
const THE_ID_HAS_A_SETTLED_RECORD: [Case; 2] = [
    Case {
        name: "record-exists-settled-as-discarded-record-read-as-printed",
        site: RECORD_EXISTS,
        cause: A_SETTLED_RECORD,
        state: &[SETUP, A_MILESTONE, Step::Jigc(DISCARD)],
        refusing: CREATE,
        acts: &[Act::Run(1)],
        untouched: &[RECORD],
        ..M
    },
    Case {
        name: "record-exists-settled-as-joined-record-read-as-printed",
        site: RECORD_EXISTS,
        cause: A_SETTLED_RECORD,
        state: fan![WORK, STAGED, Step::Jigc(FINALIZE)],
        refusing: CREATE,
        acts: &[Act::Run(1)],
        untouched: &[RECORD],
        ..M
    },
];
const THE_HOME_HOLDS_A_FILE_THAT_IS_NO_RECORD: [Case; 1] = [Case {
    name: "record-exists-a-draft-by-hand-continued-with-a-sub-task-as-printed",
    site: RECORD_EXISTS,
    cause: "a file somebody drafted by hand stands at the record's home, in no commit, and \
            does not read as a record",
    state: &[SETUP, Step::File(Place::Repo(RECORD), "a draft\n")],
    refusing: CREATE,
    acts: &[Act::Run(0)],
    holds: &[Check::Directory(Place::Repo(".jigc/tasks/intent"))],
    untouched: &[RECORD],
    ..M
}];
const THE_RECORD_IS_IN_HEAD_AND_NOT_ON_DISK: [Case; 1] = [Case {
    name: "record-exists-held-in-head-brought-back-and-continued-as-printed",
    site: RECORD_EXISTS,
    cause: "the committed record is missing from the working tree — moved out, the deletion \
            not committed — and the id is minted again",
    state: &[
        SETUP,
        A_MILESTONE,
        sh!("mv docs/milestone-records/fan-out.md ../outside/record.md"),
    ],
    refusing: CREATE,
    acts: &[Act::Run(0), Act::Run(1)],
    holds: &[
        Check::RegularFile(Place::Repo(RECORD)),
        Check::Directory(Place::Repo(".jigc/tasks/intent")),
    ],
    ..M
}];
const THE_RECORD_IS_IN_THE_INDEX_AND_NOT_ON_DISK: [Case; 1] = [Case {
    name: "record-exists-held-in-the-index-brought-back-and-continued-as-printed",
    site: RECORD_EXISTS,
    cause: "a record is staged at the record's home, in no commit, and missing from the \
            working tree — and the id is minted again",
    state: &[
        SETUP,
        A_MILESTONE,
        Step::Git(&["reset", "-q", "--soft", "HEAD~1"]),
        sh!("mv docs/milestone-records/fan-out.md ../outside/record.md"),
    ],
    refusing: CREATE,
    acts: &[Act::Run(0), Act::Run(1)],
    holds: &[
        Check::RegularFile(Place::Repo(RECORD)),
        Check::Directory(Place::Repo(".jigc/tasks/intent")),
    ],
    ..M
}];

// ---------------------------------------------------------------------------
// `zero_contribution_finding` · `milestone.zero-contribution` — a boundary that would
// land no work
// ---------------------------------------------------------------------------

const NO_WORKTREE_WAS_EVER_PROVISIONED: [Case; 1] = [Case {
    name: "zero-contribution-never-provisioned-milestone-discarded-as-printed",
    site: ZERO,
    cause: "the milestone's sub-task has no worktree — none was provisioned, as in a fresh \
            clone — and no doc",
    state: bare![],
    acts: &[Act::Run(3)],
    holds: &[Check::Absent(Place::Repo(AREA))],
    ..M
}];
const NOTHING_IS_STAGED_IN_ANY_WORKTREE: [Case; 1] = [Case {
    name: "zero-contribution-provisioned-and-nothing-staged-milestone-discarded-as-printed",
    site: ZERO,
    cause: "the sub-task's worktree is provisioned and holds nothing: no staged code, no doc",
    state: fan![],
    acts: &[Act::Run(3)],
    holds: &[
        Check::Absent(Place::Repo(AREA)),
        Check::Absent(Place::Repo(WT)),
    ],
    ..M
}];
const WORK_STANDS_UNSTAGED: &str = "the sub-task's work is written in its worktree and not \
    staged, and no doc of a sub-task's promotes";
const THE_WORK_IS_NOT_STAGED: [Case; 2] = [
    Case {
        name: "zero-contribution-work-unstaged-provisioned-added-and-finalized-as-printed",
        site: ZERO,
        cause: WORK_STANDS_UNSTAGED,
        state: fan![WORK],
        // The provision, as printed; `git add`, which names a command, on the work and
        // where the route says it belongs; the finalize, as printed.
        acts: &[
            Act::Run(0),
            Act::RunNamed(Named {
                name: 1,
                on: &[Word::Named("work.txt")],
                within: Some(Word::Named(WT)),
                means: "*their work inside their worktrees*: the file the sub-task wrote, \
                        in the sub-task's worktree — the directory the provision gives it, \
                        `.jigc/worktrees/<the sub-task's id>`; the refusal lists the \
                        milestone and neither, so the case names both",
            }),
            Act::Run(2),
        ],
        holds: &[Check::RegularFile(Place::Repo("work.txt"))],
        ..M
    },
    Case {
        name: "zero-contribution-work-unstaged-milestone-discarded-as-printed",
        site: ZERO,
        cause: WORK_STANDS_UNSTAGED,
        state: fan![WORK],
        acts: &[Act::Run(3)],
        holds: &[Check::Absent(Place::Repo(AREA))],
        ..M
    },
];
const ONLY_A_TRANSIENT_DOC: [Case; 1] = [Case {
    name: "zero-contribution-only-a-commit-doc-milestone-discarded-as-printed",
    site: ZERO,
    cause: "the one thing a sub-task holds is its commit doc, which is merged and promotes \
            nowhere: no doc lands in the store and no worktree holds staged code",
    state: fan![
        THE_FIRSTS_COMMIT_DOC[0],
        THE_FIRSTS_COMMIT_DOC[1],
        THE_FIRSTS_COMMIT_DOC[2],
        THE_FIRSTS_COMMIT_DOC[3],
    ],
    acts: &[Act::Run(3)],
    holds: &[Check::Absent(Place::Repo(AREA))],
    ..M
}];
/// A project that composes the dev pack alone — no milestone record to flip — whose
/// sub-task authored its commit doc and staged no code: the transient doc is merged and
/// promotes nowhere.
const ONLY_A_TRANSIENT_DOC_WHERE_NO_RECORD_FLIPS: [Case; 1] = [Case {
    name: "zero-contribution-no-record-doctype-only-a-commit-doc-milestone-discarded-as-printed",
    site: ZERO,
    cause: "the packs resolve no milestone record, and the one thing a sub-task holds is its \
            commit doc, which promotes nowhere",
    state: &[
        SETUP,
        Step::File(
            Place::Repo(".jigc/config/packs.yaml"),
            "compose-embedded-methodology: false\n",
        ),
        Step::Git(&["add", ".jigc/config/packs.yaml"]),
        Step::Git(&["commit", "-q", "-m", "the dev pack alone"]),
        A_MILESTONE,
        FIRST,
        PROVISIONED,
        Step::Jigc(&["workflow", "sub-task", "--task", "do-a-thing"]),
        Step::Jigc(&[
            "doc",
            "set-field",
            "commit:do-a-thing#header/type",
            "--task",
            "do-a-thing",
            "--value",
            "docs",
        ]),
    ],
    acts: &[Act::Run(3)],
    holds: &[Check::Absent(Place::Repo(AREA))],
    ..M
}];

// ---------------------------------------------------------------------------
// `collision_finding` · `combine.code-collision` — two worktrees staged one path
// ---------------------------------------------------------------------------

/// The same path staged in both sub-tasks' worktrees, with bytes of each one's own.
const ONE_PATH_STAGED_TWICE: [Step; 4] = [
    Step::File(
        Place::Repo(".jigc/worktrees/do-a-thing/shared.txt"),
        "one\n",
    ),
    Step::Git(&["-C", WT, "add", "shared.txt"]),
    Step::File(
        Place::Repo(".jigc/worktrees/second-thing/shared.txt"),
        "two\n",
    ),
    Step::Git(&["-C", WT2, "add", "shared.txt"]),
];

/// **Two worktrees staged one path, followed the one way the frame has an act for**: one
/// sub-task lets the path go — its index entry dropped, the file left where it is — and
/// the refused command is run again, the route printing no line for it.
macro_rules! collision {
    ($name:literal, $cause:literal, state: $state:expr, refusing: $refusing:expr $(, holds: $holds:expr)? $(,)?) => {
        [Case {
            name: concat!("collision-", $name, "-one-sub-task-lets-the-path-go"),
            site: COLLISION,
            cause: $cause,
            state: $state,
            refusing: $refusing,
            acts: &[
                Act::Unstage(Operand::Unprinted(".jigc/worktrees/second-thing/shared.txt")),
                Act::Rerun,
            ],
            $(holds: $holds,)?
            ..M
        }]
    };
}
const A_COLLISION_AT_THE_JOIN: [Case; 1] = collision!(
    "at-the-join",
    "two sub-tasks' worktrees staged one path, and the join's preview is asked",
    state: fan2![
        ONE_PATH_STAGED_TWICE[0],
        ONE_PATH_STAGED_TWICE[1],
        ONE_PATH_STAGED_TWICE[2],
        ONE_PATH_STAGED_TWICE[3],
    ],
    refusing: &["milestone", "join", "fan-out"],
);
const A_COLLISION_AT_THE_BOUNDARY: [Case; 1] = collision!(
    "at-the-finalize",
    "two sub-tasks' worktrees staged one path, and the milestone's finalize is asked — one \
     combined commit",
    state: fan2![
        ONE_PATH_STAGED_TWICE[0],
        ONE_PATH_STAGED_TWICE[1],
        ONE_PATH_STAGED_TWICE[2],
        ONE_PATH_STAGED_TWICE[3],
    ],
    refusing: FINALIZE,
    holds: &[Check::Contains(Place::Repo("shared.txt"), "one")],
);
const A_COLLISION_AT_THE_UNSQUASHED_BOUNDARY: [Case; 1] = collision!(
    "at-the-finalize-a-commit-a-sub-task",
    "two sub-tasks' worktrees staged one path, and the milestone's finalize is asked where \
     the project sets a commit a sub-task (`finalize.fan-out.squash: false`)",
    state: &[
        SETUP,
        Step::Jigc(&["config", "set", "finalize.fan-out.squash", "false"]),
        Step::Git(&["add", ".jigc/config"]),
        Step::Git(&["commit", "-q", "-m", "a commit a sub-task"]),
        A_MILESTONE,
        FIRST,
        SECOND,
        PROVISIONED,
        ONE_PATH_STAGED_TWICE[0],
        ONE_PATH_STAGED_TWICE[1],
        ONE_PATH_STAGED_TWICE[2],
        ONE_PATH_STAGED_TWICE[3],
        THE_FIRSTS_COMMIT_DOC[0],
        THE_FIRSTS_COMMIT_DOC[1],
        THE_FIRSTS_COMMIT_DOC[2],
        THE_FIRSTS_COMMIT_DOC[3],
        THE_SECONDS_COMMIT_DOC[0],
        THE_SECONDS_COMMIT_DOC[1],
        THE_SECONDS_COMMIT_DOC[2],
        THE_SECONDS_COMMIT_DOC[3],
    ],
    refusing: FINALIZE,
    holds: &[Check::Contains(Place::Repo("shared.txt"), "one")],
);

// ---------------------------------------------------------------------------
// `provision_failed_finding` · `milestone.provision-failed` — the provisioning stopped
// behind its probe
// ---------------------------------------------------------------------------

const GITS_WORKTREES_DIRECTORY_IS_A_FILE: [Case; 1] = [Case {
    name: "provision-failed-gits-worktrees-directory-a-file-moved-out-and-provisioned-as-printed",
    site: PROVISION_FAILED,
    cause: "a regular file stands at `.git/worktrees`, where git keeps its registrations: \
            `git worktree add` cannot make the first one",
    state: &[
        SETUP,
        A_MILESTONE,
        FIRST,
        SECOND,
        Step::File(Place::Repo(".git/worktrees"), "x\n"),
    ],
    refusing: PROVISION,
    acts: &[
        Act::MoveOut(Operand::Unprinted(".git/worktrees")),
        Act::Run(0),
    ],
    holds: &[
        Check::Directory(Place::Repo(WT)),
        Check::Directory(Place::Repo(WT2)),
    ],
    ..M
}];
const A_LINK_IN_THE_WAY_UNDER_THE_CONSENT: [Case; 1] = [Case {
    name: "provision-failed-forced-over-a-link-to-nothing-link-removed-and-provisioned-as-printed",
    site: PROVISION_FAILED,
    cause: "the consent is given over a link that leads nowhere at the second sub-task's \
            path: the clearing step sees nothing there, and `git worktree add` refuses a \
            path that exists",
    state: &[
        SETUP,
        A_MILESTONE,
        FIRST,
        SECOND,
        Step::LinkTo(Place::Repo(WT2), "nowhere"),
    ],
    refusing: &["milestone", "provision", "fan-out", "--force"],
    acts: &[Act::Remove(Operand::Unprinted(WT2)), Act::Run(0)],
    holds: &[
        Check::Directory(Place::Repo(WT)),
        Check::Directory(Place::Repo(WT2)),
    ],
    ..M
}];
const A_STALE_REGISTRATION_GIT_WILL_NOT_DROP: [Case; 1] = [Case {
    name: "provision-failed-forced-over-a-link-at-a-stale-registrations-path-link-removed-and-provisioned",
    site: PROVISION_FAILED,
    cause: "the consent is given where a sub-task's worktree directory is gone and a link \
            that leads nowhere stands at its path: git will not drop the stale registration \
            over what it cannot read as a worktree",
    state: fan![THE_WORKTREE_GONE, Step::LinkTo(Place::Repo(WT), "nowhere")],
    refusing: &["milestone", "provision", "fan-out", "--force"],
    acts: &[Act::Remove(Operand::Unprinted(WT)), Act::Run(0)],
    holds: &[Check::Directory(Place::Repo(WT))],
    ..M
}];
const A_HOOK_OF_GITS_FAILS_BEHIND_THE_CHECKOUT: [Case; 2] = [
    Case {
        name: "provision-failed-post-checkout-hook-fails-hook-removed-and-provisioned-as-printed",
        site: PROVISION_FAILED,
        cause: "a `post-checkout` hook of the repository's exits 1: `git worktree add` has \
                made the worktree and reports failure",
        state: &[
            SETUP,
            A_MILESTONE,
            FIRST,
            Step::Script(
                Place::Repo(".git/hooks/post-checkout"),
                "#!/bin/sh\nexit 1\n",
            ),
        ],
        refusing: PROVISION,
        acts: &[
            Act::Remove(Operand::Unprinted(".git/hooks/post-checkout")),
            Act::Run(0),
        ],
        holds: &[Check::Directory(Place::Repo(WT))],
        ..M
    },
    Case {
        name: "provision-failed-post-checkout-hook-fails-provisioned-again-as-printed",
        site: PROVISION_FAILED,
        cause: "a `post-checkout` hook of the repository's exits 1: `git worktree add` has \
                made the worktree and reports failure",
        state: &[
            SETUP,
            A_MILESTONE,
            FIRST,
            Step::Script(
                Place::Repo(".git/hooks/post-checkout"),
                "#!/bin/sh\nexit 1\n",
            ),
        ],
        refusing: PROVISION,
        acts: &[Act::Run(0)],
        holds: &[Check::Directory(Place::Repo(WT))],
        ..M
    },
];

// ---------------------------------------------------------------------------
// `io_finding` · `milestone.area-io` — a file of the milestone's area that cannot be read,
// or made, as what it is
// ---------------------------------------------------------------------------

/// **A `jigc milestone` verb cannot read or make an entry of the milestone's area**, over a
/// state that is no permission. The route names `.jigc/` and *a disk or permissions
/// problem*: the case makes `.jigc/` writable, which it was, and runs the command again.
macro_rules! area_io {
    ($name:literal, $cause:literal, state: $state:expr, refusing: $refusing:expr $(,)?) => {
        [Case {
            name: concat!("area-io-", $name, "-workbench-made-writable"),
            site: AREA_IO,
            cause: $cause,
            state: $state,
            refusing: $refusing,
            acts: &[Act::MakeWritable(Operand::Span(0)), Act::Rerun],
            ..M
        }]
    };
}
const ADD_A_TASK: &[&str] = &["milestone", "add-task", "fan-out", "second thing"];
const JOIN: &[&str] = &["milestone", "join", "fan-out"];
const THE_MILESTONES_DIRECTORY_A_FILE: [Case; 1] = area_io!(
    "milestones-directory-a-file-at-the-create",
    "a regular file stands at `.jigc/milestones`, and a milestone is created: its area \
     cannot be opened",
    state: &[SETUP, Step::File(Place::Repo(".jigc/milestones"), "x\n")],
    refusing: CREATE,
);
const A_LINK_TO_NOTHING_AT_THE_AREA: [Case; 1] = area_io!(
    "a-link-to-nothing-at-the-area-at-the-create",
    "a link that leads nowhere stands where the milestone's area is to be made",
    state: &[SETUP, Step::LinkTo(Place::Repo(AREA), "nowhere")],
    refusing: CREATE,
);
const THE_BASE_PIN_NOT_JSON: [Case; 1] = area_io!(
    "base-pin-not-json-at-add-task",
    "the area's `base.json` holds text that is no JSON, and a sub-task is added: the shared \
     base pin cannot be read",
    state: bare![Step::File(Place::Repo(".jigc/milestones/fan-out/base.json"), "garbage\n")],
    refusing: ADD_A_TASK,
);
const THE_TASK_LIST_NOT_JSON: [Case; 3] = [
    area_io!(
        "task-list-not-json-at-add-task",
        "the area's `tasks.json` holds text that is no JSON, and a sub-task is added: the \
         task list cannot be read",
        state: bare![Step::File(Place::Repo(".jigc/milestones/fan-out/tasks.json"), "garbage\n")],
        refusing: ADD_A_TASK,
    )[0],
    area_io!(
        "task-list-not-json-at-the-join",
        "the area's `tasks.json` holds text that is no JSON, and the join's preview is asked",
        state: bare![Step::File(Place::Repo(".jigc/milestones/fan-out/tasks.json"), "garbage\n")],
        refusing: JOIN,
    )[0],
    area_io!(
        "task-list-not-json-at-the-finalize",
        "the area's `tasks.json` holds text that is no JSON, and the milestone's finalize is \
         asked",
        state: bare![Step::File(Place::Repo(".jigc/milestones/fan-out/tasks.json"), "garbage\n")],
        refusing: FINALIZE,
    )[0],
];
const THE_TASK_LIST_A_DIRECTORY: [Case; 1] = area_io!(
    "task-list-a-directory-at-add-task",
    "a directory stands where the area's `tasks.json` was, and a sub-task is added",
    state: bare![sh!(
        "rm .jigc/milestones/fan-out/tasks.json && mkdir .jigc/milestones/fan-out/tasks.json"
    )],
    refusing: ADD_A_TASK,
);
const NO_AREA_AND_THE_MILESTONES_DIRECTORY_A_FILE: [Case; 1] = area_io!(
    "no-area-and-milestones-directory-a-file-at-add-task",
    "the record is committed and the area is gone — a fresh clone — and a regular file \
     stands at `.jigc/milestones`: the cache area cannot be opened",
    state: bare![sh!("rm -rf .jigc/milestones && printf x > .jigc/milestones")],
    refusing: ADD_A_TASK,
);
const THE_BASE_PIN_A_DIRECTORY_AT_THE_RESEED: [Case; 1] = area_io!(
    "base-pin-a-directory-and-no-task-list-at-add-task",
    "the area has no task list and a directory where its `base.json` was: the base pin \
     cache cannot be re-seeded from the record",
    state: bare![sh!(
        "cd .jigc/milestones/fan-out && rm base.json tasks.json && mkdir base.json"
    )],
    refusing: ADD_A_TASK,
);
const THE_TASK_LIST_A_DIRECTORY_AT_THE_RESEED: [Case; 1] = area_io!(
    "task-list-a-directory-and-no-base-pin-at-add-task",
    "the area has no base pin and a directory where its `tasks.json` was: the task list \
     cache cannot be re-seeded from the record",
    state: bare![sh!(
        "cd .jigc/milestones/fan-out && rm base.json tasks.json && mkdir tasks.json"
    )],
    refusing: ADD_A_TASK,
);

const A_LINK_TO_NOTHING_AT_THE_STAGING_AREA: [Case; 1] = area_io!(
    "a-link-to-nothing-at-the-staging-area-at-the-finalize",
    "a link that leads nowhere stands where the join's staging area is made, and the \
     milestone's finalize is asked",
    state: with_a_doc![Step::LinkTo(
        Place::Repo(".jigc/milestones/fan-out/merged"),
        "nowhere"
    )],
    refusing: FINALIZE,
);
const A_FILE_AT_THE_STAGING_AREA: [Case; 2] = [
    area_io!(
        "a-file-at-the-staging-area-at-the-finalize",
        "a regular file stands where the join's staging area is made",
        state: with_a_doc![Step::File(Place::Repo(".jigc/milestones/fan-out/merged"), "x\n")],
        refusing: FINALIZE,
    )[0],
    area_io!(
        "a-file-at-the-staging-areas-docs-at-the-finalize",
        "a regular file stands where the join's staging area keeps its docs",
        state: with_a_doc![Step::File(
            Place::Repo(".jigc/milestones/fan-out/merged/docs"),
            "x\n"
        )],
        refusing: FINALIZE,
    )[0],
];
const A_DIRECTORY_AT_A_MERGED_BODYS_PATH: [Case; 1] = area_io!(
    "a-directory-at-a-merged-bodys-path-at-the-finalize",
    "a directory stands at the path the join writes a merged doc's body to",
    state: with_a_doc![Step::Dir(Place::Repo(
        ".jigc/milestones/fan-out/merged/docs/commit:do-a-thing.md"
    ))],
    refusing: FINALIZE,
);
const A_SUB_TASKS_MANIFEST_NOT_JSON: [Case; 2] = [
    area_io!(
        "sub-task-manifest-not-json-at-the-join",
        "a sub-task's `docs/provenance.json` holds text that is no JSON, and the join's \
         preview is asked",
        state: with_a_doc![Step::File(
            Place::Repo(".jigc/tasks/do-a-thing/docs/provenance.json"),
            "garbage\n"
        )],
        refusing: JOIN,
    )[0],
    area_io!(
        "sub-task-manifest-not-json-at-the-finalize",
        "a sub-task's `docs/provenance.json` holds text that is no JSON, and the \
         milestone's finalize is asked",
        state: with_a_doc![Step::File(
            Place::Repo(".jigc/tasks/do-a-thing/docs/provenance.json"),
            "garbage\n"
        )],
        refusing: FINALIZE,
    )[0],
];
const A_SUB_TASKS_MANIFEST_A_DIRECTORY: [Case; 1] = area_io!(
    "sub-task-manifest-a-directory-at-the-join",
    "a directory stands where a sub-task's `docs/provenance.json` was",
    state: with_a_doc![sh!(
        "cd .jigc/tasks/do-a-thing/docs && rm provenance.json && mkdir provenance.json"
    )],
    refusing: JOIN,
);
const A_STAGED_BODY_A_DIRECTORY: [Case; 1] = area_io!(
    "staged-body-a-directory-at-the-finalize",
    "a directory stands where a sub-task's staged doc body was",
    state: with_a_doc![sh!(
        "cd .jigc/tasks/do-a-thing/docs && rm commit:do-a-thing.md && mkdir commit:do-a-thing.md"
    )],
    refusing: FINALIZE,
);
const A_STAGED_BODY_NOT_TEXT: [Case; 1] = area_io!(
    "staged-body-not-utf-8-at-the-finalize",
    "a sub-task's staged doc body holds bytes that are no UTF-8",
    state: with_a_doc![sh!(
        "printf '\\377\\376\\n' > .jigc/tasks/do-a-thing/docs/commit:do-a-thing.md"
    )],
    refusing: FINALIZE,
);
// ---------------------------------------------------------------------------
// The three doors that ask what a sub-task's worktree path still holds
// ---------------------------------------------------------------------------

// `dirty_worktree_finding` · `milestone.dirty-worktree` — `jigc milestone discard`

const UNCOMMITTED_WORK_IN_A_REGISTERED_WORKTREE: &str = "a sub-task's provisioned worktree \
    holds a file git does not track, and the milestone is abandoned";
const DISCARD_OVER_LOOSE_WORK: [Case; 2] = [
    Case {
        name: "dirty-worktree-untracked-file-moved-out-and-discarded-as-printed",
        site: DIRTY,
        cause: UNCOMMITTED_WORK_IN_A_REGISTERED_WORKTREE,
        state: fan![LOOSE],
        refusing: DISCARD,
        acts: &[
            Act::MoveOut(Operand::Unprinted(".jigc/worktrees/do-a-thing/loose.txt")),
            Act::Run(0),
        ],
        holds: &[
            Check::Absent(Place::Repo(AREA)),
            Check::Absent(Place::Repo(WT)),
            Check::RegularFile(Place::Moved(".jigc/worktrees/do-a-thing/loose.txt")),
        ],
        ..M
    },
    Case {
        name: "dirty-worktree-untracked-file-discarded-by-the-consent-as-printed",
        site: DIRTY,
        cause: UNCOMMITTED_WORK_IN_A_REGISTERED_WORKTREE,
        state: fan![LOOSE],
        refusing: DISCARD,
        acts: &[Act::Run(1)],
        holds: &[
            Check::Absent(Place::Repo(AREA)),
            Check::Absent(Place::Repo(WT)),
        ],
        ..M
    },
];
const A_COMMIT_IN_A_REGISTERED_WORKTREE: &str = "a sub-task's provisioned worktree holds a \
    commit no ref reaches — the sub-agent committed there — and the milestone is abandoned";
const DISCARD_OVER_A_COMMIT: [Case; 2] = [
    Case {
        name: "dirty-worktree-a-commit-kept-by-the-listed-command-and-discarded-as-printed",
        site: DIRTY,
        cause: A_COMMIT_IN_A_REGISTERED_WORKTREE,
        state: fan![WORK, STAGED, COMMITTED],
        refusing: DISCARD,
        acts: &[Act::RunListed(0), Act::Run(0)],
        holds: &[
            Check::Absent(Place::Repo(AREA)),
            Check::Absent(Place::Repo(WT)),
        ],
        ..M
    },
    Case {
        name: "dirty-worktree-a-commit-discarded-by-the-consent-as-printed",
        site: DIRTY,
        cause: A_COMMIT_IN_A_REGISTERED_WORKTREE,
        state: fan![WORK, STAGED, COMMITTED],
        refusing: DISCARD,
        acts: &[Act::Run(1)],
        holds: &[
            Check::Absent(Place::Repo(AREA)),
            Check::Absent(Place::Repo(WT)),
        ],
        ..M
    },
];
const DISCARD_OVER_AN_OPERATION: [Case; 1] = [Case {
    name: "dirty-worktree-a-bisect-abandoned-by-the-listed-command-and-discarded-as-printed",
    site: DIRTY,
    cause: "a sub-task's provisioned worktree is one git has left mid-operation — a bisect \
            started and not concluded — and the milestone is abandoned",
    state: fan![Step::Git(&["-C", WT, "bisect", "start"])],
    refusing: DISCARD,
    acts: &[Act::RunListed(0), Act::Run(0)],
    holds: &[
        Check::Absent(Place::Repo(AREA)),
        Check::Absent(Place::Repo(WT)),
    ],
    ..M
}];
const DISCARD_OVER_A_REGISTRATION_WITH_STAGED_PATHS: [Case; 1] = [Case {
    name: "dirty-worktree-no-directory-and-staged-paths-in-the-registration-discarded-by-the-consent",
    site: DIRTY,
    cause: "a sub-task's worktree directory is gone and git's registration of it still \
            holds a staged path, and the milestone is abandoned",
    state: fan![WORK, STAGED, THE_WORKTREE_GONE],
    refusing: DISCARD,
    acts: &[Act::Run(1)],
    holds: &[Check::Absent(Place::Repo(AREA))],
    ..M
}];
const DISCARD_OVER_A_REGISTRATION_WITH_A_COMMIT: [Case; 1] = [Case {
    name: "dirty-worktree-no-directory-and-a-commit-in-the-registration-kept-and-discarded",
    site: DIRTY,
    cause: "a sub-task's worktree directory is gone and git's registration of it still \
            holds a commit no ref reaches, and the milestone is abandoned",
    state: fan![WORK, STAGED, COMMITTED, THE_WORKTREE_GONE],
    refusing: DISCARD,
    acts: &[Act::RunListed(0), Act::Run(0)],
    holds: &[Check::Absent(Place::Repo(AREA))],
    ..M
}];
const A_DIRECTORY_NOBODY_REGISTERED: &str = "a directory that is no worktree stands at a \
    sub-task's worktree path and holds a file, and the milestone is abandoned";
const DISCARD_OVER_AN_UNREGISTERED_DIRECTORY: [Case; 2] = [
    Case {
        name: "dirty-worktree-unregistered-directory-moved-away-and-discarded-as-printed",
        site: DIRTY,
        cause: A_DIRECTORY_NOBODY_REGISTERED,
        state: bare![Step::File(
            Place::Repo(".jigc/worktrees/do-a-thing/junk.txt"),
            "junk\n"
        )],
        refusing: DISCARD,
        acts: &[Act::MoveAway(Operand::Unprinted(WT)), Act::Run(0)],
        holds: &[
            Check::Absent(Place::Repo(AREA)),
            Check::RegularFile(Place::Moved(".jigc/worktrees/do-a-thing/junk.txt")),
        ],
        ..M
    },
    Case {
        name: "dirty-worktree-unregistered-directory-discarded-by-the-consent-as-printed",
        site: DIRTY,
        cause: A_DIRECTORY_NOBODY_REGISTERED,
        state: bare![Step::File(
            Place::Repo(".jigc/worktrees/do-a-thing/junk.txt"),
            "junk\n"
        )],
        refusing: DISCARD,
        acts: &[Act::Run(1)],
        holds: &[
            Check::Absent(Place::Repo(AREA)),
            Check::RegularFile(Place::Repo(".jigc/worktrees/do-a-thing/junk.txt")),
        ],
        ..M
    },
];
const A_FILE_AT_THE_PATH: &str = "a regular file stands at a sub-task's worktree path, and \
    the milestone is abandoned";
const DISCARD_OVER_A_FILE: [Case; 2] = [
    Case {
        name: "dirty-worktree-a-file-at-the-path-moved-out-and-discarded-as-printed",
        site: DIRTY,
        cause: A_FILE_AT_THE_PATH,
        state: bare![Step::File(Place::Repo(WT), "x\n")],
        refusing: DISCARD,
        acts: &[Act::MoveOut(Operand::Unprinted(WT)), Act::Run(0)],
        holds: &[
            Check::Absent(Place::Repo(AREA)),
            Check::RegularFile(Place::Moved(WT)),
        ],
        ..M
    },
    Case {
        name: "dirty-worktree-a-file-at-the-path-discarded-by-the-consent-as-printed",
        site: DIRTY,
        cause: A_FILE_AT_THE_PATH,
        state: bare![Step::File(Place::Repo(WT), "x\n")],
        refusing: DISCARD,
        acts: &[Act::Run(1)],
        holds: &[
            Check::Absent(Place::Repo(AREA)),
            Check::RegularFile(Place::Repo(WT)),
        ],
        ..M
    },
];
const DISCARD_OVER_AN_UNREADABLE_REGISTRATION: [Case; 1] = [Case {
    name: "dirty-worktree-no-directory-and-a-registration-git-cannot-read-discarded-by-the-consent",
    site: DIRTY,
    cause: "a sub-task's worktree directory is gone and the index of git's registration of \
            it is no index: nothing can be read there, and the milestone is abandoned",
    state: fan![WORK, STAGED, THE_WORKTREE_GONE, ITS_INDEX_GARBAGE],
    refusing: DISCARD,
    acts: &[Act::Run(1)],
    holds: &[Check::Absent(Place::Repo(AREA))],
    ..M
}];
const DISCARD_OVER_AN_UNLINKED_CHECKOUT: [Case; 1] = [Case {
    name: "dirty-worktree-checkout-with-no-git-link-and-staged-paths-in-the-registration-discarded-by-the-consent",
    site: DIRTY,
    cause: "a sub-task's worktree stands with its `.git` link moved out, beside a \
            registration that still holds a staged path, and the milestone is abandoned",
    state: fan![
        WORK,
        STAGED,
        sh!("mv .jigc/worktrees/do-a-thing/.git ../outside/dotgit")
    ],
    refusing: DISCARD,
    acts: &[Act::Run(1)],
    holds: &[Check::Absent(Place::Repo(AREA))],
    ..M
}];

const DISCARD_OVER_A_WORKTREE_GIT_CANNOT_READ: [Case; 1] = [Case {
    name: "dirty-worktree-live-worktree-whose-index-is-no-index-discarded-by-the-consent",
    site: DIRTY,
    cause: "a sub-task's provisioned worktree stands and the index of git's registration of \
            it is no index: git cannot say what it holds, and the milestone is abandoned",
    state: fan![WORK, STAGED, ITS_INDEX_GARBAGE],
    refusing: DISCARD,
    acts: &[Act::Run(1)],
    holds: &[Check::Absent(Place::Repo(AREA))],
    ..M
}];
const A_GIT_ENTRY_THAT_LEADS_NOWHERE: &str = "a sub-task's worktree has a `.git` entry git \
    reads no repository through, beside a registration that still holds a staged path, and \
    the milestone is abandoned";
const DISCARD_OVER_A_GIT_ENTRY_THAT_LEADS_NOWHERE: [Case; 2] = [
    Case {
        name: "dirty-worktree-a-git-entry-that-leads-nowhere-moved-out-and-discarded-as-printed",
        site: DIRTY,
        cause: A_GIT_ENTRY_THAT_LEADS_NOWHERE,
        state: fan![
            WORK,
            STAGED,
            Step::File(
                Place::Repo(".jigc/worktrees/do-a-thing/.git"),
                "gitdir: /nowhere\n"
            )
        ],
        refusing: DISCARD,
        acts: &[
            Act::MoveOut(Operand::Unprinted(".jigc/worktrees/do-a-thing/.git")),
            Act::RunCited(Cited {
                line: 0,
                promised: THE_LINE_THAT_RE_LINKS,
            }),
        ],
        ..M
    },
    Case {
        name: "dirty-worktree-a-git-entry-that-leads-nowhere-discarded-by-the-consent",
        site: DIRTY,
        cause: A_GIT_ENTRY_THAT_LEADS_NOWHERE,
        state: fan![
            WORK,
            STAGED,
            Step::File(
                Place::Repo(".jigc/worktrees/do-a-thing/.git"),
                "gitdir: /nowhere\n"
            )
        ],
        refusing: DISCARD,
        acts: &[Act::Run(1)],
        holds: &[Check::Absent(Place::Repo(AREA))],
        ..M
    },
];
/// A copy of the whole repository beside it — `cp -R` — whose worktrees git does not list
/// under the copy's paths: a case of the copy stands in it.
const A_COPY_OF_THE_REPOSITORY: Step = sh!("cp -R . ../outside/copy");
const THE_COPY: Place = Place::Outside("copy");
const A_WORKTREE_OF_A_COPIED_REPOSITORY: &str = "the repository is a copy of another: a \
    sub-task's worktree stands at its path with a file git does not track, and this \
    repository has not registered it — the milestone is abandoned in the copy";
const DISCARD_IN_A_COPY: [Case; 2] = [
    Case {
        name: "dirty-worktree-in-a-copied-repository-file-moved-out-and-discarded-as-printed",
        site: DIRTY,
        cause: A_WORKTREE_OF_A_COPIED_REPOSITORY,
        state: fan![LOOSE, A_COPY_OF_THE_REPOSITORY],
        refusing: DISCARD,
        from: THE_COPY,
        acts: &[
            Act::MoveOut(Operand::Unprinted(".jigc/worktrees/do-a-thing/loose.txt")),
            Act::Run(0),
        ],
        holds: &[Check::Absent(Place::Outside(
            "copy/.jigc/milestones/fan-out",
        ))],
        ..M
    },
    Case {
        name: "dirty-worktree-in-a-copied-repository-discarded-by-the-consent-as-printed",
        site: DIRTY,
        cause: A_WORKTREE_OF_A_COPIED_REPOSITORY,
        state: fan![LOOSE, A_COPY_OF_THE_REPOSITORY],
        refusing: DISCARD,
        from: THE_COPY,
        acts: &[Act::Run(1)],
        holds: &[
            Check::Absent(Place::Outside("copy/.jigc/milestones/fan-out")),
            Check::RegularFile(Place::Outside("copy/.jigc/worktrees/do-a-thing/loose.txt")),
        ],
        ..M
    },
];

// `leftover_finding` · `milestone.leftover-holds-work` — `jigc milestone provision`

const A_FILE_WHERE_A_WORKTREE_IS_TO_BE_MADE: &str = "a regular file stands where a \
    sub-task's worktree is to be provisioned";
const PROVISION_OVER_A_FILE: [Case; 2] = [
    Case {
        name: "leftover-a-file-at-the-path-moved-out-and-provisioned-by-the-consent",
        site: LEFTOVER,
        cause: A_FILE_WHERE_A_WORKTREE_IS_TO_BE_MADE,
        state: bare![Step::File(Place::Repo(WT), "x\n")],
        refusing: PROVISION,
        acts: &[Act::MoveOut(Operand::Unprinted(WT)), Act::Run(0)],
        holds: &[
            Check::Directory(Place::Repo(WT)),
            Check::RegularFile(Place::Moved(WT)),
        ],
        ..M
    },
    Case {
        name: "leftover-a-file-at-the-path-provisioned-by-the-consent-as-printed",
        site: LEFTOVER,
        cause: A_FILE_WHERE_A_WORKTREE_IS_TO_BE_MADE,
        state: bare![Step::File(Place::Repo(WT), "x\n")],
        refusing: PROVISION,
        acts: &[Act::Run(0)],
        holds: &[Check::Directory(Place::Repo(WT))],
        ..M
    },
];
const PROVISION_OVER_A_LINK: [Case; 1] = [Case {
    name: "leftover-a-link-to-nothing-at-the-path-provisioned-by-the-consent-as-printed",
    site: LEFTOVER,
    cause: "a link that leads nowhere stands where a sub-task's worktree is to be \
            provisioned",
    state: bare![Step::LinkTo(Place::Repo(WT), "nowhere")],
    refusing: PROVISION,
    acts: &[Act::Run(0)],
    holds: &[Check::Directory(Place::Repo(WT))],
    ..M
}];
const A_DIRECTORY_WHERE_A_WORKTREE_IS_TO_BE_MADE: &str = "a directory that is no worktree \
    stands where a sub-task's worktree is to be provisioned, and holds a file";
const PROVISION_OVER_A_DIRECTORY: [Case; 2] = [
    Case {
        name: "leftover-unregistered-directory-its-file-moved-out-and-provisioned-by-the-consent",
        site: LEFTOVER,
        cause: A_DIRECTORY_WHERE_A_WORKTREE_IS_TO_BE_MADE,
        state: bare![Step::File(
            Place::Repo(".jigc/worktrees/do-a-thing/junk.txt"),
            "junk\n"
        )],
        refusing: PROVISION,
        acts: &[
            Act::MoveOut(Operand::Unprinted(".jigc/worktrees/do-a-thing/junk.txt")),
            Act::Run(0),
        ],
        holds: &[
            Check::RegularFile(Place::Repo(".jigc/worktrees/do-a-thing/README.md")),
            Check::RegularFile(Place::Moved(".jigc/worktrees/do-a-thing/junk.txt")),
        ],
        ..M
    },
    Case {
        name: "leftover-unregistered-directory-provisioned-by-the-consent-as-printed",
        site: LEFTOVER,
        cause: A_DIRECTORY_WHERE_A_WORKTREE_IS_TO_BE_MADE,
        state: bare![Step::File(
            Place::Repo(".jigc/worktrees/do-a-thing/junk.txt"),
            "junk\n"
        )],
        refusing: PROVISION,
        acts: &[Act::Run(0)],
        holds: &[
            Check::RegularFile(Place::Repo(".jigc/worktrees/do-a-thing/README.md")),
            Check::Absent(Place::Repo(".jigc/worktrees/do-a-thing/junk.txt")),
        ],
        ..M
    },
];
const PROVISION_OVER_A_REGISTRATION_WITH_STAGED_PATHS: [Case; 1] = [Case {
    name: "leftover-no-directory-and-staged-paths-in-the-registration-provisioned-by-the-consent",
    site: LEFTOVER,
    cause: "a sub-task's worktree directory is gone and git's registration of it still \
            holds a staged path, and the worktree is provisioned again",
    state: fan![WORK, STAGED, THE_WORKTREE_GONE],
    refusing: PROVISION,
    acts: &[Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo(
        ".jigc/worktrees/do-a-thing/README.md",
    ))],
    ..M
}];
const A_REGISTRATION_WITH_A_COMMIT: &str = "a sub-task's worktree directory is gone and \
    git's registration of it still holds a commit no ref reaches, and the worktree is \
    provisioned again";
const PROVISION_OVER_A_REGISTRATION_WITH_A_COMMIT: [Case; 2] = [
    Case {
        name: "leftover-no-directory-and-a-commit-in-the-registration-kept-and-provisioned-by-the-consent",
        site: LEFTOVER,
        cause: A_REGISTRATION_WITH_A_COMMIT,
        state: fan![WORK, STAGED, COMMITTED, THE_WORKTREE_GONE],
        refusing: PROVISION,
        acts: &[Act::RunListed(0), Act::Run(0)],
        holds: &[Check::RegularFile(Place::Repo(
            ".jigc/worktrees/do-a-thing/README.md",
        ))],
        ..M
    },
    Case {
        name: "leftover-no-directory-and-a-commit-in-the-registration-provisioned-by-the-consent",
        site: LEFTOVER,
        cause: A_REGISTRATION_WITH_A_COMMIT,
        state: fan![WORK, STAGED, COMMITTED, THE_WORKTREE_GONE],
        refusing: PROVISION,
        acts: &[Act::Run(0)],
        holds: &[Check::RegularFile(Place::Repo(
            ".jigc/worktrees/do-a-thing/README.md",
        ))],
        ..M
    },
];
const PROVISION_OVER_AN_UNREADABLE_REGISTRATION: [Case; 1] = [Case {
    name: "leftover-no-directory-and-a-registration-git-cannot-read-provisioned-by-the-consent",
    site: LEFTOVER,
    cause: "a sub-task's worktree directory is gone and the index of git's registration of \
            it is no index: nothing can be read there, and the worktree is provisioned again",
    state: fan![WORK, STAGED, THE_WORKTREE_GONE, ITS_INDEX_GARBAGE],
    refusing: PROVISION,
    acts: &[Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo(
        ".jigc/worktrees/do-a-thing/README.md",
    ))],
    ..M
}];
const PROVISION_OVER_AN_EMPTY_DIRECTORY_BESIDE_A_REGISTRATION: [Case; 1] = [Case {
    name: "leftover-an-empty-directory-and-staged-paths-in-the-registration-provisioned-by-the-consent",
    site: LEFTOVER,
    cause: "a sub-task's worktree was moved away and an empty directory stands in its \
            place, beside a registration that still holds a staged path",
    state: fan![
        WORK,
        STAGED,
        THE_WORKTREE_MOVED_AWAY,
        Step::Dir(Place::Repo(WT))
    ],
    refusing: PROVISION,
    acts: &[Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo(
        ".jigc/worktrees/do-a-thing/README.md",
    ))],
    ..M
}];

const PROVISION_IN_A_COPY: [Case; 1] = [Case {
    name: "leftover-in-a-copied-repository-worktree-with-a-loose-file-provisioned-by-the-consent",
    site: LEFTOVER,
    cause: "the repository is a copy of another: a sub-task's worktree stands at its path \
            with a file git does not track, this repository has not registered it, and the \
            worktree is provisioned in the copy",
    state: fan![LOOSE, A_COPY_OF_THE_REPOSITORY],
    refusing: PROVISION,
    from: THE_COPY,
    acts: &[Act::Run(0)],
    holds: &[Check::Absent(Place::Outside(
        "copy/.jigc/worktrees/do-a-thing/loose.txt",
    ))],
    ..M
}];
const PROVISION_IN_A_COPY_MID_OPERATION: [Case; 1] = [Case {
    name: "leftover-in-a-copied-repository-a-bisect-abandoned-and-provisioned-by-the-consent",
    site: LEFTOVER,
    cause: "the repository is a copy of another: a sub-task's worktree stands at its path, \
            one git has left mid-operation — a bisect — and this repository has not \
            registered it",
    state: fan![
        Step::Git(&["-C", WT, "bisect", "start"]),
        A_COPY_OF_THE_REPOSITORY
    ],
    refusing: PROVISION,
    from: THE_COPY,
    acts: &[Act::RunListed(0), Act::Run(0)],
    holds: &[Check::RegularFile(Place::Outside(
        "copy/.jigc/worktrees/do-a-thing/README.md",
    ))],
    ..M
}];
const PROVISION_OVER_AN_UNLINKED_CHECKOUT: [Case; 1] = [Case {
    name: "leftover-checkout-with-no-git-link-and-staged-paths-in-the-registration-provisioned-by-the-consent",
    site: LEFTOVER,
    cause: "a sub-task's worktree stands with its `.git` link moved out, beside a \
            registration that still holds a staged path, and the worktree is provisioned \
            again",
    state: fan![
        WORK,
        STAGED,
        sh!("mv .jigc/worktrees/do-a-thing/.git ../outside/dotgit")
    ],
    refusing: PROVISION,
    acts: &[Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo(
        ".jigc/worktrees/do-a-thing/README.md",
    ))],
    ..M
}];

// `unlanded_work_finding` · `milestone.unlanded-work` — `jigc milestone finalize`

/// **What a worktree door promises of a command it cites for what it prints** — held to
/// its words, whatever the command exits with (the human's ruling of 2026-10-12). The
/// promise is the refusal's own, on the line of the path or in its route, and the cited
/// command is refused again, as the refusal says it will be.
///
/// *Move that entry aside, and this door then prints the one line that re-links the
/// checkout*: the door's own re-run, and the line — git's registration written into the
/// checkout's `.git`, by both paths.
const THE_LINE_THAT_RE_LINKS: &[Promise] = &[Promise::Line(&[
    Says::Text("printf 'gitdir: %s\\n' "),
    Says::Path(Place::Repo(".git/worktrees/do-a-thing")),
    Says::Path(Place::Repo(".jigc/worktrees/do-a-thing/.git")),
])];
/// *Move it aside and re-run for the command that brings the checkout back*: the door's
/// own re-run, and the recipe — the directory made, the registration written into its
/// `.git`, the files restored.
const THE_COMMAND_THAT_BRINGS_THE_CHECKOUT_BACK: &[Promise] = &[Promise::Line(&[
    Says::Text("mkdir -p "),
    Says::Text("printf 'gitdir: %s\\n' "),
    Says::Path(Place::Repo(".git/worktrees/do-a-thing")),
    Says::Path(Place::Repo(".jigc/worktrees/do-a-thing/.git")),
    Says::Text(" restore ."),
])];
/// *Or abandon the milestone with `jigc milestone discard <id>`, which answers for the
/// same path with its own route*: a finding at that path that carries a route of its own.
const A_ROUTE_OF_ITS_OWN_AT_THE_SAME_PATH: &[Promise] = &[Promise::Route(&[Says::Text(
    "at: .jigc/worktrees/do-a-thing",
)])];

const A_COMMIT_WHERE_STAGED_PATHS_WERE_WANTED: &str = "a sub-agent committed its work in its \
    worktree instead of staging it: the worktree's `HEAD` holds a commit no ref reaches, and \
    the milestone's finalize is asked";
const FINALIZE_OVER_A_COMMIT: [Case; 3] = [
    Case {
        name: "unlanded-a-commit-kept-on-a-branch-beside-another-sub-tasks-work-and-finalized",
        site: UNLANDED,
        cause: A_COMMIT_WHERE_STAGED_PATHS_WERE_WANTED,
        state: fan2![
            Step::File(
                Place::Repo(".jigc/worktrees/second-thing/other.txt"),
                "other\n"
            ),
            Step::Git(&["-C", WT2, "add", "other.txt"]),
            WORK,
            STAGED,
            COMMITTED,
        ],
        acts: &[Act::RunListed(0), Act::Run(0)],
        holds: &[
            Check::RegularFile(Place::Repo("other.txt")),
            Check::Absent(Place::Repo(AREA)),
        ],
        ..M
    },
    Case {
        name: "unlanded-a-commit-turned-into-staged-paths-and-finalized-as-printed",
        site: UNLANDED,
        cause: A_COMMIT_WHERE_STAGED_PATHS_WERE_WANTED,
        state: fan![WORK, STAGED, COMMITTED],
        acts: &[Act::RunListed(1), Act::Run(0)],
        holds: &[Check::RegularFile(Place::Repo("work.txt"))],
        ..M
    },
    Case {
        name: "unlanded-a-commit-kept-on-a-branch-and-finalized-as-printed",
        site: UNLANDED,
        cause: A_COMMIT_WHERE_STAGED_PATHS_WERE_WANTED,
        state: fan![WORK, STAGED, COMMITTED],
        acts: &[Act::RunListed(0), Act::Run(0)],
        holds: &[Check::Absent(Place::Repo(AREA))],
        ..M
    },
];
const FINALIZE_OVER_A_REGISTRATION_WITH_STAGED_PATHS: [Case; 1] = [Case {
    name: "unlanded-no-directory-and-staged-paths-in-the-registration-milestone-discarded-as-printed",
    site: UNLANDED,
    cause: "a sub-task's worktree directory is gone and git's registration of it still \
            holds the staged path the boundary was to land",
    state: fan![WORK, STAGED, THE_WORKTREE_GONE],
    acts: &[Act::RunCited(Cited {
        line: 1,
        promised: A_ROUTE_OF_ITS_OWN_AT_THE_SAME_PATH,
    })],
    ..M
}];
const FINALIZE_OVER_A_REGISTRATION_WITH_A_COMMIT: [Case; 1] = [Case {
    name: "unlanded-no-directory-and-a-commit-in-the-registration-kept-and-finalized-as-printed",
    site: UNLANDED,
    cause: "a sub-task's worktree directory is gone and git's registration of it still \
            holds a commit no ref reaches",
    state: fan2![
        Step::File(
            Place::Repo(".jigc/worktrees/second-thing/other.txt"),
            "other\n"
        ),
        Step::Git(&["-C", WT2, "add", "other.txt"]),
        WORK,
        STAGED,
        COMMITTED,
        THE_WORKTREE_GONE,
    ],
    acts: &[Act::RunListed(0), Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo("other.txt"))],
    ..M
}];
/// A second sub-task settled as discarded — `jigc task discard` — whose provisioned
/// worktree still stands, and what it holds; the first one's staged work is what lands.
macro_rules! settled {
    ($($plant:expr),* $(,)?) => {
        fan2![
            WORK,
            STAGED,
            Step::Jigc(&["task", "discard", "second-thing"]),
            $($plant,)*
        ]
    };
}
const IN_THE_SECOND: [Step; 3] = [
    Step::File(Place::Repo(".jigc/worktrees/second-thing/s.txt"), "s\n"),
    Step::Git(&["-C", WT2, "add", "s.txt"]),
    Step::Git(&[
        "-C",
        WT2,
        "commit",
        "-q",
        "-m",
        "the settled sub-task's work",
    ]),
];
const FINALIZE_OVER_A_SETTLED_SUB_TASKS_STAGED_PATHS: [Case; 1] = [Case {
    name: "unlanded-settled-sub-task-staged-paths-stashed-and-finalized-as-printed",
    site: UNLANDED,
    cause: "a sub-task settled as discarded still has a staged path in its worktree, which \
            the boundary lands nothing from",
    state: settled![IN_THE_SECOND[0], IN_THE_SECOND[1]],
    acts: &[Act::RunListed(0), Act::Run(0)],
    holds: &[
        Check::RegularFile(Place::Repo("work.txt")),
        Check::Absent(Place::Repo("s.txt")),
    ],
    ..M
}];
const FINALIZE_OVER_A_SETTLED_SUB_TASKS_LOOSE_WORK: [Case; 1] = [Case {
    name: "unlanded-settled-sub-task-untracked-file-stashed-and-finalized-as-printed",
    site: UNLANDED,
    cause: "a sub-task settled as discarded still has a file git does not track in its \
            worktree, which the teardown would remove",
    state: settled![Step::File(
        Place::Repo(".jigc/worktrees/second-thing/u.txt"),
        "u\n"
    )],
    acts: &[Act::RunListed(0), Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo("work.txt"))],
    ..M
}];
const FINALIZE_OVER_A_SETTLED_SUB_TASKS_COMMIT: [Case; 1] = [Case {
    name: "unlanded-settled-sub-task-a-commit-kept-and-finalized-as-printed",
    site: UNLANDED,
    cause: "a sub-task settled as discarded still has a commit no ref reaches in its \
            worktree",
    state: settled![IN_THE_SECOND[0], IN_THE_SECOND[1], IN_THE_SECOND[2]],
    acts: &[Act::RunListed(0), Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo("work.txt"))],
    ..M
}];
const FINALIZE_OVER_AN_UNREADABLE_REGISTRATION: [Case; 1] = [Case {
    name: "unlanded-no-directory-and-a-registration-git-cannot-read-milestone-discarded-as-printed",
    site: UNLANDED,
    cause: "a sub-task's worktree directory is gone and the index of git's registration of \
            it is no index: nothing says what it holds",
    state: fan![WORK, STAGED, THE_WORKTREE_GONE, ITS_INDEX_GARBAGE],
    acts: &[Act::RunCited(Cited {
        line: 1,
        promised: A_ROUTE_OF_ITS_OWN_AT_THE_SAME_PATH,
    })],
    ..M
}];
const FINALIZE_OVER_A_FILE_IN_THE_WAY: [Case; 1] = [Case {
    name: "unlanded-a-file-where-the-worktree-was-moved-out-and-finalized-as-printed",
    site: UNLANDED,
    cause: "a sub-task's worktree was moved away and a file stands in its place, beside a \
            registration that still holds a staged path",
    state: fan![
        WORK,
        STAGED,
        THE_WORKTREE_MOVED_AWAY,
        Step::File(Place::Repo(WT), "x\n")
    ],
    acts: &[
        Act::MoveOut(Operand::Unprinted(WT)),
        Act::RunCited(Cited {
            line: 0,
            promised: THE_COMMAND_THAT_BRINGS_THE_CHECKOUT_BACK,
        }),
    ],
    ..M
}];
const FINALIZE_OVER_A_GIT_ENTRY_THAT_LEADS_NOWHERE: [Case; 1] = [Case {
    name: "unlanded-a-git-entry-that-leads-nowhere-moved-out-and-finalized-as-printed",
    site: UNLANDED,
    cause: "a sub-task's worktree has a `.git` entry git reads no repository through, \
            beside a registration that still holds a staged path",
    state: fan![
        WORK,
        STAGED,
        Step::File(
            Place::Repo(".jigc/worktrees/do-a-thing/.git"),
            "gitdir: /nowhere\n"
        )
    ],
    acts: &[
        Act::MoveOut(Operand::Unprinted(".jigc/worktrees/do-a-thing/.git")),
        Act::RunCited(Cited {
            line: 0,
            promised: THE_LINE_THAT_RE_LINKS,
        }),
    ],
    ..M
}];
const FINALIZE_OVER_AN_UNLINKED_CHECKOUT: [Case; 1] = [Case {
    name: "unlanded-checkout-with-no-git-link-milestone-discarded-as-printed",
    site: UNLANDED,
    cause: "a sub-task's worktree stands with its `.git` link moved out, beside a \
            registration that still holds a staged path",
    state: fan![
        WORK,
        STAGED,
        sh!("mv .jigc/worktrees/do-a-thing/.git ../outside/dotgit")
    ],
    acts: &[Act::RunCited(Cited {
        line: 1,
        promised: A_ROUTE_OF_ITS_OWN_AT_THE_SAME_PATH,
    })],
    ..M
}];

const FINALIZE_IN_A_COPY: [Case; 1] = [Case {
    name: "unlanded-in-a-copied-repository-a-commit-kept-and-finalized-as-printed",
    site: UNLANDED,
    cause: "the repository is a copy of another: a sub-task's worktree in the copy holds a \
            commit no ref reaches, and this repository has not registered that worktree",
    state: fan2![
        Step::File(
            Place::Repo(".jigc/worktrees/second-thing/other.txt"),
            "other\n"
        ),
        Step::Git(&["-C", WT2, "add", "other.txt"]),
        WORK,
        STAGED,
        COMMITTED,
        A_COPY_OF_THE_REPOSITORY,
    ],
    from: THE_COPY,
    acts: &[Act::RunListed(0), Act::Run(0)],
    ..M
}];

/// The cases of the family, a group a cause — and, within a group, a case a way out of the
/// cause's route.
pub const CASES: &[&[Case]] = &[
    &THE_ID_HAS_A_LIVE_RECORD,
    &THE_ID_HAS_A_SETTLED_RECORD,
    &THE_HOME_HOLDS_A_FILE_THAT_IS_NO_RECORD,
    &THE_RECORD_IS_IN_HEAD_AND_NOT_ON_DISK,
    &THE_RECORD_IS_IN_THE_INDEX_AND_NOT_ON_DISK,
    &NO_WORKTREE_WAS_EVER_PROVISIONED,
    &NOTHING_IS_STAGED_IN_ANY_WORKTREE,
    &THE_WORK_IS_NOT_STAGED,
    &ONLY_A_TRANSIENT_DOC,
    &ONLY_A_TRANSIENT_DOC_WHERE_NO_RECORD_FLIPS,
    &A_COLLISION_AT_THE_JOIN,
    &A_COLLISION_AT_THE_BOUNDARY,
    &A_COLLISION_AT_THE_UNSQUASHED_BOUNDARY,
    &GITS_WORKTREES_DIRECTORY_IS_A_FILE,
    &A_LINK_IN_THE_WAY_UNDER_THE_CONSENT,
    &A_STALE_REGISTRATION_GIT_WILL_NOT_DROP,
    &A_HOOK_OF_GITS_FAILS_BEHIND_THE_CHECKOUT,
    &THE_MILESTONES_DIRECTORY_A_FILE,
    &A_LINK_TO_NOTHING_AT_THE_AREA,
    &THE_BASE_PIN_NOT_JSON,
    &THE_TASK_LIST_NOT_JSON,
    &THE_TASK_LIST_A_DIRECTORY,
    &NO_AREA_AND_THE_MILESTONES_DIRECTORY_A_FILE,
    &THE_BASE_PIN_A_DIRECTORY_AT_THE_RESEED,
    &THE_TASK_LIST_A_DIRECTORY_AT_THE_RESEED,
    &A_LINK_TO_NOTHING_AT_THE_STAGING_AREA,
    &A_FILE_AT_THE_STAGING_AREA,
    &A_DIRECTORY_AT_A_MERGED_BODYS_PATH,
    &A_SUB_TASKS_MANIFEST_NOT_JSON,
    &A_SUB_TASKS_MANIFEST_A_DIRECTORY,
    &A_STAGED_BODY_A_DIRECTORY,
    &A_STAGED_BODY_NOT_TEXT,
    &DISCARD_OVER_LOOSE_WORK,
    &DISCARD_OVER_A_COMMIT,
    &DISCARD_OVER_AN_OPERATION,
    &DISCARD_OVER_A_REGISTRATION_WITH_STAGED_PATHS,
    &DISCARD_OVER_A_REGISTRATION_WITH_A_COMMIT,
    &DISCARD_OVER_AN_UNREGISTERED_DIRECTORY,
    &DISCARD_OVER_A_FILE,
    &DISCARD_OVER_AN_UNREADABLE_REGISTRATION,
    &DISCARD_OVER_AN_UNLINKED_CHECKOUT,
    &DISCARD_OVER_A_WORKTREE_GIT_CANNOT_READ,
    &DISCARD_OVER_A_GIT_ENTRY_THAT_LEADS_NOWHERE,
    &DISCARD_IN_A_COPY,
    &PROVISION_OVER_A_FILE,
    &PROVISION_OVER_A_LINK,
    &PROVISION_OVER_A_DIRECTORY,
    &PROVISION_OVER_A_REGISTRATION_WITH_STAGED_PATHS,
    &PROVISION_OVER_A_REGISTRATION_WITH_A_COMMIT,
    &PROVISION_OVER_AN_UNREADABLE_REGISTRATION,
    &PROVISION_OVER_AN_EMPTY_DIRECTORY_BESIDE_A_REGISTRATION,
    &PROVISION_IN_A_COPY,
    &PROVISION_IN_A_COPY_MID_OPERATION,
    &PROVISION_OVER_AN_UNLINKED_CHECKOUT,
    &FINALIZE_OVER_A_COMMIT,
    &FINALIZE_OVER_A_REGISTRATION_WITH_STAGED_PATHS,
    &FINALIZE_OVER_A_REGISTRATION_WITH_A_COMMIT,
    &FINALIZE_OVER_A_SETTLED_SUB_TASKS_STAGED_PATHS,
    &FINALIZE_OVER_A_SETTLED_SUB_TASKS_LOOSE_WORK,
    &FINALIZE_OVER_A_SETTLED_SUB_TASKS_COMMIT,
    &FINALIZE_OVER_AN_UNREADABLE_REGISTRATION,
    &FINALIZE_OVER_A_FILE_IN_THE_WAY,
    &FINALIZE_OVER_A_GIT_ENTRY_THAT_LEADS_NOWHERE,
    &FINALIZE_OVER_AN_UNLINKED_CHECKOUT,
    &FINALIZE_IN_A_COPY,
];

/// The sites of the family that have no case, each with why: none of the eight.
pub const REASONS: &[Reason] = &[];

/// A line of the refusal's listing that is a shell line and no command of git's or jigc's.
const A_SHELL_LINE: &str = "the line is a shell line — `mkdir -p … && printf 'gitdir: %s\\n' … \
    > …/.git && git -C … restore .`, or the `printf … > …/.git` that re-links a checkout — \
    and the frame runs one command of git's or jigc's a line, handing it no shell's \
    operators. Driven by hand at the finalize, over a worktree whose directory is gone and \
    whose registration holds a staged path: the recipe as printed exits 0, and the \
    milestone's finalize then lands the commit";

/// **What was read for each site, and how many causes it holds** — a hand count, with its
/// derivation; the causes that have no case, each with why; and the ways out of its route
/// that are not followed, each with why.
pub const READINGS: &[Reading] = &[
    Reading {
        site: RECORD_EXISTS,
        read: &[
            "crates/engine/src/milestone.rs::record_home_refusal — the one mint, and its two callers there: record_exists_finding, record_home_held_finding",
            "crates/cli/src/milestone.rs::guard_record_free — `jigc milestone create`'s question of the record's home: the disk first, then what git holds where the disk shows nothing",
        ],
        causes: 5,
        derivation: "the home holds a regular file: its status reads live, reads settled \
                     (`discarded` or `joined` — one route, the status interpolated), or does \
                     not read at all — 3. Or the disk shows nothing and git holds the \
                     record: in `HEAD`, or in the index alone — 2. (A home that holds an \
                     entry which is no regular file is the store's refusal, under another \
                     code and at another site.) Whether the milestone's workbench is there \
                     changes nothing of the refusal: a live record is followed with it and \
                     without it",
        without_case: &[],
        not_followed: &[(
            "*or create this one under a different title*",
            Why::NotConstructible(
                "the title is the reader's own and is an argument of the refused command, \
                 which the frame runs again only where the route prints no command line \
                 of the product — every route here prints one",
            ),
        )],
    },
    Reading {
        site: ZERO,
        read: &[
            "crates/cli/src/milestone.rs::zero_contribution_finding — one route, the message by whether a record would flip",
            "crates/cli/src/milestone.rs::run_milestone_finalize — the guard behind the planner: no promotion planned and no staged code in a worktree; and `has_diff`, which decides whether the planner's own empty commit answers first",
        ],
        causes: 5,
        derivation: "the predicate is *nothing promotes and no worktree holds staged \
                     code*, reached where the planner's empty-commit guard did not answer \
                     first — the record's flip is a diff, or a merged doc is. With a \
                     record: no worktree was ever provisioned; one is and holds nothing; \
                     one holds work that is not staged; a sub-task's one doc is its commit \
                     doc, which promotes nowhere — 4. With no record (the dev pack alone): \
                     only where a transient doc is merged, the empty commit answering \
                     otherwise — 1",
        without_case: &[],
        not_followed: &[(
            "*execute the sub-tasks* — the work itself, between the provision and the `git add`",
            Why::NotConstructible(
                "the sub-tasks' work is the reader's own: the way is followed from the one \
                 state in which it is already written and only the staging is left",
            ),
        )],
    },
    Reading {
        site: COLLISION,
        read: &[
            "crates/cli/src/combine.rs::collision_finding and detect_code_collision — a path staged in more than one worktree",
            "crates/cli/src/combine.rs::combine, and the two calls in crates/cli/src/milestone.rs — the three doors that ask: the join's preview, the boundary's one combined commit, the boundary where each sub-task has its own",
        ],
        causes: 3,
        derivation: "one predicate and one route, asked at three doors: `jigc milestone \
                     join`, `jigc milestone finalize`, and the finalize of a project that \
                     sets `finalize.fan-out.squash: false`, where it is asked up front. How \
                     many paths collide, and among how many sub-tasks, changes the listing \
                     and nothing of the route",
        without_case: &[],
        not_followed: &[(
            "*or combine their overlapping changes by hand*",
            Why::NotConstructible(
                "the combined bytes are the reader's own; the way ends, like the other, in \
                 one sub-task holding the path and the other none — which is what the \
                 cases do",
            ),
        )],
    },
    Reading {
        site: PROVISION_FAILED,
        read: &[
            "crates/cli/src/milestone.rs::provision_failed_finding and ProvisionStop",
            "crates/cli/src/milestone.rs::provision_worktrees — phase 2, the four fallible steps behind the probe: clear a leftover, drop a stale registration, spell the path, `git worktree add`",
            "crates/cli/src/milestone.rs::remove_leftover — what the clearing step removes, and that a link which leads nowhere is not seen by it",
        ],
        causes: 6,
        derivation: "a cause is the step that stopped, by what stopped it. The leftover \
                     cannot be cleared; the stale registration cannot be dropped — a link \
                     stands at its path, and git will not remove what it cannot read as a \
                     worktree; the path is no UTF-8; and `git worktree add` fails three \
                     ways found: git's own `worktrees` directory is a file, the path \
                     exists (a link the clearing step did not see, under the consent), a \
                     `post-checkout` hook exits non-zero behind a worktree that was made",
        without_case: &[
            (
                "the leftover at the path cannot be cleared",
                Why::NotConstructible(
                    "the removal of a directory or a file the probe has read fails only \
                     where an entry may not be written — a permission bit changed by hand, \
                     a declared bound of the catalog",
                ),
            ),
            (
                "the worktree's path is no UTF-8",
                Why::Platform(
                    "the path is the repository's own and a sub-task's id, which is a \
                     slug: only a repository under a directory whose name is no UTF-8 \
                     reaches it, and this platform's file system makes no such name",
                ),
            ),
        ],
        not_followed: &[],
    },
    Reading {
        site: AREA_IO,
        read: &[
            "crates/engine/src/milestone.rs::io_finding — one route, whatever was being done",
            "crates/engine/src/milestone.rs — every call that hands it an error, by the words it is handed: the create (three), add-task (three), the cache's re-seed from the record (three), the record's read and its write at the join, at the discard and at a sub-task's discard (four), the join's staging area and its bodies (three), the gate's read of a sub-task's manifest and of a staged body (two) — eighteen in all",
            "crates/engine/src/milestone.rs::clear_staged_bodies, materialize, reseed_cache_from_record — what each of those calls opens, reads or writes",
        ],
        causes: 26,
        derivation: "a cause is what was being done when the I/O call failed, by the \
                     door that asked — the message says the first, the command the second. \
                     Nineteen are followed, each a state that is no permission: a file \
                     where a directory is opened, a directory where a file is read or \
                     written, a link that leads nowhere, text that is no JSON, bytes that \
                     are no text — the area opened at the create (2), the base pin read, \
                     the task list read at three doors and as a directory (4), the cache \
                     opened and re-seeded (3), the staging area opened, cleared and \
                     written to (4), a sub-task's manifest read at two doors and as a \
                     directory (3), a staged body read (2). Seven have a reason",
        without_case: &[
            (
                "write the base pin, at the create",
                Why::NotConstructible(
                    "the area's directory was made by the same process a moment earlier: \
                     only a permission bit changed by hand — a declared bound — or a full \
                     disk fails the write",
                ),
            ),
            (
                "write the task list, at the create",
                Why::NotConstructible("as the base pin: a file written into a directory just made"),
            ),
            (
                "append to the task list, at add-task",
                Why::NotConstructible(
                    "the list is read a moment earlier by the same call, which answers \
                     whatever stands at its path that is no list; what is left is a \
                     permission, a declared bound",
                ),
            ),
            (
                "read the milestone record — at the join, at the discard and at a \
                 sub-task's discard",
                Why::NotConstructible(
                    "each door reads the record earlier and answers a fault of it first: \
                     an edit in no commit draws `reconciliation.conflict-block` from the \
                     record's preflight, and a record committed with bytes that are no \
                     text, with no hash of jigc's held for it, draws a plain error and no \
                     code — driven at the three doors, both ways",
                ),
            ),
            (
                "write the joined milestone record",
                Why::NotConstructible(
                    "the write is in place and through no link, and the record's door \
                     refuses a home that is no regular file before it: what is left is a \
                     file that may not be written, a declared bound",
                ),
            ),
            (
                "write the discarded milestone record",
                Why::NotConstructible("as the joined record's write"),
            ),
            (
                "write the settled sub-task's milestone record",
                Why::NotConstructible("as the joined record's write"),
            ),
        ],
        not_followed: &[(
            "the entry at fault itself put right",
            Why::NotConstructible(
                "the route prints `.jigc/` and names no entry; where a route prints a \
                 path, the frame takes no other object from a case",
            ),
        )],
    },
    Reading {
        site: DIRTY,
        read: &[
            "crates/cli/src/milestone.rs::dirty_worktree_finding — one route, grown by a clause for a path mid-operation and one for what a registration holds; a line a held path, with its fate",
            "crates/cli/src/milestone.rs::held_subtask_worktrees — every sub-task's path is asked, a registered live worktree included",
            "crates/cli/src/milestone.rs::probe_leftover and leftover_at — the three legs of the question: what stands at the path, whether git left it mid-operation, what git's registration of it holds",
            "crates/cli/src/milestone.rs::because, hold_line, held_here, anchored_words and unlinked_checkout — the reason a line gives and the commands it carries",
        ],
        causes: 12,
        derivation: "a cause is what the probe found at a sub-task's path, as the line's \
                     reason and its fate word it. In a worktree this repository \
                     registered: work in no commit; a commit no ref reaches; an operation \
                     git left un-concluded; an index git cannot read — 4. Where the \
                     directory is gone: a registration that holds staged paths, one that \
                     holds a commit, one git cannot read — 3. Where something else stands: \
                     a file; a directory that is no worktree; a checkout whose `.git` link \
                     is gone, beside its registration; one whose `.git` entry leads \
                     nowhere — 4. And a worktree this repository has not registered, in a \
                     copy of the repository — 1. A link at the path is a file to the \
                     probe, and an empty directory beside a registration reads as the \
                     checkout with no link: neither is counted apart",
        without_case: &[],
        not_followed: &[
            (
                "*commit or stash what a live worktree holds*",
                Why::NotConstructible(
                    "the route names the two acts and prints no line for either, and the \
                     frame has neither: the work is moved out of the path by the runner, \
                     which is the route's *get out what you need*",
                ),
            ),
            (
                "the recipe on a line that brings a checkout back, and the line that re-links one",
                Why::NotConstructible(A_SHELL_LINE),
            ),
        ],
    },
    Reading {
        site: LEFTOVER,
        read: &[
            "crates/cli/src/milestone.rs::leftover_finding — one route, the consent, grown by the same two clauses",
            "crates/cli/src/milestone.rs::provision_worktrees — phase 1: a path registered as a live worktree is reused and not asked; every other one is probed",
            "crates/cli/src/milestone.rs::probe_leftover, because and hold_line — as at the discard",
        ],
        causes: 10,
        derivation: "the same probe, asked of the paths the provision will not reuse — so \
                     a worktree this repository registered is no cause here. A file; a \
                     link; a directory that is no worktree; a checkout with no `.git` \
                     link, beside its registration; an empty directory beside one — 5. The \
                     directory gone: a registration that holds staged paths, one that \
                     holds a commit, one git cannot read — 3. And in a copy of the \
                     repository, a worktree it has not registered: with work in no commit, \
                     and mid-operation — 2",
        without_case: &[],
        not_followed: &[(
            "the recipe on a line that brings a checkout back, and the line that re-links one",
            Why::NotConstructible(A_SHELL_LINE),
        )],
    },
    Reading {
        site: UNLANDED,
        read: &[
            "crates/cli/src/milestone.rs::unlanded_work_finding — the reason, the fate, the exits on the line, and a route whose first move turns on whether the line carries a command",
            "crates/cli/src/milestone.rs::unlanded_work and loose_work — which paths are asked: a sub-task the boundary lands, and one settled as discarded whose worktree is still registered",
            "crates/cli/src/milestone.rs::run_milestone_finalize — where the guard stands: beside the posture findings, before the record flips",
            "crates/cli/src/milestone.rs::hold_line, anchored_words and unlinked_checkout — the commands a line carries",
        ],
        causes: 12,
        derivation: "a cause is what a sub-task's worktree holds that the boundary would \
                     land without. Of a sub-task it lands: a commit in a live worktree; \
                     the directory gone and a registration that holds staged paths, a \
                     commit, or cannot be read; a file where the worktree was; a `.git` \
                     entry that leads nowhere; a checkout with no `.git` link — 7. Of a \
                     sub-task settled as discarded, in its live worktree: staged paths, \
                     work git does not track, a commit — 3. And a worktree this \
                     repository has not registered: in a copy of the repository — 1 — and \
                     in a repository that was moved, where git lists the registration at \
                     the path it had — 1",
        without_case: &[(
            "the repository was moved, and git lists the worktree's registration at the \
             path it had before",
            Why::NotConstructible(
                "a case's repository stands where the sandbox made it and every step runs \
                 there: no step moves the repository itself. A copy beside it — followed — \
                 has the registration at the original's path with the original still there",
            ),
        )],
        not_followed: &[
            (
                "the recipe on a line that brings a checkout back, and the line that re-links one",
                Why::NotConstructible(A_SHELL_LINE),
            ),
            (
                "*mend what git's message on that line names*",
                Why::NotConstructible(
                    "the route names no act and prints no line — it says so itself — and \
                     the frame has no act that mends an index: the cause is followed by \
                     the route's other way",
                ),
            ),
            (
                "the route's other way, `jigc milestone discard <id>`, at every cause",
                Why::NotConstructible(
                    "it is one line, the same at every cause, and it leads to the discard \
                     door's own question of the same path: followed where it is the one \
                     line the frame can run — three causes — and at no other",
                ),
            ),
        ],
    },
];
