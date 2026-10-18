//! **The byte table's rows for the doc family** — the eight `jigc doc` write leaves
//! (`crates/cli/tests/byte_table.rs`), lifted quote for quote from the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`,
//! §3b), written from their docs in the plan's task `B6` in a commit of their own before a
//! cell of any of them ran.
//!
//! **A `doc` write stages, and only `finalize` commits.** Every row is the same three
//! lines — the task's working area rebuilt (`W1`), a copied-in doc's baseline recorded
//! under `.jigc/state/` (`W2`), and the home that doc is copied in from, read and never
//! written (`W3`) — with the three a refusal may rebuild; `doc rename`'s has a fourth, the
//! staged body under the name a re-slug leaves (`W4`). **What of the record is no line**:
//! `W5`, where *on a refusal: no* comes from, and `W6`, why no row lets a write change a
//! doc's home, git's index or a commit. No row names a consent: no `doc` verb takes one.

use super::{ON_A_REFUSAL, refuses};
use crate::support::byte_cell::{
    Aim, Allows, Cell, Condition, Kind, Lead, Line, Listed, Mark, May, Plant, Prepared, Quote, Row,
    When, Where,
};
use crate::support::route_case::{Place, Step};
use crate::support::trial_corpus::State;

/// The heading the copy-in's sentences stand under.
const THE_BARRIER: &str = "The write-time `--task`-scoped barrier (M8)";

/// A line that holds where the command acted, whenever it did.
const fn line(id: &'static str, allows: Allows, quotes: &'static [Quote]) -> Line {
    Line {
        id,
        allows,
        when: When::Always,
        on_refusal: false,
        quotes,
    }
}

/// **`W1`** — the task's working area, rebuilt: every write stages there.
const W1: Line = line(
    "W1",
    Allows::Files(Where::Under(".jigc/tasks/*"), May::Rebuilt),
    &[
        Quote {
            doc: "design/write-commands.md",
            heading: "Staging and the transaction model",
            words: "There is no separate proposal object: every write lands in the current task's working area immediately (no `begin`), and \"propose / review / apply\" is just `write / validate / finalize` over that area.",
        },
        Quote {
            doc: "design/write-commands.md",
            heading: THE_BARRIER,
            words: "**Where a write may be typed (the rc.24 fix pass).** Every write verb stages into the task's working area, which lives under the main checkout from every cwd",
        },
    ],
);

/// **`W2`** — `.jigc/state/`, rebuilt: the copy-in records the baseline of the doc it
/// copies, where the record holds none, under the lock its save takes.
const W2: Line = line(
    "W2",
    Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
    &[
        Quote {
            doc: "design/write-commands.md",
            heading: THE_BARRIER,
            words: "**The copy-in also records the doc's `file-state` baseline when it has none (the rc.24 fix pass)**",
        },
        Quote {
            doc: "design/reconciliation.md",
            heading: "Baseline adoption",
            words: "A write verb that stages a committed doc — the **copy-in**: the six edit leaves on their first touch, and `doc create` / `doc author` over an existing doc — reads the file at its home, and that read records the doc's baseline when the record holds none",
        },
        Quote {
            doc: "design/storage.md",
            heading: "Concurrent writers",
            words: "`engine::state::with_save_lock` runs the whole critical section under an advisory lock — taken on a **stable `<filename>.lock` sibling**",
        },
    ],
);

/// **`W3`** — the home of a committed doc the write copies in: read, and nothing written
/// there. What a plant put in a doc's place is aimed at.
const W3: Line = line(
    "W3",
    Allows::Files(Where::AStagedDocsHome, May::CopiedFrom),
    &[
        Quote {
            doc: "design/write-commands.md",
            heading: THE_BARRIER,
            words: "The first write to such a doc **copies the committed body into the sub-area** (`copy_in` → `edited-from-base`), then splices",
        },
        Quote {
            doc: "design/finalize.md",
            heading: "4. Promote managed docs",
            words: "(2) **Reads still follow a link at a home** — `jigc doc show` serves a linked doc and the copy-in reads one",
        },
        Quote {
            doc: "design/reconciliation.md",
            heading: "Detection timing",
            words: "a second write to an already-staged doc reads the staged copy and does not look at the home again.",
        },
    ],
);

/// **`W4`** — `doc rename`'s alone: the staged body of a doc the task minted, under the
/// name a re-slug leaves — re-slugged, and nothing else: the same lines under the new name
/// in the same task, its `# H1` rewritten ([`May::ReSlugged`]). No other staged doc goes
/// under it, and none of another task.
const W4: Line = line(
    "W4",
    Allows::Files(Where::Path(".jigc/tasks/*/docs/*"), May::ReSlugged),
    &[Quote {
        doc: "design/write-commands.md",
        heading: "`jigc doc rename` — the in-task title change, split on committed-store identity (M48)",
        words: "`--to` rewrites the `# H1` **and** moves the identity: the staged body, its `docs/provenance.json` entry (re-keyed, its recorded provenance intact — the by-task-id join classifies by it), every `roles.json` binding, every **staged doc's** `ref` fields, and a migration task's recorded `slug-override`.",
    }],
);

/// The home of every row of the family: the smallest prepared state in which `W2`'s write
/// happens and `W3`'s place stands — a copy-in needs a committed managed doc, and `fresh`
/// holds none.
const HOME: Prepared = Prepared::Of(State::CommittedSingletons);

/// The lines every row of the family has.
const LINES: [Line; 7] = [
    W1,
    W2,
    W3,
    ON_A_REFUSAL[0],
    ON_A_REFUSAL[1],
    ON_A_REFUSAL[2],
    ON_A_REFUSAL[3],
];

/// A row of the family: its leaf, and the lines every row has.
const fn row(leaf: &'static [&'static str]) -> Row {
    Row {
        leaf,
        consent: None,
        home: HOME,
        lines: &LINES,
    }
}

/// **The six edit leaves** — the first part of the family (the record's §3b): each writes
/// one leaf, item or heading of a doc the task stages, copying a committed doc in on its
/// first touch. `doc rename` has `W4` too.
pub const ADD_ITEM: Row = row(&["doc", "add-item"]);
pub const REMOVE_ITEM: Row = row(&["doc", "remove-item"]);
pub const RETITLE_ITEM: Row = row(&["doc", "retitle-item"]);
pub const RENAME: Row = Row {
    lines: &[
        W1,
        W2,
        W3,
        W4,
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
    ..row(&["doc", "rename"])
};
pub const SET_FIELD: Row = row(&["doc", "set-field"]);
pub const SET_SLOT: Row = row(&["doc", "set-slot"]);

/// **The minting doors** — the second part (the record's §3b): each mints a doc into the
/// task, or copies one in for update where it is already a doc.
pub const CREATE: Row = row(&["doc", "create"]);
pub const AUTHOR: Row = row(&["doc", "author"]);

/// The family's rows.
pub const ROWS: &[Row] = &[
    ADD_ITEM,
    REMOVE_ITEM,
    RETITLE_ITEM,
    RENAME,
    SET_FIELD,
    SET_SLOT,
    CREATE,
    AUTHOR,
];

// ---------------------------------------------------------------------------
// The cells
// ---------------------------------------------------------------------------

/// The binary under test, in the repository.
const fn jigc(args: &'static [&'static str]) -> Lead {
    Lead::Step(Step::Jigc(args))
}

