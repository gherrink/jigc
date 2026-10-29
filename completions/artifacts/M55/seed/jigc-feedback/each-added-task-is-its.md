---
kind: inconvenience
found-in: doctype-misfits:pre-opening-plan/M7
about: jigc milestone add-task
jigc-version: 1.0.0-rc.24
status: open
date: 2026-10-29
schema-version: 1
---

# Each added task is its own commit

## Description

`jigc milestone add-task` commits the milestone record once per task. Recording a plan's 86 tasks through the door made 86 commits, one per task, and there is no way to add a task list in one write. Row M7 of the doctype-misfits register (`completions/artifacts/M55/doctype-misfits.md`); a finding for the port's triage, never a requirement.

Re-driven on this build (`jigc 1.0.0-rc.24`, the debug binary built from `7c34a222`): **open**. In a fresh rig, two `add-task` calls moved `HEAD` by two commits, each acked `record commit: <sha>`.

## Repro

```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"
before=$(git rev-list --count HEAD)
$JIGC milestone add-task before-the-first-stabilization-run "D0 — write down the rulings"
$JIGC milestone add-task before-the-first-stabilization-run "P0 — generate the overview"
echo $(( $(git rev-list --count HEAD) - before ))     # 2
```

## Resolution
