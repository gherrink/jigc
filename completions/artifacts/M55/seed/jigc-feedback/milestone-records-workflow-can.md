---
kind: inconvenience
found-in: doctype-misfits:pre-opening-plan/M5
about: jigc milestone add-task
jigc-version: 1.0.0-rc.24
status: open
date: 2026-10-29
schema-version: 1
---

# A milestone record's workflow can only name a pack workflow

## Description

`jigc milestone add-task --workflow` must name a workflow the loaded packs provide. A plan whose tasks are run by this repository's own agents and people — a builder, a main-session edit, a git step, a review, a question to the human — cannot record who runs a task through the door; the same values written by hand validate clean, so the limit is the door's, not the schema's. Row M5 of the doctype-misfits register (`completions/artifacts/M55/doctype-misfits.md`); a finding for the port's triage, never a requirement.

Re-driven on this build (`jigc 1.0.0-rc.24`, the debug binary built from `7c34a222`): **open**. `add-task <m> "U0 — give CI room" --workflow builder` is refused with a blocking `workflow-refs.unknown-workflow` naming the pack's workflows, and nothing is minted; a record whose `workflow:` lines read `builder`, `git step` or `human question` validates with no conformance finding.

## Repro

```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"
$JIGC milestone add-task before-the-first-stabilization-run "U0 — give CI room" --workflow builder
# blocking workflow-refs.unknown-workflow — nothing minted
```

## Resolution
