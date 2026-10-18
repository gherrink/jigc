# The byte table's pilot — its findings, by key

What the byte table ([byte_table.rs](../../../../../crates/cli/tests/byte_table.rs); the frame is [byte_cell.rs](../../../../../crates/cli/tests/support/byte_cell.rs), the plan [byte-table-plan.md](byte-table-plan.md)) found: what a cell came out red with, and what the writing of its rows from the design docs found in the docs themselves. A key is in the grammar the route catalog's findings have ([route-catalog-findings.md](route-catalog-findings.md)) — `r1-<command>-<what happened, in words>` — and is what the suite's expected-red list names where a cell is under it. **Nothing here is a ruling, a grade or a word about the previous release**: a row about a cell is what one binary did on one day, and a row about the docs is two sentences set side by side.

**No ledger holds these keys.** Until a stabilization run is opened they certify themselves, as the route catalog's do.

*Changed 2026-10-18, with the first commit of the doc family's task (`B6`), for the human's rulings of 2026-10-15 on the install family's questions: §1 gains the three corrections of a design doc the rulings name and nothing else carries — a doc-only finding each; in §6 one key retires, its cells holding, one is minted, and four say how the ruling that names them is carried. What moved is said under each key.*

*Changed 2026-10-15, with the first commit of the install family's task (`B5`), for the human's rulings of 2026-10-14: §1 gains the inconsistency the ruling on `setup`'s refusals names; in §4 three keys lose the open question each was carried with — what *held* means is ruled, reading (b), and the cells stay *lost*; the record's entries 6 and 8 are ruled, and the cells they named are findings. No cell's classification moved.*

## 1. Where two design docs say two things — recorded here, and corrected by nobody in this lane

*And, since the rulings of 2026-10-15, where a design doc is to say what a ruling decided and nothing in the product changes. A ruling whose doc correction comes with a product change is recorded under the cell's key, in §6.*

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

### `r1-setup-design-stages-a-failed-install-where-the-rule-is-that-a-refusal-changes-nothing`

*Recorded 2026-10-15, as the human's ruling of 2026-10-14 on the record's §6, entry 2, asks: S12's sentence is "a design inconsistency to record … and to correct with the product's fix — not a line of the row". For the stabilization run's ledger: doctype `inconsistency`, one design doc against the rule the product is held to and a second doc's sentence.*

