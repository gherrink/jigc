//! **The byte table's rows of the `milestone` family** (`crates/cli/tests/byte_table.rs`)
//! — lifted quote for quote from the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`):
//! `jigc milestone finalize`'s, §4, and the seven other leaves', §4b — `create`,
//! `add-task`, `add-from-spec`, `provision`, `execute`, `join`, `discard` — under the rules
//! the install family's file states. A place a command mints, whose id no argv word holds,
//! is written [`MINTED`](crate::support::byte_cell::MINTED), and one of the several
//! `add-from-spec` mints [`EACH_MINTED`](crate::support::byte_cell::EACH_MINTED).
//!
//! **What of the record is no line.** `M8` is where *on a refusal: no* comes from. `M9` —
//! a refused boundary leaves objects in the store — is no difference to a table that
//! holds the store to one rule, that no object there before is absent after. **`M4` is a
//! second spelling**, `squash: false`: under `finalize.fan-out.squash: false` the boundary
//! makes one commit a sub-task that staged code and then the aggregate, and `M4` takes
//! `M3` out of force; a cell that sets the knob names the spelling ([`Spelling`]), by the
//! human's ruling of 2026-10-18.

use super::{AFTER_A_COMMIT, ON_A_REFUSAL, THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ, refuses};
use crate::support::byte_cell::{
    Aim, Allows, Cell, Holding, Kind, Lead, Line, Listed, Mark, May, Plant, Prepared, Quote, Row,
    Spelling, Typed, Unit, When, Where,
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

/// **The project config layer the aggregate carries on its first commit** —
/// `M3, the config`: its own sentence, and the layer `T3, the config` names, under the
/// same condition. **Narrowed 2026-10-19**, after the review of the task `B7`'s second
/// part (its first finding): the line had quoted beside these the doc-only arm's sentence
/// that an ordinary finalize commits a pending delta, and held under any `HEAD` — so a file
/// of the user's nobody staged under `.jigc/config`, swept into a landed boundary, held.
/// Whether a pending delta rides was the rows' record's §6, entry 19: two docs said two
/// things, and the human ruled on 2026-10-19 that it does not — nothing nobody staged.
const THE_CONFIG_CARRIED: [Quote; 2] = [
    Quote {
        doc: "design/finalize.md",
        heading: "`fan-out` finalize",
        words: "overlays the promoted docs + first-commit config",
    },
    super::THE_CONFIG_LAYERS_FIRST_COMMIT,
];

/// **The human's ruling of 2026-10-19 on the rows' record's §6, entry 18** — the sentence
/// `M6, the worktrees moved aside` rests on, as `DECISIONS.md` records it, until the design
/// docs say it: their change is a finding of the run's ledger, as the ruling has it.
const THE_WORKTREES_LEFTOVERS_MOVED_ASIDE: Quote = Quote {
    doc: "DECISIONS.md",
    heading: B8_RULINGS,
    words: "**Before removing a landed sub-task's worktree, the boundary moves its unstaged and untracked files that git does not ignore into `.jigc/displaced/<sub-task-id>/` and names them**",
};

/// The heading of `DECISIONS.md`'s entry that records the human's three rulings of
/// 2026-10-19 — the task `B8`'s first commit.
const B8_RULINGS: &str = "2026-10-19 — The human's three rulings of the day, recorded: a landed boundary moves a sub-task worktree's unstaged and untracked files that git does not ignore aside, where it took them; `milestone provision` skips a sub-task the record has settled; and bound 2 is retired — a `doc` write over a part that holds an uncommitted hand edit refuses at the write; four marks leave their cells, one line is added, and no classification moves (the byte table, task B8, the first of its commits)";

/// The re-seed's two lines of `finalize` — `M11` and `M12`, added 2026-10-19 after the
/// review of the task `B7`'s second part (its tenth finding): the sentence the re-seed's
/// lines quote names this door among the seven, and the row lacked them.
const FINALIZE_RE_SEEDS: [Line; 2] = the_re_seed("M11", "M12");

/// **`jigc milestone finalize`** — the record's §4. The record names no consent.
pub const FINALIZE: Row = Row {
    leaf: &["milestone", "finalize"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[Spelling {
        name: "squash: false",
        typed: Typed::State,
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
        // The project config layer, carried into the aggregate as it stands on the layer's
        // first commit: `M3`'s own *"overlays the promoted docs + first-commit config"*,
        // under both spellings — `squash: false` replaces `M3` and not these.
        Line {
            when: When::FirstCommitOfItsPlace,
            ..line(
                "M3, the config",
                Allows::Files(Where::Under(".jigc/config"), May::Carried),
                &THE_CONFIG_CARRIED,
            )
        },
        Line {
            when: When::FirstCommitOfItsPlace,
            ..line(
                "M3, the config's ignore file",
                Allows::Files(Where::Path(".jigc/.gitignore"), May::Carried),
                &THE_CONFIG_CARRIED,
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
                May::MovedAsideTo(".jigc/displaced/<sub-task>"),
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
        // The milestone's area keeps its complement too — *"Both keep their complement"* —
        // the join's `merged/` tree included.
        line(
            "M6, the milestone's area moved aside",
            Allows::Files(
                Where::Under(".jigc/milestones/<unit>"),
                May::MovedAsideTo(".jigc/displaced/<unit>"),
            ),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: "The per-task working area (staging)",
                    words: "A landed `jigc milestone finalize` removes each sub-task's `tasks/<sub-id>/` *and* the milestone's own `milestones/<id>/`",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: "The per-task working area (staging)",
                    words: "Both keep their complement, to `.jigc/displaced/<unit-id>/<relative>`",
                },
            ],
        ),
        // A landed sub-task's worktree keeps what git neither ignores nor holds — the
        // human's ruling of 2026-10-19 on the rows' record's §6, entry 18, the line's
        // sentence until the design docs say it — under that sub-task's place in
        // `displaced/`. Last of the moved-aside lines: a directory made at `displaced/`, or
        // under a unit's place there, is the first such line's that covers it, as it was
        // before this one.
        line(
            "M6, the worktrees moved aside",
            Allows::Files(
                Where::Under(".jigc/worktrees/<sub-task>"),
                May::MovedAsideTo(".jigc/displaced/<sub-task>"),
            ),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: "Repository layout & state location",
                    words: "ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)",
                },
                THE_WORKTREES_LEFTOVERS_MOVED_ASIDE,
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
        FINALIZE_RE_SEEDS[0],
        FINALIZE_RE_SEEDS[1],
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
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
/// that stands skipped, and one the record has settled; rebuilt before a door's own guards,
/// for the re-entry a refusal routes to.
const THE_AREAS_RE_SEEDED: [Quote; 4] = [
    Quote {
        doc: "design/team-ready-state.md",
        heading: THE_READ_BACK,
        words: "So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names",
    },
    Quote {
        doc: "design/team-ready-state.md",
        heading: THE_READ_BACK,
        words: "A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**",
    },
    EVERY_VERB_ONE_SITE,
    THE_AREAS_SERVE_A_REFUSAL,
];

/// **What the re-seed's areas are for, at a door that refuses** — the loop the
/// zero-contribution refusal routed a fresh clone into, which the areas' half closes.
const THE_AREAS_SERVE_A_REFUSAL: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: THE_READ_BACK,
    words: "A **loop**, and the one the [finalize](finalize.md) zero-contribution refusal routes a fresh clone into.",
};

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

/// **What `provision` leaves as it stands** — a registered worktree, reused; and the
/// leftover that is no worktree, which it clears before the add.
const A_REGISTERED_WORKTREE_REUSED: [Quote; 2] = [
    Quote {
        doc: "design/team-ready-state.md",
        heading: THE_RECORD_SETTLING_OP,
        words: "a **registered** worktree is reused untouched and never probed at all",
    },
    Quote {
        doc: "design/team-ready-state.md",
        heading: THE_RECORD_SETTLING_OP,
        words: "The three doors standing where **no commit has carried those bytes** refuse — `provision` (it clears a leftover before `git worktree add`), `discard` (this teardown), `uninstall` (it takes `.jigc/` and everything under it)",
    },
];

/// **What `provision` makes** — a worktree of the base's tracked files, registered.
const A_PROVISIONED_WORKTREE: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: "What a fan-out worktree isolates — and what it does not (M45)",
    words: "**A provisioned worktree carries tracked files only.** `git worktree add --detach` populates from the pinned base's *tree*",
};

/// The heading of `design/team-ready-state.md`'s milestone abandon.
const THE_RECORD_SETTLING_OP: &str = "`jigc milestone discard <id>` — the record-settling op";

/// The two lines of the re-seed, under the ids a row gives them: the milestone's cache in
/// its area, and the sub-tasks' areas — **each where none stood, of a sub-task the record
/// has not settled** (`<re-seeded>`, `byte_cell::RE_SEEDED`): an area that stands is
/// skipped untouched. **Narrowed 2026-10-19**, after the review of the task `B7`'s second
/// part (its second finding): the areas' line read `<sub-task>`, so a rewrite of a standing
/// sub-task's staged prose or base pin held at every door that re-seeds. The cache's half
/// keeps its place: its sentence re-derives the cache *"when absent or stale"*, and a
/// stale cache stands. **The areas' half holds on a refusal too** — read so 2026-10-19,
/// after the same review (its second finding's left-open, the seventh fix): *"all pass
/// through the reseed before their own guards"*, and the area it rebuilds is what the
/// zero-contribution refusal routes a fresh clone into (`THE_AREAS_SERVE_A_REFUSAL`), so a
/// door that refuses at its own guard leaves the area the re-seed made. The cache's half
/// needs no such word: a refusal's cache is `C3`'s.
const fn the_re_seed(cache: &'static str, areas: &'static str) -> [Line; 2] {
    [
        line(
            cache,
            Allows::Files(Where::Under(".jigc/milestones/<unit>"), May::Rebuilt),
            &[THE_CACHE_RE_DERIVED, EVERY_VERB_ONE_SITE],
        ),
        Line {
            on_refusal: true,
            ..line(
                areas,
                Allows::Files(Where::Under(".jigc/tasks/<re-seeded>"), May::Rebuilt),
                &THE_AREAS_RE_SEEDED,
            )
        },
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
        AFTER_A_COMMIT,
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
        AFTER_A_COMMIT,
    ],
};

/// The re-seed's two lines of `add-from-spec`.
const ADD_FROM_SPEC_RE_SEEDS: [Line; 2] = the_re_seed("MS2", "MS6");

/// **`jigc milestone add-from-spec`** — the record's §4b: `add-task`'s row, a sub-task a
/// criterion of the spec — so its areas are each sub-task's it mints (`<each-minted>`,
/// `byte_cell::EACH_MINTED`), where `add-task`'s is the one. Its home is `vendored`, the one
/// prepared state that holds a committed spec.
pub const ADD_FROM_SPEC: Row = Row {
    leaf: &["milestone", "add-from-spec"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Vendored),
    lines: &[
        // **Read 2026-10-19 as each sub-task the door mints**, after the review of the task
        // `B7`'s second part (its eighth finding): `<minted>` is now the one id the record
        // gains, and this door mints one a criterion.
        line(
            "MS1",
            Allows::Files(Where::Under(".jigc/tasks/<each-minted>"), May::Rebuilt),
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
        AFTER_A_COMMIT,
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
        // **Narrowed 2026-10-19**, after the review of the task `B7`'s second part (its
        // second finding): where no checkout stood (`<unprovisioned>`,
        // `byte_cell::UNPROVISIONED`) — the line read `<sub-task>`, so a rewrite inside a
        // live sub-task's worktree held.
        // **Narrowed 2026-10-19 again**, by the seventh fix (the sixth's left-open): what
        // the checkout holds is the pinned base's tree (`May::FromTheBasesTree`) — the line
        // read *rebuilt*, any bytes.
        line(
            "MP1",
            Allows::Files(
                Where::Under(".jigc/worktrees/<unprovisioned>"),
                May::FromTheBasesTree,
            ),
            &[
                THE_WORKTREES_DIRECTORY,
                A_PROVISIONED_WORKTREE,
                A_REGISTERED_WORKTREE_REUSED[0],
                A_REGISTERED_WORKTREE_REUSED[1],
            ],
        ),
        // At a place no checkout stood, of a sub-task the record has not settled — the
        // human's ruling of 2026-10-19 on the record's §6, entry 20, as `MP1`'s.
        line(
            "MP2",
            Allows::Registered(".jigc/worktrees/<unprovisioned>"),
            &[A_PROVISIONED_WORKTREE],
        ),
        PROVISION_RE_SEEDS[0],
        PROVISION_RE_SEEDS[1],
        ON_A_REFUSAL[0],
        ON_A_REFUSAL[1],
        ON_A_REFUSAL[2],
        ON_A_REFUSAL[3],
        AFTER_A_COMMIT,
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
        AFTER_A_COMMIT,
    ],
};

/// The re-seed's two lines of `join`.
const JOIN_RE_SEEDS: [Line; 2] = the_re_seed("MJ1", "MJ2");

/// **`jigc milestone join`** — the record's §4b: the re-seed, and the committed edge index
/// its ref walk reads, rebuilt where its file is absent or its stamp stale (`MJ4`); the
/// join is a preview and its overlay is never persisted (`MJ3`, no line).
pub const JOIN: Row = Row {
    leaf: &["milestone", "join"],
    unit: Unit::TheWordAfterTheLeaf,
    spellings: &[],
    consent: None,
    home: Prepared::Of(State::Fresh),
    lines: &[
        JOIN_RE_SEEDS[0],
        JOIN_RE_SEEDS[1],
        // **Added 2026-10-19**, after the review of the task `B7`'s second part (its fifth
        // finding): driven on both binaries, `join` at exit 0 writes `.jigc/index/edges.json`
        // where it was absent, and no line covered it — every cell's leads had built the
        // index first. *Rebuilt*, not *invalidated*: a read makes or rewrites the cache and
        // takes nothing of it away; the stamp's file gone is a finalize's alone (`M10`).
        line(
            "MJ4, the edge index",
            Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
            &[
                Quote {
                    doc: "design/storage.md",
                    heading: "The by-task-id join (M7)",
                    words: "the join runs the forward-ref check **once per sub-area**, resolving that area's outgoing edges against `committed ∪ that one area`",
                },
                Quote {
                    doc: "design/storage.md",
                    heading: "Edge index lifecycle",
                    words: "| **committed rebuild** | first read after a stamp mismatch (branch switch, pull, rebase) — rebuild from committed `.md`s | atomic; persists in `.jigc/index/` with the new stamp |",
                },
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

/// What `discard` removes, a line a place — the record's `MD4`.
const THE_WORKBENCH_REMOVED: Quote = Quote {
    doc: "design/team-ready-state.md",
    heading: THE_RECORD_SETTLING_OP,
    words: "**The workbench is actually removed.** `discard` removes the sub-task areas **and `.jigc/milestones/<id>/`**",
};

/// The re-seed's two lines of `discard` — `MD7` and `MD8`, added as `finalize`'s were.
const DISCARD_RE_SEEDS: [Line; 2] = the_re_seed("MD7", "MD8");

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
        DISCARD_RE_SEEDS[0],
        DISCARD_RE_SEEDS[1],
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
        AFTER_A_COMMIT,
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
        at: None,
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
        at: None,
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
        at: None,
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
/// join. `$before` stands before the milestone is created, after the knob's commit — where
/// a change staged meets the carryover gate — and `$after` after the join, last before the
/// command.
macro_rules! two_sub_tasks_unsquashed {
    ([$($before:expr),* $(,)?], [$($after:expr),* $(,)?]) => {
        &[
            jigc(&["config", "set", "finalize.fan-out.squash", "false"]),
            Lead::Step(Step::Git(&["add", ".jigc/config/manifest.yaml"])),
            Lead::Step(Step::Git(&["commit", "-q", "-m", "chore: squash false"])),
            $($before,)*
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
        at: None,
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
        two_sub_tasks_unsquashed!([], []),
        Listed::Acts,
        None,
        &[],
    ),
    unsquashed(
        "milestone-finalize-squash-false-a-file-staged-elsewhere",
        two_sub_tasks_unsquashed!(
            [],
            [Lead::Plant(Plant::Staged {
                at: "notes/staged-by-its-user.txt",
                kept: true,
            })]
        ),
        Listed::Acts,
        Some(Aim {
            line: "M4",
            kind: Kind::StagedElsewhere,
        }),
        &[],
    ),
    unsquashed(
        "milestone-finalize-squash-false-a-staged-file-whose-name-git-quotes",
        two_sub_tasks_unsquashed!(
            [],
            [Lead::Plant(Plant::Staged {
                at: A_NAME_GIT_QUOTES,
                kept: true,
            })]
        ),
        Listed::Acts,
        Some(Aim {
            line: "M4",
            kind: Kind::QuotedName,
        }),
        &[],
    ),
    unsquashed(
        "milestone-finalize-squash-false-a-hook-that-rejects",
        two_sub_tasks_unsquashed!([], [Lead::Plant(Plant::RejectingHook)]),
        refuses(1, "`git commit` was rejected"),
        Some(Aim {
            line: "M4",
            kind: Kind::RejectingHook,
        }),
        &[],
    ),
];

// ---------------------------------------------------------------------------
// The record doors' cells — `create`, `add-task`, `add-from-spec` (the first of the three
// parts the record's §4b names)
// ---------------------------------------------------------------------------

const FRESH: Prepared = Prepared::Of(State::Fresh);
const VENDORED: Prepared = Prepared::Of(State::Vendored);

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

/// The milestone the record doors' cells mint or continue: `ship-it`, whose title is its
/// own slug — the id the record's home and the milestone's area are named by.
const MINT_IT: Lead = jigc(&["milestone", "create", "ship-it"]);

/// Its committed record, and the directory the record stands in.
const ITS_RECORD: &str = "docs/milestone-records/ship-it.md";
const THE_RECORDS_DIRECTORY: &str = "docs/milestone-records";

/// A sub-task added before the cell's own — `do-a-first-thing`, whose area is the one the
/// re-seed's line names.
const A_FIRST_SUB_TASK: Lead = jigc(&["milestone", "add-task", "ship-it", "do a first thing"]);

const CREATE_IT: &[&str] = &["milestone", "create", "ship-it"];
const ADD_A_SUB_TASK: &[&str] = &["milestone", "add-task", "ship-it", "do the thing"];
const SEED_FROM_THE_SPEC: &[&str] = &["milestone", "add-from-spec", "ship-it", "spec:padding"];

/// A change of the user's staged at a place the command does not write, and one under a
/// name git quotes — kept in the working file both.
const STAGED_ELSEWHERE: Lead = Lead::Plant(Plant::Staged {
    at: "notes/staged-by-its-user.txt",
    kept: true,
});
const STAGED_UNDER_A_QUOTED_NAME: Lead = Lead::Plant(Plant::Staged {
    at: A_NAME_GIT_QUOTES,
    kept: true,
});

/// The words a refusal over a repository with no `jigc setup` says — every `jigc milestone`
/// door's, *"the shipped `locate::not_set_up()` answer"* (`design/team-ready-state.md` →
/// *Engine capability 2*); the words a run's.
const NOT_SET_UP: Listed = refuses(1, "this project isn't set up");

/// **The cells of `jigc milestone create`**, each listed from the row's doc:
///
/// - **the base cell mints `ship-it`** — its area, its record, one commit of the record
///   alone (`MC1` to `MC4`); the leaf as the catalog types it, `"A Milestone"`, resolves
///   nothing and mints too, so its refusing cell acts;
/// - **a file of the user's in the milestone's area** (`MC1`) is a leftover the mint
///   refuses over — *"A **residual** at the area path … takes the task mint's split above:
///   same code, same key"* (`design/write-commands.md` → *Minting a milestone*);
/// - **at the record's home** (`MC2`), a file of the user's, one the user committed, a
///   change the user staged and a record that a discarded milestone's left and the user
///   edited each own the id — *"`create` refuses when a record already owns the slug
///   (`milestone.record-exists`)"* — and a link or a directory there is no regular file:
///   *"any entry that is not a regular file — a link, a directory — refuses — under
///   `store.home-not-regular-file`"* (`design/team-ready-state.md` → *The terminal is
///   terminal*); a file in the place of the record's directory leaves the record nowhere
///   to be made, and no doc names the refusal — the words are a run's;
/// - **what is staged elsewhere stays out of the record's commit** (`MC3`: *"never
///   sweeping the agent's in-flight index/worktree WIP"*), and **a hook that rejects** the
///   commit leaves no residue (`MC5`);
/// - **a file of the user's beside the baseline** (`MC4`) stays.
const CREATE_CELLS: [Cell; 22] = [
    cell(
        "milestone-create-a-milestone-the-catalog-names",
        &["milestone", "create"],
        FRESH,
        &[],
        &["milestone", "create", "A Milestone"],
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-create-ship-it",
        &["milestone", "create"],
        FRESH,
        &[],
        CREATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-create-a-file-of-the-users-in-the-milestones-area",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::File(
            ".jigc/milestones/ship-it/notes.txt",
        ))],
        CREATE_IT,
        refuses(1, "milestone.serial-collision"),
        at("MC1", Kind::Untracked),
    ),
    cell(
        "milestone-create-a-file-of-the-users-at-the-records-home",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::File(ITS_RECORD))],
        CREATE_IT,
        refuses(1, "milestone.record-exists"),
        at("MC2", Kind::Untracked),
    ),
    cell(
        "milestone-create-a-file-the-user-committed-at-the-records-home",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::Committed(ITS_RECORD))],
        CREATE_IT,
        refuses(1, "milestone.record-exists"),
        at("MC2", Kind::Committed),
    ),
    cell(
        "milestone-create-a-link-to-nothing-at-the-records-home",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::LinkToNothing(ITS_RECORD))],
        CREATE_IT,
        refuses(1, "store.home-not-regular-file"),
        at("MC2", Kind::LinkToNothing),
    ),
    cell(
        "milestone-create-a-link-to-a-file-in-the-repository-at-the-records-home",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::LinkToAFile {
            at: ITS_RECORD,
            outside: false,
        })],
        CREATE_IT,
        refuses(1, "store.home-not-regular-file"),
        at("MC2", Kind::LinkInside),
    ),
    cell(
        "milestone-create-a-link-to-a-file-outside-at-the-records-home",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::LinkToAFile {
            at: ITS_RECORD,
            outside: true,
        })],
        CREATE_IT,
        refuses(1, "store.home-not-regular-file"),
        at("MC2", Kind::LinkOutside),
    ),
    cell(
        "milestone-create-a-directory-at-the-records-home",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::Dir(ITS_RECORD))],
        CREATE_IT,
        refuses(1, "store.home-not-regular-file"),
        at("MC2", Kind::DirForAFile),
    ),
    cell(
        "milestone-create-a-file-in-the-place-of-the-records-directory",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::File(THE_RECORDS_DIRECTORY))],
        CREATE_IT,
        refuses(1, "could not create the record home"),
        at("MC2", Kind::FileForADir),
    ),
    cell(
        "milestone-create-over-a-discarded-milestones-record-edited-by-hand",
        &["milestone", "create"],
        FRESH,
        &[
            MINT_IT,
            jigc(&["milestone", "discard", "ship-it"]),
            Lead::Plant(Plant::Edit(ITS_RECORD)),
        ],
        CREATE_IT,
        refuses(1, "milestone.record-exists"),
        at("MC2", Kind::EditAfter),
    ),
    cell(
        "milestone-create-a-change-staged-at-the-records-home",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::Staged {
            at: ITS_RECORD,
            kept: false,
        })],
        CREATE_IT,
        refuses(1, "milestone.record-exists"),
        at("MC2", Kind::StagedThere),
    ),
    cell(
        "milestone-create-a-file-staged-elsewhere",
        &["milestone", "create"],
        FRESH,
        &[STAGED_ELSEWHERE],
        CREATE_IT,
        Listed::Acts,
        at("MC3", Kind::StagedElsewhere),
    ),
    cell(
        "milestone-create-a-staged-file-whose-name-git-quotes",
        &["milestone", "create"],
        FRESH,
        &[STAGED_UNDER_A_QUOTED_NAME],
        CREATE_IT,
        Listed::Acts,
        at("MC3", Kind::QuotedName),
    ),
    cell(
        "milestone-create-a-hook-that-rejects",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::RejectingHook)],
        CREATE_IT,
        refuses(1, "`git commit` was rejected"),
        at("MC3", Kind::RejectingHook),
    ),
    cell(
        "milestone-create-a-file-of-the-users-beside-the-baseline",
        &["milestone", "create"],
        FRESH,
        &[Lead::Plant(Plant::File(".jigc/state/notes.md"))],
        CREATE_IT,
        Listed::Acts,
        at("MC4", Kind::Untracked),
    ),
    // The base cell in the never-adopted repository: the command as typed, with no lead.
    cell(
        "milestone-create-ship-it-in-bare",
        &["milestone", "create"],
        Prepared::Bare,
        &[],
        CREATE_IT,
        NOT_SET_UP,
        None,
    ),
    cell(
        "milestone-create-ship-it-in-committed-singletons",
        &["milestone", "create"],
        Prepared::Of(State::CommittedSingletons),
        &[],
        CREATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-create-ship-it-in-migrated",
        &["milestone", "create"],
        Prepared::Of(State::Migrated),
        &[],
        CREATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-create-ship-it-in-refs-post-hoc",
        &["milestone", "create"],
        Prepared::Of(State::RefsPostHoc),
        &[],
        CREATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-create-ship-it-in-chatty-hooks",
        &["milestone", "create"],
        Prepared::Of(State::ChattyHooks),
        &[],
        CREATE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-create-ship-it-in-vendored",
        &["milestone", "create"],
        VENDORED,
        &[],
        CREATE_IT,
        Listed::Acts,
        None,
    ),
];

