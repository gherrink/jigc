# Blocking refusal sites, new or changed since `jigc 1.0.0-rc.24`

*The count the human's ruling of 2026-10-11 on which families the route catalog takes in was put on ([DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-12 — Which families the route catalog takes in*). Made on 2026-10-11 by a read-only agent of the orchestrating session and committed as it was handed over: none of its numbers is changed. The suite reads its table of §2 (`route_catalog::a_scheduled_site_is_one_the_count_holds_unchanged`).*

A count, not a recommendation. Every `blocking` row of `crates/cli/tests/route_catalog/sites.tsv` (222 rows) is classed against the previous release's production source.

- **Previous release:** `91834b5e011de2c36e2be2b79e96c0b9f60a803c` (`chore(release): prepare release jigc v1.0.0-rc.24`).
- **Read at:** `ae5454bf76c613b0bcbf6dadd770b9f780d58b24` (branch `fix/rc24-route-catalog`), the commit `HEAD` named when the count began. Every byte was read through `git show <sha>:<path>`, so edits in the working tree while the count ran are not in it; the working tree's `sites.tsv` was byte-identical to that commit's.
- **Range:** 171 commits, **62** of which touch `crates/cli/src` or `crates/engine/src` (`git log 91834b5e..ae5454bf -- crates/cli/src crates/engine/src`; the two directories are everything `crates/*/src` holds). rc.24 had 211 blocking sites by the same scan; the tree now has 222.

## How it was counted

1. **The scan was ported and run on both revisions.** `crates/cli/tests/support/refusal_sites.rs` and the reader under it (`rust_source.rs`) were ported to a throwaway Python script. Run on the read commit, the port reproduces the committed `sites.tsv` **byte for byte** (all 261 rows). Run on rc.24's tree it yields that release's site list — 247 rows, 211 blocking — which no file at rc.24 holds. So an old site has the same identity, the same enclosing function and the same `route` / `route-text` reading as a row has now.
2. **Pairing.** A row is paired with the rc.24 site of the same `file`, `function` and `code`, by ordinal. One group changed its count (`write_install_span` constructed `setup.inject-allowlist` once at rc.24 and three times now); there the construction with identical form and route was paired and the other two are `new-site` — confirmed by reading.
3. **The classes, as applied.**
   - `new-code` — the code occurs in no file under `crates/cli/src` or `crates/engine/src` at rc.24, neither quoted nor as a bare word. A `dynamic` row has no code and is judged by its function (classes 2–5).
   - `new-site` — the code existed, and the row has no rc.24 counterpart: the function is absent from that file, or it exists and constructed that code fewer times.
   - `route-changed` — paired, and the route's kind or its `route-text` differs between the two scans. No row changed kind alone.
   - `function-changed` / `unchanged` — paired, route kind and text equal; the function's bytes, from the line of its `fn` to its closing brace (signature included, the doc comment above it not), differ or are identical. The function is the one that **holds the site** at each revision, found by brace matching over a view with comments and literals blanked.
   - A route of kind `variable` is built outside the route expression, so its text is in neither scan: such a row is classed 4 or 5 on the function body alone, and its clause says so. **53** blocking rows are `variable` (42 of them `unchanged`).
4. **Cross-check over all paired rows.** For each of the 210 paired rows, `git diff -U0` between the two commits was asked whether any hunk touches the function's line range. It agrees with the script on all 210 (36 touched, 174 not).

## 1. Family × class

| family | new-code | new-site | route-changed | function-changed | unchanged | total | classes 1–3 | blocking rows with a route |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `task finalize` | 2 | 1 | 1 | 2 | 52 | 58 | 4 | 45 |
| `milestone` | 1 | 1 | 1 | 1 | 27 | 31 | 3 | 31 |
| `unassigned` | 0 | 2 | 1 | 0 | 21 | 24 | 3 | 22 |
| `doc write` | 0 | 0 | 0 | 2 | 21 | 23 | 0 | 23 |
| `setup` | 1 | 3 | 0 | 18 | 1 | 23 | 4 | 23 |
| `config` | 0 | 0 | 1 | 0 | 18 | 19 | 1 | 19 |
| `uninstall` | 0 | 0 | 1 | 8 | 7 | 16 | 1 | 16 |
| `start` | 0 | 0 | 0 | 0 | 10 | 10 | 0 | 10 |
| `doc read` | 1 | 0 | 0 | 0 | 4 | 5 | 1 | 5 |
| `task discard` | 0 | 0 | 0 | 0 | 4 | 4 | 0 | 4 |
| `ingest` | 0 | 0 | 0 | 0 | 3 | 3 | 0 | 3 |
| `migrate` | 0 | 0 | 0 | 0 | 2 | 2 | 0 | 2 |
| `task bind` | 0 | 0 | 0 | 0 | 2 | 2 | 0 | 2 |
| `validate` | 0 | 0 | 0 | 0 | 2 | 2 | 0 | 2 |
| **total** | **5** | **7** | **5** | **31** | **174** | **222** | **17** | **207** |

