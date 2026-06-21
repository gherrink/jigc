# The differentiator-engaging twin pilot — clearing the value gate, reversibly

**Status:** shaped, unscheduled (parked 2026-06-21). The operational scope for the
**value gate** the [readiness assessment](../READINESS-ASSESSMENT.md) named: the
one thing that would tell us whether a productive jigc commitment is worth its
lock-in cost. Design home for the protocol is [measurement.md](../design/measurement.md)
(don't restate it); this file is the *delta* M17 left un-run + the decisions to launch it.

## Why this exists (the M17-relocated hypothesis)

M17 ran the dogfood + measurement milestone and its verdict was honest: **the one
controlled jigc-vs-static comparison tied**, and the thesis was **relocated, not
proven** — jigc's differentiating value, *if* it exists, lives "at the methodology's
native grain with a fitting domain pack, the regime the comparison never reached."
The structural reason it tied: the comparison used a `dev-task` workflow with **no
`allows-create`**, so it touched **none** of the differentiators (managed docs,
validation over them, drift/reconciliation, forward-ref integrity, supersession).

M17 explicitly named the un-run cases and why they were skipped:
- **P2 — existing-docs (lacon was the named target):** the one case where **doc↔code
  genuinely adjudicates** — "the differentiator most likely to show a measurable
  correctness win **went unexercised**." Skipped because a foreign project needs an
  **authored pack** to reach the differentiators (real work M17 did not do).
- **P3 — greenfield, human-driven:** the strongest-form, un-hollowable Half-B.

This pilot runs those cases. It is the **reversible** way to test the value
question *before* any corpus lock-in — and the readiness assessment's whole point
is that we can get this signal now without freezing formats or owning migrations.

## The shape (the delta over M17 — reuse its protocol for everything else)

- **Reuse from M17 (built + proven):** the three-arm comparison — **A** (jigc + pack)
  vs **B** (the project's existing static `CLAUDE.md`/incumbent method) vs **C** (the
  same methodology frozen flat, isolating *dynamic composition + the write channel*
  from *methodology content*); the `dogfood-record` doctype + the three measured
  facts; the **pilot gate** (4 conditions, verdict must name ≥1 **non-seeded**
  thesis observation); the honesty bounds; the two-half (automatable + live) shape.
- **The new prerequisite (what M17 punted): author a fitting domain pack** for the
  target — the doctypes + workflows that make jigc's differentiators *relevant to
  that project's real work*. This is the load-bearing scoping work. Keep it minimal:
  the pack only needs enough to engage **one** differentiator convincingly.
- **The matched task MUST engage a differentiator** (the M17 lesson) — not a bare
  code fix (that ties). Cheapest-to-richest options:
  - **supersession + forward-ref integrity** — a task that records an ADR which
    `supersedes` a prior one; finalize walks the edge index (already MVP-proven —
    the cheapest differentiator to exercise on a twin).
  - **doc↔code adjudication** — adopt an existing spec/arch-doc corpus, make a code
    change, let validation catch a doc↔code drift (the P2 differentiator; needs code
    anchors → richest signal, most setup).
  - **drift/reconciliation** — an out-of-band edit to a managed doc, detected + routed.

## Reversibility (the whole point — zero lock-in)

Everything runs on a **twin** (a throwaway copy of the real project). Records +
owner-artifacts are **exported as plain committed files** into jigc's repo under
`completions/artifacts/<pilot>/` — the real project and jigc's own repo stay
**unmanaged**. Nothing about this pilot commits a productive corpus to jigc; it is
pure measurement. (Storage is plain diff-friendly files by invariant, so export is
a copy, not a conversion.)

## Open decisions to launch (human — next session)

1. **Target + case.** P2 (existing-docs, doc↔code — richest signal, most setup;
   `lacon` was M17's named target) or P3 (greenfield human-driven — un-hollowable,
   lighter setup) — or both. *Which real project, which case first?*
2. **Domain-pack effort appetite.** Minimal (one doctype + one workflow engaging a
   single differentiator) vs a fuller pack. Minimal is the right first bet — enough
   to make the differentiator *bite*, no more.
3. **Which differentiator to exercise first.** Supersession (cheapest, MVP-proven)
   → doc↔code (richest, needs anchors) → drift. Start where the target's real work
   most naturally engages one.

## Success criterion

A pilot-gate-passing run whose verdict names **≥1 non-seeded** observation where a
differentiator **changed an outcome** jigc's static-methodology arm (C) did not get
— i.e. the first evidence that jigc **>** static, not **≈** static. That is the
signal the productive-go decision (M33 freeze + M34 corpus-migration + this) waits on.
