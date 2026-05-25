# Decisions

Running log of what we decided and **why**, dated. Short and punchy — this rots if it gets heavy. The *current* architectural truth lives in `VISION.md` and `CLAUDE.md`; this file is the history and the reasoning, not a re-explanation.

## 2026-05-23

- **Detail → part-docs, not folded into VISION** — one file, one purpose; cross-reference, don't duplicate.
- **Part-docs under `design/`, no number prefixes** — descriptive names; principle #2, avoid renumbering rot.
- **Review before commit** — Maurice eyeballs the written file first.
- **Decisions log is process & why, not architecture** — VISION + CLAUDE hold current truth; this holds when & why.
- **Constructive-critical is the standing collaboration stance** — discuss-then-write loop; default output is chat, not files.

### Document-type definition schema

- **Doc unit model — shallow ID'd tree** (`section → block → leaf`) — flat denies items the IDs validation needs; recursion is a CMS smell against small-footprint docs.
- **Trichotomy `section/slot/field`, split by adjudicability** — a field is a value the CLI can adjudicate, a slot only a human/LLM can judge; the determinism boundary drawn through one doc, and cross-refs prove fields must be a distinct kind.
- **One bounded `repeatable` construct** — a section body may be a list of ID'd blocks (per-item structure from v1, so each SPEC criterion is individually validatable); no recursion, no repeatable-in-repeatable.
- **Addressing = URI grammar `type:name#section/item/leaf`** — separates *which doc* (resource) from *where inside* (fragment); a whole-doc ref stays a clean, type-checkable atom.
- **IDs author-named vs minted** — types/sections/leaves are named in the schema; instances and repeatable items are minted, so runtime minting happens at exactly two sites (doc creation, item add).
- **Minted IDs = frozen content-slugs** — slugged from a required id-source field, frozen at creation, deterministic suffix on collision; ordinal-looking IDs rejected as a position-smell; chosen for diff-legibility.
- **Cross-ref = field + relation (two facets)** — field is the placed endpoint (home + address), relation is the type-level constraint (target type, cardinality, inverse); the ORM pattern, giving placement *and* graph integrity.
- **Cross-refs bidirectional, inverse derived not stored** — forward ref authored once, reverse edge computed into read-views; single source of truth, diff-clean, concurrency-safe; implies a rebuildable edge index, inverse-cardinality enforced at `finalize`.
- **Field types split engine-native vs pack-provided** — enum/string/date/bool/int/ref are engine-native; domain types like `code-anchor` are pack-provided with a pack-supplied adjudicator, keeping the engine empty.
- **One structural grammar, two dialects** — shared skeleton (units, ordering, blocks, repeatable, addressing, minting, include, override, validation engine); doc and workflow differ only in leaf kinds + annotations and never share a leaf kind (slot/placeholder stay opposites); two built-in dialects, not a public plugin framework.

### Write-command vocabulary

- **Write-layer ≠ override-layer** — write fills *instances* (slot/field values, repeatable items); override customizes *types* (sections/structure); they share addressing + unit kinds, not verbs.
- **Primitives are the core; the form is deferred sugar** — 6 per-leaf write verbs, MVP-sufficient and placement-safe; the fillable form compiles to a transactional batch, deferred until multi-slot flows — placement-safety comes from address-keying, not verb granularity.
- **Verb set + surface** — writes `create / set-slot / set-field / add-item / remove-item / reorder` as `tool doc <verb> <addr>`; lifecycle `diff / validate / finalize / discard` as `tool task <verb> <id>`.
- **The task is the staging unit** — the per-task isolated working area (already locked for concurrency) *is* the staging area; no separate proposal object, so "propose/review/apply" = "write/validate/finalize."
- **Write-time vs finalize-time checks** — local adjudication (field type, enum, slug) at write-time; referential/cross-doc integrity at `finalize`; the split is what stops the bootstrap deadlocking.
- **`finalize` defaults to autonomous** — the confirm-gate is an opt-in cascade setting; git/PR review is the durable correction point.
- **Content handoff splits by leaf kind** — fields inline (`--value`), slots via stdin (`--from-file -`, file-path convenience), never inline prose; `add-item` takes the id-source inline → returns the item address → prose via follow-up `set-slot` (all-at-once would resurrect the deferred form).
- **Instance provisioning: CLI creates, two triggers** — CLI always mints+places; triggered either workflow-provisioned (deterministic, id-source set by the workflow) or agent-initiated (`create`, judgment); deciding-to-create is reasoning, creating/placing is structure.
- **Agent-initiated `create` is workflow-gated** — the catalog of creatable types in a context is structure (workflow/cascade-owned); choosing among them is reasoning; the gate is a cascade setting.
- **`finalize` ≡ `validate` + commit** — one validation engine, two entry points, so report and gate can't diverge; findings are severity-tagged and finalize blocks only on the *blocking* class (severity per check is a cascade setting); `--dry-run` dropped — `validate` is the preview, `diff` shows the changeset.
- **Out-of-band reconciliation: binary, human-decided, blocks at `finalize`** — detect via `file ↔ CLI-state` hash; the agent blocks-and-routes on drift; import vs discard is the human's call; import needs round-trippable serialization (MVP detects + discards now, import lands with the format); three-way merge deferred.

