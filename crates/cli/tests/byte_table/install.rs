//! **The byte table's rows for the install family** — `jigc setup`, `jigc uninstall`,
//! `jigc ingest` and `jigc unmanage`
//! (`crates/cli/tests/byte_table.rs`). A row is what its command may change, line by
//! line, **lifted quote for quote from the committed record**
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`,
//! §1, §2, §2b and §2c): a line's `id` is its number there, and the quote fence holds every quote
//! to its design doc and to the record. A line is never changed here without the record
//! changing in the same commit, and never from what a command did.
//!
//! **What of the record is no line.** An entry that is a rule or a condition is what the
//! lines take a field from, and stands in the record alone: `S13` and `U7` are where
//! *on a refusal: no* comes from; `S14` and `U8` are the rows' consent; `S15` is the
//! condition the acceptance runs on both sides, which the cells construct. `S12` is no
//! line while the record's §6, entry 2, is open: a cell that turns on it follows the
//! reading under which a refusal of `setup` leaves nothing. And three conditions a doc
//! gives stay words, because no manifest shows them — *where its install commit can carry
//! the settings file* (`S6`), *what stands there is jigc's own copy* (`S8`, `U5`), *git
//! tracks the settings file and the record* (`U3`): each lets the command do less, and the
//! file it would spare is a plant, which no line excuses.

use super::task::A_DECISION;
use super::{AFTER_A_COMMIT, ON_A_REFUSAL, refuses};
use crate::support::byte_cell::{
    Aim, Allows, Cell, Condition, Holding, Kind, Lead, Line, Listed, May, Plant, Prepared, Quote,
    Row, Unit, When, Where,
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

/// The sentence `S16` rests on: the human's ruling of 2026-10-15 on the record's §6, entry
/// 1, as `DECISIONS.md` records it.
const LAYS_OUT: &[Quote] = &[Quote {
    doc: "DECISIONS.md",
    heading: "2026-10-18 — The human's seven rulings on the install family's questions, recorded: `setup` refuses over a path whose bytes are not jigc's own; it lays out the workbench's cache directories; it refuses a link at the root `.gitignore` on any `HEAD`; `unmanage` never commits; the install commit runs the user's hooks; the install-footprint record is no cache; and `B6` builds the plant in the place of a file that stands (the byte table, task `B6`, the first of its commits)",
    words: "**`jigc setup` also lays out the workbench's cache directories**",
}];

/// **`jigc setup`** — the record's §1. `S16` is a line whose sentence is the human's
/// ruling of 2026-10-15 on the record's §6, entry 1: `setup` lays out the workbench's two
/// cache directories, and writes no file into them.
pub const SETUP: Row = Row {
    leaf: &["setup"],
    unit: Unit::Nothing,
    spellings: &[],
    consent: Some("--force"),
    home: Prepared::Bare,
    lines: &[
        line(
            "S1",
            Allows::Files(Where::Path(".jigc/AGENT.md"), May::WriteWhole),
            &[Quote {
                doc: "design/assistant-adapter.md",
                heading: "1. Inject the bootstrap — hook (primary) + line (floor)",
                words: "a stable one-line pointer (Claude Code: an `@.jigc/AGENT.md` import) to a **managed bootstrap file** the adapter writes under `.jigc/` and **regenerates on every `setup`**",
            }],
        ),
        line(
            "S2",
            Allows::Files(Where::Path("CLAUDE.md"), May::MergeInto),
            &[Quote {
                doc: "design/assistant-adapter.md",
                heading: "1. Inject the bootstrap — hook (primary) + line (floor)",
                words: "The always-loaded file is touched only to ensure the pointer is present (idempotent on the exact reference line); the content lives in the managed file.",
            }],
        ),
        line(
            "S3",
            Allows::Files(Where::Path(".claude/settings.json"), May::MergeInto),
            &[Quote {
                doc: "design/project-setup.md",
                heading: "Idempotency & irreversibility — the acceptance obligation",
                words: "a pre-existing non-trivial `CLAUDE.md` / `.claude/settings.json` is **merged, never clobbered** (structure-aware)",
            }],
        ),
        line(
            "S4",
            Allows::Files(Where::Under(".jigc/config"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "Generated, minimal, regenerated",
                    words: "`jigc setup` also **initializes the in-repo project layer** (`.jigc/config/`) when absent",
                },
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Idempotency & irreversibility — the acceptance obligation",
                    words: "a file at a path it **replaces** (`.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`, the settings record `.jigc/settings-entries.json`, a `.jigc/version` that is not jigc's own one-line stamp) is refused before the first write while no commit holds its bytes, on an unborn `HEAD` as on a born one",
                },
            ],
        ),
        // The sentences are `S11`'s second, which names the stamp a member of the
        // install, and `S4`'s second, which says it is replaced and not merged.
        line(
            "S5",
            Allows::Files(Where::Path(".jigc/version"), May::WriteWhole),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "`jigc setup` has assistant-*neutral* install responsibilities too",
                    words: "The pathspec is **enumerated, never a blanket `git add -A`**: the always-loaded host file (`CLAUDE.md`), the settings file (`.claude/settings.json`), the managed `.jigc/AGENT.md`, `.jigc/.gitignore`, the `.jigc/version` provenance stamp, the project layer (`.jigc/config/`)",
                },
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Idempotency & irreversibility — the acceptance obligation",
                    words: "a file at a path it **replaces** (`.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`, the settings record `.jigc/settings-entries.json`, a `.jigc/version` that is not jigc's own one-line stamp) is refused before the first write while no commit holds its bytes, on an unborn `HEAD` as on a born one",
                },
            ],
        ),
        line(
            "S6, written",
            Allows::Files(Where::Path(".jigc/settings-entries.json"), May::WriteWhole),
            &[Quote {
                doc: "design/assistant-adapter.md",
                heading: "2. Make `jigc` frictionless",
                words: "`setup` writes it only where its install commit can carry the settings file, and removes a record that stands where it cannot",
            }],
        ),
        line(
            "S6, removed",
            Allows::Files(Where::Path(".jigc/settings-entries.json"), May::Removed),
            &[Quote {
                doc: "design/assistant-adapter.md",
                heading: "2. Make `jigc` frictionless",
                words: "`setup` writes it only where its install commit can carry the settings file, and removes a record that stands where it cannot",
            }],
        ),
        line(
            "S7",
            Allows::Files(Where::Path(".jigc/.gitignore"), May::MergeInto),
            &[Quote {
                doc: "design/assistant-adapter.md",
                heading: "`jigc setup` has assistant-*neutral* install responsibilities too",
                words: "`.jigc/.gitignore` is the one merged-into member a link is never followed at — its writer amends in place",
            }],
        ),
        line(
            "S8",
            Allows::Files(Where::Path(".claude/skills/jigc/SKILL.md"), May::WriteWhole),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "The adapter's owned artifacts (M48)",
                    words: "**rewritten whole on every `setup`**, the `.jigc/AGENT.md` mold",
                },
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "The adapter's owned artifacts (M48)",
                    words: "`.claude/skills/jigc/SKILL.md` is Claude Code's skill convention, not jigc's",
                },
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "The adapter's owned artifacts (M48)",
                    words: "and is left **byte-identical**, by `setup` and by `uninstall` alike",
                },
            ],
        ),
        Line {
            when: When::NoCommitYet,
            ..line(
                "S9",
                Allows::Files(Where::Path(".gitignore"), May::MergeInto),
                &[
                    Quote {
                        doc: "design/project-setup.md",
                        heading: "The secrets-floor `.gitignore` — fresh-repo install only (M36)",
                        words: "Setup seeds the floor **iff the repo has zero commits**",
                    },
                    Quote {
                        doc: "design/project-setup.md",
                        heading: "The secrets-floor `.gitignore` — fresh-repo install only (M36)",
                        words: "Absent → create; present → append-if-absent; re-run → byte-stable no-op.",
                    },
                ],
            )
        },
        line(
            "S10",
            Allows::Files(Where::Hook, May::MergeInto),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "`jigc setup` has assistant-*neutral* install responsibilities too",
                    words: "`jigc setup` installs a `pre-commit` hook (into the hooks dir git resolves — usually `.git/hooks/`, see the install discipline below)",
                },
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "`jigc setup` has assistant-*neutral* install responsibilities too",
                    words: "**non-destructive** (preserves/wraps any existing `pre-commit` hook)",
                },
            ],
        ),
        line(
            "S11",
            Allows::Commit(Holding::OnlyTheLines),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "`jigc setup` has assistant-*neutral* install responsibilities too",
                    words: "`jigc setup` lands its footprint as a dedicated `chore(jigc): install jigc workspace config` commit so the scaffolding never rides the user's first `finalize`",
                },
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "`jigc setup` has assistant-*neutral* install responsibilities too",
                    words: "The pathspec is **enumerated, never a blanket `git add -A`**: the always-loaded host file (`CLAUDE.md`), the settings file (`.claude/settings.json`), the managed `.jigc/AGENT.md`, `.jigc/.gitignore`, the `.jigc/version` provenance stamp, the project layer (`.jigc/config/`)",
                },
            ],
        ),
        // The human's ruling of 2026-10-15 on the record's §6, entry 1: the row's two lines
        // whose sentence is a ruling and not a design doc's, until the doc says it.
        line(
            "S16, the file-state hashes",
            Allows::Files(Where::Under(".jigc/state"), May::LaidOut),
            LAYS_OUT,
        ),
        line(
            "S16, the edge index",
            Allows::Files(Where::Under(".jigc/index"), May::LaidOut),
            LAYS_OUT,
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
    ],
};

/// **`jigc uninstall`** — the record's §2. Its doc is silent on git's index and on a
/// commit (the record's §6, entry 5), and the human ruled on 2026-10-14: the teardown
/// lands as one path-scoped commit of what it removed and never carries the user's own
/// uncommitted edits — `U9`, whose sentence is the ruling's. A removal of a tracked file
/// the working tree keeps with no commit carrying it is outside the allowance.
pub const UNINSTALL: Row = Row {
    leaf: &["uninstall"],
    unit: Unit::Nothing,
    spellings: &[],
    consent: Some("--force"),
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "U1",
            Allows::Files(Where::Under(".jigc"), May::Removed),
            &[Quote {
                doc: "design/project-setup.md",
                heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                words: "**`uninstall` removes exactly the enumerated repo-local setup-created set — now *complete*: `.jigc/`, the import line, all three `.claude/settings.json` writes (permit + `SessionStart` hook + `deny` floor), the `pre-commit` hook, and the adapter's owned guide artifact**",
            }],
        ),
        line(
            "U2",
            Allows::Files(Where::Path("CLAUDE.md"), May::TakeOutOf),
            &[Quote {
                doc: "design/project-setup.md",
                heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                words: "unwire the `CLAUDE.md` import line",
            }],
        ),
        line(
            "U3",
            Allows::Files(Where::Path(".claude/settings.json"), May::TakeOutOf),
            &[Quote {
                doc: "design/project-setup.md",
                heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                words: "`setup` now records the entries it adds in one committed file, `.jigc/settings-entries.json`, and the teardown removes exactly those, reading the record **only where git tracks the settings file**",
            }],
        ),
        line(
            "U4",
            Allows::Files(Where::Hook, May::TakeOutOf),
            &[Quote {
                doc: "design/project-setup.md",
                heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                words: "**`uninstall` removes exactly the lines between jigc's two markers — whatever version wrote them, whatever stands around them**: what is left is restored byte for byte, and the file is removed only where nothing is left but the frame jigc itself puts around its block",
            }],
        ),
        line(
            "U4, removed",
            Allows::Files(Where::Hook, May::Removed),
            &[Quote {
                doc: "design/project-setup.md",
                heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                words: "**`uninstall` removes exactly the lines between jigc's two markers — whatever version wrote them, whatever stands around them**: what is left is restored byte for byte, and the file is removed only where nothing is left but the frame jigc itself puts around its block",
            }],
        ),
        line(
            "U5",
            Allows::Files(Where::Path(".claude/skills/jigc/SKILL.md"), May::Removed),
            &[
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                    words: "jigc's own copy is removed, and a copy the user has edited is **left in place and reported**",
                },
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                    words: "The **directories jigc created to hold it** go with it once they empty, stopping at any directory that houses another host file",
                },
            ],
        ),
        line(
            "U6",
            Allows::Unregistered(".jigc/worktrees"),
            &[Quote {
                doc: "design/team-ready-state.md",
                heading: "`jigc milestone discard <id>` — the record-settling op",
                words: "A door removes a registration from `.git/worktrees/` **only at a path it created** under `<jigc_home>/.jigc/worktrees/` — a sub-task's `<sub-task-id>`, or a boundary's own `.combine-*` — **by path**, one record at a time (`git worktree remove --force <path>`), and **a run that refuses removes none**",
            }],
        ),
        // The human's ruling of 2026-10-14 on the record's §6, entry 5: the row's one line
        // whose sentence is a ruling and not a design doc's, until the doc says it.
        line(
            "U9",
            Allows::Commit(Holding::AllTheLinesChanged),
            &[Quote {
                doc: "DECISIONS.md",
                heading: "2026-10-15 — The human's six rulings on the byte table's first cut, recorded: the families `B5` to `B9` are built now; what is held reaches what a reflog names; a change staged at a path jigc stages itself is a finding beyond the one declared bound; every refusal of `setup` leaves the repository as it was; `uninstall` lands its teardown in one path-scoped commit; a rejected record commit's message file is a finding (the byte table, task `B5`, the first of its commits)",
                words: "**The teardown lands as one dedicated, path-scoped commit of what it removed**, the inverse of `setup`'s install commit, and **never carries the user's own uncommitted edits** at a path it touches",
            }],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
    ],
};

/// The sentence `ingest`'s two lines rest on.
const ADOPTS: &[Quote] = &[Quote {
    doc: "design/project-setup.md",
    heading: "The bounded slice — three net-new but tractable pieces",
    words: "Adopting a foreign `adoptable` doc is a distinct action: **parse → conformance-gate → `index.absorb_doc` (so an adopted `adr`'s `supersedes` edge is indexed) → record the file-state hash**.",
}];

/// **`jigc ingest`** — the record's §2b. Adoption is a registration and nothing else: the
/// doc's hash into the file-state record, its edges into the edge index. **What of the
/// record is no line**: `I3` — adopt never moves or rewrites a file — and `I4` — it
/// adopts nothing it has not schema-checked, and leaves the rest untouched — are why the
/// row has no line for any file outside the two caches. Its doc names no consent.
pub const INGEST: Row = Row {
    leaf: &["ingest"],
    unit: Unit::Nothing,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "I1",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            ADOPTS,
        ),
        line(
            "I2",
            Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
            ADOPTS,
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
    ],
};

/// The sentences `unmanage`'s two lines rest on.
const DETACHES: &[Quote] = &[
    Quote {
        doc: "design/project-setup.md",
        heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
        words: "**(a) un-manage a doc** — drop a single managed doc from the file-state record + its `edges.json` entries, leaving the file on disk (the inverse of `adopt`'s register-only mutation)",
    },
    Quote {
        doc: "design/project-setup.md",
        heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
        words: "un-manage touches only jigc's own index/state (asserted: the doc's bytes on disk are unchanged) and is idempotent (re-run on an already-unmanaged doc is a clean no-op, not an error)",
    },
];

/// **`jigc unmanage`** — the record's §2c, letter `X` for the *detach report* its output
/// is called. It touches the two caches and nothing else; whether it commits, two docs say
/// two ways (the record's §6, entry 9), and until that is ruled the row has no commit
/// line. Its doc names no consent.
pub const UNMANAGE: Row = Row {
    leaf: &["unmanage"],
    unit: Unit::Nothing,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::CommittedSingletons),
    lines: &[
        line(
            "X1",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            DETACHES,
        ),
        line(
            "X2",
            Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
            DETACHES,
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
    ],
};

/// The family's rows.
pub const ROWS: &[Row] = &[SETUP, UNINSTALL, INGEST, UNMANAGE];

// ---------------------------------------------------------------------------
// The cells
// ---------------------------------------------------------------------------

/// A file somebody left where the install writes its bootstrap whole.
const AT_THE_BOOTSTRAP: Lead = Lead::Plant(Plant::File(".jigc/AGENT.md"));

/// The suite that holds `setup`'s refusal over a file at a path it replaces, on a born
/// and on an unborn `HEAD`, ordering by ordering.
const THE_INSTALL_GUARD: &str = "setup_install_pathspec_guard::a_whole_rewrite_install_path_refuses_with_the_users_bytes_intact";

/// The suite that holds `uninstall`'s refusal over a file jigc did not write, at every
/// transient prefix of the workbench and at two depths.
const THE_TEARDOWN_GUARD: &str =
    "uninstall_workbench_subject::a_file_inside_every_transient_prefix_blocks_the_teardown";

/// One cell of `uninstall` in `fresh`: a file the product did not write, in a directory
/// the teardown removes (the plan's plant **D**).
const fn uninstall_over(
    name: &'static str,
    leads: &'static [Lead],
    says: &'static str,
    beside: &'static str,
) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf: &["uninstall"],
        state: Prepared::Of(State::Fresh),
        leads,
        argv: &["uninstall"],
        listed: refuses(1, says),
        beside: Some(beside),
        aim: Some(Aim {
            line: "U1",
            kind: Kind::Foreign,
        }),
        marks: &[],
        at: None,
    }
}

/// **The acceptance cells of the install family** (the plan's §5): the blocking rows
/// `(R1, F1)` and `(R9, F5)` of the review of `1.0.0-rc.24`, each as the one plant its
/// record names, at the place and under the condition its record names — and beside each
/// the other side: `setup` with `HEAD` born, and `uninstall` over the same file one
/// directory up, which that release refused.
///
/// **Every cell here is listed as one that refuses, and names its refusal**, because the
/// row's doc says so: a
/// file at a path the install replaces *"is refused before the first write while no
/// commit holds its bytes, on an unborn `HEAD` as on a born one"* (`S4`), and a file jigc
/// did not write under a directory of `.jigc/` is one of the four states that *"remove
/// nothing"* (`U7`).
const ACCEPTANCE: &[Cell] = &[
    Cell {
        spelling: None,
        name: "setup-head-unborn-a-file-at-the-bootstrap",
        leaf: &["setup"],
        state: Prepared::Bare,
        leads: &[Lead::Under(Condition::HeadUnborn), AT_THE_BOOTSTRAP],
        argv: &["setup"],
        listed: refuses(1, "setup.dirty-install-path"),
        beside: Some(THE_INSTALL_GUARD),
        aim: Some(Aim {
            line: "S1",
            kind: Kind::Untracked,
        }),
        marks: &[],
        at: None,
    },
    Cell {
        spelling: None,
        name: "setup-head-born-a-file-at-the-bootstrap",
        leaf: &["setup"],
        state: Prepared::Bare,
        leads: &[AT_THE_BOOTSTRAP],
        argv: &["setup"],
        listed: refuses(1, "setup.dirty-install-path"),
        beside: Some(THE_INSTALL_GUARD),
        aim: Some(Aim {
            line: "S1",
            kind: Kind::Untracked,
        }),
        marks: &[],
        at: None,
    },
    uninstall_over(
        "uninstall-a-foreign-file-under-logs",
        &[Lead::Plant(Plant::File(".jigc/logs/notes.md"))],
        "uninstall.foreign-bytes",
        THE_TEARDOWN_GUARD,
    ),
    uninstall_over(
        "uninstall-a-foreign-file-under-state",
        &[Lead::Plant(Plant::File(".jigc/state/notes.md"))],
        "uninstall.foreign-bytes",
        THE_TEARDOWN_GUARD,
    ),
    uninstall_over(
        "uninstall-a-foreign-file-under-index",
        &[Lead::Plant(Plant::File(".jigc/index/notes.md"))],
        "uninstall.foreign-bytes",
        THE_TEARDOWN_GUARD,
    ),
    uninstall_over(
        "uninstall-a-foreign-file-at-the-workbench-root",
        &[Lead::Plant(Plant::File(".jigc/notes.md"))],
        "uninstall.untracked-workbench-file",
        "uninstall_workbench_subject::a_hand_dropped_workbench_file_blocks_and_the_install_survives",
    ),
];

// ---------------------------------------------------------------------------
// The family's own cells — the plan's task `B5`
// ---------------------------------------------------------------------------

const BARE: Prepared = Prepared::Bare;
const FRESH: Prepared = Prepared::Of(State::Fresh);
const SINGLETONS: Prepared = Prepared::Of(State::CommittedSingletons);

/// What a planted cell's plant is aimed at.
const fn at(line: &'static str, kind: Kind) -> Option<Aim> {
    Some(Aim { line, kind })
}

/// One cell of `jigc setup`, run as typed, in the never-adopted repository.
const fn setup(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf: &["setup"],
        state: BARE,
        leads,
        argv: &["setup"],
        listed,
        beside: None,
        aim,
        marks: &[],
        at: None,
    }
}

/// One cell of `jigc uninstall`, run as typed, in `fresh`.
const fn uninstall(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf: &["uninstall"],
        state: FRESH,
        leads,
        argv: &["uninstall"],
        listed,
        beside: None,
        aim,
        marks: &[],
        at: None,
    }
}

/// A refusal of `setup` over what stands at a path of its install, uncommitted, on a born
/// `HEAD`: *"its install pathspec is asked against `HEAD` before the install writes a
/// byte"*, and a file at a path it replaces *"is refused before the first write while no
/// commit holds its bytes"* (`S4`).
const DIRTY: Listed = refuses(1, "setup.dirty-install-path");

/// The hooks directory moved into the working tree — `core.hooksPath` — so that what a
/// cell puts in the hook's place stands where the reader reads files: a hooks directory
/// git resolves is the one `S10` names, wherever it is.
const HOOKS_IN_THE_TREE: Lead = Lead::Step(Step::Git(&["config", "core.hooksPath", "the-hooks"]));
const THE_HOOK_IN_THE_TREE: &str = "the-hooks/pre-commit";

/// A file whose name git quotes: a quote, and a byte outside ASCII (the plan's plant **N**).
const A_NAME_GIT_QUOTES: &str = "notes/a \"quoted\" n\u{e4}me.txt";

/// A `HEAD` unborn: the root `.gitignore`'s line holds only there (`S9`).
const UNBORN: Lead = Lead::Under(Condition::HeadUnborn);

/// **The cells of `setup` at the paths it replaces and merges into** — a kind of plant
/// each, a line each. Each is listed from the row's doc, the refusal's words read off a
/// run where no doc gives them:
///
/// - **at a path it replaces** (`S1`, `S5`, `S6, written`): a link or a directory is no file
///   it can replace, and is refused before the first write; **a file the user committed**
///   there is refused as dirty too, by the human's ruling of 2026-10-15 on the record's §6,
///   entry 10 — *"while its bytes are not jigc's own"*; a change **staged** there and not in
///   the tree is refused as dirty, but at the stamp, where no sentence says the state
///   refuses;
/// - **the guide** (`S8`) is left as it stands whatever stands there — *"a copy the user
///   has edited is left"*, by `setup` as by `uninstall`;
/// - **at a path it merges into, on a born `HEAD`** (`S2`, `S3`, `S7`): a link that leads
///   nowhere or out of the checkout is refused, and one that leads into it is followed to
///   a file that is dirty; the user's own file in no commit, and a change staged, are
///   refused as dirty; a directory is refused;
/// - **on an unborn `HEAD`, at the root `.gitignore`** (`S9`): the user's own file rides
///   the first commit (`S15`); a link there is refused wherever it leads, by the human's
///   ruling of 2026-10-15 on the record's §6, entry 12, and so is a directory.
///
/// A `setup` that exits 0 lays out `.jigc/state/`, empty: `S16`, the human's ruling of
/// 2026-10-15 on the record's §6, entry 1.
const SETUP_PLANTED: [Cell; 48] = [
    setup(
        "setup-at-the-bootstrap-a-committed-file",
        &[Lead::Plant(Plant::Committed(".jigc/AGENT.md"))],
        DIRTY,
        at("S1", Kind::Committed),
    ),
    setup(
        "setup-at-the-bootstrap-a-link-to-nothing",
        &[Lead::Plant(Plant::LinkToNothing(".jigc/AGENT.md"))],
        refuses(1, "setup.write-bootstrap"),
        at("S1", Kind::LinkToNothing),
    ),
    setup(
        "setup-at-the-bootstrap-a-link-to-a-file-in-the-repository",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".jigc/AGENT.md",
            outside: false,
        })],
        refuses(1, "setup.write-bootstrap"),
        at("S1", Kind::LinkInside),
    ),
    setup(
        "setup-at-the-bootstrap-a-link-to-a-file-outside-it",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".jigc/AGENT.md",
            outside: true,
        })],
        refuses(1, "setup.write-bootstrap"),
        at("S1", Kind::LinkOutside),
    ),
    setup(
        "setup-at-the-bootstrap-a-directory",
        &[Lead::Plant(Plant::Dir(".jigc/AGENT.md"))],
        refuses(1, "setup.write-bootstrap"),
        at("S1", Kind::DirForAFile),
    ),
    setup(
        "setup-at-the-bootstrap-a-change-staged-and-not-in-the-tree",
        &[Lead::Plant(Plant::Staged {
            at: ".jigc/AGENT.md",
            kept: false,
        })],
        DIRTY,
        at("S1", Kind::StagedThere),
    ),
    setup(
        "setup-at-the-stamp-a-committed-file",
        &[Lead::Plant(Plant::Committed(".jigc/version"))],
        DIRTY,
        at("S5", Kind::Committed),
    ),
    setup(
        "setup-at-the-stamp-a-link-to-nothing",
        &[Lead::Plant(Plant::LinkToNothing(".jigc/version"))],
        refuses(1, "setup.version-stamp"),
        at("S5", Kind::LinkToNothing),
    ),
    setup(
        "setup-at-the-stamp-a-link-to-a-file-in-the-repository",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".jigc/version",
            outside: false,
        })],
        refuses(1, "setup.version-stamp"),
        at("S5", Kind::LinkInside),
    ),
    setup(
        "setup-at-the-stamp-a-link-to-a-file-outside-it",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".jigc/version",
            outside: true,
        })],
        refuses(1, "setup.version-stamp"),
        at("S5", Kind::LinkOutside),
    ),
    setup(
        "setup-at-the-stamp-a-directory",
        &[Lead::Plant(Plant::Dir(".jigc/version"))],
        refuses(1, "setup.version-stamp"),
        at("S5", Kind::DirForAFile),
    ),
    setup(
        "setup-at-the-stamp-a-change-staged-and-not-in-the-tree",
        &[Lead::Plant(Plant::Staged {
            at: ".jigc/version",
            kept: false,
        })],
        Listed::Acts,
        at("S5", Kind::StagedThere),
    ),
    setup(
        "setup-at-the-settings-record-a-committed-file",
        &[Lead::Plant(Plant::Committed(".jigc/settings-entries.json"))],
        DIRTY,
        at("S6, written", Kind::Committed),
    ),
    setup(
        "setup-at-the-settings-record-a-link-to-nothing",
        &[Lead::Plant(Plant::LinkToNothing(
            ".jigc/settings-entries.json",
        ))],
        refuses(1, "setup.inject-allowlist"),
        at("S6, written", Kind::LinkToNothing),
    ),
    setup(
        "setup-at-the-settings-record-a-link-to-a-file-in-the-repository",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".jigc/settings-entries.json",
            outside: false,
        })],
        refuses(1, "setup.inject-allowlist"),
        at("S6, written", Kind::LinkInside),
    ),
    setup(
        "setup-at-the-settings-record-a-link-to-a-file-outside-it",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".jigc/settings-entries.json",
            outside: true,
        })],
        refuses(1, "setup.inject-allowlist"),
        at("S6, written", Kind::LinkOutside),
    ),
    setup(
        "setup-at-the-settings-record-a-directory",
        &[Lead::Plant(Plant::Dir(".jigc/settings-entries.json"))],
        refuses(1, "setup.inject-allowlist"),
        at("S6, written", Kind::DirForAFile),
    ),
    setup(
        "setup-at-the-settings-record-a-change-staged-and-not-in-the-tree",
        &[Lead::Plant(Plant::Staged {
            at: ".jigc/settings-entries.json",
            kept: false,
        })],
        DIRTY,
        at("S6, written", Kind::StagedThere),
    ),
    setup(
        "setup-at-the-guide-a-committed-file",
        &[Lead::Plant(Plant::Committed(
            ".claude/skills/jigc/SKILL.md",
        ))],
        Listed::Acts,
        at("S8", Kind::Committed),
    ),
    setup(
        "setup-at-the-guide-a-link-to-nothing",
        &[Lead::Plant(Plant::LinkToNothing(
            ".claude/skills/jigc/SKILL.md",
        ))],
        Listed::Acts,
        at("S8", Kind::LinkToNothing),
    ),
    setup(
        "setup-at-the-guide-a-link-to-a-file-in-the-repository",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".claude/skills/jigc/SKILL.md",
            outside: false,
        })],
        Listed::Acts,
        at("S8", Kind::LinkInside),
    ),
    setup(
        "setup-at-the-guide-a-link-to-a-file-outside-it",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".claude/skills/jigc/SKILL.md",
            outside: true,
        })],
        Listed::Acts,
        at("S8", Kind::LinkOutside),
    ),
    setup(
        "setup-at-the-guide-a-directory",
        &[Lead::Plant(Plant::Dir(".claude/skills/jigc/SKILL.md"))],
        Listed::Acts,
        at("S8", Kind::DirForAFile),
    ),
    setup(
        "setup-at-the-guide-a-change-staged-and-not-in-the-tree",
        &[Lead::Plant(Plant::Staged {
            at: ".claude/skills/jigc/SKILL.md",
            kept: false,
        })],
        Listed::Acts,
        at("S8", Kind::StagedThere),
    ),
    setup(
        "setup-at-the-host-file-a-link-to-nothing",
        &[Lead::Plant(Plant::LinkToNothing("CLAUDE.md"))],
        refuses(1, "setup.inject-reference"),
        at("S2", Kind::LinkToNothing),
    ),
    setup(
        "setup-at-the-host-file-a-link-to-a-file-in-the-repository",
        &[Lead::Plant(Plant::LinkToAFile {
            at: "CLAUDE.md",
            outside: false,
        })],
        DIRTY,
        at("S2", Kind::LinkInside),
    ),
    setup(
        "setup-at-the-host-file-a-link-to-a-file-outside-it",
        &[Lead::Plant(Plant::LinkToAFile {
            at: "CLAUDE.md",
            outside: true,
        })],
        refuses(1, "setup.inject-reference"),
        at("S2", Kind::LinkOutside),
    ),
    setup(
        "setup-at-the-host-file-a-directory",
        &[Lead::Plant(Plant::Dir("CLAUDE.md"))],
        refuses(1, "setup.inject-reference"),
        at("S2", Kind::DirForAFile),
    ),
    setup(
        "setup-at-the-host-file-the-users-own-file",
        &[Lead::Plant(Plant::UsersLine("CLAUDE.md"))],
        DIRTY,
        at("S2", Kind::UsersLine),
    ),
    setup(
        "setup-at-the-host-file-a-change-staged-and-not-in-the-tree",
        &[Lead::Plant(Plant::Staged {
            at: "CLAUDE.md",
            kept: false,
        })],
        DIRTY,
        at("S2", Kind::StagedThere),
    ),
    setup(
        "setup-at-the-settings-a-link-to-nothing",
        &[Lead::Plant(Plant::LinkToNothing(".claude/settings.json"))],
        refuses(1, "setup.inject-allowlist"),
        at("S3", Kind::LinkToNothing),
    ),
    setup(
        "setup-at-the-settings-a-link-to-a-file-in-the-repository",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".claude/settings.json",
            outside: false,
        })],
        DIRTY,
        at("S3", Kind::LinkInside),
    ),
    setup(
        "setup-at-the-settings-a-link-to-a-file-outside-it",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".claude/settings.json",
            outside: true,
        })],
        refuses(1, "setup.inject-allowlist"),
        at("S3", Kind::LinkOutside),
    ),
    setup(
        "setup-at-the-settings-a-directory",
        &[Lead::Plant(Plant::Dir(".claude/settings.json"))],
        refuses(1, "setup.inject-allowlist"),
        at("S3", Kind::DirForAFile),
    ),
    setup(
        "setup-at-the-settings-the-users-own-file",
        &[Lead::Plant(Plant::UsersLine(".claude/settings.json"))],
        DIRTY,
        at("S3", Kind::UsersLine),
    ),
    setup(
        "setup-at-the-settings-a-change-staged-and-not-in-the-tree",
        &[Lead::Plant(Plant::Staged {
            at: ".claude/settings.json",
            kept: false,
        })],
        DIRTY,
        at("S3", Kind::StagedThere),
    ),
    setup(
        "setup-at-the-workbench-ignore-a-link-to-nothing",
        &[Lead::Plant(Plant::LinkToNothing(".jigc/.gitignore"))],
        refuses(1, "setup.init-project-layer"),
        at("S7", Kind::LinkToNothing),
    ),
    setup(
        "setup-at-the-workbench-ignore-a-link-to-a-file-in-the-repository",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".jigc/.gitignore",
            outside: false,
        })],
        refuses(1, "setup.init-project-layer"),
        at("S7", Kind::LinkInside),
    ),
    setup(
        "setup-at-the-workbench-ignore-a-link-to-a-file-outside-it",
        &[Lead::Plant(Plant::LinkToAFile {
            at: ".jigc/.gitignore",
            outside: true,
        })],
        refuses(1, "setup.init-project-layer"),
        at("S7", Kind::LinkOutside),
    ),
    setup(
        "setup-at-the-workbench-ignore-a-directory",
        &[Lead::Plant(Plant::Dir(".jigc/.gitignore"))],
        refuses(1, "setup.init-project-layer"),
        at("S7", Kind::DirForAFile),
    ),
    setup(
        "setup-at-the-workbench-ignore-the-users-own-file",
        &[Lead::Plant(Plant::UsersLine(".jigc/.gitignore"))],
        DIRTY,
        at("S7", Kind::UsersLine),
    ),
    setup(
        "setup-at-the-workbench-ignore-a-change-staged-and-not-in-the-tree",
        &[Lead::Plant(Plant::Staged {
            at: ".jigc/.gitignore",
            kept: false,
        })],
        DIRTY,
        at("S7", Kind::StagedThere),
    ),
    setup(
        "setup-head-unborn-at-the-root-ignore-a-link-to-nothing",
        &[UNBORN, Lead::Plant(Plant::LinkToNothing(".gitignore"))],
        refuses(1, "setup.secrets-gitignore"),
        at("S9", Kind::LinkToNothing),
    ),
    setup(
        "setup-head-unborn-at-the-root-ignore-a-link-to-a-file-in-the-repository",
        &[
            UNBORN,
            Lead::Plant(Plant::LinkToAFile {
                at: ".gitignore",
                outside: false,
            }),
        ],
        refuses(1, "setup.secrets-gitignore"),
        at("S9", Kind::LinkInside),
    ),
    setup(
        "setup-head-unborn-at-the-root-ignore-a-link-to-a-file-outside-it",
        &[
            UNBORN,
            Lead::Plant(Plant::LinkToAFile {
                at: ".gitignore",
                outside: true,
            }),
        ],
        refuses(1, "setup.secrets-gitignore"),
        at("S9", Kind::LinkOutside),
    ),
    setup(
        "setup-head-unborn-at-the-root-ignore-a-directory",
        &[UNBORN, Lead::Plant(Plant::Dir(".gitignore"))],
        refuses(1, "setup.secrets-gitignore"),
        at("S9", Kind::DirForAFile),
    ),
    setup(
        "setup-head-unborn-at-the-root-ignore-the-users-own-file",
        &[UNBORN, Lead::Plant(Plant::UsersLine(".gitignore"))],
        Listed::Acts,
        at("S9", Kind::UsersLine),
    ),
    setup(
        "setup-head-unborn-at-the-root-ignore-a-change-staged-and-not-in-the-tree",
        &[
            UNBORN,
            Lead::Plant(Plant::Staged {
                at: ".gitignore",
                kept: false,
            }),
        ],
        DIRTY,
        at("S9", Kind::StagedThere),
    ),
];

/// **The other cells of `jigc setup`** — its refusing cell, its base cell, and the plants
/// no macro above makes. Each is listed from the row's doc:
///
/// - **run again where it is installed** — *"`jigc setup` re-run is a byte-identical
///   no-op"*; and **in the never-adopted repository**, it installs (`S1` to `S11`);
/// - **a file in no commit at a path it replaces or merges into, on a born `HEAD`**, is
///   refused before the first write (`S4`'s second sentence; the carryover gate,
///   `design/surface-contract.md`); **one the user committed** at a path it replaces is
///   refused the same way, by the human's ruling of 2026-10-15 on the record's §6, entry
///   10; **the guide the user committed** is left
///   *byte-identical*, as a copy that is not jigc's own (`S8`);
/// - **on an unborn `HEAD`, a file at a path it merges into rides the first commit** —
///   *"an *untracked* file at a path the install **merges into** rides the repository's
///   first commit instead of refusing"* (`S15`) — the root `.gitignore`'s cells;
/// - **the hook**: a line of the user's is kept — *"preserves/wraps any existing
///   `pre-commit` hook"* (`S10`); a link or a directory in its place, or a file in its
///   directory's, is no hook to wrap, and the install cannot write its own: **a failure
///   after the first write, which leaves the repository as it was** by the human's ruling
///   of 2026-10-14 on the record's §6, entry 2 — the refusal's words are read off a run, as
///   no doc gives them;
/// - **what is staged elsewhere stays out of the install commit** (`S11`: *"enumerated,
///   never a blanket `git add -A`"*), a name git quotes included; and **a hook that
///   rejects** the install commit fails it — the human's ruling of 2026-10-15 on the
///   record's §6, entry 11: the install commit runs the user's hooks, and a rejection is a
///   late failure, rolled back; the refusal's code is the one the door gives a failed
///   install commit.
const SETUP_CELLS: [Cell; 18] = [
    Cell {
        state: FRESH,
        ..setup(
            "setup-run-again-where-it-is-installed",
            &[],
            Listed::Acts,
            None,
        )
    },
    setup(
        "setup-in-a-repository-jigc-was-never-set-up-in",
        &[],
        Listed::Acts,
        None,
    ),
    setup(
        "setup-at-the-stamp-an-untracked-file",
        &[Lead::Plant(Plant::File(".jigc/version"))],
        DIRTY,
        at("S5", Kind::Untracked),
    ),
    setup(
        "setup-at-the-settings-record-an-untracked-file",
        &[Lead::Plant(Plant::File(".jigc/settings-entries.json"))],
        DIRTY,
        at("S6, written", Kind::Untracked),
    ),
    setup(
        "setup-at-the-guide-an-untracked-file",
        &[Lead::Plant(Plant::File(".claude/skills/jigc/SKILL.md"))],
        Listed::Acts,
        at("S8", Kind::Untracked),
    ),
    setup(
        "setup-under-the-project-layer-an-untracked-file",
        &[Lead::Plant(Plant::File(".jigc/config/notes.md"))],
        Listed::Acts,
        at("S4", Kind::Untracked),
    ),
    setup(
        "setup-a-file-where-the-workbench-goes",
        &[Lead::Plant(Plant::File(".jigc"))],
        refuses(1, "setup.write-bootstrap"),
        at("S1", Kind::FileForADir),
    ),
    setup(
        "setup-a-file-where-the-guides-directory-goes",
        &[Lead::Plant(Plant::File(".claude/skills/jigc"))],
        Listed::Acts,
        at("S8", Kind::FileForADir),
    ),
    setup(
        "setup-a-file-where-the-settings-directory-goes",
        &[Lead::Plant(Plant::File(".claude"))],
        refuses(1, "setup.inject-allowlist"),
        at("S3", Kind::FileForADir),
    ),
    setup(
        "setup-the-users-own-hook",
        &[Lead::Plant(Plant::UsersHook)],
        Listed::Acts,
        at("S10", Kind::UsersLine),
    ),
    setup(
        "setup-at-the-hook-a-link-to-nothing",
        &[
            HOOKS_IN_THE_TREE,
            Lead::Plant(Plant::LinkToNothing(THE_HOOK_IN_THE_TREE)),
        ],
        refuses(1, "setup.install-hook"),
        at("S10", Kind::LinkToNothing),
    ),
    setup(
        "setup-at-the-hook-a-link-to-a-file-in-the-repository",
        &[
            HOOKS_IN_THE_TREE,
            Lead::Plant(Plant::LinkToAFile {
                at: THE_HOOK_IN_THE_TREE,
                outside: false,
            }),
        ],
        DIRTY,
        at("S10", Kind::LinkInside),
    ),
    setup(
        "setup-at-the-hook-a-link-to-a-file-outside-it",
        &[
            HOOKS_IN_THE_TREE,
            Lead::Plant(Plant::LinkToAFile {
                at: THE_HOOK_IN_THE_TREE,
                outside: true,
            }),
        ],
        refuses(1, "setup.install-hook"),
        at("S10", Kind::LinkOutside),
    ),
    setup(
        "setup-at-the-hook-a-directory",
        &[
            HOOKS_IN_THE_TREE,
            Lead::Plant(Plant::Dir(THE_HOOK_IN_THE_TREE)),
        ],
        refuses(1, "setup.install-hook"),
        at("S10", Kind::DirForAFile),
    ),
    setup(
        "setup-a-file-where-the-hooks-directory-goes",
        &[HOOKS_IN_THE_TREE, Lead::Plant(Plant::File("the-hooks"))],
        refuses(1, "setup.install-hook"),
        at("S10", Kind::FileForADir),
    ),
    setup(
        "setup-a-file-staged-elsewhere",
        &[Lead::Plant(Plant::Staged {
            at: "notes/staged.txt",
            kept: true,
        })],
        Listed::Acts,
        at("S11", Kind::StagedElsewhere),
    ),
    setup(
        "setup-a-file-staged-under-a-name-git-quotes",
        &[Lead::Plant(Plant::Staged {
            at: A_NAME_GIT_QUOTES,
            kept: true,
        })],
        Listed::Acts,
        at("S11", Kind::QuotedName),
    ),
    setup(
        "setup-a-hook-that-rejects",
        &[Lead::Plant(Plant::RejectingHook)],
        refuses(1, "setup.install-commit"),
        at("S11", Kind::RejectingHook),
    ),
];

/// The install, laid down by the binary under test itself — so that a file `setup` writes
/// stands, committed, where a cell's edit goes.
const INSTALLED: Lead = Lead::Step(Step::Jigc(&["setup"]));

/// **The cells of `setup` over an edit to a file of its own install** — the needs that
/// waited for *"a file the never-adopted repository does not have"*: each is built by a
/// lead-in that installs first, so the file stands committed, then a line added by hand
/// to it, in no commit (the plan's plant **E**, last before the command). The root
/// `.gitignore`'s is the one `S9`'s condition, an unborn `HEAD`, lets stand: no file is in
/// a commit there, so the file that stands is the user's own, staged — `git add` after
/// `git init` — and the edit is a line added after it. Each is listed from the row's doc: a file at a path
/// the install **replaces** *"is refused before the first write while no commit holds its
/// bytes"* (`S4`; *"while its bytes are not jigc's own"*, by the human's ruling of
/// 2026-10-15), and so is one at a path it **merges into** on a born `HEAD` (the carryover
/// gate, `design/surface-contract.md`); the guide a user edited is *"left
/// **byte-identical**"* (`S8`); and on an unborn `HEAD` only an *untracked* file the
/// install merges into *"rides the repository's first commit instead of refusing"*
/// (`S15`) — a file staged there is tracked, its index bytes differ from `HEAD`, and *"a
/// path whose index *or* worktree bytes differ from `HEAD` before the install writes
/// anything refuses"* (`design/assistant-adapter.md` → *The install commit's pathspec*;
/// the carryover gate says the same, *untracked* in italics), so the root `.gitignore`'s
/// cell is refused as the others are.
const SETUP_OVER_ITS_OWN: [Cell; 8] = [
    setup(
        "setup-an-edit-to-its-bootstrap",
        &[INSTALLED, Lead::Plant(Plant::Edit(".jigc/AGENT.md"))],
        DIRTY,
        at("S1", Kind::EditAfter),
    ),
    setup(
        "setup-an-edit-to-the-host-file",
        &[INSTALLED, Lead::Plant(Plant::Edit("CLAUDE.md"))],
        DIRTY,
        at("S2", Kind::EditAfter),
    ),
    setup(
        "setup-an-edit-to-the-settings-file",
        &[INSTALLED, Lead::Plant(Plant::Edit(".claude/settings.json"))],
        DIRTY,
        at("S3", Kind::EditAfter),
    ),
    setup(
        "setup-an-edit-to-the-stamp",
        &[INSTALLED, Lead::Plant(Plant::Edit(".jigc/version"))],
        DIRTY,
        at("S5", Kind::EditAfter),
    ),
    setup(
        "setup-an-edit-to-the-settings-record",
        &[
            INSTALLED,
            Lead::Plant(Plant::Edit(".jigc/settings-entries.json")),
        ],
        DIRTY,
        at("S6, written", Kind::EditAfter),
    ),
    setup(
        "setup-an-edit-to-the-workbench-ignore",
        &[INSTALLED, Lead::Plant(Plant::Edit(".jigc/.gitignore"))],
        DIRTY,
        at("S7", Kind::EditAfter),
    ),
    setup(
        "setup-an-edit-to-the-guide",
        &[
            INSTALLED,
            Lead::Plant(Plant::Edit(".claude/skills/jigc/SKILL.md")),
        ],
        Listed::Acts,
        at("S8", Kind::EditAfter),
    ),
    setup(
        "setup-head-unborn-an-edit-to-the-root-ignore",
        &[
            UNBORN,
            Lead::Step(Step::File(Place::Repo(".gitignore"), "target/\n")),
            Lead::Step(Step::Git(&["add", "--", ".gitignore"])),
            Lead::Plant(Plant::Edit(".gitignore")),
        ],
        DIRTY,
        at("S9", Kind::EditAfter),
    ),
];

/// **The cells of `setup` and `uninstall` over a file of the install simply deleted** — the
/// human's ruling of 2026-10-18 on the review of the task `B6`, option A: a file the
/// install committed is deleted by hand, outside jigc, nothing put in its place and the
/// index left as it stands — the plain `rm` ([`Plant::Deleted`], the plan's plant **E**:
/// git lists the path *deleted* among the changes not staged, as it lists an edit). A cell
/// at each line of the two rows that **E** targets, on the lead-ins of the edits'
/// ([`SETUP_OVER_ITS_OWN`]; `uninstall` in `fresh`, which is installed). Each is listed
/// from the row's doc:
///
/// - **`setup` over a deleted file of its pathspec** (`S1`, `S2`, `S3`, `S5`,
///   `S6, written`, `S7`): *"A path that was **absent from the working tree before the
///   run, with nothing in the index that `HEAD` does not hold**, is not the adopter's
///   uncommitted work: the install restores it and commits it, at every member"* — the
///   human's ruling of 2026-10-06 on the fix pass's item 1, as
///   `design/assistant-adapter.md` states it — so each acts;
/// - **the guide deleted** (`S8`): of the guide's three states *"only two of them are
///   jigc's: **absent** (the first install) and **owned**"* — `setup` writes it again, and
///   the cell acts;
/// - **the user's staged root `.gitignore` deleted from the tree, on an unborn `HEAD`**
///   (`S9`): *"What still refuses is the path absent from the working tree whose **index**
///   entry holds bytes of the adopter's (staged, then deleted)"* — refused;
/// - **`uninstall` over the host file or the settings file deleted** (`U2`, `U3`): no
///   sentence says the state refuses — as for an edit there, the cells are listed as
///   acting, and held to `U9`'s *never carries* the user's own uncommitted change, a
///   deletion included.
const A_FILE_OF_THE_INSTALL_DELETED: [Cell; 10] = [
    setup(
        "setup-its-bootstrap-deleted",
        &[INSTALLED, Lead::Plant(Plant::Deleted(".jigc/AGENT.md"))],
        Listed::Acts,
        at("S1", Kind::EditAfter),
    ),
    setup(
        "setup-the-host-file-deleted",
        &[INSTALLED, Lead::Plant(Plant::Deleted("CLAUDE.md"))],
        Listed::Acts,
        at("S2", Kind::EditAfter),
    ),
    setup(
        "setup-the-settings-file-deleted",
        &[
            INSTALLED,
            Lead::Plant(Plant::Deleted(".claude/settings.json")),
        ],
        Listed::Acts,
        at("S3", Kind::EditAfter),
    ),
    setup(
        "setup-the-stamp-deleted",
        &[INSTALLED, Lead::Plant(Plant::Deleted(".jigc/version"))],
        Listed::Acts,
        at("S5", Kind::EditAfter),
    ),
    setup(
        "setup-the-settings-record-deleted",
        &[
            INSTALLED,
            Lead::Plant(Plant::Deleted(".jigc/settings-entries.json")),
        ],
        Listed::Acts,
        at("S6, written", Kind::EditAfter),
    ),
    setup(
        "setup-the-workbench-ignore-deleted",
        &[INSTALLED, Lead::Plant(Plant::Deleted(".jigc/.gitignore"))],
        Listed::Acts,
        at("S7", Kind::EditAfter),
    ),
    setup(
        "setup-the-guide-deleted",
        &[
            INSTALLED,
            Lead::Plant(Plant::Deleted(".claude/skills/jigc/SKILL.md")),
        ],
        Listed::Acts,
        at("S8", Kind::EditAfter),
    ),
    setup(
        "setup-head-unborn-the-staged-root-ignore-deleted",
        &[
            UNBORN,
            Lead::Step(Step::File(Place::Repo(".gitignore"), "target/\n")),
            Lead::Step(Step::Git(&["add", "--", ".gitignore"])),
            Lead::Plant(Plant::Deleted(".gitignore")),
        ],
        DIRTY,
        at("S9", Kind::EditAfter),
    ),
    uninstall(
        "uninstall-the-host-file-deleted",
        &[Lead::Plant(Plant::Deleted("CLAUDE.md"))],
        Listed::Acts,
        at("U2", Kind::EditAfter),
    ),
    uninstall(
        "uninstall-the-settings-file-deleted",
        &[Lead::Plant(Plant::Deleted(".claude/settings.json"))],
        Listed::Acts,
        at("U3", Kind::EditAfter),
    ),
];

/// **A task that landed** — `record-a-decision`, created and finalized: the repository a
/// teardown or a second install meets after somebody worked in it.
const A_LANDED_TASK: [Lead; 12] = [
    A_DECISION[0],
    A_DECISION[1],
    A_DECISION[2],
    A_DECISION[3],
    A_DECISION[4],
    A_DECISION[5],
    A_DECISION[6],
    A_DECISION[7],
    A_DECISION[8],
    A_DECISION[9],
    A_DECISION[10],
    Lead::Step(Step::Jigc(&["task", "finalize", "record-a-decision"])),
];

/// **The cells of `jigc uninstall`**, beside the acceptance's four. **Every one that acts is
/// held to `U9`** — the human's ruling of 2026-10-14: the teardown lands as one path-scoped
/// commit of what it removed, and never carries the user's own uncommitted edits. Each is
/// listed from the row's doc:
///
/// - **run as typed in `fresh`, and after a task landed there**, it removes the enumerated
///   set (`U1` to `U5`);
/// - **a line of the user's, or an edit, or a change staged, at the host file and the
///   settings file** (`U2`, `U3`): no sentence says the state refuses — the ruling leaves to
///   the fixer whether it refuses or commits only jigc's part — so the cells are listed as
///   acting, and held to the ruling's *never carries*;
/// - **the user's own lines in the hook** stay — *"what is left is restored byte for byte"*
///   (`U4`); so do **a hook that rejects**'s, and a change staged elsewhere, a name git
///   quotes included, stays out of the teardown's commit;
/// - **the user's own linked worktree**, its directory there or moved away, is no
///   registration under `.jigc/worktrees/`, which is the only kind the teardown removes
///   (`U6`);
/// - **a file jigc did not write, under `.jigc/logs/`**, goes under the consent — *"the one
///   way past it is **`--force`**"* (`U8`) — named.
const UNINSTALL_CELLS: [Cell; 15] = [
    uninstall("uninstall-run-as-typed", &[], Listed::Acts, None),
    uninstall(
        "uninstall-after-a-task-landed",
        &A_LANDED_TASK,
        Listed::Acts,
        None,
    ),
    uninstall(
        "uninstall-an-edit-to-the-host-file",
        &[Lead::Plant(Plant::Edit("CLAUDE.md"))],
        Listed::Acts,
        at("U2", Kind::EditAfter),
    ),
    uninstall(
        "uninstall-a-line-of-the-users-in-the-host-file",
        &[Lead::Plant(Plant::UsersLine("CLAUDE.md"))],
        Listed::Acts,
        at("U2", Kind::UsersLine),
    ),
    uninstall(
        "uninstall-a-change-staged-at-the-host-file-and-not-in-the-file",
        &[Lead::Plant(Plant::Staged {
            at: "CLAUDE.md",
            kept: false,
        })],
        Listed::Acts,
        at("U2", Kind::StagedThere),
    ),
    uninstall(
        "uninstall-an-edit-to-the-settings-file",
        &[Lead::Plant(Plant::Edit(".claude/settings.json"))],
        refuses(1, "uninstall.remove-allowlist"),
        at("U3", Kind::EditAfter),
    ),
    uninstall(
        "uninstall-a-line-of-the-users-in-the-settings-file",
        &[Lead::Plant(Plant::UsersLine(".claude/settings.json"))],
        refuses(1, "uninstall.remove-allowlist"),
        at("U3", Kind::UsersLine),
    ),
    uninstall(
        "uninstall-a-change-staged-at-the-settings-file-and-not-in-the-file",
        &[Lead::Plant(Plant::Staged {
            at: ".claude/settings.json",
            kept: false,
        })],
        Listed::Acts,
        at("U3", Kind::StagedThere),
    ),
    uninstall(
        "uninstall-the-users-own-lines-in-the-hook",
        &[Lead::Plant(Plant::UsersHook)],
        Listed::Acts,
        at("U4", Kind::UsersLine),
    ),
    uninstall(
        "uninstall-the-users-own-worktree",
        &[Lead::Plant(Plant::Worktree { moved_away: false })],
        Listed::Acts,
        at("U6", Kind::WorktreeThere),
    ),
    uninstall(
        "uninstall-the-users-own-worktree-moved-away",
        &[Lead::Plant(Plant::Worktree { moved_away: true })],
        Listed::Acts,
        at("U6", Kind::WorktreeMovedAway),
    ),
    uninstall(
        "uninstall-a-file-staged-elsewhere",
        &[Lead::Plant(Plant::Staged {
            at: "notes/staged.txt",
            kept: true,
        })],
        Listed::Acts,
        at("U9", Kind::StagedElsewhere),
    ),
    uninstall(
        "uninstall-a-file-staged-under-a-name-git-quotes",
        &[Lead::Plant(Plant::Staged {
            at: A_NAME_GIT_QUOTES,
            kept: true,
        })],
        Listed::Acts,
        at("U9", Kind::QuotedName),
    ),
    uninstall(
        "uninstall-a-hook-that-rejects",
        &[Lead::Plant(Plant::RejectingHook)],
        Listed::Acts,
        at("U9", Kind::RejectingHook),
    ),
    Cell {
        argv: &["uninstall", "--force"],
        ..uninstall(
            "uninstall-a-foreign-file-under-logs-by-consent",
            &[Lead::Plant(Plant::File(".jigc/logs/notes.md"))],
            Listed::Acts,
            at("U1", Kind::Foreign),
        )
    },
];

/// **A foreign doc that conforms**, at a decision's home: what `ingest` adopts — an ADR as
/// the dev pack's schema has it, which no command of jigc wrote.
const A_FOREIGN_DECISION: Lead = Lead::Step(Step::File(
    Place::Repo("docs/decisions/a-foreign-way.md"),
    "---\nstatus: proposed\ndate: 2026-10-01\nschema-version: 2\n---\n\n# A foreign way\n\n\
     ## Context\n\nWhy, in a line.\n\n## Options\n\n\n\n## Decision\n\nWhat, in a line.\n\n\
     ## Consequences\n\nThen, in a line.\n",
));

/// One cell of `jigc ingest`, in `fresh`.
const fn ingest(name: &'static str, leads: &'static [Lead], aim: Option<Aim>) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf: &["ingest"],
        state: FRESH,
        leads,
        argv: &["ingest"],
        listed: Listed::Acts,
        beside: None,
        aim,
        marks: &[],
        at: None,
    }
}

/// One cell of `jigc unmanage`, over the committed vision, in `committed-singletons`.
const fn unmanage(name: &'static str, leads: &'static [Lead], aim: Option<Aim>) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf: &["unmanage"],
        state: SINGLETONS,
        leads,
        argv: &["unmanage", "VISION.md"],
        listed: Listed::Acts,
        beside: None,
        aim,
        marks: &[],
        at: None,
    }
}

/// **The cells of `jigc ingest` and `jigc unmanage`**, each listed as acting: neither doc
/// names a refusal inside a repository jigc is set up in, and *"A miss is **exit 0**"*
/// (`design/command-output-contract.md`) for `unmanage` over a path never managed — its
/// refusing spelling. `ingest` adopts the foreign decision *"register-only"* (`I3`), and
/// `unmanage` drops the vision's baseline and edges, the file left as it is (`X1`); a
/// file of the user's under each cache stays.
const REGISTER_CELLS: [Cell; 8] = [
    ingest("ingest-with-nothing-to-adopt", &[], None),
    ingest(
        "ingest-a-foreign-decision-that-conforms",
        &[A_FOREIGN_DECISION],
        None,
    ),
    ingest(
        "ingest-a-file-of-the-users-under-the-file-state-hashes",
        &[
            A_FOREIGN_DECISION,
            Lead::Plant(Plant::File(".jigc/state/notes.md")),
        ],
        at("I1", Kind::Untracked),
    ),
    ingest(
        "ingest-a-file-of-the-users-under-the-edge-index",
        &[
            A_FOREIGN_DECISION,
            Lead::Plant(Plant::File(".jigc/index/notes.md")),
        ],
        at("I2", Kind::Untracked),
    ),
    Cell {
        state: FRESH,
        argv: &["unmanage", "FOREIGN.md"],
        ..unmanage("unmanage-a-path-never-managed", &[], None)
    },
    unmanage("unmanage-the-committed-vision", &[], None),
    unmanage(
        "unmanage-a-file-of-the-users-under-the-file-state-hashes",
        &[Lead::Plant(Plant::File(".jigc/state/notes.md"))],
        at("X1", Kind::Untracked),
    ),
    unmanage(
        "unmanage-a-file-of-the-users-under-the-edge-index",
        &[Lead::Plant(Plant::File(".jigc/index/notes.md"))],
        at("X2", Kind::Untracked),
    ),
];

/// One base cell of a leaf in a prepared state that is not its home: it runs on demand.
const fn elsewhere(
    name: &'static str,
    leaf: &'static [&'static str],
    argv: &'static [&'static str],
    state: Prepared,
    leads: &'static [Lead],
) -> Cell {
    Cell {
        spelling: None,
        name,
        leaf,
        state,
        leads,
        argv,
        listed: Listed::Acts,
        beside: None,
        aim: None,
        marks: &[],
        at: None,
    }
}

/// **The base cell of each of the four leaves in the prepared states that are not its
/// home** — run on demand, in no gate. Each is listed as acting — a second install is a
/// no-op, a teardown removes the set, an ingest adopts what conforms, and `unmanage` over
/// a path never managed is a miss at exit 0 — but three: in the never-adopted repository
/// `ingest` and `unmanage` refuse, *this project isn't set up* (the words read off a run:
/// `design/bootstrap.md` routes an unset project to `jigc setup`), and in `refs-post-hoc`,
/// whose open task stages a doc, the teardown refuses over it (`U7`). In `fresh` a cell
/// that would be the leaf's refusing spelling runs after a task landed.
const ELSEWHERE: [Cell; 24] = [
    elsewhere(
        "setup-after-a-task-landed",
        &["setup"],
        &["setup"],
        FRESH,
        &A_LANDED_TASK,
    ),
    elsewhere(
        "setup-in-committed-singletons",
        &["setup"],
        &["setup"],
        SINGLETONS,
        &[],
    ),
    elsewhere(
        "setup-in-migrated",
        &["setup"],
        &["setup"],
        Prepared::Of(State::Migrated),
        &[],
    ),
    elsewhere(
        "setup-in-refs-post-hoc",
        &["setup"],
        &["setup"],
        Prepared::Of(State::RefsPostHoc),
        &[],
    ),
    elsewhere(
        "setup-in-chatty-hooks",
        &["setup"],
        &["setup"],
        Prepared::Of(State::ChattyHooks),
        &[],
    ),
    elsewhere(
        "setup-in-vendored",
        &["setup"],
        &["setup"],
        Prepared::Of(State::Vendored),
        &[],
    ),
    elsewhere(
        "uninstall-in-a-repository-jigc-was-never-set-up-in",
        &["uninstall"],
        &["uninstall"],
        BARE,
        &[],
    ),
    elsewhere(
        "uninstall-in-committed-singletons",
        &["uninstall"],
        &["uninstall"],
        SINGLETONS,
        &[],
    ),
    elsewhere(
        "uninstall-in-migrated",
        &["uninstall"],
        &["uninstall"],
        Prepared::Of(State::Migrated),
        &[],
    ),
    Cell {
        listed: refuses(1, "uninstall.staged-prose"),
        ..elsewhere(
            "uninstall-in-refs-post-hoc",
            &["uninstall"],
            &["uninstall"],
            Prepared::Of(State::RefsPostHoc),
            &[],
        )
    },
    elsewhere(
        "uninstall-in-chatty-hooks",
        &["uninstall"],
        &["uninstall"],
        Prepared::Of(State::ChattyHooks),
        &[],
    ),
    elsewhere(
        "uninstall-in-vendored",
        &["uninstall"],
        &["uninstall"],
        Prepared::Of(State::Vendored),
        &[],
    ),
    Cell {
        listed: refuses(1, "this project isn't set up"),
        ..elsewhere(
            "ingest-in-a-repository-jigc-was-never-set-up-in",
            &["ingest"],
            &["ingest"],
            BARE,
            &[],
        )
    },
    elsewhere(
        "ingest-in-committed-singletons",
        &["ingest"],
        &["ingest"],
        SINGLETONS,
        &[],
    ),
    elsewhere(
        "ingest-in-migrated",
        &["ingest"],
        &["ingest"],
        Prepared::Of(State::Migrated),
        &[],
    ),
    elsewhere(
        "ingest-in-refs-post-hoc",
        &["ingest"],
        &["ingest"],
        Prepared::Of(State::RefsPostHoc),
        &[],
    ),
    elsewhere(
        "ingest-in-chatty-hooks",
        &["ingest"],
        &["ingest"],
        Prepared::Of(State::ChattyHooks),
        &[],
    ),
    elsewhere(
        "ingest-in-vendored",
        &["ingest"],
        &["ingest"],
        Prepared::Of(State::Vendored),
        &[],
    ),
    Cell {
        listed: refuses(1, "this project isn't set up"),
        ..elsewhere(
            "unmanage-in-a-repository-jigc-was-never-set-up-in",
            &["unmanage"],
            &["unmanage", "VISION.md"],
            BARE,
            &[],
        )
    },
    elsewhere(
        "unmanage-in-fresh",
        &["unmanage"],
        &["unmanage", "VISION.md"],
        FRESH,
        &[],
    ),
    elsewhere(
        "unmanage-in-migrated",
        &["unmanage"],
        &["unmanage", "VISION.md"],
        Prepared::Of(State::Migrated),
        &[],
    ),
    elsewhere(
        "unmanage-in-refs-post-hoc",
        &["unmanage"],
        &["unmanage", "VISION.md"],
        Prepared::Of(State::RefsPostHoc),
        &[],
    ),
    elsewhere(
        "unmanage-in-chatty-hooks",
        &["unmanage"],
        &["unmanage", "VISION.md"],
        Prepared::Of(State::ChattyHooks),
        &[],
    ),
    elsewhere(
        "unmanage-in-vendored",
        &["unmanage"],
        &["unmanage", "VISION.md"],
        Prepared::Of(State::Vendored),
        &[],
    ),
];

/// The family's cells: the acceptance's, then the four leaves' own (the plan's task `B5`).
pub const CELLS: &[&[Cell]] = &[
    ACCEPTANCE,
    &SETUP_CELLS,
    &SETUP_PLANTED,
    &SETUP_OVER_ITS_OWN,
    &A_FILE_OF_THE_INSTALL_DELETED,
    &UNINSTALL_CELLS,
    &REGISTER_CELLS,
    &ELSEWHERE,
];
