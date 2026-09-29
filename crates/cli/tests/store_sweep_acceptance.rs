//! M18 inc-2 / T2 — the **acceptance**: the real `doc-code` probe driven over a
//! committed-store fixture through the T1 store-sweep entry point.
//!
//! T1 ([`engine::validate::validate_store`]) is the task-less, read-only store sweep —
//! enumerate every committed doc's `code-anchor` leaves, materialize the
//! `EffectiveStateSnapshot` to a temp scratch path, drive a CLI-supplied invoker, and
//! ingest the probe's findings + the `pack-probe-integrity.*` meta-findings into a
//! [`engine::result::ValidationReport`]. T1's own tests stubbed the invoker in-process.
//! **This test supplies the real one**: it runs the built `jigc` as its own `doc-code`
//! probe (self-exec, M54 S4) through
//! [`cli::invoke::invoke_probe`]`(.., `[`cli::invoke::DOC_CODE_BUDGET`]`)` — a genuine
//! subprocess over a snapshot the engine materialized, exactly as production
//! `jigc validate` will.
//!
//! It seeds a committed store — a `docs/decisions/`-located `adr` carrying a `cites-code`
//! anchor (`symbol-exists`), a `docs/specs/`-located `spec` carrying a criterion
//! `maps-to-test` anchor (`criterion-maps-to-test`), and an `docs/architecture/`-located
//! `arch-doc` carrying a `components/<id>/implemented-by` anchor (`symbol-exists`) — over
//! a working tree of real `.rs` files. The `arch-doc` exercises the **shipped**
//! `arch-doc.yaml` schema (its `docs/architecture/` location + schema-load interaction is the
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

use cli::invoke::{self, DOC_CODE_BUDGET, ProbeOutcome, ProbeStatus};
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
            engine::tempname::unique_nanos(),
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

/// How this in-process caller runs the probe — `doc_code_command`'s two arms with the
/// production one re-aimed: the `JIGC_DOC_CODE_PROBE` override as a standalone program
/// with no args, else the built `jigc` (`CARGO_BIN_EXE_jigc`) with
/// [`invoke::doc_code_probe_args`] (M54 S4). A test process's own `current_exe()` is the
/// test binary, so the self-exec arm names the real `jigc` directly.
fn doc_code_command() -> (PathBuf, Vec<String>) {
    match invoke::doc_code_override() {
        Some(program) => (program, Vec::new()),
        None => (
            PathBuf::from(env!("CARGO_BIN_EXE_jigc")),
            invoke::doc_code_probe_args(),
        ),
    }
}

/// Translate the CLI invoker's raw [`ProbeOutcome`] into the engine's [`ProbeRun`] — the
/// inc-3 seam between the CLI invoker and the engine's ingestion (the same translation
/// the production CLI does). A spawn failure (an absent program) is handled by
/// [`real_doc_code_invoker`]'s error arm, never a silent pass.
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