*A route* = the `route` column is not `none`. `unassigned` is the family of the 24 `dynamic` rows — sites whose code is handed in.

**Three things the table does not show, read by hand:**

- The three `unassigned` rows in classes 1–3 all mint `setup.*` codes: `merged_through_links` and `replaced_path_refusal` (both new, both called from `install`, both minting codes that existed) and `setup_dirty_install_finding` (`setup.dirty-install-path`).
- Three `function-changed` rows with a `variable` route have a route that **did change inside the function**, which the scan cannot read: `leftover_finding` (`milestone`), `dirty_worktree_finding` in `setup.rs` (`uninstall`) and `clobber_finding` (`task finalize`). Counted as class 3 they would make classes 1–3: `milestone` 4, `uninstall` 2, `task finalize` 5, total 20. The table keeps them in class 4, as the rule for a route that cannot be isolated says.
- All eight `function-changed` rows with a `variable` route were read; the other five are annotated in the list.

## 2. Every blocking row

Grouped by family in the table's order; within a family by class, then by id. `rc.24` in a clause is the previous release.

| site (`file::function::code#ordinal`) | family | class | why |
|---|---|---|---|
| `crates/cli/src/task.rs::linked_worktree_doc_finding::finalize.linked-worktree-doc#1` | task finalize | new-code | code occurs nowhere in production source at rc.24; function is new too |
| `crates/cli/src/task.rs::linked_worktree_mint_finding::finalize.linked-worktree-doc#1` | task finalize | new-code | code occurs nowhere in production source at rc.24; function is new too |
| `crates/engine/src/file_state.rs::unanswered_pin_finding::reconciliation.conflict-block#1` | task finalize | new-site | function absent from this file at rc.24; code was constructed at rc.24 in crates/engine/src/file_state.rs::conflict_block_finding |
| `crates/cli/src/task.rs::stage_failed_finding::finalize.stage-failed#1` | task finalize | route-changed | route text differs (kind mechanical both sides) |
| `crates/engine/src/finalize.rs::carried_staged_finding::finalize.carried-staged#1` | task finalize | function-changed | construction identical; the function differs elsewhere |
| `crates/engine/src/finalize.rs::clobber_finding::finalize.promote-clobber#1` | task finalize | function-changed | construction identical; the function differs elsewhere; route is built elsewhere (`variable`) — not isolated, judged on the function body — by hand: **the route changed** — message and route come from a `match` that gained five arms (sub-task, held, fixed, contested, staged), each built by a new helper; the two old arms' text is untouched |
| `crates/cli/src/doc.rs::id_from_enum_block::schema-conformance.field-value-conformant#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/doc.rs::trailer_key_refusal::schema-conformance.field-value-conformant#1` | task finalize | unchanged | function byte-identical |
| `crates/cli/src/orphan.rs::home_vacated_finding::schema-conformance.home-vacated#1` | task finalize | unchanged | function byte-identical |
| `crates/cli/src/orphan.rs::orphaned_instance_finding::schema-conformance.orphaned-instance#1` | task finalize | unchanged | function byte-identical |
| `crates/cli/src/render.rs::validation_store::reconciliation.rename#1` | task finalize | unchanged | enclosing item byte-identical — the site stands in no function: the reader names the last `fn` before it; compared the top-level item that holds it (`pub const STORE_EXIT_FLIPS: &[StoreExitFlip] = &[`) |
| `crates/cli/src/render.rs::validation_store::schema-conformance.schema-version-ahead#1` | task finalize | unchanged | enclosing item byte-identical — the site stands in no function: the reader names the last `fn` before it; compared the top-level item that holds it (`pub const STORE_EXIT_FLIPS: &[StoreExitFlip] = &[`) |
| `crates/cli/src/render.rs::validation_store::schema-conformance.schema-version-current#1` | task finalize | unchanged | enclosing item byte-identical — the site stands in no function: the reader names the last `fn` before it; compared the top-level item that holds it (`pub const STORE_EXIT_FLIPS: &[StoreExitFlip] = &[`) |
| `crates/cli/src/start.rs::head_shape_refusal::amend.head-shape#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/task.rs::amend_index_dirty_finding::finalize.amend-index-dirty#1` | task finalize | unchanged | function byte-identical |
| `crates/cli/src/task.rs::amend_staged_doc_finding::finalize.amend-staged-doc#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/task.rs::nothing_staged_finding::finalize.nothing-staged#1` | task finalize | unchanged | function byte-identical |
| `crates/cli/src/task.rs::retire_untrackable_finding::finalize.retire-untrackable#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/file_state.rs::conflict_block_finding::reconciliation.conflict-block#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/file_state.rs::conformance_block_finding::reconciliation.conformance-block#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/file_state.rs::drift_store_finding::file-state.hash-matches#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/file_state.rs::rename_strong_finding::reconciliation.rename#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/file_state.rs::rename_weak_finding::reconciliation.rename#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/finalize.rs::amend_base_mismatch_finding::finalize.base-mismatch#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/finalize.rs::base_mismatch_finding::finalize.base-mismatch#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/finalize.rs::base_overlap_finding::finalize.base-mismatch#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/finalize.rs::empty_commit_finding::finalize.empty-commit#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/finalize.rs::migration_no_replacement_finding::finalize.migration-no-replacement#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/finalize.rs::no_such_task_finding::finalize.no-task#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/finalize.rs::promote_io_finding::finalize.promote-io#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/finalize.rs::provenance_io_finding::finalize.provenance-io#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/finalize.rs::render_io_finding::finalize.render-io#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/finalize.rs::residual_task_area_finding::finalize.no-task#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/finalize.rs::source_path_io_finding::finalize.source-path-io#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/finalize.rs::task_missing_finding::finalize.no-task#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/index.rs::dangling::schema-conformance.ref-resolves#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::sub_task_finalize_finding::finalize.milestone-sub-task#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/parse.rs::ceiling_violations::conformance.slot-heading-depth#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::ceiling_violations::conformance.slot-setext-heading#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::parse_item_slots::conformance.item-slot-delimiter-shadowed#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::parse_item_slots::conformance.item-slot-label-missing#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::parse_items::conformance.item-anchor-duplicate#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::parse_items::conformance.item-anchor-malformed#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::parse_items::conformance.item-heading-unanchored#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::parse_sections::conformance.header-not-first#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::parse_sections::conformance.section-missing#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::parse_sections::conformance.section-renamed#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::read_field_block_str::conformance.duplicate-field#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/parse.rs::read_field_block_str::conformance.malformed-field-block#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::read_field_block_str::conformance.unknown-field#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/parse.rs::read_marked_field_group::conformance.orphaned-sentinel#1` | task finalize | unchanged | function byte-identical; no route |
| `crates/engine/src/state.rs::collision_finding::task.serial-collision#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/state.rs::io_finding::task.working-area-io#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/state.rs::residual_collision_finding::task.serial-collision#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/target_surface.rs::scalar::doc-code.multi-valued-anchor#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/validate.rs::owner_artifact_present::owner-artifact.present#1` | task finalize | unchanged | function byte-identical |
| `crates/engine/src/validate.rs::version_ahead_break::schema-conformance.schema-version-ahead#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/validate.rs::version_currency_break::schema-conformance.schema-version-current#1` | task finalize | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/milestone.rs::unlanded_work_finding::milestone.unlanded-work#1` | milestone | new-code | code occurs nowhere in production source at rc.24; function is new too |
| `crates/engine/src/milestone.rs::record_home_refusal::milestone.record-exists#1` | milestone | new-site | function absent from this file at rc.24; code was constructed at rc.24 in crates/engine/src/milestone.rs::record_exists_finding; rc.24's construction of it in `record_exists_finding` is gone (that fn still exists and no longer constructs it) |
| `crates/cli/src/milestone.rs::dirty_worktree_finding::milestone.dirty-worktree#1` | milestone | route-changed | route text differs (kind prose both sides) |
| `crates/cli/src/milestone.rs::leftover_finding::milestone.leftover-holds-work#1` | milestone | function-changed | construction identical; the function differs elsewhere; route is built elsewhere (`variable`) — not isolated, judged on the function body — by hand: **the route changed** — a clause (`ANCHOR_CLAUSE`) is appended to it when a held path is anchored |
| `crates/cli/src/combine.rs::collision_finding::combine.code-collision#1` | milestone | unchanged | function byte-identical |
| `crates/cli/src/milestone.rs::foreign_bytes_finding::milestone.foreign-bytes#1` | milestone | unchanged | function byte-identical |
| `crates/cli/src/milestone.rs::mint_foreign_bytes_finding::milestone.foreign-bytes#1` | milestone | unchanged | function byte-identical |
| `crates/cli/src/milestone.rs::provision_failed_finding::milestone.provision-failed#1` | milestone | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/milestone.rs::subtask_staged_prose_finding::milestone.staged-prose#1` | milestone | unchanged | function byte-identical |
| `crates/cli/src/milestone.rs::unverified_foreign_finding::milestone.foreign-bytes#1` | milestone | unchanged | function byte-identical |
| `crates/cli/src/milestone.rs::unverified_subtask_prose_finding::milestone.staged-prose#1` | milestone | unchanged | function byte-identical |
| `crates/cli/src/milestone.rs::unwind_failed_finding::milestone.unwind-failed#1` | milestone | unchanged | function byte-identical |
| `crates/cli/src/milestone.rs::zero_contribution_finding::milestone.zero-contribution#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::collision_finding::milestone.serial-collision#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::cross_group_collision_finding::join.same-doc-clash#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::io_finding::milestone.area-io#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::isolation_finding::join.area-isolation#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::missing_provenance_finding::join.missing-provenance#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::missing_schema_finding::join.unknown-type#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::no_criteria_finding::milestone.no-criteria#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::read_back_finding::milestone.record-read-back#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::record_flip_finding::milestone.record-flip#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::residual_collision_finding::milestone.serial-collision#1` | milestone | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/milestone.rs::residual_milestone_area_finding::milestone.unknown#1` | milestone | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/milestone.rs::same_doc_clash_finding::join.same-doc-clash#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::settled_sub_task_finding::milestone.terminal#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::splice_finding::join.self-ref-rewrite#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::stale_base_finding::milestone.stale-base#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::sub_task_collision_finding::milestone.sub-task-collision#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::terminal_milestone_finding::milestone.terminal#1` | milestone | unchanged | function byte-identical |
| `crates/engine/src/milestone.rs::unknown_milestone_finding::milestone.unknown#1` | milestone | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/setup.rs::merged_through_links::dynamic#1` | unassigned | new-site | function absent from this file at rc.24; code written as `code` — by hand: called from `install`; mints one of four existing codes — `setup.inject-reference`, `setup.inject-allowlist`, `setup.secrets-gitignore`, `setup.init-project-layer` |
| `crates/cli/src/setup.rs::replaced_path_refusal::dynamic#1` | unassigned | new-site | function absent from this file at rc.24; code written as `code` — by hand: called from `install`; mints the existing `setup.*` code of the install member it refuses over |
| `crates/engine/src/finalize.rs::setup_dirty_install_finding::dynamic#1` | unassigned | route-changed | route text differs (kind human both sides); code written as `CarryoverBoundary::Setup.code()` — by hand: the code is `setup.dirty-install-path`; one literal route at rc.24, now a route chosen among several arms (a `HEAD` or none, ignored paths, index-flagged paths) |
| `crates/cli/src/author.rs::reject::dynamic#1` | unassigned | unchanged | function byte-identical; no route |
| `crates/cli/src/doc_code_probe/mod.rs::dangling::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/ingest.rs::near_miss_finding::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/locate.rs::locate_finding::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/locate.rs::locate_finding::dynamic#2` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/migrate_corpus.rs::blocked_finding::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/relocate.rs::refuse::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/rename.rs::refuse::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/render.rs::commit_rejection_finding::dynamic#1` | unassigned | unchanged | function byte-identical |
| `crates/cli/src/repo.rs::finding_at::dynamic#1` | unassigned | unchanged | function byte-identical |
| `crates/cli/src/rollback.rs::rollback_conflict_finding::dynamic#1` | unassigned | unchanged | function byte-identical |
| `crates/engine/src/cascade.rs::into_finding::dynamic#1` | unassigned | unchanged | function byte-identical — fn name declared 2× now / 2× at rc.24 in this file — compared the one holding the site |
| `crates/engine/src/cascade.rs::into_finding::dynamic#2` | unassigned | unchanged | function byte-identical — fn name declared 2× now / 2× at rc.24 in this file — compared the one holding the site |
| `crates/engine/src/compose.rs::blocking_workflow_refs::dynamic#1` | unassigned | unchanged | function byte-identical |
| `crates/engine/src/finding.rs::block::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/finding.rs::blocking::dynamic#1` | unassigned | unchanged | function byte-identical; no route |
| `crates/engine/src/milestone.rs::store_block::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/override_default.rs::delta_block::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/store.rs::block::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/validate.rs::blocking_conformance::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/write.rs::blocking_write::dynamic#1` | unassigned | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/doc.rs::run_retitle_item::write.machine-maintained#1` | doc write | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/doc.rs::title_ignored_refusal::write.title-ignored#1` | doc write | function-changed | construction identical; the function differs elsewhere; route is built elsewhere (`variable`) — not isolated, judged on the function body — by hand: the only change is one more argument (`None`) to the callee that builds the route, `distinct_identity_route` |
| `crates/cli/src/doc.rs::barrier_block::write.area-barrier#1` | doc write | unchanged | function byte-identical |
| `crates/cli/src/doc.rs::committed_reslug_refusal::write.identity-change#1` | doc write | unchanged | function byte-identical |
| `crates/cli/src/doc.rs::fixed_identity_refusal::write.identity-change#1` | doc write | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/doc.rs::fixed_title_refusal::write.title-ignored#1` | doc write | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/doc.rs::id_from_field_guard::write.id-from-field#1` | doc write | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/doc.rs::identity_divergence_refusal::write.identity-change#1` | doc write | unchanged | function byte-identical |
| `crates/cli/src/doc.rs::machine_maintained_field_guard::write.machine-maintained-field#1` | doc write | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/doc.rs::machine_maintained_guard::write.machine-maintained#1` | doc write | unchanged | function byte-identical |
| `crates/cli/src/doc.rs::occupied_destination_refusal::write.already-present#1` | doc write | unchanged | function byte-identical |
| `crates/cli/src/doc.rs::retitle_id_from_refusal::write.identity-change#1` | doc write | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/doc.rs::slug_override_enum_refusal::write.identity-change#1` | doc write | unchanged | function byte-identical |
| `crates/cli/src/task.rs::reject_slug_head_over_name_ceiling::write.slug-name-ceiling#1` | doc write | unchanged | function byte-identical |
| `crates/cli/src/task.rs::reject_slug_over_name_ceiling::write.slug-name-ceiling#1` | doc write | unchanged | function byte-identical |
| `crates/engine/src/state.rs::already_exists_finding::create.already-exists#1` | doc write | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/state.rs::empty_title_finding::create.empty-title#1` | doc write | unchanged | function byte-identical |
| `crates/engine/src/state.rs::gate_blocked_finding::create.gate-blocked#1` | doc write | unchanged | function byte-identical |
| `crates/engine/src/state.rs::instance_collision_finding::create.serial-collision#1` | doc write | unchanged | function byte-identical |
| `crates/engine/src/state.rs::unknown_doctype_finding::create.unknown-doctype#1` | doc write | unchanged | function byte-identical |
| `crates/engine/src/state.rs::unslugable_title_finding::write.unslugable-title#1` | doc write | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/engine/src/write.rs::unset_eligibility_finding::write.unset-ineligible#1` | doc write | unchanged | function byte-identical |
| `crates/engine/src/write.rs::unset_eligibility_finding::write.unset-ineligible#2` | doc write | unchanged | function byte-identical |
| `crates/engine/src/finalize.rs::setup_unasked_install_finding::setup.unverified-install-path#1` | setup | new-code | code occurs nowhere in production source at rc.24; function is new too |
| `crates/cli/src/setup.rs::hook_link_refusal::setup.install-hook#1` | setup | new-site | function absent from this file at rc.24; code was constructed at rc.24 in crates/cli/src/setup.rs::write_install_span |
| `crates/cli/src/setup.rs::write_install_span::setup.inject-allowlist#1` | setup | new-site | function constructed this code 1× at rc.24, 3× now — this one has no counterpart; code was constructed at rc.24 in crates/cli/src/setup.rs::install, crates/cli/src/setup.rs::write_install_span |
| `crates/cli/src/setup.rs::write_install_span::setup.inject-allowlist#3` | setup | new-site | function constructed this code 1× at rc.24, 3× now — this one has no counterpart; code was constructed at rc.24 in crates/cli/src/setup.rs::install, crates/cli/src/setup.rs::write_install_span |
| `crates/cli/src/setup.rs::commit_install::setup.install-commit#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::finding::setup.install-commit#1` | setup | function-changed | construction identical; the function differs elsewhere; route is built elsewhere (`variable`) — not isolated, judged on the function body — by hand: this construction and its route are untouched; the change is in another arm, which returns `setup_dirty_install_finding` |
| `crates/cli/src/setup.rs::install::setup.guide-target#1` | setup | function-changed | construction identical; the function differs elsewhere; route is built elsewhere (`variable`) — not isolated, judged on the function body — by hand: the route is the validator's reason (`reason.to_string()`), the lines around the site are untouched |
| `crates/cli/src/setup.rs::install::setup.inject-allowlist#1` | setup | function-changed | construction identical; the function differs elsewhere; route is built elsewhere (`variable`) — not isolated, judged on the function body — by hand: the route is a local built in the four lines above the site, untouched |
| `crates/cli/src/setup.rs::install::setup.profile-incomplete#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::install::setup.spawn-template#1` | setup | function-changed | construction identical; the function differs elsewhere; route is built elsewhere (`variable`) — not isolated, judged on the function body — by hand: the route is the validator's reason (`reason.to_string()`), the lines around the site are untouched |
| `crates/cli/src/setup.rs::write_install_span::setup.compose-marker#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.init-project-layer#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.inject-allowlist#2` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.inject-deny#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.inject-hook#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.inject-reference#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.install-hook#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.install-hook#2` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.secrets-gitignore#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.version-stamp#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.write-bootstrap#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::write_install_span::setup.write-guide#1` | setup | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::run::setup.profile-load#1` | setup | unchanged | function byte-identical |
| `crates/cli/src/config.rs::repoint_failed::config.repoint-failed#1` | config | route-changed | route text differs (kind human both sides) |
| `crates/cli/src/config.rs::adjudicate_step_source::config.step-source-untrackable#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::check_anchor_present::config.anchor-absent#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::check_anchor_present::config.anchor-absent#2` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::check_basename_collision::config.step-id-collision#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::check_content_no_nested::config.nested-fill#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::check_fill_point_present::config.fill-point-absent#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::check_fill_point_present::config.fill-point-absent#2` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::check_not_already_forked::config.step-id-collision#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::resolve_fork_bytes::config.anchor-absent#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::run_set::config.untrackable-root#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::run_set::config.unusable-root#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::run_set::config.value-rejected#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::run_set::config.workbench-root#1` | config | unchanged | function byte-identical |
| `crates/cli/src/config.rs::undeclared_key_finding::config.undeclared-key#1` | config | unchanged | function byte-identical |
| `crates/cli/src/relocate.rs::installed_root_refusal::config.unusable-root#1` | config | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/relocate.rs::installed_root_refusal::config.workbench-root#1` | config | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/start.rs::project_step::overrides.project-step-missing#1` | config | unchanged | function byte-identical |
| `crates/cli/src/start.rs::shadowed_definition_ids::config.shadow-id-collision#1` | config | unchanged | function byte-identical |
| `crates/cli/src/setup.rs::untracked_workbench_finding::uninstall.untracked-workbench-file#1` | uninstall | route-changed | route kind prose→variable and its text differ |
| `crates/cli/src/setup.rs::dirty_worktree_finding::uninstall.dirty-worktree#1` | uninstall | function-changed | the construction itself changed (message, location or arguments); route identical; route is built elsewhere (`variable`) — not isolated, judged on the function body — by hand: **the route changed** — `ANCHOR_CLAUSE` appended when a held path is anchored, and the condition for the non-worktree clause narrowed; the message is reworded |
| `crates/cli/src/setup.rs::uninstall::uninstall.remove-allowlist#1` | uninstall | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::uninstall::uninstall.remove-deny#1` | uninstall | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::uninstall::uninstall.remove-guide#1` | uninstall | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::uninstall::uninstall.remove-hook#1` | uninstall | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::uninstall::uninstall.remove-jigc#1` | uninstall | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::uninstall::uninstall.remove-precommit#1` | uninstall | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::uninstall::uninstall.unwire-reference#1` | uninstall | function-changed | construction identical; the function differs elsewhere |
| `crates/cli/src/setup.rs::foreign_bytes_finding::uninstall.foreign-bytes#1` | uninstall | unchanged | function byte-identical |
| `crates/cli/src/setup.rs::run_uninstall::uninstall.profile-load#1` | uninstall | unchanged | function byte-identical |
| `crates/cli/src/setup.rs::staged_prose_finding::uninstall.staged-prose#1` | uninstall | unchanged | function byte-identical |
| `crates/cli/src/setup.rs::unverified_foreign_finding::uninstall.foreign-bytes#1` | uninstall | unchanged | function byte-identical |
| `crates/cli/src/setup.rs::unverified_workbench_finding::uninstall.untracked-workbench-file#1` | uninstall | unchanged | function byte-identical |
| `crates/cli/src/setup.rs::unverified_worktrees_finding::uninstall.dirty-worktree#1` | uninstall | unchanged | function byte-identical |
| `crates/cli/src/task.rs::unverified_prose_finding::uninstall.staged-prose#1` | uninstall | unchanged | function byte-identical |
| `crates/cli/src/milestone.rs::ensure_workflow_provided::workflow-refs.unknown-workflow#1` | start | unchanged | function byte-identical |
| `crates/cli/src/start.rs::pack_resource_missing::pack.resource-missing#1` | start | unchanged | function byte-identical |
| `crates/cli/src/start.rs::read_named_workflow::workflow-refs.suppressed-malformed#1` | start | unchanged | function byte-identical |
| `crates/cli/src/start.rs::read_named_workflow::workflow.verb-routed#1` | start | unchanged | function byte-identical |
| `crates/cli/src/start.rs::read_workflow::workflow-refs.unknown-workflow#1` | start | unchanged | function byte-identical — fn name declared 2× now / 2× at rc.24 in this file — compared the one holding the site |
| `crates/cli/src/start.rs::reenter_in_repo::workflow-refs.workflow-mismatch#1` | start | unchanged | function byte-identical |
| `crates/engine/src/compose.rs::anchor_position::workflow-refs.structural-anchor-resolves#1` | start | unchanged | function byte-identical |
| `crates/engine/src/compose.rs::find_fan_out_join_pairing::workflow-refs.fan-out-join-paired#1` | start | unchanged | function byte-identical |
| `crates/engine/src/compose.rs::find_fan_out_join_pairing::workflow-refs.fan-out-join-paired#2` | start | unchanged | function byte-identical |
| `crates/engine/src/compose.rs::workflow_refs_with_fills::workflow-refs.slot-fill-orphan#1` | start | unchanged | function byte-identical |
| `crates/engine/src/store.rs::home_shape_refusal::store.home-not-regular-file#1` | doc read | new-code | code occurs nowhere in production source at rc.24; function is new too |
| `crates/cli/src/task.rs::no_committed_doc::store.not-found#1` | doc read | unchanged | function byte-identical |
| `crates/cli/src/task.rs::reject_fixed_identity_slug::store.fixed-identity#1` | doc read | unchanged | function byte-identical |
| `crates/cli/src/task.rs::reject_malformed_slug_head::store.malformed-slug#1` | doc read | unchanged | function byte-identical |
| `crates/cli/src/task.rs::reject_malformed_work_unit_id::work-unit.malformed-id#1` | doc read | unchanged | function byte-identical |
| `crates/cli/src/task.rs::discard_foreign_bytes_finding::task-discard.foreign-bytes#1` | task discard | unchanged | function byte-identical |
| `crates/cli/src/task.rs::discard_staged_prose_finding::task-discard.staged-prose#1` | task discard | unchanged | function byte-identical |
| `crates/cli/src/task.rs::discard_unverified_foreign_finding::task-discard.foreign-bytes#1` | task discard | unchanged | function byte-identical |
| `crates/cli/src/task.rs::discard_unverified_prose_finding::task-discard.staged-prose#1` | task discard | unchanged | function byte-identical |
| `crates/cli/src/ingest.rs::near_miss_finding::ingest.needs-reconcile#1` | ingest | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/ingest.rs::unaddressable_identity_finding::ingest.unaddressable-identity#1` | ingest | unchanged | function byte-identical |
| `crates/cli/src/ingest.rs::wrong_location_finding::ingest.wrong-location#1` | ingest | unchanged | function byte-identical |
| `crates/cli/src/migrate.rs::adjudicate_source_path::migrate.source-untrackable#1` | migrate | unchanged | function byte-identical |
| `crates/cli/src/migrate.rs::adjudicate_source_tracked::migrate.source-untracked#1` | migrate | unchanged | function byte-identical |
| `crates/cli/src/task.rs::bind::task-bind.role-type-mismatch#1` | task bind | unchanged | function byte-identical |
| `crates/cli/src/task.rs::bind::task-bind.undeclared-role#1` | task bind | unchanged | function byte-identical; route is built elsewhere (`variable`) — not isolated, judged on the function body |
| `crates/cli/src/render.rs::validation_store::pack-probe-integrity.probe-failure#1` | validate | unchanged | enclosing item byte-identical — the site stands in no function: the reader names the last `fn` before it; compared the top-level item that holds it (`pub const STORE_EXIT_FLIPS: &[StoreExitFlip] = &[`) |
| `crates/engine/src/probe.rs::meta_finding::pack-probe-integrity.probe-failure#1` | validate | unchanged | function byte-identical |

