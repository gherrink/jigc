# Axis 5 source pass — M53 partial re-run

Read-only review of fixed source at `84db75517669bbfe3f499e5268fd2e0674e04d88`; no binary was driven and no files/directories were written.

## Claims

1. **`jigc start --explain --format json` remains an unregistered production stdout envelope.**

   - Evidence: dispatch prints `render::explain` at `crates/cli/src/cli.rs:1703`; its JSON arm serializes `ResolutionTree` at `crates/cli/src/render.rs:630-649`. `ENVELOPE_ARMS` has only four `start` arms—three `OrientationView` variants and `Composed`—at `crates/cli/src/render.rs:6066-6124`; no `ResolutionTree`/explain row exists.
   - Reproduction: `jigc --format json start --workflow single-task --explain`; expect exit 0 and a `ResolutionTree` object on stdout, but no registry row declares its keys.
   - Confidence: **High**. This is M52 `(5,D1)`, still open.

2. **The two pinned orientation rows still declare `next_steps` unconditionally although reachable serialization omits it.**

   - Evidence: both registry rows include `next_steps` at `crates/cli/src/render.rs:6078-6111`; both enum fields use `skip_serializing_if = "Vec::is_empty"` at `crates/engine/src/result.rs:127-145,158-175`. The producer legitimately constructs an empty vector when neither off-catalog workflow exists at `crates/cli/src/orient.rs:116-125`, and a test proves that state reachable at `crates/cli/src/orient.rs:515-532`.
   - Reproduction: load a valid composed pack containing neither `planning` nor `ingest-existing`; run bare `jigc --format json start`, first clean and then with an active task. Expect `next_steps` absent although the applicable registry row declares it.
   - Confidence: **High**. This is M52 `(5,C1)`, still open.

Neither claim is tier 1: both are pinned-contract completeness defects, not exit-0 loss or repository harm.

## M52 §A Axis 5 dispositions

1. **DEFECT 1 — CLOSED.** All caller-prose mint seams now reject an empty slug before writing: `start` at `crates/cli/src/start.rs:80-104`, milestone creation before schema/record/gitignore work at `crates/cli/src/milestone.rs:642-662`, and both `add-task` and `add-from-spec` through the shared engine seam at `crates/engine/src/milestone.rs:362-386`. They use `write.unslugable-title`. This closes the sole Axis 5 tier-1 row.

2. **DEFECT 2 — NOT CLOSED.** `remove-step` verifies resolved membership and then calls the pack-only byte reader at `crates/cli/src/config.rs:1810-1838`; that reader still says “body to fork” and routes back to an already-satisfied include-list condition at `crates/cli/src/config.rs:1985-1998`. `replace-step` uses the same basis/read model at `crates/cli/src/config.rs:1727-1775`.

3. **DEFECT 3 — NOT CLOSED.** `rename` still excludes malformed addresses from `RefusalKind` at `crates/cli/src/rename.rs:83-100`; `parse_addr` returns a bare `anyhow!` for a bare slug at `crates/cli/src/rename.rs:1193-1207`. It has route prose but no finding code/key.

4. **DEFECT 4 — NOT CLOSED.** `doc show` still sends its addressed read through `show_json`/the store at `crates/cli/src/doc.rs:3922-3973`, while the store defines `store.no-such-leaf` as a leaf the physical committed node does not carry at `crates/engine/src/store.rs:48-50`; no source change distinguishes a schema-declared optional-but-absent leaf.

5. **C1 — NOT CLOSED.** See Claim 2.

6. **D1 — NOT CLOSED.** See Claim 1.

## M51 §A Axis 5 dispositions

1. **DEFECT D — CLOSED.** JSON operational failures converge on `{"error": …}` at `crates/cli/src/render.rs:5633-5646`; the invocation-log/error funnel uses that renderer at `crates/cli/src/invocation_log.rs:389-396`.

2. **DEFECT A — CLOSED.** `BlockedFinding` carries whether the producer owes the findings envelope at `crates/cli/src/render.rs:5603-5617`, and `carrier` additionally consults `ENVELOPE_OWED_CODES` at `crates/cli/src/render.rs:5451,5505`; setup/uninstall therefore no longer serialize a bare `Finding`.

3. **DEFECT B — CLOSED.** `doc show` declares eight projections, including array, scalar, data-keyed, and compound shapes, at `crates/cli/src/render.rs:6369-6455`.

4. **DEFECT C — CLOSED by declaration.** The registry’s `ArmOutcome::Success` documentation no longer promises invariant exit 0; the three conditional non-zero success-document cells remain registry-covered rather than contradicting the envelope declaration.

## Serialization census and M53 consistency

Every CLI JSON stdout producer I found routes through `render::json` or a format-specific renderer. The registered families cover orientation, composed output, setup/uninstall, upgrade/validate, ingest/migrations, unmanage/rename/relocate, describe, all nine `DocAck` variants, eight `doc show` projections, schema/list, both `TaskAck` variants, task list/diff/finalize, all six `ConfigAck` variants, and milestone acknowledgements. The sole uncovered stdout producer is `render::explain` in Claim 1. Adapter configuration serialization and invocation-log JSON are file/log writers, not stdout contracts.

The M53 additions are internally consistent on this read:

- `FINALIZE_MESSAGE_FILE` is shared by both writer registries and the writer: `crates/engine/src/state.rs:116,166,224`; `crates/cli/src/task.rs:3959-3965`.
- `foreign_area_paths` descends the milestone `merged/docs` tree and treats only regular staged-doc names as owned: `crates/engine/src/state.rs:422-499`; its unwind sibling shares that membership rule.
- Residual work units consistently require the base pin in task enumeration (`crates/engine/src/state.rs:1292-1317`), task resolution (`crates/cli/src/task.rs:1495-1506`), milestone resolution (`crates/engine/src/milestone.rs:908-924`), ownership (`:1006-1020`), and rename’s first-live milestone (`crates/cli/src/rename.rs:1088-1110`).
- `finalize.foreign-bytes` is produced at the landed teardown seam (`crates/cli/src/task.rs:6296-6380`). Task finalize can carry it in its existing `findings` key; milestone finalize’s stderr-only narration is explicitly bounded because its pinned stdout remains `{"committed":…}` at `crates/cli/src/milestone.rs:5889-5907`.
- I found no new M53 code path that turns a committing/destroying/moving door into exit-0 loss or repository harm.

Zero schema-hash movement appears respected: the reviewed M53 changes are source/tests/docs-side, and the recorded schema/manifests boundary is unchanged. Git’s read emitted sandbox-related cache warnings, so this is a source/path comparison, not an independently executed hash calculation.

**Exit-rule result for Axis 5: no TIER-1 row found.** Bounds: source inspection only; no binary driving, filesystem-fault injection, clap parse-error census, or runtime verification of the 60 registered witnesses.