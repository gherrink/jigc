# The rc.24 fix pass — findings ledger

**Drafted 2026-10-04 on `fix/rc24-tier1` at `5f5b273a`, while fix round 4 was running; brought to the branch tip on 2026-10-05, at `0f34d8f0` (55 commits over `origin/main`), as the session that ran the pass closed.** This is an **index**: every finding of the gate before 1.0.0 once, with where it is recorded in full, what it is in one line, and its disposition. It grades nothing, re-tiers nothing and proposes no fix. The body of each finding stays in the file named for it.

**Read this first if you are picking the pass up:** round 4 and the last fix (`676eff57` … `0f34d8f0`, 36 commits) were **audited by no one** — the completion audit ran at `5f5b273a`. A row that reads `FIXED` below says a commit exists whose message and diff claim the fix and whose fixer reported a green full gate; it does not say an independent reviewer has looked. What is open is the closing section, [Open at the close of the session](#open-at-the-close-of-the-session).

## How to read it

**Sources, and what `<scratch>` means.** `<scratch>` is the session scratchpad the fix pass ran in; nothing under it is committed, and it does not outlive the session. The files this ledger indexes from there are copied, sanitized, beside it and committed with it: `audit/`, `fixers/`, `planning/`, `perf/` (→ [README.md](README.md) → Files). Where a row below says *scratch only*, read: *in the copy beside this ledger, and in no other committed file*.

| table | source | committed? |
|---|---|---|
| **A** | `completions/artifacts/M55/per-axis-review-rc24/README.md` (the partial re-review) | yes |
| **B** | `completions/artifacts/RC-rc24/README.md` (the blind trial) | yes |
| **C** | `audit/audit-findings.md` — the completion audit of the pass at `5f5b273a`, 54 findings | the copy beside this ledger |
| **D** | `fixers/fixer-reports.md`, `fixers/fixer-reports-round3.md` — fifteen fixers' `left_open` lists (rounds 1–3) | the copy beside this ledger |
| **E** | `planning/plan-*.md`, `planning/advocate-*.md` | the copy beside this ledger |
| **F** | `fixers/fixer-reports-round4.md`, `fixers/fixer-report-witness.md` — round 4's six area fixers and the last fix: their `left_open` lists | the copy beside this ledger |

**Disposition vocabulary.**

- `FIXED <sha>` — a fix commit of this pass closed it. Checked against the commit message (`git show --stat <sha>`) and, where the row names a symbol, the tree — at `5f5b273a` for rounds 1–3 and at `0f34d8f0` for round 4 and the last fix — unless the row says *as reported*. **For round 4 and the last fix `FIXED` is unaudited** (above).
- `RECORDED by the commit that carries this ledger` — a records finding (the logs lagged the code) discharged by the closing record commit: the `DECISIONS.md` entries of 2026-10-05 and the rows in `implementation/decisions-pending.md`.
- `CLOSED BY DECLARATION <sha>` — the commit states the behaviour as a bound instead of changing it.
- `OPEN — port` — short for: *OPEN — tier 2/3, recorded in the review (or trial) README, filed through jigc at the port (M56).*
- `OPEN — changed by the pass` — still open, and the pass moved what the row describes; the row says how.
- `DEFERRED — M57 "<row>"` — a trigger row exists in `implementation/decisions-pending.md` → *M57 — the 1.x fix pass* (sixteen rows at `614d9c99`; read at lines 77–92).
- **Round 4's areas** (table C's dispositions and table F's ids use them): **A** eol comparisons (`676eff57`) · **B** linked-worktree guard (six commits) · **C** milestone boundary (five) · **D** baseline record (four) · **E** setup/uninstall (nine) · **F** worktree doors (ten). The round-4 report numbers them *Area 1* … *Area 6* in the same order. The draft's `IN ROUND 4 (area X) — status pending` is replaced throughout table C by what the round did.
- `OPEN` in table C — an audit finding no round took, or took in part.
- `COVERED BY <what>` / `NOT RECORDED ANYWHERE ELSE — OPEN` — tables D and E: whether anything outside the fixer's or planner's own scratch file carried the item **when the table was written, at `5f5b273a`**. Those two tables are kept as written; a row that round 4, the last fix or the closing record commit changed carries a trailing **At the close:** note. Every `NOT RECORDED ANYWHERE ELSE` item now has this committed ledger as its home, and the closing section says where each stands.

**"A report is a lead."** Where this ledger says `FIXED` or `COVERED BY`, the claim was checked against `git show`, the tree, `DECISIONS.md`, `decisions-pending.md` or the committed READMEs. Where it could not be checked the row says **as reported**. Nothing in tables C, D, E or F was re-driven for this ledger: no binary was run to write it.

**Three facts about the record that bear on every table below** (checked with `git log origin/main..HEAD` and `git diff --stat origin/main..HEAD` at the close).

1. **The decision log lagged the code twice, and the closing record commit is what brings it level.** The last record commit before it, `614d9c99`, precedes all six round-3 fix commits, all thirty-five of round 4 and the last fix. Until the closing commit, `DECISIONS.md`'s top entry said of five queued members *"None is built at this entry"* (all five are built), and none of those forty-two SHAs appeared in `DECISIONS.md`, `implementation/decisions-pending.md` or `implementation/project-history.md`. The closing commit adds the as-built entries, the exit rule's revision, the audit's result and the rows for what is open. A `COVERED BY DECISIONS.md` in tables D and E still means an entry written at or before `614d9c99`.
2. **No fix commit touched `completions/`**, and `implementation/project-history.md` has no fix-pass span — it is written when the pass lands, and is owed. The row annotations the plans and fixers said the two rc.24 records were owed (a `pinned-by:` for `(R6, K-1)`, the now-driven cells of `(R6, D-7)`, the fixed rows marked as fixed) **are not made**; table A and table B are where a reader learns a committed row is closed.
3. **No testing round ran after round 4.** No auditor, no re-review row, no trial arm and no end-to-end pass has seen `676eff57` … `0f34d8f0`; each fixer ran the full gate before each commit and drove its own printed routes, and that is all.

## A. The review's findings

Full rows, repro blocks, `contract:` and `pinned-by:` fields: `completions/artifacts/M55/per-axis-review-rc24/README.md` → *FINDINGS* (and `row-<n>.md`, `tier1-verification/`). The README counts **65 finding rows: 6 tier-1 · 7 tier-2 · 52 tier-3**; the adversarial re-drive upheld five of the six and demoted `(R6, K-1)`.

### A.1 Tier 1 as assembled (6)

| key | tier | door | title | disposition |
|---|---|---|---|---|
| `(R1, F1)` | 1, upheld | `setup` | on an unborn `HEAD`, `setup` overwrites untracked user bytes at a whole-rewrite install path at exit 0 | **FIXED `dc0d7586`**; class siblings folded in: `104a7d4b` (a symlink at a replacing writer, and the stamp at `task finalize`), `ef9456b5` (a gitignored or index-flagged install path). The audit found cells still open or regressed in those fixes (C: install F1, F2, F3, F5, F8 · XC-2 · E2E F2); round 4 took all of them: `676eff57` · `f0b1120a` · `33620081` · `a330b4f9` (stated unsupported, not fixed) · `cba98c44` · `55cb29fa` — **unaudited** |
| `(R3, F7)` | 1, upheld | `task finalize` | with no file-state baseline, a hand edit made after the task's first write is overwritten at exit 0 | **FIXED `eb18e5fe`**; siblings: `8d9c3afb` (the milestone-record door), `e3a6ba58` + `a88f71cc` (the linked-worktree sibling). The audit found three sequences that still lost the edit and one false block (C: reconcile F1–F4 · E2E F3, F5): the false block and the migration source went in round 4 (`676eff57`, `4ba04efc`); the three losses needed a record the task area did not hold and were closed by **the copied-in witness, `0f34d8f0`** (the human's ruling of 2026-10-05, option A) — **unaudited** |
| `(R6, D-1)` | 1, upheld | `milestone finalize` (`milestone join` mints the id) | the join's collision suffix lands on an id already on disk and the boundary overwrites the doc there | **FIXED `3f3a724b`**. The audit found two cells of the class still losing at exit 0 and a dead route (C: CPL-1, CPL-2, CPL-3); round 4: `f031d31e` · `048724d0` · `d724365f` — **unaudited** |
| `(R6, D-7)` | 1 on the harm disjunct, upheld | `task finalize`; `doc create` passes the link | a dangling symlink at the minted home: finalize exits 0 committing the link, the finding lands untracked | **FIXED `9465f9b6`**; the in-place store doors (`rename`, the relocation primitive, the record's later writers): `5f5b273a`. Audit → C: CPL-4, CPL-6, CPL-7, CPL-8; round 4: `685a4c55` · `526141e6` · `c97b8eb6` · `242341bb` — **unaudited** |
| `(R6, K-1)` | split 1 / 2 by the reconciler → **not tier 1** by the re-drive (tier 3 with a tier-2 tail); recorded as tier 2 in `DECISIONS.md` | `doc create` · `doc author` | the create-only gate is check-then-use | **FIXED `c0c4d88c`** (the one-probe fix, folded in). The class behind it: **DEFERRED — M57 "The external-writer race class"** |
| `(R9, F5)` | 1, upheld | `uninstall` | without `--force`, `uninstall` destroys files in `.jigc/logs/`, `state/`, `index/` at exit 0, named by nothing | **FIXED `12398ddc`**; the refused teardown no longer re-mints the log: `4fa1c0f5`. Audit → C: install F4, F6 · XC-5, XC-7; round 4: `528d0205` · `1b1d7656` · `8b214ad5` · `dcfa40f0` — **unaudited** |

The sixth verified tier-1 row is the trial's `L-22` → table B.

### A.2 Tier 2 (7)

| key | tier | door | title | disposition |
|---|---|---|---|---|
| `(R3, F4)` | 2 | `task finalize` · `milestone add-task` | a pulled edit to the milestone record is absorbed by the next unrelated finalize; every milestone door then answers `milestone.terminal` | OPEN — port. **Deliberately left by `8d9c3afb`** (its commit message: a pulled edit *"is adopted as before"*; its fixer: *"TIER 2, unchanged"*; a control test pins it, as reported) |
| `(R4, F2)` = `(R6, D-8)` | 2 by row 4 · 3 by row 6 | `start` (also `workflow`, `describe`, `migrate`, `doc create`, `doc author`) | the malformed-front-matter refusal names the key and not the workflow; line number one short | OPEN — port |
| `(R4, F3)` | 2 | `relocate` | `relocate.frozen-doctype`'s route is a no-op for a frozen doctype with no versioned snapshot | OPEN — port |
| `(R5, F1)` | 2 | `task finalize` | a formatter-style `pre-commit` hook makes the doc-only commit leave a staged reversion of its own doc | OPEN — port |
| `(R5, F6)` | 2 | `task finalize` | two more finalize routes end in a bare `jigc task finalize` | OPEN — port |
| `(R6, D-2)` | 2 | `doc create` → `doc rename` → `rename` | `write.identity-change`'s route names a `doc rename` that refuses | OPEN — port |
| `(R6, D-3)` | 2 | `doc create` · `task finalize` | two open report tasks mint one id; the second's `create.already-exists` route names an exit that refuses and one that destroys | OPEN — port (`c0c4d88c` reuses the same route builders, as its fixer reports — not re-driven) |

### A.3 Tier 3 (52)

| key | door | title | disposition |
|---|---|---|---|
| `(R1, F2)` | `setup` | `setup --force` over a dirty hook spends the consent without naming it | OPEN — port (the `dc0d7586` fixer: *untouched*) |
| `(R1, F3)` | `setup` · `uninstall` | a standalone jigc hook from another path / template / with one added line is wrapped as foreign; `uninstall` leaves a jigc hook behind | OPEN — port. Neighbour, not the same cell: C install F4 |
| `(R1, F4)` | `setup` (the hook) | a `jigc` path containing a single quote installs a hook with broken quoting | OPEN — port |
| `(R1, F5)` | `setup` (the hook) | from a linked worktree the hook's sweep asks about the main checkout | OPEN — port (the `e3a6ba58` fixer: *unchanged*); carried as open question 3 of `ideas/linked-worktree-doc-work.md` |
| `(R2, T-1)` | `validate` | the probe's 30 s budget is not enforced when the killed probe leaves a child holding its pipes | OPEN — port |
| `(R2, P-1)` | `task validate` (+3) | an override that names no file takes two shapes at the task and milestone doors | OPEN — port |
| `(R2, L-1)` | `start` | orientation surfaces findings and its log record says `finding_codes: []` | OPEN — port |
| `(R2, L-2)` | `validate` | `measurement.md` says `output_bytes` is stdout; the binary records stdout plus stderr | OPEN — port |
| `(R3, F1)` | `validate` → `rename` | the strong-signal route's *adopt* exit cannot be run | OPEN — port |
| `(R3, F2)` | `ingest` | code-less OS error when a tracked `.md` is missing from the worktree | OPEN — port |
| `(R3, F3)` | `doc set-slot` | `write.non-reparseable` says *nothing was persisted* while the copy-in is persisted | OPEN — changed by the pass: since `eb18e5fe` the copy-in that survives a refused write also records a baseline; the fixer reports the bound stated in `design/command-output-contract.md` (*as reported*) → D `eb18e5fe`-5 |
| `(R3, F5)` | `unmanage` | matches its path literally and reports a managed doc as *not managed* | OPEN — port |
| `(R3, F6)` | `task amend` | a compose-gate refusal leaves a minted live task; *then re-run* collides | OPEN — port |
| `(R4, F1)` | `migrate` | leaves a minted task behind a refusal that exits 1 | OPEN — port |
| `(R4, F4)` | `relocate` | `relocate --help`'s example doctype is one the door refuses | OPEN — port |
| `(R4, F5)` | `migrate-corpus` | commits an uncommitted edit on a doc it migrates, un-narrated | OPEN — port |
| `(R4, C1)` | `migrate-corpus` | a filesystem pack's freeze manifest that vanishes mid-load is skipped silently | OPEN — port |
| `(R5, F2)` | `task finalize --dry-run` | a C-quoted path reaches the manifest quoted; forecast disagrees with the door | OPEN — port |
| `(R5, F3)` | `task finalize --dry-run` | forecast lists an untracked recorded owner-artifact as left out; the door commits it | OPEN — port |
| `(R5, F4)` | `task finalize` | a `git rm --cached` path appears twice in the doc-only left-out list | OPEN — port |
| `(R5, F5)` | `task finalize` | `finalize.stage-failed` says promotions were rolled back beside a `rollback-conflict` that says one was not | OPEN — port |
| `(R5, F7)` | `task finalize` · `milestone provision` | absolute host paths inside the repository, in a route and a message | OPEN — port. The audit saw two more producers → C E2E F9 (1), (2) |
| `(R5, F8)` | `milestone provision` | failing after its `ensure` leaves the `.jigc/.gitignore` amend on disk, unnamed | OPEN — port |
| `(R5, F9)` | `config set` | a rolled-back `config set docs-root` leaves the new root's empty directories behind | OPEN — port. Seen again behind the pass's new refusal → C CPL-9 (d) |
| `(R5, F10)` ⊂ `(R9, F1)` | `task finalize` · `task validate` | the help describes two commit models; the binary has three | OPEN — port |
| `(R6, D-4)` | `doc create` · `doc author` | `write.title-ignored` arises under a `new: true` entry | OPEN — changed by the pass: `c0c4d88c` turns the pre-check's **committed-incumbent** arm under `new: true` into `create.already-exists`; the **staged-only** arm this row drove is not claimed by the fixer and was not re-driven |
| `(R6, D-5)` | `start` | the bare `allows-create` entry form `write-commands.md` documents is refused at load | OPEN — port |
| `(R6, D-6)` | `doc create` → `task discard` | the singleton arm of `create.already-exists` routes at a `task discard` that is refused as written | OPEN — port |
| `(R6, D-8)` = `(R4, F2)` | `start` | the unknown-key load error names the key and not the workflow | OPEN — port (the full entry is `(R4, F2)`) |
| `(R6, D-9)` | `doc create` → `doc rename` | `write.identity-change`'s route names a `doc rename` onto an occupied committed id | OPEN — port |
| `(R6, D-10)` | `start` | under a shadow that drops `new: true`, the composed step still says the create is refused | OPEN — port |
| `(R7, D-1)` | `doc list` | the triage steps say `doc list` lists each record by slug and status; it lists no status | OPEN — port |
| `(R7, D-2)` | `doc show` | on `--task`, fragment-miss routes point at the committed doc, including one with no committed copy | OPEN — port |
| `(R7, D-3)` | `doc list` | a `doc list` row prints an `id` that `doc show` refuses | OPEN — port |
| `(R7, C-1)` | `doc show` · `doc list` | a body-field `default:` is projected into `fields` | OPEN — port |
| `(R8, F-1)` | `workflow` | a `squash=true` sub-task is still told to write and read back its commit doc | OPEN — port |
| `(R8, F-2)` | `workflow` | two steps say `task finalize` in a sub-task *lands a commit on this worktree's detached HEAD*; the binary refuses | OPEN — port |
| `(R8, F-3)` | `milestone add-task` · `workflow` | `add-task --workflow` accepts every loaded workflow | OPEN — port |
| `(R8, F-4)` | `start` | orientation reports the omitted commit doc as blocking on every `squash=true` sub-task | OPEN — port |
| `(R8, C-1)` | `workflow` | a project-layer step naming the per-task door as a literal survives sub-task composition | OPEN — port |
| `(R9, F1)` ⊃ `(R5, F10)` | `task finalize` · `task validate` | help and both guides state the carryover gate as a universal; the doc-only model is on none | OPEN — port |
| `(R9, F2)` = `(R10, F5)` | `task amend` | the composed `amend` step says the message carries only the trailers you author | OPEN — port (= trial `L-10`) |
| `(R9, C-1)` | `task amend` | help and guide say `HEAD`'s message is not read back; the landed message carries a line read from `HEAD` | OPEN — port |
| `(R9, F3)` | `uninstall` | with the log on, `uninstall` leaves `.jigc/` behind and its second run is not a no-op | **FIXED `12398ddc`** (the log writer creates `logs/` only, never the `.jigc/` above it — `crates/cli/src/invocation_log.rs`, read in the tree), and for the teardown itself `4fa1c0f5` |
| `(R9, F4)` | `task finalize` | a migration task's `--dry-run` forecasts two `modified` paths that are not | OPEN — port |
| `(R10, F1)` | `task finalize` | signing opens a new block under a mixed last paragraph; git stops reading the `Signed-off-by` | OPEN — port |
| `(R10, F2)` | `task finalize` (amend arm) | the amend carry misses a `HEAD` trailer git reads inside a mixed block | OPEN — port |
| `(R10, F3)` | `setup` | `setup.profile-load`'s route blames the embedded profile for a directory-selected one | OPEN — port |
| `(R10, F4)` | `milestone finalize` | the `#trailers` recording route is honoured only for a code-carrying sub-task under `squash: false` | OPEN — port |
| `(R10, F5)` = `(R9, F2)` | `task amend` | the composed amend step says *only* the trailers you add | OPEN — port (the full entry is `(R9, F2)`) |
| `(R10, F6)` | `task finalize` | a bracket-less `Co-Authored-By: <address>` is not read as the same address *(marginal)* | OPEN — port |
| `(R10, F7)` | `task finalize` (+4) | the co-author is the commit-time environment's profile, not the one `setup` installed from *(marginal)* | OPEN — port |

### A.4 Leads the re-drive raised and did not grade (4)

README → *After assembly* → *Leads the re-drive raised and did not grade*. No key in the README; `RD-n` is this ledger's label.

| label | where | what | disposition |
|---|---|---|---|
| RD-1 | `tier1-verification/R3-F7.md` §3, §7 class 5 | the linked-worktree sibling of `(R3, F7)`: a task driven from a user-made `git worktree add` loses a hand edit at exit 0 with the baseline present | **FIXED `e3a6ba58`** (the guard, fork arm A; prerequisite `a88f71cc`); arm B parked — `ideas/linked-worktree-doc-work.md`, DEFERRED — M57 "A linked worktree as a real home for doc work". The audit reports the guard over-fires and four route defects → C: linked-worktree F1–F5 · XC-4 · E2E F1, F6 **At the close:** round 4, area B took all six (`55a6e281` · `ac0f63b1` · `e590806f` · `d8f9907d` · `e0f00278` · `df58d1dc`) — unaudited. |
| RD-2 | `tier1-verification/R6-K-1.md` §7 class 3 | `task finalize`'s own plan-to-promote window against an external writer (≈26 ms, 13 losses in 80) | DEFERRED — M57 "The external-writer race class" |
| RD-3 | `tier1-verification/R6-D-1.md` §3 V9 | a directory at the suffixed home: `milestone finalize` exits 1 code-less and the message prints an absolute host path | OPEN — changed by the pass: per `DECISIONS.md` (`9465f9b6`) both committing doors now refuse a non-regular entry at a home under `finalize.promote-clobber`; **this cell was not re-driven** and the host-path datum belongs with `(R5, F7)` |
| RD-4 | `RC-rc24/tier1-verification-L-22.md` §7 | `uninstall` acks *pruned git's worktree registrations for the fan-out worktrees `.jigc/` held* where `.jigc/` held none | OPEN — changed by the pass: `8c159622` removed every repository-wide prune; the printed line still says *pruned* → C XC-7, and ownership is still decided by location → C worktree F3 **At the close: FIXED** — the line reads *dropped git's registrations of the fan-out worktrees `.jigc/` held* (`dcfa40f0`) and is printed only when one of jigc's own was dropped; ownership is what jigc recorded, not location (`c3f6dcaa`). |

### A.5 Handed between rows and graded by none (7)

README → *C · OPEN leads* → the closing table (*"the leads most likely to be lost"*). `H-n` is this ledger's label.

| label | what | disposition |
|---|---|---|
| H-1 | the doc-only left-out narration lists an **untracked** path under *"a staged path stays staged"* | OPEN — port. Seen again by the `c0c4d88c` fixer (D `c0c4d88c`-6) |
| H-2 | `jigc start --task <sub>` inside a sub-task worktree exits 0 and does not provision the commit doc | OPEN — port (related: trial `L-21`) |
| H-3 | a **directory** named `<slug>.md` at a doc home: `task finalize` exits 1 code-less; `doc list` exits 1 code-less for the whole doctype (a dangling link does the same to `doc list`) | OPEN — changed by the pass: the finalize half now refuses under `finalize.promote-clobber` (`9465f9b6`, per `DECISIONS.md`); the `doc list` half is DEFERRED — M57 "Two read seams over an entry jigc cannot read" |
| H-4 | three surfaces call an **untracked** file *committed* | OPEN — port |
| H-5 | `milestone add-task … --workflow <a workflow whose shadow fails to load>` exits 0 and commits a record row no door can compose | OPEN — port |
| H-6 | a plain task's commit between `milestone provision` and `milestone finalize` puts `finalize.base-mismatch` in front of the join | OPEN — port. The pass added a route that sends a sub-agent into it → C XC-4 **At the close:** the route no longer sends a sub-agent there (`e0f00278`); the lead itself is unchanged. |
| H-7 | a move plus an edit of a managed doc passes the hook silently | OPEN — port |

### A.6 Open leads, observations, refuted

| block | count in the README | disposition |
|---|---|---|
| *C · OPEN leads*, row 1 | 8 | OPEN — port, recorded in the README |
| row 2 | 10 | OPEN — port |
| row 3 | 9 | OPEN — port. Its *`(R3, F7)`'s bounds* entry (a location doctype, the `milestone finalize` door, cache deletion as the way in, an older binary) is what `eb18e5fe`'s class derivation covers *as reported*; the older-binary cell is read-only in the audit (C reconcile F2) and, since `0f34d8f0`, is the stated bound of the witness (an area a previous binary minted holds none) |
| row 4 | 8 | OPEN — port. Its `relocate --format json` flattening lead has a sibling at the pass's new refusals → C CPL-7, E2E F7 |
| row 5 | 4 | OPEN — port |
| row 6 | 3 + the driver's U1–U14 | OPEN — port. *`(R6, D-1)` with a foreign file, a directory or a symlink at the suffixed home* is covered by `3f3a724b` / `9465f9b6` *as reported*; a **case-sensitive filesystem** is still un-driven by everyone (C: every area's `not_examined`) |
| row 7 | 8 | OPEN — port |
| row 8 | one list | OPEN — port |
| row 9 | 6 | OPEN — port; *the ruling on `(R9, F5)`'s tier* is taken (`DECISIONS.md`, 2026-10-04) |
| row 10 | one list | OPEN — port |
| *D · Observations* | 11 | recorded in the README as **not defects**. Three touch the pass: row-6 **O-2** (an untracked conformant file at a doc home is copied in under an entry without `new: true`) is the class the `c0c4d88c` fixer's lead and `plan-promote` name (D `c0c4d88c`-2, E PP-13); row-3 **O-7** (`milestone finalize`'s landed envelope has no `findings` key) sits beside DEFERRED — M57 "The landed `jigc milestone finalize` envelope has no key for a dropped registration"; the *declared cost one cell from `(R3, F7)`* (a hand edit made **before** the first write is carried in) is the sentence the audit found the guide now over-states → C XC-3. **At the close:** since `0f34d8f0` the sentence is the rule in every checkout — an edit before the task's first write is carried and lands, with or without a baseline |
| *B · REFUTED* | 25 | refuted, each with its datum in the README; nothing owed |

