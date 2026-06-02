# Milestone planning workflow (scope → detect gaps → settle → review → decompose)

The loop for **opening a milestone** — the *planning* half of the milestone lifecycle, the bookend to [milestone completion](milestone-completion-workflow.md). Before a milestone's increments can be built, the milestone is **scoped against the full-product roadmap, adversarially checked for gaps, those gaps settled, and the milestone cut into ordered increments**. This doc owns only that opening; building the increments is the [increment workflow](increment-workflow.md), and auditing the finished whole is [milestone completion](milestone-completion-workflow.md).

It completes the set: [design](design-workflow.md) governs a *decision*, [dev](dev-workflow.md) a *task*, [increment](increment-workflow.md) an *increment*, **this one the opening of a milestone**, and [milestone completion](milestone-completion-workflow.md) its close — the `milestone > increment > task` hierarchy ([structural-grammar.md](../design/structural-grammar.md) → Work-units), bracketed end to end. Running it by hand now is the dogfood for the product **`milestone-planning`** workflow we will eventually compose.

## Where this sits: the product-build loop

The whole product is built on one loop, run once at the top and then repeated per milestone:

> **once:** full-product [roadmap](roadmap.md) (milestones only) → light doctype/relation map
> **per milestone:** *this workflow* (plan) → [increment workflow](increment-workflow.md) × N (build) → [milestone completion](milestone-completion-workflow.md) (ship) → repeat

The discipline that makes the repeat honest: **plan increments for the *next* milestone only**, never all milestones up front — the same rule the roadmap applies to tasks (`roadmap.md` → "tasks are cut per-increment when that increment is picked up, so the list stays honest against what the prior increments actually produced"), lifted one level. A milestone's real shape is only knowable against the state the prior milestone actually left behind.

## Why detect gaps first (planning isn't just decomposition)

Decomposing a milestone into increments assumes you know *what* it builds. Often you don't yet — the milestone needs a design decision that's still open, a doc that doesn't exist or has gone stale, or a doctype whose schema no prior workflow has driven. Cutting increments over those holes produces a plan that halts mid-run (the [increment workflow](increment-workflow.md) → *Precondition: settle the gates first* is where an unsettled gate becomes a halt). So planning runs an explicit **forward-looking** pass — symmetric to completion's backward-looking audit — that finds the holes *before* they become halts.

This is also where the **doctype contract gets set at the right moment**: a doctype's fields, slots, and cross-references are determined by the workflow that creates it and the one that reads it. The milestone that ships those workflows is exactly when its doctypes earn their schema — not before (defining a schema with no driving workflow is generality for a single-use thing, [PRINCIPLES](../PRINCIPLES.md) → Keep it minimal).

## The loop (one milestone)

1. **Scope** — restate, from the full-product [roadmap](roadmap.md), what this milestone *proves* and its risk-first place in the sequence. Establish the done-picture: the runnable slice it must deliver and the worked-example flow(s) that will demonstrate it. This is the bar everything below is measured against.

2. **Detect gaps** — a forward-looking, **adversarial** pass over the milestone's done-picture against the assembled product: what is *missing or unfit* to build it cleanly. Categories:
   - **Decisions** — the forward look in [decisions-pending.md](decisions-pending.md): each `(D)` this milestone forces, plus any cross-cutting one due before it.
   - **Docs** — design part-docs the milestone needs that don't exist, or that drifted as earlier milestones changed reality.
   - **Doctypes** — types this milestone's workflows will create or read, whose schema is not yet defined (consult the [doctype map](doctype-map.md) for what's coming and how it connects).
   - **Capabilities & open questions** — engine/CLI surface the milestone assumes, and any part-doc `## Open questions` that block planning.

   Output: a ranked gap list, each gap concrete enough to act on.

3. **Settle** — *the human-in-the-loop step.* For each blocking gap, resolve it through the right loop and **record it**: decisions via the [design workflow](design-workflow.md) → [DECISIONS.md](../DECISIONS.md) (graduating the `(D)` out of the pending list); docs/doctype schemas elaborated via the [design workflow](design-workflow.md) where this milestone's workflows now drive them. **Surface genuine scope and judgment calls** rather than silently deciding them — must-settle-now vs. defer-to-a-later-milestone; whether a gap warrants building machinery at all. The human owns this gate. A gap consciously deferred is logged, not forgotten.

