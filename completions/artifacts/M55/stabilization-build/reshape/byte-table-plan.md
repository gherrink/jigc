# The byte table — a plan for its pilot

**What this file is.** The plan the byte table's pilot is built to, on record beside the route catalog's: drafted on 2026-10-12, read by an independent reviewer the same day and revised on what that review found (§11). It is committed as it was reviewed — its substance is the planner's and the reviewer's, and no builder has improved it; where a builder finds a sentence of it false against the tree, the sentence stays and the finding is in that task's entry of [DECISIONS.md](../../../../../DECISIONS.md) and in the record the task writes. **The pilot's first cut is the tasks `B0` to `B4`, and at `B4` it stops: the numbers go to the human, who rules on the families after it** (§0, §8). The ruling it is built under is the human's of 2026-10-12, quoted in §0; it is recorded on the branch `fix/rc24-tier1`, in `DECISIONS.md` → *The reshaped `test` stage is run for real in parts*, ruling 3, and reaches this branch when the two are joined. What `B0` wrote from it is [byte-table-acceptance-rows.md](byte-table-acceptance-rows.md). A path in code type is from the repository's root; `<scratch>` is the session's scratch directory, whose files are working notes and are committed nowhere.
*Drafted 2026-10-12 against the worktree of branch `fix/rc24-route-catalog` at `587a4d40`; revised the same day after an independent review, whose findings and what each changed are §11. A plan: it changes nothing, decides nothing that is the human's, and is written for a builder who has not seen the code. The pending row it plans for is `wf-pre-byte-matrix` (`implementation/decisions-pending.md`); the ruling is the human's of 2026-10-12, quoted in §0.*

**How a claim is marked.** **(ran)** — established by a command run for this plan, on this machine, on 2026-10-12, under a load average of 38 to 51; the scripts and their output are the planner's working notes, under `<scratch>/byte-table/` (`touched.py`, `states.sh`, `accept.sh`, `snap.py` and their `.out` / `.tsv` files), which are leads and are not committed. **(review ran)** — run by the reviewer with scripts of its own (`<scratch>/review.TVdfPu/cells.sh`, `lib.sh`); its output was not kept as files, so its results are cited as the review gave them. **(read)** — read in the worktree's code or committed records, at the place named. **(not established)** — inferred, estimated or left open; §10 collects them.

**Words used.** A *write leaf* is one row of `VERB_KINDS` (`crates/cli/src/cli.rs`) classed `VerbKind::Write` — one command as typed after `jigc`. A *prepared state* is one of the seven named repository states the suites already build. A *plant* is one thing put into a prepared state by hand before the command runs — a file, a link, an edit — carrying bytes found nowhere else. A *lead-in* is a product command the cell runs first, on the binary under test, so that the command has something to act on. A leaf's *row* is its entry in the table: its argv, its lead-ins, and its *allowance* — what the command may change, as its design doc declares it, line by quoted line. A *cell* is defined in §2.

---

## 0. The ruling, the model, and the first cut

> **The byte table is piloted now, bounded as the route catalog was bounded**: only the writing commands the fix pass touched, against the prepared states; the rest before the port (the next milestone, M56). It is built in this worktree's lane, beside the tooling work on another branch.
> **Not taken:** all of it before the port and none now; not built at all.

The model is the route catalog (`crates/cli/tests/support/route_case.rs`, `crates/cli/tests/route_catalog.rs`, `implementation/pinning.md` → §6): a case is data, the binary is handed in, a red case is listed under a key and the list is exact, and a fence holds every site to a case, a reason or a scheduled row.

**The pilot's first cut is five tasks, `B0` to `B4`, and ends at a stop.** The acceptance rows' allowances written from the docs; the frame; the acceptance on both binaries; the fence with the scheduled list; one family end to end — `task finalize` and `task discard`. **At `B4` the numbers go to the human, who rules on the families after it** — as the route catalog's two-command pilot preceded its ruling on families. Tasks `B5` to `B9` stand in §8 as what would follow, not as what is built.

**What the table is not.** The exit rule's instrument for its first clause is *the re-review rows and the audit, graded against that scope* (§1.1). The table does not replace it.

**What the table can find, said plainly.** Four of the six blocking rows reproduce only where the plant goes into a place, or under a condition, that the row's own record names (§3). The position rules of §3 were written knowing the rows. So the table does not discover new classes of loss: **what it finds new is the same plant at a sibling command** — which is what the completion audit found by hand (*losses inside the classes the fixes claimed closed*) — and it keeps a found loss found, on any binary handed in.

---

## 1. What was established

### 1.1 The exit rule's first clause

`DECISIONS.md` → *2026-10-04 — The exit rule, revised* (read). The rule, in the human's words, exactly:

> "We do not lose files or writhe / update incorrect things. We have a working product others can use and relay on. It is usable by the agents. The planned migration for this project to use jigc will work."

The first clause is the first sentence. The two sharpenings recorded with it (read, verbatim):

> 1. **The scope of the first clause.** In a healthy repository used as documented — which includes ordinary git configuration: line-ending conversion, `status.showUntrackedFiles`, a symlinked `CLAUDE.md`, linked worktrees — no jigc command at exit 0 destroys bytes no git object holds or commits content the user did not ask for. Where jigc cannot tell (git fails, the index is unreadable) it refuses before writing. Races against a non-jigc writer inside a millisecond window, and deliberately planted states, are **declared bounds**, written down with their reach.
> 2. **One instrument per clause.** The re-review rows and the audit, graded against that scope · …

| the words | what the table takes from them |
|---|---|
| *at exit 0* | the loss rule binds a command that exited 0; a refusal is held to a second rule (§2) |
| *destroys bytes no git object holds* | lost is about **bytes**, wherever they stood. Bytes a git object still holds are not lost. What *holds* means is question 1 of §7 |
| *commits content the user did not ask for* | a commit the command makes is compared against what its doc declares it commits |
| *write / update incorrect things* (the rule itself) | a write outside what the command's doc declares is red even where nothing was lost |
| *refuses before writing* | a refusal that changed what no doc lets a refusal change is red |

**What later entries say of the clause** (read):

- **Three kinds of planted state are bounds, and no other** — `DECISIONS.md` → *2026-10-11 — Which states a route must work in*: a permission bit changed by hand · a FIFO or device node planted · a state only `JIGC_ADAPTERS_DIR` reaches. *Not bounds, so able to block:* symbolic links of every kind; what git reports — dirty, staged, unborn, a linked worktree; hand-edited or foreign content; **a directory in a file's place and a file in a directory's place**. Every plant of §3 is therefore a state that can block.
- **Four further bounds, each the human's, 2026-10-06** — `implementation/decisions-pending.md` → *The declared bounds, as one list with reach* (the list is still owed): (1) content staged for a doc a task is updating, while the working file does not carry it, is replaced at finalize; (2) an uncommitted edit made before a task first writes a doc is carried into the task, and a write by that task to the same part replaces it *[retired 2026-10-19 by the human: such a write refuses at the write — `DECISIONS.md`, the entry of that day]*; (3) while the invocation log is switched on, any jigc run brings the log back; (4) a repository that ignores jigc's own committed files is unsupported.
- **The race class** behind `(R6, K-1)` is a declared bound for 1.x. A race is not a state; the table has no cell for it.

### 1.2 The six blocking rows of `1.0.0-rc.24`

Two records name six, and they are not the same six (read): the review's own, as assembled (`completions/artifacts/M55/per-axis-review-rc24/README.md` → *The headline*), which hold the race `(R6, K-1)`; and the six the fix pass was chartered on after each row's adversarial re-drive (`completions/artifacts/M55/fix-pass-rc25/README.md`), which hold the blind trial's `L-22` in its place. **The pending row's "five of six" is the first set** (`prebuild-census.md` → §2 lists the five states by name).

**A decision of this plan, with its reason, which the human may overturn:** the acceptance is over the **second** set — the rows a fix closed — because only for those can the tree's binary show *gone*, and the one row that differs, `(R6, K-1)`, is a race no state reproduces.

