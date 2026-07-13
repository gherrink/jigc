//! Acceptance — the **managed-vs-foreign discriminator** at store scope (M42 Increment 3,
//! T1; `design/validation.md` → The managed-vs-foreign discriminator).
//!
//! Since Increment 2 the fifth content family enumerates the **placement** class, so
//! `jigc validate` reaches a repo's root `CHANGELOG.md` — and in a **stock brownfield
//! repo** that file is very often a real Keep-a-Changelog document the user **never handed
//! to jigc**. Adjudicated naively it produces four *blocking* `conformance.*` findings, each
//! routed at `jigc migrate-corpus` — a verb that does nothing for it (`0 migrated, 0 already
//! current, 1 blocked`): the tool would command a verb that cannot help, over a file it does
//! not own. That is the **in-location-squatter adoption case** (`design/storage.md` →
//! Placement), not an unmigrated corpus.
//!
//! The discriminator keys on **the doc's own committed bytes** — never on the file-state
//! record, which lives in gitignored, rebuildable `.jigc/state/` and is **empty on a fresh
//! clone of a managed repo** (under a record-keyed discriminator a teammate would be told to
//! `ingest` their own committed corpus). Three arms, all driven through the **real binary**:
//!
//! - **(foreign)** a stock brownfield repo (a real Keep-a-Changelog `CHANGELOG.md` + `jigc
//!   setup`, nothing else) → **zero blocking findings**, exactly one advisory
//!   `schema-conformance.unadopted-instance` addressed at the **file path**, naming both
//!   `jigc ingest` and `jigc migrate CHANGELOG.md --as changelog`; **exit 0**.
//! - **(managed, v0-era)** an unstamped ADR in the **prior (v1) shape** — the M34 headline
//!   detect case — stays **managed**: it is routed at the corpus migration and is **never**
//!   called unadopted (the parse-against-a-shipped-prior arm is what separates it from a
//!   foreign file).
//! - **(fresh clone)** a clone of a managed repo — no `.jigc/state`, so **no** file-state
//!   record at all — still reads its committed docs as **managed**: no adoption advisory,
//!   nothing blocking.

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
            "jigc-managed-vs-foreign-{tag}-{}-{:?}",
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
/// real subprocess the `jigc validate` pre-flight resolves.
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

/// Run a `git` command in `cwd`, asserting success.
fn git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
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

/// Make `root` a real git repo, then run `jigc setup` over it.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A **real** Keep-a-Changelog file — the stock brownfield `CHANGELOG.md`, verbatim in the
/// shape the convention prescribes. It carries no schema-version stamp and parses against
/// no shipped `changelog` schema version (neither v2 nor the v1 snapshot): a **foreign**
/// file squatting at the `changelog` doctype's placement home.
const KEEP_A_CHANGELOG: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- A new thing.

## [0.1.0] - 2026-01-01

### Added

- The first thing.
";

/// A **v0-era** ADR: the prior (v1) shape — the three prose slots, **no `## Options`** — and
/// **no** `schema-version` stamp (it predates the stamp). The M34 headline detect case: a
/// genuinely **managed** doc that a *stamp-only* discriminator would misread as foreign.
const ADR_V0_ERA: &str = "\
---
status: accepted
date: 2026-06-25
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// A **current, conformant** ADR: the v2 shape, stamped `schema-version: 2`.
const ADR_CURRENT: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// Commit an ADR body at the `adr` doctype's canonical home.
fn commit_adr(repo: &Path, body: &str) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("cache-sessions-in-memory.md"), body).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
}

/// The findings of `jigc validate --format json`, plus the exit code.
fn validate_findings(repo: &Path, home: &Path) -> (i32, Vec<serde_json::Value>) {
    let out = jigc(repo, home, &["validate", "--format", "json"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!("`jigc validate --format json` must emit JSON ({err}):\n{stdout}")
    });
    let findings = json["findings"]
        .as_array()
        .expect("the report carries a `findings` array")
        .clone();
    (out.status.code().expect("an exit code"), findings)
}

/// Every finding of `code` in the report.
fn by_code<'a>(findings: &'a [serde_json::Value], code: &str) -> Vec<&'a serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["code"].as_str() == Some(code))
        .collect()
}

