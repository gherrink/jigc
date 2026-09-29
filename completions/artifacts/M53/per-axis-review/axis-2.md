<!-- M53 PARTIAL per-axis review · axis 2 · RECONCILED (Opus driver table + reconciliation ledger) — every row driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.17`, repo HEAD `7e98faf1`, 2026-09-22. -->

<!-- M53 PARTIAL per-axis review (axes 2 · 3 · 5) — axis 2 · posture — RECONCILED (driver × Codex source pass), 2026-09-22 -->

> **Reconciler banner.** Sections 1–9 below are the **Opus driver table, unchanged**. No row was
> demoted: §10.3 records the demotion check and what was re-driven to settle it. The reconciliation
> ledger, the reconciled doors-covered list, and the reconciler's own method and bounds are §10–§12.
> Every verdict in §10 was reached by **driving `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.17`**
> in throwaway `dev/jigc-rig` roots; no verdict rests on a source read alone.

---

<!-- M53 PARTIAL per-axis review (axes 2 · 3 · 5) — axis 2 · posture — the OPUS DRIVER · driven on the installed `jigc 1.0.0-rc.17`, 2026-09-22 -->

# M53 partial per-axis review — AXIS 2 · posture — the OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.17`**, asserted before anything
else (exit 0). This is the **RELEASE** posture: the `Route::mechanical` argv fence is
`#[cfg(debug_assertions)]` and does not exist here, so every refusal below is the shipped one.

