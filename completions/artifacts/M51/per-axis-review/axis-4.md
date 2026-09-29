<!-- M51 per-axis review — axis 4 · reconciled · driven on the installed `jigc 1.0.0-rc.15` (commit 577a0099), 2026-09-16 -->

# M51 per-axis review — AXIS 4 · transaction / rollback — RECONCILED

> **This file is the Opus driver's table, unchanged (no demotions — the demotion audit and the
> test it applied are in the ledger, §R0), followed by the Reconciliation ledger against the
> Codex source pass.** Every Codex claim was entered as a lead and **driven** on
> `/Users/maurice/.local/bin/jigc` (`jigc 1.0.0-rc.15`, release posture); every driver defect was
> re-driven once to confirm its repro block. Rigs as the driver used them
> (`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`),
> two-step eval, every root a `mktemp -d`, no teardown, no `rm -rf` on a variable path.

**Binary asserted first, before anything else:**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.15
```

**Result in one line:** **4/4 Codex claims CONFIRMED with driven repro blocks** (none refuted, none
left open), **3/3 driver defect halves re-confirmed verbatim**, **0 demotions**, and the two passes
**converge on one widened class**: the compare-and-swap rollback discipline M51 built for
`CONFIG_LAYER_SPECS ▸ Rewritten` is applied at **one** of **four** worktree-restore populations, and
the other three lose a third party's bytes silently. One command — `jigc milestone finalize` —
CAS-protects two paths and silently clobbers two others in the same run.

---

## Part 1 — the driver table, as filed

# M51 per-axis review — AXIS 4 · transaction / rollback — the OPUS DRIVER

**Binary:** the installed `/Users/maurice/.local/bin/jigc`, asserted **`jigc 1.0.0-rc.15`** before
anything else (release posture — the `Route::mechanical` fence panic of `finding.rs:741` is
`#[cfg(debug_assertions)]` and does not exist here). Every rig was built with
`dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc`, two-step eval, no teardown, no
`rm -rf` on a variable path.

**The table reflects the FIXED binary** — the audit's four commits are in it: routes `6c2391c0`,
orphan territory `da5173a1`, setup guard `ff2bde99`, the LOWs `507c332d`. Two of the four are
directly on this axis and were driven as such (the setup guard is rows **C11/C11b**; the route-quoting
fix is visible in every route span below, none of which dead-ends).

**No Codex source pass was read.** Every row below was driven; nothing is marked driven from a
source read.

---

## 1 · The door set, derived from the code (counts I read)

| registry | file:line | count I read | what I took from it |
|---|---|---|---|
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:130` | **10 rows over 9 clap leaves** (`milestone finalize` carries two commit-model arms) | the axis's spine |
| `ERROR_CODE_REGISTRY` | `crates/cli/src/invocation_log.rs` | **11** (the 10 door identities + 1 non-commit identity) | the `error_code` each row must log |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:1838` | **47 rows** = **10 `CommitsOnBehalf`** · **2 `MovesOnBehalf`** · **35 `Neither`** | `setup` is the 10th commit-on-behalf door and the one `COMMITTING_DOORS` row-less committer (`--no-verify` by recorded design) |
| `IGNORE_DOORS` | `crates/cli/src/gitignore.rs:97` | **4** production `gitignore::ensure` call sites | the §6/A1 half: `setup` · `task finalize` (+ both `milestone finalize` arms) · `milestone create` · `milestone provision` |
| `CONFIG_LAYER_SPECS` | `crates/cli/src/task.rs:4038` | **3 rows** — 1 `Untouched(<stated reason>)` (`.jigc/config`, a directory) + **2 `Rewritten`** (`.jigc/.gitignore`, `.jigc/version`) | the two paths the M51 worktree CAS family covers |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:2625` | **4** | read only to confirm no axis-4 door is missing from it; axis 3 owns it |
| `VERB_KINDS` | `crates/cli/src/cli.rs:1669` | **47** leaves | the coverage spelling |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:948` | **6** (`probe-unreliable`, `oob-rename`, `unmigrated-corpus`, `ahead-corpus`, `orphaned-instance`, `foreign-squatter`) | read; axis 7's, no axis-4 row keys on it |
| `ManifestKind::ALL` | `crates/cli/src/render.rs:1834` | **6** | read; axis 5/7's |
| `SchemaChangeKind` | `crates/engine/src/schema_diff.rs` | not axis 4 | read only to confirm no transaction cell keys on it |

**The axis-4 door set as the acceptance design defines it** — `COMMITTING_DOORS` ∪ `setup` ∪
`milestone join` ∪ `milestone provision` — is therefore **13 rows over 12 clap leaves**:

`task finalize` · `milestone finalize (squash: true)` · `milestone finalize (squash: false)` ·
`rename` · `migrate-corpus` · `milestone create` · `milestone add-task` · `milestone add-from-spec` ·
`milestone discard` · `task discard` · `setup` · `milestone join` · `milestone provision`.

**The cell set** (5 failure points × 2 worktree states = 10) gives **130 (door, cell) pairs**.

---

## 2 · The matrix

Legend — **route kind**: `Human` / `Mechanical` / `Informational` / `none` (an operational-error
frame that carries a re-run argv but no `Route` value is marked `frame-argv`, because the
survivable frame is a rendered `RejectionFrame`, not a `Finding::route`).

### Cell A — hook rejection × worktree unchanged

