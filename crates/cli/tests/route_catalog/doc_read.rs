//! The route catalog's cases for the family the list of refusal sites calls `doc read`:
//! the `store.*` codes — the refusals every door that takes an address or writes a home
//! raises about the committed store itself. The family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here** is not the family: it is the one site of it that the fix pass
//! since the previous release touched, as the human ruled on 2026-10-11 (`DECISIONS.md` →
//! *Which families the route catalog takes in*) — and that site is new with the pass: the
//! one constructor of `store.home-not-regular-file`, the refusal over a managed doc's
//! home that holds a symbolic link or a directory where jigc keeps a regular file. The
//! rest are rows of the scheduled list, two of them with the change of a caller that was
//! read and why it is off the path.
//!
//! **One constructor, a door a route.** The code, the key — the entry's path — and the
//! sentence that says what stands there are the constructor's; what the door did not do
//! and the way out are the door's. So a cause here is a door and what it met: the task's
//! finalize and the milestone's over a doc a unit minted or edited, `jigc rename` at the
//! doc's own home, at a referrer's and at the destination, the in-task rename's
//! destination, a create under an entry that only creates, the milestone's create, and
//! the four doors that rewrite the milestone's record.
//!
//! **The two ways the routes offer are the frame's two newest acts**: *move the link out
//! of the doc's home* ([`Act::MoveLinkOut`]) and *put the regular file itself there — for
//! a link, a copy of the file it points at, in the link's place*
//! ([`Act::PutTheFileItself`]). A directory in the way is moved as every directory is
//! ([`Act::MoveAway`]).

use super::{Reading, Reason, Why};
use crate::support::route_case::{
    Act, Case, Check, Cited, Operand, Place, Promise, Says, Step,
};

// The site, as the committed list of refusal sites names it.
const HOME_SHAPE: &str =
    "crates/engine/src/store.rs::home_shape_refusal::store.home-not-regular-file#1";

const SETUP: Step = Step::Jigc(&["setup"]);

/// What every case of the family shares: it stands at the repository's root.
const H: Case = Case {
    name: "",
    site: HOME_SHAPE,
    cause: "",
    state: &[],
    refusing: &[],
    from: Place::Repo(""),
    env: &[],
    acts: &[],
    holds: &[],
    witnesses: &[],
    untouched: &[],
    doubt: None,
    over: None,
};

/// A shell line a `git` alias runs at the repository's root — a state has no step that
/// removes or moves.
macro_rules! sh {
    ($line:literal) => {
        Step::Git(&["-c", concat!("alias.plant=!", $line), "plant"])
    };
}

/// The two writes a `commit` doc requires, for the task `$id`.
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

/// An ADR titled *Use tabs*, whole in one payload — and the same with a `supersedes`
/// edge to it, titled *Use spaces now*.
const AN_ADR_PAYLOAD: Step = Step::File(
    Place::Outside("adr.yaml"),
    "title: \"Use tabs\"\nsections:\n  - id: context\n    set:\n      context: |-\n        \
     <<Because.>>\n  - id: decision\n    set:\n      decision: |-\n        <<Tabs.>>\n  - id: \
     consequences\n    set:\n      consequences: |-\n        <<None.>>\n",
);
const A_SUPERSEDING_PAYLOAD: Step = Step::File(
    Place::Outside("next.yaml"),
    "title: \"Use spaces now\"\nsections:\n  - id: status\n    set:\n      supersedes: \
     \"[adr:use-tabs]\"\n  - id: context\n    set:\n      context: |-\n        <<Because.>>\n  \
     - id: decision\n    set:\n      decision: |-\n        <<Spaces.>>\n  - id: consequences\n    \
     set:\n      consequences: |-\n        <<None.>>\n",
);
/// The ADR's home, the home it has once a printed in-task rename is run as printed —
/// `--to "<title>"` slugs to `title` — and its home under the title *Use spaces*.
const THE_ADR: &str = "docs/decisions/use-tabs.md";
const THE_ADR_RETITLED: &str = "docs/decisions/title.md";
const THE_ADR_AS_USE_SPACES: &str = "docs/decisions/use-spaces.md";
const PICK_TABS: &str = ".jigc/tasks/pick-tabs";

/// **An install and an open task of `record-decision` — `pick-tabs` — whose ADR is
/// authored and whose commit doc is complete: a task that finalizes clean**, and what the
/// case plants.
const PICK_TABS_COMMIT_DOC: [Step; 3] = commit_doc!("pick-tabs");
macro_rules! an_adr_ready {
    ($($plant:expr),* $(,)?) => {
        &[
            SETUP,
            Step::Jigc(&["start", "--workflow", "record-decision", "pick tabs"]),
            AN_ADR_PAYLOAD,
            Step::Jigc(&[
                "doc",
                "author",
                "adr",
                "--from-file",
                "../outside/adr.yaml",
                "--task",
                "pick-tabs",
            ]),
            PICK_TABS_COMMIT_DOC[0],
            PICK_TABS_COMMIT_DOC[1],
            PICK_TABS_COMMIT_DOC[2],
            $($plant,)*
        ]
    };
}
/// [`an_adr_ready!`], landed: the ADR committed at its home.
macro_rules! an_adr_committed {
    ($($plant:expr),* $(,)?) => {
        an_adr_ready![Step::Jigc(&["task", "finalize", "pick-tabs"]), $($plant,)*]
    };
}
const FINALIZE_PICK_TABS: &[&str] = &["task", "finalize", "pick-tabs"];

