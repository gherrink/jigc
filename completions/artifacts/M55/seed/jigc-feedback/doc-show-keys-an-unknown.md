---
kind: bug
found-in: review:M53-per-axis-rc17/(5,DEFECT A)
about: store.unknown-type
jigc-version: 1.0.0-rc.17
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Doc show keys an unknown doctype on the doc address, not the doctype

## Description

`design/command-output-contract.md` fixes `store.unknown-type`'s target as the bare doctype id, and cites `doc show`'s own read path as the precedent: *deliberately not URI-shaped — a bare id cannot be mistaken for a doc address a driver could `doc show`*. Yet `jigc --format json doc show 'nosuchtype:x'` keys the finding `{"code":"store.unknown-type","target":"nosuchtype:x"}`, the doc-address shape that sentence forbids. Its sibling producers of the same code key it bare: `doc schema`, `doc set-field`, `doc set-slot`, `doc add-item`, `doc remove-item`, `doc retitle-item`, `doc rename`, `rename`, `relocate` and `migrate`. `(code, target)` is the contract's stable per-instance identity, so an acknowledge ledger sees two keys for one condition. The door refuses at exit 1 and writes nothing. The rc.19 and rc.20 re-reviews kept it open.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. `doc show 'nosuchtype:x'` exits 1 keyed `{"code":"store.unknown-type","target":"nosuchtype:x"}`. `doc schema nosuchtype` exits 1 keyed `{"code":"store.unknown-type","target":"nosuchtype"}`.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC --format json doc show 'nosuchtype:x' 2> show.json; echo "exit $?"   # 1, the envelope on stderr
grep -A2 '"key"' show.json        # "target": "nosuchtype:x"   <- the doc address
$JIGC --format json doc schema nosuchtype 2> schema.json; echo "exit $?"  # 1
grep -A2 '"key"' schema.json      # "target": "nosuchtype"     <- bare, as the contract fixes
```

## Resolution
