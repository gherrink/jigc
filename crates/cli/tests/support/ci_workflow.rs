//! The CI workflow's identity, as the fences over it name it — the repository root, the
//! workflow file, and the one CI-only leg's step.
//!
//! These lived in `manifest_freeze_fence.rs`, which owns the leg, and three other fences
//! imported them from that suite. Since 2026-10-06 the importers sit in two homes — the
//! crate's `tests/` and the repository's tooling suites outside `crates/`
//! ([dev-workflow.md](../../../../implementation/dev-workflow.md) → Gate) — and a suite
//! cannot be reached across group roots, so the shared part is here, where both roots'
//! `support` reaches it.

use std::path::{Path, PathBuf};

/// This repository's root — two levels above the `jigc` package, whichever home the
/// calling suite's file sits in: every group root is a target of that package.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

/// The workflow that runs the gate's legs.
pub const CI_WORKFLOW: &str = ".github/workflows/ci.yml";

/// The manifest-freeze fence's **named** step — named, because a fence buried inside
/// another step's script is a fence nobody can find in a red run's log.
pub const CI_STEP_NAME: &str = "manifest-freeze fence";

/// The argv that step runs, verbatim. `--ignored --exact` is what makes the CI-only
/// sub-decision mechanically true in **both** directions: `cargo test` skips the arm
/// locally, and only this line un-skips it.
pub const CI_STEP_RUN: &str =
    "cargo test -p jigc --test g_finalize manifest_freeze_fence::live -- --ignored --exact";
