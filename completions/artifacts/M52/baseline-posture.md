# M52 baseline — area `posture` (axis 2: the posture family × every acting door)

**Provenance.** Binary `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15`,
sha256 `126f1584f183636bb6cd9e782b1dc26aca28dd1afaf2fb83da1fd0e5febc8fa9`, built from `577a0099`
(RELEASE: the `#[cfg(debug_assertions)]` route fence does not exist here). Repo HEAD `7637a46f`,
tree clean, **no cargo run, no edit to the working repo**. Date 2026-09-16/17. git **2.54.0**
(Apple Git-157) — every marker fact below is a property of this git and is stated as such.
Fixtures: `dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc`, two-step eval, three rigs
(`rigA`, `rigC`, plus two bare `mktemp -d` git repos for the marker census). No teardown anywhere;
every root is a `mktemp -d`. Probe scripts + all captured output:
`scratchpad/posture/{lib.sh,sweep12.sh,fanout.sh}` and each rig's `$RIG/out/`.

Assigned rows: D1 · D2 · D3 · D3b · codex-1 · open lead 3 (§D). **None was re-verified** (the brief
forbids it); every drive below is a *class* member the review's rows do not carry.

---

## §1 · The class enumeration

### 1.1 The git states (the class's real axis), read from git's own marker set

`git help` + driven census in two throwaway repos (`scratchpad/gitmarkers*.XXXXXX`). Columns:
the marker git writes, HEAD after, whether `InProgress::markers()` carries it.

| # | git state (how a user gets there) | marker(s) in the worktree git dir | HEAD | in the family? |
|---|---|---|---|---|
| 1 | `git merge` (conflict or `--no-commit`) | `MERGE_HEAD` + `MERGE_MSG` | attached | **yes** — `Merge` |
| 2 | `git merge --squash` (conflict or clean) | **`MERGE_MSG` + `SQUASH_MSG` only** | attached | **NO** |
| 3 | `git rebase` (merge backend, the default) | `rebase-merge/` | **detached** | yes — `Rebase` |
| 4 | `git rebase --apply` (apply backend) | `rebase-apply/` — contains **`onto`**, no `applying` | **detached** | yes — `Rebase` |
| 5 | `git am` (conflict) | `rebase-apply/` — contains **`applying`**, no `onto` | **attached** | yes — `Rebase` |
| 6 | `git cherry-pick <one>` (conflict) | `CHERRY_PICK_HEAD` + `MERGE_MSG` | attached | **NO** |
| 7 | `git cherry-pick <a> <b>` (conflict) | `CHERRY_PICK_HEAD` + **`sequencer/`** | attached | **NO** |
| 8 | `git revert <one>` (conflict) | `REVERT_HEAD` + `MERGE_MSG` | attached | **NO** |
| 9 | `git revert <a> <b>` (conflict) | `REVERT_HEAD` + **`sequencer/`** | attached | **NO** |
| 10 | `git bisect start` (before any good/bad) | `BISECT_LOG` + `BISECT_START` | **attached** | yes — `Bisect` |
| 11 | `git bisect start; bad; good` | `BISECT_LOG` + `BISECT_START` | **detached** | yes — `Bisect` |
| 12 | `git switch --detach` | — | detached | n/a (`HeadDetached`) |
| 13 | `git init`, no commit | — | unborn | n/a (`HeadUnborn`) |
| 14 | conflicted `git stash pop` / conflicted `--squash` | **none** (unmerged index entries only) | attached | **NO** |

**Eight distinct operations git can leave un-concluded; the family carries four markers over three
operations.** Two facts the review's rows do not carry:

* **`rebase-apply` is ambiguous *and git itself discriminates it*** — `.git/rebase-apply/applying`
  exists for `git am` and not for `git rebase --apply`; `onto` is the converse. That is why
  `git rebase --abort` can say *"It looks like 'git am' is in progress"*. Driven, both directions.
* **A squash merge writes no marker at all** (row 2) and an ordinary conflicted stash-pop writes
  none either (row 14), so no marker-set widening can reach them.

### 1.2 The code-side registries (grep, hit count, what the pattern misses)

