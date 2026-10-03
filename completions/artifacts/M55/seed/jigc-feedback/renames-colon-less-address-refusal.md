---
kind: bug
found-in: review:M52-per-axis/(5,DEFECT 3)
about: jigc rename
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Rename's colon-less address refusal carries no code

## Description

`jigc rename` has a refusal outside its own declared-complete refusal registry, and that refusal carries no code. `rename.rs`'s module doc claims *every one of those refusals is declared and disposed in one place — `RefusalKind`, which carries the finding code each raises*. Yet `parse_addr` bails with a bare `anyhow!` before any `RefusalKind` is reached. So the bare-slug form refuses with no code, no `key` and no error-code-registry entry. The module's own comment names that form as *the natural first guess*, and the two sibling doors accept it. The rc.17 re-review widened it. The same code-less bail sits at `doc show` and the `doc` write verbs under a second wording, so there are two producers. The M53 re-reviews kept it open through `1.0.0-rc.20`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. `jigc rename vision --to "New Vision"` exits 1 with *`vision` is not a `<type>:<slug>` address* and a route, but no `blocking · <code>` prefix. Its JSON arm is a bare `{"error": …}`. `jigc --format json doc show nosuchtype` gives the second wording (*malformed address `nosuchtype`: missing ':' …*), also as `{"error": …}` with no code. The control, `jigc doc show vision`, exits 0.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC rename vision --to "New Vision"                 # exit 1, no code
#   `vision` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`
#     route: run `jigc describe` for the doctype surface
$JIGC --format json rename vision --to "New Vision"   # {"error": "`vision` is not a …"}
$JIGC --format json doc show nosuchtype               # {"error": "malformed address …"}
$JIGC doc show vision                                 # control: exit 0, the bare form is accepted
```

## Resolution
