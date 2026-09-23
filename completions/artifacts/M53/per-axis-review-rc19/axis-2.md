<!-- M53 THIRD PARTIAL per-axis review — axis 2 — the reconciled file, copied verbatim. Driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.19` (repo HEAD `7d86f99f`), 2026-09-23. -->

# M53 — the THIRD PARTIAL per-axis review — AXIS 2 · posture — the OPUS DRIVER

**Binary.** Asserted **first, before anything else ran**:

```
argv : /Users/maurice/.local/bin/jigc --version
     -> jigc 1.0.0-rc.19
repo HEAD at read time: 7d86f99f ("chore(release): 1.0.0-rc.19 — the third M53 stamp, after the cwd-dependence arc")
```

This is the **release** posture: the debug-only `Route::mechanical` argv fence does not exist here, so a
route-fence violation shows up as a **bad emitted command**, never as a panic.

**Baselines re-driven.** [per-axis-review-rc18/axis-2.md](../../../../../../../Users/maurice/projects/gherrink-jigc/completions/artifacts/M53/per-axis-review-rc18/axis-2.md)
(rc.18, HEAD `fbd8b190`) — its two findings `(2, F-1)` and `(2, F-2)`, and the three M52 §A rows it
carries forward. **What changed under them is the cwd-dependence arc**
(`completions/artifacts/M53/cwd-census.md`; VERDICT → Addendum 2; the two reviews at
`completions/artifacts/M53/audit/cwd-fix-code-review{,-2}.md`).

**Fixtures.** `dev/jigc-rig` only (every root from `mktemp -d`, nothing torn down, `rm -rf $V/$D`
appears nowhere), plus the binary itself. **Nothing was fixed, committed or edited in the working
repository.** Ten rigs were built; two of them under a **repository path containing a space**
(`mktemp -d "$SCRATCH/jigc space.XXXXXX"`).

**The four cwds, used throughout** (the arc's own census shape, re-driven):
**(a)** `$REPO` — the repository root · **(b)** `$REPO/docs/deep` — an ordinary subdirectory ·
**(c)** `$REPO/.jigc/worktrees/<sub>` — a provisioned fan-out worktree ·
**(d)** an ordinary **linked** worktree outside `.jigc/` (branch-attached and detached, both).
A fifth, **`/`** — outside the repository entirely — was used wherever the row is about a *route the
operator pastes into a shell of unknown cwd*.

---

## 1 · THE HEADLINE — the tier-1 count

> # TIER-1 ROWS FOUND: **0**
>
> Tier 1 = exit-0 loss or repository harm through a committing, destroying or moving door. **No driven
> row on this axis reached it.** Every posture breach refused before anything durable was written; every
> destroying door refused or narrated; no byte was lost in any cell, including the two spaced-path rigs,
> the four cwds, and the foreign-worktree cell.
>
> **`(2, F-1)` — the rc.18 tier-2 row, and the reason the whole cwd arc exists — is CLOSED**, driven at
> **both** `aim_at` callers, from **five** cwds including outside the repository, and under a spaced
> repository path (§3.1).
>
> **Four new findings, none tier 1: one tier 2, three tier 3.** Two of them are **inside the cwd arc's
> own new code** (§6, N-1 and N-2 — one shared surface, two independent causes). Two are outside it
> (N-3's subject rule is M49's `provision_worktrees` reuse; N-4's baseline sharing is M46's
> *Concurrent writers*).

---

## 2 · The door set and the registry counts, read from the code at HEAD `7d86f99f`

| registry | file | count I read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1835` | **47** leaf rows (35 `Write` / 12 `Read`) |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2004` | **47** rows — the total classification |
| ↳ `ActsOnBehalf::CommitsOnBehalf` | | **10** — `setup` · `migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone finalize` · `milestone discard` |
| ↳ `ActsOnBehalf::MovesOnBehalf` | | **2** — `relocate` · `config set` |
| ↳ `ActsOnBehalf::Neither` | | **35** |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **10** rows |
| `PostureMember::ALL` | `crates/cli/src/repo.rs:167` | **3** |
| **`InProgress::ALL`** | `crates/cli/src/repo.rs:301` | **10** |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3430` | **6** (`PROVISION` · `DISCARD` · `UNINSTALL` · `TASK_DISCARD` · `TASK_FINALIZE` · `FINALIZE`) |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs` | **25** |
| `SLUG_DOORS` | `crates/cli/src/cli.rs` | **6** |
| **`PATH_ARG_OCCURRENCES`** | `crates/cli/src/cli.rs:3106` | **14** rows, and **every arm now carries a declared `base:`** (`Cwd` · `RepoRoot` · `NotAPath`) — the arc's addition. (rc.18's census read **8**; the growth is the arc's, and the rows themselves are axis 5's.) |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs` | **64** |
| `dev/jigc-rig --list-git-states` | | **17** |

**Axis 2's door set = the 12 acting `BEHALF_DOORS` rows** (10 commit-on-behalf ∪ 2 move-on-behalf),
**plus the second subject `986d5e0a` minted**: every provisioned sub-task worktree the boundary commits
from, and its preview at `jigc task validate <sub-id>`. The 35 `Neither` rows are driven as controls.

**The new mechanism, read before it was probed.** `crate::repo::jigc_home` (`repo.rs:91`) layers
`git rev-parse --path-format=absolute --git-common-dir` over the walk-up, so a store door standing in a
worktree resolves the **main** checkout; `crate::repo::aim_at` (`repo.rs:701`) is the `-C` redirection's
one home and now delegates to `engine::finding::git_at`, which renders
`git -C <shell_operand(absolute home)> <rest>`; `BreachSite` is `Here | FanOutWorktree{at, abs}`, with
`at` (repo-relative) feeding the message/locus/key and `abs` feeding the route span alone.

---

## 3 · The (door, cell) table

Exit / code / route are identical across doors within a cell unless the table says otherwise, so repro
blocks are **one per cell**.

### 3.0 · Clean control — no `repo.*` at any door, from any cwd

Driven as the pre-state of every fixture below (`git status --porcelain` empty; `ls .git | command grep
-E 'MERGE_MSG|MERGE_HEAD|CHERRY_PICK_HEAD|BISECT_LOG'` → none), and as the before-control on the hook
cell at §6 N-1 (an ordinary commit in a clean corpus prints **nothing** from the hook, rc=0).

---

### 3.1 · **`(2, F-1)` — the aimed route — CLOSED**

| door | producer | cwd | route as emitted | route run verbatim | verdict |
|---|---|---|---|---|---|
| `milestone finalize` | `BreachSite::aim` | (a)(b)(c)(d) | `git -C <ABS>/.jigc/worktrees/cw-area bisect reset` | rc **0** from (a)(b)(c)(d) **and `/`** | CLOSED |
| `task validate <sub>` | `BreachSite::aim` | (a)(b)(d) | same span, byte-identical | rc **0** | CLOSED |
| `milestone discard` | `milestone::held_here` | (a)(b)(d) | same span, inside the per-path `milestone.dirty-worktree` line | rc **0** | CLOSED |
| `uninstall` | `milestone::held_here` | (a)(b) | same span, inside `uninstall.dirty-worktree` | rc **0** | CLOSED |
| all four, **spaced repo root** | both producers | (a)(b) | `git -C '<ABS with a space>/…' bisect reset` — **single-quoted** by `shell_operand` | rc **0** from (a)(b)(c) **and `/`** | CLOSED |

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc milestone create "Cwd wave"; jigc milestone add-task cwd-wave "cw area"; jigc milestone provision cwd-wave
       W=$REPO/.jigc/worktrees/cw-area; git -C $W bisect start; git -C $W bisect bad
       mkdir -p $REPO/docs/deep; git -C $REPO worktree add -q $RIG/linked -b linkedbr
BEFORE: ls $REPO/.git/worktrees/cw-area -> BISECT_LOG BISECT_NAMES BISECT_START BISECT_TERMS …
        git -C $REPO status --porcelain -> (empty)

cwd=(a) $REPO            jigc milestone finalize cwd-wave -> exit 3
cwd=(b) $REPO/docs/deep  jigc milestone finalize cwd-wave -> exit 3     }  byte-identical
cwd=(d) $RIG/linked      jigc milestone finalize cwd-wave -> exit 3     }  message / at: / route
  blocking · repo.operation-in-progress — in the fan-out worktree `.jigc/worktrees/cw-area`, a bisect is
    in progress — the milestone boundary commits that worktree's index, and it is not in a committable state
    at: .jigc/worktrees/cw-area
    route: conclude it, or abandon it with `git -C /private/tmp/.../repo/.jigc/worktrees/cw-area bisect reset`,
           then re-run this command

THE EMITTED COMMAND, RUN VERBATIM:
  cwd=(a) $REPO                     -> rc=0  "HEAD is now at 2071345 …"
  cwd=(b) $REPO/docs/deep           -> rc=0
  cwd=(c) $REPO/.jigc/worktrees/…   -> rc=0
  cwd=(d) $RIG/linked               -> rc=0
  cwd=/   (outside the repository)  -> rc=0
(on rc.18 every one of these but (a) exited 128, `fatal: cannot change to '.jigc/…'`)

SPACED ROOT (second rig, $SCRATCH=…/jigc space.VFl1fk):
  emitted: git -C '/private/tmp/.../jigc space.VFl1fk/.../repo/.jigc/worktrees/sp-area' bisect reset
  run verbatim from (a)(b)(c) and /  -> rc=0, rc=0, rc=0, rc=0
```

**And the boundary lands once the route has been run** — nothing durable was written by the refusal:

```
argv : git -C <ABS>/.jigc/worktrees/cw-area bisect reset   (the emitted route, from (b))
       cd $REPO/.jigc/worktrees/cw-area; printf 'sub work\n' > subwork.txt; git add subwork.txt
       jigc milestone join cwd-wave; jigc milestone finalize cwd-wave       <- FROM INSIDE THE WORKTREE
observed: finalized c1f98ec — Finalize milestone cwd-wave (1 sub-task)
            modified docs/milestone-records/cwd-wave.md · added subwork.txt · 2 files committed
          exit=0 ; main HEAD 3d8a621 -> c1f98ec
