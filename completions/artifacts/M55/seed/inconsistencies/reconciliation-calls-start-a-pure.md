---
kind: code-doc
status: open
date: 2026-10-03
schema-version: 1
---

# Reconciliation calls start a pure reader

## Sides

### design/reconciliation.md:51  {#side1}

*`validate`, `start`, and read-path sweeps stay pure readers (no write side effects on a read verb).*

### design/reconciliation.md:82  {#side2}

L1's bullet, added by M55 Increment 4: *`start` and `task validate` report the absorb and write nothing*.

### crates/cli/src/start.rs  {#side3}

`jigc start` mints a task. `state::mint_task` writes `.jigc/tasks/<id>/`, with the task's base pin and its index snapshot.

## Description

`design/reconciliation.md`'s persistence rule says `start` is a pure reader, with no write side effects on a read verb. But `start` is not a read verb: it mints a task and writes the task's working area. The rule means something narrower, that a sweep's absorb is persisted only by a landed finalize, and the sentence does not say so. It bears on L1 (F4), whose cause was that `start`'s absorb is in memory only. The docs gap-detector found it.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a7a742d3`): **still open**. Line 51 is unchanged. M55 Increment 4 (`8b4f012f`) engaged it at `:82` by restating it, *`start` … write nothing*. In a `committed-singletons` rig, `jigc start --workflow park-idea "probe idea"` exits 0 and writes `.jigc/tasks/probe-idea/`, and `.jigc/state/file-state.json` stays byte-identical. Both sentences are true of the baseline and false of `start`.

## Evidence

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
shasum .jigc/state/file-state.json
$JIGC start --workflow park-idea "probe idea" > /dev/null; echo "exit $?"   # exit 0
ls .jigc/tasks                                                 # probe-idea
shasum .jigc/state/file-state.json                             # the same hash
```

## Resolution
