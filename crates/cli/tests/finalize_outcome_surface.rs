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
use std::os::unix::fs::PermissionsExt;
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
        repo.join("docs")
            .join("decisions")
            .join("single-node-cache.md")
            .exists(),
        "task A must promote docs/decisions/single-node-cache.md",
    );
}

/// A **conformant** OOB edit on the committed ADR, made outside the CLI (plain
/// `fs::write` — the human-in-git channel): prose-only, so the reconcile classifier
/// re-parses clean and routes to absorb, never to a conformance block.
fn oob_edit_committed_adr(repo: &Path) {
    let path = repo
        .join("docs")
        .join("decisions")
        .join("single-node-cache.md");
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

/// Seed the file-state record with `task`'s staged commit doc's **real** digest so
/// the sweep sees `IN_SYNC`, not a fresh baseline-adopt. `task validate`/finalize
/// never persist the working-area baseline, so without the seed the staged commit
/// doc would adopt anew on every sweep (the severity-tuning seeding pattern, here
/// with the matching hash). Merges into any existing record — a prior task's
/// landed finalize may have posted baselines this seed must not drop.
fn seed_in_sync(repo: &Path, task: &str) {
    let staged = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("commit:{task}.md"));
    let digest = engine::file_state::hash_bytes(&fs::read(&staged).expect("read staged doc"));
    let state = repo.join(".jigc").join("state");
    fs::create_dir_all(&state).expect("create .jigc/state");
    let record = state.join("file-state.json");
    let mut body: serde_json::Value = match fs::read_to_string(&record) {
        Ok(existing) => serde_json::from_str(&existing).expect("parse existing file-state.json"),
        Err(_) => serde_json::json!({ "hashes": {} }),
    };
    body["hashes"][format!("docs/commit:{task}.md")] = serde_json::Value::String(digest);
    fs::write(record, body.to_string()).expect("seed file-state.json");
}

/// The clean scenario: a commit-only task whose preflight report is **empty**.
/// Asserts the finalize lands and returns its stdout — which must carry the
/// positive no-findings signal.
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
    seed_in_sync(repo.path(), task);

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
    // Insert the `supersedes` line before the closing front-matter fence (the created
    // ADR now carries materialized `status`/`date` header lines, not an empty fence).
    let with = body.replacen("\n---\n", &format!("\nsupersedes: {target}\n---\n"), 1);
    assert_ne!(
        body, with,
        "the staged ADR carries a front-matter block to inject the ref into"
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

/// M17 increment 1, T3 — the operational-error path honors `--format json`
/// (`design/measurement.md` → The capture substrate, item 2: the second verified gap —
/// the error funnels printed plain text even under the flag). Under `--format json`
/// the error surfaces on stderr as the single-key envelope `{"error": "<anyhow
/// chain>"}`, exit stays **1** (an operational error is not a validation outcome);
/// under the agent default the output stays today's `{err:#}` plain text,
/// byte-unchanged.
#[test]
fn operational_error_honors_json() {
    let repo = TempDir::new("operr");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── json: stderr parses as the single-key error object; exit stays 1 ─────────
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "nonexistent", "--format", "json"],
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "an operational error stays exit 1 under --format json; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        out.stdout.is_empty(),
        "the error envelope rides stderr, not stdout; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    let value: serde_json::Value = serde_json::from_str(&stderr).unwrap_or_else(|err| {
        panic!(
            "under --format json, stderr must parse as the error envelope ({err}); got:\n{stderr}"
        )
    });
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("the error envelope is a JSON object; got:\n{stderr}"));
    assert_eq!(
        object.keys().collect::<Vec<_>>(),
        ["error"],
        "the envelope is the single-key object {{\"error\": …}}; got:\n{stderr}",
    );
    let chain = value["error"]
        .as_str()
        .unwrap_or_else(|| panic!("`error` carries the anyhow chain as a string; got:\n{stderr}"));
    assert!(
        chain.contains("no task `nonexistent`") && chain.contains("jigc start"),
        "the chain carries the start-a-task route; got:\n{chain}",
    );

    // ── agent (default): today's plain text, byte-unchanged ──────────────────────
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "nonexistent"],
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "an operational error stays exit 1 under the agent default; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        String::from_utf8(out.stderr).expect("utf-8 stderr"),
        "no task `nonexistent` — start one with `jigc start \"<intent>\"`\n",
        "the agent-format operational error stays the plain `{{err:#}}` bytes",
    );
}

