Verified at `HEAD` = `978577ec` on the installed release `jigc 1.0.0-rc.16`, git 2.54.0 (Apple Git-157). **This ledger is a map, not gospel.** Nothing was edited; no `cargo` ran.

# Row `(2, DEFECT A)` — baseline ledger

## 1 · The cell, driven

**DRIVEN — confirmed exactly as the review states it.** `dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc --start decided-task "axis intent"`, then:

```
$ printf 'a\n' > g1.txt; git add g1.txt; git commit -q -m base
$ git checkout -q -b cpb; printf 'PAYLOAD\n' > g2.txt; git add g2.txt
$ git commit -q -m 'the users cherry-pick target'; P=$(git rev-parse HEAD); git checkout -q main
$ git cherry-pick -n "$P"                          -> rc=0
$ ls -A .git | grep -E 'CHERRY_PICK_HEAD|sequencer|REVERT_HEAD|MERGE_HEAD|SQUASH_MSG|MERGE_MSG|AUTO_MERGE'
  AUTO_MERGE
  MERGE_MSG                                        <- the only two things git left
$ git ls-files -u | wc -l                          -> 0
$ cat .git/MERGE_MSG                               -> the users cherry-pick target
$ git diff --cached --name-status                  -> A  g2.txt

$ jigc doc set-field commit:axis-intent#header/type --task axis-intent --value docs   -> rc=0
$ printf 'jigc own work\n' | jigc doc set-slot commit:axis-intent#summary --task axis-intent --from-file -  -> rc=0
$ jigc task validate axis-intent
  no findings — the task validates clean                                       EXIT=0   <- says NOTHING about the pick
$ jigc task finalize axis-intent
  no findings — the task validates clean
  finalized c2b0b99 — docs: jigc own work
    added g2.txt
    1 file committed                                                           EXIT=0

$ git log -1 --format=%B
  docs: jigc own work
$ git log -1 --name-only --format=''   -> g2.txt        <- the user's un-concluded pick, committed under jigc's subject
$ test -f .git/MERGE_MSG && echo PRESENT || echo GONE -> GONE
$ git cherry-pick --continue
  error: no cherry-pick or revert in progress
  fatal: cherry-pick failed                                                    rc=128
```

