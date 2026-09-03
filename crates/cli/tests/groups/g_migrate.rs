// GENERATED GROUP ROOT — see implementation/dev-workflow.md (test target consolidation).
// Each suite below keeps its own file; this root only makes them one cargo target,
// so a source change relinks ~10 test binaries instead of 252.

#[path = "../support/mod.rs"]
mod support;

#[path = "../adapter_artifact.rs"]
mod adapter_artifact;
#[path = "../anchor_root_truth.rs"]
mod anchor_root_truth;
#[path = "../dev_gate_report.rs"]
mod dev_gate_report;
#[path = "../dev_rig_parity.rs"]
mod dev_rig_parity;
#[path = "../duplicate_field_finding_keys.rs"]
mod duplicate_field_finding_keys;
#[path = "../flow10_acceptance.rs"]
mod flow10_acceptance;
#[path = "../flow27_marquee.rs"]
mod flow27_marquee;
#[path = "../flow36_corpus_structural.rs"]
mod flow36_corpus_structural;
#[path = "../foreign_at_both_doors.rs"]
mod foreign_at_both_doors;
#[path = "../golden_harness.rs"]
mod golden_harness;
#[path = "../managed_vs_foreign.rs"]
mod managed_vs_foreign;
#[path = "../migrate_adr.rs"]
mod migrate_adr;
#[path = "../migrate_arch_doc.rs"]
mod migrate_arch_doc;
#[path = "../migrate_byte_floor.rs"]
mod migrate_byte_floor;
#[path = "../migrate_corpus_foreign.rs"]
mod migrate_corpus_foreign;
#[path = "../migrate_corpus_halt_causes.rs"]
mod migrate_corpus_halt_causes;
#[path = "../migrate_corpus_item_leaf_residual.rs"]
mod migrate_corpus_item_leaf_residual;
#[path = "../migrate_corpus_item_slot.rs"]
mod migrate_corpus_item_slot;
#[path = "../migrate_corpus_log_completeness.rs"]
mod migrate_corpus_log_completeness;
#[path = "../migrate_corpus_set_fields.rs"]
mod migrate_corpus_set_fields;
#[path = "../migrate_corpus_value_remap.rs"]
mod migrate_corpus_value_remap;
#[path = "../migrate_methodology.rs"]
mod migrate_methodology;
#[path = "../migrate_prd.rs"]
mod migrate_prd;
#[path = "../migrate_retire_adopt.rs"]
mod migrate_retire_adopt;
#[path = "../migrate_retire_safety.rs"]
mod migrate_retire_safety;
#[path = "../migrate_review_gate.rs"]
mod migrate_review_gate;
#[path = "../migrate_rollback.rs"]
mod migrate_rollback;
#[path = "../migrate_seam.rs"]
mod migrate_seam;
#[path = "../migrate_spec.rs"]
mod migrate_spec;
#[path = "../migrate_workflow.rs"]
mod migrate_workflow;
#[path = "../record_foreign_arm.rs"]
mod record_foreign_arm;
#[path = "../record_item_slot_kind.rs"]
mod record_item_slot_kind;
#[path = "../record_nesting_cap.rs"]
mod record_nesting_cap;
#[path = "../record_set_splice_retired.rs"]
mod record_set_splice_retired;
#[path = "../record_stale_reasons.rs"]
mod record_stale_reasons;
#[path = "../registry_seam.rs"]
mod registry_seam;
#[path = "../setup.rs"]
mod setup;
#[path = "../snapshot_store_two_snapshots.rs"]
mod snapshot_store_two_snapshots;
#[path = "../test_target_registration.rs"]
mod test_target_registration;
#[path = "../verb_suite_coverage.rs"]
mod verb_suite_coverage;
#[path = "../version_mismatch_break.rs"]
mod version_mismatch_break;
#[path = "../version_stamp_rollback.rs"]
mod version_stamp_rollback;
