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
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("probes/doc-code/Cargo.toml");
    let target_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("probes/doc-code/target");
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

/// Drive the probe over the invoker + ingestion path against a working tree with one
/// real `.rs` file, returning the ingested findings.
fn run_symbol_fixture(
    file_body: &str,
    anchors: Vec<TargetAnchor>,
) -> Vec<engine::finding::Finding> {
    let probe = build_doc_code_probe();

    let root = TempDir::new("symtree");
    fs::create_dir_all(root.path().join("crates/engine/src")).expect("mk tree");
    fs::write(
        root.path().join("crates/engine/src/lib.rs"),
        file_body.as_bytes(),
    )
    .expect("write fixture .rs");

    let scratch = TempDir::new("symscratch");
    let snapshot = EffectiveStateSnapshot::new(anchors, root.path().to_path_buf());
    let snapshot_path = scratch.path().join("snapshot.json");
    fs::write(
        &snapshot_path,
        serde_json::to_vec(&snapshot).expect("serialize snapshot"),
    )
    .expect("write snapshot");

    let request = ProbeRequest::new(
        "doc-code",
        "adr:x#status/cites-code",
        snapshot_path,
        serde_json::Map::new(),
    );
    let request_bytes = serde_json::to_vec(&request).expect("serialize request");

    let outcome = invoke::invoke_probe(&probe, &request_bytes, Duration::from_secs(60))
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
    findings
}

/// A `symbol-exists` anchor whose `#symbol` names a real top-level `fn` in a present
/// `.rs` file resolves — **no** finding.
#[test]
fn symbol_exists_resolves_present_top_level_fn() {
    let findings = run_symbol_fixture(
        "fn alpha() {}\nfn beta() {}\n",
        vec![TargetAnchor {
            address: "adr:x#status/cites-code".to_string(),
            anchor_value: "crates/engine/src/lib.rs#beta".to_string(),
            check_id: "symbol-exists".to_string(),
        }],
    );
    assert!(
        findings.is_empty(),
        "a resolvable top-level fn yields no finding: {findings:?}",
    );
}

/// A `symbol-exists` anchor whose `#symbol` names no top-level symbol in a present file
/// (the symbol was deleted / renamed) → **one** blocking `doc-code.symbol-exists` finding
/// keyed on the target address.
#[test]
fn symbol_exists_blocks_deleted_symbol_in_present_file() {
    let findings = run_symbol_fixture(
        // `beta` was renamed to `gamma`; the anchor still cites `beta`.
        "fn alpha() {}\nfn gamma() {}\n",
        vec![TargetAnchor {
            address: "adr:single-node-cache#status/cites-code".to_string(),
            anchor_value: "crates/engine/src/lib.rs#beta".to_string(),
            check_id: "symbol-exists".to_string(),
        }],
    );
    assert_eq!(
        findings.len(),
        1,
        "a deleted symbol blocks once: {findings:?}",
    );
    let finding = &findings[0];
    assert_eq!(finding.code, "doc-code.symbol-exists");
    assert_eq!(
        finding.location.as_ref().and_then(|l| l.address.as_deref()),
        Some("adr:single-node-cache#status/cites-code"),
        "the finding names the target address",
    );
}

/// A `#symbol` anchor whose **file** is absent still blocks (file-existence subsumed —
/// no file means no AST to resolve the symbol against).
#[test]
fn symbol_exists_blocks_when_file_absent() {
    let findings = run_symbol_fixture(
        "fn alpha() {}\n",
        vec![TargetAnchor {
            address: "adr:x#status/cites-code".to_string(),
            anchor_value: "crates/engine/src/gone.rs#alpha".to_string(),
            check_id: "symbol-exists".to_string(),
        }],
    );
    assert_eq!(
        findings.len(),
        1,
        "an absent file blocks even with a #symbol: {findings:?}",
    );
    assert_eq!(findings[0].code, "doc-code.symbol-exists");
}

// ----- T3: the `criterion-maps-to-test` is-a-test predicate + determinism -----

/// Drive the probe over the invoker against a working tree with one real `.rs` file and
/// return the **raw `ProbeResponse` stdout bytes** (before ingestion) — what T3's
/// determinism assertion compares byte-for-byte across repeated runs. Reuses the same
/// build + snapshot + request shape as [`run_symbol_fixture`].
fn raw_stdout_for_fixture(file_body: &str, anchors: Vec<TargetAnchor>) -> Vec<u8> {
    let probe = build_doc_code_probe();

    let root = TempDir::new("dettree");
    fs::create_dir_all(root.path().join("crates/engine/src")).expect("mk tree");
    fs::write(
        root.path().join("crates/engine/src/lib.rs"),
        file_body.as_bytes(),
    )
    .expect("write fixture .rs");

    let scratch = TempDir::new("detscratch");
    let snapshot = EffectiveStateSnapshot::new(anchors, root.path().to_path_buf());
    let snapshot_path = scratch.path().join("snapshot.json");
    fs::write(
        &snapshot_path,
        serde_json::to_vec(&snapshot).expect("serialize snapshot"),
    )
    .expect("write snapshot");

    let request = ProbeRequest::new(
        "doc-code",
        "spec:x#criteria/c/maps-to-test",
        snapshot_path,
        serde_json::Map::new(),
    );
    let request_bytes = serde_json::to_vec(&request).expect("serialize request");

    let outcome = invoke::invoke_probe(&probe, &request_bytes, Duration::from_secs(60))
        .expect("invoker drives the doc-code probe");
    assert_eq!(
        outcome.status,
        ProbeStatus::Exited { code: Some(0) },
        "a well-behaved probe exits 0: stdout={}",
        String::from_utf8_lossy(&outcome.stdout),
    );
    outcome.stdout
}

