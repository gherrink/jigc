# Codex review — Pass 3: Invariant & determinism-boundary stress test

**Date:** 2026-05-28  
**Reviewer:** OpenAI Codex CLI (cross-model second opinion)  
**Files reviewed:** all 14, with emphasis on CLAUDE.md §"Architectural invariants" and VISION.md §"The determinism boundary".  
**Lens:** Adversarial. For each of the 8 invariants + the determinism contract, try to break it using only what the docs say — walk a concrete scenario, find where the design forgets its own rule.  
**Format:** each attack has severity, which invariant it targets, the scenario walked, doc citations creating the tension, why any existing disclaimer is insufficient, and a one-line proposed fix.  
**Closing note from Codex (preserved):** *"No invariant fully survives without at least an under-specification. The narrowest survivor is invariant 1 if 'CLI core' is read literally."*

---

**[SEV: high] Slot prose can change document structure, so the LLM-owned side can perturb CLI-owned structure.**
- Invariant under attack: Determinism boundary / 6 slots vs placeholders / 8 transactional writes
- Scenario walked: The agent fills a slot with ordinary Markdown containing `## Decision`, `### Something {#id}`, or a final bullet list that looks like fields. CommonMark will expose those as block structure before the schema mapper decides slot spans, so either the CLI rejects normal prose or prose changes where sections/items/fields are parsed.
- Doc support: `implementation/parsing.md`: “the span between a heading and its trailing field group → the slot's content span” and “treats **slot prose as an opaque byte span** it never interprets”
- Why disclaimer insufficient: Calling slot prose “opaque” does not make CommonMark headings/lists inside it opaque to the block parser.
- Proposed fix: Add an explicit slot-boundary serialization rule, or formally ban/rewrite structural Markdown constructs inside slots at `set-slot`.

**[SEV: high] Body field groups are ambiguous with valid prose lists, causing silent prose-to-field reclassification.**
- Invariant under attack: 6 slots vs placeholders / 7 storage / determinism boundary
- Scenario walked: A slot legitimately ends with a bullet list like `- maps-to-test: TBD` or `- status: accepted`. The parser treats the final list whose keys match declared fields as the field group, so prose can be silently removed from the slot and promoted into CLI-owned structure.
- Doc support: `implementation/parsing.md`: “The field group is *the final List block whose item keys match this section's declared fields*” and “The one pathological case … either parses to a valid value or routes as a conflict”
- Why disclaimer insufficient: “Parses to a valid value” is exactly silent corruption if the author intended prose.
- Proposed fix: Require an explicit field-group marker/fence, or reject matching final prose lists unless the CLI wrote them.

**[SEV: high] Sub-agent spawn is trusted to a freeform adapter template, so a stale profile can bypass the blackboard.**
- Invariant under attack: 2 LLM writes only through CLI / 9 concurrency
- Scenario walked: The CLI emits fan-out payloads, but the assistant profile decides how to launch them. If the template omits `tool workflow`, passes the wrong task id, or asks the sub-agent to work from inline instructions, the sub-agent can write outside `.tool/tasks/<id>/` and the join’s assumptions collapse.
- Doc support: `assistant-adapter.md`: “The profile carries a **launch template**” and `workflow-dialect.md`: “the **assistant adapter launches** them”
- Why disclaimer insufficient: “CLI owns the payload” does not verify that the launched agent actually receives or follows that payload.
- Proposed fix: Require a CLI-recorded subtask handshake before join, and validate adapter spawn templates against a typed template schema.

**[SEV: high] Filename-as-doc-identity makes out-of-band renames mutate stable IDs.**
- Invariant under attack: 3 stable opaque IDs / 7 OOB reconciliation
- Scenario walked: A human renames `decisions/rate-limit.md` to `decisions/gateway-rate-limit.md`. Because identity is the path and the file contains no doc id, the parser can accept the renamed file as a different ADR while existing refs to `adr:rate-limit` break.
- Doc support: `design/storage.md`: “**Identity is the path**” and “A retitled H1 changes the title, never the frozen filename-id.”
- Why disclaimer insufficient: The rename guarantee covers H1 retitle, not filename/path rename, which humans will do in git.
- Proposed fix: Add explicit rename detection/reconciliation using prior file hash or require a CLI `doc rename` op for path changes.

**[SEV: high] Fan-out collision rewriting assumes cross-area refs cannot exist, but agents can author guessed sibling IDs.**
- Invariant under attack: 3 stable IDs / 8 finalize integrity / 9 concurrency
- Scenario walked: Subtask A creates `adr:cache`; subtask B independently writes `supersedes: adr:cache`, intending A’s ADR or guessing the same slug. At join, collision suffixing rewrites only local self-refs, so B’s cross-area ref can dangle or retarget semantically.
- Doc support: `workflow-dialect.md`: “rewrites that area's *local* self-references” and “Isolation guarantees the only references to a sub-agent's new doc are from its own area”
- Why disclaimer insufficient: Filesystem isolation does not prevent an LLM from typing a plausible future ID.
- Proposed fix: Forbid refs to uncommitted sibling docs unless allocated through a CLI reservation/global provisional-ID table.