/// A payload a lead-in or the command reads with `--from-file`: a file under `outside/`,
/// which a command started in the repository reaches as `../outside/<name>`.
const fn payload(name: &'static str, body: &'static str) -> Lead {
    Lead::Step(Step::File(Place::Outside(name), body))
}

/// What a planted cell's plant is aimed at.
const fn at(line: &'static str, kind: Kind) -> Option<Aim> {
    Some(Aim { line, kind })
}

/// One cell of the family.
#[allow(clippy::too_many_arguments)]
const fn cell(
    name: &'static str,
    leaf: &'static [&'static str],
    state: Prepared,
    leads: &'static [Lead],
    argv: &'static [&'static str],
    listed: Listed,
    aim: Option<Aim>,
    marks: &'static [Mark],
) -> Cell {
    Cell {
        name,
        leaf,
        state,
        leads,
        argv,
        listed,
        beside: None,
        aim,
        marks,
    }
}

const FRESH: Prepared = Prepared::Of(State::Fresh);

/// **The task every edit cell of the roadmap and the changelog writes in** —
/// `sharpen-the-roadmap`, on `single-task`: no doc write is gated but a create.
const A_TASK: Lead = jigc(&["start", "--workflow", "single-task", "sharpen the roadmap"]);

/// What `doc set-slot` writes, read with `--from-file`.
const WHAT_IT_PROVES: Lead = payload("proves", "That the composed loop lands, sharpened.\n");

/// **The task's first write to the roadmap**, before the command's own: a slot of the
/// committed milestone, which copies the roadmap in — the plan's plant **E** at its second
/// position stands after it. It reads the payload [`WHAT_IT_PROVES`] puts.
const A_FIRST_WRITE_TO_THE_ROADMAP: [Lead; 2] = [
    WHAT_IT_PROVES,
    jigc(&[
        "doc",
        "set-slot",
        "roadmap:roadmap#milestones/m-alpha/decomposition",
        "--from-file",
        "../outside/proves",
        "--task",
        "sharpen-the-roadmap",
    ]),
];

/// **A sibling task that stages the roadmap too** — `plan-the-roadmap`, started beside
/// [`A_TASK`], and its first write to the roadmap: a slot of the committed milestone, which
/// copies the roadmap into **its own** area. A write of the other task must leave that
/// area as it stands — *"a sub-agent physically cannot stage into a sibling's area"*
/// (`design/write-commands.md` → *The write-time `--task`-scoped barrier (M8)*).
const A_SIBLING_TASK_STAGING_THE_ROADMAP: [Lead; 3] = [
    jigc(&["start", "--workflow", "single-task", "plan the roadmap"]),
    WHAT_IT_PROVES,
    jigc(&[
        "doc",
        "set-slot",
        "roadmap:roadmap#milestones/m-alpha/decomposition",
        "--from-file",
        "../outside/proves",
        "--task",
        "plan-the-roadmap",
    ]),
];

/// **The task's first write to the changelog**: a staged change-group added, which copies
/// the changelog in.
const A_FIRST_WRITE_TO_THE_CHANGELOG: Lead = jigc(&[
    "doc",
    "add-item",
    "changelog:changelog#unreleased-changes",
    "--title",
    "fixed",
    "--task",
    "sharpen-the-roadmap",
]);

/// The five edit commands of the roadmap and the changelog, each the task's first touch of
/// a committed doc of `committed-singletons` — so each copies it in (`W2`, `W3`) and stages
/// it (`W1`).
const ADD_A_MILESTONE: &[&str] = &[
    "doc",
    "add-item",
    "roadmap:roadmap#milestones",
    "--title",
    "M-Beta",
    "--task",
    "sharpen-the-roadmap",
];
const REMOVE_A_MILESTONE: &[&str] = &[
    "doc",
    "remove-item",
    "roadmap:roadmap#milestones/m-alpha",
    "--task",
    "sharpen-the-roadmap",
];
const RETITLE_A_MILESTONE: &[&str] = &[
    "doc",
    "retitle-item",
    "roadmap:roadmap#milestones/m-alpha",
    "--title",
    "M-Alpha, sharpened",
    "--task",
    "sharpen-the-roadmap",
];
const SET_A_RELEASES_LINK: &[&str] = &[
    "doc",
    "set-field",
    "changelog:changelog#releases/1-0-0/link",
    "--value",
    "https://example.com/release",
    "--task",
    "sharpen-the-roadmap",
];
const SET_WHAT_IT_PROVES: &[&str] = &[
    "doc",
    "set-slot",
    "roadmap:roadmap#milestones/m-alpha/proves",
    "--from-file",
    "../outside/proves",
    "--task",
    "sharpen-the-roadmap",
];

/// **A task that made a decision it has not committed** — `record-two-ways`, which
/// created `adr:two-ways`: the never-committed arm of `doc rename`, whose re-slug moves the
/// identity (`W4`).
const A_TASK_FOR_TWO_WAYS: Lead = jigc(&["start", "--workflow", "single-task", "record two ways"]);
const TWO_WAYS: Lead = jigc(&[
    "doc",
    "create",
    "adr",
    "--title",
    "Two ways",
    "--task",
    "record-two-ways",
]);
const RENAME_TWO_WAYS: &[&str] = &[
    "doc",
    "rename",
    "adr:two-ways",
    "--to",
    "Three ways",
    "--task",
    "record-two-ways",
];

/// **A decision landed by a task of its own** — `adr:one-way`, committed at
/// `docs/decisions/one-way.md`: every doc of `committed-singletons` is a singleton, whose
/// identity `doc rename` refuses outright, so its committed arm needs a doc of its own.
const LANDS_ONE_WAY: [Lead; 12] = [
    jigc(&["start", "--workflow", "single-task", "record a decision"]),
    jigc(&[
        "doc",
        "create",
        "adr",
        "--title",
        "One way",
        "--task",
        "record-a-decision",
    ]),
    payload("context", "The forces.\n"),
    payload("decision", "The call.\n"),
    payload("consequences", "What follows.\n"),
    payload("summary", "record a decision\n"),
    jigc(&[
        "doc",
        "set-slot",
        "adr:one-way#context",
        "--from-file",
        "../outside/context",
        "--task",
        "record-a-decision",
    ]),
    jigc(&[
        "doc",
        "set-slot",
        "adr:one-way#decision",
        "--from-file",
        "../outside/decision",
        "--task",
        "record-a-decision",
    ]),
    jigc(&[
        "doc",
        "set-slot",
        "adr:one-way#consequences",
        "--from-file",
        "../outside/consequences",
        "--task",
        "record-a-decision",
    ]),
    jigc(&[
        "doc",
        "set-field",
        "commit:record-a-decision#type",
        "--value",
        "docs",
        "--task",
        "record-a-decision",
    ]),
    jigc(&[
        "doc",
        "set-slot",
        "commit:record-a-decision#summary",
        "--from-file",
        "../outside/summary",
        "--task",
        "record-a-decision",
    ]),
    jigc(&["task", "finalize", "record-a-decision"]),
];

/// The task that retitles the landed decision — `retitle-the-decision`.
const A_TASK_TO_RETITLE: Lead = jigc(&["start", "--workflow", "single-task", "retitle the decision"]);

/// **The task's first write to the landed decision**: a slot, which copies it in.
const A_FIRST_WRITE_TO_THE_DECISION: Lead = jigc(&[
    "doc",
    "set-slot",
    "adr:one-way#context",
    "--from-file",
    "../outside/context",
    "--task",
    "retitle-the-decision",
]);

