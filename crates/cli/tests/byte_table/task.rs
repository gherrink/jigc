//! **The byte table's rows for the task family** — `jigc task finalize` and `jigc task
//! discard` (`crates/cli/tests/byte_table.rs`) — lifted quote for quote from the committed
//! record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`,
//! §3 and §3a), under the rules the install family's file states. `jigc task discard`'s
//! row was written from its docs in the plan's task `B4`, in a commit of its own before a
//! cell of it ran.
//!
//! **What of the record is no line.** `T7` is where *on a refusal: no* comes from. `T8` is
//! the rule for prose handed to jigc, which the frame holds a staged doc to by itself
//! (`support::byte_cell` → *a staged doc*). **The doc-only arm is a second spelling**,
//! `doc-only`, with a commit line of its own — a task whose workflow includes
//! `step:finalize-doc-only` commits its own paths and never the index — `T9`, which takes
//! `T2` to `T4` out of force; a cell of such a task names it ([`Spelling`]), by the human's
//! ruling of 2026-10-18.

use super::{AFTER_A_COMMIT, ON_A_REFUSAL, THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ, refuses};
use crate::support::byte_cell::{
    Aim, Allows, Cell, Condition, Holding, Kind, Lead, Line, Listed, Mark, May, Plant, Prepared,
    Quote, Row, Spelling, Stages, THE_RETIRED, Typed, Unit, When, Where,
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
        spelling: None,
    }
}

/// **The project config layer a finalize lands on its first commit** — `T3, the config`:
/// the stage's sentence, which `M3, the config` quotes too, and the per-task commit set's
/// two stages.
const THE_CONFIG_CARRIED: [Quote; 2] = [
    super::THE_CONFIG_LAYERS_FIRST_COMMIT,
    Quote {
        doc: "design/storage.md",
        heading: "The per-task working area (staging)",
        words: "jigc's promotion (docs + first-commit config) — and finalize unions exactly those, never sweeping unstaged/untracked working-tree WIP",
    },
];

/// **`jigc task finalize`** — the record's §3. The record names no consent of its own.
pub const FINALIZE: Row = Row {
    leaf: &["task", "finalize"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[
        Spelling {
            name: "doc-only",
            typed: Typed::State,
            replaces: &[
                "T2",
                "T3",
                "T3, the config",
                "T3, the config's ignore file",
                "T4",
            ],
        },
        // A migration task's approved finalize, which retires the foreign original (`T10`)
        // and takes no line out of force. Added 2026-10-20, after the review of the task
        // `B9` (H6). Not the row's consent: a consent lets a plant the output names go, and
        // the warrant the approval rests on is that what it retires is git-recoverable.
        Spelling {
            name: "--approve",
            typed: Typed::Word("--approve"),
            replaces: &[],
        },
    ],
    consent: None,
    home: Prepared::Of(State::Fresh),
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
            &[Quote {
                doc: "design/finalize.md",
                heading: "5. Stage",
                words: "the `.jigc/version` stamp, refreshed to the running build first — **where it is jigc's to write**",
            }],
        ),
        line(
            "T3",
            Allows::Staged(Stages::MayStage),
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
        // The project config layer, carried into the commit as it stands on the layer's
        // first commit — `T3`'s second sentence, read as `M3, the config` reads it, and
        // the per-task commit set's *"docs + first-commit config"*. Added 2026-10-19, after
        // the review of the task `B7`'s second part (its first finding): the two rows read
        // one sentence two ways. Whether a pending delta rides a later commit was the rows'
        // record's §6, entry 19, ruled by the human on 2026-10-19: not unless it is staged.
        Line {
            when: When::FirstCommitOfItsPlace,
            ..line(
                "T3, the config",
                Allows::Files(Where::Under(".jigc/config"), May::Carried),
                &THE_CONFIG_CARRIED,
            )
        },
        Line {
            when: When::FirstCommitOfItsPlace,
            ..line(
                "T3, the config's ignore file",
                Allows::Files(Where::Path(".jigc/.gitignore"), May::Carried),
                &THE_CONFIG_CARRIED,
            )
        },
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
            Allows::Files(
                Where::Under(".jigc/index"),
                May::Invalidated(".jigc/index/edges.json"),
            ),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "- **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches). The next read rebuilds from the new HEAD.",
                },
                THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ[0],
                THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ[1],
                THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ[2],
            ],
        ),
        line(
            "T5, the file-state hashes",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[Quote {
                doc: "design/finalize.md",
                heading: "7. Post-commit (best-effort)",
                words: "- **Update `file-state` hashes** for every committed managed doc.",
            }],
        ),
        line(
            "T6",
            Allows::Files(Where::Under(".jigc/tasks/<unit>"), May::Removed),
            &[Quote {
                doc: "design/finalize.md",
                heading: "7. Post-commit (best-effort)",
                words: "- **Keep what jigc did not write, then remove `.jigc/tasks/<task-id>/`.**",
            }],
        ),
        line(
            "T6, moved aside",
            Allows::Files(
                Where::Under(".jigc/tasks/<unit>"),
                May::MovedAsideTo(".jigc/displaced/<unit>"),
            ),
            &[Quote {
                doc: "design/finalize.md",
                heading: "7. Post-commit (best-effort)",
                words: "The removal's subject is jigc's own files only: the area's **complement** ([team-ready-state.md](team-ready-state.md) → The working area's two populations) is moved aside first, to `.jigc/displaced/<task-id>/` with its relative paths intact",
            }],
        ),
        // The doc-only arm's commit (the record's `T9`): a task whose workflow includes
        // `step:finalize-doc-only` commits its own paths alone and never the index, and
        // leaves the stamp and a pending config delta out — so under it `T2` to `T4` are
        // out of force.
        Line {
            spelling: Some("doc-only"),
            ..line(
                "T9",
                Allows::Commit(Holding::OnlyTheLines),
                &[
                    Quote {
                        doc: "design/finalize.md",
                        heading: "The doc-only arm",
                        words: "commits those paths alone (`git add -- <paths>` then `git commit -F <message> -- <paths>`), never the index",
                    },
                    Quote {
                        doc: "design/finalize.md",
                        heading: "The doc-only arm",
                        words: "Whatever else is staged in the checkout — by a code task open beside it, before or after this task was minted — **stays staged**",
                    },
                    Quote {
                        doc: "design/finalize.md",
                        heading: "The doc-only arm",
                        words: "**Outside the path set by construction:** a pending `.jigc/config` delta (an ordinary code-less finalize commits it beside the doc — gap G8) and the `.jigc/version` stamp, which this arm does not refresh",
                    },
                ],
            )
        },
        // The retire a migration task's approved finalize runs (the record's `T10`): the
        // foreign original the task recorded, gone where what goes is what `HEAD` holds
        // there. Added 2026-10-20, after the review of the task `B9` (H6): the row had no
        // line for it, so no plant could be aimed where it deletes.
        Line {
            spelling: Some("--approve"),
            ..line(
                "T10",
                Allows::Files(Where::Path(THE_RETIRED), May::Retired),
                &[
                    Quote {
                        doc: "design/auto-migration.md",
                        heading: "The review gate (fidelity)",
                        words: "`jigc task finalize <id> --approve` proceeds through the retire + commit + adopt",
                    },
                    Quote {
                        doc: "design/auto-migration.md",
                        heading: THE_RETIRE,
                        words: "**Trackedness precondition (M51) — the retire deletes only a file git holds a copy of.**",
                    },
                    Quote {
                        doc: "design/auto-migration.md",
                        heading: THE_RETIRE,
                        words: "(*a guarded migrate destroys only a reviewed, in-repo, **git-recoverable** file*)",
                    },
                ],
            )
        },
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
    ],
};

