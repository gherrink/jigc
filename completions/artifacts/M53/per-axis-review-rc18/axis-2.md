<!-- AXIS 2 · posture — RECONCILED. Every row driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.18` (repo HEAD `271b0cb7`), 2026-09-23. Copied verbatim from the reconciler; this header line is the only addition. -->
<!-- M53 SECOND PARTIAL per-axis review (axes 2 · 3) — axis 2 · posture — the OPUS DRIVER · every row driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.18`, repo HEAD `271b0cb7`, 2026-09-23 -->

# M53 second partial per-axis review — AXIS 2 · posture — RECONCILED

> **Reconciler's frame.** What follows is the Opus driver's table **unchanged except for the
> demotions marked `⚠ DEMOTED`** — rows the driver presented as driven that carry **no repro block**.
> Per the reconciliation rule a row with no repro block is **not driven**, whatever it says. Each
> demoted row names, on its own line, whether this reconciler then drove it independently; where it
> did, the repro is in the **Reconciliation ledger** at §12 and the row is *re-established on that
> drive*, not on the driver's word. §12 also disposes every Codex source-pass claim (CONFIRMED with a
> repro · REFUTED with the falsifying datum · OPEN LEAD with the reason) and every driver defect.
> The binary is the same one the driver used — `/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.18`**,
> asserted before anything ran. Every drive of this reconciler's own ran in a fresh `dev/jigc-rig` root
> from `mktemp -d`; nothing was fixed, committed or edited in the working repository.