```

---

### 3.2 · **`BreachSite::Here` from inside the breaching worktree** — the site discriminator, driven

> **[RECONCILER — DEMOTED, then re-driven]** This section was marked *driven* and carried **no repro
> block**, so as filed it was not driven. It is demoted here, and then re-driven by the reconciler on
> the same binary; the repro is ledger row **D-1** below. The verdict (exit **1**, `repo.operation-in-progress`,
> the **un-aimed** `git bisect reset`) holds.

| cwd | door | exit | code | route | verdict |
|---|---|---|---|---|---|
| (c), the bisecting worktree itself | `milestone finalize` · `milestone discard` · `task validate` | **1** | `repo.operation-in-progress` | `` `git bisect reset` `` — **un-aimed**, and correct: the caller *is* standing in that checkout | matches contract |

The exit differs by site — **3** at the boundary gate (`fan_out_posture_findings`), **1** at the dispatch
guard (`posture_refusal_in`) — and both are inside the taxonomy
(`design/command-output-contract.md` → The exit-code taxonomy: 3 = *blocking findings at a task-scope
gate*, 1 = *every reject that is not a task-scope gate*). Recorded, not filed.

---

### 3.3 · The ten `InProgress::ALL` members × the MAIN checkout

**All 16 buildable `--git-state` members, one fresh rig each**, driven at four acting doors from (a).
Every cell refuses; no cell was skipped.

| `--git-state` | member answered | noun printed | `milestone create` | `migrate-corpus` | `config set docs-root` | `setup` | `relocate` |
|---|---|---|---|---|---|---|---|
| `merge` | `Merge` | *a merge is in progress* | 1 | 1 | 1 | 1 | 1 |
| `squash-merge` | `SquashMerge` | *a squash merge is staged and not committed* | 1 | 1 | 1 | 1 | 1 |
| `rebase-merge` | `Rebase` | *a rebase is in progress* | 1 | 1 | 1 | 1 | 1 |
| `rebase-apply` | `Rebase` | *a rebase is in progress* | 1 | 1 | 1 | 1 | 1 |
| `am` | `Am` | *a `git am` is in progress* | 1 | 1 | 1 | 1 | 1 |
| `cherry-pick` | `CherryPick` | *a cherry-pick is in progress* | 1 | 1 | 1 | 1 | 1 |
| `sequencer` | `CherryPick` | *a cherry-pick is in progress* | 1 | 1 | 1 | 1 | 1 |
| `dangling-sequencer` | `Sequencer` | *a cherry-pick or revert left a queue of commits in `sequencer/`* | 1 | 1 | 1 | 1 | 1 |
| `uncommitted-pick` | `UncommittedCherryPick` | *an uncommitted cherry-pick is in progress* | 1 | 1 | 1 | 1 | 1 |
| `uncommitted-pick-range` | `UncommittedCherryPick` | same | 1 | 1 | 1 | 1 | 1 |
| `uncommitted-pick-conflicted` | `UncommittedCherryPick` | same | 1 | 1 | 1 | 1 | 1 |
| `uncommitted-pick-resolved` | `UncommittedCherryPick` | same | 1 | 1 | 1 | 1 | 1 |
| `revert` | `Revert` | *a revert is in progress* | 1 | 1 | 1 | 1 | 1 |
| `unmerged-index` | `UnmergedIndex` | *a conflict left unmerged paths in the index* | 1 | 1 | 1 | 1 | 1 |
| `bisect` | `Bisect` | *a bisect is in progress* | 1 | 1 | 1 | 1 | 1 |
| `detached` | `HeadDetached` | *HEAD is detached* | 1 | 1 | **0** ✔ | 1 | (falls through to `relocate.frozen-doctype`) ✔ |

Every non-`detached` row's code is `repo.operation-in-progress`; the `detached` row's is
`repo.head-detached`. The two **`0`/fall-through cells in the `detached` row are the declared mover
rule**, not a gap: `repo.rs` — *"a mover refuses only `PostureMember::OperationInProgress`"* — and both
movers **do** refuse every operation row above.

```
setup: for S in <all 16>: rig=$(dev/jigc-rig committed-singletons --git-state $S --binary …) || continue; eval "$rig"
argv : jigc milestone create "Sweep probe" · jigc migrate-corpus · jigc config set docs-root docs2 ·
       jigc setup · jigc relocate vision --from docs/vision/
sample observed (squash-merge):
  blocking · repo.operation-in-progress — a squash merge is staged and not committed — the repository is
    not in a committable state
    route: conclude it, or abandon it with `git reset --merge` (which also discards anything else you had
           staged, from the index and from your working tree), then re-run this command
  exit=1  at all five doors
```

**The remaining eight acting doors**, driven at one representative member (`bisect`) on the main
checkout, one rig, with a live plain task and a provisioned milestone:

```
setup: rig committed-singletons; milestone remaining-wave + sub-task rm-one provisioned;
       jigc start "a live plain task" --workflow quick-fix  -> task live-plain-task
       git bisect start; git bisect bad        (in the MAIN checkout)
argv/exit:
  jigc task finalize live-plain-task              -> 1  repo.operation-in-progress
  jigc task discard  live-plain-task              -> 1  repo.operation-in-progress
  jigc rename decisions-log:decisions-log --to X  -> 1  repo.operation-in-progress
  jigc milestone add-task remaining-wave "rm two" -> 1  repo.operation-in-progress
  jigc milestone add-from-spec … spec:nope        -> 1  repo.operation-in-progress
  jigc milestone finalize remaining-wave          -> 1  repo.operation-in-progress
  jigc milestone discard  remaining-wave          -> 1  repo.operation-in-progress
  jigc task validate live-plain-task              -> 1  repo.operation-in-progress
```

**All 12 acting doors are therefore covered**, and `InProgress::ALL` is covered member-by-member.

---

### 3.4 · The ten members × a provisioned sub-task worktree — the second subject

Driven at `bisect` across the four cwds (§3.1), and by re-inducing after each verbatim route run. The
subject is the **worktree**, the site is `FanOutWorktree`, and the refusal is placed **before** the
record flip: after the whole sweep the milestone record was still open and finalizable, HEAD unmoved,
and the sub-task's staged `subwork.txt` intact (§3.1's closing block lands it).

---

### 3.5 · HEAD detached · HEAD unborn · `setup`'s exemption

```
setup: rig=$(dev/jigc-rig --git-state unborn --binary …) || exit; eval "$rig"   (standalone; 0 commits)
argv/exit (BEFORE any setup):
  jigc milestone create "Unborn probe"       -> 1  repo.head-unborn
  jigc migrate-corpus                        -> 1  repo.head-unborn
  jigc rename decisions-log:… --to "X"       -> 1  repo.head-unborn
  jigc config set docs-root docs2            -> 0      (mover: declared)
  jigc relocate vision --from docs/vision/    -> 1  store.unknown-type   (past the posture guard: mover)
  jigc setup                                 -> 0, "install commit → abdeda3", commits 0 -> 1
```

`setup`'s `PostureExemption{HeadUnborn, SETUP_UNBORN_EXEMPTION}` is the only exemption in
`BEHALF_DOORS`, it is the only one exercised, and it does exactly what it declares.

**Detached, from an ordinary linked worktree** — the standing-checkout subject, driven:

```
setup: rig committed-singletons; git worktree add -q --detach $RIG/det; cd $RIG/det
       jigc start "probe detached commit" --workflow quick-fix; commit doc authored
argv : jigc task validate probe-detached-commit  -> exit 1
       jigc task finalize probe-detached-commit  -> exit 1
  blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no branch, and
    the next checkout would leave it unreachable
    route: re-attach HEAD with `git switch <branch>`, then re-run this command
CONTROL, the same task from the MAIN root: jigc task validate … -> exit 0, "no findings"
AFTER: main HEAD cd3e3fb (unmoved) · det HEAD cd3e3fb (unmoved)
```

**The subject is per-door, and it is the checkout that door commits in** — which is the arc's central
behavioural claim, and it holds in both directions:

| door class | standing in a **detached fan-out worktree**, main attached | why |
|---|---|---|
| `task finalize` / `task validate` (commit in the standing checkout) | **refuses** `repo.head-detached` | subject = standing checkout |
| `milestone create` / `add-task` / `task discard` / `config set` (commit in `jigc_home`) | **proceeds**, exit 0, commits land on **main** | subject = `jigc_home`, which is attached |

```
setup: rig3; milestone basegate-wave provisioned; cd $REPO/.jigc/worktrees/bg-one  (DETACHED)
argv/exit: jigc milestone create "Detached probe" -> 0   (main HEAD gains 4dda8e0)
           jigc milestone add-task …              -> 0   (main HEAD gains 7981643)
           jigc task discard bg-one               -> 0   (main HEAD gains 63ac336)
           jigc config set docs-root docs2        -> 0   (3 staged `git mv`s in the MAIN index)
           jigc milestone finalize basegate-wave  -> 3   finalize.base-mismatch
           jigc milestone discard  basegate-wave  -> 1   milestone.dirty-worktree
AFTER: main branch still `main`; the bg-one worktree, its staged c.txt and its detached HEAD all intact
```

---

### 3.6 · The no-override cross — no consent flag bypasses a posture refusal

```
setup: rig5, bisect induced in the MAIN checkout
argv/exit:
  jigc task finalize live-plain-task --carry-staged   -> 1  repo.operation-in-progress
  jigc task finalize live-plain-task --dry-run        -> 1  repo.operation-in-progress
  jigc milestone discard remaining-wave --force       -> 1  repo.operation-in-progress
  jigc task discard live-plain-task --force           -> 1  repo.operation-in-progress
  jigc milestone finalize remaining-wave --format json-> 1  repo.operation-in-progress
```

The family carries **no** consent flag, at every door, under every flag the door accepts. Matches
contract (`repo.rs` — *"a posture is a repository state the user can resolve, not bytes only they can
value"*).

---

### 3.7 · The pinned surfaces — the message / locus / key stay repo-relative, only the span goes absolute

This is the arc's central **trade**, and it is the one thing a re-review has to check byte by byte.

```
setup: rig5; fan-out rm-one provisioned; git -C $W bisect start; git -C $W bisect bad
argv : jigc milestone finalize remaining-wave --format json        (from (a), then (b), then (c))
observed (a) and (b) — IDENTICAL:
  exit=3 · stdout 882 bytes · stderr 0 bytes · keys ['findings','schema_version']
  key       {"code":"repo.operation-in-progress","target":".jigc/worktrees/rm-one"}
  location  {"address":".jigc/worktrees/rm-one","line":1,"col":1}
  message   absolute? False        at:/locus absolute? False        key absolute? False
  route     absolute? True   ->  "… `git -C /private/tmp/.../repo/.jigc/worktrees/rm-one bisect reset` …"
observed (c) — the `Here` arm:
  exit=1 · stdout 0 bytes · stderr 220 bytes · keys ['error']      (the declared flattened arm)
