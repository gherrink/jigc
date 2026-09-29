<!-- M51 per-axis review — axis 4 · Codex source pass, verbatim · read against `jigc 1.0.0-rc.15` (commit 577a0099), 2026-09-16 -->

## Claims

1. **Task and milestone finalize rollback can overwrite a concurrent edit to a promoted document because promotion restoration is unconditional and has no compare-and-swap conflict finding.**

   - Evidence: `promote` captures only the destination’s pre-write bytes and then overwrites it (`crates/cli/src/task.rs:3486-3500`). On failure, `rollback_promotions` unconditionally rewrites those captured bytes, restores from `HEAD`, or deletes the destination (`crates/cli/src/task.rs:4863-4893`). By contrast, config-layer rollback explicitly compares the live bytes with jigc’s post-image and emits `finalize.rollback-conflict` on mismatch (`crates/cli/src/task.rs:4207-4239`). Both task finalize and the two milestone-finalize arms use this shared executor (`crates/cli/src/task.rs:3305-3349`).
   - Proposed reproduction: prepare a task that promotes `docs/specs/x.md`; install a rejecting `pre-commit` hook that rewrites that same destination to `CONCURRENT\n` and exits 1; run `jigc task finalize <id> --approve`. Expected wrong behaviour: the hook edit is replaced by the pre-finalize bytes (or the file is deleted if newly promoted), while output contains the commit-rejection frame but no `finalize.rollback-conflict`. The analogous `jigc milestone finalize <id> --approve` cells should behave the same.
   - Confidence: **High**.

2. **All record-only milestone commit doors can destroy a concurrent edit to the milestone record on stage/commit failure because `RecordPreImage` rollback restores worktree bytes unconditionally.**

   - Evidence: the pre-image stores only pre-write bytes and the prior index entry (`crates/cli/src/milestone.rs:774-803`). `rollback_record_pre_image` unconditionally writes those bytes or removes the file (`crates/cli/src/milestone.rs:816-825`), and `commit_record_transaction` invokes it on every stage/commit error (`crates/cli/src/milestone.rs:865-887`). Its callers cover create, append/add-task-family operations, discard, and task-discard record settlement (`crates/cli/src/milestone.rs:684,1369,1782,3261`).
   - Proposed reproduction: create a milestone, then install a rejecting hook which rewrites its committed `milestone-record` during the hook and exits 1. Run, for example, `jigc milestone add-task <milestone> "<intent>"`. Expected wrong behaviour: the hook’s edit is overwritten by the captured pre-operation record; the command reports rejection but emits no rollback-conflict finding or preserved copy.
   - Confidence: **High**.

3. **Milestone-finalize’s record flip has a second unconditional restore path that can overwrite a concurrent edit even when the shared finalize executor protects `.jigc/.gitignore` and `.jigc/version`.**

   - Evidence: `RecordFlipGuard` retains only the pre-flip string and, whenever still armed, writes it unconditionally in `Drop` (`crates/cli/src/milestone.rs:4430-4454`). The guard is armed after changing the record to `joined` (`crates/cli/src/milestone.rs:4495-4513`). Its documentation explicitly limits it to the worktree axis; the shared executor’s fifth axis covers only the index entry (`crates/cli/src/milestone.rs:4423-4429`, `crates/cli/src/task.rs:3380-3386`).
   - Proposed reproduction: prepare a finalizable milestone; install a rejecting hook that edits the milestone record and exits 1; run `jigc milestone finalize <id> --approve` in either squash mode. Expected wrong behaviour: guard drop restores the old `active` record over the hook’s edit, with no `finalize.rollback-conflict`.
   - Confidence: **High**.

4. **`milestone provision` can amend `.jigc/.gitignore` and then fail without acknowledging that surviving write.**

   - Evidence: `run_provision` calls `gitignore::ensure` before schema loading, cache reseeding, the unknown-milestone check, stale-base validation, task-list loading, or worktree creation (`crates/cli/src/milestone.rs:2158-2197`). The acknowledgement is constructed only after successful provisioning (`crates/cli/src/milestone.rs:2197-2205`). The registry explicitly describes provision as a non-committing writer whose acknowledgement is its only disclosure channel (`crates/cli/src/milestone.rs:2163-2168`; `crates/cli/src/gitignore.rs:82-96`).
   - Proposed reproduction: in a configured repository, remove one canonical entry such as `worktrees/` from `.jigc/.gitignore`, then run `jigc milestone provision nonexistent`. Expected wrong behaviour: the command fails with `milestone.unknown`, but `.jigc/.gitignore` remains amended and no finding/ack says that the worktree changed.
   - Confidence: **High**.

## Consistent reads

- Read `CLAUDE.md`, `invocation_log.rs`, the finalize planner/executor and rollback helpers in `task.rs` and `engine/finalize.rs`, all `git_commit_capture` callers, milestone record transactions and join/finalize paths, all four production `gitignore::ensure` sites, and setup’s candidate-path and install-commit machinery.
- `COMMITTING_DOORS` contains all ten hook-capable identities and distinguishes both milestone-finalize commit models; setup is expressly excluded because its install commit uses `--no-verify` (`crates/cli/src/invocation_log.rs:121-172`).
- The four `gitignore::ensure` call sites are fenced by `IGNORE_DOORS`: setup, shared finalize, milestone create, and milestone provision (`crates/cli/src/gitignore.rs:82-125`).
- Finalize captures promotion, owner-artifact, config-index, milestone-record-index, and config-worktree pre-images before in-closure failure points (`crates/cli/src/task.rs:3220-3266`).
- Config worktree restoration correctly uses compare-and-swap and emits `finalize.rollback-conflict` (`crates/cli/src/task.rs:4192-4241`).
- Retirement rollback does not overwrite a concurrently recreated retirement target: it restores captured bytes only while the path remains absent (`crates/cli/src/task.rs:4914-4923`).
- Setup stages and commits only its settled candidate paths and reports stage/commit refusals rather than silently treating them as success (`crates/cli/src/setup.rs:2312-2436`).