One blocking site of rc.24 has no counterpart now and so is no row: `crates/engine/src/milestone.rs::record_exists_finding::milestone.record-exists#1` — the function still exists and hands the construction to the new `record_home_refusal`.

## 3. The sample check

**44 rows were read at both revisions by hand** — `git grep` for the code and the `fn`, `git show <sha>:<file>` for the two function texts, `diff` of the two — without the script's own extraction. **The script's class was right on 44 of 44.**

| class | rows read | which |
|---|---:|---|
| new-code | 5 of 5 | all — the code is in no file of rc.24's whole tree, and the function's name is nowhere under `crates/` there |
| new-site | 7 of 7 | all — five functions whose name is nowhere under `crates/` at rc.24; the two extra `setup.inject-allowlist` constructions read in place |
| route-changed | 5 of 5 | all — each route differs in the text a reader would see |
| function-changed | 8 of 31 | `run_retitle_item`, `title_ignored_refusal` (`doc.rs`); `leftover_finding` (`milestone.rs`); `commit_install`, `finding`, `dirty_worktree_finding` (`setup.rs`); `carried_staged_finding`, `clobber_finding` (`engine/src/finalize.rs`) |
| unchanged | 19 of 174 | `config` 2 · `doc write` 2 · `task finalize` 2 · `milestone` 2 · `unassigned` 2 · `start` 1 · `doc read` 1 · `task discard` 1 · `ingest` 1 · `migrate` 1 · `task bind` 1 · `validate` 1 · `uninstall` 1 · `setup` 1 — two texts with an empty diff each |

