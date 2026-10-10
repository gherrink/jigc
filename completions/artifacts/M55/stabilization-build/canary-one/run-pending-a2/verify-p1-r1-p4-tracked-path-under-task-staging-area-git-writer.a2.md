# verify-real — `r1-p4-tracked-path-under-task-staging-area-git-writer` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-tracked-path-under-task-staging-area-git-writer`. One finding, handed
over: ledger key `r1-p4-tracked-path-under-task-staging-area-git-writer`, door *unlisted — the git
processes jigc spawns, as writers into a task's staging area*, the clause it is said to break
`no-lost-files`, triage's grade *unclear*. It has no block of its own: its source is item 5 of
*Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-staging-area-writers-undriven-and-unread-remainder.a1.md`
— *a path under `.jigc/tasks/<id>/docs/` that is tracked in git (force-added past
`.jigc/.gitignore`) is the one state in which a git process jigc spawns would write into a staging
area. Read, not driven; no jigc verb creates it.* That report was read because the prompt hands it
over as the finding; no other report was read, and nothing of triage's reasoning beyond the grade.

## Verdict

**`refuted` — basis `breaks-no-clause`.** Not `does-not-reproduce`: the sentence the finding rests
on is true, and was driven.

- **The write is real.** On the planted state, `jigc milestone finalize` exits 0 and its
  `git merge --ff-only` replaces a staged doc in an open task's area in the main checkout — another
  inode, 283 bytes become 333 — and creates an entry under a `.jigc/tasks/<name>/docs/` that no
  task owns. The door says so itself: `modified .jigc/tasks/<task>/docs/vision:vision.md`.
- **It takes two acts no jigc verb performs.** The path has to be force-added past
  `.jigc/.gitignore` and committed, **and** somebody has to edit git's copy of it inside a sub-task
  worktree and stage that edit there. With the first act alone, every git-spawning door driven left
  the area as it was: 12 reads, 12 unchanged.
- **Nothing the clause names happens.** Where git replaced the staged doc, the bytes it replaced
  were the ones the plant's own commit holds (the same sha256), and the commit carries exactly what
  was staged in the worktree. Where the staged doc held bytes no git object holds, the same door
  **refused**: exit 1, git's own *Your local changes … would be overwritten by merge*, nothing
  committed, `HEAD` where it was, the area byte-identical on the same inodes.
- **The previous release does the same**, cell for cell: the same exit status in all 63 cells, the
  same result of every read of the area, and 125 of 126 streams byte-identical.

`contested: false` — the finding argues against no settled decision.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a2/jigc`. The block below stops (exit 90) unless `command -v jigc` and the
  rig's `$JIGC` both print the binary it was handed; both runs passed it.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  refs-post-hoc`, stdout captured alone, the construction log to a file of its own, exit 0 each
  time.
- The clone: `HEAD` 126a8531 on `fix/canary-one`; `git diff --stat eeffe347 HEAD -- crates dev` is
  empty, so the source read below is the candidate's. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0, arm64), git 2.54.0 (Apple Git-157), as a non-root user.
