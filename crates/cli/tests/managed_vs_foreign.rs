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
//! - **(managed, stale — Inc 3 T2)** a **v1-stamped** ADR under the v2 manifest surfaces the
//!   version-currency break `schema-conformance.schema-version-current` (blocking, its own check
//!   id, at the doc's URI) routed at **`jigc migrate-corpus`** — and that verb clears it, the
//!   corpus re-validating clean.
//!
//! - **(fresh clone)** a clone of a managed repo — no `.jigc/state`, so **no** file-state
//!   record at all — still reads its committed docs as **managed**: no adoption advisory,
//!   nothing blocking.
//! - **(the `binary-mismatch` route — T4)** a store whose `.jigc/version` stamp is stale **and**
//!   whose corpus is stale routes at **`jigc migrate-corpus` first**, then the re-stamp. The old
//!   route (*"align versions or re-run `jigc setup`"*) was a **false all-clear**: `jigc setup`
//!   re-stamps `.jigc/version` and thereby **self-clears its own advisory** while the corpus
//!   stays stale. The discriminator is the machine handle T2 minted — the
//!   `schema-conformance.schema-version-current` finding in the same report — so a store on a
//!   divergent binary with a **current** corpus keeps the plain align-or-re-stamp route.
//!
//! **Increment 4, T2 — the exit flips on the managed arm.** The store sweep's report-only rule
//! keeps every *content* finding at exit 0 (the masking-trap rationale, conceded in full), but an
//! **unmigrated managed corpus** meets the exit-flipping class's own criterion — *the sweep could
//! not produce a trustworthy result*: every other family is adjudicating docs against a schema
//! they were never written to. So `render::validation_store_exit_flips` gains
//! `schema-conformance.schema-version-current` as its **third** exception, and the JSON
//! `report_only` field + the human/agent trailer follow the *same* predicate, by construction
//! (`design/validation.md` → Exit semantics — the third exception and its two pinned conditions).
//! The arms above/below pin every half of that contract:
//!
//! - a stale **v1-stamped** ADR and an **unstamped v0-era** managed doc each exit **non-zero**,
//!   with `report_only: false` and a trailer naming **`jigc migrate-corpus`**;
//! - the **stock brownfield** repo (the foreign arm above) still exits **0** — the "managed arm
//!   only" condition, proven rather than re-implemented: the code is *emitted* only on the managed
//!   arm, so keying the exit on the code **is** the condition;
//! - a **migrated** corpus exits **0** (detect → fix → clean, end to end);
//! - a **v2-stamped ADR with an invalid `status` enum** raises its **blocking**
//!   `schema-conformance.field-value-conformant` and still exits **0** — ordinary content drift,
//!   report-only, the masking trap the per-code key exists to avoid; and
//! - the **pre-commit hook** is unaffected — it keys on the *findings* in `validate --format
//!   json`, never the exit code, and always exits 0 outside the M35 rename block, so a commit over
//!   a stale corpus still lands.

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

/// A **stale managed** ADR: the shipped **prior (v1)** shape, stamped `schema-version: 1`
/// while the `adr` manifest is at 2 — the commonest stale doc in a real corpus, and the one
/// that does **not parse** under the current (v2) shape (the optional `## Options` slot makes
/// `## Decision` read as a renamed section), so it lands in the store family's **parse-failure**
/// arm.
const ADR_STALE_V1: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 1
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// A **fully-migrated but structurally broken** ADR: the v2 shape, stamped `schema-version: 2`
/// — so the corpus is **current** and no version-currency break fires — with `## Consequences`
/// **removed**. It fails to *parse* under the current shape, so the store family's
/// **parse-failure** arm surfaces its `conformance.*` findings **directly** (`crates/engine/
/// src/validate.rs`, the `parse_sections` `Err` arm). At **task** scope the very same committed
/// doc is never adjudicated by that path at all — it goes through the file-state reconciler,
/// which grades it by **baseline membership**.
const ADR_V2_MISSING_CONSEQUENCES: &str = "\
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

/// A **current-shape but non-conformant** ADR: the v2 shape, stamped `schema-version: 2` — so the
/// corpus is *migrated* — carrying an out-of-enum `status`. Ordinary **content drift**: it raises
/// a blocking `schema-conformance.field-value-conformant` and must **stay exit-0** (Inc 4 T2 — the
/// masking-trap guard the per-code exit key exists to preserve).
const ADR_V2_BAD_STATUS: &str = "\
---
status: rejected
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

/// The whole `jigc validate --format json` envelope, plus the exit code.
fn validate_json(repo: &Path, home: &Path) -> (i32, serde_json::Value) {
    let out = jigc(repo, home, &["validate", "--format", "json"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!("`jigc validate --format json` must emit JSON ({err}):\n{stdout}")
    });
    (out.status.code().expect("an exit code"), json)
}

