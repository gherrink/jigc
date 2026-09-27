<!-- Reconciled AXIS 5 file, copied verbatim. Driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.20` (repo HEAD `51e0b8e4`), 2026-09-27. -->

<!-- M53 FOURTH PARTIAL per-axis review — axis 5 · pinned contracts · THE OPUS DRIVER.
     Every row driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.20`,
     repo HEAD `51e0b8e4`, 2026-09-27. Release posture. -->

# M53 fourth partial per-axis review (rc.20, after the usability batch + F-10 `jigc task amend`) — AXIS 5 · pinned contracts · THE OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.20`**, asserted first, before anything
else ran. **Release posture** — the `#[cfg(debug_assertions)]` route/span/quoting fences do **not**
exist in this binary, so a fence violation shows up here as a bad emitted command, never as a panic.
Every route claim below is a claim about the **bytes the release binary printed**, and where it
mattered I **ran those bytes verbatim** and recorded the shell's status.

**What changed under this axis since the rc.19 run** — `7d86f99f..51e0b8e4`, 16 commits: the
**pre-v1 usability batch** (`1b45707c`…`136a0878`, six surface rows) and **F-10 `jigc task amend`**
(`9fa66338`…`e87835ea`, four build tasks + eight review fixes), then the `1.0.0-rc.20` stamp.
Axis 5 owns the question those changes could break silently: **does a new committing arm ship with a
declared envelope, a declared key, a declared exit and a stable finding key — and do the arms that
already shipped still say the same thing?**

