//! The route catalog's cases for the family the list of refusal sites calls `doc write`:
//! the refusals of the `jigc doc` verbs that write — `create`, `author`, `rename`,
//! `retitle-item` and the leaf writes. The family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here** is not the family: it is the three sites of it that the fix pass
//! since the previous release touched, as the human ruled on 2026-10-11 (`DECISIONS.md` →
//! *Which families the route catalog takes in*) — the rest are rows of the scheduled list,
//! ten of them with the change of a caller that was read and why it is off the path. The
//! three: a retitle of an item of the milestone's record; a create whose title would
//! never be written, because the id it mints already has a body — the task's own staged
//! doc, or a committed one; and a create under an entry that only creates, over a home
//! that is taken.
//!
//! **Every case starts from an install and an open task made by the binary under test** —
//! `jigc setup`, `jigc start --workflow …` — and a doc that is already there is authored
//! through it, whole in one payload (`jigc doc author`) or by `jigc doc create`.
//!
//! **A route of two of these sites prints the corrected command with the value left to
//! its reader** — `jigc doc create <type> --title <title> --slug <slug> --task <id>`,
//! `jigc doc author <type> --from-file <payload> --task <id>`. A distinct title, another
//! slug and a payload file are the reader's own to choose: no refusal can list them.
//! **Such a line is followed with the placeholder filled** — the human's ruling of
//! 2026-10-12 (`DECISIONS.md` → *A printed line that carries a placeholder for a value of
//! the reader's own is followed with the placeholder filled*): the case names the value a
//! reader would choose in its state, inside what the route's words bound it by, and says
//! why ([`Own`]); the filled line is held to exit 0, and what must hold afterwards is the
//! doc staged under the value that was named.

use super::{Reading, Reason, Why};
use crate::support::route_case::{Act, Case, Check, Chosen, Own, Place, Step};

// The sites, as the committed list of refusal sites names them.
const RETITLE_IN_THE_RECORD: &str =
    "crates/cli/src/doc.rs::run_retitle_item::write.machine-maintained#1";
const TITLE_IGNORED: &str = "crates/cli/src/doc.rs::title_ignored_refusal::write.title-ignored#1";
const ALREADY_EXISTS: &str =
    "crates/engine/src/state.rs::already_exists_finding::create.already-exists#1";

const SETUP: Step = Step::Jigc(&["setup"]);

