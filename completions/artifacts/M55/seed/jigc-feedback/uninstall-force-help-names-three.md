---
kind: bug
found-in: review:M52-per-axis/(8,N-1)
about: jigc uninstall
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Uninstall force help names three guards out of four

## Description

`jigc uninstall --help` contradicts itself about its own guard count, and the `--force` clause is the half that is false. The about text says *Four states it refuses instead of destroying … pass `--force`, which deletes all four*. The `--force` help names three: *a fan-out worktree with content, an open task's staged docs, or a workbench file no index has a copy of … Inert when all three guards are already clean*. The fourth guard, `uninstall.foreign-bytes` (anything parked under `.jigc/displaced/`, or a file jigc did not write in a working area), is M52 Increment 4's own new member. When only that guard is dirty, `--force` is not inert: it deletes the only copy of the parked bytes. The sibling `milestone discard`'s flag help was swept in the same wave and names all three of its guards, so this is an omission rather than a decision.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. The `--force` help still reads *… Inert when all three guards are already clean*, while the about text says four. With only `.jigc/displaced/u-four/notes.txt` dirty, `jigc uninstall` exits 1 with `uninstall.foreign-bytes`. `jigc uninstall --force` exits 0, warns *the relocation workbench is the only copy of these bytes — they are not recoverable*, and removes `.jigc/`.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC uninstall --help     # about: "Four states it refuses … `--force`, which deletes all four"
#                            --force: "… Inert when all three guards are already clean"
mkdir -p .jigc/displaced/u-four; printf 'mine\n' > .jigc/displaced/u-four/notes.txt
$JIGC uninstall            # exit 1, uninstall.foreign-bytes  .jigc/displaced/u-four/notes.txt
$JIGC uninstall --force    # exit 0 — not inert
#   warning: removing the relocation workbench .jigc/displaced discards work that is not in git:
#   note: the relocation workbench is the only copy of these bytes — they are not recoverable.
```

## Resolution
