# Codex review — Pass 2: Completeness & coverage gaps

**Date:** 2026-05-28  
**Reviewer:** OpenAI Codex CLI (cross-model second opinion)  
**Files reviewed:** all 14 (VISION + CLAUDE + DECISIONS + 9 design/ + 3 implementation/)  
**Lens:** What's load-bearing but undefended? Which decisions are asserted but unowned by any doc? Which subsystem interactions have no clear home?  
**Format:** each gap has severity, where-it's-claimed citation, where-it-should-live, why-it-matters, and a one-line proposed fix.

---

**[SEV: high] `finalize` has no owned transaction spec for git staging, managed-doc promotion, commit rendering, failure rollback, or dirty-tree policy.**
- Where it's claimed/implied: `CLAUDE.md:69` — "`finalize` is the commit boundary. It validates, renders the commit doc into the git commit message ... promotes any created ADR ... and `git`-commits the task's code changes + promoted docs as one commit."
- Where it should be specified but isn't: new part-doc needed, e.g. `design/finalize.md`.
- Why it matters: the core write transaction can produce partial state, wrong staging, or unrecoverable task scratch if any git step fails.
- Proposed fix: add `design/finalize.md` specifying the ordered transaction: preflight, validate effective state, render commit message, promote docs, stage files, commit, update hashes/index, cleanup, and rollback/error behavior.

**[SEV: high] The severity cascade and finalize blocking policy are asserted, but the exact severity surface and non-overridable gate rules are not specified.**
- Where it's claimed/implied: `design/write-commands.md:68` — "Which severity a check carries is a cascade setting"; `design/validation.md:74` — "`finalize` blocks on blocking-class findings within the task scope."
- Where it should be specified but isn't: `design/validation.md` plus `design/overrides.md`.
- Why it matters: projects could accidentally demote hard integrity checks, or implementations could disagree on which findings are policy-tunable versus intrinsically blocking.
- Proposed fix: add a validation severity table listing every MVP check, default severity, cascade key, allowed overrides, and whether `finalize` may ever ignore it.

**[SEV: high] Relations are described as field-backed edges, but the schema format does not actually bind relation declarations to concrete field leaves end-to-end.**
- Where it's claimed/implied: `design/document-type-schema.md:61-62` — "the field is the instance endpoint ... the relation is the type-level constraint"; `design/document-type-schema.md:121-122` — `relations: - { name: supersedes, to: adr, card: "0..1" }`
- Where it should be specified but isn't: `design/document-type-schema.md`.
- Why it matters: validation, parsing, addressability, and writer generation cannot know whether `supersedes` is a field, a relation-only declaration, a front-matter key, or both.
- Proposed fix: revise the schema format so relation fields are declared as fields with relation metadata, including source section, field id, target, cardinality, inverse name, and storage location.

**[SEV: high] Override application during workflow composition has no single algorithm for layer precedence, structural deltas, same-id step replacement, include expansion, and cycle detection.**
- Where it's claimed/implied: `design/workflow-dialect.md:103` — "steps resolve through the cascade"; `design/overrides.md:19` — "Resolution applies the base, then `team` deltas, then `project` deltas"; `design/workflow-dialect.md:52` — includes are "cycle-checked at validate-time."
- Where it should be specified but isn't: `design/overrides.md` or `design/workflow-dialect.md`.
- Why it matters: the same inputs may compose differently depending on whether overrides are applied before include expansion, after include expansion, or per referenced step.
- Proposed fix: add a "workflow resolution algorithm" section defining base load, delta application order, by-id shadowing, include expansion order, and when cycle checks run on the resolved graph.

**[SEV: high] Out-of-band import is claimed as automatic for conformant edits, but the import/conformance/writeback boundary is inconsistent and under-specified.**
- Where it's claimed/implied: `VISION.md:49` — "a conformant external edit imports automatically"; `design/write-commands.md:87` — "`Import` is not a deferred feature"; `implementation/parsing.md:40` — "MVP detects + blocks; mint-on-import lands with the full import flow."
- Where it should be specified but isn't: `design/write-commands.md` plus `implementation/parsing.md`.
- Why it matters: a human edit that is parseable but missing machine wiring, such as `{#id}`, has no clear MVP behavior or cache/hash update path.
- Proposed fix: add an OOB reconciliation state machine covering parse success/failure, auto-repairable conformance gaps, conflict detection granularity, hash re-baselining, and MVP versus post-MVP behavior.

