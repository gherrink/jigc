//! **The byte table's rows of the mint family** (`crates/cli/tests/byte_table.rs`) — `jigc
//! start`, `jigc workflow`, `jigc migrate` and `jigc task amend`, lifted quote for quote from
//! the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md` →
//! §4d), under the rules the install family's file states. Each starts work, or re-enters
//! it, and commits nothing: `start`, `migrate` and `amend` mint a task — an area under
//! `.jigc/tasks/` that carries its base pin — and `workflow` re-enters one, provisioning a
//! sub-task's area on its first entry; a read of the committed docs rebuilds the edge index
//! where its file is gone or its stamp stale.
//!
//! **What of the record is no line.** What each door refuses before it writes; and the three
//! second spellings — `jigc start` bare and `jigc start --task <id>`, which mint nothing, and
//! `jigc workflow <W> --preview`, which writes nothing — each a spelling that only takes a
//! line out of force.

use super::ON_A_REFUSAL;
use crate::support::byte_cell::{
    Allows, Cell, Line, May, Prepared, Quote, Row, Spelling, THE_SOURCE, Unit, When, Where,
};
use crate::support::trial_corpus::State;

/// A line that holds where the command acted.
const fn line(id: &'static str, allows: Allows, quotes: &'static [Quote]) -> Line {
    Line {
        id,
        allows,
        when: When::Always,
        on_refusal: false,
        quotes,
        spelling: None,
    }
}

/// The heading of the task origination section of `design/write-commands.md`.
const TASK_ORIGINATION: &str = "Task origination";

/// The heading of the `jigc migrate` verb's section of `design/auto-migration.md`.
const THE_MIGRATE_VERB: &str = "The `jigc migrate` verb (the trigger)";

/// **A mint snapshots the staged state into its area** — `design/finalize.md`'s sentence on
/// the task-minting doors.
const THE_STAGED_SNAPSHOT: Quote = Quote {
    doc: "design/finalize.md",
    heading: "Dirty-tree policy",
    words: "Each of the three task-minting doors (`jigc start` compose forms, `jigc migrate`, `jigc milestone create` for the shared checkout's aggregate index) snapshots the pre-task staged state — the staged set (path + blob) **and the staged-deletion set** — into the working area.",
};

/// **The read that finds the edge index stale rebuilds it** — `design/storage.md`'s row of
/// the index's lifecycle, as `jigc milestone join`'s `MJ4` quotes it.
const THE_COMMITTED_REBUILD: Quote = Quote {
    doc: "design/storage.md",
    heading: "Edge index lifecycle",
    words: "| **committed rebuild** | first read after a stamp mismatch (branch switch, pull, rebase) — rebuild from committed `.md`s | atomic; persists in `.jigc/index/` with the new stamp |",
};

/// **`jigc start`** — the record's §4d. Its unit is the task `--task` names, and else the
/// single task that stands; its home is `fresh`. **Two spellings**, each taking `ST1` out
/// of force: `bare`, the orientation, which never mints, and `--task`, the resume.
pub const START: Row = Row {
    leaf: &["start"],
    unit: Unit::TheTaskFlag,
    spellings: &[
        Spelling {
            name: "bare",
            replaces: &["ST1"],
        },
        Spelling {
            name: "--task",
            replaces: &["ST1"],
        },
    ],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "ST1",
            Allows::Files(Where::Under(".jigc/tasks/<minted-task>"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/write-commands.md",
                    heading: TASK_ORIGINATION,
                    words: "frozen, collision-suffixed under the same discipline as artifacts; pinned to base = HEAD; opens `.jigc/tasks/<id>/`.",
                },
                Quote {
                    doc: "design/write-commands.md",
                    heading: "The verbs",
                    words: "`jigc start` itself stages `commit:<id>.md`, so the population the guard fires over is every minted task, pristine skeleton included",
                },
                THE_STAGED_SNAPSHOT,
            ],
        ),
        line(
            "ST2",
            Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/write-commands.md",
                    heading: TASK_ORIGINATION,
                    words: "Never mints, never composes a side-effectful workflow; reporting a live task's findings does run the task-scope sweep, so it reads git and materializes the derived edge-index cache.",
                },
                THE_COMMITTED_REBUILD,
            ],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// **`jigc workflow`** — the record's §4d. Its unit is the task `--task` names; its home
