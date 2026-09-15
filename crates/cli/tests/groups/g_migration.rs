// GENERATED GROUP ROOT — see implementation/dev-workflow.md (test target consolidation).
// Each suite below keeps its own file; this root only makes them one cargo target,
// so a source change relinks ~10 test binaries instead of 252.

#[path = "../support/mod.rs"]
mod support;

#[path = "../arch_doc_acceptance.rs"]
mod arch_doc_acceptance;
#[path = "../arch_doc_components_materialize.rs"]
mod arch_doc_components_materialize;
#[path = "../cargo_install_probe.rs"]
mod cargo_install_probe;
#[path = "../design_altitude_schemas.rs"]
mod design_altitude_schemas;
#[path = "../flow14_acceptance.rs"]
mod flow14_acceptance;
#[path = "../flow24_changelog.rs"]
mod flow24_changelog;
#[path = "../flow38_adr_options.rs"]
mod flow38_adr_options;
#[path = "../flow8_override_default_warning.rs"]
mod flow8_override_default_warning;
#[path = "../migration_commit_autoprovision.rs"]
mod migration_commit_autoprovision;
#[path = "../migration_dateless_date.rs"]
mod migration_dateless_date;
#[path = "../migration_finalize_git_add.rs"]
mod migration_finalize_git_add;
#[path = "../migration_slot_fidelity.rs"]
mod migration_slot_fidelity;
#[path = "../multi_pack_acceptance.rs"]
mod multi_pack_acceptance;
#[path = "../owner_artifact_cause_axis.rs"]
mod owner_artifact_cause_axis;
#[path = "../owner_artifact_gate.rs"]
mod owner_artifact_gate;
#[path = "../owner_artifact_natural_order.rs"]
mod owner_artifact_natural_order;
#[path = "../owner_artifact_rollback.rs"]
mod owner_artifact_rollback;
#[path = "../record_decision_acceptance.rs"]
mod record_decision_acceptance;
#[path = "../single_task_changelog_gate.rs"]
mod single_task_changelog_gate;
#[path = "../spec_derived_from.rs"]
mod spec_derived_from;
#[path = "../spec_loop_prompts.rs"]
mod spec_loop_prompts;
#[path = "../spec_read_back_arms.rs"]
mod spec_read_back_arms;
#[path = "../surface_polish.rs"]
mod surface_polish;
#[path = "../upgrade.rs"]
mod upgrade;
#[path = "../upgrade_finding_keys.rs"]
mod upgrade_finding_keys;
#[path = "../upgrade_v1_v2.rs"]
mod upgrade_v1_v2;
