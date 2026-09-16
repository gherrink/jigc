<!-- M51 per-axis review — axis 2 · reconciled · driven on the installed `jigc 1.0.0-rc.15` (commit 35195f56), 2026-09-16 -->

# M51 per-axis review — AXIS 2 · posture — RECONCILED

> **Reconciliation of the Opus driver table with the Codex source pass** (reconciler: a third
> session, driving `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15`, rigs as the driver used
> them). §§1–7 below are the driver's file **unchanged except for two inline demotions**, marked
> `[DEMOTED …]`. The ledger that reconciles the two passes is §R at the end, and the clap-leaf door
> list is §D. Every Codex claim was entered as a lead and **driven**; every driver defect was
> **re-driven** once by the reconciler before being kept.

# (driver file follows) — M51 per-axis review — AXIS 2 · posture — the OPUS DRIVER

**Binary:** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15` (asserted before any drive).
**Posture:** RELEASE. The `Route::mechanical` route fence is `#[cfg(debug_assertions)]`, so it does
not exist here; every refusal below is the shipped one.
**Fixtures:** `dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc`, two-step
eval, one fresh rig per posture cell (no `rm -rf` anywhere; every root is a `mktemp -d`). Each rig
was extended through the binary with a live task (`jigc start … --workflow decided-task`) and a
milestone with one sub-task (`milestone create` + `milestone add-task`) **before** the posture was
induced, so every door's argv is runnable and the posture is the only thing it can fault on.
**The fixed binary:** the four M51 audit fixes (`8a42fbbd` routes, `b5ccd818` orphan territory,
`0fc80bab` setup guard, `dc508994` LOWs) are all in `rc.15`; the setup guard
(F2) shows up in this axis as a real interaction and is recorded as a cell, not as noise.

---

## 1 · The door set, read from the code (counts stated)

| registry | file | count I read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs` | **47** leaves |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:1838` | **47** rows — a total classification, ⇔-fenced against the clap tree |
| ↳ `ActsOnBehalf::CommitsOnBehalf` | | **10** |
| ↳ `ActsOnBehalf::MovesOnBehalf` | | **2** |
| ↳ `ActsOnBehalf::Neither` | | **35** |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:130` | **10** rows over **9** leaves (`milestone finalize` × 2 commit models) |
| `PostureMember::ALL` | `crates/cli/src/repo.rs:132` | **3** |
| `InProgress::ALL` | `crates/cli/src/repo.rs:166` | **3** operations over **4** on-disk markers (`MERGE_HEAD` · `rebase-merge` · `rebase-apply` · `BISECT_LOG`) |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs` | **4** (axis 3's set; read only to confirm it is not this axis's) |

**Axis 2's door set = the 12 acting rows.** `COMMITTING_DOORS ⊆ commit-on-behalf` holds by
inspection: its 9 leaves are `task finalize`, `milestone finalize`, `rename`, `migrate-corpus`,
`milestone create`, `milestone add-task`, `milestone add-from-spec`, `milestone discard`,
`task discard` — all `CommitsOnBehalf`; the tenth commit-on-behalf leaf is `setup`, which carries no
rejection identity (`--no-verify`) and therefore no `COMMITTING_DOORS` row. That is the acceptance
design's 12, derived rather than taken from it.

The 12 acting doors, with the argv driven (straight from each `BEHALF_DOORS` row, work-unit slot
filled from the fixture):

| # | door | class | argv driven |
|---|---|---|---|
| 1 | `setup` | Commits (exempt: `HeadUnborn`) | `jigc setup` |
| 2 | `migrate-corpus` | Commits | `jigc migrate-corpus` |
| 3 | `rename` | Commits | `jigc rename adr:keeper --to Axis` |
| 4 | `task discard` | Commits | `jigc task discard record-the-axis-decision` |
| 5 | `task finalize` | Commits | `jigc task finalize record-the-axis-decision` |
| 6 | `milestone create` | Commits | `jigc milestone create "Axis milestone two"` |
| 7 | `milestone add-task` | Commits | `jigc milestone add-task axis-milestone "axis intent two"` |
| 8 | `milestone add-from-spec` | Commits | `jigc milestone add-from-spec axis-milestone spec:axis` |
| 9 | `milestone finalize` | Commits | `jigc milestone finalize axis-milestone` |
| 10 | `milestone discard` | Commits | `jigc milestone discard axis-milestone` |
| 11 | `relocate` | Moves | `jigc relocate vision --from docs/vision/` |
| 12 | `config set` | Moves | `jigc config set docs-root docs` |

## 2 · The cell set

The acceptance design's seven, plus the four cells driving revealed as discriminating:

`{clean control · HEAD detached · HEAD unborn · merge in progress · rebase in progress (rebase-merge)
· bisect in progress · dedicated worktree (typed) · GIT_DIR redirect (declared out)}`
**+ `rebase-apply` via `git am`** (the second marker of the one `InProgress::Rebase` member, and the
only one that leaves HEAD **attached**) **+ cherry-pick in progress** and **+ revert in progress**
(the two un-enumerated markers) **+ the no-override cross** (`--force` / `--carry-staged` /
`--dry-run`).

---

## 3 · The (door, cell) table — 12 acting doors

Every cell below ran the full 12-door sweep. Exit / code / route are identical across the doors
within a cell unless the table says otherwise, so the repro blocks are one per cell (§5).

### 3.1 · Clean control (`HEAD → refs/heads/main`, nothing in progress)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| all 10 commit-on-behalf | 0 / 1 / 3 per door's own business | **no `repo.*` code at any door** | per door | `grep 'repo\.'` on stderr → empty | matches contract |
| both movers | 0 / 1 | none | — | same | matches contract |

Driven outcomes, for the record: `setup` 0 · `migrate-corpus` 0 · `rename` 1 `store.not-found` ·
`task discard` 1 `task-discard.staged-prose` · `task finalize` 3 `finalize.empty-commit` ·
`milestone create` 0 · `milestone add-task` 0 · `milestone add-from-spec` 1 `store.not-found` ·
`milestone finalize` 3 `milestone.zero-contribution` · `milestone discard` 0 · `relocate` 1 (bare) ·
`config set` 0.

### 3.2 · HEAD detached (`git switch --detach HEAD`)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| `setup` | 1 | `repo.head-detached` | **Human** | `route: re-attach HEAD with \`git switch <branch>\`, then re-run this command` | matches contract |
| `migrate-corpus` | 1 | `repo.head-detached` | Human | same | matches contract |
| `rename` | 1 | `repo.head-detached` | Human | same | matches contract |
| `task discard` | 1 | `repo.head-detached` | Human | same | matches contract |
| `task finalize` | 1 | `repo.head-detached` | Human | same | matches contract |
| `milestone create` | 1 | `repo.head-detached` | Human | same | matches contract |
| `milestone add-task` | 1 | `repo.head-detached` | Human | same | matches contract |
| `milestone add-from-spec` | 1 | `repo.head-detached` | Human | same | matches contract |
| `milestone finalize` | 1 | `repo.head-detached` | Human | same | matches contract |
| `milestone discard` | 1 | `repo.head-detached` | Human | same | matches contract |
| `relocate` (mover) | 1 | none — **proceeds past the guard** | — | no `repo.` token on stderr | matches contract (movers refuse OIP only) |
| `config set` (mover) | 0 | none — **acts** | — | knob written | matches contract |

### 3.3 · HEAD unborn

Two sub-cells, because they answer differently and only one of them is the QUICKSTART on-ramp.

