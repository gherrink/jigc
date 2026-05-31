//! The validation engine — read-only probes over a scope, severity via cascade,
//! the `finalize` gate.
//!
//! See `design/validation.md`. MVP engine-native probes: `workflow-refs`,
//! `file-state`, plus the synthetic `schema-conformance` integrity checks.