| # | door | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| A1 | `task finalize` | `jigc task finalize axis-four-probe` | 1 | log `finalize.commit-rejected` | frame-argv | `` `git commit` was rejected (no commit was made):`` + hook bytes verbatim + *"task … is intact — nothing was committed …"* + `git status --short` **identical** to pre-run, HEAD unmoved | matches contract |
| A2 | `task finalize` (with a real config-layer rewrite pending) | same, over a user-edited `.jigc/.gitignore` | 1 | log `finalize.commit-rejected` | frame-argv | `.jigc/.gitignore` **byte-identical to its pre-image** after the run (jigc's `displaced/` append gone), no `.jigc/displaced/` created | matches contract |
| A3 | `milestone finalize (squash: true)` | `jigc milestone finalize axis-four-milestone` | 1 | log `milestone-finalize.commit-rejected` | frame-argv | *"milestone:… is intact … the merged docs were rolled back, and every provisioned sub-task worktree still holds its staged code"*; record `status: active`, HEAD unmoved | matches contract |
| A4 | `milestone finalize (squash: false)` | same, `finalize.fan-out.squash=false` | 1 | log `milestone-finalize.chain-commit-rejected` | frame-argv | *"… HEAD is at its pre-finalize commit …"*; record `status: active`, **not staged** | matches contract (distinct identity per arm) |
| A5 | `rename` | `jigc rename decisions-log:decisions-log --to 'Axis Four Decisions' --slug decisions-log` | 1 | log `rename.commit-rejected` | frame-argv | *"… the rename was rolled back, so `decisions-log:decisions-log` still holds its original identity …"*; doc bytes identical; the printed re-run argv **runs and exits 0** after the hook is removed | matches contract |
| A6 | `migrate-corpus` | `jigc migrate-corpus` (one v0 committed adr) | 1 | log `migrate-corpus.commit-rejected` | frame-argv | *"… the migrated bytes are written and staged …"* — **clause verified true**: `git diff --cached --name-only` = the adr, `schema-version: 2` on disk, HEAD unmoved | matches contract |
| A7 | `milestone create` | `jigc milestone create 'Axis four milestone'` | 1 | log `milestone-create.commit-rejected` | frame-argv | *"… nothing of milestone:… survives"*; record absent, HEAD unmoved; **re-run after fixing the hook exits 0** (the M47 brick class stays closed) | matches contract |
| A8 | `milestone add-task` | `jigc milestone add-task axis-four-milestone 'axis four sub task'` | 1 | log `milestone-add-task.commit-rejected` | frame-argv | *"… the record append and the sub-task mint were both rolled back …"*; record **byte-identical**; re-run exits 0 | matches contract |
| A9 | `milestone add-from-spec` | `jigc milestone add-from-spec rate-limit spec:rate-limit` | 1 | log `milestone-add-from-spec.commit-rejected` | frame-argv | two `note:` lines (0 of 2 landed, unwound) + *"… any sub-task this run already recorded stayed committed …"*; record byte-identical; re-run seeds 2 at exit 0 | matches contract |
| A10 | `milestone discard` | `jigc milestone discard axis-four-milestone --force` | 1 | log `milestone-discard.commit-rejected` | frame-argv | *"… record is still at its pre-discard state and its workbench is untouched"*; record byte-identical, `list-tasks` still names the sub-task | matches contract |
| A11 | `task discard` (milestone sub-task) | `jigc task discard axis-four-sub-task --force` | 1 | log `task-discard.commit-rejected` | frame-argv | *"… the milestone record still names task:… and the task's working area is intact"* — working area verified present | matches contract |
| A12 | `setup` | `jigc setup` with a rejecting `.git/hooks/pre-commit` already installed | **0** | none | none | install lands, `install commit → <sha>`, HEAD moves — the `--no-verify` exemption is real | matches contract (the stated exemption) |
| A13 | `milestone create` (declared bound: a **pending** ignore amend) | `jigc milestone create …` over a user-trimmed `.jigc/.gitignore` | 1 | log `milestone-create.commit-rejected` | frame-argv | the amend **survives** the rejection (`milestones/ worktrees/ logs/ displaced/` appended) and the user's `# my private line` is intact — exactly `design/finalize.md`'s declared bound (*the three non-transaction `ensure` callers are covered by the amend, not by a rollback*) | matches contract — **one observation, §5** |
| A14 | `milestone join` | n/a (this cell) | — | — | — | — | **n/a — commits nothing** (`BEHALF_DOORS` `Neither`); no hook runs. Its own failure point is driven at C12 |
| A15 | `milestone provision` | `jigc milestone provision axis-four-milestone` over a user-trimmed `.jigc/.gitignore` | **0** | none | `Informational` | the **non-committing** `IGNORE_DOORS` member: the amend is a **union** — `worktrees/ logs/ displaced/` appended, every pre-existing line **and** the user's `# my private line` kept — acked on its own surface (`.jigc/.gitignore → appended worktrees/, logs/, displaced/   (… every line already in the file was kept)`); nothing committed | matches contract |
| A16 | `milestone join` | (see C12) | — | — | — | — | driven at C12, its own failure point |

### Cell B — hook rejection × worktree **concurrently edited**

The only deterministic in-transaction editor is the user's own hook, which the design doc itself
names as running inside the interval (`design/finalize.md` → Rollback discipline, the M51 row). The
hook appends to the raced path **through `$REPO/<path>`** (a fan-out arm commits from a dedicated
worktree, so a relative path in the hook reaches the wrong file — established live at B3).

| # | door | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| B1 | `task finalize` — `.jigc/.gitignore` raced | `jigc task finalize axis-four-probe`, hook appends then `exit 1` | 1 | `finalize.rollback-conflict` **blocking**, printed beside the frame; log `error_code=finalize.commit-rejected`, `finding_codes=["finalize.rollback-conflict"]` | `Human` | the racer's bytes stand; the pre-image is parked at `.jigc/displaced/.gitignore.pre-image.<nanos>` and is **byte-identical to the pre-run file**; the route names both copies | matches contract |
| B2 | `task finalize` — `.jigc/version` raced | same, hook rewrites the stamp | 1 | `finalize.rollback-conflict` at `.jigc/version`; log codes as B1 | `Human` | racer's `jigc-version: raced-by-someone-else` stands, parked pre-image holds `jigc-version: 1.0.0-rc.14` | matches contract |
| B3 | `milestone finalize (squash: true)` — `.jigc/.gitignore` raced | `jigc milestone finalize axis-four-milestone` | 1 | `finalize.rollback-conflict`; log `milestone-finalize.commit-rejected` + the conflict code | `Human` | identical shape; the door's own frame is kept and the conflict prints **beside** it | matches contract |
| B4 | `task finalize` — **the promotion destination** raced | same-path migration, hook appends to `VISION.md` then rejects | 1 | log `finalize.commit-rejected`, `finding_codes=[]` | frame-argv | **the racer's bytes are silently destroyed** — the rollback unconditionally `fs::write`s the displaced pre-image; **no `finalize.rollback-conflict`**, nothing parked | **DEFECT 2** |
| B5 | `task finalize` — **the retired original** raced | migration, hook re-creates `direction/plan.md` then rejects | 1 | log `finalize.commit-rejected`, `finding_codes=[]` | frame-argv | the retire rollback is guarded on `!abs.exists()`, so it **silently declines to restore** the foreign original and says nothing; the file now holds only the racer's bytes | **DEFECT 2 (the other direction)** |
| B6 | `task finalize` — `.jigc/.gitignore` raced, `--format json` | `jigc task finalize <id> --format json` | 1 | as B1 | `Human` | **stdout empty**; **stderr carries the `{"error": …}` document *and* the plain-text finding line**, so stderr does not parse as one document | **DEFECT 1** |
| B7 | `milestone finalize (squash: true)` — raced, `--format json` | `jigc milestone finalize m --format json` | 1 | as B3 | `Human` | identical — stdout 0 bytes, stderr `Extra data: line 4 column 1` | **DEFECT 1** (same producer, both doors) |
| B8 | the other **8** `COMMITTING_DOORS` rows + `setup` | n/a | — | — | — | — | **n/a with reason:** none of them rewrites a `CONFIG_LAYER_SPECS ▸ Rewritten` path *inside* a commit transaction. `IGNORE_DOORS` has **4** sites and only `task.rs::try_execute_finalize_plan` is inside one; `milestone create`/`provision`/`setup` amend **outside** any transaction (driven at A13 / E1 / D3). There is no in-transaction pre-image at those doors for a racer to contend with |
| B9 | `milestone join` / `milestone provision` | n/a | — | — | — | — | **n/a — no commit, no hook, no in-transaction pre-image** |

### Cell C — stage failure × worktree unchanged

Induced with a stale `.git/index.lock` (the cause `commit_rejected_axis.rs`'s third sweep names —
*the residue of a killed git*). Every row asserts HEAD unmoved **and** `git ls-files -s`
byte-identical to the pre-run index.

| # | door | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| C1 | `task finalize` | `jigc task finalize axis-four-probe` | **3** | `finalize.stage-failed` **blocking** | `Human` | *"jigc could not stage its own changes — no commit was made and the promotions were rolled back: `git add -- .jigc/config .jigc/.gitignore .jigc/version` failed: …"*; index identical; the pending `.jigc/.gitignore` amend **restored to its pre-image** | matches contract |
| C2 | `rename` | `jigc rename decisions-log:decisions-log --to … --slug decisions-log` | 1 | log `rename.commit-rejected` | frame-argv | git's own bytes, then *"nothing was committed — the rename was rolled back …"* and **"Resolve the cause above"** (the N20 non-hook frame, not the hook sentence); index identical | matches contract |
| C3 | `milestone add-task` | `jigc milestone add-task axis-four-milestone '…'` | 1 | log `milestone-add-task.commit-rejected` | frame-argv | non-hook frame; index identical | matches contract |
| C4 | `migrate-corpus` | `jigc migrate-corpus` | 1 | log `migrate-corpus.commit-rejected` | frame-argv | **the second state clause**: *"the migrated bytes are written to disk, the stage did not complete …"* — verified: `git diff --cached` **empty**, `schema-version: 2` on disk, index identical. The hook-cell clause (*"written and staged"*) is correctly **not** used here | matches contract |
| C5 | `milestone create` | `jigc milestone create 'Locked milestone'` | 1 | log `milestone-create.commit-rejected` | frame-argv | non-hook frame; index identical | matches contract |
| C6 | `task discard` | `jigc task discard axis-four-sub-task --force` | 1 | log `task-discard.commit-rejected` | frame-argv | non-hook frame; index identical | matches contract |
| C7 | `milestone discard` | `jigc milestone discard axis-four-milestone --force` | 1 | log `milestone-discard.commit-rejected` | frame-argv | non-hook frame; index identical | matches contract |
| C8 | `milestone add-from-spec` | `jigc milestone add-from-spec rate-limit spec:rate-limit` | 1 | log `milestone-add-from-spec.commit-rejected` | frame-argv | non-hook frame; index identical | matches contract |
| C9 | `milestone finalize (squash: true)` | `jigc milestone finalize rate-limit` | 1 | log `milestone-finalize.commit-rejected` | frame-argv | non-hook frame; index identical; record `status: active` | matches contract |
| C10 | `milestone finalize (squash: false)` | same, `squash=false` | 1 | log `milestone-finalize.chain-commit-rejected` | frame-argv | `git add -- .jigc/config .jigc/.gitignore docs/milestone-records/<m>.md` failed; **record worktree bytes identical (`RecordFlipGuard`)**, record **not staged**, index identical | matches contract |
| C11 | `setup` | `jigc setup` over a modified `.jigc/AGENT.md` + `.jigc/config/packs.yaml` | 1 | `setup.dirty-install-path` **blocking** | `Human` (names `--force` as the single consent) | *"nothing was installed and no install commit was made — `HEAD` is untouched, so every path listed above still has its pre-run bytes there"* — **verified true**: both edits present, `git status` unchanged. This is the F2 audit fix acting **before the first write** | matches contract |
| C11b | `setup --force` | `jigc setup --force` over the same two modified paths | **0** | `setup.forced-install-path` **advisory** | `Informational` | *"`--force` consented over 2 install path(s) that carried changes in no commit before this run …"* naming each path; the install runs and the authored bytes are **consumed as documented** — `--force` consents, it does not preserve | matches contract |
| C12 | `milestone join` | `jigc milestone join axis-four-milestone` with two worktrees staging `shared.ts` | 1 | `combine.code-collision` **blocking** | `Human` | *"join blocked … 0 doc(s) would merge, nothing committed"*; HEAD unmoved, index identical, both worktrees still hold their staged code | matches contract |
| C13 | `milestone provision` | n/a | — | — | — | — | **n/a — stages nothing** |

### Cell D — stage failure × worktree concurrently edited

| # | rows | verdict |
|---|---|---|
| D1 | all 13 doors | **n/a with reason, stated plainly:** the stage runs **before** the only user-code execution point a fixture controls (the hook), so no deterministic in-transaction racer exists at this failure point. The predicate under test is one shared `ConfigLayerWorktree::restore` at the single `Err` arm — *identical code on the stage-failure and hook-rejection paths* — and it is driven at B1/B2/B3. A background-process racer would test the OS scheduler, not the contract. **Not presented as driven.** |

### Cell E — retire-untrackable refusal **inside the closure** × both worktree states

Only `task finalize` runs the migration sink; the other 12 doors carry no `retirements` and the cell
is vacuous at each, which is recorded as `n/a (no retire phase)` rather than as a pass.

**Fixture honesty:** there is **no binary-only path to this cell.** The guard is defence-in-depth
over a recorded `source-path` that only a tamper or a corrupted workbench can produce — every
caller-typed route into it is refused at the door by M50/M51's path-argument registry (confirmed at
E3). The fixture therefore rewrites `.jigc/tasks/<id>/source-path` after the mint, and that is
declared, not hidden.