Nothing was driven on Linux, and nothing as root.**

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-git-writer.iFxY83` (written `<W>`). Under it:

- `<W>/c1.rig/` with `<W>/c1.run/` (candidate) and `<W>/p1.rig/` with `<W>/p1.run/` (previous
  release) — **every number in this report is read from these two**, each binary on a fresh rig of
  its own, the same block on both: 63 cells each, 54 `jigc` invocations and 9 plain `git` ones.
- `<W>/explore/` and `<W>/explore2/` — two exploration rigs, candidate only, used to learn the
  verbs' spellings and the order the block needs. Two facts are read from them and say so.
  `<W>/c.rig/` was built, read once and never driven.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. Each cell runs in the directory it names, stdin from `/dev/null` or a
file, stdout and stderr to files of their own, the exit status read directly and never through a
pipe. A *read of the area* is `find .jigc/tasks/<task>/docs -mindepth 1` handed to
`stat -f '%HT|%Sp|%z|%i|%m|%c|%N'` — shape, mode, size, inode, mtime, ctime, without following a
link — plus the sha256 of every file, plus whether `.jigc/tasks/ghost-task/docs/adr:ghost.md`
exists; two reads are compared byte for byte.

**A deviation, stated.** The block was typed as one shell function inside the command that ran it;
no driver file exists. The shell wrote the small stdin payloads and the captured streams under
`<W>`. The file tool wrote this report and nothing else.

## What was driven, and what came back

The fixture is `refs-post-hoc`: its live task `ground-the-vision-in-research` (written `<T>`) holds
three entries in `.jigc/tasks/<T>/docs/` — `commit:<T>.md`, `provenance.json`, and
`vision:vision.md`, a staged copy of the committed `VISION.md`.

### 0 · The state needs force, and no jigc verb makes it

| cell | what was run | exit | what it shows |
|---|---|---|---|
| 1 | `git add .jigc/tasks/<T>/docs/vision:vision.md` | 1 | *The following paths are ignored by one of your .gitignore files: .jigc/tasks* |
| 2 | `git add -A` | 0 | nothing staged; `git ls-files -- .jigc/tasks` prints nothing, as it did before |
| 3–4 | `git add -f .jigc/tasks/<T>/docs .jigc/tasks/ghost-task/docs/adr:ghost.md` · `git commit -q -m …` | 0 · 0 | **the plant**: four tracked paths under `.jigc/tasks/`; the ghost file is then removed from the working tree (` D`) |

On the rig as its builder leaves it — set up, a committed corpus of five jigc-made commits, one
task open — git tracks no path under `.jigc/tasks/`; at the block's end it tracks the plant's four
and no other, after two landed milestones, a landed task and a landed amend. Read beside that:
production code has ten `git add` call sites and each is `add -- <paths>` with no force flag
(`migrate_corpus.rs:481`, `rename.rs:1110` and `:1112`, `task.rs:6123`, `:6830`, `:6855`, `:9523`,
`:9531`, `milestone.rs:1007`, `setup.rs:4594` — one `grep` for the word, each hit classed against
its file's first `#[cfg(test)]`).

### O · The staged doc equals what the plant committed; a sub-task worktree stages an edit of git's copy

| cells | what was run | exit | what came back |
|---|---|---|---|
| 5–7 | `jigc milestone create "Overwrite probe"` · `add-task overwrite-probe "edit the tracked copy"` · `provision overwrite-probe` | 0 · 0 · 0 | the worktree `.jigc/worktrees/edit-the-tracked-copy/` holds git's copy of all four planted paths under its own `.jigc/tasks/` |
| 8 | in the worktree: `jigc workflow sub-task --task edit-the-tracked-copy` | 0 | |
| 9 | a line appended to the worktree's `.jigc/tasks/<T>/docs/vision:vision.md` and to its ghost file; in the worktree: `git add -u` | 0 | both staged (`M ` twice) |
| 10–11 | in the worktree: the sub-task's commit doc, `set-field …#header/type` and `set-slot …#summary` | 0 · 0 | |
| 12 | `jigc milestone join overwrite-probe` | 0 | the area: **unchanged** |
| 13 | `jigc milestone finalize overwrite-probe` | **0** | `finalized <sha> — Finalize milestone overwrite-probe (1 sub-task)` · `modified .jigc/tasks/ghost-task/docs/adr:ghost.md` · `modified .jigc/tasks/<T>/docs/vision:vision.md` · `modified docs/milestone-records/overwrite-probe.md` · `3 files committed` · `sub-tasks: edit-the-tracked-copy: 1 doc, 2 code files` |

After cell 13 the area's read is **changed**: `vision:vision.md` is another inode, 283 bytes became
333, sha256 `333772c8…` became `5083e36f…`, and it ends with the line appended in the worktree; the
other two entries keep inode, size and hash. The ghost file exists again in the main checkout.
`git status --porcelain` is empty.

