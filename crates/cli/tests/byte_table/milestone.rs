//! **The byte table's row for `jigc milestone finalize`**
//! (`crates/cli/tests/byte_table.rs`) — lifted quote for quote from the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`,
//! §4), under the rules the install family's file states. The other leaves of the family
//! join this file with their rows if the human rules for it at the pilot's stop.
//!
//! **What of the record is no line.** `M8` is where *on a refusal: no* comes from. `M9` —
//! a refused boundary leaves objects in the store — is no difference to a table that
//! holds the store to one rule, that no object there before is absent after. **`M4` is a
//! second spelling**: under `finalize.fan-out.squash: false` the boundary makes one
//! commit a sub-task that staged code and then the aggregate; no cell sets that knob, and
//! the comparison reads every commit the command made under `M3` either way.

use super::ON_A_REFUSAL;
use crate::support::byte_cell::{Allows, Holding, Line, May, Quote, Row, When, Where};

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

/// **`jigc milestone finalize`** — the record's §4. The record names no consent.
pub const FINALIZE: Row = Row {
    leaf: &["milestone", "finalize"],
    consent: None,
    lines: &[
        // The one line of the four rows whose own sentence says it holds on a refusal.
        Line {
            on_refusal: true,
            ..line(
                "M1",
                Allows::Files(Where::Under(".jigc/milestones/*/merged"), May::WriteWhole),
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
            Allows::Commit(Holding::TheWorktrees),
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
            ],
        ),
        // The record's home is the second sentence's: one file a milestone, under the
        // docs root as the prepared states have it.
        line(
            "M5",
            Allows::Files(Where::Path("docs/milestone-records/*"), May::WriteWhole),
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
            ],
        ),
        line(
            "M6, the worktrees",
            Allows::Files(Where::Under(".jigc/worktrees/*"), May::Removed),
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
            Allows::Files(Where::Under(".jigc/tasks/*"), May::Removed),
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
                Where::Under(".jigc/tasks/*"),
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
            Allows::Files(Where::Under(".jigc/milestones/*"), May::Removed),
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
            Allows::Files(Where::Under(".jigc/index"), May::WriteWhole),
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
            Allows::Files(Where::Under(".jigc/state"), May::WriteWhole),
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
