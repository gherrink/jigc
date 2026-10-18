//! **The byte table's row for `jigc milestone finalize`**
//! (`crates/cli/tests/byte_table.rs`) — lifted quote for quote from the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`,
//! §4), under the rules the install family's file states. The other leaves of the family
//! join this file with their rows if the human rules for it at the pilot's stop.
//!
//! **What of the record is no line.** `M8` is where *on a refusal: no* comes from. `M9` —
//! a refused boundary leaves objects in the store — is no difference to a table that
//! holds the store to one rule, that no object there before is absent after. **`M4` is a
//! second spelling**, `squash: false`: under `finalize.fan-out.squash: false` the boundary
//! makes one commit a sub-task that staged code and then the aggregate, and `M4` takes
//! `M3` out of force; a cell that sets the knob names the spelling ([`Spelling`]), by the
//! human's ruling of 2026-10-18.

use super::{ON_A_REFUSAL, refuses};
use crate::support::byte_cell::{
    Aim, Allows, Cell, Holding, Kind, Lead, Line, Listed, May, Plant, Prepared, Quote, Row, When,
    Where,
 Unit, Spelling, Mark,};
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

/// **Where a sub-task's code is staged** — the place `M3`'s and `M4`'s commits take staged
/// code from, and no other checkout's: each quotes the sentence that names it.
const SUB_TASK_WORKTREES: &str = ".jigc/worktrees/<sub-task>";

