//! **The byte table's row for `jigc task finalize`** (`crates/cli/tests/byte_table.rs`) —
//! lifted quote for quote from the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`,
//! §3), under the rules the install family's file states. `jigc task discard` joins this
//! file with its row in the plan's task `B4`.
//!
//! **What of the record is no line.** `T7` is where *on a refusal: no* comes from. `T8` is
//! the rule for prose handed to jigc, which the frame holds a staged doc to by itself
//! (`support::byte_cell` → *a staged doc*). **The doc-only arm is a second spelling** with
//! a commit line of its own — a task whose workflow includes `step:finalize-doc-only`
//! commits its own paths and never the index — and no cell names it yet: the comparison
//! has no switch for it, and a cell of such a task would be read under `T4`.

use super::ON_A_REFUSAL;
use crate::support::byte_cell::{
    Allows, Cell, Condition, Holding, Lead, Line, Listed, May, Plant, Prepared, Quote, Row, When, Where,
};
use crate::support::route_case::{Place, Step};
use crate::support::trial_corpus::State;

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

/// **`jigc task finalize`** — the record's §3. The record names no consent of its own.
pub const FINALIZE: Row = Row {
    leaf: &["task", "finalize"],
    consent: None,
    lines: &[
        line(
            "T1",
            Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "4. Promote managed docs",
                    words: "- **Copy** (not move) the file from the working area to its canonical path.",
                },
                Quote {
                    doc: "design/finalize.md",
                    heading: "4. Promote managed docs",
                    words: "**A promote lands a regular file at exactly its canonical path, or the transaction refuses before anything is written",
                },
                Quote {
                    doc: "design/finalize.md",
                    heading: "4. Promote managed docs",
                    words: "A `created` doc over an occupied destination blocks with **`finalize.promote-clobber`**, keyed at the destination path, and nothing is committed; an `edited-from-base` doc re-promotes over its own home, which is the update path.",
                },
            ],
        ),
        line(
            "T2",
            Allows::Files(Where::Path(".jigc/version"), May::WriteWhole),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "5. Stage",
                    words: "the `.jigc/version` stamp, refreshed to the running build first — **where it is jigc's to write**",
                },
            ],
        ),
        line(
            "T3",
            Allows::Staged,
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "5. Stage",
                    words: "This phase stages **jigc's own** managed artifacts into that existing index",
                },
                Quote {
                    doc: "design/finalize.md",
                    heading: "5. Stage",
                    words: "on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`),",
                },
            ],
        ),
        line(
            "T4",
            Allows::Commit(Holding::TheIndex),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "5. Stage",
                    words: "The commit set is therefore **the git index** — the agent's staged code plus jigc's just-staged docs/config — committed whole",
                },
                Quote {
                    doc: "design/finalize.md",
                    heading: "5. Stage",
                    words: "finalize **never** runs `git add --all`: untracked and unstaged working-tree changes stay out of the commit and are surfaced",
                },
            ],
        ),
        line(
            "T5, the edge index",
            Allows::Files(Where::Under(".jigc/index"), May::WriteWhole),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "- **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches).",
                },
            ],
        ),
        line(
            "T5, the file-state hashes",
            Allows::Files(Where::Under(".jigc/state"), May::WriteWhole),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "- **Update `file-state` hashes** for every committed managed doc.",
                },
            ],
        ),
        line(
            "T6",
            Allows::Files(Where::Under(".jigc/tasks/*"), May::Removed),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "- **Keep what jigc did not write, then remove `.jigc/tasks/<task-id>/`.**",
                },
            ],
        ),
        line(
            "T6, moved aside",
            Allows::Files(
                Where::Under(".jigc/tasks/*"),
                May::MovedAsideTo(".jigc/displaced"),
            ),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "The removal's subject is jigc's own files only: the area's **complement** ([team-ready-state.md](team-ready-state.md) → The working area's two populations) is moved aside first, to `.jigc/displaced/<task-id>/` with its relative paths intact",
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
pub const ROWS: &[Row] = &[FINALIZE];

// ---------------------------------------------------------------------------
// The cells
// ---------------------------------------------------------------------------

/// The binary under test, in the repository.
const fn jigc(args: &'static [&'static str]) -> Lead {
    Lead::Step(Step::Jigc(args))
}