Every family has at least one row in the sample.

**What had to be fixed, and what found it.** Not the sample: the script's own check that a site stands inside the body of the function it is named after. Four rows failed it — the `validation_store` rows of `crates/cli/src/render.rs` stand in a `const` table (`STORE_EXIT_FLIPS`) that follows that function, and the reader names the last `fn` before a site. The first run compared `validation_store`'s body for them. The script now compares the top-level item that holds such a site, and was re-run; the four rows were `unchanged` before and are `unchanged` after, so no count moved.

**Functions of one name in one file:** 3 rows (`read_workflow` in `crates/cli/src/start.rs`; `into_finding` twice in `crates/engine/src/cascade.rs`). Each was resolved by the site's own offset at both revisions; none is left ambiguous. Two of the three are in the hand sample.

**What the sample showed beyond right or wrong:** of the eight `function-changed` rows whose route is `variable`, three have a route that changed (section 1). The class is the one the rule prescribes; the count of rows with a changed route is three higher than class 3 says.

## 4. What the method cannot see

**A function that is identical while what surrounds it changed.** `unchanged` says the constructing function's bytes are the same. Most sites are small `*_finding` helpers, so this is the common case, not the exception: the range adds 18 477 lines and 174 of 222 rows are `unchanged`. One level around each `unchanged` row was counted, by name:

