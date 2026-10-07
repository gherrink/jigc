# Proposal: the per-axis review rows for the rc.24 fix pass's delta

*Returned 2026-10-06 by a read-only analysis of `13dc5bc7..0f34d8f0` (55 product commits). Transcribed by the orchestrator. Counts were read that day (product paths identical to `0f34d8f0`). In the Commits column, shas left of `|` were audited at `5f5b273a`; right of it are unaudited. Pair counts and the cell matrix are inferred. The human approves or cuts the rows before any brief is written.*

## Seven rows

| # | Row — subject | Doors derived from (count) | Commits | Re-drive as fixed | Descends from (rc.24 instrument) | Audit overlap | Pairs |
|---|---|---|---|---|---|---|---|
| 1 | **Install members** — what `setup` does over each path it writes | `install_tracked_paths` (7 + 3 conditional + link targets; 5 replaced, 5 merged-into) · `IGNORE_DOORS` 4 | dc0d7586 104a7d4b ef9456b5 \| 676eff57 f0b1120a 33620081 55cb29fa cba98c44 a330b4f9 | `(R1, F1)` | old 1 | 6/9 | ~70 |
| 2 | **Teardown** — what `uninstall` takes, names, refuses | `UNINSTALL_DOOR.codes` 4 (in `DESTROYING_DOORS` 6) · `PrefixOwner` 4 + default | 12398ddc 4fa1c0f5 \| 528d0205 1b1d7656 8b214ad5 dcfa40f0 | `(R9, F5)` | old 1, 2 | 4/6 | ~35 |
| 3 | **Worktree registrations** — a door drops only a registration jigc made, never one holding work | `WORKTREE_DOORS` 4 · `remove_owned_registration` (6 grep hits) · `status_argv` (10) | 8c159622 ebfc79fb e842342e \| 894f2234 f3186486 c3f6dcaa ad1d2ef7 f1b84ef1 7e5ba186 ed783637 | `L-22` (four doors, the refusing finalize, container cell, gc permanence) | none (trial) | 7/10 | ~55 |
| 4 | **Home occupancy** — a created doc never lands over a home git or a worktree holds | `COMMITTING_DOORS` 11 (finalize and `milestone create` rows) · `MINT_DOORS` 6 · `AMBUSH_CONTRACTS` 7 | 3f3a724b \| 78e8ded1 d724365f f031d31e 048724d0 | `(R6, D-1)` | old 5, 6 | 4/5 | ~45 |
| 5 | **Home shape** — no writer or mover goes through a link or non-regular entry | callers of `store::home_entry` (25 hits, 11 files) · `ROLLBACK_DOORS` 5 · `ROLLBACK_POPULATIONS` 11 | 9465f9b6 5f5b273a \| c97b8eb6 685a4c55 526141e6 242341bb | `(R6, D-7)` | old 6 | 4/6 | ~60 |
| 6 | **Reconciliation** — baseline, pin verdict, copied-in witness | `VERB_KINDS` doc × Write 8 · `STORE_FAMILIES` 7 · `STORE_EXIT_FLIPS` 7 · `reconcile_record_preflight` (8 hits) | eb18e5fe 8d9c3afb c0c4d88c \| 676eff57 4ba04efc d24a0c91 e39d9ac9 b54b58b2 0f34d8f0 | `(R3, F7)` · `(R6, K-1)` | old 3, 6 | 6/9 | ~80 |
| 7 | **Linked-worktree guard** — doc work from a user-made worktree refuses; elsewhere nothing changes | doc × Write 8 + finalize, validate, start, migrate · `finalize.linked-worktree-doc` literals (5, grep) | a88f71cc e3a6ba58 \| 55a6e281 ac0f63b1 e590806f d8f9907d e0f00278 df58d1dc | no key; lead RD-1 | none | 6/8 | ~50 |

**Cells, every row:** must-refuse beside must-not-refuse over git configuration (none · line-ending conversion · `status.showUntrackedFiles=no` · clean/smudge filter) × layout (plain · user worktree · fan-out worktree · bare + worktree · `--separate-git-dir` · submodule); every printed route run as printed; `--format json` on each new refusal. Linux, a case-sensitive filesystem and another git are reachable (Docker answers on this host). A real concurrent spawn (rows 3, 6) is no driver's cell.

## What the pass touched that no row covers
- **Guides and help:** sixteen commits moved guide bytes; no row — each row checks only the sentences stating its own behaviour.
- **Engine `pub` moves (CPL-10):** not reachable through the binary.
- **E2E F7's envelopes:** a cell, not a row.
- **Shared commits:** 33620081, 894f2234 and c3f6dcaa at `uninstall` are cells of row 2; 104a7d4b's stamp at `task finalize` is a cell of row 1.

## Overlap with an audit over `5f5b273a..0f34d8f0`
All 36 unaudited commits fall inside the rows. Round 4's areas map A→1, 6 · B→7 · C→4, 5 · D→6 · E→1, 2 · F→3, 5. Each row's source reader duplicates the audit's area reviewer; its driver duplicates the audit's end-to-end drive. Rows 3 and 7 coincide almost exactly with areas F and B.

## Size
Seven rows, about 395 (door, cell) pairs. For fewer: merge 1 + 2 and 4 + 5 (five rows); drop 7 first — no key, and it is new code the audit reads.

## Cross-model candidates (at most two)
- **Row 6:** the pass's one stored-format addition (`copied-in` in `docs/provenance.json`), built last, read by its fixer only.
- **Row 3:** destroying doors, a new pinned code, git-version-dependent parsing.

## Not determined
- The analysis brief's premise that the pass added door constants in `rollback.rs` fails: that file has no diff in the range. What moved is `FINALIZE_DOOR.codes` 0 → 1 (`milestone.unlanded-work`) and `AMBUSH_CONTRACTS` 6 → 7; every other count re-read equals the old instrument's.
- Grep counts include definitions and in-file tests.
- Read in summary only: the fixers' must-not-refuse lists and the audit's 72 `not_examined` items.
- The twenty unruled fixer decisions: whether reversing any changes cells in rows 1 and 3.
- The container's git version; whether a candidate binary exists.
