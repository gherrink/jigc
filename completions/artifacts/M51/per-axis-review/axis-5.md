<!-- M51 per-axis review — axis 5 · reconciled · driven on the installed `jigc 1.0.0-rc.15` (commit 35195f56), 2026-09-16 -->

# M51 per-axis review — AXIS 5 · pinned contracts · RECONCILED (driver + codex source pass)

**Binary.** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15` (asserted first; **release posture** —
the `Route::mechanical` fence and the other `#[cfg(debug_assertions)]` panics do not exist here).
Every row below ran on that binary, in throwaway repos minted by
`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"` (two-step eval,
`mktemp -d` roots, no teardown, no `rm -rf` on a variable path). The fixtures beyond the six rig states
were built **by driving the binary**, never by writing into `.jigc/`.

**The binary this table reflects carries the four post-build audit fixes** — routes `8a42fbbd`,
orphan territory `b5ccd818`, setup guard `0fc80bab`, the two LOWs `dc508994`. Two of them are visible in
the drives: the orphan territory fix is driven at `validate`/`doc list` (row set E), and the setup guard's
new refusal `setup.dirty-install-path` is the door that carries **DEFECT A**.


> **Reconciliation banner (this file).** The table below is the Opus driver's, carried
> **unchanged except for the two demotion notes marked `[DEMOTED — repro under-specified]`**
> in §4 D2 and §6 F. Everything after §11 is the reconciler's: the Codex source pass entered
> as leads and driven, the driver's three defects re-driven, and the doors list.
> A fourth defect — **DEFECT D**, origin **codex** — was confirmed by driving and is written up
> in the ledger, not folded into the driver's table.

---

## 0 · The door set, derived from the code

Counts read at HEAD, by symbol, not from the design doc's numbers:

| registry | file:line | rows read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1669` | **47** leaves |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs:5316` | **60** arms (58 leaf-owned + 2 cross-cutting reject rows with an empty `path`) |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:948` | **6** |
| `ManifestKind::ALL` | `crates/cli/src/render.rs:1834` | **6** |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:130` | **10** |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:2625` | **4** |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs:2935` | **14** |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs:2357` | **16** |
| `SLUG_DOORS` | `crates/cli/src/cli.rs:2664` | **6** |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2424` | **25** |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:1838` | **47** |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs:757` | **18** |

Axis 5's door set is `ENVELOPE_ARMS` — **60 arms over all 47 leaves**. The 60 arms partition by
`ArmOrigin` into **20 `Variant`** (enum-derived), **27 `Sole`**, **13 `Dispatch`**; by `ArmStatus` into
**53 `Pinned`** and **7 `Unpinned(<reason>)`** (`describe | Menu` + the five `milestone` `RecordOnlyAck`
rows + `milestone list-tasks | Listing`); by `ArmRoot` into **ResultContract** and **AdHoc**.