/// The findings of `jigc validate --format json`, plus the exit code.
fn validate_findings(repo: &Path, home: &Path) -> (i32, Vec<serde_json::Value>) {
    let (code, json) = validate_json(repo, home);
    let findings = json["findings"]
        .as_array()
        .expect("the report carries a `findings` array")
        .clone();
    (code, findings)
}

/// The `report_only` field of the store envelope — the JSON half of the exit contract, which
/// must agree with the exit code by construction (both read one predicate).
fn report_only(repo: &Path, home: &Path) -> bool {
    let (_, json) = validate_json(repo, home);
    json["report_only"]
        .as_bool()
        .expect("the store envelope carries a `report_only` bool")
}

/// The default (agent-format) `jigc validate` stdout — the trailer half of the exit contract.
fn validate_text(repo: &Path, home: &Path) -> (i32, String) {
    let out = jigc(repo, home, &["validate"]);
    (
        out.status.code().expect("an exit code"),
        String::from_utf8(out.stdout).expect("utf-8 stdout"),
    )
}

/// Assert the three surfaces of the exit contract agree on the **unmigrated-corpus** verdict:
/// exit non-zero, `report_only: false`, and a trailer that says so **and** names the verb that
/// fixes it (`jigc migrate-corpus`) — never the report-only sentence, which for this finding
/// claims a task-scope gate that does not exist.
fn assert_unmigrated_corpus_verdict(repo: &Path, home: &Path) {
    let (code, findings) = validate_findings(repo, home);
    assert_eq!(
        by_code(&findings, "schema-conformance.schema-version-current").len(),
        1,
        "the precondition of this arm — one version-currency break; got: {findings:#?}",
    );
    assert_ne!(
        code, 0,
        "an unmigrated MANAGED corpus flips the exit — every other family is adjudicating docs \
         against a schema they were never written to, so the sweep is not trustworthy",
    );
    assert!(
        !report_only(repo, home),
        "the JSON `report_only` reads the same predicate as the exit code — it must say false",
    );

    let (text_code, text) = validate_text(repo, home);
    assert_eq!(
        text_code, code,
        "the agent view exits the same way the JSON one does",
    );
    assert!(
        text.contains("jigc migrate-corpus"),
        "the trailer names the verb that clears it, verbatim; got:\n{text}",
    );
    assert!(
        text.contains("non-zero"),
        "the trailer states the exit it actually takes; got:\n{text}",
    );
    assert!(
        !text.contains("report-only at store scope (exit 0)"),
        "the report-only trailer would print `exit 0` while the tool exits {code}; got:\n{text}",
    );
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

    // **Exactly one finding, and it is the adoption advisory (T3).** Family 3's read-only
    // file↔CLI-state twin reached the same squatter and called it `file-state.un-baselined`
    // — *"no action needed — the doc is baselined on its next author or finalize"*, a promise
    // about a file jigc will never author. One foreign file is **one** fact, and it is the
    // adoption case; the discriminator suppresses the un-baselined arm for it too
    // (`design/validation.md` → the same discriminator applies to family 3's `un-baselined`).
    assert_eq!(
        findings.len(),
        1,
        "a foreign squatter is exactly ONE finding — the adoption advisory, nothing else; got: \
         {findings:#?}",
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

    // **The exit flip's "managed arm only" condition (Inc 4 T2), proven rather than
    // re-implemented.** `schema-conformance.schema-version-current` is emitted only on the
    // managed arm, so keying the exit predicate on that code *is* the condition — and this is
    // the arm that would break if it were not: a stock brownfield repo that has only ever run
    // `jigc setup` must not see `jigc validate` go non-zero.
    assert_eq!(code, 0, "the brownfield first-run stays exit 0");
    assert!(
        report_only(repo.path(), home.path()),
        "and says so in the envelope — the adoption advisory is report-only",
    );

    // **The trailer claims a gate only where one exists (Inc 4 T3).** This repo's one finding —
    // the adoption advisory — is store-scope-only: no task-scope path emits that code, so it
    // gates at neither `jigc task validate` nor `jigc task finalize`. The blanket trailer said
    // it did. On the real binary, over the corpus the stock brownfield repo actually produces,
    // the claim is gone.
    let (_, text) = validate_text(repo.path(), home.path());
    assert!(
        !text.contains("jigc task validate") && !text.contains("jigc task finalize"),
        "the sole finding gates nowhere — the trailer must not send the reader to a gate that \
         will never see it; got:\n{text}",
    );
    assert!(
        text.contains("report-only at store scope (exit 0)"),
        "it is still the report-only branch, and still names the exit; got:\n{text}",
    );
    assert!(
        text.contains("gates nowhere"),
        "and states what is actually true of it; got:\n{text}",
    );
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

    let (_, findings) = validate_findings(repo.path(), home.path());

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

    // The unstamped v0-era corpus is *unmigrated*, so the sweep is untrustworthy and the exit
    // flips (Inc 4 T2) — the second managed arm, alongside the below-version stamp.
    assert_unmigrated_corpus_verdict(repo.path(), home.path());
}

/// (managed, stale — the version-currency break) A committed **v1-stamped** ADR under the **v2**
/// manifest is a *managed* doc the corpus migration can upgrade, and the sweep now says so **as
/// data**: exactly one **blocking** `schema-conformance.schema-version-current` (M42 — its own
/// check id, the fact the Inc-4 exit predicate keys on), addressed at the doc's `<type>:<slug>`
/// URI and routed at the verb that actually fixes it, `jigc migrate-corpus`. Running that verb
/// **clears** it — detect → fix → re-validate clean, on the real binary.
///
/// Red before this task, and *loudly*: this ADR is in the prior shape, so it does not parse
/// under v2 — the store family's **parse-failure** arm emitted only routed `conformance.*`
/// parse findings and **zero** `schema-conformance.*` findings. The commonest stale doc in a
/// real corpus carried **no machine-readable staleness fact at all**.
#[test]
fn a_stale_v1_stamped_adr_surfaces_the_version_currency_break_and_migrate_corpus_clears_it() {
    let repo = TempDir::new("stale-managed");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_STALE_V1);

    // 1. DETECT — the version-currency break, its own code, blocking, at the doc's URI,
    //    routed at `jigc migrate-corpus`; and (Inc 4 T2) the sweep exits **non-zero**.
    let (_, findings) = validate_findings(repo.path(), home.path());
    let stale = by_code(&findings, "schema-conformance.schema-version-current");
    assert_eq!(
        stale.len(),
        1,
        "a v1-stamped ADR under the v2 manifest surfaces exactly one version-currency break; \
         got: {findings:#?}",
    );
    let finding = stale[0];
    assert_eq!(
        finding["severity"].as_str(),
        Some("blocking"),
        "the version-currency break is blocking: {finding:#?}",
    );
    assert_eq!(
        finding["key"]["target"].as_str(),
        Some("adr:cache-sessions-in-memory"),
        "it is addressed at the managed doc's URI — unique per instance: {finding:#?}",
    );
    let route = finding["route"]
        .as_str()
        .expect("the break carries a route");
    assert!(
        route.contains("jigc migrate-corpus"),
        "the route names the verb that upgrades a managed corpus, verbatim; got: {route}",
    );
    assert!(
        by_code(&findings, "schema-conformance.unadopted-instance").is_empty(),
        "a stale MANAGED doc is never an adoption case; got: {findings:#?}",
    );
    assert_unmigrated_corpus_verdict(repo.path(), home.path());

    // 2. MIGRATE — the routed verb, run verbatim as the finding names it.
    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    assert!(
        migrate.status.success(),
        "`jigc migrate-corpus` must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&migrate.stdout),
        String::from_utf8_lossy(&migrate.stderr),
    );

    // 3. RE-VALIDATE — the loop closes: the break is gone, nothing blocks, exit 0.
    let (code, after) = validate_findings(repo.path(), home.path());
    assert!(
        by_code(&after, "schema-conformance.schema-version-current").is_empty(),
        "the migrated doc carries no version-currency break; got: {after:#?}",
    );
    let blocking: Vec<_> = after
        .iter()
        .filter(|f| f["severity"].as_str() == Some("blocking"))
        .collect();
    assert!(
        blocking.is_empty(),
        "the migrated corpus blocks nothing; got: {blocking:#?}",
    );
    assert_eq!(code, 0, "a migrated corpus exits 0");
    assert!(
        report_only(repo.path(), home.path()),
        "and the envelope is back to report-only",
    );
}