/// The heading of the retire's section of `design/auto-migration.md`.
const THE_RETIRE: &str = "Retire-the-foreign-original (the first destructive write)";

/// The heading of the sub-task discard's section of `design/team-ready-state.md`.
const THE_SUB_TASK_TERMINAL: &str =
    "`jigc task discard <sub-id>` — the per-sub-task terminal (M49)";

/// **`jigc task discard`** — the record's §3a. Its consent is `--force` (`A4`).
///
/// **What of the record is no line.** `A2` and `A3` are the two states the command refuses
/// over, and with `A8` where *on a refusal: no* comes from; `A4` is the row's consent;
/// `A9` and `A10` are conditions the cells construct — an ordinary task, which no record
/// names, and a sub-task nobody re-entered. `A5` to `A7` hold for a sub-task's discard
/// and carry that condition in words only: each lets the command do less where the task
/// is an ordinary one.
pub const DISCARD: Row = Row {
    leaf: &["task", "discard"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: Some("--force"),
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "A1",
            Allows::Files(Where::Under(".jigc/tasks/<unit>"), May::Removed),
            &[Quote {
                doc: "design/write-commands.md",
                heading: "The verbs",
                words: "`discard` destroys a working area, and `.jigc/tasks/<id>/docs/*.md` is the only copy of every doc staged in it.",
            }],
        ),
        line(
            "A5",
            Allows::Files(
                Where::Path("<docs-root>/milestone-records/<its-milestone>.md"),
                May::WriteWhole,
            ),
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: THE_SUB_TASK_TERMINAL,
                    words: "The item's `status` flips to `discarded` through the byte-stable item-leaf splice; **the header stays `active` and every sibling item is byte-untouched**",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: "Source of truth",
                    words: "M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: "Config layout",
                    words: "The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load",
                },
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: THE_SUB_TASK_TERMINAL,
                    words: "The owning milestone is resolved from **what the committed record names**, never from the gitignored `.jigc/` cache",
                },
            ],
        ),
        line(
            "A6",
            Allows::Commit(Holding::OnlyTheLines),
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: "The commit model — path-scoped commit at each milestone op",
                    words: "it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP",
                },
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: THE_SUB_TASK_TERMINAL,
                    words: "the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook",
                },
            ],
        ),
        line(
            "A7",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: THE_SUB_TASK_TERMINAL,
                    words: "the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook",
                },
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: "Engine capability 1 (write) — `set: on-transition` machine-maintained leaves",
                    words: "the hash jigc recorded at its last landed record write — lives in `.jigc/state/`, which no clone carries",
                },
            ],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
    ],
};

/// The family's rows.
pub const ROWS: &[Row] = &[FINALIZE, DISCARD];

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
    jigc(&[
        "start",
        "--workflow",
        "single-task",
        "sharpen the open questions",
    ]),
    payload("open-questions", "What is still open, sharpened.\n"),
    jigc(WRITE_THE_OPEN_QUESTIONS),
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

/// The task's write to the vision's `open-questions` slot, its last part — the part a line
/// added by hand at the file's end stands in.
const WRITE_THE_OPEN_QUESTIONS: &[&str] = &[
    "doc",
    "set-slot",
    "vision:vision#open-questions",
    "--from-file",
    "../outside/open-questions",
    "--task",
    "sharpen-the-open-questions",
];

/// A line added by hand to the committed vision **after the task's first write** to it
/// (the plan's plant **E**, at the second of its two positions).
const A_HAND_EDIT: Lead = Lead::Plant(Plant::Edit("VISION.md"));

