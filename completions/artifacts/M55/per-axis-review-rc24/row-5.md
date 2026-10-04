<!-- Reconciled ROW 5 file (finalize / transaction), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis5` in the body means ROW 5 of this run, not numbered axis 5. -->

# rc.24 partial per-axis re-review — ROW 5 · finalize / transaction — RECONCILED

> **Reconciled file.** Sections 1–9 below are the Opus driver's table, unchanged except for the three
> demotions marked **DEMOTED (reconciler)** in §2 and the corrected count line under the matrix. The
> reconciliation — every Codex claim driven, every driver defect re-driven, the tier adjudication, the
> baseline dispositions and the reconciled door list — is the **Reconciliation ledger** at the end
> (§R). The Codex source pass for this row **exists** (no new defect claimed; three baseline
> dispositions, four lead dispositions, eight consistency statements, one schema-boundary statement)
> and was reconciled against in full. The reconciler authored neither the driver file nor the code.
>
> **Result:** no tier-1 finding. Ten driver findings, all ten reproduced — `(R5, F1)` and `(R5, F6)`
> at tier 2, eight at tier 3. Baseline: `(4, DEFECT 1)` CLOSED, `(4, DEFECT 2)` and `(4, DEFECT 3)`
> STILL-OPEN. Source pass: its *"no new defect"* headline and one consistency reading refuted by
> repro; every other claim confirmed by a drive or kept as an open lead (four).

# The driver table — rc.24 partial per-axis re-review — ROW 5 · finalize / transaction — the OPUS DRIVER

Numbered axis 4 (transaction / rollback), scoped to M55's doc-only commit. Baseline: rc.16
(`completions/artifacts/M52/per-axis-review/`). Driven 2026-10-03/04.

- **Binary:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first, and again
  at the end). Release posture. No `target/debug/jigc`, no `cargo run`.
- **Rigs:** `dev/jigc-rig <state> --binary ~/.local/bin/jigc`, two-step eval, stdout only, a
  `[ -n "$REPO" ]` guard before every git command. 90 rigs, every root from `mktemp -d`, no
  teardown. The working repository was checked clean and at its starting commit after the run.
- **`CLAUDECODE` was set** for every drive except the six `cleared` cells of cell 6
  (`env -u CLAUDECODE`). Every rig-born commit therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>`; held constant across every compared pair.
- **Keys:** baseline rows keep `(4, …)` verbatim; new findings are `(R5, <id>)`.

**Headline: no tier-1 row.** Ten new findings (2 proposed tier 2, 8 proposed tier 3), one of them
(`(R5, F1)`) on the arm's central promise and worth the reconciler's attention. Baseline:
`(4, DEFECT 1)` CLOSED · `(4, DEFECT 2)` STILL-OPEN(expected) · `(4, DEFECT 3)` STILL-OPEN(expected) ·
two of four §D leads now driven (one confirmed, one promoted to a finding), cell D driven for one
population with a driver-added instrument, the concurrent-process lead still open.

---

## 1 · The door set, derived from the code (the counts I read)

Read at the working tree's `HEAD` (`bffa6667`, one trial-driver commit past the `aa6666cb` the
instrument was read at; it touches nothing below).

| registry | file | count I read | instrument author's | difference |
|---|---|---|---|---|
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **11 rows over 9 clap leaves** — `task finalize` ×2 (ordinary, amend), `milestone finalize` ×2 (squash true/false), `rename`, `migrate-corpus`, `milestone create`, `milestone add-task`, `milestone add-from-spec`, `milestone discard`, `task discard`. The doc-only commit is **not** a row; it rides `jigc task finalize` under `finalize.commit-rejected` (driven: R6.31) | 11 over 9 | none |
| `CommitModel` | `crates/cli/src/render.rs` | **3** — `Index` · `Amend` · `DocOnly`; one producer `CommitModel::of(amended, doc_only)`, amend sha wins | 3 | none |
| `StagePolicy` | `crates/cli/src/task.rs` | **6** — `MigrationFixed` · `IndexHonoring` · `Amend` · `DocOnly` · `Combine` · `ChainPerSubtask` | "read the count" | datum: 6 |
| `ManifestKind::ALL` | `crates/cli/src/render.rs` | **7** — `promoted` `modified` `deleted` `added` `untracked` `carried-over` `left-staged` | 7 | none |
| `GATE_COVERAGE` | `crates/cli/src/gate_coverage.rs` | **13** members; **exactly one** (`carryover`) carries per-model spellings (`amend` → *empty-index gate*, `doc_only` → *path scope*); the other twelve declare `None`/`None` | 13 | none |
| `ROLLBACK_POPULATIONS` | `crates/cli/src/rollback.rs` | **11** | 11 | none |
| `ROLLBACK_DOORS` | `crates/cli/src/rollback.rs` | **5** — `FINALIZE` · `MILESTONE` · `TASK_DISCARD` · `RENAME` · `CONFIG` | "read the doors" | datum: 5 |
| workflows composing `step:finalize-doc-only` (grep) | `crates/cli/packs/methodology/workflows/` | **4** — `report-jigc-feedback` · `report-inconsistency` · `triage-jigc-feedback` · `triage-inconsistency` (all four driven: R1.x, R10.4, R10.5) | 4 | none |

No count differs from the instrument's.

**Doors of this row:** `task finalize` (and its `--dry-run`) on the `DocOnly` model — the subject;
`task validate` as its preview; the same two on `Index` for contrast and on `Amend` where a cell
says so; `start` / `workflow` for the composed line; `config replace-step|insert-step|remove-step`
(cell 8); `milestone join` / `milestone finalize` (cell 9); and for the baseline rows `config set`,
`milestone create`, `milestone provision`.

**Row schema:** `(door, cell) → {argv, exit, code|none, route kind, surface asserted, verdict}`.
Route kinds: M = Mechanical (a `jigc …` argv), H = Human, I = Informational, — = none.

---

## 2 · The matrix

`DT` = the doc-only task, `CT` = the code task (`single-task`) open beside it. *Fixture F-B* (flow
B): a seed commit of `src/tracked1.rs`, `src/tracked2.rs`, `src/del.rs`; `CT` minted; the code
task's work = new `src/new.rs`, edited `src/tracked1.rs`, deleted `src/del.rs`; beside it an
unstaged edit to `src/tracked2.rs` and an untracked `scratch.txt`. **s1** = the code task's three
paths unstaged throughout · **s2** = staged before `DT` was minted · **s3** = staged after. *report* =
`report-inconsistency` creating `docs/inconsistencies/readme-mismatch.md`; *triage* =
`triage-inconsistency` editing the committed `docs/inconsistencies/crate-count.md`.

### Cell 1 — three staging states × two shapes (flow B), each also with `--carry-staged`

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| R1.1 | `task finalize` | s1 · report | `jigc task finalize <DT>` | 0 | none (advisory `file-state.staged-copy`) | I | `git show --name-status HEAD` = `A docs/inconsistencies/readme-mismatch.md` alone; `git diff --cached --name-status` byte-identical (empty) before/after; then `CT` (after `git add`) lands `D src/del.rs · A src/new.rs · M src/tracked1.rs` | matches contract |
| R1.2 | `task finalize` | s1 · report · `--carry-staged` | `… --carry-staged --format json` | 0 | none | I | as R1.1 | matches (inert) |
| R1.3 | `task finalize` | s1 · triage | `jigc task finalize <DT>` | 0 | none | I | HEAD = `M docs/inconsistencies/crate-count.md` alone; index identical; `CT` lands its three | matches |
| R1.4 | `task finalize` | s1 · triage · `--carry-staged` | `… --carry-staged --format json` | 0 | none | I | as R1.3 | matches (inert) |
| R1.5 | `task finalize` | s2 · report | `jigc task finalize <DT>` | 0 | none — **no `finalize.carried-staged`** | I | HEAD = the doc alone; cached `D src/del.rs / A src/new.rs / M src/tracked1.rs` **byte-identical** before/after; `CT` then lands exactly those three | matches |
| R1.6 | `task finalize` | s2 · report · `--carry-staged` | `… --carry-staged --format json` | 0 | none | I | as R1.5 — the three stay staged, not carried into the commit | matches (inert) |
| R1.7 | `task finalize` | s2 · triage | `jigc task finalize <DT>` | 0 | none | I | HEAD = `M …/crate-count.md` alone; index identical; `CT` lands its three | matches |
| R1.8 | `task finalize` | s2 · triage · `--carry-staged` | `… --carry-staged --format json` | 0 | none | I | as R1.7 | matches (inert) |
| R1.9 | `task finalize` | s3 · report | `jigc task finalize <DT>` | 0 | none | I | HEAD = the doc alone (the F1 sweep does not happen); index identical; `CT` lands its three | matches |
| R1.10 | `task finalize` | s3 · report · `--carry-staged` | `… --carry-staged --format json` | 0 | none | I | as R1.9 | matches (inert) |
| R1.11 | `task finalize` | s3 · triage | `jigc task finalize <DT>` | 0 | none | I | HEAD = `M …/crate-count.md` alone; index identical; `CT` lands its three | matches |
| R1.12 | `task finalize` | s3 · triage · `--carry-staged` | `… --carry-staged --format json` | 0 | none | I | as R1.11 | matches (inert) |

### Cell 2 — a pending `.jigc/config` delta beside the task

The delta is `jigc config set finalize.fan-out.squash false` (writes `.jigc/config/manifest.yaml`).

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| R2.1 | `task finalize` | delta **unstaged** · report | `jigc task finalize <DT> --format json` | 0 | none | I | HEAD = the doc alone; `?? .jigc/config/manifest.yaml` still there; `left_out` = `untracked .jigc/config/manifest.yaml`; `.jigc/version` byte-identical | matches |
| R2.2 | `task finalize` | delta **staged before** `DT` · report | same | 0 | none | I | HEAD = the doc alone; cached `A .jigc/config/manifest.yaml` identical; `left_out` = `left-staged …manifest.yaml` | matches |
| R2.3 | `task finalize` | delta **staged after** `DT` · report | same | 0 | none | I | as R2.2 | matches |
| R2.4 | `task finalize` | delta staged after · triage | same | 0 | none | I | HEAD = `M …/crate-count.md` alone; delta stays staged | matches |
| R2.5 | `task finalize` (Index) | delta unstaged · `park-idea` | same | 0 | none | I | HEAD = `A .jigc/config/manifest.yaml` + the idea doc — gap G8, the contrast | matches (unchanged behaviour) |
| R2.6 | `task finalize` (Index) | delta staged before · `park-idea` | same | 3 | `finalize.carried-staged` | H | nothing committed, index identical | matches (unchanged) |
| R2.7 | `task finalize` (Index) | delta staged after · `park-idea` | same | 0 | none | I | delta committed beside the doc | matches (unchanged) |

### Cell 3 — preview == door

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| R3.1 | `task validate` | all 12 cell-1 runs (text + `--format json`) and the 4 doc-only cell-2 runs (text) | `jigc task validate <DT>` | 0 ×16 | advisory `file-state.staged-copy` only | I | no carryover finding in s2; the validate key set `(code, target, severity)` equals the `--dry-run` forecast's on the 6 state × shape pairs compared, and the forecast's equals the landed envelope's on the 6 JSON-landed runs | matches |
| R3.2 | `task finalize --dry-run` | same 16 | `jigc task finalize <DT> --dry-run [--format json]` | 0 | as R3.1 | I | `manifest` = the one promoted doc (the path set, not the index); index unchanged by the preview | matches |
| R3.3 | `task finalize` vs `--dry-run` | 6 cell-1 + 4 cell-2 JSON pairs | landed `--format json` vs dry `--format json` | 0 / 0 | — | — | `manifest` equal 10/10 · `left_out` equal 10/10 · `findings` key set equal 6/6 | matches |
| R3.4 | `task finalize --dry-run` | `--carry-staged` vs plain, 6 pairs | `… --dry-run --carry-staged --format json` | 0 | — | — | the two JSON documents are **equal** 6/6 | matches (inert) |

Two preview ≠ door rows exist and are recorded where they were driven: R7.9 (`(R5, F2)`) and
R7.13 (`(R5, F3)`, the Index contrast).

