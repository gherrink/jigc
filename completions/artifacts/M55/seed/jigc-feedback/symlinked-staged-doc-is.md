---
kind: bug
found-in: review:M53-per-axis-rc17/(3,F-1)
about: jigc task discard
jigc-version: 1.0.0-rc.17
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# A symlinked staged doc is foreign at discard and jigc's own at list

## Description

A symlink planted at a staged-doc name in a task's working area, `.jigc/tasks/<id>/docs/adr:via-symlink.md`, gets two identities from one binary. `jigc task discard` names it under `task-discard.foreign-bytes`, as a path jigc did not write. `jigc doc list --task` lists it as `adr:via-symlink … managed`, and `jigc task discard --force` acks *dropped staged edits to: adr:via-symlink*, the entry the same command had just called foreign. The two reads disagree on shape: `engine::state::foreign_area_paths` reads the entry with `DirEntry::file_type()`, which does not follow a symlink, and `crate::task::staged_doc_ids` reads it with `std::fs::metadata`, which does. M52 Increment 4 / T1's shape fence closed the directory axis of this class and left the symlink axis open. Nothing is lost: the symlinked doc is never promoted, and `remove_dir_all` unlinks the symlink without descending it, so its target survives `--force`. The guard over-claims, which is the safe direction. The rc.18, rc.19 and rc.20 re-reviews kept it open, and rc.20 added a fourth half: the `--force` warning calls the entry *the only copy of these bytes* while its target survives.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. `task discard` exits 1 with `task-discard.foreign-bytes` naming `.jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md`. `doc list --task` exits 0 with `adr:via-symlink  docs/decisions/via-symlink.md  managed`. `task discard --force` exits 0 with *dropped staged edits to: adr:via-symlink, commit:tidy-the-readme (transient)*, under the warning *the working area is the only copy of these bytes — they are not recoverable*. The symlink's target still holds `TARGET-BYTES`.

## Repro

```sh
rig=$(dev/jigc-rig fresh --start quick-fix "tidy the readme") || exit; eval "$rig"
OUT=$(mktemp -d); printf 'TARGET-BYTES\n' > "$OUT/body.md"
A=.jigc/tasks/tidy-the-readme/docs
mkdir -p "$A"
ln -s "$OUT/body.md" "$A/adr:via-symlink.md"
$JIGC task discard tidy-the-readme; echo "exit $?"          # 1, task-discard.foreign-bytes names it
$JIGC doc list --task tidy-the-readme; echo "exit $?"       # 0, adr:via-symlink … managed
$JIGC task discard tidy-the-readme --force; echo "exit $?"  # 0, "dropped staged edits to: adr:via-symlink"
cat "$OUT/body.md"                                          # TARGET-BYTES: the target survives
```

## Resolution