/// A link that leads nowhere, a link to a file outside the repository and an empty
/// directory, each at the ADR's home.
const A_LINK_TO_NOTHING_AT_THE_ADRS_HOME: [Step; 2] = [
    Step::Dir(Place::Repo("docs/decisions")),
    Step::LinkTo(Place::Repo(THE_ADR), "nowhere"),
];
const A_LINK_TO_A_FILE_AT_THE_ADRS_HOME: [Step; 3] = [
    Step::File(Place::Outside("theirs.md"), "theirs\n"),
    Step::Dir(Place::Repo("docs/decisions")),
    Step::Link(Place::Repo(THE_ADR), Place::Outside("theirs.md")),
];
const A_DIRECTORY_AT_THE_ADRS_HOME: Step = Step::Dir(Place::Repo(THE_ADR));
/// The committed doc's home turned into a link to a copy of it outside the repository —
/// in no commit, and committed.
const THE_ADRS_HOME_NOW_A_LINK_TO_A_COPY: [Step; 2] = [
    sh!("mv docs/decisions/use-tabs.md ../outside/copy.md"),
    Step::Link(Place::Repo(THE_ADR), Place::Outside("copy.md")),
];
const THAT_COMMITTED: [Step; 2] = [
    Step::Git(&["add", "-A"]),
    Step::Git(&["commit", "-q", "-m", "a home is a link"]),
];

// ---------------------------------------------------------------------------
// The task's finalize — a doc the task minted, or edited, whose home holds an entry that
// is no regular file
// ---------------------------------------------------------------------------

const A_MINTED_DOC_OVER_A_LINK_TO_NOTHING: &str = "`jigc task finalize`: the home of a doc \
    the task minted holds a symbolic link that leads nowhere";
const MINTED_OVER_A_LINK_TO_NOTHING: [Case; 2] = [
    Case {
        name: "home-shape-task-minted-over-a-link-to-nothing-renamed-and-finalized-as-printed",
        cause: A_MINTED_DOC_OVER_A_LINK_TO_NOTHING,
        state: an_adr_ready![
            A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[0],
            A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[1],
        ],
        refusing: FINALIZE_PICK_TABS,
        acts: &[Act::Run(1), Act::Run(3)],
        holds: &[
            Check::RegularFile(Place::Repo(THE_ADR_RETITLED)),
            Check::Absent(Place::Repo(PICK_TABS)),
        ],
        ..H
    },
    Case {
        name: "home-shape-task-minted-over-a-link-to-nothing-link-moved-out-and-finalized-as-printed",
        cause: A_MINTED_DOC_OVER_A_LINK_TO_NOTHING,
        state: an_adr_ready![
            A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[0],
            A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[1],
        ],
        refusing: FINALIZE_PICK_TABS,
        acts: &[Act::MoveLinkOut(Operand::Span(2)), Act::Run(3)],
        holds: &[
            Check::RegularFile(Place::Repo(THE_ADR)),
            Check::Absent(Place::Repo(PICK_TABS)),
        ],
        ..H
    },
];
const MINTED_OVER_A_LINK_TO_A_FILE: [Case; 1] = [Case {
    name: "home-shape-task-minted-over-a-link-to-a-file-link-moved-out-and-finalized-as-printed",
    cause: "`jigc task finalize`: the home of a doc the task minted holds a symbolic link to \
            a file outside the repository",
    state: an_adr_ready![
        A_LINK_TO_A_FILE_AT_THE_ADRS_HOME[0],
        A_LINK_TO_A_FILE_AT_THE_ADRS_HOME[1],
        A_LINK_TO_A_FILE_AT_THE_ADRS_HOME[2],
    ],
    refusing: FINALIZE_PICK_TABS,
    acts: &[Act::MoveLinkOut(Operand::Span(2)), Act::Run(3)],
    holds: &[
        Check::RegularFile(Place::Repo(THE_ADR)),
        Check::Absent(Place::Repo(PICK_TABS)),
    ],
    witnesses: &["theirs.md"],
    ..H
}];
const A_MINTED_DOC_OVER_A_DIRECTORY: &str = "`jigc task finalize`: the home of a doc the task \
    minted holds a directory";
const MINTED_OVER_A_DIRECTORY: [Case; 2] = [
    Case {
        name: "home-shape-task-minted-over-a-directory-renamed-and-finalized-as-printed",
        cause: A_MINTED_DOC_OVER_A_DIRECTORY,
        state: an_adr_ready![A_DIRECTORY_AT_THE_ADRS_HOME],
        refusing: FINALIZE_PICK_TABS,
        acts: &[Act::Run(1), Act::Run(3)],
        holds: &[
            Check::RegularFile(Place::Repo(THE_ADR_RETITLED)),
            Check::Directory(Place::Repo(THE_ADR)),
            Check::Absent(Place::Repo(PICK_TABS)),
        ],
        ..H
    },
    Case {
        name: "home-shape-task-minted-over-a-directory-moved-away-and-finalized-as-printed",
        cause: A_MINTED_DOC_OVER_A_DIRECTORY,
        state: an_adr_ready![A_DIRECTORY_AT_THE_ADRS_HOME],
        refusing: FINALIZE_PICK_TABS,
        acts: &[Act::MoveAway(Operand::Span(2)), Act::Run(3)],
        holds: &[
            Check::RegularFile(Place::Repo(THE_ADR)),
            Check::Absent(Place::Repo(PICK_TABS)),
        ],
        ..H
    },
];

/// The decisions log minted by a task of `decided-task` — a doc under a fixed identity,
/// which no rename moves: the route offers the one way.
const WRITE_A_NOTE_COMMIT_DOC: [Step; 3] = commit_doc!("write-a-note");
const THE_LOG: &str = "docs/decisions-log.md";
const A_FIXED_IDENTITY_OVER_A_LINK: [Case; 1] = [Case {
    name: "home-shape-task-fixed-identity-over-a-link-link-moved-out-and-finalized-as-printed",
    cause: "`jigc task finalize`: the home of a doc under a fixed identity, which the task \
            minted, holds a symbolic link",
    state: &[
        SETUP,
        Step::Jigc(&["start", "--workflow", "decided-task", "write a note"]),
        WRITE_A_NOTE_COMMIT_DOC[0],
        WRITE_A_NOTE_COMMIT_DOC[1],
        WRITE_A_NOTE_COMMIT_DOC[2],
        Step::Jigc(&[
            "doc",
            "create",
            "decisions-log",
            "--title",
            "Decisions Log",
            "--task",
            "write-a-note",
        ]),
        Step::Dir(Place::Repo("docs")),
        Step::LinkTo(Place::Repo(THE_LOG), "nowhere"),
    ],
    refusing: &["task", "finalize", "write-a-note"],
    acts: &[Act::MoveLinkOut(Operand::Span(1)), Act::Run(2)],
    holds: &[
        Check::RegularFile(Place::Repo(THE_LOG)),
        Check::Absent(Place::Repo(".jigc/tasks/write-a-note")),
    ],
    ..H
}];

