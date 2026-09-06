# Document-type definition schema

The **document dialect** of the [structural grammar](structural-grammar.md). The skeleton — units, blocks, leaves, addressing, minting, composition, override, and the validation framework — is dialect-neutral and specified there; **read it first.** This document defines what is specific to documents: the leaf kinds (`slot`, `field`), the determinism boundary as it falls inside a document, the `relation` construct for cross-references, and the document-specific validation probes.

This specifies the **schema** — the vocabulary *and* its on-disk form. The actual ADR / SPEC / PRD / commit definitions are development-pack content authored *in* this vocabulary and shipped with the pack; their file format is [On-disk definition format](#on-disk-definition-format) below. Notation is **illustrative** ([what that disclaims](../CLAUDE.md#how-we-work-together)); for the *why*, see [DECISIONS.md](../DECISIONS.md); for the framing, [VISION.md](../VISION.md) → Document model.

## The trichotomy: section / slot / field

In the document dialect the skeleton's **unit** is a **section**, and its **leaves** are **slot** and **field**. Section + slot + field is the [determinism boundary](../VISION.md#the-determinism-boundary) projected onto a single document:

- **Section** — structure the CLI owns (heading, ordering, required/optional). *North of the line.*
- **Field** — a wiring/data value the CLI owns (typed, validated; cross-ref targets included). *North of the line.*
- **Slot** — prose the LLM owns. *South of the line — the only thing south.*

The sharp test that places a leaf on one side or the other:

> **A field is a value the CLI can *adjudicate*. A slot is a value only a human or LLM can judge.**

The CLI can reject a malformed date, a non-member enum, or a dangling reference — those are fields. The CLI cannot reject prose for being *wrong* (only for being absent, or for an embedded reference that fails to resolve) — so prose is a slot. The LLM may *propose* a field value; it can never make it *wrong*, because the CLI governs it. Cross-references prove fields must be a distinct kind: per [VISION.md](../VISION.md) principle #3 the CLI owns cross-ref wiring, so a reference target can never be slot prose.

## Leaves

### Slot

LLM-authored prose on the write path. The CLI guarantees the slot's placement and wiring, never its content. A slot declares:

- whether it is **required** (must be non-empty at `finalize`),
- an optional **authoring hint** (guidance surfaced to the LLM when it fills the slot).

**Addressing.** A section's single unnamed slot is addressed by the unit's address (`#unit`) — there is no named leaf below it in the schema. Multi-slot **items** render each slot under a schema-fixed sub-label, and each sub-label is the slot's leaf id (`#unit/sub-label`). *(Precision, 2026-07-23: the engine models multi-slot at the **item** level only — `SectionBody::Simple` carries a single `slot`, and the Simple-section multi-slot case stays deferred for want of a driver; see [storage.md](storage.md) → Identity, order, fields, slots, items. The sub-label depth is load-bearing for the slot heading-depth ceiling — [parsing.md](../implementation/parsing.md).)* The address grammar permits both depths uniformly ([structural-grammar.md](structural-grammar.md#addressing)).

The CLI's only checks on a slot are presence and the integrity of any references embedded in its prose (a lighter "mention" check, distinct from `field`-refs — see [In-prose mentions](#in-prose-mentions-settled-m33)).

### Field

A CLI-adjudicated typed value. Field **types** split along the engine/pack seam, mirroring the validation-probe split and keeping the engine empty of domain content:

- **Engine-native:** `enum`, `string` (constrained: maxlen / pattern — also the id-source for slugging), `date`, `bool`, `int`, `ref` (a reference to a managed document or fragment; carries relation metadata — see [Cross-references](#cross-references--ref-fields-with-relation-metadata)).
- **Pack-provided:** domain types such as `code-anchor` (points at a file / symbol / test). Their **adjudicator is pack-supplied** — "does this symbol exist? does a test cover this criterion?" is exactly the development pack's `doc ↔ code` probe. The meta-schema permits field types whose adjudicator ships in a pack — the mechanism is [Pack-declared field types](#pack-declared-field-types-m10) below, built in M10 (`code-anchor` is the first).

Field **provenance** (who supplies the value) is partly implied by type:

- **CLI-derived** — `date` (now), the minted id, the slug. The CLI computes it; no one else touches it.
- **Controlled-choice** — `enum`; the LLM or human picks, the CLI validates membership.
- **Referential** — `ref` / `code-anchor`; proposed by LLM or human, integrity-validated by the CLI or a pack probe.
- **Free-but-typed** — a constrained `string`; written by LLM or human, shape-validated by the CLI.

### Pack-declared field types (M10)

The split above is a *seam*, not a closed enum. Until M10 it was aspirational — `code-anchor` existed only as a hardcoded engine variant adjudicated as a bare string, and a pack could not supply a field type at all (the M10 planning audit, [DECISIONS.md](../DECISIONS.md) 2026-06-06). M10 makes it a **genuine extension axis** (the human chose the real axis over a bound-to-engine-native shortcut), with `code-anchor` as the **first pack-declared field type**:

- **A pack declares a field type** — a `(name, adjudicator)` pair: the type's spelling (`code-anchor`) and the **probe** that adjudicates it. The engine's field-type model opens from a closed enum to *engine-native variants + a pack-declared variant* carrying the declared name and its adjudicator binding. The engine ships **no** pack-declared types itself (the engine-empty invariant holds); `code-anchor` lives in the dev pack.
- **Adjudication splits write-time vs finalize-time.** A pack-declared type may carry an optional **write-time shape check** (cheap, local — e.g. "non-empty, single-line, parses as `path#symbol`") that runs in the same `check_value` path the engine-native types use. Its **real adjudicator is the bound probe at finalize** — for `code-anchor`, the `doc-code` probe ([validation.md](validation.md) → The `doc-code` probe) which reaches real code. This is why a `code-anchor` authored by an out-of-band edit (the M10 acceptance authors `spec` criteria through the editable channel) is still caught: the probe, not a write-time gate, is the adjudicator.
- **The binding falls out of the type, no separate declaration** — a `code-anchor` leaf *means* `doc-code` applies, exactly as a `ref` leaf means `ref-resolves` applies and a relation's `card` is enforced. The pack declares the type→probe binding once; every doctype that uses the type inherits the adjudicator.

**`code-anchor` value grammar:** `<repo-relative-path>#<symbol>` (a bare `<path>` is a file-existence check) — see [validation.md](validation.md) → The anchor grammar. The two shipped M10 uses: `adr.cites-code` (a header field — the floor) and `spec` `criteria/<id>/maps-to-test` (a repeatable-block leaf — the headline). Both **optional**; the probe blocks on a *dangling* anchor, never on absence.

## Sections and repetition

A document is an ordered list of **sections** (skeleton units). A section is **simple** (one block of leaves) or **`repeatable`** (a list of blocks; see [structural-grammar.md](structural-grammar.md#repetition) for the mechanism). In the document dialect a repeatable section's designated **id-source must be a `field`** — never a slot — so the CLI has a short, adjudicable value to slug. The id-source field is rendered on disk as the item's **`###` heading** (with the minted `{#id}` anchor), not as a trailing `- key: value` field — see [storage.md](storage.md) → Identity. The field-vs-heading split is *structural vs presentational*: structurally a field (typed, adjudicated, schema-located); rendered as a heading for diff-clean human-editable storage.

The same construct scales both ways:

- down — an item is a single `slot` → an ID'd list of prose bullets;
- up — an item is `{ statement: slot, maps-to-test: field }` → a record (e.g. a SPEC criterion).

Consequence, accepted deliberately: a SPEC's criteria live *inside* the SPEC as records, addressed like `spec:auth-flow#criteria/rate-limit/statement` — not as separate child documents. The SPEC stays one cohesive, reviewable artifact; **"small footprint" therefore means one purpose, not few lines.**

## Cross-references — `ref` fields with relation metadata

A cross-reference is a **`ref`-type field** that carries **relation metadata** alongside its value. The schema declares the relation by declaring the field; one source of truth, one place to look.

A `ref` field declares:

| key | meaning | required |
|---|---|---|
| `id` | the field id (and the on-disk key) | yes |
| `type: ref` | marks this field as a cross-reference | yes |
| `to:` | the target type (e.g., `adr`, `prd`) | yes |
| `card:` | UML-style cardinality (`"0..1"` / `"1"` / `"0..*"` / `"1..*"`) | default `"0..1"` |
| `inverse:` | name of the derived back-edge in the target type's read view | required for any ref the target type expects to surface inversely |
| `inverse-card:` | inverse-side cardinality (completeness obligation) | optional |

Two worked declarations:

```yaml
# ADR's optional supersedes ref to another ADR; the target ADR's read view surfaces "superseded-by"
- { id: supersedes, type: ref, to: adr, card: "0..1", inverse: superseded-by }

# SPEC required to derive from a PRD; PRD's completeness obligation is at least one SPEC
- { id: derived-from, type: ref, to: prd, card: "1", inverse: has-specs, inverse-card: "1..*" }
```

This makes "normalized database, cross-reference instead of duplicate" real: `ref` fields are the schema's foreign-key edges, and validation walks them.

**`to:` is required *and* enforced, against the composed schema set (M50).** The table above has marked `to:` required since the first draft; nothing checked it, and nothing checked that the type it names exists. Both are now refused at **pack-load** — a `type: ref` declaring no `to:`, and a `to:` naming a doctype the *loaded composition* does not contain — because a `ref` is a **write address the schema advertises**: [`doc schema`](doc-read-surface.md) projects it, `set-field` accepts a `<type>:<slug>` value for it, and `schema-conformance.ref-resolves` blocks a finalize over it with a route that says *create the target*. A dangling target makes all three honest about a thing that cannot exist, and the blocking route's create arm answers `create.unknown-doctype` — a blocking finding with an unrunnable arm, inside the [route floor](surface-contract.md). The subject is every locus a `ref` can sit at (a section's field group, a repeatable item block, and each nested repeatable inside one), and membership is the **composite's** doctype set, never a constituent pack's in isolation — see [multi-pack.md](multi-pack.md) → *A ref target is composition-scoped* for what that means when the same pack composes two ways.

**Why one declaration, not two.** A previous schema version had a parallel `relations:` block alongside `fields:` — the conceptual "two facets" framing ("instance endpoint + type-level constraint") rendered as two separate schema blocks. That framing is right *conceptually* — the two facets do exist — but they're properties of the same thing: a typed field with extra metadata. Splitting them across two schema blocks would have:

- forced parsing, validation, and writer-generation to consult two declarations to handle one cross-reference,
- left "is `supersedes` a field, a relation, or both?" with no good answer,
- made overrides messier (an override touching cardinality would address the `relations:` block; one touching the value type would address `fields:`).

One field declaration with relation keys is the single home: parser sees a field, validator sees a field, writer sees a field, address grammar treats it like any leaf. The type-level constraint travels *with* the field.

**Address.** A `ref` field has an address like any leaf (`adr:bar#status/supersedes`), per the variable-depth grammar ([structural-grammar.md](structural-grammar.md#addressing)). Its **value** is also an address (the target it points at) — the same grammar at both ends, per the "one grammar, every reference" rule.

**Storage.** A `ref` field renders like any other field on disk ([storage.md](storage.md)):

- in a `header: true` section → front-matter (`supersedes: adr:single-node-cache`),
- in a body section → trailing bullet (`- supersedes: adr:single-node-cache`),
- `card > 1` → inline flow list (`relates-to: [adr:a, adr:b]`).

### Bidirectional, but the inverse is derived — never stored

Cross-references are bidirectional at the **graph/view** level and single-authored at the **storage** level:

- the **forward ref is authored once** (the SPEC stores `derived-from: prd:billing`, placed via the CLI, with a home in a section);
- the **inverse is derived by the CLI** — "PRD `has-specs` […]" is computed by walking refs and **injected into the PRD's read view**, never written into the PRD's file.

This is the *reliable* form of bidirectionality, forced by three locked invariants:

- **Single source of truth** — the back-edge cannot drift from the forward edge because it *is* the forward edge read backwards. The graph is whole by construction; no asymmetric period, no backfill migration.
- **Diff-friendly storage** — a stored back-edge would churn the target's file every time a new referrer appeared. Deriving keeps it stable in diffs.
- **Concurrency** — under `fan-out`, many sub-agents referencing the same target would all collide on the target *file* if the back-edge lived there. Derived = zero contention.

The determinism boundary holds: the forward ref is placed by the agent-via-CLI; the reverse edge is derived structure the CLI owns. Two riders:

- The CLI keeps a **rebuildable edge index** — a derived map of every cross-reference edge across the store (forward, with inverses computed), a cache whose source of truth stays the documents — so a read needn't rescan the store, and validation is cheap (walk the index).
- **Inverse-cardinality and orphan obligations** ("a PRD must have ≥ 1 SPEC") are **completeness, not integrity** — they depend on *other* tasks (the SPEC is a later task's job), so they are **never** a per-task `finalize` gate: advisory by default, surfaced at store/milestone scope (report-only there — the "hard-enforced at store" aspiration was never built; [validation.md](validation.md) → Integrity vs completeness + the severity inventory). *(This is specifically about **cross-task** completeness. A single doc's conformance to its **own** schema is self-contained — not another task's job — so the M34 store-scope **schema-conformance** detector and the migration-upgrade gate that blocks adopting a non-conformant corpus are integrity checks, **not** the completeness gate this line forbids; [corpus-migration.md](corpus-migration.md).)* Forward-ref integrity *is* gated at `finalize`, because the task can satisfy it against the **committed store + the same task's working area** — see [validation.md](validation.md) → Forward-ref resolution for the policy on cross-task forward-refs (not supported in MVP).
- **`ref` fields declare backward relationships to existing or same-task-created artifacts** — never planning markers. `supersedes: adr:b` means "I replace the earlier accepted ADR B," not "I plan to supersede a future ADR B"; refs to docs another task will create are unsupported (see [validation.md](validation.md) → Forward-ref resolution). If a provisional-ref pattern is ever needed, it lands as an explicit feature with its own design.

## How this keys the rest

The trichotomy and addressing are the binding surface for systems designed in their own part-docs:

- **Write-command vocabulary** — `set-slot`, `set-field`, `add-item`, and the structural ops each target an address; the trichotomy *generates* the verb set.
- **Override ladder** — its by-ID structural ops operate on sections, the document dialect's units (see [structural-grammar.md](structural-grammar.md#override)).
- **Validation** — one engine, addressable targets; the document dialect contributes the pack-provided `doc ↔ code` probe and relation-integrity checks (see [structural-grammar.md](structural-grammar.md#validation)).

## On-disk definition format

A doc-type definition is **config, not a document** — declaration-heavy with only light prose (hints) — so it lives in the **config family** ([overrides.md](overrides.md)) as **structured YAML**, not the md + front-matter form used for instances ([storage.md](storage.md)). (The parallel step/workflow definition format is in [workflow-dialect.md](workflow-dialect.md).)

Three rules keep it legible to humans *and* LLMs:

- **Document-order** — sections listed top-to-bottom as they appear in the doc, leaves under them, so the source reads like the doc's shape.
- **Self-documenting** — each slot/section carries its one-line hint inline.
- **One file per type** — the whole schema is one bounded file (e.g. `adr.yaml`) that does *not* grow with usage; instances are separate `.md` files. A genuinely-shared section can be pulled in with `include`, but inline is the default — sections are mostly self-contained, unlike workflow steps, which are externalized because they're reused.

```yaml
# doctypes/adr.yaml
type: adr
location: decisions/
id-from: title                       # filename = slug of the title

sections:
  - id: status
    header: true                     # → front-matter on the instance
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date,   type: date, set: on-create }
      - { id: supersedes, type: ref, to: adr, card: "0..*", inverse: superseded-by }
      - { id: cites-code, type: code-anchor }
  - id: context
    slot: { hint: "Why a decision was needed — the forces at play." }
  - id: options
    slot: { hint: "Alternatives weighed and why they lost — only when the choice needed weighing.", optional: true }
  - id: decision
    slot: { hint: "What we decided, in a sentence or two." }
  - id: consequences
    slot: { hint: "Tradeoffs and follow-on effects." }
```

**An `optional: true` slot section is exempt from `required-slot-present`** — a conformant instance may leave its prose empty, and (unlike a required slot) an empty optional slot never blocks finalize. Its **heading is still rendered** (the canonical writer emits every schema section's `##` unconditionally; optionality governs prose, not heading presence), so an optional slot is still a *section* structurally. The adr `options` slot (M36, added after `context`) is the first optional slot on a persisted doctype; because adding it inserts a whole `## Options` section, shipping it is a real **v1→v2 schema bump** with an `AddedOptionalSection` corpus migration, not an in-place amendment ([corpus-migration.md](corpus-migration.md) → the transform). The `context`/`decision`/`consequences` slots stay **required** ([DECISIONS.md](../DECISIONS.md) → 2026-07-02 harvest — the ceremony targets a human reader; the trivial lane is the commit body).

A repeatable section keeps its item-template inline:

```yaml
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: "The criterion, testably phrased." } }
        - { id: maps-to-test, type: code-anchor }
```

Within an item template, leaf ids and nested repeatable **block ids share one namespace** and must be unique — the pinned JSON item object keys them side-by-side ([doc-read-surface.md](doc-read-surface.md)).

**Repeatable items are conformance-validated per item (M16).** The MVP `schema-conformance` path adjudicated only simple sections (it skipped repeatable bodies); **M16 lifts that limit** — `required-slot-present` / `required-field-present` / `field-value-conformant` now run **per item** over a repeatable block's leaves, so a malformed entry (an empty required slot, a non-member enum) blocks at finalize, addressed at `#section/item/leaf`. The **`id-from` source field is exempt** from `required-field-present`: it renders as the item's `###` heading, not a trailing bullet, so checking its presence as a field would false-fail every conformant item. **Conformance-only** — no M16 doctype carries a per-entry managed `ref`, so `ref-resolves` over a repeatable-item edge stays deferred ([validation.md](validation.md#repeatable-section-conformance-m16)).

Two derived conveniences, no extra source:

- **`--template` view** — the engine renders the *blank instance* a schema produces (headings + marked slots/fields + relation notes) on demand, like the generated mermaid flow, so "what does this produce" is legible without the source being a template.
- **Validation needs no separate declaration** — it falls out of the typed leaves + relations + the cascade: a `code-anchor` field means `doc-code` applies, a relation's `card` is enforced, severities are cascade knobs ([validation.md](validation.md)).

**Authored metadata fields (M11).** A schema may carry top-level **`description`** / **`usage`** authored-prose fields (siblings of `type`/`location`/`id-from`, not inside `sections`) — the doc-type-level prose the [`describe`](introspection.md) self-description surface projects. They are a *third* prose category: not a `slot` (write-path, per-instance, LLM-filled) and not a `field` (typed leaf), but human-authored-at-definition-time usage prose, optional and skip-on-absent. Full shape and the boundary (`description` = what it *is*; `usage` = when/why to reach for it; never mechanism) in [introspection.md](introspection.md).

**Overrides** target sections/leaves by ID within the file (`adr#status`); a schema-structural delta's fragment is a small YAML section declaration — the config-family counterpart to a workflow's step-file fragment ([overrides.md](overrides.md)). The core leaf/relation keys (`id-from` — the YAML spelling of the prose *id-source* — plus `card`, `set`, `of`, `header`, `to`, `inverse`, `inverse-card`) are **shipped and golden-locked** (`engine::schema`), and are exactly the on-disk format **M33 declares frozen v1** ([corpus-migration.md](corpus-migration.md)).

**A mis-keyed leaf is refused, not absorbed (M49).** The lock above read `deny_unknown_fields` until M49, and that was false at four mappings: `Section` cannot carry the attribute beside its flattened `SectionBody` (serde forbids it), and an **untagged** enum variant — `SectionBody::Simple`, `Leaf::Slot`, `Leaf::Repeatable` — absorbs an unknown key silently. Worse, a leaf whose *own* guard fired (a `patern:` typo on a field, a stray key inside a `slot:`) failed the whole `repeatable:` body, and the simple variant's two keys both default — so the section fell through to an **empty simple section**: erased from every surface at exit 0 (verified: the typo on `commit.trailers.key` made `jigc doc schema commit` list `summary`, `body`, and no `trailers`). The engine now runs a shape pass over that whole axis — every mapping the model cannot deny — **after include-expansion and before deserialization**, refusing an unknown key with a located `<type>#<section>[/<leaf>]` message naming the key. The two leaves that *do* deny (`Field`, `Slot`) are delegated to serde, so their key sets are stated once. On a manifest-governed doctype the freeze gate would have caught the typo second-hand; the exposure this closes is the pack the freeze does **not** govern — a project-layer or third-party pack, where a typo silently deletes a section from every surface.

**Doctype-level placement/display keys (additive, M37–M38).** Alongside `type`/`location`/`id-from`/`singleton`, the `Schema` carries three optional top-level keys, each `Option` with serde-skip-when-absent (so a doctype that omits them serializes byte-identical — its frozen `schema-hash` is unchanged, the freeze-safe pattern): **`display-title`** (M37 — overrides a singleton's H1 display text; H1-only, never the filename/slug), and **`placement: { file: <repo-root-relative-path> }`** (M38 — a singleton's *literal-file home*, replacing the `<location>/<slug>.md` dir model; `location` is then `None`; full semantics + the ~9-site path↔identity census in [storage.md](storage.md) → Placement). *(`root-render` — M37 — is retired at M38.)* These extend the format additively; the frozen-v1 *guarantee* is per-doctype-hash, so adding a key that a frozen doctype does not use leaves its hash intact (only `changelog`, which adopts `placement` + `display-title` at M38, re-hashes to v2 via a corpus migration).

**The schema-version stamp (M34).** Every persisted instance carries a front-matter field recording the schema version it was authored against — **declared in each doctype's schema** (engine-injected uniformly across all doctypes), with an **engine-supplied value** (a new "current active schema version" deriver, the `status`/`date` model: in-schema field, engine-set value). It is what lets the store-scope conformance detector and the migration transform tell a *known-old-version* doc (migrate it) from a *genuinely-corrupt* one. Because the field is *in-schema*, adding it is a genuine `added-field` schema-diff the transform classifies — so stamping the corpus is a real dogfood of M34's add-field transform, not a synthetic one. For the header-less doctypes (`spec`/`prd`/`changelog`, which render no `---` block today) the stamp **introduces a front-matter block** — a real v0→v1 shape change. Full shape in [corpus-migration.md](corpus-migration.md).

## In-prose mentions (settled M33)

A **managed mention** is a cross-reference to a managed doc embedded in `slot` prose — lighter than a `field`-ref (which is a structured, typed leaf). Settled at M33 (the freeze completes the validation surface; [DECISIONS.md](../DECISIONS.md) → 2026-06-24 in-prose mentions):

- **Syntax — `#<type>:<slug>`, `<type>` a known managed doctype.** A managed mention is a slot-prose token in the canonical managed-address form (`#adr:csrf-strict-origin`). The `:`-plus-known-doctype is the **deterministic discriminator**: a bare `#token` with no `<type>:` (the original `#issue-42` example — an *external*, unmanaged reference) is **never a managed mention** and is never flagged, so external issue/PR mentions in prose are not noise. The CLI scans prose for the pattern; the LLM still owns the prose (the determinism boundary holds — the CLI checks references, never authors).
- **Resolution — the `committed_reachable` rule.** A mention resolves iff `<type>:<slug>` names a committed managed doc (`<location>/<slug>.md` exists) — the same rule the `ref-resolves` families use. *Dangling* = no such committed doc (the renamed/deleted-doc case). Doc-level only in v1 (section-anchor mentions — `#<type>:<slug>#<section>` — deferred).
- **Severity + scope — advisory, store-scope only.** The check is `schema-conformance.mention-resolves` (a sibling of `ref-resolves`), **default severity advisory** (a cascade knob), surfaced by the **store-scope `jigc validate` sweep** (the cross-doc backstop — a mention dangles when *another* doc is renamed/deleted, so store scope is its home), **never a per-task finalize gate** — this is what "lighter than `field`-refs" means (field-refs block at finalize; mentions advise at store scope). See [validation.md](validation.md) → the mention-resolves check.

## Open questions

Skeleton-level open questions (multi-level repetition, minting mechanics) are tracked in [structural-grammar.md](structural-grammar.md#open-questions).
