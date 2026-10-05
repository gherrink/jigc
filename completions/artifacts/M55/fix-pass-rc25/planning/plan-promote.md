# Fix-pass plan — the promote and copy-in family

Rows: `(R3, F7)` · `(R6, D-1)` · `(R6, D-7)` · `(R6, K-1)`. Planned against the source at
`jigc 1.0.0-rc.24`; today's behaviour driven on the installed registry build in throwaway rigs
(`dev/jigc-rig`, two-step eval, stdout only). Nothing in the repository was edited.

## 1 · The contract

> **A committing door (`task finalize`, `milestone finalize`) lands each promoted doc as a regular
> file at exactly its canonical home, and never replaces bytes there that the base pin's commit
> does not hold — a hand edit, an untracked or foreign occupant, a link — unless a blocking,
> routed finding names the path and nothing is committed; and under a `new: true` entry no path
> reaches the copy-in.**

Assertable form: for every promotion `P` at destination `D`, *either* the door exits non-zero with
one finding keyed `(code, D)` and `HEAD`, the index and the bytes at `D` are unchanged, *or* the
bytes at `D` before the door were (a) absent, or (b) a regular file whose content equals the blob
at the task's base pin, or (c) the content a recorded baseline says the task started from.

Source sentences: `design/reconciliation.md` → *What reconciliation does NOT do* (*"No silent
discard, ever … There is no door exemption from this sentence"*) and → *Conflict* (*"Discard is
always explicit, never the default, never silent"*); `design/storage.md` → *Placement* (*"the
general `plan_clobber_guard` already blocks any create-promote whose destination file already
exists"*); `design/finalize.md` → *4. Promote* (*"Copy (not move) the file from the working area
to its canonical path"*); `design/findings-channel.md` §4 (*"refuses before copy-in … cannot
overwrite an existing doc by either path"*).

## 2 · The rows

One guard function, one classifier arm, one promote call. No new finding code, no new store, no
working-area format change.

### (R3, F7) — the UNKNOWN arm consults the task

- **Site.** `crates/engine/src/file_state.rs` : `reconcile_committed`, the `None =>` arm.
  Both committing doors already feed it a base-pin seam (`PinnedBlob`): `cli/src/task.rs`
  `validate` (the task's pin) and `cli/src/milestone.rs` `milestone_boundary_gate` (the shared
  base).
- **Narrow fix.** `UNKNOWN` + `task_touched` + the pin carries a blob for the path + the on-disk
  bytes differ from it → the caller's **`reconciliation.conflict-block`** (existing code, existing
  presentation, existing route; the record is not advanced). Everything else in the arm is
  unchanged: untouched → adopt; touched and equal to the pin → adopt (advisory
  `file-state.baseline-adopt`); no blob at the pin (untracked at base, unreadable pin, the record
  door) → today's behaviour.
- **Carve-out, required.** The caller's path-keyed migration source (`ConflictBlock.keyed`) keeps
  today's UNKNOWN arm. Without it the sanctioned `jigc unmanage <source>` exit loops: the keyed
  route says *unmanage, then finalize again*, and the re-run would block on the same path.
  Pinned today by `pre_guard_repair_route::the_demoted_heading_lands_through_the_shipped_chain`.
- **The pin must be read in checked-out form.** `git_blob_at` returns the raw blob; under
  `core.autocrlf=true` the worktree holds CRLF and the blob LF (driven: 6 bytes on disk, 4 in the
  blob, 6 through `git cat-file --filters`). Compared raw, every first task in a CRLF clone would
  block. The seam's CLI side reads `git cat-file --filters <pin>:<path>`.
- **Class closed.** Both committing doors; placement and location doctypes; clone, `unmanage`
  before the task, `unmanage` after the block, cache deletion, a dropped key; the non-conformant
  hand edit (blocked before the *fix the file* advisory can precede an overwrite).
- **Class left.** A doc untracked at the base pin (no blob to compare — the tier-2 class of
  ruling 4); the linked-worktree sibling (fork `promote-2`); the record door (it splices in place,
  it does not promote).
