//! M18 inc-2 / T2 — the **acceptance**: the real `doc-code` probe driven over a
//! committed-store fixture through the T1 store-sweep entry point.
//!
//! T1 ([`engine::validate::validate_store`]) is the task-less, read-only store sweep —
//! enumerate every committed doc's `code-anchor` leaves, materialize the
//! `EffectiveStateSnapshot` to a temp scratch path, drive a CLI-supplied invoker, and
//! ingest the probe's findings + the `pack-probe-integrity.*` meta-findings into a
//! [`engine::result::ValidationReport`]. T1's own tests stubbed the invoker in-process.
//! **This test supplies the real one**: it builds the actual `doc-code` probe executable
//! (the `build_doc_code_probe` idiom, manifest at `pack/probes/doc-code`) and wraps
//! [`cli::invoke::invoke_probe`]`(`[`cli::invoke::doc_code_program`]`(), ..,
//! `[`cli::invoke::DOC_CODE_BUDGET`]`)` — a genuine subprocess over a snapshot the engine
//! materialized, exactly as production `jigc validate` will.
//!
//! It seeds a committed store — a `decisions/`-located `adr` carrying a `cites-code`
//! anchor (`symbol-exists`), a `specs/`-located `spec` carrying a criterion
//! `maps-to-test` anchor (`criterion-maps-to-test`), and an `architecture/`-located
//! `arch-doc` carrying a `components/<id>/implemented-by` anchor (`symbol-exists`) — over
//! a working tree of real `.rs` files. The `arch-doc` exercises the **shipped**
//! `arch-doc.yaml` schema (its `architecture/` location + schema-load interaction is the
//! one production code-anchor input not otherwise driven through the real sweep), and
//! asserts (`validation.md` → Store-scope re-validation; Blocking semantics):
//!
//! - **(a)** all cited symbols exist → no `doc-code` content finding, `has_blocking()`
//!   false;
//! - **(b)** a cited symbol renamed/deleted → exactly one blocking
//!   `doc-code.symbol-exists` / `criterion-maps-to-test` content finding, keyed on that
//!   anchor's address;
//! - **(c)** an injected probe failure (absent program / non-zero exit) → exactly one
//!   blocking `pack-probe-integrity.*` meta-finding (the input to inc-3's exit rule).
//!
//! All three cases share **one** `#[test]` so the `JIGC_DOC_CODE_PROBE` override (the
//! documented dev/test probe-path knob, `invoke.rs`) is mutated **sequentially** — the
//! Rust test harness runs `#[test]` fns in parallel, and that env var is process-global,
//! so splitting the cases into separate tests would race the override. No other test
//! reads `JIGC_DOC_CODE_PROBE`, so a single sequential test is collision-free.

