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
use engine::finding::Severity;
use engine::probe::{
    EffectiveStateSnapshot, ProbeRequest, ProbeRun, ProbeRunStatus, RootKind, ingest_probe_run,
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
            engine::tempname::unique_nanos(),
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
    let snapshot = EffectiveStateSnapshot::new(
        sample_anchors(),
        root.path().to_path_buf(),
        RootKind::WorkingTree,
    );
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
    let snapshot =
        EffectiveStateSnapshot::new(anchors, root.path().to_path_buf(), RootKind::WorkingTree);
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
    let snapshot =
        EffectiveStateSnapshot::new(anchors, root.path().to_path_buf(), RootKind::WorkingTree);
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

// ----- M27 follow-up: per-language gate coverage over the REAL probe binary -----
//
// The probe's behavioral unit suite lives in a *detached* workspace the root gate skips
// (see `doc_code_probe_suite.rs`). These cases drive the actual probe binary over the wire
// (same build → write-fixture → snapshot → invoke → ingest path as the helpers above) so a
// regression in the multi-language guarantees (the per-language allowlists, the
// `unsupported-language` advisory, the non-Rust `maps-to-test` truth table) is caught by the
// outer `cargo test` directly, not only by the meta-test.

/// Drive the probe over the invoker against a working tree holding ONE file at an arbitrary
/// relative path (so a `.ts`/`.js`/`.py`/`.php`/`.sh`/`.css` fixture can be cited), returning
/// the ingested findings. Generalizes [`run_symbol_fixture`] (which is `.rs`-only).
fn run_lang_fixture(
    rel_path: &str,
    file_body: &str,
    anchors: Vec<TargetAnchor>,
) -> Vec<engine::finding::Finding> {
    let probe = build_doc_code_probe();

    let root = TempDir::new("langtree");
    let abs = root.path().join(rel_path);
    if let Some(parent) = abs.parent() {
        fs::create_dir_all(parent).expect("mk fixture dir");
    }
    fs::write(&abs, file_body.as_bytes()).expect("write fixture file");

    let scratch = TempDir::new("langscratch");
    let snapshot =
        EffectiveStateSnapshot::new(anchors, root.path().to_path_buf(), RootKind::WorkingTree);
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

/// A `symbol-exists` anchor at `<rel_path>#<symbol>` (the common per-language shape).
fn symbol_exists_anchor(rel_path: &str, symbol: &str) -> TargetAnchor {
    TargetAnchor {
        address: "adr:x#status/cites-code".to_string(),
        anchor_value: format!("{rel_path}#{symbol}"),
        check_id: "symbol-exists".to_string(),
    }
}

/// TypeScript: an exported `abstract class` RESOLVES (present → no finding); a function-local
/// `let` does NOT (the value-position tightening — anchor it, expect a block since it is not
/// citable); a vanished symbol BLOCKS.
#[test]
fn ts_abstract_class_resolves_local_let_and_vanished_block() {
    const TS: &str = "\
export abstract class Shape {
    area(): number {
        return 0;
    }
}

function freeFn(): number {
    let localVar = 1;
    return localVar;
}
";
    // Present exported abstract class → no finding.
    assert!(
        run_lang_fixture(
            "src/a.ts",
            TS,
            vec![symbol_exists_anchor("src/a.ts", "Shape")]
        )
        .is_empty(),
        "an exported abstract class resolves",
    );
    // A function-local `let` is value-position, not citable → one block.
    let local = run_lang_fixture(
        "src/a.ts",
        TS,
        vec![symbol_exists_anchor("src/a.ts", "localVar")],
    );
    assert_eq!(
        local.len(),
        1,
        "a function-local let is not citable: {local:?}"
    );
    assert_eq!(local[0].code, "doc-code.symbol-exists");
    // A vanished symbol → one block.
    let gone = run_lang_fixture(
        "src/a.ts",
        TS,
        vec![symbol_exists_anchor("src/a.ts", "ShapeRenamed")],
    );
    assert_eq!(gone.len(), 1, "a vanished TS symbol blocks: {gone:?}");
    assert_eq!(gone[0].code, "doc-code.symbol-exists");
}

/// JavaScript: a real symbol resolves; a vanished one blocks.
#[test]
fn js_real_symbol_resolves_vanished_blocks() {
    const JS: &str = "export function freeFn() {\n    return 1;\n}\n";
    assert!(
        run_lang_fixture(
            "src/a.js",
            JS,
            vec![symbol_exists_anchor("src/a.js", "freeFn")]
        )
        .is_empty(),
        "a real JS function resolves",
    );
    let gone = run_lang_fixture(
        "src/a.js",
        JS,
        vec![symbol_exists_anchor("src/a.js", "freeFnRenamed")],
    );
    assert_eq!(gone.len(), 1, "a vanished JS symbol blocks: {gone:?}");
    assert_eq!(gone[0].code, "doc-code.symbol-exists");
}

/// Python: a nested `def` resolves at any nesting; a vanished symbol blocks.
#[test]
fn py_nested_def_resolves_vanished_blocks() {
    const PY: &str = "\
def free_fn():
    def nested_fn():
        return 1
    return nested_fn()
";
    assert!(
        run_lang_fixture(
            "svc/a.py",
            PY,
            vec![symbol_exists_anchor("svc/a.py", "nested_fn")]
        )
        .is_empty(),
        "a nested Python def resolves",
    );
    let gone = run_lang_fixture(
        "svc/a.py",
        PY,
        vec![symbol_exists_anchor("svc/a.py", "nested_fn_renamed")],
    );
    assert_eq!(gone.len(), 1, "a vanished Python symbol blocks: {gone:?}");
    assert_eq!(gone[0].code, "doc-code.symbol-exists");
}

/// PHP: a `trait` (and an `interface`) RESOLVE — the review's false-block fix; a vanished
/// symbol blocks.
#[test]
fn php_trait_and_interface_resolve_vanished_blocks() {
    const PHP: &str = "\
<?php

interface Drawable {}

trait Loggable {}
";
    for sym in ["Drawable", "Loggable"] {
        assert!(
            run_lang_fixture(
                "src/a.php",
                PHP,
                vec![symbol_exists_anchor("src/a.php", sym)]
            )
            .is_empty(),
            "PHP `{sym}` (trait/interface) resolves",
        );
    }
    let gone = run_lang_fixture(
        "src/a.php",
        PHP,
        vec![symbol_exists_anchor("src/a.php", "LoggableRenamed")],
    );
    assert_eq!(gone.len(), 1, "a vanished PHP symbol blocks: {gone:?}");
    assert_eq!(gone[0].code, "doc-code.symbol-exists");
}

/// bash: a `function` resolves; a called-but-never-defined name BLOCKS (not a false-pass —
/// the functions-only allowlist closes the field-walk over-match).
#[test]
fn bash_function_resolves_called_but_undefined_blocks() {
    const BASH: &str = "\
#!/usr/bin/env bash

deploy() {
    echo deploying
}

called_but_undefined
";
    assert!(
        run_lang_fixture(
            "scripts/run.sh",
            BASH,
            vec![symbol_exists_anchor("scripts/run.sh", "deploy")]
        )
        .is_empty(),
        "a bash function resolves",
    );
    let called = run_lang_fixture(
        "scripts/run.sh",
        BASH,
        vec![symbol_exists_anchor(
            "scripts/run.sh",
            "called_but_undefined",
        )],
    );
    assert_eq!(
        called.len(),
        1,
        "a called-but-undefined name blocks (not a false-pass): {called:?}",
    );
    assert_eq!(called[0].code, "doc-code.symbol-exists");
}

/// The `unsupported-language` advisory: a `#symbol` on a `.pl` file (perl — off-roadmap, no
/// shipped grammar) emits EXACTLY ONE finding with `severity: advisory`,
/// `check: unsupported-language`, `code: doc-code.unsupported-language` — NOT a block, NOT
/// silent.
#[test]
fn unsupported_language_perl_emits_one_advisory() {
    let findings = run_lang_fixture(
        "script.pl",
        "#!/usr/bin/perl\nsub thing { }\n",
        vec![symbol_exists_anchor("script.pl", "thing")],
    );
    assert_eq!(findings.len(), 1, "exactly one finding: {findings:?}");
    assert_eq!(findings[0].severity, Severity::Advisory);
    assert_eq!(findings[0].check, "unsupported-language");
    assert_eq!(findings[0].code, "doc-code.unsupported-language");
}

/// The M28 keystone through the binary: a `.css#selector` resolves through the CSS grammar —
/// a present selector emits NO finding; a vanished one BLOCKS with one
/// `doc-code.symbol-exists` (CSS is grammared, no longer the `unsupported-language` advisory).
#[test]
fn css_selector_resolves_present_blocks_vanished() {
    const CSS: &str = ".btn { color: red; }\n";
    assert!(
        run_lang_fixture(
            "styles.css",
            CSS,
            vec![symbol_exists_anchor("styles.css", "btn")]
        )
        .is_empty(),
        "a present CSS selector resolves",
    );
    let gone = run_lang_fixture(
        "styles.css",
        CSS,
        vec![symbol_exists_anchor("styles.css", "btn_renamed")],
    );
    assert_eq!(gone.len(), 1, "a vanished CSS selector blocks: {gone:?}");
    assert_eq!(gone[0].severity, Severity::Blocking);
    assert_eq!(gone[0].code, "doc-code.symbol-exists");
}

/// The non-Rust `criterion-maps-to-test` truth table through the binary: a non-Rust
/// `maps-to-test` anchor with the symbol PRESENT → one `is-a-test-unverifiable` advisory (the
/// grammar resolved the symbol, so NOT the `unsupported-language` no-grammar fork; no block);
/// with the symbol ABSENT → one blocking `doc-code.criterion-maps-to-test` (no advisory). Never
/// both.
#[test]
fn non_rust_maps_to_test_truth_table_through_binary() {
    const PY_TEST: &str = "def test_rate_limit():\n    assert True\n";
    let maps = |symbol: &str| TargetAnchor {
        address: "spec:rate-limiting#criteria/limit/maps-to-test".to_string(),
        anchor_value: format!("test_limit.py#{symbol}"),
        check_id: "criterion-maps-to-test".to_string(),
    };

    // PRESENT → one advisory, no block.
    let present = run_lang_fixture("test_limit.py", PY_TEST, vec![maps("test_rate_limit")]);
    assert_eq!(
        present.len(),
        1,
        "present: exactly one finding: {present:?}"
    );
    assert_eq!(present[0].severity, Severity::Advisory);
    assert_eq!(present[0].code, "doc-code.is-a-test-unverifiable");
    assert!(
        !present[0].message.contains("no grammar"),
        "the resolved-symbol advisory must not lie 'no grammar': {}",
        present[0].message,
    );

    // ABSENT → one block, no advisory.
    let absent = run_lang_fixture("test_limit.py", PY_TEST, vec![maps("vanished")]);
    assert_eq!(absent.len(), 1, "absent: exactly one finding: {absent:?}");
    assert_eq!(absent[0].severity, Severity::Blocking);
    assert_eq!(absent[0].code, "doc-code.criterion-maps-to-test");
}

// ----- M27 follow-up: determinism / order-invariance over the REAL probe binary -----

/// Two dangling anchors in different files/addresses are emitted in a STABLE address order
/// regardless of filesystem mtime order: drive the probe, capture the emitted findings'
/// addresses, then drive it AGAIN with the fixture files touched in the OPPOSITE order
/// (different mtimes) — the emitted findings must be byte-identical and in the same address
/// order ("findings in stable address order, not filesystem order"). The red→green discipline
/// (increment-workflow.md hardening #7) demands ≥2 divergent orders.
#[test]
fn doc_code_findings_are_address_ordered_not_filesystem_ordered() {
    // Two source files, each with a dangling symbol anchor, at two distinct addresses.
    let anchors = || {
        vec![
            symbol_exists_anchor_at(
                "zeta.ts",
                "Vanished",
                "arch-doc:g#components/aaa/implemented-by",
            ),
            symbol_exists_anchor_at(
                "alpha.ts",
                "Gone",
                "arch-doc:g#components/zzz/implemented-by",
            ),
        ]
    };

    // Run A: write alpha.ts first, then zeta.ts (alpha older).
    let run_a = run_two_file_fixture(
        &[
            ("alpha.ts", "export const a = 1;\n"),
            ("zeta.ts", "export const z = 1;\n"),
        ],
        anchors(),
    );
    // Run B: write zeta.ts first, then alpha.ts (the OPPOSITE mtime order).
    let run_b = run_two_file_fixture(
        &[
            ("zeta.ts", "export const z = 1;\n"),
            ("alpha.ts", "export const a = 1;\n"),
        ],
        anchors(),
    );

    // Both surface exactly the two dangling anchors (guard against a vacuous pass).
    assert_eq!(run_a.len(), 2, "two dangling anchors block: {run_a:?}");
    let addrs_a: Vec<_> = run_a
        .iter()
        .map(|f| f.location.as_ref().and_then(|l| l.address.clone()))
        .collect();
    let addrs_b: Vec<_> = run_b
        .iter()
        .map(|f| f.location.as_ref().and_then(|l| l.address.clone()))
        .collect();
    assert_eq!(
        addrs_a, addrs_b,
        "findings must be in the same stable address order regardless of fixture write/mtime order",
    );
}

/// A `symbol-exists` anchor with an explicit target address (the order-invariance test pins
/// addresses, so it cannot reuse the single-address [`symbol_exists_anchor`]).
fn symbol_exists_anchor_at(rel_path: &str, symbol: &str, address: &str) -> TargetAnchor {
    TargetAnchor {
        address: address.to_string(),
        anchor_value: format!("{rel_path}#{symbol}"),
        check_id: "symbol-exists".to_string(),
    }
}

/// Drive the probe over a working tree holding TWO files written in the GIVEN order (so their
/// mtimes differ run-to-run), returning the ingested findings.
fn run_two_file_fixture(
    files: &[(&str, &str)],
    anchors: Vec<TargetAnchor>,
) -> Vec<engine::finding::Finding> {
    let probe = build_doc_code_probe();

    let root = TempDir::new("ordertree");
    for (rel, body) in files {
        let abs = root.path().join(rel);
        if let Some(parent) = abs.parent() {
            fs::create_dir_all(parent).expect("mk fixture dir");
        }
        fs::write(&abs, body.as_bytes()).expect("write fixture file");
    }

    let scratch = TempDir::new("orderscratch");
    let snapshot =
        EffectiveStateSnapshot::new(anchors, root.path().to_path_buf(), RootKind::WorkingTree);
    let snapshot_path = scratch.path().join("snapshot.json");
    fs::write(
        &snapshot_path,
        serde_json::to_vec(&snapshot).expect("serialize snapshot"),
    )
    .expect("write snapshot");

    let request = ProbeRequest::new(
        "doc-code",
        "arch-doc:g#components/aaa/implemented-by",
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
