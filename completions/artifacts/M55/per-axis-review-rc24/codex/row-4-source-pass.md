<!-- The unseeded Codex source pass for ROW 4 (pack-load / manifest freeze · migration), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

## Claims

1. **A filesystem pack can lose its freeze manifest between owner enumeration and the second read; `assert_schema_freeze` silently skips that pack, so a committing door can proceed without validating its frozen schemas.**

   - Evidence: `origin_packs` identifies manifest owners by successful reads ([pack.rs:303](crates/cli/src/pack.rs:303), [pack.rs:2507](crates/cli/src/pack.rs:2507)), but a failed second manifest read takes `continue` ([pack.rs:304](crates/cli/src/pack.rs:304)). Consequently that owner’s schemas never reach `manifest::check` ([pack.rs:312](crates/cli/src/pack.rs:312), [pack.rs:324](crates/cli/src/pack.rs:324)). This is the source-level bypass behind the previously un-driven `pack-resource-missing` lead. By contrast, a schema resource disappearing during the same window propagates an error at [pack.rs:315](crates/cli/src/pack.rs:315)-[320](crates/cli/src/pack.rs:320).
   - Proposed reproduction: list a filesystem pack whose manifest freezes a deliberately drifted schema; race repeated `jigc migrate-corpus` invocations by atomically removing/restoring `config/schema-manifest.yaml` after owner discovery but before its second read. Expected wrong behaviour: an invocation exits 0 and may commit corpus changes instead of failing `pack-load freeze check failed`. A non-mutating control is `jigc validate`, expected wrongly to reach validation rather than fail pack-load.
   - Confidence: **High on the bypass, medium on deterministic reproduction timing.** It is potentially tier 1 when reached through `migrate-corpus`, but source alone does not prove the race can be won reliably.

## Baseline and open-lead dispositions

- **`(7, A7-F3)` — CLOSED.** `doc show` intercepts `store.not-found` and asks `relocated_route` ([doc.rs:4318](crates/cli/src/doc.rs:4318)-[4329](crates/cli/src/doc.rs:4329)). Recorded prior homes are matched by full identity and routed to `jigc migrate-corpus` ([doc.rs:4418](crates/cli/src/doc.rs:4418)-[4439](crates/cli/src/doc.rs:4439)); root-knob strands route through `validate`/move/re-point/unmanage ([doc.rs:4442](crates/cli/src/doc.rs:4442)-[4455](crates/cli/src/doc.rs:4455)).

- **Methodology pack’s own manifest — CLOSED in source.** It contains exactly 13 entries, including `inconsistency` and `jigc-feedback` at v1 ([schema-manifest.yaml:106](crates/cli/packs/methodology/config/schema-manifest.yaml:106)-[145](crates/cli/packs/methodology/config/schema-manifest.yaml:145)). Strict set equality and hashes are checked at load ([manifest.rs:287](crates/engine/src/manifest.rs:287)-[345](crates/engine/src/manifest.rs:345)); the shipped-set fence names all 13 ([pack.rs:4019](crates/cli/src/pack.rs:4019)-[4054](crates/cli/src/pack.rs:4054)). Neither new doctype has a snapshot, correctly, because both begin at v1.

- **Un-driven `SchemaChangeKind × LOCI` cells — source-closed, behaviour still not re-driven.** `ALL` has 18 members and `LOCI` derives to 3 ([schema_diff.rs:137](crates/engine/src/schema_diff.rs:137)-[144](crates/engine/src/schema_diff.rs:144), [schema_diff.rs:757](crates/engine/src/schema_diff.rs:757)). The exhaustive disposition table admits no `Unbuilt` cell at HEAD ([schema_diff.rs:146](crates/engine/src/schema_diff.rs:146)-[185](crates/engine/src/schema_diff.rs:185)). `AddedItemSlot` covers the new optional `inconsistency.sides[].says` shape. This closes CX-9’s completeness question, not its un-driven runtime matrix.

- **CX-8 third origin pack — CLOSED in source.** `CompositePack::origin_packs` recursively enumerates every genuine owner ([pack.rs:2498](crates/cli/src/pack.rs:2498)-[2511](crates/cli/src/pack.rs:2511)); the freeze loops that result without assuming two packs ([pack.rs:300](crates/cli/src/pack.rs:300)-[325](crates/cli/src/pack.rs:325)). The read-race in Claim 1 is the adjacent remaining bypass.

- **CX-9 — source completeness CLOSED; behavioural cells remain NOT CLOSED by this source pass.** See the exhaustive registry above.

- **CX-13 — CLOSED only for this row’s intersections.** Pack construction runs the freeze before returning the pack to any caller ([pack.rs:1997](crates/cli/src/pack.rs:1997)-[2045](crates/cli/src/pack.rs:2045)); unrelated transaction/destruction registries remain outside this scoped row.

- **`cwd-unreadable` — NOT CLOSED.** No new source establishes its runtime behaviour; it remains fixture-dependent.

- **`pack-resource-missing` — promoted to Claim 1.**

## Consistent findings

M54’s seam holds: only [pack_builtin.rs:43](crates/cli/src/pack_builtin.rs:43)-[62](crates/cli/src/pack_builtin.rs:62) names or embeds the two pack directories, and constructors take versions explicitly ([pack_builtin.rs:72](crates/cli/src/pack_builtin.rs:72)-[85](crates/cli/src/pack_builtin.rs:85)). The fence scans production literals but cannot exercise runtime reads ([pack_path_fence.rs:187](crates/cli/tests/pack_path_fence.rs:187)-[223](crates/cli/tests/pack_path_fence.rs:223)).

Production embedding sites are: `pack_builtin`’s two pack `include_dir!`s; `adapter`’s adapter-directory `include_dir!`; and `setup`’s QUICKSTART/MIGRATING `include_str!`s. I found no production embed in `engine`; its pack-byte includes are test-only.

`AllowsCreate` closes its keys to `{type, as, new}` with `deny_unknown_fields` ([compose.rs:46](crates/engine/src/compose.rs:46)-[71](crates/engine/src/compose.rs:71)); malformed entries fail `load_workflow_def` ([compose.rs:2162](crates/engine/src/compose.rs:2162)-[2183](crates/engine/src/compose.rs:2183)), and the eager pack-load sweep reaches every pack-loading door.

The two `planning-record` hint rewords correctly leave the hash unmoved: all slot hints, including nested item slots, are erased before hashing ([manifest.rs:45](crates/engine/src/manifest.rs:45)-[107](crates/engine/src/manifest.rs:107)). I found no violation of the stated boundary: M54 moved no hash; M55 added only the two v1 methodology rows; rc.24’s trailer and `claim-driven` hint changed no schema hash or pinned JSON key.

Bounds: source-only, read-only review of the rc.24 row; no binary, tests, races, or reproductions were run, and no files were written.