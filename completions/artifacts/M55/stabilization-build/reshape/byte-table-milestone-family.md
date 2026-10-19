# The byte table's milestone family — its numbers

*Written 2026-10-19 with the last commit of the milestone family's part of the plan's task `B7` ([byte-table-plan.md](byte-table-plan.md) → §8, *What would follow*), on the branch `fix/rc24-route-catalog`, after the family's rows and its three parts of cells ([DECISIONS.md](../../../../../DECISIONS.md), the entries of 2026-10-19). A record: it counts what the family built and what it costs, and **decides nothing**. The doc family's record is [byte-table-doc-family.md](byte-table-doc-family.md), the install family's [byte-table-install-family.md](byte-table-install-family.md); the findings are [byte-table-findings.md](byte-table-findings.md) → §17 to §19, the rows [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §4 and §4b, the question the cells met §6, entry 18, and [byte-table-readings.md](byte-table-readings.md) → §17.*

**The table committed beside this file is the arm's own bytes, and it is the whole table.** [byte-table-milestone-family-cells.tsv](byte-table-milestone-family-cells.tsv) is the block `cells` of `byte_table::the_cells_are_run_on_the_binary_a_variable_names` as one run printed it on 2026-10-19 — **all 519 cells** on the previous release's binary (`1.0.0-rc.24`, rebuilt from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run — the rebuild the earlier families used, not an installed `jigc`) and on this tree's debug build, on the tree of the family's fourth commit (its cells, rows and frame as committed there; no product source changed since the earlier prints), each on a copy of the state that binary built, with each cell's seconds: 303 s, no gate beside it, load not measured. A classification is what one binary did on one day. **Every count of the previous release below is read off it**; it supersedes [the doc family's print](byte-table-doc-family-cells.tsv) as the table's latest, which stays that run's record.

Sources: **(suite)** — an assertion or a list of [byte_table.rs](../../../../../crates/cli/tests/byte_table.rs); **(print)** — that run; **(gate)** — the gate of the family's fourth commit, its timing record `target/jigc-gate/test-seconds.tsv` and the files the arms leave their cells' seconds in, at a load average of about 18 to 36 when it ended; **(on demand)** — the arm of the cells on demand, run by hand with each commit. macOS arm64 throughout.

---

## 1. The family

| | the record doors (`create`, `add-task`, `add-from-spec`) | the fan-out doors (`provision`, `execute`, `join`) | the terminals (`discard`, `finalize`'s plain spelling) | all | source |
|---|---:|---:|---:|---:|---|
| rows | 3 | 3 | 1, and a line of `finalize`'s | 7 rows, 57 lines; `finalize` 21 lines, where 18 | (suite) |
| needs | 68 (22 · 23 · 23) | 31 (11 · 10 · 10) | 26 + 42 | 167 | (print), block `needs` |
| — answered by a cell | all | all | all but two | 165 | |
| — scheduled | — | — | 2 — `M2` × *E*, before and after the first write | 2 | `scheduled-cells.tsv` |
| cells in the gate | 51 | 13 | 53 | 117 | (suite) |
| cells on demand | 18 | 18 | 12 | 48 | (suite) |
| **red on this tree, in the gate** | **8** | **0** | **4** | **12** | (suite) `EXPECTED_RED` |
| red on demand | 0 | 0 | 0 | 0 | (on demand) |
| **red on the previous release** | **21** | **0** | **18** | **39**, the twelve among them | (print) |

*The cells of `milestone finalize` counted here are the plain spelling's 37 and the carryover gate's cell under `squash: false`, less the acceptance's three and the spelling's four that stood before the family — `finalize`'s 37 with its second base cell and its pending config delta's.*

**Four keys, every one red on the previous release too**, cell for cell — the twelve come to the same words on both binaries:

| key | cells | came to |
|---|---:|---|
| `r1-milestone-record-doors-replace-a-change-staged-at-the-record` | 4 | **lost** — `add-task`, `add-from-spec`, `discard` and the boundary's flip write the record over the user's staged change at it |
| `r1-milestone-record-doors-leave-their-message-file-in-the-milestones-area` | 3 | *refused, and changed what a refusal may not* — a rejected record commit leaves `record-commit-msg.txt` rewritten; at exit 0 the file stands too, unseen |
| `r1-milestone-record-doors-refuse-and-leave-the-directories-they-made` | 4 | *refused, and changed what a refusal may not* — empty directories |
| `r1-milestone-finalize-takes-a-file-nobody-staged-from-a-landed-sub-tasks-worktree` | 1 | **lost**, marked *silence 18* — the design narrates it, the exit rule forbids it |

And a key of words, held by no cell: the record doors answer a record gone from the worktree with a bare error, or `milestone.area-io` and a disk-fault route.

**What the previous release shows beside them — 27 more red there, none here** (print): at the record's home and in its place a link or a directory — the record-side twin of `(R6, D-7)` — refused there with other words, or written through (`create` over a link to nothing, *git's state moved*); a change staged at the record's home before `create`, **lost** there (the completion audit's CPL-5); the user's own worktree moved away under `discard`, its registration dropped (`L-22`); and at a created doc's home under the boundary, a link written through — **lost**, the prose at the link's target — and a change staged there **lost** (`(R6, D-1)`, `(R6, D-7)`). Each a fix of the pass the table was built to hold, and none a regression.

**The table now** (suite): **twenty-two rows**, 198 lines; **519 cells — 387 in the gate in four arms, 132 on demand**; the acceptance 12, the task family 70, `milestone finalize`'s spelling `squash: false` 4, the install family 131, the doc family 137, and the milestone family 165 beside them — one of them the carryover gate's cell under `squash: false`. **Red in the gate: 80 cells under twenty keys**; on demand four, the `uninstall` cells by `U9`. Scheduled needs: eight. **Marked cells: three, in three lines** — the two bounds and entry 18. The leaves whose rows are owed: the eight of `B8` and `B9`; whose cells are owed: none.