**[SEV: high] Forward-ref finalize blocking is labeled task-controllable, but sibling-task targets are not.**
- Invariant under attack: 8 transactional finalize gate / 9 concurrency
- Scenario walked: Task T1 creates ADR A with `supersedes: adr:b`, where ADR B is planned in T2 but not finalized yet. T1 cannot fix that without duplicating B, yet forward-ref integrity blocks per-task finalize.
- Doc support: `validation.md`: “blocks only on **integrity the task can fix** — forward-ref resolution…” and `document-type-schema.md`: “Forward-ref integrity *is* gated at `finalize`, because the task can satisfy it by creating the target in the same task.”
- Why disclaimer insufficient: “Can create in the same task” is false for legitimate cross-task plans.
- Proposed fix: Distinguish committed refs, same-task refs, and planned/provisional refs; block only the first two when actually dangling.

**[SEV: med] Task creation adds a third runtime minting site after the grammar promised exactly two.**
- Invariant under attack: 3 stable opaque IDs
- Scenario walked: Structural grammar says runtime IDs are minted only for container instances and repeatable items. `tool start --workflow` also mints a task id from intent, and fan-out join depends on task-id ordering, so task IDs are structural even if not modeled as such.
- Doc support: `design/structural-grammar.md`: “Runtime minting happens at **exactly two sites**” vs `design/write-commands.md`: “a **slug from the intent** … a third minting site”
- Why disclaimer insufficient: No disclaimer exists.
- Proposed fix: Update the structural grammar to include tasks as first-class minted containers, or state task IDs are outside the document/workflow ID invariant.

**[SEV: med] Workflow order is encoded as physical Markdown include order, despite “never positions.”**
- Invariant under attack: 3 stable IDs
- Scenario walked: Workflow definitions use a body of ordered `{{include}}` lines. A hand edit that moves a line changes structure by position, while the invariant says ordering is a separate ordered list of IDs and structural ops target IDs.
- Doc support: `workflow-dialect.md`: “the **body is the ordered `{{include}}`s** — which *is* the composition and the ordering” vs `structural-grammar.md`: “Ordering lives in a separate ordered list of IDs, never in positions”
- Why disclaimer insufficient: “Body as ordered list” is a serialization choice, but the docs do not define its conformance rules tightly enough.
- Proposed fix: Define workflow bodies as a canonical ordered-ID list with include-only parsing, or revise the invariant to allow physical-order serialization.

**[SEV: med] The workflow example uses placeholder braces for an author directive, blurring slots and placeholders.**
- Invariant under attack: 6 slots vs placeholders
- Scenario walked: The unresolved workflow example contains `{{ author: commit.summary }}`, which looks exactly like a CLI-resolved placeholder. The composed example later emits `<<author: ...>>`, but the source dialect example has already reused placeholder syntax for an LLM-write directive.
- Doc support: `workflow-dialect.md`: “Run: {{ cli.set-commit-summary }} / {{ author: commit.summary }}” and “`placeholder` — read-path, `{{…}}`, CLI-filled before the agent sees it”
- Why disclaimer insufficient: “Emitted markers illustrative” does not excuse violating the settled `{{…}}` vs `<<…>>` distinction in the definition grammar.
- Proposed fix: Replace author directives in workflow definitions with an instruction leaf that renders only as `<<author: addr>>`.

**[SEV: med] The emitted workflow/template/explain syntax is still open, so slot/placeholder separation is not enforceable across views.**
- Invariant under attack: 6 slots vs placeholders
- Scenario walked: The design relies on the agent clearly distinguishing “run command,” “author slot,” and “reason,” but the renderer syntax is explicitly undecided. `--template`, `--explain`, and composed agent-text could accidentally expose unresolved `{{…}}` beside author markers.
- Doc support: `workflow-dialect.md`: “the concrete visual grammar … is an open emitted-format question” and `module-layout.md`: “agent-text renderer *for composed workflows* is gated on the **emitted-format micro-syntax**”
- Why disclaimer insufficient: An open syntax is not an invariant.
- Proposed fix: Specify the emitted grammar now and add golden tests that composed output contains no unresolved placeholders except escaped examples.

**[SEV: med] Pack probes can be nondeterministic or model-backed while validation remains part of the deterministic boundary.**
- Invariant under attack: 1 CLI core makes no LLM calls / determinism boundary
- Scenario walked: A future pack probe is an arbitrary subprocess behind a JSON contract. It can call a model, hit the network, read time, or mutate files unless sandboxed, yet its findings feed `validate` and `finalize`.
- Doc support: `validation.md`: “Pack probes … **read-only and deterministic by contract**” and “Pack-probe sandboxing … deferred”
- Why disclaimer insufficient: “By contract” is not enforcement, and validation results are claimed deterministic.
- Proposed fix: Define probe sandboxing, no-network/no-model rules, time/env constraints, and failure semantics before enabling subprocess probes.

