# The byte table's movers family — its numbers

*Written 2026-10-19 with the last commit of the plan's task `B8` ([byte-table-plan.md](byte-table-plan.md) → §8), on the branch `fix/rc24-route-catalog`, after the human's three rulings of that day, the family's rows and its two parts of cells ([DECISIONS.md](../../../../../DECISIONS.md), the entries of 2026-10-19). A record: it counts what the family built and what it costs, and **decides nothing**. The milestone family's record is [byte-table-milestone-family.md](byte-table-milestone-family.md); the findings are [byte-table-findings.md](byte-table-findings.md) → §23 to §25, the rows [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §4c, the questions the family met §6, entries 21 to 23, and [byte-table-readings.md](byte-table-readings.md) → §20 to §22.*

**The table committed beside this file is the arm's own bytes, and it is the whole table.** [byte-table-movers-family-cells.tsv](byte-table-movers-family-cells.tsv) is the block `cells` of `byte_table::the_cells_are_run_on_the_binary_a_variable_names` as one run printed it — **all 657 cells** on the previous release's binary (`1.0.0-rc.24`, rebuilt from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run — the rebuild the earlier families used, not an installed `jigc`) and on this tree's debug build, each on a copy of the state that binary built, with each cell's seconds. Printed on the tree of the family's fourth commit (its cells, rows and frame as committed with it; no product source changed): 382 s, no gate beside it, load not measured. A classification is what one binary did on one day. **Every count of the previous release below is read off it**; it supersedes [the milestone family's print](byte-table-milestone-family-cells.tsv) as the table's latest, which stays that run's record.

Sources: **(suite)** — an assertion or a list of [byte_table.rs](../../../../../crates/cli/tests/byte_table.rs) and [movers.rs](../../../../../crates/cli/tests/byte_table/movers.rs); **(print)** — that run; **(gate)** — the full gates of the family's four commits, their `test-seconds` lines; **(on demand)** — the arm of the cells on demand, run by hand with the third and fourth commits ([byte-table-on-demand-cells.tsv](byte-table-on-demand-cells.tsv)). macOS arm64 throughout.

---

## 1. The family

| | `rename` | `migrate-corpus` | `relocate` | `config set` | all | source |
|---|---:|---:|---:|---:|---:|---|
| lines | 9 (`RN1`–`RN5`, `C1`–`C3`) | 9 (`MG1`, `MG1, from a prior home`, `MG2`, `MG3`, `MG5`, `C1`–`C3`) | 8 | 9 | **35** | (suite): each row's `lines` |
| spellings | — | 2 (`--no-commit`, `--dry-run`) | — | — | 2 | (suite) |
| needs | 29 | 37 | 23 | 32 | **121**, none scheduled | (print), block `needs` |
| cells in the gate | 25 | 33 | 17 | 28 | **103** | (suite) |
| cells on demand | 6 | 6 | 6 | 6 | **24** | (suite) |
| **red on this tree, in the gate** | **0** | **17** | **5** | **15** | **37** | (suite) `EXPECTED_RED` |
| red on demand | 0 | 0 | 0 | 0 | 0 | (on demand) |
| **red on the previous release** | **4** | **17** | **5** | **15** | **41** | (print) |
| marked cells | — | 2 (entries 21, 22) | 1 (entry 23) | 1 (entry 23) | 4 | (suite) |

*The cells by the commit that brought them: `rename`'s 31 and `migrate-corpus`'s 39 with the third (`a8d53b89`), `relocate`'s 23 and `config set`'s 34 with the fourth.*

**Twelve keys, every one red on the previous release too** — the thirty-seven red here come to the same words on both binaries but two, below; no cell is red here and clean there:

| key | cells | came to | which is wrong |
|---|---:|---|---|
| `r1-migrate-corpus-reads-past-a-link-or-a-directory-at-a-stale-docs-home-and-refuses-nothing` | 12 | *not the listed outcome* — exit 0, the stale doc named nowhere | the code, and the doc's premise |
| `r1-migrate-corpus-commits-an-uncommitted-edit-at-a-doc-it-migrates` | 2 | *git's state moved* — the review's `(R4, F5)` | the code; the doc silent at a touched path |
| `r1-migrate-corpus-replaces-a-change-staged-at-a-doc-it-migrates` | 2 | **lost** | the code |
| `r1-migrate-corpus-refuses-at-its-commit-and-leaves-the-migration-written-and-staged` | 1 | *refused, and changed*, marked *silence 22* | the doc's declared bound, and the code |
| `r1-relocate-refuses-a-doc-and-exits-0` | 4 | *not the listed outcome* | the code |
| `r1-relocate-parks-a-file-the-user-committed-at-a-new-home` | 1 | *not the listed outcome*, marked *silence 23* | the reconciliation doc, and the code |
| `r1-config-set-drops-a-comment-of-the-users-from-the-manifest` | 1 | *written outside its allowance* | the code |
| `r1-config-set-replaces-a-manifest-it-cannot-read-as-knobs` | 2 | **lost** | the code |
| `r1-config-set-writes-the-knob-through-a-link-at-the-manifest` | 3 | **lost** (2), *git's state moved* (1) | the code |
| `r1-config-set-re-points-past-a-link-to-nothing-or-a-directory-at-a-doc-it-strands` | 2 | *not the listed outcome* — exit 0, the doc stranded | the code |
| `r1-config-set-refuses-and-leaves-the-directories-its-re-point-made` | 2 | *refused, and changed* — on the previous release the link moved at exit 0 | the code |
| `r1-config-set-refuses-a-squatter-at-a-new-home-its-doc-parks` | 5 | *not the listed outcome* | the code |

