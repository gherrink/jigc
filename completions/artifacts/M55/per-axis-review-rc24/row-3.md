<!-- Reconciled ROW 3 file (store exit codes / reconciliation), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis3` in the body means ROW 3 of this run, not numbered axis 3. -->

# Row 3 · store exit codes / reconciliation — driver record (rc.24), reconciled

> **Reconciler's note.** Sections 1–7 are the Opus driver's record, unchanged except for the demotions marked
> *(reconciler)* in rows 102 and 137 and in the row count. Section 8, the **Reconciliation ledger**, is the
> reconciler's: it re-drives every driver defect, drives every claim of the Codex source pass, and **supersedes
> the driver's verdict line below** — one finding, `(R3, F7)`, shows both halves of the tier-1 predicate, and
> `(R3, F4)` is re-tiered from 3 to 2. This row **has** a source pass (`codex/axis3-codex.md`, exit 0).

**Binary:** `~/.local/bin/jigc`, asserted first: `jigc --version` → `jigc 1.0.0-rc.24`, exit 0. Release
posture. Every row below ran on that binary; nothing here is marked driven from a source read.

**Verdict for the exit rule: NO TIER-1 ROW.** No exit-0 loss and no repository harm was produced
through any committing, destroying or moving door in this row's subject. Six new rows are proposed at
tier 3; two carried baseline defects are STILL-OPEN at tier 3; the two declared gaps (`(7, C-2)`, F21)
are STILL-OPEN as expected; the `probe-unreliable` lead is CLOSED by a drive.

**Environment held constant:** `CLAUDECODE` set (every jigc-made commit in a rig carries
`Co-Authored-By: Claude <noreply@anthropic.com>`; not a datum of this row). `JIGC_DOC_CODE_PROBE` unset
except in the probe cells. Every rig came from `dev/jigc-rig <state> --binary ~/.local/bin/jigc`, two-step
eval, stdout only, `[ -n "$REPO" ]` guarded; every teammate clone was made after the eval. No teardown.

---

## 1 · Door set and registry counts — read off the code at `aa6666cb`

| registry | file | count I read | instrument author's | difference |
|---|---|---|---|---|
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs` | **7** — `probe-unreliable` · `oob-rename` · `unmigrated-corpus` · `ahead-corpus` · `orphaned-instance` · `home-vacated` · `foreign-squatter`; `oob-rename`'s predicate is `code == "reconciliation.rename" && severity == Blocking` | 7 | none |
| `STORE_FAMILIES` | `crates/engine/src/validate.rs` | **7** | 7 | none |
| `EXIT_CODES` | `crates/cli/src/task.rs` | **5** classes (0 · 1 · 2 · 3 · 4) | 5 | none |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs` | **4** (`store.not-found` · `store.no-such-leaf` · `store.unknown-type` · the fixed-identity code) | 4 | none |
| dangling-baseline producers | `crates/engine/src/file_state.rs` | **3 advisory producers of `reconciliation.rename`**: `rename_dangling_baseline_finding` (a branch carries the path) · `rename_orphaned_baseline_finding` (none does) · `live_record_finding` (the M52 carve-out: the live milestone's own record, routed at no verb) | 2 named ("one code, two routes") | **a datum:** the scope names two; the code has a third advisory producer on the same code. It is M52's, not M55's, and was **not driven** here (§5) |

**Doors (clap leaves, `VERB_KINDS` spelling) that are the door of ≥1 driven row:** `validate` ·
`task validate` · `task finalize` · `milestone finalize` · `milestone add-task` · `unmanage` · `ingest` ·
`doc list` · `rename` · `migrate-corpus` · `task amend` · `start` · `workflow` · `config insert-step` ·
`doc set-slot` · `upgrade` · `task discard` — plus the installed pre-commit hook's store sweep, which is
not a clap leaf.

**Route kind is not on the wire.** The JSON finding carries `route` as a plain string and no kind key, so
the *route kind* column below is read from the producer (`Route::human` / `Route::informational` /
`Route::mechanical`) and is the one column in this record that is not an observation of the binary.

---

## 2 · The (door, cell) table

`T` = text arm, `J` = `--format json`. Where both arms were driven the `(code, target)` key was compared
and the row says so. Repro blocks are in §3 by the id in the last column.

### Cell 1 — exit flips

| # | door | cell | argv | exit | code | route kind | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `validate` | content finding, no member (uncommitted hand edit) | `jigc validate` T+J | 0 | `file-state.hash-matches` blocking | Human | trailer `report-only at store scope (exit 0); 1 of them gate at …`; J `report_only: true`, `blocking_probes: ["file-state"]` | matches | P1 |
| 2 | `validate` | `oob-rename` strong, placement singleton, staged `git mv` | T+J | 1 | `reconciliation.rename` blocking, target `docs/roadmap.md` | Human | trailer `out-of-band rename detected — …`; J `report_only: false` | matches (exit, key parity) | P2 |
| 3 | `validate` | `oob-rename` strong, location doctype, staged `git mv` | T+J | 1 | `reconciliation.rename` blocking, target the old path | Human | same trailer; key parity | matches | P3 |
| 4 | `validate` | `oob-rename` strong, move **committed** (hook bypassed, cell is the state) | T+J | 1 | same | Human | same | matches | P2, P17 |
| 5 | `validate` | `oob-rename` weak, path **has** history, uncommitted `rm` | T+J | 1 | `reconciliation.rename` blocking | Human | trailer names a `git mv` and *this commit* | exit matches · **trailer: `(7, A7-F1)` STILL-OPEN** | P3, P7 |
| 6 | `validate` | `oob-rename` weak, committed `git rm` | T+J | 1 | same | Human | same trailer | exit matches · A7-F1 | P3, P7 |
| 7 | `validate` | `probe-unreliable`, override → non-executable file | T+J | 1 | `pack-probe-integrity.probe-failure`, target `doc-code` | Human | trailer `pack-probe-integrity finding(s) present — the sweep could not complete and exits non-zero`; J `report_only: false`, `blocking_probes: ["pack-probe-integrity"]` | matches | P4 |
| 8 | `validate` | `probe-unreliable`, override exits 3 with no output | T+J | 1 | same key | Human | same trailer | matches | P4 |
| 9 | `validate` | `probe-unreliable`, override exits 0 with unparseable output | T+J | 1 | same key | Human | same trailer | matches | P4 |
| 10 | `validate` | override names no file (the operational arm) | T+J | 1 | none (operational error) | none | T: one line on stderr; J: `{"error": …}` on **stderr**, stdout empty | matches (exit-1 class; row 2 owns the why) | P4 |
| 11 | `validate` | `unmigrated-corpus` (stamp 2 → 1, committed) | T+J | 1 | `schema-conformance.schema-version-current`, target `changelog:changelog` | Human | trailer `the committed corpus is below its schema-version — …` | matches | P5 |
| 12 | `validate` | `ahead-corpus` (stamp → 3, committed) | T+J | 1 | `schema-conformance.schema-version-ahead` | Human | trailer `a committed doc is stamped above this build's schema-version — …` | matches | P5 |
| 13 | `validate` | `orphaned-instance` (methodology pack leaves the composition) | T+J | 1 | `schema-conformance.orphaned-instance` ×2 | Human | trailer `a stamped committed doc is claimed by no resolved doctype — …` | matches | P6 |
| 14 | `validate` | `home-vacated` alone (after `unmanage` cleared the rename row) | T+J | 1 | `schema-conformance.home-vacated`, target `CHANGELOG.md` | Human | trailer `a declared home the repository committed into is empty — …` | matches | P7 |
| 15 | `validate` | `foreign-squatter` (a never-adopted file at a managed home) | T+J | 1 | `schema-conformance.unadopted-instance` **advisory** | Human | trailer `a never-adopted file sits at a managed home — …`; J `report_only: false`, `blocking_probes: []` | matches (an advisory row that flips the exit, as declared) | P8 |
| 16 | hook | strong `git mv` staged in this commit | `git commit -m …` | 1 | — | — | `jigc: out-of-band managed-doc rename staged in this commit — … (commit blocked).`; HEAD unchanged | matches | P2 |
| 17 | hook | move landed in a **prior** commit, unrelated commit now | `git commit -m unrelated` | 0 | — | — | `… exists in the committed tree … (not staged in this commit; commit not blocked).` | matches (masking-trap guard) | P2 |
| 18 | hook | `probe-unreliable` | `git commit` under the override | 0 | — | — | silent | matches (hook header: falls through to silent exit 0) | P4 |
| 19 | hook | stale / ahead stamp commits | `git commit -q -am …` ×2 | 0 | — | — | silent | matches | P5 |
| 20 | `doc list` | foreign-squatter | `jigc doc list` | 0 | — | — | row `changelog:changelog  CHANGELOG.md  unregistered` | matches | P8 |
| 21 | `ingest` | foreign-squatter | `jigc ingest` | 0 | `conformance.section-missing` on the row | Mechanical/Human | `needs-reconcile CHANGELOG.md → changelog`, routed at `jigc migrate … --as changelog` | matches | P8 |
| 22 | `doc list` | orphaned | `jigc doc list` | 0 | — | — | two rows `(none)  docs/…  orphaned` | matches | P6 |
| 23 | `ingest` | orphaned | `jigc ingest` T+J | 0 | none; J rows `finding: null` | — | `unmanaged docs/ — 2 file(s) … fine to stay plain` + `no action needed` | **DEFECT `(7, A7-F2)` STILL-OPEN** | P6 |
| 24 | `doc list` | managed singleton removed from the worktree (weak arm) | `jigc doc list` | 0 | — | — | the row is simply absent | matches (no claim made) | P7 |
| 25 | `ingest` | any tracked `.md` missing from the worktree | `jigc ingest` T+J | **1** | none | none | `could not read the candidate at "$REPO/<path>": No such file or directory (os error 2)`; J `{"error": …}` | **DEFECT `(R3, F2)`** | P7, P8 |

### Cell 2 — L1 at store scope

| # | door | cell | argv | exit | code | route kind | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 26 | `validate` | conformant · `git pull` from a second clone | T+J | 0 | `file-state.hash-matches` **advisory**, target `VISION.md` | Informational | route `the baseline lags \`HEAD\`; absorbed at the next finalize`; J `report_only: true`, `blocking_probes: []` | matches | P9 |
| 27 | `validate` | conformant · merge (`--no-ff`) | T+J | 0 | same | Informational | same | matches | P9 |
| 28 | `validate` | conformant · hand edit, plain `git commit` | T+J | 0 | same | Informational | same | matches | P9 |
| 29 | `validate` | non-conformant · pull | T+J | 0 | `file-state.hash-matches` **blocking** + three `conformance.*` blocking | Human / none | route `review the out-of-band edit …`; J `blocking_probes: ["conformance","file-state"]` | matches (keeps blocking; exit 0 — no member) | P9 |
| 30 | `validate` | non-conformant · merge | T+J | 0 | same | same | same | matches | P9 |
| 31 | `validate` | non-conformant · hand commit | T+J | 0 | same | same | same | matches | P9 |
| 32 | `validate` | stale stamp, hand commit | T+J | 1 | `file-state.hash-matches` advisory + `schema-version-current` blocking | Informational / Human | unmigrated-corpus trailer | matches (see O-12) | P9 |
| 33 | `task finalize` | the lag route's promise, conformant | an unrelated task's `jigc task finalize` then `jigc validate` J | 0 / 0 | `reconciliation.absorb` advisory at `VISION.md`; store row gone | Informational | the route is true | matches | P9 |
| 34 | `task finalize` | the lag route's promise, stale stamp | same | 0 / 1 | absorb; hash-matches row gone, `schema-version-current` stays | — | the route is true | matches | P9 |
| 35 | hook | the hand-edit commit itself | `git commit -q -am …` | 0 | — | — | silent | matches | P9 |
| 36 | `validate` | a pulled edit to a **location-doctype** doc (the milestone record) | T+J | 0 | `file-state.hash-matches` advisory at `docs/milestone-records/vision-pass.md` | Informational | same route | matches at store scope; see row 58 | P12 |

### Cell 3 — L1 at the task gate

