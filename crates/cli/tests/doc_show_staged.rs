//! M43 Increment 5 / T2 — `jigc doc show <addr> --task <id>`, the **staged read**,
//! proven end-to-end over the real `jigc` binary (`design/doc-read-surface.md` →
//! What it reads — the R7 revision; `design/surface-contract.md` → law 2: the
//! staged read; DECISIONS 2026-07-16 M43 Settle items 3 + 12).
//!
//! The contract this pins: task-less `doc show` stays the **committed** read
//! (committed-by-*default*, the R7 revision); `--task <id>` serves that task's
//! **staged** working copy through the identical parse/slice/render path at every
//! slice depth; a **staged whole-doc `--format json`** serve carries the one
//! additive marker key `"staged": "<task-id>"` while a committed serve stays
//! byte-shape-identical to the pin; a **transient** doc (`commit:<task-id>`) is
//! staged-readable (the B9 leak closed from the read side); the not-staged /
//! bad-task / transient blocks route honestly; and `--task` no longer trips the
//! misleading clap `-- --task` tip.
//!
//! Everything is asserted on the EMITTED bytes + exit codes of the real binary
//! (`CARGO_BIN_EXE_jigc`). No external test crates.

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
            "jigc-doc-show-staged-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// (dev pack only — the staged read is pack-agnostic).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
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

/// The stdout of an invocation as UTF-8.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

/// The stderr of an invocation as UTF-8.
fn stderr_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stderr.clone()).expect("utf-8 stderr")
}

/// Mint a `single-task` task on `intent` (the dev pack's Form-D door) — the fixed
/// intents below slugify to known task ids (the `doc_write.rs` proven pairs).
fn start_task(repo: &Path, home: &Path, intent: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "single-task", intent],
            None,
        ),
        &format!("`jigc start --workflow single-task \"{intent}\"`"),
    );
}

/// Set one prose slot through the binary (stdin), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, task: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, task: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value, "--task", task],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Create + fully fill an `adr:cache-strategy` in `task` (single-task's create-gate
/// `allows-create` an adr as the decision), so the staged instance parses and — when
/// the caller finalizes — the required-slot gate passes.
fn stage_cache_strategy_adr(repo: &Path, home: &Path, task: &str) {
    let create = jigc(
        repo,
        home,
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache strategy",
            "--task",
            task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    set_slot(
        repo,
        home,
        "adr:cache-strategy#context",
        task,
        b"Lookups must stay fast.\n",
    );
    set_slot(
        repo,
        home,
        "adr:cache-strategy#decision",
        task,
        b"Cache locally.\n",
    );
    set_slot(
        repo,
        home,
        "adr:cache-strategy#consequences",
        task,
        b"A cold node re-warms.\n",
    );
}

/// Fill the auto-provisioned commit doc so a doc-only task finalizes.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), task, "docs");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        task,
        b"record the cache decision\n",
    );
}

/// The staged serve: `--task <id>` reads the task's staged working copy — plain,
/// `--format json` (with the `staged` marker key carrying the task id), and at
/// slice depth — while the task-less read of the same address still blocks
/// `store.not-found` (the proof the serve came from the staged source), with the
/// not-found route now naming the staged read instead of the retired `task diff`.
#[test]
fn staged_read_serves_plain_json_and_slice() {
    let repo = TempDir::new("serve");
    let home = TempDir::new("home");
    init_repo(repo.path());
    start_task(repo.path(), home.path(), "add rate limiter");
    let task = "add-rate-limiter";
    stage_cache_strategy_adr(repo.path(), home.path(), task);

    // (1) Plain whole-doc: the staged bytes serve.
    let plain = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy", "--task", task],
        None,
    );
    assert_ok(&plain, "`jigc doc show adr:cache-strategy --task <id>`");
    let plain = stdout_of(&plain);
    assert!(
        plain.contains("# Cache strategy") && plain.contains("Cache locally."),
        "the staged whole-doc read serves the staged bytes; got:\n{plain}"
    );

    // (2) `--format json`: the pinned whole-doc shape + the one additive marker key
    //     `staged` carrying the task id.
    let json = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "adr:cache-strategy",
            "--task",
            task,
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&json, "`jigc doc show ... --task <id> --format json`");
    let value: serde_json::Value = serde_json::from_str(&stdout_of(&json)).expect("valid json");
    assert_eq!(
        value["staged"], task,
        "the staged whole-doc json carries the marker key with the task id"
    );
    assert_eq!(value["type"], "adr");
    assert_eq!(value["slug"], "cache-strategy");
    assert_eq!(value["sections"]["decision"], "Cache locally.");
    let mut keys: Vec<&str> = value
        .as_object()
        .expect("whole-doc object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["fields", "sections", "slug", "staged", "type"],
        "exactly the pinned keys + the one marker key"
    );

    // (3) A `#section` slice serves the staged prose — plain and json (a fragment
    //     slice is a bare value: no object, no marker key — the conscious bound).
    let slice = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy#decision", "--task", task],
        None,
    );
    assert_ok(&slice, "`jigc doc show adr:...#decision --task <id>`");
    assert_eq!(stdout_of(&slice).trim(), "Cache locally.");
    let slice_json = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "adr:cache-strategy#decision",
            "--task",
            task,
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(
        &slice_json,
        "`jigc doc show <slice> --task <id> --format json`",
    );
    assert_eq!(
        stdout_of(&slice_json).trim(),
        "\"Cache locally.\"",
        "a staged fragment slice is the bare value, exactly as committed"
    );

    // (4) The staged-source proof: the task-less read of the same address still
    //     blocks — nothing is committed — and its route names the STAGED read
    //     (superseding the retired `jigc task diff` route).
    let committed_miss = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy"],
        None,
    );
    assert!(
        !committed_miss.status.success(),
        "the task-less read of a staged-only doc must block; stdout:\n{}",
        stdout_of(&committed_miss),
    );
    let stderr = stderr_of(&committed_miss);
    assert!(
        stderr.contains("jigc doc show adr:cache-strategy --task <task-id>"),
        "the not-found route names the staged read; got:\n{stderr}"
    );
    assert!(
        !stderr.contains("task diff"),
        "the superseded task-diff route is retired; got:\n{stderr}"
    );
}

