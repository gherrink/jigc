# The byte table's mint family — its numbers

*Written 2026-10-20 with the last commit of the plan's task `B9` ([byte-table-plan.md](byte-table-plan.md) → §8), on the branch `fix/rc24-route-catalog`, after the human's two rulings of that day, the family's rows and its cells ([DECISIONS.md](../../../../../DECISIONS.md), the entries of 2026-10-20). **Rewritten the same day by the sixth of the fixes after the review of `B9`**, after the five fixes before it and the human's ruling on the rows' record's §6, entry 24: every count read again off the suite and a new whole-table print. A record: it counts what the family built and what it costs, and **decides nothing**. The movers family's record is [byte-table-movers-family.md](byte-table-movers-family.md); the findings are [byte-table-findings.md](byte-table-findings.md) → §26 (and §25 for the rulings' keys), the rows [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §4d, and the questions §6, entries 23 and 24, ruled, and 25, open.*

**The tables committed beside this file are the arm's own bytes, and they are the whole table.** [byte-table-mint-family-cells.tsv](byte-table-mint-family-cells.tsv) is the block `cells` of `byte_table::the_cells_are_run_on_the_binary_a_variable_names` as one run printed it — **all 760 cells** on the previous release's binary (`1.0.0-rc.24`, rebuilt from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run — not an installed `jigc`) and on this tree's debug build, each on a copy of the state that binary built, with each cell's seconds; [byte-table-mint-family-needs-and-excused.tsv](byte-table-mint-family-needs-and-excused.tsv) is the same run's blocks `needs` and `excused`, byte for byte. Printed on 2026-10-20 on the tree of the sixth fix's third commit — its cells, rows and frame as committed; no product source changed since the family began: 426 s, no gate beside it, at a load average of 23 at its start and 19 at its end. It replaces the family's first print of `5b528b33` (738 cells), which the fixes after the review each re-ran uncommitted. A classification is what one binary did on one day. **Every count of the previous release below is read off it.** It supersedes [the movers family's print](byte-table-movers-family-cells.tsv) as the table's latest, which stays that run's record.

Sources: **(suite)** — an assertion or a list of [byte_table.rs](../../../../../crates/cli/tests/byte_table.rs) and [mint.rs](../../../../../crates/cli/tests/byte_table/mint.rs); **(print)** — that run, its block `cells` and its blocks `needs` and `excused`, all committed; **(gate)** — the full gates of the sixth fix's commits, their `test-seconds` lines; **(on demand)** — the arm of the cells on demand, run by hand with the sixth fix's second commit ([byte-table-on-demand-cells.tsv](byte-table-on-demand-cells.tsv)). macOS arm64 throughout.

---

## 1. The family

| | `start` | `workflow` | `migrate` | `task amend` | all | source |
|---|---:|---:|---:|---:|---:|---|
| lines | 7 (`ST1`, `ST2`, `ST2, at a resume`, `C1`, `C2`, `C3` twice) | 6 (`WF1`, `WF2`, `C1`–`C3`) | 6 (`MI1`, `MI2`, `C1`–`C3`) | 5 (`AM1`, `C1`–`C3`) | **24** | (suite): each row's `lines` |
| spellings | 2 (`bare`, `--task`) | 1 (`--preview`) | — | — | 3 | (suite) |
| needs | 13 | 16 | 16, one scheduled | 9 | **54** | (print), block `needs` |
| cells in the gate | 9 | 11 | 9 | 4 | **33** | (suite) |
| cells on demand | 6 | 6 | 6 | 6 | **24** | (suite) |
| **red on this tree, in the gate** | **0** | **5** | **0** | **0** | **5** | (suite) `EXPECTED_RED` |
| red on demand | 0 | 0 | 0 | 0 | 0 | (on demand) |
| **red on the previous release** | **0** | **5** | **0** | **0** | **5** | (print) |
| marked cells | — | 4 (entry 25) | — | — | 4 | (suite) |

**Two keys, each the same on both binaries — not a regression:**

| key | cells | came to | which is wrong |
|---|---:|---|---|
| `r1-workflow-re-entry-takes-a-file-of-the-users-as-the-sub-tasks-commit-doc` | 4 | *not the listed outcome* — exit 0, nothing provisioned, the entry left as the sub-task's commit doc: a file of the user's, a link to a file in the repository or outside, a directory | the code; the doc owes a sentence — the human's question, entry 25 |
| `r1-workflow-re-entry-replaces-a-link-of-the-users-where-the-sub-tasks-commit-doc-goes` | 1 | *lost; also not the listed outcome* — exit 0, the link to nothing written over by jigc's file | the code, under either answer to entry 25 (the ruling on entry 23) |