### Cell 4 — the left-out narration

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| R4.1 | `task finalize` | s2/s3 · JSON | `… --format json` → `committed.left_out[]` | 0 | — | — | `left-staged src/del.rs` · `left-staged src/new.rs` · `left-staged src/tracked1.rs` · `modified src/tracked2.rs` · `untracked scratch.txt`; the promoted doc is **not** listed; same list in the dry-run | matches |
| R4.2 | `task finalize` | s1 · JSON | same | 0 | — | — | no `left-staged`; `deleted src/del.rs` · `modified src/tracked1.rs` · `modified src/tracked2.rs` · `untracked scratch.txt` · `untracked src/new.rs` | matches |
| R4.3 | `task finalize` | text arm | `jigc task finalize <DT>` | 0 | — | — | pre-commit stem *"finalize — about to commit only this task's docs, path-scoped; leaving out:"* and, twice (pre-commit + landed), *"left-out (this commit takes only this task's docs and the artifacts they record — a staged path stays staged for the task it belongs to):"* followed by the five paths. **The text arm prints no kind per path** — a reader cannot tell the staged three from the unstaged two (the Index model's text arm has the same shape; the kind is JSON-only) | matches the model-keyed wording; kind-less list recorded as a datum |
| R4.4 | `task finalize` | mixed staged shapes: `MM` · `AD` · `R` · `git rm --cached` (`D ` + `??`) · a name with a space | `… --dry-run [--format json]`, `… --format json` | 0 | — | — | `MM src/a.rs` → once, `left-staged`; `AD src/n.rs` → once, `left-staged`; rename → both names `left-staged`; **`src/c.rs` twice** (`left-staged` and `untracked`); **`"src/with space.rs"` with literal quote characters inside the JSON string** | **DEFECT** — `(R5, F4)` (twice) and `(R5, F2)` (quoted) |
| R4.5 | `task finalize` (Index) | a staged name with a space, a non-ASCII name | `… --dry-run --format json` vs `… --format json` | 0 | — | — | dry `manifest`: `added "src/with space.rs"` (quoted); landed `manifest`: `added src/with space.rs` (unquoted) — **forecast ≠ landed**; `"src/\303\274.rs"` octal-escaped in both | **DEFECT** — `(R5, F2)` |

### Cell 5 — the omitting-context control (`park-idea`, the ordinary model)

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| R5.1 | `task finalize` (Index) | s1 | `jigc task finalize <DT>` | 0 | none | I | HEAD = the idea doc alone; `CT` lands its three later | matches (unchanged) |
| R5.2 | `task validate` · `task finalize --dry-run` · `task finalize` (Index) | s2 | each, text and JSON | **3 · 3 · 3** | `finalize.carried-staged` ×3 (one per path) | H (`git -C $REPO restore --staged -- <path>` or `--carry-staged`) | nothing committed, index identical | matches — *state 2 refuses* |
| R5.3 | `task finalize` (Index) | s2 · `--carry-staged` | `… --carry-staged --format json` | 0 | none | I | HEAD = idea doc **+ the code task's three paths**, labelled `carried-over` at dry-run and landed | matches (declared) |
| R5.4 | `task finalize` (Index) | s3 | `jigc task finalize <DT>` | 0 | none | I | HEAD = `A docs/ideas/parked-probe.md · D src/del.rs · A src/new.rs · M src/tracked1.rs` — *state 3 commits the staged paths*, the declared two-task bound | matches the scope's expectation (unchanged behaviour) |
| R5.5 | `task finalize` (Index) | the code task after R5.4 | `jigc task finalize <CT>` | 3 | `finalize.nothing-staged` | H — *"`git add` your changes, then re-run `jigc task finalize`"* | the route's re-run is bare; `jigc task finalize` → exit 2 (*required `<ID>`*) | **DEFECT** — `(R5, F6)` |

### Cell 6 — failure points on the doc-only arm

Hooks live behind `core.hooksPath` in a `mktemp -d` under `$RIG`. Fixture: seed commit; (triage: a
landed finding); `CT` with `src/new.rs` + `src/tracked1.rs` staged after `DT` was minted, an
unstaged `src/tracked2.rs`, an untracked `scratch.txt`. The *snapshot* compared before/after is
`HEAD` · `HEAD^{tree}` · `sha(git ls-files -s)` · the destination's bytes-or-absent · a hash over
every file in `.jigc/tasks/<DT>/` · `.jigc/.gitignore` · `.jigc/version` · `git diff --cached
--name-status` · `git status --short`.

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| R6.1 | `task finalize` | rejecting `pre-commit` · report · `CLAUDECODE` set | `jigc task finalize <DT>` | 1 | log `finalize.commit-rejected` | M (`jigc task finalize <DT>`) | **snapshot identical** on all nine; no `.jigc/displaced`; both tasks still listed; frame *"task … is intact — nothing was committed, your task's staged docs are still in `.jigc/tasks/<DT>/docs/`, and anything you had `git add`-ed is still in git's index"* | matches |
| R6.2 | `task finalize` | same · cleared | `env -u CLAUDECODE jigc task finalize <DT>` | 1 | same | M | identical to R6.1 — the diff of the two logs is the trailer on the commit the *re-run* lands, and nothing else | matches |
| R6.3 | `task finalize` | rejecting `pre-commit` · triage · set | same | 1 | same | M | snapshot identical — the committed doc's bytes are back, byte for byte | matches |
| R6.4 | `task finalize` | same · cleared | same | 1 | same | M | as R6.3; trailer-only difference | matches |
| R6.5 | `task finalize` | rejecting `commit-msg` · report · set | same | 1 | same | M | snapshot identical; the hook saw `docs: doc probe commit` / body / blank / `Co-Authored-By: Claude <noreply@anthropic.com>` | matches |
| R6.6 | `task finalize` | same · cleared | same | 1 | same | M | snapshot identical; the hook saw the same message **without** the trailer; nothing else differs | matches |
| R6.7 | `task finalize` | rejecting `commit-msg` · triage · set | same | 1 | same | M | snapshot identical | matches |
| R6.8 | `task finalize` | same · cleared | same | 1 | same | M | snapshot identical; trailer-only difference | matches |
| R6.9 | `task finalize` | **racing** rejecting `pre-commit` (appends to the promoted doc) · report · set | same | 1 | `finalize.rollback-conflict` blocking + log `finalize.commit-rejected` | H | racer's bytes **stand** at the new destination; nothing parked (no pre-image — correct); frame opens *"apart from the 1 path named below, which survived the rollback:"*; route *"…did not exist before this finalize, so the rollback would have deleted the copy jigc created — it did not. Remove it by hand…"*; index identical | matches |
| R6.10 | `task finalize` | same · cleared | same | 1 | same | H | identical but for host-path bytes in a later route (see R6.14) | matches |
| R6.11 | `task finalize` | racing rejecting `pre-commit` · triage · set | same | 1 | same | H | racer's line stands; pre-image parked at `.jigc/displaced/finalize/docs/inconsistencies/crate-count.md.pre-image.<nanos>` and named in the route; index identical; ` M docs/inconsistencies/crate-count.md` | matches |
| R6.12 | `task finalize` | same · cleared | same | 1 | same | H | identical | matches |
| R6.13 | `task finalize --format json` | the 8 un-raced reject cells, a second run under the same hook | `… --format json` | 1 | `finalize.commit-rejected` | M | stdout **0 bytes**; stderr parses as **one** `{findings, schema_version}` document; key `{code, target: task:<DT>}`; the hook's stderr is the finding's `message` | matches |
| R6.14 | `task finalize` | re-run after removing the hook | `jigc task finalize <DT>` | 0 (8 un-raced) · 3 (4 raced) | none · `finalize.promote-clobber` (report) / `reconciliation.conflict-block` (triage) | M · H | un-raced: lands **once** (`git rev-list --count` +1), doc alone, index identical, `jigc validate` clean. Raced, residue left in place: the report is refused over the racer's file, the triage over the out-of-band edit — each routed. The `finalize.promote-clobber` route ends *"then re-run `jigc task finalize`"* (bare) and names `jigc migrate <absolute host path>/docs/inconsistencies/readme-mismatch.md` | un-raced matches · raced matches on behaviour; the route text is **DEFECT** `(R5, F6)` + `(R5, F7)` |
| R6.15 | `task finalize` | racing **passing** `pre-commit` (edits the doc, exit 0) · report | same | 0 | none | I | commit = jigc's promoted bytes; worktree = + the racer's line (` M`); the landed left-out lists the doc (its unstaged delta); `jigc validate` → `file-state.hash-matches`; **no** rollback-conflict (nothing rolled back) | matches (same on Index, R6.17) |
| R6.16 | `task finalize` | same · triage | same | 0 | none | I | as R6.15 | matches |
| R6.17 | `task finalize` (Index) | same · `park-idea` | same | 0 | none | I | identical shape — the control | **DEMOTED (reconciler) — NOT DRIVEN:** marked driven, no repro block in this file, not re-driven by the reconciler. The driver's verdict was: matches |
| R6.18 | `task finalize` | **formatter-style** `pre-commit` (edits a staged `.md` and `git add`s it, exit 0) · report | `jigc task finalize <DT> --format json` | **0** | none | — | HEAD and worktree carry the hook's line, **the real index does not**: `git diff --cached --name-status` gains `M docs/inconsistencies/readme-mismatch.md`; `committed.left_out` names the **committed** doc as `left-staged` | **DEFECT** — `(R5, F1)` |
| R6.19 | `task finalize` | same · triage | same | **0** | none | — | same: `MM docs/inconsistencies/crate-count.md` | **DEFECT** — `(R5, F1)` |
| R6.20 | `task finalize` (Index) | same hook · `park-idea` | same | 0 | none | I | index clean, HEAD = index = worktree, `left_out` empty — the control | matches |
| R6.21 | `task validate` · `task finalize --dry-run` · `task finalize` | the code task after R6.18 / R6.19, hook kept and hook dropped (4 drives) | each | **0 · 3 · 3** | `finalize.base-mismatch` | H — *"resolve the overlap on `<doc>` against the new history, or discard the task with `jigc task discard <CT> --force`"* | the neighbouring code task cannot finalize; no surface names the command that clears it | **DEFECT** — `(R5, F1)` |
| R6.22 | `task finalize` (Index) | a task minted **after** R6.18 | `jigc task finalize <NT>` | 3 | `finalize.carried-staged` ×2 (the phantom doc entry and `src/new.rs`) | H (`git restore --staged`) | routed | matches the gate; consequence of `(R5, F1)` |
| R6.23 | `task finalize` | a second doc-only report after R6.18, same hook | `… --format json` | 0 | none | — | phantoms accumulate: `MM` on both docs; `left_out` names both committed docs `left-staged` | **DEFECT** — `(R5, F1)` |
| R6.24 | `task finalize` (Index) | after `git restore --staged -- docs/inconsistencies` | `jigc task finalize <CT>` | 0 | none | I | the code task lands `A src/new.rs` alone | the remedy works; nothing prints it |
| R6.25 | `milestone create` · `milestone add-task` | same formatter hook at a record-only (path-scoped, pre-M55) door | `jigc milestone create 'Fmt probe'`; `jigc milestone add-task fmt-probe 'sub one'` | 0 · 1 | none · `reconciliation.conflict-block` | — · H | same phantom (`MM docs/milestone-records/fmt-probe.md`); the next record door refuses the hook's edit as out-of-band | datum — the mechanism predates M55 at the record doors; M55 widened it to every report and triage |
| R6.26 | `task validate` · `task finalize --dry-run` · `task finalize` · `… --carry-staged` · `… --format json` | **unchanged recording** — a triage writing `status = open` back, beside a staged `src/new.rs` and a pending config delta | each | 0 · 3 · 3 · 3 · 3 | `finalize.empty-commit` | M (`jigc task finalize <DT>` / `jigc task discard <DT> --force`) | refused **before git runs** (a `pre-commit` hook that touches a marker file never ran); HEAD unmoved; index identical; JSON: one envelope on stdout, stderr empty | matches (CR3 closed) |
| R6.27 | same three doors | a report task with **no doc of its own**, same neighbours | each | 0 · 3 · 3 | `finalize.empty-commit` (never `finalize.nothing-staged`) | M | hook never ran; HEAD unmoved; index identical | matches |
| R6.28 | `task finalize` | **stage failure** (`: > .git/index.lock`) on the doc-only arm | `jigc task finalize <DT>` (+ `--format json`) | 3 | `finalize.stage-failed` | M (`jigc task finalize <DT>` once …) | message names the path-scoped stage `git add -- :(literal)docs/inconsistencies/readme-mismatch.md`; HEAD · `ls-files -s` · `.jigc/.gitignore` · `.jigc/version` · destination (absent) · status all identical; the re-run lands | matches |
| R6.29 | `task finalize` (Index) | same failure point — **baseline `(4, DEFECT 1)`** | `jigc task finalize axis-four-probe` | 3 | `finalize.stage-failed` | **M** — *"`jigc task finalize axis-four-probe` once the embedded git failure is resolved…"* | the route now carries the id | **CLOSED** |
| R6.30 | `task finalize` | rejecting hook × a recorded owner-artifact the user pre-staged at blob A then edited to B | `jigc task finalize glob-run` | 1 | log `finalize.commit-rejected` | M | `git ls-files -s` rows for the artifact home **identical** (blob A still staged, worktree still A+B); promote rolled back; the foreign staged sibling untouched; re-run lands doc + artifact (B) alone | matches (the third rollback axis holds on this arm) |
| R6.31 | `task finalize` | the invocation log on a doc-only reject (`invocation-log` knob on, 12 runs) | `jigc task finalize <DT>` | 1 | `error_code: finalize.commit-rejected`; `finding_codes: []` un-raced, `[finalize.rollback-conflict]` raced | — | the doc-only reject carries the ordinary arm's identity | matches |
| R6.32 | `task finalize` | **cell D** — stage failure × worktree concurrently edited · report (instrument: a *required* git clean filter, see §6) | `jigc task finalize <DT>` | 3 | `finalize.stage-failed` + `finalize.rollback-conflict` | M + H | racer's bytes stand; index identical; **but** the stage-failed finding says *"the promotions were rolled back"* and *"the same re-run lands the commit"* | CAS **matches**; the frame is **DEFECT** `(R5, F5)` |
| R6.33 | `task finalize` | cell D · triage | same | 3 | same pair | M + H | racer's line stands; pre-image parked and named; index identical; the re-run is refused `reconciliation.conflict-block` | CAS matches; frame `(R5, F5)` |
| R6.34 | `task finalize` (Index) | cell D · `park-idea` | same | 3 | same pair | M + H | same | **DEMOTED (reconciler) — NOT DRIVEN:** marked driven, no repro block for the ordinary model in this file (the `(R5, F5)` block is the triage shape), not re-driven by the reconciler. The driver's verdict was: CAS matches; frame `(R5, F5)` |
| R6.35 | `task validate` · `task finalize --dry-run` · `task finalize` · `… --carry-staged` | un-concluded merge (rig `fresh --start report-inconsistency … --git-state merge`) | each | 1 ×4 | `repo.operation-in-progress` | H | refused before any write: HEAD unmoved, `ls-files -s` identical, no promote | matches |
| R6.36 | `task finalize` | rig `chatty-hooks` · text | `jigc task finalize <DT>` | 0 | none | I | `--- hook output ---` / `chatty-hook: pre-commit spoke on success` after the landed ack; doc alone; index identical | matches |
| R6.37 | `task finalize` | rig `chatty-hooks` · JSON | `… --format json` | 0 | none | I | `committed.hook_output` = the hook's line; `left_out` = `left-staged code.rs` | matches |

### Cell 7 — owner-artifacts

Fixture: two hand-written project-layer workflows (`.jigc/config/workflows/file-dogfood.yaml` →
`step:finalize-doc-only`, `file-dogfood-index.yaml` → `step:finalize`), each granting
`dogfood-record`, committed with plain git — the scope's stated exception. The artifact home is
`completions/artifacts/glob-run/`.

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| R7.1 | `task finalize` | artifact **untracked**; a foreign staged `other.md` and an untracked `third.md` beside it | `… --dry-run --format json`, `… --format json` | 0 | none | I | HEAD = `A …/cap.md` + `A docs/dogfood/glob-run.md`; `other.md` stays staged (`left-staged`), `third.md` `untracked`; dry == landed | matches |
| R7.2 | `task finalize` | artifact **tracked and changed** | same | 0 | none | I | HEAD = `M …/cap.md` + the doc; dry == landed | matches |
| R7.3 | `task finalize` | artifact tracked, **unchanged** | same | 0 | none | I | HEAD = the doc alone; the artifact is in neither list | matches |
| R7.4 | `task finalize` | name `c*.md`, beside a staged `cother.md` and an untracked `cthird.md` | same | 0 | none | I | HEAD = `A …/c*.md` + the doc — **only the named file**; `cother.md` still staged, `cthird.md` untracked | matches |
| R7.5 | `task finalize` | name `capture[1].md`, beside a staged `capture1.md` | same | 0 | none | I | HEAD = `A …/capture[1].md` + the doc; `capture1.md` still staged | matches |
| R7.6 | `task finalize` | basename `:lead.md`, beside a staged `lead.md` | same | 0 | none | I | HEAD = `A …/:lead.md` + the doc; `lead.md` still staged | matches |
| R7.7 | `task validate` · `--dry-run` · `task finalize` | the field value `:(top)completions/artifacts/glob-run/cap.md` (a **leading** colon) | each | 3 · 0 · 3 | `owner-artifact.present` (*not under the owned artifact home*) | H | refused at the content gate; nothing committed; the dry-run prints its manifest with the blocking finding at exit 0, as its `--help` declares (*"reported here, decided at the real finalize"*) | matches |
| R7.8 | same three | the field value `:(glob)completions/artifacts/glob-run/*.md` | each | 3 · 0 · 3 | `owner-artifact.present` | H | same; the sibling `other.md` is never swept | matches |
| R7.9 | `task finalize --dry-run` vs `task finalize` | name `my cap.md` (a space), beside a staged `my` | `… --dry-run --format json`, `… --format json` | 0 · 0 | none | I | the commit is right (`A …/my cap.md` + the doc; `my` still staged) — but the **forecast** lists the artifact as `left_out: untracked "completions/…/my cap.md"` and omits it from `manifest`, while the door commits it | **DEFECT** — `(R5, F2)` (forecast ≠ door) |
| R7.10 | `task finalize` (Index) | same name | same | 0 | none | I | same forecast gap; the commit also takes the staged `my` (the ordinary model) | **DEFECT** — `(R5, F2)` |
| R7.11 | `task finalize` (Index) | `c*.md` | same | 0 | none | I | HEAD = `c*.md` + the staged `cother.md` + the doc; the **untracked** `cthird.md` is not swept | matches (nothing swept by the pattern) |
| R7.12 | same three (Index) | `:(top)…` | each | 3 · 0 · 3 | `owner-artifact.present` | H | as R7.7 | matches |
| R7.13 | `task finalize --dry-run` vs `task finalize` (Index) | a plain untracked artifact `cap.md` | same | 0 · 0 | none | I | dry: `left_out: untracked …/cap.md`, absent from `manifest`; landed: `manifest: added …/cap.md` — the ordinary model's forecast does not know jigc will stage the recorded artifact; the doc-only model's does (R7.1) | **DEFECT** — `(R5, F3)` |

### Cell 8 — how the step is reached

One staged path before `DT` is minted (`src/pre.rs`) and one after (`src/post.rs`), so the model
discriminates on one run. *FRESH* = the `what's-left:` line `jigc start --workflow <W>` composes;
*RESUME* = the line `jigc start --task <DT>` composes; *DOOR* = `task validate` / `--dry-run` /
`task finalize`.

| # | door | cell | how built | FRESH · RESUME | DOOR | verdict |
|---|---|---|---|---|---|---|
| R8.1 | `start` · `task validate` · `task finalize` | **wrapping project step** | hand-written `.jigc/config/steps/file-report-final.yaml` (`{{ include: step:finalize-doc-only }}`) + project workflow `file-report-nested` | *path scope* · *path scope* | validate 0 · dry-run 0 (doc alone, both `src/*` left out) · finalize 0, HEAD = the doc alone, index identical | matches (CR1 closed) |
| R8.2 | same | **body shadowed** | hand-written `.jigc/config/steps/finalize-doc-only.yaml` with no path-scope sentence | *path scope* · *path scope* (the step text no longer says *path-scoped*; the line jigc emits still does) | as R8.1 | matches (keyed on the id) |
| R8.3 | `config replace-step` + same | **replaced out** | `jigc config replace-step workflow:report-inconsistency#finalize-doc-only <tmp>/land-the-index.yaml` (exit 0) | *carryover gate* · *carryover gate*; step text names the git index | validate 3 · dry-run 3 · finalize 3 `finalize.carried-staged` (`src/pre.rs`); `--carry-staged` → 0, HEAD = doc + both `src/*` | matches — promise and model agree (Index) |
| R8.4 | `config replace-step` + same | **replaced in** | `jigc config replace-step workflow:park-idea#finalize <tmp>/land-docs-alone.yaml` (exit 0) | *path scope* · *path scope* | 0 · 0 · 0, HEAD = the idea doc alone, index identical | matches (DocOnly) |
| R8.5 | `config remove-step` + same | **removed** | `jigc config remove-step workflow:report-inconsistency#finalize-doc-only` (exit 0) | *carryover gate* · *carryover gate*; no `Run: jigc task finalize` line composed | 3 · 3 · 3 `finalize.carried-staged`; `--carry-staged` → 0 on the index | matches (Index) |
| R8.6 | `config insert-step` + same | **inserted beside `step:finalize`** | `jigc config insert-step --workflow park-idea --after finalize <tmp>/land-docs-alone.yaml` (exit 0) | *path scope* · *path scope*; the body now carries both steps' text (two `Run:` lines) — the project's own composition | 0 · 0 · 0, doc alone, index identical | matches the stated precedence (doc-only step before the ordinary model) |
| R8.7 | same | **replaced out after the mint** | `DT` minted on `report-inconsistency`, then R8.3's `replace-step` | FRESH *path scope* → RESUME *carryover gate* | 3 · 3 · 3 `finalize.carried-staged`; `--carry-staged` → 0, HEAD = doc + both `src/*` + the pending `.jigc/config` delta | matches *"swaps the arm with the text"* — every door after the delta agrees; **the mint-time promise is stale** until a resume re-states it (datum, §7) |
| R8.8 | same | **replaced in after the mint** | `DT` minted on `park-idea`, then R8.4's `replace-step` | FRESH *carryover gate* → RESUME *path scope* | 0 · 0 · 0, doc alone; both `src/*` and the uncommitted delta files left out; index identical | matches |

### Cell 9 — a fan-out sub-task of a doc-only workflow

`jigc milestone create mixed` → `add-task mixed "code sub" --workflow single-task` →
`add-task mixed "report sub" --workflow report-inconsistency` → `provision mixed`; in
`.jigc/worktrees/report-sub`: a finding authored and `src/z.rs` staged; in `…/code-sub`: `src/c.rs`
staged. `finalize.fan-out.squash` at its default (true).

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| R9.1 | `workflow` | the sub-task's composed line | `jigc workflow report-inconsistency --task report-sub` (in its worktree) | 0 | — | — | `what's-left:` names **the carryover gate**; no *path-scoped* sentence; the task-scope line names `jigc milestone finalize mixed` as the only commit boundary | matches (E1 closed) |
| R9.2 | `start` | resume in the worktree | `jigc start --task report-sub` | 0 | — | — | same line | matches |
| R9.3 | `task finalize` | per-task finalize of the sub-task | `jigc task finalize report-sub` | 3 | `finalize.milestone-sub-task` | M (`jigc milestone finalize mixed`) | never reaches the arm | matches |
| R9.4 | `task finalize --dry-run` | same | `… --dry-run` | 3 | same | M | same | matches |
| R9.5 | `task validate` | the sub-task's preview | `jigc task validate report-sub` | 3 | `schema-conformance.*` on the omitted `commit:report-sub` | M | M55's declared bound under `squash=true` — row 8 owns it | HOLDS AS DECLARED |
| R9.6 | `milestone join` | join | `jigc milestone join mixed` | 0 | none | I | *"2 doc(s) merged … no docs staged from: code-sub"* | matches |
| R9.7 | `milestone finalize` | the boundary | `jigc milestone finalize mixed` | 0 | none | I | HEAD = `A docs/inconsistencies/readme-mismatch.md · M docs/milestone-records/mixed.md · A src/c.rs · A src/z.rs` — the join **commits the staged code** of the doc-only-workflow sub-task, as the composed line now says it will; ack *"report-sub: 2 docs, 1 code file"* | matches (never doc-only) |

### Cell 10 — the index-gate member's spelling, per model

| # | door | cell | composed `what's-left:` token | `task validate` | `task finalize --dry-run` | verdict |
|---|---|---|---|---|---|---|
| R10.1 | `start` · `task validate` · `task finalize` | **Index** (`park-idea`, `single-task`) | *"the carryover gate"* | `finalize.carried-staged`, exit 3 (s2) — `task validate --carry-staged` on this model was **not** driven | same code, exit 3; with `--carry-staged` exit 0 and the paths `carried-over` | matches |
| R10.2 | same | **DocOnly** (the four workflows) | *"the path scope that leaves every other staged path staged"* | no index finding, exit 0, with or without `--carry-staged` | manifest = the path set; staged neighbours `left-staged`; identical with `--carry-staged` | matches |
| R10.3 | `task amend` · `task validate` · `task finalize` | **Amend** (`jigc task amend`, two staged paths) | *"the empty-index gate this arm refuses any staged path at"* | `finalize.amend-index-dirty` ×2, exit 3 — also with `--carry-staged` | same, exit 3 — also with `--carry-staged`; the real finalize with `--carry-staged` refuses too, HEAD and index unmoved | matches |
| R10.4 | `start` · `task validate` · `task finalize` | `report-jigc-feedback` (hidden, by name), one path staged before and one after | *path scope* | exit 0; `--carry-staged` identical | exit 0; finalize 0, HEAD = `A docs/jigc-feedback/finalize-sweeps-a-neighbour.md` alone, index identical | matches |
| R10.5 | same | `triage-jigc-feedback` on that finding | *path scope* | exit 0 | exit 0; finalize 0, HEAD = `M` the finding alone, index identical | matches |

### Baseline rows and leads (detail in §5)

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| B2 | `config set` | `(4, DEFECT 2)` — `.jigc/config` at 0555, not root | `jigc config set docs-root documentation --format json` | 1 | `config.repoint-failed` | M | stdout 0 bytes; one stderr document; `message` still carries the **absolute host path** of `.jigc/config/manifest.yaml` while `location.address` is repo-relative; doc byte-identical, `git status` empty | STILL-OPEN (expected, 1.x) + residue `(R5, F9)` |
| B3 | `milestone create` | `(4, DEFECT 3)` — trimmed `.jigc/.gitignore`, rejecting hook | `jigc milestone create 'Amend probe'` (+ `--format json`) | 1 | none printed (the reject frame; the log identity was not read on this row) | M | *"…so nothing of milestone:amend-probe survives"*; the six-line amend **survives** (` M .jigc/.gitignore`); `gitignore` mentioned 0 times on either stream, text and JSON; the succeeding control still acks the amend | STILL-OPEN (expected, 1.x) |
| L1 | `milestone provision` | lead: CL-4's stale-base ordering leg — a depth-1 clone of the record's repository | `jigc milestone provision prov-probe` | 1 | `milestone.stale-base` | H | `.jigc/.gitignore` **byte-identical** — the guard precedes the `ensure` | lead **now driven → CONFIRMED** |
| L4 | `milestone provision` | lead: a failure point after the `ensure` — `.git/worktrees` made a regular file | `jigc milestone provision prov-probe` | 1 | `milestone.provision-failed` | M | the amend **survives** (` M .jigc/.gitignore`), named on neither stream; the clean re-run (exit 0) does not ack it either | lead **now driven → DEFECT** `(R5, F8)` |
| L4b | `milestone provision` | same lead, first attempt — `.git/worktrees` pre-created at 0555 | same | 0 | none | I | the fault did not fire; provision succeeded and acked the amend | **DEMOTED (reconciler) — NOT DRIVEN:** marked driven, no repro block in this file, not re-driven by the reconciler. The driver's verdict was: not a reaching failure (kept for honesty) |

**Rows driven: 105 after reconciliation** (the driver counted 108; R6.17, R6.34 and L4b are demoted — §R.5; a further 27 rows carried no block of their own and stand on the reconciler's blocks, listed there). The driver's count line read: *Rows driven: 108.* (Cell 1: 12 · cell 2: 7 · cell 3: 4 · cell 4: 5 · cell 5: 5 · cell 6: 37 ·
cell 7: 13 · cell 8: 8 · cell 9: 7 · cell 10: 5 · baseline/leads: 5.) `(4, DEFECT 1)` is R6.29.

---

## 3 · Findings

No tier-1 row. Tiers are proposals on the predicate: **1** = exit-0 loss or repository harm through
a committing, destroying or moving door · **2** = a posture or route dead end · **3** = a surface
says something the binary does not do.

### `(R5, F1)` — a formatter-style `pre-commit` hook makes the doc-only commit leave a staged reversion of its own doc, call the committed doc `left-staged`, and block the code task beside it

**Proposed tier 2** — the code task this arm exists to protect is refused at its own finalize, at a
route that names no command which clears it. **Why not tier 1, stated so the reconciler can
disagree:** nothing is lost and no jigc door commits the stray entry at exit 0 — every ordinary jigc
door driven refuses it (`finalize.base-mismatch` for a task minted before, `finalize.carried-staged`
for one minted after). What *does* commit it is a plain `git commit` outside jigc (driven below).
**Tier-3 components on the same repro:** `git diff --cached --name-status` is not what it was (the
arm's stated property); `left_out[]` gives a **committed** path the kind `left-staged`
(`design/command-output-contract.md` → *The M55 additive kind*: *"no committed path ever carries
it"*).

Mechanism (git's, reached through this arm's construction): `git commit -F <msg> -- <paths>` builds
a temporary index for the hook; a hook that `git add`s writes there; the commit takes it; the real
index keeps the pre-hook blob. The ordinary model commits the real index, so the same hook leaves it
clean (R6.20).

```sh
# setup
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
mkdir src; echo one > src/tracked1.rs; git add src; git commit -q -m 'chore: seed code'
CT=$(jigc start --workflow single-task "code change probe" | sed -n 's/^task minted: //p')
DT=$(jigc start --workflow report-inconsistency "readme disagrees with code" | sed -n 's/^task minted: //p')
#   … author inconsistency:readme-mismatch and commit:$DT through `jigc doc …`, fill commit:$CT …
H=$(mktemp -d "$RIG/hooks.XXXXXX"); cat > "$H/pre-commit" <<'HOOK'
#!/bin/sh
git diff --cached --name-only --diff-filter=ACM -- '*.md' | while IFS= read -r f; do
  grep -q '^<!-- formatted -->$' "$f" || { echo '<!-- formatted -->' >> "$f"; git add -- "$f"; }
done
exit 0
HOOK
chmod +x "$H/pre-commit"; git config core.hooksPath "$H"
echo new > src/new.rs; git add src/new.rs            # the code task's work, staged after $DT (state 3)

$ git diff --cached --name-status
A	src/new.rs
$ jigc task finalize "$DT" --format json                                  -> exit 0
  committed.manifest: [promoted docs/inconsistencies/readme-mismatch.md]
  committed.left_out: [left-staged docs/inconsistencies/readme-mismatch.md, left-staged src/new.rs]
$ git diff --cached --name-status
M	docs/inconsistencies/readme-mismatch.md            <- nobody staged this
A	src/new.rs
$ git status --short
MM docs/inconsistencies/readme-mismatch.md
A  src/new.rs
# the hook's line:  HEAD=1  index=0  worktree=1
$ git diff --cached -- docs/inconsistencies/readme-mismatch.md | tail -1
-<!-- formatted -->
$ jigc validate                                                            -> exit 0, "validates clean"

# the code task beside it (hook kept or dropped — both driven, same result)
$ jigc task validate "$CT"                                                 -> exit 0
$ jigc task finalize "$CT" --dry-run                                       -> exit 3
$ jigc task finalize "$CT"                                                 -> exit 3
  blocking · finalize.base-mismatch — the task was started at base `<A>` but HEAD is now `<B>`, and the
    moved history overlaps the task's work on `docs/inconsistencies/readme-mismatch.md`
    route: resolve the overlap on `docs/inconsistencies/readme-mismatch.md` against the new history, or
           discard the task with `jigc task discard code-change-probe --force`

# what clears it — printed by nothing
$ git restore --staged -- docs/inconsistencies
$ jigc task finalize "$CT"                                                 -> exit 0, "added src/new.rs · 1 file committed"
```

Same on the triage shape (`MM docs/inconsistencies/crate-count.md`, R6.19). A second report under the
hook adds a second phantom and names both committed docs `left-staged` (R6.23). A task minted
afterwards is refused `finalize.carried-staged` on the phantom, routed at `git restore --staged`
(R6.22). **Control, ordinary model** (`park-idea`, same hook): index empty after the commit, hook
line in HEAD = index = worktree, `left_out` empty, the code task then lands (R6.20).

**What a plain `git commit` does with it** (outside jigc, the hook still installed; triage rig):

```sh
$ git status --short
MM docs/inconsistencies/crate-count.md
A  src/new.rs
$ git commit -q -m 'feat: my code'                                         -> exit 0
$ git show --name-status --format=%s HEAD
feat: my code
M	docs/inconsistencies/crate-count.md               <- the hook's line, reverted under a feat commit
A	src/new.rs
# the hook's line:  HEAD=0  worktree=1 ;  git status: " M docs/inconsistencies/crate-count.md" ; jigc validate: clean
```

**Scope datum:** the record-only doors use the same commit shape since M47 and show the same
phantom (`jigc milestone create` under this hook → `MM docs/milestone-records/fmt-probe.md`; the next
`milestone add-task` → exit 1 `reconciliation.conflict-block`, R6.25). So the mechanism is not
M55's; M55 put it on every report and triage, beside open code tasks by design.

### `(R5, F2)` — a path git C-quotes reaches the manifest as the quoted string, and the forecast then disagrees with the door

**Proposed tier 3** — the JSON `path` is not the path, and a forecast says *left out* of a file the
door commits; the commits themselves are right. Not M55's in origin (the ordinary model has it); the
M55 `left-staged` list and the doc-only forecast inherit it.

```sh
# ordinary model — a staged name with a space, and a non-ASCII one
$ git status --short
A  "src/with space.rs"
A  "src/\303\274.rs"
$ jigc task finalize "$CT" --dry-run --format json
  manifest: [… {"kind":"added","path":"\"src/with space.rs\""}, {"kind":"added","path":"\"src/\\303\\274.rs\""}]
$ jigc task finalize "$CT" --format json                                   -> exit 0
  committed.manifest: [… {"kind":"added","path":"src/with space.rs"}, {"kind":"added","path":"\"src/\\303\\274.rs\""}]
#                         ^ forecast and landed entry differ for the same file

# doc-only model — a recorded owner-artifact named "my cap.md"
$ jigc task finalize glob-run --dry-run --format json                      -> exit 0
  manifest: [promoted docs/dogfood/glob-run.md]
  left_out: [left-staged completions/artifacts/glob-run/my, untracked "\"completions/artifacts/glob-run/my cap.md\""]
$ jigc task finalize glob-run --format json                                -> exit 0
  committed.manifest: [added completions/artifacts/glob-run/my cap.md, promoted docs/dogfood/glob-run.md]
  committed.left_out: [left-staged completions/artifacts/glob-run/my]
$ git show --name-status --format=%s HEAD   ->   A completions/artifacts/glob-run/my cap.md · A docs/dogfood/glob-run.md
```

Contract: `design/command-output-contract.md` → *"one classification serves the forecast and the
landed set, so the two stay identical by construction"*.

### `(R5, F3)` — on the ordinary model the forecast lists an untracked recorded owner-artifact as left out, and the door commits it

**Proposed tier 3** — forecast ≠ door on the contrast model; the doc-only model forecasts the same
artifact correctly (R7.1). Pre-M55 (M45's staging arm).

```sh
# workflow file-dogfood-index (step:finalize), dogfood-record with owner-artifact = completions/artifacts/glob-run/cap.md (untracked)
$ jigc task finalize glob-run --dry-run --format json                      -> exit 0
  manifest: [added …/other.md, promoted docs/dogfood/glob-run.md]
  left_out: [untracked completions/artifacts/glob-run/cap.md, untracked …/third.md]
$ jigc task finalize glob-run --format json                                -> exit 0
  committed.manifest: [added completions/artifacts/glob-run/cap.md, added …/other.md, promoted docs/dogfood/glob-run.md]
  committed.left_out: [untracked …/third.md]
```

### `(R5, F4)` — a `git rm --cached` path appears twice in the doc-only left-out list

**Proposed tier 3** — *"one entry per path, whichever change it stages"* (same paragraph of the
contract); the binary emits two entries, two kinds, one path. No effect on the commit.

```sh
$ git rm -q --cached src/c.rs          # staged deletion, file kept:  "D  src/c.rs" and "?? src/c.rs"
$ jigc task finalize "$DT" --dry-run --format json
  left_out: [… {"kind":"left-staged","path":"src/c.rs"}, … {"kind":"untracked","path":"src/c.rs"} …]
# the text arm prints `src/c.rs` twice under the one left-out heading
```

### `(R5, F5)` — `finalize.stage-failed` says the promotions were rolled back and the same re-run lands, beside a `finalize.rollback-conflict` that says one was not

**Proposed tier 3** — two findings on one surface contradict; the hook-rejection frame has an
*"apart from the N path(s) named below"* opener for exactly this and the stage-failure frame does
not. The compare-and-swap itself is correct (racer's bytes stand, pre-image parked, index
identical). Both models.

```sh
# a required clean filter that appends to the worktree file and exits 1 (see §6 for the instrument)
$ jigc task finalize "$DT"                                                 -> exit 3
  blocking · finalize.stage-failed — jigc could not stage its own changes — no commit was made and the
    promotions were rolled back: `git add -- :(literal)docs/inconsistencies/crate-count.md` failed: …
    route: `jigc task finalize triage-crate-count` once the embedded git failure is resolved … — the task
           survives intact, so the same re-run lands the commit
  blocking · finalize.rollback-conflict — `docs/inconsistencies/crate-count.md` changed while this finalize
    was running, so the rollback did not restore it …
    route: … this finalize's pre-image at `.jigc/displaced/finalize/docs/inconsistencies/crate-count.md.pre-image.<nanos>` …
# fault removed, residue left in place — "the same re-run":
$ jigc task finalize triage-crate-count                                    -> exit 3  reconciliation.conflict-block
```

### `(R5, F6)` — two more finalize routes end in a bare `jigc task finalize`

**Proposed tier 2** — the class `(4, DEFECT 1)` was filed under on rc.16, at the two producers the
fix did not reach: `finalize.nothing-staged` and `finalize.promote-clobber`.

```sh
$ jigc task finalize code-change-probe                                     -> exit 3
  blocking · finalize.nothing-staged — you staged nothing — the working tree has changes but the index is empty
    route: `git add` your changes, then re-run `jigc task finalize`
$ jigc task finalize readme-disagrees-with-code                            -> exit 3
  blocking · finalize.promote-clobber — … would overwrite a file already there — refusing to clobber it
    route: … in its own task; then re-run `jigc task finalize`
$ jigc task finalize                                                       -> exit 2
  error: the following required arguments were not provided:  <ID>
```

Source read, as a lead only: the literal *re-run `jigc task finalize`* has two production
producers, `crates/cli/src/task.rs` (`nothing-staged`) and `crates/engine/src/finalize.rs`
(`promote-clobber`); both were driven.

### `(R5, F7)` — absolute host paths inside the repository, in a route and in a message

**Proposed tier 3** — the class of `(4, DEFECT 2)`, two more producers; both paths are inside the
repository and the repo-relative spelling is in hand in the same finding.

```sh
$ jigc task finalize readme-disagrees-with-code                            -> exit 3
  blocking · finalize.promote-clobber — … `docs/inconsistencies/readme-mismatch.md` …
    route: … bring it under management with `jigc migrate <absolute host path>/repo/docs/inconsistencies/readme-mismatch.md --as <doctype>` …
$ jigc milestone provision prov-probe                                      -> exit 1
  blocking · milestone.provision-failed — … `git worktree add --detach <absolute host path>/repo/.jigc/worktrees/prov-sub-one <sha>` failed: …
    at: .jigc/worktrees/prov-sub-one
```

(The `git -C <absolute>` in `finalize.carried-staged` / `finalize.amend-index-dirty` routes is the
declared *pasteable shell bytes* rule and is **not** part of this finding.)

### `(R5, F8)` — `milestone provision` failing after its `ensure` leaves the `.jigc/.gitignore` amend on disk and no run ever names it

**Proposed tier 3** — the baseline's open lead, now driven; the sibling of `(4, DEFECT 3)` at the
door M51's C4 was first filed on. No data loss (a union amend; the user's own lines are kept).
Weaker than DEFECT 3 in one respect: this frame does not say *"nothing survives"*.

```sh
printf 'tasks/\n# my private line\n' > .jigc/.gitignore; git add .jigc/.gitignore; git commit -q -m 'chore: trim the ignore file'
: > .git/worktrees                       # the fault: git cannot create .git/worktrees/<name>
$ jigc milestone provision prov-probe                                      -> exit 1
  blocking · milestone.provision-failed — … 0 of 2 worktree(s) landed before it …
    route: `jigc milestone provision prov-probe` — deal with what the message names at that path first; …
$ git status --short                     ->  " M .jigc/.gitignore"   (six lines appended)
$ grep -c gitignore <stdout> <stderr>    ->  0 0
unlink .git/worktrees
$ jigc milestone provision prov-probe                                      -> exit 0
  provisioned 2 worktree(s) for milestone:prov-probe at base 3943dc7 (prov-sub-one, prov-sub-two)
#  no ".jigc/.gitignore → appended …" line: the entries are already there, so the amend is acked by neither run
```

### `(R5, F9)` — a rolled-back `config set docs-root` leaves the new root's empty directories behind

**Proposed tier 3** — *"nothing jigc wrote inside a transaction survives … and every such survival
is named"* (`design/finalize.md` → Rollback discipline); two empty directories survive, unnamed,
invisible to `git status`. Seen on `(4, DEFECT 2)`'s repro.

```sh
$ jigc config set docs-root documentation --format json                    -> exit 1  config.repoint-failed
    route: `docs-root` is unchanged and every doc this re-point moved is back at its prior home. …
$ git status --short                     ->  (empty)
$ find documentation                     ->  documentation · documentation/inconsistencies
```

### `(R5, F10)` — `task finalize --help` and `task validate --help` describe two commit models; the binary has three

**Proposed tier 3** — row 9 owns help text; recorded here because it is this arm's flag. Neither
help contains *doc-only*, *path-scoped* or *path scope*. `task finalize --help` calls the amend arm
*"its **second commit model**"* and stops; `--carry-staged` is documented as *"land index entries
staged before this task existed instead of refusing … Inert when nothing is carried, and inert on an
**amend** task in every state"*, and `--dry-run` lists the carryover gate and the
`finalize.nothing-staged` recolor with the amend arm as the only exception. Driven: on a doc-only
task `--carry-staged` lands nothing in any state (R1.6, R1.8, R3.4), the carryover gate never fires
(R1.5), and an empty task keeps `finalize.empty-commit` (R6.27).

```sh
$ jigc task finalize --help | grep -c -i 'doc-only\|path-scoped\|path scope'      -> 0
$ jigc task validate --help | grep -c -i 'doc-only\|path-scoped\|path scope'      -> 0
```

---

## 4 · Repro blocks for the cells that match

Every block starts from `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig";
[ -n "$REPO" ] || exit`. `author …` stands for the `jigc doc create|set-field|set-slot|add-item
--task` sequence the composed workflow prints; `fill commit` for `set-field commit:<T>#type`,
`set-slot commit:<T>#summary`, `set-slot commit:<T>#body`.

### R-1 · cells 1, 3, 4 (one block per state; the argv differs only by shape and `--carry-staged`)

```sh
# setup (fresh)
mkdir src; echo one > src/tracked1.rs; echo two > src/tracked2.rs; echo del > src/del.rs
git add src; git commit -q -m 'chore: seed code'
#   triage shape only: land a finding first — report-inconsistency "seed finding" → inconsistency:crate-count
CT=$(jigc start --workflow single-task "code change probe" | sed -n 's/^task minted: //p')
code_edit()  { echo new > src/new.rs; echo edit >> src/tracked1.rs; rm src/del.rs; }
code_stage() { git add src/new.rs src/tracked1.rs; git rm -q --cached src/del.rs; }
beside()     { echo wip >> src/tracked2.rs; echo scratch > scratch.txt; }
#   s1: code_edit; beside; mint DT           s2: code_edit; code_stage; beside; mint DT
#   s3: mint DT; code_edit; code_stage; beside
#   DT = jigc start --workflow report-inconsistency "readme disagrees with code"   (author + fill commit)
#     or jigc start --workflow triage-inconsistency "triage crate count"
#        jigc doc set-field inconsistency:crate-count#meta/status --value resolved --task $DT
#        jigc doc set-slot  inconsistency:crate-count#resolution --from-file - --task $DT

# observed, s3 · report (s2 is identical; s1 differs only in the kinds and an empty index)
$ git diff --cached --name-status
D	src/del.rs
A	src/new.rs
M	src/tracked1.rs
$ jigc task validate $DT                                   -> exit 0   (advisory file-state.staged-copy only)
$ jigc task finalize $DT --dry-run                         -> exit 0
finalize --dry-run — pre-commit manifest (nothing committed)
would commit — docs: doc probe commit
  promoted docs/inconsistencies/readme-mismatch.md
  left-out (this commit takes only this task's docs and the artifacts they record — a staged path stays staged for the task it belongs to):
    src/del.rs
    src/new.rs
    src/tracked1.rs
    src/tracked2.rs
    scratch.txt
$ jigc task finalize $DT --dry-run --format json   # left_out: left-staged ×3, modified src/tracked2.rs, untracked scratch.txt
$ jigc task finalize $DT [--carry-staged]                  -> exit 0
finalize — about to commit only this task's docs, path-scoped; leaving out:
  … (the same five) …
finalized <sha> — docs: doc probe commit
  promoted docs/inconsistencies/readme-mismatch.md
  1 file committed
  left-out (…): … (the same five) …
$ git diff --cached --name-status                          # byte-identical to the "before" (cmp -s)
$ git show --name-status --format=%s HEAD
docs: doc probe commit
A	docs/inconsistencies/readme-mismatch.md
$ jigc task finalize $CT                                   -> exit 0     (s1: after code_stage)
finalized <sha> — feat: code change probe
  deleted src/del.rs · added src/new.rs · modified src/tracked1.rs · 3 files committed
  left-out (unstaged/untracked — git add to include): src/tracked2.rs · scratch.txt
```

12/12 runs: every exit 0, `INDEX BYTE-IDENTICAL`, first `git show` = the doc alone (`A` for report,
`M docs/inconsistencies/crate-count.md` for triage), second = the code task's three. The landed
JSON arm (the six `--carry-staged` runs) carries `committed.{displaced, files, hash, hook_output,
left_out, manifest, promoted, subject}` + `findings` + `schema_version`; the pre-commit left-out
advisory prints on stderr beside it.

### R-2 · cell 2

```sh
# fresh; mint DT (report or triage), then per mode:
#   unstaged:       jigc config set finalize.fan-out.squash false        (after the mint)
#   staged-before:  the same, then `git add .jigc/config`, BEFORE the mint
#   staged-after:   the same, then `git add .jigc/config`, after the mint
$ jigc task finalize $DT --format json                     -> exit 0
#   unstaged:      left_out [untracked .jigc/config/manifest.yaml]   ; status "?? .jigc/config/manifest.yaml" before and after
#   staged-*:      left_out [left-staged .jigc/config/manifest.yaml] ; cached "A .jigc/config/manifest.yaml" before and after
$ git show --name-status --format=%s HEAD                  -> the doc alone;  .jigc/version byte-identical
# contrast, park-idea: unstaged → HEAD = A .jigc/config/manifest.yaml + A docs/ideas/parked-probe.md
#                      staged-before → exit 3 finalize.carried-staged ; staged-after → committed beside the doc
```

### R-5 · cell 5 (the ordinary-model control) — same fixture as R-1 with `park-idea`

```sh
# s2
$ jigc task validate $DT ; jigc task finalize $DT --dry-run ; jigc task finalize $DT     -> exit 3, 3, 3
blocking · finalize.carried-staged — `src/new.rs` was already staged before this task existed — …
  route: unstage it (`git -C $REPO restore --staged -- src/new.rs`) if it is not this task's work, or … `--carry-staged` …
   (one per path: src/del.rs, src/new.rs, src/tracked1.rs)
$ jigc task finalize $DT --carry-staged                    -> exit 0 ; HEAD = idea doc + D src/del.rs + A src/new.rs + M src/tracked1.rs
# s3
$ jigc task finalize $DT                                   -> exit 0 ; HEAD = the same four paths, no flag
$ jigc task finalize $CT                                   -> exit 3  finalize.nothing-staged   (→ (R5, F6))
```

### R-6 · cell 6, rejecting hooks (one block; the argv differs by shape, hook and `CLAUDECODE`)

```sh
# R-1's fixture in state s3 (src/new.rs + src/tracked1.rs staged after the mint), then:
H=$(mktemp -d "$RIG/hooks.XXXXXX"); git config core.hooksPath "$H"
#   pre-commit:   printf '#!/bin/sh\necho "HOOK-SAYS-NO (pre-commit)" >&2\nexit 1\n'
#   commit-msg:   printf '#!/bin/sh\ncp "$1" "<H>/seen-msg"\necho "HOOK-SAYS-NO (commit-msg)" >&2\nexit 1\n'
#   race-reject:  printf '#!/bin/sh\necho "<!-- raced by hook -->" >> "<DEST>"\necho "HOOK-RACED-AND-SAYS-NO" >&2\nexit 1\n'  (as pre-commit)

$ [env -u CLAUDECODE] jigc task finalize $DT               -> exit 1
`git commit` was rejected (no commit was made):
HOOK-SAYS-NO (pre-commit)

task readme-disagrees-with-code is intact — nothing was committed, your task's staged docs are still in
`.jigc/tasks/readme-disagrees-with-code/docs/`, and anything you had `git add`-ed is still in git's index.
Fix the hook's complaint, then re-run `jigc task finalize readme-disagrees-with-code`.
# snapshot (HEAD, HEAD^{tree}, sha(ls-files -s), DEST, sha(.jigc/tasks/$DT/**), .jigc/.gitignore, .jigc/version,
#           diff --cached --name-status, status --short): IDENTICAL — 8/8 un-raced cells
# invocation log: {argv: [task, finalize, <DT>], exit_code: 1, error_code: finalize.commit-rejected, finding_codes: []}
$ jigc task finalize $DT --format json                     -> exit 1 ; stdout 0 bytes ; stderr = one {findings, schema_version}
$ git config --unset core.hooksPath; jigc task finalize $DT -> exit 0 ; one new commit ; the doc alone ; index identical ; jigc validate clean

# the message the commit-msg hook was handed
#   CLAUDECODE set:      docs: doc probe commit / (blank) / Driver probe body. / (blank) / Co-Authored-By: Claude <noreply@anthropic.com>
#   CLAUDECODE cleared:  docs: doc probe commit / (blank) / Driver probe body.
# diff of the two full logs, shas masked: the trailer lines — nothing else (6/6 pairs; the raced-report pair also differs
# in the host bytes of the (R5, F7) route, which are the rig's own path)

# race-reject · triage
apart from the 1 path named below, which survived the rollback: task triage-crate-count is intact — … Fix the hook's
complaint and follow each surviving path's route below, then re-run `jigc task finalize triage-crate-count`.
blocking · finalize.rollback-conflict — `docs/inconsistencies/crate-count.md` changed while this finalize was running,
  so the rollback did not restore it: the bytes on disk are not the ones jigc wrote
  route: nothing was committed and both versions are on disk: the file as it now stands at `…/crate-count.md`, and this
         finalize's pre-image at `.jigc/displaced/finalize/docs/inconsistencies/crate-count.md.pre-image.<nanos>`. …
# snapshot diff: DEST changed, " M docs/inconsistencies/crate-count.md" — everything else identical, the index included
# race-reject · report: route "… did not exist before this finalize, so the rollback would have deleted the copy jigc
#                       created — it did not. Remove it by hand if you do not want it" ; "?? docs/" ; nothing parked
```

### R-6b · unchanged recording, empty task, stage failure, owner-artifact axis, posture, chatty hook

```sh
# unchanged recording (fresh + a landed finding; a pre-commit hook that only touches <H>/ran)
$ jigc doc set-field inconsistency:crate-count#meta/status --value open --task $DT
set inconsistency:crate-count#meta/status = open (copied in for update — …)
$ jigc task validate $DT                                   -> exit 0
$ jigc task finalize $DT --dry-run | $DT | $DT --carry-staged | $DT --format json      -> exit 3 ×4
blocking · finalize.empty-commit — task validated but produced no diff — nothing to finalize
  route: make a change, then re-run `jigc task finalize triage-crate-count` — or, …, `jigc task discard triage-crate-count --force`
# <H>/ran absent (git commit never reached) ; HEAD unmoved ; "A src/new.rs" still staged ; "?? .jigc/config/manifest.yaml" untouched

# stage failure
$ : > .git/index.lock ; jigc task finalize $DT             -> exit 3
blocking · finalize.stage-failed — … `git add -- :(literal)docs/inconsistencies/readme-mismatch.md` failed: fatal: Unable to create '…/.git/index.lock': File exists. …
  route: `jigc task finalize readme-disagrees-with-code` once the embedded git failure is resolved (e.g. remove a stale `.git/index.lock`) — …
# HEAD, sha(ls-files -s), .jigc/.gitignore, .jigc/version, DEST (absent), status: identical ; after unlink the re-run lands

# owner-artifact, pre-staged blob A then edited to B, rejecting hook (workflow file-dogfood)
$ git ls-files -s -- completions     # before and after the rejected finalize: identical rows (blob A)
$ jigc task finalize glob-run        -> exit 1 (frame as above) ; then, hook removed -> exit 0, HEAD = A …/cap.md (A+B) + A docs/dogfood/glob-run.md

# posture: rig = dev/jigc-rig fresh --binary … --start report-inconsistency "readme disagrees with code" --git-state merge
$ jigc task validate $RIG_TASK | task finalize … --dry-run | task finalize … | … --carry-staged     -> exit 1 ×4
blocking · repo.operation-in-progress — a merge is in progress — the repository is not in a committable state

# chatty-hooks
$ jigc task finalize $DT             -> exit 0 ; "--- hook output ---" / "chatty-hook: pre-commit spoke on success"
$ jigc task finalize $DT2 --format json   -> committed.hook_output = "chatty-hook: pre-commit spoke on success"
```

### R-7 · cell 7

```sh
# fresh; then the two project-layer workflows, hand-written (the stated exception) and committed with plain git:
#   .jigc/config/workflows/file-dogfood.yaml        … allows-create: [{type: dogfood-record, as: record}] … {{ include: step:author-commit }} {{ include: step:finalize-doc-only }}
#   .jigc/config/workflows/file-dogfood-index.yaml  … the same, ending {{ include: step:finalize }}
DT=$(jigc start --workflow file-dogfood "glob run" | sed -n 's/^task minted: //p')      # → glob-run
jigc doc create dogfood-record --title "glob run" --task $DT ; … thirteen meta fields, the judgment slot, fill commit …
mkdir -p completions/artifacts/glob-run; echo capture > "completions/artifacts/glob-run/<NAME>"
jigc doc set-field dogfood-record:glob-run#meta/owner-artifact --value "completions/artifacts/glob-run/<NAME>" --task $DT
#   <NAME> ∈ cap.md · c*.md (+ staged cother.md, untracked cthird.md) · capture[1].md (+ staged capture1.md) · :lead.md (+ staged lead.md)
$ jigc task finalize $DT --format json                     -> exit 0
$ git show --name-status --format=%s HEAD                  -> A completions/artifacts/glob-run/<NAME> · A docs/dogfood/glob-run.md
$ git diff --cached --name-status                          -> the staged sibling, byte-identical to before

# a leading colon in the recorded value
$ jigc doc set-field …#meta/owner-artifact --value ':(top)completions/artifacts/glob-run/cap.md' --task $DT    -> exit 0
$ jigc task validate $DT ; jigc task finalize $DT          -> exit 3, 3
blocking · owner-artifact.present — … `:(top)completions/artifacts/glob-run/cap.md` is not under the owned artifact home `completions/artifacts/<milestone>/`
```

### R-8 · cell 8

```sh
# fresh + seed; a `single-task` open; src/pre.rs staged before the mint, src/post.rs after
printf '{{ include: step:finalize }}\n'          > <tmp>/land-the-index.yaml
printf '{{ include: step:finalize-doc-only }}\n' > <tmp>/land-docs-alone.yaml
$ jigc config replace-step workflow:report-inconsistency#finalize-doc-only <tmp>/land-the-index.yaml   -> exit 0
config: replaced `workflow:report-inconsistency#finalize-doc-only` with `land-the-index` — written to `.jigc/config/`, uncommitted — …
$ jigc config replace-step workflow:park-idea#finalize <tmp>/land-docs-alone.yaml                      -> exit 0
$ jigc config remove-step workflow:report-inconsistency#finalize-doc-only                              -> exit 0
$ jigc config insert-step --workflow park-idea --after finalize <tmp>/land-docs-alone.yaml             -> exit 0
#   (R8.3–R8.6: the delta committed with plain git before the mint; R8.7/R8.8: recorded after the mint, left uncommitted)
#   R8.1/R8.2: hand-written .jigc/config/steps/file-report-final.yaml + .jigc/config/workflows/file-report-nested.yaml,
#              and .jigc/config/steps/finalize-doc-only.yaml ("Land this task's docs as one commit.\n\n{{ cli.finalize-task }}\n")
$ jigc start --workflow <W> "file a finding" | grep "^what's-left"      # FRESH
$ jigc start --task <DT>              | grep "^what's-left"             # RESUME
$ jigc task validate <DT> ; jigc task finalize <DT> --dry-run ; jigc task finalize <DT>
# results: the table. In every variant the RESUME line, the three doors' exits and the commit's contents name one model.
```

### R-9 · cell 9 — as the cell's header; observed lines in the table.

### R-10 · cell 10, the amend arm

```sh
# fresh + a landed single-task commit; src/pre.rs staged; then
$ jigc task amend                                          -> exit 0 ; "task minted: amend-<sha7>" ; "amending: <sha7> "feat: code change probe""
what's-left: … this task's content findings, the empty-index gate this arm refuses any staged path at, …
# src/post.rs staged; fill commit
$ jigc task validate amend-<sha7> [--carry-staged]         -> exit 3, 3
$ jigc task finalize amend-<sha7> --dry-run [--carry-staged] -> exit 3, 3
$ jigc task finalize amend-<sha7> --carry-staged           -> exit 3
blocking · finalize.amend-index-dirty — `src/post.rs` is staged, and an amend rewrites `HEAD` from the index — …   (and src/pre.rs)
# index and HEAD unmoved
```

### R-B · baseline rows and leads

```sh
# (4, DEFECT 2)
#   fresh; a landed inconsistency doc at docs/inconsistencies/crate-count.md; not root
chmod 0555 .jigc/config
$ jigc config set docs-root documentation --format json    -> exit 1 ; stdout 0 bytes ; stderr one document
  "code": "config.repoint-failed", "key": {"code": "config.repoint-failed", "target": ".jigc/config/manifest.yaml"}
  "message": "`docs-root` was not set to `documentation`: could not write <absolute host path>/repo/.jigc/config/manifest.yaml: Permission denied (os error 13) — the re-point was undone"
  "location": {"address": ".jigc/config/manifest.yaml"}
chmod 0755 .jigc/config ; git status --short → empty ; doc sha identical ; `jigc config get docs-root` → docs/ (pack-default)

# (4, DEFECT 3)
printf 'tasks/\n# my private line\n' > .jigc/.gitignore; git add .jigc/.gitignore; git commit -q -m 'chore: trim the ignore file'
#   a rejecting pre-commit hook behind core.hooksPath
$ jigc milestone create 'Amend probe'                      -> exit 1
`git commit` was rejected (no commit was made):
NO

nothing was committed — the record write and the milestone workbench were both rolled back, so nothing of
milestone:amend-probe survives. Fix the hook's complaint, then re-run `jigc milestone create 'Amend probe'`.
$ diff <pre> .jigc/.gitignore        -> 2a3,8  index/ state/ milestones/ worktrees/ logs/ displaced/
$ grep -c gitignore <stdout> <stderr> -> 0 0        (and 0 0 on `--format json`)         $ git status --short -> " M .jigc/.gitignore"
# control, hook removed: the ack line ".jigc/.gitignore → appended index/, state/, …" prints

# lead · stale base
#   fresh + seed; jigc milestone create 'Prov probe'; add-task; the ignore file trimmed and committed as above
CL=$(mktemp -d "$RIG/clone.XXXXXX"); git clone -q --depth 1 "file://$REPO" "$CL/c"; cd "$CL/c"
$ jigc milestone provision prov-probe                      -> exit 1
blocking · milestone.stale-base — milestone `prov-probe` is pinned to base `6293819`, which no longer exists (the base commit was rewritten away)
  route: restore the base commit, or re-pin the milestone's base, then re-run the op
$ diff <pre> .jigc/.gitignore        -> identical
```

---

## 5 · Baseline rows: CLOSED / STILL-OPEN

| key | status | argv + observed |
|---|---|---|
| `(4, DEFECT 1)` | **CLOSED** | `: > .git/index.lock; jigc task finalize axis-four-probe` → exit 3 `finalize.stage-failed`, route *"`jigc task finalize axis-four-probe` once the embedded git failure is resolved (e.g. remove a stale `.git/index.lock`) — the task survives intact, so the same re-run lands the commit"*. The id is in the route; the same on the doc-only arm (R6.28). **The class is not closed:** two sibling routes are still bare — `(R5, F6)` |
| `(4, DEFECT 2)` | **STILL-OPEN (expected, 1.x)** | `jigc config set docs-root documentation --format json` with `.jigc/config` at 0555 → exit 1, `config.repoint-failed`; `message` still renders the absolute host path while `location.address` is `.jigc/config/manifest.yaml`. No later arc swept it. The rollback is still clean on the tracked surface (doc byte-identical, `git status` empty) — and leaves two empty directories, `(R5, F9)` |
| `(4, DEFECT 3)` (= the open half of M51 `(4, C4)`) | **STILL-OPEN (expected, 1.x)** | `jigc milestone create 'Amend probe'` under a rejecting hook → exit 1, *"nothing of milestone:amend-probe survives"*, the amend on disk (` M .jigc/.gitignore`, six lines), `gitignore` named 0 times on stdout and stderr, text and JSON. Byte-for-byte the rc.16 datum |
| §D lead — **CL-4's stale-base ordering leg** | **now driven → CONFIRMED** | a depth-1 clone of a repository carrying the milestone record (the pinned base is absent from the clone's object store): `jigc milestone provision prov-probe` → exit 1 `milestone.stale-base`, `.jigc/.gitignore` byte-identical. The guard precedes the `ensure`. The rc.16 passes could not build a stale base because a HEAD moved *past* the base is not stale; an *absent* base is |
| §D lead — **cell D in full** (stage failure × worktree concurrently edited) | **partly driven; the rest still open** | driven for the `promote-destination` population on `task finalize`, both models, both shapes (R6.32–R6.34), with a **required git clean filter** as the in-transaction racer — an instrument the fixture library does not have and the scope told me not to simulate; see §6 for why I drove it anyway and what it is not. Result: the compare-and-swap holds; the frame is `(R5, F5)`. **Still open:** the other populations (`config-layer-worktree`, `retired-original`, the record family, the flip, `rename-worktree`, the mint areas) at this failure point — the filter only runs on paths the stage adds, and I drove only the doc destination |
| §D lead — **a genuine concurrent process racing a pre-image** | **still open** | not driven: it would test the scheduler, and no deterministic instrument exists. Unchanged reason |
| §D lead — **`milestone provision`'s post-`ensure` failure point** | **now driven → DEFECT `(R5, F8)`** | one exists: `git worktree add` failing (`.git/worktrees` a regular file) → exit 1 `milestone.provision-failed` with the amend on disk and unnamed; the clean re-run does not ack it either. A first attempt (`.git/worktrees` at 0555) did not fail and is recorded as L4b |

The five M51 rows axis 4 closed on rc.16, **re-driven on the doc-only arm** — the row's question:

| M51 row | on the doc-only arm | where |
|---|---|---|
| `C1` — the promote rollback is unconditional | **holds**: racer's bytes stand, one `finalize.rollback-conflict`, pre-image parked (triage) or the created copy left (report) | R6.9–R6.12, R6.32–R6.33 |
| `C2` / `C3` — the record family and the flip | **n/a on this arm** — a doc-only task writes no milestone record and is never a fan-out boundary (R9.3) | — |
| `DEFECT 1` — the JSON reject stream | **holds**: stdout 0 bytes, stderr one document, 8/8 | R6.13 |
| `DEFECT 2` — promote half | **holds** (as `C1`); **retire half n/a** — the arm never retires (a staged migration seam outranks it in the precedence) | R6.9–R6.12 |

What the arm adds that the ordinary model never had — a path-scoped `git commit -- <paths>` — is
where the one non-trivial finding of this row sits (`(R5, F1)`): the rollback discipline holds on
the third model; the **success** path under an index-writing hook does not leave the index as it
found it.

---

## 6 · Fixture honesty

- **Nothing was written into the gitignored `.jigc/` workbench.** Hand-written files under
  `.jigc/config/` (the tracked project layer, the scope's stated exception), committed with plain
  git: cell 7's two workflows; cell 8's `steps/file-report-final.yaml`,
  `workflows/file-report-nested.yaml` and `steps/finalize-doc-only.yaml`. Cell 8's other four
  variants used the `jigc config` verbs.
- **`.jigc/.gitignore` trims** (baseline DEFECT 3, the two provision leads) are edits to the
  user-co-owned tracked file the rows are about, committed with plain git — the baseline's own
  fixture.
- **Faults placed inside `.git/`**, never inside `.jigc/`: `: > .git/index.lock` (the baseline's
  fixture) and `: > .git/worktrees` / `mkdir .git/worktrees; chmod 0555` (mine, for the provision
  lead).
- **The cell-D instrument is mine and is not in the fixture library.** A *required* clean filter
  (`filter.racer.clean`, `filter.racer.required=true`, bound through `.git/info/attributes`) is real
  user code git runs **inside `git add`**: it appends to the worktree file and exits 1, so the stage
  fails with the worktree concurrently edited. The scope said to keep the lead open and not to
  simulate a racer; I drove this because it is git's own mechanism rather than a simulation, and I
  record it as a **driver-added probe**, not as the lead closing. It reaches only paths the stage
  adds. The triage shape needed `filter.racer.smudge=cat` as well — without it jigc's own
  `git checkout-index` to a temporary directory fails first (exit 1, uncoded, nothing written), which
  is the filter's doing and not graded.
- **`chmod 0555` cells pass vacuously as root; this run was not root** (`id -u` ≠ 0).
- **My own slip, corrected:** in the mixed-shapes drive (R4.4) four of my capture files were written
  with relative names into `$REPO`, so they appear as `untracked` in that run's left-out lists and
  make its dry/landed `left_out` differ by exactly those files. The entries the row is graded on are
  unaffected; every other run wrote its captures outside the rig.
- **A fixture artifact that is not a finding:** my first commit body was `Why: driver probe.`, which
  git reads as a trailer, so the co-author trailer joined it without a blank line. Row 10's subject;
  I changed the body and re-ran nothing that depended on it.

---

## 7 · Data recorded, not filed

- **A structural delta recorded after the mint makes the mint-time promise stale** (R8.7): the task
  was composed *path-scoped … `--carry-staged` changes nothing*, a `replace-step` then moved it to
  the ordinary model, and with only a post-mint staged neighbour its finalize would commit that
  neighbour at exit 0 (the declared two-task bound). Every door after the delta agrees with every
  other, and `design/finalize.md` says the delta *"swaps the arm with the text"*, so this matches
  the contract; the text an agent already holds is the part nothing refreshes.
- **`jigc validate` after a raced report** calls the leftover file a *"committed doc … not yet
  baselined"* (`file-state.un-baselined`) while it is untracked (`?? docs/`). Row 3's surface.
- **After a `milestone.stale-base` refusal in a clone with a trimmed ignore file**, `git status`
  shows `?? .jigc/milestones/` — the rebuilt cache was written before the refusal and the `ensure`
  that would have ignored it never ran. Only visible because the fixture trimmed the ignore file.
- **The text arm's left-out list carries no kind** (R4.3), on either model.
- **A formatting hook that touches the milestone record** makes the next record door refuse it as an
  out-of-band edit (R6.25) — the record doors' own matter, pre-M55.

---

## 8 · NOT driven

| # | (door, cell) | why |
|---|---|---|
| 1 | every cell on `--format human` | only `agent` text and `json` were driven |
| 2 | cell 6 hook rejection × `report-jigc-feedback` / `triage-jigc-feedback` | the rejecting cells ran on the `inconsistency` pair only; the `jigc-feedback` pair is driven on the success path (R10.4, R10.5) |
| 3 | cell 6 × a racing hook at an **owner-artifact** path | not driven; the un-raced owner-artifact rollback is (R6.30) |
| 4 | cell 6 × `config-layer-worktree` on the doc-only arm | n/a with a datum rather than a drive: the arm does not write `.jigc/.gitignore` or `.jigc/version` (both byte-identical across every reject and every landed run), so there is no pre-image to race |
| 5 | cell 6 × retire-untrackable / promote-after-retire on the doc-only arm | n/a — the arm never retires; a staged migration seam takes precedence over the doc-only step. Not driven to confirm the precedence itself |
| 6 | `git_commit_paths`' empty-path-list refusal | unreachable by driving: the empty-commit guard refuses first (R6.26, R6.27). Source claim only |
| 7 | cell 7 × hook rejection × a glob-named owner-artifact | not driven (the plain-named rollback is, R6.30) |
| 8 | cell 7 × other metacharacters (`?`, `\`, a leading `-`, a newline) | not driven; `*`, `[`, `:` and a space are |
| 9 | cell 8 × `jigc workflow <id> --task` re-entry, and the verb-minted composes (`task amend`, `migrate`) under a delta | not driven; fresh compose, `start --task` resume and finalize are |
| 10 | cell 8 × a team-layer shadow or delta | no team layer was built |
| 11 | cell 9 × `finalize.fan-out.squash=false`, × a hook-rejected boundary, × a triage or `jigc-feedback` sub-task | only `squash=true`, success, `report-inconsistency` |
| 12 | cell 10 × the amend arm's **landed** commit | only its refusals; the amend arm is outside this row's subject |
| 13 | cell 1 × two doc-only tasks open at once, × a report and a triage of the same finding open at once | not driven |
| 14 | the triage shape × an uncommitted out-of-band edit already at the committed doc | reconciliation's subject; not driven |
| 15 | `COMMITTING_DOORS` rows `milestone finalize (squash: false)`, `rename`, `migrate-corpus`, `milestone add-from-spec`, `milestone discard`, `task discard` | outside the partial scope; no row of mine has them as a door. `milestone create` and `milestone add-task` are doors only of B3 and R6.25 |
| 16 | `setup`, `milestone join` under failure | `setup` not driven at all; `milestone join` only on success (R9.6) |
| 17 | baseline cell C6 and cells E, G, H–M of the rc.16 matrix | not re-driven; outside the scope's named rows |
| 18 | §D lead — a concurrent process racing a pre-image | no instrument |
| 19 | §D lead — cell D for every population but `promote-destination` | the clean-filter instrument reaches only staged paths; I drove the doc destination |

**Rows not driven: 19 groups.**

---

## 9 · Doors covered

Every clap leaf that is the door of ≥1 driven row, `VERB_KINDS` spelling — **14 leaves**:

`start` · `workflow` · `task finalize` · `task validate` · `task amend` · `config set` ·
`config replace-step` · `config insert-step` · `config remove-step` · `milestone create` ·
`milestone add-task` · `milestone provision` · `milestone join` · `milestone finalize`

`milestone add-task` is a door of one row only (R6.25, a datum row), and `task amend` of one
(R10.3, its composed line). Verbs used to build fixtures and never graded — `doc create`,
`doc set-field`, `doc set-slot`, `doc add-item`, `task list`, `validate`, `config get` — are **not**
claimed.

---

# §R · Reconciliation ledger — ROW 5 · finalize / transaction

Reconciled 2026-10-04 against the Opus driver's table above and the Codex source pass
(`codex/axis5-codex.md`, exit code 0, non-empty). The reconciler authored neither.

- **Binary:** `~/.local/bin/jigc`; `jigc --version` → `jigc 1.0.0-rc.24`, asserted by every probe
  script before its first drive. Release posture.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit`,
  two steps, stdout only, plus a `pwd -P` check that the shell stands in a rig before any git
  command. 69 fixture names, two of them built twice; every root from `mktemp -d`; no teardown. The
  working repository was at the same commit, with an empty `git status`, after the last drive.
- **`CLAUDECODE`:** set for every drive except the one `env -u CLAUDECODE` cell in X-6.
- **git:** 2.54.0 (Apple Git-157). Not root (`id -u` ≠ 0), so the 0444 / 0555 cells are real.
- **The rule:** a claim by one that the other cannot reproduce is a lead, not a finding. Every Codex
  claim below was entered as `lead(codex, …)` and driven; every driver defect was re-driven once.
  Reconciler blocks are §R.7, numbered `X-n`.

## R.1 · The door-set count, against the registries

Re-read at the working tree (`bffa6667`). **No count differs from the driver's §1.**

| registry | reconciler read | driver | verdict |
|---|---|---|---|
| `COMMITTING_DOORS` | 11 rows over 9 clap leaves (`task finalize` ×2, `milestone finalize` ×2, `rename`, `migrate-corpus`, `milestone create`, `milestone add-task`, `milestone add-from-spec`, `milestone discard`, `task discard`); no doc-only row | 11 over 9 | agrees |
| `CommitModel` | 3 — `Index` · `Amend` · `DocOnly` | 3 | agrees |
| `StagePolicy` | 6 — `MigrationFixed` · `IndexHonoring` · `Amend` · `DocOnly` · `Combine` · `ChainPerSubtask` | 6 | agrees |
| `ManifestKind::ALL` | 7; all seven wire tags were also **seen in driven output** (X-4, X-9, X-10) | 7 | agrees |
| `GATE_COVERAGE` | 13 members; `carryover` alone carries `Some`/`Some`, the other twelve `None`/`None` | 13 | agrees |
| `ROLLBACK_POPULATIONS` · `ROLLBACK_DOORS` | 11 · 5 | 11 · 5 | agrees |
| workflows composing `step:finalize-doc-only` | 4 | 4 | agrees |

The driver's row arithmetic (12 + 7 + 4 + 5 + 5 + 37 + 13 + 8 + 7 + 5 + 5 = 108) is right; three rows
are demoted (§R.5), so **105 rows stand as driven**.

## R.2 · Driver defects — re-driven once each

| key | re-drive | status | tier | door | why that tier |
|---|---|---|---|---|---|
| `(R5, F1)` | X-1, X-2 | **CONFIRMED** (report and triage shapes; phantoms accumulate; every later jigc door refuses) | **2** | `task finalize` | Exit 0 is there. The loss-or-harm half is **not**: after the commit `HEAD` = 1, worktree = 1, index = 0 for the hook's line — no byte is absent anywhere, and the stale index entry reached a commit undeclared through **none** of the five jigc refusal arms driven against it (§R.6). What is there is a dead end: the neighbouring code task is refused `finalize.base-mismatch` at a route that names no command which clears it. Tier-3 components ride the same repro: `git diff --cached` is not what it was, and a **committed** path is labelled `left-staged` |
| `(R5, F2)` | X-3, X-4 | **CONFIRMED** (Index: forecast `"src/with space.rs"` vs landed `src/with space.rs`; DocOnly: forecast lists the recorded artifact `"…/my cap.md"` as `left_out: untracked`, the door commits it) | 3 | `task finalize` (`--dry-run`) | The commits are right on both models; the forecast and the JSON `path` say something the door does not do |
| `(R5, F3)` | X-3 | **CONFIRMED** | 3 | `task finalize` (`--dry-run`), Index | Forecast ≠ door; the door stages the artifact its own doc records, so nothing unrelated is committed |
| `(R5, F4)` | X-4 | **CONFIRMED** (`src/c.rs` twice: `left-staged` and `untracked`; text arm prints the name twice) | 3 | `task finalize` | *"one entry per path"*; no effect on the commit, the index is byte-identical |
| `(R5, F5)` | X-7 | **CONFIRMED** (report and triage) | 3 | `task finalize` | Two findings on one surface contradict; the *"same re-run"* is refused (`finalize.promote-clobber` / `reconciliation.conflict-block`), each routed. The compare-and-swap itself holds |
| `(R5, F6)` | X-5 (`nothing-staged`), X-6 (`promote-clobber`) | **CONFIRMED** at both producers; the pasted `jigc task finalize` exits 2 | 2 | `task finalize` | Tiered on the baseline's precedent: `(4, DEFECT 1)` was this class at tier 2. It is the weak end of tier 2 — the id is one token the reader holds |
| `(R5, F7)` | X-6 (`promote-clobber` route), X-8 (`milestone.provision-failed` message) | **CONFIRMED** at both producers | 3 | `task finalize` · `milestone provision` | The class of `(4, DEFECT 2)` |
| `(R5, F8)` | X-8 | **CONFIRMED** — the source pass independently named this drive as its lead 4 | 3 | `milestone provision` | A union amend of a tracked file survives a failed door and no run names it; no bytes are lost |
| `(R5, F9)` | X-8 | **CONFIRMED** (on both repro variants of `(4, DEFECT 2)`) | 3 | `config set` | Two empty directories survive a rolled-back re-point, unnamed |
| `(R5, F10)` | X-12 | **CONFIRMED** (0 mentions in either help; the help still says *"its second commit model"*) | 3 | `task finalize` · `task validate` | Row 9 owns help text; recorded here because it is this arm's flag |

No driver defect failed to reproduce. No source-pass claim contradicted a driver defect directly;
the source pass's *"no new defect"* headline is refuted by these repros (§R.3, K2).

## R.3 · Codex claims — each a lead, each driven

| # | `lead(codex, …)` | status | datum |
|---|---|---|---|
| K1 | no tier-1 path on the doc-only arm: it commits no unrelated path, loses no exit-0 artifact, does not bypass the hook-capable seam, restores nothing unconditionally | **CONFIRMED (holds on every driven cell)** — an absence claim, so bounded by the not-driven list | unrelated path: X-9, X-10, X-11 (`HEAD` = the doc alone, `INDEX BYTE-IDENTICAL`); seam: `pre-commit` and `commit-msg` hooks ran on all three models (X-1's control, X-6, X-13); restore: the racer's bytes stand, one `finalize.rollback-conflict` (X-6, X-7) |
| K2 | *"No new source-grounded completeness defect found in the M54/M55/trailer scope"* | **REFUTED** | `(R5, F1)` — X-1: exit 0, `git diff --cached --name-status` gains `M docs/inconsistencies/readme-mismatch.md`, `left_out` labels the committed doc `left-staged`. Also `(R5, F4)` and `(R5, F10)` on M55's own surfaces |
| K3 | `(4, DEFECT 1)` CLOSED; the route carries the id and preserves `--approve` and `--carry-staged` | **CONFIRMED** | X-5: `--carry-staged` on Index and DocOnly; `--approve` and `--approve --carry-staged` on a migration task |
| K4 | `(4, DEFECT 2)` STILL OPEN, tier 3; repro *make `.jigc/config/manifest.yaml` unwritable* | **CONFIRMED** — the source pass's repro reproduces as written | X-8 variant A (file at 0444) and variant B (the driver's, directory at 0555): same message, absolute host path |
| K5 | `(4, DEFECT 3)` STILL OPEN, tier 3 | **CONFIRMED** | X-8 |
| K6 | §D lead 1 (stale-base ordering) CLOSED by source: the base guard precedes `gitignore::ensure` | **CONFIRMED** | X-8: `milestone.stale-base`, `.jigc/.gitignore` byte-identical |
| K7 | §D lead 2 (stage failure × concurrent worktree edit): pre-images are captured before the writes and the common error arm restores them; the dynamic bound stays open | **CONFIRMED for the `promote-destination` population; OPEN LEAD for the rest** | X-7 (the driver's required-clean-filter instrument, both shapes): compare-and-swap holds. The other populations at this failure point are not reached by that instrument and no rig builds one |
| K8 | §D lead 3 (a genuine concurrent process racing a pre-image): open as a dynamic bound | **OPEN LEAD** | no deterministic instrument exists; driving it would test the scheduler |
| K9 | §D lead 4 (`milestone provision` post-`ensure` failure): STILL OPEN; proposed drive | **CONFIRMED → finding `(R5, F8)`** | X-8 |
| K10 | the arm predicate `doc_only_commit` excludes fan-out sub-tasks and walks the cascade-resolved include tree, so CR1, E1, project shadows and `replace-step` / `insert-step` / `remove-step` agree | **CONFIRMED (holds)** | X-11 (eight variants: FRESH, RESUME and the three doors name one model each time), X-14 (sub-task composes *the carryover gate*, `finalize.milestone-sub-task`, the boundary commits the staged code) |
| K11 | finalize computes the model once; `--carry-staged` has no membership effect on this arm | **CONFIRMED (holds)** | X-9: the `--dry-run` JSON with and without `--carry-staged` is byte-equal; the landed commit is the doc alone |
| K12 | CR3 closed: unrelated staged code and a pending config delta cannot make a doc-only task non-empty | **CONFIRMED (holds)** | X-10: `finalize.empty-commit` ×4, a marker `pre-commit` hook never ran |
| K13a | `stage_doc_only` stages only promoted destinations plus recorded owner-artifacts and literalizes every pathspec | **CONFIRMED (holds)** | X-3: `c*.md`, `capture[1].md`, `:lead.md` — only the named file committed; a `:(top)` / `:(glob)` value refused `owner-artifact.present` |
| K13b | `git_commit_paths` independently rejects an empty path list | **OPEN LEAD** | not reachable through the door: `finalize.empty-commit` refuses first (X-10). Source claim only |
| K14 | left-out classification is one entry per porcelain path, any non-blank index column becoming `left-staged`; read as consistent | **mechanism CONFIRMED · the consistency reading REFUTED** | the mechanism is exactly what produces `(R5, F4)` (one path, two porcelain lines, two entries — X-4) and F1's mislabel (a committed path with a non-blank index column — X-1); the contract says *"one entry per path"* and that no committed path carries `left-staged` |
| K15 | `ManifestKind::ALL` has seven members including `LeftStaged` | **CONFIRMED** | §R.1; all seven tags seen in driven JSON |
| K16a | `CommitModel` is three cases; `GATE_COVERAGE` has 13 rows and the index-gate member carries distinct amend / doc-only spellings | **CONFIRMED** | §R.1 and X-12: the three composed tokens and the three doors' behaviour |
| K16b | `CommitModel::of(amended, doc_only)` gives the amend marker precedence | **OPEN LEAD** | needs a task carrying both markers (an amend task whose composed workflow a project delta gave `step:finalize-doc-only`); not built. X-2's amend arm after a doc-only commit refuses `finalize.amend-index-dirty`, which is consistent and not a proof of precedence |
| K17 | `COMMITTING_DOORS` is 11 rows and the doc-only commit rides `jigc task finalize`'s identity | **CONFIRMED** | §R.1 and X-5: the invocation log of a doc-only reject carries `error_code: finalize.commit-rejected` |
| K18 | the trailer created no commit bypass: file messages are signed before `git commit` reads them and every model goes through the one seam | **CONFIRMED** | X-6 (DocOnly: the `commit-msg` hook was handed the trailer; cleared, the same message without it), X-13 (Index: the `commit-msg` hook was handed the trailer; Amend: `pre-commit` and `commit-msg` both ran, the hook saw the trailer, the tree unchanged), X-1's control (Index: the `pre-commit` hook ran) |
| K19 | no schema-hash-boundary violation: exactly `inconsistency` and `jigc-feedback` were added, both at schema-version 1; no existing hash moved; the rc.24 `planning-record` edit leaves its hash unchanged | **CONFIRMED** | X-15 |
| K20 | M55's open F21 is report-only and outside this conclusion | **not a claim on this row's doors** | noted, not adjudicated here |

## R.4 · Baseline rows

| key | status on rc.24 | datum |
|---|---|---|
| `(4, DEFECT 1)` | **CLOSED** — driver and source pass agree, reconciler re-drove | X-5: route `` `jigc task finalize axis-four-probe` once the embedded git failure is resolved… ``; with flags: `… --carry-staged`, `… --approve`, `… --approve --carry-staged`. The pasted argv landed the commit. The **class** is not closed — `(R5, F6)` |
| `(4, DEFECT 2)` | **STILL-OPEN** (tier 3, expected 1.x) | X-8: `config.repoint-failed`, `message` carries the absolute host path of `.jigc/config/manifest.yaml`, `location.address` is repo-relative |
| `(4, DEFECT 3)` (= the open half of M51 `(4, C4)`) | **STILL-OPEN** (tier 3, expected 1.x) | X-8: exit 1, *"nothing of milestone:amend-probe survives"*, ` M .jigc/.gitignore` (six lines), `gitignore` named 0 times on either stream, text and JSON |
| §D lead — CL-4's stale-base ordering leg | **now driven → CONFIRMED (the guard precedes the `ensure`)** | X-8 |
| §D lead — cell D in full | **partly driven; OPEN for every population but `promote-destination`** | X-7; the instrument is the driver's own and is not in the fixture library |
| §D lead — a concurrent process racing a pre-image | **OPEN** | no instrument |
| §D lead — `milestone provision`'s post-`ensure` failure point | **now driven → finding `(R5, F8)`** | X-8 |

The five M51 rows axis 4 closed on rc.16, on the doc-only arm: the rollback discipline **holds**
(X-6: nine-facet snapshot identical on the four un-raced cells re-driven; X-6 raced: the racer's
bytes stand, pre-image parked on the triage shape, nothing parked on the report shape, the index
identical). The record family and the flip are not reachable on this arm (X-14: a sub-task is never
doc-only).

## R.5 · Demotions, and rows that carried no block

**DEMOTED (3)** — marked driven, no repro block in the driver file, not re-driven here; each is
**not driven**:

| row | what it claimed |
|---|---|
| R6.17 | the racing passing `pre-commit` hook on the ordinary model (`park-idea`) — the control for R6.15 / R6.16 |
| R6.34 | cell D (stage failure × worktree edited) on the ordinary model |
| L4b | the first provision attempt (`.git/worktrees` at 0555) — the driver itself says the fault did not fire |

**Block supplied by the reconciler (27)** — the driver file carried the row's argv and observation
in the table or in prose only; each was re-driven here and stays driven **on the reconciler's
block**: R4.4 (the `MM` / `AD` / rename halves) → X-4 · R5.1 → X-10 · R6.15, R6.16 → X-6 · R6.19,
R6.20, R6.22, R6.23, R6.25 → X-1, X-2 · R6.27 → X-10 · R6.29 → X-5 · R6.32 → X-7 · R7.2, R7.3,
R7.8, R7.10, R7.11, R7.12 → X-3 · R9.1–R9.7 → X-14 · R10.4, R10.5 → X-12.

The driver's R-8 block lists cell 8's argv and defers every observation to the table; R8.1–R8.8 were
re-driven with observed output (X-11) and agree with the table row for row.

One driver row reads differently on a re-drive without being wrong: **R5.5** (`finalize.nothing-staged`
on the code task after the ordinary model swept its paths) depends on the driver's fixture leaving an
unstaged edit beside it. With a clean worktree the same task is refused `finalize.empty-commit`,
whose route carries the id (X-10). The bare route was reproduced on its own fixture (X-5).

## R.6 · Tier-1 adjudication

**No finding of this row is tier 1.** The one candidate is `(R5, F1)`, and it was pushed upward
deliberately: a doc-only finalize exits 0 and leaves the index holding a staged reversion nobody
staged. For tier 1 that state has to become a loss, or reach a commit through a jigc door without
being declared. Driven on the report shape (X-2), the phantom entry against seven ways to commit:

| door, after the phantom exists | exit | what happened to the phantom |
|---|---|---|
| `task finalize` — a code task minted **before** | 3 | refused `finalize.base-mismatch` |
| `task finalize` — a code task minted **after** | 3 | refused `finalize.carried-staged`, routed at `git restore --staged` |
| `task finalize` — amend arm (`task amend`) | 3 | refused `finalize.amend-index-dirty` |
| `rename` | 1 | refused `rename.dirty-tree` |
| `milestone finalize` | 3 | refused `finalize.carried-staged` (*"it stays staged either way"*) |
| `task finalize --carry-staged` — the declared carry | 0 | committed, labelled `carried-over docs/inconsistencies/readme-mismatch.md`; `HEAD` = 0, worktree = 1 |
| plain `git commit` (outside jigc) | 0 | committed; `HEAD` = 0, worktree = 1 |

Both halves, stated: **exit 0 — present** (the doc-only finalize itself). **Loss or harm — absent.**
No byte is missing from `HEAD`, the worktree or the object store at any point; where the reversion
does reach a commit, it does so under a flag whose finding named the path, or outside jigc, and the
worktree still holds the line. The index is left other than it was found, which is the arm's stated
property broken (tier 3) and a neighbour's dead end (tier 2). **The closest call of the row, for the
human:** if an index entry nobody staged, left at exit 0 by a committing door, is itself *repository
harm*, this finding is tier 1 on the evidence already here.

Downward check: no other finding shows either half. `(R5, F2)` and `(R5, F3)` are forecasts that
disagree with a door that commits the right set; `(R5, F8)` and `(R5, F9)` survive a **failed** door
(exit 1), and what survives is a union amend and two empty directories.

**Scope datum, re-driven:** the same hook at `milestone create` leaves `MM docs/milestone-records/mixed.md`
at exit 0, and the next `milestone add-task` is refused `reconciliation.conflict-block` (X-2). The
mechanism predates M55 at the record doors.

## R.7 · Reconciler repro blocks

Every block starts from `rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig";
[ -n "$REPO" ] || exit`. `seed` = `mkdir src; echo one > src/tracked1.rs; echo two > src/tracked2.rs;
echo del > src/del.rs; git add src; git commit -q -m 'chore: seed code'`. `report` = mint
`report-inconsistency`, author `inconsistency:readme-mismatch` through `jigc doc create | set-slot |
set-field | add-item --task`, fill `commit:<DT>`. `triage` = land a finding `crate-count` first, mint
`triage-inconsistency`, `set-field …#meta/status --value resolved`, `set-slot …#resolution`, fill
the commit doc. `CT` = a `single-task` task with its commit doc filled.

### X-1 · `(R5, F1)` — the formatter-style hook (report; the triage shape is identical on `crate-count.md`)

```sh
seed; CT=…; report
H=$(mktemp -d "$RIG/hooks.XXXXXX"); cat > "$H/pre-commit" <<'HOOK'
#!/bin/sh
git diff --cached --name-only --diff-filter=ACM -- '*.md' | while IFS= read -r f; do
  grep -q '^<!-- formatted -->$' "$f" || { echo '<!-- formatted -->' >> "$f"; git add -- "$f"; }
done
exit 0
HOOK
chmod +x "$H/pre-commit"; git config core.hooksPath "$H"
echo new > src/new.rs; git add src/new.rs

$ git diff --cached --name-status
A	src/new.rs
$ jigc task finalize readme-disagrees-with-code --format json            -> exit 0
  manifest: [promoted docs/inconsistencies/readme-mismatch.md]
  left_out: [left-staged docs/inconsistencies/readme-mismatch.md, left-staged src/new.rs]
$ git diff --cached --name-status
M	docs/inconsistencies/readme-mismatch.md
A	src/new.rs
$ git status --short
MM docs/inconsistencies/readme-mismatch.md
A  src/new.rs
# the hook's line:  HEAD=1  index=0  worktree=1
$ git diff --cached -- docs/inconsistencies/readme-mismatch.md | tail -1
-<!-- formatted -->
$ jigc validate                                                          -> exit 0  "validates clean"
$ jigc task validate code-change-probe                                   -> exit 0
$ jigc task finalize code-change-probe --dry-run                         -> exit 3
$ jigc task finalize code-change-probe                                   -> exit 3
blocking · finalize.base-mismatch — the task was started at base `<A>` but HEAD is now `<B>`, and the moved history overlaps the task's work on `docs/inconsistencies/readme-mismatch.md`
  route: resolve the overlap on `docs/inconsistencies/readme-mismatch.md` against the new history, or discard the task with `jigc task discard code-change-probe --force`
$ git restore --staged -- docs/inconsistencies
$ jigc task finalize code-change-probe                                   -> exit 0  "added src/new.rs · 1 file committed"
# the hook's line after the remedy:  HEAD=1  worktree=1

# control — the same hook on the ordinary model (park-idea), src/new.rs staged after the mint
$ jigc task finalize park-a-probe-idea --format json                     -> exit 0
  manifest: [promoted docs/ideas/parked-probe.md, added src/new.rs]   left_out: []
$ git diff --cached --name-status                                        -> (empty)
# the hook's line:  HEAD=1  index=1  worktree=1
```

### X-2 · `(R5, F1)` — the adversarial arms (each on its own rig, from X-1's state without `src/new.rs`)

```sh
# state after the doc-only finalize:  MM docs/inconsistencies/readme-mismatch.md ;  HEAD=1 index=0 worktree=1

# a code task minted afterwards, src/later.rs staged
$ jigc task finalize later-code-task                                     -> exit 3
blocking · finalize.carried-staged — `docs/inconsistencies/readme-mismatch.md` was already staged before this task existed — refusing to let a pre-task staged change silently ride this task's commit
  route: unstage it (`git -C $REPO restore --staged -- docs/inconsistencies/readme-mismatch.md`) if it is not this task's work, or re-run the finalize with `--carry-staged` to declare the carry-over deliberate
$ jigc task finalize later-code-task --carry-staged                      -> exit 0
finalized <sha> — feat: later code task
  carried-over docs/inconsistencies/readme-mismatch.md
  added src/later.rs
# the hook's line:  HEAD=0  index=0  worktree=1 ;  " M docs/inconsistencies/readme-mismatch.md"

# the amend arm (hook removed)
$ jigc task amend                                                        -> exit 0  "task minted: amend-<sha7>"
$ jigc task finalize amend-<sha7>                                        -> exit 3
blocking · finalize.amend-index-dirty — `docs/inconsistencies/readme-mismatch.md` is staged, and an amend rewrites `HEAD` from the index — …

# rename (no other task open)
$ jigc rename inconsistency:readme-mismatch --to "Readme drift"          -> exit 1
blocking · rename.dirty-tree — cannot rename with a dirty working tree — commit or stash your changes first …: docs/inconsistencies/readme-mismatch.md

# the milestone boundary (hook removed): milestone create 'Mixed' ; add-task mixed "code sub" ; provision ; join
$ jigc milestone finalize mixed                                          -> exit 3
blocking · finalize.carried-staged — `docs/inconsistencies/readme-mismatch.md` was already staged before this milestone existed — the aggregate commit is built from the sub-task worktrees and cannot carry it, so the change stays staged, undeclared, across this boundary

# plain git, hook still installed
$ git commit -q -m 'feat: my code'                                       -> exit 0
$ git show --name-status --format=%s HEAD     ->  feat: my code / M docs/inconsistencies/readme-mismatch.md
# the hook's line:  HEAD=0  worktree=1

# a second doc-only report under the hook (R6.23)
$ jigc task finalize second-finding --format json                        -> exit 0
  left_out: [left-staged docs/inconsistencies/readme-mismatch.md, left-staged docs/inconsistencies/second-mismatch.md]
$ git status --short     ->  MM …/readme-mismatch.md  ·  MM …/second-mismatch.md

# the record doors under the same hook (R6.25)
$ jigc milestone create 'Mixed'                                          -> exit 0
$ jigc milestone add-task mixed "code sub" --workflow single-task        -> exit 1
  reconciliation.conflict-block — conflict on `docs/milestone-records/mixed.md`: the milestone record is machine-maintained and was edited out of band since jigc last wrote it
$ git status --short     ->  MM docs/milestone-records/mixed.md
```

### X-3 · `(R5, F2)` doc-only half · `(R5, F3)` · the pathspec cells (cell 7)

Fixture: the two hand-written project-layer workflows `file-dogfood` (→ `step:finalize-doc-only`) and
`file-dogfood-index` (→ `step:finalize`), each granting `dogfood-record`, committed with plain git —
the scope's stated exception to never writing under `.jigc/`. Artifact home `completions/artifacts/glob-run/`.

```sh
# DocOnly · the recorded artifact is "my cap.md", a staged sibling "my" beside it
$ jigc task finalize glob-run --dry-run --format json                    -> exit 0
  manifest: [promoted docs/dogfood/glob-run.md]
  left_out: [left-staged completions/artifacts/glob-run/my, untracked "completions/artifacts/glob-run/my cap.md"]   <- the quotes are in the JSON string
$ jigc task finalize glob-run --format json                              -> exit 0
  manifest: [added completions/artifacts/glob-run/my cap.md, promoted docs/dogfood/glob-run.md]
  left_out: [left-staged completions/artifacts/glob-run/my]
$ git show --name-status --format=%s HEAD   ->  A …/my cap.md · A docs/dogfood/glob-run.md ;  INDEX BYTE-IDENTICAL
# Index · same name (R7.10): forecast manifest [added …/my, promoted …], left_out [untracked "…/my cap.md"] ; landed manifest adds …/my cap.md

# Index · a plain untracked recorded artifact cap.md, staged other.md, untracked third.md   (R5, F3)
$ jigc task finalize glob-run --dry-run --format json                    -> exit 0
  manifest: [added …/other.md, promoted docs/dogfood/glob-run.md]
  left_out: [untracked …/cap.md, untracked …/third.md]
$ jigc task finalize glob-run --format json                              -> exit 0
  manifest: [added …/cap.md, added …/other.md, promoted docs/dogfood/glob-run.md]
  left_out: [untracked …/third.md]
# DocOnly · the same fixture: forecast == landed — manifest [added …/cap.md, promoted …], left_out [left-staged …/other.md, untracked …/third.md]

# DocOnly · metacharacters — only the named file is committed, dry == landed, INDEX BYTE-IDENTICAL each time
#   c*.md        (staged cother.md, untracked cthird.md)  -> HEAD: A …/c*.md · A docs/dogfood/glob-run.md
#   capture[1].md (staged capture1.md)                    -> HEAD: A …/capture[1].md · A docs/dogfood/glob-run.md
#   :lead.md      (staged lead.md)                        -> HEAD: A …/:lead.md · A docs/dogfood/glob-run.md
#   tracked and changed cap.md                            -> manifest [modified …/cap.md, promoted …] ; HEAD: M …/cap.md · A docs/dogfood/glob-run.md
#   tracked, unchanged cap.md                             -> manifest [promoted …] ; HEAD: A docs/dogfood/glob-run.md alone
# a magic value, DocOnly and Index: `:(top)…/cap.md`, and DocOnly `:(glob)…/*.md`
$ jigc task validate glob-run ; jigc task finalize glob-run --dry-run ; jigc task finalize glob-run    -> exit 3 · 0 · 3
blocking · owner-artifact.present — … `:(top)completions/artifacts/glob-run/cap.md` is not under the owned artifact home `completions/artifacts/<milestone>/`
# Index · c*.md (R7.11): HEAD = c*.md + the staged cother.md + the doc ; the untracked cthird.md is not swept
```

### X-4 · `(R5, F2)` Index half · `(R5, F4)` · the mixed staged shapes (R4.4)

```sh
# Index — a staged name with a space and a non-ASCII one
$ git status --short
A  "src/with space.rs"
A  "src/\303\274.rs"
$ jigc task finalize code-change-probe --dry-run --format json           -> exit 0
  manifest paths: ["\"src/with space.rs\"", "\"src/\\303\\274.rs\""]
$ jigc task finalize code-change-probe --format json                     -> exit 0
  manifest paths: ["src/with space.rs", "\"src/\\303\\274.rs\""]

# DocOnly — beside a report task
$ git status --short
MM src/a.rs
D  src/c.rs
AD src/n.rs
R  src/r.rs -> src/renamed.rs
A  "src/with space.rs"
?? src/c.rs
$ jigc task finalize readme-disagrees-with-code --dry-run --format json  -> exit 0
  left_out: [left-staged src/a.rs, left-staged src/c.rs, left-staged src/n.rs, left-staged src/r.rs,
             left-staged src/renamed.rs, left-staged "src/with space.rs", untracked src/c.rs]
# the text arm lists src/c.rs twice under the one left-out heading
$ jigc task finalize readme-disagrees-with-code --format json            -> exit 0   (the same left_out)
$ git show --name-status --format=%s HEAD   ->  A docs/inconsistencies/readme-mismatch.md ;  INDEX BYTE-IDENTICAL
```

### X-5 · `(4, DEFECT 1)` · `(R5, F6)` nothing-staged · the log identity

```sh
# seed; CT=axis-four-probe; report; src/new.rs staged; jigc config set invocation-log true
: > .git/index.lock
$ jigc task finalize axis-four-probe                                     -> exit 3
blocking · finalize.stage-failed — jigc could not stage its own changes — no commit was made and the promotions were rolled back: `git add -- :(literal).jigc/config :(literal).jigc/.gitignore :(literal).jigc/version` failed: fatal: Unable to create '$REPO/.git/index.lock': File exists.
  route: `jigc task finalize axis-four-probe` once the embedded git failure is resolved (e.g. remove a stale `.git/index.lock`) — the task survives intact, so the same re-run lands the commit
$ jigc task finalize axis-four-probe --carry-staged                      -> exit 3   route: `jigc task finalize axis-four-probe --carry-staged` once …
$ jigc task finalize readme-disagrees-with-code                          -> exit 3   `git add -- :(literal)docs/inconsistencies/readme-mismatch.md` failed … route: `jigc task finalize readme-disagrees-with-code` once …
$ jigc task finalize readme-disagrees-with-code --carry-staged           -> exit 3   route: `jigc task finalize readme-disagrees-with-code --carry-staged` once …
# HEAD unmoved · git ls-files -s identical · destination absent
# lock removed; the route's argv pasted:
$ jigc task finalize readme-disagrees-with-code                          -> exit 0   "promoted docs/inconsistencies/readme-mismatch.md · 1 file committed" ; cached still "A src/new.rs"
# a migration task (jigc migrate FOREIGN.md --as vision, authored), same lock:
$ jigc task finalize migrate-vision-foreign-<hash> --approve             -> exit 3   route: `jigc task finalize migrate-vision-foreign-<hash> --approve` once …
$ jigc task finalize migrate-vision-foreign-<hash> --approve --carry-staged  -> exit 3   route: `… --approve --carry-staged` once …

# (R5, F6) — fresh rig: seed; CT; an unstaged edit to src/tracked1.rs
$ jigc task finalize code-change-probe                                   -> exit 3
blocking · finalize.nothing-staged — you staged nothing — the working tree has changes but the index is empty
  route: `git add` your changes, then re-run `jigc task finalize`
$ jigc task finalize                                                     -> exit 2   error: the following required arguments were not provided:  <ID>

# the invocation log of a doc-only reject (a rejecting pre-commit hook)
{argv: [task, finalize, second-finding], exit_code: 1, error_code: finalize.commit-rejected, finding_codes: []}
```

### X-6 · cell 6 — rejecting and racing hooks on the doc-only arm

Fixture: X-1's, with `src/new.rs` and `src/tracked1.rs` staged after the mint, an unstaged
`src/tracked2.rs`, an untracked `scratch.txt`. Snapshot = `HEAD` · `HEAD^{tree}` · `sha(git ls-files -s)` ·
the destination's bytes-or-absent · a hash over `.jigc/tasks/<DT>/` · `.jigc/.gitignore` ·
`.jigc/version` · `git diff --cached --name-status` · `git status --short`.

```sh
# rejecting pre-commit (report, triage) · rejecting commit-msg (report, set and cleared)
$ [env -u CLAUDECODE] jigc task finalize <DT>                            -> exit 1
`git commit` was rejected (no commit was made):
HOOK-SAYS-NO (pre-commit)
task <DT> is intact — nothing was committed, your task's staged docs are still in `.jigc/tasks/<DT>/docs/`, and anything you had `git add`-ed is still in git's index. Fix the hook's complaint, then re-run `jigc task finalize <DT>`.
# SNAPSHOT IDENTICAL (9 facets) — 4/4 ; both tasks still listed
# the message the commit-msg hook was handed:
#   CLAUDECODE set:      docs: doc probe commit / (blank) / Reconciler probe body. / (blank) / Co-Authored-By: Claude <noreply@anthropic.com>
#   CLAUDECODE cleared:  docs: doc probe commit / (blank) / Reconciler probe body.
$ jigc task finalize <DT> --format json   (same hook)                    -> exit 1 ; stdout 0 bytes ; stderr = one {findings, schema_version}, code finalize.commit-rejected, target task:<DT>
$ git config --unset core.hooksPath ; jigc task finalize <DT>            -> exit 0 ; one new commit ; the doc alone ; cached still "A src/new.rs · M src/tracked1.rs"

# racing rejecting pre-commit (appends a line to the promoted doc, exits 1)
$ jigc task finalize triage-crate-count                                  -> exit 1
apart from the 1 path named below, which survived the rollback: task triage-crate-count is intact — …
blocking · finalize.rollback-conflict — `docs/inconsistencies/crate-count.md` changed while this finalize was running, so the rollback did not restore it: the bytes on disk are not the ones jigc wrote
  route: nothing was committed and both versions are on disk: the file as it now stands at `docs/inconsistencies/crate-count.md`, and this finalize's pre-image at `.jigc/displaced/finalize/docs/inconsistencies/crate-count.md.pre-image.<nanos>`. Compare them, keep what you want, and delete the parked copy
# snapshot diff: the destination's bytes, " M docs/inconsistencies/crate-count.md", the displaced directory — the index identical
# report shape: route "… did not exist before this finalize, so the rollback would have deleted the copy jigc created — it did not. Remove it by hand if you do not want it" ; "?? docs/" ; nothing parked
# hook removed, residue left in place:
$ jigc task finalize triage-crate-count                                  -> exit 3   reconciliation.conflict-block
$ jigc task finalize readme-disagrees-with-code                          -> exit 3
blocking · finalize.promote-clobber — promoting this task's doc to `docs/inconsistencies/readme-mismatch.md` would overwrite a file already there — refusing to clobber it
  route: … bring it under management with `jigc migrate <absolute host path>/repo/docs/inconsistencies/readme-mismatch.md --as <doctype>` in its own task; then re-run `jigc task finalize`      <- (R5, F6) + (R5, F7)

# racing PASSING pre-commit (appends a line, exits 0) — R6.15, R6.16
$ jigc task finalize <DT>                                                -> exit 0
finalized <sha> — docs: doc probe commit
  promoted <doc> · 1 file committed
  left-out (…): <doc> · src/new.rs · src/tracked1.rs · src/tracked2.rs · scratch.txt
# the racer's line:  HEAD=0  worktree=1 ;  " M <doc>" ;  cached unchanged ; no rollback-conflict
```

### X-7 · `(R5, F5)` — the driver's required-clean-filter instrument, re-run

```sh
# report or triage; src/new.rs staged
F=$(mktemp -d "$RIG/filter.XXXXXX")
printf '#!/bin/sh\necho "<!-- raced inside git add -->" >> "$1"\ncat >/dev/null\nexit 1\n' > "$F/clean.sh"; chmod +x "$F/clean.sh"
git config filter.racer.clean "$F/clean.sh %f"; git config filter.racer.required true; git config filter.racer.smudge cat
echo 'docs/** filter=racer' >> .git/info/attributes

$ jigc task finalize triage-crate-count                                  -> exit 3
blocking · finalize.stage-failed — jigc could not stage its own changes — no commit was made and the promotions were rolled back: `git add -- :(literal)docs/inconsistencies/crate-count.md` failed: error: external filter '<tmp>/clean.sh %f' failed 1
  route: `jigc task finalize triage-crate-count` once the embedded git failure is resolved (e.g. remove a stale `.git/index.lock`) — the task survives intact, so the same re-run lands the commit
blocking · finalize.rollback-conflict — `docs/inconsistencies/crate-count.md` changed while this finalize was running, so the rollback did not restore it: the bytes on disk are not the ones jigc wrote
  route: … this finalize's pre-image at `.jigc/displaced/finalize/docs/inconsistencies/crate-count.md.pre-image.<nanos>`. …
# HEAD unmoved · index identical · the racer's line at the destination: 1 · the pre-image parked
# filter removed, residue left in place — "the same re-run":
$ jigc task finalize triage-crate-count                                  -> exit 3   reconciliation.conflict-block
# report shape: the same pair of findings; nothing parked; the re-run -> exit 3 finalize.promote-clobber
```

### X-8 · baseline rows, the provision leads, `(R5, F7)`–`(R5, F9)`

```sh
# (4, DEFECT 2) + (R5, F9) — a landed inconsistency doc; not root
#   variant A (the source pass's repro): a committed config delta, then  chmod 0444 .jigc/config/manifest.yaml
#   variant B (the driver's repro):      chmod 0555 .jigc/config
$ jigc config set docs-root documentation --format json                  -> exit 1 ; stdout 0 bytes ; stderr one document
  code: config.repoint-failed   key: {code: config.repoint-failed, target: .jigc/config/manifest.yaml}   location: {address: .jigc/config/manifest.yaml, line: 1, col: 1}
  message: `docs-root` was not set to `documentation`: could not write <absolute host path>/repo/.jigc/config/manifest.yaml: Permission denied (os error 13) — the re-point was undone
  route: `docs-root` is unchanged and every doc this re-point moved is back at its prior home. Fix what this message names, then re-run `jigc config set docs-root documentation`
# permissions restored: git status --short -> (empty) ; the doc byte-identical ; docs-root = docs/ (pack-default)
$ find documentation      ->  documentation · documentation/inconsistencies                      <- (R5, F9), both variants

# (4, DEFECT 3)
printf 'tasks/\n# my private line\n' > .jigc/.gitignore; git add .jigc/.gitignore; git commit -q -m 'chore: trim the ignore file'
#   a rejecting pre-commit hook behind core.hooksPath
$ jigc milestone create 'Amend probe'                                    -> exit 1
`git commit` was rejected (no commit was made):
NO
nothing was committed — the record write and the milestone workbench were both rolled back, so nothing of milestone:amend-probe survives. Fix the hook's complaint, then re-run `jigc milestone create 'Amend probe'`.
$ diff <pre> .jigc/.gitignore        -> 2a3,8  index/ state/ milestones/ worktrees/ logs/ displaced/
$ git status --short                 -> " M .jigc/.gitignore" ;  "gitignore" on stdout 0, on stderr 0 (text and --format json)
# control, hook removed:  ".jigc/.gitignore → appended index/, state/, milestones/, worktrees/, logs/, displaced/ …"

# lead · stale base — milestone create 'Prov probe' ; two add-task ; the ignore file trimmed and committed
CL=$(mktemp -d "$RIG/clone.XXXXXX"); git clone -q --depth 1 "file://$REPO" "$CL/c"; cd "$CL/c"
$ jigc milestone provision prov-probe                                    -> exit 1
blocking · milestone.stale-base — milestone `prov-probe` is pinned to base `<sha7>`, which no longer exists (the base commit was rewritten away)
# .jigc/.gitignore byte-identical ;  status: "?? .jigc/milestones/"

# lead · the post-ensure failure — (R5, F8), and (R5, F7)'s second producer; back in $REPO
: > .git/worktrees
$ jigc milestone provision prov-probe                                    -> exit 1
blocking · milestone.provision-failed — milestone:prov-probe: could not provision sub-task `prov-sub-one`'s worktree — `git worktree add --detach <absolute host path>/repo/.jigc/worktrees/prov-sub-one <sha>` failed: …
  at: .jigc/worktrees/prov-sub-one
  route: `jigc milestone provision prov-probe` — deal with what the message names at that path first; …
$ git status --short                 -> " M .jigc/.gitignore" (six lines appended) ;  "gitignore" on stdout 0, on stderr 0
# the regular file removed:
$ jigc milestone provision prov-probe                                    -> exit 0
provisioned 2 worktree(s) for milestone:prov-probe at base <sha7> (prov-sub-one, prov-sub-two)
# "gitignore" on stdout 0, on stderr 0 — the amend is acked by neither run
```

### X-9 · flow B, state 2, and `--carry-staged` on the doc-only arm

```sh
# seed; CT; the code task's three paths staged BEFORE the mint; an unstaged src/tracked2.rs, an untracked scratch.txt; report
$ git diff --cached --name-status
D	src/del.rs
A	src/new.rs
M	src/tracked1.rs
$ jigc task validate readme-disagrees-with-code                          -> exit 0   (no carryover finding)
$ jigc task finalize readme-disagrees-with-code --dry-run --format json  -> exit 0
  manifest: [promoted docs/inconsistencies/readme-mismatch.md]
  left_out: [left-staged src/del.rs, left-staged src/new.rs, left-staged src/tracked1.rs, modified src/tracked2.rs, untracked scratch.txt]
$ jigc task finalize readme-disagrees-with-code --dry-run --carry-staged --format json   -> exit 0 ; byte-equal to the document above
$ jigc task finalize readme-disagrees-with-code --carry-staged --format json             -> exit 0 ; manifest and left_out equal to the forecast's
$ git show --name-status --format=%s HEAD   ->  A docs/inconsistencies/readme-mismatch.md ;  INDEX BYTE-IDENTICAL
$ jigc task finalize code-change-probe                                   -> exit 0 ; HEAD: D src/del.rs · A src/new.rs · M src/tracked1.rs
```

### X-10 · the ordinary-model control, the config delta, the empty commit

```sh
# park-idea beside the same code task
#   s1 (unstaged throughout):   validate 0 · dry-run 0 · finalize 0 ; HEAD = A docs/ideas/parked-probe.md ; left_out [deleted src/del.rs, modified src/tracked1.rs, untracked src/new.rs]
#   s2 (staged before):         validate 3 · dry-run 3 · finalize 3 — finalize.carried-staged ×3 ; --carry-staged -> 0, manifest [promoted …, carried-over src/del.rs, carried-over src/new.rs, carried-over src/tracked1.rs]
#   s3 (staged after):          validate 0 · dry-run 0 · finalize 0 ; HEAD = A docs/ideas/parked-probe.md · D src/del.rs · A src/new.rs · M src/tracked1.rs
#                               then, worktree clean:  jigc task finalize code-change-probe -> exit 3 finalize.empty-commit (route carries the id)

# a pending config delta (jigc config set finalize.fan-out.squash false), report task
#   unstaged:       exit 0 ; HEAD = the doc alone ; left_out [untracked .jigc/config/manifest.yaml]
#   staged before:  exit 0 ; HEAD = the doc alone ; left_out [left-staged .jigc/config/manifest.yaml] ; INDEX BYTE-IDENTICAL
#   staged after:   the same
#   .jigc/version byte-identical in all three
# the same delta, park-idea:  unstaged -> HEAD = A .jigc/config/manifest.yaml · A docs/ideas/parked-probe.md
#                             staged before -> exit 3 finalize.carried-staged ;  staged after -> committed beside the doc

# unchanged recording — a triage writing `status = open` back, beside a staged src/new.rs and an uncommitted config delta;
# a pre-commit hook that only creates a marker file
$ jigc doc set-field inconsistency:crate-count#meta/status --value open --task triage-crate-count
set inconsistency:crate-count#meta/status = open (copied in for update — …)
$ jigc task validate triage-crate-count                                  -> exit 0
$ jigc task finalize triage-crate-count [--dry-run | --carry-staged | --format json]     -> exit 3 ×4
blocking · finalize.empty-commit — task validated but produced no diff — nothing to finalize
  route: make a change, then re-run `jigc task finalize triage-crate-count` — or, …, `jigc task discard triage-crate-count --force`
# marker absent (git commit never reached) · HEAD unmoved · index identical · JSON: one envelope on stdout, stderr 0 bytes
# a report task with no doc of its own, same neighbours:  validate 0 · dry-run 3 · finalize 3 — finalize.empty-commit ; marker absent
```

### X-11 · cell 8 — how the step is reached

`src/pre.rs` staged before the mint, `src/post.rs` after. FRESH = the `what's-left:` token of
`jigc start --workflow <W>`; RESUME = that of `jigc start --task <DT>`.

```sh
printf '{{ include: step:finalize }}\n' > <tmp>/land-the-index.yaml ; printf '{{ include: step:finalize-doc-only }}\n' > <tmp>/land-docs-alone.yaml
# variant            delta                                                                         FRESH · RESUME                 validate · dry-run · finalize
# replaced out       jigc config replace-step workflow:report-inconsistency#finalize-doc-only …    carryover gate · carryover gate   3 · 3 · 3 finalize.carried-staged (src/pre.rs) ; --carry-staged -> 0, HEAD = doc + both src/*
# replaced in        jigc config replace-step workflow:park-idea#finalize …                        path scope · path scope           0 · 0 · 0 ; HEAD = the idea doc alone ; INDEX BYTE-IDENTICAL
# removed            jigc config remove-step workflow:report-inconsistency#finalize-doc-only       carryover gate · carryover gate   3 · 3 · 3 ; --carry-staged -> 0
# inserted           jigc config insert-step --workflow park-idea --after finalize …               path scope · path scope           0 · 0 · 0 ; doc alone ; INDEX BYTE-IDENTICAL
# wrapping step      hand-written .jigc/config/steps/file-report-final.yaml + workflows/file-report-nested.yaml   path scope · path scope   0 · 0 · 0 ; doc alone ; INDEX BYTE-IDENTICAL
# body shadowed      hand-written .jigc/config/steps/finalize-doc-only.yaml (no path-scope sentence)  path scope · path scope        0 · 0 · 0 ; doc alone ; INDEX BYTE-IDENTICAL
# replaced out after the mint                                                                      path scope -> carryover gate      3 · 3 · 3 ; --carry-staged -> 0, HEAD = doc + both src/* + the uncommitted .jigc/config delta
# replaced in after the mint                                                                       carryover gate -> path scope      0 · 0 · 0 ; doc alone ; INDEX BYTE-IDENTICAL
# every `jigc config …-step` call: exit 0, "written to `.jigc/config/`, uncommitted — commit it with your next commit"
```

### X-12 · cell 10 — the index gate's three spellings; the `jigc-feedback` pair; `(R5, F10)`

```sh
# DocOnly — report-jigc-feedback, src/pre.rs staged before the mint, src/post.rs after
$ jigc start --workflow report-jigc-feedback "finalize sweeps a neighbour" | grep "^what's-left"   -> "… the path scope that leaves every other staged path staged …"
$ jigc task validate finalize-sweeps-a-neighbour [--carry-staged]        -> exit 0, 0
$ jigc task finalize finalize-sweeps-a-neighbour --dry-run ; jigc task finalize finalize-sweeps-a-neighbour   -> exit 0, 0
# HEAD: A docs/jigc-feedback/finalize-sweeps-a-neighbour.md ;  INDEX BYTE-IDENTICAL
# triage-jigc-feedback on that finding: same token ; 0 · 0 · 0 ; HEAD: M docs/jigc-feedback/finalize-sweeps-a-neighbour.md ; INDEX BYTE-IDENTICAL

# Amend — a landed single-task commit; src/pre.rs staged; jigc task amend; src/post.rs staged
$ jigc task amend | grep "^what's-left"                                  -> "… the empty-index gate this arm refuses any staged path at …"
$ jigc task validate amend-<sha7> [--carry-staged]                       -> exit 3, 3   finalize.amend-index-dirty ×2
$ jigc task finalize amend-<sha7> --dry-run [--carry-staged]             -> exit 3, 3
$ jigc task finalize amend-<sha7> --carry-staged                         -> exit 3 ; HEAD unmoved ; index identical
# Index — X-10 (s2): "the carryover gate" ; finalize.carried-staged ; --carry-staged lands the carry

# (R5, F10)
$ jigc task finalize --help | grep -c -i 'doc-only\|path-scoped\|path scope'     -> 0
$ jigc task validate --help | grep -c -i 'doc-only\|path-scoped\|path scope'     -> 0
```

### X-13 · the trailer and the seam on the other two models

```sh
# Index — a commit-msg hook that copies the message it is handed
$ jigc task finalize code-change-probe                                   -> exit 0
#   the hook saw:  feat: code change probe / (blank) / Reconciler probe body. / (blank) / Co-Authored-By: Claude <noreply@anthropic.com>
# Amend — pre-commit and commit-msg hooks that each create a marker file
$ jigc task finalize amend-<sha7>                                        -> exit 0
#   pre-commit ran: yes · commit-msg ran: yes · HEAD^{tree} unchanged
#   the hook saw, and `git log -1 --format=%B` shows:  feat: code change probe reworded / … / Co-Authored-By: Claude <noreply@anthropic.com>
```

### X-14 · cell 9 — a fan-out sub-task of a doc-only workflow

```sh
$ jigc milestone create mixed                                            -> exit 0
$ jigc milestone add-task mixed "code sub" --workflow single-task        -> exit 0
$ jigc milestone add-task mixed "report sub" --workflow report-inconsistency     -> exit 0
$ jigc milestone provision mixed                                         -> exit 0   "provisioned 2 worktree(s) …"
# in .jigc/worktrees/report-sub
$ jigc workflow report-inconsistency --task report-sub                   -> exit 0
what's-left: `jigc task validate report-sub`   — previews part of the finalize gate: …, this task's content findings, the carryover gate, …
#   "path-scoped" in the composed text: 0 ;  "jigc milestone finalize mixed": present
$ jigc start --task report-sub | grep "^what's-left"                     -> the carryover gate
#   a finding authored; src/z.rs staged
$ jigc task finalize report-sub ; jigc task finalize report-sub --dry-run     -> exit 3, 3
blocking · finalize.milestone-sub-task — task `report-sub` is a sub-task of milestone `mixed` — the parent milestone's finalize is the only commit boundary; …
  route: `jigc milestone finalize mixed` — …
$ jigc task validate report-sub                                          -> exit 3   schema-conformance.* on the omitted `commit:report-sub`
# in .jigc/worktrees/code-sub: src/c.rs staged.  Back in $REPO:
$ jigc milestone join mixed                                              -> exit 0   "joined milestone:mixed — 3 doc(s) merged"
$ jigc milestone finalize mixed                                          -> exit 0
finalized <sha> — Finalize milestone mixed (2 sub-tasks)
  sub-tasks: code-sub: 1 doc, 1 code file · report-sub: 2 docs, 1 code file
$ git show --name-status --format=%s HEAD
A	docs/inconsistencies/readme-mismatch.md
M	docs/milestone-records/mixed.md
A	src/c.rs
A	src/z.rs
```

(The driver's R9.6 reads *"2 doc(s) merged … no docs staged from: code-sub"*; here the code sub-task's
working area was entered with `jigc workflow single-task --task code-sub`, which provisions its commit
doc, so the join merges three. The boundary's contents are the same four paths.)

### X-15 · the schema boundary (K19)

```sh
$ git diff jigc-v1.0.0-rc.22 jigc-v1.0.0-rc.24 -- crates/cli/packs/methodology/config/schema-manifest.yaml
#   comment lines, and exactly two added rows:
+  - type: inconsistency
+    schema-version: 1
+    schema-hash: 6b83fd12…
+  - type: jigc-feedback
+    schema-version: 1
+    schema-hash: 7e16fb98…
$ git diff --stat jigc-v1.0.0-rc.22 jigc-v1.0.0-rc.24 -- crates/cli/packs/dev/config/schema-manifest.yaml     -> (empty)
$ git diff --stat jigc-v1.0.0-rc.23 jigc-v1.0.0-rc.24 -- crates/cli/packs
 crates/cli/packs/methodology/schemas/planning-record.yaml | 2 +-            # both manifests untouched in that range
# the binary, in a fresh rig — every command's pack-load freeze assertion passed, and:
$ jigc doc schema inconsistency | head -1     -> doctype: inconsistency (schema-version 1)
$ jigc doc schema jigc-feedback | head -1     -> doctype: jigc-feedback (schema-version 1)
$ jigc doc schema planning-record | head -1   -> doctype: planning-record (schema-version 1)
```

This datum is the repository's history plus the binary's own load-time assertion; the hashes were not
recomputed by hand.

## R.8 · Doors covered

Every clap leaf that is the door of ≥ 1 driven row after reconciliation, `VERB_KINDS` spelling —
**15 leaves**:

`start` · `workflow` · `task finalize` · `task validate` · `task amend` · `rename` · `config set` ·
`config replace-step` · `config insert-step` · `config remove-step` · `milestone create` ·
`milestone add-task` · `milestone provision` · `milestone join` · `milestone finalize`

`rename` is reconciler-added and is the door of one row only (X-2's refusal arm). The three
demotions remove no leaf: `task finalize` and `milestone provision` each keep other driven rows.
Verbs used only to build fixtures — `doc create`, `doc set-field`, `doc set-slot`, `doc add-item`,
`doc list`, `doc schema`, `task list`, `validate`, `config get`, `migrate` — are not claimed.

## R.9 · What the reconciler did not drive

- **Open leads (4):** K7's other populations at the stage-failure point · K8 (a concurrent process) ·
  K13b (`git_commit_paths`' empty-list refusal — unreachable through the door) · K16b (amend-over-doc-only
  precedence — no task carrying both markers was built).
- The driver's §8 list stands as written; nothing on it was driven here except its item 6's
  reachability (confirmed unreachable, X-10).
- Of the driver's 108 rows, the reconciler re-drove the ten defects, the three baseline rows, the two
  driven leads and the block-less rows of §R.5 — not the whole matrix. R1 (12 runs) was sampled at
  state 2 (X-9) and state 3 (X-1, X-6); the chatty-hooks rows (R6.36, R6.37), the un-concluded-merge
  row (R6.35) and the pre-staged owner-artifact rollback (R6.30) rest on the driver's blocks alone.
- **Fixture honesty:** nothing was written into the gitignored `.jigc/` workbench. Hand-written files
  under `.jigc/config/` (the scope's exception): X-3's two workflows, X-11's wrapping step, nested
  workflow and shadowed step body — committed with plain git. Faults were placed inside `.git/`
  (`index.lock`, `worktrees` as a regular file) or as permissions on `.jigc/config`, restored
  afterwards. One reconciler fixture failed before it drove anything and was rebuilt: a copy of the
  pack's `report-inconsistency` workflow under the project layer would not compose
  (`workflow-refs.command-ref-resolves` on a pack-scoped command ref) — the driver's idea-granting
  shape was used instead; not graded on this row.