**What the replaced bytes were.** Before the door ran, the staged doc's sha256 and the sha256 of
`git cat-file -p <plant commit>:.jigc/tasks/<T>/docs/vision:vision.md` are the same value,
`333772c8b77bbd615f9128b53536b800263c997c6bb3ff33c175d40d738b1310`. A git object holds them, and
the plant's commit is an ancestor of `HEAD`.

**What jigc makes of the entry git created** (cells 14–16): `jigc task list` lists one task, `<T>`,
and no `ghost-task`; `jigc doc list --task ghost-task` exits 1 with `finalize.no-task` — *a
directory carrying no base pin, so it is a leftover and not a work unit* — and a route that says
nothing was changed; `jigc doc list --task <T>` exits 0 with its two rows.

### M · The staged doc holds bytes no git object holds; every git-spawning door, watched

The ghost file is removed from the working tree again. Then cell 17, `jigc doc set-slot
vision:vision#thesis --from-file - --task <T>`, exits 0: the staged doc is now sha256 `460755c3…`
where `HEAD` holds `5083e36f…`. **This is the control** — the read after it reports *changed*
against the read before it, so the instrument sees a write — and it is the reference every read
below is compared with.

| cells | the door | what git does in it | exit | the area |
|---|---|---|---|---|
| 18 | `jigc validate` | readers | 0 | unchanged |
| 19 | `jigc setup` (a re-run) | readers; re-installs | 0 | unchanged |
| 20 | `jigc migrate-corpus` | readers | 0 | unchanged |
| 21–25 | `jigc start --workflow single-task --slug note-three …` · a file written and `git add`ed · the commit doc filled · `jigc task finalize note-three` | `add`, `commit` in the live checkout, with the hook | 0 | unchanged; the commit holds `README-note.md` and nothing else; the door names the two planted paths as *left-out (unstaged/untracked — git add to include)* |
| 26–29 | `jigc task amend "reword the last commit"` · the commit doc filled · `jigc task finalize reword-the-last-commit` | `commit --amend`, with the hook | 0 | unchanged; the tree is the amended commit's |
| 30–32 | `jigc milestone create "Fan out two notes"` · `add-task` twice | three record-only commits | 0 | unchanged (two reads) |
| 33 | `jigc milestone provision fan-out-two-notes` | `worktree add` twice | 0 | unchanged; each worktree holds git's copy of the four planted paths |
| 34–37 | `jigc doc show vision:vision#thesis --task <T>` from the main checkout and from a worktree · `jigc task list` and `jigc doc list --task ghost-task` from the worktree | — | 0 · 0 · 0 · 1 | **the two `doc show` outputs are byte-identical and are the main checkout's bytes**; git's copy in the worktree is sha256 `5083e36f…`, the older text. The worktree's own `.jigc/tasks/` is not read: three tasks listed, no `ghost-task` (`finalize.no-task`) |
| 38–45 | in each worktree: `jigc workflow sub-task --task <id>` · a file written and `git add`ed · the commit doc filled | — | 0 | unchanged |
| 46 | `jigc milestone join fan-out-two-notes` | — | 0 | unchanged |
| 47 | `jigc milestone finalize fan-out-two-notes` | a dedicated worktree, `read-tree --reset -u`, the commit with the hook, `merge --ff-only`, `worktree remove` | 0 | unchanged; the commit holds the record and the two notes; the ghost file is **not** back |
| 48 | `jigc rename research:context-loss --to "Context loss again"` | — (refused before its `git mv`) | 1 | unchanged; `rename.in-flight` |
| 49–50 | `jigc config set docs-root .jigc/tasks/<T>/docs` · the same for `placement-root` | — | 1 · 1 | `config.workbench-root`: a doc's committed home cannot be put inside the area |

Twelve reads of the area in this section, twelve unchanged.

### R · The staged doc still holds those bytes; a sub-task worktree stages an edit of git's copy

