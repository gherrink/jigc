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
//! promises fewer writes (the plan's §2), which a cell names — and each a line of its
//! spelling, `MG4` the staging the human's ruling on the record's §6, entry 21 gives it.

use super::task::A_DECISION as A_TASK_THAT_RECORDS_ONE_WAY;
use super::{
    AFTER_A_COMMIT, ON_A_REFUSAL, THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ, refuses,
    refuses_having_committed, refuses_having_moved,
};
use crate::support::byte_cell::{
    A_DOC, A_NEW_HOME, A_PRIOR_DOC, Aim, Allows, Cell, Holding, Kind, Lead, Line, Listed, May,
    PRIOR_HOME, Plant, Prepared, Quote, Row, Spelling, Stages, THE_DOC, THE_NEW_DOC, Typed, Unit,
    When, Where,
};
use crate::support::route_case::{Place, Step};
use crate::support::trial_corpus::State;

/// A line that holds where the command acted, whenever it did. (Its condition was *it
/// committed* until the eighth of the fixes after the review of `B8`, its R9: the same
/// reading with *on a refusal: no*, and a condition that belongs to `migrate-corpus`'s
/// lines that hold on a refusal, [`holds_on_a_refusal`].)
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

/// The heading of `jigc rename`'s section of `design/write-commands.md`.
const RENAME_SECTION: &str = "`jigc rename` — the CLI-owned identity refactor (M35)";

/// The heading of the freeze-exempt relocation's section of `design/corpus-migration.md`.
const THE_FREEZE_EXEMPT_SIBLING: &str =
    "The freeze-exempt sibling — the parallel relocation path (M39 inc-5)";

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

/// The heading of `DECISIONS.md`'s entry that records the human's two rulings of
/// 2026-10-20 — the task `B9`'s first commit.
const B9_RULINGS: &str = "2026-10-20 — The human's two rulings of the day, recorded: jigc never moves or overwrites a file to make room for its own, so a mover refuses over anything that stands at a destination — `relocate` blocking that doc, a root knob its whole re-point — and an acting sweep exits non-zero where an act it was asked for did not happen; the parking lines go, a mover's destination is a line's place, and `relocate`'s lines hold on a refusal that names the doc it left (the byte table, task B9, the first of its commits)";

/// **Nothing is moved aside from a destination** — the human's ruling of 2026-10-20 on the
/// rows' record's §6, entry 23, as `DECISIONS.md` records it, until the design docs say it:
/// the sentence a mover's destination line rests on beside its move's own.
const NOTHING_MOVED_ASIDE_FROM_A_DESTINATION: Quote = Quote {
    doc: "DECISIONS.md",
    heading: B9_RULINGS,
    words: "**Any file at a mover's destination — committed or untracked, managed or not — refuses**",
};

/// **`jigc relocate` blocks that doc and moves the others** — the same ruling, and the
/// human's ruling R1 of that day: the sentences `relocate`'s lines take *on a refusal: yes*
/// from, where the refusal names the doc it left.
const A_SWEEP_DOES_ALL_IT_CAN: [Quote; 2] = [
    Quote {
        doc: "DECISIONS.md",
        heading: B9_RULINGS,
        words: "`relocate` blocks that doc, moves the others, names the path to clear and exits non-zero",
    },
    Quote {
        doc: "DECISIONS.md",
        heading: B9_RULINGS,
        words: "**An acting sweep does all it can, then exits non-zero if any act it was asked for did not happen; a reporting door keeps exit 0 for what it finds.**",
    },
];

