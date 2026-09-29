<!-- M51 per-axis review — axis 6 · Codex source pass, verbatim · read against `jigc 1.0.0-rc.15` (commit 577a0099), 2026-09-16 -->

## Claims

No grounded completeness defects found.

## Consistent findings

- All composed producers use the shared rendering seam: start/resume/re-entry at [cli.rs:1451](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:1451), workflow preview/re-entry at [cli.rs:1575](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:1575), migrate at [migrate.rs:81](/Users/maurice/projects/gherrink-jigc/crates/cli/src/migrate.rs:81), and milestone execute at [milestone.rs:3659](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3659). The registry carries start, workflow, migrate, and milestone execute at [render.rs:5363](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5363), [render.rs:5375](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5375), [render.rs:5428](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5428), and [render.rs:5953](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5953).

- The three resume states are expressible: id-less compositions omit the line; top-level tasks route through `jigc start --task`; milestone sub-tasks route through `jigc workflow <W> --task` from their worktree ([render.rs:464](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:464), [render.rs:486](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:486), [render.rs:505](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:505)). The discriminator uses the same `owning_milestone` predicate as the lifecycle guard ([start.rs:2418](/Users/maurice/projects/gherrink-jigc/crates/cli/src/start.rs:2418)).

- Orientation covers exactly unset, clean, and active-task states ([render.rs:5262](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5262)). An unavailable sweep becomes `findings: unknown — <reason>`, not `none` ([render.rs:181](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:181)); orientation preserves the failure reason at [orient.rs:154](/Users/maurice/projects/gherrink-jigc/crates/cli/src/orient.rs:154).

- `describe`, `doc show`, `doc list`, and `task validate` all have registry rows ([render.rs:5492](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5492), [render.rs:5618](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5618), [render.rs:5696](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5696), [render.rs:5723](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5723)).

- Every executable `jigc …` form found in both step trees has a corresponding clap verb and named flags. In particular, staged `doc show`/`doc list --task` are real read forms ([doc.rs:451](/Users/maurice/projects/gherrink-jigc/crates/cli/src/doc.rs:451), [doc.rs:473](/Users/maurice/projects/gherrink-jigc/crates/cli/src/doc.rs:473)), and `task validate` accepts the named task id ([task.rs:225](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:225)).

- Catalog membership and narration share workflow front matter. Selectable entries are mechanically required to carry non-empty `when`, `description`, and `usage`; every off-catalog workflow must carry `suppressed: {reason, expires}` ([pack.rs:681](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:681), [pack.rs:720](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:720), [pack.rs:737](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:737)). `describe` projects the suppression reason explicitly ([introspect.rs:226](/Users/maurice/projects/gherrink-jigc/crates/engine/src/introspect.rs:226)).

Read: `CLAUDE.md`; the named render/start/migrate/milestone/describe/orientation files; `cli.rs`, `doc.rs`, `task.rs`; both command catalogs; every workflow and step YAML under the two requested step trees; and all symbol consumers found with `rg`.