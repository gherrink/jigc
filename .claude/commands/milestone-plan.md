---
description: Plan one milestone (scope → detect gaps → settle → review → decompose) — the human-led planning loop, run from the main session, delegating recon + review to subagents.
argument-hint: <milestone-id, e.g. M3>
---

Plan milestone **$ARGUMENTS** following [milestone-planning-workflow.md](../../implementation/milestone-planning-workflow.md) — read it for the *why* of each phase; this command is its **runnable overlay** and does not restate it. Drive this from the **main session** so the human-in-the-loop phases are a conversation, not halts: delegate the autonomous recon + review to subagents, and keep **Settle** and **Decompose** inline with the human.

**1 · Scope.** Read the **$ARGUMENTS** entry in [roadmap.md](../../implementation/roadmap.md), plus [VISION.md](../../VISION.md) and [doctype-map.md](../../implementation/doctype-map.md). Restate, in your own words, what this milestone *proves*, the runnable slice it must deliver, and the worked-example flow(s) that will demonstrate it. Surface that done-picture and confirm it with the human before probing.

**2 · Detect gaps.** Spawn **four `gap-detector` subagents in parallel** (one message, four Agent calls), one per dimension — **decisions**, **docs**, **doctypes**, **capabilities** — each given the milestone id, the done-picture, and its assigned dimension. Consolidate their ranked lists into one deduped, severity-ranked gap list and present it.

**3 · Settle — the human gate.** For each **blocking** gap, surface it to the human and resolve it via the [design workflow](../../implementation/design-workflow.md) — do **not** silently pick. Ask a clear question for each genuine fork (settle-now vs. defer-to-a-later-milestone; whether a gap warrants machinery at all). **Record every settled decision in [DECISIONS.md](../../DECISIONS.md)** (dated, ≤1-line why), graduate any resolved `(D)` out of [decisions-pending.md](../../implementation/decisions-pending.md), and elaborate any doc / doctype schema this milestone now drives. A consciously deferred gap is logged, not forgotten.

**4 · Review.** Spawn **one `design-reviewer` subagent** over what *Settle* produced (the new/edited DECISIONS entries, docs, doctype schemas). Bring its findings to the human and **bake the accepted ones back into the docs *before* decomposing** — an increment inherits every flaw in the design it is cut from. The human accepts or rejects each finding. (A cross-model second opinion via `/review-codex` on the settled design is fair game here too.)

**5 · Decompose.** Cut the milestone into ordered, **risk-first** increments straight from the reviewed done-picture — each with its *Deliverable* / *Grouped scope* / *Proves*, naming the design docs it builds against, in the form the [increment workflow](../../implementation/increment-workflow.md) consumes. Keep it **minimal and linear** (each increment builds on the last; no creep into a later milestone). Write the decomposition into [roadmap.md](../../implementation/roadmap.md) as a `## Milestone N — …: decomposition` section (matching the M1/M2 format), add a dated `DECISIONS.md` entry, and commit the planning artifacts.

**Done** when the blocking gaps are settled (or consciously deferred), the design has survived the independent review, and the increments are cut and ordered in the roadmap. Hand off to the build harness:

```
Workflow({ name: "milestone-build", args: { milestone: "$ARGUMENTS", base: "<current HEAD sha>" } })
```