**The family's doors meet two more keys, whose cells are other families'**: `r1-finalize-approve-retires-a-migration-source-whose-bytes-no-commit-holds` (four cells of the task family's `--approve`, *lost* on both binaries, ruled on 2026-10-20 — entry 24) and `r1-migrate-corpus-migrates-and-commits-a-doc-git-never-held` (two cells of the movers family's `migrate-corpus`, the sixth fix's).

And what no cell holds (the findings' §26, *Seen*): `migrate` over a directory at its source, and `task finalize --approve` over one after the authoring, each refuse in the OS's words — leads for the route catalog; a `doc` write over a link taken for the commit doc replaces the link and writes nothing outside — a lead for the doc family, closed at its source by entry 25's fix.

**What each line excused** (print, block `excused`), the same on both binaries: `ST1` 50, `ST2` 5, `ST2, at a resume` 5, `WF1` 22, `WF2` 27, `MI1` 84, `AM1` 66; `MI2` nothing — the source is read — and no `C` line anything: no refusal of the family rebuilt a cache.

**The table now** (suite): **thirty rows; 760 cells — 580 in the gate in four arms, 180 on demand**; the acceptance 12, the task family 78, `milestone finalize`'s spelling `squash: false` 4, the install family 131, the doc family 139, the milestone family 175, the movers family 164 and the mint family 57. **Red in the gate: 161 cells under forty-three keys**, in forty-five entries; on demand fourteen, as before. Needs **680, eleven scheduled**. **Marked cells: five** — `silence 25` four and `bound 1`.

**The whole table on the previous release** (print): **271 of the 760 cells are red there, where 175 are red on this tree** (161 in the gate, fourteen on demand; two *not run* there, as before). By family, both binaries: the mint 57 cells, five red on each; the movers 164, 60 and 49. **One cell red here and clean there** — `config-set-placement-root-a-link-to-nothing-at-the-roadmaps-new-home`, which rc.24 refused and this tree parks at exit 0: **the cause is `5f5b273a`** (2026-10-04), which changed `relocate::displace_foreign_squatter`'s occupancy test from `exists()` — a dangling link read as free, and `git mv` refused — to `home_entry() == Free`, under which the link is a squatter and is parked; rc.24 already routed the re-point through `relocate::relocate_stranded`. *[Corrected 2026-10-20 by the sixth of the fixes after the review: this record first gave the cause as that routing.]* `relocate-a-link-to-nothing-at-the-notes-new-home` moved by the same change, and is red on both binaries.

**What moved, print to print.** *From the movers family's print* (`002b2090`, 683 cells) *to the family's first* (`5b528b33`, 738): **eighteen cells moved** by the human's rulings of 2026-10-20 — `relocate`'s six, `docs-root`'s six and `placement-root`'s six at a new home — **fifty-five were new** (the mint family's fifty-one and four of the movers'), **and the other 665 came to the same** classification, exit, listing, mark and key on each binary. *[Corrected 2026-10-20 by the sixth fix: this record first said twenty-one and 662.]* *From the family's first print to this one*, through the six fixes after the review: twenty-two cells new (the first fix one, the second two, the fourth five, the fifth six, the sixth eight); twenty relisted by the sixth fix over their preconditions (§3); four marked with entry 25 and the four of entry 24 marked by the fifth fix and unmarked by the human's ruling; `migrate`'s directory cell listed by its source's path; and the link cells' reasons on the previous release the second fix named — no other cell moved, on either binary.

## 2. Cost

**A cell's seconds** — in the frame, from the copy of its state to its verdict (print, this tree's binary).

| leaf | cells | seconds | the gate's cells, summed |
|---|---:|---|---:|
| `start` — a task minted first for the spellings | 15 | 0.7 to 1.7, mean 1.2 | ≈ 12 |
| `workflow` — a milestone of one sub-task provisioned | 17 | 0.7 to 3.5, mean 2.7 | ≈ 33 |
| `migrate` — a foreign file committed | 15 | 0.9 to 1.8, mean 1.5 | ≈ 14 |
| `task amend` | 10 | 0.7 to 1.4, mean 1.1 | ≈ 5 |
| every cell of the family | 57 | 0.7 to 3.5, mean 1.7 | ≈ 63 |

**Inside a gate** (gate, measured): the second tier of the sixth fix's first two gates took 1,644 s and 1,585 s, at loads of 15 to 75 beside other lanes — no measurement of the family alone. *Against the first print* (≈ 56 cell-seconds for the gate's twenty-seven): the gate's thirty-three now ≈ 63, the six cells the fourth and sixth fixes added to `workflow` the most of it, each a milestone provisioned. *Arithmetic over measured numbers.*

## 3. The questions the family met, and the fixes after its review

- **The human's two rulings of 2026-10-20**, recorded by the family's first commit — §6, entry 23, stricter than its option A: any file at a mover's destination refuses; and R1: an acting sweep exits non-zero where an act it was asked for did not happen. Held at their doors by cells a whole refusal fails since the first fix (`relocate`) and the sixth (`migrate-corpus`, a refusal listed with the decision it committed).
- **Entry 24 — ruled on 2026-10-20, A**: the retire deletes the source only where `HEAD` holds its bytes; its four cells lost their mark and are plain red under their key.
- **Entry 25 — open**: what a sub-task's first re-entry does with an entry that stands where its commit doc goes. Four cells marked.
- **The words the fixes narrowed**: *into its home* (a doc git held, a regular file), *provisioned* (a file or a directory), *migrated* (a doc git held — and `migrate-corpus`, which commits a doc the user never committed, is red under a key of its own); the spelling a cell is judged under (the one its command types, declared where the spelling is).
- **The base cells elsewhere stand on their preconditions** (the review's N9): `workflow`'s and `migrate`'s five in the states neither `bare` nor their home, and `rename`'s and `relocate`'s at `B8` — twenty that refused for what was missing now meet the state; nineteen act and hold, and `rename` in `refs-post-hoc` refuses over that state's live task. `workflow` in `bare` stands on nothing: its milestone is `jigc`'s to make.
- **The open entries 4 and 7** — met by no door of this family.

## 4. Not established

- **That the rows are complete beyond what each leaf wrote when driven with nothing planted.** The four leaves were each driven once, over `fresh` with a task, a sub-task or a foreign file as their rows need, and every write was a line's; the review drove the thirteen leaf forms again and found the same.
- **The arm's seconds on a quiet machine, and on Linux**: no run in the runner-shaped container was made for the family.
