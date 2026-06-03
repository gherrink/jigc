# Roadmap

The full-product build order — **the milestone spine**, risk-first, each milestone a runnable slice that *proves* one thing. This is sequencing, not design: each entry names what it proves and the [VISION](../VISION.md) workflow(s) it lands; the milestone-level *what* and *why* live in the design docs. For scope and the determinism boundary see [CLAUDE.md](../CLAUDE.md); for the *why* of the ordering, [DECISIONS.md](../DECISIONS.md).

This doc owns the **milestone spine** plus the increment decomposition of milestones *already planned*. A milestone is turned into ordered increments by the [milestone-planning workflow](milestone-planning-workflow.md) **when it is picked up** — never all milestones up front — so M2–M7 below stay milestone-only until then, while M1 (shipped) carries its full decomposition. Work-unit terms are `milestone > increment > task` ([structural-grammar.md](../design/structural-grammar.md) → Work-units); **tasks are not enumerated** — they're cut per-increment at pickup, so the list stays honest against what prior increments produced. *How* an increment goes from "not started" to "validated and committed" is the [increment workflow](increment-workflow.md); how a milestone *opens* and *closes* are the [planning](milestone-planning-workflow.md) and [completion](milestone-completion-workflow.md) workflows.

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
The other half of principle #5: the engine-native **`override-default` probe** + a guarded **`jigc upgrade`** that re-applies each recorded delta against a new pack version and reports **clean / conflict / orphaned** (stateless `base-hash` compare; on conflict the agent drafts a 3-way merge **through the CLI**, the human confirms, the CLI never calls a model), plus the **severity-floor / demotion-lock** machinery the probe's tunable checks require ([overrides.md](../design/overrides.md) → Upgrade reconciliation, [validation.md](../design/validation.md)). **Proves:** principle #5's *inherit-upstream* half — *inherit every upstream improvement for free; every divergence surfaces at a known moment* — entirely unbuilt in code today (no `upgrade` command, no probe, no base-hash plumbing, no severity cascade). Rides on M4's recorded-delta substrate — hence immediately after it.

### M6 · milestone execution (fan-out / join)
The single bounded concurrency primitive — `fan-out`/`join` steps, isolated per-task working areas, a deterministic join **by task id, not completion order** — and the `milestone-execution` workflow that drives it. **Proves:** parallel sub-agent work stays reproducible under the blackboard model. The **highest architectural risk**, but an isolated add-on that rides on the proven read/write/spec/override substrate — hence after M5. VISION: `milestone execution`.

### M7 · project setup (new & existing)
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

Planned 2026-06-03; not yet built. Hand off to the [milestone-build](milestone-build) harness.
