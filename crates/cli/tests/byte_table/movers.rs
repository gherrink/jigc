//! **The byte table's rows of the movers family** (`crates/cli/tests/byte_table.rs`) —
//! `jigc rename`, `jigc relocate`, `jigc migrate-corpus` and `jigc config set`, lifted
//! quote for quote from the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md` →
//! §4c), under the rules the install family's file states. Each writes the committed store
//! where it stands, and they split by whether the door commits: `rename` and
//! `migrate-corpus` land their writes in a commit of their own, `relocate` and `config set`
//! stage their moves and commit nothing.
//!
//! **What of the record is no line.** `RN6`, `RL5` and `CS6` are where *on a refusal: no*
//! comes from, and `RN7` names what `rename` refuses before its transaction. **`MG4` is a
//! second spelling**, `--no-commit`, and `MG5` a third, `--dry-run`: each a flag whose doc
//! promises fewer writes (the plan's §2), which a cell names.

use super::{ON_A_REFUSAL, THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ};
use crate::support::byte_cell::{
    A_DOC, A_PRIOR_DOC, Allows, Cell, Holding, Line, May, PRIOR_HOME, Prepared, Quote, Row,
    Spelling, THE_DOC, Unit, When, Where,
};
use crate::support::trial_corpus::State;

/// A line that holds where the command acted, whenever it did.
const fn line(id: &'static str, allows: Allows, quotes: &'static [Quote]) -> Line {
    Line {
        id,
        allows,
        when: When::ItCommitted,
        on_refusal: false,
        quotes,
        spelling: None,
    }
}

/// The heading of `jigc rename`'s section of `design/write-commands.md`.
const RENAME_SECTION: &str = "`jigc rename` — the CLI-owned identity refactor (M35)";

/// The heading of the freeze-exempt relocation's section of `design/corpus-migration.md`.
const THE_FREEZE_EXEMPT_SIBLING: &str =
    "The freeze-exempt sibling — the parallel relocation path (M39 inc-5)";

/// The heading of the relocation collisions' section of `design/reconciliation.md`.
const RELOCATION_COLLISIONS: &str =
    "Relocation collisions — two resolutions for one collision (M39)";

/// The heading of `migrate-corpus`'s commit boundary in `design/corpus-migration.md`.
const THE_COMMIT_BOUNDARY: &str = "The commit boundary — the verb lands its own migration (M42)";

/// The heading of `migrate-corpus`'s section of `design/write-commands.md`.
const MIGRATE_CORPUS_SECTION: &str =
    "`jigc migrate-corpus` — the commit boundary and its two opt-outs (M42)";

/// The heading of the placement section of `design/storage.md`.
const PLACEMENT: &str = "Placement — direct-file and root-located homes (M38)";

/// The heading of the `jigc config` verbs' section of `design/overrides.md`.
const THE_CONFIG_VERBS: &str = "Authoring deltas — the `jigc config` verbs";

/// **A mover's `git mv` lands in the index, and no commit** — `design/finalize.md`'s
/// sentence on the two leaves that move and commit nothing.
const A_MOVE_LANDS_IN_THE_INDEX: Quote = Quote {
    doc: "design/finalize.md",
    heading: "1. Preflight",
    words: "a `git mv` lands in the index and which commit it joins stays the user's to place",
};

/// **A foreign file at a destination is parked in the workbench** — the three sentences of
/// `design/reconciliation.md` `RL3` and `CS3` quote.
const A_SQUATTER_PARKED: [Quote; 3] = [
    Quote {
        doc: "design/reconciliation.md",
        heading: RELOCATION_COLLISIONS,
        words: "It is **relocated into the gitignored `.jigc/` workbench** (a dedicated `displaced/` subpath",
    },
    Quote {
        doc: "design/reconciliation.md",
        heading: RELOCATION_COLLISIONS,
        words: "a committed-but-unmanaged squatter's tracked-path index slot is freed so the managed `git mv` into the vacated destination succeeds",
    },
    Quote {
        doc: "design/reconciliation.md",
        heading: RELOCATION_COLLISIONS,
        words: "A name already taken in `.jigc/displaced/` gets the first free `<name>.<n>` beside it",
    },
];

/// **`jigc rename`'s transaction** — `RN3`'s second sentence and `RN4`'s first.
const THE_TRANSACTION: Quote = Quote {
    doc: "design/write-commands.md",
    heading: RENAME_SECTION,
    words: "**re-baselines `file-state`** (`forget(old)` + `record(new)` + re-record every rewritten referrer) and rebuilds the edge index, stages, runs the user's git hooks (never `--no-verify`), commits",
};

/// **The same transaction, as `design/reconciliation.md` has it** — `RN4`'s and `RN5`'s.
const ITS_REBASELINE: Quote = Quote {
    doc: "design/reconciliation.md",
    heading: "Rename detection",
    words: "then re-baseline `file-state` (`forget(old)` + `record(new)` + re-record each rewritten referrer) and invalidate/rebuild the index",
};

