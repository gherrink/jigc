// GENERATED GROUP ROOT — see implementation/dev-workflow.md (test target consolidation).
// Each suite below keeps its own file; this root only makes them one cargo target,
// so a source change relinks ~10 test binaries instead of 252.

#[path = "../support/mod.rs"]
mod support;

#[path = "../author_write_contract.rs"]
mod author_write_contract;
#[path = "../batch_author_rerun.rs"]
mod batch_author_rerun;
#[path = "../catalog_shape_fence.rs"]
mod catalog_shape_fence;
#[path = "../compose_create_gates.rs"]
mod compose_create_gates;
#[path = "../compose_goldens.rs"]
mod compose_goldens;
#[path = "../compose_statefulness.rs"]
mod compose_statefulness;
#[path = "../compose_task_minted.rs"]
mod compose_task_minted;
#[path = "../create_or_update.rs"]
mod create_or_update;
#[path = "../dogfood_apparatus.rs"]
mod dogfood_apparatus;
#[path = "../dogfood_record_schema.rs"]
mod dogfood_record_schema;
#[path = "../flow16_acceptance.rs"]
mod flow16_acceptance;
#[path = "../flow20_completion_encode.rs"]
mod flow20_completion_encode;
#[path = "../flow40_acceptance.rs"]
mod flow40_acceptance;
#[path = "../flow45_acceptance.rs"]
mod flow45_acceptance;
#[path = "../optional_slot_guidance.rs"]
mod optional_slot_guidance;
#[path = "../orphan_detection.rs"]
mod orphan_detection;
#[path = "../prd_batch_author.rs"]
mod prd_batch_author;
#[path = "../prd_inverse_cardinality.rs"]
mod prd_inverse_cardinality;
#[path = "../precommit_hook_acceptance.rs"]
mod precommit_hook_acceptance;
#[path = "../read_surface_naming.rs"]
mod read_surface_naming;
#[path = "../relocate.rs"]
mod relocate;
#[path = "../schema_conformance_routing.rs"]
mod schema_conformance_routing;
#[path = "../schema_projection.rs"]
mod schema_projection;
#[path = "../schema_version_ahead_axis.rs"]
mod schema_version_ahead_axis;
#[path = "../slot_present_finding_keys.rs"]
mod slot_present_finding_keys;
#[path = "../start_compose.rs"]
mod start_compose;
#[path = "../start_explain.rs"]
mod start_explain;
#[path = "../start_orientation.rs"]
mod start_orientation;
#[path = "../start_resume.rs"]
mod start_resume;
#[path = "../stdin_form_naming.rs"]
mod stdin_form_naming;
#[path = "../superseding_decision.rs"]
mod superseding_decision;