**[SEV: high] `tool start` does not specify behavior when a task or working area already exists for the same intent/task id.**
- Where it's claimed/implied: `design/write-commands.md:49` — "`tool start --workflow <X> '<intent>'` ... opens its working area at `.tool/tasks/<id>/`"; `design/write-commands.md:50` — "`--task <id>` resumes."
- Where it should be specified but isn't: `design/write-commands.md`.
- Why it matters: task slug collisions, abandoned scratch dirs, retries, and accidental duplicate starts can corrupt or strand work before the first write.
- Proposed fix: add task lifecycle rules for existing ids: resume, reject, suffix, show status, discard, and base-mismatch handling.

**[SEV: high] The agent-facing composed workflow format is MVP-critical but explicitly undecided.**
- Where it's claimed/implied: `VISION.md:63` — "The emitted workflow is structured markdown that makes unmistakably clear which lines are 'run this exact command,' which are 'author this slot,' and which are 'reason about X.'"; `design/workflow-dialect.md:170` — "Emitted-format micro-syntax ... open question."
- Where it should be specified but isn't: `design/workflow-dialect.md`.
- Why it matters: the agent-text renderer and the central compliance loop depend on this exact surface.
- Proposed fix: decide the emitted micro-syntax before implementation and add examples for command, author-slot, reasoning, validation finding, and fan-out dispatch blocks.

**[SEV: high] Command references are load-bearing for JIT command learning, but there is no command catalog/schema for how refs resolve into exact invocations.**
- Where it's claimed/implied: `design/bootstrap.md:23` — "Every other command the agent ever needs is learned just-in-time"; `design/workflow-dialect.md:50` — "`{{cli.add_phase}}` resolves to a literal CLI invocation, via the cascade."
- Where it should be specified but isn't: `design/workflow-dialect.md` or a new `design/command-catalog.md`.
- Why it matters: workflows cannot safely emit exact commands with task ids, addresses, stdin conventions, and cascade overrides unless the command-ref namespace is typed.
- Proposed fix: add a command-ref catalog schema defining ids, parameters, interpolation sources, validation, and rendered shell-safe output.

**[SEV: high] Workflow-gated `create` is asserted, but the gate’s data model, workflow location, and enforcement behavior are missing.**
- Where it's claimed/implied: `CLAUDE.md:67` — "The agent creates an ADR in-task via the workflow-gated `create` flow"; `design/write-commands.md:78` — "The gate is a cascade setting."
- Where it should be specified but isn't: `design/write-commands.md` plus `design/workflow-dialect.md`.
- Why it matters: the MVP’s ADR path depends on the CLI knowing when `tool doc create adr` is allowed and what error/route to emit when it is not.
- Proposed fix: add a create-gate schema with allowed doctypes per workflow/step, default pack values, cascade override behavior, and CLI enforcement errors.

**[SEV: med] The edge index lifecycle across working deltas, fan-out join, OOB import, and finalize promotion has no clear owner.**
- Where it's claimed/implied: `design/document-type-schema.md:88` — "The CLI keeps a rebuildable edge index"; `design/workflow-dialect.md:124` — "the CLI merges working areas by task-id order"; `design/validation.md:56` — "overlays the task's deltas on the committed edge index."
- Where it should be specified but isn't: `design/storage.md` or new `design/index.md`.
- Why it matters: validation and read views may use stale graph data after merge/import/promote unless rebuild and overlay rules are deterministic.
- Proposed fix: add an index lifecycle section covering committed rebuild, working overlay derivation, join-time reindex, finalize-time cache update, and invalidation stamps.

**[SEV: med] Commit-doc rendering is named as a canonical-writer sink, but the commit schema-to-git-message mapping is not specified.**
- Where it's claimed/implied: `design/write-commands.md:80` — "The commit message is just a doc type"; `implementation/parsing.md:56` — "a string sink (rendering the `commit` doc into the git commit message at finalize)."
- Where it should be specified but isn't: `design/document-type-schema.md` or `design/finalize.md`.
- Why it matters: commit output can drift from project conventions despite being advertised as schema/cascade controlled.
- Proposed fix: add a commit sink section defining subject/body/trailers rendering, empty optional slots, line limits, scope formatting, and cascade customization points.

