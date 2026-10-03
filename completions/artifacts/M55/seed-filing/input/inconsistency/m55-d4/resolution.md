Resolved by M55 Increment 9 / T1 (`914df03d`). `seed-ledger.md` records every row with its sources before anything is filed:

- the rc.16 wave's 23 rows and the M53 Settle's six;
- M53's re-reviews' 19 rows: rc.17 3, rc.18 5, rc.19 6 and rc.20 5;
- the 2026-09-27 rows, which are the two owed and three CI rows, each named by its bold label;
- the register's 25 `jigc-feedback` rows and 12 `inconsistency` rows, and the declared bound.

That is 91 rows from 92 entries, because L3 is F6. `findings-channel.md` → 7 points at the ledger in place of a count, so the corrected side is findings-channel.md. A fixed row is seeded `resolved`, never dropped: the three the 2026-09-23 batch fixed are `rc16-4-defect-1`, `rc16-6-d-1` and `rc16-7-a7-f3`. Pinned by `seed_ledger::the_ledger_carries_every_source_row_exactly_once` and `seed_ledger::the_ledger_states_its_own_count`. The charter paragraph in roadmap.md is kept as chartered, and Increment 9's scope in the same file carries the counted set.

Re-driven at `a7a742d3`: `seed_ledger::` is green, and the three rows read `resolved` in the ledger.
