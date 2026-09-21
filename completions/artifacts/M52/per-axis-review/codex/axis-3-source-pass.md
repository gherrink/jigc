<!-- M52 per-axis review (re-run) — axis 3 · posture — the CODEX SOURCE PASS, verbatim. Read against the repository at commit a3eb026b, 2026-09-21. It reads; it drives nothing. Every claim below is a LEAD until the reconciler drove it — see axis-3.md for the verdict on each. -->

# M52 Axis 3 source pass

Current source: `a3eb026b699ce9c23cb0201555838931b65bd2d2` (`1.0.0-rc.16`).

## Claims

1. **No new source-grounded completeness defect found.** All six production doors that remove paths they did not write are now in `DESTROYING_DOORS`; refusing doors guard foreign bytes before removal, while both finalizing doors displace them.

   - Evidence: `DESTROYING_DOORS` contains provision, milestone discard, uninstall, task discard, task finalize, and milestone finalize at `crates/cli/src/milestone.rs:3176-3282`. Its `Disposition::{Refuse,Narrate,Displace}` semantics are defined at `crates/cli/src/milestone.rs:3054-3085`; no current row uses `Narrate`.
   - Proposed reproduction: for each door, plant `notes.txt`, `docs/attachment.txt`, a foreign directory, and wrong-shape/symlink entries in every area it removes. Expected correct behaviour: the three refusing doors exit non-zero without `--force`, naming every foreign entry; task/milestone finalize move every entry byte-intact beneath `.jigc/displaced/<id>/` and report each `{from,to}` pair.
   - Confidence: high from source; not driven.

## M51 confirmed-row dispositions

1. **C-1 class — CLOSED.** The old five-door silent loss is structurally covered.

   - Task discard calls the foreign-byte guard before staged prose or `remove_dir_all`: `crates/cli/src/task.rs:598-620`. Its guard uses `foreign_area_paths` and refuses unless forced: `task.rs:847-877`.
   - Uninstall checks dirty worktrees, foreign-area complements, staged prose, and other workbench files before removing `.jigc`: `crates/cli/src/setup.rs:2727-2786`.
   - Task finalize calls `displace_foreign_area` before cleanup: `crates/cli/src/task.rs:5952-5982`.
   - Milestone finalize passes `SubtaskComplement::Displace` at both landed paths: `crates/cli/src/milestone.rs:5097-5118,5227-5247`; cleanup performs displacement before removal at `milestone.rs:5510-5543`.
   - Milestone discard guards both task and milestone areas under `milestone.foreign-bytes`: `milestone.rs:4049-4198`, then captures/narrates forced loss at `milestone.rs:3993-4009`.
   - The shared discriminator is the complement of `TASK_AREA_FILES`/`MILESTONE_AREA_FILES`, includes shape, does not follow symlinks, and fails closed: `crates/engine/src/state.rs:129-174,235-300`.
   - Regression repro: repeat M51’s `notes.txt` and `docs/attachment.txt` plants at all five formerly losing doors. Expected: refusal or displacement, never silent deletion.

2. **D-1 — CLOSED.** Task discard now detects milestone ownership where the finding is constructed and routes sub-tasks to `jigc milestone finalize`, explicitly explaining that task finalize refuses: `crates/cli/src/task.rs:977-1024`. Uninstall carries the same per-task ownership discrimination and concrete exits: `crates/cli/src/setup.rs:2751-2759,3783-3804`.

3. **D-2 — CLOSED.** `cleanup_subtask_areas` returns whether all areas disappeared (`crates/cli/src/milestone.rs:5507-5545`), and discard’s `workbench removed` wording is selected from the aggregate teardown outcome at `milestone.rs:4003-4018`. Repro: force worktree removal failure; expected warning and no `workbench removed`.

4. **D-3 — CLOSED.** The staged-prose fail-closed error converts the task docs path through `repo_relative` before constructing the finding: `crates/cli/src/task.rs:790-797`. Repro: make `docs/` unreadable outside the sandbox; expected `.jigc/tasks/<id>/docs`, not a host-absolute path.