/// A second task — `again` — that edits the committed ADR: one slot written, its commit
/// doc complete.
const AGAIN_COMMIT_DOC: [Step; 3] = commit_doc!("again");
macro_rules! an_edit_of_the_committed_adr {
    ($($plant:expr),* $(,)?) => {
        an_adr_committed![
            Step::Jigc(&["start", "--workflow", "record-decision", "again"]),
            Step::File(Place::Outside("more"), "More.\n"),
            Step::Jigc(&[
                "doc",
                "set-slot",
                "adr:use-tabs#context",
                "--task",
                "again",
                "--from-file",
                "../outside/more",
            ]),
            AGAIN_COMMIT_DOC[0],
            AGAIN_COMMIT_DOC[1],
            AGAIN_COMMIT_DOC[2],
            $($plant,)*
        ]
    };
}
const AN_EDITED_DOC_OVER_A_LINK_TO_A_COPY: [Case; 1] = [Case {
    name: "home-shape-task-edited-over-a-link-to-a-copy-the-file-itself-put-there-and-finalized-as-printed",
    cause: "`jigc task finalize`: the home of a committed doc the task edits was turned into \
            a symbolic link to a copy of the doc",
    state: an_edit_of_the_committed_adr![
        THE_ADRS_HOME_NOW_A_LINK_TO_A_COPY[0],
        THE_ADRS_HOME_NOW_A_LINK_TO_A_COPY[1],
    ],
    refusing: &["task", "finalize", "again"],
    acts: &[Act::PutTheFileItself(Operand::Span(1)), Act::Run(2)],
    holds: &[
        Check::RegularFile(Place::Repo(THE_ADR)),
        Check::Contains(Place::Repo(THE_ADR), "More."),
        Check::Absent(Place::Repo(".jigc/tasks/again")),
    ],
    witnesses: &["copy.md"],
    ..H
}];
const AN_EDITED_DOC_OVER_A_LINK_TO_NOTHING: [Case; 1] = [Case {
    name: "home-shape-task-edited-over-a-link-to-nothing-the-file-itself-put-there-and-finalized-as-printed",
    cause: "`jigc task finalize`: the home of a committed doc the task edits was turned into \
            a symbolic link that leads nowhere",
    state: an_edit_of_the_committed_adr![
        sh!("rm docs/decisions/use-tabs.md"),
        Step::LinkTo(Place::Repo(THE_ADR), "nowhere"),
    ],
    refusing: &["task", "finalize", "again"],
    acts: &[Act::PutTheFileItself(Operand::Span(1)), Act::Run(2)],
    holds: &[
        Check::RegularFile(Place::Repo(THE_ADR)),
        Check::Absent(Place::Repo(".jigc/tasks/again")),
    ],
    ..H
}];

/// A migration's doc: a foreign note migrated into an ADR, authored, its commit doc
/// complete — and a link at the home it promotes to.
const THE_MIGRATION: &str = "migrate-adr-notes-old-e1a7d84cafc2";
const THE_MIGRATIONS_COMMIT_DOC: [Step; 3] = commit_doc!("migrate-adr-notes-old-e1a7d84cafc2");
/// What the route promises of the review it cites — *re-run `jigc task finalize <id>` to
/// review the fidelity diff*: the diff itself, the foreign note's own line against the
/// rewrite's, under the home the doc promotes to. The review exits 4 by design, and a
/// cited line is held to its words (the human's ruling of 2026-10-13); the approval behind
/// it is held to exit 0.
const THE_FIDELITY_DIFF: &[Promise] = &[
    Promise::Line(&[Says::Text(
        "+++ canonical rewrite → docs/decisions/use-tabs.md",
    )]),
    Promise::Line(&[Says::Text(
        "- Some text that is long enough to be worth migrating, really.",
    )]),
    Promise::Line(&[Says::Text("+ Because.")]),
];
const A_MIGRATIONS_DOC_OVER_A_LINK: [Case; 1] = [Case {
    name: "home-shape-migration-over-a-link-link-moved-out-reviewed-and-approved-as-printed",
    cause: "`jigc task finalize` of a migration: the home the migrated doc promotes to holds \
            a symbolic link",
    state: &[
        SETUP,
        Step::File(
            Place::Repo("notes/old.md"),
            "# Old\n\nSome text that is long enough to be worth migrating, really.\n",
        ),
        Step::Git(&["add", "notes/old.md"]),
        Step::Git(&["commit", "-q", "-m", "an old note"]),
        Step::Jigc(&["migrate", "notes/old.md", "--as", "adr"]),
        AN_ADR_PAYLOAD,
        Step::Jigc(&[
            "doc",
            "author",
            "adr",
            "--from-file",
            "../outside/adr.yaml",
            "--task",
            THE_MIGRATION,
        ]),
        THE_MIGRATIONS_COMMIT_DOC[0],
        THE_MIGRATIONS_COMMIT_DOC[1],
        THE_MIGRATIONS_COMMIT_DOC[2],
        A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[0],
        A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[1],
    ],
    refusing: &["task", "finalize", THE_MIGRATION],
    acts: &[
        Act::MoveLinkOut(Operand::Span(2)),
        Act::RunCited(Cited {
            line: 3,
            promised: THE_FIDELITY_DIFF,
        }),
        Act::Run(4),
    ],
    holds: &[Check::RegularFile(Place::Repo(THE_ADR))],
    ..H
}];

// ---------------------------------------------------------------------------
// The milestone's doors — a sub-task's doc at the boundary, the record's home at the
// create, and the record's home at the four doors that rewrite it
// ---------------------------------------------------------------------------

