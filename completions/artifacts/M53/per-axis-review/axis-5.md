<!-- M53 PARTIAL per-axis review · axis 5 · RECONCILED (Opus driver table + reconciliation ledger) — every row driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.17`, repo HEAD `7e98faf1`, 2026-09-22. -->

<!-- M53 PARTIAL per-axis review — axis 5 · pinned contracts · RECONCILED (driver table + reconciliation ledger). -->
<!-- Reconciler: drove every Codex claim and re-drove the driver's defect on the installed `jigc 1.0.0-rc.17`, 2026-09-22. -->

> **RECONCILED FILE.** §0–§13 below are **the Opus driver's table, unchanged except for the four
> demotion markers** (`[DEMOTED — no repro block]`) the reconciliation rule requires. The
> **Reconciliation ledger (§14)** and **Doors covered (§15)** are the reconciler's, appended after it.
> Every demoted claim was re-driven by the reconciler; the repro blocks are in §14.3.

# M53 per-axis review (partial: axes 2 · 3 · 5) — AXIS 5 · pinned contracts · THE OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.17`, asserted first, before anything else
ran. **Release posture** — the `Route::mechanical` span fence and the other `#[cfg(debug_assertions)]`
route fences do not exist here, so a route-fence violation shows up as a bad emitted command, never as a
panic.

**The binary this table reflects carries M53's seven post-build audit fixes** (`c9fc0d41` · `5672e32f` ·
`e15b64e3` · `e9757c94` · `5badfbda` · `5a4d12d6` · `b8d3f7bb`, plus the version stamp `7e98faf1`).
`FINALIZE_MESSAGE_FILE`'s membership in both area registries (fix 1) is visible in §5's control cells:
an area left standing after a landed finalize holds **no** file jigc wrote, so `finalize.foreign-bytes`
never fires over jigc's own temp file and the narration's count is the complement's real size.

Every row below ran on that binary, in throwaway repos minted by
`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
(two-step eval, `mktemp -d` roots, no teardown, no `rm -rf` on a variable path). Fixtures beyond the
rig states were built **by driving the binary**; the four exceptions are stated at their cells (a
plant under `.jigc/`, a `cp -R` restore of jigc's own area bytes, a `.jigc/displaced` made a regular
file to fault the move, and a `.git/hooks/pre-commit` that writes into a task area — in each the fault
*is* the file state).

**Instrument note, applied.** This harness's `grep` is a shell function honouring `.gitignore`. Every
loss/survival claim under `.jigc/` below uses `command grep` **with a BEFORE-control that finds the
plant**. The contrast is on the record at §5.1: `command grep -rl PLANTCANARY .jigc/` → 2 hits, rc 0;
the shell `grep` on the identical tree → **rc 1, no hits**. A loss claim built on the second form is
unsound.

I did **not** read the Codex source pass for this axis.

---

## 0 · The door set, derived from the code

Counts read at `HEAD = 7e98faf1` **by symbol**, not from the design doc's numbers:

| registry | file | rows read | vs M52 |
|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1834` | **47** leaves | = |
| **`ENVELOPE_ARMS`** | `crates/cli/src/render.rs:6066` | **64** arms | = |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:5451` | **4** codes (`store.not-found` · `store.no-such-leaf` · `store.unknown-type` · `task::FIXED_IDENTITY`) | = |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:968` | **7** | = |
| `ManifestKind::ALL` | `crates/cli/src/render.rs` | **6** | = |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:171` | **10** | = |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3338` | **6** (`Disposition::{Refuse, Narrate, Displace}`) | = |
| `InProgress::ALL` | `crates/cli/src/repo.rs:272` | **10** (M53's tenth: `UncommittedCherryPick`; plus `UnmergedIndex`) | **9 → 10** |
| `MINT_DOORS` | `crates/engine/src/state.rs:1510` | **5** (3 prose + 2 `Exempt`) | = |
| `TASK_AREA_FILES` | `crates/engine/src/state.rs:163` | 14, **incl. `FINALIZE_MESSAGE_FILE`** | +1 (M53 fix 1) |
| `MILESTONE_AREA_FILES` | `crates/engine/src/state.rs:222` | 6, **incl. `FINALIZE_MESSAGE_FILE`** | +1 (M53 fix 1) |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2589` | **25** | = |
| `BEHALF_DOORS` / `DOCTYPE_DOORS` / `SLUG_DOORS` / `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs` | **47 / 16 / 6 / 14** | = |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs` | **18** | = |

**Axis 5's door set is `ENVELOPE_ARMS` — 64 arms over all 47 leaves.** Partition (read from the
registry): **57 `Pinned` / 7 `Unpinned(<reason>)`** · **12 `ArmRoot::ResultContract` / 52 `AdHoc`** ·
**58 `Object` / 2 `DataKeyed` / 1 `Scalar` / 1 `ArrayOf` / 1 `ArrayOfDataKeyed` / 1 `ArrayOfScalars`** ·
**59 `Success` / 2 `Adjudicated(3)` / 1 `Adjudicated(4)` / 2 `Reject`**.

Cell set (the M51 acceptance design's): `{declared key set == driven key set · Unpinned(<reason>) ·
the four pre-pin deletes absent · the reject funnels (`error` vs findings envelope) · exit code}`.

**The M53-minted surfaces this axis must also iterate** (settle-record §14): the one new code
`finalize.foreign-bytes` · the `displaced` key at both `Displace` doors · the four new producers under
shipped codes (`milestone.terminal` for a settled sub-task item · `write.unslugable-title` at the
milestone/sub-task mint and at `jigc start` · the residual sentence under `finalize.no-task` · the
residual sentence under the two `serial-collision` codes) · `FINALIZE_MESSAGE_FILE` in both registries ·
the tenth `InProgress` member.

---

## 1 · M52 §A rows for this axis: CLOSED / STILL-OPEN — the re-run's first deliverable

M52's §A carries **six** axis-5 entries. Every one was re-driven on rc.17.

| M52 §A row | tier (M52) | re-driven verdict | the datum |
|---|---|---|---|
| **`(5, DEFECT 1)`** — two minting doors accept a title that slugs to nothing and **commit** a record at a fabricated identity at exit 0 | **tier 1** | **CLOSED** | §2.1: `milestone create` / `milestone add-task` / `milestone add-from-spec` / `jigc start --workflow` all refuse `write.unslugable-title` at exit 1 over `{"" · "   " · "!!!" · "日本語" · "the of a"}`; `git rev-parse HEAD` unmoved, `git status --porcelain` empty, `.jigc/tasks/` empty |
| **`(5, DEFECT 2)`** — `config remove-step` / `replace-step` refuse a step that **is** in the include list | tier 3 | **STILL-OPEN** (expected — triaged to 1.x) | §2.2: the resolved include list prints `probe-step` one command earlier; `remove-step` → `config.anchor-absent — no step \`probe-step\` body to fork`, route *"name a step id present in the workflow's resolved include list"* — already satisfied. `replace-step` now answers a **different** wrong code, `config.step-id-collision` |
| **`(5, DEFECT 3)`** — `jigc rename`'s bare-slug refusal is outside `RefusalKind` and carries no code | tier 3 | **STILL-OPEN** (expected), and **wider than reported** | §2.3: `rename vision --to "New Vision"` → `{"error": "\`vision\` is not a \`<type>:<slug>\` address …"}`, no `blocking · <code>` prefix. The same code-less bail is at `doc show` and the `doc` write verbs, under a **second** wording |
| **`(5, DEFECT 4)`** — `doc show` blocks a **declared but unpopulated** optional leaf with `store.no-such-leaf` | tier 3 | **STILL-OPEN** (expected) | §2.4: `doc schema vision` advertises `vision:<slug>#meta/grounded-in` `required: false`; `doc show vision:vision#meta/grounded-in` → exit 1, `store.no-such-leaf`, *"names no leaf `grounded-in` in section `meta`"* |
| **`(5, C1)`** — `start`'s two orientation rows declare `next_steps`, a reachable composition omits it | tier 3 | **STILL-OPEN** (expected), **both arms** | §2.5: `fresh --pack-from-dev`, the one remaining off-catalog workflow **moved** out → `Clean` driven keys `['header','schema_version','state','workflows']`, `ActiveTask` `['header','schema_version','state','tasks','workflows']`; declared sets carry `next_steps` |
| **`(5, D1)`** — `jigc start --explain` emits a production `--format json` stdout arm `ENVELOPE_ARMS` does not carry | tier 3 | **STILL-OPEN** (expected) | §2.6: exit 0, stdout 1218 bytes, stderr 0; top-level keys `['collision_winners','overrides_applied','pack_inputs','schema_version','steps','workflow','workflow_layer']`; `ENVELOPE_ARMS` carries exactly four `start` rows and none is it |

**M52 §A axis-5 rows: 1 CLOSED · 5 STILL-OPEN.** The one tier-1 row is closed; **all five still-open rows
are tier-3** (*a surface says something the binary does not do*), which the charter triaged to 1.x — a
still-open there is expected and is **not** a new finding. No row regressed and no closed row re-opened.