/// **The cells of `jigc milestone add-task`**, each listed from the row's doc:
///
/// - **the base cell adds `do-the-thing` to `ship-it`** — its area, the task list, one item
///   appended to the record, one commit of the record alone (`MA1` to `MA5`); the leaf as
///   the catalog types it names a milestone that is not there and refuses;
/// - **a file of the user's in the sub-task's area** (`MA1`) is a leftover the mint
///   refuses over, as `create`'s; one **in the milestone's area** (`MA2`) is in an area the
///   append writes into and no doc refuses over — it stays;
/// - **at the record** (`MA3`), a link or a directory in its place is no regular file —
///   *"an entry that is not a regular file refuses under `store.home-not-regular-file`"*;
///   an edit to it, a line of the user's in it, a change staged at it and the record
///   deleted each make the record one *"git calls modified"*, which *"conflict-blocks at
///   **every door that rewrites the record**"* (`design/team-ready-state.md` → *Engine
///   capability 1*); and with the record's directory gone there is no record to append
///   to — refused, the words a run's;
/// - **what is staged elsewhere stays out of the record's commit** (`MA4`), and **a hook
///   that rejects** it leaves no residue — the area unwound and the task list restored
///   (`MC5`);
/// - **a file of the user's beside the baseline** (`MA5`), and **one in the area of a
///   sub-task added before** (`MA6`: *"A sub-task whose area already exists is skipped
///   untouched"*), stay.
const ADD_TASK_CELLS: [Cell; 24] = [
    cell(
        "milestone-add-task-to-a-milestone-that-is-not-there",
        &["milestone", "add-task"],
        FRESH,
        &[],
        &["milestone", "add-task", "m-a-milestone", "do the thing"],
        refuses(1, "milestone.unknown"),
        None,
    ),
    cell(
        "milestone-add-task-do-the-thing",
        &["milestone", "add-task"],
        FRESH,
        &[MINT_IT],
        ADD_A_SUB_TASK,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-add-task-a-file-of-the-users-in-the-sub-tasks-area",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            Lead::Plant(Plant::File(".jigc/tasks/do-the-thing/notes.txt")),
        ],
        ADD_A_SUB_TASK,
        refuses(1, "task.serial-collision"),
        at("MA1", Kind::Untracked),
    ),
    cell(
        "milestone-add-task-a-file-of-the-users-in-the-milestones-area",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            Lead::Plant(Plant::File(".jigc/milestones/ship-it/notes.txt")),
        ],
        ADD_A_SUB_TASK,
        Listed::Acts,
        at("MA2", Kind::Untracked),
    ),
    cell(
        "milestone-add-task-a-link-to-nothing-in-the-records-place",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(ITS_RECORD))),
        ],
        ADD_A_SUB_TASK,
        refuses(1, "store.home-not-regular-file"),
        at("MA3", Kind::LinkToNothing),
    ),
    cell(
        "milestone-add-task-a-link-to-a-file-in-the-repository-in-the-records-place",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: ITS_RECORD,
                outside: false,
            })),
        ],
        ADD_A_SUB_TASK,
        refuses(1, "store.home-not-regular-file"),
        at("MA3", Kind::LinkInside),
    ),
    cell(
        "milestone-add-task-a-link-to-a-file-outside-in-the-records-place",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: ITS_RECORD,
                outside: true,
            })),
        ],
        ADD_A_SUB_TASK,
        refuses(1, "store.home-not-regular-file"),
        at("MA3", Kind::LinkOutside),
    ),
    cell(
        "milestone-add-task-a-directory-in-the-records-place",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            Lead::Plant(Plant::InPlaceOf(&Plant::Dir(ITS_RECORD))),
        ],
        ADD_A_SUB_TASK,
        refuses(1, "store.home-not-regular-file"),
        at("MA3", Kind::DirForAFile),
    ),
    cell(
        "milestone-add-task-a-file-in-the-place-of-the-records-directory",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            Lead::MovedAway("repo/docs/milestone-records"),
            Lead::Plant(Plant::File(THE_RECORDS_DIRECTORY)),
        ],
        ADD_A_SUB_TASK,
        refuses(1, ITS_RECORD),
        at("MA3", Kind::FileForADir),
    ),
    cell(
        "milestone-add-task-a-hand-edit-to-the-record",
        &["milestone", "add-task"],
        FRESH,
        &[MINT_IT, Lead::Plant(Plant::Edit(ITS_RECORD))],
        ADD_A_SUB_TASK,
        refuses(1, "reconciliation.conflict-block"),
        at("MA3", Kind::EditAfter),
    ),
    cell(
        "milestone-add-task-the-record-deleted",
        &["milestone", "add-task"],
        FRESH,
        &[MINT_IT, Lead::Plant(Plant::Deleted(ITS_RECORD))],
        ADD_A_SUB_TASK,
        refuses(1, ITS_RECORD),
        at("MA3", Kind::EditAfter),
    ),
    cell(
        "milestone-add-task-a-change-staged-at-the-record",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            Lead::Plant(Plant::Staged {
                at: ITS_RECORD,
                kept: false,
            }),
        ],
        ADD_A_SUB_TASK,
        refuses(1, "reconciliation.conflict-block"),
        at("MA3", Kind::StagedThere),
    ),
    cell(
        "milestone-add-task-a-line-of-the-users-in-the-record",
        &["milestone", "add-task"],
        FRESH,
        &[MINT_IT, Lead::Plant(Plant::UsersLine(ITS_RECORD))],
        ADD_A_SUB_TASK,
        refuses(1, "reconciliation.conflict-block"),
        at("MA3", Kind::UsersLine),
    ),
    cell(
        "milestone-add-task-a-file-staged-elsewhere",
        &["milestone", "add-task"],
        FRESH,
        &[MINT_IT, STAGED_ELSEWHERE],
        ADD_A_SUB_TASK,
        Listed::Acts,
        at("MA4", Kind::StagedElsewhere),
    ),
    cell(
        "milestone-add-task-a-staged-file-whose-name-git-quotes",
        &["milestone", "add-task"],
        FRESH,
        &[MINT_IT, STAGED_UNDER_A_QUOTED_NAME],
        ADD_A_SUB_TASK,
        Listed::Acts,
        at("MA4", Kind::QuotedName),
    ),
    cell(
        "milestone-add-task-a-hook-that-rejects",
        &["milestone", "add-task"],
        FRESH,
        &[MINT_IT, Lead::Plant(Plant::RejectingHook)],
        ADD_A_SUB_TASK,
        refuses(1, "`git commit` was rejected"),
        at("MA4", Kind::RejectingHook),
    ),
    cell(
        "milestone-add-task-a-file-of-the-users-beside-the-baseline",
        &["milestone", "add-task"],
        FRESH,
        &[MINT_IT, Lead::Plant(Plant::File(".jigc/state/notes.md"))],
        ADD_A_SUB_TASK,
        Listed::Acts,
        at("MA5", Kind::Untracked),
    ),
    cell(
        "milestone-add-task-a-file-of-the-users-in-a-first-sub-tasks-area",
        &["milestone", "add-task"],
        FRESH,
        &[
            MINT_IT,
            A_FIRST_SUB_TASK,
            Lead::Plant(Plant::File(".jigc/tasks/do-a-first-thing/notes.txt")),
        ],
        ADD_A_SUB_TASK,
        Listed::Acts,
        at("MA6", Kind::Untracked),
    ),
    // The base cell in the never-adopted repository: the command as typed, with no lead.
    cell(
        "milestone-add-task-do-the-thing-in-bare",
        &["milestone", "add-task"],
        Prepared::Bare,
        &[],
        ADD_A_SUB_TASK,
        NOT_SET_UP,
        None,
    ),
    cell(
        "milestone-add-task-do-the-thing-in-committed-singletons",
        &["milestone", "add-task"],
        Prepared::Of(State::CommittedSingletons),
        &[MINT_IT],
        ADD_A_SUB_TASK,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-add-task-do-the-thing-in-migrated",
        &["milestone", "add-task"],
        Prepared::Of(State::Migrated),
        &[MINT_IT],
        ADD_A_SUB_TASK,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-add-task-do-the-thing-in-refs-post-hoc",
        &["milestone", "add-task"],
        Prepared::Of(State::RefsPostHoc),
        &[MINT_IT],
        ADD_A_SUB_TASK,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-add-task-do-the-thing-in-chatty-hooks",
        &["milestone", "add-task"],
        Prepared::Of(State::ChattyHooks),
        &[MINT_IT],
        ADD_A_SUB_TASK,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-add-task-do-the-thing-in-vendored",
        &["milestone", "add-task"],
        VENDORED,
        &[MINT_IT],
        ADD_A_SUB_TASK,
        Listed::Acts,
        None,
    ),
];

