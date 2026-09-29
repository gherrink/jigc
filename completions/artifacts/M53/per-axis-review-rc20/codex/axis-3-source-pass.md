<!-- The unseeded Codex source pass for AXIS 3, verbatim. Source read at `1.0.0-rc.20` (repo HEAD `4d3175c3`), 2026-09-27. It drove nothing. -->

## Axis 3 source pass

### Claims

No new source-grounded completeness claim.

I found no destroying door omitted from the current registry, no removal of user/foreign bytes that bypasses the shared guard/displacement seam, and no new F-10 amend-specific bypass. Therefore there is no proposed failing argv sequence to report.

### rc.20 baseline rows

1. **CLOSED — `(3, F-A)`**

   `task finalize` displacement now derives the rendering root from `jigc_root` via `workbench_home`, eliminating the caller-supplied standing-checkout root ([task.rs:6599](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6599), [task.rs:6605](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6605), [task.rs:6613](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6613)). Both milestone-finalize callers likewise pass `jigc_home` to the remaining teardown renderer ([milestone.rs:5986](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5986), [milestone.rs:6009](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:6009)).

   Re-drive argv: from a branch-attached linked worktree, finalize a task containing `.jigc/tasks/<id>/notes.txt`, in text and JSON, including a blocked displacement destination. Expected closure: every `from`, `to`, message, and route uses `.jigc/...`, with zero host-absolute paths.

2. **STILL-OPEN — `(3, F-B)`**, expected tier 3.

   `LeftoverShape::File` correctly has an empty `entries` set ([milestone.rs:3740](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3740)), but the forced-removal narration still constructs a `DoomedLine` from the leftover file’s own basename ([milestone.rs:6986](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:6986), [milestone.rs:6998](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:6998)) and prints that line beneath “discards work …” ([milestone.rs:7130](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:7130)). The source therefore still presents the file’s basename in the child-entry position.

   Datum/argv: place a plain file at `.jigc/worktrees/<sub-id>` and run `jigc milestone provision <milestone> --force`; expect the warning to list `<sub-id>` beneath the leftover-file heading. This is a surface inaccuracy, not loss.

### M51 confirmed rows

1. **C-1 — CLOSED across all five doors.**

   The registry now contains six destroying doors, including both formerly omitted task doors ([milestone.rs:3424](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3424)). `task discard`, milestone discard, and uninstall refuse over the complement unless `--force`; task and milestone finalize displace it ([milestone.rs:3311](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3311), [milestone.rs:3321](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3321), [milestone.rs:3336](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3336), [milestone.rs:3361](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3361), [milestone.rs:3379](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3379), [milestone.rs:3401](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3401)). Failed displacement cannot be followed by recursive destruction: registry-owned entries are unwound non-recursively and foreign bytes leave the area standing ([task.rs:6874](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:6874), [state.rs:609](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:609), [state.rs:647](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:647)).

2. **D-1 — CLOSED.**

   Uninstall distinguishes milestone sub-tasks before constructing the staged-prose route ([setup.rs:2951](/Users/maurice/projects/gherrink-jigc/crates/cli/src/setup.rs:2951), [setup.rs:2956](/Users/maurice/projects/gherrink-jigc/crates/cli/src/setup.rs:2956)); the task-discard family uses the same owning-milestone distinction.

3. **D-2 — CLOSED.**

   `cleanup_subtask_areas` returns whether every area was removed and marks failed removals false ([milestone.rs:5968](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5968), [milestone.rs:6031](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:6031)). The discard acknowledgement is consequently outcome-keyed rather than unconditional.

4. **D-3 — CLOSED.**

   Staged-prose enumeration renders its unreadable `docs/` path through `repo_relative(jigc_home, …)` ([task.rs:1259](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:1259)).

5. **D-4 — CLOSED.**

   The uninstall worktree-root failure is routed as a readability failure with the same `jigc uninstall --force` consent used by its sibling guards; it no longer diagnoses this `read_dir` failure as missing `git`.

### F-10 and M52 seams

The new `amend` marker is correctly included in `TASK_AREA_FILES` ([state.rs:186](/Users/maurice/projects/gherrink-jigc/crates/engine/src/state.rs:186)). Consequently it is treated as jigc-owned by foreign-byte classification and removed only through the same guarded/unwind paths.

The amend finalize arm reaches the same registered `TASK_FINALIZE_DOOR` and `AreaTeardown`; it merely selects `StagePolicy::Amend` ([task.rs:3393](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3393), [task.rs:3429](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3429)). Before that transaction it:

- refuses every staged index path via `finalize.amend-index-dirty` ([task.rs:2554](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2554));
- refuses every staged doc satisfying the actual `promote_destination` predicate via `finalize.amend-staged-doc` ([task.rs:2589](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2589), [task.rs:2610](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2610));
- stages nothing before `git commit --amend` ([task.rs:4695](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:4695)).

This closes the review HIGH’s worktree-divergence path at both the writer and finalize backstop; I found no alternative removal or promotion route around it.

The M52 destroying-door seam is internally consistent: `Disposition::{Refuse,Narrate,Displace}` is exhaustive, silence is not an arm ([milestone.rs:3188](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3188)), and the worktree-specific subset is explicitly four doors ([milestone.rs:3439](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3439)). `probe_leftover` uses `symlink_metadata`-based shape classification and fails closed for unreadable paths ([milestone.rs:3733](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3733)).

### Removal-site census and bounds

I read `CLAUDE.md`; M53 rc.20 §A in full; M51 §A and the prior Axis 3 source pass; M53 Addendum 3; the 2026-09-26/27 decisions; F-10 baseline, settlement, and review context; `milestone.rs`, `task.rs`, `setup.rs`—there is no `crates/cli/src/uninstall.rs`—and every `remove_file`, `remove_dir`, and `remove_dir_all` match in both crates.

Production removals fall into:

- destroying-door sinks guarded, narrated, or displaced as above;
- registry-keyed mint unwind and rollback;
- transaction-owned temporary commit-message/config artifacts;
- index/cache invalidation;
- migration/rename retirement with rollback capture.

The many remaining recursive removals are test-fixture `Drop` implementations. I found no unguarded production removal of foreign/user bytes.

No schema or schema-manifest file changed in `609da011..HEAD`; the M52 zero-schema-hash-movement boundary is not violated.

This was source inspection only: I did not build or drive the binary, write files, or create directories.