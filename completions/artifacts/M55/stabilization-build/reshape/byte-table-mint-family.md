# The byte table's mint family — its numbers

*Written 2026-10-20 with the last commit of the plan's task `B9` ([byte-table-plan.md](byte-table-plan.md) → §8), on the branch `fix/rc24-route-catalog`, after the human's two rulings of that day, the family's rows and its cells ([DECISIONS.md](../../../../../DECISIONS.md), the entries of 2026-10-20). A record: it counts what the family built and what it costs, and **decides nothing**. The movers family's record is [byte-table-movers-family.md](byte-table-movers-family.md); the findings are [byte-table-findings.md](byte-table-findings.md) → §26 (and §25 for the rulings' keys), the rows [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §4d, and the questions §6, entry 23, ruled.*

**The table committed beside this file is the arm's own bytes, and it is the whole table.** [byte-table-mint-family-cells.tsv](byte-table-mint-family-cells.tsv) is the block `cells` of `byte_table::the_cells_are_run_on_the_binary_a_variable_names` as one run printed it — **all 738 cells** on the previous release's binary (`1.0.0-rc.24`, rebuilt from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run — not an installed `jigc`) and on this tree's debug build, each on a copy of the state that binary built, with each cell's seconds. Printed on 2026-10-20 on the tree of the family's third commit, `643e953e` — its cells, rows and frame as committed; no product source changed since the family began: 528 s, no gate beside it, at a load average near 90. A classification is what one binary did on one day. **Every count of the previous release below is read off it.** It supersedes [the movers family's print](byte-table-movers-family-cells.tsv) as the table's latest, which stays that run's record.

Sources: **(suite)** — an assertion or a list of [byte_table.rs](../../../../../crates/cli/tests/byte_table.rs) and [mint.rs](../../../../../crates/cli/tests/byte_table/mint.rs); **(print)** — that run, its blocks `cells`, `needs` and `excused`; **(gate)** — the full gates of the family's commits, their `test-seconds` lines and the arm's own seconds file; **(on demand)** — the arm of the cells on demand, run by hand with the family's third commit ([byte-table-on-demand-cells.tsv](byte-table-on-demand-cells.tsv)). macOS arm64 throughout.

---

## 1. The family

| | `start` | `workflow` | `migrate` | `task amend` | all | source |
|---|---:|---:|---:|---:|---:|---|
| lines | 7 (`ST1`, `ST2`, `ST2, at a resume`, `C1`–`C3`) | 6 (`WF1`, `WF2`, `C1`–`C3`) | 6 (`MI1`, `MI2`, `C1`–`C3`) | 5 (`AM1`, `C1`–`C3`) | **24** | (suite): each row's `lines` |
| spellings | 2 (`bare`, `--task`) | 1 (`--preview`) | — | — | 3 | (suite) |
| needs | 13 | 11 | 16, one scheduled | 9 | **49** | (print), block `needs` |
| cells in the gate | 9 | 5 | 9 | 4 | **27** | (suite) |
| cells on demand | 6 | 6 | 6 | 6 | **24** | (suite) |
| **red on this tree, in the gate** | **0** | **1** | **0** | **0** | **1** | (suite) `EXPECTED_RED` |
| red on demand | 0 | 0 | 0 | 0 | 0 | (on demand) |
| **red on the previous release** | **0** | **1** | **0** | **0** | **1** | (print) |
| marked cells | — | — | — | — | 0 | (suite) |

**One key, the same on both binaries — not a regression:**

| key | cells | came to | which is wrong |
|---|---:|---|---|
| `r1-workflow-re-entry-takes-a-file-of-the-users-as-the-sub-tasks-commit-doc` | 1 | *not the listed outcome* — exit 0, nothing provisioned, the user's file left as the sub-task's staged commit doc | the code; the provisioning mirrors the mint, which refuses what stands at its area |

