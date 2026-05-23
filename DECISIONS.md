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