**Count, table A:** 65 finding rows (6 + 7 + 52) · 4 re-drive leads · 7 handed-between leads · 10 open-lead blocks · 1 observations block (11) · 1 refuted block (25) = **88 rows**. Of the 65: **7 FIXED** (the five upheld tier-1 rows, `(R6, K-1)`, `(R9, F3)`), **2 OPEN — changed by the pass** (`(R3, F3)`, `(R6, D-4)`), **56 OPEN — port**. Of the 4 re-drive leads: 1 FIXED, 1 DEFERRED, 2 changed.

## B. The trial's findings

Full rows: `completions/artifacts/RC-rc24/README.md` → §7 (findings), §9 (tooling), §11 (bounds, doc drift, what is owed); controls in `findings-verification.md`; `tier1-verification-L-22.md`.

### B.1 §7 — tiered (8)

| key | tier · verdict | door | title | disposition |
|---|---|---|---|---|
| `L-22` | 1 · PARTIAL (the confirmed variant) | `milestone provision` · `milestone finalize` · `milestone discard` · `uninstall` | a foreign linked worktree whose directory is absent has its git admin record pruned at exit 0 | **FIXED `8c159622`** (no repository-wide `git worktree prune` remains: `grep '"prune"'` over both crates' `src` is empty at `0f34d8f0` too); the own-registration fork: `ebfc79fb` (provision, discard, uninstall refuse) and `e842342e` (the boundary refuses before landing). Audit → C: worktree F1–F7 · XC-7; round 4, area F: `894f2234` · `f3186486` · `c3f6dcaa` · `ad1d2ef7` · `f1b84ef1` · `7e5ba186` · `ed783637`, and `dcfa40f0` — **unaudited**. The container cell and the `git gc` permanence cell were not re-driven on any fixed binary (C E2E `not_run`) |
| `L-3` | 2 · PARTIAL | `start` | two `jigc start` refusals answer `--format json` with a code-less `{"error": …}` and log no identity | OPEN — port |
| `L-2` | 3 · CONFIRMED | `start` | orientation prints ``Run: `jigc start --workflow planning` ``, which exits 1 as printed | OPEN — port |
| `L-4` | 3 · CONFIRMED | `start` | orientation's resume line for a sub-task refuses from the checkout that printed it; no forward surface names `milestone provision` / `execute` | OPEN — port |
| `L-6` | 3 · CONFIRMED | `start` | orientation's `Project config:` header path is absolute | OPEN — port |
| `L-10` | 3 · CONFIRMED | `task amend` | the amend step says the message carries only the trailer items you add | OPEN — port (= `(R9, F2)`) |
| `L-11` | 3 · CONFIRMED | `start` (composed commit steps) · `doc schema` | composed steps print the bare `#type` / `#scope` alias; `doc schema commit` lists only `#header/…` | OPEN — port |
| `L-18` | 3 · CONFIRMED | `doc author` | a refused payload's only real coordinate is prose; JSON `location` prints a sentinel where the contract says `null` | OPEN — port |

### B.2 §7 — confirmed or partial, with no tier (14)

All fourteen: **OPEN — port** (recorded in the trial README as reproduced and contradicting no contract; none touched by a fix commit of this pass).

| key | verdict | title |
|---|---|---|
| `L-1` | PARTIAL | on a worker's path, the router catalog line is the one surface naming `report-inconsistency` |
| `L-7` | CONFIRMED | the pre-commit hook's `jigc validate` lands in the invocation log with no caller marker (the same hook invocation is what re-mints the log in C XC-5) |
| `L-8` | CONFIRMED as an observation | the hook's in-flight sweep reports `un-baselined` / `hash-matches` for the commit's own writes |
| `L-9` | PARTIAL | the `task amend` composed text prints a bare `git log -1 --format=%s` |
| `L-12` | PARTIAL | the pre-commit forecast stops at the subject line; no surface renders the trailer block before the commit |
| `L-13` | CONFIRMED | `jigc start "<intent>"` composes the router byte-identically for every intent |
| `L-14` | PARTIAL | the composed `planning` text carries eight jigc milestone tags, *"the orchestrator"* and a Rust parameter name |
| `L-15` | CONFIRMED | the `planning` composition is 45,112 bytes under `--format json`, over the harness's inline limit |
| `L-16` | CONFIRMED as an observation | `milestone create` / `add-task` each land a record-only commit at once |
| `L-17` | PARTIAL | nothing on the worker's path names `milestone join` or a pre-boundary check of the combined code |
| `L-20` | PARTIAL | sub-task composed text carries `step:implement`'s un-scoped *"before finalize"* beside *"never `jigc task finalize` here"* |
| `L-21` | CONFIRMED | `jigc start --task <sub>` inside the worktree is a designed second door beside `jigc workflow sub-task --task <sub>` (related: A H-2) |
| `L-23` | PARTIAL | co-author trailer placement under a body that ends in `Key: value` |
| `L-24` | PARTIAL | `task validate` / `task finalize` report one advisory `file-state.staged-copy` per persisted staged doc |

### B.3 §7 — refuted (2) and open (13)

| key | what | disposition |
|---|---|---|
| `L-5` | orientation offers `milestone finalize` over nothing staged | REFUTED in the README (it refuses at exit 3); nothing owed |
| `L-19` | `doc list --format json` prints a `note:` line after the document | REFUTED in the README (the note rides stderr); nothing owed |
| `O-1` | `L-22`'s everyday trigger: a repository bind-mounted into a container that cannot see the host's linked worktrees | driven by the re-drive and holds; closed with `L-22` (**FIXED `8c159622`**) — the container cell itself was **not re-driven on the fixed binary** |
| `O-2` | `L-22` at `uninstall`, the squash combine's dedicated worktree, and under `--format json` | `uninstall` and the combine worktree: **FIXED `8c159622`**; the ack's wording: **FIXED `dcfa40f0`**; `--format json` still not driven |
| `O-3` | `L-23`'s sibling edge: a prose line followed by `Signed-off-by:` in the body | OPEN — port, to be triaged as a lead of its own (§11) |
| `O-4` | `L-18`'s adjacent: `write.wrong-shape` carries two meanings and two routes at `doc author` | OPEN — port, to be triaged as a lead of its own (§11) |
| `O-5` | `L-9`'s narrower cell: the bare `git log -1` reads `HEAD` of whichever checkout the reader stands in | OPEN — port |
| `O-6` | `L-10`: a different address lands a second co-author line; a profile without the key; a non-Claude assistant | OPEN — port, not driven |
| `O-7` | `L-8`'s residue: a user hook failing on non-empty `blocking_probes` | OPEN — port, not driven |
| `O-8` | `L-6`: the two other `DeclaredAbsolute` printed classes | OPEN — port, not driven |
| `O-9` | `L-22` cell 7: the `repo.head-detached` refusal from a detached foreign worktree says *"a commit made here"* | OPEN — port (`plan-worktree` §10 leaves it to this row; no fix commit names it) |
| `O-10` | `L-3` top-level-task arm; `L-11` for other doctypes' header fields | OPEN — port, not driven |
| `O-11` | the trailer's two unexercised paths (a sub-agent's committing door; de-duplication by address) | OPEN — port, not exercised |
| `O-12` | `T-14`'s adjacent: `halted_awaiting_human` reads the last `result` event only | OPEN — trial tooling, filed at the port |
| `O-13` | `T-7` / `T-11` un-driven halves | OPEN — trial tooling, filed at the port |

### B.4 §9 — tooling rows (15 + 17)

All thirty-two: **OPEN — trial tooling, filed at the port.** None is a product row; none was touched by this pass. §11 also owes a `work/` change for the driver over `T-1`, `T-3`, `T-4`, `T-6`, `T-7`, `T-8`, `T-9`, `T-10`.

| id | verdict | what |
|---|---|---|
| `T-1` | CONFIRMED | `run.py observe` reports a correct `jigc task amend` as `HISTORY REWRITTEN` |
| `T-2` | PARTIAL | `observe` reads arm (a) turn 2 as `VOID unmeasured` |
| `T-3` | CONFIRMED | `observe`'s `git!` detector misses `git -C <path> commit` and `git -c k=v commit` |
| `T-4` | CONFIRMED | `tools/raw-git-acts.py` matches nine history verbs only |
| `T-5` | CONFIRMED | runbook §10 copied the walk record with a plain `cp` (5 host-path lines) — **fixed in the committed runbook**, per the row |
| `T-6` | CONFIRMED | `test_session.py` defines a test class after its `unittest.main()` |
| `T-7` | PARTIAL | `run.py fork` / `observe` take a sub-agent's transcript as the main session when no main file exists |
| `T-8` | CONFIRMED | `observe`'s `recs` column counts adapter-hook invocations as session records |
| `T-9` | CONFIRMED | `tools/trailer-rows.py` pairs same-second doors and commits by a tie-break |
| `T-10` | CONFIRMED | `trailer-rows.py` cannot see a commit on a fan-out worktree's detached `HEAD` once the worktree is removed |
| `T-11` | PARTIAL | the operator's saved rubric files and logs carry host paths with a login name (not committed) |
| `T-12` | CONFIRMED | arm (c)'s class table has no row for *kept the prose's framing, flagged nothing* |
| `T-13` | PARTIAL | runbook §7.3 runs the session's test suite inside the evidence out-dir |
| `T-14` | CONFIRMED | the runbook's `result` helper prints every `result` event unlabelled |
| `T-15` | PARTIAL | arm (a)'s three background sub-agents all started in the main agent's shell directory |
| `P-1` | confirmed | stale default image tags at every door |
| `P-2` | confirmed (a gap the README declares) | the harness README's check list enumerates five of seven |
| `P-3` | confirmed | a dead source path in `trial-corpus-template/README.md` |
| `P-4` | confirmed | the walk README's arm table omits arms `22`, `23` |
| `P-5` | confirmed for the sets; registry half read, not driven | `verify-pair.sh` has no probe set for any later pair |
| `P-6` | confirmed | both sha comparisons are `unknown` against `unknown` on a registry image |
| `P-7` | confirmed | check 4 spans two CLI versions |
| `P-8` | confirmed (driven) | M54's registry image tag now names another image |
| `P-9` | confirmed | the driver's own suite is run by nothing |
| `P-10` | confirmed | three things about `seed` (an unasked `rmtree`; no `--strict` / model flag; a late fixture check) |
| `P-11` | confirmed | the template commits with the ambient identity |
| `P-12` | confirmed | `corpus-src` is recorded as a host path |
| `P-13` | confirmed | a prompt file is left beside the out-dir |
| `P-14` | confirmed | two slips in the trial's own corpus record |
| `P-15` | confirmed | RC-rc14's archived evidence cannot be re-scored |
| `P-16` | confirmed | the two helpers are the trial's own and have no test |
| `P-17` | confirmed | a home path in a live script (`trial-harness/build-image.sh`) |

### B.5 §11 — doc drift and what is owed (8)

| label | what | disposition |
|---|---|---|
| DD-1 | `CLAUDE.md` → *Project state* names rc.23 where the current release is rc.24 | **DISCHARGED by the commit that carries this ledger** — the paragraph names `1.0.0-rc.24` as the current release, the gate's result and what comes next |
| DD-2 | `completions/artifacts/M55/settle-log.md` S16 still carries three withdrawn parentheticals | recorded as a dated record that stays as written; nothing owed |
| DD-3 | 78 committed files under `completions/artifacts/M51`–`M53` carry absolute host paths and a login name | OPEN — a decision owed to the port or a hygiene pass of its own |
| OW-1 | at the port: the eight tiered and fourteen untiered rows filed through jigc; `O-3` and `O-4` triaged | OPEN — port |
| OW-2 | the pointer from `decisions-pending.md`'s blind-trial entry to `protocol.md` §0 | OPEN — *as reported*; not checked here |
| OW-3 | the `work/` change for the trial driver (`T-1`, `T-3`, `T-4`, `T-6`–`T-10`) | OPEN — trial tooling |
| OW-4 | a class table that can hold arm (c)'s behaviour, and a disagreement instrument whose prose states something the code contradicts (`T-12`) | OPEN — before the findings channel is put to a blind worker again |
| OW-5 | the correction to protocol §0.1's refusal sentence, and §11 bound 4's stray `.git/lost-found/commit/…` file left under the operator's out-dir *"for the human"* | OPEN |