/// **`spec:padding` with a second criterion, committed, and `ship-it` minted** — a task
/// adds the criterion `keeps-the-width` and its statement and lands it, so the spec
/// `add-from-spec` reads holds two: the door then mints two sub-tasks and *"commits the
/// record once per sub-task"*, the first commit holding the record partway (`MS3`, `MS4`).
const A_SECOND_CRITERION: [Lead; 9] = [
    payload("statement", "Padding keeps the width it is given.\n"),
    payload("summary", "add a criterion\n"),
    jigc(&["start", "--workflow", "single-task", "add a criterion"]),
    jigc(&[
        "doc",
        "add-item",
        "spec:padding#criteria",
        "--title",
        "Keeps the width",
        "--task",
        "add-a-criterion",
    ]),
    jigc(&[
        "doc",
        "set-slot",
        "spec:padding#criteria/keeps-the-width/statement",
        "--from-file",
        "../outside/statement",
        "--task",
        "add-a-criterion",
    ]),
    jigc(&[
        "doc",
        "set-field",
        "commit:add-a-criterion#type",
        "--value",
        "docs",
        "--task",
        "add-a-criterion",
    ]),
    jigc(&[
        "doc",
        "set-slot",
        "commit:add-a-criterion#summary",
        "--from-file",
        "../outside/summary",
        "--task",
        "add-a-criterion",
    ]),
    jigc(&["task", "finalize", "add-a-criterion"]),
    MINT_IT,
];