| key | command | what had to be planted, and where | on `rc.24` | on this tree |
|---|---|---|---|---|
| `(R1, F1)` | `setup` | an untracked file at `.jigc/AGENT.md` and `.jigc/config/packs.yaml` — **and `HEAD` unborn** | exit 0; both replaced, the bytes in no object (ran; review ran). With `HEAD` born: exit 1 `setup.dirty-install-path` (review ran) | exit 1 `setup.dirty-install-path`; no difference (ran; review ran) |
| `(R3, F7)` | `task finalize` | a line added to `VISION.md` by hand **after the task's first write** to it — **in a clone with no file-state baseline** | exit 0; the line in no object (ran) | exit 3 `reconciliation.conflict-block`; the line there; 3 new files under `.jigc/index/` (ran) |
| `(R6, D-1)`, untracked half | `milestone join` → `milestone finalize` | an untracked file at the id the join numbered onto, planted after the join | exit 0; the bytes in no object (ran, with both halves at once; review ran, alone) | exit 3 `finalize.promote-clobber`; intact; 5 new files under `.jigc/milestones/<id>/merged/` (ran; review ran) |
| `(R6, D-1)`, committed half | the same | a committed doc at that id — **committed before `milestone create`**. Committed after the join it draws exit 3 `finalize.base-mismatch` on `rc.24` (review ran) | exit 0; the doc's content replaced, history holds it (ran in the order the row has; review ran) | exit 3 `finalize.promote-clobber` (ran) |
| `(R6, D-7)` | `doc create` → `task finalize` | a link to nothing at the home the create mints, before or after the create | exit 0; the commit holds a `120000` entry; a new untracked file at the link's target, **byte-identical to the staged doc** (ran; review ran) | exit 1 at `doc create`, or exit 3 at finalize, both `store.home-not-regular-file` (ran; review ran) |
| `(R9, F5)` | `uninstall` | a file **under `.jigc/logs`, `.jigc/state` or `.jigc/index`**. Directly under `.jigc/` it draws exit 1 `uninstall.untracked-workbench-file` on `rc.24` (review ran) | exit 0; three files gone, in no object (ran; review ran) | exit 1 `uninstall.foreign-bytes`; no difference (ran; review ran) |
| `L-22` | `milestone finalize` | the user's own linked worktree, holding a commit only its `HEAD` reaches and a staged file, its directory moved away | exit 0; the registration gone, one commit no longer reachable (ran) | exit 0; the registration kept (ran) |
| `(R6, K-1)` | `doc create` | a file arriving inside a window of about a millisecond | not a state; no cell | — |

`(R3, F7)` and `L-22` were run by this plan alone; nobody has run them a second time. `rc.24`'s binary has the hash the catalog's pilot record names, `accf3996…ab5d` (ran; the review confirms).

**Three things the runs showed that shape the plan.**

- **`(R6, D-7)` loses no byte.** The prose stands, byte for byte, at another path. What is wrong is what the commit holds. §2 classifies it so.
- **A plant in the wrong place finds nothing.** The bold conditions in the table are each the difference between a loss and a refusal on `rc.24`. §3 turns them into position rules.
- **Two refusals on this tree leave new files under `.jigc/`.** The docs answer most of that (§7, question 2).

The four older exit-0 losses the completion audit found beside the six (read, not run): an untracked file under `status.showUntrackedFiles=no` at `milestone discard` and `uninstall`; `setup` installing where `git status` fails; the guide's front matter replaced; an extended `pre-commit` hook deleted by `uninstall`. The last two are a user's lines in a file jigc merges into — plant **M** of §3; the first needs a git configuration and is scheduled (§6).

### 1.3 Where 36 and 7 come from

Both are the census's (`prebuild-census.md` → §2, *Missing*; read), and **both match the code's registries** (counted in source; the review confirms 36 and 12):

- **36 write leaves** — `VERB_KINDS` holds 48 rows, 36 `Write` and 12 `Read`. Top level 10 (`start`, `workflow`, `setup`, `uninstall`, `ingest`, `migrate`, `migrate-corpus`, `unmanage`, `rename`, `relocate`); `doc` 8 (`create`, `add-item`, `remove-item`, `retitle-item`, `rename`, `set-field`, `set-slot`, `author`); `task` 4 (`amend`, `discard`, `finalize`, `bind`); `config` 6 (`set`, `insert-step`, `replace-step`, `remove-step`, `fill`, `fork`); `milestone` 8 (`create`, `add-task`, `add-from-spec`, `provision`, `execute`, `join`, `finalize`, `discard`).
- **7 states** — `State::ALL` in `crates/cli/tests/support/trial_corpus.rs` has six: `fresh`, `committed-singletons`, `migrated`, `refs-post-hoc`, `chatty-hooks`, `vendored`; the seventh is `bare` (`RIG_ONLY_STATES` in `crates/cli/tests/dev_rig_parity.rs`; `TrialCorpus::build_never_adopted()`).

**No difference between the count and the lists.** Three things the count does not say: a leaf is not one behaviour (`start` is four; five leaves take `--force`; `migrate-corpus` takes `--dry-run` and `--no-commit`); the argv the census points at resolves nothing by design (`leaf_argv.rs`), so a cell that acts needs an argv and lead-ins written for it; and all seven states build on `rc.24`'s binary through `dev/jigc-rig <state> --binary <path>` (ran: 14 of 14 builds exit 0; 56.7 s together on this tree's debug binary) — through the Rust builder on a named binary, not established.

### 1.4 Which write leaves the fix pass touched — a range, and a choice

`git diff --stat 91834b5e HEAD -- crates/cli/src crates/engine/src`: 29 files, 18,477 insertions, 1,388 deletions (ran). Every command's module changed, or a module it calls did.

**The rule this plan chose.** A write leaf is *touched* when its handler reaches a function that changed since `91834b5e` and that not every write leaf reaches. **That is a choice, not a fact of the code, and the human may revisit it at the stop**: it leaves out the 61 changed functions all 36 leaves reach, and it counts a changed function whatever the change was.

**What a script gives, and where it is wrong** (ran, `touched.py`; the review re-ran it to the same table, 2,475 functions and 377 changed). It finds 28 leaves. Its call graph is by name and **drops edges**: it skips about eighty common names, `load` among them, and any name two files share, `save` among them. So:

| | leaves | why |
|---|---|---|
| counted by the script | 28 | the table below |
| counted, and touched only through what the command prints | `workflow`, `task amend` | the changed functions they reach set a presentation field (`code_only`, `crates/cli/src/start.rs:978`) and one line of `state::record_doc_provenance` |
| not counted, and one step from a changed function | `ingest`, `unmanage` | `crates/cli/src/unmanage.rs:70,88-91` and `crates/cli/src/ingest.rs:205,286-289` call `FileStateRecord::load` and `::save`, both changed (hunks at `crates/engine/src/file_state.rs:163` and `:255`; ran) |
| not counted | `task bind`, `config insert-step`, `replace-step`, `remove-step`, `fill`, `fork` | no edge found; `task bind`'s one is a name collision |

**So the count is 26 to 30 of 36, by reading**, and the script's 28 is inside it. On the command axis the bound barely binds under any reading: a command is a coarser unit than the catalog's refusal site, of which the fix pass touched 48 of 222. **The first cut does not depend on the answer** — its two leaves, and the four of the acceptance, are touched under every reading. `B3` reads each leaf the script calls untouched against its handler and commits the count with its derivation.

The script's table, for the 28:

| leaf | changed functions it reaches that not every leaf reaches | the nearest, and calls from the handler |
|---|---:|---|
| `setup` · `uninstall` | 57 · 74 | `setup::install` · `setup::uninstall`, 1 |
| `task finalize` · `task discard` | 46 · 14 | `task::finalize`, 1 · `milestone::discard_sub_task_item`, 2 |
| `doc create` · `doc author` · `doc rename` | 17 · 18 · 14 | the handler itself |
| `doc add-item` · `remove-item` · `retitle-item` · `set-field` · `set-slot` | 10 · 10 · 10 · 11 · 10 | the handler; `doc::read_or_copy_in`, 1 |
| `milestone finalize` · `milestone discard` · `provision` · `execute` · `join` | 80 · 48 · 30 · 17 · 9 | the handler, or 1 |
| `milestone create` · `add-task` · `add-from-spec` | 7 · 7 · 7 | `guard_record_free` · `append_and_commit_record`, 1 |
| `rename` · `relocate` · `migrate-corpus` · `config set` | 16 · 10 · 10 · 14 | 1 · 4 · 2 · 1 |
| `start` · `migrate` · `workflow` · `task amend` | 5 · 5 · 2 · 2 | 2 · 1 · 2 · 2 |