| cells | what was run | exit | what came back |
|---|---|---|---|
| 51–57 | `jigc milestone create "Refuse probe"` · `add-task` · `provision` · in the worktree: the re-entry, a line appended to git's copy of `vision:vision.md`, `git add -u`, the commit doc filled | 0 | the edit is staged in the worktree (`M `) |
| 58 | `jigc milestone join refuse-probe` | 0 | the area: unchanged |
| 59 | `jigc milestone finalize refuse-probe` | **1** | stdout empty; stderr `` `git merge --ff-only <sha>` failed: error: Your local changes to the following files would be overwritten by merge: .jigc/tasks/<T>/docs/vision:vision.md … Aborting `` and then *milestone:refuse-probe is intact — nothing was committed, the merged docs were rolled back, and every provisioned sub-task worktree still holds its staged code.* |
| — | | | **the area: unchanged** — the same inodes, sizes and hashes; `HEAD` has not moved; `git status` as before |
| 60 | `jigc milestone discard refuse-probe --force` | 0 | the area: unchanged |

### F · The task whose own area is tracked

Cells 61–63: the commit doc of `<T>` filled (0 · 0), then `jigc task finalize <T>` — **exit 3**,
`finalize.base-mismatch`: *the moved history overlaps the task's work on* the three planted paths
that differ from `HEAD`. Nothing committed, `HEAD` not moved, the area's three entries present, the
staged doc still sha256 `460755c3…`. The route it prints was not driven (item 2 of *Left open*).

### The totals, and the previous release

**Candidate, `<W>/c1.run/`:** 63 cells — 55 exit 0, 7 exit 1, 1 exit 3. The seven: the plain
`git add` of an ignored path (cell 1), `finalize.no-task` for the ghost twice (15, 37),
`rename.in-flight` (48), `config.workbench-root` twice (49, 50), and the refused boundary (59). The
one exit 3 is cell 63. **18 comparisons of the area: 16 unchanged, 2 changed** — cell 13, where git
wrote, and cell 17, the control. The sixteen: cell 12, the twelve of section M, and the three of
section R.

**Previous release, `<W>/p1.run/`,** the same block on a rig built with `--binary
<scratch>/bin/previous-91834b5e011d/jigc`. **Not the regression step** — the verdict is not
`confirmed`, and no `regression` field is returned — but it says whether this is the candidate's
doing. It is not: the same exit status in all 63 cells; the same 16 unchanged and 2 changed, on the
same two cells; the same file hashes before and after cell 13 (`333772c8…`, `5083e36f…`). Of the
126 streams, 125 are byte-identical with the rig's path and the commit hashes normalised. The one
that differs is cell 60's stderr: the candidate adds three lines naming what the discard takes with
the worktree — *`.jigc/tasks/<T>/docs/vision:vision.md` (staged, in no commit)* — which the
previous release does not print.

## Does it break the clause, inside its scope

The clause, in the closing condition's words (`DECISIONS.md`, 2026-10-04, *The exit rule, revised*,
sharpening 1): *In a healthy repository used as documented — which includes ordinary git
configuration: line-ending conversion, `status.showUntrackedFiles`, a symlinked `CLAUDE.md`, linked
worktrees — no jigc command at exit 0 destroys bytes no git object holds or commits content the
user did not ask for. Where jigc cannot tell (git fails, the index is unreadable) it refuses before
writing. Races against a non-jigc writer inside a millisecond window, and deliberately planted
states, are declared bounds, written down with their reach.*

Taken in two steps, because the second holds without the first.

**1. The state is not one documented use reaches.** `design/storage.md` (*The per-task working
area (staging)*): *a task's staging area is a gitignored scratch dir of working copies*;
`design/team-ready-state.md`: *`.jigc/` is a pure working directory — WIP staging, per-machine
caches, worktrees. Gitignored as a matter of identity, not convenience.* Git refuses the path
without `-f` (cell 1), `git add -A` passes it by (cell 2), and no jigc verb tracks it (section 0).
That alone moves nothing in the area — section M. For git to write there, a second hand has to
edit a file under `.jigc/worktrees/<sub-task>/.jigc/tasks/` and stage it; jigc itself never reads
or writes that copy (cells 34–37). The linked worktrees in this block are jigc's own fan-out
worktrees, used as the milestone verbs make them; what is not ordinary is the forced add and the
hand edit, and neither is a git configuration.

