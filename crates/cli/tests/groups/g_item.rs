// GENERATED GROUP ROOT — see implementation/dev-workflow.md (test target consolidation).
// Each suite below keeps its own file; this root only makes them one cargo target,
// so a source change relinks ~10 test binaries instead of 252.

#[path = "../support/mod.rs"]
mod support;

#[path = "../carryover_gate.rs"]
mod carryover_gate;
#[path = "../corpus_migration.rs"]
mod corpus_migration;
#[path = "../corpus_migration_backstop.rs"]
mod corpus_migration_backstop;
#[path = "../describe.rs"]
mod describe;
#[path = "../doc_code_probe_suite.rs"]
mod doc_code_probe_suite;
#[path = "../flow15_dogfood_walk.rs"]
mod flow15_dogfood_walk;
#[path = "../flow21_measured_run.rs"]
mod flow21_measured_run;
#[path = "../flow39_placement_layout.rs"]
mod flow39_placement_layout;
#[path = "../flow46_acceptance.rs"]
mod flow46_acceptance;
#[path = "../item_anchor_slug_word_aware.rs"]
mod item_anchor_slug_word_aware;
#[path = "../item_authoring_acceptance.rs"]
mod item_authoring_acceptance;
#[path = "../item_region_boundary.rs"]
mod item_region_boundary;
#[path = "../item_region_shape_space.rs"]
mod item_region_shape_space;
#[path = "../item_slot_ceiling_axis.rs"]
mod item_slot_ceiling_axis;
#[path = "../item_slot_corruption_acceptance.rs"]
mod item_slot_corruption_acceptance;
#[path = "../no_such_task_route.rs"]
mod no_such_task_route;
#[path = "../probe_invoker.rs"]
mod probe_invoker;
#[path = "../retitle_item.rs"]
mod retitle_item;
#[path = "../retitle_item_milestone_record.rs"]
mod retitle_item_milestone_record;
#[path = "../set_field_absent.rs"]
mod set_field_absent;
#[path = "../set_field_bracket_reject.rs"]
mod set_field_bracket_reject;
#[path = "../set_field_list_overwrite.rs"]
mod set_field_list_overwrite;
#[path = "../set_field_unset.rs"]
mod set_field_unset;
#[path = "../singleton_running_doc.rs"]
mod singleton_running_doc;
#[path = "../suppression_fence.rs"]
mod suppression_fence;
#[path = "../task_bind.rs"]
mod task_bind;
#[path = "../task_lifecycle.rs"]
mod task_lifecycle;
#[path = "../task_list.rs"]
mod task_list;