| # | door | cell | argv | exit | code | route kind | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 37 | `task validate` | change predates the task (pin has it) | T+J | 0 | `reconciliation.absorb` advisory, target `VISION.md` | Informational | `external edit absorbed` | matches | P10 |
| 38 | `task finalize` | same, `--dry-run` | T+J | 0 | same | Informational | `would commit … promoted VISION.md` | matches | P10 |
| 39 | `task finalize` | same, landing | T | 0 | same | Informational | committed `VISION.md` carries the teammate's line **and** the task's (before/after control) | matches | P10 |
| 40 | `task validate` | change during the task: stage, then pull | J | 3 | `reconciliation.conflict-block`, target `VISION.md` | Human | route `jigc task discard <id> --force … or revert the external edit on disk` | matches | P11 |
| 41 | `task finalize` | same, `--dry-run` | T | 3 | `finalize.base-mismatch`, target `task:<id>` | Human | `the moved history overlaps the task's work on \`VISION.md\`` | matches (the base pin is declared outside the preview) | P11 |
| 42 | `task finalize` | same, landing | T+J | 3 | same | Human | HEAD unchanged; teammate's line intact on disk and at HEAD; staged copy intact | matches — **no loss** | P11 |
| 43 | `start` | same, resume (`--task`) | `jigc start --task <id>` | 1 | not captured (output suppressed in the drive) | — | exit only | driven for the exit; surface NOT asserted | P11 |
| 44 | `doc set-slot` | pull during the task, **then** the first write | T | 0 | none | — | `copied in for update — the committed doc is now this task's staged copy` (the copy carries the teammate's line) | matches the write contract; see O-5 | P11 |
| 45 | `task validate` | pull, then stage | J | 3 | `reconciliation.conflict-block` | Human | same route | matches (declared: a change after the mint still blocks) | P11 |
| 46 | `task finalize` | pull, then stage (`--dry-run`, landing, J) | T+J | 3 | `finalize.base-mismatch` | Human | HEAD unchanged | matches | P11 |
| 47 | `task validate` | uncommitted hand edit during the task | J | 3 | `reconciliation.conflict-block` | Human | same route | matches | P11 |
| 48 | `task finalize` | same, `--dry-run` | T | 3 | `reconciliation.conflict-block` | Human | same | matches | P11 |
| 49 | `task finalize` | same, landing | T+J | 3 | same | Human | HEAD unchanged, hand edit still on disk | matches | P11 |
| 50 | `doc set-slot` | a pinned edit that fails conformance, first write | T | 1 | `write.non-reparseable` | Human | route opens `nothing was persisted` | **DEFECT `(R3, F3)`** — the copy-in *was* persisted | P13 |
| 51 | `task validate` | pinned non-conformant, task untouched | J | 3 | `reconciliation.conformance-block`, target `VISION.md` | Human | the hand-repair sanction | matches | P13 |
| 52 | `task validate` | pinned non-conformant, after the refused write | J | 3 | `reconciliation.conflict-block` (+ three `conformance.*`) | Human | conflict-block, **not** conformance-block | matches the cell (P1) — reached through F3 | P13 |
| 53 | `task finalize` | same, `--dry-run` and landing | J | 3 | same | Human | HEAD unchanged | matches | P13 |
| 54 | `milestone finalize` | pull **before** `milestone create` | J | 0 | none on the landed envelope | — | committed `VISION.md` carries both lines | matches | P12 |
| 55 | `milestone finalize` | pull **after** `milestone create` | J | 3 | `reconciliation.conflict-block`, target `VISION.md` | Human | route `jigc task list names the live tasks — discard the sub-task … or revert the external edit on disk, then re-run the join` | matches | P12 |
| 56 | `milestone add-task` | the milestone-record door, a pulled record edit | T+J | 1 | `reconciliation.conflict-block` (T); J is the flattened `{"error": …}` on stderr | Human | `the milestone record is machine-maintained and was edited out of band`; record untouched, HEAD unchanged | matches (no pin passed, P3) | P12 |
| 57 | `task finalize` | an unrelated task, over the pulled record edit | J | 0 | `reconciliation.absorb` advisory at the record | Informational | the record's drift is baselined | exit matches · **`(R3, F4)`** | P12 |
| 58 | `milestone add-task` | the record door again, after row 57 | T | 1 | `milestone.terminal` | Human | the hand-set `status: joined` is now the record's truth | **DEFECT `(R3, F4)`** | P12 |
| 59 | `task validate` / `task finalize` | conflict route's 2nd exit, read as *revert on disk* (pulled case) | J | 0 / 3 | none / `finalize.base-mismatch` | Human | green preview, then the block | matches the declared preview bound; O-4 | P14 |
| 60 | `task finalize` | conflict route's 2nd exit, read as *revert the commit* (`git revert`) | T | 0 | none | — | lands; the teammate's line is gone by the explicit revert commit | matches | P14 |

### Cell 4 — L2

Seven arms over a location doctype (`adr`), rig `fresh`: **local** (a local branch carries the path) ·
**remote** (only `refs/remotes/origin/…`) · **tag** (only `refs/tags/v-x`) · **reset** (`git reset --hard`
past the creating commit) · **rebase** (`git rebase --onto HEAD~2 HEAD~1`) · **amend** (the creating
commit amended to no longer carry the doc) · **ingest** (ingested, deleted before it was ever committed).

| # | door | cell | argv | exit | code | route kind | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|---|
| 61–62 | `validate` | local · remote | T+J | 0 | `reconciliation.rename` **advisory**, target the ADR path | Informational | message `… the checkout moved underneath the file-state cache, not a deletion`; route `nothing on this checkout needs to change — a branch switch left this baseline behind … switch back to that branch`; names no `unmanage`; J `report_only: true` | matches | P15 |
| 63–67 | `validate` | tag · reset · rebase · amend · ingest | T+J | 0 | `reconciliation.rename` **advisory**, same target | Human | message `… has no history at HEAD, and no branch carries it — the baseline outlived the doc`; route `no branch, local or remote-tracking, can bring <path> back — … drop it with \`jigc unmanage <path>\`` | matches (tag-only takes the `unmanage` arm, as `storage.md` → Derived caches declares) | P15 |
| 68–74 | `task validate` | all seven arms | J | 0 | the same row | same | **parity with store scope: `(code, target)`, severity, message and route byte-equal in all seven** | matches | P15 |
| 75–81 | `task finalize` | all seven arms, `--dry-run` | T | 0 | the same row | same | `would commit …` + the advisory | matches | P15 |
| 82–88 | `doc list` | all seven arms | T | 0 | — | — | the missing doc is not listed (`no committed docs`, or the seed alone) | matches | P15 |
| 89–95 | hook | all seven arms, an unrelated plain commit | `git commit -q -m …` | 0 | — | — | silent (the informational route carries no `git -C … mv`) | matches | P15 |
| 96 | `validate` | placement variant: a branch cut from the install commit, four singletons absent, `main` carries them | T+J | 0 | four `reconciliation.rename` advisory rows | Informational | switch-back route ×4; **no** `home-vacated` | matches | P16 |
| 97 | `task validate` | same | J | 0 | the same four | Informational | parity equal (4 / 4) | matches | P16 |
| 98 | `task finalize` | same, landing | J | 0 | the same four, advisory | Informational | lands `note.txt`; baselines untouched (`command grep` 1 before, 1 after) | matches | P16 |
| 99 | `ingest` | same | T | 0 | — | — | 3 candidates, nothing about the absent singletons | matches | P16 |
| 100 | `doc list` | same | T | 0 | — | — | `no committed docs` | matches | P16 |
| 101 | `milestone finalize` | local arm, a sub-task staging another ADR | J | 0 | none on the landed envelope (`committed` only) | — | lands; the dangling baseline survives (`command grep` 1) and the store row is still advisory after | matches (the advisory does not gate); O-7 | P17 |
| 102 *(reconciler: DEMOTED — not a driven row; the driver's own verdict counts it NOT DRIVEN for the L2 cell and it is §5 #2, so it cannot also be in the driven count)* | `milestone finalize` | placement variant, a sub-task with no contribution | J | 3 | `milestone.zero-contribution` | Human | — | **inconclusive for L2** — refused before the gate; counted NOT DRIVEN for the L2 cell | P16 |
| 103 | `validate` / `task validate` / `task finalize --dry-run` | **control** — absent **with** history (staged `git rm`) | T+J | 1 / 3 / 3 | `reconciliation.rename` **blocking**, same key at both scopes | Human | route `restore <path>, or confirm the deletion by dropping it from the index: \`jigc unmanage <path>\``; parity equal | matches (the genuine-deletion arm still blocks) | P15 (amend-control) |

### Cell 5 — every emitted route, run verbatim

| # | door | cell | argv | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 104 | (git) | strong arm, the **revert** exit | `git -C $REPO mv <new> <old>` then `jigc validate` | 0 / 0 | none | clears, placement and location | matches | P2, P3 |
| 105 | `rename` | strong arm, the **adopt** exit, in three states (placement staged · location staged · location committed) | `jigc rename <old-id> --to "<title>"` | **1** | `store.not-found` | `no managed doc \`<old-id>\` to rename (expected at <old path>)` | **DEFECT `(R3, F1)`** | P2, P3, P17, P18 |
| 106 | (git) + `validate` + `task validate` | L2 local, switch back | `git switch <branch>` | 0 | none | the row is gone at both scopes | matches | P15 |
| 107 | (git) + `validate` | L2 remote, switch back (git recreates the branch from the remote ref) | `git switch <branch>` | 0 | none | file present, row gone | matches | P15 |
| 108–112 | `unmanage` + `validate` + `task validate` | L2 tag · reset · rebase · amend · ingest, the emitted route | `jigc unmanage <path>` | 0 | none | ack `… there was no file at <path> to leave on disk`; baseline `command grep` 1 → 0; the row is gone at **both** scopes | matches (5 rows) | P15 |
| 113 | `unmanage` + `validate` | weak arm with history (committed `git rm`), the emitted route | `jigc unmanage <path>` | 0 | none | clears → `no findings` | matches | P3 |
| 114 | (git) + `validate` | `home-vacated`, worktree-only removal, the emitted route | `git -C $REPO checkout -- CHANGELOG.md` | 0 | none | clears | matches | P7 |
| 115 | `unmanage` + `validate` | `orphaned-instance`, the route's `unmanage` step alone | `jigc unmanage docs/roadmap.md` | 0 / 1 | row stays | exactly what the route says (*the stamp alone keeps this finding alive*) | matches | P6 |
| 116 | `start` · `task discard` | the `task.serial-collision` route's two exits (reached in cell 8) | `jigc start --task <id>` · `jigc task discard <id> --force` | 1 · 0 | `workflow-refs.include-resolves` · none | resume refuses on the same break; discard clears | matches | P20 |

### Cell 6 — `jigc unmanage`

| # | cell | argv | exit | surface asserted | before → after (`command grep -c`) | verdict | repro |
|---|---|---|---|---|---|---|---|
| 117 | managed, file present, index materialized | `jigc unmanage docs/decisions/beta-cache.md` T | 0 | `… dropped its file-state baseline + forward edges; the file is left on disk. It still sits at the managed home …`; `ls` finds the file | baseline 1 → 0 · edges 1 → 0 | matches | P19 |
| 118 | managed, file present | same, J | 0 | `{"path": …, "identity": "adr:alpha-cache", "dropped": true}`; file present | baseline 1 → 0 | matches | P19 |
| 119 | managed, file **absent** | `jigc unmanage <path>` T | 0 | `… there was no file at <path> to leave on disk` (the M55 fix); `ls` agrees | baseline 1 → 0 | matches | P3, P7, P15 |
| 120 | second run | T and J | 0 | `no-op: <path> is not managed (nothing to drop)`; J `dropped: false`, identity kept | 0 → 0 | matches | P15, P19 |
| 121 | never managed: `README.md` | T+J | 0 | no-op; J `identity: null, dropped: false` | — | matches | P19 |
| 122 | never managed: a missing path under a managed home | T+J | 0 | no-op; J `identity: "adr:never-was", dropped: false` | — | matches the declared key set | P19 |
| 123 | a directory · a path outside the repo | T | 0 | no-op | — | matches | P19 |
| 124 | a symlinked file · a path through a symlinked directory | T+J | 0 | no-op; J `identity: null`; the target's baseline untouched (1 → 1) | 1 → 1 | matches (nothing dropped through a link) | P19 |
| 125 | an orphaned doc, file present | `jigc unmanage docs/roadmap.md` | 0 | `unmanaged docs/roadmap.md — dropped its file-state baseline; the file is left on disk`; `ls` finds it | — | matches | P6 |
| 126 | re-adopt, the stated inverse | `jigc ingest` after row 117 | 0 | `adoptable … (adopted — indexed + baselined, no file moved)` | baseline 0 → 1 · edges 0 → 1 | matches | P19 |
| 127 | **other spellings of a managed path**: `./<path>` · `docs//…` · `docs/x/../…` · absolute · cwd-relative from a subdirectory | `jigc unmanage <spelling>` | 0 | `no-op: <spelling> is not managed (nothing to drop)` | baseline **1 → 1** | **DEFECT `(R3, F5)`** | P19 |

### Cell 7 — `(code, target)` parity

Asserted in every row above that carries both arms: the text arm's `code` + `at:` line and the JSON
arm's `key.code` + `key.target` agreed in all of rows 1–15, 26–32, 36–38, 61–67, 96. Store-vs-task
parity (key, severity, message, route) was compared mechanically in rows 68–74, 97 and 103 and was equal
in every one. No parity defect found.

### Cell 8 — F21

| # | door | cell | argv | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 128 | `config insert-step` | record a native step whose body is a broken include | `jigc config insert-step --workflow amend --after amend-message <file>` | 0 | none | `inserted step \`broken\` … uncommitted — commit it with your next commit` | matches (write-time adjudication does not read the body) | P20 |
| 129 | `validate` | the committed delta | T+J | 0 | **none** — `findings: []` | `no findings — the committed store validates clean` | **F21 STILL-OPEN (expected)** | P20 |
| 130 | `task amend` | the door that composes | T | 1 | `workflow-refs.include-resolves` | refuses | matches the F21 statement; and **`(R3, F6)`** | P20 |
| 131 | `validate` | **control** — the same include in a whole-file shadow | J | 0 | `workflow-refs.include-resolves`, target `workflow:amend`, blocking | reported, report-only | matches (the check runs; only the delta path is missed) | P20 |
| 132 | `validate` / `task amend` | **bound** — a cycle (`loop` includes `loop`) | J / T | 0 / 1 | none / `workflow-refs.include-cycle-absent` | missed at store scope, caught at compose | F21, same gap | P20 |
| 133 | `validate` / `task amend` | **bound** — a bad body inside the inserted native step (`Run: {{ cli.no-such-command }}`) | J / T | 0 / 1 | none / `workflow-refs.run-marker-not-shadowed` at `step:badref` | missed at store scope, caught at compose | F21, same gap (the compose gate reported the marker shadow first; the command-ref arm itself was not isolated) | P20 |
| 134 | `start` | a second composing door (`--workflow single-task` over its own broken delta) | T | 1 | `workflow-refs.include-resolves` | refuses — and leaves a live task | **`(R3, F6)`** | P20 |
| 135 | `workflow` | `--preview` over the same delta | T | 1 | `workflow-refs.include-resolves` (single-task) · `workflow.verb-routed` (amend) | refuses, mints nothing | matches | P20 |
| 136 | `upgrade` | the delta sweep over the broken delta | T | 0 | none | `1 recorded config delta(s) re-apply clean against the current pack` | matches its own contract (it checks re-application, not refs) — a second door that does not see the break | P20 |
| 137 *(reconciler: DEMOTED — marked driven with repro P20, and P20 carries no line for it; re-driven by the reconciler, §8 R-11e, where it reproduces)* | `config insert-step` | the same basename into a second workflow | T | 1 | `config.step-id-collision` | routed | matches | P20 |

### Baseline re-drives (also in §6)

| # | door | cell | argv | exit | verdict | repro |
|---|---|---|---|---|---|---|
| 138 | `validate` | `(7, A7-F1)` — `rm CHANGELOG.md`, uncommitted, unstaged | T+J | 1 | STILL-OPEN | P7 |
| 139 | `validate` + `ingest` | `(7, A7-F2)` — one commit, both doors | T+J | 1 / 0 | STILL-OPEN | P6 |
| 140 | `validate` · `doc list` · `ingest` · `migrate-corpus` | `(7, C-2)` — mentions of the root-placement orphan `VISION.md` | four doors | — | STILL-OPEN (expected) | P6 |

**Rows driven: 140. Rows not driven: 17 (§5).** *(reconciler: **138** driven by the driver — rows 102 and 137 demoted; row 43 is driven for its exit only, and rows 68–95 share one summarised block, P15, spot re-driven in §8 R-10.)*

---

## 3 · Repro blocks

Shared helpers used below (all fixtures through the binary; the teammate is a second clone made after the
eval):

```sh
J=~/.local/bin/jigc
rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
# land_adr <title> <slug>: start --workflow single-task → doc create adr --title → set-slot context/decision/
#   consequences → set-field commit#type docs, #scope → set-slot commit#summary → task finalize
# team: git clone --bare $REPO $RIG/origin.git; git remote add origin …; git clone $RIG/origin.git $RIG/teammate
#   (user.email mate@example.com); teammate edits, `git commit -qam`, pushes; the rig runs `git pull -q --ff-only`
# before/after controls under .jigc/: `command grep -c <marker> .jigc/state/file-state.json` (and index/edges.json)
```

### P1 · content finding, no member (rig `committed-singletons`)

```
(append one line to VISION.md's thesis, do not commit)
$ jigc validate                      -> exit 0
blocking (gates at finalize) · file-state.hash-matches — on-disk content of `VISION.md` differs from the recorded state
  at: VISION.md
  route: review the out-of-band edit to `VISION.md` and re-author it through the owning workflow
2 finding(s) — report-only at store scope (exit 0); 1 of them gate at `jigc task validate` / `jigc task finalize` / `jigc milestone finalize`; the rest are store-scope advisories that gate nowhere — follow each finding's route above.
$ jigc validate --format json        -> exit 0
  report_only=true scope=store blocking_probes=["file-state"] schema_version=3
  KEY (file-state.hash-matches, VISION.md) blocking
```

### P2 · `oob-rename`, strong arm, placement singleton (rig `committed-singletons`)

```
$ git mv docs/roadmap.md docs/plan.md
$ jigc validate                      -> exit 1
blocking (gates at finalize) · reconciliation.rename — tracked managed doc roadmap:roadmap (docs/roadmap.md) is missing; docs/plan.md has the same content hash — likely renamed via `git mv`
  at: docs/roadmap.md
  route: adopt it as a CLI-owned rename (re-points every referrer atomically): `jigc rename roadmap:roadmap --to "<New Title>"`; or revert the move: `git -C $REPO mv docs/plan.md docs/roadmap.md`
blocking · schema-conformance.orphaned-instance — committed doc `docs/plan.md` sits at a jigc-managed home …
blocking · schema-conformance.home-vacated — `roadmap` homes its one document at `docs/roadmap.md`, … its deletion is staged but not committed …
out-of-band rename detected — a structural-identity change this commit introduced; the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`).
$ jigc validate --format json        -> exit 1
  report_only=false blocking_probes=["reconciliation","schema-conformance"]
  KEY (reconciliation.rename, docs/roadmap.md) blocking
  KEY (schema-conformance.orphaned-instance, docs/plan.md) blocking
  KEY (schema-conformance.home-vacated, docs/roadmap.md) blocking

# the hook door
$ git commit -m "move roadmap"       -> exit 1   HEAD unchanged
jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).
$ git commit --no-verify -m "move roadmap"      (hook bypassed: the cell is the committed-move state)
$ jigc validate                      -> exit 1   (same rename row; home-vacated now names the removing commit)
$ echo x >> README.md; git add README.md; git commit -m unrelated     -> exit 0
jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate` for details (not staged in this commit; commit not blocked).

# routes, verbatim (fresh rig, same staged move)
$ git -C $REPO mv docs/plan.md docs/roadmap.md  -> exit 0 ; jigc validate -> exit 0 (the rename row is gone)
$ git mv docs/roadmap.md docs/plan.md
$ jigc rename roadmap:roadmap --to "Plan"       -> exit 1
blocking · store.not-found — no managed doc `roadmap:roadmap` to rename (expected at docs/roadmap.md)
  at: roadmap:roadmap
  route: `jigc describe` lists the doctype surface — check the id you typed against it
$ git status --short  -> R  docs/roadmap.md -> docs/plan.md      ; jigc validate -> exit 1 (unchanged)
```

### P3 · `oob-rename`, strong and weak arms, location doctype (rig `fresh`, one ADR landed)

```
$ git mv docs/decisions/single-node-cache.md docs/decisions/one-node-cache.md
$ jigc validate                      -> exit 1
advisory · file-state.un-baselined — committed doc `docs/decisions/one-node-cache.md` is not yet baselined …
blocking (gates at finalize) · reconciliation.rename — tracked managed doc adr:single-node-cache (docs/decisions/single-node-cache.md) is missing; docs/decisions/one-node-cache.md has the same content hash — likely renamed via `git mv`
  route: adopt it as a CLI-owned rename …: `jigc rename adr:single-node-cache --to "<New Title>"`; or revert the move: `git -C $REPO mv docs/decisions/one-node-cache.md docs/decisions/single-node-cache.md`
out-of-band rename detected — a structural-identity change this commit introduced; …
$ jigc validate --format json        -> exit 1 ; KEY (reconciliation.rename, docs/decisions/single-node-cache.md) blocking
$ jigc rename adr:single-node-cache --to "One node cache"   -> exit 1  store.not-found (as P2)
$ git -C $REPO mv docs/decisions/one-node-cache.md docs/decisions/single-node-cache.md ; jigc validate -> exit 0 "no findings"

# weak arm, the path has history
$ rm docs/decisions/single-node-cache.md
$ jigc validate                      -> exit 1
blocking (gates at finalize) · reconciliation.rename — tracked managed doc adr:single-node-cache (docs/decisions/single-node-cache.md) is missing
  route: restore docs/decisions/single-node-cache.md, or confirm the deletion by dropping it from the index: `jigc unmanage docs/decisions/single-node-cache.md`
out-of-band rename detected — a structural-identity change this commit introduced; the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`).
$ git rm -q docs/decisions/single-node-cache.md ; git commit -q -m "delete the adr"   -> exit 0 (hook silent)
$ jigc validate                      -> exit 1 (the same row and the same trailer)
  command grep -c 'single-node-cache' .jigc/state/file-state.json -> 1
