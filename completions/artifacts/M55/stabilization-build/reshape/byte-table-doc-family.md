# The byte table's doc family — its numbers

*Written 2026-10-18 with the last commit of the doc family's part of the plan's task `B6` ([byte-table-plan.md](byte-table-plan.md) → §8, *What would follow*), on the branch `fix/rc24-route-catalog`, after the family's rows and its two parts of cells ([DECISIONS.md](../../../../../DECISIONS.md), the entries of 2026-10-18). A record: it counts what the family built and what it costs, and **decides nothing**. The install family's record is [byte-table-install-family.md](byte-table-install-family.md), the pilot's [byte-table-pilot.md](byte-table-pilot.md); the findings are [byte-table-findings.md](byte-table-findings.md) → §10 and §11, the rows [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §3b, the question the cells met §6, entry 14, and [byte-table-readings.md](byte-table-readings.md) → §13.*

**The table committed beside this file is the arm's own bytes, and it is the whole table.** [byte-table-doc-family-cells.tsv](byte-table-doc-family-cells.tsv) is the block `cells` of `byte_table::the_cells_are_run_on_the_binary_a_variable_names` as one run printed it on 2026-10-18 — **all 344 cells** on the previous release's binary (`1.0.0-rc.24`, rebuilt from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run — the rebuild the earlier families used, not an installed `jigc`) and on this tree's debug build, on the tree of the family's third commit (its cells and frame as committed there; no product source changed since the earlier prints), each on a copy of the state that binary built, with each cell's seconds: 167 s at a load average of about 8 to 30, no gate beside it. A classification is what one binary did on one day. **Every count of the previous release below, and in the records that state one for a cell made since 2026-10-15, is read off it.**

**That print is that run's record, and it is not brought up to date.** It holds the 344 cells as they stood at the family's third commit, with **the marks they were printed with** — twenty-six marked cells, all twenty-four of the family's with entry 14 — and not what the fixes after the family's review changed since: the marks re-set by the third of them (twenty-three marked cells, thirty-eight lines; *The table now*, below) and the cell the fourth added beside a sibling task, `doc-add-a-milestone-beside-a-sibling-task-that-stages-the-roadmap`. The fourth fix ran the same arm again on both binaries (2026-10-18, 168 s, the print not committed): **every one of the 344 came to the classification and exit the committed print has, on each binary**, and the new cell holds on both — **no committed run verifies that last**.

Sources: **(suite)** — an assertion or a list of [byte_table.rs](../../../../../crates/cli/tests/byte_table.rs); **(print)** — that run; **(gate)** — the gate of the family's third commit, its timing record `target/jigc-gate/test-seconds.tsv` and the files the arms leave their cells' seconds in, at a load average of 32 to 38 while its second tier ran; **(on demand)** — the arm of the cells on demand, run by hand at a load of 8 to 24. macOS arm64 throughout.

---

## 1. The family

| | the six edit leaves | `create` | `author` | all | source |
|---|---:|---:|---:|---:|---|
| lines of a row | 7 (`rename` 8) | 7 | 7 | 57 | (suite) |
| needs | 17 a leaf | 17 | 17 | 136 — the cell beside a sibling task answers `add-item`'s *base* a second time | (print), block `needs` |
| — answered by a cell | all | all | all | 136 | |
| — scheduled | — | — | — | 0 | `scheduled-cells.tsv` |
| cells in the gate | 67 — 66, and the cell beside a sibling task since the fourth fix after the review | 11 | 11 | 89 | (suite) |
| cells on demand | 36 | 6 | 6 | 48 | (suite) |
| **red on this tree, in the gate** | **18** | **3** | **3** | **24** | (suite) `EXPECTED_RED` |
| red on demand | 0 | 0 | 0 | 0 | (on demand) |
| **red on the previous release** | **18** | **3** | **3** | **24** — the same cells, in the same class | (print) |

**Three keys, every one red on the previous release too**, cell for cell — the family's 136 cells come to the same words on both binaries, and none is red there that is not red here:

| key | cells | came to |
|---|---:|---|
| `r1-doc-write-acts-over-a-home-that-is-no-doc-of-its-type` | 6 | *not the listed outcome* — `add-item` and `create` copy in the file that stands at the home and act on it, exit 0 |
| `r1-doc-write-refuses-over-a-home-that-is-no-doc-of-its-type-and-leaves-its-copy-staged` | 15 | *refused, and changed what a refusal may not* — five edit leaves |
| `r1-doc-author-refuses-over-a-home-that-is-no-doc-of-its-type-and-leaves-its-role-bound` | 3 | *refused, and changed what a refusal may not* |

All 24 stand at the home a write copies in from, with a file there that is no doc of its type — the user's own file where the committed doc stood, or the file a live link there leads to — and each carried the mark of the rows' record's §6, entry 14 when the print was made. *Since the third of the fixes after the family's review (2026-10-18)*: entry 14 is split in two, and the six of `add-item` and `create` carry entry 14's mark, the fifteen that refuse after the copy entry 14's and entry 15's, and the three of `author` none; the print's column of marks is the run's, and stands as printed. **No byte of the user's is lost in any cell of the family, on either binary.** Where the home holds no doc body — a link to nothing, a directory, the file deleted — the six edit leaves refuse with nothing changed and the two minting doors mint a fresh doc, as the docs have each; a deletion after the task's first write is not looked at.

**The table now** (suite, after the four fixes that followed the family's review): fifteen rows, 134 lines; **345 cells — 261 in the gate in three arms, 84 on demand**; the acceptance 12, the task family 65, the install family 131, the doc family 137 (89 in the gate, 48 on demand). **Red in the gate: 68 cells under sixteen keys**; on demand four, the `uninstall` cells by `U9`. Scheduled needs: six. Marked cells: twenty-six — two bounds and twenty-four of entry 14; *since the third of the fixes after the family's review*, twenty-three — two bounds, six of entry 14 alone and fifteen of entries 14 and 15 — in thirty-eight lines of marks. The leaves whose rows are owed: the fifteen of `B7` to `B9`; whose cells are owed: `milestone finalize`.

**The whole table on the previous release** (print): **125 of the 344 cells are red there, and two did not build there — 127 not clean** — where 72 are red on this tree (68 in the gate, four on demand); the two are an edit to and a deletion of the settings record, which that release does not write in this repository. By command, the acceptance's cells with theirs: `task finalize` and `task discard`, 68 cells, 25 red there and 7 here; `setup` to `unmanage`, 137 cells, **73 red there and two not built**, and 41 here; `milestone finalize`, 3 cells, 3 and none; the eight `doc` leaves, 136 cells, 24 and 24. *[Corrected 2026-10-18 by the fourth of the fixes after the family's review: the record counted the two not built among the red — 127, and 75 for `setup` to `unmanage`.]*

## 2. Cost

**A cell's seconds** — in the frame, from the copy of its state to its verdict.

| kind | cells | in the print, this tree's binary, load about 8 to 30 | inside the gate, load 32 to 38 |
|---|---:|---:|---:|
| refusing, no task (`fresh`) | 8 | 0.7 to 0.8 s | 0.7 to 1.1 s, mean 1.0 |
| a write in `committed-singletons`, one task's start before it | 72 | 1.1 to 1.5 s, mean 1.4 | 1.3 to 2.5 s, mean 1.8 |
| `doc rename`'s committed arm — a decision landed first, twelve lead-ins | 8 | 2.9 to 3.0 s | 4.2 to 5.1 s, mean 4.5 |
| every gate cell of the family in `committed-singletons` | 80 | 1.1 to 3.0 s, mean 1.5 | 1.3 to 5.1 s, mean 2.1 |
| the cell beside a sibling task, since — four lead-ins | 1 | 1.6 s, in the fourth fix's run | not timed in a gate |
| on demand | 48 | 0.6 to 1.6 s, mean 1.2 | — |