Cell set (the acceptance design's): `{declared key set == driven key set · Unpinned(<reason>) · the four
deletes absent · the reject funnels (`error` vs findings envelope) · exit code}`.

---

## 1 · Row set A — the 60 registry arms, each driven in its declared cell

Every one of the 60 rows was driven through the installed binary. **60 / 60 declared key sets equal the
driven key sets**; **0 mismatches**; the `schema_version` partition (`schema_version` rides a row **iff**
its `ArmRoot` is `ResultContract`) holds at **60 / 60**; the stream discipline holds at **60 / 60** (a
success/adjudication arm writes exactly one JSON document to stdout and **no** JSON to stderr; a reject
leaves stdout **empty** and writes exactly one document to stderr).

| # | door (leaf) | arm | argv driven | exit | code\|none | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `start` | `OrientationView::UnsetProject` | `jigc --format json start` | 0 | — | none (success/adjudication arm) | stdout top-level keys `schema_version, state` | matches contract |
| 2 | `start` | `OrientationView::Clean` | `jigc --format json start` | 0 | — | none (success/adjudication arm) | stdout top-level keys `header, next_steps, schema_version, state, workflows` | matches contract |
| 3 | `start` | `OrientationView::ActiveTask` | `jigc --format json start` | 0 | — | none (success/adjudication arm) | stdout top-level keys `header, next_steps, schema_version, state, tasks, workflows` | matches contract |
| 4 | `start` | `Composed` | `jigc --format json start --workflow single-task harden the cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `task, text` | matches contract |
| 5 | `workflow` | `Composed` | `jigc --format json workflow single-task --preview` | 0 | — | none (success/adjudication arm) | stdout top-level keys `task, text` | matches contract |
| 6 | `setup` | `Installed` | `jigc --format json setup` | 0 | — | none (success/adjudication arm) | stdout top-level keys `allowlist_file, findings, guide_file, hook_committed, hook_file, install_commit, line_file` | matches contract |
| 7 | `uninstall` | `TornDown` | `jigc --format json uninstall` | 0 | — | none (success/adjudication arm) | stdout top-level keys `allowlist_file, findings, line_file, removed` | matches contract |
| 8 | `upgrade` | `Swept` | `jigc --format json upgrade` | 0 | — | none (success/adjudication arm) | stdout top-level keys `checked, findings, guide, schema_version` | matches contract |
| 9 | `ingest` | `Triaged` | `jigc --format json ingest` | 0 | — | none (success/adjudication arm) | stdout top-level keys `rows, summary` | matches contract |
| 10 | `migrate` | `Composed` | `jigc --format json migrate docs/direction.md --as vision` | 0 | — | none (success/adjudication arm) | stdout top-level keys `task, text` | matches contract |
| 11 | `migrate-corpus` | `Report` | `jigc --format json migrate-corpus --dry-run` | 0 | — | none (success/adjudication arm) | stdout top-level keys `already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled` | matches contract |
| 12 | `unmanage` | `Report` | `jigc --format json unmanage docs/decisions/cache-strategy.md` | 0 | — | none (success/adjudication arm) | stdout top-level keys `dropped, identity, path` | matches contract |
| 13 | `rename` | `Report` | `jigc --format json rename adr:cache-strategy --to Cache policy` | 0 | — | none (success/adjudication arm) | stdout top-level keys `commit, from, hook_output, new_path, old_path, prose_mentions, referrers, title, to` | matches contract |
| 14 | `relocate` | `Report` | `jigc --format json relocate note --from docs/legacy-notes/` | 0 | — | none (success/adjudication arm) | stdout top-level keys `blocked, displaced, moved` | matches contract |
| 15 | `describe` | `Menu` | `jigc --format json describe` | 0 | — | none (success/adjudication arm) | stdout top-level keys `commands, definitions, schema_version` | matches contract |
| 16 | `validate` | `StoreSweep` | `jigc --format json validate` | 0 | — | none (success/adjudication arm) | stdout top-level keys `blocking_probes, findings, report_only, schema_version, scope` | matches contract |
| 17 | `doc create` | `DocAck::Created` | `jigc --format json doc create adr --title Cache strategy --task harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `copied_in, existed, findings, op, target` | matches contract |
| 18 | `doc add-item` | `DocAck::AddedItem` | `jigc --format json doc add-item spec:rate-limiting#criteria --title Limits per IP --task spec-the-rate-limiter` | 0 | — | none (success/adjudication arm) | stdout top-level keys `copied_in, findings, op, target` | matches contract |
| 19 | `doc remove-item` | `DocAck::RemovedItem` | `jigc --format json doc remove-item spec:rate-limiting#criteria/limits-per-ip --task spec-the-rate-limiter` | 0 | — | none (success/adjudication arm) | stdout top-level keys `copied_in, findings, op, removed, target` | matches contract |
| 20 | `doc retitle-item` | `DocAck::RetitledItem` | `jigc --format json doc retitle-item spec:rate-limiting#criteria/limits-per-ip --title Limits by client --task spec-the-rate-limiter` | 0 | — | none (success/adjudication arm) | stdout top-level keys `copied_in, findings, op, target, title` | matches contract |
| 21 | `doc rename` | `DocAck::Renamed` | `jigc --format json doc rename adr:cache-strategy --to Cache policy --task harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `committed_identity, copied_in, findings, from, op, reslugged, target, title` | matches contract |
| 22 | `doc set-field` | `DocAck::Field` | `jigc --format json doc set-field adr:cache-strategy#status --value accepted --task harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `copied_in, findings, op, target, value` | matches contract |
| 23 | `doc set-field` | `DocAck::UnsetField` | `jigc --format json doc set-field adr:cache-strategy#status/cites-code --unset --task harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `already_absent, copied_in, findings, op, target, unset` | matches contract |
| 24 | `doc set-slot` | `DocAck::Slot` | `jigc --format json doc set-slot adr:cache-strategy#decision --from-file - --task harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `chars, copied_in, findings, op, target` | matches contract |
| 25 | `doc author` | `DocAck::Authored` | `jigc --format json doc author adr --from-file - --task harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `copied_in, findings, op, target` | matches contract |
| 26 | `doc show` | `WholeDoc::Committed` | `jigc --format json doc show adr:cache-strategy` | 0 | — | none (success/adjudication arm) | stdout top-level keys `fields, item-count, schema-version, sections, slug, type` | matches contract |
| 27 | `doc show` | `WholeDoc::Staged` | `jigc --format json doc show adr:cache-strategy --task harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `fields, item-count, schema-version, sections, slug, staged, type` | matches contract |
| 28 | `doc show` | `FieldsGroupSlice` | `jigc --format json doc show adr:cache-strategy#status` | 0 | — | none (success/adjudication arm) | stdout = one JSON object, doc-keyed: `date, schema-version, status` | matches contract |
| 29 | `doc show` | `SlotSlice` | `jigc --format json doc show adr:cache-strategy#decision` | 0 | — | none (success/adjudication arm) | stdout = one JSON scalar (`string`) | matches contract |
| 30 | `doc schema` | `Projection` | `jigc --format json doc schema adr` | 0 | — | none (success/adjudication arm) | stdout top-level keys `contract-version, fields, schema-version, sections, type` | matches contract |
| 31 | `doc list` | `Index` | `jigc --format json doc list` | 0 | — | none (success/adjudication arm) | stdout top-level keys `docs` | matches contract |
| 32 | `task list` | `Rows` | `jigc --format json task list` | 0 | — | none (success/adjudication arm) | stdout = array, element keys `id, intent, workflow` | matches contract |
| 33 | `task diff` | `Ack` | `jigc --format json task diff harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `base, code_diff, findings, op, staged_docs, task` | matches contract |
| 34 | `task validate` | `Report` | `jigc --format json task validate harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `findings, schema_version` | matches contract |
| 35 | `task discard` | `TaskAck::Discarded` | `jigc --format json task discard harden-the-cache --force` | 0 | — | none (success/adjudication arm) | stdout top-level keys `commit, dropped, findings, op, task` | matches contract |
| 36 | `task bind` | `TaskAck::Bound` | `jigc --format json task bind spec spec:rate-limiting enforce-the-rate-limit` | 0 | — | none (success/adjudication arm) | stdout top-level keys `findings, op, role, target, task` | matches contract |
| 37 | `task finalize` | `Landed` | `jigc --format json task finalize harden-the-cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `committed, findings, schema_version` | matches contract |
| 38 | `task finalize` | `Forecast` | `jigc --format json task finalize harden-the-cache --dry-run` | 0 | — | none (success/adjudication arm) | stdout top-level keys `dry_run, findings, left_out, manifest, subject` | matches contract |
| 39 | `task finalize` | `Blocked` | `jigc --format json task finalize harden-the-cache` | 3 | — | none (success/adjudication arm) | stdout top-level keys `findings, schema_version` | matches contract |
| 40 | `task finalize` | `MigrationReviewHold` | `jigc --format json task finalize migrate-vision-docs-direction-0e4626a61c6d` | 4 | — | none (success/adjudication arm) | stdout top-level keys `retires, rewrites, source, task` | matches contract |
| 41 | `config set` | `ConfigAck::Set` | `jigc --format json config set invocation-log true` | 0 | — | none (success/adjudication arm) | stdout top-level keys `committed, key, op, relocated, value` | matches contract |
| 42 | `config insert-step` | `ConfigAck::InsertStep` | `jigc --format json config insert-step --workflow single-task --after implement ./extra.yaml` | 0 | — | none (success/adjudication arm) | stdout top-level keys `anchor, committed, op, side, step, workflow` | matches contract |
| 43 | `config replace-step` | `ConfigAck::ReplaceStep` | `jigc --format json config replace-step workflow:single-task#implement ./extra.yaml` | 0 | — | none (success/adjudication arm) | stdout top-level keys `committed, op, step, target` | matches contract |
| 44 | `config remove-step` | `ConfigAck::RemoveStep` | `jigc --format json config remove-step workflow:single-task#implement` | 0 | — | none (success/adjudication arm) | stdout top-level keys `committed, op, target` | matches contract |
| 45 | `config fill` | `ConfigAck::Fill` | `jigc --format json config fill step:implement#extra-guidance --from-file -` | 0 | — | none (success/adjudication arm) | stdout top-level keys `committed, op, target` | matches contract |
| 46 | `config fork` | `ConfigAck::Fork` | `jigc --format json config fork workflow:single-task#implement` | 0 | — | none (success/adjudication arm) | stdout top-level keys `base, committed, op, path, target` | matches contract |
| 47 | `config get` | `Reading` | `jigc --format json config get docs-root` | 0 | — | none (success/adjudication arm) | stdout top-level keys `key, layer, op, rejected, value` | matches contract |
| 48 | `config list` | `Readings` | `jigc --format json config list` | 0 | — | none (success/adjudication arm) | stdout top-level keys `knobs, op` | matches contract |
| 49 | `milestone create` | `RecordOnlyAck` | `jigc --format json milestone create Cache rework` | 0 | — | none (success/adjudication arm) | stdout top-level keys `hook_output, text` | matches contract |
| 50 | `milestone add-task` | `RecordOnlyAck` | `jigc --format json milestone add-task cache-rework Warm the read cache` | 0 | — | none (success/adjudication arm) | stdout top-level keys `hook_output, text` | matches contract |
| 51 | `milestone add-from-spec` | `RecordOnlyAck` | `jigc --format json milestone add-from-spec cache-rework spec:rate-limiting` | 0 | — | none (success/adjudication arm) | stdout top-level keys `hook_output, text` | matches contract |
| 52 | `milestone provision` | `RecordOnlyAck` | `jigc --format json milestone provision cache-rework` | 0 | — | none (success/adjudication arm) | stdout top-level keys `hook_output, text` | matches contract |
| 53 | `milestone discard` | `RecordOnlyAck` | `jigc --format json milestone discard cache-rework` | 0 | — | none (success/adjudication arm) | stdout top-level keys `hook_output, text` | matches contract |
| 54 | `milestone list-tasks` | `Listing` | `jigc --format json milestone list-tasks cache-rework` | 0 | — | none (success/adjudication arm) | stdout top-level keys `text` | matches contract |
| 55 | `milestone execute` | `Composed` | `jigc --format json milestone execute cache-rework` | 0 | — | none (success/adjudication arm) | stdout top-level keys `task, text` | matches contract |
| 56 | `milestone join` | `Report` | `jigc --format json milestone join cache-rework` | 0 | — | none (success/adjudication arm) | stdout top-level keys `findings, milestone, no_docs_from, overlay, schema_version` | matches contract |
| 57 | `milestone finalize` | `Landed` | `jigc --format json milestone finalize cache-rework` | 0 | — | none (success/adjudication arm) | stdout top-level keys `committed` | matches contract |
| 58 | `milestone finalize` | `Blocked` | `jigc --format json milestone finalize cache-rework` | 3 | — | none (success/adjudication arm) | stdout top-level keys `findings, schema_version` | matches contract |
| 59 | `*(cross-cutting)*` | `Reject::Error` | `jigc --format json config remove-step workflow:single-task#not-a-step` | 1 | — | Informational (funnel) | stderr top-level keys `error` | matches contract |
| 60 | `*(cross-cutting)*` | `Reject::Findings` | `jigc --format json doc show adr:nope` | 1 | — | Informational (funnel) | stderr top-level keys `findings, schema_version` | matches contract |

Rows 59 and 60 carry a finding code the table's `code` column elides for space; driven verbatim:

```
$ jigc --format json config remove-step workflow:single-task#not-a-step
exit 1 · stdout 0 bytes · stderr:
{
  "error": "blocking · config.anchor-absent — `not-a-step` is not a step in `single-task` as of this edit — its include list is: locate, implement, record-changelog, superseded-context, author-commit, finalize\n  route: name a step id present in the workflow's resolved include list, then re-run"
}

$ jigc --format json doc show adr:nope
exit 1 · stdout 0 bytes · stderr:
{
  "schema_version": 3,
  "findings": [
    {
      "severity": "blocking",
      "probe": "store",
      "check": "not-found",
      "code": "store.not-found",
      "key": {
        "code": "store.not-found",
        "target": "adr:nope"
      },
      "message": "could not read `adr:nope` at `docs/decisions/nope.md`: No such file or directory (os error 2)",
      "location": {
        "address": "adr:nope",
        "line": 1,
        "col": 1
      },
      "route": "create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read

```

### Repro block — row set A, the whole sweep