| family | unchanged rows in a function | a direct caller is changed or new | a directly called free function is changed or new | either |
|---|---:|---:|---:|---:|
| `task finalize` | 49 | 19 | 0 | 19 |
| `milestone` | 27 | 6 | 0 | 6 |
| `doc write` | 21 | 11 | 0 | 11 |
| `unassigned` | 21 | 4 | 0 | 4 |
| `config` | 18 | 0 | 4 | 4 |
| `start` | 10 | 2 | 1 | 3 |
| `uninstall` | 7 | 4 | 1 | 5 |
| `doc read` | 4 | 2 | 0 | 2 |
| `task discard` | 4 | 0 | 0 | 0 |
| `ingest` | 3 | 0 | 0 | 0 |
| `migrate` | 2 | 2 | 0 | 2 |
| `task bind` | 2 | 0 | 0 | 0 |
| `setup` | 1 | 0 | 1 | 1 |
| `validate` | 1 | 0 | 0 | 0 |
| **total** | **170** | **50** | **7** | **57** |

This is an approximation in both directions. A name declared more than once in production source is followed only inside its own file; method calls are not followed as callees at all, since the receiver's type is not read; a caller counts whatever changed in it, whether or not the change is on the path to the site; and nothing is followed past one level. The four `const`-table rows are left out.