| # | door | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| E1 | `task finalize` (migration) | `jigc task finalize migrate-vision-direction-plan-… --approve`, `source-path` → an absolute path in a `mktemp -d` outside the repo | 1 | `finalize.retire-untrackable` **blocking** | `Human` | *"… resolves outside the repository …"*; **the canary outside the repo is intact**; the promote is rolled back (`VISION.md` absent); HEAD unmoved; `git ls-files -s` **identical** | matches contract |
| E2 | `task finalize`, same + a **pending config-layer amend** | same, over a user-trimmed `.jigc/.gitignore` | 1 | `finalize.retire-untrackable` | `Human` | `.jigc/.gitignore` **restored to its pre-image**, `git ls-files -s` identical, `git diff --cached --name-status` **empty** — the M51 Inc 1 regression (*the in-closure refusal left the index staged to delete the whole `.jigc/` layer*) **does not reproduce** | matches contract — the Inc 1 hoist holds |
| E3 | the cell's **caller-typed** route in, driven at its door (`migrate`) | `jigc migrate legacy/untracked.md --as vision` (untracked source) | 1 | `migrate.source-untracked` **blocking** | `Mechanical` — `git add -- legacy/untracked.md`, **run verbatim, exits 0** (the `6c2391c0` route-span fix) | *"… `jigc task finalize --approve` retires the source it migrates, so the file would be deleted from the worktree with nothing to recover it from …"*; **no task minted** (`jigc task list — no active tasks`) | matches contract — the retire cell is closed upstream, which is why E1/E2 need a tampered fixture |
| E4 | the other 12 doors | n/a | — | — | — | — | **n/a — no retire phase** |

### Cell F — empty commit × worktree unchanged

| # | door | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| F1 | `task finalize` | `jigc task finalize empty-probe` over a settled tree | **3** | `finalize.empty-commit` **blocking** | `Mechanical`-shaped (`jigc task finalize empty-probe`) + the discard alternative | *"task validated but produced no diff — nothing to finalize"*; HEAD unmoved; **no** *"was rejected"* anywhere; log `finding_codes=["finalize.empty-commit"]`, `error_code=null` | matches contract |
| F2 | `milestone finalize` | `jigc milestone finalize zero-milestone` | **3** | `milestone.zero-contribution` **blocking** | `Mechanical` (`jigc milestone provision …`) | *"…would land no work…"*; no rejection frame; `error_code=null` | matches contract |
| F3 | `rename` | `jigc rename decisions-log:decisions-log --to 'Decisions Log' --slug decisions-log` | **0** | none | none | *"no-op: … already holds the title … — nothing renamed, nothing committed"*; HEAD unmoved; **no rejection frame, no `*.commit-rejected` in the log** | matches contract (M48 Inc 8) |
| F4 | `migrate-corpus` | `jigc migrate-corpus` on an all-current corpus | **0** | none | `Informational` | *"corpus migration: 0 migrated, 4 already current, 0 blocked"*; no rejection frame | matches contract |
| F5 | `milestone create` | `jigc milestone create 'Zero milestone'` a second time | 1 | `milestone.record-exists` **blocking** | `Human` | the identity guard fires **before** any record write, so the empty record commit is unreachable | matches contract (the stated unreachability, observed) |
| F6 | `milestone add-task` | `jigc milestone add-task zero-milestone 'nothing at all'` a second time | 1 | `milestone.sub-task-collision` **blocking** | `Human` | same shape | matches contract |
| F7 | `milestone add-from-spec` · `milestone discard` · `task discard` · `setup` · `join` · `provision` | **not driven** | — | — | — | — | see the un-driven list, §4 |

### Cell G — promote-after-retire × both worktree states

