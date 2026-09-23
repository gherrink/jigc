<!-- M53 THIRD PARTIAL per-axis review — axis 6 — the unseeded Codex SOURCE pass, verbatim. Source-only: it drove nothing and wrote nothing. Captured 2026-09-23 against the rc.19 tree. -->

# Axis 6 · source pass — M53 rc.19

Commit/version reviewed: `1.0.0-rc.19`, after the cwd-dependence arc. Source-only; I did not execute the binary, create fixtures, or write files.

## Claims

1. **The new absolute `Spawn:` guarantee has a reachable non-UTF-8-root escape hatch that silently restores the cwd-dependent relative form.**

   - Evidence: production `milestone execute` constructs the absolute worktree from `jigc_home`, but if `Path::to_str()` fails it calls `SubTask::new`, explicitly discarding that absolute path ([milestone.rs:2656](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2656), [milestone.rs:2671](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2671), [milestone.rs:2673](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2673)). `SubTask::new` stores `worktree: None` ([data_value.rs:232](/Users/maurice/projects/gherrink-jigc/crates/engine/src/data_value.rs:232)); the fan-out renderer converts that state to `.jigc/worktrees/<id>` ([compose.rs:1771](/Users/maurice/projects/gherrink-jigc/crates/engine/src/compose.rs:1771), [compose.rs:1779](/Users/maurice/projects/gherrink-jigc/crates/engine/src/compose.rs:1779)). This contradicts the adjacent contract that the production line is absolute because it is pasted from an unknown cwd ([data_value.rs:212](/Users/maurice/projects/gherrink-jigc/crates/engine/src/data_value.rs:212), [data_value.rs:217](/Users/maurice/projects/gherrink-jigc/crates/engine/src/data_value.rs:217)).
   - Proposed reproduction: on Unix, create a repository whose pathname contains a non-UTF-8 byte; set up a milestone with a provisioned sub-task; run `jigc milestone execute <milestone>` from a repository subdirectory. Expected wrong behavior: its `Spawn:` line contains `cd .jigc/worktrees/<sub>` rather than a quoted absolute operand, and copy-running it fails because that relative directory does not exist from the caller’s cwd.
   - Confidence: **high from source; runtime confirmation needed for platform/fixture support**.

No other grounded completeness defect was found.

## M51 confirmed-row dispositions

1. **A6-1 — CLOSED (argv).** All three `resume:` states remain expressible: no task omits the lines ([render.rs:469](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:469)); a top-level task emits `jigc start --task` ([render.rs:525](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:525)); a sub-task emits its recorded workflow through `jigc workflow … --task` and distinguishes own-worktree from elsewhere ([render.rs:491](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:491), [render.rs:500](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:500)). The current arc’s `Spawn:` production path supplies the absolute worktree ([milestone.rs:2666](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2666)), subject to Claim 1’s exceptional datum.

2. **A6-2 — CLOSED (argv).** `suppressed.door` is lexically restricted to a non-empty, shell-safe `jigc …` command ([compose.rs:230](/Users/maurice/projects/gherrink-jigc/crates/engine/src/compose.rs:230), [compose.rs:248](/Users/maurice/projects/gherrink-jigc/crates/engine/src/compose.rs:248)); the CLI then parses it against the actual clap tree ([pack.rs:790](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:790), [pack.rs:816](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:816)). Named composition therefore continues to refuse verb-routed workflows as `workflow.verb-routed`.

3. **A6-3 — CLOSED (datum).** `implement-from-spec` still states the empty committed-spec case and routes to `plan`, while separately stating the unbound/empty-criteria cases ([locate-from-spec.yaml:4](/Users/maurice/projects/gherrink-jigc/crates/cli/pack/steps/locate-from-spec.yaml:4), [locate-from-spec.yaml:16](/Users/maurice/projects/gherrink-jigc/crates/cli/pack/steps/locate-from-spec.yaml:16)).

Axis 6 has no expected tier-2/3 baseline row triaged to 1.x.

## Consistent coverage

- The four producers still converge on `render::composed`: start/workflow dispatch ([cli.rs:1638](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:1638)), migrate ([migrate.rs:76](/Users/maurice/projects/gherrink-jigc/crates/cli/src/migrate.rs:76)), and milestone execute ([milestone.rs:4884](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:4884)). Their registry rows remain present, including milestone execute ([render.rs:6254](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6254), [render.rs:6367](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6367), [render.rs:6922](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6922)).
- The read-back owe-set carries `describe`, every `doc show` arm, `doc list`, and `task validate` ([render.rs:6432](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6432), [render.rs:6558](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6558), [render.rs:6665](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6665), [render.rs:6692](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6692)).
- Orientation retains unset, clean, and active-task registry arms; I found no conversion of an unavailable findings sweep into `none`.
- Every executable `jigc …` form in both requested step trees corresponds to a clap leaf with the named flags. No stale verb or flag was found.
- Catalog membership still derives from `creates-task && selectable`; hidden workflows retain `suppressed.reason`, and `describe` projects that reason through one predicate ([introspect.rs:226](/Users/maurice/projects/gherrink-jigc/crates/engine/src/introspect.rs:226), [introspect.rs:269](/Users/maurice/projects/gherrink-jigc/crates/engine/src/introspect.rs:269)). No catalog/help contradiction was found.
- I traced the M52 registries named in the brief and their consumers. Axis 6 directly intersects `suppressed.door`/`workflow.verb-routed`, `ENVELOPE_OWED_CODES`, `ENVELOPE_ARMS`, and the composed read surfaces. I found no axis-6 bypass through `ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `PRE_DISPATCH_FAULTS`, `InProgress::ALL`, `RelocateRefusal::ALL`, `DESTROYING_DOORS`, the fixed-identity predicate, `STORE_EXIT_FLIPS`, or the prior-home sweep.
- `PATH_ARG_OCCURRENCES` now declares `migrate <PATH>` as cwd-based ([cli.rs:3138](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:3138)); this does not introduce an invalid step command.
- **Zero schema-hash movement remains intact:** neither frozen schema manifest changed across the cwd arc.

## Bounds

I read `CLAUDE.md`; all requested M51/M52/M53 baselines; the cwd census, verdict addendum, and four 2026-09-23 decision entries; the named render/start/migrate/milestone/describe files; both step trees; and symbol consumers found by tree-wide search. This pass establishes source completeness, not runtime prose/help equivalence. The working tree already contained unrelated modifications; I changed nothing.