- **Red tests.**
  - `crates/engine/src/file_state.rs` (unit): `unknown_and_touched_off_the_base_pin_is_a_conflict_block`
    · `unknown_and_touched_at_the_base_pin_still_baseline_adopts`
    · `unknown_and_touched_with_no_pin_blob_still_baseline_adopts`
    · `unknown_and_untouched_off_the_pin_still_baseline_adopts`
    · `unknown_touched_migration_source_keeps_its_unknown_arm`
    · `unknown_touched_off_pin_nonconformant_blocks_instead_of_advising`.
  - `crates/cli/tests/reconciliation_baseline_contrast.rs` (registered, `g_finalize`):
    `a_hand_edit_after_the_copy_in_blocks_with_no_baseline_and_survives` (exit 3, the code,
    `HEAD` unchanged, the hand line still on disk; placement `vision` and location `adr` arms)
    · `unmanage_after_the_block_does_not_switch_the_guard_off`
    · `a_fresh_clone_with_no_hand_edit_lands_its_first_task` and its `core.autocrlf=true` twin
    (zero-false-fire) · `reverting_the_edit_lands_the_task` (the route, followed).
  - `crates/cli/tests/milestone_promote_guards.rs` (**new**, register in `g_milestone`):
    `milestone_finalize_blocks_a_hand_edit_made_after_a_sub_task_copy_in_with_no_baseline`.
  - Should have caught it: `reconciliation_baseline_contrast::a_lost_file_state_baseline_turns_a_conflict_block_into_a_silent_merge`
    — it fixes the other order by design. Its arm B flips under fork `promote-1`.
  - Must stay green: `flow59_branch_and_pull::*` (L1), `pre_guard_repair_route::*` (the migration
    exit), `singleton_running_doc` (arm A).

### (R6, D-1) — the milestone planner runs the guard

- **Site.** `crates/engine/src/finalize.rs` : `plan_milestone_finalize` (never calls
  `plan_clobber_guard`); `crates/engine/src/milestone.rs` : `materialize` (`MaterializeOutcome`
  carries no provenance); `crates/cli/src/milestone.rs` : `run_milestone_finalize`.
- **Narrow fix.** `plan_clobber_guard` takes the provenance it reads as an argument instead of
  loading `provenance.json` from the area (the merged area has none, and `clear_staged_bodies`
  deliberately treats one there as foreign). `MaterializeOutcome` gains, in process only, each
  final address's provenance, contributing sub-task and sub-area address. `plan_milestone_finalize`
  takes the repo root and that map and runs the guard after `plan_promotions`. A `created` doc
  whose destination holds any entry → **`finalize.promote-clobber`** (existing code, keyed at the
  destination path), exit 3 through the door's existing `blocked()`; the record-flip guard restores
  the record, nothing is committed.
- **The milestone arm's message and route** (`clobber_finding` gains a `Unit::Milestone` arm):
  names the sub-task and its sub-area address, and routes at
  `jigc doc rename <sub-area address> --to <title> --task <sub-task>`, then
  `jigc milestone finalize <id>`. **It must not offer "adopt the occupant in its own task" as an
  exit before the boundary**: that commit moves `HEAD` and the milestone then blocks on
  `finalize.base-mismatch`. Adoption is named as the step *after* the milestone lands.
- **Class closed.** Every `created` promotion through `milestone finalize`: suffixed or not (V2),
  committed-clean, untracked conformant, untracked foreign (V8), staged-uncommitted (V5), both
  squash modes (V6), any doctype with a home, any pack (V7); the join→finalize window for a
  jigc-visible occupant (the decision is taken at the boundary).
- **Class left.** The join's suffix assigner still mints onto an occupied id and the join preview
  still prints a plain `← suffixed` line (the boundary refuses; a store-aware suffix reverses rule
  4's stated purity — 1.x); two `created` instances of a singleton in one fan-out (reasoned, not
  driven); the plan→promote window against a non-jigc writer (ruling 4).
- **Red tests.**
  - `crates/engine/src/finalize.rs` (unit): `milestone_plan_blocks_a_created_doc_over_an_occupied_home`
    · `milestone_plan_lets_an_edited_from_base_doc_re_promote` · `milestone_plan_passes_a_free_home`.
  - `crates/cli/tests/milestone_promote_guards.rs` (**new**, `g_milestone`):
    `a_suffix_landing_on_a_committed_doc_blocks_and_names_the_sub_task`
    · `a_suffix_landing_on_an_untracked_file_blocks_and_the_bytes_survive`
    · `an_untracked_file_at_an_unsuffixed_created_home_blocks`
    · `a_foreign_file_and_a_staged_file_at_the_home_block`
    · `both_squash_modes_block` · `a_blocked_boundary_leaves_the_record_active`
    · `the_route_it_prints_lands_the_milestone`
    · controls `a_free_suffix_still_lands`, `a_sub_task_update_of_a_committed_doc_still_lands`.
  - Should have caught it: `milestone::milestone_join_suffixes_a_created_collision_and_reports_the_decision`,
    `create_only_gate::fan_out_sub_tasks_still_join_with_a_suffix` (suffixed id always free), and
    the engine's `finalize_plan_blocks_a_created_doc_that_clobbers_a_committed_doc` (calls
    `plan_finalize` only).