/// What every case of the family shares: it stands at the repository's root.
const W: Case = Case {
    name: "",
    site: "",
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

// The values of the reader's own that the cases of this file name — each the value a
// reader would choose in the case's state, inside what the route's words bound it by.

/// A title the route lets its reader keep — *or keep this one and pass `--slug <slug>`* —
/// and one of a route that asks nothing of the title: *the id comes from `--slug …`, not
/// the title*. Either way it is the title of the refused command.
macro_rules! the_title_of_the_refused_command {
    ($title:literal, $why:literal) => {
        Own {
            placeholder: "<title>",
            value: $title,
            why: $why,
        }
    };
}
/// A slug nothing answers to, where the route asks for one to mint beside what is there,
/// or for a distinct one.
macro_rules! a_free_slug {
    ($slug:literal, $why:literal) => {
        Own {
            placeholder: "<slug>",
            value: $slug,
            why: $why,
        }
    };
}
/// The reader's payload with its `title:` set to a distinct title, as the route asks: a
/// payload file of the state, by its path from where the case stands. The route says
/// *set the payload's `title:` … and re-run the same* command; a reader edits the payload
/// or writes another, and the command is handed a payload either way — the frame has no
/// act that edits a file, so the state holds the second one from the start.
const THE_PAYLOAD_UNDER_A_DISTINCT_TITLE: &str = "../outside/distinct.yaml";

// ---------------------------------------------------------------------------
// `run_retitle_item` · `write.machine-maintained` — a retitle of an item of the
// milestone's record
// ---------------------------------------------------------------------------

/// The milestone's committed record, from the repository root.
const RECORD: &str = "docs/milestone-records/fan-out.md";

/// An install and a milestone of one sub-task — whose record holds the item
/// `tasks/do-a-thing`.
const A_MILESTONE_OF_ONE: [Step; 3] = [
    SETUP,
    Step::Jigc(&["milestone", "create", "Fan out"]),
    Step::Jigc(&["milestone", "add-task", "fan-out", "do a thing"]),
];

/// The two readings of a product command a route names without the operands it requires.
const WHICH_VERBS: &str = "*leave the record to the milestone verbs — `jigc milestone add-task` \
    appends sub-tasks and `jigc milestone finalize` advances their status; no manual retitle \
    applies*: (a) the two backticked spans are command lines to run as printed, like every \
    `jigc` line a route prints — neither has the milestone's id, which both commands require; \
    (b) they name the two verbs that maintain the record, and the route names no act of the \
    reader's at all — then there is nothing to follow, and the frame has no case: it refuses \
    a case that follows a printed route with no act. The case follows (a). The surface \
    contract says of a `git` span with no operand that it *names a command rather than a \
    file* (the printed-path fence), and of a `jigc` span nothing of the kind; it is the \
    gate's first question — a product command printed without its id";

/// **The record's retitle refusal, followed the one way the frame can follow it**: the
/// two verbs the route names, run as printed.
macro_rules! retitle {
    ($name:literal, $cause:literal, state: $state:expr, $address:literal, $task:literal $(,)?) => {
        [Case {
            name: concat!("record-retitle-", $name, "-the-milestone-verbs-run-as-printed"),
            site: RETITLE_IN_THE_RECORD,
            cause: $cause,
            state: $state,
            refusing: &[
                "doc",
                "retitle-item",
                $address,
                "--title",
                "Another name",
                "--task",
                $task,
            ],
            acts: &[Act::Run(0), Act::Run(1)],
            untouched: &[RECORD],
            doubt: Some(WHICH_VERBS),
            ..W
        }]
    };
}
const RETITLED_FROM_THE_SUB_TASK: [Case; 1] = retitle!(
    "from-the-sub-task",
    "a sub-task of the milestone retitles its own item of the milestone's record",
    state: &A_MILESTONE_OF_ONE,
    "milestone-record:fan-out#tasks/do-a-thing",
    "do-a-thing",
);
const RETITLED_FROM_AN_ORDINARY_TASK: [Case; 1] = retitle!(
    "from-an-ordinary-task",
    "an ordinary task, no part of the milestone, retitles an item of the milestone's record",
    state: &[
        A_MILESTONE_OF_ONE[0],
        A_MILESTONE_OF_ONE[1],
        A_MILESTONE_OF_ONE[2],
        Step::Jigc(&["start", "--workflow", "dev-task", "write a note"]),
    ],
    "milestone-record:fan-out#tasks/do-a-thing",
    "write-a-note",
);
const AN_ITEM_THE_RECORD_DOES_NOT_HOLD: [Case; 1] = retitle!(
    "an-item-the-record-does-not-hold",
    "the addressed item is one the milestone's record does not hold: the refusal is by \
     doctype, ahead of the question whether the item is there",
    state: &A_MILESTONE_OF_ONE,
    "milestone-record:fan-out#tasks/no-such-task",
    "do-a-thing",
);

// ---------------------------------------------------------------------------
// `title_ignored_refusal` · `write.title-ignored` — a create whose title would never be
// written
// ---------------------------------------------------------------------------

/// A task that may create an ADR — its id slugs from the intent, `decide-tabs`.
const A_DECISION_TASK: Step = Step::Jigc(&["start", "--workflow", "record-decision", "decide tabs"]);
/// The staged ADR of that task, titled *Use tabs*, and the same under the slug `tabs`.
const THE_STAGED_ADR: &str = ".jigc/tasks/decide-tabs/docs/adr:use-tabs.md";
const THE_STAGED_ADR_UNDER_A_SLUG: &str = ".jigc/tasks/decide-tabs/docs/adr:tabs.md";

/// An ADR whole in one payload, by its title.
macro_rules! an_adr_titled {
    ($file:literal, $title:literal) => {
        Step::File(
            Place::Outside($file),
            concat!(
                "title: \"",
                $title,
                "\"\nsections:\n  - id: context\n    set:\n      context: |-\n        \
                 <<Because.>>\n  - id: decision\n    set:\n      decision: |-\n        \
                 <<Tabs.>>\n  - id: consequences\n    set:\n      consequences: |-\n        \
                 <<None.>>\n"
            ),
        )
    };
}
const THE_FIRST_PAYLOAD: Step = an_adr_titled!("adr.yaml", "Use tabs");
const ANOTHER_TITLE_THE_SAME_ID: Step = an_adr_titled!("again.yaml", "Use Tabs!");
const ANOTHER_TITLE: Step = an_adr_titled!("again.yaml", "Other");

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
const THE_FIRST_TASKS_COMMIT_DOC: [Step; 3] = commit_doc!("pick-tabs");

/// **The task's own staged doc already answers to the id**: the printed rename moves its
/// title, run as printed. The write the route then says to re-run is the refused command,
/// which the frame runs again only where a route prints no command line.
const STAGED_BY_A_CREATE: [Case; 1] = [Case {
    name: "title-ignored-staged-by-a-create-renamed-as-printed",
    site: TITLE_IGNORED,
    cause: "this task's `jigc doc create` already staged the doc, and a second create under \
            another title mints the same id",
    state: &[
        SETUP,
        A_DECISION_TASK,
        Step::Jigc(&["doc", "create", "adr", "--title", "Use tabs", "--task", "decide-tabs"]),
    ],
    refusing: &["doc", "create", "adr", "--title", "Use Tabs!", "--task", "decide-tabs"],
    acts: &[Act::Run(0)],
    holds: &[Check::Contains(Place::Repo(THE_STAGED_ADR), "# Use Tabs!")],
    ..W
}];
const STAGED_BY_A_CREATE_UNDER_A_SLUG: [Case; 1] = [Case {
    name: "title-ignored-staged-by-a-create-under-a-slug-renamed-as-printed",
    site: TITLE_IGNORED,
    cause: "this task's `jigc doc create --slug` already staged the doc, and a second create \
            under the same `--slug` carries another title",
    state: &[
        SETUP,
        A_DECISION_TASK,
        Step::Jigc(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Use tabs",
            "--slug",
            "tabs",
            "--task",
            "decide-tabs",
        ]),
    ],
    refusing: &[
        "doc",
        "create",
        "adr",
        "--title",
        "Other",
        "--slug",
        "tabs",
        "--task",
        "decide-tabs",
    ],
    acts: &[Act::Run(0)],
    holds: &[Check::Contains(
        Place::Repo(THE_STAGED_ADR_UNDER_A_SLUG),
        "# Other",
    )],
    ..W
}];
const STAGED_BY_AN_AUTHOR: [Case; 1] = [Case {
    name: "title-ignored-staged-by-an-author-renamed-as-printed",
    site: TITLE_IGNORED,
    cause: "this task's `jigc doc author` already staged the doc, and a second payload \
            carries another title that slugs to the same id",
    state: &[
        SETUP,
        A_DECISION_TASK,
        THE_FIRST_PAYLOAD,
        Step::Jigc(&[
            "doc",
            "author",
            "adr",
            "--from-file",
            "../outside/adr.yaml",
            "--task",
            "decide-tabs",
        ]),
        ANOTHER_TITLE_THE_SAME_ID,
    ],
    refusing: &[
        "doc",
        "author",
        "adr",
        "--from-file",
        "../outside/again.yaml",
        "--task",
        "decide-tabs",
    ],
    acts: &[Act::Run(0)],
    holds: &[Check::Contains(Place::Repo(THE_STAGED_ADR), "# Use Tabs!")],
    ..W
}];