use cli::invoke::{self, DOC_CODE_BUDGET, ProbeOutcome, ProbeStatus, doc_code_program};
use engine::probe::{ProbeRequest, ProbeRun, ProbeRunStatus};
use engine::result::ValidationReport;
use engine::schema::{PackTypeDecl, Schema, load_schema_with_types};
use engine::validate::validate_store;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-store-sweep-{tag}-{}-{:?}",
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

    /// Commit a doc at `<root>/<location>/<slug>.md` — the address slug is the filename
    /// stem the store walk reads (`enumerate_committed_surface`).
    fn commit(&self, location: &str, slug: &str, body: &str) {
        let dir = self.0.join(location);
        fs::create_dir_all(&dir).expect("mk location");
        fs::write(dir.join(format!("{slug}.md")), body).expect("commit doc");
    }

    /// Write a real `.rs` file at `<root>/<rel>` (the working tree the probe resolves
    /// symbols against; `working_tree_root = repo_root`).
    fn write_code(&self, rel: &str, body: &str) {
        let path = self.0.join(rel);
        fs::create_dir_all(path.parent().expect("rel has a parent")).expect("mk code dir");
        fs::write(&path, body).expect("write .rs");
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Build the `doc-code` probe executable (a program **outside** the workspace) and
/// return its path — the `build_doc_code_probe` idiom (manifest at
/// `pack/probes/doc-code`), built into its own target dir so it never collides with the
/// workspace build.
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

/// Translate the CLI invoker's raw [`ProbeOutcome`] into the engine's [`ProbeRun`] — the
/// inc-3 seam between the CLI invoker and the engine's ingestion (the same translation
/// the production CLI does). A spawn failure (an absent program) is mapped to a
/// signal-shaped non-zero exit, never a silent pass: a probe that could not be invoked
/// surfaces a `crash` meta-finding, exactly like a non-zero-exit run (`invoke.rs` — a
/// missing program is "an unresolvable invocation, never a silent pass").
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

/// The real invoker closure the store sweep drives — a genuine subprocess over the
/// snapshot the engine materialized, wrapping
/// [`invoke::invoke_probe`]`(`[`doc_code_program`]`(), .., `[`DOC_CODE_BUDGET`]`)`. It
/// serializes the engine's [`ProbeRequest`] to the child's stdin and translates the raw
/// outcome (or a spawn failure) into a [`ProbeRun`] for the engine to ingest.
fn real_doc_code_invoker(req: &ProbeRequest) -> std::io::Result<ProbeRun> {
    let request_bytes = serde_json::to_vec(req)?;
    match invoke::invoke_probe(&doc_code_program(), &request_bytes, DOC_CODE_BUDGET) {
        Ok(outcome) => Ok(into_run(outcome)),
        // A spawn failure (e.g. an absent program) is the crash path: no run, no
        // findings, so it must NOT propagate as `Err` (that would abort the sweep with
        // no report) — it is ingested as a non-zero exit → one `crash` meta-finding.
        Err(_spawn) => Ok(ProbeRun {
            stdout: Vec::new(),
            status: ProbeRunStatus::Exited { code: None },
        }),
    }
}

/// The seam type `validate_store` expects: a `&dyn Fn(&ProbeRequest) -> io::Result<…>`.
/// The free fn coerces to a fn-pointer, which the engine borrows as the trait object.
fn invoker() -> fn(&ProbeRequest) -> std::io::Result<ProbeRun> {
    real_doc_code_invoker
}

/// The three code-anchor doctypes the store sweep walks, loaded from the **shipped** pack
/// schemas with the dev pack's `code-anchor` field-type declaration:
/// - `adr` (`decisions/`) — a header `cites-code` (`symbol-exists`);
/// - `spec` (`specs/`) — a repeatable `criteria` block with `maps-to-test`
///   (`criterion-maps-to-test`);
/// - `arch-doc` (`architecture/`) — a repeatable `components` block with a bare
///   `implemented-by` code-anchor inheriting the type's `symbol-exists` check.
fn schemas() -> BTreeMap<String, Schema> {
    const ADR_YAML: &[u8] = include_bytes!("../pack/schemas/adr.yaml");
    const SPEC_YAML: &[u8] = include_bytes!("../pack/schemas/spec.yaml");
    const ARCH_DOC_YAML: &[u8] = include_bytes!("../pack/schemas/arch-doc.yaml");
    // The dev pack's `code-anchor` field type — the `field-types.yaml` declaration the
    // CLI feeds the engine (`doc.rs`'s `code-anchor → doc-code/symbol-exists` idiom).
    let types = vec![PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
    }];
    let mut m = BTreeMap::new();
    m.insert(
        "adr".to_owned(),
        load_schema_with_types(ADR_YAML, &types).expect("adr.yaml loads"),
    );
    m.insert(
        "spec".to_owned(),
        load_schema_with_types(SPEC_YAML, &types).expect("spec.yaml loads"),
    );
    m.insert(
        "arch-doc".to_owned(),
        load_schema_with_types(ARCH_DOC_YAML, &types).expect("arch-doc.yaml loads"),
    );
    m
}

/// A committed `adr` citing `<rel>#<symbol>` from its `cites-code` header anchor.
fn adr(rel: &str, symbol: &str) -> String {
    format!(
        "---\n\
         status: accepted\n\
         date: 2026-06-13\n\
         cites-code: {rel}#{symbol}\n\
         ---\n\
         \n\
         # The cache decision\n\
         \n\
         ## Context\n\
         Forces.\n\
         \n\
         ## Decision\n\
         Decided.\n\
         \n\
         ## Consequences\n\
         Effects.\n"
    )
}