/// The committed arm of `doc rename`: a same-slug retitle of a doc copied in.
const RETITLE_ONE_WAY: &[&str] = &[
    "doc",
    "rename",
    "adr:one-way",
    "--to",
    "One Way",
    "--task",
    "retitle-the-decision",
];

/// **The cells of the six edit leaves** — the family's first part (the record's §3b),
/// each listed from the rows' docs. With no task a write is rejected (*"no active task"*,
/// `design/write-commands.md` → *The create-gate*, 1.). A first touch of a committed doc
/// copies it in and stages it, and a file of the user's in the task's area or beside the
/// file-state record stays. **At the home the copy is read from** (`W3`): where no doc
/// body stands — a link to nothing, a directory, the file deleted — the write refuses,
/// since *"A working instance must exist before you can `set-slot` into it"*
/// (*Instance provisioning*), with the words a run gives; where a file that is no doc of
/// its type stands — the user's, or one a live link leads to — the cell is listed
/// refusing, as the human ruled on 2026-10-18 the record's §6, entry 14: a write refuses
/// **before** its copy-in where the bytes at the home do not parse as the doctype, staging
/// and binding nothing — and entry 15: every write that refuses leaves the task's area as
/// it found it. The code it is listed with is today's nearest, the one its siblings
/// refuse with; and a deletion
/// after the task's first write is not looked at, since *"a second write to an
/// already-staged doc reads the staged copy and does not look at the home again"*.
/// **What a refusal's route says is no cell's**: the six of `remove-item` and `set-slot`
/// that refuse `write.non-reparseable` print *"nothing was persisted"* over the copy they
/// leave staged — a claim of its own, held by no cell, since a cell holds bytes and not
/// words: the findings' key
/// `r1-doc-write-says-nothing-was-persisted-where-its-refusal-leaves-bytes-in-the-tasks-area`
/// (`…/reshape/byte-table-findings.md` → §12). Each stays red under its byte key.
const EDITS: [Cell; 67] = [
    cell(
        "doc-add-item-with-no-task",
        &["doc", "add-item"],
        FRESH,
        &[],
        &["doc", "add-item", "adr:a-decision#options", "--title", "An Option"],
        refuses(1, "no active task"),
        None,
        &[],
    ),
    cell(
        "doc-add-a-milestone",
        &["doc", "add-item"],
        HOME,
        &[A_TASK],
        ADD_A_MILESTONE,
        Listed::Acts,
        None,
        &[],
    ),
    // **Beside a sibling task that stages the same doc** (after the family's review, its
    // seventh finding): `W1` and `W4` reach every task's area — `Under(".jigc/tasks/*")`,
    // one component left open, since a row is one for every task its cells name — where
    // the sentence says a write cannot stage into a sibling's. This cell is the one with
    // two live tasks: the command's first touch copies the roadmap into its task while
    // `plan-the-roadmap` holds a staged roadmap of its own. A staged doc of the sibling's
    // taken away is *lost* whatever `W1` says (the rule for prose handed to jigc); one
    // **rewritten** is what `W1` lets go, and no cell can see that — the vocabulary's
    // limit, named in the rows' record's §3b. A base cell: it plants nothing.
    cell(
        "doc-add-a-milestone-beside-a-sibling-task-that-stages-the-roadmap",
        &["doc", "add-item"],
        HOME,
        &[
            A_TASK,
            A_SIBLING_TASK_STAGING_THE_ROADMAP[0],
            A_SIBLING_TASK_STAGING_THE_ROADMAP[1],
            A_SIBLING_TASK_STAGING_THE_ROADMAP[2],
        ],
        ADD_A_MILESTONE,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-add-a-milestone-a-file-of-the-users-in-the-tasks-area",
        &["doc", "add-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::File(".jigc/tasks/sharpen-the-roadmap/notes-of-its-user.md"))],
        ADD_A_MILESTONE,
        Listed::Acts,
        at("W1", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-add-a-milestone-no-baseline-a-file-of-the-users-under-the-state",
        &["doc", "add-item"],
        HOME,
        &[Lead::Under(Condition::NoBaseline), A_TASK, Lead::Plant(Plant::File(".jigc/state/notes-of-its-user.md"))],
        ADD_A_MILESTONE,
        Listed::Acts,
        at("W2", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-add-a-milestone-a-file-of-the-users-in-the-docs-place",
        &["doc", "add-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::File("docs/roadmap.md")))],
        ADD_A_MILESTONE,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-add-a-milestone-a-link-to-nothing-in-the-docs-place",
        &["doc", "add-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing("docs/roadmap.md")))],
        ADD_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::LinkToNothing),
        &[],
    ),
    cell(
        "doc-add-a-milestone-a-link-to-a-file-in-the-repository-in-the-docs-place",
        &["doc", "add-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/roadmap.md", outside: false }))],
        ADD_A_MILESTONE,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkInside),
        &[],
    ),
    cell(
        "doc-add-a-milestone-a-link-to-a-file-outside-in-the-docs-place",
        &["doc", "add-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/roadmap.md", outside: true }))],
        ADD_A_MILESTONE,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkOutside),
        &[],
    ),
    cell(
        "doc-add-a-milestone-a-directory-in-the-docs-place",
        &["doc", "add-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::Dir("docs/roadmap.md")))],
        ADD_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::DirForAFile),
        &[],
    ),
    cell(
        "doc-add-a-milestone-the-doc-deleted-before-the-first-write",
        &["doc", "add-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::Deleted("docs/roadmap.md"))],
        ADD_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::EditBefore),
        &[],
    ),
    cell(
        "doc-add-a-milestone-the-doc-deleted-after-the-first-write",
        &["doc", "add-item"],
        HOME,
        &[A_TASK, A_FIRST_WRITE_TO_THE_ROADMAP[0], A_FIRST_WRITE_TO_THE_ROADMAP[1], Lead::Plant(Plant::Deleted("docs/roadmap.md"))],
        ADD_A_MILESTONE,
        Listed::Acts,
        at("W3", Kind::EditAfter),
        &[],
    ),
    cell(
        "doc-remove-item-with-no-task",
        &["doc", "remove-item"],
        FRESH,
        &[],
        &["doc", "remove-item", "adr:a-decision#options.an-option"],
        refuses(1, "no active task"),
        None,
        &[],
    ),
    cell(
        "doc-remove-a-milestone",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK],
        REMOVE_A_MILESTONE,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-remove-a-milestone-a-file-of-the-users-in-the-tasks-area",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::File(".jigc/tasks/sharpen-the-roadmap/notes-of-its-user.md"))],
        REMOVE_A_MILESTONE,
        Listed::Acts,
        at("W1", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-remove-a-milestone-no-baseline-a-file-of-the-users-under-the-state",
        &["doc", "remove-item"],
        HOME,
        &[Lead::Under(Condition::NoBaseline), A_TASK, Lead::Plant(Plant::File(".jigc/state/notes-of-its-user.md"))],
        REMOVE_A_MILESTONE,
        Listed::Acts,
        at("W2", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-remove-a-milestone-a-file-of-the-users-in-the-docs-place",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::File("docs/roadmap.md")))],
        REMOVE_A_MILESTONE,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-remove-a-milestone-a-link-to-nothing-in-the-docs-place",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing("docs/roadmap.md")))],
        REMOVE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::LinkToNothing),
        &[],
    ),
    cell(
        "doc-remove-a-milestone-a-link-to-a-file-in-the-repository-in-the-docs-place",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/roadmap.md", outside: false }))],
        REMOVE_A_MILESTONE,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkInside),
        &[],
    ),
    cell(
        "doc-remove-a-milestone-a-link-to-a-file-outside-in-the-docs-place",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/roadmap.md", outside: true }))],
        REMOVE_A_MILESTONE,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkOutside),
        &[],
    ),
    cell(
        "doc-remove-a-milestone-a-directory-in-the-docs-place",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::Dir("docs/roadmap.md")))],
        REMOVE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::DirForAFile),
        &[],
    ),
    cell(
        "doc-remove-a-milestone-the-doc-deleted-before-the-first-write",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::Deleted("docs/roadmap.md"))],
        REMOVE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::EditBefore),
        &[],
    ),
    cell(
        "doc-remove-a-milestone-the-doc-deleted-after-the-first-write",
        &["doc", "remove-item"],
        HOME,
        &[A_TASK, A_FIRST_WRITE_TO_THE_ROADMAP[0], A_FIRST_WRITE_TO_THE_ROADMAP[1], Lead::Plant(Plant::Deleted("docs/roadmap.md"))],
        REMOVE_A_MILESTONE,
        Listed::Acts,
        at("W3", Kind::EditAfter),
        &[],
    ),
    cell(
        "doc-retitle-item-with-no-task",
        &["doc", "retitle-item"],
        FRESH,
        &[],
        &["doc", "retitle-item", "adr:a-decision#options.an-option", "--title", "Another"],
        refuses(1, "no active task"),
        None,
        &[],
    ),
    cell(
        "doc-retitle-a-milestone",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK],
        RETITLE_A_MILESTONE,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-a-file-of-the-users-in-the-tasks-area",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::File(".jigc/tasks/sharpen-the-roadmap/notes-of-its-user.md"))],
        RETITLE_A_MILESTONE,
        Listed::Acts,
        at("W1", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-no-baseline-a-file-of-the-users-under-the-state",
        &["doc", "retitle-item"],
        HOME,
        &[Lead::Under(Condition::NoBaseline), A_TASK, Lead::Plant(Plant::File(".jigc/state/notes-of-its-user.md"))],
        RETITLE_A_MILESTONE,
        Listed::Acts,
        at("W2", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-a-file-of-the-users-in-the-docs-place",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::File("docs/roadmap.md")))],
        RETITLE_A_MILESTONE,
        refuses(1, "write.wrong-shape"),
        at("W3", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-a-link-to-nothing-in-the-docs-place",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing("docs/roadmap.md")))],
        RETITLE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::LinkToNothing),
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-a-link-to-a-file-in-the-repository-in-the-docs-place",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/roadmap.md", outside: false }))],
        RETITLE_A_MILESTONE,
        refuses(1, "write.wrong-shape"),
        at("W3", Kind::LinkInside),
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-a-link-to-a-file-outside-in-the-docs-place",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/roadmap.md", outside: true }))],
        RETITLE_A_MILESTONE,
        refuses(1, "write.wrong-shape"),
        at("W3", Kind::LinkOutside),
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-a-directory-in-the-docs-place",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::Dir("docs/roadmap.md")))],
        RETITLE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::DirForAFile),
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-the-doc-deleted-before-the-first-write",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::Deleted("docs/roadmap.md"))],
        RETITLE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        at("W3", Kind::EditBefore),
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-the-doc-deleted-after-the-first-write",
        &["doc", "retitle-item"],
        HOME,
        &[A_TASK, A_FIRST_WRITE_TO_THE_ROADMAP[0], A_FIRST_WRITE_TO_THE_ROADMAP[1], Lead::Plant(Plant::Deleted("docs/roadmap.md"))],
        RETITLE_A_MILESTONE,
        Listed::Acts,
        at("W3", Kind::EditAfter),
        &[],
    ),
    cell(
        "doc-set-field-with-no-task",
        &["doc", "set-field"],
        FRESH,
        &[],
        &["doc", "set-field", "adr:a-decision#status", "--value", "accepted"],
        refuses(1, "no active task"),
        None,
        &[],
    ),
    cell(
        "doc-set-a-releases-link",
        &["doc", "set-field"],
        HOME,
        &[A_TASK],
        SET_A_RELEASES_LINK,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-set-a-releases-link-a-file-of-the-users-in-the-tasks-area",
        &["doc", "set-field"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::File(".jigc/tasks/sharpen-the-roadmap/notes-of-its-user.md"))],
        SET_A_RELEASES_LINK,
        Listed::Acts,
        at("W1", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-set-a-releases-link-no-baseline-a-file-of-the-users-under-the-state",
        &["doc", "set-field"],
        HOME,
        &[Lead::Under(Condition::NoBaseline), A_TASK, Lead::Plant(Plant::File(".jigc/state/notes-of-its-user.md"))],
        SET_A_RELEASES_LINK,
        Listed::Acts,
        at("W2", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-set-a-releases-link-a-file-of-the-users-in-the-docs-place",
        &["doc", "set-field"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::File("CHANGELOG.md")))],
        SET_A_RELEASES_LINK,
        refuses(1, "write.wrong-shape"),
        at("W3", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-set-a-releases-link-a-link-to-nothing-in-the-docs-place",
        &["doc", "set-field"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing("CHANGELOG.md")))],
        SET_A_RELEASES_LINK,
        refuses(1, "no staged instance for"),
        at("W3", Kind::LinkToNothing),
        &[],
    ),
    cell(
        "doc-set-a-releases-link-a-link-to-a-file-in-the-repository-in-the-docs-place",
        &["doc", "set-field"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "CHANGELOG.md", outside: false }))],
        SET_A_RELEASES_LINK,
        refuses(1, "write.wrong-shape"),
        at("W3", Kind::LinkInside),
        &[],
    ),
    cell(
        "doc-set-a-releases-link-a-link-to-a-file-outside-in-the-docs-place",
        &["doc", "set-field"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "CHANGELOG.md", outside: true }))],
        SET_A_RELEASES_LINK,
        refuses(1, "write.wrong-shape"),
        at("W3", Kind::LinkOutside),
        &[],
    ),
    cell(
        "doc-set-a-releases-link-a-directory-in-the-docs-place",
        &["doc", "set-field"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::InPlaceOf(&Plant::Dir("CHANGELOG.md")))],
        SET_A_RELEASES_LINK,
        refuses(1, "no staged instance for"),
        at("W3", Kind::DirForAFile),
        &[],
    ),
    cell(
        "doc-set-a-releases-link-the-doc-deleted-before-the-first-write",
        &["doc", "set-field"],
        HOME,
        &[A_TASK, Lead::Plant(Plant::Deleted("CHANGELOG.md"))],
        SET_A_RELEASES_LINK,
        refuses(1, "no staged instance for"),
        at("W3", Kind::EditBefore),
        &[],
    ),
    cell(
        "doc-set-a-releases-link-the-doc-deleted-after-the-first-write",
        &["doc", "set-field"],
        HOME,
        &[A_TASK, A_FIRST_WRITE_TO_THE_CHANGELOG, Lead::Plant(Plant::Deleted("CHANGELOG.md"))],
        SET_A_RELEASES_LINK,
        Listed::Acts,
        at("W3", Kind::EditAfter),
        &[],
    ),
    cell(
        "doc-set-slot-with-no-task",
        &["doc", "set-slot"],
        FRESH,
        &[],
        &["doc", "set-slot", "adr:a-decision#context", "--from-file", "payload.yaml"],
        refuses(1, "no active task"),
        None,
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES],
        SET_WHAT_IT_PROVES,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-a-file-of-the-users-in-the-tasks-area",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES, Lead::Plant(Plant::File(".jigc/tasks/sharpen-the-roadmap/notes-of-its-user.md"))],
        SET_WHAT_IT_PROVES,
        Listed::Acts,
        at("W1", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-no-baseline-a-file-of-the-users-under-the-state",
        &["doc", "set-slot"],
        HOME,
        &[Lead::Under(Condition::NoBaseline), A_TASK, WHAT_IT_PROVES, Lead::Plant(Plant::File(".jigc/state/notes-of-its-user.md"))],
        SET_WHAT_IT_PROVES,
        Listed::Acts,
        at("W2", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-a-file-of-the-users-in-the-docs-place",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES, Lead::Plant(Plant::InPlaceOf(&Plant::File("docs/roadmap.md")))],
        SET_WHAT_IT_PROVES,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-a-link-to-nothing-in-the-docs-place",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing("docs/roadmap.md")))],
        SET_WHAT_IT_PROVES,
        refuses(1, "no staged instance for"),
        at("W3", Kind::LinkToNothing),
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-a-link-to-a-file-in-the-repository-in-the-docs-place",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/roadmap.md", outside: false }))],
        SET_WHAT_IT_PROVES,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkInside),
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-a-link-to-a-file-outside-in-the-docs-place",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/roadmap.md", outside: true }))],
        SET_WHAT_IT_PROVES,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkOutside),
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-a-directory-in-the-docs-place",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES, Lead::Plant(Plant::InPlaceOf(&Plant::Dir("docs/roadmap.md")))],
        SET_WHAT_IT_PROVES,
        refuses(1, "no staged instance for"),
        at("W3", Kind::DirForAFile),
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-the-doc-deleted-before-the-first-write",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES, Lead::Plant(Plant::Deleted("docs/roadmap.md"))],
        SET_WHAT_IT_PROVES,
        refuses(1, "no staged instance for"),
        at("W3", Kind::EditBefore),
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-the-doc-deleted-after-the-first-write",
        &["doc", "set-slot"],
        HOME,
        &[A_TASK, WHAT_IT_PROVES, A_FIRST_WRITE_TO_THE_ROADMAP[1], Lead::Plant(Plant::Deleted("docs/roadmap.md"))],
        SET_WHAT_IT_PROVES,
        Listed::Acts,
        at("W3", Kind::EditAfter),
        &[],
    ),
    cell(
        "doc-rename-with-no-task",
        &["doc", "rename"],
        FRESH,
        &[],
        &["doc", "rename", "adr:a-decision", "--to", "Another Decision"],
        refuses(1, "no active task"),
        None,
        &[],
    ),
    cell(
        "doc-rename-a-decision-the-task-made",
        &["doc", "rename"],
        HOME,
        &[A_TASK_FOR_TWO_WAYS, TWO_WAYS],
        RENAME_TWO_WAYS,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-rename-a-decision-the-task-made-a-file-of-the-users-in-the-tasks-area",
        &["doc", "rename"],
        HOME,
        &[A_TASK_FOR_TWO_WAYS, TWO_WAYS, Lead::Plant(Plant::File(".jigc/tasks/record-two-ways/notes-of-its-user.md"))],
        RENAME_TWO_WAYS,
        Listed::Acts,
        at("W1", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-retitle-a-committed-decision-no-baseline-a-file-of-the-users-under-the-state",
        &["doc", "rename"],
        HOME,
        &[LANDS_ONE_WAY[0], LANDS_ONE_WAY[1], LANDS_ONE_WAY[2], LANDS_ONE_WAY[3], LANDS_ONE_WAY[4], LANDS_ONE_WAY[5], LANDS_ONE_WAY[6], LANDS_ONE_WAY[7], LANDS_ONE_WAY[8], LANDS_ONE_WAY[9], LANDS_ONE_WAY[10], LANDS_ONE_WAY[11], Lead::Under(Condition::NoBaseline), A_TASK_TO_RETITLE, Lead::Plant(Plant::File(".jigc/state/notes-of-its-user.md"))],
        RETITLE_ONE_WAY,
        Listed::Acts,
        at("W2", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-retitle-a-committed-decision-a-file-of-the-users-in-the-docs-place",
        &["doc", "rename"],
        HOME,
        &[LANDS_ONE_WAY[0], LANDS_ONE_WAY[1], LANDS_ONE_WAY[2], LANDS_ONE_WAY[3], LANDS_ONE_WAY[4], LANDS_ONE_WAY[5], LANDS_ONE_WAY[6], LANDS_ONE_WAY[7], LANDS_ONE_WAY[8], LANDS_ONE_WAY[9], LANDS_ONE_WAY[10], LANDS_ONE_WAY[11], A_TASK_TO_RETITLE, Lead::Plant(Plant::InPlaceOf(&Plant::File("docs/decisions/one-way.md")))],
        RETITLE_ONE_WAY,
        refuses(1, "carries no `# H1` to retitle"),
        at("W3", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-retitle-a-committed-decision-a-link-to-nothing-in-the-docs-place",
        &["doc", "rename"],
        HOME,
        &[LANDS_ONE_WAY[0], LANDS_ONE_WAY[1], LANDS_ONE_WAY[2], LANDS_ONE_WAY[3], LANDS_ONE_WAY[4], LANDS_ONE_WAY[5], LANDS_ONE_WAY[6], LANDS_ONE_WAY[7], LANDS_ONE_WAY[8], LANDS_ONE_WAY[9], LANDS_ONE_WAY[10], LANDS_ONE_WAY[11], A_TASK_TO_RETITLE, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing("docs/decisions/one-way.md")))],
        RETITLE_ONE_WAY,
        refuses(1, "no staged instance for"),
        at("W3", Kind::LinkToNothing),
        &[],
    ),
    cell(
        "doc-retitle-a-committed-decision-a-link-to-a-file-in-the-repository-in-the-docs-place",
        &["doc", "rename"],
        HOME,
        &[LANDS_ONE_WAY[0], LANDS_ONE_WAY[1], LANDS_ONE_WAY[2], LANDS_ONE_WAY[3], LANDS_ONE_WAY[4], LANDS_ONE_WAY[5], LANDS_ONE_WAY[6], LANDS_ONE_WAY[7], LANDS_ONE_WAY[8], LANDS_ONE_WAY[9], LANDS_ONE_WAY[10], LANDS_ONE_WAY[11], A_TASK_TO_RETITLE, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/decisions/one-way.md", outside: false }))],
        RETITLE_ONE_WAY,
        refuses(1, "carries no `# H1` to retitle"),
        at("W3", Kind::LinkInside),
        &[],
    ),
    cell(
        "doc-retitle-a-committed-decision-a-link-to-a-file-outside-in-the-docs-place",
        &["doc", "rename"],
        HOME,
        &[LANDS_ONE_WAY[0], LANDS_ONE_WAY[1], LANDS_ONE_WAY[2], LANDS_ONE_WAY[3], LANDS_ONE_WAY[4], LANDS_ONE_WAY[5], LANDS_ONE_WAY[6], LANDS_ONE_WAY[7], LANDS_ONE_WAY[8], LANDS_ONE_WAY[9], LANDS_ONE_WAY[10], LANDS_ONE_WAY[11], A_TASK_TO_RETITLE, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "docs/decisions/one-way.md", outside: true }))],
        RETITLE_ONE_WAY,
        refuses(1, "carries no `# H1` to retitle"),
        at("W3", Kind::LinkOutside),
        &[],
    ),
    cell(
        "doc-retitle-a-committed-decision-a-directory-in-the-docs-place",
        &["doc", "rename"],
        HOME,
        &[LANDS_ONE_WAY[0], LANDS_ONE_WAY[1], LANDS_ONE_WAY[2], LANDS_ONE_WAY[3], LANDS_ONE_WAY[4], LANDS_ONE_WAY[5], LANDS_ONE_WAY[6], LANDS_ONE_WAY[7], LANDS_ONE_WAY[8], LANDS_ONE_WAY[9], LANDS_ONE_WAY[10], LANDS_ONE_WAY[11], A_TASK_TO_RETITLE, Lead::Plant(Plant::InPlaceOf(&Plant::Dir("docs/decisions/one-way.md")))],
        RETITLE_ONE_WAY,
        refuses(1, "no staged instance for"),
        at("W3", Kind::DirForAFile),
        &[],
    ),
    cell(
        "doc-retitle-a-committed-decision-the-doc-deleted-before-the-first-write",
        &["doc", "rename"],
        HOME,
        &[LANDS_ONE_WAY[0], LANDS_ONE_WAY[1], LANDS_ONE_WAY[2], LANDS_ONE_WAY[3], LANDS_ONE_WAY[4], LANDS_ONE_WAY[5], LANDS_ONE_WAY[6], LANDS_ONE_WAY[7], LANDS_ONE_WAY[8], LANDS_ONE_WAY[9], LANDS_ONE_WAY[10], LANDS_ONE_WAY[11], A_TASK_TO_RETITLE, Lead::Plant(Plant::Deleted("docs/decisions/one-way.md"))],
        RETITLE_ONE_WAY,
        refuses(1, "no staged instance for"),
        at("W3", Kind::EditBefore),
        &[],
    ),
    cell(
        "doc-retitle-a-committed-decision-the-doc-deleted-after-the-first-write",
        &["doc", "rename"],
        HOME,
        &[LANDS_ONE_WAY[0], LANDS_ONE_WAY[1], LANDS_ONE_WAY[2], LANDS_ONE_WAY[3], LANDS_ONE_WAY[4], LANDS_ONE_WAY[5], LANDS_ONE_WAY[6], LANDS_ONE_WAY[7], LANDS_ONE_WAY[8], LANDS_ONE_WAY[9], LANDS_ONE_WAY[10], LANDS_ONE_WAY[11], A_TASK_TO_RETITLE, A_FIRST_WRITE_TO_THE_DECISION, Lead::Plant(Plant::Deleted("docs/decisions/one-way.md"))],
        RETITLE_ONE_WAY,
        Listed::Acts,
        at("W3", Kind::EditAfter),
        &[],
    ),
];