/// M17 increment 1 — the `doc` verbs' operational-error path honors `--format json`
/// too: `DocCommand::dispatch` receives `format` (its Block arm is format-rendered),
/// so its `Orchestration` arm must ride the same shared funnel as every other
/// format-bearing verb, not plain `{err:#}` text. Under `--format json` the error
/// surfaces on stderr as the single-key `{"error": …}` envelope, exit stays **1**;
/// under the agent default the plain bytes stay unchanged.
#[test]
fn doc_operational_error_honors_json() {
    let repo = TempDir::new("doc-operr");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── json: stderr parses as the single-key error object; exit stays 1 ─────────
    let out = jigc_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            "commit:nope#type",
            "--value",
            "feat",
            "--format",
            "json",
        ],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a doc operational error stays exit 1 under --format json; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    let value: serde_json::Value = serde_json::from_str(&stderr).unwrap_or_else(|err| {
        panic!(
            "under --format json, doc's stderr must parse as the error envelope ({err}); \
             got:\n{stderr}"
        )
    });
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("the error envelope is a JSON object; got:\n{stderr}"));
    assert_eq!(
        object.keys().collect::<Vec<_>>(),
        ["error"],
        "the envelope is the single-key object {{\"error\": …}}; got:\n{stderr}",
    );
    let chain = value["error"]
        .as_str()
        .unwrap_or_else(|| panic!("`error` carries the anyhow chain as a string; got:\n{stderr}"));
    assert!(
        chain.contains("no active task"),
        "the chain carries the no-active-task route; got:\n{chain}",
    );

    // ── agent (default): today's plain text, byte-unchanged ──────────────────────
    let out = jigc_doc(
        repo.path(),
        home.path(),
        &["set-field", "commit:nope#type", "--value", "feat"],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a doc operational error stays exit 1 under the agent default; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        String::from_utf8(out.stderr).expect("utf-8 stderr"),
        "no active task — start one with `jigc start`\n",
        "the agent-format doc operational error stays the plain `{{err:#}}` bytes",
    );
}

/// M17 increment 1 — the `config` verbs' operational-error path honors `--format
/// json` too: `--format` is `global = true` (every verb accepts it), so the config
/// funnel must ride the same shared `render::operational_error` as every other
/// format-bearing verb, not plain `{err:#}` text. Under `--format json` the error
/// surfaces on stderr as the single-key `{"error": …}` envelope, exit stays **1**;
/// under the agent default the plain bytes stay unchanged.
#[test]
fn config_operational_error_honors_json() {
    let repo = TempDir::new("config-operr");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── json: stderr parses as the single-key error object; exit stays 1 ─────────
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "config",
            "set",
            "bogus-knob",
            "somevalue",
            "--format",
            "json",
        ],
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a config operational error stays exit 1 under --format json; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    let value: serde_json::Value = serde_json::from_str(&stderr).unwrap_or_else(|err| {
        panic!(
            "under --format json, config's stderr must parse as the error envelope ({err}); \
             got:\n{stderr}"
        )
    });
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("the error envelope is a JSON object; got:\n{stderr}"));
    assert_eq!(
        object.keys().collect::<Vec<_>>(),
        ["error"],
        "the envelope is the single-key object {{\"error\": …}}; got:\n{stderr}",
    );
    let chain = value["error"]
        .as_str()
        .unwrap_or_else(|| panic!("`error` carries the anyhow chain as a string; got:\n{stderr}"));
    assert!(
        chain.contains("not a settable knob"),
        "the chain carries the undeclared-knob rejection; got:\n{chain}",
    );

    // ── agent (default): today's plain text, byte-unchanged ──────────────────────
    let out = jigc(
        repo.path(),
        home.path(),
        &["config", "set", "bogus-knob", "somevalue"],
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a config operational error stays exit 1 under the agent default; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        String::from_utf8(out.stderr).expect("utf-8 stderr"),
        "`bogus-knob` is not a settable knob — the cascade surface is closed\n  \
         route: run `jigc start` to orient; settable knobs are declared by the pack\n",
        "the agent-format config operational error stays the plain `{{err:#}}` bytes",
    );
}