### 1.5 What already compares bytes

| what | what it holds | what it is to the table |
|---|---|---|
| `read_verb_acts_nothing.rs` | the 12 `Read` leaves × one state; the whole tree including `.jigc/`, with git's state read through git; one admitted prefix held to two properties | **the comparator's model**. No allowance, no plant, cargo's binary only |
| the route catalog (632 cases) | per case, named files whose bytes must not change, and `Check`s over paths | **the frame** (§4). A witness is a file a case names; the table names nothing |
| `posture_door_axis.rs` | 12 acting rows of `BEHALF_DOORS` × 17 git states; the refusal's identity, not the tree's bytes | not repeated (§6, scheduled) |
| `commit_rejected_axis.rs`, the rollback suites | a rejected commit at the committing doors, each with its own assertions | the state of plant **H** (§3); a cell names the suite |
| the fix pass's suites: `setup_install_pathspec_guard` (46 tests), `uninstall_workbench_subject` (29), `copy_in_baseline` (22), `milestone_promote_guards` (25), `replacing_writers_never_follow` (14), `worktree_registration_reach` (10), and five more | each of the six rows' states and many orderings beside it, asserting on the files the test names | each holds the state of one acceptance cell already, on cargo's binary only |

**The rule against duplication.** One cell for a command, a plant and a position; the orderings a per-command suite sweeps stay there, and the cell's row names the suite and test that hold the same state. The acceptance cells do repeat states those suites hold: nothing else runs them on a named binary under an oracle over the whole tree.

---

## 2. What a cell is

**A cell is a write leaf with one argv, a prepared state with at most one plant put where its kind's rule says, and the comparison of two manifests taken around that one command.**

In order, the runner: makes a sandbox; brings the prepared state into it; runs the cell's lead-ins on the binary under test; puts the plant in; takes a manifest; runs the command; takes a manifest; classifies. One command stands between the two manifests: a row of the review that reads `join` → `finalize` is two cells, the first command a lead-in of the second.

**A cell names no path it expects to change.** The allowance belongs to the leaf's row. A cell chooses a leaf, a state, a plant and its position.

### The argv — one spelling, and when a second

| spelling | what it is | where |
|---|---|---|
| *acting* | the leaf with arguments that give it something to do, after its lead-ins, each of which must exit 0 | every prepared state but `bare` |
| *acting, in `bare`* | the same argv **with no lead-ins** — the command as typed in a repository jigc was never set up in. Five leaves were run so (`start`, `milestone create`, `config set`, `doc create`, `task finalize`): each exits 1 and changes nothing (ran) | `bare` |
| *refusing* | the leaf with the argv of `leaf_argv.rs`, which resolves nothing | `fresh` only |
| *consent* | the leaf with the flag its owner doc names as the consent to a destruction: `--force`, where the door has one — *"refuse without a consent, narrate under one, or move it aside"* (`design/team-ready-state.md:97`), *"`jigc setup --force` the single consent"* (`design/surface-contract.md:167`) | the planted cells the consent is about |
| `--dry-run`, `--no-commit` | a flag whose doc promises fewer writes | one cell each, in the leaf's home state |

### The manifest — what is compared

| layer | what is read | how |
|---|---|---|
| **files** | every entry under the sandbox's root except the inside of a `.git` directory and the frame's own `staging/` (each child's captured output): the repository's working tree — tracked, untracked, ignored, `.jigc/` included — every other checkout that stands in the sandbox, `home/`, `outside/`, and **`tmp/`**, which every child of a cell has as `TMPDIR` because the product writes under the system's temporary directory (`crates/cli/src/rename.rs:1165`, `crates/cli/src/combine.rs:298`) | kind, the permission bits, and the bytes or the link's target. A directory is an entry |
| **git, asked of git** | per checkout whose directory is there: `HEAD` and its branch, the index entries with their flags. For the repository: every ref, the stash's entries, the worktree registrations as git lists them and as `.git/worktrees/` names them, the commits within each reach of question 1, the local configuration; and **the set of object ids in the store** | `git` children under the frame's mask |
| **files of git's that nobody else reads** | `.git/hooks/*` (not the samples), `.git/info/exclude`; and, **for a registration whose directory is gone, the files of its admin directory** — `HEAD`, `index`, its logs — as bytes | bytes. No child of the frame runs in a directory that is not there, so nothing refreshes that index; without this line a moved-away worktree's index is read by nobody |

**Not compared:** the rest of `.git/` byte for byte — a plain `git status` rewrites the index's stat cache (`read_verb_acts_nothing.rs` says so); modification times.

**Three rules of the manifest the review asked for.** *Held* is computed through git's filters — `git hash-object --path <path>` — or a cell under line-ending conversion reads falsely red. A plant's bytes are **unique and not empty**: an empty file's blob is the one `.gitkeep` has, and would read as held. And one check stands apart from question 1: **no object in the store before is absent after.**

### The row and its allowance

**The vocabulary is not fixed by this plan.** A draft of it — six manners — could not carry what the owner docs declare (§11, the first finding). `B0` writes the four acceptance leaves' allowances from the docs first, in the docs' own sentences; `B1` then builds the vocabulary those rows needed and no more.

Five rules hold whatever the vocabulary comes to:

1. **An allowance line quotes the doc sentence it rests on** — the place it speaks of, what may happen there, the sentence verbatim, the doc and heading — and the fence holds the quote against the doc's text. A doc that moves reddens the row. *[Added 2026-10-20, by the eighth of the fixes after the review of `B8`, for the human's rule of 2026-10-14, sharpened on 2026-10-19 — [implementation/working-principles.md](../../../../../implementation/working-principles.md) → *Neither the doc nor the code is the oracle*:]* **and the sentence is checked for sense before it is an allowance.** A sentence that allows what makes no sense for a user — by the project's invariants, the exit rule, or how the sibling commands behave — is a finding against the doc, never an allowance: no line widens to it — the row holds the behaviour that makes sense, the stricter reading — and the finding names which of the doc and the code is wrong; where the answer would reverse a settled design, it is the human's to rule, never a silence. Neither the doc nor the code is the oracle; a line is not chosen so that a sentence already written stays true.
2. **A line may carry a condition, read off the manifest taken before the command.** `setup` seeds a root `.gitignore` *"iff the repo has zero commits"* (`design/project-setup.md:105`); a line without that condition would allow the write in every repository.
3. **A line says whether it holds when the command refuses**, from its sentence (§7, question 2).
4. **A row is committed before its cells run.** What the command writes beyond the row is red until the human rules; a row is never widened from what the command did.
5. **The arm prints, for every line, how many differences it excused** — so that a line that excuses much is seen.

**What the docs' sentences are expected to need** (read; `B0` establishes it):

| the doc says | where | what a line must be able to say |
|---|---|---|
| a write command writes the task's staged copy in its working area | `design/write-commands.md` → copy-on-first-touch; `design/storage.md:9` — the working area is *"a transient working copy"* | the product may write and rewrite **its own** files under the unit's working area; such a rewrite is not a loss |
| *"The commit set is therefore **the git index** … committed whole"*; a doc-only task's commit is path-scoped | `design/finalize.md:164`, `:218` | the commit holds the index as it stood, plus the docs and config the product staged — **not** a list of paths |
| *"The removal's subject is jigc's own files only: the area's **complement** … is moved aside first"* | `design/finalize.md:212`; `design/storage.md:302`; `design/team-ready-state.md` → The working area's two populations | a removal is of what the product wrote; what it did not write is kept, moved aside or refused over |
| `uninstall` *"removes exactly the enumerated repo-local setup-created set"*, and four states refuse | `design/project-setup.md:154`, `:160` | a removal of an enumerated set, with states that refuse |
| the root `.gitignore` *"**only** when setup itself seeded the secrets floor on a fresh repo"* | `design/assistant-adapter.md:64`; `design/project-setup.md:86`, `:105` | a condition |
| `materialize` *"writes only into the gitignored `.jigc/milestones/<id>/merged/`, which it clears and rebuilds every call"*, before the gate | `design/finalize.md:85` | a write that holds on a refusal |