/// The marker key is **staged serves only**: after finalize, the committed
/// whole-doc `--format json` shape is byte-shape-identical to the pin (exactly
/// `{type, slug, fields, sections}` — no `staged` key); and a task that has NOT
/// staged the doc gets the honest `store.not-staged` block routed at the
/// task-less read.
#[test]
fn committed_json_carries_no_marker_and_not_staged_routes_task_less() {
    let repo = TempDir::new("committed");
    let home = TempDir::new("home");
    init_repo(repo.path());
    start_task(repo.path(), home.path(), "add rate limiter");
    let task = "add-rate-limiter";
    stage_cache_strategy_adr(repo.path(), home.path(), task);
    fill_commit(repo.path(), home.path(), task);
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`jigc task finalize` — the committed adr",
    );

    // (1) The committed whole-doc json: exactly the four pinned keys, no marker.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy", "--format", "json"],
        None,
    );
    assert_ok(&json, "`jigc doc show adr:cache-strategy --format json`");
    let value: serde_json::Value = serde_json::from_str(&stdout_of(&json)).expect("valid json");
    let mut keys: Vec<&str> = value
        .as_object()
        .expect("whole-doc object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["fields", "sections", "slug", "type"],
        "a committed serve stays byte-shape-identical to the pin — no `staged` key"
    );

    // (2) A second task that never touched the adr: `--task` blocks `store.not-staged`
    //     and routes at the task-less read of the committed copy.
    start_task(repo.path(), home.path(), "tune rate limiter");
    let other = "tune-rate-limiter";
    let not_staged = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy", "--task", other],
        None,
    );
    assert!(
        !not_staged.status.success(),
        "a staged read of an un-staged doc must block; stdout:\n{}",
        stdout_of(&not_staged),
    );
    let stderr = stderr_of(&not_staged);
    assert!(
        stderr.contains("not staged"),
        "the block states the real state; got:\n{stderr}"
    );
    assert!(
        stderr.contains("jigc doc show adr:cache-strategy"),
        "the route names the task-less read of the committed copy; got:\n{stderr}"
    );
}

/// The transient staged address (the Settle-12 rider): `commit:<task>` — a doctype
/// with no committed home — is staged-readable via `--task <task>` (plain and
/// json, marker key included), and the task-less `store.transient-type` block now
/// routes at that staged read (the B9 leak closed from the read side).
#[test]
fn a_transient_commit_doc_is_staged_readable() {
    let repo = TempDir::new("transient");
    let home = TempDir::new("home");
    init_repo(repo.path());
    start_task(repo.path(), home.path(), "add rate limiter");
    let task = "add-rate-limiter";
    let addr = format!("commit:{task}");
    set_field(
        repo.path(),
        home.path(),
        &format!("{addr}#type"),
        task,
        "feat",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}#summary"),
        task,
        b"add a per-client rate limit\n",
    );

    // (1) The staged read serves the provisioned + filled commit doc.
    let plain = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", &addr, "--task", task],
        None,
    );
    assert_ok(&plain, "`jigc doc show commit:<task> --task <task>`");
    let plain = stdout_of(&plain);
    assert!(
        plain.contains("type: feat") && plain.contains("add a per-client rate limit"),
        "the staged transient read serves the staged bytes; got:\n{plain}"
    );

    // (2) json: the commit doc under the pinned shape + the marker key.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", &addr, "--task", task, "--format", "json"],
        None,
    );
    assert_ok(
        &json,
        "`jigc doc show commit:<task> --task <task> --format json`",
    );
    let value: serde_json::Value = serde_json::from_str(&stdout_of(&json)).expect("valid json");
    assert_eq!(value["type"], "commit");
    assert_eq!(value["staged"], task);
    assert_eq!(value["fields"]["type"], "feat");

    // (3) Task-less, a transient address still blocks — and the route names the
    //     staged read, no longer a dead end.
    let gate = jigc(repo.path(), home.path(), &["doc", "show", &addr], None);
    assert!(
        !gate.status.success(),
        "the task-less transient read must block; stdout:\n{}",
        stdout_of(&gate),
    );
    let stderr = stderr_of(&gate);
    assert!(
        stderr.contains("transient"),
        "the block names the transient gate; got:\n{stderr}"
    );
    assert!(
        stderr.contains(&format!("jigc doc show {addr} --task <task-id>")),
        "the transient route names the staged read; got:\n{stderr}"
    );
}

/// `--task` is a real argument now: a bad task id routes to `jigc task list` (the
/// shared wrong-id route), and the misleading clap tip — "unexpected argument
/// '--task'… to pass '--task' as a value, use '-- --task'" — is gone.
#[test]
fn a_bad_task_id_routes_to_task_list_not_the_clap_tip() {
    let repo = TempDir::new("bad-task");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:x", "--task", "t1"],
        None,
    );
    assert!(
        !out.status.success(),
        "a nonexistent task id must fail; stdout:\n{}",
        stdout_of(&out),
    );
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains("no task `t1`") && stderr.contains("jigc task list"),
        "the failure is the shared wrong-id route; got:\n{stderr}"
    );
    assert!(
        !stderr.contains("unexpected argument") && !stderr.contains("-- --task"),
        "the misleading clap `-- --task` tip is gone; got:\n{stderr}"
    );
}