/// **An open task that created a finding and is ready to finalize** — its id,
/// `file-a-finding`; the doc it created promotes to `docs/inconsistencies/two-docs.md`.
/// Its workflow, `report-inconsistency`, includes `step:finalize-doc-only`: its finalize is
/// the leaf's spelling `doc-only` (`T9`), and a cell of it names that spelling.
const A_TASK_WITH_A_CREATED_DOC: [Lead; 13] = [
    jigc(&[
        "start",
        "--workflow",
        "report-inconsistency",
        "file a finding",
    ]),
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
const THE_COPY_IN_SUITE: &str =
    "copy_in_baseline::a_hand_edit_after_the_first_write_blocks_both_committing_doors_and_survives";

/// **The acceptance cells of `jigc task finalize`** (the plan's §5): the blocking rows
/// `(R3, F7)` and `(R6, D-7)` of the review of `1.0.0-rc.24`. `(R3, F7)` is run on both
/// sides of the condition its record names — the file-state baseline absent, which is
/// the clone, and present.
///
/// **Every cell here is listed as one that refuses, and names its refusal**, from the
/// row's doc: an edit made
/// out of band after the task's first write is a conflict the finalize blocks on, and
/// *"a promote lands a regular file at exactly its canonical path, or the transaction
/// refuses before anything is written"* (`T1`).
const ACCEPTANCE: &[Cell] = &[
    Cell {
        spelling: None,
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
        listed: refuses(3, "reconciliation.conflict-block"),
        beside: Some(THE_COPY_IN_SUITE_IN_A_CLONE),
        aim: Some(Aim {
            line: "T1",
            kind: Kind::EditAfter,
        }),
        marks: &[],
        at: None,
    },
    Cell {
        spelling: None,
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
        listed: refuses(3, "reconciliation.conflict-block"),
        beside: Some(THE_COPY_IN_SUITE),
        aim: Some(Aim {
            line: "T1",
            kind: Kind::EditAfter,
        }),
        marks: &[],
        at: None,
    },
    Cell {
        spelling: Some("doc-only"),
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
        listed: refuses(3, "store.home-not-regular-file"),
        beside: Some(
            "promote_destination_shape::a_fresh_doc_never_promotes_onto_an_entry_that_is_not_a_regular_file",
        ),
        aim: Some(Aim {
            line: "T1",
            kind: Kind::LinkToNothing,
        }),
        marks: &[],
        at: None,
    },
];

// ---------------------------------------------------------------------------
// The family's cells — the plan's task `B4`
// ---------------------------------------------------------------------------

/// **An open task that created a decision and is ready to finalize** — its id,
/// `record-a-decision`; the ADR it created promotes to `docs/decisions/one-way.md`.
/// `single-task` is the leaf's plain spelling: its commit is the index, whole (`T4`).
pub(super) const A_DECISION: [Lead; 11] = [
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
    payload("context", "The forces, in a line.\n"),
    jigc(&[
        "doc",
        "set-slot",
        "adr:one-way#context",
        "--from-file",
        "../outside/context",
        "--task",
        "record-a-decision",
    ]),
    payload("decision", "The call, in a line.\n"),
    jigc(&[
        "doc",
        "set-slot",
        "adr:one-way#decision",
        "--from-file",
        "../outside/decision",
        "--task",
        "record-a-decision",
    ]),
    payload("consequences", "What follows, in a line.\n"),
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
    payload("summary", "record a decision\n"),
    jigc(&[
        "doc",
        "set-slot",
        "commit:record-a-decision#summary",
        "--from-file",
        "../outside/summary",
        "--task",
        "record-a-decision",
    ]),
];

/// [`A_DECISION`], with what stands before the task is started — which is where a file
/// the user committed goes, since `start` pins the task's base — and what is put in after
/// its last write, last before the command.
macro_rules! a_decision {
    ([$($before:expr),* $(,)?], [$($after:expr),* $(,)?]) => {
        &[
            $($before,)*
            A_DECISION[0], A_DECISION[1], A_DECISION[2], A_DECISION[3], A_DECISION[4],
            A_DECISION[5], A_DECISION[6], A_DECISION[7], A_DECISION[8], A_DECISION[9],
            A_DECISION[10],
            $($after,)*
        ]
    };
}

/// The home the created decision promotes to, and the directory it stands in.
const THE_DECISIONS_HOME: &str = "docs/decisions/one-way.md";
const THE_DECISIONS_DIRECTORY: &str = "docs/decisions";

const FINALIZE_THE_DECISION: &[&str] = &["task", "finalize", "record-a-decision"];
const FINALIZE_THE_VISION: &[&str] = &["task", "finalize", "sharpen-the-open-questions"];

/// **The task of [`A_TASK_OVER_THE_VISION`], writing another part of the vision** — its
/// thesis, the first of the doc's three slots, where a line added by hand at the file's
/// end stands in the last.
const A_WRITE_TO_THE_THESIS: [Lead; 2] = [
    payload("thesis", "A thesis, sharpened.\n"),
    jigc(&[
        "doc",
        "set-slot",
        "vision:vision#thesis",
        "--from-file",
        "../outside/thesis",
        "--task",
        "sharpen-the-open-questions",
    ]),
];

/// **A milestone with one sub-task nobody has re-entered** — `ship-it`, and its sub-task
/// `do-the-thing`: the record names it, and it stages nothing (the record's `A10`).
const A_SUB_TASK: [Lead; 2] = [
    jigc(&["milestone", "create", "Ship it"]),
    jigc(&["milestone", "add-task", "ship-it", "do the thing"]),
];

/// The milestone's committed record.
const THE_RECORD: &str = "docs/milestone-records/ship-it.md";

const DISCARD_THE_DECISION: &[&str] = &["task", "discard", "record-a-decision"];
const DISCARD_THE_DECISION_BY_CONSENT: &[&str] =
    &["task", "discard", "record-a-decision", "--force"];
const DISCARD_THE_SUB_TASK: &[&str] = &["task", "discard", "do-the-thing"];

/// What a planted cell's plant is aimed at.
const fn at(line: &'static str, kind: Kind) -> Option<Aim> {
    Some(Aim { line, kind })
}

/// One cell of `jigc task finalize`.
const fn finalize(
    name: &'static str,
    state: Prepared,
    leads: &'static [Lead],
    argv: &'static [&'static str],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf: &["task", "finalize"],
        state,
        leads,
        argv,
        listed,
        beside: None,
        aim,
        marks: &[],
        at: None,
    }
}

/// One cell of `jigc task discard`, in `fresh`.
const fn discard(
    name: &'static str,
    leads: &'static [Lead],
    argv: &'static [&'static str],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf: &["task", "discard"],
        state: FRESH,
        leads,
        argv,
        listed,
        beside: None,
        aim,
        marks: &[],
        at: None,
    }
}

const FRESH: Prepared = Prepared::Of(State::Fresh);
const SINGLETONS: Prepared = Prepared::Of(State::CommittedSingletons);

/// A file of the user's that is no doc and no file of jigc's, in a task's working area.
const NOTES_AT_THE_AREAS_ROOT: &str = ".jigc/tasks/record-a-decision/NOTES.md";
const NOTES_UNDER_THE_AREAS_DOCS: &str = ".jigc/tasks/record-a-decision/docs/notes.txt";

/// A file whose name git quotes: a quote, and a byte outside ASCII (the plan's plant **N**).
const A_NAME_GIT_QUOTES: &str = "notes/a \"quoted\" n\u{e4}me.txt";