$ jigc unmanage docs/decisions/single-node-cache.md   -> exit 0
unmanaged docs/decisions/single-node-cache.md (adr:single-node-cache) — dropped its file-state baseline + forward edges; there was no file at docs/decisions/single-node-cache.md to leave on disk
  command grep -c 'single-node-cache' .jigc/state/file-state.json -> 0
$ jigc validate                      -> exit 0 "no findings — the committed store validates clean"
```

### P4 · `probe-unreliable` (rig `vendored`; `JIGC_DOC_CODE_PROBE` unset in the control)

```
$ jigc validate                                          -> exit 0 "no findings"
# three overrides under $RIG/probes: `nonexec` (mode 644) · `exit3` (#!/bin/sh; exit 3) · `garbage` (echo garbage; exit 0)
$ JIGC_DOC_CODE_PROBE=$RIG/probes/nonexec jigc validate  -> exit 1
blocking (gates at finalize) · pack-probe-integrity.probe-failure — probe `doc-code` could not start `$RIG/probes/nonexec`: Permission denied (os error 13)
  at: doc-code
  route: the probe subprocess misbehaved (the message carries the reason) — a shipped probe runs inside `jigc`, so reinstall `jigc` (or repair the probe binary an override names), then re-run the sweep
pack-probe-integrity finding(s) present — the sweep could not complete and exits non-zero; the store result is not trustworthy.
  --format json -> exit 1 ; report_only=false blocking_probes=["pack-probe-integrity"] ; KEY (pack-probe-integrity.probe-failure, doc-code) blocking
$ … exit3    -> exit 1  "probe `doc-code` exited non-zero (exit-code 3) with no usable output"   (same key, same trailer, T+J)
$ … garbage  -> exit 1  "probe `doc-code` exited 0 but emitted unparseable output: expected value at line 1 column 1"  (same key, same trailer, T+J)
$ JIGC_DOC_CODE_PROBE=$RIG/probes/absent jigc validate   -> exit 1
`doc-code` probe not found at "$RIG/probes/absent" — `JIGC_DOC_CODE_PROBE` names no file; point it at a probe executable, or unset it to run the probe built into `jigc`
  --format json -> exit 1 ; stdout empty ; stderr {"error": "`doc-code` probe not found at …"}
# hook door
$ echo y >> README.md; git add README.md; JIGC_DOC_CODE_PROBE=$RIG/probes/nonexec git commit -q -m unrelated   -> exit 0, silent, commit landed
```

### P5 · `unmigrated-corpus` and `ahead-corpus` (rig `committed-singletons`)

```
$ sed -i '' 's/^schema-version: 2$/schema-version: 1/' CHANGELOG.md ; git commit -q -am "stale stamp"    -> exit 0 (hook silent)
$ jigc validate                      -> exit 1
advisory · file-state.hash-matches — on-disk content of `CHANGELOG.md` differs from the recorded state
  route: the baseline lags `HEAD`; absorbed at the next finalize
blocking · schema-conformance.schema-version-current — `CHANGELOG.md`: field `schema-version` is schema-version 1, below the current schema-version 2
  at: changelog:changelog
  route: migrate — … run `jigc migrate-corpus` to upgrade it
the committed corpus is below its schema-version — every other finding above was adjudicated against a schema those docs were never written to, so the sweep exits non-zero; run `jigc migrate-corpus`, then re-validate.
  --format json -> exit 1 ; report_only=false ; KEY (schema-conformance.schema-version-current, changelog:changelog) blocking
$ sed … 1 -> 3 ; git commit -q -am "ahead stamp"                                                          -> exit 0 (hook silent)
$ jigc validate                      -> exit 1
blocking · schema-conformance.schema-version-ahead — `CHANGELOG.md`: field `schema-version` is schema-version 3, above the current schema-version 2 — …
a committed doc is stamped above this build's schema-version — it was written to a schema this jigc build does not know, so the sweep could not adjudicate it and exits non-zero; upgrade jigc, or restore the stamp from git history, then re-validate.
  --format json -> exit 1 ; KEY (schema-conformance.schema-version-ahead, changelog:changelog) blocking
```

### P6 · `orphaned-instance`, `(7, A7-F2)`, `(7, C-2)` (rig `committed-singletons`)

```
# the baseline's own fixture: the tracked project-layer file .jigc/config/packs.yaml, written by hand
$ printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml ; git add -A ; git commit -q -m "drop the methodology pack"
$ jigc validate                      -> exit 1
blocking · schema-conformance.orphaned-instance — committed doc `docs/decisions-log.md` sits at a jigc-managed home and carries a `schema-version:` stamp, but no resolved doctype claims this path — …
  route: ask `jigc ingest`, which re-reads the file: … or take it out of jigc's world: `jigc unmanage docs/decisions-log.md`, then delete the file or its `schema-version:` stamp (`unmanage` drops the baseline and leaves the bytes, so the stamp alone keeps this finding alive)
blocking · schema-conformance.orphaned-instance — committed doc `docs/roadmap.md` … (same)
a stamped committed doc is claimed by no resolved doctype — no resolved doctype claims those paths, so the sweep never read them against a schema and exits non-zero; ask `jigc ingest` which of them a resolved schema still accepts, …
  --format json -> exit 1 ; KEY (schema-conformance.orphaned-instance, docs/decisions-log.md) · (…, docs/roadmap.md)

# (7, A7-F2) — the same commit
$ jigc ingest                        -> exit 0
adoptable CHANGELOG.md → changelog  (adopted — indexed + baselined, no file moved)
unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)
  unmanaged — matches no managed schema; staying a plain file is a legitimate end-state — no action needed. …
$ jigc ingest --format json          -> exit 0
  ROW docs/decisions-log.md unmanaged best_match=null finding=null adopted=false annotations=[]
  ROW docs/roadmap.md       unmanaged best_match=null finding=null adopted=false annotations=[]   findings=[]
$ jigc doc list                      -> exit 0
changelog:changelog  CHANGELOG.md  managed
(none)  docs/decisions-log.md  orphaned
(none)  docs/roadmap.md  orphaned

# (7, C-2) — VISION.md still carries its stamp (command grep -c "schema-version" VISION.md -> 1)
  jigc validate -> mentions of VISION.md: 0 · jigc doc list -> 0 · jigc ingest -> 0 (its row is `unmanaged`, folded into the root count) · jigc migrate-corpus --dry-run -> 0

# the orphan route's unmanage step, verbatim
$ jigc unmanage docs/roadmap.md      -> exit 0
unmanaged docs/roadmap.md — dropped its file-state baseline; the file is left on disk       (ls: present)
$ jigc validate --format json        -> exit 1 (both orphan rows still present — as the route itself says)
```

### P7 · `(7, A7-F1)`, `home-vacated`, and `ingest` over a missing tracked file (rig `committed-singletons`)

```
$ rm CHANGELOG.md                                    # uncommitted, unstaged
$ jigc validate                      -> exit 1
blocking (gates at finalize) · reconciliation.rename — tracked managed doc changelog:changelog (CHANGELOG.md) is missing
  route: restore CHANGELOG.md, or confirm the deletion by dropping it from the index: `jigc unmanage CHANGELOG.md`
blocking · schema-conformance.home-vacated — `changelog` homes its one document at `CHANGELOG.md`, `HEAD` still carries the `schema-version:`-stamped document … and the worktree no longer holds it …
  route: the removal is not committed — restore it: `git -C $REPO checkout -- CHANGELOG.md`; …
out-of-band rename detected — a structural-identity change this commit introduced; the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`).
  HEAD unchanged · git status --short -> " D CHANGELOG.md" · git log -1 --name-status --diff-filter=R -> (empty)
  --format json -> exit 1 ; KEY (reconciliation.rename, CHANGELOG.md) blocking · KEY (schema-conformance.home-vacated, CHANGELOG.md) blocking
$ jigc doc list                      -> exit 0 (three rows; no changelog row)
$ jigc ingest                        -> exit 1
could not read the candidate at "$REPO/CHANGELOG.md": No such file or directory (os error 2)
$ git -C $REPO checkout -- CHANGELOG.md ; jigc validate   -> exit 0 (the two rows are gone)

$ git rm -q CHANGELOG.md ; git commit -q -m "remove the changelog"     -> exit 0 (hook silent)
$ jigc validate                      -> exit 1   (same rename row; home-vacated names the removing commit; the SAME oob-rename trailer)
$ jigc unmanage CHANGELOG.md         -> exit 0  "… there was no file at CHANGELOG.md to leave on disk"
$ jigc validate                      -> exit 1
blocking · schema-conformance.home-vacated — … nothing is there now — a declared home jigc committed into has been vacated
a declared home the repository committed into is empty — the repository's history says that home held a document and nothing is there now, so the sweep exits non-zero rather than report a green over a home jigc's own document is missing from; …
  --format json -> exit 1 ; blocking_probes=["schema-conformance"] ; KEY (schema-conformance.home-vacated, CHANGELOG.md)
```

### P8 · `foreign-squatter`, and the `ingest` control (rig `fresh`)

```
$ printf '# Changelog\n\nsome history, hand written\n' > CHANGELOG.md ; git add CHANGELOG.md ; git commit -q -m "a hand-written changelog"
$ jigc validate                      -> exit 1
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — …
  at: CHANGELOG.md
  route: adopt — run `jigc ingest` to route it, or `jigc migrate $REPO/CHANGELOG.md --as changelog` …
a never-adopted file sits at a managed home — jigc was never handed it, so the sweep exits non-zero rather than report a green over a document it has never seen; …
  --format json -> exit 1 ; report_only=false blocking_probes=[] ; KEY (schema-conformance.unadopted-instance, CHANGELOG.md) advisory
$ jigc doc list                      -> exit 0   changelog:changelog  CHANGELOG.md  unregistered
$ jigc ingest                        -> exit 0   needs-reconcile CHANGELOG.md → changelog … route: `jigc migrate $REPO/CHANGELOG.md --as changelog` …

# control for (R3, F2): an UNMANAGED tracked markdown file missing from the worktree
$ rm README.md
$ jigc ingest                        -> exit 1
could not read the candidate at "$REPO/README.md": No such file or directory (os error 2)
$ jigc ingest --format json          -> exit 1 ; stdout empty ; stderr {"error": "could not read the candidate at \"$REPO/README.md\": No such file or directory (os error 2)"}
```

### P9 · L1 at store scope (rig `committed-singletons`; one rig per cell)

```
# the edit: conformant = one line appended under VISION.md's thesis; non-conformant = the `## Thesis` heading line removed;
#           stale = `schema-version: 1` -> `0`
# pull:  teammate commits + pushes, `git pull -q --ff-only`            -> exit 0
# merge: side branch commit, a commit on main, `git merge -q --no-ff`  -> exit 0
# hand:  edit in place, `git commit -q -am "docs: hand edit"`          -> exit 0 (hook silent)
# in every cell: on-disk VISION.md == HEAD blob (git hash-object vs rev-parse HEAD:VISION.md)

conformant × {pull, merge, hand}:
$ jigc validate                      -> exit 0
advisory · file-state.hash-matches — on-disk content of `VISION.md` differs from the recorded state
  at: VISION.md
  route: the baseline lags `HEAD`; absorbed at the next finalize
… report-only at store scope (exit 0); each gates nowhere — a store-scope advisory, actionable through its own route above.
$ jigc validate --format json        -> exit 0 ; report_only=true blocking_probes=[] ; KEY (file-state.hash-matches, VISION.md) advisory

non-conformant × {pull, merge, hand}:
$ jigc validate                      -> exit 0
blocking (gates at finalize) · file-state.hash-matches — on-disk content of `VISION.md` differs from the recorded state
  route: review the out-of-band edit to `VISION.md` and re-author it through the owning workflow
blocking (gates at finalize) · conformance.section-renamed — `VISION.md`: section heading "Invariants" does not match required section `thesis`
blocking (gates at finalize) · conformance.section-renamed — … "Open Questions" does not match required section `invariants`
blocking (gates at finalize) · conformance.section-missing — `VISION.md`: required section heading `## open-questions` is missing
5 finding(s) — report-only at store scope (exit 0); 4 of them gate at …
$ jigc validate --format json        -> exit 0 ; report_only=true blocking_probes=["conformance","file-state"]

the lag route's promise (hand, conformant): mint an unrelated single-task, stage note.txt, finalize
$ jigc task finalize unrelated-work  -> exit 0
advisory · reconciliation.absorb — external edit absorbed: `VISION.md`
finalized 515bb35 — docs(misc): unrelated work
$ jigc validate --format json        -> exit 0 ; the hash-matches row is gone

stale (hand): jigc validate -> exit 1 (advisory hash-matches lag row + blocking schema-version-current, unmigrated-corpus trailer);
after the same unrelated finalize (exit 0, reconciliation.absorb at VISION.md): validate -> exit 1 with schema-version-current alone
```

(The `schema-conformance.repeatable-populated` advisory at `decisions-log:decisions-log#entries` is a
standing row of this rig — see P1's control — and is elided from the blocks of P9–P12.)

### P10 · L1 at the task gate, the change predates the task (rig `committed-singletons` + team)

```
teammate appends "A teammate's pulled line." to VISION.md's thesis, commits, pushes;  $ git pull -q --ff-only -> exit 0
  [before] the line in VISION.md -> 1 ; in HEAD:VISION.md -> 1
$ jigc start --workflow single-task "sharpen the open questions"        (minted AFTER the pull)
$ jigc doc set-slot vision:vision#open-questions --from-file - --task sharpen-the-open-questions   <<< "Which domains earn a pack, and when."
$ jigc task validate sharpen-the-open-questions                 -> exit 0
advisory · file-state.staged-copy — staged copy of `VISION.md` — this task's in-flight version of the doc
advisory · reconciliation.absorb — external edit absorbed: `VISION.md`
  at: VISION.md
  route: no action needed — the external edit was absorbed into the baseline
  --format json -> exit 0 ; KEY (reconciliation.absorb, VISION.md) advisory
$ jigc task finalize sharpen-the-open-questions --dry-run       -> exit 0   "would commit — docs(vision): … promoted VISION.md" + the same rows (T+J)
$ jigc task finalize sharpen-the-open-questions                 -> exit 0   "finalized 9002e0b … promoted VISION.md"
  [after] "A teammate's pulled line." in HEAD:VISION.md -> 1 ; "Which domains earn a pack, and when." in HEAD:VISION.md -> 1 ; both in the worktree -> 1 / 1
$ jigc validate --format json                                   -> exit 0 (no VISION.md row)
```

### P11 · L1 at the task gate, the change lands during the task (three orders, one rig each)

```
stage-then-pull: mint → set-slot → teammate edit pulled
  staged copy .jigc/tasks/<id>/docs/vision:vision.md carries the teammate's line: 0
$ jigc task validate <id> --format json          -> exit 3
  KEY (reconciliation.conflict-block, VISION.md) blocking
     route: `jigc task discard <id> --force` to drop this task's staged writes (discard retires the whole task — no per-doc discard exists), or revert the external edit on disk to keep them — the damage was made out-of-band, so it is repaired where it happened
$ jigc task finalize <id> --dry-run              -> exit 3
blocking · finalize.base-mismatch — the task was started at base `<sha>` but HEAD is now `<sha>`, and the moved history overlaps the task's work on `VISION.md`
  at: task:<id>
  route: resolve the overlap on `VISION.md` against the new history, or discard the task with `jigc task discard <id> --force`
$ jigc task finalize <id>   (T and J)            -> exit 3  KEY (finalize.base-mismatch, task:<id>)
  HEAD unchanged · teammate's line in VISION.md -> 1, in HEAD:VISION.md -> 1 · the task's prose in HEAD -> 0 · staged copy still present
$ jigc start --task <id>                         -> exit 1 (output not captured)
$ jigc task validate / task finalize again       -> exit 3 / 3, unchanged

pull-then-stage: mint → teammate edit pulled → set-slot
$ jigc doc set-slot vision:vision#open-questions --from-file - --task <id>   -> exit 0
set slot vision:vision#open-questions (36 chars) (copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)
  staged copy carries the teammate's line: 1
$ jigc task validate <id> --format json -> exit 3 conflict-block · jigc task finalize (--dry-run, landing, json) -> exit 3 base-mismatch · HEAD unchanged

hand: mint → set-slot → an uncommitted hand line appended to VISION.md
$ jigc task validate <id> --format json -> exit 3 conflict-block
$ jigc task finalize <id> --dry-run / landing / json -> exit 3
blocking · reconciliation.conflict-block — conflict on `VISION.md`: an external edit and this task's staged writes both changed it
  HEAD unchanged · the hand line still in the worktree -> 1
$ jigc start --task <id> -> exit 0 ; task validate -> exit 3 ; task finalize -> exit 3
```

### P12 · the milestone doors (rig `committed-singletons` + team)

