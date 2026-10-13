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

## 2. The acceptance's cells — none is red on this tree's binary

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

**A cell that refuses is held to refusing, not to which refusal** — the frame has no word for the code. The codes above were read off each cell's output by the builder: each is the refusal the row's doc means, and none is a refusal for want of a lead-in. What the same cells come to on the previous release's binary — where nine of them act — is §3.

## 3. The acceptance — the twelve cells on the previous release's binary, beside this tree's

*2026-10-13; the task `B2e`. The arm is `byte_table::the_cells_are_run_on_the_binary_a_variable_names`, run by hand with `JIGC_BYTE_TABLE_BINARY` naming the binary built from the previous release's commit `91834b5e` — `1.0.0-rc.24`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked before the run — each cell on a copy of the state **that binary built**. macOS arm64; this tree's binary is its debug build. The arm counts and prints; it does not say that a difference between two binaries is a regression.*

**Every line comes out as the plan's §5 reads it** ([byte-table-plan.md](byte-table-plan.md) → §5): on the previous release the table shows each of the six blocking rows the pilot was built to catch, under the classification the plan names, and on this tree it shows each of them gone. The key is the row's, as the review of that release and the fix pass's record have it ([fix-pass-rc25/README.md](../../fix-pass-rc25/README.md)).

| the row | the cell | on the previous release | on this tree | as the plan's §5 reads the two | the same |
|---|---|---|---|---|---|
| `(R1, F1)` | `setup` · never adopted, `HEAD` unborn · an untracked file at `.jigc/AGENT.md` | ***lost*** — exit 0 | *refuses, nothing changed* — exit 1 | *lost* · *refuses, nothing changed* | yes |
| — its condition's other side | the same, `HEAD` born | *refuses, nothing changed* — exit 1 | *refuses, nothing changed* — exit 1 | *refuses, nothing changed*, twice | yes |
| `(R3, F7)` | `task finalize` · `committed-singletons`, no file-state baseline · a line added to `VISION.md` after the task's first write | ***lost*** — exit 0 | *refuses, nothing changed* — exit 3 | *lost* · a refusal, its new files under `.jigc/index/` covered or not by the plan's second question | yes — the question is ruled, and the files are covered (`C1`) |
| — its condition's other side | the same, the baseline present | *refuses, nothing changed* — exit 3 | *refuses, nothing changed* — exit 3 | *refuses*, twice | yes |
| `(R6, D-1)`, the untracked half | `milestone finalize` · `fresh` · an untracked file at the id the join numbered onto | ***lost*** — exit 0 | *refuses, nothing changed* — exit 3 | *lost* · a refusal, `merged/` covered | yes |
| `(R6, D-1)`, the committed half | `milestone finalize` · `fresh` · a committed file at that id, from before the milestone | ***git's state moved outside its allowance*** — exit 0 | *refuses, nothing changed* — exit 3 | *git's state moved outside its allowance* · a refusal | yes |
| `(R6, D-7)` | `task finalize` · `fresh` · a link to nothing at the created doc's home | ***git's state moved outside its allowance*** — exit 0 | *refuses, nothing changed* — exit 3 | *git's state moved outside its allowance* · a refusal | yes |
| `(R9, F5)` | `uninstall` · `fresh` · a file jigc did not write under `.jigc/logs/` | ***lost*** — exit 0 | *refuses, nothing changed* — exit 1 | *lost* · *refuses, nothing changed* | yes |
| `(R9, F5)` | — under `.jigc/state/` | ***lost*** — exit 0 | *refuses, nothing changed* — exit 1 | the same | yes |
| `(R9, F5)` | — under `.jigc/index/` | ***lost*** — exit 0 | *refuses, nothing changed* — exit 1 | the same | yes |
| — the same plant one directory up | — at `.jigc/`'s root | *refuses, nothing changed* — exit 1 | *refuses, nothing changed* — exit 1 | *refuses, nothing changed* · *not run by anybody* | yes, and the second is now run |
| `L-22` | `milestone finalize` · `fresh` · the user's own linked worktree, its directory moved away | ***git's state moved outside its allowance*** — exit 0 | *holds* — exit 0 | *git's state moved outside its allowance* · *holds* | yes |

**Nine of the twelve are red on the previous release, under six keys, and none is red on this tree**: six *lost* — a planted file gone, its bytes at no path of the sandbox and in no object within reach — and three *git's state moved outside its allowance*. The three that are not red there are the other side of a condition, twice, and the same plant one directory up.