/// **The cells of `jigc task finalize`** — its refusing cell, its base cell, and a planted
/// cell for every line of its row and every kind of plant that targets it, but for the
/// ones `scheduled-cells.tsv` holds with their reason. Each is listed from the row's doc:
///
/// - **over the created doc's home** (`T1`) a file in no commit, a link, a directory and a
///   file staged and not in the tree are each refused — *"A promote lands a regular file
///   at exactly its canonical path, or the transaction refuses before anything is
///   written"*, and *"a `created` doc over an occupied destination blocks with
///   **`finalize.promote-clobber`**"*; a file the user committed there before the task
///   began is the doc's own base — the create takes it up — and what it holds is no ADR,
///   so the finalize's validation refuses;
/// - **an edit before the task's first write** (`T1`) is carried into the task where the
///   task writes another part, and where it writes the same part the write replaces it —
///   the second of the bounds of 2026-10-06, which the human retired on 2026-10-19: such a
///   write refuses at the write, and the cell is red under its key, unmarked;
/// - **a change staged at the doc and not in the working file** (`T1`) is replaced at
///   finalize — the first of those bounds;
/// - **what is staged elsewhere rides the commit** (`T4`: *"The commit set is therefore
///   **the git index** … committed whole"*), a name git quotes included, and **a hook
///   that rejects** leaves the task as it was (`T7`);
/// - **a file of the user's in a cache the finalize writes** (`T5`) stays, and one **in
///   the task's working area** (`T6`) is *"moved aside first, to
///   `.jigc/displaced/<task-id>/` with its relative paths intact"*;
/// - **a hand edit to `.jigc/version`** (`T2`) is left: the stamp is refreshed *"where it
///   is jigc's to write"*; a change staged at it and not in the file is met by no
///   sentence, and the cell is listed as one that acts — and since the human's ruling of
///   2026-10-14 on the record's §6, entry 8, its replacement is a finding: the declared
///   bound is one case, a doc a task is updating.
///
/// Two cells run their own command a second time over what a first refusal left
/// ([`Lead::Refused`]): the ruling on a refusal's caches is bounded by *a refused command
/// run again refuses again, with the plant standing*.
const FINALIZE_CELLS: [Cell; 26] = [
    finalize(
        "task-finalize-a-task-that-is-not-there",
        FRESH,
        &[],
        &["task", "finalize", "a-task"],
        refuses(1, "finalize.no-task"),
        None,
    ),
    finalize(
        "task-finalize-a-created-decision",
        FRESH,
        a_decision!([], []),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        None,
    ),
    // The record's §6, entry 19, ruled by the human on 2026-10-19 (option A): a finalize
    // never carries a change under `.jigc/config` that nobody staged — it names it among
    // what it left out; the cell `milestone-finalize-a-pending-config-delta` is the same
    // ruling at the boundary. Red under its key until the product's fix; no mark.
    finalize(
        "task-finalize-a-pending-config-delta",
        FRESH,
        a_decision!(
            [],
            [jigc(&["config", "set", "finalize.fan-out.squash", "true"])]
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        None,
    ),
    finalize(
        "task-finalize-an-untracked-file-at-the-created-docs-home",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::File(THE_DECISIONS_HOME))]),
        FINALIZE_THE_DECISION,
        refuses(3, "finalize.promote-clobber"),
        at("T1", Kind::Untracked),
    ),
    finalize(
        "task-finalize-an-untracked-file-at-the-created-docs-home-run-again",
        FRESH,
        a_decision!(
            [],
            [
                Lead::Plant(Plant::File(THE_DECISIONS_HOME)),
                Lead::Refused(FINALIZE_THE_DECISION),
            ]
        ),
        FINALIZE_THE_DECISION,
        refuses(3, "finalize.promote-clobber"),
        at("T1", Kind::Untracked),
    ),
    // The create takes the committed file up as the doc's base, and every write of the
    // task to it is then refused — what it holds is no ADR — so the task comes to its
    // finalize with the doc as the user committed it.
    finalize(
        "task-finalize-a-committed-file-at-the-created-docs-home",
        FRESH,
        &[
            Lead::Plant(Plant::Committed(THE_DECISIONS_HOME)),
            A_DECISION[0],
            A_DECISION[1],
            A_DECISION[8],
            A_DECISION[9],
            A_DECISION[10],
        ],
        FINALIZE_THE_DECISION,
        refuses(3, "conformance.section-missing"),
        at("T1", Kind::Committed),
    ),
    finalize(
        "task-finalize-a-link-to-a-file-in-the-repository-at-the-created-docs-home",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::LinkToAFile {
                at: THE_DECISIONS_HOME,
                outside: false,
            })]
        ),
        FINALIZE_THE_DECISION,
        refuses(3, "store.home-not-regular-file"),
        at("T1", Kind::LinkInside),
    ),
    finalize(
        "task-finalize-a-link-to-a-file-outside-at-the-created-docs-home",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::LinkToAFile {
                at: THE_DECISIONS_HOME,
                outside: true,
            })]
        ),
        FINALIZE_THE_DECISION,
        refuses(3, "store.home-not-regular-file"),
        at("T1", Kind::LinkOutside),
    ),
    // The plain spelling's own cell at a link to nothing: the acceptance's runs a task of
    // the doc-only arm, and a need is keyed by its spelling.
    finalize(
        "task-finalize-a-link-to-nothing-at-the-created-decisions-home",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::LinkToNothing(THE_DECISIONS_HOME))]),
        FINALIZE_THE_DECISION,
        refuses(3, "store.home-not-regular-file"),
        at("T1", Kind::LinkToNothing),
    ),
    finalize(
        "task-finalize-a-directory-at-the-created-docs-home",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::Dir(THE_DECISIONS_HOME))]),
        FINALIZE_THE_DECISION,
        refuses(3, "store.home-not-regular-file"),
        at("T1", Kind::DirForAFile),
    ),
    finalize(
        "task-finalize-a-file-where-the-created-docs-directory-goes",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::File(THE_DECISIONS_DIRECTORY))]),
        FINALIZE_THE_DECISION,
        refuses(1, "to promote into"),
        at("T1", Kind::FileForADir),
    ),
    finalize(
        "task-finalize-a-file-staged-at-the-created-docs-home-and-not-in-the-tree",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::Staged {
                at: THE_DECISIONS_HOME,
                kept: false,
            })]
        ),
        FINALIZE_THE_DECISION,
        refuses(3, "finalize.promote-clobber"),
        at("T1", Kind::StagedThere),
    ),
    Cell {
        marks: &[Mark::Bound(1)],
        ..finalize(
            "task-finalize-a-change-staged-at-the-updated-doc-and-not-in-the-file",
            SINGLETONS,
            &[
                A_TASK_OVER_THE_VISION[0],
                A_TASK_OVER_THE_VISION[1],
                A_TASK_OVER_THE_VISION[2],
                A_TASK_OVER_THE_VISION[3],
                A_TASK_OVER_THE_VISION[4],
                A_TASK_OVER_THE_VISION[5],
                Lead::Plant(Plant::Staged {
                    at: "VISION.md",
                    kept: false,
                }),
            ],
            FINALIZE_THE_VISION,
            Listed::Acts,
            at("T1", Kind::StagedThere),
        )
    },
    finalize(
        "task-finalize-a-hand-edit-before-the-first-write-to-another-part",
        SINGLETONS,
        &[
            A_TASK_OVER_THE_VISION[0],
            A_HAND_EDIT,
            A_WRITE_TO_THE_THESIS[0],
            A_WRITE_TO_THE_THESIS[1],
            A_TASK_OVER_THE_VISION[3],
            A_TASK_OVER_THE_VISION[4],
            A_TASK_OVER_THE_VISION[5],
        ],
        FINALIZE_THE_VISION,
        Listed::Acts,
        at("T1", Kind::EditBefore),
    ),
    // The write to the part the line stands in is the lead-in the human's ruling of
    // 2026-10-19 refuses, showing the line: it may act or refuse, and the task goes on to
    // write another part, so the finalize meets a staged vision on both sides of the fix —
    // the line replaced on this tree, carried on a product that refuses the write.
    finalize(
        "task-finalize-a-hand-edit-before-the-first-write-to-the-same-part",
        SINGLETONS,
        &[
            A_TASK_OVER_THE_VISION[0],
            A_HAND_EDIT,
            A_TASK_OVER_THE_VISION[1],
            Lead::OverAHandEdit {
                step: Step::Jigc(WRITE_THE_OPEN_QUESTIONS),
                plant: 1,
            },
            A_TASK_OVER_THE_VISION[3],
            A_TASK_OVER_THE_VISION[4],
            A_TASK_OVER_THE_VISION[5],
            A_WRITE_TO_THE_THESIS[0],
            A_WRITE_TO_THE_THESIS[1],
        ],
        FINALIZE_THE_VISION,
        Listed::Acts,
        at("T1", Kind::EditBefore),
    ),
    Cell {
        beside: Some(THE_COPY_IN_SUITE),
        ..finalize(
            "task-finalize-a-hand-edit-after-the-first-write-run-again",
            SINGLETONS,
            &[
                A_TASK_OVER_THE_VISION[0],
                A_TASK_OVER_THE_VISION[1],
                A_TASK_OVER_THE_VISION[2],
                A_TASK_OVER_THE_VISION[3],
                A_TASK_OVER_THE_VISION[4],
                A_TASK_OVER_THE_VISION[5],
                A_HAND_EDIT,
                Lead::Refused(FINALIZE_THE_VISION),
            ],
            FINALIZE_THE_VISION,
            refuses(3, "reconciliation.conflict-block"),
            at("T1", Kind::EditAfter),
        )
    },
    finalize(
        "task-finalize-a-hand-edit-to-the-version-stamp",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::Edit(".jigc/version"))]),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::EditAfter),
    ),
    finalize(
        "task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::Staged {
                at: ".jigc/version",
                kept: false,
            })]
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::StagedThere),
    ),
    finalize(
        "task-finalize-a-file-staged-elsewhere",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::Staged {
                at: "notes/staged-by-its-user.txt",
                kept: true,
            })]
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T4", Kind::StagedElsewhere),
    ),
    finalize(
        "task-finalize-a-staged-file-whose-name-git-quotes",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::Staged {
                at: A_NAME_GIT_QUOTES,
                kept: true,
            })]
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T4", Kind::QuotedName),
    ),
    finalize(
        "task-finalize-a-hook-that-rejects",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::RejectingHook)]),
        FINALIZE_THE_DECISION,
        refuses(1, "`git commit` was rejected"),
        at("T4", Kind::RejectingHook),
    ),
    // The index built before the command — a `task validate` reads it, and its read builds
    // it — so the line's word meets the stamped file taken away, as no other cell's leads
    // let it (the review of `B7`'s part A, its first finding).
    finalize(
        "task-finalize-a-foreign-file-under-the-edge-index",
        FRESH,
        a_decision!(
            [],
            [
                jigc(&["task", "validate", "record-a-decision"]),
                Lead::Plant(Plant::File(".jigc/index/notes.md")),
            ]
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T5, the edge index", Kind::Untracked),
    ),
    finalize(
        "task-finalize-a-foreign-file-under-the-file-state-hashes",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::File(".jigc/state/notes.md"))]),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T5, the file-state hashes", Kind::Untracked),
    ),
    finalize(
        "task-finalize-a-foreign-file-at-the-task-areas-root",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::File(NOTES_AT_THE_AREAS_ROOT))]),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T6", Kind::Foreign),
    ),
    finalize(
        "task-finalize-a-foreign-file-under-the-task-areas-docs",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::File(NOTES_UNDER_THE_AREAS_DOCS))]),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T6", Kind::Foreign),
    ),
    // The base cell in the never-adopted repository: the command as typed, with no lead.
    finalize(
        "task-finalize-a-created-decision-in-bare",
        Prepared::Bare,
        &[],
        FINALIZE_THE_DECISION,
        refuses(1, "finalize.no-task"),
        None,
    ),
];