/// **`jigc rename` moves the doc and retitles it** — `RN1`'s first sentence, and `RN1, at
/// its new home`'s.
const THE_RETITLE: Quote = Quote {
    doc: "design/write-commands.md",
    heading: RENAME_SECTION,
    words: "**Rewrites the moved doc's H1** to `<New Title>` (the unified retitle), then **`git mv`** old→new path",
};

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
                THE_RETITLE,
                Quote {
                    doc: "design/write-commands.md",
                    heading: RENAME_SECTION,
                    words: "rename **degrades to a retitle-only**: rewrite the H1 and commit, with **no `git mv` and no referrer repoint**",
                },
            ],
        ),
        line(
            "RN1, at its new home",
            Allows::Files(Where::Path(THE_NEW_DOC), May::Retitled),
            &[
                THE_RETITLE,
                Quote {
                    doc: "design/write-commands.md",
                    heading: RENAME_SECTION,
                    words: "**Derives the new slug** = `slugify(\"<New Title>\")` (the same `engine::slug::slugify` the `create` path uses via `id-from: title`, so a renamed doc is byte-identical to a freshly-created one of that title), unless `--slug` overrides",
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
        AFTER_A_COMMIT,
    ],
};

/// The sentences `RL1` rests on: the sweep, and its move.
const THE_SWEEP: [Quote; 2] = [
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
];

/// A line of `jigc relocate` that holds where the command refused having left a doc of its
/// sweep and named it — the human's two rulings of 2026-10-20: `relocate` *blocks that doc,
/// moves the others, names the path to clear and exits non-zero*, and *an acting sweep does
/// all it can*. A refusal that names no doc it left changes nothing.
const fn swept(id: &'static str, allows: Allows, quotes: &'static [Quote]) -> Line {
    Line {
        on_refusal: true,
        when: When::ADocLeftAndNamed,
        ..line(id, allows, quotes)
    }
}

/// **`jigc relocate`** — the record's §4c. Its unit is the doctype, the first word after
/// the leaf, and its prior home is `--from`'s; its home is `fresh`, with the leads that
/// make the door's domain — a doctype a listed pack supplies and no manifest governs.
/// **Its lines that write hold on a refusal that names a doc the sweep left** — the human's
/// rulings of 2026-10-20 — and **nothing is moved aside from a destination**: what stands
/// at a doc's new home blocks that doc (the record's §6, entry 23, ruled that day; `RL3`,
/// the line that parked it, is gone).
pub const RELOCATE: Row = Row {
    leaf: &["relocate"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        swept(
            "RL1",
            Allows::Files(Where::Path(PRIOR_HOME), May::IntoItsHome),
            &[
                THE_SWEEP[0],
                THE_SWEEP[1],
                A_SWEEP_DOES_ALL_IT_CAN[0],
                A_SWEEP_DOES_ALL_IT_CAN[1],
            ],
        ),
        swept(
            "RL1, at its new home",
            Allows::Files(Where::Path(A_NEW_HOME), May::IntoItsHome),
            &[
                THE_SWEEP[0],
                NOTHING_MOVED_ASIDE_FROM_A_DESTINATION,
                A_SWEEP_DOES_ALL_IT_CAN[0],
                A_SWEEP_DOES_ALL_IT_CAN[1],
            ],
        ),
        swept(
            "RL2",
            Allows::Staged(Stages::MayStage),
            &[
                A_MOVE_LANDS_IN_THE_INDEX,
                A_SWEEP_DOES_ALL_IT_CAN[0],
                A_SWEEP_DOES_ALL_IT_CAN[1],
            ],
        ),
        swept(
            "RL4",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[
                THE_SWEEP[1],
                A_SWEEP_DOES_ALL_IT_CAN[0],
                A_SWEEP_DOES_ALL_IT_CAN[1],
            ],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
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
/// nothing — the reading of the record's §6, entry 22, which the human ruled on 2026-10-19:
/// a commit a hook rejects rolls back.
const fn holds_on_a_refusal(line: Line) -> Line {
    Line {
        on_refusal: true,
        when: When::ItCommitted,
        ..line
    }
}

/// **`jigc migrate-corpus`** — the record's §4c. It names no unit; its home is `fresh`, with
/// the leads that commit a doc below its doctype's current schema version. **Two
/// spellings**: `--no-commit`, which takes `MG3` out of force and stages what the lines
/// wrote (`MG4`) — the human's ruling of 2026-10-19 on the record's §6, entry 21; until it,
/// no line of its own staged anything — and `--dry-run`, which takes every line that writes
/// out of force and reads the corpus.
pub const MIGRATE_CORPUS: Row = Row {
    leaf: &["migrate-corpus"],
    unit: Unit::Nothing,
    spellings: &[
        Spelling {
            name: "--no-commit",
            typed: Typed::Word("--no-commit"),
            replaces: &["MG3"],
        },
        Spelling {
            name: "--dry-run",
            typed: Typed::Word("--dry-run"),
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
        // The human's ruling of 2026-10-19 on the record's §6, entry 21: `--no-commit` stages
        // exactly the paths the migration touched, both halves of a move, and commits
        // nothing — the sentence of `design/write-commands.md`, which the ruling found right.
        // A demand since the fourth of the fixes after the review of `B10` (its note 5): every
        // change of the migration's two lines stands staged, where it let the index be.
        Line {
            spelling: Some("--no-commit"),
            ..line(
                "MG4",
                Allows::Staged(Stages::AllTheChangesOf(&["MG1", "MG1, from a prior home"])),
                &[Quote {
                    doc: "design/write-commands.md",
                    heading: MIGRATE_CORPUS_SECTION,
                    words: "**`--no-commit`** performs the migration and leaves the changes staged for the operator to commit",
                }],
            )
        },
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
        AFTER_A_COMMIT,
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
            "CS2, at its new home",
            Allows::Files(Where::Path(A_NEW_HOME), May::IntoItsHome),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: PLACEMENT,
                    words: "`config set docs-root` now detects + routes + **auto-moves** each stranded instance",
                },
                NOTHING_MOVED_ASIDE_FROM_A_DESTINATION,
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
        // The knob and the moves its value needs stand staged together, or none of them does
        // — since the fourth of the fixes after the review of `B10` (its note 5), where the line
        // let the moves be staged and the knob they follow be left unstaged: one config change,
        // which reaches a commit with its task or on its own, never in halves (the findings,
        // §23, under the human's ruling on the record's §6, entry 19).
        line(
            "CS5",
            Allows::Staged(Stages::AllOrNoneOf(&["CS1", "CS2", "CS2, at its new home"])),
            &[
                A_MOVE_LANDS_IN_THE_INDEX,
                Quote {
                    doc: "design/overrides.md",
                    heading: THE_CONFIG_VERBS,
                    words: "none of the six verbs commits anything",
                },
                Quote {
                    doc: "DECISIONS.md",
                    heading: "The ruling on entry 19 — the human's, of 2026-10-19, option A, as the orchestrating session handed it on",
                    words: "Also a config change need to be committed. Either when it belongs into a task or individual.",
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
pub const ROWS: &[Row] = &[RENAME, RELOCATE, MIGRATE_CORPUS, CONFIG_SET];

// ---------------------------------------------------------------------------
// The cells
// ---------------------------------------------------------------------------

const FRESH: Prepared = Prepared::Of(State::Fresh);

/// The binary under test, in the repository.
const fn jigc(args: &'static [&'static str]) -> Lead {
    Lead::Step(Step::Jigc(args))
}

/// A file a lead writes, from the repository's root.
const fn file(path: &'static str, body: &'static str) -> Lead {
    Lead::Step(Step::File(Place::Repo(path), body))
}

/// `git`, as the user runs it in the repository.
const fn git(args: &'static [&'static str]) -> Lead {
    Lead::Step(Step::Git(args))
}

/// A payload a lead reads with `--from-file`: a file under `outside/`, which a step
/// started in the repository reaches as `../outside/<name>`.
const fn payload(name: &'static str, body: &'static str) -> Lead {
    Lead::Step(Step::File(Place::Outside(name), body))
}

/// What a planted cell's plant is aimed at.
const fn at(line: &'static str, kind: Kind) -> Option<Aim> {
    Some(Aim { line, kind })
}

/// One cell of the family's, plainly spelled and beside no suite.
const fn cell(
    name: &'static str,
    leaf: &'static [&'static str],
    state: Prepared,
    leads: &'static [Lead],
    argv: &'static [&'static str],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf,
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

/// The words a refusal over a repository with no `jigc setup` says; the words a run's.
const NOT_SET_UP: Listed = refuses(1, "this project isn't set up");

/// A change of the user's staged at a place the command does not write, and one under a
/// name git quotes — kept in the working file both.
const STAGED_ELSEWHERE: Lead = Lead::Plant(Plant::Staged {
    at: "notes/staged-by-its-user.txt",
    kept: true,
});
const A_NAME_GIT_QUOTES: &str = "notes/a \"quoted\" n\u{e4}me.txt";
const STAGED_UNDER_A_QUOTED_NAME: Lead = Lead::Plant(Plant::Staged {
    at: A_NAME_GIT_QUOTES,
    kept: true,
});

// --- `jigc rename` -----------------------------------------------------------------

/// The decision a rename moves, and the doc that cites it — a decision's home and an
/// architecture doc's, under the docs root of `fresh`.
const ONE_WAY: &str = "docs/decisions/one-way.md";
const GATEWAY: &str = "docs/architecture/gateway.md";
/// Where the decision stands once `--to "Single way"` has moved it: the slug the title gives.
const SINGLE_WAY: &str = "docs/decisions/single-way.md";
const A_DECISION: &str = "---\nschema-version: 2\nstatus: accepted\n---\n\n# One way\n\n## Context\n\nWe had to choose.\n\n## Options\n\n\n\n## Decision\n\nOne way.\n\n## Consequences\n\nIt holds.\n";
const A_DOC_THAT_CITES_IT: &str = "---\nschema-version: 1\ncites: [adr:one-way]\n---\n\n# Gateway\n\n## Overview\n\nThe gateway takes one way.\n\n## Components\n";

/// **A decision, and a doc that cites it, committed** — written by hand and committed by
/// the user, as a repository holds docs jigc was handed long ago: the rename's subject and
/// its referrer.
macro_rules! a_decision_and_its_citer {
    ($($after:expr),* $(,)?) => {
        &[
            file(ONE_WAY, A_DECISION),
            file(GATEWAY, A_DOC_THAT_CITES_IT),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "a decision, and a doc that cites it"]),
            $($after,)*
        ]
    };
}

const RENAME_IT: &[&str] = &["rename", "adr:one-way", "--to", "Single way"];

/// One cell of `jigc rename`.
const fn renames(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(name, &["rename"], FRESH, leads, RENAME_IT, listed, aim)
}

/// **The cells of `jigc rename`**, each listed from the row's doc:
///
/// - **the base cell** moves the decision to the slug `--to` gives and repoints the doc
///   that cites it (`RN1`, `RN2`), in one commit (`RN3`); **a retitle in place**, where
///   the new title slugs alike, rewrites the `# ` line and commits; and **with the edge
///   index built first** the rename takes its file away for the next read to rebuild
///   (`RN5`, the findings' §15 lead);
/// - **a state the transaction refuses before it writes** — *"**Validates** up front:
///   target exists · tree clean"* (`RN7`): a file of the user's in the doc's place or the
///   referrer's, a link, a directory, an edit in no commit, a change staged — at either
///   home or elsewhere — each leaves the tree not clean, and *"Every home the transaction
///   touches is a regular file, or the door refuses"*; where the doc gives both, the code
///   is the run's — `store.home-not-regular-file` at the doc's own home — and a file in
///   the place of the decisions' directory leaves no target (`store.not-found`);
/// - **what squats the doc's new home** (`RN1, at its new home`) — a file of the user's,
///   one committed, a link, a directory: *"a collision with a **different** existing doc
///   **blocks**"*, *"A *regular file* at the destination is still the collision arm's
///   `write.already-present`"*, and a link or a directory there answers
///   `store.home-not-regular-file`: *"The three homes now answer alike"*;
/// - **a hook that rejects** the commit: *"on **any** pre-commit failure restores all
///   captured bytes"* (`RN6`);
/// - **a file of the user's in a cache** (`RN4`, `RN5`) stays.
const RENAME_CELLS: [Cell; 37] = [
    cell(
        "rename-a-decision-the-catalog-names",
        &["rename"],
        FRESH,
        &[],
        &["rename", "adr:a-decision", "--to", "A Decision"],
        refuses(1, "store.not-found"),
        None,
    ),
    renames(
        "rename-one-way-to-single-way",
        a_decision_and_its_citer!(),
        Listed::Acts,
        None,
    ),
    cell(
        "rename-one-way-retitled-in-place",
        &["rename"],
        FRESH,
        a_decision_and_its_citer!(),
        &["rename", "adr:one-way", "--to", "One Way"],
        Listed::Acts,
        None,
    ),
    renames(
        "rename-one-way-with-the-edge-index-built",
        a_decision_and_its_citer!(
            jigc(&["start", "--workflow", "single-task", "build the index"]),
            Lead::Refused(&["task", "validate", "build-the-index"]),
            jigc(&["task", "discard", "build-the-index", "--force"]),
        ),
        Listed::Acts,
        None,
    ),
    // `RN1`, at the decision's home.
    renames(
        "rename-a-file-of-the-users-in-the-decisions-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::File(ONE_WAY)))),
        refuses(1, "rename.dirty-tree"),
        at("RN1", Kind::Untracked),
    ),
    renames(
        "rename-a-link-to-nothing-in-the-decisions-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            ONE_WAY
        )))),
        refuses(1, "store.home-not-regular-file"),
        at("RN1", Kind::LinkToNothing),
    ),
    renames(
        "rename-a-link-to-a-file-in-the-repository-in-the-decisions-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ONE_WAY,
            outside: false,
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("RN1", Kind::LinkInside),
    ),
    renames(
        "rename-a-link-to-a-file-outside-in-the-decisions-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ONE_WAY,
            outside: true,
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("RN1", Kind::LinkOutside),
    ),
    renames(
        "rename-a-directory-in-the-decisions-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(ONE_WAY)))),
        refuses(1, "store.home-not-regular-file"),
        at("RN1", Kind::DirForAFile),
    ),
    renames(
        "rename-a-file-in-the-place-of-the-decisions-directory",
        a_decision_and_its_citer!(
            Lead::MovedAway("repo/docs/decisions"),
            Lead::Plant(Plant::File("docs/decisions")),
        ),
        refuses(1, "store.not-found"),
        at("RN1", Kind::FileForADir),
    ),
    renames(
        "rename-an-edit-to-the-decision",
        a_decision_and_its_citer!(Lead::Plant(Plant::Edit(ONE_WAY))),
        refuses(1, "rename.dirty-tree"),
        at("RN1", Kind::EditAfter),
    ),
    renames(
        "rename-a-change-staged-at-the-decision",
        a_decision_and_its_citer!(Lead::Plant(Plant::Staged {
            at: ONE_WAY,
            kept: false,
        })),
        refuses(1, "rename.dirty-tree"),
        at("RN1", Kind::StagedThere),
    ),
    // `RN1, at its new home`: what squats the decision's new home.
    renames(
        "rename-a-file-of-the-users-at-the-decisions-new-home",
        a_decision_and_its_citer!(Lead::Plant(Plant::File(SINGLE_WAY))),
        refuses(1, "write.already-present"),
        at("RN1, at its new home", Kind::Untracked),
    ),
    renames(
        "rename-a-file-the-user-committed-at-the-decisions-new-home",
        a_decision_and_its_citer!(Lead::Plant(Plant::Committed(SINGLE_WAY))),
        refuses(1, "write.already-present"),
        at("RN1, at its new home", Kind::Committed),
    ),
    renames(
        "rename-a-link-to-nothing-at-the-decisions-new-home",
        a_decision_and_its_citer!(Lead::Plant(Plant::LinkToNothing(SINGLE_WAY))),
        refuses(1, "store.home-not-regular-file"),
        at("RN1, at its new home", Kind::LinkToNothing),
    ),
    renames(
        "rename-a-link-to-a-file-in-the-repository-at-the-decisions-new-home",
        a_decision_and_its_citer!(Lead::Plant(Plant::LinkToAFile {
            at: SINGLE_WAY,
            outside: false,
        })),
        refuses(1, "store.home-not-regular-file"),
        at("RN1, at its new home", Kind::LinkInside),
    ),
    renames(
        "rename-a-link-to-a-file-outside-at-the-decisions-new-home",
        a_decision_and_its_citer!(Lead::Plant(Plant::LinkToAFile {
            at: SINGLE_WAY,
            outside: true,
        })),
        refuses(1, "store.home-not-regular-file"),
        at("RN1, at its new home", Kind::LinkOutside),
    ),
    renames(
        "rename-a-directory-at-the-decisions-new-home",
        a_decision_and_its_citer!(Lead::Plant(Plant::Dir(SINGLE_WAY))),
        refuses(1, "store.home-not-regular-file"),
        at("RN1, at its new home", Kind::DirForAFile),
    ),
    // `RN2`, at the citing doc's home.
    renames(
        "rename-a-file-of-the-users-in-the-citing-docs-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::File(GATEWAY)))),
        refuses(1, "rename.dirty-tree"),
        at("RN2", Kind::Untracked),
    ),
    renames(
        "rename-a-link-to-nothing-in-the-citing-docs-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            GATEWAY
        )))),
        refuses(1, "rename.dirty-tree"),
        at("RN2", Kind::LinkToNothing),
    ),
    renames(
        "rename-a-link-to-a-file-in-the-repository-in-the-citing-docs-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: GATEWAY,
            outside: false,
        }))),
        refuses(1, "rename.dirty-tree"),
        at("RN2", Kind::LinkInside),
    ),
    renames(
        "rename-a-link-to-a-file-outside-in-the-citing-docs-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: GATEWAY,
            outside: true,
        }))),
        refuses(1, "rename.dirty-tree"),
        at("RN2", Kind::LinkOutside),
    ),
    renames(
        "rename-a-directory-in-the-citing-docs-place",
        a_decision_and_its_citer!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(GATEWAY)))),
        refuses(1, "rename.dirty-tree"),
        at("RN2", Kind::DirForAFile),
    ),
    renames(
        "rename-a-file-in-the-place-of-the-citing-docs-directory",
        a_decision_and_its_citer!(
            Lead::MovedAway("repo/docs/architecture"),
            Lead::Plant(Plant::File("docs/architecture")),
        ),
        refuses(1, "rename.dirty-tree"),
        at("RN2", Kind::FileForADir),
    ),
    renames(
        "rename-an-edit-to-the-citing-doc",
        a_decision_and_its_citer!(Lead::Plant(Plant::Edit(GATEWAY))),
        refuses(1, "rename.dirty-tree"),
        at("RN2", Kind::EditAfter),
    ),
    renames(
        "rename-a-change-staged-at-the-citing-doc",
        a_decision_and_its_citer!(Lead::Plant(Plant::Staged {
            at: GATEWAY,
            kept: false,
        })),
        refuses(1, "rename.dirty-tree"),
        at("RN2", Kind::StagedThere),
    ),
    // `RN3`, the commit.
    renames(
        "rename-a-file-staged-elsewhere",
        a_decision_and_its_citer!(STAGED_ELSEWHERE),
        refuses(1, "rename.dirty-tree"),
        at("RN3", Kind::StagedElsewhere),
    ),
    renames(
        "rename-a-staged-file-whose-name-git-quotes",
        a_decision_and_its_citer!(STAGED_UNDER_A_QUOTED_NAME),
        refuses(1, "rename.dirty-tree"),
        at("RN3", Kind::QuotedName),
    ),
    renames(
        "rename-a-hook-that-rejects",
        a_decision_and_its_citer!(Lead::Plant(Plant::RejectingHook)),
        refuses(1, "`git commit` was rejected"),
        at("RN3", Kind::RejectingHook),
    ),
    // `RN4`, `RN5`, the caches.
    renames(
        "rename-a-file-of-the-users-beside-the-baseline",
        a_decision_and_its_citer!(Lead::Plant(Plant::File(".jigc/state/notes.md"))),
        Listed::Acts,
        at("RN4", Kind::Untracked),
    ),
    renames(
        "rename-a-file-of-the-users-under-the-edge-index",
        a_decision_and_its_citer!(Lead::Plant(Plant::File(".jigc/index/notes.md"))),
        Listed::Acts,
        at("RN5", Kind::Untracked),
    ),
    // The base cell in each other prepared state, over the decision and the doc that cites it
    // its home cells stand on, so that the state's character meets the door — with no lead it
    // refused for the missing decision, `store.not-found` (the sixth of the fixes after the
    // review of the task `B9`, its N9, the pattern there and here); in `bare` they are
    // committed by git alone, and the door refuses for the state's own character.
    cell(
        "rename-one-way-in-bare",
        &["rename"],
        Prepared::Bare,
        a_decision_and_its_citer!(),
        RENAME_IT,
        NOT_SET_UP,
        None,
    ),
    cell(
        "rename-one-way-in-committed-singletons",
        &["rename"],
        Prepared::Of(State::CommittedSingletons),
        a_decision_and_its_citer!(),
        RENAME_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "rename-one-way-in-migrated",
        &["rename"],
        Prepared::Of(State::Migrated),
        a_decision_and_its_citer!(),
        RENAME_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "rename-one-way-in-refs-post-hoc",
        &["rename"],
        Prepared::Of(State::RefsPostHoc),
        a_decision_and_its_citer!(),
        RENAME_IT,
        // The state's live task: a rename is *"blocked whenever **any task working area *or*
        // milestone is in-flight**"* (`design/write-commands.md` → `jigc rename`), the door's
        // own refusal, `rename.in-flight`.
        refuses(1, "rename.in-flight"),
        None,
    ),
    cell(
        "rename-one-way-in-chatty-hooks",
        &["rename"],
        Prepared::Of(State::ChattyHooks),
        a_decision_and_its_citer!(),
        RENAME_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "rename-one-way-in-vendored",
        &["rename"],
        Prepared::Of(State::Vendored),
        a_decision_and_its_citer!(),
        RENAME_IT,
        Listed::Acts,
        None,
    ),
];

