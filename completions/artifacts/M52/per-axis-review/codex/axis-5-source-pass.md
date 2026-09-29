<!-- M52 per-axis review (re-run) — axis 5 · transaction / rollback — the CODEX SOURCE PASS, verbatim. Read against the repository at commit e519e4eb, 2026-09-21. It reads; it drives nothing. Every claim below is a LEAD until the reconciler drove it — see axis-5.md for the verdict on each. -->

# M52 Axis 5 source pass

Read-only review of HEAD `e519e4ebf952bbf71b8591294101bd63be779998` (`1.0.0-rc.16`). No files or directories were created, and no binary was driven.

## Claims

1. **`start` has an unregistered JSON key-set arm when the composed pack omits both off-catalog workflows: `next_steps` disappears, although both orientation rows declare it present.**

   - Evidence:
     - `OrientationView::{Clean,ActiveTask}.next_steps` uses `skip_serializing_if = "Vec::is_empty"`: `crates/engine/src/result.rs:127-145`, `158-175`.
     - The producer deliberately returns an empty vector when neither `planning` nor `ingest-existing` exists: `crates/cli/src/orient.rs:116-125`; this state is explicitly proven reachable with a pack omitting both: `crates/cli/src/orient.rs:515-531`.
     - Nevertheless, both registry rows require `next_steps`: `crates/cli/src/render.rs:6044-6057`, `6060-6075`.
     - The only recipes drive the stock fresh corpus, once clean and once with an active task: `crates/cli/tests/format_json_success_axis.rs:457-470`. Proof 4 consequently compares only those witnesses: `1454-1473`.
     - This falls exactly inside the registry’s admitted bound that a state not built by the census can expose an absent arm: `crates/cli/src/render.rs:6019-6025`.
   - Proposed reproduction:
     ```sh
     # Configure/setup a project with a valid custom pack containing selectable
     # workflows but neither `planning` nor `ingest-existing`.
     jigc --format json start
     jigc start --workflow <selectable-workflow> "live task"
     jigc --format json start
     ```
     Expected wrong behavior: both commands succeed, but the clean document has keys `header,schema_version,state,workflows` and the active document has `header,schema_version,state,tasks,workflows`; neither matches its sole declared registry row because `next_steps` is absent.
   - Confidence: **High**.

## M51 confirmed-row dispositions

1. **DEFECT D — CLOSED.** All dispatch paths now obtain cwd through `cwd_or_refusal`, which routes failure through the format-aware operational funnel: `crates/cli/src/cli.rs:778-806`. `PRE_DISPATCH_FAULTS` is driven across every leaf by the cited M52 test; the production funnel emits `{"error":…}` for non-finding failures: `crates/cli/src/invocation_log.rs:381-407`, `crates/cli/src/render.rs:5599-5610`.

2. **DEFECT A — CLOSED.** `setup` and `uninstall` wrap their `Finding` with `envelope_finding_error` and enter the shared funnel: `crates/cli/src/cli.rs:850-869`, `899-905`. The carrier selects the findings envelope: `crates/cli/src/render.rs:5497-5514`, whose declared root is `findings,schema_version`: `6768-6799`.

3. **DEFECT B — CLOSED.** `doc show` now declares eight projections, including repeatable arrays, items, list-field arrays and compound leaves: `crates/cli/src/render.rs:6333-6418`. `DOC_SHOW_DISPATCH` explicitly admits object, array and scalar roots: `5979-5986`.

4. **DEFECT C — CLOSED as a contradictory claim, not by changing exit semantics.** The old universal “Success means exit 0” prose is retired by a mechanically evaluated shape claim, while `STORE_EXIT_FLIPS` remains the exit authority: `crates/cli/tests/format_json_success_axis.rs:1749-1753`, `1808-1855`. Its seventh member is `home-vacated`: `crates/cli/src/render.rs:1093-1108`.

5. **Four pre-pin deletes — CLOSED/HELD.** Setup has no `installed`, uninstall no `uninstalled`: `crates/cli/src/render.rs:6098-6122`; migration review declares and emits only `retires,rewrites,source,task`: `6533-6544`, `5075-5089`; milestone `list-tasks` declares only `text`: `6683-6695`.

6. **Explicitly unpinned rows — CLOSED/HELD.** `describe` carries a stated reason and pins only `schema_version`: `6207-6219`. The five record-writing milestone acknowledgements and `list-tasks` each state their reasons: `6618-6695`. I found no reasonless `Unpinned` row.

## Serialization inventory and consistent reading

- `ENVELOPE_ARMS` currently has **64 rows covering all 47 clap leaves**, plus the two cross-cutting rejects. The clap, enum, recipe and exact-key fences are at `crates/cli/tests/format_json_success_axis.rs:1244-1435`, `1437-1525`.
- Registered stdout families:
  - orientation/composition, setup/uninstall, upgrade/validation, ingest, migrate/corpus, unmanage, rename/relocate, describe: `crates/cli/src/render.rs:6030-6235`;
  - all nine `DocAck` writes plus eight `doc show` projections, schema/list: `6236-6467`;
  - task list/validate/diff/finalize/bind/discard: `6468-6545`;
  - six `ConfigAck` writes and two reads: `6546-6617`;
  - milestone acknowledgements, execute/join/finalize: `6618-6744`;
  - both reject serializers: `6745-6799`.
- Direct stdout serialization outside the central renderer is limited to registered document surfaces: `doc schema` at `crates/cli/src/doc.rs:4189-4192`, `doc list` at `4484-4487`, and `doc show` values built at `5401-5415`.
- Serialization hits that do **not** reach command stdout: generated guide YAML scalar quoting (`crates/cli/src/setup.rs:171-180`), adapter settings (`crates/cli/src/adapter.rs:932-937`, `1076-1085`), and invocation JSONL (`crates/cli/src/invocation_log.rs:510-529`).
- `ENVELOPE_OWED_CODES` is enforced centrally at `carrier`, so listed `store.*`/fixed-identity findings cannot be accidentally flattened by an individual door: `crates/cli/src/render.rs:5424-5430`, `5470-5490`. I found no bypass after that seam.
- The other M52 registries named in the brief concern rollback, task-area destruction, git posture, relocation, or destroying-door disposition; they do not define an additional stdout JSON arm for this axis.

## Schema-hash boundary and bounds

No zero-schema-hash violation found. All three tracked `schema-manifest.yaml` files are byte-unchanged from M51 commit `577a0099` through HEAD: `packs/methodology/config/schema-manifest.yaml`, `crates/cli/pack/config/schema-manifest.yaml`, and `crates/cli/tests/fixtures/prior-schema-prd/config/schema-manifest.yaml`.

Bounds: source inspection only; no binary execution, compilation, or filesystem mutation. The sole lead depends on a valid composed pack lacking both off-catalog workflows; the source contains an explicit unit witness for that state, but only the separate driver can confirm its installed-binary reproduction.