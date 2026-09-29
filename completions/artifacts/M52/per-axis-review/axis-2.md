<!-- M52 per-axis review (re-run) — axis 2 · caller tokens — RECONCILED · driven on the installed `jigc 1.0.0-rc.16` (built from commit e519e4eb), 2026-09-21. -->

<!-- M52 per-axis review (the RE-RUN of M51's instrument) — axis 2 · posture — the OPUS DRIVER · driven on the installed `jigc 1.0.0-rc.16`, 2026-09-21 -->

# M52 per-axis review — AXIS 2 · posture — the OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.16`, asserted before any drive.
This is the **RELEASE** posture: the `Route::mechanical` argv fence is `#[cfg(debug_assertions)]`
and does not exist here, so every refusal below is the shipped one.

**The fixed binary.** M52's seven completion-audit fixes (`ad527fa4` `6c2de03a` `33bd5692`
`68d14cd3` `a83a9e60` `66af090a`, plus the fold-back `e519e4eb`) are all in this build; the two
that touch this axis are **F3** (the survivable frame's state clause is a function of the
rollback's outcome) and **F5** (`RelocateRefusal::ALL`, which gave `relocate`'s clean-control
cell a code it did not have at rc.15). Both are recorded as cells, not as noise.

**Fixtures.** `dev/jigc-rig` throughout (every root from `mktemp -d`; there is no teardown and
none is needed — no `rm -rf` on a variable path appears anywhere in this review). Two forms:

* `dev/jigc-rig <state> --git-state <member>` for the standalone posture cells, and the
  **standalone** `dev/jigc-rig --git-state unborn` for the one state that has no overlay form;
* a wrapper (`driver/axis2-build.sh`, written for this review) that builds `committed-singletons`
  with `--start decided-task "axis intent"`, then **creates a milestone and one sub-task through
  the binary** (both commit, so they must precede the posture), then induces the git state with
  the *same command sequence* `crates/cli/tests/support/git_state.rs` uses — lifted verbatim
  out of `dev/jigc-rig --print-only`. The wrapper exists only because the rig enters its git
  state **last** and the milestone doors need a milestone that was committed before the breach.

Nothing was written into `.jigc/` by hand; every fixture state was reached by running the
command a user runs.

---

## 1 · The door set, read from the code (counts stated)

| registry | file | count I read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs` | **47** leaves |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2003` | **47** rows — the total classification, ⇔-fenced against the clap tree |
| ↳ `ActsOnBehalf::CommitsOnBehalf` | | **10** (`setup` · `migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` · `add-task` · `add-from-spec` · `milestone finalize` · `milestone discard`) |
| ↳ `ActsOnBehalf::MovesOnBehalf` | | **2** (`relocate` · `config set`) |
| ↳ `ActsOnBehalf::Neither` | | **35** |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **10** rows over **9** leaves (`milestone finalize` × 2 commit models) |
| `PostureMember::ALL` | `crates/cli/src/repo.rs:157` | **3** |
| **`InProgress::ALL`** | `crates/cli/src/repo.rs:~232` | **9** — M52's widening from M51's 3 markers |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3275` | **6** (read for the cross with this axis, not as this axis's set) |
| `RelocateRefusal::ALL` | `crates/cli/src/relocate.rs:91` | **10** members / 9 codes — **no posture member**, so posture is refused at the door and at the seam, never by `relocate`'s own refusal set |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:5415` | **4** — `store.not-found` · `store.no-such-leaf` · `store.unknown-type` · `task::FIXED_IDENTITY`. **No `repo.*` member**, which is the declared arm for this axis (see §3.13) |
| `PRE_DISPATCH_FAULTS` | `crates/cli/tests/pre_dispatch_faults.rs:145` | **3** (`cwd-unreadable` · `packs-yaml-malformed` · `pack-resource-missing`); the first is this axis's D2.4 cell |

**Axis 2's door set = the 12 acting rows**, derived (not taken from the design doc):
`COMMITTING_DOORS ⊆ commit-on-behalf` holds by inspection — its 9 leaves are all
`CommitsOnBehalf`; the tenth commit-on-behalf leaf is `setup`, which commits with `--no-verify`
and so carries no rejection identity and no `COMMITTING_DOORS` row.

| # | door | class | argv driven |
|---|---|---|---|
| 1 | `setup` | Commits (`Exempt(HeadUnborn)`) | `jigc setup` |
| 2 | `migrate-corpus` | Commits | `jigc migrate-corpus` |
| 3 | `rename` | Commits | `jigc rename vision --to "Axis Vision"` |
| 4 | `task discard` | Commits | `jigc task discard axis-intent` |
| 5 | `task finalize` | Commits | `jigc task finalize axis-intent` |
| 6 | `milestone create` | Commits | `jigc milestone create "Second milestone"` |
| 7 | `milestone add-task` | Commits | `jigc milestone add-task axis-milestone "second intent"` |
| 8 | `milestone add-from-spec` | Commits | `jigc milestone add-from-spec axis-milestone spec:nope` |
| 9 | `milestone finalize` | Commits | `jigc milestone finalize axis-milestone` |
| 10 | `milestone discard` | Commits | `jigc milestone discard axis-milestone` |
| 11 | `relocate` | Moves | `jigc relocate vision --from docs/vision/` |
| 12 | `config set` | Moves | `jigc config set docs-root docs` |

## 2 · The cell set

M51's seven, **plus the six git states M52's widening added** (`InProgress::ALL` went 3 → 9, and
`dev/jigc-rig --git-state` builds **12** members), plus the cells driving revealed as
discriminating:

`{clean control · HEAD detached · HEAD unborn (two sub-cells) · the nine `InProgress` members,
reached through eleven git states: merge · squash-merge · rebase-merge · rebase-apply · am ·
cherry-pick · sequencer · dangling-sequencer · revert · unmerged-index · bisect · dedicated
worktree (typed) + three negative controls · GIT_DIR redirect (declared out) · the no-override
cross · the probe's subject (nested repo · fake `.git` · outside a repo · an unreadable cwd) ·
**the seam re-probe** (pass arm and refuse arm) · the `Neither` class as control}`

plus a **false-positive / escape hunt** over the states git leaves that no member names
(`--no-commit` forms of merge, cherry-pick and revert; a squash merge concluded by the user's
own commit; a conflicted `cherry-pick -n`).

---

## 3 · The (door, cell) table

Exit / code / route are identical across the doors within a cell unless the table says
otherwise, so repro blocks are one per cell (§5). **~321 `(door, cell)` drives are recorded
below**; §6 states what was not driven and why.

### 3.1 · Clean control (`HEAD → refs/heads/main`, nothing in progress)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| all 10 commit-on-behalf | 0 / 1 / 3 per door's own business | **no `repo.*` code at any door** | per door | `grep 'repo\.'` on stderr → empty | matches contract |
| both movers | 1 / 0 | none | — | same | matches contract |

Driven outcomes, for the record: `setup` 0 · `migrate-corpus` 0 · `rename` 1 (bare, `jigc describe`
route) · `task discard` 1 `task-discard.staged-prose` · `task finalize` 3
`schema-conformance.field-value-conformant` · `milestone create` 0 · `milestone add-task` 0 ·
`milestone add-from-spec` 1 `store.not-found` · `milestone finalize` 3
`milestone.zero-contribution` · `milestone discard` 0 · `relocate` 1 **`relocate.frozen-doctype`**
(M51 recorded this cell as *"1 (bare)"* — **M52 fix F5 is visible here**) · `config set` 0.

### 3.2 · HEAD detached (`git switch --detach HEAD`)

| door | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|
| all 10 commit-on-behalf | 1 | `repo.head-detached` | **Human** | ``route: re-attach HEAD with `git switch <branch>`, then re-run this command`` | matches contract |
| `relocate` (mover) | 1 | `relocate.frozen-doctype` — **proceeds past the posture guard** | Mechanical | no `repo.` token on stderr | matches contract (movers refuse OIP only) |
| `config set` (mover) | 0 | none — acts | — | knob written | matches contract |

### 3.3 · HEAD unborn

**(a) a genuinely unborn repo** (`dev/jigc-rig --git-state unborn`, standalone: `git init`, no
commits, a hand-made `.jigc/config/` marker, **no `jigc setup`**). Each door driven in **its own
fresh rig**, because `jigc setup` *births* HEAD and would otherwise clear the cell for every door
after it.

| door | exit | code | route | verdict |
|---|---|---|---|---|
| `setup` | **0** | none | — | **matches contract — the stated `Exempt(HeadUnborn)` row is real**: commits 0 → 1 |
| `migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` · `add-task` · `add-from-spec` · `milestone finalize` · `milestone discard` | 1 | `repo.head-unborn` | **Human** — ``land the repository's first commit with `git commit`, then re-run this command`` | matches contract |
| `relocate` (mover) | 1 | `store.unknown-type` — proceeds past the guard | — | matches contract |
| `config set` (mover) | 0 | none — acts | — | matches contract |

