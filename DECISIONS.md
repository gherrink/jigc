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
- **Engine-owns severity, via the cascade** — the probe suggests a default; the engine assigns the final severity from `project > local > pack-default`; the only option that honors the locked "severity is a cascade setting" — self-classify (severity baked into probe code) is rejected.
- **Finding = `{target, probe, severity, message, route?}`** — `route` is an optional tagged union (`fill-leaf` / `run-command` / `reconcile` / `none`) the engine **never executes**; message always present. Findings are navigable state, so a `fix-drift` workflow can `fan-out` over them and route each repair — closing the loop while staying detect-&-route.
- **Scope = effective state** — `task` validates `committed + working deltas + referential blast radius` (deltas overlaid on the committed edge index; what `finalize` gates on), plus `doc` and `store` scopes; blast-radius is required so a task can't commit breakage elsewhere.
- **Uniform probe interface, two implementations** — engine-native probes are built in (in-process); pack probes are deferred invoked processes (JSON-in/out, read-only and deterministic by contract — `doc-code` parses real code, so not declarative), behind the "not a public API yet" line.
- **MVP probes** — `workflow-refs` (every placeholder/include/command-ref resolves) and `file-state` (on-disk hash matches recorded state), both engine-native; `override-default` is engine-native at contract level (full logic awaits the override ladder); `doc-code` is the deferred pack probe.
- **The engine runs at validate/finalize** — the per-write field-type/enum/slug checks are the schema's *local adjudication*, a separate fast path, not the validation engine.
