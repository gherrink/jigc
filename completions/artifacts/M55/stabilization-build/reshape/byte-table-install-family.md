# The byte table's install family — its numbers

*Written 2026-10-15 with the last commit of the plan's task `B5` ([byte-table-plan.md](byte-table-plan.md) → §8, *What would follow*), on the branch `fix/rc24-route-catalog`, after the human's rulings of 2026-10-13 and 2026-10-14 at the pilot's stop ([DECISIONS.md](../../../../../DECISIONS.md), the entries of 2026-10-15). A record: it counts what the family built and what it costs, and **decides nothing**. The pilot's own record stays the stop's ([byte-table-pilot.md](byte-table-pilot.md)); the findings are [byte-table-findings.md](byte-table-findings.md) → §6, the rows [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §1, §2, §2b, §2c, the questions [byte-table-readings.md](byte-table-readings.md) → §10.*

**The table committed beside this file is the arm's own bytes.** [byte-table-install-family-cells.tsv](byte-table-install-family-cells.tsv) is the block `cells` of `byte_table::the_cells_are_run_on_the_binary_a_variable_names` as one run printed it on 2026-10-15 — every cell of the table on the previous release's binary (`1.0.0-rc.24`, built from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run) and on this tree's debug build, each on a copy of the state that binary built, with each cell's seconds: 123 s at a load average of 8 at its start and 9 at its end, no gate beside it. A classification is what one binary did on one day.

Sources: **(suite)** — an assertion or a list of [byte_table.rs](../../../../../crates/cli/tests/byte_table.rs); **(arm)** — that run; **(gate)** — the gate of the family's cells' commit, its timing record `target/jigc-gate/test-seconds.tsv` and the files the arms leave their cells' seconds in, at a load average of 8 at its start and 23 at its end; **(on demand)** — the arm of the cells on demand, run by hand at a load of 9 to 11. macOS arm64 throughout.

---

## 1. The family

| | `setup` | `uninstall` | `ingest` | `unmanage` | all | source |
|---|---:|---:|---:|---:|---:|---|
| lines of its row | 16 | 12 | 6 | 6 | 40 | (suite) |
| needs | 86 | 21 | 10 | 10 | 127 | (arm), block `needs` |
| — answered by a cell | 73 | 21 | 10 | 10 | 114 | |
| — scheduled, with a reason | 13 | — | — | — | 13 | `scheduled-cells.tsv` |
| cells in the gate — new · the acceptance's | 66 · 2 | 15 · 4 | 4 | 4 | 89 · 6 | (suite) |
| cells on demand | 6 | 6 | 6 | 6 | 24 | (suite) |
| **red on this tree, in the gate** | **32** | **15** | 0 | 0 | **47** | (suite) `EXPECTED_RED` |
| red on demand | 0 | 4 | 0 | 0 | 4 | (on demand) |

**The table now**: seven rows, 75 lines; 174 cells, 138 in the gate in three arms and 36 on demand; **52 red in the gate under 12 keys** — the task family's five and the family's 47 under seven; the list of leaves whose cells are owed holds `milestone finalize` alone (suite).

**The table on 2026-10-18** — after the human's rulings on §3's questions and the task `B6`'s plants, the plant in the place of a file that stands and the plant of a file simply deleted, read again by the four fixes after the review of `B6`'s first two commits ([DECISIONS.md](../../../../../DECISIONS.md), the entries of 2026-10-18). The counts on this tree are the suite's: **208 cells — 172 in the gate, 36 on demand**; the acceptance 12, the task family 65 (53 in the gate, 12 on demand), this family 131 (107 in the gate, 24 on demand). **Red in the gate: 44 cells under thirteen keys** (`EXPECTED_RED`) — this family 37 under seven, the task family 7 under six. **Six needs scheduled** with a reason (five of `setup`'s, one of `task finalize`'s); **two cells marked** (bound 1, bound 2). **On demand: four red** — the four `uninstall` cells that act, by `U9` — as [byte-table-on-demand-cells.tsv](byte-table-on-demand-cells.tsv) has them, its arm printing the same bytes again on 2026-10-18. **On the previous release, a committed run establishes only the 174 cells of the table above, as they were listed then** ([byte-table-install-family-cells.tsv](byte-table-install-family-cells.tsv)). What the records say of it since — the listings the rulings moved, the 21 cells of the plant in the place of a file that stands, the 13 of a file simply deleted — was read by the arm on a named binary in runs whose prints are not committed, and **no committed run verifies it**.

**Seven keys, every one red on the previous release too** — 40 of the 47 cells in the same class, seven in another; findings §6 has each. On this tree the classes are: *written outside its allowance* 27 (13 of them `uninstall` by `U9`, 14 `setup` by an empty `.jigc/state/`), *refused, and changed what a refusal may not* 14 (12 `setup` late failures, 2 `uninstall`), *git's state moved outside its allowance* 4, ***lost* 2** — a change staged at `.jigc/version` or at the guide, replaced at exit 0.