/// **`jigc milestone finalize`** — the record's §4. The record names no consent.
pub const FINALIZE: Row = Row {
    leaf: &["milestone", "finalize"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[Spelling {
        name: "squash: false",
        replaces: &["M3"],
    }],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        // The one line of the four rows whose own sentence says it holds on a refusal.
        Line {
            on_refusal: true,
            ..line(
                "M1",
                Allows::Files(Where::Under(".jigc/milestones/<unit>/merged"), May::WriteWhole),
                &[
                    Quote {
                        doc: "design/finalize.md",
                        heading: "2. Validate",
                        words: "`materialize` writes only into the gitignored `.jigc/milestones/<id>/merged/`, which it **clears and rebuilds every call**, so blocking after it still writes nothing durable",
                    },
                ],
            )
        },
        line(
            "M2",
            Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "`fan-out` finalize",
                    words: "The promote plan is one planner for both modes, and it carries the task door's clobber guard: a sub-task's `created` doc never promotes over a file already at its destination, and the refusal is taken before either mode lays down a commit",
                },
            ],
        ),
        line(
            "M3",
            Allows::Commit(Holding::TheWorktrees(SUB_TASK_WORKTREES)),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "`fan-out` finalize",
                    words: "The CLI folds the N worktrees' staged code-sets into one combined tree **off-line** (a throwaway index), overlays the promoted docs + first-commit config, and commits that tree from a **clean dedicated worktree**",
                },
                Quote {
                    doc: "design/finalize.md",
                    heading: "`fan-out` finalize",
                    words: "The main checkout is never mutated until that clean fast-forward, so a blocked combine or a hook rejection lands nothing and leaves unrelated main-checkout WIP intact (WIP-safe).",
                },
                Quote {
                    doc: "design/finalize.md",
                    heading: "`fan-out` finalize",
                    words: "a sub-agent's **code** lives in its own git worktree (`.jigc/worktrees/<sub-id>`, `git add`ed there)",
                },
            ],
        ),
        // `M4`: under `finalize.fan-out.squash: false` the boundary commits a sub-task's
        // staged code in a commit of its own, task by task, then the aggregate — so `M3`,
        // the one combined commit, is out of force under it.
        Line {
            spelling: Some("squash: false"),
            ..line(
                "M4",
                Allows::Commit(Holding::TheWorktrees(SUB_TASK_WORKTREES)),
                &[
                    Quote {
                        doc: "design/finalize.md",
                        heading: "`fan-out` finalize",
                        words: "**`squash: false` — one commit per code-carrying sub-task in task-id order, then the parent's aggregate commit.**",
                    },
                    Quote {
                        doc: "design/finalize.md",
                        heading: "`fan-out` finalize",
                        words: "Each per-sub-task commit applies **that one worktree's** staged code onto the checkout and `git commit -F`s the sub-task's own authored `commit` doc",
                    },
                    Quote {
                        doc: "design/finalize.md",
                        heading: "`fan-out` finalize",
                        words: "a sub-agent's **code** lives in its own git worktree (`.jigc/worktrees/<sub-id>`, `git add`ed there)",
                    },
                ],
            )
        },
        // The record's home is the second sentence's — one file a milestone — under the
        // docs root the cascade resolves, the third's; and the record is the milestone's
        // own, the first's.
        line(
            "M5",
            Allows::Files(
                Where::Path("<docs-root>/milestone-records/<unit>.md"),
                May::WriteWhole,
            ),
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: "The commit model — path-scoped commit at each milestone op",
                    words: "the record's `status`-flip write **folds into the existing join commit** (path-added alongside the promoted docs), not a second commit",
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
            ],
        ),
        line(
            "M6, the worktrees",
            Allows::Files(Where::Under(".jigc/worktrees/<sub-task>"), May::Removed),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: "Repository layout & state location",
                    words: "ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: "The per-task working area (staging)",
                    words: "Both keep their complement, to `.jigc/displaced/<unit-id>/<relative>`",
                },
            ],
        ),
        line(
            "M6, the sub-tasks' areas",
            Allows::Files(Where::Under(".jigc/tasks/<sub-task>"), May::Removed),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: "Repository layout & state location",
                    words: "ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: "The per-task working area (staging)",
                    words: "Both keep their complement, to `.jigc/displaced/<unit-id>/<relative>`",
                },
            ],
        ),
        line(
            "M6, moved aside",
            Allows::Files(
                Where::Under(".jigc/tasks/<sub-task>"),
                May::MovedAsideTo(".jigc/displaced"),
            ),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: "Repository layout & state location",
                    words: "ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: "The per-task working area (staging)",
                    words: "Both keep their complement, to `.jigc/displaced/<unit-id>/<relative>`",
                },
            ],
        ),
        line(
            "M6, the milestone's area",
            Allows::Files(Where::Under(".jigc/milestones/<unit>"), May::Removed),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: "Repository layout & state location",
                    words: "ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: "The per-task working area (staging)",
                    words: "Both keep their complement, to `.jigc/displaced/<unit-id>/<relative>`",
                },
            ],
        ),
        line(
            "M7",
            Allows::Unregistered(".jigc/worktrees"),
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: "`jigc milestone discard <id>` — the record-settling op",
                    words: "A door removes a registration from `.git/worktrees/` **only at a path it created** under `<jigc_home>/.jigc/worktrees/` — a sub-task's `<sub-task-id>`, or a boundary's own `.combine-*` — **by path**, one record at a time (`git worktree remove --force <path>`), and **a run that refuses removes none**",
                },
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: "`jigc milestone discard <id>` — the record-settling op",
                    words: "So every other registration — live, `prunable` or locked; a human's, or jigc's own under another path (a moved repository's sub-task records name the old one, and are exactly what `git worktree repair` needs) — is left byte-identical",
                },
            ],
        ),
        line(
            "M10, the edge index",
            Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "`fan-out` finalize",
                    words: "4. **Post-commit** runs once for the whole batch.",
                },
            ],
        ),
        line(
            "M10, the file-state hashes",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "`fan-out` finalize",
                    words: "4. **Post-commit** runs once for the whole batch.",
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

/// A payload a lead-in reads with `--from-file`: a file under `outside/`.
const fn payload(name: &'static str, body: &'static str) -> Lead {
    Lead::Step(Step::File(Place::Outside(name), body))
}

/// **One reporter's finding, staged in its own worktree**: the sub-task `$who-reporter`
/// creates the finding every reporter titles alike — `Stage` — so that the join has ids
/// to number apart. A sub-task is re-entered from its worktree and nowhere else, and from
/// there the sandbox's `outside/` is four directories up.
macro_rules! a_reporters_finding {
    ($task:literal) => {
        [
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &["doc", "create", "inconsistency", "--title", "Stage", "--task", $task],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &[
                    "doc",
                    "set-slot",
                    "inconsistency:stage#description",
                    "--from-file",
                    "../../../../outside/description",
                    "--task",
                    $task,
                ],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &[
                    "doc",
                    "set-field",
                    "inconsistency:stage#meta/kind",
                    "--value",
                    "doc-doc",
                    "--task",
                    $task,
                ],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &[
                    "doc",
                    "add-item",
                    "inconsistency:stage#sides",
                    "--title",
                    "a.md",
                    "--slug",
                    "a",
                    "--task",
                    $task,
                ],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &[
                    "doc",
                    "add-item",
                    "inconsistency:stage#sides",
                    "--title",
                    "b.md",
                    "--slug",
                    "b",
                    "--task",
                    $task,
                ],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &[
                    "doc",
                    "set-slot",
                    "inconsistency:stage#sides/a/says",
                    "--from-file",
                    "../../../../outside/says",
                    "--task",
                    $task,
                ],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &[
                    "doc",
                    "set-slot",
                    "inconsistency:stage#sides/b/says",
                    "--from-file",
                    "../../../../outside/says",
                    "--task",
                    $task,
                ],
            )),
        ]
    };
}

