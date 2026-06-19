//! M27 follow-up (test-hardening) — make the **detached** `doc-code` probe's own unit
//! suite **gate-visible**.
//!
//! The probe is a standalone program in a *detached* workspace
//! (`crates/cli/probes/doc-code/`, an empty `[workspace]` table detaching it from the
//! engine/cli lock graph — [module-layout.md] → Probe boundary). Because it is **not** a
//! `workspace.members` entry, the root `cargo test` gate **never runs its unit suite** — the
//! probe's multi-language guarantees (the per-language node-kind allowlists, the
//! `unsupported-language` advisory, the non-Rust `maps-to-test` truth table, the shebang
//! dispatch) live in `src/resolve.rs` / `src/main.rs` `#[cfg(test)]` modules that the
//! root gate is blind to. A regression in any of them could ship green.
//!
//! This **meta-test** closes that gap: it shells the detached suite (`cargo test
//! --manifest-path probes/doc-code/Cargo.toml`) from inside the root gate and asserts it
//! succeeds, so a probe-unit regression now fails the root `cargo test`. The probe builds to
//! its **own** target dir (detached workspace — no target-dir contention with this outer
//! test), so the nested invocation is safe.

use std::process::Command;

/// Run the detached `doc-code` probe's own `cargo test` and assert success — surfacing its
/// streams on failure so a regression is legible from the outer gate's output.
#[test]
fn detached_doc_code_probe_unit_suite_passes() {
    // The cli crate's CARGO_MANIFEST_DIR is `crates/cli`, so the detached probe manifest is
    // `probes/doc-code/Cargo.toml` relative to it.
    let out = Command::new(env!("CARGO"))
        .args(["test", "--manifest-path", "probes/doc-code/Cargo.toml"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("invoke cargo test for the detached doc-code probe");

    assert!(
        out.status.success(),
        "the detached doc-code probe unit suite must pass (gate-visible here):\n\
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}