**Why each of the three is *git's state moved*, in the frame's words on that binary:**

- *the committed half of `(R6, D-1)`* — the planted file is gone from every path of the sandbox, and an object within reach still has its bytes: the command put something else in its place. History holds what stood there, so it is no loss.
- *`(R6, D-7)`* — a commit the command made holds a link at `docs/inconsistencies/two-docs.md`, which no line declares; and the staged doc is gone from its working area and stands in no commit the command made, its bytes standing elsewhere — at the link's target, where the release wrote them through the link and said so itself (*left-out*).
- *`L-22`* — the registration `own`, of the worktree at `outside/own`, is gone.

**No verdict on either binary hangs on what *held* means.** On this tree no cell left anything under the sandbox's `tmp/`, `home/` or `outside/`, nor did the three that are not red on the previous release. **Of a red cell the frame says why in the class that outranks the others, and no more**: what else the previous release wrote in those nine — a file no line covers, beside the loss — is not in this table. **All six named states of the builder of the prepared states build on the previous release's binary** — `fresh`, `committed-singletons`, `migrated`, `refs-post-hoc`, `chatty-hooks`, `vendored` — where the plan had it established only through the shell rig.

## 4. The task family's cells — five are red on this tree's binary, under five keys

*2026-10-13, this tree's debug build, macOS arm64; the second commit of the task `B4`. The family is `jigc task finalize` and `jigc task discard`: 49 cells beside the acceptance's three of `task finalize` — 37 that the gate runs, in the arms of `fresh` and `committed-singletons`, and 12 base cells in the prepared states that are not the leaves' home, which run on demand ([byte-table-on-demand-cells.tsv](byte-table-on-demand-cells.tsv)). The suite's expected-red list names the five below, and is exact.*

**Of the gate's 37: 16 hold, 16 refuse and change nothing a refusal may not, 5 are red.** Of the 12 on demand: 10 hold and 2 refuse, none red. Each red cell is one plant at one position, and none shows a second class beside the one it is listed under.

Each row says what the cell is, what it came to, the sentence it is held against, and what the previous release's binary does with the same cell (`1.0.0-rc.24`, sha256 `accf3996…ab5d`, checked before the run). **Nothing here is a grade**, and whether a difference between two releases is anything is not this file's to say.

### `r1-task-finalize-a-hand-edit-before-the-tasks-first-write-is-replaced-by-its-write-to-the-same-part`

- *The cell.* `task-finalize-a-hand-edit-before-the-first-write-to-the-same-part`, in `committed-singletons`: a task is started; a line is added by hand to the committed `VISION.md`, in no commit; the task writes the vision's `open-questions` slot — the part the line stands in — and is finalized.
- *What it came to.* **Lost.** Exit 0, `finalized … promoted VISION.md`; the line stands at no path of the sandbox and in no object.
- *Held against.* The exit rule's first clause — *"no jigc command at exit 0 destroys bytes no git object holds"*. **It stands on a declared bound**, the second of the human's four of 2026-10-06: *an uncommitted edit made before a task first writes a doc is carried into the task, and a write by that task to the same part replaces it*. A bound takes no cell off the list.
- *The previous release.* **It does not show it.** The same cell refuses there — `reconciliation.conflict-block`, exit 3, *an external edit and this task's staged writes both changed it* — and the line stands.
- *Beside it.* `task-finalize-a-hand-edit-before-the-first-write-to-another-part` holds on this tree: the line is carried into the task and stands in the commit. The previous release refuses that cell the same way.

### `r1-task-finalize-a-change-staged-at-an-updated-doc-and-not-in-the-file-is-replaced`

- *The cell.* `task-finalize-a-change-staged-at-the-updated-doc-and-not-in-the-file`, in `committed-singletons`: after the task's last write, a line is added to `VISION.md`, staged, and the working file put back as it was.
- *What it came to.* **Lost**, and it hangs on what *held* means: exit 0, the staged bytes are in the index no longer, and in the store with nothing reaching them.
- *Held against.* The clause; and **it stands on a declared bound**, the first of the four: *content staged for a doc a task is updating, while the working file does not carry it, is replaced at finalize*.
- *The previous release.* The same: *lost*, exit 0.

### `r1-task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file-is-replaced`

