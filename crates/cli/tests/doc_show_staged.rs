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
            engine::tempname::unique_nanos(),
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
    // The item-count additive key rides the staged serve too, so `staged` stays the SOLE
    // staged/committed differentiator (an adr has no repeatable section → 0).
    assert_eq!(
        value["item-count"], 0,
        "the staged whole-doc json carries item-count (0 for a repeatable-less adr)"
    );
    let mut keys: Vec<&str> = value
        .as_object()
        .expect("whole-doc object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["fields", "item-count", "sections", "slug", "staged", "type"],
        "exactly the pinned keys + item-count + the one marker key"
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
    // The committed serve carries item-count too (present on both serves — it is `staged`,
    // not item-count, that separates the two views), and differs from the staged serve by
    // EXACTLY the `staged` key.
    assert_eq!(
        value["item-count"], 0,
        "the committed whole-doc json carries item-count (0 for a repeatable-less adr)"
    );
    let mut keys: Vec<&str> = value
        .as_object()
        .expect("whole-doc object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["fields", "item-count", "sections", "slug", "type"],
        "a committed serve differs from the staged serve by exactly the `staged` key"
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

/// The stale-read hint (M43 Inc 5 T3; `design/surface-contract.md` → law 2 + the
/// style guide): a **task-less** read of a committed doc that is ALSO staged in an
/// open task prints one **stderr** line naming the open task id + the staged-read
/// command — on the plain read AND the `--format json` read — while **stdout stays
/// byte-identical** to the no-open-task serve (the canonical render / the pinned
/// json carry no second additive key). With no open task staging the doc, the hint
/// is absent.
#[test]
fn a_task_less_read_of_a_staged_elsewhere_doc_hints_on_stderr() {
    let repo = TempDir::new("stale-hint");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Commit the adr through a first task, which finalize retires.
    start_task(repo.path(), home.path(), "add rate limiter");
    let task = "add-rate-limiter";
    stage_cache_strategy_adr(repo.path(), home.path(), task);
    fill_commit(repo.path(), home.path(), task);
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`jigc task finalize` — the committed adr",
    );

    // (1) No open task stages the adr: plain + json serve clean, hint ABSENT.
    let plain_before = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy"],
        None,
    );
    assert_ok(&plain_before, "the task-less plain read, no open task");
    assert!(
        !stderr_of(&plain_before).contains("staged"),
        "no open task stages the doc — no hint; got:\n{}",
        stderr_of(&plain_before),
    );
    let json_before = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy", "--format", "json"],
        None,
    );
    assert_ok(&json_before, "the task-less json read, no open task");
    assert!(
        !stderr_of(&json_before).contains("staged"),
        "no open task stages the doc — no hint on json either; got:\n{}",
        stderr_of(&json_before),
    );

    // (2) A second open task stages an edit (copy-on-first-touch), leaving the
    //     committed copy behind the staged one.
    start_task(repo.path(), home.path(), "tune rate limiter");
    let other = "tune-rate-limiter";
    set_slot(
        repo.path(),
        home.path(),
        "adr:cache-strategy#decision",
        other,
        b"Cache remotely.\n",
    );

    // (3) Plain: stdout byte-identical; the hint on stderr names the open task id
    //     + the copy-runnable staged-read command.
    let plain_after = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy"],
        None,
    );
    assert_ok(&plain_after, "the task-less plain read, staged elsewhere");
    assert_eq!(
        plain_after.stdout, plain_before.stdout,
        "stdout stays the committed serve, byte-identical with and without the open task"
    );
    let stderr = stderr_of(&plain_after);
    assert!(
        stderr.contains("tune-rate-limiter")
            && stderr.contains("jigc doc show adr:cache-strategy --task tune-rate-limiter"),
        "the hint names the open task id + the staged-read command; got:\n{stderr}"
    );

    // (4) json: same split — pinned stdout untouched, hint on stderr.
    let json_after = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy", "--format", "json"],
        None,
    );
    assert_ok(&json_after, "the task-less json read, staged elsewhere");
    assert_eq!(
        json_after.stdout, json_before.stdout,
        "the pinned json stays byte-identical — the hint is stderr-only, never a key"
    );
    let stderr = stderr_of(&json_after);
    assert!(
        stderr.contains("tune-rate-limiter")
            && stderr.contains("jigc doc show adr:cache-strategy --task tune-rate-limiter"),
        "the json read carries the same stderr hint; got:\n{stderr}"
    );
}