The sweep is one script per verb family; each recipe builds its own corpus through the binary and ends in
the `--format json` invocation whose streams and key set are recorded. Setup + argv + observed exit are in
each row's `argv` column above; the shared construction is:

```
# one corpus per row, built by driving the binary
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# (CommittedAdr base)   jigc start --workflow single-task "harden the cache"
#                       jigc doc create adr --title "Cache strategy" --task <t>
#                       jigc doc set-slot adr:cache-strategy#{context,decision,consequences} --from-file - --task <t>
#                       jigc doc set-field commit:<t>#type --value feat --task <t>   (+ scope, summary, body)
#                       jigc task finalize <t>
# (CommittedSpec base)  the same through `plan` / `spec:rate-limiting` + one `criteria` item
# then the row's own invocation, e.g.
$JIGC --format json doc show adr:cache-strategy
```

Observed, aggregated over the 60 rows:

```
total driven rows:                     60
key-set mismatches (declared vs driven): 0
schema_version partition violations:     0     (ResultContract <-> schema_version, both directions)
ArmOutcome stream violations:            0     (success/adjudication -> stdout only; reject -> stderr only, stdout empty)
declared exit honoured in the driven cell: 60/60   (0 / 0 / 3 / 4 / non-zero as declared)
```

---

## 2 · Row set B — the `Unpinned(<reason>)` cell

Seven arms declare `ArmStatus::Unpinned { reason, still_pinned }`. The driven check is that each one's
`still_pinned` subset is **actually on the wire** (an unpinned envelope may still carry a key another
contract pins), and that the row's declared key set is still exact.

| door | arm | argv driven | exit | still_pinned declared | present on the wire | verdict |
|---|---|---|---|---|---|---|
| `describe` | `Menu` | `jigc --format json describe` | 0 | `schema_version` | yes | matches contract |
| `milestone create` | `RecordOnlyAck` | `jigc --format json milestone create "Cache rework"` | 0 | `hook_output` | yes | matches contract |
| `milestone add-task` | `RecordOnlyAck` | `jigc --format json milestone add-task cache-rework "Warm the read cache"` | 0 | `hook_output` | yes | matches contract |
| `milestone add-from-spec` | `RecordOnlyAck` | `jigc --format json milestone add-from-spec cache-rework spec:rate-limiting` | 0 | `hook_output` | yes | matches contract |
| `milestone provision` | `RecordOnlyAck` | `jigc --format json milestone provision cache-rework` | 0 | `hook_output` | yes | matches contract |
| `milestone discard` | `RecordOnlyAck` | `jigc --format json milestone discard cache-rework` | 0 | `hook_output` | yes | matches contract |
| `milestone list-tasks` | `Listing` | `jigc --format json milestone list-tasks cache-rework` | 0 | *(none)* — and `hook_output` is one of the four deletes | `text` only | matches contract |

---

## 3 · Row set C — the four pre-pin deletes

Declared (acceptance design, arm 5): `installed`, `uninstalled`, `review`, and `hook_output` on
`milestone list-tasks` are **off the wire**, the information carried by the exit code instead.

| door | deleted key | argv driven | exit | driven top-level key set | key present? | verdict |
|---|---|---|---|---|---|---|
| `setup` | `installed` | `jigc --format json setup` | 0 | `allowlist_file, findings, guide_file, hook_committed, hook_file, install_commit, line_file` | **no** | matches contract |
| `uninstall` | `uninstalled` | `jigc --format json uninstall` | 0 | `allowlist_file, findings, line_file, removed` | **no** | matches contract |
| `task finalize` | `review` | `jigc --format json task finalize <migration-task>` | **4** | `retires, rewrites, source, task` | **no** | matches contract |
| `milestone list-tasks` | `hook_output` | `jigc --format json milestone list-tasks cache-rework` | 0 | `text` | **no** | matches contract |

```
=== the four pre-pin deletes, driven ===
setup           exit=0 keys=['allowlist_file','findings','guide_file','hook_committed','hook_file','install_commit','line_file']  'installed' present: False
uninstall       exit=0 keys=['allowlist_file','findings','line_file','removed']                                                   'uninstalled' present: False
review hold     exit=4 keys=['retires','rewrites','source','task']                                                                'review' present: False
ms list-tasks   exit=0 keys=['text']                                                                                              'hook_output' present: False
```

**Related, driven, not a defect:** `hook_output` is *nested* on the two landing envelopes rather than
top-level — `jigc task finalize` puts it inside `committed` (`committed` keys: `files, hash, hook_output,
left_out, manifest, promoted, subject`). The registry pins **top-level** keys only, so the row is exact.
Driven on the `chatty-hooks` rig, which also re-confirms JSON purity under a speaking hook:

```
rig=$(dev/jigc-rig chatty-hooks --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# ... finalize-ready single-task ...
$JIGC --format json task finalize <t>
exit 0
stdout: one JSON document, top keys ['committed','findings','schema_version']
        committed.hook_output == "chatty-hook: pre-commit spoke on success"
stderr: "--- hook output ---\nchatty-hook: pre-commit spoke on success"   (carries NO JSON)
```

---

## 4 · Row set D — the reject funnels, at every door

Two sweeps, because a reject cell has two independent axes: **which door** rejects, and **which cause**.

### D1 — every one of the 47 leaves, driven to a reject in the same way (outside a git repository)

45 of 47 answer `{"error": …}` on stderr, stdout empty, exit 1 — the declared `Reject::Error` arm.
**Two do not.** (`workflow` is included at 45: its first argv, `workflow single-task`, is a clap usage
error at exit 2 — the declared clap carve-out, not a jigc reject — and it was re-driven as
`workflow single-task --preview`, which conforms.)

```
D=$(mktemp -d "${TMPDIR:-/tmp}/nogit.XXXXXX"); cd "$D"; HOME="$D"
# every leaf, with just enough argv to satisfy clap, e.g.
jigc --format json start
jigc --format json doc show adr:x
jigc --format json milestone finalize m1
...
exit 1 · stdout 0 bytes · stderr {"error": "not inside a git repository (no `.git` found from …) — run jigc from inside the target git repository; …"}
conforming: 45 of 47   (the two exceptions are DEFECT A, below)
```

### D2 — a semantic reject per door (the cause axis)

| door | reject cell | exit | code | envelope shape on stderr | verdict |
|---|---|---|---|---|---|
| `setup` | dirty-install-path | 1 | setup.dirty-install-path | BARE-FINDING (undeclared) | **DEFECT A** |
| `uninstall` | untracked-workbench | 1 | uninstall.untracked-workbench-file | BARE-FINDING (undeclared) | **DEFECT A** |
| `task discard` | staged prose | 1 | — | Reject::Error | matches contract |
| `milestone discard` | staged prose | 1 | — | Reject::Error | matches contract |
| `migrate` | source-untracked | 1 | — | Reject::Error | matches contract |
| `migrate` | traversal escape | 1 | — | Reject::Error | matches contract |
| `config set` | unusable-root | 1 | — | Reject::Error | matches contract |
| `doc show` | malformed-slug | 1 | — | Reject::Error | matches contract |
| `doc show` | unknown-type | 1 | store.unknown-type | Reject::Findings | matches contract |
| `task finalize` | no-task | 1 | finalize.no-task | Reject::Findings | matches contract |
| `task diff` | no-task | 1 | finalize.no-task | Reject::Findings | matches contract |
| `milestone provision` | unknown | 1 | milestone.unknown | Reject::Findings | matches contract |
| `task discard` | malformed id | 1 | — | Reject::Error | matches contract |
| `task discard` | empty id | 1 | — | Reject::Error | matches contract |
| `doc create` | unknown-doctype | 1 | create.unknown-doctype | Reject::Findings | matches contract |
| `doc create` | gate-blocked | 1 | create.gate-blocked | Reject::Findings | matches contract |
| `doc set-field` | unknown-section | 1 | write.unknown-field | Reject::Findings | matches contract |
| `doc set-slot` | not-present | 1 | write.unknown-section | Reject::Findings | matches contract |
| `doc add-item` | not repeatable | 1 | write.wrong-shape | Reject::Findings | matches contract |
| `config get` | unknown knob | 1 | — | Reject::Error | matches contract |
| `config set` | unknown knob | 1 | — | Reject::Error | matches contract |
| `config remove-step` | no step | 1 | — | Reject::Error | matches contract |
| `config fork` | no step | 1 | — | Reject::Error | matches contract |
| `config fill` | no target | 1 | — | Reject::Error | matches contract |
| `config insert-step` | missing file | 1 | — | Reject::Error | matches contract |
| `config replace-step` | missing file | 1 | — | Reject::Error | matches contract |
| `workflow` | unknown id | 1 | — | Reject::Error | matches contract |
| `start` | unknown workflow | 1 | — | Reject::Error | matches contract |
| `unmanage` | unknown path | 0 | — | OTHER ['dropped', 'identity', 'path'] | n/a — not a reject (adjudication at exit 0) |
| `rename` | unknown doc | 1 | — | Reject::Error | matches contract |
| `relocate` | unknown type | 1 | — | Reject::Error | matches contract |
| `doc schema` | unknown type | 1 | store.unknown-type | Reject::Findings | matches contract |
| `describe` | unknown kind | 2 | — | NO JSON | n/a — clap carve-out (exit 2, plain text, declared) |
| `task bind` | unknown doc | 1 | finalize.no-task | Reject::Findings | matches contract |
| `task validate` | unknown task | 1 | finalize.no-task | Reject::Findings | matches contract |
| `migrate-corpus` | unknown | 2 | — | NO JSON | n/a — clap carve-out (exit 2, plain text, declared) |
| `milestone add-task` | unknown ms | 1 | milestone.unknown | Reject::Findings | matches contract |
| `milestone add-from-spec` | unknown | 1 | milestone.unknown | Reject::Findings | matches contract |
| `milestone list-tasks` | unknown | 1 | milestone.unknown | Reject::Findings | matches contract |
| `milestone execute` | unknown | 1 | milestone.unknown | Reject::Findings | matches contract |
| `milestone join` | unknown | 1 | milestone.unknown | Reject::Findings | matches contract |
| `milestone finalize` | unknown | 1 | milestone.unknown | Reject::Findings | matches contract |
| `milestone create` | dup | 0 | — | OTHER ['hook_output', 'text'] | n/a — not a reject (adjudication at exit 0) |
| `ingest` | unknown arg | 2 | — | NO JSON | n/a — clap carve-out (exit 2, plain text, declared) |
| `upgrade over torn-down` | upgrade over torn-down | 0 | — | OTHER ['allowlist_file', 'findings', 'line_file', 'removed'] | n/a — not a reject (adjudication at exit 0) |
| `upgrade after uninstall` | upgrade after uninstall | 1 | — | Reject::Error | matches contract |
| `doc list` | unknown type filter | 2 | — | NO JSON | n/a — clap carve-out (exit 2, plain text, declared) |
| `task list in bare repo` | task list in bare repo | 0 | — | OTHER ['[]empty'] | n/a — not a reject (adjudication at exit 0) |