/// One cell of `jigc milestone add-from-spec` in `vendored`, its home, over `ship-it`
/// minted first: `$after` stands after the mint, last before the command.
macro_rules! seeds {
    ($name:literal, [$($after:expr),* $(,)?], $listed:expr, $aim:expr) => {
        cell(
            $name,
            &["milestone", "add-from-spec"],
            VENDORED,
            &[MINT_IT, $($after,)*],
            SEED_FROM_THE_SPEC,
            $listed,
            $aim,
        )
    };
}

/// **The cells of `jigc milestone add-from-spec`** — `add-task`'s, a sub-task a criterion of
/// `vendored`'s committed spec `spec:padding`, whose one criterion mints `pads-the-input`;
/// each listed as `add-task`'s is. In a prepared state with no spec the base cell refuses —
/// the spec is not there, which no doc of the door's words; the words are a run's.
const ADD_FROM_SPEC_CELLS: [Cell; 24] = [
    cell(
        "milestone-add-from-spec-to-a-milestone-that-is-not-there",
        &["milestone", "add-from-spec"],
        FRESH,
        &[],
        &["milestone", "add-from-spec", "m-a-milestone", "spec:a-spec"],
        refuses(1, "milestone.unknown"),
        None,
    ),
    seeds!("milestone-add-from-spec-padding", [], Listed::Acts, None),
    cell(
        "milestone-add-from-spec-a-spec-of-two-criteria",
        &["milestone", "add-from-spec"],
        VENDORED,
        &A_SECOND_CRITERION,
        SEED_FROM_THE_SPEC,
        Listed::Acts,
        None,
    ),
    seeds!(
        "milestone-add-from-spec-a-file-of-the-users-in-the-sub-tasks-area",
        [Lead::Plant(Plant::File(
            ".jigc/tasks/pads-the-input/notes.txt"
        ))],
        refuses(1, "task.serial-collision"),
        at("MS1", Kind::Untracked)
    ),
    seeds!(
        "milestone-add-from-spec-a-file-of-the-users-in-the-milestones-area",
        [Lead::Plant(Plant::File(
            ".jigc/milestones/ship-it/notes.txt"
        ))],
        Listed::Acts,
        at("MS2", Kind::Untracked)
    ),
    seeds!(
        "milestone-add-from-spec-a-link-to-nothing-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            ITS_RECORD
        )))],
        refuses(1, "store.home-not-regular-file"),
        at("MS3", Kind::LinkToNothing)
    ),
    seeds!(
        "milestone-add-from-spec-a-link-to-a-file-in-the-repository-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ITS_RECORD,
            outside: false,
        }))],
        refuses(1, "store.home-not-regular-file"),
        at("MS3", Kind::LinkInside)
    ),
    seeds!(
        "milestone-add-from-spec-a-link-to-a-file-outside-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ITS_RECORD,
            outside: true,
        }))],
        refuses(1, "store.home-not-regular-file"),
        at("MS3", Kind::LinkOutside)
    ),
    seeds!(
        "milestone-add-from-spec-a-directory-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::Dir(ITS_RECORD)))],
        refuses(1, "store.home-not-regular-file"),
        at("MS3", Kind::DirForAFile)
    ),
    seeds!(
        "milestone-add-from-spec-a-file-in-the-place-of-the-records-directory",
        [
            Lead::MovedAway("repo/docs/milestone-records"),
            Lead::Plant(Plant::File(THE_RECORDS_DIRECTORY)),
        ],
        refuses(1, ITS_RECORD),
        at("MS3", Kind::FileForADir)
    ),
    seeds!(
        "milestone-add-from-spec-a-hand-edit-to-the-record",
        [Lead::Plant(Plant::Edit(ITS_RECORD))],
        refuses(1, "reconciliation.conflict-block"),
        at("MS3", Kind::EditAfter)
    ),
    seeds!(
        "milestone-add-from-spec-a-change-staged-at-the-record",
        [Lead::Plant(Plant::Staged {
            at: ITS_RECORD,
            kept: false,
        })],
        refuses(1, "reconciliation.conflict-block"),
        at("MS3", Kind::StagedThere)
    ),
    seeds!(
        "milestone-add-from-spec-a-line-of-the-users-in-the-record",
        [Lead::Plant(Plant::UsersLine(ITS_RECORD))],
        refuses(1, "reconciliation.conflict-block"),
        at("MS3", Kind::UsersLine)
    ),
    seeds!(
        "milestone-add-from-spec-a-file-staged-elsewhere",
        [STAGED_ELSEWHERE],
        Listed::Acts,
        at("MS4", Kind::StagedElsewhere)
    ),
    seeds!(
        "milestone-add-from-spec-a-staged-file-whose-name-git-quotes",
        [STAGED_UNDER_A_QUOTED_NAME],
        Listed::Acts,
        at("MS4", Kind::QuotedName)
    ),
    seeds!(
        "milestone-add-from-spec-a-hook-that-rejects",
        [Lead::Plant(Plant::RejectingHook)],
        refuses(1, "`git commit` was rejected"),
        at("MS4", Kind::RejectingHook)
    ),
    seeds!(
        "milestone-add-from-spec-a-file-of-the-users-beside-the-baseline",
        [Lead::Plant(Plant::File(".jigc/state/notes.md"))],
        Listed::Acts,
        at("MS5", Kind::Untracked)
    ),
    seeds!(
        "milestone-add-from-spec-a-file-of-the-users-in-a-first-sub-tasks-area",
        [
            A_FIRST_SUB_TASK,
            Lead::Plant(Plant::File(".jigc/tasks/do-a-first-thing/notes.txt")),
        ],
        Listed::Acts,
        at("MS6", Kind::Untracked)
    ),
    // The base cell in the never-adopted repository: the command as typed, with no lead;
    // in the others, the spec is not there.
    cell(
        "milestone-add-from-spec-padding-in-bare",
        &["milestone", "add-from-spec"],
        Prepared::Bare,
        &[],
        SEED_FROM_THE_SPEC,
        NOT_SET_UP,
        None,
    ),
    cell(
        "milestone-add-from-spec-padding-in-fresh",
        &["milestone", "add-from-spec"],
        FRESH,
        &[MINT_IT],
        SEED_FROM_THE_SPEC,
        refuses(1, "store.not-found"),
        None,
    ),
    cell(
        "milestone-add-from-spec-padding-in-committed-singletons",
        &["milestone", "add-from-spec"],
        Prepared::Of(State::CommittedSingletons),
        &[MINT_IT],
        SEED_FROM_THE_SPEC,
        refuses(1, "store.not-found"),
        None,
    ),
    cell(
        "milestone-add-from-spec-padding-in-migrated",
        &["milestone", "add-from-spec"],
        Prepared::Of(State::Migrated),
        &[MINT_IT],
        SEED_FROM_THE_SPEC,
        refuses(1, "store.not-found"),
        None,
    ),
    cell(
        "milestone-add-from-spec-padding-in-refs-post-hoc",
        &["milestone", "add-from-spec"],
        Prepared::Of(State::RefsPostHoc),
        &[MINT_IT],
        SEED_FROM_THE_SPEC,
        refuses(1, "store.not-found"),
        None,
    ),
    cell(
        "milestone-add-from-spec-padding-in-chatty-hooks",
        &["milestone", "add-from-spec"],
        Prepared::Of(State::ChattyHooks),
        &[MINT_IT],
        SEED_FROM_THE_SPEC,
        refuses(1, "store.not-found"),
        None,
    ),
];

// ---------------------------------------------------------------------------
// The fan-out doors' cells — `provision`, `execute`, `join` (the second of the three parts
// the record's §4b names)
// ---------------------------------------------------------------------------

/// **`ship-it` and two sub-tasks nobody has re-entered** — `add-alpha` and `add-beta`.
const TWO_SUB_TASKS: [Lead; 3] = [
    MINT_IT,
    jigc(&["milestone", "add-task", "ship-it", "add alpha"]),
    jigc(&["milestone", "add-task", "ship-it", "add beta"]),
];

/// The same, provisioned: a worktree each.
const PROVISIONED: Lead = jigc(&["milestone", "provision", "ship-it"]);

