//! End-to-end integration test for the `jigc task <verb> <id>` lifecycle surface.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo with a started
//! task, exercising the three preview/abandon verbs
//! (`design/write-commands.md` → Lifecycle: `diff` · `validate` · `discard`):
//!
//! - `task diff <id>` — after staging a write, exits 0 and surfaces the staged
//!   change (`design/write-commands.md` → Lifecycle: see the working changeset).
//! - `task validate <id>` — runs `validate(task)` and renders findings; the exit
//!   code tracks blocking (`design/validation.md` → How it gates `finalize`:
//!   validate previews what finalize blocks on). A conformance-broken instance
//!   exits non-zero listing the `schema-conformance.*` finding; a clean one exits 0.
//! - `task discard <id>` — removes `.jigc/tasks/<id>/` and exits 0
//!   (`design/write-commands.md` → Lifecycle: abandon).
//!
//! No external test crates: the binary path comes from Cargo's `CARGO_BIN_EXE_jigc`,
//! the temp repo is a real `git init` (`jigc start` reads HEAD), and a self-cleaning
//! `TempDir` keeps the test off the developer's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The **no-delta** `jigc task validate` rendered output over the freshly-provisioned
/// (conformance-broken) commit doc, byte for byte — captured from the binary as the
/// pre-M6 baseline. T3 threaded a real `Resolved` into `task.rs::validate` so the M6
/// severity post-pass has a cascade to read; this repo carries no
/// `validation.*.severity` scalar-set, so the post-pass overrides nothing and the
/// rendered bytes must equal the pre-M6 baseline (`design/validation.md` → Severity
/// assignment — the M6 post-pass: the byte-identical golden must cover the validate
/// path, not only `start_compose`; review B2).
const NO_DELTA_BROKEN_VALIDATE_GOLDEN: &str = "\
blocking · schema-conformance.field-value-conformant — `commit:add-rate-limiter`: field `type` in section `header`: \"\" is not a member of enum \"type\" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  route: `jigc doc set-field commit:add-rate-limiter#header/type --value <value>` to correct the value
blocking · schema-conformance.required-slot-present — `commit:add-rate-limiter`: required slot in section `summary` is empty
  route: `jigc doc set-slot commit:add-rate-limiter#summary --from-file -` to fill the empty slot
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  route: if the change is user-facing, record it in this task — `jigc doc create changelog --title Changelog --task add-rate-limiter`, then `jigc doc add-item changelog:changelog#unreleased-changes --title <category> --task add-rate-limiter`; if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// The **no-delta** `jigc task validate` rendered output over a conformant commit doc,
/// byte for byte — the **exit-0** companion to `NO_DELTA_BROKEN_VALIDATE_GOLDEN`.
/// The post-pass must perturb neither the blocking nor the exit-0 validate render.
/// (M43 A14: the staged transient commit doc is file-state-silent — its display was
/// the fictional `docs/commit:<id>.md` working-area key; a transient instance
/// displays at its `<type>:<slug>` identity and mints no `file-state.*` finding.)
///
/// Since M46 Inc 6 / T3 neither render is *empty*: the changelog-gate advisory joined
/// the previewed set, and `single-task` grants that gate, so a task which recorded no
/// entry draws it here — advisory, so the exit code is still 0 and the *"validates
/// clean"* empty-report line is gone (it is not clean, it is unblocked). Both literals
/// were revised in place rather than scoped away: the claim this test makes is that
/// the M6 post-pass moves no byte, and it can only make it over the render the binary
/// actually emits.
const NO_DELTA_CLEAN_VALIDATE_GOLDEN: &str = "\
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  route: if the change is user-facing, record it in this task — `jigc doc create changelog --title Changelog --task add-rate-limiter`, then `jigc doc add-item changelog:changelog#unreleased-changes --title <category> --task add-rate-limiter`; if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-task-lifecycle-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// so the cascade resolves, then `jigc start --workflow single-task "<intent>"` to
/// mint a task and provision its commit doc (post-flip the cascade default is the
/// `router`, so minting goes through Form D). Returns the repo + a `$HOME` temp dir.
fn started_repo(intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", intent])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (repo, home)
}

/// Run `jigc doc <args>` with `cwd = repo`, optionally piping `stdin`.
fn run_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc");
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// Run `jigc task <args>` with `cwd = repo`.
fn run_task(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("task")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run jigc task")
}

/// Fill every author-required field/slot of the provisioned commit doc so a
/// `task validate` over it is clean (no `schema-conformance.*` blocker): the `type`
/// + `scope` header fields and the `summary` + `body` slots.
fn make_commit_conformant(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = run_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert!(
            out.status.success(),
            "set-field {addr}={value} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "gateway");
    set_slot(
        &format!("commit:{task}#summary"),
        b"add a per-client rate limiter\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Throttle abusive clients at the gateway.\n",
    );
}

#[test]
fn task_diff_shows_the_staged_change_and_exits_zero() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // Stage a write into the working area.
    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        None,
    );
    assert!(out.status.success(), "set-field must stage");

    let out = run_task(repo.path(), home.path(), &["diff", task]);
    assert!(
        out.status.success(),
        "`jigc task diff` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains("type: feat"),
        "`task diff` must surface the staged change; got:\n{stdout}"
    );
}