/// **The base cells of the six edit leaves in the prepared states that are not their
/// home** — on demand, in no gate: the same task and the same command in each. Where no
/// committed roadmap or changelog stands there is no instance to write, and the write
/// refuses; `bare` has no task at all.
const EDITS_ELSEWHERE: [Cell; 36] = [
    cell(
        "doc-add-a-milestone-in-bare",
        &["doc", "add-item"],
        Prepared::Bare,
        &[],
        ADD_A_MILESTONE,
        refuses(1, "finalize.no-task"),
        None,
        &[],
    ),
    cell(
        "doc-add-a-milestone-in-fresh",
        &["doc", "add-item"],
        FRESH,
        &[A_TASK],
        ADD_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-add-a-milestone-in-migrated",
        &["doc", "add-item"],
        Prepared::Of(State::Migrated),
        &[A_TASK],
        ADD_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-add-a-milestone-in-refs-post-hoc",
        &["doc", "add-item"],
        Prepared::Of(State::RefsPostHoc),
        &[A_TASK],
        ADD_A_MILESTONE,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-add-a-milestone-in-chatty-hooks",
        &["doc", "add-item"],
        Prepared::Of(State::ChattyHooks),
        &[A_TASK],
        ADD_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-add-a-milestone-in-vendored",
        &["doc", "add-item"],
        Prepared::Of(State::Vendored),
        &[A_TASK],
        ADD_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-remove-a-milestone-in-bare",
        &["doc", "remove-item"],
        Prepared::Bare,
        &[],
        REMOVE_A_MILESTONE,
        refuses(1, "finalize.no-task"),
        None,
        &[],
    ),
    cell(
        "doc-remove-a-milestone-in-fresh",
        &["doc", "remove-item"],
        FRESH,
        &[A_TASK],
        REMOVE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-remove-a-milestone-in-migrated",
        &["doc", "remove-item"],
        Prepared::Of(State::Migrated),
        &[A_TASK],
        REMOVE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-remove-a-milestone-in-refs-post-hoc",
        &["doc", "remove-item"],
        Prepared::Of(State::RefsPostHoc),
        &[A_TASK],
        REMOVE_A_MILESTONE,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-remove-a-milestone-in-chatty-hooks",
        &["doc", "remove-item"],
        Prepared::Of(State::ChattyHooks),
        &[A_TASK],
        REMOVE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-remove-a-milestone-in-vendored",
        &["doc", "remove-item"],
        Prepared::Of(State::Vendored),
        &[A_TASK],
        REMOVE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-in-bare",
        &["doc", "retitle-item"],
        Prepared::Bare,
        &[],
        RETITLE_A_MILESTONE,
        refuses(1, "finalize.no-task"),
        None,
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-in-fresh",
        &["doc", "retitle-item"],
        FRESH,
        &[A_TASK],
        RETITLE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-in-migrated",
        &["doc", "retitle-item"],
        Prepared::Of(State::Migrated),
        &[A_TASK],
        RETITLE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-in-refs-post-hoc",
        &["doc", "retitle-item"],
        Prepared::Of(State::RefsPostHoc),
        &[A_TASK],
        RETITLE_A_MILESTONE,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-in-chatty-hooks",
        &["doc", "retitle-item"],
        Prepared::Of(State::ChattyHooks),
        &[A_TASK],
        RETITLE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-retitle-a-milestone-in-vendored",
        &["doc", "retitle-item"],
        Prepared::Of(State::Vendored),
        &[A_TASK],
        RETITLE_A_MILESTONE,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-set-a-releases-link-in-bare",
        &["doc", "set-field"],
        Prepared::Bare,
        &[],
        SET_A_RELEASES_LINK,
        refuses(1, "finalize.no-task"),
        None,
        &[],
    ),
    cell(
        "doc-set-a-releases-link-in-fresh",
        &["doc", "set-field"],
        FRESH,
        &[A_TASK],
        SET_A_RELEASES_LINK,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-set-a-releases-link-in-migrated",
        &["doc", "set-field"],
        Prepared::Of(State::Migrated),
        &[A_TASK],
        SET_A_RELEASES_LINK,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-set-a-releases-link-in-refs-post-hoc",
        &["doc", "set-field"],
        Prepared::Of(State::RefsPostHoc),
        &[A_TASK],
        SET_A_RELEASES_LINK,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-set-a-releases-link-in-chatty-hooks",
        &["doc", "set-field"],
        Prepared::Of(State::ChattyHooks),
        &[A_TASK],
        SET_A_RELEASES_LINK,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-set-a-releases-link-in-vendored",
        &["doc", "set-field"],
        Prepared::Of(State::Vendored),
        &[A_TASK],
        SET_A_RELEASES_LINK,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-in-bare",
        &["doc", "set-slot"],
        Prepared::Bare,
        &[],
        SET_WHAT_IT_PROVES,
        refuses(1, "finalize.no-task"),
        None,
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-in-fresh",
        &["doc", "set-slot"],
        FRESH,
        &[A_TASK, WHAT_IT_PROVES],
        SET_WHAT_IT_PROVES,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-in-migrated",
        &["doc", "set-slot"],
        Prepared::Of(State::Migrated),
        &[A_TASK, WHAT_IT_PROVES],
        SET_WHAT_IT_PROVES,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-in-refs-post-hoc",
        &["doc", "set-slot"],
        Prepared::Of(State::RefsPostHoc),
        &[A_TASK, WHAT_IT_PROVES],
        SET_WHAT_IT_PROVES,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-in-chatty-hooks",
        &["doc", "set-slot"],
        Prepared::Of(State::ChattyHooks),
        &[A_TASK, WHAT_IT_PROVES],
        SET_WHAT_IT_PROVES,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-set-what-a-milestone-proves-in-vendored",
        &["doc", "set-slot"],
        Prepared::Of(State::Vendored),
        &[A_TASK, WHAT_IT_PROVES],
        SET_WHAT_IT_PROVES,
        refuses(1, "no staged instance for"),
        None,
        &[],
    ),
    cell(
        "doc-rename-a-decision-the-task-made-in-bare",
        &["doc", "rename"],
        Prepared::Bare,
        &[],
        RENAME_TWO_WAYS,
        refuses(1, "finalize.no-task"),
        None,
        &[],
    ),
    cell(
        "doc-rename-a-decision-the-task-made-in-fresh",
        &["doc", "rename"],
        FRESH,
        &[A_TASK_FOR_TWO_WAYS, TWO_WAYS],
        RENAME_TWO_WAYS,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-rename-a-decision-the-task-made-in-migrated",
        &["doc", "rename"],
        Prepared::Of(State::Migrated),
        &[A_TASK_FOR_TWO_WAYS, TWO_WAYS],
        RENAME_TWO_WAYS,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-rename-a-decision-the-task-made-in-refs-post-hoc",
        &["doc", "rename"],
        Prepared::Of(State::RefsPostHoc),
        &[A_TASK_FOR_TWO_WAYS, TWO_WAYS],
        RENAME_TWO_WAYS,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-rename-a-decision-the-task-made-in-chatty-hooks",
        &["doc", "rename"],
        Prepared::Of(State::ChattyHooks),
        &[A_TASK_FOR_TWO_WAYS, TWO_WAYS],
        RENAME_TWO_WAYS,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-rename-a-decision-the-task-made-in-vendored",
        &["doc", "rename"],
        Prepared::Of(State::Vendored),
        &[A_TASK_FOR_TWO_WAYS, TWO_WAYS],
        RENAME_TWO_WAYS,
        Listed::Acts,
        None,
        &[],
    ),
];