/// (migrated, but non-conformant — the masking-trap guard) A **v2-stamped** ADR — a *migrated*
/// corpus — carrying an out-of-enum `status` raises its **blocking**
/// `schema-conformance.field-value-conformant` and **still exits 0**.
///
/// This is the arm the whole per-code key exists for. The version break is emitted through the
/// same `schema-conformance.*` family as this one, and until M42 wore the *same check id*; a
/// predicate keyed on `field-value-conformant` (or on a route-string prefix) would flip the exit
/// here too — turning `jigc validate` into a gate over **pre-existing content drift a commit did
/// not cause**, which is precisely the masking trap the report-only rule exists to prevent
/// (`design/validation.md` → Exit semantics — the report-only rationale, conceded in full).
#[test]
fn an_invalid_enum_on_a_migrated_adr_blocks_but_never_flips_the_store_exit() {
    let repo = TempDir::new("bad-enum");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_V2_BAD_STATUS);

    let (code, findings) = validate_findings(repo.path(), home.path());

    let value_break = by_code(&findings, "schema-conformance.field-value-conformant");
    assert_eq!(
        value_break.len(),
        1,
        "an out-of-enum `status` raises exactly one value-conformance break; got: {findings:#?}",
    );
    assert_eq!(
        value_break[0]["severity"].as_str(),
        Some("blocking"),
        "and it is blocking — a finalize/task-scope verdict: {:#?}",
        value_break[0],
    );
    assert!(
        by_code(&findings, "schema-conformance.schema-version-current").is_empty(),
        "the corpus is MIGRATED — no version-currency break; got: {findings:#?}",
    );

    assert_eq!(
        code, 0,
        "ordinary content drift stays report-only at store scope — a blocking `blocking` finding \
         must NOT flip the store exit, or `jigc validate` gates on drift the commit never caused",
    );
    assert!(
        report_only(repo.path(), home.path()),
        "and the envelope says report-only",
    );
    let (_, text) = validate_text(repo.path(), home.path());
    assert!(
        text.contains("report-only at store scope (exit 0)"),
        "the report-only trailer stands for content findings; got:\n{text}",
    );
}