// --- `jigc migrate-corpus` ---------------------------------------------------------

/// A decision at schema version 1 — the `adr` doctype's shape before its `## Options`
/// section (`schema-snapshots/adr.v1.yaml`) — at its home.
const A_DECISION_AT_V1: &str = "---\nschema-version: 1\nstatus: accepted\n---\n\n# One way\n\n## Context\n\nWe had to choose.\n\n## Decision\n\nOne way.\n\n## Consequences\n\nIt holds.\n";

/// The changelog at schema version 1, at the home its doctype had then — `changelog/`
/// under the docs root, its lowercase title — as `corpus_migration.rs` commits it.
const THE_CHANGELOGS_OLD_HOME: &str = "docs/changelog/changelog.md";
/// The changelog's home at its current version, a placement at the repository's root.
const THE_CHANGELOGS_HOME: &str = "CHANGELOG.md";
const A_CHANGELOG_AT_V1: &str =
    "---\nschema-version: 1\n---\n\n# changelog\n\n## Unreleased Changes\n\n## Releases\n";

/// **A decision below its doctype's version, committed** — written by hand and committed
/// by the user, as a corpus jigc stamped before a schema bump stands: `migrate-corpus`'s
/// subject.
macro_rules! a_stale_decision {
    ($($after:expr),* $(,)?) => {
        &[
            file(ONE_WAY, A_DECISION_AT_V1),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "a decision below its version"]),
            $($after,)*
        ]
    };
}

/// **The changelog at the home its doctype had at version 1, committed.**
macro_rules! a_stale_changelog {
    ($($after:expr),* $(,)?) => {
        &[
            file(THE_CHANGELOGS_OLD_HOME, A_CHANGELOG_AT_V1),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "the changelog at its old home"]),
            $($after,)*
        ]
    };
}

/// **A decision jigc tracks, below its doctype's version** — landed by a task of its own,
/// so the file-state baseline holds it, then rewritten at version 1 and committed by the
/// user: the doc whose baseline a migration flips (`MG2`). A doc jigc never tracked adopts
/// no baseline (`migrate_corpus.rs`: *"an untracked doc adopts nothing"*).
macro_rules! a_tracked_stale_decision {
    ($($after:expr),* $(,)?) => {
        &[
            A_TASK_THAT_RECORDS_ONE_WAY[0],
            A_TASK_THAT_RECORDS_ONE_WAY[1],
            A_TASK_THAT_RECORDS_ONE_WAY[2],
            A_TASK_THAT_RECORDS_ONE_WAY[3],
            A_TASK_THAT_RECORDS_ONE_WAY[4],
            A_TASK_THAT_RECORDS_ONE_WAY[5],
            A_TASK_THAT_RECORDS_ONE_WAY[6],
            A_TASK_THAT_RECORDS_ONE_WAY[7],
            A_TASK_THAT_RECORDS_ONE_WAY[8],
            A_TASK_THAT_RECORDS_ONE_WAY[9],
            A_TASK_THAT_RECORDS_ONE_WAY[10],
            jigc(&["task", "finalize", "record-a-decision"]),
            file(ONE_WAY, A_DECISION_AT_V1),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "the decision below its version"]),
            $($after,)*
        ]
    };
}

const MIGRATE: &[&str] = &["migrate-corpus"];

/// One cell of `jigc migrate-corpus`.
const fn migrates(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(
        name,
        &["migrate-corpus"],
        FRESH,
        leads,
        MIGRATE,
        listed,
        aim,
    )
}