/// **The task the minting cells write in** — `form-the-project-vision`, on `form-vision`,
/// whose create-gate admits the `vision`, which `committed-singletons` holds committed at
/// `VISION.md`: a create or an author of it copies the committed body in for update.
const A_TASK_FOR_THE_VISION: Lead =
    jigc(&["start", "--workflow", "form-vision", "form the project vision"]);

/// The batch payload `doc author` reads with `--from-file`: the vision's title and one
/// slot.
const THE_VISIONS_PAYLOAD: Lead = payload(
    "vision.yaml",
    "title: Vision\nsections:\n  - id: open-questions\n    set:\n      open-questions: |-\n        \
     <<Which domains earn a pack of their own, sharpened.>>\n",
);

/// **The task's first write to the vision**: a slot, which copies it in.
const A_FIRST_WRITE_TO_THE_VISION: [Lead; 2] = [
    payload("thesis", "A thesis, sharpened.\n"),
    jigc(&[
        "doc",
        "set-slot",
        "vision:vision#thesis",
        "--from-file",
        "../outside/thesis",
        "--task",
        "form-the-project-vision",
    ]),
];

/// The two minting commands, each over the committed vision.
const CREATE_THE_VISION: &[&str] = &[
    "doc",
    "create",
    "vision",
    "--title",
    "Vision",
    "--task",
    "form-the-project-vision",
];
const AUTHOR_THE_VISION: &[&str] = &[
    "doc",
    "author",
    "vision",
    "--from-file",
    "../outside/vision.yaml",
    "--task",
    "form-the-project-vision",
];

