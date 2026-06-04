# Validation engine

**One deterministic engine runs read-only probes over a scope and reports severity-classified findings.** It is the *validate* stage of the loop and the gate behind `finalize`.

Builds on [VISION.md](../VISION.md) principle #6, [structural-grammar.md](structural-grammar.md) (the edge index, the validation framing, override), [document-type-schema.md](document-type-schema.md) (the relations probes walk), [write-commands.md](write-commands.md) (`finalize ≡ validate + commit`, severity-as-cascade, the `reconcile` route), [storage.md](storage.md) (edge index, effective state), and [workflow-dialect.md](workflow-dialect.md) (findings feed composition; the `workflow-refs` probe). For the *why*, see [DECISIONS.md](../DECISIONS.md). Scope: the framework + the two engine-native MVP probes; `doc-code` is pinned at contract level only (`override-default` is fully specified in [overrides.md](overrides.md)). Notation is **illustrative**.

## What validation is — and isn't

Validation is **the determinism boundary applied to correctness**: the CLI-owned half of a project (references, structure, state) can be *mechanically verified*; the LLM-owned half (prose) cannot. So the engine checks exactly the structured half and **never judges the prose**.

- **Mechanical, never semantic** — deterministic predicates, no LLM in the engine (it makes no model calls). There is no "is this spec well-written?" probe. The integration advantage is *guaranteed referential integrity*, not content quality.
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
| **scheduling** probes over targets (independent, parallelizable) | a **default severity** (blocking / warning / advisory) |
| **severity assignment via the cascade** (the M6 post-pass below), aggregation, report | a read-only **`check(target, ctx) → [finding]`** |
| the **`finalize` gate** | — |