# M53 second partial per-axis review — AXIS 2 · posture — the OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.18`**, asserted first, before anything
else ran (exit 0). Repository at HEAD **`271b0cb7`** (*"chore(release): 1.0.0-rc.18 — the second M53
stamp, after the post-review fix and its review's seven fixes"*). This is the **RELEASE** posture: the
`Route::mechanical` argv fence is `#[cfg(debug_assertions)]` and does not exist here, so every refusal
below is the shipped one and a route-fence violation shows up as a **bad emitted command**, never as a
panic.

**The fixed binary.** The post-review fix (`3c71da87`) **and its independent review's seven fixes**
(`7329801b` · `a6711cee` · `de77b686` · `7a44d85d` · `7af8d6b4` · `5da4634a` · `16da362a`) are all in
this build; each is driven as a cell below rather than assumed. M53's **declared bounds are what I grade
against, not re-find**: the stderr-only advisory at `milestone finalize`, the 21-render
`DEBUG_REMAINDER`, the residual cleared by hand, `reseed_sub_task_areas`' `.exists()` skip. Where a
neighbouring wave's declared bound covers something I hit, I say so and do not report it
(`finalize.render-io`'s host-absolute `at:` — §7.9).

**Fixtures.** `dev/jigc-rig` throughout, **two-step eval**, every root from `mktemp -d`. There is no
teardown and none is needed — **no `rm -rf` on a variable path appears anywhere in this review.** Where
a member of `InProgress::ALL` had to be induced *inside a linked worktree* (which the rig builds only
for a plain repo) it was induced by running the real git commands a user runs, in
`driver/induce.sh`, and the resulting markers/unmerged/porcelain state is **printed in the repro block**
rather than assumed. Nothing was written into `.jigc/` by hand.

**Instrument note honoured.** This harness's `grep` is a shell function honouring `.gitignore`. Every
claim below about bytes under `.jigc/` is made with `ls` / `cat` / `head` on the exact path, or with
`command grep`, and every loss/no-loss claim carries a **before-control printed from the same
expression that printed the after**. Exit codes are measured **bare**, never through a pipe — one early
measurement (`task validate … | head -2`) read the *head's* 0, was caught and re-measured bare; the
corrected figure is the one recorded.

**Nothing was fixed, committed or edited in the working repository.** Every drive ran inside a
throwaway rig root. The one cell that needs two repositories (`GIT_DIR`, §6.4) was built from **two
rigs**, with both roots asserted to exist before any `git -C` ran — the rc.17 instrument failure
(parsing the rig's stdout instead of eval-ing it) explicitly guarded against.

---

## 1 · The door set and the registry counts, read from the code at HEAD `271b0cb7`

| registry | file | count I read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1830` | **47** leaf rows |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:1999` | **47** rows — the total classification |
| ↳ `CommitsOnBehalf` | | **10** |
| ↳ `MovesOnBehalf` | | **2** |
| ↳ `Neither` | | **35** |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **10** rows |
| `PostureMember::ALL` | `crates/cli/src/repo.rs:157` | **3** |
| **`InProgress::ALL`** | `crates/cli/src/repo.rs:291` | **10** |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3369` | **6** (`WORKTREE_DOORS` ⊂ it: **4**) |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs` | **26** `WorkUnitIdDoor {` literals (the 25 rows + the struct's own definition line) |
| `MINT_DOORS` | `crates/engine/src/state.rs` | **6** `MintDoor {` literals (5 rows + the struct line) |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs` | **65** `EnvelopeArm {` literals |
| `dev/jigc-rig --list-git-states` | | **17** |

**Axis 2's door set = the 12 acting `BEHALF_DOORS` rows** (10 `CommitsOnBehalf` ∪ 2 `MovesOnBehalf`),
**plus a second subject the post-review fix minted**: every **provisioned sub-task worktree**
`jigc milestone finalize` commits from, and its preview at `jigc task validate <sub-id>`. The 35
`Neither` rows are driven as controls.

**The new mechanism, read before it was probed:** `crate::repo::adjudicated_breach` (`repo.rs:780`) is
the one composition both `cli::posture_refusal_in` (`cli.rs:605`) and the boundary's
`fan_out_posture_findings` (`milestone.rs:3946`) ask; `BreachSite` (`repo.rs:584`) is `Here` |
`FanOutWorktree(&str)`; `LeftoverHold.operation` (`milestone.rs:3472`) is the destroying doors' second
leg, asked through the same composition (`held_operation`, `:3484`); `repo::aim_at` (`repo.rs:669`) is
the `-C` redirection's one home, with **two** callers (`BreachSite::aim`, `milestone::held_here`).

---

## 2 · The cell set

`{the ten InProgress members × the main checkout · the ten members × a provisioned sub-task worktree ·
HEAD detached · HEAD unborn · the clean control · the no-override cross · the `--format json` arm at
**both** `BreachSite`s · the invocation log · the `task validate` preview at both sites · the commit-seam
race · the dedicated-worktree cross · the `Neither` class as control · `PRE_DISPATCH_FAULTS` precedence ·
the `GIT_DIR` redirect (declared out) · a false-positive hunt **inside the new subject** · the three
refusing destroying doors × a **clean-tree** live operation, without and with `--force` · the render
root from a non-root cwd · both `finalize.fan-out.squash` commit models · one and two breaching
worktrees}`

**≈210 `(door, cell)` drives are recorded below.** §7 states what was not driven and why.

---

## 3 · THE HEADLINE — the tier-1 count

> # TIER-1 ROWS FOUND: **0**
>
> **`(2, DEFECT 1)` — the rc.17 tier-1 row — is CLOSED**, driven over **all ten** `InProgress::ALL`
> members inside a provisioned sub-task worktree and at **both** commit models, with the boundary
> refusing at exit 3 before anything durable is written, the operation's authored message intact
> afterwards, and the boundary landing cleanly once the emitted route has been run.
>
> **Two new findings, neither tier 1:** one **tier 2** (a route the caller cannot run from where they
> are standing — *inside the post-review fix's own new code*) and one **tier 3** (the boundary commits a
> second repository's worktree bytes and acks them as the sub-task's work — *outside* it; the subject
> rule is M46's).

---

## 4 · The (door, cell) table

Exit / code / route are identical across the doors within a cell unless the table says otherwise, so
repro blocks are **one per cell**.

### 4.1 · Clean control — no `repo.*` at any door

| door | exit | code | verdict |
|---|---|---|---|
| all 12 acting | own business | **no `repo.*` at any door** | matches contract |

Driven inside every fixture below as the pre-state (e.g. §4.9's `git -C $REPO status --porcelain` →
empty, `ls .git | command grep -E 'MERGE_MSG\|MERGE_HEAD\|CHERRY_PICK_HEAD'` → none) and as the
zero-false-fire control at §4.13.

### 4.2 · The ten `InProgress::ALL` members × the MAIN checkout, at a commit door

**All 16 buildable `--git-state` members driven at `jigc milestone create`, one fresh rig each.**

```
setup: for st in merge squash-merge rebase-merge rebase-apply am cherry-pick sequencer \
                 dangling-sequencer uncommitted-pick uncommitted-pick-range \
                 uncommitted-pick-conflicted uncommitted-pick-resolved revert unmerged-index \
                 bisect detached; do
         rig=$(dev/jigc-rig committed-singletons --git-state $st \
               --binary /Users/maurice/.local/bin/jigc) || continue; eval "$rig"
argv : jigc milestone create Zz1

observed (exit=1 at every row; HEAD attachment measured per row):
  merge                        attached  repo.operation-in-progress — a merge is in progress
     route: conclude it with `git merge --continue` once its conflicts are resolved, or abandon it
            with `git merge --abort` (which also discards anything else you had staged, from the
            index and from your working tree), then re-run this command
  squash-merge                 attached  — a squash merge is staged and not committed     [git reset --merge]
  rebase-merge                 DETACHED  — a rebase is in progress                        [git rebase --abort]
  rebase-apply                 DETACHED  — a rebase is in progress                        [git rebase --abort]
  am                           attached  — a `git am` is in progress                      [git am --abort]
  cherry-pick                  attached  — a cherry-pick is in progress                   [git cherry-pick --abort]
  sequencer                    attached  — a cherry-pick is in progress                   [git cherry-pick --abort]
  dangling-sequencer           attached  — a cherry-pick or revert left a queue of commits in `sequencer/`
                                                                                          [git cherry-pick --quit]
  uncommitted-pick             attached  — an uncommitted cherry-pick is in progress       [git reset]
  uncommitted-pick-range       attached  — an uncommitted cherry-pick is in progress       [git reset]
  uncommitted-pick-conflicted  attached  — an uncommitted cherry-pick is in progress       [git reset]
  uncommitted-pick-resolved    attached  — an uncommitted cherry-pick is in progress       [git reset]
  revert                       attached  — a revert is in progress                         [git revert --abort]
  unmerged-index               attached  — a conflict left unmerged paths in the index     [git reset --merge]
  bisect                       attached  — a bisect is in progress                         [git bisect reset]
  detached                     DETACHED  repo.head-detached — HEAD is detached             [git switch <branch>]
```

**Verdict: matches contract at every row, byte-for-byte with what rc.17 recorded.** M51's `D1`
(rebase/bisect answer the operation, not the detachment) and `D2` (`am` vs `rebase --apply`) are
**still CLOSED**, driven.

### 4.3 · The 12 acting doors × the four `uncommitted-pick*` cells — M52 `(2, DEFECT A)`

```
setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "axis intent" \
             --git-state <cell> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc milestone create "Axis milestone"
argv : the 12 acting BEHALF_DOORS rows:
       setup · migrate-corpus · rename vision --to "Renamed Vision" · task discard <t> ·
       task finalize <t> · milestone create "Zz probe" · milestone add-task <m> "zz sub" ·
       milestone add-from-spec <m> spec:nope · milestone finalize <m> · milestone discard <m> ·
       relocate vision --from docs/old-vision.md · config set docs-root docs2

observed, IDENTICAL in all four cells (uncommitted-pick · -range · -conflicted · -resolved):
  all 12 -> exit=1  blocking · repo.operation-in-progress
                    — an uncommitted cherry-pick is in progress
  after the full 12-door sweep:
    HEAD        UNMOVED
    .git/MERGE_MSG  md5 IDENTICAL
    index           md5 IDENTICAL
```

*(Method note: the first pass drove `relocate vision --to …`, which is a clap usage error — exit 2, not
a driven row. The argv was corrected against `jigc relocate --help` to `--from` and re-driven; the table
records the corrected drive.)*

**`(2, DEFECT A)` is CLOSED on rc.18.**

### 4.4 · **The ten members × a PROVISIONED SUB-TASK WORKTREE — `(2, DEFECT 1)` re-driven**

This is the cell the exit rule points at.

```
setup (per member, one fresh rig each):
  rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
  jigc milestone create "M <member>"; jigc milestone add-task <m> "sub <member>"
  jigc milestone provision <m>
  W=$REPO/.jigc/worktrees/sub-<member>
  printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt
  driver/induce.sh "$W" <member>        # real git commands only; markers printed below
argv : jigc milestone finalize <m>

observed — EXIT 3 AT EVERY MEMBER, each naming its own worktree and aiming its own route:
  member (markers left / unmerged / porcelain)     rendered message + route
  merge             MERGE_HEAD MERGE_MSG / 3 / 2   in the fan-out worktree `.jigc/worktrees/sub-merge`, a merge is in
                                                   progress — the milestone boundary commits that worktree's index, and
                                                   it is not in a committable state
                                                   at: .jigc/worktrees/sub-merge
                                                   route: … `git -C .jigc/worktrees/sub-merge merge --continue` … `… merge --abort` …
  squash-merge      MERGE_MSG SQUASH_MSG / 3 / 2   … a squash merge is staged and not committed … `git -C … reset --merge`
  rebase-merge      MERGE_MSG rebase-merge / 3 / 1 … a rebase is in progress … `git -C … rebase --abort`
  rebase-break      rebase-merge / 0 / 0           … a rebase is in progress … (TREE CLEAN — the cell the HIGH is about)
  rebase-apply(am)  rebase-apply / 0 / 1           … a `git am` is in progress … `git -C … am --abort`
  am                rebase-apply / 0 / 1           … a `git am` is in progress … `git -C … am --abort`
  cherry-pick       MERGE_MSG CHERRY_PICK_HEAD/3/2 … a cherry-pick is in progress … `git -C … cherry-pick --abort`
  sequencer         + sequencer / 3 / 2            … a cherry-pick is in progress …
  dangling-sequencer sequencer ONLY / 0 / –        … a cherry-pick or revert left a queue of commits in `sequencer/` …
                                                     `git -C .jigc/worktrees/ds-area cherry-pick --quit`
  revert            MERGE_MSG REVERT_HEAD / 0 / 1  … a revert is in progress … `git -C … revert --abort`
  bisect            BISECT_LOG / 0 / 1             … a bisect is in progress … `git -C … bisect reset`
  unmerged-index    (none) / 3 / 2                 … a conflict left unmerged paths in the index …
                                                     `git -C .jigc/worktrees/sub-unmerged-index reset --merge`
  uncommitted-pick  MERGE_MSG / 0 / 2              … an uncommitted cherry-pick is in progress …
                                                     `git -C … commit` / `git -C … reset`
```

**All ten `InProgress::ALL` members reached**, including the two the rc.17 record named as *not driven*
in a worktree (bound §7 of that file: *"`InProgress` states inside a sub-task worktree other than merge
and uncommitted-pick"*). **The site clause LEADS at every member** — including the two whose predicate
ends in a prepositional phrase (`UnmergedIndex`, `Sequencer`), which is the post-review review's LOW 1,
closed.

### 4.5 · The same cell, end to end — nothing durable, the route runs, the boundary lands

```
setup: the swallow fixture the rc.17 record used, verbatim:
       jigc milestone create "Swallow milestone"; jigc milestone add-task swallow-milestone "swallow sub intent"
       jigc milestone provision swallow-milestone; W=$REPO/.jigc/worktrees/swallow-sub-intent
       printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt
       printf 'PICKED PAYLOAD\n' > $W/picked.txt; git -C $W add picked.txt
       git -C $W commit -m 'the users authored pick message'; P=$(git -C $W rev-parse HEAD)
       git -C $W reset --hard HEAD~1; printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt
       git -C $W cherry-pick -n "$P"                                        -> rc=0

BEFORE (measured, not assumed):
  head -1 .git/worktrees/swallow-sub-intent/MERGE_MSG  -> the users authored pick message
  git -C $W diff --cached --name-only                  -> picked.txt  subwork.txt
  git -C $REPO status --porcelain                      -> (empty)    <- the MAIN checkout is clean
  ls .git | command grep -E 'MERGE_MSG|MERGE_HEAD|CHERRY_PICK_HEAD'  -> (none)
  (cd $W && jigc task finalize swallow-sub-intent)     -> exit 1  repo.operation-in-progress (Here site)

argv : jigc task validate swallow-sub-intent                      (from the MAIN checkout)
  -> exit=3
     blocking · repo.operation-in-progress — in the fan-out worktree `.jigc/worktrees/swallow-sub-intent`,
       an uncommitted cherry-pick is in progress — the milestone boundary commits that worktree's index,
       and it is not in a committable state
       at: .jigc/worktrees/swallow-sub-intent
       route: conclude it with `git -C .jigc/worktrees/swallow-sub-intent commit` (…), or abandon it
              with `git -C .jigc/worktrees/swallow-sub-intent reset` (…), then re-run this command

argv : jigc milestone join swallow-milestone            -> exit 0, "0 doc(s) merged"
       jigc milestone finalize swallow-milestone        -> exit 3, the SAME finding, byte-identical
       jigc --format json milestone finalize swallow-milestone
  -> exit=3   stdout 1038 bytes   stderr 0 bytes
     top keys ['findings','schema_version']
     findings[0].key = {"code":"repo.operation-in-progress","target":".jigc/worktrees/swallow-sub-intent"}

AFTER the refusals (nothing durable):
  HEAD                                          5201c3e7 -> 5201c3e7   (UNMOVED)
  .jigc/worktrees/swallow-sub-intent            still there
  .git/worktrees/swallow-sub-intent/MERGE_MSG   "the users authored pick message"  (INTACT)
  jigc milestone list-tasks swallow-milestone   exit 0 — the record is still open/finalizable

THE ROUTE, RUN VERBATIM FROM THE REPO ROOT:
  git -C .jigc/worktrees/swallow-sub-intent reset   -> rc=0
  jigc milestone finalize swallow-milestone         -> exit 3 milestone.zero-contribution
                                                       (the reset unstaged EVERYTHING, as its clause says)
  git cat-file -p HEAD:picked.txt                   -> fatal: does not exist in 'HEAD'
```

**Verdict: `(2, DEFECT 1)` CLOSED.** The payload is not swallowed, the authored message is not
destroyed, the refusal is placed before anything durable, and the preview and the door say the same
thing at the same exit code and on the same arm.

### 4.6 · **Both commit models** — the rc.17 declared bound discharged

The rc.17 record carried, twice: *"the non-squash commit model is **not driven to a landed commit**
… recorded un-driven, not passing."* Driven here, both halves:

```
setup: rig committed-singletons; jigc config set finalize.fan-out.squash false      -> exit 0
       jigc milestone create "E3 wave"; jigc milestone add-task e3-wave "e3 area"
       jigc milestone provision e3-wave
       (cd $W && jigc workflow sub-task --task e3-area)        # provisions the sub-task's commit doc
       printf 'work\n' > $W/w.txt; git -C $W add w.txt
       (cd $W && jigc doc set-field commit:e3-area#header/type --task e3-area --value feat)   -> 0
       (cd $W && printf 'the e3 subject\n' | jigc doc set-slot commit:e3-area#summary --task e3-area --from-file -) -> 0
       git -C $W bisect start; git -C $W bisect bad
       jigc milestone join e3-wave

argv : jigc milestone finalize e3-wave
  1) -> exit=3  repo.operation-in-progress — in the fan-out worktree `.jigc/worktrees/e3-area`, a bisect …
argv : git -C .jigc/worktrees/e3-area bisect reset            -> rc=0     (from the repo root)
       jigc milestone finalize e3-wave
  2) -> exit=0  finalized e7d5489 — Finalize milestone e3-wave (1 sub-task)
                  added w.txt … 3 files committed
                  commits (oldest first, each with the paths it landed):
                    078a859 feat: the e3 subject
                      w.txt
     git cat-file -p HEAD:w.txt -> work
```

**Both models refuse; the `squash: false` model is driven to a landed commit after the route.**

### 4.7 · One finding per breaching worktree — two discriminating keys

```
setup: milestone two-wave with sub-tasks alpha-area and beta-area, both provisioned
       git -C $WA bisect start; git -C $WA bisect bad                      (alpha only)
argv : jigc --format json milestone finalize two-wave
  -> findings: 1   [{"code":"repo.operation-in-progress","target":".jigc/worktrees/alpha-area"}]

then: an un-concluded `git merge --no-commit --no-ff` in beta-area as well
argv : jigc --format json milestone finalize two-wave
  -> findings: 2
     {"code":"repo.operation-in-progress","target":".jigc/worktrees/alpha-area"} | … a bisect is in pro…
     {"code":"repo.operation-in-progress","target":".jigc/worktrees/beta-area"}  | … a merge is in pro…
```

Matches the stated contract (*"two breaching worktrees are two discriminating `(code, target)` keys
rather than one"*). It is also the driven proxy for the review's LOW 3 (the test fixture's
one-worktree-per-repository bound): production reaches two breaching worktrees and answers per
worktree.

### 4.8 · The pinned surfaces — **one code, two sites, one declared form each**

The post-review review's MEDIUM 2.

| site | door | exit | stream / shape | verdict |
|---|---|---|---|---|
| ⚠ **DEMOTED** — `BreachSite::Here` | `jigc --format json milestone create` under `uncommitted-pick` | **1** | **stdout 0 bytes**; stderr one document, keys **`['error']`** | **no repro block in this file — not driven as recorded.** Driven independently by the reconciler (§12, L-9): the claim **holds byte-for-byte** (exit 1 · stdout **0** bytes · stderr **445** bytes, single key `error`) |
| `BreachSite::FanOutWorktree` | `jigc --format json milestone finalize` | **3** | **stdout** the pinned findings envelope, `['findings','schema_version']`, `key.target` = the worktree's repo-relative path | matches contract — the **filesystem-path** form, declared at `design/command-output-contract.md:247` (repro block at §4.5) |
| ⚠ **DEMOTED** — the same site's **preview** | `jigc --format json task validate <sub>` | **3** | **the same arm, the same document** | **no repro block in this file — not driven as recorded.** Driven independently by the reconciler (§12, L-8): the claim **holds** — exit 3, stdout the `{schema_version, findings}` envelope, stderr 0 bytes, `key.target = .jigc/worktrees/wt-two` |
| route count | every cell above | | exactly **one** `route:` line per finding | matches contract |
| invocation log | see §4.8.1 | | code carried at both sites, `binary_version` `1.0.0-rc.18` | matches contract |

#### 4.8.1 · invocation log

```
setup: rig committed-singletons; jigc config set invocation-log true   (BEFORE the breach is induced)
       fan-out provisioned; git -C $W bisect start; git -C $W bisect bad
observed rows in .jigc/logs/invocations.jsonl:
 {'argv':['milestone','finalize','log-wave'],'exit_code':3,
  'finding_codes':['repo.operation-in-progress'],'error_code':None,'binary_version':'1.0.0-rc.18'}
 {'argv':['task','validate','lg-area'],      'exit_code':3,
  'finding_codes':['repo.operation-in-progress'],'error_code':None,'binary_version':'1.0.0-rc.18'}
 {'argv':['milestone','discard','log-wave'], 'exit_code':1,
  'finding_codes':['milestone.dirty-worktree'],'error_code':None,'binary_version':'1.0.0-rc.18'}
```

### 4.9 · **The render root — `jigc_home`, not the cwd's repo root** (MEDIUM 1)

```
setup: milestone zeta-probe, sub-tasks zed-one + zed-two, both provisioned;
       git -C $W2 bisect start; git -C $W2 bisect bad
       cd $REPO/.jigc/worktrees/zed-one          <- run from INSIDE a sibling fan-out worktree

argv : jigc milestone finalize zeta-probe        -> exit=3
       jigc task validate zed-two                -> exit=3
       jigc --format json milestone finalize zeta-probe

observed:
  message  "in the fan-out worktree `.jigc/worktrees/zed-two`, a bisect is in progress — …"
  at:      .jigc/worktrees/zed-two
  key      {"code":"repo.operation-in-progress","target":".jigc/worktrees/zed-two"}
  location {"address":".jigc/worktrees/zed-two","line":1,"col":1}
  host-path scan of the whole printed surface (`command grep -c '/private/var\|/var/folders'`) -> 0
```

**MEDIUM 1 CLOSED** — the leak into the message, the `at:` locus, the pinned key and the route is gone
at every render, from inside a linked worktree. *(What the relative spelling costs at a non-root cwd is
**F-1**, §5.)*

### 4.10 · HEAD detached · HEAD unborn · `setup`'s exemption

| cell | doors | exit | code | verdict |
|---|---|---|---|---|
| ⚠ **DEMOTED** — detached | `setup` · `migrate-corpus` · `rename` · `milestone create` | 1 | `repo.head-detached`, **Human** — ``re-attach HEAD with `git switch <branch>`, then re-run this command`` | **the block below covers only the *unborn* rows — the detached rows carry no repro block and are not driven as recorded.** Driven independently by the reconciler (§12, L-10): all four **hold**, verbatim |
| ⚠ **DEMOTED** — detached | `relocate` (mover) | 1 | `relocate.frozen-doctype` — proceeds **past** the posture guard | **no repro block — not driven as recorded.** Driven by the reconciler (§12, L-10): **holds** |
| ⚠ **DEMOTED** — detached | `config set` (mover) | **0** | none — acts, knob written | **no repro block — not driven as recorded.** Driven by the reconciler (§12, L-10): **holds** (exit 0, `config get docs-root` → `docs2 (project)`) |
| unborn | **`setup`**, own fresh rig | **0** | none; **commits 0 → 1** | matches contract — the stated `Exempt(HeadUnborn)` row, visible as a commit count moving |
| unborn | `migrate-corpus` · `rename` · `milestone create`, each in its own fresh rig | 1 | `repo.head-unborn`, **Human** — ``land the repository's first commit with `git commit`, then re-run this command`` | matches contract |

```
setup: rig=$(dev/jigc-rig --git-state unborn --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
observed: setup 0 (commits 0->1) · migrate-corpus 1 repo.head-unborn · rename 1 repo.head-unborn ·
          milestone create 1 repo.head-unborn
```

### 4.11 · The no-override cross — the refusal carries no consent flag

All under `--git-state uncommitted-pick`:

```
setup --force                          -> exit=1  repo.operation-in-progress
task finalize <t> --carry-staged       -> exit=1  repo.operation-in-progress
task finalize <t> --dry-run            -> exit=1  repo.operation-in-progress
task finalize <t> --approve            -> exit=1  repo.operation-in-progress
task discard  <t> --force              -> exit=1  repo.operation-in-progress
migrate-corpus --dry-run               -> exit=1  repo.operation-in-progress
```

**No consent flag reaches past the guard.** Matches contract.

### 4.12 · Precedence against `PRE_DISPATCH_FAULTS`

```
setup: --git-state uncommitted-pick, then printf 'just-a-scalar\n' > .jigc/config/packs.yaml
milestone create Zp1  (commit door) -> exit=1  the POSTURE refusal
config set docs-root dd (mover)     -> exit=1  the POSTURE refusal
doc list              (Neither)     -> exit=1  the PACK fault
   (".jigc/config/packs.yaml is not a valid pack-set list: invalid type: string \"just-a-scalar\"…")
```

Matches contract — the posture guard sits at dispatch top, ahead of pack load, for the acting classes
only.

### 4.13 · The zero-false-fire controls and the **false-positive hunt inside the new subject**

The new subject is the risk surface the fix creates: a probe that now runs in every provisioned
worktree could refuse an ordinary fan-out.

```
control: an ordinary clean fan-out (one sub-task, staged code, no operation anywhere)
  jigc milestone finalize  -> exit 0, commit landed, worktrees torn down.  Driven 4× across this review.

false-positive hunt, every cell induced INSIDE the provisioned worktree, one fresh rig each:
  clean `git stash apply`        markers=[AUTO_MERGE]            unmerged=0  -> finalize exit=0   (silent)
  conflicting `git stash pop`    markers=[AUTO_MERGE]            unmerged=3  -> exit=3  UnmergedIndex — correct member
  clean `git revert -n`          markers=[AUTO_MERGE MERGE_MSG REVERT_HEAD]  -> exit=3  Revert — NOT the pick's noun
  merge concluded by the user    markers=[]                      unmerged=0  -> finalize exit=0   (silent)
  pick  concluded by the user    markers=[]                      unmerged=0  -> finalize exit=0   (silent)
```

**Zero false positives.** `AUTO_MERGE` alone never fires; both user-concluded operations land.
*(Instrument honesty: a first pass at the last two cells derived the milestone id with
`ls .jigc/milestones | head -1` and one rig's worktree was never built, so the exit-0 was a docs-only
boundary rather than a control. The cells were rebuilt with a fixed milestone id and the worktree's
existence asserted before the state was induced; only the rebuilt drive is recorded.)*

### 4.14 · The dedicated-worktree cross (typed `DedicatedWorktree`, never sniffed)

> ⚠ **THE WHOLE OF §4.14 IS DEMOTED.** The section is a five-row table with **no setup / argv /
> observed block anywhere in it** — no row of it is driven as recorded, and §7's LOW 2 verdict, which
> rests on it, is demoted with it. **The reconciler drove all five rows independently** (§12, L-11);
> **all five hold**, so each is re-established on that drive rather than on the driver's word.

| cell | door | exit | code | verdict |
|---|---|---|---|---|
| ⚠ inside a clean provisioned worktree (git's own `--detach`) | `milestone create` | **0** | **no `repo.head-detached`** | not driven as recorded — **re-established** by the reconciler (§12, L-11a): exit 0, `symbolic-ref -q HEAD` in the worktree → DETACHED, milestone minted |
| ⚠ the same worktree carrying a bisect, door run **inside** it | `milestone create` | 1 | `repo.operation-in-progress` — *a bisect* | not driven as recorded — **re-established** (§12, L-11b) |
| ⚠ the **main** checkout while the worktree holds it | `milestone create` | **0** | none | not driven as recorded — **re-established** (§12, L-11c) |
| ⚠ the **boundary** while the worktree holds it | `milestone finalize` | **3** | `repo.operation-in-progress` @ `FanOutWorktree` | not driven *here* as recorded; the same behaviour **is** driven with a block at §4.4/§4.5, and **re-established** (§12, L-11d) |
| ⚠ a worktree git **cannot vouch for** (`.git` pointer corrupted to `gitdir: /nonexistent/place`), staged bytes present | `milestone finalize` | 3 | `milestone.zero-contribution` | not driven as recorded — **re-established** (§12, L-11e): exit 3, `git cat-file -p HEAD:x.txt` → *path does not exist in 'HEAD'*, no panic |

### 4.15 · `jigc task discard` is **out of** the operation class, driven

```
setup: fan-out provisioned; git -C $W bisect start; git -C $W bisect bad
argv : jigc task discard td-area          -> exit=0, "discarded task td-area", record commit landed
after: .jigc/worktrees/td-area still there;  BISECT_LOG present
```

Matches the stated class (`probe_leftover`'s doc-comment: the callers are the **three refusing**
worktree doors; `task discard` stands at `.jigc/tasks/<id>/`, where there is no checkout).

### 4.16 · The commit-seam re-probe, raced

```
setup: rig committed-singletons --start decided-task "seam probe"; task authored and validating clean
       ("no findings — the task validates clean"); a deterministic `git` shim first on PATH that fires
       `git bisect start` once in $REPO on the first `git add` of the run, then execs /usr/bin/git.
argv : jigc task finalize seam-probe
observed: exit=1
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command
  shim stamp: "shim: fired [bisect start] rc=0"       <- the race really fired mid-transaction
state after (verified, and UNSAID by that surface): HEAD unmoved · 1 active task ·
  `A work.txt` still staged · .jigc/tasks/seam-probe/docs/ intact · .git/BISECT_LOG present
```

The guard holds at the seam. What the surface does **not** say is M52 `(2, DEFECT C)`, §6.2.

---

## 5 · Findings

### F-1 · **TIER 2** — the aimed route `git -C <repo-relative-path>` does not run from the checkout that printed it; driven, it exits **128** from any cwd that is not the repository root

**Where it lives: inside the post-review fix's own new code.** `crate::repo::aim_at`
(`crates/cli/src/repo.rs:669`) is minted by `3c71da87` and widened to its second caller by that fix's
independent review (the HIGH, `16da362a`/`7af8d6b4` family). Both callers are affected:
`BreachSite::aim` (`repo.rs:641`) and `crate::milestone::held_here` (`milestone.rs:3523`).

**Contract violated — the fix's own stated rationale.** `design/finalize.md:35`, written by this fix:

> *"…the refusal says which worktree and routes at `git -C <worktree> …`, **because a route the caller
> cannot run from where they are standing is not a route**."*

Driven, the caller cannot run it from where they are standing, at **every** cwd except the repository
root. `design/surface-contract.md`'s printed-path fence admits exactly this shape as a reason an
absolute **stays** — *"pasteable shell bytes — a `route:`/remedy span the operator pastes into a shell
of unknown cwd"* — and that disposition is not taken here.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc milestone create "Cwd wave"; jigc milestone add-task cwd-wave "cw area"
       jigc milestone provision cwd-wave
       W=$REPO/.jigc/worktrees/cw-area; git -C $W bisect start; git -C $W bisect bad
       mkdir -p $REPO/docs/deep; cd $REPO/docs/deep       <- an ORDINARY subdirectory of the main checkout

argv : jigc milestone discard cwd-wave
  -> emits:  abandon it with `git -C .jigc/worktrees/cw-area bisect reset`          [held_here]
argv : jigc milestone finalize cwd-wave
  -> emits:  route: … abandon it with `git -C .jigc/worktrees/cw-area bisect reset` [BreachSite::aim]

THE EMITTED COMMAND, RUN VERBATIM FROM THE CWD THAT PRINTED IT:
  git -C .jigc/worktrees/cw-area bisect reset
    fatal: cannot change to '.jigc/worktrees/cw-area': No such file or directory
    rc=128                                    <- both producers, identically

CONTROL, same command, from the repository root:
  cd $REPO; git -C .jigc/worktrees/cw-area bisect reset
    HEAD is now at 7f12971 …
    rc=0

SECOND CWD, the topology the fix's own review used to find MEDIUM 1 —
from inside a SIBLING fan-out worktree ($REPO/.jigc/worktrees/zed-one):
  jigc milestone finalize zeta-probe  -> route: `git -C .jigc/worktrees/zed-two bisect reset`
  git -C .jigc/worktrees/zed-two bisect reset
    fatal: cannot change to '.jigc/worktrees/zed-two': No such file or directory
    rc=128
```

**Surfaces affected (driven):** `repo.operation-in-progress` @ `BreachSite::FanOutWorktree` at
`jigc milestone finalize` and at `jigc task validate <sub-id>`; the per-path abandon command in
`milestone.dirty-worktree` (`jigc milestone discard`), `uninstall.dirty-worktree` (`jigc uninstall`) and
`milestone.leftover-holds-work` (`jigc milestone provision`).

**Why tier 2 and not tier 1:** nothing is lost and nothing is committed — every one of these doors
*refused*. The charter's tier-2 predicate is *a posture or route dead end*, and this is a route that,
followed exactly, does nothing and says only that the path does not exist.

**Why tier 2 and not tier 3:** it is not that a surface says something the binary does not do; it is
that the binary's own stated remedy is unrunnable as printed, which is the thing law 2 and the route
floor exist to prevent. The honest counter-argument, recorded rather than suppressed: the relative path
is correct *relative to the repository root*, a reader who notices that can act on it, and a bare
absolute would re-open the MEDIUM 1 key leak — so the fix that closes this has to keep the **message,
the `at:` locus and the key** repo-relative while making the **command span** resolvable, which is a
real design call and not a one-line swap.

### F-2 · **TIER 3** — `jigc milestone finalize` commits a **second repository's** linked worktree's staged bytes into this repository at exit 0 and acks them as the sub-task's own work

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       B=$(mktemp -d "${TMPDIR:-/tmp}/repoB.XXXXXX"); git -C $B init -b main; two commits in B
       jigc milestone create "Foreign wave"; jigc milestone add-task foreign-wave "fw area"
       # NO `jigc milestone provision` — B's own worktree is parked at the sub-task path instead:
       mkdir -p $REPO/.jigc/worktrees
       git -C $B worktree add -q "$REPO/.jigc/worktrees/fw-area" -b fwbranch
       printf 'B-SECRET-PAYLOAD\n' > $W/bsecret.txt; git -C $W add bsecret.txt
       printf 'B-UNTRACKED-KEEP\n'  > $W/keepme.txt

BEFORE: git -C $W rev-parse --show-toplevel -> the squatted path itself (so classify_leftover says OwnWorktree)
        git -C $W symbolic-ref -q HEAD      -> refs/heads/fwbranch   (B's branch, ATTACHED)
        git -C $W diff --cached --name-only -> bsecret.txt

argv : jigc milestone join foreign-wave      -> exit 0
       jigc milestone finalize foreign-wave
observed:
  finalized 1e5c9c5 — Finalize milestone foreign-wave (1 sub-task)
    added bsecret.txt
    modified docs/milestone-records/foreign-wave.md
    2 files committed
    sub-tasks: fw-area: 1 code file           <- B's bytes, acked as the sub-task's own work
  exit=0

AFTER: git -C $REPO cat-file -p HEAD:bsecret.txt      -> B-SECRET-PAYLOAD
       $W/keepme.txt                                   -> B-UNTRACKED-KEEP     (intact)
       $W/bsecret.txt                                  -> B-SECRET-PAYLOAD     (intact)
       git -C $B worktree list                         -> both entries intact
       .jigc/displaced/                                -> not created
```

**Why tier 3, measured rather than assumed:** **no byte is lost** — B's worktree, its index, its
untracked plant and its registration all survive; the teardown does not reach an unregistered worktree.
What is wrong is the **ack**: *"sub-tasks: `fw-area`: 1 code file"* names as the sub-task's work a file
that belongs to another repository, which is a law-1 claim the binary cannot support. The subject rule
that admits the worktree is **M46 Increment 2 / T1** (*"`provisioned_worktrees` takes the on-disk path
instead of the registered set"*), not anything M53 landed, and the shipped policy already treats that
path as jigc's territory (`team-ready-state.md`, quoted as a declared bound in
[M53/VERDICT.md](../../../../completions/artifacts/M53/VERDICT.md) → Addendum: *"a clean and concluded
foreign worktree at an unregistered sub-task path is still cleared by `provision` without consent"*).
The posture family behaved exactly as designed here — it probed the worktree (`posture_subject` → all
three legs pass, since `fw-area` **is** a registered sub-task id) and found no breach.

**Stated so a next reader does not re-tier it silently:** a human could argue tier 1 on *repository
harm through a committing door*. I do not, because the harm is a reversible commit, nothing is
destroyed, and the reachability requires a foreign repository's worktree parked at exactly
`.jigc/worktrees/<sub-task-id>` with `provision` never run.

---

## 6 · M52 §A rows for this axis — CLOSED / STILL-OPEN

All three §A rows M52 carries for axis 2, **all tiers**, re-driven on `1.0.0-rc.18`.

| M52 §A row | tier | verdict on rc.18 | the argv + the observation that settles it |
|---|---|---|---|
| **`(2, DEFECT A)`** — a clean `git cherry-pick --no-commit` is a member of no `InProgress::ALL` row, and `jigc task finalize` concludes it at exit 0, destroying the picked commit's authored message | **1** | **CLOSED** | §4.3: all four `uncommitted-pick*` cells × all 12 acting doors → exit 1 `repo.operation-in-progress — an uncommitted cherry-pick is in progress`; after the full sweep HEAD, `.git/MERGE_MSG` and the index are **byte-identical**. `--carry-staged` / `--force` / `--dry-run` / `--approve` all refuse first (§4.11). §4.2 reproduces the member at a commit door in all four cells |
| **`(2, DEFECT C)`** — a posture breach raised at the **commit seam** prints no state-truth clause and no copy-runnable re-run, while every other in-transaction failure at the same door prints both | **2** | **STILL-OPEN** — *expected: triaged to the 1.x ledger by the charter, **not** a new finding* | §6.2 |
| **`(2, DEFECT B)`** — a conflicted `git merge --squash` is answered by `SquashMerge`, whose predicate is false of the state | **3** | **STILL-OPEN** — *expected: triaged to the 1.x ledger, **not** a new finding* | §6.1 |

### 6.1 · `(2, DEFECT B)` — reproduces exactly

```
setup: rig committed-singletons; a conflicting branch; git merge --squash cb      -> rc=1, CONFLICT
state: markers -> MERGE_MSG, SQUASH_MSG ;  git ls-files -u | wc -l -> 3   <- the index IS conflicted
argv : jigc milestone create CS1
  -> exit=1  blocking · repo.operation-in-progress — a squash merge is STAGED AND NOT COMMITTED
             route: conclude it, or abandon it with `git reset --merge` (…), then re-run this command
route still effective: git reset --merge -> rc=0, unmerged 3 -> 0, door after -> exit 0
```

The noun asserts the index is *staged*; `git ls-files -u` says 3 paths are **unmerged**. The conclude
arm is command-less here. Unchanged from rc.16 and rc.17.

### 6.2 · `(2, DEFECT C)` — reproduces byte-for-byte, with its contrast arm re-driven

The posture arm is §4.16 in full: one code, one `route:` line, **and nothing else**, while HEAD is
unmoved, the task is intact, the staged docs are on disk and the index still carries `A work.txt` —
none of which the surface states. The contrast, same door, one step apart:

```
ARM A — a rejecting pre-commit hook, fresh rig, task "hook probe":
  `git commit` was rejected (no commit was made):
  the hook says no

  task hook-probe is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/hook-probe/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize hook-probe`.
  exit=1
```

Two in-transaction failure causes at one seam; one prints the state-truth clause **and** the
copy-runnable re-run, the posture one prints neither and offers only *"re-run this command"*.

### 6.3 · The M51 rows this axis carries forward

- **`D1`** (rebase/bisect answer the operation, not the detachment) — **still CLOSED**, driven across
  all 16 git states at a commit door (§4.2): `rebase-merge` and `rebase-apply` are HEAD-**DETACHED** and
  still answer the *operation*; the bare `detached` state is the only row reaching `repo.head-detached`.
- **`D2`** (`am` vs `rebase --apply`) — **still CLOSED**, both directions, two nouns, two abort commands.
- **`D3`** (the uncommitted-pick cells) — **still CLOSED**, all four.
- **`D3b`** (a git refusal not dressed as a hook rejection) — **not re-driven** (§7.4); §6.2's hook arm
  shows the hook frame intact and M53 changed no commit-seam classification.
- **`codex-1`** (the `relocate` / `config set placement-root` seam re-probe) — **not re-raced** (§7.3);
  it remains the open lead the rc.17 record carried, neither closed nor promoted here.
  **[Reconciler, superseding: this row is now CLOSED, driven.** The Codex source pass claimed closure
  from `relocate.rs:238` / `:802`; the reconciler raced the shared `move_doc` seam through
  `jigc rename` and the re-probe fired — `git mv` never ran and the doc never moved. Repro at §12,
  L-5.**]**

### 6.4 · `GIT_DIR` — the **declared-out** bound, re-driven

```
setup: TWO rigs, both roots asserted to exist before any `git -C` ran (the rc.17 instrument failure,
       guarded): A = committed-singletons, B = committed-singletons --git-state uncommitted-pick
before: A commits 5 · B commits 6 · head -1 $B/.git/MERGE_MSG -> "posture fixture: the picked commit"
argv : (cd $A && HOME=$AHOME GIT_DIR=$B/.git jigc milestone create Gd1)
observed: exit=0  "minted milestone:gd1 … record commit: 809918f"
after : A commits 5 (UNMOVED) · B commits 6 -> 7 · $B/.git/MERGE_MSG -> No such file or directory
```

**Matches the family's declared bound** (`repo.rs` module header: *"its subject is the path it is handed,
and it passes the ambient environment through untouched"*, with a written reopening condition).
Recorded as an amplifier on the declared bound, **not** a new defect — the same disposition M52 and M53
gave it.

---

## 7 · The post-review fix's seven review findings — each verified closed on rc.18

| # | the review's finding | verdict | driven |
|---|---|---|---|
| **HIGH** | `milestone discard` destroys a fan-out worktree's live git operation at exit 0, no consent, no narration — the clean-tree cell, where `git status --porcelain` fires no guard | **CLOSED at all three refusing doors** | §7.1 |
| **MEDIUM 1** | the new finding leaks an absolute host path into its message, its pinned key and its route when run from inside a linked worktree | **CLOSED** | §4.9 — 0 host-path hits, key repo-relative |
| **MEDIUM 2** | one code answering on two declared envelope arms, two streams and two exit codes at the door and at its own preview; the new target form registered in neither contract home | **CLOSED** | §4.8 — door and preview both on `blocked` at exit 3, `Here` the declared singleton, `FanOutWorktree` the declared filesystem-path form (`command-output-contract.md:247`, `finding.rs::is_declared_singleton`) |
| ⚠ **MEDIUM 3** — **DEMOTED** | *"nine of the ten `InProgress` members"* — a count with a reason naming a non-member, in six homes, past a fence a line-wrap evaded | **CLOSED** *(behavioural half only)* | **the doc-text half carries no repro block** — no `command grep` invocation or output appears anywhere in this file, so *"the numeral is struck in six homes"* is not driven as recorded. The **behavioural** half — that all ten members are handled in a worktree — **is** driven, at §4.4, and independently re-established by the reconciler (§12, L-3). A doc-text claim is in any case a source read, not a drive |
| **LOW 1** | the site clause lands after a predicate that already ends in a prepositional phrase | **CLOSED** | §4.4 — the clause **leads** at all ten members, including `UnmergedIndex` (*"in the fan-out worktree `X`, a conflict left unmerged paths in the index"*) and `Sequencer` |
| ⚠ **LOW 2** — **DEMOTED with §4.14** | the acceptance's refusing cells run against an attached worktree HEAD; production worktrees are detached | **CLOSED behaviourally** | its cited evidence is §4.14, which carries no repro block. **Re-established** by the reconciler (§12, L-11a/b): `git -C <provisioned worktree> symbolic-ref -q HEAD` → **DETACHED**, and the operation cell refuses there. The suite-content half is not a drive and is not claimed |
| **LOW 3** | `overlay_worktree`'s one-worktree-per-repository bound is prose only | **CLOSED by proxy** | §4.7 — production reaches two simultaneous breaching worktrees and answers **one finding per worktree, two discriminating keys**. The fixture-internal assertion is a test-content fact, not driven here |

### 7.1 · The HIGH, re-driven at all three refusing doors — and at the cell that was in no finding

```
setup (per door, one fresh rig each): fan-out provisioned; the operation induced INSIDE the worktree
       with a SPOTLESS TREE, asserted:  git -C $W status --porcelain  ->  []   (empty)
       members driven: `rebase -i` paused at a `break` (markers=[rebase-merge]) and `git bisect`
                        (markers=[BISECT_LOG])

jigc milestone discard h-wave        -> exit=1
  blocking · milestone.dirty-worktree — milestone:h-wave: 1 sub-task worktree path(s) hold something
    the abandon cannot prove is disposable …:
    .jigc/worktrees/beta-area: a rebase git has left un-concluded (abandon it with
      `git -C .jigc/worktrees/beta-area rebase --abort`) — it is a live git worktree git has left
      mid-operation, and the removal takes the checkout that operation can only be concluded or
      abandoned from; registered here, so the teardown removes it and this content is destroyed
    route: … or run `jigc milestone discard h-wave --force` … . A path listed above as mid-operation
      holds no bytes to move: conclude or abandon the operation in that checkout — the command that
      abandons it is on its line — and the path clears
  worktree dir after: STILL THERE

jigc uninstall                       -> exit=1  uninstall.dirty-worktree, the same listed line
  worktree dir after: STILL THERE

jigc milestone provision h-wave      -> exit=0   (a REGISTERED worktree of THIS milestone is not a
  leftover — the idempotent re-provision.  Verified non-destructive: BISECT_LOG present before AND
  after, `git -C $W bisect log` still reads the started bisect.)

the --force consent, both refusing doors:
jigc milestone discard f-wave --force  -> exit=0
  warning: removing the fan-out worktree .jigc/worktrees/delta-area discards work that is not in git:
      a rebase git had left un-concluded — it can only be concluded or abandoned from this checkout
    note: the fan-out worktree is the only copy of these bytes — they are not recoverable.
  discarded milestone:f-wave (1 sub-task(s); workbench removed)
jigc uninstall --force                 -> exit=0, the identical warning block, then the uninstall report
```

**The cell the fix's own note says was in no finding** — `milestone provision` over an **unregistered**
path holding a *second repository's* linked worktree, tree spotless, operation only:

```
setup: A = the rig; B = a second repository (mktemp -d, git init, two commits)
       jigc milestone create "P wave"; jigc milestone add-task p-wave "gamma area"   (NOT provisioned)
       git -C $B worktree add -q --detach "$REPO/.jigc/worktrees/gamma-area" HEAD
       git -C $W bisect start; git -C $W bisect bad
BEFORE: git -C $W status --porcelain -> []   (empty — no byte guard can fire)
        BISECT_LOG present

argv : jigc milestone provision p-wave        -> exit=1
  blocking · milestone.leftover-holds-work — milestone:p-wave: `jigc milestone provision` would delete
    1 path(s) it cannot prove are disposable, so nothing was removed:
    .jigc/worktrees/gamma-area: a bisect git has left un-concluded (abandon it with
      `git -C .jigc/worktrees/gamma-area bisect reset`) — it is a live git worktree git has left
      mid-operation, …
  AFTER: the directory is still there

argv : jigc milestone provision p-wave --force -> exit=0
  warning: … a bisect git had left un-concluded — it can only be concluded or abandoned from this checkout
    note: the fan-out worktree is the only copy of these bytes — they are not recoverable.
  provisioned 1 worktree(s) …
```

**Verdict: the HIGH is closed, including the largest cell.** The consent is asked, the narration names
the operation rather than counting bytes, and no door takes a live operation silently.

---

## 8 · What I did NOT drive, and why

1. **A genuine concurrent process** rather than a deterministic shim racer at any seam — the standing
   headless bound M51, M52 and M53 all declare. §4.16's racer is a shim whose fire is verified by a
   stamp.
2. **A race positioned *inside* the boundary's own apply window** — i.e. an operation opened in a
   provisioned worktree **after** `fan_out_posture_findings` runs (call 7–10 of the boundary's git
   trace, which I captured) and **before** `git diff --cached --binary` / `git apply --cached` (calls
   13–14). The window exists and is ~3 git calls wide; whether the boundary re-probes it is **not
   established here**. Recorded as un-driven, **not** as passing.
3. **The `relocate` / `config set placement-root` seam re-probe** (M51 `codex-1`). The *mechanism* was
   driven at the `task finalize` commit seam (§4.16); the two `relocate.rs` `verify` sites were not
   re-raced. Neither the rc.17 pass nor this one drove it, so it is neither a finding nor a closure.
4. **`D3b`** (a git refusal not dressed as a hook rejection). Not re-driven; M53's post-review fix
   changed no commit-seam classification and §6.2's hook arm reproduces intact.
5. **`SeamSubject::verify`'s identity-drift and expected-ref-drift legs** — only the posture leg is this
   axis's.
6. **`GIT_DIR` at 11 of the 12 acting doors** — driven at `milestone create` only (§6.4); the bound is a
   property of the *probe* and the ambient environment, declared out with a written reopening condition.
7. **Non-`main` default branch names, submodule worktrees, `core.worktree` redirects, and git versions
   other than the installed `/usr/bin/git`** — the family's existing declared git-2.54.0 deferral.
8. **The four cells my `induce.sh` maps imprecisely**: my `rebase-apply` inducer uses `git am` and
   therefore lands the `Am` member rather than `Rebase`-via-`rebase-apply/onto`. The distinction **is**
   driven — in the main checkout, at §4.2, where the rig's own builder separates them. Inside a worktree
   the `Rebase` member is driven by `rebase-merge` and `rebase-break`.
9. **`finalize.render-io`'s host-absolute `at:` path**, hit incidentally at §4.6's first fixture
   attempt. **Not reported:** `crates/engine/src/finalize.rs` is a declared row of
   `crates/cli/tests/repo_relative_paths.rs`'s `UNSWEPT_PRODUCERS` (`render_io` named explicitly) — a
   counted, fenced, declared remainder. Graded against, not re-found.
10. **F-2's variant where the squatting worktree belongs to the *same* repository** (a user's own
    `git worktree add .jigc/worktrees/<id>`) — not driven; the foreign-repository cell is the harder one
    and is the one recorded.
11. **The five axes NOT in this re-run** (1, 4, 6, 7, 8) — out of scope by the charter; nothing here
    says they hold, only that they were not asked.

---

## 9 · What this adds over flow-54 arm 4

Arm 4 iterates **the four new `GitState` variants × `BEHALF_DOORS`' acting rows** — a manufactured
fixture set crossed with a total classification — asserting that every acting door refuses with the
member's noun, that `.git/MERGE_MSG` (resolved per worktree) and the index are byte-unchanged, that the
conclude phrase carries no *"once its conflicts are resolved"* on the clean cells, and that the abandon
argv runs verbatim to exit 0 in all four states.

This axis adds six things the arm structurally cannot:

1. **The second subject, over its whole member set.** Arm 4 induces its state in the repository the
   door is run in. §4.4 induces **all ten** `InProgress::ALL` members in a *provisioned sub-task
   worktree* and asks the boundary — the subject the post-review fix minted. An arm crossing
   `GitState::ALL` with the door set proves every *member* is handled at every *door*; it cannot ask
   whether a *second* subject exists, which is precisely how the rc.17 tier-1 row survived a green arm.
2. **Both commit models driven to their conclusion.** §4.6 refuses under `squash: false` **and** lands
   the per-sub-task commit after the route runs — the bound both the rc.17 driver and its reconciler
   recorded as un-driven.
3. **The route's claims tested against the world rather than against the string.** §4.5 and §5 run the
   emitted command verbatim and measure what happens — which is how F-1 was found, and how §7.1's
   `--force` narration was checked against what actually disappeared.
4. **The false-positive hunt inside the new subject.** §4.13 drives the states the fixture builder does
   not build — a clean `git stash apply`, a conflicting `git stash pop`, a clean `git revert -n`, and
   two user-concluded operations — *inside a provisioned worktree*, which is where the new probe's
   false-positive risk lives. The arm iterates the members it has; it cannot show that nothing benign
   has joined them.
5. **The destroying doors' second leg at a spotless tree.** §7.1 drives the cell no byte-counting guard
   can see (`git status --porcelain` → empty), at all three refusing doors and at the unregistered
   second-repository path, without and with `--force`.
6. **The composite precedence.** Posture vs `PRE_DISPATCH_FAULTS`, vs `setup`'s unborn exemption, vs
   `RelocateRefusal`, vs the `DESTROYING_DOORS` cross, vs `WORKTREE_DOORS`' exemption of
   `task discard`, vs the invocation log and **both** `--format json` arms — seven registries meeting
   one guard, asserted once each, where the arms own their registries separately.

---

## 10 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row above, `VERB_KINDS` spelling.

**`BEHALF_DOORS ▸ CommitsOnBehalf` (10):** `setup` · `migrate-corpus` · `rename` · `task discard` ·
`task finalize` · `milestone create` · `milestone add-task` · `milestone add-from-spec` ·
`milestone finalize` · `milestone discard`

**`BEHALF_DOORS ▸ MovesOnBehalf` (2):** `relocate` · `config set`

**`BEHALF_DOORS ▸ Neither` (35), driven as controls under the breach** (34 silent — `repo.*` hits 0 —
and `task validate`, which answers by the stated preview design): `start` · `workflow` · `uninstall` ·
`upgrade` · `ingest` · `migrate` · `unmanage` · `describe` · `validate` · `doc create` ·
`doc add-item` · `doc remove-item` · `doc retitle-item` · `doc rename` · `doc set-field` ·
`doc set-slot` · `doc author` · `doc show` · `doc schema` · `doc list` · `task list` · `task diff` ·
`task validate` · `task bind` · `config insert-step` · `config replace-step` · `config remove-step` ·
`config fill` · `config fork` · `config get` · `config list` · `milestone list-tasks` ·
`milestone provision` · `milestone execute` · `milestone join`

**Total: 47 of 47 — `uncovered: none`.**

*Method note on the control sweep:* nine argvs first exited **2** (clap usage — not a driven row, since
dispatch never ran). Each was corrected against its own `--help` and re-driven; only the corrected
drives are counted. One — `config insert-step` — needed both `--workflow` and `--after` before it
reached dispatch (exit 1, `repo.*` hits 0).

Beyond the control sweep, these leaves carry **condition-specific** rows: all 12 acting doors (§4.3,
§4.10), `milestone provision` (§7.1, both the registered and the unregistered-foreign cells),
`milestone join` and `milestone execute` (§4.13, §7 — driven under a breaching worktree, both exit 0
with zero `repo.*`, which is consistent with their `Neither` classification: neither moves HEAD —
measured), `task validate` (both `BreachSite`s, §4.5 and §4.11), `task list` / `doc list` (state
verification and the precedence cell), and `workflow` (the sub-task re-entry that provisions the commit
doc at §4.6).

---

## 11 · Instrument honesty

Three things on the record rather than buried.

1. **One exit code was read through a pipe** — `jigc task validate … | head -2` reported the *head's*
   `0`. Caught, re-measured bare (**exit 1**), and only the bare figure is recorded. It is the same
   family as the `cargo test | tail` rule this repo's dev tooling exists to enforce.
2. **Two false-positive control cells were initially built wrong.** The milestone id was derived with
   `ls .jigc/milestones | head -1`, and in two rigs the worktree was never built — so the `exit 0` those
   cells produced was a docs-only boundary, not a control over a concluded operation. Both were rebuilt
   with a fixed milestone id and the worktree's existence asserted first; only the rebuilt drive is in
   §4.13.
3. **The `GIT_DIR` cell was built by eval-ing each rig in a subshell and reading `$REPO` back**, with
   both roots asserted to be existing directories before any `git -C` ran — the rc.17 axis-2 instrument
   failure (a `sed` parse of the rig's stdout that silently produced empty paths and ran three git
   commands in the working repository) explicitly designed against. `git status` in
   `/Users/maurice/projects/gherrink-jigc` was the same at the end of this review as at the start: one
   modified file (`completions/artifacts/M53/per-axis-review-rc18/instrument/per-axis-review.workflow.js`),
   which was already modified before the first drive. **Nothing was fixed, committed or edited.**

---

# 12 · Reconciliation ledger

**The rule applied** (`acceptance-design.md` → *The reconciliation rule*): a claim by one pass that the
other cannot reproduce is a **LEAD**, never a finding. Every Codex claim below was entered as
`lead(codex, …)` and then **driven** — to a repro block (CONFIRMED, origin `codex`) or to a recorded
refutation carrying the falsifying datum. Every driver **defect** stays a finding, because it was
driven; each was **re-driven once by this reconciler** to confirm its repro block. A claim that could
not be driven at all stays an **OPEN LEAD** with the reason.

**All reconciler drives**: binary `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.18` (asserted),
repo HEAD `271b0cb7`, 2026-09-23, macOS, `/usr/bin/git` (Apple Git-157 / 2.54.0). Fixtures
`dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc`, two-step eval, every root from
`mktemp -d`. No teardown, no `rm -rf` on a variable path, nothing written by hand into `.jigc/`.

## 12.1 · Codex source-pass claims

### L-1 · Claim 1 — **CONFIRMED** (origin **codex**, and independently the driver's §6.1)

> *"M52 DEFECT B remains: a conflicted squash merge is classified as a completed staged squash rather
> than as an unmerged-index conflict."*

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"
       printf 'base\n' > conf.txt;  git add conf.txt; git commit -q -m 'conf seed'
       git checkout -q -b cb; printf 'theirs\n' > conf.txt; git add conf.txt; git commit -q -m theirs
       git checkout -q -;    printf 'ours\n'   > conf.txt; git add conf.txt; git commit -q -m ours
       git merge --squash cb                                   -> rc=1  (CONFLICT)
state: markers            -> MERGE_MSG SQUASH_MSG
       git ls-files -u    -> 3            <- the index IS conflicted
argv : jigc milestone create CS1
  -> exit=1
     blocking · repo.operation-in-progress — a squash merge is staged and not committed —
       the repository is not in a committable state
       route: conclude it, or abandon it with `git reset --merge` (…), then re-run this command
route still effective: git reset --merge -> rc=0 ; git ls-files -u -> 0 ; door after -> exit 0
```

The noun asserts the index is **staged**; `git ls-files -u` says **3 paths are unmerged**, and the
conclude arm carries no command. Reproduces exactly as Codex predicted and as the driver recorded.
**Disposition is unchanged**: M52 `(2, DEFECT B)`, tier 3, already triaged to the 1.x ledger by the
charter — confirmed as **still open**, *not* a new finding.

### L-2 · Claim 2 — **CONFIRMED** (origin **codex**, and independently the driver's §4.16/§6.2)

> *"M52 DEFECT C remains: a posture race raised at a commit seam bypasses the committing door's
> survivable frame."*

Codex's proposed reproduction (a git shim that starts `git bisect` mid-transaction) was run verbatim
in shape:

```
setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "seam probe" \
             --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       printf 'work\n' > work.txt; git add work.txt
       jigc doc set-field commit:seam-probe#header/type --task seam-probe --value feat   -> 0
       printf 'the seam subject\n' | jigc doc set-slot commit:seam-probe#summary \
              --task seam-probe --from-file -                                            -> 0
       jigc task validate seam-probe  -> "no findings — the task validates clean", exit 0 (measured BARE)
       a deterministic `git` shim first on PATH, firing `git bisect start` in $REPO once, on the
       first `git add` of the run, then exec'ing /usr/bin/git.
BEFORE: HEAD=6f7ff7e   staged=work.txt
argv : jigc task finalize seam-probe
  -> exit=1
     blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
       committable state
       route: conclude it, or abandon it with `git bisect reset`, then re-run this command
     shim log: "status: waiting for both good and bad commits" / "shim: fired [bisect start] rc=0"
AFTER (true, and said by NOTHING on that surface):
  HEAD                        6f7ff7e -> 6f7ff7e   (UNMOVED)
  git diff --cached           work.txt still staged
  .jigc/tasks/seam-probe/docs commit:seam-probe.md · provenance.json  (INTACT)
  .git/BISECT_LOG             present

CONTRAST ARM, same door, one cause apart — a rejecting pre-commit hook, fresh rig:
  `git commit` was rejected (no commit was made):
  the hook says no

  task hook-probe is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/hook-probe/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize hook-probe`.
  exit=1
```

Two in-transaction failure causes at one seam: the hook arm prints the **state-truth clause** *and* the
**copy-runnable re-run**; the posture arm prints **neither**, offering only *"re-run this command"*.
**Disposition unchanged**: M52 `(2, DEFECT C)`, tier 2, triaged to the 1.x ledger — confirmed **still
open**, not a new finding. *(The same arm also re-drives M51's `D3b`: the hook frame is intact and a
git/hook commit failure is still typed as a commit failure, so `D3b` stays **CLOSED**.)*

### L-3 · Disposition *"`(2, DEFECT 1)` — CLOSED"* — **CONFIRMED** (agrees with the driver's §4.4/§4.5)

The rc.17 tier-1 row. Re-driven on the exact cell that defined it — the swallow fixture:

```
setup: rig committed-singletons
       jigc milestone create "Swallow milestone"
       jigc milestone add-task swallow-milestone "swallow sub intent"
       jigc milestone provision swallow-milestone
       W=$REPO/.jigc/worktrees/swallow-sub-intent            (asserted to exist)
       printf 'sub work\n'      > $W/subwork.txt; git -C $W add subwork.txt
       printf 'PICKED PAYLOAD\n'> $W/picked.txt;  git -C $W add picked.txt
       git -C $W commit -m 'the users authored pick message'; P=$(git -C $W rev-parse HEAD)
       git -C $W reset --hard HEAD~1
       printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt
       git -C $W cherry-pick -n "$P"                          -> rc=0
BEFORE: head -1 <worktree git-dir>/MERGE_MSG -> the users authored pick message
        git -C $W diff --cached --name-only  -> picked.txt subwork.txt
        git -C $REPO status --porcelain      -> []            (main checkout clean)
        A-HEAD = d162cbe

argv : jigc task validate swallow-sub-intent        (the preview, from the MAIN checkout)
  -> exit=3   blocking · repo.operation-in-progress — in the fan-out worktree
     `.jigc/worktrees/swallow-sub-intent`, an uncommitted cherry-pick is in progress — the milestone
     boundary commits that worktree's index, and it is not in a committable state
     at: .jigc/worktrees/swallow-sub-intent
argv : jigc milestone join swallow-milestone        -> exit 0
       jigc milestone finalize swallow-milestone    -> exit=3, the SAME finding, byte-identical
AFTER: A-HEAD d162cbe -> d162cbe (UNMOVED) · MERGE_MSG "the users authored pick message" (INTACT) ·
       worktree present · git cat-file -p HEAD:picked.txt -> fatal: path does not exist in 'HEAD'
```

Nothing durable is written, the authored message survives, and the door and its preview answer with the
same code at the same exit on the same arm. **CLOSED**, driven, as both passes say.

The **site clause leads** at the two members whose predicate ends in a prepositional phrase — the
post-review review's LOW 1, driven in a provisioned worktree by `driver/induce.sh`:

```
unmerged-index  markers []                              unmerged 3 ->
  "in the fan-out worktree `.jigc/worktrees/sub-unmerged-index`, a conflict left unmerged paths in
   the index — …"                                                                        exit=3
sequencer       markers MERGE_MSG CHERRY_PICK_HEAD sequencer  unmerged 3 ->
  "in the fan-out worktree `.jigc/worktrees/sub-sequencer`, a cherry-pick is in progress — …"  exit=3
merge           markers MERGE_HEAD MERGE_MSG           unmerged 3 ->
  "in the fan-out worktree `.jigc/worktrees/sub-merge`, a merge is in progress — …"       exit=3
```

### L-4 · Disposition *"M52 DEFECT A — CLOSED"* — **CONFIRMED**

```
setup: rig=$(dev/jigc-rig committed-singletons --git-state uncommitted-pick \
             --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
argv : jigc milestone create Az1
  -> exit=1  blocking · repo.operation-in-progress — an uncommitted cherry-pick is in progress
             route: conclude it with `git commit` (…), or abandon it with `git reset` (…)
       jigc task discard <t> --force        -> exit=1
       jigc setup --force                   -> exit=1
       jigc migrate-corpus --dry-run        -> exit=1
AFTER the sweep: HEAD unmoved=yes · .git/MERGE_MSG md5 identical=yes · .git/index md5 identical=yes
```

`UncommittedCherryPick` is a real member, no consent flag reaches past the guard, and nothing moved.

### L-5 · Disposition *"M51 codex-1 — CLOSED"* — **CONFIRMED, and it CLOSES A LEAD THE DRIVER LEFT OPEN**

Codex closed this on a source read (`relocate.rs:790` for the squatter `git rm --cached`, `:238` for
`git mv`); the driver's §7.3 and §6.3 explicitly recorded it **not re-raced**, so it was the one row
where the two passes disagreed about whether anything is known. Under the rule it was a lead, and it
was **driven** — by racing the **shared** `move_doc` primitive (`relocate.rs:210`, whose
`SeamSubject::live(repo_root).verify(SeamAct::Move)` at `:238` is the site Codex cites) through
`jigc rename`, which reaches it:

```
CONTROL — the unraced git trace of the move (a logging-only shim on PATH):
   … 9  ls-files --stage -z -- docs docs/research
    13  symbolic-ref -q HEAD
    14  ls-files -u        ┐
    15  rev-parse --verify │ the re-probe
    17  symbolic-ref       ┘
    18  mv docs/research/context-loss.md docs/research/ctx-loss.md      <- the act
    26  commit -F …
  -> exit 0, "renamed research:context-loss -> research:ctx-loss"

RACE — same fixture, a shim firing `git bisect start` in $REPO once, on the FIRST
`ls-files --stage` (call 9: after the door-top guard at calls 1-3, before the re-probe at 14):
setup: rig=$(dev/jigc-rig refs-post-hoc --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc task discard ground-the-vision-in-research --force        (rename refuses in-flight tasks)
BEFORE: docs/research -> context-loss.md ; HEAD=3eda4cd
argv : jigc rename research:context-loss --to "Context Loss Revisited" --slug ctx-loss
  -> exit=1
     blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
       committable state
       route: conclude it, or abandon it with `git bisect reset`, then re-run this command
     shim log: "shim: fired [-C <repo> bisect start] rc=0"
     trace ENDS at:  13 symbolic-ref · 14 symbolic-ref · 15 rev-parse --verify ·
                     16 restore --staged docs/research/context-loss.md ·
                     17 restore --staged docs/research/ctx-loss.md
                     ** no `git mv`, no `git commit` **
AFTER: docs/research -> context-loss.md (UNMOVED) · HEAD=3eda4cd (UNMOVED) · BISECT_LOG present
```

The re-probe fires inside the window, the move never runs, and the staged pre-image is rolled back.
**`codex-1` is CLOSED, driven** — a closure neither pass alone could record.

### L-6 · Disposition *"M51 D1 — CLOSED"* (operation answered, not the detachment) — **CONFIRMED**

### L-7 · Disposition *"M51 D2 — CLOSED"* (`git am` vs apply-backend rebase are disjoint) — **CONFIRMED**

L-6 and L-7 share one drive — `jigc milestone create` in a fresh rig per git state:

```
rebase-merge        DETACHED  exit=1  "a rebase is in progress"     [git rebase --continue/--abort]
rebase-apply        DETACHED  exit=1  "a rebase is in progress"     [git rebase --continue/--abort]
am                  attached  exit=1  "a `git am` is in progress"   [git am --continue/--abort]
detached            DETACHED  exit=1  repo.head-detached            [git switch <branch>]
bisect              attached  exit=1  "a bisect is in progress"     [git bisect reset]
unmerged-index      attached  exit=1  "a conflict left unmerged paths in the index" [git reset --merge]
dangling-sequencer  attached  exit=1  "a cherry-pick or revert left a queue of commits in `sequencer/`"
                                                                    [git cherry-pick --quit]
```

Two HEAD-detached states answer the **operation**, not the detachment; `detached` is the only state that
reaches `repo.head-detached`; `am` and the apply-backend rebase carry **two nouns and two abort
commands**. Both rows **CLOSED**, driven.

### L-8 · Completeness claim *"`BEHALF_DOORS` remains total; `COMMITTING_DOORS` carries both milestone-finalize commit models"* — **CONFIRMED** (source read + drive)

Source read at HEAD `271b0cb7`, bracket-matched over each array literal, so the driver's §1 counts are
independently reproduced rather than taken:

```
crates/cli/src/cli.rs:1999  BEHALF_DOORS   -> 47 rows  (CommitsOnBehalf 10 · MovesOnBehalf 2 · Neither 35)
crates/cli/src/repo.rs      PostureMember::ALL -> 3
crates/cli/src/repo.rs      InProgress::ALL    -> 10   (…, UncommittedCherryPick, UnmergedIndex)
crates/cli/src/invocation_log.rs  COMMITTING_DOORS -> 10
```

The behavioural half of the `--format json` preview arm (the driver's demoted §4.8 row 3) drove to:

```
setup: milestone wt-wave, sub-tasks wt-one + wt-two, both provisioned;
       git -C $REPO/.jigc/worktrees/wt-two bisect start && … bisect bad
argv : jigc --format json task validate wt-two
  -> exit=3   stdout 727 bytes   stderr 0 bytes
     {"schema_version":3,"findings":[{"severity":"blocking","probe":"repo",
       "check":"operation-in-progress","code":"repo.operation-in-progress",
       "key":{"code":"repo.operation-in-progress","target":".jigc/worktrees/wt-two"},
       "message":"in the fan-out worktree `.jigc/worktrees/wt-two`, a bisect is in progress — …",
       "location":{"address":".jigc/worktrees/wt-two","line":1,"col":1},
       "route":"conclude it, or abandon it with `git -C .jigc/worktrees/wt-two bisect reset`, …"}]}
```

### L-9 · The `BreachSite::Here` `--format json` arm (the driver's demoted §4.8 row 1) — **CONFIRMED by the reconciler's own drive**

```
setup: rig committed-singletons --git-state uncommitted-pick
argv : jigc --format json milestone create Zz9
  -> exit=1   stdout 0 bytes   stderr 445 bytes
     {"error":"blocking · repo.operation-in-progress — an uncommitted cherry-pick is in progress — …"}
```

One key, `error`; nothing on stdout. The declared-singleton shape, as recorded.

### L-10 · The detached / unborn cells (the driver's demoted §4.10 rows) — **CONFIRMED by the reconciler's own drive**

```
setup: rig committed-singletons --git-state detached      (HEAD detached at 37d3bd8 / b359656)
  jigc setup                                    -> exit=1  repo.head-detached  [git switch <branch>]
  jigc migrate-corpus                           -> exit=1  repo.head-detached
  jigc rename research:nope --to Zz --slug zz   -> exit=1  repo.head-detached
  jigc milestone create Dz1                     -> exit=1  repo.head-detached
  jigc relocate vision --from docs/old-vision.md-> exit=1  relocate.frozen-doctype   (past the guard)
  jigc config set docs-root docs2               -> exit=0  knob written; config get -> docs2 (project)

setup: rig --git-state unborn                            (commits before: 0)
  jigc migrate-corpus  -> exit=1 repo.head-unborn   |  jigc milestone create -> exit=1 repo.head-unborn
  jigc setup           -> exit=0 ; commits after: 1        <- the stated Exempt(HeadUnborn) row
```

*(Instrument honesty: the first pass of this drive iterated the argvs through an unquoted shell
variable, so two multi-word argvs reached clap as one token and exited 2 — a usage error, not a driven
row. Both were re-driven as separate quoted argvs; only the corrected drive is recorded above.)*

### L-11 · The dedicated-worktree cross (the driver's demoted §4.14, all five rows) — **CONFIRMED by the reconciler's own drive**

```
setup: rig committed-singletons; milestone wt-wave with sub-tasks wt-one + wt-two, provisioned
       git -C $REPO/.jigc/worktrees/wt-two bisect start && … bisect bad
  a) (cd .jigc/worktrees/wt-one && jigc milestone create "Inside one")
       git -C wt-one symbolic-ref -q HEAD -> DETACHED
       -> exit=0, "minted milestone:inside-one", NO repo.head-detached
  b) (cd .jigc/worktrees/wt-two && jigc milestone create "Inside two")
       -> exit=1  repo.operation-in-progress — a bisect is in progress   (the Here site, un-aimed route)
  c) jigc milestone create "Main side"          (from the MAIN checkout)  -> exit=0, minted
  d) jigc milestone finalize wt-wave            -> exit=3  repo.operation-in-progress @ FanOutWorktree
                                                   at: .jigc/worktrees/wt-two
  e) a worktree git cannot vouch for: printf 'gitdir: /nonexistent/place\n' > $W/.git, x.txt staged first
       git -C $W rev-parse --show-toplevel -> fatal: not a git repository: (null)
       jigc milestone finalize corrupt-wave -> exit=3  milestone.zero-contribution
       git cat-file -p HEAD:x.txt           -> fatal: path 'x.txt' does not exist in 'HEAD'
       (no panic; the unvouchable worktree is excluded from BOTH the probe and the commit)
```

Row (a) is also the drive that re-establishes the demoted **LOW 2**: production's provisioned worktree
is git's own `--detach`, and (b) shows the operation cell refusing there.

### L-12 · Codex's remaining completeness statements — **OPEN LEADS, source-only, not driveable here**

Codex also states, from the source: that `DedicatedWorktree`'s discriminant is private and its sole
constructor requires a linked-worktree `.git` file plus canonical location plus registered ownership;
that the `setup` unborn exemption does not leak; that no production HEAD-changing `git switch` /
`git checkout` exists; and that the post-rc.18 fix commits move **no** schema-hash, `schema-version`,
`contract-version` or envelope key. Each is a claim about **what the source does not contain**. A drive
can exhibit an instance but cannot exhibit an absence, so these stay **OPEN LEADS (source-only)** — the
observable halves that *are* driveable were driven and hold: the unborn exemption is visible as `setup`
moving the repository from 0 to 1 commits while every other acting door refuses (L-10), and the typed
worktree exemption is visible at L-11a/b. No promotion on the source read, and nothing dropped.

## 12.2 · Driver defects

### D-F1 · **F-1** (tier 2) — the aimed route `git -C <repo-relative-path>` does not run from the checkout that printed it — **STANDS**, re-driven

The Codex pass is **silent** on this row: its "Relevant production acts remain guarded" section asserts
the re-probes exist, which is about *whether the guard fires*, not about *whether the emitted remedy
runs*. Silence is not a refutation, and the row was driven, so it stays a finding. Re-driven once:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"
       jigc milestone create "Cwd wave"; jigc milestone add-task cwd-wave "cw area"
       jigc milestone provision cwd-wave                       (worktree asserted to exist)
       W=$REPO/.jigc/worktrees/cw-area; git -C $W bisect start; git -C $W bisect bad
       mkdir -p $REPO/docs/deep && cd $REPO/docs/deep     <- an ORDINARY subdirectory of the checkout

argv : jigc milestone discard cwd-wave        [the milestone::held_here producer]
  -> blocking · milestone.dirty-worktree … `.jigc/worktrees/cw-area`: a bisect git has left
     un-concluded (abandon it with `git -C .jigc/worktrees/cw-area bisect reset`) …
argv : jigc milestone finalize cwd-wave       [the BreachSite::aim producer]
  -> blocking · repo.operation-in-progress … route: conclude it, or abandon it with
     `git -C .jigc/worktrees/cw-area bisect reset`, then re-run this command

THE EMITTED COMMAND, RUN VERBATIM FROM THE CWD THAT PRINTED IT:
  git -C .jigc/worktrees/cw-area bisect reset
    fatal: cannot change to '.jigc/worktrees/cw-area': No such file or directory
    rc=128                                          <- both producers, identically

CONTROL, the same command from the repository root:
  cd $REPO && git -C .jigc/worktrees/cw-area bisect reset
    HEAD is now at 55b1b7e docs(changelog): cut the first release
    rc=0
```

Confirmed at both producers. Tier 2 as the driver graded it: every door **refused**, so nothing is lost
or committed; what fails is the remedy the refusal prints.

### D-F2 · **F-2** (tier 3) — `jigc milestone finalize` commits a second repository's worktree bytes and acks them as the sub-task's work — **STANDS**, re-driven

Codex is silent here too; its posture analysis concerns whether a breach is detected, and in this cell
there **is** no breach — the foreign worktree is clean and concluded, so the probe correctly passes. The
defect is the **subject**, not the posture verdict.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"
       B=$(mktemp -d "${TMPDIR:-/tmp}/repoB.XXXXXX"); git -C $B init -b main; two commits in B
       jigc milestone create "Foreign wave"; jigc milestone add-task foreign-wave "fw area"
       # NO `jigc milestone provision` — B's own worktree is parked at the sub-task path instead:
       git -C $B worktree add -q "$REPO/.jigc/worktrees/fw-area" -b fwbranch
       printf 'B-SECRET-PAYLOAD\n' > $W/bsecret.txt; git -C $W add bsecret.txt
       printf 'B-UNTRACKED-KEEP\n'  > $W/keepme.txt
BEFORE: git -C $W rev-parse --show-toplevel -> the squatted path itself
        git -C $W symbolic-ref -q HEAD      -> refs/heads/fwbranch      (B's branch, ATTACHED)
        git -C $W diff --cached --name-only -> bsecret.txt
        A-HEAD = d9f4cd6
argv : jigc milestone join foreign-wave   -> exit 0, "0 doc(s) merged  /  no docs staged from: fw-area"
       jigc milestone finalize foreign-wave
  -> exit=0
     finalized 3e2d426 — Finalize milestone foreign-wave (1 sub-task)
       added bsecret.txt
       modified docs/milestone-records/foreign-wave.md
       2 files committed
       sub-tasks: fw-area: 1 code file          <- B's bytes, acked as the sub-task's own work
AFTER: git -C $REPO cat-file -p HEAD:bsecret.txt -> B-SECRET-PAYLOAD
       $W/keepme.txt -> B-UNTRACKED-KEEP (intact) · B's worktree registration intact ·
       .jigc/displaced/ -> absent
```

**No byte is lost** — B's index, its untracked plant and its registration all survive. What is wrong is
the **ack**: a file belonging to another repository is named as this sub-task's work. Tier 3 stands, and
the driver's own note that a reader could argue tier 1 (*repository harm through a committing door*) is
carried forward rather than dropped.

## 12.3 · Summary

| # | origin | claim | disposition |
|---|---|---|---|
| L-1 | codex | M52 DEFECT B still open (conflicted squash misclassified) | **CONFIRMED** (repro) — pre-existing, already 1.x-ledgered |
| L-2 | codex | M52 DEFECT C still open (commit-seam race skips the survivable frame) | **CONFIRMED** (repro + contrast arm) — pre-existing, already 1.x-ledgered |
| L-3 | codex | `(2, DEFECT 1)` CLOSED | **CONFIRMED** (repro) |
| L-4 | codex | M52 DEFECT A CLOSED | **CONFIRMED** (repro) |
| L-5 | codex | M51 `codex-1` CLOSED | **CONFIRMED** (repro) — **closes a lead the driver left open** |
| L-6 | codex | M51 `D1` CLOSED | **CONFIRMED** (repro) |
| L-7 | codex | M51 `D2` CLOSED | **CONFIRMED** (repro) |
| L-8 | codex | `D3` CLOSED · `D3b` CLOSED · registries total | **CONFIRMED** (source read reproduced + repro) |
| L-9 | driver (demoted) | the `Here` `--format json` arm | **CONFIRMED** by the reconciler's drive |
| L-10 | driver (demoted) | the detached / unborn cells | **CONFIRMED** by the reconciler's drive |
| L-11 | driver (demoted) | the dedicated-worktree cross, all five rows | **CONFIRMED** by the reconciler's drive |
| L-12 | codex | four *absence* claims (private discriminant, no leak, no HEAD-changing checkout, no schema movement) | **OPEN LEAD (source-only)** — a drive cannot exhibit an absence |
| D-F1 | driver | F-1, tier 2, the unrunnable aimed route | **STANDS** — re-driven, both producers, rc=128 |
| D-F2 | driver | F-2, tier 3, a foreign repository's bytes committed and acked | **STANDS** — re-driven, exit 0 |

**REFUTED: none.** Neither pass contradicted a driven observation of the other; the only genuine
disagreement was about *codex-1*, where one pass had read and the other had not driven — and driving it
settled it (L-5).

**Tier-1 count after reconciliation: 0.** Unchanged from the driver's headline.

---

# 13 · Doors covered (reconciled)

Every clap leaf that is the door of ≥1 **driven** row in the reconciled file — the driver's rows that
retain a repro block, plus every row re-established by the reconciler's own drives at §12.
`VERB_KINDS` spelling.

**`BEHALF_DOORS ▸ CommitsOnBehalf` (10 of 10):** `setup` · `migrate-corpus` · `rename` ·
`task discard` · `task finalize` · `milestone create` · `milestone add-task` ·
`milestone add-from-spec` · `milestone finalize` · `milestone discard`

**`BEHALF_DOORS ▸ MovesOnBehalf` (2 of 2):** `relocate` · `config set`

**`BEHALF_DOORS ▸ Neither` (35 of 35), driven as controls under the breach:** `start` · `workflow` ·
`uninstall` · `upgrade` · `ingest` · `migrate` · `unmanage` · `describe` · `validate` · `doc create` ·
`doc add-item` · `doc remove-item` · `doc retitle-item` · `doc rename` · `doc set-field` ·
`doc set-slot` · `doc author` · `doc show` · `doc schema` · `doc list` · `task list` · `task diff` ·
`task validate` · `task bind` · `config insert-step` · `config replace-step` · `config remove-step` ·
`config fill` · `config fork` · `config get` · `config list` · `milestone list-tasks` ·
`milestone provision` · `milestone execute` · `milestone join`

**Total: 47 of 47 — `uncovered: none`.**

Doors the **reconciler itself** drove at least one row at (a proper subset of the above):
`setup` · `migrate-corpus` · `rename` · `task discard` · `task finalize` · `task validate` ·
`milestone create` · `milestone add-task` · `milestone provision` · `milestone join` ·
`milestone finalize` · `milestone discard` · `relocate` · `config set` · `config get` · `doc list` ·
`doc set-field` · `doc set-slot`.
