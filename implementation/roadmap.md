# Roadmap

The full-product build order — **the milestone spine**, risk-first, each milestone a runnable slice that *proves* one thing. This is sequencing, not design: each entry names what it proves and the [VISION](../VISION.md) workflow(s) it lands; the milestone-level *what* and *why* live in the design docs. For scope and the determinism boundary see [CLAUDE.md](../CLAUDE.md); for the *why* of the ordering, [DECISIONS.md](../DECISIONS.md).

This doc owns the **milestone spine** plus the increment decomposition of milestones *already planned*. A milestone is turned into ordered increments by the [milestone-planning workflow](milestone-planning-workflow.md) **when it is picked up** — never all milestones up front — so M2–M8 below stay milestone-only until then, while M1 (shipped) carries its full decomposition. Work-unit terms are `milestone > increment > task` ([structural-grammar.md](../design/structural-grammar.md) → Work-units); **tasks are not enumerated** — they're cut per-increment at pickup, so the list stays honest against what prior increments produced. *How* an increment goes from "not started" to "validated and committed" is the [increment workflow](increment-workflow.md); how a milestone *opens* and *closes* are the [planning](milestone-planning-workflow.md) and [completion](milestone-completion-workflow.md) workflows.

The ordering principle is **risk-first**: stand up the spine, retire the #1 technical risk in isolation, build the loop, prove the differentiators, then widen. The spine is strictly linear — each milestone builds on the one before.

## The milestone spine

Each entry is milestone-level only — its increments are cut by the [milestone-planning workflow](milestone-planning-workflow.md) when the milestone is picked up. M1 is shipped; its decomposition follows below.

### M1 · single-task execution loop — ✅ shipped (2026-05-31)
The core loop (discover → compose → execute → validate → finalize), proving it beats a plain `CLAUDE.md` via the superseding-decision flow. VISION: `single task execution`. **Decomposition below.**

### M2 · router + second work-workflow — ✅ shipped (2026-06-01)
A second work-workflow (`quick-fix` — commit-only, `allows-create: []`) ships and the cascade's `default-workflow` flips from `single-task` to a **router** (`creates-task: false`), so `jigc start "<intent>"` routes to a model-free selection among the work-workflows. **Proves:** the orthogonal `creates-task` / `default-workflow` knobs (designed in [write-commands.md](../design/write-commands.md), unexercised in M1) hold, and the model-free selection path. No new doctype. *Engine cost (corrected from the first-draft "no engine change" after the planning review — see [DECISIONS.md](../DECISIONS.md)):* a **minimal engine-native `catalog` data-value root** exposing the live work-workflow list to composition, plus the **`creates-task: false` compose contract** (no task minted, intent threaded by agent-substitution). Form D (`jigc start --workflow <X>`) was designed but never built in M1.

### M3 · spec-driven planning — ✅ shipped (2026-06-02)
The `spec` doctype + the flow that creates it, so a spec-driven workflow reads `{{@task.spec#…}}` (the MVP was deliberately spec-less). **Decomposition below.** **Proves:** the doc-creation differentiator beyond `adr`, and the intent → spec → implementation arc — `spec` earns its schema from its real creator and consumer, not up front. VISION: `project planning` / `milestone planning`.

### M4 · override application
Recorded **deltas** — `scalar-set` · `structural-op` by id (insert/replace/remove) · `slot-fill` · `tracked-fork` — applied through the cascade's 9-phase resolution so a project composes a workflow/schema **differently from the pack default without forking** ([overrides.md](../design/overrides.md)). **Proves:** principle #5's *individualize* half — customize one thing against a versioned base via tracked deltas, the cascade wired **live** (today the resolver is a unit-proven pure function with no runtime feeder — `cascade::resolve` is called only for the provenance header; compose reads from a single layer). **Sequenced before fan-out** because customization is core to the adoption story and touches far more of the system than the bounded concurrency primitive does.

### M5 · upgrade reconciliation
The other half of principle #5: the engine-native **`override-default` probe** + a guarded **`jigc upgrade`** that re-applies each recorded delta against a new pack version and reports **clean / conflict / orphaned** (stateless `base-hash` compare). On `conflict` it **blocks with a review route** (keep / re-target / drop, showing what changed) — the true 3-way-merge authoring is deferred (matching `file_state`'s own deferral, [reconciliation.md](../design/reconciliation.md)); `override-default`'s checks ship **blocking-by-default** (cascade-tunability is M6). **Proves:** principle #5's *inherit-upstream* half — *inherit every upstream improvement for free; every divergence surfaces at a known moment* — entirely unbuilt in code today (no `upgrade` command, no probe, no version trigger). Rides on M4's recorded-delta substrate — hence immediately after it ([overrides.md](../design/overrides.md) → Upgrade reconciliation). *(Split from the old combined M5 — see [DECISIONS.md](../DECISIONS.md) 2026-06-03; the severity subsystem the probe's tunability rests on became M6.)*

