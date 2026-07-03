//! M36 inc-2 / T2 — docs-root orphan detection, end-to-end through the built `jigc`
//! binary (no `JIGC_PACK_DIR`; the pack is the embedded dev pack).
//!
//! Two faces of the same coverage hole (`design/validation.md` → Orphan detection;
//! `design/storage.md` → docs-root):
//!
//! - **`jigc validate`** raises a store-scope `file-state.orphaned-doc` advisory (exit 0)
//!   for a committed managed doc stranded outside the resolved doctype roots after a
//!   `docs-root` re-point — while a `README.md`, an ordinary prose file under the
//!   docs-root parent, and a *live* doc at the *current* resolved root never flag (the
//!   bare-`.md`-match false-positive the location-basename predicate rules out).
//! - **`jigc config set docs-root`** warns *before the knob lands* when the re-point
//!   would orphan committed docs under the prior resolved root, routing the operator.
//!
//! The `jigc` path comes from `CARGO_BIN_EXE_jigc`; the temp repo is a real `git init`;
//! the `doc-code` probe (the `validate` pre-flight requires it) is the real binary built
//! from the pack and selected via `JIGC_DOC_CODE_PROBE` (the `validate_command.rs` idiom).

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
            "jigc-orphan-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path.
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
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

/// A conformant, hand-written `adr` body (no `cites-code`, so it needs no code file).
fn adr(title: &str) -> String {
    format!(
        "---\n\
         status: accepted\n\
         date: 2026-06-13\n\
         ---\n\
         \n\
         # {title}\n\
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

/// Seed a real git repo with the `.jigc/config/` project layer and:
///  - a committed `adr` at the DEFAULT resolved root `docs/decisions/cache.md`;
///  - a committed `adr` at `archive/decisions/live.md` (the *future* resolved root);
///  - a `README.md` at the repo root + an ordinary prose file `docs/guide.md`.
///
/// After a re-point to `docs-root = archive`, `docs/decisions/cache.md` is stranded
/// (orphan), while `archive/decisions/live.md` sits at the current root and the two prose
/// files never look like managed docs.
fn seed(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    fs::write(repo.join("README.md"), "# The project\n").expect("write readme");
    fs::create_dir_all(repo.join("docs")).expect("mk docs");
    fs::write(repo.join("docs/guide.md"), "# A guide\n\nProse.\n").expect("write guide");

    fs::create_dir_all(repo.join("docs/decisions")).expect("mk decisions");
    fs::write(
        repo.join("docs/decisions/cache.md"),
        adr("The cache decision"),
    )
    .expect("write orphan adr");

    fs::create_dir_all(repo.join("archive/decisions")).expect("mk archive decisions");
    fs::write(
        repo.join("archive/decisions/live.md"),
        adr("The live decision"),
    )
    .expect("write live adr");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    // The project layer — the locate-preamble's `require_project_layer` gate.
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// The orphaned-doc message lines from a rendered store report (the `file-state.orphaned-doc`
/// finding lines, excluding the trailing `  route:` continuation).
fn orphan_lines(stdout: &str) -> Vec<&str> {
    stdout
        .lines()
        .filter(|l| l.contains("file-state.orphaned-doc"))
        .collect()
}

/// `jigc validate` after a `docs-root` re-point: the committed doc left at the old root
/// surfaces `file-state.orphaned-doc` (exit 0), while README.md, ordinary prose, and a
/// live doc at the current root never flag.
#[test]
fn validate_flags_orphaned_doc_after_docs_root_repoint_only() {
    let repo = TempDir::new("validate");
    seed(repo.path());

    // Re-point docs-root docs/ → archive/. `docs/decisions/cache.md` is now stranded.
    let out = jigc(repo.path(), &["config", "set", "docs-root", "archive"]);
    assert!(
        out.status.success(),
        "`jigc config set docs-root archive` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let out = jigc(repo.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "`jigc validate` with only an orphan advisory must exit 0 (report-only); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    let lines = orphan_lines(&stdout);
    assert!(
        lines.iter().any(|l| l.contains("docs/decisions/cache.md")),
        "the doc stranded at the old root must surface a file-state.orphaned-doc finding; \
         stdout:\n{stdout}",
    );
    assert!(
        !lines.iter().any(|l| l.contains("README.md")),
        "README.md (root, no location-named parent) must never flag as an orphan; \
         orphan lines:\n{lines:?}",
    );
    assert!(
        !lines.iter().any(|l| l.contains("docs/guide.md")),
        "ordinary prose under the docs-root parent must never flag as an orphan; \
         orphan lines:\n{lines:?}",
    );
    assert!(
        !lines
            .iter()
            .any(|l| l.contains("archive/decisions/live.md")),
        "a live doc under the CURRENT resolved root must never flag as an orphan; \
         orphan lines:\n{lines:?}",
    );
}

/// `jigc config set docs-root <new>` over a repo with committed docs at the prior root
/// warns (naming the doc that would orphan) and routes — before the knob lands (exit 0).
#[test]
fn config_set_docs_root_warns_when_the_repoint_would_orphan() {
    let repo = TempDir::new("configset");
    seed(repo.path());

    let out = jigc(repo.path(), &["config", "set", "docs-root", "archive"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "the warning must not fail the write — `config set` still lands (exit 0); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("docs/decisions/cache.md"),
        "the warning must name the committed doc the re-point would orphan; stderr:\n{stderr}",
    );
    // A live doc at the FUTURE root is not orphaned by the move — it must not be named.
    assert!(
        !stderr.contains("archive/decisions/live.md"),
        "a doc already under the new root is not orphaned by the re-point; stderr:\n{stderr}",
    );
    assert!(
        stderr.to_lowercase().contains("orphan"),
        "the warning must announce the orphaning consequence; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc unmanage") || stderr.contains("re-point"),
        "the warning must route the operator (move / re-point / unmanage); stderr:\n{stderr}",
    );
}