/// A foreign file git holds, and its migration into an ADR under the recorded slug `tabs`
/// — the migration's task id is derived from the source's path.
const A_FOREIGN_NOTE_COMMITTED: [Step; 3] = [
    Step::File(Place::Repo("notes/old.md"), "# Old\n\nText.\n"),
    Step::Git(&["add", "notes/old.md"]),
    Step::Git(&["commit", "-q", "-m", "an old note"]),
];
const THE_NOTES_MIGRATION: &str = "migrate-adr-notes-old-e1a7d84cafc2";
const THE_MIGRATIONS_STAGED_ADR: &str =
    ".jigc/tasks/migrate-adr-notes-old-e1a7d84cafc2/docs/adr:tabs.md";
macro_rules! authored_in_the_migration {
    ($payload:literal) => {
        Step::Jigc(&[
            "doc",
            "author",
            "adr",
            "--from-file",
            concat!("../outside/", $payload),
            "--task",
            THE_NOTES_MIGRATION,
        ])
    };
}
const STAGED_BY_AN_AUTHOR_UNDER_A_MIGRATIONS_SLUG: [Case; 1] = [Case {
    name: "title-ignored-staged-under-a-migrations-slug-renamed-as-printed",
    site: TITLE_IGNORED,
    cause: "a migration recorded a `--slug`, its `jigc doc author` already staged the doc \
            under it, and a second payload carries another title",
    state: &[
        SETUP,
        A_FOREIGN_NOTE_COMMITTED[0],
        A_FOREIGN_NOTE_COMMITTED[1],
        A_FOREIGN_NOTE_COMMITTED[2],
        Step::Jigc(&["migrate", "notes/old.md", "--as", "adr", "--slug", "tabs"]),
        THE_FIRST_PAYLOAD,
        authored_in_the_migration!("adr.yaml"),
        ANOTHER_TITLE,
    ],
    refusing: &[
        "doc",
        "author",
        "adr",
        "--from-file",
        "../outside/again.yaml",
        "--task",
        THE_NOTES_MIGRATION,
    ],
    acts: &[Act::Run(0)],
    holds: &[Check::Contains(
        Place::Repo(THE_MIGRATIONS_STAGED_ADR),
        "# Other",
    )],
    ..W
}];

/// **An ADR titled *Use tabs*, committed by an earlier task, and a second task that may
/// create one** — its id slugs from the intent, `again`.
macro_rules! a_committed_adr {
    ($($plant:expr),* $(,)?) => {
        &[
            SETUP,
            Step::Jigc(&["start", "--workflow", "record-decision", "pick tabs"]),
            THE_FIRST_PAYLOAD,
            Step::Jigc(&[
                "doc",
                "author",
                "adr",
                "--from-file",
                "../outside/adr.yaml",
                "--task",
                "pick-tabs",
            ]),
            THE_FIRST_TASKS_COMMIT_DOC[0],
            THE_FIRST_TASKS_COMMIT_DOC[1],
            THE_FIRST_TASKS_COMMIT_DOC[2],
            Step::Jigc(&["task", "finalize", "pick-tabs"]),
            $($plant,)*
        ]
    };
}
const A_SECOND_TASK: Step = Step::Jigc(&["start", "--workflow", "record-decision", "again"]);
/// The committed ADR, from the repository root: no case's route may change it.
const THE_COMMITTED_ADR: &str = "docs/decisions/use-tabs.md";

/// The second task's staged ADR under each id a case below mints.
const THE_ADR_BESIDE_THE_COMMITTED_ONE: &str = ".jigc/tasks/again/docs/adr:use-tabs-again.md";
const THE_ADR_UNDER_A_DISTINCT_SLUG: &str = ".jigc/tasks/again/docs/adr:other.md";
const THE_ADR_UNDER_A_DISTINCT_TITLE: &str = ".jigc/tasks/again/docs/adr:use-spaces.md";

