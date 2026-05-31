//! The `Probe` trait (`check(target, ctx) -> Vec<Finding>`, read-only) and the
//! engine-native in-process implementations.
//!
//! The subprocess (pack-probe) seam is post-MVP; the trait must admit it with
//! zero engine change. See `design/validation.md` → the engine/probe boundary
//! and the pack-probe determinism contract.