/// A committed `spec` whose single criterion (item id `per-second-limit`) carries a
/// `maps-to-test` code-anchor field citing `<rel>#<symbol>` (resolved with the
/// `criterion-maps-to-test` predicate — the symbol must be a `#[test]` fn). The criterion
/// renders in the on-disk repeatable-item form: `### <title>  {#id}`, the `statement`
/// slot prose, then the `<!-- fields -->` group carrying `maps-to-test`.
fn spec(rel: &str, symbol: &str) -> String {
    format!(
        "# Rate limiting\n\
         \n\
         ## Goal\n\
         \n\
         Bound the request rate.\n\
         \n\
         ## Context\n\
         \n\
         Bursts overwhelm the backend.\n\
         \n\
         ## Criteria\n\
         \n\
         ### Per-second limit  {{#per-second-limit}}\n\
         \n\
         Requests over the limit are rejected.\n\
         \n\
         <!-- fields -->\n\
         - maps-to-test: {rel}#{symbol}\n"
    )
}

/// A committed `arch-doc` (rendered from the **shipped** `arch-doc.yaml`) whose single
/// component (item id `edge-index`) carries a bare `implemented-by` code-anchor citing
/// `<rel>#<symbol>` (resolved with the inherited `symbol-exists` predicate — the anchor
/// address is `arch-doc:<slug>#components/<id>/implemented-by`). The empty `meta` header
/// renders as `---\n---`, the `## Overview` slot prose, then the `## Components` group:
/// `### <title>  {#id}`, the `description` slot prose, and the `<!-- fields -->` block
/// carrying `implemented-by`. The bytes are the canonical render of the shipped schema
/// (`render(parse(body)) == body`), so the store walk parses them identically.
fn arch_doc(rel: &str, symbol: &str) -> String {
    format!(
        "---\n\
         ---\n\
         \n\
         # Index layer\n\
         \n\
         ## Overview\n\
         \n\
         The edge index and target surface.\n\
         \n\
         ## Components\n\
         \n\
         ### Edge index  {{#edge-index}}\n\
         \n\
         Walks forward refs.\n\
         \n\
         <!-- fields -->\n\
         - implemented-by: {rel}#{symbol}\n"
    )
}

/// A no-delta resolved cascade — the post-pass leaves every emitted severity untouched,
/// so these sweeps assert the byte-identical no-override path (the same shape the engine
/// T1 tests use).
fn no_delta_resolved() -> engine::cascade::Resolved {
    engine::cascade::resolve(
        &engine::cascade::PackDefaultLayer::new("dev-pack", "0.1.0", BTreeMap::new(), Vec::new()),
        None,
        None,
    )
    .expect("resolves")
}

/// Seed a committed store over a working tree of real `.rs` files. The `adr` cites
/// `adr_symbol` in `crates/engine/src/cache.rs`; the `spec`'s criterion maps to
/// `spec_symbol` in `crates/engine/src/limiter.rs`; the `arch-doc`'s component anchors
/// `arch_symbol` in `crates/engine/src/index.rs`. The code bodies define `evict_lru`
/// (a plain `fn`), `covers_burst` (a `#[test]` fn), and `walk_edges` (a plain `fn`), so
/// passing those names is the clean case and any other name is the dangling case.
fn seed_store(tag: &str, adr_symbol: &str, spec_symbol: &str, arch_symbol: &str) -> TempDir {
    let repo = TempDir::new(tag);
    repo.write_code(
        "crates/engine/src/cache.rs",
        "pub fn evict_lru() {}\nfn helper() {}\n",
    );
    repo.write_code(
        "crates/engine/src/limiter.rs",
        "#[test]\nfn covers_burst() {}\nfn plain() {}\n",
    );
    repo.write_code(
        "crates/engine/src/index.rs",
        "pub fn walk_edges() {}\nfn helper() {}\n",
    );
    repo.commit(
        "decisions",
        "cache",
        &adr("crates/engine/src/cache.rs", adr_symbol),
    );
    repo.commit(
        "specs",
        "rate-limiting",
        &spec("crates/engine/src/limiter.rs", spec_symbol),
    );
    repo.commit(
        "architecture",
        "index-layer",
        &arch_doc("crates/engine/src/index.rs", arch_symbol),
    );
    repo
}

