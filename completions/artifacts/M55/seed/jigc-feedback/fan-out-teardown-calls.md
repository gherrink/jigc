---
kind: bug
found-in: review:M53-per-axis-rc18/(3,F-4)
about: jigc milestone finalize
jigc-version: 1.0.0-rc.18
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Fan-out teardown calls a deleted tracked file unstaged and unrecoverable

## Description

In a sub-task worktree, a tracked file deleted without staging the deletion is narrated by the boundary's teardown as `keeper.md (never staged)`. It sits under *discarded with the fan-out worktrees (not committed, not recoverable)* and under the stderr note *the fan-out worktree is the only copy of these bytes — they are not recoverable*. The bytes are in git at `HEAD` and in the main checkout. The same block is right for the deletion's true siblings, an ignored `build/` and an untracked `scratch.txt`, so the defect is the deletion cell. No byte is lost and no exit code is wrong. What it costs is the credibility of the one warning in front of a real `--force` deletion. The rc.19 and rc.20 re-reviews kept it open.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. `git -C .jigc/worktrees/first-sub status --porcelain` reads ` D keeper.md` and `A  subwork.txt`. `jigc milestone finalize axis-three-probe` exits 0, commits `subwork.txt`, and prints `first-sub: keeper.md (never staged)` under *not committed, not recoverable*, with the same file under the stderr warning. `git show HEAD:keeper.md` and the main checkout's `keeper.md` both read `TRACKED-BYTES-AT-HEAD`.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
printf 'TRACKED-BYTES-AT-HEAD\n' > keeper.md; git add keeper.md; git commit -qm "chore: keeper"
$JIGC milestone create "Axis three probe" > /dev/null
$JIGC milestone add-task axis-three-probe "first sub" > /dev/null
$JIGC milestone provision axis-three-probe
W=.jigc/worktrees/first-sub
printf 'SUBWORK\n' > "$W/subwork.txt"; git -C "$W" add subwork.txt
rm "$W/keeper.md"                                  # delete a TRACKED file; do not stage it
git -C "$W" status --porcelain                     # " D keeper.md"
$JIGC milestone join axis-three-probe
$JIGC milestone finalize axis-three-probe; echo "exit $?"
#   0; "first-sub: keeper.md (never staged)" under "not committed, not recoverable"
git show HEAD:keeper.md; cat keeper.md             # TRACKED-BYTES-AT-HEAD, twice: the bytes are in git
```

## Resolution