/// A sub-task re-entered from its own worktree: its area provisioned with its commit doc,
/// which the join then merges.
macro_rules! re_entered {
    ($task:literal) => {
        Lead::Step(Step::JigcAt(
            Place::Repo(concat!(".jigc/worktrees/", $task)),
            &["workflow", "sub-task", "--task", $task],
        ))
    };
}

const PROVISION_IT: &[&str] = &["milestone", "provision", "ship-it"];
const EXECUTE_IT: &[&str] = &["milestone", "execute", "ship-it"];
const JOIN_IT: &[&str] = &["milestone", "join", "ship-it"];

/// The base cell of a fan-out door in every prepared state but its home: in the
/// never-adopted repository the command as typed, with no lead, which every `milestone`
/// door refuses; in the others the same leads as the home's base cell.
macro_rules! fan_out_elsewhere {
    ($leaf:literal, $unit:literal, $argv:expr, [$($lead:expr),* $(,)?]) => {
        [
            cell(
                concat!("milestone-", $leaf, "-", $unit, "-in-bare"),
                &["milestone", $leaf],
                Prepared::Bare,
                &[],
                $argv,
                NOT_SET_UP,
                None,
            ),
            cell(
                concat!("milestone-", $leaf, "-", $unit, "-in-committed-singletons"),
                &["milestone", $leaf],
                Prepared::Of(State::CommittedSingletons),
                &[$($lead,)*],
                $argv,
                Listed::Acts,
                None,
            ),
            cell(
                concat!("milestone-", $leaf, "-", $unit, "-in-migrated"),
                &["milestone", $leaf],
                Prepared::Of(State::Migrated),
                &[$($lead,)*],
                $argv,
                Listed::Acts,
                None,
            ),
            cell(
                concat!("milestone-", $leaf, "-", $unit, "-in-refs-post-hoc"),
                &["milestone", $leaf],
                Prepared::Of(State::RefsPostHoc),
                &[$($lead,)*],
                $argv,
                Listed::Acts,
                None,
            ),
            cell(
                concat!("milestone-", $leaf, "-", $unit, "-in-chatty-hooks"),
                &["milestone", $leaf],
                Prepared::Of(State::ChattyHooks),
                &[$($lead,)*],
                $argv,
                Listed::Acts,
                None,
            ),
            cell(
                concat!("milestone-", $leaf, "-", $unit, "-in-vendored"),
                &["milestone", $leaf],
                VENDORED,
                &[$($lead,)*],
                $argv,
                Listed::Acts,
                None,
            ),
        ]
    };
}

/// **The cells of `jigc milestone provision`**, each listed from the row's doc: the base
/// cell provisions a worktree a sub-task and registers it (`MP1`, `MP2`); **a file of the
/// user's where a sub-task's worktree goes** (`MP1`) is a leftover the door *"cannot prove
/// is disposable"* and refuses over — `provision` *"clears a leftover before `git worktree
/// add`"* among *"the three doors standing where **no commit has carried those bytes**"*
/// (`MP5`); a file of the user's in the milestone's area (`MP3`) or a sub-task's (`MP4`)
/// is where the re-seed writes nothing over an area that stands — it stays. **A sub-task
/// the record has settled** gets no worktree (`MP1`, `MP2`, the human's ruling of
/// 2026-10-19 on the record's §6, entry 20: `provision` skips a sub-task the record has
/// settled, as the re-seed does).
const PROVISION_CELLS: [Cell; 6] = [
    cell(
        "milestone-provision-a-milestone-that-is-not-there",
        &["milestone", "provision"],
        FRESH,
        &[],
        &["milestone", "provision", "m-a-milestone"],
        refuses(1, "milestone.unknown"),
        None,
    ),
    cell(
        "milestone-provision-two-sub-tasks",
        &["milestone", "provision"],
        FRESH,
        &TWO_SUB_TASKS,
        PROVISION_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-provision-a-file-of-the-users-where-a-worktree-goes",
        &["milestone", "provision"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            Lead::Plant(Plant::File(".jigc/worktrees/add-alpha/notes.txt")),
        ],
        PROVISION_IT,
        refuses(1, "milestone.leftover-holds-work"),
        at("MP1", Kind::Untracked),
    ),
    cell(
        "milestone-provision-a-file-of-the-users-in-the-milestones-area",
        &["milestone", "provision"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            Lead::Plant(Plant::File(".jigc/milestones/ship-it/notes.txt")),
        ],
        PROVISION_IT,
        Listed::Acts,
        at("MP3", Kind::Untracked),
    ),
    cell(
        "milestone-provision-a-file-of-the-users-in-a-sub-tasks-area",
        &["milestone", "provision"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            Lead::Plant(Plant::File(".jigc/tasks/add-alpha/notes.txt")),
        ],
        PROVISION_IT,
        Listed::Acts,
        at("MP4", Kind::Untracked),
    ),
    cell(
        "milestone-provision-a-sub-task-the-record-has-settled",
        &["milestone", "provision"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            jigc(&["task", "discard", "add-beta"]),
        ],
        PROVISION_IT,
        Listed::Acts,
        None,
    ),
];

const PROVISION_ELSEWHERE: [Cell; 6] = fan_out_elsewhere!(
    "provision",
    "ship-it",
    PROVISION_IT,
    [TWO_SUB_TASKS[0], TWO_SUB_TASKS[1], TWO_SUB_TASKS[2]]
);

/// **The cells of `jigc milestone execute`**: the base cell composes the walk over a
/// provisioned milestone and mints nothing (`ME3`); a file of the user's in the milestone's
/// area (`ME1`) or a sub-task's (`ME2`) stays; **the cache's task list gone** is re-derived
/// (`ME1`), and **a sub-task's area gone** is rebuilt where none stood (`ME2`) — the re-seed
/// of what was absent, which no cell built before.
const EXECUTE_CELLS: [Cell; 6] = [
    cell(
        "milestone-execute-a-milestone-that-is-not-there",
        &["milestone", "execute"],
        FRESH,
        &[],
        &["milestone", "execute", "m-a-milestone"],
        refuses(1, "milestone.unknown"),
        None,
    ),
    cell(
        "milestone-execute-two-provisioned-sub-tasks",
        &["milestone", "execute"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
        ],
        EXECUTE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-execute-a-file-of-the-users-in-the-milestones-area",
        &["milestone", "execute"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            Lead::Plant(Plant::File(".jigc/milestones/ship-it/notes.txt")),
        ],
        EXECUTE_IT,
        Listed::Acts,
        at("ME1", Kind::Untracked),
    ),
    cell(
        "milestone-execute-a-file-of-the-users-in-a-sub-tasks-area",
        &["milestone", "execute"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            Lead::Plant(Plant::File(".jigc/tasks/add-alpha/notes.txt")),
        ],
        EXECUTE_IT,
        Listed::Acts,
        at("ME2", Kind::Untracked),
    ),
    cell(
        "milestone-execute-the-caches-task-list-gone",
        &["milestone", "execute"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            Lead::MovedAway("repo/.jigc/milestones/ship-it/tasks.json"),
        ],
        EXECUTE_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-execute-a-sub-tasks-area-gone",
        &["milestone", "execute"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            Lead::MovedAway("repo/.jigc/tasks/add-alpha"),
        ],
        EXECUTE_IT,
        Listed::Acts,
        None,
    ),
];

const EXECUTE_ELSEWHERE: [Cell; 6] = fan_out_elsewhere!(
    "execute",
    "ship-it",
    EXECUTE_IT,
    [
        TWO_SUB_TASKS[0],
        TWO_SUB_TASKS[1],
        TWO_SUB_TASKS[2],
        PROVISIONED
    ]
);

/// **The cells of `jigc milestone join`**: the base cell merges the two sub-tasks' commit
/// docs, each re-entered from its worktree, and persists nothing (`MJ3`); a file of the
/// user's in the milestone's area (`MJ1`) or a sub-task's (`MJ2`) stays; the edge index's
/// file gone, the join's read rebuilds it, and a file of the user's beside it stays
/// (`MJ4`).
const JOIN_CELLS: [Cell; 6] = [
    cell(
        "milestone-join-a-milestone-that-is-not-there",
        &["milestone", "join"],
        FRESH,
        &[],
        &["milestone", "join", "m-a-milestone"],
        refuses(1, "milestone.unknown"),
        None,
    ),
    cell(
        "milestone-join-two-re-entered-sub-tasks",
        &["milestone", "join"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            re_entered!("add-alpha"),
            re_entered!("add-beta"),
        ],
        JOIN_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-join-a-file-of-the-users-in-the-milestones-area",
        &["milestone", "join"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            re_entered!("add-alpha"),
            re_entered!("add-beta"),
            Lead::Plant(Plant::File(".jigc/milestones/ship-it/notes.txt")),
        ],
        JOIN_IT,
        Listed::Acts,
        at("MJ1", Kind::Untracked),
    ),
    cell(
        "milestone-join-a-file-of-the-users-in-a-sub-tasks-area",
        &["milestone", "join"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            re_entered!("add-alpha"),
            re_entered!("add-beta"),
            Lead::Plant(Plant::File(".jigc/tasks/add-alpha/notes.txt")),
        ],
        JOIN_IT,
        Listed::Acts,
        at("MJ2", Kind::Untracked),
    ),
    // **The edge index's file gone before the join** — moved away, as a user's `rm` of a
    // cache leaves it: the join's ref walk reads the committed index, and that read rebuilds
    // what is absent (`MJ4`). Every other cell of the door re-enters its sub-tasks first,
    // which builds the index, so none met the rebuild (the review of the task `B7`'s second
    // part, its fifth finding).
    cell(
        "milestone-join-the-edge-index-gone",
        &["milestone", "join"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            re_entered!("add-alpha"),
            re_entered!("add-beta"),
            Lead::MovedAway("repo/.jigc/index/edges.json"),
        ],
        JOIN_IT,
        Listed::Acts,
        None,
    ),
    cell(
        "milestone-join-a-file-of-the-users-under-the-edge-index",
        &["milestone", "join"],
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            PROVISIONED,
            re_entered!("add-alpha"),
            re_entered!("add-beta"),
            Lead::Plant(Plant::File(".jigc/index/notes.md")),
        ],
        JOIN_IT,
        Listed::Acts,
        at("MJ4, the edge index", Kind::Untracked),
    ),
];

const JOIN_ELSEWHERE: [Cell; 6] = fan_out_elsewhere!(
    "join",
    "ship-it",
    JOIN_IT,
    [
        TWO_SUB_TASKS[0],
        TWO_SUB_TASKS[1],
        TWO_SUB_TASKS[2],
        PROVISIONED,
        re_entered!("add-alpha"),
        re_entered!("add-beta"),
    ]
);

// ---------------------------------------------------------------------------
// The terminals' cells — `discard` and `finalize`'s plain spelling (the third of the three
// parts the record's §4b names)
// ---------------------------------------------------------------------------

const DISCARD_IT: &[&str] = &["milestone", "discard", "ship-it"];
const DISCARD_IT_BY_CONSENT: &[&str] = &["milestone", "discard", "ship-it", "--force"];