const ALPHA: [Lead; 7] = a_reporters_finding!("alpha-reporter");
const BRAVO: [Lead; 7] = a_reporters_finding!("bravo-reporter");

/// **A milestone of two reporters, joined** — `stage-reports`: each sub-task staged a
/// finding titled `Stage`, and the join numbered the second one onto `stage-2`, whose
/// home is `docs/inconsistencies/stage-2.md`. `$before` stands before the milestone is
/// created — which is where the plan's plant **T** goes, since `milestone create` pins the
/// unit's base — and `$after` after the join.
macro_rules! two_reporters_joined {
    ([$($before:expr),* $(,)?], [$($after:expr),* $(,)?]) => {
        &[
            $($before,)*
            jigc(&["milestone", "create", "stage reports"]),
            jigc(&[
                "milestone",
                "add-task",
                "stage-reports",
                "alpha reporter",
                "--workflow",
                "report-inconsistency",
            ]),
            jigc(&[
                "milestone",
                "add-task",
                "stage-reports",
                "bravo reporter",
                "--workflow",
                "report-inconsistency",
            ]),
            jigc(&["milestone", "provision", "stage-reports"]),
            jigc(&["milestone", "execute", "stage-reports"]),
            payload("description", "Two docs disagree.\n"),
            payload("says", "It says one thing.\n"),
            ALPHA[0], ALPHA[1], ALPHA[2], ALPHA[3], ALPHA[4], ALPHA[5], ALPHA[6],
            BRAVO[0], BRAVO[1], BRAVO[2], BRAVO[3], BRAVO[4], BRAVO[5], BRAVO[6],
            jigc(&["milestone", "join", "stage-reports"]),
            $($after,)*
        ]
    };
}

/// The home the join numbered the second reporter's finding onto.
const THE_NUMBERED_HOME: &str = "docs/inconsistencies/stage-2.md";

/// One line of code a sub-task wrote and staged in its own worktree.
macro_rules! staged_code {
    ($task:literal, $file:literal) => {
        [
            Lead::Step(Step::File(
                Place::Repo(concat!(".jigc/worktrees/", $task, "/", $file)),
                "code\n",
            )),
            Lead::Step(Step::Git(&[
                "-C",
                concat!(".jigc/worktrees/", $task),
                "add",
                $file,
            ])),
        ]
    };
}

const ALPHAS_CODE: [Lead; 2] = staged_code!("add-alpha-file", "alpha.txt");
const BETAS_CODE: [Lead; 2] = staged_code!("add-beta-file", "beta.txt");

