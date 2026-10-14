# The byte table's pilot — its numbers at the stop

*Written 2026-10-13 with the last commit of the plan's task `B4` ([byte-table-plan.md](byte-table-plan.md) → §6, §8), on the branch `fix/rc24-route-catalog`. A record: it counts what the pilot's first cut built and what it costs, and it **decides nothing** — not which families follow, not what *touched* means, not what share of the gate the table may take. Those three are the human's, at this stop. What each piece is and why stands in [DECISIONS.md](../../../../../DECISIONS.md), under the entries of the tasks `B0` to `B4`; the findings are [byte-table-findings.md](byte-table-findings.md), the questions of reading [byte-table-readings.md](byte-table-readings.md), the rows' record [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md), the count [byte-table-leaves-touched-since-rc24.md](byte-table-leaves-touched-since-rc24.md).*

*Changed 2026-10-15, with the first commit of the install family's task (`B5`), which records the human's rulings at this stop ([DECISIONS.md](../../../../../DECISIONS.md), the entry of that date). This record stays the stop's; **what the rulings moved, recomputed**: the rows hold **63** lines, not 62 — `uninstall` gains `U9` — of which **42** are named as excusing nothing, not 41; `uninstall` needs **21** cells, not 18, so the five rows need **200**, of which 46 are answered by a cell, 14 scheduled and **140** owed whole (the 139 of §1's table was a slip for 137); **two** cells carry a mark — bound 1 and bound 2 — where five carried eight (§5's table: what *held* means and silences 6 and 8 are ruled, and their marks are gone); and no verdict *hangs on what held means* any more — the three cells §2 lists so are *lost*, as they were, under the ruled reading. **No cell's classification moved on either binary**: the arm on a named binary, run again on 2026-10-15 on the previous release's binary, prints the pairs of §2 with the three *hangs* rows read as *lost*. The table beside this file is the stop's print and is not printed again here.*

**The two tables committed beside this file are the arms' own bytes.** [byte-table-pilot-cells.tsv](byte-table-pilot-cells.tsv) is the block `cells` of `byte_table::the_cells_are_run_on_the_binary_a_variable_names` as one run printed it on 2026-10-13 — every cell on the previous release's binary (`1.0.0-rc.24`, built from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run) and on this tree's debug build, each on a copy of the state that binary built, with each cell's seconds. The run took 88.6 s at a load average of 9.9 at its start and 6.6 at its end, with no gate beside it; a run before it, at a load of 38 to 51 and beside the arms of this suite, took 278 s and printed every classification, every exit and every count of what a line excused the same. [byte-table-on-demand-cells.tsv](byte-table-on-demand-cells.tsv) is the block of the arm of the cells on demand, from the commit before this one. A classification is what one binary did on one day; the seconds are that run's and no other's.

Every number names its source: **(suite)** — an assertion or a list of [byte_table.rs](../../../../../crates/cli/tests/byte_table.rs); **(arm)** — a block of the arm on a named binary, in the run above; **(gate)** — the timing record of a gate of this branch, `target/jigc-gate/test-seconds.tsv`, and the file a prepared state's arm leaves its cells' seconds in; **(alone)** — a run by hand with no gate of this branch beside it. macOS arm64 throughout, but for §7's last line.

---

## 1. What stands in the tree

| | | source |
|---|---:|---|
| rows — a write leaf each | 5: `setup`, `uninstall`, `task finalize`, `task discard`, `milestone finalize` | (suite) |
| lines of the five rows | 62 | (suite) |
| — that excuse a difference in some cell the gate runs | 21 | (suite) `EXCUSED_IN` |
| — named as excusing nothing, each with what would use it | 41 | (suite) `EXCUSES_NOTHING` |
| cells | **61** | (suite) |
| — in the gate, in the arms of three prepared states | 49: `bare` 2 · `fresh` 41 · `committed-singletons` 6 | (suite) |
| — on demand | 12 | (suite) |
| write leaves with no row yet, owed by a task of the plan | 25 | (suite) `OWED` |
| write leaves scheduled — untouched under the plan's rule | 6 | (suite) |

**Cells by layer** (the plan's §3; a cell's layer is derived by the frame):

| layer | the acceptance | `task finalize` | `task discard` | all |
|---|---:|---:|---:|---:|
| refusing — the argv that resolves nothing, in `fresh` | — | 1 | 1 | 2 |
| base, in the leaf's home state | — | 1 | 3 | 4 |
| base, in the six other states — **on demand** | — | 6 | 6 | 12 |
| planted | 12 | 21 | 10 | 43 |
| **all** | 12 | 29 | 20 | **61** |

**Cells by prepared state**: `bare` 4 (2 of them on demand) · `fresh` 41 · `committed-singletons` 8 (2 on demand) · `migrated`, `refs-post-hoc`, `chatty-hooks`, `vendored` 2 each, all on demand.

**Planted cells by the kind of plant, at its position** (the plan's letters; 43 — (arm), the block `needs`):

| plant | cells | aimed at the line |
|---|---:|---|
| **U** — an untracked file | 8 | `S1` × 2 · `M2` · `T1` × 2 · `T5` × 2 · `A7` |
| **T** — a committed file | 2 | `M2` · `T1` |
| **L** — a link: to nothing · to a file in the repository · to one outside it | 1 · 1 · 1 | `T1` |
| **K** — a directory in a file's place · a file in a directory's place | 1 · 1 | `T1` |
| **E** — an edit: before the unit's first write · after it | 2 · 5 | `T1` × 2 · `T1` × 3, `T2`, `A5` |
| **S** — staged: where the command writes · where it does not | 4 · 2 | `T1` × 2, `T2`, `A5` · `T4`, `A6` |
| **N** — a name git quotes | 2 | `T4` · `A6` |
| **D** — a file the product did not write, in a directory the command removes | 10 | `U1` × 4 · `T6` × 2 · `A1` × 4 |
| **H** — a hook that rejects | 2 | `T4` · `A6` |
| **W** — the user's own worktree, its directory moved away | 1 | `M7` |
| **M** — a user's line in a file jigc merges into | 0 | no line of the two families' rows that have cells merges |

Two of the planted cells also run their own command once before it, as a lead that must refuse.

**What the five rows need a cell for**, by the fence's own reading of their lines ((arm), the block `needs`; a need is the refusing cell, the base cell, the base cell of each other state, or a plant of one kind aimed at one line):

| row | needs | answered by a cell | scheduled, with a reason | owed whole |
|---|---:|---:|---:|---|
| `setup` | 86 | 1 | — | by `B5` |
| `uninstall` | 18 | 1 | — | by `B5` |
| `task finalize` | 33 | 26 | 7 | — |
| `task discard` | 22 | 15 | 7 | — |
| `milestone finalize` | 38 | 3 | — | by `B7` |
| **all** | **197** | 46 | 14 | 139 |

The family's 49 cells answer 40 of its 55 needs — one more is answered by a cell of the acceptance alone, and 14 wait — and nine of them are a second cell at a need: a second place for **D**, a command under its consent, a command run again, the other shape of a staged change and of an edit before the first write, two more base cells of `task discard`.

---

## 2. Red cells — by classification, by key, on both binaries

((arm), the blocks `cells` and `pairs`; (suite) for the list.)

| | cells | hold | refuse, nothing changed | red |
|---|---:|---:|---:|---:|
| **this tree** | 61 | 27 | 29 | **5** — 4 *lost*, 1 *refused, and changed what a refusal may not* |
| — the acceptance | 12 | 1 | 11 | 0 |
| — `task finalize` | 29 | 14 | 12 | 3 |
| — `task discard` | 20 | 12 | 6 | 2 |
| **the previous release** | 61 | 24 | 17 | **20** — 11 *lost*, 3 *git's state moved outside its allowance*, 5 *not the listed outcome*, 1 *refused, and changed what a refusal may not* |
| — the acceptance | 12 | — | 3 | 9 |
| — `task finalize` | 29 | 12 | 8 | 9 |
| — `task discard` | 20 | 12 | 6 | 2 |

**Keys on this tree: five, all new, a cell each**; of the five cells **two stand on a declared bound** and four carry a mark for an open question, one of them both (§5). Keys on the previous release: the six of the acceptance, as the review of that release has them, and no key is minted for a cell that is red only there.

| key | the cell | this tree | the previous release |
|---|---|---|---|
| `r1-task-finalize-a-hand-edit-before-the-tasks-first-write-is-replaced-by-its-write-to-the-same-part` | `task-finalize-a-hand-edit-before-the-first-write-to-the-same-part` | *lost* — exit 0 | *not the listed outcome* — it refuses, exit 3, and the line stands |
| `r1-task-finalize-a-change-staged-at-an-updated-doc-and-not-in-the-file-is-replaced` | `task-finalize-a-change-staged-at-the-updated-doc-and-not-in-the-file` | *lost*, hangs on what *held* means — exit 0 | the same |
| `r1-task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file-is-replaced` | `task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file` | *lost*, hangs — exit 0 | the same |
| `r1-task-discard-a-change-staged-at-the-record-and-not-in-the-file-is-replaced` | `task-discard-a-change-staged-at-the-record-and-not-in-the-file` | *lost*, hangs — exit 0 | the same |
| `r1-task-discard-a-rejected-record-commit-leaves-its-message-file-in-the-working-area` | `task-discard-a-hook-that-rejects` | *refused, and changed what a refusal may not* — exit 1 | the same |

**Every pair of outcomes** — the previous release's, then this tree's — with how many cells came to it:

| the previous release | this tree | cells |
|---|---|---:|
| git's state moved outside its allowance | holds | 1 |
| git's state moved outside its allowance; also not the listed outcome | refuses, nothing changed | 1 |
| git's state moved outside its allowance; also written outside its allowance; also not the listed outcome | refuses, nothing changed | 1 |
| holds | holds | 24 |
| lost | holds | 1 |
| lost (hangs on what *held* means) | lost (hangs on what *held* means) | 3 |
| lost (hangs on what *held* means); also not the listed outcome | refuses, nothing changed | 1 |
| lost; also not the listed outcome | refuses, nothing changed | 1 |
| lost; also written outside its allowance; also not the listed outcome | refuses, nothing changed | 5 |
| not the listed outcome | holds | 1 |
| not the listed outcome | lost | 1 |
| not the listed outcome | refuses, nothing changed | 3 |
| refused, and changed what a refusal may not | refused, and changed what a refusal may not | 1 |
| refuses, nothing changed | refuses, nothing changed | 17 |

**What a red cell shows beside the class it is listed under** — the builder before this one left it unseen. On this tree, nothing: each of the five is red in one class. On the previous release 9 of the 20 show a second or a third: *written outside its allowance* beside a loss in five, and beside *git's state moved* in one — a file no line covers, in the cells where that release acted over a plant — and *not the listed outcome* wherever a cell that names its refusal met a command that acted.

---

## 3. The acceptance — unchanged

The twelve cells of the plan's §5 come out as [byte-table-findings.md](byte-table-findings.md) → §3 has them, on both binaries, with one thing more: each of the eleven that refuse on this tree now **names** its refusal, and is held to it. On the previous release the three that refuse there refuse with the refusal the cell names.

---

## 4. What each line excused

((arm), the block `excused`: the differences each line of each row excused, over every cell of its leaf — the previous release's binary, then this tree's. Lines that excused nothing on either are left out here and are in the suite's list.)

| command | line | the previous release | this tree |
|---|---|---:|---:|
| `jigc setup` | `S1` | 2 | 0 |
| `jigc setup` | `S2` | 3 | 0 |
| `jigc setup` | `S3` | 4 | 0 |
| `jigc setup` | `S4` | 7 | 0 |
| `jigc setup` | `S5` | 3 | 0 |
| `jigc setup` | `S7` | 3 | 0 |
| `jigc setup` | `S8` | 5 | 0 |
| `jigc setup` | `S9` | 3 | 0 |
| `jigc setup` | `S10` | 1 | 0 |
| `jigc setup` | `S11` | 1 | 0 |
| `jigc uninstall` | `U1` | 26 | 0 |
| `jigc uninstall` | `U4, removed` | 3 | 0 |
| `jigc uninstall` | `U5` | 9 | 0 |
| `jigc task finalize` | `T1` | 75 | 74 |
| `jigc task finalize` | `T2` | 1 | 1 |
| `jigc task finalize` | `T4` | 21 | 19 |
| `jigc task finalize` | `T5, the edge index` | 23 | 19 |
| `jigc task finalize` | `T5, the file-state hashes` | 31 | 27 |
| `jigc task finalize` | `T6` | 178 | 167 |
| `jigc task finalize` | `T6, moved aside` | 7 | 7 |
| `jigc task finalize` | `C1` | 24 | 29 |
| `jigc task discard` | `A1` | 72 | 72 |
| `jigc task discard` | `A5` | 30 | 30 |
| `jigc task discard` | `A6` | 10 | 10 |
| `jigc task discard` | `A7` | 10 | 10 |
| `jigc milestone finalize` | `M1` | 0 | 8 |
| `jigc milestone finalize` | `M2` | 10 | 0 |
| `jigc milestone finalize` | `M3` | 9 | 7 |
| `jigc milestone finalize` | `M5` | 9 | 3 |
| `jigc milestone finalize` | `M6, the worktrees` | 104 | 36 |
| `jigc milestone finalize` | `M6, the sub-tasks' areas` | 40 | 8 |
| `jigc milestone finalize` | `M6, the milestone's area` | 15 | 5 |
| `jigc milestone finalize` | `M7` | 12 | 4 |
| `jigc milestone finalize` | `M10, the edge index` | 2 | 2 |
| `jigc milestone finalize` | `M10, the file-state hashes` | 3 | 1 |

**The line that excuses most is `T6`** — a task's working area going at a finalize that landed: ten files a cell. **`C1` excused 29 differences on this tree, all at `task finalize`**: three files of the edge index, rebuilt by each of eight refusals of the family, and one, one and three by the acceptance's three. `C2` and `C3` excused nothing in any cell on either binary: no refusal of the 61 wrote under `.jigc/state/` or to a milestone's cache.

---

## 5. The cells each open question hangs on

([byte-table-readings.md](byte-table-readings.md) has the words and the readings; (arm), the block `marks`.)

| the question, or the bound | cells that carry its mark | red under the reading the cells follow | red under the other |
|---|---:|---|---|
| what *held* means — how far an object's reach goes (the plan's §7, question 1) | 3 | 3, *lost*: reachable (a), and reachable through a reflog (b) | 0 under (c), in the store |
| silence 2 — `setup` refuses in two ways | 0 | — | — |
| silence 5 — `uninstall`, git's index, and a commit | 0 | — | — |
| silence 6 — the rendered message's temporary file | 1 | 1, under (a): a write a refusal may not make | 0 under (b): part of the survivable frame |
| silence 7 — `task discard` and the edge index | 0 — met by no cell: no discard wrote there | — | — |
| silence 8 — a change staged at a path the command stages itself | 2 | 2, as findings, under (b): the bound is one case | 2 under (a) too, each then on a declared bound, as the third cell of the shape is |
| bound 1 — content staged for a doc a task is updating is replaced at finalize | 1 | 1 — a bound makes no cell green | — |
| bound 2 — an edit before a task's first write is carried in, and a write to the same part replaces it | 1 | 1 | — |

**The two readings the plan's review carried open.** *No git object holds*, three readings: the three cells above are what the family shows of it, and they are red under two of the three. *A hand edit that rides the commit at `migrate-corpus` and `relocate`*: the family's cells show nothing — neither command has a row.

---

## 6. The count the bound stands on

[byte-table-leaves-touched-since-rc24.md](byte-table-leaves-touched-since-rc24.md), each leaf the plan's script called untouched read against its handler:

| the reading of *touched* | touched | untouched |
|---|---:|---:|
| every changed function counts | 36 | 0 |
| **the plan's rule** — the handler reaches a changed function that not every write leaf reaches | **30** | 6 |
| only a change that moves a byte the command writes or a refusal it takes | at most 26 | at least 10 |

---

## 7. Seconds

**A cell's seconds, by what it has to build** — measured in the frame, from the copy of its prepared state to its verdict: the sandbox, the leads, three manifests, the command. This tree's debug binary. ((gate), the files the arms left; (alone).)

| the cell | cells | alone — its arm run by hand, twice, no gate of this branch beside it; load 12 to 14 | inside the gate of the commit before this one; load 11 at its start, 26 at its end |
|---|---:|---:|---:|
| `setup` or `uninstall`, no lead-in | 6 | 0.8 to 0.9 s | 0.8 to 1.6 s |
| a refusing cell, no lead-in | 2 | 0.6 s | 0.9 to 1.1 s |
| a task cell — eleven lead-ins, then `task discard` | 6 | 1.2 to 1.3 s | 2.1 to 2.6 s |
| a task cell — six to thirteen lead-ins, then `task finalize` | 23 | 1.3 to 2.0 s, mean 1.75 | 2.1 to 4.6 s, mean 3.6 |
| — one that runs its command twice | 2 | 1.6 to 2.0 s | 2.9 to 4.5 s |
| a sub-task cell — a milestone and one sub-task made, then `task discard` | 7 | 1.8 to 2.2 s, mean 2.1 | 3.3 to 4.5 s, mean 3.9 |
| a fan-out — `milestone finalize` over two provisioned sub-tasks | 3 | 4.2 to 4.7 s, mean 4.5 | 7.7 to 8.5 s, mean 8.0 |

The two runs alone agree to a tenth of a second a cell. On the arm on a named binary — 88.6 s for the 61 cells on two binaries and the six named states built once more on the named one, at a load of 10 falling to 7 — a cell of the family took 0.6 to 2.9 s on either binary, and a fan-out 4.8 to 6.0 s ((arm), the table beside this file). The twelve cells on demand, with their seven states built, ran in about a minute at a load of 36 to 42.

**The arms that run cells, as tests of the gate** — each is one test, a pool ten threads wide on this machine, and the gate's runner runs them one at a time, with the route catalog's. ((gate); (alone).)

| arm | cells | alone, load 12 to 14 — two runs | inside the gate of the commit before this one, load 11 to 26 | inside the gate of the task `B3`, before the family, load 42 to 51 |
|---|---:|---:|---:|---:|
| `bare` | 2 | 0.9 · 1.0 s | 1.0 s | 1.8 s, 2 cells |
| `committed-singletons` | 6 | 5.4 · 5.4 s — its state 3.6 s | 10.7 s — its state 7.4 s | 11.2 s, 2 cells |
| `fresh` | 41 | 10.7 · 10.8 s — its state 0.5 s | 20.2 s — its state 0.7 s | 27.6 s, 8 cells |
| **the three** | **49** | **16.9 s** | **32.0 s** | 40.6 s, 12 cells |

**What the family adds to a gate, three ways, none of them a promise.**

- *The same gate, cell by cell.* The family's 37 gate cells summed 126 cell-seconds inside that gate — 114 in `fresh`, 12 in `committed-singletons` — which a pool of ten runs in **about 13 s** of wall time; alone they sum 63, **about 6 s**.
- *Gate against gate.* The three arms were 15.8 s in the gate of the task `B2e`, with 12 cells, and 32.0 s in the last one, with 49: **16 s more**, at loads that were not measured alike.
- *Against the load.* Between the gate of `B3` and the last one the route catalog's nine arms, which did not change, went from 400.4 s to 269.6 s — the machine's load fell — while the byte table's three went from 40.6 s to 32.0 s **with 37 cells more**. Scaled by the catalog's arms, the twelve cells would have run 27 s in the last gate: the family is **about 5 s** over that.

In the last gate the byte table's 39 tests summed 48.6 s, and the three arms were 32.0 s of a one-at-a-time chain whose other part, the route catalog's nine arms, was 269.6 s. The gate's second tier ran 1,134 s; the two before it 1,475 s and 1,619 s, each beside another lane's gate.

**The whole table, projected — arithmetic over the one family, and not a measurement.**

The family is two leaves and 37 gate cells: **18.5 gate cells a leaf**, and 6 more on demand. A cell of it costs 1.70 cell-seconds alone and 3.4 inside the last gate (63 ÷ 37; 126 ÷ 37); a fan-out cell 4.5 and 8.0. The fence reads **33 gate needs a leaf** off the five rows it has (197 needs, less 6 a leaf on demand; `setup` alone is 80, the median row 27), of which the family answered two in three — 29 of its 43, with 14 waiting.

| the reading of *touched* | leaves | gate cells, at the family's 18.5 a leaf | cell-seconds alone · inside a gate like the last | wall time added to the one-at-a-time chain, a pool of ten: alone · inside such a gate | cells on demand |
|---|---:|---:|---:|---:|---:|
| every changed function | 36 | 666 | 1,132 · 2,264 | 1.9 min · 3.8 min | 216 |
| **the plan's rule** | **30** | **555** | 944 · 1,887 | **1.6 min · 3.1 min** | 180 |
| a change that moves a byte or a refusal | at most 26 | 481 | 818 · 1,635 | 1.4 min · 2.7 min | 156 |

*The arithmetic*: leaves × 18.5 = cells; cells × 1.70 and × 3.4 = cell-seconds; ÷ 10 threads = wall seconds. **Three things it leaves out, each of which makes it more.** The eight `milestone` leaves build a fan-out a cell: at the fan-out's rate their 148 cells are 662 cell-seconds alone and 1,184 in a gate where the task cell's rate gives 252 and 503 — **under the plan's rule that is 2.3 min alone and 4.3 min in a gate like the last**, and not 1.6 and 3.1. Fourteen of the family's 55 needs wait; a table that answers every need of every row has 33 gate cells a leaf and not 18.5 — **1.8 times the cells**. And a gate beside another lane's gate ran a cell two to four times slower than the last one did: the `fresh` arm was 27.6 s for 8 cells at a load of 42 to 51.

**On Linux, in the runner-shaped container.** `dev/runner-faithful --commit eba666f0 cargo test -p jigc --test g_migration byte_table::`, once, on the commit before this one: `linux/aarch64`, 8 CPUs, `rustc 1.95.0`, `git 2.43.0`, the committed revision and no uncommitted byte. **39 passed, 0 failed, 3 ignored** — the three arms that are tables — in 23.9 s, after a build of 42.6 s; the whole run took 77 s, the host at a load of 6 rising to 25. **The expected-red list is the same there**: the three arms hold it exactly, five cells under five keys, and with it what every line is listed as excusing in the cells of each state. A cell's seconds there were not read: the container keeps no file of them.

---

## 8. What a row and a cell cost to make

The orchestrating session has each task's wall time and tokens, and gives them. What the builder of `B3` and `B4` clocked itself: one read-only helper read the ten leaves of the count against their handlers — 1,625 s and 477 k tokens, apart from the builder's own; the row of `task discard` was written from its docs in one sitting — four lines of its own and six rules that are no line — with a red-first arm for the three corrected lines; the family's 49 cells were written as data in one sitting, and came out as listed on their first run but for one whose lead-in the binary refused, and five that are red.

**What the plan's pattern asked and what it took.** *The rows from the docs in a commit of their own, then the cells* held. It bent in three places, each in an entry: a cell needed to **name its refusal**, which the frame had no word for; the frame's rule for prose handed to jigc had no place for **a consent**, which `task discard --force` is; and the plan's *about 17 planted cells* for the two leaves came to 31, because a kind of plant is a letter **at a position**, and a line that names one file's place draws ten of them.

---

## 9. What the pilot's first cut does not establish

- **That the rows are complete.** Each is read from the docs named in the record; the sections not read are named there.
- **That the kinds of plant target the right lines** (`byte_cell::aimed`): the builder's reading of the plan's §3 against a vocabulary the plan had not seen. Fourteen needs of the family wait with a reason, and 139 are owed whole.
- **The two spellings** — the doc-only arm and `squash: false`: no cell names either, and the comparison has no switch for them.
- **Plant M**, a link or a directory in the place of a file that stands, a name with a line break, a git configuration, the three declared bounds as plants, two plants at once, a plant in a state that is not its leaf's home.
- **That a cache a command wrote is true**: a manifest shows that it was written.
- **A cell's seconds on a quiet machine, and on the hosted runner.**
- **`(R6, K-1)`**, the race: no state reproduces it.
- **That the on-demand cells stay as they were printed**: no gate holds them beyond a line a cell.