const RECORD: &str = "docs/milestone-records/fan-out.md";
const A_MILESTONE: Step = Step::Jigc(&["milestone", "create", "Fan out"]);
const FINALIZE_THE_MILESTONE: &[&str] = &["milestone", "finalize", "fan-out"];

/// A milestone whose one sub-task, added under `record-decision`, authored its ADR.
macro_rules! a_sub_tasks_adr {
    ($($plant:expr),* $(,)?) => {
        &[
            SETUP,
            A_MILESTONE,
            Step::Jigc(&[
                "milestone",
                "add-task",
                "fan-out",
                "do a thing",
                "--workflow",
                "record-decision",
            ]),
            Step::Jigc(&["milestone", "provision", "fan-out"]),
            AN_ADR_PAYLOAD,
            Step::Jigc(&[
                "doc",
                "author",
                "adr",
                "--from-file",
                "../outside/adr.yaml",
                "--task",
                "do-a-thing",
            ]),
            $($plant,)*
        ]
    };
}
const A_SUB_TASKS_DOC_OVER_A_LINK: &str = "`jigc milestone finalize`: the home of a doc a \
    sub-task minted holds a symbolic link";
const A_SUB_TASKS_MINTED_DOC_OVER_A_LINK: [Case; 2] = [
    Case {
        name: "home-shape-milestone-minted-over-a-link-renamed-and-finalized-as-printed",
        cause: A_SUB_TASKS_DOC_OVER_A_LINK,
        state: a_sub_tasks_adr![
            A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[0],
            A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[1],
        ],
        refusing: FINALIZE_THE_MILESTONE,
        acts: &[Act::Run(1), Act::Run(4)],
        holds: &[Check::RegularFile(Place::Repo(THE_ADR_RETITLED))],
        ..H
    },
    Case {
        name: "home-shape-milestone-minted-over-a-link-link-moved-out-and-finalized-as-printed",
        cause: A_SUB_TASKS_DOC_OVER_A_LINK,
        state: a_sub_tasks_adr![
            A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[0],
            A_LINK_TO_NOTHING_AT_THE_ADRS_HOME[1],
        ],
        refusing: FINALIZE_THE_MILESTONE,
        acts: &[Act::MoveLinkOut(Operand::Span(2)), Act::Run(4)],
        holds: &[Check::RegularFile(Place::Repo(THE_ADR))],
        ..H
    },
];

/// A milestone whose one sub-task, added under `decided-task`, minted the decisions log —
/// a doc under a fixed identity — and a link at the log's home.
const A_SUB_TASKS_FIXED_IDENTITY_OVER_A_LINK: [Case; 1] = [Case {
    name: "home-shape-milestone-fixed-identity-over-a-link-link-moved-out-and-finalized-as-printed",
    cause: "`jigc milestone finalize`: the home of a doc under a fixed identity, which a \
            sub-task minted, holds a symbolic link",
    state: &[
        SETUP,
        A_MILESTONE,
        Step::Jigc(&[
            "milestone",
            "add-task",
            "fan-out",
            "do a thing",
            "--workflow",
            "decided-task",
        ]),
        Step::Jigc(&["milestone", "provision", "fan-out"]),
        Step::Jigc(&[
            "doc",
            "create",
            "decisions-log",
            "--title",
            "Decisions Log",
            "--task",
            "do-a-thing",
        ]),
        Step::Dir(Place::Repo("docs")),
        Step::LinkTo(Place::Repo(THE_LOG), "nowhere"),
    ],
    refusing: FINALIZE_THE_MILESTONE,
    acts: &[Act::MoveLinkOut(Operand::Span(1)), Act::Run(3)],
    holds: &[Check::RegularFile(Place::Repo(THE_LOG))],
    ..H
}];
/// A milestone whose one sub-task edits the committed ADR, and the ADR's home turned into
/// a link to a copy of it.
const A_SUB_TASKS_EDITED_DOC_OVER_A_LINK_TO_A_COPY: [Case; 1] = [Case {
    name: "home-shape-milestone-edited-over-a-link-to-a-copy-the-file-itself-put-there-and-finalized-as-printed",
    cause: "`jigc milestone finalize`: the home of a committed doc a sub-task edits was \
            turned into a symbolic link to a copy of the doc",
    state: an_adr_committed![
        A_MILESTONE,
        Step::Jigc(&[
            "milestone",
            "add-task",
            "fan-out",
            "do a thing",
            "--workflow",
            "record-decision",
        ]),
        Step::Jigc(&["milestone", "provision", "fan-out"]),
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
        THE_ADRS_HOME_NOW_A_LINK_TO_A_COPY[0],
        THE_ADRS_HOME_NOW_A_LINK_TO_A_COPY[1],
    ],
    refusing: FINALIZE_THE_MILESTONE,
    acts: &[Act::PutTheFileItself(Operand::Span(1)), Act::Run(3)],
    holds: &[
        Check::RegularFile(Place::Repo(THE_ADR)),
        Check::Contains(Place::Repo(THE_ADR), "More."),
    ],
    witnesses: &["copy.md"],
    ..H
}];

const THE_RECORDS_HOME_TAKEN_AT_THE_CREATE: [Case; 2] = [
    Case {
        name: "home-shape-milestone-create-over-a-link-link-moved-out-and-created-again",
        cause: "`jigc milestone create`: the record's home holds a symbolic link",
        state: &[
            SETUP,
            Step::Dir(Place::Repo("docs/milestone-records")),
            Step::LinkTo(Place::Repo(RECORD), "nowhere"),
        ],
        refusing: &["milestone", "create", "Fan out"],
        acts: &[Act::MoveLinkOut(Operand::Span(1)), Act::Rerun],
        holds: &[Check::RegularFile(Place::Repo(RECORD))],
        ..H
    },
    Case {
        name: "home-shape-milestone-create-over-a-directory-moved-away-and-created-again",
        cause: "`jigc milestone create`: the record's home holds a directory",
        state: &[SETUP, Step::Dir(Place::Repo(RECORD))],
        refusing: &["milestone", "create", "Fan out"],
        acts: &[Act::MoveAway(Operand::Span(1)), Act::Rerun],
        holds: &[Check::RegularFile(Place::Repo(RECORD))],
        ..H
    },
];