/// A `maps-to-test` anchor whose `#symbol` names a real `#[test]`-attributed fn resolves
/// — **no** finding (symbol exists AND the is-a-test predicate holds).
#[test]
fn criterion_maps_to_test_resolves_real_test_fn() {
    let findings = run_symbol_fixture(
        "#[test]\nfn covers_limit() {}\nfn helper() {}\n",
        vec![TargetAnchor {
            address: "spec:rate-limiting#criteria/limit/maps-to-test".to_string(),
            anchor_value: "crates/engine/src/lib.rs#covers_limit".to_string(),
            check_id: "criterion-maps-to-test".to_string(),
        }],
    );
    assert!(
        findings.is_empty(),
        "a real #[test] fn satisfies criterion-maps-to-test: {findings:?}",
    );
}

/// A `maps-to-test` anchor whose `#symbol` resolves to a real fn that is **not** a
/// `#[test]` (a plain fn) → **one** blocking `doc-code.criterion-maps-to-test` finding:
/// the symbol exists but fails the is-a-test predicate.
#[test]
fn criterion_maps_to_test_blocks_non_test_fn() {
    let findings = run_symbol_fixture(
        // `covers_limit` exists but carries no `#[test]` attribute.
        "fn covers_limit() {}\n#[test]\nfn other() {}\n",
        vec![TargetAnchor {
            address: "spec:rate-limiting#criteria/limit/maps-to-test".to_string(),
            anchor_value: "crates/engine/src/lib.rs#covers_limit".to_string(),
            check_id: "criterion-maps-to-test".to_string(),
        }],
    );
    assert_eq!(
        findings.len(),
        1,
        "a non-#[test] fn fails criterion-maps-to-test: {findings:?}",
    );
    let finding = &findings[0];
    assert_eq!(finding.code, "doc-code.criterion-maps-to-test");
    assert_eq!(
        finding.location.as_ref().and_then(|l| l.address.as_deref()),
        Some("spec:rate-limiting#criteria/limit/maps-to-test"),
        "the finding names the target address",
    );
}

/// A `maps-to-test` anchor whose `#symbol` names a non-existent fn → **one** blocking
/// `doc-code.criterion-maps-to-test` finding (symbol existence is the floor of the check).
#[test]
fn criterion_maps_to_test_blocks_missing_fn() {
    let findings = run_symbol_fixture(
        "#[test]\nfn other() {}\n",
        vec![TargetAnchor {
            address: "spec:rate-limiting#criteria/limit/maps-to-test".to_string(),
            anchor_value: "crates/engine/src/lib.rs#covers_limit".to_string(),
            check_id: "criterion-maps-to-test".to_string(),
        }],
    );
    assert_eq!(
        findings.len(),
        1,
        "a non-existent fn fails criterion-maps-to-test: {findings:?}",
    );
    assert_eq!(findings[0].code, "doc-code.criterion-maps-to-test");
}

/// A `symbol-exists` anchor is **unaffected** by the is-a-test predicate: a plain
/// (non-`#[test]`) fn resolves for `symbol-exists` and yields no finding — the predicate
/// applies only to `criterion-maps-to-test`.
#[test]
fn symbol_exists_unaffected_by_test_predicate() {
    let findings = run_symbol_fixture(
        "fn covers_limit() {}\n",
        vec![TargetAnchor {
            address: "adr:x#status/cites-code".to_string(),
            anchor_value: "crates/engine/src/lib.rs#covers_limit".to_string(),
            check_id: "symbol-exists".to_string(),
        }],
    );
    assert!(
        findings.is_empty(),
        "symbol-exists ignores the is-a-test predicate: {findings:?}",
    );
}

/// Determinism by re-execution (increment-workflow.md → hardening #7): a static parse is
/// a pure function of (code + anchors), so the same request + snapshot driven **twice**
/// yields **byte-identical** `ProbeResponse` stdout. A mixed fixture (a passing test fn, a
/// failing non-test fn, a `symbol-exists` anchor) exercises every emission path.
#[test]
fn doc_code_response_is_byte_identical_across_runs() {
    let anchors = vec![
        TargetAnchor {
            address: "spec:rate-limiting#criteria/limit/maps-to-test".to_string(),
            anchor_value: "crates/engine/src/lib.rs#covers_limit".to_string(),
            check_id: "criterion-maps-to-test".to_string(),
        },
        TargetAnchor {
            address: "spec:rate-limiting#criteria/burst/maps-to-test".to_string(),
            anchor_value: "crates/engine/src/lib.rs#helper".to_string(),
            check_id: "criterion-maps-to-test".to_string(),
        },
        TargetAnchor {
            address: "adr:x#status/cites-code".to_string(),
            anchor_value: "crates/engine/src/lib.rs#helper".to_string(),
            check_id: "symbol-exists".to_string(),
        },
    ];
    let body = "#[test]\nfn covers_limit() {}\nfn helper() {}\n";

    let first = raw_stdout_for_fixture(body, anchors.clone());
    let second = raw_stdout_for_fixture(body, anchors);
    assert_eq!(
        first, second,
        "the same request + snapshot must yield byte-identical ProbeResponse stdout",
    );
    // Guard against a vacuous pass: the response must carry the one expected finding
    // (the non-test `helper` fails maps-to-test; the test fn and symbol-exists pass).
    let text = String::from_utf8(first).expect("utf8 stdout");
    assert!(
        text.contains("\"doc-code.criterion-maps-to-test\""),
        "the mixed fixture must surface the one blocking finding: {text}",
    );
}
