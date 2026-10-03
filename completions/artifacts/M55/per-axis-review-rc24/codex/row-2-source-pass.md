<!-- The unseeded Codex source pass for ROW 2 (probe integrity · measurement / the invocation log), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

## Claims

1. **A rejected agent-authored commit permanently signs its message file, so a later human retry can land a false `Co-Authored-By` trailer at exit 0.**

   - Evidence: the seam resolves the co-author from the current environment at [crates/cli/src/task.rs:8162](crates/cli/src/task.rs:8162), reads and then overwrites a file-backed message at [crates/cli/src/task.rs:8166](crates/cli/src/task.rs:8166), and only afterwards invokes `git commit` at [crates/cli/src/task.rs:8191](crates/cli/src/task.rs:8191). A rejected commit returns without restoring that file at [crates/cli/src/task.rs:8200](crates/cli/src/task.rs:8200). On retry without the agent environment, `co_author_for` returns `None` at [crates/cli/src/task.rs:8045](crates/cli/src/task.rs:8045), but the already-signed file is passed unchanged. This contradicts the absent-environment rule encoded by `session_co_author` at [crates/cli/src/adapter.rs:244](crates/cli/src/adapter.rs:244). The trailer suite exercises successful agent and human invocations but explicitly removes the rejecting hook before each door at [crates/cli/tests/agent_co_author.rs:166](crates/cli/tests/agent_co_author.rs:166), leaving this transition uncovered.
   - Proposed reproduction: create a finalizable task and install a rejecting `pre-commit` hook; run `CLAUDECODE=1 jigc task finalize <id>` and expect exit 1/no commit; remove the hook, then run `env -u CLAUDECODE jigc task finalize <id>`. Expected wrong behaviour: exit 0 and the human retry’s commit still contains `Co-Authored-By: Claude <noreply@anthropic.com>`.
   - Confidence: **high**. This is an exit-0 law-1 attribution lie inside the trailer’s new seam code and therefore matches the stated tier-1 class.

## M54 finding and bound dispositions

- **M54 code-review finding 3 remains fixed.** All five probe-local failure arms call the one-line `refuse` emitter at [crates/cli/src/doc_code_probe/mod.rs:620](crates/cli/src/doc_code_probe/mod.rs:620) and [crates/cli/src/doc_code_probe/mod.rs:662](crates/cli/src/doc_code_probe/mod.rs:662). Spawn failure carries the attempted program at [crates/cli/src/task.rs:1698](crates/cli/src/task.rs:1698), and the engine renders it into the `crash` finding at [crates/engine/src/probe.rs:330](crates/engine/src/probe.rs:330). Non-zero stderr is bounded and included at [crates/engine/src/probe.rs:340](crates/engine/src/probe.rs:340).

- **Linux replaced-binary `/proc/self/exe`: bound still carried, source arm intact.** Linux selects `/proc/self/exe` at [crates/cli/src/invoke.rs:113](crates/cli/src/invoke.rs:113), and the dedicated test remains at [crates/cli/tests/probe_self_image.rs:66](crates/cli/tests/probe_self_image.rs:66). This source pass did not execute it.

- The other M54 `Not run` entries—real Task-tool spawning, CI timing, release infrastructure, and the four-CPU runner variant—do not fall within this row’s source seam. The rc.22 “silent failure arms” declared bound is closed in rc.24 by the cited `refuse` implementation. The partial re-review bound is this review itself.

## Consistent clean read

- `probe_intercept` is the first executable statement of `main`, before route fencing, clap, teeing, and logging: [crates/cli/src/main.rs:33](crates/cli/src/main.rs:33). `ProbeArgv::{NotProbe, Run, Refuse}` closes the literal first-word `__probe` classification at [crates/cli/src/invoke.rs:125](crates/cli/src/invoke.rs:125).

- The override treats empty as unset and non-empty as the standalone program: [crates/cli/src/invoke.rs:76](crates/cli/src/invoke.rs:76). Production self-exec and its sole argv spelling are owned at [crates/cli/src/invoke.rs:65](crates/cli/src/invoke.rs:65) and [crates/cli/src/invoke.rs:100](crates/cli/src/invoke.rs:100).

- The three production invoker consumers are present: store validation at [crates/cli/src/cli.rs:1486](crates/cli/src/cli.rs:1486), task validation/finalization at [crates/cli/src/task.rs:2495](crates/cli/src/task.rs:2495), and milestone boundary validation at [crates/cli/src/milestone.rs:6606](crates/cli/src/milestone.rs:6606). Bare orientation reaches the task sweep at [crates/cli/src/orient.rs:202](crates/cli/src/orient.rs:202); no fourth production spawn was found.

- Invocation logging is one post-dispatch best-effort append at [crates/cli/src/main.rs:101](crates/cli/src/main.rs:101). It writes only under `<jigc_home>/.jigc/logs`, and disables itself outside a project layer at [crates/cli/src/invocation_log.rs:464](crates/cli/src/invocation_log.rs:464). Append errors are discarded at [crates/cli/src/invocation_log.rs:452](crates/cli/src/invocation_log.rs:452). The record’s eight declared keys are at [crates/cli/src/invocation_log.rs:494](crates/cli/src/invocation_log.rs:494).

- `COMMITTING_DOORS` contains 11 rows at [crates/cli/src/invocation_log.rs:188](crates/cli/src/invocation_log.rs:188); `ERROR_CODE_REGISTRY` contains their 11 identities plus `migrate.review-pending` at [crates/cli/src/invocation_log.rs:267](crates/cli/src/invocation_log.rs:267). I found no release-reachable `Outcome::error`/`coded_error` identity outside it and no production hook-capable commit bypass; setup’s `--no-verify` exclusion signs separately at [crates/cli/src/setup.rs:2678](crates/cli/src/setup.rs:2678).

- The named probe and logging fences cover ordinary/skew/malformed argv, five failure reasons, bounded stderr, replaced-image self-exec, zero child log records, record shape, clap errors, review hold, and all registered committing-door rejection identities. They do **not** cover the claim’s rejected-agent → human-retry transition.

## Bounds and schema boundary

Read-only source inspection only: no binary was driven and no files or directories were written. Completeness is bounded to production Rust call sites and the named tests; platform/runtime and remote release behaviour remain undriven.

No schema-hash-boundary violation found. M55’s methodology manifest has exactly the two new v1 rows at [crates/cli/packs/methodology/config/schema-manifest.yaml:125](crates/cli/packs/methodology/config/schema-manifest.yaml:125). The rc.24 planning-record edit changes only `Slot.hint`, deliberately erased from the hash projection at [crates/engine/src/manifest.rs:45](crates/engine/src/manifest.rs:45). The trailer changes neither the frozen `commit` schema nor any pinned JSON key.