**Whose bytes they are is settled by construction, never by a list copied out of the code.** The docs speak of removal by ownership — jigc's own files, and the complement. Importing the product's own registry of its files (`TASK_AREA_FILES`) to tell them apart would make the test agree with the code by definition. So:

- **A planted entry is the user's.** If its marker stands nowhere after exit 0 — in no file of the sandbox, in no git object within the reach question 1 settles — **the cell is *lost* whatever the row declares.** Only a consent cell is exempt, and there the command's output must name the planted path, as the doc's *narrate under one* asks.
- **What the binary under test wrote in the cell's lead-ins is the product's own**, read off a manifest taken before them. A difference there is judged against the row's lines, and is never *lost*.
- **One rule for prose handed to jigc**: a task's staged doc must, once its working area is gone, stand byte for byte in the commit the command made (`design/finalize.md` → 4. Promote managed docs). Where it does not, the cell is *git's state moved outside its allowance* — unless the bytes stand nowhere, and then it is *lost*. The commit doc is rendered into the message (`design/finalize.md` → 3) and is held to the message.

### What makes a cell red

The classifications, in the order they are read:

| classification | when |
|---|---|
| *the command did not end* | not ended at the frame's limit |
| *lost* | exit 0, no consent, and the user's bytes stand nowhere after: a plant's marker; or, in a cell with no plant, bytes no line of the row covers — at no path of the sandbox and in no git object within reach |
| *git's state moved outside its allowance* | a ref, a stash entry, a registration, another checkout's `HEAD` or index is not what it was; an object that was in the store is gone; a commit the command made holds what the row's lines do not declare — **a link where the row declares a doc, as in `(R6, D-7)`**, a path the index did not hold, a doc landed on a doc |
| *written outside its allowance* | a file-layer difference no line covers — in the repository, another checkout, `home/`, `outside/` or `tmp/` |
| *refused, and changed what a refusal may not* | a non-zero exit, and a difference no line that holds on a refusal covers |
| *not the listed outcome* | the cell is listed as acting and the command refused, or the reverse |
| *holds* · *refuses, nothing changed* | everything else; the two that are not red |

A classification is what one binary did on one day, never a verdict.

---

## 3. The states, the plants, and the count

### The prepared states — as they exist

`bare` · `fresh` · `committed-singletons` · `migrated` · `refs-post-hoc` · `chatty-hooks` · `vendored` (`crates/cli/tests/support/trial_corpus.rs`). Nothing is added to that list.

### The plants — each with the rule that places it

*Each plant once, after the lead-ins* does not determine a cell: the review placed three of the rows' plants by that rule on `rc.24` and drew three refusals (§1.2). A plant's kind therefore carries a position rule, and **a condition the owner doc names for the command's guard is run on both of its sides.**

| plant | what is planted | where and when — the rule | from |
|---|---|---|---|
| **U** | an untracked file | at each place a line of the row lets the command create or replace; after the last lead-in | `(R1, F1)`; `(R6, D-1)` |
| **T** | a committed file | at a place the command creates; **committed before the lead-in that pins the work unit's base** — a commit after it moves `HEAD` off the pin, which is another refusal (`finalize.base-mismatch`). Where the leaf has no work unit, committed last before the command | `(R6, D-1)` |
| **L** | a link — to nothing, to a file in the repository, to a file under `outside/` (three plants) | at each place the command creates, replaces or merges into; after the last lead-in | `(R6, D-7)`; the symlink sibling of `(R1, F1)` |
| **K** | a directory in a file's place; a file in a directory's place (two plants) | at a place the command writes a file; at the parent of one | the ruling of 2026-10-11, which names both as able to block |
| **E** | an edit in no commit, in a tracked file the command rewrites — *[2026-10-18, the human's ruling after the review of `B6`: or the file simply deleted, the index left as it stands — the plain `rm`, which git lists among the changes not staged, beside an edit; a deletion staged with it is **S** (`byte_cell::Plant::Deleted`)]* | where a task stands between: once before the task's first write to the file and once after (two plants); else last before the command | `(R3, F7)`; `(R4, F5)`; bound (2) |
| **M** | a user's own line in a file jigc merges into, the file **in no commit** | the hook, the settings file, the host file; before the command | the audit's guide front matter and extended hook |
| **S** | a staged change | at a place the command writes, and at one it does not; last before the command | `(R5, F1)`; bound (1) |
| **N** | a file whose name git quotes — a quote, a byte outside ASCII | as **U** or **S**, where the command lists or stages paths | the fix pass changed `combine::block_set` for it; `(R5, F2)` |
| **D** | a file the product did not write, in a directory the command removes | **once in each sub-directory the owner doc's layout names** (`design/storage.md` → Repository layout: `tasks/`, `index/`, `state/`, `milestones/`, `worktrees/`, `displaced/`; and `logs/`), and once at the directory's own root | `(R9, F5)` |
| **H** | a `pre-commit` hook of the user's that rejects | at every leaf whose row has a commit; before the command | the main source of a refusal that changes the tree |
| **W** | the user's own linked worktree, holding a commit only its `HEAD` reaches and a staged file — its directory there, or moved away (two plants) | outside `.jigc/worktrees/`; after the lead-ins | `L-22` |

**Conditions run on both sides**, each named by a doc: `HEAD` born or unborn, for `setup` (`design/project-setup.md:105`; `SETUP_UNBORN_EXEMPTION` in `cli.rs`); the file-state baseline present or absent — the clone — for the commands that copy a committed doc into a task and for `task finalize` (`design/storage.md:335`, *What none of this buys*: a lost baseline switches the guarantee off).

**Not plants of the pilot**, each scheduled with its reason (§6): the three declared bounds — the catalog holds cases on all three; a git configuration — the regression set's second part owns those fixtures.

### How the cross is kept from exploding

Three layers, and which of them the gate runs (§9):

1. **Planted** — for every leaf and every line of its row, each plant that targets what the line allows, at each position its rule gives, **in the leaf's home state only** — the smallest prepared state in which that write happens.
2. **Refusing** — every leaf, its refusing spelling, in `fresh`.
3. **Base** — every leaf, its acting spelling, no plant: in its home state, and in the other six.

**The first cut**, `B0` to `B4`:

| | cells in the gate | cells on demand |
|---|---:|---:|
| the acceptance (§5): seven lost-or-moved cells, three more for `(R9, F5)`'s other sub-areas and root, two for the other side of a condition | 12 | — |
| `task finalize`, `task discard`: home-state base, refusing | 4 | 12 (base, the other six states) |
| the same two leaves, planted — an estimate | about 17 | — |
| **together** | **about 33** | **12** |

Three of the gate's 33 need a fan-out.

**The whole pilot, were the human to rule for it at the stop** — not established: 28 leaves give 28 refusing cells, 28 home-state base cells and 168 other base cells; the planted cells were estimated at about 110 before the review added plants and positions, and are more now — about 150 is this plan's guess, from registry comments and not from the owner docs.

Never crossed in the pilot, each a row of the scheduled list: a plant with a state that is not its leaf's home; two plants at once; a plant with a git state mid-operation.

*What the base layer is worth:* the census found no canary finding at a base state (the review confirms the sentence), and every blocking row needed a plant. The base cells are what the pending row names; the findings are expected in the planted layer.

*Rejected:* **planted cells in every prepared state** — seven times the cells for a plant whose effect does not depend on the docs a state holds. **The catalog's 632 states run again as refusal cells** — no construction, but outside the ruling's bound and a change to the catalog's runner; the scheduled list's first row.

---

## 4. The frame

**One new file, `crates/cli/tests/support/byte_cell.rs`, beside `route_case.rs`**, and the suite `crates/cli/tests/byte_table.rs` with `crates/cli/tests/byte_table/`.

### Reused from the route catalog's frame as it stands

The sandbox's layout and its removal whatever a state shut · the faithful mask on every child, with the sandbox's identity through `GIT_CONFIG_COUNT` · **the binary handed in** · `Step` and `Place` for lead-ins and plants, as data · `Var` · the child waited for no longer than a limit, with `LIMIT`, `EXPECTED_HOLD`, `limit_of` · `pooled`, `Gate`, `width` · `Bound` and the reading of a bound off a state's steps · the form of `ExpectedRed` and `differences`, of the fence, of `scheduled.tsv`, of the arm on a binary a variable names.

### The seam — more than visibility

The sandbox and its child runner are private to `route_case.rs` (read). Three changes there, and `B2b`'s *Writes* names the file:

1. **A second constructor.** `Sandbox::new` (`route_case.rs:1359`) runs `git init` and commits. A prepared state needs a sandbox made *over* a repository and a home that already exist.
2. **`pub(super)` on the sandbox, its places, `child`, `run` and `plant`** — visible to the sibling module under `support/` and to no suite, so a suite still cannot run a child except through `follow` or through the byte frame's runner.
3. **`tmp/` as a place**, so that `TMPDIR` can be set through `Var::Path` for the command under test — and the byte frame sets it for its lead-ins too.

No act, check or classification of the catalog changes, and the catalog's arms must print the tables they print now, byte for byte.

### Added

1. **The manifest and the comparator** — a pure function from the manifests, an exit and a row to a classification, with no process in it.
2. **The row** — the leaf as `VERB_KINDS` holds it, the argv and lead-ins, the home state, the allowance's lines.
3. **A prepared state brought into the sandbox.** `TrialCorpus` starts cargo's binary by name at two places and nowhere else (`env!("CARGO_BIN_EXE_jigc")`, `trial_corpus.rs:458` and `:504`); it gains the binary as a value, cargo's where nobody hands one in. That is a small change: 88 suites name `TrialCorpus` and no call site moves (counted). The state is built once a process and copied a cell, as the builder's own rule for a mutating arm asks; `bare` is `build_never_adopted()`.
4. **Bounds by name** — the 2026-10-06 bounds are not readable off a step, so a cell that stands on one says which, from a closed list that holds each one's `DECISIONS.md` entry. A bound makes no cell green.

*Rejected:* **the rig as the builder** — it works on both binaries (ran), and it is the shell probe's builder: a suite that shells out to it has two builders to keep alike. **The prepared states as lists of `Step`s** — a third builder. **A frame on `TrialCorpus` alone** — no limit on a child, no mask, no pool.

### What must not be loosened

- The binary is handed in. Neither the frame nor a cell names one.
- A cell is data and names no path it expects to change.
- A row is committed before its cells run, and is never widened from what the command did.
- An allowance line quotes its sentence, and the fence holds the quote.
- The mask on every child, the manifest's `git` children included.
- The expected-red list is exact three ways wherever cells run in the gate; a bound takes no cell off it.
- A lead-in must exit 0, except that a base cell in `bare` has none. A cell whose lead-in is refused is, on the tree's binary, a fault of the cell; on a named binary its line is kept with why.
- The catalog's own arms — no classification and no second of theirs moves by the seam.

### Red first — the arms that hold the comparator

Each arm is seen red against a comparator that declares the classification and returns *holds* for everything. **The binary under test in these arms is a script the arm hands in**, placed by a child process as the Linux rule for an executable asks. Each arm drives its pair.

| arm | the stand-in does | must come out | its pair, which must not |
|---|---|---|---|
| a plant that is gone is lost whatever the row says | removes a planted file at a place a line lets the command remove, exits 0 | *lost* | the same under the consent spelling, the path named in the output |
| a consent that names nothing | the same, the path not named | red | — |
| bytes that stand elsewhere are not lost | moves a planted file to another path | *written outside*, not *lost* | the move covered by a line |
| held is asked through git's filters | rewrites a file to the same content under line-ending conversion | *holds* | one byte of content changed — *written outside* |
| an empty plant is refused | — the frame, handed a plant with no bytes | a fault of the cell | — |
| the product's own file rewritten is no loss | rewrites a file a lead-in wrote, under a line that covers it | *holds* | the same file with no line — *written outside* |
| a line's condition is read | writes a file a line allows only where the repository has no commit, in one that has | *written outside* | the same with no commit |
| a line's quote is held | — the fence, over a row whose quote is one word off its doc | red | the quote as the doc has it |
| a write outside the repository is seen | writes under `outside/`, `home/`, `tmp/` | *written outside* three times | each covered |
| another checkout is seen | removes a file in a linked worktree; drops its registration | *written outside*; *git's state moved* | the registration at a path a line names |
| a moved-away worktree's index is read | rewrites the admin `index` of a registration whose directory is gone | *git's state moved* | — |
| git's state is seen | drops a stash entry; moves a branch; unstages a path; commits a link where a line declares a doc | *git's state moved* four times | each covered |
| an object that is gone is seen | prunes an unreachable object | *git's state moved* | — |
| a staged doc is accounted for | removes a working area and commits nothing; the doc's bytes at no path | *lost* | the bytes at another path — *git's state moved*; committed at the home — *holds* |
| a refusal may change only what a line lets it | writes one file, exits 1 | *refused, and changed what a refusal may not* | writes under a line that holds on a refusal |
| what is not compared stays quiet, and what is compared is not missed | runs `git status`; changes a modification time | *holds* | **changes one byte and puts size and time back — *written outside*, which is the arm's red against the stub** |
| a command that does not end | sleeps | *the command did not end* | — |
| the order of the classifications | a loss and a write outside at once | *lost* | — |
| the excused are counted | writes under two lines | the arm's block names each line with its count | — |

---

## 5. The acceptance on the previous release

**The arm.** `byte_table::the_cells_are_run_on_the_binary_a_variable_names`, ignored, run by hand, on the catalog's pattern: every cell on the named binary and on the tree's, each from the state that binary built, printed side by side. The variable is the table's own, `JIGC_BYTE_TABLE_BINARY` (*rejected:* the catalog's variable — one export would start both arms). The binary is the one the catalog's pilot record names, built from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, at `<scratch>/rc24-tree.XhX18W/target/release/jigc`.