5. **D-4 — CLOSED.** The unreadable-worktrees-root finding now identifies `read_dir`, denies that it is a git fault, and names `jigc uninstall --force`: `crates/cli/src/setup.rs:3736-3753`.

## Destroying-axis consistency

- `LeftoverAt` dispatch is shape-complete: directories use `remove_dir_all`; leaf, absent, and unreadable states use `remove_file`, preserving the real errno (`crates/cli/src/milestone.rs:2906-2927`). The provision path captures loss, removes, then outcome-filters narration (`milestone.rs:2867-2884`). I found no surviving M49-style `is_dir()` bypass.
- `--force` is attached only to the four refusing rows’ common consent (`milestone.rs:3172-3234`). The two `Displace` rows expose no consent (`milestone.rs:3236-3266`), so `--force` cannot silently widen finalize.
- `TASK_AREA_FILES` includes all fixed task-writer names; staged bodies require the writer-emittable `<type>:<slug>.md` form, not arbitrary `.md` (`crates/engine/src/state.rs:129-160,204-233`). Foreign directory, file, symlink, and wrong-shape entries remain in the destroying-door subject (`state.rs:244-300`).
- Writer rollback uses the same registry and non-recursive deletion, leaving a non-empty area intact (`state.rs:351-460`), closing the hook-created-file bypass adjacent to this axis.

## Removal-site census

Production removal sites and their preceding discipline:

- `cli/setup.rs:799`: standalone-hook ownership classifier; `:1999`: jigc-owned footprint record; `:2785`: four uninstall guards or explicit `--force`; `:2875`: guide ownership or force; `:2936`: verified-empty directories.
- `cli/milestone.rs:2920-2925`: classified/narrated provision leftover; `:4465`: probed worktree teardown; `:5538`: foreign complement already refused or displaced. The later `:7059` occurrence is test code.
- `cli/task.rs:618`: foreign and staged-prose guards; `:4056`: transaction-owned commit-message temporary; `:4442`: compare-and-swap retirement; `:5642`: verified-empty parent; `:5981`: foreign complement displaced first; `:6874` and later occurrences are test cleanup.
- `cli/adapter.rs:811`: removes only a file consisting exactly of setup’s injected section.
- `cli/migrate_corpus.rs:998`: destination persisted before relocation source removal.
- `cli/rename.rs:990`: transaction-owned message temporary.
- `cli/rollback.rs:758`: compare-and-swap proves current bytes equal this transaction’s post-image.
- `cli/combine.rs:284,295`: process-unique temporary index only.
- `engine/state.rs:415,420,424,454,456`: registry-bounded unwind; `:1552`: captured absent pre-image.
- `engine/milestone.rs:1892`: rebuild of the registry-owned `merged/docs` staging tree.
- `engine/validate.rs:1848`: process-unique scratch snapshot.
- `engine/index.rs:179`: derived cache invalidation.
- `engine/file_state.rs:2614`: owner-artifact removal after ownership planning.
- `engine/target_surface.rs:669` and the many `Drop`/test-module `remove_dir_all` sites are temporary fixture destructors, not CLI doors.

I found no unguarded production removal of adopter bytes outside an ownership, transaction, cache, temporary-file, refusal, narration, or displacement seam.

## M52 seams and bounds

I also traced the requested adjacent registries: `ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `PRE_DISPATCH_FAULTS`, `InProgress::ALL`, `RelocateRefusal::ALL`, `ENVELOPE_OWED_CODES`, `suppressed.door`/`workflow.verb-routed`, the shared fixed-identity predicate, and `STORE_EXIT_FLIPS`. Within this axis, the material contact is the writer-set complement and destroying-door dispositions. `schema-conformance.home-vacated` is the seventh exit flip at `crates/cli/src/render.rs:1072-1117`; `ENVELOPE_OWED_CODES` is enforced centrally at `render.rs:5415-5420,5461-5474`.

I performed no binary driving, permission-mode experiment, or write. Test-only destructors and external crates are outside the CLI-door census. The diff from M51 commit `35195f56` to HEAD showed **no schema-manifest or schema-file changes**: I see no violation of M52’s zero-schema-hash-movement boundary.