/// **The cells of `jigc migrate-corpus`**, each listed from the row's doc:
///
/// - **the base cells** migrate a decision below its version where it stands (`MG1`)
///   and the changelog from its old home to `CHANGELOG.md` (`MG1, from a prior home`),
///   each in one commit (`MG3`), and a decision a task landed, so that the file-state
///   baseline holds it and the migration flips it (`MG2`); the leaf as typed in `fresh`
///   finds nothing stale, and acts;
/// - **at a doc's home, what is no doc of its type**: a file of the user's there is a file
///   the verb *"is not for"* — *"a file it calls foreign never becomes a `PreparedDoc`"*
///   (`design/corpus-migration.md` → *The foreign arm*) — and the run acts having written
///   nothing; a link or a directory is *"refused under `store.home-not-regular-file`"*,
///   one doc, and the run exits non-zero (`design/finalize.md` → *4. Promote managed
///   docs*); a file in the place of the home's directory leaves no doc to find;
/// - **an edit in no commit** at the doc, and **a change staged** at it: the migration
///   reads the working file and lands it — and the doc of the commit says *"an ambient
///   dirty tree is never swept in"*, so each is listed as acting, and its commit is held
///   to what `HEAD` held;
/// - **what is staged elsewhere** stays out of the commit, *"`git add` over exactly the
///   paths the migration touched"*; **a hook that rejects** the commit refuses, and leaves
///   nothing changed — the human's ruling of 2026-10-19 on the record's §6, entry 22;
/// - **a file of the user's beside the baseline** (`MG2`), over the decision a task
///   landed, stays.
const MIGRATE_CORPUS_CELLS: [Cell; 42] = [
    migrates("migrate-corpus-the-catalog-names", &[], Listed::Acts, None),
    migrates(
        "migrate-corpus-a-decision-below-its-version",
        a_stale_decision!(),
        Listed::Acts,
        None,
    ),
    migrates(
        "migrate-corpus-a-decision-jigc-tracks-below-its-version",
        a_tracked_stale_decision!(),
        Listed::Acts,
        None,
    ),
    migrates(
        "migrate-corpus-the-changelog-at-its-old-home",
        a_stale_changelog!(),
        Listed::Acts,
        None,
    ),
    // The two spellings' base cells.
    Cell {
        spelling: Some("--no-commit"),
        ..cell(
            "migrate-corpus-no-commit-a-decision-below-its-version",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(),
            &["migrate-corpus", "--no-commit"],
            Listed::Acts,
            None,
        )
    },
    Cell {
        spelling: Some("--dry-run"),
        ..cell(
            "migrate-corpus-dry-run-a-decision-below-its-version",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(),
            &["migrate-corpus", "--dry-run"],
            Listed::Acts,
            None,
        )
    },
    // `MG1`, at the stale decision's home.
    migrates(
        "migrate-corpus-a-file-of-the-users-in-the-decisions-place",
        a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::File(ONE_WAY)))),
        Listed::Acts,
        at("MG1", Kind::Untracked),
    ),
    migrates(
        "migrate-corpus-a-link-to-nothing-in-the-decisions-place",
        a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            ONE_WAY
        )))),
        refuses(1, "store.home-not-regular-file"),
        at("MG1", Kind::LinkToNothing),
    ),
    migrates(
        "migrate-corpus-a-link-to-a-file-in-the-repository-in-the-decisions-place",
        a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ONE_WAY,
            outside: false,
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("MG1", Kind::LinkInside),
    ),
    migrates(
        "migrate-corpus-a-link-to-a-file-outside-in-the-decisions-place",
        a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ONE_WAY,
            outside: true,
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("MG1", Kind::LinkOutside),
    ),
    migrates(
        "migrate-corpus-a-directory-in-the-decisions-place",
        a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(ONE_WAY)))),
        refuses(1, "store.home-not-regular-file"),
        at("MG1", Kind::DirForAFile),
    ),
    migrates(
        "migrate-corpus-a-file-in-the-place-of-the-decisions-directory",
        a_stale_decision!(
            Lead::MovedAway("repo/docs/decisions"),
            Lead::Plant(Plant::File("docs/decisions")),
        ),
        Listed::Acts,
        at("MG1", Kind::FileForADir),
    ),
    migrates(
        "migrate-corpus-an-edit-to-the-decision",
        a_stale_decision!(Lead::Plant(Plant::Edit(ONE_WAY))),
        Listed::Acts,
        at("MG1", Kind::EditAfter),
    ),
    migrates(
        "migrate-corpus-a-change-staged-at-the-decision",
        a_stale_decision!(Lead::Plant(Plant::Staged {
            at: ONE_WAY,
            kept: false,
        })),
        Listed::Acts,
        at("MG1", Kind::StagedThere),
    ),
    // `MG1, from a prior home`, at the changelog's old home.
    migrates(
        "migrate-corpus-a-file-of-the-users-in-the-old-changelogs-place",
        a_stale_changelog!(Lead::Plant(Plant::InPlaceOf(&Plant::File(
            THE_CHANGELOGS_OLD_HOME
        )))),
        Listed::Acts,
        at("MG1, from a prior home", Kind::Untracked),
    ),
    migrates(
        "migrate-corpus-a-link-to-nothing-in-the-old-changelogs-place",
        a_stale_changelog!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            THE_CHANGELOGS_OLD_HOME
        )))),
        refuses(1, "store.home-not-regular-file"),
        at("MG1, from a prior home", Kind::LinkToNothing),
    ),
    migrates(
        "migrate-corpus-a-link-to-a-file-in-the-repository-in-the-old-changelogs-place",
        a_stale_changelog!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: THE_CHANGELOGS_OLD_HOME,
            outside: false,
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("MG1, from a prior home", Kind::LinkInside),
    ),
    migrates(
        "migrate-corpus-a-link-to-a-file-outside-in-the-old-changelogs-place",
        a_stale_changelog!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: THE_CHANGELOGS_OLD_HOME,
            outside: true,
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("MG1, from a prior home", Kind::LinkOutside),
    ),
    migrates(
        "migrate-corpus-a-directory-in-the-old-changelogs-place",
        a_stale_changelog!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(
            THE_CHANGELOGS_OLD_HOME
        )))),
        refuses(1, "store.home-not-regular-file"),
        at("MG1, from a prior home", Kind::DirForAFile),
    ),
    migrates(
        "migrate-corpus-a-file-in-the-place-of-the-old-changelogs-directory",
        a_stale_changelog!(
            Lead::MovedAway("repo/docs/changelog"),
            Lead::Plant(Plant::File("docs/changelog")),
        ),
        Listed::Acts,
        at("MG1, from a prior home", Kind::FileForADir),
    ),
    migrates(
        "migrate-corpus-an-edit-to-the-old-changelog",
        a_stale_changelog!(Lead::Plant(Plant::Edit(THE_CHANGELOGS_OLD_HOME))),
        Listed::Acts,
        at("MG1, from a prior home", Kind::EditAfter),
    ),
    migrates(
        "migrate-corpus-a-change-staged-at-the-old-changelog",
        a_stale_changelog!(Lead::Plant(Plant::Staged {
            at: THE_CHANGELOGS_OLD_HOME,
            kept: false,
        })),
        Listed::Acts,
        at("MG1, from a prior home", Kind::StagedThere),
    ),
    // `MG2`, `MG3`.
    migrates(
        "migrate-corpus-a-file-of-the-users-beside-the-baseline",
        a_tracked_stale_decision!(Lead::Plant(Plant::File(".jigc/state/notes.md"))),
        Listed::Acts,
        at("MG2", Kind::Untracked),
    ),
    migrates(
        "migrate-corpus-a-file-staged-elsewhere",
        a_stale_decision!(STAGED_ELSEWHERE),
        Listed::Acts,
        at("MG3", Kind::StagedElsewhere),
    ),
    migrates(
        "migrate-corpus-a-staged-file-whose-name-git-quotes",
        a_stale_decision!(STAGED_UNDER_A_QUOTED_NAME),
        Listed::Acts,
        at("MG3", Kind::QuotedName),
    ),
    migrates(
        "migrate-corpus-a-hook-that-rejects",
        a_stale_decision!(Lead::Plant(Plant::RejectingHook)),
        refuses(1, "`git commit` was rejected"),
        at("MG3", Kind::RejectingHook),
    ),
    // Two stale docs, one blocked: the changelog at its earlier home, where `CHANGELOG.md` is
    // a file the user committed — *"the doc is **blocked with a route**; nothing is written,
    // nothing is removed"* — and the decision beside it, migrated and committed: *"the run's
    // other docs still migrate and commit, and the run exits non-zero"*, the human's ruling
    // R1 of 2026-10-20 (*an acting sweep does all it can*). The decision's migration in the
    // commit is listed: a refusal of the whole run is not this outcome (the sixth of the fixes
    // after the review of the task `B9`, the first fix's left-open).
    migrates(
        "migrate-corpus-a-file-the-user-committed-at-the-changelogs-home-and-a-decision-below-its-version",
        &[
            file(ONE_WAY, A_DECISION_AT_V1),
            file(THE_CHANGELOGS_OLD_HOME, A_CHANGELOG_AT_V1),
            file(THE_CHANGELOGS_HOME, "the user's own changelog\n"),
            git(&["add", "--", "docs", THE_CHANGELOGS_HOME]),
            git(&[
                "commit",
                "-q",
                "-m",
                "two docs below their versions, and a changelog of the user's",
            ]),
        ],
        refuses_having_committed(1, "migrate-corpus.destination-collision", &[ONE_WAY]),
        None,
    ),
    // A doc git never held, below its version, at the decision's home and at the changelog's
    // earlier home: the walk *"enumerates a doctype's **committed** instances"* and *"re-parses
    // every committed doc"* (`design/corpus-migration.md`), so the run leaves it as it stands
    // and commits nothing of it — the user's bytes, never committed, are no subject of a
    // migration's commit (the sixth of the fixes after the review of the task `B9`: the second
    // fix's left-open, *migrated* narrowed to a doc git holds).
    migrates(
        "migrate-corpus-a-decision-below-its-version-git-never-held",
        &[file(ONE_WAY, A_DECISION_AT_V1)],
        Listed::Acts,
        None,
    ),
    migrates(
        "migrate-corpus-the-changelog-at-its-old-home-git-never-held",
        &[file(THE_CHANGELOGS_OLD_HOME, A_CHANGELOG_AT_V1)],
        Listed::Acts,
        None,
    ),
    // `MG5`, under `--dry-run`: the corpus read where a stale doc stands — what stands in
    // the doc's place there is what the run reads, and nothing is written over it.
    Cell {
        spelling: Some("--dry-run"),
        ..cell(
            "migrate-corpus-dry-run-a-file-of-the-users-in-the-decisions-place",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::File(ONE_WAY)))),
            &["migrate-corpus", "--dry-run"],
            Listed::Acts,
            at("MG5", Kind::Untracked),
        )
    },
    Cell {
        spelling: Some("--dry-run"),
        ..cell(
            "migrate-corpus-dry-run-a-link-to-nothing-in-the-decisions-place",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
                ONE_WAY
            )))),
            &["migrate-corpus", "--dry-run"],
            refuses(1, "store.home-not-regular-file"),
            at("MG5", Kind::LinkToNothing),
        )
    },
    Cell {
        spelling: Some("--dry-run"),
        ..cell(
            "migrate-corpus-dry-run-a-link-to-a-file-in-the-repository-in-the-decisions-place",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: ONE_WAY,
                outside: false
            }))),
            &["migrate-corpus", "--dry-run"],
            refuses(1, "store.home-not-regular-file"),
            at("MG5", Kind::LinkInside),
        )
    },
    Cell {
        spelling: Some("--dry-run"),
        ..cell(
            "migrate-corpus-dry-run-a-link-to-a-file-outside-in-the-decisions-place",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: ONE_WAY,
                outside: true
            }))),
            &["migrate-corpus", "--dry-run"],
            refuses(1, "store.home-not-regular-file"),
            at("MG5", Kind::LinkOutside),
        )
    },
    Cell {
        spelling: Some("--dry-run"),
        ..cell(
            "migrate-corpus-dry-run-a-directory-in-the-decisions-place",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(ONE_WAY)))),
            &["migrate-corpus", "--dry-run"],
            refuses(1, "store.home-not-regular-file"),
            at("MG5", Kind::DirForAFile),
        )
    },
    Cell {
        spelling: Some("--dry-run"),
        ..cell(
            "migrate-corpus-dry-run-the-decision-deleted",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(Lead::Plant(Plant::Deleted(ONE_WAY))),
            &["migrate-corpus", "--dry-run"],
            Listed::Acts,
            at("MG5", Kind::EditBefore),
        )
    },
    Cell {
        spelling: Some("--dry-run"),
        ..cell(
            "migrate-corpus-dry-run-an-edit-to-the-decision",
            &["migrate-corpus"],
            FRESH,
            a_stale_decision!(Lead::Plant(Plant::Edit(ONE_WAY))),
            &["migrate-corpus", "--dry-run"],
            Listed::Acts,
            at("MG5", Kind::EditAfter),
        )
    },
    cell(
        "migrate-corpus-in-bare",
        &["migrate-corpus"],
        Prepared::Bare,
        &[],
        MIGRATE,
        NOT_SET_UP,
        None,
    ),
    cell(
        "migrate-corpus-in-committed-singletons",
        &["migrate-corpus"],
        Prepared::Of(State::CommittedSingletons),
        &[],
        MIGRATE,
        Listed::Acts,
        None,
    ),
    cell(
        "migrate-corpus-in-migrated",
        &["migrate-corpus"],
        Prepared::Of(State::Migrated),
        &[],
        MIGRATE,
        Listed::Acts,
        None,
    ),
    cell(
        "migrate-corpus-in-refs-post-hoc",
        &["migrate-corpus"],
        Prepared::Of(State::RefsPostHoc),
        &[],
        MIGRATE,
        Listed::Acts,
        None,
    ),
    cell(
        "migrate-corpus-in-chatty-hooks",
        &["migrate-corpus"],
        Prepared::Of(State::ChattyHooks),
        &[],
        MIGRATE,
        Listed::Acts,
        None,
    ),
    cell(
        "migrate-corpus-in-vendored",
        &["migrate-corpus"],
        Prepared::Of(State::Vendored),
        &[],
        MIGRATE,
        Listed::Acts,
        None,
    ),
];

