# Structural grammar

The shared skeleton beneath every managed artifact. There is **one structural engine**; **document** and **workflow** are two *dialects* over it, sharing all structure and differing only in their leaf kinds and a few unit/type-level annotations. This document specifies the dialect-neutral skeleton. The document dialect is specified in [document-type-schema.md](document-type-schema.md); the workflow dialect in its own part-doc (not yet written).

For the *why* behind each choice see [DECISIONS.md](../DECISIONS.md); for the framing, [VISION.md](../VISION.md) → The determinism boundary and principles #2/#3/#5/#6. Notation below is **illustrative** — the on-disk serialization format is an open question (see [VISION.md](../VISION.md) → Open questions).

## Skeleton vs dialects

| | **shared skeleton** | **document dialect** | **workflow dialect** |
|---|---|---|---|
| unit | ID'd, ordered, block-bodied, `repeatable` | section | step |
| leaves | *(extension point)* | `slot`, `field` | `placeholder`, `instruction` |
| type-level extra | *(extension point)* | `relation` (edges) | `fan-out`/`join` marker |
| addressing | `type:name#unit/item/leaf` | same | same |
| ID minting | types/units/leaves author-named; instances/items slug-minted | same | same |
| composition | `include` by ID | same | same |
| override | scalar · by-ID structural · leaf-fill · tracked-fork | same | same |
| validation | one engine, addressable targets | doc↔code, relation integrity | workflow↔references |

The payoff: **one addressing scheme, one override system, one composer, one minting rule — written once, serving both.** The dialects **never share a leaf kind**, so a document's `slot` (LLM-filled, write-path) and a workflow's `placeholder` (CLI-filled, read-path) stay strict opposites, exactly as VISION requires.

Scope discipline: **two built-in dialects, full stop.** The "extension point" (leaf kinds and unit/type annotations) is internal design discipline, not a public plugin API ([VISION.md](../VISION.md) non-goal: the engine/pack boundary is not a public surface yet).

## Units, blocks, leaves

The tree shape, dialect-neutral:

```
container            (document | workflow)
└─ unit              (section | step) — ordered, ID'd
   └─ block          (ordered set of leaves)
      └─ leaf        (dialect-defined kinds)
   └─ block, block…  (only when the unit is `repeatable`; each block = one minted-ID item)
```

- **Leaf** — the atom: an addressable, author-named hole. Its *kinds* are dialect-defined; the skeleton requires only that every dialect's leaf kinds divide into **CLI-adjudicated** and **agent-authored**, and that no kind is shared across dialects.
- **Block** — an ordered set of leaves. *A block contains leaves only — never a unit.* This is the depth cap.
- **Unit** — an ID'd, ordered node. Its body is **either** one block (a *simple* unit) **or** a list of blocks (a *repeatable* unit; see [Repetition](#repetition)).
- **Container** — an ordered list of units. Ordering lives in a separate ordered list of IDs, never in positions ([VISION.md](../VISION.md) principle #2).

Why shallow (not flat, not recursive): flat denies repeated items the individual IDs that validation must address; arbitrary recursion is a CMS smell that fights the "small footprint, one purpose" model. A thing that seems to want a deep tree is the signal to split it and cross-reference instead.

## Repetition

A unit body is **either** one block (simple) **or** a list of blocks (`repeatable`) — the *only* place repetition lives. Each block in the list is an **item** with a minted, stable ID; ordering is a separate list. **One bounded level:** no repeatable-inside-repeatable, and a block still holds leaves only.

A repeatable unit must designate one leaf as its **id-source** for slugging (see [Minting](#ids-provenance-and-minting)). It must be an *adjudicable* leaf — in the document dialect, a `field` — never agent-authored prose, because the CLI needs a short, clean value to slug.

## Addressing

Every addressable unit has a URI-shaped address — *which resource*, then *where inside it*:

```
type:name # unit / item / leaf
└─ ref ─┘ ↑ └──  fragment  ──┘
          location-within
```

- **Before `#` — the container** (the resource): `type:name`, type-prefixed so a reference is self-describing *and* type-checkable from the id alone. The `item` hop appears only inside a repeatable unit.
- **After `#` — the fragment**: `/`-separated `unit / item / leaf`.

The common case — referencing a whole container — is just `type:name`, a clean quotable atom with no fragment. This address is the single object every downstream system targets: a write command, a validation target, an override op, a cross-reference endpoint.

## IDs: provenance and minting

| Hop | Source |
|---|---|
| **type**, **unit**, **leaf** | author-named in the schema — fixed, never minted |
| container **instance**, repeatable **item** | minted by the CLI at creation |

Runtime minting happens at **exactly two sites**: creating a container, and adding an item to a repeatable unit. This tightly scopes the only hard part (minting under concurrency).

Minted IDs are **frozen content-slugs**:

- slugged from the designated id-source,
- **minted once and frozen** at creation — the id outlives a later rename of its source,
- never ordinal-looking (an ordinal is a position-smell; ids must not imply order),
- under concurrency, colliding slugs get a **deterministic suffix** applied in task-id merge order, so ids stay stable and reproducible across a `fan-out`/`join` ([VISION.md](../VISION.md) → Parallelism).

## Composition

A unit or block may be pulled by ID via `include` — the reuse primitive shared between document structure and workflow composition.

## Override

The override ladder ([VISION.md](../VISION.md) principle #5) operates on skeleton units: scalar override · by-ID structural ops (`insert --after`, `replace`, `remove`) · leaf-fill · tracked-fork. Because there is one unit model, "the write vocabulary shares the override system's unit model" holds by construction. The ladder's mechanics (delta recording, upgrade reconciliation) live in VISION and the override part-doc, not here.

## Validation

One validation engine pointed at addressable targets. The framework is dialect-neutral; the **probes** split engine-native vs pack-provided ([VISION.md](../VISION.md) principle #6):

- **engine-native** — `workflow ↔ references`, `file ↔ CLI-state`; need no pack content;
- **dialect/pack** — e.g. the document dialect's `doc ↔ code`; ship in a pack.

Integrity holds at the **`finalize` boundary**, not on every write (else bootstrapping deadlocks).

## Open questions

- **Multi-level repetition** — deferred unless a real type forces a second repeatable level.
- **Minting mechanics** — exact slug normalization (case/charset) and collision-suffix form.