### M6 · severity tuning & demotion-lock
The validation-severity subsystem the cascade has always promised but no probe yet honors: **severity assigned via the cascade** (every tunable probe site reads `validation.<probe>.<check>.severity` from `project > team > pack-default` instead of a hardcoded literal), **per-check knob granularity** (replacing M4's per-probe keys), the **intrinsic / tunable two-tier** model with the **demotion-lock floor** (a `scalar-set` demoting an intrinsic check below `blocking` is rejected at cascade resolution), and a **non-task probe entry point** so non-task-scoped probes (like `override-default`) participate. **Proves:** "severity is a cascade setting, never a code change" ([validation.md](../design/validation.md) → Severity inventory) holds end-to-end. **Sequenced here** to finish the override/validation theme (M4 apply · M5 reconcile · M6 tune) before the concurrency primitive — and because nothing in the product yet *needs* tunable severities, so it follows the milestone that introduces the first tunable probe (`override-default`). *(Split from the old combined M5 — see [DECISIONS.md](../DECISIONS.md) 2026-06-03.)*

### M7 · milestone execution — the deterministic join core (data plane)
The engine half of the single bounded concurrency primitive: a **`milestone` work-unit** (`milestone > task`, one shared base) with task-list verbs (`create` / `add-task` / `add-from-spec`), **isolated sub-task area dirs** + a **join-time isolation check**, **parallel doc minting** with a deterministic task-id-ordered collision-suffix + intra-doc self-ref rewrite (over `created` vs `edited-from-base` provenance), and the **by-task-id join merge** (edge-index "site 4") — a disjoint union, **blocking** on a same-doc clash, with **cross-area refs rejected** (resolved per sub-area against `committed ∪ that sub-task's own area` only). **Proves:** the join is a *pure function of the set of sub-task areas* — reproducible regardless of completion order — proven by **permutation tests over real multi-area fixtures** (not a real agent spawn; that's M8). The **highest architectural risk**, isolated to the engine data plane. *(Split from the old combined M7 — see [DECISIONS.md](../DECISIONS.md) 2026-06-04; the spawn binding + workflow + genuine end-to-end became M8.)* VISION: `milestone execution`.

### M8 · milestone execution — spawn binding + acceptance (control plane)
The assistant half: **`fan-out`/`join` step kinds** + the spawn-instruction emit class, the **`{{milestone.tasks}}` data-value root** the fan-out step consumes, the **adapter spawn binding** + the `jigc workflow W --task <id>` sub-agent re-entry + the **write-time `--task`-scoped barrier**, the **`milestone-execution` workflow** that composes the fan-out over `{{milestone.tasks}}`, and the **genuine real-Task-tool end-to-end** acceptance ([worked-examples.md](../design/worked-examples.md) flow 9, driven by real spawn). **Proves:** parallel sub-agent work stays reproducible *under the blackboard model end-to-end* — the CLI emits spawn instructions deterministically, the adapter launches, the M7 join recombines. Rides directly on M7's data plane. *(Split from the old combined M7 — see [DECISIONS.md](../DECISIONS.md) 2026-06-04.)* VISION: `milestone execution`.

### M9 · project setup (new & existing)
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

All three increments (1–3) complete as of 2026-06-01 — the routing loop is built end-to-end: bare `jigc start "<intent>"` composes the **router** (no mint), which lists `single-task` + `quick-fix` over `{{catalog}}` and routes the agent to an explicit `jigc start --workflow <chosen>` mint; `quick-fix` composes materially differently from `single-task`. Tasks were cut per-increment at pickup via the [increment workflow](increment-workflow.md); the execute phase surfaced one genuine fork (resume must compose the task's *own* minted workflow across the default flip — see [DECISIONS.md](../DECISIONS.md) 2026-06-01) which was resolved before the flip landed. The milestone was then audited and remediated via the [milestone-completion workflow](milestone-completion-workflow.md) (external code review + end-to-end tests → triage → one fix: an orphaned `build_catalog` removed).

## Milestone 3 — spec-driven planning: decomposition

Cut 2026-06-01 via the [`/milestone-plan`](../.claude/commands/milestone-plan.md) command (scope → detect gaps → settle → review → decompose; its first live use). Four `gap-detector` agents + an independent `design-reviewer` reshaped the design: M3's real cost is the **cross-task binding** surface — the `jigc task bind` verb + a `reads:` role declaration — not the "trivial pack addition" the spec-less MVP decision assumed; the read path, persisted-slice read, edge-walk, router N-enumeration, and four-class emit are all already built ([DECISIONS.md](../DECISIONS.md) 2026-06-01). Risk-first, linear: the schema foundation first, the cross-task binding spine (M3's #1 new risk) in isolation next, then the two halves of the arc — spec authoring, then spec-consuming implementation + the acceptance path. **Status: planned, not yet built.**

### Increment 1 — `spec` doctype + `commit.implements` edge (schema foundation)

**Deliverable:** the `spec` schema ships and round-trips; the `commit` schema gains the `implements` ref; both proven on fixtures over the byte-stable parser/writer + the edge / `ref-resolves` machinery — not yet any workflow.

**Grouped scope:**
- `crates/cli/pack/schemas/spec.yaml`: `goal` (slot), `context` (slot), `criteria` (repeatable, `id-from: title`, item = title + `statement` slot); `location: specs/`, `id-from: title`; **no** `status`/`date`, **no** `decided-by` ([document-type-schema.md](../design/document-type-schema.md), [doctype-map.md](doctype-map.md)).
- `commit.yaml`: add the `implements` ref (`type: ref, to: spec, card: "0..1", inverse: implemented-by`) as a section field mirroring `adr`'s `supersedes`; pin its address fragment against the canonical grammar (resolving the flow-1-flat vs flow-5-section-qualified inconsistency — review #7) ([document-type-schema.md](../design/document-type-schema.md), [structural-grammar.md](../design/structural-grammar.md) → Addressing).
- Golden + round-trip tests: a `spec` instance parses → canonical-writes idempotently; the transient-source `commit.implements` edge emits into the working overlay and `ref-resolves` against a committed-store spec ([parsing.md](parsing.md) → Round-trip, [validation.md](../design/validation.md) → Forward-ref).

**Proves:** the new doctype + the first transient-source edge parse/write/validate cleanly on the proven substrate, before the workflows depend on them.

### Increment 2 — `jigc task bind` + `reads:` declaration (the cross-task binding spine)

**Deliverable:** a workflow declaring `reads: [{role: spec, type: spec}]` makes `task.spec` a valid `workflow-refs` root; `jigc task bind spec <addr>` binds a committed spec into the task's roles; on resume re-compose `{{@task.spec#criteria}}` resolves over that committed file — proven on fixtures (a test `reads`-declaring workflow + a committed spec fixture), not yet the pack workflows.

**Grouped scope:**
- `reads:` front-matter parse + seed the declared role into the task's role map (distinct from `allows-create`); undeclared `task.<role>` stays a `workflow-refs` conformance error, declared-but-unbound resolves to empty ([workflow-dialect.md](../design/workflow-dialect.md) → `reads`, [validation.md](../design/validation.md)).
- `jigc task bind <role> <addr>` verb (clap + dispatch): the five-step enforcement — active task; role declared; target resolves in committed store; doctype matches; record binding (last-write-wins) ([write-commands.md](../design/write-commands.md) → Binding a context role).
- The bound role persists in the task working area and is re-read on `resume_in_repo`, so the resume re-compose resolves the slice — reusing the M1 deferred-bind-then-resume path proven by `superseded-context` ([storage.md](../design/storage.md), [worked-examples.md](../design/worked-examples.md) → flows 5/6).

**Proves:** M3's one genuinely new surface — binding an existing committed doc into a later task — in isolation, retired before the pack workflows ride on it. The #1 new risk.

### Increment 3 — the `plan` workflow (spec authoring + code-less finalize)

**Deliverable:** `jigc start --workflow plan "<intent>"` mints a task, the agent authors a spec via the create-gate, and `finalize` promotes it to `specs/` and commits a code-less, doc-only commit — end-to-end.

**Grouped scope:**
- Pack: the `plan` workflow (`creates-task: true`, `allows-create: [{type: spec, as: spec}]`, a `when` hint) + its steps (author-spec + finalize); spec born **only** in `plan` ([workflow-dialect.md](../design/workflow-dialect.md), [write-commands.md](../design/write-commands.md) → The create-gate).
- Finalize for a code-less task: the promoted spec is a non-empty diff (empty-commit guard satisfied), commit `type: docs`; a golden/e2e proving a spec-only finalize lands one commit ([finalize.md](../design/finalize.md) → Promote, empty-commit guard).

**Proves:** the doc-creation differentiator *beyond* `adr` — agent-authored, persisted `spec` via the create-gate — and that a doc-only task finalizes cleanly.

### Increment 4 — `implement-from-spec` + spec discovery + the two-task acceptance

**Deliverable:** the full M3 arc — a `plan` task authors a spec, then `jigc start --workflow implement-from-spec "<intent>"` surfaces the committed specs, the agent binds one, re-composes to read `{{@task.spec#criteria}}`, implements, and finalize records + walks `commit —implements→ spec`. The router lists four work-workflows.

**Grouped scope:**
- Engine: `store.<doctype-id>` enumerates committed instances as a collection, and a collection in a plain step emits as a readable Content list (`{{store.specs}}`) ([workflow-dialect.md](../design/workflow-dialect.md) → data-value roots).
- Pack: the `implement-from-spec` workflow (`creates-task: true`, `reads: [{role: spec, type: spec}]`, `allows-create: [{type: adr, as: decision}]`, a `when` hint) + the new `step:locate-from-spec` (surfaces `{{store.specs}}`, emits the `bind` + the `Run: jigc start --task <id>` re-compose, reads `{{@task.spec#criteria}}`) — distinct from the shared `step:locate` ([workflow-dialect.md](../design/workflow-dialect.md), [worked-examples.md](../design/worked-examples.md) → flow 6).
- The router now lists single-task / quick-fix / plan / implement-from-spec; write **non-overlapping** `when` hints; orientation surfaces them automatically ([workflow-dialect.md](../design/workflow-dialect.md) → Workflow selection, [bootstrap.md](../design/bootstrap.md)).
- e2e acceptance in a throwaway repo: the two-task arc end-to-end (plan → implement-from-spec), asserting the spec slice resolves on resume, the `implements` edge walks at finalize (passes when the spec exists, blocks when it dangles), and the four `when` hints are non-overlapping ([worked-examples.md](../design/worked-examples.md) → flow 6, [validation.md](../design/validation.md)).

**Proves:** M3's headline — the intent → spec → implementation arc end-to-end, with cross-task binding and `implements` edge integrity — the mandated acceptance path.

### Status

All four increments (1–4) complete as of 2026-06-02 — the intent → spec → implementation arc works end-to-end across two tasks: a `plan` task authors and commits a `spec` (code-less `docs(spec):` finalize), and a later `implement-from-spec` task discovers it via `{{store.specs}}`, binds it with `jigc task bind spec <addr> <id>`, reads `{{@task.spec#criteria}}` on the resume re-compose, and finalize walks the `commit —implements→ spec` edge (passing when the spec exists, blocking when it dangles). Built via the [milestone-build](milestone-build) harness on its **first live run** — which surfaced two genuine planning-gap halts (the create-gate hardcoded `single-task`; the spec read path sliced only slot sections, not the shipped repeatable `criteria`), both human-gated and fixed before the harness resumed. Tasks were cut per-increment at pickup via the [increment workflow](increment-workflow.md); the milestone was then audited and triaged via the [milestone-completion workflow](milestone-completion-workflow.md) (independent code-review + e2e → one fix: the emitted `jigc task bind` line was missing the task-id positional; one finding accepted as deferred: `criteria` authoring awaits the unbuilt `add-item` verb).

## Milestone 4 — override application: decomposition

Cut 2026-06-03 via the [`/milestone-plan`](../.claude/commands/milestone-plan.md) command (scope → detect gaps → settle → review → decompose). Four `capability-auditor` subagents established the baseline, four `gap-detector` subagents + an independent `design-reviewer` reshaped the design ([DECISIONS.md](../DECISIONS.md) 2026-06-03). The headline finding: M4's reuse claims were mostly **`unverified-reuse`** — the cascade resolver (`cascade::resolve`, phases 2–3) is unit-proven but **unwired** (zero production callers for `Resolved::file_owner`/`scalar`; its one live caller uses it for the provenance header alone), resolution **phases 4 (structural-op) and 5 (slot-fill) are absent**, the content `Address` grammar is instance-scoped and does **not** fit a definition-target (spiked + refuted), and no typed knob is declared anywhere. So M4's real cost is *wiring the unwired cascade live into compose + building two resolution phases from scratch + a full config-write verb surface* — not "add delta application on a working cascade." The milestone was also split from the old "override machinery" entry (upgrade reconciliation → [M5](#m5--upgrade-reconciliation)); everything `override-default` / `jigc upgrade` / base-hash-compare / 3-way-merge / severity-floor-enforcement is **out**, deferred to M5. The design for every shape below is settled in [overrides.md](../design/overrides.md) (the home of record), [storage.md](../design/storage.md) (config layout), [workflow-dialect.md](../design/workflow-dialect.md) (`{{fill:}}`), and [worked-examples.md](../design/worked-examples.md) flow 3 (the acceptance flow). Risk-first, linear: the determinism-trap wiring first, then the two from-scratch resolution phases with their verbs, then `tracked-fork`, then the acceptance. **Status: planned, not yet built.**

**Pack content rides along.** Two pack additions accrete where first needed: the `config/knobs.yaml` declaration in increment 1, and the `{{fill: extra-guidance}}` extension point in `steps/implement.yaml` in increment 4.

### Increment 1 — Config loader + live scalar cascade (the wiring spine + the determinism trap)

**Deliverable:** a `scalar-set` recorded in `.jigc/config/manifest.yaml` resolves through the cascade and changes composed output — `default-workflow` overridden at the project layer flips bare-`jigc start` minting — while the **no-override** path stays **byte-identical** to today. Proven on a hand-authored fixture manifest + `jigc config set`.

**Grouped scope:**
- The `config/knobs.yaml` declaration format (`{key, type, of?, default}` reusing `schema::FieldType`) + the loader that seeds the `PackDefaultLayer` scalar surface (closed key set + materialized defaults) from it; `default-workflow` + the `validation.*.severity` keys declared, `pack-id` kept as a non-knob identity field ([overrides.md](../design/overrides.md) → Scalar knobs).
- The `.jigc/config/manifest.yaml` loader for the `scalar:` block → a populated `OverrideLayer`; wire the resolved cascade into the live compose path so `default-workflow` is read via `resolved.scalar(...)`, not ad-hoc `serde_yaml_ng::get()` ([overrides.md](../design/overrides.md) → Resolution algorithm, [storage.md](../design/storage.md) → Config layout).
- The **read-side determinism invariant**: `resolved.scalar(k)` returning `None` for a compose-read key is a hard error, not a fallback; a **no-delta byte-identical golden** over the existing `start_compose` fixtures + a test that an undeclared read-key fails loudly.
- `jigc config set <key> <value>` (clap `config` subcommand tree + dispatch) writing the `scalar:` block, adjudicated at write time via `check_value` (undeclared key / wrong type rejected).
- Correct the overstated `cascade.rs` / `compose.rs` doc-comments that claim phase-2 shadowing is "already wired into compose."

**Proves:** the unwired resolver is wired **live** for the scalar path, the cascade *applies* a delta (not just locates a layer), and the determinism boundary holds (no-override = byte-identical) — the milestone's #1 risk, retired in isolation.

### Increment 2 — Structural-op application: step shadowing (phase 2) + include-list mutation (phase 4)

**Deliverable:** a `structural-op` delta (insert / replace / remove) in a hand-authored manifest changes a composed workflow's include list — a project step shadows or augments the pack's, proven on fixtures (worked-examples flow 3a's compose-time application), not yet a verb.

**Grouped scope:**
- A layer-aware `StepSource` that consults `Resolved::file_owner` to read a step from the highest-precedence layer (project `.jigc/config/steps/<id>.yaml` → pack), replacing the single-layer `PackStepSource` — phase-2 by-id shadowing wired into compose at last ([overrides.md](../design/overrides.md) → Resolution algorithm phase 2).
- The **definition-target** address model — `workflow:<id>` + `#<step-id>` / `after:` / `before:` over `WorkflowDef.includes` — distinct from the instance-scoped content `Address` (the spiked-and-refuted reuse); the insert / replace / remove pass applied at **phase 4, before include expansion** ([overrides.md](../design/overrides.md) → Delta targets, Why structural deltas precede expansion).
- Cycle/orphan interaction: a structural delta whose anchor a same-manifest delta removed surfaces at resolution via `workflow-refs`; replace's "replacement is another step id" semantics ([overrides.md](../design/overrides.md) → `replace` vs `tracked-fork`).

**Proves:** the second from-scratch resolution phase — structural deltas mutate the composition tree deterministically before expansion, over a target namespace that does not collide with document addressing — the milestone's #2 risk.

### Increment 3 — Structural-op verbs (`config insert-step` / `replace-step` / `remove-step`)

**Deliverable:** the three structural verbs record the deltas + native step files increment 2 applies; `jigc config insert-step --workflow single-task --after implement ./x.yaml` lands a runnable override end-to-end.

**Grouped scope:**
- clap + dispatch for `insert-step` / `replace-step` / `remove-step`; native step file written to `.jigc/config/steps/<basename>.yaml`, the delta referencing `step:<basename>` (**id = filename basename**); collision-with-existing-id rejected ([overrides.md](../design/overrides.md) → Authoring deltas).
- Write-time validation: the anchor/target step-id present in the resolution **as of this edit** (a snapshot; whole-cascade consequences stay at resolution-time per the write/resolve split).

**Proves:** the structural override path is authored through the CLI, not just hand-edited — the agent/human records a shape change without forking.

### Increment 4 — slot-fill end-to-end (`{{fill:}}` + the orphan check + `config fill`)

**Deliverable:** a pack step ships a `{{fill: extra-guidance}}` point; a `slot-fill` delta fills it and the content appears in composed output; an orphaned or nested `{{fill:}}` is a blocking `workflow-refs` finding.

**Grouped scope:**
- The `{{fill:<id>}}` placeholder (the 4th read-path placeholder kind) resolved at **phase 5, before expansion**, from the cascade's `slot-fill` deltas (or the pack default body); **no nested `{{fill:}}`** ([overrides.md](../design/overrides.md) → The `{{fill:}}` placeholder; [workflow-dialect.md](../design/workflow-dialect.md) → Leaves).
- The engine-native `workflow-refs` checks: a `slot-fill` targeting a `<fill-id>` no resolved body declares = blocking (closed-surface, both authoring paths); a surviving lone `{{fill:}}` = blocking.
- `jigc config fill <step:id#fill-id> --from-file <file>` writing the `slot-fill` delta + `.jigc/config/fills/<id>.md`; write-time rejection of fill content containing `{{fill:}}`.
- Pack: add the `{{fill: extra-guidance}}` extension point to `steps/implement.yaml`.

**Proves:** the doc-creation-style extension point — a project injects content into an anticipated point without forking the step, and the closed-surface discipline holds in M4 (orphans caught now, not deferred to M5).

### Increment 5 — `tracked-fork` (`config fork` + base-hash recording)

**Deliverable:** `jigc config fork workflow:single-task#implement` copies the resolved step into a project native file that shadows it (applied as a phase-2 file shadow) and records `base-version` + `base-hash`; the recorded delta round-trips.

**Grouped scope:**
- `jigc config fork <addr>` (clap + dispatch): copy the resolved native step/section bytes into `.jigc/config/steps/<id>.yaml`, record the `tracked-fork` delta with `base-version` + the **blake3** `base-hash` of those bytes (the pinned basis — recorded in M4, compared only in M5) ([overrides.md](../design/overrides.md) → `tracked-fork` hash basis).
- Apply path: a `tracked-fork`'d unit is just a shadowed file at phase 2 (no new resolution logic) — the increment is the verb + the recording, not a new phase.

**Proves:** the last-resort rung records its ancestor honestly — the M5-reconciliation precondition (a fork that knows what it forked) exists, with the hash basis pinned so M5's stateless compare can't silently break.

### Increment 6 — `--explain` overrides + the e2e acceptance

**Deliverable:** `jigc start --explain` shows the resolution tree with per-layer override provenance (worked-examples flow 3a's tree); an e2e in a throwaway repo drives all four delta kinds and asserts each changes composed output while the no-override path stays byte-identical.

**Grouped scope:**
- Extend the `--explain` resolution-tree output ([workflow-dialect.md](../design/workflow-dialect.md) → `--explain` output contract) to render `overrides applied: N`, the winning layer per step/knob, and the `← replaces … at position` annotations.
- e2e acceptance (throwaway repo): `config set` flips `default-workflow`; `insert/replace-step` changes the include list; `config fill` fills a `{{fill:}}` point; `config fork` records a base-hash; the orphan/wrong-type/undeclared-key rejections fire; **assert no-override compose output is byte-identical** to the pre-M4 baseline ([worked-examples.md](../design/worked-examples.md) → flow 3).

**Proves:** M4's headline — a project composes differently from the pack default via recorded deltas across all four kinds, with the cascade applied live and the determinism boundary intact — the mandated acceptance path. Sets up [M5](#m5--upgrade-reconciliation) (the recorded deltas, now with base-hashes, become reconciliation's input).

### Status

All six increments (1–6) complete as of 2026-06-03 — override application works end-to-end: the previously-unwired cascade resolver is wired **live** into compose (phases 2–5), all four delta kinds (`scalar-set` · `structural-op` insert/replace/remove · `slot-fill` via `{{fill:}}` · `tracked-fork`) record through the `jigc config` verb suite and demonstrably shift composed output, typed knobs are declared in `config/knobs.yaml` and adjudicated via `check_value`, and the **no-override path stays byte-identical** to the pre-M4 golden (the determinism trap, held). Built via the [milestone-build](milestone-build) harness (a 48-agent run, no build halts — the front-loaded risks held). The [milestone-completion audit](milestone-completion-workflow.md) (independent code-review + real-binary e2e) surfaced **five findings**, all human-triaged for fix and **remediated + re-verified through the binary**: (HIGH) a `slot-fill-orphan` check scoped to the composed workflow instead of the target step *bricked the front door* once any slot-fill was recorded — re-keyed on the target step (`7a1d07f`); (MEDIUM) `{{include:}}` in a mixed prose+include step body hoisted to the end instead of expanding in place, inverting worked-examples-3a order — fixed (`789eaeb`); (LOW) the line-based `{{fill:}}` machinery let an inline mid-line nested fill leak unresolved — made token-based (`e02ad08`); plus two advisories (dead cascade accessors removed `50e452f`; JSON `overrides_applied` aligned with the text total `eeee1f6`). Both the HIGH and MEDIUM defects had been masked by builder in-loop tests (the acceptance only composed `single-task`; the replace-step test declined to assert intra-step order) — caught only by the independent audit.

## Milestone 5 — upgrade reconciliation: decomposition

Cut 2026-06-03 via the [`/milestone-plan`](../.claude/commands/milestone-plan.md) command (scope → detect gaps → settle → review → decompose) — the first run against the M4-hardened harness. Four `capability-auditor` subagents established a clean post-M4 baseline ("no claim-vs-reality landmines"); four `gap-detector`s + an independent `design-reviewer` (both spiking the real binary per the hardening) shaped the design ([DECISIONS.md](../DECISIONS.md) 2026-06-03). Key settled calls: the milestone was **split** (the severity-tuning subsystem the probe's tunability rests on became [M6](#m6--severity-tuning--demotion-lock)); the conflict path **detects-and-surfaces** (blocks with a String review route — 3-way merge deferred, since the stateless base-*hash* design can't feed a true merge); `jigc upgrade` is **report-and-route only** (no manifest mutation); the version is **narrative-only** (the hash compare needs no version diff); and the genuine `v1→v2` e2e rides a new **`FilesystemPack` + `JIGC_PACK_DIR`** seam. The design-reviewer's catch that shaped increment 2: a `tracked-fork` *shadows the same step id*, so the probe must re-read the **pack-default** unit (`PackSource::read`), never the cascade-resolved owner, or every fork falsely classifies `clean`. Design home of record: [overrides.md](../design/overrides.md) → Upgrade reconciliation; acceptance = [worked-examples.md](../design/worked-examples.md) → flow 7. Risk-first, linear: the recording substrate first, the classifier (the core logic) next, the test seam, then the command + genuine acceptance. **Status: planned, not yet built.**

### Increment 1 — base-hash substrate for `replace` / `remove`

**Deliverable:** `jigc config replace-step`/`remove-step` record the displaced pack unit's `base-version`+`base-hash` (pack-direct blake3, the same basis `config fork` already writes), so all three content-bearing delta kinds carry a re-comparable basis — proven on fixtures (manifest round-trip + recorded-basis), with the no-override compose **byte-identical**.

**Grouped scope:**
- `config.rs`: `replace-step`/`remove-step` resolve the *displaced pack unit's* bytes (pack-direct — reuse `resolve_fork_bytes`/`hash_bytes`) and write `base-version`/`base-hash` keys on the manifest entry ([overrides.md](../design/overrides.md) → Per-kind base-hash basis).
- **Representation pin (design-review B2):** the basis rides in a **separate in-memory record keyed by target**, *not* as new fields on the compose-facing `StructuralDelta` — so phase-4 compose and its byte-identical goldens are untouched; the loader parses the keys **Optional** for `replace`/`remove` (old manifests lack them), while `tracked-fork`'s stay required.
- Tests: manifest round-trip with and without the basis; the recorded basis equals an independent pack-direct hash; a compose golden proving `StructuralDelta` (hence composed output) is unchanged.

**Proves:** the conflict table's content-dependent kinds carry a basis recorded the *same pack-direct way* for all three, without perturbing the compose path the basis must never touch.

### Increment 2 — the `override-default` classifier (the core logic)

**Deliverable:** an engine classifier maps each recorded delta against a given `PackSource` to **clean / conflict / orphaned / needs-rebasing** — existence + pack-direct re-hash with the fork shadow-bypass — proven in-process over a two-version `FakePack`, including the changed-fork→conflict case and a delta whose target the v2 pack omits.

**Grouped scope:**
- engine: the `override-default` classifier `(deltas, &dyn PackSource) → Vec<Finding>` — per-kind existence + content compare; **re-reads the pack-default unit via `PackSource::read`, never the cascade-resolved owner** (the tracked-fork shadow-bypass — design-review B3; a shadow-aware read compares a fork to itself and falsely says `clean`); each delta identified in its finding by its **target string**; a **`String` route**; **blocking-by-default** ([overrides.md](../design/overrides.md), [validation.md](../design/validation.md) → `override-default`).
- The four outcomes, with **`needs-rebasing` (no recorded basis) distinct from `clean` (basis present and equal)**.
- Tests (FakePack `v1→v2`, per [Validation hardening](increment-workflow.md) #5 — exercise a context that *omits* the target): clean (unchanged fork), conflict (changed `replace`/`remove`/**fork**), orphaned (target absent in v2), needs-rebasing (basis-less legacy delta).

**Proves:** M5's core logic — correct classification including the fork shadow-bypass a naive read would silently defeat — in isolation, before any command or CLI surface. The #1 risk.

### Increment 3 — `FilesystemPack` + the pack-source factory seam

**Deliverable:** a `FilesystemPack` `PackSource` (reads a pack tree from a dir; `pack_version` from the dir's `config/defaults.yaml` `version` key, `fs-local` sentinel if absent), selected by a **`JIGC_PACK_DIR`** env var via **one pack-source factory** that replaces every production `EmbeddedPack::new()` site; with the env unset, all output is **byte-identical** to today.

**Grouped scope:**
- `FilesystemPack` impl (`list`/`read`/`pack_version` over a directory) ([overrides.md](../design/overrides.md) → the FilesystemPack seam; [module-layout.md](module-layout.md)).
- The factory honoring `JIGC_PACK_DIR`; route **every** production pack-source construction through it — the *recording* verbs and the *upgrade* path read the **same** env-selected pack (design-review N1: the obligation, not a site count).
- **Determinism guard:** a no-env golden over the existing fixtures stays byte-identical; `JIGC_PACK_DIR=<dir>` loads the directory pack; the two pack dirs report distinct `version`s.

**Proves:** a genuine alternate pack can drive the built binary (the `v1→v2` testability seam) with zero behavior change when unused.

### Increment 4 — `jigc upgrade` + the genuine v1→v2 acceptance

**Deliverable:** `jigc upgrade` runs the `override-default` classifier over every recorded delta against the current pack, renders findings+routes, blocks on non-clean, mutates nothing; a **genuine two-pack e2e** (`JIGC_PACK_DIR=<v1>` record → `JIGC_PACK_DIR=<v2>` upgrade) classifies every outcome and the re-pin→clean loop.

**Grouped scope:**
- `cli`: `Command::Upgrade` (clap + dispatch); **report-and-route only** — load the cascade deltas, run the classifier, render via the existing findings+routes renderer, exit non-zero on blocking; no manifest write, no transaction ([overrides.md](../design/overrides.md) → The `jigc upgrade` command).
- e2e via the FilesystemPack seam: record the deltas against v1, `jigc upgrade` against a v2 that changes some targets, removes a `{{fill:}}` point, leaves others — assert clean / conflict (incl. the **fork** conflict) / orphaned / needs-rebasing per [worked-examples.md](../design/worked-examples.md) flow 7; then resolve via `jigc config` verbs and re-run to all-clean.

**Proves:** M5's headline — `jigc upgrade` surfaces every divergence at one known moment (*no upstream change silently lost; no override silently broken*), proven **genuinely** through the binary across a real pack change = [worked-examples.md](../design/worked-examples.md) flow 7. The recorded deltas (now with complete bases) are M6's input.

### Status

All four increments (1–4) complete as of 2026-06-03 — upgrade reconciliation works end-to-end: `jigc upgrade` runs the `override-default` classifier over every recorded delta against the current pack (via the `FilesystemPack`/`JIGC_PACK_DIR` seam), classifying **clean / conflict / orphaned / needs-rebasing** with a stateless pack-direct hash compare, **report-and-route only** (mutates nothing), blocking on non-clean. The genuine `v1→v2` e2e (worked-examples flow 7) passes through the real binary, including the **fork shadow-bypass** the design-review pre-empted (an unchanged fork stays `clean`, a changed one `conflict`s — proof the probe re-reads the pack unit, not the project shadow) and the re-pin→all-clean loop. Built via the [milestone-build](milestone-build) harness (a 24-agent run — ~half M4's footprint — **no build halts**, every increment validated clean with **0 fix rounds**; the first run on the M4-hardened harness). The [milestone-completion audit](milestone-completion-workflow.md) (independent code-review + real-binary e2e) returned **e2e overall-pass with no findings** and two LOW code-review notes: (FIXED `ba7f86f`) `jigc upgrade` now also classifies `scalar-set`/`insert` existence (orphaned on a removed knob key / dropped anchor — completing the conflict table); (ACCEPTED as deferred) content-target existence checks step-*file* presence, not workflow include-list membership — MVP-correct for the single-workflow pack, revisit for multi-workflow. The B3 fork-shadow trap was caught in **design** (the hardened design-reviewer) and never reached the build — the harness hardening working as intended.

## Milestone 6 — severity tuning & demotion-lock: decomposition

Cut 2026-06-04 via the [`/milestone-plan`](../.claude/commands/milestone-plan.md) command (scope → detect gaps → settle → review → decompose). Three `capability-auditor` subagents verified the baseline (**severity-into-probes is ~0% built**: severity 100% hardcoded across ~17 sites; `Probe` trait a 6-line stub; `validate_task` monolithic + task-scoped, receiving no `Resolved`; M4's two per-probe severity knobs declared but **read by nothing**; no demotion-lock; `override-default` runs a bespoke `classify(deltas, pack)`). Four `gap-detector`s + an independent `design-reviewer` (which spiked the code) shaped the design ([DECISIONS.md](../DECISIONS.md) 2026-06-04). Settled calls: `Finding` grows structured `(probe, check)` fields (re-pins the JSON golden); severity is assigned by **one engine-owned post-pass** over aggregated findings (validation.md's "engine assigns, probe suggests"), **override-only-on-explicit-delta** so the no-override path is byte-identical; the demotion-lock floor is a **`floor:` field on `KnobDecl`** with an engine load-time assertion that the 11 intrinsic checks are floored; `warning` becomes a live third tier; the `Probe` seam is built **minimally** to fit `override-default` (no rewrite of the byte-stable `validate_task` path). The review's headline catch: the real byte-identity risk is that `task validate`/`finalize` and `jigc upgrade` resolve **no cascade today** and must gain `Resolved` to feed the post-pass — so the no-delta golden must cover those paths, not just compose. Design home of record: [validation.md](../design/validation.md) → Severity assignment / The non-task `Probe` seam / Severity inventory; [overrides.md](../design/overrides.md) → Locked keys; acceptance = [worked-examples.md](../design/worked-examples.md) flow 8. Risk-first, linear: the determinism-trap wiring spine first, the knob-surface + demotion-lock next, the seam retrofit + genuine acceptance last. **Status: planned, not yet built.**

### Increment 1 — `Finding` (probe, check) + the engine-owned severity post-pass (the determinism spine)

**Deliverable:** `Finding` carries structured `(probe, check)`; one engine post-pass assigns severity over the aggregated findings at `ValidationReport` construction (override-only-on-explicit-delta, matched by inventory `(probe, check)` membership, before `has_blocking()`); `Resolved` is threaded into the `task validate`/`finalize` path; the **no-override path is byte-identical** to pre-M6 over compose + validate. Proven on goldens + a unit-tested post-pass (fed a synthetic resolved-override map) — not yet the real pack knob surface.

**Grouped scope:**
- Restructure `Finding` with `probe`/`check` fields (keep `code` for render); **re-pin the JSON projection golden**. Reconcile every emitted code to its inventory `(probe, check)` — including the `override-default` 5-code→3-check mapping ([validation.md](../design/validation.md) → Code-id reconciliation) and the `file-state`/`reconciliation.*` and `schema-conformance.*` codes; codes with no inventory row carry their `(probe, check)` but are **exempt** from the post-pass ([validation.md](../design/validation.md) → matched by membership, never prefix).
- The post-pass: severity assigned inside `ValidationReport::new(findings, &resolved)` (or a contract-required assignment before any read) via the three-step lookup (per-check → per-probe → declared default), overriding **only** when the resolved cascade carries a `scalar-set` for the finding's key; runs before `has_blocking()` ([validation.md](../design/validation.md) → Severity assignment — the M6 post-pass; review B1/S4).
- Thread `Resolved` into `task.rs::validate` / the finalize path so the post-pass has a cascade to read (compose already resolves one).
- Determinism goldens: **no-delta byte-identical** over the existing compose fixtures *and* the `task validate`/`finalize` path (review B2); a post-pass unit test proving a synthetic per-check override flips a finding's severity and `has_blocking()` reflects it.

**Proves:** the wiring spine + the determinism guard — the milestone's #1 risk, retired in isolation before any knob exists.

### Increment 2 — the per-check knob surface + `KnobDecl.floor` + the demotion-lock (soft-rejection)

**Deliverable:** `knobs.yaml` declares the full per-check severity surface (~17 keys, per-probe keys retained as defaults); `KnobDecl` gains `floor`; the engine asserts at pack-load that the 11 intrinsic checks are floored; a below-floor `scalar-set` is **soft-rejected at cascade resolution** (dropped, resolution continues, surfaced via `--explain`) while an undeclared key still hard-aborts. A real-pack tunable demotion now changes a finding's severity end-to-end, and an intrinsic demotion is floor-rejected.

**Grouped scope:**
- `knobs.yaml`: declare the per-check keys with defaults matching the [Severity inventory](../design/validation.md#mvp-check-inventory) (18 checks / 11 intrinsic — the inventory is the SoT); `floor: blocking` on the 11 intrinsic; retain M4's per-probe keys as additive defaults (never a rename); **re-pin the `loaded_knobs_seed…` golden** (review B3).
- `KnobDecl.floor` field + loader + the **load-time intrinsic-floored assertion** over the engine's known intrinsic id set (review M1 hybrid — fails loudly on an unfloored intrinsic check; assertion-only).
- The soft-rejection channel in `cascade::resolve`: severity order `blocking > warning > advisory`; a below-floor delta is **dropped** (value resolves from the remaining layers, not a synthetic floor literal) and recorded on a new **rejected-deltas surface on `Resolved`**; resolution does not abort; an undeclared key stays a hard `UndeclaredScalar` error (review S2).
- `--explain` + `ResolutionTree`: the rejected-demotion line (attempted value · floor · source layer), a new `ResolutionTree` field + projection/`SCHEMA_VERSION` bump ([workflow-dialect.md](../design/workflow-dialect.md) → `--explain` output contract; review S1).
- e2e: a real-key tunable demotion (e.g. `validation.file-state.hash-matches.severity: advisory`) stops that check blocking; an intrinsic demotion is floor-rejected, logged, shown in `--explain`, not applied.

**Proves:** severity is a cascade setting with a demotion-lock floor — the two-tier rule end-to-end over the real pack surface.

### Increment 3 — the non-task `Probe` seam + `override-default` retrofit + flow 8 acceptance (the headline)

**Deliverable:** the minimal non-task `Probe` seam; `override-default` routed through it with the resolved severity threaded into `classify`; `jigc upgrade` gains `Resolved` (its no-delta path byte-identical); `warning` is a live tier (produce/render/exit); the **flow 8 acceptance** through the real binary — an `override-default` conflict demoted `blocking → warning` so `jigc upgrade` **warns instead of blocks** (exit 0), with the tunable-demote / intrinsic-floor-rejection / byte-identical assertions as supporting proof.

**Grouped scope:**
- The non-task `Probe` seam in `probe.rs` (minimal — fits `override-default`'s `(deltas, pack)` ctx; does **not** unify a trait over `validate_task`'s shape); route `classify` through it and thread the resolved severity lookup (`validation.override-default.<check>.severity`) into it; `upgrade_in_repo` builds `Resolved` (+ the no-delta byte-identical golden for the upgrade path, review B2) ([validation.md](../design/validation.md) → The non-task `Probe` seam).
- The `warning` render path (distinct from `advisory`) + exit-code coverage: a `warning` finding flows through report → render → exit as **non-blocking** (review M3); confirm bare `jigc start --explain` resolves `default-workflow` (review M2).
- flow 8 e2e via the `FilesystemPack`/`JIGC_PACK_DIR` seam (reusing M5's `tests/pack_source_determinism.rs` idiom): record an `override-default` conflict, `config set validation.override-default.target-unchanged.severity warning`, `jigc upgrade` warns + exits 0; a tunable demote stops blocking; an intrinsic demote is floor-rejected + shown in `--explain`; the no-override path byte-identical ([worked-examples.md](../design/worked-examples.md) → flow 8).

**Proves:** M6's headline — severity is a cascade setting, never a code change, *including* the M5 `override-default` probe now cascade-tuned on the seam — the mandated acceptance path = [worked-examples.md](../design/worked-examples.md) flow 8.

### Status

All three increments (1–3) complete as of 2026-06-04 — severity tuning works end-to-end: severity is assigned by **one engine-owned post-pass** over aggregated findings (`ValidationReport::new(findings, &resolved)`), **override-only-on-explicit-delta** so the no-override path is byte-identical (proven by captured pre-M6 goldens over compose / validate / finalize / upgrade); the full **18-key per-check knob surface** ships with the **11 intrinsic checks floored**; a below-floor `scalar-set` is **soft-rejected at cascade resolution** (dropped, resolution continues, surfaced on `rejected_demotions` + `--explain`, `SCHEMA_VERSION` 1→2) while an undeclared key still hard-aborts; and the **non-task `Probe` seam** is non-hollow — the M5 `override-default` probe is routed through it from production `jigc upgrade`. The flow-8 headline (`flow8_override_default_warning.rs`) drives a genuine v1→v2 pack change through the real binary, demotes the conflict `blocking → warning`, and exits 0. Built via the [milestone-build](milestone-build) harness (a 21-agent run — **no build halts**, every increment validated clean with **0 fix rounds**). The [milestone-completion audit](milestone-completion-workflow.md) (independent code-review + real-binary e2e) returned **e2e overall-pass with no defects** and 2 LOW code-review notes + 1 advisory, all human-triaged and remediated: (FIXED `8f58708`) deduped the duplicated severity-token parser; (FIXED `771e161`) hardened the demotion-lock so every intrinsic check must be *declared* + floored at pack-load (`KnobError::UndeclaredIntrinsic`), not resting on the closed-surface rule; (FIXED `39a91bb`) corrected the stale inventory tally to 18 checks / 11 intrinsic. The harness bet held: hardening #5/#6 fired *inside* increment 2 (T5's e2e caught floors built-but-unwired to the two production resolve sites — the exact M5-class "planned-but-unwired" gap — and it was fixed before the increment validated), so the milestone audit found zero completeness shortfalls.

## Milestone 7 — milestone execution, the deterministic join core: decomposition

Cut 2026-06-04 via the [`/milestone-plan`](../.claude/commands/milestone-plan.md) command. Three `capability-auditor`s established the baseline (the fan-out/join machinery is **entirely greenfield** — edge-index "site 4" is the only unbuilt lifecycle site; no step-kind concept; `milestone` is rejected as a data-value root); four `gap-detector`s found one keystone gap (the fan-out list-source had no entity behind it); a forward-looking **process review of the harness itself** surfaced a genuinely new **7th failure class** — *single-execution determinism trust* — folded into the gates ([increment-workflow.md](increment-workflow.md) → A second principle); and an independent **design-reviewer** caught two blocking flaws (the clash discriminator was undecidable from recorded state; the write barrier had no M7 mechanism), both **baked out before this cut** ([DECISIONS.md](../DECISIONS.md) 2026-06-04). The milestone was **split along the determinism boundary**: M7 is the engine **data plane** (the deterministic join), and the control plane (spawn binding, `jigc workflow` re-entry, fan-out/join step kinds, the `{{milestone.tasks}}` data-value root, the `milestone-execution` workflow, the genuine real-spawn end-to-end) is **M8**. Design homes: [storage.md](../design/storage.md) → The by-task-id join (the merge algorithm); [validation.md](../design/validation.md) → Fan-out cross-area refs; [structural-grammar.md](../design/structural-grammar.md) → Work-units; [write-commands.md](../design/write-commands.md) → Minting a milestone; acceptance = [worked-examples.md](../design/worked-examples.md) flow 9. Risk-first, linear: the work-unit substrate first, the join's input substrate (provenance + populator) next, the deterministic join itself (the #1 risk) in isolation, then milestone finalize + the genuine permutation acceptance. **Status: planned, not yet built.**

### Increment 1 — the `milestone` work-unit + `create` / `add-task` (the substrate spine)

**Deliverable:** `jigc milestone create "<title>"` mints a `milestone` work-unit (engine-native state, address `milestone:<slug>`) pinned to **one shared base**, with an empty task list; `jigc milestone add-task <milestone-id> "<intent>"` mints a sub-task under it (inheriting the milestone's base, an isolated `tasks/<sub>/` area) and appends it — proven through the binary, with the task list enumerable in deterministic **sorted-task-id** order. No join yet.

**Grouped scope:**
- The `milestone` work-unit in the work-unit family: mint site, on-disk state under `.jigc/`, address `milestone:<slug>` (no fragment), the **single shared base** pinned at create ([structural-grammar.md](../design/structural-grammar.md#work-units-and-runtime-identity), [storage.md](../design/storage.md#the-by-task-id-join-m7)).
- `jigc milestone create` + `jigc milestone add-task` (clap + dispatch); add-task mints a sub-task work-unit pinned to the milestone base, opens its isolated area, appends to the milestone's task list; a serial slug collision within one milestone **rejects** (no suffix at incremental add) ([write-commands.md](../design/write-commands.md) → Minting a milestone).
- The task list as deterministically-ordered engine state — sorted by sub-task id, never `read_dir`/insertion order; a `read_dir`-order ≠ id-order test asserts ordering is by id (Validation hardening #7).

**Proves:** the work-unit family extends to `milestone > task`; N isolated sub-task areas coexist under one shared base; the task list the join will enumerate is deterministically ordered — the substrate, before the join consumes it.

### Increment 2 — stage-time provenance + `add-from-spec` (the join's input substrate)

**Deliverable:** every staged doc in a sub-task area records **`created`** (minted here) vs **`edited-from-base`** (existed at the milestone base, copied in) provenance; `jigc milestone add-from-spec <milestone-id> <spec-addr>` seeds the task list with one sub-task per repeatable `criterion` of a committed spec — proven on fixtures, with a zero-criteria spec a blocking finding.

**Grouped scope:**
- The provenance bit recorded at stage time — `provision_doc` → `created`, `copy_in` → `edited-from-base`, measured against the milestone's shared base — in the staged-doc on-disk representation the join reads ([storage.md](../design/storage.md#the-by-task-id-join-m7) → classification by provenance).
- `jigc milestone add-from-spec` (clap + dispatch): enumerate the committed spec's `criteria` items via the **parse-items** read path (`parse_sections` → the section's `items`, *not* the `jigc task bind` slice), minting one sub-task per item with the criterion text as intent; a spec with **zero criteria** is a blocking *"nothing to seed from"* ([write-commands.md](../design/write-commands.md) → Minting a milestone).
- Tests: a `created` doc and an `edited-from-base` doc in one area carry distinct provenance; add-from-spec over a 3-criteria spec mints 3 sub-tasks; the zero-criteria block fires.

**Proves:** the discriminator the join's clash rule needs is recorded correctly *before* the join depends on it (the design-review B1 fix, in isolation); the spec-seed populator works on the M3 substrate without unverified reuse.

### Increment 3 — the by-task-id join merge (the #1 risk, in isolation)

**Deliverable:** `jigc milestone join <milestone-id>` enumerates the sub-task areas by sorted task id and merges them into the parent working overlay deterministically — disjoint union; provenance-based clash (block two `edited-from-base` writes to one committed-at-base slug + the mixed case; suffix two colliding `created` slugs in task-id order with intra-doc self-ref rewrite); per-sub-area cross-area ref rejection; the join-time isolation check — **proven order-invariant by permutation** over real multi-area fixtures (id / reverse / shuffled → byte-identical merged overlay). Not yet wired to finalize/commit.

**Grouped scope:**
- Edge-index **site 4**: the by-task-id join merge — enumerate sub-areas by sorted task id (`BTreeMap`/sorted `Vec`, never `read_dir`/completion order), fold staged docs into the parent overlay; reuse the single-area working-overlay derivation as the per-area basis ([storage.md](../design/storage.md#the-by-task-id-join-m7)).
- The provenance-based **clash rule** (block) vs **collision-suffix** (distinct `created` instances) + the **self-ref rewrite** of the suffixed instance's *local* self-references ([structural-grammar.md](../design/structural-grammar.md#ids-provenance-and-minting)).
- **Cross-area ref rejection**, run **once per sub-area** (each area's outgoing edges against `committed ∪ that one area`, reusing the single-area `ref_resolves` unchanged — never a merged multi-area overlay) ([validation.md](../design/validation.md) → Fan-out cross-area refs); the **join-time isolation check** (a staged doc not attributable to its own sub-area is blocking). Both intrinsic, floored.
- **Determinism acceptance (Validation hardening #7):** the merge fed the same area set under **≥3 divergent orders** (id, reverse, shuffled-seed) asserts byte-identical output; the fixture **forces genuine overlap** (two same-slug `created`, one self-referential; a cross-area ref; an `edited-from-base` clash) and a `read_dir`-order ≠ id-order layout; no unsorted hash-container iteration reaches output.

**Proves:** M7's #1 risk — the join is a *pure function of the set of sub-task areas*, order-invariant, with correct provenance-clash / collision-suffix / cross-area handling — retired in isolation before the commit path depends on it.

### Increment 4 — milestone finalize + the flow-9 acceptance (the headline)

**Deliverable:** `finalize` over a milestone **materializes** the join's suffix-rewritten doc bodies, then consumes the **post-join, suffix-resolved** overlay and commits it as one boundary with a **CLI-synthesized deterministic commit message** (a structural projection of the milestone id + its id-ordered sub-task list — no authored prose); the full **flow-9** acceptance runs through the real binary end-to-end — a milestone with ≥2 sub-tasks exercising the collision-suffix, self-ref rewrite, cross-area rejection, and same-doc clash, with the join run under multiple completion orders asserting byte-identical committed state (message + tree).

**Grouped scope:**
- **Materialize** the join's merged overlay into the parent area's `docs/` (the join keeps only edges today — re-derive the suffix-rewritten bodies), then feed it into the existing finalize transaction (already suffix-resolved, so the destination-path sort never collides) ([storage.md](../design/storage.md#the-by-task-id-join-m7)).
- **The synthesized commit message** ([DECISIONS.md](../DECISIONS.md) 2026-06-04 — the inc-4 fork resolution): one commit per milestone finalize, message rendered by the CLI from the milestone id + its **id-ordered** sub-task list — deterministic (byte-identical across feed orders), no commit doc, no authored prose; the per-sub-task-commit variant + the `finalize.fan-out.squash` knob stay deferred to M8+ ([finalize.md](../design/finalize.md) → `fan-out` finalize).
- The **flow-9 e2e** in a throwaway repo ([worked-examples.md](../design/worked-examples.md) → flow 9): `jigc milestone create` → `add-task`×N (and an `add-from-spec` path) → populate areas as fixtures → `jigc milestone join` under id/reverse/shuffled orders → assert byte-identical commit; assert suffix-by-task-id, self-ref rewrite, cross-area ref blocked, same-doc clash blocked.

**Proves:** M7's headline — the deterministic join commits reproducibly end-to-end (engine-genuine, permutation-proven) = [worked-examples.md](../design/worked-examples.md) flow 9. Sets up **M8** (the control plane fans real sub-agents into these areas and drives this join).

### Status

All four increments (1–4) complete as of 2026-06-04 — the deterministic join core works end-to-end: a `milestone` work-unit (`milestone > task`, one shared base) with `create` / `add-task` / `add-from-spec` verbs; isolated per-sub-task areas; the **by-task-id join merge** (`jigc milestone join`) folds sub-areas in id-sorted order into the parent overlay — disjoint union, provenance-based clash-block (same-doc `edited-from-base` + the mixed case), collision-suffix on distinct `created` slugs with intra-doc self-ref rewrite, and per-sub-area cross-area ref rejection (`committed ∪ that one area`) + the join-time isolation check; and **`jigc milestone finalize`** materializes the suffix-rewritten bodies and lands **one** commit with a **CLI-synthesized deterministic message** (the inc-4 fork resolution — [DECISIONS.md](../DECISIONS.md) 2026-06-04). The headline **flow-9 permutation acceptance** passes through the real binary: an identical populated fixture finalized under id / reverse / shuffled feed orders yields **byte-identical commit message *and* tree**. Built via the [milestone-build](milestone-build) harness: **one halt** at inc-4 Plan — a genuine fork (the milestone commit-message source) under-specified at Settle, human-settled (synthesized message) and resumed — and **1 fix round total** (inc-1; inc-2/3/4 validated clean with 0). The **independent design-review caught two blocking flaws *in the design* before the build** (the clash discriminator was undecidable from recorded state → one shared base + `created`/`edited-from-base` provenance; the write barrier had no M7 mechanism → join-time isolation check, write-time barrier deferred to M8). The [milestone-completion audit](milestone-completion-workflow.md) returned **e2e overall-pass (every scenario green)** + code-review **"deliverable genuinely holds"** with only **2 LOW edge-case findings**, both **fixed + re-verified through the gate**: (FIXED `f648549`) a suffixed join address could silently overwrite a separate group's same-named address — now a blocking cross-group `join.same-doc-clash`; (FIXED `fd62887`) the join-time isolation check false-positived on an unknown-doctype staged doc — now keyed on physical body presence, not the schema-gated `task_froms`. **The harness bet held off-substrate, and a proactive forward-looking review *added a gate before the failure for the first time*:** the M7-Settle process review surfaced a genuinely new **7th failure class** — *single-execution determinism trust* ([increment-workflow.md](increment-workflow.md) → A second principle) — which the join (inc 3) then passed cleanly (byte-identical across id/reverse/shuffled + 3 process runs), and **no concurrency-class failure slipped to the audit** (the 2 findings are correctness edges, not determinism/isolation-leak/race). The control-plane half — fan-out/join step kinds, the `{{milestone.tasks}}` data-value root, the adapter spawn binding + `jigc workflow` re-entry + the write-time barrier, the `milestone-execution` workflow, and the genuine real-spawn end-to-end — is [M8](#m8--milestone-execution--spawn-binding--acceptance-control-plane).

## Milestone 8 — milestone execution, spawn binding + acceptance: decomposition

Cut 2026-06-04 via the [`/milestone-plan`](../.claude/commands/milestone-plan.md) command. Three `capability-auditor`s established the baseline (**all eight pieces genuinely net-new, nothing half-scaffolded**; the design docs more settled than expected; the decisive finding: **the M7 join's input preconditions are met only by fixture-staging today** — `copy_in` has zero production callers so `edited-from-base`/`same-doc-clash` is unreachable from the binary, and `add-task` mints areas with no `docs/`). Four `gap-detector`s + an independent **design-reviewer** + the **forward-looking harness review** (the conditional Settle phase for an off-substrate milestone) shaped it ([DECISIONS.md](../DECISIONS.md) 2026-06-04). The four pre-loaded ledger items all settled, plus two **new** forks the ledger didn't carry (the finalize-less sub-workflow; the spawn-emit marker convention). The human chose the **fuller M8** at every fork — per-sub-task authored commits + the `finalize.fan-out.squash` knob, the full edit-staging path (`copy_in` into production), the new `jigc workflow <W> --task` verb, the two-half acceptance. The forward review confirmed the M7 lesson again (**no new principle #8 — three new faces of #4/#5/#7** + the structural fact that the genuine spawn is unprovable by any automated gate → an orchestrator-driven completion artifact). Design homes: [workflow-dialect.md](../design/workflow-dialect.md) (step kinds, the `Spawn:` emit class, `{{milestone.tasks}}`, the `milestone-execution`/`sub-task` workflows), [write-commands.md](../design/write-commands.md) (the re-entry verb + the write-time barrier + copy-on-first-touch), [assistant-adapter.md](../design/assistant-adapter.md) (the spawn binding + install-time validation), [finalize.md](../design/finalize.md) (the squash knob), [validation.md](../design/validation.md) (the two new `workflow-refs` checks); acceptance = [worked-examples.md](../design/worked-examples.md) flow 10. Risk-first, linear: the compose-side spine first, the re-entry + the write→join seam (the #1 integration risk) next, the adapter, then the pack + the genuine two-half acceptance. **Status: planned, not yet built.**

**Pack content rides along.** The `sub-task` + `milestone-execution` workflows + their steps accrete in increment 5; the `finalize.fan-out.squash` + the two new `validation.workflow-refs.*` knob declarations in `knobs.yaml` accrete where first needed (increments 5 and 1).

### Increment 1 — `{{milestone.tasks}}` root + `fan-out`/`join` step kinds + the `Spawn:` emit class (the compose-side spine)

**Deliverable:** composing a workflow with a `fan-out` step over `{{milestone.tasks}}` (and a matching `join` step) emits **N deterministic `Spawn:` directives** (one per id-sorted sub-task, each carrying the `jigc workflow <W> --task <id>` re-entry command) — proven on fixtures (a test milestone + a fan-out workflow), not yet a real spawn or the pack workflow.

**Grouped scope:**
- The `milestone` data-value root: a `milestone` field on `ComposeContext` + a resolver arm mirroring `store`'s collection-leaf shape, fed the id-sorted `TaskList::enumerate()` output; `{{milestone.tasks}}` resolves to a collection of bare sub-task ids; `milestone.*` is no longer an `undeclared-root` error ([workflow-dialect.md](../design/workflow-dialect.md) → data-value roots).
- `fan-out`/`join` **step kinds**: `load_step_def` **parses and honors** step front-matter (today it strips-and-discards — a `fan-out:` marker composes green as inert prose); `StepDef` gains a kind (`fan-out {over, run}` / `join {}`); a plain step stays kindless ([workflow-dialect.md](../design/workflow-dialect.md) → On-disk definition format).
- The **`Spawn:` 5th emit class** in `emit_line`: a `fan-out` step resolves its `over:` collection and emits one `` Spawn: `<re-entry-cmd>` `` per id, in id order (the CLI payload; the adapter launch wrapper is increment 4) ([workflow-dialect.md](../design/workflow-dialect.md) → Emitted format).
- `workflow-refs`: the new **`spawn-marker-not-shadowed`** + **`fan-out-join-paired`** checks (intrinsic, floored) + their `knobs.yaml` keys; inventory 18 → 20 / 11 → 13 intrinsic ([validation.md](../design/validation.md) → Severity inventory).
- **Determinism (hardening #7 + the #5 face):** the N `Spawn:` directives are byte-identical and id-ordered across ≥2 divergent orders; a `fan-out` step composes **differently** than the same step with the marker absent (proving the marker is honored, not parsed-but-ignored).

**Proves:** the compose-side fan-out spine — deterministic spawn-directive emission from `{{milestone.tasks}}` — in isolation, engine-testable, before any real spawn or write.

### Increment 2 — `jigc workflow <W> --task <id>` re-entry + `add-task --workflow` + sub-area provisioning (the re-entry spine)

**Deliverable:** `jigc workflow <W> --task <id>` composes `W` for a milestone sub-task, **asserts `W` equals the sub-task's recorded mint workflow**, and **provisions the write-ready area on first entry**; `jigc milestone add-task`/`add-from-spec` gain `--workflow <id>` (default `sub-task`) and record it — proven through the binary on a milestone with sub-tasks.

**Grouped scope:**
- `add-task`/`add-from-spec`: the `--workflow` arg (default `sub-task`) recorded in the sub-task area, replacing the hardcoded `SUB_TASK_WORKFLOW = "single-task"` ([write-commands.md](../design/write-commands.md) → Minting a milestone).
- `jigc workflow` clap verb + dispatch: compose `W` for the named sub-task; the **W-equality guard** (mismatch with the recorded workflow → reject, fails loudly on a stale template); distinct from `jigc start --task` (top-level resume) ([write-commands.md](../design/write-commands.md) → Sub-agent re-entry).
- **Provision-on-first-entry**: the first re-entry provisions the sub-workflow's deterministic instances (the commit doc, etc.) into `tasks/<sub>/docs/` — mirroring `start`'s mint-time provisioning, deferred to first entry so unspawned areas aren't provisioned.

**Proves:** a fanned sub-agent can re-enter, get the *same deterministic composed workflow*, and land in a write-ready isolated area — the re-entry contract, in isolation, before the write path rides on it.

### Increment 3 — the write-time `--task` barrier + copy-on-first-touch (the write→join seam, the #1 integration risk)

**Deliverable:** `jigc doc <verb> --task <id>` scopes staging writes to the named sub-area (a staged-doc destination outside `tasks/<id>/` is rejected at write-time); editing a doc that existed at the milestone base **copies-in → `edited-from-base`**, a `create` stages `created`, provenance recorded **write-once, keyed on base-membership**; so the M7 join's `same-doc-clash` / collision-suffix are reachable from **real `--task` writes** — proven through the binary (real writes feed the existing join, not fixtures).

**Grouped scope:**
- The `--task <id>` selector on `jigc doc create`/`set-field`/`set-slot`/`add-item`; active-task resolution (explicit wins; else single; else reject); the **barrier** (reject a staged-doc destination outside the named sub-area; `bind`'s committed-store target is exempt — it stages nothing) ([write-commands.md](../design/write-commands.md) → The write-time barrier).
- **copy-on-first-touch**: `copy_in` wired into the production `--task` edit path (first write to a base-existing doc copies the committed body, then splices); `provision_commit_doc` records `created` provenance; provenance is write-once and sticky-`created` ([storage.md](../design/storage.md#the-by-task-id-join-m7)).
- **The seam verification (the integration-seam discipline):** assert a real `jigc workflow … --task <id>` + `jigc doc … --task <id>` sequence produces exactly the `docs/<addr>.md` + conforming `provenance.json` the M7 join consumes — replacing flow-9's hand-staging — and that a real `edited-from-base` × `created` overlap triggers the existing `same-doc-clash` block.
- **The #7-barrier face (hardening):** drive a write *through the barrier from the binary* and attempt a write *outside* the named sub-area, asserting it is refused — never pre-stage the area.

**Proves:** M8's #1 integration seam — real `--task`-scoped writes satisfy the M7 join's preconditions with correct provenance — closed through the binary, before the pack workflow drives it.

### Increment 4 — the adapter spawn binding + install-time template validation

**Deliverable:** the Claude Code profile carries a `spawn:` block (the launch template); `jigc setup` / `jigc adapter install` validates it (required placeholders + the required `jigc workflow … --task` invocation pattern + the **decidable** forbidden-pattern rule); a broken template **fails install** with a precise pointer — proven through the binary (setup with a good template + several rejected ones).

**Grouped scope:**
- `AdapterProfile` gains a `spawn` field (the model must stop rejecting it under `deny_unknown_fields`); the launch-template render (`jigc workflow {{workflow}} --task {{task_id}}` → the assistant's Task-tool invocation) ([assistant-adapter.md](../design/assistant-adapter.md) → Bind the spawn mechanism).
- Install-time schema validation: the full decidable rule (both placeholders present; the invocation pattern present; ≤1 newline in the whole rendered template, exactly one backticked span, no `<<author:`/ATX-heading/`Run:`/`Spawn:`/`> ` markers) — run at `setup`/`adapter install`/`regenerate`, a broken template is an install error.
- **The #4 face (hardening):** render the spawn template through the CLI for a real `(workflow, task_id)`, extract the rendered command verbatim, and **execute it as a process**, asserting it resolves to the real `jigc workflow … --task` verb (guards the L1 landmine — a template naming a nonexistent command).

**Proves:** the adapter owns a validated launch (the assistant-specific bit), and the launch line is an executed-bytes contract — the spawn binding, with the L1 landmine guarded at install.

### Increment 5 — the `sub-task` + `milestone-execution` pack workflows + per-sub-task commits + `finalize.fan-out.squash`

**Deliverable:** the pack ships the finalize-less **`sub-task`** workflow and the **`milestone-execution`** workflow (fan-out over `{{milestone.tasks}}` → join → milestone-finalize); `jigc start --workflow milestone-execution --task <milestone-id>` composes the fan-out end-to-end; the **`finalize.fan-out.squash`** knob (default `true`) shapes the commit (one CLI-synthesized aggregate vs per-sub-task authored commits in id order) — proven through the binary on a milestone (driving increments 1–4, sub-agents simulated as N separate binary invocations).

**Grouped scope:**
- Pack: the `sub-task` workflow (`creates-task: true`, `allows-create: [{type: adr, as: decision}]`, no `finalize` step — fan-out-free by construction) + its steps (`locate`/`implement`/`author-commit`); the `milestone-execution` workflow (`creates-task: false`) + the `implement-tasks` (fan-out) / `join-tasks` (join) / `milestone-finalize` steps; a `when` hint ([workflow-dialect.md](../design/workflow-dialect.md) → On-disk definition format).
- `finalize` fan-out: the **`finalize.fan-out.squash`** knob + both modes — `true` → one aggregate CLI-synthesized commit (the M7 form); `false` → one commit per sub-task in id order rendering each authored commit doc, plus the parent's — each **byte-identical / id-ordered across feed orders** ([finalize.md](../design/finalize.md#fan-out-finalize)); `knobs.yaml` declares the knob.

**Proves:** the end-to-end pack composition — a real milestone fanned out, recombined by the M7 join, and committed under both squash modes deterministically — everything but the genuine assistant launch.

### Increment 6 — the two-half acceptance (flow 10, the headline)

**Deliverable:** **Half A** (automated `cargo test`) — `milestone-execution`'s fan-out emits N `Spawn:` directives deterministically, the test invokes `jigc workflow <W> --task <id>` as **N separate binary processes** through the real write→join, asserting **byte-identical finalize** across ≥2 divergent orders, and renders+executes the spawn template (the L1 guard); **Half B** (recorded, orchestrator-driven) — a **genuine concurrent Claude-Code-Task-tool spawn** of ≥2 sub-agents into isolated areas, recombined by the join, with the committed **tree-hash asserted to match Half A's golden** and the transcript **witnessing the blackboard invariant** — a milestone-completion **audit artifact, not a CI gate**.

**Grouped scope:**
- The flow-10 Half-A e2e in a throwaway repo: the N-process sim through the real binary over the full `create`/`add-from-spec` → `milestone-execution` → join → finalize arc ([worked-examples.md](../design/worked-examples.md) → flow 10).
- The Half-B procedure: documented + run by the orchestrator at milestone completion (the genuine spawn + tree-hash match + blackboard witness), per [milestone-completion-workflow.md](milestone-completion-workflow.md) → the spawn-class artifact. **The milestone is not shippable until the Half-B artifact exists and matches** (a missing artifact is a blocking completion finding — the hollow-spawn trap).

**Proves:** M8's headline — **parallel sub-agent work stays reproducible under the blackboard model end-to-end**: the CLI emits spawn instructions deterministically, the adapter launches real concurrent sub-agents, the M7 join recombines them into one reproducible commit. Closes the milestone-execution primitive.

### Status

Planned 2026-06-04; not yet built. Cut into six risk-first increments after the settled design survived the independent pre-decompose review (two blocking findings + five advisories, all baked before this cut — see [DECISIONS.md](../DECISIONS.md) 2026-06-04). Built via the [milestone-build](milestone-build) harness, hardened this cycle with the **M8 faces** of validation principles #4/#5/#7 and the spawn-class completion artifact ([increment-workflow.md](increment-workflow.md) → M8 faces; [milestone-completion-workflow.md](milestone-completion-workflow.md)).
