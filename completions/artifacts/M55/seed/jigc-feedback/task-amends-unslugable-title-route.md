---
kind: inconvenience
found-in: review:M53-per-axis-rc20/(3,F-C)
about: write.unslugable-title
jigc-version: 1.0.0-rc.20
status: resolved
tier: tier-3
pinned-by: work_unit_id_axis::a_mint_door_whose_title_is_optional_routes_at_the_exit_that_omits_it
date: 2026-10-03
schema-version: 1
---

# Task amend's unslugable-title route omits the door's own amend-sha exit

## Description

`jigc task amend ""` refused with `write.unslugable-title`, routed *re-run with a title carrying ASCII letters or digits — the task id is slugged from it*. The route never named the one exit only this door has: `jigc task amend` with no intent mints `amend-<sha7>`, as its own help says. The route is shared across `MINT_DOORS` and is right at the five members with no fallback. F-10 added a sixth member whose exits are wider, and the shared route was not re-derived for it. Nothing was minted. It was inside F-10's own new code.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC task amend ""; echo "exit $?"          # 1, write.unslugable-title; the route names the bare door
$JIGC task list                              # no active tasks: nothing minted
$JIGC task amend > /dev/null; echo "exit $?" # 0, the exit the route names
$JIGC task list                              # amend-<sha7>
$JIGC milestone create ""; echo "exit $?"    # 1, the control: no fallback there, and the route says so
```

## Resolution

Fixed in M53's last batch (`8a40a5e2`), stamped `1.0.0-rc.21`: the mint refusal names the exit only this door has.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). `jigc task amend ""` exits 1 with `write.unslugable-title`, routed *… or omit the title — `jigc task amend` names the task after the commit it rewrites (`amend-<sha7>`)*, and `jigc task list` reports no active tasks. `jigc task amend` with no intent then exits 0 and mints `amend-<sha7>`. The control, `jigc milestone create ""`, routes at the title alone, which is right there.