/// **The cells of the two minting doors** — the family's second part (the record's §3b),
/// each listed from the rows' docs. A create over a doc that is already one copies it in
/// for update; over a home with no doc body — a link to nothing, a directory — it *"still
/// mints fresh and is refused at the committing door, not at the create"*
/// (`design/finalize.md` → *4. Promote managed docs*, *Declared bounds*, (4)). Over the
/// doc's file deleted the home is no free one — git still holds the doc — and the docs say
/// so in their own words: *"Every probe that asks the disk reads that home as free — the
/// create included, so the doc is `created`"*, and the committing door refuses it
/// (the same section, *A home git holds is occupied*), as `finalize.promote-clobber` does
/// at *"a path git still holds — a committed doc deleted from the worktree"*
/// (`design/command-output-contract.md` → *The membership test*, the `finalize.*` table's
/// `promote-clobber` row). `doc author` runs through the same `create_gated` path, so the
/// same holds of it. A file that is no doc of its type at the home is the record's §6,
/// entry 14, ruled on 2026-10-18: both doors refuse before the copy-in, staging and
/// binding nothing — `create` acts today, and `author` refuses and leaves its role bound,
/// where its own doc has a refusal persist nothing (entry 15, ruled the same day for every
/// `doc` write). The three of `author` refuse `write.non-reparseable` and print *"nothing
/// was persisted"* over the role they leave bound
/// — the findings' key of §12, which no cell holds, as at the edit leaves ([`EDITS`]).
const MINTS: [Cell; 22] = [
    cell(
        "doc-create-with-no-task",
        &["doc", "create"],
        FRESH,
        &[],
        &["doc", "create", "adr", "--title", "A Decision"],
        refuses(1, "no active task"),
        None,
        &[],
    ),
    cell(
        "doc-create-the-vision",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION],
        CREATE_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-create-the-vision-a-file-of-the-users-in-the-tasks-area",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION, Lead::Plant(Plant::File(".jigc/tasks/form-the-project-vision/notes-of-its-user.md"))],
        CREATE_THE_VISION,
        Listed::Acts,
        at("W1", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-create-the-vision-no-baseline-a-file-of-the-users-under-the-state",
        &["doc", "create"],
        HOME,
        &[Lead::Under(Condition::NoBaseline), A_TASK_FOR_THE_VISION, Lead::Plant(Plant::File(".jigc/state/notes-of-its-user.md"))],
        CREATE_THE_VISION,
        Listed::Acts,
        at("W2", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-create-the-vision-a-file-of-the-users-in-the-docs-place",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION, Lead::Plant(Plant::InPlaceOf(&Plant::File("VISION.md")))],
        CREATE_THE_VISION,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-create-the-vision-a-link-to-nothing-in-the-docs-place",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing("VISION.md")))],
        CREATE_THE_VISION,
        Listed::Acts,
        at("W3", Kind::LinkToNothing),
        &[],
    ),
    cell(
        "doc-create-the-vision-a-link-to-a-file-in-the-repository-in-the-docs-place",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "VISION.md", outside: false }))],
        CREATE_THE_VISION,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkInside),
        &[],
    ),
    cell(
        "doc-create-the-vision-a-link-to-a-file-outside-in-the-docs-place",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "VISION.md", outside: true }))],
        CREATE_THE_VISION,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkOutside),
        &[],
    ),
    cell(
        "doc-create-the-vision-a-directory-in-the-docs-place",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION, Lead::Plant(Plant::InPlaceOf(&Plant::Dir("VISION.md")))],
        CREATE_THE_VISION,
        Listed::Acts,
        at("W3", Kind::DirForAFile),
        &[],
    ),
    cell(
        "doc-create-the-vision-the-doc-deleted-before-the-first-write",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION, Lead::Plant(Plant::Deleted("VISION.md"))],
        CREATE_THE_VISION,
        Listed::Acts,
        at("W3", Kind::EditBefore),
        &[],
    ),
    cell(
        "doc-create-the-vision-the-doc-deleted-after-the-first-write",
        &["doc", "create"],
        HOME,
        &[A_TASK_FOR_THE_VISION, A_FIRST_WRITE_TO_THE_VISION[0], A_FIRST_WRITE_TO_THE_VISION[1], Lead::Plant(Plant::Deleted("VISION.md"))],
        CREATE_THE_VISION,
        Listed::Acts,
        at("W3", Kind::EditAfter),
        &[],
    ),
    cell(
        "doc-author-with-no-task",
        &["doc", "author"],
        FRESH,
        &[],
        &["doc", "author", "adr", "--from-file", "payload.yaml"],
        refuses(1, "no active task"),
        None,
        &[],
    ),
    cell(
        "doc-author-the-vision",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD],
        AUTHOR_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-author-the-vision-a-file-of-the-users-in-the-tasks-area",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, Lead::Plant(Plant::File(".jigc/tasks/form-the-project-vision/notes-of-its-user.md"))],
        AUTHOR_THE_VISION,
        Listed::Acts,
        at("W1", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-author-the-vision-no-baseline-a-file-of-the-users-under-the-state",
        &["doc", "author"],
        HOME,
        &[Lead::Under(Condition::NoBaseline), A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, Lead::Plant(Plant::File(".jigc/state/notes-of-its-user.md"))],
        AUTHOR_THE_VISION,
        Listed::Acts,
        at("W2", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-author-the-vision-a-file-of-the-users-in-the-docs-place",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, Lead::Plant(Plant::InPlaceOf(&Plant::File("VISION.md")))],
        AUTHOR_THE_VISION,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::Untracked),
        &[],
    ),
    cell(
        "doc-author-the-vision-a-link-to-nothing-in-the-docs-place",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing("VISION.md")))],
        AUTHOR_THE_VISION,
        Listed::Acts,
        at("W3", Kind::LinkToNothing),
        &[],
    ),
    cell(
        "doc-author-the-vision-a-link-to-a-file-in-the-repository-in-the-docs-place",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "VISION.md", outside: false }))],
        AUTHOR_THE_VISION,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkInside),
        &[],
    ),
    cell(
        "doc-author-the-vision-a-link-to-a-file-outside-in-the-docs-place",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile { at: "VISION.md", outside: true }))],
        AUTHOR_THE_VISION,
        refuses(1, "write.non-reparseable"),
        at("W3", Kind::LinkOutside),
        &[],
    ),
    cell(
        "doc-author-the-vision-a-directory-in-the-docs-place",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, Lead::Plant(Plant::InPlaceOf(&Plant::Dir("VISION.md")))],
        AUTHOR_THE_VISION,
        Listed::Acts,
        at("W3", Kind::DirForAFile),
        &[],
    ),
    cell(
        "doc-author-the-vision-the-doc-deleted-before-the-first-write",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, Lead::Plant(Plant::Deleted("VISION.md"))],
        AUTHOR_THE_VISION,
        Listed::Acts,
        at("W3", Kind::EditBefore),
        &[],
    ),
    cell(
        "doc-author-the-vision-the-doc-deleted-after-the-first-write",
        &["doc", "author"],
        HOME,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD, A_FIRST_WRITE_TO_THE_VISION[0], A_FIRST_WRITE_TO_THE_VISION[1], Lead::Plant(Plant::Deleted("VISION.md"))],
        AUTHOR_THE_VISION,
        Listed::Acts,
        at("W3", Kind::EditAfter),
        &[],
    ),
];