**New datum vs. the review:** `AUTO_MERGE` is also written (git's `ort` auto-merged-tree cache). It is **not** in `MARKER_UNIVERSE` (`crates/cli/tests/support/git_state.rs:244`) and is written by clean `stash apply/pop` and a concluded rebase too, so it is **not a usable discriminator** — noted so a plan does not reach for it.

**Honest bound on the loss:** the picked commit's message is still reachable from the source branch in this fixture. What is unrecoverably destroyed is `.git/MERGE_MSG` itself (the pending, user-editable message) and the operation state; the *silent conclusion* is the harm, and it is identical to the `SquashMerge` cell `design/validation.md` already records as the reason that member exists.

---

## 2 · The detection truth table — DRIVEN, git 2.54.0

Every row is a fresh `mktemp -d` repo driven with real git. `COMMIT_EDITMSG` is present in every repo with one commit and is omitted. `AUTO_MERGE` is listed because it surprised me; it is outside `MARKER_UNIVERSE`.

| # | git act | markers left | unmerged | HEAD | charter predicate `MERGE_MSG ∧ ¬(MERGE_HEAD∨CHERRY_PICK_HEAD∨REVERT_HEAD∨SQUASH_MSG)` |
|---|---|---|---|---|---|
| A | `cherry-pick -n` clean, 1 commit | `MERGE_MSG`, `AUTO_MERGE` | 0 | attached | **MATCH** ← the defect |
| B | `cherry-pick -n main..side` clean, **2 commits** | `MERGE_MSG`, `AUTO_MERGE` — **no `sequencer/`** | 0 | attached | **MATCH**; `MERGE_MSG` holds only the **last** commit's message (`pick two`) — the first's is already gone by git's design |
| C | `cherry-pick -n` **CONFLICTED** | `MERGE_MSG`, `AUTO_MERGE` — **no `CHERRY_PICK_HEAD`** | 3 | attached | **MATCH** (today answered by `UnmergedIndex`) |
| T | `cherry-pick` conflicted **without** `-n` (contrast) | `CHERRY_PICK_HEAD`, `MERGE_MSG`, `AUTO_MERGE` | 3 | attached | no |
| U | C, then `git add` the resolution | `MERGE_MSG`, `AUTO_MERGE` | **0** | attached | **MATCH** — byte-identical to A; today answered by **nothing** |
| D | `revert -n` **clean** | `REVERT_HEAD`, `MERGE_MSG`, `AUTO_MERGE` | 0 | attached | no — `REVERT_HEAD` is written **unconditionally**, caught today |
| E | `revert -n` conflicted | `REVERT_HEAD`, `MERGE_MSG`, `AUTO_MERGE` | 3 | attached | no |
| F | `merge --no-commit --no-ff` clean | `MERGE_HEAD`, `MERGE_MSG`, `AUTO_MERGE` | 0 | attached | no |
| G | `merge --squash` clean | `SQUASH_MSG`, `AUTO_MERGE` — **no `MERGE_MSG`** | 0 | attached | no |
| H | `merge` **fast-forward** | — | 0 | attached | no |
| I | `merge --no-ff -m` concluded | — | 0 | attached | no |
| J | `stash apply` clean | `AUTO_MERGE` | 0 | attached | no |
| K | `stash pop` clean | `AUTO_MERGE` | 0 | attached | no |
| L | `stash pop` **conflicted** | `AUTO_MERGE` | 3 | attached | no — `UnmergedIndex`'s own fixture, **unaffected** |
| Z5 | `git am` paused | `rebase-apply/` (`applying`) | 0 | attached | no |
| Z6 | rebase concluded | `AUTO_MERGE` | 0 | attached | no |
| Z7 | rebase **paused** (merge backend) | `rebase-merge/`, `MERGE_MSG`, `AUTO_MERGE` | 3 | **detached** | **MATCH — FALSE POSITIVE against `Rebase`** |
| Z11 | `bisect start` | `BISECT_LOG`, `BISECT_START` | 0 | attached | no |
| Z12 | `cherry-pick -n -m 1 <merge>` | `MERGE_MSG`, `AUTO_MERGE` | 0 | attached | **MATCH** |

**Same table driven through the 12 rig git-states** (`dev/jigc-rig --git-state <m>`, marker presence read at `git rev-parse --git-path`):

```
state                MMSG   MHEAD  CPHEAD RVHEAD SQMSG  charter-predicate | other markers
merge                Y      Y      n      n      n      NO        |
squash-merge         n      n      n      n      Y      NO        |
rebase-merge         Y      n      n      n      n      **MATCH** | rebase-merge
rebase-apply         n      n      n      n      n      NO        | rebase-apply
am                   n      n      n      n      n      NO        | rebase-apply
cherry-pick          Y      n      Y      n      n      NO        |
sequencer            Y      n      Y      n      n      NO        | sequencer
dangling-sequencer   n      n      n      n      n      NO        | sequencer
revert               Y      n      n      Y      n      NO        |
unmerged-index       n      n      n      n      n      NO        |
bisect               n      n      n      n      n      NO        | BISECT_LOG
detached             n      n      n      n      n      NO        |
```

**FALSE POSITIVES — the hunt, DRIVEN. Result: none found in any concluded state.**

| candidate false positive | driven result |
|---|---|
| ordinary `git commit` rejected by a failing **pre-commit** hook | markers: none. `MERGE_MSG` is never created by a commit. **No FP.** |
| ordinary `git commit` rejected by a failing **commit-msg** hook | markers: none. **No FP.** |
| `merge --no-commit`, then commit rejected by pre-commit hook | `MERGE_HEAD` + `MERGE_MSG` → answered `Merge` today. **No new FP.** |
| concluded merge, then a **later** ordinary commit hook-rejected | markers: none. **No FP.** |
| jigc's own hook-rejected finalize | jigc shells `git commit`; same as above — creates no `MERGE_MSG`. **No FP** (READ + implied by the two hook rows). |
| `merge` fast-forward (H) / `pull --no-commit` fast-forwardable (Z1) | markers: none. **No FP.** |
| `pull --no-commit --no-ff` (Z2) | `MERGE_HEAD` + `MERGE_MSG` → `Merge`. **No new FP.** |
| `merge --squash` concluded by `git commit` (Z3) | markers: none. **No FP.** |
| conflicting merge then `merge --abort` (Z4) | markers: none. **No FP.** |
| `cherry-pick -n` then `git checkout -b other` (Z8) | **`MERGE_MSG` cleared**, pick stays staged — git drops the pending state itself |
| `cherry-pick -n` then `git stash push -u` (Z9) | `MERGE_MSG` cleared, tree clean |
| IDE/GUI clients | **NOT DRIVEN** — outside this environment; the only mechanism by which a lingering `MERGE_MSG` could exist with nothing pending would be a crashed git process. Declare as a bound. |

**The only real false positive is `rebase-merge` (Z7)** — and it is an *intra-family* one, resolvable by probe order (see §4).

**FALSE NEGATIVES of the charter's predicate — two, both DRIVEN:**

1. **Row U** — a conflicted `cherry-pick -n` **that the user has already `git add`-ed** is byte-identical to row A (`MERGE_MSG` only, index merged). Today answered by **no member at all** — same exit-0 swallow. The charter's predicate **does** catch it. Good.
2. **Row C** — a conflicted `cherry-pick -n` (unresolved). Today answered as `UnmergedIndex` (*"a conflict"*, route `git reset --merge`). The charter's predicate matches it **before** `UnmergedIndex` (which is probed last), so this cell **changes noun and route**. That is arguably a *correction* (the state is a cherry-pick, not an anonymous conflict) but it is a behaviour change the plan must decide, and the new route must work on a conflicted index — **DRIVEN: it does** (§4 route table).
3. `revert -n` clean (D) is **not** an identical state: git writes `REVERT_HEAD` unconditionally, so it is already a member. The asymmetry the review names is confirmed. **No coverage owed there.**

---

## 3 · The code, read (file:line) — **READ**

| thing | home |
|---|---|
| `InProgress` enum + doc-comment (*"the member set is the operations git can leave un-concluded, not the markers it writes"*) | `crates/cli/src/repo.rs:169–222` |
| `InProgress::ALL` — `[InProgress; 9]`, **probe order is load-bearing**, doc-comment names the three orderings | `crates/cli/src/repo.rs:231–241` |
| `InProgress::detect(git_dir, repo_root)` — per-variant predicate | `crates/cli/src/repo.rs:249–274` |
| `noun()` · `predicate()` · `conclude_command() -> Option<&str>` · `abandon() -> &'static str` | `repo.rs:278–290` · `295–302` · `312–323` · `331–348` |
| `operation_in_progress` — **`InProgress::ALL.into_iter().find(…)`, first match wins** | `crates/cli/src/repo.rs` (just below `index_has_unmerged_paths`) |
| `index_has_unmerged_paths` — the one shell-out (`git ls-files -u`), fails **open** | `repo.rs:352–362` |
| `posture(repo_root)` — resolves `worktree_git_dir` first, returns `[]` if no `HEAD`; operation probed **before** detached | `repo.rs:748–783` |
| `worktree_git_dir` — `.git` dir, else parses the `gitdir:` pointer → **per-worktree git dir** | `repo.rs:852–871` |
| `PostureBreach::finding()` — `Finding::block(code, message, Route::human(route))`; route mold `"conclude it with \`X\` once its conflicts are resolved, or abandon it with \`Y\`"` / `"conclude it, or abandon it with \`Y\`"` | `repo.rs:~390–437` |
| finding code — **one**, `repo.operation-in-progress` (`PostureMember::code`) | `repo.rs:148–154` |
| `BEHALF_DOORS` — 47 rows, total classification | `crates/cli/src/cli.rs:2003` |
| the door guard `refuse_on_posture` / `posture_refusal_in` | `crates/cli/src/cli.rs:576–620` |
| the `task validate` preview `finalize_posture_refusal` (one producer, same `ActsOnBehalf` row) | `crates/cli/src/cli.rs:~622+` |
| seam re-probe consumers | `task.rs:6609`, `task.rs:6818` (`SeamAct::Commit`); `relocate.rs:238`, `relocate.rs:802` (`SeamAct::Move`); `setup.rs:2417` (live_exempt `HeadUnborn`) |

**Route construction:** `Route::human(String)` only — a single free-text string. It already carries **two** alternatives today (`conclude … or abandon …`), so "two alternative commands" is **not** a missing capability.

**The "each route is run and accepted by git" fence — two suites:**
- `crates/cli/tests/repo_posture.rs:449` `every_git_state_names_its_own_operation_and_a_route_git_accepts` — extracts the argv from the **rendered** bytes after the literal lead `ABANDON_LEAD = "abandon it with \`"` (`repo_posture.rs:~527`), runs it with `Command::new(argv[0]).args(…)` in the fixture repo, asserts exit 0 **and** that `posture()` afterwards holds no operation. It also asserts `InProgress::ALL` is **surjectively covered** by `GitState::ALL` (`repo_posture.rs:523–531`).
- `crates/cli/tests/flow53_acceptance.rs:864+` `every_operation_refuses_at_every_acting_door_and_its_route_concludes_it` — iterates `InProgress::ALL × BEHALF_DOORS`' acting rows, runs the abandon argv **through a real shell** (`support::shell_words`), `state_producing(op)` panics if no fixture produces a member (`flow53_acceptance.rs:817`).

**Fixture builders / fences that must gain the state:**

| home | what is owed |
|---|---|
| `crates/cli/tests/support/git_state.rs:72` `GitState` enum + `:135` `ALL` + `:154` `name()` + `in_progress()` map (exhaustive match) | a new variant |
| `crates/cli/tests/support/git_state.rs:282` `EXPECTATIONS` table | one row `markers: &["MERGE_MSG"], head: Attached, unmerged: 0` — **the table already supports 0, 2 and 4 markers per state, so one-marker-per-state is NOT a constraint** |
| `crates/cli/tests/support/git_state.rs:244` `MARKER_UNIVERSE` | **no change** — `MERGE_MSG` is already a member; `AUTO_MERGE` is deliberately outside it |
| `dev/jigc-rig:178` `GIT_STATES` array + `:207` `git_state_expect` case + a `gen_git_state_<name>()` generator (`dev/jigc-rig:772` is the ~4-line cherry-pick template) | three small edits |
| `dev/jigc-rig:110` help text *"(`--list-git-states` prints **the twelve**)"* | prose numeral — **NOT fenced by any assertion** (grep: no `assert_eq!(…len(), 12)` anywhere) |
| `crates/cli/tests/dev_rig_parity.rs:200` set-equality + `:247` surjectivity onto `InProgress::ALL` | satisfied automatically once both sides gain the member |
| `crates/cli/tests/git_state_fixtures.rs:70` `GitState::ALL.len()`, `:41` universe cross-check | **derived** — no numeral |
| `crates/cli/tests/posture_door_axis.rs:350` `acting_rows * GitState::ALL.len()` | **derived** |

**Every occurrence of the numeral *nine* for this family (grep):**

| file:line | text |
|---|---|
| `crates/cli/src/repo.rs:231` | `pub const ALL: [InProgress; 9]` — the array length, compiler-checked |
| `crates/cli/tests/repo_posture.rs:13` | *"nine of them"* (module doc) |
| `crates/cli/tests/repo_posture.rs:431` | *"which of nine things they are in the middle of"* |
| `crates/cli/tests/repo_posture.rs:485` | *"one of nine things"* (assert message) |
| `crates/cli/tests/posture_member_inventory.rs:12` | *"`InProgress::ALL` now carries nine members"* |
| `crates/cli/tests/flow53_acceptance.rs:915` | *"one of nine things"* (assert message) |
| `crates/cli/tests/posture_door_axis.rs:273` | *"one of nine things"* (assert message) |
| `crates/cli/src/repo.rs:~285` (`noun()` doc) | *"a user in the middle of **one of nine things** learns which one"* |
| `design/validation.md` / `design/finalize.md` | **no numeral** — both state the member set as a table, not a count |

None of these nine is a *fence*; all are prose. `crates/cli/tests/count_fences.rs:1296` names `posture_member_inventory.rs` as the posture family's count home but fences no integer for it.

**"Kept true by generation" — what it means concretely today: `crates/cli/tests/posture_member_inventory.rs`.** Arm 1 `both_prose_homes_state_the_whole_operation_inventory` derives the expected inventory as `InProgress::ALL.map(|op| (op.noun(), op.abandon()))` and asserts **vector equality in probe order** against the tables parsed out of `design/finalize.md:37` and `design/validation.md:776` (header prefix `| the operation | the route abandons it with |`, exactly one table per home). **So two of the three homes are generation-fenced; the enum's own doc-comment is the third and is fenced by nothing.** A tenth member reddens both docs until both name it.

---

## 4 · The halt question — answered

**Verdict: this is a new enum variant + table rows + one fixture state. No mechanism is missing.** Four things the plan must decide, none of them new machinery:

| # | concern | status | evidence |
|---|---|---|---|
| 1 | *"the detection function has no way to express a conjunction with negations"* | **FALSE — it already does.** `InProgress::SquashMerge => present("SQUASH_MSG") && !present("MERGE_HEAD")` and `Rebase => present("rebase-merge") \|\| (present("rebase-apply") && !present("rebase-apply/applying"))` | `crates/cli/src/repo.rs:253–262` (READ) |
| 2 | *"the fixture builder asserts one-marker-per-state"* | **FALSE.** `EXPECTATIONS` rows already carry 0 (`unmerged-index`), 2 (`merge`) and 4 (`sequencer`) markers | `support/git_state.rs:282–398` (READ) |
| 3 | *"the route type can't express two alternative commands"* | **FALSE.** `Route::human(String)` is free text and the shipped mold already prints two: `"conclude it with \`X\` … or abandon it with \`Y\`"` | `repo.rs:~408–424` (READ) + DRIVEN contrast: `jigc task validate` under `--git-state merge` prints ``conclude it with `git merge --continue` … or abandon it with `git merge --abort` `` |
| 4 | **Probe-order dependency** — the charter's predicate is **true in the `rebase-merge` state** | **REAL, DRIVEN.** Must either sit **after** `Rebase` in `ALL` (first-match wins in `operation_in_progress`) or add `∧ ¬rebase-merge ∧ ¬rebase-apply`. The enum's own doc-comment already documents both styles (`Am`/`Rebase` disjoint *by predicate*; `Sequencer` disjoint *by position*), so either is in-house | rig table, §2 |

**Two further constraints the plan must honour — both DRIVEN, neither a halt:**

- **`MERGE_MSG` is per-worktree.** In a linked worktree, `git rev-parse --git-path MERGE_MSG` → `.git/worktrees/<name>/MERGE_MSG`; `.git/MERGE_MSG` does **not** exist. `detect()` is already handed `git_dir` from `worktree_git_dir` (`repo.rs:852`), so **detection is already correct** — but a route literal spelling `rm .git/MERGE_MSG` would be **false inside a fan-out worktree**, and `milestone finalize` reaches exactly there.
- **`abandon()` returns one `&'static str` and `repo_posture.rs`/`flow53` run it verbatim.** Candidate abandon commands, all DRIVEN from the clean `-n` state (and the conflicted one):

| candidate | rc | `MERGE_MSG` | picked payload | staged | note |
|---|---|---|---|---|---|
| `git reset` (mixed) | 0 | cleared | **kept** in worktree | unstaged | **works on the conflicted cell too** (rc 0) — the least destructive git-native drop |
| `git reset --merge` | 0 | cleared | **GONE** | — | what `SquashMerge`/`UnmergedIndex` use; here it **destroys the user's picked bytes** |
| `git reset --hard` | 0 | cleared | **GONE** | — | same loss |
| `git commit --no-edit` | 0 | cleared | kept | committed **with the picked message** | the *conclude* half |
| `rm .git/MERGE_MSG` | 0 | cleared | kept | **stays staged** | runnable through the harness (`Command::new("rm")`), but not a git command, and **wrong in a worktree** |
| `git cherry-pick --abort` | **128** | survives | — | — | *"no cherry-pick or revert in progress"* — **not a valid route** |
| `git merge --abort` | **128** | survives | — | — | *"There is no merge to abort (MERGE_HEAD missing)"* — **not a valid route** |

So the charter's prescribed `rm .git/MERGE_MSG` is the only one that preserves the staged pick, and it is the one that is not git-native and not worktree-correct. `git reset` is git-native, worktree-correct, non-destroying on both the clean and conflicted cells — at the cost of unstaging. **This is a Settle fork, not a halt.**

**Noun collision:** `CherryPick`'s noun is already `"a cherry-pick"`; the inventory fence keys rows on `(noun, abandon)` pairs, so a duplicate noun is representable but would make both design tables ambiguous to a reader. A distinct noun is owed.

---

## 5 · Sibling cells — every acting door driven under the clean `cherry-pick -n` state

`BEHALF_DOORS`' **12 acting rows** (read: `crates/cli/src/cli.rs:2003`; 10 `CommitsOnBehalf` + 2 `MovesOnBehalf`, 35 `Neither`). Each row below is a **fresh rig** (`committed-singletons` + `--start decided-task` + a committed milestone `axis-milestone` with sub-task `second-intent`, then the clean pick). **DRIVEN.**

| door | class | exit | `MERGE_MSG` | HEAD | pick payload |
|---|---|---|---|---|---|
| `task finalize axis-intent` | Commits | **0** | **DESTROYED** | moved | **committed under jigc's subject** ("swallowed") |
| `milestone finalize ms-one` | Commits | **0** | **DESTROYED** | moved | **left staged** — the ack even *names* it (`staged in the shared checkout … g2.txt`) while silently killing the message |
| `milestone create "Second Milestone"` | Commits | **0** | **DESTROYED** | moved (record-only) | left staged |
| `milestone add-task axis-milestone "third intent"` | Commits | **0** | **DESTROYED** | moved (record-only) | left staged |
| `milestone discard axis-milestone` | Commits | **0** | **DESTROYED** | moved (record-only) | left staged |
| `config set docs-root docs2` | **Moves** | **0** | survives | unmoved | **`git mv`s a committed doc *into* the user's pending pick's index** — the user's later `git commit` would sweep jigc's move into the picked message |
| `setup` | Commits (exempt unborn) | 0 | survives | unmoved | no commit made (already set up) |
| `migrate-corpus` | Commits | 0 | survives | unmoved | nothing to migrate → no commit |
| `rename vision --to "Axis Vision"` | Commits | 1 | survives | unmoved | refused for its own reason (not a `<type>:<slug>` address) |
| `milestone add-from-spec axis-milestone spec:nope` | Commits | 1 | survives | unmoved | `store.not-found` |
| `task discard axis-intent` | Commits | 1 | survives | unmoved | `task-discard.staged-prose` |
| `relocate vision --from docs/vision/` | **Moves** | 1 | survives | unmoved | `relocate.frozen-doctype` |

Controls / non-`BEHALF_DOORS` rows driven for completeness:

| door | class | exit | observed |
|---|---|---|---|
| `task validate axis-intent` | Neither | **0** | `no findings — the task validates clean` — **the posture preview says nothing.** Contrast DRIVEN under `--git-state merge`: exit **1**, `blocking · repo.operation-in-progress — a merge is in progress` |
| `start` | Neither | 0 | orientation prints `findings: none` and no posture line (the separately-tracked tier-2 row `(6, D-1)`) |
| `uninstall` | Neither (destroying) | 1 | `uninstall.staged-prose` |
| `ingest` | Neither | 0 | register-only, 9 candidates classified, no commit |
| `unmanage vision` | Neither | 0 | no-op |
| `doc show vision` | Neither | 0 | reads clean |
| `migrate foreign.md --as adr --approve` | Neither | 2 | **`--approve` is not a `migrate` flag** (`Usage: jigc migrate --as <AS> <PATH>`); the approve arm is `jigc task finalize <migration-task> --approve`, i.e. it **inherits `task finalize`'s row** — **READ, not driven** |

**Three distinct damage shapes, not one** — this is the axis a fix must iterate:
1. **swallow** (`task finalize`): the pick's payload is committed under jigc's subject and the message dies;
2. **message-only kill** (the three `milestone` record-only doors, and `milestone finalize`): jigc's own commit consumes `MERGE_MSG` while leaving the pick staged — **silent, and invisible until `git cherry-pick --continue` fails**;
3. **index contamination** (`config set docs-root`, a mover): jigc stages a `git mv` into the user's pending pick, so the *user's* conclusion commits bytes jigc placed there.

The review recorded only (1). Shapes (2) and (3) are new here.

**The carryover gate is not a backstop for the common ordering** — the review's own driven cells, re-read and consistent with mine: when the pick **post-dates** the task there is no gate at all (the exit-0 path above); when it **pre-dates**, `finalize.carried-staged` blocks at exit 3 naming a *path*, and `--carry-staged` (a *staging* consent) then concludes the pick at exit 0.

---

## 6 · Doc homes and pinning suites

**`design/finalize.md:31` (§1. Preflight)** — quoted verbatim:

> **The repository posture is one this door may act in.** **Three** predicates, not one — **HEAD detached** · **HEAD unborn** · **an operation in progress**, the third being **any operation git can leave un-concluded** rather than a list of the markers git happens to write …

and its fenced table at `design/finalize.md:37`:

```
| the operation | the route abandons it with |
|---|---|
| a merge | `git merge --abort` |          | a cherry-pick | `git cherry-pick --abort` |
| a squash merge | `git reset --merge` |   | a revert | `git revert --abort` |
| a rebase | `git rebase --abort` |        | a cherry-pick or revert | `git cherry-pick --quit` |
| a `git am` | `git am --abort` |          | a bisect | `git bisect reset` |
                                           | a conflict | `git reset --merge` |
```

**`design/validation.md:664`** — the `repo.operation-in-progress` registration row:

> **any operation git can leave un-concluded**, started by the user and not concluded — the member this doc has promised since it was written. … The member set is now the operations themselves, `cli::repo::InProgress::ALL`, each with its own detector, noun and command … Route: **`Human`, naming the command that concludes or abandons *this* operation** — never a menu

and its three-column inventory at `design/validation.md:776` (same nine rows plus a *why* column). Two rows are the precedent the fix rides:

> \| a squash merge \| `git reset --merge` \| `SQUASH_MSG` with **no** `MERGE_HEAD` … **Driven at the M52 baseline as the worst cell: the entire squashed payload landed inside jigc's own commit at exit 0 and the merge's authored message was destroyed with `SQUASH_MSG`.** …
>
> \| a revert \| `git revert --abort` \| `REVERT_HEAD` … Driven: *every* commit door concluded the user's revert at exit 0, after which `git revert --continue` answered *"no cherry-pick or revert in progress"*.

**Third home, unfenced:** `crates/cli/src/repo.rs:169–176` — *"**The member set is the operations git can leave un-concluded, not the markers it writes**."*

**Suites pinning the family, and what each iterates:**

| suite | iterates |
|---|---|
| `crates/cli/tests/repo_posture.rs` | `GitState::ALL` × 4 claims/member, incl. running the abandon argv out of the **rendered** bytes; + surjectivity `GitState::ALL ↠ InProgress::ALL` |
| `crates/cli/tests/posture_member_inventory.rs` | `InProgress::ALL` mapped through `noun()`/`abandon()` vs. the two design-doc tables (equality, in probe order) + the `git 2.54.0` deferral subject/trigger + the struck-claim guard |
| `crates/cli/tests/posture_door_axis.rs` | `GitState::ALL` × `BEHALF_DOORS`' acting rows (`acting_rows * GitState::ALL.len()` cells); owns each cell's route text |
| `crates/cli/tests/flow53_acceptance.rs` (arm 2) | `InProgress::ALL` × acting rows; route run **through a real shell**; asserts the whole sweep **consumes no marker** |
| `crates/cli/tests/dev_rig_parity.rs` | rig `--git-state` set ⇔ `GitState::ALL`; complement is exactly `{detached, unborn}`; surjectivity onto `InProgress::ALL` |
| `crates/cli/tests/git_state_fixtures.rs` | `EXPECTATIONS` rows ⊆ `MARKER_UNIVERSE`; each of the 12 built from scratch and overlaid |
| `crates/cli/tests/flow52_acceptance.rs` | `BEHALF_DOORS` × `PostureMember::ALL` (the M51 cross) |

**Note for the acceptance design:** `flow53`'s arm 2 asserts *"the whole sweep consumes no marker"* by reading the fixture's own driven expectation back after every acting door has run. Under the new member's fixture that assertion is exactly the regression fence this row needs — it would be **red today**, since (per §5) five doors consume `MERGE_MSG`.

---

## Ledger fragment — classification

| capability | status | evidence | gap |
|---|---|---|---|
| The `InProgress` family detects an un-concluded operation and refuses at every acting door | **shape-limited** | DRIVEN §2 (12/12 rig states answer correctly) + §5 contrast (`--git-state merge` → exit 1 at the preview) | The unexercised shape is **`git cherry-pick --no-commit` that applies cleanly** — and its two siblings, a **multi-commit range** `-n` (no `sequencer/` written) and a **conflicted `-n` whose conflicts the user has resolved** (`git add`), both byte-identically `MERGE_MSG`-only |
| `jigc task finalize` under a clean `cherry-pick -n` | **latent defect** | DRIVEN §1, exit 0, `git log -1 --format=%B` = `docs: jigc own work`, `g2.txt` in that commit, `.git/MERGE_MSG` gone, `git cherry-pick --continue` → 128 | Exit-0 commit of a third party's payload under jigc's subject; the pending authored message destroyed |
| The four `milestone` doors under the same state | **latent defect, unreported** | DRIVEN §5 — `milestone create` / `add-task` / `discard` / `finalize` all exit 0, all destroy `MERGE_MSG`, all leave the pick staged | A record-only commit silently consumes the user's pending message. **Not in the review's row**; the fix's axis must cover it |
| `config set docs-root` (mover) under the same state | **latent defect, unreported** | DRIVEN §5 — exit 0, `git mv` staged **alongside** the pick | jigc's move joins the user's pending commit. **Not in the review's row** |
| `jigc task validate`'s posture preview | **shape-limited** | DRIVEN §5 — exit 0 `no findings` under the pick vs. exit 1 `repo.operation-in-progress` under `--git-state merge` | It is a faithful mirror of the door; it is silent here because the family is, not because the preview is broken. It corrects itself the moment the member lands |
| `design/finalize.md:31` / `validation.md:664` — *"any operation git can leave un-concluded"* | **claim-vs-reality discrepancy** | Both quoted §6; falsified by §1 | Marker-keyed for 8 of 9; the 9th (`git ls-files -u`) answers no on a clean `-n` |
| `posture_member_inventory.rs`'s generation fence over the two doc tables | **built + proven** | READ `posture_member_inventory.rs:152–190`; derivation is `InProgress::ALL.map(noun, abandon)`, equality in probe order | none — a tenth member reddens both homes. The **third** home (`repo.rs`'s doc-comment) is fenced by nothing |
| `repo_posture.rs` / `flow53` route-runnability fence | **built + proven** | READ `repo_posture.rs:449–531`, `flow53_acceptance.rs:864+`; argv extracted from rendered bytes after `ABANDON_LEAD` | It constrains the new member's `abandon()` to a command that exits 0 **and clears the state** — `git cherry-pick --abort` and `git merge --abort` both exit **128** here (DRIVEN §4) |
| Fixture substrate (`git_state.rs`, `dev/jigc-rig`, `dev_rig_parity.rs`) admits the new state | **built + proven** | READ §3 table; `EXPECTATIONS` already carries 0/2/4-marker rows; `MARKER_UNIVERSE` already holds `MERGE_MSG` | Three mechanical edits + one ~4-line generator; one **unfenced prose numeral** (`dev/jigc-rig:110` *"the twelve"*) |
| The charter's predicate as written | **latent defect in the prescription** | DRIVEN §2 — it **matches `rebase-merge`** (`MERGE_MSG` + `rebase-merge/`, none of the four negated markers) | Must sit **after** `Rebase` in `InProgress::ALL` (first-match in `operation_in_progress`) or add `∧ ¬rebase-merge ∧ ¬rebase-apply`. Both styles have in-house precedent |
| The charter's prescribed route `rm .git/MERGE_MSG` | **shape-limited prescription** | DRIVEN §4 + Z10 — `git rev-parse --git-path MERGE_MSG` in a linked worktree is `.git/worktrees/<n>/MERGE_MSG`, and `.git/MERGE_MSG` does not exist there | The literal is **false inside a fan-out worktree**, which `milestone finalize` reaches. `git reset` is git-native, worktree-correct and non-destroying on both the clean and conflicted cells (rc 0, payload kept, unstaged); `git reset --merge` — the siblings' route — **destroys the picked bytes** |
| False positives (the fix refusing a concluded repo) | **built + proven — none found** | DRIVEN §2, 12 candidate states incl. both hook-rejection shapes, fast-forward merge, `pull --no-commit` ff, concluded squash, `merge --abort`, concluded rebase | **Bound:** IDE/GUI clients and a crashed git process were **not driven** — declare as the family's existing `git 2.54.0` deferral does |

**Halt verdict: NOT a halt.** A new `InProgress` variant + a `detect` conjunction the enum already uses twice + one `EXPECTATIONS` row + one rig generator + two design-table rows the existing fence will demand. Three decisions are owed to Settle, none of them mechanism: (a) probe **position vs. negated conjunct** for the `rebase-merge` overlap; (b) the **abandon command** (`git reset` vs. the non-git-native `rm`, and whether the conflicted `-n` cell moves off `UnmergedIndex`); (c) a **noun** distinct from `"a cherry-pick"`. The fix's axis is **{clean 1-commit, clean N-commit, conflicted-then-resolved} × {12 acting doors}** — three damage shapes, not one.
