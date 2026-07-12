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

## The invocation-output face (added 2026-07-06, RC greenfield trial A7)

The trial surfaced the same cost on the *per-invocation* axis: the agent chained many `jigc` commands in one row and the accumulated output flooded its context with unwanted/unrequired data ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md) → A7). Compose payload is one emitter; the sum of every ack, advisory, and re-printed template across a 30-command session is the other. Levers on this face: terser acks where they're still verbose, suppressing re-prints (largely [state-aware-compose](state-aware-compose.md)'s job), and possibly a minimal-output format tier below `agent`. Any budget/accounting mechanism built here should count both faces.

**2026-07-12 datum (implementation-half trial):** a single `jigc task diff` emitted **187,851 bytes** — 4× the whole trial's next-largest output ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md) → Log analysis). The fd-tee measures; nothing bounds. First concrete ceiling candidate on the invocation-output face.

## Trigger

A measurement run or real dogfood shows composed-view size is a material cost — or a large corpus makes a composed slice overflow a model's practical budget. *(The invocation-output face has a live data source now: the RC invocation logs record per-command usage — the analysis can measure output volume per session.)*
