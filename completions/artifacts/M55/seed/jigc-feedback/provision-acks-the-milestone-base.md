---
kind: bug
found-in: review:M53-per-axis-rc19/(2,N-3)
about: jigc milestone provision
jigc-version: 1.0.0-rc.19
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Provision acks the milestone base over a worktree it reused at another commit

## Description

`jigc milestone provision` reuses a worktree already registered at a sub-task's path untouched, as declared since M49. When that worktree stands at a different commit, the ack still reads *provisioned 1 worktree(s) for milestone:reuse-wave at base <pin> (ru-one)*. `jigc milestone finalize` later refuses with `finalize.base-mismatch` and says *the sub-task worktrees were cut from <pin>*. Neither claim is true of the reused worktree. The boundary blocks at exit 3 and nothing lands. The rc.20 re-review kept it open.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. The worktree parked by hand at the sub-task path stands at the commit after the milestone's pin. `provision` exits 0 and names the pin as the base, the worktree's `HEAD` is unchanged afterwards, and `milestone finalize` exits 3 with `finalize.base-mismatch`, again saying the worktrees were cut from the pin.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC milestone create "Reuse wave"                  # "… (shared base <pin>)"
$JIGC milestone add-task reuse-wave "ru one" > /dev/null
printf 'x\n' > extra.txt; git add extra.txt; git -c core.hooksPath=/dev/null commit -qm "chore: advance"
git worktree add -q --detach "$REPO/.jigc/worktrees/ru-one"   # parked by hand at HEAD, not the pin
git -C .jigc/worktrees/ru-one rev-parse --short HEAD # the commit after the pin
$JIGC milestone provision reuse-wave; echo "exit $?" # 0, "… at base <pin> (ru-one)"
git -C .jigc/worktrees/ru-one rev-parse --short HEAD # unchanged: not the pin the ack names
$JIGC milestone finalize reuse-wave; echo "exit $?"  # 3, finalize.base-mismatch, "cut from <pin>"
```

## Resolution
