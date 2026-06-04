# Decisions pending — the forward look

Two altitudes of "surface it before it's forced," each **consulted at a fixed trigger** so a deferral can't evaporate into [DECISIONS.md](../DECISIONS.md) prose and be forgotten:

- **Before planning a milestone** — consulted at [milestone-planning](milestone-planning-workflow.md) → **Scope**. The **discuss-later log**: milestone-keyed *discussion topics / design forks* that earlier work deferred "to settle before milestone N." When we park a topic with a trigger milestone, it lands here, so the next planning run *starts* by pulling its due topics into the done-picture and gap list — instead of relying on memory that "we said we'd discuss this before then." The backlog to triage these from is [VISION.md](../VISION.md) → Open questions (the categorized index); this log adds the missing piece — each due topic's **trigger milestone**.
- **Before building an increment** — consulted at [milestone-planning](milestone-planning-workflow.md) → **Detect gaps**. Increment-keyed decisions a dev-workflow run will **force**, tagged **(D)** a genuine open *design* question or **(I)** an *implementation* pick (crate/algorithm that falls out at pickup) — surfaced so nothing ambushes a build mid-task.

This is a *decision/discussion* backlog, not a *task* backlog — tasks are still cut per-increment at pickup ([roadmap.md](roadmap.md)). Entries **graduate to [DECISIONS.md](../DECISIONS.md) when settled; delete them here once logged.**

**How entries arrive — the write side (everyone, not just the planner).** Whoever *defers* a topic to a future milestone — in a [design](design-workflow.md) discussion, a [dev](dev-workflow.md) run, a planning [Settle](milestone-planning-workflow.md#the-loop-one-milestone), or a [completion](milestone-completion-workflow.md) triage — **records it here in the same motion as the deferral**, keyed to its trigger. The split is the point: [DECISIONS.md](../DECISIONS.md) logs what was *decided*; this file logs what is *owed and when it comes due*. A deferral that lives only in a DECISIONS sentence has no trigger and will be forgotten — which is the exact failure this log exists to prevent.

## Before planning — milestone-keyed deferrals

### Before planning M8 (milestone execution — control plane)

*Seeded 2026-06-04 from the M7 run — the topics M7 consciously deferred to M8, surfaced here so M8's Scope pulls them up front rather than rediscovering them.*

- **(D) Commit shape under real sub-agents.** M7 ships **one CLI-synthesized** milestone-finalize commit (no authored prose — [DECISIONS.md](../DECISIONS.md) 2026-06-04, the inc-4 fork). With M8's real sub-agents authoring, does it stay one synthesized commit, or move to the eventual **per-sub-task** model (each sub-task an authored commit doc) gated by the `finalize.fan-out.squash` knob ([finalize.md](../design/finalize.md) → `fan-out` finalize, the "eventual design")? A real fork to settle before cutting M8.
- **(D) Fan-out progress / resumption.** The standing [workflow-dialect](../design/workflow-dialect.md#open-questions) open question, now **due**: when a partial fan-out dies (k of N sub-agents finished), what does re-entry do — re-spawn only the un-acked, re-spawn all (idempotent), or restart? M7 deferred it ("restarts from scratch"); M8 owns the real spawn, so the policy must be settled.
- **(D) The spawn re-entry contract.** Reconcile the design's `jigc workflow W --task <id>` (named across [assistant-adapter.md](../design/assistant-adapter.md) / [workflow-dialect.md](../design/workflow-dialect.md)) with the built `jigc start --task` resume (which `conflicts_with --workflow`): add the verb, or render `start --task` and drop the redundant `{{workflow}}`? (The L1 landmine from M7 planning, deferred to M8 where the spawn binding lands.)
- **(integration seam) The real-write path must satisfy the M7 join's preconditions.** M7's join was proven over **fixture-staged** sub-areas; M8's real `--task`-scoped writes are where the deferred **write-time isolation barrier** and the now-reachable suffix/cross-group edges (the 2 LOW audit fixes guard the *data plane*; the *write path* must feed them correctly) are actually exercised. Spike this seam at M8 Settle — the inc-4 lesson generalized (don't assume the new producer satisfies the existing consumer's preconditions).

### Before planning M9 (project setup — new & existing)

- **(D) Legacy ingestion / migration.** The `project setup (existing project)` flow must ingest docs in inconsistent states — **research-grade**, flagged hard-not-solved ([VISION.md](../VISION.md) → Open questions). M9's headline risk and likely #1 gap; settle the approach (or consciously bound its scope) before cutting M9.
- **(D) Profiles beyond Claude Code** — only if M9 targets a second assistant; else stays deferred ([assistant-adapter.md](../design/assistant-adapter.md) → Open questions).

### After the spine completes (post-M9 — these need a milestone *created* for them)

The roadmap spine ends at M9; these are real, locked-direction items with **no home milestone on the current spine** — listed so they get one, rather than drifting:

- **Self-hosting** — distill the harness into a jigc pack + dogfood it ([ideas/self-hosting.md](../ideas/self-hosting.md)); the designed terminus, presupposing M7 (done) + M8 + workflow-as-artifact machinery.
- **`describe` / introspection surface** — parked post-MVP, direction locked ([ideas/describe.md](../ideas/describe.md)); candidate home `design/introspection.md`.
- **The `doc↔code` pack-probe family** — pack-probe sandboxing, `doc-code` logic, the subprocess probe contract ([validation.md](../design/validation.md) / [module-layout.md](module-layout.md) → Open questions); lands after the doc-creation flows exist (VISION principle #6). Wants its own pack-probe milestone.
- **Multi-pack composition** — a second domain pack + cascade composition ([VISION.md](../VISION.md)); blocked on a real second pack (the "not a second domain yet" non-goal).

### No firm trigger yet — condition-keyed (revisit when the condition arrives, not on a date)

Tracked so they're not lost, but honestly *not* milestone-scheduled — assigning each a milestone now would be false precision:

- **Per-developer `local` layer · team-layer distribution** ([overrides.md](../design/overrides.md)) — when multi-developer / team adoption is real.
- **`import` three-way-merge authoring** ([write-commands.md](../design/write-commands.md)) — detection ships; *authoring* deferred past M5/M6, fires when a real merge-conflict authoring need arises.
- **Reconciliation strictness · concurrent-OOB-during-a-task · external-edit notification** ([reconciliation.md](../design/reconciliation.md)) — post-MVP robustness.
- **Misc hardening opens** — `finalize --dry-run` + commit-msg hook capture ([finalize.md](../design/finalize.md)); command-ref scoping / stdin binding / multi-target ([command-catalog.md](../design/command-catalog.md)); parser span-precision / mentions-in-prose ([parsing.md](parsing.md)); engine→crate splits ([module-layout.md](module-layout.md)); multi-slot sub-label syntax.

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
