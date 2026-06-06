//! M10 inc-4 / T1 — the dev pack's `doc-code` probe executable skeleton, proven on a
//! fixture **over the proven seam + invoker** (inc 3).
//!
//! The probe is a standalone program living **outside** the workspace
//! ([module-layout.md](../../../implementation/module-layout.md) → Probe boundary). This
//! test builds it (`cargo build --manifest-path …`), then drives it the way the engine
//! will: serialize a crafted [`EffectiveStateSnapshot`] to a temp file, build a
//! [`ProbeRequest`] carrying its path-ref, hand the request to the CLI invoker
//! ([`cli::invoke::invoke_probe`]), and **ingest** the invoker's raw outcome through the
//! engine's [`engine::probe::ingest_probe_run`] (the inc-3 ingestion path).
//!
//! T1 scope: file-existence resolution (the `<path>` before any `#`). A snapshot with
//! one anchor at a **present** file and one at an **absent** file ingests to **exactly
//! one** `doc-code` finding (the absent one), with **no** `pack-probe-integrity`
//! meta-finding (the probe is well-behaved: exit 0, valid JSON). A **bare** `<path>`
//! anchor (no `#symbol`) at a present file passes — bare-path is the file-existence
//! check. No tree-sitter / symbol resolution is exercised (T2/T3).

use cli::invoke::{self, ProbeOutcome, ProbeStatus};
use engine::probe::{
    EffectiveStateSnapshot, ProbeRequest, ProbeRun, ProbeRunStatus, ingest_probe_run,
};
use engine::target_surface::TargetAnchor;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-doc-code-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Build the `doc-code` probe executable (a program **outside** the workspace) and
/// return its path. Built once per test via its own manifest, into its own target dir
/// so it never collides with the workspace build.
fn build_doc_code_probe() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack/probes/doc-code/Cargo.toml");
    let target_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack/probes/doc-code/target");
    let out = std::process::Command::new(env!("CARGO"))
        .arg("build")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--target-dir")
        .arg(&target_dir)
        .output()
        .expect("invoke cargo build for the doc-code probe");
    assert!(
        out.status.success(),
        "cargo build failed for the doc-code probe:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    target_dir.join("debug/doc-code")
}

/// Translate the CLI invoker's raw outcome into the engine's [`ProbeRun`] (the inc-3
/// seam between the CLI invoker and the engine's ingestion).
fn into_run(outcome: ProbeOutcome) -> ProbeRun {
    let status = match outcome.status {
        ProbeStatus::Exited { code } => ProbeRunStatus::Exited { code },
        ProbeStatus::TimedOut => ProbeRunStatus::TimedOut,
    };
    ProbeRun {
        stdout: outcome.stdout,
        status,
    }
}

/// A present and an absent file, plus a **bare-path** anchor at a present file — the
/// fixture for T1. The first anchor (`symbol-exists`) points at a file that EXISTS, the
/// second (`criterion-maps-to-test`) at a file that does NOT, the third is a bare path
/// (no `#`) at a present file.
fn sample_anchors() -> Vec<TargetAnchor> {
    vec![
        TargetAnchor {
            address: "adr:single-node-cache#status/cites-code".to_string(),
            anchor_value: "crates/engine/src/present.rs#some_symbol".to_string(),
            check_id: "symbol-exists".to_string(),
        },
        TargetAnchor {
            address: "spec:rate-limiting#criteria/rate-limit/maps-to-test".to_string(),
            anchor_value: "crates/engine/src/missing.rs#nope".to_string(),
            check_id: "criterion-maps-to-test".to_string(),
        },
        TargetAnchor {
            address: "adr:single-node-cache#status/bare-path".to_string(),
            anchor_value: "crates/engine/src/present.rs".to_string(),
            check_id: "symbol-exists".to_string(),
        },
    ]
}

/// The done-criterion: the probe, driven over the invoker + ingested through the engine,
/// emits **exactly one** `doc-code` finding (for the absent file), **no** meta-finding,
/// and the bare-path anchor at a present file passes.
#[test]
fn doc_code_probe_blocks_missing_file_passes_present_and_bare_path() {
    let probe = build_doc_code_probe();

    // A working tree where `present.rs` EXISTS and `missing.rs` does not.
    let root = TempDir::new("tree");
    fs::create_dir_all(root.path().join("crates/engine/src")).expect("mk tree");
    fs::write(
        root.path().join("crates/engine/src/present.rs"),
        b"fn some_symbol() {}\n",
    )
    .expect("present file");

    // Materialize + serialize the snapshot to a temp file the request points at.
    let scratch = TempDir::new("scratch");
    let snapshot = EffectiveStateSnapshot::new(sample_anchors(), root.path().to_path_buf());
    let snapshot_path = scratch.path().join("snapshot.json");
    fs::write(
        &snapshot_path,
        serde_json::to_vec(&snapshot).expect("serialize snapshot"),
    )
    .expect("write snapshot");

    let request = ProbeRequest::new(
        "doc-code",
        "spec:rate-limiting#criteria/rate-limit/maps-to-test",
        snapshot_path,
        serde_json::Map::new(),
    );
    let request_bytes = serde_json::to_vec(&request).expect("serialize request");

    let outcome = invoke::invoke_probe(&probe, &request_bytes, Duration::from_secs(30))
        .expect("invoker drives the doc-code probe");

    assert_eq!(
        outcome.status,
        ProbeStatus::Exited { code: Some(0) },
        "a well-behaved probe exits 0: stdout={}",
        String::from_utf8_lossy(&outcome.stdout),
    );

    let findings = ingest_probe_run("doc-code", &into_run(outcome));

    assert!(
        findings.iter().all(|f| f.probe != "pack-probe-integrity"),
        "a well-behaved probe synthesizes no meta-finding: {findings:?}",
    );
    assert_eq!(
        findings.len(),
        1,
        "exactly the absent file surfaces (present + bare-path pass): {findings:?}",
    );
    let finding = &findings[0];
    assert_eq!(finding.code, "doc-code.criterion-maps-to-test");
    assert_eq!(
        finding.location.as_ref().and_then(|l| l.address.as_deref()),
        Some("spec:rate-limiting#criteria/rate-limit/maps-to-test"),
        "the finding is keyed on the absent anchor's address",
    );
}
