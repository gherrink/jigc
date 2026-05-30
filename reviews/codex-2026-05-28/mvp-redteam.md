# Codex review — Pass 4: MVP red-team

**Date:** 2026-05-28  
**Reviewer:** OpenAI Codex CLI (cross-model second opinion)  
**Files reviewed:** VISION + CLAUDE (§MVP scope) + DECISIONS (2026-05-25 "MVP scope clarifications" + "MVP thickening"), plus design/ and implementation/ as needed to verify the MVP.  
**Lens:** Does the thickened MVP (commit + adr, single-task workflow, engine-native probes) actually prove its five claimed differentiators end-to-end? Are there things inside the MVP that aren't load-bearing, or things outside the MVP that it secretly depends on?  
**Five claimed differentiators:** (1) lossless round-trip on a committed, human-editable file; (2) `file ↔ CLI-state` drift detection + OOB reconciliation; (3) context-slice assembly over a real persisted doc; (4) referential-integrity advantage via edge index + `supersedes`; (5) deterministic compose → execute → validate → finalize loop.  
**Format:** per-differentiator verdict (PROVEN | PARTIALLY PROVEN | NOT PROVEN) with scenario walked, gap, and minimum scope adjustment. Followed by a scope audit.

---

## Differentiator 1: Lossless round-trip on a committed, human-editable file

**Verdict: PARTIALLY PROVEN**

**Scenario walked (end-to-end, MVP-only commands):**
1. `tool start --workflow single-task "Add gateway rate limit"` creates task `add-gateway-rate-limit`, pins base `HEAD`, provisions `commit:add-gateway-rate-limit`.
2. Agent creates `adr:rate-limit-at-the-gateway`:
   `tool doc create adr --title "Rate-limit at the gateway"`.
3. Agent fills fields/slots:
   `adr:rate-limit-at-the-gateway#status`, `#context`, `#decision`, `#consequences`.
4. `tool task validate add-gateway-rate-limit` reparses working ADR, checks required slots, malformed fields, workflow refs, file-state.
5. `tool task finalize add-gateway-rate-limit` promotes `decisions/rate-limit-at-the-gateway.md` and commits it with code + commit sink.

**Gap:** This proves canonical generation + parse-after-write on a persisted ADR, but not necessarily *lossless round-trip of an already committed, externally edited human file* unless the MVP acceptance scenario includes re-reading/touching that committed ADR and asserting byte stability. The parser plan says the contract is “idempotent on canonical content” and “surgical on edits,” tested by golden/property tests, but that is an implementation test promise, not automatically exercised by the single-task flow. `implementation/parsing.md:Round-trip guarantees:82-100`. The thickening claim is explicit: ADR was added to retire “lossless round-trip” because commit-only dodged it. `DECISIONS.md:MVP thickening:173-175`.

**Minimum scope adjustment to convert to PROVEN:** Add an MVP acceptance test: commit ADR → no-op read/write or single-slot edit on committed ADR → assert byte-identical outside the target span.

## Differentiator 2: `file ↔ CLI-state` drift detection + OOB reconciliation

**Verdict: PROVEN**

**Scenario walked (end-to-end, MVP-only commands):**
1. Finalize an ADR as above, producing `decisions/rate-limit-at-the-gateway.md` and baseline file-state hash.
2. Human edits the committed ADR directly.
3. Run `tool task validate <new-task>` or `tool task finalize <new-task>`.
4. `file-state` compares raw file bytes to recorded state.
5. If edit is conformant/non-conflicting, CLI reparses and rebuilds caches; if nonconformant, conformance blocks; if same doc was also changed in task, finalize blocks with `reconcile`.

**Gap:** No material gap for MVP scope. Three-way merge is deferred, but the docs do not claim to solve it in MVP: true conflicts block and route to human. `design/write-commands.md:Out-of-band reconciliation:82-91`. The probe is MVP-native: `file-state` detects hash drift and emits a reconcile finding. `design/validation.md:Probes:66-68`.

**Minimum scope adjustment to convert to PROVEN:** None, but include all three fixtures: conformant import, nonconformant block, same-doc conflict route.

## Differentiator 3: Context-slice assembly over a real persisted doc

**Verdict: NOT PROVEN**

**Scenario walked (end-to-end, MVP-only commands):**
1. Finalize `adr:rate-limit-at-the-gateway`.
2. Attempt to compose a workflow that embeds `{{adr:rate-limit-at-the-gateway#decision}}`.
3. Expected: composer resolves a persisted ADR slice into agent-text.

**Gap:** The MVP scope says this is proven via `{{adr:x#decision}}`, but the scoped `single-task` workflow is spec-less and only guaranteed to embed `{{task.intent}}`. `CLAUDE.md:MVP scope:65`; `DECISIONS.md:MVP thickening:176`. ADR is described as an optional output “when warranted,” not an input to the workflow. `DECISIONS.md:MVP thickening:179`. The workflow dialect supports literal doc-slice data-values, but no MVP workflow step is required to exercise a persisted ADR slice. `design/workflow-dialect.md:Leaves:46-68`.

