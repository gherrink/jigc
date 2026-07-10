# pack doctype visibility — stop offering dogfood-record to every adopter

**Status: parked 2026-07-10.** From the RC adoption trial's doctype-gap feedback, finding D5 ([trial-record](../completions/artifacts/RC-adoption/trial-record.md) → Doctype gaps). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

`dogfood-record` — a doctype for measuring jigc's own instrumented runs — ships in the methodology pack, so every project that installs it sees a doctype in `jigc describe` that only jigc's own developers will ever author. The trial agent flagged it as the one doctype that "reads as out of place." Not a doctype flaw — a **pack-composition/visibility** question.

## The shape

Candidate mechanisms, smallest first: a per-doctype `visibility:`/`internal:` schema knob that drops it from `describe` and the router while keeping it functional (a cascade-overridable delta, so jigc's own project layer re-enables it); or splitting a `jigc-dev` pack off the methodology pack (heavier — a third pack for one doctype is overbuilt today). The knob composes with the cascade's existing whole-definition shadow model; no engine parse changes.

## Trigger

The doctype-completeness milestone (it touches pack composition anyway), or the first external methodology-pack adopter complaint — whichever first.
