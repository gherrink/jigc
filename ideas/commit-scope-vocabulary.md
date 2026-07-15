# commit scope vocabulary — offer the project's observed `#scope` values at authoring

**Status: parked 2026-07-15 (thin).** From the lacon trial, task-2 feedback ([trial-record](../completions/artifacts/RC-lacon/trial-record.md) → Task 2). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

jigc mints the `#scope` field but the agent had to `git log | grep` to learn the project's scope convention. Structurally deeper than it looks ([findings-verification](../completions/artifacts/RC-lacon/findings-verification.md) → B16): the commit doctype is *transient* (sink = the git message, never persisted), so there is **no managed store of past commit docs to mine** — the vocabulary lives only in git history, which no jigc verb reads.

## The shape

Two candidates: (a) a **git-log-derived hint** at authoring (deterministic parse of prior conventional-commit scopes — a read of git history, a surface jigc already shells out to); (b) a **declared project-layer scope list** — a cascade knob, which makes this a natural datum for [cascade-profile-knobs](cascade-profile-knobs.md) (declared project facts fanning out) rather than its own feature. Lean (b): declared beats mined (the mined set includes retired conventions — the trial repo's own v1 phase-number scopes are exactly the stale kind), and a knob is cheap. Either way the enum sibling is already solved (`#type` is a schema enum, discoverable via `doc schema commit`).

## Trigger

A second session guessing at scope conventions, or the cascade-profile-knobs de-park (fold this in as one of its knobs).