**The fixed binary.** M53's seven completion-audit fixes (`c9fc0d41` `5672e32f` `e15b64e3`
`e9757c94` `5badfbda` `5a4d12d6` `b8d3f7bb`) are all in this build. Two touch this axis and are
driven as cells rather than assumed: **finding 4 + finding 7** (`b8d3f7bb` — the abandon-route
qualifier, one clause per *fate*, the `SHIPPED_ROUTE_LINES` pins re-blessed) and **D4 itself**
(the tenth `InProgress` member). M53's **declared bounds are what I grade against, not re-find**:
the stderr-only advisory at `milestone finalize`, the 21-render `DEBUG_REMAINDER`, the residual
cleared by hand, `reseed_sub_task_areas`' `.exists()` skip. Where a bound of a *neighbouring* wave
covers something I hit, I say so and do not report it (`finalize.render-io`'s host path — §6).

**Fixtures.** `dev/jigc-rig` throughout, two-step eval, every root from `mktemp -d`; there is no
teardown and none is needed — **no `rm -rf` on a variable path appears anywhere in this review.**
Two forms:

* `dev/jigc-rig <state> [--start decided-task "axis intent"] --git-state <member>` for the
  standalone posture cells, and the standalone `dev/jigc-rig --git-state unborn`;
* a wrapper (`driver/build.sh`, written for this review) that builds `committed-singletons` with
  `--start decided-task "axis intent"`, then **creates a milestone and one sub-task through the
  binary** (both commit, so they must precede the posture), then induces the git state with the
  same command sequence `dev/jigc-rig --print-only` emits. The wrapper exists only because the rig
  enters its git state **last** and the milestone doors need a milestone committed before the
  breach. `driver/doors.sh` is the 12-acting-door sweep.

Nothing was written into `.jigc/` by hand; every fixture state was reached by running the command
a user runs. Exit codes measured **bare**, never through a pipe (one early aggregate was measured
through `| sed` and read as the *sed's* 0 — caught and re-measured bare; the corrected figures are
the ones below).

**Instrument note honoured.** This harness's `grep` is a shell function honouring `.gitignore`.
Every claim below about bytes under `.jigc/` (worktree admin dirs, `MERGE_MSG` per worktree,
`invocations.jsonl`) is made with `ls`/`cat`/`head` on the exact path, or with a **before-control**
printed from the same expression that prints the after — never with a bare `grep -rl` under a
gitignored tree.

---

## 1 · The door set, read from the code (counts stated)

| registry | file | count I read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1834` | **47** leaves (35 `Write` · 12 `Read`) |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2003` | **47** rows — the total classification, ⇔-fenced against the clap tree |
| ↳ `ActsOnBehalf::CommitsOnBehalf` | | **10** — `setup` · `migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone finalize` · `milestone discard` |
| ↳ `ActsOnBehalf::MovesOnBehalf` | | **2** — `relocate` · `config set` |
| ↳ `ActsOnBehalf::Neither` | | **35** |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **10** rows over **9** leaves (`milestone finalize` × 2 commit models) |
| `PostureMember::ALL` | `crates/cli/src/repo.rs:138` | **3** |
| **`InProgress::ALL`** | `crates/cli/src/repo.rs:272` | **10** — M53 D4's widening from M52's 9 (`UncommittedCherryPick` inserted after every marker-keyed member and **before** `UnmergedIndex`) |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3338` | **6** (read for the cross with this axis, not as this axis's set) |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs` | **25** rows (read for the cross; the residual cell is axes 3/5) |
| `MINT_DOORS` | `crates/engine/src/state.rs:1510` | **5** (3 prose + 2 `Snapshot::Exempt`) |
| `dev/jigc-rig --list-git-states` | | **17** — M53 added the four `uncommitted-pick*` cells |

**Axis 2's door set = the 12 acting rows**, derived and not taken from the design doc:
`COMMITTING_DOORS ⊆ commit-on-behalf` holds by inspection — its 9 leaves are all
`CommitsOnBehalf`; the tenth commit-on-behalf leaf is `setup`, which commits with `--no-verify`
and so carries no rejection identity and no `COMMITTING_DOORS` row.

## 2 · The cell set

`{clean control · HEAD detached · HEAD unborn · the ten `InProgress` members reached through
fifteen git states · the four new `uncommitted-pick*` cells · the no-override cross · the
`task validate` preview · the `--format json` arm · the invocation log · the seam re-probe ·
the dedicated-worktree cross · the `Neither` class as control · `PRE_DISPATCH_FAULTS` precedence ·
the `GIT_DIR` redirect (declared out) · a **false-positive hunt** over states git leaves that
write `MERGE_MSG` or unmerged entries without being an uncommitted pick · the **abandon-command
effect** on unrelated staged work, per member, in both directions}`

---

## 3 · The (door, cell) table

Exit / code / route are identical across the doors within a cell unless the table says otherwise,
so repro blocks are one per cell. **~190 `(door, cell)` drives are recorded below**; §6 states what
was not driven and why.

### 3.1 · Clean control (`HEAD → refs/heads/main`, nothing in progress)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| all 10 commit-on-behalf | per door's own business | **no `repo.*` code at any door** | per door | `grep 'blocking · repo\.'` on the combined stream → empty | matches contract |
| both movers | 1 / 0 | none / own business | — | same | matches contract |

Driven outcomes, byte-for-byte as M52 recorded them on rc.16 — **no regression**:
`setup` 0 · `migrate-corpus` 0 · `rename` 1 (bare) · `task discard` 1 `task-discard.staged-prose` ·
`task finalize` 3 `schema-conformance.field-value-conformant` · `milestone create` 0 ·
`milestone add-task` 0 · `milestone add-from-spec` 1 `store.not-found` · `milestone finalize` 3
`milestone.zero-contribution` · `milestone discard` 0 · `relocate` 1 `relocate.frozen-doctype` ·
`config set` 0.

```
setup: env=$(driver/build.sh none) || exit; eval "$env"
argv : driver/doors.sh axis-intent axis-milestone
```

### 3.2 · HEAD detached (`git switch --detach HEAD`)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| all 10 commit-on-behalf | 1 | `repo.head-detached` | **Human** | ``route: re-attach HEAD with `git switch <branch>`, then re-run this command`` | matches contract |
| `relocate` (mover) | 1 | `relocate.frozen-doctype` — proceeds past the posture guard | Mechanical | no `repo.` token | matches contract (movers refuse OIP only) |
| `config set` (mover) | 0 | none — acts | — | knob written | matches contract |

### 3.3 · HEAD unborn — each door in its **own** fresh unborn rig

`setup` *births* HEAD, so a shared rig would clear the cell for every door after it.

| door | exit | code | commits | verdict |
|---|---|---|---|---|
| `setup` | **0** | none | **0 → 1** | matches contract — the stated `Exempt(HeadUnborn)` row is real, and visible as a commit count moving |
| `migrate-corpus` · `rename` · `task finalize` · `milestone create` | 1 | `repo.head-unborn`, **Human** — ``land the repository's first commit with `git commit`, then re-run this command`` | 0 → 0 | matches contract |
| `relocate` (mover) | 1 | `relocate.frozen-doctype` — proceeds past the guard | 0 → 0 | matches contract. *Observation:* M52 recorded `store.unknown-type` here on rc.16; the code changed with `relocate`'s own refusal ordering, not with the posture family. Not a posture cell and not graded |
| `config set` (mover) | 0 | none — acts | 0 → 0 | matches contract |

```
setup: rig=$(dev/jigc-rig --git-state unborn --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
observed: setup 0 (commits 0→1) · migrate-corpus 1 repo.head-unborn · rename 1 repo.head-unborn ·
          milestone create 1 repo.head-unborn · task finalize 1 repo.head-unborn ·
          relocate 1 relocate.frozen-doctype · config set 0
```

### 3.4 · The ten `InProgress::ALL` members × 12 doors — and the route run verbatim

`merge` was driven at all **12** doors (exit 1, `repo.operation-in-progress`, one `route:` line
each). Every member below was then driven at a commit door, its **abandoning command lifted out of
the emitted bytes and run in the repository that printed it**, and the door re-asked.

| git state | member answered | noun rendered | abandon command | run → | door after |
|---|---|---|---|---|---|
| `merge` | `Merge` | *a merge is in progress* | `git merge --abort` | 0 | 0 |
| `squash-merge` | `SquashMerge` | *a squash merge is staged and not committed* | `git reset --merge` | 0 | 0 |
| `rebase-merge` | `Rebase` | *a rebase is in progress* | `git rebase --abort` | 0 | 0 |
| `rebase-apply` | `Rebase` | *a rebase is in progress* | `git rebase --abort` | 0 | 0 |
| `am` | `Am` | *a `git am` is in progress* | `git am --abort` | 0 | 0 |
| `cherry-pick` | `CherryPick` | *a cherry-pick is in progress* | `git cherry-pick --abort` | 0 | 0 |
| `sequencer` | `CherryPick` (marker live) | *a cherry-pick is in progress* | `git cherry-pick --abort` | 0 | 0 |
| `dangling-sequencer` | `Sequencer` | *a cherry-pick or revert left a queue of commits in `sequencer/`* | `git cherry-pick --quit` | 0 | 0 |
| `revert` | `Revert` | *a revert is in progress* | `git revert --abort` | 0 | 0 |
| `bisect` | `Bisect` | *a bisect is in progress* | `git bisect reset` | 0 | 0 |
| **`uncommitted-pick`** | **`UncommittedCherryPick`** | ***an uncommitted cherry-pick is in progress*** | **`git reset`** | **0** | **0** |
| **`uncommitted-pick-range`** | **`UncommittedCherryPick`** | same | `git reset` | 0 | 0 |
| **`uncommitted-pick-conflicted`** | **`UncommittedCherryPick`** | same | `git reset` | 0 | 0 |
| **`uncommitted-pick-resolved`** | **`UncommittedCherryPick`** | same | `git reset` | 0 | 0 |
| `unmerged-index` | `UnmergedIndex` | *a conflict left unmerged paths in the index* | `git reset --merge` | 0 | 0 |

**Verdict: matches contract at every cell.** One code, exactly one `route:` line, a `Human` route
naming a command git accepts in that state, and the state is gone after the route runs.

### 3.5 · **M53 D4's member, driven at all 12 acting doors, with the state measured before and after**

This is the cell the exit rule points at: M52's tier-1 `(2, DEFECT A)`.

```
setup: env=$(driver/build.sh uncommitted-pick) || exit; eval "$env"
       # committed-singletons + `--start decided-task "axis intent"` + a milestone and a
       # sub-task created THROUGH THE BINARY, then:
       #   git checkout -b posture-clean; <new file>; commit 'the picked commit'
       #   git checkout main; git cherry-pick -n posture-clean
state : markers = MERGE_MSG (and AUTO_MERGE, which discriminates nothing);
        git ls-files -u -> 0;  head -1 .git/MERGE_MSG -> "posture fixture: the picked commit"
        git diff --cached --name-status -> A  posture-other.txt

argv  : driver/doors.sh axis-intent axis-milestone     (the 12 acting BEHALF_DOORS rows)

observed — ALL TWELVE:
  setup · migrate-corpus · rename · task discard · task finalize · milestone create ·
  milestone add-task · milestone add-from-spec · milestone finalize · milestone discard ·
  relocate · config set
    -> exit=1, blocking · repo.operation-in-progress
       — an uncommitted cherry-pick is in progress — the repository is not in a committable state
       route: conclude it with `git commit` (which uses the pick's own message, once any
       conflicts are resolved), or abandon it with `git reset` (which unstages everything —
       the picked changes and anything else you had staged — keeping all of it in your
       working tree), then re-run this command

after the full 12-door sweep:
  HEAD            7b5988f7… -> 7b5988f7…      (unmoved)
  .git/MERGE_MSG  md5 7ff8a3a3… -> 7ff8a3a3…  (byte-identical)
  index           md5 b228847b… -> b228847b…  (byte-identical)
```

### 3.6 · **The route's two arms, each claim driven against the state it claims about**

`b8d3f7bb` and D4/§10 both make *claims about what a command does*, which a source read cannot
settle. Each was run.

| arm | cell | driven | verdict |
|---|---|---|---|
| **conclude** `git commit` *(which uses the pick's own message, once any conflicts are resolved)* | clean pick | `GIT_EDITOR=true git commit` → exit 0; `git log -1 --format=%s` → **`posture fixture: the picked commit`** — the picked commit's own message, as claimed; `MERGE_MSG` gone; door after → exit 0 | matches contract |
| | conflicted pick | `git commit` → **exit 128**, *"Committing is not possible because you have unmerged files"* — which is what the qualifier's *"once any conflicts are resolved"* is for | matches contract |
| | resolved pick (`git add`-ed) | exit 0, subject `posture fixture: theirs`, door after → 0 | matches contract |
| **abandon** `git reset` *(which unstages everything … keeping all of it in your working tree)* | clean pick **+ one unrelated file staged beforehand** | `git reset` → 0; `MERGE_MSG` **gone**; `posture-other.txt` **and** `unrelated.txt` both still on disk, both `??`; door after → exit 0 | matches contract — both halves of the clause |
| | conflicted pick + unrelated staged | `git reset` → 0; unmerged 3 → 0; **conflict markers still in `posture-shared.txt`** (2 marker lines); `unrelated.txt` still on disk | matches contract |

### 3.7 · **The abandon qualifier over all ten members — the audit's own finding-4/7 fix, driven in both directions**

One unrelated **new** file staged *and* one staged modification to a tracked file, in every state,
before the emitted abandon command ran.

| member (state) | clause rendered | new file after | staged modification after | verdict |
|---|---|---|---|---|
| `Merge` · `Rebase` (both backends) · `Am` · `CherryPick` · `Revert` · `SquashMerge` · `UnmergedIndex` | *" (which also discards anything else you had staged, from the index and from your working tree)"* | **GONE from index and worktree** | **reverted** | matches contract — the clause is true at all seven |
| `UncommittedCherryPick` | *" (which unstages everything — the picked changes and anything else you had staged — keeping all of it in your working tree)"* | present, unstaged | present, unstaged | matches contract |
| `Sequencer` (`dangling-sequencer`) · `Bisect` | **empty** | **present, still staged** | **still staged** | matches contract — the empty clause is honest at exactly the two members the struck sentence was true of |

```
setup: for st in merge squash-merge rebase-merge rebase-apply am cherry-pick sequencer \
                 dangling-sequencer revert unmerged-index bisect; do
         rig=$(dev/jigc-rig committed-singletons --git-state $st --binary …) || continue; eval "$rig"
argv : jigc milestone create Zz1           # lift the route
       printf 'unrelated\n' > zz-unrelated.txt; git add zz-unrelated.txt
       printf 'mod\n' >> README.md;        git add README.md
       <the emitted abandon command, verbatim>
observed (excerpt):
  merge              abandon=git merge --abort       rc=0  new-file:GONE     index:[]  README-mod:0
  dangling-sequencer abandon=git cherry-pick --quit  rc=0  new-file:present  index:[README.md,zz-unrelated.txt]  README-mod:1
  bisect             abandon=git bisect reset        rc=0  new-file:present  index:[README.md,zz-unrelated.txt]  README-mod:1
```

### 3.8 · The pinned surfaces of a posture refusal, under the new member

| surface | driven | verdict |
|---|---|---|
| `--format json`, commit door | **stdout 0 bytes**; stderr one document, keys **`['error']`**, exit 1 | matches contract — `design/validation.md` declares all three `repo.*` codes *"neither — un-keyed"* and `ENVELOPE_OWED_CODES` carries no `repo.*` member, so the flattened reject arm is the declared one |
| route count | exactly one `route:` line per refusal at every door, every cell | matches contract |
| invocation log | `{"argv":["milestone","create","Zz1"],"exit_code":1,"finding_codes":["repo.operation-in-progress"],…,"binary_version":"1.0.0-rc.17","error_code":null}`, likewise `task finalize` | matches contract |
| `jigc task validate` preview | exit 1, `repo.operation-in-progress` — the committing door's finding, route and exit code | matches contract (M52 D2.6) |

*Method note, recorded because it bit:* `jigc config set invocation-log true` is a **mover** and
therefore refuses under the breach — the log has to be enabled **before** the state is induced, or
the cell silently measures a repo with no log at all.

### 3.9 · The no-override cross — the refusal carries no consent flag (all under `uncommitted-pick`)

| door + flag | exit | code | verdict |
|---|---|---|---|
| `setup --force` | 1 | `repo.operation-in-progress` | matches contract |
| `task finalize --carry-staged` | 1 | same | matches contract |
| `task finalize --dry-run` | 1 | same | matches contract |
| `task finalize --approve` | 1 | same | matches contract |
| `task discard --force` | 1 | same | matches contract |
| `milestone discard --force` | 1 | same | matches contract |
| `migrate-corpus --dry-run` | 1 | same | matches contract |

**M52 `(2, DEFECT A)`'s *second ordering* re-driven** — the pick pre-dates the task, so on rc.16
the carryover gate fired at exit 3 and `--carry-staged` then concluded the pick at exit 0:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       git checkout -b cpb; printf 'PAYLOAD\n' > g2.txt; git add g2.txt
       git commit -m 'the users cherry-pick target'; P=$(git rev-parse HEAD); git checkout main
argv : git cherry-pick -n "$P"                                                     -> 0
       jigc start "carry probe" --workflow decided-task                            -> 0
       jigc task finalize carry-probe                 -> exit 1  repo.operation-in-progress
       jigc task finalize carry-probe --carry-staged  -> exit 1  repo.operation-in-progress
observed: .git/MERGE_MSG still reads "the users cherry-pick target";
          g2.txt is NOT in HEAD ("exists on disk, but not in 'HEAD'")
```

The posture refusal now pre-empts the carryover gate on both orderings.

### 3.10 · The seam re-probe — `SeamSubject::verify` immediately before the act, with the **new** member raced in

A deterministic `git` shim on `PATH` that fires one real operation once on a named call (with
`GIT_INDEX_FILE`/`GIT_DIR`/`GIT_WORK_TREE` unset so it acts on the main checkout), then `exec`s the
real git at an absolute path.

| seam | racer fired after | exit | code | state after | verdict |
|---|---|---|---|---|---|
| `task finalize` → commit seam | the first `git add -- …` (after the stage, before the commit), firing **`git cherry-pick -n <target>`** | 1 | `repo.operation-in-progress` — *an uncommitted cherry-pick* | HEAD unmoved; `.git/MERGE_MSG` = *"the raced pick message"*, intact | **matches contract — M53's member is enforced at the seam, not only at the door** |
| `task finalize` → commit seam | the same, firing `git bisect start` | 1 | `repo.operation-in-progress` — *a bisect* | HEAD unmoved; task still open; `work.txt` still staged; jigc's own `.jigc/*` staging rolled back | matches contract on the *guard*; see **§4 · M52 DEFECT C** for what it does **not** say |

### 3.11 · The dedicated-worktree cross (typed `DedicatedWorktree`, never sniffed)

| cell | doors | exit | code | verdict |
|---|---|---|---|---|
| inside `$REPO/.jigc/worktrees/<sub>` (git's own `--detach`), clean | `milestone create` · `task finalize` | 0 / own business | **no `repo.head-detached`** | matches contract — the exemption is real |
| the same worktree carrying a **clean uncommitted cherry-pick**, doors run **inside** it | `milestone create` · `task finalize <sub>` | 1 | `repo.operation-in-progress` — *an uncommitted cherry-pick* | matches contract — exempt from `HeadDetached` **and that member only**, now including M53's member |
| the **main** checkout while the linked worktree holds the pick | `milestone create` | **0** | none | matches contract — per-worktree git-dir resolution (`.git/worktrees/<n>/MERGE_MSG` is where git wrote it; the main `.git` has none) |
| the whole fan-out, clean: `create` → `add-task` → `provision` → work → `join` → `milestone finalize` | | **0** | none | matches contract — the zero-false-fire control: an ordinary fan-out finalizes clean, 2 files committed |
| **the main checkout clean, a provisioned sub-task worktree holding an un-concluded operation, then `jigc milestone finalize`** | `milestone finalize` | **0** | **none** | **DEFECT 1 — see §4** |

### 3.12 · The `Neither` class — **all 35 of 35** driven as controls, under `uncommitted-pick`

`start` · `workflow` · `uninstall` · `upgrade` · `ingest` · `migrate` · `unmanage` · `describe` ·
`validate` · `doc create` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc author` · `doc show` · `doc schema` ·
`doc list` · `task list` · `task diff` · `task validate` · `task bind` · `config insert-step` ·
`config replace-step` · `config remove-step` · `config fill` · `config fork` · `config get` ·
`config list` · `milestone list-tasks` · `milestone provision` · `milestone execute` ·
`milestone join`.

**34 of 35 silent** (`repo.*` hits 0, clap-usage exits 0 — every argv was checked to parse, and the
two that first exited 2 were corrected and re-driven: `start` bare, and
`task bind <ROLE> <ADDR> <ID>`). The one that answers is **`task validate`** — exit 1,
`repo.operation-in-progress` — by the stated preview design, which the same file records at §3.8.
`.git/MERGE_MSG` was **present** and HEAD unmoved after the whole sweep.

`uninstall --force` was driven in **its own rig** (it removes the install and would clear the cell
for everything after it): **exit 0**, `repo.*` hits 0, `.git/MERGE_MSG` untouched, jigc's five
tracked files left ` D` in the worktree. It neither commits nor `git mv`s, so the `Neither`
classification is consistent with the rule as `BEHALF_DOORS` states it — the M52 observation,
unchanged. `milestone provision` likewise: **exit 0** under the breach, worktree cut, `MERGE_MSG`
untouched.

### 3.13 · Precedence against `PRE_DISPATCH_FAULTS`

| cell | door class | exit | answered by | verdict |
|---|---|---|---|---|
| `uncommitted-pick` + a malformed `.jigc/config/packs.yaml` (a bare scalar) | commit (`milestone create`) | 1 | the **posture** refusal | matches contract — the guard sits at dispatch top, ahead of pack load |
| same | mover (`config set`) | 1 | the posture refusal | matches contract |
| same | `Neither` (`doc list`) | 1 | the **pack** fault (*"is not a valid pack-set list: invalid type: string \"just-a-scalar\""*) | matches contract |
| unreadable cwd (`G=$(mktemp -d); cd "$G"; rmdir "$G"`) + `uncommitted-pick` | commit | 1 | the **cwd** fault (*"cannot determine the current directory: No such file or directory (os error 2)"*) | matches contract — `cwd_or_refusal` runs before `posture_refusal_in` |

### 3.14 · The **false-positive hunt** — states that write `MERGE_MSG` or unmerged entries and are *not* an uncommitted pick

This is the cell set M53's new member creates the risk in: its predicate is *`MERGE_MSG` minus six
markers*, so any benign state leaving `MERGE_MSG` alone would now be named a cherry-pick.

| the user's command | what git leaves | member answered | verdict |
|---|---|---|---|
| `git stash pop` that **conflicts** | `AUTO_MERGE` only, 3 unmerged, **no `MERGE_MSG`** | `UnmergedIndex` | matches contract — the doc's own cell, unmoved; `git stash list` still holds the entry after the route |
| `git stash apply` **clean** | `AUTO_MERGE` only | **none**, door exit 0 | matches contract — no false positive |
| `git revert -n` **clean** (single commit) | `REVERT_HEAD` + `MERGE_MSG` | `Revert` | matches contract — the member's doc-comment claim (*git writes `REVERT_HEAD` clean or conflicting*) is **true on this git**, so an uncommitted revert never lands on the pick's noun |
| `git revert -n` **conflicted** | `REVERT_HEAD` + `MERGE_MSG`, 3 unmerged | `Revert` | matches contract |
| `git merge --no-commit --no-ff` clean | `MERGE_HEAD` + `MERGE_MSG` | `Merge` | matches contract — the negated conjunct holds |
| `git merge --squash` clean, **then the user's own `git commit`** | nothing | **none**, door exit 0 | matches contract — no false positive |
| `git merge --squash` that **conflicts** | `SQUASH_MSG` + `MERGE_MSG`, 3 unmerged | `SquashMerge` | **M52 `(2, DEFECT B)` — STILL-OPEN**, see §5 |
| `git cherry-pick -n` clean, **then the user's own `git commit`** | nothing | **none**, door exit 0 | matches contract — the state does not survive its own conclusion |

**Zero false positives found.** Every marker-writing sibling is still answered by the member that
owns it, and the two marker-free states (`AUTO_MERGE`-only) stay silent.

### 3.15 · `GIT_DIR` redirect — the **declared-out** row, re-driven with the new member

| cell | driven | verdict |
|---|---|---|
| repo A clean, repo B holding a clean uncommitted pick, `GIT_DIR=$B/.git jigc milestone create` run in A | **exit 0**; A commits 5 → 5, B commits 5 → **6**; **B's `.git/MERGE_MSG` GONE** — B's un-concluded pick concluded by a commit of A's record | **matches the declared bound** (`repo.rs` module header: *"its subject is the path it is handed, and it passes the ambient environment through untouched"*, with a written reopening condition). Recorded as an **amplifier on the declared bound**, not a new defect — the same disposition M52 gave its `MERGE_HEAD` variant. All eight filesystem legs read **A** (clean), so the family cannot see B's state at all |

---

## 4 · Defects

### DEFECT 1 — `jigc milestone finalize` commits a sub-task worktree's **un-concluded operation** at exit 0, under jigc's own subject, and destroys the picked/merged commit's authored message — while the *same* worktree refuses its own `task finalize` one command earlier

**Tier: 1** (the charter's predicate — *exit-0 loss or repository harm through a committing,
destroying or moving door*). `milestone finalize` is a `CommitsOnBehalf` door and a
`COMMITTING_DOORS` leaf.

**Contracts violated.**

* `crates/cli/src/repo.rs`, module header — *"[`posture`] answers one question — is **this
  repository** in a state a door may commit or move in? … The probe is the family's one home **so
  the doors that read it cannot each hand-enumerate a different three**."* The boundary reads the
  probe **for the main checkout** and then commits from a **different** working tree it never asks
  about.
* `design/finalize.md` → *1. Preflight* — *"No in-progress merge/rebase/bisect"* — which the
  boundary is a finalize under.
* `cli.rs` `BEHALF_DOORS` — `milestone finalize` is `CommitsOnBehalf`, and M52's own axis-2 rule is
  that **every** acting door refuses under every member.
* The narration: the ack calls the swallowed payload *"sub-tasks: `<sub>`: 2 code files"*, i.e. the
  sub-task's own work — a law-1 claim that is false of a byte the user's un-concluded pick put
  there.

**Why it survived M53.** D4's subject is *the repository the door was handed*; its acceptance (flow
54 arm 4) iterates `GitState::ALL × BEHALF_DOORS`' acting rows with the state induced in the **main
checkout**. The boundary's act is `git diff --cached --binary` read out of each provisioned
worktree's index and applied off-line — the one committing path whose *source* index is not the
one the guard probed. This is M52's `(2, DEFECT A)` damage shape reappearing on an un-swept axis
(the **subject** axis rather than the **member** axis): the sixth-consecutive-wave signature, one
layer out.

**Repro — the uncommitted-cherry-pick cell (the default `finalize.fan-out.squash` commit model):**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"
       jigc milestone create "Swallow milestone"
       jigc milestone add-task swallow-milestone "swallow sub intent"
       jigc milestone provision swallow-milestone
       W=$REPO/.jigc/worktrees/swallow-sub-intent
       # the sub-task's own legitimate work
       printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt
       # the USER's un-concluded pick, inside that same worktree
       printf 'PICKED PAYLOAD\n' > $W/picked.txt; git -C $W add picked.txt
       git -C $W commit -m 'the users authored pick message'; P=$(git -C $W rev-parse HEAD)
       git -C $W reset --hard HEAD~1; printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt
       git -C $W cherry-pick -n "$P"

BEFORE (measured, not assumed):
  head -1 .git/worktrees/swallow-sub-intent/MERGE_MSG  -> the users authored pick message
  git -C $W diff --cached --name-only                  -> picked.txt  subwork.txt
  (cd $W && jigc task finalize swallow-sub-intent)     -> exit 1  blocking · repo.operation-in-progress
  jigc <anything> in the main checkout                 -> no repo.* code (the main checkout IS clean)

argv : jigc milestone join swallow-milestone            -> exit 0
       jigc milestone finalize swallow-milestone

observed:
  finalized 194735d — Finalize milestone swallow-milestone (1 sub-task)
    modified docs/milestone-records/swallow-milestone.md
    added picked.txt                     <- the USER's un-concluded pick
    added subwork.txt
    3 files committed
    sub-tasks: swallow-sub-intent: 2 code files                                exit=0

AFTER:
  git cat-file -p HEAD:picked.txt                      -> PICKED PAYLOAD
  worktree dir .jigc/worktrees/swallow-sub-intent      -> REMOVED
  .git/worktrees/swallow-sub-intent/MERGE_MSG          -> GONE   (the authored message, destroyed)
```

**The class is the whole family, not the new member.** The identical drive with a `git merge
--no-commit --no-ff` left un-concluded in the sub-task worktree:

```
  BEFORE: .git/worktrees/fm-sub-intent holds MERGE_HEAD + MERGE_MSG; staged = merged.txt subwork.txt
  jigc milestone finalize fm-milestone
    finalized aff8cc4 — Finalize milestone fm-milestone (1 sub-task)
      added merged.txt        <- the user's un-concluded merge payload
      added subwork.txt
      3 files committed                                                         exit=0
  AFTER: MERGE_HEAD GONE · MERGE_MSG GONE
```

**Zero-false-fire control, driven:** the same fan-out with no operation anywhere finalizes clean —
`exit 0`, `2 files committed`, `sub-tasks: clean-sub-intent: 1 code file`. The defect is the
*subject*, not the door.

**Source observation, stated as one rather than driven** (it is a claim about a test file, not
about the binary): `crates/cli/tests/posture_door_axis.rs`'s
`a_dedicated_worktree_is_exempt_from_the_detached_member_and_from_nothing_else` asserts leg (2) —
*a merge left un-concluded **inside** that worktree still refuses* — which is exactly the leg that
**holds** here, driven above. No arm asks what `milestone finalize`, run from the main checkout,
does about the worktree it is about to commit from. That is why the suite is green over this cell.

**Bound on the finding.** The **non-squash** commit model
(`config set finalize.fan-out.squash false`) is **not driven to a landed commit**: two attempts
stopped earlier at `finalize.render-io` (a sub-task's commit doc is provisioned on its first
re-entry, and my fixture kept losing it to the reset used to build the pick). The squash arm is the
shipped default and is where the repro lands; the non-squash arm is recorded as **un-driven**, not
as passing.

---

## 5 · M52 §A rows for this axis — CLOSED / STILL-OPEN

Every §A row [M52's ledger](../../../completions/artifacts/M52/per-axis-review/README.md) carries
for axis 2, re-driven on `1.0.0-rc.17`. **All tiers**, as instructed.

| M52 §A row | tier | verdict on rc.17 | the argv + observation that settles it |
|---|---|---|---|
| **`(2, DEFECT A)`** — a clean `git cherry-pick --no-commit` is a member of no `InProgress::ALL` row, and `jigc task finalize` concludes it at exit 0, destroying the picked commit's authored message | **1** | **CLOSED** | `dev/jigc-rig committed-singletons --git-state uncommitted-pick` (and `-range`, `-conflicted`, `-resolved`): all **four** answer `repo.operation-in-progress — an uncommitted cherry-pick is in progress`; driven at **all 12** acting doors, exit 1 each, and after the full sweep HEAD, `.git/MERGE_MSG` and the index are **byte-identical** (§3.5). **Both orderings** closed: `--carry-staged` refuses first (§3.9), so the second exit-0 path is gone too. The route's two arms were **run** and both claims hold (§3.6) |
| **`(2, DEFECT C)`** — a posture breach raised at the **commit seam** prints no state-truth clause and no copy-runnable re-run, while every other in-transaction failure at the same door prints both | **2** | **STILL-OPEN** — *expected: triaged to the 1.x ledger, not a new finding* | Same rig, same racer as M52. **B (posture raced in):** `blocking · repo.operation-in-progress — a bisect is in progress` + one `route:` line, and **nothing else**; invocation log `"finding_codes":["repo.operation-in-progress"],"error_code":null`. State verified on disk and **unsaid**: HEAD unmoved, `jigc task list` → 1 active task, `work.txt` still `A` in the index, jigc's own `.jigc/*` staging rolled back, `.git/BISECT_LOG` present. **A (the contrast arm, same door, one step apart):** a rejecting `pre-commit` → `` `git commit` was rejected (no commit was made): `` + *"task axis-intent is intact — nothing was committed, your task's staged docs are still in `.jigc/tasks/axis-intent/docs/`, and anything you had `git add`-ed is still in git's index. Fix the hook's complaint, then re-run `jigc task finalize axis-intent`."*, log `"error_code":"finalize.commit-rejected"`. Reproduces byte-for-byte |
| **`(2, DEFECT B)`** — a conflicted `git merge --squash` is answered by `SquashMerge` with a predicate that is false of the state | **3** | **STILL-OPEN** — *expected: triaged to the 1.x ledger, not a new finding* | `git merge --squash cb` (conflicting) → `.git` holds `MERGE_MSG` + `SQUASH_MSG`, `git ls-files -u` → 3; `jigc milestone create CS1` → exit 1, *"a squash merge is staged and not committed"*. `SQUASH_MSG` is a marker, so `SquashMerge` is probed before `UnmergedIndex` and answers. The route is still effective (`git reset --merge` → 0, door then 0). `design/validation.md`'s *a conflict* row and `UnmergedIndex`'s doc-comment still attribute this cell to a member that never sees it |

**And the M51 rows M52 re-drove through this axis, spot-re-driven here so a regression could not
hide:** `D1` (rebase/bisect answer the operation, not the detachment) — **still CLOSED**, both
states answer `repo.operation-in-progress` and the route runs to exit 0; `D2` (`am` vs
`rebase --apply` discriminator) — **still CLOSED**, both directions; `D3b` — not re-driven (see
§6); `codex-1` (the `relocate` seam re-probe) — not re-driven at `relocate` specifically, but the
**same `SeamSubject::verify` mechanism** was driven at the `task finalize` commit seam in both
racer shapes (§3.10) and refuses before the act.

**Roll-up for this axis, M52 → M53:** 3 §A rows → **1 CLOSED (the tier-1 one), 2 STILL-OPEN and
both expected** (tier-2 and tier-3, triaged to 1.x by the exit rule). **Nothing regressed; no closed
row re-opened; zero false positives introduced by the new member.** **1 new finding**, tier 1
(§4).

---

## 6 · What I did NOT drive, and why

1. **The `milestone finalize` non-squash commit model to a landed commit** (DEFECT 1's second
   `COMMITTING_DOORS` row). Attempted twice; both stopped at `finalize.render-io` because the
   sub-task's commit doc is provisioned on first re-entry and my pick-building reset removed it.
   Recorded as **un-driven**, not as passing.
2. **`GIT_DIR` at 11 of the 12 acting doors.** Driven at `milestone create` only. The bound is a
   property of the *probe* and the ambient environment, not of a door, and is declared out with a
   written reopening condition.
3. **The `relocate` / `config set placement-root` seam re-probe** (M51 codex-1's own sites). The
   *mechanism* was driven at the `task finalize` commit seam in two racer shapes; the two
   `relocate.rs` `verify` sites were not re-raced on rc.17. M52 closed them with repros and nothing
   in M53 touches `relocate.rs`'s seam.
4. **`D3b`** (git's own refusal not dressed as a hook rejection) — not re-driven; M53 changed no
   commit-seam classification and the contrast arm of §5 shows the hook arm's own frame intact.
5. **`SeamSubject::verify`'s identity-drift and expected-ref-drift legs.** Not driven; the
   *posture* leg is this axis's and is driven at §3.10.
6. **A genuine concurrent process** rather than a shim racer at any seam. The shim is deterministic
   by design; a real race is not drivable here.
7. **`InProgress` states inside a sub-task worktree other than merge and uncommitted-pick**
   (bisect, rebase, am, revert). Two members establish the DEFECT 1 class at both ends of the
   family's detection styles (marker-keyed and marker-negated); the rest were not driven.
8. **Non-`main` default branch names, submodule worktrees, `core.worktree` redirects, and git
   versions other than the installed `/usr/bin/git`.** Out of the cell set; every marker fact here
   is that git's on-disk contract, which is the family's existing declared deferral.
9. **`finalize.render-io`'s host-absolute `at:` path**, hit incidentally at §4's non-squash
   attempt. **Not reported**: `crates/engine/src/finalize.rs` is row 1 of
   `crates/cli/tests/repo_relative_paths.rs`'s `UNSWEPT_PRODUCERS` (11 sites, `render_io` named
   explicitly), a counted, fenced, declared remainder. Graded against, not re-found.

---

## 7 · What this adds over flow-54 arm 4

Arm 4 iterates **the four new `GitState` variants × `BEHALF_DOORS`' acting rows** — a manufactured
fixture set crossed with a total classification — and asserts that every acting door refuses with
the member's noun, that `.git/MERGE_MSG` and the index are byte-unchanged after the refusal, that
the conclude phrase carries no *"once its conflicts are resolved"* on the clean cells, and that the
abandon argv runs verbatim to exit 0 in all four states.

This axis adds five things the arm structurally cannot:

1. **The subject axis, which the arm has no dimension for.** Arm 4's state is induced in the
   repository the door is run in. **DEFECT 1 lives in the complement**: the main checkout is clean,
   the operation is in a *provisioned sub-task worktree*, and the door that commits that worktree's
   index never asks it. An arm crossing `GitState::ALL` with the door set proves every *member* is
   handled at every *door*; it cannot discover a *second subject*.
2. **The complement of the fixture builder's own state set.** §3.14 drives the states the builder
   does not build — a conflicted `git stash pop`, a clean and a conflicted `git revert -n`, a clean
   `git stash apply`, a squash merge concluded by the user's own commit, a pick concluded by the
   user's own commit — which is where the new member's **false-positive** risk lives. The arm
   iterates the members it has; it cannot show that nothing benign has joined them.
3. **The route's claims, tested against the world rather than against the string.** §3.6 and §3.7
   run every abandon command with unrelated staged work planted first and measure the index **and**
   the working tree afterwards, in both directions — the seven that discard, the one that unstages,
   the two whose empty clause is honest. That is the audit's finding-4/7 fix re-measured
   independently of the fence that shipped with it.
4. **The seam, raced.** §3.10 opens an uncommitted cherry-pick **after** the door's own verdict,
   between the stage and the commit, and shows the re-probe catches M53's member — a window no
   door-level arm reaches.
5. **The composite precedence.** Posture vs `PRE_DISPATCH_FAULTS` (both fault shapes), vs an
   unreadable cwd, vs `setup`'s unborn exemption, vs `RelocateRefusal`, vs the `DESTROYING_DOORS`
   cross, vs the invocation log and the `--format json` reject arm — six registries meeting one
   guard, asserted once each, where the arms own their registries separately.

---

## 8 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row above, `VERB_KINDS` spelling.

**The 12 acting doors** (`BEHALF_DOORS` ▸ `CommitsOnBehalf` ∪ `MovesOnBehalf`): `setup` ·
`migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` ·
`milestone add-task` · `milestone add-from-spec` · `milestone finalize` · `milestone discard` ·
`relocate` · `config set`

**The 35 `Neither` leaves**, driven as controls: `start` · `workflow` · `uninstall` · `upgrade` ·
`ingest` · `migrate` · `unmanage` · `describe` · `validate` · `doc create` · `doc add-item` ·
`doc remove-item` · `doc retitle-item` · `doc rename` · `doc set-field` · `doc set-slot` ·
`doc author` · `doc show` · `doc schema` · `doc list` · `task list` · `task diff` ·
`task validate` · `task bind` · `config insert-step` · `config replace-step` ·
`config remove-step` · `config fill` · `config fork` · `config get` · `config list` ·
`milestone list-tasks` · `milestone provision` · `milestone execute` · `milestone join`

**Total: 47 — `uncovered: none`.**

---

## 9 · Instrument honesty

One fixture-construction error is on the record rather than buried. Building the `GIT_DIR` cell I
parsed the rig's stdout with `sed` instead of eval-ing it; the pattern did not match, `$AREPO` and
`$BREPO` came back **empty**, and the subsequent `git -C "" …` calls ran in
`/Users/maurice/projects/gherrink-jigc` itself — creating a branch `bpick`, a no-op
`cherry-pick -n`, and a stray `.git/MERGE_MSG`. **Nothing was committed** (HEAD stayed `7e98faf1`
on `main`, nothing was staged, the only working-tree entry was the pre-existing untracked
`completions/artifacts/M53/per-axis-review/`). It was restored immediately — `git reset` (nothing
staged, so a no-op except clearing `MERGE_MSG`) and `git branch -D bpick` — and re-verified: branch
`main`, HEAD `7e98faf1`, no markers, no stray branch, `git status --porcelain` showing only that
one pre-existing untracked directory. The cell was then re-driven correctly by reading the paths
back out of a subshell that eval-ed the rig, which is the prescribed form. **The lesson is the rig's
own rule one level up:** *never parse the rig's output — eval it*, because a failed parse is silent
in exactly the way a failed `eval "$(…)"` is.

---

## 10 · Reconciliation ledger

**The rule applied** (`acceptance-design.md` → *The reconciliation rule*): a claim by one pass that
the other cannot reproduce is a **lead**, not a finding. Every Codex claim below was entered as
`lead(codex, …)` and then **driven** — to a repro block (CONFIRMED, origin `codex`) or to a
recorded refutation carrying the falsifying datum. Every driver **defect** was **re-driven by the
reconciler** on the same binary before it was allowed to stand.

Binary for every drive in this section: `/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.17`**
(asserted, exit 0). Release posture. Fixtures: `dev/jigc-rig`, two-step eval, every root from
`mktemp -d`; **no `rm -rf` on a variable path appears anywhere in this reconciliation.**

### 10.1 · Codex claims → verdict

| # | lead(codex, …) | verdict | settled by |
|---|---|---|---|
| C-1 | **M52 DEFECT B remains** — a *conflicted* `git merge --squash` is answered by `SquashMerge` (*"a squash merge is staged and not committed"*) rather than by the extant unmerged-index conflict | **CONFIRMED** (origin `codex`; the driver drove the same cell independently at §3.14/§5 — joint) | §10.1.1 |
| C-2 | **M52 DEFECT C remains** — a posture race detected at the **commit seam** is a `BlockedFinding`, exempt from `CommitFailed` wrapping, so `surface_commit_rejection` attaches neither the state-truth clause, nor the copy-runnable re-run argv, nor the committing-door error identity | **CONFIRMED** (origin `codex`; joint with driver §3.10/§5) | §10.1.2 |
| C-3 | **`(2, DEFECT A)` is CLOSED** — `UncommittedCherryPick` is the tenth `InProgress::ALL` member, ordered after every marker-backed operation and before `UnmergedIndex`; both door and seam guards consume the same probe | **CONFIRMED** | §10.1.3 |
| C-4 | **M51 `D1` is CLOSED** — `posture()` emits the operation before detached/unborn HEAD, so rebase/bisect cannot mask as detachment | **CONFIRMED** | §10.1.4 |
| C-5 | **M51 `D2` is CLOSED** — `rebase-apply/applying` identifies `git am` while bare `rebase-apply` identifies rebase; distinct nouns and distinct abort commands | **CONFIRMED** | §10.1.4 |
| C-6 | **M51 `D3` is CLOSED, including M52's formerly open clean-`-n` cell** | **CONFIRMED** | §10.1.4 |
| C-7 | **M51 `D3b` is CLOSED** — a git refusal is distinguished from a hook rejection via `GIT_HOOK_EXIT`, producing *"failed"*, not *"rejected"* | **CONFIRMED** — and the drive **sharpens C-2**, because the git-failure arm carries the full survivable frame that the posture arm does not | §10.1.5 |
| C-8 | **M51 `codex-1` is CLOSED** — `displace_foreign_squatter` calls `verify(Move)` immediately before `git rm --cached`; the later `git mv` re-probes independently | **OPEN LEAD** | §10.1.6 |
| C-9 | *Completeness read* — `BEHALF_DOORS` is bijective with the clap leaves; acting rows are 10 commit-on-behalf + 2 movers; `COMMITTING_DOORS ⊆ CommitsOnBehalf` asserted; `setup` the sole unborn exemption and sole no-hook addition | **CONFIRMED** on its **driven** half (`setup` exempt, every other door refused); the *asserted-in-source* half is a source fact both passes agree on and neither drove as such | §10.1.7 |
| C-10 | *Completeness read* — *"no production HEAD-changing `git switch` or `git checkout` was found"* and the enumerated production acts (`git commit`, setup commit, `git merge --ff-only`, `git mv`, `git rm --cached`) each carry an immediately preceding `verify` | **REFUTED as a completeness claim** — the enumeration is **incomplete**: the milestone boundary's commit act reads `git diff --cached --binary` out of a *provisioned sub-task worktree's* index and applies it off-line, and **no `verify` probes that worktree**. The falsifying datum is the reconciler's own re-drive of DEFECT 1 (§10.2.1): the main checkout is clean, the worktree holds an un-concluded operation, and `jigc milestone finalize` commits its payload at **exit 0**. The claim is true of each *listed* act and false of the *list* | §10.2.1 |
| C-11 | *Completeness read* — *"I found no `Neither` leaf reaching these commit/move seams and no setup unborn-exemption leak"* | **CONFIRMED** on the unborn half (§10.1.7); the `Neither` half matches the driver's 35-of-35 control sweep (§3.12) and was not independently re-driven by the reconciler | §10.1.7 + driver §3.12 |

#### 10.1.1 · C-1 driven — conflicted `git merge --squash`

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"
       printf 'seed\n' > cs.txt;   git add cs.txt; git commit -m 'cs seed'
       git checkout -b cb; printf 'theirs\n' > cs.txt; git add cs.txt; git commit -m theirs
       git checkout main; printf 'ours\n' > cs.txt; git add cs.txt; git commit -m ours
       git merge --squash cb                      -> rc=1, "CONFLICT (content): Merge conflict in cs.txt"

state: markers  -> MERGE_MSG, SQUASH_MSG
       git ls-files -u | wc -l  -> 3            <- the index IS conflicted

argv : jigc milestone create CS1

observed:
  blocking · repo.operation-in-progress — a squash merge is staged and not committed
    — the repository is not in a committable state
    route: conclude it, or abandon it with `git reset --merge` (which also discards anything else
           you had staged, from the index and from your working tree), then re-run this command
  exit=1

  -> the noun asserts the index is STAGED; `git ls-files -u` says 3 paths are UNMERGED.
     The conclude arm is also command-less here ("conclude it"), and the `git commit` it elides
     would exit 128 on this state.

route still effective: git reset --merge -> 0; unmerged 3 -> 0; door after -> exit 0
```

**Disposition unchanged: tier 3, STILL-OPEN, expected — triaged to the 1.x ledger by the exit
rule, not a new finding.** Both passes now agree on the cell and on the datum.

#### 10.1.2 · C-2 driven — the posture race at the commit seam

The racer is a deterministic `git` shim first on `PATH` that, on the **first `git add` of the run**,
fires one real operation in the main checkout with `GIT_INDEX_FILE`/`GIT_DIR`/`GIT_WORK_TREE`
unset, then `exec`s `/usr/bin/git`. The fire therefore lands **after** `jigc`'s door-top probe and
**after** its staging, and **before** its `git commit`.

```
setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "seam probe" \
             --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       printf 'fn main() {}\n' > work.txt; git add work.txt
       jigc doc set-field commit:seam-probe#header/type --task seam-probe --value feat
       printf 'the seam probe subject\n' | jigc doc set-slot commit:seam-probe#summary \
             --task seam-probe --from-file -
       jigc task validate seam-probe        -> "no findings — the task validates clean"
       jigc task finalize seam-probe --dry-run -> exit 0, "would commit — feat: the seam probe subject"
       PATH=<shim>:$PATH  SHIM_CMD="bisect start"  SHIM_REPO=$REPO  SHIM_STAMP=<fresh>

argv : jigc task finalize seam-probe

observed — the WHOLE surface, nothing elided:
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
    committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  exit=1
  shim stamp: "shim: fired [bisect start] rc=0"      <- the race really fired mid-transaction

state after, measured and UNSAID by that surface:
  HEAD                                  unmoved (72a86f5 docs(changelog): cut the first release)
  jigc task list                        1 active task — seam-probe [decided-task]
  git diff --cached --name-status       A  work.txt          <- still staged
  .jigc/tasks/seam-probe/docs/          commit:seam-probe.md, provenance.json   <- still there
  .git/BISECT_LOG                       present
```

**The contrast, same door, same window, one step apart** — this is what makes it a defect rather
than a design. Two *other* failure causes at the identical seam both print the full frame:

```
ARM A — a rejecting pre-commit hook (fresh rig, task "hook probe"):
  `git commit` was rejected (no commit was made):
  the hook says no

  task hook-probe is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/hook-probe/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize hook-probe`.
  exit=1

ARM A' — a GIT refusal, no hook involved (fresh rig, task "gpg probe";
         git config commit.gpgSign true + gpg.program /nonexistent/gpg-binary):
  `git commit` failed (no commit was made):
  fatal: cannot exec '/nonexistent/gpg-binary': No such file or directory
  error: gpg failed to sign the data:
  fatal: failed to write commit object

  task gpg-probe is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/gpg-probe/docs/`, and anything you had `git add`-ed is still in git's index.
  Resolve the cause above, then re-run `jigc task finalize gpg-probe`.
  exit=1
```

So at one seam there are **three** in-transaction failure causes: a hook rejection and a git
refusal each print the state-truth clause **and** the copy-runnable `jigc task finalize <id>`; the
**posture** race prints neither, and offers only *"re-run this command"*. Codex's mechanism is the
right one — the seam raises a `BlockedFinding`, which `already_typed` exempts from `CommitFailed`
wrapping, so `surface_commit_rejection` never sees it.

**Disposition unchanged: tier 2, STILL-OPEN, expected — triaged to the 1.x ledger.** Both passes
agree; the reconciler adds ARM A′ (the *git-refusal* arm), which neither pass had driven and which
removes the last reading under which the posture arm's silence could be called consistent.

#### 10.1.3 · C-3 driven — `(2, DEFECT A)` is closed

```
setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "json probe" \
             --git-state uncommitted-pick --binary …) || exit; eval "$rig"
state: .git markers -> MERGE_MSG alone

argv/observed:
  jigc milestone create Zj1 --format json         exit=1  (see §10.3 for the arm)
  jigc task validate json-probe                   exit=1  repo.operation-in-progress
  jigc task finalize json-probe --carry-staged    exit=1  repo.operation-in-progress
  jigc task finalize json-probe --dry-run         exit=1  repo.operation-in-progress
  jigc task discard  json-probe --force           exit=1  repo.operation-in-progress
  jigc migrate-corpus --dry-run                   exit=1  repo.operation-in-progress
  jigc setup --force                              exit=1  repo.operation-in-progress
```

The no-override cross holds: **no consent flag reaches past the guard**, including the two that
closed the exit-0 path on rc.16 (`--carry-staged`, `--force`).

#### 10.1.4 · C-4 / C-5 / C-6 driven — one sweep over all fifteen git states at a commit door

`jigc milestone create Zz1`, one fresh `dev/jigc-rig committed-singletons --git-state <m>` per row:

```
rebase-merge                 HEAD=DETACHED  exit=1  repo.operation-in-progress — a rebase is in progress
rebase-apply                 HEAD=DETACHED  exit=1  repo.operation-in-progress — a rebase is in progress
bisect                       HEAD=attached  exit=1  repo.operation-in-progress — a bisect is in progress
detached                     HEAD=DETACHED  exit=1  repo.head-detached — HEAD is detached
am                           HEAD=attached  exit=1  repo.operation-in-progress — a `git am` is in progress
uncommitted-pick             HEAD=attached  exit=1  repo.operation-in-progress — an uncommitted cherry-pick is in progress
uncommitted-pick-range       HEAD=attached  exit=1  repo.operation-in-progress — an uncommitted cherry-pick is in progress
uncommitted-pick-conflicted  HEAD=attached  exit=1  repo.operation-in-progress — an uncommitted cherry-pick is in progress
uncommitted-pick-resolved    HEAD=attached  exit=1  repo.operation-in-progress — an uncommitted cherry-pick is in progress
squash-merge                 HEAD=attached  exit=1  repo.operation-in-progress — a squash merge is staged and not committed
unmerged-index               HEAD=attached  exit=1  repo.operation-in-progress — a conflict left unmerged paths in the index
sequencer                    HEAD=attached  exit=1  repo.operation-in-progress — a cherry-pick is in progress
dangling-sequencer           HEAD=attached  exit=1  repo.operation-in-progress — a cherry-pick or revert left a queue of commits in `sequencer/`
revert                       HEAD=attached  exit=1  repo.operation-in-progress — a revert is in progress
cherry-pick                  HEAD=attached  exit=1  repo.operation-in-progress — a cherry-pick is in progress
merge                        HEAD=attached  exit=1  repo.operation-in-progress — a merge is in progress
```

* **C-4 (`D1`)**: `rebase-merge`, `rebase-apply` are HEAD-**DETACHED** and still answer the
  *operation*, not `repo.head-detached`; the bare `detached` state is the only row that reaches the
  detachment member. Closed.
* **C-5 (`D2`)**: `am` → *a `git am` is in progress*, `rebase-apply` → *a rebase is in progress*.
  Two nouns, two members, one discriminator. Closed.
* **C-6 (`D3`)**: all **four** `uncommitted-pick*` cells answer `UncommittedCherryPick`, including
  the clean `-n` cell M52 left open. Closed.

This sweep also reproduces the driver's §3.4 noun column **independently, at every one of the ten
members**.

#### 10.1.5 · C-7 driven — `D3b`

Quoted in full at §10.1.2 ARM A′. `git commit` fails for a **non-hook** reason (a missing
`gpg.program` binary) and the surface reads ``` `git commit` failed (no commit was made): ```, never
*rejected* — with jigc's own warn-only `pre-commit` hook present and exiting 0 throughout, so the
discrimination is not an artefact of there being no hook. Closed.

#### 10.1.6 · C-8 — OPEN LEAD, with the reason

`codex-1` is a **closure** claim about call ordering inside `relocate.rs`: that `verify(Move)` runs
immediately before `git rm --cached` in `displace_foreign_squatter`, and that the later `git mv`
re-probes. Driving it needs a racer positioned **inside the mover seam** — i.e. a shim that fires
an operation after the door-top probe and before that specific `git` call. The reconciler's shim
fires on the first `git add` of a run, which the relocate path does not necessarily reach in that
window, and building a call-indexed racer for `relocate` was not attempted here. The driver
likewise did not re-race it (its §6.3 says so). **Neither pass drove it, so it does not become a
finding on the source read and it does not become a closure on one either.** Recorded as an open
lead: *the `relocate` / `config set placement-root` seam re-probe is unraced on rc.17.* Standing
mitigation, stated rather than relied on: M52 closed these two sites with repros, and nothing in
M53 touches `relocate.rs`'s seam.

#### 10.1.7 · C-9 / C-11 driven — `setup`'s exemption, and the family around it

```
setup: rig=$(dev/jigc-rig --git-state unborn --binary …) || exit; eval "$rig"     (standalone form)
argv : jigc setup
observed: commits 0 -> 1, exit 0, zero `repo.*` tokens on the surface

second fresh unborn rig:
argv : jigc milestone create Zu1
observed: exit 1
  blocking · repo.head-unborn — HEAD is unborn — this repository has no commits yet, so there is
    no base for jigc to commit against
    route: land the repository's first commit with `git commit`, then re-run this command
```

The exemption is real, is `setup`'s alone among the doors driven, and is visible as a **commit
count moving** rather than inferred from an absent code.

### 10.2 · Driver defects → status

| driver finding | tier | reconciler's status | what settles it |
|---|---|---|---|
| **DEFECT 1** — `jigc milestone finalize` commits a provisioned sub-task worktree's **un-concluded operation** at exit 0, under jigc's own subject, destroying the operation's authored message, while a door run **inside** that worktree refuses one command earlier | **1** | **STANDS — re-driven by the reconciler, twice, on two different `InProgress` members** | §10.2.1 |

The Codex source pass is **silent** on DEFECT 1 — it does not say it cannot happen. Its
enumeration of production commit/move acts (§*Relevant production acts*) simply does not contain
the milestone boundary's apply-a-worktree's-staged-diff path, which is why its completeness
sentence C-10 is refuted rather than its dispositions.

#### 10.2.1 · DEFECT 1 re-driven

**Arm 1 — the uncommitted-cherry-pick member (M53's new one), default `finalize.fan-out.squash`:**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"
       jigc milestone create "Swallow milestone"        -> minted milestone:swallow-milestone
       jigc milestone add-task swallow-milestone "swallow sub intent"
       jigc milestone provision swallow-milestone       -> provisioned 1 worktree(s)
       W=$REPO/.jigc/worktrees/swallow-sub-intent
       printf 'sub work\n'       > $W/subwork.txt; git -C $W add subwork.txt
       printf 'PICKED PAYLOAD\n' > $W/picked.txt;  git -C $W add picked.txt
       git -C $W commit -m 'the users authored pick message'; P=$(git -C $W rev-parse HEAD)
       git -C $W reset --hard HEAD~1
       printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt
       git -C $W cherry-pick -n "$P"                    -> rc=0

BEFORE (measured, not assumed):
  head -1 .git/worktrees/swallow-sub-intent/MERGE_MSG  -> the users authored pick message
  git -C $W diff --cached --name-only                  -> picked.txt  subwork.txt
  git -C $REPO status --porcelain                      -> (empty)      <- the MAIN checkout is clean
  ls .git | grep -E 'MERGE_MSG|MERGE_HEAD|CHERRY_PICK_HEAD'  -> (none)  <- and carries no marker
  (cd $W && jigc task finalize swallow-sub-intent)
     -> blocking · repo.operation-in-progress — an uncommitted cherry-pick is in progress
        route: conclude it with `git commit` (…), or abandon it with `git reset` (…)
  jigc milestone list-tasks swallow-milestone (main checkout) -> exit 0, no repo.* code

argv : jigc milestone join     swallow-milestone   -> exit 0, "0 doc(s) merged"
       jigc milestone finalize swallow-milestone

observed:
  finalized 19d3b94 — Finalize milestone swallow-milestone (1 sub-task)
    modified docs/milestone-records/swallow-milestone.md
    added picked.txt                      <- the USER's un-concluded pick
    added subwork.txt
    3 files committed
    sub-tasks: swallow-sub-intent: 2 code files
  exit=0

AFTER:
  git cat-file -p HEAD:picked.txt                     -> PICKED PAYLOAD
  git log --all --oneline                             -> the pick's own commit is NOWHERE
  .jigc/worktrees/                                    -> empty (worktree removed)
  .git/worktrees/swallow-sub-intent/MERGE_MSG         -> No such file or directory
     i.e. "the users authored pick message" is destroyed and recoverable from no git object
```

**Arm 2 — the merge member (a marker-backed one), to establish the class rather than the member.**
The driver's own merge repro is reproduced here with a corrected fixture: the merge must be entered
on a **clean** worktree, and the sub-task's staged work added afterwards (entering it over staged
work makes git refuse the merge — one construction attempt did exactly that and is recorded here
rather than dropped).

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       jigc milestone create "FM milestone"; jigc milestone add-task fm-milestone "fm sub intent"
       jigc milestone provision fm-milestone;  W=$REPO/.jigc/worktrees/fm-sub-intent
       git -C $W checkout -b mergesrc
       printf 'MERGED PAYLOAD\n' > $W/merged.txt; git -C $W add merged.txt
       git -C $W commit -m 'the users authored merge source'
       git -C $W checkout -
       git -C $W merge --no-commit --no-ff mergesrc   -> rc=0, "stopped before committing as requested"
       printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt

BEFORE:
  ls .git/worktrees/fm-sub-intent | grep -E '^(MERGE_HEAD|MERGE_MSG)$'  -> MERGE_HEAD  MERGE_MSG
  head -1 .git/worktrees/fm-sub-intent/MERGE_MSG   -> Merge branch 'mergesrc' into HEAD
  git -C $W diff --cached --name-only              -> merged.txt  subwork.txt
  git -C $REPO status --porcelain                  -> (empty)
  ls .git | grep -E '^(MERGE_HEAD|MERGE_MSG)$'     -> (none)
  (cd $W && jigc milestone create Zw1)
     -> blocking · repo.operation-in-progress — a merge is in progress
        route: conclude it with `git merge --continue` once its conflicts are resolved, or abandon
               it with `git merge --abort` (…)

argv : jigc milestone join fm-milestone   -> exit 0
       jigc milestone finalize fm-milestone

observed:
  finalized c6d9492 — Finalize milestone fm-milestone (1 sub-task)
    modified docs/milestone-records/fm-milestone.md
    added merged.txt                      <- the user's un-concluded merge payload
    added subwork.txt
    3 files committed
    sub-tasks: fm-sub-intent: 2 code files
  exit=0

AFTER:
  git cat-file -p HEAD:merged.txt                  -> MERGED PAYLOAD
  ls .git/worktrees/fm-sub-intent | grep -E '^(MERGE_HEAD|MERGE_MSG)$'  -> (GONE)
```

Two members, two detection styles (marker-negated and marker-backed), one shape: **the door that
commits a worktree's index never asks that worktree's posture.** The driver's framing of the class
— *the subject axis, not the member axis* — is reproduced, not merely restated.

**Bound carried forward, unchanged:** the **non-squash** commit model
(`config set finalize.fan-out.squash false`) remains **un-driven to a landed commit** by both the
driver and the reconciler. It is recorded as un-driven, not as passing.

**Second bound, the reconciler's own:** neither arm above was driven with the *operation's own
conflict* unresolved in the worktree (both were clean/staged states). Whether the boundary's apply
step would fail loudly on a genuinely unmerged sub-task index is **not established here**.

### 10.3 · The demotion check — rows marked driven that carry no fenced repro block

The driver states its convention up front (§3: *"repro blocks are one per cell"*), and several
tables record their construction in the row rather than in a fenced block. Applied strictly, that
would demote most of §3. The reconciler applied the check as written and then **settled it by
driving**, rather than by demoting on formatting: for every section whose table carries no fenced
block, the construction was recovered from the row and re-run.

| driver section | fenced block? | reconciler's action | outcome |
|---|---|---|---|
| §3.2 HEAD detached | no | re-driven at `milestone create`, `rename`, `migrate-corpus`, `relocate`, `config set` on `--git-state detached` | **reproduces exactly** — `repo.head-detached`, Human route *"re-attach HEAD with `git switch <branch>`, then re-run this command"*; `relocate` → `relocate.frozen-doctype`; `config set` → exit 0. **No demotion** |
| §3.4 the ten members × noun | no (the abandon table is fenced at §3.7) | re-driven at `milestone create` across **all fifteen** git states | **reproduces exactly**, every noun (§10.1.4). **No demotion** |
| §3.8 pinned surfaces | no | `--format json` arm re-driven under `uncommitted-pick` | **reproduces**: stdout **0 bytes**, stderr one JSON document with keys `['error']` carrying the flattened blocking line + route, exit 1. **No demotion**. *(The invocation-log row was not re-driven — the reconciler's rigs ran with the log off; the driver's own method note at §3.8 explains why the log must be enabled before the state is induced.)* |
| §3.9 no-override cross | the second half only | all seven flag cells re-driven | **reproduces exactly** — every one exit 1 `repo.operation-in-progress` (§10.1.3). **No demotion** |
| §3.10 the seam re-probe | no | re-driven with an independently built shim, bisect racer | **reproduces** (§10.1.2). **No demotion** |
| §3.11 the worktree cross | no (DEFECT 1's block is at §4) | the defect row re-driven twice (§10.2.1); the *exempt* row re-driven as the `milestone create` inside the worktree in both arms | **reproduces** — a clean provisioned worktree is exempt from `head-detached`, and an operation inside it still refuses. **No demotion** |
| §3.12 the 35 `Neither` controls | no | **not** re-driven as a 35-leaf sweep; the `task validate` row (the one leaf that answers) re-driven | `task validate` → exit 1 `repo.operation-in-progress`, as recorded. **No demotion**, but the 34-silent half rests on the driver's run alone — stated here rather than assumed |
| §3.13 precedence | no | both pack-fault cells re-driven | **reproduces exactly** — commit door answers the **posture**, `doc list` answers the **pack** fault (*"is not a valid pack-set list: invalid type: string \"just-a-scalar\""*). The cwd cell was not re-driven. **No demotion** |
| §3.14 the false-positive hunt | no | the `squash-merge`-conflicted row re-driven (§10.1.1); `unmerged-index`, clean `squash-merge`, `revert` re-driven via §10.1.4 | **reproduces**. The `git stash apply` / user-concludes-it rows were not re-driven. **No demotion** |
| §3.15 `GIT_DIR` | no | **not** re-driven | stays as the driver recorded it — a **declared-out** bound, not a finding either way |

**Net: zero rows demoted.** Every unfenced row the reconciler re-drove reproduced byte-consistently
with what the driver recorded; the three it did not re-drive are named above and none of them
carries a defect.

---

## 11 · Doors covered (reconciled)

Every clap leaf that is the door of ≥1 **driven** row in this file, `VERB_KINDS` spelling. The
axis's coverage is the union of the driver's sweep and the reconciler's re-drives.

**`BEHALF_DOORS ▸ CommitsOnBehalf` (10):** `setup` · `migrate-corpus` · `rename` · `task discard` ·
`task finalize` · `milestone create` · `milestone add-task` · `milestone add-from-spec` ·
`milestone finalize` · `milestone discard`

**`BEHALF_DOORS ▸ MovesOnBehalf` (2):** `relocate` · `config set`

**`BEHALF_DOORS ▸ Neither` (35), driven as controls:** `start` · `workflow` · `uninstall` ·
`upgrade` · `ingest` · `migrate` · `unmanage` · `describe` · `validate` · `doc create` ·
`doc add-item` · `doc remove-item` · `doc retitle-item` · `doc rename` · `doc set-field` ·
`doc set-slot` · `doc author` · `doc show` · `doc schema` · `doc list` · `task list` · `task diff` ·
`task validate` · `task bind` · `config insert-step` · `config replace-step` · `config remove-step` ·
`config fill` · `config fork` · `config get` · `config list` · `milestone list-tasks` ·
`milestone provision` · `milestone execute` · `milestone join`

**Total: 47 of 47 — `uncovered: none`.**

Doors the **reconciler itself** drove (the subset that carries the §10 verdicts): `setup` ·
`migrate-corpus` · `rename` · `task discard` · `task finalize` · `task validate` · `milestone create` ·
`milestone join` · `milestone finalize` · `milestone add-task` · `milestone provision` ·
`milestone list-tasks` · `relocate` · `config set` · `doc list` · `doc set-field` · `doc set-slot` ·
`task list`.

---

## 12 · The reconciler's method and bounds

**Method.** Every Codex claim was entered as a lead and driven before it was given a verdict; no
verdict in §10 rests on reading source. Every driver defect was re-driven from a rig the
reconciler built itself, and DEFECT 1 was driven on a **second** `InProgress` member the driver's
own repro did not land (its merge arm; the reconciler's first attempt at that fixture failed
because the merge was entered over staged work, and that failure is recorded at §10.2.1 rather than
dropped). Exit codes were read **bare**, never through a pipe. Every fixture root came from
`mktemp -d`; there is no teardown and none is needed.

**Bounds.**

1. **C-8 (`codex-1`) is an open lead, not a closure.** Neither pass raced the `relocate` /
   `config set placement-root` seam on rc.17. A source read showing the calls adjacent is not a
   drive, and this reconciliation does not upgrade it into one.
2. **C-9's `⊆`-assertion half and C-11's `Neither` half are source facts both passes agree on.**
   The reconciler drove the *behavioural* halves (`setup` exempt at unborn; `task validate` as the
   one answering `Neither` leaf) and did not re-run the 34-leaf silent sweep.
3. **DEFECT 1's non-squash commit model is still un-driven to a landed commit**, by both passes.
4. **DEFECT 1 was not driven with a genuinely conflicted sub-task index** — both arms carried a
   clean or fully-staged operation.
5. **The invocation-log row (§3.8) was not re-driven** — the reconciler's rigs ran with the
   invocation log off.
6. **`GIT_DIR` (§3.15) was not re-driven** and stays the family's declared-out bound.
7. **The racer is a deterministic shim, not a genuine concurrent process.** The window it opens is
   real and the fire is verified by a stamp, but a true race is not drivable here — the same bound
   the driver carried.
8. **Nothing was fixed, committed or edited in the repository under review.** `git status` in
   `/Users/maurice/projects/gherrink-jigc` was checked clean before the first drive (one
   pre-existing untracked directory, `completions/artifacts/M53/per-axis-review/`) and every drive
   ran inside a throwaway rig root.