/// The family's cells: the doors that commit — `rename`, `migrate-corpus` — and the
/// movers, `relocate` and `config set`, the two parts the docs split the family in.
pub const CELLS: &[&[Cell]] = &[
    &RENAME_CELLS,
    &MIGRATE_CORPUS_CELLS,
    &RELOCATE_CELLS,
    &CONFIG_SET_CELLS,
];

// --- `jigc relocate` ---------------------------------------------------------------

/// The listed pack's `note` doctype — freeze-exempt, homed at `notes/` under the docs root
/// — the door's domain: no manifest governs it.
const A_NOTE_SCHEMA: &str = "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n";
/// The packs the project composes: the note pack, from the repository's root, beside the
/// methodology pack `fresh` composes.
const PACKS_WITH_THE_NOTE_PACK: &str =
    "packs:\n  - note-pack\ncompose-embedded-methodology: true\n";
const A_NOTE: &str = "# Cache benchmarks\n\n## Body\n\nA single node caps throughput.\n";
/// The note where it stood before its doctype's home moved, and its home now.
const STRANDED: &str = "docs/legacy-notes/cache-benchmarks.md";
const THE_NOTES_HOME: &str = "docs/notes/cache-benchmarks.md";
/// A doc of the user's in the note's new home, beside the place the note lands at.
const BESIDE_THE_NOTES_HOME: &str = "docs/notes/the-users-own-notes.md";
/// A `.md` of the user's at the prior home, beside the stranded note: one git never held.
const BESIDE_THE_STRANDED_NOTE: &str = "docs/legacy-notes/scratch.md";
/// A second note stranded at the prior home, and its bytes.
const A_SECOND_STRANDED: &str = "docs/legacy-notes/load-tests.md";
const A_SECOND_NOTE: &str = "# Load tests\n\n## Body\n\nTwo nodes hold the load.\n";
/// The second note moved into its home: what a sweep that blocks the first note still does.
const THE_SECOND_NOTE_MOVED: &[(&str, &str)] = &[(A_SECOND_STRANDED, "docs/notes/load-tests.md")];

/// **A note stranded at a prior home, committed** — the note pack and its listing, and a
/// note under `docs/legacy-notes/`, committed by the user: `relocate`'s subject.
/// **The note pack and a stranded note, committed by git alone** — what of
/// [`a_stranded_note!`] stands in a repository before `jigc setup`: the project's pack list is
/// `jigc`'s configuration, and not written.
const A_STRANDED_NOTE_BY_GIT_ALONE: &[Lead] = &[
    file("note-pack/schemas/note.yaml", A_NOTE_SCHEMA),
    file(STRANDED, A_NOTE),
    git(&["add", "--", "note-pack", "docs"]),
    git(&[
        "commit",
        "-q",
        "-m",
        "a note pack, and a note at its old home",
    ]),
];

macro_rules! a_stranded_note {
    ($($after:expr),* $(,)?) => {
        &[
            file("note-pack/schemas/note.yaml", A_NOTE_SCHEMA),
            file(".jigc/config/packs.yaml", PACKS_WITH_THE_NOTE_PACK),
            file(STRANDED, A_NOTE),
            git(&["add", "--", "note-pack", ".jigc/config/packs.yaml", "docs"]),
            git(&["commit", "-q", "-m", "a note pack, and a note at its old home"]),
            $($after,)*
        ]
    };
}

/// **A note jigc manages at its home, and a note of the same name stranded at the prior
/// home** — the note pack listed and a note committed at `docs/notes/`, which a task of its
/// own writes and lands, so the file-state baseline records it; then the stranded note,
/// committed by the user: a destination a managed instance occupies.
macro_rules! a_managed_note_at_the_new_home {
    ($($after:expr),* $(,)?) => {
        &[
            file("note-pack/schemas/note.yaml", A_NOTE_SCHEMA),
            file(".jigc/config/packs.yaml", PACKS_WITH_THE_NOTE_PACK),
            file(THE_NOTES_HOME, A_MANAGED_NOTE),
            git(&["add", "--", "note-pack", ".jigc/config/packs.yaml", "docs"]),
            git(&["commit", "-q", "-m", "a note pack, and a note at its home"]),
            jigc(&["start", "--workflow", "single-task", "touch a note"]),
            payload("body", "Benchmarked again.\n"),
            jigc(&[
                "doc",
                "set-slot",
                "note:cache-benchmarks#body",
                "--from-file",
                "../outside/body",
                "--task",
                "touch-a-note",
            ]),
            jigc(&[
                "doc",
                "set-field",
                "commit:touch-a-note#type",
                "--value",
                "docs",
                "--task",
                "touch-a-note",
            ]),
            payload("summary", "touch a note\n"),
            jigc(&[
                "doc",
                "set-slot",
                "commit:touch-a-note#summary",
                "--from-file",
                "../outside/summary",
                "--task",
                "touch-a-note",
            ]),
            jigc(&["task", "finalize", "touch-a-note"]),
            file(STRANDED, A_NOTE),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "a note at its old home"]),
            $($after,)*
        ]
    };
}
/// The note jigc manages at its home.
const A_MANAGED_NOTE: &str = "# Cache benchmarks\n\n## Body\n\nThe managed one.\n";

const RELOCATE_IT: &[&str] = &["relocate", "note", "--from", "docs/legacy-notes/"];

/// One cell of `jigc relocate`.
const fn relocates(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(name, &["relocate"], FRESH, leads, RELOCATE_IT, listed, aim)
}

