---
kind: bug
found-in: review:M53-per-axis-rc19/(3,F-A)
about: jigc task finalize
jigc-version: 1.0.0-rc.19
status: resolved
pinned-by: finalize_displacement::every_displace_surface_is_repo_relative_from_a_linked_worktree
date: 2026-10-03
schema-version: 1
---

# Task finalize prints host-absolute displacement paths from a linked worktree

## Description

Run from a branch-attached linked worktree, `jigc task finalize`'s displacement surfaces printed host-absolute paths for a foreign byte moved out of `.jigc/tasks/<id>/`. They appeared on the 1.0-pinned `committed.displaced[].from` and `.to` keys and in the stderr note, and, when the move failed, in the `finalize.foreign-bytes` message and route. `design/surface-contract.md`'s printed-path fence renders paths repo-relative and keeps the absolute for a path outside the repository. `.jigc/tasks/<id>/notes.txt` is inside the repository; it is outside only the standing checkout. The same cell run from the repository root, and at `milestone finalize` from the linked worktree, printed no host path. No byte was lost. The rc.19 review graded it *tier 2 / 3* and declined to collapse it, because two of the three surfaces are pinned keys, so this doc carries no `tier`. It was the un-swept sibling of rc.18's post-review MEDIUM 1.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
L=$(mktemp -d)/linked; git worktree add -q -b feat "$L"; cd "$L"
$JIGC start --workflow quick-fix "tidy the readme" > /dev/null
$JIGC doc set-field commit:tidy-the-readme#header/type --value fix
printf 'tidy\n' | $JIGC doc set-slot commit:tidy-the-readme#summary --from-file -
echo code > src.txt; git add src.txt
printf 'KEEPHP\n' > "$REPO/.jigc/tasks/tidy-the-readme/notes.txt"   # a byte jigc did not write
$JIGC --format json task finalize tidy-the-readme; echo "exit $?"
#   0; committed.displaced: from ".jigc/tasks/tidy-the-readme/notes.txt"
#                           to   ".jigc/displaced/tidy-the-readme/notes.txt"
cat "$REPO/.jigc/displaced/tidy-the-readme/notes.txt"                # KEEPHP: moved aside, not lost
```

## Resolution

Fixed in M53's pre-v1 usability batch (`d556b887`, *the displacement surfaces spell workbench paths against the workbench root*), stamped `1.0.0-rc.20`. The class was eight sites, not three. The rc.20 re-review recorded it *CLOSED* in both cells, with no host-absolute path over stdout and stderr.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). From a linked worktree on branch `feat`, a quick-fix task with a planted `.jigc/tasks/tidy-the-readme/notes.txt` finalizes at exit 0. `committed.displaced` is `[{"from": ".jigc/tasks/tidy-the-readme/notes.txt", "to": ".jigc/displaced/tidy-the-readme/notes.txt"}]`, the stderr note names the same two repo-relative paths, and the parked file holds `KEEPHP`.
