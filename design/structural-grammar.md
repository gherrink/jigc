# Structural grammar

The shared skeleton beneath every managed artifact. There is **one structural engine**; **document** and **workflow** are two *dialects* over it, sharing all structure and differing only in their leaf kinds and a few unit/type-level annotations. This document specifies the dialect-neutral skeleton. The document dialect is specified in [document-type-schema.md](document-type-schema.md); the workflow dialect in [workflow-dialect.md](workflow-dialect.md).

For the *why* behind each choice see [DECISIONS.md](../DECISIONS.md); for the framing, [VISION.md](../VISION.md) → The determinism boundary and principles #2/#3/#5/#6. Notation below is **illustrative** — the on-disk serialization format is specified in [storage.md](storage.md) (and the doc-type definition format in [document-type-schema.md](document-type-schema.md)).

## Skeleton vs dialects

| | **shared skeleton** | **document dialect** | **workflow dialect** |
|---|---|---|---|
| unit | ID'd, ordered, block-bodied, `repeatable` | section | step |
| leaves | *(extension point)* | `slot`, `field` | `placeholder`, `instruction` |
| type-level extra | *(extension point)* | `ref`-field relation metadata (`to`, `card`, `inverse`) | `fan-out`/`join` marker |
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
- **Container** — an ordered list of units. Ordering lives in a separate ordered list of IDs ([VISION.md](../VISION.md) principle #2). "Never positions" bans position-token IDs (`01-`, `02.5-` prefixes that encode order in the identifier); **physical-order serialization** of that list — where items carry their own non-positional IDs — is fine and is used uniformly (doc instances' repeatable items in [storage.md](storage.md); workflow bodies' include lines in [workflow-dialect.md](workflow-dialect.md)).

Why shallow (not flat, not recursive): flat denies repeated items the individual IDs that validation must address; arbitrary recursion is a CMS smell that fights the "small footprint, one purpose" model. A thing that seems to want a deep tree is the signal to split it and cross-reference instead.

## Repetition

A unit body is **either** one block (simple) **or** a list of blocks (`repeatable`) — the *only* place repetition lives. Each block in the list is an **item** with a minted, stable ID; ordering is a separate list. **Nesting is bounded by depth, not by one level:** a repeatable may itself nest a repeatable (the schema model's `Leaf::Repeatable`, M22), recursively, capped at **four nesting levels** — items render at heading level `2 + nesting-depth` (section `##`, first-level items `###`, nested `####`, …, `######`), so a fifth level (`H7`) is rejected at load (a documented cap, not silent truncation). A block still holds leaves only — and a nested repeatable *is* a leaf.

A repeatable unit must designate one leaf as its **id-source** for slugging (see [Minting](#ids-provenance-and-minting)). It must be an *adjudicable* leaf — in the document dialect, a `field` — never agent-authored prose, because the CLI needs a short, clean value to slug. *On-disk rendering is dialect-specific: in the document dialect, the id-source field is rendered as the item's `###` heading text (with `{#id}` carrying the frozen minted id), not as a trailing `- key: value` field — see [storage.md](storage.md) → Identity.*

## Addressing

Every addressable target has a URI-shaped address — *which resource*, then *where inside it*:

```
address  := ref ( "#" fragment )?
ref      := type ":" slug                 # the container (a doc or workflow)
fragment := unit ( "/" leaf )?            # in a non-repeatable unit
          | unit "/" item ( "/" leaf )?   # in a repeatable unit
```

**One principle: address at the depth of what you target.** Every path through the addressable tree is a valid address, pointing at whatever sits at that depth — a container, a unit (section/step), an item, or a leaf (field, slot, placeholder/instruction):

| Form | What it points at |
|---|---|
| `type:slug` | the whole container (a doc, a workflow) |
| `type:slug#unit` | a unit (section/step); when that unit declares a single unnamed slot, the unit's address *is* the slot's address |
| `type:slug#unit/leaf` | a named leaf inside a non-repeatable unit (a field, a sub-labelled slot) |
| `type:slug#unit/item` | a whole repeatable item (e.g. a SPEC criterion) |
| `type:slug#unit/item/leaf` | a named leaf inside a repeatable item |

The `item` hop appears **iff** the unit is repeatable; the `leaf` hop appears iff the schema names a leaf at that depth. So the schema *determines* which depths are valid for any given unit; the grammar *permits* every depth uniformly.

**One grammar, every reference.** This is the single address atom every downstream system targets: write commands (`jigc doc set-slot <addr>`), validation targets, override ops, data-values in workflows (`{{adr:foo#decision}}`), AND the **values of `ref` fields** that author cross-references (the `supersedes: adr:single-node-cache` pattern — see [document-type-schema.md](document-type-schema.md) → Cross-references). One grammar, one shape, one place to learn — used for every kind of reference, at whichever depth the reference needs.

## IDs: provenance and minting

| Hop | Source |
|---|---|
| **type**, **unit**, **leaf** | author-named in the schema — fixed, never minted |
| container **instance**, repeatable **item** | minted by the CLI at creation |

Runtime minting happens at **exactly two sites *in the managed-artifact family***: creating a container, and adding an item to a repeatable unit. This tightly scopes the only hard part (minting under concurrency). (The work-unit family — tasks, and planned milestone/increment — has its own mint sites; see [Work-units and runtime identity](#work-units-and-runtime-identity).)

Minted IDs are **content-slugs**:

- slugged from the designated id-source,
- **minted at creation and stable under reorder and ordinary retitle-without-reslug** — editing the id-source's displayed value (an item's `###` heading, a doc's H1) does *not* change the id; a deliberate **identity change** (re-slug) is allowed only through one explicit, atomic CLI op — `jigc rename` for managed docs ([write-commands.md](write-commands.md) → `jigc rename`; [VISION.md](../VISION.md) → Stable IDs) — never an untracked move, with every structured referrer repointed in lockstep. *(This managed-doc identity revision does **not** make workflow/step ids mutable — those are pack-definition filenames, out of scope.)*
- never ordinal-looking (an ordinal is a position-smell; ids must not imply order),
- under concurrency, colliding slugs get a **deterministic suffix** applied in task-id merge order, so ids stay stable and reproducible across a `fan-out`/`join` ([VISION.md](../VISION.md) → Parallelism).

## Work-units and runtime identity

Documents and workflows are the **managed-artifact family** — minted, structurally rich, dialect-specific. A second family — **work-units** — shares the minting discipline but has no internal section/leaf structure. Work-units are first-class minted identities that scope state, coordination, and validation:

- **task** (MVP) — the staging unit; one task → one `finalize` → one logical commit. Per-task working area, base pin, validation scope, fan-out join key.
- **milestone** (**M7**) — a higher-level work container; tasks roll up to a milestone for store-scope validation (completeness obligations like inverse-cardinality). M7 builds it as the **fan-out container**: its task list is what the by-task-id join enumerates ([storage.md](storage.md#the-by-task-id-join-m7)), populated by `jigc milestone add-task <intent>` and a spec-seed path ([write-commands.md](write-commands.md) → Minting a milestone). The `{{milestone.tasks}}` *data-value root* that exposes the same list to the read path lands in **M8** with its consumer, the `fan-out` step ([workflow-dialect.md](workflow-dialect.md) → data-value roots). For M7 the hierarchy is **`milestone > task`** directly — `increment` minting stays deferred.
- **increment** (planned) — sits between milestone and task: a **deliverable increment**, a coherent group of tasks that together deliver one vertical slice of a milestone's outcome. It is the unit `milestone-planning` decomposes a milestone into, and the rollup parent of its tasks; the full hierarchy is `milestone > increment > task`. (Whether the increment or the task is the one-level `fan-out` boundary is a post-MVP question, deferred with increment minting.)

All work-units use the same slug-from-source / frozen / collision-suffix discipline as artifacts; they don't have leaves to address into, so addresses are `type:name` with no fragment (`task:add-rate-limiter`, `milestone:m1`). They are referenced as live-state roots by data-values (`task.intent`, `milestone.tasks`), as command surfaces (`jigc task …`, future `jigc milestone …`), and as validation scopes.

**Minting sites — full picture:**

- managed-artifact family: exactly two sites (container creation, repeatable-item add) — see [Repetition](#repetition) and [IDs](#ids-provenance-and-minting).
- work-unit family: each work-unit `create`/`start` operation (`jigc start --workflow` mints a task; the **M7** `jigc milestone create` mints a milestone; future `jigc increment create`).

Both families' mints are deterministic under concurrency (task-id-ordered collision suffix), so reproducibility holds across the whole runtime, not just inside artifacts.

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

- ~~**Multi-level repetition**~~ — **landed (M22).** A repeatable may nest a repeatable, recursively, capped at four nesting levels (`H6`); see [Repetition](#repetition). The `changelog`'s `release → change-group` shape is the first consumer ([design/changelog.md](changelog.md) → engine work #1).
- ~~**Minting mechanics**~~ — **slug normalization landed (M39).** The mint now cuts at **word boundaries** with a **~5-word cap** (from the RC greenfield trial's unreadable mid-word truncations, [ideas/slug-minting-ergonomics.md](../ideas/slug-minting-ergonomics.md)), and mint-time **`--slug` overrides** are accepted on `jigc start` + `jigc doc create` (the human/agent proposes identity; the CLI still validates the grammar + owns collision handling — [write-commands.md](write-commands.md) → Task origination). This is a pure *minting* change inside the CLI's authority: existing committed IDs never move (that stays `jigc rename`'s job), the stable-ID invariant is untouched, and it is CLI behaviour not doctype schema shape, so no version-gate rides on it. The **collision-suffix form is settled for M7**: a numeric `-2`/`-3`/… suffix applied **in task-id order at the `join`** (lower task-id keeps the bare slug), with the renamed instance's intra-document self-references rewritten in lockstep ([storage.md](storage.md#the-by-task-id-join-m7)).