/// **The acceptance cells of `jigc milestone finalize`** (the plan's §5): the two halves
/// of the blocking row `(R6, D-1)` of the review of `1.0.0-rc.24`, and the blind trial's
/// `L-22`. Each needs a fan-out, and each is built here after the prepared state is
/// copied in: a state that holds worktrees is never copied.
///
/// **The two cells of `(R6, D-1)` are listed as ones that refuse** — *"a sub-task's
/// `created` doc never promotes over a file already at its destination, and the refusal
/// is taken before either mode lays down a commit"* (`M2`). **The cell of `L-22` is
/// listed as one that acts**: a registration of somebody's own worktree is no reason to
/// refuse, and *"every other registration … is left byte-identical"* (`M7`).
const ACCEPTANCE: &[Cell] = &[
    Cell {
        spelling: None,
        name: "milestone-finalize-an-untracked-file-at-the-id-the-join-numbered-onto",
        leaf: &["milestone", "finalize"],
        state: Prepared::Of(State::Fresh),
        leads: two_reporters_joined!([], [Lead::Plant(Plant::File(THE_NUMBERED_HOME))]),
        argv: &["milestone", "finalize", "stage-reports"],
        listed: refuses(3, "finalize.promote-clobber"),
        beside: Some("milestone_promote_guards::a_suffix_landing_on_an_occupied_home_blocks_and_its_route_lands_the_milestone"),
        aim: Some(Aim {
            line: "M2",
            kind: Kind::Untracked,
        }),
        marks: &[],
    },
    Cell {
        spelling: None,
        name: "milestone-finalize-a-committed-file-at-that-id-from-before-the-milestone",
        leaf: &["milestone", "finalize"],
        state: Prepared::Of(State::Fresh),
        leads: two_reporters_joined!([Lead::Plant(Plant::Committed(THE_NUMBERED_HOME))], []),
        argv: &["milestone", "finalize", "stage-reports"],
        listed: refuses(3, "finalize.promote-clobber"),
        beside: Some("milestone_promote_guards::a_committed_occupant_is_never_routed_at_moving_out"),
        aim: Some(Aim {
            line: "M2",
            kind: Kind::Committed,
        }),
        marks: &[],
    },
    Cell {
        spelling: None,
        name: "milestone-finalize-the-users-own-worktree-moved-away",
        leaf: &["milestone", "finalize"],
        state: Prepared::Of(State::Fresh),
        leads: &[
            jigc(&["milestone", "create", "Foreign worktree probe"]),
            jigc(&["milestone", "add-task", "foreign-worktree-probe", "Add alpha file"]),
            jigc(&["milestone", "add-task", "foreign-worktree-probe", "Add beta file"]),
            jigc(&["milestone", "provision", "foreign-worktree-probe"]),
            ALPHAS_CODE[0],
            ALPHAS_CODE[1],
            BETAS_CODE[0],
            BETAS_CODE[1],
            Lead::Plant(Plant::Worktree { moved_away: true }),
        ],
        argv: &["milestone", "finalize", "foreign-worktree-probe"],
        listed: Listed::Acts,
        beside: Some("worktree_registration_reach::every_worktree_door_leaves_a_foreign_registration_exactly_as_it_found_it"),
        aim: Some(Aim {
            line: "M7",
            kind: Kind::WorktreeMovedAway,
        }),
        marks: &[],
    },
];

/// **One sub-task's commit doc, authored in its own worktree** — under `squash: false`
/// each code-carrying sub-task lands in a commit of its own, whose message is its own
/// `commit` doc: the sub-task re-entered from its worktree, and the doc's type and summary
/// set there.
macro_rules! its_commit_doc {
    ($task:literal) => {
        [
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &["workflow", "sub-task", "--task", $task],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &[
                    "doc",
                    "set-field",
                    concat!("commit:", $task, "#type"),
                    "--value",
                    "feat",
                    "--task",
                    $task,
                ],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(concat!(".jigc/worktrees/", $task)),
                &[
                    "doc",
                    "set-slot",
                    concat!("commit:", $task, "#summary"),
                    "--from-file",
                    "../../../../outside/summary",
                    "--task",
                    $task,
                ],
            )),
        ]
    };
}

const ALPHAS_COMMIT_DOC: [Lead; 3] = its_commit_doc!("add-alpha-file");
const BETAS_COMMIT_DOC: [Lead; 3] = its_commit_doc!("add-beta-file");

