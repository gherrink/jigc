<!-- M52 per-axis review (re-run) — axis 4 · destroying doors — RECONCILED · driven on the installed `jigc 1.0.0-rc.16` (built from commit e519e4eb), 2026-09-21. -->

<!-- M52 per-axis review (the re-run of M51's instrument) — axis 4 · transaction / rollback — the OPUS DRIVER -->

# M52 per-axis review — AXIS 4 · transaction / rollback — the OPUS DRIVER

**Binary asserted first, before anything else:**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.16
```

Release posture (the `Route::mechanical` fence panic is `#[cfg(debug_assertions)]` and does not
exist here). Every rig built with `rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`,
two-step eval, every root a `mktemp -d`, no teardown, no `rm -rf` on a variable path. The Codex
source pass for this axis was **not** read.

**Result in one line.** **Six of M51's seven §A halves for this axis are CLOSED on rc.16**, each
re-driven with its argv — the four worktree-restore populations M51 found un-swept are now one
compare-and-swap family across **eleven** populations and **five** `<door>.rollback-conflict`
identities, and the JSON reject stream is one document at every cell I could reach. **One half is
STILL-OPEN**: C4's `milestone create` arm, narrowed by M52's own fix to the **hook-rejected** run,
where the `.jigc/.gitignore` amend still survives unnamed. **Two new defects**, both surface-tier:
`finalize.stage-failed`'s route is not copy-runnable, and `config.repoint-failed` renders an absolute
host path in the message a pinned envelope carries.

---

## 1 · The door set, derived from the code (the counts I read)

| registry | file:line | count I read | what I took from it |
|---|---|---|---|
| `ROLLBACK_POPULATIONS` | `crates/cli/src/rollback.rs:169` | **11** rows (`config-layer-worktree`, `promote-destination`, `retired-original`, `milestone-record`, `fan-out-record-flip`, `rename-worktree`, `created-doc-staged-write`, `milestone-mint-area`, `unrecorded-seed-areas`, `config-root-relocation`, `setup-install-path`) | **the axis's new spine** — it widens M51's door set by `doc author` and `config set` |
| `ROLLBACK_DOORS` | `crates/cli/src/rollback.rs` (`FINALIZE`/`MILESTONE`/`TASK_DISCARD`/`RENAME`/`CONFIG`) | **5** conflict-door identities | the `<door>.rollback-conflict` code each raced restore must raise |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:171` | **10 rows over 9 clap leaves** (`milestone finalize` carries two commit-model arms) | M51's spine, unchanged |
| `IGNORE_DOORS` | `crates/cli/src/gitignore.rs:97` | **4** production `gitignore::ensure` call sites (`setup` · `task finalize`(+both fan-out arms) · `milestone create` · `milestone provision`) | the C4/A13 half |
| `CONFIG_LAYER_SPECS` | `crates/cli/src/task.rs:4789` | **3** rows — 1 `Untouched(<reason>)` (`.jigc/config`, a directory) + **2 `Rewritten`** | the two files the M51 CAS family covered |
| `TASK_AREA_FILES` / `MILESTONE_AREA_FILES` / `WorkArea` | `crates/engine/src/state.rs:129 / :168 / :178` | **13** / **5** / **2** | the `MintedSet` rows' subject — their **complement** is what a rejected mint must not destroy |
| `DESTROYING_DOORS` (with `Disposition`) | `crates/cli/src/milestone.rs:3275` | **6** (`WORKTREE_DOORS` 4) | read; axis 3's, but `task finalize`/`milestone finalize` are the two `Displace` rows and their displacement is inside the transaction — driven at cell I |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:5415` | **4** (`store.not-found`, `store.no-such-leaf`, `store.unknown-type`, `task::FIXED_IDENTITY` = `store.fixed-identity`) | M52 audit fix 4 — driven at the committing doors in cell K |
| `InProgress::ALL` / `PostureMember::ALL` | `crates/cli/src/repo.rs:234 / :138` | **9** / **3** | the pre-guard cell — the transaction question is *does the refusal land before the first write* |
| `RelocateRefusal::ALL` | `crates/cli/src/relocate.rs:91` | **10** members | `config-root-relocation`'s door; the untrackable/unusable-root floors driven at cell L |
| `PRE_DISPATCH_FAULTS` | `crates/cli/tests/pre_dispatch_faults.rs:145` | **4** rows | read; axis 6's — no axis-4 row keys on it |
| `VERB_KINDS` | `crates/cli/src/cli.rs` | **47** leaves | the coverage spelling |

**The axis-4 door set I drove:** `COMMITTING_DOORS`' 10 rows ∪ `setup` ∪ `milestone join` ∪
`milestone provision` (M51's) **∪ `doc author` ∪ `config set`** (`ROLLBACK_POPULATIONS`' two
non-committing doors) ∪ `task bind` (a committing-door-adjacent `ENVELOPE_OWED_CODES` producer) —
**15 rows over 14 leaves**, plus `migrate` as the caller-typed route into the retire cell.

**The cell set:** M51's five failure points × two worktree states (**10**), widened by two M52 cells
the registries mint — the **minted-area** cell (a foreign byte inside the area a failed mint unwinds)
and the **posture pre-guard** cell (does an un-concluded git operation refuse before the first
write). **12 cells × 15 rows = 180 pairs.**

---

## 2 · M51 rows: CLOSED / STILL-OPEN

Every §A row of `M51/per-axis-review/README.md` → *Axis 4* re-driven on rc.16. Argv and observed
output per row; the repro blocks are in §4.

| M51 §A row | origin | verdict on rc.16 | argv driven · observed |
|---|---|---|---|
| **C1** — the promote rollback is unconditional and emits no conflict finding (driven at `task finalize` and `milestone finalize`) | codex + driver | **CLOSED** | `jigc task finalize migrate-vision-vision-cd709ad9fcd8 --approve` under a hook that appends to `VISION.md` → exit 1, **the racer's line stands** (`grep -c` → 1), one blocking `finalize.rollback-conflict` at `VISION.md`, pre-image parked at `.jigc/displaced/finalize/VISION.md.pre-image.<nanos>` and named in the route, HEAD unmoved. And `jigc milestone finalize axis-four-milestone` under a hook that appends to the **new** promotion destination `docs/decisions/eviction-policy.md` → exit 1, the destination **still there with the racer's bytes**, `finalize.rollback-conflict` whose route states the case explicitly (*"did not exist before this finalize, so the rollback would have deleted the copy jigc created — it did not"*) |
| **C2** — `RecordPreImage` rollback restores the milestone record unconditionally | codex | **CLOSED**, and at **all five** doors the registry names | `jigc milestone add-task axis-four-milestone 'axis four sub task'` under a hook that appends to the record → exit 1, the racer's line **still in the record**, `milestone.rollback-conflict` keyed at `docs/milestone-records/axis-four-milestone.md`, pre-image parked under `.jigc/displaced/milestone-op/`. Same shape driven at `milestone create` (pre-image **absent** — the route says the created copy was *not* deleted), `milestone discard --force`, `milestone add-from-spec`, and `task discard --force` (which raises its **own** `task-discard.rollback-conflict`, parked under `.jigc/displaced/task-discard/`) |
| **C3** — `RecordFlipGuard`'s `Drop` is a second unconditional restore | codex | **CLOSED**, on **both** commit-model arms | `jigc milestone finalize axis-four-milestone` under a hook that appends to the record, at `finalize.fan-out.squash` = **true** and **false** → exit 1 both times, racer's line survives both times, `milestone.rollback-conflict` + parked pre-image both times; the two arms' frames differ correctly (*"the merged docs were rolled back"* vs *"HEAD is at its pre-finalize commit"*) |
| **C4** — `milestone provision` amends `.jigc/.gitignore` and then fails without acknowledging it | codex (driver A13 its `create` instance) | **PARTIALLY CLOSED — the `provision`/guard arm CLOSED, the hook-rejected `create` arm STILL-OPEN** | CLOSED half: `jigc milestone provision nonexistent-milestone` → exit 1, `milestone.unknown`, `.jigc/.gitignore` **byte-identical**; `jigc milestone create 'Axis four milestone'` a second time → exit 1, `milestone.record-exists`, file **byte-identical**; the succeeding control still amends and still acks the union. **STILL-OPEN half (the datum):** `jigc milestone create 'Amend probe'` under a plain rejecting hook → exit 1, `.jigc/.gitignore` **amended** (` M` in `git status`), **zero** mentions of `gitignore` anywhere in stdout+stderr, and the frame reads *"nothing of milestone:amend-probe survives"* |
| **DEFECT 1** — a rollback conflict breaks `--format json` stream discipline at the one class where the document is on stderr | driver | **CLOSED** | `jigc task finalize axis-four-probe --format json` with `.jigc/.gitignore` trimmed (so the amend genuinely writes) and a hook that appends to it → exit 1, **stdout 0 bytes**, stderr parses as **exactly one** document `{findings, schema_version}` carrying **both** `finalize.commit-rejected` (keyed `task:axis-four-probe`) and `finalize.rollback-conflict` (keyed `.jigc/.gitignore`), the parked `.jigc/displaced/` path inside its route. M51's `JSONDecodeError: Extra data` does not reproduce |
| **DEFECT 2, promote half** — the promote axis silently destroys the racer's bytes | driver + codex | **CLOSED** | see C1 |
| **DEFECT 2, retire half** — the retire axis silently declines to restore | driver | **CLOSED** | `jigc task finalize migrate-vision-direction-plan-0410d8e1466d --approve` under a hook that re-creates the retired `direction/plan.md` → exit 1, the racer's file stands, `finalize.rollback-conflict` whose message states the **deletion** shape (*"jigc removed it and something has since put a file back"*), pre-image parked **directory-preserving** at `.jigc/displaced/finalize/direction/plan.md.pre-image.<nanos>`, and the promote destination `VISION.md` correctly absent |

**Score: 6 CLOSED · 1 STILL-OPEN** (C4's hook-rejected `create` arm). The STILL-OPEN half is
re-filed as DEFECT 3 below, narrowed to what rc.16 actually does.

---

## 3 · The matrix

### Cell A — hook rejection × worktree **unchanged** (no racer)

| # | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| A1 | `task finalize` | `jigc task finalize migrate-vision-direction-plan-0410d8e1466d --approve` | 1 | log `finalize.commit-rejected` | frame-argv | frame carries **no** *"apart from"* opener (nothing survived); the retired original is **byte-identical** to its pre-finalize bytes; `VISION.md` absent; `git ls-files -s` identical; HEAD unmoved | matches contract — F3's clause is correctly absent on the zero-survivor cell |
| A2 | `migrate-corpus` | `jigc migrate-corpus` (one downgraded adr) | 1 | log `migrate-corpus.commit-rejected` | frame-argv | *"the migrated bytes are **written and staged**"* — verified: `git diff --cached --name-only` = the adr, on-disk stamp `schema-version: 2`, HEAD unmoved | matches contract |
| A3 | `milestone create` | `jigc milestone create 'Amend probe'` | 1 | log `milestone-create.commit-rejected` | frame-argv | *"nothing of milestone:amend-probe survives"*; record absent; HEAD unmoved | matches contract **for the record** — and carries **DEFECT 3** for `.jigc/.gitignore` |
| A4 | `setup` | `jigc setup` on `bare` with a rejecting `.git/hooks/pre-commit` | **0** | none | none | install lands, HEAD moves — the `--no-verify` exemption is real; the pre-existing foreign hook is **preserved**, jigc's block prepended above it (`# jigc-managed pre-commit hook — end` then the user's body) | matches contract (the stated exemption) |
| A5 | `milestone provision` | `jigc milestone provision axis-four-milestone` over a user-trimmed `.jigc/.gitignore` | **0** | none | `Informational` | the **non-committing** `IGNORE_DOORS` member: union amend, user's `# my private line` kept, acked on its own line (`.jigc/.gitignore → appended index/, state/, milestones/, worktrees/, logs/, displaced/ …`) | matches contract |
| A6 | `milestone add-from-spec` | `jigc milestone add-from-spec axis-four-milestone spec:rate-limit` (hook rejects) | 1 | log `milestone-add-from-spec.commit-rejected` | frame-argv | two `note:` lines (*0 of 2 … landed; the 2 un-recorded mint(s) were unwound*), record's committed copy unchanged, `list-tasks` → 0 | matches contract |
| A7 | `milestone finalize` (×2 arms) · `rename` · `milestone add-task` · `milestone discard` · `task discard` | **not driven in this cell** | — | — | — | — | driven in **cell B** instead, which is the discriminating cell for those doors; stated in §5 rather than presented as driven |
| A8 | `milestone join` | n/a | — | — | — | — | **n/a — commits nothing** (`BEHALF_DOORS` `Neither`); no hook runs. Its own failure point is cell C4 |