**Minimum scope adjustment to convert to PROVEN:** Add one MVP workflow step or smoke workflow that composes `{{adr:<known-id>#decision}}` from a finalized ADR.

## Differentiator 4: Referential-integrity advantage via edge index + `supersedes`

**Verdict: PARTIALLY PROVEN**

**Scenario walked (end-to-end, MVP-only commands):**
1. Finalize `adr:single-node-cache`.
2. Start a new task.
3. Create `adr:replace-single-node-cache`.
4. Set relation field:
   `tool doc set-field adr:replace-single-node-cache#supersedes --value adr:single-node-cache`.
5. Fill required ADR slots.
6. `tool task validate` overlays working deltas on committed graph and walks the edge index.
7. `tool task finalize` passes if target exists; blocks if `supersedes` points to a missing ADR.

**Gap:** This proves forward-ref resolution over the edge index only if the scenario deliberately sets `supersedes`. The relation is optional `0..1`, so the normal ADR path can skip it and dodge the integrity check. `design/document-type-schema.md:ADR schema:110-122`. It is also a self-relation, so it exercises target existence and inverse derivation minimally, not cross-type graph behavior. Forward refs are gated at finalize, while inverse/minimum-cardinality remains advisory. `design/document-type-schema.md:Cross-references:86-89`; `design/validation.md:Integrity vs completeness:60`.

**Minimum scope adjustment to convert to PROVEN:** Make the MVP acceptance scenario require one passing `supersedes` and one dangling `supersedes` finalize-block test.

## Differentiator 5: Deterministic compose → execute → validate → finalize loop

**Verdict: PARTIALLY PROVEN**

**Scenario walked (end-to-end, MVP-only commands):**
1. `tool start --workflow single-task "Add gateway rate limit"` mints task and composes fixed steps.
2. Composer resolves includes, command-refs, and `{{task.intent}}`.
3. Agent follows emitted commands: writes code directly, fills `commit:add-gateway-rate-limit#type` and `#summary`, optionally creates/fills ADR.
4. `tool task validate add-gateway-rate-limit` runs workflow-refs, file-state, required-slot/malformed-value/ref integrity.
5. `tool task finalize add-gateway-rate-limit` validates again, renders commit sink, promotes ADR, runs git commit.

**Gap:** The core loop is scoped, but the agent-facing composed workflow renderer still depends on an open “emitted-format micro-syntax.” `implementation/module-layout.md:Renderers:35-38`; `design/workflow-dialect.md:Leaves:43-45`. There is also a small front-door ambiguity: CLAUDE says MVP `tool start` mints/resolves default `single-task`, while write-commands describes no-workflow `tool start "<intent>"` as router orientation with no task minted, then says MVP defaults to `single-task`. `CLAUDE.md:MVP scope:66`; `design/write-commands.md:Task origination:48-52`.

**Minimum scope adjustment to convert to PROVEN:** Freeze the MVP emitted workflow format and clarify `tool start "<intent>"` semantics when default workflow is `single-task`.

## Scope audit

- **Inside the MVP but not load-bearing:** Claude Code adapter profile static line + allowlist. Useful adoption path, but the five differentiators can be proven by direct CLI invocation. `implementation/module-layout.md:Adapter:42-44`.
- **Inside the MVP but not load-bearing:** Full team cascade layer. Pack-default/project config are needed for default workflow and severities; team layer is not needed to prove the MVP loop.
- **Inside the MVP but not load-bearing:** ADR `status` beyond a basic enum check. One enum field is useful for field validation, but the specific status lifecycle is not load-bearing.
- **Outside the MVP but secretly depended on:** Emitted-format micro-syntax. The agent must reliably distinguish “run command” vs “author slot”; currently open.
- **Outside the MVP but secretly depended on:** A required ADR-slice composition fixture. The claim names `{{adr:x#decision}}`, but MVP workflow only guarantees `task.intent`.
- **Outside the MVP but secretly depended on:** Precise task scoping for git staging. MVP “stages the dirty working tree,” so proof assumes a clean repo or risks committing unrelated changes. `DECISIONS.md:MVP scope clarifications:169`; `DECISIONS.md:MVP thickening:178`.

- **The `commit`-as-sink bet:** PARTIAL. It exercises schema mapping, required slots, field validation, and canonical writer-to-string, but it is explicitly degenerate for storage, round-trip, file-state, and persisted read paths. The docs already admit commit-only left the #1 risk unretired. `DECISIONS.md:MVP thickening:173`.

- **The `supersedes` choice:** PARTIAL. A 0..1 ADR self-relation is the smallest viable edge-index proof, but optionality means it is easy to skip. It proves forward target resolution, not rich graph behavior. Strong enough if the MVP acceptance path mandates it; weak if left to “when warranted.”

- **The “no router” stance:** Mostly coherent for one work-workflow. With only `single-task`, defaulting directly is fine. But selection guidance is load-bearing once there are two workflows, and the current docs should remove ambiguity around whether `tool start "<intent>"` mints immediately in MVP.