**Instrument note, applied.** This harness's `grep` is a shell function honouring `.gitignore`. Every
claim under `.jigc/` below uses `command grep` (or `ls`/`find`), with a before-control where it is a
survival claim. Rigs: `rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit;
eval "$rig"` — two-step eval, `mktemp -d` roots, **no teardown and no `rm -rf` on a variable path
anywhere in this review**. Every fixture beyond a rig state was built by **driving the binary**; the
exceptions are stated at their cells (a `git mv` of a managed doc, which *is* the condition under
test; a `git bisect start` / `git worktree add`; and one `mv` of a pack file into a `mktemp -d`
stash, restored immediately, which is the fence probe's own subject).

**Exit-code discipline.** No exit code in this review was read through a pipe. Every drive is
`cmd > out 2> err; rc=$?`.

I did **not** read the Codex source pass for this axis.

---

## 0 · The door set, derived from the code

Counts read **by symbol** at `HEAD = 51e0b8e4`, not from the design doc's numbers:

| registry | file:line | rows read | vs the rc.19 run |
|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1877` | **48** leaves | 47 → **48** (`task amend`) |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2047` | **48** | 47 → **48** (`task amend` = `Neither`) |
| **`ENVELOPE_ARMS`** | `crates/cli/src/render.rs:6572` | **66** arms | 64 → **66** (`task amend | Composed`; `task finalize | LandedAmend`) |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:1049` | **7** | = |
| **`COMMITTING_DOORS`** | `crates/cli/src/invocation_log.rs:188` | **11** | 10 → **11** (`jigc task finalize (amend)`) |
| `ERROR_CODE_REGISTRY` | `crates/cli/src/invocation_log.rs:267` | **12** | 11 → **12** (`ERROR_AMEND_REJECTED`) |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3430` | **6** | = (the settle's `+0`, confirmed) |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2641` | **25** | = (the settle's `+0`, confirmed — `task amend` takes no id) |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs:2574` | **16** (9 `Address` + 7 `Bare`) | the rc.19 file said 17; **16 is what the symbol carries** at this HEAD, and the block is byte-unchanged in `7d86f99f..51e0b8e4` — the rc.19 count was of `DoctypeArg::` occurrences including one in a doc-comment |
| `SLUG_DOORS` | `crates/cli/src/cli.rs:2881` | **6** | = |
| **`PATH_ARG_OCCURRENCES`** | `crates/cli/src/cli.rs:3189` | **14** occurrences / **18** arms | = (`Cwd 6 · RepoRoot 4 · NotAPath 8`); **no `task amend` row** — correct, the door takes no path |
| `ROLLBACK_POPULATIONS` | `crates/cli/src/rollback.rs:169` | **11** | = (the settle's `+0`, confirmed) |
| `AMBUSH_CONTRACTS` | `crates/cli/src/pack.rs:990` | **6** | 4 → **6**; dispositions **3 `Owed` · 2 `DeclaredWhereReachable` (new, both amend) · 1 `Exempt`** |
| `CONSTRAINT_REQUIRED_TOKENS` | `crates/cli/src/pack.rs:1658` | **8** rows | 6 → **8** |
| `MINT_DOORS` | `crates/engine/src/state.rs:1560` | **6** | 5 → **6** |
| `TASK_AREA_FILES` | `crates/engine/src/state.rs:186` | **15** | 14 → **15** (`AMEND_PIN_FILE`) |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs:757` | **18** | = |
| goldens | `crates/cli/tests/goldens` | **646** | 634 → **646** |

**Axis 5's door set is `ENVELOPE_ARMS` — 66 arms over all 48 leaves.**
Partition read from the registry: **60 `Success` / 3 `Adjudicated` / 2 `Reject`** ·
**`Pinned` 60 / `Unpinned` 6**.

**Cell set** (the M51 acceptance design's Part 2 axis-5 row): `{declared key set == driven key set ·
Unpinned(<reason>) · the four pre-pin deletes absent · the reject funnels (`error` vs findings
envelope) · exit code}` — crossed with the rc.19 run's fifth axis, **the working directory**
`{repo root · a subdirectory · a provisioned fan-out worktree · an ordinary linked worktree · a
spaced repository path}` — and with this run's sixth, **the commit model** `{ordinary · amend}`.

---

## 1 · Baseline rows re-driven on rc.20 — CLOSED / STILL-OPEN

Baseline: `completions/artifacts/M53/per-axis-review-rc19/axis-5.md` (reconciled), which carries one
CLOSED tier-1 row and six STILL-OPEN tier-3 rows. All seven re-driven.

| baseline row | tier | rc.20 verdict | the datum |
|---|---|---|---|
| **`(5, DEFECT 1)`** — a mint door commits a record at a fabricated identity | 1 | **CLOSED (stays closed)** | §2.1 — `milestone create ""` and `task amend ""` both exit 1 `write.unslugable-title`, HEAD unmoved, nothing minted |
| **`(5, DEFECT 2)`** — `config remove-step`/`replace-step` refuse a step that **is** in the resolved include list | 3 | **STILL-OPEN** (expected — triaged to 1.x) | §2.2 |
| **`(5, DEFECT 3)`** — the colon-less-address bail is outside `RefusalKind` and carries no code | 3 | **STILL-OPEN** (expected), still two producers under two wordings | §2.3 |
| **`(5, DEFECT 4)`** — `doc show` blocks a **declared but unpopulated** optional leaf with `store.no-such-leaf` | 3 | **STILL-OPEN** (expected) | §2.4 |
| **`(5, C1)`** — the two pinned orientation rows declare `next_steps`, a reachable composition omits it | 3 | **STILL-OPEN**, driven at **both** arms this run | §2.5 |
| **`(5, D1)`** — `start --explain` emits a production `--format json` stdout arm `ENVELOPE_ARMS` does not carry | 3 | **STILL-OPEN**, and **one key wider** than rc.19 recorded | §2.6 |
| **rc.17 `DEFECT A`** — `store.unknown-type` emits a URI-shaped target at `jigc doc show` | 3 | **STILL-OPEN** | §2.7 |

**Baseline rows: 1 CLOSED · 6 STILL-OPEN, every still-open one tier 3**, which the charter triaged to
1.x — a still-open there is expected and is **not** a new finding. **No row regressed and no closed
row re-opened.**

**And the two rc.19 §A headline rows that were the reason for the usability batch, re-driven here
because they are printed surfaces in a committing path:** `(2, N-1)` / `(6, A6-R1)` (the hook
announcing a rename that is not there) and `(2, N-2)` (the blocking backstop unreachable for the
placement family) are **both CLOSED** — §5.4.

---

## 2 · Repro blocks for §1

### 2.1 — `(5, DEFECT 1)` CLOSED, and the new mint door joins it

```
rig: standalone --git-state unborn, then `jigc setup` (which births HEAD)   cwd = $REPO
$ jigc --version                                 -> jigc 1.0.0-rc.20
$ jigc --format json milestone create ""         -> 1  {"error":"blocking · write.unslugable-title — cannot mint a milestone …"}
$ jigc --format json task amend ""               -> 1  {"error":"blocking · write.unslugable-title — cannot mint a task …"}
$ ls .jigc/tasks                                 -> No such file or directory
$ git status --porcelain                         -> (empty)   HEAD unmoved
```

### 2.2 — `(5, DEFECT 2)` STILL-OPEN

```
rig: committed-singletons (virgin)               cwd = $REPO
$ printf 'id: probe-step\ntitle: Probe\nbody: |\n  Probe.\n' > $RIG/probe-step.yaml
$ jigc --format json config insert-step --workflow single-task --before finalize $RIG/probe-step.yaml
  -> 0 {anchor,committed,op,side,step,workflow}     step: "probe-step"
$ jigc --format json start --explain --workflow single-task | jq -r '.steps[].id'
  locate implement record-changelog superseded-context author-commit newstep PROBE-STEP finalize
$ jigc --format json config remove-step 'workflow:single-task#probe-step'                 -> 1
  {"error":"blocking · config.anchor-absent — no step `probe-step` body to fork
            route: name a step id present in the workflow's resolved include list, then re-run"}
$ jigc --format json config replace-step 'workflow:single-task#probe-step' $RIG/probe-step.yaml -> 1
  {"error":"blocking · config.step-id-collision — `probe-step` is already a step id …"}
control (a PACK step, same door, same rig):
$ jigc --format json config remove-step 'workflow:single-task#implement'
  -> 0 {"committed":false,"op":"config-remove-step","target":"workflow:single-task#implement"}
```

### 2.3 — `(5, DEFECT 3)` STILL-OPEN, still two producers

```
rig: committed-singletons
$ jigc --format json rename vision --to "New Vision"                                      -> 1
  {"error":"`vision` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`
            route: run `jigc describe` for the doctype surface"}          <- no code, no key
$ jigc --format json doc show nosuchtype                                                  -> 1
  {"error":"malformed address `nosuchtype`: missing ':' between type and slug — …"}       <- no code
```

### 2.4 — `(5, DEFECT 4)` STILL-OPEN

```
rig: committed-singletons
$ jigc doc schema vision --format json | jq '.fields[]|select(.id=="grounded-in")'
  {"id":"grounded-in","type":"ref","to":"research","required":false,"author-required":false,
   "section":"meta","set-field":"vision:<slug>#meta/grounded-in"}
$ jigc --format json doc show 'vision:vision#meta/grounded-in'                            -> 1
  findings[0].key {"code":"store.no-such-leaf","target":"vision:vision#meta/grounded-in"}
  "`vision:vision#meta/grounded-in` names no leaf `grounded-in` in section `meta`"
```

### 2.5 — `(5, C1)` STILL-OPEN, driven at BOTH arms

```
rig: fresh --pack-from-dev   (JIGC_PACK_DIR = a throwaway dev-pack copy)
CONTROL, the populated arm:
$ jigc --format json start  -> 0  keys ['header','next_steps','schema_version','state','tasks','workflows']
  next_steps [{"id":"ingest-existing","gist":"bring an existing repo's docs under management"}]
$ STASH=$(mktemp -d "${TMPDIR:-/tmp}/packstash.XXXXXX")
$ mv "$JIGC_PACK_DIR/workflows/ingest-existing.yaml" "$STASH/"    # a MOVE, restored after
CLEAN arm:
$ jigc --format json start  -> 0  driven ['header','schema_version','state','workflows']  state=clean
  DECLARED (render.rs) ['header','next_steps','schema_version','state','workflows']    has next_steps: False
ACTIVE arm (same pack-set, one task minted):
$ jigc --format json start  -> 0  driven ['header','schema_version','state','tasks','workflows']
  DECLARED ['header','next_steps','schema_version','state','tasks','workflows']         has next_steps: False
$ mv "$STASH/ingest-existing.yaml" "$JIGC_PACK_DIR/workflows/"    # restored; pack-load clean
```

### 2.6 — `(5, D1)` STILL-OPEN, one key wider than rc.19

```
rig: committed-singletons, after `jigc config set placement-root myroot`
$ jigc --format json start --explain > o 2> e; rc=$?
  rc=0   stdout 1061 bytes   stderr 0 bytes
  keys ['collision_winners','overrides_applied','pack_inputs','scalar_overrides','schema_version',
        'steps','workflow','workflow_layer']                       <- `scalar_overrides` is new here
$ jigc --format json start --explain --workflow single-task        -> the same eight keys
ENVELOPE_ARMS' four `start` rows: OrientationView::{UnsetProject,Clean,ActiveTask} + Composed. None is it.
```
The added key is not a regression — it is the arm's own optional member surfacing once a scalar
override exists. It is recorded because it sharpens the row: the **undeclared** arm's key set is not
even fixed.

### 2.7 — rc.17 `DEFECT A` STILL-OPEN

```
rig: committed-singletons
$ jigc --format json doc show 'nosuchtype:x'   -> findings[0].key {"code":"store.unknown-type","target":"nosuchtype:x"}
$ jigc --format json doc schema nosuchtype     -> findings[0].key {"code":"store.unknown-type","target":"nosuchtype"}
```

---

## 3 · Row set A — `ENVELOPE_ARMS`, all 66 rows, driven arm by arm

Each arm driven with `--format json`, its key set compared against the declared
`ArmShape::…` **extracted mechanically** from `render.rs:6572` (not transcribed), and the
**stream** recorded (which stream carried the document, and how many bytes the other carried).

| # | leaf | arm | exit | driven key set / shape | verdict |
|---|---|---|---|---|---|
| 1 | `start` | `OrientationView::UnsetProject` | 0 | `schema_version, state` | = declared |
| 2 | `start` | `OrientationView::Clean` | 0 | `header, next_steps, schema_version, state, workflows` | = declared |
| 3 | `start` | `OrientationView::ActiveTask` | 0 | `header, next_steps, schema_version, state, tasks, workflows` | = declared |
| 4 | `start` | `Composed` | 0 | `task, text` | = declared |
| 5 | `workflow` | `Composed` (`--preview`) | 0 | `task, text` | = declared |
| 6 | `setup` | `Installed` | 0 | `allowlist_file, findings, guide_file, hook_committed, hook_file, install_commit, line_file` | = declared; **no `installed` key** |
| 7 | `uninstall` | `TornDown` | 0 | `allowlist_file, findings, line_file, removed` | = declared; **no `uninstalled` key** |
| 8 | `upgrade` | `Swept` | 0 | `checked, findings, guide, schema_version` | = declared |
| 9 | `ingest` | `Triaged` | 0 | `findings, rows, summary` | = declared |
| 10 | `migrate` | `Composed` | 0 | `task, text` (stderr 428 B plain) | = declared |
| 11 | `migrate-corpus` | `Report` (`--dry-run`) | 0 | `already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled` | = declared |
| 12 | `unmanage` | `Report` | 0 | `dropped, identity, path` | = declared |
| 13 | `rename` | `Report` | 0 | `commit, findings, from, hook_output, new_path, old_path, prose_mentions, referrers, title, to` | = declared |
| 14 | `relocate` | `Report` | 0 | `blocked, displaced, moved` | = declared (§5.5) |
| 15 | `describe` | `Menu` *(Unpinned)* | 0 | `commands, definitions, schema_version` | `still_pinned ["schema_version"]` on the wire |
| 16 | `validate` | `StoreSweep` | 0 | `blocking_probes, findings, report_only, schema_version, scope` | = declared |
| 17 | `doc create` | `DocAck::Created` | 0 | `copied_in, existed, findings, op, target` | = declared |
| 18 | `doc add-item` | `DocAck::AddedItem` | 0 | `copied_in, findings, op, target` | = declared |
| 19 | `doc remove-item` | `DocAck::RemovedItem` | 0 | `copied_in, findings, op, removed, target` | = declared |
| 20 | `doc retitle-item` | `DocAck::RetitledItem` | 0 | `copied_in, findings, op, target, title` | = declared |
| 21 | `doc rename` | `DocAck::Renamed` | 0 | `committed_identity, copied_in, findings, from, op, reslugged, target, title` | = declared |
| 22 | `doc set-field` | `DocAck::Field` | 0 | `copied_in, findings, op, target, value` | = declared |
| 23 | `doc set-field` | `DocAck::UnsetField` | 0 | `already_absent, copied_in, findings, op, target, unset` | = declared |
| 24 | `doc set-slot` | `DocAck::Slot` | 0 | `chars, copied_in, findings, op, target` | = declared |
| 25 | `doc author` | `DocAck::Authored` | 0 | `copied_in, findings, op, target` | = declared |
| 26 | `doc show` | `WholeDoc::Committed` | 0 | `fields, item-count, schema-version, sections, slug, type` | = declared |
| 27 | `doc show` | `WholeDoc::Staged` | 0 | + `staged` | = declared |
| 28 | `doc show` | `FieldsGroupSlice` | 0 | `DataKeyed` — `{date, schema-version, status}` | shape = declared |
| 29 | `doc show` | `SlotSlice` | 0 | `Scalar` — `""` | shape = declared |
| 30 | `doc show` | `ItemArraySlice` | 0 | `ArrayOfDataKeyed` — `[{changes,date,id,link,title}]` | shape = declared |
| 31 | `doc show` | `ItemSlice` | 0 | `DataKeyed` — `{changes,date,id,link,title}` | shape = declared |
| 32 | `doc show` | `ListFieldSlice` | 0 | `ArrayOfScalars` — `["research:context-loss"]` | shape = declared |
| 33 | `doc show` | `CompoundFieldSlice` | — | **NOT DRIVEN — unreachable** (§8.1) | — |
| 34 | `doc schema` | `Projection` | 0 | `contract-version, fields, home, identity, schema-version, sections, type` (**contract-version 7**, unmoved) | = declared |
| 35 | `doc list` | `Index` | 0 | `docs` (stderr 170 B plain `note:`) | = declared |
| 36 | `task list` | `Rows` | 0 | `ArrayOf` — `[{id,intent,workflow}]` | = declared |
| 37 | `task diff` | `Ack` | 0 | `base, code_diff, findings, op, staged_docs, task` | = declared — driven on an **amend** task too (§4.10) |
| 38 | `task validate` | `Report` | 3 | `findings, schema_version` | = declared |
| **39** | **`task amend`** | **`Composed`** *(new)* | 0 | **`task, text`** | **= declared** (§4.2) |
| 40 | `task discard` | `TaskAck::Discarded` | 0 | `commit, dropped, findings, op, task` | = declared |
| 41 | `task bind` | `TaskAck::Bound` | 0 | `findings, op, role, target, task` | = declared — **driven at success this run** (rc.19 could not) |
| 42 | `task finalize` | `Landed` | 0 | `committed, findings, schema_version` | = declared |
| **43** | **`task finalize`** | **`LandedAmend`** *(new)* | 0 | **`committed, findings, schema_version`**, `committed.amended` present | **= declared** (§4.6) |
| 44 | `task finalize` | `Forecast` | 0 | `dry_run, findings, left_out, manifest, subject` | = declared |
| 45 | `task finalize` | `Blocked` | 3 | `findings, schema_version` | = declared |
| 46 | `task finalize` | `MigrationReviewHold` | **4** | `retires, rewrites, source, task` | = declared; `retires` repo-relative |
| 47 | `config set` | `ConfigAck::Set` | 0 | `committed, key, op, relocated, value` | = declared |
| 48 | `config insert-step` | `InsertStep` | 0 | `anchor, committed, op, side, step, workflow` | = declared |
| 49 | `config replace-step` | `ReplaceStep` | 0 | `committed, op, step, target` | = declared |
| 50 | `config remove-step` | `RemoveStep` | 0 | `committed, op, target` | = declared |
| 51 | `config fill` | `Fill` | 0 | `committed, op, target` | = declared |
| 52 | `config fork` | `Fork` | 0 | `base, committed, op, path, target` | = declared |
| 53 | `config get` | `Reading` | 0 | `key, layer, op, rejected, value` | = declared |
| 54 | `config list` | `Readings` | 0 | `knobs, op` | = declared |
| 55 | `milestone create` | `RecordOnlyAck` *(Unpinned)* | 0 | `hook_output, text` | = declared |
| 56 | `milestone add-task` | `RecordOnlyAck` | 0 | `hook_output, text` | = declared |
| 57 | `milestone add-from-spec` | `RecordOnlyAck` | 0 | `hook_output, text` | = declared — **driven at success this run** (rc.19 could not) |
| 58 | `milestone provision` | `RecordOnlyAck` | 0 | `hook_output, text` | = declared |
| 59 | `milestone discard` | `RecordOnlyAck` | 0 | `hook_output, text` | = declared |
| 60 | `milestone list-tasks` | `Listing` *(Unpinned)* | 0 | `text` — **no `hook_output`** | = declared (the pre-pin delete) |
| 61 | `milestone execute` | `Composed` | 0 | `task, text` | = declared |
| 62 | `milestone join` | `Report` | 0 | `findings, milestone, no_docs_from, overlay, schema_version` | = declared |
| 63 | `milestone finalize` | `Landed` | 0 | `committed` | = declared |
| 64 | `milestone finalize` | `Blocked` | 3 | `findings, schema_version` | = declared |
| 65 | *(cross-cutting)* | `Reject::Error` | 1 | `error` | = declared — 94 of the 96 hostile-cwd cells (§6) |
| 66 | *(cross-cutting)* | `Reject::Findings` | 1/3 | `findings, schema_version` | = declared |

**65 of 66 arms driven with their key set captured; 65 / 65 declared == driven. 0 mismatches. 0
undeclared shapes.** Row 33 is the one not driven, unreachable at HEAD, reason at §8.1.
**Stream discipline holds on every driven row**: the JSON-bearing stream parses as exactly one
document and the other stream carries no JSON — the only non-empty other-stream bytes seen were
plain `note:` / advisory text on stderr beside a stdout document (rows 10, 30, 31, 35), which is
what `command-output-contract.md` → *Stream discipline* pins.
**The four pre-pin deletes are absent: 4 / 4** — `setup.installed` (row 6), `uninstall.uninstalled`
(row 7), the review hold's `review` (row 46), `milestone list-tasks`' `hook_output` (row 60).
**`schema_version ⇔ ArmRoot::ResultContract` holds on every driven row.**

---

## 4 · Row set B — F-10 `jigc task amend`, probed hardest

This is what this run adds. Every cell here is a `(door, cell)` row on the new capability.

### 4.1 — the mint door's own contract, and the `amend` marker

```
rig: fresh; one task finalized so HEAD is a single-parent jigc commit (9b53804)
$ jigc task amend "repair the summary"                                         -> rc=0
  task minted: repair-the-summary
  amending: 9b53804 "feat: first repair subject"
    `jigc task finalize repair-the-summary` replaces that message and leaves the commit's tree
    exactly as it is — so this repairs the message, never the change.
    If this commit has already been pushed, amending it rewrites shared history — jigc cannot tell.
$ ls .jigc/tasks/repair-the-summary/
  amend  base.json  docs  intent  staged-snapshot.json  workflow      <- `amend` is the 15th TASK_AREA_FILES member
$ command cat .jigc/tasks/repair-the-summary/amend
  9b538045ae49faf46dbdc9370546ba92f8e9971d                            <- the full pinned sha
$ command cat .jigc/tasks/repair-the-summary/base.json
  {"sha":"9b538045…","short":"9b53804"}                               <- base.json pins the same commit
```
**`BEHALF_DOORS` = `Neither`, confirmed by behaviour** (§4.5), and `VERB_KINDS` = `Write`.

### 4.2 — `ENVELOPE_ARMS` row 39, and the declared bound that the `amending:` block is text-only

```
$ jigc --format json task amend "second repair"        -> rc=0  keys ['task','text']
  'amending:' in text -> False                          <- the block reaches agent/human text only
$ jigc start --task repair-the-summary                 -> rc=0  "amending:" count 1  (MEDIUM-2 fix)
$ jigc workflow amend --task repair-the-summary        -> rc=0  "amending:" count 1  (MEDIUM-2 fix)
$ jigc --format json start --task repair-the-summary   -> rc=0  keys ['task','text']; 'amending:' False
```
The row's own doc-comment declares exactly this (*"no new key … the `amending:` presentation block
reaches agent/human text only"*), so the json arm's omission is **declared, not a divergence**, and
the step body no longer points at "the ack above" — it names a command
(`` `git log -1 --format=%s` ``). **MEDIUM-2 closed at both re-compose doors on the text arm.**

### 4.3 — `amend.head-shape`, all three shapes × both formats × the LOW-5 locus

```
rig: standalone --git-state unborn (+ `jigc setup`, which births a ROOT HEAD), then a merge built by hand
UNBORN:
$ jigc --format json task amend "unborn probe"   -> 1
  {"error":"blocking · amend.head-shape — … HEAD is unborn — this repository has no commit yet — nothing was minted
            at: work-unit:unborn-probe
            route: there is no commit to repair; make one first — `jigc start \"<intent>\"` …"}
ROOT (with an intent — the LOW-5 cell):
$ jigc task amend "root probe"                   -> 1   at: work-unit:root-probe      <- the id the mint WOULD take
ROOT (no intent):
$ jigc task amend                                -> 1   at: work-unit:amend-76d6bd1   <- the sha fallback
MERGE (2 parents):
$ jigc task amend "merge probe"                  -> 1   "HEAD is a merge commit — it has 2 parents"
ALL FOUR: `ls .jigc/tasks` -> No such file or directory   ·  `git status --porcelain` -> (empty)
```
**LOW-5 CLOSED.** All four refuse **before any mint**, text and `--format json` byte-identical
(the json arm is the declared `Reject::Error` row, flattened).

### 4.4 — `finalize.amend-index-dirty` over the whole index axis

```
rig: fresh; amend task `repair-the-summary`, commit doc authored, HEAD 9b53804
each cell:  <stage the shape>;  jigc --format json task finalize repair-the-summary > so 2> se; rc=$?
  ADD      (git add added.txt)      -> rc=3  stdout {findings,schema_version}  key {finalize.amend-index-dirty, "added.txt"}
  MODIFY   (git add code.txt)       -> rc=3  key {finalize.amend-index-dirty, "code.txt"}
  DELETE   (git rm code.txt)        -> rc=3  key {finalize.amend-index-dirty, "code.txt"}
  RENAME   (git mv code renamed)    -> rc=3  key {finalize.amend-index-dirty, "renamed.txt"}
  TWO PATHS(git add p1 p2)          -> rc=3  TWO findings, one per path: p1.txt, p2.txt
AFTER EVERY CELL: git rev-parse --short HEAD -> 9b53804 (unmoved)
the route, in full:
  "unstage it (`git -C <abs repo> restore --staged -- added.txt`) and re-run the finalize —
   this arm takes no `--carry-staged`, because an amend that carried anything would change a tree
   it promised not to touch"
```

### 4.5 — the repository-posture family: the mint door adjudicates none of it, the finalize arm all of it

`InProgress::ALL` (10 members) + `detached` + `squash-merge`, each a fresh
`dev/jigc-rig fresh --git-state <m>`; in each, `jigc task amend` then the authored finalize:

```
member              mint rc  minted id       finalize rc  code                         HEAD
merge                 0      posture-probe        1       repo.operation-in-progress   unmoved
squash-merge          0      posture-probe        1       repo.operation-in-progress   unmoved
rebase-merge          0      posture-probe        1       repo.operation-in-progress   unmoved
rebase-apply          0      posture-probe        1       repo.operation-in-progress   unmoved
am                    0      posture-probe        1       repo.operation-in-progress   unmoved
cherry-pick           0      posture-probe        1       repo.operation-in-progress   unmoved
revert                0      posture-probe        1       repo.operation-in-progress   unmoved
sequencer             0      posture-probe        1       repo.operation-in-progress   unmoved
bisect                0      posture-probe        1       repo.operation-in-progress   unmoved
uncommitted-pick      0      posture-probe        1       repo.operation-in-progress   unmoved
unmerged-index        0      posture-probe        1       repo.operation-in-progress   unmoved
detached              0      posture-probe        1       repo.head-detached           unmoved
```
**12 / 12 at both doors.** `design/finalize.md` → The amend arm records this correctly (the mint
door is `Neither`), and **LOW-7's first half is closed at the design home**. The finalize arm's
refusal shape is **byte-identical to the ordinary arm's** — control, `rig fresh --start single-task
--git-state bisect`: `{"error":"blocking · repo.operation-in-progress — a bisect is in progress …"}`,
rc=1, stdout 0 bytes. So the flattened arm here is the door's pre-existing shape, not the amend arm's.

### 4.6 — the happy path, and everything the rewrite does and does not move

```
rig: committed-singletons; an adr landed at a710668 ("docs: record the single-node cache decision")
$ jigc task amend "rename probe"; commit doc authored (type=docs, summary, body)
BEFORE: H0=a710668  TREE0=df52398  P0=16f7ed6  AD0=Sun Sep 27 01:43:53  CD0=Sun Sep 27 01:43:53
$ jigc task finalize rename-probe                                              -> rc=0
  finalize — about to rewrite HEAD's message; the committed tree does not move, so it leaves out:
    left-out (unstaged/untracked — an amend commits no tree change, so none of it can join):
      README.md / untracked.txt
  no findings — the task validates clean
  amended a710668 → bdd6142 — docs: record the cache decision with a clearer subject
    the tree and the author are unchanged; the message is re-authored and the committer becomes you, now
    the superseded commit stays reachable in the reflog
AFTER:  TREE same? YES   PARENT same? YES   commit count 6 (unmoved)
        AD unchanged (01:43:53)   CD moved (01:45:12)            <- LOW-8's committer clause, true
$ git reflog | head -2
  bdd6142 HEAD@{0}: commit (amend): docs: record the cache decision with a clearer subject
  a710668 HEAD@{1}: commit: docs: record the single-node cache decision
$ ls .jigc/tasks ; jigc task list        -> area torn down; no active tasks
$ jigc validate                          -> rc=0 (the promoted-doc control: file-state valid after)
```
**MEDIUM-3 CLOSED**: the left-out header on this arm reads *"an amend commits no tree change, so none
of it can join"*, not *"git add to include"* — the clause the same binary would then refuse. And the
pre-commit stem reads *"about to rewrite HEAD's message; the committed tree does not move"*.
**LOW-8 CLOSED**: the committer sentence rides both the forecast and the landed ack.

### 4.7 — row 43's envelope, driven from a subdirectory

```
rig: committed-singletons + docs/deep;  cwd = $REPO/docs/deep
$ jigc --format json task amend "json land probe"            -> 0 {task,text}
  … commit doc authored …
$ jigc --format json task finalize json-land-probe > o 2> e; rc=$?
  rc=0   stdout 303 bytes   stderr 0 bytes
  top-level ['committed','findings','schema_version']                     <- = declared
  committed  ['amended','displaced','files','hash','hook_output','left_out','manifest','promoted','subject']
  amended = 8750171   hash = 69c2baa   files = 0   displaced = []
  HEAD 8750171 -> 69c2baa
```

### 4.8 — `finalize.base-mismatch` on a moved HEAD, and its route run verbatim

```
$ jigc task amend "moved head probe"; commit doc authored;  pinned 69c2baa
$ echo drift > drift.txt; git add drift.txt; git commit -m "feat: unrelated drift"   # HEAD -> c7e3cb4
$ jigc --format json task finalize moved-head-probe                            -> rc=3
  keys ['findings','schema_version']
  key {"code":"finalize.base-mismatch","target":"task:moved-head-probe"}
  "`HEAD` is no longer the commit this amend was minted against — it pinned `69c2baa…` and `HEAD`
   is now `c7e3cb4…`, so the message this task authored would rewrite a different commit"
  route: "start the repair again against the commit that is there now
          (`jigc task discard moved-head-probe --force`, then `jigc task amend`), or return `HEAD` to `69c2baa…` first"
$ git rev-parse --short HEAD   -> c7e3cb4 (unmoved by the refusal)
THE EMITTED ROUTE, RUN VERBATIM: jigc task discard moved-head-probe --force    -> rc=0
```

### 4.9 — the hook-rejected cell: HEAD byte-identical, and `finalize.amend-rejected` in the log

```
rig: committed-singletons; `jigc config set invocation-log true`
$ printf '#!/bin/sh\necho "commit-msg hook: refusing" >&2\nexit 1\n' > .git/hooks/commit-msg; chmod +x
$ jigc task amend "hook rejected probe"; commit doc authored
BEFORE: H0=c7e3cb4…  raw-object digest 82b01ec9724d
$ jigc --format json task finalize hook-rejected-probe > o 2> e; rc=$?
  rc=1   stdout 0 bytes   stderr 801 bytes (the findings envelope)
  key {"code":"finalize.amend-rejected","target":"task:hook-rejected-probe"}
  message "`git commit` was rejected (no commit was made):\ncommit-msg hook: refusing"
  route   "`HEAD` is unchanged — the commit this task is repairing still carries the message it had,
           and task hook-rejected-probe's authored commit doc is still in `.jigc/tasks/…/docs/`.
           Fix the hook's complaint, then re-run `jigc task finalize hook-rejected-probe`."
AFTER:  H1=c7e3cb4…  raw-object digest 82b01ec9724d   -> IDENTICAL
        jigc task list -> the task is still open and re-runnable
$ command cat .jigc/logs/invocations.jsonl | (the 5th record)
  {"argv":["--format","json","task","finalize","hook-rejected-probe"],"exit_code":1,
   "finding_codes":[],"output_bytes":801,"binary_version":"1.0.0-rc.20",
   "error_code":"finalize.amend-rejected"}
```
The `COMMITTING_DOORS` 11th row's **error identity reaches the log**, on `error_code` — the
route-exempt slot `ERROR_CODE_REGISTRY` declares — with `finding_codes: []`, which is that
registry's shape, not a dropped finding. The survivable frame's **state-truth clause for this arm
(*HEAD is unchanged*) is true and is in the route**.

### 4.10 — the previews, `--carry-staged`, and the `--help` truth

```
$ jigc task validate rename-probe        (clean index)  -> rc=0  "no findings — the task validates clean"
$ echo x > staged.txt; git add staged.txt
$ jigc task validate rename-probe        (dirty index)  -> rc=3  finalize.amend-index-dirty @ staged.txt
$ jigc task finalize rename-probe --dry-run (dirty)     -> rc=3  the same finding, same route
$ jigc task finalize rename-probe --carry-staged (dirty)-> rc=3  the same finding  (inert, as declared)
$ jigc task finalize rename-probe --dry-run (clean, dirty TREE) -> rc=0
  would rewrite a710668 "…" → "docs: record the cache decision with a clearer subject"
    the tree and the author are unchanged; the message is re-authored and the committer becomes you, now
    left-out (unstaged/untracked — an amend commits no tree change, so none of it can join): …
`jigc task finalize --help`:
  "On a task minted by `jigc task amend` it takes its **second commit model**: it rewrites the commit
   at `HEAD` with `git commit --amend`, leaving its tree untouched … there is no flag that declares
   that carry-over deliberate, so on that arm the index gate above is this refusal and not the
   carryover gate."                                                           <- the 11th COMMITTING_DOORS row's `commits:` clause
  --dry-run  "… On an **amend** task the last two read differently … the index gate there is the arm's
              own `finalize.amend-index-dirty` over *any* staged path, which `--carry-staged` cannot forecast past."
  --carry-staged "… inert on an **amend** task in every state …"
other amend-task surfaces:
$ jigc --format json task diff also-open-probe  -> 0 {base,code_diff,findings,op,staged_docs,task}
   staged_docs [{"id":"commit:also-open-probe"}]   base {"sha":"e6e5b9e…","short":"e6e5b9e"}
$ jigc --format json doc list --task also-open-probe -> 0 {"docs":[{"id":"commit:also-open-probe", …}]}
$ jigc --format json start --workflow amend "x"      -> 1  key {workflow.verb-routed, "workflow:amend"}
$ jigc --format json workflow amend --preview        -> 1  the same key
$ jigc describe --format json | (the `amend` definition)
   "router_hidden": "verb-routed — reached only through `jigc task amend`, which pins the commit at
    HEAD and provisions the empty commit doc the amend renders; a router pick would compose with no
    pinned commit and finalize would have nothing to rewrite."
$ jigc task amend "also open probe"  (2 sibling tasks open) -> 0, and the ack carries:
   "also open: 2 other tasks were already open before this call — nothing here touched them …"
```

### 4.11 — HIGH-1's class: `finalize.amend-staged-doc` over all 8 `doc` write leaves

Class derived from the code: `command grep '(&\["doc", "…"\], VerbKind::Write)'` → **8** leaves
(`create · add-item · remove-item · retitle-item · rename · set-field · set-slot · author`).

```
rig: committed-singletons; `jigc task amend "doc edit probe"` (and a second amend task for the
     committed non-singleton cells, over a landed `adr:single-node-cache`)
after EVERY cell:  git status --porcelain -> (empty)   ·   ls .jigc/tasks/<T>/docs -> commit:<T>.md provenance.json

leaf              argv                                                        rc  answer
doc set-slot      doc set-slot vision:vision#thesis --from-file - --task T     1   finalize.amend-staged-doc @ VISION.md
doc add-item      doc add-item changelog:changelog#releases --title R --task T 1   finalize.amend-staged-doc @ CHANGELOG.md
doc remove-item   doc remove-item changelog:…#releases/0-1-0 --task T          1   finalize.amend-staged-doc @ CHANGELOG.md
doc retitle-item  doc retitle-item roadmap:roadmap#milestones/m-alpha …        1   finalize.amend-staged-doc @ docs/roadmap.md
doc set-field     doc set-field changelog:…#releases/0-1-0/date --value …      1   finalize.amend-staged-doc @ CHANGELOG.md
doc rename        doc rename adr:single-node-cache --to "Single Node Cache"    1   finalize.amend-staged-doc @ docs/decisions/single-node-cache.md
doc rename        doc rename adr:single-node-cache --to "Multi Node Cache"     1   finalize.amend-staged-doc @ docs/decisions/single-node-cache.md   <- the RE-SLUG cell the review found staging first
doc create        doc create adr --title "Some Decision" --task T              1   create.gate-blocked @ adr        (pre-empted)
doc author        doc author vision --from-file <conforming payload> --task T  1   create.gate-blocked @ vision     (pre-empted)
CONTROL, the transient commit doc: doc set-field commit:T#header/type --value docs --task T -> 0 {copied_in,findings,op,target,value}
CONTROL, an ORDINARY task: doc author vision … -> create.gate-blocked; doc rename vision:vision --to X -> write.identity-change
  (both identical to the amend cells, so the two pre-emptions are pre-existing and not amend-specific)
the message, in full:
  "`vision:vision` is a managed doc, and it promotes to `VISION.md` — but this task's commit model is
   an amend, which changes no tree: its finalize would write `VISION.md` into the worktree and commit
   none of it, leaving the file diverged from the commit it just rewrote"
  route: "`jigc start \"<intent>\"` mints an ordinary task, whose finalize commits the promoted doc;
          this amend task keeps its own job, repairing `HEAD`'s message"
```
**HIGH-1 CLOSED over its whole 8-leaf axis.** Six leaves answer at the new code, two are pre-empted
by a shipped refusal that also stages nothing, and in **every** cell the worktree is clean and the
task area holds only its own transient commit doc. The exit-0 divergence the review drove (`git
status` ` M VISION.md`, `git show HEAD:VISION.md` stale, `jigc validate` exit 0) **does not
reproduce**.

### 4.12 — the shell-hostile cells: a spaced repository root **and** a spaced staged path

```
rig: committed-singletons under  …/scratchpad/axis-review-rc20/jigc space.XqCcR0/…/repo
cwd = $REPO/docs/deep
$ jigc task amend "spaced probe"                                        -> rc=0, mint ack correct
$ echo s > "my file.txt"; git add "my file.txt"
$ jigc task finalize spaced-probe                                       -> rc=3
  blocking · finalize.amend-index-dirty — `docs/deep/my file.txt` is staged …
    at: docs/deep/my file.txt                                            <- law 1: repo-relative locus
    route: unstage it (`git -C '/…/jigc space.XqCcR0/…/repo' restore --staged -- 'docs/deep/my file.txt'`) …
$ <the emitted git span, taken verbatim, run from docs/deep>            -> rc=0
  git status --porcelain -> ?? "docs/deep/my file.txt"                   (the right index changed)
$ jigc --format json task finalize spaced-probe                          -> rc=0
  top ['committed','findings','schema_version']   amended 2c4e4ff  hash 2025030  displaced []
  TREE same? YES
$ jigc validate                                                          -> rc=0 (1 store advisory, unrelated)
```
**Both operands single-quoted, the repo root and the path, and the span runs.**

### 4.13 — the `AMBUSH_CONTRACTS` third disposition, and the named-fact fence it buys

```
registry, read by symbol: 6 rows — 3 Owed · 2 DeclaredWhereReachable (finalize.amend-index-dirty,
finalize.amend-staged-doc; declarer crates/cli/pack/steps/amend-message.yaml) · 1 Exempt
CONSTRAINT_REQUIRED_TOKENS: 8 rows, the two new ones carrying 3 tokens each.

POSITIVE half of the DeclaredWhereReachable reason ("a methodology-alone composition cannot mint
at all, so a declarer there would bind on nothing"):
$ M=$(mktemp -d …); cp -R packs/methodology/. "$M/"
$ JIGC_PACK_DIR="$M" jigc describe > /dev/null 2> err; rc=$?     -> rc=0   err empty
$ (the shipped dev+methodology pair, no JIGC_PACK_DIR)           -> rc=0, 51 definitions

THE FENCE, driven token by token on a MANIFEST-KEEPING pack copy (`dev/jigc-rig fresh --repin`);
each round restores the file from a `mktemp -d` stash first, so exactly one token is removed:
  control (unedited)         -> pack-load rc=0
  "fold the whole index"     -> rc=1  pack-load named-fact fence failed: step `amend-message` declares
                                      `finalize.amend-index-dirty` but its prose never says "fold the whole index"
  "non-empty index"          -> rc=1  … `finalize.amend-index-dirty` … never says "non-empty index"
  "no flag"                  -> rc=1  … `finalize.amend-index-dirty` … never says "no flag"
  "exactly one doc"          -> rc=1  … `finalize.amend-staged-doc` … never says "exactly one doc"
  "promotes"                 -> rc=1  … `finalize.amend-staged-doc` … never says "promotes"
  "ordinary task"            -> rc=1  … `finalize.amend-staged-doc` … never says "ordinary task"
  restored                   -> rc=0
AND the composed step body states all six (driven on `jigc start --task <amend-id>`; each token count ≥1).
```
**MEDIUM-4 CLOSED: 6 / 6 tokens buy their facts.** *Recorded, and it is the fence's own declared
bound rather than a defect:* on a **manifest-less** pack (`dev/jigc-rig fresh --pack-from-dev`, which
drops the freeze manifest) the fence **skips entirely** — all six tokens removed, six doors driven
(`describe · start · validate · doc list · task list · config list`), every one exit 0. That is
`assert_named_facts_stated`'s stated skip-on-absent scope, driven rather than assumed.

### 4.14 — four cwds at the mint door, and LOW-7's checkout sentence

```
rig: fresh + docs/deep + a linked worktree (branch feat, its own commit) + milestone cwd-wave
     provisioned (fan-out worktree .jigc/worktrees/area-one, DETACHED at the milestone base)
argv: jigc task amend "cwd probe"     (the task discarded between cwds)
  (a) $REPO                 -> 0  amending: f2a8c3f "chore(milestone): record task:area-one …"   (main HEAD)
  (b) $REPO/docs/deep       -> 0  the identical line                                             (main HEAD)
  (c) .jigc/worktrees/area-one -> 0  amending: 5505877 "chore: deep dir"
        + "that is the `HEAD` of the linked worktree at `.jigc/worktrees/area-one` — not of the main
           checkout jigc's workbench binds to"                        <- repo-relative, law 1
  (d) $RIG/linked           -> 0  amending: 5634192 "feat: worktree own commit"
        + "that is the `HEAD` of the linked worktree at `/…/linked` on branch `feat` — not of the main
           checkout jigc's workbench binds to"
```
**LOW-7's second half CLOSED**: `AmendTarget.checkout` rides the **mint**, and the amend pins the
**standing** checkout's HEAD (the C2-09 rule), named at the moment of pinning rather than after the
fact.

### 4.15 — the amend marker is jigc's own byte, not a foreign one

```
rig: committed-singletons;  jigc task amend "marker probe"; commit doc authored
$ ls .jigc/tasks/marker-probe        -> amend base.json docs intent staged-snapshot.json workflow
$ jigc --format json task finalize marker-probe   -> rc=0
  committed.displaced = []        committed.amended = 1fa14ae
$ ls .jigc/displaced                -> (only `displace-probe`, from the unrelated §5.6 probe)
```
The 15th `TASK_AREA_FILES` member is **not** classified foreign by the displacing door — the
complement rule holds with the new member in it.

---

## 5 · Row set C — the usability batch's surfaces that bear on this axis

### 5.1 — `PATH_ARG_OCCURRENCES`: the registry is byte-unchanged, and the two bases that discriminate were re-driven

`git diff 7d86f99f..51e0b8e4 -- crates/cli/src/cli.rs` touches **no** `PathArgOccurrence`,
`PathArgBase`, `DOCTYPE_DOORS`, `SLUG_DOORS` or `WORK_UNIT_ID_DOORS` line, so the rc.19 run's
**18 / 18** stands. Re-driven here from a subdirectory, because a changed **verb** can still change
an unchanged base's behaviour:

```
rig: committed-singletons + docs/deep + a committed docs/deep/foreign.md;  cwd = $REPO/docs/deep
  migrate `path`, base = Cwd
    jigc --format json migrate foreign.md --as adr            -> 0  task minted
    jigc --format json migrate docs/deep/foreign.md --as adr  -> 1  "could not read the foreign `adr`
                                                                     source at `docs/deep/foreign.md`"
  config set `value`, key ∈ ROOT_KNOBS, base = RepoRoot (a HOME)
    jigc --format json config set placement-root myroot       -> 0
    ls $REPO/myroot -> exists      ·      ls $REPO/docs/deep/myroot -> No such file or directory
THE ESCAPE SHAPES of the one base that moved in the cwd arc, all from docs/deep, all rc=1,
all `migrate.source-untrackable`, all refused BEFORE any read:
    ../../../outside.md · <an absolute path outside> · :/CHANGELOG.md · ../../.jigc/AGENT.md · ../../.git/config
```

### 5.2 — `doc schema` contract-version and `doc show`'s top-level `schema-version` are unmoved

```
$ jigc doc schema adr --format json  -> contract-version 7   (M52's number, unmoved)
$ jigc doc show vision --format json -> "schema-version": 1  (the M49 top-level number, unmoved)
```
The wave's stated boundary (*zero contract-version / schema-version movement*) holds at the two
read surfaces that carry a number.

### 5.3 — the F-10 settle's "what does not move" list, checked against the symbols

`ROLLBACK_POPULATIONS` **11** (+0) · `DESTROYING_DOORS` **6** (+0) · `WORK_UNIT_ID_DOORS` **25**
(+0, and `task amend` takes no id) · `PATH_ARG_OCCURRENCES` **14/18** (+0, no `amend` row) ·
`contract-version` **7** (+0). All five claims hold by symbol.

### 5.4 — the rc.19 §A headline rows `(2, N-1)` and `(2, N-2)`, driven

```
rig: committed-singletons
(a) a commit with NO rename:
  $ echo "just a file" > plain.txt; git add plain.txt; git commit -m "chore: an ordinary commit …"
    -> rc=0, and the hook printed NOTHING                                    <- (2, N-1) CLOSED
(b) a real out-of-band rename of a PLACEMENT doc:
  $ git mv VISION.md VISION-OLD.md; git commit -m "bare git mv of a placement doc"      -> rc=1
    jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc
          identity tracking; use `jigc rename` instead (commit blocked).      <- (2, N-2) CLOSED
(c) a real out-of-band rename of a LOCATION doc (a landed adr):
  $ git mv docs/decisions/hook-decision.md docs/decisions/hook-decision-two.md; git commit …  -> rc=1
    the identical blocking sentence
```
Both the false-positive half and the structurally-unreachable half are closed, and the false
sentence that rode the failure (*"not staged in this commit; commit not blocked"*) is gone.

### 5.5 — `relocate`'s pinned Report, driven (rc.19 drove it on a manufactured pack; so does this run, and it says so)

```
rig: fresh --pack-from-dev (the freeze manifest is dropped, so `adr` is freeze-exempt)
  … an adr landed at docs/decisions/relocatable-decision.md, then `git mv`d to docs/old-decisions/ and committed …
$ jigc --format json relocate adr --from docs/old-decisions                 -> rc=0
  {"moved":[["docs/old-decisions/relocatable-decision.md","docs/decisions/relocatable-decision.md"]],
   "blocked":[], "displaced":[]}                                            <- = declared
CONTROL, a frozen doctype on the shipped pack:
$ jigc --format json relocate research --from docs/old-research             -> rc=1
  "blocking · relocate.frozen-doctype — `research` is a frozen doctype — relocate it through the
   version-gated `jigc migrate-corpus`, not the freeze-exempt path"
```

### 5.6 — the displacement surface on a committing door, from a linked worktree (`bd8c1d67`)

```
rig: committed-singletons + a linked worktree (branch feat2);  cwd = $RIG/lw
  … ordinary task `displace-probe`, commit doc authored, zz.txt staged …
$ echo "FOREIGN" > $REPO/.jigc/tasks/displace-probe/foreign-byte.txt
$ jigc --format json task finalize displace-probe                            -> rc=0
  committed.displaced [{"from":".jigc/tasks/displace-probe/foreign-byte.txt",
                        "to":".jigc/displaced/displace-probe/foreign-byte.txt"}]   <- workbench-relative
  stderr: "note: the working area held 1 entry jigc did not write, and removing it would have
           destroyed bytes no commit has a copy of — they were moved aside, not taken:
             .jigc/tasks/displace-probe/foreign-byte.txt → .jigc/displaced/displace-probe/foreign-byte.txt"
$ find $REPO/.jigc/displaced -type f -> the file is there (before-control: the plant existed; after: it survives)
```
**No host-absolute path on either surface, driven from a checkout that is not the workbench's.**

---

## 6 · Row set D — the reject funnels, re-driven whole over **48** leaves

### 6.1 — outside a git repository · **48 / 48 in a declared arm**

```
$ OUT=$(mktemp -d "$SCRATCH/outrepo2.XXXXXX"); export HOME=$OUT/home; cd "$OUT"
$ for each of the 48 VERB_KINDS leaves: jigc --format json <minimal argv>
roll-up:  Reject::Error 46 (exit 1)  ·  Reject::Findings 2 (exit 1)  ·  total 48
NOT-JSON: 0    bare-`Finding` roots: 0    stdout: 0 bytes on all 48    non-1 exits: none
the 2 findings-arm leaves: `setup` (setup.repo-root) · `uninstall` (uninstall.repo-root)
`task amend`, the 48th leaf: Reject::Error, exit 1, stdout empty
```

### 6.2 — the cwd deleted under the process · **48 / 48**

```
$ for each leaf:  sh -c "cd '$D' && rmdir '$D' && exec jigc --format json <argv>"
roll-up:  Reject::Error 48 (exit 1)  ·  NOT-JSON 0  ·  bare-`Finding` roots 0  ·  stdout 0 bytes on all 48
stderr:   {"error": "cannot determine the current directory: No such file or directory (os error 2)"}
```

**96 / 96 hostile-cwd cells land in a declared arm with the new leaf in the set.**

---

## 7 · Row set E — the four-cwd envelope-divergence sweep, re-run with 48 leaves

```
rig: committed-singletons + docs/deep + a linked worktree (feat) + milestone sweep-wave provisioned (zeta-area)
cwds: $REPO · $REPO/docs/deep · $REPO/.jigc/worktrees/zeta-area · $RIG/linked
46 leaves × 4 cwds = 184 cells    (setup and uninstall excluded — destructive; both driven at their own cells)
compared per cell: exit · arm key set · every finding's (code, target) · the route text, host paths normalized
DIVERGENT on the first pass: 8 of 46
```
**All eight are artefacts of the sweep's own ordering, not of the cwd**: the first cwd's call
*succeeded* and mutated the corpus (`doc add-item` minted the item, `task amend` minted the task,
`milestone create` minted the milestone, …), so cwds 2–4 met the post-mutation state. Each of the
eight was **re-driven at a refusal cell that is state-independent**, four cwds each:

```
8 refusing leaves × 4 cwds = 32 cells, DIVERGENT = 0:
  doc add-item nosuchtype:x#s --title R                    4/4 identical
  doc remove-item nosuchtype:x#s/i                         4/4 identical
  task amend ""                                            4/4 identical  write.unslugable-title
  config insert-step --workflow nosuchwf … /dev/null       4/4 identical
  config remove-step workflow:nosuchwf#locate              4/4 identical
  milestone create ""                                      4/4 identical  write.unslugable-title
  milestone add-task nosuchmilestone T                     4/4 identical  {milestone.unknown, milestone:nosuchmilestone}
  milestone discard nosuchmilestone                        4/4 identical  {milestone.unknown, milestone:nosuchmilestone}
```
**46 / 46 leaves byte-identical across all four working directories — 216 cells, 0 real divergences.**
(The rc.19 run's two declared divergences — `config insert-step`/`replace-step`'s workbench-root
guard firing from inside `.jigc/worktrees/` — did not recur because this run's argv for those two
leaves reaches an earlier refusal; that is a difference in the probe, not in the binary, and is
stated rather than presented as a change.)

---

## 8 · Defects

### DEFECT 1 (tier 2) — inside the usability batch's own new code: `jigc task validate` and `jigc start` report a clean repository posture from a provisioned fan-out worktree, where `jigc task finalize` in the same directory refuses

**The contract contradicted, in three locked homes:**
- `design/finalize.md:76` — *"So the preview asks the **committing door's own guard**, with that
  door's own exemptions, and renders the identical finding at the identical exit code."*
- `design/command-output-contract.md:488` — *"`task validate` asks the committing door's own guard
  and renders the identical finding, at the identical exit code: **1**."*
- `crates/cli/src/cli.rs:640-653` (`finalize_posture_refusal`'s doc-comment) — *"What it must **not**
  be is a second answer … so the finding, its route and its exit code are the committing door's,
  byte for byte."*
- and the row this run's own baseline carries: `per-axis-review-rc19/axis-2.md:279` —
  *"standing in a **detached fan-out worktree**, main attached | `task finalize` / `task validate` |
  **refuses** `repo.head-detached`"*.

**Driven, it is not byte for byte, and it is not the same exit — it is silence against a refusal.**

```
rig: fresh + docs/deep + a linked worktree + milestone cwd-wave provisioned (area-one, DETACHED)
setup: cd $REPO/.jigc/worktrees/area-one
       git rev-parse --abbrev-ref HEAD -> HEAD ; git status --porcelain=v2 -b | '# branch.head (detached)'
       jigc start --workflow single-task "ordinary in worktree"      -> 0  (an ORDINARY task, not a sub-task)
       commit doc authored; echo k > k.txt; git add k.txt

argv (all three from that same cwd):
  jigc task validate ordinary-in-worktree            -> exit 0   "advisory · changelog-recording…" only
                                                                  NO repo.* finding
  jigc task finalize ordinary-in-worktree --dry-run  -> exit 0   "would commit — fix: ordinary in a
                                                                  detached worktree
                                                                  would commit in the linked worktree at
                                                                  `.jigc/worktrees/area-one` …"
  jigc task finalize ordinary-in-worktree            -> exit 1
      blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no
        branch, and the next checkout would leave it unreachable
        route: re-attach HEAD with `git switch <branch>`, then re-run this command
  git rev-parse --short HEAD -> 5505877 (unmoved)

THE SAME ON AN AMEND TASK (the new committing arm), same cwd:
  jigc task amend "fanout amend probe"               -> 0, ack names the worktree
  jigc task validate fanout-amend-probe              -> 0   "no findings — the task validates clean"
  jigc task finalize fanout-amend-probe --dry-run    -> 0   "would rewrite 5505877 … would commit in
                                                             the linked worktree at `.jigc/worktrees/area-one`"
  jigc task finalize fanout-amend-probe              -> 1   repo.head-detached

AND THE M53 USABILITY ROW `11fe5941` ("a live task's findings carry the repository posture finalize
refuses under") IS SILENT IN THE SAME CELL:
  from $REPO/.jigc/worktrees/area-one:  jigc --format json start
      every tasks[].findings -> NO repo.head-detached on any of the four live tasks
  from $RIG/det (an ORDINARY linked worktree, git worktree add --detach):  jigc --format json start
      every tasks[].findings -> "blocking repo.head-detached …" on all four            <- the working half

CONTROLS that isolate the cell:
  (a) the STANDING checkout detached (rig fresh --git-state detached, ordinary task):
      task validate -> 1 repo.head-detached   ·   dry-run -> 1   ·   finalize -> 1     (all three agree)
  (b) an ORDINARY LINKED detached worktree ($RIG/det):
      task validate -> 1 repo.head-detached   ·   finalize -> 1                         (both agree)
  (c) the FAN-OUT worktree, with a bisect additionally running in the MAIN checkout:
      from the main root:   task validate ordinary-in-worktree -> 1 repo.operation-in-progress
      from the fan-out wt:  task validate ordinary-in-worktree -> 0 (neither the main's bisect nor
                                                                     the worktree's detached HEAD)
      from the fan-out wt:  task finalize ordinary-in-worktree -> 1 repo.head-detached
```

**Why it happens, stated from the code I read and consistent with every cell above** (the drives are
the evidence; this is the mechanism, marked as a read): the preview layers ask
`cli::posture_breach_in` → `repo::adjudicated_breach`, whose subject is `repo::posture_subject`, and
`PostureSubject::adjudicates` **exempts a `Checkout::Dedicated` from `PostureMember::HeadDetached`**
(`repo.rs:851-864`) — a deliberate carve-out so jigc's own `--detach` provisioning is not refused.
The **commit seam** does not take that carve-out: it re-probes with a typed `SeamSubject` that is
`dedicated` only for the throwaway combine worktree the milestone boundary builds
(`task.rs:7979`), so every other commit is adjudicated `Live` and refuses. Two askers, two answers,
in the one cell where the exemption bites.

**Why it is tier 2 and not tier 1.** Nothing is lost and nothing is committed: the gate holds, HEAD
is unmoved in every refusing cell, and the refusal carries a code and a runnable route. What fails
is the **preview contract** — the surface an agent reads to decide whether to finalize says *nothing
this side of the commit blocks it* over a state the commit door refuses, which is the class the
F-10 review itself graded MEDIUM (*"the preview lied over a dirty index"*), one axis over.

**It is wider than the amend arm** — the ordinary `task finalize` arm has the identical divergence
in the same cwd (driven above), so it is **not** a finding inside F-10's new code. **The `jigc start`
half is inside M53's own new code** (`11fe5941`, the usability batch's row 2), which is the exit
rule's third clause.

**Door:** `task validate` (and `start`, and `task finalize --dry-run`). **Cell:** cwd = a registered,
provisioned fan-out worktree with a detached HEAD, over a task whose commit boundary is
`jigc task finalize`.

---

### DEFECT 2 (tier 3) — `amend.head-shape`'s locus uses a work-unit address spelling no grammar in the product declares

```
$ jigc task amend "root probe"        (HEAD is a root commit)              -> 1
  blocking · amend.head-shape — … nothing was minted
    at: work-unit:root-probe
$ jigc task amend                                                          -> 1
    at: work-unit:amend-76d6bd1
producer: crates/cli/src/start.rs:358  `format!("work-unit:{id_hint}")`   (the only `work-unit:` producer)
```
`design/structural-grammar.md:98` declares the work-unit family's address form as `type:name` —
*"`task:add-rate-limiter`, `milestone:m1`"* — and `command-output-contract.md`'s work-unit target row
declares *"the work-unit ref `task:<id>` / `milestone:<id>`"*. `work-unit:<id>` is neither, and no
read surface resolves it (`jigc doc show work-unit:root-probe` → `store.unknown-type`).

**Graded honestly, and the contradiction is thin.** The finding **projects no key** — every producer
flattens through `finding_to_err`, which `command-output-contract.md` → *The membership test*
explicitly puts outside the envelope — so no **target-form** row is falsified; what the spelling
reaches is the printed `at:` locus alone. And the code's own comment gives a real rationale: the id
is one the mint *would* have taken, so `task:<id>` would name a task that does not exist. The row is
that **nothing declares the third spelling**, on a code minted this wave, while the two declared
ones are enumerated in two locked docs. Tier 3, surface-tier, reversible after 1.0.

---

**Carried, unchanged, all tier 3 and all triaged to 1.x by the charter:** `(5, DEFECT 2)` ·
`(5, DEFECT 3)` · `(5, DEFECT 4)` · `(5, C1)` · `(5, D1)` · rc.17 `DEFECT A`. Repro blocks at §2.
These are **not new findings**.

**TIER-1 ROWS ON THIS AXIS: 0.** Every arm that reports a landed commit was checked against
`git rev-parse HEAD`, `HEAD^{tree}` and `HEAD^`; every refusing cell of the new committing arm was
checked for HEAD movement (12 posture cells, 5 index-dirty cells, the moved-HEAD cell, the
hook-rejected cell with the raw object digest); every one of the 8 `doc` write leaves was checked
with `git status --porcelain` and an `ls` of the task area; the displacing door's plant was checked
with a `find` before-control; and no `--format json` surface on this axis reported success over a
byte that died.

---

## 9 · What was NOT driven, and why — stated plainly

1. **`doc show | CompoundFieldSlice`** (row 33). The arm declares `Object(&["sha","short"])` and
   **no shipped doctype declares a field of that shape** — driven, `jigc doc schema` over all
   fourteen shipped doctypes (`commit adr spec prd arch-doc changelog vision research idea roadmap
   decisions-log milestone-record completion-record planning-record`) returns no non-scalar field
   but `completion-record.owner-artifact: owned-location`. The `{sha,short}` shape *is* on the wire
   at `task diff`'s `base` key (row 37, driven), but that is not a `doc show` slice. Recorded as not
   driven rather than presented as covered — the same disposition, and the same reason, as rc.19.
2. **`finalize.stage-failed`'s copy-runnable route** (`f54036f6`). Reaching a stage failure needs a
   manufactured index fault; the row is an emitted-argv row that belongs to axis 2/6's route
   question rather than to a pinned envelope, and I did not drive it. Named rather than implied.
3. **`4493c6cd` — `doc show` over a *relocated* doc routing at the store sweep's repair.** My fixture
   for this cell did not reach the relocated state (a prior `git reset --hard` had reverted the
   out-of-band move), so what I drove was an ordinary `store.not-found` over a doc that does not
   exist. **Not driven at its cell**, and stated as such rather than reported.
4. **`STORE_EXIT_FLIPS` at 6 of its 7 members.** What this axis owes is key-set invariance under the
   flip; the exit-flip members themselves need a manufactured pack, a down-stamped corpus or a
   removed doctype (axis 7's fixture work). Not re-derived this run — the registry is byte-unchanged
   in `7d86f99f..51e0b8e4`, which is stated, not driven.
5. **`ManifestKind::ALL` (6) and `SchemaChangeKind::ALL × LOCI` (18).** They shape values *inside*
   `task finalize | Forecast`'s `manifest` and `migrate-corpus | Report`'s report, not any envelope's
   top-level key set. Both carrying envelopes are driven (rows 44 and 11). Axes 4 and 7 own them.
6. **A genuine concurrent / Task-tool fan-out envelope.** `milestone join` and both `milestone
   finalize` arms were driven single-process through real provisioned worktrees. The genuine spawn is
   the standing honest bound M51's VERDICT carries, unchanged by this review.
7. **`setup` / `uninstall` inside the four-cwd divergence sweep** (§7). Excluded as destructive; both
   driven at their own cells (rows 6, 7, and §6's two hostile-cwd sweeps).
8. **The `Route` *kind*** (`Mechanical` / `Human` / `Informational`) is not observable in the release
   binary — the constructors' fences are `#[cfg(debug_assertions)]`. Every route here is recorded by
   the **bytes it emitted** and, where it is a command, by **running it**; the kind is inferred (a
   span leading with `jigc` or `git` is mechanical) and never asserted.
9. **The `DeclaredWhereReachable` reason's *negative* half** — *"`Owed` reddens pack-load for the
   methodology pack and every composition holding it"* — was not driven: it asks what a **different**
   registry value would do, which needs a rebuild. Its **positive** half was driven (§4.13): a
   methodology-alone composition pack-loads clean at exit 0, and the six named facts do buy their
   fence.
10. **The amend arm's `--format json` refusals are the flattened `Reject::Error` row** at
    `amend.head-shape` and at the posture family. That is a **declared** arm and the ordinary arm's
    own shape at the same conditions (control driven at §4.5), so it is recorded rather than filed —
    but it means a driver reads those two conditions' identity out of a message, not out of a key.

---

## 10 · Counts

| | |
|---|---|
| binary | `jigc 1.0.0-rc.20`, asserted before anything else ran; repo HEAD `51e0b8e4` |
| **rows driven** | **~500** — A: 65 `ENVELOPE_ARMS` rows with key sets · B: 96 F-10 cells (§4.1–§4.15: 12 posture × 2 doors = 24 · 5 index shapes · 8 doc write leaves + 3 controls · 4 head-shape cells × 2 formats · 4 cwds · 2 spaced cells · 12 fence/token cells · the rest) · C: 7 path-arg / usability cells + 5 migrate escape shapes · D: 96 hostile-cwd cells (48 outside-repo + 48 deleted-cwd) · E: 216 four-cwd cells (46 leaves × 4, of which 8 re-driven at refusal cells) · F: 7 baseline rows · G: 3 emitted spans run verbatim |
| rows not applicable / not driven | **10 classes** (§9) |
| declared key set == driven key set | **65 / 65** driven arms (1 of 66 not driven, §9.1) |
| undeclared envelope **shapes** found | **0** |
| the four pre-pin deletes absent | **4 / 4** |
| hostile-cwd reject-funnel conformance | **96 / 96** (0 NOT-JSON, 0 bare-`Finding` roots, stdout empty on all 96) |
| four-cwd byte-identical leaves | **46 / 46** (216 cells, 0 real divergences) |
| stream-discipline conformance | **every driven row** (one document on its own stream; the other stream carried plain text only, never JSON) |
| emitted command spans run **verbatim** | **3 / 3 rc=0** (the spaced-root + spaced-path `git -C … restore --staged`; the base-mismatch `jigc task discard --force`; the `git -C … restore --staged` at §4.4) |
| F-10 review findings re-verified **closed by driving** | **8 / 8** — HIGH-1 §4.11 · MEDIUM-2 §4.2 · MEDIUM-3 §4.6/§4.10 · MEDIUM-4 §4.13 · LOW-5 §4.3 · LOW-7 (both halves) §4.5/§4.14 · LOW-8 §4.6 · LOW-6 (the suite's overstated claim) **not driven** — it is a test-comment row with no release-posture surface, recorded as not observable rather than as closed |
| rc.19 §A headline rows re-driven | `(2, N-1)` **CLOSED** · `(2, N-2)` **CLOSED** (§5.4) |
| **baseline §A rows CLOSED** | **1 / 7** (the one tier-1 row, still closed) |
| **baseline §A rows STILL-OPEN** | **6 / 7** — all tier-3, all triaged to 1.x, none a new finding |
| **new defects** | **2** — one tier 2 (§8 DEFECT 1), one tier 3 (§8 DEFECT 2) |
| **tier-1 rows on this axis** | **0** |
| `doors_covered` | **48 / 48** `VERB_KINDS` leaves · uncovered: none |

---

## 11 · What this adds over flow-54 arm 2 (and arms 1, 3, 5), and over `task_amend::`

Flow 54's five arms and `crates/cli/tests/task_amend.rs`'s ten iterate M53's own classes against the
**debug** binary in the build tree. This review re-drives the **pinned-contract** question on the
installed **release `1.0.0-rc.20`** — a different binary, a different posture (no
`#[cfg(debug_assertions)]` route, span or quoting fence), built after the F-10 review's eight fixes
— and then asks what those suites cannot:

- **The acceptance suite is a claim about one working directory; this axis's sixth cell axis is the
  working directory.** `task_amend.rs`'s own doc-comment (the review's LOW-6) overstates its posture
  coverage precisely because the suite's subject is fixed; §4.5 drives **12 git states × 2 doors**
  through the real `dev/jigc-rig --git-state` builder, and §4.14 drives the mint door from **four
  checkouts**. That is how DEFECT 1 was found: it exists in exactly one of those four.
- **`ENVELOPE_ARMS` has four in-tree proofs and all four iterate `ENVELOPE_ARMS`.** *"Is there a
  production arm the registry does not carry?"* is outside all four by construction — it is where
  M51's DEFECTS A and B lived and where `(5, D1)` still lives. §6 asks it with **96** uniform
  hostile-cwd cells over the **48**-leaf set (the suites' leaf set was 47 until this wave) and §7
  with **216** four-cwd cells; zero new undeclared shapes came back, and `(5, D1)` came back **one
  key wider** than the registry-blind suites can see.
- **The named-fact fence's own scope is a claim no in-tree suite drives, because the suites run over
  the shipped pack.** §4.13 drives the fence **token by token** on a manifest-keeping throwaway pack
  (6/6 red) *and* on a manifest-less one (0/6 — the declared skip-on-absent bound, measured rather
  than read). MEDIUM-4's claim is *"delete any one of those sentences and pack-load reddens"*; that
  is now true with its scope stated.
- **HIGH-1's class was reported as three leaves and fixed at one seam; §4.11 drives all eight**,
  including the two the create-gate pre-empts and a control on an ordinary task proving those two
  pre-emptions are pre-existing rather than the amend gate under another name.
- **A release binary is the only posture in which *does the emitted route run?* can be asked.** §4.4,
  §4.8 and §4.12 take three emitted spans — including one on a spaced repository root with a spaced
  staged path — and run them verbatim, recording the shell's status.
- **The three previews and the door are four surfaces of one gate, and no in-tree arm compares all
  four in a checkout jigc itself provisioned.** §8's DEFECT 1 is exactly that comparison, with three
  controls isolating the cell and the `11fe5941` orientation half driven beside it.

---

## 12 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs:1877`, **48** leaves read by symbol at `HEAD = 51e0b8e4`).
**48 / 48 · uncovered: none.**

```
start · workflow · setup · uninstall · upgrade · ingest · migrate · migrate-corpus · unmanage ·
rename · relocate · describe · validate ·
doc create · doc add-item · doc remove-item · doc retitle-item · doc rename · doc set-field ·
doc set-slot · doc author · doc show · doc schema · doc list ·
task list · task diff · task validate · task amend · task discard · task finalize · task bind ·
config set · config insert-step · config replace-step · config remove-step · config fill ·
config fork · config get · config list ·
milestone create · milestone add-task · milestone add-from-spec · milestone list-tasks ·
milestone provision · milestone execute · milestone join · milestone finalize · milestone discard
```

**How the coverage is carried, stated rather than implied.** All 48 are the door of ≥1 row in each of
the two uniform 48-leaf hostile-cwd sweeps (§6.1, §6.2). **46** are additionally the door of ≥1 row
in the four-cwd divergence sweep (§7) — the two excluded are `setup` and `uninstall`, each driven at
its own success cell (rows 6, 7). **47** are additionally the door of a success-arm or
condition-specific row with a captured key set (§3); the one leaf whose axis-5 coverage beyond the
sweeps rests on a **partial** arm set is **`doc show`**, driven at seven of its eight arms with the
eighth (`CompoundFieldSlice`) unreachable and named at §9.1. **`task bind`** and **`milestone
add-from-spec`**, which rc.19 could drive only at their reject arms, are **driven at success this
run** (rows 41, 57).

---

## 13 · Notes

- **Nothing was fixed, committed or edited in the working repository.** The only writes were to the
  session scratchpad and to throwaway `mktemp -d` rigs; the two pack-file edits (§2.5, §4.13) were a
  `mv` and a `cp` into a `mktemp -d` stash, each restored in the same command and each verified
  restored by a re-drive at exit 0.
- **No `rm -rf` on a variable path appears anywhere in this review**, and nothing needed teardown:
  every root came from `mktemp -d`.
- **The one count I corrected against the baseline is `DOCTYPE_DOORS`** — rc.19 recorded 17, the
  symbol carries **16** at this HEAD, and the block is byte-unchanged in the range, so the rc.19
  figure counted one `DoctypeArg::` occurrence inside a doc-comment. Recorded rather than silently
  re-stated.
- **Two registries moved that the F-10 settle said would not, and both are correct**: none did.
  All five "what does not move" claims (`ROLLBACK_POPULATIONS`, `DESTROYING_DOORS`,
  `WORK_UNIT_ID_DOORS`, `PATH_ARG_OCCURRENCES`, `contract-version`) hold by symbol (§5.3).
- **The exit rule's third clause is engaged by DEFECT 1's orientation half**, which sits inside the
  usability batch's own new code (`11fe5941`) — the adjudication is the human's and this record does
  not pre-empt it. Read by **tier**, both defects are below the tier-1 line and neither blocks the
  call.

---
---

# Reconciliation ledger — AXIS 5 · pinned contracts

**The reconciler.** Driver table above: `driver/axis5.md`, unchanged. Source pass: `codex/axis5-codex.md`
(read-only, its own stated bound: *"no binary behavior, tier-2/3 repro, release-only behavior, hostile
cwd, or hook execution was independently driven"*). Binary for every drive in this ledger:
`/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.20`**, asserted first. Repo `HEAD = 51e0b8e4`.
Rigs two-step-evalled with a non-empty-`$REPO` guard, `mktemp -d` roots, no teardown.

**The rule applied** (acceptance-design.md → *The reconciliation rule*): a claim by one pass that the
other cannot reproduce is a **lead**, not a finding. Every Codex claim below was entered as
`lead(codex, …)` and then **driven** — to a repro block, or to a refutation carrying the falsifying
datum. Every driver **defect** was **re-driven by the reconciler** before it was allowed to stand.

**The reconciler's own prompt note:** the task named `codex/axis5-prompt.md` as sitting beside the
source pass. **That file does not exist** in `codex/` (contents: `axis2-codex.md`, `axis2.stderr`,
`axis2.stdout`, `axis3-*`, `axis5-codex.md`, `axis5.stderr`, `axis5.stdout`, `rc.txt`). The pass was
therefore reconciled on its own text, with no sight of the instruction that produced it — stated
rather than silently worked around.

---

## A · Demotions — rows marked driven that carry no repro block

**None. Zero demotions.**

The driver's §3 is a 66-row sweep in which each row's **datum is its captured key set**, and the one
row without one (**row 33, `doc show | CompoundFieldSlice`**) is already marked *NOT DRIVEN* by the
driver itself, with its reason at §9.1 — so it was never a driven row to demote. Every other verdict
in the file (§2, §4, §5, §6, §7, §8) carries an inline repro block, and the three places the driver
asserts something it did **not** drive are labelled as such at the point of assertion (§5.3 *"hold by
symbol"*; §9.4 `STORE_EXIT_FLIPS` *"stated, not driven"*; §9.9 the `DeclaredWhereReachable` negative
half).

**To test whether the §3 table's *driven* marks are real rather than transcribed, the reconciler
independently re-drove a sample of eight rows.** All eight matched the driver's recorded key set
**exactly**:

```
binary asserted: jigc 1.0.0-rc.20
row 6  setup            | Installed      rc=0  allowlist_file, findings, guide_file, hook_committed,
                                               hook_file, install_commit, line_file     has "installed": false
row 7  uninstall        | TornDown       rc=0  allowlist_file, findings, line_file, removed
                                               has "uninstalled": false
row 39 task amend       | Composed       rc=0  task, text        .text contains "amending:" -> false
row 41 task bind        | TaskAck::Bound rc=0  findings, op, role, target, task
       {"findings":[],"op":"task-bind","role":"spec",
        "target":{"doctype":"spec","slug":"rate-limiter"},"task":"implement-the-limiter"}
row 43 task finalize    | LandedAmend    rc=0  committed, findings, schema_version
       committed = amended, displaced, files, hash, hook_output, left_out, manifest, promoted, subject
       amended 77d6494 · hash 05a600b · files 0
       BEFORE HEAD=77d6494 TREE=9dc5c91 PARENT=d6ad8b5   AFTER HEAD=05a600b TREE=9dc5c91 PARENT=d6ad8b5
       (tree and parent unmoved; only the message moved — the arm's whole claim)
row 46 task finalize    | MigrationReviewHold  rc=4  retires, rewrites, source, task
       retires ["foreign-adr.md"]  (repo-relative)      has "review": false
row 57 milestone add-from-spec | RecordOnlyAck rc=0  hook_output, text
row 60 milestone list-tasks    | Listing       rc=0  text      (no hook_output — the pre-pin delete)
```

**One correction to the driver, not a demotion — §3's partition line is miscounted in two places.**
The driver's preamble reads *"**60 `Success` / 3 `Adjudicated` / 2 `Reject`** · **`Pinned` 60 /
`Unpinned` 6**"*. Read by symbol at `HEAD = 51e0b8e4`:

```
$ awk 'NR>=6572 && /^];/{exit} NR>=6572' crates/cli/src/render.rs > arms.txt
$ grep -c 'EnvelopeArm {'               arms.txt   -> 66     (the driver's total: correct)
$ grep -c 'outcome: ArmOutcome::Success' arms.txt  -> 61     (the driver wrote 60; 60+3+2 = 65 ≠ 66)
$ grep -c 'outcome: ArmOutcome::Adjudicated'       -> 3
$ grep -c 'outcome: ArmOutcome::Reject'            -> 2
$ grep -c 'status: ArmStatus::Pinned'    arms.txt  -> 59     (the driver wrote 60)
$ grep -c 'status: ArmStatus::Unpinned'  arms.txt  -> 7      (the driver wrote 6)
the seven Unpinned rows, by symbol:
  describe|Menu · milestone create|add-task|add-from-spec|provision|discard (RecordOnlyAck, 5) ·
  milestone list-tasks|Listing
```

Both errors are **off by one in the same direction** and both self-evidently arithmetic: 60+3+2 = 65
and 60+6 = 66 cannot both be partitions of one 66-row registry. The driver's §3 table marks
*(Unpinned)* on three rows (15, 55, 60) and leaves it off the other four `RecordOnlyAck` rows, which
is the likely origin. **No row verdict moves** — all seven Unpinned rows are driven in the table and
all seven are `= declared`. Recorded because this axis's subject is *counted registries*, so a count
the review states wrong is a datum the next wave would inherit. It also **corroborates Codex's C-15**
(below), whose characterization of the Unpinned set is the correct one.

**Everything else the driver counts, re-read by symbol and holding:** `VERB_KINDS` **48** ·
`ENVELOPE_ARMS` **66** · `COMMITTING_DOORS` **11** · `ERROR_CODE_REGISTRY` **12** ·
`WORK_UNIT_ID_DOORS` **25** · `AMBUSH_CONTRACTS` **6** · goldens **646** (`find … -type f`).

---

## B · Codex claims → CONFIRMED / REFUTED / OPEN LEAD

Twenty leads were extracted from the source pass: its two numbered **Claims**, its seven **rc.20
baseline dispositions**, its six **M51 confirmed-row source statuses**, and the five load-bearing
assertions in its census/bounds sections. Each was driven.

### The two numbered claims

**C-1 — *"the pinned orientation rows still declare `next_steps`, although serialization omits that
key for a reachable empty set"*** → **CONFIRMED (repro).** Codex's proposed reproduction was followed
exactly, at **both** declared arms. *(Same row as the driver's `(5, C1)`, reached independently.)*

```
rig: dev/jigc-rig fresh --pack-from-dev   (JIGC_PACK_DIR = a throwaway dev-pack copy; no methodology
                                           pack, so `planning` is absent; `ingest-existing` present)
CONTROL, the populated arm:
$ jigc --format json start        -> rc=0  keys: header, next_steps, schema_version, state, workflows
  .state "clean"   .next_steps [{"id":"ingest-existing","gist":"bring an existing repo's docs under management"}]

$ STASH=$(mktemp -d "${TMPDIR:-/tmp}/packstash.XXXXXX")
$ mv "$JIGC_PACK_DIR/workflows/ingest-existing.yaml" "$STASH/"     # a MOVE, restored below

CLEAN arm  (no OFF_CATALOG_VERBS member shipped):
$ jigc --format json start        -> rc=0  keys: header, schema_version, state, workflows
  .state "clean"          has("next_steps") -> FALSE       stderr 0 bytes
DECLARED at render.rs:6585-6600 (OrientationView::Clean):
  ArmShape::Object(["header","next_steps","schema_version","state","workflows"])

ACTIVE arm (same pack-set, one task minted with `jigc start --workflow single-task "probe task"`):
$ jigc --format json start        -> rc=0  keys: header, schema_version, state, tasks, workflows
  .state "active-task"   .tasks ["probe-task"]   has("next_steps") -> FALSE
DECLARED (OrientationView::ActiveTask): ["header","next_steps","schema_version","state","tasks","workflows"]

$ mv "$STASH/ingest-existing.yaml" "$JIGC_PACK_DIR/workflows/"     # restored
$ jigc --format json start        -> rc=0   has("next_steps") -> true    (restoration verified)
```

The mechanism Codex read is the one the drives show: `crates/engine/src/result.rs:138-145,172-175`
carries `#[serde(default, skip_serializing_if = "Vec::is_empty")]` on both `next_steps` fields, and
`crates/cli/src/orient.rs:121-125` builds the vector by filtering `OFF_CATALOG_VERBS` against
`pack.list(Workflows)` membership, so a composition shipping neither `planning` nor `ingest-existing`
yields the empty set at exit 0. **A driver reading the pinned key set gets a document missing a
declared key, with nothing on either stream saying so.**

**C-2 — *"`start --explain` remains a production JSON-success envelope with no `ENVELOPE_ARMS`
row"*** → **CONFIRMED (repro).** *(Same row as the driver's `(5, D1)`.)*

```
rig: committed-singletons                                cwd = $REPO
$ jigc --format json start --explain > o 2> e; rc=$?
  rc=0    stdout 939 bytes    stderr 0 bytes
  keys: collision_winners, overrides_applied, pack_inputs, schema_version, steps, workflow, workflow_layer
the registry, read by symbol — its four `start` rows and only those:
$ awk 'NR>=6572 && NR<=7400 && /path: &\["start"\]/{print NR; getline; print}' crates/cli/src/render.rs
  6574 arm: "OrientationView::UnsetProject"
  6585 arm: "OrientationView::Clean"
  6602 arm: "OrientationView::ActiveTask"
  6620 arm: "Composed"
$ grep -n 'ResolutionTree' crates/cli/src/render.rs | grep -v '^8[0-9][0-9][0-9]:'
  711: /// Render the `--explain` [`ResolutionTree`] …      722: pub fn explain(format, tree, pack_label)
  (no ENVELOPE_ARMS row names it)
```

**The reconciler's own drive returned seven keys where the driver recorded eight** — the driver drove
it *after* a `jigc config set placement-root myroot`, which surfaces the arm's optional
`scalar_overrides` member; the reconciler drove a virgin rig, which does not. **That difference is
itself the row's sharpest datum and both passes are right:** the undeclared arm's key set is not
merely undeclared, it is **not fixed** — it varies with cascade state, which is exactly what a pinned
row would have had to say.

### The seven rc.20 baseline dispositions

Codex disposed the baseline as **1 CLOSED / 6 STILL-OPEN**. The driver disposed it identically. The
reconciler re-drove **all seven** rather than accept the agreement.

**C-3 — `(5, DEFECT 1)` fabricated milestone identity, CLOSED** → **CONFIRMED (repro).**

```
rig: committed-singletons        HEAD before: 77d6494
$ jigc --format json milestone create ""  -> rc=1
  {"error":"blocking · write.unslugable-title — cannot mint a milestone: its id is slugged from the
            title, and this title slugs to nothing …\n  at: milestone\n  route: re-run with a title
            carrying ASCII letters or digits — the milestone id is slugged from it"}
$ jigc --format json task amend ""        -> rc=1   the same code, "cannot mint a task", at: task
HEAD after: 77d6494 (unmoved)
```
The tier-1 row stays closed, and the **new** mint door (`task amend`) is inside the guard.

**C-4 — `(5, DEFECT 2)` a resolved included step refused by the config mutators, STILL-OPEN** →
**CONFIRMED (repro).**

```
rig: committed-singletons
$ printf 'id: probe-step\ntitle: Probe\nbody: |\n  Probe.\n' > /tmp/probe-step.yaml
$ jigc --format json config insert-step --workflow single-task --before finalize /tmp/probe-step.yaml
  -> rc=0   {"op":"config-insert-step","step":"probe-step","anchor":"finalize"}
$ jigc --format json start --explain --workflow single-task | jq -r '.steps[].id'
  locate implement record-changelog superseded-context author-commit probe-step finalize
                                                        ^^^^^^^^^^^  IT IS IN THE RESOLVED LIST
$ jigc --format json config remove-step 'workflow:single-task#probe-step'   -> rc=1
  {"error":"blocking · config.anchor-absent — no step `probe-step` body to fork
            route: name a step id present in the workflow's resolved include list, then re-run"}
CONTROL, a pack step at the same door:
$ jigc --format json config remove-step 'workflow:single-task#implement'    -> rc=0
  {"committed":false,"op":"config-remove-step", …}
```
The route names the exact precondition the state satisfies. Tier 3, triaged to 1.x.

**C-5 — `(5, DEFECT 3)` the colon-less-address bail carries no code, two producers, STILL-OPEN** →
**CONFIRMED (repro).** Codex's two source loci (`rename.rs:1193-1207`, `doc.rs:6511-6518`) both
answer with a bare flattened `error`:

```
$ jigc --format json rename vision --to "New Vision"   -> rc=1
  {"error":"`vision` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`
            route: run `jigc describe` for the doctype surface"}          <- no code, no key
$ jigc --format json doc show nosuchtype               -> rc=1
  {"error":"malformed address `nosuchtype`: missing ':' between type and slug — a doc is addressed as
            `<type>:<slug>` … route: run `jigc describe` for the doctype surface"}   <- no code, no key
```
Two producers, two wordings, one condition — as both passes record.

**C-6 — `(5, DEFECT 4)` a declared-but-unpopulated optional leaf blocks with `store.no-such-leaf`,
STILL-OPEN** → **CONFIRMED (repro).**

```
$ jigc doc schema vision --format json | jq -c '.fields[]|select(.id=="grounded-in")'
  {"id":"grounded-in","type":"ref","to":"research","required":false,"author-required":false,
   "section":"meta","set-field":"vision:<slug>#meta/grounded-in"}          <- the schema declares it
$ jigc --format json doc show 'vision:vision#meta/grounded-in'             -> rc=1
  findings[0].key {"code":"store.no-such-leaf","target":"vision:vision#meta/grounded-in"}
  message "`vision:vision#meta/grounded-in` names no leaf `grounded-in` in section `meta`"
  route   "name a field the committed section carries …"
```
The read surface denies the existence of a leaf its own schema surface advertises one screen earlier.

**C-7 / C-8 — `(5, C1)` and `(5, D1)` STILL-OPEN** → **CONFIRMED**, the same rows as C-1 and C-2
above; repro blocks there. Codex reached both independently of the driver.

**C-9 — rc.17 `DEFECT A`, `doc show` uses the full address as the `store.unknown-type` target,
STILL-OPEN** → **CONFIRMED (repro).**

```
$ jigc --format json doc show   'nosuchtype:x'  -> findings[0].key {"code":"store.unknown-type","target":"nosuchtype:x"}
$ jigc --format json doc schema  nosuchtype     -> findings[0].key {"code":"store.unknown-type","target":"nosuchtype"}
```
One code, two target forms — the doctype-id form at `doc schema`, a doc-URI-shaped one at `doc show`.
`command-output-contract.md`'s six declared target forms give `store.unknown-type` the **doctype id**;
the stable `(code, target)` key therefore differs between two doors answering about the same absent
doctype. Tier 3.

### The six M51 confirmed-row source statuses

**C-10 — the pre-dispatch `current_dir()` bypass is CLOSED** → **CONFIRMED (repro).**

```
$ D=$(mktemp -d "${TMPDIR:-/tmp}/gonecwd.XXXXXX")
$ sh -c "cd '$D' && rmdir '$D'; exec jigc --format json validate"   -> rc=1
  {"error":"cannot determine the current directory: No such file or directory (os error 2)"}
$ (same, describe)                                                  -> rc=1  the identical document
```
One declared `Reject::Error` document, exit 1, on a deleted working directory — no panic, no bare
prose, no split answer per leaf. (The driver's §6.2 drives this over all 48 leaves; the reconciler
spot-drove two.)

**C-11 — setup/uninstall's third bare-`Finding` reject shape is CLOSED** → **CONFIRMED (repro).**

```
$ OUT=$(mktemp -d …); cd "$OUT"; export HOME="$OUT/home"      # not a git repository
$ jigc --format json setup      -> rc=1  root = Reject::Findings
$ jigc --format json uninstall  -> rc=1  root = Reject::Findings
$ jigc --format json validate   -> rc=1  root = Reject::Error
$ jigc --format json describe   -> rc=1  root = Reject::Error
$ jigc --format json task amend -> rc=1  root = Reject::Error   stdout 0 bytes
  {"error":"not inside a git repository (no `.git` found from /private/var/…/outrepo.AUNKJL) — run
            jigc from inside the target git repository; if this project isn't one yet, `git init` here first"}
```
Two declared reject roots and no third shape, with the new 48th leaf inside the set.

**C-12 — the missing `doc show` array/data-keyed projections are CLOSED (eight projection rows
present)** → **CONFIRMED**, by symbol plus two driven slices.

```
$ jigc --format json doc show 'changelog:changelog#releases'        (ArrayOfDataKeyed)
  [{"changes":[{"category":"added","id":"added","notes":"- the trial-shaped fixture builder"}],
    "date":"2026-09-27","id":"1-0-0","link":"https://example.com/compare/0.9.0...1.0.0","title":"1.0.0"}]
$ jigc --format json doc show 'changelog:changelog#releases/1-0-0'  (DataKeyed)
  keys ["changes","date","id","link","title"]
```
Rows 30 and 31 of the driver's table, independently reproduced. The remaining projection rows rest on
the driver's §3 captures plus the registry read; the reconciler did not re-drive all eight.

**C-13 — *"success rows falsely promising invariant exit 0"* is CLOSED as a prose claim** →
**OPEN LEAD.** *Reason: not drivable.* The claim's subject is the **wording of the registry's own
doc-comment** about its status/outcome model, not a byte any invocation emits. A source read agrees
with Codex (the outcome model is stated per row rather than as a blanket exit-0 promise), but that is
a second source read, not a drive, and this ledger does not promote a source read to a finding. It
stays a lead with its reason written.

**C-14 — the four pre-pin deletes are CLOSED** → **CONFIRMED (repro).** All four driven by the
reconciler: `setup` carries no `installed`, `uninstall` no `uninstalled`, the migration review hold no
`review`, `milestone list-tasks` no `hook_output` — the four blocks are in §A above (rows 6, 7, 46,
60), each with `has(<key>) -> false` read off the driven document.

**C-15 — the Unpinned-shape classification is CLOSED: only `describe` and the milestone text
acknowledgements are Unpinned, each with a reason** → **CONFIRMED (symbol read), and it corrects the
driver.** Seven `status: ArmStatus::Unpinned` rows, and they are exactly the set Codex names:
`describe|Menu` + the five `milestone …|RecordOnlyAck` + `milestone list-tasks|Listing`. This is the
datum that falsifies the driver's *"Unpinned 6"* (§A).

### The census and bounds assertions

**C-16 — *"the registry now contains 66 arms over 48 clap leaves"*** → **CONFIRMED (symbol).**
`grep -c 'EnvelopeArm {'` over the registry span → **66**; `grep -c 'VerbKind::'` over `VERB_KINDS` →
**48**. Codex and the driver agree, and the reconciler's independent read agrees with both.

**C-17 — the two new arms are correctly represented (`task amend | Composed` = `{task,text}`;
`task finalize | LandedAmend` top-level `{committed,findings,schema_version}` with the additive
nested `committed.amended` from the pinned sha)** → **CONFIRMED (repro).** Driven end to end by the
reconciler on a landed amend; the block is in §A (rows 39 and 43). The load-bearing half — **the tree
and parent do not move and only the message does** — was checked with `git rev-parse` before and
after: `TREE 9dc5c91 → 9dc5c91`, `PARENT d6ad8b5 → d6ad8b5`, `HEAD 77d6494 → 05a600b`,
`committed.amended = 77d6494`.

**C-18 — *"the four proof fences remain"* (`format_json_success_axis.rs:1272-1575`)** →
**OPEN LEAD.** *Reason: not drivable in this posture.* These are `#[test]` targets; observing them
requires building and running the debug test target, and this review's binary and posture are the
**installed release** `1.0.0-rc.20`. Named rather than promoted on the source read, and rather than
dropped.

**C-19 — *"the sole uncovered stdout serialization is Claim 2"*** → **CONFIRMED as a converged
result, with its bound stated.** Two passes reached it by disjoint methods — Codex by a source census
of every command stdout JSON path, the driver by **96** uniform hostile-cwd cells over all 48 leaves
plus **216** four-cwd cells, both returning exactly one undeclared arm — and the reconciler's own
spot-drives added none. **Neither method establishes exhaustiveness**: a key set reachable only under
a state neither pass built would appear in neither, which is the registry's own declared bound
(`render.rs:6560-6568`, *"arm completeness is bounded by driving"*). Recorded as convergence, not as
proof.

**C-20 — *"the schema manifest SHA-256 is identical … zero schema-hash movement was not violated"***
→ **CONFIRMED (repro), with one reading correction.** Codex's sentence reads as though `834772b6`
were a file hash; it is a **commit** — the rc.19 review commit — and `e15d332b…` is the file digest.
Read that way the claim is exactly right:

```
$ git show 834772b6:crates/cli/pack/config/schema-manifest.yaml | shasum -a 256   -> e15d332bc8dd…
$ git show 51e0b8e4:crates/cli/pack/config/schema-manifest.yaml | shasum -a 256   -> e15d332bc8dd…
$ git show 834772b6:packs/methodology/config/schema-manifest.yaml | shasum -a 256 -> df0484d706a5…
$ git show 51e0b8e4:packs/methodology/config/schema-manifest.yaml | shasum -a 256 -> df0484d706a5…
```
Both manifests are byte-identical across `834772b6..51e0b8e4`. The driver's §5.2 reaches the same
boundary from the other side, and the reconciler re-drove that too: `jigc doc schema adr --format
json` → `contract-version` **7**; `jigc doc show vision --format json` → `schema-version` **1**.

**Codex claims: 20 entered · 18 CONFIRMED · 0 REFUTED · 2 OPEN LEADS (C-13, C-18).**

---

## C · Driver defects → status

**DEFECT 1 (tier 2) — the preview surfaces report a clean repository posture from a provisioned
fan-out worktree where the committing door refuses** → **CONFIRMED, stands. Re-driven by the
reconciler from scratch, with both isolating controls.**

*Codex is **silent** on it.* That is not a contradiction: the source pass's own bounds exclude it
(*"no binary behavior, tier-2/3 repro, release-only behavior … was independently driven"*), and its
one negative statement — *"no new F-10-specific completeness defect was found"* — is about
`ENVELOPE_ARMS` completeness inside F-10's new code, while this defect is (a) a **preview-contract**
divergence, not an envelope-completeness one, and (b) explicitly **wider than F-10**, reproducing on
the ordinary `task finalize` arm as well. Nothing in the source pass would have reached it.

```
binary: jigc 1.0.0-rc.20                       rigs: dev/jigc-rig fresh --binary <installed>
fixture, built by driving the binary:
  $ jigc milestone create "cwd wave"           -> rc=0   record commit 20d100b
  $ jigc milestone add-task cwd-wave "area one"-> rc=0   record commit b06b237
  $ jigc milestone provision cwd-wave          -> rc=0   provisioned 1 worktree at base cea1417 (area-one)
  $ git worktree list
      …/repo                                  b06b237 [main]
      …/repo/.jigc/worktrees/area-one         cea1417 (detached HEAD)
  cd .jigc/worktrees/area-one
  $ git status --porcelain=v2 -b | grep '^# branch.head'   -> # branch.head (detached)
  $ jigc start --workflow single-task "ordinary in worktree"   -> rc=0  (an ORDINARY task)
  … commit doc authored (type=feat, scope=core, summary, body) …
  $ echo k > k.txt; git add k.txt      ->  git status --porcelain: A  k.txt

THE THREE SURFACES OF ONE GATE, all from that same cwd:
  $ jigc task validate ordinary-in-worktree               -> EXIT 0
      advisory · changelog-recording.gate-granted-unused  (only)      NO repo.* finding
  $ jigc task finalize ordinary-in-worktree --dry-run     -> EXIT 0
      "would commit — feat(core): ordinary in a detached worktree
         would commit in the linked worktree at `.jigc/worktrees/area-one` — not in the main checkout …"
  $ jigc task finalize ordinary-in-worktree               -> EXIT 1
      blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no branch,
        and the next checkout would leave it unreachable
        route: re-attach HEAD with `git switch <branch>`, then re-run this command
  $ git rev-parse --short HEAD   -> cea1417 (unmoved by the refusal)

THE ORIENTATION HALF (M53's own usability row `11fe5941`), same cwd:
  $ jigc --format json start | jq -c '.tasks[]|{id, findings:(.findings//[]|map(.code//.))}'
      {"id":"area-one","findings":[]}
      {"id":"ordinary-in-worktree","findings":["changelog-recording.gate-granted-unused"]}
                                    ^^^ NO repo.head-detached on either live task

CONTROL — an ORDINARY linked detached worktree (git worktree add --detach), same repo, same tasks:
  $ jigc --format json start | jq -c '.tasks[]|{id, findings:…}'
      {"id":"area-one","findings":["repo.head-detached"]}
      {"id":"ordinary-in-worktree","findings":["repo.head-detached","changelog-recording.gate-granted-unused"]}
  $ jigc task validate ordinary-in-worktree   -> EXIT 1  repo.head-detached
  $ jigc task finalize ordinary-in-worktree   -> EXIT 1  repo.head-detached
      (all surfaces agree — the divergence is specific to the PROVISIONED worktree)
```

**Two independent fixtures, the same result.** The reconciler built its milestone, its provisioned
worktree and its task from nothing, on a different rig root from the driver's, and reproduced all
four cells — `task validate` 0, `--dry-run` 0, `start` silent, `task finalize` 1 — plus the control
that isolates the cell to a worktree jigc itself provisioned. The contradiction the driver cites is
real and quotable: `design/finalize.md:76` and `design/command-output-contract.md:488` both state the
preview asks *the committing door's own guard* and renders *the identical finding at the identical
exit code*. Driven, it is exit **0** against exit **1**, and silence against a coded refusal.

**Nothing is lost and nothing is committed in any cell** — HEAD unmoved at `cea1417` after every
refusal — which is why it stays tier 2.

**DEFECT 2 (tier 3) — `amend.head-shape`'s locus uses a work-unit address spelling no grammar in the
product declares** → **CONFIRMED, stands. Re-driven.** Codex is silent; its bounds exclude driving,
and a printed `at:` locus on a flattened `Reject::Error` is not an envelope-registry row, so the
source pass's completeness census would not surface it.

```
rig: dev/jigc-rig --git-state unborn, then `jigc setup` (which births a ROOT HEAD)
$ jigc --format json task amend "unborn probe"           -> rc=1
  {"error":"blocking · amend.head-shape — `jigc task amend` rewrites a single-parent commit, and HEAD
            is unborn — this repository has no commit yet — nothing was minted
            \n  at: work-unit:unborn-probe\n  route: there is no commit to repair; make one first …"}
$ jigc setup   -> rc=0   HEAD born: 9b5667c   (git rev-list --parents -n1 HEAD -> 1 word = root)
$ jigc task amend "root probe"                           -> rc=1   at: work-unit:root-probe
$ jigc task amend            (no intent)                 -> rc=1   at: work-unit:amend-9b5667c
$ ls .jigc/tasks                                         -> No such file or directory   (nothing minted)

the sole producer, by symbol:
$ grep -rn 'format!("work-unit:' crates/cli/src crates/engine/src
  crates/cli/src/start.rs:358:            format!("work-unit:{id_hint}"),

what the locked docs declare:
  design/structural-grammar.md:98 — "addresses are `type:name` with no fragment
                                     (`task:add-rate-limiter`, `milestone:m1`)"
  design/command-output-contract.md:249 — "the work-unit ref `task:<id>` / `milestone:<id>`"
and no read surface resolves the third spelling:
$ jigc --format json doc show work-unit:root-probe       -> rc=1
  findings[0].key {"code":"store.unknown-type","target":"work-unit:root-probe"}
  "unknown doctype `work-unit` for `work-unit:root-probe`"
```

The driver's own grading is upheld on re-drive and is the honest one: the finding **projects no key**
(it flattens through `finding_to_err`, which `command-output-contract.md` → *The membership test*
puts outside the envelope), so no declared **target-form** row is falsified — what the spelling
reaches is the printed `at:` locus alone, on a code minted this wave. Tier 3, surface-tier.

**Driver defects: 2 entered · 2 CONFIRMED (both re-driven) · 0 refuted · 0 demoted.**
**Tier-1 rows on this axis after reconciliation: 0.**

---

## D · Doors covered

Every clap leaf that is the door of **≥ 1 driven row** in this reconciled file, in `VERB_KINDS`
spelling (`crates/cli/src/cli.rs:1877`, **48** leaves read by symbol at `HEAD = 51e0b8e4`).
**48 / 48 · uncovered: none.**

```
start · workflow · setup · uninstall · upgrade · ingest · migrate · migrate-corpus · unmanage ·
rename · relocate · describe · validate ·
doc create · doc add-item · doc remove-item · doc retitle-item · doc rename · doc set-field ·
doc set-slot · doc author · doc show · doc schema · doc list ·
task list · task diff · task validate · task amend · task discard · task finalize · task bind ·
config set · config insert-step · config replace-step · config remove-step · config fill ·
config fork · config get · config list ·
milestone create · milestone add-task · milestone add-from-spec · milestone list-tasks ·
milestone provision · milestone execute · milestone join · milestone finalize · milestone discard
```

**How the coverage is carried, unchanged from the driver's §12 and restated here only where the
reconciliation moved it.** All 48 are the door of ≥ 1 row in each of the two uniform 48-leaf
hostile-cwd sweeps (§6.1, §6.2); 46 are additionally doors in the four-cwd divergence sweep (§7),
`setup` and `uninstall` being excluded there as destructive and driven at their own cells; 47 are
additionally the door of a success-arm or condition-specific row with a captured key set (§3). The
one leaf whose coverage beyond the sweeps rests on a **partial** arm set is **`doc show`** — seven of
eight arms, the eighth (`CompoundFieldSlice`) unreachable at HEAD and named at §9.1.

**The reconciler's own drives touched 17 of the 48** — `start`, `setup`, `uninstall`, `validate`,
`describe`, `doc show`, `doc schema`, `doc create`, `doc set-slot`, `doc add-item`, `doc set-field`,
`rename`, `migrate`, `task amend`, `task bind`, `task finalize`, `task discard`, plus
`config insert-step`, `config remove-step`, `milestone create`, `milestone add-task`,
`milestone add-from-spec`, `milestone provision` and `milestone list-tasks` (24 leaves in total) —
and **found no leaf where the driver's recorded answer failed to reproduce**.

---

## E · Reconciler's notes

1. **No fixes, no commits, no repo edits** — with one exception the reconciler caused and undid, and
   records rather than hides. Early in the run a `rig=$(…); eval "$rig"` left `$REPO` **empty**
   (the rig script's banner lines had been swallowed by a stdout redirect on the `eval`), and the
   subsequent `cd "$REPO"` was a no-op in zsh rather than an error — so a `mkdir -p docs/deep`, a
   `git add` and a `git commit -q` landed **in the working repository**, creating commit `344c08e6`
   *"chore: deep dir"* on `main`. It was undone in the same minute: `git reset --mixed 51e0b8e4`,
   then `find docs -mindepth 1 -delete && rmdir docs` (no `rm -rf`, no variable path). **State
   restored and verified**: `HEAD = 51e0b8e4`, working tree carrying only the one pre-existing
   modification (`completions/artifacts/M53/per-axis-review-rc20/instrument/per-axis-review.workflow.js`),
   `docs/` absent, no stray object reachable from any ref. Every later rig eval carries an explicit
   `[ -n "$REPO" ] || exit` guard. Recorded because a review whose subject is *doors that destroy or
   commit at exit 0* has no business quietly committing to its own repository.
2. **No `rm -rf` on a variable path appears anywhere in this reconciliation**, and nothing needed
   teardown: every rig root and every scratch directory came from `mktemp -d`. The two pack-file
   manipulations (C-1's `mv` into a `mktemp -d` stash) were restored in the same arc and the
   restoration was **verified by a re-drive at exit 0** with the key back.
3. **The driver's file is reproduced above byte-for-byte.** No row was edited, reworded or removed.
   The only changes this reconciliation makes to the driver's content are **stated as corrections in
   §A**, not applied silently: the §3 partition line's two off-by-one counts.
4. **The two OPEN LEADS are open for opposite reasons and neither should be read as weak.** C-13 is a
   claim about **prose in a doc-comment** — there is no invocation that can confirm or refute it, so
   it cannot become a finding by driving, only by a second reader agreeing with the first. C-18 is a
   claim about **test targets**, which this posture (installed release binary, no build tree in
   play) cannot execute. Both are recoverable at zero cost by the party that owns the tree.
5. **What the convergence on C-19 is worth, stated rather than implied.** Two passes with disjoint
   methods finding the same single undeclared arm is the strongest completeness evidence this
   instrument produces, and it is still **not** a proof — the registry's own declared bound says an
   arm reachable only under an unbuilt state appears in neither pass. The honest summary is *no third
   undeclared arm was found by either method*, not *there is no third undeclared arm*.
