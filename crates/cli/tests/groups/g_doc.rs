// GENERATED GROUP ROOT — see implementation/dev-workflow.md (test target consolidation).
// Each suite below keeps its own file; this root only makes them one cargo target,
// so a source change relinks ~10 test binaries instead of 252.

#[path = "../support/mod.rs"]
mod support;

#[path = "../address_parse_error_axis.rs"]
mod address_parse_error_axis;
#[path = "../author_batch_scaling.rs"]
mod author_batch_scaling;
#[path = "../copy_in_ack.rs"]
mod copy_in_ack;
#[path = "../doc_author.rs"]
mod doc_author;
#[path = "../doc_author_help.rs"]
mod doc_author_help;
#[path = "../doc_barrier.rs"]
mod doc_barrier;
#[path = "../doc_code_gate.rs"]
mod doc_code_gate;
#[path = "../doc_code_probe.rs"]
mod doc_code_probe;
#[path = "../doc_copy_in.rs"]
mod doc_copy_in;
#[path = "../doc_list.rs"]
mod doc_list;
#[path = "../doc_read_surface.rs"]
mod doc_read_surface;
#[path = "../doc_remove_item.rs"]
mod doc_remove_item;
#[path = "../doc_rename_in_task.rs"]
mod doc_rename_in_task;
#[path = "../doc_schema.rs"]
mod doc_schema;
#[path = "../doc_show.rs"]
mod doc_show;
#[path = "../doc_show_item_leaf.rs"]
mod doc_show_item_leaf;
#[path = "../doc_show_nested.rs"]
mod doc_show_nested;
#[path = "../doc_show_staged.rs"]
mod doc_show_staged;
#[path = "../doc_task_scope.rs"]
mod doc_task_scope;
#[path = "../doc_write.rs"]
mod doc_write;
#[path = "../doc_write_milestone_record.rs"]
mod doc_write_milestone_record;
#[path = "../flow30_acceptance.rs"]
mod flow30_acceptance;
#[path = "../flow31_acceptance.rs"]
mod flow31_acceptance;
#[path = "../implement_from_spec.rs"]
mod implement_from_spec;
#[path = "../increment_workflow_compose.rs"]
mod increment_workflow_compose;
#[path = "../roadmap_batch_author.rs"]
mod roadmap_batch_author;
#[path = "../roundtrip_registry_fence.rs"]
mod roundtrip_registry_fence;
#[path = "../schema_load_strictness.rs"]
mod schema_load_strictness;
#[path = "../schema_resolution_unified.rs"]
mod schema_resolution_unified;
#[path = "../uninstall.rs"]
mod uninstall;
#[path = "../unknown_subcommand_tip.rs"]
mod unknown_subcommand_tip;
#[path = "../write_title_divergence.rs"]
mod write_title_divergence;
