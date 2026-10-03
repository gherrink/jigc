---
kind: bug
found-in: review:M53-per-axis-rc18/(3,F-5)
about: jigc task list
jigc-version: 1.0.0-rc.18
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# A settled sub-task's restored area reads as an active task

## Description

After `jigc milestone finalize` lands, a sub-task's area restored from a faithful backup, a `cp -R` taken before the boundary with its base pin, reads three ways. `jigc task list` counts it: *1 active task(s)*, `second-sub [sub-task]`. `jigc task discard second-sub`, with `--force` or without it, refuses with `milestone.terminal` and calls it *a leftover, not live work*. `jigc task finalize second-sub` refuses with `finalize.empty-commit` and routes at *abandon it with `jigc task discard second-sub --force`*, the command that has just refused. The enumerating door asks `carries_base_pin` and never asks the committed record's terminal state. HEAD and the record do not move. Reachability is not discharged: the state is built by restoring a backup, and no jigc sequence any review drove produces it, which is why the row is tier 3. The rc.19 and rc.20 re-reviews kept all three halves open.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open, all three halves**. After the boundary lands and the backup is restored, `jigc task list` exits 0 with *1 active task(s)* and `second-sub [sub-task] second sub`. `task discard second-sub` and `task discard second-sub --force` each exit 1 with `milestone.terminal`. `task finalize second-sub` exits 3 with `finalize.empty-commit`, routed at `jigc task discard second-sub --force`.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Axis three probe" > /dev/null
$JIGC milestone add-task axis-three-probe "first sub" > /dev/null
$JIGC milestone add-task axis-three-probe "second sub" > /dev/null
$JIGC milestone provision axis-three-probe
for s in first-sub second-sub; do
  printf '%s\n' "$s" > ".jigc/worktrees/$s/$s.txt"; git -C ".jigc/worktrees/$s" add "$s.txt"
done
SAVE=$(mktemp -d); cp -R .jigc/tasks/second-sub "$SAVE/"     # a faithful backup, pin and all
$JIGC milestone join axis-three-probe
$JIGC milestone finalize axis-three-probe; echo "exit $?"    # 0
cp -R "$SAVE/second-sub" .jigc/tasks/second-sub              # restored, as a backup would be
$JIGC task list; echo "exit $?"                              # 0, "1 active task(s)": second-sub
$JIGC task discard second-sub; echo "exit $?"                # 1, milestone.terminal
$JIGC task discard second-sub --force; echo "exit $?"        # 1, milestone.terminal
$JIGC task finalize second-sub; echo "exit $?"               # 3, routed at the refusing --force
```

## Resolution