### (R6, D-7) — the home entry is read without following links

- **Sites.** `engine/src/finalize.rs` : `plan_clobber_guard` (`is_file()`);
  `engine/src/state.rs` : `create_occupied` and `create` step 4 (`is_file()`);
  `cli/src/task.rs` : `promote` (`fs::copy` follows a link at the destination).
- **Narrow fix.** One shared no-follow predicate over a home path (`symlink_metadata`):
  *free · regular file · other*.
  1. Planner, both doors: a `created` doc over **any** entry, and **any** promotion over an entry
     that is not a regular file → **`finalize.promote-clobber`** with a shape arm (*"`<dest>` is a
     symbolic link / a directory — jigc lands a regular file at exactly its home"*), exit 3,
     repo-relative path.
  2. Sink, `promote`: re-reads the entry no-follow immediately before the copy and refuses the
     transaction on a non-regular one (the M51 `retire` pattern) — the planner is the routed
     refusal, the sink the backstop.
  3. Gate, `new: true`: occupancy is *any directory entry* → **`create.already-exists`** (the
     design's own subject: *"the doctype's home on disk"*).
  4. `create` under a plain entry is unchanged in what it copies in (a link to a readable doc is
     still copied in, so authored work is kept); the planner then refuses at finalize.
- **The link arm's route.** Not the existing *"bring it under management with `jigc migrate`"*:
  driven, `jigc migrate <link>` answers `migrate.source-untrackable`. The arm routes at
  `jigc doc rename <addr> --to <title> --task <id>` (driven: runs) or at replacing the link with
  the regular file it should be — the entry is the user's, jigc writes no links.
- **Class closed.** Dangling link (in-repo and out-of-repo target), link to a device, tracked
  dangling link with or without staged code (V3, V13), a live link at a copied-in home, a
  directory (now exit 3 with a repo-relative path — the absolute path of the D-1 V9 side datum is
  no longer reached this way), placement and location homes, both committing doors, the migrate
  landing; the rollback never sees a link it could remove (V7), because the promote never runs.
- **Class left.** `doc list`'s code-less exit 1 over one unreadable entry (a read seam, the
  record's L-3); `doc show` reading through a link; a symlinked **parent** directory (followed as
  today); `PreImage::capture`'s follow semantics (unreachable for a promote after this fix); a link
  that appears between the sink's check and its copy (ruling 4).
- **Red tests.**
  - `crates/cli/tests/promote_destination_shape.rs` (**new**, register in `g_finalize`,
    `#[cfg(unix)]`): `a_dangling_link_at_a_created_home_is_refused_and_nothing_is_written`
    · `a_link_out_of_the_repository_writes_nothing_outside_it`
    · `a_link_to_a_device_does_not_swallow_the_doc`
    · `a_tracked_dangling_link_with_staged_code_commits_nothing`
    · `a_live_link_at_a_copied_in_home_is_refused_not_written_through`
    · `a_directory_at_the_home_is_refused_with_a_repo_relative_path`
    · `a_placement_home_that_is_a_link_is_refused`
    · `the_milestone_boundary_refuses_a_link_at_a_promotion_home`
    · `a_refused_promote_leaves_the_link_standing` · `the_refusal_route_lands`.
  - `crates/cli/tests/create_only_gate.rs` (`g_doc`):
    `a_dangling_link_at_the_home_is_refused_by_both_doors_with_nothing_staged`.
  - `crates/engine/src/state.rs` (unit): extend `create_occupied_probes_the_home_on_disk_whatever_is_staged`
    with the link and directory shapes; `crates/engine/src/finalize.rs` (unit): the guard's shape arm.
  - Should have caught it: `create_only_gate::an_untracked_file_at_the_home_is_refused` (one entry
    shape only).

### (R6, K-1) — the one-probe fix only

- **Site.** `engine/src/state.rs` : `create_gated` (takes the gate entry, never reads `new`) →
  `create` step 4.
- **Narrow fix.** `create_gated` hands the entry's key to `create`; the probe `create` already
  performs decides both outcomes — under `new: true` an occupied home returns
  **`create.already-exists`** (the CLI's existing two-arm route) and nothing is staged, no role
  bound, no provenance written. The CLI pre-check's rank 0 stays as the ranked early refusal over
  the same predicate.
- **Class closed.** `doc create` and `doc author` under `new: true`: no path reaches the copy-in,
  with or without a racer.
- **Class left (ruling 4, tier 2 for 1.x).** The `write.title-ignored` probe/re-probe pair; the
  untracked copy-in under a plain entry; `task finalize`'s own plan→promote window.
- **Red tests.** `crates/engine/src/state.rs` (unit):
  `create_gated_under_a_new_entry_refuses_an_occupied_home_and_copies_nothing_in` (red today with
  no concurrency) · control `create_gated_under_a_plain_entry_still_copies_in`. Should have caught
  it: `create_only_gate::a_reused_title_is_refused_by_both_doors_with_nothing_staged` and
  `state::tests::create_gated_enforces_the_allows_create_gate` (neither calls the engine's create
  with a `new` entry over an occupied home).

## 3 · Next-step drives the fixer must run on the built binary

Driven today (rc.24), as the baseline each drive is compared against:

- conflict-block with a baseline present → `git checkout -- <doc>` → `task finalize` exits 0 and
  the task's prose is at `HEAD` — the route works from that state;
- `jigc doc rename idea:pair-idea --to "…" --task <sub>` from the main checkout re-slugs the
  sub-task's staged doc and the next `milestone join` shows no suffix;
- `jigc migrate <dangling link> --as adr` → exit 1 `migrate.source-untrackable`;
- under `new: true`, the `create.already-exists` route (a distinct `--title`) runs;
- a second clone has no `file-state.json` before or after its first write.

To run after the fix: see `next_step_drives` in the structured result.

## 4 · What must not move

- Frozen schema hashes and both `schema-manifest.yaml` files: untouched (no pack file changes).
- Pinned `--format json` keys and every `contract-version`: untouched — no key added, removed or
  renamed; the two codes used are existing members with their existing `(code, target)` shape.
- Compose goldens: untouched — no step, workflow or command-catalog text changes.
- `STORE_EXIT_FLIPS`: untouched — the store-scope twin (`detect_committed_store`) is not edited.
- `COMMITTING_DOORS`, `VERB_KINDS`, `DESTROYING_DOORS`, `BOUNDARY_DOORS`, `ROLLBACK_POPULATIONS`,
  `GATE_COVERAGE`, `CONSTRAINT_REQUIRED_TOKENS`: untouched.
- `AMBUSH_CONTRACTS`: the `finalize.promote-clobber` row names the `task finalize` door. If
  `stated_at_fence.rs` wants the milestone door named, that is one row in an existing registry
  for an already-owed code — no new owed statement, no golden movement.
- M55 behaviours: L1's absorb arm is not edited (the `--filters` read makes it correct in a CRLF
  checkout as well); the migration `unmanage` exit is carved out by path; untouched first-encounter
  adoption is unchanged; `--approve` and the in-place migration carve-out in the clobber guard are
  unchanged.
- Text pins that do move: `reconciliation_baseline_contrast` arm B (fork `promote-1`), and
  whatever `help_truth.rs` pins of the two finalize help texts.

## 5 · Docs

Listed in `docs_to_revise` in the structured result; each is revised in the commit that changes
its rule.

## 6 · Forks

Two. `promote-1`: the declared, pinned silent-merge order now blocks under the narrow fix.
`promote-2`: whether the linked-worktree sibling is in this pass. Detail in the structured result.
Not forks, and why: the store-aware suffix (ruling 2 — it reverses a declared rule and the narrow
guard is complete without it); a live link at a home (it never committed the doc — refusing is the
fix); K-1 (ruling 3).

## 7 · Commits

1. `fix(task): the base-pin blob is read in its checked-out form` — prerequisite; own red test
   (a pulled edit in a `core.autocrlf=true` checkout). Fold into 2 if not red today.
2. `fix(reconcile): a staged doc with no baseline is checked against the base pin` — (R3, F7).
3. `fix(milestone): the milestone planner runs the clobber guard` — (R6, D-1).
4. `fix(create): a create under a new: true entry never reaches the copy-in` — (R6, K-1).
5. `fix(finalize): a promote lands a regular file at its home or refuses` — (R6, D-7).

Each carries its tests, its design/help sentences and its `DECISIONS.md` line, and passes
`dev/gate` on its own.

## 8 · Left open

In `left_open` in the structured result.