/// is `fresh`, with the leads that mint a milestone of one sub-task and provision its
/// worktree, where the re-entry runs. **One spelling**, `--preview`, which takes both lines
/// out of force.
pub const WORKFLOW: Row = Row {
    leaf: &["workflow"],
    unit: Unit::TheTaskFlag,
    spellings: &[Spelling {
        name: "--preview",
        replaces: &["WF1", "WF2"],
    }],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "WF1",
            Allows::Files(Where::Under(".jigc/tasks/<unit>"), May::Provisioned),
            &[
                Quote {
                    doc: "design/write-commands.md",
                    heading: "Sub-agent re-entry — `jigc workflow <W> --task <id>` (M8)",
                    words: "The first `jigc workflow <W> --task <id>` re-entry **provisions** the sub-workflow's deterministic instances (the commit doc, any workflow-provisioned docs) into the sub-area — mirroring how `jigc start` provisions at mint time, just deferred to the sub-agent's first entry rather than at `add-task` (so areas that are never spawned are never provisioned).",
                },
                Quote {
                    doc: "design/write-commands.md",
                    heading: "The write-time `--task`-scoped barrier (M8)",
                    words: "`provision_commit_doc` likewise records `created` provenance under M8 — a per-sub-task commit doc must appear in the provenance manifest the join consumes",
                },
            ],
        ),
        line(
            "WF2",
            Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
            &[THE_COMMITTED_REBUILD],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// **`jigc migrate`** — the record's §4d. It names no unit — it mints one; its home is
/// `fresh`, with the leads that commit a foreign file to migrate.
pub const MIGRATE: Row = Row {
    leaf: &["migrate"],
    unit: Unit::Nothing,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "MI1",
            Allows::Files(Where::Under(".jigc/tasks/<minted-task>"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/auto-migration.md",
                    heading: THE_MIGRATE_VERB,
                    words: "Mints an **off-router migration task** (mirrors `record-change`: `creates-task: true`, `selectable: false`, `allows-create: [{type: changelog, as: changelog}]`), recording the foreign source path on the task.",
                },
                Quote {
                    doc: "design/auto-migration.md",
                    heading: "The source seam (foreign content → composed context)",
                    words: "**Settled: a CLI-owned source seam** — the `migrate` verb stages the foreign bytes into the task (a task-scoped source artifact) and a new placeholder surfaces them into the composed migration workflow as deterministic context.",
                },
                THE_STAGED_SNAPSHOT,
            ],
        ),
        line(
            "MI2",
            Allows::Files(Where::Path(THE_SOURCE), May::CopiedFrom),
            &[Quote {
                doc: "design/auto-migration.md",
                heading: THE_MIGRATE_VERB,
                words: "Reads the foreign file's bytes.",
            }],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// **`jigc task amend`** — the record's §4d. It names no unit — it mints one; its home is
/// `fresh`, whose `HEAD` is an ordinary commit an amend can rewrite.
pub const TASK_AMEND: Row = Row {
    leaf: &["task", "amend"],
    unit: Unit::Nothing,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "AM1",
            Allows::Files(Where::Under(".jigc/tasks/<minted-task>"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/write-commands.md",
                    heading: TASK_ORIGINATION,
                    words: "**`jigc task amend [\"<intent>\"]`** mints a task whose base pin is **`HEAD`**, writes an `amend` marker holding that commit's sha, provisions an **empty** `commit` doc, and composes the dev pack's verb-routed `amend` workflow",
                },
                Quote {
                    doc: "design/write-commands.md",
                    heading: TASK_ORIGINATION,
                    words: "It **commits nothing**: it is `BEHALF_DOORS`' `Neither`, and `jigc task finalize <id>` is what rewrites the commit, on its own second commit model",
                },
            ],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// The family's rows.
pub const ROWS: &[Row] = &[START, WORKFLOW, MIGRATE, TASK_AMEND];

/// The family's cells: none yet — they come with the task's next commits, and the four
/// leaves stand on the list of leaves whose cells are owed until then.
pub const CELLS: &[&[Cell]] = &[];
