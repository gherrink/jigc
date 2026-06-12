//! M17 increment 2, T1 — file-state soundness: the absorb baseline-advance is
//! persisted by a landed `task finalize` only (`design/reconciliation.md` →
//! Persistence of the shifted baseline, the M17 amendment; `design/measurement.md`
//! → The capture substrate).
//!
//! Two contracts:
//!
//! - **One edit, one absorb.** A conformant OOB edit on a committed ADR (the
//!   human-in-git channel: edit + `git commit` outside the CLI) fires its
//!   `reconciliation.absorb` advisory **exactly once** across subsequent tasks —
//!   task A's landed finalize emits it in its envelope and persists the shifted
//!   baseline; task B's validate + landed finalize emit **zero** absorb for that
//!   path. The persisted record carries committed-store keys only (no `docs/`-
//!   prefixed staged working-area baselines — those are per-run by design,
//!   `crates/cli/src/task.rs` → `TaskArea::validate`).
//! - **Pure readers stay pure.** A standalone `jigc task validate` absorbs in
//!   memory for its own run (the advisory still surfaces) but leaves
//!   `.jigc/state/file-state.json` byte-unchanged — and never mints it when
//!   absent. No write side effects on a read verb.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! temp repo is a real `git init`, and a self-cleaning `TempDir` keeps the test
//! off the dev's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The committed ADR the OOB edit hits, at its canonical path / record key.
const ADR_PATH: &str = "decisions/single-node-cache.md";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-fssound-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        path.push(unique);
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
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

/// Fill every author-required field/slot of the provisioned commit doc so a sweep
/// over it validates clean of `schema-conformance` findings.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
}

/// Run `jigc task finalize <task> --format json`, returning the raw output.
fn finalize_json(repo: &Path, home: &Path, task: &str) -> std::process::Output {
    jigc(repo, home, &["task", "finalize", task, "--format", "json"])
}