/// (foreign) A **stock brownfield repo** — a real Keep-a-Changelog `CHANGELOG.md` and `jigc
/// setup`, nothing else — yields **zero blocking findings** and exactly **one** advisory
/// `schema-conformance.unadopted-instance`, addressed at the **file path** and routed at the
/// adoption verbs (`jigc ingest` / `jigc migrate CHANGELOG.md --as changelog`), **exit 0**.
///
/// Red before the discriminator: four blocking `conformance.*` findings routed at `jigc
/// migrate-corpus`, a verb that reports `1 blocked` on that file.
#[test]
fn a_foreign_changelog_at_the_placement_home_is_an_adoption_case_not_an_unmigrated_corpus() {
    let repo = TempDir::new("foreign");
    let home = TempDir::new("home");
    fs::write(repo.path().join("CHANGELOG.md"), KEEP_A_CHANGELOG).expect("write CHANGELOG.md");
    setup_repo(repo.path(), home.path());

    let (code, findings) = validate_findings(repo.path(), home.path());

    let blocking: Vec<_> = findings
        .iter()
        .filter(|f| f["severity"].as_str() == Some("blocking"))
        .collect();
    assert!(
        blocking.is_empty(),
        "a never-adopted foreign file the user never handed to jigc must block nothing; got: {blocking:#?}",
    );

    let unadopted = by_code(&findings, "schema-conformance.unadopted-instance");
    assert_eq!(
        unadopted.len(),
        1,
        "exactly one adoption advisory for the squatter; got: {findings:#?}",
    );
    let finding = unadopted[0];
    assert_eq!(
        finding["severity"].as_str(),
        Some("advisory"),
        "the adoption advisory is advisory, never a gate: {finding:#?}",
    );
    assert_eq!(
        finding["location"]["address"].as_str(),
        Some("CHANGELOG.md"),
        "a foreign file has no managed identity to claim — it is addressed at its PATH: {finding:#?}",
    );
    let route = finding["route"]
        .as_str()
        .expect("the advisory carries a route");
    assert!(
        route.contains("jigc ingest"),
        "the route names the adoption front door; got: {route}",
    );
    assert!(
        route.contains("jigc migrate CHANGELOG.md --as changelog"),
        "the route names the doctype-directed adoption verb, verbatim; got: {route}",
    );
    assert!(
        !route.contains("migrate-corpus"),
        "a foreign file is NOT an unmigrated corpus — the route must not name `migrate-corpus`; got: {route}",
    );
    assert_eq!(code, 0, "the brownfield first-run stays exit 0");
}

/// (managed, v0-era) An **unstamped** ADR in the shipped **prior (v1)** shape is a genuinely
/// **managed** doc — the M34 headline detect case. It is routed at the **corpus migration**
/// and is never called unadopted: the parse-against-a-shipped-prior arm of the classifier is
/// exactly what separates it from the foreign file above.
#[test]
fn an_unstamped_v0_era_managed_doc_still_reads_as_managed() {
    let repo = TempDir::new("v0");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_V0_ERA);

    let (code, findings) = validate_findings(repo.path(), home.path());

    assert!(
        by_code(&findings, "schema-conformance.unadopted-instance").is_empty(),
        "a v0-era MANAGED doc must never be routed at adoption; got: {findings:#?}",
    );
    let routed_at_migration: Vec<_> = findings
        .iter()
        .filter(|f| {
            f["route"]
                .as_str()
                .is_some_and(|r| r.contains("corpus migration"))
        })
        .collect();
    assert!(
        !routed_at_migration.is_empty(),
        "the stale managed doc routes at the corpus migration; got: {findings:#?}",
    );
    assert_eq!(code, 0, "the store sweep is report-only here");
}

/// (fresh clone) A **clone of a managed repo** carries **no `.jigc/state`** (it is gitignored
/// and rebuildable), so a discriminator keyed on the file-state record would read **every**
/// committed doc as foreign and tell a teammate to `ingest` their own corpus. Keyed on the
/// committed bytes it does not: the conformant, stamped ADR reads as managed — no adoption
/// advisory, nothing blocking.
#[test]
fn a_fresh_clone_of_a_managed_repo_still_reads_its_docs_as_managed() {
    let repo = TempDir::new("origin");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_CURRENT);

    let clones = TempDir::new("clones");
    let clone = clones.path().join("fresh");
    git(
        clones.path(),
        &[
            "clone",
            "-q",
            repo.path().to_str().expect("utf-8 path"),
            clone.to_str().expect("utf-8 path"),
        ],
    );
    assert!(
        !clone.join(".jigc").join("state").exists(),
        "the fresh clone must carry no file-state record — that is the whole point",
    );

    let (code, findings) = validate_findings(&clone, home.path());

    assert!(
        by_code(&findings, "schema-conformance.unadopted-instance").is_empty(),
        "a fresh clone's committed corpus is MANAGED, not foreign; got: {findings:#?}",
    );
    let blocking: Vec<_> = findings
        .iter()
        .filter(|f| f["severity"].as_str() == Some("blocking"))
        .collect();
    assert!(
        blocking.is_empty(),
        "a conformant, stamped corpus blocks nothing on a fresh clone; got: {blocking:#?}",
    );
    assert_eq!(code, 0, "a clean clone exits 0");
}