**One side** — `design/project-setup.md` → *Idempotency & irreversibility — the acceptance obligation* (from M54's S22), of a `setup` that fails after its first write:

> records the install footprint (M51's `record_install_footprint`) over the paths it wrote in that run, clean against `HEAD` before it and dirty at the failure, and **stages them**

**The other side** — the ruling: **every refusal of `setup` leaves the repository as it was** (caches aside); a failure after the first write rolls back every path the run wrote — each was clean against `HEAD`, absent, or an untracked file `setup` read to merge into, so its bytes before the run are known — instead of staging them; the install-footprint record keeps only the job of a run that was killed mid-write. And `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*:

> **And because the ask is pre-write, the refusal installs nothing**

**What disagrees.** The first doc declares a refusal that leaves the install written and staged; the second says a refusal installs nothing, and the ruling makes that the rule for every refusal. **What follows** is a change to the product — the rollback — and the first doc's sentence corrected with it. The cells that show it are the install family's late failures.

### `r1-assistant-adapter-design-does-not-name-the-cache-directories-setup-lays-out`

*Recorded 2026-10-18, for the human's ruling of 2026-10-15 on the rows' record's §6, entry 1, option A. For the stabilization run's ledger: doctype `inconsistency`, a doc that does not say what the product does and is ruled to.*

**The doc** — `design/assistant-adapter.md` → *Generated, minimal, regenerated* names what `setup` lays down, and no directory under `.jigc/` beyond the project layer:

> `jigc setup` also **initializes the in-repo project layer** (`.jigc/config/`) when absent

**The product** makes an empty `.jigc/state/` on every install — seen in twenty cells of the install family. **The ruling**: the product is unchanged, and the doc says that `setup` also lays out the workbench's cache directories. Until it does, the row carries the ruling's sentence as `S16`, and the twenty cells are marked off.

### `r1-write-commands-design-counts-unmanage-among-the-self-committing-ops-where-it-never-commits`

*Recorded 2026-10-18, for the human's ruling of 2026-10-15 on the rows' record's §6, entry 9, option A — *"stick with what git does"*. For the stabilization run's ledger: doctype `inconsistency`, two design docs that disagree, the product on one side.*

**One side** — `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*:

> un-manage touches only jigc's own index/state (asserted: the doc's bytes on disk are unchanged) and is idempotent (re-run on an already-unmanaged doc is a clean no-op, not an error)

**The other side** — `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*, which counts `unmanage` among *"the other top-level direct-store ops"* beside a verb that is *"task-less and self-committing"*, and → *`jigc migrate-corpus` — the commit boundary and its two opt-outs (M42)*, which says `migrate-corpus`, *"unlike them"*, has *"**never committed its own writes**"*.

**The ruling**: `unmanage` never commits; the two sentences of `design/write-commands.md` are corrected. No product change — it committed nothing in any of its ten cells.

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
- *What it came to.* **Lost**: exit 0, the staged bytes are in the index no longer, and in the store with nothing reaching them — not held, by the human's ruling of 2026-10-14 on what *held* means (reading (b): a reflog names no blob that only an index held).
- *Held against.* The clause; and **it stands on a declared bound**, the first of the four: *content staged for a doc a task is updating, while the working file does not carry it, is replaced at finalize*.
- *The previous release.* The same: *lost*, exit 0.

### `r1-task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file-is-replaced`

- *The cell.* `task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file`, in `fresh`: the same act at `.jigc/version`, last before the finalize of a task that created a decision.
- *What it came to.* **Lost**: exit 0, *1 file committed* — the decision — and the index entry of `.jigc/version` is no longer what the user staged; the bytes are in the store alone, which is not held (reading (b), ruled).
- *Held against.* The clause; and `T3`'s sentence, *"This phase stages **jigc's own** managed artifacts into that existing index"* — what the user staged there was not jigc's own. No declared bound names this path: **a finding**, by the human's ruling of 2026-10-14 on the record's §6, entry 8 — the bound is one case, a doc a task is updating.
- *The previous release.* The same: *lost*, exit 0.
- *Beside it.* A hand edit to the stamp **in the working file** is left alone on this tree — *jigc left it untouched and out of this commit* — where the previous release replaced it at exit 0: *lost* there, *holds* here.

### `r1-task-discard-a-change-staged-at-the-record-and-not-in-the-file-is-replaced`

- *The cell.* `task-discard-a-change-staged-at-the-record-and-not-in-the-file`, in `fresh`: a milestone with one sub-task nobody re-entered; a line is added to the milestone's committed record, staged, and the working file put back; the sub-task is discarded.
- *What it came to.* **Lost**: exit 0, `record commit: …`, and the staged bytes are in the index no longer, and in no object within reach (reading (b), ruled).
- *Held against.* The clause; and `A6`'s sentence — the record's commit *"stages and commits **only the record file** … never sweeping the agent's in-flight index/worktree WIP"*: the staged change was index work in flight, at the one path the door stages. A hand edit **in the working file** is refused at the same door (`reconciliation.conflict-block`), on both binaries. **A finding**, by the ruling on the record's §6, entry 8.
- *The previous release.* The same: *lost*, exit 0.

### `r1-task-discard-a-rejected-record-commit-leaves-its-message-file-in-the-working-area`

- *The cell.* `task-discard-a-hook-that-rejects`, in `fresh`: the same sub-task, and the user's own lines in the `pre-commit` hook, which reject every commit.
- *What it came to.* **Refused, and changed what a refusal may not.** Exit 1, *nothing was committed — the milestone record still names task:do-the-thing as it did, and the task's working area is intact*; and `.jigc/tasks/do-the-thing/record-commit-msg.txt` was made.
- *Held against.* `A8`'s sentence — *"a rejected `pre-commit` hook leaves the record *and* the task exactly as the frame says"* — and `design/finalize.md` → *6. Commit*: *"a temp file, deleted after"*. **A finding**, by the human's ruling of 2026-10-14 on the record's §6, entry 6: the file is deleted whether or not the commit landed, as `design/finalize.md` says and as `task finalize` does.
- *The previous release.* The same.
- *Beside it.* `task-finalize-a-hook-that-rejects` leaves no file.

### What the same cells show the other way — red on the previous release, and not on this tree

Seven cells of the family come out otherwise on the previous release, beside the acceptance's three — six of them red there and not here, and one red on both, differently: a hand edit to `.jigc/version` (*lost* there, *holds* here) · a file staged at the created doc's home and not in the tree (*lost* there, refused here) · a link to a file in the repository, a link to one outside it, and a directory at the created doc's home, each refused there with another refusal than the one the cell names (`finalize.promote-clobber` twice; an unnamed *could not read … Is a directory* at exit 1) and refused here with `store.home-not-regular-file` · and the two cells of the hand edit before the first write, which that release refuses and this tree does not — one of them the first key above, red here as a loss. The arm counts; it does not say which of these is a regression.

## 5. Seen by hand, and no cell

*Each was read off a cell's output or a probe of the builder's on 2026-10-13; none is a difference of bytes, so none is a cell's to hold, and each is the route catalog's kind of finding.*

- **`jigc task discard <id>` over a task that is not there answers `finalize.no-task`** — a code of another command. Exit 1, *no task `a-task`*, nothing changed. The cell names the refusal by its sentence.
- **`jigc task finalize` over a file where the created doc's directory goes exits 1 with no finding**: *could not create "<sandbox>/repo/docs/decisions" to promote into: File exists (os error 17)* — an operating-system error, an absolute path, and no code — where a link or a directory at the doc's own home is `store.home-not-regular-file` at exit 3. Nothing is changed but the edge index, rebuilt.
- **A `doc create` over a file the user committed at the minted id takes the file up as the doc's base, and a write of the task to it is then refused** (`write.non-reparseable`, seen at `doc set-slot`) when the file is no doc of that type. The finalize refuses on the file's conformance (`conformance.section-missing`), and the file stands.

## 6. The install family's cells — 47 are red on this tree's binary, under seven keys

*Read again on 2026-10-18, with the first commit of the task `B6`, for the human's rulings of 2026-10-15: one key retires — its thirteen cells hold under `S16` — and one is minted, so **34 of the family's cells are red** in the gate under seven keys, where 47 were; the counts below are the record of 2026-10-15, and each key says what moved.*

*2026-10-15, this tree's debug build, macOS arm64; the third commit of the task `B5`. The family is `jigc setup`, `jigc uninstall`, `jigc ingest` and `jigc unmanage`: 113 cells beside the acceptance's six — 89 that the gate runs, in the arms of `bare`, `fresh` and `committed-singletons`, and 24 base cells on demand ([byte-table-on-demand-cells.tsv](byte-table-on-demand-cells.tsv)). The suite's expected-red list names the 47 below, and is exact. **Every one of the 47 is red on the previous release too** (`1.0.0-rc.24`, sha256 `accf3996…ab5d`, checked before the run; the arm on a named binary, 2026-10-15) — 40 in the same class, seven in another, said under each key — so none is a cell the fix pass made red. Nothing here is a grade.*

### `r1-setup-makes-a-cache-directory-no-sentence-of-its-doc-names` — retired 2026-10-18

*Retired by the human's ruling of 2026-10-15 on the rows' record's §6, entry 1: the product is unchanged, the row gains `S16`, and the thirteen cells red for this alone hold — the fourteenth, `setup-a-hook-that-rejects`, is red under `r1-setup-runs-no-hook-at-its-install-commit` by the ruling on entry 11. The doc's half is §1's `r1-assistant-adapter-design-does-not-name-the-cache-directories-setup-lays-out`. As it stood:*

- *The cells.* Fourteen in the gate in which `setup` acts and nothing else is red: the never-adopted repository as typed; a file staged elsewhere, under a name git quotes, a hook that rejects, the user's own hook; the guide, whatever stands at it — a file, a committed one, three links, a directory, a file at its directory's place; a file under the project layer; and on an unborn `HEAD`, the user's own root `.gitignore`. Six more red cells show it beside their own class.
- *What they came to.* **Written outside its allowance**: exit 0, the install landed, and `.jigc/state/` was made — an empty directory no line of the row covers.
- *Held against.* The row, which has no line for it: the record's §6, entry 1, *what the ruling leaves open*, (i) — whether a command that exits 0 may write a cache its own doc names no write to. Each cell carries that entry's mark; the record says what the product should do. **No byte is lost.**
- *The previous release.* The same, in all fourteen — one of them, the user's committed guide, as *git's state moved outside its allowance* there.

### `r1-setup-replaces-a-file-the-user-committed-at-a-path-it-writes-whole`

- *The cells.* `setup-at-the-bootstrap-a-committed-file`, `setup-at-the-stamp-a-committed-file`, `setup-at-the-settings-record-a-committed-file`: a file of the user's, committed, at `.jigc/AGENT.md`, `.jigc/version` or `.jigc/settings-entries.json`.
- *What they came to.* **Git's state moved outside its allowance**: exit 0, the install commit replaces the user's file; history holds its bytes, so it is no loss.
- *Held against.* The plan's rule that a planted entry is the user's; `S4`'s second sentence refuses such a file only *"while no commit holds its bytes"*. The record's §6, entry 10, has what the product should do — refuse, as its siblings do — and the other reading. Each cell carries entry 10's mark.
- *The previous release.* Red in all three: two the same, one *written outside its allowance*.
- *Ruled on 2026-10-15*, option A: **`setup` refuses without `--force` where the bytes at a path it replaces are not jigc's own**, committed or not, and the sentence of `design/project-setup.md` becomes *"while its bytes are not jigc's own"* — a product change and its doc's correction, one finding. The cells lose entry 10's mark and are listed as refused with `setup.dirty-install-path`; on this tree each stays *git's state moved outside its allowance*, now with *not the listed outcome* beside it.

### `r1-setup-commits-the-file-a-root-ignore-link-leads-to-and-the-link-on-an-unborn-head`

- *The cell.* `setup-head-unborn-at-the-root-ignore-a-link-to-a-file-in-the-repository`.
- *What it came to.* **Git's state moved outside its allowance**: exit 0; `setup` merged into the file the link leads to — *"note: `.gitignore` is a link, so `jigc setup` merged into the file it leads to … that is the file the install commit carries"* — and the install commit holds the link and that file, a file of the user's in no commit before.
- *Held against.* `S9` and `S11`: the row declares the root `.gitignore` merged into, and no line the file a link there leads to. The record's §6, entry 12.
- *The previous release.* Red, as *written outside its allowance*.
- *Ruled on 2026-10-15*, option A: **`setup` refuses a link at the root `.gitignore` on any `HEAD`**, as on a born one, and `design/assistant-adapter.md` lists the root `.gitignore` beside `.jigc/.gitignore` as a member a link is never followed at — a product change and its doc's correction, one finding. The cell is listed as refused with `setup.secrets-gitignore`; on this tree it stays *git's state moved outside its allowance*, now with *not the listed outcome* beside it.

### `r1-setup-replaces-a-change-staged-at-a-path-it-writes-whole`

- *The cells.* `setup-at-the-stamp-a-change-staged-and-not-in-the-tree`, `setup-at-the-guide-a-change-staged-and-not-in-the-tree`.
- *What they came to.* **Lost**: exit 0, and what the user staged at `.jigc/version` or at the guide is in the index no longer and in no object within reach.
- *Held against.* The exit rule's first clause; and the human's ruling of 2026-10-14 on the record's §6, entry 8 — the declared bound is one case, a doc a task is updating, so a staged change replaced at a path jigc stages is a finding. At the bootstrap and the settings record the same change is refused (the next key).
- *The previous release.* The same: *lost*, both.

### `r1-setup-refuses-after-its-first-write-and-leaves-the-install-written`

- *The cells.* Twelve: a change staged and not in the tree at `.jigc/AGENT.md`, `.jigc/settings-entries.json`, `CLAUDE.md`, `.claude/settings.json`, `.jigc/.gitignore`, and on an unborn `HEAD` the root `.gitignore`; a directory at `CLAUDE.md`, at `.jigc/.gitignore` and on an unborn `HEAD` at the root `.gitignore`; at the hook, a directory in its place, a file in its directory's place, and a link to a file in the repository.
- *What they came to.* **Refused, and changed what a refusal may not**: exit 1, with `setup.dirty-install-path`, `setup.install-hook`, `setup.inject-reference`, `setup.init-project-layer` or `setup.secrets-gitignore` — *"the install files are written and staged, and no install commit was made"* — and the files of the install written, in most staged, and the install-footprint record under `.jigc/state/`.
- *Held against.* **The human's ruling of 2026-10-14 on the record's §6, entry 2: every refusal of `setup` leaves the repository as it was**, caches aside — a failure after the first write rolls back. These are the late failures it decides, and the `S12` behaviour it overrules (§1 of this file, `r1-setup-design-stages-a-failed-install-where-the-rule-is-that-a-refusal-changes-nothing`). **No byte of the user's is lost** in any of the twelve: the plant stands in each.
- *The previous release.* Red in all twelve: eleven the same, one *written outside its allowance*.
- *Ruled on 2026-10-15, the record's §6, entry 3*, option A: **the install-footprint record is named in the doc and moves out of `.jigc/state/`, since it is not rebuildable; a refusal leaves none**, and it is part of this finding; the fixer settles how the old path is read. The table checks its path: `C2` no longer excuses `.jigc/state/setup-install-footprint` (`byte_cell::NO_CACHE`), so each of the twelve names it among what the refusal changed. The class is the one it was.

### `r1-setup-runs-no-hook-at-its-install-commit`

- *The cell.* `setup-a-hook-that-rejects` — a `pre-commit` hook of the user's that rejects every commit, put in the hook git resolves before `setup` runs in the never-adopted repository.
- *What it came to.* **Not the listed outcome**: exit 0 — the install commit landed past the hook, which never ran — where the cell is listed as refused with `setup.install-commit`, the code the door gives a failed install commit. No byte is lost: the hook's lines stand, wrapped by jigc's block.
- *Held against.* **The human's ruling of 2026-10-15 on the rows' record's §6, entry 11**, option A (*"we should not work around the users existing setup"*): `setup`'s install commit runs the user's hooks, jigc's own hook kept from triggering itself; a rejection is a late failure, rolled back as the ruling on entry 2 has it. The commit is made with `--no-verify` today, by a code comment's reasoning and no doc's. **What follows** is a change to the product and the sentence of `design/assistant-adapter.md` → *`jigc setup` has assistant-neutral install responsibilities too* that says the install commit runs the user's hooks.
- *The previous release.* The same: *not the listed outcome* — its install commit runs no hook either.

### `r1-uninstall-leaves-its-teardown-in-the-working-tree-and-commits-nothing`

- *The cells.* Thirteen in the gate — every cell in which `uninstall` acts: as typed, after a task landed, over an edit, a line of the user's or a staged change at the host file, a staged change at the settings file, the user's lines in the hook, a hook that rejects, the user's worktree there and moved away, a file staged elsewhere and under a name git quotes, and a foreign file under `.jigc/logs/` under the consent — and the four acting cells on demand.
- *What they came to.* **Written outside its allowance**: exit 0; the tracked files of the install are gone from the working tree, the host file and the settings file taken out of, and no commit carries any of it — `HEAD` and the index are as they were.
- *Held against.* `U9` — the human's ruling of 2026-10-14 on the record's §6, entry 5: *"the teardown lands as one dedicated, path-scoped commit of what it removed"*. **No loss**: what left the tree, `HEAD` still holds. In the three cells with an edit of the user's at the host file or the settings file, the edit stands; whether the fixed teardown refuses over it or commits only jigc's part is the fixer's, and either holds the ruling's *never carries*.
- *The previous release.* Red in all thirteen: eleven the same, one *lost*, one *git's state moved outside its allowance*.

### `r1-uninstall-refuses-over-a-settings-file-it-cannot-parse-after-removing-the-workbench`

- *The cells.* `uninstall-an-edit-to-the-settings-file`, `uninstall-a-line-of-the-users-in-the-settings-file` — the user's line makes `.claude/settings.json` no JSON.
- *What they came to.* **Refused, and changed what a refusal may not**: exit 1, `uninstall.remove-allowlist`, and `.jigc/` — its tracked files and its caches — already removed.
- *Held against.* `U7`: *"All four remove nothing, name what they found, and route at the honest exits"* — a refusal of the teardown removes nothing; and the project's rule that a refusal changes nothing. No byte is lost: `HEAD` holds the tracked files, and the caches are rebuildable.
- *The previous release.* The same, in both.

### What the same cells show the other way

Twenty-seven cells of the family are red on the previous release and not on this tree: eight refused there with another refusal than the one the cell names, or acted where it refuses here; six wrote outside the allowance; five refused after writing; four lost a plant — an untracked file or a link at a path the install replaces, written over at exit 0; four moved git's state. The arm counts; it does not say which of these is a regression.

## 7. The plant in the place of a file that stands — 22 cells, two red on this tree's binary under two keys

*2026-10-18, this tree's debug build, macOS arm64; the second commit of the task `B6`. The human's ruling of 2026-10-18 had `B6` build **a link or a directory in the place of a file that stands** (`byte_cell::Plant::InPlaceOf`) and fill the needs that waited for it: the task family's fourteen — `T2`, `.jigc/version` at `task finalize`, and `A5`, the milestone's record at a sub-task's `task discard`, each by a file of the user's, one the user committed, three links, a directory, and a file in the place of the directory the file stands in — and the install family's eight, an edit to each file of `setup`'s own install, built by a lead-in that installs first. All 22 run in the gate. On the previous release (`1.0.0-rc.24`, sha256 `accf3996…ab5d`, checked; the arm on a named binary, 130 s at a load of about 10): **ten of them are red there and not here**: five at exit 0 at the stamp — what the user put in its place written over, *lost* three times (a file of the user's, and the file each of two links led to) and *git's state moved* twice (the file the user committed, and the link to nothing) — the directory in the stamp's place refused with another refusal, and the three links and the directory in the record's place refused with another refusal than this tree's; and one cell does not build there (`setup` in the never-adopted repository writes no settings record there to edit). Nothing here is a grade.*

### `r1-task-discard-settles-no-record-and-discards-a-sub-task-whose-record-is-not-on-disk`

- *The cell.* `task-discard-a-file-in-the-place-of-the-records-directory` — a milestone with one sub-task nobody re-entered; the directory `docs/milestone-records/` moved away and a file of the user's put in its place; then `jigc task discard do-the-thing`.
- *What it came to.* **Not the listed outcome**: exit 0, *"discarded task do-the-thing"* — the sub-task's working area removed and no record settled, where the record `HEAD` holds still names the sub-task. No byte is lost: the area held nothing staged, the record stands in `HEAD` and under the sandbox's `moved/`, and the file in its directory's place stands.
- *Held against.* `A8`: *"The record commit runs **before** the working area is removed"* — the discard settles before it destroys; and `A9`, which makes the discard that writes no record an **ordinary** task's: one *"named by no record"*. This sub-task is named by a committed record; the discard read the record off the working tree, found none, and discarded it as an ordinary task. The cell is listed as refused, naming the record, and no run gives the words.
- *The previous release.* The same: exit 0, *not the listed outcome*.

### `r1-setup-refuses-a-staged-root-ignore-on-an-unborn-head-where-an-untracked-one-rides`

- *The cell.* `setup-head-unborn-an-edit-to-the-root-ignore` — the never-adopted repository with its `HEAD` unborn; a root `.gitignore` of the user's own, `git add`ed, and a line added after it; then `jigc setup`.
- *What it came to.* **Not the listed outcome**: exit 1, `setup.dirty-install-path` over `.gitignore`, nothing changed — where the cell is listed as acting, by S15's on-ramp.
- *Held against.* The rows' record's §6, **entry 13**, open: S15 lets an *untracked* file at a path the install merges into ride the first commit on an unborn `HEAD`, and no sentence says what becomes of a **staged** one. The cell follows the reading by which it is red, and carries the entry's mark. The record leads with what the product should do — refuse, as it does — so the ruling may retire this key.
- *The previous release.* The same.

### What the other twenty show

- **At the stamp (`T2`)**, `task finalize` leaves what stands in jigc's stamp's place — a file of the user's, one the user committed, three links, a directory — and lands at exit 0; with the workbench itself replaced by a file, there is no task and it refuses (`finalize.no-task`). Seven hold or refuse with nothing changed.
- **At the record (`A5`)**, a sub-task's discard refuses over a file or a committed file in the record's place (`reconciliation.conflict-block`) and over a link or a directory there (`store.home-not-regular-file`), each with nothing changed.
- **At `setup`'s own files**, an edit is refused as dirty at the bootstrap, the host file, the settings file, the stamp, the settings record and the workbench's ignore file — the edit's bytes in no commit — and the guide a user edited is left as it is, the install acting around it.

