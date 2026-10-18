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

use super::ON_A_REFUSAL;
use crate::support::byte_cell::{Allows, Cell, Line, May, Prepared, Quote, Row, When, Where};
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
/// name a re-slug leaves.
const W4: Line = line(
    "W4",
    Allows::Files(Where::Path(".jigc/tasks/*/docs/*"), May::Removed),
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

/// The family's cells — owed by the task `B6`, whose later commits bring them.
pub const CELLS: &[&[Cell]] = &[];