**(b) an already-installed repo driven unborn** (`git symbolic-ref HEAD refs/heads/nothing-yet`):
`setup` → exit 1 **`setup.dirty-install-path`**, *not* `repo.head-unborn`, naming all six tracked
install paths and `--force` as the single consent. Matches contract: the posture exemption holds
(the family's code never fires) and M51 F2 / M52 Increment 3's pre-write guard fires on its own
terms — relative to an unborn HEAD all six install paths genuinely carry bytes in no commit.

### 3.4–3.12 · The nine `InProgress::ALL` members, over eleven git states × 12 doors

**Every one of these cells was driven at all 12 doors.** The route was then **lifted out of the
emitted bytes and run in the repository that printed it**, and the door re-asked afterwards.

| git state | member answered | exit | the message's noun + predicate | the route's abandoning command | route run → | door after |
|---|---|---|---|---|---|---|
| `merge` | `Merge` | 1 | *a merge is in progress* | `git merge --abort` | exit 0 | exit 0, no code |
| `squash-merge` | `SquashMerge` | 1 | *a squash merge is staged and not committed* | `git reset --merge` | exit 0 | exit 0, no code |
| `rebase-merge` | `Rebase` | 1 | *a rebase is in progress* | `git rebase --abort` | exit 0 | exit 0, no code |
| `rebase-apply` | `Rebase` | 1 | *a rebase is in progress* | `git rebase --abort` | exit 0 | exit 0, no code |
| `am` | **`Am`** | 1 | *a `git am` is in progress* | **`git am --abort`** | exit 0 | exit 0, no code |
| `cherry-pick` | `CherryPick` | 1 | *a cherry-pick is in progress* | `git cherry-pick --abort` | exit 0 | exit 0, no code |
| `sequencer` | `CherryPick` (the marker is live) | 1 | *a cherry-pick is in progress* | `git cherry-pick --abort` | exit 0 | exit 0, no code |
| `dangling-sequencer` | **`Sequencer`** | 1 | *a cherry-pick or revert left a queue of commits in `sequencer/`* | **`git cherry-pick --quit`** | exit 0 | exit 0, no code |
| `revert` | `Revert` | 1 | *a revert is in progress* | `git revert --abort` | exit 0 | exit 0, no code |
| `unmerged-index` | **`UnmergedIndex`** | 1 | *a conflict left unmerged paths in the index* | `git reset --merge` | exit 0 | exit 0, no code |
| `bisect` | `Bisect` | 1 | *a bisect is in progress* | `git bisect reset` | exit 0 | exit 0, no code |

**Verdict: matches contract at every cell** — one code, exactly one `route:` line, a `Human`
route naming a command git accepts in that state, and the state is gone after the route runs.
`rebase-merge` and `bisect` are the two states where git has detached HEAD itself: both now
answer **`repo.operation-in-progress`**, the cause before the symptom (M52 D2.2 — `posture()`
asks the operation *first*). **This is M51's D1 and D2 closed, driven.**

Movers: refuse the **whole** `InProgress` family (all eleven states, both movers), and **only**
that family — driven exempt from `HeadDetached` (§3.2) and from `HeadUnborn` (§3.3).

### 3.13 · The pinned surfaces of a posture refusal

| surface | driven | verdict |
|---|---|---|
| `--format json`, commit door, merge | `{"error": "blocking · repo.operation-in-progress — … \n  route: …"}` on **stderr**, exit 1, **stdout empty** | matches contract — `design/validation.md:662-664` declares all three `repo.*` codes **"neither — un-keyed"**, and `render::ENVELOPE_OWED_CODES` carries no `repo.*` member, so the flattened single-key reject arm is the declared one |
| `--format json`, mover, merge | identical bytes | matches contract |
| `--format agent` / `--format human` | identical text at both | matches contract |
| invocation log | `{"argv":["milestone","create","zz1"],"exit_code":1,"finding_codes":["repo.operation-in-progress"]}`, likewise `task finalize` and `relocate` | matches contract — *a posture refusal is as legible in the invocation log as it is on stderr* |
| route count | exactly one `route:` line per refusal at every door, every cell | matches contract |

### 3.14 · The no-override cross — *the refusal carries no consent flag*

All driven under `merge`:

| door + flag | exit | code | verdict |
|---|---|---|---|
| `setup --force` | 1 | `repo.operation-in-progress` | matches contract |
| `task finalize --carry-staged` | 1 | `repo.operation-in-progress` | matches contract — *"`--carry-staged` stops concluding a merge it never consented to conclude"*, driven |
| `task finalize --dry-run` | 1 | `repo.operation-in-progress` | matches contract (leaf-keyed) |
| `task finalize --approve` | 1 | `repo.operation-in-progress` | matches contract |
| `task discard --force` | 1 | `repo.operation-in-progress` | matches contract |
| `milestone discard --force` | 1 | `repo.operation-in-progress` | matches contract |
| `migrate-corpus --dry-run` | 1 | `repo.operation-in-progress` | matches contract |

### 3.15 · `jigc task validate`'s posture preview (M52 D2.6 — new since M51)

Driven at **all eleven** breached states plus the clean control:

| cell | exit | code | verdict |
|---|---|---|---|
| each of the nine `InProgress` members (11 states) | 1 | `repo.operation-in-progress` | matches contract — *"the finding, its route and its exit code are the committing door's, byte for byte"* |
| detached | 1 | `repo.head-detached` | matches contract |
| clean | 3 | the task's own content findings | matches contract |
| `--format json`, merge | 1 | `{"error": …}`, byte-identical to `task finalize`'s | matches contract |

**Observation, not a defect.** The preview **pre-empts** the task's content findings: under a
breach the user sees only the posture line, never what else `finalize` would gate on. That is
`gate_coverage::Invocation::SeparatelyAtDoor`'s stated design (*"folding it into the sweep would
report a repository state at the task-content severity"*), recorded here because it is the one
cell where *preview* and *refuse* pull in opposite directions.

### 3.16 · Dedicated worktree (typed `DedicatedWorktree`, never sniffed)

| cell | doors driven | exit | code | verdict |
|---|---|---|---|---|
| inside `$REPO/.jigc/worktrees/axis-sub-intent` (git's own `--detach`), clean | `setup` · `migrate-corpus` · `task finalize` · `milestone create` · `milestone discard` · `relocate` · `config set` | per door's own business | **no `repo.head-detached` at any door** | matches contract — the `PostureSubject::adjudicates` exemption is real |
| same worktree + a conflicting merge **inside it** | `setup` · `milestone create` · `config set` | 1 | `repo.operation-in-progress`, `git merge --abort` | matches contract — exempt from `HeadDetached` **and that member only** |
| the **main** checkout while the linked worktree holds the merge | `milestone create` · `config set` | 0 | **no `repo.*`** | matches contract — per-worktree `git_dir` resolution |
| the whole fan-out, clean: `provision` → work → `join` → `milestone finalize` | | **0** | none | matches contract — *"a fan-out worktree … finalizes clean"*: `finalized adaec41 — Finalize milestone axis-milestone (1 sub-task)`, and the `live.verify(SeamAct::Commit)` before `git merge --ff-only` **passes** |
| **negative control (a)** a real linked worktree at `.jigc/worktrees/not-a-subtask` (unregistered id) | `milestone create` · `task finalize` · `setup` | 1 | `repo.head-detached` | matches contract — the **registry leg** discriminates |
| **negative control (b)** a user's own linked worktree elsewhere, detached | `milestone create` · `task finalize` | 1 | `repo.head-detached` | matches contract |
| **negative control (c)** a user's own linked worktree, attached | `milestone create` | 0 | none | matches contract |

### 3.17 · The **seam** re-probe — `SeamSubject::verify` immediately before the act

The cell M51 could not close: a posture opened **after** the door's own verdict. Driven with a
deterministic racer (a `git` shim on `PATH` that fires one real operation once, on a named git
call, then passes everything through to the real git).

| seam | racer fired after | exit | code | state after | verdict |
|---|---|---|---|---|---|
| `relocate` → `SeamAct::Move` (`relocate.rs:238`) | `git ls-files -z` (**after** the door's probe) | 1 | `repo.operation-in-progress` | the committed destination squatter's `git ls-files --stage` entry **byte-identical**; **no `git rm --cached` in the trace at all** | **M51 codex-1 CLOSED** |
| `config set placement-root` → the relocation arm (`relocate.rs:802`) | `git ls-files -z` | 1 | `repo.operation-in-progress`; the knob **not written** (`config get placement-root` still pack-default), nothing moved | **M51 R.1's second half CLOSED** |
| `milestone create` → commit seam (`task.rs:6609`) | `git add -- docs/milestone-records/…` | 1 | `repo.operation-in-progress`; HEAD unmoved; the staged record rolled back (`update-index --force-remove` in the trace); the record file gone from disk | matches contract |
| `task finalize` → commit seam | `git add -- .jigc/config …` (racer: `git bisect start`) | 1 | `repo.operation-in-progress`; HEAD unmoved; the task still open; the user's `git add`-ed work still staged | matches contract |
| **`milestone finalize` → the fan-out `git merge --ff-only` re-probe** (`task.rs:6818`) | `git write-tree` (inside `overlay_docs_commit_and_ff`, after `SeamSubject::live`) | 1 | `repo.operation-in-progress`; **`ff-only` never ran** (`grep -c ff-only trace.log` → **0**); HEAD unmoved; the sub-task worktree still holds `A  ffwork.txt` | **M51's OPEN lead CLOSED, driven at the seam** |

### 3.18 · The probe's subject — *it answers about the repository it was handed*

| cell | door(s) | exit | code | verdict |
|---|---|---|---|---|
| a real **nested** repo, clean, inside a **detached** ancestor | `milestone create` · `config set` | 1 | none | matches contract — no posture inherited |
| the same nested repo, itself detached | `milestone create` | 1 | `repo.head-detached` | matches contract |
| a **fake** `.git` *directory* inside a detached real repo | `task finalize` · `milestone create` · `setup` | 1 / 1 / 0 | `finalize.no-task` / *not set up* / none | matches contract — `posture` resolves the git dir by walk-up, finds no `HEAD`, and asks nothing; the ancestor's log count is unmoved |
| outside any git repository | `task finalize` · `milestone create` · `config set` · `relocate` | 1 | none | matches contract — the door's own ``not inside a git repository (no `.git` found from …)`` answer, **not** pre-empted by the guard |
| **an unreadable cwd** (`cd X && rmdir X`) — M52 D2.4 | `milestone create` (commit) · `milestone create --format json` · `doc list` (`Neither`) | 1 / 1 / 1 | none | matches contract — ``cannot determine the current directory: No such file or directory (os error 2)`` on the `{error}` arm at the *same* exit code, from the guard for an acting door and from its own arm for a `Neither` door |
| **git absent from `PATH`**, detached | `milestone create` | 1 | none — ``could not run `git` (is it on PATH?)`` | matches contract — the "a probe that cannot answer reads as no breach" bound, with no damage behind it: the door fails at its own git call, log unmoved |
| **git absent from `PATH`**, merge | `milestone create` | 1 | `repo.operation-in-progress` | matches contract — the eight filesystem legs need no git |

### 3.19 · The `Neither` class — **all 35 of 35** driven as controls

Under `merge` (and six of them again under `detached`), **none** raised any `repo.*` code:

`start` · `workflow` · `uninstall` · `upgrade` · `ingest` · `migrate` · `unmanage` · `describe` ·
`validate` · `doc create` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc author` · `doc show` · `doc schema` ·
`doc list` · `task list` · `task diff` · `task validate` · `task bind` · `config insert-step` ·
`config replace-step` · `config remove-step` · `config fill` · `config fork` · `config get` ·
`config list` · `milestone list-tasks` · `milestone provision` · `milestone execute` ·
`milestone join`.

**Verdict: matches contract** — a door that neither commits nor moves a committed file is asked
nothing. (Every argv was checked to **parse**: four `config` step verbs first exited 2 on a clap
usage error, which never reaches the post-parse guard, and were re-driven with argv the binary
accepts.)

**Observation on the `DESTROYING_DOORS` × posture cross.** `DESTROYING_DOORS` has **6** members;
four are commit-on-behalf and refuse under every breach, and **two — `milestone provision` and
`uninstall` — are `Neither` and act under every breach**: driven under `merge`, `uninstall --force`
removed jigc's five tracked files and the open task's staged prose at **exit 0** (HEAD and
`MERGE_HEAD` untouched, files left ` D` in the worktree so `git checkout --` recovers them), and
`milestone provision` cut a worktree at exit 0. Neither commits nor `git mv`s, so the
classification is consistent with the rule as `BEHALF_DOORS` states it; recorded because it is
the one place a *destroying* act proceeds under a breached posture.

### 3.20 · GIT_DIR redirect — the **declared-out** row, re-driven

| cell | driven | verdict |
|---|---|---|
| `milestone create` with `GIT_DIR=<repoB>/.git`, run in repo A | exit **0**; the record file written into **A**'s worktree, the base read from **B**'s HEAD, the commit landed in **B** (B's log 1 → 2); A ends `?? docs/milestone-records/redirected-milestone.md`, B ends ` D docs/milestone-records/redirected-milestone.md` | **matches the declared bound exactly** (`repo.rs` module header; M51 VERDICT bound 2) — a **stated row**, re-driven byte-for-byte as M51 recorded it |
| **new datum** — B carries a live `MERGE_HEAD` (or `CHERRY_PICK_HEAD`), A is clean | A's door refuses `repo.operation-in-progress` — but as **`a conflict left unmerged paths in the index`**, routed `git reset --merge`, because the **eight filesystem legs read A's git dir** (no markers) while the **`UnmergedIndex` leg shells to git and therefore reads B**. The route *does* clear B's state (driven: `git reset --merge` in B removed `CHERRY_PICK_HEAD`) | **amplifier on the declared bound, not a new defect** — the subject is *mixed* rather than simply unadjudicated, so the noun is wrong while the route is effective. Recorded as evidence for the reopening condition (a repository-**identity** check), which the module header already carries |

### 3.21 · Precedence against `PRE_DISPATCH_FAULTS`

| cell | door class | exit | answered by | verdict |
|---|---|---|---|---|
| `merge` + a malformed `.jigc/config/packs.yaml` | commit (`milestone create`) | 1 | the **posture** refusal, one JSON document | matches contract — the guard sits at dispatch top, ahead of pack load |
| same | mover (`config set`) | 1 | the posture refusal | matches contract |
| same | `Neither` (`doc list`) | 1 | the **pack** fault (`… is not a valid pack-set list`) | matches contract |
| unreadable cwd + `merge` | commit | 1 | the **cwd** fault | matches contract — `cwd_or_refusal` runs before `posture_refusal_in` |

### 3.22 · The escape hunt — states git leaves that the family does not name

This is the cell set the acceptance design does **not** enumerate, driven because M52's own claim
is *"any operation git can leave un-concluded, not a list of the markers git happens to write."*

| the user's command | what git leaves | member answered | door | verdict |
|---|---|---|---|---|
| `git merge --squash` (clean), before the user's commit | `SQUASH_MSG`, no `MERGE_HEAD` | `SquashMerge` | 1 | matches contract |
| …then the user's own `git commit` | nothing | none | 0 | matches contract — **no false positive**, `SQUASH_MSG` is gone |
| …or `git reset --merge` (the printed route) | nothing | none | 0 | matches contract |
| `git merge --squash` that **conflicts** | `SQUASH_MSG` + `MERGE_MSG`, 3 unmerged | `SquashMerge` | 1 | **DEFECT B** — see §4 |
| `git merge --no-commit --no-ff` (clean) | `MERGE_HEAD` + `MERGE_MSG` | `Merge` | 1 | matches contract |
| `git revert --no-commit` (clean, single **and** multi-commit) | `REVERT_HEAD` + `MERGE_MSG` | `Revert` | 1 | matches contract |
| `git stash pop` that conflicts | no marker, 3 unmerged | `UnmergedIndex` | 1 | matches contract — and the route `git reset --merge` is **safe**: `git stash list` still holds the entry afterwards (driven) |
| `git cherry-pick --no-commit` that **conflicts** | `MERGE_MSG`, 3 unmerged, **no `CHERRY_PICK_HEAD`** | `UnmergedIndex` | 1 | matches contract (the member's stated *"what is left when none of them answered"*) |
| **`git cherry-pick --no-commit` that applies CLEANLY** (single **and** multi-commit) | **`MERGE_MSG` only** — no `CHERRY_PICK_HEAD`, no `sequencer/`, `git ls-files -u` **empty** | **none — all nine members answer no** | **0** | **DEFECT A** — see §4 |

---

## 4 · Defects

### DEFECT A — a clean `git cherry-pick --no-commit` is not a member of the operation family, and `jigc task finalize` concludes it under jigc's own subject at exit 0, destroying the picked commit's authored message

**Contract violated.** Three homes state the same rule:

* `design/finalize.md:31` — *"an operation in progress, the third being **any operation git can
  leave un-concluded** rather than a list of the markers git happens to write"*;
* `design/validation.md:664` (as corrected 2026-09-17, M52 Increment 3) — *"The member set is now
  the operations themselves, `cli::repo::InProgress::ALL`, each with its own detector, noun and
  command"*;
* `crates/cli/src/repo.rs`, `InProgress`'s own doc-comment — *"**The member set is the operations
  git can leave un-concluded, not the markers it writes**."*

Driven, the member set is still **marker-keyed for eight of nine**, and `git cherry-pick -n`
writes **none of the markers** when the pick applies cleanly (git writes `CHERRY_PICK_HEAD` only
where the pick is left mid-conflict) and leaves the index **fully merged**, so the ninth member —
the `git ls-files -u` leg — answers no as well.

This is **M51's D3 on an un-swept cell.** M52 closed D3 for the *conflicted* cherry-pick and for
revert, and `git merge --squash`'s clean `-n`-shaped sibling was made a member *precisely because
jigc swallowed it* (`validation.md`'s `a squash merge` row: *"the entire squashed payload landed
inside jigc's own commit at exit 0 and the merge's authored message was destroyed with
`SQUASH_MSG`"*). The identical damage is reachable today through `git cherry-pick -n`.

```
setup: dev/jigc-rig committed-singletons --start decided-task "axis intent"
$ printf 'a\n'>g1.txt; git add g1.txt; git commit -q -m base
$ git checkout -q -b cpb; printf 'PAYLOAD\n'>g2.txt; git add g2.txt
$ git commit -q -m 'the users cherry-pick target'; P=$(git rev-parse HEAD); git checkout -q main
$ git cherry-pick -n "$P"                                             → exit 0
$ ls -A .git | grep -E 'CHERRY_PICK_HEAD|sequencer|REVERT_HEAD|MERGE_HEAD|SQUASH_MSG|MERGE_MSG'
  MERGE_MSG                                       ← the ONLY thing git left
$ git ls-files -u | wc -l                         → 0
$ cat .git/MERGE_MSG | head -1
  the users cherry-pick target                    ← the picked commit's authored message
$ git diff --cached --name-status                 → A  g2.txt

(author the task's commit doc, then:)
$ jigc task validate axis-intent
  no findings — the task validates clean                                       exit 0
$ jigc task finalize axis-intent
  finalized e5e75f2 — docs(axis): jigc own work
    added g2.txt
    1 file committed                                                            exit 0
$ git log -1 --name-only --format=''            → g2.txt   ← the USER's un-concluded pick
$ ls .git/MERGE_MSG                             → GONE     ← the authored message, destroyed
$ git cherry-pick --continue
  error: no cherry-pick or revert in progress
  fatal: cherry-pick failed                                                     exit 128
```

That last sentence is **byte-for-byte** the evidence `design/validation.md` quotes for why
`Revert` had to become a member (*"after which `git revert --continue` answered 'no cherry-pick or
revert in progress'"*).

**The carryover gate is a partial backstop and only on one ordering.** When the pick predates the
task, plain `task finalize` blocks at exit 3 with `finalize.carried-staged` naming `g2.txt` — but
it names a **path**, never the cherry-pick, and `--carry-staged` (a *staging* consent) then
concludes the pick at exit 0 and destroys `MERGE_MSG`. That is verbatim M51 D3's mechanism, which
M52's own `posture_door_axis::carry_staged_does_not_conclude_a_live_merge` pins for the merge cell
and not for this one. When the pick **post-dates** the task (the mint's staged snapshot predates
it), there is **no gate at all** — the exit-0 path above.

```
$ git cherry-pick -n "$P"; jigc start "carry probe" --workflow decided-task   → task minted
$ jigc task finalize carry-probe
  blocking · finalize.carried-staged — `g2.txt` was already staged before this task existed …  exit 3
$ jigc task finalize carry-probe --carry-staged
  finalized a610965 — docs(axis): jigc own work
    carried-over g2.txt                                                                        exit 0
$ ls .git/MERGE_MSG                             → GONE
```

**The axis, stated.** The escaping cell is exactly *`git cherry-pick --no-commit` that applies
cleanly* (single- or multi-commit). Its two nearest siblings are both caught and were driven so:
a conflicting `cherry-pick -n` (caught by `UnmergedIndex`) and a clean `revert -n` (caught by
`REVERT_HEAD`) — git writes `REVERT_HEAD` unconditionally and `CHERRY_PICK_HEAD` only on conflict,
which is the asymmetry the marker-keyed detector inherits.

### DEFECT B — a conflicted `git merge --squash` is answered by the wrong member, with a predicate that is false of the state

**Contract violated.** `design/validation.md` → *The M52 widening*, the `a conflict` row:
*"**Unmerged paths with no operation marker at all** — a conflicted `git stash pop`, **or a
conflicted `git merge --squash`**"*, and `crates/cli/src/repo.rs`'s `UnmergedIndex` doc-comment,
which says the same. Driven, a conflicted `git merge --squash` leaves `SQUASH_MSG` — a marker — so
`InProgress::SquashMerge` is probed first and answers; the user is told the squash *"is staged and
not committed"* when it is in fact **conflicted**, with three unmerged index entries and nothing
cleanly staged.

```
setup: dev/jigc-rig committed-singletons  (then a conflicting branch pair on c.txt)
$ git merge --squash cb                                                → exit 1
$ ls -A .git | grep -E 'MERGE_HEAD|SQUASH_MSG|MERGE_MSG'
  MERGE_MSG
  SQUASH_MSG                                    ← a marker, so UnmergedIndex is never reached
$ git ls-files -u | wc -l                       → 3
$ jigc milestone create CS1
  blocking · repo.operation-in-progress — a squash merge is staged and not committed —
    the repository is not in a committable state
    route: conclude it, or abandon it with `git reset --merge`, then re-run this command   exit 1
```

Severity is **surface-tier**: the route is effective (`git reset --merge` cleared it, driven, and
the door then exited 0). What is wrong is the claim — a law-1 accuracy failure on the same member
whose whole purpose is naming *which* of nine things the user is in the middle of — and the fact
that a **stated** doc row attributes this cell to a member that never sees it.

### DEFECT C — a posture breach raised at the **commit seam** prints no state-truth clause and no copy-runnable re-run, while every other in-transaction failure at the same door prints both

**Contract cited, both sides.** `design/finalize.md` → *6. Commit* requires, of a refused commit,
*"a **state-truth clause** describing what *this* rejection leaves … the door's **own**
copy-runnable re-run argv … the door's **own** route-exempt error identity"*, and states the
composition is *"one function serving all three render arms — hook rejection, git's own refusal,
and the non-hook `CommitFailed` cell — so no arm can take the constant and skip the outcome."*
The **counter-citation is in the code and I quote it rather than hide it**:
`crates/cli/src/task.rs`'s `already_typed` lists *"a `BlockedFinding` — a routed refusal carrying
its own code and route, **including the `repo.*` posture refusals the commit seam raises**"* as a
deliberate passthrough. So this is a contradiction **between two stated homes**, not a silent
omission — and what the passthrough's justification gets right is the *cause*, which the finding
does carry, and wrong is the *state*, which nothing carries. The reconciler should judge which
home governs.

The contrast is one door, one transaction, one step apart:

```
setup: dev/jigc-rig committed-singletons --start decided-task "axis intent",
       commit doc authored, `git add work.txt`

A · a rejecting pre-commit hook
$ jigc task finalize axis-intent
  `git commit` was rejected (no commit was made):
  the hook says no

  task axis-intent is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/axis-intent/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize axis-intent`.               exit 1
  invocation log →  error_code: "finalize.commit-rejected"

B · a posture raced in at the same point (a `git` shim firing `git bisect start`
    right after `git add -- .jigc/config …`, i.e. after the stage and before the commit)
$ jigc task finalize axis-intent
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in
    a committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command  exit 1
  invocation log →  finding_codes: ["repo.operation-in-progress"], error_code: null
  (state actually on disk, verified: HEAD unmoved, the task still open, `work.txt` still staged,
   jigc's own `.jigc/*` staging rolled back — all true, and none of it said)
```

Driven identically at `milestone finalize`'s fan-out seam, where the transaction had gone
*further* — the dedicated commit object was created and the promoted docs were written and rolled
back — and the whole surface was still those two lines, with no mention that
`milestone:ff-two` is intact or that the sub-task worktrees still hold their staged code (both
verified true afterwards). The contrast at that same seam is on the record from the same rig: a
`--ff-only` refusal over an untracked collision printed *"milestone:race-milestone is intact —
nothing was committed, the merged docs were rolled back, and every provisioned sub-task worktree
still holds its staged code. Resolve the cause above, then re-run
`jigc milestone finalize race-milestone`."*

**The class:** `COMMITTING_DOORS ∩ {the doors whose seam can raise a `BlockedFinding` after the
stage}` — driven at 2 of them (`task finalize`, `milestone finalize`); the other eight share the
one `git_commit_capture` seam, so the class is the axis, not the two repros.

---

## 5 · Repro-block index (setup · argv · observed)

Every block above is self-contained. The three reusable fixtures:

1. **`driver/axis2-build.sh <git-state|none>`** — `dev/jigc-rig committed-singletons --start decided-task
   "axis intent" --binary /Users/maurice/.local/bin/jigc`, two-step eval, then
   `jigc milestone create "Axis milestone"` + `jigc milestone add-task axis-milestone
   "axis sub intent"` through the binary, then the named git state by the exact command sequence
   `dev/jigc-rig --print-only` emits (`gs_seed` / `gs_diverge` / `gs_must_conflict` …). It prints
   the rig env on stdout and the observed markers, HEAD shape and `git ls-files -u` count on
   stderr, so every cell's state is a **measurement**, not an intention.
2. **`driver/axis2-doors.sh`** — the 12 acting doors in one pass, printing `exit`, the first
   `blocking · <code>`, and the first `route:` line per door.
3. **the `git` shim** — `$RIG/shim/git`, a POSIX `sh` script that appends every argv to a trace,
   fires one real operation once on a named call (with `GIT_INDEX_FILE`/`GIT_DIR`/`GIT_WORK_TREE`
   unset so it acts on the main checkout), then `exec`s the real `/usr/bin/git`. **`REAL` is
   hard-coded to the absolute path**: a `command -v git` inside the shim resolves to the shim and
   forks forever (hit once during this review and fixed).

`dev/jigc-rig --git-state unborn` (standalone) is the only fixture not built by (1).

---

## 6 · What I did NOT drive, and why

1. **`GIT_DIR` at 11 of the 12 acting doors.** Driven at `milestone create` only (plus the new
   mixed-subject datum). The bound is a property of the *probe* and of the ambient environment,
   not of a door, and is declared out with a written reopening condition.
2. **`SeamSubject::verify`'s identity-drift and expected-ref-drift legs.** Not driven. They fire
   only when the checkout's git dir or HEAD ref changes between a subject's construction and its
   `verify`; the *posture* leg of that same re-probe is driven at five seams in §3.17, which is
   the leg this axis owns.
3. **`milestone finalize`'s two `COMMITTING_DOORS` rows as distinct posture rows.** Both commit
   models driven at the leaf (`finalize.fan-out.squash` = true and = false), identical answer;
   the guard is leaf-keyed (`Command::leaf()`), so the distinction is axis 4's.
4. **The `task finalize` seam raced by a *merge*.** Attempted and **not reachable**: `git merge`
   refuses to begin over the index jigc has just staged (`error: Your local changes … would be
   overwritten by merge`, measured). The cell was reached with `git bisect start` instead, which
   is a real user command and a real family member; recorded so the failed attempt is not
   mistaken for a passing one.
5. **A genuine concurrent process** rather than a shim/hook racer at any seam. The shim is
   deterministic by design; a real race is not drivable here.
6. **`--ignored`/`--force` interactions at the two `Neither` destroying doors under a breach**
   beyond the one `uninstall --force` drive: that is axis 3's product.
7. **Non-`main` default branch names, submodule worktrees, and `core.worktree` redirects.** Out of
   the acceptance design's cell set; no member keys on any of them.
8. **git versions other than the one installed** (`/usr/bin/git`, Apple Git). Every marker fact
   here is that git's on-disk contract — which is M52's own declared deferral (a), and **DEFECT A
   is a fact about `git cherry-pick -n`'s marker behaviour on this git**; a git that writes
   `CHERRY_PICK_HEAD` on a clean `-n` would close it, and none is known.

---

## 7 · M51 rows: CLOSED / STILL-OPEN

Every §A row of [M51's ledger](../../../completions/artifacts/M51/per-axis-review/README.md) for
this axis, re-driven on `1.0.0-rc.16`.

| M51 row | verdict | the argv + observation that settles it |
|---|---|---|
| **codex-1** — `jigc relocate` mutates the index with `git rm --cached` under a live merge with no re-probe before the mutation | **CLOSED** | Fresh `dev/jigc-rig fresh --pack-from-dev` rig, committed destination squatter, a `git` shim firing one real merge **after** `ls-files -z` (i.e. after the door's own verdict). `jigc relocate adr --from docs/old` → **exit 1**, `blocking · repo.operation-in-progress — a merge is in progress`, route `git merge --abort`. `git ls-files --stage docs/decisions/zsquat.md` **byte-identical** before and after (`100644 8ea9d415… 0`); the shim's full trace contains **no `rm --cached` at all** — the re-probe runs *before* the mutation, not after it |
| **D1** — a commit-on-behalf door never reports `repo.operation-in-progress` under a rebase or a bisect, and the code it reports instead routes at a command git refuses | **CLOSED** | `rebase-merge`: all 10 commit doors → `repo.operation-in-progress — a rebase is in progress`, route ``git rebase --abort``, run verbatim → **exit 0**, state gone, door then exit 0. `bisect`: all 10 → `a bisect is in progress`, route `git bisect reset` → exit 0. `posture()` now asks the operation **first** (M52 D2.2), so `HeadDetached` no longer masks it |
| **D2** — `rebase-apply` is `git am`'s marker too, and jigc names *a rebase* and routes at `git rebase --abort`, which git refuses | **CLOSED** | `am` state (`rebase-apply/applying`, HEAD attached): all 12 doors → ``a `git am` is in progress``, route ``git am --abort``, run verbatim → **exit 0**, `rebase-apply/` gone. The sibling `rebase --apply` state (`rebase-apply/onto`) still answers *a rebase* with `git rebase --abort` → exit 0 — the discriminator is real in both directions |
| **D3** — an un-concluded cherry-pick or revert is not a member of the family, and `jigc task finalize` concludes it under jigc's own subject at exit 0 | **CLOSED for the conflicted cells · STILL-OPEN for the clean `--no-commit` cherry-pick** | *Closed:* `cherry-pick` and `revert` states → all 12 doors refuse at exit 1 with their own nouns and routes, `--carry-staged` refuses first (§3.14), and after a full 12-door sweep the markers are still on disk. *Still open:* **DEFECT A** — `git cherry-pick -n` that applies cleanly leaves `MERGE_MSG` alone, zero unmerged entries and **no marker any member reads**; `jigc task finalize` committed the user's picked payload at exit 0 and `MERGE_MSG` is gone, after which `git cherry-pick --continue` answers *"no cherry-pick or revert in progress"* |
| **D3b** — git's own refusal during a cherry-pick is dressed as a pre-commit hook rejection, with no code and no route | **CLOSED** | The cherry-pick route in is now blocked at the door, so the cell was reached the way M52's own `commit_rejected_axis` reaches it: `git config commit.gpgsign true; git config gpg.program /bin/false`, **no hook anywhere**. `jigc milestone create "Signed milestone"` → exit 1, `` `git commit` failed (no commit was made): `` + git's bytes **verbatim** (`fatal: cannot exec '/bin/false'` …), **zero occurrences of the word "hook"**, then the door's own state clause (*"nothing was committed — the record write and the milestone workbench were both rolled back …"*) and its own copy-runnable re-run |
| **OPEN lead** — the fan-out `git merge --ff-only` re-probe, *"not driven at the seam by either pass"* | **CLOSED** | Full fan-out (`milestone create` → `add-task` → `provision` → work staged in the worktree), a `git` shim firing one real conflicting merge on `git write-tree` inside `overlay_docs_commit_and_ff` — i.e. after `SeamSubject::live` and before the fast-forward. `jigc milestone finalize` → **exit 1**, `repo.operation-in-progress`; `grep -c 'ff-only' trace.log` → **0** (the fast-forward never ran); HEAD unmoved; the sub-task worktree still holds `A  ffwork.txt`. Also driven on its **pass** arm: the same fan-out with no racer finalizes clean at exit 0 |

**Roll-up for this axis, M51 → M52:** 5 CONFIRMED rows + 1 OPEN lead → **5 CLOSED, 1 partially
still-open** (D3's clean-`-n` cell, re-raised as DEFECT A), plus **2 new defects** (B, C) and
**3 recorded observations** (the mixed `GIT_DIR` subject · the two `Neither` destroying doors ·
`task validate`'s pre-emption). The **coverage table gains**: M51 drove 22 of 47 leaves on this
axis; this re-run drives **47 of 47** (all 12 acting doors across every cell, and **all 35**
`Neither` leaves as controls rather than M51's 10), so no leaf loses an axis.

---

## 8 · What this adds over flow-53 arm 2

Arm 2 iterates **`InProgress::ALL` × `BEHALF_DOORS`'s acting members** — a code-side enum crossed
with a code-side registry, over the git states the new `git_state.rs` fixture builder manufactures
— and asserts the partition: every acting door refuses naming the operation, the route's argv is
**run** and git accepts it, `task validate` previews the same refusal, no marker is consumed by
any door, and the `Neither` class stays silent.

This axis adds four things the arm structurally cannot:

1. **The complement of the fixture builder's own state set.** Arm 2's subject is
   `GitState::ALL` — twelve states the builder knows how to make. §3.22 drives the *states the
   builder does not build*: the three `--no-commit` forms, a squash merge concluded by the user's
   own commit, a conflicted `cherry-pick -n`. **DEFECT A lives in exactly that complement** — a
   state no `GitState` member names, which is why an arm iterating `InProgress::ALL` is green over
   it. An enum-keyed arm can only ever prove that the members it has are handled; it cannot
   discover a tenth.
2. **The seam, raced.** Arm 2 drives doors; §3.17 drives the **`SeamSubject::verify` re-probe**
   at five sites with a deterministic `git` shim that opens an operation *after* the door's own
   verdict — the window that made M51's codex-1 a finding and left the fan-out `--ff-only`
   re-probe an open lead. Both are closed here by driving, not by reading the `verify` call.
3. **The surface *around* the finding.** Arm 2 asserts the code, the noun and the route's
   runnability. **DEFECT C** is about what the same refusal does **not** say when it fires inside
   a transaction — no state-truth clause, no copy-runnable re-run, `error_code: null` in the log —
   measured against the *same door's* hook-rejection arm one step away. That comparison spans two
   contracts (the posture family's and the survivable frame's) and belongs to neither arm alone.
4. **The composite precedence.** Posture vs `PRE_DISPATCH_FAULTS` (§3.21), posture vs
   `setup`'s pre-write install guard on an unborn HEAD (§3.3b), posture vs `RelocateRefusal`
   (§3.1's `relocate.frozen-doctype`), posture vs the `DESTROYING_DOORS` cross (§3.19) — four
   registries meeting one guard, asserted once each, where the arms own their own registries
   separately.

---

# Reconciliation ledger — AXIS 2 · posture

**Everything above this line is the Opus driver's table, byte-identical.** Below is the
reconciliation of that table against the Codex source pass (`codex/axis2-codex.md`, prompt beside
it as `codex/axis2-prompt.md`), under the rule in `acceptance-design.md` → *The reconciliation
rule*: **a claim by one that the other cannot reproduce is a lead, not a finding.** Every Codex
claim was entered as a lead and then **driven** — to a repro block, or to a recorded refutation
with the falsifying datum. Every driver defect was **re-driven once by the reconciler** before
being carried.

**Reconciler posture.** Binary asserted first: `/Users/maurice/.local/bin/jigc --version` →
`jigc 1.0.0-rc.16`, exit 0. Rigs built exactly as the driver built them
(`rig=$(dev/jigc-rig <state> [--git-state <member>] --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`,
two-step eval, every root from `mktemp -d`; there is no teardown and none is needed — **no
`rm -rf` on a variable path appears anywhere in this file**). States used: `committed-singletons`
(bare, and with each of `--git-state merge · squash-merge · rebase-merge · rebase-apply · am ·
cherry-pick · sequencer · dangling-sequencer · revert · unmerged-index · bisect · detached`),
`fresh --pack-from-dev`, the standalone `--git-state unborn`, plus two `mktemp -d` directories for
the outside-a-repo and unreadable-cwd cells. **~230 further invocations of the installed binary**,
every one behind a repro block below. Exit codes measured **bare**, never through a pipe.

**Codex's headline is the axis's completeness question, and it returned empty:** *"No grounded
source claim survives this pass. I found no posture-registry omission, misclassified `Neither`
door, forgeable dedicated-worktree exemption, leaked `setup` exemption, or unguarded production
`git commit`/`mv`/`merge`/`rm` seam."* and, under **Claims**, *"None."* So there is **no
Codex-origin CONFIRMED defect on this axis**. What the pass does carry — five M51 row
dispositions, nine consistency claims, one boundary check and two declared bounds — is entered
below as **seventeen leads** and driven. **One of them is REFUTED**, and it is the one that
collides with a driver defect.

---

## A — Codex claims, each driven

### CX-1 — `lead(codex, M51 codex-1 is CLOSED: `displace_foreign_squatter` constructs a live `SeamSubject` and calls `verify(SeamAct::Move)` immediately before `git rm --cached`; relocate.rs:790-818, and the `git mv` independently re-probes at 217-240)` → **CONFIRMED (repro)**

Driven at the **displacement** path specifically — the one codex-1 named — with the `git mv` path
as the control, through the `config set placement-root` mover (which reaches both
`relocate.rs:238` and `relocate.rs:802`), raced by a `git` shim that fires **one real merge once**
on `git ls-files -z`, i.e. **after** the door's own posture probe (`ls-files -u`, visible first in
the trace) has already returned clean.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       a diverging pair on m.txt (seed → theirs → ours), then a COMMITTED foreign squatter:
       mkdir -p newroot; printf '# a foreign squatter\n' > newroot/roadmap.md; git add …; git commit
       $RIG/shim/git: on `ls-files -z`, run the real call, then (unset GIT_INDEX_FILE GIT_DIR
       GIT_WORK_TREE; cd $REPO && git merge theirs), then exit the real call's status.

A · CONTROL, no racer — the displacement path really runs
$ jigc config set placement-root newroot
  relocating the committed doc(s) stranded by the `placement-root` re-point to `newroot` …
    - docs/decisions-log.md → newroot/decisions-log.md
    - docs/roadmap.md       → newroot/roadmap.md
    - newroot/roadmap.md    → .jigc/displaced/roadmap.md (a foreign file at the new home, parked out of the way)
  config: set `placement-root` = `newroot` …                                          exit=0

B · RACED — the merge opens after the door's verdict
$ squatter index BEFORE: 100644 af0b8dafcaae4d0ad9b8f278e302cf24bf953257 0  newroot/roadmap.md
$ PATH="$RIG/shim:$PATH" jigc config set placement-root newroot
  relocating the committed doc(s) stranded by the `placement-root` re-point to `newroot` …
  blocking · repo.operation-in-progress — a merge is in progress — the repository is not in a
    committable state
    route: conclude it with `git merge --continue` once its conflicts are resolved, or abandon it
    with `git merge --abort`, then re-run this command                                 exit=1
$ fired? yes
$ grep -c '^rm \|rm --cached' trace.log   → 0        ← no index mutation at all
$ grep -c '^mv ' trace.log                → 0
$ squatter index AFTER : 100644 af0b8dafcaae4d0ad9b8f278e302cf24bf953257 0  newroot/roadmap.md   ← byte-identical
$ git ls-files | grep '^docs/'  → docs/decisions-log.md  docs/roadmap.md               ← nothing moved
$ jigc config get placement-root → placement-root =   (pack-default)                   ← knob not written
```

The source legs Codex cited were read and agree: `relocate.rs:238` `SeamSubject::live(repo_root).verify(SeamAct::Move)?` immediately precedes `git_run(repo_root, &["mv", …])` at 239, and `relocate.rs:802` the same immediately precedes the `"rm"` at 809.

### CX-2 — `lead(codex, M51 D1 is CLOSED: `posture` checks an operation before detached HEAD, so rebase/bisect detachment no longer masks the actionable operation; repo.rs:760-783, first-applicable-breach at 659-675)` → **CONFIRMED (repro)**

```
setup: for st in rebase-merge bisect; do
         rig=$(dev/jigc-rig committed-singletons --git-state $st --binary …) || exit; eval "$rig"

$ jigc milestone create Mx        # rebase-merge (git has detached HEAD itself)
  blocking · repo.operation-in-progress — a rebase is in progress — …
    route: conclude it with `git rebase --continue` once its conflicts are resolved, or abandon it
    with `git rebase --abort`, then re-run this command                                 exit=1
$ git rebase --abort     → exit 0        $ jigc milestone create Mx → exit 0
$ jigc milestone create Mx        # bisect (likewise detached by git)
  blocking · repo.operation-in-progress — a bisect is in progress — …
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command  exit=1
$ git bisect reset       → exit 0        $ jigc milestone create Mx → exit 0
```

Driven at six doors per state (`milestone create` · `rename` · `migrate-corpus` · `task discard` ·
`relocate` · `config set`) — identical code and identical route at all six. **The cause is
reported before the symptom in both states**: neither answers `repo.head-detached`.

### CX-3 — `lead(codex, M51 D2 is CLOSED: `git am` and rebase are distinct typed states — `Am` detects `rebase-apply/applying`, apply-backend rebase excludes that discriminator; routes `git am --abort` vs `git rebase --abort`; repo.rs:181-190,252-268,312-348)` → **CONFIRMED (repro)**

The whole nine-member family driven at one commit door, with the **abandoning command lifted out
of the emitted bytes and run in the repository that printed it**, then the door re-asked:

```
setup: rig=$(dev/jigc-rig committed-singletons --git-state <state> --binary …) || exit; eval "$rig"
       then: jigc milestone create Mx ; run the printed `abandon it with \`…\`` verbatim ; re-ask

state                noun answered                                             abandon route        route  door after
merge                a merge is in progress                                    git merge --abort      0        0
squash-merge         a squash merge is staged and not committed                git reset --merge      0        0
rebase-merge         a rebase is in progress                                   git rebase --abort     0        0
rebase-apply         a rebase is in progress                                   git rebase --abort     0        0
am                   a `git am` is in progress                                 git am --abort         0        0
cherry-pick          a cherry-pick is in progress                              git cherry-pick --abort 0       0
sequencer            a cherry-pick is in progress                              git cherry-pick --abort 0       0
dangling-sequencer   a cherry-pick or revert left a queue of commits in `sequencer/`  git cherry-pick --quit  0  0
revert               a revert is in progress                                   git revert --abort     0        0
unmerged-index       a conflict left unmerged paths in the index               git reset --merge      0        0
bisect               a bisect is in progress                                   git bisect reset       0        0
                                                       (every row exit=1 at the door)
```

`am` and `rebase-apply` are the discriminator in both directions, exactly as Codex read it, and
neither route is one git refuses. (Incidentally driven and recorded because it is the honest
half: in a **conflicted** state git refuses the *concluding* arm — `git am --continue`,
`git cherry-pick --continue`, `git revert --continue` all exit 128 with conflicts unresolved.
That is git's own contract, not a jigc route defect: the refusal prints **both** arms and the
abandoning arm is the one that always works.)

### CX-4 — `lead(codex, M51 D3 is CLOSED: `InProgress::ALL` contains all nine declared Git states … operation-first posture prevents any committing door from reaching its commit seam under them; repo.rs:173-222,225-244,268-272)` → **REFUTED (datum)**

The **conflicted** cells are closed — driven above (CX-3), and the markers survive a full
door sweep. The disposition **D3 is CLOSED** is not: a `git cherry-pick --no-commit` that applies
**cleanly** is a member of no row of `InProgress::ALL`, so `posture` has nothing to be
operation-first about, the committing door reaches its commit seam, and it commits. The
falsifying datum is the driver's DEFECT A, re-driven here by the reconciler (§B-A below). In one
line: **`git cherry-pick -n <clean pick>` → `jigc task finalize` → exit 0, the user's picked
payload inside jigc's commit, `.git/MERGE_MSG` gone, `git cherry-pick --continue` → *"no
cherry-pick or revert in progress"*, exit 128.**

Codex's *source* reading is correct as far as it goes — `InProgress::ALL` does carry nine
members and `CherryPick` is one of them. The step that does not hold is the leap from *"the enum
has a `CherryPick` member"* to *"an un-concluded cherry-pick is caught"*: the member's detector
is `CHERRY_PICK_HEAD`, and git writes that file **only when the pick is left mid-conflict**. A
completeness question answered against the enum cannot see that, which is exactly the boundary
the reconciliation rule exists to police.

### CX-5 — `lead(codex, M51 D3b is CLOSED: the shared hook-capable commit seam classifies only Git exit 1 as a hook rejection and renders other failures as "`git commit` failed", preserving Git's streams; task.rs:6540-6570,6605-6634)` → **CONFIRMED (repro)**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       git config commit.gpgsign true; git config gpg.program /bin/false     # NO hook edited

$ jigc milestone create "Signed milestone"
  `git commit` failed (no commit was made):
  fatal: cannot exec '/bin/false': No such file or directory
  error: gpg failed to sign the data:
  (no gpg output)
  fatal: failed to write commit object

  nothing was committed — the record write and the milestone workbench were both rolled back, so
  nothing of milestone:signed-milestone survives. Resolve the cause above, then re-run
  `jigc milestone create 'Signed milestone'`.                                           exit=1
$ (occurrences of the word "hook" in that output) → 0
```

Note for §B-C: this arm carries **both** a state-truth clause and a copy-runnable re-run argv —
it is the contrast DEFECT C is measured against, at the same door.

### CX-6 — `lead(codex, `BEHALF_DOORS` is ⇔-fenced as a bijection with every clap leaf; parsed leaf identity independently checked; acting rows owe a parseable argv reaching their own leaf; cli.rs:4711-4750, 4835-4880, 4883-4937)` → **CONFIRMED (source + a driven 47/47 control)**

The fence is a source fact and reads as Codex states it. What the reconciler can drive is its
consequence, and did: **all 47 `VERB_KINDS` leaves driven** under a live merge (the 12 acting doors
in §A/§B above, the 35 `Neither` leaves in CX-7's block), every argv checked to **parse** (exit 2 is
a clap usage error that never reaches the post-parse guard, so five argvs were corrected and
re-driven — see §C). No leaf is unclassified and no leaf answers out of its class, with the single
stated-design exception CX-7 records.

### CX-7 — `lead(codex, the acting set is complete: ten hook-capable `COMMITTING_DOORS` rows plus `setup`; movers are `relocate` and `config set`; `COMMITTING_DOORS ⊆ CommitsOnBehalf` asserted; invocation_log.rs:171-222, cli.rs:4939-4986, 4989-5014)` → **CONFIRMED (repro)**

Driven as its contrapositive — the complement must be silent. All **35** `Neither` leaves under
`--git-state merge`, in one rig with a live task so the `doc` verbs reach their own business:

```
setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "axis intent" \
             --git-state merge --binary …) || exit; eval "$rig"

  start · workflow … --preview · upgrade · ingest · migrate … --as vision · unmanage … ·
  describe · validate · doc create · doc add-item · doc remove-item · doc retitle-item ·
  doc rename … --to "Renamed Vision" · doc set-field · doc set-slot · doc author · doc show ·
  doc schema · doc list · task list · task diff · task bind · config insert-step --workflow … ·
  config replace-step workflow:single-task#compose … · config remove-step … · config fill … ·
  config fork · config get · config list · milestone list-tasks · milestone provision ·
  milestone execute · milestone join                     → 34 of 34, `repo.*` hits = 0
  task validate axis-intent                              → exit 1, repo.operation-in-progress
  uninstall --force   (own rig)                          → exit 0, `repo.*` hits = 0
$ git ls-files (md5) before/after the whole sweep → unchanged
```

`task validate` is the **one** `Neither` leaf that raises a `repo.*` code, and it is the stated
preview design the driver records at §3.15 (`gate_coverage::Invocation::SeparatelyAtDoor`); see §C
for the correction it forces on the driver's §3.19 aggregate sentence. `uninstall --force` acting
at exit 0 under a live merge is confirmed and carried as the driver's §3.19 observation:

```
$ jigc uninstall --force                                                                exit=0
$ ls .git/MERGE_HEAD  → .git/MERGE_HEAD            ← the breach untouched
$ git status --porcelain | head -5
   M .claude/settings.json
   D .claude/skills/jigc/SKILL.md
   D .jigc/.gitignore
   D .jigc/AGENT.md
   D .jigc/config/.gitkeep
```

It neither commits nor `git mv`s, so the `Neither` classification is consistent with the rule as
`BEHALF_DOORS` states it.

### CX-8 — `lead(codex, the production subprocess census for commit/mv/merge/rm/switch/checkout is five sites, each immediately preceded by a posture probe, and no production ordinary `git switch`/`git checkout` site exists)` → **CONFIRMED (source census, re-derived)**

Re-derived independently rather than taken on the pass's word — `grep -rn 'Command::new("git")'`
over `crates/cli/src` (71 sites) plus a subcommand grep, then each hit's `#[cfg(test)]` ancestry
checked by line number:

| production act | site | the probe immediately before it |
|---|---|---|
| `git commit` (shared hook-capable seam) | `task.rs:6611` | `subject.verify(SeamAct::Commit)?` — `task.rs:6609` |
| `git commit --no-verify` (`setup`) | `setup.rs:2439` | `…live_exempt(…).verify(SeamAct::Commit)` — `setup.rs:2418` |
| `git merge --ff-only` (fan-out) | `task.rs:6819` | `live.verify(SeamAct::Commit)?` — `task.rs:6818` |
| `git mv` | `relocate.rs:239` | `SeamSubject::live(repo_root).verify(SeamAct::Move)?` — `relocate.rs:238` |
| `git rm --cached` (displacement) | `relocate.rs:809` | `SeamSubject::live(repo_root).verify(SeamAct::Move)?` — `relocate.rs:802` |

Five acts, five `verify` sites, and the whole production `verify` set is those five — there is no
sixth `.verify(` outside tests. The three apparent extra `mv`/`rm` hits are all inside test
modules and were checked as such: `start.rs:4268` (inside `#[cfg(test)]` from 4133),
`combine.rs:494` (from 299), `orphan.rs:1634` (from 1104). `task.rs:2357`'s `checkout-index -a` is
index materialization, not a HEAD-changing checkout, as Codex says. **No `git switch` and no
ordinary `git checkout` in production.**

### CX-9 — `lead(codex, `SeamSubject::dedicated` requires `&DedicatedWorktree` whose construction and fields are private to task.rs; the door-level classifier additionally requires a real linked-worktree `.git` file, the canonical worktree path and a registered milestone subtask; repo.rs:613-629, 485-523, task.rs:6970-6988)` → **CONFIRMED (repro — the exemption is registry-gated in all three directions)**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       jigc milestone create "Axis milestone"; jigc milestone add-task axis-milestone "axis sub intent"
       jigc milestone provision axis-milestone     → provisioned 1 worktree(s) … exit 0

POSITIVE — inside .jigc/worktrees/axis-sub-intent (git's own --detach; `git symbolic-ref -q HEAD` empty)
  jigc milestone create Zz        exit=0   (no repo.* code)
  jigc config set docs-root docs  exit=0   (no repo.* code)
  jigc validate                   exit=0   (no repo.* code)

NEGATIVE (a) — a REAL linked worktree at .jigc/worktrees/not-a-subtask (id in no registry)
  git worktree add --detach .jigc/worktrees/not-a-subtask HEAD   → exit 0
  jigc milestone create Zz        exit=1   blocking · repo.head-detached
  jigc setup                      exit=1   blocking · repo.head-detached

NEGATIVE (b) — a user's own linked worktree elsewhere, detached
  jigc milestone create Zz        exit=1   blocking · repo.head-detached

NEGATIVE (c) — a user's own linked worktree, ATTACHED
  jigc milestone create Zz        exit=1   blocking · milestone.record-exists   ← its own business, no posture
```

(a) is the sharp one: the **path** is right and the **shape** is right — a real linked worktree
directly under `.jigc/worktrees/` — and it is still refused, so the discriminator really is the
registry and not the location. No forging route was found from outside the crate.

### CX-10 — `lead(codex, `setup`'s exemption is confined to `live_exempt`, documented as having one production caller; every other live seam uses `SeamSubject::live`; repo.rs:597-610)` → **CONFIRMED (repro)**

Each door in **its own fresh unborn rig**, because `setup` births HEAD and would otherwise clear
the cell for whatever ran after it:

```
setup: rig=$(dev/jigc-rig --git-state unborn --binary …) || exit; eval "$rig"   # standalone: no jigc setup

  jigc setup            exit=0  (no code)              commits 0→1   ← the Exempt(HeadUnborn) row is real
  jigc migrate-corpus   exit=1  repo.head-unborn       commits 0→0
  jigc rename vision …  exit=1  repo.head-unborn       commits 0→0
  jigc milestone create exit=1  repo.head-unborn       commits 0→0
      route (all three): land the repository's first commit with `git commit`, then re-run this command
  jigc validate         exit=1  schema-conformance.home-vacated   commits 0→0  ← a `Neither` door, asked nothing
```

The exemption does not leak: it is `setup`'s and only `setup`'s, and it is visible as a **commit
count moving**, not merely as an exit code.

### CX-11 — `lead(codex, `RelocateRefusal::ALL` carries all ten refusal kinds and their finding-code mapping and introduces no alternate move path; relocate.rs:60-118)` → **CONFIRMED (repro)**

Driven as *the refusal set does not contain a posture escape*: under every breached state
`relocate` answers the posture family, never one of its own ten kinds, and under a clean control
it answers its own (`relocate.frozen-doctype`, M52 fix F5, reproduced):

```
  --git-state rebase-merge · bisect · am · cherry-pick · revert · merge · squash-merge ·
  rebase-apply · sequencer · dangling-sequencer · unmerged-index
    jigc relocate adr --from docs/old   → exit 1, blocking · repo.operation-in-progress, one route
  --git-state detached
    jigc relocate adr --from docs/old   → exit 1, relocate.frozen-doctype   ← movers refuse OIP only
  --git-state unborn (standalone)
    jigc relocate adr --from docs/old   → exit 1, store.unknown-type        ← likewise
```

### CX-12 — `lead(codex, the M52 `ROLLBACK_POPULATIONS` `config-root-relocation` population is transaction coverage — prior homes, destinations, file-state rekey, index rollback under `FileCas` — not a posture bypass; rollback.rs:365-389)` → **CONFIRMED (repro)**

The same raced drive as CX-1, read for the *other* property: when the seam refuses, the
transaction leaves **nothing** — not a partial move, not a half-written knob.

```
$ PATH="$RIG/shim:$PATH" jigc config set placement-root newroot      exit=1  repo.operation-in-progress
$ git ls-files | grep -E '^docs/|^newroot/'   → docs/decisions-log.md  docs/roadmap.md   ← BEFORE == AFTER
$ jigc config get placement-root              → placement-root =   (pack-default)        ← never written
$ grep -c '^mv \|^rm ' trace.log              → 0
```

### CX-13 — `lead(codex, `TASK_AREA_FILES`, `DESTROYING_DOORS`/`Disposition`, `ENVELOPE_OWED_CODES`, `STORE_EXIT_FLIPS` incl. `schema-conformance.home-vacated`, `suppressed.door`/`workflow.verb-routed` and the shared fixed-identity predicate add no commit/move seam in this axis)` → **CONFIRMED (repro)**

Two consequences driven. First, `ENVELOPE_OWED_CODES` carrying no `repo.*` member means a posture
refusal takes the **flattened single-key reject arm** rather than the findings envelope — driven
at a commit door and a mover, identical bytes, **stdout empty**:

```
setup: rig=$(dev/jigc-rig committed-singletons --git-state merge --binary …) || exit; eval "$rig"

$ jigc milestone create Mx --format json            # stdout
  (empty)                                                                               exit=1
$ jigc milestone create Mx --format json  2>&1 1>/dev/null      # stderr
  {
    "error": "blocking · repo.operation-in-progress — a merge is in progress — the repository is
    not in a committable state\n  route: conclude it with `git merge --continue` once its
    conflicts are resolved, or abandon it with `git merge --abort`, then re-run this command"
  }                                                                                     exit=1
$ jigc config set docs-root docs --format json 2>&1 1>/dev/null   → byte-identical shape exit=1
```

Second, the `DESTROYING_DOORS` × posture cross: its two `Neither` members act under a breach
(CX-7's `uninstall --force` block; `milestone provision` likewise) and **neither commits nor
moves a committed file**, so no seam is added — which is what the claim says.

### CX-14 — `lead(codex, the every-prior-home store sweep is included in validation and document enumeration (cli.rs:1332-1436, doc.rs:4206-4344) and does not bypass relocation posture)` → **CONFIRMED (repro)**

```
setup: rig=$(dev/jigc-rig committed-singletons --git-state merge --binary …) || exit; eval "$rig"

$ jigc validate    exit=0, no repo.* code      $ jigc doc list   exit=0, no repo.* code
$ git ls-files (md5) before/after the 35-leaf sweep including both → unchanged
```

The sweep reads; it does not move, so there is no act for a posture probe to precede. Its
**blocking** arm was driven separately in the unborn rig (CX-10's `validate` row →
`schema-conformance.home-vacated`, exit 1) and likewise moves nothing.

### CX-15 — `lead(codex, zero schema-hash movement: the manifest `schema-hash` values at M51 commit `577a0099` are byte-identical to the current manifest)` → **CONFIRMED (datum)**

```
$ git diff --stat 577a0099 HEAD -- crates/cli/pack/config/schema-manifest.yaml \
                                   packs/methodology/config/schema-manifest.yaml
  (no output)                                                                            exit=0
$ grep -c "schema-hash" crates/cli/pack/config/schema-manifest.yaml       → 10
$ grep -c "schema-hash" packs/methodology/config/schema-manifest.yaml     → 14
```

Both manifests are byte-identical across M51→M52, not only the six Codex counted. **No boundary
violation.**

### CX-16 — `lead(codex, declared bound: ambient `GIT_DIR` redirection is explicitly out of posture identity enforcement; repo.rs:38-55,644-647)` → **CONFIRMED as a bound (repro)**

Two rigs, A and B, both `committed-singletons`, both at 5 commits:

```
$ cd $A && GIT_DIR=$B/.git jigc milestone create "Redirected milestone"
  record commit: 5a1cead   — the record on its own; anything else you had staged stayed staged
  next: `jigc milestone add-task redirected-milestone "<intent>"`                        exit=0
$ git -C $A rev-list --count HEAD → 5   (unmoved)
$ git -C $B rev-list --count HEAD → 6   (the commit landed HERE)
$ git -C $A status --porcelain → ?? docs/milestone-records/
$ git -C $B status --porcelain →  D docs/milestone-records/redirected-milestone.md
```

The record is written into **A**'s worktree and committed into **B**. This is the declared row,
re-driven byte-for-byte as the driver recorded it, and it is a bound rather than a finding
because the module header carries it with a written reopening condition.

### CX-17 — `lead(codex, declared bound: unanswerable Git probes fail open by recorded design)` → **CONFIRMED as a bound (repro), with the half that does NOT fail open measured too**

```
setup: rig=$(dev/jigc-rig committed-singletons --git-state detached --binary …) || exit; eval "$rig"

$ jigc milestone create Mx                       # git on PATH
  blocking · repo.head-detached — HEAD is detached — …
    route: re-attach HEAD with `git switch <branch>`, then re-run this command           exit=1
$ PATH="$RIG/nogit" jigc milestone create Mx     # git NOT on PATH
  could not run `git` (is it on PATH?): No such file or directory (os error 2)
                                                 ← fails open: no posture code, and the door
                                                   then fails at its own git call anyway

  …and under --git-state merge, git NOT on PATH:
$ PATH="$RIG/nogit" jigc milestone create Mx
  blocking · repo.operation-in-progress — a merge is in progress — …                     exit=1
                                                 ← the filesystem legs need no git at all
```

The bound is real and **narrow**: only the one leg that shells to git (`git ls-files -u`, and the
HEAD-shape questions) fails open; the eight marker legs are filesystem reads and keep answering.

---

## B — driver defects, each re-driven by the reconciler

### DEFECT A — a clean `git cherry-pick --no-commit` is a member of no row of `InProgress::ALL`, and `jigc task finalize` concludes it at exit 0, destroying the picked commit's authored message → **CONFIRMED (re-driven; this is the datum that refutes CX-4)**

```
setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "axis intent" \
             --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc doc set-field commit:axis-intent#header/type --task axis-intent --value docs
       printf 'jigc own work\n' | jigc doc set-slot commit:axis-intent#summary --task axis-intent --from-file -
       jigc task validate axis-intent → no findings — the task validates clean          exit=0

$ printf 'a\n' > g1.txt; git add g1.txt; git commit -q -m base
$ git checkout -q -b cpb; printf 'PAYLOAD\n' > g2.txt; git add g2.txt
$ git commit -q -m 'the users cherry-pick target'; P=$(git rev-parse HEAD); git checkout -q main
$ git cherry-pick -n "$P"                                                                exit=0
$ ls -A .git | grep -E 'CHERRY_PICK_HEAD|sequencer|REVERT_HEAD|MERGE_HEAD|SQUASH_MSG|MERGE_MSG|rebase|BISECT'
  MERGE_MSG                                     ← the ONLY thing git left
$ git ls-files -u | wc -l                       → 0        ← the ninth member's leg answers no too
$ head -1 .git/MERGE_MSG                        → the users cherry-pick target
$ git diff --cached --name-status               → A  g2.txt

$ jigc task validate axis-intent
  no findings — the task validates clean                                                 exit=0
$ jigc task finalize axis-intent
  no findings — the task validates clean
  finalized fe1bf88 — docs: jigc own work
    added g2.txt
    1 file committed                                                                     exit=0
$ git log -1 --name-only --format='%H %s'
  fe1bf886aeaa8d680d789720ba71644588ff68e1 docs: jigc own work
  g2.txt                                        ← the USER's un-concluded pick, inside jigc's commit
$ ls .git/MERGE_MSG                             → ls: .git/MERGE_MSG: No such file or directory
$ git cherry-pick --continue
  error: no cherry-pick or revert in progress
  fatal: cherry-pick failed                                                              exit=128
```

**The second ordering re-driven too** — the pick *pre*-dates the task, so the carryover gate is
reached, and `--carry-staged` (a *staging* consent) concludes the pick anyway:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       the same base/cpb pair, then:  git cherry-pick -n "$P"                            exit=0
$ jigc start "carry probe" --workflow decided-task                                       exit=0
  (author commit:carry-probe's type + summary)
$ jigc task finalize carry-probe
  blocking · finalize.carried-staged — `g2.txt` was already staged before this task existed —
    refusing to let a pre-task staged change silently ride this task's commit
    at: g2.txt
    route: unstage it (`git restore --staged -- g2.txt`) if it is not this task's work, or
    re-run the finalize with `--carry-staged` to declare the carry-over deliberate        exit=3
$ ls .git/MERGE_MSG                             → .git/MERGE_MSG                         ← still there
$ jigc task finalize carry-probe --carry-staged
  finalize — about to commit the index; carrying over (staged before this task existed …):
    carried-over g2.txt
  finalized c243d2d — docs: carry work
    carried-over g2.txt
    1 file committed                                                                     exit=0
$ ls .git/MERGE_MSG                             → No such file or directory
$ git cherry-pick --continue                    → "no cherry-pick or revert in progress" exit=128
```

The gate names a **path**, never the cherry-pick, and the consent it asks for is about staging,
not about concluding someone else's operation. **Status: CONFIRMED, origin driver.** Codex's
D3-CLOSED disposition is **refuted by this repro** (CX-4).

### DEFECT B — a conflicted `git merge --squash` is answered by `SquashMerge` with a predicate that is false of the state, and a locked doc attributes the cell to a member that never sees it → **CONFIRMED (re-driven)**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
       printf 'seed\n' > c.txt; git add c.txt; git commit -q -m seed
       git checkout -q -b cb; printf 'theirs\n' > c.txt; git add …; git commit -q -m theirs
       git checkout -q main; printf 'ours\n' > c.txt;  git add …; git commit -q -m ours

$ git merge --squash cb
  CONFLICT (content): Merge conflict in c.txt
  Squash commit -- not updating HEAD
  Automatic merge failed; fix conflicts and then commit the result.                      exit=1
$ ls -A .git | grep -E 'MERGE_HEAD|SQUASH_MSG|MERGE_MSG'
  MERGE_MSG
  SQUASH_MSG                                    ← a marker, so the UnmergedIndex leg is never reached
$ git ls-files -u | wc -l                       → 3        ← the state is CONFLICTED
$ jigc milestone create CS1
  blocking · repo.operation-in-progress — a squash merge is staged and not committed —
    the repository is not in a committable state
    route: conclude it, or abandon it with `git reset --merge`, then re-run this command exit=1
$ git reset --merge        → exit 0
$ jigc milestone create CS1                                                              exit=0
```

Both contract homes re-read and both say the opposite of the binary:

* `design/validation.md:786`, the `a conflict` row — *"**Unmerged paths with no operation marker
  at all** — a conflicted `git stash pop`, **or a conflicted `git merge --squash`**"*;
* `crates/cli/src/repo.rs:213-214`, `UnmergedIndex`'s own doc-comment — *"**Unmerged paths in the
  index with no operation marker at all** — a conflicted `git stash pop`, or a conflicted
  `git merge --squash`."*

Severity is surface-tier, as the driver graded it: the route is effective. What is wrong is the
claim, on the one member whose whole job is naming *which* of nine things the user is inside.
**Status: CONFIRMED, origin driver.** Codex is **silent** on this cell — it is a claim about what
the binary *says*, which a completeness-of-the-row-set source pass does not ask.

### DEFECT C — a posture breach raised at the **commit seam** prints no state-truth clause and no copy-runnable re-run, while every other in-transaction failure at the same door prints both → **CONFIRMED (re-driven, both arms, including the invocation log)**

One door, one transaction, one step apart. Both arms in the same rig, the racer being a `git` shim
that fires `git bisect start` once, right after the `git add -- .jigc/config …` that stages jigc's
own docs — i.e. **after** the stage, **before** the commit.

```
setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "axis intent" --binary …) || exit
       eval "$rig"; jigc config set invocation-log true                                  exit=0
       (author commit:axis-intent's type + summary); printf 'work\n' > work.txt; git add work.txt

A · a rejecting pre-commit hook
$ jigc task finalize axis-intent
  `git commit` was rejected (no commit was made):
  the hook says no

  task axis-intent is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/axis-intent/docs/`, and anything you had `git add`-ed is still in git's index.
  Fix the hook's complaint, then re-run `jigc task finalize axis-intent`.                exit=1
  .jigc/logs/invocations.jsonl →  "exit_code":1  "finding_codes":[]  "error_code":"finalize.commit-rejected"

B · a posture raced in at the same point
$ PATH="$RIG/shim:$PATH" jigc task finalize axis-intent
  blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a
    committable state
    route: conclude it, or abandon it with `git bisect reset`, then re-run this command  exit=1
  .jigc/logs/invocations.jsonl →
  {"timestamp":"2026-09-21T04:22:35Z","argv":["task","finalize","axis-intent"],"exit_code":1,
   "duration_ms":501,"finding_codes":["repo.operation-in-progress"],"output_bytes":202,
   "binary_version":"1.0.0-rc.16","error_code":null}

  state actually on disk after B, each measured:
    HEAD unmoved                     a3a83b7f… → a3a83b7f…
    the task still open              jigc task list → 1 active task(s): axis-intent
    the user's work still staged     git diff --cached --name-status -- work.txt → A  work.txt
    jigc's own staging rolled back   git diff --cached --name-only | grep '\.jigc' → (none)
    the breach really landed         ls .git/BISECT_LOG → .git/BISECT_LOG
  — all four true, and **none of them said**.
```

The contrast arm is not hypothetical: CX-5's `git commit` **failure** arm at `milestone create`
prints *"nothing was committed — the record write and the milestone workbench were both rolled
back, so nothing of milestone:signed-milestone survives. Resolve the cause above, then re-run
`jigc milestone create 'Signed milestone'`."* — cause **and** state **and** re-run. The posture
arm at the same seam prints cause only.

The driver quotes the counter-citation rather than hiding it (`task.rs`'s `already_typed` lists
`BlockedFinding` — *"including the `repo.*` posture refusals the commit seam raises"* — as a
deliberate passthrough), so this is a contradiction **between two stated homes**, not a silent
omission, and the reconciler's judgment is asked for. **Judgment:** the passthrough clause governs
the **identity** (the finding keeps its own code and route rather than being re-dressed as
`finalize.commit-rejected`, which is right and which arm B confirms), while
`design/finalize.md` → *6. Commit* governs the **state clause and the re-run**, which are not
identity and which the passthrough does not speak to. The two are compatible; what is missing is
the second, and `error_code: null` in the log is the same gap in machine form — a posture refusal
raised **at the seam** is indistinguishable in the log from one raised **at the door**, though only
the first left a half-run transaction behind. **Status: CONFIRMED, origin driver.** Codex is
**silent** on this cell — it is a claim about what a refusal *says*, not about whether a seam is
guarded, and on the guarding question Codex and the driver agree.

**Scope re-measured.** The driver states the class as `COMMITTING_DOORS ∩ {doors whose seam can
raise a `BlockedFinding` after the stage}`, driven at 2. The reconciler's CX-8 census supports
that framing: eight of the ten commit-on-behalf doors share the one `git_commit_capture` seam at
`task.rs:6611`, so the class is the seam, not the two repros.

---

## C — the "driven with no repro block" check

**Rows demoted: 0.** The check was run and is reported in full, including the one place the
driver's prose overstates its own measurement.

1. **The §3 cell tables are aggregates, not un-evidenced rows.** Each carries its fixture by name
   (§5: `driver/axis2-build.sh`, `driver/axis2-doors.sh`, the `$RIG/shim/git` racer — the first
   two are on disk at `scratchpad/axis-review/driver/` and were read) and its argv (§1's 12-door
   table). They are not demoted, because the reconciler **re-drove the cells themselves**: all
   eleven git states at a commit door with the route run verbatim (CX-3), all 35 `Neither` leaves
   (CX-7), the unborn family (CX-10), the dedicated-worktree four-arm cross (CX-9), the
   `--format json` pair (CX-13), the `GIT_DIR` row (CX-16), the git-absent pair (CX-17), the
   no-override cross and the `PRE_DISPATCH_FAULTS` precedence (§D-1, §D-2), the probe-subject
   cells (§D-3) and the fan-out pass arm (§D-4).

2. **One aggregate claim is CORRECTED, not demoted — §3.19.** The sentence *"Under `merge` …
   **none** raised any `repo.*` code"* is **false for exactly one of its 35 members**:
   `task validate` raises `repo.operation-in-progress` at exit 1, which the **same file** records
   two sections earlier (§3.15, *"the finding, its route and its exit code are the committing
   door's, byte for byte"*). The drives are real; the roll-up sentence is wrong. Corrected reading:
   **34 of 35 silent; `task validate` answers by stated preview design.** Driven:

   ```
   setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "axis intent" \
                --git-state merge --binary …) || exit; eval "$rig"
   $ jigc task validate nope           → exit 1, blocking · repo.operation-in-progress
   $ jigc task validate axis-intent    → exit 1, blocking · repo.operation-in-progress
   $ jigc task finalize axis-intent    → exit 1, blocking · repo.operation-in-progress   ← byte-identical
   $ jigc task validate axis-intent --format json 2>&1 1>/dev/null
     {"error": "blocking · repo.operation-in-progress — a merge is in progress — …"}      exit=1
   ```

   This is **not** a product defect — it is §3.15's design, and the preview answering exactly what
   the door answers is the property M52 built. It is a defect in the driver's *summary sentence*,
   and it matters because that sentence is the axis's `Neither`-class completeness claim.

3. **Five argvs in the `Neither` sweep exited 2 (clap usage) on the reconciler's first pass** —
   `config insert-step` · `config replace-step` · `config remove-step` · `config fill` ·
   `doc rename` — and **exit 2 never reaches the post-parse guard**, so those runs prove nothing.
   All five were re-driven with argv the binary accepts (`config insert-step --workflow single-task
   --after compose <file>`, `config replace-step workflow:single-task#compose <file>`,
   `config remove-step workflow:single-task#compose`, `config fill step:compose#nope --from-file
   <file>`, `doc rename vision --to "Renamed Vision"`) → **exit 1 each, `repo.*` hits 0**, their own
   business (`write.identity-change` at `doc rename`). The driver's §3.19 parenthesis says it hit
   and fixed the same four; recorded here so the two passes' handling of it is comparable.

4. **No graded finding rests on an un-evidenced row.** DEFECTS A, B and C each carry a full repro
   block in the driver's file and a second, independent one above. §7's six M51 dispositions each
   carry argv plus the observation that settles them, and all six were re-driven here (CX-1..CX-5
   and §D-4).

---

## D — reconciler drives neither pass graded, recorded so the file is not silent about them

### D-1 — §3.14's no-override cross, re-driven

```
setup: rig=$(dev/jigc-rig committed-singletons --start decided-task "axis intent" \
             --git-state merge --binary …) || exit; eval "$rig"

  setup --force                              exit=1  repo.operation-in-progress
  task finalize axis-intent --carry-staged   exit=1  repo.operation-in-progress
  task finalize axis-intent --dry-run        exit=1  repo.operation-in-progress
  task finalize axis-intent --approve        exit=1  repo.operation-in-progress
  task discard axis-intent --force           exit=1  repo.operation-in-progress
  milestone discard nope --force             exit=1  repo.operation-in-progress
  migrate-corpus --dry-run                   exit=1  repo.operation-in-progress
```

**Matches the driver's row at all seven.** Worth stating plainly next to DEFECT A: *"`--carry-staged`
stops concluding a merge it never consented to conclude"* holds for every **member** of the family
— and DEFECT A is precisely the state that is not a member.

### D-2 — §3.21's `PRE_DISPATCH_FAULTS` precedence, re-driven — and a fixture correction

The driver's row stands, but it needs a **genuinely** malformed `packs.yaml`. The reconciler's
first fixture (`not: a list`) is valid YAML and a tolerated unknown key, and `doc list` exited **0**
over it, which would have read as a contradiction of the driver's row. Re-driven with two real
faults:

```
setup: rig=$(dev/jigc-rig committed-singletons --git-state merge --binary …) || exit; eval "$rig"
       (shipped content is `compose-embedded-methodology: true`)

packs.yaml = a bare scalar (`just-a-scalar`)
  milestone create Mx        exit=1  blocking · repo.operation-in-progress            ← the POSTURE refusal
  config set docs-root docs  exit=1  blocking · repo.operation-in-progress            ← the POSTURE refusal
  doc list                   exit=1  .jigc/config/packs.yaml is not a valid pack-set list:
                                     invalid type: string "just-a-scalar", expected struct PacksFile
packs.yaml = a YAML syntax error (`packs: [unclosed`)
  milestone create Mx        exit=1  blocking · repo.operation-in-progress
  config set docs-root docs  exit=1  blocking · repo.operation-in-progress
  doc list                   exit=1  .jigc/config/packs.yaml is not a valid pack-set list:
                                     did not find expected ',' or ']' at line 2 column 1 …
```

**Confirms §3.21 exactly**: the guard sits at dispatch top, ahead of pack load, for the acting
classes; a `Neither` door answers the pack fault. The correction is to the reconciler's fixture,
not to the driver's row.

### D-3 — §3.18's probe-subject cells, re-driven (with the exit code measured bare)

```
-- a clean NESTED repo inside a detached ancestor (git init inside $REPO/nested)
   jigc milestone create Nn        exit=1, no repo.* code      ← no posture inherited from the ancestor

-- outside any git repository (a fresh mktemp -d)
   jigc task finalize x            exit=1  not inside a git repository (no `.git` found from /…/outside.Q2jX2y)
   jigc milestone create Y         exit=1  (identical)
   jigc config set docs-root docs  exit=1  (identical)
   jigc relocate adr --from docs/old exit=1 (identical)        ← the door's own answer, not pre-empted

-- an unreadable cwd (`GONE=$(mktemp -d …); cd "$GONE"; rmdir "$GONE"`), measured BARE
   jigc milestone create Zz        cannot determine the current directory: No such file or
                                   directory (os error 2)                                exit=1
   jigc doc list                   (identical text)                                      exit=1
```

The last pair is recorded with a method note: measured through `| head` the exit code reads **0**
(head's), which is the `cmd | tail` family this repo's own gate rule exists to stop. Re-measured
bare, both are **1** — which is what the driver's §3.18 row says.

### D-4 — §3.17's fan-out seam, the PASS arm, driven end to end

The refuse arm is CX-1/CX-12's territory; the pass arm is what proves the re-probe is not simply
always-refusing:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary …) || exit; eval "$rig"
$ jigc milestone create "FF milestone"                                                   exit=0
$ jigc milestone add-task ff-milestone "ff sub intent"                                   exit=0
$ jigc milestone provision ff-milestone                                                  exit=0
$ ( cd .jigc/worktrees/ff-sub-intent; printf 'ff\n' > ffwork.txt; git add ffwork.txt
    jigc doc set-field commit:ff-sub-intent#header/type --task ff-sub-intent --value docs
    printf 'ff sub work\n' | jigc doc set-slot commit:ff-sub-intent#summary --task ff-sub-intent --from-file -
    jigc task finalize ff-sub-intent )
    route: `jigc milestone finalize ff-milestone` — the milestone finalize folds every sub-task's
    staged work into the one aggregate commit                                            exit=3
$ jigc milestone join ff-milestone     → joined milestone:ff-milestone — 0 doc(s) merged exit=0
$ jigc milestone finalize ff-milestone
  finalized 511a7ce — Finalize milestone ff-milestone (1 sub-task)
    modified docs/milestone-records/ff-milestone.md
    added ffwork.txt
    2 files committed
    sub-tasks: ff-sub-intent: 1 code file                                                exit=0
```

`live.verify(SeamAct::Commit)` at `task.rs:6818` **passes** and `git merge --ff-only` runs. (The
`join` line *"0 doc(s) merged — no docs staged from: ff-sub-intent"* is the docs-only-fold residue
already chartered at M45, not an axis-2 cell.)

---

## Counts

* **Codex claims entered as leads: 17.** CONFIRMED **16** (each with a repro block above, or — for
  CX-8 and CX-15 — a re-derived source/`git` datum); **REFUTED 1** (CX-4, D3-is-CLOSED, falsified
  by DEFECT A's repro); **OPEN LEADS 0** — every claim was drivable on this binary with the rigs
  available, so none is carried on the source read.
* **Codex-origin CONFIRMED defects: 0.** The pass made no claims; its dispositions and consistency
  statements all held except CX-4.
* **Driver defects carried: 3** (A, B, C), each **re-driven once by the reconciler** with an
  independent repro block. **0 refuted, 0 demoted.**
* **Driver claims corrected: 1** — §3.19's `Neither`-class roll-up, from *"none raised any `repo.*`
  code"* to **34 of 35**, `task validate` excepted by stated preview design (§C-2). The underlying
  drives stand; only the summary sentence changes.
* **Reconciler fixture corrections: 2** — §D-2's malformed-`packs.yaml` shape, and §D-3's
  bare-vs-piped exit-code measurement. Neither changes a driver row.
* **Doors driven by the reconciler: 47 of 47** `VERB_KINDS` leaves.

---

# Doors covered

Every clap leaf that is the door of ≥1 **driven** row in this reconciled file, `VERB_KINDS`
spelling. The driver's **47 of 47** stand; the reconciler independently re-drove **all 47** and
added none beyond the set.

**The 12 acting doors** (`BEHALF_DOORS` ▸ `CommitsOnBehalf` ∪ `MovesOnBehalf`):

`setup` · `migrate-corpus` · `rename` · `task discard` · `task finalize` · `milestone create` ·
`milestone add-task` · `milestone add-from-spec` · `milestone finalize` · `milestone discard` ·
`relocate` · `config set`

**The 35 `Neither` leaves**, driven as controls:

`start` · `workflow` · `uninstall` · `upgrade` · `ingest` · `migrate` · `unmanage` · `describe` ·
`validate` · `doc create` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc author` · `doc show` · `doc schema` ·
`doc list` · `task list` · `task diff` · `task validate` · `task bind` · `config insert-step` ·
`config replace-step` · `config remove-step` · `config fill` · `config fork` · `config get` ·
`config list` · `milestone list-tasks` · `milestone provision` · `milestone execute` ·
`milestone join`

**Total: 47 — `uncovered: none`.**