/// The `--format json` structured report is the primary machine output: it must ride
/// **stdout** regardless of the exit code, so `jigc --format json task finalize <task>
/// > report.json` captures the findings on a block, not an empty file. The exit code
/// (0 vs 3) carries pass/block; the report carries the findings. A blocked
/// `--format json` finalize must therefore exit 3 with a non-empty, parseable report
/// on stdout (the `{schema_version, findings}` envelope, ≥1 blocking finding), never on
/// stderr. (Human-oriented agent-text diagnostics stay on stderr — `validation_blocked_exits_3`.)
#[test]
fn blocked_json_finalize_report_rides_stdout() {
    let repo = TempDir::new("blocked-json-stdout");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let task = stage_dangling_supersedes(repo.path(), home.path());

    let out = finalize(repo.path(), home.path(), task, Some("json"));
    assert_eq!(
        out.status.code(),
        Some(3),
        "a validation-blocked --format json finalize must exit 3; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.stdout.is_empty(),
        "the --format json report must ride stdout on a block (so `> report.json` is \
         non-empty), not stderr; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!("the blocked json report on stdout must parse ({err}); got:\n{stdout}")
    });
    assert_eq!(
        value["schema_version"].as_u64(),
        Some(2),
        "the stdout report carries the schema_version envelope; got:\n{stdout}",
    );
    let findings = parse_envelope(&stdout, "blocked finalize (json, stdout)");
    assert!(
        findings.iter().any(|f| f["severity"] == "blocking"),
        "the stdout report carries ≥1 blocking finding; got:\n{stdout}",
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

/// Mint + fill + land a commit-only task in `repo`, returning the finalize output
/// for the caller's exit-code/stream assertions. The fill is clean (in-sync seeded)
/// so the landed envelope is the deterministic positive signal; each task writes
/// one distinct code file (the agent's authored change) so its commit is never
/// empty — a second commit-only task in the same repo would otherwise have nothing
/// for git to commit.
fn land_commit_only(
    repo: &Path,
    home: &Path,
    task: &str,
    intent: &str,
    format: Option<&str>,
) -> std::process::Output {
    let out = jigc(repo, home, &["start", "--workflow", "single-task", intent]);
    assert_ok(&out, &format!("`jigc start` ({task})"));
    fs::write(repo.join(format!("{task}.txt")), "the code change\n").expect("write code change");
    fill_commit(repo, home, task);
    seed_in_sync(repo, task);
    finalize(repo, home, task, format)
}

/// The exit code of an outcome, with the streams surfaced on a missing code.
fn code_of(out: &std::process::Output, what: &str) -> i32 {
    out.status.code().unwrap_or_else(|| {
        panic!(
            "{what} must exit with a code (not a signal); stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        )
    })
}

/// M17 increment 1, T4 — the discrimination matrix (`design/measurement.md` →
/// drift-caught definition + The capture substrate: the harness-side tally keys on
/// exit code + finding codes from output, so every invocation outcome must be
/// pairwise distinguishable by exit code + output alone). One throwaway repo, both
/// renderer families (agent default + `--format json`):
///
/// - **landed finalize** → exit 0, the findings envelope on **stdout**;
/// - **validation-blocked finalize** → exit 3; the agent-text diagnostic on **stderr**,
///   but the `--format json` structured report on **stdout** (machine output, captured
///   by `> report.json` regardless of exit code);
/// - **operational error** → exit 1, the `{"error": …}` envelope on stderr under
///   `--format json` (plain `{err:#}` text under agent);
/// - **usage error** → exit 2 (clap's convention), format-independent.
///
/// The code space is asserted collision-free per family — the drift-caught counting
/// contract: a tally that reads exit codes never conflates a stopped bad commit
/// with a crashed binary or a typo'd invocation.
#[test]
fn outcome_space_is_discriminable() {
    let repo = TempDir::new("matrix");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── outcome: landed finalize → 0 + envelope on stdout, both families ─────────
    let landed_agent = land_commit_only(
        repo.path(),
        home.path(),
        "land-the-agent-probe",
        "land the agent probe",
        None,
    );
    assert_eq!(
        code_of(&landed_agent, "landed finalize (agent)"),
        0,
        "a landed finalize exits 0; stderr:\n{}",
        String::from_utf8_lossy(&landed_agent.stderr),
    );
    let stdout = String::from_utf8_lossy(&landed_agent.stdout);
    assert!(
        stdout.contains("no findings — the task validates clean"),
        "the landed agent envelope rides stdout; got:\n{stdout}",
    );
    assert!(
        landed_agent.stderr.is_empty(),
        "a landed finalize keeps stderr silent (the envelope-stream split is the \
         landed/blocked discriminator); stderr:\n{}",
        String::from_utf8_lossy(&landed_agent.stderr),
    );

    let landed_json = land_commit_only(
        repo.path(),
        home.path(),
        "land-the-json-probe",
        "land the json probe",
        Some("json"),
    );
    assert_eq!(code_of(&landed_json, "landed finalize (json)"), 0);
    let stdout = String::from_utf8(landed_json.stdout.clone()).expect("utf-8 stdout");
    let findings = parse_envelope(&stdout, "landed finalize (json)");
    assert!(
        findings.is_empty(),
        "the clean landed json envelope carries an empty findings array; got:\n{stdout}",
    );
    assert!(
        landed_json.stderr.is_empty(),
        "a landed json finalize keeps stderr silent; stderr:\n{}",
        String::from_utf8_lossy(&landed_json.stderr),
    );

    // ── outcome: validation-blocked finalize → 3 + envelope on stderr ────────────
    let task = stage_dangling_supersedes(repo.path(), home.path());

    let blocked_agent = finalize(repo.path(), home.path(), task, None);
    assert_eq!(
        code_of(&blocked_agent, "blocked finalize (agent)"),
        3,
        "a validation-blocked finalize exits 3; stderr:\n{}",
        String::from_utf8_lossy(&blocked_agent.stderr),
    );
    assert!(
        blocked_agent.stdout.is_empty(),
        "a blocked finalize keeps stdout silent; stdout:\n{}",
        String::from_utf8_lossy(&blocked_agent.stdout),
    );
    let stderr = String::from_utf8_lossy(&blocked_agent.stderr);
    assert!(
        stderr.contains("blocking") && stderr.contains("adr:typo-nonexistent"),
        "the blocked agent envelope rides stderr, naming the dangling target; got:\n{stderr}",
    );

    // A block consumes nothing — the same blocked task re-runs under json. The
    // `--format json` report is machine output: it rides stdout regardless of the
    // blocking exit (so `> report.json` captures it), unlike the agent-text diagnostic.
    let blocked_json = finalize(repo.path(), home.path(), task, Some("json"));
    assert_eq!(code_of(&blocked_json, "blocked finalize (json)"), 3);
    let stdout = String::from_utf8(blocked_json.stdout.clone()).expect("utf-8 stdout");
    let findings = parse_envelope(&stdout, "blocked finalize (json, stdout)");
    assert!(
        findings.iter().any(|f| f["severity"] == "blocking"),
        "the blocked json envelope rides stdout and carries ≥1 blocking finding; got:\n{stdout}",
    );

    // ── outcome: operational error → 1; json gets the error envelope ─────────────
    let op_agent = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "nonexistent"],
    );
    assert_eq!(
        code_of(&op_agent, "operational error (agent)"),
        1,
        "an operational error stays exit 1; stderr:\n{}",
        String::from_utf8_lossy(&op_agent.stderr),
    );
    let stderr = String::from_utf8_lossy(&op_agent.stderr);
    assert!(
        stderr.contains("no task `nonexistent`"),
        "the agent operational error carries the plain chain; got:\n{stderr}",
    );

    let op_json = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "nonexistent", "--format", "json"],
    );
    assert_eq!(code_of(&op_json, "operational error (json)"), 1);
    let stderr = String::from_utf8(op_json.stderr.clone()).expect("utf-8 stderr");
    let value: serde_json::Value = serde_json::from_str(&stderr).unwrap_or_else(|err| {
        panic!("the json operational error parses as the error envelope ({err}); got:\n{stderr}")
    });
    assert!(
        value["error"].is_string() && value["findings"].is_null(),
        "the operational envelope is `{{\"error\": …}}`, never a findings report \
         (a tally must not read a crash as a validation outcome); got:\n{stderr}",
    );

    // ── outcome: usage error → 2, format-independent ──────────────────────────────
    let usage_agent = jigc(repo.path(), home.path(), &["task", "finalize"]);
    assert_eq!(
        code_of(&usage_agent, "usage error (agent)"),
        2,
        "a usage error exits 2 (clap's convention); stderr:\n{}",
        String::from_utf8_lossy(&usage_agent.stderr),
    );
    assert!(
        String::from_utf8_lossy(&usage_agent.stderr).contains("Usage"),
        "the usage error carries clap's usage text; stderr:\n{}",
        String::from_utf8_lossy(&usage_agent.stderr),
    );

    let usage_json = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", "--format", "json"],
    );
    assert_eq!(
        code_of(&usage_json, "usage error (json)"),
        2,
        "a usage error stays 2 under --format json; stderr:\n{}",
        String::from_utf8_lossy(&usage_json.stderr),
    );

    // ── the collision-free code space, per renderer family ───────────────────────
    for (family, codes) in [
        (
            "agent",
            [
                code_of(&landed_agent, "landed (agent)"),
                code_of(&blocked_agent, "blocked (agent)"),
                code_of(&op_agent, "operational (agent)"),
                code_of(&usage_agent, "usage (agent)"),
            ],
        ),
        (
            "json",
            [
                code_of(&landed_json, "landed (json)"),
                code_of(&blocked_json, "blocked (json)"),
                code_of(&op_json, "operational (json)"),
                code_of(&usage_json, "usage (json)"),
            ],
        ),
    ] {
        let distinct: std::collections::HashSet<i32> = codes.iter().copied().collect();
        assert_eq!(
            distinct.len(),
            codes.len(),
            "the {family}-family exit-code space must be collision-free \
             (landed/blocked/operational/usage); got {codes:?}",
        );
    }
}