/// One cell of `jigc milestone discard` in `fresh`, over `ship-it`'s two sub-tasks
/// provisioned: `$after` stands after the provision, last before the command.
macro_rules! discards {
    ($name:literal, [$($after:expr),* $(,)?], $argv:expr, $listed:expr, $aim:expr) => {
        cell(
            $name,
            &["milestone", "discard"],
            FRESH,
            &[
                TWO_SUB_TASKS[0],
                TWO_SUB_TASKS[1],
                TWO_SUB_TASKS[2],
                PROVISIONED,
                $($after,)*
            ],
            $argv,
            $listed,
            $aim,
        )
    };
}

/// **The cells of `jigc milestone discard`**, each listed from the row's doc:
///
/// - **the base cell abandons `ship-it`** — two sub-tasks nobody re-entered, their
///   worktrees clean: the record settled, one commit of it alone, the workbench removed
///   and its registrations with it (`MD1` to `MD4`, `MD6`);
/// - **at the record** (`MD1`), what stands in its place, an edit, a change staged at it
///   and the record deleted are each refused as at `add-task` — *"a record git calls
///   modified conflict-blocks at **every door that rewrites the record**"* and *"an entry
///   that is not a regular file refuses under `store.home-not-regular-file`"*;
/// - **what is staged elsewhere stays out of the record's commit** (`MD2`), and **a hook
///   that rejects** it leaves the record and the workbench as they were;
/// - **a file of the user's beside the baseline** (`MD3`) stays;
/// - **a file jigc did not write in the milestone's area or a sub-task's** (`MD4`) is a
///   byte the abandon refuses over — *"On a **byte jigc did not write** — in any sub-task's
///   working area *or* in `.jigc/milestones/<id>/` itself … it blocks with
///   **`milestone.foreign-bytes`**"* — and under the consent the command names it as it
///   takes it (`MD5`); one **in a sub-task's worktree** is content the door *"cannot prove
///   is disposable"*: refused;
/// - **the user's own linked worktree** (`MD6`), its directory there or moved away, is
///   *"left byte-identical"*.
const DISCARD_CELLS: [Cell; 22] = [
    cell(
        "milestone-discard-a-milestone-that-is-not-there",
        &["milestone", "discard"],
        FRESH,
        &[],
        &["milestone", "discard", "m-a-milestone"],
        refuses(1, "milestone.unknown"),
        None,
    ),
    discards!(
        "milestone-discard-ship-it",
        [],
        DISCARD_IT,
        Listed::Acts,
        None
    ),
    discards!(
        "milestone-discard-a-file-of-the-users-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::File(ITS_RECORD)))],
        DISCARD_IT,
        refuses(1, "reconciliation.conflict-block"),
        at("MD1", Kind::Untracked)
    ),
    discards!(
        "milestone-discard-a-file-the-user-committed-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::Committed(ITS_RECORD)))],
        DISCARD_IT,
        refuses(1, "reconciliation.conflict-block"),
        at("MD1", Kind::Committed)
    ),
    discards!(
        "milestone-discard-a-link-to-nothing-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
            ITS_RECORD
        )))],
        DISCARD_IT,
        refuses(1, "store.home-not-regular-file"),
        at("MD1", Kind::LinkToNothing)
    ),
    discards!(
        "milestone-discard-a-link-to-a-file-in-the-repository-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ITS_RECORD,
            outside: false,
        }))],
        DISCARD_IT,
        refuses(1, "store.home-not-regular-file"),
        at("MD1", Kind::LinkInside)
    ),
    discards!(
        "milestone-discard-a-link-to-a-file-outside-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: ITS_RECORD,
            outside: true,
        }))],
        DISCARD_IT,
        refuses(1, "store.home-not-regular-file"),
        at("MD1", Kind::LinkOutside)
    ),
    discards!(
        "milestone-discard-a-directory-in-the-records-place",
        [Lead::Plant(Plant::InPlaceOf(&Plant::Dir(ITS_RECORD)))],
        DISCARD_IT,
        refuses(1, "store.home-not-regular-file"),
        at("MD1", Kind::DirForAFile)
    ),
    discards!(
        "milestone-discard-a-file-in-the-place-of-the-records-directory",
        [
            Lead::MovedAway("repo/docs/milestone-records"),
            Lead::Plant(Plant::File(THE_RECORDS_DIRECTORY)),
        ],
        DISCARD_IT,
        refuses(1, ITS_RECORD),
        at("MD1", Kind::FileForADir)
    ),
    discards!(
        "milestone-discard-a-hand-edit-to-the-record",
        [Lead::Plant(Plant::Edit(ITS_RECORD))],
        DISCARD_IT,
        refuses(1, "reconciliation.conflict-block"),
        at("MD1", Kind::EditAfter)
    ),
    discards!(
        "milestone-discard-the-record-deleted",
        [Lead::Plant(Plant::Deleted(ITS_RECORD))],
        DISCARD_IT,
        refuses(1, "milestone.area-io"),
        at("MD1", Kind::EditAfter)
    ),
    discards!(
        "milestone-discard-a-change-staged-at-the-record",
        [Lead::Plant(Plant::Staged {
            at: ITS_RECORD,
            kept: false,
        })],
        DISCARD_IT,
        refuses(1, "reconciliation.conflict-block"),
        at("MD1", Kind::StagedThere)
    ),
    discards!(
        "milestone-discard-a-file-staged-elsewhere",
        [STAGED_ELSEWHERE],
        DISCARD_IT,
        Listed::Acts,
        at("MD2", Kind::StagedElsewhere)
    ),
    discards!(
        "milestone-discard-a-staged-file-whose-name-git-quotes",
        [STAGED_UNDER_A_QUOTED_NAME],
        DISCARD_IT,
        Listed::Acts,
        at("MD2", Kind::QuotedName)
    ),
    discards!(
        "milestone-discard-a-hook-that-rejects",
        [Lead::Plant(Plant::RejectingHook)],
        DISCARD_IT,
        refuses(1, "`git commit` was rejected"),
        at("MD2", Kind::RejectingHook)
    ),
    discards!(
        "milestone-discard-a-file-of-the-users-beside-the-baseline",
        [Lead::Plant(Plant::File(".jigc/state/notes.md"))],
        DISCARD_IT,
        Listed::Acts,
        at("MD3", Kind::Untracked)
    ),
    discards!(
        "milestone-discard-a-foreign-file-in-the-milestones-area",
        [Lead::Plant(Plant::File(
            ".jigc/milestones/ship-it/notes.txt"
        ))],
        DISCARD_IT,
        refuses(1, "milestone.foreign-bytes"),
        at("MD4, the milestone's area", Kind::Foreign)
    ),
    discards!(
        "milestone-discard-a-foreign-file-in-the-milestones-area-by-consent",
        [Lead::Plant(Plant::File(
            ".jigc/milestones/ship-it/notes.txt"
        ))],
        DISCARD_IT_BY_CONSENT,
        Listed::Acts,
        at("MD4, the milestone's area", Kind::Foreign)
    ),
    discards!(
        "milestone-discard-a-foreign-file-in-a-sub-tasks-area",
        [Lead::Plant(Plant::File(".jigc/tasks/add-alpha/notes.txt"))],
        DISCARD_IT,
        refuses(1, "milestone.foreign-bytes"),
        at("MD4, the sub-tasks' areas", Kind::Foreign)
    ),
    discards!(
        "milestone-discard-a-file-of-the-users-in-a-sub-tasks-worktree",
        [Lead::Plant(Plant::File(
            ".jigc/worktrees/add-alpha/notes.txt"
        ))],
        DISCARD_IT,
        refuses(1, "milestone.dirty-worktree"),
        at("MD4, the worktrees", Kind::Foreign)
    ),
    discards!(
        "milestone-discard-the-users-own-worktree",
        [Lead::Plant(Plant::Worktree { moved_away: false })],
        DISCARD_IT,
        Listed::Acts,
        at("MD6", Kind::WorktreeThere)
    ),
    discards!(
        "milestone-discard-the-users-own-worktree-moved-away",
        [Lead::Plant(Plant::Worktree { moved_away: true })],
        DISCARD_IT,
        Listed::Acts,
        at("MD6", Kind::WorktreeMovedAway)
    ),
];

const DISCARD_ELSEWHERE: [Cell; 6] = fan_out_elsewhere!(
    "discard",
    "ship-it",
    DISCARD_IT,
    [
        TWO_SUB_TASKS[0],
        TWO_SUB_TASKS[1],
        TWO_SUB_TASKS[2],
        PROVISIONED,
    ]
);

/// **A milestone of two code-carrying sub-tasks, joined** — `land-it`, under the plain
/// spelling: each sub-task re-entered in its worktree, its commit doc authored and one
/// file of code staged there; then the join, whose re-entry builds the edge index, so `M10`
/// meets the stamp's file. `$before` stands before the milestone is created — where a
/// change staged meets the carryover gate — and `$after` after the join.
macro_rules! two_sub_tasks_joined {
    ([$($before:expr),* $(,)?], [$($after:expr),* $(,)?]) => {
        &[
            $($before,)*
            jigc(&["milestone", "create", "Land it"]),
            jigc(&["milestone", "add-task", "land-it", "Add alpha file"]),
            jigc(&["milestone", "add-task", "land-it", "Add beta file"]),
            jigc(&["milestone", "provision", "land-it"]),
            jigc(&["milestone", "execute", "land-it"]),
            payload("summary", "add a file\n"),
            ALPHAS_COMMIT_DOC[0], ALPHAS_COMMIT_DOC[1], ALPHAS_COMMIT_DOC[2],
            BETAS_COMMIT_DOC[0], BETAS_COMMIT_DOC[1], BETAS_COMMIT_DOC[2],
            ALPHAS_CODE[0], ALPHAS_CODE[1], BETAS_CODE[0], BETAS_CODE[1],
            jigc(&["milestone", "join", "land-it"]),
            $($after,)*
        ]
    };
}

const FINALIZE_IT: &[&str] = &["milestone", "finalize", "land-it"];
const LAND_ITS_RECORD: &str = "docs/milestone-records/land-it.md";

/// One cell of `jigc milestone finalize`, plainly spelled, in `state`.
const fn finalizes(
    name: &'static str,
    state: Prepared,
    leads: &'static [Lead],
    argv: &'static [&'static str],
    listed: Listed,
    aim: Option<Aim>,
) -> Cell {
    cell(
        name,
        &["milestone", "finalize"],
        state,
        leads,
        argv,
        listed,
        aim,
    )
}

