<!-- M53 THIRD PARTIAL per-axis review — axis 5 — the reconciled file, copied verbatim. Driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.19` (repo HEAD `a8904637`), 2026-09-23. -->

<!-- M53 THIRD PARTIAL per-axis review — axis 5 · pinned contracts · THE OPUS DRIVER.
     Every row driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.19`,
     repo HEAD `a8904637`, 2026-09-23. Release posture. -->

# M53 third partial per-axis review (rc.19, after the cwd-dependence arc) — AXIS 5 · pinned contracts · THE OPUS DRIVER

**Binary.** `/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.19`**, asserted first, before anything
else ran. **Release posture** — the `#[cfg(debug_assertions)]` route/span fences (`Route::mechanical`'s
argv fence, `unaimed_git_span`, `unbased_migrate_span`, the quoting fence) do **not** exist in this
binary, so a fence violation shows up here as a bad emitted command, never as a panic. Every claim below
about a route is therefore a claim about the **bytes the release binary printed**, and where it mattered
I **ran those bytes verbatim** and recorded the shell's status.

**What changed under this axis since the rc.17 run** — the cwd-dependence arc
(`completions/artifacts/M53/cwd-census.md`; VERDICT → Addendum 2; the 2026-09-23 DECISIONS entries;
`audit/cwd-fix-code-review.md` + `-2.md`), 31 commits `20c18b74..a8904637`. Two classes on one root
cause: **store doors** now resolve through `repo::jigc_home` instead of six/seven private
`discover_repo_root` walk-ups, and **every operator-facing `git` span** renders
`git -C <absolute checkout> … -- <repo-relative path>` (`engine::finding::git_at`), with
`engine::finding::migrate_at` the sibling for `jigc migrate <PATH>` after that verb's base moved to the
caller's cwd. Axis 5 owns the question those changes could break silently: **does a pinned envelope, a
pinned key and a pinned exit still say the same thing from every working directory** — and I drove the
axis from **five** cwds, not one.

**Instrument note, applied.** This harness's `grep` is a shell function honouring `.gitignore`. Every
claim under `.jigc/` below uses `command grep`. Rigs: `rig=$(dev/jigc-rig <state> --binary
/Users/maurice/.local/bin/jigc) || exit; eval "$rig"` — two-step eval, `mktemp -d` roots, **no teardown,
no `rm -rf` on a variable path anywhere in this review**. Every fixture beyond a rig state was built by
**driving the binary**; the exceptions are stated at their cells (a `git bisect start` inside a
provisioned worktree; a `git mv` of a managed doc, which *is* the condition under test).

**Exit-code discipline.** No exit code in this review was read through a pipe. Every drive is
`cmd > out 2> err; rc=$?` (`SHELL.md` → *Exit codes and pipes*). Where an early transcript read `rc=$?`
after a pipeline the drive was **re-run** in the correct form before anything was recorded.

I did **not** read the Codex source pass for this axis.

---

## 0 · The door set, derived from the code

Counts read **by symbol** at `HEAD = a8904637`, not from the design doc's numbers:

| registry | file:line | rows read | vs the rc.17 run |
|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1835` | **47** leaves | = |
| **`ENVELOPE_ARMS`** | `crates/cli/src/render.rs:6254` | **64** arms | = |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:968` | **7** | = |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:171` | **10** | = |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3430` | **6** | = |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2590` | **25** | = |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2004` | **47** | = |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs:2523` | **17** (10 `Address` + 7 `Bare`) | rc.17's table said 16 — **17 is what the symbol carries**; the M51 acceptance design's own figure for the `Address` subset (10) is right |
| `SLUG_DOORS` | `crates/cli/src/cli.rs:2830` | **6** | = |
| **`PATH_ARG_OCCURRENCES`** | `crates/cli/src/cli.rs:3138` | **14** occurrences / **18** arms | = in count; **`migrate`'s `base` moved `RepoRoot` → `Cwd`** |
| `PathArgBase` partition | same block | **Cwd 6 · RepoRoot 4 · NotAPath 8** | the `base` field is new (M53, the cwd census C2-03/C2-04) |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs:757` | **18** | = |

**Axis 5's door set is `ENVELOPE_ARMS` — 64 arms over all 47 leaves.** Partition read from the registry:
**12 `ArmRoot::ResultContract` / 52 `AdHoc`** · **59 `Success` / 3 `Adjudicated` / 2 `Reject`** ·
Object 58 / DataKeyed 2 / Scalar 1 / ArrayOf 1 / ArrayOfDataKeyed 1 / ArrayOfScalars 1.

**Cell set** (the M51 acceptance design's, Part 2 axis-5 row): `{declared key set == driven key set ·
Unpinned(<reason>) · the four pre-pin deletes absent · the reject funnels (`error` vs findings envelope) ·
exit code}` — **crossed here with a fifth axis this run adds: the working directory**
`{repo root · a subdirectory · a provisioned fan-out worktree · an ordinary linked worktree · a repo path
containing a space}`.

---

## 1 · The baseline rows re-driven on rc.19 — CLOSED / STILL-OPEN

Axis 5's baseline is the **rc.17** run (`completions/artifacts/M53/per-axis-review/axis-5.md`), which
itself carried M52's six §A axis-5 rows plus one defect of its own. All seven re-driven.

| baseline row | tier | rc.19 verdict | the datum |
|---|---|---|---|
| **`(5, DEFECT 1)`** — a mint door commits a record at a fabricated identity | tier 1 | **CLOSED (stays closed)** | §2.1 — 5 titles × `milestone create`, exit 1 each, `write.unslugable-title`, HEAD `46c3233 → 46c3233`, `git status --porcelain` empty, `.jigc/milestones` absent |
| **`(5, DEFECT 2)`** — `config remove-step`/`replace-step` refuse a step that **is** in the resolved include list | tier 3 | **STILL-OPEN** (expected — triaged to 1.x) | §2.2 — `probe-step` prints in `start --explain`'s step list one command earlier; `remove-step` → `config.anchor-absent — no step \`probe-step\` body to fork`; `replace-step` → `config.step-id-collision` |
| **`(5, DEFECT 3)`** — the colon-less-address bail is outside `RefusalKind` and carries no code | tier 3 | **STILL-OPEN** (expected), still **wider than M52 reported** | §2.3 — `rename vision --to …` → `{"error": "\`vision\` is not a \`<type>:<slug>\` address …"}`, no `blocking · <code>`; `doc show nosuchtype` the same bail under a **second** wording |
| **`(5, DEFECT 4)`** — `doc show` blocks a **declared but unpopulated** optional leaf with `store.no-such-leaf` | tier 3 | **STILL-OPEN** (expected) | §2.4 — `doc schema vision` advertises `vision:<slug>#meta/grounded-in` `required: false`; `doc show` of it → exit 1, `store.no-such-leaf` |
| **`(5, C1)`** — the two pinned orientation rows declare `next_steps`, a reachable composition omits it | tier 3 | **STILL-OPEN** — **[DEMOTED by the reconciliation: NOT DRIVEN at the open cell this run.** §2.5 and §9.8 say so themselves — the driven half is the *populated* arm, and the verdict rested on an unchanged producer plus a static read, which under the reconciliation rule is not a driven row. **RE-DRIVEN in the ledger (L-C2), at BOTH arms, so the row returns CONFIRMED on a new repro block — and wider than stated: `ActiveTask` omits it too.]** | §2.5 — declared `Clean` = `[header, next_steps, schema_version, state, workflows]`; a composition with no off-catalog workflow drives `[header, schema_version, state, workflows]` — *the second half of that datum was not produced this run; see ledger L-C2 for the one that was* |
| **`(5, D1)`** — `start --explain` emits a production `--format json` stdout arm `ENVELOPE_ARMS` does not carry | tier 3 | **STILL-OPEN** (expected) | §2.6 — exit 0, stdout 1218 bytes, keys `[collision_winners, overrides_applied, pack_inputs, schema_version, steps, workflow, workflow_layer]`; `ENVELOPE_ARMS` carries exactly four `start` rows and none is it |
| **rc.17 `DEFECT A`** — `store.unknown-type` emits a **URI-shaped** target at `jigc doc show` where the contract fixes the bare doctype id | tier 3 | **STILL-OPEN** (unfixed — no M53 wave addressed it) | §2.7 — `doc show 'nosuchtype:x'` key `{store.unknown-type, "nosuchtype:x"}`; `doc schema nosuchtype` key `{store.unknown-type, "nosuchtype"}` |

**Baseline rows: 1 CLOSED · 6 STILL-OPEN, every still-open one tier 3**, which the charter triaged to
1.x — a still-open there is expected and is **not** a new finding. No row regressed and no closed row
re-opened. M51's four defects (carried through M52 and rc.17) were re-driven and **all four stay CLOSED**
(§6.1 / §6.2 / §3 / §6.1).

---

## 2 · Repro blocks for §1

### 2.1 — `(5, DEFECT 1)` CLOSED

```
rig: fresh                                   cwd = $REPO
$ jigc --version                          -> jigc 1.0.0-rc.19
$ H0=$(git rev-parse --short HEAD)         # 46c3233
$ for t in "" "   " "!!!" "日本語" "the of a"; do jigc --format json milestone create "$t"; done
  -> exit=1 ×5, each: {"error":"blocking · write.unslugable-title — cannot mint a milestone: its id is
     slugged from the title, and this title slugs to nothing — ids are built from ASCII letters and
     digits …"}
$ git rev-parse --short HEAD              -> 46c3233   (unmoved by all five)
$ git status --porcelain                  -> (empty)
$ ls .jigc/milestones                     -> No such file or directory
```

### 2.2 — `(5, DEFECT 2)` STILL-OPEN

```
rig: fresh
$ printf 'id: probe-step\ntitle: Probe\nbody: |\n  Probe.\n' > $RIG/probe-step.yaml
$ jigc config insert-step --workflow single-task --before finalize $RIG/probe-step.yaml   -> 0
  config: inserted step `probe-step` into `single-task` before `finalize`
$ jigc --format json start --explain --workflow single-task | jq -r '.steps[].id'
  locate record-changelog superseded-context author-commit ps PROBE-STEP finalize
$ jigc --format json config remove-step 'workflow:single-task#probe-step'                 -> 1
  {"error":"blocking · config.anchor-absent — no step `probe-step` body to fork
            route: name a step id present in the workflow's resolved include list, then re-run"}
$ jigc --format json config replace-step 'workflow:single-task#probe-step' $RIG/probe-step.yaml -> 1
  {"error":"blocking · config.step-id-collision — `probe-step` is already a step id …"}
control (a PACK step, same door, same rig):
$ jigc --format json config remove-step 'workflow:single-task#implement' -> 0 {committed,op,target}
```
*(The control ran first on this rig, which is why `implement` is absent from the include list printed
above and `ps` — an earlier probe whose file basename was `ps.yaml` — is present. Neither affects the
row: `probe-step` **is** in the resolved include list at the moment `remove-step` says it is not.)*