/// (the pre-commit hook is unaffected) The warn-only backstop keys on the **findings** in
/// `jigc validate --format json`, never on the exit code, and always exits 0 outside the M35
/// rename block. Asserted, not assumed: over a corpus stale enough that the hook's own sweep now
/// exits **non-zero**, an ordinary commit still lands.
#[test]
fn the_pre_commit_hook_still_lets_a_commit_land_over_an_exit_flipping_stale_corpus() {
    let repo = TempDir::new("hook-stale");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_STALE_V1);

    // The committed store is now stale — the hook's own `jigc validate` exits non-zero.
    let (code, _) = validate_findings(repo.path(), home.path());
    assert_ne!(
        code, 0,
        "the precondition: the sweep the hook runs exits non-zero"
    );

    // An ordinary, unrelated commit — with the probe resolvable, so the hook runs the *real*
    // sweep rather than bailing on a missing probe.
    fs::write(repo.path().join("note.txt"), "unrelated\n").expect("write file");
    git(repo.path(), &["add", "note.txt"]);
    let out = Command::new("git")
        .args(["commit", "-q", "-m", "an unrelated change"])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run git commit");
    assert!(
        out.status.success(),
        "the warn-only hook keys on findings, never the exit code — a commit over a stale corpus \
         must still land; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Rewrite the committed binary-provenance stamp to a **different** build, so the store reads
/// as "last written by another `jigc`" — the `store-version.binary-mismatch` precondition.
fn stale_version_stamp(repo: &Path) {
    let stamp = repo.join(".jigc").join("version");
    assert!(
        stamp.is_file(),
        "`jigc setup` must have written the provenance stamp at {stamp:?}",
    );
    fs::write(&stamp, "jigc-version: 0.9.0-elsewhere\n").expect("rewrite the version stamp");
}

/// The one `store-version.binary-mismatch` advisory in a report, or `None`.
fn binary_mismatch(findings: &[serde_json::Value]) -> Option<&serde_json::Value> {
    let hits = by_code(findings, "store-version.binary-mismatch");
    assert!(
        hits.len() <= 1,
        "the store is a singleton — at most one binary-mismatch advisory; got: {hits:#?}",
    );
    hits.first().copied()
}

/// (T4 — the `binary-mismatch` route stops producing a false all-clear) A store whose
/// `.jigc/version` stamp is **stale** *and* whose committed corpus is **stale** (a v1-stamped ADR
/// under the v2 manifest) routes at **`jigc migrate-corpus`** — the verb that actually fixes the
/// corpus — before the re-stamp.
///
/// Red before this task, and driven end-to-end to prove *why*: the advisory routed only *"align
/// versions or re-run `jigc setup`"*, and running `jigc setup` **re-stamps `.jigc/version`**,
/// which **self-clears the advisory** — a **false all-clear**, since the corpus is still stale.
/// This test walks exactly that path: route → `jigc setup` → re-validate, and pins that the
/// binary-mismatch advisory is gone while the version-currency break **remains**. The route the
/// user is handed must therefore name the corpus migration first, or the tool talks them into a
/// green light over a stale corpus.
#[test]
fn a_stale_stamp_over_a_stale_corpus_routes_at_migrate_corpus_and_setup_alone_is_no_all_clear() {
    let repo = TempDir::new("stale-stamp-stale-corpus");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_STALE_V1);
    stale_version_stamp(repo.path());

    // 1. The advisory names the verb that fixes the CORPUS, verbatim — and the re-stamp only
    //    after it (re-stamping alone leaves the corpus stale).
    let (code, findings) = validate_findings(repo.path(), home.path());
    let mismatch = binary_mismatch(&findings).expect("a divergent stamp raises the advisory");
    let route = mismatch["route"]
        .as_str()
        .expect("the advisory carries a route (the advisory-route floor)");
    assert!(
        route.contains("jigc migrate-corpus"),
        "a store whose CORPUS is also stale must be routed at the corpus migration, verbatim; \
         got: {route}",
    );
    assert!(
        route.contains("jigc setup"),
        "the re-stamp is still part of the route — after the migration; got: {route}",
    );
    assert!(
        route.find("jigc migrate-corpus") < route.find("jigc setup"),
        "the corpus migration comes FIRST — `jigc setup` re-stamps and self-clears this advisory, \
         so naming it first is the false all-clear; got: {route}",
    );
    assert_eq!(
        mismatch["severity"].as_str(),
        Some("advisory"),
        "the provenance stamp never gates: {mismatch:#?}",
    );
    assert_eq!(
        by_code(&findings, "schema-conformance.schema-version-current").len(),
        1,
        "the stale corpus is the precondition of this arm; got: {findings:#?}",
    );
    // The exit is non-zero here because the **corpus** is unmigrated (Inc 4 T2), never because
    // of the advisory — the sibling arm below (a divergent binary over a *current* corpus) is
    // what pins that the provenance advisory itself never flips the exit.
    assert_ne!(code, 0, "the unmigrated corpus flips the exit");

    // 2. Run `jigc setup` — the verb the OLD route named — and re-validate.
    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // 3. The all-clear is a LIE: setup re-stamped `.jigc/version` (its own advisory is gone) but
    //    the corpus is untouched — the version-currency break still stands. This is exactly what
    //    the corrected route exists to stop a human from mistaking for "handled".
    let (code, after) = validate_findings(repo.path(), home.path());
    assert!(
        binary_mismatch(&after).is_none(),
        "`jigc setup` re-stamps the store, so its own advisory clears; got: {after:#?}",
    );
    assert_eq!(
        by_code(&after, "schema-conformance.schema-version-current").len(),
        1,
        "the CORPUS is still stale after a bare re-stamp — the all-clear was never real; \
         got: {after:#?}",
    );
    assert_ne!(
        code, 0,
        "and the exit stays non-zero after the bare re-stamp — the false all-clear is now false \
         to a machine too, not just to a reader (Inc 4 T2)",
    );
}