/// **The cells of `jigc task finalize` at the stamp that stands** — `T2`'s needs a plant in
/// the place of a file that stands answers (the human's ruling of 2026-10-18): a file of
/// the user's, one the user committed before the task began, three links and a directory
/// where jigc's own `.jigc/version` stood. Each is listed from the row's doc: the stamp is
/// refreshed *"where it is jigc's to write"* (`T2`), and a stamp that is not jigc's own
/// one-line stamp is not jigc's to write — so the finalize acts and leaves it, where a link
/// or a directory is no stamp to refresh either. The file of the user's is untracked, as
/// **U** says — the index's entry for the stamp goes with the stamp — so the commit
/// carries the stamp's deletion the user staged: *"the git index … committed whole"*
/// (`T4`).
///
/// **A file in the place of the directory the stamp stands in has no cell, and a scheduled
/// row says why**: `.jigc/` holds the task's working area too, so a file there takes the
/// task with it and the finalize refuses `finalize.no-task` before it reaches the stamp — a
/// cell of it would hold a refusal over a task that is not there, and never `T2`.
const FINALIZE_AT_THE_STAMP: [Cell; 6] = [
    finalize(
        "task-finalize-a-file-of-the-users-in-the-stamps-place",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::InPlaceOf(&Plant::File(THE_STAMP)))]),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::Untracked),
    ),
    finalize(
        "task-finalize-a-file-the-user-committed-in-the-stamps-place",
        FRESH,
        a_decision!(
            [Lead::Plant(Plant::InPlaceOf(&Plant::Committed(THE_STAMP)))],
            []
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::Committed),
    ),
    finalize(
        "task-finalize-a-link-to-nothing-in-the-stamps-place",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
                THE_STAMP
            )))]
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::LinkToNothing),
    ),
    finalize(
        "task-finalize-a-link-to-a-file-in-the-repository-in-the-stamps-place",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: THE_STAMP,
                outside: false,
            }))]
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::LinkInside),
    ),
    finalize(
        "task-finalize-a-link-to-a-file-outside-in-the-stamps-place",
        FRESH,
        a_decision!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: THE_STAMP,
                outside: true,
            }))]
        ),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::LinkOutside),
    ),
    finalize(
        "task-finalize-a-directory-in-the-stamps-place",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::InPlaceOf(&Plant::Dir(THE_STAMP)))]),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::DirForAFile),
    ),
];

/// The stamp `T2` refreshes.
const THE_STAMP: &str = ".jigc/version";

/// **The base cell of `jigc task finalize` in the prepared states that are not its home**
/// — on demand, in no gate: the same task, the same command.
const FINALIZE_ELSEWHERE: [Cell; 5] = [
    finalize(
        "task-finalize-a-created-decision-in-committed-singletons",
        SINGLETONS,
        a_decision!([], []),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        None,
    ),
    finalize(
        "task-finalize-a-created-decision-in-migrated",
        Prepared::Of(State::Migrated),
        a_decision!([], []),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        None,
    ),
    finalize(
        "task-finalize-a-created-decision-in-refs-post-hoc",
        Prepared::Of(State::RefsPostHoc),
        a_decision!([], []),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        None,
    ),
    finalize(
        "task-finalize-a-created-decision-in-chatty-hooks",
        Prepared::Of(State::ChattyHooks),
        a_decision!([], []),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        None,
    ),
    finalize(
        "task-finalize-a-created-decision-in-vendored",
        Prepared::Of(State::Vendored),
        a_decision!([], []),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        None,
    ),
];

