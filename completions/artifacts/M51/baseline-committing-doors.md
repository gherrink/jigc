<!-- M51 baseline · companion 2 of 4 — the committing-door seam and git posture. Driven 2026-09-10 by one Opus capability-auditor against the release binary `1.0.0-rc.14` at HEAD `74627547`; no cargo run, no repo file edited. Verbatim as returned (§1–§7 plus Honest bounds); consolidated in [baseline-ledger.md](baseline-ledger.md). -->

# M51 baseline — the committing-door seam and git posture

Verified at HEAD `74627547`, release binary `/Users/maurice/projects/gherrink-jigc/target/release/jigc` = `1.0.0-rc.14`. **A map at one sha, not gospel.** Every row was driven through the release binary on throwaway `dev/jigc-rig` corpora unless it says *source-read*. No repo file edited, no cargo run. Raw transcripts under `/private/tmp/claude-501/-Users-maurice-projects-gherrink-jigc/dd9473b1-8a73-47ad-8f77-67d4d3237620/scratchpad/baseline2/*.txt`.

---

## 1. `COMMITTING_DOORS` — membership, the shared seam, and where a posture probe could live

`crates/cli/src/invocation_log.rs:130` — **10 members**, one `error_code` each.

| # | door | commit call path | rides `git_commit_capture`? |
|---|---|---|---|
| 1 | `jigc task finalize` | `task.rs:2820/2833` → `git_commit` (`:4225`) → `git_commit_capture` (`:4255`) | yes — **live checkout**, whole index |
| 2 | `milestone finalize (squash: true)` | `combine_commit` `task.rs:4364` → `overlay_docs_commit_and_ff` `:4406` → `commit_combined_tree_with_hooks` `:4543` → `git_commit` | yes — but in a **dedicated detached worktree**; lands via `git merge --ff-only` in the live checkout |
| 3 | `milestone finalize (squash: false)` | `chain_commit` `task.rs:4477`, `git_commit(wt,…)` at `:4496` per sub-task + the same aggregate seam | same — **dedicated detached worktree** |
| 4 | `jigc rename` | `rename.rs:777` → `git_commit` | yes — live checkout, **no pathspec** |
| 5 | `jigc migrate-corpus` | `migrate_corpus.rs:470` → `git_commit_capture` directly (pathspec) | yes |
| 6 | `milestone create` | `materialize_and_commit_record` `milestone.rs:594` → `commit_record_transaction:801` → `commit_record_only:658` → `git_commit_pathspec:820` → seam | yes |
| 7 | `milestone add-task` | `append_and_commit_record` `milestone.rs:1265` (call `:1291`) → same chain | yes |
| 8 | `milestone add-from-spec` | `append_and_commit_record` (call `milestone.rs:1415`) → same chain | yes |
| 9 | `milestone discard` | `run_discard` `milestone.rs:3013` (call `:3087`) → same chain | yes |
| 10 | `jigc task discard` (milestone sub-task) | `task.rs:516` → `milestone::settle_discarded_sub_task:1666` (call `:1703`) → same chain | yes |
| — | `jigc setup` — **excluded by recorded design** | `setup.rs:1709`, `["commit","--no-verify","-m",…,"--",<pathspec>]` via `git_output` | **NO — bypasses the seam entirely** |

**Not committing doors** (verified, not assumed): `jigc config set` inserts a constant `"committed": false` on every ack (`render.rs:2698-2703`) — it *stages* `git mv`s on a root re-point but never commits; `jigc upgrade` has no `commit_install`/`install_commit` reference in `upgrade.rs`. Every remaining `git commit` in `crates/cli/src` is `#[cfg(test)]` — verified per site (`combine.rs:374` under `cfg(test)`@299; `start.rs:3890/3962`@3833; `task.rs:5215/5368`@4918).

### The fences and what each asserts about membership