/// **The cells of `jigc relocate`**, each listed from the row's doc:
///
/// - **the base cell** moves the stranded note into its home with a staged `git mv`
///   (`RL1`, `RL2`) and re-keys the baseline (`RL4`); the leaf as the catalog types it
///   names a frozen doctype, and is refused;
/// - **at the stranded note's place**: a link or a directory is refused —
///   *"A relocation lands a regular file or refuses, keyed at the source"*
///   (`design/finalize.md` → *4. Promote managed docs*); a file of the user's, an edit, a
///   change staged move with the doc, and a file in the place of its directory leaves no
///   doc to move; **beside it**, a `.md` of the user's git never held and a link stay
///   where they stood — *"every **committed** `.md` under it"*;
/// - **at the note's new home** (`RL1, at its new home`), anything that stands blocks the
///   move, naming the home — a file of the user's, one the user committed, a link, a
///   directory: the human's ruling of 2026-10-20 on the record's §6, entry 23, *jigc never
///   moves or overwrites a file to make room for its own*; **a note jigc manages** there
///   blocks it, under the code the run gives — *"A *managed* instance already at the
///   destination … the relocation **surfaces the collision as a block** (never a clobber)"*
///   (`design/reconciliation.md` → *Relocation collisions*); **a doc of the user's beside
///   the new home**, untracked or committed, blocks nothing — the ruling's (A);
/// - **a second note beside a blocked first** — a link in the first's place, a file of the
///   user's at its new home — is moved, and the refusal lists the move: *an acting sweep
///   does all it can* (the human's ruling R1 of 2026-10-20), `relocate` *moves the others*
///   (entry 23), and a refusal of the whole sweep is not that outcome;
/// - **a file of the user's beside the baseline** stays.
const RELOCATE_CELLS: [Cell; 30] = [
    cell(
        "relocate-the-changelog-the-catalog-names",
        &["relocate"],
        FRESH,
        &[],
        &["relocate", "changelog", "--from", "docs"],
        refuses(1, "relocate.frozen-doctype"),
        None,
    ),
    relocates(
        "relocate-a-stranded-note",
        a_stranded_note!(),
        Listed::Acts,
        None,
    ),
    // `RL1`, at the stranded note's place.
    relocates(
        "relocate-a-file-of-the-users-in-the-stranded-notes-place",
        a_stranded_note!(Lead::Plant(Plant::InPlaceOf(&Plant::File(STRANDED)))),
        Listed::Acts,
        at("RL1", Kind::Untracked),
    ),
    relocates(
        "relocate-a-link-to-nothing-in-the-stranded-notes-place",
        a_stranded_note!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            STRANDED
        )))),
        refuses(1, "store.home-not-regular-file"),
        at("RL1", Kind::LinkToNothing),
    ),
    relocates(
        "relocate-a-link-to-a-file-in-the-repository-in-the-stranded-notes-place",
        a_stranded_note!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: STRANDED,
            outside: false
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("RL1", Kind::LinkInside),
    ),
    relocates(
        "relocate-a-link-to-a-file-outside-in-the-stranded-notes-place",
        a_stranded_note!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: STRANDED,
            outside: true
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("RL1", Kind::LinkOutside),
    ),
    relocates(
        "relocate-a-directory-in-the-stranded-notes-place",
        a_stranded_note!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(STRANDED)))),
        refuses(1, "store.home-not-regular-file"),
        at("RL1", Kind::DirForAFile),
    ),
    relocates(
        "relocate-a-file-in-the-place-of-the-legacy-notes-directory",
        a_stranded_note!(
            Lead::MovedAway("repo/docs/legacy-notes"),
            Lead::Plant(Plant::File("docs/legacy-notes"))
        ),
        Listed::Acts,
        at("RL1", Kind::FileForADir),
    ),
    relocates(
        "relocate-an-edit-to-the-stranded-note",
        a_stranded_note!(Lead::Plant(Plant::Edit(STRANDED))),
        Listed::Acts,
        at("RL1", Kind::EditAfter),
    ),
    relocates(
        "relocate-a-change-staged-at-the-stranded-note",
        a_stranded_note!(Lead::Plant(Plant::Staged {
            at: STRANDED,
            kept: false
        })),
        Listed::Acts,
        at("RL1", Kind::StagedThere),
    ),
    // Beside the stranded note, a `.md` of the user's git never held, and a link: neither is
    // a committed `.md`, so neither moves — *"every **committed** `.md` under it"*, *"A
    // relocation **lands a regular file** or refuses"* (the review of the task `B9`, H2,
    // where no cell planted either and the word that judges a move took both).
    relocates(
        "relocate-a-md-of-the-users-beside-the-stranded-note",
        a_stranded_note!(Lead::Plant(Plant::File(BESIDE_THE_STRANDED_NOTE))),
        Listed::Acts,
        at("RL1", Kind::Untracked),
    ),
    relocates(
        "relocate-a-link-to-nothing-beside-the-stranded-note",
        a_stranded_note!(Lead::Plant(Plant::LinkToNothing(BESIDE_THE_STRANDED_NOTE))),
        Listed::Acts,
        at("RL1", Kind::LinkToNothing),
    ),
    // `RL1, at its new home`: whatever stands at the note's new home blocks the note, and the
    // refusal names the path to clear — nothing is moved aside from a destination (the
    // human's ruling of 2026-10-20 on the record's §6, entry 23).
    relocates(
        "relocate-a-file-of-the-users-at-the-notes-new-home",
        a_stranded_note!(Lead::Plant(Plant::File(THE_NOTES_HOME))),
        refuses(1, THE_NOTES_HOME),
        at("RL1, at its new home", Kind::Untracked),
    ),
    relocates(
        "relocate-a-file-the-user-committed-at-the-notes-new-home",
        a_stranded_note!(Lead::Plant(Plant::Committed(THE_NOTES_HOME))),
        refuses(1, THE_NOTES_HOME),
        at("RL1, at its new home", Kind::Committed),
    ),
    relocates(
        "relocate-a-link-to-nothing-at-the-notes-new-home",
        a_stranded_note!(Lead::Plant(Plant::LinkToNothing(THE_NOTES_HOME))),
        refuses(1, THE_NOTES_HOME),
        at("RL1, at its new home", Kind::LinkToNothing),
    ),
    relocates(
        "relocate-a-link-to-a-file-in-the-repository-at-the-notes-new-home",
        a_stranded_note!(Lead::Plant(Plant::LinkToAFile {
            at: THE_NOTES_HOME,
            outside: false
        })),
        refuses(1, THE_NOTES_HOME),
        at("RL1, at its new home", Kind::LinkInside),
    ),
    relocates(
        "relocate-a-link-to-a-file-outside-at-the-notes-new-home",
        a_stranded_note!(Lead::Plant(Plant::LinkToAFile {
            at: THE_NOTES_HOME,
            outside: true
        })),
        refuses(1, THE_NOTES_HOME),
        at("RL1, at its new home", Kind::LinkOutside),
    ),
    relocates(
        "relocate-a-directory-at-the-notes-new-home",
        a_stranded_note!(Lead::Plant(Plant::Dir(THE_NOTES_HOME))),
        refuses(1, THE_NOTES_HOME),
        at("RL1, at its new home", Kind::DirForAFile),
    ),
    // Its twin under the same ruling's (A): a doc of the user's elsewhere in the note's new
    // home blocks nothing — the note moves, the doc stays where it stood.
    relocates(
        "relocate-a-doc-of-the-users-beside-the-notes-new-home",
        a_stranded_note!(Lead::Plant(Plant::File(BESIDE_THE_NOTES_HOME))),
        Listed::Acts,
        at("RL1, at its new home", Kind::Untracked),
    ),
    // And the same doc committed — the ruling's *"an unmanaged doc"*, whatever git holds of it
    // (the sixth of the fixes after the review of the task `B9`, its N10).
    relocates(
        "relocate-a-doc-the-user-committed-beside-the-notes-new-home",
        a_stranded_note!(Lead::Plant(Plant::Committed(BESIDE_THE_NOTES_HOME))),
        Listed::Acts,
        at("RL1, at its new home", Kind::Committed),
    ),
    // A second stranded note, and a link in the first's place: the sweep blocks the first and
    // moves the second, then exits non-zero naming what it left — the human's rulings of
    // 2026-10-20, R1 and entry 23. The second note's move is listed: a refusal of the whole
    // sweep is not this outcome (the review of the task `B9`, H1).
    relocates(
        "relocate-a-link-to-nothing-in-one-stranded-notes-place-and-a-second-note",
        a_stranded_note!(
            file(A_SECOND_STRANDED, A_SECOND_NOTE),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "a second note at its old home"]),
            Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(STRANDED)))
        ),
        refuses_having_moved(1, "store.home-not-regular-file", THE_SECOND_NOTE_MOVED),
        at("RL1", Kind::LinkToNothing),
    ),
    // And a file of the user's at the first note's new home, beside a second note: the sweep
    // blocks the first, naming the path to clear, and moves the second — entry 23's *moves
    // the others*, which no cell with one note can tell from a refusal of the whole sweep.
    relocates(
        "relocate-a-file-of-the-users-at-the-notes-new-home-and-a-second-note",
        a_stranded_note!(
            file(A_SECOND_STRANDED, A_SECOND_NOTE),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "a second note at its old home"]),
            Lead::Plant(Plant::File(THE_NOTES_HOME))
        ),
        refuses_having_moved(1, THE_NOTES_HOME, THE_SECOND_NOTE_MOVED),
        at("RL1, at its new home", Kind::Untracked),
    ),
    // A managed note at the new home blocks the note too.
    relocates(
        "relocate-a-note-a-task-landed-at-the-notes-new-home",
        a_managed_note_at_the_new_home!(),
        refuses(1, "write.already-present"),
        None,
    ),
    // `RL4`, the baseline.
    relocates(
        "relocate-a-file-of-the-users-beside-the-baseline",
        a_stranded_note!(Lead::Plant(Plant::File(".jigc/state/notes.md"))),
        Listed::Acts,
        at("RL4", Kind::Untracked),
    ),
    // The base cell in each other prepared state, over the note pack and the stranded note its
    // home cells stand on, so that the state's character meets the door — with no lead it
    // refused for the missing doctype, `store.unknown-type` (the sixth of the fixes after the
    // review of the task `B9`, its N9); in `bare` the pack and the note are committed by git
    // alone, and the project's pack list, which is `jigc`'s configuration, is not written.
    cell(
        "relocate-a-note-in-bare",
        &["relocate"],
        Prepared::Bare,
        A_STRANDED_NOTE_BY_GIT_ALONE,
        RELOCATE_IT,
        NOT_SET_UP,
        None,
    ),
    cell(
        "relocate-a-note-in-committed-singletons",
        &["relocate"],
        Prepared::Of(State::CommittedSingletons),
        a_stranded_note!(),
        RELOCATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "relocate-a-note-in-migrated",
        &["relocate"],
        Prepared::Of(State::Migrated),
        a_stranded_note!(),
        RELOCATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "relocate-a-note-in-refs-post-hoc",
        &["relocate"],
        Prepared::Of(State::RefsPostHoc),
        a_stranded_note!(),
        RELOCATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "relocate-a-note-in-chatty-hooks",
        &["relocate"],
        Prepared::Of(State::ChattyHooks),
        a_stranded_note!(),
        RELOCATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "relocate-a-note-in-vendored",
        &["relocate"],
        Prepared::Of(State::Vendored),
        a_stranded_note!(),
        RELOCATE_IT,
        Listed::Acts,
        None,
    ),
];

// --- `jigc config set` -------------------------------------------------------------

/// The project manifest, where `config set` records a knob.
const THE_MANIFEST: &str = ".jigc/config/manifest.yaml";
const A_MANIFEST: &str = "scalar:\n  finalize.fan-out.squash: true\n";
/// A manifest the user wrote, a comment of their own in it.
const A_MANIFEST_WITH_A_COMMENT: &str =
    "# the user's own note on the knobs below\nscalar:\n  finalize.fan-out.squash: true\n";
/// The decision's home once `docs-root` is `notes`.
const THE_DECISIONS_NEW_HOME: &str = "notes/decisions/one-way.md";
/// A doc of the user's in the decision's new home, beside the place the decision lands at.
const BESIDE_THE_DECISIONS_NEW_HOME: &str = "notes/decisions/the-users-own-notes.md";

/// **A decision, committed** — written by hand and committed by the user: the doc a
/// `docs-root` re-point strands, and moves.
macro_rules! a_committed_decision {
    ($($after:expr),* $(,)?) => {
        &[
            file(ONE_WAY, A_DECISION),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "a decision"]),
            $($after,)*
        ]
    };
}

const SET_DOCS_ROOT: &[&str] = &["config", "set", "docs-root", "notes"];

/// The roadmap's home, the methodology pack's `placement: { file: docs/roadmap.md }`, and
/// its home once `placement-root` is `notes`.
const THE_ROADMAP: &str = "docs/roadmap.md";
const THE_ROADMAPS_NEW_HOME: &str = "notes/roadmap.md";
/// A doc of the user's under the new root, beside the place the roadmap lands at.
const BESIDE_THE_ROADMAPS_NEW_HOME: &str = "notes/the-users-own-notes.md";
/// A roadmap, as `committed-singletons` holds one.
const A_ROADMAP: &str = "---\nschema-version: 1\n---\n\n# Roadmap\n\n## Milestones\n\n### M-Alpha  {#m-alpha}\n\n#### Proves\n\nThat the composed loop lands one task end to end.\n\n#### Decomposition\n\nIncrement 1 — the enumeration seam. Increment 2 — the fixture builder.\n";

/// **A roadmap, committed** — written by hand and committed by the user: the doc a
/// `placement-root` re-point strands, and moves (`fresh` composes the methodology pack).
macro_rules! a_committed_roadmap {
    ($($after:expr),* $(,)?) => {
        &[
            file(THE_ROADMAP, A_ROADMAP),
            git(&["add", "--", "docs"]),
            git(&["commit", "-q", "-m", "a roadmap"]),
            $($after,)*
        ]
    };
}

const SET_PLACEMENT_ROOT: &[&str] = &["config", "set", "placement-root", "notes"];

/// One cell of `jigc config set placement-root` — the other root knob, whose re-point
/// moves through `relocate::relocate_stranded` where `docs-root`'s has a loop of its own
/// (`design/storage.md` → *Placement*).
const fn sets_the_placement_root(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(
        name,
        &["config", "set"],
        FRESH,
        leads,
        SET_PLACEMENT_ROOT,
        listed,
        aim,
    )
}

/// One cell of `jigc config set`.
const fn sets(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(
        name,
        &["config", "set"],
        FRESH,
        leads,
        SET_DOCS_ROOT,
        listed,
        aim,
    )
}