/// A milestone of one sub-task, and its record's home turned into a link to a copy of the
/// record outside the repository — in no commit: `HEAD` still holds the record as a file.
const A_SUB_TASK: Step = Step::Jigc(&["milestone", "add-task", "fan-out", "do a thing"]);
const THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY: [Step; 2] = [
    sh!("mv docs/milestone-records/fan-out.md ../outside/record.md"),
    Step::Link(Place::Repo(RECORD), Place::Outside("record.md")),
];
/// **A door that rewrites the record, over a record whose home is a link `HEAD` does not
/// hold**: the printed `git checkout` puts the record back, and the refused command is run
/// again — the route prints no line for it.
macro_rules! record_door {
    ($name:literal, $cause:literal, state: $state:expr, refusing: $refusing:expr $(, holds: $holds:expr)? $(,)?) => {
        [Case {
            name: concat!("home-shape-record-", $name, "-restored-from-head-as-printed-and-run-again"),
            cause: $cause,
            state: $state,
            refusing: $refusing,
            acts: &[Act::Run(1), Act::Rerun],
            $(holds: $holds,)?
            witnesses: &["record.md"],
            ..H
        }]
    };
}
const THE_RECORD_A_LINK_AT_ADD_TASK: [Case; 1] = record_door!(
    "a-link-at-add-task",
    "`jigc milestone add-task`: the record's home was turned into a symbolic link, in no \
     commit",
    state: &[
        SETUP,
        A_MILESTONE,
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[0],
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[1],
    ],
    refusing: &["milestone", "add-task", "fan-out", "do a thing"],
    holds: &[
        Check::RegularFile(Place::Repo(RECORD)),
        Check::Contains(Place::Repo(RECORD), "do-a-thing"),
    ],
);
const THE_RECORD_A_LINK_AT_THE_DISCARD: [Case; 1] = record_door!(
    "a-link-at-the-milestones-discard",
    "`jigc milestone discard`: the record's home was turned into a symbolic link, in no \
     commit",
    state: &[
        SETUP,
        A_MILESTONE,
        A_SUB_TASK,
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[0],
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[1],
    ],
    refusing: &["milestone", "discard", "fan-out"],
    holds: &[
        Check::RegularFile(Place::Repo(RECORD)),
        Check::Contains(Place::Repo(RECORD), "discarded"),
    ],
);
const THE_RECORD_A_LINK_AT_A_SUB_TASKS_DISCARD: [Case; 1] = record_door!(
    "a-link-at-a-sub-tasks-discard",
    "`jigc task discard` of a sub-task: the record's home was turned into a symbolic link, \
     in no commit",
    state: &[
        SETUP,
        A_MILESTONE,
        A_SUB_TASK,
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[0],
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[1],
    ],
    refusing: &["task", "discard", "do-a-thing", "--force"],
    holds: &[
        Check::RegularFile(Place::Repo(RECORD)),
        Check::Absent(Place::Repo(".jigc/tasks/do-a-thing")),
    ],
);
const THE_RECORD_A_LINK_AT_THE_FINALIZE: [Case; 1] = record_door!(
    "a-link-at-the-milestones-finalize",
    "`jigc milestone finalize`: the record's home was turned into a symbolic link, in no \
     commit",
    state: &[
        SETUP,
        A_MILESTONE,
        A_SUB_TASK,
        Step::Jigc(&["milestone", "provision", "fan-out"]),
        Step::File(Place::Repo(".jigc/worktrees/do-a-thing/work.txt"), "work\n"),
        Step::Git(&["-C", ".jigc/worktrees/do-a-thing", "add", "work.txt"]),
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[0],
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[1],
    ],
    refusing: FINALIZE_THE_MILESTONE,
    holds: &[
        Check::RegularFile(Place::Repo(RECORD)),
        Check::RegularFile(Place::Repo("work.txt")),
    ],
);
const THE_RECORD_A_COMMITTED_LINK: [Case; 1] = [Case {
    name: "home-shape-record-a-committed-link-at-add-task-the-file-itself-put-there-committed-and-run-again",
    cause: "`jigc milestone add-task`: the record's home is a symbolic link, and the link \
            itself was committed",
    state: &[
        SETUP,
        A_MILESTONE,
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[0],
        THE_RECORDS_HOME_NOW_A_LINK_TO_A_COPY[1],
        THAT_COMMITTED[0],
        THAT_COMMITTED[1],
    ],
    refusing: &["milestone", "add-task", "fan-out", "do a thing"],
    acts: &[
        Act::PutTheFileItself(Operand::Span(0)),
        Act::Commit(Operand::Span(0)),
        Act::Rerun,
    ],
    holds: &[
        Check::RegularFile(Place::Repo(RECORD)),
        Check::Contains(Place::Repo(RECORD), "do-a-thing"),
    ],
    witnesses: &["record.md"],
    ..H
}];

// ---------------------------------------------------------------------------
// `jigc rename` — the doc's own home, a referrer's, and the destination
// ---------------------------------------------------------------------------