argv : jigc task validate rm-one --format json  -> exit 3, the same envelope, same key
```

**MEDIUM 1 of the rc.18 post-review review stays closed on rc.19**, and the arc did **not** leak the
absolute into the key. The `Here` `--format json` arm is the same single-`error` document rc.18 recorded
and the reconciler independently confirmed there (L-9).

**Invocation log**, both sites plus the `Here` site, one rig:

```
{'argv':['milestone','finalize','remaining-wave'],'exit_code':3,'finding_codes':['repo.operation-in-progress'],'error_code':None,'binary_version':'1.0.0-rc.19'}
{'argv':['task','validate','rm-one'],            'exit_code':3,'finding_codes':['repo.operation-in-progress'],'error_code':None,'binary_version':'1.0.0-rc.19'}
{'argv':['milestone','discard','remaining-wave'],'exit_code':1,'finding_codes':['milestone.dirty-worktree'], 'error_code':None,'binary_version':'1.0.0-rc.19'}
{'argv':['milestone','finalize','remaining-wave'],'exit_code':1,'finding_codes':['repo.operation-in-progress'],'error_code':None,'binary_version':'1.0.0-rc.19'}   <- run from INSIDE the worktree
```

The fourth row also proves the log itself binds `jigc_home`: the run from (c) landed in the **main**
checkout's `.jigc/logs/invocations.jsonl`.

---

### 3.8 · `PRE_DISPATCH_FAULTS` precedence — the deleted cwd beats the posture guard

```
setup: rig5 with the MAIN checkout mid-bisect; mkdir $REPO/gonedir; cd $REPO/gonedir; rmdir $REPO/gonedir
argv/exit (4 leaves across 3 verb kinds):
  jigc milestone create "Gone probe"     -> 1 | cannot determine the current directory: No such file or directory (os error 2)
  jigc task finalize live-plain-task     -> 1 | (identical line)
  jigc validate                          -> 1 | (identical line)
  jigc start                             -> 1 | (identical line)
  jigc milestone create … --format json  -> 1 | {"error": "cannot determine the current directory: …"}
```

One text at every leaf, ahead of the posture guard, on both arms. It carries **no code and no route** —
that is the shipped pre-dispatch-fault shape (M52 Increment 1), outside the findings envelope and
therefore outside the route floor. Recorded as observed, **not filed**.

---

### 3.9 · The cwd-arc verb cells the census ranked, re-driven

| census row | rc.18 behaviour | rc.19 behaviour, driven | verdict |
|---|---|---|---|
| **C2-06** — `milestone finalize` from inside a fan-out worktree | **rc=1, hard fail every time**, a raw git error with no code / route / `at:` | **exit 0**, boundary lands, `main HEAD 3d8a621 -> c1f98ec`, 2 files committed (§3.1) | **CLOSED** |
| **C2-11** — the base-mismatch gate does not fire from a worktree | the gate compared the **standing** checkout's HEAD, so a worktree pinned to the base evaded it | fires at **exit 3** from (c), comparing the **main** checkout's HEAD — driven in the discriminating shape (worktree HEAD **=** pin, main advanced past it) | **CLOSED** |
| **C2-07** — `uninstall` from a worktree half-uninstalls at exit 0 and claims completion | took the shared hook, left the main install standing, reported *"repo-local install removed"* | takes the **main** install in full, **prunes git's worktree registrations**, and the site line names the main checkout **and** says *"that workbench held the worktree you are standing in, which this removed"* | **CLOSED** |
| **C1-01** — `finalize.carried-staged` → `git restore --staged -- <root-rel>` | rc=1 from (b), unfixable from (c) | `git restore --staged` **aimed at the index that actually holds the path** (the linked worktree's, in the drive below); run verbatim rc=**0** from (a)(b)(d) and `/` | **CLOSED** |
| **C1-14 / C3-01** — the `Spawn: cd .jigc/worktrees/<id> && …` line | rc=1 from (b) and from a sibling worktree | **absolute and single-quoted**; run verbatim rc=**0** from (a), (b) and `/`, under a **spaced** repository path | **CLOSED** |
| the seventh `discover_repo_root` copy (pack loader) | the project pack-set vanished inside a worktree | `jigc describe` **md5-identical** from (a), (b) and (c) (`661b9df4…`, 105 lines each) | **CLOSED** |
| **C2-09** — `task finalize` from an ordinary linked worktree commits onto that worktree's branch | recorded as *undeclared cwd-determined commit targeting* | **unchanged**: commits onto `lwbr` (56b02b1), main unmoved. Correct for a plain task (the commit belongs where you stand) — but see **N-4**, which is its consequence for the shared baseline | **UNCHANGED**, and N-4 |

> **[RECONCILER — DEMOTIONS in this table]** Three rows above are marked driven and carry **no repro
> block anywhere in this file**: **C1-01** (`finalize.carried-staged`'s aimed route), **C1-14 / C3-01**
> (the `Spawn:` line's runnability) and **the seventh `discover_repo_root` copy** (the `describe`
> md5). As filed they are **not driven**; they are demoted. All three were re-driven by the
> reconciler on the same binary and hold — ledger rows **D-2**, **D-3**, **D-4**. **C2-09** is not
> demoted: it carries no block of its own, but **N-4**'s block is its repro.

**C2-06 / C2-11 repro:**

```
setup: rig3; milestone basegate-wave, sub-task bg-one provisioned; W=$REPO/.jigc/worktrees/bg-one
       printf 'code\n' > $W/c.txt; git -C $W add c.txt
       cd $REPO; printf 'y\n' > adv.txt; git add adv.txt; git -c core.hooksPath=/dev/null commit -m "feat: advance main"
BEFORE: PIN = worktree HEAD = 921290c ;  main HEAD = 08b4fc0   <- they DIFFER, which is what discriminates
argv : (cd $REPO && jigc milestone finalize basegate-wave)          -> exit 3  finalize.base-mismatch
       (cd $W    && jigc milestone finalize basegate-wave)          -> exit 3  finalize.base-mismatch   <- was silent on rc.18
  "…pinned to base `921290c…` but HEAD is now `08b4fc0…`…"   (the MAIN HEAD, from inside the worktree)
AFTER: main HEAD 08b4fc0 (unmoved)
```

**C2-07 repro (uninstall from a fan-out worktree):**

```
setup: rig6; milestone uninst-wave, sub-task un-one provisioned; cd $REPO/.jigc/worktrees/un-one
BEFORE: main .jigc/AGENT.md present · main .git/hooks/pre-commit present · main SKILL.md present · 2 worktrees
argv : jigc uninstall                                              -> exit 0
observed:
  warning: removing `.jigc/` also removes 5 tracked file(s) under it: …
    note: each is in the index, so `git -C /private/tmp/.../repo checkout -- <path>` brings it back.
  jigc uninstall — repo-local install removed
    - removed .jigc/
    - pruned git's worktree registrations for the fan-out worktrees `.jigc/` held
    … - removed pre-commit hook  - removed jigc guide artifact
    removed at `/private/tmp/.../repo` — the main checkout this repository's jigc install and `.jigc/`
    workbench bind to, and that workbench held the worktree you are standing in, which this removed
AFTER: main .jigc GONE · main hook GONE · main SKILL.md GONE · worktree dir GONE ·
       `git worktree list` -> 1 entry (the prune really ran)
```

**and its loss guard still fires from (c)** — the destroying door is not merely narrating:

```
setup: rig7; milestone guard-wave, sub-task gd-one provisioned; printf 'UNCOMMITTED-WORK\n' > $W/work.txt
       cd $W
argv : jigc uninstall                                              -> exit 1
  blocking · uninstall.dirty-worktree — `.jigc/` holds 1 fan-out sub-task worktree path(s) that removing
    it would destroy and nothing can say are disposable:
      .jigc/worktrees/gd-one: ?? work.txt — …
    route: get the work out of those paths first …; `jigc uninstall --force` deletes them with the install
AFTER: $W/work.txt INTACT · main .jigc/ present
```

---

### 3.10 · The two spaced-path rigs — the arc's own second question

The rc.18 fix's review named the lesson: *an absolute raises two questions — does it survive a shell, and
does everything that consumes it still parse it.* Both were driven.

| surface | spaced repository root | spaced **`docs-root`** |
|---|---|---|
| `aim_at` route span | `git -C '<…/jigc space…/…>' bisect reset` — single-quoted, rc **0** from 4 cwds | n/a |
| `Spawn:` `cd` | `cd '<…/jigc space…/…>' && jigc workflow sub-task --task sp-area` — rc **0** from (a), (b) and `/` | n/a |
| `git_at` in `home-vacated` / `carried-staged` / `migrate.source-untracked` | single-quoted, runnable | — |

> **[RECONCILER — DEMOTED, partly re-driven]** This row is marked driven and carries **no repro
> block**. Demoted. Two of its three surfaces were re-driven by the reconciler and hold —
> `carried-staged` (ledger **D-2**) and `home-vacated` (ledger **D-5**). **`migrate.source-untracked`
> was driven by neither pass and stays demoted.**
| the installed pre-commit **rename backstop** (review-2 MEDIUM 1) | a location-doctype staged `git mv` under a spaced **repository** root → commit **blocked**, rc 1 — identical to the plain control | `jigc config set docs-root "my docs"` at exit 0, then a staged `git mv` under `my docs/` → commit **blocked**, rc 1 | **CLOSED** |

```
setup: rig7; jigc config set docs-root "my docs"   -> exit 0, 1 doc relocated by staged `git mv`; committed
argv : git mv "my docs/milestone-records/guard-wave.md" "my docs/milestone-records/renamed-oob.md"; git commit -m …
observed: rc=1
  jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity
        tracking; use `jigc rename` instead (commit blocked).
CONTROL, plain docs-root, same act: rc=1, byte-identical line.
```

---

### 3.11 · Review-1 **MEDIUM 4** — `milestone create`'s base pin — CLOSED

```
setup: rig3; git -C $REPO worktree add -q $RIG/lw -b lwbr; cd $RIG/lw
       printf 'divergent\n' > d.txt; git add d.txt; git -c core.hooksPath=/dev/null commit -m "feat: diverge"
BEFORE: linked HEAD = 92d6160 · main HEAD = 08b4fc0        <- they DIFFER
argv : jigc milestone create "Linked wave"            -> exit 0
observed: "minted milestone:linked-wave (shared base 08b4fc0)"   <- the MAIN checkout's HEAD, not 92d6160
then  : jigc milestone add-task / provision / (stage work) / join / finalize   — all from $RIG/lw
observed: finalized a32c8bf — Finalize milestone linked-wave (1 sub-task), exit 0
AFTER : main HEAD = a32c8bf  ·  linked HEAD = 92d6160 (unmoved)
```

The milestone is **not born un-finalizable**: it was created, provisioned, joined and finalized end to
end from the linked worktree, and the boundary committed onto **main**. MEDIUM 4 closed on both legs.

---

### 3.12 · `jigc milestone provision` over a mid-operation worktree — idempotent, nothing destroyed

```
setup: rig_space (spaced root); sp-area provisioned; git -C $W bisect start; git -C $W bisect bad
       printf 'unstaged plant\n' > $W/plant.txt
