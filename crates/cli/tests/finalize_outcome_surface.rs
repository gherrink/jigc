//! M17 increment 1 — the landed `task finalize` outcome surface.
//!
//! A landed finalize must emit the findings envelope on **stdout**, symmetric with
//! `task validate` and advisories included (`design/measurement.md` → The capture
//! substrate, item 2): absorb evidence observed during the preflight sweep was
//! previously swallowed, making adapter-adherence's absorb term structurally
//! invisible to a harness-side tally. Two landed shapes, both formats:
//!
//! - **Absorb surfaced:** task A promotes an ADR; a conformant OOB edit (outside the
//!   CLI) hits the committed ADR; task B's finalize lands (exit 0) AND its stdout
//!   carries the `reconciliation.absorb` advisory.
//! - **Clean positive signal:** a landed finalize whose preflight report is empty
//!   emits the positive no-findings line (the `validate` renderer's clean signal),
//!   never silence.
//!
//! Both repeated under `--format json`: stdout parses as the report envelope.
//!
//! And the exit-code split (T2): a validation-blocked `task validate` / `task
//! finalize` exits **3** (`EXIT_VALIDATION_BLOCKED` — drift-caught and the
//! `validate-blocks` paired count key on it), while an operational error stays **1**.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! dev's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-finsurface-{tag}-{}-{:?}",
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

/// Fill every author-required field/slot of the provisioned commit doc so a finalize
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

/// Run `jigc task finalize <task>` (plus `--format <f>` when given), returning the
/// raw output for the caller's stdout assertions.
fn finalize(repo: &Path, home: &Path, task: &str, format: Option<&str>) -> std::process::Output {
    let mut args = vec!["task", "finalize", task];
    if let Some(f) = format {
        args.extend(["--format", f]);
    }
    jigc(repo, home, &args)
}

/// Task A — create + finalize `adr:single-node-cache`, the committed managed doc the
/// OOB edit will hit. Its landed finalize posts the committed doc's file-state
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
    assert_ok(&out, "`jigc start` (task A)");
    let task = "cache-sessions-in-a-single-in-memory-node";

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (task A)");

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

    let out = finalize(repo, home, task, None);
    assert_ok(&out, "`jigc task finalize` (task A)");
    assert!(
        repo.join("decisions").join("single-node-cache.md").exists(),
        "task A must promote decisions/single-node-cache.md",
    );
}

/// A **conformant** OOB edit on the committed ADR, made outside the CLI (plain
/// `fs::write` — the human-in-git channel): prose-only, so the reconcile classifier
/// re-parses clean and routes to absorb, never to a conformance block.
fn oob_edit_committed_adr(repo: &Path) {
    let path = repo.join("decisions").join("single-node-cache.md");
    let body = fs::read_to_string(&path).expect("read the committed ADR");
    let edited = body.replacen(
        "A cold node loses its sessions.",
        "A cold node loses its sessions; clients re-authenticate.",
        1,
    );
    assert_ne!(body, edited, "the OOB edit must change the committed ADR");
    fs::write(&path, edited).expect("apply the OOB edit");
}

