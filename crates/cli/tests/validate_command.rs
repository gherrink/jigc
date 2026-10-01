//! M18 inc-3 — the `jigc validate` top-level command, end-to-end through the built
//! `jigc` binary against the **real** `doc-code` probe. T1 is the clean path (always
//! exit 0); T3 adds the two-class exit rule + the milestone headline acceptance flow.
//!
//! The store-scope re-validation sweep ([`engine::validate::validate_store`], inc-2) is
//! task-less by construction: it enumerates every committed doc's `code-anchor` leaves
//! and resolves each against the working tree, with no working area open. T1 wires it to
//! a new top-level `jigc validate` (the `run_ingest`/`run_upgrade` locate-preamble +
//! `render::validation` precedent). This test drives that command as a real process over
//! a committed store whose anchors all resolve, asserting the **headline clean path**
//! (`design/validation.md` → Store-scope re-validation → The command):
//!
//! - a committed `adr` (`docs/decisions/`) citing an existing symbol + a committed `spec`
//!   (`docs/specs/`) whose criterion maps to an existing `#[test]` fn → `jigc validate`
//!   **exits 0** with the clean report rendered and **no `doc-code` content finding** in
//!   stdout.
//!
//! The exit-class halves (probe pre-flight → one operational error; the
//! `pack-probe-integrity.*` non-zero exit rule) land in T2/T3; this task is the clean
//! path only (always exit 0). The `jigc` path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and the `doc-code` probe is the real one, resolved through
//! the production path with the `JIGC_DOC_CODE_PROBE` override removed (the flow29 idiom).