/// **The cells of `jigc task discard`**, each listed from the row's doc:
///
/// - **an ordinary task refuses from the moment it exists** (`A2`: *"an ordinary discard
///   needs the consent from the moment the task exists"*), and under the consent its
///   working area goes with what is staged in it, which the command's output names
///   (`A4`);
/// - **a sub-task nobody re-entered** is settled with no consent (`A10`): one item of the
///   record flips, in a commit of the record alone, and the area goes (`A5` to `A7`) —
///   **the family's base cell**, the one spelling in which the command acts without its
///   consent flag;
/// - **a file jigc did not write in the working area** is refused, *"naming every path"*
///   (`A3`), and goes under the consent, named;
/// - **the record** (`A5`): a hand edit *"conflict-blocks on drift"*
///   (`design/team-ready-state.md` → *Engine capability 1*); a change staged at it and not
///   in the working file is met by no sentence of the row's docs — the cell is listed as
///   one that acts, since no sentence says the state is refused, and its replacement is a
///   finding by the human's ruling of 2026-10-14 on the record's §6, entry 8;
/// - **what is staged elsewhere stays out of the record's commit** (`A6`: *"never
///   sweeping the agent's in-flight index/worktree WIP"*), and **a hook that rejects**
///   leaves *"the record **and** the task exactly as the frame says"* (`A8`);
/// - **a file of the user's beside the baseline** (`A7`) stays.
const DISCARD_CELLS: [Cell; 15] = [
    discard(
        "task-discard-a-task-that-is-not-there",
        &[],
        &["task", "discard", "a-task"],
        refuses(1, "no task `a-task`"),
        None,
    ),
    discard(
        "task-discard-a-task-with-a-staged-decision",
        a_decision!([], []),
        DISCARD_THE_DECISION,
        refuses(1, "task-discard.staged-prose"),
        None,
    ),
    discard(
        "task-discard-a-task-with-a-staged-decision-under-the-consent",
        a_decision!([], []),
        DISCARD_THE_DECISION_BY_CONSENT,
        Listed::Acts,
        None,
    ),
    discard(
        "task-discard-a-sub-task-nobody-re-entered",
        &[A_SUB_TASK[0], A_SUB_TASK[1]],
        DISCARD_THE_SUB_TASK,
        Listed::Acts,
        None,
    ),
    discard(
        "task-discard-a-foreign-file-at-the-task-areas-root",
        a_decision!([], [Lead::Plant(Plant::File(NOTES_AT_THE_AREAS_ROOT))]),
        DISCARD_THE_DECISION,
        refuses(1, "task-discard.foreign-bytes"),
        at("A1", Kind::Foreign),
    ),
    discard(
        "task-discard-a-foreign-file-at-the-task-areas-root-under-the-consent",
        a_decision!([], [Lead::Plant(Plant::File(NOTES_AT_THE_AREAS_ROOT))]),
        DISCARD_THE_DECISION_BY_CONSENT,
        Listed::Acts,
        at("A1", Kind::Foreign),
    ),
    discard(
        "task-discard-a-foreign-file-under-the-task-areas-docs",
        a_decision!([], [Lead::Plant(Plant::File(NOTES_UNDER_THE_AREAS_DOCS))]),
        DISCARD_THE_DECISION,
        refuses(1, "task-discard.foreign-bytes"),
        at("A1", Kind::Foreign),
    ),
    discard(
        "task-discard-a-foreign-file-under-the-task-areas-docs-under-the-consent",
        a_decision!([], [Lead::Plant(Plant::File(NOTES_UNDER_THE_AREAS_DOCS))]),
        DISCARD_THE_DECISION_BY_CONSENT,
        Listed::Acts,
        at("A1", Kind::Foreign),
    ),
    discard(
        "task-discard-a-hand-edit-to-the-milestones-record",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::Edit(THE_RECORD)),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "reconciliation.conflict-block"),
        at("A5", Kind::EditAfter),
    ),
    discard(
        "task-discard-a-change-staged-at-the-record-and-not-in-the-file",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::Staged {
                at: THE_RECORD,
                kept: false,
            }),
        ],
        DISCARD_THE_SUB_TASK,
        Listed::Acts,
        at("A5", Kind::StagedThere),
    ),
    discard(
        "task-discard-a-file-staged-elsewhere",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::Staged {
                at: "notes/staged-by-its-user.txt",
                kept: true,
            }),
        ],
        DISCARD_THE_SUB_TASK,
        Listed::Acts,
        at("A6", Kind::StagedElsewhere),
    ),
    discard(
        "task-discard-a-staged-file-whose-name-git-quotes",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::Staged {
                at: A_NAME_GIT_QUOTES,
                kept: true,
            }),
        ],
        DISCARD_THE_SUB_TASK,
        Listed::Acts,
        at("A6", Kind::QuotedName),
    ),
    discard(
        "task-discard-a-hook-that-rejects",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::RejectingHook),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "`git commit` was rejected"),
        at("A6", Kind::RejectingHook),
    ),
    discard(
        "task-discard-a-foreign-file-under-the-file-state-hashes",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::File(".jigc/state/notes.md")),
        ],
        DISCARD_THE_SUB_TASK,
        Listed::Acts,
        at("A7", Kind::Untracked),
    ),
    // The base cell in the never-adopted repository: the command as typed, with no lead.
    Cell {
        state: Prepared::Bare,
        ..discard(
            "task-discard-a-sub-task-nobody-re-entered-in-bare",
            &[],
            DISCARD_THE_SUB_TASK,
            refuses(1, "no task `do-the-thing`"),
            None,
        )
    },
];

/// **The cells of `jigc task discard` at the record that stands** — `A5`'s needs a plant in
/// the place of a file that stands answers (the human's ruling of 2026-10-18): where the
/// milestone's committed record stood, a file of the user's (untracked: the index's entry
/// for the record goes with it), one the user committed — last
/// before the command, since the record stands only from the lead-in that mints the
/// milestone — three links and a directory; and a file in the place of the directory the
/// record stands in. Each is listed from the row's doc: the record is written through
/// *"the byte-stable item-leaf splice"* (`A5`), and what stands there is no record to splice
/// — an edit to the record *"conflict-blocks on drift"*, and a link or a directory is no
/// record at all — so each refuses, with the words read off a run where no doc gives them;
/// and with the record's directory gone there is no record to settle, which the discard
/// does *"before it destroys"* (`A8`), so it refuses there too — no run gives its words,
/// and any refusal names the record.
const DISCARD_AT_THE_RECORD: [Cell; 7] = [
    discard(
        "task-discard-a-file-of-the-users-in-the-records-place",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::InPlaceOf(&Plant::File(THE_RECORD))),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "reconciliation.conflict-block"),
        at("A5", Kind::Untracked),
    ),
    discard(
        "task-discard-a-file-the-user-committed-in-the-records-place",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::InPlaceOf(&Plant::Committed(THE_RECORD))),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "reconciliation.conflict-block"),
        at("A5", Kind::Committed),
    ),
    discard(
        "task-discard-a-link-to-nothing-in-the-records-place",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(THE_RECORD))),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "store.home-not-regular-file"),
        at("A5", Kind::LinkToNothing),
    ),
    discard(
        "task-discard-a-link-to-a-file-in-the-repository-in-the-records-place",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: THE_RECORD,
                outside: false,
            })),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "store.home-not-regular-file"),
        at("A5", Kind::LinkInside),
    ),
    discard(
        "task-discard-a-link-to-a-file-outside-in-the-records-place",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: THE_RECORD,
                outside: true,
            })),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "store.home-not-regular-file"),
        at("A5", Kind::LinkOutside),
    ),
    discard(
        "task-discard-a-directory-in-the-records-place",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::Plant(Plant::InPlaceOf(&Plant::Dir(THE_RECORD))),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "store.home-not-regular-file"),
        at("A5", Kind::DirForAFile),
    ),
    discard(
        "task-discard-a-file-in-the-place-of-the-records-directory",
        &[
            A_SUB_TASK[0],
            A_SUB_TASK[1],
            Lead::MovedAway("repo/docs/milestone-records"),
            Lead::Plant(Plant::File("docs/milestone-records")),
        ],
        DISCARD_THE_SUB_TASK,
        refuses(1, "docs/milestone-records/ship-it.md"),
        at("A5", Kind::FileForADir),
    ),
];