/// A payload a lead-in reads with `--from-file`: a file under `outside/`, which a step
/// started in the repository reaches as `../outside/<name>`.
const fn payload(name: &'static str, body: &'static str) -> Lead {
    Lead::Step(Step::File(Place::Outside(name), body))
}

/// **An open task that has written the committed vision once, and is ready to finalize**
/// — its id slugs from the intent, `sharpen-the-open-questions`. The first write copies
/// the committed `VISION.md` into the task; the commit doc has its type and its summary.
const A_TASK_OVER_THE_VISION: [Lead; 6] = [
    jigc(&["start", "--workflow", "single-task", "sharpen the open questions"]),
    payload("open-questions", "What is still open, sharpened.\n"),
    jigc(&[
        "doc",
        "set-slot",
        "vision:vision#open-questions",
        "--from-file",
        "../outside/open-questions",
        "--task",
        "sharpen-the-open-questions",
    ]),
    jigc(&[
        "doc",
        "set-field",
        "commit:sharpen-the-open-questions#type",
        "--value",
        "docs",
        "--task",
        "sharpen-the-open-questions",
    ]),
    payload("summary", "sharpen the open questions\n"),
    jigc(&[
        "doc",
        "set-slot",
        "commit:sharpen-the-open-questions#summary",
        "--from-file",
        "../outside/summary",
        "--task",
        "sharpen-the-open-questions",
    ]),
];

/// A line added by hand to the committed vision **after the task's first write** to it
/// (the plan's plant **E**, at the second of its two positions).
const A_HAND_EDIT: Lead = Lead::Plant(Plant::Edit("VISION.md"));

/// **An open task that created a finding and is ready to finalize** — its id,
/// `file-a-finding`; the doc it created promotes to `docs/inconsistencies/two-docs.md`.
const A_TASK_WITH_A_CREATED_DOC: [Lead; 13] = [
    jigc(&["start", "--workflow", "report-inconsistency", "file a finding"]),
    jigc(&[
        "doc",
        "create",
        "inconsistency",
        "--title",
        "Two docs",
        "--task",
        "file-a-finding",
    ]),
    payload("description", "Two docs disagree.\n"),
    jigc(&[
        "doc",
        "set-slot",
        "inconsistency:two-docs#description",
        "--from-file",
        "../outside/description",
        "--task",
        "file-a-finding",
    ]),
    jigc(&[
        "doc",
        "set-field",
        "inconsistency:two-docs#meta/kind",
        "--value",
        "doc-doc",
        "--task",
        "file-a-finding",
    ]),
    jigc(&[
        "doc",
        "add-item",
        "inconsistency:two-docs#sides",
        "--title",
        "a.md",
        "--slug",
        "a",
        "--task",
        "file-a-finding",
    ]),
    jigc(&[
        "doc",
        "add-item",
        "inconsistency:two-docs#sides",
        "--title",
        "b.md",
        "--slug",
        "b",
        "--task",
        "file-a-finding",
    ]),
    payload("says", "It says one thing.\n"),
    jigc(&[
        "doc",
        "set-slot",
        "inconsistency:two-docs#sides/a/says",
        "--from-file",
        "../outside/says",
        "--task",
        "file-a-finding",
    ]),
    jigc(&[
        "doc",
        "set-slot",
        "inconsistency:two-docs#sides/b/says",
        "--from-file",
        "../outside/says",
        "--task",
        "file-a-finding",
    ]),
    jigc(&[
        "doc",
        "set-field",
        "commit:file-a-finding#type",
        "--value",
        "docs",
        "--task",
        "file-a-finding",
    ]),
    payload("summary", "file a finding\n"),
    jigc(&[
        "doc",
        "set-slot",
        "commit:file-a-finding#summary",
        "--from-file",
        "../outside/summary",
        "--task",
        "file-a-finding",
    ]),
];

