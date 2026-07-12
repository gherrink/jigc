# flow doctype — data flows, process flows, progress flows

**Status: parked 2026-07-12.** From the human's trial scratch notes. Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

No doctype captures a *flow* — how data moves through a system, how a process runs end-to-end, how a request travels. Arch-docs carry components (nouns); flows are the edges-with-order (verbs). The trial corpus's monorepo topology and pipeline-execution chains were squeezed into arch-doc components.

**When a flow earns its own doc** (the human's criteria, 2026-07-12) — behavior with several of these traits:

- crosses multiple systems (the knowledge lives in no single component's doc),
- several steps or state transitions (ordering is the content),
- frequently changed or debugged (the doc pays rent),
- involves external services (the boundary behavior isn't in this repo's code),
- carries important error handling (the unhappy paths are the point),

…and things of that nature. The common thread: exactly the knowledge that evaporates fastest and that an arch-doc component list structurally can't hold. Notably, the trial's own worst incidents fit the profile — the audit gate (multi-container, state transitions, dead error path) and the update/backup/rollback pipeline (external CMS service, staged transitions, recovery handling) are both flows nobody could read anywhere.

## The shape

A doctype candidate for the post-1.0 doctype-completeness milestone's ranking, alongside [reference-doctype](reference-doctype.md)/[finding-doctype](finding-doctype.md)/[postmortem-and-runbook-doctypes](postmortem-and-runbook-doctypes.md). Schema instinct when a driver arrives: ordered repeatable steps, each with an `implemented-by`-style code anchor and an error-handling slot; possibly a `crosses` ref to arch-doc components — the anchor gate applied to a *sequence*, which also makes a flow doc self-invalidating when the code it traces moves (the doc↔code pattern on the doctype that's "frequently changed or debugged"). The when-to-use criteria above belong in the doctype's `when:`/description so the router can offer it at the right moment.

## Trigger

The doctype-completeness milestone, earn-with-driver as always — a workflow that authors or reads flows.