/// **`jigc rename`** — the record's §4c. Its unit is the doc its address names, the first
/// word after the leaf; its home is `fresh`, with the leads that commit a doc whose id can
/// move and one that refers to it — no prepared state holds a committed doc another refers
/// to.
pub const RENAME: Row = Row {
    leaf: &["rename"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "RN1",
            Allows::Files(Where::Path(THE_DOC), May::Retitled),
            &[
                Quote {
                    doc: "design/write-commands.md",
                    heading: RENAME_SECTION,
                    words: "**Rewrites the moved doc's H1** to `<New Title>` (the unified retitle), then **`git mv`** old→new path",
                },
                Quote {
                    doc: "design/write-commands.md",
                    heading: RENAME_SECTION,
                    words: "rename **degrades to a retitle-only**: rewrite the H1 and commit, with **no `git mv` and no referrer repoint**",
                },
            ],
        ),
        line(
            "RN2",
            Allows::Files(Where::Path(A_DOC), May::Repointed),
            &[
                Quote {
                    doc: "design/write-commands.md",
                    heading: RENAME_SECTION,
                    words: "rewrites each referrer's structured ref-field old→new across the **three persisted ref-fields** (`adr.supersedes`, `arch-doc.cites`, `spec.derived-from`)",
                },
                Quote {
                    doc: "design/write-commands.md",
                    heading: RENAME_SECTION,
                    words: "the CLI rewrites **structured ref-fields in the managed store** and nothing else",
                },
            ],
        ),
        line(
            "RN3",
            Allows::Commit(Holding::OnlyTheLines),
            &[
                Quote {
                    doc: "design/write-commands.md",
                    heading: RENAME_SECTION,
                    words: "it is **task-less and self-committing**, mutating the committed store directly",
                },
                THE_TRANSACTION,
            ],
        ),
        line(
            "RN4",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[THE_TRANSACTION, ITS_REBASELINE],
        ),
        line(
            "RN5",
            Allows::Files(
                Where::Under(".jigc/index"),
                May::Invalidated(".jigc/index/edges.json"),
            ),
            &[
                ITS_REBASELINE,
                THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ[0],
                THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ[1],
                THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ[2],
            ],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// **`jigc relocate`** — the record's §4c. Its unit is the doctype, the first word after
/// the leaf, and its prior home is `--from`'s; its home is `fresh`, with the leads that
/// make the door's domain — a doctype a listed pack supplies and no manifest governs.
pub const RELOCATE: Row = Row {
    leaf: &["relocate"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "RL1",
            Allows::Files(Where::Path(PRIOR_HOME), May::IntoItsHome),
            &[
                Quote {
                    doc: "design/corpus-migration.md",
                    heading: THE_FREEZE_EXEMPT_SIBLING,
                    words: "the door sweeps every committed `.md` under it into the doctype's current home",
                },
                Quote {
                    doc: "design/corpus-migration.md",
                    heading: THE_FREEZE_EXEMPT_SIBLING,
                    words: "The move is the **T1 standalone move primitive** (`relocate::move_doc` — `git mv old→new` + file-state re-key)",
                },
            ],
        ),
        line("RL2", Allows::Staged, &[A_MOVE_LANDS_IN_THE_INDEX]),
        line(
            "RL3",
            Allows::Files(Where::Path(A_DOC), May::MovedAsideTo(".jigc/displaced")),
            &A_SQUATTER_PARKED,
        ),
        line(
            "RL4",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[Quote {
                doc: "design/corpus-migration.md",
                heading: THE_FREEZE_EXEMPT_SIBLING,
                words: "The move is the **T1 standalone move primitive** (`relocate::move_doc` — `git mv old→new` + file-state re-key)",
            }],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// **A refusal of one doc, while the run's others migrate and commit** — the sentence the
/// lines of `migrate-corpus` that write take their *on a refusal: yes* from.
const ONE_DOC_REFUSED: Quote = Quote {
    doc: "design/finalize.md",
    heading: "4. Promote managed docs",
    words: "It is a refusal of **one doc**: like every refusal this command makes outside the transform's fold, the doc joins `blocked`, the run's other docs still migrate and commit, and the run exits non-zero",
};

/// A line of `migrate-corpus` that holds where the command refused having committed the
/// run's other docs — MG1's second sentence; a refusal that committed nothing changes
/// nothing, the reading of the record's §6, entry 22, until it is ruled.
const fn holds_on_a_refusal(line: Line) -> Line {
    Line {
        on_refusal: true,
        when: When::ItCommitted,
        ..line
    }
}

/// **`jigc migrate-corpus`** — the record's §4c. It names no unit; its home is `fresh`, with
/// the leads that commit a doc below its doctype's current schema version. **Two
/// spellings**: `--no-commit`, which takes `MG3` out of force — the reading until the
/// record's §6, entry 21 is ruled, under which no line of its own stages anything — and
/// `--dry-run`, which takes every line that writes out of force and reads the corpus.
pub const MIGRATE_CORPUS: Row = Row {
    leaf: &["migrate-corpus"],
    unit: Unit::Nothing,
    spellings: &[
        Spelling {
            name: "--no-commit",
            replaces: &["MG3"],
        },
        Spelling {
            name: "--dry-run",
            replaces: &["MG1", "MG1, from a prior home", "MG2", "MG3"],
        },
    ],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        holds_on_a_refusal(line(
            "MG1",
            Allows::Files(Where::Path(A_DOC), May::Migrated),
            &[
                Quote {
                    doc: "design/corpus-migration.md",
                    heading: "The determinism boundary holds (the auto-migration distinction)",
                    words: "The prose is **unchanged**",
                },
                ONE_DOC_REFUSED,
            ],
        )),
        holds_on_a_refusal(line(
            "MG1, from a prior home",
            Allows::Files(Where::Path(A_PRIOR_DOC), May::Migrated),
            &[
                Quote {
                    doc: "design/corpus-migration.md",
                    heading: "Relocation — the file-move + display-title transform (M38)",
                    words: "**write once to `to`** → `remove(from)` → re-key file-state `from → to`",
                },
                ONE_DOC_REFUSED,
            ],
        )),
        holds_on_a_refusal(line(
            "MG2",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/corpus-migration.md",
                    heading: THE_COMMIT_BOUNDARY,
                    words: "(`engine::state::persist` is an internal call that writes the bytes and flips the file-state baseline in one motion)",
                },
                ONE_DOC_REFUSED,
            ],
        )),
        holds_on_a_refusal(line(
            "MG3",
            Allows::Commit(Holding::OnlyTheLines),
            &[
                Quote {
                    doc: "design/corpus-migration.md",
                    heading: THE_COMMIT_BOUNDARY,
                    words: "`git add` over exactly the paths the migration touched (added, modified, and both halves of a move) — explicitly **never a blanket `git add -A`**, so an ambient dirty tree is never swept in",
                },
                ONE_DOC_REFUSED,
            ],
        )),
        Line {
            spelling: Some("--dry-run"),
            ..line(
                "MG5",
                Allows::Files(Where::Path(A_DOC), May::CopiedFrom),
                &[Quote {
                    doc: "design/write-commands.md",
                    heading: MIGRATE_CORPUS_SECTION,
                    words: "**`--dry-run`** reports what the migration *would* do and **writes nothing** — no disk write, no commit",
                }],
            )
        },
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// **`jigc config set`** — the record's §4c. Its unit is the knob, the first word after the
/// leaf; its home is `fresh`.
pub const CONFIG_SET: Row = Row {
    leaf: &["config", "set"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "CS1",
            Allows::Files(Where::Path(".jigc/config/manifest.yaml"), May::ItsKeySet),
            &[
                Quote {
                    doc: "design/overrides.md",
                    heading: THE_CONFIG_VERBS,
                    words: "| `jigc config set <key> <value>` | a `scalar-set` |",
                },
                Quote {
                    doc: "design/overrides.md",
                    heading: THE_CONFIG_VERBS,
                    words: "**by hand** (edit `manifest.yaml` + drop a native file",
                },
            ],
        ),
        line(
            "CS2",
            Allows::Files(Where::Path(A_DOC), May::IntoItsHome),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: PLACEMENT,
                    words: "`config set docs-root` now detects + routes + **auto-moves** each stranded instance",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: PLACEMENT,
                    words: "`jigc config set placement-root` **detects + routes + moves** every committed doc at a placement doctype's prior home *before* the knob lands",
                },
            ],
        ),
        line(
            "CS3",
            Allows::Files(Where::Path(A_DOC), May::MovedAsideTo(".jigc/displaced")),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: PLACEMENT,
                    words: "delegating to `relocate::relocate_stranded` so a foreign file squatting the new home is parked in the gitignored `.jigc/displaced/` workbench rather than clobbered",
                },
                A_SQUATTER_PARKED[0],
                A_SQUATTER_PARKED[1],
                A_SQUATTER_PARKED[2],
            ],
        ),
        line(
            "CS4",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[Quote {
                doc: "design/validation.md",
                heading: "The M52 registrations — Increment 5: the rollback-conflict family",
                words: "the gitignored `.jigc/state/file-state.json` the move primitive re-keys on every call",
            }],
        ),
        line(
            "CS5",
            Allows::Staged,
            &[
                A_MOVE_LANDS_IN_THE_INDEX,
                Quote {
                    doc: "design/overrides.md",
                    heading: THE_CONFIG_VERBS,
                    words: "none of the six verbs commits anything",
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
pub const ROWS: &[Row] = &[RENAME, RELOCATE, MIGRATE_CORPUS, CONFIG_SET];

/// The family's cells — owed by the task `B8` until they come, in the commits after its
/// rows (`owed.rs` → `CELLS_OWED`).
pub const CELLS: &[&[Cell]] = &[];
