<!-- M52 per-axis review (re-run) — axis 6 · pinned contracts — the CODEX SOURCE PASS, verbatim. Read against the repository at commit e519e4eb, 2026-09-21. It reads; it drives nothing. Every claim below is a LEAD until the reconciler drove it — see axis-6.md for the verdict on each. -->

# Axis 6 · source pass — M52 / 1.0.0-rc.16

Commit reviewed: `e519e4ebf952bbf71b8591294101bd63be779998`.

## Claims

No grounded completeness defects found. I found no omitted door, missing composed-output state, invalid executable step command, orientation null/unknown collapse, or catalog/help contradiction that supports a new reproduction.

## M51 confirmed-row dispositions

1. **A6-1 — CLOSED.** Sub-task composition now records whether invocation occurred in its own worktree or elsewhere at [start.rs:2617](/Users/maurice/projects/gherrink-jigc/crates/cli/src/start.rs:2617), using canonical path comparison at [start.rs:2646](/Users/maurice/projects/gherrink-jigc/crates/cli/src/start.rs:2646). The renderer emits different `resume:` location clauses for both postures at [render.rs:500](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:500). The milestone boundary also reads the shared checkout’s still-staged set after landing at [milestone.rs:6618](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:6618) and [milestone.rs:6648](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:6648), providing the owed narration.

2. **A6-2 — CLOSED.** `suppressed.door` is parsed as a non-empty, shell-safe `jigc …` command at [compose.rs:248](/Users/maurice/projects/gherrink-jigc/crates/engine/src/compose.rs:248). Its clap shape is checked at pack load at [pack.rs:800](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:800). Named composition detects the declaration and returns blocking `workflow.verb-routed`, routed to the declared door, at [start.rs:834](/Users/maurice/projects/gherrink-jigc/crates/cli/src/start.rs:834) and [start.rs:872](/Users/maurice/projects/gherrink-jigc/crates/cli/src/start.rs:872). Thus verb-routed workflows cannot mint or emit degenerate composed bodies through either named-compose door.

3. **A6-3 — CLOSED.** The `implement-from-spec` body now explicitly states that nothing is listed without a committed spec and routes the reader to author one through `plan` at [locate-from-spec.yaml:4](/Users/maurice/projects/gherrink-jigc/crates/cli/pack/steps/locate-from-spec.yaml:4). The criteria projection independently states both its unbound and empty-criteria cases at [locate-from-spec.yaml:16](/Users/maurice/projects/gherrink-jigc/crates/cli/pack/steps/locate-from-spec.yaml:16).

## Re-check of the six prior source assertions

1. **Shared composed seam — remains complete.** `render::composed` owns text/JSON projection at [render.rs:330](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:330). Current callers cover start/workflow dispatch at [cli.rs:1637](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:1637), migrate at [migrate.rs:76](/Users/maurice/projects/gherrink-jigc/crates/cli/src/migrate.rs:76), and milestone execute at [milestone.rs:4501](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:4501). Their envelope rows are declared at [render.rs:6077](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6077), [render.rs:6089](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6089), [render.rs:6142](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6142), and [render.rs:6697](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6697).

2. **Resume states — strengthened and complete.** An id-less composition omits task-state lines at [render.rs:469](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:469); top-level tasks name `jigc start --task` at [render.rs:525](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:525); sub-tasks name the provisioning workflow door and now distinguish own-worktree from elsewhere at [render.rs:491](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:491).

3. **Orientation’s three states — complete.** `ENVELOPE_ARMS` declares unset, clean, and active-task variants at [render.rs:6030](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6030), [render.rs:6042](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6042), and [render.rs:6059](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6059). I found no path converting an unavailable findings sweep to `none`; the existing unknown-with-reason arm remains intact.

4. **Read-back owe-set — complete.** The envelope registry’s clap-leaf completeness contract is stated at [render.rs:6001](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:6001); the axis’s `describe`, `doc show`, `doc list`, and `task validate` rows remain present. No additional read verb was introduced into this axis’s stated owe-set.

5. **Pack step commands — consistent.** I read every step body under both requested trees and checked each executable `jigc …` form against the clap surface. Newly touched commands—including `doc set-field`, `doc show --task`, `config set finalize.fan-out.squash`, and the milestone sequence—have corresponding verbs/flags. The `suppressed.door` route family additionally receives a production clap check at [pack.rs:774](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:774).

6. **Catalog and off-catalog narration — strengthened and complete.** Catalog membership remains exactly `creates-task && selectable`; every complement member must carry `suppressed: {reason, expires}` at [pack.rs:692](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:692) and [pack.rs:731](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:731). Selectable entries require non-empty `when`, `description`, and `usage` at [pack.rs:748](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:748). `describe` projects the suppression reason through one predicate at [introspect.rs:244](/Users/maurice/projects/gherrink-jigc/crates/engine/src/introspect.rs:244) and [introspect.rs:268](/Users/maurice/projects/gherrink-jigc/crates/engine/src/introspect.rs:268).

## M52 seams and bounds

Of the newly minted seams, `suppressed.door` plus `workflow.verb-routed` directly intersects Axis 6 and is covered above. The rollback populations, task-area complement, pre-dispatch faults, git in-progress states, relocation refusals, findings-envelope owed codes, destroying-door dispositions, fixed-identity predicate, and store home-vacated sweep govern other axes; I found no Axis-6 producer or step-render bypass through them.

This was source-only: I did not execute the binary or test runtime prose/help equivalence. JSON composed output deliberately remains `{task,text}` and excludes presentation-only task-state lines at [render.rs:322](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:322).

**Schema-hash boundary:** no violation found. The diff from M51 commit `577a0099` to HEAD changes neither frozen `schema-manifest.yaml`; M52’s workflow/step/front-matter changes do not move schema hashes.