And what no cell holds (the findings' §26, *Seen*): `migrate` over a directory at its source refuses in the run's words, a lead for the route catalog; and `migrate` admits a source whose working bytes no git object holds, which its retire at `jigc task finalize --approve` would remove — a lead for the task family, not driven to the retire.

**What each line excused** (print), the same on both binaries: `ST1` 50, `ST2` 5, `ST2, at a resume` 5, `WF1` 6, `WF2` 8, `MI1` 33, `AM1` 66; `MI2` nothing — the source is read — and no `C` line anything: no refusal of the family rebuilt a cache.

**The table now** (suite): **thirty rows; 738 cells — 558 in the gate in four arms, 180 on demand**; the acceptance 12, the task family 71, `milestone finalize`'s spelling `squash: false` 4, the install family 131, the doc family 139, the milestone family 175, the movers family 155 and the mint family 51. **Red in the gate: 150 cells under forty keys**; on demand fourteen, as before. Needs **670, eleven scheduled**. **Marked cells: one** — bound 1. The owed lists — of leaves whose rows, and whose cells, the plan's tasks owed — are empty, and gone with `byte_table/owed.rs`.

**The whole table on the previous release** (print): **259 of the 738 cells are red there, where 164 are red on this tree** (150 in the gate, fourteen on demand). By family, both binaries: the mint 51 cells, one red on each; the movers 155, 57 and 46 (the first commit's rulings — the findings' §25, one cell red here and clean there, `placement-root`'s parking of a link to nothing, which rc.24 refused). **What moved since the movers family's print** (`002b2090`, 683 cells): twenty-one cells by the human's rulings of 2026-10-20 and four new ones (the task's first commit; the movers record's note), and the fifty-one of this family; the other 662 to the same classification, exit, listing, mark and key on each binary.

## 2. Cost

**A cell's seconds** — in the frame, from the copy of its state to its verdict (print, this tree's binary, at a load near 90).

| leaf | cells | seconds | the gate's cells, summed |
|---|---:|---|---:|
| `start` — a task minted first for the spellings | 15 | 0.9 to 2.1, mean 1.5 | ≈ 15 |
| `workflow` — a milestone of one sub-task provisioned | 11 | 0.8 to 4.1, mean 2.1 | ≈ 17 |
| `migrate` — a foreign file committed | 15 | 0.8 to 2.2, mean 1.6 | ≈ 17 |
| `task amend` | 10 | 0.8 to 2.1, mean 1.3 | ≈ 6 |
| every cell of the family | 51 | 0.8 to 4.1, mean 1.6 | ≈ 56 |

**Inside a gate** (gate, the family's third commit, measured): the family's twenty-seven cells 1.1 to 5.6 s, ≈ 70 cell-seconds; the arm of `fresh` **171.7 s**, where it took **146.9 s** in the gate of the family's first commit — a difference of runs at different loads (both near 90), not a measurement of the family alone. The gate's second tier took 1,496 s, 1,540 s and 1,981 s across the family's three gates, each beside another lane's load. **Against the milestone family's projection** for the eight leaves of `B8` and `B9` (≈ 140 gate cells, ≈ 370 cell-seconds): measured, `B8`'s four came to 127 and ≈ 251, and `B9`'s four to 27 and ≈ 56 — 154 gate cells and ≈ 307 cell-seconds for the eight. *Arithmetic over measured numbers.*

## 3. The questions the family met

- **The human's two rulings of 2026-10-20**, recorded by the family's first commit — the rows' record's §6, entry 23, stricter than its option A: any file at a mover's destination refuses; and R1: an acting sweep exits non-zero where an act it was asked for did not happen. Three marks left their cells; twenty-one cells moved, red on both binaries but one.
- **No new entry of §6.** Each place the docs seemed silent or two-sided was answered by a sentence (the rows' record's §4d, *What the docs do not say*): bare `start`'s edge index against the *"pure readers"* of the file-state baseline; a colliding slug, *"suffix only in the deterministic parallel case"*; and the re-entry over a file of the user's, by the mint's refusal the provisioning mirrors — a key the human may take as a question. One doc lead: `design/finalize.md` names *"the three task-minting doors"*, and the amend door mints a task too.
- **The open entries 4 and 7** — met by no door of this family.

## 4. Not established

- **That the rows are complete beyond what each leaf wrote when driven with nothing planted.** The four leaves were each driven once, over `fresh` with a task, a sub-task or a foreign file as their rows need, and every write was a line's.
- **`migrate`'s source edited before the mint and retired at `--approve`**: the retire is the task family's door, and no cell drives a migration to it.
- **The arm's seconds on a quiet machine, and on Linux**: no run in the runner-shaped container was made for the family.
