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

use super::{AFTER_A_COMMIT, ON_A_REFUSAL, refuses};
use crate::support::byte_cell::{
    Aim, Allows, Cell, Kind, Lead, Line, Listed, May, Plant, Prepared, Quote, Row, Spelling,
    THE_SOURCE, Typed, Unit, When, Where,
};
use crate::support::route_case::{Place, Step};
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
/// of force and rebuilding the edge index by a line of its own: `bare`, the orientation,
/// which never mints, and `--task`, the resume.
pub const START: Row = Row {
    leaf: &["start"],
    unit: Unit::TheTaskFlag,
    spellings: &[
        Spelling {
            name: "bare",
            typed: Typed::Bare,
            replaces: &["ST1"],
        },
        Spelling {
            name: "--task",
            typed: Typed::Word("--task"),
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
        // The edge index, rebuilt by the two spellings that read it — each a line of its
        // spelling, so a mint that wrote it would be outside every line (narrowed with the
        // family's cells, where a line of every spelling met no write of the mint's).
        Line {
            spelling: Some("bare"),
            ..line(
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
            )
        },
        Line {
            spelling: Some("--task"),
            ..line(
                "ST2, at a resume",
                Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
                &[
                    Quote {
                        doc: "design/write-commands.md",
                        heading: TASK_ORIGINATION,
                        words: "resumes an existing task: loads its working area, recomposes the task's own persisted workflow, shows where it is. No minting.",
                    },
                    THE_COMMITTED_REBUILD,
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

/// **`jigc workflow`** — the record's §4d. Its unit is the task `--task` names; its home
/// is `fresh`, with the leads that mint a milestone of one sub-task and provision its
/// worktree, where the re-entry runs. **One spelling**, `--preview`, which takes both lines
/// out of force.
pub const WORKFLOW: Row = Row {
    leaf: &["workflow"],
    unit: Unit::TheTaskFlag,
    spellings: &[Spelling {
        name: "--preview",
        typed: Typed::Word("--preview"),
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
        AFTER_A_COMMIT,
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
        AFTER_A_COMMIT,
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
        AFTER_A_COMMIT,
    ],
};

/// The family's rows.
pub const ROWS: &[Row] = &[START, WORKFLOW, MIGRATE, TASK_AMEND];

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

/// What a planted cell's plant is aimed at.
const fn at(line: &'static str, kind: Kind) -> Option<Aim> {
    Some(Aim { line, kind })
}

/// One cell of the family's, plainly spelled, its command run at the repository's root.
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

/// The base cell of a leaf in each prepared state but its home: the command as typed — with
/// no lead, or over the leads that make its precondition (`$leads`; in `bare`, `$bare_leads`,
/// what can be made there without `jigc`) and run where its home cells run (`$at`, but in
/// `bare`) — in `bare` refused, a project not set up, and in the others as `listed`.
macro_rules! elsewhere {
    ($prefix:literal, $leaf:expr, $argv:expr, $listed:expr) => {
        elsewhere!($prefix, $leaf, $argv, $listed; &[], &[], None)
    };
    ($prefix:literal, $leaf:expr, $argv:expr, $listed:expr; $bare_leads:expr, $leads:expr, $at:expr) => {
        [
            // In `bare` the command runs at the repository's root: no checkout of `jigc`'s
            // stands there.
            Cell {
                at: None,
                ..cell(
                    concat!($prefix, "-in-bare"),
                    $leaf,
                    Prepared::Bare,
                    $bare_leads,
                    $argv,
                    NOT_SET_UP,
                    None,
                )
            },
            Cell {
                at: $at,
                ..cell(
                    concat!($prefix, "-in-committed-singletons"),
                    $leaf,
                    Prepared::Of(State::CommittedSingletons),
                    $leads,
                    $argv,
                    $listed,
                    None,
                )
            },
            Cell {
                at: $at,
                ..cell(
                    concat!($prefix, "-in-migrated"),
                    $leaf,
                    Prepared::Of(State::Migrated),
                    $leads,
                    $argv,
                    $listed,
                    None,
                )
            },
            Cell {
                at: $at,
                ..cell(
                    concat!($prefix, "-in-refs-post-hoc"),
                    $leaf,
                    Prepared::Of(State::RefsPostHoc),
                    $leads,
                    $argv,
                    $listed,
                    None,
                )
            },
            Cell {
                at: $at,
                ..cell(
                    concat!($prefix, "-in-chatty-hooks"),
                    $leaf,
                    Prepared::Of(State::ChattyHooks),
                    $leads,
                    $argv,
                    $listed,
                    None,
                )
            },
            Cell {
                at: $at,
                ..cell(
                    concat!($prefix, "-in-vendored"),
                    $leaf,
                    Prepared::Of(State::Vendored),
                    $leads,
                    $argv,
                    $listed,
                    None,
                )
            },
        ]
    };
}

// --- `jigc start` ------------------------------------------------------------------

/// The command of the base cell: a task minted under `single-task`.
const ADD_A_FILE: &[&str] = &["start", "--workflow", "single-task", "add a file"];

/// One cell of `jigc start`.
const fn starts(
    name: &'static str,
    leads: &'static [Lead],
    argv: &'static [&'static str],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(name, &["start"], FRESH, leads, argv, listed, aim)
}

/// **The cells of `jigc start`**, each listed from the row's doc:
///
/// - **the leaf as the catalog types it**, bare `jigc start` over `fresh`, orients and acts,
///   judged under the spelling it types, `bare`, so a mint there is outside its lines;
///   **the base cell** mints a task (`ST1`); **its two spellings** over a task minted
///   first — the orientation reports it, the resume recomposes it, each rebuilding the edge
///   index (`ST2`, `ST2, at a resume`) and minting nothing;
/// - **at the minted task's place**, a residual of the user's — a directory holding no base
///   pin — is refused: *"→ **reject under the same code and the same stable key**"*, the
///   serial collision's; so is a live task of the same id, and a title that slugs to
///   nothing (`write.unslugable-title`) — *"before any write"*;
/// - **under the edge index**, a file of the user's stays, under each spelling that rebuilds
///   it.
const START_CELLS: [Cell; 9] = [
    Cell {
        spelling: Some("bare"),
        ..starts(
            "start-as-the-catalog-names-it",
            &[],
            &["start"],
            Listed::Acts,
            None,
        )
    },
    starts("start-a-task", &[], ADD_A_FILE, Listed::Acts, None),
    Cell {
        spelling: Some("bare"),
        ..starts(
            "start-bare-over-a-task-it-reports",
            &[jigc(ADD_A_FILE)],
            &["start"],
            Listed::Acts,
            None,
        )
    },
    Cell {
        spelling: Some("--task"),
        ..starts(
            "start-the-task-again",
            &[jigc(ADD_A_FILE)],
            &["start", "--task", "add-a-file"],
            Listed::Acts,
            None,
        )
    },
    // `ST1`, at the minted task's place.
    starts(
        "start-a-file-of-the-users-where-the-task-is-minted",
        &[Lead::Plant(Plant::File(".jigc/tasks/add-a-file/notes.md"))],
        ADD_A_FILE,
        refuses(1, "task.serial-collision"),
        at("ST1", Kind::Untracked),
    ),
    starts(
        "start-over-a-task-of-the-same-name",
        &[jigc(ADD_A_FILE)],
        ADD_A_FILE,
        refuses(1, "task.serial-collision"),
        None,
    ),
    starts(
        "start-a-title-that-slugs-to-nothing",
        &[],
        &["start", "--workflow", "single-task", "!!!"],
        refuses(1, "write.unslugable-title"),
        None,
    ),
    // `ST2` and `ST2, at a resume`, the edge index each spelling rebuilds.
    Cell {
        spelling: Some("bare"),
        ..starts(
            "start-bare-a-file-of-the-users-under-the-edge-index",
            &[
                jigc(ADD_A_FILE),
                Lead::Plant(Plant::File(".jigc/index/notes.md")),
            ],
            &["start"],
            Listed::Acts,
            at("ST2", Kind::Untracked),
        )
    },
    Cell {
        spelling: Some("--task"),
        ..starts(
            "start-the-task-again-a-file-of-the-users-under-the-edge-index",
            &[
                jigc(ADD_A_FILE),
                Lead::Plant(Plant::File(".jigc/index/notes.md")),
            ],
            &["start", "--task", "add-a-file"],
            Listed::Acts,
            at("ST2, at a resume", Kind::Untracked),
        )
    },
];

// --- `jigc workflow` ---------------------------------------------------------------

/// **A milestone of one sub-task, its worktree provisioned** — the state a sub-agent
/// re-enters: the milestone minted, the sub-task added, its worktree cut from the pinned
/// base.
macro_rules! a_sub_task_provisioned {
    ($($after:expr),* $(,)?) => {
        &[
            jigc(&["milestone", "create", "Ship it"]),
            jigc(&["milestone", "add-task", "ship-it", "add alpha"]),
            jigc(&["milestone", "provision", "ship-it"]),
            $($after,)*
        ]
    };
}

/// The sub-task's worktree, where its re-entry is typed.
const ITS_WORKTREE: Place = Place::Repo(".jigc/worktrees/add-alpha");
const RE_ENTER_IT: &[&str] = &["workflow", "sub-task", "--task", "add-alpha"];

/// Where the sub-task's commit doc is provisioned, in the main checkout's workbench.
const THE_COMMIT_DOC: &str = ".jigc/tasks/add-alpha/docs/commit:add-alpha.md";

/// One cell of `jigc workflow`, its command run at the sub-task's worktree.
const fn re_enters(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    Cell {
        at: Some(ITS_WORKTREE),
        ..cell(name, &["workflow"], FRESH, leads, RE_ENTER_IT, listed, aim)
    }
}

/// **The cells of `jigc workflow`**, each listed from the row's doc:
///
/// - **the leaf as the catalog types it**, `--preview` of `single-task` over `fresh`, acts
///   and writes nothing, judged under the spelling it types, `--preview`; **the base
///   cell** re-enters the provisioned sub-task at its worktree — its commit doc and
///   provenance manifest provisioned in its area (`WF1`), the edge index rebuilt (`WF2`);
///   **its spelling** `--preview` over the same state writes nothing — of `single-task`,
///   since a preview of the verb-routed `sub-task` is refused, `workflow.verb-routed`;
/// - **where the commit doc is provisioned**, a file of the user's there: the provisioning
///   makes what stood nowhere — *"mirroring how `jigc start` provisions at mint time"*, and
///   the mint *"keeps refusing rather than adopting the directory"* — so it refuses, naming
///   what stands there;
/// - **a link or a directory where the commit doc is provisioned** — a link to nothing, to
///   a file in the repository, to one outside it, a directory: none is the commit doc the
///   provisioning makes, and each is refused as the user's file there is, naming the path
///   (after the review of the task `B9`, H4's lead); **a file where the commit doc's
///   directory goes** is refused, changing nothing;
/// - **under the edge index**, a file of the user's stays.
const WORKFLOW_CELLS: [Cell; 12] = [
    Cell {
        spelling: Some("--preview"),
        ..cell(
            "workflow-as-the-catalog-names-it",
            &["workflow"],
            FRESH,
            &[],
            &["workflow", "single-task", "--preview"],
            Listed::Acts,
            None,
        )
    },
    re_enters(
        "workflow-a-sub-task-re-entered",
        a_sub_task_provisioned!(),
        Listed::Acts,
        None,
    ),
    Cell {
        spelling: Some("--preview"),
        argv: &["workflow", "single-task", "--preview"],
        ..re_enters(
            "workflow-a-preview-at-the-sub-tasks-worktree",
            a_sub_task_provisioned!(),
            Listed::Acts,
            None,
        )
    },
    // `WF1`, where the commit doc is provisioned: the provisioning mirrors the mint, which
    // refuses what stands at its area rather than land jigc's files beside it — so a file of
    // the user's where the commit doc goes is refused, naming it (the findings' §26). What the
    // re-entry does with what stands there is the record's §6, entry 25 — the review of the
    // task `B9`'s Q-a — **ruled by the human on 2026-10-26**: *first entry* is decided by
    // jigc's own provenance record, and an entry at the commit doc's place with no provenance
    // entry — a file, a directory, a link of any kind — refuses, naming the path, the check
    // following no link. The four cells it takes, marked until the ruling, are plain red under
    // their key; the link to nothing it writes over was never marked — the ruling on entry 23
    // forbade that overwrite under either answer.
    re_enters(
        "workflow-a-file-of-the-users-where-the-commit-doc-is-provisioned",
        a_sub_task_provisioned!(Lead::Plant(Plant::File(THE_COMMIT_DOC))),
        refuses(1, "commit:add-alpha"),
        at("WF1", Kind::Untracked),
    ),
    // `WF1`, a link or a directory where the commit doc is provisioned, and a file where its
    // directory goes (after the review of the task `B9`, H4's lead): what stands there is
    // refused as the user's file is.
    re_enters(
        "workflow-a-link-to-nothing-where-the-commit-doc-is-provisioned",
        a_sub_task_provisioned!(Lead::Plant(Plant::LinkToNothing(THE_COMMIT_DOC))),
        refuses(1, "commit:add-alpha"),
        at("WF1", Kind::LinkToNothing),
    ),
    re_enters(
        "workflow-a-link-to-a-file-in-the-repository-where-the-commit-doc-is-provisioned",
        a_sub_task_provisioned!(Lead::Plant(Plant::LinkToAFile {
            at: THE_COMMIT_DOC,
            outside: false,
        })),
        refuses(1, "commit:add-alpha"),
        at("WF1", Kind::LinkInside),
    ),
    re_enters(
        "workflow-a-link-to-a-file-outside-where-the-commit-doc-is-provisioned",
        a_sub_task_provisioned!(Lead::Plant(Plant::LinkToAFile {
            at: THE_COMMIT_DOC,
            outside: true,
        })),
        refuses(1, "commit:add-alpha"),
        at("WF1", Kind::LinkOutside),
    ),
    re_enters(
        "workflow-a-link-to-a-directory-outside-at-the-sub-tasks-docs-directory",
        a_sub_task_provisioned!(Lead::Plant(Plant::LinkToADir(".jigc/tasks/add-alpha/docs"))),
        refuses(1, ".jigc/tasks/add-alpha/docs"),
        at("WF1", Kind::LinkOnTheWay),
    ),
    re_enters(
        "workflow-a-directory-where-the-commit-doc-is-provisioned",
        a_sub_task_provisioned!(Lead::Plant(Plant::Dir(THE_COMMIT_DOC))),
        refuses(1, "commit:add-alpha"),
        at("WF1", Kind::DirForAFile),
    ),
    re_enters(
        "workflow-a-file-in-the-place-of-the-commit-docs-directory",
        a_sub_task_provisioned!(Lead::Plant(Plant::File(".jigc/tasks/add-alpha/docs"))),
        refuses(1, "add-alpha"),
        at("WF1", Kind::FileForADir),
    ),
    // `WF1`, a second re-entry after the sub-agent wrote its commit doc: *"so a sub-agent's
    // in-progress edits survive a later re-entry"* (`provision_on_first_entry`) — the doc is
    // provisioned once, and *provisioned* lets nothing that stood be rewritten (the sixth of
    // the fixes after the review of the task `B9`, the fourth fix's left-open: no plant of
    // the frame's is an edit under the workbench, and the sub-agent's own write through
    // `jigc` is the edit, a lead).
    re_enters(
        "workflow-a-sub-tasks-commit-doc-written-then-re-entered",
        a_sub_task_provisioned!(
            Lead::Step(Step::JigcAt(ITS_WORKTREE, RE_ENTER_IT)),
            Lead::Step(Step::JigcAt(
                ITS_WORKTREE,
                &[
                    "doc",
                    "set-field",
                    "commit:add-alpha#type",
                    "--value",
                    "docs",
                    "--task",
                    "add-alpha",
                ],
            )),
        ),
        Listed::Acts,
        None,
    ),
    // `WF2`, the edge index.
    re_enters(
        "workflow-a-file-of-the-users-under-the-edge-index",
        a_sub_task_provisioned!(Lead::Plant(Plant::File(".jigc/index/notes.md"))),
        Listed::Acts,
        at("WF2", Kind::Untracked),
    ),
];

// --- `jigc migrate` ----------------------------------------------------------------

/// The foreign file a migration is of, committed by the user, and the command.
const HISTORY: &str = "HISTORY.md";
const A_HISTORY: &str = "# History\n\n## 1.0.0\n\n- Added a thing.\n";
const MIGRATE_IT: &[&str] = &["migrate", "HISTORY.md", "--as", "changelog"];

/// **A foreign changelog, committed** — written by hand and committed by the user, as a
/// repository holds a history file jigc never managed: the migration's source.
macro_rules! a_foreign_history {
    ($($after:expr),* $(,)?) => {
        &[
            file(HISTORY, A_HISTORY),
            git(&["add", "--", HISTORY]),
            git(&["commit", "-q", "-m", "a history"]),
            $($after,)*
        ]
    };
}

/// One cell of `jigc migrate`.
const fn migrates(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(name, &["migrate"], FRESH, leads, MIGRATE_IT, listed, aim)
}

/// **The cells of `jigc migrate`**, each listed from the row's doc:
///
/// - **the leaf as the catalog types it**, over a path that names nothing, is refused;
///   **the base cell** migrates a committed `HISTORY.md` — the task's area made (`MI1`),
///   the source read and left as it stands (`MI2`);
/// - **at the migration task's place**, a residual of the user's is refused, the serial
///   collision's code;
/// - **at the source**, what stands in its place — a file of the user's, a link, a
///   directory — and an edit in no commit: a link or a directory is no file git can record,
///   *"refusing with `migrate.source-untrackable`"* (a directory refused by the read, in the
///   run's words); whatever the door reads, it writes nothing there.
const MIGRATE_CELLS: [Cell; 9] = [
    cell(
        "migrate-as-the-catalog-names-it",
        &["migrate"],
        FRESH,
        &[],
        &["migrate", "FOREIGN.md", "--as", "changelog"],
        refuses(1, "FOREIGN.md"),
        None,
    ),
    migrates(
        "migrate-a-foreign-history",
        a_foreign_history!(),
        Listed::Acts,
        None,
    ),
    // `MI1`, at the migration task's place.
    migrates(
        "migrate-a-file-of-the-users-where-the-migration-task-is-minted",
        a_foreign_history!(Lead::Plant(Plant::File(
            ".jigc/tasks/migrate-changelog-history-3268e06b69e1/notes.md"
        ))),
        refuses(1, "task.serial-collision"),
        at("MI1", Kind::Untracked),
    ),
    // `MI2`, at the source.
    migrates(
        "migrate-a-file-of-the-users-in-the-sources-place",
        a_foreign_history!(Lead::Plant(Plant::InPlaceOf(&Plant::File(HISTORY)))),
        Listed::Acts,
        at("MI2", Kind::Untracked),
    ),
    migrates(
        "migrate-a-link-to-nothing-in-the-sources-place",
        a_foreign_history!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            HISTORY
        )))),
        refuses(1, "migrate.source-untrackable"),
        at("MI2", Kind::LinkToNothing),
    ),
    migrates(
        "migrate-a-link-to-a-file-in-the-repository-in-the-sources-place",
        a_foreign_history!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: HISTORY,
            outside: false
        }))),
        refuses(1, "migrate.source-untrackable"),
        at("MI2", Kind::LinkInside),
    ),
    migrates(
        "migrate-a-link-to-a-file-outside-in-the-sources-place",
        a_foreign_history!(Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: HISTORY,
            outside: true
        }))),
        refuses(1, "migrate.source-untrackable"),
        at("MI2", Kind::LinkOutside),
    ),
    // A directory is no file the door's location predicates name: it is refused by the read,
    // in the run's words — no code, which no doc gives (the findings' §26, seen). The listing
    // names the source and no more, so that a fix which gives the refusal a code of its own,
    // naming the path, keeps the cell green (the sixth of the fixes after the review of the
    // task `B9`, its N8: the run's words were listed, which that fix would turn red).
    migrates(
        "migrate-a-directory-in-the-sources-place",
        a_foreign_history!(Lead::Plant(Plant::InPlaceOf(&Plant::Dir(HISTORY)))),
        refuses(1, HISTORY),
        at("MI2", Kind::DirForAFile),
    ),
    migrates(
        "migrate-an-edit-to-the-source",
        a_foreign_history!(Lead::Plant(Plant::Edit(HISTORY))),
        Listed::Acts,
        at("MI2", Kind::EditAfter),
    ),
];