### On-disk storage

- **Single-file canonical Markdown** — the `.md` *is* the source of truth, not a view over a model file; our locked principles (plain files are source, humans edit through git, OOB detected/reconciled) already imply it — a hidden model file would make the edited `.md` second-class.
- **Schema-driven parse** — the file is read against its type's schema, so sections/slots/fields are schema-located and need no markers; only instance-minted identity gets marked.
- **`{#id}` heading anchors are the only in-body markers** — they mark repeatable items (the one instance-minted body identity); the on-disk token *is* the address fragment, and the frozen slug survives a title rename.
- **One field syntax, two locations** — fields are always `key: value`; the doc's header block → front-matter, a section/item block → a trailing `key: value` group; slots are the prose.
- **id-source = the heading** — a doc's title is its H1 (frozen id = filename), an item's title is its `###` heading (frozen id = `{#id}`); same pattern at both levels.
- **One prose slot per section preferred** — multi-slot renders under schema-fixed sub-labels (matched like headings, no new marker); nudges schemas toward one-purpose sections.
- **One `.md` per instance; path = identity** — filed at the type's location, filename = frozen id; items live inside the parent file; physical order = order (reorder = move a block, no renumber).
- **Committed Markdown is the only source of truth** — everything else is a rebuildable cache or transient working area; delete it all, rebuild from the `.md`, lose nothing — the system degrades to plain Markdown.
- **All CLI state under gitignored `.tool/`** — edge index + file↔state hashes are derived caches, gitignored not committed (committing churns diffs and reintroduces dual-source-of-truth); they invalidate/rebuild when the checkout moves (stamped with HEAD/fingerprint).
- **Staging = gitignored scratch dir of working copies** — `.tool/tasks/<task-id>/` in the same Markdown format; realizes the concurrency primitive directly (isolated subdirs + CLI by-task-id join), not git worktrees (git's text-merge ≠ our deterministic join); a task is pinned to its base, base-mismatch is detected-and-routed (rebase deferred).
- **CLI orchestrates, git executes VCS** — the CLI never reimplements branch/worktree/merge; a workflow may trigger git deterministically; a fresh worktree rebuilds caches and seeds only local config, copies nothing else.
- **git merges code; the CLI merges managed docs — never the reverse** — blind text-merging a `.md` would corrupt `{#id}` anchors/structure and reintroduce non-determinism; overlapping doc-writes go through the CLI join, disjoint coarse work may use worktrees + git merge.

### Workflow dialect

- **Dialect shape: shared definition-grammar, divergent runtime** — a workflow is the skeleton's steps / addressing / `include` / override with read-path-only leaves (`placeholder`, `instruction`) and a `fan-out`/`join` step marker; no write path, no slots/fields, no persisted instance, no minting. The one-grammar bet holds on *how a workflow is built/addressed/composed/overridden*; doc = fill & persist, workflow = resolve & emit.
- **Composed output is ephemeral** — a derived view, re-composed each call from `definition + cascade + live state`, never persisted or source-of-truth; `--explain` re-computes the resolution tree on demand.
- **Composition is substitution, not control flow** — structure is fixed by `definition + cascade` (task-independent); only resolved values vary by task; adaptation lives at the cascade (project) or in agent reasoning (situation), never in composer branching.
- **Three placeholder kinds** — command-ref (cascade → literal CLI invocation, overridable, validated), data-value (live-state value / doc-slice, embedded), include (step/block by id, recursively expanded, cycle-checked); includes expand first then values resolve; an empty resolution yields empty text, never conditional prose.
- **Data-values: full graph navigation, no logic** — multi-hop relation traversal + fragment slicing, no filtering/selection; broken paths caught at validate-time (`workflow ↔ references`).
- **`fan-out` realization** — a step marker declaring a list-source (a data-value resolving to a collection) + a referenced sub-workflow; the CLI resolves the list and emits dispatch instructions, the assistant adapter launches (CLI owns the payload, assistant owns the launch — the CLI never spawns); sub-agents re-enter the composer (referenced, not inline); sub-task id = the fanned item's id; ack is `status + task_id`, state is re-derived from the CLI, never trusted from the message.
- **`join` is a barrier, merged by task-id order** — all sub-tasks complete, the CLI merges working areas by task-id (never completion order), the main workflow re-composes its post-join steps against the merged state; synchronization is via CLI state, not messages.
- **Fan out over the whole collection; nested `fan-out` banned** — no subset filtering (selection is logic; each sub-agent's workflow + reasoning handles "nothing to do"); parallelism is capped at one bounded level, mirroring no-repeatable-in-repeatable — tree-parallelism deferred.

### Validation engine

- **Validation is mechanical, never semantic** — the engine checks the CLI-owned half (references, structure, state) and never judges prose; deterministic predicates, no LLM in the engine. The moat is guaranteed referential integrity, not content quality; detect & route, never auto-fix; read-only — same effective state → same findings.
- **Fat engine, thin probes** — the engine owns scope→target resolution, the read-only effective-state graph/edge-index, scheduling, severity assignment, aggregation/report, and the `finalize` gate; a probe just declares an id + target-type + default-severity and implements a read-only `check(target, ctx) → [finding]`.
- **Engine-owns severity, via the cascade** — the probe suggests a default; the engine assigns the final severity from `project > team > pack-default`; the only option that honors the locked "severity is a cascade setting" — self-classify (severity baked into probe code) is rejected.
- **Finding = `{target, probe, severity, message, route?}`** — `route` is an optional tagged union (`fill-leaf` / `run-command` / `reconcile` / `none`) the engine **never executes**; message always present. Findings are navigable state, so a `fix-drift` workflow can `fan-out` over them and route each repair — closing the loop while staying detect-&-route.
- **Scope = effective state** — `task` validates `committed + working deltas + referential blast radius` (deltas overlaid on the committed edge index; what `finalize` gates on), plus `doc` and `store` scopes; blast-radius is required so a task can't commit breakage elsewhere.
- **Uniform probe interface, two implementations** — engine-native probes are built in (in-process); pack probes are deferred invoked processes (JSON-in/out, read-only and deterministic by contract — `doc-code` parses real code, so not declarative), behind the "not a public API yet" line.
- **MVP probes** — `workflow-refs` (every placeholder/include/command-ref resolves) and `file-state` (on-disk hash matches recorded state), both engine-native; `override-default` is engine-native at contract level (full logic awaits the override ladder); `doc-code` is the deferred pack probe.
- **The engine runs at validate/finalize** — the per-write field-type/enum/slug checks are the schema's *local adjudication*, a separate fast path, not the validation engine.

### Override system

- **Cascade: `project > team > pack-default`, by specificity** — `team` is a cross-project generality layer (not per-developer); most-specific-wins, so the project — the layer that differs most — overrides. Per-developer config doesn't fit a governance domain (doc types, workflows, conventions, severities).
- **Cascade disk locations** — pack-default ships with the pack; project = a committed config dir (source of truth, diff-reviewed); team = external `~/.config/<tool>/`; the in-repo `.tool/` stays purely derived/transient (no config in it).
- **Determinism = pure function of the resolved cascade** — composition is reproducible given the resolved cascade (a declared input), not project-state alone; anything that must be reproducible from the repo alone belongs at project level.
- **Per-developer `local` layer deferred** — if ever added it sits on top (`local > project`), restricted to non-structural / tighten-only; the cascade is layer-count-agnostic, so adding it later is free.
- **One delta model; the ladder's rungs are delta kinds** — `scalar-set` / `structural-op` / `slot-fill` / `tracked-fork`; the cascade is the ordered application of recorded deltas over a versioned base, so "ladder" and "cascade" are one system.
- **Delta manifest = config-format per layer; content as native files** — `scalar-set` is inline (`key = value`); content-bearing deltas reference native-format files (a step file), never inline-in-config — no prose-in-config.
- **Base-hash on all content-bearing deltas** — each records base-version + the target's base-hash, making upgrade reconciliation a stateless existence + hash compare (no old pack kept around).
- **`override-default` reconciliation** — engine-native probe on a guarded `tool upgrade`: per delta, target gone → `orphaned`; target content changed *and the delta depends on it* → `conflict`; else `clean`. Emits validation findings + routes (orphaned → remove/re-target; conflict → 3-way merge — agent proposes, human confirms).
- **Knobs = config-level fields, closed surface** — a scalar knob is a pack-declared typed field (enum/string/bool/int + default), type-checked like `set-field` and reconciled like any delta; only declared keys are settable (undeclared → error); structural change beyond the knob surface uses structural-ops by address.
- **Step/workflow definition format** — one file per step (md + YAML front-matter config + body = the prompt, with placeholders); a workflow is one file (front-matter + body = ordered `{{include}}`s); id = filename (frozen slug); step-ids resolve through the cascade (same-id higher-layer wins = override for free). Doc-type schema definition format kept separate (open).
- **Mermaid flow is a generated view** — derived from the composition (the includes), output on demand (`--explain` / `--diagram` / composed output), descriptive never prescriptive; never a hand-authored second source (would drift and back-door the banned DAGs/conditionals).

### Bootstrap sentence

- **One routing pointer, not content** — the bootstrap replaces the static rules file with a single permanent-context line that *routes*, never embeds rules/conventions; the moment it carries content it's the bloated rules file again. It names categories (workflow, project state, doc context), never their contents.
- **Single state-aware front door** — the agent runs one zero-knowledge entry; the CLI composes orientation / the right workflow / setup as appropriate, and the agent learns every other command just-in-time via command-refs. The agent's entire a-priori knowledge is "run the front door and follow it."
- **Must signal the benefit, not just instruct** — the sentence establishes `tool` as the *authoritative, current, assembled, validated* source (vs raw/scattered/possibly-stale files), so the agent perceives it as the path of least resistance; the front-door's self-describing output reinforces it (advertise + demonstrate). "Get your instructions" alone is too thin a hook to beat the grep instinct.
- **Three jobs, nothing more** — (1) `tool` is the interface + single current source for what you need; (2) files are storage — never read/edit managed docs directly (code untouched); (3) one front door + all writes through the CLI.
- **Form: a 3-sentence micro-block** — final wording is current truth in [design/bootstrap.md](design/bootstrap.md); command tokens (`tool` / `tool start`) are product-name placeholders, placement is the per-assistant adapter (separate topic), and the sentence is assistant-neutral.

### Doc-type schema definition format

- **Doc-type definitions are config, not documents** — a schema is declaration-heavy with only light prose (hints), so it joins the *config* family (with the delta manifests), not the md + front-matter *document* family. Content lives in its natural format; this was never "everything is md + front-matter."
- **Structured config, YAML, document-order, self-documenting** — sections listed top-to-bottom as they appear in the doc, leaves under them, one-line hints inline, so the source reads like the doc's shape for humans and LLMs alike; YAML (over TOML/JSON) for readable nesting.
- **One file per type (definition); instances stay one-file-each** — the whole schema for a type is one bounded file (`adr.yaml`) that doesn't grow with usage; the actual ADRs are separate `.md` instances ([storage.md](design/storage.md)). Rule: *externalize a unit for reuse* (workflow steps), *inline it for self-containment* (doc-type sections); `include` is the escape hatch for a genuinely-shared section.
- **Generated `--template` view** — the engine renders a blank instance from the schema on demand (derived, like the mermaid flow), so "what does this produce" is legible without the source being a template.
- **Validation hooks fall out** — no separate probe declaration: a `code-anchor` field means `doc-code` applies, a relation's cardinality is checked, severities are cascade knobs. The schema's typed leaves + relations + the cascade drive validation.
- **One config language (YAML) across the config family** — the delta manifests are re-expressed in YAML to match schemas (notation only; decisions unchanged).

### Assistant adapter

- **Neutral core, per-assistant profile** — the core CLI knows nothing about any assistant; a small adapter *profile* (config-family YAML) holds all assistant-specific knowledge (inject / allowlist / spawn). Swap the profile → new assistant, zero core changes. Two orthogonal axes: domain = pack, assistant = profile.
- **Generated + minimal, not hand-maintained** — `tool setup` / `adapter install` generates the adapter from the profile and regenerates it on upgrade, so it can't rot into a new bloated rules-pile; no per-command wrappers (the agent shell-calls `tool`, learning commands JIT).
- **Three responsibilities** — (1) inject the bootstrap; (2) make `tool` frictionless (allowlist/permissions — the path-of-least-resistance the bootstrap relies on); (3) bind the spawn mechanism.
- **Inject via best mechanism: hook (primary) + static line (floor)** — a hook that *calls the CLI* injects the bootstrap + a thin live nudge at session start (fresher, higher-salience; e.g. Claude Code `SessionStart`); a static line in the always-loaded file (`CLAUDE.md` / `AGENT.md` / Cursor rules) is the universal floor. Both, profile-driven; capability-dependent (profile declares what the assistant supports).
- **Hook stays routing + thin nudge, never a content dump** — same "routing, not content" discipline; its value is freshness/salience, not volume; the agent still pulls real context JIT. It also makes the bootstrap's "advertise + demonstrate" automatic (the hook runs the front door, so the first context already shows live orientation).
- **Spawn binding = a launch template in the profile** — the CLI renders its fan-out dispatch (`task_id` + entrypoint) through the profile's template into the assistant's primitive (Claude Code: a Task-tool call running `tool workflow W --task <sub>`); CLI owns the payload, the template is the only assistant-specific bit.
- **Profiles ship with the CLI + are installable** — known assistants (Claude Code first) ship in-box; more profiles can be installed later — a swappable layer parallel to packs.

## 2026-05-25

### Review remediation

Resolutions to gaps surfaced by a four-agent read-only review of all docs.

- **Task origination: `tool start` is the front door *and* create verb** — `tool start ["<intent>"] [--workflow X]` mints the task (slug-from-intent, base = HEAD, opens `.tool/tasks/<id>/`), minted at workflow-*selection* time (not the bare orientation call). Closes the MVP's missing "discover" entry.
- **Workflow selection: hybrid, options-always-shown** — the catalog is always offered with a *recommended* default (a hint, never a silent force); the agent picks; a human can force one via a catalog-generated adapter command. The CLI never infers a workflow (model-free).
- **The default is a router workflow** — "no workflow specified" composes a cascade-overridable *selection-guidance* workflow that presents the catalog + each workflow's "when-to-use" hint and routes the agent to pick. Workflow definitions gain a "when-to-use" front-matter hint; task context roles (e.g. `spec`) are workflow-declared and agent-bound (CLI never infers).
- **Data-value path grammar** — `head ("." relation)* ("#" fragment)?`: `.` crosses a relation edge (inter-doc), `#` enters a doc (intra-doc), `:` names a doc literally; `head` is a declared live-state root (engine-native `task`/`store`, pack-provided `milestone`/domain) or a literal `type:name`. Resolves to scalar / doc-slice / collection; pure navigation, no logic; validated at compose-time.
- **Finalize blocks only on task-controllable integrity** — forward-ref integrity, required slots, malformed values block (the task can fix them). Inverse/minimum-cardinality ("a PRD must have ≥1 SPEC") is *completeness*, not integrity — advisory by default, hard-enforced only at store/milestone scope, **never** a per-task finalize gate. Resolves the cross-task deadlock.
- **fan-out join: sub-agents validate but never commit; the parent finalize is the commit boundary** — one commit per sub-task in task-id order (squash a cascade knob). Slug collisions are handled by isolation + CLI-owned wiring (the join suffixes the loser and rewrites its *local* self-refs in the merge pass; workflow-provisioned ids are sub-task-derived and can't collide). No sub-agent runs git → no races.
- **Out-of-band reconciliation: files are truth** — a conformant, non-conflicting external edit is *accepted* (import is just the normal canonical-Markdown parse, not a deferred feature); a nonconformant edit *blocks* with a precise conformance error; a true conflict *routes to the human* — discard is an explicit choice, never silent. No silent data loss; three-way merge deferred.

### Implementation foundation

First decisions about *how we build it* (vs. `design/`'s *what it is*).

- **New `implementation/` directory, parallel to `design/`** — build decisions are a new category that don't fit the language-neutral design docs; same conventions (descriptive names, no number prefixes, cross-reference never restate). `design/` = what the system is; `implementation/` = how we build it. Detail in [implementation/language-runtime.md](implementation/language-runtime.md).
- **Core language = Rust** ([implementation/language-runtime.md](implementation/language-runtime.md)) — engine + CLI in Rust because the design is union-heavy (≥5 tagged unions + enums) and the project's value *is* exhaustive correctness; Rust makes "handle every case" a compile error, not a discipline. Ties or wins on cold-start, binary size (~2–5 MB), and Markdown byte-offsets; borrow-checker cost is low for per-invocation parse→transform→serialize work. **Why-not:** TS-on-Bun (compiled-binary startup is tens-of-ms not single-digit; `ink` is Node-bound so Bun XOR a good TUI; the Markdown edge dissolves into a language-agnostic strategy), Go (close — best TUI/velocity, but no exhaustiveness), Python (startup + distribution hit our hardest constraints; AI-ecosystem edge moot since the core makes no LLM calls). Decided after research that **reversed an initial TS-on-Bun lean**.

### Implementation: parsing & on-disk round-trip

How the CLI reads/edits/serializes the canonical Markdown ([implementation/parsing.md](implementation/parsing.md)). Mechanism only; the on-disk *format* stays in [design/storage.md](design/storage.md).

- **Offset-splice, never re-stringify; re-parse to validate** — every Markdown AST stringifier reformats, so the diff-clean path is splicing edits into the original byte buffer (the `markdownlint --fix` model). This is the strategy the whole doc hangs on; it makes lossless round-trip a property of *approach*, not library.
- **Parse model: full CommonMark block parse (`pulldown-cmark`), mapped onto the schema** — a line scanner misreads `##` inside code fences; a real block parse gives trustworthy byte offsets. Slot prose is an **opaque span** — parsing draws the determinism boundary literally (CLI owns block structure, never interprets prose). Rejected `comrak` (line/col-only spans) and `tree-sitter-markdown` (C dep, highlight-grammar) — the latter kept as a fallback.
- **Front-matter is a flat field block, not full YAML** — "one field syntax, two locations" forbids two field syntaxes; fields are schema-typed so YAML's dynamic typing fights us; flat lines splice diff-clean. Body field groups are a trailing **bullet list** (a distinct block type the schema disambiguates against prose). One grammar, two structural frames (`---` fences / bullets).
- **`{#id}` via pulldown's heading-attributes extension** — gives clean title text + frozen id separately; items only (doc identity is the path); missing→mint-on-import, duplicate/malformed→conformance error; slug rules stay minting's, not parsing's.
- **Write pipeline: splice to edit, generate only new** — one canonical writer (serves `create`/`add-item`/field-insert/`--template`) with golden-tested parse↔write symmetry; **validate-after-write** re-parses and aborts on anomaly (local gate, distinct from the `finalize` engine). Edits land in the task working area; committed file untouched until finalize.
- **Conformance falls out of the parser** — the schema-mapping *is* the conformance check; diagnostics are located, collected-with-resync, and use the validation `finding` shape. Conformance is **binary/intrinsic** (the pre-gate below the engine); validation severity is graded/cascade-tunable.
- **Round-trip contract: idempotent on canonical content, surgical on edits** — minimally + locally canonicalizing on first touch (final newline, BOM strip, EOL matched locally never globally rewritten); UTF-8 only; enforced by golden + property/fuzz tests, not assertion.

### Implementation: module layout

How the locked architecture becomes Rust crates ([implementation/module-layout.md](implementation/module-layout.md)). Realization only; the architecture itself stays in VISION/CLAUDE.

- **Two-crate workspace: `engine` (lib) + `cli` (bin), `cli → engine`** — the engine/frontend boundary is the one with architectural weight (neutral, empty, makes no LLM calls; an `mcp` bin later is a third crate over the same engine). Engine internals are **modules** (cascade/schema/parse/registry/compose/validate/state/index), not micro-crates; `parse` is the first split-out if needed. Rejected single-crate (engine not separately consumable) and micro-crates (premature ceremony).
- **Dev pack (and adapter profiles) embedded in the `cli` binary** — data not logic, embedded so `tool` is one self-contained artifact; the engine reads pack-default through a **source abstraction**, staying empty. Pack versions with the release (= what override-reconciliation needs). Filesystem/installable packs come later through the same abstraction. Boundary stays internal discipline, not a public API.
- **I/O split: CLI locates, engine resolves** — `cli` locates the three cascade layers (embedded pack-default · external team · in-repo project) + repo root and renders results; `engine` resolves the cascade and owns managed I/O (docs, `.tool/`, edge index). Feed layers in, assert results out → the core is directly testable.
- **Renderers live in `cli`, downstream of engine result types** — result types are `Serialize`, so JSON is generic; agent-text/human are per-type. Default agent-text (the agent reads non-TTY stdout); TTY→human-pretty; `--format` overrides; ratatui TUI is an additive post-MVP module preserving the non-interactive floor. The composed-workflow agent-text renderer is gated on the open emitted-format micro-syntax.
- **Adapter generation in `cli`** — profiles embedded (Claude Code first); generates/regenerates the static line + allowlist + catalog-derived launchers into the host project. MVP = line + allowlist; hook + spawn binding are post-MVP (spawn rides on fan-out).
- **Probe boundary in `engine`: `Probe` trait, in-process (MVP) + subprocess seam (post-MVP)** — MVP ships the trait + in-process probes only; the trait must admit the subprocess (JSON-contract) impl with zero engine change; probe executables live outside the workspace (any language).

### MVP scope clarifications

Pinning what the first runnable single-task loop actually reads/writes, before phase planning. Current MVP boundary lives in [CLAUDE.md](CLAUDE.md); this is the reasoning.

- **MVP single-task is spec-*less*** — no `task.spec` reference, no `spec` schema ships; the "what" comes from the human `intent` + the codebase. A spec type with no creation flow and a deferred `doc-code` validator would be dead weight; adding spec support later is a trivial pack addition (schema + one `locate` placeholder). Refines an earlier "spec-optional." The placeholder taxonomy is still fully proven spec-lessly — command-ref (`{{cli.finalize}}`), include (the workflow's step includes), data-value (intrinsic task `intent` + the provisioned commit-doc context).
- **Front door built, router not** — `tool start` (mint task, resolve default workflow, orient) is essential; the **router** selection-guidance workflow is not built. The "no-workflow-specified" default is a cascade knob, set to `single-task` for the MVP (the design's sanctioned alternative to a router-default); it flips to the router when ≥2 work-workflows exist. Model-free and never-force hold vacuously at one option.
- **MVP deliverable = `engine` + `cli` + a minimal embedded dev pack + the Claude Code profile** — the engine ships empty by invariant, so a runnable MVP *must* include pack content. Pack: **`commit`** doc-type only (engine-native field types → no adjudicator), **`single-task`** workflow (no router) + its ~3 steps, cascade pack-default config (no-workflow default = `single-task`, MVP probe severities, finalize-autonomous, commit→git rendering), and **zero probes** (the two MVP probes are engine-native code, not pack data). Out of MVP: spec/ADR/PRD types + their creation flows, the router, planning/milestone/setup workflows, pack probes (`doc-code`), fan-out/join, and the adapter hook + spawn binding.
- **`finalize` renders the commit doc into the git commit** — the `commit` doc-type's *sink* is the git commit message, not a repo file (transient instance, working-area only). Finalize validates (required `summary`, valid `type`, no drift) → renders `type(scope): summary` + body + trailers via the commit schema + cascade → `git add` + `git commit` the task's code changes + promoted managed docs as **one** commit → discards the working area. Clarifies that **the CLI commits code too** — the agent *authors* code directly (the bootstrap's "code untouched" is about authoring), but *committing* is the CLI's job at finalize ("CLI orchestrates, git executes"); one task → one logical commit. MVP: current branch, working-tree staging (precise task-scoping is a refinement), refuse empties.