const RENAME: &[&str] = &["rename", "adr:use-tabs", "--to", "Use spaces"];
const THE_DOCS_OWN_HOME_AT_THE_RENAME: [Case; 1] = [Case {
    name: "home-shape-rename-own-home-a-link-the-file-itself-put-there-committed-and-renamed-as-printed",
    cause: "`jigc rename`: the home of the doc to rename is a committed symbolic link to a \
            copy of it",
    state: an_adr_committed![
        THE_ADRS_HOME_NOW_A_LINK_TO_A_COPY[0],
        THE_ADRS_HOME_NOW_A_LINK_TO_A_COPY[1],
        THAT_COMMITTED[0],
        THAT_COMMITTED[1],
    ],
    refusing: RENAME,
    acts: &[
        Act::PutTheFileItself(Operand::Span(0)),
        Act::Commit(Operand::Span(0)),
        Act::Run(3),
    ],
    holds: &[
        Check::RegularFile(Place::Repo(THE_ADR_AS_USE_SPACES)),
        Check::Absent(Place::Repo(THE_ADR)),
    ],
    witnesses: &["copy.md"],
    ..H
}];
/// A second ADR that supersedes the first, committed by a second task — a referrer the
/// rename repoints — and its home.
const NEXT_COMMIT_DOC: [Step; 3] = commit_doc!("next");
const THE_REFERRER: &str = "docs/decisions/use-spaces-now.md";
const A_REFERRERS_HOME_AT_THE_RENAME: [Case; 1] = [Case {
    name: "home-shape-rename-a-referrers-home-a-link-the-file-itself-put-there-committed-and-renamed-as-printed",
    cause: "`jigc rename`: the home of a doc that refers to the one to rename is a committed \
            symbolic link to a copy of it",
    state: an_adr_committed![
        Step::Jigc(&["start", "--workflow", "record-decision", "next"]),
        A_SUPERSEDING_PAYLOAD,
        Step::Jigc(&[
            "doc",
            "author",
            "adr",
            "--from-file",
            "../outside/next.yaml",
            "--task",
            "next",
        ]),
        NEXT_COMMIT_DOC[0],
        NEXT_COMMIT_DOC[1],
        NEXT_COMMIT_DOC[2],
        Step::Jigc(&["task", "finalize", "next"]),
        sh!("mv docs/decisions/use-spaces-now.md ../outside/copy.md"),
        Step::Link(Place::Repo(THE_REFERRER), Place::Outside("copy.md")),
        THAT_COMMITTED[0],
        THAT_COMMITTED[1],
    ],
    refusing: RENAME,
    acts: &[
        Act::PutTheFileItself(Operand::Span(0)),
        Act::Commit(Operand::Span(0)),
        Act::Run(3),
    ],
    holds: &[
        Check::RegularFile(Place::Repo(THE_ADR_AS_USE_SPACES)),
        Check::Contains(Place::Repo(THE_REFERRER), "adr:use-spaces"),
    ],
    witnesses: &["copy.md"],
    ..H
}];

/// The two readings of a printed line whose placeholder stands for a slug of the reader's
/// own choosing.
const ANOTHER_SLUG_IS_THE_READERS: &str = "*give this doc an id whose home is free — re-run \
    `… --slug <other-slug>`*: (a) the backticked line is a command line to run as printed, \
    like every `jigc` line a route prints — a bare word in angle brackets is a redirection \
    to a shell, and the line cannot be split; (b) it is a shape the reader fills with a slug \
    of their own — then the value is the reader's, the frame fills a placeholder only with a \
    value the refusal lists, and it has no case. The case follows (a). The surface contract \
    says *a `<placeholder>` the reader fills is legal bare or as a quoted span* (the span \
    fence); it is what the pilot's first question left open, over a value nobody but the \
    reader has";
const A_LINK_AT_THE_RENAMES_DESTINATION: &str = "`jigc rename`: the home of the id the doc is \
    renamed to holds a symbolic link, in no commit";
const THE_DESTINATION_A_LINK_AT_THE_RENAME: [Case; 2] = [
    Case {
        name: "home-shape-rename-destination-a-link-another-slug-line-run-as-printed",
        cause: A_LINK_AT_THE_RENAMES_DESTINATION,
        state: an_adr_committed![Step::LinkTo(Place::Repo(THE_ADR_AS_USE_SPACES), "nowhere")],
        refusing: RENAME,
        acts: &[Act::Run(0)],
        untouched: &[THE_ADR],
        doubt: Some(ANOTHER_SLUG_IS_THE_READERS),
        ..H
    },
    Case {
        name: "home-shape-rename-destination-a-link-link-moved-out-and-renamed-as-printed",
        cause: A_LINK_AT_THE_RENAMES_DESTINATION,
        state: an_adr_committed![Step::LinkTo(Place::Repo(THE_ADR_AS_USE_SPACES), "nowhere")],
        refusing: RENAME,
        acts: &[Act::MoveLinkOut(Operand::Span(1)), Act::Run(2)],
        holds: &[
            Check::RegularFile(Place::Repo(THE_ADR_AS_USE_SPACES)),
            Check::Absent(Place::Repo(THE_ADR)),
        ],
        ..H
    },
];
const THE_DESTINATION_A_COMMITTED_LINK_AT_THE_RENAME: [Case; 1] = [Case {
    name: "home-shape-rename-destination-a-committed-link-moved-out-committed-and-renamed-as-printed",
    cause: "`jigc rename`: the home of the id the doc is renamed to holds a symbolic link \
            that git tracks",
    state: an_adr_committed![
        Step::LinkTo(Place::Repo(THE_ADR_AS_USE_SPACES), "nowhere"),
        THAT_COMMITTED[0],
        THAT_COMMITTED[1],
    ],
    refusing: RENAME,
    acts: &[
        Act::MoveLinkOut(Operand::Span(1)),
        Act::Commit(Operand::Span(1)),
        Act::Run(2),
    ],
    holds: &[
        Check::RegularFile(Place::Repo(THE_ADR_AS_USE_SPACES)),
        Check::Absent(Place::Repo(THE_ADR)),
    ],
    ..H
}];
const THE_DESTINATION_A_DIRECTORY_AT_THE_RENAME: [Case; 1] = [Case {
    name: "home-shape-rename-destination-a-directory-moved-away-and-renamed-as-printed",
    cause: "`jigc rename`: the home of the id the doc is renamed to holds a directory",
    state: an_adr_committed![Step::Dir(Place::Repo(THE_ADR_AS_USE_SPACES))],
    refusing: RENAME,
    acts: &[Act::MoveAway(Operand::Span(1)), Act::Run(2)],
    holds: &[
        Check::RegularFile(Place::Repo(THE_ADR_AS_USE_SPACES)),
        Check::Absent(Place::Repo(THE_ADR)),
    ],
    ..H
}];

// ---------------------------------------------------------------------------
// The in-task doors — `jigc doc rename`'s destination, and a create under an entry that
// only creates
// ---------------------------------------------------------------------------