### Cell B — hook rejection × worktree **concurrently edited** — the discriminating cell, keyed by `ROLLBACK_POPULATIONS` row

Every row: the hook writes at the population's path and exits 1; the assertion is *the third party's
bytes stand · jigc's pre-image is parked in the gitignored workbench · exactly one
`<door>.rollback-conflict` names both copies*.

| # | population | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| B1 | `config-layer-worktree` | `task finalize` | `jigc task finalize axis-four-probe`, `.jigc/.gitignore` trimmed then raced | 1 | `finalize.rollback-conflict` **blocking** + log `finalize.commit-rejected` | `Human` | racer's `# raced` stands; pre-image at `.jigc/displaced/finalize/.jigc/.gitignore.pre-image.<nanos>`; frame opens *"apart from the 1 path named below, which survived the rollback:"* | matches contract |
| B2 | same, `--format json` | `task finalize` | `… --format json` | 1 | as B1 | `Human` | stdout **0 bytes**; stderr **one** document; both codes in `findings`, each with a non-null `(code, target)` key | matches contract — **M51 DEFECT 1 closed** |
| B3 | `promote-destination` (same-path) | `task finalize` | `jigc task finalize migrate-vision-vision-cd709ad9fcd8 --approve`, hook appends to `VISION.md` | 1 | `finalize.rollback-conflict` | `Human` | racer's line stands; parked pre-image; ` M VISION.md`; HEAD unmoved | matches contract — **M51 DEFECT 2 promote half closed** |
| B4 | `promote-destination` (new destination) | `milestone finalize` | `jigc milestone finalize axis-four-milestone`, hook appends to `docs/decisions/eviction-policy.md` | 1 | `finalize.rollback-conflict` | `Human` | the created destination is **left standing with the racer's bytes**; route: *"…did not exist before this finalize, so the rollback would have deleted the copy jigc created — it did not. Remove it by hand…"*; record `status: active`; HEAD unmoved | matches contract — **M51 C1's milestone arm closed** |
| B5 | `retired-original` | `task finalize` | `… migrate-vision-direction-plan-0410d8e1466d --approve`, hook re-creates `direction/plan.md` | 1 | `finalize.rollback-conflict` | `Human` | racer's file stands; message names the **deletion** shape; pre-image parked **with its directory** under `.jigc/displaced/finalize/direction/` | matches contract — **M51 DEFECT 2 retire half closed** |
| B6 | `milestone-record` | `milestone add-task` | `jigc milestone add-task axis-four-milestone 'axis four sub task'` | 1 | `milestone.rollback-conflict` | `Human` | racer's line survives; parked under `.jigc/displaced/milestone-op/` | matches contract — **M51 C2 closed** |
| B7 | `milestone-record` (**pre-image absent**) | `milestone create` | `jigc milestone create 'Mcreate'`, hook *creates* the record path | 1 | `milestone.rollback-conflict` | `Human` | the racer's file is **not deleted**; route states the absent-pre-image case in its own words; **nothing parked** (correct — there is no pre-image to park) | matches contract |
| B8 | `milestone-record` | `milestone discard` | `jigc milestone discard axis-four-milestone --force` | 1 | `milestone.rollback-conflict` | `Human` | racer's line survives; parked; frame *"…record is still at its pre-discard state and its workbench is untouched"* | matches contract |
| B9 | `milestone-record` (**fifth door**) | `task discard` | `jigc task discard axis-four-sub-task --force` | 1 | **`task-discard.rollback-conflict`** | `Human` | its **own** identity and its **own** park directory `.jigc/displaced/task-discard/`; the sub-task's working area still present | matches contract — the §19 fifth door is real |
| B10 | `milestone-record` | `milestone add-from-spec` | `jigc milestone add-from-spec axis-four-milestone spec:rate-limit` | 1 | `milestone.rollback-conflict` | `Human` | racer's line survives; parked; the resume notes print; `tasks.json` restored to `{"tasks": []}` | matches contract |
| B11 | `fan-out-record-flip` | `milestone finalize` (squash **true**) | `jigc milestone finalize axis-four-milestone` | 1 | `milestone.rollback-conflict` | `Human` | racer's line survives; parked; committed record still `active` at HEAD | matches contract — **M51 C3 closed** |
| B12 | `fan-out-record-flip` | `milestone finalize` (squash **false**) | same, `finalize.fan-out.squash=false` | 1 | `milestone.rollback-conflict` | `Human` | identical, with the arm's own frame (*"HEAD is at its pre-finalize commit"*) | matches contract — C3's un-driven arm now driven |
| B13 | `rename-worktree` (old doc path) | `rename` | `jigc rename adr:eviction-policy --to 'Cache eviction' --slug cache-eviction`, hook writes at the old path | 1 | **`rename.rollback-conflict`** | `Human` | racer's bytes stand at `docs/decisions/eviction-policy.md`; message names the **deletion** shape; parked under `.jigc/displaced/rename/`; landing path absent; HEAD unmoved | matches contract — the `rename-head-restore` row's `DoorGuard("rename.dirty-tree")` classification, struck at T6/T10 on a driven datum, is confirmed gone: this arm is compare-and-swap |
| B14 | `rename-worktree` (landing path) | `rename` | same, hook writes at `docs/decisions/cache-eviction.md` | 1 | `rename.rollback-conflict` | `Human` | the created landing path is left standing (`?? docs/decisions/cache-eviction.md`), the **old doc is byte-identical to its pre-run bytes**, route states the created-copy case | matches contract |
| B15 | `rename-worktree` (`.jigc/state/file-state.json`) | `rename` | same, hook appends to the file-state record | 1 | `rename.rollback-conflict` | `Human` | racer's bytes stand; parked at `.jigc/displaced/rename/.jigc/state/file-state.json.pre-image.<nanos>`; the doc stays at its original path | matches contract — the gitignored fourth path the row names |
| B16 | `milestone-mint-area` (`MintedSet[Milestone,Task]`) | `milestone create` | `jigc milestone create 'Mint probe'`, hook plants `foreign-note.txt` in every minted area | 1 | **`milestone.foreign-bytes`** | `Human` | the area is **left standing** with the foreign file; the finding names the path and says `.jigc/` is gitignored so nothing else holds a copy; route names the re-run block on the minted id | matches contract |
| B17 | `milestone-mint-area` | `milestone add-task` | `jigc milestone add-task axis-four-milestone 'area low'`, same hook | 1 | `milestone.foreign-bytes` | `Human` | `.jigc/tasks/area-low` left standing with the foreign file; **`tasks.json` restored to `{"tasks": []}`** (the one byte this unwind puts back) | matches contract |
| B18 | `unrecorded-seed-areas` | `milestone add-from-spec` | `jigc milestone add-from-spec axis-four-milestone spec:rate-limit`, hook plants in both seeded areas | 1 | `milestone.foreign-bytes` **×2** | `Human` ×2 | **plural** frame — *"apart from the **2 paths** named below"* — one finding per area, record byte-identical, `list-tasks` → 0 | matches contract |
| B19 | the unraced **control** | `task finalize` | same fixture, plain rejecting hook | 1 | log `finalize.commit-rejected`, **no** conflict | frame-argv | retired original **byte-identical**, `.jigc/displaced/` absent, no *"apart from"* clause, index identical | matches contract — what shipped is compare-and-swap, not a blanket refusal to roll back |
| B20 | `created-doc-staged-write` | `doc author` | n/a | — | — | — | — | **n/a — the row's `Declared` reason, driven-checkable:** the create → leaf-chain → rollback span spawns no subprocess, so no hook or `git` call can race it. Its *unraced* rollback is driven at cell J |
| B21 | `config-root-relocation` | `config set` | n/a | — | — | — | — | **n/a — no hook in the window.** The transaction is `git mv` batch → `write_scalar`; `git mv` runs no hook and the door never commits, so there is no deterministic in-transaction racer. Its *unraced* rollback is driven at cell C5 |
| B22 | `setup-install-path` | `setup` | n/a | — | — | — | — | **n/a — `Site::NoRestore` by decision**; the door refuses **before** the first write (driven at C3) |

### Cell C — stage / non-hook failure × worktree unchanged

Induced with a stale `.git/index.lock` (the residue-of-a-killed-git cause), except C3–C5 whose
failure points are their own.

| # | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| C1 | `task finalize` | `jigc task finalize axis-four-probe` with `: > .git/index.lock` | **3** | `finalize.stage-failed` **blocking** | `Human` | *"jigc could not stage its own changes — no commit was made and the promotions were rolled back: `git add -- .jigc/config .jigc/.gitignore .jigc/version` failed: …"*; the pending `.jigc/.gitignore` amend **restored byte-identically to its pre-image**; `git ls-files -s` identical; HEAD unmoved; **no** *"apart from"* clause | restore + state clause match contract — **route is DEFECT 1** |
| C2 | `migrate-corpus` | `jigc migrate-corpus` with a stale `index.lock` | 1 | log `migrate-corpus.commit-rejected` | frame-argv | the **second** state clause — *"the migrated bytes are written to disk, **the stage did not complete**"* — verified: `git diff --cached` **empty**, on-disk stamp `schema-version: 2`, HEAD unmoved. The hook-cell clause (*"written and staged"*) is correctly **not** used here | matches contract |
| C3 | `setup` | `jigc setup` over an uncommitted edit in `.jigc/AGENT.md` | 1 | `setup.dirty-install-path` **blocking** | `Human` (names `--force` as the single consent) | *"nothing was installed and no install commit was made — `HEAD` is untouched…"* — verified: the user's line still on disk, ` M .jigc/AGENT.md`, HEAD unmoved | matches contract — the refusal is before the first write |
| C3b | `setup --force` | same state | **0** | `setup.forced-install-path` **advisory** | `Informational` | the install runs; the authored bytes are consumed *as documented* (`grep -c` → 0) and the advisory says so, naming each path and the recovery bound | matches contract — `--force` consents, it does not preserve |
| C4 | `milestone join` | `jigc milestone join axis-four-milestone` with two worktrees staging `shared.ts` | 1 | `combine.code-collision` **blocking** | `Human` | *"join blocked … 0 doc(s) would merge, nothing committed"*; HEAD unmoved, `git ls-files -s` identical, **both worktrees still hold their staged `shared.ts`** | matches contract |
| C4b | `milestone join --format json` | same | 1 | as C4 | `Human` | **stdout** carries exactly one document (`findings` + `milestone` + `no_docs_from` + `overlay` + `schema_version`); stderr carries the finding as plain text and **no JSON** | matches contract — this is the *adjudication* row of the stream table (document on stdout, plain diagnostics on stderr), not the reject row |
| C5 | `config set <root-knob>` | `jigc config set docs-root documentation` with `.jigc/config` chmod 0555 so `write_scalar` fails **after** the `git mv` batch lands | 1 | `config.repoint-failed` **blocking** | `Human` | the moved doc is **back at its prior home byte-identically**, `git status --short` **empty**, `git ls-files -s` row identical (same blob sha), `jigc config get docs-root` still `docs/ (pack-default)`; no conflict finding (nothing raced) | restore matches contract — **the message is DEFECT 2**. **M52's own baseline** recorded this population as `Site::NoRestore` (`rollback.rs`' sited note; `baseline-rollback.md` §1 table A's last row) — M51's axis-4 ledger never listed it as a population at all; T8's fix is real |
| C6 | `rename` · `milestone create/add-task/add-from-spec/discard` · `task discard` | **not driven in this cell** | — | — | — | — | M51 drove all six here and graded them *matches contract*; `commit_rejected_axis.rs`' third sweep iterates the same table. Not re-driven — stated in §5 |
| C7 | `milestone provision` | n/a | — | — | — | — | **n/a — stages nothing** |

### Cell D — stage failure × worktree concurrently edited

| # | rows | verdict |
|---|---|---|
| D1 | all 15 rows | **n/a with reason, not presented as driven.** The stage runs **before** the only user-code execution point a fixture controls (the hook), so no deterministic in-transaction racer exists at this failure point; the predicate under test is the same `PreImageFamily::restore` entry the hook cells drive at B1–B18. This is M52's own declared bound (VERDICT → F3: *"the non-hook cell × a surviving path cannot be driven (no racer instrument exists for it)"*), and I reach the same conclusion by driving rather than by reading it |

### Cell E — retire-untrackable refusal **inside the closure**

| # | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| E1 | `task finalize` (migration) | `jigc task finalize migrate-vision-legacy-untracked-de3d096a358f --approve` with `.jigc/tasks/<id>/source-path` rewritten to an absolute path in a `mktemp -d` outside the repo | 1 | `finalize.retire-untrackable` **blocking** | `Human` | *"…resolves outside the repository…"*; **the canary outside the repo is intact** (`CANARY` still there); the promote is rolled back (`VISION.md` absent); `git ls-files -s` **identical**; HEAD unmoved | matches contract — the M51 Inc 1 capture hoist holds on rc.16 |
| E2 | `migrate` — the cell's **caller-typed** route in | `jigc migrate legacy/untracked.md --as vision` (untracked in-repo source) | 1 | `migrate.source-untracked` **blocking** | `Mechanical` — `git add -- legacy/untracked.md` | **run verbatim → exit 0**, and the re-run then mints the task; **no task minted** by the refused run (`jigc task list — no active tasks`) | matches contract |
| E3 | the other 14 rows | n/a | — | — | — | — | **n/a — no retire phase** |

### Cell F — empty commit × worktree unchanged

| # | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| F1 | `task finalize` | `jigc task finalize record-the-eviction-policy` over a settled tree | **3** | `finalize.empty-commit` **blocking** | `Mechanical`-shaped + the discard alternative | *"task validated but produced no diff — nothing to finalize"*; route names the **runnable** re-run **and** `jigc task discard record-the-eviction-policy --force`; no *"rejected"* anywhere | matches contract |
| F2 | `milestone finalize` | `jigc milestone finalize zero-milestone` | **3** | `milestone.zero-contribution` **blocking** | `Mechanical` | *"…would land no work…"* naming the terminal-`joined` consequence; no rejection frame | matches contract |
| F3 | `rename` | `jigc rename adr:eviction-policy --to 'Eviction policy' --slug eviction-policy` | **0** | none | none | *"no-op: … already holds the title … — nothing renamed, nothing committed"*; HEAD unmoved; `grep -c rejected` → 0 | matches contract (M48 Inc 8) |
| F4 | `migrate-corpus` | `jigc migrate-corpus` on an all-current corpus | **0** | none | `Informational` | *"corpus migration: 0 migrated, 5 already current, 0 blocked"*; no rejection frame | matches contract |
| F5 | `milestone create` | `jigc milestone create 'Zero milestone'` twice | 1 | `milestone.record-exists` **blocking** | `Human` | the identity guard fires **before** any record write, so the empty record commit is unreachable | matches contract |
| F6 | `milestone add-task` | same intent twice | 1 | `milestone.sub-task-collision` **blocking** | `Human` | same shape | matches contract |
| F7 | `milestone add-from-spec` · `milestone discard` · `task discard` · `setup` · `join` · `provision` · `config set` · `doc author` | **not driven** | — | — | — | — | see §5 |

### Cell G — promote-after-retire × both worktree states

| # | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| G1 | `task finalize`, **new** destination, unraced | `jigc task finalize migrate-vision-direction-plan-0410d8e1466d --approve`, plain rejecting hook | 1 | log `finalize.commit-rejected` | frame-argv | the retired original is **byte-identical**, the promoted `VISION.md` **deleted**, `git status --short` **empty**, index identical, HEAD unmoved — full all-or-nothing across both phase-4 writes, and **no** conflict finding | matches contract |
| G2 | `task finalize`, **same-path**, raced | see B3 | 1 | `finalize.rollback-conflict` | `Human` | — | matches contract |
| G3 | `task finalize`, destination pre-occupied by an unrelated **untracked** file | same fixture + `printf 'someone else was here\n' > VISION.md` | **3** | `finalize.promote-clobber` **blocking** | `Human` | the guard refuses **before** any write; the occupant's bytes are untouched (`someone else was here`); the foreign source intact; index identical | matches contract |
| G4 | the other 14 rows | n/a | — | — | — | — | **n/a — no promote/retire phase** |

### Cell H (M52-new) — posture pre-guard × the transaction

