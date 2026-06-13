//! M18 inc-3 / T1 — the `jigc validate` top-level command, **clean-path** end-to-end
//! through the built `jigc` binary against the **real** `doc-code` probe.
//!
//! The store-scope re-validation sweep ([`engine::validate::validate_store`], inc-2) is
//! task-less by construction: it enumerates every committed doc's `code-anchor` leaves
//! and resolves each against the working tree, with no working area open. T1 wires it to
//! a new top-level `jigc validate` (the `run_ingest`/`run_upgrade` locate-preamble +
//! `render::validation` precedent). This test drives that command as a real process over
//! a committed store whose anchors all resolve, asserting the **headline clean path**
//! (`design/validation.md` → Store-scope re-validation → The command):
//!
//! - a committed `adr` (`decisions/`) citing an existing symbol + a committed `spec`
//!   (`specs/`) whose criterion maps to an existing `#[test]` fn → `jigc validate`
//!   **exits 0** with the clean report rendered and **no `doc-code` content finding** in
//!   stdout.
//!
//! The exit-class halves (probe pre-flight → one operational error; the
//! `pack-probe-integrity.*` non-zero exit rule) land in T2/T3; this task is the clean
//! path only (always exit 0). The `jigc` path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and the `doc-code` probe is the real binary built from the
//! pack and selected via `JIGC_DOC_CODE_PROBE` (the flow13_acceptance idiom).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-validate-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path —
/// the **real** tree-sitter subprocess the engine/CLI seam drives, never a mock.
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("pack")
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
}

/// Run a `git` command in `repo`, asserting success.
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

/// Run `jigc <args>` with `cwd = repo` and the real doc-code probe selected via
/// `JIGC_DOC_CODE_PROBE`, capturing output.
fn jigc(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// A committed `adr` citing `<rel>#<symbol>` from its `cites-code` header anchor
/// (the dev pack's `code-anchor` field type → `doc-code/symbol-exists`).
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
/// `maps-to-test` code-anchor citing `<rel>#<symbol>` (resolved with the
/// `criterion-maps-to-test` predicate — the symbol must be a `#[test]` fn).
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

/// Seed a real git repo with the `.jigc/config/` project layer + a committed store whose
/// anchors all resolve: an `adr` citing `evict_lru` in `crates/engine/src/cache.rs` and a
/// `spec` mapping to the `#[test]` fn `covers_burst` in `crates/engine/src/limiter.rs`.
fn seed_clean_store(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    let cache = repo.join("crates/engine/src/cache.rs");
    fs::create_dir_all(cache.parent().unwrap()).expect("mk code dir");
    fs::write(&cache, "pub fn evict_lru() {}\nfn helper() {}\n").expect("write cache.rs");
    fs::write(
        repo.join("crates/engine/src/limiter.rs"),
        "#[test]\nfn covers_burst() {}\nfn plain() {}\n",
    )
    .expect("write limiter.rs");

    fs::create_dir_all(repo.join("decisions")).expect("mk decisions");
    fs::write(
        repo.join("decisions/cache.md"),
        adr("crates/engine/src/cache.rs", "evict_lru"),
    )
    .expect("write adr");
    fs::create_dir_all(repo.join("specs")).expect("mk specs");
    fs::write(
        repo.join("specs/rate-limiting.md"),
        spec("crates/engine/src/limiter.rs", "covers_burst"),
    )
    .expect("write spec");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    // The project layer — the locate-preamble's `require_project_layer` gate.
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// The clean path: a committed store whose every cited symbol exists → `jigc validate`
/// exits 0 and renders the clean report with no `doc-code` content finding in stdout.
#[test]
fn validate_clean_store_exits_zero_with_no_content_finding() {
    let repo = TempDir::new("clean");
    seed_clean_store(repo.path());

    let out = jigc(repo.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "`jigc validate` over a clean store must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("doc-code"),
        "a clean store must surface no doc-code content finding; stdout:\n{stdout}",
    );
}