const A_LINK_AT_THE_IN_TASK_RENAMES_DESTINATION: &str = "`jigc doc rename`: the home of the id \
    the task's doc is renamed to holds a symbolic link";
const THE_DESTINATION_A_LINK_AT_THE_IN_TASK_RENAME: [Case; 2] = [
    Case {
        name: "home-shape-doc-rename-destination-a-link-another-slug-line-run-as-printed",
        cause: A_LINK_AT_THE_IN_TASK_RENAMES_DESTINATION,
        state: an_adr_ready![
            Step::Dir(Place::Repo("docs/decisions")),
            Step::LinkTo(Place::Repo(THE_ADR_AS_USE_SPACES), "nowhere"),
        ],
        refusing: &[
            "doc",
            "rename",
            "adr:use-tabs",
            "--to",
            "Use spaces",
            "--task",
            "pick-tabs",
        ],
        acts: &[Act::Run(0)],
        doubt: Some(ANOTHER_SLUG_IS_THE_READERS),
        ..H
    },
    Case {
        name: "home-shape-doc-rename-destination-a-link-link-moved-out",
        cause: A_LINK_AT_THE_IN_TASK_RENAMES_DESTINATION,
        state: an_adr_ready![
            Step::Dir(Place::Repo("docs/decisions")),
            Step::LinkTo(Place::Repo(THE_ADR_AS_USE_SPACES), "nowhere"),
        ],
        refusing: &[
            "doc",
            "rename",
            "adr:use-tabs",
            "--to",
            "Use spaces",
            "--task",
            "pick-tabs",
        ],
        acts: &[Act::MoveLinkOut(Operand::Span(1))],
        holds: &[Check::Absent(Place::Repo(THE_ADR_AS_USE_SPACES))],
        ..H
    },
];

/// The two readings of a printed line whose placeholder stands for a value that is the
/// reader's own to choose — the question of `doc_write.rs`, at the same function's route.
const A_VALUE_OF_THE_READERS_OWN: &str = "the route prints the corrected command with a word in \
    angle brackets where the reader's own choice goes — `--title <title> --slug <slug>`, \
    `--from-file <payload>` — over a refusal that lists no such value and could list none: (a) \
    the backticked line is a command line to run as printed — a bare word in angle brackets \
    is a redirection to a shell, and the line cannot be split; (b) it is a shape the reader \
    fills with a value of their own — then the frame has no case. The case follows (a). The \
    surface contract says *a `<placeholder>` the reader fills is legal bare or as a quoted \
    span* (the span fence); it is what the pilot's first question left open, over a value \
    nobody but the reader has";
const THE_FINDINGS_HOME: &str = "docs/jigc-feedback/slow-start.md";
const A_CREATE_ONLY_ENTRY_OVER_A_LINK: [Case; 2] = [
    Case {
        name: "home-shape-create-only-create-over-a-link-line-run-as-printed",
        cause: "`jigc doc create` under an entry that only creates: the home of the id it \
                mints holds a symbolic link",
        state: &[
            SETUP,
            Step::Jigc(&["start", "--workflow", "report-jigc-feedback", "report a bug"]),
            Step::Dir(Place::Repo("docs/jigc-feedback")),
            Step::LinkTo(Place::Repo(THE_FINDINGS_HOME), "nowhere"),
        ],
        refusing: &[
            "doc",
            "create",
            "jigc-feedback",
            "--title",
            "Slow start",
            "--task",
            "report-a-bug",
        ],
        acts: &[Act::Run(2)],
        doubt: Some(A_VALUE_OF_THE_READERS_OWN),
        ..H
    },
    Case {
        name: "home-shape-create-only-author-over-a-link-line-run-as-printed",
        cause: "`jigc doc author` under an entry that only creates: the home of the id it \
                mints holds a symbolic link",
        state: &[
            SETUP,
            Step::Jigc(&["start", "--workflow", "report-jigc-feedback", "report a bug"]),
            Step::Dir(Place::Repo("docs/jigc-feedback")),
            Step::LinkTo(Place::Repo(THE_FINDINGS_HOME), "nowhere"),
            Step::File(Place::Outside("finding.yaml"), "title: \"Slow start\"\n"),
        ],
        refusing: &[
            "doc",
            "author",
            "jigc-feedback",
            "--from-file",
            "../outside/finding.yaml",
            "--task",
            "report-a-bug",
        ],
        acts: &[Act::Run(1)],
        doubt: Some(A_VALUE_OF_THE_READERS_OWN),
        ..H
    },
];

/// The cases of the family, a group a cause — and, within a group, a case a way out of the
/// cause's route.
pub const CASES: &[&[Case]] = &[
    &MINTED_OVER_A_LINK_TO_NOTHING,
    &MINTED_OVER_A_LINK_TO_A_FILE,
    &MINTED_OVER_A_DIRECTORY,
    &A_FIXED_IDENTITY_OVER_A_LINK,
    &AN_EDITED_DOC_OVER_A_LINK_TO_A_COPY,
    &AN_EDITED_DOC_OVER_A_LINK_TO_NOTHING,
    &A_MIGRATIONS_DOC_OVER_A_LINK,
    &A_SUB_TASKS_MINTED_DOC_OVER_A_LINK,
    &A_SUB_TASKS_FIXED_IDENTITY_OVER_A_LINK,
    &A_SUB_TASKS_EDITED_DOC_OVER_A_LINK_TO_A_COPY,
    &THE_RECORDS_HOME_TAKEN_AT_THE_CREATE,
    &THE_RECORD_A_LINK_AT_ADD_TASK,
    &THE_RECORD_A_LINK_AT_THE_DISCARD,
    &THE_RECORD_A_LINK_AT_A_SUB_TASKS_DISCARD,
    &THE_RECORD_A_LINK_AT_THE_FINALIZE,
    &THE_RECORD_A_COMMITTED_LINK,
    &THE_DOCS_OWN_HOME_AT_THE_RENAME,
    &A_REFERRERS_HOME_AT_THE_RENAME,
    &THE_DESTINATION_A_LINK_AT_THE_RENAME,
    &THE_DESTINATION_A_COMMITTED_LINK_AT_THE_RENAME,
    &THE_DESTINATION_A_DIRECTORY_AT_THE_RENAME,
    &THE_DESTINATION_A_LINK_AT_THE_IN_TASK_RENAME,
    &A_CREATE_ONLY_ENTRY_OVER_A_LINK,
];

