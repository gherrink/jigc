//! M42 Increment 9 (fix) — **the fourth funnel**: `jigc ingest`'s triage rows carry a
//! `Finding`, so they project a `key` — and until this fix they projected a *different* key
//! than `jigc validate` did for the **same defect in the same doc**
//! (`design/command-output-contract.md` → the stable finding key; the parse-conformance
//! sub-table: `unknown-field` → `type:slug#<section>/<field-key>`).
//!
//! `TriageRow` (`crates/cli/src/ingest.rs`) derives `Serialize` and holds an
//! `Option<Finding>`, so an ingest row's finding **is** serialized as a `Finding` — a member
//! of the envelope-projecting set by the increment's own predicate — yet the row's finding
//! re-derivation discarded the parse finding's normalized address and re-stamped the
//! candidate's **file path**. Same doc, same defect, two keys:
//!
//! ```text
//! jigc validate --format json → {"code":"conformance.unknown-field",
//!                                "target":"arch-doc:nasty-arch#components/alpha/bogus-key"}
//! jigc ingest   --format json → {"code":"conformance.unknown-field",
//!                                "target":"docs/architecture/nasty-arch.md"}
//! ```
//!
//! A driver dedupes on `(code, target)`; two keys for one defect is precisely the thing the
//! key exists to prevent. Both verbs are driven through the **real binary** here.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-ingest-keys-{tag}-{}-{:?}",
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

/// Run `jigc <args>` against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>`, asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(repo, home, args);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8")
}

/// Parse `stdout` as JSON, failing loudly with the raw output (a debug-build panic exits 101
/// and prints nothing parseable — say so rather than blaming the JSON).
fn json_of(out: &std::process::Output, what: &str) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(
        out.status.code(),
        Some(101),
        "`{what}` PANICKED — the seam must route a malformed candidate, never crash on \
         it\nstderr:\n{stderr}",
    );
    serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`{what}` must print JSON ({e});\nstdout:\n{stdout}\nstderr:\n{stderr}")
    })
}

/// A real git repo with one commit, then `jigc setup`, holding a **managed** (stamped)
/// arch-doc whose component item carries an undeclared field key — one defect, seen by two
/// verbs.
fn repo_with_a_bogus_key(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    ok(repo, home, &["setup"], "jigc setup");

    let dir = repo.join("docs").join("architecture");
    fs::create_dir_all(&dir).expect("mk docs/architecture/");
    fs::write(
        dir.join("nasty-arch.md"),
        "\
---
schema-version: 1
---

# Nasty arch

## Overview

The system, in one paragraph.

## Components

### Alpha  {#alpha}

<!-- fields -->
- implemented-by: README.md
- bogus-key: nonsense

The alpha component.
",
    )
    .expect("write the arch-doc");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed arch-doc"]);
}

/// The `(code, target)` key of the first `conformance.unknown-field` finding in `findings`.
fn unknown_field_key(findings: &[serde_json::Value], what: &str) -> serde_json::Value {
    findings
        .iter()
        .find(|f| f["code"] == "conformance.unknown-field")
        .unwrap_or_else(|| {
            panic!("`{what}` must report the undeclared `bogus-key`; got:\n{findings:#?}")
        })["key"]
        .clone()
}

/// **The fix, proven through the real binary**: `jigc validate` and `jigc ingest` emit the
/// **same** `(code, target)` key for the same undeclared field in the same doc — the URI form
/// the normalization table pins (`type:slug#<section>/<field-key>`), not the candidate's file
/// path. Red before the fix: ingest keyed at `docs/architecture/nasty-arch.md`.
#[test]
fn ingest_and_validate_key_one_defect_identically() {
    let repo = TempDir::new("agree");
    let home = TempDir::new("home");
    repo_with_a_bogus_key(repo.path(), home.path());

    let validate_out = jigc(repo.path(), home.path(), &["--format", "json", "validate"]);
    let validate = json_of(&validate_out, "jigc validate --format json");
    let validate_findings = validate["findings"]
        .as_array()
        .expect("the validate envelope carries a `findings` array")
        .clone();
    let validate_key = unknown_field_key(&validate_findings, "jigc validate --format json");

    let ingest_out = jigc(repo.path(), home.path(), &["--format", "json", "ingest"]);
    let ingest = json_of(&ingest_out, "jigc ingest --format json");
    let ingest_findings: Vec<serde_json::Value> = ingest["rows"]
        .as_array()
        .expect("the ingest report carries a `rows` array")
        .iter()
        .filter_map(|row| row.get("finding"))
        .filter(|f| !f.is_null())
        .cloned()
        .collect();
    let ingest_key = unknown_field_key(&ingest_findings, "jigc ingest --format json");

    assert_eq!(
        ingest_key,
        serde_json::json!({
            "code": "conformance.unknown-field",
            "target": "arch-doc:nasty-arch#components/alpha/bogus-key",
        }),
        "an ingest row's finding must key at the candidate's managed identity — the URI form \
         the normalization table pins for `unknown-field` — not at its file path",
    );
    assert_eq!(
        validate_key, ingest_key,
        "the same defect in the same doc must project ONE key, whichever verb reports it — a \
         driver dedupes on `(code, target)`",
    );
}