BEFORE: BISECT_LOG present · plant.txt present
argv : jigc milestone provision space-wave          -> exit 0, "provisioned 1 worktree(s) … (sp-area)"
AFTER : BISECT_LOG present · plant.txt present        <- reused untouched, as `provision_route` declares
```

Matches contract. (The **ack's base clause** over a *pre-existing, un-provisioned* worktree is a
different cell, and it is **N-3** below.)

---

## 4 · M52 §A rows for this axis — CLOSED / STILL-OPEN

| M52 §A row | tier | verdict on rc.19 | the argv + observation that settles it |
|---|---|---|---|
| **`(2, DEFECT A)`** — a clean `git cherry-pick --no-commit` is a member of no `InProgress` row and `task finalize` concludes it at exit 0 | **1** | **CLOSED** | §3.3: all four `uncommitted-pick*` states × 5 doors → exit 1 `repo.operation-in-progress`, *an uncommitted cherry-pick is in progress*; §3.3's second block reaches `task finalize` / `task discard` / `rename` / the four milestone doors under `bisect`. No consent flag bypasses (§3.6) |
| **`(2, DEFECT C)`** — a posture breach at the **commit seam** prints no state-truth clause and no copy-runnable re-run | **2** | **STILL-OPEN** — *expected: triaged to the 1.x ledger by the charter, **not** a new finding* | §4.1 |
| **`(2, DEFECT B)`** — a conflicted `git merge --squash` is answered by `SquashMerge`, whose predicate is false of the state | **3** | **STILL-OPEN** — *expected: triaged to the 1.x ledger, **not** a new finding* | §4.2 |
| **`(2, F-1)`** (rc.18) | 2 | **CLOSED** | §3.1 |
| **`(2, F-2)`** (rc.18) | 3 | **STILL-OPEN**, reproduces exactly | §4.3 |
| M51 **D1** (rebase/bisect answer the operation, not the detachment) | — | **still CLOSED** | §3.3: `rebase-merge` and `rebase-apply` are HEAD-detached and still answer *a rebase*; only the bare `detached` state reaches `repo.head-detached` |
| M51 **D2** (`am` vs apply-backend rebase disjoint) | — | **still CLOSED** | §3.3: two nouns (*a `git am`* / *a rebase*), two abort commands (`git am --abort` / the rebase route) |

### 4.1 · `(2, DEFECT C)` — reproduces byte-for-byte, with its contrast arm

```
setup: rig=$(dev/jigc-rig committed-singletons --start quick-fix "seam probe" --binary …) || exit; eval "$rig"
       commit doc authored; `jigc task validate seam-probe` -> "no findings — the task validates clean"
       git add work.txt
       a deterministic `git` shim first on PATH that fires `git -C $REPO bisect start`+`bad` ONCE on the
       first `git add` of the run, then execs /usr/bin/git
argv : PATH="$RIG/shim:$PATH" jigc task finalize seam-probe
observed: exit=1
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  shim log: "shim: fired [bisect start]"                  <- the race really fired mid-transaction
STATE AFTER, all true and NONE of it stated by that surface:
  HEAD 049f0e6 (unmoved) · `work.txt` still staged · .jigc/tasks/seam-probe/docs/ intact
  (commit:seam-probe.md + provenance.json) · 1 active task · .git/BISECT_LOG present

CONTRAST — same door, same seam, a rejecting pre-commit hook instead:
  `git commit` was rejected (no commit was made):
  the hook says no

  task seam-probe is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/seam-probe/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize seam-probe`.
  exit=1
```

Two in-transaction failure causes at one seam; one prints the state-truth clause **and** the
copy-runnable re-run, the posture one prints neither.

### 4.2 · `(2, DEFECT B)` — reproduces exactly

```
setup: rig committed-singletons; branch `cb` and `main` both touch conf.txt; git merge --squash cb -> rc=1
state: markers -> MERGE_MSG, SQUASH_MSG  ·  git ls-files -u | wc -l -> 2   <- the index IS conflicted
argv : jigc milestone create "DefB probe"   -> exit 1
  blocking · repo.operation-in-progress — a squash merge is staged and not committed — …
```

*"staged and not committed"* is false of a state whose index carries conflicts. Tier 3, unchanged.

### 4.3 · `(2, F-2)` — reproduces exactly, no byte lost

```
setup: rig9; repo B = a SECOND repository (mktemp -d, 2 commits); milestone foreign-wave + sub-task fw-area
       NO `jigc milestone provision` — B's own worktree is parked at the sub-task path instead:
       git -C $B worktree add -q "$REPO/.jigc/worktrees/fw-area" -b fwbranch
       printf 'B-SECRET-PAYLOAD\n' > $W/bsecret.txt; git -C $W add bsecret.txt
       printf 'B-UNTRACKED-KEEP\n'  > $W/keepme.txt
BEFORE: git -C $W rev-parse --show-toplevel -> the squatted path itself · HEAD -> refs/heads/fwbranch
argv : jigc milestone join foreign-wave      -> exit 0
       jigc milestone finalize foreign-wave  -> exit 0
observed: finalized a5c2e0b — Finalize milestone foreign-wave (1 sub-task)
            added bsecret.txt · modified docs/milestone-records/foreign-wave.md · 2 files committed
            sub-tasks: fw-area: 1 code file            <- B's bytes, acked as the sub-task's own work
AFTER: git -C $REPO cat-file -p HEAD:bsecret.txt -> B-SECRET-PAYLOAD
       $W/keepme.txt -> B-UNTRACKED-KEEP (intact) · B's worktree registrations intact (2) ·
       .jigc/displaced/ -> absent
```

Tier 3 for the reason rc.18 gave and I re-measured: **no byte is lost**; the defect is the **ack**.

---

## 5 · What I did NOT drive, and why

Stated plainly; none of these is presented as driven.

1. **`InProgress::ALL` × each of the 12 acting doors as a full 10×12 cross.** I drove **10 members × 5
   doors** (§3.3, one fresh rig per member) and **1 member × all 12 doors** (§3.3, second block). The
   un-driven cells are the 9 non-`bisect` members at the 7 doors of the second block. The reason: the
   member is decided by one shared `adjudicated_breach` composition and the door only filters on
   `PostureMember`, so the cross is not independent — and both projections of it are driven.
2. **`InProgress::ALL` × the fan-out-worktree subject, member by member.** Driven at `bisect` only
   (§3.1/§3.4), across four cwds and both producers. rc.18 drove all ten there; the arc changed the
   *render*, not the detection, and the render is member-independent by construction
   (`aim_at` wraps whatever `site.aim` is handed).
3. **Both `finalize.fan-out.squash` commit models at the fan-out posture cell.** rc.18 drove both
   (§4.6) and the arc moved no commit-model code. I drove the default model only.
4. **The `GIT_DIR` redirect** — the family's *declared-out* bound, unchanged by the arc; rc.18 §6.4
   re-drove it and its disposition is written.
5. **`(2, DEFECT C)` at the milestone boundary's seam** (I raced the `task finalize` seam only).
6. **A genuinely concurrent racer** on any rollback population — out of this axis's scope and stated as
   a declared bound in M52's own record.
7. **The apostrophe / `#`-bearing repository root** — I drove the **space** axis on two rigs; the
   confirmation pass drove the apostrophe. Not re-driven here.
8. **Axis 5's `PATH_ARG_OCCURRENCES` base rows and axis 6's spawn-line arm** — I read the registry
   (§2) and drove the spawn line's **runnability** because it makes cwd (c), but those axes' tables are
   not mine and I do not claim them.

---

## 6 · Findings

### N-1 · **TIER 3** — *inside the cwd arc's own new code* — the installed pre-commit hook announces an out-of-band **rename** on a commit that contains no rename, and says the change is *"not staged in this commit"* while it is

**Cause, read after it was driven.** `crates/cli/src/setup.rs:634` gates its warning on
`moves="$(printf '%s' "$report" | grep -o 'git -C [^\`]*')"`. Before the arc, an operator-facing git
route was `git mv <new> <old>`; **the arc made every operator-facing git span `git -C <absolute> …`**
(`34584687`), so that grep now matches *any* route in the sweep report — `home-vacated`'s
`git -C <abs> show <sha> -- <path>`, `reconciliation.rename`'s `git -C <abs> restore --source=HEAD …`,
and so on. The `awk` below it correctly scans for the `mv` token and finds none, so the block does not
fire — and the **`else` echo** then prints a sentence about a rename, in a commit that has none.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
BEFORE-CONTROL (same rig, same hook): printf 'ordinary\n' > ord.txt; git add ord.txt; git commit -m "probe"
  -> rc=0, hook prints NOTHING
argv : git rm -q VISION.md ;  git status --porcelain -> "D  VISION.md"   (a DELETION; no rename anywhere)
       git commit -m "probe: delete a managed singleton, no rename anywhere"
observed: rc=0
  jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate` for
        details (not staged in this commit; commit not blocked).
  [main da29059] probe: delete a managed singleton, no rename anywhere
the report the guard grepped (driven separately):
  jigc validate -> exit 1, and the only `git -C` span in it is
  `git -C /private/tmp/.../repo restore --source=HEAD --staged --worktree -- CHANGELOG.md`   <- not a move
```

Both clauses of the printed sentence are false of the state it was printed for: there is **no rename**,
and the deletion **is** staged in this commit. Law 1 (*nothing lies*), at the one surface an operator
meets without asking for it.

**Why not tier 1:** nothing is destroyed and nothing is committed that the operator did not stage. The
sweep catches the real condition afterwards and **flips the exit** (`jigc validate` → exit **1**,
measured bare, not through a pipe).

---

### N-2 · **TIER 3** — the hook's *blocking* rename backstop is structurally unreachable for the **entire placement-doctype family**, and the same false sentence covers for it

Same surface, different cause, so filed separately. `reconciliation.rename` pairs old↔new by content
hash and emits the `git -C <repo> mv <new> <old>` revert route **only for a `location` doctype**. For a
`placement` doctype the sweep emits *"… is missing"* with a `jigc unmanage` route and **no `mv` pair at
all** — so the hook's `W[i]=="mv" && W[i+1] in S && W[i+2] in S` predicate can never be satisfied, for
any placement doc, at any repository path.

Driven over **all four** managed singletons a stock corpus has — every one of them a placement doctype —
one fresh rig each, with the staged state shown before each commit:

```
setup (×4): rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
argv       : git mv <A> <B> ; git status --porcelain ; git commit -m "probe <A>"

  docs/roadmap.md      -> docs/roadmap-oob.md      | staged: R  docs/roadmap.md -> docs/roadmap-oob.md      | rc=0
  VISION.md            -> VISION-OOB.md            | staged: R  VISION.md -> VISION-OOB.md                  | rc=0
  CHANGELOG.md         -> CHANGELOG-OOB.md         | staged: R  CHANGELOG.md -> CHANGELOG-OOB.md            | rc=0
  docs/decisions-log.md-> docs/dl-oob.md           | staged: R  docs/decisions-log.md -> docs/dl-oob.md     | rc=0
each printing: "…(not staged in this commit; commit not blocked)."

CONTRAST, same binary, same hook, a LOCATION doctype:
  docs/milestone-records/leftover-wave.md -> …/zz-oob.md | staged: R … | rc=1
  "jigc: out-of-band managed-doc rename staged in this commit — … (commit blocked)."

AND THE ASYMMETRY AT ITS SOURCE (one rig, both acts, `jigc validate`):
  location : "…is missing; docs/milestone-records/rp-oob.md has the same content hash — likely renamed
              via `git mv`"  route: … or revert the move: `git -C <abs> mv <new> <old>`
  placement: "…tracked managed doc vision:vision (VISION.md) is missing"
              route: restore VISION.md, or confirm the deletion by dropping it from the index:
                     `jigc unmanage VISION.md`                      <- no pair, so no block is possible
```