use crate::support::run_then_parse::stdout_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-validate-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// Run `jigc <args>` with `cwd = repo` and the `JIGC_DOC_CODE_PROBE` override removed,
/// so the real probe resolves through the production path, capturing output.
fn jigc(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>` with `cwd = repo` and an explicit `JIGC_DOC_CODE_PROBE`,
/// capturing output — lets the pre-flight test point the override at a path that
/// does not resolve to an executable.
fn jigc_with_probe(repo: &Path, args: &[&str], probe: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("JIGC_DOC_CODE_PROBE", probe)
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

    fs::create_dir_all(repo.join("docs/decisions")).expect("mk decisions");
    fs::write(
        repo.join("docs/decisions/cache.md"),
        adr("crates/engine/src/cache.rs", "evict_lru"),
    )
    .expect("write adr");
    fs::create_dir_all(repo.join("docs/specs")).expect("mk specs");
    fs::write(
        repo.join("docs/specs/rate-limiting.md"),
        spec("crates/engine/src/limiter.rs", "covers_burst"),
    )
    .expect("write spec");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    // The project layer — the locate-preamble's `require_project_layer` gate.
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// The doc↔code-clean path: a committed store whose every cited symbol exists → `jigc validate`
/// renders the report with **no `doc-code` content finding**.
///
/// The store is doc↔code clean but *version*-stale: this fixture's docs are hand-committed and
/// unstamped, so since M42 the sweep exits **non-zero** on the version-currency break (the third
/// exit-flipping exception — `design/validation.md` → Exit semantics). Asserted, not glossed:
/// the flip is the corpus's staleness, never a doc↔code verdict. (A genuinely clean — migrated —
/// store exiting 0 is pinned in `managed_vs_foreign.rs`.)
#[test]
fn validate_doc_code_clean_store_surfaces_no_content_finding() {
    let repo = TempDir::new("clean");
    seed_clean_store(repo.path());

    let out = jigc(repo.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stdout.contains("doc-code"),
        "a doc↔code-clean store must surface no doc-code content finding; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("schema-conformance.schema-version-current") && !out.status.success(),
        "the unstamped fixture corpus is unmigrated, so the sweep exits non-zero on the \
         version-currency break — and on nothing else; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// The probe pre-flight (M18 inc-3 / T2): a committed store carrying ≥1 anchor, but
/// `JIGC_DOC_CODE_PROBE` pointed at a path that does not resolve to an executable →
/// `jigc validate` exits **non-zero** with **exactly one** "`doc-code` probe not found"
/// operational error on stderr (the count asserted, so an N-per-anchor crash-meta
/// regression fails), and **no `pack-probe-integrity` meta-finding** in stdout — the
/// sweep never ran (`design/validation.md` → Distribution bound; review S2). A missing
/// probe is a misconfiguration to report once, not N findings.
#[test]
fn validate_probe_absent_reports_one_operational_error_non_zero() {
    let repo = TempDir::new("probe-absent");
    seed_clean_store(repo.path());

    let missing = repo.path().join("nonexistent-doc-code-probe");
    let out = jigc_with_probe(repo.path(), &["validate"], &missing);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        !out.status.success(),
        "a missing probe must exit non-zero; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    let hits = stderr.matches("`doc-code` probe not found").count();
    assert_eq!(
        hits, 1,
        "exactly one `doc-code probe not found` operational error must surface on stderr \
         (an N-per-anchor regression fails this); stderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("pack-probe-integrity"),
        "the sweep must not run when the probe is absent — no pack-probe-integrity meta-finding; \
         stdout:\n{stdout}",
    );
}

/// Compile a tiny **real** probe program that drains its stdin request then exits 2 — a
/// crashing `doc-code` the invoker drives as an actual subprocess (the non-zero-exit
/// failure mode, the `store_sweep_acceptance.rs` `build_crasher` idiom). It is a real
/// file, so it **passes** the T2 pre-flight (the probe *is* resolvable) yet yields a
/// `pack-probe-integrity.crash` meta-finding when run — the injected-failure input to the
/// two-class exit rule. Named `doc-code` so a `JIGC_DOC_CODE_PROBE` pointed at it
/// resolves.
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
    let out = Command::new("rustc")
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

/// The milestone **headline acceptance flow** (M18 inc-3 / T3): seed a committed store
/// (an `adr` + a `spec` with valid `code-anchor`s over real `.rs` files), then **rename a
/// cited symbol** in the working tree — the over-time drift an unrelated `task validate` /
/// `finalize` would *not* catch. `jigc validate` surfaces the now-stale anchor as a
/// `doc-code.*` content finding keyed on that anchor's address, and — because this is
/// content-only, it contributes **no exit flip of its own** (detect-and-report, the `jigc ingest`
/// precedent; `design/validation.md` → Exit semantics): the exit rule keys on the probe id /
/// check id directly, never on `has_blocking()`. *(This fixture's docs are unstamped, so the
/// sweep nevertheless exits non-zero on the M42 version-currency break — the corpus is
/// unmigrated. The doc↔code finding's exit-0-ness is pinned over a migrated corpus in
/// `managed_vs_foreign.rs`; asserted here so the two causes are never conflated.)*
#[test]
fn validate_stale_anchor_surfaces_finding_without_flipping_the_exit_itself() {
    let repo = TempDir::new("stale");
    seed_clean_store(repo.path());

    // Rename the symbol the committed `adr` cites: `evict_lru` → `evicted`. The doc still
    // cites `evict_lru`, which no longer exists in the working tree → a stale anchor that
    // only a store-scope sweep catches.
    fs::write(
        repo.path().join("crates/engine/src/cache.rs"),
        "pub fn evicted() {}\nfn helper() {}\n",
    )
    .expect("rename the cited symbol");

    let out = jigc(repo.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stdout.contains("schema-conformance.schema-version-current") && !out.status.success(),
        "the unstamped fixture corpus is unmigrated, so the exit flips on the version-currency \
         break — the stale anchor itself contributes no flip; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("doc-code.symbol-exists")
            && stdout.contains("crates/engine/src/cache.rs#evict_lru"),
        "the renamed cited symbol must surface a doc-code.symbol-exists finding naming the \
         now-stale anchor; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("pack-probe-integrity"),
        "a healthy probe yields no pack-probe-integrity meta-finding; stdout:\n{stdout}",
    );
    // The trailer must match the exit the tool actually takes. Over this (unmigrated) corpus that
    // is the M42 version-currency case: it says the sweep exits non-zero and names the verb that
    // clears it — never the report-only sentence, which would print `exit 0` while exiting 1.
    // (The report-only trailer's own case is pinned over a migrated corpus in
    // `managed_vs_foreign.rs`.)
    assert!(
        stdout.contains("jigc migrate-corpus") && !stdout.contains("report-only at store scope"),
        "an unmigrated corpus must carry the version-currency trailer, not the report-only one; \
         stdout:\n{stdout}",
    );

    // The `--format json` surface carries the same signal, from the same predicate.
    let out = jigc(repo.path(), &["validate", "--format", "json"]);
    let value: serde_json::Value = stdout_json(&out, &[1], "`jigc validate --format json`");
    assert_eq!(value["scope"], "store");
    assert_eq!(
        value["report_only"],
        serde_json::Value::Bool(false),
        "an unmigrated corpus is not report-only — the exit flips; json:\n{value}",
    );
}

/// The injected-probe-failure half of the two-class exit rule (M18 inc-3 / T3): the probe
/// is **present** (it passes the T2 pre-flight) but crashes (exits non-zero) over every
/// anchor → a `pack-probe-integrity.*` meta-finding is present → `jigc validate` exits
/// **non-zero** (the sweep can't claim a trustworthy result — the probe didn't run). Never
/// a falsely-green run. This is *not* a transaction gate: it is the command honestly
/// reporting it could not complete (`design/validation.md` → Severity → review B1).
#[test]
fn validate_injected_probe_failure_exits_non_zero() {
    let repo = TempDir::new("crash");
    seed_clean_store(repo.path());

    let probe_dir = TempDir::new("crasher");
    let crasher = build_crasher(probe_dir.path());
    let out = jigc_with_probe(repo.path(), &["validate"], &crasher);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        !out.status.success(),
        "an injected probe failure (a present-but-crashing probe) must exit non-zero — the \
         sweep could not complete; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("pack-probe-integrity"),
        "a crashing probe must surface a pack-probe-integrity meta-finding in the rendered \
         report; stdout:\n{stdout}",
    );
    // The probe-integrity path must stay clearly distinguished from a report-only content
    // finding: it does not claim report-only, it says the sweep could not complete.
    assert!(
        stdout.contains("the sweep could not complete and exits"),
        "the probe-integrity (exit-non-zero) path must be distinguished from a report-only \
         content finding; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("report-only at store scope"),
        "a probe-integrity run is not report-only — it must not claim so; stdout:\n{stdout}",
    );

    // The `--format json` surface marks the run not-report-only.
    let out = jigc_with_probe(repo.path(), &["validate", "--format", "json"], &crasher);
    let value: serde_json::Value = stdout_json(&out, &[1], "`jigc validate --format json`");
    assert_eq!(
        value["report_only"],
        serde_json::Value::Bool(false),
        "a probe-integrity run is not report-only; json:\n{value}",
    );
}

/// A probe that **cannot start** says so (M54 Inc 2 T2; DECISIONS.md → *M54 settled*, S1).
/// `JIGC_DOC_CODE_PROBE` points at an **existing, non-executable regular file**: it passes
/// the `is_file` pre-flight, so the sweep runs, and the spawn fails with `EACCES`. The
/// report carries exactly one `pack-probe-integrity.probe-failure` finding, check `crash`
/// (no new check id), whose message says *could not start* and carries the io error. Red
/// before: the spawn failure was mapped to a signal-shaped exit and read *exited non-zero
/// (exit-code signal)* — the message the gate's `ENOENT` race was misread through.
#[test]
fn validate_unstartable_probe_says_could_not_start() {
    let repo = TempDir::new("could-not-start");
    seed_clean_store(repo.path());

    let probe_dir = TempDir::new("not-executable");
    let not_executable = probe_dir.path().join("doc-code");
    fs::write(&not_executable, "not a program\n").expect("write the non-executable probe");
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&not_executable, fs::Permissions::from_mode(0o644))
            .expect("clear the exec bits");
    }

    let out = jigc_with_probe(
        repo.path(),
        &["validate", "--format", "json"],
        &not_executable,
    );
    let value: serde_json::Value = stdout_json(&out, &[1], "`jigc validate --format json`");
    let failures: Vec<&serde_json::Value> = value["findings"]
        .as_array()
        .expect("a findings array")
        .iter()
        .filter(|f| f["probe"] == "pack-probe-integrity")
        .collect();

    assert_eq!(
        failures.len(),
        1,
        "exactly one pack-probe-integrity finding for a probe that cannot start; json:\n{value}",
    );
    let failure = failures[0];
    assert_eq!(failure["code"], "pack-probe-integrity.probe-failure");
    assert_eq!(failure["check"], "crash", "no new check id; json:\n{value}");
    let message = failure["message"].as_str().expect("a message");
    assert!(
        message.contains("could not start") && message.contains("os error"),
        "the message says the probe could not start and carries the io error: {message}",
    );
    // The program that could not start is named (the M54 audit finding that completes
    // S4): a bare `Permission denied` does not say which file to repair.
    let named = format!("could not start `{}`: ", not_executable.display());
    assert!(
        message.contains(&named),
        "the message names the program that could not start (`{named}`): {message}",
    );
    assert!(
        !message.contains("exited non-zero"),
        "a probe that never ran did not exit: {message}",
    );
}

/// A **current (v2-stamped), conformant** `adr` whose `supersedes` names an ADR that is not in
/// the store — a **dangling committed forward edge**, the store sweep's fourth family. Stamped
/// at the `adr` manifest's current version so the corpus is *migrated* (no version-currency
/// break, so the report-only trailer branch is the one under test).
const ADR_DANGLING_REF: &str = "\
---
status: accepted
date: 2026-06-25
supersedes: adr:no-such-decision
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

/// A **current (v2-stamped) but non-conformant** `adr`: `## Consequences` removed, so it fails to
/// parse under the current shape and the store family's parse-failure arm raises `conformance.*`
/// **directly over the committed doc**. Left **un-baselined** (no `FileStateRecord` entry — the
/// fresh-clone / brownfield state): at task scope the reconciler grades exactly this doc
/// *advisory*, so the break gates **nowhere**.
const ADR_UNBASELINED_NONCONFORMANT: &str = "\
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

/// Seed a real git repo with the `.jigc/config/` project layer and one committed `adr` body at
/// the doctype's canonical home — no code anchors, so the store sweep's verdict is the doc's
/// own, not the probe's.
fn seed_adr_store(repo: &Path, body: &str) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    fs::create_dir_all(repo.join("docs/decisions")).expect("mk decisions");
    fs::write(repo.join(ADR_REL), body).expect("write adr");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// The seeded ADR's repo-relative home (`id-from: title` over the fixture's H1).
const ADR_REL: &str = "docs/decisions/cache-sessions-in-memory.md";

/// Baseline the file-state record for `rel` at its current raw-byte hash — the committed-store
/// baseline that makes the reconciler grade a drift on this doc **blocking** at task scope (the
/// `flow37_rename.rs` idiom).
fn baseline_file_state(repo: &Path, rel: &str) {
    use engine::file_state::{FileStateRecord, hash_bytes};
    let mut record = FileStateRecord::new();
    let bytes = fs::read(repo.join(rel)).expect("read the managed doc to baseline");
    record.record(rel, hash_bytes(&bytes));
    record
        .save(&repo.join(".jigc"))
        .expect("save the file-state baseline");
}

/// **A store-scope finding says where it gates** (M42 Inc 12 / T5; `design/validation.md` → The
/// trailer must not claim a gate that does not exist). The store sweep prints each finding at its
/// **cascade** severity — `blocking · …` — while exiting 0, so the severity token alone cannot
/// tell a reader whether anything will ever *stop* on it. The per-finding label carries the
/// trailer's own (Inc 4) gate criterion onto the row: a finding that really does gate at the task
/// boundary renders `blocking (gates at finalize) · <code> — …`.
///
/// The gating arm: a **baselined** committed ADR whose `supersedes` dangles. At task scope this
/// doc's drift reaches `reconcile_committed_store`'s **blocking** arm, so the gate is real and the
/// label is earned.
#[test]
fn validate_labels_a_gating_store_finding_with_the_gate_it_carries() {
    let repo = TempDir::new("gates-at-finalize");
    seed_adr_store(repo.path(), ADR_DANGLING_REF);
    baseline_file_state(repo.path(), ADR_REL);

    let out = jigc(repo.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stdout.contains("blocking (gates at finalize) · schema-conformance.ref-resolves"),
        "a dangling ref over a baselined committed doc DOES gate at the task boundary — the row \
         must say so; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    // The store sweep is still report-only for content: the label names the gate, it is not one.
    assert!(
        out.status.success(),
        "a content-only finding over a migrated corpus stays exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains(
            "these gate at `jigc task validate` / `jigc task finalize` / `jigc milestone finalize`."
        ),
        "the trailer's claim and the row's label are one criterion — and the door list is \
         complete: the milestone-boundary gate (M47 inc-8 / T4) drives the same shared \
         `validate_task` entry and blocks exit 3 on this same finding; stdout:\n{stdout}",
    );

    // The **JSON severity token is unchanged** — the label is a text-surface affordance, never a
    // break of the machine contract (`command-output-contract.md`).
    let out = jigc(repo.path(), &["validate", "--format", "json"]);
    let value: serde_json::Value = stdout_json(&out, &[0], "`jigc validate --format json`");
    let findings = value["findings"].as_array().expect("a findings array");
    let ref_finding = findings
        .iter()
        .find(|f| f["code"] == "schema-conformance.ref-resolves")
        .expect("the dangling-ref finding rides the JSON report");
    assert_eq!(
        ref_finding["severity"], "blocking",
        "the JSON severity token stays the bare cascade severity; json:\n{value}",
    );
}

/// The **non**-gating arm of the same label (M42 Inc 12 / T5): an **un-baselined** non-conformant
/// committed ADR. The store sweep raises the parse-failure `conformance.*` break directly over the
/// committed doc, but at task scope the reconciler grades an un-baselined nonconformant file
/// *advisory* — nothing gates on it, ever — so the row must print the **bare** `blocking` token and
/// the trailer must claim no gate. (This is the dominant `jigc validate` corpus: `.jigc/state/` is
/// gitignored, so every committed doc in a fresh clone is un-baselined.)
#[test]
fn validate_leaves_a_gateless_store_finding_unlabelled() {
    let repo = TempDir::new("gates-nowhere");
    seed_adr_store(repo.path(), ADR_UNBASELINED_NONCONFORMANT);

    let out = jigc(repo.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stdout.contains("blocking · conformance."),
        "the parse-failure break renders at its bare cascade severity; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("(gates at finalize)"),
        "no gate exists for a break over an un-baselined committed doc — the row must not claim \
         one; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("jigc task validate")
            && !stdout.contains("jigc task finalize")
            && !stdout.contains("jigc milestone finalize"),
        "and the trailer must claim none either; stdout:\n{stdout}",
    );

    // The JSON severity token is unchanged here too — the two surfaces do not diverge.
    let out = jigc(repo.path(), &["validate", "--format", "json"]);
    let value: serde_json::Value = stdout_json(&out, &[0], "`jigc validate --format json`");
    let findings = value["findings"].as_array().expect("a findings array");
    let break_finding = findings
        .iter()
        .find(|f| {
            f["code"]
                .as_str()
                .is_some_and(|c| c.starts_with("conformance."))
        })
        .expect("the parse-failure break rides the JSON report");
    assert_eq!(
        break_finding["severity"], "blocking",
        "the JSON severity token stays the bare cascade severity; json:\n{value}",
    );
}
