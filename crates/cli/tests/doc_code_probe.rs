//! M10 inc-4 / T1 — the dev pack's `doc-code` probe executable skeleton, proven on a
//! fixture **over the proven seam + invoker** (inc 3).
//!
//! The probe is bundled in `jigc` and run by **self-exec** — `jigc __probe doc-code
//! --build <version>` ([module-layout.md](../../../implementation/module-layout.md) →
//! Probe boundary, the bundled probe; M54 S4). This test runs the built `jigc` as that
//! probe and drives it the way the engine will: serialize a crafted [`EffectiveStateSnapshot`] to a temp file, build a
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

/// The program an in-process caller runs the bundled probe from: the built `jigc`
/// (`CARGO_BIN_EXE_jigc`), paired at every call with [`invoke::doc_code_probe_args`] —
/// the in-process arm of `doc_code_command` (M54 S4). A test process's own
/// `current_exe()` is the test binary, so it names the real `jigc` directly.
fn doc_code_probe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_jigc"))
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
        stderr: outcome.stderr,
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
    let probe = doc_code_probe();

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

    let outcome = invoke::invoke_probe(
        &probe,
        &invoke::doc_code_probe_args(),
        &request_bytes,
        Duration::from_secs(30),
    )
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
    let probe = doc_code_probe();

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

    let outcome = invoke::invoke_probe(
        &probe,
        &invoke::doc_code_probe_args(),
        &request_bytes,
        Duration::from_secs(60),
    )
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
    let probe = doc_code_probe();

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

    let outcome = invoke::invoke_probe(
        &probe,
        &invoke::doc_code_probe_args(),
        &request_bytes,
        Duration::from_secs(60),
    )
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
    let probe = doc_code_probe();

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

    let outcome = invoke::invoke_probe(
        &probe,
        &invoke::doc_code_probe_args(),
        &request_bytes,
        Duration::from_secs(60),
    )
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
    let probe = doc_code_probe();

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

    let outcome = invoke::invoke_probe(
        &probe,
        &invoke::doc_code_probe_args(),
        &request_bytes,
        Duration::from_secs(60),
    )
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

// ----- M46 inc-7 / T1: the closure-framework cell, over the REAL probe binary -----

/// The N-5 fixture, verbatim: a vitest file whose only test is registered by a **closure**,
/// so the test's name is present in the file's text and declares no unit. The same shape
/// three frameworks share (PHP/Pest, TS/vitest, `node:test`), reported by three parties.
const CLOSURE_TEST_TS: &str = "\
import { it, expect } from \"vitest\";

it('rejects a burst beyond the cap', () => {
    expect(true).toBe(true);
});
";

/// The wire-byte proof of the text-present cell: the real probe binary, driven over the
/// invoker + the engine's ingestion, emits a message that states the comparison it made (the
/// text occurs in the root it read; no declared unit matched) and a route that leads with the
/// **file-only fallback** and says what it buys. Asserted on the **emitted bytes**, not on a
/// reconstruction — this is the sentence an agent reads.
#[test]
fn closure_registered_test_name_reports_what_the_probe_compared_not_an_absence() {
    let probe = doc_code_probe();

    let root = TempDir::new("closuretree");
    fs::create_dir_all(root.path().join("tests")).expect("mk tests dir");
    fs::write(
        root.path().join("tests/rate_limit.test.ts"),
        CLOSURE_TEST_TS.as_bytes(),
    )
    .expect("write the closure fixture");

    let scratch = TempDir::new("closurescratch");
    // Task scope: the root the engine hands over is the **materialized index**, which is why
    // "the indexed blob contains it" is a fact about the tree the finding names.
    let snapshot = EffectiveStateSnapshot::new(
        vec![TargetAnchor {
            address: "spec:rate-limiting#criteria/burst/maps-to-test".to_string(),
            anchor_value: "tests/rate_limit.test.ts#rejects a burst beyond the cap".to_string(),
            check_id: "criterion-maps-to-test".to_string(),
        }],
        root.path().to_path_buf(),
        RootKind::StagedIndex,
    );
    let snapshot_path = scratch.path().join("snapshot.json");
    fs::write(
        &snapshot_path,
        serde_json::to_vec(&snapshot).expect("serialize snapshot"),
    )
    .expect("write snapshot");

    let request = ProbeRequest::new(
        "doc-code",
        "spec:rate-limiting#criteria/burst/maps-to-test",
        snapshot_path,
        serde_json::Map::new(),
    );
    let request_bytes = serde_json::to_vec(&request).expect("serialize request");

    let outcome = invoke::invoke_probe(
        &probe,
        &invoke::doc_code_probe_args(),
        &request_bytes,
        Duration::from_secs(60),
    )
    .expect("invoker drives the doc-code probe");
    assert_eq!(
        outcome.status,
        ProbeStatus::Exited { code: Some(0) },
        "a well-behaved probe exits 0: stdout={}",
        String::from_utf8_lossy(&outcome.stdout),
    );

    let findings = ingest_probe_run("doc-code", &into_run(outcome));
    assert_eq!(findings.len(), 1, "exactly one finding: {findings:?}");
    let finding = &findings[0];

    // The verdict does not move: the closure registration is still a non-resolution, and
    // `validation.md`'s sanction (a name living only in a string never resolves) keeps it
    // blocking, keyed and located exactly where it was.
    assert_eq!(finding.severity, Severity::Blocking);
    assert_eq!(finding.code, "doc-code.criterion-maps-to-test");
    assert_eq!(finding.check, "criterion-maps-to-test");
    assert_eq!(
        finding.location.as_ref().and_then(|l| l.address.as_deref()),
        Some("spec:rate-limiting#criteria/burst/maps-to-test"),
    );

    assert_eq!(
        finding.message,
        "anchor `tests/rate_limit.test.ts#rejects a burst beyond the cap` resolves to no symbol \
         (the text `rejects a burst beyond the cap` occurs in `tests/rate_limit.test.ts` in the \
         staged index, but declares nothing there — a name inside a string, a comment or a \
         framework registration is not a declared unit)",
        "the emitted message must state the comparison the probe made, never an absence the \
         bytes it parsed contradict",
    );
    assert_eq!(
        finding.route.as_deref(),
        Some(
            "cite `tests/rate_limit.test.ts` alone (drop `#rejects a burst beyond the cap`) — a \
             file-only anchor is accepted and buys the file's presence, not that a test exists; \
             or cite a unit the file declares (a function, class or method); if a declaration by \
             that name is on disk but unstaged, `git add` it — finalize adjudicates the staged \
             index, not the working tree"
        ),
        "the emitted route must lead with the repair that works, state what it does not buy, \
         and keep the staging clause the probe's evidence leaves possible",
    );
}

