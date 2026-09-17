// GENERATED GROUP ROOT — see implementation/dev-workflow.md (test target consolidation).
// Each suite below keeps its own file; this root only makes them one cargo target,
// so a source change relinks ~10 test binaries instead of 252.

#[path = "../support/mod.rs"]
mod support;

#[path = "../destroying_door_sibling_surfaces.rs"]
mod destroying_door_sibling_surfaces;
#[path = "../flow42_acceptance.rs"]
mod flow42_acceptance;
#[path = "../flow43_acceptance.rs"]
mod flow43_acceptance;
#[path = "../leftover_probe_fail_closed.rs"]
mod leftover_probe_fail_closed;
#[path = "../malformed_work_unit_id.rs"]
mod malformed_work_unit_id;
#[path = "../milestone.rs"]
mod milestone;
#[path = "../milestone_abort_survives.rs"]
mod milestone_abort_survives;
#[path = "../milestone_boundary_displacement.rs"]
mod milestone_boundary_displacement;
#[path = "../milestone_boundary_gate.rs"]
mod milestone_boundary_gate;
#[path = "../milestone_discard.rs"]
mod milestone_discard;
#[path = "../milestone_envelope_arm.rs"]
mod milestone_envelope_arm;
#[path = "../milestone_fanout_concurrency.rs"]
mod milestone_fanout_concurrency;
#[path = "../milestone_finalize_base_guard.rs"]
mod milestone_finalize_base_guard;
#[path = "../milestone_join_collision.rs"]
mod milestone_join_collision;
#[path = "../milestone_landed_attribution.rs"]
mod milestone_landed_attribution;
#[path = "../milestone_path_subject.rs"]
mod milestone_path_subject;
#[path = "../milestone_provision_handoff.rs"]
mod milestone_provision_handoff;
#[path = "../milestone_record_add_from_spec.rs"]
mod milestone_record_add_from_spec;
#[path = "../milestone_record_add_task.rs"]
mod milestone_record_add_task;
#[path = "../milestone_record_create.rs"]
mod milestone_record_create;
#[path = "../milestone_record_ff_rollback.rs"]
mod milestone_record_ff_rollback;
#[path = "../milestone_record_finalize.rs"]
mod milestone_record_finalize;
#[path = "../milestone_record_fresh_clone.rs"]
mod milestone_record_fresh_clone;
#[path = "../milestone_record_reconcile.rs"]
mod milestone_record_reconcile;
#[path = "../milestone_record_rollback.rs"]
mod milestone_record_rollback;
#[path = "../milestone_record_schema.rs"]
mod milestone_record_schema;
#[path = "../milestone_record_stale_base.rs"]
mod milestone_record_stale_base;
#[path = "../milestone_spawn_recorded_workflow.rs"]
mod milestone_spawn_recorded_workflow;
#[path = "../milestone_task_list_concurrency.rs"]
mod milestone_task_list_concurrency;
#[path = "../milestone_teardown_loss.rs"]
mod milestone_teardown_loss;
#[path = "../milestone_workflow_membership.rs"]
mod milestone_workflow_membership;
#[path = "../milestone_zero_contribution.rs"]
mod milestone_zero_contribution;
#[path = "../mint_doors.rs"]
mod mint_doors;
#[path = "../pinned_facts.rs"]
mod pinned_facts;
#[path = "../placement_acceptance.rs"]
mod placement_acceptance;
#[path = "../provision_leftover_guard.rs"]
mod provision_leftover_guard;
#[path = "../read_verb_acts_nothing.rs"]
mod read_verb_acts_nothing;
#[path = "../repo_relative_paths.rs"]
mod repo_relative_paths;
#[path = "../spawn_template_executes.rs"]
mod spawn_template_executes;
#[path = "../staged_prose_consent_axis.rs"]
mod staged_prose_consent_axis;
#[path = "../staged_snapshot.rs"]
mod staged_snapshot;
#[path = "../subtask_discard_record.rs"]
mod subtask_discard_record;
#[path = "../task_area_writer_registry.rs"]
mod task_area_writer_registry;
#[path = "../uninstall_workbench_subject.rs"]
mod uninstall_workbench_subject;
#[path = "../uninstall_worktree_guard.rs"]
mod uninstall_worktree_guard;
#[path = "../work_unit_id_axis.rs"]
mod work_unit_id_axis;
