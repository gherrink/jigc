---
kind: bug
found-in: review:M52-per-axis/(6,D-3)
about: jigc start
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Orientation preview footer misses the verb-routed carve-out

## Description

The orientation footer's `Preview:` line still states the pre-M52 rule, which `milestone-execution` falsifies. The footer says *a workflow that mints nothing has no preview — `jigc start --workflow <id>` composes it directly, and mints nothing either*. `jigc describe --workflows` carries the swept sentence. It adds that neither door reaches *a workflow whose line below says it is reached only through a verb … and both of these refuse it by name*. One claim lives on two surfaces, and M52 Increment 9 swept only one of them. The M53 re-reviews kept it open through `1.0.0-rc.19`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. Bare `jigc start` prints the unswept `Preview:` line, and `jigc describe --workflows` prints the swept one. `jigc start --workflow milestone-execution` exits 1 with `workflow.verb-routed`, and the control `--workflow router` exits 0.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC start
#   Preview: `jigc workflow <id> --preview`   — … a workflow that mints nothing has no
#     preview — `jigc start --workflow <id>` composes it directly, and mints nothing either
$JIGC describe --workflows      # … Neither reaches a workflow whose line below says it is
#                                 reached only through a verb … both of these refuse it by name.
$JIGC start --workflow milestone-execution     # exit 1, workflow.verb-routed
$JIGC start --workflow router                  # control: exit 0
```

## Resolution