```
pull BEFORE create:
$ jigc milestone create "Vision pass" ; jigc milestone add-task vision-pass "Sharpen the open questions" ; set-slot vision:vision#open-questions on the sub-task
$ jigc milestone finalize vision-pass --format json   -> exit 0 ; top-level key `committed` only ; commits[0].paths = [VISION.md, docs/milestone-records/vision-pass.md]
  [after] teammate's line in HEAD:VISION.md -> 1 ; the sub-task's prose -> 1

pull AFTER create:
$ jigc milestone finalize vision-pass --format json   -> exit 3
  KEY (reconciliation.conflict-block, VISION.md) blocking
     route: `jigc task list` names the live tasks — discard the sub-task that staged this doc, or revert the external edit on disk, then re-run the join
  HEAD unchanged · teammate's line in HEAD:VISION.md -> 1
$ jigc validate --format json                         -> exit 0 ; KEY (file-state.hash-matches, VISION.md) advisory, the lag route

the record door: teammate edits docs/milestone-records/vision-pass.md (`status: active` -> `status: joined`), pulled
$ jigc milestone add-task vision-pass "Evict cold entries"   -> exit 1
blocking · reconciliation.conflict-block — conflict on `docs/milestone-records/vision-pass.md`: the milestone record is machine-maintained and was edited out of band since jigc last wrote it
  route: restore `docs/milestone-records/vision-pass.md` to what jigc last wrote (`git -C $REPO checkout -- …` for an uncommitted edit, else revert the commit that changed it) and re-run this command — an external edit to a machine-maintained record is never merged and never clobbered
  --format json -> exit 1 ; stdout empty ; stderr {"error": "blocking · reconciliation.conflict-block — …"}
  HEAD unchanged · record byte-identical to the pulled copy
$ jigc validate                                       -> exit 0
advisory · file-state.hash-matches — on-disk content of `docs/milestone-records/vision-pass.md` differs from the recorded state
  route: the baseline lags `HEAD`; absorbed at the next finalize

(R3, F4) — the follow-on:
$ jigc task finalize unrelated-work --format json     -> exit 0
  KEY (reconciliation.absorb, docs/milestone-records/vision-pass.md) advisory — "no action needed — the external edit was absorbed into the baseline"
$ jigc validate --format json                         -> exit 0 (no record row)
$ jigc milestone add-task vision-pass "Evict cold entries"   -> exit 1
blocking · milestone.terminal — milestone `vision-pass` is `joined` — a settled milestone is over and has no workbench
  route: read the settled record with `jigc doc show milestone-record:vision-pass`; new work starts a new milestone (`jigc milestone create "<title>"`)
```

### P13 · `(R3, F3)` — a pinned non-conformant edit, and what the refused write leaves (rig `committed-singletons` + team)

```
teammate removes the `## Thesis` heading line from VISION.md, commits, pushes;  $ git pull -q --ff-only
$ jigc start --workflow single-task "sharpen the open questions"
  before the write: ls .jigc/tasks/<id>/docs -> commit:<id>.md provenance.json
$ jigc task validate <id> --format json               -> exit 3
  KEY (reconciliation.conformance-block, VISION.md) blocking
     route: fix the file to restore conformance, or revert the edit — this is the one case a managed file is yours to hand-edit: …
$ jigc doc set-slot vision:vision#open-questions --from-file - --task <id>   -> exit 1
blocking · write.non-reparseable — write rejected: the source does not conform to the schema (section heading "Invariants" does not match required section `thesis`)
  at: vision:vision#open-questions · line 10
  route: nothing was persisted — revise the payload so the result still conforms (the message names the break), then re-run the same write; if the staged source itself is what no longer parses, `jigc task discard <task-id> --force` and start the task over
  after the write:  ls .jigc/tasks/<id>/docs -> commit:<id>.md provenance.json vision:vision.md
  staged copy == on-disk VISION.md (cmp): yes