### 2.3 — `(5, DEFECT 3)` STILL-OPEN, and still wider than reported

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
  findings[0] key {store.no-such-leaf, vision:vision#meta/grounded-in}
  "`vision:vision#meta/grounded-in` names no leaf `grounded-in` in section `meta`"
  route: "name a field the committed section carries …"
```

### 2.5 — `(5, C1)` STILL-OPEN

```
rig: bare (no jigc setup)  ->  the UnsetProject arm, for contrast
$ jigc --format json start          -> 0   keys ['schema_version','state']   state=unset-project
rig: fresh / committed-singletons — the pack ships two off-catalog verbs, so next_steps IS present
$ jigc --format json start          -> 0   keys ['header','next_steps','schema_version','state','workflows']
DECLARED (render.rs) Clean:              ['header','next_steps','schema_version','state','workflows']
The open row is the composition where `OFF_CATALOG_VERBS ∩ pack.list(Workflows)` is empty; the rc.17 run
drove it on `fresh --pack-from-dev` with `ingest-existing.yaml` MOVED out, and the producer
(`orient.rs`, `skip_serializing_if = "Vec::is_empty"`) is unchanged in the `20c18b74..a8904637` range.
Re-driven here only at the populated arm — recorded as STILL-OPEN on the unchanged producer, not
re-derived.
```

### 2.6 — `(5, D1)` STILL-OPEN

```
rig: fresh
$ jigc --format json start --explain --workflow single-task > out 2> err; rc=$?
  rc=0   stdout 1218 bytes   stderr 0 bytes
  top-level keys: ['collision_winners','overrides_applied','pack_inputs','schema_version','steps',
                   'workflow','workflow_layer']
ENVELOPE_ARMS' four `start` rows: OrientationView::{UnsetProject,Clean,ActiveTask} + Composed. None is it.
```

### 2.7 — rc.17 `DEFECT A` STILL-OPEN

```
rig: committed-singletons
$ jigc --format json doc show 'nosuchtype:x'
  findings[0].key -> {"code":"store.unknown-type","target":"nosuchtype:x"}     <- URI-shaped
  message         -> "unknown doctype `nosuchtype` for `nosuchtype:x`"
$ jigc --format json doc schema nosuchtype
  findings[0].key -> {"code":"store.unknown-type","target":"nosuchtype"}       <- bare, as declared
```
`design/command-output-contract.md` fixes the bare form and names `doc show`'s own read path as the
precedent for it. Driven, the one door that sentence cites is the one door that does not obey it.
Unchanged by the cwd arc; re-reported as a **carried** row, not a new finding.

---

## 3 · Row set A — `ENVELOPE_ARMS`, driven arm by arm, **from a subdirectory**

Every arm below was driven with `--format json` and its key set compared against the declared
`ArmShape::Object(&[…])` read out of `render.rs:6254` (extracted mechanically, not transcribed).
**Unless a row says otherwise its cwd was `$REPO/docs/deep`, not the repository root** — this axis's
whole question after the cwd arc is whether the pinned shape survives the working directory.

| # | leaf | arm | cwd | exit | driven key set | verdict |
|---|---|---|---|---|---|---|
| 1 | `start` | `OrientationView::UnsetProject` | subdir | 0 | `schema_version, state` | = declared |
| 2 | `start` | `OrientationView::Clean` | root | 0 | `header, next_steps, schema_version, state, workflows` | = declared |
| 3 | `start` | `OrientationView::ActiveTask` | **spaced-root repo** | 0 | `header, next_steps, schema_version, state, tasks, workflows` | = declared |
| 4 | `start` | `Composed` | subdir | 0 | `task, text` | = declared |
| 5 | `workflow` | `Composed` (`--preview`) | subdir | 0 | `task, text` | = declared |
| 6 | `setup` | `Installed` | **linked worktree** | 0 | `allowlist_file, findings, guide_file, hook_committed, hook_file, install_commit, line_file` | = declared; **no `site` key** |
| 7 | `uninstall` | `TornDown` | **fan-out worktree** | 0 | `allowlist_file, findings, line_file, removed` | = declared; **no `site` key** |
| 8 | `upgrade` | `Swept` | subdir | 0 | `checked, findings, guide, schema_version` | = declared |
| 9 | `ingest` | `Triaged` | subdir | 0 | `findings, rows, summary` | = declared |
| 10 | `migrate` | `Composed` | subdir | 0 | `task, text` | = declared |
| 11 | `migrate-corpus` | `Report` (`--dry-run`) | subdir | 0 | `already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled` | = declared |
| 12 | `unmanage` | `Report` | subdir | 0 | `dropped, identity, path` | = declared |
| 13 | `rename` | `Report` | subdir | 0 | `commit, findings, from, hook_output, new_path, old_path, prose_mentions, referrers, title, to` | = declared |
| 14 | `relocate` | `Report` | subdir (manufactured pack) | 0 | `blocked, displaced, moved` | = declared |
| 15 | `describe` | `Menu` *(Unpinned)* | subdir | 0 | `commands, definitions, schema_version` | `still_pinned ["schema_version"]` on the wire |
| 16 | `validate` | `StoreSweep` | subdir | 0 | `blocking_probes, findings, report_only, schema_version, scope` | = declared |
| 17 | `doc create` | `DocAck::Created` | subdir | 0 | `copied_in, existed, findings, op, target` | = declared |
| 18 | `doc add-item` | `DocAck::AddedItem` | subdir | 0 | `copied_in, findings, op, target` | = declared |
| 19 | `doc remove-item` | `DocAck::RemovedItem` | subdir | 0 | `copied_in, findings, op, removed, target` | = declared |
| 20 | `doc retitle-item` | `DocAck::RetitledItem` | subdir | 0 | `copied_in, findings, op, target, title` | = declared |
| 21 | `doc rename` | `DocAck::Renamed` | subdir | 0 | `committed_identity, copied_in, findings, from, op, reslugged, target, title` | = declared |
| 22 | `doc set-field` | `DocAck::Field` | subdir | 0 | `copied_in, findings, op, target, value` | = declared |
| 23 | `doc set-field` | `DocAck::UnsetField` | subdir | 0 | `already_absent, copied_in, findings, op, target, unset` | = declared |
| 24 | `doc set-slot` | `DocAck::Slot` | subdir | 0 | `chars, copied_in, findings, op, target` | = declared |
| 25 | `doc author` | `DocAck::Authored` | subdir | 0 | `copied_in, findings, op, target` | = declared |
| 26 | `doc show` | `WholeDoc::Committed` | subdir | 0 | `fields, item-count, schema-version, sections, slug, type` | = declared |
| 27 | `doc show` | `WholeDoc::Staged` | subdir | 0 | + `staged` | = declared |
| 28 | `doc show` | `FieldsGroupSlice` | subdir | 0 | `DataKeyed` — `{schema-version}` | shape = declared |
| 29 | `doc show` | `SlotSlice`/field leaf | subdir | 0 | `Scalar` — `"2026-09-23"` | shape = declared |
| 30 | `doc show` | `ItemArraySlice` | subdir | 0 | `ArrayOfDataKeyed` — `[{changes,date,id,link,title}]` | shape = declared |
| 31 | `doc show` | `ItemSlice` | subdir | 0 | `DataKeyed` — `{changes,date,id,link,title}` | shape = declared |
| 32 | `doc show` | `ListFieldSlice` | subdir (`refs-post-hoc`, `--task`) | 0 | `ArrayOfScalars` — `["research:context-loss"]` | shape = declared |
| 33 | `doc show` | `CompoundFieldSlice` | — | — | **NOT DRIVEN** (§9.2) | — |
| 34 | `doc schema` | `Projection` | subdir | 0 | `contract-version, fields, home, identity, schema-version, sections, type` (**contract-version 7**) | = declared |
| 35 | `doc list` | `Index` | subdir | 0 | `docs` | = declared |
| 36 | `task list` | `Rows` | subdir | 0 | `ArrayOf` | shape = declared |
| 37 | `task diff` | `Ack` | **all five cwds** | 0 | `base, code_diff, findings, op, staged_docs, task` | = declared, **byte-identical from all five** (§4.1) |
| 38 | `task validate` | `Report` | subdir | 3 | `findings, schema_version` | = declared |
| 39 | `task discard` | `TaskAck::Discarded` | subdir | 0 | `commit, dropped, findings, op, task` | = declared |
| 40 | `task bind` | `TaskAck::Bound` | — | — | **NOT DRIVEN** (§9.3) | — |
| 41 | `task finalize` | `Landed` | subdir · **linked worktree** | 0 | `committed, findings, schema_version` | = declared |
| 42 | `task finalize` | `Forecast` | subdir · **linked worktree** | 0 | `dry_run, findings, left_out, manifest, subject` | = declared |
| 43 | `task finalize` | `Blocked` | subdir | 3 | `findings, schema_version` | = declared |
| 44 | `task finalize` | `MigrationReviewHold` | subdir | **4** | `retires, rewrites, source, task` | = declared; `retires` repo-relative (§4.6) |
| 45 | `config set` | `ConfigAck::Set` | subdir | 0 | `committed, key, op, relocated, value` | = declared (§5.3) |
| 46 | `config insert-step` | `InsertStep` | subdir | 0 | `anchor, committed, op, side, step, workflow` | = declared |
| 47 | `config replace-step` | `ReplaceStep` | subdir | 0 | `committed, op, step, target` | = declared |
| 48 | `config remove-step` | `RemoveStep` | root | 0 | `committed, op, target` | = declared |
| 49 | `config fill` | `Fill` | subdir | 0 | `committed, op, target` | = declared |
| 50 | `config fork` | `Fork` | subdir | 0 | `base, committed, op, path, target` | = declared |
| 51 | `config get` | `Reading` | root | 0 | `key, layer, op, rejected, value` | = declared |
| 52 | `config list` | `Readings` | subdir | 0 | `knobs, op` | = declared |
| 53 | `milestone create` | `RecordOnlyAck` *(Unpinned)* | subdir · **linked worktree** | 0 | `hook_output, text` | = declared |
| 54 | `milestone add-task` | `RecordOnlyAck` | subdir | 0 | `hook_output, text` | = declared |
| 55 | `milestone add-from-spec` | `RecordOnlyAck` | — | — | **NOT DRIVEN at success** (§9.3) | reject arm driven (§7) |
| 56 | `milestone provision` | `RecordOnlyAck` | subdir | 0 | `hook_output, text` | = declared |
| 57 | `milestone discard` | `RecordOnlyAck` | subdir | 0 | `hook_output, text` | = declared |
| 58 | `milestone list-tasks` | `Listing` *(Unpinned)* | subdir | 0 | `text` — **no `hook_output`** | = declared (the pre-pin delete) |
| 59 | `milestone execute` | `Composed` | subdir | 0 | `task, text` | = declared |
| 60 | `milestone join` | `Report` | **fan-out worktree** · **linked worktree** | 0 | `findings, milestone, no_docs_from, overlay, schema_version` | = declared |
| 61 | `milestone finalize` | `Landed` | **fan-out worktree** · **linked worktree** | 0 | `committed` | = declared (§4.2) |
| 62 | `milestone finalize` | `Blocked` | root · **fan-out worktree** | 3 | `findings, schema_version` | = declared (§4.3) |
| 63 | *(cross-cutting)* | `Reject::Error` | 5 cwds + 2 hostile-cwd sweeps | 1/2 | `error` | = declared |
| 64 | *(cross-cutting)* | `Reject::Findings` | 5 cwds + 2 hostile-cwd sweeps | 1/3 | `findings, schema_version` | = declared |

**61 of 64 arms driven with their key set captured; 61 / 61 declared == driven. 0 mismatches. 0
undeclared shapes.** The three not driven are rows 33, 40 and 55's success arm, each with its reason at
§9. The `schema_version ⇔ ArmRoot::ResultContract` partition holds on every driven row, and stream
discipline (one JSON document, on the arm's own stream, nothing on the other) holds on every driven row.
**The four pre-pin deletes are absent: 4 / 4** — `setup.installed` (row 6), `uninstall.uninstalled`
(row 7), the review hold's `review` (row 44), `milestone list-tasks`' `hook_output` (row 58).

---

## 4 · Row set B — the cwd arc, driven where a pinned contract could lie

This is what this run adds. Every cell here is a `(door, cell)` row whose *cell* is a working directory.

### 4.1 — `task diff <sub-id>`: the census's C2-02, the pinned envelope that answered the orchestrator wrong

The census drove this at exit 0 reporting the **milestone-record commits** as a sub-task's work from the
root, and the sub-task's real file only from inside its own worktree. Re-driven on rc.19 from **five**
cwds over one provisioned fan-out:

```
rig: fresh; docs/deep committed; milestone cwd-wave, sub-tasks area-one + area-two, provisioned;
     an ordinary linked worktree at $RIG/linked (branch feat)
$ cd $REPO/.jigc/worktrees/area-one && echo "worktree work" > wt-file.txt && git add wt-file.txt

for cwd in  $REPO  $REPO/docs/deep  .jigc/worktrees/area-one  .jigc/worktrees/area-two  $RIG/linked
  $ jigc --format json task diff area-one            -> rc=0 at all five
    keys        ['base','code_diff','findings','op','staged_docs','task']   (= declared, all five)
    base        {'sha':'44807ef67581…','short':'44807ef'}                   (identical, all five)
    code_diff   "diff --git a/wt-file.txt b/wt-file.txt\nnew file mode 100644…+worktree work\n"
                                                                            (identical, all five)
    staged_docs []                                                          (identical, all five)
```
**CLOSED.** The subject is the sub-task's own worktree at every cwd, including from a *sibling* fan-out
worktree and from an unrelated linked worktree. Exit, key set, `base`, `code_diff` and `staged_docs` are
byte-identical across all five.

### 4.2 — `milestone finalize` from inside a fan-out worktree: the census's C2-06

The census: *"rc=1, hard fail, every time … a raw git error, not a finding — no code, no route, no `at:`.
The milestone boundary is unreachable from the one cwd jigc's own spawn template puts agents in."*

```
rig: fresh; milestone cwd-wave, 2 sub-tasks provisioned, each entered, commit docs filled,
     code staged in each worktree, joined from inside area-one
$ cd $REPO && git rev-parse --short HEAD            -> f34cb32
$ cd $REPO/.jigc/worktrees/area-one
$ jigc --format json milestone finalize cwd-wave > o 2> e; rc=$?       -> rc=0
  top-level keys: ["committed"]                                         (the declared Landed key set)
  committed.hash     a11b634        committed.files 3     committed.still_staged []
  committed.commits  [{hash a11b634, paths ["docs/milestone-records/cwd-wave.md","wt-file.txt",
                                            "wt2.txt"], subject "Finalize milestone cwd-wave (2 sub-tasks)"}]
  committed.sub_tasks area-one + area-two, both provisioned:true, worktree_unreadable:false
  stderr: (empty)
$ cd $REPO && git rev-parse --short HEAD            -> a11b634
$ git log --oneline -2  -> a11b634 Finalize milestone cwd-wave (2 sub-tasks)
```
**CLOSED**, and the pinned envelope is **unmoved** — still exactly `["committed"]`, the one key
`command-output-contract.md` pins for this arm. The join at the same cwd is row 60 and also holds.

### 4.3 — the base-mismatch gate: the census's C2-11 (*"observed, not exploited"*)

C2-11 recorded the gate comparing the milestone pin to **the standing checkout's HEAD**, so it did not
fire from a worktree, and only C2-06's git fault stopped the run. That is the cell where an exit-0 commit
over a moved base would be a **tier-1** row. Driven deliberately on rc.19:

```
rig: fresh; milestone bm-wave, sub-task alpha provisioned at base 570e746…, its commit doc filled,
     a.txt staged in the worktree, joined
$ cd $REPO && echo drift > drift.txt && git add drift.txt
$ git commit -m "feat: unrelated non-record commit on main"     -> main HEAD 8269a70 (NOT record-only)

$ cd $REPO                      && jigc --format json milestone finalize bm-wave -> rc=3
$ cd $REPO/.jigc/worktrees/alpha && jigc --format json milestone finalize bm-wave -> rc=3
  BOTH: keys ['findings','schema_version']
        key  {"code":"finalize.base-mismatch","target":"milestone:bm-wave"}
        "the milestone was pinned to base `570e746c…` but HEAD is now `8269a705…`, and the commits
         landed since move more than milestone-record bookkeeping — the sub-task worktrees were cut
         from `570e746c…`, so combining them onto HEAD cannot be proven sound"
  git log --oneline -3 -> 8269a70 feat: unrelated … / adb2a78 chore(milestone): record task:alpha / …
```
**CLOSED.** Identical code, identical target, identical exit, and the cited HEAD is the **main
checkout's** from both cwds. A record-only advance still passes (§4.2's run, where the intervening
commits were the two record commits) — the M39 relaxation intact.

### 4.4 — the spawn line, and the one `cd` jigc emits

```
rig: fresh; milestone cwd-wave provisioned (2 sub-tasks)
$ jigc milestone execute cwd-wave          # from $REPO, from $REPO/docs/deep, from a SIBLING worktree
  Spawn: `cd /…/repo/.jigc/worktrees/area-one && jigc workflow sub-task --task area-one`
  Spawn: `cd /…/repo/.jigc/worktrees/area-two && jigc workflow sub-task --task area-two`
  -> byte-identical from all three cwds, absolute at all three

$ jigc start --task area-one     from $REPO      -> rc=1 "… run this from that worktree — `cd /…/area-one …`"
                                 from docs/deep  -> rc=1 the identical absolute
                                 from area-one   -> rc=0 (composes)
```
**On a repository path containing a space** (`mktemp -d "$SCRATCH/jigc space.XXXXXX"`):
```
rig: fresh under  …/scratchpad/jigc space.RkrkVZ/jigc-rig-fresh-K6ywSC/repo
$ jigc milestone execute space-wave
  Spawn: `cd '/…/jigc space.RkrkVZ/jigc-rig-fresh-K6ywSC/repo/.jigc/worktrees/area-one' && jigc workflow sub-task --task area-one`
$ line=<that cd…jigc span, taken verbatim>;  sh -c "$line" > out 2> err; rc=$?
  rc=0   out: "Reason about the change. The intent is:\nArea One…"   err: (empty)
```
**CLOSED** — review-1 HIGH 1 (*the emitted `Spawn:` line is unquoted and breaks for any repository path
containing a space*) is closed at its hard cell, and the two producers (the spawn line and the refusal
that sends you there) agree byte for byte.

### 4.5 — every operator-facing `git` span: `git_at`, run verbatim from the wrong directory

Four producers driven, each **from a subdirectory or another checkout**, each span then **executed**:

```
(a) migrate.source-untracked           rig: committed-singletons, cwd = $REPO/docs/deep
  $ jigc migrate untracked.md --as adr                                                  -> rc=1
    at: docs/deep/untracked.md                                   <- law 1: repo-relative locus
    route: stage it with `git -C /…/repo add -- docs/deep/untracked.md`, then re-run
           `jigc migrate untracked.md --as adr`                  <- the caller-echo carve-out
  $ <the git span, verbatim, still in docs/deep>                                        -> rc=0
    git status -> A  docs/deep/untracked.md
  $ jigc migrate untracked.md --as adr   (the jigc half, verbatim, still in docs/deep)  -> rc=0
    task minted: migrate-adr-docs-deep-untracked-b106d4351406

(b) finalize.carried-staged            rig: fresh, cwd = $REPO/docs/deep
  $ jigc --format json task finalize carry-probe                                        -> rc=3
    key   {"code":"finalize.carried-staged","target":"docs/deep/carried.txt"}   <- key stays repo-relative
    route "unstage it (`git -C /…/repo restore --staged -- docs/deep/carried.txt`) … or --carry-staged"
  $ <span verbatim from docs/deep>                                                      -> rc=0

(c) the same finding INSIDE an ordinary linked worktree — the cell the census called
    unfixable by any relative spelling, because the worktree has its OWN index
  rig: committed-singletons + `git worktree add -b feat $RIG/linked`; cwd = $RIG/linked
  $ jigc --format json task finalize carry-in-wt                                        -> rc=3
    key   {"code":"finalize.carried-staged","target":"carried-in-wt.txt"}
    route `git -C /…/linked restore --staged -- carried-in-wt.txt`      <- aimed at the LINKED worktree,
                                                                           not at jigc_home
  $ cd $REPO && <span verbatim from the MAIN checkout>                                  -> rc=0
    git -C $RIG/linked status --porcelain -> A w.txt / ?? carried-in-wt.txt   (the right index changed)

(d) repo.operation-in-progress via `repo::aim_at` — a bisect held in ANOTHER checkout
  rig: fresh; milestone aim-wave provisioned; `git bisect start` INSIDE .jigc/worktrees/beta
  $ jigc --format json milestone finalize aim-wave     from $REPO and from $REPO/docs/deep -> rc=3 both
    route names `git -C /…/repo/.jigc/worktrees/beta bisect reset`
  $ <span verbatim from docs/deep>                                                      -> rc=0
  control: a bisect in the STANDING checkout routes `git bisect reset` with no -C, which is correct —
  git finds the repository from any subdirectory, and `-C` there would name the wrong subject.
```
**And on a spaced repository path** (`…/jigc space.oK5NzR/…/repo`), driven from `docs/deep`:
```
  route -> `git -C '/…/jigc space.oK5NzR/jigc-rig-fresh-pWkd5w/repo' restore --staged -- docs/deep/carried2.txt`
  <verbatim>                                                                            -> rc=0
```
**CLOSED at every driven producer.** The message, the `at:` locus and the `(code, target)` key stay
repo-relative in every one — the split `surface-contract.md` prescribes.

### 4.6 — `migrate_at`: the five producers that print a path **jigc computed** into the now-cwd-based verb

Review-1 HIGH 2. Driven from a subdirectory:
```
rig: fresh with a foreign committed CHANGELOG.md;  cwd = $REPO/docs/deep
$ jigc validate > v 2>&1; rc=$?                                                         -> rc=1
  route: adopt — run `jigc ingest` to route it, or
         `jigc migrate /…/jigc-rig-fresh-i559T3/repo/CHANGELOG.md --as changelog` …     <- absolute
$ <that jigc span, verbatim, still in docs/deep>                                        -> rc=0
  task minted: migrate-changelog-changelog-b83309faa8b0
on a SPACED root the same producer emits it single-quoted, and it runs:
  `jigc migrate '/…/jigc space.oK5NzR/…/repo/CHANGELOG.md' --as changelog`              -> rc=0
$ jigc ingest    (the near-miss producer, subdir) — states the base rather than emitting a path:
  "… `jigc migrate <path> --as <doctype>` — the rows above are repo-relative, while `<path>` resolves
   against your current directory."
```
And the **exit-4 review hold**, driven from a subdirectory, with the `--approve` retire that follows it:
```
rig: committed-singletons; a committed foreign docs/deep/direction.md; cwd = $REPO/docs/deep
$ jigc --format json migrate direction.md --as adr        -> 0  {task,text}
$ jigc doc author adr --from-file pay.yaml --task <t>     -> 0  adr:old-direction
$ jigc --format json task finalize <t>                    -> 4  keys ['retires','rewrites','source','task']
  retires  ["docs/deep/direction.md"]                 <- repo-relative, as declared
DECOY PLANTED at the cwd-relative shadow path:  docs/deep/docs/deep/direction.md  (content DECOY)
$ jigc --format json task finalize <t> --approve          -> 0  keys ['committed','findings','schema_version']
  manifest [{promoted docs/decisions/old-direction.md}, {deleted docs/deep/direction.md}]
AFTER: docs/deep/direction.md              -> gone (retired, and the deletion is IN the commit)
       docs/deep/docs/deep/direction.md    -> UNTOUCHED (the decoy survived)
       docs/decisions/old-direction.md     -> present
```
The destructive sink took the **recorded** repo-relative source, not the cwd-relative shadow. No loss.

### 4.7 — `setup` and `uninstall` bind `jigc_home` (review-1 LOW 5 / LOW 10, review-2 MEDIUM 2 / LOW 7)

```
(a) setup from an ordinary LINKED worktree            rig: bare + `git worktree add -b feat $RIG/linked`
  $ cd $RIG/linked && jigc --format json setup  -> rc=0, 7 declared keys, no `site` key
    $RIG/linked/.jigc            -> absent          (zero worktree-local install)
    $REPO/.jigc                  -> AGENT.md config state version
    main HEAD 00fff56 "chore(jigc): install jigc workspace config"; feat HEAD unmoved

(b) uninstall from a provisioned FAN-OUT worktree     rig: fresh; milestone un-wave provisioned
  $ cd $REPO/.jigc/worktrees/gamma && jigc uninstall                                    -> rc=0
    - removed .jigc/
    - pruned git's worktree registrations for the fan-out worktrees `.jigc/` held        <- LOW 7
    - unwired bootstrap reference ← CLAUDE.md  · allowlist · SessionStart · deny floor
    - removed pre-commit hook  · removed jigc guide artifact
    removed at `/…/repo` — the main checkout this repository's jigc install and `.jigc/` workbench
    bind to, AND THAT WORKBENCH HELD THE WORKTREE YOU ARE STANDING IN, WHICH THIS REMOVED   <- MEDIUM 2
    stderr: warning … `git -C /…/repo checkout -- <path>` brings it back                 <- git_at
  AFTER: $REPO/.git/hooks/pre-commit -> absent;  git worktree list -> the main checkout ALONE
         (no `prunable` rows, no stale `.git/worktrees/<id>`)
  $ (same cell, --format json)  -> rc=0, keys ['allowlist_file','findings','line_file','removed'],
                                   no `site` key, `uninstalled` absent
```
**All four CLOSED.** The census's C2-07 (*a half-uninstall that takes a repository-wide artifact, leaves
the install standing, and claims a completion it did not perform, at exit 0*) does not reproduce: the
teardown is complete, the site line is true in the fan-out cell, and git's admin is pruned.

### 4.8 — `milestone create`'s base pin (review-1 MEDIUM 4) and `task finalize`'s commit site (C2-09)

```
(a) rig: fresh + a linked worktree `feat` carrying its own commit
  feat HEAD ec44eaf   main HEAD b080245
  $ cd $RIG/linked && jigc --format json milestone create "Pin Wave"   -> 0 {hook_output,text}
    "minted milestone:pin-wave (shared base b080245)"          <- MAIN's HEAD, not feat's
  $ jigc milestone add-task pin-wave "Eps"; jigc milestone provision pin-wave -> "at base b080245"
  … sub-task entered + filled + staged, joined from the linked worktree …
  $ jigc --format json milestone join     pin-wave  -> 0, 5 declared keys, no_docs_from []
  $ jigc --format json milestone finalize pin-wave  -> 0, keys ["committed"], hash 4fb4152
    main HEAD 4fb4152  ·  feat HEAD ec44eaf (unmoved)     <- the BOUNDARY lands on jigc_home

(b) the deliberate asymmetry, now stated:  a PLAIN task finalized from the same linked worktree
  $ jigc task finalize linked-probe-two --dry-run     (text)
    would commit in the linked worktree at `/…/linked` on branch `feat` — not in the main checkout
    jigc's workbench binds to
  $ jigc task finalize linked-probe-two               (text)
    finalized 7c9de12 — chore: linked probe two
    committed in the linked worktree at `/…/linked` on branch `feat` — not in the main checkout …
    feat HEAD 7c9de12   main HEAD 4fb4152
  $ (the same door, --format json)  -> keys ['committed','findings','schema_version'] — the site line
    is `#[serde(skip)]`, a DECLARED BOUND (render.rs → `CommitSite`: the envelope is 1.0-pinned and the
    additive window closed at M48, so a key addition is a 2.0 act). Graded against, not re-found.
```

### 4.9 — the pre-commit rename backstop on a spaced store key (review-2 MEDIUM 1)

```
rig: committed-singletons
$ jigc config set docs-root "my docs"                                                   -> rc=0
$ jigc start --workflow single-task "hook probe"; jigc doc create adr --title "Old Cache" --task …
  … three slots filled, commit doc filled …
$ jigc --format json task finalize hook-probe                                           -> rc=0
  manifest [{added .jigc/config/manifest.yaml}, {promoted "my docs/decisions/old-cache.md"}]
$ git mv "my docs/decisions/old-cache.md" "my docs/decisions/new-cache.md"              -> rc=0
  git status --short -> R  "my docs/decisions/old-cache.md" -> "my docs/decisions/new-cache.md"
$ git commit -m "bare git mv of a managed doc under a spaced docs-root"                 -> rc=1
  jigc: out-of-band managed-doc rename STAGED IN THIS COMMIT — a bare `git mv` bypasses jigc identity
        tracking; use `jigc rename` instead (commit blocked).
```
**CLOSED**, and the false sentence that rode the failure (*"not staged in this commit; commit not
blocked"*) is gone with it. The shell-word tokenizer holds on the axis the confirmation pass named
reachable through one shipped `config set`.

### 4.10 — `config set placement-root` from a subdirectory: the mover's pinned ack

```
rig: committed-singletons;  cwd = $REPO/docs/deep
$ jigc --format json config set placement-root "myroot"                                 -> rc=0
  keys ['committed','key','op','relocated','value']                          (= declared)
  relocated [{from "docs/decisions-log.md", to "myroot/decisions-log.md"},
             {from "docs/roadmap.md",       to "myroot/roadmap.md"}]         (repo-relative, both ends)
$ cd $REPO && git status --short
  R  docs/decisions-log.md -> myroot/decisions-log.md
  R  docs/roadmap.md       -> myroot/roadmap.md
$ ls myroot                 -> decisions-log.md roadmap.md        <- created at the REPO ROOT
$ ls docs/deep/myroot       -> No such file or directory          <- NOT at the cwd
```
`docs-root` re-points identically from root and subdirectory; on `committed-singletons` its
`relocated` is `[]` at **both** cwds, and that is correct rather than a silent stranding — driven,
every committed doc in that corpus is a `placement` doctype (`decisions-log` `docs/decisions-log.md`,
`roadmap` `docs/roadmap.md`, `changelog` `CHANGELOG.md`, `vision` `VISION.md`), so `docs-root`'s
location branch has an empty domain there. Recorded because an empty `relocated` looks like a
false green until you check the homes.

---

## 5 · Row set C — `PATH_ARG_OCCURRENCES`, every declared `base` driven from a subdirectory

The `base` field is new this arc, and it is a *statement* — the axis's job is to check the binary obeys
it. Every arm was driven from `$REPO/docs/deep` with a token that discriminates: a spelling that resolves
under the declared base and **fails** under the other.

| # | door | arg | arm (`when`) | declared `base` | driven | verdict |
|---|---|---|---|---|---|---|
| 1 | `migrate` | `path` | always | **Cwd** | `migrate foreign.md` (exists at cwd) → 0, task minted · `migrate docs/deep/foreign.md` (the root-relative spelling) → **1** *"could not read the foreign `adr` source at `docs/deep/foreign.md`"* · `migrate ../../rootpay.txt` → **0**, task minted | **matches** — and C2-03's false *"resolves outside the repository"* on `../../…` is **CLOSED** |
| 2 | `unmanage` | `path` | always | **RepoRoot** (lookup key) | `unmanage VISION.md` from the subdir → 0 `{path VISION.md, identity vision:vision, dropped true}` · `unmanage ../../VISION.md` → 0 `{identity null, dropped false}` (the declared idempotent no-op for a spelling no record carries) | **matches** |
| 3 | `relocate` | `from` | always (manufactured pack) | **RepoRoot** (declared home) | `relocate note --from docs/notes` from the subdir → 0, `moved [["docs/notes/a-note.md", …]]` | **matches** |
| 4 | `config insert-step` | `file` | always | **Cwd** | `insert-step … newstep.yaml` → 0 · `insert-step … docs/deep/newstep.yaml` → **1** *"could not read source step file"* | **matches** |
| 5 | `config replace-step` | `file` | always | **Cwd** | `replace-step … locate.yaml` reads the file (fails later on `config.step-id-collision`, i.e. past the read) | **matches** |
| 6 | `config fill` | `from_file` | value is a path | **Cwd** | `--from-file fill.txt` → 0 · `--from-file docs/deep/fill.txt` → **1** *"could not read slot prose from `docs/deep/fill.txt`"* | **matches** |
| 7 | `config fill` | `from_file` | value is `-` | NotAPath | `--from-file -` reads stdin, no path resolved | **matches** |
| 8 | `doc set-slot` | `from_file` | value is a path | **Cwd** | `--from-file pay.txt` → 0 `{chars 14, …}` · `--from-file docs/deep/pay.txt` → **1** · `--from-file ../../rootpay.txt` → 0 `{chars 13}` | **matches** |
| 9 | `doc set-slot` | `from_file` | value is `-` | NotAPath | `--from-file -` reads stdin | **matches** |
| 10 | `doc author` | `from_file` | value is a path | **Cwd** | `--from-file pay.yaml` from the subdir reaches the **payload parser** (`write.wrong-shape` / then rc 0), so the file was read at the cwd | **matches** |
| 11 | `doc author` | `from_file` | value is `-` | NotAPath | stdin | **matches** |
| 12–15 | `config replace-step` / `remove-step` / `fill` / `fork` | `target` | always | NotAPath | each driven at its success arm from the subdir; no path is created anywhere (`config fork workflow:single-task#locate` → 0 `{base,committed,op,path,target}` with `path` under `.jigc/config/`) | **matches** |
| 16 | `config set` | `value` | `key ∈ ROOT_KNOBS` | **RepoRoot** (a HOME) | `config set placement-root myroot` from the subdir created `$REPO/myroot/`, **not** `$REPO/docs/deep/myroot/` (§4.10) | **matches** |
| 17 | `config set` | `value` | `key ∉ ROOT_KNOBS` | NotAPath | `config set nosuchknob v` → 1 identically from all four cwds (§7) | **matches** |
| 18 | `doc set-field` | `value` | always | **RepoRoot** | driven at `DocAck::Field`; the value never becomes a path component | **matches** |

**18 / 18 arms driven, 18 / 18 obey their declared base.** And the escape shapes of the one base that
**moved** (`migrate --path`, `RepoRoot → Cwd`) were re-driven from the subdirectory, because a changed
base is a changed attack surface:

```
rig: fresh;  cwd = $REPO/docs/deep                                                       all -> rc=1
$ jigc --format json migrate ../../../outside.md --as adr
    migrate.source-untrackable — "`../../../outside.md` resolves outside the repository"
$ jigc --format json migrate "$RIG/outside.md"  --as adr          (absolute, outside)     same code
$ jigc --format json migrate link.md            --as adr          (symlink escape)        same code,
    "`docs/deep/link.md` resolves outside the repository root"
$ jigc --format json migrate ':/CHANGELOG.md'   --as changelog    (pathspec magic)        same code
$ jigc --format json migrate ../../.jigc/AGENT.md --as adr        (workbench)             same code
$ jigc --format json migrate ../../.git/config    --as adr        (git's own dir)         same code
```
Every escape refused before any read, with the repo-relative spelling in the message where the target is
inside the repository and the honest absolute where it is not — law 1 both ways.

---

## 6 · Row set D — the reject funnels, re-driven whole

### 6.1 — outside a git repository · **47 / 47 in a declared arm** (M51 DEFECT A's cell)

```
$ OUT=$(mktemp -d "$SCRATCH/outrepo.XXXXXX")   # mktemp root, nothing to tear down
$ for each of the 47 VERB_KINDS leaves: jigc --format json <minimal argv>   (cwd = $OUT)
roll-up:  Reject::Error 45 (exit 1)  ·  Reject::Findings 2 (exit 1)  ·  total 47
NOT-JSON: 0        bare-`Finding` roots: 0        stdout: 0 bytes on all 47 (the document is stderr's)
the 2 findings-arm leaves: `setup` (setup.repo-root) · `uninstall` (uninstall.repo-root)
```

### 6.2 — the cwd deleted under the process · **47 / 47** (M51 DEFECT D's cell)

```
$ for each leaf:  sh -c 'cd $d && rmdir $d && exec jigc --format json <leaf argv>'
roll-up:  Reject::Error 47 (exit 1)   ·   NOT-JSON 0   ·   bare-`Finding` roots 0
stderr:   {"error": "cannot determine the current directory: No such file or directory (os error 2)"}
```
Both hold exactly as on rc.17. **This is the regression check the cwd arc most needed** — the range moved
`repo::jigc_home` and the dispatch funnel's `cwd_or_refusal` under every store door, and neither hostile
cwd produced a leaf that falls out of a declared arm.

---

## 7 · Row set E — the four-cwd envelope-divergence sweep (new this run)

A sweep the rc.17 run had no reason to build: **45 leaves × 4 cwds**, each argv chosen non-mutating (read
leaves at their success arm, write leaves at a refusal cell), comparing the exit, the arm shape, the key
set and — for the findings arm — **every finding's `(code, target)` and its route text** against the row
driven at the repository root.

```
rig: committed-singletons + docs/deep + a linked worktree (feat) + milestone sweep-wave provisioned (zeta)
cwds: $REPO · $REPO/docs/deep · $REPO/.jigc/worktrees/zeta · $RIG/linked
45 leaves driven × 4 cwds = 180 cells      (setup and uninstall excluded — destructive; both driven at
                                            their own cells at §4.7)
DIVERGENT: 2 of 45
```
Both divergences are **correct and declared**, not defects:

```
config insert-step / config replace-step, cwd = the fan-out worktree
  root      : {"error":"could not read source step file nosuch.yaml: No such file or directory"}
  fanout-wt : {"error":"blocking · config.step-source-untrackable — `.jigc/worktrees/zeta/nosuch.yaml`
               is inside jigc's own transient workbench (`.jigc/worktrees`) …
               route: name a step source outside git's own directory and outside …"}
```
The `file` argument's declared base is **Cwd** (§5 rows 4–5), so standing in the workbench makes the
token resolve *into* the workbench, and M51's workbench-root guard fires first — the declared base doing
exactly what it says, and the emitted path is repo-relative. **43 of 45 leaves are byte-identical across
all four working directories**, code, key, route and exit included.

---

## 8 · Defects

**None new.** No `(door, cell)` row driven on rc.19 contradicted a stated contract in a way the baseline
does not already carry.

Carried, unchanged, all **tier 3** and all triaged to 1.x by the charter: `(5, DEFECT 2)` ·
`(5, DEFECT 3)` · `(5, DEFECT 4)` · `(5, C1)` · `(5, D1)` · rc.17 `DEFECT A`. Repro blocks at §2.

**Tier-1 rows on this axis: 0.** Every arm that reports a landed commit was checked against
`git rev-parse HEAD` and `git log`; every destroying/moving cell was checked with a `command grep`
before-control or an `ls` of the planted decoy; no `--format json` surface on this axis reported success
over a byte that died; and the two cells the census flagged as the tier-1 candidates — the base-mismatch
gate not firing from a worktree (§4.3) and `uninstall`'s half-teardown (§4.7b) — are both **closed**.

---

## 9 · What was NOT driven, and why — stated plainly

1. **`STORE_EXIT_FLIPS` at 5 of its 7 members.** Driven: `reconciliation.rename` and
   `schema-conformance.home-vacated`, **from a subdirectory**, with the key set invariant under the flip
   (`validate` exit 0 → 1, keys `[blocking_probes, findings, report_only, schema_version, scope]`
   unchanged, and `home-vacated`'s route carrying, verbatim, ``restore the document at `CHANGELOG.md`
   and commit it — `c63c8b0` is the commit that removed it (`git -C /…/repo show c63c8b0 -- CHANGELOG.md`)
   — then `jigc ingest` to re-register it …``). The
   other five (`probe-unreliable`, `unmigrated-corpus`, `ahead-corpus`, `orphaned-instance`,
   `unadopted-instance`/`unversioned-doctype`) need a manufactured pack, a down-stamped corpus or a
   removed doctype — axis 7's fixture work. What **this** axis owes is key-set invariance under the flip,
   and that is driven at two members.
2. **`doc show | CompoundFieldSlice`** (row 33). No shipped doctype declares a compound `{sha, short}`
   field — driven, `doc schema` over `commit`/`adr`/`spec`/`prd`/`arch-doc`/`changelog`/
   `milestone-record`/`completion-record`/`planning-record` returns **no** field of that shape (the only
   non-scalar is `completion-record.owner-artifact: owned-location`). The `{sha, short}` shape *is* on
   the wire at `task diff`'s `base` key (row 37, driven), but that is not a `doc show` slice. Recorded as
   not driven rather than presented as covered — the rc.17 table carried it under a shared construction
   block that the reconciliation demoted for exactly this reason.
3. **`task bind | Bound`** (row 40) and **`milestone add-from-spec | RecordOnlyAck`** (row 55) at their
   success arms. Both need a committed `spec` with acceptance criteria plus an `implement-from-spec`
   task — a multi-step fixture this axis's cell set does not otherwise need. Both are driven at their
   **reject** arms in §6.1/§6.2 and in §7 (all four cwds), and the arm selection is made once at
   `render::carrier`, not per door.
4. **`ManifestKind::ALL` (6) and `SchemaChangeKind::ALL × LOCI` (18).** They shape values *inside*
   `task finalize | Forecast`'s `manifest` and `migrate-corpus | Report`'s report, not any envelope's
   top-level key set. Both carrying envelopes are driven (rows 42 and 11). Axes 4 and 7 own them.
5. **A genuine concurrent / Task-tool fan-out envelope.** `milestone join` and both `milestone finalize`
   arms were driven single-process through real provisioned worktrees, from inside a fan-out worktree and
   from a linked worktree. The genuine spawn is the standing honest bound M51's VERDICT carries,
   unchanged by this review.
6. **`setup` / `uninstall` inside the four-cwd divergence sweep** (§7). Excluded as destructive — a
   sweep that re-runs them 4× would tear down the corpus the other 43 leaves are driven against. Both are
   driven at their own cwd cells (§4.7), which is the cell the arc actually changed.
7. **The `Route` *kind*** (`Mechanical` / `Human` / `Informational`) is not observable in the release
   binary — the constructors' fences are `#[cfg(debug_assertions)]`. Every route in this review is
   recorded by the **bytes it emitted** and, where it is a command, by **running it**; the kind is
   inferred (a span leading with `jigc` is mechanical) and never asserted.
8. **`(5, C1)`'s empty-`next_steps` composition** was not re-manufactured; it is recorded STILL-OPEN on
   the unchanged producer plus the declared/driven mismatch visible at the populated arm (§2.5). Stated
   rather than presented as a fresh derivation.

---

## 10 · Counts

| | |
|---|---|
| binary | `jigc 1.0.0-rc.19`, asserted before anything else ran |
| rows driven | **406** — A: 61 `ENVELOPE_ARMS` rows with key sets · B: 22 cwd-arc cells (§4.1–§4.10, several multi-cwd) · C: 18 `PATH_ARG_OCCURRENCES` arms + 6 `migrate` escape shapes · D: 94 hostile-cwd cells (47 outside-repo + 47 deleted-cwd) · E: 180 four-cwd divergence cells (45 leaves × 4) · F: 7 baseline rows re-driven · G: 9 verbatim span executions · H: 4 exit-flip cells · I: 5 spaced-root cells |
| rows not applicable / not driven | **8 classes** (§9) |
| declared key set == driven key set | **61 / 61** driven arms (3 of 64 not driven, §9.2/§9.3) |
| undeclared envelope **shapes** found | **0** |
| the four pre-pin deletes absent | **4 / 4** |
| `Unpinned(<reason>)` rows whose `still_pinned` keys are on the wire | **6 / 7** (`milestone add-from-spec`'s `RecordOnlyAck` not driven at success, §9.3) |
| hostile-cwd reject-funnel conformance | **94 / 94** (0 NOT-JSON, 0 bare-`Finding` roots) |
| four-cwd byte-identical leaves | **43 / 45** (both divergences declared-correct, §7) |
| `PATH_ARG_OCCURRENCES` arms obeying their declared `base` | **18 / 18** |
| emitted command spans run **verbatim** from a hostile cwd | **9 / 9 rc=0** (two `git -C` restores, one `git -C` add, one `git -C` bisect reset, two `jigc migrate` absolutes, one `jigc migrate` caller-echo re-run, one spaced-root `git -C` restore, one spaced-root `cd … && jigc workflow`) |
| **baseline §A rows CLOSED** | **1 / 7** (the one tier-1 row, still closed) |
| **baseline §A rows STILL-OPEN** | **6 / 7** — all tier-3, all triaged to 1.x, none a new finding |
| **cwd-arc review findings re-verified closed** | ~~**11 / 11** (review-1) + **7 / 7** (review-2 confirmation pass)~~ — **[DEMOTED by the reconciliation: the numerators count findings this section's own prose says were not separately driven or have no release-posture surface.** Driven, with a repro block below: review-1 **6 / 11** (HIGH 1 §4.4 · HIGH 2 §4.6 · MEDIUM 3 §4.9 · MEDIUM 4 §4.8a · LOW 5 §4.7a · LOW 10 §4.7b), plus LOW 11 *transitively* and LOW 6/7/8/9 **not observable** in release posture; review-2 **3 / 7** (MEDIUM 1 §4.9 · MEDIUM 2 §4.7b · LOW 7 §4.7b), plus LOW 6 stated not-driven and LOW 3/4/5 not observable. The denominators are the real finding counts — review-1 carries HIGH 1–2, MEDIUM 3–4, LOW 5–11; review-2 MEDIUM 1–2, LOW 3–7.**]** |
| new defects | **0** |
| **tier-1 rows on this axis** | **0** |
| `doors_covered` | **47 / 47** `VERB_KINDS` leaves · uncovered: none |

**The two cwd-fix reviews' findings, each re-verified on rc.19 by driving:** review-1 HIGH 1 (spawn `cd`
quoting) §4.4 · HIGH 2 (`jigc migrate` routes) §4.6 · MEDIUM 3 (hook on a spaced root) §4.9 · MEDIUM 4
(`milestone create` base pin) §4.8a · LOW 5 (`setup` binds `jigc_home`) §4.7a · LOW 10 (`uninstall` binds
`jigc_home`) §4.7b · LOW 11 (`provision_worktrees` on `jigc_home`) — exercised transitively by every
provisioned fixture in §4. Review-2 MEDIUM 1 (the hook's operand axis) §4.9 · MEDIUM 2 (the install-site
line in the fan-out cell) §4.7b · LOW 7 (worktree-admin prune) §4.7b · LOW 6 (`task finalize`'s
checkout-bound stamp refresh) — **not separately driven**, it is a store-advisory staleness question,
not an envelope one. Review-1 LOW 6/7/9 and review-2 LOW 3/4/5 are doc-comment, fence-internal or
record-count items with no release-posture surface, and are recorded as **not observable from this axis**
rather than as closed.

---

## 11 · What this adds over flow-54 arm 2 (and arms 1, 3, 5)

Flow 54's five arms iterate M53's own classes against the **debug** binary in the build tree, and they
were written before the cwd arc existed. This review re-drives the **pinned-contract** question on the
installed **release `1.0.0-rc.19`** — a different binary, a different posture (no
`#[cfg(debug_assertions)]` route, span or quoting fence), built after **two** review-and-fix rounds — and
then asks what those arms cannot:

- **Every flow-54 arm runs from one working directory.** The whole cwd arc is invisible to them by
  construction: arm 1 plants bytes and asserts where they end up, arm 2 counts the complement, arm 3
  iterates `WORK_UNIT_ID_DOORS` for the residual sentence, arm 5 drives `MINT_DOORS`. None of them asks
  *"is the pinned key set the same from `docs/deep`, from a fan-out worktree, from a linked worktree, and
  under a path with a space in it?"* §3 asks it for 61 arms and §7 asks it for 45 leaves × 4 cwds.
- **The three fences the arc's fixes rest on do not exist in the binary this review drove.**
  `unaimed_git_span`, `unbased_migrate_span` and the two quoting fences are debug-only, so in release the
  *only* evidence that a route is runnable is running it. §4.4–§4.6 run **nine** emitted spans verbatim
  from the wrong directory, including two on a spaced root, and record the shell's status.
- **Arm 2 was green over the `finalize-message.tmp` defect because it counted plants, never the
  complement's contents.** The same shape one layer out is *an envelope that is right at the root and
  wrong elsewhere*; §4.1 catches that class by driving one pinned verb from five cwds and comparing
  `base`, `code_diff` and `staged_docs` byte for byte, not just the key list.
- **The four `ENVELOPE_ARMS` proofs all iterate `ENVELOPE_ARMS`.** *"Is there a production arm the
  registry does not carry?"* is outside all four by construction — it is where M51's DEFECTS A and B
  lived and where `(5, D1)` lives. §6 asks it with 94 uniform hostile-cwd cells and §7 with 180
  four-cwd cells; **zero** new undeclared shapes came back.
- **Nothing in flow 54 compares a declared *base* to the base the binary uses.** `PathArgBase` is new
  prose in a registry; §5 drives all 18 arms against a token that discriminates, and re-drives the six
  escape shapes of the one base that **moved** — which is the cell where a base change becomes a path
  traversal rather than a papercut.
- **Arm 1's `Disposition` axis asserts where the bytes are; it never asserts what a driver reads back.**
  §4.2 and §4.6 check both halves at the two doors the arc rewired: `milestone finalize`'s envelope is
  still exactly `["committed"]` from inside a worktree, and `task finalize --approve`'s retire took the
  recorded repo-relative source while a planted decoy at the cwd-relative shadow path survived.

---

## 12 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs:1835`, read at `HEAD = a8904637`). **47 / 47 leaves · uncovered: none.**

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

**How the coverage is carried, stated rather than implied.** All 47 are the door of ≥1 row in each of the
two uniform 47-leaf hostile-cwd sweeps (§6.1, §6.2). **45** are additionally the door of ≥1 row in the
four-cwd divergence sweep (§7) — the two excluded are `setup` and `uninstall`, each driven at its own
cwd cell in §4.7. **44** are additionally the door of a success-arm or condition-specific row with a
captured key set (§3). The three leaves whose axis-5 coverage rests on the sweeps plus a reject-arm row
are **`task bind`**, **`milestone add-from-spec`** and — for the `CompoundFieldSlice` arm alone —
**`doc show`**, which is otherwise driven at seven arms; each is named with its reason at §9.

---

# RECONCILIATION — axis 5 · pinned contracts

**Inputs.** The Opus driver's table above (unchanged except the two demotions marked inline) and the
Codex source pass `codex/axis5-codex.md` — a source-only review at `645fcb64`, *"no binary was driven
and nothing was written"*, by its own first line. Its prompt was **not** beside it: no
`codex/axis5-prompt.md` exists in the run directory (only `axis{2,3,5,6}-codex.md`, their `.stdout` /
`.stderr` and `rc.txt`), so the pass was reconciled against its own text with no prompt to check it
against. Stated, not worked around.

**Binary.** `/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.19`**, asserted before anything else ran.
Release posture. Repo `HEAD = a8904637` (the driver's; Codex read `645fcb64`, five commits earlier —
`git log --oneline 645fcb64..a8904637` is the three record/release commits plus the addendum, none of
which touches `render.rs`, `orient.rs`, `store.rs` or `doc.rs`, so the two passes read the same code on
every symbol either of them cites).

**The rule applied.** Every Codex claim is entered as a lead and then **driven** — to a repro block, or
to a refutation with the falsifying datum. Nothing here is promoted on a source read. Rigs as the driver
used them (`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`),
two-step eval, `mktemp -d` roots, **no teardown and no `rm -rf` on a variable path anywhere** — the one
fixture edit that removes a file from a pack copy is a `mv` into a `mktemp -d` stash, not a delete. Every
exit code below is `cmd > out 2> err; rc=$?`, never read through a pipe.

---

## Reconciliation ledger

### Codex claims — 3 driven, 3 CONFIRMED, 0 refuted, 0 open

#### L-C1 · `start --explain --format json` is a production JSON stdout arm `ENVELOPE_ARMS` does not carry — **CONFIRMED (repro)**

`lead(codex, "dispatch prints render::explain at cli.rs:1702-1706; its JSON branch serializes
ResolutionTree at render.rs:630-649; the registry's four start arms are three OrientationView variants
plus Composed at render.rs:6254-6312, and there is no explain/ResolutionTree row")`. Driven:

```
rig: fresh                                            cwd = $REPO   (jigc 1.0.0-rc.19)
$ jigc --format json start --workflow single-task --explain > o 2> e; rc=$?
  rc=0   stdout 1218 bytes   stderr 0 bytes
  keys: ["collision_winners","overrides_applied","pack_inputs","schema_version","steps",
         "workflow","workflow_layer"]
$ jigc --format json start --explain > o2 2> e2; rc=$?      # the bare form, same shape
  rc=0   stdout 939 bytes   same key set   "workflow":"router","schema_version":3
registry, read by symbol at HEAD a8904637:
$ awk '/pub const ENVELOPE_ARMS/,/^];/' crates/cli/src/render.rs | grep -c 'EnvelopeArm {'   -> 64
$ …                                     | grep 'ResolutionTree\|collision_winners\|explain'  -> (none)
$ …                                     | grep -A1 'path: &\["start"\]'
  OrientationView::UnsetProject · OrientationView::Clean · OrientationView::ActiveTask · Composed
```

A `--format json` success document on **stdout**, at exit 0, carrying a `schema_version` key, from a leaf
the registry covers with four other arms — and no row declares it. Same cell as the driver's `(5, D1)`
(§2.6), independently re-driven here and **widened**: the defect is not conditional on `--workflow`, the
bare `start --explain` form emits the same undeclared shape.

**Agreement:** driver `(5, D1)` STILL-OPEN · Codex claim 1 STILL-OPEN · re-driven CONFIRMED. Tier 3,
triaged to 1.x by the charter.

#### L-C2 · the pinned `Clean` and `ActiveTask` rows declare `next_steps`, and a reachable serialization omits it — **CONFIRMED (repro)** · *and this is the row the driver did not drive*

`lead(codex, "the rows declare next_steps at render.rs:6267-6300; both fields carry
skip_serializing_if = \"Vec::is_empty\" at result.rs:144-145,174-175; the producer legitimately creates an
empty vector at orient.rs:121-141 and the no-off-catalog-workflow state is explicitly proven at
orient.rs:517-532")`.

The driver recorded this STILL-OPEN **without re-driving the open cell** (§2.5: *"Re-driven here only at
the populated arm"*; §9.8: *"not re-manufactured … Stated rather than presented as a fresh derivation"*).
Under the reconciliation rule that row is not driven — hence the demotion marked in §1. Driven here, at
**both** declared arms:

```
rig: fresh --pack-from-dev        (JIGC_PACK_DIR = a throwaway dev-pack copy; the composed pack-set is
                                   the dev pack alone, so `planning` is already absent)
$ jigc --format json start                       -> rc=0
  keys ["header","next_steps","schema_version","state","workflows"]   next_steps
  [{"id":"ingest-existing","gist":"bring an existing repo's docs under management"}]   <- populated arm

$ STASH=$(mktemp -d "${TMPDIR:-/tmp}/packstash.XXXXXX")
$ mv "$JIGC_PACK_DIR/workflows/ingest-existing.yaml" "$STASH/"     # a MOVE, not a delete
  -> the pack-set now ships NEITHER off-catalog verb

CLEAN arm:
$ jigc --format json start > m1 2> m1e; rc=$?     -> rc=0  stdout 1283 bytes  stderr 0 bytes
  driven keys  ["header","schema_version","state","workflows"]        state = "clean"
  has("next_steps") -> false
  DECLARED     ["header","next_steps","schema_version","state","workflows"]   (render.rs:6269-6277)

ACTIVE arm (same pack-set):
$ jigc start --workflow single-task "probe intent"   -> rc=0, task minted: probe-intent
$ jigc --format json start > m2 2> m2e; rc=$?        -> rc=0
  driven keys  ["header","schema_version","state","tasks","workflows"]   state = "active-task"
  has("next_steps") -> false
  DECLARED     ["header","next_steps","schema_version","state","tasks","workflows"]  (render.rs:6284-6294)
```

Both pinned rows declare a key a reachable, legitimate composition does not serialize — declared ⊋ driven
at two of the four `start` arms. **Wider than either input reported:** Codex named the cell without
driving it and the driver named only the `Clean` arm's declared set; the `ActiveTask` arm omits it too,
and the omission is the *designed* behaviour (`skip_serializing_if` plus the gated producer, with a unit
test at `orient.rs:517-532` asserting the empty vector), so the contract statement — not the producer —
is the thing that is wrong.

**Agreement:** driver `(5, C1)` STILL-OPEN (undriven) · Codex claim 2 STILL-OPEN · **driven here,
CONFIRMED**. Tier 3, triaged to 1.x.

*Source-read observation, marked as such and not driven:* the acceptance fence Codex cites
(`crates/cli/tests/format_json_success_axis.rs`) stays green over this because its recipe reaches each
row at its **populated** arm — the same shape as M45's lens: the fence iterates the registry, not the
registry's own optional-key axis.

#### L-C3 · `doc show` gives `store.unknown-type` an address-shaped target where the code family's stable key is the bare doctype — **CONFIRMED (repro)**

`lead(codex, "resolve_read_schema keys the finding with the entire address_str at store.rs:241-255 while
the shared doctype-scoped producer emits the bare type at store.rs:402-422; ENVELOPE_OWED_CODES makes it
driver-visible at render.rs:5600-5645")`. Driven:

```
rig: committed-singletons                                             cwd = $REPO
$ jigc --format json doc show 'nosuchtype:x' > a 2> ae; rc=$?         -> rc=1
  stdout 0 bytes · stderr 508 bytes (the findings envelope, on its declared stream)
  findings[0].key  {"code":"store.unknown-type","target":"nosuchtype:x"}      <- ADDRESS-shaped
  message          "unknown doctype `nosuchtype` for `nosuchtype:x`"
$ jigc --format json doc schema nosuchtype > b 2> be; rc=$?           -> rc=1
  findings[0].key  {"code":"store.unknown-type","target":"nosuchtype"}        <- bare, as declared
```

One code, two target shapes, so the stable `(code, target)` key does not discriminate the same condition
the same way at two doors. Same cell as the driver's rc.17 `DEFECT A` (§2.7), byte-for-byte reproduced.

**Agreement:** driver `DEFECT A` STILL-OPEN · Codex claim 3 STILL-OPEN · re-driven CONFIRMED. Tier 3,
triaged to 1.x; outside the cwd arc, carried rather than new.

### Codex's prior-row dispositions — entered as leads, every one driven

| Codex disposition | its basis | driven here | verdict |
|---|---|---|---|
| M52 **DEFECT 2** STILL-OPEN — `remove-step` resolves membership and then calls the pack-direct `resolve_fork_bytes` (`config.rs:1810-1838`) | source | **re-driven, and cleaner than §2.2**: on a virgin `fresh` rig `insert-step` lands `probe-step` (rc=0), `start --explain` prints the include list `locate implement record-changelog superseded-context author-commit probe-step finalize`, then `config remove-step 'workflow:single-task#probe-step'` → rc=1 `{"error":"blocking · config.anchor-absent — no step \`probe-step\` body to fork · route: name a step id present in the workflow's resolved include list, then re-run"}` and `config replace-step` → rc=1 `config.step-id-collision`; control `config remove-step 'workflow:single-task#implement'` → rc=0 `{committed:false, op:"config-remove-step", target:"workflow:single-task#implement"}` | **CONFIRMED** — and the driver's §2.2 confound (its control ran first, so `implement` was already gone and a stray `ps` step was present) is **removed**: the row holds on a fixture with no prior mutation |
| M52 **DEFECT 3** STILL-OPEN — the colon-less-address bail is a code-less `anyhow!` (`rename.rs:1191-1207`) | source | `rename vision --to "New Vision"` → rc=1 `{"error":"\`vision\` is not a \`<type>:<slug>\` address — e.g. \`adr:single-node-cache\`\n  route: run \`jigc describe\` for the doctype surface"}`; `doc show nosuchtype` → rc=1 `{"error":"malformed address \`nosuchtype\`: missing ':' between type and slug …"}` — no `blocking · <code>` prefix, no `findings` envelope, no `(code, target)` key, at **either** door | **CONFIRMED**, and **wider than Codex's citation**: the defect is two producers under two wordings, not one bail in `rename.rs` |
| M52 **DEFECT 4** STILL-OPEN — an optional-but-absent leaf converges on the physical `store.no-such-leaf` (`doc.rs:3922-3973`) | source | `doc schema vision --format json` declares `{"id":"grounded-in","type":"ref","to":"research","required":false,"author-required":false,"section":"meta","set-field":"vision:<slug>#meta/grounded-in"}`; `doc show 'vision:vision#meta/grounded-in'` → rc=1, key `{store.no-such-leaf, vision:vision#meta/grounded-in}`, message *"names no leaf `grounded-in` in section `meta`"*, route *"name a field the committed section carries"* — the schema surface advertises the address the read surface calls nonexistent | **CONFIRMED** |
| M52 **DEFECT 1** CLOSED — `reject_unslugable_title` is asked before any write (`start.rs:80-104`) | source | the driver drove it at §2.1 (5 titles × `milestone create`, exit 1 each, `write.unslugable-title`, HEAD `46c3233 → 46c3233`, clean `git status --porcelain`, `.jigc/milestones` absent). Not contradicted by either pass, so not re-driven | **agreed CLOSED** (driver's repro stands) |
| M51 **DEFECT A / B / C / D** all CLOSED | source | spot-checked against the driver's §6.1/§6.2 roll-ups rather than taken on either read: **outside any git repo** — `start`·`validate`·`doc list`·`task list` → rc=1, stdout 0 bytes, stderr root `["error"]`; `setup`·`uninstall` → rc=1, stdout 0 bytes, stderr root `["findings","schema_version"]` (the two declared findings-arm leaves the driver names); **cwd deleted under the process** — `start`·`validate`·`doc list`·`task list` → rc=1, `{"error":"cannot determine the current directory: No such file or directory (os error 2)"}`, 0 NOT-JSON, 0 bare-`Finding` roots | **agreed CLOSED**, 10 spot cells consistent with the driver's 94 |
| the census — *"64 rows over all 47 clap leaves, including its two cross-cutting reject rows"*; *"every CLI JSON stdout serialization found is covered except Claim 1"* | source | the countable half verified by symbol at `HEAD a8904637`: `ENVELOPE_ARMS` = **64** `EnvelopeArm {` rows; `VERB_KINDS` = **47** leaves; the registry's distinct `path:` spellings = **47** real leaves **+ 2 empty-path reject rows**. The negative half (*no other undeclared production arm*) is not drivable as stated — it is a claim over all serialization sites — but the driver's 94 hostile-cwd cells and 180 four-cwd cells returned **0** undeclared shapes, and this pass's `--explain` drive found the one exception both passes name | **counts CONFIRMED; the negative recorded as corroborated-not-proven**, which is what a negative over a code-wide set can be |

### Driver defects — status after reconciliation

The driver reports **0 new defects** and carries six. The Codex pass **contradicts none of them** — it
independently reaches STILL-OPEN on five and is silent on none — so under the rule every one stays a
finding, and each was re-driven here rather than taken on the driver's word.

| driver row | tier | Codex | re-driven here | status |
|---|---|---|---|---|
| `(5, DEFECT 1)` — mint at a fabricated identity | 1 | agrees CLOSED | not re-driven (both passes agree; driver repro §2.1 stands) | **CLOSED** |
| `(5, DEFECT 2)` — `remove-step`/`replace-step` refuse a step that *is* in the resolved include list | 3 | agrees STILL-OPEN | **yes**, on an unconfounded fixture | **STILL-OPEN · CONFIRMED** |
| `(5, DEFECT 3)` — the colon-less-address bail carries no code | 3 | agrees STILL-OPEN | **yes**, both wordings | **STILL-OPEN · CONFIRMED**, wider than Codex cited |
| `(5, DEFECT 4)` — a declared-optional leaf blocks with `store.no-such-leaf` | 3 | agrees STILL-OPEN | **yes** | **STILL-OPEN · CONFIRMED** |
| `(5, C1)` — the orientation rows declare a key a reachable composition omits | 3 | agrees STILL-OPEN | **yes — and it is the row the driver did NOT drive** (demoted in §1, restored here at both arms) | **STILL-OPEN · CONFIRMED on a new repro** |
| `(5, D1)` — `start --explain` emits an undeclared production JSON arm | 3 | agrees STILL-OPEN | **yes**, plus the bare `--explain` form | **STILL-OPEN · CONFIRMED**, wider |
| rc.17 `DEFECT A` — `store.unknown-type`'s address-shaped target at `doc show` | 3 | agrees STILL-OPEN | **yes** | **STILL-OPEN · CONFIRMED** |

**Net: 0 new defects, 6 carried, all tier 3, all already triaged to 1.x by the charter. Tier-1 rows on
this axis: 0** — and that zero is now carried by two passes that disagree about nothing.

### Demotions applied to the driver table

1. **`(5, C1)`'s row in §1** — marked with a verdict whose datum was not produced this run. §2.5 and §9.8
   state it plainly (*"Re-driven here only at the populated arm"*; *"not re-manufactured"*), so the
   honesty is the driver's, not the reconciliation's; the demotion is only that a row carrying no repro
   block for its own cell is not a driven row. **Restored as driven by L-C2**, at both arms.
2. **§10's `cwd-arc review findings re-verified closed` count** — `11 / 11` + `7 / 7` counts, as
   *re-verified closed*, findings the same section's prose says were **not separately driven**
   (review-2 LOW 6), verified only **transitively** (review-1 LOW 11), or have **no release-posture
   surface** (review-1 LOW 6/7/8/9, review-2 LOW 3/4/5). Driven-with-a-repro: **6 / 11** and **3 / 7**.
   The denominators check out — `completions/artifacts/M53/audit/cwd-fix-code-review.md` carries HIGH 1–2,
   MEDIUM 3–4, LOW 5–11 (11) and `-2.md` carries MEDIUM 1–2, LOW 3–7 (7). Nothing about the *findings*
   changes: the six and the three are genuinely driven and genuinely closed; the count was the
   overstatement, and a not-observable item is not a closed one.

No other row was demoted. §3's rows 33 (`doc show | CompoundFieldSlice`), 40 (`task bind | Bound`) and 55
(`milestone add-from-spec | RecordOnlyAck` at success) are already labelled **NOT DRIVEN** with reasons at
§9, which is the correct disposition, not a demotion. §3's other 61 rows each carry their driven key set
inline — the datum is in the row — and §4–§7's cells each carry a repro block.

### Open leads

**None.** All three Codex claims were driven to a repro block on the installed release binary; the census
counts were verified by symbol; and the only half of the Codex pass that cannot be driven as stated is its
*negative* (no other undeclared arm exists anywhere in the code), which is recorded above as corroborated
by 274 driven hostile-cwd and four-cwd cells rather than promoted to a finding or dropped.

---

## Doors covered

Every clap leaf that is the door of ≥1 **driven** row in this reconciled file, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs:1835`, **47** leaves read by symbol at `HEAD = a8904637`). The two demotions
remove no door: `(5, C1)`'s door is `start`, which the reconciliation itself drove, and §10's count is a
tally, not a row.

**47 / 47 · uncovered: none.**

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

**How the coverage is carried after reconciliation.** All 47 are the door of ≥1 row in each of the two
uniform 47-leaf hostile-cwd sweeps (§6.1, §6.2), 10 cells of which were independently spot-driven here.
45 are additionally the door of ≥1 four-cwd divergence row (§7); 44 additionally the door of a
condition-specific row with a captured key set (§3). The reconciliation's own drives touch **six** leaves
— `start` (L-C1, L-C2, the spot checks), `doc show` (L-C3, DEFECT 3, DEFECT 4), `doc schema` (L-C3,
DEFECT 4), `rename` (DEFECT 3), `config insert-step` / `config remove-step` / `config replace-step`
(DEFECT 2), `validate` · `doc list` · `task list` · `setup` · `uninstall` (the hostile-cwd spot checks) —
and confirm rather than extend the driver's coverage claim. The three leaves whose axis-5 coverage rests
on the sweeps plus a reject-arm row remain **`task bind`**, **`milestone add-from-spec`** and, for the
`CompoundFieldSlice` arm alone, **`doc show`** (otherwise driven at seven arms), each named with its
reason at §9.

---

## Notes

- **The two passes disagree about nothing on this axis.** Codex reached STILL-OPEN on all six carried
  rows from the source alone and CLOSED on the five the driver drove closed; every one of its three
  headline claims is a row the driver already carries. That is a **corroboration**, not new coverage —
  the value it added was on `(5, C1)`, the one row the driver had not driven, which is exactly where the
  reconciliation rule earned its keep.
- **The one substantive gain is L-C2**, driven for the first time in this arc: both pinned orientation
  rows declare `next_steps`, and a legitimate pack-set omits it at **both** arms. The producer is
  deliberate and unit-tested; it is the **declaration** that overstates. Fixing it is a registry edit
  (an optional-key notion, or two more arms), not a behaviour change — but it is a **pinned-contract**
  edit, so the 1.x window is the right home and the charter's triage stands.
- **Two claims came back wider than either input stated**, which is this axis's recurring shape: `(5,
  D1)` is not conditional on `--workflow` (the bare `start --explain` emits the same undeclared shape),
  and `(5, DEFECT 3)` is two producers under two wordings, not the single `rename.rs` bail Codex cited.
- **`(5, DEFECT 2)`'s repro is now confound-free.** The driver's own block noted its control had run
  first, leaving `implement` removed and a stray `ps` step in the include list; re-driven on a virgin
  `fresh` rig the row holds identically, so the defect does not depend on a mutated workflow.
- **Posture bound, inherited and restated.** This binary carries no `#[cfg(debug_assertions)]` route,
  span or quoting fence, so every route claim is a claim about **emitted bytes**. Nothing in this
  reconciliation's own drives required running an emitted span; the driver's nine verbatim executions are
  its evidence, unre-run here.
- **Codex read `645fcb64`, the driver `a8904637`.** The five-commit gap is release-stamp and record
  commits; `git log --oneline 645fcb64..a8904637` touches none of `render.rs`, `orient.rs`,
  `engine/src/store.rs` or `doc.rs`, so no citation in either pass is stale. Recorded rather than assumed.
- **No `axis5-prompt.md` exists** beside the Codex pass, so what the source reviewer was asked could not
  be checked against what it answered. The pass states its own bounds (source-only, no driving, no
  independent execution of the 64 recipe witnesses), and those bounds are what this ledger held it to.
- **Nothing was fixed, committed or edited in the repository.** The only writes were to the scratchpad and
  to throwaway `mktemp -d` rigs; the one pack-copy edit was a `mv` into a `mktemp -d` stash.
