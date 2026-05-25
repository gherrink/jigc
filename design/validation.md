# Validation engine

The moat: **one deterministic engine runs read-only probes over a scope and reports severity-classified findings.** It is the *validate* stage of the loop and the gate behind `finalize`.

Builds on [VISION.md](../VISION.md) principle #6, [structural-grammar.md](structural-grammar.md) (the edge index, the validation framing, override), [document-type-schema.md](document-type-schema.md) (the relations probes walk), [write-commands.md](write-commands.md) (`finalize ≡ validate + commit`, severity-as-cascade, the `reconcile` route), [storage.md](storage.md) (edge index, effective state), and [workflow-dialect.md](workflow-dialect.md) (findings feed composition; the `workflow-refs` probe). For the *why*, see [DECISIONS.md](../DECISIONS.md). Scope: the framework + the two engine-native MVP probes; `doc-code` is pinned at contract level only (`override-default` is fully specified in [overrides.md](overrides.md)). Notation is **illustrative**.

## What validation is — and isn't

Validation is **the determinism boundary applied to correctness**: the CLI-owned half of a project (references, structure, state) can be *mechanically verified*; the LLM-owned half (prose) cannot. So the engine checks exactly the structured half and **never judges the prose**.

- **Mechanical, never semantic** — deterministic predicates, no LLM in the engine (it makes no model calls). There is no "is this spec well-written?" probe. The moat is *guaranteed referential integrity*, not content quality.
- **Detect & route, never auto-fix** — the engine reports what's wrong and a direction to repair; it never authors the fix.
- **Read-only & deterministic** — probes never mutate; same effective state → same findings.

*Honest boundary* ([VISION.md](../VISION.md) principle #6): this catches **structural/syntactic** breakage (dangling refs, missing coverage mappings, drift), not **semantic** drift (upstream changing the meaning around an untouched reference). "Always knowing what's stale" is the achievable, valuable promise.

**Where it sits:** the engine runs at **`validate` / `finalize`**. The per-write field-type / enum / slug checks are the schema's *local adjudication* — a separate fast path at write-time, not the validation engine ([write-commands.md](write-commands.md): write-time vs finalize-time checks).

## The engine / probe boundary

**Fat engine, thin probes.** The engine owns everything reusable; a probe is the narrowest domain predicate.

| the **engine** provides | a **probe** declares / implements |
|---|---|
| scope → target resolution (task / doc / store) | an **id** (`workflow-refs`, `file-state`, `doc-code`, …) |
| the read-only **effective-state graph + edge index** | a **target-type** (any-doc / a doc-type / workflow / file / override) |
| **scheduling** probes over targets (independent, parallelizable) | a **default severity** (blocking / advisory) |
| **severity assignment via the cascade**, aggregation, report | a read-only **`check(target, ctx) → [finding]`** |
| the **`finalize` gate** | — |

**Severity is engine-owned, via the cascade.** A probe returns a finding with a *suggested default*; the engine assigns the **final** severity from `project > team > pack-default`. This is the only design that honors the locked "severity is a cascade setting" — a project promotes or demotes a check (e.g. `doc-code.missing-test: advisory`) as a recorded override delta, never a code change. Self-classify (severity baked into probe code) is rejected for exactly that reason.

## Findings

A finding is `{ target, probe, severity, message, route? }`:

- **target** — the address it concerns (`spec:auth-flow#criteria/rate-limit`)
- **probe** — who raised it · **severity** — engine-assigned via cascade · **message** — human-readable (always present)
- **route** — an *optional* machine-actionable repair direction, a tagged union the engine **never executes**:
  `fill-leaf {address}` · `run-command {command-ref}` · `reconcile {target}` · `none`

```text
{ target:   spec:auth-flow#criteria/rate-limit,
  probe:    doc-code,
  severity: blocking,                          # from cascade; probe default was blocking
  message:  "criterion maps to no test",
  route:    fill-leaf spec:auth-flow#criteria/rate-limit/maps-to-test }
```

Because findings are **navigable state**, a `fix-drift` workflow can `fan-out` over `{{store.findings}}` and route each to its repair — *validate → compose a repair workflow → agent authors the fix → re-validate*. The loop closes while the engine still only **detects and routes**. The route is a *direction*, not a guarantee the repair is correct or complete — the agent still reasons and authors — and `none` is fine where there's no mechanical route.

## Scope = effective state

`validate` runs over a scope, and it checks the **effective state**, not committed-only:

- **task** (what `finalize` gates on) — `committed + the task's working deltas + the referential blast radius`. The engine overlays the task's deltas on the committed edge index and runs probes over the prospective merged graph. **Blast-radius is required**: if a task removes a ref target, the now-dangling referrers it didn't directly edit are flagged too (cheap — walk inbound edges), so a task can't commit breakage elsewhere.
- **doc** — a single doc and its immediate references.
- **store** — the whole committed state (a health check).

**Integrity vs completeness.** The per-task `finalize` gate blocks only on **integrity the task can fix** — forward-ref resolution, required slots, malformed values. **Completeness obligations** (inverse/minimum-cardinality — "every PRD needs a SPEC") depend on *other* tasks, so they are **advisory by default and hard-enforced only at `store`/milestone scope**, never the per-task gate — otherwise legitimate PRD-first / SPEC-later authoring would deadlock. (Severity and scope are cascade-tunable as usual.)

## Probes

One uniform interface (`check(target, ctx) → [finding]`), two implementations:

- **Engine-native** — built in, in-process. The MVP ships:
  - **`workflow-refs`** (target: workflow) — every `placeholder` / `include` / `command-ref` resolves, and no include cycles.
  - **`file-state`** (target: file) — the on-disk content hash matches the recorded state; drift → a `reconcile` finding. With no recorded hash (first run / fresh checkout), the current on-disk content is adopted as the baseline — absent-hash is not drift.
  - **`override-default`** (target: override) — per-delta `clean / conflict / orphaned` reconciliation on a guarded upgrade; full logic in [overrides.md](overrides.md).
- **Pack probes** — *deferred*, the "not a public API yet" line. An **invoked process** with a JSON-in / JSON-out contract: language-neutral, **read-only and deterministic by contract** (`doc-code` must parse real code, so it can't be declarative). The development pack's **`doc-code`** (does this symbol exist? does a test cover this criterion? did referenced code change after the doc's timestamp?) lands here, after the doc-creation flows exist.

## How it gates `finalize`

`finalize` ≡ `validate(task) + commit`: one engine, two entry points, so what `validate` reports and what `finalize` blocks on can never diverge. `finalize` blocks on **blocking-class findings within the task scope**; advisory findings are surfaced but don't stop the commit. `finalize` defaults to autonomous (a cascade-settable confirm-gate aside), with git/PR as the durable correction point.

## Open questions

- **Pack-probe sandboxing** — pack probes are arbitrary repo-reading code; enforcing read-only/determinism is a real concern, deferred with the pack-probe API (moot for the MVP, where no external probe runs).
- **`doc-code` logic** — the development pack's symbol/test/timestamp checks.
- **Findings: recomputed vs cached** — leaning recompute-on-demand (derived state, like composed workflows), but not yet decided.