| registry | file | hits | count |
|---|---|---|---|
| `BEHALF_DOORS` rows (`grep -c 'door: &\['` inside the const) | `crates/cli/src/cli.rs:1838` | 47 | **47** leaves, total classification |
| ↳ `CommitsOnBehalf` | | 10 | **10** |
| ↳ `MovesOnBehalf` | | 2 | **2** |
| ↳ `Neither` | | 35 | **35** |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:130` | 10 | **10** rows / 9 leaves |
| `PostureMember::ALL` | `crates/cli/src/repo.rs:132` | — | **3** (`HeadDetached`, `HeadUnborn`, `OperationInProgress`, in that order) |
| `InProgress::ALL` / `markers()` | `crates/cli/src/repo.rs:166` | — | **3** operations over **4** markers |
| `GATE_COVERAGE` | `crates/cli/src/gate_coverage.rs:119` | 12 | **12** rows — **`grep -in 'posture\|repo\.'` → 0 hits** (see §4 L1) |

**The ordering that produces D1 is in two places, both taking the first match:**
`repo.rs:posture()` pushes `HeadDetached` → `HeadUnborn` → the operations, and both consumers
(`cli.rs:534 refuse_on_posture`, `repo.rs:~485 SeamSubject::verify`) use `.find(…)`. The movers
filter to `OperationInProgress`, which is why the right answer is in the binary one class over.

### 1.3 Index-mutating production sites (codex-1's real class)

`grep -rn 'vec!\["add"\|&\["add"\|&\["mv"\|"rm",' crates/cli/src/*.rs`, test modules removed by
hand (the pattern **misses** any future `git update-index` / `git restore --staged` /
`git stash` call and any argv built from a `String` vec with no literal head — I did not find such
a site, but the pattern cannot prove it):

| # | site | act | nearest preceding posture probe |
|---|---|---|---|
| 1 | `relocate.rs:79` | `git mv` | **immediately before** (`move_doc`, `SeamAct::Move`) |
| 2 | `relocate.rs:314` | `git rm --cached` + `fs::rename` of the squatter | **none** — the door's, many calls earlier (**codex-1**) |
| 3 | `rename.rs:732` | `git add` | door's; commit seam verifies after |
| 4 | `rename.rs:734` | `git add` | ditto |
| 5 | `migrate_corpus.rs:455` | `git add` | ditto |
| 6 | `milestone.rs:735` | `git add` (add-from-spec) | ditto |
| 7 | `task.rs:3779` | `git add` (finalize stage) | ditto |
| 8 | `task.rs:4495` | `git add` (owner-artifact stage) | ditto |
| 9 | `task.rs:5602` | `git add` into the **off-line** `CombineIndex` | n/a (not the live index) |
| 10 | `task.rs:5610` | `git add` into the **live** index before the ff | `live.verify` fires 4 lines later |
| 11 | `setup.rs:2507` | `git add` (install commit) | door's; seam verifies after |

**Eleven sites; one probed immediately; nine covered by a following commit-seam `verify` whose
refusal I drove *and* whose rollback I observed restoring the index; one uncovered** (site 2).

---

## §2 · The drives

### 2.1 The state × door matrix (12 acting doors)

Cells marked ✔ were swept over all 12 doors in one run; cells marked (n) name how many doors were
driven. Every row's `argv`/observed lines are in the subsections below or in the rig's `$RIG/out/`.

| git state | 10 commit-on-behalf doors | 2 mover doors | verdict class |
|---|---|---|---|
| merge (1) | `repo.operation-in-progress`, route `git merge --abort` (review) | same | correct |
| **squash merge (2)** | **no code — the door acts** (2 driven) | not driven | **unnamed** |
| rebase-merge (3) | `repo.head-detached`, route exits 128 (review D1) | `git rebase --abort` ✔ works | masked |
| **rebase --apply (4)** ✔ | `repo.head-detached`, route `git switch main` → **exit 128** | `repo.operation-in-progress` + `git rebase --abort` → **exit 0, resolves it** | **masked — and a 3rd D1 cell** |
| `git am` (5) | `a rebase is in progress` / `git rebase --abort` → 128 (review D2) | identical | named-with-wrong-route |
| **cherry-pick 1 (6)** | split by commit model (7 driven) | **the door acts** (1 driven) | **unnamed** |
| **cherry-pick N (7)** | `task finalize --carry-staged` acts (1 driven) | not driven | **unnamed** |
| **revert 1 (8)** | **every** door acts — incl. the pathspec ones (3 driven) | not driven | **unnamed** |
| revert N (9) | not driven | not driven | unnamed (by construction) |
| **bisect before good/bad (10)** ✔ | `repo.operation-in-progress`, `a bisect is in progress`, route `git bisect reset` | same | **correct — unmasked** |
| bisect after good/bad (11) | `repo.head-detached` (review D1) | `git bisect reset` | masked |
| detached (12) / unborn (13) | review | review | correct |
| **unmerged index, no op (14)** | **no code — the door acts, then git refuses** (1 driven) | not driven | **unnamed** |

### 2.2 `rebase --apply` — D1's third cell, and the reason D2's fix cannot be a rename

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
# fixture: a live dev-task + milestone axis-milestone + sub-task, minted before the state
git checkout -q -b side2; printf 'side\n' > c.txt; git add c.txt; git commit -q -m side
git checkout -q main;     printf 'main\n' > c.txt; git add c.txt; git commit -q -m main
git checkout -q side2; GIT_EDITOR=true git rebase --apply main     # exit 1
# observed: markers=[rebase-apply] head=DETACHED  applying?=no  onto?=YES
```
All 12 doors driven (`sweep12.sh rebase-apply-BACKEND-detached`):
```
rebase-apply-BACKEND-detached/setup ... /milestone-discard    exit=1  blocking · repo.head-detached
    route: re-attach HEAD with `git switch <branch>`, then re-run this command
rebase-apply-BACKEND-detached/relocate, /config-set           exit=1  blocking · repo.operation-in-progress
    route: conclude it, or abandon it with `git rebase --abort`, then re-run this command
# the commit doors' route, run verbatim:
git switch main   → fatal: cannot switch branch while rebasing   exit 128 (state unchanged)
# the movers' route, run verbatim:
git rebase --abort → exit 0, markers=[] head=attached:refs/heads/side2
```
**Classification: latent defect (D1's class is 3 cells, not the 2 the row names).**
**Measured constraint for the fix:** in this cell `git rebase --abort` is *correct* and in the
`git am` cell it is *refused by git*. Both write `rebase-apply/`. So D2 cannot be fixed by renaming
the noun — the two must be discriminated, and git's own discriminator
(`rebase-apply/applying` ⇒ am, `rebase-apply/onto` ⇒ rebase) is driven above in both directions.

### 2.3 `git bisect start` before the first good/bad — the cell that is **correct**

```
git bisect start            # exit 0; markers=[BISECT_LOG BISECT_START]; HEAD ATTACHED
```
All 12 doors: `exit=1 · blocking · repo.operation-in-progress — a bisect is in progress …
route: conclude it, or abandon it with 'git bisect reset', then re-run this command`, markers
unchanged at every door. **Classification: built + proven.** It matters for the fix: the bisect
half of D1 is a *HEAD-detachment* artefact, not a bisect artefact, so a fix keyed on "bisect" would
be over-broad — the discriminating property is `HeadDetached ∧ an operation is in progress`.

### 2.4 The cherry-pick cell across the acting doors — the commit model is the discriminator

Fixture (rigA): committed `adr:keeper-decision`, milestone `bee-milestone` + sub-task `bee-intent`,
a live dev-task; then a conflicted `git cherry-pick`, conflict resolved and staged by the user.

| door | commit shape | exit | observed |
|---|---|---|---|
| `task finalize --carry-staged` | `git commit -F` (whole index) | 0 | review D3 — marker consumed |
| **`jigc rename adr:keeper-decision --to "Axis decision"`** | `git commit -F` (whole index) | **0** | `renamed adr:keeper-decision -> adr:axis-decision …`; commits 11→12; **`CHERRY_PICK_HEAD` GONE** |
| `milestone create` | `git commit -F -- <path>` | 1 | review D3b |
| **`milestone add-task bee-milestone "second bee"`** | pathspec | 1 | ``fatal: cannot do a partial commit during a cherry-pick.`` … *"Fix the hook's complaint"* |
| **`milestone discard bee-milestone --force`** | pathspec | 1 | same frame, `…record is still at its pre-discard state…  Fix the hook's complaint` |
| **`task discard bee-intent --force`** (sub-task arm) | pathspec | 1 | same frame, *"the milestone record still names task:bee-intent … Fix the hook's complaint"* |
| `task discard <plain dev-task> --force` | no commit | 0 | no commit path — nothing to conclude |
| `setup` (already installed) | — | 0 | `nothing_staged` skip; install commit never reached |
| `migrate-corpus` (current corpus) | — | 0 | `0 migrated, 7 already current` — commit never reached |
| `milestone add-from-spec … spec:nope` | — | 1 | `store.not-found` before any commit |
| **`milestone finalize <fan-out>`** | worktree commit + `git merge --ff-only` | 1 | see §2.7 |
| **`config set docs-root docs2`** (mover) | `git mv` ×4 | **0** | see §2.8 |

**`rename` needed two of its own gates cleared to reach its commit** — `rename.dirty-tree` (fires
when the pick's resolution is staged) and `rename.in-flight` (fires while any task is open). With
both clear (the user resolved the pick *to ours*, so the index matches HEAD) it commits:
```
git cherry-pick "$SHA"; printf 'BETA\n' > cp.txt; git add cp.txt   # resolve keeping ours
git status --short → (empty)   markers=[MERGE_MSG CHERRY_PICK_HEAD]
jigc rename adr:keeper-decision --to "Axis decision"
# exit 0 · renamed adr:keeper-decision -> adr:axis-decision …, repointed 0 referrer(s)
# commits 11 -> 12 · markers after=[] · CHERRY_PICK_HEAD GONE
```
**Classification: latent defect — D3's class has a second door the row does not name.**

### 2.5 The **revert** cell — git does not guard it, so *every* commit door consumes it

git's `determine_whence()` reads `MERGE_HEAD` and `CHERRY_PICK_HEAD`; it does **not** read
`REVERT_HEAD`. Driven consequence: the partial-commit refusal that accidentally protects the four
record-only doors under a cherry-pick **does not exist under a revert**.

```
printf 'Z1\nkeep\n' > rz.txt; git add rz.txt; git commit -q -m z1; SHA=$(git rev-parse HEAD)
printf 'Z2\nkeep\n' > rz.txt; git add rz.txt; git commit -q -m z2
git revert --no-edit "$SHA"                       # exit 1 — conflict
printf 'REVERTED-PAYLOAD\nkeep\n' > rz.txt; git add rz.txt   # a REAL resolution
# markers=[MERGE_MSG REVERT_HEAD]  status=[M  rz.txt]

jigc milestone create "Mno milestone"
# exit 0
#   minted milestone:mno-milestone (shared base bedbac2)
#   record commit: 4d282eb   — the record on its own; anything else you had staged stayed staged
# markers after=[]           <-- REVERT_HEAD and MERGE_MSG are GONE
git status --short → M  rz.txt          (the revert's payload, now orphaned)
git revert --continue
#   error: no cherry-pick or revert in progress
#   fatal: revert failed                                        exit 128
```
Driven at a second pathspec door in the same shape: `jigc milestone add-task kay-milestone "kay work"`
→ `exit 0`, `added task:kay-work …`, commits 25→26, `markers=[]`, `git status` clean.

**Classification: latent defect.** Three things the fix must know: (a) the ack's
*"anything else you had staged stayed staged"* is true of the bytes and false of the operation —
a law-1 lie in this cell; (b) git's own recovery is destroyed, not just the marker; (c) the
cherry-pick cell's exit-1 is git's accident, not jigc's guard, so **any fix that only widens the
marker set must still carry the revert cell, where the damage is exit 0 at every commit door.**

### 2.6 The **squash merge** — no marker exists, and `task finalize` concludes it

```
git merge --squash sq              # exit 1 (conflict); markers=[MERGE_MSG SQUASH_MSG]; NO MERGE_HEAD
# (a) conflict UNRESOLVED
jigc task finalize land-under-a-squash-merge
#   exit 3 · blocking · finalize.carried-staged (×2: sq.txt, sq2.txt)
#     route: … re-run the finalize with `--carry-staged`
jigc task finalize land-under-a-squash-merge --carry-staged
#   exit 1
#   `git commit` was rejected (no commit was made):
#   U	sq.txterror: Committing is not possible because you have unmerged files.
#   …  Fix the hook's complaint, then re-run `jigc task finalize … --carry-staged`.
# (b) the user resolves it, as git instructs
printf 'RESOLVED-SQ\n' > sq.txt; git add sq.txt
jigc task finalize land-under-a-squash-merge --carry-staged
#   exit 0 · finalized 1414577 — feat(axis): land under a squash merge
#     modified sq.txt / carried-over sq2.txt / 2 files committed
# markers after=[]   git status → nothing to commit, working tree clean
git show --stat HEAD → sq.txt | 2 +-   sq2.txt | 1 +      <-- the WHOLE merge payload
```
**Classification: latent defect — D3's class is not a marker set.** The entire squash merge landed
inside jigc's `feat(axis):` commit. `MERGE_MSG`/`SQUASH_MSG` were consumed and the merge's
authored message is gone. The only honest discriminator available is `SQUASH_MSG ∧ ¬MERGE_HEAD`
(or an unmerged-index probe for (a)); the family's four markers cannot see it.

### 2.7 The multi-commit sequencer — a dangling sequencer whose documented recovery **destroys jigc's commit**

```
git cherry-pick "$C1" "$C2"                 # exit 1; markers=[MERGE_MSG CHERRY_PICK_HEAD sequencer]
printf 'RESOLVED1\n' > mc.txt; git add mc.txt
printf 'code\n' > feature.txt; git add feature.txt
jigc task finalize land-under-a-multi-pick --carry-staged
#   exit 0 · finalized e2d3009 — feat(axis): land under a multi pick
#     added feature.txt / carried-over mc.txt / 2 files committed
# markers after = [sequencer]          <-- CHERRY_PICK_HEAD consumed, sequencer/ left behind
git status
#   On branch main
#   Cherry-pick currently in progress.
#     (run "git cherry-pick --continue" to continue)
#     (use "git cherry-pick --abort" to cancel the cherry-pick operation)
#   nothing to commit, working tree clean
cat .git/sequencer/todo → pick 98a906d m1 / pick 9b78e82 m2   (m1 still listed as un-done)

# the user takes the recovery git itself printed:
git cherry-pick --abort        # exit 0
# HEAD e2d3009 -> 720e594 ;  feature.txt GONE ;  `git branch --contains e2d3009` → (empty)
```
**Classification: latent defect — data loss at the end of an exit-0 path.** jigc reported the task
finalized (and its own state records it as finalized); git then discarded that commit and its files
on the very command `git status` advertises. Reachable only from the multi-commit variant, which is
exactly the variant the review's §6.5 named as *not driven*.

### 2.8 The movers under a sequencer state — `git mv` into an index the pick owns

`SeamAct::Move` adjudicates `OperationInProgress` **only**, and the sequencer family is not a
member, so both movers act:
```
# live CHERRY_PICK_HEAD, index clean
jigc config set docs-root docs2
# exit 0
#   relocating 4 committed doc(s) stranded by the `docs-root` re-point to `docs2` …
#   - docs/milestone-records/fan-milestone.md → docs2/milestone-records/fan-milestone.md   (×4)
git status --short →  R  docs/…/fan-milestone.md -> docs2/…/fan-milestone.md   (×4)

# the user takes git's advertised recovery:
git cherry-pick --abort        # exit 0
ls docs2/milestone-records → No such file or directory        <-- all four moves UNDONE
jigc validate
# exit 1
#   blocking (gates at finalize) · reconciliation.rename — tracked managed doc
#     milestone-record:fan-milestone (docs2/milestone-records/fan-milestone.md) is missing   (×4)
#     route: restore … or confirm the deletion by dropping it from the index: `jigc unmanage …`
```
**Classification: latent defect.** No bytes were destroyed (the files are back at the old home) but
the knob still says `docs2` while the store says `docs`, `jigc validate` goes red with four
blocking findings, and the offered route tells the operator to `jigc unmanage` four managed docs.

### 2.9 D3b's cause set — every non-zero `git commit` is rendered as a hook rejection

`git_commit_capture` (`task.rs:5433`) builds `CommitRejected` from **any** non-zero `git commit`,
and `render::commit_rejected` (`render.rs:1998`) closes every one with *"Fix the hook's
complaint"*. Driven causes, each at a real door on this binary:

| # | cause | door driven | observed |
|---|---|---|---|
| 1 | a `pre-commit` hook rejects | — | the designed cell (not re-driven) |
| 2 | `fatal: cannot do a partial commit during a cherry-pick.` | 4 doors (§2.4) | *"Fix the hook's complaint"* |
| 3 | `error: Committing is not possible because you have unmerged files.` | `task finalize --carry-staged` | *"Fix the hook's complaint"* |
| 4 | `error: gpg failed to sign the data` (`commit.gpgsign=true`, bogus `user.signingkey`) | `milestone create`, `task finalize` | *"…fatal: failed to write commit object … Fix the hook's complaint, then re-run `jigc milestone create 'Gee milestone'`"* |

**A second defect inside the frame, driven:** cause 3 rendered as
`U	sq.txterror: Committing is not possible…` — `format!("{}{}", stdout.trim(), stderr.trim())`
concatenates git's two streams with **no separator**, so whenever git writes to both, the last token
of stdout is glued to the first token of stderr. **Classification: latent defect** (law 1 — the
relay is supposed to be verbatim).

Not reachable on this host: an unset committer identity — the rig's global gitconfig supplies one,
and with `GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_SYSTEM=/dev/null` git still derived a default
identity and committed at exit 0. Stated as **not driven**, not as absent.

### 2.10 codex-1's class — the nine `git add` sites *are* covered; the `git rm --cached` is not

Instrument: a `git` PATH shim that, on the first call whose argv contains a trigger token, runs one
**real** `git bisect start` against the rig and then execs the real git (`$RIG/shim/git`; a merge
was tried first and is unusable — git refuses to start a merge with a dirty index, which at
`task finalize` is the normal state; a bisect starts regardless and is a genuine family member).

```
SHIMTRIG=add  jigc task finalize plant-a-bisect-mid-run
# exit 1
#   blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
#     committable state
#     route: conclude it, or abandon it with `git bisect reset`, then re-run this command
# git-call log:  … symbolic-ref -q HEAD  |  add -- .jigc/config .jigc/.gitignore .jigc/version
#                SHIM FIRED → BISECT_LOG=yes
#                symbolic-ref -q HEAD (the seam's re-probe)  → refusal
#                update-index --cacheinfo … (×4)             <-- the index rollback ran
# commits 8 -> 8 ; the user's own staged feat3.txt untouched
```
**Classification: built + proven** for the stage-then-commit sites (7 of the 11). The uncovered
site remains `relocate.rs:314` exactly as codex-1 states it; I did not re-drive it (review §R.1
drives it at two doors) — what this adds is the *denominator*: **1 of 11 index-mutating sites is
uncovered, not "the movers are unprobed".**

### 2.11 Open lead 3 — the fan-out `git merge --ff-only` re-probe, driven at the seam

Fixture: `milestone create` + `add-task` + `provision` + staged code in the worktree +
`jigc workflow sub-task --task <sub>` (without which the sub-task's `commit` doc is not
provisioned) + its filled commit doc. Baseline: the same fixture finalizes clean
(`finalized b80eb72 — Finalize milestone fan-milestone (1 sub-task)`, exit 0).

```
SHIMTRIG=write-tree  jigc milestone finalize gan-milestone
# exit 1
#   blocking · repo.operation-in-progress — a bisect is in progress …
#     route: conclude it, or abandon it with `git bisect reset`, then re-run this command
# git-call log, in order:
#   write-tree | SHIM FIRED → BISECT_LOG=yes | worktree add --detach …/.combine-… |
#   read-tree --reset -u | symbolic-ref (×3) | commit -F …/finalize-message.tmp   <-- LANDED
#   | worktree remove --force | add -- … | symbolic-ref  <-- live.verify REFUSES
#   | update-index --cacheinfo (×4)                       <-- rollback
#   grep -c "merge --ff-only" → 0
# commits 13 -> 13 ; markers=[BISECT_LOG]
```
**Classification: built + proven, with one fact the lead does not carry.** The re-probe fires and
`--ff-only` never runs. But the **dedicated worktree's posture is per-worktree**, so the boundary
commit *was created* (the `commit -F` above) before the live refusal and is now an unreferenced
commit object. Nothing landed on the branch, the index was restored, and the worktree was removed —
the orphan is garbage, not damage; recorded because a fix that moves the probe earlier would also
remove the orphan.

**The same seam under a pre-existing cherry-pick** (state present before the door, so the re-probe
sees nothing to refuse) reaches `--ff-only` and gets git's own refusal, rendered through the
**non-hook** frame — no hook lie, but no code and no route:
```
jigc milestone finalize ian-milestone
# exit 1
#   `git merge --ff-only 3db5cec…` failed: fatal: You have not concluded your cherry-pick
#   (CHERRY_PICK_HEAD exists).
#   Please, commit your changes before you merge.
#
#   milestone:ian-milestone is intact — nothing was committed, the merged docs were rolled back,
#   and every provisioned sub-task worktree still holds its staged code. Resolve the cause above,
#   then re-run `jigc milestone finalize ian-milestone`.
# commits 19 -> 19 ; markers unchanged
```

---

## §3 · What changed against the review's rows

1. **D1's class is three cells, not two, and one *bisect* cell is correct.** `git rebase --apply`
   (§2.2) is a third masked cell; `git bisect start` before the first good/bad (§2.3) is
   **unmasked and correct**. The discriminating property is `HeadDetached ∧ OperationInProgress`,
   not "rebase or bisect". The masking is an ordering property of `posture()` + two `.find(…)`
   consumers, and the movers already get it right by filtering — so ordering alone changes the
   commit doors' answer without touching the mover class.
2. **D2 cannot be fixed by renaming the noun.** Both `git am` **and** `git rebase --apply` write
   `rebase-apply/`, and in the latter the printed `git rebase --abort` is correct and resolves the
   state (driven, exit 0). git's own discriminator — `rebase-apply/applying` vs `rebase-apply/onto`
   — is driven in both directions (§1.1).
3. **D3's class is four operations, not two markers, and it reaches doors the row does not name.**
   `jigc rename` concludes a cherry-pick at exit 0 (§2.4); **every** commit door concludes a
   **revert** at exit 0, because git's partial-commit guard reads `CHERRY_PICK_HEAD` and not
   `REVERT_HEAD` (§2.5); a **squash merge** has no marker at all and is swallowed whole (§2.6); the
   **multi-commit** variant leaves a dangling `sequencer/` whose advertised recovery then destroys
   jigc's own commit and files (§2.7). The row's *"4 of the 6 markers"* framing understates it:
   the axis is *operations git can leave un-concluded*, and two of them write no member of any
   marker set.
4. **D3b's cause set is at least four**, three of them driven here (§2.9), and the frame itself
   mis-renders git's two streams (`U\tsq.txterror:`). The review's *"no hook exists in the rig"*
   correction is already recorded; what is new is that the lie is produced by a **cause axis**, not
   by the cherry-pick cell.
5. **codex-1's denominator.** The class is 11 production index-mutating sites; the nine
   stage-then-commit ones are genuinely covered by the seam's re-probe *and* by a rollback I
   observed running (§2.10), so the finding is one site, not a door-class.
6. **Open lead 3 is discharged as built + proven** (§2.11), with the per-worktree posture fact and
   the orphaned boundary commit recorded.

**Which states are masked / unnamed / named-with-wrong-route** (the §3 question, answered):
*masked* — rebase-merge, rebase-apply-as-rebase, bisect-after-good/bad (commit doors only);
*named with a route git refuses* — `git am`; *unnamed, no member at all* — squash merge,
cherry-pick (single and sequencer), revert (single and sequencer), and a conflicted index with no
operation; *correct* — merge, bisect-before-good/bad, detached, unborn.

**Which doors' commits consume an in-progress marker** (driven):
`task finalize` (cherry-pick · revert · squash merge) · `rename` (cherry-pick) · and — under a
**revert only** — `milestone create`, `milestone add-task` and, by the shared
`git_commit_pathspec` seam, the rest of the record-only family. Under a **cherry-pick** the
pathspec doors are stopped by *git*, not by jigc.

**The constraint the fix inherits (measured, not recommended).** A `git commit` under
`CHERRY_PICK_HEAD` / `REVERT_HEAD` / `MERGE_HEAD` **is** the conclusion of that operation as far as
git is concerned: it clears the marker, consumes `MERGE_MSG`, and for `MERGE_HEAD` records the
merge parents. There is no argument to `git commit` that makes it not conclude. So a *consent*
answer at the Settle cannot be "consent and commit safely" — it can only be "consent to conclude
the user's operation under jigc's subject". Correspondingly: `--carry-staged`, `--force` and
`--dry-run` are leaf-keyed and carry no posture consent today (review §3.12), so any consent flag
would be a new one. And no marker-based widening reaches the squash merge or the conflicted-index
cell; those need `SQUASH_MSG ∧ ¬MERGE_HEAD` and an unmerged-index probe respectively.

---

## §4 · Latent defects (not in any §A row), each driven

| id | one line | argv that shows it |
|---|---|---|
| **L1** | `jigc task validate` exits **0** in a state where `jigc task finalize` exits **1** on `repo.operation-in-progress` — and `GATE_COVERAGE` (12 rows, `crates/cli/src/gate_coverage.rs`) carries **no** posture row, so the coverage claim stated on **eight** surfaces (the composed `what's-left:` line, both packs' finalize steps, the `task` unknown-subcommand tip, the `validate-task` catalog hint, QUICKSTART.md, `design/command-output-contract.md`, `design/finalize.md`) is one *family* short. This is precisely the class the `changelog-gate` row's own comment exists to prevent. | `git merge --no-ff --no-commit sidecar; jigc task validate preview-under-a-merge` → **exit 0**; `jigc task finalize preview-under-a-merge` → exit 1 `repo.operation-in-progress` |
| **L2** | A **revert** is concluded at exit 0 by *every* commit door, pathspec ones included; the user's `git revert --continue` then exits 128 *"no cherry-pick or revert in progress"* and the revert's payload is orphaned in the index while jigc's ack says *"anything else you had staged stayed staged"*. | `git revert --no-edit <sha>` (conflict) → resolve+stage → `jigc milestone create "Mno milestone"` → exit 0, `REVERT_HEAD` gone |
| **L3** | An un-concluded **`git merge --squash`** — which writes **no marker of any kind** — is swallowed whole by `jigc task finalize --carry-staged`: the entire merge payload lands inside jigc's `feat(axis):` commit at exit 0. | `git merge --squash sq` (conflict) → resolve+stage → `jigc task finalize land-under-a-squash-merge --carry-staged` → exit 0, `2 files committed` |
| **L4** | `jigc rename` concludes a cherry-pick at exit 0 (a whole-index `git commit -F`, `rename.rs:792`); its two own gates (`rename.dirty-tree`, `rename.in-flight`) are what usually hide it. | `jigc rename adr:keeper-decision --to "Axis decision"` under `CHERRY_PICK_HEAD` with a clean index → exit 0, marker gone |
| **L5** | Under a **multi-commit** pick, jigc's exit-0 finalize leaves `.git/sequencer/` dangling; `git status` then advertises `git cherry-pick --abort`, and running it **destroys jigc's commit and its files** (`feature.txt` gone, commit unreachable from any branch) while jigc's own state still records the task as finalized. | `git cherry-pick <a> <b>` → resolve+stage → `jigc task finalize … --carry-staged` (exit 0) → `git cherry-pick --abort` (exit 0) → `git branch --contains <sha>` empty |
| **L6** | A conflicted index with **no** operation marker (conflicted `--squash` merge, conflicted `git stash pop`) reaches the commit seam; git's `Committing is not possible because you have unmerged files` is dressed as *"Fix the hook's complaint"* with no code and no route — **and** git's stdout and stderr are concatenated with no separator (`U\tsq.txterror: …`, `task.rs:5433`). | `jigc task finalize land-under-a-squash-merge --carry-staged` with the conflict unresolved |
| **L7** | Both **mover** doors act inside a sequencer-owned index (`SeamAct::Move` adjudicates `OperationInProgress` only): `jigc config set docs-root docs2` `git mv`s four managed docs at exit 0 under a live cherry-pick; `git cherry-pick --abort` silently undoes all four, leaving the knob at `docs2`, the store at `docs`, `jigc validate` red with four blocking `reconciliation.rename` findings, and the offered route `jigc unmanage <doc>`. | `jigc config set docs-root docs2` under `CHERRY_PICK_HEAD` → exit 0 → `git cherry-pick --abort` → `jigc validate` exit 1 |
| **L8** | A `commit.gpgsign` failure is rendered as *"Fix the hook's complaint"* — a third cause in the same class as L6/D3b. | `git config commit.gpgsign true; git config user.signingkey DEADBEEFNOTAKEY; jigc milestone create "Gee milestone"` → exit 1, `error: gpg failed to sign the data … Fix the hook's complaint` |

---

## §5 · Honest bounds

1. **Everything above is one git (2.54.0, Apple Git-157).** `determine_whence()`'s blindness to
   `REVERT_HEAD` (L2), the `rebase-apply/applying` vs `onto` discriminator, and the squash merge's
   marker set are facts about this git's on-disk contract. I did not check them against another
   git, and the fix's fence should not assume they are stable across versions without saying so.
2. **`revert × N` (sequencer) was not driven**, nor was the fan-out boundary under a revert, nor a
   mover under a revert. Reason: the single-commit revert is driven at three doors and the
   sequencer's *additional* damage is driven on the cherry-pick side (§2.7); I am asserting the
   combination by composition and saying so rather than presenting it as measured.
3. **`setup`'s install commit and `migrate-corpus`' self-commit were never reached** in any
   sequencer state — in a set-up, already-current rig both skip before committing. Their commit
   shapes are pathspec (`setup.rs:2507`+`--no-verify`; `migrate_corpus.rs:~476`), so they belong to
   the same class by code read, **not** by drive. A `bare` rig plus an unmigrated corpus would
   reach them; I did not build one.
4. **`milestone add-from-spec`'s commit was never reached** (no committed `spec` in the rig; it
   exits at `store.not-found`). It shares `git_commit_pathspec` with the four doors driven.
5. **The mid-run window is a race in both directions.** The PATH-shim plants a *real*
   `git bisect start` from inside jigc's own process tree; no argv sequence opens the window, and I
   could not use a merge as the planted operation at all (git refuses to start one with a dirty
   index — the normal state at `task finalize`). So the "covered" verdict in §2.10 is proven for
   the **bisect** member and inferred for the other two, which share the identical `posture()` call.
6. **I did not drive the 35 `Neither` leaves**, the `GIT_DIR` redirect, the dedicated-worktree
   exemption, or the unborn/detached cells — all covered by the review, and re-driving them buys
   nothing this brief asked for.
7. **`task.rs:5602`'s `git add`** writes the off-line `CombineIndex`, not the live index; I
   classified it from the code and from the git-call log's `git_index` prefix, not from a separate
   drive.
8. **No invocation-log read-back** for the consuming cells (L2–L5). Whether an exit-0 run that
   concluded a user's operation is legible in `.jigc/logs/invocations.jsonl` is unmeasured.