4. **Review** — an **independent** pass over what *settle* produced. A separate agent that **did not author** the decisions, docs, or doctype schemas reads them adversarially for coherence, gaps the first pass missed, over- or under-design, and conflicts with the locked invariants ([CLAUDE.md](../CLAUDE.md), [VISION.md](../VISION.md)) — a cross-model second opinion is fair game. Its findings are **baked back into the docs** through the [design workflow](design-workflow.md) *before* anything is decomposed: a settled doc that survives an adversarial read is a sound basis for increments; an unreviewed one propagates its flaws into every increment cut from it. The human owns accepting or rejecting each finding — the same gate as *settle*.

5. **Decompose** — cut the milestone into ordered, **risk-first** increments straight from the (now reviewed) scoped done-picture, each carrying its *Deliverable* / *Grouped scope* / *Proves* and naming the design docs it builds against — the form the [increment workflow](increment-workflow.md) consumes. Keep the cut **minimal**: the smallest sequence that delivers the milestone, no creep into a later one. The spine stays linear — each increment builds on the last. Record the increments in the [roadmap](roadmap.md).

The milestone is **planned** when its blocking gaps are settled (or consciously deferred), the decisions/docs/doctypes it needs are in place and have survived an independent review, and its increments are cut and ordered in the roadmap — ready for the [increment workflow](increment-workflow.md) to build the first one.

## Orchestration

Run as the **`/milestone-plan <id>`** command — the runnable overlay of this loop. Unlike the build loops (the `milestone-build` *workflow*, which runs autonomously and halts on the rare fork), planning is **human-led**, so it is driven from the **main session** as a conversation: the autonomous bursts delegate to subagents (`gap-detector` ×4 in parallel for *Detect gaps*; `design-reviewer` for *Review*), while **Settle** and **Decompose** stay inline with the human. A background workflow is the wrong vehicle here — its center of gravity is settling forks, which is irreducibly the human's. The vehicle matches the work: a *workflow* for autonomous loops, a *command* for human-led planning.

## Why this shape

- **Gaps before increments.** Decomposing over an unsettled decision or a missing doc produces a plan that halts mid-build. Finding the holes first is what lets the increment loop run unattended — a settled gate is implemented, an undiscovered one is a stop.
- **Reuse is a claim, not a fact.** The gap pass's most dangerous miss is "this capability already exists, the milestone just reuses it" — reasoned by analogy from a *proven* path to the milestone's *new shape*. The operation usually does exist; the gap is the shape it's applied to (a repeatable section where the proven path read a slot, a second workflow where one was hardcoded). Such a claim is **unverified until the new shape is exercised** — a spike against the real binary or a read of the path's code confirming it branches for the new shape — and an unverified reuse claim is a blocking gap, not a covered one. (M3 lost three build halts to analogy-reasoned reuse; the gap-detectors now spike instead.)
- **The human owns settling.** Which gaps block now vs. later, and whether a gap deserves machinery at all, are decisions, not mechanics — they surface, never get silently picked ([CLAUDE.md](../CLAUDE.md) → design decisions are surfaced to the human).
- **Review before decompose, not after.** The increment plan inherits every flaw in the docs it's cut from, so an independent, adversarial read of the settled design — folded back *before* decomposition — is far cheaper than discovering the flaw mid-build. The author can't grade its own design, the same reason completion's audit and the increment validator are independent.
- **Next milestone only.** Planning all milestones up front plans against state that doesn't exist yet. The roadmap holds the milestone *spine*; this workflow turns exactly one milestone into increments, when it's picked up.
- **It is the design and increment workflows' customer.** Settling delegates to the [design workflow](design-workflow.md); decomposition feeds the [increment workflow](increment-workflow.md). This doc owns only the scope, the gap pass, and the cut around them. Cross-reference, never restate.