- *The cell.* `task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file`, in `fresh`: the same act at `.jigc/version`, last before the finalize of a task that created a decision.
- *What it came to.* **Lost**, hanging on what *held* means: exit 0, *1 file committed* — the decision — and the index entry of `.jigc/version` is no longer what the user staged.
- *Held against.* The clause; and `T3`'s sentence, *"This phase stages **jigc's own** managed artifacts into that existing index"* — what the user staged there was not jigc's own. No declared bound names this path: the record's §6, entry 8, has the two readings.
- *The previous release.* The same: *lost*, exit 0.
- *Beside it.* A hand edit to the stamp **in the working file** is left alone on this tree — *jigc left it untouched and out of this commit* — where the previous release replaced it at exit 0: *lost* there, *holds* here.

### `r1-task-discard-a-change-staged-at-the-record-and-not-in-the-file-is-replaced`

- *The cell.* `task-discard-a-change-staged-at-the-record-and-not-in-the-file`, in `fresh`: a milestone with one sub-task nobody re-entered; a line is added to the milestone's committed record, staged, and the working file put back; the sub-task is discarded.
- *What it came to.* **Lost**, hanging on what *held* means: exit 0, `record commit: …`, and the staged bytes are in the index no longer.
- *Held against.* The clause; and `A6`'s sentence — the record's commit *"stages and commits **only the record file** … never sweeping the agent's in-flight index/worktree WIP"*: the staged change was index work in flight, at the one path the door stages. A hand edit **in the working file** is refused at the same door (`reconciliation.conflict-block`), on both binaries. The record's §6, entry 8.
- *The previous release.* The same: *lost*, exit 0.

### `r1-task-discard-a-rejected-record-commit-leaves-its-message-file-in-the-working-area`

- *The cell.* `task-discard-a-hook-that-rejects`, in `fresh`: the same sub-task, and the user's own lines in the `pre-commit` hook, which reject every commit.
- *What it came to.* **Refused, and changed what a refusal may not.** Exit 1, *nothing was committed — the milestone record still names task:do-the-thing as it did, and the task's working area is intact*; and `.jigc/tasks/do-the-thing/record-commit-msg.txt` was made.
- *Held against.* `A8`'s sentence — *"a rejected `pre-commit` hook leaves the record *and* the task exactly as the frame says"* — and `design/finalize.md` → *6. Commit*: *"a temp file, deleted after"*. The record's §6, entry 6, has the two readings: the file may be part of *the survivable frame*, which no doc describes.
- *The previous release.* The same.
- *Beside it.* `task-finalize-a-hook-that-rejects` leaves no file.

### What the same cells show the other way — red on the previous release, and not on this tree

Seven cells of the family come out otherwise on the previous release, beside the acceptance's three — six of them red there and not here, and one red on both, differently: a hand edit to `.jigc/version` (*lost* there, *holds* here) · a file staged at the created doc's home and not in the tree (*lost* there, refused here) · a link to a file in the repository, a link to one outside it, and a directory at the created doc's home, each refused there with another refusal than the one the cell names (`finalize.promote-clobber` twice; an unnamed *could not read … Is a directory* at exit 1) and refused here with `store.home-not-regular-file` · and the two cells of the hand edit before the first write, which that release refuses and this tree does not — one of them the first key above, red here as a loss. The arm counts; it does not say which of these is a regression.

## 5. Seen by hand, and no cell

*Each was read off a cell's output or a probe of the builder's on 2026-10-13; none is a difference of bytes, so none is a cell's to hold, and each is the route catalog's kind of finding.*

- **`jigc task discard <id>` over a task that is not there answers `finalize.no-task`** — a code of another command. Exit 1, *no task `a-task`*, nothing changed. The cell names the refusal by its sentence.
- **`jigc task finalize` over a file where the created doc's directory goes exits 1 with no finding**: *could not create "<sandbox>/repo/docs/decisions" to promote into: File exists (os error 17)* — an operating-system error, an absolute path, and no code — where a link or a directory at the doc's own home is `store.home-not-regular-file` at exit 3. Nothing is changed but the edge index, rebuilt.
- **A `doc create` over a file the user committed at the minted id takes the file up as the doc's base, and a write of the task to it is then refused** (`write.non-reparseable`, seen at `doc set-slot`) when the file is no doc of that type. The finalize refuses on the file's conformance (`conformance.section-missing`), and the file stands.
