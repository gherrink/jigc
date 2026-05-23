# Document-type definition schema

The **document dialect** of the [structural grammar](structural-grammar.md). The skeleton — units, blocks, leaves, addressing, minting, composition, override, and the validation framework — is dialect-neutral and specified there; **read it first.** This document defines what is specific to documents: the leaf kinds (`slot`, `field`), the determinism boundary as it falls inside a document, the `relation` construct for cross-references, and the document-specific validation probes.

This specifies the **schema**, not the pack content. The actual ADR / SPEC / PRD / commit definitions are development-pack content authored *in* this vocabulary and live elsewhere. Notation is **illustrative**; for the *why*, see [DECISIONS.md](../DECISIONS.md); for the framing, [VISION.md](../VISION.md) → Document model.

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

The CLI's only checks on a slot are presence and the integrity of any references embedded in its prose (a lighter "mention" check, distinct from `field`-refs — see [Open questions](#open-questions)).

### Field

A CLI-adjudicated typed value. Field **types** split along the engine/pack seam, mirroring the validation-probe split and keeping the engine empty of domain content:

- **Engine-native:** `enum`, `string` (constrained: maxlen / pattern — also the id-source for slugging), `date`, `bool`, `int`, `ref` (a reference to a managed document or fragment).
- **Pack-provided:** domain types such as `code-anchor` (points at a module / symbol / test). Their **adjudicator is pack-supplied** — "does this symbol exist? does this test cover this criterion?" is exactly the development pack's `doc ↔ code` probe. The meta-schema permits field types whose adjudicator ships in a pack.

Field **provenance** (who supplies the value) is partly implied by type:

- **CLI-derived** — `date` (now), the minted id, the slug. The CLI computes it; no one else touches it.
- **Controlled-choice** — `enum`; the LLM or human picks, the CLI validates membership.
- **Referential** — `ref` / `code-anchor`; proposed by LLM or human, integrity-validated by the CLI or a pack probe.
- **Free-but-typed** — a constrained `string`; written by LLM or human, shape-validated by the CLI.

## Sections and repetition

A document is an ordered list of **sections** (skeleton units). A section is **simple** (one block of leaves) or **`repeatable`** (a list of blocks; see [structural-grammar.md](structural-grammar.md#repetition) for the mechanism). In the document dialect a repeatable section's designated **id-source must be a `field`** — never a slot — so the CLI has a short, adjudicable value to slug.

The same construct scales both ways:

- down — an item is a single `slot` → an ID'd list of prose bullets;
- up — an item is `{ statement: slot, maps-to-test: field }` → a record (e.g. a SPEC criterion).

Consequence, accepted deliberately: a SPEC's criteria live *inside* the SPEC as records, addressed like `spec:auth-flow#criteria/rate-limit/statement` — not as separate child documents. The SPEC stays one cohesive, reviewable artifact; **"small footprint" therefore means one purpose, not few lines.**

## Cross-references — the `relation` construct

A cross-reference is **two facets, not two choices** — the ORM pattern:

- the **field** is the instance endpoint — it lives in a section, has an address, and is placed like any leaf;
- the **relation** is the type-level constraint that governs it.

A relation declares:

```
source-type · forward-name + cardinality · target-type · inverse-name + inverse-cardinality
e.g.   spec  ·  derived-from  (exactly 1)  ·   prd      ·   has-specs   (≥ 1)
```

This makes "normalized database, cross-reference instead of duplicate" real: relations are the schema's foreign-key edges, and validation walks them.

### Bidirectional, but the inverse is derived — never stored

Cross-references are bidirectional at the **graph/view** level and single-authored at the **storage** level:

- the **forward ref is authored once** (the SPEC stores `derived-from: prd:billing`, placed via the CLI, with a home in a section);
- the **inverse is derived by the CLI** — "PRD `has-specs` […]" is computed by walking refs and **injected into the PRD's read view**, never written into the PRD's file.

This is the *reliable* form of bidirectionality, forced by three locked invariants:

- **Single source of truth** — the back-edge cannot drift from the forward edge because it *is* the forward edge read backwards. The graph is whole by construction; no asymmetric period, no backfill migration.
- **Diff-friendly storage** — a stored back-edge would churn the target's file every time a new referrer appeared. Deriving keeps it stable in diffs.
- **Concurrency** — under `fan-out`, many sub-agents referencing the same target would all collide on the target *file* if the back-edge lived there. Derived = zero contention.

The determinism boundary holds: the forward ref is placed by the agent-via-CLI; the reverse edge is derived structure the CLI owns. Two riders:

- The CLI keeps a **rebuildable edge index** — a derived cache (source of truth stays the documents) so a read needn't rescan the store; it also makes validation cheap (walk the index).
- **Inverse-cardinality and orphan obligations** ("a PRD must have ≥ 1 SPEC") are checked at the **`finalize` boundary**, not per-write — otherwise a PRD could never be created before its first SPEC exists.

## How this keys the rest

The trichotomy and addressing are the binding surface for systems designed in their own part-docs:

- **Write-command vocabulary** — `set-slot`, `set-field`, `add-item`, and the structural ops each target an address; the trichotomy *generates* the verb set.
- **Override ladder** — its by-ID structural ops operate on sections, the document dialect's units (see [structural-grammar.md](structural-grammar.md#override)).
- **Validation** — one engine, addressable targets; the document dialect contributes the pack-provided `doc ↔ code` probe and relation-integrity checks (see [structural-grammar.md](structural-grammar.md#validation)).

## Open questions

- **Inline references in prose** — "mentions" embedded in a `slot` (e.g. `#issue-42` in commit-body prose) are a lighter validation concern than `field`-refs; not yet designed.

Skeleton-level open questions (serialization, the workflow dialect, multi-level repetition, minting mechanics) are tracked in [structural-grammar.md](structural-grammar.md#open-questions).
