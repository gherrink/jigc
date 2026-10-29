---
kind: feedback
found-in: doctype-misfits:pre-opening-plan/M3
about: roadmap doctype
jigc-version: 1.0.0-rc.24
status: open
date: 2026-10-29
schema-version: 1
---

# A roadmap decomposition is one level, and this plan is two

## Description

The pre-opening plan is sixteen steps with tasks under each; the roadmap entry's `decomposition` slot holds one level of prose, so the step → task structure is prose and a hand-held table. The recorded trigger for the roadmap's increment tier asks for a **second** independent driver that is not this repository's own planning convenience (`design/methodology-docs.md` → *The one-level bound (roadmap)*), so **this row alone does not fire it**. Row M3 of the doctype-misfits register (`completions/artifacts/M55/doctype-misfits.md`); a finding for the port's triage, never a requirement.

Re-driven on this build (`jigc 1.0.0-rc.24`, the debug binary built from `7c34a222`): **open**. `jigc doc schema roadmap` shows `milestones` as the one repeatable, its block `title` · `proves` · `decomposition`, with no nested repeatable.

## Repro

```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC doc schema roadmap      # one repeatable, no nested level
```

## Resolution
