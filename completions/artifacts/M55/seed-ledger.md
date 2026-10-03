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
- **seed doc** — the doc the seed's one fan-out filed for the row, linked under `seed/`. The driver's own output is [`seed-filing/filing-log.txt`](seed-filing/filing-log.txt).

## The ledger

| key | doctype | sources | verdict | seed doc |
|---|---|---|---|---|
| `rc16-1-a1-n1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(1, A1-N1)` | open | [jigc-feedback/routes-whose-operand-starts-with.md](seed/jigc-feedback/routes-whose-operand-starts-with.md) |
| `rc16-1-a1-n2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(1, A1-N2)` | open | [jigc-feedback/dash-root-knob-value.md](seed/jigc-feedback/dash-root-knob-value.md) |
| `rc16-2-defect-c` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(2, DEFECT C)` | open | [jigc-feedback/commit-seam-posture-breach-prints.md](seed/jigc-feedback/commit-seam-posture-breach-prints.md) |
| `rc16-4-defect-1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(4, DEFECT 1)` | resolved | [jigc-feedback/stage-failed-route-at-task.md](seed/jigc-feedback/stage-failed-route-at-task.md) |
| `rc16-5-defect-2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, DEFECT 2)` | open | [jigc-feedback/config-remove-step-and-replace.md](seed/jigc-feedback/config-remove-step-and-replace.md) |
| `rc16-6-d-1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(6, D-1)` | resolved | [jigc-feedback/orientation-omits-the-repository-posture.md](seed/jigc-feedback/orientation-omits-the-repository-posture.md) |
| `rc16-6-d-2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(6, D-2)` | open | [jigc-feedback/fix-task-composes-by-name.md](seed/jigc-feedback/fix-task-composes-by-name.md) |
| `rc16-7-a7-f3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(7, A7-F3)` | resolved | [jigc-feedback/doc-show-over-a-relocated.md](seed/jigc-feedback/doc-show-over-a-relocated.md) |
| `rc16-1-a1-n3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(1, A1-N3)` | open | [jigc-feedback/flatten-exemption-list-omits-write.md](seed/jigc-feedback/flatten-exemption-list-omits-write.md) |
| `rc16-2-defect-b` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(2, DEFECT B)` | open | [jigc-feedback/conflicted-squash-merge-is-named.md](seed/jigc-feedback/conflicted-squash-merge-is-named.md) |
| `rc16-3-a3-3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(3, A3-3)` | resolved | [jigc-feedback/boundary-displacement-suite-never-tests.md](seed/jigc-feedback/boundary-displacement-suite-never-tests.md) |
| `rc16-4-defect-2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(4, DEFECT 2)` | open | [jigc-feedback/repoint-failed-message-prints.md](seed/jigc-feedback/repoint-failed-message-prints.md) |
| `rc16-4-defect-3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(4, DEFECT 3)` | open | [jigc-feedback/hook-rejected-milestone-create-keeps.md](seed/jigc-feedback/hook-rejected-milestone-create-keeps.md) |
| `rc16-5-defect-3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, DEFECT 3)` | open | [jigc-feedback/renames-colon-less-address-refusal.md](seed/jigc-feedback/renames-colon-less-address-refusal.md) |
| `rc16-5-defect-4` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, DEFECT 4)` | open | [jigc-feedback/doc-show-calls-a-declared.md](seed/jigc-feedback/doc-show-calls-a-declared.md) |
| `rc16-5-c1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, C1)` | open | [jigc-feedback/orientation-json-omits-its-declared.md](seed/jigc-feedback/orientation-json-omits-its-declared.md) |
| `rc16-5-d1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(5, D1)` | open | [jigc-feedback/start-explain-json-is.md](seed/jigc-feedback/start-explain-json-is.md) |
| `rc16-6-d-3` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(6, D-3)` | open | [jigc-feedback/orientation-preview-footer-misses.md](seed/jigc-feedback/orientation-preview-footer-misses.md) |
| `rc16-6-d-4` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(6, D-4)` | open | [jigc-feedback/empty-milestone-execute-walk-never.md](seed/jigc-feedback/empty-milestone-execute-walk-never.md) |
| `rc16-7-a7-f1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(7, A7-F1)` | open | [jigc-feedback/validate-trailer-asserts-a-commit.md](seed/jigc-feedback/validate-trailer-asserts-a-commit.md) |
| `rc16-7-a7-f2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(7, A7-F2)` | open | [jigc-feedback/ingest-says-no-action-needed.md](seed/jigc-feedback/ingest-says-no-action-needed.md) |
| `rc16-8-n-1` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(8, N-1)` | open | [jigc-feedback/uninstall-force-help-names-three.md](seed/jigc-feedback/uninstall-force-help-names-three.md) |
| `rc16-8-n-2` | `jigc-feedback` | [M52](../M52/per-axis-review/README.md) `(8, N-2)` | open | [jigc-feedback/retitle-only-rename-acks.md](seed/jigc-feedback/retitle-only-rename-acks.md) |
| `m53-settle-a` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (a)** | open | [jigc-feedback/empty-add-item-title.md](seed/jigc-feedback/empty-add-item-title.md) |
| `m53-settle-b` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (b)** | open | [jigc-feedback/empty-title-at-doc.md](seed/jigc-feedback/empty-title-at-doc.md) |
| `m53-settle-c` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (c)** | open | [jigc-feedback/task-with-no-recorded.md](seed/jigc-feedback/task-with-no-recorded.md) |
| `m53-settle-d` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (d)** | open | [jigc-feedback/displacement-suffix-walk-has.md](seed/jigc-feedback/displacement-suffix-walk-has.md) |
| `m53-settle-e` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (e)** | open | [jigc-feedback/task-mint-writes-its.md](seed/jigc-feedback/task-mint-writes-its.md) |
| `m53-settle-f` | `jigc-feedback` | [M53 Settle](../../../implementation/decisions-pending.md) **(D) (f)** | open | [jigc-feedback/m52-reviews-r-i.md](seed/jigc-feedback/m52-reviews-r-i.md) |
| `m53-rc17-3-f-1` | `jigc-feedback` | [rc.17](../M53/per-axis-review/README.md) `(3, F-1)` | open | [jigc-feedback/symlinked-staged-doc-is.md](seed/jigc-feedback/symlinked-staged-doc-is.md) |
| `m53-rc17-3-f-2` | `jigc-feedback` | [rc.17](../M53/per-axis-review/README.md) `(3, F-2)` | open | [jigc-feedback/leftover-area-note-calls.md](seed/jigc-feedback/leftover-area-note-calls.md) |
| `m53-rc17-5-defect-a` | `jigc-feedback` | [rc.17](../M53/per-axis-review/README.md) `(5, DEFECT A)` | open | [jigc-feedback/doc-show-keys-an-unknown.md](seed/jigc-feedback/doc-show-keys-an-unknown.md) |
| `m53-rc18-2-f-1` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(2, F-1)` | resolved | [jigc-feedback/aimed-git-c-route.md](seed/jigc-feedback/aimed-git-c-route.md) |
| `m53-rc18-3-f-3` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(3, F-3)` | resolved | [jigc-feedback/uninstall-from-a-fan-out.md](seed/jigc-feedback/uninstall-from-a-fan-out.md) |
| `m53-rc18-2-f-2` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(2, F-2)` | open | [jigc-feedback/milestone-finalize-commits-a-foreign.md](seed/jigc-feedback/milestone-finalize-commits-a-foreign.md) |
| `m53-rc18-3-f-4` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(3, F-4)` | open | [jigc-feedback/fan-out-teardown-calls.md](seed/jigc-feedback/fan-out-teardown-calls.md) |
| `m53-rc18-3-f-5` | `jigc-feedback` | [rc.18](../M53/per-axis-review-rc18/README.md) `(3, F-5)` | open | [jigc-feedback/settled-sub-tasks-restored.md](seed/jigc-feedback/settled-sub-tasks-restored.md) |
| `m53-rc19-3-f-a` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(3, F-A)` | resolved | [jigc-feedback/task-finalize-prints-host-absolute.md](seed/jigc-feedback/task-finalize-prints-host-absolute.md) |
| `m53-rc19-2-n-1` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(2, N-1)` | resolved | [jigc-feedback/pre-commit-hook-announces.md](seed/jigc-feedback/pre-commit-hook-announces.md) |
| `m53-rc19-2-n-2` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(2, N-2)` | resolved | [jigc-feedback/hooks-rename-block-never.md](seed/jigc-feedback/hooks-rename-block-never.md) |
| `m53-rc19-2-n-3` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(2, N-3)` | open | [jigc-feedback/provision-acks-the-milestone-base.md](seed/jigc-feedback/provision-acks-the-milestone-base.md) |
| `m53-rc19-2-n-4` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(2, N-4)` | open | [jigc-feedback/linked-worktree-finalize-leaves.md](seed/jigc-feedback/linked-worktree-finalize-leaves.md) |
| `m53-rc19-3-f-b` | `jigc-feedback` | [rc.19](../M53/per-axis-review-rc19/README.md) `(3, F-B)` | open | [jigc-feedback/leftover-file-consent-warning.md](seed/jigc-feedback/leftover-file-consent-warning.md) |
| `m53-rc20-2-a2-2` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(2, A2-2)` | resolved | [jigc-feedback/previews-call-a-fan-out.md](seed/jigc-feedback/previews-call-a-fan-out.md) |
| `m53-rc20-2-a2-3` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(2, A2-3)` | resolved | [jigc-feedback/f-10-settle-said.md](seed/jigc-feedback/f-10-settle-said.md) |
| `m53-rc20-2-a2-1` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(2, A2-1)` | resolved | [jigc-feedback/dry-run-help-says-its.md](seed/jigc-feedback/dry-run-help-says-its.md) |
| `m53-rc20-3-f-c` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(3, F-C)` | resolved | [jigc-feedback/task-amends-unslugable-title-route.md](seed/jigc-feedback/task-amends-unslugable-title-route.md) |
| `m53-rc20-5-defect-2` | `jigc-feedback` | [rc.20](../M53/per-axis-review-rc20/README.md) `(5, DEFECT 2 · rc.20)` | resolved | [jigc-feedback/amend-head-shape-locus-prints.md](seed/jigc-feedback/amend-head-shape-locus-prints.md) |
| `owed-rig-eval-capture` | `jigc-feedback` | [owed](../../../implementation/decisions-pending.md) **`dev/jigc-rig` emits its assignments as text the caller must `eval`, and a capture that folds stderr in half-applies.** | open | [jigc-feedback/rig-capture-that-folds.md](seed/jigc-feedback/rig-capture-that-folds.md) |
| `owed-private-target-litter` | `jigc-feedback` | [owed](../../../implementation/decisions-pending.md) **`dev/gate --private-target` mints a fresh `$TMPDIR/jigc-gate-target-*` per run and `clean-litter` never sees them** | open | [jigc-feedback/gate-private-targets-pile-up.md](seed/jigc-feedback/gate-private-targets-pile-up.md) |
| `ci-git-marker-contract` | `jigc-feedback` | [CI](../../../implementation/decisions-pending.md) **The git 2.54.0 marker contract deferral was put on a live trigger — and the trigger did not fire.** | open | [jigc-feedback/git-marker-contract-still.md](seed/jigc-feedback/git-marker-contract-still.md) |
| `ci-gpg-signed-test-commits` | `jigc-feedback` | [CI](../../../implementation/decisions-pending.md) **(I) Test commits are signed with the developer's real GPG key.** | open | [jigc-feedback/test-repositories-sign-their-throwaway.md](seed/jigc-feedback/test-repositories-sign-their-throwaway.md) |
| `ci-undeclared-python3` | `jigc-feedback` | [CI](../../../implementation/decisions-pending.md) **(I) Eight `dogfood_apparatus::*` tests need `python3` and do not declare it.** | open | [jigc-feedback/dogfood-apparatus-tests-fail.md](seed/jigc-feedback/dogfood-apparatus-tests-fail.md) |
| `m55-f1` | `jigc-feedback` | [register](planning-findings.md) F1 | resolved | [jigc-feedback/code-less-finalize-commits.md](seed/jigc-feedback/code-less-finalize-commits.md) |
| `m55-f2` | `jigc-feedback` | [register](planning-findings.md) F2 | resolved | [jigc-feedback/create-over-an-existing.md](seed/jigc-feedback/create-over-an-existing.md) |
| `m55-f3` | `jigc-feedback` | [register](planning-findings.md) F3 | resolved | [jigc-feedback/slug-collision-is-routed.md](seed/jigc-feedback/slug-collision-is-routed.md) |
| `m55-f4` | `jigc-feedback` | [register](planning-findings.md) F4 | resolved | [jigc-feedback/pulled-edit-conflict-blocks.md](seed/jigc-feedback/pulled-edit-conflict-blocks.md) |
| `m55-f5` | `jigc-feedback` | [register](planning-findings.md) F5 | resolved | [jigc-feedback/store-validate-blocks.md](seed/jigc-feedback/store-validate-blocks.md) |
| `m55-f6` | `jigc-feedback` | [register](planning-findings.md) F6<br>[bound](../../../design/findings-channel.md) **L3** | open | [jigc-feedback/user-created-worktree.md](seed/jigc-feedback/user-created-worktree.md) |
| `m55-f7` | `jigc-feedback` | [register](planning-findings.md) F7 | resolved | [jigc-feedback/fan-out-sub-task.md](seed/jigc-feedback/fan-out-sub-task.md) |
| `m55-f8` | `jigc-feedback` | [register](planning-findings.md) F8 | open | [jigc-feedback/union-resolved-singleton-loses.md](seed/jigc-feedback/union-resolved-singleton-loses.md) |
| `m55-f9` | `jigc-feedback` | [register](planning-findings.md) F9 | open | [jigc-feedback/required-on-create-date.md](seed/jigc-feedback/required-on-create-date.md) |
| `m55-f10` | `jigc-feedback` | [register](planning-findings.md) F10 | resolved | [jigc-feedback/doc-show-json-omits.md](seed/jigc-feedback/doc-show-json-omits.md) |
| `m55-f11` | `jigc-feedback` | [register](planning-findings.md) F11 | open | [jigc-feedback/card-on-a-non.md](seed/jigc-feedback/card-on-a-non.md) |
| `m55-f12` | `jigc-feedback` | [register](planning-findings.md) F12 | open | [jigc-feedback/type-change-rides.md](seed/jigc-feedback/type-change-rides.md) |
| `m55-f13` | `jigc-feedback` | [register](planning-findings.md) F13 | open | [jigc-feedback/same-id-built-in.md](seed/jigc-feedback/same-id-built-in.md) |
| `m55-f14` | `jigc-feedback` | [register](planning-findings.md) F14 | open | [jigc-feedback/any-task-can-edit-any.md](seed/jigc-feedback/any-task-can-edit-any.md) |
| `m55-f15` | `jigc-feedback` | [register](planning-findings.md) F15 | open | [jigc-feedback/setup-re-enables-the-methodology.md](seed/jigc-feedback/setup-re-enables-the-methodology.md) |
| `m55-f16` | `jigc-feedback` | [register](planning-findings.md) F16 | open | [jigc-feedback/project-workflow-shadow-goes.md](seed/jigc-feedback/project-workflow-shadow-goes.md) |
| `m55-f17` | `jigc-feedback` | [register](planning-findings.md) F17 | open | [jigc-feedback/rig-cannot-prototype.md](seed/jigc-feedback/rig-cannot-prototype.md) |
| `m55-f18` | `jigc-feedback` | [register](planning-findings.md) F18 | resolved | [jigc-feedback/listing-open-findings-costs.md](seed/jigc-feedback/listing-open-findings-costs.md) |
| `m55-f19` | `jigc-feedback` | [register](planning-findings.md) F19 | open | [jigc-feedback/task-can-create-only.md](seed/jigc-feedback/task-can-create-only.md) |
| `m55-f20` | `jigc-feedback` | [register](planning-findings.md) F20 | open | [jigc-feedback/ingest-adopts-files-and-leaves.md](seed/jigc-feedback/ingest-adopts-files-and-leaves.md) |
| `m55-t1` | `jigc-feedback` | [register](planning-findings.md) T1 | resolved | [jigc-feedback/no-cd-assertion-trips.md](seed/jigc-feedback/no-cd-assertion-trips.md) |
| `m55-t2` | `jigc-feedback` | [register](planning-findings.md) T2 | resolved | [jigc-feedback/test-helpers-panic-when-jigc.md](seed/jigc-feedback/test-helpers-panic-when-jigc.md) |
| `m55-t3` | `jigc-feedback` | [register](planning-findings.md) T3 | resolved | [jigc-feedback/every-gate-run-leaks-scratch.md](seed/jigc-feedback/every-gate-run-leaks-scratch.md) |
| `m55-t4` | `jigc-feedback` | [register](planning-findings.md) T4 | resolved | [jigc-feedback/gate-pays-the-xcrun.md](seed/jigc-feedback/gate-pays-the-xcrun.md) |
| `m55-t5` | `jigc-feedback` | [register](planning-findings.md) T5 | open | [jigc-feedback/batch-apply-growth-ratio.md](seed/jigc-feedback/batch-apply-growth-ratio.md) |
| `m55-bound-two-code-tasks` | `jigc-feedback` | [bound](../../../design/findings-channel.md) **Two code tasks in one checkout** | open | [jigc-feedback/two-tasks-on-the-ordinary.md](seed/jigc-feedback/two-tasks-on-the-ordinary.md) |
| `m55-d1` | `inconsistency` | [register](planning-findings.md) D1 | resolved | [inconsistencies/record-only-names-two-different.md](seed/inconsistencies/record-only-names-two-different.md) |
| `m55-d2` | `inconsistency` | [register](planning-findings.md) D2 | resolved | [inconsistencies/two-definitions-of-pinned-by.md](seed/inconsistencies/two-definitions-of-pinned-by.md) |
| `m55-d3` | `inconsistency` | [register](planning-findings.md) D3 | resolved | [inconsistencies/three-finding-vocabularies-and-tier.md](seed/inconsistencies/three-finding-vocabularies-and-tier.md) |
| `m55-d4` | `inconsistency` | [register](planning-findings.md) D4 | resolved | [inconsistencies/m55-charter-miscounts-its.md](seed/inconsistencies/m55-charter-miscounts-its.md) |
| `m55-d5` | `inconsistency` | [register](planning-findings.md) D5 | open | [inconsistencies/methodology-docs-call-remove-item.md](seed/inconsistencies/methodology-docs-call-remove-item.md) |
| `m55-d6` | `inconsistency` | [register](planning-findings.md) D6 | open | [inconsistencies/methodology-docs-show-the-deferral.md](seed/inconsistencies/methodology-docs-show-the-deferral.md) |
| `m55-d7` | `inconsistency` | [register](planning-findings.md) D7 | resolved | [inconsistencies/project-guide-miscounts.md](seed/inconsistencies/project-guide-miscounts.md) |
| `m55-d8` | `inconsistency` | [register](planning-findings.md) D8 | open | [inconsistencies/staging-contract-test-hand.md](seed/inconsistencies/staging-contract-test-hand.md) |
| `m55-d9` | `inconsistency` | [register](planning-findings.md) D9 | resolved | [inconsistencies/registration-checklist-names-none.md](seed/inconsistencies/registration-checklist-names-none.md) |
| `m55-d10` | `inconsistency` | [register](planning-findings.md) D10 | open | [inconsistencies/reconciliation-calls-start-a-pure.md](seed/inconsistencies/reconciliation-calls-start-a-pure.md) |
| `m55-d11` | `inconsistency` | [register](planning-findings.md) D11 | open | [inconsistencies/worked-examples-index-lists.md](seed/inconsistencies/worked-examples-index-lists.md) |
| `m55-d12` | `inconsistency` | [register](planning-findings.md) D12 | resolved | [inconsistencies/release-docs-cite-a-stale.md](seed/inconsistencies/release-docs-cite-a-stale.md) |
