# Roadmap

The full-product build order — **the milestone spine**, risk-first, each milestone a runnable slice that *proves* one thing. This is sequencing, not design: each entry names what it proves and the [VISION](../VISION.md) workflow(s) it lands; the milestone-level *what* and *why* live in the design docs. For scope and the determinism boundary see [CLAUDE.md](../CLAUDE.md); for the *why* of the ordering, [DECISIONS.md](../DECISIONS.md).

This doc owns the **milestone spine** plus the increment decomposition of milestones *already planned*. A milestone is turned into ordered increments by the [milestone-planning workflow](milestone-planning-workflow.md) **when it is picked up** — never all milestones up front — so M2–M6 below stay milestone-only until then, while M1 (shipped) carries its full decomposition. Work-unit terms are `milestone > increment > task` ([structural-grammar.md](../design/structural-grammar.md) → Work-units); **tasks are not enumerated** — they're cut per-increment at pickup, so the list stays honest against what prior increments produced. *How* an increment goes from "not started" to "validated and committed" is the [increment workflow](increment-workflow.md); how a milestone *opens* and *closes* are the [planning](milestone-planning-workflow.md) and [completion](milestone-completion-workflow.md) workflows.

The ordering principle is **risk-first**: stand up the spine, retire the #1 technical risk in isolation, build the loop, prove the differentiators, then widen. The spine is strictly linear — each milestone builds on the one before.

## The milestone spine

Each entry is milestone-level only — its increments are cut by the [milestone-planning workflow](milestone-planning-workflow.md) when the milestone is picked up. M1 is shipped; its decomposition follows below.

### M1 · single-task execution loop — ✅ shipped (2026-05-31)
The core loop (discover → compose → execute → validate → finalize), proving it beats a plain `CLAUDE.md` via the superseding-decision flow. VISION: `single task execution`. **Decomposition below.**

### M2 · router + second work-workflow
A second work-workflow (`quick-fix` — commit-only, `allows-create: []`) ships and the cascade's `default-workflow` flips from `single-task` to a **router** (`creates-task: false`), so `jigc start "<intent>"` routes to a model-free selection among the work-workflows. **Proves:** the orthogonal `creates-task` / `default-workflow` knobs (designed in [write-commands.md](../design/write-commands.md), unexercised in M1) hold, and the model-free selection path. No new doctype. *Engine cost (corrected from the first-draft "no engine change" after the planning review — see [DECISIONS.md](../DECISIONS.md)):* a **minimal engine-native `catalog` data-value root** exposing the live work-workflow list to composition, plus the **`creates-task: false` compose contract** (no task minted, intent threaded by agent-substitution). Form D (`jigc start --workflow <X>`) was designed but never built in M1.

### M3 · spec-driven planning
The `spec` doctype + the flow that creates it, so `single-task` reads `{{@task.spec#…}}` (the MVP was deliberately spec-less). **Proves:** the doc-creation differentiator beyond `adr`, and the intent → spec → implementation arc — `spec` earns its schema from its real creator and consumer, not up front. VISION: `project planning` / `milestone planning`.

### M4 · override machinery
Recorded **deltas** (scalar override · structural ops by id · slot-fills · tracked fork) and the **upgrade reconciliation** that re-applies them and reports clean / conflict / orphaned ([overrides.md](../design/overrides.md)). **Proves:** principle #5 — *individualize and still inherit every upstream improvement; every divergence surfaces at a known moment* — entirely unproven in code today. **Sequenced before fan-out** because customization is core to the adoption story and touches far more of the system than the bounded concurrency primitive does.

### M5 · milestone execution (fan-out / join)
The single bounded concurrency primitive — `fan-out`/`join` steps, isolated per-task working areas, a deterministic join **by task id, not completion order** — and the `milestone-execution` workflow that drives it. **Proves:** parallel sub-agent work stays reproducible under the blackboard model. The **highest architectural risk**, but an isolated add-on that rides on the proven read/write/spec/override substrate — hence after M4. VISION: `milestone execution`.

