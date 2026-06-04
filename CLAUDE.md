# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project state

Implementation has begun. Design lives in `VISION.md` (thesis), `DECISIONS.md` (running log of *what* and *why*), the `design/` part-docs, and `implementation/` (build decisions, the [roadmap](implementation/roadmap.md), and the [milestone-planning](implementation/milestone-planning-workflow.md) / [milestone-completion](implementation/milestone-completion-workflow.md) / [increment](implementation/increment-workflow.md) / [dev](implementation/dev-workflow.md) / [design](implementation/design-workflow.md) workflows). Core language is **Rust** ([implementation/language-runtime.md](implementation/language-runtime.md)); the code is a two-crate cargo workspace (`crates/engine` + `crates/cli`, binary `jigc`). The product name is undecided — the CLI is referred to as `jigc` as a placeholder throughout.

**Build / lint / test:** `cargo build` · `cargo test` · `cargo clippy --all-targets -- -D warnings` · `cargo fmt --check` (the dev-workflow gate).

**Quickstart:** [QUICKSTART.md](QUICKSTART.md) walks the MVP loop on a real machine — install → `jigc setup` → `jigc start "<intent>"` → `jigc task finalize <id>`.

## How we work together

Two operating modes, each with its own loop:

- **Design decisions & docs** → the [design workflow](implementation/design-workflow.md) (discuss → converge → write; default output is discussion).
- **Code** → the [dev workflow](implementation/dev-workflow.md) (test-first: scope → red → green → refactor → gate → commit).

The conventions below apply across both.

**Conventions:**
- **One file, one purpose — route by home.** Thesis/framing → `VISION.md`; working truth & how-we-work → this file; the *why* of a decision → `DECISIONS.md`; architecture detail → a part-doc in `design/`; a shaped-but-unscheduled idea → one descriptive file in `ideas/`. **Cross-reference, never restate** — if two docs state the same fact, one is wrong. Shared content earns its own part-doc (the document model, applied to us).
- **Parked ideas → `ideas/`.** A direction too detailed for a one-line `DECISIONS.md` entry but too immature for a `design/` part-doc parks as one descriptive file in `ideas/` — indexed from `VISION.md` → Open questions, with a terse `DECISIONS.md` pointer when the direction is locked. Promote into `design/` (and the reading order) when de-parked.
- **Reading order for `design/`.** VISION → `structural-grammar` → `document-type-schema` → `write-commands` → `overrides` → `storage` → `workflow-dialect` → `command-catalog` → `validation` → `reconciliation` → `finalize` → `bootstrap` → `assistant-adapter` → `worked-examples`. The first two are the foundation; `overrides` sits before `storage`/`workflow-dialect` so the cascade vocabulary (`project > team > pack-default`, *delta*, *config family*) is in hand before those docs reach for it; `command-catalog` sits after `workflow-dialect` (whose command-ref placeholders it defines) and before `validation` (whose `workflow-refs.command-ref-resolves` check targets it); `reconciliation` consumes the `file-state` probe from `validation` and is consumed by `finalize`'s preflight, so the trio reads in that order; `bootstrap`/`assistant-adapter` are integration layers; `worked-examples` is last — it consolidates end-to-end flows that touch every prior doc. Read `DECISIONS.md` alongside for the *why*.
- **No number prefixes** on part-docs (descriptive names only) — principle #2; we don't grow our own `01-`/`02.5-` disease.
- **Flag the unsettled.** Mark illustrative notation as illustrative; list open questions explicitly rather than implying resolution.
- **Decisions log.** Record every decision in `DECISIONS.md` (dated, ≤1 line of why) as it's made — the *when & why*, not the architecture (`VISION.md` + this file hold current truth). A decision you *defer to a future milestone* (rather than make) goes in [implementation/decisions-pending.md](implementation/decisions-pending.md) → *Before planning*, keyed to its **trigger milestone**, in the same motion as the deferral — so milestone planning resurfaces it. `DECISIONS.md` = what was *decided*; `decisions-pending.md` = what is *owed and when it's due*. A deferral left only in prose has no trigger and will be forgotten.
- **Trivial lane.** Small clarifications skip the loop — don't ceremony-tax a typo.

