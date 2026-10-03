---
kind: bug
found-in: review:M52-per-axis/(6,D-2)
about: workflow:fix-task
jigc-version: 1.0.0-rc.16
status: open
tier: tier-2
date: 2026-10-03
schema-version: 1
---

# Fix-task composes by name and lands through its forbidden door

## Description

`fix-task` is composable by name with `jigc start --workflow fix-task "<intent>"`, which mints a top-level task with no milestone and no worktree. Line 25 of the composed walk forbids the one door that then works: *Never `git commit` and never `jigc task finalize` here.* `jigc task finalize` lands the commit at exit 0. The pack's own `suppressed.reason` says *it has no commit boundary of its own, so a router pick could never land*. Take both packs' workflows that are `selectable: false`, `creates-task: true` and declare no door. There are five. Four say they are reached *by name* and compose correctly. `fix-task` is the one whose reason says it is *spawned*, and it shares that clause verbatim with `sub-task`, which is declared. The M53 re-reviews kept it open through `1.0.0-rc.19`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. `start --workflow fix-task` exits 0 with `task minted: fix-the-thing`, and line 25 of the composed text still forbids `jigc task finalize`. `jigc task finalize fix-the-thing` then exits 0 and lands `fix: fix the thing`.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC start --workflow fix-task "fix the thing"      # exit 0, task minted: fix-the-thing
#   line 25: Never `git commit` and never `jigc task finalize` here. …
$JIGC doc set-field commit:fix-the-thing#header/type --value fix
printf 'fix the thing\n' | $JIGC doc set-slot commit:fix-the-thing#summary --from-file -
echo f > fixed.txt; git add fixed.txt
$JIGC task finalize fix-the-thing                    # exit 0 — the door line 25 forbids
git log -1 --format=%s                               # fix: fix the thing
```

## Resolution