### M6 · project setup (new & existing)
The setup workflows: bootstrap a **new** project (with idea development) and **ingest an existing** one. **Proves:** jigc wires into a real project from zero. **Last** because it is the broadest and least-defined — legacy ingestion is flagged research-grade in [VISION.md](../VISION.md) → Open questions. VISION: `project setup (existing)` + `project setup (new + idea development)`.

## Milestone 1 — single-task execution loop (shipped): decomposition

The six-increment decomposition of M1, kept as the shipped record and the worked example of what a milestone-planning cut produces. A usable `jigc` an agent is pointed at, proving the core loop (discover → compose → execute → validate → finalize) beats a plain `CLAUDE.md`. The **superseding-decision** flow ([worked-examples.md](../design/worked-examples.md) → Superseding decision) is the headline acceptance test — it's what converts the differentiators from *supported* to *proven*. Once all the increments below are built, the milestone is finished through the [milestone-completion workflow](milestone-completion-workflow.md) (independent audit → triage → fix → re-verify) — the acceptance gate before it ships.

**Pack content rides along, not as its own increment.** The embedded dev pack accretes where each increment first needs it: the `commit` schema in increment 2, `single-task` + its steps in 3, the `adr` schema in 5, pack-default config (`default-workflow: single-task`, the `allows-create` create-gate) across 3–5, and the whole pack finalized in 6.

## Increment 1 — Foundations & orientation

**Deliverable:** bare `jigc start` (read-only) runs end-to-end against the embedded pack — prints project state, the workflow catalog with each workflow's `when` hint, and the routing footer.

**Grouped scope:**
- Workspace deps wired in; the two gating build decisions closed: **embed mechanism** (`rust-embed` vs `include_dir` vs build-script) and the **config-family YAML crate**.
- Address grammar + core id/newtypes ([structural-grammar.md](../design/structural-grammar.md) → Addressing).
- `PackSource` trait + `EmbeddedPack`; config-family YAML loading ([module-layout.md](module-layout.md), [overrides.md](../design/overrides.md)).
- Cascade-layer location + read-path resolution for the layers present ([overrides.md](../design/overrides.md)).
- Result-type foundation + the agent-text / json renderers + routing footer ([module-layout.md](module-layout.md) → Renderers).
- The `jigc` command tree skeleton + the bare-`start` orientation output ([bootstrap.md](../design/bootstrap.md), [write-commands.md](../design/write-commands.md) → Task origination).

**Proves:** the CLI-locates / engine-resolves spine, cascade, pack loading, and rendering — with no writes, no composition, no git.

## Increment 2 — Round-trip parser/writer (the risk spike)

**Deliverable:** schema-driven parse + canonical writer + surgical splice for the MVP doc-types, proven by **golden + property/fuzz** tests. Exercised via fixtures, not yet the loop.

**Grouped scope:**
- In-memory doc-type schema model + loading the `commit` (and skeleton `adr`) schema YAML ([document-type-schema.md](../design/document-type-schema.md)).
- `pulldown-cmark` parse → schema mapping; the canonical writer (parser inverse); the splice/generation write pipeline; validate-after-write; conformance diagnostics ([parsing.md](parsing.md), [storage.md](../design/storage.md)).
- The round-trip contract as tests: idempotent-on-canonical + surgical-on-edit golden tests, plus the parse→no-op-write fuzz test ([parsing.md](parsing.md) → Round-trip guarantees).

**Proves:** the **#1 technical risk** — lossless, diff-clean, schema-driven editing of the source-of-truth files — retired before the loop depends on it.

## Increment 3 — Compose (read path)

**Deliverable:** `jigc start "<intent>"` mints a task and emits the composed `single-task` workflow with every placeholder resolved (four-class emitted format).

**Grouped scope:**
- Workflow/step definition loading; include expansion; placeholder resolution — command-refs (via the catalog), data-values (`{{…}}` / `{{@…}}`), includes ([workflow-dialect.md](../design/workflow-dialect.md), [command-catalog.md](../design/command-catalog.md)).
- The four-class emitted format (Run / Content / Author / Reason) + `--explain` ([workflow-dialect.md](../design/workflow-dialect.md) → Emitted format).
- `workflow-refs` validation at compose-time ([validation.md](../design/validation.md)).
- Task origination + minimal task working area + base pin ([write-commands.md](../design/write-commands.md) → Task origination, [storage.md](../design/storage.md)).

