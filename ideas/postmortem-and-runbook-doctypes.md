# postmortem + runbook doctypes — the two thin operational-record candidates

**Status: parked 2026-07-10.** From the RC adoption trial's doctype-gap feedback, finding D3 ([trial-record](../completions/artifacts/RC-adoption/trial-record.md) → Doctype gaps). Two shaped-but-thin candidates in one file — split if either grows a driver of its own. Indexed from [VISION.md](../VISION.md) → Open questions.

## Postmortem / incident record

The trial deleted `.planning/debug/resolved/audit-chrome-econnrefused.md` — symptom, root cause, fix, from a real debugging session — because nothing held it: not an `adr` (no decision), not `research` (no forward question), not an `idea` (nothing owed). In practice this is the most re-read artifact type there is ("we've seen this error before, here's why"). Shape sketch: `{date, symptom-slot, root-cause-slot, fix-slot}` + an optional code anchor at the fix site.

## Runbook

A reproducible operational procedure (the trial's example: working curl invocations exercising deploy endpoints). Minor but a real category; nothing holds it today. Shape sketch: `{purpose-slot}` + a repeatable `steps` section — possibly just a `reference` doc with a convention, not its own doctype.

## Weighing

Both are candidates, not commitments — rank them at the doctype-completeness milestone against [finding-doctype](finding-doctype.md) and [reference-doctype](reference-doctype.md); the runbook in particular may collapse into `reference`. Neither appeared as a gap in the [self-migration coverage audit](../completions/artifacts/RC-adoption/self-migration-coverage.md) (jigc's own corpus has no incident/runbook docs), so the driver is adopter corpora, not self-hosting.

## Trigger

The post-1.0 **doctype-completeness milestone**, ranked; or a second adopter corpus hitting the same hole first.
