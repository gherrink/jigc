---
kind: bug
found-in: review:M52-per-axis/(6,D-1)
about: jigc start
jigc-version: 1.0.0-rc.16
status: resolved
tier: tier-2
pinned-by: validate_previews_posture::orientation_reports_every_posture_finalize_refuses_without_refusing
date: 2026-10-03
schema-version: 1
---

# Orientation omits the repository posture that blocks finalize

## Description

`jigc start` orientation reported a live task's findings without the repository posture that blocks every door it then routes to. During a `git bisect`, `jigc start` read *findings: 2 blocking, 1 advisory* on the text and JSON arms alike, with `repo.operation-in-progress` absent. Its own `Run: jigc task validate <id>` line promises a preview of *the repository posture finalize refuses under*, and `task validate` and `task finalize` both exit 1 with that posture. This contradicted the M50 design of record: orientation's findings come from the task-scope sweep, *so orientation and its own `task validate` route cannot diverge*.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC start --workflow single-task "posture parity"
git bisect start
$JIGC start                                   # exit 0; findings: 3 blocking, 1 advisory
#   blocking · repo.operation-in-progress — a bisect is in progress — …
$JIGC --format json start                     # tasks[].findings[].code carries
#   repo.operation-in-progress
$JIGC task validate posture-parity            # exit 1, the same code
```

## Resolution

Fixed in M53's pre-v1 usability batch (DECISIONS.md → *2026-09-23 — the pre-v1 usability batch: six surface rows*). Orientation now asks the posture family's one producer, with the `task finalize` row's own exemptions, so the report and the refusal are the same bytes under the same key. The breach rides each row's existing `findings` array, so no key was added and `SCHEMA_VERSION` did not move.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`). During a `git bisect`, `jigc start` exits 0 and reports *findings: 3 blocking, 1 advisory*, the first of them `repo.operation-in-progress — a bisect is in progress`. The JSON arm carries `repo.operation-in-progress` among the task's finding codes, and `jigc task validate posture-parity` exits 1 with the same code. The pinning test drives every overlayable git state.