/// **A committed doc already answers to the id**: the route is at a distinct identity, and
/// its one command line leaves the title, the slug or the payload to its reader — the
/// printed line is followed with the reader's own value in each placeholder.
const COMMITTED_AND_A_CREATE: [Case; 1] = [Case {
    name: "title-ignored-committed-create-line-run-as-printed",
    site: TITLE_IGNORED,
    cause: "a committed doc answers to the id a `jigc doc create` mints from its title, and \
            the title differs from the doc's",
    state: a_committed_adr![A_SECOND_TASK],
    refusing: &["doc", "create", "adr", "--title", "Use Tabs!", "--task", "again"],
    acts: &[Act::RunChosen(Chosen {
        line: 2,
        own: &[
            the_title_of_the_refused_command!(
                "Use Tabs!",
                "the title the reader typed, kept: the route says *or keep this one and pass \
                 `--slug <slug>` to mint beside the existing doc*"
            ),
            a_free_slug!(
                "use-tabs-again",
                "a slug nothing answers to, to mint beside the existing doc: that doc is \
                 `adr:use-tabs`, and the title alone slugs to its id"
            ),
        ],
    })],
    holds: &[Check::Contains(
        Place::Repo(THE_ADR_BESIDE_THE_COMMITTED_ONE),
        "# Use Tabs!",
    )],
    untouched: &[THE_COMMITTED_ADR],
    ..W
}];
const COMMITTED_AND_A_CREATE_UNDER_A_SLUG: [Case; 1] = [Case {
    name: "title-ignored-committed-create-under-a-slug-line-run-as-printed",
    site: TITLE_IGNORED,
    cause: "a committed doc answers to the id a `jigc doc create --slug` names, and the \
            title differs from the doc's",
    state: a_committed_adr![A_SECOND_TASK],
    refusing: &[
        "doc",
        "create",
        "adr",
        "--title",
        "Other",
        "--slug",
        "use-tabs",
        "--task",
        "again",
    ],
    acts: &[Act::RunChosen(Chosen {
        line: 2,
        own: &[
            the_title_of_the_refused_command!(
                "Other",
                "the title the reader typed: the route says the id comes from the slug and \
                 not from the title, and asks nothing of the title"
            ),
            a_free_slug!(
                "other",
                "a distinct slug, as the route asks: other than `use-tabs`, which the \
                 committed doc answers to, and nothing answers to it — it is what the title \
                 slugs to"
            ),
        ],
    })],
    holds: &[Check::Contains(
        Place::Repo(THE_ADR_UNDER_A_DISTINCT_SLUG),
        "# Other",
    )],
    untouched: &[THE_COMMITTED_ADR],
    ..W
}];
const COMMITTED_AND_AN_AUTHOR: [Case; 1] = [Case {
    name: "title-ignored-committed-author-line-run-as-printed",
    site: TITLE_IGNORED,
    cause: "a committed doc answers to the id a `jigc doc author` mints from its payload's \
            title, and the title differs from the doc's",
    state: a_committed_adr![
        A_SECOND_TASK,
        ANOTHER_TITLE_THE_SAME_ID,
        an_adr_titled!("distinct.yaml", "Use spaces"),
    ],
    refusing: &[
        "doc",
        "author",
        "adr",
        "--from-file",
        "../outside/again.yaml",
        "--task",
        "again",
    ],
    acts: &[Act::RunChosen(Chosen {
        line: 1,
        own: &[Own {
            placeholder: "<payload>",
            value: THE_PAYLOAD_UNDER_A_DISTINCT_TITLE,
            why: "a payload that is one — an ADR, whole — with its `title:` set to a \
                  distinct title, as the route asks: *Use spaces*, whose slug nothing \
                  answers to",
        }],
    })],
    holds: &[Check::Contains(
        Place::Repo(THE_ADR_UNDER_A_DISTINCT_TITLE),
        "# Use spaces",
    )],
    untouched: &[THE_COMMITTED_ADR],
    ..W
}];
/// Where a migration keeps the slug it was started under.
const THE_MIGRATIONS_RECORDED_SLUG: &str =
    ".jigc/tasks/migrate-adr-notes-old-e1a7d84cafc2/slug-override";
const COMMITTED_AND_AN_AUTHOR_UNDER_A_MIGRATIONS_SLUG: [Case; 1] = [Case {
    name: "title-ignored-committed-under-a-migrations-slug-discarded-and-re-migrated-as-printed",
    site: TITLE_IGNORED,
    cause: "a migration recorded a `--slug` a committed doc already answers to, and its \
            payload carries another title than that doc's",
    state: a_committed_adr![
        A_FOREIGN_NOTE_COMMITTED[0],
        A_FOREIGN_NOTE_COMMITTED[1],
        A_FOREIGN_NOTE_COMMITTED[2],
        Step::Jigc(&["migrate", "notes/old.md", "--as", "adr", "--slug", "use-tabs"]),
        ANOTHER_TITLE,
    ],
    refusing: &[
        "doc",
        "author",
        "adr",
        "--from-file",
        "../outside/again.yaml",
        "--task",
        THE_NOTES_MIGRATION,
    ],
    // The discard, as printed; then the migration again under the reader's slug. The
    // migration's task is named after its source, so the area the discard removed is
    // there again — under the slug that was named.
    acts: &[
        Act::Run(2),
        Act::RunChosen(Chosen {
            line: 3,
            own: &[a_free_slug!(
                "old-note",
                "a slug of the migration's own, distinct as the route asks: other than \
                 `use-tabs`, which the committed doc answers to, and nothing answers to it"
            )],
        }),
    ],
    holds: &[Check::Contains(
        Place::Repo(THE_MIGRATIONS_RECORDED_SLUG),
        "old-note",
    )],
    untouched: &[THE_COMMITTED_ADR],
    ..W
}];

