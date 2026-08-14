//! Acceptance — **one foreign file, one code, one route at both doors** (M48 Increment 4,
//! T1; `design/validation.md` → The managed-vs-foreign discriminator).
//!
//! M42 shipped the managed-vs-foreign discriminator and swept it through the **store**
//! family: a committed file squatting at a managed home that jigc never wrote draws the
//! adoption advisory `schema-conformance.unadopted-instance`, routed at `jigc ingest` /
//! `jigc migrate <path> --as <doctype>`. That sweep **never reached the task-scope
//! reconciler**, which adjudicates the very same committed file through
//! `engine::file_state::reconcile_committed`'s `UNKNOWN` + non-conformant arm — and graded
//! **every** such file `reconciliation.conformance-block`, foreign and managed alike.
//!
//! So one file answered **two codes** depending on which door asked, and the task-scope
//! door's advisory carried a route (*"ingest, migrate, or move it out of the managed
//! location"*) that is prose about a class, not about this file — while the store door
//! already named the two concrete verbs with the path substituted in.
//!
//! This suite drives the **real binary** over a repo carrying **both** cells at once, so the
//! split is proven per *file*, not per *report*:
//!
//! - **the foreign cell** — a freeform `docs/decisions/notes.md`, committed in git outside
//!   jigc — draws the **same `(code, target)` pair** and a **byte-identical route** at all
//!   three doors (`jigc validate` store scope, `jigc task validate`, the `task finalize`
//!   preflight), draws **zero** `reconciliation.conformance-block`, is **not** baseline-
//!   adopted into `.jigc/state/file-state.json`, **re-fires** on a second sweep (the
//!   routed-but-not-recorded recurrence), and leaves the task doors at **exit 0**;
//! - **the managed cell** — a v2-stamped ADR with `## Consequences` removed — keeps its
//!   `reconciliation.conformance-block` and draws **no** `unadopted-instance`: the
//!   discriminator suppresses nothing it cannot prove foreign.
//!
//! The route is **lifted from the store report and compared verbatim** — the emitted bytes
//! are the contract, never a route rebuilt in test code.

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
            "jigc-foreign-both-doors-{tag}-{}-{:?}",
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

/// Run `jigc doc <args>`, piping `stdin`.
fn jigc_doc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> std::process::Output {
    use std::io::Write;
    use std::process::Stdio;
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .arg("doc")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
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

/// Make `root` a real git repo, then run `jigc setup` over it.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    assert_ok(&jigc(repo, home, &["setup"]), "`jigc setup`");
}

/// The **foreign cell**: freeform scratch prose committed into the `adr` location dir
/// outside jigc. It carries no schema-version stamp and parses against no shipped `adr`
/// schema version — never adopted, by its own committed bytes.
const NOTES: &str = "docs/decisions/notes.md";
const NOTES_BODY: &str = "# scratch notes\n\nrandom thoughts, not an ADR\n";

/// The **managed cell**: a v2-**stamped** ADR (so the discriminator's first arm reads it
/// managed outright) whose `## Consequences` is missing, so it fails to parse under the
/// current shape. Its break is a real break on jigc's own doc — never an adoption case.
const MANAGED_ADR: &str = "docs/decisions/cache-sessions-in-memory.md";
const MANAGED_ADR_BODY: &str = "\
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

/// The findings array of a `--format json` envelope.
fn envelope_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8(out.stdout.clone()).expect("utf-8 stdout");
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("{what} envelope must parse ({e}); got:\n{stdout}"));
    value["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("{what}: the envelope carries a `findings` array; got:\n{stdout}")
        })
        .clone()
}

/// Every finding whose stable key is exactly `(code, target)` — the discriminating key the
/// contract pins (`design/command-output-contract.md`), read from the emitted `key` object
/// rather than re-derived from the message.
fn by_key<'a>(
    findings: &'a [serde_json::Value],
    code: &str,
    target: &str,
) -> Vec<&'a serde_json::Value> {
    findings
        .iter()
        .filter(|f| {
            f["key"]["code"].as_str() == Some(code) && f["key"]["target"].as_str() == Some(target)
        })
        .collect()
}

/// The one finding at `(code, target)`, or a panic naming the whole report.
fn one_at<'a>(
    findings: &'a [serde_json::Value],
    code: &str,
    target: &str,
    door: &str,
) -> &'a serde_json::Value {
    let hits = by_key(findings, code, target);
    assert_eq!(
        hits.len(),
        1,
        "{door}: exactly one `{code}` at `{target}`; got:\n{findings:#?}",
    );
    hits[0]
}

/// The route string a finding carries, verbatim.
fn route_of(finding: &serde_json::Value, door: &str) -> String {
    finding["route"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("{door}: the adoption advisory carries a route; got:\n{finding:#?}")
        })
        .to_string()
}