And three keys held by no cell, of the docs or of words: `r1-config-set-stages-a-root-knobs-moves-and-leaves-the-knob-they-follow-unstaged` and `r1-config-set-says-commit-it-with-your-next-commit-where-a-config-change-lands-with-its-task-or-on-its-own` (§23, the docs agree on what makes no sense), `r1-rename-answers-a-doc-gone-from-the-worktree-with-check-the-id-you-typed` (§24, words). **Where the two binaries part**: `rename` over a link or a directory at the decision's home refuses `store.not-found` on the previous release — the read through the entry finds nothing — and `store.home-not-regular-file` here (four cells, red there); `config set` over a link to a file at the decision's home moves the link at exit 0 there and refuses here, leaving the directories its re-point made (two cells, red on both in different classes). Each a fix of the pass the table was built to hold, and none a regression.

**The table now** (suite): **twenty-six rows; 657 cells — 501 in the gate in four arms, 156 on demand**; the acceptance 12, the task family 71, `milestone finalize`'s spelling `squash: false` 4, the install family 131, the doc family 137, the milestone family 175, and the movers family 127. **Red in the gate: 139 cells under thirty-six keys**; on demand fourteen, as before. Needs 615, ten scheduled. **Marked cells: five** — bound 1, entries 21, 22, and 23 twice. The leaves whose rows are owed: `B9`'s four; whose cells are owed: none.

**The whole table on the previous release** (print): **241 of the 657 cells are red there, where 153 are red on this tree** (139 in the gate, fourteen on demand). By family, both binaries: the movers 127 cells, 41 red there and 37 here. **What moved since the milestone family's print**: no cell of the 530, on either binary, through the family's four commits; the 127 are new.

## 2. Cost

**A cell's seconds** — in the frame, from the copy of its state to its verdict (print, this tree's binary).

| leaf | cells | seconds | the gate's cells, summed |
|---|---:|---|---:|
| `rename` — two docs committed by the leads | 31 | 0.7 to 2.9, mean 1.6 | ≈ 45 |
| `migrate-corpus` — a stale doc committed; `MG2`'s a task landed first | 39 | 0.7 to 4.3, mean 2.2 | ≈ 79 |
| `relocate` — a pack and a stranded note committed | 23 | 0.7 to 2.3, mean 1.6 | ≈ 31 |
| `config set` — a decision committed | 34 | 0.7 to 1.9, mean 1.5 | ≈ 47 |
| every cell of the family | 127 | 0.7 to 4.3, mean 1.8 | ≈ 200 |

**What the family adds to a gate** (gate): the arm of `fresh`, where all its gate cells run, took **120.1 s** in the gate of the family's second commit (no cell of it), **127.1 s** in the third's (58 cells more), and **138.4 s** in the fourth's (45 more, 103 in all) — a difference of runs at different loads, not a measurement of the family alone. The arm runs its cells in parallel, so two hundred cell-seconds cost the chain far less than two hundred seconds.

**Against the projection** (the milestone family's record → §2: the eight leaves of `B8` and `B9` at ≈ 140 gate cells, ≈ 370 cell-seconds, + ≈ 40 s): the four leaves of `B8`, **measured**, are 103 gate cells and ≈ 200 cell-seconds — about half of the eight leaves' projection, for half of the leaves, and a cell cheaper than the families' 2.6 s mean. *Arithmetic over measured numbers.*

## 3. The questions the family met

- **The human's three rulings of 2026-10-19**, recorded by the family's first commit: entry 18 (a landed boundary moves a sub-task worktree's leftovers aside), entry 20 (`provision` skips a settled sub-task), and bound 2 retired. Four marks left their cells; no classification moved.
- **Entry 21, open** — `migrate-corpus --no-commit`: two docs say two things of whether the migrated paths are staged. *Robust*: stage exactly the touched paths. One cell, marked; it holds on both binaries.
- **Entry 22, open** — a `migrate-corpus` whose commit a hook rejects: a builder's declared bound against the human's ruling on `setup` and `rename`'s restore. *Robust*: roll back. One cell, marked; red on both.
- **Entry 23, open** — a file the user committed at a mover's new home: parked (`design/reconciliation.md`) or the move blocked (`design/corpus-migration.md`). *Robust*: blocked. Two cells, marked; `relocate`'s red on both, `config set`'s refuses having changed nothing on both.
- **Under the human's rule of 2026-10-19** — neither the docs nor the code is the oracle: two findings where the docs agree on what makes no sense (§23 of the findings), and every key of §24 and §25 says which of the doc and the code it judges wrong.

## 4. Not established

- **That the rows are complete.** `design/validation.md` was read for the four doors' refusals and `design/command-output-contract.md` for their acks, not for a write a row might lack.
- **`config set placement-root`**: the rows hold both root knobs, and every cell sets `docs-root`; a placement doctype's file moved under `placement-root` is held by a fence of the frame, not a cell.
- **`CS3` excuses nothing on this tree** — `config set` refuses every squatter where its doc parks it; the line is held by `relocate`'s twin and the fence, not a cell of its own.
- **A doc jigc never tracked, migrated, adopts no baseline**: `MG2` is held over a decision a task landed; whether a migration should baseline a hand-committed doc no doc says.
- **The arm's seconds on a quiet machine, and on Linux**: no run in the runner-shaped container was made for the family.