/// (T4 — the omitting context) A store on a **divergent binary** whose corpus is **current** (a
/// v2-stamped ADR under the v2 manifest) keeps the **plain** align-or-re-stamp route: the
/// migrate-corpus route must not leak into every mismatch. The discriminator is the
/// `schema-conformance.schema-version-current` finding in the same report — a machine handle,
/// absent here — so a naive "always name `migrate-corpus`" fix would command a verb with nothing
/// to do (`0 migrated, 1 already current`), the very false-route class this increment deletes.
#[test]
fn a_current_corpus_on_a_divergent_binary_keeps_the_plain_re_stamp_route() {
    let repo = TempDir::new("stale-stamp-current-corpus");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_CURRENT);
    stale_version_stamp(repo.path());

    let (code, findings) = validate_findings(repo.path(), home.path());
    assert!(
        by_code(&findings, "schema-conformance.schema-version-current").is_empty(),
        "the corpus is current — the precondition of this arm; got: {findings:#?}",
    );

    let mismatch = binary_mismatch(&findings).expect("a divergent stamp raises the advisory");
    let route = mismatch["route"]
        .as_str()
        .expect("the advisory carries a route (the advisory-route floor)");
    assert!(
        !route.contains("migrate-corpus"),
        "a CURRENT corpus is not a migration case — the route must not name `migrate-corpus`; \
         got: {route}",
    );
    assert!(
        route.contains("jigc setup"),
        "the plain route stands: align versions, or re-run `jigc setup` to re-stamp; got: {route}",
    );
    assert_eq!(code, 0, "the binary-mismatch advisory never flips the exit");
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