**Why not tier 1:** the renamed file is on disk, nothing is destroyed, and `jigc validate` raises
`reconciliation.rename` **and** `schema-conformance.home-vacated` and exits **1** (measured bare). The
defect is that a blocking backstop is silently inert over half its subject while printing a line that
asserts the opposite.

---

### N-3 · **TIER 3** — `jigc milestone provision` acks *"at base &lt;pin&gt;"* over a worktree it **reused** at a different commit, and the boundary's later refusal repeats the false claim

The reuse itself is declared (`provision_route`'s doc-comment: *"a worktree already registered at a
sub-task's path is reused untouched"* — M49). What is not declared is that the ack states the
milestone's pin as the base of a worktree standing somewhere else.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       jigc milestone create "Reuse wave"        -> "minted milestone:reuse-wave (shared base b0b890e)"
       jigc milestone add-task reuse-wave "ru one"
       printf 'x\n' > extra.txt; git add extra.txt; git -c core.hooksPath=/dev/null commit -m "chore: advance"
       git worktree add -q --detach "$REPO/.jigc/worktrees/ru-one"
BEFORE: milestone pin = b0b890e  ·  the parked worktree's HEAD = 155c759
argv : jigc milestone provision reuse-wave                     -> exit 0
observed: "provisioned 1 worktree(s) for milestone:reuse-wave at base b0b890e (ru-one)"
AFTER : git -C $REPO/.jigc/worktrees/ru-one rev-parse --short HEAD -> 155c759   <- NOT b0b890e
then  : jigc milestone finalize reuse-wave                     -> exit 3
  blocking · finalize.base-mismatch — …the sub-task worktrees were cut from `b0b890e…`, so combining them
    onto HEAD cannot be proven sound
```

Two law-1 claims the binary cannot support: the ack's *"at base b0b890e"*, and the refusal's *"the
sub-task worktrees were cut from b0b890e"* — that worktree was cut from `155c759`.

**Why not tier 1:** the boundary **blocks** (exit 3, `finalize.base-mismatch`), nothing lands and no
byte dies. The subject rule that admits the reuse is M49's, not the arc's.

---

### N-4 · **TIER 2** — a promoting `jigc task finalize` from an ordinary linked worktree lands the doc on that worktree's branch and re-baselines the **shared** `.jigc/` store, so the main checkout is left with a blocking finding that names an out-of-band edit that never happened, and a route that does not repair it

> **[RECONCILER — CONFIRMED IN PART, TIER CORRECTED TO 3]** The false diagnosis reproduces exactly.
> The **tier-2 escalation leg does not**: driven twice, the main checkout's next `task finalize`
> **absorbs** the drift (`advisory · reconciliation.absorb`, route *no action needed*) and lands at
> **exit 0**. And the supporting clause *"what no surface declares is that a commit target and a
> baseline home can be two different checkouts"* is falsified at the door itself — see ledger row
> **F-4**.

```
setup: rig3; git -C $REPO worktree add -q $RIG/lw -b lwbr; cd $RIG/lw
       jigc start "record the linked decision" --workflow decided-task    -> task record-the-linked-decision
       jigc doc add-item decisions-log#entries --task … --title "Linked branch decision"
         -> "(copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)"
       slots + commit doc authored
BEFORE: md5 $REPO/docs/decisions-log.md == md5 $RIG/lw/docs/decisions-log.md == 41fd3323a5b7566764cb350ceeaa6130
argv : jigc task finalize record-the-linked-decision            -> exit 0
observed: finalized 70bb73a — docs: record the linked decision
            promoted docs/decisions-log.md · 1 file committed
AFTER : main HEAD a32c8bf [main] (UNMOVED)   ·   linked HEAD 70bb73a [lwbr]
        grep -c 'Linked branch decision' $REPO/docs/decisions-log.md  -> 0
        grep -c 'Linked branch decision' $RIG/lw/docs/decisions-log.md -> 1
then  : (cd $REPO && jigc validate)                              -> exit 0 (store scope), but:
  blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/decisions-log.md`
    differs from the recorded state
    at: docs/decisions-log.md
    route: review the out-of-band edit to `docs/decisions-log.md` and re-author it through the owning workflow
```

There **was** no out-of-band edit: jigc itself wrote the new baseline into the shared `.jigc/state/`
from a different checkout at exit 0. The main checkout's next `task finalize` is now gated by that
finding, and the emitted route — *re-author it through the owning workflow* — would duplicate the entry
rather than reconcile the two checkouts. The `.jigc/` sharing is M46's declared design
(`design/storage.md` → Concurrent writers); what no surface declares is that a **commit target** and a
**baseline home** can be two different checkouts at the same door.

**Why tier 2 and not tier 1:** no byte is lost — the authored prose is committed on `lwbr` and
recoverable by an ordinary merge; nothing is destroyed; the main checkout's finding is *literally*
correct about the hash. It is a **route dead end** plus a diagnosis that names a cause that did not
occur, which is the charter's tier-2 predicate.

**Why tier 2 and not tier 3:** it is not only that a surface says something untrue; the state it leaves
behind **blocks a committing door** and the only route offered does not clear it.

---

## 7 · What this adds over flow-54 arm 4

`crates/cli/tests/flow54_acceptance.rs`'s posture arm iterates `InProgress::ALL × BEHALF_DOORS`' acting
rows from a **built fixture root**, in the **debug** binary, through the library seam. Four things here
are outside what that arm can reach, and three of them are the arc's whole point:

1. **Release posture.** Every row above ran on the installed `1.0.0-rc.19`, where the
   `Route::mechanical` argv fence does not exist. A bad emitted command is a *bad command*, not a panic
   — which is the only posture in which §3.1's central question (*does the emitted route run?*) can be
   asked at all.
2. **The route is executed, not asserted.** The arm asserts the route's **text**. §3.1 runs it —
   `git -C <abs> bisect reset`, verbatim, from five cwds including outside the repository and under a
   spaced repository path — and records git's exit. rc.18's tier-2 row was a route whose *text* was
   right and whose *execution* was 128; a text assertion cannot tell those apart.
3. **cwd is a dimension the suite does not have.** A `#[test]` runs from the crate's working directory.
   §3.5's split — the same binary refusing `repo.head-detached` at `task finalize` and proceeding at
   `milestone create`, from one cwd, because the two doors commit in different checkouts — only exists
   once the driver *moves*. So do C2-06, C2-11, the `uninstall` prune and the pack-set identity
   (§3.9), all four of which were **behaviour changes** the arc landed.
4. **Shell-level composition.** N-1 lives in a **shell script the binary writes**, invoked by **git**,
   greping a report the binary printed. No Rust-side fence spans those three. It was found by staging a
   deletion and reading what the hook then said, which is a thing only a driver does.

---

## 8 · Doors covered

Every clap leaf that is the door of at least one **driven** row above (`VERB_KINDS` spelling), **21**:

`setup` · `uninstall` · `migrate-corpus` · `rename` · `relocate` · `config set` · `validate` ·
`describe` · `start` · `workflow` · `task finalize` · `task discard` · `task validate` ·
`milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone finalize` ·
`milestone discard` · `milestone provision` · `milestone join` · `milestone execute`

**All 12 acting `BEHALF_DOORS` rows are among them.** Leaves used only to build fixtures
(`doc create`, `doc add-item`, `doc set-field`, `doc set-slot`, `doc list`, `doc schema`, `task list`,
`config get`) are deliberately **not** claimed as covered: no verdict row is keyed on them.

---

## 9 · Instrument honesty

- Two exit codes in this file were first read **through a pipe** and were wrong; both were re-measured
  **bare** before being written down (`jigc validate` on the vacated-home corpus: `| head` gave 0, bare
  gives **1**; `jigc milestone finalize` at §3.9: `| head -3` gave 0, bare gives **3**). No exit code in
  the tables above comes from a pipeline.
- Every rig root came from `mktemp -d`; nothing was torn down; `rm -rf` on a variable path appears
  nowhere in this review.
- The Codex source pass for this axis was **not read**.
- N-1 and N-2 are one printed surface with two independent causes and two different fixes; filing them
  as one row would have hidden whichever fix was not taken.
- The tiering of N-4 is the judgment I am least sure of and I have written both sides of it rather than
  only the one I chose.

---

# RECONCILIATION — axis 2 · posture · rc.19

**Inputs.** The Opus driver table above (`driver/axis2.md`) and the unseeded Codex source pass
(`codex/axis2-codex.md`, source-only by its own stated bound — it drove nothing). **The rule
applied** (acceptance-design.md → *The reconciliation rule*): *a claim by one that the other cannot
reproduce is a **lead**, not a finding.* Every Codex claim below was entered as `lead(codex, …)` and
then **driven** — to a repro block, or to a refutation with the falsifying datum, or left an **OPEN
LEAD** with the reason it could not be driven. No Codex claim was promoted on the source read alone.

**Binary, asserted first, before any probe ran:**

```
argv : /Users/maurice/.local/bin/jigc --version
     -> jigc 1.0.0-rc.19
repo HEAD at read time: 7d86f99f
```

**Fixtures.** `dev/jigc-rig` only, two-step eval (`rig=$(dev/jigc-rig <state> --binary …) || exit;
eval "$rig"`), every root from `mktemp -d`, no teardown, no `rm -rf` on a variable path anywhere.
Nineteen rigs. **Nothing was fixed, committed or edited in the working repository.** Where the
reconciler's probes needed a second repository or a spaced root, those too came from `mktemp -d`.

**Instrument honesty.** Every exit code below was read **bare**, never through a pipe. Three probes
were written wrong first and re-run, and the wrong runs are named where they change what a row can
claim: `set-field` was called with the wrong argument shape (no verdict was taken from it); the
`carried-staged` probe staged its plant **after** the mint, which is not the carryover cell, and was
re-driven staging **before** it; and the mover-seam probe first ran on a corpus with **no**
location-doctype instance, so it moved nothing and proved nothing, and was re-driven with a
milestone record present.

---

## 1 · Codex claims → CONFIRMED · REFUTED · OPEN LEAD

Twenty-one claims read out of the source pass (its one "Claims" entry, its ten baseline-row
dispositions, and the nine propositions in its *Completeness and consistency read*).
**17 CONFIRMED · 0 REFUTED · 4 OPEN LEAD.** No Codex claim contradicted a driven driver row.

### CX-1 — a foreign repository's worktree parked at `.jigc/worktrees/<sub-id>` is accepted as a live sub-task worktree, and `milestone finalize` commits that repository's staged bytes as jigc work — **CONFIRMED** (this is `(2, F-2)`)

Driven by the reconciler, independently of the driver, on a second repository built from `mktemp -d`:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       B=$(mktemp -d …); git -C $B init -q; two commits in B
       jigc milestone create "Foreign wave"; jigc milestone add-task foreign-wave "fw area"
       NO `jigc milestone provision` — B's own worktree is parked at the sub-task path instead:
       git -C $B worktree add -q "$REPO/.jigc/worktrees/fw-area" -b fwbranch
       printf 'B-SECRET-PAYLOAD\n' > $W/bsecret.txt; git -C $W add bsecret.txt
       printf 'B-UNTRACKED-KEEP\n'  > $W/keepme.txt
BEFORE: git -C $W rev-parse --show-toplevel -> the squatted path itself
        git -C $W symbolic-ref HEAD          -> refs/heads/fwbranch   (B's branch)
        main HEAD add9582
argv : jigc milestone join foreign-wave      -> exit 0
         "joined milestone:foreign-wave — 0 doc(s) merged / no docs staged from: fw-area"
       jigc milestone finalize foreign-wave  -> exit 0
         "finalized 7762776 — Finalize milestone foreign-wave (1 sub-task)
            added bsecret.txt · modified docs/milestone-records/foreign-wave.md · 2 files committed
            sub-tasks: fw-area: 1 code file"        <- B's bytes, acked as the sub-task's own work
AFTER: git -C $REPO cat-file -p HEAD:bsecret.txt -> B-SECRET-PAYLOAD
       $W/keepme.txt -> B-UNTRACKED-KEEP (intact) · B's worktree registrations intact (2) ·
       .jigc/displaced/ -> absent
```

Byte-for-byte the driver's §4.3. **Tier 3 stands and the reason stands**: no byte is lost — B's
tracked payload is *copied* into A's commit and B's untracked file is untouched; the defect is that
the boundary accepts a checkout it never cut and the ack attributes those bytes to the sub-task.

### CX-2 — `(2, DEFECT 1)` CLOSED: all ten `InProgress::ALL` members remain enumerated — **CONFIRMED**

```
setup: for S in 14 git states: rig=$(dev/jigc-rig committed-singletons --git-state $S --binary …) || continue; eval "$rig"
argv : jigc milestone create "Sweep probe"          (one fresh rig per state)
observed (state -> exit · code · noun · abandon route):
  uncommitted-pick            1 repo.operation-in-progress  an uncommitted cherry-pick is in progress  `git reset`
  uncommitted-pick-range      1 repo.operation-in-progress  an uncommitted cherry-pick is in progress  `git reset`
  uncommitted-pick-conflicted 1 repo.operation-in-progress  an uncommitted cherry-pick is in progress  `git reset`
  uncommitted-pick-resolved   1 repo.operation-in-progress  an uncommitted cherry-pick is in progress  `git reset`
  am                          1 repo.operation-in-progress  a `git am` is in progress                  `git am --abort`
  rebase-apply                1 repo.operation-in-progress  a rebase is in progress                    `git rebase --abort`
  rebase-merge                1 repo.operation-in-progress  a rebase is in progress                    `git rebase --abort`
  detached                    1 repo.head-detached          HEAD is detached …                         (re-attach route)
  unmerged-index              1 repo.operation-in-progress  a conflict left unmerged paths in the index `git reset --merge`
  dangling-sequencer          1 repo.operation-in-progress  a cherry-pick or revert left a queue …      `git cherry-pick --quit`
  revert                      1 repo.operation-in-progress  a revert is in progress                    `git revert --abort`
  cherry-pick                 1 repo.operation-in-progress  a cherry-pick is in progress               `git cherry-pick --abort`
  sequencer                   1 repo.operation-in-progress  a cherry-pick is in progress               `git cherry-pick --abort`
  merge                       1 repo.operation-in-progress  a merge is in progress                     `git merge --abort`
```

Ten distinct members answered, `UncommittedCherryPick` and `UnmergedIndex` among them; every cell
refuses. (The driver drove the same set at five doors; this is the reconciler's own datum.)

### CX-3 — `(2, DEFECT A)` / M51 `D3` CLOSED: the four uncommitted-pick shapes are `UncommittedCherryPick`, detected **before** the unmerged-index fallback — **CONFIRMED**

CX-2's first four rows. The discriminating cell is `uncommitted-pick-conflicted`: its index **is**
conflicted, so a detection ordered the other way would answer *a conflict left unmerged paths in the
index*; it answers *an uncommitted cherry-pick is in progress*.

### CX-4 — M51 `D1` CLOSED: operation detection takes precedence over detached-HEAD rendering — **CONFIRMED**

CX-2: `rebase-apply` and `rebase-merge` are detached-HEAD states in git and answer
`repo.operation-in-progress` / *a rebase is in progress*. Only the bare `detached` state reaches
`repo.head-detached`.

### CX-5 — M51 `D2` CLOSED: `am` and apply-backend `rebase` are disjoint — **CONFIRMED**

CX-2: two nouns (*a `git am` is in progress* / *a rebase is in progress*) and two abort commands
(`git am --abort` / `git rebase --abort`).

### CX-6 — M51 `D3b` CLOSED on source: committing doors converge on `git_commit_capture`, which re-verifies immediately before invoking git — **CONFIRMED, driven**

The source claim is about an *ordering inside a seam*; it was driven by making the posture appear
**mid-transaction**, after the door's own front-gate probe had already passed:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       jigc start "seam probe" --workflow quick-fix ; commit doc authored
       jigc task validate seam-probe -> "no findings — the task validates clean"
       /usr/bin/git add work.txt
       a `git` shim first on PATH that fires `git -C $REPO bisect start` + `bad` ONCE, on the first
       `git add` OF THE FINALIZE RUN, then execs /usr/bin/git
argv : PATH="$RIG/shim:$PATH" jigc task finalize seam-probe
observed: exit=1
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  shim log: "shim: fired [bisect start] on: add -- .jigc/config .jigc/.gitignore .jigc/version"
AFTER: HEAD b3096a3 (UNMOVED) · work.txt still staged · .jigc/tasks/seam-probe/docs/ intact
       (commit:seam-probe.md + provenance.json) · 1 active task · BISECT_LOG present
```

The re-probe fires and **no commit is made**. (What that surface then *says* is `(2, DEFECT C)` —
CX-11.)

### CX-7a — M51 `codex-1` CLOSED, the `git mv` leg: an immediate `verify(Move)` precedes the move — **CONFIRMED, driven**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       jigc milestone create "Mv wave"        (so a LOCATION doctype instance exists to move)
       a `git` shim first on PATH that logs every git call and fires `bisect start`+`bad` on the
       run's FIRST git call — which is the door's own posture probe, `git ls-files -u`
argv : PATH="$RIG/shim:$PATH" jigc config set docs-root docs2
observed: exit=1
  relocating 1 committed doc(s) stranded by the `docs-root` re-point to `docs2` …
  blocking · config.repoint-failed — `docs-root` was not set to `docs2`: blocking ·
    repo.operation-in-progress — a bisect is in progress — the repository is not in a committable state
    at: docs/milestone-records/mv-wave.md
    route: … — the re-point was undone
    route: `docs-root` is unchanged and every doc this re-point moved is back at its prior home …
git-call log: `git mv` executed 0 times   (the run's 12 git calls are rev-parse / ls-files / symbolic-ref)
AFTER: docs/milestone-records/mv-wave.md at its prior home · index clean · docs-root unchanged
```

The posture appeared **after** the front-gate probe read a clean index, and the mover still refused
**before** any `git mv` ran. *Observation, not a finding:* the undone re-point leaves an empty
`docs2/milestone-records` directory behind; no byte is lost and the ack's claim (*every doc … is back
at its prior home*) is true.

### CX-7b — M51 `codex-1`, the displacement `git rm --cached` leg — **OPEN LEAD**

Reason: driving it needs **two** things at once — a foreign-squatter displacement at a relocation
destination, *and* a racer that lands the posture between that seam's own probe and its `git rm
--cached`. The rig builds no displacement state and the shim can only fire on a git call, which is
one side of the window. Not driven; **not promoted on the source read**.

### CX-8 — `(2, F-1)` CLOSED: `aim_at` delegates to `git_at`, producing `git -C <quoted absolute> …` — **CONFIRMED**

Re-driven by the reconciler at the boundary door from four cwds, then the emitted command run
verbatim from five (the fifth outside the repository), then again under a **spaced** repository root:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       jigc milestone create "Cwd wave"; jigc milestone add-task cwd-wave "cw area"
       jigc milestone provision cwd-wave
       W=$REPO/.jigc/worktrees/cw-area; git -C $W bisect start; git -C $W bisect bad
       mkdir -p $REPO/docs/deep; git -C $REPO worktree add -q $RIG/linked -b linkedbr
BEFORE: ls $REPO/.git/worktrees/cw-area -> BISECT_LOG BISECT_NAMES BISECT_START BISECT_TERMS …
        git -C $REPO status --porcelain -> (empty)
argv : jigc milestone finalize cwd-wave, from (a) $REPO, (b) $REPO/docs/deep, (d) $RIG/linked
observed: exit 3 at all three, byte-identical:
  blocking · repo.operation-in-progress — in the fan-out worktree `.jigc/worktrees/cw-area`, a bisect is
    in progress — the milestone boundary commits that worktree's index, and it is not in a committable state
    at: .jigc/worktrees/cw-area
    route: … abandon it with `git -C /private/var/folders/…/repo/.jigc/worktrees/cw-area bisect reset` …
THE EMITTED COMMAND, RUN VERBATIM (re-inducing the bisect between runs):
  (a) $REPO  rc=0 · (b) docs/deep  rc=0 · (c) the worktree  rc=0 · (d) $RIG/linked  rc=0 · / rc=0
SPACED ROOT (TMPDIR="…/jigc space.EVQQFY/"):
  emitted: git -C '/private/var/folders/…/jigc space.EVQQFY/…/repo/.jigc/worktrees/sp-area' bisect reset
  run verbatim from (a) (b) (c) and /  -> rc=0, rc=0, rc=0, rc=0
```

### CX-9 — `(2, F-2)` STILL-OPEN — **CONFIRMED** (= CX-1)

### CX-10 — `(2, DEFECT B)` STILL-OPEN: `SquashMerge` is detected by `SQUASH_MSG && !MERGE_HEAD`, so it answers a *conflicted* squash merge with a predicate that is false of it — **CONFIRMED**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       conf.txt touched on `main` and on branch `cb`; git merge --squash cb -> rc=1
state: ls .git -> MERGE_MSG SQUASH_MSG      ·      git ls-files -u | wc -l -> 3
argv : jigc milestone create "DefB probe"   -> exit 1
  blocking · repo.operation-in-progress — a squash merge is staged and not committed — the repository
    is not in a committable state
    route: conclude it, or abandon it with `git reset --merge` …
```

Nothing is *staged*: the index carries three unmerged entries. Tier 3, unchanged.

### CX-11 — `(2, DEFECT C)` STILL-OPEN: the commit seam prints a raw posture refusal instead of the door's state-truth clause and copy-runnable re-run — **CONFIRMED**

CX-6's block is the first arm: the refusal says nothing about HEAD being unmoved, the staged index,
the intact task docs or how to resume. The contrast arm, same door, same seam, a rejecting
pre-commit hook instead of a posture, driven on its own rig:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       jigc start "hook probe" --workflow quick-fix; commit doc authored; git add work.txt
       printf '#!/bin/sh\necho "the hook says no" >&2\nexit 1\n' > .git/hooks/pre-commit
argv : jigc task finalize hook-probe   -> exit 1
  `git commit` was rejected (no commit was made):
  the hook says no

  task hook-probe is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/hook-probe/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize hook-probe`.
```

Two in-transaction failure causes at one seam; one carries the frame, the other carries neither half.

### CX-12 — rc.18 MEDIUM 1 CLOSED: the fan-out message / locus / key stay repo-relative and only the route span goes absolute — **CONFIRMED**

CX-8's observed block: `at: .jigc/worktrees/cw-area` and the message's own path are repo-relative in
all four cwds, and the only absolute in the surface is inside the route's `git -C` span — including
under the spaced root, where it is single-quoted.

### CX-13 — `BEHALF_DOORS` is total: ten commit-on-behalf + two mover leaves acting, 35 `Neither` — **CONFIRMED behaviourally**, with a stated bound

```
setup: rig=$(dev/jigc-rig committed-singletons --git-state bisect --binary …) || exit; eval "$rig"
argv/exit (same rig, same induced bisect):
  jigc describe              -> 0   posture findings: 0
  jigc validate              -> 0   posture findings: 0
  jigc task list             -> 0   posture findings: 0
  jigc doc list              -> 0   posture findings: 0
  jigc config get docs-root  -> 0   posture findings: 0
  jigc doc show vision       -> 0   posture findings: 0
  jigc start                 -> 0   posture findings: 0
  jigc milestone create "…"  -> 1   posture findings: 1   <- the acting contrast, same rig
```

**Bound:** the *bijection fence* over the clap tree is a source-side fact neither pass drove; what is
driven is that seven `Neither` leaves proceed under a posture that refuses an acting leaf in the same
repository, and that the driver's §3.3 reaches all twelve acting rows.

### CX-14 — `COMMITTING_DOORS` is the hook-capable subset and `setup` is the intentional `--no-verify` exception — **CONFIRMED**

```
setup: rig=$(dev/jigc-rig bare --binary …) || exit; eval "$rig"     (a repo with NO jigc setup)
       printf '#!/bin/sh\n# FOREIGN-HOOK-SENTINEL\necho "HOOK SAYS NO" >&2\nexit 1\n' > .git/hooks/pre-commit
BEFORE: a plain `git commit` -> rc=1 (the foreign hook rejects)
argv : jigc setup            -> exit 0, commits 1 -> 2, "install commit → 335e9c4"
AFTER: the foreign hook is NOT clobbered — its sentinel survives at .git/hooks/pre-commit:59, chained
       inside jigc's regenerated hook, and a plain `git commit` still rc=1
```

`setup` commits over a hook that rejects every other commit, and it does so without destroying the
hook it found. (Checked because the same probe could have exposed a clobber; it did not.)

### CX-15 — no production HEAD-changing `git switch` or ordinary `git checkout`; `checkout-index` / `read-tree` do not change HEAD — **CONFIRMED over the driven door set**, with a stated bound

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       a `git` shim first on PATH logging EVERY invocation, then exec'ing /usr/bin/git
argv : milestone create · add-task · provision · join · finalize · start · task finalize ·
       start · task discard --force · rename · milestone create · milestone discard --force ·
       migrate-corpus · config set docs-root · relocate · uninstall --force      (16 runs)
observed: 256 git invocations. Subcommand census:
  rev-parse 57 · symbolic-ref 41 · ls-files 33 · diff 19 · worktree 18 · show 8 · status 7 · add 7 ·
  commit 6 · read-tree 5 · write-tree 3 · log 3 · mv 2 · cat-file 2 · apply 2 · rev-list 1 ·
  merge-base 1 · merge 1 · checkout-index 1
  `git switch`: 0    bare `git checkout`: 0
  the checkout family in full: 4× `git read-tree …`, 1× `git read-tree --reset -u …`,
                               1× `git checkout-index -a --prefix=/var/folders/…/jigc-index-…/`
```

**Bound:** this is a census over the doors driven here, not over the source. A production
`git switch` on a path none of these 16 runs reaches would not appear.

### CX-16 — no `Neither` leaf reaches the commit/move primitives — **CONFIRMED in part**, with a stated bound

CX-13's seven `Neither` leaves commit and move nothing under a posture (exit 0, no refusal, no
index change), and CX-15's 256-call log attributes every `commit`, `mv`, `merge` and `write-tree`
to an acting door's run. **Bound:** seven of thirty-five leaves, not an exhaustive sweep.

### CX-17 — `DedicatedWorktree` is unforgeable through public construction (private fields, private `add`) — **OPEN LEAD**

Reason: a claim about Rust item visibility. No state a driven binary can be put into exhibits or
falsifies it; a fence for it is a compile-time fact. Recorded, **not promoted**.

### CX-18 — the unborn exemption is confined to `setup` — **CONFIRMED on the driver's repro, not re-driven here**

The driver's §3.5 block drives the whole unborn cell on a standalone `--git-state unborn` rig
(`milestone create` / `migrate-corpus` / `rename` → `repo.head-unborn`; `config set` → 0 as the
declared mover rule; `setup` → 0 and commits 0 → 1). That row **has** a repro block and stands as
driven; the reconciler did not spend a nineteenth rig on it.

### CX-19 — the M52 registries (rollback populations, work-area contents, pre-dispatch precedence, destruction, envelopes, fixed identity, store exits) introduce no axis-2 bypass — **OPEN LEAD**

Reason: as stated it is a negative over source structure with no named state to reach. Its one
*drivable* projection — that the pre-dispatch fault registry outranks the posture guard — is already
a driven row above (driver §3.8, which carries its repro block). Recorded, not promoted.

### CX-20 — `discover_repo_root` has one implementation and the store/checkout subject choice is explicit at callers — **CONFIRMED behaviourally**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       a milestone with two provisioned sub-task worktrees; mkdir -p $REPO/docs/deep
argv : jigc describe, from (a) $REPO, (b) $REPO/docs/deep, (c) $REPO/.jigc/worktrees/sp-one
observed: md5 661b9df4cfb5f5757ec6b221e6244c1b at all three, 105 lines at all three
```

The pack-set the composition resolves is identical from a subdirectory and from inside a fan-out
worktree — and the md5 is the same value the driver recorded independently on its own rig.

### CX-21 — no schema-hash movement or schema-shape change in this arc — **CONFIRMED by repo datum**

```
argv : git diff --name-only fbd8b190..HEAD          (fbd8b190 = the rc.18 stamp; HEAD = 7d86f99f)
       | grep -E 'schema|manifest'                  -> (empty)
       git diff --stat fbd8b190..HEAD -- crates/cli/pack packs/methodology  -> (empty)
```

Nothing under either pack moved across the whole rc.18 → rc.19 arc, so no `schema-hash` could.

---

## 2 · Driver rows → status

Every driver **finding** was re-driven by the reconciler on the same binary, independently of the
driver's scripts. The source pass contradicts none of them; where it is **silent**, the row still
stands, because it was driven.

| driver row | tier | source pass | reconciler's status |
|---|---|---|---|
| `(2, F-1)` — the aimed route | 2 (rc.18) | agrees CLOSED | **CLOSED**, re-driven (CX-8), incl. the spaced-root leg |
| `(2, F-2)` — the foreign worktree | 3 | agrees STILL-OPEN | **STILL-OPEN**, re-driven (CX-1) |
| `(2, DEFECT A)` — the uncommitted cherry-pick | 1 | agrees CLOSED | **CLOSED**, re-driven (CX-2/CX-3) |
| `(2, DEFECT B)` — the conflicted squash merge | 3 | agrees STILL-OPEN | **STILL-OPEN**, re-driven (CX-10) |
| `(2, DEFECT C)` — the commit seam's bare refusal | 2 | agrees STILL-OPEN | **STILL-OPEN**, re-driven (CX-6/CX-11) |
| M51 `D1` / `D2` | — | agrees CLOSED | **CLOSED**, re-driven (CX-4/CX-5) |
| **N-1** — the hook announces a rename on a commit with none | 3 | silent | **CONFIRMED** — F-1 below |
| **N-2** — the blocking backstop is inert for the whole placement family | 3 | silent | **CONFIRMED** — F-2 below |
| **N-3** — `provision` acks a base the reused worktree does not stand at | 3 | silent | **CONFIRMED** — F-3 below |
| **N-4** — the linked-worktree promotion's residue | **2** | silent | **CONFIRMED IN PART, TIER CORRECTED TO 3** — F-4 below |

### F-1 · N-1 re-driven — the installed pre-commit hook announces an out-of-band **rename** on a commit that contains none, and calls the change *"not staged in this commit"* while it is

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
BEFORE-CONTROL (same rig, same hook): printf 'ordinary\n' > ord.txt; git add ord.txt; git commit -m "probe: ordinary"
  -> rc=0, the hook prints NOTHING
argv : git rm -q VISION.md
       git status --porcelain  ->  "D  VISION.md"        (a DELETION; no rename anywhere)
       git commit -m "probe: delete a managed singleton, no rename anywhere"
observed: rc=0
  jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate` for
        details (not staged in this commit; commit not blocked).
  [main 379ff3c] probe: delete a managed singleton, no rename anywhere
the report the guard greps (driven bare, after the commit): jigc validate -> exit 1, and its only
  `git -C` span is `git -C <abs>/repo show 379ff3c -- VISION.md`   <- a `show`, not a move
```

Both clauses are false of the state they were printed for. Tier 3: nothing is destroyed, and
`jigc validate` catches the real condition and flips the exit.

### F-2 · N-2 re-driven — the hook's *blocking* backstop is structurally unreachable for the entire placement-doctype family, and the same false sentence covers for it

```
setup (×4, one fresh rig each): rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
argv  (×4): git mv <A> <B> ; git status --porcelain ; git commit -m "probe <A>"

  VISION.md             -> VISION-OOB.md         | staged: R  VISION.md -> VISION-OOB.md            | rc=0
  CHANGELOG.md          -> CHANGELOG-OOB.md      | staged: R  CHANGELOG.md -> CHANGELOG-OOB.md      | rc=0
  docs/roadmap.md       -> docs/roadmap-oob.md   | staged: R  docs/roadmap.md -> docs/roadmap-oob.md| rc=0
  docs/decisions-log.md -> docs/dl-oob.md        | staged: R  docs/decisions-log.md -> docs/dl-oob.md| rc=0
each printing: "…an out-of-band managed-doc rename exists in the committed tree … (not staged in this
               commit; commit not blocked)."

CONTRAST — same binary, same hook, a LOCATION doctype (milestone-record):
  docs/milestone-records/leftover-wave.md -> …/zz-oob.md | staged: R … | rc=1
  "jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity
   tracking; use `jigc rename` instead (commit blocked)."

AND THE ASYMMETRY AT ITS SOURCE (jigc validate, one rig per family):
  location : "…is missing; docs/milestone-records/zz-oob.md has the same content hash — likely renamed
              via `git mv`"   route: … or revert the move: `git -C <abs> mv <new> <old>`
  placement: "tracked managed doc vision:vision (VISION.md) is missing"
              route: restore VISION.md, or … `jigc unmanage VISION.md`     <- no `mv` pair, so the
                     hook's `mv`-token predicate can never be satisfied
```

Tier 3: the file is on disk, and the placement rename is still caught by `validate`
(`reconciliation.rename` **and** `schema-conformance.home-vacated`, exit non-zero).

### F-3 · N-3 re-driven — `milestone provision` acks *"at base &lt;pin&gt;"* over a worktree it reused at a different commit, and the boundary's refusal repeats the false claim

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       jigc milestone create "Reuse wave"     -> "minted milestone:reuse-wave (shared base 621706f)"
       jigc milestone add-task reuse-wave "ru one"
       printf 'x\n' > extra.txt; git add extra.txt; git -c core.hooksPath=/dev/null commit -m "chore: advance"
       git worktree add -q --detach "$REPO/.jigc/worktrees/ru-one"      (NOT provisioned by jigc)
BEFORE: milestone pin 621706f  ·  the parked worktree's HEAD 75ff78b
argv : jigc milestone provision reuse-wave    -> exit 0
  "provisioned 1 worktree(s) for milestone:reuse-wave at base 621706f (ru-one)"
AFTER: git -C .jigc/worktrees/ru-one rev-parse --short HEAD -> 75ff78b     <- NOT 621706f
then : jigc milestone finalize reuse-wave     -> exit 3
  blocking · finalize.base-mismatch — … the sub-task worktrees were cut from `621706f…`, so combining
    them onto HEAD cannot be proven sound
```

Two law-1 claims the binary cannot support. Tier 3: the boundary blocks, nothing lands, no byte dies.

### F-4 · N-4 re-driven — **confirmed in part, tier corrected 2 → 3**

**What reproduces.** A promoting `task finalize` run from an ordinary linked worktree commits onto
that worktree's branch and re-baselines the **shared** `.jigc/` store, after which the main
checkout's store sweep reports an out-of-band edit that never happened:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       git -C $REPO worktree add -q $RIG/lw -b lwbr ; cd $RIG/lw
       jigc start "record the linked decision" --workflow decided-task
       jigc doc add-item decisions-log#entries --task … --title "Linked branch decision"
         -> "(copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)"
       the entry's `why` slot + the commit doc authored
BEFORE: md5 $REPO/docs/decisions-log.md == md5 $RIG/lw/docs/decisions-log.md == 41fd3323a5b7566764cb350ceeaa6130
        main HEAD f3380ea [main] · lw HEAD f3380ea [lwbr]
argv : jigc task finalize record-the-linked-decision      -> exit 0
AFTER: main HEAD f3380ea [main] (UNMOVED) · lw HEAD d4cae97 [lwbr]
       grep -c 'Linked branch decision' $REPO/docs/decisions-log.md  -> 0
       grep -c 'Linked branch decision' $RIG/lw/docs/decisions-log.md -> 1
then : (cd $REPO && jigc validate)  -> exit 0 (store scope), carrying:
  blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/decisions-log.md`
    differs from the recorded state
    at: docs/decisions-log.md
    route: review the out-of-band edit to `docs/decisions-log.md` and re-author it through the owning workflow
```

**What does not reproduce — the tier-2 leg.** The driver's escalation is *"the main checkout's next
`task finalize` is now gated by that finding, and the emitted route does not clear it"*, and the
driver drove the sweep but **not** that door. Driven here **twice**, on two fresh rigs, with a real
code diff so the task is not empty:

```
continuing the same rig, in the MAIN checkout:
       jigc start "main real work" --workflow quick-fix ; commit doc authored
       printf 'console.log(1)\n' > newcode.js ; git add newcode.js
argv : jigc task validate main-real-work   -> exit 0
         advisory · reconciliation.absorb — external edit absorbed: `docs/decisions-log.md`
         route: no action needed — the external edit was absorbed into the baseline
       jigc task finalize main-real-work   -> exit 0
         finalized b32a23c — feat(probe): main real work
```

The committing door **absorbs** the drift and lands. The store sweep's *"gates at finalize"* label is
scope-relative, and the task-scope reconciler resolves this cell as a conformant non-conflicting
external edit — which is the designed behaviour
(`design/reconciliation.md`: *conformant non-conflicts absorbed*).

**And one supporting clause is falsified.** The driver writes *"what no surface declares is that a
**commit target** and a **baseline home** can be two different checkouts at the same door."* The
rc.19 finalize ack declares exactly the commit-target half, in the same run:

```
  finalized d4cae97 — docs(decisions): record the linked decision
    promoted docs/decisions-log.md
    1 file committed
    committed in the linked worktree at `/private/var/folders/…/lw` on branch `lwbr` — not in the
    main checkout jigc's workbench binds to
```

**So the row survives as tier 3, not tier 2**, by the charter's own predicate: a surface (the store
sweep) says something untrue about a cause — there was no out-of-band edit; jigc wrote that baseline
itself from another checkout — but nothing is destroyed, no committing door is blocked, and the
route an operator actually meets at the door is *no action needed*. The residual worth carrying is
the **diagnosis**, not the dead end.

