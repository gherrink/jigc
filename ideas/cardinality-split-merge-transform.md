# cardinality split/merge — a deterministic transform between one file and many

**Status: parked 2026-10-02.** Parked by the M55 Settle, S1 ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled; design of record [design/findings-channel.md](../design/findings-channel.md) → 1). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

A doctype's **cardinality** — one singleton file holding every entry as a repeatable item, or one file per entry — is chosen once, at the doctype's first ship, and nothing can change it afterwards. The corpus-migration classifier knows changes *within* a doctype's shape (added optional fields, value remaps, relocations — [corpus-migration.md](../design/corpus-migration.md) → The classifier's holes); it has no kind that turns N items of one doc into N docs, or N docs into N items of one. So the choice is a one-way door. M55 met it head-on: the findings channel chose **one doc per finding** over a singleton ledger and a per-batch doc, and the only thing that made the vote final was that no transform exists to revisit it.

## The direction

A deterministic, LLM-free transform in both directions — the human's words: *"parse from one file to many and back when needed."*

- **Split** (singleton → per-doc): each repeatable item becomes an instance — its frozen item id becomes the slug, its item fields become header fields, its item slots become slots, the singleton's header fields either drop or copy per a declared map.
- **Merge** (per-doc → singleton): the inverse, ordered by a declared key (slug, or a date field), each instance becoming one item.
- **References follow.** A `<ty>:<slug>#<section>/<id>` item address becomes `<ty>:<id>`, and back — the repoint is `jigc rename`'s lockstep machinery ([write-commands.md](../design/write-commands.md) → `jigc rename`), applied to every structured referrer, so reference integrity survives the identity change exactly as it does for a re-slug.

Determinism boundary: pure structure, no prose judgment — the CLI moves bytes between homes it already owns. The adjacent, *prose*-judging fork — one **foreign** source doc to many doctypes — is a different thing and is recorded on its own ([decisions-pending.md](../implementation/decisions-pending.md), the one-source→many-doctypes migration (F11), re-opened 2026-07-10).

## Why parked

S1 settled the findings channel on one doc per finding, and the transform is not needed to ship it. The human asked for it to be kept *"so we can add it later"*: with it, a cardinality choice stops being a one-way door, which lowers the stakes of every future cardinality vote. It is new mechanism with no consumer today, so the mechanism line (build now only when cheaper than later or blocking) does not admit it.

## What it would cost

- A new transform kind in the corpus-migration pipeline, with a schema pair whose diff the classifier must recognize as a cardinality change rather than a removed section plus an added doctype — plus its byte-stability goldens in both directions.
- An identity rule for the split: item ids are minted-and-frozen and may diverge from a re-slug of their titles (`-2` suffixes, retitles), so the new slug must be the item id, never a re-mint.
- The repoint over the edge index and every placement/location branch, and a `schema-version` bump with its migration on the doctype(s) involved — the M38 location-is-gated rule applies, since a home changes.
- A declared map for header fields that have no item counterpart (merge) or no instance counterpart (split).

## Trigger

The first time a shipped doctype's cardinality turns out wrong on a real corpus — the findings store at M57's triage wanting one ledger view, or a singleton outgrowing its file — or the pickup of the one-source→many-doctypes fork, which needs the same identity-and-repoint half.