/// (Inc 4 T3 — the gate claim over a **committed doc's** conformance break) The report-only
/// trailer's sentence *"these gate at `jigc task validate` / `jigc task finalize`"* is **false**
/// for a `conformance.*` finding the store sweep raises over an **un-baselined** committed doc —
/// and un-baselined is the state of **every** committed doc in a fresh clone (`.jigc/state/` is
/// gitignored), in a brownfield adoption, and in any hand-authored corpus: the dominant `jigc
/// validate` corpus.
///
/// The two scopes adjudicate the same file through **different paths**:
///
/// - **store**: the fifth family parses the committed doc itself, so a parse failure surfaces its
///   `conformance.*` findings **directly** (`crates/engine/src/validate.rs`, the `Err` arm);
/// - **task**: `validate_task` parses only the task's *staged* instances. The committed doc goes
///   through `file_state::reconcile_committed_store`, whose **UNKNOWN** arm (no baseline recorded)
///   grades a nonconformant file **advisory** (`conformance_advisory_finding` — *routed, not
///   recorded*), never blocking.
///
/// So the finding gates **nowhere, permanently**, while the trailer sends the reader to a gate
/// that will never see it. Driven through the real binary, over the corpus a fresh clone actually
/// produces — and the emitted bytes are read verbatim, not reconstructed.
#[test]
fn a_conformance_break_on_an_un_baselined_committed_doc_gates_nowhere_and_the_trailer_says_so() {
    let repo = TempDir::new("unbaselined-gate-claim");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_V2_MISSING_CONSEQUENCES);

    // The precondition: a v2-STAMPED (fully migrated) doc — so this is not the unmigrated-corpus
    // branch — whose missing `## Consequences` raises a blocking `conformance.*` parse finding,
    // report-only at exit 0.
    let (code, findings) = validate_findings(repo.path(), home.path());
    assert!(
        by_code(&findings, "schema-conformance.schema-version-current").is_empty(),
        "the corpus is MIGRATED — this is the report-only branch, not the unmigrated one; \
         got: {findings:#?}",
    );
    let missing = by_code(&findings, "conformance.section-missing");
    assert_eq!(
        missing.len(),
        1,
        "the removed `## Consequences` raises one section-missing break; got: {findings:#?}",
    );
    assert_eq!(
        missing[0]["severity"].as_str(),
        Some("blocking"),
        "and it renders `blocking` — which is exactly why the gate claim must be true: {:#?}",
        missing[0],
    );
    assert_eq!(
        by_code(&findings, "file-state.un-baselined").len(),
        1,
        "the doc is UN-BASELINED — the state of every committed doc in a fresh clone, and the \
         discriminator this fix turns on; got: {findings:#?}",
    );
    assert_eq!(code, 0, "content drift stays report-only at store scope");

    // The gate the trailer claims does not exist: at task scope the SAME doc is advisory.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "prove the gate"],
        ),
        "`jigc start`",
    );
    let task = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "prove-the-gate", "--format", "json"],
    );
    let task_findings = findings_of(&task);
    assert!(
        by_code(&task_findings, "conformance.section-missing").is_empty(),
        "no task-scope path emits the store sweep's `conformance.*` code over a COMMITTED doc — \
         the committed store is adjudicated by the reconciler, not the parser; got: \
         {task_findings:#?}",
    );
    let block = by_code(&task_findings, "reconciliation.conformance-block");
    assert_eq!(
        block.len(),
        1,
        "the reconciler is the only task-scope path that sees it; got: {task_findings:#?}",
    );
    assert_eq!(
        block[0]["severity"].as_str(),
        Some("advisory"),
        "and on an UN-BASELINED doc it is ADVISORY — routed, not recorded, and gating nothing: \
         {:#?}",
        block[0],
    );

    // Therefore the trailer must not claim a gate for it.
    let (_, text) = validate_text(repo.path(), home.path());
    assert!(
        !text.contains("jigc task validate") && !text.contains("jigc task finalize"),
        "every finding here gates nowhere — the trailer must not send the reader to a gate that \
         will never fire; got:\n{text}",
    );
    assert!(
        text.contains("report-only at store scope (exit 0)"),
        "it is still the report-only branch, and still names the exit; got:\n{text}",
    );
    assert!(
        text.contains("gates nowhere"),
        "and states what is actually true of it; got:\n{text}",
    );
}

/// (Inc 4 T3 — the **non-vacuous half**) The fix must not silence the claim where the gate is
/// real. A doc jigc has **baselined** (its hash is in `.jigc/state/file-state.json`) and that is
/// then broken **out of band** takes the reconciler's `DRIFTED` arm, where a nonconformant file is
/// **blocking** `reconciliation.conformance-block` — a genuine `jigc task validate` / `jigc task
/// finalize` gate. The trailer keeps the sentence for it, and the gate is proven to fire, through
/// the real binary.
#[test]
fn a_conformance_break_on_a_baselined_committed_doc_really_does_gate_and_the_trailer_keeps_it() {
    let repo = TempDir::new("baselined-gate-claim");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_CURRENT);

    // Baseline the committed ADR the way jigc really does: the **finalize** preflight's
    // committed-store reconcile adopts the conformant doc (its UNKNOWN + conformant arm) and
    // finalize **persists** the swept record into gitignored `.jigc/state/file-state.json`.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "seed the baseline"],
        ),
        "`jigc start`",
    );
    fill_commit_doc(repo.path(), home.path(), "seed-the-baseline");
    fs::write(repo.path().join("limiter.rs"), "// a rate limiter\n").expect("write code change");
    git(repo.path(), &["add", "limiter.rs"]);
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", "seed-the-baseline"],
        ),
        "`jigc task finalize`",
    );

    // Break it out of band — the drift the reconciler blocks on.
    let adr = repo
        .path()
        .join("docs")
        .join("decisions")
        .join("cache-sessions-in-memory.md");
    fs::write(&adr, ADR_V2_MISSING_CONSEQUENCES).expect("break the baselined ADR");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "break the adr"]);

    // The store sweep raises the same `conformance.*` break — but this doc IS baselined.
    let (_, findings) = validate_findings(repo.path(), home.path());
    assert_eq!(
        by_code(&findings, "conformance.section-missing").len(),
        1,
        "the same store-scope break; got: {findings:#?}",
    );
    assert!(
        by_code(&findings, "file-state.un-baselined").is_empty(),
        "and the doc is BASELINED — the discriminator's other side; got: {findings:#?}",
    );

    // The gate is real: task scope blocks on it.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "prove the gate"],
        ),
        "`jigc start`",
    );
    let task = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "prove-the-gate", "--format", "json"],
    );
    let task_findings = findings_of(&task);
    let block = by_code(&task_findings, "reconciliation.conformance-block");
    assert_eq!(
        block.len(),
        1,
        "the drifted baselined doc reaches the reconciler's blocking arm; got: {task_findings:#?}",
    );
    assert_eq!(
        block[0]["severity"].as_str(),
        Some("blocking"),
        "and it BLOCKS — the gate the trailer names really exists here: {:#?}",
        block[0],
    );

    // So the trailer keeps the claim.
    let (_, text) = validate_text(repo.path(), home.path());
    assert!(
        text.contains("jigc task validate") && text.contains("jigc task finalize"),
        "the gate exists for this finding — suppressing the claim here would be the opposite \
         lie; got:\n{text}",
    );
}