#[test]
fn task_validate_exit_code_tracks_blocking_findings() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // The freshly-provisioned commit doc has empty required slots/fields → blocking
    // `schema-conformance.*`. validate must exit non-zero and list a finding.
    let out = run_task(repo.path(), home.path(), &["validate", task]);
    assert!(
        !out.status.success(),
        "a conformance-broken instance must exit non-zero"
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        rendered.contains("schema-conformance."),
        "validate must list the `schema-conformance.*` finding; got:\n{rendered}"
    );

    // Fill every required field/slot → validate is clean and exits 0.
    make_commit_conformant(repo.path(), home.path(), task);
    let out = run_task(repo.path(), home.path(), &["validate", task]);
    assert!(
        out.status.success(),
        "a conformant instance must exit 0; output:\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// M43 A14, the do-NOT-do-it guard through the real binary: a stale recorded
/// baseline for a staged doc's key mints **no** drift at `task validate` — the
/// staged sweep never keys a staged instance against the record (pre-M43 this exact
/// seed produced a blocking `file-state.hash-matches` at the fictional
/// `docs/commit:<id>.md` key; keying the repo-real destination against the record
/// instead would mint false blocking drift on every in-flight edit of a copied-in
/// committed doc). The conformant task validates clean, exit 0. (The severity
/// post-pass threading through `task validate` stays proven by flow 13's tunable
/// `doc-code` demotion; the `file-state.hash-matches` knob's live surface is the
/// store-scope sweep — `severity_tuning.rs`.)
#[test]
fn task_validate_ignores_a_stale_staged_record_key() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // Fill the commit doc conformant so `schema-conformance.*` is clean.
    make_commit_conformant(repo.path(), home.path(), task);

    // Seed a *wrong* baseline hash at both the legacy staged key and the never-legal
    // repo-real-shaped key — neither may be consulted for a staged instance.
    let rel_key = format!("docs/commit:{task}.md");
    let state_dir = repo.path().join(".jigc").join("state");
    fs::create_dir_all(&state_dir).expect("mk state dir");
    let bogus_hash = "0".repeat(64);
    fs::write(
        state_dir.join("file-state.json"),
        format!("{{\n  \"hashes\": {{\n    \"{rel_key}\": \"{bogus_hash}\"\n  }}\n}}\n"),
    )
    .expect("seed file-state record");

    let out = run_task(repo.path(), home.path(), &["validate", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.status.success(),
        "a stale staged record key must not block validate (staged instances never \
         key against the record); output:\n{rendered}"
    );
    assert!(
        !rendered.contains("file-state."),
        "a transient staged instance is file-state-silent; got:\n{rendered}"
    );
}

#[test]
fn no_delta_validate_render_is_byte_identical_to_the_baseline() {
    // T4 (M6 Increment 1) determinism guard: T3 threaded a real `Resolved` into
    // `task.rs::validate` so the M6 severity post-pass has a cascade to read. Building
    // that `Resolved` must not perturb the no-override render — `design/validation.md`
    // flags the validate path (not only `start_compose`) as the real M6 risk surface
    // (review B2). This repo carries no `validation.*.severity` scalar-set, so the
    // post-pass overrides nothing and the emitted bytes must equal the captured pre-M6
    // baselines — for both the conformance-broken (blocking) and the clean render.
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // The blocking render: the freshly-provisioned commit doc has empty required
    // slots/fields → blocking `schema-conformance.*`.
    let out = run_task(repo.path(), home.path(), &["validate", task]);
    assert!(
        !out.status.success(),
        "the conformance-broken instance must still exit non-zero"
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout, NO_DELTA_BROKEN_VALIDATE_GOLDEN,
        "the no-delta broken validate render must stay byte-identical to the pre-M6 baseline",
    );

    // The clean render: fill every required field/slot → validate is clean (exit 0).
    make_commit_conformant(repo.path(), home.path(), task);
    let out = run_task(repo.path(), home.path(), &["validate", task]);
    assert!(
        out.status.success(),
        "the conformant instance must still exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout, NO_DELTA_CLEAN_VALIDATE_GOLDEN,
        "the no-delta clean validate render must stay byte-identical to the pre-M6 baseline",
    );
}

#[test]
fn task_discard_removes_the_working_area_and_exits_zero() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(area.is_dir(), "the started task working area must exist");

    let out = run_task(repo.path(), home.path(), &["discard", task]);
    assert!(
        out.status.success(),
        "`jigc task discard` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !area.exists(),
        "discard must remove `.jigc/tasks/<id>/`; it still exists at {area:?}"
    );
    // (B3, 2026-07-17 surface review) The ack states what the discard threw away:
    // the provisioned commit doc was a staged edit silently dropped before; now it is
    // enumerated, and marked transient (a commit doc never commits as a file anyway).
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("discarded task add-rate-limiter")
            && stdout.contains("dropped staged edits to: commit:add-rate-limiter (transient)"),
        "the discard ack enumerates the dropped staged docs; got:\n{stdout}"
    );
}
