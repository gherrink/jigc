# Coverage — every changed surface in exactly one column

[protocol.md](protocol.md) §6: M47's rule over the M49 + `1799a2d` diff. Each changed surface
lands in **trial-reached** (naming the arm or session) · **test-fenced** (naming the suite) ·
**neither** (explained). Derived from the increments' own surfaces as enumerated against the
roadmap, the VERDICT and the DECISIONS entries, and from the registries the arms iterate —
never from a changed-file list. *Reachable* is not *exercised*: a row is trial-reached only when
a recorded arm or session drove it.

| increment · surface | trial-reached | test-fenced | neither |
|---|---|---|---|
| **1** `--unset` of an absent item field acks `already_absent` | walk 11 | `item_region_shape_space.rs` | |
| **1** `conformance.duplicate-field` flips `task validate` | walk 11 | `item_region_shape_space.rs` | |
| **1** prose at the prescribed slot depth lands (audit fix 2) | walk 11 (changelog, roadmap, spec) | `item_region_shape_space.rs` (slot-position dimension) | |
| **1** the 18-cell manufactured cube · `write.target-escape` | — | `item_region_shape_space.rs` | |
| **2** `add-task --workflow <unknown>` blocks before the mint | walk 12 · verify-pair m49 | `milestone_workflow_membership.rs` | |
| **2** sub-task `task discard` → committed `discarded`; `list-tasks` writes nothing | walk 12 | `subtask_discard_record.rs` · `read_verb_acts_nothing.rs` | |
| **2** `set:` closed vocabulary at schema load | — | `set_kind_vocabulary.rs` · flow50 arm 2 | *(pack-author-reachable only; a `JIGC_PACK_DIR` arm was not written — declared)* |
| **3** a project shadow blocks every door by name; presentation-only loads | walk 13 | `freeze_enforcement.rs` (47 doors) | |
| **3** `setup` exit 0 over a shape-changing shadow (declared bound) | walk 13 — **measured 0/4** | — | |
| **3** mis-keyed `repeatable:` leaf refused at load | walk 13 | `schema_load_strictness.rs` | |
| **4** `AddedItemSlot` · `removed-item-slot` · nested `Unclassified` | walk 14/21 (completion-record 1→2 lands) | `migrate_corpus_item_slot.rs` · flow50 arm 3 | |
| **5** `doc add-item --slug` + enum-id refusal + the `--slug <id>-N` route | walk 11 | `add_item_slug.rs` | |
| **5** fragment-fault routes at `jigc doc schema` | walk 18 (address misses) | flow50 arm 4 | |
| **5** the nesting ceiling derived (loader 4 → 2) | — | `record_nesting_cap.rs` · flow50 arm 4 | *(needs a manufactured pack; declared)* |
| **5** `doc author` batch performance | — | — | *(no bytes move; not trial-testable)* |
| **6** PB-1: `[listed ▸ dev ▸ methodology]`, demotion, `--explain` winner | walk 20 | `project_pack_composition.rs` | |
| **7** `placement-root` re-roots via `git mv`; root-declared never re-roots; `""` → `.` | walk 15 | `placement_override.rs` | |
| **7** `.git` refused (audit HIGH) | walk 15 | `placement_override.rs` (the untrackable predicate) | |
| **7** the 8 mover sites | — | `placement_override.rs` | |
| **8** N1 `write.unknown-section` at every item door | walk 16 (6 write doors) · walk 18 · **W-15** (the section-only `set-field` cell is bare) | `write_miss_shape_axis.rs` | |
| **8** N2 `schema-version` integer on `doc show` | walk 16 · verify-pair m49 · every blind session's JSON reads | flow50 arm 5 | |
| **8** D1 located text | walk 16 · walk 11 (`… · line 15`) | flow50 arm 5 | |
| **8** D5 `is_route_exempt` enumerated | — | flow50 arm 5 | |
| **8** the gate route carries `--task` with two open tasks | walk 16 · walk 21 (`--task prune-on-overflow`) | `milestone_boundary_gate.rs` | |
| **9** `planning-record`, 14 gates, `finalize` blocks | walk 19 · **B4-h** (the worker stopped at the human Settle gate before creating it) · B4-s | `planning_gate_forcing.rs` | |
| **9** `completion-record` 1→2 (`HIGH`, `detail`) | walk 14/21 | `doctype_map_versions.rs` · flow50 arm 5 | |
| **9** `milestone-record` 2→3 (`workflow` leaf) | walk 14/21 (`workflow: sub-task` in the landed record) | `doctype_map_versions.rs` | |
| **9** the corpus migration path (`schema-version-current`, `migrate-corpus` first) | walk 21 | `schema_version_ahead_axis.rs` (the ahead side) | |
| **10** delegation prose · the copy-in clause | every session's composed steps (B3, B3-h2, B4) | `methodology_delegation_prose.rs` | |
| **10** the `Spawn:` line runs the recorded workflow | walk 14/21 (extracted verbatim) | flow50 arm 2 | |
| **10** `milestone.provision-failed` · the partial-worktree advisory | — | flow50 arm 2 | *(needs a mid-provision fault; declared)* |
| **10** `finalize.render-io` NotFound vs I/O | — | `finalize_render_io_absent.rs` | |
| **11** one unknown-doctype answer at 15 doors, no Debug leak | walk 18 (8 doors) | `unknown_doctype_axis.rs` | |
| **11** not-a-git-repo, once, 47 verbs | walk 18 (2 verbs) | `not_in_repo_axis.rs` | |
| **11** `rename`'s nine refusals with code + exit | walk 18 (9/9) | flow50 arm 6 · `rename.rs`'s `RefusalKind::ALL` | |
| **11** the clap seam (`InvalidUtf8` no panic) | walk 18 (exit 2) | `clap_error_kind_axis.rs` | |
| **11** `describe --commands` union + `pack` | walk 18 · verify-pair m49 | flow50 arm 6 | |
| **11** read surfaces + the heredoc form named in the pack (S-1) | walk 18 · every session's composed text | `read_surface_naming.rs` · `stdin_form_naming.rs` | |
| **11** S-4: `jigc rename --task` names `doc rename` | walk 18 | `clap_error_kind_axis.rs` | |
| **12** `MINT_DOORS` · goldens · ledgers | — | `mint_doors.rs` · the 624 goldens | *(no verb, finding or route — declared)* |
| **`1799a2d`** the milestone boundary gates the sub-task commit doc | walk 14/21 (baseline landed → blocks; route run verbatim) · **W-1** (text prefix) | `milestone_boundary_gate.rs` | |
| **audit 1** `.git` as placement root | walk 15 | `placement_override.rs` | |
| **audit 5/uninstall** a non-directory leftover at a destroying door | walk 02 cell C · **W-2** | flow49 `DESTROYING_DOORS` arm (registered) | |
| **audit 3/4/6/7** whitespace literals · `non-reparseable` target · `FREEZE_DOORS` · `location` concatenation | — | their own suites (`freeze_enforcement.rs` etc.) | *(no verb of their own; the doors they sit behind are reached above)* |

## What the trial reached that no increment named

- **The empty-id axis** (walk 17): 25 doors, and `task discard ""` destroying `.jigc/tasks/` —
  **W-13**, in no increment, no suite, no ledger.
- **`placement-root .jigc`** (walk 15): accepted, then removed by `uninstall` unnamed — **W-14**,
  the `.git` guard's un-swept sibling.
- **A listed pack's missing catalog blamed on the embedded pack** (walk 20) — **W-5**.

## The declared *neither* set

Four rows sit in *neither* and each says why: two need a manufactured pack (`set:` vocabulary,
the nesting ceiling), one a mid-provision fault, one is performance. None is a gap the trial
could have closed with another arm without changing what it measures.