**No byte of the user's is lost by a refusal, on this tree.** The two losses are at exit 0, at paths `setup` stages itself. **The previous release loses more**: of the family's cells it is red in 78 of 119 (arm), and 27 of them are red there and not here — four of those a plant lost at exit 0, a file or a link at a path the install replaces, written over.

## 2. Cost

**A cell's seconds, by kind** — in the frame, from the copy of its state to its verdict.

| kind | cells | in the arm on a named binary, this tree's binary, load 8 to 9 | inside the gate, load 8 rising to 31 |
|---|---:|---:|---:|
| `setup` that acts | 26 | 1.2 to 2.4 s, mean 2.1 | — |
| `setup` that refuses | 47 | 1.2 to 2.0 s, mean 1.4 | — |
| `setup`, every cell of `bare` | 68 | — | 1.3 to 6.2 s, mean 2.9 |
| `uninstall` | 24 | 0.7 to 2.0 s, mean 1.3 | 1.3 to 4.2 s, mean 1.8 (19) |
| `ingest`, `unmanage` | 20 | 0.7 to 1.5 s, mean 1.0 | 0.9 to 1.8 s, mean 1.4 (8) |
| a cell after a task landed — eleven lead-ins and a finalize | 2 | 3.1 to 3.7 s | — |

**What the family adds to a gate.** The arms that run cells: **56.2 s** in this gate — `bare` 21.1, `fresh` 25.5, `committed-singletons` 9.6 (gate) — where the pilot's stop measured **32.0 s** for 49 cells in the gate of `B4` at a load of 11 to 26: **about 24 s more**, at loads not measured alike. Cell by cell: the family's 89 cells summed about 235 cell-seconds inside the gate, which a pool of ten runs in **about 24 s** of wall time — the same number two ways. The route catalog's nine arms, in the same one-at-a-time chain, were 397 s. The gate's second tier ran 1,206 s, against 1,762 s and 1,515 s for the two gates before it on this day, each beside another lane's gate.

**The running total against the projection** (the pilot's record → §7, which projected from one family at 18.5 gate cells a leaf and 1.70 cell-seconds alone, 3.4 inside a gate):

| | gate cells | a leaf | cell-seconds inside a gate | wall time in the chain, a pool of ten |
|---|---:|---:|---:|---:|
| the pilot's projection for this family's four leaves | 74 | 18.5 | 252 | 25 s |
| **the family, measured** | **89** | **22.3** | **≈ 235** | **≈ 24 s** |
| the table now — six leaves with cells, and `milestone finalize`'s acceptance | 138 | 22.5 | ≈ 415 (gate) | 56 s (gate) |
| the 24 leaves still to come under the ruling, at this table's 22.5 a leaf · 3.0 cell-seconds | ≈ 540 | 22.5 | ≈ 1,620 | ≈ 2.7 min |
| — the eight `milestone` leaves at a fan-out's 9.0 cell-seconds (gate) in place of 3.0 | (180 of them) | | + ≈ 1,080 | + ≈ 1.8 min |
| **the whole table under the ruling, projected** | **≈ 680** | | **≈ 3,100** | **≈ 5.2 min** |

*Arithmetic over measured numbers, not a measurement.* The family drew more cells a leaf than the projection (22 against 18.5) and cost about the same a cell inside a gate (2.6 against 3.4), so its wall time came out where the projection had it. The pilot's projection for the whole table was 3.1 min in a gate, 4.3 with the `milestone` leaves at a fan-out's rate; this one is about 5.2.

## 3. The questions the family met

*Ruled on 2026-10-15, each option A — the human's rulings are recorded in [DECISIONS.md](../../../../../DECISIONS.md) → the entry of 2026-10-18, *The human's seven rulings on the install family's questions, recorded*, and the rows' record's §6 says under each entry how it is carried. What follows is the questions as they stood on 2026-10-15.*

The record's §6, each leading with what the product should do, as the human's rule of 2026-10-14 asks:

- **Entry 1, its first open point** — a `setup` that exits 0 makes `.jigc/state/`, which no sentence names: 20 cells marked, 14 red for it alone.
- **Entry 3** — the install-footprint record of a failed `setup`, seen at `.jigc/state/setup-install-footprint` in all twelve late failures, and excused there by `C2`.
- **Entry 9** — whether `unmanage` commits: met by no cell — it committed nothing in any of its ten.
- **Entry 10** — a file the user committed at a path `setup` replaces: 3 cells marked, red.
- **Entry 11** — `setup`'s install commit runs no hook: no cell red, none marked.
- **Entry 12** — a link at the root `.gitignore` followed on an unborn `HEAD`: 1 cell, red, under its own key.

## 4. Not established

- **That the rows are complete.** `setup`'s and `uninstall`'s are the first cut's; `ingest`'s and `unmanage`'s were read from `design/project-setup.md` alone.
- **Thirteen needs of `setup`** wait (scheduled), each with its reason.
- **A cell's seconds on a quiet machine, and on Linux**: no run in the runner-shaped container was made for the family.
- **That the refusals' words are the right ones** beyond the run each was read off.
