# M55 — the seed ledger

The count of record for the M55 seed ([roadmap.md](../../../implementation/roadmap.md) → Milestone 55 → Increment 9; [findings-channel.md](../../../design/findings-channel.md) → 7; [DECISIONS.md](../../../DECISIONS.md) → *2026-10-03 — M55 Increment 9 planning*, P1 and P2). **One row per distinct finding, recorded with every source row before anything is filed.** Each row becomes one seed doc, filed through the real binary by one fan-out. `crates/cli/tests/seed_ledger.rs` reads the sources themselves and holds this table to them: each source row sits in exactly one row's sources, and the keys are unique.

**The count: 91 rows** (79 `jigc-feedback`, 12 `inconsistency`) from 92 source entries. The one dedupe is the declared bound **L3**, which is register row F6 and joins `m55-f6`'s sources.

| Source | Rows | Tag |
|---|---|---|
| the rc.16 wave's tier-2/3 heads, under `### Tier 2` and `### Tier 3` of the [M52 per-axis review](../M52/per-axis-review/README.md) | 23 | `rc16-` |
| the M53 Settle's six, `(D) (a)`–`(f)` under [decisions-pending.md](../../../implementation/decisions-pending.md) → *Deferred at the M53 Settle* | 6 | `m53-settle-` |
| the tier-2/3 heads of M53's four re-reviews: [rc.17](../M53/per-axis-review/README.md) 3 · [rc.18](../M53/per-axis-review-rc18/README.md) 5 · [rc.19](../M53/per-axis-review-rc19/README.md) 6 · [rc.20](../M53/per-axis-review-rc20/README.md) 5 | 19 | `m53-rc17-` … `m53-rc20-` |
| decisions-pending → *Owed after M53's post-review arcs*, the two `dev/` rows | 2 | `owed-` |
| decisions-pending → the 2026-09-27 CI rows under *The road to 1.0.0 and the port* | 3 | `ci-` |
| the [planning register](planning-findings.md)'s F1–F20 and T1–T5 | 25 | `m55-f` · `m55-t` |
| the declared bound *two code tasks in one checkout* ([findings-channel.md](../../../design/findings-channel.md) → 6, *Declared bounds*) | 1 | `m55-bound-` |
| the register's D1–D12, as `inconsistency` | 12 | `m55-d` |

**Excluded, each with its reason** (in the sources grammar below, so the test can tell an exclusion from a row it lost):

- [owed](../../../implementation/decisions-pending.md) **A blind agent trial on the release binary — the usability instrument the five stamps never ran.** — an owed act, not a finding: it runs on the release candidate before the call and records its own rows.
- [M52 Settle](../../../implementation/decisions-pending.md) **(D) (a)** — the git-marker contract's origin, not a member of the set. The CI row `ci-git-marker-contract` cross-references it, and its text and trigger stand unchanged.

**Columns.**

