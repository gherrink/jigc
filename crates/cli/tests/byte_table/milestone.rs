//! **The byte table's rows of the `milestone` family** (`crates/cli/tests/byte_table.rs`)
//! — lifted quote for quote from the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`):
//! `jigc milestone finalize`'s, §4, and the seven other leaves', §4b — `create`,
//! `add-task`, `add-from-spec`, `provision`, `execute`, `join`, `discard` — under the rules
//! the install family's file states. A place a command mints, whose id no argv word holds,
//! is written [`MINTED`](crate::support::byte_cell::MINTED).
//!
//! **What of the record is no line.** `M8` is where *on a refusal: no* comes from. `M9` —
//! a refused boundary leaves objects in the store — is no difference to a table that
//! holds the store to one rule, that no object there before is absent after. **`M4` is a
//! second spelling**, `squash: false`: under `finalize.fan-out.squash: false` the boundary
//! makes one commit a sub-task that staged code and then the aggregate, and `M4` takes
//! `M3` out of force; a cell that sets the knob names the spelling ([`Spelling`]), by the
//! human's ruling of 2026-10-18.

use super::{ON_A_REFUSAL, THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ, refuses};
use crate::support::byte_cell::{
    Aim, Allows, Cell, Holding, Kind, Lead, Line, Listed, Mark, May, Plant, Prepared, Quote, Row,
    Spelling, Unit, When, Where,
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

/// **Where a sub-task's code is staged** — the place `M3`'s and `M4`'s commits take staged
/// code from, and no other checkout's: each quotes the sentence that names it.
const SUB_TASK_WORKTREES: &str = ".jigc/worktrees/<sub-task>";

/// **The project config layer, a pending delta of which the aggregate carries** —
/// `M3, the config`: its own sentence, the layer `T3` names, and the doc-only arm's
/// sentence that an ordinary finalize commits a pending delta.
const THE_CONFIG_CARRIED: [Quote; 3] = [
    Quote {
        doc: "design/finalize.md",
        heading: "`fan-out` finalize",
        words: "overlays the promoted docs + first-commit config",
    },
    Quote {
        doc: "design/finalize.md",
        heading: "5. Stage",
        words: "on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`),",
    },
    Quote {
        doc: "design/finalize.md",
        heading: "The doc-only arm",
        words: "**Outside the path set by construction:** a pending `.jigc/config` delta (an ordinary code-less finalize commits it beside the doc — gap G8) and the `.jigc/version` stamp, which this arm does not refresh",
    },
];

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
                Allows::Files(
                    Where::Under(".jigc/milestones/<unit>/merged"),
                    May::WriteWhole,
                ),
                &[Quote {
                    doc: "design/finalize.md",
                    heading: "2. Validate",
                    words: "`materialize` writes only into the gitignored `.jigc/milestones/<id>/merged/`, which it **clears and rebuilds every call**, so blocking after it still writes nothing durable",
                }],
            )
        },
        line(
            "M2",
            Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
            &[Quote {
                doc: "design/finalize.md",
                heading: "`fan-out` finalize",
                words: "The promote plan is one planner for both modes, and it carries the task door's clobber guard: a sub-task's `created` doc never promotes over a file already at its destination, and the refusal is taken before either mode lays down a commit",
            }],
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
        // the one combined commit, is out of force under it, and the commits the boundary
        // made are held to that chain: how many, in which order, each holding what.
        Line {
            spelling: Some("squash: false"),
            ..line(
                "M4",
                Allows::Commit(Holding::TheWorktreesOneByOne(SUB_TASK_WORKTREES)),
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
        // The project config layer, carried into the aggregate as it stands: `M3`'s own
        // *"overlays the promoted docs + first-commit config"*, under both spellings —
        // `squash: false` replaces `M3` and not these.
        line(
            "M3, the config",
            Allows::Files(Where::Under(".jigc/config"), May::Carried),
            &THE_CONFIG_CARRIED,
        ),
        line(
            "M3, the config's ignore file",
            Allows::Files(Where::Path(".jigc/.gitignore"), May::Carried),
            &THE_CONFIG_CARRIED,
        ),
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
            Allows::Files(
                Where::Under(".jigc/index"),
                May::Invalidated(".jigc/index/edges.json"),
            ),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "`fan-out` finalize",
                    words: "4. **Post-commit** runs once for the whole batch.",
                },
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
            "M10, the file-state hashes",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &[Quote {
                doc: "design/finalize.md",
                heading: "`fan-out` finalize",
                words: "4. **Post-commit** runs once for the whole batch.",
            }],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

// ---------------------------------------------------------------------------
// The family's other seven rows — the record's §4b
// ---------------------------------------------------------------------------

/// The heading of `design/team-ready-state.md`'s commit model.
const THE_COMMIT_MODEL: &str = "The commit model — path-scoped commit at each milestone op";

/// The heading of `design/team-ready-state.md`'s read-back capability, where the re-seed is.
const THE_READ_BACK: &str =
    "Engine capability 2 (read-back) — the committed record is the engine's authority";

/// The heading of `design/write-commands.md`'s milestone verbs.
const MINTING_A_MILESTONE: &str = "Minting a milestone + its task list (M7)";

/// **A mint makes what was not there** — the sentence a place `<minted>` rests on beside
/// the door's own.
const A_MINT_REFUSES_WHAT_STOOD: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: THE_COMMIT_MODEL,
    words: "both mints still refuse a pre-existing id before creating anything, which is exactly what makes removing what this call minted safe",
};

/// **The record's home** — `M5`'s second and third sentences: one file a milestone, under
/// the docs root the cascade resolves.
const THE_RECORDS_HOME: [Quote; 2] = [
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
];

/// **The record doors' one commit**: the record alone — `A6`'s sentence.
const THE_RECORD_ALONE: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: THE_COMMIT_MODEL,
    words: "it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP",
};