/// Run `git <args>` in `repo`, returning trimmed stdout (asserting success). Used to
/// learn the just-landed commit's real short hash + subject so the success-summary
/// assertions check against ground truth, not a reconstruction.
fn git_out(repo: &Path, args: &[&str]) -> String {
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
        .expect("utf-8 git output")
        .trim()
        .to_string()
}

/// Mint + author an ADR-promoting single-task, finalize it, and return
/// `(finalize stdout, the landed short hash, the landed subject)`. The ADR promotes to
/// `docs/decisions/single-node-cache.md`, so the caller can assert the success summary names
/// the landed commit and the promoted persisted doc.
fn promote_adr_and_finalize(format: Option<&str>) -> (String, String, String) {
    let repo = TempDir::new("landed-summary");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start`");
    let task = "cache-sessions-in-a-single-in-memory-node";

    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
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
    fill_commit(repo.path(), home.path(), task);

    let out = finalize(repo.path(), home.path(), task, format);
    assert_ok(&out, "`jigc task finalize`");
    assert!(
        repo.path()
            .join("docs")
            .join("decisions")
            .join("single-node-cache.md")
            .exists(),
        "the finalize must promote docs/decisions/single-node-cache.md",
    );
    let short = git_out(repo.path(), &["rev-parse", "--short", "HEAD"]);
    let subject = git_out(repo.path(), &["log", "-1", "--pretty=format:%s"]);
    (
        String::from_utf8(out.stdout).expect("utf-8 stdout"),
        short,
        subject,
    )
}

/// M26 post-completion shakedown — a SUCCESSFUL `task finalize` must confirm what it
/// did: the landed commit (short hash + subject) and each promoted persisted doc, so a
/// user need not run `git log` to tell it worked. Both renderer families, over an
/// ADR-promoting finalize.
#[test]
fn finalize_reports_landed_commit_and_promotions() {
    // ── agent: the summary names the short hash, subject, and promoted ADR path ───
    let (stdout, short, subject) = promote_adr_and_finalize(None);
    assert!(
        stdout.contains(&short),
        "the success summary must name the short commit hash {short}; got:\n{stdout}",
    );
    assert!(
        stdout.contains(&subject),
        "the success summary must name the commit subject {subject:?}; got:\n{stdout}",
    );
    assert!(
        stdout.contains("docs/decisions/single-node-cache.md"),
        "the success summary must name the promoted ADR path; got:\n{stdout}",
    );

    // ── json: the committed facts ride the report envelope as a `committed` object ─
    let (stdout, short, subject) = promote_adr_and_finalize(Some("json"));
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!("json finalize stdout must parse as one object ({err}); got:\n{stdout}")
    });
    assert!(
        value["findings"].is_array(),
        "the json envelope keeps its findings array (no JSON consumer break); got:\n{stdout}",
    );
    let committed = &value["committed"];
    assert_eq!(
        committed["hash"].as_str(),
        Some(short.as_str()),
        "committed.hash is the landed short hash; got:\n{stdout}",
    );
    assert_eq!(
        committed["subject"].as_str(),
        Some(subject.as_str()),
        "committed.subject is the landed commit subject; got:\n{stdout}",
    );
    let promoted: Vec<&str> = committed["promoted"]
        .as_array()
        .unwrap_or_else(|| panic!("committed.promoted is an array; got:\n{stdout}"))
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect();
    assert!(
        promoted.contains(&"docs/decisions/single-node-cache.md"),
        "committed.promoted names the promoted ADR path; got:\n{stdout}",
    );
    assert!(
        committed["files"].as_u64().is_some(),
        "committed.files is a file count; got:\n{stdout}",
    );
}

/// The marker a non-blocking `pre-commit` hook writes to **stderr** before exiting 0 —
/// git redirects a hook's own stdout to stderr, so a warn-only hook (e.g. the M19
/// doc↔code backstop) speaks on the hook stream `git_commit` captures.
const HOOK_WARNING: &str = "NON-BLOCKING-HOOK-WARNING";

/// Install a **non-blocking** `pre-commit` hook into `repo`'s `.git/hooks` that prints
/// `HOOK_WARNING` and exits 0 — git surfaces it on the commit's stderr (the hook
/// stream), the warn-only backstop's shape. Marked executable so git runs it.
fn install_warning_hook(repo: &Path) {
    let hooks = repo.join(".git").join("hooks");
    fs::create_dir_all(&hooks).expect("create .git/hooks");
    let path = hooks.join("pre-commit");
    fs::write(
        &path,
        format!("#!/bin/sh\necho {HOOK_WARNING} 1>&2\nexit 0\n"),
    )
    .expect("write the non-blocking pre-commit hook");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
        .expect("make the hook executable");
}

/// M19 increment 2, T2 — `jigc task finalize` relays a non-blocking `pre-commit`
/// hook's success output to the agent (today only on rejection), closing the
/// detect→surface loop the M19 spike exposed (`design/finalize.md` → 6. Commit,
/// success-relay; review S1: delimited section **after** the success result, never in
/// the routing footer or the `--format json` envelope). Three sub-cases:
///
/// - **agent-text:** the hook warning appears in a delimited section **after** the
///   `render::validation` routing footer; the commit lands (exit 0).
/// - **`--format json`:** stdout still parses as the un-corrupted report envelope, AND
///   the hook warning rides **stderr** (never the stdout envelope).
/// - **no hook output:** a finalize with no hook speaking emits **no** delimiter.
#[test]
fn finalize_relays_hook_output_on_success() {
    // ── agent-text: the warning is a delimited section after the footer ──────────
    let repo = TempDir::new("relay-agent");
    let home = TempDir::new("home");
    init_repo(repo.path());
    install_warning_hook(repo.path());
    let out = land_commit_only(
        repo.path(),
        home.path(),
        "relay-the-agent-warning",
        "relay the agent warning",
        None,
    );
    assert_eq!(
        code_of(&out, "relay finalize (agent)"),
        0,
        "the non-blocking hook must not block the landed commit; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let footer_at = stdout.find("— jigc ·").unwrap_or_else(|| {
        panic!("the agent-text output carries the routing footer; got:\n{stdout}")
    });
    let warning_at = stdout.find(HOOK_WARNING).unwrap_or_else(|| {
        panic!("the hook warning must be relayed to the agent on success; got:\n{stdout}")
    });
    assert!(
        warning_at > footer_at,
        "the hook relay is a delimited section AFTER the routing footer (review S1), \
         never interleaved into it; got:\n{stdout}",
    );
    // The relay section is the last thing emitted; its output shape must be clean —
    // exactly one trailing newline, never a spurious trailing blank line (M19
    // completion audit LOW). The agent-text relay ends `--- hook output ---\n<warning>\n`.
    assert!(
        stdout.ends_with(&format!("{HOOK_WARNING}\n")),
        "the relay section ends with the warning + a single newline; got:\n{stdout:?}",
    );
    assert!(
        !stdout.ends_with(&format!("{HOOK_WARNING}\n\n")),
        "the relayed hook-output section must not end with a trailing blank line; got:\n{stdout:?}",
    );

    // ── --format json: stdout stays the un-corrupted envelope; warning on stderr ─
    let repo = TempDir::new("relay-json");
    let home = TempDir::new("home");
    init_repo(repo.path());
    install_warning_hook(repo.path());
    let out = land_commit_only(
        repo.path(),
        home.path(),
        "relay-the-json-warning",
        "relay the json warning",
        Some("json"),
    );
    assert_eq!(code_of(&out, "relay finalize (json)"), 0);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let findings = parse_envelope(&stdout, "relay finalize (json)");
    assert!(
        findings.is_empty(),
        "the clean json envelope stays un-corrupted by the relay; got:\n{stdout}",
    );
    assert!(
        !stdout.contains(HOOK_WARNING),
        "under --format json the relay never touches the stdout envelope; got:\n{stdout}",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains(HOOK_WARNING),
        "under --format json the hook relay rides stderr; got stderr:\n{stderr}",
    );

    // ── no hook output: no delimiter is emitted ──────────────────────────────────
    let repo = TempDir::new("relay-none");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let out = land_commit_only(
        repo.path(),
        home.path(),
        "relay-no-warning",
        "relay no warning",
        None,
    );
    assert_eq!(code_of(&out, "no-hook finalize (agent)"), 0);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        !stdout.contains("hook output"),
        "a finalize with no hook output emits no relay delimiter; got:\n{stdout}",
    );
}