// ---------------------------------------------------------------------------
// `already_exists_finding` · `create.already-exists` — a create under an entry that only
// creates, over a home that is taken
// ---------------------------------------------------------------------------

/// A task of the workflow whose `allows-create` entry carries `new: true` — its id slugs
/// from the intent, `report-a-bug` — and a file somebody wrote at the home of the finding
/// *Slow start*, in no commit.
const A_REPORT_TASK: Step =
    Step::Jigc(&["start", "--workflow", "report-jigc-feedback", "report a bug"]);
const THE_TAKEN_HOME: &str = "docs/jigc-feedback/slow-start.md";
const A_FILE_AT_THE_HOME: Step = Step::File(Place::Repo(THE_TAKEN_HOME), "# Slow start\n");
const REPORT_AREA: &str = ".jigc/tasks/report-a-bug";
/// The task's staged finding under each id a case below mints.
const THE_FINDING_BESIDE_THE_FILE: &str =
    ".jigc/tasks/report-a-bug/docs/jigc-feedback:slow-start-again.md";
const THE_FINDING_UNDER_A_DISTINCT_SLUG: &str =
    ".jigc/tasks/report-a-bug/docs/jigc-feedback:other.md";

const TAKEN_AND_A_CREATE: [Case; 1] = [Case {
    name: "already-exists-create-line-run-as-printed",
    site: ALREADY_EXISTS,
    cause: "the home of the id a `jigc doc create` mints from its title holds a file, and \
            the task holds no doc yet",
    state: &[SETUP, A_REPORT_TASK, A_FILE_AT_THE_HOME],
    refusing: &[
        "doc",
        "create",
        "jigc-feedback",
        "--title",
        "Slow start",
        "--task",
        "report-a-bug",
    ],
    acts: &[Act::RunChosen(Chosen {
        line: 2,
        own: &[
            the_title_of_the_refused_command!(
                "Slow start",
                "the title the reader typed, kept: the route says *or keep this one and pass \
                 `--slug <slug>` to mint beside the existing doc*"
            ),
            a_free_slug!(
                "slow-start-again",
                "a slug whose home nothing holds, to mint beside the file that is there: \
                 that file stands at the home of `slow-start`, which the title alone slugs to"
            ),
        ],
    })],
    holds: &[Check::Contains(
        Place::Repo(THE_FINDING_BESIDE_THE_FILE),
        "# Slow start",
    )],
    untouched: &[THE_TAKEN_HOME],
    ..W
}];
const TAKEN_AND_A_CREATE_UNDER_A_SLUG: [Case; 1] = [Case {
    name: "already-exists-create-under-a-slug-line-run-as-printed",
    site: ALREADY_EXISTS,
    cause: "the home of the id a `jigc doc create --slug` names holds a file, and the task \
            holds no doc yet",
    state: &[SETUP, A_REPORT_TASK, A_FILE_AT_THE_HOME],
    refusing: &[
        "doc",
        "create",
        "jigc-feedback",
        "--title",
        "Other",
        "--slug",
        "slow-start",
        "--task",
        "report-a-bug",
    ],
    acts: &[Act::RunChosen(Chosen {
        line: 2,
        own: &[
            the_title_of_the_refused_command!(
                "Other",
                "the title the reader typed: the route says the id comes from the slug and \
                 not from the title, and asks nothing of the title"
            ),
            a_free_slug!(
                "other",
                "a distinct slug, as the route asks: other than `slow-start`, whose home \
                 holds the file, and nothing stands at its own — it is what the title slugs to"
            ),
        ],
    })],
    holds: &[Check::Contains(
        Place::Repo(THE_FINDING_UNDER_A_DISTINCT_SLUG),
        "# Other",
    )],
    untouched: &[THE_TAKEN_HOME],
    ..W
}];
const TAKEN_AND_AN_AUTHOR: [Case; 1] = [Case {
    name: "already-exists-author-line-run-as-printed",
    site: ALREADY_EXISTS,
    cause: "the home of the id a `jigc doc author` mints from its payload's title holds a \
            file, and the task holds no doc yet",
    state: &[
        SETUP,
        A_REPORT_TASK,
        A_FILE_AT_THE_HOME,
        Step::File(Place::Outside("finding.yaml"), "title: \"Slow start\"\n"),
        Step::File(
            Place::Outside("distinct.yaml"),
            "title: \"Slow start again\"\n",
        ),
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
    acts: &[Act::RunChosen(Chosen {
        line: 1,
        own: &[Own {
            placeholder: "<payload>",
            value: THE_PAYLOAD_UNDER_A_DISTINCT_TITLE,
            why: "a payload that is one — as much of a finding as the refused one holds — \
                  with its `title:` set to a distinct title, as the route asks: *Slow start \
                  again*, at whose slug's home nothing stands",
        }],
    })],
    holds: &[Check::Contains(
        Place::Repo(THE_FINDING_BESIDE_THE_FILE),
        "# Slow start again",
    )],
    untouched: &[THE_TAKEN_HOME],
    ..W
}];

/// A finding of the task's own, filed whole — created, its three required fields and its
/// one required slot written — in the task `$task`.
macro_rules! a_finding_filed_in {
    ($task:literal) => {
        [
            Step::Jigc(&[
                "doc",
                "create",
                "jigc-feedback",
                "--title",
                "Mine",
                "--task",
                $task,
            ]),
            Step::Jigc(&[
                "doc",
                "set-field",
                "jigc-feedback:mine#meta/kind",
                "--task",
                $task,
                "--value",
                "bug",
            ]),
            Step::Jigc(&[
                "doc",
                "set-field",
                "jigc-feedback:mine#meta/found-in",
                "--task",
                $task,
                "--value",
                "a route case",
            ]),
            Step::Jigc(&[
                "doc",
                "set-field",
                "jigc-feedback:mine#meta/jigc-version",
                "--task",
                $task,
                "--value",
                "1.0.0",
            ]),
            Step::File(Place::Outside("description"), "It is slow.\n"),
            Step::Jigc(&[
                "doc",
                "set-slot",
                "jigc-feedback:mine#description",
                "--task",
                $task,
                "--from-file",
                "../outside/description",
            ]),
        ]
    };
}
const A_FINDING_FILED: [Step; 6] = a_finding_filed_in!("report-a-bug");
const THE_REPORTS_COMMIT_DOC: [Step; 3] = commit_doc!("report-a-bug");
const A_SECOND_CREATE: &[&str] = &[
    "doc",
    "create",
    "jigc-feedback",
    "--title",
    "Slow start",
    "--task",
    "report-a-bug",
];
/// The task's own finding, once its task has landed.
const THE_FILED_FINDING: &str = "docs/jigc-feedback/mine.md";
/// The task the printed front door mints, run as printed: its quoted `<intent>` slugs to
/// `intent`.
const THE_NEXT_TASK: &str = ".jigc/tasks/intent";

/// **The task already holds its doc, and a second create lands on a home that is taken**:
/// the route ends this task — landed, or abandoned — and files the next finding in a task
/// of its own.
macro_rules! held {
    ($($plant:expr),* $(,)?) => {
        &[
            SETUP,
            A_REPORT_TASK,
            A_FILE_AT_THE_HOME,
            A_FINDING_FILED[0],
            A_FINDING_FILED[1],
            A_FINDING_FILED[2],
            A_FINDING_FILED[3],
            A_FINDING_FILED[4],
            A_FINDING_FILED[5],
            THE_REPORTS_COMMIT_DOC[0],
            THE_REPORTS_COMMIT_DOC[1],
            THE_REPORTS_COMMIT_DOC[2],
            $($plant,)*
        ]
    };
}
const THE_TASK_HOLDS_ITS_DOC: &str = "the task already holds the one doc its role carries, and \
    a second `jigc doc create` mints an id whose home holds a file";
const HELD_BY_THE_TASK: [Case; 2] = [
    Case {
        name: "already-exists-held-task-landed-and-the-next-started-as-printed",
        site: ALREADY_EXISTS,
        cause: THE_TASK_HOLDS_ITS_DOC,
        state: held![],
        refusing: A_SECOND_CREATE,
        acts: &[Act::Run(4), Act::Run(6)],
        holds: &[
            Check::RegularFile(Place::Repo(THE_FILED_FINDING)),
            Check::Absent(Place::Repo(REPORT_AREA)),
            Check::Directory(Place::Repo(THE_NEXT_TASK)),
        ],
        untouched: &[THE_TAKEN_HOME],
        ..W
    },
    Case {
        name: "already-exists-held-task-discarded-and-the-next-started-as-printed",
        site: ALREADY_EXISTS,
        cause: THE_TASK_HOLDS_ITS_DOC,
        state: held![],
        refusing: A_SECOND_CREATE,
        acts: &[Act::Run(5), Act::Run(6)],
        holds: &[
            Check::Absent(Place::Repo(REPORT_AREA)),
            Check::Directory(Place::Repo(THE_NEXT_TASK)),
        ],
        untouched: &[THE_TAKEN_HOME],
        ..W
    },
];

/// The same in a milestone's sub-task, added under the workflow that only creates: the
/// route lands it with the milestone's boundary.
const A_FINDING_FILED_IN_THE_SUB_TASK: [Step; 6] = a_finding_filed_in!("report-it");
macro_rules! held_in_a_sub_task {
    ($($plant:expr),* $(,)?) => {
        &[
            SETUP,
            Step::Jigc(&["milestone", "create", "Fan out"]),
            Step::Jigc(&[
                "milestone",
                "add-task",
                "fan-out",
                "report it",
                "--workflow",
                "report-jigc-feedback",
            ]),
            Step::Jigc(&["milestone", "provision", "fan-out"]),
            A_FILE_AT_THE_HOME,
            A_FINDING_FILED_IN_THE_SUB_TASK[0],
            A_FINDING_FILED_IN_THE_SUB_TASK[1],
            A_FINDING_FILED_IN_THE_SUB_TASK[2],
            A_FINDING_FILED_IN_THE_SUB_TASK[3],
            A_FINDING_FILED_IN_THE_SUB_TASK[4],
            A_FINDING_FILED_IN_THE_SUB_TASK[5],
            $($plant,)*
        ]
    };
}
const THE_SUB_TASK_HOLDS_ITS_DOC: &str = "a milestone's sub-task already holds the one doc its \
    role carries, and a second `jigc doc create` mints an id whose home holds a file";
const A_SECOND_CREATE_IN_THE_SUB_TASK: &[&str] = &[
    "doc",
    "create",
    "jigc-feedback",
    "--title",
    "Slow start",
    "--task",
    "report-it",
];
const HELD_BY_A_SUB_TASK: [Case; 2] = [
    Case {
        name: "already-exists-held-sub-task-milestone-landed-and-the-next-started-as-printed",
        site: ALREADY_EXISTS,
        cause: THE_SUB_TASK_HOLDS_ITS_DOC,
        state: held_in_a_sub_task![],
        refusing: A_SECOND_CREATE_IN_THE_SUB_TASK,
        acts: &[Act::Run(4), Act::Run(7)],
        holds: &[
            Check::RegularFile(Place::Repo(THE_FILED_FINDING)),
            Check::Directory(Place::Repo(THE_NEXT_TASK)),
        ],
        untouched: &[THE_TAKEN_HOME],
        ..W
    },
    Case {
        name: "already-exists-held-sub-task-discarded-and-the-next-started-as-printed",
        site: ALREADY_EXISTS,
        cause: THE_SUB_TASK_HOLDS_ITS_DOC,
        state: held_in_a_sub_task![],
        refusing: A_SECOND_CREATE_IN_THE_SUB_TASK,
        acts: &[Act::Run(6), Act::Run(7)],
        holds: &[Check::Directory(Place::Repo(THE_NEXT_TASK))],
        untouched: &[THE_TAKEN_HOME],
        ..W
    },
];

/// The cases of the family, a group a cause — and, within a group, a case a way out of the
/// cause's route.
pub const CASES: &[&[Case]] = &[
    &RETITLED_FROM_THE_SUB_TASK,
    &RETITLED_FROM_AN_ORDINARY_TASK,
    &AN_ITEM_THE_RECORD_DOES_NOT_HOLD,
    &STAGED_BY_A_CREATE,
    &STAGED_BY_A_CREATE_UNDER_A_SLUG,
    &STAGED_BY_AN_AUTHOR,
    &STAGED_BY_AN_AUTHOR_UNDER_A_MIGRATIONS_SLUG,
    &COMMITTED_AND_A_CREATE,
    &COMMITTED_AND_A_CREATE_UNDER_A_SLUG,
    &COMMITTED_AND_AN_AUTHOR,
    &COMMITTED_AND_AN_AUTHOR_UNDER_A_MIGRATIONS_SLUG,
    &TAKEN_AND_A_CREATE,
    &TAKEN_AND_A_CREATE_UNDER_A_SLUG,
    &TAKEN_AND_AN_AUTHOR,
    &HELD_BY_THE_TASK,
    &HELD_BY_A_SUB_TASK,
];

/// The sites of the family that have no case, each with why: none of the three.
pub const REASONS: &[Reason] = &[];

/// A doctype with a fixed identity and no fixed title, or one under an entry that only
/// creates.
const A_PACK_OF_THE_READERS_OWN: &str = "the arm is for a doctype with a fixed identity — one \
    instance at one home, whatever the title — and of the shipped packs' five such doctypes \
    each is a singleton whose title the schema fixes, which an earlier rank of the same \
    pre-check answers, and none stands under an entry that carries `new: true`: it takes a \
    pack of the reader's own";

/// The write a route says to make again, where the route prints a command line.
const THE_REFUSED_WRITE_AGAIN: &str = "the write is the refused command, and the frame runs a \
    refusing command again only where the route prints no command line of the product — \
    this one prints the rename. Driven by hand after the printed rename, at the first \
    cause: the same `jigc doc create` exits 0 with *already existed — copied in for update*";

/// **What was read for each site, and how many causes it holds** — a hand count, with its
/// derivation; the causes that have no case, each with why; and the ways out of its route
/// that are not followed, each with why.
pub const READINGS: &[Reading] = &[
    Reading {
        site: RETITLE_IN_THE_RECORD,
        read: &[
            "crates/cli/src/doc.rs::run_retitle_item — the refusal by doctype, behind the task's resolution, the address's parse and the item target's shape, ahead of every read",
            "crates/cli/src/doc.rs::run_retitle_item against the previous release (`git show 91834b5e:crates/cli/src/doc.rs`) — what changed: the binding of what the read hands back and one argument of the acknowledgement, both behind this refusal",
        ],
        causes: 3,
        derivation: "one predicate — the addressed doc is a milestone record — and one route. \
                     The states that reach it differ by who asks and by what is asked for: \
                     a sub-task of the milestone, an ordinary task beside it, and an item \
                     the record does not hold (the refusal is not gated on the item being \
                     there). The record has one repeatable section and no nested one, so \
                     the nested arm of the item's path is reached by no shipped doctype \
                     here",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: TITLE_IGNORED,
        read: &[
            "crates/cli/src/doc.rs::title_ignored_refusal — two arms by where the incumbent body stands, staged or committed; the staged arm's rename carries a `--slug` in force",
            "crates/cli/src/doc.rs::title_pre_check — rank 3, behind the create-only gate, the fixed title and the identity divergence: an incumbent body whose `# H1` is not the supplied title",
            "crates/cli/src/doc.rs::distinct_identity_route — the committed arm's route, by where the id comes from: a fixed identity, a `--slug` at `create`, a migration's recorded slug at `author`, the title at `create`, the payload's title at `author`",
            "crates/cli/src/doc.rs::title_ignored_refusal against the previous release — the one change: `distinct_identity_route` is handed one argument more, `None`, for an occupant that is no doc; the committed arm's words are the same",
        ],
        causes: 9,
        derivation: "the staged arm: the doc was staged by a `create`, by a `create --slug`, \
                     by an `author`, or by an `author` under a migration's recorded slug — \
                     4, the last three by what the printed rename carries and which verb \
                     the message names. The committed arm, one cause a branch of \
                     `distinct_identity_route`: a fixed identity; a `create --slug`; an \
                     `author` under a migration's recorded slug; a `create` from its title; \
                     an `author` from its payload's title — 5",
        without_case: &[(
            "a committed doc of a doctype with a fixed identity",
            Why::NotConstructible(A_PACK_OF_THE_READERS_OWN),
        )],
        not_followed: &[
            (
                "*then re-run this write unchanged* — the staged arm, behind the printed rename",
                Why::NotConstructible(THE_REFUSED_WRITE_AGAIN),
            ),
            (
                "*choose a distinct `--title`* — the committed arm's first way at `create`, in words",
                Why::NotConstructible(
                    "a distinct title and no `--slug` is the refused command with another \
                     argument, and no line the route prints: the line it prints carries \
                     `--slug <slug>` and is followed with the title kept, the route's \
                     second way. The frame runs a refusing command again only as it was \
                     typed, and only where the route prints no command line. Driven by \
                     hand: `jigc doc create adr --title 'Use spaces' --task again` exits 0 \
                     and stages a second ADR",
                ),
            ),
        ],
    },
    Reading {
        site: ALREADY_EXISTS,
        read: &[
            "crates/engine/src/state.rs::already_exists_finding — the message; the route is handed in",
            "crates/cli/src/doc.rs::already_exists_refusal — new since the previous release: the one builder of the refusal, which hands a home that holds no regular file to the store's refusal and every other occupied home here, and picks the route by whether the task already holds its doc",
            "crates/cli/src/doc.rs::title_pre_check, create_refused — its three producers: the ranked early ask, the incumbent probe's second look, and the create itself",
            "crates/cli/src/doc.rs::one_doc_per_task_route, distinct_identity_route — the two routes: the task's exits where it holds its doc (a sub-task's is its milestone's boundary), a distinct identity where it holds none",
            "crates/engine/src/state.rs::create_gated, create_occupied, occupied_home — what *occupied* is: any entry on disk at the minted identity's home, read without following a link",
        ],
        causes: 9,
        derivation: "under an entry that carries `new: true`, a home that holds a regular \
                     file. The task holds no doc and the id comes from a `create`'s title, \
                     a `create --slug`, or an `author`'s payload — 3 routes; from a \
                     migration's recorded slug — 1; or the doctype has a fixed identity — \
                     1. The task holds its doc already: an ordinary task, or a milestone's \
                     sub-task, whose route names the milestone's boundary — 2 (the verb \
                     changes one clause of that route, *a distinct `--title` or a `--slug`* \
                     against *a distinct payload `title:`*, and none of its commands). And \
                     the two later looks at the home — the incumbent probe, the create \
                     itself — where the home was free at the first — 2. Whether the file \
                     at the home is a committed doc or one somebody wrote by hand changes \
                     no word of the refusal: it is followed over a file in no commit",
        without_case: &[
            (
                "the id comes from a migration's recorded `--slug`",
                Why::NotConstructible(
                    "a migration's workflow is `migrate-<doctype>`, and the two doctypes \
                     whose entry carries `new: true` have none: it takes a pack of the \
                     reader's own",
                ),
            ),
            (
                "a doctype with a fixed identity under an entry that only creates",
                Why::NotConstructible(A_PACK_OF_THE_READERS_OWN),
            ),
            (
                "the home is free at the ranked ask and taken at the incumbent probe's look",
                Why::NotConstructible(
                    "two reads of one path inside one command, a few statements apart: a \
                     state has no step that writes between them",
                ),
            ),
            (
                "the home is free at both looks of the pre-check and taken when the create probes it",
                Why::NotConstructible(
                    "the same: a file that arrives between two reads of one command",
                ),
            ),
        ],
        not_followed: &[(
            "*choose a distinct `--title`* — the first way of a `create` in a task that holds no doc, in words",
            Why::NotConstructible(
                "a distinct title and no `--slug` is the refused command with another \
                 argument, and no line the route prints: the line it prints carries \
                 `--slug <slug>` and is followed with the title kept, the route's second \
                 way. Driven by hand: `jigc doc create jigc-feedback --title 'Another' \
                 --task report-a-bug` exits 0 and stages the finding",
            ),
        )],
    },
];
