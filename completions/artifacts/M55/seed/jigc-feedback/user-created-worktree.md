---
kind: bug
found-in: milestone:M55-planning/F6
about: jigc task finalize
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# In a user-created worktree every finalize after the first is blocked

## Description

In a git worktree the user created (`git worktree add`, not a fan-out area), committed-store reads bind to the main checkout, while the history check binds to the worktree's `HEAD`. So once one report lands there, `jigc doc show` of it in the worktree answers `store.not-found`, and every later finalize in the worktree exits 3 with `reconciliation.rename … missing`. The baseline concurrency auditor saw 12 of 12 later pairs blocked, and the main checkout's `jigc validate` exited 1. It is the baseline ledger's **L3**, which the Settle declared a bound (`design/findings-channel.md` → 6, S13), so this one doc carries both sources. The fix is a per-worktree store root, a new mechanism, and M55 does not build it.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. In a worktree added beside a fresh rig, the first `report-jigc-feedback` task finalizes at exit 0. `jigc doc show jigc-feedback:first-worktree-report` there then exits 1 with `store.not-found` at `docs/jigc-feedback/first-worktree-report.md`, a path the worktree's own commit carries and the main checkout does not, and the second report's `jigc task finalize` exits 3 with a blocking `reconciliation.rename … is missing`, still routed at `jigc unmanage`. One thing moved: the main checkout's `jigc validate` now exits 0, because M55 Increment 5 grades a history-less missing baseline advisory at store scope.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
report() {   # report <title> <slug> <intent>: one report-jigc-feedback task, filed and finalized
  local t; t=$($JIGC start --workflow report-jigc-feedback "$3" | sed -n 's/^task minted: //p')
  $JIGC doc create jigc-feedback --title "$1" --task $t > /dev/null
  $JIGC doc set-field jigc-feedback:$2#meta/kind --value bug --task $t > /dev/null
  $JIGC doc set-field jigc-feedback:$2#meta/found-in --value task:probe --task $t > /dev/null
  $JIGC doc set-field jigc-feedback:$2#meta/jigc-version --value 1.0.0-rc.22 --task $t > /dev/null
  printf 'Observed.\n' | $JIGC doc set-slot jigc-feedback:$2#description --from-file - --task $t > /dev/null
  $JIGC doc set-field commit:$t#type --value docs --task $t > /dev/null
  $JIGC doc set-field commit:$t#scope --value findings --task $t > /dev/null
  printf 'file %s\n' "$2" | $JIGC doc set-slot commit:$t#summary --from-file - --task $t > /dev/null
  printf 'why\n' | $JIGC doc set-slot commit:$t#body --from-file - --task $t > /dev/null
  $JIGC task finalize $t
}
git worktree add -q "$RIG/wt"; cd "$RIG/wt"                         # a user-created worktree
report "First worktree report" first-worktree-report first > /dev/null 2>&1; echo "first: exit $?"   # 0
$JIGC doc show jigc-feedback:first-worktree-report; echo "doc show: exit $?"                   # 1, store.not-found
report "Second worktree report" second-worktree-report second; echo "second: exit $?"         # 3, reconciliation.rename
```

## Resolution