/// The real invoker closure the store sweep drives — a genuine subprocess over the
/// snapshot the engine materialized, wrapping
/// [`invoke::invoke_probe`]`(`[`doc_code_command`]`(), .., `[`DOC_CODE_BUDGET`]`)`. It
/// serializes the engine's [`ProbeRequest`] to the child's stdin and translates the raw
/// outcome (or a spawn failure) into a [`ProbeRun`] for the engine to ingest.
fn real_doc_code_invoker(req: &ProbeRequest) -> std::io::Result<ProbeRun> {
    let request_bytes = serde_json::to_vec(req)?;
    let (program, args) = doc_code_command();
    match invoke::invoke_probe(&program, &args, &request_bytes, DOC_CODE_BUDGET) {
        Ok(outcome) => Ok(into_run(outcome)),
        // A spawn failure (e.g. an absent program) is the crash path: no run, no
        // findings, so it must NOT propagate as `Err` (that would abort the sweep with
        // no report) — it is ingested as *could not start* → one `crash` meta-finding
        // (the production mapping, `task.rs` → `doc_code_invoker`).
        Err(spawn) => Ok(ProbeRun {
            stdout: Vec::new(),
            stderr: Vec::new(),
            status: ProbeRunStatus::CouldNotStart {
                error: spawn.to_string(),
            },
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
/// - `adr` (`docs/decisions/`) — a header `cites-code` (`symbol-exists`);
/// - `spec` (`docs/specs/`) — a repeatable `criteria` block with `maps-to-test`
///   (`criterion-maps-to-test`);
/// - `arch-doc` (`docs/architecture/`) — a repeatable `components` block with a bare
///   `implemented-by` code-anchor inheriting the type's `symbol-exists` check.
fn schemas() -> BTreeMap<String, Schema> {
    const ADR_YAML: &[u8] = include_bytes!(cli::pack_path!(dev, "schemas/adr.yaml"));
    const SPEC_YAML: &[u8] = include_bytes!(cli::pack_path!(dev, "schemas/spec.yaml"));
    const ARCH_DOC_YAML: &[u8] = include_bytes!(cli::pack_path!(dev, "schemas/arch-doc.yaml"));
    // The dev pack's `code-anchor` field type — the `field-types.yaml` declaration the
    // CLI feeds the engine (`doc.rs`'s `code-anchor → doc-code/symbol-exists` idiom).
    let types = vec![PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
        hint: None,
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
         ## Options\n\
         Alternatives were weighed and rejected.\n\
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
    // SAFETY: the documented dev/test override of the probe-path knob. The Rust harness
    // runs `#[test]` fns in parallel and this env var is process-global, so all three
    // cases live in this one test to keep the mutations sequential; no other test reads
    // `JIGC_DOC_CODE_PROBE`, so this is the sole in-process writer. Removed first, so an
    // ambient value cannot mask the self-exec arm (a) and (b) exercise.
    unsafe { std::env::remove_var("JIGC_DOC_CODE_PROBE") };

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
    let meta = report
        .findings
        .iter()
        .find(|f| f.code.starts_with("pack-probe-integrity"))
        .expect("the one meta-finding");
    assert!(
        meta.check == "crash" && meta.message.contains("could not start"),
        "an absent program is a `crash` that says it could not start (M54 Inc 2 T2): {meta:?}",
    );

    // non-zero exit: a real program that drains stdin and exits 2 (a crashing probe).
    let probe_dir = TempDir::new("crasher");
    let crasher = build_crasher(probe_dir.path());
    // SAFETY: see the top-of-test note — sole in-process writer of the dev/test knob.
    unsafe { std::env::set_var("JIGC_DOC_CODE_PROBE", &crasher) };
    let report = validate_store(failing.path(), &schemas(), &no_delta_resolved(), &invoker())
        .expect("store sweep runs even when the probe crashes");
    assert_one_meta("non-zero exit", &report);
}

/// M21 inc-3 / T1 — the **G4 baseline-adopt gate** acceptance over the **real binary**.
///
/// The store sweep ([`engine::file_state::reconcile_committed_store`], reached by
/// `jigc task validate`/`finalize`-preflight) no longer silently baseline-adopts an
/// unvetted foreign `.md` squatting in a `location:` dir. This drives the production
/// binary (`CARGO_BIN_EXE_jigc`) over a real `git init` repo (the
/// `file_state_soundness.rs` harness shape):
///
/// - a freeform `docs/decisions/notes.md` (no recorded hash → the `UNKNOWN` arm) is routed as
///   an **advisory** (the sweep does **not** block — exit 0) and **not** baseline-adopted,
///   so a second `task validate` **re-fires** the same advisory (the routed-but-not-recorded
///   recurrence, `design/project-setup.md` → Flow 2 hardening, consequence note `:115`);
/// - a conformant jigc-minted `docs/decisions/<slug>.md` baselines **without** a false
///   conformance-block advisory (the M20 clean-store guarantee holds).
///
/// **Which advisory** is the managed-vs-foreign discriminator's call since M48 Inc 4 / T1.
/// Freeform scratch prose parses against no shipped `adr` schema version and carries no
/// stamp, so it is a **never-adopted foreign** file: it converges on the store family's
/// `schema-conformance.unadopted-instance` and the adoption route naming its own path,
/// instead of the `reconciliation.conformance-block` this door used to grade every
/// non-conformant file with. The gate this suite pins is untouched — routed, not recorded,
/// exit 0, recurring — only the code it is routed under. The convergence itself is pinned
/// at `crates/cli/tests/foreign_at_both_doors.rs`.
mod g4_baseline_adopt_gate {
    use crate::support::run_then_parse::stdout_json;
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    /// A throwaway repo dir that removes itself on drop.
    struct Repo(std::path::PathBuf);

    impl Repo {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-g4-{tag}-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            ));
            fs::create_dir_all(&path).expect("create temp repo");
            Repo(path)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Repo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn git(repo: &Path, args: &[&str]) {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
    fn git_out(repo: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout)
            .expect("utf-8")
            .trim_end_matches('\n')
            .to_string()
    }

    /// A real `git init` repo with one commit + the `.jigc/config/` project layer.
    fn init_repo(repo: &Path) {
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write file");
        git(repo, &["add", "."]);
        git(repo, &["commit", "-q", "-m", "initial"]);
        fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    }

    /// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
    fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(repo)
            .env("HOME", home)
            .output()
            .expect("run the jigc binary")
    }

    /// Run `jigc doc <args>`, piping `stdin`, capturing output.
    fn jigc_doc_stdin(
        repo: &Path,
        home: &Path,
        args: &[&str],
        stdin: &[u8],
    ) -> std::process::Output {
        use std::io::Write;
        use std::process::Stdio;
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .arg("doc")
            .args(args)
            .current_dir(repo)
            .env("HOME", home)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().expect("spawn jigc");
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(stdin)
            .expect("write stdin");
        child.wait_with_output().expect("wait for jigc")
    }

    fn assert_ok(out: &std::process::Output, what: &str) {
        assert!(
            out.status.success(),
            "{what} must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }

    /// Start a commit-only `single-task` for `intent` (the task id is the slugified
    /// intent), stage one code file + fill the commit doc, leaving the caller to drive
    /// validate.
    fn stage_commit_only(repo: &Path, home: &Path, task: &str, intent: &str) {
        let out = jigc(repo, home, &["start", "--workflow", "single-task", intent]);
        assert_ok(&out, &format!("`jigc start` ({task})"));
        fs::write(repo.join(format!("{task}.txt")), "the code change\n")
            .expect("write code change");
        // M30 G5 — the agent stages its own edit so the per-task narrowing commits it (the
        // narrowing no longer sweeps the unstaged tree, so a finalize here would otherwise
        // block as nothing-staged).
        git(repo, &["add", &format!("{task}.txt")]);
        let set_field = |addr: &str, value: &str| {
            assert_ok(
                &jigc_doc_stdin(repo, home, &["set-field", addr, "--value", value], b""),
                &format!("set-field {addr}"),
            );
        };
        let set_slot = |addr: &str, prose: &[u8]| {
            assert_ok(
                &jigc_doc_stdin(repo, home, &["set-slot", addr, "--from-file", "-"], prose),
                &format!("set-slot {addr}"),
            );
        };
        set_field(&format!("commit:{task}#type"), "feat");
        set_field(&format!("commit:{task}#scope"), "cache");
        set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
        set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
    }

    /// The findings of a parsed `task validate --format json` envelope.
    fn validate_findings(repo: &Path, home: &Path, task: &str) -> Vec<serde_json::Value> {
        let out = jigc(repo, home, &["task", "validate", task, "--format", "json"]);
        assert!(
            out.status.success(),
            "`task validate` must not block on an advisory-only sweep; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        envelope_findings(&out, "task validate")
    }

    /// The findings of a parsed `task finalize --format json` envelope. The
    /// finalize-preflight runs the same store sweep, so an advisory-only sweep must
    /// **land** (exit 0) — the criterion's "exit not blocked" at the finalize boundary.
    fn finalize_findings(repo: &Path, home: &Path, task: &str) -> Vec<serde_json::Value> {
        let out = jigc(repo, home, &["task", "finalize", task, "--format", "json"]);
        assert!(
            out.status.success(),
            "`task finalize` must land on an advisory-only sweep; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        envelope_findings(&out, "task finalize")
    }

    /// Parse the `findings` array of a `--format json` report envelope — both callers
    /// assert the run exited 0 first.
    fn envelope_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
        let value: serde_json::Value = stdout_json(out, &[0], what);
        let stdout = String::from_utf8_lossy(&out.stdout);
        value["findings"]
            .as_array()
            .unwrap_or_else(|| {
                panic!("{what}: the envelope carries a `findings` array; got:\n{stdout}")
            })
            .clone()
    }

    /// The task-scope advisory a **never-adopted foreign** file draws at the `UNKNOWN` arm
    /// (M48 Inc 4 / T1) — the store family's adoption code, served here too.
    const UNADOPTED: &str = "schema-conformance.unadopted-instance";
    /// The advisory a **managed** non-conformant file keeps at that same arm.
    const CONFORMANCE_BLOCK: &str = "reconciliation.conformance-block";

    /// Count the advisory findings of `code` whose stable key targets `path` — read from the
    /// emitted `key` object, the contract's own discriminating handle.
    fn advisory_count(findings: &[serde_json::Value], code: &str, path: &str) -> usize {
        findings
            .iter()
            .filter(|f| {
                f["key"]["code"] == code
                    && f["key"]["target"] == path
                    && f["severity"] == "advisory"
            })
            .count()
    }

    /// A freeform `notes.md` squatting in `docs/decisions/` (the foreign-file hazard) is
    /// **routed advisory** and **not** baseline-adopted on a first `task validate`
    /// sweep (exit not blocked), and the advisory **re-fires** on a second sweep — while
    /// a conformant jigc-minted ADR in `docs/decisions/` produces **no** false advisory.
    #[test]
    fn freeform_notes_in_decisions_routes_advisory_and_recurs() {
        let repo = Repo::new("notes");
        let home = Repo::new("home");
        init_repo(repo.path());

        // A foreign, non-conformant `.md` dropped into the `adr` location dir, committed
        // in git outside the CLI (the human-in-git channel).
        const NOTES: &str = "docs/decisions/notes.md";
        fs::create_dir_all(repo.path().join("docs").join("decisions")).expect("mk docs/decisions/");
        fs::write(
            repo.path().join(NOTES),
            "# scratch notes\n\nrandom thoughts, not an ADR\n",
        )
        .expect("write freeform notes");
        git(repo.path(), &["add", NOTES]);
        git(repo.path(), &["commit", "-q", "-m", "wip: stray notes"]);

        // ── first sweep: routed advisory, NOT baseline-adopted, exit not blocked ──────
        // One task is active at a time: each is finalized before the next starts, so the
        // internal `doc set-*` calls resolve a single active task. The finalize is also
        // the finalize-preflight store sweep the criterion names.
        let task_a = "first-pass";
        stage_commit_only(repo.path(), home.path(), task_a, task_a);
        let findings = validate_findings(repo.path(), home.path(), task_a);
        assert_eq!(
            advisory_count(&findings, UNADOPTED, NOTES),
            1,
            "the freeform notes.md is routed exactly one adoption advisory; got:\n{findings:#?}",
        );
        // The advisory-only sweep lands at the finalize boundary (exit not blocked).
        let findings = finalize_findings(repo.path(), home.path(), task_a);
        assert_eq!(
            advisory_count(&findings, UNADOPTED, NOTES),
            1,
            "the advisory also surfaces in the finalize-preflight sweep; got:\n{findings:#?}",
        );
        // Not baseline-adopted: task_a's landed finalize persisted the record, but the
        // foreign notes.md was routed-not-recorded, so it never entered the record.
        let record_path = repo
            .path()
            .join(".jigc")
            .join("state")
            .join("file-state.json");
        let record = fs::read_to_string(&record_path)
            .expect("task_a's landed finalize persists the file-state record");
        let value: serde_json::Value =
            serde_json::from_str(&record).expect("file-state.json parses");
        let hashes = value["hashes"]
            .as_object()
            .expect("the record carries a `hashes` map");
        assert!(
            !hashes.contains_key(NOTES),
            "the foreign notes.md must not be baseline-adopted into the record; got:\n{record}",
        );

        // ── second sweep: the advisory re-fires (routed-but-not-recorded recurrence) ──
        let task_b = "second-pass";
        stage_commit_only(repo.path(), home.path(), task_b, task_b);
        let findings = finalize_findings(repo.path(), home.path(), task_b);
        assert_eq!(
            advisory_count(&findings, UNADOPTED, NOTES),
            1,
            "the advisory re-fires on a second sweep (not silently absorbed); got:\n{findings:#?}",
        );

        // ── a conformant minted ADR baselines without a false advisory ────────────────
        let task_c_slug = "decide-the-cache-topology";
        let out = jigc(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "single-task",
                "decide the cache topology",
            ],
        );
        assert_ok(&out, "`jigc start` (task C)");
        let create = jigc_doc_stdin(
            repo.path(),
            home.path(),
            &["create", "adr", "--title", "Cache topology"],
            b"",
        );
        assert_ok(&create, "`jigc doc create adr` (task C)");
        for (slot, prose) in [
            ("context", "Session lookups must stay fast.\n"),
            ("decision", "Replicate the cache across nodes.\n"),
            ("consequences", "Higher write latency for resilience.\n"),
        ] {
            assert_ok(
                &jigc_doc_stdin(
                    repo.path(),
                    home.path(),
                    &[
                        "set-slot",
                        &format!("adr:cache-topology#{slot}"),
                        "--from-file",
                        "-",
                    ],
                    prose.as_bytes(),
                ),
                &format!("set-slot #{slot}"),
            );
        }
        // Stage the commit doc + a code file so the task is finalizable.
        fs::write(repo.path().join(format!("{task_c_slug}.txt")), "code\n").expect("write code");
        let set_field = |addr: &str, value: &str| {
            assert_ok(
                &jigc_doc_stdin(
                    repo.path(),
                    home.path(),
                    &["set-field", addr, "--value", value],
                    b"",
                ),
                &format!("set-field {addr}"),
            );
        };
        let set_slot = |addr: &str, prose: &[u8]| {
            assert_ok(
                &jigc_doc_stdin(
                    repo.path(),
                    home.path(),
                    &["set-slot", addr, "--from-file", "-"],
                    prose,
                ),
                &format!("set-slot {addr}"),
            );
        };
        set_field(&format!("commit:{task_c_slug}#type"), "feat");
        set_field(&format!("commit:{task_c_slug}#scope"), "cache");
        set_slot(
            &format!("commit:{task_c_slug}#summary"),
            b"replicate the cache\n",
        );
        set_slot(&format!("commit:{task_c_slug}#body"), b"A cache change.\n");
        // Finalize promotes the ADR to docs/decisions/ as a conformant jigc-minted doc.
        let out = jigc(repo.path(), home.path(), &["task", "finalize", task_c_slug]);
        assert_ok(&out, "`jigc task finalize` (task C)");
        const MINTED: &str = "docs/decisions/cache-topology.md";
        assert!(
            repo.path().join(MINTED).exists(),
            "task C must promote {MINTED}",
        );

        // A sweep over the conformant minted ADR produces NO false conformance-block
        // advisory for it (the legitimate baseline-adopt path is preserved).
        let task_d = "after-mint";
        stage_commit_only(repo.path(), home.path(), task_d, task_d);
        let findings = validate_findings(repo.path(), home.path(), task_d);
        assert_eq!(
            advisory_count(&findings, CONFORMANCE_BLOCK, MINTED),
            0,
            "the conformant minted ADR draws no false conformance-block advisory; got:\n{findings:#?}",
        );
    }

    /// The **finalize post-commit door** the G4 gate must also guard (M21 inc-3 fix),
    /// re-cast for M30: a freeform `notes.md` that is **uncommitted at `jigc start`** is
    /// detected by the finalize-preflight store sweep (which reads the on-disk tree) and
    /// routed an advisory — and must **not** be baseline-adopted by the post-commit
    /// `advance_file_state` re-hash, else the foreign file becomes `IN_SYNC` and the
    /// advisory never recurs. Under the M30 per-task narrowing the unstaged notes.md is
    /// **no longer swept into the commit** (it stays untracked in the working tree), which
    /// makes the not-adopted guarantee even tighter — and the advisory still re-fires from
    /// the on-disk preflight sweep on a later task.
    ///
    /// Distinct from `freeform_notes_in_decisions_routes_advisory_and_recurs`, where the
    /// notes are committed to git **before** `jigc start`. Asserts: finalize 1 emits the
    /// advisory once, notes.md is NOT in the landed commit, the landed record does **not**
    /// contain the foreign path, and a second task's finalize **re-fires** the advisory.
    #[test]
    fn freeform_notes_uncommitted_route_advisory_not_baseline_adopted() {
        let repo = Repo::new("notes-mid-task");
        let home = Repo::new("home-mid-task");
        init_repo(repo.path());

        const NOTES: &str = "docs/decisions/notes.md";
        let record_path = repo
            .path()
            .join(".jigc")
            .join("state")
            .join("file-state.json");

        // ── task A: drop the foreign notes.md AFTER start (uncommitted), then finalize ──
        // The on-disk preflight sweep sees it; the M30 narrowing does NOT commit it.
        let task_a = "first-pass";
        stage_commit_only(repo.path(), home.path(), task_a, task_a);
        fs::create_dir_all(repo.path().join("docs").join("decisions")).expect("mk docs/decisions/");
        fs::write(
            repo.path().join(NOTES),
            "# scratch notes\n\nrandom thoughts, not an ADR\n",
        )
        .expect("write freeform notes");

        let findings = finalize_findings(repo.path(), home.path(), task_a);
        assert_eq!(
            advisory_count(&findings, UNADOPTED, NOTES),
            1,
            "finalize 1 routes the freeform notes.md exactly one advisory; got:\n{findings:#?}",
        );

        // M30 — the unstaged notes.md is NOT swept into the commit, and stays untracked.
        let committed = git_out(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
        assert!(
            !committed.lines().any(|l| l == NOTES),
            "the unstaged freeform notes.md must NOT ride the commit (M30 narrowing); files:\n{committed}",
        );
        let status = git_out(
            repo.path(),
            &["status", "--porcelain", "--untracked-files=all"],
        );
        assert!(
            status.lines().any(|l| l.contains(NOTES)),
            "the freeform notes.md stays uncommitted in the working tree; status:\n{status}",
        );

        // The post-commit re-hash must NOT record notes.md (it failed the conformance
        // gate) — it stays UNKNOWN so the advisory recurs.
        let record = fs::read_to_string(&record_path)
            .expect("task A's landed finalize persists the file-state record");
        let value: serde_json::Value =
            serde_json::from_str(&record).expect("file-state.json parses");
        let hashes = value["hashes"]
            .as_object()
            .expect("the record carries a `hashes` map");
        assert!(
            !hashes.contains_key(NOTES),
            "the foreign notes.md must not be baseline-adopted into the record; got:\n{record}",
        );

        // ── task B: a second finalize re-fires the advisory (still UNKNOWN) ─────────────
        let task_b = "second-pass";
        stage_commit_only(repo.path(), home.path(), task_b, task_b);
        let findings = finalize_findings(repo.path(), home.path(), task_b);
        assert_eq!(
            advisory_count(&findings, UNADOPTED, NOTES),
            1,
            "the advisory re-fires on the second finalize (not silently absorbed at the \
             post-commit door); got:\n{findings:#?}",
        );
    }
}