/// (M47 inc-10 T5 — D5) **The stale-read note is phrased for a reader who may BE the
/// staging task** (`design/surface-contract.md` → the style guide;
/// `design/doc-read-surface.md` → The stale-read hint).
///
/// A task-less read carries no task id, so the note cannot know whose task it names —
/// and in the field the commonest stager is the reader's own open task, where
/// *"the committed copy served here may be stale"* read as a third-party warning about
/// someone else's edit (the trial worker reported it as an alarm about their own task).
/// The note now says which copy this read served, that edits staged in the named task
/// are not in it, and hands over the staged read under an explicit *if that task is
/// yours* clause — true whether the stager is the reader or a teammate.
///
/// The axis is the **number of staging tasks**, because the sentence branches on it:
/// one (named, `--task <id>`) and two (listed, the `<task-id>` placeholder).
#[test]
fn the_stale_read_note_addresses_a_reader_who_may_be_the_staging_task() {
    let repo = TempDir::new("stale-self");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Commit the adr through a first task, which finalize retires.
    start_task(repo.path(), home.path(), "add rate limiter");
    stage_cache_strategy_adr(repo.path(), home.path(), "add-rate-limiter");
    fill_commit(repo.path(), home.path(), "add-rate-limiter");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", "add-rate-limiter"],
            None,
        ),
        "`jigc task finalize` — the committed adr",
    );

    // (1) ONE staging task — the reader's own, the D5 case. It copied the doc in and
    //     has not diverged from committed, so the note may not assert that it has.
    start_task(repo.path(), home.path(), "tune rate limiter");
    set_slot(
        repo.path(),
        home.path(),
        "adr:cache-strategy#decision",
        "tune-rate-limiter",
        b"Cache remotely.\n",
    );
    let one = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy"],
        None,
    );
    assert_ok(&one, "the task-less read with one staging task");
    let stderr = stderr_of(&one);
    assert!(
        stderr.contains("is also staged in open task tune-rate-limiter"),
        "the note still names the staging task; got:\n{stderr}",
    );
    assert!(
        stderr.contains("this read served the committed copy"),
        "the note says which copy it served, not what the reader should fear; \
         got:\n{stderr}",
    );
    assert!(
        stderr.contains("any edits staged there are not shown"),
        "the consequence is stated hedged — the check is existence-only, so an \
         identical staged copy is not asserted to differ; got:\n{stderr}",
    );
    assert!(
        stderr.contains("if that task is yours, read your staged work:")
            && stderr.contains("jigc doc show adr:cache-strategy --task tune-rate-limiter"),
        "the staged read is handed over under an explicit `if that task is yours` \
         clause, naming the task; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("may be stale"),
        "the third-party framing is gone — it read as a warning about someone else; \
         got:\n{stderr}",
    );

    // (2) TWO staging tasks — the plural branch: both ids listed, the clause asks
    //     whether ONE of them is the reader's, and the command carries the shared
    //     `<task-id>` placeholder (no single id could be right).
    start_task(repo.path(), home.path(), "trim rate limiter");
    set_slot(
        repo.path(),
        home.path(),
        "adr:cache-strategy#context",
        "trim-rate-limiter",
        b"Traffic doubled.\n",
    );
    let two = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:cache-strategy"],
        None,
    );
    assert_ok(&two, "the task-less read with two staging tasks");
    let stderr = stderr_of(&two);
    assert!(
        stderr.contains("open tasks")
            && stderr.contains("tune-rate-limiter")
            && stderr.contains("trim-rate-limiter"),
        "both staging tasks are listed; got:\n{stderr}",
    );
    assert!(
        stderr.contains("if one of them is yours, read your staged work:")
            && stderr.contains("jigc doc show adr:cache-strategy --task <task-id>"),
        "the plural clause asks whether one of them is the reader's, and the command \
         carries the shared placeholder; got:\n{stderr}",
    );
}

/// The read-side **cause** finding, through the real binary (M45 Increment 2 / T3;
/// `design/validation.md` → The M45 registrations, row 2). A `### Ghost` hand-written
/// into a staged `spec` criterion's slot prose — the out-of-band arm the write gate
/// cannot reach — is read as an item boundary, and the staged read must print *why*:
/// the heading sits at the section's schema-reserved item depth. Before T3 the only
/// thing printed was "repeatable item `Ghost` has no `{#id}` anchor", which blames a
/// missing anchor and never the cause.
#[test]
fn a_ghost_heading_in_staged_item_prose_names_the_reserved_depth_cause() {
    let repo = TempDir::new("ghost");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let task = "add-a-spec";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "plan", "add a spec"],
            None,
        ),
        "`jigc start --workflow plan`",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "create", "spec", "--title", "Widget", "--task", task],
            None,
        ),
        "`jigc doc create spec`",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "spec:widget#criteria",
                "--title",
                "It works",
                "--task",
                task,
            ],
            None,
        ),
        "`jigc doc add-item`",
    );

    // The out-of-band edit: prose, then a heading at the reserved item depth.
    let staged = repo
        .path()
        .join(".jigc/tasks/add-a-spec/docs/spec:widget.md");
    let mut source = fs::read_to_string(&staged).expect("read the staged spec");
    source.push_str("\nSome prose.\n\n### Ghost\n\nmore prose.\n");
    fs::write(&staged, source).expect("write the staged spec");

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "spec:widget", "--task", task],
        None,
    );
    assert!(
        !out.status.success(),
        "the unparseable staged copy must block; stdout:\n{}",
        stdout_of(&out),
    );
    let printed = format!("{}{}", stdout_of(&out), stderr_of(&out));
    assert!(
        printed.contains("schema-reserved item depth"),
        "the staged read prints the cause; got:\n{printed}"
    );
    assert!(
        printed.contains("`####`") && printed.contains("`### Ghost  {#<id>}`"),
        "…with both repairs — demote the heading, or anchor it in place. Neither is a \
         CLI write: the corruption is what stops the parse, so every write door refuses \
         this doc (M46 Increment 5 / T1 — `crates/cli/tests/pre_guard_repair_route.rs`); \
         got:\n{printed}"
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
