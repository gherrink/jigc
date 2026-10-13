//! **The byte table's rows for the install family** — `jigc setup` and `jigc uninstall`
//! (`crates/cli/tests/byte_table.rs`). A row is what its command may change, line by
//! line, **lifted quote for quote from the committed record**
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`,
//! §1 and §2): a line's `id` is its number there, and the quote fence holds every quote
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

use super::{ON_A_REFUSAL, refuses};
use crate::support::byte_cell::{
    Aim, Allows, Cell, Condition, Holding, Kind, Lead, Line, May, Plant, Prepared, Quote, Row,
    When, Where,
};
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

/// **`jigc setup`** — the record's §1.
pub const SETUP: Row = Row {
    leaf: &["setup"],
    consent: Some("--force"),
    home: Prepared::Bare,
    lines: &[
        line(
            "S1",
            Allows::Files(Where::Path(".jigc/AGENT.md"), May::WriteWhole),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "1. Inject the bootstrap — hook (primary) + line (floor)",
                    words: "a stable one-line pointer (Claude Code: an `@.jigc/AGENT.md` import) to a **managed bootstrap file** the adapter writes under `.jigc/` and **regenerates on every `setup`**",
                },
            ],
        ),
        line(
            "S2",
            Allows::Files(Where::Path("CLAUDE.md"), May::MergeInto),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "1. Inject the bootstrap — hook (primary) + line (floor)",
                    words: "The always-loaded file is touched only to ensure the pointer is present (idempotent on the exact reference line); the content lives in the managed file.",
                },
            ],
        ),
        line(
            "S3",
            Allows::Files(Where::Path(".claude/settings.json"), May::MergeInto),
            &[
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Idempotency & irreversibility — the acceptance obligation",
                    words: "a pre-existing non-trivial `CLAUDE.md` / `.claude/settings.json` is **merged, never clobbered** (structure-aware)",
                },
            ],
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
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "2. Make `jigc` frictionless",
                    words: "`setup` writes it only where its install commit can carry the settings file, and removes a record that stands where it cannot",
                },
            ],
        ),
        line(
            "S6, removed",
            Allows::Files(Where::Path(".jigc/settings-entries.json"), May::Removed),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "2. Make `jigc` frictionless",
                    words: "`setup` writes it only where its install commit can carry the settings file, and removes a record that stands where it cannot",
                },
            ],
        ),
        line(
            "S7",
            Allows::Files(Where::Path(".jigc/.gitignore"), May::MergeInto),
            &[
                Quote {
                    doc: "design/assistant-adapter.md",
                    heading: "`jigc setup` has assistant-*neutral* install responsibilities too",
                    words: "`.jigc/.gitignore` is the one merged-into member a link is never followed at — its writer amends in place",
                },
            ],
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
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// **`jigc uninstall`** — the record's §2. It has no line for the index and none for a
/// commit: the doc is silent on both (the record's §6, entry 5), and until that is ruled
/// a staged removal is *git's state moved outside its allowance*.
pub const UNINSTALL: Row = Row {
    leaf: &["uninstall"],
    consent: Some("--force"),
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "U1",
            Allows::Files(Where::Under(".jigc"), May::Removed),
            &[
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                    words: "**`uninstall` removes exactly the enumerated repo-local setup-created set — now *complete*: `.jigc/`, the import line, all three `.claude/settings.json` writes (permit + `SessionStart` hook + `deny` floor), the `pre-commit` hook, and the adapter's owned guide artifact**",
                },
            ],
        ),
        line(
            "U2",
            Allows::Files(Where::Path("CLAUDE.md"), May::TakeOutOf),
            &[
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                    words: "unwire the `CLAUDE.md` import line",
                },
            ],
        ),
        line(
            "U3",
            Allows::Files(Where::Path(".claude/settings.json"), May::TakeOutOf),
            &[
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                    words: "`setup` now records the entries it adds in one committed file, `.jigc/settings-entries.json`, and the teardown removes exactly those, reading the record **only where git tracks the settings file**",
                },
            ],
        ),
        line(
            "U4",
            Allows::Files(Where::Hook, May::TakeOutOf),
            &[
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                    words: "**`uninstall` removes exactly the lines between jigc's two markers — whatever version wrote them, whatever stands around them**: what is left is restored byte for byte, and the file is removed only where nothing is left but the frame jigc itself puts around its block",
                },
            ],
        ),
        line(
            "U4, removed",
            Allows::Files(Where::Hook, May::Removed),
            &[
                Quote {
                    doc: "design/project-setup.md",
                    heading: "Flow 2 hardening — full ingest/adopt/cleanup (M21)",
                    words: "**`uninstall` removes exactly the lines between jigc's two markers — whatever version wrote them, whatever stands around them**: what is left is restored byte for byte, and the file is removed only where nothing is left but the frame jigc itself puts around its block",
                },
            ],
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
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: "`jigc milestone discard <id>` — the record-settling op",
                    words: "A door removes a registration from `.git/worktrees/` **only at a path it created** under `<jigc_home>/.jigc/worktrees/` — a sub-task's `<sub-task-id>`, or a boundary's own `.combine-*` — **by path**, one record at a time (`git worktree remove --force <path>`), and **a run that refuses removes none**",
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
pub const ROWS: &[Row] = &[SETUP, UNINSTALL];

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
    },
    Cell {
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

/// The family's cells: the acceptance's. Its other cells are a later task's, if the human
/// rules for it at the pilot's stop.
pub const CELLS: &[&[Cell]] = &[ACCEPTANCE];