---

## 3 · Demotions — driver rows marked driven that carry no repro block

Per the reconciliation brief: *a row marked driven that carries no repro block is not driven.* Five
such rows, all demoted in place above. Four were then re-driven by the reconciler and hold; one was
driven by neither pass and stays demoted.

| driver row | why demoted | re-driven? |
|---|---|---|
| **D-1** · §3.2 — `BreachSite::Here` from inside the breaching worktree | a table, no repro block anywhere in the file | **yes** — see below; holds |
| **D-2** · §3.9 **C1-01** — `finalize.carried-staged`'s aimed route | table row, no block | **yes** — see below; holds, incl. the index-aiming leg |
| **D-3** · §3.9 **C1-14 / C3-01** — the `Spawn:` line's runnability | table row, no block (§3.10 restates it, also unblocked) | **yes** — see below; holds, plain and spaced |
| **D-4** · §3.9 — the seventh `discover_repo_root` copy (`describe` md5) | table row, no block | **yes** — CX-20; holds, same md5 |
| **D-5** · §3.10 — `git_at` in `home-vacated` / `carried-staged` / `migrate.source-untracked` | table row, no block | **partly** — `carried-staged` (D-2) and `home-vacated` (F-2's validate block: `git -C <abs> restore --source=HEAD --staged --worktree -- VISION.md`) re-driven; **`migrate.source-untracked` driven by neither pass — stays demoted** |

§3.4 is **not** demoted: it states no cell of its own and delegates explicitly to §3.1's blocks.
§3.9's **C2-09** is **not** demoted: it carries no block of its own, but F-4's block is its repro.

**D-1 re-driven** (same rig as CX-8, the third cwd):

```
cwd=(c) $REPO/.jigc/worktrees/cw-area   jigc milestone finalize cwd-wave   -> exit 1
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
    committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
```

Un-aimed, and correct: the caller **is** standing in that checkout. The exit differs from the
boundary gate's 3 by site, as the driver recorded.

**D-2 re-driven**, both arms — the second is the one that makes the row load-bearing, because the
carried path lives in a *different index* from the one the operator is standing in:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       git -C $REPO worktree add -q $RIG/lw -b lwbr
arm A: printf 'carried\n' > carried.txt; git -C $REPO add carried.txt     <- staged BEFORE the mint
       jigc start "carry probe" --workflow quick-fix ; commit doc authored
       (cd $REPO/docs/deep && jigc task finalize carry-probe)             -> exit 3
  blocking · finalize.carried-staged — `carried.txt` was already staged before this task existed …
    route: unstage it (`git -C <abs>/repo restore --staged -- carried.txt`) …
  run verbatim: from $REPO rc=0 · from docs/deep rc=0 · from $RIG/lw rc=0 · from / rc=0
arm B: the same act with the plant and the task inside the LINKED WORKTREE:
       route: `git -C <abs>/lw restore --staged -- lwcarried.txt`          <- aimed at lw's index
  run verbatim FROM $REPO (the other checkout) -> rc=0, and `git -C $RIG/lw diff --cached` is empty
```

**D-3 re-driven**, plain and spaced:

```
argv : jigc milestone execute <milestone>
plain root : Spawn: `cd /private/var/folders/…/repo/.jigc/worktrees/sw-one && jigc workflow sub-task --task sw-one`
             run verbatim from $REPO rc=0 · from $REPO/docs/deep rc=0 · from / rc=0
spaced root: Spawn: `cd '/private/var/folders/…/jigc space.EVQQFY/…/repo/.jigc/worktrees/sp-area' && jigc workflow sub-task --task sp-area`
             run verbatim from $REPO rc=0 · from $REPO/docs/deep rc=0 · from / rc=0
```

---

## 4 · Doors covered

Every clap leaf that is the door of **at least one driven row** in this reconciled file
(`VERB_KINDS` spelling) — **25**:

`setup` · `uninstall` · `migrate-corpus` · `rename` · `relocate` · `config set` · `config get` ·
`validate` · `describe` · `start` · `workflow` · `doc show` · `doc list` · `task list` ·
`task finalize` · `task discard` · `task validate` · `milestone create` · `milestone add-task` ·
`milestone add-from-spec` · `milestone finalize` · `milestone discard` · `milestone provision` ·
`milestone join` · `milestone execute`

That is the driver's 21 plus four the reconciler keyed a verdict on: `doc show`, `doc list`,
`task list` and `config get` are the `Neither` controls of CX-13 — their verdict *is* that they
proceed under a posture that refuses an acting leaf in the same repository.

**All 12 acting `BEHALF_DOORS` rows are among them** (10 commit-on-behalf: `setup` ·
`migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` ·
`milestone add-task` · `milestone add-from-spec` · `milestone finalize` · `milestone discard`;
2 move-on-behalf: `relocate` · `config set`). Leaves used only to *build* fixtures
(`doc create`, `doc add-item`, `doc set-field`, `doc set-slot`, `doc schema`) are deliberately not
claimed: no verdict row is keyed on them.

---

## 5 · What the reconciliation did not settle

1. **CX-7b** — the displacement `git rm --cached` seam's immediate re-probe: needs a foreign-squatter
   displacement state the rig does not build **and** a racer inside that seam's window. OPEN LEAD.
2. **CX-17** — `DedicatedWorktree`'s unforgeability: a compile-time visibility fact, not a drivable
   state. OPEN LEAD.
3. **CX-19** — *the M52 registries add no axis-2 bypass*: a negative over source structure with no
   named state to reach; its one drivable projection is already a driven driver row. OPEN LEAD.
4. **D-5's third surface** — `migrate.source-untracked`'s `git_at` span: driven by neither pass.
   Stays demoted.
5. **`(2, DEFECT C)` at the milestone boundary's seam** — the reconciler raced the `task finalize`
   seam only, as the driver did. The boundary's own commit seam is unraced by both passes.
6. **A genuinely concurrent racer** on any rollback population — out of this axis's scope, and a
   declared bound in M52's own record. The shim used above is a deterministic injection at a known
   git call, not a concurrent process.
