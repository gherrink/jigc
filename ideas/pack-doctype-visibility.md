# pack doctype visibility — stop offering dogfood-record to every adopter

**Status: parked 2026-07-10.** From the RC adoption trial's doctype-gap feedback, finding D5 ([trial-record](../completions/artifacts/RC-adoption/trial-record.md) → Doctype gaps). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

`dogfood-record` — a doctype for measuring jigc's own instrumented runs — ships in the methodology pack, so every project that installs it sees a doctype in `jigc describe` that only jigc's own developers will ever author. The trial agent flagged it as the one doctype that "reads as out of place." Not a doctype flaw — a **pack-composition/visibility** question.

## The shape

Candidate mechanisms, smallest first: a per-doctype `visibility:`/`internal:` schema knob that drops it from `describe` and the router while keeping it functional (a cascade-overridable delta, so jigc's own project layer re-enables it); or splitting a `jigc-dev` pack off the methodology pack (heavier — a third pack for one doctype is overbuilt today). The knob composes with the cascade's existing whole-definition shadow model; no engine parse changes.

## Trigger

The doctype-completeness milestone (it touches pack composition anyway), or the first external methodology-pack adopter complaint — whichever first.

**2026-10-02 addendum (the M55 Settle, S3 — a second tenant, and the wider direction parked here).** M55 ships a second jigc-only doctype into every adopter's methodology pack: `jigc-feedback`, findings *about* jigc ([design/findings-channel.md](../design/findings-channel.md) → 2). The Settle took the cheap half of this file's question and left the rest parked: the **workflow** is hidden — `report-jigc-feedback` is `selectable: false` + `suppressed: {reason, expires: never}`, the `record-dogfood` precedent, callable by name — while the **doctype** stays in every adopter's `jigc describe`, because doctypes have no hide knob; accepted at the Settle ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled). The namespaced id (`jigc-feedback`) keeps it from shadowing an adopter's own doctype.

*The wider direction (S3 option c) — further packs, and refactoring out of the built-in ones.* Beyond one visibility knob: other methodology packs, and moving what only jigc's own developers use (`dogfood-record`, `jigc-feedback`) out of the built-in packs into a pack of its own — this file's heavier candidate, the `jigc-dev` split, now with two tenants instead of one. **Why parked:** a third embedded pack needs composition plumbing that does not exist — every embed is owned by `crates/cli/src/pack_builtin.rs`, and `jigc setup` composes exactly `[dev ▸ methodology]` — which is new mechanism with no cheaper-now argument. **What it would cost:** the embed and a composition rule deciding where the third pack composes (jigc's own repository, and nowhere else by default); a manifest per pack; and moving a shipped doctype between packs changes its origin pack, which the per-origin-pack manifest resolution governs — a migration story, not a file move ([corpus-migration.md](../design/corpus-migration.md) → The freeze-exempt sibling); plus the describe/router goldens. **Trigger,** added to the two above: the feedback web service opening `report-jigc-feedback` to adopters ([feedback-web-service](feedback-web-service.md)), or a third jigc-only doctype — whichever comes first. Project-authored packs, the adopter-side sibling, are parked separately in [project-authored-doctype-packs](project-authored-doctype-packs.md).