| # | door | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| G1 | `task finalize`, new destination | `jigc task finalize <migration> --approve`, rejecting hook | 1 | log `finalize.commit-rejected` | frame-argv | **the foreign original is restored byte-identical**, the promoted `VISION.md` is **deleted**, `git status --short` identical to pre-run, HEAD unmoved — full all-or-nothing after both phase-4 writes | matches contract |
| G2 | `task finalize`, **same-path** migration (destination == recorded source) | same, foreign `VISION.md` migrated in place | 1 | log `finalize.commit-rejected` | frame-argv | the destination holds **the pre-finalize foreign bytes, byte-identical** (the `displaced` capture), status identical | matches contract |
| G3 | `task finalize`, destination pre-occupied by an **unrelated untracked** file | same | **3** | `finalize.promote-clobber` **blocking** | `Human` | the pre-promotion clobber guard refuses **before** any write; the occupant's bytes are untouched; foreign intact | matches contract |
| G4 | `task finalize`, destination / retired path **raced inside the transaction** | see B4 / B5 | 1 | — | — | — | **DEFECT 2** |
| G5 | the other 12 doors | n/a | — | — | — | — | **n/a — no promote/retire phase** |

---

## 3 · Counts

- **(door, cell) pairs in the axis:** 13 rows × 10 cells = **130**
- **Driven rows recorded with a repro block:** **46** (A1–A13, A15 = 14 · B1–B7 = 7 · C1–C12 + C11b = 13 · E1–E3 = 3 · F1–F6 = 6 · G1–G3 = 3)
- **Rows recorded `n/a` with a stated reason:** **A14, B8, B9, D1, E4, F7, G5** — covering
  **≈ 83** of the 130 pairs (every one keyed to *why the failure point does not exist at that door*,
  never to "not reached")
