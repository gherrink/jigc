<!-- The unseeded Codex source pass for ROW 6 (write surface), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

Source reviewed at tag `jigc-v1.0.0-rc.24` (`e3a52c58`), not the checkout’s later trial-driver-only commit.

## CLAIMS

1. **The create-only decision has a check/use race: `new: true` is checked only by the CLI pre-check, while the subsequent engine create can copy in a home that becomes occupied between those operations, allowing an existing document to enter the task despite the create-only gate.**

   - Evidence:
     - `title_pre_check` performs the sole `entry.new`/`create_occupied` check: `crates/cli/src/doc.rs:3420-3440`.
     - Both doors then separately call `create_gated`: `crates/cli/src/doc.rs:3867-3892` and `crates/cli/src/doc.rs:3979-4004`.
     - `create_gated` retrieves the entry but never reads `entry.new`; after its staged-copy check it calls ordinary `create`: `crates/engine/src/state.rs:2371-2407`.
     - `create` observes a now-present canonical home and copies it into the task as an update: `crates/engine/src/state.rs:2288-2318`.
     - The initial probe is merely `canonical_path(...).is_file()`: `crates/engine/src/state.rs:2516-2528`.
     - Only the two CLI doors are production callers, so this is a race through the intended seam rather than an unregistered third door: `crates/cli/src/doc.rs:3882,3994`.
   - Proposed reproduction:
     1. Start task A and task B with `jigc start --workflow report-jigc-feedback ...`, using the same eventual title.
     2. In B, run `jigc doc create jigc-feedback --title "Race" --task <B>`.
     3. Repeatedly race `jigc doc create jigc-feedback --title "Race" --task <A>` against `jigc task finalize <B>` until B lands after A’s `title_pre_check` but before A’s `create`.
     4. Expected wrong behavior: A exits 0 with “copied in for update” and stages the just-landed document instead of exiting 1 with `create.already-exists`. Subsequent write leaves plus `jigc task finalize <A>` can commit changes to that existing finding.
   - Confidence: **high in the source defect; medium in practical race reproducibility without instrumentation.** This is potentially tier 1 because the successful path reaches a committing door and can modify the pre-existing document.

## O23 and §4 dispositions

- **O23 — carried as recorded.** When the role already holds a present document, `title_pre_check` chooses `one_doc_per_task_route`, not a distinct identity (`doc.rs:3430-3440`). That route names the held doc, the applicable task/milestone completion boundary, forced discard, and a fresh `jigc start --workflow` (`doc.rs:3534-3574`). This is the post-verdict `4aca049b` behavior.
- **Both create doors consult the gate — carried.** `run_create` and `run_author` both perform `create_admission`, then the shared `title_pre_check`, before `create_gated` (`doc.rs:3864-3892,3940-4004`).
- **Refusal before copy-in — carried in ordinary sequential execution, but incomplete under the race claimed above.** The pre-check precedes persistence at both doors (`doc.rs:3867-3872,3979-3993`).
- **On-disk scope and same-task re-run — carried.** `create_occupied` probes only the canonical home with `is_file`, independently of the staged area (`state.rs:2503-2528`); `create_gated` separately recognizes the task’s staged copy as idempotent (`state.rs:2376-2397`).
- **Strict entry keys — carried.** `AllowsCreate` is `deny_unknown_fields` and has exactly `type`, `as`, and defaulted `new` (`compose.rs:52-71`); deserialization failures become `workflow-refs.malformed-front-matter` in `load_workflow_def` (`compose.rs:2162-2183`).
- **Precedence over `write.title-ignored` — carried.** The `new` check is rank 0 (`doc.rs:3420-3441`); `write.identity-change` follows at `3468-3479`, and `write.title-ignored` at `3482-3507`.
- **Entry without `new` — carried.** The committed `write.title-ignored` arm uses `distinct_identity_route`; only the task-staged arm routes through `doc rename` (`doc.rs:3758-3804`).

## Consistent surface read

- `VERB_KINDS` includes create, author, add-item, remove-item, retitle-item, rename, set-field, and set-slot as write leaves (`cli.rs:1976-1983`); `DOCTYPE_DOORS` carries the same doctype-bearing doc surface (`cli.rs:2663-2673`). I found no third production create path.
- The non-create leaves deliberately address and mutate an already-existing document; they do not mint and therefore do not bypass the create-gate seam.
- `create.already-exists` is blocking and instance-keyed beside `create.serial-collision` (`state.rs:2649-2682`).
- Both shipped opt-ins are present: `report-inconsistency.yaml:7-8` and `report-jigc-feedback.yaml:9-10`. All other shipped entries omit `new`.
- The methodology command registry supplies `create-jigc-feedback` and `create-inconsistency` without a CLI flag (`commands.yaml:180-214`).
- The fan-out join remains outside this pre-check and retains its established suffixing path; no `new` consumer was found there.
- M55’s manifest delta is exactly two new schema-version-1 rows, `inconsistency` and `jigc-feedback` (`methodology/config/schema-manifest.yaml:125-130`); no other hash moved. The rc.23→rc.24 diff changes no schema manifest or pinned `--format json` key. **Boundary not violated.**
- F21 remains a report-only structural-delta validation gap and does not open a write-gate bypass.

## Bounds

Read: all requested implementation/design/history files, both packs’ complete workflow sets, both command registries, manifest diffs, and every consumer of `AllowsCreate`, `create_gated`, `create_occupied`, `title_pre_check`, and `already_exists_finding`. No binary scenarios or tests were run, no files or directories were written, and case-folding behavior was not empirically exercised.