**2. On that state, what the clause forbids still does not happen.**

- *Destroys bytes no git object holds* — no. The boundary's last git step is a fast-forward merge
  in the live checkout, and git's own rule decided both cells: a staged doc that is byte-for-byte
  the committed blob was replaced (O — the bytes are in the plant's commit), and one that differs
  from it was refused over (R — exit 1, nothing moved). That is the contract the door's source
  states — `crates/cli/src/task.rs:9479`–`9489`, *carry-or-refuse, never destroy*, with
  `design/finalize.md` → `fan-out` finalize named there as its home.
- *Commits content the user did not ask for* — no. `design/finalize.md` → *Dirty-tree policy*: *the
  commit set is the git index, not a sweep of the working tree.* The boundary's commit in O carries
  the two paths staged in the worktree by `git add -u` and names both in its own report; the three
  landed commits of M carry none of the planted paths, and the task door lists them as left out.
- *Where jigc cannot tell it refuses before writing* — R and F are refusals: exit 1 and exit 3,
  nothing committed, the area intact.

So the finding is a true statement about a planted state, and a row of the ledger; it is not a
blocker under `no-lost-files`. One thing about the scope is not this verifier's to settle and is
item 1 of *Left open*: the clause says planted states are *declared bounds, written down with
their reach*, and this run declares none.

## Scope of what was verified

**`instance, unbounded`.** Driven: one fixture; one planted doc of one doctype (`vision`, a staged
copy of a committed doc) and one planted file under a directory no task owns; the plant committed,
never left in the index alone; the default (squashed) milestone boundary; two binaries; one
platform; a non-root caller. The git call sites of production code were **not** enumerated again —
the source report's reading of them was used to choose the doors, and four were read here
(`task.rs:7441` the rollback's `restore --source=HEAD --worktree`, `:9294` the commit seam, `:9491`
the overlay and fast-forward, `:9628` the dedicated worktree). The one count taken here is the ten
`git add` sites of section 0.

## Repro T-1

```yaml
claim: "a path under .jigc/tasks/<id>/docs/ that is tracked in git is a state in which a git process jigc spawns writes into a task's staging area — and that breaks no-lost-files"
verdict: REFUTED   # basis breaks-no-clause: the write is real on a planted state; no byte without a git object is destroyed at exit 0, and nothing unstaged is committed
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exit status in all 63 cells, the same result of every read of the area, 125 of 126 streams byte-identical (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2 arm64, git 2.54.0, a non-root caller; NOT driven on Linux, NOT driven as root"
setup:
  - fixture: refs-post-hoc          # its live task <T> = ground-the-vision-in-research; <A> = .jigc/tasks/<T>/docs
  - ["git", "add", "<A>/vision:vision.md"]                          # control: exit 1, the path is ignored
  - ["git", "add", "-f", "<A>"]                                     # the plant
  - ["git", "commit", "-q", "-m", "chore: track a task area (a plant)"]
repro:
  # O — the staged doc equals the committed blob
  - ["jigc", "milestone", "create", "Overwrite probe"]
  - ["jigc", "milestone", "add-task", "overwrite-probe", "edit the tracked copy"]
  - ["jigc", "milestone", "provision", "overwrite-probe"]
  - ["jigc", "workflow", "sub-task", "--task", "edit-the-tracked-copy"]                     # cwd: .jigc/worktrees/edit-the-tracked-copy
  - write: ".jigc/worktrees/edit-the-tracked-copy/<A>/vision:vision.md — one line appended"
  - ["git", "add", "-u"]                                                                    # cwd: that worktree
  - ["jigc", "doc", "set-field", "commit:edit-the-tracked-copy#header/type", "--value", "docs", "--task", "edit-the-tracked-copy"]
  - ["jigc", "doc", "set-slot", "commit:edit-the-tracked-copy#summary", "--from-file", "-", "--task", "edit-the-tracked-copy"]   # stdin: one line
  - ["jigc", "milestone", "join", "overwrite-probe"]
  - ["jigc", "milestone", "finalize", "overwrite-probe"]
  # M — the staged doc holds bytes no git object holds
  - ["jigc", "doc", "set-slot", "vision:vision#thesis", "--from-file", "-", "--task", "<T>"]  # stdin: one line of prose
  - ["jigc", "start", "--workflow", "single-task", "--slug", "note-three", "add a third readme note"]
  - write: "README-note.md — one line"
  - ["git", "add", "README-note.md"]
  - ["jigc", "doc", "set-field", "commit:note-three#header/type", "--value", "docs", "--task", "note-three"]
  - ["jigc", "doc", "set-slot", "commit:note-three#summary", "--from-file", "-", "--task", "note-three"]
  - ["jigc", "task", "finalize", "note-three"]
  # R — the same edit of git's copy, staged in a second milestone's worktree
  - ["jigc", "milestone", "create", "Refuse probe"]
  - ["jigc", "milestone", "add-task", "refuse-probe", "edit the tracked copy again"]
  - ["jigc", "milestone", "provision", "refuse-probe"]
  - ["jigc", "workflow", "sub-task", "--task", "edit-the-tracked-copy-again"]               # cwd: .jigc/worktrees/edit-the-tracked-copy-again
  - write: ".jigc/worktrees/edit-the-tracked-copy-again/<A>/vision:vision.md — one line appended"
  - ["git", "add", "-u"]                                                                    # cwd: that worktree
  - ["jigc", "doc", "set-field", "commit:edit-the-tracked-copy-again#header/type", "--value", "docs", "--task", "edit-the-tracked-copy-again"]
  - ["jigc", "doc", "set-slot", "commit:edit-the-tracked-copy-again#summary", "--from-file", "-", "--task", "edit-the-tracked-copy-again"]
  - ["jigc", "milestone", "join", "refuse-probe"]
  - ["jigc", "milestone", "finalize", "refuse-probe"]
expect:
  - setup: "the plain add exits 1 (`The following paths are ignored`); the forced add and the commit exit 0; `git ls-files -- .jigc/tasks` now prints the three paths of <A>"
  - step: "milestone finalize overwrite-probe"
    exit: 0
    stdout_contains: "modified .jigc/tasks/ground-the-vision-in-research/docs/vision:vision.md"
    tree: "<A>/vision:vision.md now ends with the appended line; the bytes it held before the step are byte-for-byte `git cat-file -p <plant commit>:<A>/vision:vision.md`; the other two entries of <A> are unchanged"
  - step: "doc set-slot vision:vision#thesis"
    exit: 0
    tree: "<A>/vision:vision.md differs from `git cat-file -p HEAD:<A>/vision:vision.md`"
  - step: "task finalize note-three"
    exit: 0
    stdout_contains: "left-out (unstaged/untracked — git add to include):"
    tree: "the commit's only path is README-note.md; every entry of <A> is byte-identical to what it was before the step"
  - step: "milestone finalize refuse-probe"
    exit: 1
    stderr_contains: "would be overwritten by merge"
    tree: "HEAD has not moved; every entry of <A> is byte-identical to what it was before the step"
  - every other step: { exit: 0 }
variants:   # each driven in the same block, on both binaries, unless it says otherwise
  - "a second planted file, .jigc/tasks/ghost-task/docs/adr:ghost.md, committed and then deleted from the working tree, with an edit of git's copy staged in the first worktree: the first boundary writes it back into the main checkout (exit 0); `jigc task list` does not list ghost-task and `jigc doc list --task ghost-task` exits 1, finalize.no-task; no later door writes it back once it is deleted again"
  - "with the staged doc differing from HEAD: `jigc validate`, `jigc setup`, `jigc migrate-corpus`, a landed amend, and a milestone of two sub-tasks that lands (create, add-task twice, provision, join, finalize) — exit 0 each, the entries of <A> byte-identical after every one"
  - "`jigc doc show vision:vision#thesis --task <T>` from a sub-task worktree prints the main checkout's staged bytes, never git's copy under the worktree's own .jigc/tasks/"
  - "`jigc task finalize <T>` — the task whose area is tracked — exits 3, finalize.base-mismatch; nothing committed, <A> intact"
  - "the block exactly as written above, with no ghost file: driven once, in the second exploration rig, candidate only, as far as the first boundary — exit 0, `2 files committed`, the same replacement"
control: "`jigc doc set-slot vision:vision#thesis --from-file - --task <T>` exits 0 and the next read of <A> reports another inode and another sha256 for vision:vision.md — the read sees a write"
observed: "<W>/c1.run/log with <W>/c1.run/cells/ and <W>/c1.run/watch/; the previous release: <W>/p1.run/"
pinned-by: "UNPINNED: found this round. Searched: `command grep -rn` for a forced `git add` under crates/cli/tests, crates/engine/tests and tooling-tests gives seven sites in five files (uninstall_worktree_guard.rs, freeze_enforcement.rs, repo_relative_paths.rs, uninstall_workbench_subject.rs, path_arg_occurrence_axis.rs); none names a path under `.jigc/tasks`. No suite was run by this verifier"
```

**Pinnable as it stands: yes, by hand conversion.** The fixture is a named state of the shared
builder, every step is an argv or a one-line file write, and each assertion is an exit status, a
substring of a stream, or the bytes of a named file compared with a git blob — no inode, no mode,
no umask. Two notes for whoever converts it: three steps of each milestone run with a sub-task
worktree as the directory, and the comparison with the previous release is not a suite's to hold —
a suite drives one binary. The `R` half is the one worth the test: it is the cell where bytes with
no second copy sit under a git writer.

## Left open

Not pursued; each is for triage like any finding.

1. **No declared bound names this state.** The clause's scope says deliberately planted states are
   declared bounds written down with their reach; this run's opening declares no bound. The
   verdict above does not lean on a bound — step 2 of the clause reading holds on the planted
   state itself — but whether a path force-added under `.jigc/tasks/` belongs on that list, and
   with what reach, is the human's.
2. **Under the plant, the task whose area is tracked cannot land, and its refusal's route was not
   driven.** `jigc task finalize <T>` exits 3, `finalize.base-mismatch`, naming the planted paths
   as *the task's work* that the moved history overlaps; the route is *resolve the overlap … or
   discard the task with `jigc task discard <T> --force`*. The same on both binaries. Seen once
   more in the first exploration rig, candidate only: a task minted **before** the plant's commit
   and holding nothing of the planted area was refused the same way, the two planted paths that
   differed from `HEAD` named as its work.
3. **A landed boundary counts tracked paths under `.jigc/tasks/` as a sub-task's code** (`1 doc, 2
   code files`) and commits them under `modified .jigc/tasks/…`; the task door lists them as
   *left-out (unstaged/untracked — git add to include)*. Both binaries. One observation each.
4. **`jigc milestone discard --force` on the previous release removes a worktree holding a staged
   edit without naming it; the candidate names it** (cell 60's stderr, the one stream that
   differs). The edit is this verifier's plant.
5. **Not driven:** Linux, and a root caller; a planted path left in the index and never committed,
   at the finalize of the task that owns the area; an edit of git's copy left **unstaged** in a
   sub-task worktree when a landed boundary removes that worktree; the `squash: false` boundary;
   `jigc relocate`'s `git mv`; the finalize rollback's `git restore --source=HEAD --worktree`
   (its operand is a doc's committed home, which cells 49 and 50 show cannot be put inside an
   area — read, not reached).
6. **Git stages a tracked path under an ignored directory and exits 1 over it.** In the first
   exploration rig, `git add .jigc/tasks/<T>/docs/vision:vision.md` typed in a worktree — no `-f`,
   the path tracked there — printed *The following paths are ignored* and exited 1, and `git
   status` then showed the path staged. Git's behaviour (2.54.0), not jigc's; it is why the block
   stages with `git add -u`.

<!-- end of report -->