/// The sites of the family that have no case, each with why: not the one.
pub const REASONS: &[Reason] = &[];

/// A way that ends in the refused command, where the route prints a command line.
const THE_REFUSED_COMMAND_AGAIN: &str = "the re-run is the refused command, and the frame runs \
    a refusing command again only where the route prints no command line of the product — \
    this one prints the rename under another slug";

/// **What was read for the site, and how many causes it holds** — a hand count, with its
/// derivation; the causes that have no case, each with why; and the ways out of its route
/// that are not followed, each with why.
pub const READINGS: &[Reading] = &[Reading {
    site: HOME_SHAPE,
    read: &[
        "crates/engine/src/store.rs::home_shape_refusal, home_entry, ForeignEntry — the one constructor: the code, the key, the sentence; three shapes — a symbolic link wherever it leads, a directory, a special file",
        "crates/engine/src/finalize.rs::shape_refusal, store_home_refusal, promote_sink_refusal — the committing doors' wording, by unit (a task, a migration, a milestone, the promote's sink, a door of the store) and by exit (rename or move out, move out, the regular file itself)",
        "crates/cli/src/rename.rs::foreign_home, foreign_destination — `jigc rename`: the doc's own home, a referrer's, the destination",
        "crates/cli/src/relocate.rs::refuse_foreign_source — the move primitive, behind `jigc relocate` and both root knobs",
        "crates/cli/src/doc.rs::free_destination, already_exists_refusal with crates/engine/src/state.rs::create_only_home_refusal — the in-task rename's destination, and a create under an entry that only creates",
        "crates/cli/src/milestone.rs::record_shape_block and crates/engine/src/milestone.rs — the five doors that rewrite the milestone's record, and `jigc milestone create`",
        "crates/cli/src/migrate_corpus.rs::queue_or_refuse, destination_collision_finding — `jigc migrate-corpus`: a doc's own home, a relocation's destination",
        "crates/cli/tests/home_shape_one_code.rs — the product's own registry of the constructor's callers and the commands that reach each, as a lead",
    ],
    causes: 32,
    derivation: "a cause is a door and what it met. The task's finalize: a doc it minted \
                 over a link to nothing, a link to a file, a directory; a doc under a fixed \
                 identity over a link; a committed doc it edits over a link to a copy, a \
                 link to nothing, a directory; a migration's doc over a link — 8. The \
                 milestone's finalize over a sub-task's doc: minted, under a fixed identity, \
                 a committed one it edits — 3; the promote's sink, \
                 an entry that appears behind the plan — 1. `jigc milestone create` over a \
                 link, a directory — 2. The record's doors, a link in no commit at each of \
                 five, and a committed link — 6. `jigc rename`: the doc's own home, a \
                 referrer's, the destination a link in no commit, a committed one, a \
                 directory — 5. `jigc doc rename`'s destination — 1. A create and an author \
                 under an entry that only creates — 2. The two root knobs and `jigc \
                 relocate`, where the move primitive refuses — 2. `jigc migrate-corpus`, a \
                 doc's own home and a relocation's destination — 2. And a special file at \
                 any of them, which the constructor words as a third shape — counted with \
                 the door, never constructed: a declared bound",
    without_case: &[
        (
            "the task's finalize: a committed doc it edits, over a directory",
            Why::NotConstructible(
                "the route's one way is *put the regular file itself there — for a link, \
                 a copy of the file it points at*, and over a directory it names no \
                 source for the file: it is the reader's to bring back, and the frame has \
                 no act that writes a file of a case's own",
            ),
        ),
        (
            "the promote's sink: an entry that appears at the home behind the plan",
            Why::NotConstructible(
                "the planner asks the home and the sink asks it again inside one command: \
                 a state has no step that writes between them",
            ),
        ),
        (
            "the record's home a link at `jigc milestone add-from-spec`",
            Why::NotConstructible(
                "the door shares the preflight the four doors with a case go through and \
                 needs a committed spec with a criterion to seed from: the state was not \
                 built — the product's own suite drives the door \
                 (`home_shape_one_code.rs`)",
            ),
        ),
        (
            "`jigc config set docs-root` · `placement-root`: the home of a doc the re-point moves",
            Why::NotConstructible(
                "the refusal reaches its reader inside the message of \
                 `config.repoint-failed`, and the frame reads a refusal by the finding \
                 that carries its code. Both are followed at that site, in `config.rs`",
            ),
        ),
        (
            "`jigc relocate`: the home of a doc it moves",
            Why::NotConstructible(
                "the door moves the instances of a doctype no manifest governs, and every \
                 shipped doctype is governed by one: it takes a pack of the reader's own",
            ),
        ),
        (
            "`jigc migrate-corpus`: the home of a doc it would rewrite",
            Why::NotConstructible(
                "the door prints the refusal as a row of its report — `blocked`, then the \
                 code and a colon — and not as a finding the frame reads a route from. \
                 Driven by hand over an unstamped ADR behind a committed link: exit 1, \
                 the row carries the code and the store's route",
            ),
        ),
        (
            "`jigc migrate-corpus`: a relocation's destination",
            Why::NotConstructible(
                "it takes a doctype whose home moved at a version bump, which no shipped \
                 schema has pending: a pack of the reader's own",
            ),
        ),
    ],
    not_followed: &[
        (
            "*and re-run this rename* — `jigc doc rename`'s destination, behind the link moved out",
            Why::NotConstructible(THE_REFUSED_COMMAND_AGAIN),
        ),
        (
            "*create this milestone under a different title*",
            Why::NotConstructible(
                "the title is the reader's own and an argument of the refused command, \
                 which the frame runs again only as it was typed",
            ),
        ),
    ],
}];
