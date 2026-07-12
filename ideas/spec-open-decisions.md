# spec open-decisions — machine-readable open calls, resolution-gated

**Status: parked 2026-07-12.** From the RC implementation-half trial, probe 3 ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md) → Probe 3). **Frozen-v1 gated** like its sibling [spec-criterion-status](spec-criterion-status.md). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

The trial spec carried a prose "Open decisions for the builder" section — two load-bearing product calls. The builder resolved both with the human, and the resolutions landed **only in the commit body**: nothing connected "this spec ships with unresolved decisions" to "record how you resolved them" in the decisions log. The workflow even offered `doc create adr` as an option — optional, so skipped.

> If specs can carry a machine-readable open-decisions block, jigc could gate finalize on them. […] That seems like exactly what the decision log is for, and the workflow walked right past it.

Same soft-gate lesson from the changelog in the same session: *"If it's optional, I'll skip it; if it matters, gate it."*

## The shape

A repeatable `open-decisions` section on `spec` (title + statement per item), and an `implement-from-spec` finalize obligation: each open decision either resolved-with-a-recorded-home (an adr / decisions-log entry the item then points at — `resolved-by`, an edge the index can check) or explicitly carried forward. The gate is structural (presence of a resolution ref), never a judgment of the resolution — determinism boundary intact. Schema half rides the frozen-v1 version gate; the *prompting* half (the workflow demanding the recording) is pack YAML and could ship earlier against the prose section.

## Trigger

The same frozen-set version wave as [spec-criterion-status](spec-criterion-status.md) — the two are one `spec` schema-version bump.
