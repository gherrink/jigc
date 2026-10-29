---
kind: feedback
found-in: doctype-misfits:pre-opening-plan/M2
about: methodology pack doctypes
jigc-version: 1.0.0-rc.24
status: open
date: 2026-10-29
schema-version: 1
---

# No doctype holds a builder-level task spec

## Description

A plan's per-task spec — what each task writes, its red-first arms, its done-when, its gate count — has no doctype. The pre-opening plan keeps it as a plain record (`detail.md`) beside the managed roadmap entry, and the table that maps the entry's steps to the spec's tasks, which is what keeps the two from drifting, is held by hand. Row M2 of the doctype-misfits register (`completions/artifacts/M55/doctype-misfits.md`); a finding for the port's triage, never a requirement.

Re-driven on this build (`jigc 1.0.0-rc.24`, the debug binary built from `7c34a222`): **open**. The thirteen methodology schemas were read; none has a per-task block with writes, arms, a done-when and gates — the closest, `milestone-record`, carries `task-id`, `intent`, `workflow` and `status` only, all machine-maintained, as `jigc doc schema milestone-record` shows.

## Repro

```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC doc schema milestone-record   # tasks: task-id · intent · workflow · status — no spec
$JIGC doc schema roadmap            # decomposition is one prose slot
```

## Resolution
