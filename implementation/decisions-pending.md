# Decisions pending — the forward look

Two altitudes of "surface it before it's forced," each **consulted at a fixed trigger** so a deferral can't evaporate into [DECISIONS.md](../DECISIONS.md) prose and be forgotten:

- **Before planning a milestone** — consulted at [milestone-planning](milestone-planning-workflow.md) → **Scope**. The **discuss-later log**: milestone-keyed *discussion topics / design forks* that earlier work deferred "to settle before milestone N." When we park a topic with a trigger milestone, it lands here, so the next planning run *starts* by pulling its due topics into the done-picture and gap list — instead of relying on memory that "we said we'd discuss this before then." The backlog to triage these from is [VISION.md](../VISION.md) → Open questions (the categorized index); this log adds the missing piece — each due topic's **trigger milestone**.
- **Before building an increment** — consulted at [milestone-planning](milestone-planning-workflow.md) → **Detect gaps**. Increment-keyed decisions a dev-workflow run will **force**, tagged **(D)** a genuine open *design* question or **(I)** an *implementation* pick (crate/algorithm that falls out at pickup) — surfaced so nothing ambushes a build mid-task.

This is a *decision/discussion* backlog, not a *task* backlog — tasks are still cut per-increment at pickup ([roadmap.md](roadmap.md)). Entries **graduate to [DECISIONS.md](../DECISIONS.md) when settled; delete them here once logged.**