/// **The base cells of the two minting doors in the prepared states that are not their
/// home** — on demand, in no gate: the same task and the same command in each; where no
/// vision stands, the create mints one.
const MINTS_ELSEWHERE: [Cell; 12] = [
    cell(
        "doc-create-the-vision-in-bare",
        &["doc", "create"],
        Prepared::Bare,
        &[],
        CREATE_THE_VISION,
        refuses(1, "finalize.no-task"),
        None,
        &[],
    ),
    cell(
        "doc-create-the-vision-in-fresh",
        &["doc", "create"],
        FRESH,
        &[A_TASK_FOR_THE_VISION],
        CREATE_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-create-the-vision-in-migrated",
        &["doc", "create"],
        Prepared::Of(State::Migrated),
        &[A_TASK_FOR_THE_VISION],
        CREATE_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-create-the-vision-in-refs-post-hoc",
        &["doc", "create"],
        Prepared::Of(State::RefsPostHoc),
        &[A_TASK_FOR_THE_VISION],
        CREATE_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-create-the-vision-in-chatty-hooks",
        &["doc", "create"],
        Prepared::Of(State::ChattyHooks),
        &[A_TASK_FOR_THE_VISION],
        CREATE_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-create-the-vision-in-vendored",
        &["doc", "create"],
        Prepared::Of(State::Vendored),
        &[A_TASK_FOR_THE_VISION],
        CREATE_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-author-the-vision-in-bare",
        &["doc", "author"],
        Prepared::Bare,
        &[],
        AUTHOR_THE_VISION,
        refuses(1, "finalize.no-task"),
        None,
        &[],
    ),
    cell(
        "doc-author-the-vision-in-fresh",
        &["doc", "author"],
        FRESH,
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD],
        AUTHOR_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-author-the-vision-in-migrated",
        &["doc", "author"],
        Prepared::Of(State::Migrated),
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD],
        AUTHOR_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-author-the-vision-in-refs-post-hoc",
        &["doc", "author"],
        Prepared::Of(State::RefsPostHoc),
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD],
        AUTHOR_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-author-the-vision-in-chatty-hooks",
        &["doc", "author"],
        Prepared::Of(State::ChattyHooks),
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD],
        AUTHOR_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
    cell(
        "doc-author-the-vision-in-vendored",
        &["doc", "author"],
        Prepared::Of(State::Vendored),
        &[A_TASK_FOR_THE_VISION, THE_VISIONS_PAYLOAD],
        AUTHOR_THE_VISION,
        Listed::Acts,
        None,
        &[],
    ),
];

/// The family's cells, a part after another.
pub const CELLS: &[&[Cell]] = &[&EDITS, &EDITS_ELSEWHERE, &MINTS, &MINTS_ELSEWHERE];
