---
kind: bug
found-in: review:M53-per-axis-rc18/(2,F-2)
about: jigc milestone finalize
jigc-version: 1.0.0-rc.18
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Milestone finalize commits a foreign repository's worktree as sub-task work

## Description

A second repository's linked worktree, parked at `.jigc/worktrees/<sub-id>` with `provision` never run, is committed into this repository by `jigc milestone finalize` at exit 0, and acked as the sub-task's own work: *added bsecret.txt … sub-tasks: fw-area: 1 code file*. The subject rule that admits it is M46 Increment 2 / T1: `provisioned_worktrees` takes the on-disk path rather than the registered set. No byte is lost. The other repository's worktree, its index, an untracked plant and its registration all survive, and `.jigc/displaced/` is never created. What is wrong is the ack, a law-1 claim. The rc.18 driver and reconciler did not grade it tier 1, because the commit is reversible, nothing is destroyed, and the state needs a foreign worktree at exactly that path. The variant where the squatting worktree belongs to the same repository was named and not driven. The rc.19 and rc.20 re-reviews kept it open.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. `jigc milestone join foreign-wave` exits 0 with *0 doc(s) merged*. `jigc milestone finalize foreign-wave` exits 0 with *added bsecret.txt … 2 files committed … sub-tasks: fw-area: 1 code file*. `git cat-file -p HEAD:bsecret.txt` prints `B-SECRET-PAYLOAD`.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
B=$(mktemp -d)                                   # a second repository
git -C "$B" init -q -b main
printf 'one\n' > "$B/one.txt"; git -C "$B" add one.txt; git -C "$B" commit -qm one
$JIGC milestone create "Foreign wave" > /dev/null
$JIGC milestone add-task foreign-wave "fw area" > /dev/null
W=$REPO/.jigc/worktrees/fw-area                  # not provisioned: B's own worktree parks there
git -C "$B" worktree add -q "$W" -b fwbranch
printf 'B-SECRET-PAYLOAD\n' > "$W/bsecret.txt"; git -C "$W" add bsecret.txt
$JIGC milestone join foreign-wave; echo "exit $?"       # 0
$JIGC milestone finalize foreign-wave; echo "exit $?"   # 0, "sub-tasks: fw-area: 1 code file"
git cat-file -p HEAD:bsecret.txt                         # B-SECRET-PAYLOAD, committed into this repo
```

## Resolution