**[DEMOTED — repro under-specified]** 44 of D2's 48 rows name a *reject cell* ("unknown knob",
"staged prose", "traversal escape") rather than the argv that produced it, so the row as written is not
re-runnable; only the four rows with verbatim blocks (`task discard '../..'`, `doc show 'adr:../../etc/passwd'`,
and DEFECT A's two doors) carry a full repro. The reconciler re-drove a **sample of four** — `relocate`
unknown type, `unmanage` unknown path, `setup` dirty-install-path, `uninstall` untracked-workbench — and all
four matched the table exactly (repros in the ledger). The set is therefore recorded as **driven with an
under-specified repro**, not as undriven; no finding rests on an unsampled row.

**Reading of D2.** Every door but `setup` and `uninstall` funnels a reject into exactly one of the two
declared cross-cutting arms. The split is the one `envelope_finding_error`'s doc-comment declares — a code
listed under a declared **target form** in `design/command-output-contract.md` owes the findings envelope
(`finalize.no-task`, `milestone.unknown`, `store.unknown-type`, `create.*`, `write.*`), everything else
flattens to `{"error": …}` and pays the declared price of losing the envelope *key* while keeping the
code **in the message text** — driven:

```
$ jigc --format json task discard '../..'
exit 1 · stdout empty · stderr:
{ "error": "blocking · work-unit.malformed-id — \"../..\" is not a valid work-unit id\n  route: use lowercase letters, digits, and single hyphens …" }

$ jigc --format json doc show 'adr:../../etc/passwd'
exit 1 · stdout empty · stderr:
{ "error": "blocking · store.malformed-slug — \"../../etc/passwd\" is not a valid doc slug — the `<slug>` head of address `adr:../../etc/passwd` …" }
```

### D3 — the clap carve-out (declared: **no** JSON on either stream)

| argv driven | exit | stdout JSON | stderr JSON | verdict |
|---|---|---|---|---|
| `jigc --format json --version` | 0 | no (17 bytes plain) | no (empty) | matches contract |
| `jigc --format json --help` | 0 | no (6248 bytes plain) | no (empty) | matches contract |
| `jigc --format json doc show` *(missing operand)* | 2 | no (empty) | no (134 bytes plain) | matches contract |
| `jigc --format json nosuchverb` | 2 | no (empty) | no (114 bytes plain) | matches contract |
| `jigc --format json doc list --type adr` *(no such flag)* | 2 | no (empty) | no (174 bytes plain) | matches contract |

---

## 5 · Row set E — the exit-code cell beyond the declared cell

`ArmOutcome::Success` is documented as *"**Exit 0**, the document on **stdout**, no JSON on stderr"*.
Driven, three rows ship their declared key set on stdout at a **non-zero** exit, by design — each exit
named by the exit-code taxonomy in the very contract doc the registry serves.

| door | arm | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| `validate` | `StoreSweep` (declared `Success`) | `jigc --format json validate` over a doc stamped `schema-version: 99` | **1** | `schema-conformance.schema-version-ahead` | Human (in-finding) | stdout keys `blocking_probes, findings, report_only, schema_version, scope` | **DEFECT C** |
| `validate` | `StoreSweep` (declared `Success`) | `jigc --format json validate` over a stamped `docs/stray.md` no doctype claims | **1** | `schema-conformance.orphaned-instance` | Human | same key set, on stdout | **DEFECT C** |
| `migrate-corpus` | `Report` (declared `Success`) | `jigc --format json migrate-corpus --dry-run` over the ahead stamp | **1** | *(report-carried)* | — | stdout keys `already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled` | **DEFECT C** |
| `task validate` | `Report` (declared `Success`) | `jigc --format json task validate <task with empty required slots>` | **3** | `schema-conformance.required-slot-present` | Mechanical | stdout keys `findings, schema_version`, stderr 0 bytes | **DEFECT C** |
| `task finalize` | `Blocked` (declared `Adjudicated(3)`) | `jigc --format json task finalize <t> --dry-run` over a blocking task | 3 | — | — | stdout keys `findings, schema_version` | matches contract (the `--dry-run` mode reaches the `Blocked` arm, not `Forecast`) |

The **key sets are exact** in every one of those runs — the registry's *shape* claim survives; only its
*outcome* claim does not. Row set E also drives the D12 orphan arm end to end (`STORE_EXIT_FLIPS`' sixth
member, the F1 audit fix's subject), which is the cell that most needed the **fixed** binary:

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# ... a committed adr:cache-strategy ...
printf -- '---\nschema-version: 1\n---\n\n# Stray\n\nprose\n' > docs/stray.md
git add -A && git commit -q -m 'stray in docs-root'

$ jigc --format json validate
exit 1 · stdout (one document):
  findings[0].code   = "schema-conformance.orphaned-instance"
  findings[0].key    = {"code":"schema-conformance.orphaned-instance","target":"docs/stray.md"}
  findings[0].route  = "restore what claims it — re-add the pack that defines its type, or move it to a
                        resolved doctype's home — or take it out of jigc's world: `jigc unmanage docs/stray.md` …"

$ jigc --format json doc list
exit 0 · stdout:
  {"docs":[{"id":"adr:cache-strategy","path":"docs/decisions/cache-strategy.md","state":"managed","item-count":0},
           {"id":null,"path":"docs/stray.md","state":"orphaned","item-count":null}]}
```

The declared third state renders with `id: null` and `item-count: null`, exactly as
`design/doc-read-surface.md` carries it, and the top-level key set (`docs`) is unmoved — the `Index` row
is exact. **This also confirms the F1 fix's territory bound from the other side**: the same stamped bytes
at the repository root are *not* claimed (out of territory), and a stamped file inside a doctype's own
home is claimed by that doctype instead (driven: `docs/decisions/stray.md` reads back as `adr:stray`,
state `managed`, and fires `schema-version-ahead` rather than `orphaned-instance`).

---

## 6 · Row set F — the `Sole` claim, driven across run-modes

`ArmOrigin::Sole` states *"the verb answers with exactly one key set … its several run-modes (`--dry-run`
vs committed, hit vs miss, empty vs populated) move **values**, never keys"*. 27 rows declare it. Driven a
second (and for some a third) run-mode each:

| door | second run-mode driven | exit | key set vs the declared one | verdict |
|---|---|---|---|---|
| `workflow` | `workflow sub-task --task warm-the-read-cache` (post-provision) | 1 | `Reject::Error` (a reject, not a second arm) | matches contract |
| `setup` | first install on `bare`; and `setup --force` over a dirty install path | 0 | same | matches contract |
| `uninstall` | after a first install; over leftover provisioned worktrees | 0 | same | matches contract |
| `upgrade` | after a cascade write | 0 | same | matches contract |
| `ingest` | with a foreign candidate present | 0 | same | matches contract |
| `migrate` | `--as adr --slug the-direction` | 0 | same | matches contract |
| `migrate-corpus` | committing run (no `--dry-run`) | 0 | same | matches contract |
| `unmanage` | hit *and* miss (`README.md`) | 0 / 0 | same | matches contract |
| `rename` | `--to … --slug caching` | 0 | same | matches contract |
| `relocate` | empty source directory | 0 | same | matches contract |
| `describe` | `--workflows`, `--doctypes`, `--commands` | 0 | same | matches contract |
| `validate` | store-flip (row set E) | 1 | same | keys match; **exit → DEFECT C** |
| `doc schema` | `spec` instead of `adr` | 0 | same | matches contract |
| `doc list` | empty store · `--task <id>` · with an orphaned row | 0 | same | matches contract |
| `task list` | no tasks (`[]`) | 0 | empty array — element keys vacuous | matches contract (declared `ArrayOf`; the populated drive in row set A carries the element keys) |
| `task diff` | task with nothing staged | 0 | same | matches contract |
| `task validate` | blocking task | **3** | same | keys match; **exit → DEFECT C** |
| `config get` | after a `config set` | 0 | same | matches contract |
| `config list` | after a `config set` | 0 | same | matches contract |
| `milestone create/add-task/add-from-spec/provision/discard` | empty milestone · no sub-tasks | 0 | same | matches contract |
| `milestone list-tasks` | populated | 0 | same | matches contract |
| `milestone execute` | second run | 0 | same | matches contract |
| `milestone join` | after `provision` | 0 | same | matches contract |

**[DEMOTED — repro under-specified]** ~11 of row set F's 30 cells name a run-mode in prose
("empty store", "after a cascade write", "with a foreign candidate present", "second run") instead of the
argv + fixture that reaches it, so those cells are not re-runnable as written. The reconciler re-drove a
**sample** — `config get` / `config list` on a populated store, `doc list` with an orphaned row, `describe`
across its kind flags, `relocate` on an empty source directory — and each matched. Recorded as **driven
with an under-specified repro**. The two cells that carry a verdict other than "matches contract"
(`validate`, `task validate`) are DEFECT C rows and carry full repros in §5 and in the ledger.

**No undeclared key set appeared at any `Sole` row.** The two undeclared arms this review found are both
at `Dispatch` verbs (`setup`/`uninstall`'s reject → DEFECT A; `doc show`'s slice projections → DEFECT B).

---

## 7 · Row set G — the `Dispatch` verbs' arm space, hunted by driving

`doc show`'s four rows all carry the same `Dispatch` reason: *"the address's DEPTH picks the projection —
whole doc, field group, slot — and two of **the four** are not objects at all"*. Driven at every depth the
address grammar reaches:

| argv driven | exit | JSON root type | driven key set | declared row | verdict |
|---|---|---|---|---|---|
| `doc show adr:cache-strategy` | 0 | object | `fields, item-count, schema-version, sections, slug, type` | `WholeDoc::Committed` | matches contract |
| `doc show adr:cache-strategy --task <t>` | 0 | object | + `staged` | `WholeDoc::Staged` | matches contract |
| `doc show adr:cache-strategy#status` | 0 | object | `date, schema-version, status` | `FieldsGroupSlice` (`DataKeyed`) | matches contract |
| `doc show adr:cache-strategy#status/status` | 0 | **scalar** | — | `SlotSlice` (`Scalar`) | matches contract |
| `doc show adr:cache-strategy#decision` | 0 | **scalar** | — | `SlotSlice` | matches contract |
| `doc show spec:rate-limiting#criteria` | 0 | **array** | elements `id, statement, title` | **none** | **DEFECT B** |
| `doc show roadmap#milestones` | 0 | **array** | elements `decomposition, id, proves, title` | **none** | **DEFECT B** |
| `doc show changelog#releases` | 0 | **array** | elements `changes, date, id, link, title` | **none** | **DEFECT B** |
| `doc show spec:rate-limiting#criteria/limits-per-ip` | 0 | object | `id, statement, title` | **none** (the `DataKeyed` row is scoped in prose to the *field group*) | **DEFECT B** |
| `doc show spec:rate-limiting#criteria/limits-per-ip/statement` | 0 | scalar | — | `SlotSlice` | matches contract |

`task finalize`'s four rows, driven across their run-modes: `--dry-run` over a *blocking* task reaches
`Blocked` (exit 3, `findings, schema_version`), not `Forecast` — no fifth arm. `start`'s `Composed` row
also answers `start --task <id>` (resume) with the same two keys.

---

## 8 · Defects

### DEFECT A — `jigc setup` and `jigc uninstall` reject with a **third**, undeclared envelope shape

**What the contract says.** `design/command-output-contract.md`:
*"**The two reject arms — the only rows no single leaf owns.** An operational error carries the single key
**`error`** — the message, as `{"error": …}` on **stderr**, exit 1 — **and nothing else**. A rejected run
that carries findings is the findings envelope — **`findings`** + **`schema_version`** — also on stderr."*
Its *Stream discipline* table repeats it: a reject is *"the single-key `{"error": …}` envelope … or a
blocked write (the findings envelope of §3)"*. `ENVELOPE_ARMS` carries exactly those two reject rows.
And both doors' registry rows declare `ArmOrigin::Sole` — *"the verb answers with **exactly one key
set**"*.

**What the binary does.** Both doors serialize a **bare `Finding`** at the JSON top level — eight keys,
neither `error` nor `findings` among them. A driver running the contract's own discrimination predicate
(*parse stdout; if stdout is empty, parse stderr*) reaches a document it cannot classify, and a driver
deserializing into the declared union fails outright.

```
# cell 1 — repo-root, both doors
$ D=$(mktemp -d "${TMPDIR:-/tmp}/nogit.XXXXXX"); cd "$D"; HOME="$D"
$ jigc --format json setup ; echo "exit=$?"
exit=1        # stdout: 0 bytes
stderr:
{
  "severity": "blocking",
  "probe": "setup",
  "check": "repo-root",
  "code": "setup.repo-root",
  "key": { "code": "setup.repo-root", "target": null },
  "message": "not inside a git repository (no `.git` found from /private/var/.../nogit.XRRiNv)",
  "location": null,
  "route": "run jigc from inside the target git repository; if this project isn't one yet, `git init` here first"
}
$ jigc --format json uninstall ; echo "exit=$?"
exit=1        # identical shape, code "uninstall.repo-root"

# every other leaf, same directory, same cause:
$ jigc --format json start
exit=1 · stderr {"error": "not inside a git repository (…) — run jigc from inside the target git repository; …"}
# 45 of 47 leaves answer that way.

# cell 2 — the wave's OWN new refusal (audit fix 0fc80bab), on a real repo
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$ printf '\n## TEAM RULES\n\nNEVER deploy on Friday.\n' >> CLAUDE.md
$ $JIGC --format json setup ; echo "exit=$?"
exit=1        # stdout: 0 bytes
stderr: { "severity":"blocking", "probe":"setup", "check":"dirty-install-path",
          "code":"setup.dirty-install-path", "key":{…}, "message":"1 path(s) in the install footprint …",
          "location":null, "route":"commit or stash the work at those path(s) … `jigc setup --force` …" }

# cell 3 — M50's uninstall guard, same shape
$ printf 'mine\n' > .jigc/scratch.txt
$ $JIGC --format json uninstall ; echo "exit=$?"
exit=1 · stderr: bare Finding, code "uninstall.untracked-workbench-file"
```

**Class, bounded by driving:** **2 doors** (`setup`, `uninstall`) — the two whose whole surface is
`Result<_, Finding>` (`crates/cli/src/locate.rs:97` states it: *"for the two doors (`jigc setup` /
`jigc uninstall`) whose whole surface is a `Result<_, Finding>`"*) — over **4 driven codes**
(`setup.repo-root`, `setup.dirty-install-path`, `uninstall.repo-root`,
`uninstall.untracked-workbench-file`). Every other reject cause at every other door funnels correctly
(row sets D1/D2, 45 of 47 and 38 of 40 conforming rows).

**Why the fences do not see it.** `format_json_success_axis.rs`'s proof 2 only reconciles
*enum-derived* arms; proofs 3 and 4 iterate the registry's own rows, so a **production** arm with no row
is invisible to all four. And `ArmOrigin::Sole`'s checkable half is *"a `Sole` row is the only row its
path has"* — a statement about the **table**, not about the binary, which is exactly the half that is
true here while the prose half is false.

### DEFECT B — `jigc doc show <addr>#<repeatable-section>` is a second top-level array, declared nowhere

**What the contract says.** `ArmShape::ArrayOf`'s own doc-comment: *"`jigc task list` is **the surface's
one array**, declared rather than reshaped."* `doc show`'s four registry rows declare *"the address's
DEPTH picks the projection — whole doc, field group, slot — and two of **the four** are not objects at
all, so no result enum could model the set."* Proof 2's statement is *"every production arm has exactly
one row."*

**What the binary does.** A repeatable section slice answers with a **top-level JSON array**, and an item
slice with an object whose keys are the item's own — neither has a row, and the array falsifies *the
surface's one array* by name. Driven on three doctypes from two packs:

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$ $JIGC --format json doc show roadmap#milestones
exit 0 · stdout root type: ARRAY · element keys: decomposition, id, proves, title
$ $JIGC --format json doc show changelog#releases
exit 0 · stdout root type: ARRAY · element keys: changes, date, id, link, title

# and on the dev pack's `spec`, both committed and staged:
$ $JIGC --format json doc show spec:rate-limiting#criteria
exit 0 · ARRAY · element keys: id, statement, title
$ $JIGC --format json doc show spec:rate-limiting#criteria/limits-per-ip
exit 0 · OBJECT · keys: id, statement, title          # matches no declared doc show row
$ $JIGC --format json doc show spec:rate-limiting#criteria/limits-per-ip --task <t>
exit 0 · same two shapes on the staged path (no `staged` key on a slice)
```

**The binary is right and the registry is the false home** — the same shape as DEFECT C.
`design/doc-read-surface.md` pins exactly these projections: *"a **repeatable section** (`#section`) → its
**item array**"*, and *"**An item object** carries its **`id`** … its `id-from` leaf … each other field
keyed by leaf id"* — driven byte-for-byte (`id, statement, title`; `id, title, proves, decomposition`).
So `doc show` conforms to the read contract, and it is `ENVELOPE_ARMS` — the list
`design/command-output-contract.md` now *names* instead of quantifying over — that is incomplete, on the
most-read surface jigc has.

Six projections ship where the registry declares four. The registry's *declared bound* — *"an arm exists
here iff a driven invocation produced a distinct key set … a key set reachable only under a state the
census did not build would not appear"* — is the mechanism that let it through, but the states here are
ordinary reads of two shipped doctypes from two packs, not exotic ones, and `doc show`'s own `Dispatch`
reason enumerates the projection space as *"whole doc, field group, slot"* — a three-member enumeration of
a five-member space.

### DEFECT C — `ArmOutcome::Success` claims exit 0 at three rows that ship non-zero by design

`ArmOutcome::Success` is documented as *"**Exit 0**, the document on stdout, no JSON on stderr."* Driven,
three rows ship the **declared key set on stdout at a non-zero exit**, and each exit is explicitly
declared by the exit-code taxonomy in the same contract doc:

- `validate | StoreSweep` → **exit 1** on any `STORE_EXIT_FLIPS` member (taxonomy row 1: *"the store-scope
  **exit flips** the `render::STORE_EXIT_FLIPS` table enumerates"*). Driven on two members.
- `migrate-corpus | Report` → **exit 1** on the same condition. Driven.
- `task validate | Report` → **exit 3** on a blocking preview (taxonomy row 3: *"the transaction gates only
  — `jigc task validate`, `jigc task finalize`, `jigc milestone finalize`"*). Driven.

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# committed adr, then: sed the front matter to `schema-version: 99`, commit
$ $JIGC --format json validate        -> exit 1, stdout keys blocking_probes, findings, report_only, schema_version, scope
$ $JIGC --format json migrate-corpus --dry-run
                                      -> exit 1, stdout keys already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled
# a live task whose adr slots are empty:
$ $JIGC --format json task validate <t>
                                      -> exit 3, stdout keys findings, schema_version, stderr 0 bytes
```

The binary is right and the **registry's outcome column is the false home** — two pinned statements about
one surface disagree. `format_json_success_axis.rs` proof 4 asserts `Success ⇒ exit 0`, but only over the
one cell each recipe drives (a clean store, a clean `--dry-run`, a finalize-ready task), so the
disagreement is invisible to it. Lowest severity of the three: a driver reading the stream-discipline
predicate is unharmed; a driver reading `ENVELOPE_ARMS` as the contract is told the wrong exit.

---

## 9 · What was NOT driven, and why

Stated plainly rather than presented as covered:

1. **`ENVELOPE_ARMS` × `ManifestKind::ALL` (6) and × `SchemaChangeKind::ALL` (18).** Those registries
   shape the *values inside* `task finalize | Forecast`'s `manifest` and `migrate-corpus | Report`'s
   report, not any envelope's top-level key set. They are axes 7 and 4's door sets; driving them here
   would prove their axes, not this one. The two envelopes that carry them **are** driven (rows 38, 11).
2. **`doc list`'s `orphaned` row through a doctype that leaves the resolved set** (a pack dropped from
   `packs.yaml`). I reached the same finding and the same row through the cheaper fixture — a stamped
   committed `.md` inside `docs-root` that no doctype claims — and drove it (row set E). The pack-dropping
   route was attempted and blocked by the create-gate (`create.gate-blocked` refuses `jigc doc create
   note` under `single-task`), so landing a committed instance of a fixture doctype needs the
   ingest/migrate route; that is flow-52 arm 9's and axis 7's job, and the envelope claim it would test
   here (`Index`'s top-level `docs` key) is already driven three ways.
3. **`ENVELOPE_ARMS` rows under a `--pack-from-dev` / project-pack composition.** The registry's key sets
   are pack-independent by construction (no row's key set is derived from a doctype), and the one
   pack-sensitive row (`describe | Menu`) is declared `Unpinned`. Axis 7 owns the pack axis.
4. **`hook_output`'s *value* across the `COMMITTING_DOORS` producer axis.** Driven at one producer
   (`task finalize`, on `chatty-hooks`) to confirm the key is on the wire and JSON purity survives a
   speaking hook; the per-producer value axis is `tests/hook_output_axis.rs`' and axis 4's.
5. **The `GIT_DIR` redirect posture** — declared out of scope wave-wide, and not an envelope cell.
6. **A concurrent/fan-out envelope.** `milestone join`'s row is driven single-process; the genuine
   Task-tool spawn is the standing honest bound M51's own VERDICT carries, unchanged by this review.
7. **`task list`'s element keys on an empty list.** Driven (`[]`, exit 0) but vacuous for the key claim;
   the populated drive in row set A is the one that carries it.

---

## 10 · What this adds over flow-52 arm 5

Flow-52 arm 5 (`crates/cli/tests/format_json_success_axis.rs`, the `EnvelopeArm` registry with its four
proofs) drives **every registry row once, in its declared cell, against the debug binary from the build
tree**. This review re-drives all 60 rows **on the installed release `1.0.0-rc.15`** — a different binary,
a different posture (no `#[cfg(debug_assertions)]` route fence), built after four audit fixes the suite
predates — and then goes at the two questions the four proofs are structurally unable to ask:

- **Proofs 1–4 all iterate the registry.** Every one of them takes `ENVELOPE_ARMS` (or the recipe set
  reconciled against it) as its subject, so *"is there a production arm the registry does not carry?"* is
  outside all four by construction — the registry's own doc-comment says so (*"a suite that renders a
  witness cannot see an arm the dispatch chooses"*), and it is exactly where **DEFECT A** and **DEFECT B**
  live. This review asked that question by driving instead: 47 leaves × a uniform reject cause (row set
  D1), 48 door-specific causes of which 40 are rejects (D2), 30 alternate run-modes at the 27 `Sole` rows (F), and 19
  address depths at `doc show` plus 4 other dispatch modes (G). Two undeclared arms fell out.
- **Proof 4 asserts `Success ⇒ exit 0` at one cell per row.** The recipes drive a clean store, a clean
  `--dry-run` and a finalize-ready task, so the three rows that ship their declared key set at exit 1 or 3
  by design never meet the assertion — **DEFECT C**.
- **The reject arms are proven by two recipes in the suite** (one `config remove-step` miss, one
  `doc show adr:nope`). Here they are proven as a **funnel over the whole door set**: 45/47 leaves in D1
  and 38 of the 40 reject cells in D2 land in one of the two declared shapes, which is what makes the two exceptions a
  bounded class rather than an anecdote.
- **Cross-axis confirmation of the audit fixes on the shipped binary**: F1's orphan territory driven from
  both sides (claimed inside a doctype home, orphaned inside `docs-root`, silent at the repo root), and
  F2's new `setup.dirty-install-path` refusal driven — which is how DEFECT A surfaced on the wave's own
  newest surface.

---

## 11 · Counts

| | |
|---|---|
| rows driven | **229** — A 60 registry arms · B 7 unpinned cells · C 4 delete cells · D1 47 uniform-reject cells · D2 48 cause-specific cells · D3 5 clap cells · E 5 exit-code cells · F 30 run-mode cells · G 23 dispatch-depth cells |
| rows not applicable / not driven | **7** (section 9) |
| declared key set == driven key set | **60 / 60** |
| `schema_version` ⇔ `ArmRoot::ResultContract` | **60 / 60** |
| stream discipline (one document, right stream, other stream JSON-free) | **60 / 60**, plus 45/47 (D1) and 38/40 (D2) in the reject sweeps |
| the four pre-pin deletes absent | **4 / 4** |
| `Unpinned(<reason>)` rows whose `still_pinned` keys are on the wire | **7 / 7** |
| clap carve-out rows conforming | **5 / 5** |
| defects | **3** (A · B · C) |
| `doors_covered` (leaves that are the door of ≥ 1 driven row) | **47 / 47** |

---

# Reconciliation ledger

Every Codex claim entered as `lead(codex, …)` and then **driven** on the same binary
(`/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15`), in throwaway roots minted by `mktemp -d` /
`dev/jigc-rig` (two-step eval, no teardown, no `rm -rf` on a variable path). Every driver defect
re-driven once by the reconciler. Nothing is promoted on a source read.

## A · Codex claims

### `lead(codex, "the pre-dispatch current_dir() failure bypasses both reject funnels, so --format json emits plain stderr")` → **CONFIRMED** · **DEFECT D** (origin codex)

Driven, and the class is **larger than the claim**: Codex wrote *"many other doors, including `setup`,
`uninstall`, `task list`, `config list`, and `milestone list-tasks`"*. Driven, it is **47 of 47 clap
leaves** — the entire door set. Under `--format json` every leaf answers a deleted cwd with **plain,
non-JSON text on stderr**, which is neither declared reject arm (`{"error": …}` / the findings envelope)
and is not the clap carve-out either (that carve-out is exit 2 from clap; this is exit 1 from jigc's own
dispatch). A driver running the contract's discrimination predicate — *parse stdout; if stdout is empty,
parse stderr* — gets a parse error with no envelope to classify.

```
# repro — the whole 47-leaf sweep
cd /tmp && D=$(mktemp -d "${TMPDIR:-/tmp}/gonecwd.XXXXXX")
cd "$D" && rmdir "$D"          # the cwd now does not exist; no rm -rf on a variable path
jigc --format json describe ; echo "exit=$?"
exit=1
stdout: 0 bytes
stderr: cannot determine the current directory: No such file or directory (os error 2)
        -> json.loads() raises; NOT-JSON

# driven across the whole VERB_KINDS leaf set, one minimal-clap argv each:
start · workflow --preview · setup · uninstall · upgrade · ingest · migrate · migrate-corpus ·
unmanage · rename · relocate · describe · validate · doc {create,add-item,remove-item,retitle-item,
rename,set-field,set-slot,author,show,schema,list} · task {list,diff,validate,discard,finalize,bind} ·
config {set,insert-step,replace-step,remove-step,fill,fork,get,list} ·
milestone {create,add-task,add-from-spec,list-tasks,provision,execute,join,finalize,discard}

  47 / 47   exit=1 · stdout 0 bytes · stderr "cannot determine the current directory: …" · NOT-JSON
   0 / 47   emitted either declared reject arm
```

Contrast, same binary, the *declared* funnel one directory up (a real directory, no git repo):

```
$ jigc --format json start
exit=1 · stdout 0 bytes · stderr {"error": "not inside a git repository (…) — run jigc from inside …"}
```

Source bound, read after driving (Codex's file:line evidence, re-counted at HEAD): **23** sites in
`crates/cli/src/cli.rs` of the shape

```rust
let cwd = match std::env::current_dir() {
    Ok(cwd) => cwd,
    Err(err) => {
        eprintln!("cannot determine the current directory: {err}");
        return Outcome::failure();
    }
};
```

(`crates/cli/src/cli.rs:695, 720, 753, 776, 794, 810, 830, 875, 900, 923, 941, 959, 983, 1067, 1428, 1444,
1470, 1495, 1519, 1543, 1568, 1590, 1610`, plus the `.ok()?` at `:540`). Each site has `format: Format`
in scope and ignores it; the shared funnels that honour it are
`render::operational_error` / `invocation_log::operational_failure`. The two declared reject rows are the
only two in the table (`crates/cli/src/render.rs:6001` and `:6014`, both `path: &[]`,
`outcome: ArmOutcome::Reject`).

**Why the four proofs cannot see it** — the same structural reason the driver names for DEFECTS A and B:
the reject half of `format_json_success_axis.rs` drives *jigc-level* rejects through the funnels, and every
proof's subject is `ENVELOPE_ARMS` or a recipe reconciled against it, so a production path that emits no
envelope at all is outside all four.

**Honest bound on reachability.** The trigger driven here is a deleted cwd, which is exotic as a hand-typed
state. A product-shaped route exists on paper — jigc's own `milestone discard` / `uninstall` / worktree
teardown removes a provisioned worktree an agent may be `cd`'d into — but the reconciler **did not drive
that route**, so it is named as a mechanism, not claimed as driven.

### `lead(codex, "the registry contains all 47 clap leaves and 60 arms; the unusual closed shapes are explicitly represented rather than hidden — doc show has scalar and data-keyed slice arms, and all are driven")` → **REFUTED**

Falsifying datum — driven on the `committed-singletons` rig (`roadmap`, `changelog`), release rc.15:

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

$ jigc --format json doc show roadmap#milestones
exit 0 · stdout root type: LIST (top-level JSON array) · element keys: decomposition, id, proves, title
$ jigc --format json doc show changelog#releases
exit 0 · stdout root type: LIST (top-level JSON array) · element keys: changes, date, id, link, title
$ jigc --format json doc show roadmap#milestones/m-alpha
exit 0 · stdout root type: OBJECT · keys: decomposition, id, proves, title
$ jigc --format json doc show changelog#releases/1-0-0
exit 0 · stdout root type: OBJECT · keys: changes, date, id, link, title

# contrast, the declared SlotSlice row, same rig:
$ jigc --format json doc show roadmap#milestones/m-alpha/proves
exit 0 · "That the composed loop lands one task end to end."      (scalar — declared)
```

`doc show` ships **six** projections where `ENVELOPE_ARMS` declares four, and one of the two undeclared
ones is a **top-level array** — which falsifies `ArmShape::ArrayOf`'s own doc-comment (*"`jigc task list` is
the surface's one array"*) by name. So the source pass's *"all are driven"* and *"explicitly represented
rather than hidden"* is refuted; this is the driver's DEFECT B, independently reproduced.

### `lead(codex, "stdout JSON renderers for setup/uninstall … each map to registry rows")` → **REFUTED (on the reject path)**

Codex's sentence is scoped to *stdout*, and on stdout it holds. It is entered and refuted here because the
source pass's completeness question — *is there a bypass of the seam this axis is about?* — is answered
"no" for these two doors, and driving says otherwise on their **stderr** path. Falsifying datum is
DEFECT A's repro (below): both doors serialize a **bare `Finding`** — eight keys, neither `error` nor
`findings` among them — at the JSON top level, a third reject shape the registry does not carry. The source
pass located the two declared reject rows correctly (`render.rs:6002-6023`) and did not find the path that
escapes them.

### `lead(codex, "the requested removals are reflected in declarations — setup has no installed, uninstall no uninstalled, migration review no review, milestone list-tasks only text")` → **CONFIRMED**

Driven (this is the driver's row set C, re-driven at its hardest cell — the migration review hold, which
needs an authored payload to reach exit 4 at all):

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf '# Direction\n\nWe are building a thing.\n\n## Invariants\n\nOne.\n' > docs/direction.md
git add -A && git commit -q -m 'a foreign direction doc'
jigc --format json migrate docs/direction.md --as vision        # -> task migrate-vision-docs-direction-0e4626a61c6d
jigc doc author vision --from-file - --task <t> <<'EOF'         # the <<…>> markers are required literal syntax
title: Vision
sections:
  - id: thesis
    set: { thesis: "<<We are building a thing, deterministically.>>" }
  - id: invariants
    set: { invariants: "<<One invariant holds.>>" }
  - id: open-questions
    set: { open-questions: "<<Nothing open yet.>>" }
EOF
$ jigc --format json task finalize <t>
exit 4 · stderr 0 bytes · stdout top-level keys ['retires','rewrites','source','task'] · 'review' present: False
```

(Driven en route: without the authored payload the same door is `Blocked` at **exit 3** with
`['findings','schema_version']` — a different declared arm, not the review hold. The driver's row 40 and
row-set-C `review` row both stand.)

### `lead(codex, "the only explicitly unpinned success shapes are describe and the milestone text acknowledgements, each with a reason")` → **CONFIRMED**

Driven at `describe` (`still_pinned: schema_version`), re-confirming the driver's row set B:

```
$ jigc --format json describe
exit 0 · stdout top-level keys: commands, definitions, schema_version      # schema_version on the wire
```

The remaining six unpinned rows are the five milestone `RecordOnlyAck`s + `milestone list-tasks | Listing`,
driven in the driver's row set B; the reconciler did not re-drive them and records that.

## B · Driver defects, re-driven

### DEFECT A — `setup` / `uninstall` reject with a third, undeclared envelope → **CONFIRMED, stands** (source pass silent)

Re-driven at all three of the driver's cells. The source pass did **not** contradict it — it never reached
the path — so this is recorded as *silent*, not refuted.

```
# cell 1 — outside a git repository
$ D=$(mktemp -d "${TMPDIR:-/tmp}/nogitA.XXXXXX"); cd "$D"
$ HOME="$D" jigc --format json setup ; echo "exit=$?"
exit=1 · stdout 0 bytes · stderr:
{ "severity":"blocking", "probe":"setup", "check":"repo-root", "code":"setup.repo-root",
  "key":{"code":"setup.repo-root","target":null},
  "message":"not inside a git repository (no `.git` found from /private/var/…/nogitA.5hj4mJ)",
  "location":null,
  "route":"run jigc from inside the target git repository; if this project isn't one yet, `git init` here first" }

$ HOME="$D" jigc --format json uninstall     -> exit 1, identical shape, code "uninstall.repo-root"

# the SAME directory, any other leaf — the declared funnel:
$ HOME="$D" jigc --format json start
exit=1 · stderr { "error": "not inside a git repository (…) — run jigc from inside the target git repository; …" }

# cell 2 — setup.dirty-install-path (the wave's own new refusal), on a real repo
rig=$(dev/jigc-rig bare --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$ printf '\n## TEAM RULES\n\nNEVER deploy on Friday.\n' >> CLAUDE.md
$ jigc --format json setup ; echo "exit=$?"
exit=1 · stdout 0 bytes · stderr: bare Finding, 8 keys, code "setup.dirty-install-path"

# cell 3 — uninstall.untracked-workbench-file
$ jigc setup >/dev/null ; printf 'mine\n' > .jigc/scratch.txt
$ jigc --format json uninstall ; echo "exit=$?"
exit=1 · stdout 0 bytes · stderr: bare Finding, 8 keys, code "uninstall.untracked-workbench-file"
```

Class as the reconciler bounds it: **2 doors × 4 driven codes**, exactly the driver's bound. Both doors'
whole surface is `Result<_, Finding>` (`crates/cli/src/locate.rs:97` says so by name).

### DEFECT B — `doc show <addr>#<repeatable-section>` is a second top-level array → **CONFIRMED, stands** (source pass **contradicted** it; refuted above)

Re-driven; repro and falsifying datum are under the second Codex lead. The reconciler reproduced the array
shape on **two** doctypes (`roadmap`, `changelog`) and the undeclared **item-object** shape on the same two,
against the declared `SlotSlice` scalar as control.

### DEFECT C — `ArmOutcome::Success` claims exit 0 at three rows that ship non-zero by design → **CONFIRMED, stands** (source pass silent on the binary; it asserted only what the *tests* enforce)

Re-driven, all three rows, one fixture:

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
sed -i '' 's/^schema-version: .*/schema-version: 99/' docs/roadmap.md
git add -A && git commit -q -m 'ahead stamp'

$ jigc --format json validate
exit 1 · stderr 0 bytes · stdout top-level keys ['blocking_probes','findings','report_only','schema_version','scope']
  findings: blocking file-state.hash-matches
            blocking schema-conformance.schema-version-ahead     <- the STORE_EXIT_FLIPS member
            advisory schema-conformance.repeatable-populated

$ jigc --format json migrate-corpus --dry-run
exit 1 · stderr 0 bytes · stdout top-level keys ['already_current','blocked','commit','dry_run','hook_output','migrated','unadopted','unfilled']

# a live task with unfilled author-required slots:
$ jigc start --workflow single-task "harden the cache" ; jigc doc create adr --title "Cache strategy" --task harden-the-cache
$ jigc --format json task validate harden-the-cache
exit 3 · stderr 0 bytes · stdout top-level keys ['findings','schema_version']
  codes incl. schema-conformance.required-slot-present
```

Key sets exact in all three; only the declared **outcome** is false. `ArmOutcome::Success`'s doc-comment at
`crates/cli/src/render.rs:5136` reads *"Exit 0, the document on **stdout**, no JSON on stderr."*

## C · Rows demoted

| where | what | disposition |
|---|---|---|
| §4 D2 | 44 of 48 rows name a reject *cell*, not the argv | **[DEMOTED — repro under-specified]**; 4 re-driven by the reconciler, all held; no finding rests on an unsampled row |
| §6 F | ~11 of 30 cells name a run-mode in prose, not an argv + fixture | **[DEMOTED — repro under-specified]**; sampled and held |
| §1 row 14 (`relocate \| Report`) | argv names doctype `note`, which **no shipped pack carries** — the argv as written rejects on every rig state, and §1's shared construction does not build the fixture | **NOT demoted** — the reconciler rebuilt the fixture (`dev/jigc-rig fresh --pack-from-dev --schema note <f>`, a freeze-exempt doctype; every shipped doctype refuses `relocate` as frozen) and reproduced the row exactly: `exit 0`, stdout `{"moved":[["docs/legacy-notes/a-stray-note.md","docs/notes/a-stray-note.md"]],"blocked":[],"displaced":[]}`. Repro was **incomplete as written**, not absent. |

Rows independently re-driven by the reconciler and matching the driver's table: **12, 14, 15, 26, 28, 29,
30, 31, 40, 46, 47, 48** (row set A), the `review` and `describe` cells (sets B/C), 47/47 of D1's leaf
sweep in an adjacent state, four D2 cells, all of E, and five of G.

## D · Open leads

**None.** Every Codex claim was driven to a repro or to a falsifying datum. The one reachability question
left open — whether DEFECT D is reachable through jigc's own worktree teardown rather than a hand-deleted
cwd — is recorded as an **undriven mechanism** inside DEFECT D's honest bound, not as a claim.

---

# Doors covered

Every clap leaf that is the door of ≥ 1 driven row, in `VERB_KINDS` spelling — **47 / 47**. D1's uniform
reject sweep and DEFECT D's 47-leaf sweep each touch the whole set; row set A drives each leaf in its
declared success cell.

```
start · workflow · setup · uninstall · upgrade · ingest · migrate · migrate-corpus · unmanage · rename ·
relocate · describe · validate ·
doc create · doc add-item · doc remove-item · doc retitle-item · doc rename · doc set-field ·
doc set-slot · doc author · doc show · doc schema · doc list ·
task list · task diff · task validate · task discard · task finalize · task bind ·
config set · config insert-step · config replace-step · config remove-step · config fill · config fork ·
config get · config list ·
milestone create · milestone add-task · milestone add-from-spec · milestone list-tasks ·
milestone provision · milestone execute · milestone join · milestone finalize · milestone discard
```

---

# Reconciled counts

| | |
|---|---|
| Codex claims entered as leads | **5** (1 numbered claim + 4 consistency statements load-bearing on completeness) |
| CONFIRMED | **3** (DEFECT D · the four deletes · the unpinned set) |
| REFUTED | **2** (`doc show` "all are driven" · setup/uninstall "map to registry rows", on the reject path) |
| OPEN LEADS | **0** |
| driver defects re-driven and standing | **3 / 3** (A · B · C) |
| driver defects the source pass contradicted | **1** (B — refuted with the driven datum) |
| driver defects the source pass was silent on | **2** (A · C) |
| rows demoted to *repro under-specified* | **2 sets** (D2 · F); **0** rows demoted to *undriven* |
| total defects on this axis after reconciliation | **4** — A · B · C (origin driver) · **D** (origin codex) |
| doors covered | **47 / 47** |