**[SEV: med] `--explain` is promised, but the resolution tree contents and format are not specified.**
- Where it's claimed/implied: `VISION.md:63` — "A `--explain` mode shows the resolution tree"; `design/workflow-dialect.md:24` — "`--explain` re-computes the resolution tree on demand."
- Where it should be specified but isn't: `design/workflow-dialect.md`.
- Why it matters: explainability is the only way to debug cascade, include, placeholder, and live-state resolution without reading internal state.
- Proposed fix: add an explain output contract showing cascade provenance, include expansion tree, command-ref resolution, data-value path resolution, and validation failures.

**[SEV: med] The bootstrap advertises and demonstrates the tool, but the front-door orientation/nudge payload is still undefined.**
- Where it's claimed/implied: `design/bootstrap.md:29` — "The first `tool` call then demonstrates that breadth"; `design/assistant-adapter.md:65` — "Hook event(s) + nudge content ... not yet decided."
- Where it should be specified but isn't: `design/bootstrap.md` plus `design/assistant-adapter.md`.
- Why it matters: agent compliance depends on this ergonomic surface, not on sandboxing.
- Proposed fix: add concrete `tool start --orient` output examples for unset project, clean project, active task, and blocked task.

**[SEV: med] Worked examples do not cover several MVP-critical flows now claimed by the docs.**
- Where it's claimed/implied: `VISION.md:124-145` — only `single-task-execution`; `CLAUDE.md:67-71` — MVP includes ADR creation, OOB reconciliation, edge index, and finalize gate.
- Where it should be specified but isn't: `VISION.md` or a new `design/worked-examples.md`.
- Why it matters: the current example still centers a spec-shaped flow while the MVP is spec-less and ADR-producing.
- Proposed fix: add worked examples for spec-less single-task with optional ADR create, OOB edit reconciliation, override application at compose time, and finalize-to-git.

**[SEV: med] VISION’s open-question list is no longer a complete index of actual unresolved design work.**
- Where it's claimed/implied: `VISION.md:194` — "what remains genuinely open is short"; `VISION.md:205-208` lists only multi-pack composition, legacy ingestion, and product name.
- Where it should be specified but isn't: `VISION.md`.
- Why it matters: readers will miss MVP-blocking open items already acknowledged in part-docs, such as emitted workflow syntax, progress/resumption, config layout, task error payloads, and index/findings caching.
- Proposed fix: replace VISION’s short list with a categorized open-question index linking to each part-doc’s open section.

**[SEV: med] The stated reading order uses concepts before their owning docs define them.**
- Where it's claimed/implied: `CLAUDE.md:25` — reading order puts `storage` and `workflow-dialect` before `overrides`; `design/workflow-dialect.md:5` — "Builds on ... `overrides.md`"; `design/storage.md:82` — "The three cascade layers..."
- Where it should be specified but isn't: `CLAUDE.md` reading order, or earlier glossary in `structural-grammar.md`.
- Why it matters: a reader following the prescribed order encounters cascade/config-family/override mechanics before their owner doc.
- Proposed fix: move `overrides.md` earlier in the reading order or add a short glossary covering cascade, config family, delta, and pack-default before first use.

**[SEV: low] The non-sandboxed compliance boundary is not repeated where later docs state stronger invariants.**
- Where it's claimed/implied: `VISION.md:50` — "Agent compliance is adapter-enforced, not sandboxed"; `CLAUDE.md:52` — "The LLM writes only through the CLI"; `design/assistant-adapter.md:37` — "Generating this makes it a guaranteed setup step."
- Where it should be specified but isn't: `CLAUDE.md` and `design/assistant-adapter.md`.
- Why it matters: later docs can be read as promising enforcement against a bypassing agent, which the architecture explicitly does not provide.
- Proposed fix: add the same honest-boundary note to the architectural invariant and adapter responsibility sections.
