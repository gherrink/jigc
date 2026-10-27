# The byte table's movers family — its numbers

*Since 2026-10-26 the table as a whole is counted in [byte-table-record.md](byte-table-record.md), every number re-derived over all thirty rows and both binaries (the task `B10`); this record stays the count of what its family built, as of when it was written.*

*Written 2026-10-19 with the last commit of the plan's task `B8` ([byte-table-plan.md](byte-table-plan.md) → §8), on the branch `fix/rc24-route-catalog`, after the human's three rulings of that day, the family's rows and its two parts of cells ([DECISIONS.md](../../../../../DECISIONS.md), the entries of 2026-10-19). **Reconciled 2026-10-20 by the eighth of the fixes after the family's review**, after the seven before it and the human's rulings on entries 21 and 22 (the entries of 2026-10-20): every count below is read again off the suite and a new whole-table print, which replaces the family's first beside this file. A record: it counts what the family built and what it costs, and **decides nothing**. The milestone family's record is [byte-table-milestone-family.md](byte-table-milestone-family.md); the findings are [byte-table-findings.md](byte-table-findings.md) → §23 to §25, the rows [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §4c, the questions the family met §6, entries 21 to 23, and [byte-table-readings.md](byte-table-readings.md) → §20 to §22.*

*Changed 2026-10-20, by the first commit of the mint family's task (`B9`), which records the human's two rulings of that day — entry 23, stricter than its option A, and the review of `B8`'s R1: the counts below are the family's as this record reconciled them, and no longer the family's now. Since the rulings `RL3` and `CS3` are struck and the destination is a line's place; fifteen cells with an untracked occupant at a new home are listed refusing, the three marks of entry 23 are off, and four cells joined — the family is **155 cells, 131 in the gate, 46 red there under fourteen keys and 57 on the previous release, one of them red here and clean there** (the findings' §25). The mint family's record carries the counts from then on.*

**The table committed beside this file is the arm's own bytes, and it is the whole table.** [byte-table-movers-family-cells.tsv](byte-table-movers-family-cells.tsv) is the block `cells` of `byte_table::the_cells_are_run_on_the_binary_a_variable_names` as one run printed it — **all 683 cells** on the previous release's binary (`1.0.0-rc.24`, rebuilt from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run — the rebuild the earlier families used, not an installed `jigc`) and on this tree's debug build, each on a copy of the state that binary built, with each cell's seconds. Printed on 2026-10-20 on the tree of the eighth fix's first commit (its cells, rows and frame as committed with it, but one edit made after the print: how *moved aside* reads a directory's place that names no unit, which no row's line has — each line's place ends in the unit word its directory names — so no cell can see it; no product source changed since the family began): 385 s, no gate beside it, load not measured. A classification is what one binary did on one day. **Every count of the previous release below is read off it.** It replaces the print of the family's fifth commit (`29774121`, 657 cells), which git keeps; it supersedes [the milestone family's print](byte-table-milestone-family-cells.tsv) as the table's latest, which stays that run's record.

Sources: **(suite)** — an assertion or a list of [byte_table.rs](../../../../../crates/cli/tests/byte_table.rs) and [movers.rs](../../../../../crates/cli/tests/byte_table/movers.rs); **(print)** — that run; **(gate)** — the full gates of the family's commits and its fixes', their `test-seconds` lines; **(on demand)** — the arm of the cells on demand, run by hand with the family's third and fourth commits ([byte-table-on-demand-cells.tsv](byte-table-on-demand-cells.tsv)), whose cells no fix since changed. macOS arm64 throughout.

---

## 1. The family

| | `rename` | `migrate-corpus` | `relocate` | `config set` | all | source |
|---|---:|---:|---:|---:|---:|---|
| lines | 10 (`RN1`, `RN1, at its new home`, `RN2`–`RN5`, `C1`–`C3`) | 10 (`MG1`, `MG1, from a prior home`, `MG2`–`MG5`, `C1`–`C3`) | 8 | 9 | **37** | (suite): each row's `lines` |
| spellings | — | 2 (`--no-commit`, `--dry-run`) | — | — | 2 | (suite) |
| needs | 35 | 37 | 23 | 32 | **127**, none scheduled | (print), block `needs` |
| cells in the gate | 31 | 33 | 18 | 45 | **127** | (suite) |
| cells on demand | 6 | 6 | 6 | 6 | **24** | (suite) |
| **red on this tree, in the gate** | **0** | **17** | **6** | **17** | **40** | (suite) `EXPECTED_RED` |
| red on demand | 0 | 0 | 0 | 0 | 0 | (on demand) |
| **red on the previous release** | **8** | **17** | **6** | **22** | **53** | (print) |
| marked cells | — | — | 1 (entry 23) | 2 (entry 23) | 3 | (suite) |