**(a) a genuinely unborn repo (`git init -q .`, no commits, no prior jigc):**

| door | exit | code | route | verdict |
|---|---|---|---|---|
| `setup` (empty repo) | **0** | none | — | **matches contract — the stated `Exempt(HeadUnborn)` row is real**: `install commit → e8af66e`, `git log` shows `chore(jigc): install jigc workspace config` |
| `setup` (one untracked `src.txt`) | **0** | none | — | matches contract; `src.txt` left untracked, untouched |

**(b) an already-installed repo driven unborn (`git symbolic-ref HEAD refs/heads/nothing-yet`):**

| door | exit | code | route kind | verdict |
|---|---|---|---|---|
| `setup` | 1 | **`setup.dirty-install-path`** — *not* `repo.head-unborn` | Human (`--force` named) | matches contract: the posture exemption holds (the family's code never fires), and M51 F2's pre-write guard fires on its own terms — relative to an unborn HEAD all 6 tracked install paths genuinely carry bytes in no commit. The F2 fix's own stated design (*"a loud false alarm `--force` clears, never a loss"*) |
| `migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone finalize` · `milestone discard` | 1 | `repo.head-unborn` | **Human** — `route: land the repository's first commit with \`git commit\`, then re-run this command` | matches contract |
| `relocate` (mover) | 1 | none — proceeds | — | matches contract |
| `config set` (mover) | 0 | none — acts | — | matches contract |

### 3.4 · Merge in progress (`MERGE_HEAD`, HEAD attached)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| **all 12** (10 commit + **both movers**) | 1 | `repo.operation-in-progress` | **Human** | `a merge is in progress — the repository is not in a committable state` · `route: conclude it, or abandon it with \`git merge --abort\`, then re-run this command` | matches contract — the one member the movers refuse too |

### 3.5 · Rebase in progress — `rebase-merge` (`git rebase` stopping on a conflict)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| all 10 commit-on-behalf | 1 | **`repo.head-detached`** | Human | `route: re-attach HEAD with \`git switch <branch>\`` | **DEFECT — D1** (the route dead-ends; see §4) |
| `relocate`, `config set` (movers) | 1 | `repo.operation-in-progress` | Human | `a rebase is in progress` · `route: … \`git rebase --abort\`` | matches contract |

### 3.6 · Rebase in progress — `rebase-apply` (`git am` stopping on a conflict, HEAD **attached**)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| **all 12** | 1 | `repo.operation-in-progress` | Human | `a rebase is in progress` · `route: conclude it, or abandon it with \`git rebase --abort\`` | **DEFECT — D2**: the operation is a `git am`, not a rebase; the named command exits 128 (`fatal: It looks like 'git am' is in progress. Cannot rebase.`) and leaves the state unchanged |

### 3.7 · Bisect in progress (`BISECT_LOG`; git detaches HEAD at the first `bisect good/bad`)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| all 10 commit-on-behalf | 1 | **`repo.head-detached`** | Human | `route: re-attach HEAD with \`git switch <branch>\`` | **DEFECT — D1** (misdiagnosis + a two-step recovery whose first step git warns about) |
| both movers | 1 | `repo.operation-in-progress` | Human | `a bisect is in progress` · `route: … \`git bisect reset\`` | matches contract |

### 3.8 · Cherry-pick in progress (`CHERRY_PICK_HEAD`, HEAD attached) — **un-enumerated marker**

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| `task finalize --carry-staged` | **0** | none | — | `finalized fc1a2e9 — feat(axis): land under a cherry pick` · `carried-over cp.txt` | **DEFECT — D3**: the user's cherry-pick is **concluded under jigc's own subject** and `CHERRY_PICK_HEAD` is gone; `git status` → *nothing to commit, working tree clean* |
| `task finalize` (no flag) | 3 | `finalize.carried-staged` | Mechanical-ish Human | `route: … or re-run the finalize with \`--carry-staged\` to declare the carry-over deliberate` | the route into the damage above — the gate is a *staging* gate and never names the cherry-pick |
| `milestone create` | 1 | **none** | **none** | `` `git commit` was rejected (no commit was made): fatal: cannot do a partial commit during a cherry-pick.`` … `Fix the hook's complaint` | **DEFECT — D3b**: git refused, no hook was involved; the survivable-hook-rejection frame names a cause that did not occur and the refusal carries no code and no route |

### 3.9 · Revert in progress (`REVERT_HEAD`, HEAD attached) — **un-enumerated marker**

| door | exit | code | verdict |
|---|---|---|---|
| `task finalize --carry-staged` | **0** | none | **DEFECT — D3**, second marker: `finalized b8a4901 — feat(axis): land under a revert`, `REVERT_HEAD` gone, tree clean |

> **[DEMOTED — no repro block in the driver file.]** D3's revert half was carried as a one-line
> *"same shape"* reference, not as a block: by this review's own rule that row was **not driven**.
> It is demoted here and **re-entered as reconciler-driven** with a full block — §R · D3-revert —
> where it reproduces exactly as stated (`finalized a872fc1 — feat(axis): land under a revert`,
> `REVERT_HEAD` GONE, tree clean, exit 0).

### 3.10 · Dedicated worktree (typed `DedicatedWorktree`, never sniffed)

| door | cell | exit | code | verdict |
|---|---|---|---|---|
| all 12, run **inside** `$REPO/.jigc/worktrees/axis-intent` | detached (git's own `--detach`) | per door's own business | **no `repo.head-detached` at any door** | matches contract — the `PostureSubject::adjudicates` exemption is real |
| all 12, same worktree | + a conflicting merge **inside it** | 1 | `repo.operation-in-progress`, `git merge --abort` | matches contract — exempt from `HeadDetached` **and from that member only** |
| all 12, in the **main** checkout | merge in progress inside the linked worktree | per door's own business | **no `repo.*`** | matches contract — the per-worktree `git_dir` resolution: `MERGE_HEAD` lands in `.git/worktrees/axis-intent/`, the main `.git` has none |
| `task finalize` (sub-task, in the worktree) then `milestone finalize` (main) | fan-out, detached worktree | 3 then **0** | `finalize.milestone-sub-task` then none | matches contract — *"a fan-out worktree … finalizes clean"*: `finalized dd68483 — Finalize milestone axis-milestone (1 sub-task)` |
| **negative control (a)** `task finalize`, `milestone create` in a **lookalike** at `.jigc/worktrees/not-a-subtask` (real linked worktree, unregistered id) | detached | 1 | `repo.head-detached` | matches contract — the **registry leg** discriminates, so a directory that merely looks provisioned is not exempt |
| **negative control (b)** same two doors in a user's own linked worktree elsewhere | detached | 1 | `repo.head-detached` | matches contract |
| **negative control (c)** `milestone create` in a user's own linked worktree, **attached** | clean | 0 | none | matches contract |

> **[DEMOTED — rows (b) and (c) carry no induction in the §5 repro-block index]** (every other row
> of §3.10 does). They are demoted here and **re-entered as reconciler-driven** with their
> inductions and observed output — §R · WT — where both reproduce exactly as stated.

### 3.11 · GIT_DIR redirect — the **declared-out** row, driven

| door | exit | code | verdict |
|---|---|---|---|
| `milestone create` with `GIT_DIR=<repoB>/.git` | **0** | none | **matches the declared bound exactly** (`repo.rs` module header, VERDICT bound 2): the record file is written into repo **A**'s worktree, the base is read from repo **B**'s HEAD (`33bde1f`), and the commit `0fcb7b1` lands in repo **B**. Repo A ends with `?? docs/milestone-records/redirected-milestone.md`; repo B ends with ` D docs/milestone-records/redirected-milestone.md`. Not a defect — a **stated row**, now with a driven repro. Reopening condition unchanged: a repository-**identity** check the fake-`.git` fixtures survive |

### 3.12 · The no-override cross — *the refusal carries no consent flag*

| door + flag | cell | exit | code | verdict |
|---|---|---|---|---|
| `setup --force` | merge | 1 | `repo.operation-in-progress` | matches contract |
| `task finalize --carry-staged` | merge | 1 | `repo.operation-in-progress` | matches contract — *"`--carry-staged` stops concluding a merge it never consented to conclude"*, verbatim, driven |
| `task discard --force` | merge | 1 | `repo.operation-in-progress` | matches contract |
| `milestone discard --force` | merge | 1 | `repo.operation-in-progress` | matches contract |
| `task finalize --dry-run` | merge | 1 | `repo.operation-in-progress` | matches contract (leaf-keyed) — **observation**, not a defect: the pinned *forecast arm* of `task finalize --format json` is unreachable in any breached posture, because the class is a property of the leaf and not of the flag |
| `migrate-corpus --dry-run` | merge | 1 | `repo.operation-in-progress` | same |
| `setup --force`, `task finalize --carry-staged`, `task finalize --dry-run` | detached | 1 | `repo.head-detached` | matches contract |

### 3.13 · The probe's subject — *it answers about the repository it was handed*

| cell | door | exit | code | verdict |
|---|---|---|---|---|
| a real **nested** repo, clean, inside a **detached** ancestor | `milestone create` | 0 | none | matches contract — no posture inherited from the ancestor |
| the same nested repo, itself **detached** | `milestone create` | 1 | `repo.head-detached` | matches contract |
| a **fake** `.git` *directory* (`mkdir -p nested/.git`) inside a detached real repo | `task finalize` / `milestone create` / `setup` | 1 / 0 / 0 | `finalize.no-task` / none / none | matches contract — `posture` finds no `HEAD` in the resolved git dir and asks nothing, so the fixture does not inherit the ancestor's posture. **Observation:** the *act* is not similarly bounded — `milestone create` there minted workbench state at exit 0 while printing **neither** the `record:` nor the `record commit:` line its clean-posture ack carries; no commit reached the ancestor (log count 7 → 7) and no record file was written. A synthetic shape, recorded rather than pursued |
| outside any git repository | `task finalize`, `milestone create --format json` | 1 | none | matches contract — the door's own `not inside a git repository (no \`.git\` found from …)` answer, **not** pre-empted by the guard (`refuse_on_posture` returns `None` when the walk-up does) |

### 3.14 · The pinned surfaces of a posture refusal

| surface | driven | verdict |
|---|---|---|
| `--format json`, commit door, detached | `{"error": "blocking · repo.head-detached — … \n  route: re-attach HEAD with \`git switch <branch>\`, then re-run this command"}` on **stderr**, exit 1, stdout empty | **matches contract** — `design/validation.md:661-663` declares all three `repo.*` codes **"neither — un-keyed"** in the target-form column, so the flattened single-key reject arm is the declared one, not the findings envelope |
| `--format json`, mover, merge | `{"error": "blocking · repo.operation-in-progress — …"}` | matches contract |
| invocation log | `{"argv":["task","finalize","record-the-axis-decision"],"exit_code":1,"finding_codes":["repo.head-detached"],…}` and `…["milestone","create","zz1"],"finding_codes":["repo.operation-in-progress"]` | matches contract — *"a posture refusal is as legible in the invocation log as it is on stderr"* |
| route count | exactly one `route:` line per refusal at every door driven | matches contract |

### 3.15 · The `Neither` class — 10 of 35 driven as controls

Under **detached** and under **merge**, none of these raised any `repo.*` code:
`start` (0) · `validate` (0) · `doc list` (0) · `uninstall` (1 `uninstall.staged-prose`) ·
`milestone provision` (0) · `milestone join` (0) · `ingest` (0) · `unmanage` (0) ·
`doc rename` (1, bare) · `migrate` (1, bare). **Verdict: matches contract** — a door that neither
commits nor moves a committed file is asked nothing.

---

## 4 · Defects

### D1 — a commit-on-behalf door never reports `repo.operation-in-progress` under a rebase or a bisect, and the code it reports instead routes at a command git refuses

**Contract violated.** `design/validation.md:663`: *"`repo.operation-in-progress` — a merge, rebase
or bisect the user started and has not concluded … Route: **`Human`, naming the command that
concludes *this* operation** — `git merge --abort` / `git rebase --abort` / `git bisect reset`,
never a menu of three"*, and `design/finalize.md:31`, which states the same three-predicate promise
for the commit-on-behalf class. `PostureBreach::finding`'s own doc-comment: *"a `Route::human`
naming **the git command that resolves the state**"*. Plus the route floor — a blocking finding
names a recovery that runs.

**What the binary does.** `posture()` returns breaches in `PostureMember::ALL` order and both
`refuse_on_posture` (`cli.rs:534`) and `SeamSubject::verify` (`repo.rs:~485`) take the **first**
match. Git detaches HEAD for the duration of both a stopped `rebase-merge` rebase and a bisect, so
`HeadDetached` always precedes and **masks** `OperationInProgress` at the 10 commit-on-behalf doors.
The two movers, which filter to `OperationInProgress`, print the correct diagnosis and the correct
command — so the right answer is in the binary, one class over, in the same run.

**Severity split.** *Rebase* is a hard dead end: the printed command exits 128 and changes nothing.
*Bisect* is a misdiagnosis plus a two-step recovery whose first step git warns about
(`warning: you are switching branch while bisecting`).

**Repro — rebase (release binary):**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC start "record the axis decision" --workflow decided-task >/dev/null
git checkout -q -b side; printf 'side\n' > c.txt; git add c.txt; git commit -q -m side
git checkout -q main;    printf 'main\n' > c.txt; git add c.txt; git commit -q -m main
git checkout -q side
GIT_EDITOR=true git rebase main            # exit 1 — stops on the conflict
ls -d .git/rebase-merge                    # present
git symbolic-ref -q HEAD; echo $?          # 1 — git detached HEAD for the rebase

$JIGC task finalize record-the-axis-decision
# exit 1
#   blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no
#     branch, and the next checkout would leave it unreachable
#     route: re-attach HEAD with `git switch <branch>`, then re-run this command

# follow that route VERBATIM:
git switch main
#   fatal: cannot switch branch while rebasing
#   Consider "git rebase --quit" or "git worktree add".
#   exit 128
ls -d .git/rebase-merge                    # still present — state unchanged
$JIGC task finalize record-the-axis-decision   # byte-identical refusal, exit 1

# the command the MOVER printed in the same state, run verbatim:
$JIGC relocate vision --from docs/vision/
#   blocking · repo.operation-in-progress — a rebase is in progress — …
#     route: conclude it, or abandon it with `git rebase --abort`, then re-run this command
git rebase --abort                         # exit 0 — this is the command that resolves it
```

**Repro — bisect (same rig, `git bisect start; git bisect bad HEAD; git bisect good <root>`):**

```
$JIGC milestone create zz
#   blocking · repo.head-detached — …  route: re-attach HEAD with `git switch <branch>` …   exit 1
git switch main
#   warning: you are switching branch while bisecting        exit 0
ls .git/BISECT_LOG                          # STILL present
$JIGC milestone create zz
#   blocking · repo.operation-in-progress — a bisect is in progress — …
#     route: conclude it, or abandon it with `git bisect reset` …                           exit 1
git bisect reset                            # exit 0
$JIGC milestone create zz                   # exit 0 — minted
```

**Why no suite caught it.** `crates/cli/tests/posture_door_axis.rs:165 expected_code()`
**re-implements production's `.find(…)` ordering in the test** and `git_command_for("repo.head-detached", _)`
returns `"git switch <branch>"` — so under `State::Rebase` the suite *expects* the masked code and
*expects* the dead-end route. The unit suite `repo_posture.rs` calls `posture()` directly, gets both
breaches, and reaches past the masking with `only(breaches, OperationInProgress)`. The cell is green
in both places and red at the door.

### D2 — `rebase-apply` is `git am`'s marker too, and jigc names a `rebase` and routes at `git rebase --abort`, which git refuses

**Contract violated.** Same two rows as D1, plus law 1: `InProgress::markers()`'s own doc-comment
already says *"`rebase-apply/` (the am backend)"*, so the source knows what it is looking at and the
message and route still say *rebase*. This cell is the one where HEAD stays **attached**, so all 12
doors reach it — the diagnosis and the dead end are unmasked and universal.

**Repro (release binary), all 12 acting doors:**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC start "record the axis decision" --workflow decided-task >/dev/null
BASE=$(git rev-parse HEAD)
git checkout -q -B pa "$BASE"; printf 'PATCH\n' > am.txt; git add am.txt; git commit -q -m "patch commit"
git format-patch -1 --stdout > "$RIG/p.patch"
git checkout -q main; printf 'CONFLICT\n' > am.txt; git add am.txt; git commit -q -m conflicting
git am "$RIG/p.patch"                       # exit 128 — stops on the conflict
ls -d .git/rebase-apply                     # present
git symbolic-ref -q HEAD                    # refs/heads/main — ATTACHED

$JIGC task finalize record-the-axis-decision
# exit 1
#   blocking · repo.operation-in-progress — a rebase is in progress — the repository is not in a
#     committable state
#     route: conclude it, or abandon it with `git rebase --abort`, then re-run this command
# (identical at setup · migrate-corpus · rename · task discard · milestone create · add-task ·
#  add-from-spec · milestone finalize · milestone discard · relocate · config set)

git rebase --abort
#   fatal: It looks like 'git am' is in progress. Cannot rebase.     exit 128
ls -d .git/rebase-apply                     # still present — state unchanged

git am --abort                              # exit 0 — the command that actually resolves it
$JIGC milestone create zz9                  # exit 0
```

### D3 — an un-concluded **cherry-pick** or **revert** is not a member of the family, and `jigc task finalize` concludes it under jigc's own subject at exit 0

**Contract violated.** The member's own warrant, `ActsOnBehalf::CommitsOnBehalf`'s doc-comment:
*"a commit made under a live `MERGE_HEAD` **concludes a merge the user started, under jigc's own
subject**"*; `design/finalize.md`:31/35 and `design/validation.md`:663 state the damage the family
exists to prevent in exactly those words. The family's markers are `{MERGE_HEAD, rebase-merge,
rebase-apply, BISECT_LOG}` — 4 of the 6 markers git actually writes for an un-concluded operation.
`CHERRY_PICK_HEAD` and `REVERT_HEAD` are absent, so the damage the member names happens at exit 0
with no finding of any kind.

The route into it is jigc's own: the carryover gate refuses first with
`finalize.carried-staged` and routes at `--carry-staged` — a *staging* consent, which the user gives
about their own staged file and which then also silently concludes their cherry-pick.

**Repro — cherry-pick (release binary):**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
BASE=$(git rev-parse HEAD)
git checkout -q -B cpa "$BASE"; printf 'ALPHA\n' > cp.txt; git add cp.txt; git commit -q -m alpha
SHA=$(git rev-parse HEAD)
git checkout -q main;           printf 'BETA\n'  > cp.txt; git add cp.txt; git commit -q -m beta
git cherry-pick "$SHA"                     # exit 1 — conflict
printf 'RESOLVED\n' > cp.txt; git add cp.txt    # the user resolves it, as git instructs
ls .git/CHERRY_PICK_HEAD                   # present; HEAD = refs/heads/main (attached)

$JIGC start "land under a cherry pick" --workflow dev-task >/dev/null
T=land-under-a-cherry-pick
printf 'code\n' > feature.txt; git add feature.txt
$JIGC doc set-field commit:$T#type  --value feat --task $T
$JIGC doc set-field commit:$T#scope --value axis --task $T
$JIGC doc set-slot  commit:$T#summary --from-file - --task $T <<'EOF'
land under a cherry pick
EOF
$JIGC doc set-slot  commit:$T#body    --from-file - --task $T <<'EOF'
posture axis drive
EOF

$JIGC task finalize $T
# exit 3
#   blocking · finalize.carried-staged — `cp.txt` was already staged before this task existed …
#     route: unstage it (`git restore --staged -- cp.txt`) if it is not this task's work, or
#            re-run the finalize with `--carry-staged` to declare the carry-over deliberate

$JIGC task finalize $T --carry-staged
# exit 0
#   finalized fc1a2e9 — feat(axis): land under a cherry pick
#     carried-over cp.txt
#     added feature.txt
#     3 files committed
ls .git/CHERRY_PICK_HEAD                   # No such file or directory — the cherry-pick is GONE
git status                                 # On branch main / nothing to commit, working tree clean
```

**Repro — revert (same shape, `git revert --no-edit <sha>` → `REVERT_HEAD`):** `task finalize
--carry-staged` → exit 0, `finalized b8a4901 — feat(axis): land under a revert`, `REVERT_HEAD` gone,
tree clean.

**Positive control, same rig shape:** the identical sequence under an open **merge** is refused —
`task finalize --carry-staged` → exit 1 `repo.operation-in-progress`. So the class is real and these
two markers are its un-swept axis members.

### D3b — git's own refusal during a cherry-pick is dressed as a pre-commit hook rejection, with no code and no route

At the four record-only milestone doors git refuses the pathspec-limited commit itself
(`fatal: cannot do a partial commit during a cherry-pick.`). jigc renders it through the
survivable-**hook**-rejection frame:

```
$JIGC milestone create cpm2
# exit 1
#   `git commit` was rejected (no commit was made):
#   fatal: cannot do a partial commit during a cherry-pick.
#
#   nothing was committed — the record write and the milestone workbench were both rolled back, so
#   nothing of milestone:cpm2 survives. Fix the hook's complaint, then re-run `jigc milestone create cpm2`.
```

No hook exists in the rig. *"Fix the hook's complaint"* names a mechanism that did not act (law 1),
the refusal carries **no finding code** and **no `route:` line**, and the state-truth clause
(*"nothing … survives"*) is true while the sentence that follows sends the user to look at a hook.
The rollback itself is correct and was verified: repo log unmoved, no record file on disk.

---

## 5 · Repro-block index (setup · argv · observed)

Every cell above was driven from a fresh `dev/jigc-rig committed-singletons --binary
/Users/maurice/.local/bin/jigc` rig extended by `$D/build-fixture.sh` (`jigc start … --workflow
decided-task` + `jigc milestone create "Axis milestone"` + `jigc milestone add-task axis-milestone
"axis intent"`), then driven into the cell's git state, then swept with the 12 argvs of §1. The
per-cell state inductions, each verified on disk before the sweep:

| cell | induction | verified by |
|---|---|---|
| clean | none | `git symbolic-ref HEAD` → `refs/heads/main` |
| detached | `git switch --detach HEAD` | `git symbolic-ref -q HEAD` exit 1 |
| unborn (a) | `mkdir a; git init -q .` | `git log` → fatal, 0 commits |
| unborn (b) | `git symbolic-ref HEAD refs/heads/nothing-yet` | `git rev-parse --verify HEAD` exit 128 |
| merge | two branches conflicting on `k.txt`, `git merge wa` | `.git/MERGE_HEAD` present |
| rebase (`rebase-merge`) | `GIT_EDITOR=true git rebase main` on a conflict | `.git/rebase-merge` present |
| rebase (`rebase-apply`) | `git format-patch` + conflicting `git am` | `.git/rebase-apply` present, HEAD attached |
| bisect | `git bisect start; bad HEAD; good <root>` | `.git/BISECT_LOG` present |
| cherry-pick | conflicting `git cherry-pick`, conflict resolved + staged | `.git/CHERRY_PICK_HEAD` present |
| revert | conflicting `git revert --no-edit`, resolved + staged | `.git/REVERT_HEAD` present |
| dedicated worktree | `jigc milestone provision axis-milestone` | `.git` is a **file**; `git symbolic-ref -q HEAD` exit 1 inside it |
| worktree lookalike | `git worktree add --detach .jigc/worktrees/not-a-subtask` | unregistered id |
| GIT_DIR redirect | second `git init` repo B; `GIT_DIR=<B>/.git jigc milestone create` | repo A `??`, repo B ` D`, commit in B |
| nested real repo | `git init inner` inside a detached ancestor | inner `refs/heads/main` |
| fake `.git` | `mkdir -p nested/.git` inside a detached repo | `.git` is an empty dir with no `HEAD` |
| no repo | `mktemp -d`, no git | `not inside a git repository (no \`.git\` found …)` |
| invocation log | `jigc config set invocation-log true` | `.jigc/logs/invocations.jsonl` |

---

## 6 · What I did NOT drive, and why

1. **25 of the 35 `Neither` leaves × every posture cell.** I drove 10 as controls (§3.15). The
   remaining 25 are not driven. Reason: `refuse_on_posture` returns `None` for the whole class at a
   single shared seam keyed on `Command::leaf()`, and the class's membership is ⇔-fenced against the
   clap tree — so driving the 26th adds a row and no information the 10 do not already carry. Stated
   as un-driven rather than presented as covered.
2. **`SeamSubject::verify`'s identity-drift and ref-drift legs.** Not driven. They fire only when
   the checkout changes **between** a subject's construction and its `verify`, both of which happen
   inside one process within milliseconds; no CLI sequence opens that window from outside, and a
   `pre-commit` hook runs *after* `verify` has already passed. The three posture members the seam
   re-asks are driven at all 12 doors (§3.2–3.7) and at the two seams that build a subject from a
   user-visible argument (`relocate` → `SeamAct::Move`, `setup` → `live_exempt`).
3. **`milestone finalize (squash: true)` vs `(squash: false)` as two distinct `COMMITTING_DOORS`
   rows.** Driven once, at the `milestone finalize` leaf. The posture guard is leaf-keyed
   (`Command::leaf()`), so the two commit models are indistinguishable at the door; the distinction
   is axis 4's.
4. **The GIT_DIR redirect at the other 11 acting doors.** Driven at `milestone create` only. The
   bound is a property of the probe (`repo.rs` header) and of the ambient environment, not of a
   door, and it is declared out with a reopening condition rather than owed a per-door sweep.
5. **`.git/sequencer` — a *multi*-commit cherry-pick or revert sequence.** Not driven. The
   single-commit markers `CHERRY_PICK_HEAD` / `REVERT_HEAD` are, and D3's class statement is about
   the marker set, not about the sequencer directory.
6. **The three `only-5` leaves** (`task list`, `config get`, `config list`) and every other
   `Neither` leaf are outside this axis's door set by the acceptance design's own construction.

---

## 7 · What this adds over flow-52 arm 2

Arm 2 (`flow52_acceptance.rs:676-1010`, and the per-row suite `posture_door_axis.rs` beside it)
iterates the acting rows of `BEHALF_DOORS` × `PostureMember::ALL` in fresh repos and asserts the
**partition** — commit doors refuse the full family, movers refuse operation-in-progress only,
`setup` is exempt from unborn and from nothing else, a linked worktree is typed rather than sniffed,
nothing commits and HEAD does not move. All of that reproduced here, on the release binary, and all
of it holds.

Four things this review adds that no arm reaches:

1. **It drives the cell set by the *marker*, not by the *member*.** `InProgress` is three members
   over **four** on-disk markers, and both suites induce only three of them (`MERGE_HEAD`,
   `rebase-merge`, `BISECT_LOG`). Driving `rebase-apply` through `git am` — the one operation cell
   that leaves HEAD **attached**, and therefore the only one where `OperationInProgress` reaches a
   commit door unmasked — is what surfaced **D2**.
2. **It runs the printed route as bytes.** Arm 2 asserts a route *names* a git command
   (`git_command_for`); it never executes it. Running them verbatim is what turned the rebase cell
   from "the suite's sharpest cell, green" into **D1**: the route exits 128 and changes nothing. The
   suite could not see it because `expected_code()` re-implements production's own `.find(…)`
   ordering, so the test's expectation and the code's behaviour are the same statement — the shape
   M45 named (*a statement is not a fence when it equals the constant it is fencing*), here between
   a test and the function it tests.
3. **It asks what the *class* is, not what the *registry* is.** Both suites take
   `InProgress::ALL` as the case-set and are exhaustive over it by construction. The class the
   design states is *"a git operation the user started and has not concluded"*, and git's marker set
   for that class has six members. Driving the two the registry omits produced **D3** — an exit-0
   commit that concludes a user's cherry-pick and a user's revert under jigc's own subject, which is
   the damage the merge member's own doc-comment exists to name, and which no arm's case-set could
   have reached.
4. **It drives the declared-out row instead of citing it.** The `GIT_DIR` redirect is carried in
   three documents as a stated bound; §3.11 is the first driven repro of what it actually costs
   (record in repo A, base and commit in repo B, both repositories left inconsistent, exit 0). That
   does not change its disposition — it makes the bound checkable.

Minor additions in the same spirit: the cross with `--force` / `--carry-staged` / `--dry-run` at the
flags that exist (§3.12), the un-driven-in-arm-2 sub-cell of `setup` × unborn **over an
already-installed repo** where M51 F2's new guard answers instead of the family (§3.3b), the
per-worktree `git_dir` isolation in both directions (§3.10), and the invocation-log read-back of a
posture refusal's code (§3.14).

---

# §R · Reconciliation ledger

**Binary for every block below:** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15`, asserted in
each rig before the drive. Fixtures: `dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc`,
two-step eval, one fresh rig per block (every root a `mktemp -d`; no teardown, no `rm -rf`).
Probe scripts as run: `scratchpad/axis-review/probe/{c1,c1-control,c1c,d1,d2,d3,d3rev,wt,surf,misc}.sh`.

## R.1 · Codex claims

### lead(codex, 1) — `jigc relocate` runs `git rm --cached` on a committed destination squatter with no posture re-probe immediately before that index mutation → **CONFIRMED (origin codex)**

Codex's evidence (`relocate.rs:241-258` displacement before the guarded move; `relocate.rs:280-323`
the `git rm --cached`; `relocate.rs:65-79` the only re-probe, inside `move_doc` immediately before
`git mv`) is correct, and the consequence drives. Codex proposed a two-process synchronisation; the
window was opened deterministically instead with a `git` **PATH shim** that starts a *real*
`git merge --no-ff --no-commit` immediately after jigc's enumeration call (`ls-files -z`) and
returns — i.e. **after** the door's posture probe and **before** `displace_foreign_squatter` is
entered for any doc, which is where a re-probe would have had to sit. The binary is unmodified; the
state induced is a genuine merge-in-progress; the shim is a stand-in for the concurrent process the
claim posits, and the git-call log below shows the ordering.

**Repro — door 1, `jigc relocate` (release binary):**

```
rig=$(dev/jigc-rig fresh --pack-from-dev --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
#   --pack-from-dev DROPS the dev pack's freeze manifest, so `adr` is freeze-exempt and
#   `jigc relocate` does not refuse it to `migrate-corpus`.
mkdir -p docs/old docs/decisions
for i in 1 2 3; do printf '# Doc %s\n' "$i" > "docs/old/a00$i.md"; done
printf '# Squat source\n'            > docs/old/zsquat.md
printf 'FOREIGN SQUATTER BYTES\n'    > docs/decisions/zsquat.md   # committed-but-unmanaged
git add docs/old docs/decisions/zsquat.md && git commit -q -m "stranded docs + a squatter"
git checkout -q -b sidecar; printf 'side\n' > sidecar.txt; git add sidecar.txt; git commit -q -m sidecar
git checkout -q main

# the concurrent process, made deterministic: a `git` shim that fires ONE real merge after the
# first `ls-files` and then passes everything through to the real git.
cat > "$RIG/shim/git" <<'EOF'
#!/bin/sh
REAL=/usr/bin/git; LOG="$RIG/gitcalls.log"; MARK="$RIG/fired"
printf '%s\n' "$*" >> "$LOG"; "$REAL" "$@"; st=$?
if [ ! -e "$MARK" ]; then for a in "$@"; do [ "$a" = ls-files ] && { : > "$MARK"
    "$REAL" -C "$REPO" merge --no-ff --no-commit sidecar >/dev/null 2>&1; break; }; done; fi
exit $st
EOF
chmod +x "$RIG/shim/git"; export PATH="$RIG/shim:$PATH"

jigc relocate adr --from docs/old/
# exit 0
#   freeze-exempt relocation: 0 moved, 0 displaced, 4 blocked
#     blocked   docs/old/a001.md
#       blocking · repo.operation-in-progress — a merge is in progress — the repository is not in
#         a committable state
#       route: conclude it, or abandon it with `git merge --abort`, then re-run this command
#     … a002, a003, zsquat identically

git status --short
#   D  docs/decisions/zsquat.md        <-- the index WAS mutated, under the live merge
#   A  sidecar.txt
ls .jigc/displaced                     #   zsquat.md   <-- the user's file WAS moved
test -e docs/decisions/zsquat.md; echo $?   # 1 — gone from the destination

cat "$RIG/gitcalls.log"
#   symbolic-ref -q HEAD                     <-- the DOOR's posture probe (passes: no MERGE_HEAD)
#   rev-parse --verify -q HEAD
#   ls-files -z
#   SHIM fired after: ls-files -z -> MERGE_HEAD present     <-- the window opens here
#   …
#   rm --cached --ignore-unmatch --quiet -- docs/decisions/zsquat.md   <-- NO probe before this
#   symbolic-ref -q HEAD / rev-parse --verify -q HEAD                  <-- move_doc's probe, after
```

Three things the block establishes beyond the claim as written:

1. **The report denies it happened.** `0 moved, 0 displaced, 4 blocked` — `relocate_one` returns the
   displaced pair only on the success path, so when `move_doc` refuses the pair is dropped and the
   displacement is printed nowhere.
2. **Exit 0.** `blocked` is surfaced, never fatal, so the run that mutated the index and moved a
   user's file under a merge exits **0**.
3. **The same window is reachable at a second mover door.** `config set placement-root` consumes the
   same `relocate_stranded` (`config.rs:1110`), and drives identically:

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
mkdir -p docs2; printf 'FOREIGN SQUATTER AT THE NEW PLACEMENT HOME\n' > docs2/roadmap.md
git add docs2/roadmap.md && git commit -q -m squatter          # roadmap is the re-rooted placement doc
…same sidecar branch + same shim…
jigc config set placement-root docs2
# exit 0
#   relocating the committed doc(s) stranded by the `placement-root` re-point to `docs2` …
#     - docs/decisions-log.md: could not relocate (blocking · repo.operation-in-progress — a merge
#       is in progress …) — move it by hand
#     - docs/roadmap.md:      could not relocate (… same …) — move it by hand
#   config: set `placement-root` = `docs2` — written to `.jigc/config/`, uncommitted …
git status --short   #   D  docs2/roadmap.md      <-- index mutated under the merge
ls .jigc/displaced   #   roadmap.md              <-- squatter moved, and NOT named in the output
```

**Positive control (the door does hold when the merge precedes it):**

```
…same fixture, NO shim…
git merge --no-ff --no-commit sidecar        # MERGE_HEAD present BEFORE the door
jigc relocate adr --from docs/old/
# exit 1
#   blocking · repo.operation-in-progress — a merge is in progress — …
#     route: conclude it, or abandon it with `git merge --abort`, then re-run this command
git status --short   #   A  sidecar.txt        — index untouched
ls .jigc/displaced   #   No such file or directory
test -e docs/decisions/zsquat.md; echo $?     # 0 — squatter untouched
```

**Bound on the finding, stated:** the window is a **race** — no single-process argv sequence opens
it, because nothing jigc does starts a git operation mid-run. Its reachability is therefore
"a concurrent process (a second shell, a hook in another checkout, a watcher) starts a merge/rebase/
bisect between the door and the displacement", which is exactly the condition `move_doc`'s own
doc-comment already says the re-probe exists for (*"a hook, a concurrent process or an earlier phase
of this same run could have started one in between"*) — the displacement is the sibling that did not
get the treatment the comment describes.

### lead(codex, "consistent findings") — the source pass's clean-read list → recorded, 5 of 13 re-driven, none contradicted

Codex's non-claim list is not a set of findings, but the rule's *silent-omission-vs-clean-read*
distinction means each is worth a disposition:

| source statement | disposition |
|---|---|
| `BEHALF_DOORS` total over the clap leaf tree, ⇔-fenced (`cli.rs:4533-4557`); 10 committing + 2 movers | **consistent** with the driver's own count read (47 leaves / 47 rows / 10 / 2 / 35) |
| all 10 `COMMITTING_DOORS` rows resolve to commit-on-behalf; `setup` the extra no-hook committing door | **consistent** with §1 |
| the hook-capable `git commit` re-probes immediately before execution (`task.rs:5414-5423`) | **driven, consistent** — §3.4/3.6 and R.2 · D2 (every commit door refuses under a live marker) |
| `setup`'s `--no-verify` commit re-probes and carries only the unborn exemption (`setup.rs:2398-2428`) | **driven, consistent** — §3.3(a) exempt at unborn; §3.4/3.6/R.2 refused under merge and under `rebase-apply`; §3.12 `setup --force` refused under merge |
| the fan-out `git merge --ff-only` re-probes (`task.rs:5614-5619`) | not driven by either pass at the seam; the door above it (`milestone finalize`) is driven in §3.2–3.7. **OPEN (not a claim)** |
| the sole production `git mv` re-probes (`relocate.rs:65-79`) | **driven, consistent** — and it is the *only* re-probe on that path, which is claim 1 |
| no production `git switch`/`checkout` act | consistent with the driver's read; nothing in either pass contradicts it |
| no `neither` door reaches a commit or a committed-file move | **driven, consistent** — §3.15, 10 of 35 raised no `repo.*` code under detached and under merge |
| `DedicatedWorktree` cannot be forged (private fields + `add`, `task.rs:5753-5783`; dedicated seam needs a live handle, `repo.rs:432-448`) | **driven, consistent at the behaviour** — §3.10 negative controls (a)/(b), re-driven in R.2 · WT: a real linked worktree at `.jigc/worktrees/<unregistered-id>` is **not** exempt |
| the unborn exemption does not leak (`live_exempt` has one caller) | **driven, consistent** — §3.3(b) and R.2 · MISC: every other door answers `repo.head-unborn` |
| `GIT_DIR` redirection declared out (`repo.rs:38-55`, `463-466`) | **driven, consistent** — §3.11, re-driven in R.2 · MISC |

**None of the 13 contradicts any driver defect** — the source pass is *silent* on the route-ordering
mask (D1), on the `rebase-apply`/`git am` naming (D2) and on the `CHERRY_PICK_HEAD`/`REVERT_HEAD`
markers (D3/D3b). Under the rule, those stay findings; each was re-driven below.

## R.2 · Driver defects, re-driven

### D1 — under a stopped rebase and under a bisect, a commit-on-behalf door reports `repo.head-detached` and routes at a command git refuses → **CONFIRMED (origin driver)**; source pass **silent**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
jigc start "record the axis decision" --workflow decided-task >/dev/null
git checkout -q -b side; printf 'side\n' > c.txt; git add c.txt; git commit -q -m side
git checkout -q main;    printf 'main\n' > c.txt; git add c.txt; git commit -q -m main
git checkout -q side; GIT_EDITOR=true git rebase main      # exit 1 — stops on the conflict
# rebase-merge present? yes      HEAD attached? exit=1 (detached)

jigc task finalize record-the-axis-decision
#   blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no
#     branch, and the next checkout would leave it unreachable
#     route: re-attach HEAD with `git switch <branch>`, then re-run this command      exit 1
git switch main
#   fatal: cannot switch branch while rebasing
#   Consider "git rebase --quit" or "git worktree add."                               exit 128
# rebase-merge still present? yes  — the route changed nothing

jigc relocate vision --from docs/vision/       # the MOVER, same state, same run
#   blocking · repo.operation-in-progress — a rebase is in progress — …
#     route: conclude it, or abandon it with `git rebase --abort`, then re-run this command
git rebase --abort                                                                   # exit 0
```

Bisect half, same rig:

```
git bisect start; git bisect bad HEAD; git bisect good $(git rev-list --max-parents=0 HEAD | tail -1)
# BISECT_LOG present? yes   HEAD detached
jigc milestone create zz
#   blocking · repo.head-detached — …  route: re-attach HEAD with `git switch <branch>` …   exit 1
git switch main
#   warning: you are switching branch while bisecting                                       exit 0
# BISECT_LOG STILL present
jigc milestone create zz
#   blocking · repo.operation-in-progress — a bisect is in progress — …
#     route: conclude it, or abandon it with `git bisect reset` …                           exit 1
jigc config set docs-root docs      # the mover, same state: bisect named correctly first time
#   blocking · repo.operation-in-progress — a bisect is in progress — …                     exit 1
git bisect reset                                                                            # exit 0
```

Confirmed as the driver states it, including the severity split (rebase = dead end at exit 128;
bisect = misdiagnosis + a two-step recovery git warns about).

### D2 — `rebase-apply` is `git am`'s marker too; jigc names *a rebase* and routes at `git rebase --abort`, which git refuses → **CONFIRMED (origin driver)**; source pass **silent**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
jigc start "record the axis decision" --workflow decided-task >/dev/null
BASE=$(git rev-parse HEAD)
git checkout -q -B pa "$BASE"; printf 'PATCH\n' > am.txt; git add am.txt; git commit -q -m "patch commit"
git format-patch -1 --stdout > "$RIG/p.patch"
git checkout -q main; printf 'CONFLICT\n' > am.txt; git add am.txt; git commit -q -m conflicting
git am "$RIG/p.patch"        # exit 128 — stops on the conflict
# rebase-apply present? yes ; rebase-merge? no ; HEAD = refs/heads/main  (ATTACHED)

# all seven doors driven here answer identically:
#   task finalize · milestone create · rename · migrate-corpus · setup · relocate · config set
#   blocking · repo.operation-in-progress — a rebase is in progress — the repository is not in a
#     committable state
#     route: conclude it, or abandon it with `git rebase --abort`, then re-run this command

git rebase --abort
#   fatal: It looks like 'git am' is in progress. Cannot rebase.                      exit 128
# rebase-apply still present? yes
git am --abort                                                                      # exit 0
jigc milestone create zz9                                                           # exit 0
```

Confirmed, with the driver's load-bearing structural point re-observed: this is the one operation
cell that leaves HEAD **attached**, so both movers reach it too and the dead-end route is universal
rather than masked.

### D3 — an un-concluded **cherry-pick** or **revert** is not a member of the family, and `jigc task finalize --carry-staged` concludes it under jigc's own subject at exit 0 → **CONFIRMED (origin driver)**; source pass **silent**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
BASE=$(git rev-parse HEAD)
git checkout -q -B cpa "$BASE"; printf 'ALPHA\n' > cp.txt; git add cp.txt; git commit -q -m alpha
SHA=$(git rev-parse HEAD)
git checkout -q main;           printf 'BETA\n'  > cp.txt; git add cp.txt; git commit -q -m beta
git cherry-pick "$SHA"                       # exit 1 — conflict
printf 'RESOLVED\n' > cp.txt; git add cp.txt # the user resolves it, as git instructs
# CHERRY_PICK_HEAD present? yes ; HEAD=refs/heads/main

jigc start "land under a cherry pick" --workflow dev-task >/dev/null
T=land-under-a-cherry-pick; printf 'code\n' > feature.txt; git add feature.txt
jigc doc set-field commit:$T#type  --value feat --task $T
jigc doc set-field commit:$T#scope --value axis --task $T
jigc doc set-slot  commit:$T#summary --from-file - --task $T <<<'land under a cherry pick'
jigc doc set-slot  commit:$T#body    --from-file - --task $T <<<'posture axis drive'

jigc task finalize $T
#   blocking · finalize.carried-staged — `cp.txt` was already staged before this task existed …
#     route: unstage it (`git restore --staged -- cp.txt`) …, or re-run the finalize with
#            `--carry-staged` to declare the carry-over deliberate                    exit 3
jigc task finalize $T --carry-staged
#   finalize — about to commit the index; carrying over … carried-over cp.txt
#   no findings — the task validates clean
#   finalized 9ddeed3 — feat(axis): land under a cherry pick
#     carried-over cp.txt / added feature.txt / 2 files committed                     exit 0
# CHERRY_PICK_HEAD after? GONE
git status   #   On branch main / nothing to commit, working tree clean
```

**D3-revert (the demoted §3.9 row, now driven in full):**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf 'V1\n' > rv.txt; git add rv.txt; git commit -q -m v1; SHA=$(git rev-parse HEAD)
printf 'V2\n' > rv.txt; git add rv.txt; git commit -q -m v2
git revert --no-edit "$SHA"                  # exit 1 — conflict
printf 'RESOLVED\n' > rv.txt; git add rv.txt
# REVERT_HEAD present? yes ; HEAD=refs/heads/main
…same commit-doc fill under task `land-under-a-revert`…
jigc task finalize land-under-a-revert --carry-staged
#   finalized a872fc1 — feat(axis): land under a revert
#     added feature.txt / carried-over rv.txt / 2 files committed                     exit 0
# REVERT_HEAD after? GONE ;  git status → nothing to commit, working tree clean
```

**Positive control, driven in the same rig shape:** the identical sequence under an open **merge** is
refused — `jigc task finalize land-under-a-merge --carry-staged` →
`blocking · repo.operation-in-progress — a merge is in progress …`, route `git merge --abort`. So
the class is real and these two markers are its un-swept axis members, exactly as the driver states.

### D3b — git's own refusal during a cherry-pick is dressed as a pre-commit hook rejection, with no code and no route → **CONFIRMED (origin driver), with one prose correction**; source pass **silent**

```
…the D3 cherry-pick fixture, before any task…
jigc milestone create cpm2
# exit 1
#   `git commit` was rejected (no commit was made):
#   fatal: cannot do a partial commit during a cherry-pick.
#
#   nothing was committed — the record write and the milestone workbench were both rolled back, so
#   nothing of milestone:cpm2 survives. Fix the hook's complaint, then re-run
#   `jigc milestone create cpm2`.
```

Confirmed: no finding code, no `route:` line, and *"Fix the hook's complaint"* names a mechanism that
did not produce this error (git refuses the pathspec-limited commit itself, before any hook runs).

**Correction to the driver's prose, on driven evidence:** the driver writes *"No hook exists in the
rig."* — that is **false**. `jigc setup` installs `.git/hooks/pre-commit`, and the rig has it
(`test -e .git/hooks/pre-commit` → PRESENT). The defect is unaffected: a hook exists and **did not
act**, which is the same law-1 violation by a different route, and arguably a sharper one — the user
sent to *"fix the hook's complaint"* will find a hook, read it, and find nothing wrong with it.

## R.3 · Driver rows re-driven and held (no status change)

| driver row | re-driven observation |
|---|---|
| §3.3(b) `setup` × unborn-over-an-installed-repo | `blocking · setup.dirty-install-path — 6 path(s) in the install footprint carried changes that were in no commit before this run …` route names `--force`; exit 1. Every **other** commit door in the same state: `repo.head-unborn`, route `land the repository's first commit with \`git commit\``; `config set docs-root` (mover) exit **0**. Holds |
| §3.10 worktree, typed exemption | inside `.jigc/worktrees/axis-intent` (`.git` a FILE, HEAD DETACHED): `milestone create` raises **no `repo.*`**; `task finalize <sub-task>` blocks on `finalize.milestone-sub-task`, not on posture. Holds |
| §3.10 **[re-entered after demotion]** negative control (b) — a user's own linked worktree elsewhere, detached (`git worktree add --detach "$RIG/userwt"`) | `milestone create` and `task finalize` both → `blocking · repo.head-detached`. Holds |
| §3.10 **[re-entered after demotion]** negative control (c) — a user's own linked worktree, attached (`git worktree add -b userbr "$RIG/userwt2"`) | `milestone create` → exit **0**. Holds |
| §3.10 negative control (a) — lookalike `.jigc/worktrees/not-a-subtask` | `milestone create` → `repo.head-detached`. Holds — the registry leg discriminates |
| §3.11 GIT_DIR redirect (declared out) | `GIT_DIR=<B>/.git jigc milestone create "redirected milestone"` → exit **0**, `record: docs/milestone-records/redirected-milestone.md`, `record commit: 51b8ea8`. Repo **A** log 5→5 with `?? docs/milestone-records/`; repo **B** log 1→2 with ` D docs/milestone-records/redirected-milestone.md`. Holds — the stated bound, driven |
| §3.13 outside any git repository | `milestone create zz --format json` → `{"error": "not inside a git repository (no \`.git\` found from …) — run jigc from inside the target git repository; if this project isn't one yet, \`git init\` here first"}`, exit 1; same text bare at `task finalize`. Holds — not pre-empted by the guard |
| §3.14 `--format json` at a commit door, detached | stdout **0 bytes**; stderr the flattened single-key `{"error": "blocking · repo.head-detached — … \n  route: re-attach HEAD with \`git switch <branch>\`, then re-run this command"}`; exit 1. Holds |
| §3.14 `--format json` at a mover under a merge | `{"error": "blocking · repo.operation-in-progress — a merge is in progress …"}`, exit 1. Holds |
| §3.14 invocation log | `"argv":["task","finalize","record-the-axis-decision","--format","json"],"exit_code":1` with `"finding_codes":["repo.head-detached"]`; `"argv":["config","set","docs-root","docs","--format","json"],"exit_code":1` with `"finding_codes":["repo.operation-in-progress"]`. Holds at a commit door **and** at a mover |
| §3.14 route count | exactly **1** `route:` line in the refusal (`grep -c 'route:'` → 1). Holds |

## R.4 · Status roll-up

| id | origin | status |
|---|---|---|
| codex 1 — displacement's un-probed `git rm --cached` + squatter move (2 mover doors) | codex | **CONFIRMED** (repro + positive control; race-only reachability stated) |
| codex "consistent findings" ×13 | codex | recorded — 5 re-driven consistent, 7 consistent by read, **1 OPEN** (`git merge --ff-only` fan-out seam, not driven at the seam by either pass) |
| D1 rebase/bisect masked code + dead-end route | driver | **CONFIRMED** (re-driven) |
| D2 `rebase-apply` named *rebase*, route refused by git | driver | **CONFIRMED** (re-driven) |
| D3 cherry-pick + revert concluded at exit 0 | driver | **CONFIRMED** (re-driven; revert half re-entered after demotion) |
| D3b git's refusal dressed as a hook rejection | driver | **CONFIRMED** (re-driven) + one prose correction (a hook *does* exist in the rig) |
| §3.9 revert row | driver | **DEMOTED** (no block) → re-driven, holds |
| §3.10 negative controls (b), (c) | driver | **DEMOTED** (no induction in §5) → re-driven, hold |

Observations carried, not findings: §3.12's `--dry-run` forecast arm unreachable in a breached
posture (leaf-keyed, by design); §3.13's fake-`.git` mint printing neither `record:` nor
`record commit:`; §3.11's declared bound, now with a driven cost.

---

# §D · Doors covered

Every clap leaf that is the door of ≥1 **driven** row in this file, in `VERB_KINDS` spelling.
Acting class (12, driven in every cell of §3 plus R.2):

`setup` · `migrate-corpus` · `rename` · `relocate` · `task discard` · `task finalize` ·
`milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone finalize` ·
`milestone discard` · `config set`

`Neither` class driven as controls (§3.15, and `milestone provision` again in R.3 · WT):

`start` · `validate` · `doc list` · `uninstall` · `milestone provision` · `milestone join` ·
`ingest` · `unmanage` · `doc rename` · `migrate`

Write doors driven only as fixture construction for D3/D3-revert (not posture rows):
`doc set-field` · `doc set-slot`.

**22 leaves of 47** carry ≥1 driven *posture* row (24 leaves were driven in total, counting the two
write doors above, used only as fixture construction). Not driven, and stated as such: the remaining 25 `Neither`
leaves (the driver's §6.1 reason — one shared `refuse_on_posture` seam keyed on `Command::leaf()`,
⇔-fenced against the clap tree — re-read and accepted here).