**The whole table on the previous release** (print): **165 of the 519 cells are red there, and two did not build there — 167 not clean** — where 84 are red on this tree (80 in the gate, four on demand). By command, both binaries: the eight `milestone` leaves, 172 cells, **42 red there and 12 here** (the family's 39 and 12, and the acceptance's three there and none here); `task finalize` and `task discard`, 73 cells, 26 and 7; `setup` to `unmanage`, 137 cells, 73 red there and two not built, and 41 here; the eight `doc` leaves, 137 cells, 24 and 24.

## 2. Cost

**A cell's seconds** — in the frame, from the copy of its state to its verdict.

| kind | cells | in the print, this tree's binary | inside the gate (`fresh`, `vendored`) |
|---|---:|---:|---:|
| `create` | 22 | — | 1.3 to 4.1 s, mean 2.0 |
| `add-task` | 24 | — | 1.0 to 4.4 s, mean 3.1 |
| `add-from-spec`, in `vendored` | 17 | — | 2.7 to 4.4 s, mean 3.4 |
| `provision`, `execute`, `join` — a milestone of two sub-tasks, up to seven lead-ins | 13 | — | 0.8 to 6.8 s, mean 4.5 |
| `discard` — two sub-tasks provisioned | 22 | — | 1.0 to 8.6 s, mean 6.3 |
| `milestone finalize`, every gate cell — a fan-out of two code-carrying sub-tasks, about twenty lead-ins | 38 | — | 1.3 to 15.2 s, mean 11.6 |
| every cell of the family | 165 | 0.7 to 8.8 s, mean 3.5 | — |
| on demand | 48 | in the print, among the 165 | — |

**What the family adds to a gate.** The arm of `fresh` ran **102.6 s** in the gate of the family's fourth commit, where it ran 30.8 s in the gate of the doc family's third ([the doc family's record](byte-table-doc-family.md) → §2) — **about 72 s more**; the new arm of `vendored` 11.5 s. The 108 `milestone` gate cells in `fresh` — the family's 101 and the seven that stood — summed **about 730 cell-seconds**, about three in four of that arm's 994; and the four arms that run cells were **163.5 s** of the gate's one-at-a-time chain (`bare` 22.9, `committed-singletons` 26.6, `fresh` 102.6, `vendored` 11.5), where 78.4 s. The gate's second tier ran 1,037 s.

**Against the projection** (the doc family's record → §2: the whole table under the ruling at about 540 gate cells and 3.8 minutes of the chain, the `milestone` leaves at a fan-out's 9.0 cell-seconds):

| | gate cells | cell-seconds inside a gate | wall time in the chain |
|---|---:|---:|---:|
| the projection for the eight `milestone` leaves | ≈ 140 | + ≈ 900 at 9.0 | + ≈ 1.5 min |
| **the family, measured** | **117** | **≈ 785**, the seven that stood among them | **+ ≈ 83 s** |
| the table now | 387 | — | 163.5 s ≈ 2.7 min |
| the eight leaves of `B8` and `B9`, at the families' 17.7 gate cells a leaf and 2.6 cell-seconds | ≈ 140 | ≈ 370 | + ≈ 40 s |
| **the whole table under the ruling, projected** | **≈ 530** | | **≈ 3.4 min** |

*Arithmetic over measured numbers, not a measurement.* A fan-out cell cost about what the projection had — 6 to 12 s a cell at `discard` and the boundary — and the record doors about a third of it; the family drew fewer gate cells than projected because the fan-out doors' rows are the re-seed's two lines over directories, which only a file of the user's targets.

## 3. The question the family met

The rows' record's §6, **entry 18**, open: whether a landed milestone boundary may take a file nobody staged from a live sub-task's worktree. The design doc narrates the loss on purpose — *"visible, not prevented"*, on measured evidence against a refusal on the success path — and the exit rule's first clause has no command at exit 0 destroy bytes no git object holds. *Robust*: move what git neither ignores nor holds aside to `.jigc/displaced/<sub-task-id>/` before the removal, as the same boundary keeps a working area's complement — no refusal, no loss. *Until ruled*: the clause; one cell red and marked, the same on both binaries.

**Answered by the docs, as the rows were written**: whether a pending config delta rides the boundary (`M3`'s own *"overlays the promoted docs + first-commit config"* — `M3, the config`); what the boundary does with a change staged before `milestone create` (the carryover gate, refused — `design/finalize.md` → *Dirty-tree policy*, `design/command-output-contract.md`); whether the milestone's own area keeps its complement (`M6`'s *"Both keep their complement"* — `M6, the milestone's area moved aside`, added before its cells ran).

## 4. Not established

- **That the rows are complete.** `add-from-spec` reads the committed spec, and no line names that home or puts a plant there; and a spec of more than one criterion lands a commit a criterion, which `MS4`'s line reads against the working tree after the command — the home state's spec has one.
- **A mint of two, and a re-seed of what was absent.** `<minted>` cannot tell one mint from two, and no cell takes a sub-task's area or the milestone's cache away before an operating door, so the re-seed's six lines excuse nothing in any cell.
- **`M2` × *E***, scheduled: no prepared state the boundary's cells stand in holds a committed doc a sub-task copies in.
- **The message file at exit 0.** The record doors' areas are lines whole, so the file a landed record commit leaves is covered; the refusal cells and a repro hold the key.
- **What the fixed product's refusals say** at the cells whose words are a run's — the bare errors §17 and §19 name.
- **A cell's seconds on a quiet machine, and on Linux**: no run in the runner-shaped container was made for the family.
