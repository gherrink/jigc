---
kind: inconvenience
found-in: doctype-misfits:pre-opening-plan/M4
about: milestone-record doctype
jigc-version: 1.0.0-rc.24
status: open
date: 2026-10-29
schema-version: 1
---

# A milestone record cannot say a task has not started

## Description

A milestone record's task `status` is `active`, `joined` or `discarded`, and `jigc milestone add-task` writes `active` the moment a task is added, so a planned task that has not started — or one that waits for the human — reads as in progress. An empty value is refused. Leaving the leaf out validates clean, which is what the pre-opening plan's `progress.md` does, but the door never writes that shape. Row M4 of the doctype-misfits register (`completions/artifacts/M55/doctype-misfits.md`); a finding for the port's triage, never a requirement.

Re-driven on this build (`jigc 1.0.0-rc.24`, the debug binary built from `7c34a222`): **open**. In a fresh rig, `milestone add-task` wrote `status: active`; the record committed with `status:` empty draws a blocking `schema-conformance.field-value-conformant` (`"" is not a member of enum "status"`) from `jigc validate`; committed with the line removed, it draws no conformance finding.

## Repro

```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"
m=before-the-first-stabilization-run
$JIGC milestone add-task $m "D0 — write down the rulings"
grep status docs/milestone-records/$m.md        # status: active, for a task nobody started
perl -pi -e 's/^- status: active$/- status:/' docs/milestone-records/$m.md   # the one task's status, emptied
git commit -qam "empty status"
$JIGC validate        # blocking schema-conformance.field-value-conformant
```

## Resolution