## What this project is

A **context compiler for coding agents**. A deterministic CLI assembles exactly the instructions and document slices an agent needs for a specific task, just-in-time, and is the sole channel through which the agent reads and writes managed project documents (specs, ADRs, PRDs, commit messages, architecture docs). It replaces static rules files (`CLAUDE.md`, Cursor rules), GSD-style skeleton generation, and Spec-Kit-style workflows.

## The one load-bearing idea

**Take every *structural* operation away from the LLM and give it to the CLI. Leave the LLM only the prose.** Every architectural decision descends from this. The determinism boundary is a hard contract — the canonical table lives in [VISION.md](VISION.md#the-determinism-boundary) (CLI owns: which docs exist, structure, ordering, cross-references, workflow composition + placeholder resolution, placement of every write, validation results; LLM owns: the prose inside slots). "Reproducible" applies to **structure**, never to LLM prose. Do not chase reproducibility of content.

## Architectural invariants (do not violate without revising VISION.md)

- **The CLI core makes no LLM calls.** Composition is deterministic: same *resolved cascade* in → same workflow out. (The *coding agent* may use the CLI to draft a merge proposal — that is the agent acting, not the CLI.)
- **The LLM writes only through the CLI.** It hands content for a *named slot*; the CLI owns placement, cross-ref wiring, versioning, and commit. The LLM places nothing, so it can misplace nothing. *Honest boundary:* this is **adapter-enforced, not sandboxed** — frictionless `jigc` access + bootstrap *advertise+demonstrate* make the CLI the path of least resistance, but an agent that ignores the adapter can still edit files directly. The bet is on ergonomics, not prevention; see [VISION.md](VISION.md) principle #3 and [design/assistant-adapter.md](design/assistant-adapter.md).
- **Stable opaque IDs, never positions.** Managed-artifact IDs (docs, sections, steps, slots, items) and work-unit IDs (tasks; planned: milestones, increments — `milestone > increment > task`) are stable; ordering lives in a separate ordered list. No renumbering; cross-references point at IDs and survive reorder.
- **Engine vs CLI vs domain pack are separate** — the canonical statement lives in [VISION.md](VISION.md) → "What it is" / "Design principles." Working implication for this repo: the engine ships **empty by invariant**; pack content + adapter profiles ride in `cli` per [implementation/module-layout.md](implementation/module-layout.md).
- **Config cascades: project > team > pack-default.** (`team` = a cross-project layer, not per-developer.) All customization is a recorded **delta** against a known base version (never an untracked fork), so upgrades re-apply deltas and report clean/conflict/orphaned.
- **Slots vs placeholders are opposites and must stay syntactically distinct.** *Slots* (`<<slot>>`) are LLM-filled on the write path. *Placeholders* (`{{…}}`) are CLI-filled on the read path (command refs, data values, includes).
- **Storage is plain, human-readable, diff-friendly files.** Humans review and edit through git regardless of the CLI; out-of-band edits are **detected and routed** — conformant non-conflicts absorbed, conflicts blocked and routed to a human, never silently merged or forbidden (see [design/reconciliation.md](design/reconciliation.md)).
- **Writes are transactional.** Integrity holds at a `finalize`/commit boundary, not on every write (else bootstrapping deadlocks).
- **Concurrency is one bounded primitive only.** `fan-out`/`join` steps; each sub-agent writes to an isolated working area keyed by task ID; the CLI merges at a join ordered **by task ID, not completion order**. No DAGs, conditionals, cross-agent messaging, or runtime. Sub-agents coordinate through the CLI (blackboard pattern), never directly — messages carry only `task_id` + status; state is always re-derived from the CLI.

## MVP scope

First loop to build end-to-end: **`single task execution`** (discover → compose → execute → validate) — the cheapest proof the core loop beats a plain `CLAUDE.md`. Scope calls (rationale in [DECISIONS.md](DECISIONS.md), 2026-05-25):

- **Spec-less.** The MVP `single-task` does **not** read a SPEC (no `task.spec`, no `spec` schema ships); the "what" comes from the human `intent` + the codebase. The composed workflow *does* embed `{{task.intent}}` (an engine-native `task` data-value root), so real data-value resolution is exercised. Adding spec support later is a trivial pack addition.
- **Front door, no router workflow yet.** `jigc start` is built in all four forms (orient · cascade-default compose · `--workflow X` · `--task <id>` — see [design/write-commands.md](design/write-commands.md) → Task origination). The **router** selection-guidance workflow is **not** built; instead the cascade knob `default-workflow` ships pointing at `single-task` directly (a `creates-task: true` work-workflow), so MVP's `jigc start "<intent>"` mints in one call. Post-MVP, when ≥2 work-workflows exist, the cascade flips `default-workflow` to a router workflow (`creates-task: false`) and the model-free selection path activates without any code change.
- **Two doc types — one transient, one persisted.** The pack ships **`commit`** (sink = the git message, transient) *and* **`adr`** (persisted to `decisions/`; engine-native fields `status`/`date` + prose slots + the optional **`supersedes`** relation, adr→adr). The agent **creates** an ADR in-task via the workflow-gated `create` flow when a decision is warranted (`single-task` ships with `allows-create: [{type: adr, as: decision}]` in its front-matter — the created ADR binds to `task.decision`; see [design/workflow-dialect.md](design/workflow-dialect.md) → On-disk definition format and [design/write-commands.md](design/write-commands.md) → The create-gate). The persisted ADR is what makes the MVP *prove its differentiators*, and the proof is **mandated by one acceptance path** — the **superseding-decision flow** ([design/worked-examples.md](design/worked-examples.md) → Superseding decision): a later task records an ADR that `supersedes` a committed one, so the loop (a) round-trips a committed, human-editable file — parsing/slicing it in-loop over the byte-stable, golden-tested splice path that **retires the #1 risk** ([implementation/parsing.md](implementation/parsing.md) owns byte-stability as a writer property), (b) detects `file ↔ CLI-state` drift + reconciles OOB edits, (c) composes a **context-slice over that persisted ADR** (`{{@task.decision.supersedes#decision}}`), and (d) walks the **edge index** for forward-ref integrity at finalize — passing when the target exists, blocking when it dangles (the **"validate against reality" integration advantage**). `supersedes` is the only logic-free handle from task state to a committed doc in a spec-less MVP, so this single path converts the differentiators from *supported* to *proven*.
- **The deliverable is `engine` + `cli` + a minimal *embedded* dev pack + the Claude Code profile** — not just the engine (which ships empty by invariant). MVP pack: the **`commit`** + **`adr`** doc-types, the **`single-task`** workflow + its steps, and the cascade pack-default config (incl. the `create`-gate entry `{type: adr, as: decision}`). **Zero *pack* probes** — the MVP checks are engine-native.
- **`finalize` is the commit boundary.** It validates, **renders the commit doc into the git commit message** (the commit type's sink is the VCS message, not a repo file), **promotes any created ADR** to `decisions/`, and `git`-commits the task's code changes + promoted docs as one commit. The CLI commits code too (the agent authors it directly; committing is the CLI's job — "CLI orchestrates, git executes").

Validation is in the MVP: the framework + the **two engine-native probes** (`workflow-refs`, `file-state`) **plus the intrinsic `finalize` integrity gate** — which the ADR + `supersedes` now genuinely exercise: **forward-ref resolution over a real edge index** (the integration advantage), required-slot presence, malformed-value rejection. Pack-provided `doc ↔ code` probes come *after* (they need real code anchors); inverse/minimum-cardinality stays advisory at store scope, never a per-task gate.

Implementation foundations (language/runtime, parsing, module layout) live in [implementation/](implementation/).

## Non-goals

Not a general-purpose workflow engine (no DAGs/conditionals/runtime), not an auto-doc-writer (detects and routes drift; never auto-authors prose), not one-size-fits-all, not a second domain yet (engine/pack boundary is internal discipline, not a public API), not an LLM wrapper.
