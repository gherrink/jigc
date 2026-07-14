//! M42 Increment 9 — a **repeated field key** reports ONCE, through the real binary
//! (`design/command-output-contract.md` → the stable finding key; the parse-conformance
//! sub-table's `unknown-field` row: key granularity **per `(section, key)`**).
//!
//! The last un-enumerated sibling of the collision class, and the one the family was *named*
//! for. `read_field_block_str` pushed one `conformance.unknown-field` per **field line**,
//! addressed at the field's key — so two lines carrying the **same** key addressed at the
//! **same** `#<section>/<field-key>` by construction, and the shipped binary handed a driver
//! two byte-identical `(code, target)` keys **inside one JSON array**. The debug binary was
//! worse: the increment's own membership seam turned an ordinary out-of-band hand edit into a
//! **panic** (exit 101), which the reconciliation invariant forbids outright — an OOB edit is
//! *detected and routed*, never crashed on.
//!
//! No test fed a duplicated field line, at any scope, so the whole gate was green over it.
//! Both funnels are driven here, because the defect fired in both: the task-scope
//! `jigc task validate --format json` envelope, and the store-scope `jigc validate --format
//! json` report (the CI-gating contract).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-dup-field-keys-{tag}-{}-{:?}",
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

/// A real git repo with one commit, then `jigc setup`.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    ok(repo, home, &["setup"], "jigc setup");
}

/// Parse a findings envelope, failing loudly with the raw output (a **panic** exits 101 and
/// prints nothing parseable — the defect's debug-binary face, so say so).
fn findings_of(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(
        out.status.code(),
        Some(101),
        "`{what}` PANICKED on an ordinary out-of-band hand edit — a repeated field line is \
         *detected and routed*, never crashed on (CLAUDE.md → reconciliation)\nstderr:\n{stderr}",
    );
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "`{what}` must print a findings envelope ({e});\nstdout:\n{stdout}\nstderr:\n{stderr}"
        )
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// Assert the repeated key `owner` produced exactly ONE `conformance.unknown-field` finding,
/// keyed at `target` — i.e. no two findings in the array share one `(code, target)`.
fn assert_reported_once(findings: &[serde_json::Value], target: &str) {
    let keys: Vec<String> = findings
        .iter()
        .filter(|f| f["code"] == "conformance.unknown-field")
        .filter_map(|f| f["key"]["target"].as_str().map(str::to_string))
        .filter(|t| t == target)
        .collect();
    assert_eq!(
        keys.len(),
        1,
        "the repeated field key must report ONCE at `{target}` — one repeated key is one \
         defect and one repair (delete the stray line). {} byte-identical `(code, target)` \
         keys in one JSON array is a key a driver cannot tell apart, which is the one thing \
         the finding key exists to prevent.\nfindings:\n{findings:#?}",
        keys.len(),
    );
}

/// (funnel 1 — the task gate) An out-of-band hand edit repeating a field line inside a task's
/// working copy: `jigc task validate --format json` reports the unknown key once, and does not
/// panic.
#[test]
fn a_repeated_field_key_reports_once_at_task_scope() {
    let repo = TempDir::new("task");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    ok(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "architecture-documentation",
            "doc the arch",
        ],
        "jigc start",
    );
    ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "arch-doc",
            "--title",
            "Overview",
            "--task",
            "doc-the-arch",
        ],
        "jigc doc create arch-doc",
    );
    ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            "arch-doc:overview#components",
            "--title",
            "Api",
            "--task",
            "doc-the-arch",
        ],
        "jigc doc add-item",
    );
    // A field write materializes the item's `<!-- fields -->` group (an item carries no
    // sentinel until it holds a field).
    ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "arch-doc:overview#components/api/implemented-by",
            "--value",
            "README.md",
            "--task",
            "doc-the-arch",
        ],
        "jigc doc set-field",
    );

    // The out-of-band hand edit: a repeated field line in the item's field group — the shape
    // a human produces by copying a bullet and forgetting to change its key.
    let doc = repo
        .path()
        .join(".jigc/tasks/doc-the-arch/docs/arch-doc:overview.md");
    let src = fs::read_to_string(&doc).expect("read the task's working copy");
    let patched = src.replace(
        "<!-- fields -->\n",
        "<!-- fields -->\n- owner: alice\n- owner: bob\n",
    );
    assert_ne!(src, patched, "the item's field group carries a sentinel");
    fs::write(&doc, patched).expect("write the hand edit");

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "doc-the-arch", "--format", "json"],
    );
    let findings = findings_of(&out, "jigc task validate --format json");
    assert_reported_once(&findings, "arch-doc:overview#components/api/owner");
}

/// (funnel 2 — the store sweep, the CI-gating contract) The same repeated field line in a
/// **committed** doc: `jigc validate --format json` reports it once, and does not panic.
#[test]
fn a_repeated_field_key_reports_once_at_store_scope() {
    let repo = TempDir::new("store");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // A committed arch-doc whose component item's field group repeats a key.
    let dir = repo.path().join("docs").join("architecture");
    fs::create_dir_all(&dir).expect("mk docs/architecture/");
    fs::write(
        dir.join("overview.md"),
        "\
---
schema-version: 1
---

# Overview

## Overview

The system, in one paragraph.

## Components

### Api  {#api}

<!-- fields -->
- implemented-by: README.md
- owner: alice
- owner: bob

The API surface.
",
    )
    .expect("write arch-doc");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed arch-doc"]);

    let out = jigc(repo.path(), home.path(), &["--format", "json", "validate"]);
    let findings = findings_of(&out, "jigc validate --format json");
    assert_reported_once(&findings, "arch-doc:overview#components/api/owner");
}
