# M53 axis 3 source pass — destroying doors

Source reviewed at `84db75517669bbfe3f499e5268fd2e0674e04d88`, the fixed rc.17 source after all seven audit fixes. The worktree contains unrelated documentation/golden changes, but no modified production Rust source.

## Claims

**No source-grounded completeness lead.** I found no unregistered destroying door, no removal bypassing the applicable ownership/consent/displacement seam, and no source-grounded tier-1 path to exit-0 byte loss or repository harm.

Accordingly, there is no numbered defect claim or proposed wrong-behaviour reproduction to report. Confidence: **high from source inspection; not binary-driven**.

## M52 §A axis-3 dispositions

1. **A3-1 — CLOSED.** Milestone finalize now supplies its own area as `WorkArea::Milestone` to the shared teardown, collects that area’s displacement alongside all sub-task displacements, and does so in both chain and squash arms: `crates/cli/src/milestone.rs:5137-5167,5181-5193,5328-5340`. The milestone complement now walks `merged/`, treating only `merged/docs/<staged-id>.md` as jigc-written: `crates/engine/src/state.rs:465-495`. Its removal twin applies the same membership partition non-recursively: `crates/engine/src/state.rs:585-715`. Thus root, `merged/`, and `merged/docs/` foreign entries are moved or left standing, not recursively destroyed.

   Proposed regression: plant foreign files at the milestone root, `merged/top.txt`, `merged/docs/provenance.json`, and a directory under `merged/`; run both milestone-finalize strategies. Expected: exit 0, byte-identical survivors under `.jigc/displaced/<milestone>/`, with all moved pairs in `committed.displaced`.

2. **A3-2 — CLOSED.** Both finalizing doors call displacement and then `unwind_area`, not `remove_dir_all`: `crates/cli/src/task.rs:6196-6222,6274-6291`; milestone sub-task cleanup uses the same seam at `crates/cli/src/milestone.rs:5638-5664`. `unwind_area` removes only registry members and finishes with non-recursive `remove_dir`, so unmoved foreign bytes structurally prevent area removal: `crates/engine/src/state.rs:550-625`. Both `Foreign` and unwind-error results mint one `finalize.foreign-bytes` advisory: `crates/cli/src/task.rs:6274-6291,6331-6384`.

   Proposed regression: occupy `.jigc/displaced/<id>` so one or all renames fail, then finalize. Expected: exit 0 because the commit landed; every unmoved byte remains in its area, the area remains standing, and one `finalize.foreign-bytes` identifies it.

3. **A3-3 — CLOSED.** The milestone-boundary subject now expressly includes the milestone area in addition to sub-task areas: `crates/cli/src/milestone.rs:5137-5167,5181-5193,5328-5340`. The old test-subject omission is no longer reflected in production structure.

## M51 §A axis-3 dispositions

1. **C-1 class — CLOSED.** The complete six-door registry contains provision, milestone discard, uninstall, task discard, task finalize, and milestone finalize: `crates/cli/src/milestone.rs:3269-3345`. Refusing doors use foreign-byte guards before recursive removal; displacing doors use the safe unwind described above. The shared task/milestone writer rows, including `FINALIZE_MESSAGE_FILE` in both, are at `crates/engine/src/state.rs:163-178,222-229`.

2. **D-1 — CLOSED.** Task discard distinguishes milestone-owned tasks and routes them to milestone finalize: `crates/cli/src/task.rs:977-1024`. Uninstall renders the corresponding per-task milestone route: `crates/cli/src/setup.rs:3783-3804`.

3. **D-2 — CLOSED.** `cleanup_subtask_areas` aggregates actual removal success at `crates/cli/src/milestone.rs:5614-5683`; discard chooses its workbench acknowledgement from that outcome at `crates/cli/src/milestone.rs:4071-4088`.

4. **D-3 — CLOSED.** Staged-prose failures render paths through the repository-relative path seam before constructing the finding: `crates/cli/src/task.rs:790-797`.

5. **D-4 — CLOSED.** The unreadable-worktrees-root route correctly identifies a `read_dir` failure, expressly rejects the git diagnosis, and names `jigc uninstall --force`: `crates/cli/src/setup.rs:3742-3753`.

## M53 seam consistency

- The residual rule is centralized: a work unit exists only when registry member zero is a regular base-pin file, `crates/engine/src/state.rs:240-258,271-304`.
- Active-task listing filters through it: `crates/engine/src/state.rs:1306-1313`.
- By-id task resolution rejects a pin-less directory as a residual: `crates/cli/src/task.rs:1495-1511`.
- Milestone resolution and ownership use the same predicate: `crates/engine/src/milestone.rs:908-927,1006-1015`.
- Rename’s first-live-milestone scan also uses it: `crates/cli/src/rename.rs:1098-1105`.
- Uninstall still sees residual directories as teardown subjects rather than silently excluding them: it enumerates every child under `tasks/` and `milestones/`, then applies the foreign complement guard: `crates/cli/src/setup.rs:3286-3316`.
- `FINALIZE_MESSAGE_FILE` is a member of both writer rows: `crates/engine/src/state.rs:163-178,222-229`.
- `LeftoverShape` remains shape-complete and removal dispatches directories to `remove_dir_all`, every other present/fail-closed leaf shape to `remove_file`: `crates/cli/src/milestone.rs:3399-3414,3551-3568,2960-2978`.
- `--force` belongs only to refusing dispositions; both finalize rows are `Displace` with no consent flag: `crates/cli/src/milestone.rs:3276-3306,3326-3345`.

## Removal-site census

Production removals were read as follows:

- `cli/setup.rs:799,1999,2785,2875,2936`: owned hook/footprint cleanup; guarded uninstall; owned guide; verified-empty directories.
- `cli/milestone.rs:2971-2976,4527,5674`: classified provision leftover; probed worktree cleanup; consented milestone-discard `Take` arm.
- `cli/task.rs:618,4169,4555,6191`: guarded discard; owned temporary; compare-and-swap retirement; production-dead/test-only `AreaTeardown::None` fallback.
- `cli/adapter.rs:811`: exact injected-section ownership.
- `cli/migrate_corpus.rs:998`: persisted destination precedes source removal.
- `cli/rename.rs:990`: transaction-owned message temporary.
- `cli/rollback.rs:758`: compare-and-swap rollback ownership.
- `cli/combine.rs:284,295`: process-unique temporary index.
- `engine/state.rs:619,669,709,1054,1058,1952`: registry-bounded unwind or owned temporary/guard cleanup.
- `engine/milestone.rs:2228`: selective materialized-body removal.
- `engine/validate.rs:1848`, `engine/index.rs:179`, `engine/file_state.rs:2614`: scratch/cache/owner-artifact removal.

All remaining `remove_dir_all`/`remove_file` matches are test fixture destructors or RAII temporary-directory cleanup. I found no unguarded production removal of adopter bytes.

## Bounds

I read the requested M51/M52 ledgers and prior source pass, M53 settle §14 and verdict, all named axis files, all consumers of the new membership/displacement symbols, and every Rust filesystem-removal match. I performed no binary driving, permission experiment, write, or directory creation.

The diff from M52’s `e519e4eb` through fixed rc.17 contains **no schema manifest, schema file, or schema JSON movement**. The zero-schema-hash boundary holds.