/// `create`'s and `add-task`'s record commit.
const EACH_A_RECORD_COMMIT: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: THE_COMMIT_MODEL,
    words: "**`create` / `add-task`** — a **separate** record-only path-scoped commit each.",
};

/// `add-from-spec` and `discard` among the doors that write the record.
const THE_FIVE_RECORD_DOORS: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: THE_COMMIT_MODEL,
    words: "the **five record-only doors** are `cli::rollback::ROLLBACK_POPULATIONS`' `milestone-record` row — the four milestone ops (`create`, `add-task`, `add-from-spec`, `discard`)",
};

/// **The record's baseline, advanced** — `A7`'s two sentences.
const THE_BASELINE_ADVANCE: [Quote; 2] = [
    Quote {
        doc: "design/team-ready-state.md",
        heading: "`jigc task discard <sub-id>` — the per-sub-task terminal (M49)",
        words: "the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook",
    },
    Quote {
        doc: "design/team-ready-state.md",
        heading: "Engine capability 1 (write) — `set: on-transition` machine-maintained leaves",
        words: "the hash jigc recorded at its last landed record write — lives in `.jigc/state/`, which no clone carries",
    },
];

/// The append of one item to the record.
const AN_ITEM_APPENDED: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: "Engine capability 1 (write) — `set: on-transition` machine-maintained leaves",
    words: "**`add-task` — append.** Appends one `tasks` item (`id`/`intent`/`status: active`) via the M16 `add-item` primitive. Ordered, byte-stable.",
};

/// **The cache, re-derived** — the half of the re-seed at the milestone's area.
const THE_CACHE_RE_DERIVED: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: THE_READ_BACK,
    words: "the cache is **re-derived from the committed record when absent or stale**",
};

/// **Every operating door passes through the re-seed.**
const EVERY_VERB_ONE_SITE: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: "The terminal is terminal — a settled record has no workbench (M42 completion audit)",
    words: "**Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards",
};

/// **The re-seed's other half** — one standard area a sub-task the record names, an area
/// that stands skipped.
const THE_AREAS_RE_SEEDED: [Quote; 3] = [
    Quote {
        doc: "design/team-ready-state.md",
        heading: THE_READ_BACK,
        words: "So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names",
    },
    Quote {
        doc: "design/team-ready-state.md",
        heading: THE_READ_BACK,
        words: "A sub-task whose area already exists is skipped untouched",
    },
    EVERY_VERB_ONE_SITE,
];

/// `add-task`'s mint and its append.
const ADD_TASK_MINTS: Quote = Quote {
    doc: "design/write-commands.md",
    heading: MINTING_A_MILESTONE,
    words: "mints a sub-task under the milestone (a task work-unit: id = slug from intent, pinned to the **milestone's** base, isolated `tasks/<sub>/` area) and appends it to the milestone's task list",
};

