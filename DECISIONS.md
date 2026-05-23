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