M52's axis-5 file also carries its own §1 ledger — **M51's four defects, all CLOSED on rc.16**. Re-driven
on rc.17 at §4: DEFECT A (setup/uninstall's third undeclared envelope) **stays closed** — 47/47 in a
declared arm; DEFECT B (`doc show`'s undeclared array/object) **stays closed** — 8 registry rows + 9
deeper dispatch cells, zero undeclared shapes; DEFECT C (`Success` vs non-zero exit) **stays closed by
declaration** and re-driven; DEFECT D (pre-dispatch `current_dir()` bypass) **stays closed** — 47/47.

---

## 2 · Repro blocks for §1

### 2.1 — `(5, DEFECT 1)` CLOSED · the degenerate-mint axis, 4 doors × 5 titles

```
rig: fresh
$ for t in "" "   " "!!!" "日本語" "the of a"; do jigc milestone create "$t"; done
  → exit=1 ×5, each:
    blocking · write.unslugable-title — cannot mint a milestone: its id is slugged from the title,
    and this title slugs to nothing — ids are built from ASCII letters and digits, so a title in
    another script, or of stopwords only, yields none
      at: milestone
      route: re-run with a title carrying ASCII letters or digits — the milestone id is slugged from it
$ jigc milestone create "Real one"                       -> 0
$ jigc --format json milestone add-task real-one "日本語" -> 1  {"error":"blocking · write.unslugable-title … at: task"}
$ jigc --format json start --workflow single-task "!!!"  -> 1  same code, at: task     (and "" / 日本語 / "the of a")
$ git rev-parse --short HEAD   -> unmoved by every refusal
$ git status --porcelain       -> empty
$ ls .jigc/tasks               -> (absent)
```
Arm: **`Reject::Error`** (the flattened `{error}`) at all four doors — uniform, and within the declared
complement (the code is listed under **no** target form; its target is the bare work-unit *type* token
`milestone` / `task` on the doctype-scoped-blocks precedent). §7.1 records the consequence.

### 2.2 — `(5, DEFECT 2)` STILL-OPEN

```
rig: fresh
$ printf 'id: probe-step\ntitle: Probe\nbody: |\n  Probe.\n' > $RIG/ps.yaml
$ jigc config insert-step --workflow single-task --before finalize $RIG/ps.yaml   -> 0
  config: inserted step `probe-step` into `single-task` before `finalize`
$ jigc --format json start --explain --workflow single-task | jq -r '.steps[]'
  locate implement record-changelog superseded-context author-commit PROBE-STEP finalize
$ jigc --format json config remove-step 'workflow:single-task#probe-step'   -> 1
  {"error":"blocking · config.anchor-absent — no step `probe-step` body to fork
            route: name a step id present in the workflow's resolved include list, then re-run"}
$ jigc --format json config replace-step 'workflow:single-task#probe-step' $RIG/ps.yaml -> 1
  {"error":"blocking · config.step-id-collision — `probe-step` is already a step id …"}
control (a PACK step, same door, same second):
$ jigc --format json config remove-step 'workflow:single-task#implement'    -> 0  {"committed","op","target"}
```

### 2.3 — `(5, DEFECT 3)` STILL-OPEN, and wider

```
rig: committed-singletons
$ jigc --format json rename vision --to "New Vision"   -> 1
  {"error":"`vision` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`
            route: run `jigc describe` for the doctype surface"}          <- no code, no key
$ jigc doc show vision      -> 0    (the two sibling doors ACCEPT the bare form)
$ jigc doc schema vision    -> 0
the same colon-less shape at two more doors, under a SECOND wording:
$ jigc --format json doc show nosuchtype                         -> 1
  {"error":"malformed address `nosuchtype`: missing ':' between type and slug — …"}   <- no code
$ jigc --format json doc set-field 'nosuchtype#meta/k' --value v --task <t>  -> 1   same, no code
```

### 2.4 — `(5, DEFECT 4)` STILL-OPEN

```
rig: committed-singletons
$ jigc doc schema vision --format json | jq '.fields[]|select(.id=="grounded-in")'
  { "id":"grounded-in", "required": false, "set-field":"vision:<slug>#meta/grounded-in" }
$ command grep -n 'grounded-in' VISION.md     -> (absent: declared, optional, unpopulated)
$ jigc --format json doc show 'vision:vision#meta/grounded-in'   -> 1
  findings[0]: store.no-such-leaf  key {store.no-such-leaf, vision:vision#meta/grounded-in}
  "`vision:vision#meta/grounded-in` names no leaf `grounded-in` in section `meta`"
control (a populated leaf, same doc):
$ jigc --format json doc show 'vision:vision#meta/schema-version'  -> 0   "1"
```
`design/doc-read-surface.md:86` scopes the block to an **undeclared** leaf.

### 2.5 — `(5, C1)` STILL-OPEN, both arms

```
rig: fresh --pack-from-dev
$ jigc --format json start | jq -c 'keys'   -> ["header","next_steps","schema_version","state","workflows"]
$ mkdir -p $RIG/parked && mv $JIGC_PACK_DIR/workflows/ingest-existing.yaml $RIG/parked/   (a MOVE, no rm)
$ jigc --format json start                 -> 0, stderr 0 bytes
  driven keys: ['header','schema_version','state','workflows']       state=clean
  DECLARED   : ['header','next_steps','schema_version','state','workflows']
$ jigc start --workflow single-task "probe c1" >/dev/null
$ jigc --format json start
  driven keys: ['header','schema_version','state','tasks','workflows']   state=active-task
  DECLARED   : ['header','next_steps','schema_version','state','tasks','workflows']
```

### 2.6 — `(5, D1)` STILL-OPEN

```
rig: fresh
$ jigc --format json start --explain --workflow single-task   -> 0
  stdout 1218 bytes, stderr 0 bytes
  top-level keys: ['collision_winners','overrides_applied','pack_inputs','schema_version','steps',
                   'workflow','workflow_layer']
ENVELOPE_ARMS' `start` rows: OrientationView::{UnsetProject,Clean,ActiveTask} + Composed.  None is it.
```

---

## 3 · Row set A — the 64 registry arms, each driven in its declared cell

> **[DEMOTED — PARTIAL, no *per-arm* repro block].** The table below carries **one shared construction
> block** (below it) for the whole 64-row set and **no per-arm driven output**. Under the reconciliation
> rule the *set-level* claim is construction-backed, but each row's "driven key set" is not individually
> evidenced. **The reconciler independently re-drove 22 of the 64 arms with output** (rows 1–3, 5, 11,
> 13-adjacent, 15, 16, 22–24, 26–27, 28–33 as shapes, 34–37, 51–58, 63–64) — **22 / 22 matched the
> declared key set**, 0 mismatches. The remaining 42 rows stay **construction-backed, not per-row
> evidenced**. Repro: §14.3.a / §14.3.b / §14.3.c.

**64 / 64 driven. 64 / 64 declared key sets equal the driven key sets. 0 mismatches.** The
`schema_version` partition (`schema_version` rides a row **iff** its `ArmRoot` is `ResultContract`) holds
**64 / 64**. Stream discipline holds **64 / 64** (one JSON document on the arm's own stream; no JSON on
the other).

| # | leaf | arm | exit | driven key set (= declared) |
|---|---|---|---|---|
| 1 | `start` | `OrientationView::UnsetProject` | 0 | `schema_version, state` |
| 2 | `start` | `OrientationView::Clean` | 0 | `header, next_steps, schema_version, state, workflows` |
| 3 | `start` | `OrientationView::ActiveTask` | 0 | `header, next_steps, schema_version, state, tasks, workflows` |
| 4 | `start` | `Composed` | 0 | `task, text` |
| 5 | `workflow` | `Composed` | 0 | `task, text` (both `--preview` and `--task`) |
| 6 | `setup` | `Installed` | 0 | `allowlist_file, findings, guide_file, hook_committed, hook_file, install_commit, line_file` |
| 7 | `uninstall` | `TornDown` | 0 | `allowlist_file, findings, line_file, removed` |
| 8 | `upgrade` | `Swept` | 0 | `checked, findings, guide, schema_version` |
| 9 | `ingest` | `Triaged` | 0 | `findings, rows, summary` |
| 10 | `migrate` | `Composed` | 0 | `task, text` |
| 11 | `migrate-corpus` | `Report` | 0 | `already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled` |
| 12 | `unmanage` | `Report` | 0 | `dropped, identity, path` |
| 13 | `rename` | `Report` | 0 | `commit, findings, from, hook_output, new_path, old_path, prose_mentions, referrers, title, to` |
| 14 | `relocate` | `Report` | 0 | `blocked, displaced, moved` |
| 15 | `describe` | `Menu` (Unpinned) | 0 | `commands, definitions, schema_version` |
| 16 | `validate` | `StoreSweep` | 0 | `blocking_probes, findings, report_only, schema_version, scope` |
| 17 | `doc create` | `DocAck::Created` | 0 | `copied_in, existed, findings, op, target` |
| 18 | `doc add-item` | `DocAck::AddedItem` | 0 | `copied_in, findings, op, target` |
| 19 | `doc remove-item` | `DocAck::RemovedItem` | 0 | `copied_in, findings, op, removed, target` |
| 20 | `doc retitle-item` | `DocAck::RetitledItem` | 0 | `copied_in, findings, op, target, title` |
| 21 | `doc rename` | `DocAck::Renamed` | 0 | `committed_identity, copied_in, findings, from, op, reslugged, target, title` |
| 22 | `doc set-field` | `DocAck::Field` | 0 | `copied_in, findings, op, target, value` |
| 23 | `doc set-field` | `DocAck::UnsetField` | 0 | `already_absent, copied_in, findings, op, target, unset` |
| 24 | `doc set-slot` | `DocAck::Slot` | 0 | `chars, copied_in, findings, op, target` |
| 25 | `doc author` | `DocAck::Authored` | 0 | `copied_in, findings, op, target` |
| 26 | `doc show` | `WholeDoc::Committed` | 0 | `fields, item-count, schema-version, sections, slug, type` |
| 27 | `doc show` | `WholeDoc::Staged` | 0 | + `staged` |
| 28 | `doc show` | `FieldsGroupSlice` | 0 | `DataKeyed` — e.g. `base, schema-version, status` |
| 29 | `doc show` | `SlotSlice` | 0 | `Scalar` — `"Cap request rates per API key."` |
| 30 | `doc show` | `ItemArraySlice` | 0 | `ArrayOfDataKeyed` — `[{id, statement, title}]` |
| 31 | `doc show` | `ItemSlice` | 0 | `DataKeyed` — `{id, statement, title}` |
| 32 | `doc show` | `ListFieldSlice` | 0 | `ArrayOfScalars` — `["research:context-loss"]` |
| 33 | `doc show` | `CompoundFieldSlice` | 0 | `sha, short` |
| 34 | `doc schema` | `Projection` | 0 | `contract-version, fields, home, identity, schema-version, sections, type` (**contract-version 7**) |
| 35 | `doc list` | `Index` | 0 | `docs` |
| 36 | `task list` | `Rows` | 0 | `ArrayOf{id, intent, workflow}` |
| 37 | `task diff` | `Ack` | 0 | `base, code_diff, findings, op, staged_docs, task` |
| 38 | `task validate` | `Report` | 3 | `findings, schema_version` |
| 39 | `task discard` | `TaskAck::Discarded` | 0 | `commit, dropped, findings, op, task` |
| 40 | `task bind` | `TaskAck::Bound` | 0 | `findings, op, role, target, task` |
| 41 | `task finalize` | `Landed` | 0 | `committed, findings, schema_version` |
| 42 | `task finalize` | `Forecast` | 0 | `dry_run, findings, left_out, manifest, subject` |
| 43 | `task finalize` | `Blocked` | 3 | `findings, schema_version` |
| 44 | `task finalize` | `MigrationReviewHold` | **4** | `retires, rewrites, source, task` |
| 45 | `config set` | `ConfigAck::Set` | 0 | `committed, key, op, relocated, value` |
| 46 | `config insert-step` | `InsertStep` | 0 | `anchor, committed, op, side, step, workflow` |
| 47 | `config replace-step` | `ReplaceStep` | 0 | `committed, op, step, target` |
| 48 | `config remove-step` | `RemoveStep` | 0 | `committed, op, target` |
| 49 | `config fill` | `Fill` | 0 | `committed, op, target` |
| 50 | `config fork` | `Fork` | 0 | `base, committed, op, path, target` |
| 51 | `config get` | `Reading` | 0 | `key, layer, op, rejected, value` |
| 52 | `config list` | `Readings` | 0 | `knobs, op` |
| 53–57 | `milestone create/add-task/add-from-spec/provision/discard` | `RecordOnlyAck` (Unpinned) | 0 | `hook_output, text` |
| 58 | `milestone list-tasks` | `Listing` (Unpinned) | 0 | `text` |
| 59 | `milestone execute` | `Composed` | 0 | `task, text` |
| 60 | `milestone join` | `Report` | 0 | `findings, milestone, no_docs_from, overlay, schema_version` |
| 61 | `milestone finalize` | `Landed` | 0 | `committed` |
| 62 | `milestone finalize` | `Blocked` | 3 | `findings, schema_version` |
| 63 | *(cross-cutting)* | `Reject::Error` | 1/2 | `error` |
| 64 | *(cross-cutting)* | `Reject::Findings` | 1/3 | `findings, schema_version` |

### Repro block — row set A (the shared construction)

```
# each corpus built by DRIVING the binary, never by writing into .jigc/
rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

(fresh)                 rows 1-5, 7-8, 15-16, 36-58 + the milestone lifecycle 53-62
(bare)                  row 1 (UnsetProject: `jigc start` before `jigc setup`), 6, 9, 10-11
(committed-singletons)  rows 12-14, 26, 28, 33-35, and the nested `doc show` depths at §6
(refs-post-hoc)         row 32 (the `card: "0..*"` ref `vision:vision#meta/grounded-in`)
(fresh --pack-from-dev --schema note <f>)  row 14 (`relocate` needs a freeze-exempt doctype)

# the arms that need a built-up state, each driven end to end:
#  41/42/43  jigc start --workflow quick-fix … ; doc set-field commit:<t>#header/type --value chore ;
#            doc set-slot commit:<t>#summary --from-file - ; git add <file> ; task finalize <t> [--dry-run]
#  44        a committed foreign docs/direction.md → `jigc migrate … --as vision` →
#            `jigc doc author vision --from-file <payload>` → `jigc task finalize <t>`   (exit 4)
#  40        `plan` → author a spec → finalize → `start --workflow implement-from-spec` →
#            `jigc task bind spec spec:auth-tokens build-auth`
#  25/30/31  `doc author spec --from-file <payload>` with the `<<…>>` slot grammar
#  59-62     milestone create → add-task ×N → provision → (per worktree: fill the commit doc, git add)
#            → join → finalize
```

Two arms needed a **non-default run mode** to reach their declared cell, and both are recorded rather
than assumed: `config remove-step` reaches `RemoveStep` only over a **pack** step
(`workflow:single-task#implement`) — over a step `insert-step` created it takes the reject arm, which is
`(5, DEFECT 2)`; and `uninstall` reaches `TornDown` only on a pristine `fresh` rig — over a recorded
config delta or a parked `.jigc/displaced/` it refuses (correctly, §7.3).

---

## 4 · Row set B/C/D — the Unpinned reasons, the pre-pin deletes, the reject funnels

### 4.1 — `Unpinned(<reason>)` · **7 / 7**, every `still_pinned` key on the wire

> **[DEMOTED — no repro block].** The ✓ table below carries no commands and no driven output.
> **Re-driven by the reconciler: 7 / 7 hold exactly as stated** — repro at §14.3.b. Promoted back on
> the reconciler's evidence, not the driver's.

| row | `still_pinned` | on the wire |
|---|---|---|
| `describe \| Menu` | `["schema_version"]` | ✓ (and under all three of `--workflows` / `--doctypes` / `--commands`, same key set) |
| `milestone create / add-task / add-from-spec / provision / discard \| RecordOnlyAck` | `["hook_output"]` | ✓ ×5 |
| `milestone list-tasks \| Listing` | `[]` | ✓ — `text` alone, **no `hook_output`** |

### 4.2 — the four pre-pin deletes · **4 / 4 absent**

```
rig: bare
$ jigc --format json setup      -> keys …; `installed`   ABSENT
$ jigc --format json uninstall  -> keys …; `uninstalled` ABSENT
$ (the exit-4 hold, §3 row 44)  -> keys retires,rewrites,source,task; `review` ABSENT
$ jigc --format json milestone list-tasks <m> -> keys ["text"]; `hook_output` ABSENT
```

### 4.3 — outside a git repository · **47 / 47 in a declared arm** (M51 DEFECT A's cell)

```
$ OUT=$(mktemp -d); python3 sweep.py "$OUT"      # every VERB_KINDS leaf, minimal argv, --format json
roll-up: ('Reject::Error', exit 1): 45   ('Reject::Findings', exit 1): 2   total 47
NOT-JSON leaves: none.   stdout 0 bytes on all 47; the document is stderr's.
the 2 findings-arm leaves are `setup` (setup.repo-root) and `uninstall` (uninstall.repo-root).
zero bare-`Finding` roots — M51 DEFECT A stays CLOSED.
```

### 4.4 — the cwd deleted under the process · **47 / 47** (M51 DEFECT D's cell)

```
$ for each leaf:  sh -c 'cd $d && rmdir $d && exec jigc --format json <leaf argv>'
roll-up: ('Reject::Error', exit 1): 47   total 47   NOT-JSON: none
stderr: {"error": "cannot determine the current directory: No such file or directory (os error 2)"}
```

### 4.5 — the clap carve-out (declared: **no** JSON on either stream) · **5 / 5**

```
$ jigc --format json --version      -> 0  stdout non-JSON, stderr non-JSON
$ jigc --format json --help         -> 0  "
$ jigc --format json doc --help     -> 0  "
$ jigc --format json nosuchverb     -> 2  "
$ jigc --format json doc nosuchverb -> 2  "
```

---

## 5 · Row set E — M53's own new surfaces, driven

### 5.1 — the instrument control, and the `displaced` key at `task finalize`

```
rig: fresh; jigc start --workflow quick-fix "displace probe"; fill the commit doc; echo z > d.txt; git add d.txt
$ echo PLANTCANARY-AREA > .jigc/tasks/displace-probe/plant-root.txt
$ mkdir -p .jigc/tasks/displace-probe/docs && echo PLANTCANARY-DOCS > .../docs/plant-docs.txt
BEFORE-control:
$ command grep -rl 'PLANTCANARY' .jigc/   -> 2 hits, rc 0        <- the control finds the plant
$ grep -rl 'PLANTCANARY' .jigc/           -> rc 1, no hits       <- the shell function; unsound
$ jigc --format json task finalize displace-probe   -> exit 0
  top-level: ["committed","findings","schema_version"]      (the declared Landed key set)
  committed.displaced: [{from ".jigc/tasks/displace-probe/docs/plant-docs.txt",
                         to   ".jigc/displaced/displace-probe/docs/plant-docs.txt"},
                        {from ".jigc/tasks/displace-probe/plant-root.txt", to ".jigc/displaced/…"}]
  findings: []                                              (no advisory — nothing was left standing)
  stderr: the two pairs, named, as a loss-shaped side channel
AFTER:  command grep -rl 'PLANTCANARY' .jigc/  -> both under .jigc/displaced/, rc 0.  Zero loss.
```

**Control (zero-false-fire), same rig, an ordinary task:** `committed.displaced: []`, `findings: []`,
`.jigc/displaced` **not created**, stderr silent — `displaced` is **present always**, as
`command-output-contract.md:526` declares.

### 5.2 — `finalize.foreign-bytes` on the `task finalize` findings arm (a faulted move)

```
rig: fresh; a quick-fix task with its commit doc filled and f.txt staged
$ echo FAULTCANARY > .jigc/tasks/fault-probe/foreign.txt
$ printf 'not a directory\n' > .jigc/displaced          # the parking home made a FILE, so every move faults
BEFORE-control: command grep -rl FAULTCANARY .jigc/ -> 1 hit, rc 0
$ jigc --format json task finalize fault-probe   -> exit 0
  committed.displaced: []
  findings: [{ severity "advisory", code "finalize.foreign-bytes",
               key {finalize.foreign-bytes, "task:fault-probe"},
               location.address "task:fault-probe",
               message "`.jigc/tasks/fault-probe` holds 1 path(s) jigc did not write … 1 of them could
                        not be moved aside: `.jigc/tasks/fault-probe/foreign.txt` — could not open
                        .jigc/displaced/fault-probe to park it: Not a directory (os error 20)",
               route "the commit landed and nothing in it is affected. Keep what you need …" }]
AFTER: command grep -rl FAULTCANARY .jigc/ -> still at .jigc/tasks/fault-probe/foreign.txt.  Zero loss.
$ git log --oneline -1 -> chore: fault probe        (the commit landed; the advisory did not block it)
```
**Matches `design/command-output-contract.md:347` exactly** — advisory, exit 0, work-unit-ref target, the
path set a post-unwind re-read, one advisory per area.

### 5.3 — `milestone finalize`: the walked `merged/`, and the declared stderr-only asymmetry

```
rig: fresh; milestone cache-rework, 2 sub-tasks, provisioned, staged, joined
$ A=.jigc/milestones/cache-rework
$ echo MPLANT-ROOT > $A/mroot.txt ; echo MPLANT-MERGED-TOP > $A/merged/mtop.txt
$ echo MPLANT-MERGED-DOCS > $A/merged/docs/mdocs.txt ; echo MPLANT-MERGED-DIR > $A/merged/sub/deep.txt
BEFORE-control: command grep -rl MPLANT .jigc/ -> 4 hits, rc 0
$ jigc --format json milestone finalize cache-rework   -> exit 0
  top-level: ["committed"]                             (the declared Landed key set — unmoved)
  committed keys: commits, displaced, files, hash, hook_output, manifest, still_staged, sub_tasks, subject
  committed.displaced: all FOUR pairs, incl. `merged/sub` moved WHOLE as one entry
AFTER: all four under .jigc/displaced/cache-rework/…, rc 0.  Zero loss, two levels into merged/.
```

**The faulted cell at the boundary, the declared bound honoured** (rig `fresh`, `.jigc/displaced` made a
file): exit **0**, `committed.displaced: []`, top-level keys still `["committed"]` — **no `findings`
key** — and the advisory on **stderr** as the house findings line, `at: milestone:fault-wave`. The bytes
survive. `command-output-contract.md:361` declares that asymmetry and claims the identity still reaches
the invocation log; **driven with `invocation-log true`**:

```
tail -1 .jigc/logs/invocations.jsonl
{"argv":["--format","json","milestone","finalize","log-wave"], "exit_code":0,
 "finding_codes":["finalize.foreign-bytes"], "binary_version":"1.0.0-rc.17", "error_code":null}
```

### 5.4 — the union key over N sub-task areas, and N advisories keyed apart

```
rig: fresh; milestone union-wave, sub-one + sub-two, provisioned, staged, joined
plants in all THREE areas (2 sub-task + 1 milestone)
(a) moves succeed:  committed.displaced = the 3-pair UNION, sorted by `from`; 3 per-area stderr notes;
                    AFTER: all three under .jigc/displaced/{union-wave,sub-one,sub-two}/.  Zero loss.
(b) moves fault (.jigc/displaced a file):  exit 0, displaced [], top-level ["committed"],
    THREE `finalize.foreign-bytes` advisories on stderr with THREE distinct targets —
      milestone:fault-union · task:sub-one · task:sub-two
    invocation log: finding_codes ["finalize.foreign-bytes","finalize.foreign-bytes","finalize.foreign-bytes"]
    AFTER: all three plants still in place.  Zero loss.
```
`command-output-contract.md:528`'s *union over every sub-task the boundary settled* and :347's *keys them
apart* both hold.

### 5.5 — the destination-collision bound: `displaced.to` reports the **real** destination

```
rig: fresh; the same slug minted, planted and landed twice (plus a sibling slug in between)
1st: to ".jigc/displaced/first-probe/same-name.txt"
2nd (a different task): to ".jigc/displaced/second-probe/same-name.txt"
3rd (the SAME slug again): to ".jigc/displaced/first-probe/same-name.txt.2"      <- suffix-resolved
contents: …/same-name.txt=CANARY-A  …/same-name.txt.2=CANARY-C  second-probe/same-name.txt=CANARY-B
```
No clobber, and the pinned key names the destination that actually exists.

### 5.6 — the residual rule at all **25** `WORK_UNIT_ID_DOORS`, plus the enumerating doors

```
rig: fresh
$ mkdir -p .jigc/tasks/ghost-task/docs && echo RESIDCANARY > .jigc/tasks/ghost-task/docs/note.md
$ mkdir -p .jigc/milestones/ghost-ms   && echo MRESIDCANARY > .jigc/milestones/ghost-ms/note.txt
BEFORE-control: command grep -rl 'RESIDCANARY\|MRESIDCANARY' .jigc/ -> 2 hits, rc 0
enumerating doors:   jigc task list -> []   ·   jigc start -> state=clean, tasks absent
17 task-side doors  (start --task · workflow --task · the 10 doc verbs · doc list --task ·
                     task diff/validate/discard/finalize/bind):
  every one exit 1, arm Reject::Findings, code finalize.no-task, target "task:ghost-task",
  message "…`.jigc/tasks/ghost-task` is a directory carrying no base pin, so it is a leftover
           and not a work unit…", route naming the REPO-RELATIVE path.  No host-absolute path anywhere.
 8 milestone-side doors (add-task · add-from-spec · list-tasks · provision · execute · join ·
                         finalize · discard):
  every one exit 1, arm Reject::Findings, code milestone.unknown, target "milestone:ghost-ms",
  the same residual sentence.
AFTER: both canaries untouched.  25/25 keyed, 0 flattened, 0 host paths, 0 bytes moved.
```

**The two `serial-collision` cells** (same rig, same residuals), **flattened** `{error}` with the residual
sentence and the by-hand route: `jigc start --workflow single-task "ghost task"` →
`task.serial-collision`; `jigc milestone create "ghost ms"` → `milestone.serial-collision`. §7.1 records
that arm choice against its declaration.

### 5.7 — `milestone.terminal`'s new producer (§8), and its six shipped siblings

```
rig: fresh; milestone term-wave with one sub-task, joined and FINALIZED
$ SNAP=$(mktemp -d); cp -R .jigc/tasks/sub-alpha $SNAP/   (before the boundary)
$ jigc milestone finalize term-wave  -> 0   (the area is torn down)
$ cp -R $SNAP/sub-alpha .jigc/tasks/        (restore jigc's OWN area bytes, base.json included)
$ jigc --format json task discard sub-alpha  -> 1
  {"error":"blocking · milestone.terminal — sub-task `sub-alpha` of milestone `term-wave` is already
            `joined` on the committed record — its working area is a leftover, not live work, and
            discarding it would settle an item the record has already settled
            at: task:sub-alpha
            route: … `jigc doc show milestone-record:term-wave`"}
$ git log --oneline -1  -> unmoved (no record commit)
the six shipped producers, same corpus: milestone provision/execute/join/finalize/discard <terminal-id>
  -> all exit 1, all Reject::Error, all `milestone.terminal`, at: milestone:<id>
```
**One code, one arm, at all seven producers.** The new producer does not introduce a second spelling.

### 5.8 — `milestone discard --force` over a sub-task area holding a foreign byte (§3's cell)

```
rig: fresh; milestone discard-wave provisioned; echo DISCARDCANARY > .jigc/tasks/sub-one/foreign.txt
$ jigc --format json milestone discard discard-wave     -> 1  Reject::Findings
    milestone.foreign-bytes, target null (a declared singleton), the path named
$ jigc --format json milestone discard discard-wave --force -> 0  {"hook_output","text"}
    text: "discarded milestone:discard-wave (1 sub-task(s); workbench removed)"
    stderr: "warning: removing the working area … discards work that is not in git: …
             note: the working area is the only copy of these bytes — they are not recoverable."
AFTER: command grep -rl DISCARDCANARY .jigc/ -> rc 1.  Taken, as §3 decides, under the single consent.
```

### 5.9 — `FINALIZE_MESSAGE_FILE` (audit fix 1): an area left standing holds no file jigc wrote

Across §5.2 and §5.3's faulted cells the advisory's path set and the narration's count are **1**, and the
path named is the plant — never `finalize-message.tmp`. In §5.8 the refusal names one path, not two. The
e2e defect's symptom (a refusal at exit 1 *over nothing foreign*, and an off-by-one count) did not
reproduce at any door.

---

## 6 · Row set F/G — `doc show`'s dispatch space, driven at every depth · 0 undeclared shapes

> **[DEMOTED — no repro block].** The address × shape table below carries no commands and no driven
> output. **Re-driven by the reconciler over all ten addresses on `committed-singletons`: every row
> holds, 0 undeclared shapes** — repro at §14.3.c. Promoted back on the reconciler's evidence.

Eight registry arms (§3 rows 26–33) plus nine deeper address forms on the committed `changelog`
(a nested repeatable: `releases > changes`), **all on `committed-singletons`**:

| address | driven shape | declared arm |
|---|---|---|
| `changelog:changelog` | `Object` | `WholeDoc::Committed` |
| `#releases` | `ArrayOfDataKeyed` `[{changes,date,id,link,title}]` | `ItemArraySlice` |
| `#releases/1-0-0` | `DataKeyed` | `ItemSlice` |
| `#releases/1-0-0/date` · `/link` | `Scalar` | field leaf |
| `#releases/1-0-0/changes` | `ArrayOfDataKeyed` `[{category,id,notes}]` | `ItemArraySlice` (nested) |
| `#releases/1-0-0/changes/added` | `DataKeyed` | `ItemSlice` (nested) |
| `#releases/1-0-0/changes/added/notes` · `/category` | `Scalar` | slot / field leaf |
| `#unreleased-changes/changed/notes` | `Scalar` | slot leaf |

**No nested depth produces a shape outside the six declared `ArmShape` members.** M51 DEFECT B's class
stays closed one level deeper than it was closed at.

---

## 7 · Row set H — `ENVELOPE_OWED_CODES` × their producers, and the target forms

### 7.1 — the arm · driven at 14 coordinates

> **[DEMOTED — no repro block, and the count is wrong].** The paragraph below carries no commands and
> no driven output of its own, and its own enumeration names **15** coordinates (11 `unknown-type` + 2
> `not-found` + 1 `no-such-leaf` + 1 `fixed-identity`), not the 14 it reports. **Re-driven by the
> reconciler at all 15: 15 / 15 on the owed findings arm, 0 flattened** — repro at §14.3.d. Promoted
> back with the count corrected **14 → 15**.

`store.unknown-type` reaches the **findings envelope** at `migrate --as`, `relocate`, `rename`,
`doc show`, `doc schema`, `doc set-field`, `doc set-slot`, `doc add-item`, `doc remove-item`,
`doc retitle-item`, `doc rename`. `store.not-found` at `rename` and `doc show`. `store.no-such-leaf` at
`doc show`. `store.fixed-identity` at `doc show`. **M52's audit fix 4 holds: 14 / 14 on the owed arm, 0
flattened.** (`task bind` and `milestone add-from-spec` answer an earlier guard on the argv I could
reach them with — recorded as not-driven at §9.2, not as a pass.)

### 7.2 — **DEFECT A (new)** · one code, two target forms

The **target form**, which the arm sweep does not ask, diverges at exactly one door.

```
rig: committed-singletons
$ jigc --format json doc show 'nosuchtype:x'
  findings[0].key    -> {"code":"store.unknown-type","target":"nosuchtype:x"}     <- URI-shaped
  findings[0].message-> "unknown doctype `nosuchtype` for `nosuchtype:x`"
  location.address   -> "nosuchtype:x"
$ jigc --format json doc show 'nosuchtype:x#meta'        -> same, target "nosuchtype:x"
$ jigc --format json doc show 'nosuchtype:x' --task <t>  -> same, target "nosuchtype:x"

the same code, same condition, NINE other producers, same second:
$ jigc --format json doc schema      nosuchtype                       -> target "nosuchtype"
$ jigc --format json doc set-field   'nosuchtype:x#meta/k' --value v --task <t> -> target "nosuchtype"
$ jigc --format json doc set-slot    'nosuchtype:x#body' --from-file /dev/null --task <t> -> "nosuchtype"
$ jigc --format json doc add-item    'nosuchtype:x#s' --title T --task <t>      -> "nosuchtype"
$ jigc --format json doc remove-item 'nosuchtype:x#s/i' --task <t>              -> "nosuchtype"
$ jigc --format json doc retitle-item 'nosuchtype:x#s/i' --title T --task <t>   -> "nosuchtype"
$ jigc --format json doc rename      'nosuchtype:x' --to Y --task <t>           -> "nosuchtype"
$ jigc --format json rename          'nosuchtype:x' --to Y                      -> "nosuchtype"
$ jigc --format json relocate        nosuchtype --from docs/old/                -> "nosuchtype"
$ jigc --format json migrate         CHANGELOG.md --as nosuchtype               -> "nosuchtype"
  (message at all nine: "unknown doctype `nosuchtype`")
```

`design/command-output-contract.md:237` fixes the form and **names `doc show`'s own read path as the
precedent for it**: *"it is the form the read path **already** emits for a doctype-scoped block
(`store.unknown-type` addresses the bare doctype — `Location::addressed(doctype, …)`) … It is
deliberately **not** URI-shaped — a bare id cannot be mistaken for a doc address a driver could
`doc show`."* Driven, the one door that sentence cites is the one door that does not obey it.

**What it costs a driver:** `(code, target)` is the contract's stable per-instance identity, so a driver
or an acknowledge-ledger sees two different keys for one condition depending on which verb it asked —
and the key `doc show` emits (`nosuchtype:x`) is exactly the doc-address shape the sentence forbids,
which a driver may follow back into `jigc doc show`. **Tier 3** on the charter's predicate (a surface
says something the binary does not do): exit 1 at a read door, no write, no loss, no repository harm.

### 7.3 — the destroying doors' refusal envelope · 3 driven

```
rig: fresh; a provisioned milestone + a plain task, one foreign byte planted in each task area
$ jigc --format json milestone discard dd-wave -> 1  ENV  milestone.foreign-bytes     target null
$ jigc --format json uninstall                 -> 1  ENV  uninstall.foreign-bytes     target null
$ jigc --format json task discard plain-task   -> 1  ENV  task-discard.foreign-bytes  target null
AFTER: both canaries in place, rc 0 on the control grep.
```
All three `target: null` values are **declared singletons** (`engine::finding::is_declared_singleton`
carries `milestone.foreign-bytes`, `task-discard.foreign-bytes` and the whole `uninstall.` prefix), so
the closure claim holds. `milestone provision` re-provisioned at exit 0 over the same plant — its subject
is worktree paths, which is axis 3's row, not this one's.

### 7.4 — the posture refusal's arm across **16** git states (M53's tenth member included)

```
for g in merge squash-merge rebase-merge rebase-apply am cherry-pick sequencer dangling-sequencer
         uncommitted-pick uncommitted-pick-range uncommitted-pick-conflicted uncommitted-pick-resolved
         revert unmerged-index bisect detached
  rig=$(dev/jigc-rig fresh --git-state $g …);  jigc --format json task finalize nosuch
15 × Reject::Error `repo.operation-in-progress` with the member's own noun;
 1 × Reject::Error `repo.head-detached` (detached).
```
Uniform. M53's four `UncommittedCherryPick` shapes answer the same arm with the same noun as the six
shipped members. This is M52's recorded observation 2 (*every posture refusal carries a `Finding` and
still takes `{error}`*) — declared in M52's VERDICT under F4 with a trigger, **not re-reported as new**.

---

## 8 · Row set I — the exit-code cell and `STORE_EXIT_FLIPS`

```
rig: committed-singletons
control:  jigc --format json validate -> exit 0, keys as declared, 1 advisory
$ git rm --cached CHANGELOG.md && git commit -m … && rm -f CHANGELOG.md
$ jigc --format json validate -> exit 1, key set UNCHANGED
  blocking reconciliation.rename        CHANGELOG.md      (member `oob-rename`)
  blocking schema-conformance.home-vacated CHANGELOG.md   (member `home-vacated`)
```
Two of the seven members driven; the key set is invariant under the flip, which is DEFECT C's
declaration (*`ArmOutcome::Success` is not a claim that the verb can never exit non-zero*) holding at two
more cells. The other five need axis 7's fixture work and are listed at §9.

Other non-zero `Success`-arm cells re-driven: `task validate` exit 3, `task finalize --dry-run` exit 3
over a blocking gate, `migrate-corpus --dry-run` exit 0 on a clean corpus. **No fourth cell found** where
a `Success` row ships its key set at an *undeclared* exit.

---

## 9 · What was NOT driven, and why — stated plainly

1. **`STORE_EXIT_FLIPS` at 5 of its 7 members** (`probe-unreliable`, `unmigrated-corpus`,
   `ahead-corpus`, `orphaned-instance`, `foreign-squatter`). Each needs a manufactured pack, a
   down-stamped corpus or a removed doctype — axis 7's fixture work. What this axis owes is the
   **key-set invariance under the flip**, and that is driven at the two members above.
2. **`ENVELOPE_OWED_CODES` at `task bind` and `milestone add-from-spec`.** Both answered an earlier
   guard on every argv I could build without a longer fixture (`task-bind.undeclared-role`,
   `milestone.unknown`). Not presented as driven; the code's arm is driven at 14 other coordinates and
   the selection is made once at `render::carrier`, not per door.
3. **`RelocateRefusal::ALL` and `RefusalKind::ALL` member-by-member.** `relocate` and `rename` are driven
   at their success arms and at three refusals between them; the per-member refusal subjects are axis
   1's and axis 3's fixture work, and the **envelope** claim they would test here is driven at §7.
4. **`ManifestKind::ALL` (6) and `SchemaChangeKind::ALL × LOCI` (18).** They shape values *inside*
   `task finalize | Forecast`'s `manifest` and `migrate-corpus | Report`'s report, not any envelope's
   top-level key set. Both carrying envelopes are driven (§3 rows 42 and 11). Axes 4 and 7 own them.
5. **`DESTROYING_DOORS` at `milestone provision`** (1 of 6, in the foreign-bytes cell). Its subject is a
   worktree path, not a task-area complement; driving it is axis 3's row.
6. **A genuine concurrent / Task-tool fan-out envelope.** `milestone join` and both `milestone finalize`
   arms were driven single-process through real provisioned worktrees. The genuine spawn is the standing
   honest bound M51's VERDICT carries, unchanged by this review.
7. **`ENVELOPE_ARMS` under a project-layer pack shadow.** No row's key set is derived from a doctype and
   the one pack-sensitive row (`describe | Menu`) is `Unpinned`; the one manufactured pack I did need
   (`relocate | Report`, via `--schema note`) is at §3. Axis 7 owns the pack axis.
8. **The `hook_output` *value* across the whole `COMMITTING_DOORS` producer axis.** Driven at
   `task finalize` and `milestone finalize` to confirm the key is on the wire; the per-producer value is
   axis 4's.

---

## 10 · Observations that are **not** defects (each with its declaration)

1. **`write.unslugable-title` takes the flattened `{error}` arm at all four mint doors.** Its target is
   the bare work-unit *type* token (`milestone` / `task`), the doctype-scoped-blocks form, and the code
   is listed under **no** target form in the contract's table — so the flattened arm is the declared
   default (`command-output-contract.md` → *the complement*). Uniform at four producers.
2. **The two `serial-collision` codes flatten while the residual `finalize.no-task` / `milestone.unknown`
   cells envelope — at the same doors.** Both are individually declared (`finalize.no-task` is listed
   under the work-unit form and is an M51 Increment 6 door; the `serial-collision` codes are listed under
   none), and each is uniform across its own producers. Recorded because at `jigc start` /
   `jigc milestone create` a driver sees two machine shapes for two neighbouring "this id is not live
   work" states; it is the declared-broader-rule class M52's VERDICT flagged under F4 with a trigger.
3. **`milestone finalize`'s advisory is stderr-only, and that is the declared bound**
   (`command-output-contract.md:361`, M53 VERDICT → Declared bounds). Graded against, not re-found: the
   identity reaches the invocation log, and §5.3/§5.4 confirm both halves.
4. **The colon-less-address bail is code-less at `doc show` and the `doc` write verbs too, under a second
   wording** — a widening of `(5, DEFECT 3)`, which M52 reported at `rename`. Not minted as a separate
   finding: one class, one still-open row.
5. **`jigc uninstall` refuses over `.jigc/displaced/`.** After any displacement the parking home holds
   bytes no index has a copy of, so the teardown blocks with `uninstall.foreign-bytes` and a route that
   says plainly *"jigc has no verb that clears `.jigc/displaced/`"* and names `--force`. Declared at the
   M53 VERDICT (*the residual cleared by hand*); driven, and the route is followable.
6. **`doc show`'s `--task` staged arm carries `staged` and nothing else new**, and the eight `doc show`
   rows all reach their declared shape from the staged copy as well as the committed one.

---

## 11 · Defects

### DEFECT A — `store.unknown-type` emits a **URI-shaped** target at `jigc doc show` where the contract fixes the bare doctype id, and nine sibling producers emit the bare form

**Tier 3** (a surface says something the binary does not do). Repro, contradiction and cost: **§7.2**.
New — M52's axis-5 §8 drove this code's **arm** at 36 coordinates and passed it 36/36; the **target
form** is a different cell and no M52 §A row carries it.

*(No other defect was found on this axis. In particular: no tier-1 row — every arm that reports a landed
commit was checked against `git rev-parse HEAD` and `git log`, every displacement claim against a
`command grep` before-control, and no `--format json` surface on this axis reported success over a byte
that died.)*

---

## 12 · Counts

| | |
|---|---|
| rows driven | **≈303** — A 64 registry arms · B 7 unpinned · C 4 deletes · D 99 reject-funnel cells (47 outside-repo + 47 deleted-cwd + 5 clap) · E 24 M53-surface cells (displacement loci, fault cells, union cells, collision, `milestone.terminal`, `--force`) · F 25 residual `WORK_UNIT_ID_DOORS` + 2 serial-collision + 2 enumerating · G 9 nested `doc show` depths · H 14 owed-code + 10 target-form + 3 destroying-door + 16 posture cells · I 5 exit-code cells · J 13 degenerate-mint cells · K 16 alternate run-modes |
| rows not applicable / not driven | **8 classes** (§9) |
| declared key set == driven key set | **64 / 64** |
| `schema_version` ⇔ `ArmRoot::ResultContract` | **64 / 64** |
| stream discipline | **64 / 64**, plus **94 / 94** across the two uniform 47-leaf sweeps |
| the four pre-pin deletes absent | **4 / 4** |
| `Unpinned(<reason>)` rows whose `still_pinned` keys are on the wire | **7 / 7** |
| clap carve-out rows conforming | **5 / 5** |
| `ENVELOPE_OWED_CODES` producers on the owed **arm** | **14 / 14** |
| undeclared envelope **shapes** found | **0** |
| **M52 §A axis-5 rows CLOSED** | **1 / 6** (the one tier-1 row) |
| **M52 §A axis-5 rows STILL-OPEN** | **5 / 6** — all tier-3, all triaged to 1.x, none a new finding |
| **M51 rows (carried through M52) re-checked** | **4 / 4 still CLOSED** |
| new defects | **1** (DEFECT A, tier 3) |
| **tier-1 rows on this axis** | **0** |
| `doors_covered` (leaves that are the door of ≥1 driven row) | **47 / 47** |

---

## 13 · What this adds over flow-54 arm 2 (and arms 1, 3, 5)

Flow 54's five arms iterate M53's own classes against the **debug** binary in the build tree. This review
re-drives the **pinned-contract** question on the installed **release `1.0.0-rc.17`** — a different
binary, a different posture (no `#[cfg(debug_assertions)]` route fence), built *after* the audit's seven
fixes — and then asks what those arms cannot:

- **Arm 1 iterates `DESTROYING_DOORS ▸ Disposition` × six plant loci; arm 2 iterates the move/unwind
  shape space.** Both assert *where the bytes are*. Neither asserts **which envelope key set a driver
  reads back**, which is this axis's whole cell set. §5.1–§5.5 drive the same classes through
  `--format json` and check `committed.displaced` against the filesystem in both directions: present
  always, sorted, repo-relative, the **real** destination after suffix resolution, `[]` on the ordinary
  path — and the top-level key set **unmoved** at both doors, which is the 1.0 pin the wave promised not
  to touch.
- **Arm 2 counts plants; it was green over the `finalize-message.tmp` defect for exactly that reason**
  (VERDICT: *"arm 2 counted plants, never the complement's contents"*). §5.9 asks the complement
  question at every faulted and refusing door instead, and the symptom does not reproduce.
- **Arm 3 iterates `WORK_UNIT_ID_DOORS` for the residual *sentence*.** §5.6 iterates the same 25 rows for
  the **machine key**: `(code, target)` non-null at 25/25, `Reject::Findings` at 25/25, zero flattened,
  zero host-absolute paths — the four proofs' question asked over a registry the four proofs do not
  iterate.
- **The four `ENVELOPE_ARMS` proofs all iterate `ENVELOPE_ARMS`.** *"Is there a production arm the
  registry does not carry?"* is outside all four by construction — it is where M51's DEFECTS A and B
  lived and where M52's D1 lives. I asked it by driving: 94 uniform-fault cells across two states × 47
  leaves, 16 alternate run-modes, 9 `doc show` depths below the eight declared rows, 16 git states.
  **Zero** new undeclared shapes came back, and D1 is confirmed still-open rather than re-discovered.
- **Proof 4 compares declared keys to driven keys; nothing compares a *target form* to the form the
  contract fixes.** DEFECT A fell out of that gap: the arm is right at all fourteen coordinates and the
  key is wrong at three of them, at the one door the contract's own sentence cites as the precedent.
- **Arm 5 drives `MINT_DOORS` for the refusal-before-any-write property.** §2.1 re-drives the same five
  titles on the release binary for the **arm and the exit**, which is what closes `(5, DEFECT 1)` as an
  axis-5 row rather than as a state claim.

---
---

# 14 · Reconciliation ledger — the reconciler's section

**Inputs.** The Opus driver table above (`driver/axis5.md`) and the Codex source pass
(`codex/axis5-codex.md`, read-only review of source at `84db7551`, **no binary driven**).
**Binary.** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.17`, asserted before anything ran.
**Rigs.** `rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
(two-step eval; `mktemp -d` roots; no teardown; no `rm` on a variable path).

**The rule applied.** Every Codex claim was entered as `lead(codex, …)` and then **driven** — to a
repro block or to a refutation with the falsifying datum. Nothing was promoted on the source read.

---

## 14.1 — Codex claims → verdicts

### `lead(codex, 1)` — `jigc start --explain --format json` is an unregistered production stdout envelope → **CONFIRMED**

Driven. It is the same condition as the driver's `(5, D1)`, reached independently from the source side.

```
rig: fresh
$ jigc --version                                   -> jigc 1.0.0-rc.17
$ jigc --format json start --explain --workflow single-task
  exit = 0 · stdout 1218 bytes · stderr 0 bytes
  top-level keys: ["collision_winners","overrides_applied","pack_inputs","schema_version",
                   "steps","workflow","workflow_layer"]
source leg (read, not driven): crates/cli/src/render.rs:6066 ENVELOPE_ARMS carries exactly FOUR
  `start` rows — OrientationView::{UnsetProject, Clean, ActiveTask} + Composed.  None is a
  ResolutionTree/explain row.
```

**Widened by the reconciler:** the claim's repro uses `--workflow single-task`; the **bare** form emits
the identical unregistered shape.

```
$ jigc --format json start --explain            -> exit 0, same seven keys
```

Codex's confidence (High) and its tiering (not tier 1) both hold: exit 0, a read door, no write, no loss.
**Origin: codex — but not a *new* finding.** It is `(5, D1)`, already an open M52 §A row (tier 3,
triaged to 1.x). The two passes agree, from opposite directions.

### `lead(codex, 2)` — the two pinned orientation rows declare `next_steps` although a reachable composition omits it → **CONFIRMED**, both arms

Driven. Same condition as the driver's `(5, C1)`.

```
rig: fresh --pack-from-dev
baseline: $ jigc --format json start | jq -c 'keys'
          ["header","next_steps","schema_version","state","workflows"]
$ mkdir -p $RIG/parked && mv $JIGC_PACK_DIR/workflows/ingest-existing.yaml $RIG/parked/   (a MOVE, no rm)
   (the dev-pack floor ships no `planning`, so the composed pack now ships NEITHER off-catalog verb)

arm Clean:
$ jigc --format json start                 -> exit 0, stderr 0 bytes
  driven keys: ["header","schema_version","state","workflows"]        .state = "clean"
  DECLARED   : ["header","next_steps","schema_version","state","workflows"]     (render.rs:6079-6085)

arm ActiveTask:
$ jigc start --workflow single-task "probe c1 recon" >/dev/null
$ jigc --format json start                 -> exit 0
  driven keys: ["header","schema_version","state","tasks","workflows"]  .state = "active-task"
  DECLARED   : ["header","next_steps","schema_version","state","tasks","workflows"]  (render.rs:6096-6103)
```

Codex's source leg is accurate: the producer at `crates/cli/src/orient.rs:120-126` filters
`OFF_CATALOG_VERBS` on `pack.list(Workflows)` membership and legitimately yields an empty vector, and
`skip_serializing_if = "Vec::is_empty"` then drops the key. **Origin: codex — not a *new* finding**; it
is `(5, C1)`, an open M52 §A row (tier 3, triaged to 1.x).

### `lead(codex, census)` — *"the sole uncovered stdout producer is `render::explain`"* → **OPEN LEAD**

**Reason: a negative universal over all production stdout producers is not establishable by driving a
finite argv set, and the claim is explicitly a source read ("Every CLI JSON stdout producer I found").**
Its *positive* half is `lead(codex, 1)` and is CONFIRMED above. Its *universal* half is left open, with
the driven evidence that bears on it recorded rather than spent:

- the driver's two uniform 47-leaf sweeps (§4.3 outside-repo, §4.4 deleted cwd) — **94 cells, 0
  NOT-JSON, 0 undeclared shapes**; the outside-repo half **independently reproduced by the reconciler**
  (§14.3.a);
- the driver's §6 nine nested `doc show` depths — **re-driven by the reconciler, 10 / 10 inside the six
  declared `ArmShape` members** (§14.3.c);
- an **alternate-run-mode sweep the reconciler added** (14 cells: `validate` · `doc list` ·
  `doc list --task` · `task diff` · `task list` · `describe --commands` · `migrate-corpus --dry-run` ·
  `doc show --task` · `workflow --preview` · `config get` · `config list` · both `--explain` forms) —
  **every cell inside a declared arm except the two `--explain` cells**, which are Claim 1.

No second unregistered shape came back on any of those. That is evidence, not proof, and the claim stays
OPEN rather than being promoted on a source read.

### Codex's M52 §A dispositions (its §"M52 §A Axis 5 dispositions") — all six **AGREE** with the driver

| Codex disposition | driver verdict | reconciler |
|---|---|---|
| DEFECT 1 — **CLOSED** (empty-slug refused at `start.rs:80-104`, `milestone.rs:642-662`, `engine/milestone.rs:362-386` under `write.unslugable-title`) | CLOSED | **CONFIRMED** — re-driven §14.3.e |
| DEFECT 2 — **NOT CLOSED** (`remove-step` reads the pack-only byte reader, `config.rs:1810-1838`/`1985-1998`; `replace-step` same basis) | STILL-OPEN | **AGREES** — driver repro §2.2 stands; no contradiction to drive |
| DEFECT 3 — **NOT CLOSED** (`rename.rs:83-100` excludes malformed addresses from `RefusalKind`; `parse_addr` bare `anyhow!` at `:1193-1207`) | STILL-OPEN, **and wider** | **AGREES**; the driver's widening (the same code-less bail at `doc show` and the `doc` write verbs under a second wording) is **outside Codex's read** and is not contradicted by it |
| DEFECT 4 — **NOT CLOSED** (`doc show` routes through `show_json`/the store; `store.rs:48-50` defines `store.no-such-leaf` as a *physically* absent leaf; no source change distinguishes declared-but-unpopulated) | STILL-OPEN | **AGREES** — and the reconciler re-drove the cell as part of §14.3.d: `doc show 'vision:vision#meta/grounded-in'` → exit 1, `store.no-such-leaf`, against a control `#meta/schema-version` → exit 0 |
| C1 — **NOT CLOSED** | STILL-OPEN | **CONFIRMED** = `lead(codex, 2)` |
| D1 — **NOT CLOSED** | STILL-OPEN | **CONFIRMED** = `lead(codex, 1)` |

### Codex's M51 §A dispositions — all four **AGREE** with the driver's §1/§4

DEFECT A (setup/uninstall's bare `Finding`) **CLOSED** · DEFECT B (`doc show`'s undeclared shapes)
**CLOSED** · DEFECT C (`ArmOutcome::Success` vs non-zero exit) **CLOSED by declaration** · DEFECT D
(pre-dispatch `current_dir()` bypass) **CLOSED**. The driver drove all four; Codex read all four to the
same verdict. The reconciler independently reproduced **DEFECT A's cell** (§14.3.a: 45 `Reject::Error` +
2 `Reject::Findings` = 47, **zero bare-`Finding` roots**) and **DEFECT B's class** (§14.3.c). No
contradiction anywhere; nothing to drive.

