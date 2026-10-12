//! **The byte table's row for `jigc task finalize`** (`crates/cli/tests/byte_table.rs`) —
//! lifted quote for quote from the committed record
//! (`completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md`,
//! §3), under the rules the install family's file states. `jigc task discard` joins this
//! file with its row in the plan's task `B4`.
//!
//! **What of the record is no line.** `T7` is where *on a refusal: no* comes from. `T8` is
//! the rule for prose handed to jigc, which the frame holds a staged doc to by itself
//! (`support::byte_cell` → *a staged doc*). **The doc-only arm is a second spelling** with
//! a commit line of its own — a task whose workflow includes `step:finalize-doc-only`
//! commits its own paths and never the index — and no cell names it yet: the comparison
//! has no switch for it, and a cell of such a task would be read under `T4`.

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

/// **`jigc task finalize`** — the record's §3. The record names no consent of its own.
pub const FINALIZE: Row = Row {
    leaf: &["task", "finalize"],
    consent: None,
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
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "5. Stage",
                    words: "the `.jigc/version` stamp, refreshed to the running build first — **where it is jigc's to write**",
                },
            ],
        ),
        line(
            "T3",
            Allows::Staged,
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
            Allows::Files(Where::Under(".jigc/index"), May::WriteWhole),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "- **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches).",
                },
            ],
        ),
        line(
            "T5, the file-state hashes",
            Allows::Files(Where::Under(".jigc/state"), May::WriteWhole),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "- **Update `file-state` hashes** for every committed managed doc.",
                },
            ],
        ),
        line(
            "T6",
            Allows::Files(Where::Under(".jigc/tasks/*"), May::Removed),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "- **Keep what jigc did not write, then remove `.jigc/tasks/<task-id>/`.**",
                },
            ],
        ),
        line(
            "T6, moved aside",
            Allows::Files(
                Where::Under(".jigc/tasks/*"),
                May::MovedAsideTo(".jigc/displaced"),
            ),
            &[
                Quote {
                    doc: "design/finalize.md",
                    heading: "7. Post-commit (best-effort)",
                    words: "The removal's subject is jigc's own files only: the area's **complement** ([team-ready-state.md](team-ready-state.md) → The working area's two populations) is moved aside first, to `.jigc/displaced/<task-id>/` with its relative paths intact",
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
