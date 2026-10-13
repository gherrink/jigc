# The byte table's pilot — its findings, by key

What the byte table ([byte_table.rs](../../../../../crates/cli/tests/byte_table.rs); the frame is [byte_cell.rs](../../../../../crates/cli/tests/support/byte_cell.rs), the plan [byte-table-plan.md](byte-table-plan.md)) found: what a cell came out red with, and what the writing of its rows from the design docs found in the docs themselves. A key is in the grammar the route catalog's findings have ([route-catalog-findings.md](route-catalog-findings.md)) — `r1-<command>-<what happened, in words>` — and is what the suite's expected-red list names where a cell is under it. **Nothing here is a ruling, a grade or a word about the previous release**: a row about a cell is what one binary did on one day, and a row about the docs is two sentences set side by side.

**No ledger holds these keys.** Until a stabilization run is opened they certify themselves, as the route catalog's do.

## 1. Where two design docs say two things — recorded here, and corrected by nobody in this lane

### `r1-finalize-design-says-no-side-effects-where-storage-design-says-caches-are-rebuilt`

*Recorded 2026-10-13, as the human's ruling of 2026-10-12 asks: the sentence is "an inconsistency between two design docs, to be recorded as one and corrected — not a rule the table holds a command to" ([DECISIONS.md](../../../../../DECISIONS.md) → *A refusal may rebuild a cache*). For the stabilization run's ledger: doctype `inconsistency`, two docs that disagree.*

**One side** — `design/finalize.md` → *2. Validate*, of a finalize that validation blocks:

> - **Blocking findings present** → abort with the finding list. No side effects. Agent fixes and re-runs `finalize`.

**The other side** — `design/storage.md` → *Source of truth*:

> Every other artifact — the staging working area, the edge index, the file↔state hashes — is either a rebuildable cache or a transient working copy.

and `design/storage.md` → *Edge index lifecycle*, the first row of its table of the five sites at which the index is written:

> | **committed rebuild** | first read after a stamp mismatch (branch switch, pull, rebase) — rebuild from committed `.md`s | atomic; persists in `.jigc/index/` with the new stamp |

**What disagrees.** A finalize reads the committed store before it can block, and by the second doc the first read after a stamp mismatch rebuilds the edge index **and persists it**. So a finalize that then blocks has had a side effect — files written under `.jigc/index/` — which the first doc's two words rule out and the second doc's design intends. Neither sentence says of the other that it is an exception.

**Where it was met.** Writing the row of `jigc task finalize` from the docs ([byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §6, entry 1): the row could not say whether a refused finalize may leave the caches changed, because the two docs answer differently. The plan's own run of a refused `jigc task finalize` on this tree left three new files under `.jigc/index/` ([byte-table-plan.md](byte-table-plan.md) → §1.2).

**What was ruled, and what was not.** The table follows the storage design: a refusal may rebuild a cache, and no cell is red for it (the record's §4a, lines `C1` to `C3`). Which of the two sentences is corrected, and to what, is not ruled here and no design doc is edited in this lane. A neighbour of the same question is in the record's §6, entry 1, *what the ruling leaves open*: `design/storage.md` → *Derived caches* calls the file↔state hashes *rebuildable from the committed docs* in one sentence and says in the next paragraph that they *"carry **no stamp and no rebuild path**"* — one doc, two sentences.

## 2. The cells — none is red on this tree's binary

*2026-10-13, this tree's debug build, macOS arm64; the task `B2d`. The suite's expected-red list is empty, and exact: a cell that comes out red is named by the arm of its state.*

The twelve cells of the pilot's acceptance ([byte-table-plan.md](byte-table-plan.md) → §5), each one plant at the place and under the condition the blocking row's own record names. **Eleven refuse and change nothing a refusal may not; one acts and holds.** No key is minted: nothing here is a finding of the product.

| the cell | came out | the refusal it printed | what its row's lines excused |
|---|---|---|---|
| `setup` · never adopted, `HEAD` unborn · an untracked file at `.jigc/AGENT.md` | *refuses, nothing changed* | exit 1, `setup.dirty-install-path` | nothing |
| — the same, `HEAD` born | *refuses, nothing changed* | exit 1, `setup.dirty-install-path` | nothing |
| `task finalize` · `committed-singletons`, no file-state baseline · a line added to `VISION.md` after the task's first write | *refuses, nothing changed* | exit 3, `reconciliation.conflict-block` | `C1` × 1 |
| — the same, the baseline present | *refuses, nothing changed* | exit 3, `reconciliation.conflict-block` | `C1` × 1 |
| `task finalize` · `fresh` · a link to nothing at the created doc's home | *refuses, nothing changed* | exit 3, `store.home-not-regular-file` | `C1` × 3 |
| `milestone finalize` · `fresh` · an untracked file at the id the join numbered onto | *refuses, nothing changed* | exit 3, `finalize.promote-clobber` | `M1` × 4 |
| `milestone finalize` · `fresh` · a committed file at that id, from before the milestone | *refuses, nothing changed* | exit 3, `finalize.promote-clobber` | `M1` × 4 |
| `uninstall` · `fresh` · a file jigc did not write under `.jigc/logs/` | *refuses, nothing changed* | exit 1, `uninstall.foreign-bytes` | nothing |
| — under `.jigc/state/` | *refuses, nothing changed* | exit 1, `uninstall.foreign-bytes` | nothing |
| — under `.jigc/index/` | *refuses, nothing changed* | exit 1, `uninstall.foreign-bytes` | nothing |
| — at `.jigc/`'s root | *refuses, nothing changed* | exit 1, `uninstall.untracked-workbench-file` | nothing |
| `milestone finalize` · `fresh` · the user's own linked worktree, its directory moved away | *holds* | exit 0 | `M3` × 7 · `M5` × 3 · `M6`, the worktrees × 36 · `M6`, the sub-tasks' areas × 8 · `M6`, the milestone's area × 5 · `M7` × 4 · `M10`, the edge index × 2 · `M10`, the file-state hashes × 1 |

**A cell that refuses is held to refusing, not to which refusal** — the frame has no word for the code. The codes above were read off each cell's output by the builder: each is the refusal the row's doc means, and none is a refusal for want of a lead-in. What the same cells come to on the previous release's binary — where eight of them act — is the table of the task after this one.