/// **The cells of `jigc config set`**, each listed from the row's doc:
///
/// - **the base cell** re-points `docs-root` over a committed decision: the knob set in
///   the manifest (`CS1`), the decision moved into its new home with a staged `git mv`
///   (`CS2`, `CS5`), the baseline re-keyed (`CS4`); a knob that moves nothing; and the knob
///   set over a manifest that holds a comment of the user's — the doc's two *"co-equal"*
///   ways to author a delta, **by hand** and through the verbs (`design/overrides.md` →
///   *Authoring deltas*), ask the verb to keep what the hand wrote; the leaf as the catalog
///   types it sets `docs-root` to its default;
/// - **at the manifest**: a change staged is kept, the knob's line beside it; what does not
///   read as knobs — a file of the user's, a line of theirs, an edit that breaks the YAML —
///   the verb refuses, as the run refuses the edit, *"not valid YAML"*, and so a link or a
///   directory, no file to set a knob in (no doc says so: the sensible reading, and a
///   finding where the run says otherwise); a file in the place of the config layer's
///   directory leaves no project set up;
/// - **at the decision's home**, as `relocate`'s at the stranded note's; **at its new
///   home** (`CS2, at its new home`), anything that stands refuses the whole re-point, the
///   knob unchanged — the human's ruling of 2026-10-20 on the record's §6, entry 23, for a
///   root knob; **a decision jigc manages** there blocks it, as it blocks `relocate`
///   (`design/reconciliation.md` → *Relocation collisions*); a doc of the user's beside the
///   new home, untracked or committed, blocks nothing — the ruling's (A);
/// - **a file of the user's beside the baseline** stays;
/// - **the other root knob, `placement-root`**, over a committed roadmap — its re-point
///   moves through `relocate::relocate_stranded`, which *"a foreign file squatting the new
///   home is parked"* names (`design/storage.md` → *Placement*), where `docs-root`'s moves
///   through a loop of its own: the base cell; **at the roadmap's place** and **at its new
///   home** every plant the decision's two places take, the mechanism being another; a file
///   in the place of the roadmap's directory leaves a committed doc the re-point cannot
///   move, and a root knob that cannot move a doc it strands refuses the re-point whole,
///   naming it — the knob unchanged, as a link or a directory in the doc's place is refused
///   (a doc of the re-point blocked while the knob lands is a doc left at a home the cascade
///   no longer resolves); anything at the new home refuses it whole, by the ruling on entry
///   23 — and a roadmap jigc moved there blocks it. The
///   manifest, the baseline and the staging are the knob's write and the move primitive's,
///   one for both knobs, and their plants stand at `docs-root`'s cells (added 2026-10-20,
///   after the review of the task `B8`, H5).
const CONFIG_SET_CELLS: [Cell; 55] = [
    cell(
        "config-set-docs-root-the-catalog-names",
        &["config", "set"],
        FRESH,
        &[],
        &["config", "set", "docs-root", "docs"],
        Listed::Acts,
        None,
    ),
    sets(
        "config-set-docs-root-to-notes",
        a_committed_decision!(),
        Listed::Acts,
        None,
    ),
    cell(
        "config-set-a-knob-that-moves-nothing",
        &["config", "set"],
        FRESH,
        a_committed_decision!(),
        &["config", "set", "finalize.fan-out.squash", "false"],
        Listed::Acts,
        None,
    ),
    cell(
        "config-set-over-a-manifest-that-holds-the-users-comment",
        &["config", "set"],
        FRESH,
        a_committed_decision!(file(THE_MANIFEST, A_MANIFEST_WITH_A_COMMENT)),
        &["config", "set", "finalize.fan-out.squash", "false"],
        Listed::Acts,
        None,
    ),
    // `CS1`, at the project manifest.
    sets(
        "config-set-a-file-of-the-users-in-the-manifests-place",
        a_committed_decision!(Lead::Plant(Plant::File(THE_MANIFEST))),
        refuses(1, "config.repoint-failed"),
        at("CS1", Kind::Untracked),
    ),
    sets(
        "config-set-a-link-to-nothing-in-the-manifests-place",
        a_committed_decision!(Lead::Plant(Plant::LinkToNothing(THE_MANIFEST))),
        refuses(1, "store.home-not-regular-file"),
        at("CS1", Kind::LinkToNothing),
    ),
    sets(
        "config-set-a-link-to-a-file-in-the-repository-in-the-manifests-place",
        a_committed_decision!(Lead::Plant(Plant::LinkToAFile {
            at: THE_MANIFEST,
            outside: false
        })),
        refuses(1, "store.home-not-regular-file"),
        at("CS1", Kind::LinkInside),
    ),
    sets(
        "config-set-a-link-to-a-file-outside-in-the-manifests-place",
        a_committed_decision!(Lead::Plant(Plant::LinkToAFile {
            at: THE_MANIFEST,
            outside: true
        })),
        refuses(1, "store.home-not-regular-file"),
        at("CS1", Kind::LinkOutside),
    ),
    sets(
        "config-set-a-directory-in-the-manifests-place",
        a_committed_decision!(Lead::Plant(Plant::Dir(THE_MANIFEST))),
        refuses(1, "config.repoint-failed"),
        at("CS1", Kind::DirForAFile),
    ),
    sets(
        "config-set-a-file-in-the-place-of-the-config-layers-directory",
        a_committed_decision!(
            Lead::MovedAway("repo/.jigc/config"),
            Lead::Plant(Plant::File(".jigc/config"))
        ),
        NOT_SET_UP,
        at("CS1", Kind::FileForADir),
    ),
    sets(
        "config-set-a-line-of-the-users-in-the-manifest",
        a_committed_decision!(Lead::Plant(Plant::UsersLine(THE_MANIFEST))),
        refuses(1, "config.repoint-failed"),
        at("CS1", Kind::UsersLine),
    ),
    sets(
        "config-set-an-edit-to-the-committed-manifest",
        a_committed_decision!(
            file(THE_MANIFEST, A_MANIFEST),
            git(&["add", "--", ".jigc/config"]),
            git(&["commit", "-q", "-m", "a manifest"]),
            Lead::Plant(Plant::Edit(THE_MANIFEST))
        ),
        refuses(1, "config.repoint-failed"),
        at("CS1", Kind::EditAfter),
    ),
    sets(
        "config-set-a-change-staged-at-the-manifest",
        a_committed_decision!(Lead::Plant(Plant::Staged {
            at: THE_MANIFEST,
            kept: false
        })),
        Listed::Acts,
        at("CS1", Kind::StagedThere),
    ),
    // `CS2`, at the decision's home.
    sets(
        "config-set-a-file-of-the-users-in-the-decisions-place",
        a_committed_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::File(ONE_WAY)))),
        Listed::Acts,
        at("CS2", Kind::Untracked),
    ),
    sets(
        "config-set-a-link-to-nothing-in-the-decisions-place",
        a_committed_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            ONE_WAY
        )))),
        refuses(1, "store.home-not-regular-file"),
        at("CS2", Kind::LinkToNothing),
    ),
    sets(
        "config-set-a-link-to-a-file-in-the-repository-in-the-decisions-place",
        a_committed_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ONE_WAY,
            outside: false
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("CS2", Kind::LinkInside),
    ),
    sets(
        "config-set-a-link-to-a-file-outside-in-the-decisions-place",
        a_committed_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ONE_WAY,
            outside: true
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("CS2", Kind::LinkOutside),
    ),
    sets(
        "config-set-a-directory-in-the-decisions-place",
        a_committed_decision!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(ONE_WAY)))),
        refuses(1, "store.home-not-regular-file"),
        at("CS2", Kind::DirForAFile),
    ),
    sets(
        "config-set-a-file-in-the-place-of-the-decisions-directory",
        a_committed_decision!(
            Lead::MovedAway("repo/docs/decisions"),
            Lead::Plant(Plant::File("docs/decisions"))
        ),
        // The committed decision is stranded by the re-point, which cannot move it: the
        // re-point refuses whole, naming the doc, the knob unchanged — as its `placement-root`
        // twin does, and as `r1-config-set-re-points-past-a-link-to-nothing-or-a-directory-
        // at-a-doc-it-strands` asks (the fifth of the fixes after the review of `B8`, its
        // left-open; relisted by the eighth).
        refuses(1, ONE_WAY),
        at("CS2", Kind::FileForADir),
    ),
    sets(
        "config-set-an-edit-to-the-decision",
        a_committed_decision!(Lead::Plant(Plant::Edit(ONE_WAY))),
        Listed::Acts,
        at("CS2", Kind::EditAfter),
    ),
    sets(
        "config-set-a-change-staged-at-the-decision",
        a_committed_decision!(Lead::Plant(Plant::Staged {
            at: ONE_WAY,
            kept: false
        })),
        Listed::Acts,
        at("CS2", Kind::StagedThere),
    ),
    // `CS2, at its new home`: whatever stands at the decision's new home refuses the whole
    // re-point, the knob unchanged, naming the path to clear — nothing is moved aside from a
    // destination (the human's ruling of 2026-10-20 on the record's §6, entry 23).
    sets(
        "config-set-a-file-of-the-users-at-the-decisions-new-home",
        a_committed_decision!(Lead::Plant(Plant::File(THE_DECISIONS_NEW_HOME))),
        refuses(1, THE_DECISIONS_NEW_HOME),
        at("CS2, at its new home", Kind::Untracked),
    ),
    sets(
        "config-set-a-file-the-user-committed-at-the-decisions-new-home",
        a_committed_decision!(Lead::Plant(Plant::Committed(THE_DECISIONS_NEW_HOME))),
        refuses(1, THE_DECISIONS_NEW_HOME),
        at("CS2, at its new home", Kind::Committed),
    ),
    sets(
        "config-set-a-link-to-nothing-at-the-decisions-new-home",
        a_committed_decision!(Lead::Plant(Plant::LinkToNothing(THE_DECISIONS_NEW_HOME))),
        refuses(1, THE_DECISIONS_NEW_HOME),
        at("CS2, at its new home", Kind::LinkToNothing),
    ),
    sets(
        "config-set-a-link-to-a-file-in-the-repository-at-the-decisions-new-home",
        a_committed_decision!(Lead::Plant(Plant::LinkToAFile {
            at: THE_DECISIONS_NEW_HOME,
            outside: false
        })),
        refuses(1, THE_DECISIONS_NEW_HOME),
        at("CS2, at its new home", Kind::LinkInside),
    ),
    sets(
        "config-set-a-link-to-a-file-outside-at-the-decisions-new-home",
        a_committed_decision!(Lead::Plant(Plant::LinkToAFile {
            at: THE_DECISIONS_NEW_HOME,
            outside: true
        })),
        refuses(1, THE_DECISIONS_NEW_HOME),
        at("CS2, at its new home", Kind::LinkOutside),
    ),
    sets(
        "config-set-a-directory-at-the-decisions-new-home",
        a_committed_decision!(Lead::Plant(Plant::Dir(THE_DECISIONS_NEW_HOME))),
        refuses(1, THE_DECISIONS_NEW_HOME),
        at("CS2, at its new home", Kind::DirForAFile),
    ),
    // Its twin under the same ruling's (A): a doc of the user's elsewhere in the decision's new
    // home blocks nothing — the re-point lands, the doc stays where it stood.
    sets(
        "config-set-a-doc-of-the-users-beside-the-decisions-new-home",
        a_committed_decision!(Lead::Plant(Plant::File(BESIDE_THE_DECISIONS_NEW_HOME))),
        Listed::Acts,
        at("CS2, at its new home", Kind::Untracked),
    ),
    // And the same doc committed (the sixth of the fixes after the review of `B9`, N10).
    sets(
        "config-set-a-doc-the-user-committed-beside-the-decisions-new-home",
        a_committed_decision!(Lead::Plant(Plant::Committed(BESIDE_THE_DECISIONS_NEW_HOME))),
        Listed::Acts,
        at("CS2, at its new home", Kind::Committed),
    ),
    // A decision jigc manages at the new home — landed by a task while the docs root was
    // `notes`, set by hand and set back — and one of the same name at the old: it refuses the
    // re-point too.
    sets(
        "config-set-a-decision-a-task-landed-at-the-decisions-new-home",
        &[
            file(THE_MANIFEST, "scalar:\n  docs-root: notes\n"),
            A_TASK_THAT_RECORDS_ONE_WAY[0],
            A_TASK_THAT_RECORDS_ONE_WAY[1],
            A_TASK_THAT_RECORDS_ONE_WAY[2],
            A_TASK_THAT_RECORDS_ONE_WAY[3],
            A_TASK_THAT_RECORDS_ONE_WAY[4],
            A_TASK_THAT_RECORDS_ONE_WAY[5],
            A_TASK_THAT_RECORDS_ONE_WAY[6],
            A_TASK_THAT_RECORDS_ONE_WAY[7],
            A_TASK_THAT_RECORDS_ONE_WAY[8],
            A_TASK_THAT_RECORDS_ONE_WAY[9],
            A_TASK_THAT_RECORDS_ONE_WAY[10],
            jigc(&["task", "finalize", "record-a-decision"]),
            file(THE_MANIFEST, "scalar:\n  docs-root: docs\n"),
            file(ONE_WAY, A_DECISION),
            git(&["add", "--", ".jigc/config", "docs"]),
            git(&["commit", "-q", "-m", "the docs root back, and a decision"]),
        ],
        refuses(1, "config.repoint-failed"),
        None,
    ),
    // `CS4`, the baseline.
    sets(
        "config-set-a-file-of-the-users-beside-the-baseline",
        a_committed_decision!(Lead::Plant(Plant::File(".jigc/state/notes.md"))),
        Listed::Acts,
        at("CS4", Kind::Untracked),
    ),
    // `placement-root`, over a committed roadmap.
    sets_the_placement_root(
        "config-set-placement-root-to-notes",
        a_committed_roadmap!(),
        Listed::Acts,
        None,
    ),
    // `CS2`, at the roadmap's home.
    sets_the_placement_root(
        "config-set-placement-root-a-file-of-the-users-in-the-roadmaps-place",
        a_committed_roadmap!(Lead::Plant(Plant::InPlaceOf(&Plant::File(THE_ROADMAP)))),
        Listed::Acts,
        at("CS2", Kind::Untracked),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-link-to-nothing-in-the-roadmaps-place",
        a_committed_roadmap!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            THE_ROADMAP
        )))),
        refuses(1, "store.home-not-regular-file"),
        at("CS2", Kind::LinkToNothing),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-link-to-a-file-in-the-repository-in-the-roadmaps-place",
        a_committed_roadmap!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: THE_ROADMAP,
            outside: false
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("CS2", Kind::LinkInside),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-link-to-a-file-outside-in-the-roadmaps-place",
        a_committed_roadmap!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: THE_ROADMAP,
            outside: true
        }))),
        refuses(1, "store.home-not-regular-file"),
        at("CS2", Kind::LinkOutside),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-directory-in-the-roadmaps-place",
        a_committed_roadmap!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(THE_ROADMAP)))),
        refuses(1, "store.home-not-regular-file"),
        at("CS2", Kind::DirForAFile),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-file-in-the-place-of-the-roadmaps-directory",
        a_committed_roadmap!(
            Lead::MovedAway("repo/docs"),
            Lead::Plant(Plant::File("docs"))
        ),
        refuses(1, THE_ROADMAP),
        at("CS2", Kind::FileForADir),
    ),
    sets_the_placement_root(
        "config-set-placement-root-an-edit-to-the-roadmap",
        a_committed_roadmap!(Lead::Plant(Plant::Edit(THE_ROADMAP))),
        Listed::Acts,
        at("CS2", Kind::EditAfter),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-change-staged-at-the-roadmap",
        a_committed_roadmap!(Lead::Plant(Plant::Staged {
            at: THE_ROADMAP,
            kept: false
        })),
        Listed::Acts,
        at("CS2", Kind::StagedThere),
    ),
    // `CS2, at its new home`, at the roadmap's: whatever stands there refuses the whole
    // re-point, the knob unchanged, naming the path to clear.
    sets_the_placement_root(
        "config-set-placement-root-a-file-of-the-users-at-the-roadmaps-new-home",
        a_committed_roadmap!(Lead::Plant(Plant::File(THE_ROADMAPS_NEW_HOME))),
        refuses(1, THE_ROADMAPS_NEW_HOME),
        at("CS2, at its new home", Kind::Untracked),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-file-the-user-committed-at-the-roadmaps-new-home",
        a_committed_roadmap!(Lead::Plant(Plant::Committed(THE_ROADMAPS_NEW_HOME))),
        refuses(1, THE_ROADMAPS_NEW_HOME),
        at("CS2, at its new home", Kind::Committed),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-link-to-nothing-at-the-roadmaps-new-home",
        a_committed_roadmap!(Lead::Plant(Plant::LinkToNothing(THE_ROADMAPS_NEW_HOME))),
        refuses(1, THE_ROADMAPS_NEW_HOME),
        at("CS2, at its new home", Kind::LinkToNothing),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-link-to-a-file-in-the-repository-at-the-roadmaps-new-home",
        a_committed_roadmap!(Lead::Plant(Plant::LinkToAFile {
            at: THE_ROADMAPS_NEW_HOME,
            outside: false
        })),
        refuses(1, THE_ROADMAPS_NEW_HOME),
        at("CS2, at its new home", Kind::LinkInside),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-link-to-a-file-outside-at-the-roadmaps-new-home",
        a_committed_roadmap!(Lead::Plant(Plant::LinkToAFile {
            at: THE_ROADMAPS_NEW_HOME,
            outside: true
        })),
        refuses(1, THE_ROADMAPS_NEW_HOME),
        at("CS2, at its new home", Kind::LinkOutside),
    ),
    sets_the_placement_root(
        "config-set-placement-root-a-directory-at-the-roadmaps-new-home",
        a_committed_roadmap!(Lead::Plant(Plant::Dir(THE_ROADMAPS_NEW_HOME))),
        refuses(1, THE_ROADMAPS_NEW_HOME),
        at("CS2, at its new home", Kind::DirForAFile),
    ),
    // Its twin under the same ruling's (A): a doc of the user's elsewhere in the new root
    // blocks nothing.
    sets_the_placement_root(
        "config-set-placement-root-a-doc-of-the-users-beside-the-roadmaps-new-home",
        a_committed_roadmap!(Lead::Plant(Plant::File(BESIDE_THE_ROADMAPS_NEW_HOME))),
        Listed::Acts,
        at("CS2, at its new home", Kind::Untracked),
    ),
    // And the same doc committed (the sixth of the fixes after the review of `B9`, N10).
    sets_the_placement_root(
        "config-set-placement-root-a-doc-the-user-committed-beside-the-roadmaps-new-home",
        a_committed_roadmap!(Lead::Plant(Plant::Committed(BESIDE_THE_ROADMAPS_NEW_HOME))),
        Listed::Acts,
        at("CS2, at its new home", Kind::Committed),
    ),
    // A roadmap jigc manages at the new home — moved there by a re-point the user
    // committed, the knob then taken out of the manifest by hand — and one committed at the
    // old: it refuses the re-point too.
    sets_the_placement_root(
        "config-set-placement-root-a-roadmap-a-re-point-moved-to-the-roadmaps-new-home",
        a_committed_roadmap!(
            jigc(SET_PLACEMENT_ROOT),
            git(&["add", "--", ".jigc/config"]),
            git(&["commit", "-q", "-m", "the roadmap re-pointed"]),
            git(&["rm", "-q", "--", THE_MANIFEST]),
            file(THE_ROADMAP, A_ROADMAP),
            git(&["add", "--", "docs"]),
            git(&[
                "commit",
                "-q",
                "-m",
                "the placement root back, and a roadmap"
            ]),
        ),
        refuses(1, "write.already-present"),
        None,
    ),
    // The base cell in each other prepared state: the command as typed, with no lead.
    cell(
        "config-set-docs-root-to-notes-in-bare",
        &["config", "set"],
        Prepared::Bare,
        &[],
        SET_DOCS_ROOT,
        NOT_SET_UP,
        None,
    ),
    cell(
        "config-set-docs-root-to-notes-in-committed-singletons",
        &["config", "set"],
        Prepared::Of(State::CommittedSingletons),
        &[],
        SET_DOCS_ROOT,
        Listed::Acts,
        None,
    ),
    cell(
        "config-set-docs-root-to-notes-in-migrated",
        &["config", "set"],
        Prepared::Of(State::Migrated),
        &[],
        SET_DOCS_ROOT,
        Listed::Acts,
        None,
    ),
    cell(
        "config-set-docs-root-to-notes-in-refs-post-hoc",
        &["config", "set"],
        Prepared::Of(State::RefsPostHoc),
        &[],
        SET_DOCS_ROOT,
        Listed::Acts,
        None,
    ),
    cell(
        "config-set-docs-root-to-notes-in-chatty-hooks",
        &["config", "set"],
        Prepared::Of(State::ChattyHooks),
        &[],
        SET_DOCS_ROOT,
        Listed::Acts,
        None,
    ),
    cell(
        "config-set-docs-root-to-notes-in-vendored",
        &["config", "set"],
        Prepared::Of(State::Vendored),
        &[],
        SET_DOCS_ROOT,
        Listed::Acts,
        None,
    ),
];
