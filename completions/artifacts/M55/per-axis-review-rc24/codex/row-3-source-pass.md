<!-- The unseeded Codex source pass for ROW 3 (store exit codes / reconciliation), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

Source-only review of `jigc-v1.0.0-rc.24`; no binary, tests, or reproductions were run, and no files were written.

## Claims

1. **F21 remains open: store-scope `jigc validate` validates the raw workflow definition and therefore misses project `structural-op` deltas.**

   - Evidence: the store sweep calls `enumerate_store_workflows`, which reads `defs.read_workflow` and directly stores those bytes without calling `apply_project_structure` ([start.rs:4698](crates/cli/src/start.rs:4698), [start.rs:4710](crates/cli/src/start.rs:4710)). By contrast, compose-time consumers apply scoped deltas to `def.includes` through the shared seam ([start.rs:2025](crates/cli/src/start.rs:2025), [start.rs:2030](crates/cli/src/start.rs:2030), [start.rs:3331](crates/cli/src/start.rs:3331)). The raw bytes reach every `workflow_refs_store` check through the store-family loop ([validate.rs:683](crates/engine/src/validate.rs:683), [compose.rs:3311](crates/engine/src/compose.rs:3311)).
   - Bound: the shared raw read affects all store-scope readers inside `workflow_refs_store`: workflow loading/front matter, include resolution and cycle detection, fan-out/join pairing, marker-shadow checks, command-ref membership, and schema-ref membership ([compose.rs:3311](crates/engine/src/compose.rs:3311), [compose.rs:3318](crates/engine/src/compose.rs:3318), [compose.rs:3325](crates/engine/src/compose.rs:3325), [compose.rs:3331](crates/engine/src/compose.rs:3331)). It does not affect the other six `STORE_FAMILIES`, which do not consume workflow definitions.
   - Reproduction: add a project `config insert-step`/`replace-step` producing a broken include, cycle, command ref, or schema ref; `jigc validate --format json` incorrectly exits 0 without that finding, while a compose door such as `jigc task amend` rejects it. Put the same break in a whole-file workflow shadow and validate reports it.
   - Confidence: **High; declared report-only gap, not tier 1.**

2. **`(7, A7-F1)` is not source-closed: the selected exit trailer can still claim “this commit introduced” a `git mv` when the matching blocking rename came from a deletion.**

   - Evidence: every blocking `reconciliation.rename` matches `oob-rename` regardless of strong/weak producer ([render.rs:1081](crates/cli/src/render.rs:1081)); the weak missing-path producer is explicitly a deletion finding ([file_state.rs:1562](crates/engine/src/file_state.rs:1562)); its trailer still asserts a commit and `git mv` ([render.rs:1771](crates/cli/src/render.rs:1771)).
   - Reproduction: manage `CHANGELOG.md`, remove it unstaged or with `git rm`, then run `jigc validate`; expect exit 1 with a trailer falsely describing a commit-introduced `git mv`.
   - Confidence: **High; carried defect remains open.**

3. **`(7, A7-F2)` is not source-closed: `ingest` still labels a stamped orphan that store validation blocks as safely unmanaged with “no action needed.”**

   - Evidence: the orphan producer is blocking and explicitly sends the reader first to `jigc ingest` ([orphan.rs:666](crates/cli/src/orphan.rs:666), [orphan.rs:677](crates/cli/src/orphan.rs:677)); ingest’s unmanaged legend still says the plain-file state is legitimate and needs no action ([render.rs:4633](crates/cli/src/render.rs:4633)).
   - Reproduction: retain a stamped instance after removing its doctype; `jigc validate` exits 1 with `schema-conformance.orphaned-instance`, while `jigc ingest` exits 0 and says no action is needed.
   - Confidence: **High; carried defect remains open.**

4. **`(7, C-2)` remains STILL-OPEN as expected: a departed doctype’s stamped root-placement instance remains outside orphan territory.**

   - Evidence: the source explicitly declares this residual ([orphan.rs:485](crates/cli/src/orphan.rs:485)); `Territory` rejects empty/root directories and a root placement contributes no parent ([orphan.rs:518](crates/cli/src/orphan.rs:518), [orphan.rs:535](crates/cli/src/orphan.rs:535)); orphan enumeration requires territory membership ([orphan.rs:616](crates/cli/src/orphan.rs:616)).
   - Reproduction: adopt a stamped root-placement `VISION.md`, remove its defining pack, then validate; expect no orphan finding and exit 0 absent another flip.
   - Confidence: **High; declared residual on the written namespaced-stamp trigger.**

## Consistent completeness findings

- `STORE_EXIT_FLIPS` still has exactly seven members. `probe-unreliable` matches every `pack-probe-integrity` meta-finding by probe id, while `oob-rename` now additionally requires blocking severity ([render.rs:1054](crates/cli/src/render.rs:1054), [render.rs:1062](crates/cli/src/render.rs:1062), [render.rs:1083](crates/cli/src/render.rs:1083)). The store exit, JSON report-only value, and trailer share this registry ([cli.rs:1317](crates/cli/src/cli.rs:1317), [render.rs:1249](crates/cli/src/render.rs:1249)). I found no new blocking-store code omitted from the intended exception set and no advisory incorrectly matched beyond the deliberate advisory `foreign-squatter`.
- L1 is correctly gated: task absorption requires base-pin byte equality and successful conformance; failure preserves the caller’s conflict block ([file_state.rs:543](crates/engine/src/file_state.rs:543)). Store scope likewise checks HEAD equality and conformance before emitting advisory `file-state.hash-matches` ([file_state.rs:970](crates/engine/src/file_state.rs:970)). Git/blob failures return `None`, the safer blocking result ([task.rs:7787](crates/cli/src/task.rs:7787)).
- L2 agrees at both scopes: history-present blocks; history-less plus another branch routes at switching back; no carrying branch routes at `unmanage` ([file_state.rs:1514](crates/engine/src/file_state.rs:1514), [file_state.rs:1521](crates/engine/src/file_state.rs:1521)). Only local and remote-tracking branch tips count; tags deliberately do not ([task.rs:7705](crates/cli/src/task.rs:7705)). History failures answer present and branch-read failures answer carried, both conservative ([cli.rs:1473](crates/cli/src/cli.rs:1473), [cli.rs:1479](crates/cli/src/cli.rs:1479)).
- `unmanage` drops baseline and edges only on a real managed entry, persists neither on a no-op, and detects an absent file without following symlinks ([unmanage.rs:82](crates/cli/src/unmanage.rs:82), [unmanage.rs:95](crates/cli/src/unmanage.rs:95)); its text no longer claims absent bytes were left on disk ([render.rs:4655](crates/cli/src/render.rs:4655)).
- I found no schema boundary violation: M55’s methodology manifest contains exactly the added `inconsistency` and `jigc-feedback` v1 rows ([schema-manifest.yaml:125](crates/cli/packs/methodology/config/schema-manifest.yaml:125)); the rc.23→rc.24 source range changes no manifest or pinned JSON key. The trailer uses adapter configuration rather than a frozen schema.

**Result:** four grounded open leads—three carried/declared and F21. No new tier-1 lead found.