/// Start a commit-only `single-task` for `intent` (its task id is the slugified intent),
/// stage one code file and fill the commit doc, leaving the caller to drive the doors.
fn stage_commit_only(repo: &Path, home: &Path, task: &str, intent: &str) {
    assert_ok(
        &jigc(repo, home, &["start", "--workflow", "single-task", intent]),
        &format!("`jigc start` ({task})"),
    );
    fs::write(repo.join(format!("{task}.txt")), "the code change\n").expect("write code change");
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

const UNADOPTED: &str = "schema-conformance.unadopted-instance";
const CONFORMANCE_BLOCK: &str = "reconciliation.conformance-block";

/// One foreign file answers **one code and one route** at the store door, the `task
/// validate` door and the `task finalize` preflight — while the managed cell in the very
/// same report keeps its conformance block.
#[test]
fn a_foreign_file_answers_one_code_and_one_route_at_every_door() {
    let repo = TempDir::new("converge");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // Both cells, committed in git outside jigc — the human-in-git channel.
    fs::create_dir_all(repo.path().join("docs").join("decisions")).expect("mk docs/decisions/");
    fs::write(repo.path().join(NOTES), NOTES_BODY).expect("write the foreign notes");
    fs::write(repo.path().join(MANAGED_ADR), MANAGED_ADR_BODY).expect("write the managed adr");
    git(repo.path(), &["add", "docs"]);
    git(repo.path(), &["commit", "-q", "-m", "seed both cells"]);

    // ── the store door: lift the emitted route verbatim ──────────────────────────────
    let store = jigc(repo.path(), home.path(), &["validate", "--format", "json"]);
    let store_findings = envelope_findings(&store, "jigc validate");
    let store_route = route_of(one_at(&store_findings, UNADOPTED, NOTES, "store"), "store");
    assert!(
        store_route.contains("jigc ingest"),
        "the adoption route names the always-applicable front door; got:\n{store_route}",
    );
    assert!(
        store_route.contains(&format!("jigc migrate {NOTES} --as adr")),
        "and — since `migrate-adr` ships — the doctype-directed verb with the path \
         substituted in (the M40 two-verb tier); got:\n{store_route}",
    );

    // ── the `task validate` door: the same pair, the same bytes ──────────────────────
    let task_a = "first-pass";
    stage_commit_only(repo.path(), home.path(), task_a, task_a);
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task_a, "--format", "json"],
    );
    assert!(
        out.status.success(),
        "an advisory-only sweep does not block `task validate`; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let task_findings = envelope_findings(&out, "task validate");
    assert_eq!(
        route_of(
            one_at(&task_findings, UNADOPTED, NOTES, "task validate"),
            "task validate"
        ),
        store_route,
        "the task door serves the store door's route byte for byte; got:\n{task_findings:#?}",
    );
    assert!(
        by_key(&task_findings, CONFORMANCE_BLOCK, NOTES).is_empty(),
        "and the foreign file draws no conformance block at all — one file, one code; \
         got:\n{task_findings:#?}",
    );

    // The managed cell in the SAME report keeps its block and is never called unadopted.
    let managed = one_at(
        &task_findings,
        CONFORMANCE_BLOCK,
        MANAGED_ADR,
        "task validate",
    );
    assert_eq!(
        managed["severity"].as_str(),
        Some("advisory"),
        "the un-baselined managed doc stays advisory (routed, not recorded); got:\n{managed:#?}",
    );
    assert!(
        by_key(&task_findings, UNADOPTED, MANAGED_ADR).is_empty(),
        "a STAMPED managed doc is never adjudicated foreign; got:\n{task_findings:#?}",
    );

    // ── the `task finalize` preflight: the same pair, the same bytes, and it lands ────
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task_a, "--format", "json"],
    );
    assert!(
        out.status.success(),
        "an advisory-only sweep lands at the finalize boundary; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let finalize_findings = envelope_findings(&out, "task finalize");
    assert_eq!(
        route_of(
            one_at(&finalize_findings, UNADOPTED, NOTES, "task finalize"),
            "task finalize"
        ),
        store_route,
        "the finalize preflight serves the same route; got:\n{finalize_findings:#?}",
    );
    assert!(
        by_key(&finalize_findings, CONFORMANCE_BLOCK, NOTES).is_empty(),
        "no conformance block at the committing door either; got:\n{finalize_findings:#?}",
    );

    // ── routed, NOT recorded ─────────────────────────────────────────────────────────
    let record = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("state")
            .join("file-state.json"),
    )
    .expect("the landed finalize persists the file-state record");
    let value: serde_json::Value = serde_json::from_str(&record).expect("file-state.json parses");
    let hashes = value["hashes"]
        .as_object()
        .expect("the record carries a `hashes` map");
    assert!(
        !hashes.contains_key(NOTES),
        "the foreign file must not be baseline-adopted into the record; got:\n{record}",
    );

    // ── and it re-fires on the next sweep (routed-but-not-recorded recurrence) ────────
    let task_b = "second-pass";
    stage_commit_only(repo.path(), home.path(), task_b, task_b);
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task_b, "--format", "json"],
    );
    assert_ok(&out, "`jigc task finalize` (second pass)");
    let again = envelope_findings(&out, "task finalize (second pass)");
    assert_eq!(
        route_of(
            one_at(&again, UNADOPTED, NOTES, "second sweep"),
            "second sweep"
        ),
        store_route,
        "the advisory re-fires unchanged, never silently absorbed; got:\n{again:#?}",
    );
}