/// **A milestone of two code-carrying sub-tasks, joined, under `squash: false`** —
/// `squash-off`: the knob set and committed before the milestone is created, which pins
/// its base, so that no config delta is pending at the boundary; each sub-task re-entered
/// in its worktree, its commit doc authored and one file of code staged there; then the
/// join. `$after` stands after the join, last before the command.
macro_rules! two_sub_tasks_unsquashed {
    ([$($after:expr),* $(,)?]) => {
        &[
            jigc(&["config", "set", "finalize.fan-out.squash", "false"]),
            Lead::Step(Step::Git(&["add", ".jigc/config/manifest.yaml"])),
            Lead::Step(Step::Git(&["commit", "-q", "-m", "chore: squash false"])),
            jigc(&["milestone", "create", "Squash off"]),
            jigc(&["milestone", "add-task", "squash-off", "Add alpha file"]),
            jigc(&["milestone", "add-task", "squash-off", "Add beta file"]),
            jigc(&["milestone", "provision", "squash-off"]),
            jigc(&["milestone", "execute", "squash-off"]),
            payload("summary", "add a file\n"),
            ALPHAS_COMMIT_DOC[0], ALPHAS_COMMIT_DOC[1], ALPHAS_COMMIT_DOC[2],
            BETAS_COMMIT_DOC[0], BETAS_COMMIT_DOC[1], BETAS_COMMIT_DOC[2],
            ALPHAS_CODE[0], ALPHAS_CODE[1], BETAS_CODE[0], BETAS_CODE[1],
            jigc(&["milestone", "join", "squash-off"]),
            $($after,)*
        ]
    };
}

/// The mark of a cell whose boundary lands under `squash: false` and takes the edge index's
/// file away: the rows' record's §6, entry 16 — is a stamp invalidated by taking its file
/// away?
const AT_THE_EDGE_INDEX: &[Mark] = &[Mark::Silence(16)];

/// One cell of `jigc milestone finalize`'s spelling `squash: false`.
const fn unsquashed(
    name: &'static str,
    leads: &'static [Lead],
    listed: Listed,
    aim: Option<Aim>,
    marks: &'static [Mark],
) -> Cell {
    Cell {
        name,
        leaf: &["milestone", "finalize"],
        state: Prepared::Of(State::Fresh),
        leads,
        argv: &["milestone", "finalize", "squash-off"],
        listed,
        beside: None,
        aim,
        marks,
        spelling: Some("squash: false"),
    }
}

/// A file whose name git quotes, staged in the main checkout.
const A_NAME_GIT_QUOTES: &str = "notes/a \"quoted\" n\u{e4}me.txt";

/// **The cells of the spelling `squash: false`** — the human's ruling of 2026-10-18 on the
/// two spellings: one commit a code-carrying sub-task, then the aggregate (`M4`). Each
/// needs a fan-out, built after the prepared state is copied in: a state that holds
/// worktrees is never copied. A change the user staged in the main checkout, or under a
/// name git quotes, is no sub-task's and rides no commit of the boundary's; a hook that
/// rejects a commit of the chain is a refusal that tears nothing down (`M8`). The words of
/// the refusal are a run's.
const UNSQUASHED: [Cell; 4] = [
    unsquashed(
        "milestone-finalize-squash-false-two-sub-tasks",
        two_sub_tasks_unsquashed!([]),
        Listed::Acts,
        None,
        AT_THE_EDGE_INDEX,
    ),
    unsquashed(
        "milestone-finalize-squash-false-a-file-staged-elsewhere",
        two_sub_tasks_unsquashed!([Lead::Plant(Plant::Staged {
            at: "notes/staged-by-its-user.txt",
            kept: true,
        })]),
        Listed::Acts,
        Some(Aim {
            line: "M4",
            kind: Kind::StagedElsewhere,
        }),
        AT_THE_EDGE_INDEX,
    ),
    unsquashed(
        "milestone-finalize-squash-false-a-staged-file-whose-name-git-quotes",
        two_sub_tasks_unsquashed!([Lead::Plant(Plant::Staged {
            at: A_NAME_GIT_QUOTES,
            kept: true,
        })]),
        Listed::Acts,
        Some(Aim {
            line: "M4",
            kind: Kind::QuotedName,
        }),
        AT_THE_EDGE_INDEX,
    ),
    unsquashed(
        "milestone-finalize-squash-false-a-hook-that-rejects",
        two_sub_tasks_unsquashed!([Lead::Plant(Plant::RejectingHook)]),
        refuses(1, "`git commit` was rejected"),
        Some(Aim {
            line: "M4",
            kind: Kind::RejectingHook,
        }),
        &[],
    ),
];

/// The family's cells: the acceptance's. Its other cells are a later task's, if the human
/// rules for it at the pilot's stop.
pub const CELLS: &[&[Cell]] = &[ACCEPTANCE, &UNSQUASHED];
