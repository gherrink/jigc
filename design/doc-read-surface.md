# The doc read surface — `jigc doc show` + the pinned `--format json` contract

**M39 design of record.** Where the *committed-doc read* is pinned. `jigc doc show <ref>` is the one verb that reads a **filled**, committed managed doc (or an addressed slice of one) back out of the store. It is the read-path counterpart to the write verbs (`jigc doc set`/`add-item`/…) and the sibling of the two other read surfaces — [introspection.md](introspection.md) (`jigc describe`, the *menu* of what can be composed, deliberately excluding filled prose) and [validation.md](validation.md) (`jigc validate`, the *check*). Homed here (settled 2026-07-06, M39 Settle — [DECISIONS.md](../DECISIONS.md)) rather than folded into [write-commands.md](write-commands.md) (the write side) or [introspection.md](introspection.md) (which by charter excludes filled-prose reads), because the read-back of authored content is a distinct concern with a **1.0 stable machine contract** that neither of those docs owns.

Reads after the surfaces it consumes — the address grammar ([storage.md](storage.md) → Identity/addressing), the schema ([document-type-schema.md](document-type-schema.md)), and the byte-exact slice path ([storage.md](storage.md)) — alongside the other read-path/integration surfaces. Its conformance witness (the machine-consumed record whose shape pins the contract) is the **milestone record** ([team-ready-state.md](team-ready-state.md) → The read surface).

## What it reads — a committed doc, not task WIP

`jigc doc show` is **task-less**: it resolves the cascade schema set + repo root exactly as `jigc validate` does, then serves the read through the canonical parse path. It reads the **committed** store — not a task's in-flight working copy — so it is the surface a fresh agent session (or a teammate on a clone) uses to see current authored state. A doc **staged in a task** is read via `jigc task diff` instead; the `store.not-found` route says so (M40 — it previously routed "create the referenced doc", actively misleading for a staged-but-unfinalized doc). A read-side block (unknown type / a transient doctype / not-found / an unparseable committed doc / a `#fragment` naming nothing) routes through the same non-zero-exit + route envelope a write block uses.

## The slice grammar — whole-doc, `#section`, item, leaf

The `<ref>` is a doc address, optionally carrying a `#fragment` that names a sub-node. Six resolution depths (the two nested depths join the contract at M40 — nested repeatables, below), each serving the corresponding node:

| Address | Resolves to |
|---|---|
| `type:slug` (whole-doc) | the whole document |
| `type:slug#section` | one section — a **slot** section's prose, or a **repeatable** section's item array |
| `type:slug#section/<item>` | one repeatable **item** object |
| `type:slug#section/<item>/<leaf>` | one **leaf** — a slot's prose, a field's value, or the `id-from` leaf (the item's heading) |
| `type:slug#section/<id>/<block>/<gid>` | one **nested item** object (M40 — a nested repeatable's member) |
| `type:slug#section/<id>/<block>/<gid>/<leaf>` | one **nested leaf** (M40) |

## Plain text vs. `--format json`

- **Plain text** (default / `--format agent` / `--format human`) is the **byte-exact** slice — the store's own bytes for that node, verbatim, so a human or agent sees exactly what is committed.
- **`--format json`** is the **pinned 1.0 stable contract** below — a machine-legible projection of the parsed structure (a one-way door, pinned now rather than left to the first implementation).

The two are complementary: byte-exactness is the plain path's job; the json path trades it for a clean, keyed, deserializable shape.

## The pinned `--format json` contract (1.0)

Pinned once here; every doctype's json read conforms. The **milestone record is the conformance witness** — the first doctype whose *primary* consumer is a machine read (fresh-clone continuation), so its shape is the pin's proof.

**Whole-doc** → an object with exactly four top-level keys, `{ type, slug, fields, sections }`:

```json
{ "type": <doctype>, "slug": <slug>, "fields": { … }, "sections": { … } }
```

- **`type`** — the doctype id. **`slug`** — the addressed slug.
- **`fields`** — every *simple* section's leaves flattened, keyed by leaf id (the header front-matter + any body field group). A **scalar** field serializes as its string; a **list**-cardinality field as a json array of strings; an **enum** leaf as its **lowercase string** (the enum member is already its canonical lowercase form). The one **compound** field in the pinned surface is the milestone-record's `base` pin: it projects as a structured object `{ "sha": …, "short": … }` (clean lossless access to both SHAs; the committed `.md` stores the space-joined `<sha> <short>` scalar, so this is json-projection-only — [team-ready-state.md](team-ready-state.md) → The read surface, [DECISIONS.md](../DECISIONS.md) → 2026-07-07). Every other field stays a scalar.
- **`sections`** — one entry per **slot** section (its **trimmed prose string** — the clean machine value; leading/trailing whitespace stripped) and per **repeatable** section (its **item array**). A header/fields-only simple section contributes to `fields` alone and has no `sections` entry. An unfilled/absent slot is the empty string.

**A `#fragment` slice** returns the corresponding sub-node of the whole-doc shape:

- a **slot section** → its trimmed prose string;
- a **repeatable section** (`#section`) → its **item array**;
- an **item** (`#section/<item>`) → its item object;
- a **leaf** (`#section/<item>/<leaf>`) → the leaf value (a slot's trimmed prose string, a field value shaped as in `fields`, or the block's `id-from` leaf → the item's heading string).

**An item object** carries its `id-from` leaf keyed by the block's declared `id-from` → the item's heading (its stable id-source), each other field keyed by leaf id (scalar → string, list → array), and each slot keyed by leaf id → its trimmed prose.

**Nested repeatables join the pin (M40 — supersedes the M39 exclusion).** Nested `Leaf::Repeatable` groups (the M22 changelog nested shape) **join the pinned item object**: each nested repeatable keys the item object by its declared block id → an **array of recursive item objects** — e.g. a changelog release item: `{"title":"1.0.0","date":"…","changes":[{"category":"added","notes":"…"}]}`. The **section-qualified write address is accepted on show**: `#releases/<id>/changes` → the nested array, `…/changes/<gid>` → the nested item object, `…/<gid>/<leaf>` → the leaf. Two undeclared defects die with the pin: the false `store.no-such-item` on the canonical write address, and the segment-less **wrong-node degrade** (a deeper path silently returning the enclosing item, exit 0) — the rule is *honest error or correct answer, never wrong-node-exit-0*. The **plain** (markdown) item/section slice must likewise carry the item's fields + nested content — the byte-exactness claim above currently **fails** for a field-carrying item (fields + nested groups are dropped; only the whole-doc plain read was complete), a recorded defect the rc.4 wave fixes ([DECISIONS.md](../DECISIONS.md) → 2026-07-10 M40 Settle).

**Evolution posture (declared — previously undeclared).** Additive keys are permitted **pre-1.0 only**; from the 1.0 pin, the shape evolves only by an **explicitly versioned extension** — never a silent additive key. Two additive keys land in the rc.4 wave under the pre-1.0 rule: the milestone-record's `fields` gains `schema-version` (the methodology stamp — [corpus-migration.md](corpus-migration.md) → The freeze-exempt sibling), and nested repeatable groups join the item object (above).

## Why json is a contract here (and describe's is the opposite)

`jigc describe`'s output is deliberately **non-contractual** — format-hostile to parsing so nothing depends on it ([introspection.md](introspection.md) → Non-contractual by design). `jigc doc show --format json` is the deliberate inverse: its whole purpose is machine consumption (fresh-clone continuation re-derives milestone state from it), so it **is** a stable contract, pinned at 1.0. The two read surfaces sit on opposite sides of the same axis on purpose: describe is a menu you orient by, `doc show` is content you compute over. **A third surface joins at M40**: `jigc doc schema <doctype> --format json` — the structural projection that *cannot* ride describe (its format predicate forbids parseable structure) — ships as a **separately-pinned, explicitly versioned contract**, pinned here:

```json
{ "contract-version": 2, "type": <doctype>, "schema-version": <the doctype's stamped version, else null>,
  "fields":   [ { "id", "type", "of"?, "required", "author-required", "default"?, "set"?, "section"? }, … ],
  "sections": [ { "id", "kind": "slot"|"repeatable", "optional"?, "item": { "fields", "slots", "nested" } }, … ] }
```

— the `item` object is **recursive** for nested repeatables (`nested` carries the inner blocks); a field's `of` carries an `enum`'s legal members (universal across depths — an agent reads the legal values from the contract, no failed-write probe), and `section` names the owning simple-section id (**top-level fields only** — an item field carries its section structurally, under `sections[].item`); `contract-version` bumps on any structural change to this projection — **no additive carve-out** (the M41 rc.5 addition of the `of`/`section` keys bumped it **1→2**) — the *values* track the schemas as they evolve, the *keys/structure* are the pin; golden-pinned at ship ([introspection.md](introspection.md)).