- **key** — stable, filesystem-safe (`a-z0-9` words joined by `-`), prefixed by its first source's tag. The per-row filing inputs live at `seed-filing/input/<doctype>/<key>/`, and the re-drive batches glob on the prefix.
- **doctype** — `jigc-feedback`, or `inconsistency` for a register D row.
- **sources** — every source row the finding was recorded at, separated by `<br>`. Each is `[<tag>](<file>)` followed by the row's own id as its source spells it: a register id (`F4`), a review head's first id (`` `(2, DEFECT C)` ``), or a bold label (`**(D) (a)**`, `**L3**`, an owed or CI row's opening bold sentence, verbatim). A repeat source joins the first row's sources and is never a second doc, because `duplicate-of` cannot name a sibling sub-task's doc inside the one fan-out.
- **verdict** — blank until the row is re-driven on this build: `open`, `resolved`, `refuted`, or, for an `inconsistency`, `intended`.
- **seed doc** — blank until the seed is filed: the doc's path under `seed/`.

## The ledger

| key | doctype | sources | verdict | seed doc |
|---|---|---|---|---|
| `rc16-1-a1-n1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(1, A1-N1)` | | |
| `rc16-1-a1-n2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(1, A1-N2)` | | |
| `rc16-2-defect-c` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(2, DEFECT C)` | | |
| `rc16-4-defect-1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(4, DEFECT 1)` | | |
| `rc16-5-defect-2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, DEFECT 2)` | | |
| `rc16-6-d-1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(6, D-1)` | | |
| `rc16-6-d-2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(6, D-2)` | | |
| `rc16-7-a7-f3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(7, A7-F3)` | | |
| `rc16-1-a1-n3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(1, A1-N3)` | | |
| `rc16-2-defect-b` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(2, DEFECT B)` | | |
| `rc16-3-a3-3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(3, A3-3)` | | |
| `rc16-4-defect-2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(4, DEFECT 2)` | | |
| `rc16-4-defect-3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(4, DEFECT 3)` | | |
| `rc16-5-defect-3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, DEFECT 3)` | | |
| `rc16-5-defect-4` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, DEFECT 4)` | | |
| `rc16-5-c1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, C1)` | | |
| `rc16-5-d1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, D1)` | | |
| `rc16-6-d-3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(6, D-3)` | | |
| `rc16-6-d-4` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(6, D-4)` | | |
| `rc16-7-a7-f1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(7, A7-F1)` | | |
| `rc16-7-a7-f2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(7, A7-F2)` | | |
| `rc16-8-n-1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(8, N-1)` | | |
| `rc16-8-n-2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(8, N-2)` | | |
| `m53-settle-a` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (a)** | | |
| `m53-settle-b` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (b)** | | |
| `m53-settle-c` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (c)** | | |
| `m53-settle-d` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (d)** | | |
| `m53-settle-e` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (e)** | | |
| `m53-settle-f` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (f)** | | |
| `m53-rc17-3-f-1` | `jigc-feedback` | [rc.17](../M53/per-axis-review/README.md) `(3, F-1)` | | |
| `m53-rc17-3-f-2` | `jigc-feedback` | [rc.17](../M53/per-axis-review/README.md) `(3, F-2)` | | |
| `m53-rc17-5-defect-a` | `jigc-feedback` | [rc.17](../M53/per-axis-review/README.md) `(5, DEFECT A)` | | |
| `m53-rc18-2-f-1` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(2, F-1)` | | |
| `m53-rc18-3-f-3` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(3, F-3)` | | |
| `m53-rc18-2-f-2` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(2, F-2)` | | |
| `m53-rc18-3-f-4` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(3, F-4)` | | |
| `m53-rc18-3-f-5` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(3, F-5)` | | |
| `m53-rc19-3-f-a` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(3, F-A)` | | |
| `m53-rc19-2-n-1` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(2, N-1)` | | |
| `m53-rc19-2-n-2` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(2, N-2)` | | |
| `m53-rc19-2-n-3` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(2, N-3)` | | |
| `m53-rc19-2-n-4` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(2, N-4)` | | |
| `m53-rc19-3-f-b` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(3, F-B)` | | |
| `m53-rc20-2-a2-2` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(2, A2-2)` | | |
| `m53-rc20-2-a2-3` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(2, A2-3)` | | |
| `m53-rc20-2-a2-1` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(2, A2-1)` | | |
| `m53-rc20-3-f-c` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(3, F-C)` | | |
| `m53-rc20-5-defect-2` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(5, DEFECT 2 · rc.20)` | | |
| `owed-rig-eval-capture` | `jigc-feedback` | [owed](../../../implementation/decisions-pending.md) **`dev/jigc-rig` emits its assignments as text the caller must `eval`, and a capture that folds stderr in half-applies.** | | |
| `owed-private-target-litter` | `jigc-feedback` | [owed](../../../implementation/decisions-pending.md) **`dev/gate --private-target` mints a fresh `$TMPDIR/jigc-gate-target-*` per run and `clean-litter` never sees them** | | |
| `ci-git-marker-contract` | `jigc-feedback` | [CI](../../../implementation/decisions-pending.md) **The git 2.54.0 marker contract deferral was put on a live trigger — and the trigger did not fire.** | | |
| `ci-gpg-signed-test-commits` | `jigc-feedback` | [CI](../../../implementation/decisions-pending.md) **(I) Test commits are signed with the developer's real GPG key.** | | |
| `ci-undeclared-python3` | `jigc-feedback` | [CI](../../../implementation/decisions-pending.md) **(I) Eight `dogfood_apparatus::*` tests need `python3` and do not declare it.** | | |
| `m55-f1` | `jigc-feedback` | [register](planning-findings.md) F1 | | |
| `m55-f2` | `jigc-feedback` | [register](planning-findings.md) F2 | | |
| `m55-f3` | `jigc-feedback` | [register](planning-findings.md) F3 | | |
| `m55-f4` | `jigc-feedback` | [register](planning-findings.md) F4 | | |
| `m55-f5` | `jigc-feedback` | [register](planning-findings.md) F5 | | |
| `m55-f6` | `jigc-feedback` | [register](planning-findings.md) F6<br>[bound](../../../design/findings-channel.md) **L3** | | |
| `m55-f7` | `jigc-feedback` | [register](planning-findings.md) F7 | | |
| `m55-f8` | `jigc-feedback` | [register](planning-findings.md) F8 | | |
| `m55-f9` | `jigc-feedback` | [register](planning-findings.md) F9 | | |
| `m55-f10` | `jigc-feedback` | [register](planning-findings.md) F10 | | |
| `m55-f11` | `jigc-feedback` | [register](planning-findings.md) F11 | | |
| `m55-f12` | `jigc-feedback` | [register](planning-findings.md) F12 | | |
| `m55-f13` | `jigc-feedback` | [register](planning-findings.md) F13 | | |
| `m55-f14` | `jigc-feedback` | [register](planning-findings.md) F14 | | |
| `m55-f15` | `jigc-feedback` | [register](planning-findings.md) F15 | | |
| `m55-f16` | `jigc-feedback` | [register](planning-findings.md) F16 | | |
| `m55-f17` | `jigc-feedback` | [register](planning-findings.md) F17 | | |
| `m55-f18` | `jigc-feedback` | [register](planning-findings.md) F18 | | |
| `m55-f19` | `jigc-feedback` | [register](planning-findings.md) F19 | | |
| `m55-f20` | `jigc-feedback` | [register](planning-findings.md) F20 | | |
| `m55-t1` | `jigc-feedback` | [register](planning-findings.md) T1 | | |
| `m55-t2` | `jigc-feedback` | [register](planning-findings.md) T2 | | |
| `m55-t3` | `jigc-feedback` | [register](planning-findings.md) T3 | | |
| `m55-t4` | `jigc-feedback` | [register](planning-findings.md) T4 | | |
| `m55-t5` | `jigc-feedback` | [register](planning-findings.md) T5 | | |
| `m55-bound-two-code-tasks` | `jigc-feedback` | [bound](../../../design/findings-channel.md) **Two code tasks in one checkout** | | |
| `m55-d1` | `inconsistency` | [register](planning-findings.md) D1 | | |
| `m55-d2` | `inconsistency` | [register](planning-findings.md) D2 | | |
| `m55-d3` | `inconsistency` | [register](planning-findings.md) D3 | | |
| `m55-d4` | `inconsistency` | [register](planning-findings.md) D4 | | |
| `m55-d5` | `inconsistency` | [register](planning-findings.md) D5 | | |
| `m55-d6` | `inconsistency` | [register](planning-findings.md) D6 | | |
| `m55-d7` | `inconsistency` | [register](planning-findings.md) D7 | | |
| `m55-d8` | `inconsistency` | [register](planning-findings.md) D8 | | |
| `m55-d9` | `inconsistency` | [register](planning-findings.md) D9 | | |
| `m55-d10` | `inconsistency` | [register](planning-findings.md) D10 | | |
| `m55-d11` | `inconsistency` | [register](planning-findings.md) D11 | | |
| `m55-d12` | `inconsistency` | [register](planning-findings.md) D12 | | |