- **Defects:** **2** (one of them with two driven directions)
- **Doors of ≥1 driven row:** **all 13 rows / all 12 leaves** of the axis-4 door set, plus
  **`migrate`** (E3, the door that closes the retire cell's caller-typed route). See `doors_covered`.

---

## 4 · What I did NOT drive, and why

Stated plainly, none of it presented as driven:

1. **Cell D in full (stage failure × concurrently edited), all 13 doors.** No deterministic
   in-transaction racer exists before the stage; the hook is the only controllable execution point
   and it runs after. The restore predicate is one shared call at one `Err` arm and is driven at
   B1/B2/B3.
2. **Cell F at `milestone add-from-spec`** (the *"seeded 0 sub-task(s)"* resumable arm),
   **`milestone discard`** and **`task discard`** (the `milestone.terminal` identity guard), and
   **`setup`** (a clean re-run). Time; each is fenced by `commit_rejected_axis.rs`'s second sweep,
   which iterates the same table over the same cell.
3. **Cell B at `migrate-corpus`'s relocation arm.** It is a mover inside a committing door, but it
   moves by `fs::rename` and stages once at the boundary, and it reaches no
   `CONFIG_LAYER_SPECS ▸ Rewritten` path in-transaction — the same reason as B8. Its move/commit
   posture belongs to axis 2.
4. **A genuine concurrent *process* (not a hook) racing a pre-image.** Every race above is the
   hook, which is the instrument `design/finalize.md` itself names. A second process would test the
   scheduler.
5. **`.jigc/version` at a door other than `task finalize`.** The fan-out arms never reach the stamp
   refresh (stated in `ConfigLayerWorktree`'s own doc-comment) and no other door rewrites it.
6. **The `--format json` shape of every cell.** I drove it where the axis is about the envelope
   (B6/B7, C1 at exit 3, C12) and read text elsewhere.

**Fixture honesty, restated:** cells E1/E2 required rewriting `.jigc/tasks/<id>/source-path` — the
only way to reach a guard that exists because that value can be corrupted. Every other fixture was
built by driving the binary; the only files I wrote into `.jigc/` besides that one are
`.jigc/.gitignore` and `.jigc/version`, which are the user-co-owned files the cell is *about*
(a user's private ignore line; a corpus installed by an older binary — simulated, because only one
binary is available here).

---

## 5 · Defects

### DEFECT 1 — a rollback conflict breaks `--format json` stream discipline at the one class where the document is on stderr

**Contract violated.** `design/command-output-contract.md` → *Stream discipline*: *"the JSON-bearing
stream parses as exactly one document, and the other stream carries no JSON at all"*, and the pinned
driver predicate *"parse stdout; if stdout is empty, parse stderr."* For the **reject** class the
JSON-bearing stream **is stderr** (the doc's own table row). `carry_rollback_conflicts`
(`crates/cli/src/task.rs:4328`) prints the finding to stderr *"whatever the format"*, and its
doc-comment justifies that with *"so under `--format json` the document on stdout still parses as
exactly one JSON value"* — a premise that is false for exactly this class, because on a rejection
stdout is empty and the document is on stderr beside the finding text.

**Blast radius:** both doors that reach `ConfigLayerWorktree::restore` — `task finalize` and
`milestone finalize` (driven at both). A driver following the pinned predicate gets a parse failure
instead of the error envelope; and the finding itself reaches the machine surface **nowhere** (the
`{"error": …}` envelope is single-key, so a driver is told neither that a file was left raced nor
where the pre-image was parked — it is in the invocation log only).

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC config set invocation-log true
$JIGC start "axis four probe" --workflow dev-task --format json >/dev/null
mkdir -p src; printf 'export const axis = 4;\n' > src/axis.ts; git add -A
$JIGC doc set-field 'commit:axis-four-probe#header/type' --task axis-four-probe --value chore
printf 'axis four probe change' | $JIGC doc set-slot 'commit:axis-four-probe#summary' --task axis-four-probe --from-file -
printf 'tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\n# my private line\nscratch/\n' > .jigc/.gitignore
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '# raced\n' >> "$REPO/.jigc/.gitignore"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit

$JIGC task finalize axis-four-probe --format json > out.json 2> err.txt ; echo $?
# -> 1
wc -c < out.json          # ->  0        (stdout empty: the reject class)
cat err.txt
# {
#   "error": "`git commit` was rejected (no commit was made):\nNO\n\ntask axis-four-probe is intact …"
# }
# blocking · finalize.rollback-conflict — `.jigc/.gitignore` changed while this finalize was running, …
#   at: .jigc/.gitignore
#   route: nothing was committed and both versions are on disk: …
python3 -c "import json;json.load(open('err.txt'))"
# json.decoder.JSONDecodeError: Extra data: line 4 column 1 (char 337)
```

Same at `jigc milestone finalize <m> --format json` (`Extra data: line 4 column 1 (char 289)`).
**Control that passes:** the *adjudication* class is clean — `finalize.stage-failed` under
`--format json` puts exactly one document on stdout (935 bytes, parses) and leaves stderr empty;
and a blocked `milestone join --format json` puts one document on stdout with the finding text on
stderr, which is the contract.

---

### DEFECT 2 — M51's compare-and-swap rollback discipline is applied to one of the three worktree axes, and the other two lose a third party's bytes silently

**Contract violated.** `design/finalize.md` → *Rollback discipline*, **the promise's one stated
exception**, which is written as a property of *the promise*, not of one family:

> When a path's bytes changed between jigc's write and the rollback, the transaction has no honest
> all-or-nothing move available: restoring overwrites a third party's edit, and doing nothing leaves
> jigc's bytes behind. **It takes neither silently** — it **preserves both versions and says so**,
> with one blocking `finalize.rollback-conflict` naming each. So the invariant a reader can rely on
> is: *before phase 6, nothing jigc wrote survives a failure except where surviving it is the only
> way to avoid destroying someone else's bytes, and every such survival is named.*

And the M51 config-layer row's own rationale generalizes: *"the interval it spans is not jigc's alone:
promotion, retirement, staging and the user's own hooks all run inside it, so an unconditional
restore destroys a concurrent edit — the same loss this family exists to prevent, in the other
direction."*

Driven, `ConfigLayerWorktree::restore` is compare-and-swap; the **promote** and **retire** worktree
axes in `rollback_promotions` (`crates/cli/src/task.rs:4850`) are not:

- **promote** (`:4875-4879`) — unconditional `fs::write` of the displaced pre-image → a third
  party's edit inside the window is **overwritten silently**, no finding, nothing parked;
- **retire** (`:4919-4923`) — guarded on `!abs.exists()` → when the window's editor re-created the
  path, jigc **silently declines to restore** the original and says nothing, so the transaction's
  own "all-or-nothing" is broken in the other direction with no notice.

Both are the same un-swept axis: *the worktree paths a failed transaction restores*. `CONFIG_LAYER_SPECS`
has **2 `Rewritten` rows**; the promote/retire/owner-artifact worktree writes are a **third and
fourth** population of the same class, and only the first got the M51 treatment.

**B4 — the promote axis silently destroys the racer's bytes:**

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf '# Direction\n\nOur aim is a deterministic compiler.\n\n## Principles\n\nStructure to the CLI.\n' > VISION.md
git add -A && git commit -q -m 'foreign VISION.md'
$JIGC migrate VISION.md --as vision                 # mints migrate-vision-vision-cd709ad9fcd8
printf 'title: Vision\nsections:\n  - id: thesis\n    set:\n      thesis: |-\n        <<Deterministic context compiler.>>\n  - id: invariants\n    set:\n      invariants: |-\n        <<Structure to the CLI.>>\n  - id: open-questions\n    set:\n      open-questions: |-\n        <<Which domains earn a pack.>>\n' \
  | $JIGC doc author vision --from-file - --task migrate-vision-vision-cd709ad9fcd8
$JIGC doc set-field 'commit:migrate-vision-vision-cd709ad9fcd8#header/type' --task migrate-vision-vision-cd709ad9fcd8 --value docs
printf 'migrate in place' | $JIGC doc set-slot 'commit:migrate-vision-vision-cd709ad9fcd8#summary' --task migrate-vision-vision-cd709ad9fcd8 --from-file -
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/VISION.md"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit

$JIGC task finalize migrate-vision-vision-cd709ad9fcd8 --approve ; echo $?
# -> 1 · "`git commit` was rejected (no commit was made): NO … task … is intact …"
grep -c 'raced by a concurrent editor' VISION.md      # -> 0      the editor's bytes are GONE
grep -c 'rollback-conflict' <the output>              # -> 0      nothing named it
find .jigc/displaced -type f                          # -> no such directory   nothing parked
```

**B5 — the retire axis silently declines to restore (same fixture, foreign at `direction/plan.md`,
destination new):**

```
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '# recovered by a concurrent editor\n' > "$REPO/direction/plan.md"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit
$JIGC task finalize migrate-vision-direction-plan-0410d8e1466d --approve ; echo $?
# -> 1, the ordinary rejection frame
cat direction/plan.md
# -> "# recovered by a concurrent editor"     the retired original was NOT put back
grep -c 'rollback-conflict' <the output>      # -> 0   nothing said so
```

**Control that passes (the M51 family, same fixture shape):** racing `.jigc/.gitignore` or
`.jigc/version` yields exit 1 **plus** one blocking `finalize.rollback-conflict` per raced path, the
racer's bytes standing, the pre-image parked under `.jigc/displaced/` and named in the route (B1/B2/B3).
So the discipline exists in the binary — it is applied at one axis of the class.

**Bound I will state rather than overclaim:** the window is *inside the transaction*, so the editor
is a hook or a concurrent process, not an ordinary interactive edit. It is not exotic — a formatting
or lint `pre-commit` hook that rewrites files before failing is exactly the shape — and it is the
shape the design doc names by name when it justifies CAS for the sibling family.

---

## 6 · Observation (not graded a defect)

**A13 — a rejected `milestone create` leaves the `.jigc/.gitignore` amend on disk and says nothing
about it.** The survival is the declared bound (`design/finalize.md`: the three non-transaction
`ensure` callers are covered by the amend, not by a rollback), and nothing is lost — the amend is a
union and the user's private line is intact. But `IGNORE_DOORS`' ack for this door is *"the `minted
milestone:` ack itself"*, which only exists on the landed arm, and `task finalize`'s row states its
landed-arm-only ack on the ground that *"a finalize that did not land speaks about the whole
transaction through its rejection frame"* — true there, because there the amend **is** rolled back.
At `milestone create` the amend is not rolled back and the frame says *"nothing of milestone:… survives"*.
No stated rule is contradicted (the registry declares no rejected-run arm), so it is recorded here
rather than as a defect.

**Stated bound, confirmed not a new defect:** `finalize.render-io` prints an absolute host path
(`/private/var/…/repo/.jigc/tasks/<id>/docs/commit:<id>.md`). `crates/engine/src/finalize.rs` is a
**declared** `UNSWEPT_PRODUCERS` member (11 sites, `render_io` named in its reason) in
`crates/cli/tests/repo_relative_paths.rs:746`, so this is a pre-existing declared gap, not an axis-4
finding. Likewise, git's own stderr quoted verbatim inside `finalize.stage-failed` carries an
absolute path, which `design/surface-contract.md` disposes as the *"quoting an invocation"* case.

---

## 7 · What this review ADDS over flow-52 arm 4 (and the shipped axis suites beside it)

Flow-52 **arm 4** iterates *the config-layer CAS pre-image* — a **manufactured shape space**,
`{stage failure, hook rejection} × {unchanged since jigc's post-write image, concurrently edited}`
over the **two named files** `.jigc/.gitignore` and `.jigc/version` — and asserts the restore, the
rollback-conflict on the raced cell, and `gitignore::ensure`'s byte-idempotence across its four
callers. Beside it, `crates/cli/tests/commit_rejected_axis.rs` already sweeps `COMMITTING_DOORS`
three times: × hook rejection, × empty commit, × non-hook failure.

This review adds four things none of those reach:

1. **The class's *other* worktree axes.** Arm 4's subject is the two `Rewritten` config-layer
   files. The promote and retire worktree axes are the same class — *what a failed transaction puts
   back on disk* — and they are governed by **different, un-swept** rules (unconditional write;
   restore-only-if-absent). Driving the concurrently-edited cell **across the axis rather than
   across the two named files** is what surfaced DEFECT 2. This is the *fix-applied-where-its-wave-
   pointed* shape M49 named, one wave later.
2. **The cross of the rollback surface with the output contract.** Arm 4 asserts the finding;
   it does not ask which stream carries it when the door's own document is *also* on stderr. Driving
   the raced cell **under `--format json`** is what surfaced DEFECT 1 — a contract the suite's
   text-mode assertions cannot see, and one whose own code comment states the premise that fails.
3. **The `n/a` half, with reasons derived from the code rather than from a cell list.** The review
   states, per door, *why* a failure point does not exist there — `IGNORE_DOORS` has 4 sites and
   only one is inside a transaction; the fan-out arms never reach the stamp refresh; 12 of 13 doors
   carry no retire phase. Those are the rows a manufactured shape space cannot express, and they are
   what makes the driven set a measurement of the axis rather than of a table.
4. **Live confirmation of the two audit fixes that land on this axis, on the release binary.** The
   F2 pre-write install guard (C11: refusal before the first write, bytes verified present) and the
   M51 Inc 1 capture hoist (E2: the in-closure retire refusal no longer leaves the whole tracked
   `.jigc/` layer staged for deletion — the driven regression does not reproduce). The audit
   verified these against the debug binary before the bump; this is the first drive of them on the
   installed `1.0.0-rc.15`.

---

# Part 2 — Reconciliation ledger

The rule applied (`completions/artifacts/M51/acceptance-design.md` → *The reconciliation rule*):
a claim by one that the other cannot reproduce is a **lead, not a finding**. Every Codex claim below
was entered as `lead(codex, …)` and then **driven**; every driver defect was **re-driven once** by me
before its status was recorded.

## R0 · Demotion audit — 0 demotions, and the test I applied

**The test.** The row schema the acceptance design fixes is
`(door, cell) → {argv driven, exit, code|none, route kind, surface asserted, verdict}`, and *"a row is
**driven** iff its argv ran on the installed `1.0.0-rc.15` and its verdict was recorded with a repro
block."* The operative reading — the one the sibling axis-8 reconciliation applied, where the single
demotion was the row carrying **no argv and no exit** — is that **the row itself is the repro record**:
argv + exit + code + asserted surface. A row carrying none of those did not run.

**Applied to this table: no row marked driven fails that test.** All 46 rows the driver counts
(A1–A13, A15 · B1–B7 · C1–C12 + C11b · E1–E3 · F1–F6 · G1–G3) carry argv, exit, code|none and an
asserted surface. The rows that carry none of it are **not** counted as driven and are labelled `n/a`
with a stated reason (A14, B8, B9, C13, D1, E4, F7, G5) or point at the row that drove them (G4 → B4/B5),
which is the honest form, not a demotion case. §4 states six un-driven areas in the open.

**What I did instead of demoting, because the strict reading (a literal transcript per row) would
demote 43 of 46 and measure formatting rather than the binary.** Literal transcripts are filed for
3 rows (DEFECT 1, B4, B5). I **independently re-drove 6 of the 46** — chosen across four doors and four
cells, without reading the driver's outputs first — and **all six reproduced verbatim, 0 discrepancies**:

| driver row | what I re-drove | result |
|---|---|---|
| A12 | `jigc setup` on `bare` with a rejecting `.git/hooks/pre-commit` | **exit 0**, `jigc setup — adapter installed`, HEAD moved `e203887 → af00639` — the `--no-verify` exemption is real |
| C11 | `jigc setup` over a modified `.jigc/AGENT.md` | **exit 1**, `blocking · setup.dirty-install-path`, *"nothing was installed and no install commit was made — `HEAD` is untouched"*, the user's line still on disk (`grep -c` → 1) |
| C11b | `jigc setup --force`, same state | **exit 0**, `advisory · setup.forced-install-path — `--force` consented over 1 install path(s) …` |
| F1 | `jigc task finalize empty-probe` over a settled tree | **exit 3**, `blocking · finalize.empty-commit — task validated but produced no diff`, route names re-run **and** `jigc task discard empty-probe --force` |
| F3 | `jigc rename decisions-log:decisions-log --to 'Decisions Log' --slug decisions-log` | **exit 0**, `no-op: … already holds the title … — nothing renamed, nothing committed`; `grep -c rejected` → 0 |
| B6 / DEFECT 1 | the filed repro verbatim | reproduced **byte-for-byte**, incl. `JSONDecodeError: Extra data: line 4 column 1 (char 337)` |

So 6 of 46 rows are independently measured by me, 40 rest on the driver's report. Per the standing rule
(*an agent's report is a lead, not a measurement*) those 40 are **driver-reported driven rows**, not rows
I measured — stated here rather than silently inherited. The demotion instrument does not reach them:
they carry argv and exit, and nothing in the six I checked suggests the table is anything but accurate.

**One table artefact, no demotion needed:** row **A16** duplicates A14 (`milestone join`, *"(see C12)"`*)
and claims nothing. It is a redundant row, not a driven one.

---

## R1 · Codex claim 1 — the promote rollback is unconditional and emits no conflict finding → **CONFIRMED (driven)**

`lead(codex, "task and milestone finalize rollback can overwrite a concurrent edit to a promoted
document because promotion restoration is unconditional and has no compare-and-swap conflict finding"
— task.rs:3486-3500 / 4863-4893 vs the config-layer CAS at 4207-4239)`

**This is the driver's DEFECT 2 (promote half, row B4) reached independently from the source side.**
Both passes agree, so it is a finding with **two origins**. I re-drove the driver's repro and then drove
the milestone arm Codex proposed but the driver did not (its *"the analogous `jigc milestone finalize`
cells should behave the same"*), which had been an open half of the claim.

**Re-drive of B4 — `task finalize`, same-path migration, the racer edits the promotion destination:**

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf '# Direction\n\nOur aim is a deterministic compiler.\n\n## Principles\n\nStructure to the CLI.\n' > VISION.md
git add -A && git commit -q -m 'foreign VISION.md'
$JIGC migrate VISION.md --as vision                  # -> task minted: migrate-vision-vision-cd709ad9fcd8
printf 'title: Vision\nsections:\n  - id: thesis\n    set:\n      thesis: |-\n        <<Deterministic context compiler.>>\n  - id: invariants\n    set:\n      invariants: |-\n        <<Structure to the CLI.>>\n  - id: open-questions\n    set:\n      open-questions: |-\n        <<Which domains earn a pack.>>\n' \
  | $JIGC doc author vision --from-file - --task migrate-vision-vision-cd709ad9fcd8
$JIGC doc set-field 'commit:migrate-vision-vision-cd709ad9fcd8#header/type' --task … --value docs
printf 'migrate in place' | $JIGC doc set-slot 'commit:…#summary' --task … --from-file -
cp VISION.md /tmp/ax4-vision-pre.txt
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/VISION.md"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit

$JIGC task finalize migrate-vision-vision-cd709ad9fcd8 --approve ; echo $?
# -> 1 · "`git commit` was rejected (no commit was made): NO … task … is intact …"
grep -c 'raced by a concurrent editor' VISION.md    # -> 0     the racer's bytes are GONE
diff /tmp/ax4-vision-pre.txt VISION.md              # -> (empty)  restored to the pre-finalize foreign bytes
grep -c 'rollback-conflict' <the output>            # -> 0     nothing named it
find .jigc/displaced -type f                        # -> No such file or directory   nothing parked
```

**The milestone arm, driven (new — neither pass had it):** the same class at `jigc milestone finalize`,
on a milestone built entirely by driving (create → add-task → provision → `doc create adr` +
`doc set-slot` ×3 from inside `.jigc/worktrees/area-low`), with the hook racing the **promoted
destination** `docs/decisions/eviction-policy.md`:

```
$JIGC milestone finalize axis-four-milestone ; echo $?
# -> 1 · "milestone:axis-four-milestone is intact — nothing was committed, the merged docs were rolled back …"
ls docs/decisions/eviction-policy.md      # -> No such file or directory
grep -rc 'raced by a concurrent editor' docs/   # -> nothing: the racer's appended bytes went with the file
grep -c 'rollback-conflict' <the output>  # -> 0
find .jigc/displaced -type f              # -> No such file or directory
```

For a **new** destination the rollback deletes the promoted copy (`design/finalize.md` row 6: *"the
promoted copy deleted for a new one"*), and it deletes it **with whatever a third party wrote into it
inside the window**. That is the promise's stated exception firing unnamed:
*"nothing jigc wrote survives a failure except where surviving it is the only way to avoid destroying
someone else's bytes, **and every such survival is named**"*.

**Status: CONFIRMED · origin driver + codex** (driver B4/G4; codex claim 1). The milestone half is
**new to this reconciliation**.

---

## R2 · Codex claim 2 — `RecordPreImage` rollback restores the milestone record unconditionally → **CONFIRMED (driven)**

`lead(codex, "all record-only milestone commit doors can destroy a concurrent edit to the milestone
record on stage/commit failure because RecordPreImage rollback restores worktree bytes unconditionally"
— milestone.rs:774-803, 816-825, 865-887; callers at 684, 1369, 1782, 3261)`

**Neither the driver's table nor its DEFECT 2 reaches this population.** The driver's B8 row rules the
other committing doors `n/a` on the ground that *"none of them rewrites a `CONFIG_LAYER_SPECS ▸
Rewritten` path inside a commit transaction"* — true, and exactly why the record's **own** pre-image
family went unexamined: it is a different capture, with the same un-swept rule. This is the **third**
population of DEFECT 2's class, found from the source side and driven here.

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC config set invocation-log true
$JIGC milestone create 'Axis four milestone'
#   record: docs/milestone-records/axis-four-milestone.md   record commit: 6815f02
cp docs/milestone-records/axis-four-milestone.md /tmp/ax4-rec-pre.txt
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/docs/milestone-records/axis-four-milestone.md"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit

$JIGC milestone add-task axis-four-milestone 'axis four sub task' ; echo $?
# -> 1
# `git commit` was rejected (no commit was made):
# NO
#
# nothing was committed — the record append and the sub-task mint were both rolled back, so
# milestone:axis-four-milestone is unchanged. Fix the hook's complaint, then re-run
# `jigc milestone add-task axis-four-milestone 'axis four sub task'`.

grep -c 'raced by a concurrent editor' docs/milestone-records/axis-four-milestone.md   # -> 0   GONE
diff /tmp/ax4-rec-pre.txt docs/milestone-records/axis-four-milestone.md                # -> (empty)
grep -c 'rollback-conflict' <the output>                                               # -> 0
find .jigc/displaced -type f            # -> No such file or directory
git log --oneline -1                    # -> 6815f02  (HEAD unmoved — the rest of the frame is true)
```

The frame's state-truth clause (*"the record append and the sub-task mint were both rolled back, so
milestone:… is unchanged"*) is **true about jigc's own write and false about the file**: the file is not
what it was when the hook ran, and the sentence is the only thing an operator gets. The driver's rows
A8 / C3 assert *"record byte-identical"* on the **unchanged**-worktree cell — the correct result there
— and it is the same unconditional `fs::write` that produces both.

**Status: CONFIRMED · origin codex.** Driven at `milestone add-task`; the three sibling callers Codex
names (`create`, `add-from-spec`, `discard`, plus the `task discard` record settlement) share the one
`rollback_record_pre_image` and were **not** driven — recorded as the claim's un-driven breadth, not as
a driven fact.

---

## R3 · Codex claim 3 — `RecordFlipGuard`'s `Drop` is a second unconditional restore → **CONFIRMED (driven)**

`lead(codex, "milestone-finalize's record flip has a second unconditional restore path that can
overwrite a concurrent edit even when the shared finalize executor protects .jigc/.gitignore and
.jigc/version" — milestone.rs:4430-4454 armed at 4495-4513; the guard's own doc-comment limits it to
the worktree axis)`

**Driven, on a milestone built by driving** (create → add-task → provision → author an `adr` from inside
the sub-task worktree), with the hook racing the **committed milestone record** during
`jigc milestone finalize`:

```
# … fixture as in R1's milestone arm …
cp docs/milestone-records/axis-four-milestone.md /tmp/ax4-rec3-pre.txt     # status: active
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/docs/milestone-records/axis-four-milestone.md"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit

$JIGC milestone finalize axis-four-milestone ; echo $?
# -> 1 · "milestone:axis-four-milestone is intact — nothing was committed, the merged docs were
#         rolled back, and every provisioned sub-task worktree still holds its staged code."
grep -c 'raced by a concurrent editor' docs/milestone-records/axis-four-milestone.md   # -> 0   GONE
diff /tmp/ax4-rec3-pre.txt docs/milestone-records/axis-four-milestone.md               # -> (empty)
grep -c 'rollback-conflict' <the output>   # -> 0
find .jigc/displaced -type f               # -> No such file or directory
```

**The sharpest single-command statement on this axis.** In the driver's B3 the *same verb* CAS-protects
`.jigc/.gitignore`, parks the pre-image and prints a blocking `finalize.rollback-conflict`; in this run
the same verb silently overwrites the milestone record (this claim) **and** silently deletes the
promoted destination (R1's milestone arm). One command, four worktree-restore populations, one of them
swept.

**Status: CONFIRMED · origin codex.** Driven in the default (`squash: true`) commit model; the
`squash: false` arm shares `flip_record_for_finalize`/`RecordFlipGuard` and was **not** driven —
recorded as un-driven breadth.

---

## R4 · Codex claim 4 — `milestone provision` amends `.jigc/.gitignore` then fails without acknowledging it → **CONFIRMED (driven)**

`lead(codex, "milestone provision can amend .jigc/.gitignore and then fail without acknowledging that
surviving write" — run_provision calls gitignore::ensure at milestone.rs:2158-2197 before the schema
load, the cache reseed, the unknown-milestone check, the stale-base guard, the task-list read and the
worktree adds; the ack is built only at 2197-2205)`

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf 'tasks/\nindex/\nstate/\n# my private line\n' > .jigc/.gitignore     # a trimmed, user-edited file
cp .jigc/.gitignore /tmp/ax4-ign-pre.txt

$JIGC milestone provision nonexistent-milestone ; echo $?
# -> 1
# blocking · milestone.unknown — milestone `nonexistent-milestone` does not exist
#   at: milestone:nonexistent-milestone
#   route: create it first with `jigc milestone create "<title>"`

cat .jigc/.gitignore
# tasks/ index/ state/ "# my private line"  ← kept …
# milestones/ worktrees/ logs/ displaced/   ← … and four lines appended by the run that failed
diff /tmp/ax4-ign-pre.txt .jigc/.gitignore   # -> 4a5,8 (AMENDED — the write survived the failure)
git status --short                           # ->  M .jigc/.gitignore
```

**Why this is a contradiction and not a bound.** `gitignore::IGNORE_DOORS`' own row for this door reads:
*"the `provisioned N worktree(s)` ack itself (again the envelope's `text`) — **the door that never
commits, so what it writes into the file lives in the worktree alone and this line is the only notice of
it**"*, and `run_provision`'s sited comment repeats it: *"This door **never commits**, so an entry
appended here lives in the worktree alone — which makes naming it the only channel there is."* The
`ensure` call is placed **before five failure points**, and on every one of them the declared *only*
notice never fires. Nothing is lost (the amend is a union — the private line is intact), so this is a
**law-1 disclosure** finding, not data loss.

**It re-grades the driver's §6 observation rather than duplicating it.** The driver recorded A13
(`milestone create`, rejected commit, the amend survives) as *"no stated rule is contradicted (the
registry declares no rejected-run arm)"*. Read against the registry text above, the `create` row makes
the same claim in the same words (*"the `minted milestone:` ack itself"* is the door's disclosure), and
the `provision` row goes further with *"the only notice of it"* — so at **both** doors the registry
names a channel that a failing run never opens. The two are one finding of the same shape; A13's
observation is its `create` instance.

**Status: CONFIRMED · origin codex** (driver A13 is the sibling instance, re-graded).

---

## R5 · Driver DEFECT 1 — `--format json` stream discipline at the rollback-conflict class → **STANDS (re-driven)**

**The source pass is silent on it, and does not contradict it.** Its consistent-reads bullet —
*"Config worktree restoration correctly uses compare-and-swap and emits `finalize.rollback-conflict`"* —
is about the **restore**, not the **stream**, and is itself correct (driven: the racer's bytes stand,
the pre-image is parked, the finding prints). Nothing in the Codex pass asserts that the JSON-bearing
stream parses; a silent pass is not a refutation.

**Re-driven verbatim from the driver's own repro, on `1.0.0-rc.15`:**

```
$JIGC task finalize axis-four-probe --format json > out.json 2> err.txt ; echo $?
# -> 1
wc -c < out.json          # -> 0    (stdout empty: this is the reject class, the document is on stderr)
cat err.txt
# {
#   "error": "`git commit` was rejected (no commit was made):\nNO\n\ntask axis-four-probe is intact — …"
# }
# blocking · finalize.rollback-conflict — `.jigc/.gitignore` changed while this finalize was running, …
#   at: .jigc/.gitignore
#   route: nothing was committed and both versions are on disk: … `.jigc/displaced/.gitignore.pre-image.1789569554667582000` …
python3 -c "import json;json.load(open('err.txt'))"
# json.decoder.JSONDecodeError: Extra data: line 4 column 1 (char 337)
```

Byte-for-byte the filed result, down to the decoder's char offset. The producer's own doc-comment states
the premise that fails here (*"so under `--format json` the document on stdout still parses as exactly
one JSON value"* — on this class stdout is empty and the document is on **stderr**, beside the finding
text), and the finding reaches no machine surface at all: the `{"error": …}` envelope is single-key, so
a driver is told neither that a file was left raced nor where the pre-image was parked.

**Status: CONFIRMED · origin driver** (source pass silent, not contradicting).

---

## R6 · Driver DEFECT 2, retire half (row B5) — → **STANDS (re-driven); the source pass's clean read is REFUTED with the driven datum**

**The source pass graded this same mechanism as consistent:** *"Retirement rollback does not overwrite a
concurrently recreated retirement target: it restores captured bytes only while the path remains
absent."* The **mechanism** description is right and both passes agree on it. The **grading** is what the
drive refutes: the guard's *silence* is the defect, not its restraint.

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
mkdir -p direction
printf '# Direction\n\nOur aim is a deterministic compiler.\n\n## Principles\n\nStructure to the CLI.\n' > direction/plan.md
git add -A && git commit -q -m 'foreign direction/plan.md'
cp direction/plan.md /tmp/ax4-plan-pre.txt
$JIGC migrate direction/plan.md --as vision      # -> task minted: migrate-vision-direction-plan-0410d8e1466d
#   … doc author vision … ; set-field commit#header/type=docs ; set-slot commit#summary …
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '# recovered by a concurrent editor\n' > "$REPO/direction/plan.md"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit

$JIGC task finalize migrate-vision-direction-plan-0410d8e1466d --approve ; echo $?
# -> 1, the ordinary rejection frame: "task … is intact — nothing was committed …"
cat direction/plan.md
# -> "# recovered by a concurrent editor"        the retired original was NOT put back
diff /tmp/ax4-plan-pre.txt direction/plan.md     # -> differs
grep -c 'rollback-conflict' <the output>         # -> 0      nothing said so
ls VISION.md                                     # -> No such file or directory  (the promote DID roll back)
git status --short                               # ->  M direction/plan.md
git show HEAD:direction/plan.md | head -3        # -> the original bytes are still in the object DB
```

**The falsifying datum for the clean read** is the last four lines together: the transaction ends with
one of its two phase-4 writes undone and the other not, the worktree holding a third party's bytes at
the path jigc deleted, and **no finding, no route, no parked copy** — the exact condition
`design/finalize.md` → *The promise's one stated exception* says must be named (*"It takes neither
silently — it **preserves both versions and says so**"*). A reader of the frame is told the task is
intact; nothing tells them the retired original is not where it was.

**Bound I state rather than overclaim, and it is the honest half the source read was reaching for:**
because `jigc migrate` refuses an untracked source (driver row E3, `migrate.source-untracked`), the
retired original is always **tracked**, so its bytes survive in the object DB (`git show HEAD:<path>`
above) — this half of DEFECT 2 is **silent divergence, recoverable by git**, where R1/R2/R3 destroy
**uncommitted** bytes that nothing can recover. That distinction belongs in the finding and was not in
the driver's text.

**Status: CONFIRMED · origin driver · source pass's "consistent" grading REFUTED** with the repro above.

---

## R7 · The widened class — what the two passes jointly establish

Not a new claim: the arithmetic of R1–R3 and DEFECT 2 put together, stated once so it is not
re-derived. **The worktree paths a failed transaction restores** now has **four** driven populations:

| # | population | restore rule, driven | on a racer |
|---|---|---|---|
| 1 | `CONFIG_LAYER_SPECS ▸ Rewritten` (`.jigc/.gitignore`, `.jigc/version`) | **compare-and-swap** (M51) | racer's bytes stand · pre-image parked in `.jigc/displaced/` · one blocking `finalize.rollback-conflict` per path |
| 2 | the **promote** destination (`rollback_promotions`) | unconditional `fs::write` of the displaced pre-image, or **delete** for a new destination | racer's bytes **destroyed silently**, nothing parked, no finding — driven at `task finalize` (B4) **and** `milestone finalize` (R1) |
| 3 | the **retire**d original | restore **only while the path is absent** | jigc's deletion **survives silently**, original not restored, no finding (R6) |
| 4 | the **milestone record** (`RecordPreImage` + `RecordFlipGuard`) | unconditional `fs::write` (both) | racer's bytes **destroyed silently**, no finding — driven at `milestone add-task` (R2) **and** `milestone finalize` (R3) |

Population 1 is the one M51 built the discipline for; 2–4 are the same class through un-swept axes —
M45's complete-fix lens (*a fix applied where its wave pointed is not applied at all*) landing on M51's
own centrepiece. The window is *inside the transaction*, so the third party is a hook or a concurrent
process, not an interactive edit — and a formatting/lint `pre-commit` hook that rewrites files before
failing is exactly that shape, which is the shape `design/finalize.md` itself names when it justifies
CAS for population 1.

---

## R8 · Ledger summary

| claim / defect | origin | status | evidence |
|---|---|---|---|
| C1 · promote rollback unconditional, no conflict finding | codex (= driver DEFECT 2 promote half) | **CONFIRMED** | R1 — re-driven at `task finalize`; **new**: driven at `milestone finalize` |
| C2 · `RecordPreImage` restores the milestone record unconditionally | codex | **CONFIRMED** | R2 — `milestone add-task`, racer's edit gone, no finding |
| C3 · `RecordFlipGuard::drop` a second unconditional restore | codex | **CONFIRMED** | R3 — `milestone finalize`, racer's edit gone, no finding |
| C4 · `provision` amends `.jigc/.gitignore` then fails unacknowledged | codex | **CONFIRMED** | R4 — `milestone.unknown` at exit 1, ` M .jigc/.gitignore`, registry says this ack is *"the only notice"* |
| D1 · `--format json` stream discipline at the rollback-conflict class | driver | **CONFIRMED (stands)** | R5 — re-driven byte-for-byte; source pass silent |
| D2 promote half (B4) | driver + codex | **CONFIRMED (stands)** | R1 |
| D2 retire half (B5) | driver | **CONFIRMED (stands)**; source pass's clean read **REFUTED** | R6 — datum: worktree holds the racer's bytes, no finding, no parked copy |
| Obs. A13 (`milestone create` amend survives a rejection, unsaid) | driver | **re-graded** — the `create` instance of C4, not a separate observation | R4 |
| OPEN LEADS | — | **none** | every Codex claim was driven |

**Un-driven breadth, stated rather than implied:** C2's three sibling `RecordPreImage` callers
(`milestone create` · `milestone add-from-spec` · `milestone discard`, plus the `task discard` record
settlement) and C3's `squash: false` arm share the restore paths driven above but were **not** driven
here. The driver's §4 list stands unchanged beside this.

---

## R9 · Doors covered (`VERB_KINDS` spelling) — every leaf that is the door of ≥1 driven row

**13 leaves.** From the driver's table, plus the leads driven here (which add no new leaf — R2/R3/R4
land on `milestone add-task`, `milestone finalize`, `milestone provision`, all already doors of driven
driver rows):

`setup` · `migrate` · `migrate-corpus` · `rename` · `task discard` · `task finalize` ·
`milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone provision` ·
`milestone join` · `milestone finalize` · `milestone discard`

Verbs used only to build fixture state (`start`, `workflow`, `doc create`, `doc author`,
`doc set-field`, `doc set-slot`, `doc schema`, `doc list`, `task list`, `config set`, `validate`) are
**not** counted — a fixture builder is not the door of a row.

`milestone execute` is **not** a door of any driven row on this axis (it neither commits nor amends the
ignore file); `jigc milestone finalize`'s two commit-model arms are one leaf and are recorded as two
rows, which is the `COMMITTING_DOORS` shape (10 rows / 9 leaves) and not a coverage inflation.
