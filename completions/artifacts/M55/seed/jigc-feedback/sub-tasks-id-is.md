---
kind: inconvenience
found-in: doctype-misfits:pre-opening-plan/M6
about: jigc milestone add-task
jigc-version: 1.0.0-rc.24
status: open
date: 2026-10-29
schema-version: 1
---

# A sub-task's id is cut from its intent

## Description

A sub-task's id is the first few words of its intent, slugged, so a plan's own short task code becomes the id only when the intent opens with it, and the id is not chosen. The milestone's id and a roadmap entry's id are cut from their titles the same way and lose a title's last words. Row M6 of the doctype-misfits register (`completions/artifacts/M55/doctype-misfits.md`); a finding for the port's triage, never a requirement.

Re-driven on this build (`jigc 1.0.0-rc.24`, the debug binary built from `7c34a222`): **open**. `add-task` with the intent `D0 — write down the rulings, the owed rows and the tool candidates` minted `d0-write-down-the-rulings`; `milestone create "Before the first stabilization run opens"` minted `before-the-first-stabilization-run`, and `doc add-item roadmap:roadmap#milestones --title` with the same title minted the same id.

## Repro

```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"       # milestone:before-the-first-stabilization-run
$JIGC milestone add-task before-the-first-stabilization-run "D0 — write down the rulings, the owed rows and the tool candidates"
# added task:d0-write-down-the-rulings
```

## Resolution