**Proves:** the context-compiler core — deterministic composition from definition + cascade + live state.

## Increment 4 — Write path + commit-only finalize

**Deliverable:** the full commit-only loop — `jigc start "..."` → implement → `set-slot`/`set-field` the commit doc → `task validate` → `task finalize` → **one git commit**.

**Grouped scope:**
- Write verbs against the commit doc (`create`, `set-field`, `set-slot`) staging into the working area ([write-commands.md](../design/write-commands.md), [parsing.md](parsing.md) → write pipeline).
- `task diff` / `task validate` / `task discard`; `schema-conformance` integrity checks (required-slot/field, field-value) ([validation.md](../design/validation.md)).
- `finalize` for the commit-only case: validate → render the commit doc to the git message → stage code → `git commit` → post-commit ([finalize.md](../design/finalize.md)).
- `file-state` baseline/hashing for the working area ([reconciliation.md](../design/reconciliation.md), [validation.md](../design/validation.md)).

**Proves:** the cheapest thing that **beats `CLAUDE.md`**. = [worked-examples.md](../design/worked-examples.md) flows #1 + #4.

## Increment 5 — Persisted ADR + edge index (the differentiator)

**Deliverable:** the superseding-decision acceptance path — supersede a committed ADR, context-slice it into a later task, and have finalize walk the edge index (passes when the target exists, blocks when it dangles).

**Grouped scope:**
- ADR `create` via the create-gate (`allows-create: [{type: adr, as: decision}]`); promotion to `decisions/` at finalize ([write-commands.md](../design/write-commands.md) → The create-gate, [finalize.md](../design/finalize.md) → Promote).
- The edge index (committed rebuild + working overlay) + forward-ref integrity (`ref-resolves`) at finalize ([storage.md](../design/storage.md) → Edge-index lifecycle, [validation.md](../design/validation.md) → Forward-ref resolution).
- Reconciliation: the file-state state machine — baseline-adopt, absorb, conformance-block, conflict-block, rename detection ([reconciliation.md](../design/reconciliation.md)).
- The `superseded-context` step + context-slice over the persisted ADR ([worked-examples.md](../design/worked-examples.md) → Superseding decision).

**Proves:** the differentiators, **mandated** by one acceptance path = [worked-examples.md](../design/worked-examples.md) flows #5 + #2.

## Increment 6 — Adapter install & ship

**Deliverable:** a `jigc` an agent can be handed — installed, allowlisted, and documented.

**Grouped scope:**
- `jigc setup` / adapter install: the bootstrap **reference** (`@.jigc/AGENT.md` import) into `CLAUDE.md` + a managed `.jigc/AGENT.md`, the **`SessionStart` hook** + the `jigc` allowlist into `.claude/settings.json`, and project-layer init (`.jigc/config/`) — the adapter MVP scope; the Claude Code profile ([assistant-adapter.md](../design/assistant-adapter.md), [module-layout.md](module-layout.md) → Adapter). (The `Resume` hook + fan-out spawn binding stay post-MVP.)
- The embedded dev pack finalized (`commit` + `adr` doc-types, `single-task` + steps, pack-default config incl. the create-gate).
- Release build + a quickstart.

**Proves:** the deliverable is real — the bootstrap's *path of least resistance* exists on a real machine.

### Status

All six increments (1–6) complete as of 2026-05-31 — the single-task execution loop is built end-to-end (the [superseding-decision](../design/worked-examples.md) acceptance path passes). Tasks were cut per-increment at pickup via the [increment workflow](increment-workflow.md); the milestone was then audited and remediated via the [milestone-completion workflow](milestone-completion-workflow.md) (external code review + end-to-end tests → triage → fixes).

## Milestone 2 — router + second work-workflow: decomposition

