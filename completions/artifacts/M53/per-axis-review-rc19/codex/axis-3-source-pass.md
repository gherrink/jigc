<!-- M53 THIRD PARTIAL per-axis review — axis 3 — the unseeded Codex SOURCE pass, verbatim. Source-only: it drove nothing and wrote nothing. Captured 2026-09-23 against the rc.19 tree. -->

# Axis 3 source pass — rc.19

Read-only review of rc.19’s production source through `4a862b96`; the two later commits modify only M53 records, not production Rust. I did not build, drive the binary, or modify files.

## Claims

1. **STILL-OPEN (tier 3, expected): a symlink named as a staged document is classified simultaneously as managed prose and as foreign bytes.**

   Evidence: `staged_doc_ids` follows the symlink with `metadata()` and accepts a regular-file target ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:701)); the destroying-door complement instead uses non-following `DirEntry::file_type()` and recognizes only regular files ([state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:452), [state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:457)). Task discard runs the foreign and prose guards consecutively before recursive removal ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:619), [task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:637)).

   Proposed reproduction: symlink `.jigc/tasks/<id>/docs/adr:linked.md` to an external regular Markdown file; run `jigc task discard <id>`, then repeat with `--force`. Expected wrong behaviour: the refusal calls the link foreign, while the successful acknowledgement calls the same entry dropped staged prose. The external target survives.

   Confidence: high.

2. **STILL-OPEN (tier 3, expected): the residual-area message asserts “directory” without testing that the area has that shape.**

   Evidence: `carries_base_pin` tests only whether `<area>/base.json` is a regular file ([state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:302)); `residual_area_note` unconditionally says the area “is a directory” ([state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:321)).

   Proposed reproduction: make `.jigc/tasks/ghost` a symlink to an external directory and run `jigc task discard ghost`. Expected wrong behaviour: `finalize.no-task` describes the symlink itself as a directory carrying no base pin. Target bytes survive.

   Confidence: high.

No new completeness or tier-1 lead was found inside the cwd-dependence arc.

## Baseline dispositions

- rc.18 `(3,F-1)` and `(3,F-2)`: **STILL-OPEN(1.x, expected)**, as claims 1–2 show.
- M52 `A3-1`: **CLOSED (source)**. Milestone complement membership walks `merged/` and recognizes only regular staged bodies ([state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:469), [state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:490)); teardown removes those members non-recursively and cannot remove a non-empty foreign remainder ([state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:652), [state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:681)).
- M52 `A3-2`: **CLOSED (source)**. Both finalize paths displace first and then use registry-bounded `unwind_area`; `Foreign` or an error leaves the area standing and produces `finalize.foreign-bytes` ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6286), [task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6364)).
- M52 `A3-3`: **CLOSED (source)**. Both `WorkArea::Task` and `WorkArea::Milestone` have explicit writer rows ([state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:222), [state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:231)); milestone finalize passes both subjects through the shared teardown.
- Post-review operation-bearing-worktree HIGH: **CLOSED (source)**. `probe_leftover` accepts only absent, empty, or clean-and-concluded paths; its `OwnWorktree` arm records `held_operation` ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3733), [milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3760)). Provision, discard, and uninstall consume that shared probe; finalize uses its earlier fan-out posture preflight.

## M51 dispositions

The three confirmed M51 source rows are now **CLOSED**:

- M51 claim 1, task-discard foreign-file loss: foreign complement refusal precedes recursive deletion ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:619)).
- M51 claim 2, milestone-discard sub-task-area loss: discard uses the same task-area complement, and only the consented `Take` arm retains `remove_dir_all` ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:6025)).
- M51 claim 3, milestone-finalize sub-task-area loss: finalize uses `Displace`, followed by `unwind_settled_area`, not recursive deletion ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5997)).

M51 §A’s `C-1`, `D-1`, `D-2`, `D-3`, and `D-4` remain **CLOSED (source)**: six-door enumeration and dispositions at [milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3430); ownership remains base-pin-derived; teardown acknowledgements key on actual removal success ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:4844)); paths render relative to `jigc_home`; and `symlink_metadata` distinguishes absent, directory, leaf, and unreadable ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3664)).

## Consistent census

The current registry correctly contains **six**, not four, destroying doors: provision, milestone discard, uninstall, task discard, task finalize, and milestone finalize, split into four `Refuse` and two `Displace` rows ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3315), [milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3430)). The brief’s four-door set is the narrower `WORKTREE_DOORS` subset ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3447)).

Production removal sites were accounted for as follows:

- Destructive doors: provision is guarded by `probe_leftover`; task/milestone discard by foreign-prose/worktree guards; uninstall by four pre-removal populations; finalizers by displacement plus registry-bounded unwind.
- Owned-artifact removal: hook/adapter uninstall ([setup.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/setup.rs:888), [setup.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/setup.rs:3062)); migration retirement ([migrate_corpus.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/migrate_corpus.rs:999)); index invalidation ([index.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/index.rs:179)); rollback populations ([rollback.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:758)); source retirement is re-adjudicated immediately before unlink ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4645)).
- Unguarded removals are confined to minted temporary message/snapshot cleanup or empty-directory pruning, not operator-owned recursive subjects ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4254), [rename.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rename.rs:990), [state.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:1054)).

`--force` is scoped to the four refusing rows’ declared populations; the two finalizers expose no invented force consent. Narration is outcome-filtered after removal.

No schema-manifest or schema-hash file changed in `20c18b74..4a862b96`: **M52’s zero schema-hash movement boundary is not violated.**

Bounds: source completeness only; no argv was driven. Test-only temporary-directory destructors were inspected but are not reachable CLI doors.