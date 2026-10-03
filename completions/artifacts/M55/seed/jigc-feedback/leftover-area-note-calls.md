---
kind: bug
found-in: review:M53-per-axis-rc17/(3,F-2)
about: finalize.no-task
jigc-version: 1.0.0-rc.17
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# The leftover-area note calls a symlink a directory

## Description

`engine::state::residual_area_note` hard-codes *"`<path>` is a directory carrying no base pin"*, but the predicate the door asks, `carries_base_pin`, says nothing about the area's shape. With `.jigc/tasks/ghost` a symlink to a directory outside the repository, `jigc task discard ghost` exits 1 with `finalize.no-task` and says *`.jigc/tasks/ghost` is a directory carrying no base pin*. It is not a directory. Nothing is destroyed, with `--force` or without it. Over a real directory, the sentence is true. The rc.19 re-review drove it at a second door, `task validate`, and rc.20 at a third, `task diff`, each with the byte-identical sentence from the one producer.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. `task discard ghost` and `task validate ghost` both exit 1 with *`.jigc/tasks/ghost` is a directory carrying no base pin, so it is a leftover and not a work unit*. The outside directory's `keep.txt` still holds `OUTSIDE2-KEEP`. The control over a real directory, `ghost-dir`, gets the same sentence, which is true there.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
OUT=$(mktemp -d); printf 'OUTSIDE2-KEEP\n' > "$OUT/keep.txt"
mkdir -p .jigc/tasks; ln -s "$OUT" .jigc/tasks/ghost        # a symlink, not a directory
$JIGC task discard ghost; echo "exit $?"
#   1, finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` is a directory carrying no base pin …
$JIGC task validate ghost; echo "exit $?"                   # 1, the byte-identical sentence
cat "$OUT/keep.txt"                                         # OUTSIDE2-KEEP: nothing destroyed
mkdir .jigc/tasks/ghost-dir                                 # the control: a real directory
$JIGC task discard ghost-dir; echo "exit $?"                # 1, the same sentence, true here
```

## Resolution