Cut 2026-06-01 via the [milestone-planning workflow](milestone-planning-workflow.md) (scope → detect gaps → settle → review → decompose); the independent review reshaped the scope (the `catalog` root + the `creates-task: false` compose contract are real engine work — see [DECISIONS.md](../DECISIONS.md)). Risk-first, linear: the novel engine spine first, the explicit front door next, the pack + flip + acceptance last. **Status: planned, not yet built.**

### Increment 1 — `catalog` root + `creates-task: false` compose contract (the engine spine)

**Deliverable:** composition resolves the engine-native `{{catalog}}` root to the live selectable-work-workflow list (ids + `when` hints), and `compose_in_repo` composes a `creates-task: false` workflow with **no task context** — proven on fixtures (a test `creates-task: false` workflow interpolating the catalog), not yet the live front door.

**Grouped scope:**
- The `catalog` engine-native data-value root: enumerate `creates-task: true` work-workflows + `when` hints from the resolved cascade, deterministic, wired into `ComposeContext` ([workflow-dialect.md](../design/workflow-dialect.md) → data-value roots).
- `compose_in_repo`: gate mint / commit-doc provision / `commit`-role binding on `def.creates_task`; build a no-task compose context for the false case ([write-commands.md](../design/write-commands.md) → Task origination, the `creates-task: false` compose contract).
- `workflow-refs`: a `task.*` placeholder inside a `creates-task: false` workflow is a conformance error ([validation.md](../design/validation.md)).
- The latent-bug retirement the review flagged: a `creates-task: false` compose unit test (the path M1 never exercised).

**Proves:** the two blocking gaps the planning review surfaced (catalog exposure, no-task compose) are retired in isolation before the front door depends on them.

### Increment 2 — Form D front door (`jigc start --workflow <X>`)

**Deliverable:** `jigc start --workflow <X> "<intent>"` composes `X` explicitly and mints iff `X` is `creates-task: true`; unknown `<X>` rejected with a routed finding; `--workflow` combines with the intent positional and is mutually exclusive with `--task`.

**Grouped scope:**
- clap: add `--workflow`, conflicting with `--task`, combining with the `<intent>` positional (`cli.rs`).
- dispatch arm composing `X` via the existing path; unknown-id → routed finding ([write-commands.md](../design/write-commands.md) → Task origination, Form D).
- tests: Form-D mint, unknown-workflow rejection, arg-exclusivity.

**Proves:** the explicit-selection front door — the target the router's output names — works (the never-built fourth `start` form from M1).

### Increment 3 — router + quick-fix + the flip (the acceptance path)

**Deliverable:** the full routing loop — `jigc start "<intent>"` composes the **router** (no mint), which lists `single-task` + `quick-fix` with `when` hints and instructs the agent to re-run `jigc start --workflow <chosen> "<intent>"` → mint → loop. `quick-fix` composes to materially different output than `single-task`.

**Grouped scope:**
- Pack: the `router` workflow (`creates-task: false`, `when` hint) + its `present-catalog` / `route-to-workflow` steps over `{{catalog}}` + the agent-substitution re-run; the `quick-fix` work-workflow (`allows-create: []`, shorter step list, an `implement` variant without the `create-adr` line); flip `pack/config/defaults.yaml` → `default-workflow: router` ([workflow-dialect.md](../design/workflow-dialect.md) → Workflow selection, On-disk definition format).
- Orientation footer copy post-flip — bare `jigc start` advice now leads to the router ([bootstrap.md](../design/bootstrap.md)).
- e2e acceptance in a throwaway repo: the routing path end-to-end; assert `quick-fix` ≠ `single-task` composed output (no ADR affordance, no supersedes line) with non-overlapping `when`; update M1 tests that asserted bare-intent mints `single-task` (backward-compat of the default flip).

**Proves:** M2's headline — model-free selection among ≥2 work-workflows, with a *real* (not hollow) router.

### Status

Planned 2026-06-01; not yet built. The first increment begins through the [increment workflow](increment-workflow.md) when picked up.