**Severity is engine-owned, via the cascade.** A probe returns a finding with a *suggested default*; the engine assigns the **final** severity from `project > team > pack-default`. This is the only design that honors the locked "severity is a cascade setting" — a project promotes or demotes a check (e.g. `doc-code.missing-test: advisory`) as a recorded override delta, never a code change. Self-classify (severity baked into probe code) is rejected for exactly that reason. **Wired in M6** ([Severity assignment — the M6 post-pass](#severity-assignment--the-m6-post-pass)); pre-M6 every finding carries its hardcoded default and no cascade read happens.

## Findings

A finding is `{ target, probe, severity, message, route? }`:

- **target** — the address it concerns (`spec:auth-flow#criteria/rate-limit`)
- **probe** — who raised it · **severity** — engine-assigned via cascade · **message** — human-readable (always present)
- **route** — an *optional* repair direction the engine **never executes**. **As built, `route` is a free `Option<String>`** (a human-readable prose direction; the pinned `Finding` JSON envelope). The tagged-union form below — `fill-leaf {address}` · `run-command {command-ref}` · `reconcile {target}` · `none` — is the **aspirational** target shape; promoting `route` to it is a deferred cross-cutting result-contract change (it would re-shape the pinned golden and every route producer), **not** assumed by any current probe (M5's `override-default` emits a `String` route):
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

**Forward-ref resolution — the two reachable surfaces.** "Forward-ref resolution" covers refs whose targets sit in *exactly* two places: (a) the **committed store** (already on disk), or (b) the **same task's working area** (created by deltas in the same task — resolved against the working overlay, [storage.md](storage.md) → Edge index lifecycle). A ref whose target is in *neither* — that is, a ref to a doc that doesn't exist anywhere reachable at finalize time — is a **blocking integrity error**. **Cross-task forward-refs are not supported in MVP** (or post-MVP without an explicit provisional-ref feature): a ref to a doc *another* task plans to create cannot be resolved by *this* task's finalize, so it fails. The failure is the design — the deadlock Codex's review walked (T1 authors `supersedes: adr:b` where B will be created in T2) is rejected at its source: the agent never authors a cross-task forward-ref; if it does, finalize blocks. The error message names the three routing options: **(i)** order the tasks so the target exists first (T2 before T1); **(ii)** move the target's creation into this task; **(iii)** drop the ref. No new syntax (no `?` provisional marker), no new state machine — easier to relax later if a real cross-task pattern needs it than to ship speculative grammar now. The integration-advantage's mechanical floor (forward-ref resolution at finalize) holds without exception.

**Fan-out cross-area refs follow the same policy.** A sub-agent in sub-task B authoring a ref into sub-task A's working area (e.g. typing `supersedes: adr:cache` while guessing A's slug) is the *fan-out variant* of the cross-task forward-ref case. Same rule, different surface: each sub-task's outgoing refs are validated at **`join`** against `(committed store ∪ this sub-task's own working area)` only; cross-area refs are rejected as blocking integrity errors ([workflow-dialect.md](workflow-dialect.md) → `fan-out` / `join`). The two surfaces — cross-task at finalize, cross-area at join — close the boundary against the same failure mode with symmetric rules.

> **Enforcement is a *narrowing*, not the existing walk re-pointed (M7).** The join validation entry point is built in M7 ([storage.md](storage.md#the-by-task-id-join-m7)). The danger: forward-ref resolution today keys on **filesystem-path existence** in the task's `docs/`, so once N sub-areas physically coexist on disk, a naïve overlay that unions every sub-area's `docs/` would make a cross-area ref *resolve clean* — silently defeating this rule. So resolution must be scoped **per-`from`**: each outgoing ref resolves only against `committed ∪ the area that authored that `from`*, never the flattened multi-area view. **Mechanism:** the join invokes the existing single-area forward-ref walk **once per sub-area**, passing that area's own `task_dir` — so the narrowing is simply "call per area," sibling areas are never in any single call's surface, and the single-area walk stays authoritative and unchanged. At finalize the sibling areas aren't reachable; at join they are — so this is the one case where the surface must be *actively excluded*, not merely "not added." The check is **intrinsic** (floored at `blocking`, M6 floor machinery — [Severity inventory](#mvp-check-inventory)).

## Probes

One uniform interface (`check(target, ctx) → [finding]`), two implementations:

- **Engine-native** — built in, in-process. **The MVP ships two:**
  - **`workflow-refs`** (target: workflow) — every `placeholder` / `include` / `command-ref` resolves, no include cycles, no `@` on a scalar-resolving path, no step's instruction prose shadows the composer-reserved `Run: ` line-start ([workflow-dialect.md](workflow-dialect.md#emitted-format)), and workflow bodies contain only `{{include}}` lines / blank lines / HTML comments at top level ([workflow-dialect.md](workflow-dialect.md#on-disk-definition-format)).
  - **`file-state`** (target: file) — the on-disk content hash matches the recorded state; drift → a `reconcile` finding consumed by the reconciliation classifier ([reconciliation.md](reconciliation.md)). With no recorded hash (first run / fresh checkout), the current on-disk content is adopted as the baseline — absent-hash is not drift.

  **Engine-native, full logic in M5:**
  - **`override-default`** (target: override) — per-delta `clean / conflict / orphaned / needs-rebasing` reconciliation, run by `jigc upgrade`. **M5** shipped it via a **bespoke upgrade-time path** (a direct `classify(deltas, pack)` call, not `validate_task`) with **blocking-by-default** hardcoded severities. **M6** moves it onto the **non-task `Probe` seam** ([The non-task `Probe` seam](#the-non-task-probe-seam-m6)) and makes its three checks **cascade-tunable** via the M6 post-pass — it is the seam's first (and this milestone's only) real consumer, the proof the seam is non-hollow. Findings carry a **human-readable `String` route** (not the tagged union below). Full logic in [overrides.md](overrides.md) → Upgrade reconciliation.
- **Pack probes** — *post-MVP*, the "not a public API yet" line. An **invoked process** with a JSON-in / JSON-out contract: language-neutral, **read-only and deterministic by contract** (`doc-code` must parse real code, so it can't be declarative). The development pack's **`doc-code`** (does this symbol exist? does a test cover this criterion? did referenced code change after the doc's timestamp?) lands here, after the doc-creation flows exist. **"Deterministic by contract" is locked, not aspirational** — see [Pack-probe determinism contract](#pack-probe-determinism-contract) below for the six rules the subprocess implementation must satisfy and the four meta-finding modes that surface every misbehavior; the implementation (OS-level sandboxing) is deferred with the contract as binding requirement.

## Pack-probe determinism contract

Pack probes are subprocess invocations behind a JSON-in / JSON-out contract; their findings feed `validate` and `finalize`, so they must satisfy the determinism boundary the engine itself satisfies. **"Deterministic by contract" is locked, not aspirational** — when subprocess probes ship post-MVP, the implementation must satisfy the rules below. Implementation (OS-level sandboxing — seccomp, Landlock, or equivalents) is deferred, but cannot ship without satisfying the contract. Codex's Pass 3 invariant attack is closed by making the rules explicit and forward-binding.

### The six rules

A subprocess pack probe must:

1. **No network access** — no TCP/UDP/Unix sockets to external endpoints. The probe runs against the local repo and its scratch dir; nothing else.
2. **No model calls** — no LLM, local or remote. The engine's no-LLM rule ([VISION.md](../VISION.md) principle #1) extends to its probe interface; a probe that calls a model is an engine-extension that bypasses the determinism boundary.
3. **No time / random reads** — no `time()`, `/dev/urandom`, `random()`, etc. Same input + same repo → same output, always. Deterministic randomness seeded by the probe input (e.g. a hash-derived seed) is allowed.
4. **Read-only filesystem outside scratch** — may read the repo and the cascade-resolved schema; may write only to a probe-supplied scratch dir; the engine cleans the scratch dir between invocations. No persistent state across runs.
5. **Bounded resources** — engine-set CPU-time, memory, and output-size limits. A runaway probe is killed and surfaces as a `timeout` failure.
6. **JSON-in / JSON-out, nothing else** — stdin = JSON request; stdout = JSON findings; stderr = diagnostic logging only (for human debugging). No shelling out to other processes — each shell-out would be a separate determinism review that the contract intentionally precludes.

### The wire contract — request / response

The wire shape below is **illustrative and minimal**. What stays open is the *concrete* schema (versioning policy, finding envelope, snapshot format); what's locked is the **seam that makes the `Probe` trait's "zero engine change" claim true**. The in-process trait is `check(target, ctx) -> Vec<Finding>`; for a subprocess impl to satisfy the *same* trait with no engine change, **`ctx` must be effective-state the engine can serialize** — in-process and subprocess impls then consume one logical input, a live graph in one case and its serialized projection in the other. A `ctx` defined as a live in-memory handle would force a new engine mechanism when subprocess probes ship — exactly the foreclosure this rules out.

**Effective-state access falls out of the six rules.** A subprocess can hold no live graph handle (no callbacks, rule 6) and reach the engine over no socket (no network, rule 1), so the engine **materializes a read-only effective-state snapshot** in the probe's read scope (rule 4 permits read-only fs) and the request carries a **path-ref** to it. The probe reads the snapshot; it never calls back.

```
request:   { probe_id, target: <address>, effective_state: { snapshot_path }, config: { … }, schema_version }
response:  { findings: [ { target: <address>, severity?, message, route? } ], schema_version }
```

- **`target`** (request and each finding) uses the one address grammar ([structural-grammar.md](structural-grammar.md#addressing)).
- **`severity?`** is advisory and usually omitted — the engine assigns the final severity via the cascade (severity is engine-owned, never baked into a probe). Each finding is the engine's one `finding` shape ([Findings](#findings)).
- **`schema_version`** on both sides is present *as a field* now so the contract can evolve without silently breaking a probe built against an older engine; the *versioning policy* itself is deferred (below).

### Failure semantics — meta-findings

A probe that misbehaves surfaces its failure as a **meta-finding**; the engine never crashes. Every meta-finding is **intrinsic blocking** — a probe that timed out, crashed, returned malformed output, or violated its sandbox can't be trusted to have validated anything, and a missing finding could mask a real integrity error:

| failure | meta-finding |
|---|---|
| **timeout** (probe exceeded the time budget) | `probe-failure { probe, reason: timeout }` |
| **crash** (non-zero exit, no JSON output) | `probe-failure { probe, reason: crash, exit-code }` |
| **malformed JSON output** | `probe-failure { probe, reason: malformed-output, parse-error }` |
| **sandbox violation** (blocked network call, fs write outside scratch, etc.) | `probe-failure { probe, reason: sandbox-violation, detail }` |

The engine surfaces every meta-finding through the same `finding` shape ([Findings](#findings)) — the probe's user-facing failure looks like any other finding to the agent.

### MVP scope

MVP ships no subprocess pack probes, so no meta-findings are surfaced in MVP. The contract is locked here to **bind the post-MVP implementation**. When subprocess probes ship, the severity inventory gains a `pack-probe-integrity.*` category mirroring the four failure modes (parallel to the `doc-code.*` placeholder), each intrinsic blocking. Implementation details — sandbox tech, exact resource limits, and the *concrete* request/response schema (versioning policy, finding envelope, snapshot format) — land with the implementation; the **wire seam** ([The wire contract](#the-wire-contract--request--response)) is sketched so the trait's zero-engine-change claim is checkable now, and the contract above is the floor the implementation must stand on.

## Severity inventory

Every MVP check ships with a **declared default severity** and an **intrinsic-or-tunable** classification. The cascade can promote or demote a *tunable* check's severity; it cannot demote an *intrinsic* check below `blocking`.

### The two-tier rule

- **Intrinsic** — the determinism boundary cannot survive demotion. Locked at `blocking`. **Mechanically (M6): the check's knob carries `floor: blocking`** ([overrides.md](overrides.md) → Locked keys; the per-key floor is a `KnobDecl` field). The floor *value* and severity *assignment* are pack/cascade-driven, but the engine **also asserts at pack-load that its known intrinsic check-id set each carries `floor: blocking`** — a mis-declared pack (an intrinsic check left unfloored) fails loudly at load rather than silently un-locking. This is legitimately engine knowledge: the intrinsic checks *are* the engine's own load-bearing invariants ([What "intrinsic" means mechanically](#what-intrinsic-means-mechanically)), and the assertion is *assertion-only* — it never assigns severity or ships pack content, so the engine-empty invariant holds. A `scalar-set` attempting to set the key below its floor is **soft-rejected at cascade resolution** (the offending delta is dropped, not applied; resolution continues; visible via `--explain`). This is distinct from setting an *undeclared* key, which stays a hard resolution error.
- **Tunable** — cascade can promote or demote freely (its knob declares no `floor`, or a floor it sits above). Use a `scalar-set` keyed `validation.<probe>.<check>.severity: <level>` for per-check tuning, or `validation.<probe>.severity: <level>` to set a default for every check under a probe. **Per-check wins** via a deliberate three-step lookup (M6): the post-pass resolves a finding's severity by trying its per-check key, then the per-probe key, then the knob's declared default — *not* plain exact-key resolution (which has no prefix fallback).

### MVP check inventory

| probe / category | check id | default severity | intrinsic? | cascade key |
|---|---|---|---|---|
| **`workflow-refs`** | `placeholder-resolves` | blocking | **yes** | `validation.workflow-refs.placeholder-resolves.severity` |
| | `include-resolves` | blocking | **yes** | `validation.workflow-refs.include-resolves.severity` |
| | `command-ref-resolves` (target: [command catalog](command-catalog.md)) | blocking | **yes** | `validation.workflow-refs.command-ref-resolves.severity` |
| | `include-cycle-absent` | blocking | **yes** | `validation.workflow-refs.include-cycle-absent.severity` |
| | `at-marker-on-non-scalar` | blocking | **yes** | `validation.workflow-refs.at-marker-on-non-scalar.severity` |
| | `run-marker-not-shadowed` | blocking | **yes** | `validation.workflow-refs.run-marker-not-shadowed.severity` |
| | `body-include-only` (workflow body contains only `{{include}}` / blanks / HTML comments at top level) | blocking | **yes** | `validation.workflow-refs.body-include-only.severity` |
| **`schema-conformance`** *(synthetic — schema-driven integrity at validate / finalize)* | `ref-resolves` (forward-ref integrity) | blocking | **yes** | `validation.schema-conformance.ref-resolves.severity` |
| | `required-slot-present` | blocking | **yes** | `validation.schema-conformance.required-slot-present.severity` |
| | `required-field-present` | blocking | **yes** | `validation.schema-conformance.required-field-present.severity` |
| | `field-value-conformant` (date / enum / string-shape) | blocking | **yes** | `validation.schema-conformance.field-value-conformant.severity` |
| **`file-state`** | `hash-matches` (drift → `reconcile` route, [reconciliation.md](reconciliation.md)) | blocking | no — tunable | `validation.file-state.hash-matches.severity` |
| **`schema-completeness`** *(synthetic — completeness, not integrity; runs at store / milestone scope by default)* | `inverse-cardinality` | advisory at task / blocking at store | no — tunable | `validation.schema-completeness.inverse-cardinality.severity` |
| **`override-default`** *(full logic M5; blocking-by-default, cascade-tunable from M6 — [overrides.md](overrides.md))* | `target-exists` (emits `orphaned`) | blocking | no — tunable (M6) | `validation.override-default.target-exists.severity` |
| | `target-unchanged` (emits `conflict`) | blocking | no — tunable (M6) | `validation.override-default.target-unchanged.severity` |
| | `basis-recorded` (emits `needs-rebasing` — a content delta with no base-hash) | blocking | no — tunable (M6) | `validation.override-default.basis-recorded.severity` |
| **`commit-rendering`** *(advisory-by-default convention checks for commit doc → git message, [finalize.md](finalize.md))* | `line-limit-subject` (72ch) | advisory | no — tunable | `validation.commit-rendering.line-limit-subject.severity` |
| | `line-limit-body` (72ch wrap) | advisory | no — tunable | `validation.commit-rendering.line-limit-body.severity` |
| **`doc-code`** *(pack-provided, post-MVP — placeholder)* | TBD per check | TBD | no — tunable | `validation.doc-code.*.severity` |

**18 MVP-shipping checks across 6 categories.** Roughly half intrinsic (load-bearing for composition + integration advantage), half tunable. **This table is the single source of truth** for the check-id set, the per-check default severity, and intrinsic-ness — `knobs.yaml` and the engine's emitted `check` ids must agree with it. The count is **18 checks / 11 intrinsic** — `workflow-refs.*` ×7 + `schema-conformance.*` ×4 (the 11 intrinsic) + `file-state.hash-matches` + `schema-completeness.inverse-cardinality` + `override-default.*` ×3 + `commit-rendering.*` ×2 (the 7 tunable). (Earlier "16/10" then "17/11" tallies were stale as the inventory grew; `knobs.yaml` declares exactly these 18 keys.)

**Inventory = the keyed (tunable + floored) surface, not every finding the engine emits.** A finding carries a `(probe, check)` only some of which are inventory rows; the rest are **un-keyed and exempt from the post-pass** (they keep their emitted severity). Two distinct kinds of un-keyed finding, not to be conflated:
- **Informational outcomes — emitted advisory, intentionally not tunable.** `file-state.baseline-adopt` and `reconciliation.absorb`: not pass/fail checks, just routing notes. No row, stay advisory.
- **Blocking-but-untunable outcomes.** `reconciliation.rename` / `reconciliation.conformance-block` / `reconciliation.conflict-block`: genuine blocking failure modes that are *deliberately* not exposed as a tunable knob (distinct from `file-state.hash-matches`, which *is* tunable). No row, stay blocking — so demoting `hash-matches` to advisory does **not** silently disable a conflict-block on the same file.

Only checks a project should be able to tune, or that must be floored, get an inventory row + a `knobs.yaml` key.

**Code-id reconciliation (M6) — the built `override-default` codes map many-to-one onto the three canonical checks** (the build aligns the emitted `check` field to the table id; the descriptive `code` is retained for rendering):

| built code (M5) | emits | → inventory `check` | tunable key |
|---|---|---|---|
| `scalar-set-orphaned` | orphaned | `target-exists` | `validation.override-default.target-exists.severity` |
| `slot-fill-orphaned` | orphaned | `target-exists` | *(same key)* |
| structural target-missing | orphaned | `target-exists` | *(same key)* |
| `content-changed` | conflict | `target-unchanged` | `validation.override-default.target-unchanged.severity` |
| `needs-rebasing` | needs-rebasing | `basis-recorded` | `validation.override-default.basis-recorded.severity` |

The three orphan emissions **deliberately collapse to one `target-exists` knob** — a project tunes "an override's target vanished" as one thing, not per-delta-kind (a minimality call; per-orphan-kind tuning is not a use case). `content-changed`→`target-unchanged` and `needs-rebasing`→`basis-recorded` are renames flow 8's headline key depends on.

### What "intrinsic" means mechanically

An intrinsic check is one whose demotion would break a load-bearing invariant of the system itself:

- **`workflow-refs.*`** — composition would emit broken / ambiguous / cyclic output to the agent. The structural-determinism bet fails at the surface where compliance actually happens.
- **`schema-conformance.*`** — the four schema-driven integrity checks at finalize. Forward-ref resolution is the integration advantage's mechanical floor; required-slot / required-field presence is what makes a schema meaningful at all; field-value conformance is the field-type adjudication that distinguishes structured values from prose.

Tunable checks protect themselves with their *route* and *default severity*, not with lock semantics: `file-state` drift routes to reconciliation (so demotion just means "absorb silently" — still safe); `inverse-cardinality` is advisory at task scope by design (because completeness depends on other tasks); commit line-limits are conventional and a project may rationally not want them enforced.

### Synthetic categories

`schema-conformance`, `schema-completeness`, and `commit-rendering` are **synthetic probe categories** — not literal probes with a `check(target, ctx) → [finding]` implementation, but namespaces for cascade-key consistency. The engine runs these checks as part of other pipelines (the parse + schema-validate path for `schema-conformance`, the edge-index walk for `schema-completeness`, the commit-doc renderer for `commit-rendering`). The cascade key naming is uniform regardless of whether a check sits in a literal probe or a synthetic category — a project tuning severity doesn't need to know the implementation detail. **This is exactly why severity assignment is a post-pass** (below): a synthetic check has no `check()` site to thread a cascade read into, but it still produces a `Finding` carrying a `(probe, check)`, so the engine's aggregate post-pass tunes it identically to a real probe's finding.

## Severity assignment — the M6 post-pass

Severity is assigned in **one engine-owned pass over the aggregated findings**, *after* every probe and synthetic pipeline has run — never threaded into each probe site. This is what "the engine assigns, the probe suggests" means concretely, and it is the design that makes the synthetic categories, the inline `workflow-refs`/`schema-conformance` emissions, and the non-task `override-default` all tune through one mechanism.

- **A `Finding` carries `(probe, check)` as structured fields** (M6 — alongside the existing `code`/`message`/`route`; the dotted `code` is retained for rendering but is no longer the severity handle). The pre-M6 finding had only a dotted `code`, and the built codes did **not** all parse to a clean `<probe>.<check>` (e.g. `override-default.content-changed` vs the inventory's `target-unchanged`; `file-state` emitting `reconciliation.*`), so the handle is made explicit rather than parsed.
- **Override-only-on-explicit-delta.** The post-pass overrides a finding's severity **only when the resolved cascade actually carries a `scalar-set` for that finding's key** (per-check, then per-probe — the three-step lookup above). Absent any delta, the finding keeps the **hardcoded default** the probe emitted. This makes the **no-override path byte-identical** to pre-M6 output *automatically* — the determinism boundary's #1 risk for this milestone — and sidesteps any need for `knobs.yaml` defaults to exactly mirror the code's literals (the knob `default` exists for `check_value`, the floor, and `--explain`, not the hot path).
- **Matched by inventory `(probe, check)` membership, never by code-prefix.** A finding is tuned only if its `(probe, check)` is an inventory row. The per-probe fallback step applies *only* to findings whose check has a row — so an unkeyed sibling carrying a category-prefixed code (e.g. `schema-conformance.unknown-type`, or the parser's `conformance.*` codes — neither in the inventory) is **exempt from the post-pass entirely** and keeps its emitted (blocking) severity. A per-probe key never accidentally catches a determinism-boundary code that has no row.
- **Assigned once, at report construction — before the gate.** Severity assignment runs as the single construction point of the `ValidationReport` (`ValidationReport::new(findings, &resolved)` — or an explicit assignment the contract requires before any read), so `has_blocking()` and every downstream consumer see *post-pass* severities. There is no window where a caller reads a finding's pre-assignment severity. A test pins that `has_blocking()` reflects the assigned severities (a demoted blocking check stops gating; a promoted advisory one starts).
- **The gate itself is unchanged.** `finalize` still blocks on `severity == blocking` ([How it gates `finalize`](#how-it-gates-finalize)); once severity is cascade-assigned at construction, a demoted check simply stops being blocking with no change to the gate predicate.
- **Every finding-emitting entry point must resolve the cascade.** Compose already builds a `Resolved`; **`task validate` / `finalize` and `jigc upgrade` do not today** and must gain one to feed the post-pass. Building a `Resolved` at those entry points is itself behaviour that must not perturb the no-override path, so the **byte-identical golden must cover the validate and upgrade paths**, not only `start_compose` — the determinism guard's real surface this milestone.

**`warning` is a live third tier (M6).** Pre-M6 only `blocking`/`advisory` were ever produced. M6 makes `warning` a settable, produced severity: it is **surfaced but non-blocking** (the gate keys on `blocking` only), rendered distinctly from `advisory`. A check demoted `blocking → warning` still appears in every report and `--explain`, but does not stop `finalize`/`upgrade`.

## The non-task `Probe` seam (M6)

M6 stands up the `Probe` seam (`probe.rs` is a stub today) **minimally** — shaped to fit `override-default`, not as a universal trait forced over every existing probe. `validate_task`'s proven inline probes (`file-state`, `schema-conformance`, `ref-resolves`) stay wired as they are; they become severity-tunable purely via the post-pass above, with **no rewrite of the byte-stable task path**. The seam's job is narrower: give a **non-task-scoped** probe (one whose ctx is `(recorded deltas, pack)`, not a task working area) a first-class entry point so it participates in aggregation + the post-pass.

- **`override-default` is the retrofit.** M5 ran it as a bespoke `classify(deltas, pack)` call inside `jigc upgrade`; M6 routes it through the seam and threads the resolved severity lookup into it, so its three checks tune via `validation.override-default.*.severity` like any other. `upgrade_in_repo` is the only production caller, so the retrofit touches one CLI seam.
- **Two ctx shapes, not one trait over both.** Task-scoped and override-scoped probes consume structurally different inputs; the seam admits the non-task shape rather than coercing both into a single generic ctx (the over-generalization trap M3 paid for). A future store-scoped probe extends the seam then, when a real consumer exists.

## How it gates `finalize`

`finalize` ≡ `validate(task) + commit`: one engine, two entry points, so what `validate` reports and what `finalize` blocks on can never diverge. `finalize` blocks on **blocking-class findings within the task scope**; advisory findings are surfaced but don't stop the commit. `finalize` defaults to autonomous (a cascade-settable confirm-gate aside), with git/PR as the durable correction point.

## Open questions

- **Pack-probe implementation** — the OS-level sandboxing tech (seccomp, Landlock, equivalents) and the concrete enforcement mechanism for the locked determinism contract ([Pack-probe determinism contract](#pack-probe-determinism-contract)), plus the *concrete* request/response schema (versioning policy, finding envelope, snapshot format) over the now-sketched [wire seam](#the-wire-contract--request--response). Deferred with the pack-probe API; the contract and the wire seam are locked, and the implementation must satisfy them.
- **`doc-code` logic** — the development pack's symbol/test/timestamp checks.
- **Findings: recomputed vs cached** — leaning recompute-on-demand (derived state, like composed workflows), but not yet decided.