### Codex's M53-consistency notes → **AGREE, none contradicts a driven row**

`FINALIZE_MESSAGE_FILE` shared by both registries · `foreign_area_paths` descending `merged/docs` ·
the residual base-pin rule at five resolution sites · `finalize.foreign-bytes` at the landed teardown
seam with `milestone finalize`'s stderr-only narration **explicitly bounded** · *"no new M53 code path
turns a committing/destroying/moving door into exit-0 loss"*. Each is the **source-side reading of a
driven driver row** (§5.9 · §5.3 · §5.6 · §5.2–§5.4 · §11's tier-1 zero). Agreement, not a new claim.
Codex's own bound is kept on the record: its schema-freeze leg is *"a source/path comparison, not an
independently executed hash calculation"*.

---

## 14.2 — driver defects → status

### DEFECT A — `store.unknown-type` emits a **URI-shaped** target at `jigc doc show` where the contract fixes the bare doctype id → **STANDS (CONFIRMED)**, tier 3

**The Codex pass is silent on it** — its census asks *which arm* each producer takes, never *what target
form the key carries*, which is exactly the gap the driver names at §13. Under the rule a driven defect
the source pass does not reach stays a finding; **re-driven once by the reconciler**:

```
rig: committed-singletons
$ jigc --format json doc show 'nosuchtype:x'
  findings[0].key -> {"code":"store.unknown-type","target":"nosuchtype:x"}      <- URI-shaped
  message         -> "unknown doctype `nosuchtype` for `nosuchtype:x`"
  location.address-> "nosuchtype:x"
$ jigc --format json doc show 'nosuchtype:x#meta'                 -> target "nosuchtype:x"

the same code, same condition, NINE sibling producers, all BARE:
$ jigc --format json doc schema       nosuchtype                                  -> "nosuchtype"
$ jigc --format json doc set-field    'nosuchtype:x#meta/k' --value v --task <t>   -> "nosuchtype"
$ jigc --format json doc set-slot     'nosuchtype:x#body' --from-file /dev/null --task <t> -> "nosuchtype"
$ jigc --format json doc add-item     'nosuchtype:x#s' --title T --task <t>        -> "nosuchtype"
$ jigc --format json doc remove-item  'nosuchtype:x#s/i' --task <t>                -> "nosuchtype"
$ jigc --format json doc retitle-item 'nosuchtype:x#s/i' --title T --task <t>      -> "nosuchtype"
$ jigc --format json doc rename       'nosuchtype:x' --to Y --task <t>             -> "nosuchtype"
$ jigc --format json rename           'nosuchtype:x' --to Y                        -> "nosuchtype"
$ jigc --format json relocate         nosuchtype --from docs/old/                  -> "nosuchtype"
$ jigc --format json migrate          CHANGELOG.md --as nosuchtype                 -> "nosuchtype"
  (message at all nine: "unknown doctype `nosuchtype`")
```

**The contradiction leg, re-read at the source** — `design/command-output-contract.md`, the
doctype-scoped-blocks bullet: *"it is the form the read path **already** emits for a doctype-scoped
block (`store.unknown-type` addresses the bare doctype — `Location::addressed(doctype, …)`,
`doc.rs:1724-1729`) and the form `jigc doc schema <doctype>` already takes. It is deliberately **not**
URI-shaped — a bare id cannot be mistaken for a doc address a driver could `doc show`."* Driven, the one
door that sentence names as the precedent is the one door that does not obey it, and the key it emits is
exactly the doc-address shape the sentence forbids. **Tier 3** (a surface says something the binary does
not do): exit 1 at a read door, no write, no loss, no repository harm. **Origin: driver.**

### The driver's zero-tier-1 claim → **AGREES with Codex**

Codex's exit-rule result for axis 5 is *"no TIER-1 row found"*, independently of the driver's §11
*"no tier-1 row"*. Two instruments, two methods, same verdict.

---

## 14.3 — the reconciler's own repro blocks (the demotions, re-driven)

### a. §4.3's 47-leaf outside-a-repo sweep — **reproduced**

```
$ OUT=$(mktemp -d "${TMPDIR:-/tmp}/outrepo.XXXXXX"); cd "$OUT"    # mktemp root, no teardown
$ for each of the 47 VERB_KINDS leaves: jigc --format json <minimal argv>
roll-up: Reject::Error 45 · Reject::Findings 2 · NOT-JSON 0 · total 47
the 2 findings-arm leaves: `setup` (setup.repo-root) · `uninstall` (uninstall.repo-root)
stdout 0 bytes on all 47 — the document is stderr's.  ZERO bare-`Finding` roots.
```
*(Two leaves first came back NOT-JSON at exit 2 on the reconciler's own malformed argv — `workflow`
without `--preview|--task`, `config fill` without `--from-file`. Both are clap usage errors, i.e. the
declared §4.5 carve-out, and both land in a declared arm once the argv is legal:
`workflow single-task --preview` → `Reject::Error`, `config fill step:s#f --from-file /dev/null` →
`Reject::Error`. Recorded because the first run is what the number would otherwise have hidden.)*

### b. §4.1's seven `Unpinned` rows — **re-driven, 7 / 7**

```
rig: fresh
$ jigc --format json describe                 -> ["commands","definitions","schema_version"]
  --workflows / --doctypes / --commands        -> the identical key set ×3
$ jigc --format json milestone create "Recon Wave"      -> ["hook_output","text"]
$ jigc --format json milestone add-task recon-wave "Sub One" -> ["hook_output","text"]
$ jigc --format json milestone list-tasks recon-wave    -> ["text"]        <- no `hook_output`
```

Plus the further registry arms the reconciler drove while spot-checking §3 (rig
`committed-singletons`): `doc schema vision` → `["contract-version","fields","home","identity",
"schema-version","sections","type"]` with **`contract-version` 7** · `doc set-field` →
`["copied_in","findings","op","target","value"]` · `doc set-slot` →
`["chars","copied_in","findings","op","target"]` · `doc show <committed>` →
`["fields","item-count","schema-version","sections","slug","type"]` · `doc show --task <staged>` →
the same **plus `staged`** · `validate` →
`["blocking_probes","findings","report_only","schema_version","scope"]` · `task diff` →
`["base","code_diff","findings","op","staged_docs","task"]` · `task list` → `ArrayOf{id,intent,workflow}` ·
`migrate-corpus --dry-run` → `["already_current","blocked","commit","dry_run","hook_output","migrated",
"unadopted","unfilled"]` · `doc list` and `doc list --task` → `["docs"]` · `workflow --preview` →
`["task","text"]` · `config get` → `["key","layer","op","rejected","value"]` · `config list` →
`["knobs","op"]`. **22 / 22 equal to the declared key set.**

### c. §6's `doc show` dispatch space — **re-driven, 10 / 10, 0 undeclared shapes**

```
rig: committed-singletons
changelog:changelog                                     exit 0  Object       [fields,item-count,schema-version,sections,slug,type]
changelog:changelog#releases                            exit 0  ArrayOfDataKeyed [changes,date,id,link,title]
changelog:changelog#releases/1-0-0                      exit 0  DataKeyed    [changes,date,id,link,title]
changelog:changelog#releases/1-0-0/date                 exit 0  Scalar       "2026-09-22"
changelog:changelog#releases/1-0-0/link                 exit 0  Scalar       "https://example.com/compare/0.9.0...1.0.0"
changelog:changelog#releases/1-0-0/changes              exit 0  ArrayOfDataKeyed [category,id,notes]
changelog:changelog#releases/1-0-0/changes/added        exit 0  DataKeyed    [category,id,notes]
changelog:changelog#releases/1-0-0/changes/added/notes  exit 0  Scalar
changelog:changelog#releases/1-0-0/changes/added/category exit 0 Scalar      "added"
changelog:changelog#unreleased-changes/changed/notes    exit 0  Scalar
```

### d. §7.1's owed codes — **re-driven at 15 (not 14) coordinates, 15 / 15 on the owed findings arm**

The eleven `store.unknown-type` coordinates are §14.2's block. The remaining four:

```
rig: committed-singletons
$ jigc --format json doc show 'adr:no-such-doc'
  exit 1 · Reject::Findings · key {"store.not-found","adr:no-such-doc"}
$ jigc --format json rename 'adr:no-such-doc' --to X
  exit 1 · Reject::Findings · key {"store.not-found","adr:no-such-doc"}
$ jigc --format json doc show 'vision:wrong-slug'
  exit 1 · Reject::Findings · key {"store.fixed-identity","vision:wrong-slug"}
$ jigc --format json doc show 'vision:vision#meta/grounded-in'
  exit 1 · Reject::Findings · key {"store.no-such-leaf","vision:vision#meta/grounded-in"}
control: $ jigc --format json doc show 'vision:vision#meta/schema-version'  -> exit 0, "1"
```

**15 / 15 on the owed arm, 0 flattened.** The driver's `14 / 14` undercounts its own enumeration by one;
the arm claim survives the correction intact.

### e. `(5, DEFECT 1)`'s closure — **re-driven, and the HEAD isolation tightened**

```
rig: fresh          H0 = git rev-parse --short HEAD
$ for t in "" "   " "!!!" "日本語" "the of a"; do jigc milestone create "$t"; done
  -> exit 1 ×5, each `blocking · write.unslugable-title — cannot mint a milestone: its id is slugged
     from the title …`
  AFTER the five refusals and BEFORE any successful mint:
    HEAD c8198b6 -> c8198b6   git status --porcelain -> empty   .jigc/milestones -> absent
$ jigc milestone create "Real one"        -> 0     H1 = HEAD
$ for t in "" "日本語" "the of a"; do jigc milestone add-task real-one "$t";
                                      jigc start --workflow single-task "$t"; done
  -> every one `write.unslugable-title — cannot mint a task: …`
    HEAD 28459dd -> 28459dd   .jigc/tasks -> absent
```
*(The driver's §2.1 asserts *"`git rev-parse HEAD` unmoved by every refusal"* while its transcript
interleaves a **successful** `milestone create "Real one"`, which does move HEAD. The reconciler split
the two segments so the unmoved-HEAD claim is measured over refusals only. **The claim holds** — this is
a tightening of the driver's instrument, not a correction of its verdict.)*

---

## 14.4 — reconciler's notes

1. **No claim was refuted on this axis, by either instrument.** The two passes agree on all ten M52/M51
   §A dispositions, on the zero-tier-1 verdict, and on both Codex claims; the one asymmetry is DEFECT A,
   which the source pass does not reach rather than contradicts.
2. **Both Codex claims land on already-open M52 §A rows** (`D1`, `C1`), so neither is a *new* finding.
   Their value is the second, independent derivation: the driver reached them by driving, Codex by
   reading, and the registry/source citations Codex adds (`render.rs:6066-6124`, `result.rs:127-175`,
   `orient.rs:116-125`) are the source-side half the driver's repro blocks lacked.
3. **The one methodological gap is the same on both sides**: nothing in either pass compares a *target
   form* to the form the contract fixes. That gap is where DEFECT A lives, and Codex's census — which
   enumerates arms exhaustively — walks straight past it.
4. **Four driver rows were demoted for carrying no repro block**; all four were re-driven by the
   reconciler and all four hold. One carried a real counting error (§7.1: 14 reported, 15 enumerated,
   15 driven). The 42 un-spot-checked `ENVELOPE_ARMS` rows of §3 stay **construction-backed, not
   per-row evidenced** — stated rather than silently accepted.
5. **The universal half of Codex's serialization census stays OPEN.** It is the only lead on this axis
   that could not be driven, and the reason is structural (a negative universal), not a missing fixture.
6. **Bound carried, unchanged:** the release posture means the `#[cfg(debug_assertions)]` route fences
   do not exist in this binary, so a route-fence violation shows up as a bad emitted command rather than
   a panic — the driver states this and the reconciler drove under the same posture.

---

# 15 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling (`crates/cli/src/cli.rs:1834`,
read at `HEAD = 7e98faf1`). **47 / 47 leaves · uncovered: none.**

```
start · workflow · setup · uninstall · upgrade · ingest · migrate · migrate-corpus · unmanage ·
rename · relocate · describe · validate ·
doc create · doc add-item · doc remove-item · doc retitle-item · doc rename · doc set-field ·
doc set-slot · doc author · doc show · doc schema · doc list ·
task list · task diff · task validate · task discard · task finalize · task bind ·
config set · config insert-step · config replace-step · config remove-step · config fill ·
config fork · config get · config list ·
milestone create · milestone add-task · milestone add-from-spec · milestone list-tasks ·
milestone provision · milestone execute · milestone join · milestone finalize · milestone discard
```

**How the coverage is carried, stated rather than implied.** Every one of the 47 is the door of at least
one row in the driver's two uniform 47-leaf sweeps (§4.3 outside-a-repo — **independently reproduced by
the reconciler at §14.3.a** — and §4.4 the deleted cwd). **43** of the 47 are additionally the door of at
least one *success-arm* or *condition-specific* row in §3/§5/§6/§7. The four whose axis-5 coverage rests
on the uniform sweeps plus a reject-arm row only are **`ingest` · `upgrade` · `doc author` ·
`config fork`** — the driver drives each at one registry arm (§3 rows 9, 8, 25, 50) under the shared
construction block, which §3's demotion marks as construction-backed rather than per-row evidenced.

**Doors the reconciler itself drove** (a subset, listed so the ledger's evidence is attributable):
`start` · `workflow` · `setup` · `uninstall` · `migrate` · `migrate-corpus` · `rename` · `relocate` ·
`describe` · `validate` · `doc show` · `doc schema` · `doc list` · `doc set-field` · `doc set-slot` ·
`doc create` · `doc add-item` · `doc remove-item` · `doc retitle-item` · `doc rename` · `task list` ·
`task diff` · `config get` · `config list` · `config fill` · `milestone create` · `milestone add-task` ·
`milestone list-tasks` — **28**, plus all 47 through the outside-a-repo sweep.
