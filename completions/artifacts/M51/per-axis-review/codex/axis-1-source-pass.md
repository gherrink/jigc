<!-- M51 per-axis review — axis 1 · Codex source pass, verbatim · read against `jigc 1.0.0-rc.15` (commit 35195f56), 2026-09-16 -->

## Claims

No grounded completeness defect found. I found neither an omitted door nor a caller-token path that bypasses the relevant adjudication seam.

## Consistent findings

- `ARG_TOKENS` classifies every path-bearing argument ID as `path`, `file`, `from_file`, `from`, `target`, or `value;` its documented limitation is only a deliberately incorrect `PlainValue::Other` classification. [crates/cli/src/cli.rs:2154](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2154), [crates/cli/src/cli.rs:2234](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2234)

- The clap-tree fences are bidirectional: all arguments must appear in `ARG_TOKENS`, and every occurrence classified as path-bearing must have a `PATH_ARG_OCCURRENCES` row whose argv parses to that exact leaf and argument. [crates/cli/src/cli.rs:3948](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:3948), [crates/cli/src/cli.rs:4225](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:4225)

- `DOCTYPE_DOORS` includes all bare-doctype and address-bearing leaves, including the non-obvious `milestone add-from-spec` `spec_addr` occurrence. [crates/cli/src/cli.rs:2357](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2357)

- Work-unit IDs and verbatim `--slug` overrides are projected from the same total argument classification rather than independent name lists. [crates/cli/src/cli.rs:2311](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2311), [crates/cli/src/cli.rs:2328](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2328)

- The path registry distinguishes occurrences and conditional arms, including the `-` stdin sentinel, root versus non-root config values, address tails that resolve against closed vocabularies, and deliberate unrestricted handoff-file reads. [crates/cli/src/cli.rs:2808](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2808), [crates/cli/src/cli.rs:2846](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2846), [crates/cli/src/cli.rs:3220](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:3220)

- Migration-source admission uses `resolve_source_token`, covering pathspec magic, repository confinement, `.git`, `.jigc`, and symlinked components before recording the source. [crates/cli/src/trackable.rs:280](/Users/maurice/projects/gherrink-jigc/crates/cli/src/trackable.rs:280)

- The persisted `source-path` has one raw reader returning the provenance-only `MigrationSource` type; consumers receive only `recorded()` or lexical `normalized()` views. [crates/engine/src/state.rs:700](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:700), [crates/engine/src/state.rs:751](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:751)

- Retirement planning may compare or carry the normalized source, but the destructive sink re-adjudicates it into `ValidatedRetirement` immediately before `remove_file`; the review-hold display uses that same validated value. [crates/engine/src/finalize.rs:400](/Users/maurice/projects/gherrink-jigc/crates/engine/src/finalize.rs:400), [crates/cli/src/task.rs:3540](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3540), [crates/cli/src/task.rs:3664](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3664)

- Printed route operands pass through `shell_token`/`shell_operand`; all route constructors invoke the backticked-command span fence in debug builds. [crates/engine/src/finding.rs:814](/Users/maurice/projects/gherrink-jigc/crates/engine/src/finding.rs:814), [crates/engine/src/finding.rs:840](/Users/maurice/projects/gherrink-jigc/crates/engine/src/finding.rs:840), [crates/engine/src/finding.rs:1041](/Users/maurice/projects/gherrink-jigc/crates/engine/src/finding.rs:1041)

Read: `CLAUDE.md`; the requested `cli.rs`, `trackable.rs`, `config.rs`, `migrate.rs`, `task.rs`, `finalize.rs`, and `finding.rs` regions; `engine/state.rs`; and tree-wide references to all named registries, path predicates, migration-source readers, retirement types, and shell-rendering helpers.