$ jigc task validate <id> --format json               -> exit 3
  KEY (file-state.staged-copy, VISION.md) advisory
  KEY (conformance.section-renamed, vision:vision#thesis) blocking · (…, vision:vision#invariants) · (conformance.section-missing, vision:vision#open-questions)
  KEY (reconciliation.conflict-block, VISION.md) blocking      <- was conformance-block before the refused write
$ jigc doc list --task <id>                           -> exit 0   vision:vision  VISION.md  managed     <- now in the task's staged index
$ jigc task finalize <id> --dry-run --format json / landing --format json -> exit 3 / 3, HEAD unchanged
```

### P14 · the conflict route's second exit, pulled-during-task case

```
mint → set-slot → teammate edit pulled (as P11). PIN = the sha in .jigc/tasks/<id>/base.json (read, not written)
(a) $ git checkout <PIN> -- VISION.md
    $ jigc task validate <id> --format json   -> exit 0 (staged-copy advisory only)
    $ jigc task finalize <id> --format json   -> exit 3 KEY (finalize.base-mismatch, task:<id>)
(b) $ git checkout HEAD -- VISION.md ; git revert --no-edit HEAD          -> exit 0
    $ jigc task validate <id> --format json   -> exit 0
    $ jigc task finalize <id>                 -> exit 0 "finalized d347327 … promoted VISION.md"
      the task's prose in HEAD:VISION.md -> 1 ; the teammate's line -> 0 (removed by the explicit revert commit)
```

### P15 · L2, seven arms (rig `fresh`; ADR `docs/decisions/single-node-cache.md`)

```
local:  git switch -c milestone/x/main ; land_adr ; git switch main
remote: as local, then push the branch to $RIG/origin.git, fetch, `git branch -D milestone/x/main`
        refs carrying the path: refs/remotes/origin/HEAD refs/remotes/origin/milestone/x/main
tag:    as local, then `git tag v-x`, `git branch -D milestone/x/main`      refs carrying the path: refs/tags/v-x
reset:  land_adr on main ; git reset -q --hard HEAD~1
rebase: land_adr ; one later plain commit ; git rebase -q --onto HEAD~2 HEAD~1
amend:  land_adr ; git rm the ADR ; git add keep.txt ; git commit -q --amend --no-edit
ingest: an uncommitted conformant ADR written at the path ; jigc ingest ("adopted — indexed + baselined") ; rm the file
in every arm: file present: no ; history at HEAD: no ; command grep -c 'single-node-cache' .jigc/state/file-state.json -> 1

local, remote:
$ jigc validate                      -> exit 0
advisory · reconciliation.rename — tracked managed doc adr:single-node-cache (docs/decisions/single-node-cache.md) is missing, but the path has no history — the checkout moved underneath the file-state cache, not a deletion
  at: docs/decisions/single-node-cache.md
  route: nothing on this checkout needs to change — a branch switch left this baseline behind, and docs/decisions/single-node-cache.md lives on a branch this checkout does not carry: switch back to that branch to work on it again
1 finding(s) — report-only at store scope (exit 0); each gates nowhere — …
$ jigc validate --format json        -> exit 0 ; report_only=true blocking_probes=[] ; KEY (reconciliation.rename, docs/decisions/single-node-cache.md) advisory

tag, reset, rebase, amend, ingest:
$ jigc validate                      -> exit 0
advisory · reconciliation.rename — tracked managed doc adr:single-node-cache (docs/decisions/single-node-cache.md) is missing, has no history at HEAD, and no branch carries it — the baseline outlived the doc
  route: no branch, local or remote-tracking, can bring docs/decisions/single-node-cache.md back — a hard reset or a rebase past its creating commit, or deleting it before it was ever committed, leaves this baseline behind: drop it with `jigc unmanage docs/decisions/single-node-cache.md`
$ jigc validate --format json        -> exit 0 ; same key, advisory

every arm, the task gate (a single-task minted on this checkout, staging only note.txt):
$ jigc task validate unrelated-work --format json     -> exit 0 ; the same row
  PARITY store-vs-task: key + severity + route + message equal -> True (1 row / 1 row), all seven arms
$ jigc task finalize unrelated-work --dry-run         -> exit 0  "would commit — docs(misc): unrelated work" + the same advisory
$ jigc doc list                                       -> exit 0  (the ADR is not listed)
$ echo y >> README.md ; git add README.md ; git commit -q -m "unrelated plain commit"   -> exit 0, hook silent

the emitted route, verbatim:
local:  $ git switch -q milestone/x/main  -> jigc validate --format json: no row ; jigc task validate: no row
remote: $ git switch -q milestone/x/main  -> file present: yes ; jigc validate --format json: no row
others: $ jigc unmanage docs/decisions/single-node-cache.md     -> exit 0
        unmanaged docs/decisions/single-node-cache.md (adr:single-node-cache) — dropped its file-state baseline + forward edges; there was no file at docs/decisions/single-node-cache.md to leave on disk
          command grep -c 'single-node-cache' .jigc/state/file-state.json -> 0
        $ jigc validate --format json -> exit 0, no row ; jigc task validate --format json -> exit 0, no row
        $ jigc unmanage <path>        -> exit 0  "no-op: … is not managed (nothing to drop)"
        $ jigc unmanage <path> --format json -> {"path": "…", "identity": "adr:single-node-cache", "dropped": false}
tag, after the unmanage: git cat-file -e v-x:<path> -> still there ; git checkout -q v-x -> file present ;
        jigc validate --format json -> exit 0 ; KEY (file-state.un-baselined, <path>) advisory — no bytes were dropped

amend-control (the first attempt at the amend arm, where git refused the empty amend and left a STAGED deletion with history):
$ jigc validate                      -> exit 1  blocking reconciliation.rename, route `restore …, or … jigc unmanage …`, the oob-rename trailer
$ jigc task validate unrelated-work --format json  -> exit 3 ; the same key blocking (+ finalize.carried-staged for the staged deletion) ; PARITY True
$ jigc task finalize unrelated-work --dry-run      -> exit 3
```

### P16 · L2 over placement singletons (rig `committed-singletons`)

```
$ git switch -q -c work/old <the install commit>          # worktree: CLAUDE.md README.md
$ jigc validate                      -> exit 0   four advisory reconciliation.rename rows (CHANGELOG.md · VISION.md · docs/decisions-log.md · docs/roadmap.md), each with the switch-back route; no home-vacated row
$ jigc validate --format json        -> exit 0 ; report_only=true ; four keys, advisory
$ jigc doc list                      -> exit 0  "no committed docs"
$ jigc ingest                        -> exit 0  "3 candidate(s) classified"
$ jigc task validate unrelated-work --format json   -> exit 0 ; PARITY store-vs-task equal (4 / 4)
$ jigc task finalize unrelated-work --format json   -> exit 0 ; committed.hash 7614e2a ; the four advisories on the envelope
  command grep -c 'VISION.md' .jigc/state/file-state.json -> 1  (the baselines survive the landed finalize)
$ jigc milestone create "Old pass" ; jigc milestone add-task old-pass "Touch nothing managed"
$ jigc milestone finalize old-pass --format json    -> exit 3 KEY (milestone.zero-contribution, milestone:old-pass)   <- inconclusive for L2
$ git switch -q main ; jigc validate --format json  -> exit 0 ; the four rows are gone (the milestone record minted on work/old now dangles, advisory, switch-back route)
```

### P17 · L2 at `milestone finalize`; the adopt route over a committed move (rig `fresh`)

```
git switch -c milestone/x/main ; land_adr ; git switch main
$ jigc milestone create "Main pass" ; jigc milestone add-task main-pass "Record another decision" ; doc create adr --title "Other cache" --task <sub> + three slots
$ jigc milestone finalize main-pass --format json   -> exit 0 ; top-level key `committed` only ; paths [docs/decisions/other-cache.md, docs/milestone-records/main-pass.md]
  command grep -c 'single-node-cache' .jigc/state/file-state.json -> 1
$ jigc validate --format json                       -> exit 0 ; KEY (reconciliation.rename, docs/decisions/single-node-cache.md) advisory, switch-back route

$ git switch -q milestone/x/main ; git mv <adr> docs/decisions/one-node-cache.md ; git commit -q --no-verify -m "move the adr"
$ jigc validate --format json                       -> exit 1 ; KEY (reconciliation.rename, docs/decisions/single-node-cache.md) blocking, the adopt-or-revert route
$ jigc rename adr:single-node-cache --to "One node cache"   -> exit 1  store.not-found
```

### P18 · what does clear a committed out-of-band move, short of reverting it (rig `fresh`)

```
land_adr ; git mv <adr> docs/decisions/one-node-cache.md ; git commit -q --no-verify -m "move the adr"
$ jigc rename adr:single-node-cache --to "One node cache"   -> exit 1 store.not-found
$ jigc rename adr:one-node-cache --to "One node cache"      -> exit 0  "renamed adr:one-node-cache -> adr:one-node-cache (… -> …), repointed 0 referrer(s)"  (a commit lands)
$ jigc validate --format json        -> exit 1 ; KEY (reconciliation.rename, docs/decisions/single-node-cache.md) blocking — now the WEAK arm:
     route: restore docs/decisions/single-node-cache.md, or confirm the deletion by dropping it from the index: `jigc unmanage docs/decisions/single-node-cache.md`
$ jigc unmanage docs/decisions/single-node-cache.md ; jigc validate --format json -> exit 0, clean
```

### P19 · `jigc unmanage` (rig `fresh`; ADR A `alpha-cache`, ADR B `beta-cache` with `supersedes: adr:alpha-cache`)

```
(the edge index is lazy: after the two finalizes .jigc/index/edges.json is absent; `jigc unmanage B` then `jigc ingest` materializes it)
before:  command grep -c 'beta-cache' .jigc/state/file-state.json -> 1 ; … .jigc/index/edges.json -> 1  ("from": "adr:beta-cache")
$ jigc unmanage docs/decisions/beta-cache.md        -> exit 0
unmanaged docs/decisions/beta-cache.md (adr:beta-cache) — dropped its file-state baseline + forward edges; the file is left on disk. It still sits at the managed home, so the op that relocates that home — … — still carries it, managed or not; move it out of the managed location to fully detach it
after:   file-state.json -> 0 ; edges.json -> 0 ; ls docs/decisions/beta-cache.md -> present
$ jigc unmanage docs/decisions/beta-cache.md        -> exit 0  "no-op: … is not managed (nothing to drop)"
$ jigc unmanage docs/decisions/beta-cache.md --format json -> {"path": …, "identity": "adr:beta-cache", "dropped": false}
$ jigc validate   -> exit 0  advisory · file-state.un-baselined — committed doc `docs/decisions/beta-cache.md` is not yet baselined …
$ jigc doc list   -> exit 0  both ADRs `managed`
$ jigc ingest     -> "adoptable docs/decisions/beta-cache.md → adr (adopted — indexed + baselined, no file moved)" ; file-state 0 -> 1, edges 0 -> 1
$ jigc unmanage docs/decisions/alpha-cache.md --format json -> exit 0 {"path": …, "identity": "adr:alpha-cache", "dropped": true} ; file present ; baseline 1 -> 0

never managed:
$ jigc unmanage README.md (T, J)                    -> exit 0 no-op ; {"identity": null, "dropped": false}
$ jigc unmanage docs/decisions/never-was.md (T, J)  -> exit 0 no-op ; {"identity": "adr:never-was", "dropped": false}
$ jigc unmanage docs/decisions                      -> exit 0 no-op
$ jigc unmanage ../outside.md                       -> exit 0 no-op

symlinks:  ln -s docs/decisions/alpha-cache.md link.md ; ln -s docs/decisions dlink     (baseline before: 1)
$ jigc unmanage link.md                             -> exit 0 no-op ; baseline 1
$ jigc unmanage dlink/alpha-cache.md (T, J)         -> exit 0 no-op ; {"identity": null, "dropped": false} ; baseline 1 ; file present

(R3, F5) — other spellings of the managed path docs/decisions/alpha-cache.md (baseline 1 before each):
$ jigc unmanage ./docs/decisions/alpha-cache.md             -> exit 0  "no-op: ./docs/decisions/alpha-cache.md is not managed (nothing to drop)" ; baseline after: 1
$ jigc unmanage docs//decisions/alpha-cache.md              -> exit 0  no-op ; baseline after: 1
$ jigc unmanage docs/decisions/../decisions/alpha-cache.md  -> exit 0  no-op ; baseline after: 1
$ jigc unmanage $REPO/docs/decisions/alpha-cache.md         -> exit 0  no-op ; baseline after: 1
$ (cd docs && jigc unmanage decisions/alpha-cache.md)       -> exit 0  no-op ; baseline after: 1
$ (cd docs && jigc unmanage docs/decisions/alpha-cache.md)  -> exit 0  "unmanaged … (adr:alpha-cache) — dropped …" ; baseline after: 0
```

### P20 · F21, its control, its bounds, and `(R3, F6)` (rig `fresh`; source step files under `$RIG/src`, outside `.jigc/`)

```
$ printf '{{ include: step:no-such-step }}\n' > $RIG/src/broken.yaml
$ jigc config insert-step --workflow amend --after amend-message $RIG/src/broken.yaml   -> exit 0
config: inserted step `broken` into `amend` after `amend-message` — written to `.jigc/config/`, uncommitted — commit it with your next commit
$ git add -- .jigc/config ; git commit -q -m "chore: a project structural delta"        -> exit 0
$ jigc validate                      -> exit 0  "no findings — the committed store validates clean"
$ jigc validate --format json        -> exit 0  findings=[] report_only=true blocking_probes=[]
$ jigc task amend "repair the message"  -> exit 1
blocking · workflow-refs.include-resolves — include `step:no-such-step` resolves to no step file in the cascade
  route: fix the workflow/step/catalog definition the message names (a definition defect, repaired once at its source), then re-run
$ jigc upgrade                       -> exit 0  "no findings — 1 recorded config delta(s) re-apply clean against the current pack, …"

control — the scope's granted exception, a hand-written whole-file shadow at .jigc/config/workflows/amend.yaml (the shipped amend.yaml + the same include), committed:
$ jigc validate --format json        -> exit 0 ; blocking_probes=["workflow-refs"] ; KEY (workflow-refs.include-resolves, workflow:amend) blocking
$ jigc task amend "repair the message"  -> exit 1  the same finding

bounds:
cycle  ($RIG/src/loop.yaml = `{{ include: step:loop }}`):   insert-step -> 0 ; validate (T, J) -> 0, findings=[] ;
        task amend -> 1 "blocking · workflow-refs.include-cycle-absent — include cycle: loop -> loop"
badref ($RIG/src/badref.yaml = `Run: {{ cli.no-such-command }}`): insert-step -> 0 ; validate (T, J) -> 0, findings=[] ;
        task amend -> 1 "blocking · workflow-refs.run-marker-not-shadowed — step prose shadows the composer-reserved `Run: ` marker: … at: step:badref"

(R3, F6) — what the refused compose leaves:
  before: ls .jigc/tasks -> (empty) ; jigc task list -> "no active tasks"
$ jigc task amend "repair the message"            -> exit 1 (as above)
  after:  ls .jigc/tasks -> repair-the-message ; jigc task list -> "1 active task(s)   repair-the-message  [amend]  repair the message"
          area: amend base.json docs intent staged-snapshot.json workflow
$ jigc task amend "repair the message"            -> exit 1   (the route's "then re-run")
  {"error": "blocking · task.serial-collision — task `repair-the-message` is already active\n  at: task:repair-the-message\n  route: resume with `jigc start --task repair-the-message` or abandon with `jigc task discard repair-the-message --force`"}
$ jigc start --task repair-the-message            -> exit 1  workflow-refs.include-resolves (the same break)
$ jigc task discard repair-the-message --force    -> exit 0  "discarded task repair-the-message — dropped staged edits to: commit:repair-the-message (transient)"
second door, over `jigc config insert-step --workflow single-task --after locate $RIG/src/broken2.yaml` (committed):
$ jigc start --workflow single-task "probe the delta"   -> exit 1 workflow-refs.include-resolves ; ls .jigc/tasks -> probe-the-delta ; task list -> 1 active
$ jigc workflow single-task --preview                   -> exit 1 the same finding ; nothing minted
the same leftover with the whole-file shadows of `amend` and `single-task` (so it is not specific to the delta path):
  task amend -> exit 1, tasks = [repair-the-message] ; start --workflow single-task -> exit 1, tasks = [probe-the-shadow repair-the-message]
```

---

## 4 · Defects

Tier predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary
does not do.

### `(R3, F1)` — the strong-signal route's *adopt* exit cannot be run: `jigc rename <old-id>` answers `store.not-found`

- **Door / cell:** `validate` (and through it the hook and the task gates) → the route run at `rename`;
  cell 5, strong arm. Rows 105. Repro P2, P3, P17, P18.
- **Observed:** exit 1, `store.not-found — no managed doc \`<old-id>\` to rename (expected at <old path>)`,
  in all three states driven (placement singleton staged · location doc staged · location doc committed).
  The out-of-band move is the very thing that makes the old id unresolvable at the door the route names.
- **Contract contradicted:** the finding's own route (*adopt it as a CLI-owned rename …: `jigc rename
  <id> --to "<New Title>"`; or revert the move*), `design/reconciliation.md` → *Rename detection* (the
  resolution block, *routing to the owned op first, revert second*), and the trailer (*revert the `git mv`
  or adopt it via `jigc rename`*).
- **Proposed tier: 3** — one of the route's two printed exits names a command that refuses; the other
  (the revert) works verbatim and clears the row, so the finding is not a dead end. What does work
  short of reverting is not what the route says: `jigc unmanage <old path>` (P18).

### `(R3, F2)` — `jigc ingest` fails with a code-less, route-less OS error when any tracked `.md` is missing from the worktree

- **Door / cell:** `ingest` × the weak-arm / `home-vacated` worktree-removal state, and the unmanaged
  control. Row 25. Repro P7, P8.
- **Observed:** exit 1, `could not read the candidate at "$REPO/<path>": No such file or directory (os
  error 2)`; JSON arm `{"error": …}`. The whole triage is lost, not the one row. It is not specific to
  managed docs: `rm README.md` reproduces it.
- **Contract contradicted:** `jigc ingest --help` (*Misplaced or non-conformant files are flagged for a
  human; exits 0*); the route floor's spirit (no code, no route).
- **Proposed tier: 3** — the help text states an exit the binary does not deliver in this posture;
  nothing is written and the message names the path, so the operator can recover.

### `(R3, F3)` — `write.non-reparseable` says *nothing was persisted* while the copy-in is persisted and changes the task's reconcile state

- **Door / cell:** `doc set-slot` × a pinned edit that fails conformance. Rows 50–52. Repro P13.
- **Observed:** the refused write (exit 1) leaves `.jigc/tasks/<id>/docs/vision:vision.md` (absent
  before, present after, byte-equal to the on-disk doc). The task is now `TOUCHED`: `task validate`
  moves from `reconciliation.conformance-block` to `file-state.staged-copy` +
  `reconciliation.conflict-block` (*an external edit and this task's staged writes both changed it* —
  the task has written nothing), and `jigc doc list --task <id>` lists `vision:vision` as staged.
- **Contract contradicted:** the finding's own route, which opens *nothing was persisted*.
- **Proposed tier: 3** — the surface states a no-op the binary did not perform; both the before and the
  after state block at exit 3, so nothing lands and nothing is lost. **Neighbour:** this is a
  write-surface seam (row 6) reached from this row's cell 3.

### `(R3, F4)` — a pulled edit to the machine-owned milestone record is absorbed by the next unrelated finalize

- **Door / cell:** `task finalize` (unrelated task) then `milestone add-task` × the milestone-record
  door. Rows 57–58. Repro P12.
- **Observed:** the record door conflict-blocks the pulled edit (row 56, as designed). An unrelated
  `jigc task finalize` then reports `reconciliation.absorb` at the record and baselines it; the record
  door no longer sees drift and reads the hand-set `status: joined` as the milestone's truth
  (`milestone.terminal`, *a settled milestone is over and has no workbench*).
- **Contract contradicted:** `design/team-ready-state.md` → *No-silent-overwrite discipline* (*the record
  is … detected + conflict-blocked, **not** "hand-editable and merged like an authored doc" … whose OOB
  drift is caught, not absorbed*) and `design/reconciliation.md` (*every drift of the machine-owned
  record conflict-blocks there, pulled or not*). The store sweep's own route on that row (*absorbed at
  the next finalize*) is **true** — the binary is self-consistent; the design sentence is what it
  contradicts.
- **Proposed tier: 3** — a stated guarantee holds only until any finalize lands. No bytes are lost (the
  record keeps exactly the pulled content). Whether the unrelated-finalize absorb predates M55 was not
  established here; M55 added the store route that announces it.

### `(R3, F5)` — `jigc unmanage` matches its path literally and reports a managed doc as *not managed*

- **Door / cell:** `unmanage` × other spellings of a managed path. Row 127. Repro P19.
- **Observed:** `./<path>`, `docs//…`, `…/x/../…`, the absolute path and a cwd-relative path from a
  subdirectory all answer `no-op: <spelling> is not managed (nothing to drop)` at exit 0 while the
  baseline is present before and after (`command grep` 1 → 1). The JSON arm was not driven for these
  spellings.
- **Contract contradicted:** the ack sentence itself (the doc at that path *is* managed); the miss is
  exit 0 by contract, so the sentence and `dropped` are the only discriminators a caller has.
- **Proposed tier: 3** — a false statement at exit 0 with no drop and no loss; every route jigc emits
  uses the repo-relative spelling, which works. Not checked against numbered axis 1's baseline (caller
  tokens), which is outside this row.

### `(R3, F6)` — a compose-gate refusal leaves a minted live task, and the route's *then re-run* collides with it

- **Door / cell:** `task amend` and `start --workflow` × a workflow whose resolved definition fails
  `workflow-refs`. Rows 130, 134, 116. Repro P20.
- **Observed:** the refusal (exit 1) leaves `.jigc/tasks/<id>/` and a row in `jigc task list`; re-running
  the same command, as the route says, answers `task.serial-collision`. `jigc workflow <W> --preview`
  mints nothing. The leftover is the same under a whole-file shadow, so it is not F21's delta path.
- **Contract contradicted:** the finding's route (*fix the … definition …, then re-run*); and the
  refusal names no task although one now exists (`design/validation.md` states the sibling refusal
  `workflow.verb-routed` is raised *before anything mints, so a refused name strands no task dir*).
- **Proposed tier: 3** — the collision's own route (`jigc task discard <id> --force`) works, so it is
  one hop from clean; nothing is committed. **Neighbour:** outside this row's subject (rows 5 / 8);
  recorded because it was driven on F21's composing doors. Whether it predates the range was not
  established.

### Carried baseline defects (keys verbatim)

- **`(7, A7-F1)` — STILL-OPEN, tier 3 as recorded.** Row 138, P7. The `oob-rename` trailer still reads
  *out-of-band rename detected — a structural-identity change this commit introduced; … (revert the
  `git mv` or adopt it via `jigc rename`)* over an uncommitted, unstaged `rm` (HEAD unchanged, no rename
  rows in HEAD), over a committed `git rm`, and — new on this run — over a move that landed in a *prior*
  commit (P2). M55 narrowed the member to the blocking arm and left the trailer text as it was.
- **`(7, A7-F2)` — STILL-OPEN, tier 3 as recorded.** Row 139, P6. At one commit `jigc validate` exits 1
  on two `orphaned-instance` rows whose first exit is `jigc ingest`, and `jigc ingest` exits 0 filing
  both under *fine to stay plain … no action needed*, JSON rows `finding: null`.

---

## 5 · NOT DRIVEN — and why

| # | (door, cell) | reason |
|---|---|---|
| 1 | every task gate × the `live_record_finding` arm (the third advisory producer of `reconciliation.rename`) | needs a sub-task pinned before its milestone's record commit on a checkout that predates it; M52's carve-out, outside the M54/M55 range; not constructed |
| 2 | `milestone finalize` × L2 over placement singletons | driven argv exited 3 on `milestone.zero-contribution` before the gate (row 102) — inconclusive for the L2 row; the location-doctype cell (row 101) was driven instead |
| 3 | `task validate` / `task finalize` × L1 reached by **merge** or by a **hand commit** with the doc `TOUCHED` | only the pull was driven at the task gate (rows 37–39); the hand-commit way was driven `UNTOUCHED` (rows 33–34). The three ways leave identical bytes at `HEAD`, which is the fact the arm reads |
| 4 | L1 at the task gate × a location-doctype doc other than the milestone record | the touched doc was the placement singleton `VISION.md` throughout |
| 5 | `milestone add-task` × a pinned **non-conformant** record edit | not constructed |
| 6 | L2 × a git failure on the other-refs read (answers *carried*) | no way to fail that one read on the installed binary without breaking every other git call |
| 7 | L2 × a doc only the **stash** holds | not constructed; `storage.md` declares it takes the `unmanage` arm, as the tag and the reflog-only (reset) arms did |
| 8 | `task finalize` (landing) × L2 for the six location arms | `--dry-run` was driven in all seven; the landing was driven in the placement variant (row 98) |
| 9 | `ingest` / `doc list` × each L1 state | `ingest` and `doc list` were driven over the exit-flip and L2 states; neither reads the baseline, and L1 changes nothing they list |
| 10 | hook × a delete-plus-add (non-`git mv`) form of the strong arm | only the `git mv` form was staged |
| 11 | `validate` × `probe-unreliable` by **timeout** (the 30 s budget) and by a build-id mismatch | row 2 owns why a probe did not run; three failure shapes were driven here for the exit, trailer and key |
| 12 | every task gate × `probe-unreliable` | row 2's doors |
| 13 | `validate` × precedence between members other than `oob-rename` over `home-vacated` | one two-member state was observed (P7); the other pairs were not constructed |
| 14 | every door × `--format human` | not driven anywhere in this row |
| 15 | F21 × `config replace-step` and `config remove-step` deltas | only `insert-step` was driven (three bodies) |
| 16 | F21 bound × a bad **command-ref** in isolation | the driven body tripped `run-marker-not-shadowed` first (row 133); a body carrying the ref without the `Run:` marker was not driven |
| 17 | `unmanage` × a managed doc a **live task** holds a staged copy of | the cost is declared (`design/storage.md` → *What none of this buys*: a lost baseline switches the conflict guard off, at exit 0, including *after `jigc unmanage`*); not re-driven. It is the nearest thing to a tier-1 shape on this surface and it is a stated bound, not a finding of this run |

Also not a row of this record: exit code **4** (migration review hold) and exit **2** (clap) are classes
of `EXIT_CODES` outside this row's doors; exit 2 was seen once, incidentally (`jigc milestone finalize
<id> --dry-run` — the flag does not exist on that leaf).

---

## 6 · Baseline rows: CLOSED / STILL-OPEN

| key | status | datum |
|---|---|---|
| `(7, A7-F1)` | **STILL-OPEN** | `rm CHANGELOG.md` (uncommitted) → `jigc validate` exit 1 with the trailer *out-of-band rename detected — a structural-identity change this commit introduced; the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`)*; HEAD unchanged, `git log -1 --diff-filter=R` empty. Same trailer over the committed `git rm`. P7 |
| `(7, A7-F2)` (= the open half of M51 `(7, D-4)`) | **STILL-OPEN** | one commit: `jigc validate --format json` exit 1, two `(schema-conformance.orphaned-instance, docs/…)` keys; `jigc ingest` exit 0, *fine to stay plain … no action needed*, JSON rows `finding: null, annotations: []`. P6 |
| `(7, C-2)` (M51) | **STILL-OPEN (expected)** — a declared residual on a written trigger; not re-filed | departed-doctype fixture: `VISION.md` still stamped; mentions of `VISION.md` at `validate` 0 · `doc list` 0 · `ingest` 0 · `migrate-corpus --dry-run` 0. P6 |
| §D axis-7 lead — `probe-unreliable`, the member no axis-7 door had driven | **CLOSED** (the lead is discharged by a drive; no defect) | three failure shapes → `jigc validate` exit 1, `(pack-probe-integrity.probe-failure, doc-code)` blocking, the member's own trailer, `report_only: false`, on both arms. P4 |
| **F21** (`completions/artifacts/M55/VERDICT.md`) | **STILL-OPEN (expected)** — a measurement, not a new finding | `jigc config insert-step` of a broken include, committed → `jigc validate --format json` exit 0, `findings: []`; `jigc task amend` exit 1 `workflow-refs.include-resolves`; the same include in a whole-file shadow **is** reported. Bounded: a cycle and a marker-shadowing native-step body are missed the same way. P20 |

---

## 7 · Observations (driven, not defects)

- **O-1 — three diagnoses for one staged move of a placement singleton.** `reconciliation.rename`
  (strong) + `schema-conformance.orphaned-instance` at the new path + `schema-conformance.home-vacated`
  at the old one (P2). The orphan row's route offers `jigc unmanage <new path>`, *then delete the file*
  — for a file that is the renamed doc. `reconciliation.md` promises one diagnosis only against the
  `ref-resolves` family, so this is recorded and not graded.
- **O-2 — only the first matching member's trailer prints** (declared: table order is precedence);
  `oob-rename` outranks `home-vacated`, which is the mechanism under `(7, A7-F1)`.
- **O-3 — `task validate` and `task finalize` name one pulled-during-task state by two codes**
  (`reconciliation.conflict-block` vs `finalize.base-mismatch`), both exit 3. Declared: the base pin is
  outside the preview (`command-output-contract.md` → the exit-code taxonomy, third clause).
- **O-4 — the conflict route's *revert the external edit on disk* exit, read literally in the pulled
  case, yields a green `task validate` and then `finalize.base-mismatch`** (P14 a). Read as *revert the
  commit* it lands (P14 b). `finalize.base-mismatch`'s own first exit (*resolve the overlap … against the
  new history*) names no verb.
