//! Acceptance — **version-mismatch is itself a surfaced break** for the store-scope
//! schema-conformance detector (M34 Increment 3, the Inc-3 T4 build-halt resolution).
//!
//! The T3 routing path *labels* `schema-conformance.*` findings the detector already
//! produced — but a committed persisted doc whose only problem is an absent / below-current
//! schema-version stamp is **otherwise fully conformant**, so a labeler-only path produces no
//! finding to annotate and `jigc validate` stays silent (exit 0). That silent case is the
//! M34 dogfood's own headline: a **pure-stamp v0 corpus** carries no *other* non-conformance
//! (`design/validation.md` → Version-mismatch is itself a surfaced break; DECISIONS
//! 2026-06-25 → M34 Inc-3 T4 build halt resolved).
//!
//! This drives the **built `jigc` binary** through the real store-scope `jigc validate` over a
//! committed corpus with **no schema shadow** (the bytes an operator actually sees), and
//! asserts:
//!
//! - an **unstamped (v0)**, otherwise-conformant ADR is **reported** with a
//!   `schema-conformance.*` finding routed `migrate`, and `jigc validate` still **exits 0**
//!   (report-only at store scope — the CLI keys non-zero exit on `pack-probe-integrity` only).
//! - a **current-stamped** store surfaces **no** version finding and exits 0 (the
//!   false-positive guard).

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
            "jigc-version-mismatch-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path — the
/// real tree-sitter subprocess the `jigc validate` pre-flight resolves.
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

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
        .trim()
        .to_string()
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Make `root` a real git repo with identity, then run `jigc setup` over it.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// A conformant `adr` body under the **pack** adr schema, optionally carrying a
/// `schema-version` stamp line in its header (`None` = the unstamped v0 state). The prose is
/// conformant under the real (un-shadowed) schema — the *only* signal under test is the stamp.
fn adr_body(title: &str, stamp: Option<u32>) -> String {
    let stamp_line = match stamp {
        Some(v) => format!("schema-version: {v}\n"),
        None => String::new(),
    };
    format!(
        "\
---
status: accepted
date: 2026-06-25
{stamp_line}---

# {title}

## Context

Session lookups must stay sub-millisecond.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
"
    )
}

/// Commit an `adr` at its canonical `docs/decisions/<slug>.md`.
fn commit_adr(repo: &Path, slug: &str, title: &str, stamp: Option<u32>) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_body(title, stamp)).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// (the headline case) An ingested + baselined v0 corpus ADR (**no** schema-version stamp,
/// **no** schema shadow, otherwise conformant) is *reported* by `jigc validate` with a
/// `schema-conformance.*` break routed `migrate` — emission, not annotation — and the sweep
/// still exits 0. This is the case a labeler-only path leaves silent (its own dogfood).
#[test]
fn store_sweep_reports_unstamped_v0_doc_as_migrate_and_exits_zero() {
    let repo = TempDir::new("v0");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // The v0 corpus state: a committed ADR with NO schema-version stamp, fully conformant
    // under the un-shadowed pack schema.
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // Report-only: a content-only sweep exits 0 even with blocking-severity findings.
    assert!(
        out.status.success(),
        "a store-scope version-mismatch break must not gate `jigc validate` (exit 0); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // The unstamped doc surfaces exactly one schema-conformance break (the version break),
    // emitted even though the doc is otherwise structurally clean.
    assert_eq!(
        count(&stdout, "schema-conformance."),
        1,
        "an unstamped v0 ADR must surface exactly one schema-conformance break; \
         stdout:\n{stdout}",
    );
    // And it routes `migrate` — the emitted bytes an operator reads.
    assert_eq!(
        count(&stdout, "route: migrate"),
        1,
        "the unstamped (v0) ADR's version break must route `migrate`; stdout:\n{stdout}",
    );
}

/// (the false-positive guard) A current-stamped store surfaces **no** version finding, no
/// route line, and exits 0 — a doc stamped at the manifest version is clean.
#[test]
fn store_sweep_clean_on_current_stamped_doc() {
    let repo = TempDir::new("current");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(1));

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a current-stamped store must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("schema-conformance"),
        "a current-stamped, conformant committed ADR must surface NO schema-conformance \
         finding; stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "route: migrate"),
        0,
        "a current-stamped store must carry no migrate route line; stdout:\n{stdout}",
    );
}
