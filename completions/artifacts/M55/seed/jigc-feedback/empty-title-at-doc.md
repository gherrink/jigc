---
kind: bug
found-in: milestone:M53-settle/(D)(b)
about: create.empty-title
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# An empty title at doc author is routed at doc create's flag

## Description

`engine::state::empty_title_finding` is parameterised on the doctype alone, so both of its sentences are literals. `jigc doc author adr` over a payload whose `title:` is `"!!!"` answers *`jigc doc create adr` needs a title that yields an id*, routed *re-run with a non-empty `--title`*. That is a verb the operator did not run and a flag the door does not take, because the title came from the payload's `title:` key. The refusal is right and nothing is written. What is false is which door it tells the operator to fix, which is why the Settle graded it tier 3. The Settle first recorded only the route; driven, the whole finding is wrong, not only its route. What is owed is the message and the route parameterised on the door, as the producer's doc comment already scopes them to `create`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. `jigc doc author adr --from-file payload.yaml` over `title: "!!!"` exits 1 with `create.empty-title`: *`jigc doc create adr` needs a title that yields an id, but the given title is empty or slugs to nothing*, routed *re-run with a non-empty `--title` (its slug becomes the doc id)*. `jigc doc author --help` mentions no `--title`.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons --start single-task "probe author") || exit; eval "$rig"
printf 'title: "!!!"\n' > payload.yaml
$JIGC doc author adr --from-file payload.yaml; echo "exit $?"
#   1, create.empty-title — `jigc doc create adr` needs a title …   <- a verb the operator did not run
#   route: re-run with a non-empty `--title` …                      <- a flag `doc author` does not take
$JIGC doc author --help | grep -c -- '--title'                     # 0: the door has no such flag
```

## Resolution
