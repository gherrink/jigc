<!-- M52 per-axis review (re-run) — axis 5 · transaction / rollback — RECONCILED · driven on the installed `jigc 1.0.0-rc.16` (built from commit e519e4eb), 2026-09-21. -->

<!-- M52 per-axis review (the RE-RUN of M51's instrument) — axis 5 · pinned contracts · RECONCILED -->

# M52 per-axis review — AXIS 5 · pinned contracts · RECONCILED

**Reconciler.** Opus, 2026-09-21. Inputs: the Opus driver's table (`driver/axis5.md`) and the
independent Codex source pass (`codex/axis5-codex.md`, prompt beside it). Binary asserted first:
`/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.16`. Rigs built exactly as the driver built them
(`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"` — two-step
eval, `mktemp -d` roots, no teardown, no `rm -rf` on a variable path). No repo edits, no fixes, no commits.

**The rule applied** (`acceptance-design.md` → The reconciliation rule): a claim by one pass that the
other cannot reproduce is a **lead**, not a finding. Every Codex claim was entered as
`lead(codex, …)` and then **driven** — to a repro block (CONFIRMED, origin codex) or to a recorded
refutation carrying the falsifying datum. Every driver defect was **re-driven once** here before it
was allowed to stand.

**Outcome in one line.** 1 Codex claim CONFIRMED by drive · 6 Codex dispositions CONFIRMED by drive ·
2 Codex completeness claims CONFIRMED (one by drive, one refuted in scope and re-stated) · 0 REFUTED ·
0 OPEN · the driver's 4 defects all re-driven and standing · **1 new defect found by the reconciler
that neither pass carried** (`jigc start --explain` is a production stdout arm `ENVELOPE_ARMS` does not
carry) · 0 demotions · 2 corrections to the driver table (both recorded, neither changing a verdict).

---

# Part I — the driver's table, carried unchanged

Reproduced verbatim from `driver/axis5.md`. No row was demoted: every row marked driven carries either
its own argv, a repro block, or a named runnable script (`causes.py` / `sweep.py` / `leaves.py`, all
present in `driver/` and read by the reconciler — `causes.py` carries a literal argv vector per cell,
so §5's "the argv is in the script" claim is true). The two corrections the reconciler owes the table
are recorded in Part II §D rather than written into the driver's own text.

---

<!-- M52 per-axis review (the RE-RUN of M51's instrument) — axis 5 · the Opus driver · driven on the installed `jigc 1.0.0-rc.16`, 2026-09-21 -->

# M52 per-axis review — AXIS 5 · pinned contracts · THE OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.16`, asserted first, before anything
else ran. **Release posture** — the `Route::mechanical` span fence and the other
`#[cfg(debug_assertions)]` panics do not exist here.

**The binary this table reflects carries M52's seven post-build audit fixes** (`ad527fa4` ·
`6c2de03a` · `33bd5692` · `68d14cd3` · `a83a9e60` · `66af090a`, plus the fold-back `e519e4eb`).
Four of the seven are visible in the drives: F4's `ENVELOPE_OWED_CODES` at §5, F5's
`RelocateRefusal::ALL` at §8, F3's rollback-conflict fold at §9, and F1's `home-vacated`
territory at §6.

Every row below ran on that binary, in throwaway repos minted by
`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
(two-step eval, `mktemp -d` roots, no teardown, no `rm -rf` on a variable path). Fixtures beyond
the rig states were built **by driving the binary**, never by writing into `.jigc/` — the two
exceptions are the two `PRE_DISPATCH_FAULTS` fixtures, whose whole definition is a corrupted
`.jigc/config/packs.yaml` and an empty `JIGC_PACK_DIR`, i.e. the fault *is* the file state.

I did **not** read the Codex source pass for this axis.

---

## 0 · The door set, derived from the code

Counts read at `HEAD = e519e4eb` by symbol, not from the design doc's numbers:

| registry | file | rows read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1834` | **47** leaves |
| **`ENVELOPE_ARMS`** | `crates/cli/src/render.rs:6030` | **64** arms (62 leaf-owned + 2 cross-cutting reject rows with an empty `path`) — **M51 read 60** |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:5415` | **4** codes (M52-minted) |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:968` | **7** (M51 read 6; `home-vacated` joined at M52) |
| `ManifestKind::ALL` | `crates/cli/src/render.rs:1985` | **6** |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **10** |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3275` | **6** (M51 read 4) — `Disposition::{Refuse, Narrate, Displace}` |
| `ROLLBACK_POPULATIONS` | `crates/cli/src/rollback.rs:169` | **11** (M52-minted) |
| `PRE_DISPATCH_FAULTS` | `crates/cli/tests/pre_dispatch_faults.rs:145` | **3** faults × 47 leaves (M52-minted) |
| `RelocateRefusal::ALL` | `crates/cli/src/relocate.rs:91` | **10** members / **9** codes (M52-minted) |
| `RefusalKind::ALL` | `crates/cli/src/rename.rs:151` | **11** members |
| `InProgress::ALL` | `crates/cli/src/repo.rs:234` | **9** git states |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs` | **14** |
| `DOCTYPE_DOORS` / `SLUG_DOORS` / `WORK_UNIT_ID_DOORS` / `BEHALF_DOORS` | `crates/cli/src/cli.rs` | **16 / 6 / 25 / 47** |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs` | **18** |

**Axis 5's door set is `ENVELOPE_ARMS` — 64 arms over all 47 leaves.** The 64 partition by
`ArmOrigin` into **20 `Variant`** / **27 `Sole`** / **17 `Dispatch`**; by `ArmStatus` into
**57 `Pinned`** / **7 `Unpinned(<reason>)`**; by `ArmRoot` into **12 `ResultContract`** /
**52 `AdHoc`**; by `ArmShape` into **58 `Object`** / 2 `DataKeyed` / 1 `Scalar` /
1 `ArrayOf` / 1 `ArrayOfDataKeyed` / 1 `ArrayOfScalars`; by `ArmOutcome` into
**59 `Success`** / 2 `Adjudicated(3)` / 1 `Adjudicated(4)` / 2 `Reject`.

**The four new rows since M51 are all `doc show`** — `ItemArraySlice`, `ItemSlice`,
`ListFieldSlice`, `CompoundFieldSlice` — which is M51 DEFECT B's fix, and the verb now carries
**8** rows where it carried 4. Two existing rows also gained a declared key: `ingest | Triaged`
gained `findings`, `rename | Report` gained `findings`.

Cell set (the acceptance design's): `{declared key set == driven key set · Unpinned(<reason>) ·
the four deletes absent · the reject funnels (`error` vs findings envelope) · exit code}`.

---

## 1 · M51 rows: CLOSED / STILL-OPEN — the re-run's first deliverable

M51's axis-5 ledger carries **four defects** (A·B·C driver-origin, D codex-origin) and **five
Codex leads**. Every one was re-driven on rc.16.

| M51 row | what it claimed | re-driven verdict | the datum |
|---|---|---|---|
| **DEFECT A** — `setup`/`uninstall` reject with a **third, undeclared** envelope (a bare `Finding` at the JSON root; 2 doors × 4 codes) | the driver's | **CLOSED** | §4.1's 47-leaf outside-repo sweep: **45 `{error}` + 2 `{findings, schema_version}` = 47/47 in a declared arm**, zero bare-`Finding` roots. Driven at all four of M51's codes (`setup.repo-root`, `setup.dirty-install-path`, `uninstall.repo-root`, `uninstall.untracked-workbench-file`) |
| **DEFECT B** — `doc show <addr>#<repeatable>` is a **second top-level array** and `#…/<item>` an undeclared object; six projections where the registry declared four | the driver's | **CLOSED** | the registry now declares **eight** `doc show` rows over **six** `ArmShape` members, and all eight were driven (§7). `ArmShape::ArrayOf`'s doc-comment no longer says *"the surface's one array"* — it now names `ArrayOfDataKeyed`/`ArrayOfScalars` as `doc show`'s own |
| **DEFECT C** — `ArmOutcome::Success` claims *"Exit 0"* while three rows ship their declared key set at exit 1/3 by design | the driver's | **CLOSED by declaration, and re-driven** | `ArmOutcome::Success`'s doc-comment now carries *"**Not a claim that the verb can never exit non-zero**"* and names the three cells. Driven: `validate` exit 1, `migrate-corpus --dry-run` exit 1, `task validate` exit 3 — key sets exact in all three, stderr 0 bytes. **No fourth cell found** (§6) |
| **DEFECT D** — a pre-dispatch `current_dir()` failure bypassed both funnels: **47/47 leaves** emitted plain non-JSON on stderr | codex-origin | **CLOSED** | §4.2's 47-leaf deleted-cwd sweep: **47/47 exit 1 · stdout 0 bytes · stderr `{"error": "cannot determine the current directory: …"}`**. 0/47 NOT-JSON |
| lead — *"the registry contains all 47 leaves and 60 arms; the unusual closed shapes are explicitly represented"* (M51: **REFUTED**) | codex | **the refutation's cause is CLOSED** | 64 arms; the two shapes that refuted it (`roadmap#milestones` array, `#…/<item>` object) now have rows and were driven |
| lead — *"stdout JSON renderers for setup/uninstall each map to registry rows"* (M51: **REFUTED on the reject path**) | codex | **CLOSED** | same datum as DEFECT A |
| lead — *"the four requested removals are reflected in declarations"* (M51: CONFIRMED) | codex | **STILL TRUE** | §3: 4/4 absent, re-driven incl. the exit-4 review hold |
| lead — *"the only explicitly unpinned success shapes are `describe` and the milestone text acks"* (M51: CONFIRMED) | codex | **STILL TRUE** | §2: 7 `Unpinned` rows, 7/7 `still_pinned` keys on the wire |
| M51 §C — **44 of D2's 48 rows demoted** *repro under-specified* | instrument | **DISCHARGED** | §5 and §8 re-drive the cause axis with the **argv in the table**, not a cell name; the script is at `causes.py` and every row is re-runnable |
| M51 §C — **~11 of row set F's 30 cells demoted** *repro under-specified* | instrument | **DISCHARGED** | §6's run-mode cells carry their argv and their fixture |
| M51 §C — row 14 (`relocate \| Report`) argv names doctype `note`, which no shipped pack carries | instrument | **DISCHARGED** | §8 rebuilds it the same way (`dev/jigc-rig fresh --pack-from-dev --schema note <f>`) and records the schema body used |

**M51 rows: 4 CLOSED / 0 STILL-OPEN.** All four defects are gone from the binary, and the two
demoted repro sets are re-driven with runnable argv.

---

## 2 · Row set A — the 64 registry arms, each driven in its declared cell

**64 / 64 driven. 64 / 64 declared key sets equal the driven key sets. 0 mismatches.**
The `schema_version` partition (`schema_version` rides a row **iff** its `ArmRoot` is
`ResultContract`) holds **64 / 64**. The stream discipline holds **64 / 64** (a success or
adjudication arm writes exactly one JSON document to stdout and no JSON to stderr; a reject
leaves stdout empty and writes exactly one document to stderr).

| # | door (leaf) | arm | argv driven | exit | code\|none | route kind | surface asserted (top-level keys) | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `start` | `OrientationView::UnsetProject` | `jigc --format json start` *(rig `bare`, before `setup`)* | 0 | — | none | `schema_version, state` | matches |
| 2 | `start` | `OrientationView::Clean` | `jigc --format json start` *(rig `fresh`)* | 0 | — | none | `header, next_steps, schema_version, state, workflows` | matches |
| 3 | `start` | `OrientationView::ActiveTask` | `jigc --format json start` *(one open task)* | 0 | — | none | `header, next_steps, schema_version, state, tasks, workflows` | matches |
| 4 | `start` | `Composed` | `jigc --format json start --workflow single-task "harden the cache"` | 0 | — | none | `task, text` | matches |
| 5 | `workflow` | `Composed` | `jigc --format json workflow single-task --preview` | 0 | — | none | `task, text` | matches |
| 6 | `setup` | `Installed` | `jigc --format json setup` *(rig `bare`)* | 0 | — | none | `allowlist_file, findings, guide_file, hook_committed, hook_file, install_commit, line_file` | matches |
| 7 | `uninstall` | `TornDown` | `jigc --format json uninstall` | 0 | — | none | `allowlist_file, findings, line_file, removed` | matches |
| 8 | `upgrade` | `Swept` | `jigc --format json upgrade` | 0 | — | none | `checked, findings, guide, schema_version` | matches |
| 9 | `ingest` | `Triaged` | `jigc --format json ingest` | 0 | — | none | `findings, rows, summary` *(`findings` is new since M51)* | matches |
| 10 | `migrate` | `Composed` | `jigc --format json migrate docs/direction.md --as vision` | 0 | — | none | `task, text` | matches |
| 11 | `migrate-corpus` | `Report` | `jigc --format json migrate-corpus --dry-run` | 0 | — | none | `already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled` | matches |
| 12 | `unmanage` | `Report` | `jigc --format json unmanage docs/decisions/cache-policy.md` | 0 | — | none | `dropped, identity, path` | matches |
| 13 | `rename` | `Report` | `jigc --format json rename adr:cache-strategy --to "Cache policy"` | 0 | — | none | `commit, findings, from, hook_output, new_path, old_path, prose_mentions, referrers, title, to` *(`findings` new since M51)* | matches |
| 14 | `relocate` | `Report` | `jigc --format json relocate note --from docs/legacy-notes/` *(manufactured pack, §8)* | 0 | — | none | `blocked, displaced, moved` | matches |
| 15 | `describe` | `Menu` | `jigc --format json describe` | 0 | — | none | `commands, definitions, schema_version` | matches |
| 16 | `validate` | `StoreSweep` | `jigc --format json validate` | 0 | — | none | `blocking_probes, findings, report_only, schema_version, scope` | matches |
| 17 | `doc create` | `DocAck::Created` | `jigc --format json doc create adr --title "Cache strategy" --task harden-the-cache` | 0 | — | none | `copied_in, existed, findings, op, target` | matches |
| 18 | `doc add-item` | `DocAck::AddedItem` | `jigc --format json doc add-item commit:<t>#trailers --title "Reviewed-by" --task <t>` | 0 | — | none | `copied_in, findings, op, target` | matches |
| 19 | `doc remove-item` | `DocAck::RemovedItem` | `jigc --format json doc remove-item commit:<t>#trailers/reviewed-by --task <t>` | 0 | — | none | `copied_in, findings, op, removed, target` | matches |
| 20 | `doc retitle-item` | `DocAck::RetitledItem` | `jigc --format json doc retitle-item commit:<t>#trailers/reviewed-by --title "Acked-by" --task <t>` | 0 | — | none | `copied_in, findings, op, target, title` | matches |
| 21 | `doc rename` | `DocAck::Renamed` | `jigc --format json doc rename adr:cache-strategy --to "Cache policy" --task <t>` | 0 | — | none | `committed_identity, copied_in, findings, from, op, reslugged, target, title` | matches |
| 22 | `doc set-field` | `DocAck::Field` | `jigc --format json doc set-field adr:cache-strategy#status --value accepted --task <t>` | 0 | — | none | `copied_in, findings, op, target, value` | matches |
| 23 | `doc set-field` | `DocAck::UnsetField` | `jigc --format json doc set-field adr:cache-strategy#status/cites-code --unset --task <t>` | 0 | — | none | `already_absent, copied_in, findings, op, target, unset` | matches |
| 24 | `doc set-slot` | `DocAck::Slot` | `jigc --format json doc set-slot adr:cache-strategy#decision --from-file /tmp/p.txt --task <t>` | 0 | — | none | `chars, copied_in, findings, op, target` | matches |
| 25 | `doc author` | `DocAck::Authored` | `jigc --format json doc author adr --from-file /tmp/pay.yaml --task author-a-decision` | 0 | — | none | `copied_in, findings, op, target` | matches |
| 26 | `doc show` | `WholeDoc::Committed` | `jigc --format json doc show adr:cache-strategy` | 0 | — | none | `fields, item-count, schema-version, sections, slug, type` | matches |
| 27 | `doc show` | `WholeDoc::Staged` | `jigc --format json doc show adr:cache-strategy --task <t>` | 0 | — | none | + `staged` | matches |
| 28 | `doc show` | `FieldsGroupSlice` | `jigc --format json doc show adr:cache-strategy#status` | 0 | — | none | **DataKeyed** object, doc-keyed: `date, schema-version, status` | matches |
| 29 | `doc show` | `SlotSlice` | `jigc --format json doc show adr:cache-strategy#decision` | 0 | — | none | bare JSON **scalar** (string) | matches |
| 30 | `doc show` | **`ItemArraySlice`** | `jigc --format json doc show spec:rate-limiting#criteria` | 0 | — | none | **array of DataKeyed**, element keys `id, statement, title` | matches *(new row)* |
| 31 | `doc show` | **`ItemSlice`** | `jigc --format json doc show milestone-record:cache-rework#tasks/warm-the-read-cache` | 0 | — | none | **DataKeyed** object: `id, intent, status, task-id, workflow` | matches *(new row)* |
| 32 | `doc show` | **`ListFieldSlice`** | `jigc --format json doc show adr:cache-strategy#status/supersedes --task <t>` *(field populated)* | 0 | — | none | **array of bare scalars** | matches *(new row)* |
| 33 | `doc show` | **`CompoundFieldSlice`** | `jigc --format json doc show milestone-record:cache-rework#meta/base` | 0 | — | none | `sha, short` | matches *(new row)* |
| 34 | `doc schema` | `Projection` | `jigc --format json doc schema adr` | 0 | — | none | `contract-version, fields, home, identity, schema-version, sections, type` · `contract-version: 7` | matches |
| 35 | `doc list` | `Index` | `jigc --format json doc list` | 0 | — | none | `docs` | matches |
| 36 | `task list` | `Rows` | `jigc --format json task list` | 0 | — | none | top-level **array**, element keys `id, intent, workflow` | matches |
| 37 | `task diff` | `Ack` | `jigc --format json task diff <t>` | 0 | — | none | `base, code_diff, findings, op, staged_docs, task` | matches |
| 38 | `task validate` | `Report` | `jigc --format json task validate <t>` | 0 | — | none | `findings, schema_version` | matches |
| 39 | `task discard` | `TaskAck::Discarded` | `jigc --format json task discard author-a-decision --force` | 0 | — | none | `commit, dropped, findings, op, task` | matches |
| 40 | `task bind` | `TaskAck::Bound` | `jigc --format json task bind spec spec:rate-limiting enforce-the-rate-limit` | 0 | — | none | `findings, op, role, target, task` | matches |
| 41 | `task finalize` | `Landed` | `jigc --format json task finalize spec-the-rate-limiter` | 0 | — | none | `committed, findings, schema_version` (`committed` carries `displaced, files, hash, hook_output, left_out, manifest, promoted, subject`) | matches |
| 42 | `task finalize` | `Forecast` | `jigc --format json task finalize <clean t> --dry-run` | 0 | — | none | `dry_run, findings, left_out, manifest, subject` | matches |
| 43 | `task finalize` | `Blocked` | `jigc --format json task finalize <blocking t>` | **3** | — | none | `findings, schema_version` | matches |
| 44 | `task finalize` | `MigrationReviewHold` | `jigc --format json task finalize migrate-vision-docs-direction-0e4626a61c6d` | **4** | — | none | `retires, rewrites, source, task` | matches |
| 45 | `config set` | `ConfigAck::Set` | `jigc --format json config set invocation-log true` | 0 | — | none | `committed, key, op, relocated, value` | matches |
| 46 | `config insert-step` | `ConfigAck::InsertStep` | `jigc --format json config insert-step extra.yaml --workflow single-task --after implement` | 0 | — | none | `anchor, committed, op, side, step, workflow` | matches |
| 47 | `config replace-step` | `ConfigAck::ReplaceStep` | `jigc --format json config replace-step workflow:single-task#implement extra.yaml` | 0 | — | none | `committed, op, step, target` | matches |
| 48 | `config remove-step` | `ConfigAck::RemoveStep` | `jigc --format json config remove-step workflow:single-task#implement` | 0 | — | none | `committed, op, target` | matches |
| 49 | `config fill` | `ConfigAck::Fill` | `jigc --format json config fill step:implement#extra-guidance --from-file g.txt` | 0 | — | none | `committed, op, target` | matches |
| 50 | `config fork` | `ConfigAck::Fork` | `jigc --format json config fork workflow:single-task#locate` | 0 | — | none | `base, committed, op, path, target` | matches |
| 51 | `config get` | `Reading` | `jigc --format json config get docs-root` | 0 | — | none | `key, layer, op, rejected, value` | matches |
| 52 | `config list` | `Readings` | `jigc --format json config list` | 0 | — | none | `knobs, op` | matches |
| 53 | `milestone create` | `RecordOnlyAck` | `jigc --format json milestone create "Second rework"` | 0 | — | none | `hook_output, text` | matches |
| 54 | `milestone add-task` | `RecordOnlyAck` | `jigc --format json milestone add-task cache-rework "Third task"` | 0 | — | none | `hook_output, text` | matches |
| 55 | `milestone add-from-spec` | `RecordOnlyAck` | `jigc --format json milestone add-from-spec rate-work spec:rate-limiting` | 0 | — | none | `hook_output, text` | matches |
| 56 | `milestone provision` | `RecordOnlyAck` | `jigc --format json milestone provision cache-rework` | 0 | — | none | `hook_output, text` | matches |
| 57 | `milestone discard` | `RecordOnlyAck` | `jigc --format json milestone discard second-rework` | 0 | — | none | `hook_output, text` | matches |
| 58 | `milestone list-tasks` | `Listing` | `jigc --format json milestone list-tasks cache-rework` | 0 | — | none | `text` | matches |
| 59 | `milestone execute` | `Composed` | `jigc --format json milestone execute cache-rework` | 0 | — | none | `task, text` | matches |
| 60 | `milestone join` | `Report` | `jigc --format json milestone join cache-rework` | 0 | — | none | `findings, milestone, no_docs_from, overlay, schema_version` | matches |
| 61 | `milestone finalize` | `Landed` | `jigc --format json milestone finalize cache-rework` *(1 sub-task, joined)* | 0 | — | none | `committed` | matches |
| 62 | `milestone finalize` | `Blocked` | `jigc --format json milestone finalize cache-rework` *(unjoined)* | **3** | — | none | `findings, schema_version` | matches |
| 63 | *(cross-cutting)* | `Reject::Error` | `jigc --format json config remove-step workflow:single-task#nope` | 1 | `config.anchor-absent` (in the message) | Human (in-message) | stderr `error` | matches |
| 64 | *(cross-cutting)* | `Reject::Findings` | `jigc --format json doc show adr:nope` | 1 | `store.not-found` | Human | stderr `findings, schema_version` | matches |

### Repro block — row set A

```
# the shared construction: each corpus built by DRIVING the binary, never by writing into .jigc/
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

# (CommittedAdr)  jigc start --workflow single-task "harden the cache"
#                 jigc doc create adr --title "Cache strategy" --task harden-the-cache
#                 printf 'Prose.\n' | jigc doc set-slot adr:cache-strategy#{context,decision,consequences} --from-file - --task <t>
#                 jigc doc set-field commit:<t>#type --value feat --task <t>
#                 printf 'a change\n'  | jigc doc set-slot commit:<t>#summary --from-file - --task <t>
#                 echo x > f.txt && git add f.txt && jigc task finalize <t>
# (CommittedSpec) the same through `plan` / `spec:rate-limiting` + one `criteria` item and its statement slot
# (Milestone)     jigc milestone create "Cache rework"; jigc milestone add-task cache-rework "Warm the read cache"
# (ReviewHold)    a committed foreign docs/direction.md, then `jigc migrate … --as vision` + `jigc doc author vision …`
# (Note pack)     dev/jigc-rig fresh --pack-from-dev --schema note note.yaml   (§8 carries note.yaml verbatim)
# then the row's own invocation, e.g.
$JIGC --format json doc show spec:rate-limiting#criteria
```

Observed, aggregated over the 64 rows:

```
total driven rows:                         64
key-set mismatches (declared vs driven):    0
schema_version partition violations:        0    (ResultContract <-> schema_version, both directions)
stream-discipline violations:               0    (success/adjudication -> stdout only; reject -> stderr only, stdout empty)
declared exit honoured in the driven cell: 64/64 (0 / 0 / 3 / 4 / 1 as declared)
```

---

## 3 · Row set B — the `Unpinned(<reason>)` cell · **7 / 7**

Seven arms declare `ArmStatus::Unpinned { reason, still_pinned }`. The driven check is that each
`still_pinned` key is actually on the wire.

| door | arm | argv | exit | `still_pinned` declared | on the wire | verdict |
|---|---|---|---|---|---|---|
| `describe` | `Menu` | `jigc --format json describe` | 0 | `schema_version` | yes | matches |
| `milestone create` | `RecordOnlyAck` | `jigc --format json milestone create "Second rework"` | 0 | `hook_output` | yes | matches |
| `milestone add-task` | `RecordOnlyAck` | `jigc --format json milestone add-task cache-rework "Third task"` | 0 | `hook_output` | yes | matches |
| `milestone add-from-spec` | `RecordOnlyAck` | `jigc --format json milestone add-from-spec rate-work spec:rate-limiting` | 0 | `hook_output` | yes | matches |
| `milestone provision` | `RecordOnlyAck` | `jigc --format json milestone provision cache-rework` | 0 | `hook_output` | yes | matches |
| `milestone discard` | `RecordOnlyAck` | `jigc --format json milestone discard second-rework` | 0 | `hook_output` | yes | matches |
| `milestone list-tasks` | `Listing` | `jigc --format json milestone list-tasks cache-rework` | 0 | *(none)* — and `hook_output` is one of the four deletes | `text` only | matches |

## 3b · Row set C — the four pre-pin deletes · **4 / 4 absent**

| door | deleted key | argv | exit | driven top-level key set | key present? |
|---|---|---|---|---|---|
| `setup` | `installed` | `jigc --format json setup` | 0 | `allowlist_file, findings, guide_file, hook_committed, hook_file, install_commit, line_file` | **no** |
| `uninstall` | `uninstalled` | `jigc --format json uninstall` | 0 | `allowlist_file, findings, line_file, removed` | **no** |
| `task finalize` | `review` | `jigc --format json task finalize migrate-vision-docs-direction-0e4626a61c6d` | **4** | `retires, rewrites, source, task` | **no** |
| `milestone list-tasks` | `hook_output` | `jigc --format json milestone list-tasks cache-rework` | 0 | `text` | **no** |

`hook_output` remains *nested* on the two landing envelopes (inside `committed`), which the
registry's top-level-only pin permits. Driven:
`committed` keys = `displaced, files, hash, hook_output, left_out, manifest, promoted, subject`.

---

## 4 · Row set D — the reject funnels, swept over the whole leaf set

Four **uniform-cause** sweeps, each hitting all 47 leaves with one minimal clap-satisfying argv
per leaf (the argv table is `crates/cli/tests/support/leaf_argv.rs` → `MINIMAL_ARGV`, transcribed
into `leaves.py`; the driver is `sweep.py`). **188 driven cells, 0 undeclared shapes.**

### 4.1 — outside a git repository (47 / 47 in a declared arm) — **M51 DEFECT A's cell**

```
D=$(mktemp -d "${TMPDIR:-/tmp}/nogit2.XXXXXX"); cd "$D"
printf '# Foreign\n' > FOREIGN.md; printf 'title: X\n' > payload.yaml; printf '# Step\n' > step.md
HOME="$D" JIGC=/Users/maurice/.local/bin/jigc python3 sweep.py out.json

  stderr shapes: {('OBJECT', ('error',)): 45, ('OBJECT', ('findings','schema_version')): 2}
  exits:         {1: 47}
  stdout:        EMPTY at 47/47
```

The two `findings` rows are `setup` and `uninstall` — **the M52 fix**, verbatim:

```
$ jigc --format json setup ; echo "exit=$?"
exit=1 · stdout 0 bytes · stderr:
{ "schema_version": 3,
  "findings": [ { "severity":"blocking","probe":"setup","check":"repo-root",
                  "code":"setup.repo-root","key":{"code":"setup.repo-root","target":null},
                  "message":"not inside a git repository (no `.git` found from /private/var/…/nogit.Xq55v8)",
                  "location":null,
                  "route":"run jigc from inside the target git repository; if this project isn't one yet, `git init` here first" } ] }
$ jigc --format json uninstall   -> exit 1, same shape, code "uninstall.repo-root"
```

The other two M51 cells, re-driven on real repos: `setup.dirty-install-path` (after appending to
`CLAUDE.md` on a `fresh` rig) and `uninstall.untracked-workbench-file` (after
`printf 'mine\n' > .jigc/scratch.txt`) — **both now `{findings, schema_version}` on stderr, exit 1,
stdout empty.**

### 4.2 — `PRE_DISPATCH_FAULTS` × `VERB_KINDS`, all three faults (141 / 141)

| fault | phase | fixture | driven result |
|---|---|---|---|
| `cwd-unreadable` | before discovery | `cd "$D" && rmdir "$D"` | **47/47** exit 1 · stdout 0 B · stderr `{"error": "cannot determine the current directory: No such file or directory (os error 2)"}` · **0/47 NOT-JSON** — M51 DEFECT D's cell, now closed |
| `packs-yaml-malformed` | after discovery | `printf 'not: [valid: yaml\n' > .jigc/config/packs.yaml` | **47/47** in a declared arm: 39 `{error}` · 7 `{findings, schema_version}` (the six `task *` doors + `setup`/`uninstall` — an unknown work-unit id or a setup fault adjudicated before pack load) · 1 EMPTY (`task list`, exit 0, reads no pack) |
| `pack-resource-missing` | after pack load | `JIGC_PACK_DIR=$(mktemp -d)` | **47/47** in a declared arm: 39 `{error}` · 5 `{findings, schema_version}` · 2 EMPTY (`task list`, `setup` — both exit 0) · 1 non-JSON prose **beside** a stdout document (`uninstall`'s tracked-file warning — §10 observation 5, permitted by the stream rule) |

```
# the cwd-unreadable repro
cd /tmp && D=$(mktemp -d "${TMPDIR:-/tmp}/gonecwd.XXXXXX"); cd "$D" && rmdir "$D"
JIGC=/Users/maurice/.local/bin/jigc python3 sweep.py out.json
  --- stderr shapes: Counter({('OBJECT', ('error',)): 47})
  --- exits:         Counter({1: 47})
```

### 4.3 — the clap carve-out (declared: **no** JSON on either stream) · 5 / 5

| argv | exit | stdout | stderr | verdict |
|---|---|---|---|---|
| `jigc --format json --version` | 0 | plain, 17 B | empty | matches |
| `jigc --format json --help` | 0 | plain, 6825 B | empty | matches |
| `jigc --format json doc show` *(missing operand)* | 2 | empty | plain, 134 B | matches |
| `jigc --format json nosuchverb` | 2 | empty | plain, 114 B | matches |
| `jigc --format json doc list --type adr` *(no such flag)* | 2 | empty | plain, 174 B | matches |

---

## 5 · Row set E — the cause axis, one argv per cell (43 driven)

Driven by `causes.py` on a corpus carrying a committed `spec:rate-limiting`, an open task
`enforce-the-rate-limit`, and the shipped packs. Every row's **argv is in the script**, so the
demotion M51's reconciler applied to this set does not recur.

| door | cause cell | exit | arm | code |
|---|---|---|---|---|
| `start` | unknown workflow | 1 | `{error}` | `workflow-refs.unknown-workflow` |
| `workflow` | unknown id | 1 | `{error}` | `workflow-refs.unknown-workflow` |
| `migrate` | missing source | 1 | `{error}` | **none** *(§10 obs. 1)* |
| `migrate` | traversal escape | 1 | `{error}` | `migrate.source-untrackable` |
| `unmanage` | traversal token | **0** | `Report` | n/a — declared `PathArgDisposition::NoRule` *(§10 obs. 3)* |
| `rename` | malformed slug head | 1 | `{error}` | `store.malformed-slug` |
| `rename` | **bare slug (`roadmap`)** | 1 | `{error}` | **none** → **DEFECT 3** |
| `relocate` | unknown type | 1 | `{findings}` | `store.unknown-type` |
| `doc create` | gate-blocked | 1 | `{findings}` | `create.gate-blocked` |
| `doc create` | empty title | 1 | `{findings}` | `create.empty-title` |
| `doc add-item` | absent staged instance | 1 | `{error}` | **none** *(§10 obs. 1)* |
| `doc remove-item` | address is not an item | 1 | `{findings}` | `write.wrong-shape` |
| `doc retitle-item` | address is not an item | 1 | `{findings}` | `write.wrong-shape` |
| `doc rename` | malformed slug head | 1 | `{error}` | `store.malformed-slug` |
| `doc set-field` | unknown field | 1 | `{findings}` | `write.unknown-field` |
| `doc set-slot` | unknown section | 1 | `{findings}` | `write.unknown-section` |
| `doc author` | unparseable payload | 1 | `{findings}` | `write.wrong-shape` |
| `doc show` | malformed slug head | 1 | `{error}` | `store.malformed-slug` *(declared negative half of `ENVELOPE_OWED_CODES`)* |
| `doc schema` | unknown type | 1 | `{findings}` | `store.unknown-type` |
| `task diff` · `task validate` · `task finalize` · `task discard` | no such task | 1 | `{findings}` | `finalize.no-task` |
| `task discard` | malformed id (`../..`) | 1 | `{error}` | `work-unit.malformed-id` |
| `task discard` | empty id (`""`) | 1 | `{error}` | `work-unit.malformed-id` |
| `task bind` | undeclared role | 1 | `{error}` | `task-bind.undeclared-role` *(M52 F5 mint; declared flattened)* |
| `config set` | unknown knob | 1 | `{error}` | `config.undeclared-key` |
| `config set` | unusable root (`/etc`) | 1 | `{error}` | `config.unusable-root` |
| `config insert-step` · `config replace-step` | missing source file | 1 | `{error}` | **none** *(§10 obs. 1)* |
| `config remove-step` · `config fork` | no such step | 1 | `{error}` | `config.anchor-absent` |
| `config remove-step` · `config replace-step` | **step present, project-layer only** | 1 | `{error}` | `config.anchor-absent`, wrong verb + dead-end route → **DEFECT 2** |
| `config fill` | no such fill point | 1 | `{error}` | `config.fill-point-absent` |
| `config get` | unknown knob | 1 | `{error}` | `config.undeclared-key` |
| `milestone create` | **empty / unslugable title** | **0** | `RecordOnlyAck` | **none** → **DEFECT 1** |
| `milestone add-task` | **empty intent** | **0** | `RecordOnlyAck` | **none** → **DEFECT 1** |
| `milestone add-task/add-from-spec/list-tasks/provision/execute/join/finalize/discard` | unknown milestone | 1 | `{findings}` | `milestone.unknown` *(8 doors)* |

**Reading.** Every one of the 43 cells lands in **one of the two declared reject arms** (or, for
the two exit-0 rows, in its verb's own success arm). The split is the one `carrier` implements —
a code in `ENVELOPE_OWED_CODES` reaches the findings envelope whichever constructor the producer
reached for; everything else flattens. **No third envelope shape exists anywhere on this axis
any more.** What the cause axis *did* surface is three defects about **content**, not shape
(§11), and one code-less class (§10 obs. 1).

---

## 6 · Row set F — the exit-code cell, and `STORE_EXIT_FLIPS`' seventh member (8 driven)

M51's DEFECT C is closed by declaration. I re-drove all three declared cells **and** hunted a
fourth on the same corpus.

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
sed -i '' 's/^schema-version: .*/schema-version: 99/' docs/roadmap.md
git add -A && git commit -q -m 'ahead stamp'
```

| door | arm | exit | stdout key set | verdict |
|---|---|---|---|---|
| `validate` | `StoreSweep` (`Success`) | **1** | `blocking_probes, findings, report_only, schema_version, scope` | matches the **revised** contract (`schema-conformance.schema-version-ahead`, a `STORE_EXIT_FLIPS` member) |
| `migrate-corpus --dry-run` | `Report` (`Success`) | **1** | `already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled` | matches |
| `task validate` | `Report` (`Success`) | **3** | `findings, schema_version`, stderr 0 B | matches |
| `doc list` | `Index` | 0 | `docs` | **no fourth cell** |
| `ingest` | `Triaged` | 0 | `findings, rows, summary` | **no fourth cell** |
| `doc show roadmap` | `WholeDoc::Committed` | 0 | the declared six | **no fourth cell** |
| `task list` | `Rows` | 0 | `[]` | **no fourth cell** |
| `describe` | `Menu` | 0 | `commands, definitions, schema_version` | **no fourth cell** |

The `Success` doc-comment's *"three rows"* claim is **exact on the corpus I could build**.

Run-mode cells at `Sole` rows (each with its argv, so nothing is demoted): `describe --workflows`
/ `--doctypes` / `--commands` (same three keys each, 28 317 B / 7 387 B / 4 706 B);
`describe single-task` → exit 2, the declared *single-item form is not built* carve-out;
`doc list --task <id>` and `doc list spec` (same `docs` key); `task diff` with nothing staged;
`migrate-corpus` committing run; `unmanage` hit and miss; `relocate` on an empty source directory;
`config get` / `config list` after a `config set`; `upgrade` before and after `uninstall`;
`ingest` with and without candidates; `setup` on `bare` and re-run. **No undeclared key set
appeared at any of them.**

---

## 7 · Row set G — the `doc show` dispatch space, driven at every depth (12 cells)

| argv | exit | JSON root | key set | declared row | verdict |
|---|---|---|---|---|---|
| `doc show adr:cache-strategy` | 0 | object | `fields, item-count, schema-version, sections, slug, type` | `WholeDoc::Committed` | matches |
| `doc show adr:cache-strategy --task <t>` | 0 | object | + `staged` | `WholeDoc::Staged` | matches |
| `doc show adr:cache-strategy#status` | 0 | object | `date, schema-version, status` | `FieldsGroupSlice` | matches |
| `doc show adr:cache-strategy#decision` | 0 | **scalar** | — | `SlotSlice` | matches |
| `doc show adr:cache-strategy#status/supersedes --task <t>` *(populated)* | 0 | **array of scalars** | — | `ListFieldSlice` | matches |
| `doc show spec:rate-limiting#criteria` | 0 | **array** | elements `id, statement, title` | `ItemArraySlice` | matches |
| `doc show milestone-record:cache-rework#tasks` | 0 | **array** | elements `id, intent, status, task-id, workflow` | `ItemArraySlice` | matches |
| `doc show milestone-record:cache-rework#tasks/warm-the-read-cache` | 0 | object | `id, intent, status, task-id, workflow` | `ItemSlice` | matches |
| `doc show milestone-record:cache-rework#meta/base` | 0 | object | `sha, short` | `CompoundFieldSlice` | matches |
| `doc show roadmap` / `doc show roadmap:roadmap` | 0 | object | the declared six | `WholeDoc::Committed` | matches *(both spellings resolve — §10 obs. 4)* |
| `doc show adr:cache-strategy#nosuchsection` | 1 | — | `findings, schema_version` | `Reject::Findings` | matches (`store.no-such-section`) |
| `doc show adr:cache-strategy#status/supersedes` *(declared, **unpopulated**)* | 1 | — | `findings, schema_version` | `Reject::Findings` | shape matches; **the rule it states → DEFECT 4** |

`task finalize`'s four rows driven across their run-modes: `--dry-run` over a *blocking* task
reaches `Blocked` (exit 3), not `Forecast` — **no fifth arm**. `start`'s `Composed` row also
answers `start --task <id>` (resume) with the same two keys and byte-identical length (10 849 B).

---

## 8 · Row set H — the M52-minted registries this axis touches

### `ENVELOPE_OWED_CODES` (4 codes) × their producers — **36 driven cells, 36 / 36 on the owed arm**

The audit's fix 4 made the arm the **code's** property at `carrier`. Driven at every producer I
could reach:

| code | producers driven | arm |
|---|---|---|
| `store.unknown-type` | `doc show` · `doc schema` · `doc add-item` · `doc set-field` · `doc rename` · `rename` · `relocate` · `migrate --as` · `milestone add-from-spec` (**9**) | `{findings, schema_version}` at **9 / 9** |
| `store.not-found` | `doc show` · `rename` · `task bind` · `milestone add-from-spec` (**4**) | `{findings, schema_version}` at **4 / 4** |
| `store.no-such-leaf` | `doc show` (**1** — the only producer) | `{findings, schema_version}` |
| `store.fixed-identity` | `doc show` · `doc set-field` · `doc rename` · `rename` · `milestone add-from-spec` (**5**) | `{findings, schema_version}` at **5 / 5** |

```
$ jigc --format json task bind spec spec:nope enforce-the-rate-limit
exit 1 · stdout 0 B · stderr keys ['findings','schema_version'] · codes ['store.not-found']
    # the M52 fix's own headline cell: this door flattened through rc.15
$ jigc --format json rename adr:nope --to Y
exit 1 · stderr ['findings','schema_version'] · ['store.not-found']
$ jigc --format json milestone add-from-spec cache-rework roadmap:bogus
exit 1 · stderr ['findings','schema_version'] · ['store.fixed-identity']
```

The declared **negative** half holds too: `store.malformed-slug` stays flattened at
`doc show` / `doc rename` / `rename` (3 driven), which is M50's recorded decision.

### `RelocateRefusal::ALL` (10 members / 9 codes) — 7 driven

Built with a manufactured freeze-exempt pack, because every shipped doctype is manifest-governed:

```
cat > note.yaml <<'EOF'
type: note
location: notes/
id-from: title
description: A short freeform note.
usage: something needs writing down and no other doctype fits.
sections:
  - id: body
    slot: { hint: "The note." }
EOF
dev/jigc-rig fresh --pack-from-dev --schema note note.yaml --binary /Users/maurice/.local/bin/jigc
```

| member | argv | exit | arm | code |
|---|---|---|---|---|
| `UnknownDoctype` | `relocate nosuchtype --from docs` | 1 | `{findings}` | `store.unknown-type` *(the one member that projects a key — declared)* |
| `PriorHomeMissing` | `relocate note --from ""` | 1 | `{error}` | `relocate.malformed-prior-home` |
| `PriorHomeUnusable` | `relocate note --from /` | 1 | `{error}` | `relocate.malformed-prior-home` |
| `WorkbenchPriorHome` | `relocate note --from .jigc` | 1 | `{error}` | `config.workbench-root` |
| `InstalledPriorHome` | `relocate note --from .claude` | 1 | `{error}` | `config.unusable-root` |
| `TransientDoctype` | `relocate commit --from docs` | 1 | `{error}` | `store.transient-type` |
| `FrozenDoctype` | `relocate roadmap --from docs` *(shipped packs)* | 1 | `{error}` | `relocate.frozen-doctype` |
| `OccupiedDestination` · `UntrackableDestination` · `UnaddressableDestination` | — | — | — | **not driven** (§12) |

### `ROLLBACK_POPULATIONS` (11) — the reject envelope's `findings`, driven at `config-layer-worktree`/`promote-destination`

```
# a deterministic in-transaction racer: the pre-commit hook edits the promoted destination, then refuses
cat > .git/hooks/pre-commit <<'HK'
#!/bin/sh
echo "RACER WAS HERE" >> docs/decisions/cache-strategy.md 2>/dev/null || true
echo "hook: refusing" >&2
exit 1
HK
chmod +x .git/hooks/pre-commit
$ jigc --format json task finalize do-a-thing
exit 1 · stdout 0 bytes · stderr = ONE document, 1658 B
  keys:  ['findings','schema_version']
  codes: ['finalize.commit-rejected', 'finalize.rollback-conflict']
```

Both findings ride **one** document; the hook's bytes are folded into `commit-rejected`'s
`message`; nothing else is on stderr. That is the M52 stream clause (*"on the arm whose document
is stderr's, nothing else is written there"*) holding at its hardest cell. The non-racing hook
rejection was driven separately (one finding, 793 B, same arm).

### `DESTROYING_DOORS` (6) × `Disposition` — the refusal arm, 2 `Refuse` members driven

| door | disposition | argv | exit | arm | code |
|---|---|---|---|---|---|
| `task discard` | `Refuse{--force}` | foreign file at `.jigc/tasks/<t>/MYNOTES.txt`, then `jigc --format json task discard <t>` | 1 | `{findings, schema_version}` | `task-discard.foreign-bytes` |
| `uninstall` | `Refuse{--force}` | same fixture, `jigc --format json uninstall` | 1 | `{findings, schema_version}` | `uninstall.foreign-bytes` |
| `task finalize` · `milestone finalize` | `Displace` | the landed envelope carries `committed.displaced` | 0 | `Landed` | — (key present, array) |
| `milestone provision` · `milestone discard` | `Refuse{--force}` | — | — | — | **not driven** (§12) |

### `InProgress::ALL` (9) — the posture refusal's arm, 7 driven

`dev/jigc-rig fresh --git-state <member>`, then `jigc --format json task finalize nope`:

```
merge · rebase-merge · cherry-pick · revert · bisect · unmerged-index   -> exit 1 · {"error"} · repo.operation-in-progress
detached                                                                -> exit 1 · {"error"} · repo.head-detached
```

7 / 7 on the flattened arm with a code and a `Human` route inside the message; stdout empty;
stderr exactly one document. This is the **declared** shape and not `:197`'s literal rule — see
§10 obs. 2, which M52's own VERDICT carries as *flagged, not fixed*.

### The fixed-identity doctype set (5) and `doc schema` at contract-version 7 — 16 driven

All 16 resolved doctypes driven through `jigc --format json doc schema <ty>`:
**16 / 16 `contract-version: 7`**, `identity` and `home` present and total on every one
(`home.path: null` only for the transient `commit`, with `kind: "transient"` carrying the fact).
The five `identity.kind: "fixed"` members are `changelog`, `decisions-log`, `deferral-ledger`,
`roadmap`, `vision` — matching F1's "five members across both packs".

---

## 9 · Row set I — the degenerate-mint axis (9 driven)

Driven because the cell set's **exit code** column asked *which arm does this call take*, and two
minting doors take the success arm where their siblings refuse.

| door | title / intent | exit | arm | result |
|---|---|---|---|---|
| `doc create adr --title ""` | empty | 1 | `{findings}` | `create.empty-title` — **guarded** |
| `start --workflow single-task ""` | empty | 1 | `{error}` | `intent must contain at least one letter or digit` — **guarded** (code-less) |
| `milestone create ""` | empty | **0** | `RecordOnlyAck` | mints `milestone:milestone`, **commits** `docs/milestone-records/milestone.md` |
| `milestone create "   "` / `"###"` / `"!!!"` / `"-"` | unslugable | 1 | `{error}` | `milestone.record-exists — milestone \`milestone\` already has a record` |
| `milestone add-task <ms> ""` | empty | **0** | `RecordOnlyAck` | mints `task:task`, **commits** the record item with `intent: ""` |

→ **DEFECT 1.**

---

## 10 · Observations that are **not** defects on this axis (each with its declaration)

1. **A code-less flattened refusal class at four producers.** `crates/cli/src/doc.rs`'s
   `read_staged_routed` (absent staged instance — reached by `doc add-item` / `remove-item` /
   `retitle-item` / `rename` / `set-field` / `set-slot`), `migrate`'s foreign-source read,
   `config insert-step`/`replace-step`'s step-file read, and `start`'s empty-intent bail all
   return a bare `anyhow!` with a route and **no code**. The envelope shape is the declared
   `{error}`; what a driver cannot key is the identity. `design/command-output-contract.md` →
   *the complement* declares `{error}` as the default for a bare `anyhow`, so this is **within**
   the contract as written. Recorded as measured surface, not as a defect: the routes are
   followable and each names a real next act.
2. **`:197`'s selection rule is broader than the binary, and it is declared so.** Every posture
   refusal, every `RelocateRefusal` bar one, and every `config.*` refusal *carries* a `Finding`
   and still takes `{error}`. M52's VERDICT states this verbatim under F4 — *"**Flagged, not
   fixed:** the contract's wider `:197` rule is broader than the binary and is a separate act"* —
   with a trigger. **Not re-reported.**
3. **`jigc unmanage ../../etc/passwd` exits 0 and reports a miss.** `PATH_ARG_OCCURRENCES` row 2
   declares `PathArgDisposition::NoRule { why: "the token is a LOOKUP KEY, matched against the
   spellings the store already records" }`. Declared; **not a defect**.
4. **Two spellings for a fixed identity, both resolving.** `doc schema roadmap` declares
   `identity.address: "roadmap"`; `store.fixed-identity`'s route names `jigc doc show
   roadmap:roadmap`. Driven, **both** resolve at exit 0, so the route is followable — a cosmetic
   divergence between two pinned surfaces, recorded for the reconciler, not raised.
5. **`jigc uninstall` writes a plain-text warning to stderr beside its stdout document**
   (5 tracked paths named). The stream rule says *no JSON on the other stream*, which holds; the
   side-channel clause binds only the arm whose document is stderr's. But the five named paths
   appear on **no** machine key (`findings: []`, `removed` is a seven-flag boolean map), so a
   `--format json` driver never learns which tracked files the teardown removed. Text/JSON parity
   is axis 8's; recorded here because it was found while driving row 7.

---

## 11 · Defects

### DEFECT 1 — two minting doors accept a title that slugs to nothing, and **commit** a record at a fabricated identity at exit 0

**What the contract says.** `crates/engine/src/state.rs:2041`, the guard's own doc-comment:
*"The empty-title block for a non-singleton `create` whose title slugs to nothing (`--title ""`,
`--title "!!!"`): **left unguarded it mints a degenerate `<ty>:<ty>`**."* `design/surface-contract.md`
law 1 (nothing lies) binds the ack and the refusal alike, and `jigc milestone create` /
`jigc milestone add-task` are both `COMMITTING_DOORS` members
(`crates/cli/src/invocation_log.rs` → rows 6 and 7).

**What the binary does.** `engine::milestone::mint_id` (`crates/engine/src/milestone.rs:2470`) and its sibling `mint_sub_id` (`:727`)
apply an *empty → type-name fallback* with **no guard in front of it**:

```rust
pub fn mint_id(title: &str) -> String {
    let slug = crate::slug::slugify(title);
    if slug.is_empty() { crate::slug::slugify("milestone") } else { slug }
}
```

So the exact degenerate `<ty>:<ty>` the engine's guard exists to prevent is what these two doors
mint — and then **commit**.

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

$ jigc --format json milestone create ""
exit=0
{ "hook_output": "",
  "text": "minted milestone:milestone (shared base b52c600)\nrecord: docs/milestone-records/milestone.md …\nrecord commit: a59a108 …\nnext: `jigc milestone add-task milestone \"<intent>\"` …" }

$ git log --oneline -2
a59a108 chore(milestone): open record for milestone:milestone
b52c600 chore(jigc): install jigc workspace config

$ cat docs/milestone-records/milestone.md
---
base: b52c6007968e5df207b2bcb77b82554934848203 b52c600
status: active
schema-version: 3
---

# milestone            <- the H1 is a word the caller never typed

# every later unslugable title collides on that one id:
$ jigc --format json milestone create "   "   # and "###", "!!!", "-"
exit=1
{ "error": "blocking · milestone.record-exists — milestone `milestone` already has a record (its record reads `active`)\n  at: milestone:milestone\n  route: continue it with `jigc milestone add-task milestone \"<intent>\"`, or create this one under a different title" }

# the sibling door, same shape:
$ jigc milestone create "M one" >/dev/null
$ jigc --format json milestone add-task m-one ""
exit=0
{ "hook_output": "", "text": "added task:task to milestone:m-one\nrecord commit: b23c6ec …" }
$ jigc --format json doc show milestone-record:m-one#tasks
[ { "id": "task", "intent": "", "status": "active", "task-id": "task", "workflow": "sub-task" } ]

# the two guarded siblings, same corpus, for contrast:
$ jigc --format json doc create adr --title "" --task <t>
exit=1 · {"findings":[…"code":"create.empty-title"…]}
$ jigc --format json start --workflow single-task ""
exit=1 · {"error":"intent must contain at least one letter or digit (got \"\")"}
```

**Three separate contract breaks in one cell.** (a) A caller token becomes a **path component**
(`docs/milestone-records/milestone.md`) with no door validating it — the class M50 closed for
every *other* token family. (b) The committed record's H1 is `# milestone`, a title the caller
never supplied — law 1 on a **committed** artifact. (c) Every subsequent unslugable title is
refused by a message naming a milestone `milestone` the caller never created, with a route
(`jigc milestone add-task milestone "<intent>"`) pointing at someone else's milestone — law 1 on
the refusal *and* an unfollowable route.

**Class axis:** the four id-minting doors. `doc create` and `start` guard; `milestone create` and
`milestone add-task` do not. Both unguarded doors are `COMMITTING_DOORS` members.

**Tier (charter predicate):** tier 1 — *exit-0 repository harm through a committing door*. It is
not byte **loss**, and that is stated rather than implied.

---

### DEFECT 2 — `config remove-step` and `config replace-step` refuse a step that **is** in the include list, name the wrong verb, and print a route the caller has already satisfied

**What the contract says.** `design/surface-contract.md` law 1 (*nothing lies*) and the universal
route floor (*every finding carries a route that answers*). `crates/cli/src/config.rs:1990`'s
`resolve_fork_bytes` is reached by **three** callers — `run_replace_step` (`:1778`),
`run_remove_step` (`:1832`) and `run_fork` (`:1933`) — and its refusal message is written for
one of them.

**What the binary does.** A step inserted by `jigc config insert-step` exists only in the project
layer, so the pack read fails and both sibling verbs raise *`no step \`X\` body **to fork**`* —
while the very next invocation of the same binary lists `extra` in the resolved include list.

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf 'id: extra\nkind: instruction\nbody: |\n  Do a thing.\n' > extra.yaml
printf 'id: extra2\nkind: instruction\nbody: |\n  Another.\n'  > extra2.yaml
jigc config insert-step extra.yaml --workflow single-task --after implement
  config: inserted step `extra` into `single-task` after `implement` — …

# the binary's OWN include list, three seconds later:
$ jigc --format json config remove-step workflow:single-task#zzz
{ "error": "blocking · config.anchor-absent — `zzz` is not a step in `single-task` as of this edit — its include list is: locate, implement, extra, record-changelog, superseded-context, author-commit, finalize\n  route: name a step id present in the workflow's resolved include list, then re-run" }

# now name one that IS present:
$ jigc --format json config remove-step workflow:single-task#extra ; echo "exit=$?"
exit=1
{ "error": "blocking · config.anchor-absent — no step `extra` body to fork\n  route: name a step id present in the workflow's resolved include list, then re-run" }

$ jigc --format json config replace-step workflow:single-task#extra extra2.yaml ; echo "exit=$?"
exit=1
{ "error": "blocking · config.anchor-absent — no step `extra` body to fork\n  route: name a step id present in the workflow's resolved include list, then re-run" }
```

**Three breaks.** (a) The message names **`fork`** at a `remove-step` and a `replace-step` door.
(b) The route prescribes exactly what the caller did — `extra` *is* in the resolved include list,
by the same binary's own listing — so the route, followed verbatim, re-produces the refusal:
a dead end. (c) One code, `config.anchor-absent`, now covers two different facts (*the id names
no step* and *the step has no pack body*) whose repairs differ, so `(code, target)` does not
discriminate them.

**Class axis:** `resolve_fork_bytes`'s three callers × {step present in the project layer only}.
`config fork` is **unreachable** in that cell (the `check_not_already_forked` guard fires first
with `config.step-id-collision`), so the reachable class is **2 doors**, and the natural sequence
that reaches both is `insert-step` → `remove-step`.

**Tier:** tier 2 — a route dead end and a surface that says something the binary does not do.

---

### DEFECT 3 — `jigc rename` has a refusal outside its own declared-complete refusal registry, and it carries no code

**What the contract says.** `crates/cli/src/rename.rs:40-42`, the module's own claim:
*"**Every one of those refusals** is declared and disposed in one place — [`RefusalKind`], which
carries **the finding code each raises** and whether its route is a command or a judgment."*
`RefusalKind::ALL` has 11 members, each with a code.

**What the binary does.** `parse_addr` (`rename.rs:1186`) bails with a bare `anyhow!` before any
`RefusalKind` is reached, so the bare-slug form — the *natural first guess*, and the form the
module's own comment names as such — refuses with no code, no `key`, and no entry in the error-code
registry. It is also the one form the two sibling doors **accept**.

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

$ jigc --format json rename roadmap --to "New Roadmap" ; echo "exit=$?"
exit=1
{ "error": "`roadmap` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`\n  route: run `jigc describe` for the doctype surface" }
        # no `blocking · <code> —` stem, so no identity and no (code, target) key

# the same token, the same corpus, at the two sibling doors:
$ jigc doc show roadmap                      ; echo "exit=$?"   # exit=0, serves the doc
$ jigc --format json doc rename roadmap --to "New Roadmap" --task <t>
exit=1 · stderr ['findings','schema_version'] · ['write.identity-change']   # accepted AS an address, refused on the real rule

# and `doc schema` declares that spelling canonical:
$ jigc --format json doc schema roadmap | jq '.identity'
{ "kind": "fixed", "address": "roadmap" }
```

**Two breaks.** (a) A refusal of `jigc rename` exists that `RefusalKind` does not declare and
that carries no code, against the module's own stated completeness. (b) Three doors disagree
about whether `roadmap` is an address: `doc show` accepts it, `doc rename` accepts it and refuses
on the identity rule, `rename` says it is not an address at all — while `doc schema`'s
contract-version-7 `identity.address` declares exactly that spelling.

**Class axis:** `parse_addr`'s two bail sites at `jigc rename` × {bare slug, empty half}. All
three cells driven, all three code-less:

```
$ jigc --format json rename ':x'     --to Y   ->  {"error":"`:x` is not a `<type>:<slug>` address — …"}
$ jigc --format json rename 'x:'     --to Y   ->  {"error":"`x:` is not a `<type>:<slug>` address — …"}
$ jigc --format json rename 'roadmap' --to Y  ->  {"error":"`roadmap` is not a `<type>:<slug>` address — …"}
```

**Tier:** tier 3 — a surface that says something the binary does not do (the module's
completeness claim), plus a cross-door grammar divergence.

---

### DEFECT 4 — `doc show` blocks a **declared but unpopulated** optional leaf with `store.no-such-leaf`, and the design doc states the rule over *undeclared* leaves only

**What the contract says.** `design/doc-read-surface.md:86`: *"`engine::store::resolve_leaf` and
its CLI json twin `leaf_json` both gate the slot span on the template's own declaration, so an
**undeclared** leaf **blocks** — `store.no-such-leaf`, routed, on both surfaces."* And
`doc schema` at contract-version 7 advertises the address: `adr` field `supersedes` carries
`"set-field": "adr:<slug>#status/supersedes"`.

**What the binary does.** `resolve_section_leaf` (`crates/engine/src/store.rs:724`) blocks on a
**declared, optional, absent** field with a message asserting the leaf does not exist.

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# … a committed adr:cache-strategy with `supersedes` never set …

$ jigc --format json doc schema adr | jq '.fields[] | select(.id=="supersedes")'
{ "id": "supersedes", "type": "ref", "to": "adr", "required": false,
  "author-required": false, "section": "status",
  "set-field": "adr:<slug>#status/supersedes" }

$ jigc --format json doc show adr:cache-strategy#status/supersedes ; echo "exit=$?"
exit=1
  code:    store.no-such-leaf
  message: `adr:cache-strategy#status/supersedes` names no leaf `supersedes` in section `status`
  route:   name a field the committed section carries (a section's prose slot is the section itself — address it as `#<section>`)

# the SAME address on a corpus where the field is populated:
$ jigc --format json doc show adr:cache-strategy#status/supersedes --task <t>
exit=0 · top-level ARRAY of scalars          # the ListFieldSlice arm
```

**The two homes state two subjects.** `store.rs:718`'s own doc-comment declares the wider
behaviour — *"any other name — **including an optional field the committed doc does not carry** —
is an honest `store.no-such-leaf` block"* — while `design/doc-read-surface.md:86`, the **locked
1.0 contract doc**, states the rule over *undeclared* leaves. One of the two is wrong, and the
message the user reads (*"names no leaf `supersedes` in section `status`"*) is false of the
schema the same binary projects one command earlier. A driver that reads `doc schema`'s
`set-field` addresses and then reads them back cannot tell *"this doctype has no such leaf"* from
*"this instance has not filled it"* — they are one code and one message.

**Class axis:** every optional, unpopulated field leaf of every doctype. Driven on both of `adr`'s optional leaves, on a freshly committed `adr:cache-strategy`:

```
$ jigc --format json doc show adr:cache-strategy#status/cites-code
   store.no-such-leaf | `adr:cache-strategy#status/cites-code` names no leaf `cites-code` in section `status`
$ jigc --format json doc show adr:cache-strategy#status/supersedes
   store.no-such-leaf | `adr:cache-strategy#status/supersedes` names no leaf `supersedes` in section `status`
```

The `ListFieldSlice` arm is reachable only
when the field is populated, which is why the M52 registry's own *declared bound* (*"an arm
exists here iff a driven invocation produced a distinct key set"*) did not surface it.

**Tier:** tier 3 — a pinned surface saying something the binary does not do. **Lowest confidence
of the four**, because it is a two-homes disagreement rather than an unambiguous violation, and
the reconciler should decide which home is normative.

---

## 12 · What was NOT driven, and why — stated plainly

1. **`RelocateRefusal::{OccupiedDestination, UntrackableDestination, UnaddressableDestination}`**
   (3 of 10). Each needs a second managed instance at the destination, a submodule/`.git` home, or
   a stamped file at a managed home under no `<type>:<slug>` identity — all three are axis 1's and
   axis 3's fixture work, and the **envelope** claim they would test here (the flattened arm with a
   code) is already driven at the other seven members.
2. **`ROLLBACK_POPULATIONS` at 9 of its 11 rows.** I drove the two reachable through a
   `task finalize` racer (`config-layer-worktree`, `promote-destination`). The remaining nine need a
   racer per door; that is flow-53 arm 1's and axis 4's subject. What this axis owes is the
   **arm**, and the arm is one document with both findings folded, driven.
3. **`DESTROYING_DOORS` at `milestone provision` and `milestone discard`** (2 of 6). The
   `Refuse` arm is driven at the other two `Refuse` members with the identical code family and the
   identical envelope; the per-door subject classification is axis 3's.
4. **`InProgress::ALL` at `squash-merge`, `am`, `sequencer`, `dangling-sequencer`, `unborn`**
   (5 of 9 members; 7 of the 12 rig git-states driven). The arm was identical at all seven driven,
   and posture is axis 2's door set.
5. **`ManifestKind::ALL` (6) and `SchemaChangeKind::ALL × LOCI` (18).** They shape the *values
   inside* `task finalize | Forecast`'s `manifest` and `migrate-corpus | Report`'s report, not any
   envelope's top-level key set. The two envelopes that carry them **are** driven (rows 42 and 11).
   Axes 4 and 7 own them.
6. **`ENVELOPE_ARMS` under a project-pack composition.** No row's key set is derived from a
   doctype, and the one pack-sensitive row (`describe | Menu`) is declared `Unpinned`. The one
   manufactured-pack drive I did need (`relocate | Report`) is in §8. Axis 7 owns the pack axis.
7. **`hook_output`'s *value* across the whole `COMMITTING_DOORS` producer axis.** Driven at one
   producer (`task finalize`, plus the `chatty-hooks` posture at M51) to confirm the key is on the
   wire and JSON purity survives a speaking hook; `tests/hook_output_axis.rs` and axis 4 own the
   per-producer value.
8. **A concurrent / fan-out envelope.** `milestone join` and `milestone finalize | Landed` were
   driven single-process through a real provisioned worktree; the genuine Task-tool spawn is the
   standing honest bound M51's VERDICT carries, unchanged by this review.
9. **`store.no-such-leaf` at a second producer.** There is only one (`engine::store`'s resolve
   path, reached by `doc show`); stated rather than presented as a 1-of-N sweep.

---

## 13 · Counts

| | |
|---|---|
| rows driven | **431** — A 64 registry arms · B 7 unpinned · C 4 deletes · D 188 uniform-cause leaf sweeps (4 faults/causes × 47 leaves) · D3 5 clap cells · E 43 cause cells · F 8 exit-code + 20 run-mode cells · G 12 dispatch-depth cells · H 36 owed-code + 7 relocate + 5 rollback/destroying + 7 posture + 16 `doc schema` cells · I 9 degenerate-mint cells |
| rows not applicable / not driven | **9 classes** (§12) |
| declared key set == driven key set | **64 / 64** |
| `schema_version` ⇔ `ArmRoot::ResultContract` | **64 / 64** |
| stream discipline (one document, right stream, other stream JSON-free) | **64 / 64**, plus **188 / 188** across the four uniform sweeps and **43 / 43** on the cause axis |
| the four pre-pin deletes absent | **4 / 4** |
| `Unpinned(<reason>)` rows whose `still_pinned` keys are on the wire | **7 / 7** |
| clap carve-out rows conforming | **5 / 5** |
| `ENVELOPE_OWED_CODES` producers on the owed arm | **19 / 19** (9 + 4 + 1 + 5) |
| undeclared envelope shapes found | **0** |
| **M51 rows CLOSED** | **4 / 4** (A · B · C · D) |
| **M51 rows STILL-OPEN** | **0** |
| new defects | **4** (1 · 2 · 3 · 4) |
| `doors_covered` (leaves that are the door of ≥ 1 driven row) | **47 / 47** |

---

## 14 · What this adds over flow-53 arm 6

Flow-53 arm 6 (`crates/cli/tests/flow53_acceptance.rs` → arm 6, on
`crates/cli/tests/pre_dispatch_faults.rs`) crosses **`PRE_DISPATCH_FAULTS` × `VERB_KINDS`** with a
declared `(fault, verb) → (arm, exit)` column and adds `ENVELOPE_ARMS` *where membership is the
assertion*; `format_json_success_axis.rs` carries the registry's four proofs. All of it runs
against the **debug** binary from the build tree, before the completion audit's six fixes.

This review re-drives the whole registry **on the installed release `1.0.0-rc.16`** — a different
binary, a different posture (no `#[cfg(debug_assertions)]` route fence), built *after* those
fixes — and then goes at the questions the suites cannot ask:

- **The four proofs all iterate `ENVELOPE_ARMS`.** *"Is there a production arm the registry does
  not carry?"* is outside all four by construction — the registry's own doc-comment says so, and
  it is where M51's DEFECTS A and B lived. I asked it by driving instead: **188** uniform-cause
  cells across four fault states × 47 leaves, **43** door-specific causes, **20** alternate
  run-modes at the 27 `Sole` rows, **12** address depths at `doc show`. **Zero** undeclared shapes
  came back — which is the positive result that makes M51's two undeclared arms a *closed* class
  rather than an unrepeated anecdote.
- **Arm 6's fault set is three; the reject *cause* space is not.** The suite's `Unreached` cells
  are driven against a control run of the same argv, which proves the phase relation and nothing
  about the cause axis. §5's 43 cells are the cause axis, with the argv in the table — which is
  where DEFECTS 1, 2 and 3 fell out, none of them reachable from a pre-dispatch fault.
- **Proof 4 asserts `Success ⇒ exit 0` at one cell per row.** M51 turned that into DEFECT C; M52
  closed it by rewriting the declaration to name three cells. §6 re-drives all three **and hunts a
  fourth** on the same ahead-stamped corpus across five other `Success` verbs — a negative result
  no registry-iterating proof can produce.
- **`ENVELOPE_OWED_CODES` is a new registry and its fence is a *membership* test.** §8 asks the
  question membership cannot: *does each code reach the owed arm at **every** producer?* — 19
  producer coordinates driven, including the two (`task bind`, `milestone add-from-spec`) the
  audit's fix 4 found in no finding.
- **The three minting doors are not in any of this axis's registries.** They surfaced only because
  the cell set includes *exit code*, and `milestone create ""` takes the **success** arm where its
  two siblings refuse — DEFECT 1, the one tier-1 row this axis produced.

---

# Part II — Reconciliation ledger

Every row below was driven by the **reconciler** on `jigc 1.0.0-rc.16`, independently of the driver's
run. A claim the reconciler could not drive would be an OPEN LEAD with its reason; there are none.

## A · Codex claims → verdict

### C1 — `start`'s two orientation rows declare `next_steps`, and a reachable composition omits it · **CONFIRMED (origin: codex)**

**The claim.** `OrientationView::{Clean, ActiveTask}.next_steps` is `skip_serializing_if = "Vec::is_empty"`
(`crates/engine/src/result.rs:143-145`, `174-175`); the producer returns an empty vector when the composed
pack-set ships neither `planning` nor `ingest-existing` (`crates/cli/src/orient.rs:120-125`, with an explicit
unit witness at `:515-531`); yet **both** registry rows declare `next_steps` present
(`crates/cli/src/render.rs:6043-6057` and `6060-6075`). So on such a pack-set the driven key set is a
**proper subset** of the declared one, at exit 0, on a `Pinned` row — which proof 4
(*"the driven top-level key set equals the declared one"*) asserts cannot happen.

**Source verified by the reconciler**, then **driven**:

```
# a valid custom pack: selectable work-workflows, router, NEITHER off-catalog verb.
cd /Users/maurice/projects/gherrink-jigc
rig=$(dev/jigc-rig fresh --pack-from-dev --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

# baseline on that rig: `ingest-existing` present, `planning` not composed
$ jigc --format json start | python3 -c 'import sys,json;d=json.load(sys.stdin);print(sorted(d))'
['header', 'next_steps', 'schema_version', 'state', 'workflows']

# take the one remaining off-catalog verb out of the throwaway pack (a MOVE, no rm)
mkdir -p "$JIGC_PACK_DIR/../stash"
mv "$JIGC_PACK_DIR/workflows/ingest-existing.yaml" "$JIGC_PACK_DIR/../stash/"

# --- OrientationView::Clean ---
$ jigc --format json start ; echo "exit=$?"
exit=0 · stderr 0 bytes
{ "state": "clean", "schema_version": 3, "header": "Pack: dev/fs-local · Project config: …",
  "workflows": [ architecture-documentation, implement-from-spec, plan, project-setup,
                 quick-fix, record-decision, single-task ] }
   driven top-level keys: ['header', 'schema_version', 'state', 'workflows']
   DECLARED (render.rs:6043-6057): ['header', 'next_steps', 'schema_version', 'state', 'workflows']
                                              ^^^^^^^^^^^^ absent on the wire

# --- OrientationView::ActiveTask ---
$ jigc start --workflow single-task "live task" >/dev/null   # exit 0, mints
$ jigc --format json start ; echo "exit=$?"
exit=0 · stderr 0 bytes
   driven top-level keys: ['header', 'schema_version', 'state', 'tasks', 'workflows']   (state: "active-task")
   DECLARED (render.rs:6060-6075): ['header', 'next_steps', 'schema_version', 'state', 'tasks', 'workflows']
```

**Verdict: CONFIRMED.** Both arms reproduce, on a pack the binary loads without complaint, at exit 0,
with an empty stderr. Codex's own framing — *"this falls inside the registry's admitted bound"* — is
**not quite right and the correction matters**: the declared bound (`render.rs:6019-6025`) covers a
**missing row**, i.e. a key set the census never built. Here the **row exists and its declared key set is
false** for a reachable state. That is the axis's first cell (*declared key set == driven key set*)
failing, not the bound absorbing it. Tier: a pinned contract that says something the binary does not do.

### C2 — M51 DEFECT D is CLOSED (the pre-dispatch `current_dir()` fault reaches a format-aware funnel) · **CONFIRMED (origin: codex)**

```
cd /tmp && D=$(mktemp -d "${TMPDIR:-/tmp}/gonecwd3.XXXXXX"); cd "$D" && rmdir "$D"
for each of: start · validate · describe · doc list · task list · doc show adr:x · config list ·
             task finalize nope · uninstall
  -> exit 1 · stdout 0 bytes · stderr exactly
     {"error": "cannot determine the current directory: No such file or directory (os error 2)"}
  0 of 9 NOT-JSON
```
Nine leaves spot-driven by the reconciler (the driver drove 47/47). The declared `{error}` arm, on the
declared stream, at the declared exit.

### C3 — M51 DEFECT A is CLOSED (`setup`/`uninstall` reject through the findings envelope) · **CONFIRMED (origin: codex)**

```
D=$(mktemp -d "${TMPDIR:-/tmp}/nogit.XXXXXX"); cd "$D"; export HOME="$D"

$ jigc --format json setup      -> exit 1 · stdout 0 B · stderr keys ['findings','schema_version'] · ['setup.repo-root']
$ jigc --format json uninstall  -> exit 1 · stdout 0 B · stderr keys ['findings','schema_version'] · ['uninstall.repo-root']
$ jigc --format json validate   -> exit 1 · stderr {"error": "not inside a git repository (no `.git` found from …)"}
```
The two doors take the findings envelope and their 45 siblings take `{error}` — the M52 split, and no
third (bare-`Finding`-at-root) shape anywhere.

### C4 — M51 DEFECT B is CLOSED (`doc show` declares eight projections incl. arrays, items, list fields, compound leaves) · **CONFIRMED (origin: codex)**

Four of the eight spot-driven by the reconciler on a `fresh` rig carrying a milestone record, plus three
more on the ADR corpus built for C7:

```
$ jigc --format json doc show milestone-record:m-one#tasks            -> exit 0 · ARRAY, elem keys ['id','intent','status','task-id','workflow']   (ItemArraySlice)
$ jigc --format json doc show milestone-record:m-one#tasks/task       -> exit 0 · OBJECT ['id','intent','status','task-id','workflow']               (ItemSlice)
$ jigc --format json doc show milestone-record:m-one#meta/base        -> exit 0 · OBJECT ['sha','short']                                             (CompoundFieldSlice)
$ jigc --format json doc show milestone-record:m-one                  -> exit 0 · OBJECT ['fields','item-count','schema-version','sections','slug','type']
$ jigc --format json doc show adr:cache-policy-v2#status/supersedes --task second-decision
      (value = [adr:older-call, adr:second-call])                     -> exit 0 · ARRAY of bare scalars                                              (ListFieldSlice)
```
`DOC_SHOW_DISPATCH` (`render.rs:5984-5986`) admits object, array and bare-scalar roots, as Codex read it.

### C5 — M51 DEFECT C is CLOSED by declaration, and `STORE_EXIT_FLIPS`' member set is 7 with `home-vacated` among them · **CONFIRMED (origin: codex)**

Source, read by symbol rather than from prose:

```
$ python3 -c "…parse crates/cli/src/render.rs STORE_EXIT_FLIPS…"
7 ['probe-unreliable', 'oob-rename', 'unmigrated-corpus', 'ahead-corpus',
   'orphaned-instance', 'home-vacated', 'foreign-squatter']
```
`home-vacated` is a member; the driver re-drove the three declared `Success`-at-non-zero cells and hunted
a fourth across five other `Success` verbs and found none (§6). Nothing in the reconciler's drives
contradicts either half.

### C6 — the four pre-pin deletes are absent · **CONFIRMED (origin: codex)** — all four re-driven

```
# 1. `installed` absent  (rig `bare`)
$ jigc --format json setup                                  -> exit 0 · keys
  ['allowlist_file','findings','guide_file','hook_committed','hook_file','install_commit','line_file']

# 2. `uninstalled` absent  (rig `fresh`, clean)
$ jigc --format json uninstall                              -> exit 0 · keys ['allowlist_file','findings','line_file','removed']

# 3. `review` absent on the exit-4 hold  (built by driving: a committed foreign docs/direction.md,
#    `jigc migrate docs/direction.md --as vision` -> task migrate-vision-docs-direction-0e4626a61c6d,
#    then the generated `jigc doc author vision --from-file -` payload, then:)
$ jigc --format json task finalize migrate-vision-docs-direction-0e4626a61c6d ; echo "exit=$?"
exit=4 · stderr 0 bytes · stdout keys ['retires','rewrites','source','task']

# 4. `hook_output` absent on the listing
$ jigc --format json milestone list-tasks m-one             -> exit 0 · keys ['text']
```
Note the minted task id is **byte-identical** to the driver's (`…-0e4626a61c6d`), which is the M44
path-hash being deterministic across two independently built corpora — a free cross-check that both
passes drove the same cell.

### C7 — every `Unpinned` row carries a stated reason; none is reasonless · **CONFIRMED (origin: codex)**

```
$ python3 -c "…parse ENVELOPE_ARMS…"
rows: 64 · Unpinned rows: 7 · Unpinned carrying `reason`: 7 · distinct path tuples: 48
        (47 leaf paths + 1 empty path shared by the two cross-cutting reject rows)
```
The driver additionally drove all 7 and found each row's `still_pinned` key on the wire (§3).

### C8 — the serialization inventory: `ENVELOPE_ARMS` is 64 rows over all 47 clap leaves, and no stdout serialization escapes the central renderer · **CONFIRMED in its counts, REFUTED in its completeness** — see D1

Counts verified by the reconciler against the symbols, not the prose: **64** rows, **47** leaf paths +
the 2 empty-path reject rows, `VERB_KINDS` = **47** leaves. `ENVELOPE_OWED_CODES` = 4 members
(`store::NOT_FOUND`, `store::NO_SUCH_LEAF`, `store::UNKNOWN_TYPE`, `task::FIXED_IDENTITY`), enforced at
`carrier`. The `serde_json::to_string*` sites outside the renderer are the three Codex named as
non-stdout (`setup.rs:176` guide front-matter, `adapter.rs` settings, `invocation_log.rs` JSONL) — all
confirmed non-stdout by reading their callers.

**But the completeness half is false, and the reconciler found the counter-example by driving:** Codex's
own inventory lists `cli.rs:1703 println!("{}", render::explain(format, &tree, &pack_label))` inside the
"orientation/composition" family it declared covered by `render.rs:6030-6235`. It is not covered. See
**D1** below — this is the axis's stated hunt (*"an envelope the registry does not carry"*) and neither
pass caught it.

### C9 — zero schema-hash movement across M52 · **CONFIRMED (origin: codex)**

```
$ git ls-files | grep schema-manifest.yaml
crates/cli/pack/config/schema-manifest.yaml
crates/cli/tests/fixtures/prior-schema-prd/config/schema-manifest.yaml
packs/methodology/config/schema-manifest.yaml

$ git diff --stat 577a0099 HEAD -- '*schema-manifest.yaml'
(empty)
```
All three tracked manifests are byte-unchanged from the M51 tag commit through `e519e4eb`. M52's declared
boundary held.

---

## B · Driver defects → status (each **re-driven by the reconciler**)

### DEFECT 1 — two minting doors accept a title that slugs to nothing and **commit** a record at a fabricated identity, at exit 0 · **STANDS (origin: driver, re-driven)**

The source pass is **silent** on this cell — it neither claims it cannot happen nor names it — so the
driver's finding stands on its drive. Re-driven independently:

```
cd /Users/maurice/projects/gherrink-jigc
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

$ jigc --format json milestone create "" ; echo "exit=$?"
exit=0
{ "hook_output": "",
  "text": "minted milestone:milestone (shared base 7e30283)\nrecord: docs/milestone-records/milestone.md …\nrecord commit: c127c16 …\nnext: `jigc milestone add-task milestone \"<intent>\"` …" }

$ git log --oneline -2
c127c16 chore(milestone): open record for milestone:milestone
7e30283 chore(jigc): install jigc workspace config

$ cat docs/milestone-records/milestone.md
---
base: 7e30283ea690c8c99acf8e647f2d18e5785e9dd7 7e30283
status: active
schema-version: 3
---

# milestone                      <- an H1 the caller never typed

$ jigc --format json milestone create "   " ; echo "exit=$?"
exit=1
{ "error": "blocking · milestone.record-exists — milestone `milestone` already has a record (its record reads `active`)\n  at: milestone:milestone\n  route: continue it with `jigc milestone add-task milestone \"<intent>\"`, or create this one under a different title" }

# the sibling door
$ jigc milestone create "M one" >/dev/null
$ jigc --format json milestone add-task m-one "" ; echo "exit=$?"
exit=0
{ "hook_output": "", "text": "added task:task to milestone:m-one\nrecord commit: 7a5cdf2 …" }
$ jigc --format json doc show milestone-record:m-one#tasks
[ { "id": "task", "intent": "", "status": "active", "task-id": "task", "workflow": "sub-task" } ]

# the two guarded siblings on the same corpus, driven bare (no pipe, so the exit is the verb's)
$ jigc --format json doc create adr --title "" --task harden-the-cache ; echo "exit=$?"
exit=1 · stdout 0 B · stderr ['findings','schema_version'] · ['create.empty-title']
$ jigc --format json start --workflow single-task ""
{ "error": "intent must contain at least one letter or digit (got \"\")" }
```
Every element of the driver's account reproduces byte-for-byte, including the fabricated `# milestone`
H1 on a **committed** record and the refusal route that points the caller at a milestone they never made.

### DEFECT 2 — `config remove-step` / `config replace-step` refuse a step that **is** in the resolved include list, name the wrong verb, and print a route the caller has already satisfied · **STANDS (origin: driver, re-driven)**

Source pass silent. Re-driven:

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf 'id: extra\nkind: instruction\nbody: |\n  Do a thing.\n'  > extra.yaml
printf 'id: extra2\nkind: instruction\nbody: |\n  Another.\n'    > extra2.yaml
$ jigc config insert-step extra.yaml --workflow single-task --after implement
config: inserted step `extra` into `single-task` after `implement` — written to `.jigc/config/`, uncommitted …

$ jigc --format json config remove-step 'workflow:single-task#zzz'
{ "error": "blocking · config.anchor-absent — `zzz` is not a step in `single-task` as of this edit — its include list is: locate, implement, extra, record-changelog, superseded-context, author-commit, finalize\n  route: name a step id present in the workflow's resolved include list, then re-run" }
                                                      ^^^^^ the binary's own listing names `extra`

$ jigc --format json config remove-step 'workflow:single-task#extra'  ; echo "exit=$?"
exit=1
{ "error": "blocking · config.anchor-absent — no step `extra` body to fork\n  route: name a step id present in the workflow's resolved include list, then re-run" }

$ jigc --format json config replace-step 'workflow:single-task#extra' extra2.yaml ; echo "exit=$?"
exit=1
{ "error": "blocking · config.anchor-absent — no step `extra` body to fork\n  route: name a step id present in the workflow's resolved include list, then re-run" }
```
All three breaks reproduce: the message names **`fork`** at two non-fork doors; the route prescribes
exactly the state the caller is already in (a verbatim dead end); and one code now covers two facts whose
repairs differ, so `(code, target)` does not discriminate them.

### DEFECT 3 — `jigc rename` carries a refusal outside its own declared-complete `RefusalKind` registry, with no code · **STANDS (origin: driver, re-driven)**

Source pass silent. The module's completeness claim verified in source at `crates/cli/src/rename.rs:40-42`
(*"**Every one of those refusals** is declared and disposed in one place — `RefusalKind`, which carries the
finding code each raises"*), `RefusalKind::ALL` = 11 members (`rename.rs:151-163`), and the code-less bail
at `parse_addr` (`rename.rs:1186-1195`, a bare `anyhow!` reached **before** any `RefusalKind`). Driven:

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

$ jigc --format json rename roadmap --to "New Roadmap" ; echo "exit=$?"
exit=1
{ "error": "`roadmap` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`\n  route: run `jigc describe` for the doctype surface" }
        # no `blocking · <code> —` stem: no identity, no (code, target) key, no registry entry

$ jigc --format json rename ':x' --to Y   -> same code-less text
$ jigc --format json rename 'x:' --to Y   -> same code-less text

# the same token at the sibling doors, same corpus:
$ jigc doc show roadmap ; echo "exit=$?"                                 -> exit=0, serves the doc
$ jigc --format json doc rename roadmap --to "New Roadmap" --task thing
  exit 1 · stderr ['findings','schema_version'] · ['write.identity-change']   # accepted AS an address
$ jigc --format json doc schema roadmap | jq '.identity'
  { "kind": "fixed", "address": "roadmap" }                                   # that spelling is canonical
```
Three doors disagree about whether `roadmap` is an address, while `doc schema` at contract-version 7
declares exactly that spelling. Both halves of the driver's finding reproduce.

### DEFECT 4 — `doc show` blocks a **declared but unpopulated** optional leaf with `store.no-such-leaf`, and the locked contract doc states that rule over *undeclared* leaves only · **STANDS (origin: driver, re-driven; the two-homes reading confirmed in both homes)**

Source pass silent. Both homes read by the reconciler:

* `design/doc-read-surface.md` → *"`engine::store::resolve_leaf` and its CLI json twin `leaf_json` both gate
  the slot span on the template's own declaration, so an **undeclared** leaf **blocks** — `store.no-such-leaf`"*.
* `crates/engine/src/store.rs:718-722` → *"any other name — **including an optional field the committed doc
  does not carry** — is an honest `store.no-such-leaf` block"*.

Driven, on a corpus built entirely by driving the binary (a committed `adr:cache-strategy`):

```
$ jigc --format json doc schema adr | jq '.fields[] | select(.id=="supersedes")'
{ "id":"supersedes","type":"ref","to":"adr","required":false,"author-required":false,
  "section":"status","set-field":"adr:<slug>#status/supersedes" }

$ jigc --format json doc show 'adr:cache-strategy#status/supersedes' ; echo "exit=$?"
exit=1 · stdout 0 B · stderr ['findings','schema_version']
  code:    store.no-such-leaf
  message: `adr:cache-strategy#status/supersedes` names no leaf `supersedes` in section `status`
  route:   name a field the committed section carries (a section's prose slot is the section itself — address it as `#<section>`)

$ jigc --format json doc show 'adr:cache-strategy#status/cites-code'
  store.no-such-leaf | `adr:cache-strategy#status/cites-code` names no leaf `cites-code` in section `status`
```
Both of `adr`'s optional leaves reproduce. The message is false of the schema the same binary projects
one command earlier, and a driver reading `doc schema`'s own `set-field` addresses back cannot tell
*"this doctype has no such leaf"* from *"this instance has not filled it"* — one code, one message.
The driver's own tiering (*lowest confidence of the four; a two-homes disagreement the reconciler should
adjudicate*) is carried forward unchanged: **which home is normative is a decision, not a drive.**

---

## C · New — found by the reconciler, carried by neither pass

### D1 — `jigc start --explain` emits a production `--format json` stdout arm that `ENVELOPE_ARMS` does not carry · **CONFIRMED (origin: reconciler)**

This is the axis's stated hunt, and it is the same class as M51's DEFECT B: an arm the registry does not
carry. It survived **both** passes — the driver's 64-row sweep never reached it (`MINIMAL_ARGV` for
`start` is `&[]`, the orient form, so none of the four uniform 47-leaf sweeps reaches `--explain`
either), and the source pass listed `render::explain`'s print site inside a family it declared covered.

**Source.** `Command::Start { explain: true, .. }` **short-circuits before compose-or-orient**
(`crates/cli/src/cli.rs:664-674`) into `run_explain` (`:1696-1708`), which prints
`render::explain(format, &tree, &pack_label)` — for `Format::Json`, `json(tree)` over
`engine::result::ResolutionTree` (`render.rs:641-650`). `ENVELOPE_ARMS` carries exactly four `start`
rows — `OrientationView::{UnsetProject, Clean, ActiveTask}` and `Composed` — and the `Composed` row's
own `ArmOrigin::Dispatch` reason names only *"an intent, `--workflow` or `--task` composes, a bare
`start` orients"*. Mechanically: the strings `workflow_layer`, `collision_winners` and `pack_inputs`
appear **nowhere** inside the `ENVELOPE_ARMS` literal.

**Driven** (two rigs, two different key sets, both undeclared):

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

$ jigc --format json start --explain ; echo "exit=$?"
exit=0 · stderr 0 bytes · stdout ONE json object, top-level keys:
  ['collision_winners', 'overrides_applied', 'pack_inputs', 'schema_version', 'steps', 'workflow', 'workflow_layer']

$ jigc --format json start --explain "harden the cache"         -> exit 0, the same seven keys
$ jigc --format json start --explain --workflow plan            -> exit 0, the same seven keys
$ jigc --format json task list                                  -> []        # mints nothing, as documented

# and the key set is itself variable (skip-if-empty on two of the seven):
rig=$(dev/jigc-rig fresh --pack-from-dev --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$ jigc --format json start --explain
  ['overrides_applied', 'pack_inputs', 'schema_version', 'steps', 'workflow', 'workflow_layer']
                                                                     # collision_winners drops

# the reject path IS declared-shaped:
$ jigc --format json start --explain --workflow nope ; echo "exit=$?"
exit=1 · stdout 0 B · stderr {"error": "blocking · workflow-refs.unknown-workflow — no workflow `nope` …"}
```

**Why the registry's declared bound does not absorb it.** `render.rs:6019-6025` bounds completeness at
*"a key set reachable only under a **state** the census did not build"*. `--explain` needs no state at
all: it is an argv run-mode on the stock `fresh` corpus, at exit 0, on the surface the registry exists to
close. Proof 1's `⇔` against the clap tree is satisfied (the `start` **leaf** has rows), which is exactly
the hole the registry's own doc-comment warns of — *"a verb can be missing an arm, never a row"*.
`crates/cli/tests/format_json_success_axis.rs` contains no drive of `--explain` (`grep explain` → one
prose hit only), so nothing in the suite would redden.

**Tier:** a pinned contract that is incomplete over a reachable arm — the same tier the wave assigned
M51's DEFECT B, and it carries the `next_steps` variability of **C1** inside it as well
(`collision_winners`/`pack_inputs` are skip-if-empty), so the two findings share a root: *a declared key
set is only as true as the run-mode that produced it.*

### D2 — observation, not a defect: a `0..*` field leaf serves a **bare scalar** at cardinality 1 and an **array** at cardinality ≥ 2

Recorded because it corrects the driver's repro for row 32 (see D · correction 1) and because it is the
kind of shape-by-value dispatch the axis exists to notice. Both shapes have a declared row
(`SlotSlice`/`ArmShape::Scalar` and `ListFieldSlice`/`ArmShape::ArrayOfScalars`), so **no undeclared shape
appears** — the arm *attribution* is what is value-dependent, not the pin.

```
$ jigc --format json doc set-field 'adr:cache-policy-v2#status/supersedes' --value 'adr:cache-strategy' --task second-decision
  exit 0 · ack "value": "adr:cache-strategy"
$ jigc --format json doc show 'adr:cache-policy-v2#status/supersedes' --task second-decision
  "adr:cache-strategy"                    <- bare SCALAR, exit 0

$ jigc --format json doc set-field 'adr:cache-policy-v2#status/supersedes' --value 'adr:cache-strategy,adr:third-decision' --task second-decision
  exit 1 · write.malformed-value          <- the comma form is NOT the list grammar
$ jigc --format json doc set-field 'adr:cache-policy-v2#status/supersedes' --value '[adr:older-call, adr:second-call]' --task second-decision
  exit 0
$ jigc --format json doc show 'adr:cache-policy-v2#status/supersedes' --task second-decision
  [ "adr:older-call", "adr:second-call" ]  <- ARRAY, exit 0
```

---

## D · Corrections to the driver table (recorded here, not written into Part I)

**No row was demoted.** Two corrections, neither of which changes a verdict:

1. **Row 32 / §7 — `ListFieldSlice`'s repro is under-specified.** The table's argv is
   `doc show adr:cache-strategy#status/supersedes --task <t>` annotated *"(field populated)"*. Driven, a
   field populated with **one** value serves a bare scalar, not an array (D2 above). The reported
   `ArrayOfScalars` shape needs **cardinality ≥ 2** — which is what the production recipe uses
   (`crates/cli/tests/format_json_success_axis.rs:790-806`, value `[adr:older-call, adr:second-call]`,
   with its own comment saying so). The reconciler drove the corrected form and the row's verdict
   (*matches*) **stands**; only the repro line needs the cardinality.

2. **§8 heading count — "36 driven cells" is not evidenced; the evidenced number is 19.** The
   `ENVELOPE_OWED_CODES` table beneath it sums to 9 + 4 + 1 + 5 = **19** producer coordinates, and the
   driver's own §13 counts row reports **19 / 19**. The `36` propagates once more into §13's
   per-set tally (*"H 36 owed-code + …"*), so the overall **431** rows-driven figure is high by 17.
   Corrected total: **414**. Nothing about the verdicts changes — 19 / 19 producers on the owed arm is
   what was driven and what holds.

**Checked and NOT corrected:** §5's cause axis was challenged for repro under-specification (it is the
set M51's reconciler demoted). `driver/causes.py` is present and carries a **literal argv vector per
cell** for 43 of the rows; the remaining table rows (`rename` bare slug, `config remove-step`/`replace-step`
on a project-layer step, `milestone add-task` empty intent) carry their argv inside DEFECT 3 / 2 / 1's own
repro blocks. Three cell *names* in the §5 table are spelled differently from their `causes.py` entries
(e.g. *"absent staged instance"* vs `doc add-item / not repeatable`), which is a labelling slip, not a
missing repro. §6's twenty run-mode cells and §8's seven `RelocateRefusal` / seven `InProgress` /
sixteen `doc schema` cells all carry argv. **No demotions.**

---

## E · Doors covered

Every clap leaf that is the door of ≥ 1 **driven** row, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs` → `VERB_KINDS`, read by symbol: 47 leaves). The driver's four uniform sweeps
drive all 47 leaves each; the reconciler re-drove the subset named in Part II.

**47 / 47 — uncovered: none.**

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

Doors the **reconciler itself** drove (the subset above that carries a Part II repro block): `start`,
`setup`, `uninstall`, `validate`, `describe`, `rename`, `migrate`, `doc create`, `doc author`,
`doc rename`, `doc set-field`, `doc set-slot`, `doc show`, `doc schema`, `doc list`, `task list`,
`task finalize`, `config insert-step`, `config remove-step`, `config replace-step`, `config list`,
`milestone create`, `milestone add-task`, `milestone list-tasks` — **24 of 47**.

---

## F · Counts

| | |
|---|---|
| Codex claims entered as leads | **9** (1 lead + 6 M51 dispositions + 2 completeness claims) |
| CONFIRMED by drive | **9** (C8's counts confirmed; its completeness half refuted in scope → D1) |
| REFUTED | **0** |
| OPEN LEADS | **0** — every Codex claim was drivable on the rigs this axis uses |
| driver defects re-driven and standing | **4 / 4** |
| driver defects the source pass contradicted | **0** (the pass is silent on all four) |
| new findings, origin reconciler | **1** (D1) + 1 observation (D2) |
| driver rows demoted for a missing repro | **0** |
| driver-table corrections recorded | **2** (row-32 repro cardinality · the §8/§13 count, 431 → 414) |
| doors covered | **47 / 47**, uncovered: none |

**Net for the axis:** M51's four defects are closed and stay closed under an independent re-drive; the
registry is materially more complete than it was (60 → 64 rows, the two M51 undeclared shapes now
declared); and the axis's own hunt still lands twice — once on a **declared key set that is false for a
reachable pack composition** (C1) and once on a **production arm the registry does not carry at all**
(D1). Both share one root, and it is the registry's method rather than any single row: an arm's declared
key set is only as true as the run-modes that were driven to produce it, and `--explain`,
`next_steps`-less pack compositions and skip-if-empty keys are all outside what the census drove.