**What it must show, each cell named.** A table that does not print these lines is the wrong table:

| cell | on `rc.24` | on this tree |
|---|---|---|
| `setup` · `bare`, `HEAD` unborn · **U** at a path the install replaces whole | *lost* | *refuses, nothing changed* |
| — the same, `HEAD` born (the condition's other side) | *refuses, nothing changed* | *refuses, nothing changed* |
| `task finalize` · `committed-singletons`, no baseline · **E** after the task's first write | *lost* | a refusal; its three new files under `.jigc/index/` are covered or not by question 2 |
| — the same, the baseline present | *refuses* | *refuses* |
| `milestone finalize` · `fresh` · **U** at the id the join numbered onto | *lost* | a refusal; `merged/` covered by `design/finalize.md:85` |
| `milestone finalize` · `fresh` · **T** at that id, committed before `milestone create` | ***git's state moved outside its allowance*** — the commit lands a doc on a doc; the bytes that stood there are in history | a refusal, as above |
| `task finalize` · `fresh` · **L** to nothing at the created doc's home | ***git's state moved outside its allowance*** — the commit holds a link where the row declares a doc; the prose stands at another path | a refusal |
| `uninstall` · `fresh` · **D** under `logs/`, under `state/`, under `index/` (three cells) | *lost* three times | *refuses, nothing changed* three times |
| — **D** at `.jigc/`'s root | *refuses, nothing changed* | not run by anybody |
| `milestone finalize` · `fresh` · **W**, its directory moved away | *git's state moved outside its allowance* | *holds* |

The first draft of this table had the link's line as *lost* and the committed doc's as *written outside its allowance*; both contradicted §2 (§11). `(R6, K-1)` has no line and the record says why. The arm counts; it does not say that a difference between the two binaries is a regression.

---

## 6. The records

| what | where | form |
|---|---|---|
| the acceptance rows' allowances, from the docs | `completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md` | `B0`'s: per leaf, each line with its place, its quoted sentence, its doc and heading, its condition; what the docs do not say, listed |
| the frame | `crates/cli/tests/support/byte_cell.rs` | — |
| the suite: the expected-red list, the fences, the comparator's arms, **one arm a prepared state** | `crates/cli/tests/byte_table.rs` | on `route_catalog.rs` |
| the table: a file a family, holding rows and planted cells as data | `crates/cli/tests/byte_table/{install,task,…}.rs` | a file is wired in by one line; it makes no arm of its own — a cell runs in the arm of its state |
| the scheduled list | `crates/cli/tests/byte_table/scheduled.tsv` | a line a row, tab-separated, with its reason |
| the cells that run on demand, as the arm last printed them | `…/reshape/byte-table-on-demand-cells.tsv` | dated; **not held by the gate** |
| the count the bound stands on | `…/reshape/byte-table-leaves-touched-since-rc24.md` | §1.4's range, each leaf read against its handler |
| the findings, by key | `…/reshape/byte-table-findings.md` | on `route-catalog-findings.md` |
| the questions of reading | `…/reshape/byte-table-readings.md` | on `route-catalog-doubts.md`: the words, what a doc answers, what stays open, the cells |
| the pilot's dated record, and the table as the arm printed it | `…/reshape/byte-table-pilot.md`, `…/reshape/byte-table-pilot-cells.tsv` | on `route-catalog-pilot.md` |
| the mechanism | `implementation/pinning.md` → a §7 beside the catalog's §6 | written if the human rules to go on; a paragraph at the stop |
| the decisions, as made | `DECISIONS.md`, an entry a task | dated, with the options not taken |
| the pending row | `implementation/decisions-pending.md` → `wf-pre-byte-matrix` | bracketed as piloted to the stop |

**Keys.** In the grammar the catalog's keys have — `r1-<command>-<what happened, in words>`. A red cell that reproduces a finding with a key keeps that key. Until a stabilization run is opened no ledger holds a key.

**The scheduled list at the stop**, each a row with its reason, owed before the port begins: every pilot leaf outside the first cut, until the human rules on the families · the leaves the count holds untouched · the catalog's states as refusal cells · a plant in a state that is not its leaf's home; two plants at once · the cross with the 17 git states of `posture_door_axis.rs` · the nine ordinary configurations, by pointer to `wf-pre-regression-part-two` · `--format json` as a spelling · the three declared bounds as plants · **a CI job of the table's own, owed at the join with the tooling lane** (§9).

**What the record at the stop must count:** cells, by layer, in the gate and on demand · red, and of them the ones on a declared bound · keys, new and kept · per command · per prepared state · per plant and position · per classification · the same on `rc.24`'s binary, as pairs · the acceptance lines · **per allowance line, the differences it excused** · the cells each open question of §7 hangs on · the arms' seconds — one fan-out cell and one task cell alone, the arms inside a gate, on Linux, each with its load · what each task cost, as the orchestrating session gives it · what the pilot does not establish.

---

## 7. The questions of reading

Each says what a committed doc answers and what stays open. Only what stays open goes to the human, one question at a time. A cell that hangs on an open question follows the stricter reading and is marked.

**1. *Bytes no git object holds* — open. Three readings, each with its reach pinned.**

| reading | *held* means | the reach, exactly |
|---|---|---|
| (a) reachable | from a ref, a checkout's `HEAD`, the index, or the newest stash entry | `git rev-list --objects --all`, each registered checkout's `HEAD`, and every blob `git ls-files -s` lists in each checkout |
| (b) reachable, reflogs included | (a), and what any reflog still names — which is where the older stash entries are | `git rev-list --objects --all --reflog`, and the same indexes |
| (c) in the store | any object, reachable or not | `git cat-file -e <id>` |

No doc says which. The review graded `L-22` a loss with its commit still in the store, and bound (1) treats staged-only bytes as inside the clause; neither names a reach. *Cells that turn on it:* the **S** cells and the **W** cells with the directory moved away. `L-22` is red under all three, by its registration. The check *no object present before is absent after* does not depend on the answer.

**2. *A refusal that left the tree changed* — answered by the docs for what was seen; one thing stays open.**

- *Answered.* `design/storage.md:9`: the working area, the edge index and the file-state hashes are *"either a rebuildable cache or a transient working copy"*. `design/finalize.md:85`: `materialize` runs before the boundary's gate and *"writes only into the gitignored `.jigc/milestones/<id>/merged/`, which it **clears and rebuilds every call**, so blocking after it still writes nothing durable"*. So the two refusals seen on this tree — `.jigc/index/` after a refused `task finalize`, `merged/` after a refused `milestone finalize` — are writes a doc declares, and `rc.24` leaves `merged/` on a refusal too (review ran).
- *The plan's one rule, where the draft said three things:* **a refusal may change only what an allowance line that holds on a refusal covers** — and such a line quotes the sentence that says so. Nothing outside `.jigc/`; nothing of the user's anywhere.
- *Open:* a refusal that leaves behind something a later command reads as **authority** and no sentence declares — a minted task area is the known shape, in the review's `(R4, F1)` and `(R3, F6)`. No question is put for it in general: by rule 4 of §2 each such cell is red until the human rules on it.

**3. A hand edit that rides jigc's commit, and 4. an index entry changed at a path the command does not name — answered per command; open at three.**

| command | what a doc says | open |
|---|---|---|
| `task finalize` | *"The commit set is therefore **the git index** … committed whole … finalize **never** runs `git add --all`: untracked and unstaged working-tree changes stay out"* (`design/finalize.md:164`); a doc-only task's commit is path-scoped. An edit to the doc itself is bound (2) before the first write and a conflict-block after | nothing |
| `setup` | its install pathspec is asked against `HEAD` *"before the install writes a byte"*; non-empty, *"the door refuses having installed **nothing**"* (`design/surface-contract.md:167`) | nothing |
| `rename` | *"a committed-store git transaction end to end (clean-tree precondition, …)"* (`design/write-commands.md:64`) — the review listed this command as open; the sentence answers it: a dirty tree is refused | nothing, unless `B5`'s reading of the `jigc rename` section finds the precondition narrower than the tree |
| the record commands — `milestone create`, `add-task`, `add-from-spec`, `discard` | *"a **separate** record-only path-scoped commit each"* (`design/team-ready-state.md:75`); an edit to the record *"conflict-blocks on drift"* (`:49`) — the review listed these as open; both sentences answer | nothing |
| `migrate-corpus` | *"`git add` over exactly the paths the migration touched … so an ambient dirty tree is never swept in"* (`design/corpus-migration.md:253`) — answers a path the migration does not touch | **an edit in no commit at a path the migration does touch** — the review's `(R4, F5)`, where it rode the commit |
| `relocate` | it commits nothing; the move lands in the index for the user to commit (`BEHALF_DOORS`, a code comment) | **no design doc sentence was found** for what it may do to an uncommitted edit on the doc it moves, or to the index |
| `milestone finalize` | the fan-out commit (`design/finalize.md` → `fan-out` finalize) | **not read for this plan** |

**Three things the first draft decided without saying so.**

- *What touched means* — a choice of this plan, stated in §1.4 with its reason; the human may revisit it at the stop.
- *Which six* — a decision of this plan, stated in §1.2 with its reason.
- *Whether a rewrite of the product's own uncommitted workbench bytes destroys bytes* — **answered**: `design/storage.md:9` calls them a cache or a transient working copy, and `design/finalize.md:212` with `design/storage.md:302` draw the line the clause needs — the product's own files, and the complement, which is never taken silently. The staged prose is the one part of the product's own that must not vanish, by the promote's rule (§2).

Two things that look like questions and are not: whether a link, an untracked file, a foreign worktree, or a directory in a file's place is *a deliberately planted state* — ruled on 2026-10-11, they can block; and whether a file the product moves aside is lost — its bytes are kept, and the move must be a line of the row.

---

## 8. The tasks

Each is single-concern, red first, behind one full `dev/gate`, with its `DECISIONS.md` entry in the same commit. No task changes product source. A red cell is listed with its key and written into the findings file in the commit that adds the cell.

### The first cut

**B0 — the acceptance rows' allowances, from the docs.**
- *Writes:* `…/reshape/byte-table-acceptance-rows.md`.
- *What:* for `setup`, `uninstall`, `task finalize` and `milestone finalize`: every write its owner docs declare, each as a place, what may happen there, the sentence verbatim with its doc and heading, its condition, and whether it holds on a refusal. And what the docs do not say of each command, listed.
- *Done when:* every line's quote is found in its doc by a search for the sentence; the six rows of §5 are each read against the four rows by hand, and the record says for each which line or rule makes it red; the gate.
- *Must not:* run the binary to learn what a command writes; take a file list from the product's source.

**B1 — the frame, and the fence over the leaves.**
- *Writes:* `support/byte_cell.rs`; `support/mod.rs`; `byte_table.rs`; `byte_table/scheduled.tsv`; `groups/g_migration.rs` (one registration).
- *What:* the manifest; the comparator as a pure function; the classifications in their order; the row and the line as data, in the vocabulary `B0`'s rows needed; the fence that holds a line's quote against its doc; and **the fence that holds every `Write` row of `VERB_KINDS` to exactly one answer — a row of the table, a row of the scheduled list, or a row of an owed list in the suite**. All of the pilot's leaves start on the owed list, which the later tasks empty; the leaves the script counts untouched start on the scheduled list with that as their reason.
- *Done when:* every arm of §4's table that needs no sandbox was red against the stub and is green; the two fences were red over a fixture — a write leaf with no answer, a quote one word off; `test_target_registration` green; the gate.
- *Must not:* name a binary; read `.git/` byte for byte; add a manner of write no row of `B0` needed.

**B2a — `TrialCorpus` takes the binary as a value.** *Writes:* `support/trial_corpus.rs`. *Done when:* an arm builds `fresh` on a binary it hands in — a script that records it was started; no call site of the 88 suites moves; the gate. *Must not:* change what any state holds.

**B2b — a prepared state in the sandbox.**
- *Writes:* `support/route_case.rs` (the second constructor; `pub(super)`; `tmp/` as a place); `support/byte_cell.rs` (the runner; a state brought in; `TMPDIR`; `staging/` left out).
- *Done when:* the arms of §4's table that need a sandbox were red and are green; an arm holds that `fresh` brought in from `TrialCorpus` and `bare` with a `setup` step in the sandbox agree **in what can agree** — the same paths, kinds, permission bits and bytes outside `.git`, but for a file that holds a commit id; the same index entries by path, mode and blob; the same ref names — and not in commit ids, which carry a time and the mask's identity where `TrialCorpus` sets its own; the catalog's `cases` and `sites` blocks print byte for byte what they printed; the gate.
- *Must not:* change what an act, a check or a classification of the catalog does; make the sandbox reachable from a suite.

**B2c — the four acceptance rows, as data.** *Writes:* `byte_table/install.rs`, `task.rs`, `milestone.rs` (rows only); the owed list, four leaves shorter. *Done when:* each line is `B0`'s, quote for quote, and the quote fence is green over them; the gate. *Must not:* add a cell; change a line from `B0`'s record without changing the record in the same commit, with why.

**B2d — the acceptance cells, on the tree's binary.**
- *Writes:* `support/byte_cell.rs` (the plants **U**, **T**, **L** to nothing, **E**, **D**, **W** and their positions; the two conditions); `byte_table/` (the cells of §5); `byte_table.rs` (the arms of `bare`, `fresh` and `committed-singletons`; the expected-red list); `.config/nextest.toml` (§9); the findings and readings files.
- *Done when:* the cells come out on the tree's binary as §5's right column says, or the difference is recorded as a finding under a key; the gate.
- *Must not:* touch a row; decide an open question of §7.

**B2e — the arm on a named binary, and the first measurement.**
- *Writes:* `byte_table.rs` (the ignored arm).
- *Done when:* run with `rc.24`'s binary, the arm prints §5's left column, or the record says which line differs and why; **one fan-out cell and one task cell are timed in the frame — alone, and inside the gate of this commit — and the entry says, on those two numbers, which cells the gate runs** (§9); the six states build through `TrialCorpus` on `rc.24`'s binary, or the record says which do not; the Linux line of `dev/runner-faithful` for `byte_table::` and for the whole group is in the entry.
- **The first numbers worth showing the human are this task's.**

**B3 — the rest of the fence, the scheduled list, and the count.**
- *Writes:* `byte_table.rs`; `byte_table/scheduled.tsv`; `…/reshape/byte-table-leaves-touched-since-rc24.md`.
- *What:* every leaf with a row has its refusing cell, its home-state base cell and, for every line of its row and every plant that targets it, a cell at each position, a reason the frame derives, or a scheduled row; every cell belongs to the arm of its state; every cell that runs on demand has a line in the committed table of that arm; every line of a row excuses at least one difference in some cell, or is named; every scheduled leaf is one the count holds; the count itself — **each leaf the script calls untouched read against its handler**, and the range of §1.4 made a number with its derivation.
- *Done when:* each arm was red first over a fixture; the fence is shown not to be vacuous, as the catalog's is; the gate.
- *Must not:* let a leaf be answered twice; type a reason the frame can derive.

**B4 — one family end to end: `task finalize`, `task discard`. The stop.**
- *Writes:* `byte_table/task.rs`; `support/byte_cell.rs` only for a plant a cell needs — **K**, **S**, **N**, **H** — each red first; the expected-red list; the findings and readings files; then, in a closing commit, `…/reshape/byte-table-pilot.md`, the two printed tables, and the pending row's bracket.
- *What, in three commits:* `task discard`'s row, from its docs, as `B0` did the four; the two leaves' cells — refusing, base, planted, the consent cell of `task discard --force`; the record.
- *Done when:* the family's gate cells are followed and the list is exact; the on-demand cells are printed once and committed; §6's counts are in the record, each naming its source; the gate.
- *Must not:* construct an ordering `copy_in_baseline.rs` already sweeps; widen a row from what a cell showed.
- **This is the stop.** The record goes to the human with: cells, red and keys; what each allowance line excused; seconds a cell; what a row and a cell cost to make; the questions that stayed open. The human rules on the families after it, on what *touched* means, and on the gate's share.

### What would follow, if the human rules for it

`B5` `install` (`setup`, `uninstall`: the remaining cells, plant **M**) · `B6` `doc_write` (the eight `doc` leaves, in two parts) · `B7` `milestone` (the eight, in three parts; a state that holds worktrees is never copied — a cell provisions after the copy) · `B8` `movers` (`rename`, `relocate`, `migrate-corpus`, `config set`) · `B9` `mint` (`start`, `workflow`, `migrate`, `task amend`), which empties the owed list and removes it with its type · `B10` the full record and `pinning.md` → §7. Each on `B4`'s pattern: the rows from the docs in a commit of their own, then the cells, about ten constructed cells a commit.

---

## 9. What it adds to the gate and to CI

**Nothing here is promised before it is measured.** The first draft put a cell at 1.0 to 1.5 times a catalog case. That is contradicted by this plan's own runs — 3.2 to 35.8 s a cell on the tree and 4.7 to 74.6 s on `rc.24`, in shell with a rig build and an interpreter a cell, under load — and by the review's, 4 to 22 s a cell at a load of 50, 13 to 36 s for one that needs a fan-out. The catalog's own record says its cost a case does not carry to a command whose state is an open task or a milestone (`route-catalog-pilot.md:504`). About fifty fan-out cells at 13 to 36 s would be 11 to 30 minutes of summed test time. **So one fan-out cell and one task cell are timed in the Rust frame at `B2e`, and the gate's share is decided there.**

**The design to plan for.**

- **In the gate:** the planted cells, the refusing cells and the home-state base cells. At the stop that is about 33 cells, three of them fan-outs.
- **On demand:** the base cells in the six states that are not a leaf's home — an ignored arm, run by hand, as the catalog's named-binary arm is. At the stop 12 cells; 168 for the whole pilot.
- **What that costs:** the expected-red list is exact only where cells run. An on-demand cell has a committed line of what the arm last printed and nothing holds it between runs: a write leaf that starts losing bytes in `vendored` and not in its home state is seen when somebody runs the arm, not at the commit that did it. The record says when the arm was last run; a stabilization round's preflight is the natural place to run it, and that is not this plan's to decide.

**The group under the gate's runner — a choice.** The table's arms **join the catalog's one-at-a-time group** rather than run beside it. *Reason:* each arm is a pool as wide as the machine; in this worktree's last full gate four of the catalog's arms took 114 to 169 s each, one behind the other (read: `target/jigc-gate/test-seconds.tsv`), against a runner that ends a test at 480 s — a second pool beside them slows both toward that limit. *Price:* the table's seconds add to that chain end to end. It is one more term in one filter line of `.config/nextest.toml`. The human may revisit it at the stop on `B2e`'s numbers.

**CI.** The table joins `g_migration`, whose leg exists: **no new CI job in this lane before the join.** The tooling lane has just changed the CI matrix and its fence, and a second change to them from this branch would collide at the merge; a leg of the table's own, if the numbers ask for one, is owed at the join and is a row of the scheduled list. The leg's headroom is known: on this branch's tip, run 38294853708, the step `cargo test --test g_migration` ran 196 s of its 600 s bound (ran: `gh run view`, 19:01:57 to 19:05:13). What the table adds to it is `B2e`'s Linux line.

**What it costs to build** — arithmetic over the catalog's ceilings (`route-catalog-pilot.md` → §2), which the review checked as arithmetic and which the record's line 504 says may not carry over: the frame's fixed part there was at least 9,503 wall seconds and 649 k subagent tokens; a site cost 877 s and 48.8 k, a case 132 s and 7.4 k. With a row priced as a site and a constructed cell as a case, the first cut — five rows, about 33 cells — is **about 5 hours of builder wall time and 1.1 M subagent tokens** (9,503 + 5 × 877 + 33 × 132 s; 649 + 5 × 48.8 + 33 × 7.4 k), `B0` not counted; the whole pilot, at 28 rows and 110 cells, was 13.5 hours and 2.83 M before the review added plants. Not a measurement.

---

## 10. Risks, and what is not established

**Risks.**

1. **A wrong allowance is still a wrong green — for what is not planted.** The plant rule takes the row out of the question for a plant; for every other difference the comparator excuses what a line covers. *Held by:* the quote, which the fence checks against the doc; the row committed before its cells; the count of what each line excused; and `B0`, which reads the docs before there is a vocabulary to bend them to.
2. **The table finds siblings, not classes.** Its plants and their positions come from six known rows. A loss that needs a state nobody has seen is found by a review, as before.
3. **The gate.** A fan-out cell is slow, and the first cut has three. The whole pilot would have about fifty.
4. **On-demand cells go stale unseen.**
5. **A lead-in is not under the comparator.** Each lead-in is a leaf with cells of its own, which is most of an answer.
6. **A prepared state copied into another frame's sandbox** — `B2b`'s arm is the check, and it cannot compare commit ids.
7. **Two instruments over one state.** A planted cell and a per-command suite can disagree after a fix; the row's pointer to the suite tells a reader both exist.
8. **A shared file edited from two lanes** — `.config/nextest.toml`, one term.

**Not established — by this plan or by its review.**

- `(R3, F7)` and `L-22` beyond this plan's own output; nobody ran them twice.
- A cell's seconds in the Rust frame, on a quiet machine, inside a gate, or on the hosted runner.
- That the expected-red list is the same on Linux.
- The planted count — at the stop and for the whole pilot.
- That the six prepared states build through `TrialCorpus` on `rc.24`'s binary. Through the rig they do.
- What `leaf_argv.rs` and `BEHALF_DOORS` hold beyond the lines this plan quotes; no reviewer read them.
- What the catalog's later families cost a case — only its first two commands were priced.
- That the frame's comparator classifies the acceptance cells as the shell manifests showed them; that `uninstall` on this tree refuses a file at `.jigc/`'s root.
- The vocabulary of the allowance: `B0` and `B1` establish it.
- That every leaf has an acting spelling with lead-ins a `Step` can express — a step's argv is fixed text, and a payload goes in by a file under `outside/`, which was not tried.
- The touched count as a number: 26 to 30 by reading, with six leaves read by nobody.
- What `milestone finalize`'s docs declare its commit to hold.
- That `(R1, F1)`'s second repro, the audit's four older losses, and the catalog's *seen by hand, and no case* at `uninstall` reproduce as cells.
- Whether any command writes under `$HOME` or leaves a file under `TMPDIR`: none of the six cells wrote under `home/` on either binary (ran); `TMPDIR` was not redirected in any run.

---

## 11. What the review changed

An independent reviewer read the first draft against the worktree and re-ran four of the six rows on both binaries. Its verdict: sound in its frame reuse, its records and its honesty about the bound; the losses real on `rc.24` and gone on the tree; three things to settle before tasks are cut. Each finding was checked against the code and the docs while it was worked in; **the lines it cites say what it says they say**, with the one exception in the last row.

| finding | what the plan said | what it says now |
|---|---|---|
| **The allowance cannot carry what the docs declare.** *Removes* excused nothing while `design/project-setup.md:154,160` declare that `uninstall` removes `.jigc/`, so `(R9, F5)` was not *lost* by the plan's own definition; *commits* contradicted `design/finalize.md:164,218`; *replaces* read every clean `doc create` as a loss (`provenance.json`, in this plan's own output); `setup`'s root `.gitignore` is declared under a condition | six manners of write, fixed by the plan; an allowance cited a heading | no vocabulary is fixed here; `B0` writes the acceptance rows from the docs and `B1` builds what they needed. A plant is the user's by construction and its loss is *lost* whatever the row says, but for a consent cell. A line quotes its sentence and the fence holds the quote; a line may carry a condition; a row is committed before its cells and never widened from a run; the arm counts what each line excused (§2, §4, §8) |
| **One acceptance line contradicted the definition of *lost*.** In `(R6, D-7)` the prose stands byte for byte at another path | the link's cell must come out *lost* on `rc.24` | *git's state moved outside its allowance*: the commit holds a link where the row declares a doc. A staged doc that is not in its commit is classified so unless its bytes stand nowhere (§2, §5) |
| **A plant had no position.** Placed by the draft's rule, three of the rows' plants draw a refusal on `rc.24` | each plant once, in the home state, after the lead-ins | each kind has a position rule; a condition a doc names is run on both sides; the committed half of `(R6, D-1)` is committed before `milestone create`, and its line is corrected; and §0 says that the table finds the same plant at sibling commands, not new classes of loss (§1.2, §3, §5) |
| **The pilot had no stop.** | ten tasks, `B1` to `B10`, with two points to show numbers | the first cut is `B0` to `B4` and ends at a stop where the human rules on the rest; `B5` to `B10` are what would follow (§0, §8) |
| **The touched count was a script's, and the script drops edges.** `load` and `save` are skipped, and `ingest` and `unmanage` call both one step from the handler | 28 of 36, with 8 scheduled | 26 to 30 by reading; *touched* is a choice the human may revisit; `B3` reads the leaves counted untouched (§1.4) |
| **The cost was promised, not measured.** The factor against a catalog case is contradicted by both sets of runs | 2 minutes alone, 3.5 to 10 minutes in a loaded gate, every cell in the gate | nothing promised; two cells timed in the frame at `B2e`; planted, refusing and home-state base cells in the gate, the other base cells on demand, with what that costs; the arms join the catalog's group; no new CI job before the join; the leg's headroom from a run (§9) |
| **Questions went to the human that the docs answer, and three decisions were made without one.** | four questions, each with two readings | each question says what a doc answers and what stays open; the first has three readings with each reach pinned; the second is one rule; *touched*, the choice of six, and the product's own workbench bytes are stated as decisions or answers (§7) |
| **The seam is more than visibility.** `Sandbox::new` commits; `staging/` is under the root; the product writes under the temporary directory; the *same manifest* arm cannot hold on commit ids | visibility only; an arm that held the two builders to the same manifest | a second constructor, `pub(super)`, `tmp/` in the manifest, `staging/` out; the arm holds what can agree (§4, §8) |
| **`B2` held five concerns, rows were born before the fence, and three smaller things.** An arm per family in one section and per state in another; *a lead-in must exit 0* against base cells in `bare`; an arm that could not be red | one `B2`; the fence at `B3`; both arm rules; no word on `bare` | `B2a` to `B2e`; the leaf fence in `B1` with an owed list; an arm a state; a base cell in `bare` has no lead-ins (run for five leaves); the quiet arm has a pair that is red (§2, §4, §6, §8) |
| **Four plants were missing.** | seven kinds | eleven: a directory in a file's place and the reverse, a rejecting hook, a user's line in an uncommitted file jigc merges into, a name git quotes (§3) |
| **Four notes.** | — | *held* through git's filters; a plant's bytes unique and not empty; *no object present before is absent after*; a moved-away worktree's index read from its admin directory (§2) |
| **Where the reviewer is wrong.** It lists `rename` and the record commands among those whose commit the docs leave open | — | `design/write-commands.md:64` gives `rename` a *clean-tree precondition*; `design/team-ready-state.md:75` and `:49` give the record commands a path-scoped commit and a conflict-block on drift; `design/corpus-migration.md:253` answers `migrate-corpus` for every path the migration does not touch. Open are `migrate-corpus` at a path it does touch, `relocate`, and `milestone finalize`, which this plan did not read (§7) |