/// **The cells of the task family over a file simply deleted** — the human's ruling of
/// 2026-10-18 on the review of the task `B6`, option A: the file that stands at a line's
/// place is deleted by hand, outside jigc, nothing put in its place and the index left as
/// it stands — the plain `rm` ([`Plant::Deleted`], the plan's plant **E**: git lists the
/// path *deleted* among the changes not staged, as it lists an edit). A cell at each line
/// of the two rows that **E** targets, after the work unit's first write; before it
/// (`T1`, **E** before the first write) the task's own first write copies the doc in from
/// its home, which is gone, so the lead-in is refused — that write's question is the `doc`
/// family's. Each is listed from the row's doc:
///
/// - **the vision deleted after the task's first write to it** (`T1`), in a checkout that
///   holds its baseline: *"Weak signal — tracked path missing, no content-matching new
///   file. The tracked file is simply gone (deleted, accidentally removed)"*, blocking at
///   task scope and routed to restore (`design/reconciliation.md` → *Rename detection*) —
///   and *"a checkout holding a `file-state` key for the doc blocks first, on
///   `reconciliation.rename`"* (`design/finalize.md` → *4. Promote managed docs*);
/// - **jigc's stamp deleted** (`T2`): the stamp is refreshed *"where it is jigc's to
///   write"*, as an edit to it is — the cell acts;
/// - **the milestone's record deleted** (`A5`) — the shape the review of `B6` reproduced
///   at exit 0: there is no record to settle, which the discard does *"before it
///   destroys"* (`A8`), and the record `HEAD` holds names the sub-task — refused, as with
///   the record's directory gone, and any refusal names the record.
const FINALIZE_A_FILE_DELETED: [Cell; 2] = [
    finalize(
        "task-finalize-the-vision-deleted-after-the-first-write",
        SINGLETONS,
        &[
            A_TASK_OVER_THE_VISION[0],
            A_TASK_OVER_THE_VISION[1],
            A_TASK_OVER_THE_VISION[2],
            A_TASK_OVER_THE_VISION[3],
            A_TASK_OVER_THE_VISION[4],
            A_TASK_OVER_THE_VISION[5],
            Lead::Plant(Plant::Deleted("VISION.md")),
        ],
        FINALIZE_THE_VISION,
        refuses(3, "reconciliation.rename"),
        at("T1", Kind::EditAfter),
    ),
    finalize(
        "task-finalize-the-version-stamp-deleted",
        FRESH,
        a_decision!([], [Lead::Plant(Plant::Deleted(THE_STAMP))]),
        FINALIZE_THE_DECISION,
        Listed::Acts,
        at("T2", Kind::EditAfter),
    ),
];

/// The record deleted: [`FINALIZE_A_FILE_DELETED`] says how it is listed.
const DISCARD_A_FILE_DELETED: [Cell; 1] = [discard(
    "task-discard-the-milestones-record-deleted",
    &[
        A_SUB_TASK[0],
        A_SUB_TASK[1],
        Lead::Plant(Plant::Deleted(THE_RECORD)),
    ],
    DISCARD_THE_SUB_TASK,
    refuses(1, "docs/milestone-records/ship-it.md"),
    at("A5", Kind::EditAfter),
)];

/// One base cell of `jigc task discard` in a prepared state that is not its home.
const fn discard_in(name: &'static str, state: State) -> Cell {
    Cell {
        state: Prepared::Of(state),
        ..discard(
            name,
            &[A_SUB_TASK[0], A_SUB_TASK[1]],
            DISCARD_THE_SUB_TASK,
            Listed::Acts,
            None,
        )
    }
}

/// **The base cell of `jigc task discard` in the prepared states that are not its home**
/// — on demand, in no gate.
const DISCARD_ELSEWHERE: [Cell; 5] = [
    discard_in(
        "task-discard-a-sub-task-nobody-re-entered-in-committed-singletons",
        State::CommittedSingletons,
    ),
    discard_in(
        "task-discard-a-sub-task-nobody-re-entered-in-migrated",
        State::Migrated,
    ),
    discard_in(
        "task-discard-a-sub-task-nobody-re-entered-in-refs-post-hoc",
        State::RefsPostHoc,
    ),
    discard_in(
        "task-discard-a-sub-task-nobody-re-entered-in-chatty-hooks",
        State::ChattyHooks,
    ),
    discard_in(
        "task-discard-a-sub-task-nobody-re-entered-in-vendored",
        State::Vendored,
    ),
];

/// **The family's cells**: the acceptance's three, then the two leaves' own — the ones the
/// gate runs, in the arm of their state, and the base cells in the states that are not a
/// leaf's home, which run on demand.
/// [`A_TASK_WITH_A_CREATED_DOC`], with what is put in after its last write, last before
/// the command.
macro_rules! a_finding {
    ([$($after:expr),* $(,)?]) => {
        &[
            A_TASK_WITH_A_CREATED_DOC[0], A_TASK_WITH_A_CREATED_DOC[1],
            A_TASK_WITH_A_CREATED_DOC[2], A_TASK_WITH_A_CREATED_DOC[3],
            A_TASK_WITH_A_CREATED_DOC[4], A_TASK_WITH_A_CREATED_DOC[5],
            A_TASK_WITH_A_CREATED_DOC[6], A_TASK_WITH_A_CREATED_DOC[7],
            A_TASK_WITH_A_CREATED_DOC[8], A_TASK_WITH_A_CREATED_DOC[9],
            A_TASK_WITH_A_CREATED_DOC[10], A_TASK_WITH_A_CREATED_DOC[11],
            A_TASK_WITH_A_CREATED_DOC[12],
            $($after,)*
        ]
    };
}

/// One cell of `jigc task finalize`'s spelling `doc-only`: the finding's task, finalized.
const fn doc_only(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    Cell {
        spelling: Some("doc-only"),
        ..finalize(
            name,
            FRESH,
            leads,
            &["task", "finalize", "file-a-finding"],
            listed,
            aim,
        )
    }
}

/// **The cells of the spelling `doc-only`** — the human's ruling of 2026-10-18 on the two
/// spellings: a task whose workflow includes `step:finalize-doc-only` commits its own
/// paths and never the index, so a change the user staged elsewhere, or under a name git
/// quotes, **stays staged** (`T9`), and a hook that rejects the commit is a refusal that
/// leaves the task as it was (`T7`). The words of the refusal are a run's: *"`git commit`
/// was rejected"*, as the plain spelling's hook cell has them.
const FINALIZE_DOC_ONLY: [Cell; 4] = [
    doc_only(
        "task-finalize-doc-only-a-finding",
        a_finding!([]),
        Listed::Acts,
        None,
    ),
    doc_only(
        "task-finalize-doc-only-a-file-staged-elsewhere",
        a_finding!([Lead::Plant(Plant::Staged {
            at: "notes/staged-by-its-user.txt",
            kept: true,
        })]),
        Listed::Acts,
        at("T9", Kind::StagedElsewhere),
    ),
    doc_only(
        "task-finalize-doc-only-a-staged-file-whose-name-git-quotes",
        a_finding!([Lead::Plant(Plant::Staged {
            at: A_NAME_GIT_QUOTES,
            kept: true,
        })]),
        Listed::Acts,
        at("T9", Kind::QuotedName),
    ),
    doc_only(
        "task-finalize-doc-only-a-hook-that-rejects",
        a_finding!([Lead::Plant(Plant::RejectingHook)]),
        refuses(1, "`git commit` was rejected"),
        at("T9", Kind::RejectingHook),
    ),
];

