---
kind: bug
found-in: milestone:M53-settle/(D)(a)
about: jigc doc add-item
jigc-version: 1.0.0-rc.16
status: open
date: 2026-10-03
schema-version: 1
---

# An empty add-item title answers under a third code

## Description

One condition, a title that yields no id, carries three codes across the write surface: `create.empty-title` at `jigc doc create` and `jigc doc author`, `write.unslugable-title` at the engine write path, and `schema-conformance.field-value-conformant` at `jigc doc add-item --title ""`. The last is a conformance identity about a field value, raised where the fault is an unslugable title. At the one door, `--title "!!!"` answers `write.unslugable-title` and `--title ""` answers `schema-conformance.field-value-conformant`, both at exit 1. Every cell refuses and writes nothing. A caller keying on `(code, target)` cannot recognise one condition across three spellings, which is the stable-key discipline M42 shipped, applied one door short. What is owed is one identity for the condition, or a stated reason why the empty cell is a different condition from the unslugable one. The M53 baseline surfaced it, and the M53 Settle sent it to the 1.x ledger with the charter's trigger.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. In a `single-task` task, `jigc doc add-item decisions-log#entries --title "!!!"` exits 1 with `write.unslugable-title`, and `--title ""` exits 1 with `schema-conformance.field-value-conformant` (*add-item rejected: the heading is empty*). `jigc doc create adr --title ""` exits 1 with `create.empty-title`. `git status --porcelain` is empty.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons --start single-task "probe titles") || exit; eval "$rig"
$JIGC doc add-item decisions-log#entries --title "!!!"; echo "exit $?"   # 1, write.unslugable-title
$JIGC doc add-item decisions-log#entries --title ""; echo "exit $?"
#   1, schema-conformance.field-value-conformant — the same condition, a second code, the same door
$JIGC doc create adr --title ""; echo "exit $?"                          # 1, create.empty-title, a third
git status --porcelain                                                    # empty: nothing written
```

## Resolution