/// **The cells of `jigc milestone finalize`, plainly spelled** — beside the acceptance's
/// three and the spelling `squash: false`'s four. Each listed from the row's doc:
///
/// - **the base cell lands `land-it`** — two code-carrying sub-tasks, one aggregate
///   commit, the record flipped in it, the workbench torn down and the edge index's file
///   taken away (`M3`, `M5`, `M6`, `M7`, `M10`); the leaf as the catalog types it names a
///   milestone that is not there;
/// - **at a created doc's home** (`M2`) — the numbered home the join leaves — a link, a
///   directory, a file in its directory's place and a change staged there are refused:
///   *"a sub-task's `created` doc never promotes over a file already at its destination,
///   and the refusal is taken before either mode lays down a commit"* — a change staged
///   there as `finalize.promote-clobber`, a link or a directory as
///   `store.home-not-regular-file`, *"the one code every command answers this state with"*
///   (`design/team-ready-state.md` → *The terminal is terminal*, the human's ruling of
///   2026-10-06); a file in the place of a home's directory has no doc's words, and a run's
///   are listed;
/// - **what is staged elsewhere stays out of the boundary** (`M3`, and
///   `design/command-output-contract.md`'s *"that entry structurally cannot ride the
///   boundary commit"*), a hook that rejects tears nothing down (`M8`); **a change staged
///   before the milestone was created** meets the carryover gate, *"`jigc milestone
///   create` for the shared checkout's aggregate index"* — the boundary refuses over it
///   (`finalize.carried-staged`); and **a pending config delta** rides the aggregate,
///   which `M3, the config` carries on the layer's first commit alone — a later one it
///   may not ride, by the human's ruling on the rows' record's §6, entry 19;
/// - **at the record** (`M5`), what stands in its place and an edit or a change staged at
///   it are refused as at the other record doors;
/// - **a file jigc did not write in a sub-task's area or the milestone's** is moved aside
///   (`M6`); **one in a live sub-task's worktree** is moved aside to
///   `.jigc/displaced/<sub-task-id>/` before the worktree goes — the human's ruling of
///   2026-10-19 on the rows' record's §6, entry 18, `M6, the worktrees moved aside` — so
///   the cell acts; the product takes it with the checkout and names it, and the plant
///   rule reads that as lost, a finding of the product;
/// - **the user's own worktree** (`M7`) and **a file of the user's in a cache** (`M10`)
///   stay;
/// - **a sub-task's area gone, at a milestone with no work to land**: the re-seed rebuilds
///   the area before the door's own guard refuses (`milestone.zero-contribution`), and the
///   area stays — the areas' half of the re-seed holds on a refusal (`M12`).
const FINALIZE_CELLS: [Cell; 31] = [
    finalizes(
        "milestone-finalize-a-milestone-that-is-not-there",
        FRESH,
        &[],
        &["milestone", "finalize", "m-a-milestone"],
        refuses(1, "milestone.unknown"),
        None,
    ),
    finalizes(
        "milestone-finalize-two-code-carrying-sub-tasks",
        FRESH,
        two_sub_tasks_joined!([], []),
        FINALIZE_IT,
        Listed::Acts,
        None,
    ),
    // A second base cell: two reporters' findings, the second numbered onto `stage-2` —
    // the created docs `M2` lands, which the code-carrying milestone's commit docs are not.
    finalizes(
        "milestone-finalize-two-reporters-findings",
        FRESH,
        two_reporters_joined!([], []),
        &["milestone", "finalize", "stage-reports"],
        Listed::Acts,
        None,
    ),
    // The record's §6, entry 19, ruled by the human on 2026-10-19 (option A): a finalize
    // never carries a change under `.jigc/config` that nobody staged. Red under its key
    // until the product's fix; no mark.
    finalizes(
        "milestone-finalize-a-pending-config-delta",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [jigc(&["config", "set", "finalize.fan-out.squash", "true"])]
        ),
        FINALIZE_IT,
        Listed::Acts,
        None,
    ),
    finalizes(
        "milestone-finalize-a-file-of-the-users-in-the-merged-set",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::File(
                ".jigc/milestones/land-it/merged/notes.txt"
            ))]
        ),
        FINALIZE_IT,
        Listed::Acts,
        at("M1", Kind::Untracked),
    ),
    finalizes(
        "milestone-finalize-a-link-to-nothing-at-the-id-the-join-numbered-onto",
        FRESH,
        two_reporters_joined!([], [Lead::Plant(Plant::LinkToNothing(THE_NUMBERED_HOME))]),
        &["milestone", "finalize", "stage-reports"],
        refuses(3, "store.home-not-regular-file"),
        at("M2", Kind::LinkToNothing),
    ),
    finalizes(
        "milestone-finalize-a-link-to-a-file-in-the-repository-at-the-id-the-join-numbered-onto",
        FRESH,
        two_reporters_joined!(
            [],
            [Lead::Plant(Plant::LinkToAFile {
                at: THE_NUMBERED_HOME,
                outside: false,
            })]
        ),
        &["milestone", "finalize", "stage-reports"],
        refuses(3, "store.home-not-regular-file"),
        at("M2", Kind::LinkInside),
    ),
    finalizes(
        "milestone-finalize-a-link-to-a-file-outside-at-the-id-the-join-numbered-onto",
        FRESH,
        two_reporters_joined!(
            [],
            [Lead::Plant(Plant::LinkToAFile {
                at: THE_NUMBERED_HOME,
                outside: true,
            })]
        ),
        &["milestone", "finalize", "stage-reports"],
        refuses(3, "store.home-not-regular-file"),
        at("M2", Kind::LinkOutside),
    ),
    finalizes(
        "milestone-finalize-a-directory-at-the-id-the-join-numbered-onto",
        FRESH,
        two_reporters_joined!([], [Lead::Plant(Plant::Dir(THE_NUMBERED_HOME))]),
        &["milestone", "finalize", "stage-reports"],
        refuses(3, "store.home-not-regular-file"),
        at("M2", Kind::DirForAFile),
    ),
    finalizes(
        "milestone-finalize-a-file-in-the-place-of-the-findings-directory",
        FRESH,
        two_reporters_joined!([], [Lead::Plant(Plant::File("docs/inconsistencies"))]),
        &["milestone", "finalize", "stage-reports"],
        refuses(1, "could not create"),
        at("M2", Kind::FileForADir),
    ),
    finalizes(
        "milestone-finalize-a-change-staged-at-the-id-the-join-numbered-onto",
        FRESH,
        two_reporters_joined!(
            [],
            [Lead::Plant(Plant::Staged {
                at: THE_NUMBERED_HOME,
                kept: false,
            })]
        ),
        &["milestone", "finalize", "stage-reports"],
        refuses(3, "finalize.promote-clobber"),
        at("M2", Kind::StagedThere),
    ),
    finalizes(
        "milestone-finalize-a-file-staged-elsewhere",
        FRESH,
        two_sub_tasks_joined!([], [STAGED_ELSEWHERE]),
        FINALIZE_IT,
        Listed::Acts,
        at("M3", Kind::StagedElsewhere),
    ),
    finalizes(
        "milestone-finalize-a-file-staged-before-the-milestone",
        FRESH,
        two_sub_tasks_joined!([STAGED_ELSEWHERE], []),
        FINALIZE_IT,
        refuses(3, "finalize.carried-staged"),
        at("M3", Kind::StagedElsewhere),
    ),
    finalizes(
        "milestone-finalize-a-staged-file-whose-name-git-quotes",
        FRESH,
        two_sub_tasks_joined!([], [STAGED_UNDER_A_QUOTED_NAME]),
        FINALIZE_IT,
        Listed::Acts,
        at("M3", Kind::QuotedName),
    ),
    finalizes(
        "milestone-finalize-a-hook-that-rejects",
        FRESH,
        two_sub_tasks_joined!([], [Lead::Plant(Plant::RejectingHook)]),
        FINALIZE_IT,
        refuses(1, "`git commit` was rejected"),
        at("M3", Kind::RejectingHook),
    ),
    finalizes(
        "milestone-finalize-a-file-of-the-users-in-the-records-place",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::File(LAND_ITS_RECORD)))]
        ),
        FINALIZE_IT,
        refuses(1, "reconciliation.conflict-block"),
        at("M5", Kind::Untracked),
    ),
    finalizes(
        "milestone-finalize-a-file-the-user-committed-in-the-records-place",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::Committed(
                LAND_ITS_RECORD
            )))]
        ),
        FINALIZE_IT,
        refuses(1, "reconciliation.conflict-block"),
        at("M5", Kind::Committed),
    ),
    finalizes(
        "milestone-finalize-a-link-to-nothing-in-the-records-place",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(
                LAND_ITS_RECORD
            )))]
        ),
        FINALIZE_IT,
        refuses(1, "store.home-not-regular-file"),
        at("M5", Kind::LinkToNothing),
    ),
    finalizes(
        "milestone-finalize-a-link-to-a-file-in-the-repository-in-the-records-place",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: LAND_ITS_RECORD,
                outside: false,
            }))]
        ),
        FINALIZE_IT,
        refuses(1, "store.home-not-regular-file"),
        at("M5", Kind::LinkInside),
    ),
    finalizes(
        "milestone-finalize-a-link-to-a-file-outside-in-the-records-place",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
                at: LAND_ITS_RECORD,
                outside: true,
            }))]
        ),
        FINALIZE_IT,
        refuses(1, "store.home-not-regular-file"),
        at("M5", Kind::LinkOutside),
    ),
    finalizes(
        "milestone-finalize-a-directory-in-the-records-place",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::InPlaceOf(&Plant::Dir(LAND_ITS_RECORD)))]
        ),
        FINALIZE_IT,
        refuses(1, "store.home-not-regular-file"),
        at("M5", Kind::DirForAFile),
    ),
    finalizes(
        "milestone-finalize-a-file-in-the-place-of-the-records-directory",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [
                Lead::MovedAway("repo/docs/milestone-records"),
                Lead::Plant(Plant::File(THE_RECORDS_DIRECTORY)),
            ]
        ),
        FINALIZE_IT,
        refuses(1, "Not a directory"),
        at("M5", Kind::FileForADir),
    ),
    finalizes(
        "milestone-finalize-a-hand-edit-to-the-record",
        FRESH,
        two_sub_tasks_joined!([], [Lead::Plant(Plant::Edit(LAND_ITS_RECORD))]),
        FINALIZE_IT,
        refuses(1, "reconciliation.conflict-block"),
        at("M5", Kind::EditAfter),
    ),
    finalizes(
        "milestone-finalize-a-change-staged-at-the-record",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::Staged {
                at: LAND_ITS_RECORD,
                kept: false,
            })]
        ),
        FINALIZE_IT,
        refuses(1, "reconciliation.conflict-block"),
        at("M5", Kind::StagedThere),
    ),
    // The record's §6, entry 18, ruled 2026-10-19: the boundary moves the file aside
    // before it removes the worktree (`M6, the worktrees moved aside`); the product takes
    // it, a finding of the product.
    finalizes(
        "milestone-finalize-a-file-of-the-users-in-a-sub-tasks-worktree",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::File(
                ".jigc/worktrees/add-alpha-file/notes.txt"
            ))]
        ),
        FINALIZE_IT,
        Listed::Acts,
        at("M6, the worktrees", Kind::Foreign),
    ),
    finalizes(
        "milestone-finalize-a-foreign-file-in-a-sub-tasks-area",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::File(
                ".jigc/tasks/add-alpha-file/notes.txt"
            ))]
        ),
        FINALIZE_IT,
        Listed::Acts,
        at("M6, the sub-tasks' areas", Kind::Foreign),
    ),
    finalizes(
        "milestone-finalize-a-foreign-file-in-the-milestones-area",
        FRESH,
        two_sub_tasks_joined!(
            [],
            [Lead::Plant(Plant::File(
                ".jigc/milestones/land-it/notes.txt"
            ))]
        ),
        FINALIZE_IT,
        Listed::Acts,
        at("M6, the milestone's area", Kind::Foreign),
    ),
    finalizes(
        "milestone-finalize-the-users-own-worktree",
        FRESH,
        two_sub_tasks_joined!([], [Lead::Plant(Plant::Worktree { moved_away: false })]),
        FINALIZE_IT,
        Listed::Acts,
        at("M7", Kind::WorktreeThere),
    ),
    finalizes(
        "milestone-finalize-a-file-of-the-users-under-the-edge-index",
        FRESH,
        two_sub_tasks_joined!([], [Lead::Plant(Plant::File(".jigc/index/notes.md"))]),
        FINALIZE_IT,
        Listed::Acts,
        at("M10, the edge index", Kind::Untracked),
    ),
    finalizes(
        "milestone-finalize-a-file-of-the-users-beside-the-file-state-hashes",
        FRESH,
        two_sub_tasks_joined!([], [Lead::Plant(Plant::File(".jigc/state/notes.md"))]),
        FINALIZE_IT,
        Listed::Acts,
        at("M10, the file-state hashes", Kind::Untracked),
    ),
    finalizes(
        "milestone-finalize-a-sub-tasks-area-gone-and-no-work-to-land",
        FRESH,
        &[
            TWO_SUB_TASKS[0],
            TWO_SUB_TASKS[1],
            TWO_SUB_TASKS[2],
            Lead::MovedAway("repo/.jigc/tasks/add-alpha"),
        ],
        &["milestone", "finalize", "ship-it"],
        refuses(3, "milestone.zero-contribution"),
        None,
    ),
];