/// Task 0 — create + finalize `adr:single-node-cache`, the committed managed doc
/// the OOB edit will hit. Its landed finalize posts the committed doc's file-state
/// baseline (finalize phase 7), the precondition for the drift→absorb classification.
fn commit_prior_adr(repo: &Path, home: &Path) {
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start` (task 0)");
    let task = "cache-sessions-in-a-single-in-memory-node";

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (task 0)");

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot(
        "adr:single-node-cache#context",
        b"Session lookups must stay sub-millisecond.\n",
    );
    set_slot(
        "adr:single-node-cache#decision",
        b"A single in-memory node keeps lookups fast.\n",
    );
    set_slot(
        "adr:single-node-cache#consequences",
        b"A cold node loses its sessions.\n",
    );
    fill_commit(repo, home, task);

    let out = finalize_json(repo, home, task);
    assert_ok(&out, "`jigc task finalize` (task 0)");
    assert!(
        repo.join(ADR_PATH).exists(),
        "task 0 must promote {ADR_PATH}",
    );
}

/// A **conformant** OOB edit on the committed ADR, made outside the CLI (plain
/// `fs::write` — the human-in-git channel): prose-only, so the reconcile classifier
/// re-parses clean and routes to absorb, never to a conformance block.
fn oob_edit_committed_adr(repo: &Path) {
    let path = repo.join(ADR_PATH);
    let body = fs::read_to_string(&path).expect("read the committed ADR");
    let edited = body.replacen(
        "A cold node loses its sessions.",
        "A cold node loses its sessions; clients re-authenticate.",
        1,
    );
    assert_ne!(body, edited, "the OOB edit must change the committed ADR");
    fs::write(&path, edited).expect("apply the OOB edit");
}

/// The findings array of a parsed JSON report envelope.
fn parse_envelope(stdout: &str, what: &str) -> Vec<serde_json::Value> {
    let value: serde_json::Value = serde_json::from_str(stdout).unwrap_or_else(|err| {
        panic!("{what}: stdout must parse as the report envelope ({err}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("{what}: the envelope must carry a `findings` array; got:\n{stdout}")
        })
        .clone()
}

/// Count the `reconciliation.absorb` findings naming the committed ADR's path.
fn absorb_count(findings: &[serde_json::Value]) -> usize {
    findings
        .iter()
        .filter(|f| {
            f["code"] == "reconciliation.absorb"
                && f["message"].as_str().is_some_and(|m| m.contains(ADR_PATH))
        })
        .count()
}

/// Mint a commit-only task with one code file (so its commit is never empty) and a
/// filled commit doc, returning nothing — the caller drives validate/finalize.
fn stage_commit_only(repo: &Path, home: &Path, task: &str, intent: &str) {
    let out = jigc(repo, home, &["start", "--workflow", "single-task", intent]);
    assert_ok(&out, &format!("`jigc start` ({task})"));
    fs::write(repo.join(format!("{task}.txt")), "the code change\n").expect("write code change");
    fill_commit(repo, home, task);
}

/// The re-fire defect (`design/reconciliation.md` → Persistence of the shifted
/// baseline): one conformant OOB edit — made **and committed** outside the CLI, so
/// no later task-commit re-hashes its path — fires `reconciliation.absorb` exactly
/// once across subsequent tasks. Task A's landed finalize emits it once in its
/// envelope and persists the shifted baseline (committed-store keys only); task B's
/// validate + landed finalize emit zero absorb for that path.
#[test]
fn absorbed_oob_edit_fires_absorb_exactly_once_across_tasks() {
    let repo = TempDir::new("refire");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Task 0 lands the committed ADR (its baseline is posted at finalize phase 7).
    commit_prior_adr(repo.path(), home.path());

    // The human channel: edit the committed ADR and commit it in git directly. The
    // working tree is clean afterward, so no later task's commit touches this path —
    // only the persisted absorb baseline can stop the re-fire.
    oob_edit_committed_adr(repo.path());
    git(repo.path(), &["add", ADR_PATH]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "docs: tighten the consequences"],
    );

    // ── task A: the landed finalize emits the absorb exactly once ────────────────
    let task_a = "absorb-the-external-edit";
    stage_commit_only(repo.path(), home.path(), task_a, "absorb the external edit");
    let out = finalize_json(repo.path(), home.path(), task_a);
    assert_ok(
        &out,
        "`jigc task finalize` (task A) — the absorb must not block",
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let findings = parse_envelope(&stdout, "task A landed finalize");
    assert_eq!(
        absorb_count(&findings),
        1,
        "task A's landed envelope carries the absorb exactly once; got:\n{stdout}",
    );

    // The durable record advanced to the edited bytes' hash, committed-store keys
    // only — no `docs/`-prefixed staged working-area baselines leak into it.
    let record = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("state")
            .join("file-state.json"),
    )
    .expect("a landed finalize persists the file-state record");
    let value: serde_json::Value = serde_json::from_str(&record).expect("file-state.json parses");
    let hashes = value["hashes"]
        .as_object()
        .expect("the record carries a `hashes` map");
    let on_disk = fs::read(repo.path().join(ADR_PATH)).expect("read the absorbed ADR");
    assert_eq!(
        hashes[ADR_PATH],
        serde_json::Value::String(engine::file_state::hash_bytes(&on_disk)),
        "the landed finalize persists the absorbed baseline; got:\n{record}",
    );
    assert!(
        hashes.keys().all(|k| !k.starts_with("docs/")),
        "the persisted record carries no `docs/`-prefixed staged keys; got:\n{record}",
    );

    // ── task B: validate + landed finalize emit zero absorb for that path ────────
    let task_b = "tighten-the-cache-docs";
    stage_commit_only(repo.path(), home.path(), task_b, "tighten the cache docs");

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task_b, "--format", "json"],
    );
    assert_ok(&out, "`jigc task validate` (task B)");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let findings = parse_envelope(&stdout, "task B validate");
    assert_eq!(
        absorb_count(&findings),
        0,
        "task B's validate must not re-fire the already-persisted absorb; got:\n{stdout}",
    );

    let out = finalize_json(repo.path(), home.path(), task_b);
    assert_ok(&out, "`jigc task finalize` (task B)");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let findings = parse_envelope(&stdout, "task B landed finalize");
    assert_eq!(
        absorb_count(&findings),
        0,
        "task B's landed finalize must not re-fire the absorb; got:\n{stdout}",
    );
}

/// The pure-reader guard (`design/reconciliation.md` → Persistence of the shifted
/// baseline: "`validate`, `start`, and read-path sweeps stay pure readers"). A
/// standalone `task validate` absorbs in memory for its own run — the advisory still
/// surfaces — but leaves `.jigc/state/file-state.json` byte-unchanged; and on a
/// fresh repo with no record at all, it never mints one.
#[test]
fn validate_does_not_persist_file_state() {
    // ── byte-unchanged: a sweep that genuinely absorbs persists nothing ──────────
    let repo = TempDir::new("pure");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());
    oob_edit_committed_adr(repo.path());

    let task = "preview-the-drift";
    stage_commit_only(repo.path(), home.path(), task, "preview the drift");

    let record_path = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    let before = fs::read(&record_path).expect("task 0's landed finalize wrote the record");

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task, "--format", "json"],
    );
    assert_ok(&out, "`jigc task validate` (drifted repo)");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let findings = parse_envelope(&stdout, "standalone validate");
    assert_eq!(
        absorb_count(&findings),
        1,
        "the sweep absorbs in memory for its own run (the advisory surfaces); got:\n{stdout}",
    );
    let after = fs::read(&record_path).expect("the record still exists");
    assert_eq!(
        before, after,
        "a standalone `task validate` leaves file-state.json byte-unchanged",
    );

    // ── absent stays absent: no record is minted on a read verb ──────────────────
    let repo = TempDir::new("pure-fresh");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let task = "first-ever-task";
    stage_commit_only(repo.path(), home.path(), task, "first ever task");
    let out = jigc(repo.path(), home.path(), &["task", "validate", task]);
    assert_ok(&out, "`jigc task validate` (fresh repo)");
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("state")
            .join("file-state.json")
            .exists(),
        "a standalone `task validate` never mints the file-state record",
    );
}