/// `add-from-spec`'s mints and its seeding.
const ADD_FROM_SPEC_MINTS: Quote = Quote {
    doc: "design/write-commands.md",
    heading: MINTING_A_MILESTONE,
    words: "seeds the task list from a committed spec by **enumerating its repeatable `criteria` items** (the parse-items read path — `parse_sections` → the section's `items`, *not* the `jigc task bind` slice path) and minting one sub-task per criterion, the criterion's text as that sub-task's intent",
};

/// **A sub-task's worktree** — `M6`'s first sentence, which names the directory.
const THE_WORKTREES_DIRECTORY: Quote = Quote {
    doc: "design/storage.md",
    heading: "Repository layout & state location",
    words: "ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)",
};

/// **What `provision` makes** — a worktree of the base's tracked files, registered.
const A_PROVISIONED_WORKTREE: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: "What a fan-out worktree isolates — and what it does not (M45)",
    words: "**A provisioned worktree carries tracked files only.** `git worktree add --detach` populates from the pinned base's *tree*",
};

/// The heading of `design/team-ready-state.md`'s milestone abandon.
const THE_RECORD_SETTLING_OP: &str = "`jigc milestone discard <id>` — the record-settling op";

/// The two lines of the re-seed, under the ids a row gives them: the milestone's cache in
/// its area, and the sub-tasks' areas.
const fn the_re_seed(cache: &'static str, areas: &'static str) -> [Line; 2] {
    [
        line(
            cache,
            Allows::Files(Where::Under(".jigc/milestones/<unit>"), May::Rebuilt),
            &[THE_CACHE_RE_DERIVED, EVERY_VERB_ONE_SITE],
        ),
        line(
            areas,
            Allows::Files(Where::Under(".jigc/tasks/<sub-task>"), May::Rebuilt),
            &THE_AREAS_RE_SEEDED,
        ),
    ]
}

