# Document-type definition schema

The **document dialect** of the [structural grammar](structural-grammar.md). The skeleton — units, blocks, leaves, addressing, minting, composition, override, and the validation framework — is dialect-neutral and specified there; **read it first.** This document defines what is specific to documents: the leaf kinds (`slot`, `field`), the determinism boundary as it falls inside a document, the `relation` construct for cross-references, and the document-specific validation probes.

This specifies the **schema** — the vocabulary *and* its on-disk form. The actual ADR / SPEC / PRD / commit definitions are development-pack content authored *in* this vocabulary and shipped with the pack; their file format is [On-disk definition format](#on-disk-definition-format) below. Notation is **illustrative**; for the *why*, see [DECISIONS.md](../DECISIONS.md); for the framing, [VISION.md](../VISION.md) → Document model.

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

**Addressing.** A section's single unnamed slot is addressed by the unit's address (`#unit`) — there is no named leaf below it in the schema. Multi-slot sections render each slot under a schema-fixed sub-label, and each sub-label is the slot's leaf id (`#unit/sub-label`). The address grammar permits both depths uniformly ([structural-grammar.md](structural-grammar.md#addressing)).

The CLI's only checks on a slot are presence and the integrity of any references embedded in its prose (a lighter "mention" check, distinct from `field`-refs — see [Open questions](#open-questions)).

### Field

A CLI-adjudicated typed value. Field **types** split along the engine/pack seam, mirroring the validation-probe split and keeping the engine empty of domain content:

- **Engine-native:** `enum`, `string` (constrained: maxlen / pattern — also the id-source for slugging), `date`, `bool`, `int`, `ref` (a reference to a managed document or fragment; carries relation metadata — see [Cross-references](#cross-references--ref-fields-with-relation-metadata)).
- **Pack-provided:** domain types such as `code-anchor` (points at a module / symbol / test). Their **adjudicator is pack-supplied** — "does this symbol exist? does this test cover this criterion?" is exactly the development pack's `doc ↔ code` probe. The meta-schema permits field types whose adjudicator ships in a pack.

Field **provenance** (who supplies the value) is partly implied by type:

- **CLI-derived** — `date` (now), the minted id, the slug. The CLI computes it; no one else touches it.
- **Controlled-choice** — `enum`; the LLM or human picks, the CLI validates membership.
- **Referential** — `ref` / `code-anchor`; proposed by LLM or human, integrity-validated by the CLI or a pack probe.
- **Free-but-typed** — a constrained `string`; written by LLM or human, shape-validated by the CLI.

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
- **Inverse-cardinality and orphan obligations** ("a PRD must have ≥ 1 SPEC") are **completeness, not integrity** — they depend on *other* tasks (the SPEC is a later task's job), so they are **never** a per-task `finalize` gate: advisory by default, hard-enforced only at store/milestone scope ([validation.md](validation.md)). Forward-ref integrity *is* gated at `finalize`, because the task can satisfy it against the **committed store + the same task's working area** — see [validation.md](validation.md) → Forward-ref resolution for the policy on cross-task forward-refs (not supported in MVP).
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
      - { id: supersedes, type: ref, to: adr, card: "0..1", inverse: superseded-by }
  - id: context
    slot: { hint: "Why a decision was needed — the forces at play." }
  - id: decision
    slot: { hint: "What we decided, in a sentence or two." }
  - id: consequences
    slot: { hint: "Tradeoffs and follow-on effects." }
```

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

Two derived conveniences, no extra source:

- **`--template` view** — the engine renders the *blank instance* a schema produces (headings + marked slots/fields + relation notes) on demand, like the generated mermaid flow, so "what does this produce" is legible without the source being a template.
- **Validation needs no separate declaration** — it falls out of the typed leaves + relations + the cascade: a `code-anchor` field means `doc-code` applies, a relation's `card` is enforced, severities are cascade knobs ([validation.md](validation.md)).

**Overrides** target sections/leaves by ID within the file (`adr#status`); a schema-structural delta's fragment is a small YAML section declaration — the config-family counterpart to a workflow's step-file fragment ([overrides.md](overrides.md)). Notation above is illustrative — the keys are placeholders pending implementation (`id-from` is the YAML spelling of the prose *id-source*; plus `card`, `set`, `of`, `header`, `to`, `inverse`, `inverse-card`).

## Open questions

- **Inline references in prose** — "mentions" embedded in a `slot` (e.g. `#issue-42` in commit-body prose) are a lighter validation concern than `field`-refs; not yet designed.

Skeleton-level open questions (multi-level repetition, minting mechanics) are tracked in [structural-grammar.md](structural-grammar.md#open-questions).
