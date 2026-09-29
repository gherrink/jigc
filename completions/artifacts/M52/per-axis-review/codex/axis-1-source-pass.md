<!-- M52 per-axis review (re-run) — axis 1 ·  — the CODEX SOURCE PASS, verbatim. Read against the repository at commit e519e4eb, 2026-09-21. It reads; it drives nothing. Every claim below is a LEAD until the reconciler drove it — see axis-1.md for the verdict on each. -->

# M52 Axis 1 source pass — caller tokens

Reviewed commit `e519e4ebf952bbf71b8591294101bd63be779998` (`1.0.0-rc.16`). Read-only review; I drove no binary and wrote no files.

## M51 §A row dispositions

1. **A1-D1 — CLOSED.** Non-canonical identities for placement and singleton doctypes now refuse through the shared fixed-identity predicate `placement.is_some() || singleton` (`crates/engine/src/schema.rs:140-162`). The nine `doc` doors enforce it in `parse_verb_addr` (`crates/cli/src/doc.rs:6310-6325`); the three sibling boundaries enforce the same rule in `rename` (`crates/cli/src/rename.rs:380-399`), `milestone add-from-spec` (`crates/cli/src/milestone.rs:1878-1894`), and `task bind` (`crates/cli/src/task.rs:3273-3306`). The shared refusal permits only `slug == schema.ty` and emits `store.fixed-identity` otherwise (`crates/cli/src/task.rs:1687-1709`). The standing door sweep covers both placement and manufactured location-singleton shapes (`crates/cli/tests/fixed_identity_axis.rs:591-646`).

2. **A1-D2 — CLOSED.** `relocate --from` now declares and applies an installed-root refusal derived from `is_workbench_root` and `installed_artifact_root` (`crates/cli/src/cli.rs:3150-3168`). Installed roots are derived from the adapter profile rather than a `.claude` literal (`crates/cli/src/config.rs:896-900`), and the common unusable-root family returns a refusal for them (`crates/cli/src/config.rs:1014-1026`). The test enumerates `.jigc` plus every declared installed-artifact root, requires the proper blocking code and runnable route, and verifies an unchanged tree (`crates/cli/tests/path_arg_occurrence_axis.rs:836-889`).

3. **A1-D3 — CLOSED.** The registry now states that the stock corpus has no reachable freeze-exempt relocation row and names the manufactured manifest-less witness (`crates/cli/src/cli.rs:3153-3159`). The whole-axis driver constructs exactly that witness and selects it for the `relocate/from` occurrence (`crates/cli/tests/path_arg_occurrence_axis.rs:559-580`, `605-617`). Thus the registered argv reaches its caller-token arm rather than faulting first on frozen-doctype posture.

4. **A1-D4 — CLOSED.** Every user-address boundary now runs the shared malformed-slug-head guard before filesystem resolution: `doc` (`crates/cli/src/doc.rs:6315-6324`), `task bind` (`crates/cli/src/task.rs:3273-3276`), `rename` before `doc_path` (`crates/cli/src/rename.rs:380-399`), and `milestone add-from-spec` before reconciliation (`crates/cli/src/milestone.rs:1878-1894`). The four-boundary test requires exit 1, `store.name-ceiling`, no raw OS fault, and no staged mutation (`crates/cli/tests/fixed_identity_axis.rs:1513-1585`).

5. **A1-D5 — CLOSED.** `unusable_root_reason` checks every component against the OS-name ceiling before attempting the filesystem walk (`crates/cli/src/config.rs:945-987`). The standing test sweeps both root knobs, requires `config.unusable-root`, verifies no staged change, and checks that the reason identifies the over-limit component length (`crates/cli/tests/root_knob_rules.rs:853-878`).

## Claims

No grounded completeness defect found. Consequently there is no proposed binary reproduction: I found neither an omitted Axis 1 door nor a caller-token bypass with file-and-line evidence.

## Consistent source findings

- `ARG_TOKENS` remains total over the clap argument vocabulary and classifies `file`, `from`, `from_file`, `path`, `target`, and `value` as path-bearing (`crates/cli/src/cli.rs:2399-2438`).
- `DOCTYPE_DOORS` includes every bare-doctype/address leaf, including `milestone add-from-spec` (`crates/cli/src/cli.rs:2522-2543`).
- `WORK_UNIT_ID_DOORS` and `SLUG_DOORS` are projections of that same classification, not independent argument-name lists (`crates/cli/src/cli.rs:2486-2504`, `2589-2783`, `2829-2938`).
- The occurrence fence is bidirectional over every real `(leaf, argument)` pair and checks that each row’s argv parses to its declared leaf and argument (`crates/cli/src/cli.rs:4413-4488`).
- Migration admission and destructive retirement share `resolve_source_token`, covering pathspec magic, outside-repository resolution, `.git`, `.jigc`, and symlink components (`crates/cli/src/trackable.rs:291-330`; `crates/cli/src/migrate.rs:210-225`; `crates/cli/src/task.rs:4270-4313`).
- The only persisted-source reader returns the provenance wrapper `MigrationSource` (`crates/engine/src/state.rs:1081-1127`). The retirement consent surface adjudicates it (`crates/cli/src/task.rs:2764-2787`), and the unlink sink independently re-adjudicates immediately before `remove_file` (`crates/cli/src/task.rs:4392-4443`). I found no raw source-path read bypassing that typed sink.
- Routes continue to quote caller bytes through `shell_token`/`shell_operand`; the command-span parser remains centralized (`crates/engine/src/finding.rs:814-866`, `1030-1060`).

The M52 registries named in the rerun brief do not expose an Axis 1 bypass where they intersect this subject: fixed identity is centralized; `ENVELOPE_OWED_CODES` carries the resulting store finding; and the new rollback, task-area, pre-dispatch, in-progress, relocation-refusal, destroying-door, suppression, and home-vacated registries do not introduce additional caller path components outside `ARG_TOKENS`.

## Bounds

This is source completeness only: no runtime behavior was driven. The remaining declared limitation is explicit—an argument incorrectly classified as `PlainValue::Other` could evade the path-occurrence registry (`crates/cli/src/cli.rs:2380-2396`).

**Zero schema-hash movement holds.** `git diff --name-only 65d3cd76..HEAD` over both shipped schema trees, both manifests, and both snapshot trees produced no paths. I see no violation: M52 changed the `doc schema` projection contract, not frozen schema hashes, schema versions, or corpus migrations.