- **O-5 — a write is accepted into a task that can no longer land.** After a pull during the task, the
  first `doc set-slot` copies the pulled doc in (the staged copy carries the teammate's line) at exit 0,
  and both gates then block (P11). Declared: a change after the mint still conflict-blocks.
- **O-6 — route kind is not on the wire** (§1).
- **O-7 — `milestone finalize`'s landed envelope has one top-level key, `committed`, and no `findings`
  key**, so a non-blocking L1/L2 advisory present at the join is not on its JSON arm (P12, P17).
- **O-8 — the advisory `reconciliation.rename` has three producers, not two** (§1).
- **O-9 — `jigc rename <id> --to <its current title>` commits** (P18): `renamed adr:one-node-cache ->
  adr:one-node-cache … repointed 0 referrer(s)`, HEAD moved.
- **O-10 — a blocking `conformance.*` row carries no route at either scope** (P9, P13). Declared:
  `engine::finding::is_route_exempt` names those codes.
- **O-11 — `jigc start --task <id>` exits 1 once the moved history overlaps the task** (P11); the
  surface was not captured.
- **O-12 — a committed stale-stamp or ahead-stamp edit grades the L1 lag row advisory** (P5, P9): the
  version stamp is not the conformance gate. The route's promise was driven true for the stale stamp —
  the next finalize absorbs it and the lag row clears, leaving `schema-version-current`.
- **O-13 — `ingest` reports `adoptable CHANGELOG.md → changelog (adopted …)` for an already-managed
  changelog** after the pack composition changed (P6). Not chased.

---

## 8 · Reconciliation ledger (the reconciler's; rc.24)

**Who wrote this section.** A third agent that neither authored the driver record above nor built the
code. **Binary:** `~/.local/bin/jigc`, asserted first: `jigc --version` → `jigc 1.0.0-rc.24`. **Source pass:**
present (`codex/axis3-codex.md`, 8.3 KB, exit code 0) — a source-only read that states of itself *no binary,
tests, or reproductions were run*. **Rule applied** (`completions/artifacts/M51/acceptance-design.md` → *The
reconciliation rule*): a claim by one party that the other cannot reproduce is a lead; every Codex claim
was entered as a lead and driven, every driver defect was re-driven once.

**How it was driven.** Every rig from `dev/jigc-rig <state> --binary ~/.local/bin/jigc`, two-step eval,
stdout only, `[ -n "$REPO" ]` and a not-the-host-repository guard before any git command; every scratch
root from `mktemp -d`; no teardown. Every fixture through the binary and plain git — the two hand-written
files are the ones the scope grants (`.jigc/config/packs.yaml`, `.jigc/config/workflows/amend.yaml`). Every
exit code was read bare (output redirected to a file, `$?` captured on the next statement), never through
a pipe. Under `.jigc/` every count is `command grep` with a before-control. Reconciler repro blocks are
`R-1` … `R-13`, in §8.6.

### 8.1 · Verdict for the exit rule

**One finding shows both halves of the tier-1 predicate: `(R3, F7)`.** It is the cell the driver listed
NOT DRIVEN (§5 #17) and called *the nearest thing to a tier-1 shape on this surface … a stated bound, not a
finding of this run*. Driven, it is an exit-0 loss through a committing door, it does **not** need
`jigc unmanage` (a second clone of an adopted repository reproduces it with no other act), and the stated
bound describes a different cost (a silent *merge*) than the one observed (a *loss*). Its mechanism predates
the M54/M55 range, and whether the declaration covers it is the triage's call — both are said in §8.5.

Every other row of this record stays below tier 1; `(R3, F4)` is re-tiered **3 → 2**. The driver's line
*NO TIER-1 ROW* is true of the 138 rows it drove and is superseded by `(R3, F7)`.

### 8.2 · The driver table: demotions, and its door-set count against the registries

| check | result |
|---|---|
| rows marked driven with no repro block | **Row 137** (`config insert-step` × the same basename into a second workflow → `config.step-id-collision`): marked driven with repro P20; P20's second-door line uses a different basename (`broken2.yaml`) and carries no line for the collision. **Demoted.** Re-driven by the reconciler (R-11e): it reproduces exactly as the row says. |
| rows counted twice | **Row 102**: its own verdict column reads *counted NOT DRIVEN for the L2 cell* and it is §5 #2, yet it sits inside *Rows driven: 140*. **Demoted.** Driven count: **138**. |
| rows driven for less than they claim | **Row 43** carries an exit and no surface (it says so). **Row 21**'s code column (`conformance.section-missing`) is not in P8's text; R-11d confirms it. **Row 135**'s `workflow.verb-routed (amend)` half is not in P20's text; R-3 confirms it. **Rows 68–95** (28 rows) rest on one summarised block, P15; R-10 re-drives four arms and the control across `validate`, `task validate`, `task finalize --dry-run` and `doc list`, all equal to the summary. None demoted. |
| `STORE_EXIT_FLIPS` (`crates/cli/src/render.rs`) | **7** `StoreExitFlip` entries at the tag — the driver's 7. |
| `STORE_FAMILIES` (`crates/engine/src/validate.rs`) | **7** (`doc↔code` · `workflow↔refs` · `file↔CLI-state` · `forward-ref integrity` · `schema-completeness` · `mention integrity` · `schema-conformance`) — the driver's 7. |
| `EXIT_CODES` (`crates/cli/src/task.rs`) | **5** classes — the driver's 5. |
| `ENVELOPE_OWED_CODES` (`crates/cli/src/render.rs`) | **4** — the driver's 4. |
| dangling-baseline producers (`crates/engine/src/file_state.rs`) | three functions exist (`rename_dangling_baseline_finding` · `rename_orphaned_baseline_finding` · `live_record_finding`) — the driver's 3 against the scope's 2. Its datum stands. |
| the driver's door list (17 clap leaves + the hook) | each leaf is a `VERB_KINDS` row and is the door of ≥1 driven row (`config insert-step` survives row 137's demotion through row 128). The scope's own door set (7 leaves + the hook) is covered whole. |
| source identity | `git diff --stat jigc-v1.0.0-rc.24 HEAD -- crates` is empty: the counts above are read off the released source. |

### 8.3 · Codex claims — each entered as a lead, then driven

| # | lead(codex, …) | status | datum |
|---|---|---|---|
| C1 | **F21 remains open** — store-scope `jigc validate` reads the raw workflow definition and misses project structural-op deltas; a compose door rejects the same break; a whole-file shadow is reported | **CONFIRMED** | R-3: `config insert-step` of a broken include, committed → `jigc validate` exit 0, `findings: []`; `jigc task amend` exit 1 `workflow-refs.include-resolves`; the shadow control → `(workflow-refs.include-resolves, workflow:amend)` blocking. |
| C1a | …bound: the raw read is shared by every reader inside `workflow_refs_store` — include resolution, cycle detection, command-ref membership, schema-ref membership, marker-shadow, fan-out/join pairing, front matter | **CONFIRMED for five readers · OPEN LEAD for two** | R-3: include, cycle, **command-ref in isolation** (`workflow-refs.command-ref-resolves` at `step:cmdref` — closes the driver's §5 #16), **schema-ref** (`workflow-refs.schema-ref-resolves`), and the driver's marker-shadow (P20) are each missed at store scope and caught at compose. Fan-out/join pairing and the front-matter read were not driven. |
| C1b | …and `config replace-step` deltas are missed the same way | **CONFIRMED** | R-3 (replace arm): `config replace-step workflow:amend#amend-message` → `jigc validate` exit 0, clean; `jigc task amend` exit 1. Half-closes the driver's §5 #15 (`remove-step` not driven). |
| C1c | …`jigc validate --format json` *incorrectly exits 0* without that finding | **REFUTED as worded** | The exit is not the defect: the whole-file-shadow control **reports** the finding and exits 0 too (R-3 shadow arm: `report_only: true`, `blocking_probes: ["workflow-refs"]`). `workflow-refs` is no `STORE_EXIT_FLIPS` member, so exit 0 is the contract either way; what is missing is the row. |
| C1d | …it does not affect the other six `STORE_FAMILIES` | **OPEN LEAD** | A universal negative; nothing driven contradicts it, and nothing driven proves it. |
| C2 | **`(7, A7-F1)` is not closed** — the `oob-rename` trailer still claims a commit-introduced `git mv` over a deletion | **CONFIRMED** | R-1: `rm CHANGELOG.md` (unstaged) → exit 1, trailer *out-of-band rename detected — a structural-identity change this commit introduced; … (revert the `git mv` or adopt it via `jigc rename`)*; HEAD unchanged, status ` D CHANGELOG.md`, no rename in HEAD. Same trailer after a committed `git rm`. |
| C3 | **`(7, A7-F2)` is not closed** — `ingest` files a stamped orphan `validate` blocks on under *no action needed* | **CONFIRMED** | R-2: one commit — `jigc validate` exit 1, two `schema-conformance.orphaned-instance` rows whose route opens *ask `jigc ingest`*; `jigc ingest` exit 0, *unmanaged docs/ — 2 file(s) … fine to stay plain* + *no action needed*; JSON rows `finding: null`. |
| C4 | **`(7, C-2)` STILL-OPEN as expected** — a departed doctype's stamped root-placement instance is outside orphan territory; *no orphan finding and exit 0 absent another flip* | **CONFIRMED** (declared residual; not re-filed) | R-2: `VISION.md` still stamped, named by 0 lines at `validate` · `doc list` · `ingest` · `migrate-corpus --dry-run`. **The exit-0 half, which the driver's fixture could not show** (its two `docs/` orphans flip the exit): with those two retired, `jigc validate` → *no findings — the committed store validates clean*, exit 0, `report_only: true`, the stamped `VISION.md` tracked and unclaimed. |
| K1 | `STORE_EXIT_FLIPS` has seven members; `probe-unreliable` matches the probe-integrity meta-finding; `oob-rename` requires blocking severity; exit, JSON `report_only` and trailer share the registry | **CONFIRMED** | Count: §8.2. R-11a (probe: exit 1, `report_only: false`, the member's trailer, both arms); R-10 (the advisory `reconciliation.rename` arms exit 0 with `report_only: true`; the with-history arm exits 1); R-11d (advisory `foreign-squatter` flips the exit, `blocking_probes: []`). |
| K1a | …*no new blocking-store code omitted from the exception set and no advisory incorrectly matched* | **OPEN LEAD** | A completeness negative. Not contradicted: blocking `file-state.hash-matches` + `conformance.*` exit 0 as content findings (R-11c). Cannot be driven to a repro. |
| K2 | L1 is gated — the task-gate absorb needs base-pin byte equality and conformance; store scope checks HEAD equality and conformance before the advisory | **CONFIRMED** | Driver P10 / P11 / P13; R-11b (conformant hand commit → advisory, lag route) and R-11c (non-conformant → blocking, no lag route); R-9 (a change during the task still blocks at exit 3 even after an unrelated finalize absorbed the baseline: `finalize.base-mismatch`, HEAD unchanged). **What the claim does not reach:** `(R3, F4)` and `(R3, F7)` — neither contradicts it, the source pass is silent on both. |
| K2a | …a failed blob read answers `None`, the blocking result | **CONFIRMED** | R-12c: a PATH shim failing only `git cat-file blob <pin>:<path>` turns the absorb into `reconciliation.conflict-block`; the pass-through control absorbs. |
| K3 | L2 agrees at both scopes: with history blocks; history-less + a carrying branch routes at the switch; none routes at `unmanage`; tags do not count | **CONFIRMED** | R-10: local, remote-tracking-only, tag-only, reset, and the with-history control — `(code, target)`, severity, message and route equal at store and task scope in the four advisory arms; tag-only takes the `unmanage` arm. |
| K3a | …a failed history read answers *present*; a failed branch read answers *carried* | **CONFIRMED** (closes the driver's §5 #6) | R-12a: `git for-each-ref refs/heads refs/remotes` failing → the reset arm (no ref carries the path) is routed at the **switch-back**, never at `unmanage`. R-12b: `git log HEAD -1 -- <path>` failing → the local arm goes **blocking**, exit 1. |
| K4 | `unmanage` drops baseline + edges only on a managed entry, persists nothing on a no-op, does not follow symlinks, and no longer claims absent bytes were left on disk | **CONFIRMED** | R-7 (present: 1 → 0, file kept; second run no-op; never-managed no-op; a symlink no-op, baseline 1 → 1), R-10 (absent: *there was no file at … to leave on disk*, 1 → 0). `(R3, F5)` is not a contradiction — it drops nothing — and `(R3, F7)` is outside what the claim says. |
| K5 | no schema boundary violation: rc.23 → rc.24 moves no manifest and no pinned JSON key | **CONFIRMED for the manifests · OPEN LEAD for the JSON keys** | `git diff --stat jigc-v1.0.0-rc.23 jigc-v1.0.0-rc.24 -- '*schema-manifest.yaml'` is empty (a repository read), and pack-load's freeze assertion passed in every rig above (each ran the binary). The pinned-key half is row 10's and was not driven here. |

**Net:** four claims and five consistency findings — nothing the binary contradicts except one phrase
(C1c). Three sub-claims stay open because they are negatives or were not constructed. The source pass's
closing line, *No new tier-1 lead found*, is a statement of what it found, not a claim about the binary;
`(R3, F7)` is recorded against it as **silence, not refutation, in either direction**.

### 8.4 · Driver defects and baseline rows — status and tier

Tier predicate: **1** = exit-0 loss or repository harm through a committing, destroying or moving door ·
**2** = a posture or route dead end · **3** = a surface says something the binary does not do.

| key | origin | status | tier | why this tier — and, for tier 1, which half is missing |
|---|---|---|---|---|
| `(R3, F1)` — the strong arm's *adopt* exit refuses: `jigc rename <old-id>` → `store.not-found` | driver | **CONFIRMED** (R-4, three states) | **3** | The route's other exit (the revert) runs verbatim and clears the row, so it is not a dead end. Not tier 1: exit 1, HEAD unchanged, nothing written. |
| `(R3, F2)` — `jigc ingest` dies on any tracked `.md` missing from the worktree | driver | **CONFIRMED** (R-1) | **3** | `ingest --help` promises exit 0. Not tier 1: the exit is 1 (the exit-0 half is missing) and `file-state.json` is byte-identical before and after (`cmp`), so the loss half is missing too. |
| `(R3, F3)` — `write.non-reparseable` says *nothing was persisted*; the copy-in is persisted | driver | **CONFIRMED** (R-5, R-5b) | **3** | False sentence; both gates block at exit 3 before and after, HEAD unchanged. R-5b adds: the refused write turns a hand-repairable posture (repair the file → the block clears) into a discard-only one — the write route does name the discard, so still not a dead end. Not tier 1: no exit 0, no loss. |
| `(R3, F4)` — a pulled edit to the machine-owned milestone record is absorbed by the next unrelated finalize | driver | **CONFIRMED, RE-TIERED** (R-6, R-6e, R-6f) | **2** (driver: 3) | After the absorb every milestone door — `finalize`, `join`, `list-tasks`, `discard`, `add-task` — answers `milestone.terminal` (*a settled milestone is over and has no workbench*) while `jigc task list` still shows the live sub-task holding staged prose, and the sub-task's own `task finalize` answers `finalize.milestone-sub-task`. Running the record door's earlier route late (*revert the commit that changed it*) restores the record to what jigc last wrote and the door **conflict-blocks on the restored bytes, with the same route**. The exit that works — a second unrelated finalize, which absorbs the restored record — is on no route of that door. **Not tier 1:** the exit-0 half is there (the absorbing finalize exits 0), the loss-or-harm half is not — the sub-task's staged prose is 1 before and 1 after, the record's bytes at HEAD are the teammate's own commit, and the absorbing commit carries `note1.txt` only. |
| `(R3, F5)` — `jigc unmanage` matches its path literally | driver | **CONFIRMED** (R-7) | **3** | False *is not managed* at exit 0 with the baseline 1 → 1. The JSON arm, which the driver left undriven, answers `{"identity": null, "dropped": false}` for `./<path>` against `"identity": "adr:alpha-cache"` for the bare spelling. Nothing dropped, nothing lost. |
| `(R3, F6)` — a compose-gate refusal leaves a minted live task; *then re-run* collides | driver | **CONFIRMED** (R-3, six arms on `task amend`) | **3** | One hop from clean (`task discard <id> --force` exits 0 and clears it). The second door (`start --workflow single-task`) was not re-driven; the driver's P20 stands for it. |
| **`(R3, F7)`** — with no file-state baseline, an uncommitted out-of-band edit made after a task's first write to that doc is overwritten by `task finalize` at exit 0 | driver's §5 #17, driven by the reconciler | **CONFIRMED** (R-8, R-13) | **1** | Both halves — §8.5. |
| `(7, A7-F1)` | baseline | **STILL-OPEN** (R-1) | 3, as recorded | The trailer asserts a commit and a `git mv`; neither happened. |
| `(7, A7-F2)` | baseline | **STILL-OPEN** (R-2) | 3, as recorded | Two doors disagree about the same two files at one commit. |
| `(7, C-2)` | baseline | **STILL-OPEN (expected)** — declared residual, not re-filed (R-2) | — | Now with the exit-0 half shown. |
| §D axis-7 lead — `probe-unreliable` | baseline | **CLOSED** by a drive, no defect (R-11a; driver P4) | — | Exit 1, the member's own trailer, `report_only: false`, one key on both arms. |
| **F21** | declared open gap | **STILL-OPEN (expected)** — a measurement (R-3) | — | Bounded wider than the driver's three bodies: also a command-ref, a schema-ref and a `replace-step` delta. |

### 8.5 · `(R3, F7)` — the finding

**Statement.** When jigc holds no file-state baseline for a managed doc, an out-of-band edit made to that
doc **after** a live task's first write to it (the copy-in) and never committed is overwritten when that
task finalizes. `jigc task validate` exits 0, `jigc task finalize --dry-run` exits 0 with
*would commit … promoted <doc>*, `jigc task finalize` exits 0, and the edit is in no commit, not in the
worktree, and nowhere under `.jigc/`.

**Door / cell.** `task finalize` (committing) × DRIFTED + TOUCHED with the baseline absent. Three ways in,
all driven:

| way in | acts besides the task and the edit | repro |
|---|---|---|
| a **second clone** of an adopted repository, before its first finalize | none | R-13 (fresh-clone) |
| `jigc unmanage <doc>` **before** the task's first write | one `unmanage` | R-13 (unmanage-first) |
| `jigc unmanage <doc>` **after** the conflict-block has fired | one `unmanage`, off the emitted route | R-8 |

**Both halves of the predicate.** *Exit 0:* every jigc call in the sequence. *Loss:* before-control — the
hand line counts 1 in the worktree `VISION.md`, 0 in the staged copy, 0 at HEAD; after — 0 in the worktree,
0 at HEAD, 0 across every commit on every ref, 0 files under `.jigc/` (`command grep -rlF`); `git status`
clean. *Door:* the promotion `task finalize` performs.

**Controls and bounds (all driven).** With the baseline present the same sequence blocks:
`reconciliation.conflict-block`, exit 3, HEAD unchanged, the hand line still on disk (R-8 control; R-13b —
the second clone after one unrelated finalize has adopted the baselines). The edit made **before** the
task's first write is carried into the copy-in and lands with the task's prose — the *silent merge*, no
loss (R-8b hand-then-write). The edit **committed** during the task is caught by `finalize.base-mismatch`,
exit 3 (R-8b write-then-handcommit). So the loss is exactly: baseline absent × edit after the copy-in ×
edit uncommitted.

**Is it declared?** `design/storage.md` → *What none of this buys* declares that a lost baseline *switches
the guarantee off — silently, at exit 0, with both sides' bytes merged into one commit*, names the fresh
clone and `jigc unmanage` as legitimate origins, and points at the standing pair
`crates/cli/tests/reconciliation_baseline_contrast.rs`. That pair makes the human edit **before** the
in-task write and asserts the commit carries both. The order driven here is the other one, and its cost is
not a merge: one side's bytes are gone. Neither the paragraph nor the pair states that. The driver's §5 #17
read the paragraph as covering the cell; the drive says it covers the neighbouring one.

**Surfaces on the way.** On the second clone `jigc validate` reports `file-state.un-baselined` with the
route *no action needed — the doc is baselined on its next author or finalize*; a `doc set-slot` write did
not baseline it (0 after the write). At the gate the only rows are `file-state.staged-copy` and
`file-state.baseline-adopt`, both *no action needed*. `jigc unmanage` over a doc a live task holds a staged
copy of says *the file is left on disk* and nothing about the task.

**What bounds its weight — for the triage, not graded here.** (1) The bytes lost are uncommitted worktree
bytes written outside jigc into a managed doc while a task held a staged copy of it; with a baseline this
is the posture jigc blocks. (2) The mechanism (`UNKNOWN → baseline-adopt`) predates the M54/M55 range — the
pair's header dates it M46; that was **read, not driven** on an older binary. (3) The window on a clone
closes at its first finalize (R-13b). (4) The two `unmanage` ways need an operator act no emitted route
asks for; the second-clone way needs none.

### 8.6 · Reconciler repro blocks

Shared: `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit`.
`land_adr`, the teammate clone and the unrelated single-task (`jigc start --workflow single-task "unrelated
work N"`, `git add noteN.txt`, the three commit-doc writes) are the driver's §3 helpers, rebuilt.
`THESIS` = the rig's thesis sentence; `TP` = *Which domains earn a pack, and when.*; `HL` = *A hand line
written out of band.*

#### R-1 · `(7, A7-F1)`, `(R3, F2)` (rig `committed-singletons`)

```
$ rm CHANGELOG.md                                   # unstaged, uncommitted
$ jigc validate                       -> exit 1
blocking (gates at finalize) · reconciliation.rename — tracked managed doc changelog:changelog (CHANGELOG.md) is missing
  route: restore CHANGELOG.md, or confirm the deletion by dropping it from the index: `jigc unmanage CHANGELOG.md`
blocking · schema-conformance.home-vacated — … the worktree no longer holds it …
out-of-band rename detected — a structural-identity change this commit introduced; the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`).
$ jigc validate --format json         -> exit 1 ; report_only=false ; KEY (reconciliation.rename, CHANGELOG.md) blocking · (schema-conformance.home-vacated, CHANGELOG.md) blocking
  HEAD unchanged · git status --short -> " D CHANGELOG.md" · renames in the HEAD commit -> none
$ jigc doc list                       -> exit 0 (three rows; no changelog row)
$ jigc ingest                         -> exit 1
could not read the candidate at "$REPO/CHANGELOG.md": No such file or directory (os error 2)
$ jigc ingest --format json           -> exit 1 ; stdout empty ; stderr {"error": "could not read the candidate at …"}
  .jigc/state/file-state.json before vs after the failed ingest (cmp) -> identical
$ git -C $REPO checkout -- CHANGELOG.md ; jigc validate   -> exit 0
$ git rm -q CHANGELOG.md ; git commit -q -m "remove the changelog"   -> exit 0
$ jigc validate                       -> exit 1   (the same rename row and the SAME trailer; home-vacated names the removing commit)
```

#### R-2 · `(7, A7-F2)`, `(7, C-2)` and its exit-0 half (rig `committed-singletons`)

```
$ printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml ; git add -A ; git commit -q -m "drop the methodology pack"
  stamps: VISION.md 1 · docs/roadmap.md 1 · docs/decisions-log.md 1
$ jigc validate                       -> exit 1
blocking · schema-conformance.orphaned-instance — committed doc `docs/decisions-log.md` … no resolved doctype claims this path …
  route: ask `jigc ingest`, which re-reads the file: … or take it out of jigc's world: `jigc unmanage docs/decisions-log.md`, then delete the file or its `schema-version:` stamp …
blocking · schema-conformance.orphaned-instance — committed doc `docs/roadmap.md` … (same)
a stamped committed doc is claimed by no resolved doctype — … ask `jigc ingest` which of them a resolved schema still accepts, …
$ jigc validate --format json         -> exit 1 ; KEY (schema-conformance.orphaned-instance, docs/decisions-log.md) · (…, docs/roadmap.md)
$ jigc ingest                         -> exit 0
adoptable CHANGELOG.md → changelog  (adopted — indexed + baselined, no file moved)
unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)
  unmanaged — matches no managed schema; staying a plain file is a legitimate end-state — no action needed. …
$ jigc ingest --format json           -> exit 0 ; seven rows, `finding: null` on each
$ jigc doc list                       -> exit 0 ; (none) docs/decisions-log.md orphaned · (none) docs/roadmap.md orphaned
  lines naming VISION.md: validate 0 · doc list 0 · ingest 0 · migrate-corpus --dry-run 0

# the exit-0 half of (7, C-2): retire the two docs/ orphans, leave the stamped root-placement instance
$ git rm -q docs/roadmap.md docs/decisions-log.md ; git commit -q -m "remove the two docs orphans"
$ jigc unmanage docs/roadmap.md ; jigc unmanage docs/decisions-log.md   -> exit 0 / 0 ("… there was no file at … to leave on disk")
  stamp on VISION.md: 1 ; git ls-files VISION.md -> VISION.md
$ jigc validate                       -> exit 0
no findings — the committed store validates clean
$ jigc validate --format json         -> exit 0 ; report_only=true ; findings=[]
```

#### R-3 · F21, its bound, and `(R3, F6)` (rig `fresh`, one rig per arm; step sources under `$RIG/src`)

```
bodies:  include   = `{{ include: step:no-such-step }}`        cycle     = `{{ include: step:loop }}` (file loop.yaml)
         cmdref    = `Use the command below.\n\n{{ cli.no-such-command }}`
         schemaref = `Read the schema below.\n\n{{ schema:no-such-doctype }}`

each of include · cycle · cmdref · schemaref:
$ jigc config insert-step --workflow amend --after amend-message $RIG/src/<f>.yaml   -> exit 0
$ git add -- .jigc/config ; git commit -q -m "chore: a project structural delta"     -> exit 0
$ jigc validate                       -> exit 0  "no findings — the committed store validates clean"
$ jigc validate --format json         -> exit 0  findings=[] report_only=true blocking_probes=[]
  ls .jigc/tasks -> (empty)
$ jigc task amend "repair the message"   -> exit 1
  include:   blocking · workflow-refs.include-resolves — include `step:no-such-step` resolves to no step file in the cascade
  cycle:     blocking · workflow-refs.include-cycle-absent — include cycle: loop -> loop
  cmdref:    blocking · workflow-refs.command-ref-resolves — command-ref `{{cli.no-such-command}}` resolves to no catalog entry   at: step:cmdref
  schemaref: blocking · workflow-refs.schema-ref-resolves — schema-ref `{{schema:no-such-doctype}}` resolves to no doctype in the composed cascade   at: step:schemaref
  route (all four): fix the workflow/step/catalog definition the message names (…), then re-run
  ls .jigc/tasks -> repair-the-message                                   <- (R3, F6)
$ jigc task amend "repair the message"   -> exit 1
blocking · task.serial-collision — task `repair-the-message` is already active
  route: resume with `jigc start --task repair-the-message` or abandon with `jigc task discard repair-the-message --force`
$ jigc start --task repair-the-message   -> exit 1  (the same workflow-refs finding)
$ jigc task discard repair-the-message --force   -> exit 0 ; ls .jigc/tasks -> (empty)
$ jigc workflow amend --preview          -> exit 1  workflow.verb-routed ; nothing minted
$ jigc upgrade                           -> exit 0  "no findings — 1 recorded config delta(s) re-apply clean against the current pack, …"

replace arm:
$ jigc config replace-step workflow:amend#amend-message $RIG/src/swapped.yaml   -> exit 0   (swapped.yaml = the broken include)
$ git add -- .jigc/config ; git commit -q -m "chore: a project structural delta (replace-step)"
$ jigc validate (T, J)                -> exit 0, clean, findings=[]
$ jigc task amend "repair the message"   -> exit 1  workflow-refs.include-resolves ; ls .jigc/tasks -> repair-the-message

shadow control (the granted hand-written `.jigc/config/workflows/amend.yaml` = the shipped amend.yaml + the broken include, committed):
$ jigc validate                       -> exit 0
blocking · workflow-refs.include-resolves — include `step:no-such-step` resolves to no step file in the cascade
  at: workflow:amend
1 finding(s) — report-only at store scope (exit 0); these gate at compose (`jigc start`), never at the task or milestone boundary.
$ jigc validate --format json         -> exit 0 ; report_only=true blocking_probes=["workflow-refs"] ; KEY (workflow-refs.include-resolves, workflow:amend) blocking
$ jigc task amend "repair the message"   -> exit 1 ; ls .jigc/tasks -> repair-the-message   (F6 is not the delta path's)
```

#### R-4 · `(R3, F1)` (three states)

```
placement, staged (rig `committed-singletons`):
$ git mv docs/roadmap.md docs/plan.md ; jigc validate   -> exit 1
blocking (gates at finalize) · reconciliation.rename — … docs/plan.md has the same content hash — likely renamed via `git mv`
  route: adopt it as a CLI-owned rename (re-points every referrer atomically): `jigc rename roadmap:roadmap --to "<New Title>"`; or revert the move: `git -C $REPO mv docs/plan.md docs/roadmap.md`
$ jigc rename roadmap:roadmap --to "Plan"               -> exit 1
blocking · store.not-found — no managed doc `roadmap:roadmap` to rename (expected at docs/roadmap.md)
  route: `jigc describe` lists the doctype surface — check the id you typed against it
$ jigc rename roadmap:roadmap --to "Plan" --format json -> exit 1 ; stdout empty ; stderr findings envelope, KEY (store.not-found, roadmap:roadmap)
  HEAD unchanged · git status --short -> "R  docs/roadmap.md -> docs/plan.md" · jigc validate -> exit 1
$ git -C $REPO mv docs/plan.md docs/roadmap.md ; jigc validate   -> exit 0, no rename row

location, staged (rig `fresh`, one ADR landed):
$ git mv docs/decisions/single-node-cache.md docs/decisions/one-node-cache.md ; jigc validate -> exit 1 (the same route shape)
$ jigc rename adr:single-node-cache --to "One node cache"   -> exit 1  store.not-found
$ git -C $REPO mv docs/decisions/one-node-cache.md docs/decisions/single-node-cache.md ; jigc validate -> exit 0 "no findings"

location, committed:
$ git mv … ; git commit -q -m "move the adr"            -> exit 1  (hook: "… use `jigc rename` instead (commit blocked).")
$ git commit -q --no-verify -m "move the adr"           -> exit 0  (hook bypassed: the cell is the committed-move state)
$ jigc validate --format json         -> exit 1 ; KEY (reconciliation.rename, docs/decisions/single-node-cache.md) blocking, the adopt-or-revert route
$ jigc rename adr:single-node-cache --to "One node cache"   -> exit 1  store.not-found
$ jigc rename adr:one-node-cache --to "One node cache"      -> exit 0  "renamed adr:one-node-cache -> adr:one-node-cache (… -> …), repointed 0 referrer(s)"
  a commit lands: "rename docs/decisions/one-node-cache.md -> docs/decisions/one-node-cache.md", 1 file, 1 insertion, 1 deletion
$ jigc validate --format json         -> exit 1 ; the rename row is now the weak arm (route: restore …, or … `jigc unmanage docs/decisions/single-node-cache.md`)
$ jigc unmanage docs/decisions/single-node-cache.md ; jigc validate --format json   -> exit 0 / exit 0, clean
```

#### R-5 · `(R3, F3)`; R-5b · what the refused write does to the hand-repair route (rig `committed-singletons` + teammate)

```
teammate removes the `## Thesis` heading line from VISION.md, commits, pushes;  $ git pull -q --ff-only   -> exit 0
$ jigc start --workflow single-task "sharpen the open questions"
  before the write: ls .jigc/tasks/<id>/docs -> commit:<id>.md provenance.json
$ jigc task validate <id> --format json   -> exit 3 ; KEY (reconciliation.conformance-block, VISION.md) blocking
     route: fix the file to restore conformance, or revert the edit — this is the one case a managed file is yours to hand-edit: …
$ jigc doc set-slot vision:vision#open-questions --from-file - --task <id>   -> exit 1
blocking · write.non-reparseable — write rejected: the source does not conform to the schema (section heading "Invariants" does not match required section `thesis`)
  route: nothing was persisted — revise the payload so the result still conforms …; if the staged source itself is what no longer parses, `jigc task discard <task-id> --force` and start the task over
  after the write: ls .jigc/tasks/<id>/docs -> commit:<id>.md provenance.json vision:vision.md
  staged copy == on-disk VISION.md (cmp): yes ; the payload in the staged copy: 0
$ jigc task validate <id> --format json   -> exit 3 ; KEY (file-state.staged-copy, VISION.md) advisory · three conformance.* blocking · (reconciliation.conflict-block, VISION.md) blocking
$ jigc doc list --task <id>               -> exit 0 ; vision:vision  VISION.md  managed
$ jigc task finalize <id> --format json   -> exit 3 ; HEAD unchanged

R-5b, the conformance-block route (put the heading line back on disk, uncommitted), two rigs:
control (no write attempted first):  task validate -> the conformance-block row is gone ; doc set-slot -> exit 0 (copied in) ; task validate -> (reconciliation.conflict-block, VISION.md)
after the refused write:             task validate -> unchanged (three conformance.* + conflict-block) ; doc set-slot -> exit 1 write.non-reparseable again
```

#### R-6 · `(R3, F4)`; R-6e · the record door at each stage; R-6f · the terminal posture (rig `committed-singletons` + teammate)

```
$ jigc milestone create "Vision pass" ; jigc milestone add-task vision-pass "Sharpen the open questions"   -> exit 0 / 0
$ jigc doc set-slot vision:vision#open-questions --from-file - --task sharpen-the-open-questions            (the sub-task's staged prose: 1)
teammate edits docs/milestone-records/vision-pass.md (`status: active` -> `status: joined`), commits, pushes;  $ git pull -q --ff-only -> exit 0
$ jigc milestone add-task vision-pass "Evict cold entries"   -> exit 1
blocking · reconciliation.conflict-block — conflict on `docs/milestone-records/vision-pass.md`: the milestone record is machine-maintained and was edited out of band since jigc last wrote it
  route: restore `docs/milestone-records/vision-pass.md` to what jigc last wrote (`git -C $REPO checkout -- …` for an uncommitted edit, else revert the commit that changed it) and re-run this command — an external edit to a machine-maintained record is never merged and never clobbered
$ jigc validate --format json         -> exit 0 ; KEY (file-state.hash-matches, docs/milestone-records/vision-pass.md) advisory, route "the baseline lags `HEAD`; absorbed at the next finalize"
$ jigc task finalize unrelated-work-1 --format json   -> exit 0 ; committed manifest: note1.txt only
  KEY (reconciliation.absorb, docs/milestone-records/vision-pass.md) advisory — "no action needed — the external edit was absorbed into the baseline"
$ jigc validate --format json         -> exit 0 (no record row)
$ jigc milestone add-task vision-pass "Evict cold entries"   -> exit 1
blocking · milestone.terminal — milestone `vision-pass` is `joined` — a settled milestone is over and has no workbench
  route: read the settled record with `jigc doc show milestone-record:vision-pass`; new work starts a new milestone (`jigc milestone create "<title>"`)

R-6f, the same posture, every door of the live sub-task:
$ jigc milestone finalize vision-pass   -> exit 1  milestone.terminal        $ jigc milestone join vision-pass        -> exit 1  milestone.terminal
$ jigc milestone list-tasks vision-pass -> exit 1  milestone.terminal        $ jigc milestone discard vision-pass     -> exit 1  milestone.terminal
$ jigc task finalize sharpen-the-open-questions   -> exit 3  finalize.milestone-sub-task
$ jigc task list                        -> exit 0  "1 active task(s)" — sharpen-the-open-questions [sub-task]
  the sub-task's staged prose: 1 before, 1 after ; TP in HEAD:VISION.md -> 0

R-6e, the record door (`jigc milestone add-task`) through one clean sequence:
  after the pull                                             -> exit 1  reconciliation.conflict-block
  after the unrelated finalize (1 absorb row)                -> exit 1  milestone.terminal
  after `git revert <the teammate's commit>` (status: active, the bytes jigc last wrote)
                                                             -> exit 1  reconciliation.conflict-block, the same "restore … to what jigc last wrote" route
  after a second unrelated finalize (1 absorb row)           -> exit 0  "added task:probe-four to milestone:vision-pass"
control (a separate rig): the route run BEFORE any finalize — pull, `git revert <the teammate's commit>` -> add-task exit 0
```

#### R-7 · `(R3, F5)` and the `unmanage` consistency cells (rig `fresh`, one ADR `docs/decisions/alpha-cache.md`)

```
baseline before each: command grep -c 'alpha-cache' .jigc/state/file-state.json -> 1
$ jigc unmanage ./docs/decisions/alpha-cache.md              -> exit 0  "no-op: ./docs/decisions/alpha-cache.md is not managed (nothing to drop)" ; baseline 1
$ jigc unmanage docs//decisions/alpha-cache.md               -> exit 0  no-op ; baseline 1
$ jigc unmanage docs/decisions/../decisions/alpha-cache.md   -> exit 0  no-op ; baseline 1
$ jigc unmanage $REPO/docs/decisions/alpha-cache.md          -> exit 0  no-op ; baseline 1
$ jigc unmanage ./docs/decisions/alpha-cache.md --format json -> exit 0  {"path": "./docs/decisions/alpha-cache.md", "identity": null, "dropped": false} ; baseline 1
$ (cd docs && jigc unmanage decisions/alpha-cache.md)        -> exit 0  no-op ; baseline 1
$ ln -s docs/decisions/alpha-cache.md link.md ; jigc unmanage link.md   -> exit 0  no-op ; baseline 1
$ jigc unmanage docs/decisions/alpha-cache.md                -> exit 0  "unmanaged … (adr:alpha-cache) — dropped its file-state baseline + forward edges; the file is left on disk. …" ; baseline 0 ; ls: present
$ jigc unmanage docs/decisions/alpha-cache.md                -> exit 0  no-op
$ jigc unmanage docs/decisions/alpha-cache.md --format json  -> exit 0  {"path": …, "identity": "adr:alpha-cache", "dropped": false}
$ jigc unmanage README.md --format json                      -> exit 0  {"path": "README.md", "identity": null, "dropped": false}
```

#### R-8 · `(R3, F7)` by `unmanage` after the block; R-8b · the three orders (rig `committed-singletons`)

```
$ jigc start --workflow single-task "sharpen the open questions"
$ jigc doc set-slot vision:vision#open-questions --from-file - --task <id>   <<< "Which domains earn a pack, and when."     (+ the three commit-doc writes)
(append HL under VISION.md's thesis, in the worktree, uncommitted)
  [before] HL: worktree VISION.md -> 1 ; the task's staged copy -> 0 ; HEAD -> 0        TP: staged copy -> 1 ; worktree -> 0
  [before] command grep -c 'VISION.md' .jigc/state/file-state.json -> 1

control (stop here, no unmanage):
$ jigc task validate <id> --format json   -> exit 3 ; KEY (reconciliation.conflict-block, VISION.md) blocking
$ jigc task finalize <id> --dry-run / --format json   -> exit 3 / 3 ; HEAD unchanged ; HL in the worktree -> 1

the arm:
$ jigc task validate <id>                 -> exit 3  reconciliation.conflict-block
$ jigc unmanage VISION.md                 -> exit 0
unmanaged VISION.md (vision:vision) — dropped its file-state baseline + forward edges; the file is left on disk. It still sits at the managed home, …
  command grep -c 'VISION.md' .jigc/state/file-state.json -> 0
$ jigc task validate <id> --format json   -> exit 0 ; KEY (file-state.staged-copy, VISION.md) advisory · (file-state.baseline-adopt, VISION.md) advisory "no action needed — the baseline was adopted on first encounter"
$ jigc task finalize <id> --dry-run       -> exit 0
would commit — docs(vision): sharpen the open questions
  promoted VISION.md
$ jigc task finalize <id> --format json   -> exit 0 ; committed manifest [{"kind": "promoted", "path": "VISION.md"}]
  HEAD moved ; git status --short -> (clean)
  [after] HL: worktree VISION.md -> 0 ; HEAD -> 0 ; commits on any ref carrying it -> 0 ; files under .jigc carrying it (command grep -rlF) -> 0
  [after] TP: HEAD -> 1

R-8b, the same with the order varied (one rig each; `unmanage VISION.md` exit 0 in all three):
  write-then-hand        (as above)                         task finalize -> exit 0 ; HL after: worktree 0 · HEAD 0 · any ref 0 · .jigc 0     <- the loss
  hand-then-write        (HL in the copy-in: staged copy 1)  task finalize -> exit 0 ; HL after: worktree 1 · HEAD 1                          <- the declared silent merge
  write-then-handcommit  (HL committed with plain git)       task finalize -> exit 3  finalize.base-mismatch ; HL after: worktree 1 · HEAD 1   <- caught
```

#### R-9 · adversarial L1 probe: an unrelated finalize between a task's write and its gate (rig `committed-singletons` + teammate)

```
task A: start, set-slot vision:vision#open-questions ; the teammate's line pulled (HEAD 1 · worktree 1 · A's staged copy 0)
$ jigc task validate A                    -> exit 3  reconciliation.conflict-block
$ jigc task finalize unrelated-work-1 --format json   -> exit 0 ; KEY (reconciliation.absorb, VISION.md) advisory
$ jigc task validate A --format json      -> exit 0  (staged-copy advisory only — the declared preview bound, O-4)
$ jigc task finalize A --dry-run / --format json      -> exit 3 / 3  KEY (finalize.base-mismatch, task:A) ; HEAD unchanged
  [after] the teammate's line: HEAD 1 · worktree 1 ; A's prose in HEAD -> 0        — no loss
```

#### R-10 · L2, four arms and the control (rig `fresh`; one rig per arm)

```
arm      refs carrying the path                      store row (validate)                 task row (task validate)       parity   exit
local    refs/heads/milestone/x/main                 reconciliation.rename advisory       the same                       equal    0 / 0     switch-back route, no `unmanage`
remote   refs/remotes/origin/milestone/x/main        reconciliation.rename advisory       the same                       equal    0 / 0     switch-back route
tag      refs/tags/v-x                               reconciliation.rename advisory       the same                       equal    0 / 0     "no branch, local or remote-tracking, can bring … back …: drop it with `jigc unmanage <path>`"
reset    (none)                                      reconciliation.rename advisory       the same                       equal    0 / 0     the `unmanage` route
history  refs/heads/main (staged `git rm`)           reconciliation.rename BLOCKING       the same + finalize.carried-staged      —       1 / 3     "restore …, or confirm the deletion …: `jigc unmanage <path>`" + the oob-rename trailer
every arm: file present no ; baseline 1 ; `jigc task finalize <unrelated> --dry-run` -> exit 0 "would commit …" (history arm: exit 3) ; `jigc doc list` -> exit 0, the ADR not listed
routes run: local/remote `git switch -q milestone/x/main` -> file present, 0 rows at the path ;
            tag/reset   `jigc unmanage <path>` -> exit 0 "… there was no file at <path> to leave on disk", baseline 1 -> 0, edges 0, 0 rows at both scopes
```

#### R-11 · the remaining consistency cells

```
a · probe-unreliable (rig `vendored`; JIGC_DOC_CODE_PROBE unset in the control, set to a mode-644 file in the arm)
$ jigc validate                       -> exit 0 "no findings"
$ JIGC_DOC_CODE_PROBE=$RIG/probes/nonexec jigc validate   -> exit 1
blocking (gates at finalize) · pack-probe-integrity.probe-failure — probe `doc-code` could not start `$RIG/probes/nonexec`: Permission denied (os error 13)
pack-probe-integrity finding(s) present — the sweep could not complete and exits non-zero; the store result is not trustworthy.
  --format json -> exit 1 ; report_only=false blocking_probes=["pack-probe-integrity"] ; KEY (pack-probe-integrity.probe-failure, doc-code) blocking

b · L1 store scope, conformant hand commit (rig `committed-singletons`; on-disk == HEAD blob: yes)
$ jigc validate (T, J)                -> exit 0 ; KEY (file-state.hash-matches, VISION.md) advisory, route "the baseline lags `HEAD`; absorbed at the next finalize" ; report_only=true blocking_probes=[]

c · L1 store scope, non-conformant hand commit (the `## Thesis` line removed; on-disk == HEAD blob: yes)
$ jigc validate (T, J)                -> exit 0 ; KEY (file-state.hash-matches, VISION.md) blocking, route "review the out-of-band edit …" + three conformance.* blocking ; blocking_probes=["conformance","file-state"]

d · foreign-squatter (rig `fresh`; a hand-written CHANGELOG.md committed)
$ jigc validate (T, J)                -> exit 1 ; KEY (schema-conformance.unadopted-instance, CHANGELOG.md) advisory ; report_only=false blocking_probes=[] ; the member's trailer
$ jigc doc list                       -> exit 0 ; changelog:changelog  CHANGELOG.md  unregistered
$ jigc ingest (T, J)                  -> exit 0 ; needs-reconcile CHANGELOG.md → changelog ; row finding KEY (conformance.section-missing, changelog:changelog#unreleased-changes) ; route `jigc migrate $REPO/CHANGELOG.md --as changelog` …

e · the driver's row 137 (rig `fresh`)
$ jigc config insert-step --workflow amend --after amend-message $RIG/src/extra.yaml        -> exit 0
$ jigc config insert-step --workflow single-task --after locate $RIG/src/extra.yaml         -> exit 1
blocking · config.step-id-collision — `extra` is already a step id — the native file's id is its basename, so this would shadow the existing step, not add a new one
  route: rename the source file so its basename is a fresh step id, then re-run
  --format json -> exit 1 ; stdout empty ; stderr {"error": "blocking · config.step-id-collision — …"}
```

#### R-12 · one failed git read (a `git` shim first on `PATH` fails exactly one read and passes every other call through; the shim's log counts both)

```
a · rig `fresh`, the reset arm (no ref carries the path)
  control, pass-through shim:  jigc validate -> exit 0 ; advisory, the `unmanage` route                                   (10 git calls passed, 0 failed)
  `git for-each-ref --format=%(refname) refs/heads refs/remotes` fails:
                               jigc validate -> exit 0 ; advisory, the SWITCH-BACK route, no `unmanage`                   (8 passed, 1 failed)
b · rig `fresh`, the local arm
  control:                     jigc validate -> exit 0 ; advisory, the switch-back route
  `git log HEAD -1 --format=%H -- <the ADR path>` fails:
                               jigc validate -> exit 1 ; BLOCKING reconciliation.rename, "restore …, or confirm the deletion …", the oob-rename trailer   (7 passed, 1 failed)
c · rig `committed-singletons` + teammate; a task minted after the pull, one staged write
  `git cat-file blob <pin>:VISION.md` fails:
                               jigc task validate <id> -> blocking · reconciliation.conflict-block — conflict on `VISION.md` …   (9 passed, 1 failed)
  control, run second:         jigc task validate <id> -> advisory · reconciliation.absorb — external edit absorbed: `VISION.md`
```

#### R-13 · `(R3, F7)` with the baseline absent before the task's first write; R-13b · the control

```
fresh-clone (a second clone of the rig repository, made after the eval; user.email mate@example.com):
$ git clone -q $REPO $RIG/clone2 ; cd $RIG/clone2
  .jigc/state/file-state.json present: no ; tracked files under .jigc: 5
$ jigc validate                       -> exit 0 ; advisory · file-state.un-baselined — committed doc `VISION.md` is not yet baselined in the file-state record
$ jigc start --workflow single-task "sharpen the open questions" ; jigc doc set-slot vision:vision#open-questions … ; the three commit-doc writes
  baseline for VISION.md after the first write: none
(append HL under VISION.md's thesis, uncommitted)
  [before] HL: worktree -> 1 ; staged copy -> 0 ; HEAD -> 0
$ jigc task validate <id>             -> exit 0 ; rows at VISION.md: advisory · file-state.staged-copy ; advisory · file-state.baseline-adopt
$ jigc task finalize <id>             -> exit 0
advisory · file-state.baseline-adopt — baseline adopted: `VISION.md`   (and the three other singletons)
finalized <sha> — docs(vision): sharpen the open questions
  HEAD moved ; git status --short -> (clean)
  [after] HL: worktree -> 0 ; HEAD -> 0 ; files under .jigc carrying it -> 0 ; TP in HEAD -> 1

unmanage-first (the rig itself): `jigc unmanage VISION.md` (exit 0, baseline 1 -> 0), then the same sequence
$ jigc task validate <id> -> exit 0 ; jigc task finalize <id> -> exit 0 ; [after] HL: worktree 0 · HEAD 0 · .jigc 0 ; TP in HEAD 1

R-13b, control — the second clone after one unrelated finalize (4 baseline-adopt rows; baseline for VISION.md: 1), then the same sequence:
$ jigc task finalize <id>             -> exit 3
blocking · reconciliation.conflict-block — conflict on `VISION.md`: an external edit and this task's staged writes both changed it
  HEAD unchanged ; HL in the worktree -> 1
```

### 8.7 · Open leads

| # | lead | why it is open |
|---|---|---|
| L1 | codex C1a — F21's raw read also hides fan-out/join pairing and front-matter breaks | not constructed |
| L2 | codex C1d · K1a — the two completeness negatives (*no other family affected*, *no blocking store code omitted from the flip set*) | a negative cannot be driven to a repro; nothing driven contradicts either |
| L3 | codex K5, second half — the trailer moved no pinned `--format json` key | row 10's subject; not driven here |
| L4 | F21 × a `config remove-step` delta — store scope would validate a step the delta removed (a possible false positive, the inverse of F21) | not constructed (the driver's §5 #15, second half) |
| L5 | `(R3, F4)` extension — a pulled record edit that drops a sub-task row, absorbed, then the join | not driven to a landing: any foreign commit after the milestone's base puts `finalize.base-mismatch` in front of the join (R-6c control: a milestone, one unrelated finalize, `jigc milestone finalize` → exit 3) |
| L6 | `(R3, F4)` — `jigc task discard <sub> --force` in the absorbed-terminal posture | not driven; it bounds how dead the dead end is |
| L7 | `(R3, F7)` bounds — a location-doctype doc · the `milestone finalize` door · the sanctioned cache deletion as the way in · an rc.23-or-older binary (does it predate the range?) | not driven; the cache-deletion way would be a fixture written by removing files under `.jigc/`, which the instrument forbids |
| L8 | the driver's §5 rows still not driven: #1 #2 #3 #4 #5 #7 #8 #9 #10 #11 #12 #13 #14 | unchanged. Closed by the reconciler: #6 (R-12), #16 (R-3), #17 (→ `(R3, F7)`); #15 half-closed (R-3 replace arm) |
| L9 | the driver's row 43 — the surface of `jigc start --task <id>` once the moved history overlaps the task | exit only; not captured by either drive |

**Observations of the reconciler (driven, not graded).** (a) R-2: after the methodology pack leaves, a
committed `git rm` of the two orphaned docs leaves their baselines in place and `jigc validate` says *no
findings* — no dangling-baseline row is raised for a doc whose doctype no longer resolves. (b) R-5b: the
conformance-block's sanctioned hand repair, left uncommitted, is itself an external edit — the task's first
write after it conflict-blocks. (c) R-6c: an unrelated `task finalize` during a live milestone puts
`finalize.base-mismatch` in front of that milestone's join. (d) R-12a: on a failed branch read the
switch-back route states a cause (*lives on a branch this checkout does not carry*) that is a guess —
declared conservative.

### 8.8 · Doors covered

Every clap leaf that is the door of ≥1 driven row, `VERB_KINDS` spelling — the driver's 17 and one the
reconciler added (`config replace-step`):

`validate` · `task validate` · `task finalize` · `task amend` · `task discard` · `milestone finalize` ·
`milestone add-task` · `unmanage` · `ingest` · `doc list` · `doc set-slot` · `rename` · `migrate-corpus` ·
`start` · `workflow` · `config insert-step` · `config replace-step` · `upgrade`

Plus the installed pre-commit hook's store sweep, which is not a clap leaf. Driven by the reconciler only as
posture probes inside R-6f, and not counted: `milestone join` · `milestone list-tasks` · `milestone discard`
· `task list`.
