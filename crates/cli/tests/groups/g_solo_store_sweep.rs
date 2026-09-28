// GENERATED GROUP ROOT — see implementation/dev-workflow.md (test target consolidation).
//
// SOLO BY NECESSITY, not by grouping. This suite mutates **process-global** environment
// state, which is only sound while it is the sole test in its process. Sharing a target
// with other suites reintroduces exactly the cross-suite leak the consolidation first hit:
// `store_sweep_acceptance` pointed `JIGC_DOC_CODE_PROBE` at a deliberately-crashing probe
// and `severity_tuning`, co-resident for the first time, picked it up and failed. Keep
// this root at exactly one suite.

#[path = "../support/mod.rs"]
mod support;

#[path = "../store_sweep_acceptance.rs"]
mod store_sweep_acceptance;
