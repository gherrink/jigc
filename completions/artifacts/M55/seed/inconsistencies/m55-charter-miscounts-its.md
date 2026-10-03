---
kind: doc-doc
status: resolved
date: 2026-10-03
schema-version: 1
---

# The M55 charter miscounts its seed

## Sides

### implementation/roadmap.md:2944  {#side1}

The M55 charter: *the channel seeded with the rows already recorded for 1.x (the rc.16 wave's tier-2/3 set and the M53 Settle's six)*.

### implementation/decisions-pending.md:105  {#side2}

The port paragraph: *the 23 rows above are its seed*.

### completions/artifacts/M55/seed-ledger.md  {#side3}

The count of record: 91 rows from 92 source entries, with M53's four re-reviews' 19 rows among them, and each fixed row seeded `resolved`.

### design/findings-channel.md:255  {#side4}

§7, *Generation*: the count of record is the seed ledger.

## Description

At planning, the seed's description was stale in three ways. Three of the rc.16 wave's 23 rows had already been fixed by the 2026-09-23 batch: `(6, D-1)`, `(7, A7-F3)` and `(4, DEFECT 1)`. The port paragraph still called the 23 the seed. Its *rows added on 2026-09-27* matched no source row. And M53's four re-reviews' tier-2/3 rows, which each README's header sends to the ledger for 1.x, were counted nowhere. The read-back auditor found it.

## Evidence

Read at planning by the read-back auditor. Re-driven at `a7a742d3` through the ledger's own fence: `seed_ledger` reads each source itself and holds the ledger to it.

## Resolution

Resolved by M55 Increment 9 / T1 (`914df03d`). `seed-ledger.md` records every row with its sources before anything is filed:

- the rc.16 wave's 23 rows and the M53 Settle's six;
- M53's re-reviews' 19 rows: rc.17 3, rc.18 5, rc.19 6 and rc.20 5;
- the 2026-09-27 rows, which are the two owed and three CI rows, each named by its bold label;
- the register's 25 `jigc-feedback` rows and 12 `inconsistency` rows, and the declared bound.

That is 91 rows from 92 entries, because L3 is F6. `findings-channel.md` → 7 points at the ledger in place of a count, so the corrected side is findings-channel.md. A fixed row is seeded `resolved`, never dropped: the three the 2026-09-23 batch fixed are `rc16-4-defect-1`, `rc16-6-d-1` and `rc16-7-a7-f3`. Pinned by `seed_ledger::the_ledger_carries_every_source_row_exactly_once` and `seed_ledger::the_ledger_states_its_own_count`. The charter paragraph in roadmap.md is kept as chartered, and Increment 9's scope in the same file carries the counted set.

Re-driven at `a7a742d3`: `seed_ledger::` is green, and the three rows read `resolved` in the ledger.