| fence | asserts |
|---|---|
| `crates/cli/src/invocation_log.rs::tests::registry_mirrors_the_declared_members` (`:504`) | `ERROR_CODE_REGISTRY` **==** `COMMITTING_DOORS.map(error_code) ++ [ERROR_REVIEW_PENDING]`, length 11, pairwise-distinct. A door without a code, or a code without a door, is red. |
| `crates/cli/tests/commit_rejected_axis.rs` | Two sweeps, both `assert_eq!(COMMITTING_DOORS.len(), 10, "…+ jigc setup excluded by its recorded --no-verify reason")` and both `panic!` on an unclassified member. Sweep 1 (`:697`): each door under a **rejecting `pre-commit`** → non-zero exit · git's bytes verbatim · a state-truth clause · the door's **own** re-run argv · the door's `error_code` read back **from the log** (closing the release hole — `Outcome::error`'s membership check is a `debug_assert!`) · the printed argv lifted verbatim exits 0 after the hook is removed. Sweep 2 (`:1484`): the same table driven into the **empty-commit** cell with no hook — the frame's assertion, git's empty-commit prose and any `*.commit-rejected` identity all absent. |
| `crates/cli/tests/hook_output_axis.rs` | The **non-blocking** half. Its enumeration is **prose, not the table** (`:45` says so and points at `COMMITTING_DOORS`): 7 producer groups + the same one exclusion. Asserts a marker-speaking `exit 0` hook's stream reaches the caller at each producer. **Gap by construction:** nothing machine-checks that this file's arms cover the table. |
| `crates/cli/tests/registry_seam.rs` | **Does not touch `COMMITTING_DOORS`.** It fences the *pack* registry (34 workflows / 16 deduped doctypes), the clap tree walk, and the two exit-code constants. Named in the task, but it is a different seam. |
| `crates/cli/tests/flow47_acceptance.rs:556` / `flow48_acceptance.rs:2784` / `invocation_log.rs:799` | Three more consumers of the same table (recoverability sweep, empty-commit class property, log-code derivation). |

### Where a HEAD-posture probe could live — and what it would get wrong

9 of 10 doors funnel into one function, `task::git_commit_capture`. But **members 2 and 3 pass a deliberately detached `DedicatedWorktree` through that same function** (`task.rs:4581`/`:4599` — `git worktree add --detach`), so a naive probe at the seam is wrong in exactly the two members that need the exemption; and it would still miss the act that matters at the boundary, `git merge --ff-only` in `overlay_docs_commit_and_ff`, which is downstream of the seam and runs in the **live** checkout. `jigc setup` is reached by no seam probe at all. The honest shape is: a probe at the **door**, before any work, plus one at the fast-forward — not one at `git_commit_capture`.

---

## 2. Door × posture (all driven) — the ledger

### **latent defect — EC-2 confirmed and widened: no door asks HEAD's posture**

**Status:** latent defect (data loss at exit 0). **Evidence:** `grep -rn "symbolic-ref\|symbolic_ref\|rev-parse --abbrev-ref" crates/` → **0 hits** at HEAD. Driven, every door on a detached HEAD:

| door | E | git | jigc said |
|---|---|---|---|
| `setup` | 0 | install commit `c121078` branchless; `main` stays `aa07cfc` | `install commit → c121078` |
| `task finalize` | 0 | `08ebb2c` branchless, task area **destroyed** in the same act | `finalized 08ebb2c … promoted VISION.md` |
| `milestone create` | 0 | `371fb54` branchless | `record commit: 371fb54` |
| `milestone add-task` | 0 | `8a3ac10` branchless | `added task:alpha-task` |
| `milestone add-from-spec` | 0 | `6db4d79` branchless | `seeded 1 sub-task(s)` |
| `milestone provision` | 0 | worktrees `--detach` at base (correct) | `provisioned 1 worktree(s)` |
| `milestone join` | 0 | no commit | `joined … 2 doc(s) merged` |
| `milestone finalize` | 0 | `8c6d190` branchless carrying **both sub-tasks' code + the merged docs**; worktrees torn down | `finalized 8c6d190 … 3 files committed` |
| `task discard` (sub-task) | 0 | `673f82e` branchless | `discarded task alpha-task` |
| `milestone discard` | 0 | `b8a94af` branchless | `discarded milestone:probe-milestone` |
| `rename` (retitle) | 0 | `2c382ea` branchless; `main`'s H1 still `# Decisions Log` | `renamed … repointed 0 referrer(s)` |
| `migrate-corpus` | 0 | `471ddec` branchless; `main` still at the old stamp | `committed 471ddec — only the migrated paths were staged` |
| `config set docs-root` | 0 | staged only | `…uncommitted — commit it with your next commit` |

Loss driven end to end on the finalize arm: after exit 0 on a detached HEAD, `git checkout main` restores `VISION.md`'s pre-task bytes and `jigc validate` then prints a **phantom** `blocking (gates at finalize) · file-state.hash-matches — on-disk content of VISION.md differs from the recorded state`, routed *"review the out-of-band edit"* — there was none.