The axis-4 question is not *does it refuse* (axis 2's) but *does the refusal land before the first
write, so that no rollback is owed*. Fixture: `dev/jigc-rig committed-singletons --git-state <member>`.

| # | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| H1–H5 | `task finalize` × `{merge, rebase-merge, cherry-pick, revert, bisect}` | `jigc task finalize axis-four-probe` | 1 | `repo.operation-in-progress` **blocking** | `Human` naming the git command per operation (`git merge --continue`/`--abort`, `git rebase …`, `git cherry-pick …`, `git revert …`, `git bisect reset`) | the trimmed `.jigc/.gitignore` is **byte-identical** after the run (the amend never ran) and `git ls-files -s` is identical at every one — the refusal is **before** the first transaction write | matches contract |
| H6 | the remaining 4 `InProgress` members (`squash-merge`, `rebase-apply`, `am`, `sequencer` / `dangling-sequencer` / `unmerged-index`) | **not driven here** | — | — | — | — | axis 2 owns the member sweep; I drove the 5 that discriminate the *transaction* question and say so |

### Cell I (M52-new) — the displacing disposition inside the transaction

`DESTROYING_DOORS`' two `Disposition::Displace` rows act on the **landed** arm; the axis-4 question
is whether a rejected run leaves the complement alone.

| # | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| I1 | `task finalize` landed, with a foreign `foreign-note.txt` in the task area | `jigc task finalize axis-four-probe --format json` | **0** | none | `Informational` | stdout document carries `committed.displaced: [{from: ".jigc/tasks/axis-four-probe/foreign-note.txt", to: ".jigc/displaced/axis-four-probe/foreign-note.txt"}]`, **repo-relative on both sides**; stderr carries the `note:` prose and **no JSON**; the file is at the `to` path | matches contract |
| I2 | same, hook-rejected | `… --format json` under a rejecting hook | 1 | `finalize.commit-rejected` | `Human` | stdout **empty**, stderr one document; **no displacement happened** — the foreign file and the whole task area are still in place | matches contract — the teardown is post-commit, so a reject destroys nothing |

### Cell J — `created-doc-staged-write`, the unraced rollback

| # | door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| J1 | `doc author` | `jigc doc author adr --from-file <payload naming an undeclared section> --task record-the-eviction-policy` | 1 | `write.unknown-section` **blocking** | `Mechanical` (`jigc doc schema adr`) | the created staged instance is **removed** — `.jigc/tasks/<id>/docs/` is back to `commit:<id>.md` + `provenance.json` | matches contract for the **bytes**; see OBSERVATION 1 for the residue |

### Cell K — `ENVELOPE_OWED_CODES` at the committing doors (M52 audit fix 4)

| # | door × code | argv driven | exit | arm | key | verdict |
|---|---|---|---|---|---|---|
| K1 | `rename` × `store.not-found` | `jigc rename adr:nope --to 'X' --slug xx --format json` | 1 | **findings envelope** | `{store.not-found, adr:nope}` | matches contract |
| K2 | `rename` × `store.unknown-type` | `jigc rename nosuchtype:nope --to 'X' --slug xx --format json` | 1 | **findings envelope** | `{store.unknown-type, nosuchtype}` | matches contract |
| K3 | `milestone add-from-spec` × `store.not-found` | `jigc milestone add-from-spec axis-four-milestone spec:nope --format json` | 1 | **findings envelope** | `{store.not-found, spec:nope}` | matches contract |
| K4 | `milestone add-from-spec` × `store.unknown-type` | `… nosuchtype:nope --format json` | 1 | **findings envelope** | `{store.unknown-type, nosuchtype:nope}` | matches contract |
| K5 | `task bind` × `task-bind.undeclared-role` | `jigc task bind decision nosuchtype:nope bind-probe --format json` | 1 | flattened `{error}` | — | matches contract — F5's declared posture: the new `task bind` codes flatten with code and route rather than joining the owed set |
| K6 | `rename` × `store.malformed-slug` | `jigc rename 'adr:Bad Slug' --to 'X' --slug xx --format json` | 1 | flattened `{error}` | — | matches contract — the declared **negative half** of the owed set (M50's measured grounds) |
| K7 | `rename` × `write.identity-change` | `jigc rename vision:vision --to 'X' --slug xx --format json` | 1 | flattened `{error}` | — | matches contract — a non-member code; `store.fixed-identity` (the owed member) is a different cell, axis 1/5's |

Every row: stdout **0 bytes**, stderr exactly one JSON document.

### Cell L — the root-knob move floors (`config-root-relocation`'s pre-guards)

| # | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|
| L1 | `jigc config set docs-root .git` | 1 | `config.untrackable-root` | `Human` | names git's own `error: invalid path`; `git ls-files -s` **identical**, nothing moved | matches contract (the M49 HIGH stays closed) |
| L2 | `jigc config set placement-root .git` | 1 | `config.untrackable-root` | `Human` | same, at the second knob | matches contract |
| L3 | `jigc config set docs-root .jigc` | 1 | `config.workbench-root` | `Human` | *"the tree `jigc uninstall` removes whole"*; index identical | matches contract |
| L4 | `jigc config set docs-root /absolute/elsewhere` | 1 | `config.unusable-root` | `Human` | names the read-back divergence; index identical | matches contract |
| L5 | `jigc config set docs-root ../outside` | 1 | `config.untrackable-root` | `Human` | *"resolves outside the repository root"*; index identical | matches contract |

### Cell M — the transaction's identity in the invocation log

| # | door | argv driven | observed | verdict |
|---|---|---|---|---|
| M1 | `task finalize` (raced config layer) | `jigc config set invocation-log true` then the B1 fixture | the record carries `exit 1`, `error_code: finalize.commit-rejected`, `finding_codes: ["finalize.rollback-conflict"]` — the carried finding is **not** dropped | matches contract (M50 audit MEDIUM stays closed) |

---

## 4 · Repro blocks

### R-A · DEFECT 2 promote half (M51 §A) → CLOSED

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf '# Direction\n\nOur aim is a deterministic compiler.\n\n## Principles\n\nStructure to the CLI.\n' > VISION.md
git add -A && git commit -q -m 'foreign VISION.md'
$JIGC migrate VISION.md --as vision                      # task minted: migrate-vision-vision-cd709ad9fcd8
printf 'title: Vision\nsections:\n  - id: thesis\n    set:\n      thesis: |-\n        <<Deterministic context compiler.>>\n  - id: invariants\n    set:\n      invariants: |-\n        <<Structure to the CLI.>>\n  - id: open-questions\n    set:\n      open-questions: |-\n        <<Which domains earn a pack.>>\n' \
  | $JIGC doc author vision --from-file - --task migrate-vision-vision-cd709ad9fcd8
$JIGC doc set-field 'commit:migrate-vision-vision-cd709ad9fcd8#header/type' --task … --value docs
printf 'migrate in place' | $JIGC doc set-slot 'commit:…#summary' --task … --from-file -
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/VISION.md"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit

$JIGC task finalize migrate-vision-vision-cd709ad9fcd8 --approve ; echo $?
# -> 1
# `git commit` was rejected (no commit was made):
# NO
#
# apart from the 1 path named below, which survived the rollback: task migrate-vision-vision-cd709ad9fcd8
# is intact — nothing was committed, … Fix the hook's complaint and follow each surviving path's
# route below, then re-run `jigc task finalize migrate-vision-vision-cd709ad9fcd8 --approve`.
# blocking · finalize.rollback-conflict — `VISION.md` changed while this finalize was running, so the
#   rollback did not restore it: the bytes on disk are not the ones jigc wrote
#   at: VISION.md
#   route: … this finalize's pre-image at `.jigc/displaced/finalize/VISION.md.pre-image.1789962378282863000` …
grep -c 'raced by a concurrent editor' VISION.md   # -> 1     M51: 0 (the bytes were GONE)
find .jigc/displaced -type f                       # -> .jigc/displaced/finalize/VISION.md.pre-image.…
git log --oneline -1                               # -> bca6804 foreign VISION.md   (unmoved)
```

### R-B · DEFECT 2 retire half → CLOSED

```
… same shape, foreign at direction/plan.md, destination new …
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '# recovered by a concurrent editor\n' > "$REPO/direction/plan.md"
echo NO >&2
exit 1
HOOK
$JIGC task finalize migrate-vision-direction-plan-0410d8e1466d --approve ; echo $?
# -> 1 · blocking · finalize.rollback-conflict — `direction/plan.md` changed while this finalize was
#        running, so the rollback did not restore it: jigc removed it and something has since put a file back
cat direction/plan.md              # -> "# recovered by a concurrent editor"  (the racer's file stands)
find .jigc/displaced -type f       # -> .jigc/displaced/finalize/direction/plan.md.pre-image.…  (dir preserved)
ls VISION.md                       # -> No such file or directory   (the promote correctly undone)
```

### R-C · C2/C3 — the record family, five doors and both flip arms → CLOSED

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC milestone create 'Axis four milestone'
REC=docs/milestone-records/axis-four-milestone.md
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/$REC"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit
$JIGC milestone add-task axis-four-milestone 'axis four sub task' ; echo $?
# -> 1 · blocking · milestone.rollback-conflict at docs/milestone-records/axis-four-milestone.md
#        route: … this milestone-op's pre-image at `.jigc/displaced/milestone-op/docs/milestone-records/…`
grep -c 'raced by a concurrent editor' $REC     # -> 1      M51: 0 (GONE)

# the other four doors, same hook:
$JIGC milestone create 'Mcreate'                # pre-image ABSENT -> conflict; route says the created copy was NOT deleted
$JIGC milestone discard axis-four-milestone --force   # -> milestone.rollback-conflict
$JIGC task discard axis-four-sub-task --force         # -> task-discard.rollback-conflict, parked under .jigc/displaced/task-discard/
$JIGC milestone add-from-spec axis-four-milestone spec:rate-limit  # -> milestone.rollback-conflict

# the flip, both commit models (fixture: create -> add-task -> provision -> author an adr in the worktree):
$JIGC milestone finalize axis-four-milestone                        # squash: true
$JIGC config set finalize.fan-out.squash false ; $JIGC milestone finalize axis-four-milestone
# both -> 1 · milestone.rollback-conflict; racer's line survives; pre-image parked
```

### R-D · DEFECT 1 — the JSON reject stream → CLOSED

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC start --workflow quick-fix "axis four probe"        # -> axis-four-probe
echo x > touched.txt ; git add -A
$JIGC doc set-field 'commit:axis-four-probe#header/type' --task axis-four-probe --value chore
printf 'axis four probe' | $JIGC doc set-slot 'commit:axis-four-probe#summary' --task axis-four-probe --from-file -
printf 'tasks/\n# my private line\n' > .jigc/.gitignore      # trimmed, so the amend genuinely WRITES
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '# raced\n' >> "$REPO/.jigc/.gitignore"
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit
$JIGC task finalize axis-four-probe --format json > out.json 2> err.txt ; echo $?
# -> 1
wc -c < out.json                      # -> 0
python3 -c "import json;d=json.load(open('err.txt'));print(sorted(d));print([f['code'] for f in d['findings']])"
# -> ['findings', 'schema_version']
# -> ['finalize.commit-rejected', 'finalize.rollback-conflict']
# M51: json.decoder.JSONDecodeError: Extra data: line 4 column 1
```

**The `PostWrite::Untouched` cell, driven and worth stating**: with the ignore file left at its full
entry set, the same fixture raises **no** conflict at all — jigc wrote nothing there, so the racer's
append is not jigc's to reconcile and the rollback correctly says nothing. That is why the repro above
trims the file first, and it is the shape a reader re-running M51's block verbatim will hit.

### R-E · DEFECT 3 (STILL-OPEN) — the `milestone create` amend survives a rejected commit, unnamed

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf 'tasks/\n# my private line\n' > .jigc/.gitignore
cp .jigc/.gitignore /tmp/ign-pre.txt
cat > .git/hooks/pre-commit <<'HOOK'
#!/bin/sh
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit
$JIGC milestone create 'Amend probe' > o.txt 2> e.txt ; echo $?
# -> 1
# `git commit` was rejected (no commit was made):
# NO
#
# nothing was committed — the record write and the milestone workbench were both rolled back, so
# nothing of milestone:amend-probe survives. Fix the hook's complaint, then re-run `jigc milestone create 'Amend probe'`.
diff /tmp/ign-pre.txt .jigc/.gitignore
# -> 2a3,8   index/ state/ milestones/ worktrees/ logs/ displaced/     (the amend SURVIVED)
grep -c gitignore o.txt e.txt      # -> 0 0     (the run named it nowhere)
git status --short                  # ->  M .jigc/.gitignore
```

### R-F · DEFECT 1 (new) — `finalize.stage-failed`'s route is not copy-runnable

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC start --workflow quick-fix "axis four probe"
echo x > touched.txt ; git add -A
$JIGC doc set-field 'commit:axis-four-probe#header/type' --task axis-four-probe --value chore
printf 'axis four probe' | $JIGC doc set-slot 'commit:axis-four-probe#summary' --task axis-four-probe --from-file -
: > .git/index.lock
$JIGC task finalize axis-four-probe ; echo $?
# -> 3
# blocking · finalize.stage-failed — jigc could not stage its own changes — no commit was made and the
#   promotions were rolled back: `git add -- .jigc/config .jigc/.gitignore .jigc/version` failed: …
#   at: task:axis-four-probe
#   route: resolve the embedded git failure (e.g. remove a stale `.git/index.lock`), then re-run `jigc task finalize`
rm -f .git/index.lock
jigc task finalize ; echo $?                # the route, followed verbatim
# -> 2
# error: the following required arguments were not provided:
#   <ID>
# Usage: jigc task finalize <ID>

# the SAME door's hook-rejection frame, for contrast:
$JIGC task finalize axis-four-probe 2>&1 | grep -o 're-run `jigc task finalize[^`]*`'
# -> re-run `jigc task finalize axis-four-probe`
```

### R-G · DEFECT 2 (new) — `config.repoint-failed` renders an absolute host path in the pinned envelope

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# … land a committed adr at docs/decisions/eviction-policy.md through record-decision …
chmod 0555 .jigc/config                      # make write_scalar fail AFTER the git mv batch lands
$JIGC config set docs-root documentation --format json > o.json 2> e.json ; echo $?
chmod 0755 .jigc/config
# -> 1 ; o.json is 0 bytes ; e.json parses as one {findings, schema_version} document:
# {
#   "code": "config.repoint-failed",
#   "key": { "code": "config.repoint-failed", "target": ".jigc/config/manifest.yaml" },
#   "message": "`docs-root` was not set to `documentation`: could not write
#               /private/var/folders/nj/…/repo/.jigc/config/manifest.yaml: Permission denied (os error 13)
#               — the re-point was undone",
#   "location": { "address": ".jigc/config/manifest.yaml", … },
#   …
# }
# The rollback itself is clean: the doc is back at docs/decisions/eviction-policy.md byte-identically,
# `git status --short` is EMPTY, and `git ls-files -s` carries the same blob sha as before the run.
```

### R-H · the `MintedSet` rows

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
cat > .git/hooks/pre-commit <<'HOOK'
#!/bin/sh
for d in "$REPO"/.jigc/milestones/*/ "$REPO"/.jigc/tasks/*/ ; do
  [ -d "$d" ] && printf 'a third party wrote this\n' > "$d/foreign-note.txt"
done
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit
$JIGC milestone create 'Mint probe' ; echo $?
# -> 1 · blocking · milestone.foreign-bytes — `.jigc/milestones/mint-probe` holds bytes jigc did not
#        write, so the working area this call minted was left standing rather than removed with them
find .jigc/milestones -mindepth 1    # -> the area + foreign-note.txt   (the third party's bytes survive)
find .jigc/tasks -mindepth 1         # -> (empty: create mints no task area)

# add-task: same hook -> milestone.foreign-bytes at .jigc/tasks/area-low, and:
cat .jigc/milestones/axis-four-milestone/tasks.json   # -> {"tasks": []}   (the one byte this unwind puts back)

# add-from-spec with two criteria: TWO findings and the plural frame
#   "apart from the 2 paths named below, which survived the rollback: …"
```

### R-I · the `rename-worktree` population, all three path shapes

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# … land adr:eviction-policy through record-decision …
# (a) racer at the OLD doc path (jigc git-mv'd it away, the hook re-creates it):
#     -> 1 · rename.rollback-conflict — "jigc removed it and something has since put a file back"
#            parked at .jigc/displaced/rename/docs/decisions/eviction-policy.md.pre-image.…
# (b) racer at the LANDING path:
#     -> 1 · rename.rollback-conflict — "did not exist before this rename, so the rollback would have
#            deleted the copy jigc created — it did not"; the OLD doc is byte-identical to its pre-run bytes
# (c) racer at .jigc/state/file-state.json:
#     -> 1 · rename.rollback-conflict at .jigc/state/file-state.json
#            parked at .jigc/displaced/rename/.jigc/state/file-state.json.pre-image.…
# all three: HEAD unmoved, docs/decisions/ still holds eviction-policy.md
```

### R-J · the posture pre-guard, five git states

```
for g in merge rebase-merge cherry-pick revert bisect; do
  rig=$(dev/jigc-rig committed-singletons --git-state $g --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
  $JIGC start --workflow quick-fix "axis four probe" ; …
  printf 'tasks/\n# private\n' > .jigc/.gitignore ; cp .jigc/.gitignore /tmp/ign-pre.txt
  $JIGC task finalize axis-four-probe ; echo $?
  # -> 1 · blocking · repo.operation-in-progress — a <op> is in progress — the repository is not in a
  #        committable state ; route names `git <op> --continue` / `--abort` (bisect: `git bisect reset`)
  diff /tmp/ign-pre.txt .jigc/.gitignore    # -> identical at every one: the refusal precedes the first write
done
```

---

## 5 · What I did NOT drive, and why

Stated plainly; none of it presented as driven.

1. **Cell D in full (stage failure × concurrently edited), all 15 rows.** No deterministic
   in-transaction racer exists before the stage — the hook is the only controllable execution point
   and it runs after. The restore predicate is the one shared `PreImageFamily::restore` entry, driven
   at B1–B18. M52's own VERDICT declares this cell undrivable for the same reason; I reached it by
   driving rather than by reading it.
2. **Cell B at `created-doc-staged-write` (`doc author`) and `config-root-relocation` (`config set`)** —
   B20/B21. Both rows' windows spawn no subprocess a fixture can put user code into: `doc author`'s
   create → leaf-chain → rollback span runs no `git` call (which is the `Declared` row's own stated
   reason, and it is checkable by driving: no hook fires), and `config set`'s transaction is a
   `git mv` batch (no hooks) followed by a file write, with no commit. Both rows' **unraced**
   rollbacks are driven (J1, C5).
3. **Cell A at `milestone finalize` (both arms), `rename`, `milestone add-task`, `milestone discard`,
   `task discard`** — the plain hook-rejection-with-no-racer cell. Each of those five doors is driven
   in **cell B** (the racer cell), which exercises the same frame plus the conflict; M51 drove all
   five in cell A and graded them *matches contract*, and `commit_rejected_axis.rs` sweeps the same
   table. Not re-driven for time.
4. **Cell C at `rename`, `milestone create/add-task/add-from-spec/discard`, `task discard`** — same
   reason; M51's C2–C10 stand and the non-hook frame is shared code, driven here at C1 and C2.
5. **Cell F at `milestone add-from-spec`, `milestone discard`, `task discard`, `setup`, `join`,
   `provision`, `config set`, `doc author`.** Time; each is fenced by `commit_rejected_axis.rs`'
   second sweep.
6. **The remaining four `InProgress::ALL` members** (`squash-merge`, `rebase-apply` / `am`,
   `sequencer` / `dangling-sequencer`, `unmerged-index`) at the committing doors. Axis 2 owns the
   member sweep; I drove the five that discriminate the *transaction* question (refusal before the
   first write) and stopped there.
7. **A failure point at `milestone provision` that lies AFTER its `gitignore::ensure`.** I probed the
   zero-sub-task arm and a HEAD-moved-past-base arm; both exited **0** and acked. M52's T9 moved the
   `ensure` behind the guards, so the M51 C4 shape may no longer be reachable at this door at all —
   I could not construct a reaching failure and say so rather than claiming the cell closed.
8. **A genuine concurrent *process* racing a pre-image.** Every race above is the `pre-commit` hook,
   which is the instrument `design/finalize.md` names. A second process would test the scheduler.
9. **`.jigc/version` as the raced path** (the second `CONFIG_LAYER_SPECS ▸ Rewritten` row). Only
   `task finalize` reaches the stamp refresh and the restore entry is shared with `.jigc/.gitignore`,
   driven at B1/B2; M51 drove the `.jigc/version` cell itself and graded it *matches contract*.

**Fixture honesty.** Two fixtures wrote into `.jigc/` rather than reaching a state by driving:
E1 rewrites `.jigc/tasks/<id>/source-path` (the only way to reach a guard that exists because that
value can be corrupted — the caller-typed route in is refused at the door, driven at E2), and the
`.jigc/.gitignore` trims, which are the user-co-owned file the cell is *about*. Everything else was
built by driving the binary. The `chmod 0555` cell (C5) carries the platform bound M52 declares:
it passes vacuously as root, and this run was not root.

---

## 6 · Defects

### DEFECT 1 — `finalize.stage-failed`'s route is not copy-runnable, while the same door's other frame is

**Severity:** LOW (surface/route tier; no bytes at risk). **Repro:** §4 R-F.

**Contract violated.** M47 swept the survivable frame over the committing axis with, in the
roadmap's own words, *"a shell-safe **copy-runnable re-run argv**"* per door, and M51's route floor
widened to blocking findings so that *a blocked finalize names its recovery*. Driven, the recovery
`finalize.stage-failed` names is **`jigc task finalize`** with no `<ID>`; pasted, it exits **2** with
a clap usage error. The task id is not missing — `stage_failed_finding(task_id, git_error)`
(`crates/cli/src/task.rs:5508`) **takes it** and spends it on the locus (`work_unit_location(task_id)`)
while the route is a bare `&'static str` (`:5518-5519`). The sibling frame at the identical door, in
the identical run shape, prints `re-run \`jigc task finalize axis-four-probe\``.

**Checked and NOT overclaimed:** `finalize.stage-failed` is a `gate_coverage.rs` member (`:255`)
but its row is `Door::FinalizeOnly(NotPreviewable::NotYetExistent)` at `Tier::LaterCause`, so
`jigc task validate` deliberately does **not** preview it. The route reaches a reader at the
finalize door only — which is where it was driven.

### DEFECT 2 — `config.repoint-failed` renders an absolute host path in the message a pinned envelope carries, and the declared reason for leaving it says it is not a finding surface

**Severity:** LOW (law 1, printed-path rule). **Repro:** §4 R-G.

**Contract violated.** `design/surface-contract.md` → law 1's printed-path fence: the rule has one
home, `crate::render::repo_relative`, *"the honest absolute for a path genuinely outside the
repository"* — and this path is **inside** it. The finding's own
`location.address` is the repo-relative `.jigc/config/manifest.yaml`, so the correct spelling is in
hand **at the same construction**: `RepointBlocker::Cause { at, cause }` documents `at` as *"the path
the reason is about, repo-relative"* and `cause` as *"a rendered `anyhow` chain"* — and the chain is
what carries `/private/var/folders/…/repo/.jigc/config/manifest.yaml` onto the text surface **and
into the pinned `findings[].message`**.

**The second half is the record, not just the render.** `crates/cli/tests/repo_relative_paths.rs`'
`UNSWEPT_PRODUCERS` row for `crates/cli/src/config.rs` (20 sites) gives as its reason
*"`with_context` I/O faults on cascade-layer reads and writes — **an error channel, not a finding
surface**; the door's own finding text is closed by `trackable.rs`"*. Driven, one of those 20 sites'
text **is** a finding surface: it is the `message` of a blocking `Finding` with a `(code, target)`
key, on both the text arm and the 1.0-pinned JSON arm. That row was re-read and corrected at M52
Increment 10 / T10 — the correction fixed the read-vs-write split and **kept** the clause this
repro falsifies. The count (20) is fenced; the reason is prose, and this is the datum against it.

### DEFECT 3 — a hook-rejected `milestone create` leaves its `.jigc/.gitignore` amend on disk, names it nowhere, and says *"nothing … survives"*

**Severity:** LOW (law-1 disclosure; **no data loss** — the amend is a union and the user's own lines
are kept, verified). **Repro:** §4 R-E. **This is M51 §A's C4 narrowed to what rc.16 still does**,
and is the only §A half not closed.

**Contract violated.** `gitignore::IGNORE_DOORS`' row for this door declares its ack to be *"the
`minted milestone:` ack itself, under the record lines — this door's whole ack rides the envelope's
`text` key, so the line reaches a driver without a second channel"*. On a rejected run that ack does
not exist, so the declared channel never opens while the write survives — driven: zero occurrences
of `gitignore` in stdout+stderr, ` M .jigc/.gitignore` in `git status`. And the frame's own state
clause reads *"nothing of milestone:amend-probe survives"* with **no** *"apart from"* opener, which
is the shape M52's F3 fix exists to prevent — here the surviving path is outside
`ROLLBACK_POPULATIONS` (`config-layer-worktree`'s doors are `task finalize` / `milestone finalize`
only), so the threaded count cannot see it and the clause is composed as if nothing survived.

**What M52 did fix, driven, so the finding is not overstated:** the *guard-refusal* arm at **both**
doors is closed — `milestone provision nonexistent-milestone` and a taken-slug `milestone create`
leave the file byte-identical, and the succeeding control still amends and still acks. What remains
is the **hook-rejected** arm of the one door that both amends and commits outside the finalize
executor.

---

## 7 · Observation (not graded a defect)

**A rolled-back `jigc doc author` discards the created instance's bytes but keeps its provenance
entry and its role binding.** Driven (§3 J1): after `doc author adr` fails at an undeclared section,
`.jigc/tasks/<id>/docs/` is back to what the create found — but `docs/provenance.json` still reads
`"adr:eviction-policy": "created"` and `roles.json` still binds `"decision": "adr:eviction-policy"`,
for an instance that does not exist. `jigc task validate` is clean, `jigc task finalize` lands at
**exit 0**, and the commit carries no adr while the task's recorded `decision` role names one.

It is recorded here rather than as a defect for two reasons, both checked: **there is no dead end**
— `jigc doc show adr:eviction-policy --task <id>` answers `store.not-staged` with a route that runs,
and both `doc create adr --title 'Eviction policy'` and a corrected re-`author` succeed (M48's HIGH
fix holds) — and **the orphaned binding is already on the record** as M48's own subject, whose fix
was the route rather than the binding's removal. `design/write-commands.md`'s *"a corrupt payload is
rejected whole (atomic), never half-applied"* is about the payload's **leaves**, and those are
fully discarded. `ROLLBACK_POPULATIONS`' `created-doc-staged-write` row states its subject as
*"a staged instance … restored to what the create found"*, which is exactly what it does.

**Stated bound, confirmed not a new finding:** `finalize.stage-failed`'s *message* quotes git's own
stderr verbatim, which carries an absolute path — `design/surface-contract.md` disposes that as the
*"quoting an invocation"* case, and it is why DEFECT 2 above is filed against `config.rs`'s composed
`cause` and not against this one.

---

## 8 · Counts

- **(door, cell) pairs in the axis:** 15 rows × 12 cells = **180**
- **Driven rows recorded with a repro block:** **61** — cell A 6 · cell B 19 · cell C 6 · cell E 2 ·
  cell F 6 · cell G 2 (G2 = B3) · cell H 5 · cell I 2 · cell J 1 · cell K 7 · cell L 5 · cell M 1 ·
  the `PostWrite::Untouched` control (R-D's second half)
- **Rows recorded `n/a` with a stated reason:** A7, A8, B20–B22, C6, C7, D1, E3, F7, G4, H6 — covering
  the remainder, each keyed to *why the failure point does not exist at that door* or *why no
  instrument reaches it*, never to "not reached"
- **M51 §A halves re-driven:** **7** — **6 CLOSED**, **1 STILL-OPEN**
- **New defects:** **2** (plus the STILL-OPEN half re-filed as DEFECT 3) · **Observations:** 1
- **Doors of ≥1 driven row:** **16 leaves** (M51: 13; the three new ones are `doc author`,
  `config set` and `task bind`, all reached through `ROLLBACK_POPULATIONS` and `ENVELOPE_OWED_CODES`)

---

## 9 · What this adds over flow-53 arm 1 (and the shipped axis suites beside it)

Flow-53 **arm 1** iterates `ROLLBACK_POPULATIONS` — a code-side registry, count-fenced by a source
scan over the restore functions — and asserts, per population, that a deterministic in-transaction
racer's bytes are preserved, that `FileCas` rows emit `<door>.rollback-conflict`, that `MintedSet`
rows leave the area and name it, and that the JSON reject stream is one document at every cell.
Beside it, `crates/cli/tests/commit_rejected_axis.rs` sweeps `COMMITTING_DOORS` three times and
`rollback_population_registry.rs` fences each row's discipline against the source.

This review adds five things none of those reach:

1. **The row-by-row verdict against M51's own ledger, on the installed binary.** Arm 1 asserts the
   property; it cannot say *the seven halves M51 filed against rc.15 are six closed and one open*.
   That comparison is the re-run's first deliverable and it is a fact about `1.0.0-rc.16` as
   installed, after the seven audit fixes, not about the tree the suite compiles from.
2. **The populations' cells the registry does not carry, driven anyway.** `milestone-record`'s
   **absent-pre-image** arm (the route that says the created copy was *not* deleted), `rename`'s
   **third** path (`.jigc/state/file-state.json`, gitignored, in no commit), `add-from-spec`'s
   **plural** survivor frame, and the `PostWrite::Untouched` cell — where jigc wrote nothing and
   therefore correctly raises nothing, which is the cell that makes M51's DEFECT-1 repro block
   non-reproducing verbatim and would read as a regression to anyone who did not drive it.
3. **The cross of the rollback surface with the *route*, not just the finding.** Arm 1 asserts the
   conflict's presence and its key. Driving each frame's recovery **as a shell command** is what
   surfaced DEFECT 1 — a route on the previewed tier that exits 2 when pasted, three lines below a
   sibling route at the same door that runs.
4. **The `n/a` half with reasons derived by driving rather than from a cell list.** Cell D and rows
   B20–B22 are recorded *because I looked for an instrument and there is none* — no subprocess in
   `doc author`'s window, no hook in `config set`'s, no execution point before the stage — which is
   the same conclusion M52's VERDICT states, reached independently instead of inherited.
5. **Live confirmation, on the release binary, of four M52 fixes that land on this axis.** T6's
   `rename` FileCas (the struck `DoorGuard` classification), T8's `config-root-relocation` restore (the baseline's
   `Site::NoRestore` row, and a population M51's axis-4 door set never carried), Increment 1's one-document reject arm, and audit fix 4's
   `ENVELOPE_OWED_CODES` at two committing doors. The audit verified these before the bump; this is
   the first drive of them on the installed `1.0.0-rc.16`.

---
---

# RECONCILIATION — axis 4 · transaction / rollback

**Reconciler's posture.** Everything below was driven by me on `/Users/maurice/.local/bin/jigc`,
asserted first: `jigc --version` → **`jigc 1.0.0-rc.16`**. Rigs built two-step
(`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`), every
root a `mktemp -d`, no teardown, no `rm -rf` on a variable path. Source-shape claims (registry
counts, call-site sets) are verified against the tree at `HEAD` and are marked **source-datum**
rather than driven, because a count is not a behaviour.

**The rule applied** (`acceptance-design.md` → The reconciliation rule): a claim by one pass the
other cannot reproduce is a **lead**, not a finding. Every Codex claim was entered as
`lead(codex, …)` and then driven to a repro or to a falsifying datum; every driver DEFECT the
source pass contradicts or is silent on stays a finding and was **re-driven once here**.

**Headline.** The Codex pass's top-line conclusion — *"no omitted transaction door, uncaptured
in-closure write, unconditional rollback remaining in the reviewed populations, or staged-path
family missing its corresponding pre-image axis"* — is **REFUTED on one leg**, by the driver's
DEFECT 3 re-driven here: `milestone create` writes `.jigc/.gitignore` **inside its commit
transaction with no captured pre-image**, and on a hook rejection that write survives, is restored
by nothing, is named nowhere, and is contradicted by the frame's own *"nothing … survives"*. Every
other Codex claim — all four M51 row dispositions and all six completeness reads — is **CONFIRMED**.

---

## Demotions

**None.** Every row the driver marks *driven* carries, in the row itself or in its §4 block, both an
**argv** and an **observed result**; no row asserts an outcome without a command that produced it.
Three blocks are **elliptical** in their fixture construction rather than absent (R-I's hook script,
R-H's `add-from-spec` arm, R-G's `record-decision` preamble), so I re-drove the thinnest of them —
**R-I(a) and R-I(b) both reproduce exactly** (below, D-4), which settles them as driven rather than
inferred. The blocks stay as written, with the ellipsis noted here.

**One record-accuracy correction, not a demotion.** §8's *"Driven rows recorded with a repro block:
**61**"* does not reconcile with its own table: cell C holds **seven** driven rows (C1, C2, C3, C3b,
C4, C4b, C5), not the six the line credits, and the per-cell figures as printed sum to 62 (+1
control = 63), not 61. Counted off the table: **63 driven rows + the `PostWrite::Untouched`
control = 64**. The rows are all there; the total is wrong in the conservative direction.

---

## Reconciliation ledger — Codex claims

Codex filed **no leads** for this axis (its CLAIMS section reports none). What it *did* file — four
M51 row dispositions and six completeness reads — is claim-shaped and load-bearing, so each is
entered and driven.

### CL-0 · "No source-grounded completeness defect found" — no omitted transaction door, no uncaptured in-closure write, no unconditional rollback remaining, no staged-path family missing its pre-image axis

**REFUTED** (on the *uncaptured in-closure write* leg; the other three legs stand — see CL-1…CL-10).

**Falsifying datum, driven** (my re-drive of the driver's R-E, rig `committed-singletons`):

```
printf 'tasks/\n# my private line\n' > .jigc/.gitignore ; cp .jigc/.gitignore "$RIG/ign-pre.txt"
cat > .git/hooks/pre-commit <<'HOOK'
#!/bin/sh
echo NO >&2
exit 1
HOOK
chmod +x .git/hooks/pre-commit
$JIGC milestone create 'Amend probe' > o.txt 2> e.txt ; echo $?
# -> 1
# `git commit` was rejected (no commit was made):
# NO
#
# nothing was committed — the record write and the milestone workbench were both rolled back, so
# nothing of milestone:amend-probe survives. Fix the hook's complaint, then re-run
# `jigc milestone create 'Amend probe'`.
diff "$RIG/ign-pre.txt" .jigc/.gitignore
# -> 2a3,8
# >  index/ state/ milestones/ worktrees/ logs/ displaced/      (the amend SURVIVED)
grep -c gitignore o.txt e.txt      # -> 0  0      (named nowhere on either stream)
git status --short                 # ->  M .jigc/.gitignore
```

**And the source leg, read at `HEAD`** — this is precisely Codex's own hunt item *"a write inside a
commit closure with no captured pre-image"*: `crates/cli/src/milestone.rs:657` is
`let ignore = crate::gitignore::ensure(&jigc_root)?;`, and `awk 'NR>=600 && NR<=720'` over that
function finds **no** `PreImageFamily`, no `capture`, no `pre_image` — the only match in the window
is an unrelated comment about the record commit's hook stream. `ROLLBACK_POPULATIONS`'
`config-layer-worktree` row (`rollback.rs:171`) carries `doors: &[&["task","finalize"],
&["milestone","finalize"]]` — `milestone create` is **not** one of them, which is why the threaded
survivor count cannot see this path and the frame composes as if nothing survived.

**Why Codex missed it, stated plainly rather than left implicit:** its CL-4 examined
`gitignore::ensure`'s ordering at **`milestone provision`** (`milestone.rs:2732`) and found it moved
behind the guards — which is true and is confirmed below. The **sibling call site at `milestone
create`** (`milestone.rs:657`) received the same guard-ordering fix (M52 Inc 5 / T9, and its
doc-comment says so) but **not** a pre-image, and the pass generalized the one to the other. This is
the incomplete-sweep shape by axis: the fix was applied over the *guard-refusal* arm at both doors
and not over the *hook-rejection* arm at either.

### CL-1 · M51 C1 — the promotion rollback is CLOSED: destinations captured before `fs::copy`, retirements in the same family, failure calls the CAS restore which preserves live bytes, parks the pre-image and returns a conflict finding

**CONFIRMED — both halves driven.**

*Promote half* (rig `fresh`; the same-path arm, where bytes are genuinely at risk):

```
printf '# Direction\n\nOur aim is a deterministic compiler.\n\n## Principles\n\nStructure to the CLI.\n' > VISION.md
git add -A && git commit -q -m 'foreign VISION.md'
$JIGC migrate VISION.md --as vision                 # task minted: migrate-vision-vision-cd709ad9fcd8
…author vision, set commit type/summary…
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/VISION.md"
echo NO >&2
exit 1
HOOK
$JIGC task finalize migrate-vision-vision-cd709ad9fcd8 --approve ; echo $?
# -> 1
# apart from the 1 path named below, which survived the rollback: …
# blocking · finalize.rollback-conflict — `VISION.md` changed while this finalize was running, so
#   the rollback did not restore it: the bytes on disk are not the ones jigc wrote
#   at: VISION.md
#   route: … this finalize's pre-image at
#          `.jigc/displaced/finalize/VISION.md.pre-image.1789964382258655000` …
grep -c 'raced by a concurrent editor' VISION.md  # -> 1     (M51: 0 — the bytes were GONE)
find .jigc/displaced -type f                      # -> .jigc/displaced/finalize/VISION.md.pre-image.…
git log --oneline -1                              # -> 55d209c foreign VISION.md   (unmoved)
```

*Retire half* (rig `fresh`, foreign at `direction/plan.md`, destination `VISION.md` new; hook
re-creates the retired original):

```
$JIGC task finalize migrate-vision-direction-plan-0410d8e1466d --approve ; echo $?
# -> 1 · blocking · finalize.rollback-conflict — `direction/plan.md` changed while this finalize was
#        running, so the rollback did not restore it: jigc removed it and something has since put a
#        file back
cat direction/plan.md          # -> "# recovered by a concurrent editor"   (racer's file stands)
find .jigc/displaced -type f   # -> .jigc/displaced/finalize/direction/plan.md.pre-image.…  (dir kept)
ls VISION.md                   # -> No such file or directory   (the promote correctly undone)
git log --oneline -1           # -> efb5f3e foreign direction/plan.md      (unmoved)
```

Agrees with the driver's C1 / DEFECT-2-both-halves verdict.

### CL-2 · M51 C2 — `RecordPreImage`'s unconditional restore is CLOSED; the registry carries all five callers (create, add-task, add-from-spec, milestone discard, sub-task discard) as one `FileCas` population

**CONFIRMED — four of the five callers driven by me, the fifth (`add-from-spec`) by the driver
(B10) and left at that.**

```
rig=$(dev/jigc-rig committed-singletons …) ; eval "$rig"
$JIGC milestone create 'Axis four milestone'
REC=docs/milestone-records/axis-four-milestone.md
cat > .git/hooks/pre-commit <<HOOK
#!/bin/sh
printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/$REC"
echo NO >&2
exit 1
HOOK

$JIGC milestone add-task axis-four-milestone 'axis four sub task' ; echo $?
# -> 1 · blocking · milestone.rollback-conflict at docs/milestone-records/axis-four-milestone.md
#        frame: "apart from the 1 path named below, which survived the rollback: …"
#        route: … pre-image at `.jigc/displaced/milestone-op/docs/milestone-records/….pre-image.…`
grep -c 'raced by a concurrent editor' $REC          # -> 1      (M51: 0)

$JIGC task discard axis-four-sub-task --force       # -> 1 · task-discard.rollback-conflict
find .jigc/displaced -maxdepth 1 -type d            # -> .jigc/displaced/task-discard   (its OWN park dir)

$JIGC milestone discard axis-four-milestone --force # -> 1 · milestone.rollback-conflict
                                                    #    racer line survives; pre-image parked

# the pre-image-ABSENT arm: hook CREATES the record path a fresh `create` is about to write
$JIGC milestone create 'Mcreate'                    # -> 1 · milestone.rollback-conflict
cat docs/milestone-records/mcreate.md               # -> "racer created this"  (NOT deleted)
find .jigc/displaced -type f                        # -> (none — correct: there is no pre-image to park)
#  route: "…did not exist before this milestone-op, so the rollback would have deleted the copy
#          jigc created — it did not."
```

`task discard` raising its **own** `task-discard.rollback-conflict` into its **own** park directory
confirms the fifth-door claim the driver files at B9.

### CL-3 · M51 C3 — `RecordFlipGuard::Drop`'s unconditional restore is CLOSED; the registry identifies both milestone-finalize commit models through the `fan-out-record-flip` `FileCas` row

**CONFIRMED — both commit models driven.** Fixture: `committed-singletons` → `milestone create
'Flip probe'` → `add-task` → `provision` → in the worktree `jigc workflow sub-task --task
flip-sub-task` then author the commit doc → `milestone join`. Hook appends to
`docs/milestone-records/flip-probe.md` and exits 1.

```
# squash: true (pack default)
$JIGC milestone finalize flip-probe ; echo $?
# -> 1 · milestone.rollback-conflict
#    state clause: "the merged docs were rolled back"
grep -c 'raced by a concurrent editor' docs/milestone-records/flip-probe.md   # -> 1
find .jigc/displaced -type f   # -> .jigc/displaced/milestone-op/…/flip-probe.md.pre-image.1789964430213308000
git log --oneline -1           # -> 201c190 chore(milestone): record task:flip-sub-task …  (unmoved)

# squash: false
$JIGC config set finalize.fan-out.squash false
$JIGC milestone finalize flip-probe ; echo $?
# -> 1 · milestone.rollback-conflict
#    state clause: "HEAD is at its pre-finalize commit"     (the arm's own frame — correctly different)
grep -c 'raced by a concurrent editor' docs/milestone-records/flip-probe.md   # -> 1
find .jigc/displaced -type f   # -> …/flip-probe.md.pre-image.1789964473296762000
```

**Instrument note, recorded because it cost a run and would cost the next reader one:** on the
`squash: false` arm the flip cell is **unreachable** until each sub-task has authored its commit doc
— the chain path refuses first with `blocking · finalize.render-io — no commit doc for sub-task
flip-sub-task`. My first `squash: false` attempt exited 1 on *that*, with no conflict and no racer
line, which reads exactly like a regression if the fixture is not completed. The driver's R-C
fixture line (*"author an adr in the worktree"*) is what supplies it.

### CL-4 · M51 C4 — `milestone provision`'s unacknowledged early `.gitignore` mutation is CLOSED: the guards precede `gitignore::ensure`, and a successful amendment rides the ack

**CONFIRMED on the arm a rig reaches; one sub-leg OPEN.**

```
printf 'tasks/\n# my private line\n' > .jigc/.gitignore ; cp .jigc/.gitignore "$RIG/ign2.txt"
$JIGC milestone provision nonexistent-milestone ; echo $?
# -> 1 · blocking · milestone.unknown — milestone `nonexistent-milestone` does not exist
#        route: create it first with `jigc milestone create "<title>"`
diff "$RIG/ign2.txt" .jigc/.gitignore      # -> rc 0 — BYTE-IDENTICAL (the guard precedes the ensure)

# the successful control still amends AND still acks, on its own line:
$JIGC milestone provision prov-probe
# -> provisioned 1 worktree(s) for milestone:prov-probe at base 56a4248 (prov-sub-task)
#    .jigc/.gitignore → appended index/, state/, milestones/, worktrees/, logs/, displaced/
#      (jigc's transient-runtime entries; every line already in the file was kept)
```

**OPEN sub-leg (reason stated, not promoted):** Codex's ordering claim also names *"rejects a stale
base"* as a guard that now precedes the `ensure`. I could not reach a stale-base refusal: after
committing a drift commit past the record commit, `milestone provision prov-probe` exited **0** and
provisioned at the **pinned** base. So the refusal exists in the source ordering but I have no rig
state that fires it, and the ordering leg is confirmed only through the `milestone.unknown` guard.
This is the same hole the driver records at §5 item 7 (*"I could not construct a reaching failure
and say so rather than claiming the cell closed"*), reached independently.

**Scope note that matters for CL-0:** CL-4 is about **provision**. The driver's STILL-OPEN half is
about **`milestone create`**, a different call site. The two do not contradict; CL-4 is confirmed
*and* DEFECT 3 stands.

### CL-5 · `ROLLBACK_POPULATIONS` enumerates eleven populations with explicit `FileCas` / `MintedSet` / `DoorGuard` / `Declared` discipline

**CONFIRMED — source-datum.** `grep -c '^        id: "' crates/cli/src/rollback.rs` → **11**, at
`:171 :191 :212 :231 :258 :276 :310 :331 :351 :365 :391`, ids exactly the eleven both passes name
(`config-layer-worktree`, `promote-destination`, `retired-original`, `milestone-record`,
`fan-out-record-flip`, `rename-worktree`, `created-doc-staged-write`, `milestone-mint-area`,
`unrecorded-seed-areas`, `config-root-relocation`, `setup-install-path`). Agrees with the driver's
§1 count.

### CL-6 · the shared finalize closure captures owner-artifact, promotion, config-layer, milestone-record index and config-worktree pre-images **before** promotion, retirement, ignore amendment, staging or commit; its failure arm restores all five and carries conflicts

**CONFIRMED — source-datum for the five axes, driven for the worktree axis.** Source, `task.rs`
Phase 4a: `capture_owner_artifact_index` (owner), `capture_owner_artifact_index` (promotions),
`capture_config_layer_index`, `capture_owner_artifact_index` (record paths), and
`capture_config_layer_worktree` — all five assigned before the promote/retire/`gitignore::ensure`
(`task.rs:3994`)/stage/commit sequence, with the in-file comment stating *"Every index axis's
capture precedes every failure point"*. Driven on the worktree axis:

```
printf 'tasks/\n# my private line\n' > .jigc/.gitignore       # trimmed, so the amend genuinely WRITES
hook: printf '# raced\n' >> "$REPO/.jigc/.gitignore" ; exit 1
$JIGC task finalize axis-four-probe --format json > o.json 2> e.json ; echo $?
# -> 1 ; stdout = 0 bytes
# e.json parses as EXACTLY ONE document:
#   top keys: ['findings', 'schema_version']
#   codes:    ['finalize.commit-rejected', 'finalize.rollback-conflict']
#   keys:     [{'code':'finalize.commit-rejected','target':'task:axis-four-probe'},
#              {'code':'finalize.rollback-conflict','target':'.jigc/.gitignore'}]
grep -c '# raced' .jigc/.gitignore   # -> 1   (the racer's line stands)
find .jigc/displaced -type f         # -> .jigc/displaced/finalize/.jigc/.gitignore.pre-image.…
```

This also independently re-confirms the driver's **M51 DEFECT 1 → CLOSED** (one JSON document,
both findings, non-null `(code, target)` on each).

### CL-7 · `COMMITTING_DOORS` contains the ten hook-capable identities; `setup` is deliberately outside because its install commit uses `--no-verify`

**CONFIRMED — source-datum (10) + driven (the exemption).**
`awk '/pub const COMMITTING_DOORS/,/^\];/' … | grep -c 'CommittingDoor {'` → **10**, with
`milestone finalize` carrying both `(squash: true)` and `(squash: false)` identities. Driven on rig
`bare` with a rejecting `.git/hooks/pre-commit` in place:

```
git log --oneline -1     # -> 8331b1c initial
$JIGC setup ; echo $?    # -> 0
git log --oneline -1     # -> 0c5de53 chore(jigc): install jigc workspace config   (HEAD MOVED)
```

The exemption is real and behaves as declared. Agrees with the driver's A4.

### CL-8 · all four production `gitignore::ensure` writers remain setup/adapter, milestone create, milestone provision, shared finalize

**CONFIRMED — source-datum, with the test-side site excluded on inspection.**
`grep -rn 'gitignore::ensure' crates/cli/src/*.rs` yields call sites at `adapter.rs:1115`,
`milestone.rs:657`, `milestone.rs:2732`, `task.rs:3994` — four — plus `relocate.rs:1222`, which is
inside that file's `mod tests` (its neighbours are `fn git_ignores(repo: &TempRepo, …)` and
`#[test] fn a_foreign_squatter_at_the_destination_moves_into_the_workbench_and_the_managed_lands`),
so it is not a production writer. Matches the driver's `IGNORE_DOORS` count of 4.

**This claim is the one that carries CL-0's refutation:** the four writers are correctly enumerated
and *two* of them (`milestone create`, `milestone provision`) sit outside the finalize executor's
capture family — which is a completeness fact about the enumeration, and a **gap** about the
rollback.

### CL-9 · `TASK_AREA_FILES` contains thirteen root entries, with a separate staged-doc rule, including the cross-area record commit message

**CONFIRMED — source-datum.** `crates/engine/src/state.rs:129` → thirteen members
(`BASE_PIN_FILE`, `DOCS_DIR`, `INTENT_FILE`, `RENAMES_FILE`, `ROLES_FILE`, `SLUG_OVERRIDE_FILE`,
`SOURCE_FILE`, `SOURCE_PATH_FILE`, `STAGED_SNAPSHOT_FILE`, `WORKFLOW_FILE`,
`milestone::RECORD_COMMIT_MSG_FILE`, `validate::BASE_SNAPSHOT_FILE`, `validate::SNAPSHOT_FILE`),
`DOCS_DIR` carried as a tree with `TASK_DOCS_FILES` as its own rule, and `RECORD_COMMIT_MSG_FILE`
present on this row as well as the milestone row exactly as the claim states.

### CL-10 · `STORE_EXIT_FLIPS` has the seventh `home-vacated` member; `ENVELOPE_OWED_CODES` includes all four targeted codes

**CONFIRMED — source-datum + driven at two doors.**
`STORE_EXIT_FLIPS` → **7** rows, the seventh `id: "home-vacated"` (`render.rs:1093`) matching on
`crate::orphan::HOME_VACATED_CODE`. `ENVELOPE_OWED_CODES` (`render.rs`) → exactly four:
`engine::store::NOT_FOUND`, `engine::store::NO_SUCH_LEAF`, `engine::store::UNKNOWN_TYPE`,
`crate::task::FIXED_IDENTITY`. The driver drives the behavioural half at K1–K4 (envelope) and
K5–K7 (the declared flattened complement); I did not re-drive those rows, and they carry argv and
observed arm each.

### CL-11 · zero schema-hash boundary movement since M51's `577a0099` — no shipped schema YAML and neither manifest changed

**CONFIRMED — source-datum.**
`git diff --stat 577a0099..HEAD -- '*schema*.yaml' crates/cli/pack/config/schema-manifest.yaml packs/methodology/config/schema-manifest.yaml crates/cli/pack/doctypes packs/methodology/doctypes` → **empty**, and
`git diff --name-only 577a0099..HEAD | grep -E '\.ya?ml$' | grep -iE 'schema|doctype'` → **empty**.
M52's declared boundary held.

---

## Reconciliation ledger — driver defects

### DEFECT 1 · `finalize.stage-failed`'s route is not copy-runnable — **STANDS** (re-driven)

The source pass is **silent** on route text (its hunt is capture/restore completeness), so the rule
keeps this a finding; re-driven here:

```
rig=$(dev/jigc-rig committed-singletons …) ; eval "$rig"
$JIGC start --workflow quick-fix "axis four probe"
echo x > touched.txt ; git add -A
$JIGC doc set-field 'commit:axis-four-probe#header/type' --task axis-four-probe --value chore
printf 'axis four probe' | $JIGC doc set-slot 'commit:axis-four-probe#summary' --task axis-four-probe --from-file -
: > .git/index.lock
$JIGC task finalize axis-four-probe ; echo $?
# -> 3
# blocking · finalize.stage-failed — jigc could not stage its own changes — no commit was made and
#   the promotions were rolled back: `git add -- .jigc/config .jigc/.gitignore .jigc/version` failed:
#   fatal: Unable to create '…/.git/index.lock': File exists. …
#   at: task:axis-four-probe
#   route: resolve the embedded git failure (e.g. remove a stale `.git/index.lock`), then re-run
#          `jigc task finalize`
rm -f .git/index.lock
$JIGC task finalize ; echo $?          # the route, followed verbatim
# -> 2
# error: the following required arguments were not provided:
#   <ID>
# Usage: jigc task finalize <ID>

# the SAME door's hook-rejection frame, same run shape, for contrast:
$JIGC task finalize axis-four-probe 2>&1 | grep -o 're-run `jigc task finalize[^`]*`'
# -> re-run `jigc task finalize axis-four-probe`
```

Exit **2** captured bare, not through a pipe. Confirmed at the driver's severity (LOW, surface/route
tier — the restore itself is correct and the state clause is true).

### DEFECT 2 · `config.repoint-failed` renders an absolute host path in the pinned envelope — **STANDS** (re-driven, and *reached with a smaller fixture than the block claims*)

Source pass silent. Re-driven — note I did **not** need R-G's *"land a committed adr through
record-decision"* preamble; the two committed docs the `committed-singletons` rig already carries
are enough, so the block is over-specified rather than under-specified:

```
rig=$(dev/jigc-rig committed-singletons …) ; eval "$rig"
git ls-files docs                 # -> docs/decisions-log.md, docs/roadmap.md
chmod 0555 .jigc/config           # make write_scalar fail AFTER the git mv batch lands
$JIGC config set docs-root documentation --format json > o.json 2> e.json ; echo $?
chmod 0755 .jigc/config
# -> 1 ; o.json is 0 bytes ; e.json is ONE {findings, schema_version} document:
# "code": "config.repoint-failed",
# "key": {"code":"config.repoint-failed","target":".jigc/config/manifest.yaml"},
# "message": "`docs-root` was not set to `documentation`: could not write
#             /private/var/folders/nj/…/jigc-rig-committed-singletons-BGBfFz/repo/.jigc/config/manifest.yaml:
#             Permission denied (os error 13) — the re-point was undone",
# "location": {"address": ".jigc/config/manifest.yaml", "line": 1, "col": 1}
# "route": "`docs-root` is unchanged and every doc this re-point moved is back at its prior home. …"
git status --short                # -> EMPTY
git ls-files -s docs              # -> same two blob shas as before the run
ls documentation                  # -> No such file or directory
$JIGC config get docs-root        # -> docs-root = docs/  (pack-default)
```

The **rollback is clean**; the defect is the message alone, and the repo-relative spelling is in the
same finding's `location.address`. Confirmed at LOW.

**A reconciliation datum that sharpens this finding rather than widening it.** While driving CL-3 I
hit a second absolute-path render — `blocking · finalize.render-io — no commit doc for sub-task
flip-sub-task … at: /private/var/folders/…/repo/.jigc/tasks/flip-sub-task/docs/commit:flip-sub-task.md`.
That one is **already declared**: `crates/cli/tests/repo_relative_paths.rs`' `UNSWEPT_PRODUCERS`
carries `("crates/engine/src/finalize.rs", 11, "five I/O-fault finding helpers (`provenance_io`,
`promote_io`, `source_path_io`, `task_missing`, `render_io`) name a task dir or a staged doc in
message, route AND `file_location` locus; none of them is handed a repo root, so closing them means
threading one through the finalize plan — its own increment, not a triage fix")`. So it is **not a
new defect** — and the contrast is exactly what makes DEFECT 2 a defect: the `finalize.rs` row
**admits** its sites are finding surfaces and prices the fix, while the `config.rs` row asserts
*"an error channel, **not a finding surface**"* — a clause the driven envelope above falsifies.

### DEFECT 3 · a hook-rejected `milestone create` leaves its `.jigc/.gitignore` amend on disk, names it nowhere, and says *"nothing … survives"* — **STANDS**, and it is the **refutation of the source pass's top-line claim**

Codex does not say this cannot happen; it says *nothing of this class exists*. Driven above (CL-0),
with the source site (`milestone.rs:657`, no capture in the window) and the registry gap
(`config-layer-worktree.doors` = `task finalize` ∪ `milestone finalize`) both read at `HEAD`.
Confirmed at the driver's severity (LOW — law-1 disclosure; the amend is a **union**, the user's own
lines are kept, verified in my own run: `# my private line` survives, the six entries are appended).
Scope kept as the driver has it: the **guard-refusal** arm is genuinely closed at both doors (CL-4),
so what remains is the **hook-rejected** arm.

### Driver's six CLOSED M51 halves — **all six independently re-driven here and CONFIRMED CLOSED**

C1 promote (CL-1), C1/DEFECT-2 retire (CL-1), C2 (CL-2), C3 both arms (CL-3), C4's provision/guard
half (CL-4), DEFECT 1's JSON stream (CL-6). No driver CLOSED verdict was contradicted by either the
source pass or by my re-drive.

### Driver's OBSERVATION 1 (rolled-back `doc author` keeps provenance + role binding) — **carried unchanged**

Not driven again. Codex is silent on it; the driver files it as an observation rather than a defect
with two checked reasons (no dead end, and the orphaned binding is M48's own on-the-record subject).
Nothing in this reconciliation bears on it, so it stays as filed and is **not** promoted.

---

## Open leads

1. **CL-4's stale-base ordering leg.** Codex cites `run_provision` rejecting a stale base before
   `gitignore::ensure`. I drove a HEAD-moved-past-base state and `milestone provision` exited **0**
   at the pinned base, so no rig I have fires that refusal. The claim is neither confirmed nor
   refuted on behaviour — only the `milestone.unknown` guard leg is driven. **Reason it stays open:**
   no corpus state builds a stale base, and I will not promote an ordering claim on a source read.
   Same hole the driver records at §5 item 7.
2. **Cell D in full (stage failure × worktree concurrently edited), all 15 rows.** Both passes and
   M52's own VERDICT reach the same conclusion — no deterministic in-transaction racer exists before
   the stage. Stays an OPEN lead rather than an `n/a`, because *no instrument exists* is a statement
   about the fixture library, not about the binary.
3. **A genuine concurrent *process* racing a pre-image** (as opposed to the `pre-commit` hook, which
   is the instrument `design/finalize.md` names). Not driven by either pass; it would test the
   scheduler.
4. **`milestone provision`'s post-`ensure` failure point.** M52's T9 moved the `ensure` behind the
   guards; whether any failure point now lies *after* it at that door is unestablished. I probed the
   unknown-milestone and HEAD-drift arms (exit 1 pre-write, exit 0 respectively) and reached neither.

---

## Doors covered

Every clap leaf that is the door of ≥1 **driven** row (driver's table ∪ my re-drives), in
`VERB_KINDS` spelling — **16 leaves**, which matches the driver's §8 figure:

| leaf | driven rows |
|---|---|
| `task finalize` | A1 · B1–B5 · B19 · C1 · F1 · G1 · G3 · H1–H5 · I1 · I2 · M1 · (recon: CL-1 ×2, CL-6, DEFECT 1) |
| `milestone finalize` | B4 · B11 · B12 · F2 · (recon: CL-3 both arms) |
| `milestone create` | A3 · B7 · B16 · F5 · (recon: CL-0/DEFECT 3, CL-2 absent-pre-image arm) |
| `milestone add-task` | B6 · B17 · F6 · (recon: CL-2) |
| `milestone add-from-spec` | A6 · B10 · B18 · K3 · K4 |
| `milestone discard` | B8 · (recon: CL-2) |
| `milestone provision` | A5 · (recon: CL-4 both arms) |
| `milestone join` | C4 · C4b · (recon: CL-3 fixture) |
| `task discard` | B9 · (recon: CL-2, own conflict identity) |
| `task bind` | K5 |
| `rename` | B13 · B14 · B15 · F3 · K1 · K2 · K6 · K7 · (recon: R-I(a), R-I(b)) |
| `migrate-corpus` | A2 · C2 · F4 |
| `migrate` | E2 |
| `setup` | A4 · C3 · C3b · (recon: CL-7 `--no-verify` exemption) |
| `config set` | C5 · L1–L5 · (recon: DEFECT 2) |
| `doc author` | J1 |

**Not a door of any driven row** (recorded so a silent omission is distinguishable from a
deliberate one): `milestone execute`, `milestone list-tasks`, `uninstall`, `upgrade`, `ingest`,
`unmanage`, `relocate`, `describe`, `validate`, the nine `doc` read/write leaves other than
`doc author`, `task list` / `task diff` / `task validate`, the seven `config` leaves other than
`config set`, `start`, `workflow`. Of these, `start`, `workflow`, `doc set-field`, `doc set-slot`,
`doc schema`, `config get`, `task list` and `milestone execute` were **run as fixture
construction** in my re-drives; none is a row of this axis, and none is counted as covered.