**[SEV: med] Adapter hook uses an undefined orientation command, risking side effects or drift from `tool start` semantics.**
- Invariant under attack: 2 bootstrap/write channel / 8 transaction model
- Scenario walked: The adapter profile runs `tool start --orient`, but task origination defines bare `tool start` and `tool start --workflow`; `--orient` is not specified. A generated hook must be guaranteed read-only, or session injection could accidentally create state.
- Doc support: `assistant-adapter.md`: `run: "tool start --orient"` vs `write-commands.md`: “bare **`tool start`** — orients”
- Why disclaimer insufficient: No disclaimer exists.
- Proposed fix: Specify a read-only `tool orient` or `tool start --orient` command and forbid task minting in adapter hooks.

**[SEV: med] Missing `{#id}` on imported repeatable items is simultaneously auto-handled and MVP-blocking.**
- Invariant under attack: 7 storage/OOB reconciliation
- Scenario walked: A human adds a new criteria item without `{#id}`. One doc says this is auto-handled by mint-on-import; another says MVP detects and blocks; the OOB policy says conformant non-conflicting edits are accepted.
- Doc support: `implementation/parsing.md`: “Missing `{#id}` → mint-on-import” and “MVP caveat … MVP detects + blocks”
- Why disclaimer insufficient: The same edit cannot be both conformant-importable and nonconformant-blocking without a versioned rule.
- Proposed fix: Classify missing item anchors as either conformant-with-import or nonconformant in all docs; do not split by MVP unless the invariant is scoped.

**[SEV: med] Embedding the dev pack in the CLI binary blurs CLI vs domain pack for non-CLI frontends.**
- Invariant under attack: 4 engine vs CLI vs domain pack separation
- Scenario walked: The engine is empty, but the only concrete pack source is embedded in `cli`. A future MCP frontend over `engine` does not automatically have the same pack-default source unless it also embeds or depends on CLI-owned assets.
- Doc support: `module-layout.md`: “The development pack … is **embedded in the `cli` binary**” and “the **engine reads pack-default through a source abstraction**”
- Why disclaimer insufficient: “Data, not logic” preserves engine purity but not artifact-level separation.
- Proposed fix: Define pack assets as a separate package/source artifact that each frontend embeds or supplies explicitly.

**[SEV: low] External team config makes “same repo, same binary” nondeterministic unless the resolved cascade is surfaced every time.**
- Invariant under attack: 5 config cascades / determinism boundary
- Scenario walked: Two agents on two machines run the same repo with different `~/.config/<tool>/` team layers and get different workflows. The docs allow this, but the bootstrap may make the CLI feel repo-authoritative when a hidden external layer is changing structure.
- Doc support: `overrides.md`: “team … external `~/.config/<tool>/`” and “Composition is a pure function of the *resolved cascade*”
- Why disclaimer insufficient: The resolved cascade is a declared input, but not necessarily visible in normal agent output.
- Proposed fix: Include cascade provenance/version in `tool start`, `workflow`, `validate`, and `--explain` output.

**[SEV: low] Bootstrap routing may not survive context compaction because reinjection is session-start scoped.**
- Invariant under attack: 2 LLM writes only through CLI / bootstrap boundary
- Scenario walked: An agent starts correctly, then its context is compacted mid-task and the static/hook-injected routing pointer is dropped or diluted. The hook only runs at session start, so the adapter may not reassert “use tool” before the agent resumes.
- Doc support: `bootstrap.md`: “The agent's *entire* a-priori knowledge is ‘run the front door and follow it.’” and `assistant-adapter.md`: “hook … at session start”
- Why disclaimer insufficient: “Path of least resistance” does not address context lifecycle loss.
- Proposed fix: Add resume/compaction reinjection where supported, and include a routing footer in every composed workflow/result.

**[SEV: low] “Detected and reconciled, never forbidden” overpromises while true merge is deferred.**
- Invariant under attack: 7 OOB edits detected and reconciled
- Scenario walked: If both task working copy and committed file changed the same managed doc, the system blocks and asks a human; three-way merge is explicitly deferred. That is detection and routing, not reconciliation.
- Doc support: `write-commands.md`: “True conflict … **blocks and routes to the human**” and “Three-way merge … is deferred”
- Why disclaimer insufficient: The invariant says reconciled, but the mechanism only blocks pending a future merge path.
- Proposed fix: Reword the invariant to “detected and routed; conformant non-conflicts imported,” or specify the minimal human reconciliation operation now.

No invariant fully survives without at least an under-specification. The narrowest survivor is invariant 1 if “CLI core” is read literally: I found no doc where the Rust engine/CLI itself calls an LLM, but subprocess probes and adapter launch templates can reintroduce nondeterminism unless their boundary is tightened.