// --- `jigc task amend` -------------------------------------------------------------

const AMEND_IT: &[&str] = &["task", "amend", "fix the message"];

/// One cell of `jigc task amend`.
const fn amends(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(
        name,
        &["task", "amend"],
        FRESH,
        leads,
        AMEND_IT,
        listed,
        aim,
    )
}

/// **The cells of `jigc task amend`**, each listed from the row's doc:
///
/// - **the leaf as the catalog types it**, with no intent, mints a task named after
///   `HEAD`'s commit; **the base cell** mints one named after its intent (`AM1`);
/// - **at the amend task's place**, a residual of the user's is refused, the serial
///   collision's code;
/// - **a merge commit at `HEAD`** is refused *"before anything mints"*,
///   `amend.head-shape`.
const AMEND_CELLS: [Cell; 4] = [
    cell(
        "task-amend-as-the-catalog-names-it",
        &["task", "amend"],
        FRESH,
        &[],
        &["task", "amend"],
        Listed::Acts,
        None,
    ),
    amends("task-amend-the-last-commit", &[], Listed::Acts, None),
    // `AM1`, at the amend task's place.
    amends(
        "task-amend-a-file-of-the-users-where-the-task-is-minted",
        &[Lead::Plant(Plant::File(
            ".jigc/tasks/fix-the-message/notes.md",
        ))],
        refuses(1, "task.serial-collision"),
        at("AM1", Kind::Untracked),
    ),
    amends(
        "task-amend-a-merge-commit",
        &[
            git(&["checkout", "-q", "-b", "side"]),
            file("side.txt", "a side\n"),
            git(&["add", "--", "side.txt"]),
            git(&["commit", "-q", "-m", "a side"]),
            git(&["checkout", "-q", "-"]),
            file("main.txt", "a main\n"),
            git(&["add", "--", "main.txt"]),
            git(&["commit", "-q", "-m", "a main"]),
            git(&["merge", "-q", "--no-ff", "side", "-m", "a merge"]),
        ],
        refuses(1, "amend.head-shape"),
        None,
    ),
];