/// The `state` `jigc doc list --format json` reports for `id`, or `None` when it lists no
/// such instance — the fourth read surface's verdict, read from its emitted bytes.
fn listed_state(repo: &Path, home: &Path, id: &str) -> Option<String> {
    let out = jigc(repo, home, &["doc", "list", "--format", "json"]);
    assert_ok(&out, "`jigc doc list --format json`");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|err| panic!("`doc list --format json` must emit JSON ({err}):\n{stdout}"));
    json["docs"]
        .as_array()
        .expect("the listing carries a `docs` array")
        .iter()
        .find(|row| row["id"].as_str() == Some(id))
        .map(|row| {
            row["state"]
                .as_str()
                .expect("every row carries a state")
                .to_string()
        })
}

/// The stderr of a `jigc doc show <addr>` that must **block** — the emitted block envelope
/// (code + message + `route:`), read verbatim off the real binary.
fn show_block_stderr(repo: &Path, home: &Path, addr: &str, format: &[&str]) -> String {
    let mut args = vec!["doc", "show", addr];
    args.extend_from_slice(format);
    let out = jigc(repo, home, &args);
    assert!(
        !out.status.success(),
        "`jigc doc show {addr}` {format:?} must block on a doc that does not parse; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    String::from_utf8(out.stderr).expect("utf-8 stderr")
}

/// (T7 — the **foreign** arm) `jigc doc show` over a never-adopted foreign file squatting at a
/// managed home routes at **adoption**, not at hand-repair — and the three read/check surfaces
/// tell **one story** about that file (`design/doc-read-surface.md` → `jigc doc list`, third
/// bullet: *"`doc show`'s block on an unregistered instance routes to adoption"*).
///
/// Red before this task: `doc show changelog:changelog` blocked `store.unparseable` routed
/// *"fix the committed file so it conforms to its schema"* — true of a corrupted **managed** doc,
/// a **lie** about a stock brownfield repo's own Keep-a-Changelog file, whose repair is `jigc
/// ingest` / `jigc migrate`. Meanwhile `doc list` already called it `unregistered` and `validate`
/// already called it foreign: three surfaces, three stories, in the wave whose whole claim is
/// that the tool's own routes tell the truth.
///
/// Both output formats are driven — the block rides the same envelope on the plain and the json
/// read path, so a driver on `--format json` must not be handed the hand-repair lie either.
#[test]
fn doc_show_over_a_foreign_squatter_routes_at_adoption_and_the_surfaces_tell_one_story() {
    let repo = TempDir::new("show-foreign");
    let home = TempDir::new("home");
    fs::write(repo.path().join("CHANGELOG.md"), KEEP_A_CHANGELOG).expect("write CHANGELOG.md");
    setup_repo(repo.path(), home.path());

    for format in [&[][..], &["--format", "json"][..]] {
        let stderr = show_block_stderr(repo.path(), home.path(), "changelog:changelog", format);
        assert!(
            stderr.contains("jigc ingest"),
            "the route names the adoption front door; got ({format:?}):\n{stderr}",
        );
        assert!(
            stderr.contains("jigc migrate CHANGELOG.md --as changelog"),
            "and the doctype-directed adoption verb, verbatim — the file's own path, so the agent \
             runs it as emitted; got ({format:?}):\n{stderr}",
        );
        assert!(
            !stderr.contains("fix the committed file so it conforms to its schema"),
            "the hand-repair route is a LIE about a file jigc never wrote; got ({format:?}):\n\
             {stderr}",
        );
        assert!(
            !stderr.contains("migrate-corpus"),
            "a foreign file is not an unmigrated corpus — that verb does nothing for it; got \
             ({format:?}):\n{stderr}",
        );
    }

    // One story, three surfaces: `doc list` says *unregistered*, `doc show` (above) says *adopt
    // it*, `validate` says *foreign — adopt it*.
    assert_eq!(
        listed_state(repo.path(), home.path(), "changelog:changelog").as_deref(),
        Some("unregistered"),
        "the fourth read surface flags the squatter",
    );
    let (code, findings) = validate_findings(repo.path(), home.path());
    assert_eq!(
        by_code(&findings, "schema-conformance.unadopted-instance").len(),
        1,
        "and the sweep calls the same file foreign; got: {findings:#?}",
    );
    assert_eq!(code, 0, "the brownfield first-run still exits 0");
}

/// (T7 — the **managed** arm; the omitting context) The split is **real, not a blanket swap**: a
/// **stamped** (v2, migrated) but structurally **corrupt** managed doc still blocks with the
/// hand-repair route — *"fix the committed file so it conforms to its schema"* — and is never
/// routed at adoption. jigc wrote this file; telling its author to `jigc migrate` it would be the
/// mirror-image lie, and a route that sent a corrupt managed ADR through the foreign-adoption path
/// would rewrite a doc that only needs its `## Consequences` back.
#[test]
fn doc_show_over_a_corrupt_managed_doc_keeps_the_repair_route() {
    let repo = TempDir::new("show-corrupt-managed");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), ADR_V2_MISSING_CONSEQUENCES);

    let stderr = show_block_stderr(
        repo.path(),
        home.path(),
        "adr:cache-sessions-in-memory",
        &[],
    );
    assert!(
        stderr.contains("fix the committed file so it conforms to its schema"),
        "a corrupt MANAGED doc is repaired, not adopted; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("jigc ingest") && !stderr.contains("jigc migrate "),
        "and it is never routed at the adoption verbs — jigc wrote this file; got:\n{stderr}",
    );

    // The same discriminator, the same story: `doc list` calls it managed.
    assert_eq!(
        listed_state(repo.path(), home.path(), "adr:cache-sessions-in-memory").as_deref(),
        Some("managed"),
        "the corrupt doc is jigc's own — the two surfaces agree",
    );
}

