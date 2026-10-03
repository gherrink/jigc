---
kind: code-doc
status: resolved
date: 2026-10-03
schema-version: 1
---

# Three finding vocabularies, and tier meaning three things

## Sides

### implementation/decisions-pending.md:105  {#side1}

The port paragraph's `findings-ledger` item: tier · door · repro block · status · `pinned-by`.

### ideas/finding-doctype.md:13  {#side2}

A standalone `finding`: `severity`, `disposition` and `evidence`, plus a code anchor.

### crates/cli/packs/methodology/schemas/completion-record.yaml:57  {#side3}

A completion finding: `severity` in `[blocking, advisory, HIGH, MEDIUM, LOW]` and `disposition` in `[fixed, deferred, contested]`.

### implementation/decisions-pending.md:123  {#side4}

*Tier 0 — exit-0 loss or repository harm*: the rc.16 charter's consequence predicate, numbered 0 · 1 · 2. The M52 review numbers the same predicate `tier-1` · `tier-2` · `tier-3`.

### implementation/decisions-pending.md:213  {#side5}

*Tier 0 — blocks the 1.0.0 call* through *Tier 4 — record corrections*: tiers as M50's work batches.

## Description

Before M55, findings were recorded in three vocabularies: the port paragraph's ledger item, the parked finding-doctype idea, and the shipped `completion-record` finding. *Tier* also meant three things: the rc.16 charter's consequence predicate numbered 0 to 2, the same predicate numbered `tier-1` to `tier-3` from the M52 review on, and M50's work batches, Tier 0 to Tier 4. A findings doctype had to take one field set and one meaning of `tier`. Planning's gap list found it (G9).

## Evidence

Read at planning (`gap-list.md`, G9). Re-read at `a7a742d3`: the five sides, `design/findings-channel.md` → 1.1, 1.4 and 1.6, and the `tier` leaf in `crates/cli/packs/methodology/schemas/jigc-feedback.yaml:40`.

## Resolution

Settled at the M55 Settle (S1, S2, S6; `9fd644b7`) and registered by M55 Increment 7 (`9dcb664d`). `jigc-feedback` and `inconsistency` each take one field set (`findings-channel.md` → 1.1, 1.2). `tier` is the enum `tier-1 · tier-2 · tier-3`. `findings-channel.md` → 1.4 gives its meaning one home, the rc.16 charter's three headings, and states that the charter's 0 to 2 and the review's `tier-1` to `tier-3` number the same predicate. The port paragraph is kept as recorded under a `[Settled differently 2026-10-02 …]` bracket, and the idea file under its head note. `completion-record` keeps its own vocabulary for its own purpose, and `findings-channel.md` → 1.6 routes a deferred completion finding about jigc into `jigc-feedback` through `found-in`. UNPINNED: the vocabularies are prose. The two field sets are held by the methodology manifest's `schema-hash` at pack-load, which is not a test of this disagreement.

Not changed: `decisions-pending.md:213` still uses *Tier* for M50's work batches. It is a dated record, left as written.

Re-driven at `a7a742d3`: the sides read as quoted.