/// **`jigc milestone create`** — the record's §4b. It mints the milestone its title names,
/// whose id no argv word holds as such: its places are where nothing stood (`<minted>`, `byte_cell::MINTED`).
pub const CREATE: Row = Row {
    leaf: &["milestone", "create"],
    unit: Unit::Nothing,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "MC1",
            Allows::Files(Where::Under(".jigc/milestones/<minted>"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/write-commands.md",
                    heading: MINTING_A_MILESTONE,
                    words: "opens its area with an empty task list, and **pins one shared base** (= HEAD) that every sub-task inherits",
                },
                Quote {
                    doc: "design/finalize.md",
                    heading: "Dirty-tree policy",
                    words: "snapshots the pre-task staged state — the staged set (path + blob) **and the staged-deletion set** — into the working area",
                },
                A_MINT_REFUSES_WHAT_STOOD,
            ],
        ),
        line(
            "MC2",
            Allows::Files(
                Where::Path("<docs-root>/milestone-records/<minted>.md"),
                May::WriteWhole,
            ),
            &[
                EACH_A_RECORD_COMMIT,
                THE_RECORDS_HOME[0],
                THE_RECORDS_HOME[1],
            ],
        ),
        line(
            "MC3",
            Allows::Commit(Holding::OnlyTheLines),
            &[THE_RECORD_ALONE],
        ),
        line(
            "MC4",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &THE_BASELINE_ADVANCE,
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// The re-seed's two lines of `add-task`.
const ADD_TASK_RE_SEEDS: [Line; 2] = the_re_seed("MA2", "MA6");

/// **`jigc milestone add-task`** — the record's §4b. Its unit is the milestone, the first
/// word after the leaf; the sub-task it mints is named by an intent, and its area is where
/// nothing stood.
pub const ADD_TASK: Row = Row {
    leaf: &["milestone", "add-task"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "MA1",
            Allows::Files(Where::Under(".jigc/tasks/<minted>"), May::Rebuilt),
            &[ADD_TASK_MINTS, A_MINT_REFUSES_WHAT_STOOD],
        ),
        Line {
            quotes: &[ADD_TASK_MINTS, THE_CACHE_RE_DERIVED],
            ..ADD_TASK_RE_SEEDS[0]
        },
        line(
            "MA3",
            Allows::Files(
                Where::Path("<docs-root>/milestone-records/<unit>.md"),
                May::MergeInto,
            ),
            &[
                AN_ITEM_APPENDED,
                EACH_A_RECORD_COMMIT,
                THE_RECORDS_HOME[0],
                THE_RECORDS_HOME[1],
            ],
        ),
        line(
            "MA4",
            Allows::Commit(Holding::OnlyTheLines),
            &[THE_RECORD_ALONE],
        ),
        line(
            "MA5",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &THE_BASELINE_ADVANCE,
        ),
        ADD_TASK_RE_SEEDS[1],
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// The re-seed's two lines of `add-from-spec`.
const ADD_FROM_SPEC_RE_SEEDS: [Line; 2] = the_re_seed("MS2", "MS6");

/// **`jigc milestone add-from-spec`** — the record's §4b: `add-task`'s row, a sub-task a
/// criterion of the spec. Its home is `vendored`, the one prepared state that holds a
/// committed spec.
pub const ADD_FROM_SPEC: Row = Row {
    leaf: &["milestone", "add-from-spec"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Vendored),
    lines: &[
        line(
            "MS1",
            Allows::Files(Where::Under(".jigc/tasks/<minted>"), May::Rebuilt),
            &[ADD_FROM_SPEC_MINTS, A_MINT_REFUSES_WHAT_STOOD],
        ),
        Line {
            quotes: &[ADD_FROM_SPEC_MINTS, THE_CACHE_RE_DERIVED],
            ..ADD_FROM_SPEC_RE_SEEDS[0]
        },
        line(
            "MS3",
            Allows::Files(
                Where::Path("<docs-root>/milestone-records/<unit>.md"),
                May::MergeInto,
            ),
            &[
                THE_FIVE_RECORD_DOORS,
                AN_ITEM_APPENDED,
                THE_RECORDS_HOME[0],
                THE_RECORDS_HOME[1],
            ],
        ),
        line(
            "MS4",
            Allows::Commit(Holding::OnlyTheLines),
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: THE_COMMIT_MODEL,
                    words: "It mints one sub-task per spec criterion and commits the record once per sub-task",
                },
                THE_RECORD_ALONE,
            ],
        ),
        line(
            "MS5",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &THE_BASELINE_ADVANCE,
        ),
        ADD_FROM_SPEC_RE_SEEDS[1],
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// The re-seed's two lines of `provision`.
const PROVISION_RE_SEEDS: [Line; 2] = the_re_seed("MP3", "MP4");

/// **`jigc milestone provision`** — the record's §4b. It writes no record and commits
/// nothing: a worktree a sub-task, registered, and the re-seed.
pub const PROVISION: Row = Row {
    leaf: &["milestone", "provision"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "MP1",
            Allows::Files(Where::Under(SUB_TASK_WORKTREES), May::Rebuilt),
            &[THE_WORKTREES_DIRECTORY, A_PROVISIONED_WORKTREE],
        ),
        line(
            "MP2",
            Allows::Registered(SUB_TASK_WORKTREES),
            &[A_PROVISIONED_WORKTREE],
        ),
        PROVISION_RE_SEEDS[0],
        PROVISION_RE_SEEDS[1],
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// The re-seed's two lines of `execute`.
const EXECUTE_RE_SEEDS: [Line; 2] = the_re_seed("ME1", "ME2");

/// **`jigc milestone execute`** — the record's §4b: the re-seed, and nothing else; the door
/// composes the fan-out walk and mints nothing (`ME3`, no line).
pub const EXECUTE: Row = Row {
    leaf: &["milestone", "execute"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        EXECUTE_RE_SEEDS[0],
        EXECUTE_RE_SEEDS[1],
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// The re-seed's two lines of `join`.
const JOIN_RE_SEEDS: [Line; 2] = the_re_seed("MJ1", "MJ2");

/// **`jigc milestone join`** — the record's §4b: the re-seed, and nothing else; the join is
/// a preview and its overlay is never persisted (`MJ3`, no line).
pub const JOIN: Row = Row {
    leaf: &["milestone", "join"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        JOIN_RE_SEEDS[0],
        JOIN_RE_SEEDS[1],
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// What `discard` removes, a line a place — the record's `MD4`.
const THE_WORKBENCH_REMOVED: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: THE_RECORD_SETTLING_OP,
    words: "**The workbench is actually removed.** `discard` removes the sub-task areas **and `.jigc/milestones/<id>/`**",
};

/// **`jigc milestone discard`** — the record's §4b. Its consent is `--force` (`MD5`): it
/// refuses on three subjects and removes nothing, and under the consent the command's
/// output names what it takes.
pub const DISCARD: Row = Row {
    leaf: &["milestone", "discard"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: Some("--force"),
    home: Prepared::Of(State::Fresh),
    lines: &[
        line(
            "MD1",
            Allows::Files(
                Where::Path("<docs-root>/milestone-records/<unit>.md"),
                May::WriteWhole,
            ),
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: THE_RECORD_SETTLING_OP,
                    words: "Every **non-joined** item flips to `discarded`; the header flips to `discarded`.",
                },
                THE_RECORDS_HOME[0],
                THE_RECORDS_HOME[1],
            ],
        ),
        line(
            "MD2",
            Allows::Commit(Holding::OnlyTheLines),
            &[THE_FIVE_RECORD_DOORS, THE_RECORD_ALONE],
        ),
        line(
            "MD3",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
            &THE_BASELINE_ADVANCE,
        ),
        line(
            "MD4, the milestone's area",
            Allows::Files(Where::Under(".jigc/milestones/<unit>"), May::Removed),
            &[THE_WORKBENCH_REMOVED],
        ),
        line(
            "MD4, the sub-tasks' areas",
            Allows::Files(Where::Under(".jigc/tasks/<sub-task>"), May::Removed),
            &[THE_WORKBENCH_REMOVED],
        ),
        line(
            "MD4, the worktrees",
            Allows::Files(Where::Under(SUB_TASK_WORKTREES), May::Removed),
            &[THE_WORKBENCH_REMOVED, THE_WORKTREES_DIRECTORY],
        ),
        line(
            "MD6",
            Allows::Unregistered(".jigc/worktrees"),
            &[
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: THE_RECORD_SETTLING_OP,
                    words: "A door removes a registration from `.git/worktrees/` **only at a path it created** under `<jigc_home>/.jigc/worktrees/` — a sub-task's `<sub-task-id>`, or a boundary's own `.combine-*` — **by path**, one record at a time (`git worktree remove --force <path>`), and **a run that refuses removes none**",
                },
                Quote {
                    doc: "design/team-ready-state.md",
                    heading: THE_RECORD_SETTLING_OP,
                    words: "So every other registration — live, `prunable` or locked; a human's, or jigc's own under another path (a moved repository's sub-task records name the old one, and are exactly what `git worktree repair` needs) — is left byte-identical",
                },
            ],
        ),
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
    ],
};

/// The family's rows: the boundary's, then the seven of the record's §4b.
pub const ROWS: &[Row] = &[
    FINALIZE,
    CREATE,
    ADD_TASK,
    ADD_FROM_SPEC,
    PROVISION,
    EXECUTE,
    JOIN,
    DISCARD,
];

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
                &[
                    "doc",
                    "create",
                    "inconsistency",
                    "--title",
                    "Stage",
                    "--task",
                    $task,
                ],
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
        beside: Some(
            "milestone_promote_guards::a_suffix_landing_on_an_occupied_home_blocks_and_its_route_lands_the_milestone",
        ),
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
        beside: Some(
            "milestone_promote_guards::a_committed_occupant_is_never_routed_at_moving_out",
        ),
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
            jigc(&[
                "milestone",
                "add-task",
                "foreign-worktree-probe",
                "Add alpha file",
            ]),
            jigc(&[
                "milestone",
                "add-task",
                "foreign-worktree-probe",
                "Add beta file",
            ]),
            jigc(&["milestone", "provision", "foreign-worktree-probe"]),
            ALPHAS_CODE[0],
            ALPHAS_CODE[1],
            BETAS_CODE[0],
            BETAS_CODE[1],
            Lead::Plant(Plant::Worktree { moved_away: true }),
        ],
        argv: &["milestone", "finalize", "foreign-worktree-probe"],
        listed: Listed::Acts,
        beside: Some(
            "worktree_registration_reach::every_worktree_door_leaves_a_foreign_registration_exactly_as_it_found_it",
        ),
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
        &[],
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
        &[],
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
        &[],
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
