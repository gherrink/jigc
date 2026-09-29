# Axis 3 source pass — second M53 partial re-run

Reviewed production Rust at `1cef812d`—the rc.18 post-review source lineage (`3c71da87` plus all seven audit fixes through `16da362a`). I performed no writes, builds, or binary driving.

## Claims

1. **F-1 remains open: a symlink with a staged-document filename is simultaneously classified as staged prose and as foreign bytes.**

   Evidence: `staged_doc_ids` follows symlinks with `std::fs::metadata` and accepts the target when it is a file ([task.rs:701](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:701), [task.rs:713](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:713), [task.rs:725](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:725)). Conversely, `foreign_area_paths` uses `DirEntry::file_type`, does not follow the link, and treats only regular files as jigc-written ([state.rs:450](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:450), [state.rs:456](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:456), [state.rs:457](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:457)).

   Proposed reproduction: symlink `.jigc/tasks/<id>/docs/adr:linked.md` to an external regular Markdown file; run `jigc task discard <id>`, then `--force`. Expected current inconsistency: the refusal calls it foreign, while the success acknowledgement lists it among dropped staged edits. The external target remains intact.

   Confidence: **high, source-grounded; tier 3, not tier 1**.

2. **F-2 remains open: the residual message asserts “directory” although the residual predicate does not test the area’s shape.**

   Evidence: `carries_base_pin` examines only `<area>/<registry-member-0>` using `symlink_metadata` ([state.rs:249](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:249), [state.rs:302](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:302)); `residual_area_note` unconditionally says the area “is a directory” ([state.rs:321](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:321), [state.rs:323](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:323)).

   Proposed reproduction: make `.jigc/tasks/ghost` a symlink to an external directory, then run `jigc task discard ghost`. Expected current behavior: `finalize.no-task` falsely describes the symlink as a directory carrying no base pin. No target bytes are destroyed.

   Confidence: **high, source-grounded; tier 3, not tier 1**.

**No new completeness defect or tier-1 lead found.** In particular, I found no removal bypass that can cause exit-0 loss or repository harm through a destroying door.

## M53 rc.18 baseline dispositions

- **A3-1 — CLOSED.** The milestone complement walks `merged/` and recognizes only regular staged bodies under `merged/docs` as jigc-written ([state.rs:465](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:465), [state.rs:480](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:480), [state.rs:490](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:490)). Its removal twin is non-recursive and preserves foreign entries ([state.rs:632](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:632), [state.rs:652](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:652), [state.rs:681](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:681)).

- **A3-2 — CLOSED.** Finalization uses registry-bounded `unwind_area`; a non-empty area becomes `AreaUnwind::Foreign`, not recursively removed ([state.rs:585](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:585), [state.rs:623](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:623)). Both `Foreign` and unwind errors produce `finalize.foreign-bytes` and leave the area standing ([task.rs:6275](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6275), [task.rs:6283](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6283), [task.rs:6387](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6387)).

- **A3-3 — CLOSED.** Production now supplies both task and milestone `WorkArea` subjects to the same teardown structure; milestone membership is explicitly represented by `MILESTONE_AREA_FILES` and its `merged/` tree rule ([state.rs:231](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:231), [state.rs:240](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:240)).

- **F-1 — NOT CLOSED**, as claim 1 shows.

- **F-2 — NOT CLOSED**, as claim 2 shows.

- **Post-review operation-bearing-worktree HIGH — CLOSED.** `probe_leftover` admits clearance only when both the status entries and operation are empty ([milestone.rs:3699](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3699), [milestone.rs:3702](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3702), [milestone.rs:3730](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3730)). `held_operation` delegates to the shared `adjudicated_breach` seam and selects `OperationInProgress` ([milestone.rs:3475](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3475), [milestone.rs:3484](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3484)); that seam resolves and probes the supplied checkout ([repo.rs:770](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:770), [repo.rs:780](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:780)). Provision, discard, and uninstall share this classifier; milestone finalize separately preflights every committing worktree before durable writes ([milestone.rs:5160](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5160), [milestone.rs:5171](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5171)).

## M52 §A axis-3 dispositions

- **A3-1: CLOSED** — cited above.
- **A3-2: CLOSED** — cited above.
- **A3-3: CLOSED** — cited above.

## M51 §A axis-3 dispositions

- **C-1: CLOSED.** `DESTROYING_DOORS` contains all six relevant doors with refusal or displacement dispositions ([milestone.rs:3260](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3260), [milestone.rs:3363](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3363)). Task discard guards before `remove_dir_all` ([task.rs:607](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:607), [task.rs:627](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:627)); finalizers use displacement plus non-recursive unwind.

- **D-1: CLOSED.** Milestone ownership is derived only from live, base-pinned milestone areas ([milestone.rs:986](/Users/maurice/projects/gherrink-jigc/crates/engine/src/milestone.rs:986), [milestone.rs:1006](/Users/maurice/projects/gherrink-jigc/crates/engine/src/milestone.rs:1006)) and feeds the milestone-finalize route.

- **D-2: CLOSED.** Teardown tracks actual removal success rather than merely attempting removal ([milestone.rs:5953](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5953), [milestone.rs:5962](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5962)).

- **D-3: CLOSED.** Destroying-door paths render relative to `jigc_home`, including leftover holds ([milestone.rs:3496](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3496)).

- **D-4: CLOSED.** Unreadable/non-directory leftovers are distinct fail-closed shapes, and only `NotFound` means absence ([milestone.rs:3603](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3603), [milestone.rs:3620](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3620)).

## Consistency and bounds

The residual rule is consistently consumed by active-task listing, task/milestone resolution, milestone ownership, and rename’s first-live scan ([state.rs:1306](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:1306), [task.rs:1504](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:1504), [milestone.rs:908](/Users/maurice/projects/gherrink-jigc/crates/engine/src/milestone.rs:908), [rename.rs:1098](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rename.rs:1098)). `FINALIZE_MESSAGE_FILE` is present in the task writer row ([state.rs:159](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:159)); the paired milestone row was also inspected.

I inspected every production `remove_dir_all`, `remove_file`, and `remove_dir` match. They are guarded destroying-door removals; registry-bounded unwinds; transaction/CAS cleanup; exact owned hook/guide/temporary cleanup; persisted-before-source relocation; or cache/scratch invalidation. I found no unguarded production removal of adopter bytes.

No schema, schema-manifest, or schema-hash movement appears in the post-M53 source range. The zero-schema-hash boundary holds.