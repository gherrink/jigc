# Composed-context token budget — measure what the compiler emits

**Status: parked 2026-07-02, unscheduled.** From the KB/research ideas harvest (both harvest agents converged on this independently). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

jigc's thesis is composing *exactly* the slices a task needs, just-in-time — progressive disclosure applied to managed docs. But the compiler is blind to the size of what it emits: no measurement of a composed view's token footprint, no ceiling, no verbosity tier. The research measures context as *the* binding cost (context rot; every loaded token is paid every turn; a `response_format` knob swung a single response 206→72 tokens) — research/03 §1/§7, research/04 §3, research/08 §4–5 of the claude-project-structure corpus.

## The shape

All deterministic, no LLM judgment:

- **Accounting** — compose reports its own token/size footprint (natural home near `describe`/introspection).
- **Budget knob** — a cascade-set ceiling; exceeding it warns (or blocks, severity-knobbed).
- **Composition tier** — a concise vs. detailed knob for *which* slot slices / cross-ref expansions inline vs. reference — one more structural decision the CLI already owns.

Turns "context compiler" from a metaphor into a measurable property. The read-path sibling of [cost-of-enforcement](cost-of-enforcement.md) (that's the block-loop dollar tax; this is the compose-path payload size).

## Trigger

A measurement run or real dogfood shows composed-view size is a material cost — or a large corpus makes a composed slice overflow a model's practical budget.
