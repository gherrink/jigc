# Structural grammar

The shared skeleton beneath every managed artifact. There is **one structural engine**; **document** and **workflow** are two *dialects* over it, sharing all structure and differing only in their leaf kinds and a few unit/type-level annotations. This document specifies the dialect-neutral skeleton. The document dialect is specified in [document-type-schema.md](document-type-schema.md); the workflow dialect in [workflow-dialect.md](workflow-dialect.md).

For the *why* behind each choice see [DECISIONS.md](../DECISIONS.md); for the framing, [VISION.md](../VISION.md) → The determinism boundary and principles #2/#3/#5/#6. Notation below is **illustrative** ([what that disclaims](../CLAUDE.md#how-we-work-together)) — the on-disk serialization format is specified in [storage.md](storage.md) (and the doc-type definition format in [document-type-schema.md](document-type-schema.md)).

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

A unit body is **either** one block (simple) **or** a list of blocks (`repeatable`) — the *only* place repetition lives. Each block in the list is an **item** with a minted, stable ID; ordering is a separate list. **Nesting is bounded by depth, not by one level:** a repeatable may itself nest a repeatable (the schema model's `Leaf::Repeatable`, M22), recursively, capped at **2 nesting levels** — and that number is *derived*, never declared: a leaf write at nesting depth `D` costs `2D + 1` address hops (the address chain alternates an item id and the nested-section id declaring the next block), so the deepest addressable depth is `(MAX_FRAGMENT_HOPS − 1) / 2` — `engine::schema`'s `MAX_NESTING_DEPTH`, computed from `engine::address`'s `MAX_FRAGMENT_HOPS`. A level past it is one whose own leaf addresses the address grammar rejects, so declaring it is rejected at load (a documented cap, not silent truncation) rather than advertised as a write nothing could take. The **heading grammar is the other, now non-binding ceiling**: items render at heading level `2 + nesting-depth` (section `##`, first-level items `###`, nested `####`), and Markdown has no seventh heading level — but the address hop budget runs out first, which is why the cap is stated as one number derived from that budget and from nothing else (M49; `DECISIONS.md` → 2026-08-29). **Breadth per block is capped at one:** a block may declare at most **one** nested repeatable — the parsed item carries a single undifferentiated nested item list (heading depth alone cannot attribute an item to one of two sibling nested sections), so a second sibling group is unrepresentable and is likewise rejected loudly at schema load, naming the block and both nested ids. A block still holds leaves only — and a nested repeatable *is* a leaf.

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

**Resolution is minting's other half — an id arriving from outside is checked against the same grammar before it is anything else.** Work-unit ids are the one identity a caller *hands* jigc: every `jigc task <verb> <id>`, every `--task <id>`, every `jigc milestone <verb> <id>`. And `.jigc/tasks/<id>/` and `.jigc/milestones/<id>/` are built by **joining that token onto a path**, so it becomes a path *component* before any door has asked whether it is an id at all. The rule is therefore symmetric with the mint: a token that is not a well-formed slug is not a *wrong* id, it is **not an id**, and it is refused at the **resolve seam** — below clap, above the filesystem op — with the blocking finding `work-unit.malformed-id` whose route states the grammar: *use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)*. Not a clap `value_parser`: that exits 2 carrying neither a code nor a route, which is the route floor breached at every door that takes an id ([surface-contract.md](surface-contract.md) → law 2 / the route floor). The predicate is the **recognition** rule every frozen id read back from disk is already checked against — not `slugify(x) == x`, which the mint-time word cap and edge-stopword drop would make reject ids the mint itself produced.

So every id-taking door gives **two** answers where it used to give one: **malformed** → the grammar, and **well-formed but unknown** → the roster (`jigc task list`), which remains the converged wrong-id route ([write-commands.md](write-commands.md) → Task-id collision & resume). The missing half was load-bearing rather than tidy: until **M50** `jigc task discard "../.."` resolved the *repository root* as a working area and removed it — `.git/` included — at **exit 0**, and `--task ""` reported a clean task that does not exist.

**Every by-id door gives a third, because *existing* is not the same as *being a work unit* (M53).** A task id resolves to a directory, and until M53 a directory *was* a task — so a bare `mkdir .jigc/tasks/anything` was a work unit at every by-id door, each answering a later question about a unit that does not exist (`jigc task validate` reported *validates clean* at exit **0**). What makes a directory a task, and why a pin-less one is a **residual**, is [storage.md](storage.md) → The per-task working area; what belongs here is the **answer**: a residual is the *same* absence as an unknown id, so it carries the same `finalize.no-task` at the same `task:<id>` key, and differs only where the recovery differs — a route naming the **path to clear by hand**, because no roster answers a leftover and jigc mints no verb that clears one. This third answer is **both families'**, each under its own identity — `finalize.no-task` at `task:<id>`, `milestone.unknown` at `milestone:<id>` — because the condition is the same one and only the noun changes. [**Corrected 2026-09-22 (M53 Increment 3, the fix):** this read *"This third answer is the **task** family's: a milestone id resolves from its committed record rather than from a working area, so a record-less `.jigc/milestones/<id>` is not a cell of it"*, and that premise is false at the by-id doors — it was the only stated warrant for building 17 of the 25 rows. Falsifying datum, driven on `afd76ea3`: a bare `mkdir .jigc/milestones/stray-mile` made `list-tasks` / `provision` / `execute` / `finalize` / `discard` answer the code-less `{"error": "could not read the task list for milestone … (os error 2)"}`, `join` and `add-task` a **false** `milestone.area-io` (*"a disk or permissions problem"*), and `add-from-spec` answer about the *spec* — while the same id with **no** directory answered the keyed, routed `(milestone.unknown, milestone:<id>)` at all eight. What **is** record-shaped is the converse, and it is the rule that separates the two states: a pin-less area a committed record still names is a **cache the fresh-clone re-seed rebuilds**, never a leftover.]

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

- ~~**Multi-level repetition**~~ — **landed (M22).** A repeatable may nest a repeatable, recursively, capped at 2 nesting levels — the **address hop budget** binds, not the heading grammar (M49); see [Repetition](#repetition) for the derivation. The `changelog`'s `release → change-group` shape is the first consumer ([design/changelog.md](changelog.md) → engine work #1).
- ~~**Minting mechanics**~~ — **slug normalization landed (M39).** The mint now cuts at **word boundaries** with a **~5-word cap** (from the RC greenfield trial's unreadable mid-word truncations, [ideas/slug-minting-ergonomics.md](../ideas/slug-minting-ergonomics.md)), and mint-time **`--slug` overrides** are accepted on `jigc start` + `jigc doc create` (the human/agent proposes identity; the CLI still validates the grammar + owns collision handling — [write-commands.md](write-commands.md) → Task origination). This is a pure *minting* change inside the CLI's authority: existing committed IDs never move (that stays `jigc rename`'s job) and the stable-ID invariant is untouched. **A version-gate *does* ride on the rule itself (M42).** It is not doctype schema *shape*, so it sits in no `schema-hash` — which is exactly the hole: the pack-load freeze assert that blocks renaming a *field* was blind to a change in the function that *names every id in every corpus*, and no transform kind can re-mint an id, so such a change cannot be migrated after the fact. The rule is therefore itself declared and asserted — a `slug-rule:` version + behaviour fingerprint pinned beside each pack's schema manifest, blocking loudly at pack-load on an undeclared change ([storage.md](storage.md) → Identity → *The slug rule is itself a versioned rule (M42)*). The **collision-suffix form is settled for M7**: a numeric `-2`/`-3`/… suffix applied **in task-id order at the `join`** (lower task-id keeps the bare slug), with the renamed instance's intra-document self-references rewritten in lockstep ([storage.md](storage.md#the-by-task-id-join-m7)).
