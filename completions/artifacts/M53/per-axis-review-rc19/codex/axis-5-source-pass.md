<!-- M53 THIRD PARTIAL per-axis review — axis 5 — the unseeded Codex SOURCE pass, verbatim. Source-only: it drove nothing and wrote nothing. Captured 2026-09-23 against the rc.19 tree. -->

Source-only review at `645fcb64685073d1a90d9e65e4ac067849a53aab`; no binary was driven and nothing was written.

## Claims

1. **`jigc start --explain --format json` remains a production stdout envelope absent from `ENVELOPE_ARMS`.**

   - Evidence: dispatch prints `render::explain` at `crates/cli/src/cli.rs:1702-1706`; its JSON branch serializes `ResolutionTree` at `crates/cli/src/render.rs:630-649`. The registry’s four `start` arms are three `OrientationView` variants plus `Composed`, at `crates/cli/src/render.rs:6254-6312`; there is no explain/`ResolutionTree` row.
   - Proposed reproduction: `jigc --format json start --workflow single-task --explain`. Expected wrong behaviour: exit 0 with a `ResolutionTree` object on stdout whose keys have no registry declaration.
   - Confidence: **High**. M52 `(5,D1)`: **STILL-OPEN (datum)**, expected tier-3/1.x residue.

2. **The pinned clean and active orientation rows still require `next_steps`, although reachable serialization omits that key.**

   - Evidence: the rows declare `next_steps` at `crates/cli/src/render.rs:6267-6300`; both fields use `skip_serializing_if = "Vec::is_empty"` at `crates/engine/src/result.rs:144-145,174-175`. The producer legitimately creates an empty vector at `crates/cli/src/orient.rs:121-141`, and the no-off-catalog-workflow state is explicitly proven at `crates/cli/src/orient.rs:517-532`.
   - Proposed reproduction: compose a valid project pack lacking `planning` and `ingest-existing`, then run `jigc --format json start`, both clean and with an active task. Expected wrong behaviour: `next_steps` is absent while the applicable pinned row declares it.
   - Confidence: **High**. M52 `(5,C1)`: **STILL-OPEN (datum)**, expected tier-3/1.x residue.

3. **`doc show` still gives `store.unknown-type` an address-shaped target, contradicting the code family’s bare-doctype stable-key form.**

   - Evidence: `doc show` reaches `show_json`/the store at `crates/cli/src/doc.rs:3953-3973`; `resolve_read_schema` keys the finding with the entire `address_str` at `crates/engine/src/store.rs:241-255`. The shared doctype-scoped producer instead documents and emits the bare type at `crates/engine/src/store.rs:402-422`. `ENVELOPE_OWED_CODES` makes this structured and driver-visible at `crates/cli/src/render.rs:5600-5645`.
   - Proposed reproduction: `jigc --format json doc show 'nosuchtype:x'`. Expected wrong behaviour: exit 1 with `findings[0].key.target == "nosuchtype:x"`; sibling producers such as `doc schema nosuchtype` emit `"nosuchtype"`.
   - Confidence: **High**. rc.19 `(5, DEFECT A)`: **STILL-OPEN (datum)**. This is outside the cwd arc.

## Prior confirmed-row dispositions

M51 Axis 5:

- **DEFECT D — CLOSED (argv).** Pre-dispatch failures use the format-aware operational funnel: `cwd_or_refusal` dispatch is visible at `crates/cli/src/cli.rs:1695-1701`, and JSON operational errors serialize as `{"error":…}` at `crates/cli/src/render.rs:5828-5834`. `PRE_DISPATCH_FAULTS` remains phase-typed and crossed with the registry in `crates/cli/tests/flow53_acceptance.rs:2448-2627`.
- **DEFECT A — CLOSED (argv).** `carrier` selects the findings envelope from either the constructor or `ENVELOPE_OWED_CODES`: `crates/cli/src/render.rs:5639-5698`. Setup/uninstall no longer have a bare-`Finding` escape.
- **DEFECT B — CLOSED (argv).** Eight `doc show` projections explicitly cover object, scalar, data-keyed and array forms: `crates/cli/src/render.rs:6558-6643`.
- **DEFECT C — CLOSED by declaration (datum).** Registry outcomes distinguish success, adjudicated non-zero, and reject streams; the real-binary proof enforces each declared outcome at `crates/cli/tests/format_json_success_axis.rs:1450-1475`.

M52 Axis 5:

- **DEFECT 1 — CLOSED (datum):** caller-prose task minting asks `reject_unslugable_title` before writing at `crates/cli/src/start.rs:80-104`.
- **DEFECT 2 — STILL-OPEN:** `remove-step` still resolves membership and then calls the pack-direct `resolve_fork_bytes` at `crates/cli/src/config.rs:1810-1838`, retaining the already-satisfied “body to fork” dead end.
- **DEFECT 3 — STILL-OPEN:** rename’s `parse_addr` still returns a code-less `anyhow!` for malformed/bare addresses at `crates/cli/src/rename.rs:1191-1207`.
- **DEFECT 4 — STILL-OPEN:** optional-but-absent leaves still converge on the store’s physical `store.no-such-leaf`; `doc show` has no distinct schema-declared-optional state (`crates/cli/src/doc.rs:3922-3973`).
- **C1 and D1 — STILL-OPEN:** Claims 2 and 1 above.

The tier-2/3 rows triaged to 1.x are therefore expected **STILL-OPEN**, not regressions attributed to the cwd arc.

## Consistent reading and serialization census

The current registry contains **64 rows over all 47 clap leaves**, including its two cross-cutting reject rows. Its proofs enforce clap-leaf coverage, enum-arm equality, declared/driven row equality, exact keys, streams and exits at `crates/cli/tests/format_json_success_axis.rs:1239-1475`.

Every CLI JSON stdout serialization found is covered except Claim 1:

- `render.rs`: orientation/composition; setup/uninstall; validation/finalize; ingest, migration, rename/relocate/unmanage; all nine `DocAck`, two `TaskAck`, six `ConfigAck` variants; document show/schema/list; task list/diff; milestone operations; describe; and both reject funnels all have registry rows.
- `doc.rs:5413`: constructs nested document data consumed by the registered `doc show` projections.
- `setup.rs:204`: diagnostic formatting, not a stdout document.
- `adapter.rs:928,1078,2285,2344,2356,2438,2444`: adapter/settings files, not command stdout.
- `invocation_log.rs:523`: JSONL log writer, not stdout.

The four deleted keys remain absent: no `installed`, `uninstalled`, migration `review`, or milestone-list `hook_output` row. The only deliberately unpinned success documents remain `describe` and milestone prose acknowledgements, each with a reason.

The cwd arc’s root discovery, `jigc_home`, absolute spawn, typed path bases and aimed `git` routes introduce no new JSON envelope or bypass. No schema or manifest file changed across the inspected arc-to-HEAD diff: **zero schema-hash movement is respected**.

Bounds: source inspection only; no binary driving, runtime fault injection, or independent execution of the 64 recipe witnesses. Git emitted sandbox cache warnings, but the read-only source/diff queries completed.