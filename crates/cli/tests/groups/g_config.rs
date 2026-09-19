// GENERATED GROUP ROOT — see implementation/dev-workflow.md (test target consolidation).
// Each suite below keeps its own file; this root only makes them one cargo target,
// so a source change relinks ~10 test binaries instead of 252.

#[path = "../support/mod.rs"]
mod support;

#[path = "../absorb_surface.rs"]
mod absorb_surface;
#[path = "../changelog_cold_create.rs"]
mod changelog_cold_create;
#[path = "../changelog_gate_advisory.rs"]
mod changelog_gate_advisory;
#[path = "../changelog_step_subset.rs"]
mod changelog_step_subset;
#[path = "../changelog_write_touch.rs"]
mod changelog_write_touch;
#[path = "../checkpoint_acceptance.rs"]
mod checkpoint_acceptance;
#[path = "../code_anchor_grammar_sites.rs"]
mod code_anchor_grammar_sites;
#[path = "../cold_start_zero_commit.rs"]
mod cold_start_zero_commit;
#[path = "../config_ack_uncommitted.rs"]
mod config_ack_uncommitted;
#[path = "../config_fill.rs"]
mod config_fill;
#[path = "../config_fork.rs"]
mod config_fork;
#[path = "../config_fork_compose.rs"]
mod config_fork_compose;
#[path = "../config_insert_step.rs"]
mod config_insert_step;
#[path = "../config_read.rs"]
mod config_read;
#[path = "../config_relocation_rollback.rs"]
mod config_relocation_rollback;
#[path = "../config_replace_remove_step.rs"]
mod config_replace_remove_step;
#[path = "../config_set_relocation_ack.rs"]
mod config_set_relocation_ack;
#[path = "../doctype_authoring_fences.rs"]
mod doctype_authoring_fences;
#[path = "../flow13_acceptance.rs"]
mod flow13_acceptance;
#[path = "../flow13_contract_and_severity.rs"]
mod flow13_contract_and_severity;
#[path = "../flow18_acceptance.rs"]
mod flow18_acceptance;
#[path = "../flow19_planning_encode.rs"]
mod flow19_planning_encode;
#[path = "../flow41_acceptance.rs"]
mod flow41_acceptance;
#[path = "../flow44_acceptance.rs"]
mod flow44_acceptance;
#[path = "../gate_coverage_fence.rs"]
mod gate_coverage_fence;
#[path = "../illustrative_disclaimer_scope.rs"]
mod illustrative_disclaimer_scope;
#[path = "../ingest.rs"]
mod ingest;
#[path = "../ingest_finding_keys.rs"]
mod ingest_finding_keys;
#[path = "../ingest_flow12.rs"]
mod ingest_flow12;
#[path = "../maps_to_test_caveat_fence.rs"]
mod maps_to_test_caveat_fence;
#[path = "../nested_add_item_on_create.rs"]
mod nested_add_item_on_create;
#[path = "../nested_item_addressing.rs"]
mod nested_item_addressing;
#[path = "../pack_source_determinism.rs"]
mod pack_source_determinism;
#[path = "../placement_override.rs"]
mod placement_override;
#[path = "../planning_checklist_sanction.rs"]
mod planning_checklist_sanction;
#[path = "../read_back_fence.rs"]
mod read_back_fence;
#[path = "../root_knob_rules.rs"]
mod root_knob_rules;
#[path = "../set_kind_vocabulary.rs"]
mod set_kind_vocabulary;
#[path = "../slug_override.rs"]
mod slug_override;
#[path = "../slug_override_axis.rs"]
mod slug_override_axis;
#[path = "../stated_at_fence.rs"]
mod stated_at_fence;
#[path = "../untrackable_home_axis.rs"]
mod untrackable_home_axis;