// ----- M50 inc-12 / T5: the anchor miss names the grammar (RC-m50 F-2) -----

/// The trial's fixture, rebuilt: a TypeScript module declaring exactly one unit. F-2's
/// three probes ran against a `src/pad.ts` shaped like this one.
const PAD_TS: &str = "\
export function pad(n: number): string {
    return String(n).padStart(2, \"0\");
}
";

/// The anchor grammar, in the one spelling every surface states it. `code_anchor_grammar_sites.rs`
/// fences the shipping homes; this literal is a test's expectation of the emitted bytes.
const ANCHOR_GRAMMAR: &str = "<repo-relative-path>[#<symbol>]";

/// Drive the **real probe binary** over the invoker + the engine's ingestion against a
/// staged index holding `src/pad.ts`, for one anchor value — task scope, because the task
/// gate is where the trial's worker typed the value and read the answer.
fn pad_ts_findings(anchor_value: &str) -> Vec<engine::finding::Finding> {
    let probe = doc_code_probe();

    let root = TempDir::new("padtree");
    fs::create_dir_all(root.path().join("src")).expect("mk src dir");
    fs::write(root.path().join("src/pad.ts"), PAD_TS.as_bytes()).expect("write the fixture");

    let scratch = TempDir::new("padscratch");
    let snapshot = EffectiveStateSnapshot::new(
        vec![TargetAnchor {
            address: "adr:probe#status/cites-code".to_string(),
            anchor_value: anchor_value.to_string(),
            check_id: "symbol-exists".to_string(),
        }],
        root.path().to_path_buf(),
        RootKind::StagedIndex,
    );
    let snapshot_path = scratch.path().join("snapshot.json");
    fs::write(
        &snapshot_path,
        serde_json::to_vec(&snapshot).expect("serialize snapshot"),
    )
    .expect("write snapshot");

    let request = ProbeRequest::new(
        "doc-code",
        "adr:probe#status/cites-code",
        snapshot_path,
        serde_json::Map::new(),
    );
    let request_bytes = serde_json::to_vec(&request).expect("serialize request");

    let outcome = invoke::invoke_probe(
        &probe,
        &invoke::doc_code_probe_args(),
        &request_bytes,
        Duration::from_secs(60),
    )
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

/// The wire-byte proof of F-2's three cells, in the order the trial's worker hit them: a
/// `file:line` value, the anchor that works, and a `#symbol` the file does not declare.
/// Asserted on the **emitted bytes** the engine ingested — the sentence an agent reads —
/// never on a reconstruction.
#[test]
fn a_file_line_value_names_the_grammar_and_the_symbol_side_is_unmoved() {
    // The value that works — the control. A grammar-naming message is worth nothing if the
    // grammar it names is not the one that resolves.
    let clean = pad_ts_findings("src/pad.ts#pad");
    assert!(clean.is_empty(), "`src/pad.ts#pad` resolves: {clean:?}");

    // The miss. `src/pad.ts` IS in the staged index, so "absent from the staged index" was a
    // true sentence about a reading nobody meant and a false one about the file the worker
    // cited — F-2's *"true and misleading"*.
    let miss = pad_ts_findings("src/pad.ts:5");
    assert_eq!(miss.len(), 1, "exactly one finding: {miss:?}");
    let miss = &miss[0];
    assert_eq!(miss.severity, Severity::Blocking, "the verdict is unmoved");
    assert_eq!(miss.code, "doc-code.symbol-exists");
    assert_eq!(miss.check, "symbol-exists");
    assert_eq!(
        miss.location.as_ref().and_then(|l| l.address.as_deref()),
        Some("adr:probe#status/cites-code"),
    );
    assert!(
        miss.message.contains(ANCHOR_GRAMMAR),
        "the emitted message must name the grammar: {}",
        miss.message,
    );
    assert_eq!(
        miss.message,
        "anchor `src/pad.ts:5` does not match the anchor grammar `<repo-relative-path>[#<symbol>]` \
         — `src/pad.ts` is a file in the staged index and the trailing `:5` is not part of an \
         anchor",
    );
    assert_eq!(
        miss.route.as_deref(),
        Some(
            "drop the trailing `:5` — an anchor is `src/pad.ts` alone (which buys the file's \
             presence, not that a test exists) or `src/pad.ts#<symbol>` naming a unit the file \
             declares"
        ),
        "the emitted route must be runnable as written: the edit that makes the value an anchor, \
         then both legal shapes",
    );

    // The sibling producer keeps its shipped wording byte-for-byte: this value DOES parse as
    // an anchor, so its miss is a symbol miss, not a grammar miss.
    let symbol = pad_ts_findings("src/pad.ts#pad.method");
    assert_eq!(symbol.len(), 1, "exactly one finding: {symbol:?}");
    let symbol = &symbol[0];
    assert_eq!(
        symbol.message,
        "anchor `src/pad.ts#pad.method` resolves to no symbol (`pad.method` is absent from \
         `src/pad.ts` in the staged index)",
    );
    assert_eq!(
        symbol.route.as_deref(),
        Some(
            "if the cited code is on disk but unstaged, `git add` it — finalize adjudicates the \
             staged index, not the working tree; otherwise update the citation to match the \
             renamed/moved code, or restore the cited symbol (e.g. revert the change)"
        ),
    );
    assert!(
        !symbol.message.contains(ANCHOR_GRAMMAR),
        "a value that parses as an anchor is not a grammar miss: {}",
        symbol.message,
    );
}

// ----- M54 Inc 2 T4: the self-exec argv, driven through the built binary -----

/// A real request over a one-anchor snapshot (a present symbol), written to `scratch`.
fn one_anchor_request(root: &Path, scratch: &Path) -> Vec<u8> {
    fs::create_dir_all(root.join("src")).expect("mk tree");
    fs::write(root.join("src/lib.rs"), b"pub fn alpha() {}\n").expect("write lib.rs");
    let snapshot = EffectiveStateSnapshot::new(
        vec![symbol_exists_anchor("src/lib.rs", "alpha")],
        root.to_path_buf(),
        RootKind::WorkingTree,
    );
    let snapshot_path = scratch.join("snapshot.json");
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
    serde_json::to_vec(&request).expect("serialize request")
}

/// Spawn the built `jigc` with `args`, feed it `request` on stdin, and collect its output.
fn run_jigc_with_stdin(args: &[String], request: &[u8]) -> std::process::Output {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the built jigc");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(request)
        .expect("write the request");
    child.wait_with_output().expect("wait for jigc")
}

/// `jigc` + the one args spelling (`__probe doc-code --build <own version>`) is the
/// `doc-code` probe: fed a real request it prints a valid response and exits 0.
#[test]
fn jigc_runs_as_the_doc_code_probe_on_its_own_build_id() {
    let root = TempDir::new("self-exec-tree");
    let scratch = TempDir::new("self-exec-scratch");
    let request = one_anchor_request(root.path(), scratch.path());

    let out = run_jigc_with_stdin(&invoke::doc_code_probe_args(), &request);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the self-exec probe exits 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let response: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("the self-exec probe prints valid JSON");
    assert_eq!(
        response["findings"],
        serde_json::json!([]),
        "a present symbol draws no finding: {response}",
    );
    assert!(
        response["schema_version"].is_u64(),
        "the response carries the wire schema version: {response}",
    );
}

/// A skewed `--build` (another `jigc` version asked this one for its probe) is refused:
/// non-zero exit, nothing on stdout, and a stderr reason naming **both** versions — the
/// text the invoker carries into the `probe-failure` finding.
#[test]
fn jigc_refuses_the_probe_argv_of_another_build() {
    let root = TempDir::new("skew-tree");
    let scratch = TempDir::new("skew-scratch");
    let request = one_anchor_request(root.path(), scratch.path());

    let mut args = invoke::doc_code_probe_args();
    *args.last_mut().expect("the args end in the build id") = "0.0.0-skew".to_owned();
    let out = run_jigc_with_stdin(&args, &request);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a skewed build id exits non-zero; stderr:\n{stderr}"
    );
    assert!(
        out.stdout.is_empty(),
        "a refused probe prints nothing on stdout: {:?}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("0.0.0-skew") && stderr.contains(env!("CARGO_PKG_VERSION")),
        "the refusal names both versions; stderr:\n{stderr}",
    );
}
