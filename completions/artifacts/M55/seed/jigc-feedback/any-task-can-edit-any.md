---
kind: feedback
found-in: milestone:M55-planning/F14
about: jigc doc set-slot
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# Any task can edit any committed doc whatever its create-gate

## Description

There is no edit gate on committed docs. Any task can `add-item`, `remove-item`, `set-field` or `set-slot` on any committed doc, whatever its workflow's `allows-create`. Only `doc create` and `doc author` are gated. It is the baseline ledger's **C7**. The findings channel's append-only convention rests on it, because the engine does not enforce the convention (`design/findings-channel.md` → 1.5). An edit gate would close it for every doctype. It is not cheaper now than later and blocks nothing, so it is parked as an idea (`ideas/committed-doc-edit-gate.md`) and seeded open as a declared bound. It names a missing capability, not a wrong act, so its kind is `feedback`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. A `park-idea` task, whose create-gate allows only `idea`, sets `vision:vision#thesis` at exit 0 (*copied in for update*), and its `jigc task finalize` exits 0 with a commit holding `VISION.md` alone.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
T=$($JIGC start --workflow park-idea "park a thought" | sed -n 's/^task minted: //p')   # allows-create: idea only
printf 'A thesis rewritten by a task that may create only an idea.\n' |
  $JIGC doc set-slot vision:vision#thesis --from-file - --task $T; echo "set-slot: exit $?"   # 0
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value ideas --task $T > /dev/null
printf 'park a thought\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
printf 'why\n' | $JIGC doc set-slot commit:$T#body --from-file - --task $T > /dev/null
$JIGC task finalize $T > /dev/null 2>&1; echo "finalize: exit $?"                     # 0
git show --name-only --format=%s HEAD                                                 # VISION.md
```

## Resolution
