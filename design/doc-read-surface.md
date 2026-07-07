# The doc read surface — `jigc doc show` + the pinned `--format json` contract

**M39 design of record.** Where the *committed-doc read* is pinned. `jigc doc show <ref>` is the one verb that reads a **filled**, committed managed doc (or an addressed slice of one) back out of the store. It is the read-path counterpart to the write verbs (`jigc doc set`/`add-item`/…) and the sibling of the two other read surfaces — [introspection.md](introspection.md) (`jigc describe`, the *menu* of what can be composed, deliberately excluding filled prose) and [validation.md](validation.md) (`jigc validate`, the *check*). Homed here (settled 2026-07-06, M39 Settle — [DECISIONS.md](../DECISIONS.md)) rather than folded into [write-commands.md](write-commands.md) (the write side) or [introspection.md](introspection.md) (which by charter excludes filled-prose reads), because the read-back of authored content is a distinct concern with a **1.0 stable machine contract** that neither of those docs owns.

Reads after the surfaces it consumes — the address grammar ([storage.md](storage.md) → Identity/addressing), the schema ([document-type-schema.md](document-type-schema.md)), and the byte-exact slice path ([storage.md](storage.md)) — alongside the other read-path/integration surfaces. Its conformance witness (the machine-consumed record whose shape pins the contract) is the **milestone record** ([team-ready-state.md](team-ready-state.md) → The read surface).

## What it reads — a committed doc, not task WIP

`jigc doc show` is **task-less**: it resolves the cascade schema set + repo root exactly as `jigc validate` does, then serves the read through the canonical parse path. It reads the **committed** store — not a task's in-flight working copy — so it is the surface a fresh agent session (or a teammate on a clone) uses to see current authored state. A read-side block (unknown type / a transient doctype / not-found / an unparseable committed doc / a `#fragment` naming nothing) routes through the same non-zero-exit + route envelope a write block uses.

## The slice grammar — whole-doc, `#section`, item, leaf

The `<ref>` is a doc address, optionally carrying a `#fragment` that names a sub-node. Four resolution depths, each serving the corresponding node:

| Address | Resolves to |
|---|---|
| `type:slug` (whole-doc) | the whole document |
| `type:slug#section` | one section — a **slot** section's prose, or a **repeatable** section's item array |
| `type:slug#section/<item>` | one repeatable **item** object |
| `type:slug#section/<item>/<leaf>` | one **leaf** — a slot's prose, a field's value, or the `id-from` leaf (the item's heading) |

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

**Out of the pinned contract:** nested repeatable-in-item content (the M22 changelog nested shape) is **not** modelled — every witness plus the milestone record is flat, so a deeper path degrades to the enclosing item object rather than pinning an unpinned nested json shape.

## Why json is a contract here (and describe's is the opposite)

`jigc describe`'s output is deliberately **non-contractual** — format-hostile to parsing so nothing depends on it ([introspection.md](introspection.md) → Non-contractual by design). `jigc doc show --format json` is the deliberate inverse: its whole purpose is machine consumption (fresh-clone continuation re-derives milestone state from it), so it **is** a stable contract, pinned at 1.0. The two read surfaces sit on opposite sides of the same axis on purpose: describe is a menu you orient by, `doc show` is content you compute over.
