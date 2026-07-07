# form-vision research routing — detect a missing ground, route to do-research

**Status: ✅ shipped M39** (parked 2026-07-06, direction settled at parking; built as the empty-research-store `do-research` advisory on `form-vision` — route-not-merge, non-blocking). From RC greenfield trial 1, finding A5 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)); the route-vs-merge fork settled by the human 2026-07-06. Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

`form-vision` composes a vision-authoring session even when the store holds no committed research to ground it. The `grounded-in` chain is the altitude's killer feature (the trial's own words) — but it only fires if research exists *first*; a fresh project running `form-vision` directly authors an ungrounded vision with no nudge toward the research step.

## Settled at parking

**Route, never merge.** Folding research into `form-vision` would collapse the two-artifact provenance that makes grounding real (vision *cites* committed research; the citation is the value). And the routing must be **advisory, not blocking**:

- research is not always required — some visions are legitimately grounded in experience, not a research doc;
- when it is required, it may take **multiple** `do-research` rounds before the vision forms — the routing is "consider research first," not "exactly one research step then vision."

## The shape

A deterministic store query at compose: zero committed `research` docs → `form-vision`'s opening step carries an advisory line ("no committed research exists; a vision grounds in research — consider `do-research` first"), never a hard gate. This is exactly a [state-aware-compose](state-aware-compose.md) instruction (state in → instruction rendered), so the two should land aware of each other. Methodology-pack guidance change; expect no engine work and no schema change (verify `grounded-in` cardinality is already `0..*` at pickup).

## Trigger

Next milestone touching the methodology pack / design-altitude workflows — pairs with [state-aware-compose](state-aware-compose.md).