/// The suite that sweeps a hand edit against a task's first write, ordering by ordering,
/// with and without a file-state baseline.
const THE_COPY_IN_SUITE_IN_A_CLONE: &str =
    "copy_in_baseline::a_second_clones_first_task_does_not_overwrite_a_hand_edit";
const THE_COPY_IN_SUITE: &str = "copy_in_baseline::a_hand_edit_after_the_first_write_blocks_both_committing_doors_and_survives";

/// **The acceptance cells of `jigc task finalize`** (the plan's §5): the blocking rows
/// `(R3, F7)` and `(R6, D-7)` of the review of `1.0.0-rc.24`. `(R3, F7)` is run on both
/// sides of the condition its record names — the file-state baseline absent, which is
/// the clone, and present.
///
/// **Every cell here is listed as one that refuses**, from the row's doc: an edit made
/// out of band after the task's first write is a conflict the finalize blocks on, and
/// *"a promote lands a regular file at exactly its canonical path, or the transaction
/// refuses before anything is written"* (`T1`).
pub const CELLS: &[Cell] = &[
    Cell {
        name: "task-finalize-no-baseline-a-hand-edit-after-the-first-write",
        leaf: &["task", "finalize"],
        state: Prepared::Of(State::CommittedSingletons),
        leads: &[
            Lead::Under(Condition::NoBaseline),
            A_TASK_OVER_THE_VISION[0],
            A_TASK_OVER_THE_VISION[1],
            A_TASK_OVER_THE_VISION[2],
            A_TASK_OVER_THE_VISION[3],
            A_TASK_OVER_THE_VISION[4],
            A_TASK_OVER_THE_VISION[5],
            A_HAND_EDIT,
        ],
        argv: &["task", "finalize", "sharpen-the-open-questions"],
        listed: Listed::Refuses,
        beside: Some(THE_COPY_IN_SUITE_IN_A_CLONE),
    },
    Cell {
        name: "task-finalize-a-baseline-a-hand-edit-after-the-first-write",
        leaf: &["task", "finalize"],
        state: Prepared::Of(State::CommittedSingletons),
        leads: &[
            A_TASK_OVER_THE_VISION[0],
            A_TASK_OVER_THE_VISION[1],
            A_TASK_OVER_THE_VISION[2],
            A_TASK_OVER_THE_VISION[3],
            A_TASK_OVER_THE_VISION[4],
            A_TASK_OVER_THE_VISION[5],
            A_HAND_EDIT,
        ],
        argv: &["task", "finalize", "sharpen-the-open-questions"],
        listed: Listed::Refuses,
        beside: Some(THE_COPY_IN_SUITE),
    },
    Cell {
        name: "task-finalize-a-link-to-nothing-at-the-created-docs-home",
        leaf: &["task", "finalize"],
        state: Prepared::Of(State::Fresh),
        leads: &[
            A_TASK_WITH_A_CREATED_DOC[0],
            A_TASK_WITH_A_CREATED_DOC[1],
            A_TASK_WITH_A_CREATED_DOC[2],
            A_TASK_WITH_A_CREATED_DOC[3],
            A_TASK_WITH_A_CREATED_DOC[4],
            A_TASK_WITH_A_CREATED_DOC[5],
            A_TASK_WITH_A_CREATED_DOC[6],
            A_TASK_WITH_A_CREATED_DOC[7],
            A_TASK_WITH_A_CREATED_DOC[8],
            A_TASK_WITH_A_CREATED_DOC[9],
            A_TASK_WITH_A_CREATED_DOC[10],
            A_TASK_WITH_A_CREATED_DOC[11],
            A_TASK_WITH_A_CREATED_DOC[12],
            Lead::Plant(Plant::LinkToNothing("docs/inconsistencies/two-docs.md")),
        ],
        argv: &["task", "finalize", "file-a-finding"],
        listed: Listed::Refuses,
        beside: Some("promote_destination_shape::a_fresh_doc_never_promotes_onto_an_entry_that_is_not_a_regular_file"),
    },
];
