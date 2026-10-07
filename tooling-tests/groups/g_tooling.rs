// GROUP ROOT — the repository's own tooling suites (implementation/dev-workflow.md → Gate,
// *Where the tooling's own tests live*).
//
// The suites below test this repository's tooling and records — a `dev/` script, the CI
// and release workflows, the stabilization harness, a doc's links, a ledger — and never
// the product: none drives `jigc`. They live outside `crates/` so that a change to one is
// not a change to a product path (DECISIONS.md → 2026-10-06, the human's ruling on the
// workflow's own tests). This root makes them one cargo target of the `jigc` package,
// declared once in `crates/cli/Cargo.toml`.
//
// A new tooling suite is a file beside the others and one `#[path = "../<name>.rs"]
// mod <name>;` line here — nothing under `crates/`.
// `crates/cli/tests/test_target_registration.rs` holds this directory to the same
// bijection as the crate's own `tests/`, finding it through the manifest's `[[test]]`
// path.

// The shared test support stays where the product's suites reach it too.
#[path = "../../crates/cli/tests/support/mod.rs"]
mod support;

#[path = "../ci_matrix_fence.rs"]
mod ci_matrix_fence;
#[path = "../dev_gate_report.rs"]
mod dev_gate_report;
#[path = "../dev_regression_set.rs"]
mod dev_regression_set;
#[path = "../dev_stabilize_canary.rs"]
mod dev_stabilize_canary;
#[path = "../dev_stabilize_probe.rs"]
mod dev_stabilize_probe;
#[path = "../dev_stabilize_record.rs"]
mod dev_stabilize_record;
#[path = "../dev_stabilize_step.rs"]
mod dev_stabilize_step;
#[path = "../doc_link_fence.rs"]
mod doc_link_fence;
#[path = "../dogfood_apparatus.rs"]
mod dogfood_apparatus;
#[path = "../evolution_posture_policy.rs"]
mod evolution_posture_policy;
#[path = "../illustrative_disclaimer_scope.rs"]
mod illustrative_disclaimer_scope;
#[path = "../ledger_entry_seven_discharged.rs"]
mod ledger_entry_seven_discharged;
#[path = "../ledger_record_truth.rs"]
mod ledger_record_truth;
#[path = "../merge_logs_fence.rs"]
mod merge_logs_fence;
#[path = "../placed_executable.rs"]
mod placed_executable;
#[path = "../record_foreign_arm.rs"]
mod record_foreign_arm;
#[path = "../release_pipeline_fence.rs"]
mod release_pipeline_fence;
#[path = "../seed_ledger.rs"]
mod seed_ledger;
#[path = "../stabilize_harness_fence.rs"]
mod stabilize_harness_fence;
#[path = "../stabilize_simulation.rs"]
mod stabilize_simulation;
#[path = "../workflow_action_runtime_fence.rs"]
mod workflow_action_runtime_fence;
