---
kind: bug
found-in: review:M52-per-axis/(5,DEFECT 2)
about: jigc config remove-step
jigc-version: 1.0.0-rc.16
status: open
tier: tier-2
date: 2026-10-03
schema-version: 1
---

# Config remove-step and replace-step refuse an inserted step

## Description

`jigc config remove-step` and `jigc config replace-step` refuse a step that **is** in the workflow's resolved include list. Their message names the wrong verb, and their route is one the caller has already satisfied. `resolve_fork_bytes` has three callers, and its refusal is written for only one of them. A step inserted by `jigc config insert-step` exists only in the project layer, so the pack read fails, and both sibling verbs raise *no step `X` body to fork*. Yet `start --explain` lists that step in the resolved include list one command earlier. The M53 re-reviews kept it open through `1.0.0-rc.20`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. After `insert-step`, the resolved list is `locate implement record-changelog superseded-context author-commit probe-step finalize`. Both `remove-step` and `replace-step` then exit 1 with `config.anchor-absent — no step probe-step body to fork`, and route at *name a step id present in the workflow's resolved include list*. The control, `remove-step` on a pack step (`implement`), exits 0.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
printf 'id: probe-step\ntitle: Probe\nbody: |\n  Probe.\n' > "$RIG/probe-step.yaml"
$JIGC config insert-step --workflow single-task --before finalize "$RIG/probe-step.yaml"   # exit 0
$JIGC --format json start --explain --workflow single-task   # .steps[].id carries probe-step
$JIGC config remove-step 'workflow:single-task#probe-step'   # exit 1
#   blocking · config.anchor-absent — no step `probe-step` body to fork
#     route: name a step id present in the workflow's resolved include list, then re-run
printf 'id: probe-step\ntitle: Probe2\nbody: |\n  Probe two.\n' > "$RIG/repl.yaml"
$JIGC config replace-step 'workflow:single-task#probe-step' "$RIG/repl.yaml"   # exit 1, the same
$JIGC config remove-step 'workflow:single-task#implement'    # control: exit 0
```

## Resolution
