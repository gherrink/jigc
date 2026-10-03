---
kind: bug
found-in: milestone:M53-settle/(D)(d)
about: jigc task finalize
jigc-version: 1.0.0-rc.16
status: open
date: 2026-10-03
schema-version: 1
---

# The displacement suffix walk has no ceiling

## Description

`free_displacement_path` in `crates/cli/src/task.rs`, the parking home's no-clobber walk, tries `<name>.2`, `.3`, … in a `loop` with no ceiling. So a pathological `.jigc/displaced/` tree hangs the displacement instead of refusing it. It loses nothing, because the loop's whole job is to avoid overwriting. It was read at the M53 baseline, not driven, and M53 Increment 2 left it untouched by declared bound, since that increment's subject was the removal and a ceiling is a condition on a different guard. What is owed is a ceiling, and the refusal it hands back when the ceiling is reached.

Re-driven on this build, by reading the source as the row was found: **still open**. `free_displacement_path` still counts `n` up from 2 in a bare `loop` whose only exit is a free path, and nothing bounds it. The walk itself was not driven.

## Repro

```sh
# a source read, from the jigc checkout: the walk's only exit is a free path
sed -n '/^fn free_displacement_path/,/^}/p' crates/cli/src/task.rs
#   let mut n = 2u32;
#   loop { … if std::fs::symlink_metadata(&candidate).is_err() { return candidate; } n += 1; }
```

## Resolution
