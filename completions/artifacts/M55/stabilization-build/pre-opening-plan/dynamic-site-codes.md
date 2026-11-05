# What the dynamic sites mint

**What this is.** Task PC of the plan before the first stabilization run opens ([detail.md](detail.md) → §2 → Phase 0 → PC). It lists every finding code each dynamic refusal site left after B1 can print, the source location that spells each code, and how the list was established. **Read at `4218e3df`** (`fix/rc24-tier1`), by reading only: nothing was built or run. **This is an agent's reading. It is not held by anything yet.** S2's arm (`every_code_a_dynamic_site_mints_has_its_case_or_a_ruled_reason`, C16a's first commit) and P-Z's arms are what will hold it. Line numbers are at `4218e3df` and will drift.

**Columns.** *Spelled at* is where the code's string literal stands. Where the code reaches the site through a constant, it gives the constant's line and the line that hands it over. *Catalog* says what `crates/cli/tests/route_catalog.rs` already records for this site and code:
- `MINTED_AT`: a row of that table.
- `CLAIMS`: a claim, so the code already has its own answer.
- `—`: neither table names this code at this site.

## How the seventeen were established

1. The committed site list `crates/cli/tests/route_catalog/sites.tsv` has **27** rows whose `code` is `dynamic`.
2. **B1** (P1 commit 1) leaves out the bodies of `impl Finding` in `crates/engine/src/finding.rs`. That removes three rows: `finding.rs::block`, `::blocking` and `::graded`. **24** rows are left.
3. **Six** of the 24 are answered whole through `CLAIMS` and are not in `IN_PART`. `owing()` (`route_catalog.rs`) therefore asks nothing of them under their own id. They are:
   - `engine/finalize.rs::setup_dirty_install_finding`;
   - `cli/locate.rs::locate_finding` #1 and #2;
   - `cli/setup.rs::replaced_path_refusal`, `::merged_through_links` and `::edited_through_links`.
4. **One** row is advisory, so it owes nothing: `engine/file_state.rs::lagging_baseline_finding`.
5. **17** rows are left. They are exactly the dynamic rows of `crates/cli/tests/route_catalog/scheduled.tsv` once the three `finding.rs` rows are taken out. **This matches the spec's 17.**

**How each site was followed.** For each site I took the function that holds the constructor and found every production caller of it. Where the callers hand a code over through an enum's `code()` or through a constant, I resolved it to the literal. Test regions (`#[cfg(test)]`) were left out. Enumerations over an enum are `match`es with no wildcard arm, so the compiler keeps them total. Where a variant is named only "so the axis is total" and never reaches the site's function, it is left out and the section says so.

---

## 1. `crates/cli/src/author.rs::reject` — 2 codes

| code | spelled at | catalog |
|---|---|---|
| `write.wrong-shape` | `crates/cli/src/author.rs:173` (`Ungrammatical`, `MissingTitle`) | `MINTED_AT` |
| `write.malformed-value` | `crates/cli/src/author.rs:175` (`BareSlotValue`, `WrappedFieldValue`) | `MINTED_AT` |

**Complete:** yes. `reject` (`:209`) is private and passes `kind.code()`. `PayloadReject::code` (`:171`) is an exhaustive match over the four variants.

## 2. `crates/cli/src/doc_code_probe/mod.rs::dangling` — 2 codes from the shipped packs; open by construction

The code is `format!("doc-code.{}", anchor.check_id)` at `crates/cli/src/doc_code_probe/mod.rs:456`. Every caller goes through it: `dangling_file`, `not_an_anchor`, `dangling_symbol` (twice) and `not_a_test`.

| code | spelled at | catalog |
|---|---|---|
| `doc-code.symbol-exists` | `crates/cli/packs/dev/config/field-types.yaml:22` (`code-anchor`'s `check`), formatted at `doc_code_probe/mod.rs:456` | — |
| `doc-code.criterion-maps-to-test` | `crates/cli/packs/dev/schemas/spec.yaml:38` (the field's `check:` override), formatted at `:456` | — |

**Complete: not by reading Rust.** `check_id` comes from `engine::target_surface::resolve_check_id` (`crates/engine/src/target_surface.rs:526`): the field's own `check:` if it has one, otherwise its pack field type's `check`. Both are free strings in YAML, and I found no vocabulary check on them. The probe also accepts any value: anything other than `criterion-maps-to-test` takes the symbol arm (`:598`). So a schema that declares another `check:` mints `doc-code.<that>` here.

The two codes above are complete for the packs this tree ships; the methodology pack declares no `check:`. The vocabulary fence cannot see either code, because neither is spelled whole in production Rust. `crates/engine/src/result.rs:358-359` spells them as `(probe, check)` pairs. This is the fence's declared blind spot ("a code built at run time is not found").

## 3. `crates/cli/src/ingest.rs::near_miss_finding` — 17 codes

The site (`crates/cli/src/ingest.rs:524`) takes the code of another finding: the first finding of `parse_sections(home, source)` when that returns `Err`, otherwise the first of `schema_conformance(home, source, &doc)` (`:509-511`). When neither yields a finding, the function falls to the fixed `ingest.needs-reconcile` (`:557`), which is a site of its own. So this site prints the union of what those two functions can return.

| code | spelled at | catalog |
|---|---|---|
| `conformance.header-not-first` | `crates/engine/src/parse.rs:221` | — |
| `conformance.section-missing` | `crates/engine/src/parse.rs:303` | — |
| `conformance.section-renamed` | `crates/engine/src/parse.rs:317` | — |
| `conformance.orphaned-sentinel` | `crates/engine/src/parse.rs:995` | — |
| `conformance.duplicate-field` | `crates/engine/src/parse.rs:1050` | — |
| `conformance.unknown-field` | `crates/engine/src/parse.rs:1069` | — |
| `conformance.malformed-field-block` | `crates/engine/src/parse.rs:1085` | — |
| `conformance.item-heading-unanchored` | `crates/engine/src/parse.rs:1169` | — |
| `conformance.item-anchor-malformed` | `crates/engine/src/parse.rs:1187` | — |
| `conformance.item-anchor-duplicate` | `crates/engine/src/parse.rs:1200` | — |
| `conformance.item-slot-label-missing` | `crates/engine/src/parse.rs:1382` | — |
| `conformance.item-slot-delimiter-shadowed` | `crates/engine/src/parse.rs:1450` | — |
| `conformance.slot-setext-heading` | `crates/engine/src/parse.rs:1749` | — |
| `conformance.slot-heading-depth` | `crates/engine/src/parse.rs:1759` | — |
| `schema-conformance.required-slot-present` | `crates/engine/src/validate.rs:2991`, `:3245` | — |
| `schema-conformance.required-field-present` | `crates/engine/src/validate.rs:3025`, `:3278` | — |
| `schema-conformance.field-value-conformant` | `crates/engine/src/validate.rs:3016`, `:3316`; `ID_FROM_ENUM_CODE` `:3044`, handed over at `:3205` | — |

**Complete:** yes, by reading.
- **Parse.** Every `findings.push` / `append` in the production part of `parse.rs` (`:1-1775`) pushes one of the 14 `Finding::blocking` / `graded` literals above. `parse.rs` takes no finding from another module.
- **Conformance.** `schema_conformance` (`validate.rs:2700`) reaches:
  - `check_slot_present`, `check_field` and `check_field_value`;
  - `check_repeatable`, which goes on to `check_item_leaves`, `check_item_slot_present`, `check_item_field` and `check_id_from_enum`.

  All of these mint through `blocking_conformance` with the three codes shown.
- **Why no case owes them yet.** All 17 codes are listed sites' codes elsewhere: the static `parse.rs` sites, and `blocking_conformance`, section 16. The vocabulary arm therefore asks nothing of this site. Here they carry this site's own route (`near_miss_route`, `:513`).

## 4. `crates/cli/src/migrate_corpus.rs::blocked_finding` — 11 codes

| code | spelled at | catalog |
|---|---|---|
| `migrate-corpus.schema-version-ahead` | `crates/cli/src/migrate_corpus.rs:1642` | `MINTED_AT` |
| `migrate-corpus.missing-snapshot` | `crates/cli/src/migrate_corpus.rs:1670`, `:1707` | `MINTED_AT` |
| `migrate-corpus.unclassified-change` | `crates/cli/src/migrate_corpus.rs:1734` | `MINTED_AT` |
| `migrate-corpus.narrowed-cardinality` | `crates/cli/src/migrate_corpus.rs:1770` | `MINTED_AT` |
| `migrate-corpus.removed-field` | `crates/cli/src/migrate_corpus.rs:1808` | `MINTED_AT` |
| `migrate-corpus.removed-item-slot` | `crates/cli/src/migrate_corpus.rs:1846` | `MINTED_AT` |
| `migrate-corpus.item-unmodelled-content` | `crates/cli/src/migrate_corpus.rs:2097` | `MINTED_AT` |
| `migrate-corpus.prose-needed` | `crates/cli/src/migrate_corpus.rs:2179` | `MINTED_AT` |
| `migrate-corpus.fold-refused` | `crates/cli/src/migrate_corpus.rs:2201` | `MINTED_AT` |
| `migrate-corpus.destination-collision` | `crates/cli/src/migrate_corpus.rs:2312` | `CLAIMS` |
| `migrate-corpus.deferred` | `crates/cli/src/migrate_corpus.rs:2324` | `MINTED_AT` |

**Complete:** yes. `blocked_finding` (`:1477`) is private. Its twelve production calls all pass a literal code; the thirteenth call, `:2889`, is under `#[cfg(test)]` (`:2727`).

**A note on the scheduled list.** Its row says *"the eleven other codes"* beside the claimed one. The function mints ten others, through eleven calls.

## 5. `crates/cli/src/relocate.rs::refuse` — 5 codes

| code | spelled at | catalog |
|---|---|---|
| `write.untrackable-destination` | `crates/cli/src/relocate.rs:132`, handed over at `:302` | `MINTED_AT` |
| `relocate.malformed-prior-home` | `MALFORMED_PRIOR_HOME` `crates/cli/src/relocate.rs:157`, handed over at `:537` and `:553` | — |
| `relocate.frozen-doctype` | `FROZEN_DOCTYPE` `crates/cli/src/relocate.rs:161`, handed over at `:657` | — |
| `store.transient-type` | `crates/cli/src/relocate.rs:130`, handed over at `:673` | `MINTED_AT` |
| `write.already-present` | `crates/cli/src/relocate.rs:131`, handed over at `:900` | `CLAIMS` |

**Complete:** yes. `refuse` (`:186`) is private, and its six calls each name a `RelocateRefusal` variant. Six variants never reach this site, though `RelocateRefusal::code` maps them; they are minted elsewhere:
- `UnknownDoctype`;
- `WorkbenchPriorHome` and `InstalledPriorHome`;
- `UnaddressableDestination`;
- `ForeignSource` and `LinkOnTheWay`.

The `relocate` prefix is in neither `FAMILIES` nor `CATALOGUED`.

## 6. `crates/cli/src/rename.rs::refuse` — 9 codes

| code | spelled at | catalog |
|---|---|---|
| `store.not-found` | `NOT_FOUND` `crates/engine/src/store.rs:46`, mapped at `crates/cli/src/rename.rs:215`, handed over at `:528` | `CLAIMS` |
| `write.malformed-slug` | `crates/cli/src/rename.rs:221`, handed over at `:569` | `MINTED_AT` |
| `write.unslugable-title` | `crates/cli/src/rename.rs:216`, handed over at `:594` | — |
| `rename.dirty-tree` | `crates/cli/src/rename.rs:217`, handed over at `:627` | — |
| `rename.in-flight` | `crates/cli/src/rename.rs:218`, handed over at `:647` | — |
| `write.identity-change` | `crates/cli/src/rename.rs:219`, handed over at `:685` (`FixedIdentity`) and `:710` (`WorkUnitIdentity`) | — |
| `write.already-present` | `crates/cli/src/rename.rs:220`, handed over at `:738` | `CLAIMS` |
| `write.untrackable-destination` | `crates/cli/src/rename.rs:222`, handed over at `:782` | `MINTED_AT` |
| `store.transient-type` | `crates/cli/src/rename.rs:214`, handed over at `:1413` | `MINTED_AT` |

**Complete:** yes. `refuse` (`:285`) is private, and its ten production calls each name a `RefusalKind` variant (tests start at `:1513`). Two variants never reach this site: `UnknownDoctype` and `ForeignHome`.

## 7. `crates/cli/src/render.rs::commit_rejection_finding` — 11 codes

The code is the `RejectionFrame.code` of the door whose commit was rejected. The only caller is `crates/cli/src/task.rs::surface_commit_rejection` (`task.rs:7143`), and it builds the finding **only on the `--format json` arm**. The text arm prints the frame and carries the code in the invocation log alone. All the constants are in `crates/cli/src/invocation_log.rs`.

| code | spelled at | handed over at | catalog |
|---|---|---|---|
| `finalize.commit-rejected` | `invocation_log.rs:66` | `crates/cli/src/task.rs:4382` | `MINTED_AT` |
| `finalize.amend-rejected` | `invocation_log.rs:76` | `crates/cli/src/task.rs:4374` | `MINTED_AT` |
| `milestone-finalize.commit-rejected` | `invocation_log.rs:80` | `crates/cli/src/milestone.rs:7351` | — |
| `milestone-finalize.chain-commit-rejected` | `invocation_log.rs:84` | `crates/cli/src/milestone.rs:7226` | — |
| `rename.commit-rejected` | `invocation_log.rs:87` | `crates/cli/src/cli.rs:1461` | — |
| `migrate-corpus.commit-rejected` | `invocation_log.rs:90` | `crates/cli/src/migrate_corpus.rs:316` | `MINTED_AT` |
| `milestone-create.commit-rejected` | `invocation_log.rs:93` | `crates/cli/src/milestone.rs:571` | — |
| `milestone-add-task.commit-rejected` | `invocation_log.rs:96` | `crates/cli/src/milestone.rs:585` | — |
| `milestone-add-from-spec.commit-rejected` | `invocation_log.rs:99` | `crates/cli/src/milestone.rs:603` | — |
| `milestone-discard.commit-rejected` | `invocation_log.rs:102` | `crates/cli/src/milestone.rs:628` | — |
| `task-discard.commit-rejected` | `invocation_log.rs:106` | `crates/cli/src/task.rs:1163` | — |

**Complete:** yes, by reading. `surface_commit_rejection` has seven callers:
- `task.rs:825` and `:4390`;
- `cli.rs:1457`;
- `migrate_corpus.rs:312`;
- `milestone.rs:497`, `:7222` and `:7347`.

Every `RejectionFrame` they pass takes its `code` from one of the eleven `ERROR_*_REJECTED` constants. There are seven constructions: `task.rs:1162` and `:4393`, `cli.rs:1460`, `migrate_corpus.rs:315`, and `milestone.rs:646`, `:7225` and `:7350`. In two of them `code` is a variable; I resolved it at `task.rs:4372-4382` and `milestone.rs:570-630`.

## 8. `crates/cli/src/repo.rs::finding_at` — 3 codes

| code | spelled at | catalog |
|---|---|---|
| `repo.head-detached` | `crates/cli/src/repo.rs:234` | — |
| `repo.head-unborn` | `crates/cli/src/repo.rs:235` | — |
| `repo.operation-in-progress` | `crates/cli/src/repo.rs:236` | — |

**Complete:** yes. The code is `self.member.code()`, and `PostureMember::code` (`:232`) is exhaustive over the three variants. The posture probe constructs each one in production (`:1269`, `:1276`, `:1285`). The `repo` prefix is in neither `FAMILIES` nor `CATALOGUED`.

## 9. `crates/cli/src/rollback.rs::rollback_conflict_finding` — 5 codes

| code | spelled at | catalog |
|---|---|---|
| `finalize.rollback-conflict` | `crates/cli/src/rollback.rs:446` (`FINALIZE_DOOR`) | `MINTED_AT` |
| `milestone.rollback-conflict` | `crates/cli/src/rollback.rs:461` (`MILESTONE_DOOR`) | `MINTED_AT` |
| `task-discard.rollback-conflict` | `crates/cli/src/rollback.rs:474` (`TASK_DISCARD_DOOR`) | — |
| `rename.rollback-conflict` | `crates/cli/src/rollback.rs:488` (`RENAME_DOOR`) | — |
| `config.rollback-conflict` | `crates/cli/src/rollback.rs:503` (`CONFIG_DOOR`) | `MINTED_AT` |

**Complete:** yes. The code is `door.code`. The five `ConflictDoor` constants above are the only `ConflictDoor { … }` constructions in production. Each one is passed by a production door:
- `task.rs` and `milestone.rs` pass `FINALIZE_DOOR`;
- `milestone.rs` passes `MILESTONE_DOOR` and `TASK_DISCARD_DOOR`;
- `rename.rs:893` passes `RENAME_DOOR`;
- `config.rs:547` passes `CONFIG_DOOR`.

The `MINTED_AT` comment names all five.

## 10. `crates/engine/src/cascade.rs::into_finding` #1 — 8 codes

`TargetParseError::into_finding` (`:157`), reached through `StructuralTarget::parse` (`:185`).

| code | spelled at | catalog |
|---|---|---|
| `structural-target.wrong-scheme` | `crates/engine/src/cascade.rs:120` | — |
| `structural-target.missing-colon` | `crates/engine/src/cascade.rs:121` | — |
| `structural-target.empty-workflow-id` | `crates/engine/src/cascade.rs:122` | — |
| `structural-target.empty-step-id` | `crates/engine/src/cascade.rs:123` | — |
| `structural-target.empty-anchor-step-id` | `crates/engine/src/cascade.rs:124` | — |
| `structural-target.missing-anchor` | `crates/engine/src/cascade.rs:125` | — |
| `structural-target.conflicting-anchors` | `crates/engine/src/cascade.rs:126` | — |
| `structural-target.non-ascii` | `crates/engine/src/cascade.rs:127` | — |

**Complete:** yes. `code()` (`:118`) is exhaustive, and `parse_inner` (`:173-260`) constructs every variant. The prefix is in neither `FAMILIES` nor `CATALOGUED`.

## 11. `crates/engine/src/cascade.rs::into_finding` #2 — 6 codes

`SlotFillTargetParseError::into_finding` (`:364`), reached through `SlotFillTarget::parse` (`:387`).

| code | spelled at | catalog |
|---|---|---|
| `slot-fill-target.wrong-scheme` | `crates/engine/src/cascade.rs:337` | — |
| `slot-fill-target.missing-colon` | `crates/engine/src/cascade.rs:338` | — |
| `slot-fill-target.missing-hash` | `crates/engine/src/cascade.rs:339` | — |
| `slot-fill-target.empty-step-id` | `crates/engine/src/cascade.rs:340` | — |
| `slot-fill-target.empty-fill-id` | `crates/engine/src/cascade.rs:341` | — |
| `slot-fill-target.non-ascii` | `crates/engine/src/cascade.rs:342` | — |

**Complete:** yes. `code()` (`:335`) is exhaustive, and `parse_inner` (`:379-505`) constructs every variant. The prefix is in neither `FAMILIES` nor `CATALOGUED`.

## 12. `crates/engine/src/compose.rs::blocking_workflow_refs` — 28 codes

`pub(crate)`, so its callers are in `compose.rs` and `data_value.rs` only. There are **37** production calls, every one with its code as the first argument. The `MINTED_AT` comment says *"thirty-eight calls"*.

| code | spelled at (the code argument's line) | catalog |
|---|---|---|
| `workflow-refs.suppressed-malformed` | `SUPPRESSED_MALFORMED_CODE` `crates/engine/src/compose.rs:219`, handed over at `:244` | — |
| `workflow-refs.not-utf8` | `crates/engine/src/compose.rs:394`, `:2069`, `:2165` | `MINTED_AT` |
| `workflow-refs.malformed-command-catalog` | `crates/engine/src/compose.rs:401` | `MINTED_AT` |
| `workflow-refs.duplicate-command-id` | `crates/engine/src/compose.rs:422` | `MINTED_AT` |
| `workflow-refs.malformed-command-arg` | `crates/engine/src/compose.rs:511` | `MINTED_AT` |
| `workflow-refs.malformed-data-value` | `crates/engine/src/compose.rs:624`, `:1139`, `:1754`, `:1764`, `:1908` | `MINTED_AT` |
| `workflow-refs.command-ref-resolves` | `crates/engine/src/compose.rs:760`, `:3419` | `MINTED_AT` |
| `workflow-refs.schema-ref-resolves` | `crates/engine/src/compose.rs:804`, `:3451` | `MINTED_AT` |
| `workflow-refs.collection-not-lone` | `crates/engine/src/compose.rs:1923` | `MINTED_AT` |
| `workflow-refs.step-kind-malformed` | `crates/engine/src/compose.rs:2108` | `MINTED_AT` |
| `workflow-refs.missing-front-matter` | `crates/engine/src/compose.rs:2172` | `MINTED_AT` |
| `workflow-refs.malformed-front-matter` | `crates/engine/src/compose.rs:2179` | `MINTED_AT` |
| `workflow-refs.allows-create-duplicate-type` | `crates/engine/src/compose.rs:2204` | `MINTED_AT` |
| `workflow-refs.body-include-only` | `crates/engine/src/compose.rs:2267` | `MINTED_AT` |
| `workflow-refs.include-cycle-absent` | `crates/engine/src/compose.rs:2837` | `MINTED_AT` |
| `workflow-refs.include-resolves` | `crates/engine/src/compose.rs:2845` | `MINTED_AT` |
| `workflow-refs.run-marker-not-shadowed` | `crates/engine/src/compose.rs:3614` | `MINTED_AT` |
| `workflow-refs.spawn-marker-not-shadowed` | `crates/engine/src/compose.rs:3640` | `MINTED_AT` |
| `workflow-refs.checkpoint-marker-not-shadowed` | `crates/engine/src/compose.rs:3666` | `MINTED_AT` |
| `workflow-refs.fill-survivor` | `crates/engine/src/compose.rs:3731` | `MINTED_AT` |
| `workflow-refs.catalog-not-navigable` | `crates/engine/src/data_value.rs:477` | `MINTED_AT` |
| `workflow-refs.store-not-navigable` | `crates/engine/src/data_value.rs:502` | `MINTED_AT` |
| `workflow-refs.milestone-not-navigable` | `crates/engine/src/data_value.rs:553` | `MINTED_AT` |
| `workflow-refs.undeclared-root` | `crates/engine/src/data_value.rs:566` | `MINTED_AT` |
| `workflow-refs.task-ref-in-no-task-workflow` | `crates/engine/src/data_value.rs:579` | `MINTED_AT` |
| `workflow-refs.undeclared-role` | `crates/engine/src/data_value.rs:592`, `:622` | `MINTED_AT` |
| `workflow-refs.at-marker-on-non-scalar` | `crates/engine/src/data_value.rs:608` | `MINTED_AT` |
| `workflow-refs.unresolved` | `crates/engine/src/data_value.rs:652` | `MINTED_AT` |

**Complete:** yes. All 37 calls (28 in `compose.rs` before `:3741`, nine in `data_value.rs` before `:679`) pass a literal or the one resolved constant. The 28th code, `suppressed-malformed`, is a listed site's code too (`crates/cli/src/start.rs::read_named_workflow`), which is why the vocabulary arm did not ask for it here. **The plan's "`compose`'s 27" is 28.**

## 13. `crates/engine/src/milestone.rs::store_block` — 5 codes

| code | spelled at | catalog |
|---|---|---|
| `store.unparseable` | `crates/engine/src/milestone.rs:623`, `:671` | `MINTED_AT` |
| `store.unknown-type` | `UNKNOWN_TYPE` `crates/engine/src/store.rs:54`, handed over at `crates/engine/src/milestone.rs:634` | — |
| `store.transient-type` | `crates/engine/src/milestone.rs:643` | `MINTED_AT` |
| `store.not-found` | `NOT_FOUND` `crates/engine/src/store.rs:46`, handed over at `crates/engine/src/milestone.rs:654` | — |
| `store.no-such-section` | `crates/engine/src/milestone.rs:690` | `MINTED_AT` |

**Complete:** yes. `store_block` (`:713`) is private, and its six calls (`:622-689`, all before the tests at `:3017`) pass these codes.

## 14. `crates/engine/src/override_default.rs::delta_block` — 5 codes

| code | spelled at | catalog |
|---|---|---|
| `override-default.scalar-set-orphaned` | `crates/engine/src/override_default.rs:207` | — |
| `override-default.slot-fill-orphaned` | `crates/engine/src/override_default.rs:279` | — |
| `override-default.content-changed` | `crates/engine/src/override_default.rs:333` | — |
| `override-default.needs-rebasing` | `crates/engine/src/override_default.rs:352` | — |
| `override-default.target-exists` | `crates/engine/src/override_default.rs:384` | — |

**Complete:** yes. `delta_block` (`:229`) is private, and its five calls each pass a literal; tests start at `:410`. The `override-default` prefix is in neither `FAMILIES` nor `CATALOGUED`.

## 15. `crates/engine/src/store.rs::block` — 8 codes

| code | spelled at | catalog |
|---|---|---|
| `store.transient-type` | `crates/engine/src/store.rs:371` | `MINTED_AT` |
| `store.not-found` | `NOT_FOUND` `crates/engine/src/store.rs:46`, handed over at `:399`, `:520`, `:531` | — |
| `store.unknown-type` | `UNKNOWN_TYPE` `crates/engine/src/store.rs:54`, handed over at `:481`, `:647` | `MINTED_AT` |
| `store.unparseable` | `crates/engine/src/store.rs:577` | `MINTED_AT` |
| `store.not-staged` | `crates/engine/src/store.rs:609`, `:619` | `MINTED_AT` |
| `store.no-such-section` | `crates/engine/src/store.rs:706`, `:795` | `MINTED_AT` |
| `store.no-such-item` | `crates/engine/src/store.rs:810`, `:923`, `:937` | `MINTED_AT` |
| `store.no-such-leaf` | `NO_SUCH_LEAF` `crates/engine/src/store.rs:50`, handed over at `:967`, `:1054` | `MINTED_AT` |

**Complete:** yes. `block` (`:654`) is private, and its sixteen production calls (before `:1149`) pass these codes. `crates/cli/src/doc.rs` has a `block` of its own with the same name; that one is not this site.

## 16. `crates/engine/src/validate.rs::blocking_conformance` — 5 codes

| code | spelled at | catalog |
|---|---|---|
| `schema-conformance.required-slot-present` | `crates/engine/src/validate.rs:2991`, `:3245` | `MINTED_AT` |
| `schema-conformance.required-field-present` | `crates/engine/src/validate.rs:3025`, `:3278` | `MINTED_AT` |
| `schema-conformance.field-value-conformant` | `crates/engine/src/validate.rs:3016`, `:3316`; `ID_FROM_ENUM_CODE` `:3044`, handed over at `:3205` | — |
| `schema-conformance.unknown-type` | `crates/engine/src/validate.rs:2394` | `MINTED_AT` |
| `owner-artifact.present` | `crates/engine/src/validate.rs:2591` (the arm that is not the untracked cause) | — |

**Complete:** yes, and the code holds it too. `blocking_conformance` (`:3401`) asserts that its code is in `CONFORMANCE_ROUTE_CODES` (`:3424-3430`), which holds exactly these five. Its nine production calls are at `:2393`, `:2591`, `:2990`, `:3015`, `:3024`, `:3204`, `:3244`, `:3277` and `:3315`. The two codes marked `—` are listed sites' codes elsewhere: `doc.rs::id_from_enum_block` and `::trailer_key_refusal`, and `validate.rs::owner_artifact_present`.

## 17. `crates/engine/src/write.rs::blocking_write` — 13 codes

Codes reach the site three ways:
- 14 direct calls with a literal;
- `generate_error_finding` (`:7653`), an exhaustive match over the seven `GenerateError` variants;
- `splice_error_finding` (`:7977`), an exhaustive match over the four `SpliceError` variants.

| code | spelled at | catalog |
|---|---|---|
| `write.list-overwrite` | `crates/engine/src/write.rs:6588` | `MINTED_AT` |
| `write.non-reparseable` | `crates/engine/src/write.rs:6695`, `:6808`, `:7063`, `:7358`; splice map `:7981` | `MINTED_AT` |
| `write.target-escape` | `crates/engine/src/write.rs:6709` | `MINTED_AT` |
| `write.unknown-field` | `crates/engine/src/write.rs:6738`, `:6895`, `:6986`; generate map `:7684`; splice map `:7991` | `MINTED_AT` |
| `write.malformed-value` | `crates/engine/src/write.rs:6745` | `MINTED_AT` |
| `write.not-present` | `crates/engine/src/write.rs:7181`, `:7337`; generate map `:7677`; splice map `:7979` | `MINTED_AT` |
| `write.slot-setext-heading` | `crates/engine/src/write.rs:7615` | `MINTED_AT` |
| `write.slot-heading-depth` | `crates/engine/src/write.rs:7624` | `MINTED_AT` |
| `write.already-present` | generate map `crates/engine/src/write.rs:7656` | — |
| `write.unknown-section` | generate map `crates/engine/src/write.rs:7660`; splice map `:7987` | `MINTED_AT` |
| `write.wrong-shape` | generate map `crates/engine/src/write.rs:7670` | `MINTED_AT` |
| `write.unslugable-title` | generate map `crates/engine/src/write.rs:7688` | — |
| `schema-conformance.field-value-conformant` | generate map `crates/engine/src/write.rs:7696` | — |

**Complete:** yes. `blocking_write` (`:6255`) is private. Its sixteen production calls are the 14 literal ones above and the two that pass the mapped `code` (`:7700`, `:7994`); the two calls at `:15318` and `:15343` are under `#[cfg(test)]`. Every variant of both error enums is constructed in production.

---

## Total

| # | site | codes | `MINTED_AT` | `CLAIMS` | neither |
|---|---|---|---|---|---|
| 1 | `author.rs::reject` | 2 | 2 | 0 | 0 |
| 2 | `doc_code_probe/mod.rs::dangling` | 2 (open) | 0 | 0 | 2 |
| 3 | `ingest.rs::near_miss_finding` | 17 | 0 | 0 | 17 |
| 4 | `migrate_corpus.rs::blocked_finding` | 11 | 10 | 1 | 0 |
| 5 | `relocate.rs::refuse` | 5 | 2 | 1 | 2 |
| 6 | `rename.rs::refuse` | 9 | 3 | 2 | 4 |
| 7 | `render.rs::commit_rejection_finding` | 11 | 3 | 0 | 8 |
| 8 | `repo.rs::finding_at` | 3 | 0 | 0 | 3 |
| 9 | `rollback.rs::rollback_conflict_finding` | 5 | 3 | 0 | 2 |
| 10 | `cascade.rs::into_finding` #1 | 8 | 0 | 0 | 8 |
| 11 | `cascade.rs::into_finding` #2 | 6 | 0 | 0 | 6 |
| 12 | `compose.rs::blocking_workflow_refs` | 28 | 27 | 0 | 1 |
| 13 | `engine/milestone.rs::store_block` | 5 | 3 | 0 | 2 |
| 14 | `override_default.rs::delta_block` | 5 | 0 | 0 | 5 |
| 15 | `store.rs::block` | 8 | 7 | 0 | 1 |
| 16 | `validate.rs::blocking_conformance` | 5 | 3 | 0 | 2 |
| 17 | `write.rs::blocking_write` | 13 | 10 | 0 | 3 |
| | **total** | **143** | **73** | **4** | **66** |

**143 site-and-code pairs**: one case each under fork 3 A.
- **Already answered:** 4 of them, through `CLAIMS`.
- **Owed:** 139.
- **`MINTED_AT`:** every one of its 73 rows is among the 143.

**The 66 that neither table names:**
- **36 are of prefixes outside `CATALOGUED`:**
  - `relocate` 2;
  - `rename` 4;
  - `milestone-finalize` 2;
  - `milestone-create`, `milestone-add-task`, `milestone-add-from-spec` and `milestone-discard` 4 together;
  - `task-discard` 2;
  - `repo` 3;
  - `structural-target` 8;
  - `slot-fill-target` 6;
  - `override-default` 5.
- **30 are of catalogued prefixes.** They are listed sites' codes elsewhere, or a code built at run time (`doc-code.*`), so the vocabulary arm never asked for them at these sites.

**Against the plan.**
- **The price.** §4 prices the dynamic units at *"73 `MINTED_AT` codes, plus an estimated 20–60 codes of uncatalogued prefixes"*, which is 93–133. This reading gives 139 owed: **6 above the high end**, before any project-declared `doc-code` check.
- **Units of work in P.** With the 125 static sites, P comes to **264**, against the plan's 218–258.
- **The C16a–C18 cut.** Its *"`compose`'s 27"* is **28**.

**Sites whose completeness could not be established by reading:** one, `doc_code_probe/mod.rs::dangling`. Its code is built from a schema's `check:` value, and nothing in the Rust holds that value to a vocabulary. The two codes above are complete for the packs this tree ships, and open for a schema a project declares. Every other site's list follows from a private constructor with every caller read, or from an exhaustive `match` over an enum.