/// Assert a `jigc` invocation succeeded.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill `task`'s commit doc conformantly, so `jigc task finalize` clears the gate and lands its
/// commit — the only path that **persists** the file-state record (the baseline this arm needs).
fn fill_commit_doc(repo: &Path, home: &Path, task: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#type"),
                "--value",
                "docs",
            ],
        ),
        "`jigc doc set-field type`",
    );
    let summary = repo.join("summary.txt");
    fs::write(&summary, "seed the file-state baseline\n").expect("write the slot prose");
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#summary"),
                "--from-file",
                summary.to_str().expect("utf-8 path"),
            ],
        ),
        "`jigc doc set-slot summary`",
    );
    fs::remove_file(&summary).expect("remove the scratch prose file");
}

/// The `findings` array of a `--format json` validation envelope.
fn findings_of(out: &std::process::Output) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|err| panic!("`--format json` must emit JSON ({err}):\n{stdout}"));
    json["findings"]
        .as_array()
        .expect("the report carries a `findings` array")
        .clone()
}

/// The **non-migratable** arm of the adoption route (2026-07-24 mutation audit, finding
/// #3): `milestone-record` ships **no** `migrate-milestone-record` workflow in either
/// pack, so a foreign file squatting at its home must get the **ingest-only** route —
/// never `jigc migrate <path> --as milestone-record`, a verb that hard-errors ("not
/// migratable") on it. That is the M40 two-tier rule (never command a verb that
/// hard-errors) on the arm the migratable-changelog test above cannot reach: a
/// `migratable` flag inverted to `any(w.id != "migrate-<ty>")` is true whenever ANY
/// other workflow exists, and only this arm reddens for it.
#[test]
fn a_foreign_file_at_a_non_migratable_doctypes_home_routes_ingest_only() {
    let repo = TempDir::new("non-migratable-foreign");
    let home = TempDir::new("home");
    // A foreign markdown file squatting at the `milestone-record` home
    // (`docs/milestone-records/` under the default docs-root), committed before setup
    // so the store sweep walks it as a committed instance.
    let dir = repo.path().join("docs").join("milestone-records");
    fs::create_dir_all(&dir).expect("mk docs/milestone-records/");
    fs::write(
        dir.join("notes.md"),
        "# Meeting notes\n\nJust some notes nobody handed to jigc.\n",
    )
    .expect("write the foreign squatter");
    setup_repo(repo.path(), home.path());

    let (_, findings) = validate_findings(repo.path(), home.path());
    let unadopted = by_code(&findings, "schema-conformance.unadopted-instance");
    assert_eq!(
        unadopted.len(),
        1,
        "exactly one adoption advisory for the squatter; got: {findings:#?}",
    );
    let finding = unadopted[0];
    assert_eq!(
        finding["location"]["address"].as_str(),
        Some("docs/milestone-records/notes.md"),
        "the advisory is addressed at the file path: {finding:#?}",
    );
    let route = finding["route"]
        .as_str()
        .expect("the advisory carries a route");
    assert!(
        route.contains("jigc ingest"),
        "the route names the adoption front door; got: {route}",
    );
    assert!(
        !route.contains("--as milestone-record") && !route.contains("jigc migrate "),
        "`milestone-record` is not migratable — the route must NOT command the \
         hard-erroring `jigc migrate <path> --as milestone-record`; got: {route}",
    );
}
