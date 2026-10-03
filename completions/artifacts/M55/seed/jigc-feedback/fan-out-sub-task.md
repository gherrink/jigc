---
kind: bug
found-in: milestone:M55-planning/F7
about: step:finalize
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: sub_task_composition::a_sub_task_composes_no_per_task_finalize_door_through_either_door
date: 2026-10-03
schema-version: 1
---

# A fan-out sub-task is told to run the per-task finalize it is refused

## Description

A fan-out sub-task composed from a workflow that includes `step:finalize` was told `Run: jigc task finalize <id>`, which exits 3 with `finalize.milestone-sub-task`. `render.rs:138-145` already switched orientation to the milestone door, but the composed `{{ cli.finalize-task }}` ref did not follow it. It is the baseline ledger's **S2** (`design/findings-channel.md` → 6). The baseline and the capabilities gap-detector both drove it.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
PATH=$(dirname "$JIGC"):$PATH                         # the Spawn: line runs `jigc` off PATH
$JIGC milestone create "Probe wave" > /dev/null
$JIGC milestone add-task probe-wave "fix the greeting" --workflow single-task > /dev/null
$JIGC milestone provision probe-wave > /dev/null
spawn=$($JIGC milestone execute probe-wave | sed -n 's/^.*Spawn: `\(.*\)`.*$/\1/p' | head -1)
eval "$spawn" > "$RIG/composed.txt"; echo "spawn: exit $?"            # the emitted line, verbatim: 0
grep 'jigc task finalize' "$RIG/composed.txt"; echo "grep: exit $?"  # 1: no per-task door
grep -o 'jigc milestone finalize probe-wave` is its only commit boundary' "$RIG/composed.txt"
```

## Resolution

Fixed in M55 Increment 3 (`197287bf`, *a fan-out sub-task's composed text omits the per-task finalize door*, and `f37d385b`, *a fan-out sub-task is asked for its commit doc only when the join reads it*). When the CLI composes for a fan-out sub-task it omits the commit-boundary steps, a set derived from the composed packs rather than hand-listed, and emits the sub-task trailer naming `jigc milestone finalize <m>` as the only commit boundary (`design/findings-channel.md` → 6, S2).

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`). A `single-task` sub-task's `Spawn:` line, run verbatim with the rig binary first on `PATH`, exits 0. Its composed text carries no `jigc task finalize`, and its trailer names `` `jigc milestone finalize probe-wave` is its only commit boundary ``.