**Count, table B:** 8 tiered · 14 untiered · 2 refuted · 13 open (`O-`) · 15 `T-` · 17 `P-` · 8 drift/owed = **77 rows**. **1 FIXED** (`L-22`, with `O-1` and `O-2`'s driven halves); 1 discharged (DD-1); 2 refuted; everything else OPEN.

## C. The completion audit's 54 findings

Full findings — severity, `where:`, `class:`, the repro block — in `audit/audit-findings.md` (the auditors' structured returns, verbatim, copied here). Six code reviewers, one area each, and one end-to-end tester, all on the tree at `5f5b273a`. **Every area's verdict is red; the E2E verdict is FAIL.** The *disposition* column is what happened **after** the audit — round 4, the last fix, the closing record commit — and none of that has been audited in turn. The file's own header: *findings are the auditors' hypotheses with their own repro blocks: verify-real (reproduce as a red test) before fixing.*

Ids repeat across areas (`F1` exists five times), so every row carries its area. `driven` = the auditor drove the binary; `read` = source, logs or tests read only. The *class* column is the auditor's class sentence cut to a few words — the sentence itself, with its counts, is in the file.

**Only three of the six reviewers wrote a full report file** (`audit/review-install-teardown.md`, `review-linked-worktree-guard.md`, `review-reconcile-baseline.md`); the cross-cutting, worktree-registrations and create-promote-link reviewers and the E2E tester returned the structured result only, so for those four `audit-findings.md` is the whole record.

### C.1 Cross-cutting (`XC`) — 7

| id | sev | door | title | class, in a few words | driven / read | disposition |
|---|---|---|---|---|---|---|
| XC-1 | MEDIUM | the shared logs (`DECISIONS.md`, `decisions-pending.md`) | the logs do not record the six round-3 fixes; `milestone.unlanded-work` has no stated reason; eleven fixer-declared human's-call items have no pending row; three sentences and one pending row are now false | record-versus-tree gap over round 3; 18 lookups, 0 hits | read | **RECORDED by the commit that carries this ledger** — `DECISIONS.md` gains the as-built entries for rounds 3 and 4 and the stated reason for `milestone.unlanded-work`; the human's-call items have a row in `implementation/decisions-pending.md`; the stale sentences carry dated corrections. **One sub-item open:** the doc comment on `contribution_label` in `crates/cli/src/render.rs` still says the line states *what git's registration of it held* — product code, outside a record commit |
| XC-2 | MEDIUM | `setup` · `DECISIONS.md` | the `(R1, F1)` as-built claim overstates the fix: with git unable to answer, `setup` still replaces a file holding bytes no commit holds | instance, unbounded — one trigger of the `Unknown` arm driven | driven | **FIXED `f0b1120a`** (the behaviour, with install F2: where git cannot answer, nothing is installed); the as-built paragraph's bound is stated in `DECISIONS.md` by the commit that carries this ledger. Left: where the install home is no work tree the dirty question is not asked (F `4E-2`) |
| XC-3 | MEDIUM | `crates/cli/guides/MIGRATING.md` item 8 (met at `task finalize`) | the guide's new sentence *"An edit you made before the task first wrote the doc is carried into the task and lands with it"* is false wherever a baseline is held | instance — one false sentence found by driving | driven | **FIXED `b54b58b2`** (the sentence replaced by the three driven cells) — and **the behaviour itself then changed at `0f34d8f0`**: an edit before the first write is carried and lands with or without a baseline, and the guide's item 8 states that one rule |
| XC-4 | MEDIUM | `start --workflow` (a doc-only mint) from a fan-out worktree → `milestone finalize` | the guard's route sends a doc-only mint to the main checkout, where landing it blocks the reader's own milestone on `finalize.base-mismatch` | the routes of `finalize.linked-worktree-doc` from a fan-out worktree; 1 of 4 call sites driven | driven | **FIXED `e0f00278`** (six producers of the `cd`, against the one driven; from a fan-out worktree no stash is printed). The reverse order to `finalize.base-mismatch` was not re-driven by the fixer |
| XC-5 | LOW | `uninstall` (the stated bound) · the pre-commit hook | the bound on *"the teardown never starts the log"* undercounts: a plain `git commit` mints the log through the hook, and `uninstall.dirty-worktree`'s route offers *commit* | instance, unbounded | driven | **FIXED `8b214ad5`** (the bound restated in its three homes; the order that ends it in one pass documented and driven) |
| XC-6 | LOW | the gate's test step | the pass's suites are the bulk of the step's growth; 537 s against the 600 s foreground ceiling | the 14 suites the pass added or grew, timed | driven (nextest) | **OPEN.** Not taken, and worse: the step measured 573–600 s on round 4's last runs and 590 s at `0f34d8f0`, as the fixers reported, against the 600 s foreground ceiling → `perf/run-performance.md` (recommendations only) |
| XC-7 | LOW | `uninstall` (a printed line) | `uninstall` still prints *pruned git's worktree registrations* after the pass removed every prune; the test pins the old word under a message naming the new one | instance — one printed line | driven (line) · read (test) | **FIXED `dcfa40f0`** (three suites pinned the old word, not one) |

### C.2 Linked-worktree guard (`a88f71cc` · `e3a6ba58`) — 5

| id | sev | door | title | class, in a few words | driven / read | disposition |
|---|---|---|---|---|---|---|
| linked-worktree F1 | HIGH | every `jigc doc` write leaf · `task finalize` · `start` · `migrate` | the guard fires where `jigc_home` is not a checkout (pointer-to-bare, sibling bare, submodule, `--separate-git-dir`): doc creation that landed on rc.24 is refused, and every printed route dead-ends | one predicate, three constructors; four layouts driven, unbounded | driven | **FIXED `55a6e281`** — the predicate is asked of git: the guard binds only where the doc store's home is itself a checkout of the same repository. Closes E2E F1 |
| linked-worktree F2 | MEDIUM | `task validate` · `task finalize` (the `doc-code` blast walk) | `a88f71cc` leaves the second reader of a staged doc's committed bytes open: the blast-radius walk still blocks on an anchor the staged copy removed | 2 readers, one fixed, one open | driven | **FIXED `df58d1dc`** (decided from the code to be the same defect as `a88f71cc`, closed the same way) |
| linked-worktree F3 | MEDIUM | the relocation route (a `doc` write leaf's dangling-anchor arm; the changelog gate) | the relocation route's mint collides with the task it relocates when `<intent>` is the task's own | 2 print sites of one producer | driven | **FIXED `e590806f`** (the discard leads). Closes E2E F6. The red the fixer observed was its order assertion, not the collision |
| linked-worktree F4 | MEDIUM | `task finalize` from the main checkout (the backstop's route) | the *"finalize it from the main checkout"* route commits the main checkout's pre-task staged work at exit 0 | instance, unbounded | driven | **FIXED `ac0f63b1`** (`HomeLanding`: the route is decided over the pin and the main checkout's index) |
| linked-worktree F5 | LOW | route text (three print sites, one print site) | two route sentences are false in a reachable state | (a) 3 sites · (b) 1 site, false in 2 driven states | driven | **FIXED `d8f9907d`** (both sentences) |

### C.3 Worktree registrations (`8c159622` · `ebfc79fb` · `e842342e`) — 7

| id | sev | door | title | class, in a few words | driven / read | disposition |
|---|---|---|---|---|---|---|
| worktree F1 | HIGH | `milestone discard` (un-forced) · `uninstall` | with `status.showUntrackedFiles=no` set, both destroy a sub-agent's untracked work at exit 0 (pre-existing) | 8 production `git status` probes, 3 pass no `--untracked-files`; 1 examined, 2 doors driven | driven | **FIXED `894f2234`** — nine production `git status` sites through one seam, against the three named; a source fence |
| worktree F2 | MEDIUM | `milestone provision` | on git older than 2.31, `provision` no longer re-creates a sub-task worktree whose directory is gone, and the advisory's route does nothing — a regression against rc.24 | the one door that reads `prunable` | driven **under emulation** (no real old git) | **FIXED `f3186486`** — driven on a real git 2.30.9 built in a container, as the fixer reported; the suite's cell is a stated stand-in |
| worktree F3 | MEDIUM | `uninstall` | still drops a registration jigc did not create when its recorded path is under `.jigc/worktrees/`, and attributes it to the fan-out | 1 of 5 `remove_owned_registration` sites | driven | **FIXED `c3f6dcaa`** (ownership is what jigc recorded, not where the record points) |
| worktree F4 | MEDIUM | `milestone finalize` | in a moved repository the boundary still lands, at exit 0, without staged code one of its own sub-task registrations holds | instance bounded by mechanism — one path-keyed lookup, 3 consumers | driven | **FIXED `7e5ba186`** (only the boundary asks; three cells driven, against one handed) |
| worktree F5 | MEDIUM | `milestone finalize` (the new refusal) | the refusal says *run the command on that line* over a line with no command, when the checkout's `.git` entry is present but unreadable | 2 cells without a command-bearing arm; 1 driven | driven | **FIXED `f1b84ef1`** (the cell is said on the shared line; the rule that no printed command overwrites a `.git` entry is kept) |
| worktree F6 | MEDIUM | the keep command (four worktree doors; driven at `milestone finalize`) | the keep command still exits 128 as printed when a branch differing only in case exists on a case-insensitive filesystem | one predicate, 2 callers | driven | **FIXED `ad1d2ef7`** (case-folded comparison, on every filesystem) |
| worktree F7 | MEDIUM | `DECISIONS.md` | the decision log contradicts the design doc and source on who ruled the boundary refusal, still states the superseded behaviour, and has no entry for `e842342e` or its code | instance, unbounded | read | **FIXED `ed783637`** (the design doc and three rustdoc sites point at the log instead of attributing the call) **and RECORDED by the commit that carries this ledger**: the pre-landing refusal was the orchestrator's reading of the fork under the standing rule, not a separate ruling; the entry for `e842342e` and its code is written |

### C.4 Create · promote · link (`c0c4d88c` · `3f3a724b` · `9465f9b6` · `5f5b273a`) — 10

| id | sev | door | title | class, in a few words | driven / read | disposition |
|---|---|---|---|---|---|---|
| CPL-1 | HIGH | `milestone finalize` | two sub-tasks that each mint a placement singleton both promote to one path — one sub-task's doc is lost at exit 0, in no commit and on no disk | two promotions in one plan sharing a destination; 5 shipped placement doctypes | driven | **FIXED `f031d31e`** (a contested destination is refused at the boundary, keyed at it; the join and its preview are unchanged) |
| CPL-2 | HIGH | `milestone finalize` | a promote overwrites a file a sub-task worktree staged at the same path — exit 0, the staged file in no commit, the manifest counts it as landed | a promote destination that is also in a worktree's staged set; 4 cells driven | driven | **FIXED `048724d0`** (`HomeClaims::staged`; exits: the in-task rename, or a stash) |
| CPL-3 | HIGH | `milestone finalize` (the new refusal's route) | the promote-clobber refusal prints a `jigc doc rename` that refuses for every fixed-identity doctype — a dead-end route | file-arm route × 5 placement singletons; 1 driven | driven | **FIXED `d724365f`** at both committing doors (*free the home*, or *drop the mint*). The task door's migration arm still routes a fixed-identity doc at re-slugging — read, not driven |
| CPL-4 | MEDIUM | `relocate` · `config set placement-root` | a displaced squatter is parked by basename and overwrites an earlier parked file of that name — exit 0, sole copy gone (pre-existing; in a function the pass widened) | instance, unbounded — one park site | driven | **FIXED `685a4c55`** |
| CPL-5 | MEDIUM | `task finalize` (also `milestone finalize`, `milestone create`) | the absent-home cell is still open at both doors and its only record is a design-doc open question — no trigger row | a home git holds and the disk does not; 3 doors plus an index-only variant | driven | **FIXED `78e8ded1`** — *an identity git holds is occupied*; three doors, against two handed. One sequence jigc's own route produces now refuses → the closing section, (a) |
| CPL-6 | MEDIUM | `migrate-corpus` | the relocation arm probes its destination through links and replaces a dangling link there at exit 0, unnamed | 2 follow-links sites in `migrate_corpus.rs`; 2 cells driven | driven | **FIXED `526141e6`** (the relocation arm). The in-place arm still replaces a link at the doc's own home, unnamed |
| CPL-7 | MEDIUM | `rename --format json` (also `relocate`, `config set docs-root` / `placement-root`) | `finalize.promote-clobber` at `rename` reaches a JSON driver as an `{"error": …}` string with no key, while the contract row says it is keyed at the home | 2 production sites, 4 doors; wire shape driven at 1 | driven | **FIXED `c97b8eb6`** (`rename` and the move primitive's own refusal). The four other doors are E2E F7, open |
| CPL-8 | LOW | `config set placement-root` | a refused re-point leaves an already-displaced squatter in the gitignored workbench; the route's all-undone sentence does not mention it | the one declared residual of the rollback, newly reachable | driven | **FIXED `242341bb`** (the route names the parked file; it is still not moved back, the declared bound) |
| CPL-9 | LOW | the shared logs · `relocate --help` · `config set --help` · the dangling-link route | `5f5b273a` has no as-built `DECISIONS` entry and no pending rows; two helps do not state their new refusal; the dangling-link route names no `HEAD` restore; a refused re-point leaves empty directories | instance, unbounded — records and wording | read (a) · driven (b)–(d) | **OPEN — in part.** (a) the `5f5b273a` as-built entry and the pending rows: RECORDED by the commit that carries this ledger. Not taken by any round, and not re-checked here: (b) `relocate --help` and `config set --help` do not state the link refusal; (c) the dangling-link route at `rename` names no `HEAD` restore; (d) a refused `docs-root` re-point leaves empty destination directories (the class of `(R5, F9)`) |
| CPL-10 | LOW | `jigc-engine`'s public API | engine `pub` signatures moved in a published rc crate; every in-workspace caller is updated; the release PR's semver check may flag it | the `pub` diff of the whole engine crate across the pass | read | **OPEN — recorded.** The engine `pub` moves of the whole pass are listed by commit in `DECISIONS.md` by the commit that carries this ledger; round 4 and the last fix added more (`676eff57`, `78e8ded1`, `4ba04efc`, `f0b1120a`, `0f34d8f0`). The release PR's semver check has not seen any of them |

### C.5 Install · teardown (`12398ddc` · `dc0d7586` · `104a7d4b` · `ef9456b5` · `4fa1c0f5`) — 8

| id | sev | door | title | class, in a few words | driven / read | disposition |
|---|---|---|---|---|---|---|
| install F1 | HIGH | `setup` | **regression with a dead-end route:** a clean tracked install path whose committed blob holds CRLF refuses `setup` as *flagged*, on every run | one comparison site, asked of every present tracked member; 2 triggers driven | driven | **FIXED `676eff57`** — the flag is read from git, never inferred from a hash mismatch; the index-flag class survives and is kept at replaced and merged-into members alike |
| install F2 | HIGH | `setup` | exit-0 loss, unchanged from rc.24 and declared nowhere: when `git status` fails, `setup` installs blind over uncommitted bytes | 3 `InstallSubject::Unknown` sites; 3 triggers driven | driven | **FIXED `f0b1120a`** — five triggers driven, against three handed; `--force` does not pass the refusal, which is the fixer's reading of the decision |
| install F3 | HIGH | `setup` · `uninstall` | exit-0 loss, pre-existing: the guide's ownership oracle proves the body only, so an uncommitted front-matter edit is replaced by `setup` and deleted by `uninstall` | 4 consumers of the oracle; the 2 that destroy both driven | driven | **FIXED `33620081`** (one whole-file oracle, all four consumers) |
| install F4 | HIGH | `uninstall` | exit-0 loss, pre-existing, outside the class `(R9, F5)` names: `uninstall` deletes a standalone jigc `pre-commit` hook the adopter extended | instance, unbounded | driven | **FIXED `528d0205`** (the teardown cuts exactly the block jigc wrote; a block it cannot find leaves the hook, and says so) |
| install F5 | MEDIUM | `task finalize` · `uninstall` | the repository `ef9456b5` makes first-class is certified at `setup` only: with jigc's paths gitignored, `task finalize` cannot land and `uninstall` refuses over jigc's own files | 4 ignore shapes × 3 doors driven | driven | **CLOSED BY DECLARATION `a330b4f9`** — ignoring jigc's own paths is stated *unsupported* beyond `setup` losing nothing in it, and `finalize.stage-failed`'s route now names the ignore rule. The doors were **not** made to agree: the fixer's judgement, the human's to reverse → the closing section, (a) |
| install F6 | MEDIUM | `uninstall` (a design sentence) | a doc that is false: *"whatever this door takes, it names"* — `uninstall` takes jigc's own files in a milestone area at exit 0, named by nothing | 6 milestone + 15 task registry rows; 4 driven | driven | **FIXED `1b1d7656`** (each work unit the teardown takes is named) |
| install F7 | MEDIUM | the shared logs | the three round-3 commits are in no log, and what they left open has no trigger row; two engine API changes unrecorded | 6 open items counted from three fixer reports | read | **RECORDED by the commit that carries this ledger** (the log entries; rows for the items that lived only in fixer reports) |
| install F8 | LOW | `setup` (the unborn refusal's commit arm) | following the commit arm silently forfeits the secrets-floor `.gitignore` | instance, unbounded | driven | **FIXED `cba98c44`** |

### C.6 Reconcile · baseline (`eb18e5fe` · `8d9c3afb`) — 8

| id | sev | door | title | class, in a few words | driven / read | disposition |
|---|---|---|---|---|---|---|
| reconcile F1 | HIGH | `task finalize` (after another task's finalize, or `ingest`) | another writer moves the doc's baseline between the hand edit and the holding task's finalize: exit 0, the edit in no git object | every production writer of a path's key but the staging task — 11 sites, 2 movers driven | driven | **FIXED `0f34d8f0`** — the copied-in witness. Round 4 returned it *could-not-fix*: the working area held nothing that answered *is this what the task copied in?*. Driven by the last fixer: two movers at the task door, `ingest` at the milestone door; five of the eleven key writers not driven |
| reconcile F2 | HIGH | `task finalize` | the declared residual (staged doc, no record, no blob at the pin) is reachable from one task by a plain sequence, and jigc's own route directs the hand edit it then overwrites | {no record} × {no blob}; 3 of 4 cells driven, all exit-0 losses | driven | **FIXED `0f34d8f0`** (*could-not-fix* in round 4, same missing witness) |
| reconcile F3 | HIGH | `task finalize` · `milestone add-task` | in an eol-converting checkout the backstop conflict-blocks a doc nobody edited, and no printed exit works — rc.24 landed | 3 working-file forms, the seam answers 2; 4 consumers, 2 driven | driven | **FIXED `676eff57`** — the seam carries git's verdict, not bytes; four consumers, against two handed |
| reconcile F4 | HIGH | `task finalize` (a migration task) | a migration task's own source is exempt from the backstop unconditionally: a hand edit to it after the mint is replaced at exit 0 | instance, unbounded — one doctype, one source shape, one door | driven | **FIXED `4ba04efc`** — three source shapes, against the one reported. Closes E2E F3 |
| reconcile F5 | MEDIUM | every first-touch `doc` write | an unreadable `file-state.json` is a new refusal with no route and a message naming the committed doc instead of the cache | 3 fail-closed arms; the record arm has no route | driven | **FIXED `d24a0c91`** (at `FileStateRecord::load`, so at every reader) |
| reconcile F6 | MEDIUM | `MIGRATING.md` · `design/reconciliation.md` · `design/validation.md` · `decisions-pending.md` | four sentences the pass wrote are false on this build | instance list | read, against driven repros | **FIXED `b54b58b2`** (three sentences); the fourth, `decisions-pending.md`'s residual row, by the commit that carries this ledger. All four were then overtaken by `0f34d8f0`, which rewrote the same passages to the one rule |
| reconcile F7 | MEDIUM | `copy_in_baseline.rs` · `record_door_baseline.rs` | the acceptance suites iterate the axis but skip its crossing cells — the three where the losses and the false block live | three masked cells | read | **FIXED** — the migration-source, unreadable-record and undone-edit cells in round 4 (`4ba04efc`, `d24a0c91`, `e39d9ac9`; *partial* in the round's own report), the crossing cells at `0f34d8f0` |
| reconcile F8 | LOW | `task finalize` | a hand edit reverted after the copy-in is committed anyway and announced as *external edit absorbed* | instance, unbounded | driven | **FIXED `e39d9ac9`** (the advisory's words); the behaviour then changed at `0f34d8f0` — an edit undone after the copy-in now blocks |

### C.7 End to end (`E2E`) — 9

| id | sev | door | title | class, in a few words | driven / read | disposition |
|---|---|---|---|---|---|---|
| E2E F1 | HIGH | the four raise sites of `finalize.linked-worktree-doc` | the guard fires where no main checkout exists and its route dead-ends (bare repo + worktree, `--separate-git-dir`, submodule) | 4 raise sites, 4 layouts driven, unbounded | driven | **FIXED `55a6e281`** — the same cell as linked-worktree F1; the fixer reproduced it in all four layouts against the published rc.24 |
| E2E F2 | MEDIUM | `setup` | `setup` still writes through a committed link at a merged-into install member — into, or creating, a file outside the repository — at exit 0 | 5 `Preserves` members; 2 driven write through | driven | **FIXED `55cb29fa`** — five merged-into members dispositioned, against two driven |
| E2E F3 | MEDIUM | `task finalize --approve` (a migration) | a hand edit to a migration's same-path source after `jigc migrate` is replaced at exit 0, never shown in the review | instance — the changelog same-path migration | driven | **FIXED `4ba04efc`** — the same cell as reconcile F4 |
| E2E F4 | MEDIUM | `DECISIONS.md` | false at HEAD: it says five queued members are not built and admits one new finding code; HEAD builds all five and carries a second | the three orchestrator-owned logs | read | **RECORDED by the commit that carries this ledger** (as XC-1) |
| E2E F5 | LOW | `task finalize` | the declared no-blob cell is still an exit-0 loss (untracked doc copied in + cache deleted + hand edit) | one of three adoption-keeping cells | driven | **FIXED `0f34d8f0`** (*could-not-fix* in round 4) |
| E2E F6 | LOW | the `finalize.linked-worktree-doc` write-leaf route | the dangling-anchor exit's mint step collides when the linked task's intent is reused | instance (one route text) | driven | **FIXED `e590806f`** — the same cell as linked-worktree F3 |
| E2E F7 | LOW | `rename` · the milestone-record door · `milestone provision` · `milestone discard` · `milestone create` | new refusals at five doors print the `{error}` envelope with the finding as text only — no `findings`, no `key`, no `route` field | instance, unbounded — five doors observed | driven | **OPEN.** `c97b8eb6` converted `rename` and the move primitive; the milestone-record door, `milestone provision`, `milestone discard` and `milestone create` were taken by no round |
| E2E F8 | LOW | `milestone finalize` | a FIFO at a doc home hangs `jigc milestone finalize` | instance — one door driven | driven | **OPEN** — taken by no round. The M57 row *Two read seams over an entry jigc cannot read* is noted with this door by the commit that carries this ledger |
| E2E F9 | LOW | various | six riders seen while driving, all pre-existing or off the printed route — see C.8 | six instances, unbounded | driven | **OPEN** — six riders, taken by no round → C.8. (6)'s guide half is now *stated* in `design/assistant-adapter.md` (`33620081`), not fixed |

### C.8 The findings round 4 was not handed — what each needs

Written at `5f5b273a`, and kept as written: what each auditor said its finding needs. **At the close:** XC-1, install F7, E2E F4 and worktree F7's log half are discharged by the closing record commit; XC-2, E2E F1, F3 and F6 by round-4 commits (the rows above). **What stays open of this table: XC-1's one doc comment, XC-6, CPL-9 (b)–(d), CPL-10, E2E F7, E2E F8 and all six riders of E2E F9.**

| id | what it needs, as its auditor states it |
|---|---|
| XC-1 | an as-built `DECISIONS.md` entry for `104a7d4b`, `ef9456b5`, `4fa1c0f5`, `8d9c3afb`, `e842342e`, `5f5b273a`; the stated reason for `milestone.unlanded-work` (it exists only in the fixer's report); a pending row or a ruling for each of the eleven human's-call items (→ table D); and five stale statements corrected — the top entry's *"None is built"*, its item 1 on `write_version_stamp`, its item 5's cite of `finalize.md` *Declared bounds* (3), `decisions-pending.md`'s row on the dropped-registration key (*"is on stderr in both formats"*, removed by `e842342e`), and the doc comment at `crates/cli/src/render.rs:5848` |
| XC-2 | the bound on the `(R1, F1)` as-built paragraph (*"never replaces bytes no commit holds without refusing first"* holds only where git answers), and a pending row for the `Unknown` arm if install F2 is not closed in round 4 |
| XC-6 | a measurement and a decision: the test step is at 537 s (one fixer measured 554 s) against the 600 s foreground ceiling; the ranking of the pass's suites is in the row |
| worktree F7 | the decision log corrected on who ruled the boundary refusal, the superseded sentence struck, an entry for `e842342e` and its code (overlaps XC-1) |
| CPL-9 | the `5f5b273a` as-built entry and pending rows (overlaps XC-1); `relocate --help` and `config set --help` stating the new link refusal; the dangling-link route at `rename` naming a `HEAD` restore as the record door's does; a refused `docs-root` re-point leaving empty destination directories (the class of `(R5, F9)`) |
| CPL-10 | the engine `pub` moves recorded for the release pipeline's version decision: `DECISIONS.md` records `create_gated` / `state::create`; **not** `create_occupied`, `already_exists_finding`, `plan_milestone_finalize`, `MaterializeOutcome`'s new field, nor (install F7) `setup_dirty_install_finding`'s fourth argument and the new `SetupUnseen` |
| install F7 | log entries for the three install/teardown round-3 commits, and trigger rows for the six items that live only in fixer reports: the `Unknown` arm; the index-flag class at merged-into members; leave-and-say-so at `task finalize`; the teardown-log bound; a linked `.claude/skills`; the tracked-path-deleted wedge |
| E2E F1 | as linked-worktree F1 — and `decisions-pending.md`'s row *"A worktree of a bare repository answers isn't set up at every door"* is false for the pointer-to-bare layout, where the guard does speak |
| E2E F3 | as reconcile F4; un-driven: the other `migrate-*` workflows and a non-same-path source |
| E2E F4 | as XC-1 |
| E2E F6 | as linked-worktree F3 |
| E2E F7 | a ruling: whether the four doors beside `rename` owe a keyed findings envelope (the milestone-record door, `milestone provision`, `milestone discard`, `milestone create`), or a written bound |
| E2E F8 | the read seam closed (M57 row), or its reach restated to include `milestone finalize` |
| E2E F9 (1) | after `(R6, D-7)`'s route the link is still at the home: `jigc doc list <type>` exits 1 with a raw OS error that prints an absolute host path (also on rc.24) |
| E2E F9 (2) | `finalize.promote-clobber`'s regular-file route prints an absolute host path inside `jigc migrate <path> --as <doctype>` |
| E2E F9 (3) | `jigc task validate` exits 0 over an occupant that `finalize --dry-run` and `finalize` refuse with `finalize.promote-clobber` (also on rc.24) |
| E2E F9 (4) | on an orphan branch the setup refusal says *"this repository has no commit yet"*; `git rev-list --count --all` is 1 |
| E2E F9 (5) | a committed link at `.jigc/AGENT.md` removed without committing the removal: `setup` exits 1 with the install written and staged, and a second run exits 1 again (the fixer's note says a second run completes) |
| E2E F9 (6) | in an autocrlf clone the first `setup` rewrites CRLF checkouts of its own files (status stays clean) and reports the guide under its user-modified advisory |

### C.9 What the auditors held, and what they did not examine

Not findings, and not counted in the 54. Each area closes with a `held` list (cells it tried to break and could not: 11 · 14 · 12 · 14 · 16 · 12) and a `not_examined` list — 11 · 8 · 12 · 10 · 9 · 9 items, and 13 under the E2E's `not_run`. **The `not_examined` lists are un-driven cells, and they also live in scratch only.** The ones that recur across areas:

- **Linux, a case-sensitive filesystem, and any git other than 2.54.0** — every drive of the audit, and of every fixer, ran on macOS (five areas and the E2E say so). CI is the first Linux run of the pass's `O_NOFOLLOW` / dev-ino / FIFO-harness code.
- **A genuine concurrent Task-tool spawn** — the E2E's fan-out rows are an N-process simulation; the genuine spawn is the orchestrator's main-session half.
- **The release-profile binary** for the cross-cutting drives (debug build, whose route fences panic where release prints).
- **Concurrency:** save-lock contention on the copy-in under a real N-agent fan-out; every external-writer race the pass deferred.
- **`jigc uninstall` from inside a fan-out worktree or a subdirectory with the log knob on**; a FIFO at `.jigc/version`; the `MigrationFixed` finalize arm; the `jigc-feedback` doctype and the two triage workflows; `jigc ingest`.
- **`L-22`'s container cell and the `git gc` permanence cell**, and `(R6, K-1)`'s timed one-shot plant, on the fixed binary.
- **The two other production `git status` probes that pass no `--untracked-files`** (`setup.rs`, `task.rs`) — worktree F1 examined one of three.
- **The full `dev/gate`** was run by no auditor; each ran its area's suites.

**At the close.** Round 4 moved three of these, as its fixers reported, and none was independently checked: one cell was driven on a **real git 2.30.9** built in a container (`f3186486`, the provision cell only); the two other flagless `git status` probes were converted with six more (`894f2234`); and each round-4 fixer and the last fixer **did** run the full gate before every commit. Everything else in these lists is as the auditors left it — and round 4 and the last fix have `not covered` lists of their own (`fixers/fixer-reports-round4.md` → each area's `must_not_refuse_cells`; `fixers/fixer-report-witness.md` → *Not covered*): Linux and Windows, a case-sensitive filesystem, a clean/smudge filter in most layouts, LFS and `working-tree-encoding`, the off-diagonal layout × conversion cells, two open tasks crossed with conversions.

**Count, table C: 54 findings** — 7 `XC` · 5 linked-worktree · 7 worktree · 10 `CPL` · 8 install · 8 reconcile · 9 `E2E`. By severity: **14 HIGH · 26 MEDIUM · 14 LOW.** **At the close: 45 closed by a fix commit** (44 `FIXED`, among them four closed only by the last fix, `0f34d8f0` — reconcile F1, F2, F7's crossing cells, E2E F5 — and 1 `CLOSED BY DECLARATION`, install F5) · **3 RECORDED** by the closing record commit (XC-1, install F7, E2E F4) · **6 OPEN** (XC-6, CPL-9, CPL-10, E2E F7, F8, F9). All fourteen HIGH findings read `FIXED`. **None of the 45 has been audited.**

## D. What the fixers left open

One row per `left_open` item of the fifteen fixer reports of rounds 1–3 — `fixers/fixer-reports.md` (rounds 1 and 2: nine reports, ten commits) and `fixers/fixer-reports-round3.md` (six reports); the reports are *leads, not verified facts* (the round-3 file's own header). **Written at `5f5b273a` and kept as written**; a row whose state moved afterwards ends in an **At the close:** note. Round 4's and the last fix's own `left_open` lists are table F. The *mark* column is the fixer's own word for the item. The last column says what, outside the fixer's own report, carries it:

- a **committed** home — a later fix commit, `DECISIONS.md`, a `decisions-pending.md` M57 row, a design doc, the review or trial README;
- or an **audit** finding (table C) — which is itself scratch, so the item still has **no committed home**; the row says which round-4 area, if any, holds it;
- or **`NOT RECORDED ANYWHERE ELSE — OPEN`**.

Row ids are `<commit>-<n>`, `n` being the item's position in that report's `left_open` list.

### D.1 `12398ddc` — `(R9, F5)`, the teardown answers for every byte inside a transient directory

| id | what | mark | covered? |
|---|---|---|---|
| `12398ddc`-1 | the shared logs owe the fork-3 ruling and the robust alternative left for 1.x | owed | COVERED BY `DECISIONS.md` (*fork `(R9, F5)`*; the as-built paragraph) and DEFERRED — M57 "`jigc uninstall`'s guard subject inverted into a writer registry" |
| `12398ddc`-2 | the pack-default `knobs.yaml` comments still say one record per invocation and list five of eight keys | left alone (pack files) | COVERED BY M57 "The pack-default `invocation-log` knob comments are stale" |
| `12398ddc`-3 | the guide batch: this commit moved one QUICKSTART sentence; if the rule is one guide commit per pass, the rest join a second | process | **NOT RECORDED ANYWHERE ELSE — OPEN.** As it turned out, eight commits of the pass touch `crates/cli/guides/` (`git log origin/main..HEAD -- crates/cli/guides`); one body-digest move per *release* still holds |
| `12398ddc`-4 | a force-added, unmodified foreign file inside `logs/`, `state/` or `index/` refuses with a message saying nothing else has a copy — an over-claim for that cell | wording, not changed | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `12398ddc`-5 | with the knob on, a refused `uninstall` still appends its record, so a deleted log reappears and blocks the next run once | human call | **FIXED `4fa1c0f5`** (queued as item 2 of `DECISIONS.md`'s top entry). Its own bound → audit XC-5 (round 4, area E) **At the close:** XC-5 FIXED `8b214ad5` — the bound is restated; no option of the four was picked. |
| `12398ddc`-6 | `flow53_acceptance` still drives `uninstall` over one subject; the class axis is pinned in `uninstall_workbench_subject.rs` instead | test coverage | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `12398ddc`-7 | `decisions-pending` (I), the footprint subtraction at `uninstall`, unchanged | still owed | COVERED BY M57 "`jigc uninstall` takes the same footprint subtraction `setup` gains at M54" |

### D.2 `dc0d7586` — `(R1, F1)`, a file the install would replace refuses on an unborn HEAD too

| id | what | mark | covered? |
|---|---|---|---|
| `dc0d7586`-1 | a pre-seeded `packs.yaml` on an unborn `HEAD` now refuses, comments or not | decision for the human to confirm | COVERED BY `DECISIONS.md` (*two rulings*, 2: kept) and M57 "A comment-preserving writer for `.jigc/config/packs.yaml`" |
| `dc0d7586`-2 | a **gitignored** file at a replacing member is destroyed at exit 0 at either `HEAD` | sibling, the human's call | **FIXED `ef9456b5`** (ruled in *two rulings*, 1). The fix's own regression and gaps → audit install F1 (area A), F5 (area E) **At the close:** install F1 FIXED `676eff57`; install F5 closed by declaration, `a330b4f9` (the layout is stated unsupported). |
| `dc0d7586`-3 | `task finalize` rewrites an uncommitted hand-edited `.jigc/version` at exit 0 | sibling at another door | **FIXED `104a7d4b`** (leave-and-say-so: `stamp_standing`, `crates/cli/src/setup.rs`, read in the tree) |
| `dc0d7586`-4 | a committed symlink at a replacing member on a born `HEAD` is written through | open fork | **FIXED `104a7d4b`** (`DECISIONS.md`, *fork on a symlink at `jigc setup`'s replacing writers*). Merged-into members still write through → audit E2E F2 (area E) **At the close:** E2E F2 FIXED `55cb29fa`. |
| `dc0d7586`-5 | on an unborn `HEAD` an untracked merged-into file rides the first commit under `--no-verify`, past a preserved user hook | declared and unchanged; no byte lost | declared exemption — *as reported* (`design/validation.md`, the `setup.dirty-install-path` row); not in the logs |
| `dc0d7586`-6 | the commit arm's path list excludes the hook: an unborn repo with an in-worktree `core.hooksPath`, an untracked foreign `pre-commit` and a refused path meets a second refusal | route bound, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (the install auditor lists the cell under `not_examined`) |
| `dc0d7586`-7 | three pre-existing things, seen and not touched: the born message says the install would *sweep* work even at a path it would replace; the commit-time backstop's route over a staged-then-recreated path takes two runs; `design/worked-examples.md` still says the suite has eighteen cells (it had 22, then 29) | pre-existing | **NOT RECORDED ANYWHERE ELSE — OPEN** (the *eighteen hand-built cells* sentence is still in `design/worked-examples.md` at `5f5b273a`) |
| `dc0d7586`-8 | `(R1, F2)` — `--force` over a dirty hook names nothing | untouched | COVERED BY the review README, `(R1, F2)` — OPEN — port |

### D.3 `c0c4d88c` — `(R6, K-1)`, a create under a `new: true` entry never reaches the copy-in

| id | what | mark | covered? |
|---|---|---|---|
| `c0c4d88c`-1 | an **edit** verb addressed at an existing finding inside a report task still copies it in | human's call (non-race) | COVERED BY M57 "Three write-door cells the pass left as they were" (a); stated in `design/findings-channel.md` §4 |
| `c0c4d88c`-2 | the same edit door over an **untracked** conformant file at a home inside a report task: `set-slot`, `task validate`, `task finalize` all exit 0; the file's marker 1 → 0, in no commit | lead for the `(R3, F7)` fixer (non-race) | partly: the review README's observation row-6 **O-2** carries the plain-entry form as *not a defect*, and the M57 row above carries the committed-finding form. **The untracked × `new: true` cell itself is in neither — OPEN.** The `eb18e5fe` report does not name it |
| `c0c4d88c`-3 | under a plain entry an occupant with a different title landing between the incumbent probe and the create is copied in with the title dropped; the suite does not pin it | ruled out (M57, tier 2) | COVERED BY M57 "The external-writer race class" |
| `c0c4d88c`-4 | a file landing at the home after the create has minted; `task finalize`'s plan→promote window | ruled out (M57), not driven | COVERED BY M57 "The external-writer race class" |
| `c0c4d88c`-5 | not driven: the `jigc-feedback` doctype at the binary; a case-sensitive filesystem; Linux (the FIFO harness ran on macOS only) | not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (the E2E's `not_run` repeats all three as still un-driven) |
| `c0c4d88c`-6 | the doc-only finalize's left-out narration calls an untracked file *a staged path stays staged* | wording nit, not touched | COVERED BY the review README → *C · OPEN leads*, the handed-between table (A H-1) |
| `c0c4d88c`-7 | owed: a `DECISIONS.md` entry for `(R6, K-1)`; the review record's `(R6, K-1)` `UNPINNED` note can be marked pinned by two named tests | owed | the entry: COVERED BY `DECISIONS.md` (the as-built paragraph). **The review record's annotation is not made — OPEN** (nothing under `completions/` changed on the branch) |

### D.4 `3f3a724b` — `(R6, D-1)`, a created doc never promotes over a file at its home at the milestone boundary

| id | what | mark | covered? |
|---|---|---|---|
| `3f3a724b`-1 | a store-aware join suffix (the join skips an occupied `<slug>-N`) | human's call | COVERED BY M57 "A store-aware join suffix, and whether `jigc milestone join` previews the boundary's refusal" (a) |
| `3f3a724b`-2 | whether `jigc milestone join` should preview the refusal | human's call | COVERED BY the same M57 row (b) |
| `3f3a724b`-3 | a **committed occupant deleted from the worktree**, with no baseline: `milestone finalize` exits 0 and the suffixed doc replaces the committed doc under its id | lead for the `(R3, F7)` fixer | stated in `design/reconciliation.md` → Open questions (committed with `8d9c3afb`); no pending row → audit CPL-5 (round 4, area C). See `8d9c3afb`-1 **At the close:** FIXED `78e8ded1` (*an identity git holds is occupied*); the design doc's open question is rewritten as closed in that commit. |
| `3f3a724b`-4 | the entry shapes `is_file()` does not see (a directory, a dangling symlink at a promote home) | for the `(R6, D-7)` fixer | **FIXED `9465f9b6`** (`engine::store::home_entry`) |
| `3f3a724b`-5 | the fan-out step prose does not state the boundary's `promote-clobber` refusal | human's call | COVERED BY M57 "The fan-out step prose does not state the milestone boundary's `promote-clobber` refusal" |
| `3f3a724b`-6 | not driven: two `created` instances of a placement singleton in one fan-out; a promotion in `merged/docs/` with no join origin; the plan-to-promote window | not driven | the first was then driven by the audit and **loses a doc at exit 0** → CPL-1 (round 4, area C). The third: M57 "The external-writer race class". **The second is NOT RECORDED ANYWHERE ELSE — OPEN** **At the close:** CPL-1 FIXED `f031d31e`. The second item is still a lead; `048724d0`'s engine cell covers a body with no origin for the staged guard only. |
| `3f3a724b`-7 | owed: a `DECISIONS.md` line for `(R6, D-1)`; the `project-history.md` fix-pass span | owed | the line: COVERED BY `DECISIONS.md`. **The span is not written — OPEN** (owed at the pass's close) |

### D.5 `9465f9b6` — `(R6, D-7)`, a managed doc lands as a regular file at its home, never through a link

| id | what | mark | covered? |
|---|---|---|---|
| `9465f9b6`-1 | `jigc rename` of a committed doc whose home is a **live** link writes the retitle through it | human's call | **FIXED `5f5b273a`** (queued as item 5 of `DECISIONS.md`'s top entry) |
| `9465f9b6`-2 | `jigc rename` **onto** an id whose home is a dangling link: refused code-less and route-less by git's own `fatal` | cheap follow-up, not done | **FIXED `5f5b273a`** (`write.already-present` at the destination, per the commit message) |
| `9465f9b6`-3 | a plain create over a body-less entry (dangling link, directory) mints fresh and is refused only at finalize | human's call | COVERED BY M57 "Three write-door cells" (b); `design/finalize.md` → *Declared bounds* (4) |
| `9465f9b6`-4 | copy-on-first-touch of a doc whose home is a live link acks the copy-in; the finalize it promises now refuses | human's call | COVERED BY M57 "Three write-door cells" (c) |
| `9465f9b6`-5 | the milestone record's later per-op writers over a record replaced with a link: not driven, not converted | not converted | **FIXED `5f5b273a`** (the record preflight and `engine::store::rewrite_home`, per the commit message) |
| `9465f9b6`-6 | a FIFO under a doctype's directory parks `task finalize` / `task validate` forever, before any planner runs | read seam, separate fix | COVERED BY M57 "Two read seams over an entry jigc cannot read"; also at `milestone finalize` → audit E2E F8 (not in round 4) **At the close:** E2E F8 is open; the M57 row is noted with the `milestone finalize` door by the closing record commit. |
| `9465f9b6`-7 | `jigc doc list` answers a code-less exit 1 over one unreadable entry; `doc show` serves a doc through a live link | read seam, pre-existing | COVERED BY the same M57 row and `design/finalize.md` → *Declared bounds* (2) |
| `9465f9b6`-8 | between the sink's re-read and its copy, `PreImage::capture` still reads through a link (all five rollback doors) | external-writer window (tier 2, M57) | COVERED BY M57 "The external-writer race class" as a class; **the seam itself is named only in the fixers' reports** |
| `9465f9b6`-9 | a symlinked **parent** directory of a home is followed as before | declared bound | COVERED BY `design/finalize.md` → *Declared bounds* (1) |
| `9465f9b6`-10 | `migrate-corpus` rewrites by temp+rename, so it replaces a link with a regular file — read, not driven | different outcome | driven and pinned by `5f5b273a` (*as reported*), left as the human's call → `5f5b273a`-3; the relocation arm → audit CPL-6 (round 4, area F) **At the close:** the relocation arm FIXED `526141e6`; the in-place arm still replaces a link unnamed → the closing section, (a). |
| `9465f9b6`-11 | not driven on Linux; CI is the first Linux run | not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (every audit area repeats it) |
| `9465f9b6`-12 | under a `new: true` entry on a fixed-identity doctype whose home is a link, the route names a doc that is not there | edge, reached by no shipped workflow | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `9465f9b6`-13 | owed: a `DECISIONS.md` line for the contract and for folding in `milestone create`; the project-history fold-back; the review README's `(R6, D-7)` entry, whose *not driven* cells are now driven | owed | the line: COVERED BY `DECISIONS.md`. **The fold-back and the README annotation are not made — OPEN** |

### D.6 `eb18e5fe` — `(R3, F7)`, a task records what it copies in

| id | what | mark | covered? |
|---|---|---|---|
| `eb18e5fe`-1 | the **milestone-record door**: in a clone a hand line appended to the record is lost at `milestone add-task`, exit 0 | human's call | **FIXED `8d9c3afb`** (queued as item 3 of `DECISIONS.md`'s top entry). The audit drove a false block at this door → reconcile F3 (round 4, area A) **At the close:** reconcile F3 FIXED `676eff57`. |
| `eb18e5fe`-2 | Detection timing's write row: *block if the doc has become conflicted since the task started* was reworded, not built | human's call | COVERED BY M57 "The write row of Detection timing" |
| `eb18e5fe`-3 | a staged doc with neither a record nor a blob at the pin still adopts — *reasoned, not driven* | residual | COVERED BY M57 "A staged doc with neither a baseline record nor a blob at the base pin still adopts". **The audit then drove it as an exit-0 loss from one task** — reconcile F2, E2E F5 (round 4, area D) — so the row's *"not driven"* sentence is now false (reconcile F6) **At the close:** **FIXED `0f34d8f0`** — the per-task record this row asked for is the copied-in witness. The M57 row is corrected by the closing record commit; its remaining bound is an area a previous binary minted. |
| `eb18e5fe`-4 | a non-jigc writer landing between one copy-in's read and another task's, or between the sweep and the promote | tier 2, external-writer class | COVERED BY the two M57 rows above |
| `eb18e5fe`-5 | a write refused after its copy-in ran prints only the refusal; the doc is staged and its baseline recorded, neither stated | tier 3 surfacing bound | COVERED BY the review README, `(R3, F3)`; stated in `design/command-output-contract.md` *as reported* |
| `eb18e5fe`-6 | the milestone-boundary conflict route ends *"…then re-run the join"* at the `milestone finalize` door | tier 3 wording lead | **NOT RECORDED ANYWHERE ELSE — OPEN** (repeated at `8d9c3afb`-6) |
| `eb18e5fe`-7 | with a baseline present, a hand edit made **before** the first write still blocks at finalize, and the route's revert exit then lands a staged copy that carries the edit | arm A unchanged by decision | the behaviour is in no log. The guide sentence that contradicts it → audit XC-3 (round 4, area D) **At the close:** **reversed by `0f34d8f0`** (accepted consequence 1): an edit made before the first write is carried into the staged copy and lands, with or without a baseline. `DECISIONS.md`, 2026-10-05, records it. |
| `eb18e5fe`-8 | the linked-worktree sibling | its own fork | **FIXED `e3a6ba58`** |

### D.7 `8c159622` — `L-22`, a door removes a worktree registration only at a path it created

| id | what | mark | covered? |
|---|---|---|---|
| `8c159622`-1 | jigc's **own** stale registration still has its `HEAD`, index and reflog dropped with the record, unnamed | human's call / the next fixer's | **FIXED `ebfc79fb`** (`DECISIONS.md`, the registration fork), and at the boundary `e842342e`; the reflog stays a bound → `ebfc79fb`-3 |
| `8c159622`-2 | row P: a locked own registration whose directory is gone — `provision` exits 0 and reports it provisioned | human's call | COVERED BY M57 "A locked registration of jigc's own whose directory is gone is still reported as provisioned" |
| `8c159622`-3 | a stale `.combine-*` record lingers as `prunable` until `uninstall` or git's expiry; a crashed finalize's **live** `.combine-*` worktree is cleaned by no door | residue only | **NOT RECORDED ANYWHERE ELSE — OPEN** (`plan-worktree` §10 assigned it a 1.x ledger row; none was written) |
| `8c159622`-4 | jigc's own records at a previous home path (a moved repository) linger as `prunable` after a `--force` re-provision | residue only | **NOT RECORDED ANYWHERE ELSE — OPEN.** Neighbour: audit worktree F4 (round 4, area F) **At the close:** the boundary in a moved repository: FIXED `7e5ba186`. The lingering records are unchanged. |
| `8c159622`-5 | `milestone.dirty-worktree`'s per-path line *"registered here, so the teardown removes it and this content is destroyed"* is false for a registered-but-unlinked directory; the leftover refusal's route does not name `git worktree repair` for a moved repository | pre-existing text, tier-3 shape | **NOT RECORDED ANYWHERE ELSE — OPEN** (the sentence is still at `crates/cli/src/milestone.rs`; the worktree auditor lists the route under `not_examined`) |
| `8c159622`-6 | staleness is read from git's `prunable` line (git ≥ 2.31); on an older git a stale own registration reads as live | bound, stated in rustdoc only | audit worktree F2 (round 4, area F) — driven under emulation as a regression **At the close:** FIXED `f3186486`, driven on a real git 2.30.9 as its fixer reported. |
| `8c159622`-7 | test-coverage caveats: one cell never observed red; row P and the stale-`.combine-*` lingering driven, not pinned; `--format json` not iterated on the remedy cells | test coverage | **NOT RECORDED ANYWHERE ELSE — OPEN** |

### D.8 `ebfc79fb` — the own-registration guard

| id | what | mark | covered? |
|---|---|---|---|
| `ebfc79fb`-1 | owed: a `DECISIONS.md` entry (the fork ruled *refuse*, the struck bound, four new bounds, the `kept/<sub-task-id>` name); the project-history span | owed | the entry: COVERED BY `DECISIONS.md`. **The span is not written — OPEN** |
| `ebfc79fb`-2 | owed: `decisions-pending.md` 1.x rows for the bounds below | owed | **partly**: three rows written (the envelope key and the reflog; the locked registration; the stdout panic). The keep-branch name, the unlinked checkout, the sibling-`HEAD` refusal and older git have none **At the close:** the missing rows are written by the closing record commit (`decisions-pending.md` → *M57*, the worktree-registration row). |
| `ebfc79fb`-3 | the reflog bound: a commit a sub-agent made and `git reset` away from is not held | human's call | COVERED BY M57 "The landed `jigc milestone finalize` envelope has no key for a dropped registration, and the reflog bound" (b) |
| `ebfc79fb`-4 | the landed envelope has no key for a dropped registration; what it held is on stderr only | human's call | COVERED BY the same M57 row (a) — **whose sentence *"is on stderr in both formats"* `e842342e` made false** (audit XC-1, not in round 4) **At the close:** the M57 row's false sentence is corrected by the closing record commit. |
| `ebfc79fb`-5 | a pre-landing refusal at `milestone finalize` | human's call | **FIXED `e842342e`** (queued as item 4 of `DECISIONS.md`'s top entry) |
| `ebfc79fb`-6 | the keep branch name `kept/<sub-task-id>`: the printed command fails if that branch, or one named `kept`, exists | human's call, not driven | changed by `e842342e` (the name is chosen against existing refs). Still exits 128 across a case-only difference → audit worktree F6 (round 4, area F) **At the close:** worktree F6 FIXED `ad1d2ef7`. |
| `ebfc79fb`-7 | an unlinked checkout whose registration holds staged paths: the refusal prints no re-link command | lead | changed by `e842342e` (a printed re-link where no `.git` entry exists). A present-but-unreadable `.git` still gets none → audit worktree F5 (round 4, area F) **At the close:** worktree F5 FIXED `f1b84ef1` — the cell is said on the line; no overwrite is printed, by rule. |
| `ebfc79fb`-8 | sibling-`HEAD` false refusal: reachability is asked under `--single-worktree` | lead | **NOT RECORDED ANYWHERE ELSE — OPEN** (restated at `e842342e`-9) |
| `ebfc79fb`-9 | older git not driven (Docker was down): the recipe needs `git restore` (≥ 2.23); all driven on 2.54.0 only | not driven | **NOT RECORDED ANYWHERE ELSE — OPEN.** audit worktree F2 emulated one old-git cell; no real old git was run by anyone **At the close:** `f3186486` drove a real git 2.30.9 for the provision cell only (as reported). The recipe's `git restore` (git ≥ 2.23) and the rest are still un-driven on an old git. |
| `ebfc79fb`-10 | the binary panics with *Broken pipe* when its stdout is closed early | lead, pre-existing | COVERED BY M57 "`jigc` panics when its stdout is closed early" |
| `ebfc79fb`-11 | a locked own registration whose directory is gone is still reused by `provision` | pre-existing, unchanged | COVERED BY M57 "A locked registration of jigc's own…" |
| `ebfc79fb`-12 | `uninstall` with `.jigc/` already gone drops no registration, refuses over none, leaves them for git to expire | pre-existing, now stated | stated in the design doc *as reported*; not in the logs |
| `ebfc79fb`-13 | `provision`'s heading still says *"would delete N path(s)"* over a path that is only a registration | wording | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `ebfc79fb`-14 | the gate's test step ran 497–504 s | gate timing | audit XC-6 (not in round 4) |

### D.9 `a88f71cc` · `e3a6ba58` — the linked-worktree guard and its prerequisite

| id | what | mark | covered? |
|---|---|---|---|
| `e3a6ba58`-1 | the second commit: `a88f71cc` fixes a defect outside the handed finding, separately gated so it can be reverted alone | human's call | COVERED BY `DECISIONS.md` (the as-built paragraph). Its second reader is still open → audit linked-worktree F2 (round 4, area B) **At the close:** linked-worktree F2 FIXED `df58d1dc`. |
| `e3a6ba58`-2 | `task-discard.staged-prose` and `uninstall.staged-prose` still route at `jigc task finalize <id>`, which in the backstop state now refuses with the guard — two hops | read, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e3a6ba58`-3 | the `doc-code.symbol-exists` route still says *update the citation* from a code-only checkout, where it meets the guard | not changed | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e3a6ba58`-4 | the relocation route when the worktree's `HEAD` is detached omits the merge step | read, not driven | audit linked-worktree F5 (b) drove the detached state (round 4, area B) **At the close:** FIXED `d8f9907d` (`TaskHere::Detached`). |
| `e3a6ba58`-5 | `git stash` in the route takes all tracked uncommitted changes; a pop into a dirty main checkout can conflict | not driven with a dirty main | **NOT RECORDED ANYWHERE ELSE — OPEN** (the linked-worktree auditor lists *stash-route conflict cases* under `not_examined`). Neighbour: linked-worktree F4 **At the close:** linked-worktree F4 FIXED `ac0f63b1`; from a fan-out worktree no stash is printed (`e0f00278`). A pop into a dirty main checkout is still not driven. |
| `e3a6ba58`-6 | the bare-repository layout: from a worktree of a bare repo every door answers *isn't set up* | pre-existing; the human's call | COVERED BY M57 "A worktree of a bare repository answers *isn't set up* at every door" — **which the audit says is a false record** for the pointer-to-bare layout: linked-worktree F1 (round 4, area B), E2E F1 **At the close:** FIXED `55a6e281` — the guard is silent in those layouts and behaviour is rc.24's; the M57 row is corrected by the closing record commit. |
| `e3a6ba58`-7 | do `jigc rename`, `migrate-corpus` and the milestone record commits *say* they acted in the main checkout when run from a linked worktree | law-1 lead, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (the audit drove that they *act* there, not what they print) |
| `e3a6ba58`-8 | `(R1, F5)` — the hook's nested `validate` asks about the main checkout | unchanged | COVERED BY the review README, `(R1, F5)`; `ideas/linked-worktree-doc-work.md`, open question 3 |
| `e3a6ba58`-9 | a sub-task's doc write typed from a user-made worktree is exempt by the predicate | read, not driven | the cross-cutting auditor drove the exemption's milestone verbs from a user-made linked worktree and held it (`audit/audit-findings.md`, *held*) — scratch; **the sub-task doc write itself is named as driven by no one** |
| `e3a6ba58`-10 | ordinary-model doc workflows still mint from a linked worktree and refuse at the first write | as ruled | COVERED BY `DECISIONS.md` (*fork on doc work from a user-made linked worktree*) |

### D.10 `104a7d4b` — a file jigc replaces is written as a regular file, never through a link (round 3)

| id | what | mark | covered? |
|---|---|---|---|
| `104a7d4b`-1 | owed in `DECISIONS.md`: (a) finalize is leave-and-say-so, not refuse; (b) directories on the way are in the rule; (c) the guide's leaf link is answered by its oracle; (d) the setup route says commit the removal, with no *put your own file there* arm; (e) the pinned cells that moved | owed | **not written.** audit XC-1, install F7, E2E F4 — none in round 4 **At the close:** written by the closing record commit (`DECISIONS.md`, the round-3 as-built entry). |
| `104a7d4b`-2 | a **tracked install path deleted before the run** passes the pre-write gate; the install is written and the commit-time backstop then refuses over the path jigc just wrote | pre-existing wedge; the human's call | named by audit install F7 (not in round 4); a variant driven at E2E F9 (5), where the second run does **not** complete **At the close:** still open; taken by no round → the closing section, (a). |
| `104a7d4b`-3 | a linked `.claude/skills` cannot take the install at all (exit 1 before and after) | not built | named by audit install F7 (not in round 4) **At the close:** still open → the closing section, (e). |
| `104a7d4b`-4 | the guide advisory's wording is loose for a link | not reworded | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `104a7d4b`-5 | a FIFO at `.jigc/version`: `read_version_stamp` still does a plain read, which would block at any door running the store sweep | reasoned, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (the install auditor lists it under `not_examined`; the M57 read-seams row names a different seam) |
| `104a7d4b`-6 | a write-time link fails the write under the member's code with the *"ensure … is writable"* route, imprecise for a link | reachable only by a race | the race: M57 "The external-writer race class". **The route wording is NOT RECORDED ANYWHERE ELSE** |
| `104a7d4b`-7 | the two codes are spelled twice (the install table; `write_install_span`'s error mapping) with nothing fencing the two spellings | not fenced | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `104a7d4b`-8 | `milestone finalize` neither stages nor writes the stamp | read, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `104a7d4b`-9 | the ruling that a gitignored regular file at a replaced install path refuses | still unbuilt | **FIXED `ef9456b5`** |

### D.11 `ef9456b5` — an install path is asked what it holds where git status cannot say (round 3)

| id | what | mark | covered? |
|---|---|---|---|
| `ef9456b5`-1 | git cannot answer (`InstallSubject::Unknown`): with no git on `PATH`, tracked-modified prose is destroyed and the run exits 1 at the hook | **decision for the human** | audit install F2 (round 4, area E — driven there as an **exit-0** loss under two other triggers) and XC-2 (not in round 4). No pending row **At the close:** **FIXED `f0b1120a`** — refuses before the first write under the existing `setup.dirty-install-path`; `--force` does not pass it (the fixer's reading). |
| `ef9456b5`-2 | the index-flag class (assume-unchanged, skip-worktree) was converted **including at merged-into members**, which the handed decision said are unchanged | **confirm** | **no ruling recorded.** audit XC-1 (4), install F7 — not in round 4 **At the close:** kept by `676eff57`, at replaced and merged-into members alike; still never confirmed → the closing section, (a). |
| `ef9456b5`-3 | a build older than this rule installed into an ignored path, then a fixed build with a different generated body: one refusal, cleared by `--force` or deleting the file | stated bound | *as reported* stated in the design row; not in the logs |
| `ef9456b5`-4 | the flagged hook is not asked: a hidden edit in an assume-unchanged in-worktree `pre-commit` can still ride the install commit | not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (install `not_examined`) |
| `ef9456b5`-5 | a status-visible replaced path holding jigc's own bytes still refuses | not widened | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `ef9456b5`-6 | a skip-worktree install path makes `git add` refuse on any run that reaches the commit | pre-existing, not touched | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `ef9456b5`-7 | a run that fails mid-span after rewriting a flagged tracked path records nothing for it; a re-run would refuse over jigc's own bytes | edge not covered | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `ef9456b5`-8 | owed: a `DECISIONS.md` line, the project-history span, a trigger row for the `Unknown` arm if deferred | owed | **not written.** audit XC-1, XC-2, install F7 — none in round 4 **At the close:** written by the closing record commit; the `Unknown` arm is gone (`f0b1120a`). |

### D.12 `4fa1c0f5` — the teardown never starts the invocation log it blocks over (round 3)

| id | what | mark | covered? |
|---|---|---|---|
| `4fa1c0f5`-1 | the bound of the ruling: two of the teardown's four refusals route through another jigc verb, which re-mints the log; four options, none picked | **human call** | stated in `design/measurement.md` item 7 and `uninstall --help` (committed with the fix). The audit says the stated bound **undercounts** → XC-5 (round 4, area E) **At the close:** XC-5 FIXED `8b214ad5`; no option was picked → the closing section, (a). |
| `4fa1c0f5`-2 | `DECISIONS.md` owes an entry: the rule is per verb, not per outcome; the bound; the QUICKSTART byte moved again | owed | **not written.** audit XC-1, install F7 — not in round 4 **At the close:** written by the closing record commit. |
| `4fa1c0f5`-3 | the verb of a clap-rejected argv is read by walking the clap tree: `jigc --format uninstall …` (a usage error) is read as the teardown; the error is one-sided | bound, not changed | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `4fa1c0f5`-4 | the pack-default `knobs.yaml` comments; `MIGRATING.md` says *"every jigc run appends one record"* | left alone | the comments: M57 "The pack-default `invocation-log` knob comments are stale". **The `MIGRATING.md` sentence is NOT RECORDED ANYWHERE ELSE** |
| `4fa1c0f5`-5 | an `uninstall` run from inside a fan-out worktree or a subdirectory with the knob on | not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (install and E2E both list it as still un-driven) |

### D.13 `8d9c3afb` — a record door with no baseline compares the record against HEAD (round 3)

| id | what | mark | covered? |
|---|---|---|---|
| `8d9c3afb`-1 | the **absent-home cell**: a committed doc deleted from the worktree, no key — a created doc lands under its id at exit 0 at `task finalize`, `milestone finalize` and `milestone create`; `jigc unmanage` is the printed way into the state. Three options, none picked | **human's call** | stated in `design/reconciliation.md` → Open questions (committed with the fix). No pending row → audit CPL-5 (round 4, area C) **At the close:** **FIXED `78e8ded1`**, at all three doors, on the ruling *an identity git holds is occupied*. |
| `8d9c3afb`-2 | `(R3, F4)`: a pulled edit to the record still conflict-blocks at the record door with a held baseline, and the next unrelated finalize still absorbs it | tier 2, unchanged | COVERED BY the review README, `(R3, F4)` — OPEN — port |
| `8d9c3afb`-3 | a record with neither a key nor a blob at `HEAD` (never committed; an unborn `HEAD`), or any git failure, still adopts | residual; reasoned, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (the M57 residual row is the task door's, not this door's) |
| `8d9c3afb`-4 | a record file absent on disk at the non-create doors; a sub-task `task discard` whose record file is deleted discards the area without settling the record | not converted; one cell reasoned, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `8d9c3afb`-5 | the either-form pin read: a file matching neither the checked-out form nor the raw blob reads as an edit; only `eol=crlf` driven | bound | the audit drove the third form and a false block → reconcile F3 (round 4, area A) **At the close:** FIXED `676eff57` — the seam no longer compares bytes at all. |
| `8d9c3afb`-6 | the boundary conflict route ends *"then re-run the join"* | tier 3 wording lead, untouched | **NOT RECORDED ANYWHERE ELSE — OPEN** (= `eb18e5fe`-6) |

### D.14 `e842342e` — the boundary refuses before it lands without work a sub-task worktree holds (round 3)

| id | what | mark | covered? |
|---|---|---|---|
| `e842342e`-1 | owed: a `DECISIONS.md` entry — the pre-landing refusal; **the new code `milestone.unlanded-work` and why no existing code fit**; the struck sentence; the settled-sub-task cells; the keep-name and re-link rules; the project-history span. (The report's notes add a third thing to rule on: `FINALIZE_DOOR.codes` is no longer empty, so a `flow53_acceptance` assertion was re-derived per door) | owed | **not written.** audit XC-1, worktree F7, E2E F4 **At the close:** written by the closing record commit — the code, why no existing code fit, and whose call the pre-landing refusal was. |
| `e842342e`-2 | owed: `decisions-pending.md` 1.x rows for the bounds below | owed | **not written** **At the close:** written by the closing record commit. |
| `e842342e`-3 | a sub-task settled by `task discard` whose **live** worktree still has staged paths now refuses at finalize (was exit 0); never put to the human by name | **human's call** | **no ruling recorded.** audit XC-1 (4) names it — not in round 4 **At the close:** still no ruling → the closing section, (a). |
| `e842342e`-4 | where the consent for a settled sub-task's worktree belongs — `task discard` asks nothing about it; finalize is now the door that refuses | **human's call** | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e842342e`-5 | the bytes leg for a **settled** sub-task's worktree at finalize: its unstaged and untracked files are still destroyed by the landed teardown and named afterwards | **human's call** | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e842342e`-6 | an un-concluded git operation (paused rebase, bisect) in a settled sub-task's worktree is torn down by a landed finalize, narrated, not refused | lead; read, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e842342e`-7 | a sub-agent that committed on a **branch** inside its worktree: nothing is held, and the boundary lands without that work saying *nothing staged* | lead | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e842342e`-8 | `jigc task validate <sub-task-id>` does not preview `milestone.unlanded-work` | bound; an undecided fork if wanted | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e842342e`-9 | the sibling-`HEAD` false refusal now has no `--force` at finalize; the keep command clears it | bound, changed in consequence; not driven at finalize | **NOT RECORDED ANYWHERE ELSE — OPEN** (= `ebfc79fb`-8) |
| `e842342e`-10 | not driven: a settled sub-task's worktree this repository has not registered; git older than 2.54.0 | not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e842342e`-11 | the keep name when a branch sits **beneath** it (`kept/<id>/x`): driven by hand, no suite cell | not pinned | **NOT RECORDED ANYWHERE ELSE — OPEN** (neighbour: worktree F6, the case-only collision) **At the close:** `ad1d2ef7`'s cells include a branch beneath the name in another letter case (`KEPT/<id>/older`), as its fixer reported. |
| `e842342e`-12 | the discard staged-prose route still describes finalize as refusing under two conditions; a third now exists | wording | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `e842342e`-13 | the gate's test step ran 554 s, close to the foreground ceiling | gate timing | audit XC-6 (not in round 4) |

### D.15 `5f5b273a` — a door that writes a committed doc in place never writes through or moves a link (round 3)

| id | what | mark | covered? |
|---|---|---|---|
| `5f5b273a`-1 | the source code is the fixer's resolution of the brief: `finalize.promote-clobber` at `jigc rename` (own home, referrer) and at the move primitive | **confirm** | **no ruling recorded.** audit CPL-9 (not in round 4); its JSON shape → CPL-7 (round 4, area C) **At the close:** CPL-7 FIXED `c97b8eb6`; the code itself is still unconfirmed → the closing section, (a). |
| `5f5b273a`-2 | the record doors' code, `reconciliation.conflict-block`, was the fixer's choice | **confirm** | **no ruling recorded.** audit CPL-9 (not in round 4) |
| `5f5b273a`-3 | `jigc migrate-corpus` over a doc whose home is a link **replaces the user's link without naming it**; pinned by a test | **human's call** | **no ruling recorded.** audit CPL-9 (not in round 4); the relocation arm → CPL-6 (round 4, area F) **At the close:** the relocation arm FIXED `526141e6`; the in-place arm is unchanged → the closing section, (a). |
| `5f5b273a`-4 | the move-primitive refusal turns two previously-succeeding commands into refusals over a link source (`config set docs-root` / `placement-root`, `jigc relocate`) | the fixer's conversion, open to overturn | `DECISIONS.md`'s top entry weighs it for `rename` only (*"3, 4 and 5 each make a command that exits 0 today refuse"*). The two config doors and `relocate`: audit XC-1 (4), CPL-9 — not in round 4 |
| `5f5b273a`-5 | under `config.repoint-failed` a per-doc cause that is itself a finding prints nested, with two `at:` and two `route:` lines | rendering, pre-existing shape | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `5f5b273a`-6 | `config set docs-root`'s loop silently skips a dangling link under a doctype directory; the link stays at the old home | not changed | **NOT RECORDED ANYWHERE ELSE — OPEN** (the create-promote-link auditor lists it under `not_examined`) |
| `5f5b273a`-7 | config-family writers under `.jigc/config/**` (`config set`'s manifest, `insert-step` / `replace-step`, workflow files) are still plain `fs::write` | out of this class, not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** (install `not_examined`: *`jigc config set` as a replacing writer through a link*) |
| `5f5b273a`-8 | still open from `9465f9b6`: a plain create over a body-less entry; copy-on-first-touch over a live link; the FIFO read seam; `doc list`'s code-less exit 1; the rollback's follow-links `PreImage::capture` | carried | COVERED BY M57 "Three write-door cells" (b), (c) · "Two read seams" · "The external-writer race class" |
| `5f5b273a`-9 | not driven on Linux | not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| `5f5b273a`-10 | owed: a `DECISIONS.md` line for the contract extending to the store doors, the two code choices, the move-primitive conversion and the migrate-corpus disposition; the project-history span; pending rows for what the human defers | owed | **not written.** audit XC-1, CPL-9 — not in round 4 **At the close:** written by the closing record commit. |

**Count, table D: 132 rows** — rounds 1 and 2: 81 (7 · 8 · 7 · 7 · 13 · 8 · 7 · 14 · 10); round 3: 51 (9 · 8 · 5 · 6 · 13 · 10).

## E. What the planners and advocates left open

From the four plans (`planning/plan-*.md`) and the five robust-case advocacies (`planning/advocate-*.md`), written before any fix. **Written at `5f5b273a` and kept as written**, with **At the close:** notes as in table D. All five forks went the advocate's way: `install-1` refuse, `install-2` folded in, `promote-1` the copy-in record, `promote-2` arm A now, `worktree-1` refuse (`DECISIONS.md`, the 2026-10-04 fork entries) — so most of what each advocate listed under *what the narrow option leaves* was closed by the arm taken; the rows below are what a plan or an advocacy named as left, un-driven or un-verified. Columns as table D.

**A gap in the source itself:** `plan-install.md` §8 and `plan-promote.md` §8 each say only *"in `left_open` in the structured result"*. **That structured result is not among the scratch files**, so those two plans' own left-open lists are not on disk; the rows below are reconstructed from the *Class left* paragraphs in their bodies.

### E.1 `plan-promote.md`

| id | what | mark | covered? |
|---|---|---|---|
| PP-1 | `(R3, F7)` class left: a doc untracked at the base pin (no blob to compare) | tier-2 class of ruling 4 | narrowed by the arm taken (`eb18e5fe`'s copy-in record); the residual: M57 "A staged doc with neither a baseline record nor a blob…" → audit reconcile F2, E2E F5 (round 4, area D) **At the close:** FIXED `0f34d8f0`. |
| PP-2 | `(R3, F7)` class left: the linked-worktree sibling | fork `promote-2` | **FIXED `e3a6ba58`** |
| PP-3 | `(R3, F7)` class left: the record door (*"it splices in place, it does not promote"*) | excluded | **FIXED `8d9c3afb`** — the `eb18e5fe` fixer drove it and found it re-renders over the edit |
| PP-4 | `(R6, D-1)` class left: the join's suffix assigner still mints onto an occupied id; the join preview prints a plain `← suffixed` line | 1.x | COVERED BY M57 "A store-aware join suffix…" |
| PP-5 | `(R6, D-1)`: two `created` instances of a singleton in one fan-out | reasoned, not driven | driven by the audit as an exit-0 loss → CPL-1 (round 4, area C) **At the close:** CPL-1 FIXED `f031d31e`. |
| PP-6 | `(R6, D-1)`: the plan→promote window against a non-jigc writer | ruling 4 | COVERED BY M57 "The external-writer race class" |
| PP-7 | `(R6, D-7)` class left: `doc list`'s code-less exit 1 over one unreadable entry | a read seam | COVERED BY M57 "Two read seams…" |
| PP-8 | `(R6, D-7)`: `doc show` reading through a link | left | COVERED BY `design/finalize.md` → *Declared bounds* (2) |
| PP-9 | `(R6, D-7)`: a symlinked parent directory | followed as today | COVERED BY `design/finalize.md` → *Declared bounds* (1) |
| PP-10 | `(R6, D-7)`: `PreImage::capture`'s follow semantics | *unreachable for a promote after this fix* | the class: M57 "The external-writer race class"; the seam is named in scratch only (= D `9465f9b6`-8) |
| PP-11 | `(R6, D-7)`: a link appearing between the sink's check and its copy | ruling 4 | COVERED BY M57 "The external-writer race class" |
| PP-12 | `(R6, K-1)` class left: the `write.title-ignored` probe / re-probe pair | ruling 4, tier 2 | converted under `new: true` by `c0c4d88c` (*as reported*); the plain-entry half: M57 "The external-writer race class" |
| PP-13 | `(R6, K-1)`: the untracked copy-in under a plain entry | ruling 4, tier 2 | COVERED BY the review README → *D · Observations* (row-6 O-2), as *not a defect*; no pending row names the non-race form (= D `c0c4d88c`-2) |
| PP-14 | `(R6, K-1)`: `task finalize`'s own plan→promote window | ruling 4 | COVERED BY M57 "The external-writer race class" (A RD-2) |
| PP-15 | `AMBUSH_CONTRACTS`: the `finalize.promote-clobber` row names the `task finalize` door only | one row, if the fence wants it | neighbour: M57 "The fan-out step prose does not state the milestone boundary's `promote-clobber` refusal". **The registry row itself is NOT RECORDED ANYWHERE ELSE** |
| PP-16 | the prerequisite: the base-pin blob read in its checked-out form (a `core.autocrlf=true` checkout) | commit 1 of the plan | built into `eb18e5fe` / `8d9c3afb` (`git cat-file --filters`, per `DECISIONS.md`). The audit drove a third working-file form the seam does not answer → reconcile F3 (round 4, area A) **At the close:** reconcile F3 FIXED `676eff57`: git is asked for a verdict, and no blob is read in any form. |
| PP-17 | §8 *Left open* — *"in `left_open` in the structured result"* | — | **the list is not on disk — NOT RECOVERABLE from scratch.** PP-1 … PP-15 are its reconstruction |

### E.2 `plan-install.md`

| id | what | mark | covered? |
|---|---|---|---|
| PI-1 | untracked bytes at a merged-into member ride the first commit under `--no-verify` | the declared exemption; no byte lost | declared — *as reported* (= D `dc0d7586`-5) |
| PI-2 | the born-`HEAD` symlink sibling found in planning | fork `install-2` | **FIXED `104a7d4b`**; merged-into members → audit E2E F2 (round 4, area E) **At the close:** E2E F2 FIXED `55cb29fa`. |
| PI-3 | `(R1, F2)` — `--force` over a dirty hook names nothing | already recorded | COVERED BY the review README, `(R1, F2)` — OPEN — port |
| PI-4 | the writer registry inverted over the whole tree with a source-scan fence | new mechanism — 1.x | COVERED BY M57 "`jigc uninstall`'s guard subject inverted into a writer registry" |
| PI-5 | `(R9, F3)` unless `install-1` takes the refuse arm | conditional | **FIXED `12398ddc`** (the refuse arm was taken) |
| PI-6 | `decisions-pending.md` (I), the footprint subtraction at `uninstall` | still owed at M57 | COVERED BY M57 "`jigc uninstall` takes the same footprint subtraction…" |
| PI-7 | the external-writer race | ruling 4 | COVERED BY M57 "The external-writer race class" |
| PI-8 | *"three surface defects observed in planning"* | named in §8, **itemised only in the structured result** | **the three are not on disk — NOT RECOVERABLE from scratch.** The plan's body names four candidates: the unborn refusal saying *"`HEAD` is untouched"* of a repository with no `HEAD` (closed by `dc0d7586` — the tree carries a test asserting the unborn wording does not contain it); `design/project-setup.md` G5's *"the two subtrees (c) structurally excludes"* called a miscount (the phrase is still in the doc); `design/worked-examples.md` flow 50's *"a prefix added … narrows the subject automatically"* (still in the doc); `uninstall --help`'s `--force` *"all three guards"*. Which three the planner meant, and whether the last three were revised in substance, was **not checked** |
| PI-9 | the two-step trap: following the unborn refusal by committing only the named path draws a second refusal over `CLAUDE.md`, and the stash route then strands a stash | driven in planning; the route must lead with move-out | the route leads with move-out since `dc0d7586` (*as reported*). The commit arm's other cost → audit install F8 (round 4, area E) **At the close:** install F8 FIXED `cba98c44`. |
| PI-10 | guides are embedded in the installed skill; *"one guide batch per pass"* | M53's rule | **NOT RECORDED ANYWHERE ELSE — OPEN** (= D `12398ddc`-3: eight commits moved guide bytes) |

### E.3 `plan-worktree.md`

| id | what | mark | covered? |
|---|---|---|---|
| PW-1 | jigc's own stale registration's `HEAD` / reflog / index dropped unnamed | fork `worktree-1` | **FIXED `ebfc79fb`**, `e842342e`; the reflog: M57 (the envelope-key-and-reflog row) |
| PW-2 | stale own records at a previous home path, and hand-deleted crashed `.combine-*` records, linger as `prunable` until git's gc | tier 3, bookkeeping residue; home: the 1.x ledger | **the ledger row was never written — NOT RECORDED ANYWHERE ELSE — OPEN** (= D `8c159622`-3, -4) |
| PW-3 | a crashed finalize's **live** `.combine-*` worktree is cleaned by no milestone door | tier 3, pre-existing; home: the 1.x ledger | **never written — NOT RECORDED ANYWHERE ELSE — OPEN** |
| PW-4 | the leftover refusal's route does not name `git worktree repair` for a moved repository | tier 3; home: the 1.x ledger | **never written — NOT RECORDED ANYWHERE ELSE — OPEN.** New datum from `e842342e`'s fixer, in the rustdoc at `crates/cli/src/milestone.rs`: driven, `git worktree repair` re-points every other worktree the repository registers, which is why the re-link is written by hand |
| PW-5 | `milestone.dirty-worktree`'s *"the teardown removes it and this content is destroyed"* over a registered-but-unlinked directory | tier 3, pre-existing, driven; home: the 1.x ledger | **never written — NOT RECORDED ANYWHERE ELSE — OPEN** (= D `8c159622`-5) |
| PW-6 | the external-writer race class | tier 2 by ruling 4 | COVERED BY M57 "The external-writer race class" |
| PW-7 | `O-9` — the `repo.head-detached` sentence from inside a detached foreign worktree | tier 3, not this family | COVERED BY the trial README, `O-9` — OPEN — port |
| PW-8 | the verifier's *not covered*: an in-progress operation, or `config.worktree`, in the foreign worktree | covered by construction, not by a cell | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| PW-9 | row P — a locked own registration whose directory is gone | declared | COVERED BY M57 "A locked registration of jigc's own…" |
| PW-10 | §9 commit 7: the pass's record commit carries *"the row annotations in the two rc.24 records"* | planned | **not made — OPEN** (nothing under `completions/` changed on the branch) |

### E.4 `plan-linked-worktree.md`

| id | what | mark | covered? |
|---|---|---|---|
| PL-1 | §0: *nothing was driven* — every claim about rc.24's behaviour is marked read, pinned, brief or owed-drive | bound on the plan | the fixer drove the cells (*as reported*, D.9); the auditors then drove the guard's layouts → C.2 |
| PL-2 | arm (B): a linked worktree as a real doc home, and its seven open questions | parked | COVERED BY `ideas/linked-worktree-doc-work.md` (all seven questions are in the file) and M57 "A linked worktree as a real home for doc work" |
| PL-3 | F1 — the mint: state only, or also refuse where the mint cannot land | fork for the human | COVERED BY `DECISIONS.md` (the as-built paragraph: a doc-only workflow and `jigc migrate` refuse before they mint). The mint's route from a fan-out worktree → audit XC-4 (round 4, area B) **At the close:** XC-4 FIXED `e0f00278`. |
| PL-4 | F2 — `jigc migrate` in this pass | fork | COVERED BY `DECISIONS.md` (taken) |
| PL-5 | F3 — a sentence in `AGENT.md` (moves six compose goldens); recommended no | fork | **no ruling recorded — NOT RECORDED ANYWHERE ELSE.** No pack or adapter file changed on the branch, so the bootstrap does not state the rule |
| PL-6 | F4 — the code's name, and an `Exempt` row in `AMBUSH_CONTRACTS` | fork | COVERED BY `DECISIONS.md` (`finalize.linked-worktree-doc`; the `Exempt` row) |
| PL-7 | F5 — the dangling-anchor cell: a stated bound of (A), or a route to the main checkout | fork | COVERED BY `DECISIONS.md` (the stash route). Its mint step collides → audit linked-worktree F3, E2E F6 (round 4, area B) **At the close:** FIXED `e590806f`. |
| PL-8 | submodule and `--separate-git-dir` layouts also keep `.git` as a file (parked in `ideas/monorepo-submodule-support.md`) | open question | **the guard fires in exactly those layouts** → audit linked-worktree F1 (round 4, area B), E2E F1 **At the close:** FIXED `55a6e281` — the guard is silent in exactly those layouts. |

### E.5 The advocacies

| id | from | what | mark | covered? |
|---|---|---|---|---|
| AD-1 | `advocate-install-1` | any jigc run between the `mv` and the re-run re-creates the log and refuses again — the pre-commit hook's `validate` included | real cost 3 | the teardown's own half: **FIXED `4fa1c0f5`**. The hook and the routed verbs → audit XC-5 (round 4, area E) **At the close:** XC-5 FIXED `8b214ad5` (the bound restated, not removed). |
| AD-2 | `advocate-install-1` | the route for the mixed population (the log beside an addable path) must be chosen and pinned | real cost 4 | COVERED BY `12398ddc` (the route asks `git check-ignore` per path — `DECISIONS.md`, the as-built paragraph) |
| AD-3 | `advocate-install-1` | the message must not quote a record count (the refused run appends its own) | real cost 5 | **not checked; NOT RECORDED ANYWHERE ELSE** |
| AD-4 | `advocate-install-1` | not driven: the team-layer knob; the release-posture binary; that `uninstall` is the only whole-workbench sink (cited, not re-verified) | not driven | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| AD-5 | `advocate-install-2` | the fifth cell: a committed `.jigc/version` link at `task finalize` | *one more guard or a recorded row — yours to decide* | **FIXED `104a7d4b`** (leave-and-say-so) |
| AD-6 | `advocate-install-2` | a mid-span refusal is messier than a pre-write one; check all four paths before the first write and drive the route at a member that is not the first write | cost | COVERED BY `104a7d4b` (*"before its first write"*, per the commit message; *as reported* for the drive) |
| AD-7 | `advocate-install-2` | a symlinked `CLAUDE.md` is a merged-into member *"whose writer preserves bytes (read from the code, not driven)"* | not driven | driven by the audit: it **writes through** the link → E2E F2 (round 4, area E) **At the close:** FIXED `55cb29fa`. |
| AD-8 | `advocate-install-2` | the precedent's absolute path and *"ensure `.jigc/` is writable"* route must not be copied | caution | the write-time arm still carries that route (D `104a7d4b`-6) — **NOT RECORDED ANYWHERE ELSE** |
| AD-9 | `advocate-promote-1` | cell 4 of the new writer: N sub-agents' first writes contend on one `state/file-state.json` — a new population | needs a test | **un-driven under a real fan-out** (cross-cutting `not_examined`) — NOT RECORDED ANYWHERE ELSE |
| AD-10 | `advocate-promote-1` | cell 7: after a copy-in, a later hand edit in a clone reads as drift at `jigc validate` and the pre-commit hook, not as `un-baselined` | behaviour change | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| AD-11 | `advocate-promote-1` | cell 8: the record is per path, not per task — two tasks on one doc and a hand edit between their copy-ins; *"the one place the robust option is worse on bytes"* | a real cell | COVERED BY M57 "A staged doc with neither a baseline record nor a blob…"; driven by the audit → reconcile F1, F2 (round 4, area D) **At the close:** FIXED `0f34d8f0` — the record is per task now. |
| AD-12 | `advocate-promote-1` | cell 9: tasks copied in by an older binary get only the backstop | bound | read-only in the audit (reconcile F2's fourth cell) — scratch **At the close:** still the bound, restated by `0f34d8f0`: an area a previous binary minted holds no witness. |
| AD-13 | `advocate-promote-1` | the UNKNOWN arm now compares worktree bytes with a git blob: autocrlf, eol attributes, smudge/clean filters are one false-fire family; `--filters` not driven | false block, never a loss | driven by the audit → reconcile F3, install F1 (round 4, area A) **At the close:** FIXED `676eff57`. |
| AD-14 | `advocate-promote-1` | two false surfaces stay false: Detection timing's write row, and the `un-baselined` route (*"baselined on its next author or finalize"*) | surfaces | the first: M57 "The write row of Detection timing". **The second was not checked — NOT RECORDED ANYWHERE ELSE** |
| AD-15 | `advocate-promote-1` | the doc-comment calling `read_or_copy_in` *"the only production caller of copy_in"* is wrong | wording | **not checked; NOT RECORDED ANYWHERE ELSE** |
| AD-16 | `advocate-promote-1` | the verifier's stronger design: a task-area record plus a compare at the promote | 1.x | COVERED BY M57 "A staged doc with neither…" (*what is owed: a per-task record of what was copied in*) **At the close:** **built**, `0f34d8f0`. |
| AD-17 | `advocate-promote-2` | a task carrying both staged code in the linked worktree and docs has no one-commit route; the mixed arm not driven | behaviour cost | COVERED BY `DECISIONS.md` (*the price of A, stated*). The route that moves the finalize → audit linked-worktree F4 (round 4, area B) **At the close:** linked-worktree F4 FIXED `ac0f63b1`. |
| AD-18 | `advocate-promote-2` | not verified: `--format json` on any cell; whether an existing suite promotes a doc from a linked worktree | not verified | **NOT RECORDED ANYWHERE ELSE — OPEN** |
| AD-19 | `advocate-worktree-1` | the zero-contribution route sends the reader to `jigc milestone provision` saying *"an existing one is reused"* — the command that destroyed the record | route | `provision` now refuses over a stale registration holding work (`ebfc79fb`); **the route's sentence was not checked — NOT RECORDED ANYWHERE ELSE** |
| AD-20 | `advocate-worktree-1` | not driven: the stale cell at `uninstall`; `git gc` permanence for the own-set; git older than 2.54.0 | not driven | `uninstall`: `ebfc79fb` (*as reported*). gc permanence and older git: **still un-driven** (E2E `not_run`; worktree F2 is an emulation) **At the close:** a real git 2.30.9 was driven for one cell (`f3186486`, as reported); `git gc` permanence is still un-driven. |
| AD-21 | `advocate-worktree-1` | the live sibling widens the blast radius: a sub-agent that commits instead of staging meets a refusal at `discard` and `uninstall` — *"deserves its own cell and its own ruling"* | behaviour change | COVERED BY `DECISIONS.md` (the registration fork: *"stale … or live"*) |

**Count, table E: 66 rows** — `plan-promote` 17 · `plan-install` 10 · `plan-worktree` 10 · `plan-linked-worktree` 8 · the advocacies 21.

## F. What round 4 and the last fix left open

One row per `left_open` item of the seven reports written after the audit — the six area fixers of round 4 (`fixers/fixer-reports-round4.md`, *Area 1* … *Area 6* = areas **A** … **F**) and the fixer of the last fix (`fixers/fixer-report-witness.md`) — plus the items those reports raise in their `notes` as decisions for the human, marked *(notes)*. The reports are leads, not verified facts; nothing here was re-driven. The *mark* column is the fixer's own word. Row ids are `4<area>-<n>` and `W-<n>`.

In the last column, **rc-row** is a row of `implementation/decisions-pending.md` → *Before the next release candidate — the fix pass's re-audit*, and **M57-row** a row of the same file → *M57 — the 1.x fix pass*, each named by its opening words; *(a)* … *(f)* are the groups of the closing section. **The record commit** is the commit that carries this ledger.

### F.1 Area A — `676eff57`, a comparison against git is asked of git

| id | what | mark | where it is now recorded |
|---|---|---|---|
| `4A-1` | the recorded baseline is still a hash of bytes: in a converting checkout every landed doc reads drifted at once. It is absorbed instead of blocked now, but under *external edit absorbed* for an edit nobody made, and `jigc validate` shows it as a lagging-baseline advisory | pre-existing, identical on rc.24 | `design/reconciliation.md` → Open questions (per the commit message); M57-row *Line-ending conversion* · (e) |
| `4A-2` | the record door with a **held** key after git rewrote the record in converted form (a branch switch and back, a stash): exit 1 *edited out of band*, and its route `git checkout -- <record>` restores nothing; `jigc unmanage <record>` then the door works on both binaries | not a regression and not closed — **needs a decision** on whether the record door's recorded arm may consult git | rc-row *Decisions taken inside the pass* · (a) |
| `4A-3` | two more sites with the same arithmetic, not converted: `promotion_changes_head` (fails toward *changed*) and the guide ownership digest — in an autocrlf clone of an installed repository `setup` exits 0 with a false `adapter-guide.user-modified` and never upgrades the guide | neither refuses a clean repository; rc.24-identical | the guide half is stated in `design/assistant-adapter.md` (`33620081`); M57-row *Line-ending conversion* · (e) |
| `4A-4` | `setup`'s `InstallSubject::Unknown` arm: if `git ls-files -v` fails after `git status` succeeded, the ask returns into the arm that installs blind | not handed to this area | **FIXED `f0b1120a`** — the variant no longer exists in `crates/cli/src/setup.rs` (read in the tree). This exact trigger was driven by no one |
| `4A-5` | fail-closed wording at `setup`: a flagged entry that cannot be asked about is listed under *with the change hidden by its index entry*, which is then unproven | wording; the route still lands | (e) |
| `4A-6` | the flagged / no-index-entry branch compares over a scratch index, so where git itself is inconsistent (a CRLF blob under a non-auto text attribute) it can call a file modified while a stat-clean `git status` says clean | bound; reachable only where a flag is set or the path was un-tracked | M57-row *Line-ending conversion* · (e) |
| `4A-7` | owed in the logs: the `(R3, F7)` as-built sentence on reading the pin in checked-out form; the as-built entry with the engine API move; rows for `4A-1` and `4A-2` | owed | written by the record commit |
| `4A-8` | layouts where `jigc setup` never reaches the guard on either binary: `--separate-git-dir` and a worktree of a bare repository exit 1 at `setup.install-hook`; in a submodule it installs at `<super>/.git/modules` | observed while driving; the home-derivation defect | M57-row *A worktree of a bare repository* (corrected by the record commit) · (e) |

### F.2 Area B — the linked-worktree guard, six commits

| id | what | mark | where it is now recorded |
|---|---|---|---|
| `4B-1` | owed: the pending row *A worktree of a bare repository answers isn't set up at every door* is false — after `jigc setup`, rc.24 created and finalized a doc there | owed | corrected by the record commit |
| `4B-2` | owed: as-built `DECISIONS.md` entries for the six commits | owed | written by the record commit |
| `4B-3` | `ac0f63b1`: where the main checkout's index holds entries the task's snapshot does not cover, the route re-authors instead of finalizing there — also for a task minted in the main checkout whose finalize was mistyped in the worktree, where it is safe but more work than needed. Telling the two apart needs a record of the minting checkout | **human's call**; a new persisted mechanism, not built | rc-row *Decisions taken inside the pass* · (a) |
| `4B-4` | `e0f00278`: a sub-agent in a fan-out worktree still cannot file a finding mid-milestone; the route says to hold it until the milestone lands | **human's call** — a design question for the findings channel | rc-row *Decisions taken inside the pass* · (a) |
| `4B-5` | `jigc setup` prints *installed at <dir> — the main checkout this repository's jigc install … bind to* in the pointer-to-bare and submodule layouts, where that directory is a bare repository's parent or `<super>/.git/modules` | not in this area; pre-existing (M53), in rc.24 | M57-row *A worktree of a bare repository* · (e) |
| `4B-6` | `CommitSite::differing` — the finalize ack and forecast line naming *the linked worktree* — still fires in the no-main-checkout layouts | not changed; rc.24 behaviour | M57-row *A worktree of a bare repository* · (e) |
| `4B-7` | the `checkout:` header *a task here commits here, and code only* is loose in a detached worktree and in a fan-out worktree | not changed; the audit did not flag it | (e) |
| `4B-8` | a pointer-to-bare repository with `core.bare=false`: git reports the home as a work tree, so the guard fires | not driven | (f) |
| `4B-9` | the gate's test step ran 547–581 s over the six runs | gate timing | C XC-6 · `perf/run-performance.md` |

### F.3 Area C — the milestone boundary, five commits

| id | what | mark | where it is now recorded |
|---|---|---|---|
| `4C-1` | **an index-only staged blob at the home of a doc the task *updates*** (`edited-from-base`; `git update-index --cacheinfo`, the worktree file untouched) is replaced by the promote's `git add` at `jigc task finalize`, exit 0 — the staged bytes in no commit | **driven, still exit 0**; a neighbour of CPL-5 outside the handed class — *if the exit rule is read as covering staged-only bytes, it is the next thing to close* | rc-row *An index-only staged blob* · (b) |
| `4C-2` | at the boundary the aggregate also `git add`s the milestone record, `.jigc/config` and `.jigc/.gitignore` over the combined code tree, so a sub-task worktree that staged one of those paths would have its staged content replaced as CPL-2's file was | reasoned, not driven | M57-row *The promote guards and the store doors* · (e) |
| `4C-3` | in a worktree of a bare repository, a `--separate-git-dir` checkout and a submodule the planner stats a directory that is no checkout, so neither the disk nor git is asked at the right place: the absent-home cell and the ordinary file clobber both remain, as on rc.24 | declared bound | `78e8ded1`'s commit message; M57-row *A worktree of a bare repository* · (e) |
| `4C-4` | the drop-the-mint exit discards a whole work unit by consent; no verb unstages one doc from a task. A dropped sub-task whose worktree still holds staged code then meets `milestone.unlanded-work`'s own route | a verb would be a new mechanism; the combination is not driven | M57-row *The promote guards and the store doors* · (e) |
| `4C-5` | `jigc milestone join` still lists a second singleton as `<id>-2 ← suffixed`; only the boundary refuses | stated in the help | the existing M57-row *A store-aware join suffix* (b) |
| `4C-6` | the task door's migration arm still routes a fixed-identity doc at re-slugging (`--slug <different-slug>`) | read, not driven | M57-row *The promote guards and the store doors* · (e) |
| `4C-7` | contested destinations at the task door (two doctypes of one pack sharing a `location:`) are pinned by an engine cell only | no shipped pack reaches it | M57-row *The promote guards and the store doors* · (e) |
| `4C-8` | the gate's test step read 561–579 s across the five passing runs | gate timing | C XC-6 |
| `4C-9` | owed in the logs: the as-built entries; the ruling *an identity git holds is occupied* and its consequence for the `jigc unmanage` route; the choice of `finalize.promote-clobber` for the contested and the staged cells; the engine `pub` moves; rows for `4C-1`, `4C-2`, `4C-3` and `4C-6` | owed | written by the record commit |
| `4C-10` | *(notes)* `78e8ded1` changes one flow that exits 0 on rc.24: after `jigc unmanage` confirms a deletion, a task that had minted a doc at that id is refused at finalize — *the one place the pass newly refuses a sequence jigc's own route produces* | decision for the orchestrator and the human | `DECISIONS.md`, the round-4 entry; rc-row *Decisions taken inside the pass* · (a) |
| `4C-11` | *(notes)* two existing tests encoded the defects as intended behaviour and were re-shaped; `milestone_landed_attribution`'s rule — *one manifest entry, owned by the last commit* — is no longer exercised by any test | stated | M57-row *Leads, wording and coverage* · (e) |

### F.4 Area D — the baseline record, four commits

| id | what | mark | where it is now recorded |
|---|---|---|---|
| `4D-1` | reconcile F1, F2 and E2E F5 — an exit-0 loss of a hand edit made after a task's first write, all three reproduced on the round-4 tree | *could-not-fix*: they need a per-task witness, the human decides | **FIXED `0f34d8f0`** (the human's ruling of 2026-10-05, option A) |
| `4D-2` | F8's silent sibling: where the key was already held, edit → copy-in → revert reads in sync and the undone edit lands with no advisory | same on rc.24; needs the same witness | **FIXED `0f34d8f0`** — an edit undone after the copy-in blocks (its commit message) |
| `4D-3` | the non-conformant advisory *fix the file … yours to hand-edit* is printed for a doc the task itself has staged — it invites the hand edit the promote then replaces | not reworded; belongs with the F2 decision | **FIXED `0f34d8f0`** — not printed over a doc the sweeping task has staged (its commit message) |
| `4D-4` | owed in the logs: four as-built entries; a dated correction of the M46 Increment 5 entry (*an edit after the mint is replaced without appearing in the fidelity diff*); the residual row's *not driven*; a row for F1's movers; the M46 ledger row describing the old route | owed | written by the record commit — dated brackets on the M46 entry in `DECISIONS.md` and on the M46 table's entry 7 in `decisions-pending.md` |
| `4D-5` | every reader's context prefix for an unreadable record still prints the absolute `.jigc` path | pre-existing | the class of `(R5, F7)` · (e) |
| `4D-6` | a repository converted to `--separate-git-dir` after `jigc setup` reads as *not set up* | observed, not this area's; identical on rc.24 | M57-row *A worktree of a bare repository* · (e) |
| `4D-7` | the gate's test step ran 563–569 s; *no room left for another fixer to gate in the foreground* | gate timing | C XC-6 |
| `4D-8` | *(routes driven)* the printed `jigc migrate <absolute path>` exits 1 when run from `/`: the verb discovers its repository from the working directory, as every other emitted migrate route does | bound found by driving | (e) |

### F.5 Area E — setup and uninstall, nine commits

| id | what | mark | where it is now recorded |
|---|---|---|---|
| `4E-1` | **`jigc uninstall` removes a `permissions.deny` or `permissions.allow` entry the adopter already had if it is identical to one jigc installs.** Driven on the release build: committed settings with the adopter's own `Bash(rm -rf:*)` deny and `Bash(jigc:*)` allow → `setup` → `uninstall` → both gone, and an empty `hooks.SessionStart: []` left. Visible as a modified file where the settings file is tracked; silent where it is untracked or ignored. Nothing records which entries were there first | **new finding, not fixed** — closing it needs a new record (the round's stop rule); three options; the human's call | rc-row *`jigc uninstall` removes a permission entry* · (b) |
| `4E-2` | `jigc_home` resolves to a directory that is no work tree in three layouts (submodule, `--separate-git-dir`, a worktree of a bare repository). There the pre-write dirty question is not asked, so a hand edit to a file jigc regenerates under that home is still replaced, as on rc.24, and `setup` exits 1 at its hook step in two of them after writing | pre-existing, in `repo::jigc_home`, outside this area; reach: every door in those layouts | `design/validation.md` (per `f0b1120a`'s message); M57-row *A worktree of a bare repository* · (e) |
| `4E-3` | the guide artifact in a fresh clone whose checkout git converted to CRLF reads as edited and is never refreshed | advisory, exit 0, nothing lost; identical on rc.24 | `design/assistant-adapter.md` (`33620081`); M57-row *Line-ending conversion* · (e) |
| `4E-4` | the cross-build bound of the whole-file guide oracle: a future change to the profile's guide front matter, or to the artifact's opening sentence, makes every older unedited copy read as the adopter's | fails closed; no published build affected; *needs a row keyed to the next change to either* | M57-row *Install and teardown* — keyed there to that change · (e) |
| `4E-5` | `528d0205`: a standalone hook whose jigc block cannot be found — edited inside jigc's lines, or written by another build — is **left** by the teardown with a stderr line, where it used to be deleted; `--force` removes it. Also seen, pre-existing: `setup` treats an older build's standalone hook as foreign and wraps it | the fixer's choice, *the human's to reverse*; the second half untouched | rc-row *Decisions taken inside the pass* · (a); neighbour `(R1, F3)` |
| `4E-6` | a directory or special file (not a link) at a merged-into install path still fails after the first write, under a write-permission route | pre-existing; deliberately outside the link rule | M57-row *Install and teardown* · (e) |
| `4E-7` | a `core.hooksPath` that itself points outside the repository still has jigc's block written into that directory's `pre-commit` | documented behaviour, not a link | M57-row *Install and teardown* · (e) |
| `4E-8` | a failed `remove_dir_all(.jigc)` is partial: it empties whichever child directory the read order hands it first | pre-existing; now named by the work-unit narration, not prevented | M57-row *Install and teardown* · (e) |
| `4E-9` | one full-gate run failed inside this round's own cell on `git clone` of a local repository (*failed to copy file … No such file or directory*); 0 of 16 isolated reruns reproduced it. The two clones the pass added use `--no-local`; 31 other suites clone the local way | **unexplained flake**; root cause not established | M57-row *Leads, wording and coverage* · (e) |
| `4E-10` | the gate's test step ran 573–590 s; a foreground wait timed out twice | gate budget | C XC-6 |
| `4E-11` | owed to the logs: nine as-built entries; the bound on `(R1, F1)`'s sentence; the F5 decision and its reasoning; the F2 choices; the engine's new `pub fn`; a row per item above | owed | written by the record commit |
| `4E-12` | *(notes)* `a330b4f9`: *stop calling it supported* was chosen over making the three doors agree — an adopter who gitignores `.jigc/` whole still cannot `task finalize` until the rule is dropped, now with a route that says so | *a judgement call the human may want to reverse* | `DECISIONS.md`, the round-4 entry; rc-row *Decisions taken inside the pass* · (a) |
| `4E-13` | *(notes)* `f0b1120a`: `--force` does **not** pass the git-cannot-answer refusal; and the refusal rides the existing `setup.dirty-install-path`, so a JSON driver sees one code for *dirty* and for *could not ask* | the fixer's readings, *the human's to reverse* | the same two homes · (a) |
| `4E-14` | *(notes)* `55cb29fa`: a link to an in-repository **ignored** file is refused, although a regular ignored `CLAUDE.md` is still merged into at exit 0 | a deliberate asymmetry, *the human's to reverse* | the same two homes; stated in `design/assistant-adapter.md` as the fixer reported · (a) |

### F.6 Area F — the worktree doors, ten commits

| id | what | mark | where it is now recorded |
|---|---|---|---|
| `4F-1` | owed to `DECISIONS.md`: ten as-built entries; and for worktree F7 — the queued entry's *None is built*, the fork entry's *names what it drops*, the code `milestone.unlanded-work` and its reason, and who ruled item 4 | owed | written by the record commit |
| `4F-2` | owed rows for the bounds these fixes state: (a) a leaked worktree whose milestone is already torn down is on no roster and stays registered; the `.combine-*` name is reserved by name, with no record. (b) in a moved repository the old registration is superseded, not asked, once `provision` puts a fresh worktree at the path; `provision`, `discard` and `uninstall` there still print `--force` as their only command where `git worktree repair` would mend; the landed teardown leaves a re-linked worktree registered at the old path. (c) `242341bb`: a refusal carried verbatim from another seam has no `config.repoint-failed` finding to carry the parked-file clause, and the squatter is still not moved back. (d) `526141e6`: the in-place arm still replaces a link at the doc's own home unnamed | owed bounds; (b) pre-existing | M57-rows *Worktree registrations* (a, b) and *The promote guards and the store doors* (c, d) · (e); (d) also (a) |
| `4F-3` | in a moved repository whose worktree came along with its old `.git` link the route is two steps — move the link aside by hand, then the printed re-link. Printing the overwrite would make it one and reverses the rule that no printed command overwrites a `.git` entry | **decision for the human**; the rule was kept | rc-row *Decisions taken inside the pass* · (a) |
| `4F-4` | `milestone.unlanded-work` is named in `design/finalize.md` and `design/team-ready-state.md` and not in `crates/cli/guides/MIGRATING.md` or `design/worked-examples.md`, where the sibling door codes are | not added — the guide is embedded in the installed skill | (e) (read in the tree at the close: neither file names it) |
| `4F-5` | not swept: `git diff` call sites inherit `diff.*` (`diff.relative`, `diff.orderFile`); the non-`-z` status and `ls-files` parsers inherit git's path quoting. Version assumptions read, not fixed: `git restore` in the recipe needs git 2.23; `rev-parse --path-format=absolute` in the linked-worktree guard needs 2.31 and fails closed to *no main checkout* | outside the handed mechanism | M57-row *git configuration and version assumptions* · (e) |
| `4F-6` | the route prose *commit or stash what a live worktree holds* at `discard` and `uninstall` does not work for an untracked file with a plain `git stash` (it needs `-u`) | pre-existing prose, no printed command | (e) |

### F.7 The last fix — `0f34d8f0`, the copied-in witness

| id | what | mark | where it is now recorded |
|---|---|---|---|
| `W-1` | an uncommitted hand edit made **before** the task's first write is carried into the staged copy; if the task then writes that same slot, the task's prose replaces it and the edit ends in no git object at exit 0. Already the behaviour with no baseline (rc.24 included); with option A it is the behaviour under a held baseline too | *judged inside the decision* — where accepted consequence 1 touches the exit rule; the fixer rewrote its first advisory wording for over-claiming | `DECISIONS.md`, 2026-10-05 (the witness), as a **declared behaviour**; `design/reconciliation.md` → *What a task copied in* (read in the tree) · (b) |
| `W-2` | a working area minted by a binary older than the witness keeps the recorded baseline and the base-pin backstop, with both old bounds, until its task lands or is discarded | stated bound | `design/reconciliation.md` (per the commit message); the M57-row *A staged doc with neither a baseline record nor a blob* (corrected) · (e) |
| `W-3` | the conflict's words are unchanged: in the undone-edit cell *revert the external edit* means putting the line back, and the only printed command is the discard, which drops the sole copy of that line | as instructed | the same M57-row · (e) |
| `W-4` | the milestone door with an unrelated finalize as mover is pre-empted by `finalize.base-mismatch`, so only `ingest` is a mover there | observation | (e) |
| `W-5` | an unreadable provenance manifest yields no verdicts at the previews, so `task validate` does not newly fail; the finalize itself still refuses on `finalize.provenance-io` before any write | stated | (e) |
| `W-6` | under a committed CRLF blob plus `core.autocrlf=input`, a no-change task ends in `finalize.commit-rejected` (*nothing to commit*) routed at *fix the hook's complaint* | pre-existing, not this door | M57-row *Line-ending conversion* · (e) |
| `W-7` | under `core.safecrlf=true`, `git add` of a mixed-eol file is fatal | git only, not driven through jigc | M57-row *Line-ending conversion* · (e) |
| `W-8` | the sweep skips a home it cannot read | pre-existing | neighbour of the M57-row *Two read seams* · (e) |
| `W-9` | F1's eleven key writers are *closed by construction* (the record no longer decides a held doc). Driven: two at the task door, `ingest` at the milestone door. **Not driven:** `rename`, `migrate-corpus`, `relocate`, the join's sweep, a sibling sub-task finalize | not driven | (f); the rc-row *The re-audit* |
| `W-10` | not covered: a real `git clone` for the unedited side; two open tasks crossed with conversions; LFS-style process filters, `working-tree-encoding`, Windows; submodule, bare repository plus worktree, `--separate-git-dir` | not covered | (f) |
| `W-11` | one deviation from the brief: the stored value is two hashes — blake3 of the raw bytes, and git's id for them at the doc's home — not one, because byte equality must never depend on git and the re-checkout case needs git's id | the fixer's, stated | `DECISIONS.md`, 2026-10-05 (the witness) |
| `W-12` | evidence caveats: the migration's `jigc unmanage <source>` route was driven by the suite on the debug binary and not re-driven on release; the provisioned fan-out worktree cell was driven by script on the release binary only, not in a test | stated | (f) |
| `W-13` | the gate's test step was 590 s; one discovery run took 1047 s under load. *Headroom to 600 s is gone* | test cost | C XC-6 · `perf/run-performance.md` |
| `W-14` | owed in the logs: the 2026-10-05 entry `design/reconciliation.md` already cites; the `(R3, F7)` entry's *both orders block when the key is lost*, true now only of a previous-format area; the residual row, obsolete but for that bound | owed | written by the record commit |

**Count, table F: 70 rows** — area A 8 · B 9 · C 11 · D 8 · E 14 · F 6 · the last fix 14.

## Open at the close of the session

**The complete list of what is not fixed, at `0f34d8f0`, 2026-10-05.** Everything a table above marks open, deferred, left or un-examined is in one of the groups below, by id; a reader who fixes from this section and nothing else misses no finding the gate, the audit or a fixer raised. Nothing in it is graded against the exit rule — that triage is owed (→ [README.md](README.md) → *What is owed next, in order*), and by the rule a finding blocks the 1.0.0 call only if it breaks a clause inside its scope.

**Read with it:** every `FIXED` of round 4 and the last fix is unaudited, so the re-audit can move a row from a table into this section.

### (a) Decisions taken or deferred inside the pass that were never put to the human — 20

Each is a fixer's own *human's call*, *decision for the human*, *confirm* or *the human's to reverse*, or a choice the orchestrator took under the standing rule. The draft of this ledger listed thirteen at `5f5b273a`; two of those are settled since — the absent-home cell (`8d9c3afb`-1: built at `78e8ded1` on the ruling *an identity git holds is occupied*) and the git-cannot-answer arm (`ef9456b5`-1: built at `f0b1120a`, whose own choices are item 17) — and round 4 added nine. All twenty are one row of `implementation/decisions-pending.md` (→ *Before the next release candidate*).

From rounds 1–3 (table D):

1. `104a7d4b`-2 — **the tracked-install-path-deleted wedge.** A tracked install path deleted before the run passes the pre-write gate; the install is written and the commit-time backstop then refuses over the path jigc just wrote. Closing it changes the settled M51 predicate. Driven again at E2E F9 (5), where a second run does not complete.
2. `ef9456b5`-2 — **the index-flag class at merged-into members.** Converted there too, which the handed decision said were unchanged; kept by `676eff57` on the must-not-refuse evidence; never confirmed.
3. `4fa1c0f5`-1 — **the bound of the teardown-log ruling.** Any other jigc run re-mints the log, the pre-commit hook's included; `8b214ad5` restated the bound and documented the order that ends it in one pass. Of the four options the fixer listed, none was picked.
4. `e842342e`-3 — a sub-task settled by `task discard` whose **live** worktree still has staged paths now refuses at `milestone finalize` (it exited 0); never put to the human by name.
5. `e842342e`-4 — where the consent for a settled sub-task's worktree belongs: `task discard` asks nothing about it, and finalize is the door that refuses.
6. `e842342e`-5 — a settled sub-task's unstaged and untracked files are still destroyed by the landed teardown and named afterwards.
7. `5f5b273a`-1 — **`finalize.promote-clobber` as the code** at `jigc rename` (own home, referrer) and at the move primitive — and, since round 4, for a contested destination (`f031d31e`) and a path a worktree staged (`048724d0`). Each is the fixer's choice of an existing code; `c97b8eb6` made the store-door arm a keyed envelope.
8. `5f5b273a`-2 — `reconciliation.conflict-block` as the record doors' code over a link.
9. `5f5b273a`-3 — **`jigc migrate-corpus` replaces a link at a doc's own home without naming it** — the in-place arm, pinned by a test, declared in `design/finalize.md`. `526141e6` closed the relocation arm only.
10. `e842342e`-1 — **the second new finding code, `milestone.unlanded-work`, and the pre-landing refusal it serves.** Admitted by no ruling: the refusal is the orchestrator's reading of the registration fork under the standing rule. The reason for the code is now in `DECISIONS.md` (the record commit); the call stays open to overturn.
11. `5f5b273a`-4 — **the move-primitive conversion.** `config set docs-root`, `config set placement-root` and `jigc relocate` refuse over a link source where they exited 0; the log weighed this for `rename` only.

From round 4 (table F):

12. `4A-2` — whether the record door's recorded arm may consult git (a record git rewrote in converted form while the key was held blocks, with a route that restores nothing).
13. `4B-3` — the backstop re-authors rather than finalizing where the main checkout's index holds uncovered entries; telling a mistyped finalize apart needs a record of the minting checkout.
14. `4B-4` — a sub-agent in a fan-out worktree cannot file a finding mid-milestone.
15. `4C-10` — after `jigc unmanage` confirms a deletion, a task that had minted a doc at that id is refused at finalize: the one place the pass newly refuses a sequence jigc's own route produces.
16. `4E-12` — ignoring jigc's own paths is stated **unsupported** rather than the three doors made to agree (`a330b4f9`).
17. `4E-13` — at `setup`, `--force` does not pass the git-cannot-answer refusal, and that refusal shares `setup.dirty-install-path` with the dirty one (`f0b1120a`).
18. `4E-14` — a link to an in-repository ignored file is refused while a regular ignored `CLAUDE.md` is merged into (`55cb29fa`).
19. `4E-5` — a `pre-commit` hook whose jigc block cannot be found is left by the teardown, not deleted (`528d0205`).
20. `4F-3` — in a moved repository the re-link route is two steps, the rule that no printed command overwrites a `.git` entry being kept.

Not in this count, because the log already records each as the orchestrator's call open to overturn: the three queued members that make an exit-0 command refuse (`DECISIONS.md`, 2026-10-04, *five further class members queued*, items 3–5).

### (b) New findings surfaced after the audit and not fixed — 3

1. `4E-1` — **`jigc uninstall` removes a `permissions.deny` / `permissions.allow` entry the adopter already had** if it is identical to one jigc installs. Driven on the release build by the area E fixer (the cell is named open in `528d0205`'s commit message); silent where the settings file is untracked or ignored. Closing it needs a record of what was there first.
2. `4C-1` — **an index-only staged blob at the home of a doc the task updates is replaced by the promote's `git add` at exit 0.** Driven by the area C fixer; the re-shaped rollback test walks through the state.
3. `W-1` — **a declared behaviour, not a defect, and listed so it is not rediscovered as one:** an uncommitted edit made before a task's first write is carried into the staged copy, and a write by that task to the same slot replaces it; never committed, it is then in no git object at exit 0. It is the stated consequence of the human's ruling of 2026-10-05 (`DECISIONS.md`), and the question it leaves is the exit rule's first clause — *content the user did not ask for* against *bytes no git object holds* — for the triage to read.

### (c) Audit findings no round took — 6, and one sub-item

Detail: table C, and C.8 for what each needs.

- **XC-6** · LOW — the gate's test step against the 600 s foreground ceiling: 537 s at the audit, 590 s at the close.
- **CPL-9** · LOW, (b)–(d) — two helps silent on their new link refusal; the dangling-link route at `rename` names no `HEAD` restore; empty directories after a refused `docs-root` re-point.
- **CPL-10** · LOW — engine `pub` signatures moved in a published rc crate; recorded by commit in `DECISIONS.md`, unseen by the release PR's semver check.
- **E2E F7** · LOW — new refusals at four doors (the milestone-record door, `milestone provision`, `milestone discard`, `milestone create`) reach a JSON driver as an `{error}` string with no key.
- **E2E F8** · LOW — a FIFO at a doc home hangs `jigc milestone finalize`.
- **E2E F9** · LOW — six riders: (1) `doc list` answers a raw OS error with an absolute host path over a link left at a home; (2) an absolute host path inside `promote-clobber`'s `jigc migrate <path>` route; (3) `task validate` exits 0 over an occupant finalize refuses; (4) *"no commit yet"* on an orphan branch; (5) a removed-but-uncommitted link at `.jigc/AGENT.md` wedges `setup` twice; (6) an autocrlf clone's first `setup` rewrites its own files and reports the guide as user-modified.
- **XC-1, one sub-item** — the doc comment on `contribution_label` in `crates/cli/src/render.rs` still describes a clause `e842342e` removed.

### (d) The review's and the trial's tier-2 and tier-3 rows — by reference

Recorded in full in the two committed READMEs and filed through jigc at the port (M56); none was touched except as tables A and B say. They were tiered under the rule that stood before 2026-10-04 and **have not been graded against the revised exit rule**, under which the clause a finding breaks, not its tier, decides whether it blocks.

- **The partial re-review** (`completions/artifacts/M55/per-axis-review-rc24/README.md`): of its 65 finding rows, **58 open** — all 7 tier-2 rows and 51 of the 52 tier-3 rows (two of them changed by the pass: `(R3, F3)`, `(R6, D-4)`). Beside them: 3 of the 4 re-drive leads (RD-2 deferred to M57; RD-3 changed and not re-driven), the 7 handed-between leads, the 10 open-lead blocks.
- **The blind trial** (`completions/artifacts/RC-rc24/README.md`): **7 of its 8 tiered rows** (`L-3` at tier 2, six at tier 3), all **14 untiered** rows, **11 of the 13 `O-` rows** whole and `O-2`'s `--format json` half, the **32 tooling rows** (`T-1`…`T-15`, `P-1`…`P-17`), and **6 of the 8 drift-and-owed rows** (DD-3, OW-1…OW-5).

### (e) Leads, bounds, residue and wording

Stated by a fixer, a planner or an advocate and closed by nobody. The lists from rounds 1–3 are **as they stood at `5f5b273a`** — no item was re-read at the close unless it says so — and an item round 4 or the last fix closed has been taken out.

**Leads and bounds on behaviour — rounds 1–3 (tables D and E)**

- `c0c4d88c`-2 — an edit verb over an **untracked** conformant file at a home inside a report task (`new: true`): three exit 0s, the file's content replaced, in no commit.
- `3f3a724b`-6 (second part) — a promotion in `merged/docs/` with no join origin is treated as not-created.
- `9465f9b6`-12 — under `new: true` on a fixed-identity doctype whose home is a link, the route names a doc that is not there.
- `ebfc79fb`-8 = `e842342e`-9 — the sibling-`HEAD` false refusal (reachability asked under `--single-worktree`), with no `--force` at finalize.
- `e3a6ba58`-2 — `task-discard.staged-prose` and `uninstall.staged-prose` route at a `task finalize` the guard refuses: two hops.
- `e3a6ba58`-3 — `doc-code.symbol-exists` still says *update the citation* from a code-only checkout.
- `e3a6ba58`-5 — the stash route popped into a dirty main checkout.
- `e3a6ba58`-7 — whether `rename`, `migrate-corpus` and the record commits *say* they acted in the main checkout from a linked worktree.
- `e3a6ba58`-9 — a sub-task's doc write typed from a user-made worktree: exempt by the predicate, driven by no one.
- `104a7d4b`-3 — a linked `.claude/skills` cannot take the install at all.
- `104a7d4b`-5 — a FIFO at `.jigc/version` blocks `read_version_stamp` at any door running the store sweep.
- `104a7d4b`-8 — `milestone finalize` neither stages nor writes the stamp.
- `ef9456b5`-3 — an older build's install in an ignored path, then a fixed build with a different body: one refusal.
- `ef9456b5`-4 — a hidden edit in an assume-unchanged in-worktree `pre-commit` can ride the install commit.
- `ef9456b5`-5 — a status-visible replaced path holding jigc's own bytes still refuses.
- `ef9456b5`-6 — a skip-worktree install path makes `git add` refuse on any run that reaches the commit.
- `ef9456b5`-7 — a run failing mid-span after rewriting a flagged tracked path records nothing for it.
- `4fa1c0f5`-3 — the clap-rejected argv walk reads `jigc --format uninstall …` as the teardown.
- `8d9c3afb`-3 — a record with neither a key nor a blob at `HEAD` (never committed; an unborn `HEAD`) still adopts. The *git failure* half is closed as `676eff57` reports it: a git that cannot answer refuses.
- `8d9c3afb`-4 — a record file absent on disk at the non-create doors; a sub-task `task discard` then discards the area without settling the record.
- `e842342e`-6 — a paused rebase or bisect in a settled sub-task's worktree is torn down by a landed finalize.
- `e842342e`-7 — a sub-agent that committed on a **branch** inside its worktree: the boundary lands without that work, saying *nothing staged*.
- `e842342e`-8 — `task validate <sub-task>` does not preview `milestone.unlanded-work`.
- `5f5b273a`-6 — `config set docs-root` silently skips a dangling link under a doctype directory.
- `5f5b273a`-7 — config-family writers under `.jigc/config/**` are still plain `fs::write`.
- `dc0d7586`-6 — the unborn commit arm's path list excludes the hook: a second refusal after committing.
- `dc0d7586`-5 = PI-1 · `ebfc79fb`-12 — two declared bounds (an untracked merged-into file rides the first commit under `--no-verify`; `uninstall` with `.jigc/` already gone drops no registration).
- PL-5 — the bootstrap (`AGENT.md`) does not state the linked-worktree rule; no ruling recorded.
- PW-8 — an in-progress operation or `config.worktree` in a foreign worktree: covered by construction, by no cell.
- AD-10 — after a copy-in, a later hand edit in a clone reads as drift at `validate` and the hook, not as un-baselined.
- AD-12 = `W-2` — a task area a previous binary minted holds no witness and gets the older arms.

**Leads and bounds on behaviour — round 4 and the last fix (table F)**

- `4A-1` · `4A-3` · `4A-6` · `4E-3` · `W-6` · `W-7` — **line-ending conversion:** the recorded baseline is a hash of bytes; `promotion_changes_head` and the guide digest use the same arithmetic; the scratch-index branch can disagree with a stat-clean status; a no-change task under a CRLF blob ends in `finalize.commit-rejected`; `core.safecrlf=true`.
- `4A-8` · `4B-5` · `4B-6` · `4C-3` · `4D-6` · `4E-2` — **the layouts whose home is no work tree** (a worktree of a bare repository, `--separate-git-dir`, a submodule): the guards are silent and behaviour is rc.24's, which there means the store, the planner and the install look under a directory that is no checkout.
- `4C-2` — the boundary's aggregate `git add` over the milestone record and `.jigc/config` against a worktree that staged one of those paths (reasoned).
- `4C-4` — drop-the-mint crossed with `milestone.unlanded-work` (not driven); no verb unstages one doc.
- `4C-6` · `4C-7` — the task door's migration arm for a fixed-identity doc; contested destinations at the task door.
- `4E-4` — the cross-build bound of the whole-file guide oracle.
- `4E-6` · `4E-7` · `4E-8` — a directory at a merged-into path; a `core.hooksPath` outside the repository; a partial `remove_dir_all`.
- `4F-2` — the worktree-registration bounds (a leaked worktree; `.combine-*` by name; a moved repository's superseded registration and `--force`-only routes), the `Raised` arm at `config set`, the parked squatter not moved back.
- `4F-5` — `diff.*` inheritance, path quoting in the non-`-z` parsers, and two git-version assumptions.
- `W-3` · `W-4` · `W-5` · `W-8` — the conflict's printed command in the undone-edit cell; the milestone door's one mover; an unreadable manifest at the previews; a home the sweep cannot read.
- `4D-8` — a printed `jigc migrate <absolute path>` run from outside the repository.

**Residue**

- `8c159622`-3 — a stale `.combine-*` record lingers as `prunable`; a crashed finalize's live `.combine-*` worktree is cleaned by no door (= PW-2, PW-3).
- `8c159622`-4 — jigc's own records at a previous home path linger after a `--force` re-provision (= PW-2).

**Wording**

- `12398ddc`-4 — the foreign-bytes refusal over a force-added unmodified file says nothing else has a copy.
- `dc0d7586`-7 — the born message says the install would *sweep* work at a path it would replace; the commit-time backstop's route takes two runs; a cell count in `design/worked-examples.md`.
- `eb18e5fe`-6 = `8d9c3afb`-6 — the boundary conflict route ends *"then re-run the join"* at the `milestone finalize` door.
- `8c159622`-5 — `milestone.dirty-worktree`'s *"this content is destroyed"* over a registered-but-unlinked directory; the leftover route does not name `git worktree repair` (= PW-4, PW-5).
- `ebfc79fb`-13 — `provision`'s heading says *"would delete N path(s)"* over a path that is only a registration.
- `104a7d4b`-4 — the guide advisory's wording is loose for a link.
- `104a7d4b`-6 — a write-time link fails under the *"ensure … is writable"* route (= AD-8).
- `4fa1c0f5`-4 (second part) — `MIGRATING.md`: *"every jigc run appends one record"* (still in the guide at the close).
- `e842342e`-12 — the discard staged-prose route names two of finalize's three refusing conditions.
- `5f5b273a`-5 — a nested finding under `config.repoint-failed` prints two `at:` and two `route:` lines.
- `4A-5` — *with the change hidden by its index entry* over an entry nobody could ask about.
- `4B-7` — the `checkout:` header in a detached or fan-out worktree.
- `4D-5` — an absolute `.jigc` path in the unreadable-record context prefix.
- `4F-4` — `milestone.unlanded-work` in neither the migrating guide nor the worked examples.
- `4F-6` — *commit or stash* over an untracked file.
- AD-3 · AD-14 (second part) · AD-15 · AD-19 — four sentences an advocate named and nobody checked: a record count in the log refusal; the `un-baselined` route; a doc comment on `read_or_copy_in`; the zero-contribution route's *"an existing one is reused"*.

**Not pinned, not fenced, test coverage**

- `12398ddc`-6 — `flow53_acceptance` drives `uninstall` over one subject.
- `8c159622`-7 — one cell never observed red; two driven cells unpinned; `--format json` not iterated on the remedy cells.
- `104a7d4b`-7 — the two write-failure codes are spelled twice with nothing fencing the spellings.
- `9465f9b6`-8 — `PreImage::capture` reads through a link at all five rollback doors (the class has an M57 row; the seam is named only here).
- PP-15 — `AMBUSH_CONTRACTS`' `finalize.promote-clobber` row names the `task finalize` door only.
- `4C-11` — the *one manifest entry, owned by the last commit* rule is exercised by no test since `048724d0`.
- `4E-9` — the unexplained `git clone` flake.

**Process**

- `12398ddc`-3 = PI-10 — *one guide batch per pass*: sixteen commits of the pass moved bytes under `crates/cli/guides/` (`git log origin/main..HEAD -- crates/cli/guides`), each moving the installed guide's body digest.
- PP-17 · PI-8 — two plans' own left-open lists point at a structured result that was never on disk.

### (f) What nobody examined — by reference

- **The auditors' `not_examined` and `not_run` lists** — 72 items across the seven returns (11 · 8 · 12 · 10 · 9 · 9, and 13 under the E2E's `not_run`), in `audit/audit-findings.md`; C.9 names the ones that recur.
- **Round 4's and the last fix's own `not covered` lists** — each area's `must_not_refuse_cells` field in `fixers/fixer-reports-round4.md`, and *Not covered* in `fixers/fixer-report-witness.md` (`W-9`, `W-10`, `W-12`, `4B-8`).
- **What recurs across all of them:** Linux and Windows (every drive of the pass, the audit and the fixers ran on macOS — CI is the first Linux run of the pass's `O_NOFOLLOW`, dev-ino and FIFO-harness code); a case-sensitive filesystem; any git but 2.54.0, one cell on 2.30.9 aside; a genuine concurrent Task-tool spawn, and save-lock contention under a real fan-out (AD-9); a clean/smudge filter, LFS and `working-tree-encoding` in most cells; the release-profile binary for the cross-cutting drives; `uninstall` from inside a fan-out worktree or a subdirectory with the log knob on (`4fa1c0f5`-5); `L-22`'s container cell and `git gc` permanence (AD-20) on any fixed binary; the `jigc-feedback` doctype and the two triage workflows at the binary (`c0c4d88c`-5); a settled sub-task's worktree this repository has not registered (`e842342e`-10); the keep name beneath `kept/<id>/` as a suite cell (`e842342e`-11); the team-layer knob and whether `uninstall` is the only whole-workbench sink (AD-4); `--format json` on the linked-worktree cells (AD-18).
- **Round 4 and the last fix themselves** — 36 commits, read and driven by their own fixers only.

### (g) Owed to the record itself

Not findings. The closing record commit wrote the as-built entries, the exit rule, the pending rows and the corrections; what it deliberately did not write — the pass has not landed — is the pass's fold-back: the `implementation/project-history.md` span (`3f3a724b`-7, `9465f9b6`-13, `ebfc79fb`-1 and the later reports), the verdict addendum, and the row annotations in the two rc.24 records (`c0c4d88c`-7, `9465f9b6`-13, PW-10). Until those annotations are made, tables A and B of this ledger are the only place a committed row reads as closed. The list, in order, is [README.md](README.md) → *What is owed next, in order*.

## Counts

| table | rows | of which |
|---|---|---|
| **A** — the review | **88** | 65 finding rows (7 FIXED · 2 OPEN — changed by the pass · 56 OPEN — port) · 4 re-drive leads (1 FIXED · 1 DEFERRED · 2 changed) · 7 handed-between leads · 10 open-lead blocks · 1 observations block (11) · 1 refuted block (25) |
| **B** — the trial | **77** | 8 tiered (1 FIXED) · 14 untiered · 2 refuted · 13 `O-` · 15 `T-` · 17 `P-` · 8 drift / owed (1 discharged) |
| **C** — the completion audit | **54** | 14 HIGH · 26 MEDIUM · 14 LOW — at the close **45 closed by a fix commit** (44 FIXED · 1 closed by declaration) · **3 RECORDED** · **6 OPEN** |
| **D** — the fixers' `left_open` | **132** | 81 from rounds 1–2 · 51 from round 3 |
| **E** — planners and advocates | **66** | 45 from four plans · 21 from five advocacies |
| **F** — round 4's and the last fix's `left_open` | **70** | 56 from six area reports · 14 from the last fix |
| | **487** | |

**Open at the close of the session, by group:** (a) 20 decisions never put to the human · (b) 3 new findings, one of them a declared behaviour · (c) 6 audit findings and one sub-item · (d) by reference — 58 review rows and their leads, 7 tiered and 14 untiered trial rows, 11 `O-` rows, 32 tooling rows, 6 drift-and-owed rows · (e) 69 entries (42 leads and bounds, 2 residue, 16 wording, 7 coverage, 2 process) · (f) by reference — 72 auditor items and the fixers' own lists · (g) the pass's fold-back.