**How entries arrive — the write side (everyone, not just the planner).** Whoever *defers* a topic to a future milestone — in a [design](design-workflow.md) discussion, a [dev](dev-workflow.md) run, a planning [Settle](milestone-planning-workflow.md#the-loop-one-milestone), or a [completion](milestone-completion-workflow.md) triage — **records it here in the same motion as the deferral**, keyed to its trigger. The split is the point: [DECISIONS.md](../DECISIONS.md) logs what was *decided*; this file logs what is *owed and when it comes due*. A deferral that lives only in a DECISIONS sentence has no trigger and will be forgotten — which is the exact failure this log exists to prevent.

## Before planning — milestone-keyed deferrals

*(The spine's milestone-keyed deferrals are all graduated — M9 was the last spine milestone and its two `(D)` items are settled at M9 planning, 2026-06-06: legacy ingestion **bounded** to detect-and-route (the research-grade auto-migrate core deferred below), and profiles-beyond-Claude-Code **confirmed deferred** since M9 targets only Claude Code. Post-spine items below + the condition-keyed backlog are what remain.)*

### After the spine completes (post-M9 — these need a milestone *created* for them)

The roadmap spine ends at M9; these are real, locked-direction items with **no home milestone on the current spine** — listed so they get one, rather than drifting:

- **Legacy auto-migration** — the research-grade core of existing-project ingestion: *rewriting* a non-conformant foreign doc into conformant shape (vs M9's bounded **detect-and-route**, which classifies + routes but never rewrites — [project-setup.md](../design/project-setup.md) → Flow 2). Needs tolerant/fuzzy heading-mapping machinery the strict canonical parser deliberately withholds ([VISION.md](../VISION.md) → Open questions). *Trigger:* a real foreign-doc corpus + a measured migration-quality baseline justifying the machinery (and a willingness to relax the determinism boundary around the parse). Wants its own post-spine milestone.
- **Self-hosting** — distill the harness into a jigc pack + dogfood it ([ideas/self-hosting.md](../ideas/self-hosting.md)); the designed terminus, presupposing M7 (done) + M8 + workflow-as-artifact machinery.
- **`describe` / introspection surface** — parked post-MVP, direction locked ([ideas/describe.md](../ideas/describe.md)); candidate home `design/introspection.md`.
- **The `doc↔code` pack-probe family** — pack-probe sandboxing, `doc-code` logic, the subprocess probe contract ([validation.md](../design/validation.md) / [module-layout.md](module-layout.md) → Open questions); lands after the doc-creation flows exist (VISION principle #6). Wants its own pack-probe milestone.
- **Multi-pack composition** — a second domain pack + cascade composition ([VISION.md](../VISION.md)); blocked on a real second pack (the "not a second domain yet" non-goal).

### Document shapes — discuss when a workflow drives each

The doctype/relation/grammar shapes deliberately left un-schema'd until a workflow creates or reads them ([doctype-map.md](doctype-map.md) → Unsettled; [document-type-schema.md](../design/document-type-schema.md) / [structural-grammar.md](../design/structural-grammar.md) opens). Each earns its schema *with its driving workflow* — defining one earlier is generality for a single-use thing ([PRINCIPLES](../PRINCIPLES.md)). Most are gated on a workflow that doesn't exist on the M1–M9 spine, so they cluster as "discuss when that workflow is designed." *(Recovered 2026-06-04 by mining DECISIONS + the design opens — the "document shapes we meant to discuss." The discussions themselves are a focused [design-workflow](design-workflow.md) session each, one at a time; this log only ensures they resurface.)*

- **`prd` doctype** — *settled M9* (2026-06-06): driven by the new-project idea-development setup workflow, schema earned there with **fixed prose slots** ([project-setup.md](../design/project-setup.md) → Flow 1; [doctype-map.md](doctype-map.md)). The **repeatable-requirements** shape (per-requirement anchors + the `prd —decomposes-into→ spec` edge) stays deferred — it needs the unbuilt `add-item` authoring verb (below) and the multi-word-heading parser fix (below); fires when a flow must author structured, individually-addressable requirements.
- **`arch-doc` doctype** (living architecture documentation). *No firm trigger* — needs a yet-undesigned arch workflow (post-roadmap).
- **`spec —decided-by→ adr` edge** (n→n) — a spec pointing at the decisions that shaped it. Deliberately **cut from M3** as single-use generality ([DECISIONS.md](../DECISIONS.md):1250); a one-line schema delta when a spec-reading flow earns it. *No firm trigger.*
- **Inline references ("mentions") in slot prose** — the lighter `#issue-42`-style mention vs the heavyweight `field`-ref ([document-type-schema.md](../design/document-type-schema.md) opens; [parsing.md](parsing.md) → mentions-in-prose). *No firm trigger* — earns design when a doctype's slot prose must carry adjudicable mentions.
- **Multi-level repetition** — a second repeatable level (repeatable-inside-repeatable), today capped at one ([structural-grammar.md](../design/structural-grammar.md) opens). *Condition:* fires behind whichever doctype first needs nesting (a `prd` requirements→sub-requirements, a spec criterion with sub-checks).
- **Multi-slot sections + their rendering** — multiple prose slots per section and how they render under sub-labels ([DECISIONS.md](../DECISIONS.md):374; pairs with the multi-slot sub-label syntax open). *No firm trigger.*

### No firm trigger yet — condition-keyed (revisit when the condition arrives, not on a date)

Tracked so they're not lost, but honestly *not* milestone-scheduled — assigning each a milestone now would be false precision:

- **Per-developer `local` layer · team-layer distribution** ([overrides.md](../design/overrides.md)) — when multi-developer / team adoption is real.
- **`import` three-way-merge authoring** ([write-commands.md](../design/write-commands.md)) — detection ships; *authoring* deferred past M5/M6, fires when a real merge-conflict authoring need arises.
- **Reconciliation strictness · concurrent-OOB-during-a-task · external-edit notification** ([reconciliation.md](../design/reconciliation.md)) — post-MVP robustness.
- **Misc hardening opens** — `finalize --dry-run` + commit-msg hook capture ([finalize.md](../design/finalize.md)); command-ref scoping / stdin binding / multi-target ([command-catalog.md](../design/command-catalog.md)); parser span-precision / mentions-in-prose ([parsing.md](parsing.md)); engine→crate splits ([module-layout.md](module-layout.md)); multi-slot sub-label syntax.
- **`route` tagged-union promotion** — `route` stays `Option<String>`; the `fill-leaf`/`run-command`/`reconcile`/`none` tagged union is a separate cross-cutting decision, re-deferred at M5 + M6 ([DECISIONS.md](../DECISIONS.md):1742).
- **`add-item` / `remove-item` / `reorder` write verbs** — designed in [write-commands.md](../design/write-commands.md), unbuilt; fires when a flow must **author** repeatable items through the binary (M3's `criteria` authoring awaits it — `add-from-spec` only reads).
- **Optional-field / optional-slot capability** — `commit.scope`/`body` are author-required though conventional-commits treats them optional (engine work); a logged latent defect, fires when a doctype needs truly-optional fields/slots ([DECISIONS.md](../DECISIONS.md):1320).
- **Multi-word section-id round-trip defect** — a schema section id with a hyphen (`out-of-scope`, `success-metrics`) makes the **whole doc non-reparseable**: the writer title-cases the id → heading "Out Of Scope", but `parse::heading_matches` does a flat case-compare against the raw id `out-of-scope` and fails (`crates/engine/src/parse.rs` — the comment admits the MVP-schemas-are-single-word assumption). A **logged latent defect** found at M9 Scope; the fix is to re-slug the heading text before comparing (one-line parser change). **Not fixed in M9** — `prd`'s single-word slots route around it. *Trigger:* a doctype needs a multi-word section id (the deferred `prd` repeatable-requirements shape will).
- **Required read-role finalize gate** — "a required `reads:` role blocks finalize when unbound"; fires when a workflow ships a required read-role ([DECISIONS.md](../DECISIONS.md):1259).
- **Fillable `form` write sugar** — the all-at-once transactional batch over the 6 per-leaf verbs + its marker syntax; fires on multi-slot authoring flows ([write-commands.md](../design/write-commands.md) opens).
- **`override-default` content-target check** — checks step-*file* presence, not workflow include-list membership; revisits with a future **multi-workflow pack** ([DECISIONS.md](../DECISIONS.md):1710).
- **Store/milestone-scope inverse-/min-cardinality enforcement** — designed as "hard-enforced at store/milestone scope," never built (task scope is advisory-only); needs store-scope completeness validation (post-roadmap).
- **Minor / perf** — the unbound-spec-role "did you mean single-task?" advisory ([DECISIONS.md](../DECISIONS.md):1262); findings recomputed-vs-cached ([validation.md](../design/validation.md) opens).
- **Never-started fan-out backstop** — the runtime guard that surfaces a sub-task with no recorded CLI activity as a distinct join outcome (catching a launch template that *passes* install-time schema validation but still misfires at runtime). **Deferred at M8 Settle** (2026-06-04): M8 builds install-time template validation only; the runtime misfire of a schema-valid template is an accepted unsurfaced gap ([workflow-dialect.md](../design/workflow-dialect.md) → `fan-out`/`join`; [assistant-adapter.md](../design/assistant-adapter.md) → Bind the spawn mechanism). *Condition:* fires when a real fan-out launch-misfire is observed, or a workflow earns mid-fan-out resumption (pairs with the restart-from-scratch resumption policy). Likewise the **Resume / post-compaction hook** — capability-gated on the assistant exposing a resume event, not M8 ([assistant-adapter.md](../design/assistant-adapter.md) opens).

*(This is the **triage view** of [VISION.md](../VISION.md) → Open questions, not a second copy — VISION stays the full categorized index; this adds each still-open item's trigger. Several VISION opens are already **resolved** and not repeated here: slug-normalization + collision-suffix form (M7), `milestone-execution` orchestration + milestone-worktree (M7), config layout + committed-config-dir (M4), emitted-format micro-syntax (2026-05-28).)*

## Build-time — increment-keyed (historical: M1)

*The original worked example: the decisions M1's increments forced, surfaced before each build. Kept as the pattern; M2–M7 recorded their per-increment decisions directly to [DECISIONS.md](../DECISIONS.md) via the build harness.*

## Cross-cutting — settled 2026-05-31

Made up front because they shape many signatures; recorded in [DECISIONS.md](../DECISIONS.md). Listed here only as pointers:

- **(I) Error strategy** — `thiserror` (engine, typed) + `anyhow` (cli).
- **(I) Hashing** — `blake3`, one algo for `file-state` + content-drift.
- **(I) Test tooling** — `insta` (snapshot/golden) + `proptest` (property/fuzz).

## Cross-cutting — still open

- *(none — slug / minting normalization **settled 2026-05-31**: lowercase ASCII kebab-case + transliterate non-ASCII + numeric collision suffix in task-id merge order; see [DECISIONS.md](../DECISIONS.md). The write path is unblocked.)*

## Increment 3 — compose

- **(cleanup) Stale emitted-format open question** — [module-layout.md](module-layout.md) → Renderers still calls the emitted-format micro-syntax "an open question," but it's settled (the four-class format in [workflow-dialect.md](../design/workflow-dialect.md#emitted-format); VISION says settled 2026-05-28). Confirm and remove the stale ref.

## Increment 4 — write + finalize

- *(both **settled 2026-05-31** — git invocation: shell out to the `git` binary; blocked/error payload: reuse the `finding` shape (severity + located message + `route`). See [DECISIONS.md](../DECISIONS.md).)*

## Increment 5 — persisted ADR + edge index

- *(reconciliation OOB state machine + rename detection both **settled 2026-05-31** — `engine::file_state::reconcile_committed` (absorb / conformance-block / conflict-block over a committed doc) and `engine::file_state::detect_rename` (strong/weak signal for a missing tracked path, routed to git-revert, no ref rewrite); **wired to the command surface 2026-05-31** via `engine::file_state::reconcile_committed_store` inside `validate_task` (the `task validate` / `finalize`-preflight full sweep); see [DECISIONS.md](../DECISIONS.md).)*

## Increment 6 — adapter & ship

- **(D) Product name** — *settled 2026-05-31: ship the MVP as `jigc` (adopt the placeholder as the name). See [DECISIONS.md](../DECISIONS.md).*
- *(release / quickstart **settled 2026-05-31** — a clean `cargo build --release -p cli` produces `target/release/jigc`; `crates/cli/tests/release_smoke.rs` verifies the built binary's `--version` + `jigc setup`; `QUICKSTART.md` documents the loop, referenced from CLAUDE.md. Cross-compile is out of MVP scope. See [DECISIONS.md](../DECISIONS.md).)*