- **A callee that changed.** Beyond the count above: a route or message built by a helper, a constant spliced into a route (`ANCHOR_CLAUSE`, `FORCE`), a `Display` impl, a method on a type.
- **A state that newly reaches an old site.** A row is one site however many states reach it. New callers, new arms in a caller, a guard that moved — none of it is in the row. The caller column is the nearest measure and it is one level deep.
- **The text of a `variable` route.** 53 blocking rows; 42 of them `unchanged`. For those the route is built by a caller or a callee and nothing here says whether it reads the same.
- **A route's operands.** `route-text` holds the string literals of the route expression. A changed format argument, or a changed constant behind an unchanged name, leaves it equal. No row has a route expression that differs while its literals are equal, but a constant defined elsewhere would not show either way.
- **The message.** A changed message is class 4, never class 3. One row says so in its clause (`dirty_worktree_finding`, `uninstall`).
- **Which codes a `dynamic` site mints** (24 rows). A check beside the count: the code-shaped string literals of production source that are new since rc.24 are four — `finalize.linked-worktree-doc`, `milestone.unlanded-work`, `setup.unverified-install-path`, `store.home-not-regular-file` — and all four are the codes of the five `new-code` rows; none disappeared. A code assembled otherwise than as one literal is outside that check.
- **Severity.** Only rows written `blocking` are counted. One row's severity is handed in (`dynamic`) and 38 are `advisory`; a finding re-graded after construction is not seen. No paired row changed severity or form.
- **Pairing by ordinal.** Two same-code sites in one function that swapped places would pair crosswise. A function renamed or moved to another file reads as `new-site` by definition; none of the seven `new-site` rows is such a move.
- **Byte-identity is strict.** A comment or a re-wrapped line inside a function makes it `function-changed`. None of the 31 is comment-or-whitespace-only. In the three large functions (`write_install_span` 12 rows beside its two new ones, `uninstall` 7, `install` 4) every site shares one verdict: the clause says whether the construction itself is the same, not how near the change is.
- **What the scan itself cannot see** is declared in `crates/cli/tests/support/refusal_sites.rs`: a finding built through a macro or an alias, a route set after construction, a site in no function.
- **Everything outside `crates/*/src`.** Pack content, adapter profiles, the guides and the tests are not production source to this count.

## Context: where the range changed production source

`git diff --stat 91834b5e ae5454bf -- crates/cli/src crates/engine/src`: **29 files changed, 18 477 insertions, 1 388 deletions** (28 modified, 1 added). The ten files with the most changed lines:

| file | changed lines | + | − |
|---|---:|---:|---:|
| `crates/cli/src/setup.rs` | 4039 | 3548 | 491 |
| `crates/engine/src/finalize.rs` | 3733 | 3635 | 98 |
| `crates/cli/src/milestone.rs` | 2589 | 2397 | 192 |
| `crates/cli/src/task.rs` | 1925 | 1792 | 133 |
| `crates/engine/src/file_state.rs` | 1578 | 1487 | 91 |
| `crates/engine/src/state.rs` | 1539 | 1501 | 38 |
| `crates/cli/src/adapter.rs` | 618 | 509 | 109 |
| `crates/cli/src/doc.rs` | 529 | 488 | 41 |
| `crates/cli/src/render.rs` | 461 | 446 | 15 |
| `crates/cli/src/regular_file.rs (new)` | 307 | 307 | 0 |

Commits in the range that touch `crates/*/src`: **62** of 171.
