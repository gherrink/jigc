<!-- M51 per-axis review — axis 5 · Codex source pass, verbatim · read against `jigc 1.0.0-rc.15` (commit 35195f56), 2026-09-16 -->

## Claims

1. **The pre-dispatch `current_dir()` failure bypasses both registered reject funnels, so `--format json` emits plain stderr instead of the declared `Reject::Error` JSON document.**

   - Evidence:
     - Dispatchers print the error directly and return exit 1 before reaching the format-aware funnel: `crates/cli/src/cli.rs:695-700`, `720-725`, `753-758`, `776-781`, `794-799`, `810-815`, `830-835`; the same pattern recurs at `875-1073` and `1428-1615`.
     - The proper shared funnel serializes operational failures as `{"error": …}`: `crates/cli/src/invocation_log.rs:322-343`, `crates/cli/src/render.rs:4937-4950`.
     - The registry declares only two reject shapes—`Reject::Error` and `Reject::Findings`: `crates/cli/src/render.rs:6002-6023`.
     - Its driving proof requires each reject to produce exactly one JSON document on stderr with stdout empty: `crates/cli/tests/format_json_success_axis.rs:1378-1436`.
   - Proposed reproduction:
     ```sh
     d="$(mktemp -d)"
     cd "$d"
     rmdir "$d"
     /absolute/path/to/jigc --format json describe
     ```
     Expected wrong behavior: exit 1, empty stdout, and plain stderr resembling `cannot determine the current directory: No such file or directory`; stderr is not parseable JSON. The same bypass is reachable through many other doors, including `setup`, `uninstall`, `task list`, `config list`, and `milestone list-tasks`.
   - Confidence: **High**.

## Consistent reading

- Read `CLAUDE.md`, all of `ENVELOPE_ARMS`, the `DocAck`, `TaskAck`, and `ConfigAck` definitions/renderers and their consumers, `OrientationView`, the four proof tests, and every `serde_json::to_string*`/`json!` occurrence under `crates/cli/src`.
- The registry contains all 47 clap leaves and 60 arms; the tests enforce clap-leaf coverage, enum-arm coverage, declared/driven arm equality, exact keys, stream, and exit behavior: `crates/cli/tests/format_json_success_axis.rs:1175-1569`.
- The requested removals are reflected in declarations:
  - setup has no `installed`: `crates/cli/src/render.rs:5385-5397`
  - uninstall has no `uninstalled`: `5402-5406`
  - migration review has no `review`: `5790-5798`
  - milestone `list-tasks` has only `text`, no `hook_output`: `5940-5950`
- The only explicitly unpinned success shapes are `describe` and the milestone text acknowledgements; each supplies a reason, while the remaining success rows are pinned.
- The unusual closed shapes are explicitly represented rather than hidden: `task list` is `ArrayOf(id,intent,workflow)`, `doc show` has scalar and data-keyed slice arms, and all are driven.
- Stdout JSON renderers for orientation/composition, validation, finalize, document acknowledgements/reads, task acknowledgements/diff/list, config reads/writes, setup/uninstall, ingest, unmanage, rename/relocate, corpus migration, milestone operations, migration review, and describe each map to registry rows.
- Other serialization hits do not reach command stdout: `setup.rs:163` formats generated front matter; `adapter.rs:932,1082` writes adapter settings; `invocation_log.rs:455` writes the JSONL log; `doc.rs:5271,5608` constructs nested values consumed by registered document projections.