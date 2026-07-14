//! M42 Increment 9 — `schema-conformance.required-slot-present` carries its declared **key
//! target**, through the real binary (`design/command-output-contract.md` → the stable
//! finding key; the `schema-conformance` row's `#<section>` fragment).
//!
//! It was the family's un-swept sibling: `check_field` / `check_field_value` address at
//! `#<section>/<field>` and the item-level slot check at `#<item-path>/<leaf>`, but the
//! **doc-level section slot** set no address at all — so the path→URI flip keyed all of a
//! doc's empty slots at the bare `type:slug`. A pristine `jigc doc create adr` has **three**
//! empty required slots, so the very first finalize of the very first ADR in a fresh repo
//! emitted **three byte-identical keys in one JSON array** — the collision-inside-a-single-
//! document shape the contract pins as impossible, on the highest-traffic finding in the loop.
//!
//! Both funnels are driven, because the defect fires in both: the task-scope blocked
//! `jigc task finalize --format json` envelope, and the store-scope
//! `jigc validate --format json` report (the CI-gating contract).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-slot-present-keys-{tag}-{}-{:?}",
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

/// The `key.target` of every `schema-conformance.required-slot-present` finding in `findings`
/// whose target names `doc` (its `type:slug` identity), in emission order.
fn slot_targets(findings: &[serde_json::Value], doc: &str) -> Vec<String> {
    findings
        .iter()
        .filter(|f| f["code"] == "schema-conformance.required-slot-present")
        .filter_map(|f| f["key"]["target"].as_str().map(str::to_string))
        .filter(|t| t.split('#').next() == Some(doc))
        .collect()
}

/// Assert the three targets are the three pinned per-section addresses — distinct keys, not
/// three copies of the bare doc URI.
fn assert_three_distinct(targets: &[String], doc: &str, findings: &[serde_json::Value]) {
    let mut sorted = targets.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        3,
        "the three empty required slots of one `{doc}` must project THREE DISTINCT keys in \
         ONE envelope — a driver deserializing the array cannot tell byte-identical keys \
         apart; got: {targets:?}\nfindings:\n{findings:#?}",
    );
    assert_eq!(
        sorted,
        vec![
            format!("{doc}#consequences"),
            format!("{doc}#context"),
            format!("{doc}#decision"),
        ],
        "each empty slot keys at its own section — `<type>:<slug>#<section>`",
    );
}

/// (funnel 1 — the task gate) A pristine `jigc doc create adr` has three empty required
/// slots; the blocked `jigc task finalize --format json` envelope must key each at its own
/// section.
#[test]
fn a_pristine_adr_keys_its_three_empty_slots_distinctly_at_finalize() {
    let repo = TempDir::new("finalize");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "pick a db"],
        "jigc start",
    );
    ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "pick a db",
            "--task",
            "pick-a-db",
        ],
        "jigc doc create adr",
    );

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", "pick-a-db", "--format", "json"],
    );
    assert!(
        !out.status.success(),
        "an ADR with three empty required slots must block finalize"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("the blocked finalize prints the findings envelope ({e}); got:\n{stdout}")
    });
    let findings = value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone();

    let targets = slot_targets(&findings, "adr:pick-a-db");
    assert_three_distinct(&targets, "adr:pick-a-db", &findings);
}

/// (funnel 2 — the store sweep, the CI-gating contract) A committed ADR whose three required
/// slots were hand-emptied must key each at its own section in the
/// `jigc validate --format json` report.
#[test]
fn a_committed_adr_keys_its_three_empty_slots_distinctly_at_store_scope() {
    let repo = TempDir::new("validate");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // A committed ADR at its canonical home whose three required slots hold no prose (the
    // headings are present — the unfilled-slot case, not a parse break).
    let dir = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(
        dir.join("use-rust.md"),
        "\
---
status: accepted
date: 2026-07-14
schema-version: 2
---

# Use Rust

## Context

## Options

Weighed Go, rejected on the borrow-checker guarantees.

## Decision

## Consequences
",
    )
    .expect("write adr");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed adr"]);

    let stdout = ok(
        repo.path(),
        home.path(),
        &["--format", "json", "validate"],
        "jigc validate --format json",
    );
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the store sweep prints a JSON report ({e}); got:\n{stdout}"));
    let findings = value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the report carries a `findings` array; got:\n{stdout}"))
        .clone();

    let targets = slot_targets(&findings, "adr:use-rust");
    assert_three_distinct(&targets, "adr:use-rust", &findings);
}