**What the family adds to a gate.** The arm of `committed-singletons` ran **25.6 s** in the gate of the family's third commit, where it held ten cells and ran 9.6 s in the gate of `B5`'s cells ([the install family's record](byte-table-install-family.md) → §2) — **about 16 s more**, at loads not measured alike; and the eight refusing cells add under a second to `fresh`'s arm. Cell by cell: the family's 88 gate cells summed **about 172 cell-seconds** inside the gate, which a pool of ten runs in about 17 s — the same number two ways. The three arms that run cells were 78.4 s of that gate's one-at-a-time chain (`bare` 22.0, `fresh` 30.8, `committed-singletons` 25.6); the gate's second tier ran 1,017 s, beside another lane's load.

**The running total against the projection** (the install family's record → §2, which projected the whole table under the ruling at about 680 gate cells and 5.2 minutes of the chain):

| | gate cells | a leaf | cell-seconds inside a gate | wall time in the chain, a pool of ten |
|---|---:|---:|---:|---:|
| the projection for this family's eight leaves, at 22.5 a leaf · 3.0 cell-seconds | 180 | 22.5 | 540 | 54 s |
| **the family, measured** | **88** | **11** | **≈ 172** | **≈ 17 s** |
| the table now — fourteen leaves with cells, and `milestone finalize`'s acceptance | 260 | 17.7 (the families' 248 over 14) | ≈ 650 (gate) | 78 s (gate) |
| the sixteen leaves still to come under the ruling, at 17.7 a leaf · 2.6 cell-seconds | ≈ 280 | 17.7 | ≈ 740 | ≈ 1.2 min |
| — the eight `milestone` leaves at a fan-out's 9.0 cell-seconds in place of 2.6 | (140 of them) | | + ≈ 900 | + ≈ 1.5 min |
| **the whole table under the ruling, projected** | **≈ 540** | | **≈ 2,300** | **≈ 3.8 min** |

*Arithmetic over measured numbers, not a measurement.* *Counted at the family's third commit: the cell beside a sibling task, added since, makes the table 261 gate cells and the families' 249 over fourteen leaves, 17.8 a leaf — the projection moves by less than its rounding (≈ 285 for the sixteen leaves to come).* The family drew half the cells a leaf the projection had — eleven, where the task and install families drew 26 and 27 — because its rows are one row eight times with no commit line: no plant of a staged change, a name git quotes or a hook targets a line of it. Its cells cost about what the projection had a cell; its wall time came out at a third.

## 3. The question the family met

The rows' record's §6, **entry 14**, open: what a `doc` write does where its home holds a file that is no doc of its type. The robust answer the record leads with — **refuse before the copy-in, staging nothing, one rule at all eight leaves** — is the reading in force, under which all 24 red cells are red; under the other, the six that act hold and the eighteen that refuse stay red. **[Superseded 2026-10-18, by the third of the fixes after the family's review: the docs say two things there, and the entry held two questions — (a), whether the copy-in takes such a file, entry 14 still, open, over six cells and with the fifteen below; (b), whether a write that copied in and then refuses keeps the copy, entry 15, answered at `doc author` by its own doc (its three cells red under every reading, unmarked) and open at the five edit leaves (their fifteen cells carry both marks). The robust answers: refuse at the copy-in where the bytes do not parse as the doctype; and a refusal leaves the task's area as it found it.]** **[Ruled 2026-10-18 by the human, each option A — the two robust answers, taken whole: a `doc` write refuses before its copy-in where the bytes at the home do not parse as its doctype, staging and binding nothing and naming what stands there; and every `doc` write that refuses leaves the task's area as it found it. The marks leave the twenty-one cells, and no classification moves → `DECISIONS.md`, the entry of that date, *the human's three rulings*.]** The findings' §10 also records, for the run's ledger and held by no cell, what a write's refusal says where the home holds no doc body: *"no staged instance for `<address>`"*, routed at a create of a doc the committed store holds.

## 4. Not established

- **That the rows are complete.** A `doc` write's own checks (`design/validation.md`) were not read for a write a row might lack; no cell plants at a place a write reads other than the home it copies in from — a ref's target, a role's bound doc.
- **What the fixed product's refusals say** at entry 14's cells: the listed codes are today's.
- **That a `doc` write leaves another task's staged doc as it stands.** `W1` lets a write rebuild any task's area — a line names a place with one component open for the task — so the cell beside a sibling task sees a staged doc of the sibling's taken away (*lost*), and not one rewritten ([the rows' record](byte-table-acceptance-rows.md) → §3b, *What a line cannot say*). **[Said since 2026-10-18, by the task `B7`'s second commit: `W1` and `W4` reach the area of the task the command names alone, and a sibling's staged doc rewritten is *written outside its allowance* → `DECISIONS.md`, the entry of that date, *a line reaches its own unit's area*.]**
- **A cell's seconds on a quiet machine, and on Linux**: no run in the runner-shaped container was made for the family.
