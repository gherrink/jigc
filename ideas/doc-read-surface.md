# Doc read surface — `jigc doc show` + render every grounded source on compose

**Status: parked 2026-07-06, unscheduled — the trial's #1 finding, strongest next-milestone candidate.** From RC greenfield trial 1, finding F1 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

The adapter's contract says *never read managed files directly* — but the interface has no read verb for committed **instances**. `jigc describe` (M11) projects *definitions* (workflows, doctypes, the cascade); nothing projects a committed doc's content. And on re-compose, only the **first** grounded research's findings render — a vision grounded in three research docs shows one. The trial got away with it because the other two were in session memory; a fresh session revising that vision must either violate the never-read-directly rule or work blind. The rule is only honest if the interface can serve every read the rule forbids.

## The shape

Both halves deterministic — read-path projection of committed store content, no LLM anywhere:

- **`jigc doc show <ref>`** — render a committed doc (or an addressed slice: `#section`, item) through the canonical parse/render path. The instance sibling of `describe`; the same addressing grammar every reference already uses.
- **Compose renders *all* grounded sources**, not just the first — the `grounded-in` chain is the killer feature (the trial's own words) exactly because the findings are in view; a cardinality-1 render of an n-cardinality edge is a plain bug-shaped gap.

Interacts with [composed-context-token-budget](composed-context-token-budget.md): rendering *all* sources raises compose payload, which is that idea's accounting/budget problem — the pair should land aware of each other.

## Trigger

Next milestone planning (the post-RC fix wave), or any flow where a fresh session must revise a committed doc through the interface — the vision-revision case the trial hit is already real.