*The cells by the commit that brought them: `rename`'s 31 and `migrate-corpus`'s 39 with the third (`a8d53b89`), `relocate`'s 23 and `config set`'s 34 with the fourth; since the review, a managed occupant at each mover (the second fix), `rename`'s six at the doc's new home (the fourth), `placement-root`'s sixteen (the fifth); the eighth relisted one cell and moved two marks.*

**Thirteen keys, every one red on the previous release too** — the forty red here come to the same words on both binaries but two, below; no cell is red here and clean there:

| key | cells | came to | which is wrong |
|---|---:|---|---|
| `r1-migrate-corpus-reads-past-a-link-or-a-directory-at-a-stale-docs-home-and-refuses-nothing` | 12 | *not the listed outcome* — exit 0, the stale doc named nowhere | the code, and the doc's premise |
| `r1-migrate-corpus-commits-an-uncommitted-edit-at-a-doc-it-migrates` | 2 | *git's state moved* — the review's `(R4, F5)` | the code; the doc silent at a touched path |
| `r1-migrate-corpus-replaces-a-change-staged-at-a-doc-it-migrates` | 2 | **lost** | the code |
| `r1-migrate-corpus-refuses-at-its-commit-and-leaves-the-migration-written-and-staged` | 1 | *refused, and changed* — unmarked since the human's ruling on entry 22 | the doc's declared bound, its echo in `design/finalize.md`, and the code |
| `r1-relocate-refuses-a-doc-and-exits-0` | 5 | *not the listed outcome* | the two docs that choose exit 0 on purpose, and the code |
| `r1-relocate-parks-a-file-the-user-committed-at-a-new-home` | 1 | *not the listed outcome*, marked *silence 23* | the reconciliation doc's settled bound, and the code — the human's (entry 23) |
| `r1-config-set-drops-a-comment-of-the-users-from-the-manifest` | 1 | *written outside its allowance* | the code |
| `r1-config-set-replaces-a-manifest-it-cannot-read-as-knobs` | 2 | **lost** | the code |
| `r1-config-set-writes-the-knob-through-a-link-at-the-manifest` | 3 | **lost** (2), *git's state moved* (1) | the code |
| `r1-config-set-re-points-past-a-link-to-nothing-or-a-directory-at-a-doc-it-strands` | 3 | *not the listed outcome* — exit 0, the doc stranded | the code |
| `r1-config-set-refuses-and-leaves-the-directories-its-re-point-made` | 2 | *refused, and changed* — on the previous release the link moved at exit 0 | the code |
| `r1-config-set-refuses-a-squatter-at-a-new-home-its-doc-parks` | 5 | *not the listed outcome* | the code |
| `r1-config-set-parks-a-file-the-user-committed-at-a-new-home` | 1 | *not the listed outcome*, marked *silence 23* | the docs that park it, and the code — the human's (entry 23) |

And four keys held by no cell, of the docs or of words: `r1-config-set-stages-a-root-knobs-moves-and-leaves-the-knob-they-follow-unstaged` and `r1-config-set-says-commit-it-with-your-next-commit-where-a-config-change-lands-with-its-task-or-on-its-own` (§23, the docs agree on what makes no sense), `r1-rename-answers-a-doc-gone-from-the-worktree-with-check-the-id-you-typed` (§24, words), and `r1-migrate-corpus-no-commit-leaves-the-migration-unstaged` (§24, the human's ruling on entry 21: a line lets a write happen and does not demand it). **Where the two binaries part** — thirteen of the family's cells, none of them a regression: `rename` over a link or a directory at the decision's home or at its new home refuses `store.home-not-regular-file` here and is *not the listed outcome* there (eight cells, red there); `config set docs-root` over a link to a file at the decision's home moves the link at exit 0 there and refuses here, leaving the directories its re-point made (two cells, red on both in different classes); `config set placement-root` over the two links to a file in the roadmap's place moves through at exit 0 there, over the link to nothing and the directory there refuses in the OS's words, and over a link to nothing at the new home meets *"fatal: destination exists"* — five cells, *not the listed outcome* there, and refusing having changed nothing, or holding, here. Each a fix of the pass the table was built to hold.

**The table now** (suite): **twenty-six rows; 683 cells — 527 in the gate in four arms, 156 on demand**; the acceptance 12, the task family 71, `milestone finalize`'s spelling `squash: false` 4, the install family 131, the doc family 139, the milestone family 175, and the movers family 151. **Red in the gate: 143 cells under thirty-eight keys**; on demand fourteen, as before. Needs 621, ten scheduled. **Marked cells: four** — bound 1, and entry 23 three times. The leaves whose rows are owed: `B9`'s four; whose cells are owed: none.

**The whole table on the previous release** (print): **254 of the 683 cells are red there, where 157 are red on this tree** (143 in the gate, fourteen on demand). By family, both binaries: the movers 151 cells, 53 red there and 40 here; the doc family 139, 25 and 25. **What moved since the family's first print** (`29774121`, 657 cells): of the 657, three cells, the same on both binaries — `migrate-corpus-no-commit-a-decision-below-its-version` and `migrate-corpus-a-hook-that-rejects` lost their marks and came out as before, and `config-set-a-file-in-the-place-of-the-decisions-directory`, listed refusing since the eighth fix, came out *not the listed outcome* where it had held; the other 654 to the same classification, exit, listing, mark and key on each binary. The 26 cells the fixes added came out as their fixes record.

## 2. Cost

**A cell's seconds** — in the frame, from the copy of its state to its verdict (print, this tree's binary).

