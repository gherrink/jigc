---
kind: bug
found-in: review:M52-per-axis/(5,DEFECT 4)
about: store.no-such-leaf
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Doc show calls a declared empty optional leaf nonexistent

## Description

`jigc doc show` blocks a **declared but unpopulated** optional leaf with `store.no-such-leaf`. `design/doc-read-surface.md` scopes that block to an **undeclared** leaf only. `jigc doc schema` advertises the address with `required: false`, and the read surface then says the leaf does not exist. `resolve_section_leaf` blocks on a declared, optional, absent field, with a message asserting that the leaf does not exist. The M53 re-reviews kept it open through `1.0.0-rc.20`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. `jigc doc schema vision --format json` lists the field `grounded-in` (`required: false`, `set-field: vision:<slug>#meta/grounded-in`). `jigc doc show 'vision:vision#meta/grounded-in'` exits 1 with *store.no-such-leaf — `vision:vision#meta/grounded-in` names no leaf `grounded-in` in section `meta`*. The control `#meta/schema-version` exits 0.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC doc schema vision --format json        # fields[]: grounded-in, required: false,
#                                              set-field: vision:<slug>#meta/grounded-in
$JIGC doc show 'vision:vision#meta/grounded-in'      # exit 1
#   blocking · store.no-such-leaf — `vision:vision#meta/grounded-in` names no leaf
#     `grounded-in` in section `meta`
$JIGC doc show 'vision:vision#meta/schema-version'   # control: exit 0
```

## Resolution
