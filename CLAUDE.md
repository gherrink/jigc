# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project state

Pre-implementation. The repository currently contains **only `VISION.md`** — the design thesis. There is no code, build system, test suite, or chosen language/runtime yet. `VISION.md` is the authoritative source; read it before doing substantive work. The product name is undecided — the CLI is referred to as `tool` as a placeholder throughout.

When implementation begins, update this file with the actual build/lint/test commands.

## How we work together

This is a pre-implementation **design collaboration**. Maurice develops the idea; Claude is a constructive-critical partner, not a code monkey. The default output is **discussion, not artifacts** — do not write files until told to.

**The loop:**
0. **Scope** — Maurice says roughly what's next. Before developing, restate in one line *what* we're deciding and *at what depth*; if ambiguous, wait for a nod.
1. **Develop & present in chat** — options, tradeoffs, a recommendation with reasoning. One decision at a time; never a batched "whole design" dump.
2. **Discuss & iterate** — push back, name risks, flag honest boundaries. Constructive-and-critical is the standing stance, not a per-request ask.
3. **Converge** — write **only** on an explicit "write it." Never infer convergence from enthusiasm.
4. **Write** — into the agreed home at the agreed granularity.
5. **Review, then commit** — Maurice reviews the written file; commit (conventional message, one logical change) only after he's satisfied.

**Conventions:**
- **One file, one purpose — route by home.** Thesis/framing → `VISION.md`; working truth & how-we-work → this file; the *why* of a decision → `DECISIONS.md`; architecture detail → a part-doc in `design/`. **Cross-reference, never restate** — if two docs state the same fact, one is wrong. Shared content earns its own part-doc (the document model, applied to us).
- **Reading order for `design/`.** VISION → `structural-grammar` → `document-type-schema` → `write-commands` → `storage` → `workflow-dialect` → `validation` → `overrides` → `bootstrap` → `assistant-adapter`. The first two are the foundation the rest build on; read `DECISIONS.md` alongside for the *why*.
- **No number prefixes** on part-docs (descriptive names only) — principle #2; we don't grow our own `01-`/`02.5-` disease.
- **Flag the unsettled.** Mark illustrative notation as illustrative; list open questions explicitly rather than implying resolution.
- **Decisions log.** Record every decision in `DECISIONS.md` (dated, ≤1 line of why) as it's made — the *when & why*, not the architecture (`VISION.md` + this file hold current truth).
- **Trivial lane.** Small clarifications skip the loop — don't ceremony-tax a typo.

## What this project is

A **context compiler for coding agents**. A deterministic CLI assembles exactly the instructions and document slices an agent needs for a specific task, just-in-time, and is the sole channel through which the agent reads and writes managed project documents (specs, ADRs, PRDs, commit messages, architecture docs). It replaces static rules files (`CLAUDE.md`, Cursor rules), GSD-style skeleton generation, and Spec-Kit-style workflows.

## The one load-bearing idea

**Take every *structural* operation away from the LLM and give it to the CLI. Leave the LLM only the prose.** Every architectural decision descends from this. The determinism boundary is a hard contract:

| Deterministic — owned by the CLI | Non-deterministic — authored by the LLM |
|---|---|
| Which documents exist and where | The prose inside a slot |
| Document structure, sections, ordering | A spec's actual wording |
| Cross-references and their integrity | |
| Workflow composition + placeholder resolution | |
| Placement of every write; validation results | |

"Reproducible" applies to **structure**, never to LLM prose. Do not chase reproducibility of content.

## Architectural invariants (do not violate without revising VISION.md)

- **The CLI core makes no LLM calls.** Composition is deterministic: same *resolved cascade* in → same workflow out. (The *coding agent* may use the CLI to draft a merge proposal — that is the agent acting, not the CLI.)
- **The LLM writes only through the CLI.** It hands content for a *named slot*; the CLI owns placement, cross-ref wiring, versioning, and commit. The LLM places nothing, so it can misplace nothing.
- **Stable opaque IDs, never positions.** Phases/docs/steps/slots have stable IDs; ordering lives in a separate ordered list. No renumbering; cross-references point at IDs and survive reorder.
- **Engine vs CLI vs domain pack are separate.** The *engine* (config resolution, doc registry, workflow composition, validation, state) is frontend-neutral and ships **empty**. The *CLI* is the first frontend (an MCP adapter could come later). A *domain pack* supplies the bottom config layer (doc types, workflows, steps, defaults, probes). **Development is the first and only pack** — keep domain content out of the engine.
- **Config cascades: project > team > pack-default.** (`team` = a cross-project layer, not per-developer.) All customization is a recorded **delta** against a known base version (never an untracked fork), so upgrades re-apply deltas and report clean/conflict/orphaned.
- **Slots vs placeholders are opposites and must stay syntactically distinct.** *Slots* (`<<slot>>`) are LLM-filled on the write path. *Placeholders* (`{{…}}`) are CLI-filled on the read path (command refs, data values, includes).
- **Storage is plain, human-readable, diff-friendly files.** Humans review and edit through git regardless of the CLI; out-of-band edits are **detected and reconciled, never forbidden**.
- **Writes are transactional.** Integrity holds at a `finalize`/commit boundary, not on every write (else bootstrapping deadlocks).
- **Concurrency is one bounded primitive only.** `fan-out`/`join` steps; each sub-agent writes to an isolated working area keyed by task ID; the CLI merges at a join ordered **by task ID, not completion order**. No DAGs, conditionals, cross-agent messaging, or runtime. Sub-agents coordinate through the CLI (blackboard pattern), never directly — messages carry only `task_id` + status; state is always re-derived from the CLI.

## MVP scope

First loop to build end-to-end: **`single task execution`** (discover → compose → execute → validate) — the cheapest proof the core loop beats a plain `CLAUDE.md`. Scope calls (rationale in [DECISIONS.md](DECISIONS.md), 2026-05-25):

- **Spec-less.** The MVP `single-task` does **not** read a SPEC (no `task.spec`, no `spec` schema ships); the "what" comes from the human `intent` + the codebase. Adding spec support later is a trivial pack addition.
- **Front door, no router.** `tool start` (the state-aware entry — mint task, resolve the default workflow, orient) is built; the **router** selection-guidance workflow is not. The "no-workflow-specified" default is a cascade knob set to `single-task`; it flips to the router when ≥2 work-workflows exist.
- **The deliverable is `engine` + `cli` + a minimal *embedded* dev pack + the Claude Code profile** — not just the engine (which ships empty by invariant). MVP pack: the **`commit`** doc-type only, the **`single-task`** workflow + its steps, the cascade pack-default config, and **zero probes** (the two MVP checks are engine-native code).
- **`finalize` is the commit boundary.** It validates, **renders the commit doc into the git commit message** (the commit type's sink is the VCS message, not a repo file), and `git`-commits the task's code changes + promoted managed docs as one commit. The CLI commits code too (the agent authors it directly; committing is the CLI's job — "CLI orchestrates, git executes").

Validation is in the MVP but **only the framework plus the two engine-native checks** that need no pack content or pre-existing docs: `workflow ↔ references` and `file ↔ CLI-state`. Pack-provided `doc ↔ code` probes come *after* the doc-creation flows exist (avoids a bootstrap circularity).

Implementation foundations (language/runtime, parsing, module layout) live in [implementation/](implementation/).

## Non-goals

Not a workflow engine (no DAGs/conditionals/runtime), not an auto-doc-writer (detects and routes drift; never auto-authors prose), not one-size-fits-all, not a second domain yet (engine/pack boundary is internal discipline, not a public API), not an LLM wrapper.