| leaf | cells | seconds | the gate's cells, summed |
|---|---:|---|---:|
| `rename` — two docs committed by the leads | 37 | 0.7 to 2.9, mean 1.7 | ≈ 56 |
| `migrate-corpus` — a stale doc committed; `MG2`'s a task landed first | 39 | 0.7 to 4.3, mean 2.2 | ≈ 80 |
| `relocate` — a pack and a stranded note committed | 24 | 0.7 to 3.6, mean 1.7 | ≈ 35 |
| `config set` — a decision, or a roadmap, committed | 51 | 0.6 to 3.5, mean 1.7 | ≈ 81 |
| every cell of the family | 151 | 0.6 to 4.3, mean 1.8 | ≈ 251 |

**What the family adds to a gate** (gate): the arm of `fresh`, where all its gate cells run, took **120.1 s** in the gate of the family's second commit (no cell of it), **127.1 s** in the third's (58 cells more), and **138.4 s** in the fourth's (45 more, 103 in all); and **142.1 s** in the gate of the eighth fix's first commit (127 in all), at a load average near 100 — differences of runs at different loads, not measurements of the family alone. The arm runs its cells in parallel, so two hundred and fifty cell-seconds cost the chain far less than two hundred and fifty seconds.

**Against the projection** (the milestone family's record → §2: the eight leaves of `B8` and `B9` at ≈ 140 gate cells, ≈ 370 cell-seconds, + ≈ 40 s): the four leaves of `B8`, **measured**, are 127 gate cells and ≈ 251 cell-seconds since the fixes after the review — about two thirds of the eight leaves' projection for half of the leaves, and a cell cheaper than the families' 2.6 s mean. *Arithmetic over measured numbers.*

## 3. The questions the family met

- **The human's three rulings of 2026-10-19**, recorded by the family's first commit: entry 18 (a landed boundary moves a sub-task worktree's leftovers aside), entry 20 (`provision` skips a settled sub-task), and bound 2 retired. Four marks left their cells; no classification moved.
- **Entry 21, ruled 2026-10-19** — `migrate-corpus --no-commit` stages exactly the paths the migration touched and commits nothing; `design/write-commands.md` was right. `MG4` is the spelling's staging line; the one cell lost its mark and holds on both binaries; the product and doc change is a key no cell holds.
- **Entry 22, ruled 2026-10-19** — a `migrate-corpus` whose commit a hook rejects rolls back. The one cell lost its mark and stays red on both, a finding of the product and of the doc's bound.
- **Entry 23, open** — a file the user committed at a mover's new home: the design parks it on purpose, *"the settled **foreign-only** bound"* (`design/reconciliation.md`), and what makes sense for a user reverses it. *Robust*: block that doc at `relocate`, refuse the whole re-point at a root knob. Three cells, marked: `relocate`'s and `config set placement-root`'s red on both, `config set docs-root`'s refuses having changed nothing on both. *[Reframed by the eighth fix, the review's R2: it read as two docs saying two things, with one doc blocked as the answer everywhere, and `config set docs-root`'s refusal as the config door's whole answer — `placement-root` parks (the fifth fix).]*
- **Under the human's rule of 2026-10-19** — neither the docs nor the code is the oracle: two findings where the docs agree on what makes no sense (§23 of the findings), `relocate`'s exit 0 on a blocked doc a third of that class since the eighth fix (§25), and every key of §24 and §25 says which of the doc and the code it judges wrong.

## 4. Not established

- **That the rows are complete.** `design/validation.md` was read for the four doors' refusals and `design/command-output-contract.md` for their acks, not for a write a row might lack.
- ~~**`config set placement-root`**: … held by a fence of the frame, not a cell~~ — *since the fifth fix after the review, sixteen cells hold it.* ~~**`CS3` excuses nothing on this tree** — `config set` refuses every squatter~~ — *true of `docs-root` alone: `placement-root` parks, and `CS3` excuses in `fresh` (the review's R6).*
- ~~**A doc jigc never tracked, migrated, adopts no baseline** — whether a migration should baseline a hand-committed doc no doc says~~ — *answered (the review's R10)*: *"Absent-hash is not drift"* — the next read adopts it (`design/reconciliation.md` → *Baseline adoption*).
- **The arm's seconds on a quiet machine, and on Linux**: no run in the runner-shaped container was made for the family.