// --- The spelling `--approve`: a migration task's retire ---------------------------

/// The foreign changelog a migration is of, committed by the user, and the task `jigc
/// migrate` mints for it.
const THE_HISTORY: &str = "HISTORY.md";
const A_HISTORY: &str = "# History\n\n## 1.0.0\n\n- Added a thing.\n";
const THE_MIGRATION: &str = "migrate-changelog-history-3268e06b69e1";

/// **The changelog the migration's author writes** — one release, one change-group, the
/// foreign file's one line: read with `--from-file`.
const THE_CHANGELOG: &str = "title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: \"1.0.0\"\n        sections:\n          - id: changes\n            items:\n              - title: \"added\"\n                set:\n                  notes: |-\n                    <<- Added a thing.>>\n";

/// **A migration task ready to finalize** — the user's `HISTORY.md` committed, `jigc
/// migrate` over it, and the canonical changelog authored in the task; what is put in
/// before the migration and after the authoring stands where the two lists say.
macro_rules! a_migration {
    ([$($before:expr),* $(,)?], [$($after:expr),* $(,)?]) => {
        &[
            Lead::Step(Step::File(Place::Repo(THE_HISTORY), A_HISTORY)),
            Lead::Step(Step::Git(&["add", "--", THE_HISTORY])),
            Lead::Step(Step::Git(&["commit", "-q", "-m", "a history"])),
            $($before,)*
            jigc(&["migrate", THE_HISTORY, "--as", "changelog"]),
            payload("changelog", THE_CHANGELOG),
            jigc(&[
                "doc",
                "author",
                "changelog",
                "--from-file",
                "../outside/changelog",
                "--task",
                THE_MIGRATION,
            ]),
            $($after,)*
        ]
    };
}

/// One cell of `jigc task finalize`'s spelling `--approve`: the migration task, approved.
const fn approves(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    Cell {
        spelling: Some("--approve"),
        ..finalize(
            name,
            FRESH,
            leads,
            &["task", "finalize", THE_MIGRATION, "--approve"],
            listed,
            aim,
        )
    }
}

/// **The cells of the spelling `--approve`** — a migration task's approved finalize, which
/// promotes the canonical doc, retires the foreign original and commits (`T10`, after the
/// review of the task `B9`, H6), each listed from the row's doc:
///
/// - **the base cell** retires a source `HEAD` holds as it stands, and acts;
/// - **a line added to the source after the mint** is refused at exit 3,
///   `reconciliation.conflict-block` — *"a source edited after the mint blocks"*, the source
///   compared with the seam the migration recorded (`design/reconciliation.md` → *Conflict —
///   block at file level*);
/// - **a file of the user's in the source's place, a line added before the migration copies
///   the source in, a change staged there, and a source staged and never committed** —
///   bytes no commit holds, which the seam records or does not see — are refused at exit 3,
///   naming the source, as the edit after the mint is: the retire *"deletes only a file git
///   holds a copy of"* on the warrant that it *"destroys only a reviewed, in-repo,
///   **git-recoverable** file"*. The doc's own test asks git for the path, not the bytes —
///   the rows' record's §6, entry 24, **ruled by the human on 2026-10-20**: the retire
///   deletes the source only where `HEAD` holds its bytes, both the working file's and the
///   index's, and otherwise refuses before anything is promoted, naming the source. The
///   listing is the ruling's; the cells, marked until it, are plain red under their key.
const FINALIZE_APPROVED: [Cell; 7] = [
    approves(
        "task-finalize-approve-a-migration",
        a_migration!([], []),
        Listed::Acts,
        None,
    ),
    approves(
        "task-finalize-approve-a-file-of-the-users-in-the-sources-place",
        a_migration!(
            [Lead::Plant(Plant::InPlaceOf(&Plant::File(THE_HISTORY)))],
            []
        ),
        refuses(3, THE_HISTORY),
        at("T10", Kind::Untracked),
    ),
    approves(
        "task-finalize-approve-an-edit-to-the-source-before-the-migration",
        a_migration!([Lead::Plant(Plant::Edit(THE_HISTORY))], []),
        refuses(3, THE_HISTORY),
        at("T10", Kind::EditBefore),
    ),
    approves(
        "task-finalize-approve-an-edit-to-the-source-after-the-authoring",
        a_migration!([], [Lead::Plant(Plant::Edit(THE_HISTORY))]),
        refuses(3, "reconciliation.conflict-block"),
        at("T10", Kind::EditAfter),
    ),
    // **E**'s other shape, the source deleted by hand after the authoring — the user's `rm`,
    // the index left as it stands: what the retire takes out of the index is what `HEAD`
    // holds there, and nothing of the user's is in the working tree to lose (the sixth of the
    // fixes after the review of the task `B9`, the fifth fix's left-open). Before the
    // migration the same deletion leaves `jigc migrate` no source, and the cell's own lead
    // refuses: no cell of **E** before stands on it.
    approves(
        "task-finalize-approve-the-source-deleted-after-the-authoring",
        a_migration!([], [Lead::Plant(Plant::Deleted(THE_HISTORY))]),
        Listed::Acts,
        at("T10", Kind::EditAfter),
    ),
    approves(
        "task-finalize-approve-a-change-staged-at-the-source",
        a_migration!(
            [],
            [Lead::Plant(Plant::Staged {
                at: THE_HISTORY,
                kept: false,
            })]
        ),
        refuses(3, THE_HISTORY),
        at("T10", Kind::StagedThere),
    ),
    // The source `git add`ed and never committed, which `jigc migrate` admits on purpose —
    // M51's *"staged-only cell"*, priced there (`DECISIONS.md`): the retire's commit takes
    // the index's entry away, and the bytes it held with it.
    approves(
        "task-finalize-approve-a-source-staged-and-never-committed",
        &[
            Lead::Plant(Plant::Staged {
                at: THE_HISTORY,
                kept: true,
            }),
            jigc(&["migrate", THE_HISTORY, "--as", "changelog"]),
            payload("changelog", THE_CHANGELOG),
            jigc(&[
                "doc",
                "author",
                "changelog",
                "--from-file",
                "../outside/changelog",
                "--task",
                THE_MIGRATION,
            ]),
        ],
        refuses(3, THE_HISTORY),
        at("T10", Kind::StagedThere),
    ),
];

pub const CELLS: &[&[Cell]] = &[
    ACCEPTANCE,
    &FINALIZE_CELLS,
    &FINALIZE_AT_THE_STAMP,
    &FINALIZE_A_FILE_DELETED,
    &FINALIZE_ELSEWHERE,
    &FINALIZE_DOC_ONLY,
    &FINALIZE_APPROVED,
    &DISCARD_CELLS,
    &DISCARD_AT_THE_RECORD,
    &DISCARD_A_FILE_DELETED,
    &DISCARD_ELSEWHERE,
];
