<!-- M52 per-axis review (re-run) — axis 4 · destroying doors — the CODEX SOURCE PASS, verbatim. Read against the repository at commit a3eb026b, 2026-09-21. It reads; it drives nothing. Every claim below is a LEAD until the reconciler drove it — see axis-4.md for the verdict on each. -->

## CLAIMS

No source-grounded completeness defect found. I found no omitted transaction door, uncaptured in-closure write, unconditional rollback remaining in the reviewed populations, or staged-path family missing its corresponding pre-image axis. Accordingly, there is no wrong-behaviour reproduction to report.

## M51 confirmed-row dispositions

1. **C1 — promotion rollback: CLOSED.**

   Promotion destinations are captured before `fs::copy`, including partial-copy failures, and their post-images are recorded immediately afterward ([task.rs:4200](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4200)). Retirements likewise enter the same family as explicit removed post-images ([task.rs:4427](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4427)). Failure calls the family’s CAS restore ([task.rs:5606](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5606)); it restores only when live bytes still equal jigc’s post-image, otherwise preserves the live file, parks the pre-image, and returns a conflict finding ([rollback.rs:719](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:719), [rollback.rs:738](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:738), [rollback.rs:763](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:763)).

2. **C2 — `RecordPreImage` unconditional restore: CLOSED.**

   Capture now constructs a `PreImageFamily` before the record write ([milestone.rs:960](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:960)); fresh-record creation records the post-image immediately after writing ([milestone.rs:796](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:796)). Rollback invokes CAS before restoring the scoped index entry ([milestone.rs:992](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:992)). The registry carries all five callers—create, add-task, add-from-spec, milestone discard, and sub-task discard—as one `FileCas` population ([rollback.rs:230](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:230)).

3. **C3 — `RecordFlipGuard::Drop` unconditional restore: CLOSED.**

   The guard now owns a `PreImageFamily` and a conflict sink ([milestone.rs:5334](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5334)); its named rollback delegates to CAS rather than rewriting the old string ([milestone.rs:5359](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5359)). The registry identifies both milestone-finalize commit models through the `fan-out-record-flip` `FileCas` row ([rollback.rs:257](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:257)).

4. **C4 — provision’s unacknowledged early `.gitignore` mutation: CLOSED.**

   `run_provision` now resolves schemas, reseeds, rejects an unknown milestone, reads the base, rejects a stale base, and reads the task list before calling `gitignore::ensure` ([milestone.rs:2683](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2683), [milestone.rs:2693](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2693), [milestone.rs:2705](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2705), [milestone.rs:2732](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2732)). A successful amendment is included in the returned acknowledgement ([milestone.rs:2741](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2741)).

## Consistent completeness reads

- `ROLLBACK_POPULATIONS` enumerates eleven populations with explicit `FileCas`, `MintedSet`, `DoorGuard`, or `Declared` discipline. It includes finalize configuration, promotion, retirement, record transactions, record flip, rename, staged author rollback, both minted-area classes, root relocation, and setup’s pre-write refusal ([rollback.rs:169](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:169), [rollback.rs:309](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:309), [rollback.rs:330](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:330), [rollback.rs:364](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:364), [rollback.rs:390](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:390)).

- The shared finalize closure captures owner-artifact, promotion, config-layer, milestone-record index, and config-worktree pre-images before promotion, retirement, ignore amendment, staging, or commit ([task.rs:3924](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3924), [task.rs:3941](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3941), [task.rs:3950](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3950), [task.rs:3958](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3958), [task.rs:3964](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3964), [task.rs:3970](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3970)). Its common failure arm restores all five index/worktree axes and carries conflicts ([task.rs:4067](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4067), [task.rs:4079](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4079), [task.rs:4090](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4090), [task.rs:4099](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4099)).

- `COMMITTING_DOORS` contains the ten hook-capable identities, including both milestone-finalize models and task discard ([invocation_log.rs:171](/Users/maurice/projects/gherrink-jigc/crates/cli/src/invocation_log.rs:171)). Setup remains deliberately outside because its install commit uses `--no-verify`; its candidate-set refusal precedes writes, represented by the setup `DoorGuard` row ([rollback.rs:390](/Users/maurice/projects/gherrink-jigc/crates/cli/src/rollback.rs:390)).

- All four production `gitignore::ensure` writers remain setup/adapter, milestone create, milestone provision, and shared finalize ([adapter.rs:1115](/Users/maurice/projects/gherrink-jigc/crates/cli/src/adapter.rs:1115), [milestone.rs:657](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:657), [milestone.rs:2732](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2732), [task.rs:3994](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3994)).

- `TASK_AREA_FILES` contains thirteen root entries, with a separate staged-doc rule; it includes the cross-area record commit message that rejected sub-task discard can leave behind ([state.rs:101](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:101), [state.rs:129](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:129), [state.rs:145](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:145)). Thus the destroying-door subject is explicitly its complement, rather than a hand-maintained deletion list.

- The newly mentioned non-transaction seams were inspected where they intersect this axis: `STORE_EXIT_FLIPS` has the seventh home-vacated member; `ENVELOPE_OWED_CODES` includes all four targeted `store.*`/fixed-identity codes ([render.rs:968](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:968), [render.rs:5415](/Users/maurice/projects/gherrink-jigc/crates/cli/src/render.rs:5415)). I found no route around these seams relevant to rollback.

## Schema-hash boundary and bounds

No zero-schema-hash boundary violation observed: compared with M51 commit `35195f56`, neither shipped schema YAML nor either schema manifest changed. `engine/src/schema.rs` changed to centralize the fixed-identity predicate, but that is implementation logic, not persisted schema shape or manifest hash.

Read-only source pass only: I did not build, run tests, drive the binary, mutate files, or create directories. `PRE_DISPATCH_FAULTS` is test-side rather than production-side; its phase/product completeness was inspected as context, not dynamically verified. Confidence in the no-lead conclusion: **high**.