/// The absorb scenario: task A promotes the ADR, the OOB edit hits it, then task B
/// (commit-only) finalizes. Asserts the finalize **lands** (exit 0) and returns its
/// stdout — which must carry the `reconciliation.absorb` advisory.
fn landed_finalize_after_oob(format: Option<&str>) -> String {
    let repo = TempDir::new("absorb");
    let home = TempDir::new("home");
    init_repo(repo.path());

    commit_prior_adr(repo.path(), home.path());
    oob_edit_committed_adr(repo.path());

    let task = "tighten-the-cache-docs";
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "tighten the cache docs",
        ],
    );
    assert_ok(&out, "`jigc start` (task B)");
    fill_commit(repo.path(), home.path(), task);

    let out = finalize(repo.path(), home.path(), task, format);
    assert_ok(
        &out,
        "`jigc task finalize` (task B) — the absorb must not block",
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// The clean scenario: a commit-only task whose preflight report is **empty**.
/// `task validate`/finalize never persist the working-area baseline, so the staged
/// commit doc would baseline-adopt fresh on every sweep; seeding the file-state
/// record with the doc's *matching* hash (the severity-tuning seeding pattern, here
/// with the real digest) puts it `IN_SYNC` — zero findings. Asserts the finalize
/// lands and returns its stdout — which must carry the positive no-findings signal.
fn landed_clean_finalize(format: Option<&str>) -> String {
    let repo = TempDir::new("clean");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let task = "add-rate-limiter";
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert_ok(&out, "`jigc start` (clean task)");
    fill_commit(repo.path(), home.path(), task);

    // Seed the staged commit doc's real hash so the sweep sees IN_SYNC, not adopt.
    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("commit:{task}.md"));
    let digest = engine::file_state::hash_bytes(&fs::read(&staged).expect("read staged doc"));
    let state = repo.path().join(".jigc").join("state");
    fs::create_dir_all(&state).expect("create .jigc/state");
    let body = serde_json::json!({ "hashes": { format!("docs/commit:{task}.md"): digest } });
    fs::write(state.join("file-state.json"), body.to_string()).expect("seed file-state.json");

    let out = finalize(repo.path(), home.path(), task, format);
    assert_ok(&out, "`jigc task finalize` (clean task)");
    String::from_utf8(out.stdout).expect("utf-8 stdout")
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

/// Inject a `supersedes: <target>` line into the staged ADR's empty front-matter block
/// (mirrors `superseding_decision.rs` — the concern here is the exit code of the block,
/// not the write-path field generation).
fn inject_supersedes(repo: &Path, task: &str, slug: &str, target: &str) {
    let staged = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"));
    let body = fs::read_to_string(&staged).expect("read staged ADR");
    let with = body.replacen(
        "---\n---\n",
        &format!("---\nsupersedes: {target}\n---\n"),
        1,
    );
    assert_ne!(
        body, with,
        "the staged ADR carries an empty front-matter block"
    );
    fs::write(&staged, &with).expect("inject supersedes ref");
}

/// Stage a task whose ADR carries a **dangling** `supersedes` forward-ref — the
/// validation-blocked shape (`schema-conformance.ref-resolves`, blocking). The commit
/// doc is filled so the ref is the only block.
fn stage_dangling_supersedes(repo: &Path, home: &Path) -> &'static str {
    let task = "supersede-the-cache-decision";
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "supersede the cache decision",
        ],
    );
    assert_ok(&out, "`jigc start` (dangling task)");

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Shared redis cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (dangling task)");

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
        "adr:shared-redis-cache#context",
        b"A single node is a single point of failure.\n",
    );
    set_slot(
        "adr:shared-redis-cache#decision",
        b"Replicate the session cache across nodes.\n",
    );
    set_slot(
        "adr:shared-redis-cache#consequences",
        b"Cache reads cross the network.\n",
    );
    // The dangling forward-ref: a target in NEITHER surface.
    inject_supersedes(repo, task, "shared-redis-cache", "adr:typo-nonexistent");
    fill_commit(repo, home, task);
    task
}

/// M17 increment 1, T2 — the exit-code split (`design/measurement.md` → The capture
/// substrate: exit 3 = validation-blocked, the code drift-caught and the
/// `validate-blocks` paired count key on; 1 stays operational error; 2 stays clap
/// usage). Blocked finalize and blocked validate must exit **3**; an operational
/// error must stay **1**.
#[test]
fn validation_blocked_exits_3() {
    let repo = TempDir::new("blocked");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let task = stage_dangling_supersedes(repo.path(), home.path());

    // ── a blocked `task validate` exits 3 (the validate-blocks paired count) ─────
    let out = jigc(repo.path(), home.path(), &["task", "validate", task]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a blocking `task validate` must exit 3; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // ── a blocked `task finalize` exits 3 with the envelope on stderr ────────────
    let out = finalize(repo.path(), home.path(), task, None);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a validation-blocked finalize must exit 3; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("blocking") && stderr.contains("adr:typo-nonexistent"),
        "the blocked finalize carries the findings envelope on stderr, naming the \
         dangling target; got:\n{stderr}",
    );

    // ── an operational error stays exit 1 ────────────────────────────────────────
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "nonexistent"],
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "an operational error must stay exit 1; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

#[test]
fn landed_finalize_emits_findings() {
    // ── agent format: the absorb advisory is on stdout, not swallowed ────────────
    let stdout = landed_finalize_after_oob(None);
    assert!(
        stdout.contains("advisory · reconciliation.absorb"),
        "a landed finalize must surface the absorb advisory on stdout; got:\n{stdout}",
    );

    // ── agent format: a clean landed finalize emits the positive signal ──────────
    let stdout = landed_clean_finalize(None);
    assert!(
        stdout.contains("no findings — the task validates clean"),
        "a clean landed finalize must emit the positive no-findings line; got:\n{stdout}",
    );

    // ── json format: stdout parses as the report envelope, absorb included ───────
    let stdout = landed_finalize_after_oob(Some("json"));
    let findings = parse_envelope(&stdout, "absorbed landed finalize (json)");
    assert!(
        findings
            .iter()
            .any(|f| f["code"] == "reconciliation.absorb"),
        "the json envelope must carry the reconciliation.absorb finding; got:\n{stdout}",
    );

    // ── json format: the clean envelope is the empty report, never silence ───────
    let stdout = landed_clean_finalize(Some("json"));
    let findings = parse_envelope(&stdout, "clean landed finalize (json)");
    assert!(
        findings.is_empty(),
        "the clean json envelope must carry an empty findings array; got:\n{stdout}",
    );
}
