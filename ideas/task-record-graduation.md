# Task-record graduation — should a standalone in-flight task be team-visible too?

**Status: parked 2026-07-06, unscheduled — split off M39's team-ready-state scope at Settle.** Sibling of [team-ready-state-externalization](team-ready-state-externalization.md); indexed from [VISION.md](../VISION.md) → Open questions.

## The question

M39 graduates the **milestone** record to a committed managed doctype (a teammate can see and continue in-flight *milestone* work). It deliberately leaves a **standalone (non-milestone) single-task**'s state (`intent`, `status`, roles) as gitignored `.jigc/tasks/<id>/` WIP. The open question: does a standalone in-flight task *also* warrant a committed, team-visible record?

The M39 call was **milestone-only**, on two grounds: a lone single-task's in-flight window is short (it finalizes into a commit quickly), and its durable value is **already captured elsewhere that brings value** — the finalize-rendered commit, any created ADR/decisions-log entry, the changelog. A teammate rarely "continues" a lone task; the milestone is the coordination unit picked up.

## What would de-park it

Evidence that a standalone task's *in-flight* state holds value **not already captured** in the committed outputs — concretely, when the task record carries information valuable to **follow-up tasks** (accumulated context, a partial-work handoff, a rationale the commit doesn't hold). If that value is real and measurable (the same "structured meta documents make agents measurably more efficient" signal that grounded the milestone-record decision), a `task-record` doctype earns its schema — reusing the same **set-on-transition machine-maintained** shape the milestone record establishes (so it is a cheap second consumer, not new engine work).

Bound: this is *state legibility*, not the `commit`/decisions-log authoring path that already captures a task's durable prose — it must not duplicate what those already hold.

## Trigger

A dogfood/adoption run shows a teammate (or a follow-up agent session) needed in-flight standalone-task state that no committed artifact carried — **or** the milestone-record doctype ships and a driver wants the same shape for lone tasks. Validate value before building (the human's explicit call: "keep it as an idea we need to validate/test if it brings value").