const START_ELSEWHERE: [Cell; 6] = elsewhere!("start-a-task", &["start"], ADD_A_FILE, Listed::Acts);
// `workflow` and `migrate` in each other prepared state, over the precondition their home
// cells stand on — a milestone of one sub-task provisioned, re-entered at its worktree; a
// foreign history committed — so that the state's character meets the door, where a base
// cell with no lead refused for the missing sub-task or source (the sixth of the fixes after
// the review of the task `B9`, its N9). In `bare` the history is committed, by git alone;
// the milestone is made by `jigc`, which refuses before `jigc setup`, so the re-entry there
// stands on nothing, and refuses for the state's own character.
const WORKFLOW_ELSEWHERE: [Cell; 6] = elsewhere!(
    "workflow-a-sub-task-re-entered",
    &["workflow"],
    RE_ENTER_IT,
    Listed::Acts;
    &[],
    a_sub_task_provisioned!(),
    Some(ITS_WORKTREE)
);
const MIGRATE_ELSEWHERE: [Cell; 6] = elsewhere!(
    "migrate-a-foreign-history",
    &["migrate"],
    MIGRATE_IT,
    Listed::Acts;
    a_foreign_history!(),
    a_foreign_history!(),
    None
);
const AMEND_ELSEWHERE: [Cell; 6] = elsewhere!(
    "task-amend-the-last-commit",
    &["task", "amend"],
    AMEND_IT,
    Listed::Acts
);

/// The family's cells, a group a leaf, and its base cells in the other prepared states.
pub const CELLS: &[&[Cell]] = &[
    &START_CELLS,
    &START_ELSEWHERE,
    &WORKFLOW_CELLS,
    &WORKFLOW_ELSEWHERE,
    &MIGRATE_CELLS,
    &MIGRATE_ELSEWHERE,
    &AMEND_CELLS,
    &AMEND_ELSEWHERE,
];