**Exact gap, wider than EC-2 states:** the *fan-out* boundary is the sharpest cell (N sub-agents' code + docs on one branchless commit, with every worktree torn down in the same act), and `jigc setup` silently un-installs.

### **latent defect (new) — `GIT_DIR` redirects a committing door into a different repository at exit 0**

**Status:** latent defect. **Evidence:** with `GIT_DIR=<otherrepo>/.git` exported, run from repo A:

```
jigc milestone create "probe"    → EXIT=0
   "minted milestone:probe (shared base 50a4206)"
   "record commit: db89e71 — the record on its own; anything else you had staged stayed staged"
git --git-dir=<other>/.git log --oneline -1  →  db89e71 chore(milestone): open record for milestone:probe
```

The record **file** was written into repo A's working tree; the **commit** landed in repo B, whose status is left ` D x | ?? .jigc/ | ?? VISION.md | …`. `grep -rn "env_remove\|env_clear" crates/cli/src` → no production hit; `Command::current_dir()` does not override `GIT_DIR`. **Gap:** the same root cause as EC-2 one level out — nobody asks *which repository* either, only *which directory*.

### **latent defect (new) — `milestone create` on an unborn HEAD mints a permanently unusable milestone**

**Status:** latent defect (unrecoverable state, exit 0 at the door that causes it). **Evidence:**

```
git checkout --orphan fresh-start        # unborn HEAD, index intact
jigc milestone create "probe"            → EXIT=0
   "minted milestone:probe (shared base 4b825dc)"
grep base docs/milestone-records/probe.md
   base: 4b825dc642cb6eb9a060e54bf8d69288fbee4904 4b825dc     # git's EMPTY TREE
git cat-file -t 4b825dc                  → fatal: Not a valid object name
jigc milestone add-task probe "alpha task"   → EXIT=0
jigc milestone provision probe               → EXIT=1
   blocking · milestone.provision-failed — `git worktree add --detach … 4b825dc…` failed:
   error: object 4b825dc… is a tree, not a commit
   route: `jigc milestone provision probe` — …the re-run reuses every worktree that landed
jigc milestone finalize probe                → EXIT=1
   `git worktree add --detach …/.combine-… 4b825dc…` failed: … is a tree, not a commit
   (invocation log: exit 1, error_code: null, finding_codes: [])
jigc milestone discard probe --force         → EXIT=0   # the only exit
```

Re-driven **after a real first commit lands** — identical, because the base is read from the committed record. **Gap:** `EMPTY_TREE_SHA` (`task.rs:4160`) is a sentinel proven on the **single-task** shape ("the first finalize diffs against the empty tree"); the **milestone** shape feeds it to `git worktree add`, which needs a commit. Shape-limited sentinel → unrecoverable milestone, with `provision`'s route a dead end that can only fail identically.

### **shape-limited — a linked worktree on another branch: `task finalize` and the milestone family disagree about which repo they act on**

**Status:** shape-limited. **Evidence** (`git worktree add -b feature <wt>`, jigc driven from `<wt>`):

```
jigc task finalize <id>        → EXIT=0, commits 9504afa onto **feature**, ADR promoted into
                                  the worktree and inside the commit.  MAIN untouched. CORRECT.
jigc milestone create "probe"  → EXIT=0
   "record: docs/milestone-records/probe.md"   ← does not exist from where the operator stands
   "record commit: d753cdc"
   MAIN log: d753cdc  (branch **main**);  WT log: unchanged
   record file present in MAIN only;  workbench in MAIN/.jigc/milestones
jigc milestone add-task probe "alpha task"  → EXIT=0, also onto **main**
```

The `jigc_home` indirection (`locate.rs:27-30`) is correct for **fan-out** worktrees; the unexercised adjacent shape is a **user's own** linked worktree, where the milestone record lands on another branch, silently, under a printed repo-relative path that resolves to nothing from the operator's cwd (law 1). **Gap:** no probe distinguishes a jigc fan-out worktree from an ordinary one.

### **shape-limited — `task finalize --carry-staged` silently concludes an in-progress merge**

**Status:** shape-limited (law 1 / contract). **Evidence:** MERGE_HEAD present with the conflict resolved and staged:

```
jigc task finalize tweak-the-greeting                → EXIT=3  finalize.carried-staged (the gate catches it)
jigc task finalize tweak-the-greeting --carry-staged → EXIT=0
   "finalized 6940d02 — chore: tweak the greeting / carried-over conflict.txt / 1 file committed"
git rev-list --parents -n1 HEAD → 6940d02 633c110 d85eeea      ← TWO parents
MERGE_HEAD after: gone
```

**Gap:** the whole-index door concludes the user's merge under jigc's Conventional-Commits subject; the ack cannot express a second parent, `design/finalize.md`'s manifest has no cell for it, and "one task → one commit" quietly becomes "one task → the merge commit". Not loss — the merge genuinely lands.

### **built + proven — the merge/`core.bare` refusals are state-safe, and the record-only rollback holds**

**Status:** built + proven. **Evidence:** under `fatal: cannot do a partial commit during a merge`, `milestone create` exit 1 with `git ls-files -s` sha **identical**, record file gone, workbench gone, `jigc validate` clean; `task discard` (sub-task) exit 1 with the record still naming the sub-task and the working area intact; `setup` exit 1 with `setup.install-commit` and everything left staged (declared). Under `core.bare=true`, `milestone create` exit 1 and the record file + workbench both rolled back, tree clean once `core.bare` is restored. `rename` refuses ahead of all of it with `rename.dirty-tree`, routed.

### **latent defect — the survivable frame's fixed sentence assumes a hook that did not speak**

**Status:** latent defect (law 1, un-swept axis). **Evidence:** the frame has **one** home, `render.rs:1851`:

```rust
"{rejection}\n\n{survived}. Fix the hook's complaint, then re-run {fence}{rerun}{fence}."
```

while `CommitRejected`'s own doc-comment (`task.rs:3478`) correctly says *"the user's `pre-commit` / `commit-msg` hook **(or git itself)** refused"*. Driven on three doors with a non-hook cause:

```
milestone create  → "fatal: cannot do a partial commit during a merge. … Fix the hook's complaint,
                     then re-run `jigc milestone create probe`."     (can only fail identically)
migrate-corpus    → same sentence, same dead-end route
task discard      → same sentence, same dead-end route
```

**Gap:** the frame's cause vocabulary is one member wide. The state-truth clause and the `error_code` are correct (`milestone-create.commit-rejected` read back from the log); only the diagnosis and the route are.

### **latent defect — N20 reproduced at HEAD and widened past its recorded scope**

**Status:** latent defect, **carried** (`implementation/decisions-pending.md:597`). **Evidence, `squash: true` with ordinary untracked main-checkout WIP:**

```
jigc milestone finalize probe-milestone → EXIT=1
  `git merge --ff-only 0273928…` failed: error: The following untracked working tree files
  would be overwritten by merge:  src-alpha-task.txt … Aborting
invocation log:  exit 1, error_code: None, finding_codes: []
record status:   status: active     (state-safe)
```

**Widening, driven:** the *same* frame loss reproduces on a **third, unrelated** cause — the empty-tree base at `milestone finalize` above (`git worktree add … is a tree, not a commit`, `error_code: null`). EC-37 already corrected N20's `squash: true` conditional; the further correction is that the class is **every non-`CommitRejected` boundary error**, because `task.rs:3534` renders the frame only on the downcast and `git_run`/`git_worktree` failures are plain `anyhow`. N20's own trigger — *"the next wave that opens `surface_commit_rejection`'s downcast"* — is the right one; its recorded subject is one cause too narrow.

---

## 3. The fan-out worktree path

**Status:** built + proven (posture), with one composed-flow break below.

- `provision_worktrees` (`milestone.rs:2139`) → `git worktree add --detach <path> <base_sha>` (`milestone.rs:2251`). **Driven:**
  `git worktree list` → `.jigc/worktrees/first-sub-task  9a853a7 (detached HEAD)`; `git -C <wt> symbolic-ref -q HEAD` exits 1 for each. Never a branch, always the milestone's **base pin** (not HEAD).
- `join` writes no commit (driven: HEAD unchanged, exit 0).
- `milestone finalize` reaches the main checkout twice: it commits inside a **second** detached worktree (`DedicatedWorktree::add`, `task.rs:4599`) and then lands with `git merge --ff-only` in the live checkout.
- **Exemption surface for a "HEAD on a branch" rule:** (i) every `git_commit` whose `repo_root` is a `DedicatedWorktree` — otherwise both boundary arms would refuse themselves; (ii) nothing on the sub-task worktrees, which never commit. That fence is **built and proven**: `jigc task finalize <sub>` from inside a sub-task worktree → **exit 3**, `blocking · finalize.milestone-sub-task`, routed at `jigc milestone finalize probe-milestone`.

### **latent defect (new, composed-flow) — the sub-task workflow's own `resume:` line does not provision its commit doc**

**Status:** latent defect. **Evidence**, two fresh sub-tasks in one corpus:

```
# ARM 1 — the composed footer's own resume line
cd .jigc/worktrees/alpha-task && jigc start --task alpha-task      → EXIT=0
ls .jigc/tasks/alpha-task/docs/  → No such file or directory
jigc doc set-field commit:alpha-task#type --task alpha-task --value feat
  → "no staged instance for `commit:alpha-task#type` — task alpha-task's workflow provisions
     its `commit` doc at compose and grants no in-task create for it; list what task
     alpha-task stages with `jigc doc list --task alpha-task`"
jigc doc list --task alpha-task  → "no docs staged in task alpha-task"

# ARM 2 — the spawn line milestone execute prints
cd .jigc/worktrees/beta-task && jigc workflow sub-task --task beta-task  → EXIT=0
ls .jigc/tasks/beta-task/docs/   → commit:beta-task.md   provenance.json
```

`milestone execute` prints `Spawn: cd .jigc/worktrees/alpha-task && jigc workflow sub-task --task alpha-task` (provisions), while the composed step's own footer prints `resume: jigc start --task alpha-task — re-composes this workflow if context is lost` (does **not** provision). **Gap:** a sub-agent whose first or resumed entry is the `resume:` line gets a workflow it cannot author, and the refusal's route (`jigc doc list --task …`) prints an empty list and never names the verb that provisions. Recoverable by running the workflow verb — nothing tells you to. This is the M31→M32 lesson's exact shape, on the surface built to answer it.

---

## 4. The carryover gate (M43)

**Status:** built + proven for the three doors it binds; **latent defect** at `setup`.

- Decision: `engine::finalize::decide_carryover` (`crates/engine/src/finalize.rs:735`), one blocking `finalize.carried-staged` per carried path, exempting the migration retire path and (task boundary only) recorded owner-artifacts. `None` snapshot ⇒ fail-open (declared bound).
- Snapshot written at `engine::state::MINT_DOORS` (`state.rs:808`) — 5 members, 3 `Snapshot::Written` (`jigc start`, `jigc migrate`, `milestone create`), 2 `Snapshot::Exempt` with stated reasons (`add_task`, `reseed_sub_task_areas`).
- **Consumers — three, all in the finalize family:** `task.rs:1429` (`task validate` preview), `task.rs:1797` (`task finalize`), `milestone.rs:3965` (`milestone finalize`). **7 of 10 committing doors are outside the gate.** Six of them commit pathspec-limited and structurally cannot absorb foreign work; `rename` guards with `rename.dirty-tree`.
- **Why `setup` is outside it:** it predates the gate, mints no task and therefore has no snapshot to compare against; its pathspec is presumed to be jigc-owned files.

**EC-26 reproduced verbatim:**

```
# CLAUDE.md committed, then edited by the user; feature.txt staged, unrelated
git status --porcelain → " M CLAUDE.md" / "A  feature.txt"
jigc setup → EXIT=0, ack mentions carryover nowhere
git show --stat HEAD → chore(jigc): install jigc workspace config
   .claude/settings.json | .claude/skills/jigc/SKILL.md | .jigc/* | CLAUDE.md | 8 files
git show HEAD:CLAUDE.md | grep -c precious → 1      ← the user's uncommitted line rode it
git status --porcelain → "A  feature.txt"           ← the unrelated staged file was left alone
```

**Gap:** not a pathspec leak — the swallow is confined to setup's own pathspec, which is exactly where a user's edits to `CLAUDE.md` / `.jigc/.gitignore` / `.claude/settings.json` also live. The gate's subject is a *task*; here it needs to be a *door*.

---

## 5. Rollback discipline — driven, with the exact residue set

`design/finalize.md:163-183`, five captured-pre-image families. Driven under a rejecting `pre-commit`:

| family | where captured / restored | driven result |
|---|---|---|
| promotions — worktree | `finalize.md:174` | **clean** (no `VISION.md` residue) |
| promotions — index | `:175` (pre-finalize index entry incl. *absent*) | **clean** — `git ls-files -s` sha identical |
| retirements (two-axis) | `:176` | not driven here — **unprobed, not cleared** |
| owner-artifact (third index axis) | `:177` | not driven here — **unprobed, not cleared** |
| **config layer** (fourth axis) | `:178` — *"worktree untouched"* **by design** | **residue** (below) |
| milestone record-only (fifth family, `commit_record_transaction` `milestone.rs:801`) | `:179` | **clean** — record file gone, workbench gone, index sha identical, `validate` clean |
| milestone record — fan-out stage | `:180` | **clean** — record still `status: active`, both worktrees present, `alpha-task` still holds `src-alpha-task.txt` staged, index sha identical |

**EC-29 reproduced; the exact residue set is two paths and no more:**

```
clean tree in → jigc task finalize <id> under a rejecting hook → EXIT=1
  "`git commit` was rejected (no commit was made): HOOK SAYS NO
   task <id> is intact — nothing was committed, your task's staged docs are still in
   .jigc/tasks/<id>/docs/, and anything you had `git add`-ed is still in git's index."
HEAD unchanged? YES     index (ls-files -s sha) unchanged? YES     task area intact? YES
git status --porcelain →
   M .jigc/.gitignore
   M .jigc/version
```

`.jigc/version` is left holding jigc's refreshed stamp. Design-declared (`:178` says "worktree untouched" on purpose), while the section header two rows above reads *"Before phase 6, all-or-nothing."* Reachable by the ordinary sequence *upgrade jigc → finalize → hook rejects*.

**This run also discharges the VERDICT's honest bound** that `milestone finalize` under a rejecting hook was graded from state checks rather than a sha-diff snapshot: driven with an index-sha comparison it is **zero residue**.

---

## 6. `gitignore::ensure` — EC-18 confirmed on both arms

**Status:** latent defect (recoverable loss + a doc-comment that states the opposite).

`crates/cli/src/gitignore.rs:32`. Whenever **any** `ENTRIES` line is missing it runs `std::fs::write(&path, ENTRIES)` — a **whole-file replace**. Its own doc-comment (`:29-30`) reads *"so an adapter-written `.gitignore` predating any later entry … is **amended once to the union**"*. Production callers: `adapter.rs:1108` (setup), `task.rs:2778` (finalize), `milestone.rs:472` (`milestone create`), `milestone.rs:2066` (`milestone provision`). `relocate.rs:666` is `#[cfg(test)]`.

**Negative arm** (hook rejects — the residue above): the user's `# my own note` and `scratch/` lines are already gone from the worktree copy.
**Positive arm** (clean finalize) — the deletion is **committed**:

```
committed before:  tasks/ index/ state/ milestones/ worktrees/ logs/ # my own note  scratch/
jigc task finalize <id> → EXIT=0
  "finalized 900df6f … modified .jigc/.gitignore / promoted VISION.md / 2 files committed"
committed after:   tasks/ index/ state/ milestones/ worktrees/ logs/ displaced/
git show --stat HEAD → .jigc/.gitignore | 3 +--
```

**Gap:** the manifest names the file (`modified .jigc/.gitignore`) but nothing names the **content loss**. `SKILL.md` already has the right shape for this class (refuse to clobber once edited).

---

## 7. Carried / deferred in this area

- **N20** (`implementation/decisions-pending.md:597`) — the only committing-door entry in the file. Reproduced at HEAD and **widened**: see the N20 row above. Its recorded scope names one cause (`--ff-only` under `squash: true`); EC-37 removed the knob conditional and this baseline removes the cause conditional.
- `N15`, `N23`, `N26`, `N27`, `N28`, `N31` (struck) touch other areas; none is a committing-door entry.
- `decisions-pending.md:35` places **N20** in the rc.15 Tier 1 list, alongside EC-3/4/5/6/7/8/9/10 and F-5/F-11/N23.

---

## Honest bounds of this baseline

- **Not driven, therefore not cleared:** the **retirements** and **owner-artifact** rollback axes under a rejecting hook (the migration retire byte-capture set and the recorded `owned-location` index pre-images); `migrate-corpus --approve`-style rollback with pending migrations under a hook; a rebase (as opposed to merge) in progress — I drove `.git/MERGE_HEAD` only, not `.git/rebase-merge`; `GIT_WORK_TREE`/`GIT_INDEX_FILE` set oddly (only `GIT_DIR` was driven); and concurrent commits at two doors racing one index.
- The linked-worktree cell was driven for `task finalize`, `start`, `milestone create` and `milestone add-task` only — the other six committing doors are inferred to follow `milestone create` because they share `commit_record_transaction`, but that inference is **not driven**.
- The `hook_output_axis.rs` membership gap is a **source-read** conclusion (its enumeration is prose that no assertion ties back to `COMMITTING_DOORS`), not a driven one.
- Everything else above carries a quoted argv + exit + observed output or a `file:line`.
