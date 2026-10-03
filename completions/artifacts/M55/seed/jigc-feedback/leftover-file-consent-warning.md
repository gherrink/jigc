---
kind: bug
found-in: review:M53-per-axis-rc19/(3,F-B)
about: jigc milestone provision
jigc-version: 1.0.0-rc.19
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# The leftover-file consent warning lists the file's own name as its contents

## Description

When a sub-task's worktree path holds a regular file, `jigc milestone provision` refuses correctly: *the file itself — it is a file, not a worktree*. With `--force`, the warning reads *removing the leftover file .jigc/worktrees/area-one discards work that is not in git:* and then lists `area-one`, the path's own basename, where a directory leftover lists its children. `LeftoverHold.entries`' doc comment states the rule the refusal arm obeys: the entries are empty for the two shapes with no inside, *because listing an empty set beside either would read as "it holds nothing"*. The `--force` arm does not obey it. The rc.20 re-review kept it open, the one row both of its passes reached independently.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. `provision quebec-probe` exits 1 with `milestone.leftover-holds-work` and *the file itself*. `provision quebec-probe --force` exits 0 after a warning that lists `area-one` under *discards work that is not in git:*.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Quebec probe" > /dev/null
$JIGC milestone add-task quebec-probe "area one" > /dev/null   # not provisioned
mkdir -p .jigc/worktrees
printf 'LEFTOVER-FILE-BYTES\n' > .jigc/worktrees/area-one        # a FILE at the worktree path
$JIGC milestone provision quebec-probe; echo "exit $?"          # 1, refusal arm: "the file itself"
$JIGC milestone provision quebec-probe --force; echo "exit $?"  # 0
#   warning: removing the leftover file .jigc/worktrees/area-one discards work that is not in git:
#       area-one          <- the path's own basename, in the child position
```

## Resolution
