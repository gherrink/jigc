---
kind: feedback
found-in: doctype-misfits:pre-opening-plan/M1
about: roadmap doctype
jigc-version: 1.0.0-rc.24
status: open
date: 2026-10-29
schema-version: 1
---

# A roadmap entry has no place for a step's question, estimate or safe break

## Description

A `roadmap` entry is a title, a `proves` slot and a `decomposition` slot, the decomposition being "the increments, as prose". A plan that is meant to be run in steps by a human who takes breaks needs three things per step that the entry has no place for: what the human is asked at that step (with the recommendation), the step's estimated machine hours, and whether the step ends where the work can stop. In the pre-opening plan's roadmap entry they are bold labels inside the decomposition prose, so nothing can read, count or check them. Row M1 of the doctype-misfits register (`completions/artifacts/M55/doctype-misfits.md`); a finding for the port's triage, never a requirement.

Re-driven on this build (`jigc 1.0.0-rc.24`, the debug binary built from `7c34a222`): **open**. `jigc doc schema roadmap` lists the entry's leaves as `title`, `proves` and `decomposition` and nothing else; the entry authored through `doc add-item` and two `doc set-slot` calls finalizes clean, with the questions, estimates and breaks only as prose.

## Repro

```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC doc schema roadmap      # milestones: title · proves · decomposition — no leaf for a question, an estimate or a break
```

## Resolution