/// Assert a failed-probe report carries exactly one blocking `pack-probe-integrity.*`
/// meta-finding and blocks.
fn assert_one_meta(label: &str, report: &ValidationReport) {
    let meta: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.code.starts_with("pack-probe-integrity"))
        .collect();
    assert_eq!(
        meta.len(),
        1,
        "a {label} invocation must yield exactly one pack-probe-integrity meta-finding: {:?}",
        report.findings,
    );
    assert_eq!(
        meta[0].severity,
        engine::finding::Severity::Blocking,
        "{label}: the meta-finding is intrinsic-blocking",
    );
    assert!(
        report.has_blocking(),
        "{label}: the report must block (the probe could not be trusted): {:?}",
        report.findings,
    );
}

/// Compile a tiny **real** probe program that drains its stdin request then exits 2 — a
/// crashing `doc-code` the invoker drives as an actual subprocess (the non-zero-exit
/// failure mode, the `build_stub` idiom from `probe_invoker.rs`). Named `doc-code` so a
/// `JIGC_DOC_CODE_PROBE` pointed at it resolves.
fn build_crasher(dir: &Path) -> PathBuf {
    let src = dir.join("crasher.rs");
    fs::write(
        &src,
        "fn main() {\n\
         use std::io::Read;\n\
         let mut buf = String::new();\n\
         std::io::stdin().read_to_string(&mut buf).ok();\n\
         std::process::exit(2);\n\
         }\n",
    )
    .expect("write crasher source");
    let bin = dir.join("doc-code");
    let out = std::process::Command::new("rustc")
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .arg("--edition")
        .arg("2021")
        .output()
        .expect("invoke rustc for the crasher stub");
    assert!(
        out.status.success(),
        "rustc failed to build the crasher stub:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    bin
}

/// The increment acceptance: the real `doc-code` probe over a committed store, driven
/// through the T1 entry point with the production invoker. The three cases share one
/// test because each mutates the process-global `JIGC_DOC_CODE_PROBE` override.
#[test]
fn real_doc_code_probe_over_committed_store() {
    let probe = build_doc_code_probe();
    // SAFETY: the documented dev/test override of the probe-path knob. The Rust harness
    // runs `#[test]` fns in parallel and this env var is process-global, so all three
    // cases live in this one test to keep the mutations sequential; no other test reads
    // `JIGC_DOC_CODE_PROBE`, so this is the sole in-process writer.
    unsafe { std::env::set_var("JIGC_DOC_CODE_PROBE", &probe) };

    // --- (a) every cited symbol exists → no doc-code content finding, no block.
    let clean = seed_store("clean", "evict_lru", "covers_burst", "walk_edges");
    let report = validate_store(clean.path(), &schemas(), &no_delta_resolved(), &invoker())
        .expect("store sweep runs");
    assert!(
        report
            .findings
            .iter()
            .all(|f| !f.code.starts_with("doc-code")),
        "every cited symbol exists, so no doc-code content finding: {:?}",
        report.findings,
    );
    assert!(
        !report.has_blocking(),
        "a clean store must not block: {:?}",
        report.findings,
    );

    // --- (b) renamed/deleted cited symbols → one blocking content finding each, keyed
    // on the anchor's address. `cache.rs` renamed `evict_lru` → `vanished`; `limiter.rs`
    // deleted the `#[test]` fn; the committed docs still cite the now-gone symbols.
    let dangling = TempDir::new("dangling");
    dangling.write_code(
        "crates/engine/src/cache.rs",
        "pub fn vanished() {}\nfn helper() {}\n",
    );
    dangling.write_code("crates/engine/src/limiter.rs", "fn plain() {}\n");
    // arch-doc's implemented-by symbol renamed away too: `walk_edges` → `gone`.
    dangling.write_code(
        "crates/engine/src/index.rs",
        "pub fn gone() {}\nfn helper() {}\n",
    );
    dangling.commit(
        "decisions",
        "cache",
        &adr("crates/engine/src/cache.rs", "evict_lru"),
    );
    dangling.commit(
        "specs",
        "rate-limiting",
        &spec("crates/engine/src/limiter.rs", "covers_burst"),
    );
    dangling.commit(
        "architecture",
        "index-layer",
        &arch_doc("crates/engine/src/index.rs", "walk_edges"),
    );
    let report = validate_store(
        dangling.path(),
        &schemas(),
        &no_delta_resolved(),
        &invoker(),
    )
    .expect("store sweep runs");

    // The renamed `adr.cites-code` and `arch-doc.components/<id>/implemented-by` symbols
    // both surface a `doc-code.symbol-exists` finding (the bare `symbol-exists` check the
    // type declares), each keyed on its own anchor's address.
    let symbol_exists: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.code == "doc-code.symbol-exists")
        .collect();
    assert_eq!(
        symbol_exists.len(),
        2,
        "the renamed cites-code and implemented-by symbols surface two symbol-exists \
         findings: {:?}",
        report.findings,
    );
    assert!(
        symbol_exists
            .iter()
            .all(|f| f.severity == engine::finding::Severity::Blocking),
        "every symbol-exists finding is blocking: {:?}",
        report.findings,
    );
    let symbol_exists_addrs: Vec<_> = symbol_exists
        .iter()
        .filter_map(|f| f.location.as_ref().and_then(|l| l.address.as_deref()))
        .collect();
    assert!(
        symbol_exists_addrs.contains(&"adr:cache#status/cites-code"),
        "one symbol-exists finding is keyed on the adr anchor's address: {symbol_exists_addrs:?}",
    );
    assert!(
        symbol_exists_addrs.contains(&"arch-doc:index-layer#components/edge-index/implemented-by"),
        "one symbol-exists finding is keyed on the arch-doc component anchor's address: \
         {symbol_exists_addrs:?}",
    );

    let maps_to_test: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.code == "doc-code.criterion-maps-to-test")
        .collect();
    assert_eq!(
        maps_to_test.len(),
        1,
        "the deleted test surfaces exactly one criterion-maps-to-test finding: {:?}",
        report.findings,
    );
    assert_eq!(
        maps_to_test[0].severity,
        engine::finding::Severity::Blocking
    );
    assert_eq!(
        maps_to_test[0]
            .location
            .as_ref()
            .and_then(|l| l.address.as_deref()),
        Some("spec:rate-limiting#criteria/per-second-limit/maps-to-test"),
        "the criterion-maps-to-test finding is keyed on the criterion item's address",
    );
    assert!(
        report.has_blocking(),
        "dangling anchors must make the report block: {:?}",
        report.findings,
    );

    // --- (c) injected probe failure → one blocking pack-probe-integrity meta-finding.
    let failing = seed_store("failed-probe", "evict_lru", "covers_burst", "walk_edges");

    // absent program: point the dev/test knob at a path with no executable.
    let missing = TempDir::new("no-probe");
    let absent = missing.path().join("does-not-exist-doc-code");
    // SAFETY: see the top-of-test note — sole in-process writer of the dev/test knob.
    unsafe { std::env::set_var("JIGC_DOC_CODE_PROBE", &absent) };
    let report = validate_store(failing.path(), &schemas(), &no_delta_resolved(), &invoker())
        .expect("store sweep runs even when the probe is absent");
    assert_one_meta("absent program", &report);

    // non-zero exit: a real program that drains stdin and exits 2 (a crashing probe).
    let probe_dir = TempDir::new("crasher");
    let crasher = build_crasher(probe_dir.path());
    // SAFETY: see the top-of-test note — sole in-process writer of the dev/test knob.
    unsafe { std::env::set_var("JIGC_DOC_CODE_PROBE", &crasher) };
    let report = validate_store(failing.path(), &schemas(), &no_delta_resolved(), &invoker())
        .expect("store sweep runs even when the probe crashes");
    assert_one_meta("non-zero exit", &report);
}