const FINALIZE_ELSEWHERE: [Cell; 6] = fan_out_elsewhere!(
    "finalize",
    "land-it",
    FINALIZE_IT,
    [
        jigc(&["milestone", "create", "Land it"]),
        jigc(&["milestone", "add-task", "land-it", "Add alpha file"]),
        jigc(&["milestone", "add-task", "land-it", "Add beta file"]),
        jigc(&["milestone", "provision", "land-it"]),
        jigc(&["milestone", "execute", "land-it"]),
        payload("summary", "add a file\n"),
        ALPHAS_COMMIT_DOC[0],
        ALPHAS_COMMIT_DOC[1],
        ALPHAS_COMMIT_DOC[2],
        BETAS_COMMIT_DOC[0],
        BETAS_COMMIT_DOC[1],
        BETAS_COMMIT_DOC[2],
        ALPHAS_CODE[0],
        ALPHAS_CODE[1],
        BETAS_CODE[0],
        BETAS_CODE[1],
        jigc(&["milestone", "join", "land-it"]),
    ]
);

/// **A milestone of one sub-task that updates the committed vision from its worktree** —
/// `land-it` in `committed-singletons`: the sub-task `sharpen-the-vision` re-entered in its
/// worktree, its commit doc authored there, and slots of `vision:vision` written —
/// `$write`, each from `outside/prose`, the first of them the sub-task's first write to the
/// doc; then the join. `$before` stands after the provision and before that first write,
/// `$after` after the join, last before the command. The sub-task's doc-store is the main
/// checkout's (`design/storage.md` → *CLI and git*), so its first write copies in the main
/// checkout's `VISION.md`, whatever its worktree holds.
macro_rules! a_sub_task_over_the_vision {
    ([$($write:expr),* $(,)?], [$($before:expr),* $(,)?], [$($after:expr),* $(,)?]) => {
        &[
            jigc(&["milestone", "create", "Land it"]),
            jigc(&["milestone", "add-task", "land-it", "Sharpen the vision"]),
            jigc(&["milestone", "provision", "land-it"]),
            $($before,)*
            payload("summary", "sharpen the vision\n"),
            payload("prose", "Sharper.\n"),
            Lead::Step(Step::JigcAt(
                Place::Repo(".jigc/worktrees/sharpen-the-vision"),
                &["workflow", "sub-task", "--task", "sharpen-the-vision"],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(".jigc/worktrees/sharpen-the-vision"),
                &[
                    "doc",
                    "set-field",
                    "commit:sharpen-the-vision#type",
                    "--value",
                    "docs",
                    "--task",
                    "sharpen-the-vision",
                ],
            )),
            Lead::Step(Step::JigcAt(
                Place::Repo(".jigc/worktrees/sharpen-the-vision"),
                &[
                    "doc",
                    "set-slot",
                    "commit:sharpen-the-vision#summary",
                    "--from-file",
                    "../../../../outside/summary",
                    "--task",
                    "sharpen-the-vision",
                ],
            )),
            $($write,)*
            jigc(&["milestone", "join", "land-it"]),
            $($after,)*
        ]
    };
}

/// A line added by hand to the committed vision in the main checkout, in no commit (the
/// plan's plant **E**). It stands in the vision's last part, `open-questions`.
const A_HAND_EDIT_TO_THE_VISION: Lead = Lead::Plant(Plant::Edit("VISION.md"));

/// Where the sub-task of [`a_sub_task_over_the_vision`] is re-entered and writes.
const IN_THE_SUB_TASKS_WORKTREE: Place = Place::Repo(".jigc/worktrees/sharpen-the-vision");

/// The sub-task's write to a slot of the vision, from its worktree — the thesis, the first
/// of the doc's three parts, and `open-questions`, the last, where
/// [`A_HAND_EDIT_TO_THE_VISION`] stands.
const WRITE_THE_THESIS: &[&str] = &[
    "doc",
    "set-slot",
    "vision:vision#thesis",
    "--from-file",
    "../../../../outside/prose",
    "--task",
    "sharpen-the-vision",
];
const WRITE_THE_OPEN_QUESTIONS: &[&str] = &[
    "doc",
    "set-slot",
    "vision:vision#open-questions",
    "--from-file",
    "../../../../outside/prose",
    "--task",
    "sharpen-the-vision",
];
const A_WRITE_TO_THE_THESIS: Lead =
    Lead::Step(Step::JigcAt(IN_THE_SUB_TASKS_WORKTREE, WRITE_THE_THESIS));

/// **The sub-task's write to the part [`A_HAND_EDIT_TO_THE_VISION`] stands in** — the write
/// the human's ruling of 2026-10-19 refuses, showing the line: it may act or refuse
/// ([`Lead::OverAHandEdit`]). The edit is the cell's fourth lead, after the milestone's
/// create, add-task and provision.
const A_WRITE_OVER_THE_HAND_EDIT: Lead = Lead::OverAHandEdit {
    step: Step::JigcAt(IN_THE_SUB_TASKS_WORKTREE, WRITE_THE_OPEN_QUESTIONS),
    plant: 3,
};

const SINGLETONS: Prepared = Prepared::Of(State::CommittedSingletons);

/// **The cells of `M2` × *E*** — a hand edit at the home of a doc a sub-task updates, in
/// `committed-singletons`, the one prepared state whose committed doc a sub-task copies
/// in. Each is listed from the docs, as the task family's cells at `T1` are:
///
/// - **before the sub-task's first write**, the edit *"is carried by the copy-in and
///   recorded as the baseline, so that order keeps landing with both sides in one
///   commit"* (`design/reconciliation.md` → *Baseline adoption*), and at the boundary *"a
///   sub-task's area is a task area"*, and *"The file is what the task copied in → never
///   a conflict"* (→ *What a task copied in*): the cell acts. Where the sub-task writes
///   **another part**, the line lands with it; where it writes **the part the line
///   stands in**, its write replaces it — what the human's declared bound of 2026-10-06
///   on item 23 reached, *"a task whose first write to that doc copies the file in, then a
///   write by that same task to the slot or field the hand edit changed"*, until the human
///   retired the bound on 2026-10-19: such a write refuses at the write, naming the part
///   and showing the hand-edited text; the cell is red under its key, a finding of the
///   product, and carries no mark. Its write to that part is [`A_WRITE_OVER_THE_HAND_EDIT`],
///   which may act or refuse, and the sub-task goes on to write the thesis: on a product
///   that refuses the write, the line lands with the thesis, and the cell holds;
/// - **after it**, *"The file is no longer what the task copied in → the caller's
///   `reconciliation.conflict-block`"*, and nothing is recorded.
const FINALIZE_A_HAND_EDIT: [Cell; 3] = [
    finalizes(
        "milestone-finalize-a-hand-edit-before-a-sub-tasks-first-write-to-another-part",
        SINGLETONS,
        a_sub_task_over_the_vision!([A_WRITE_TO_THE_THESIS], [A_HAND_EDIT_TO_THE_VISION], []),
        FINALIZE_IT,
        Listed::Acts,
        at("M2", Kind::EditBefore),
    ),
    finalizes(
        "milestone-finalize-a-hand-edit-before-a-sub-tasks-first-write-to-the-same-part",
        SINGLETONS,
        a_sub_task_over_the_vision!(
            [A_WRITE_OVER_THE_HAND_EDIT, A_WRITE_TO_THE_THESIS],
            [A_HAND_EDIT_TO_THE_VISION],
            []
        ),
        FINALIZE_IT,
        Listed::Acts,
        at("M2", Kind::EditBefore),
    ),
    finalizes(
        "milestone-finalize-a-hand-edit-after-a-sub-tasks-first-write",
        SINGLETONS,
        a_sub_task_over_the_vision!([A_WRITE_TO_THE_THESIS], [], [A_HAND_EDIT_TO_THE_VISION]),
        FINALIZE_IT,
        refuses(3, "reconciliation.conflict-block"),
        at("M2", Kind::EditAfter),
    ),
];

/// **A change staged before the milestone, under `squash: false`** — the carryover gate is
/// the boundary's under both spellings (`M4`): refused, as under the plain one.
const UNSQUASHED_CARRIED: [Cell; 1] = [unsquashed(
    "milestone-finalize-squash-false-a-file-staged-before-the-milestone",
    two_sub_tasks_unsquashed!([STAGED_ELSEWHERE], []),
    refuses(3, "finalize.carried-staged"),
    Some(Aim {
        line: "M4",
        kind: Kind::StagedElsewhere,
    }),
    &[],
)];

/// **The family's cells**: `milestone finalize`'s acceptance and its spelling `squash:
/// false`, then the record doors' — `create`, `add-task`, `add-from-spec` — the fan-out
/// doors' — `provision`, `execute`, `join` — and the terminals' — `discard`, and
/// `finalize`'s plain spelling.
pub const CELLS: &[&[Cell]] = &[
    ACCEPTANCE,
    &UNSQUASHED,
    &CREATE_CELLS,
    &ADD_TASK_CELLS,
    &ADD_FROM_SPEC_CELLS,
    &PROVISION_CELLS,
    &PROVISION_ELSEWHERE,
    &EXECUTE_CELLS,
    &EXECUTE_ELSEWHERE,
    &JOIN_CELLS,
    &JOIN_ELSEWHERE,
    &DISCARD_CELLS,
    &DISCARD_ELSEWHERE,
    &FINALIZE_CELLS,
    &FINALIZE_ELSEWHERE,
    &FINALIZE_A_HAND_EDIT,
    &UNSQUASHED_CARRIED,
];
