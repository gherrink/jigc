//! End-to-end integration test for `jigc start "<intent>"` composition.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts
//! the Increment-3 deliverable (`implementation/roadmap.md` → Increment 3): an
//! intent-bearing `jigc start` mints the task, runs the `workflow-refs` gate,
//! composes `single-task` with `{{task.intent}}` = the intent, and prints the
//! composed four-class view through the selected format.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init` (composition mints,
//! which reads HEAD), and a self-cleaning `TempDir` keeps the test off the
//! developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-start-compose-{tag}-{}-{:?}",
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

const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// Initialize a real git repo with one commit (composition mints, which reads
/// HEAD via `git rev-parse`), and create the `.jigc/config/` project layer so the
/// cascade resolves.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
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
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc start <args>` with `cwd = repo` and `$HOME = home`.
fn run_start(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("start");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn start_with_intent_mints_and_composes_the_resolved_view() {
    let repo = TempDir::new("compose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The task minted under .jigc/tasks/<slug>/ with its base pin.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("add-rate-limiter");
    assert!(
        task_dir.join("base.json").is_file(),
        "minting must open .jigc/tasks/add-rate-limiter/ with a base pin",
    );

    // The composed view embeds the resolved intent (the locate step's
    // {{task.intent}}), a Run: command line, the <<author:>> directive, and the
    // routing footer.
    assert!(
        stdout.contains("add rate limiter"),
        "the composed view must embed the resolved intent; got:\n{stdout}",
    );
    assert!(
        stdout.lines().any(|l| l.starts_with("Run: ")),
        "the composed view must carry a `Run:` command line; got:\n{stdout}",
    );
    assert!(
        stdout.contains("<<author:"),
        "the composed view must carry the `<<author:` directive; got:\n{stdout}",
    );
    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "agent-text composition must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn start_with_intent_json_format_carries_no_footer() {
    let repo = TempDir::new("compose-json");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(
        repo.path(),
        home.path(),
        &["--format", "json", "add rate limiter"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`--format json` composition must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !stdout.contains(ROUTING_FOOTER),
        "JSON composition output must carry no routing footer; got:\n{stdout}",
    );
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("must be valid JSON ({e}); got:\n{stdout}"));
    assert!(
        value["text"]
            .as_str()
            .is_some_and(|t| t.contains("add rate limiter")),
        "the JSON view's `text` must carry the composed workflow; got:\n{stdout}",
    );
}

#[test]
fn form_d_named_workflow_mints_and_composes_through_dispatch() {
    let repo = TempDir::new("form-d");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `--workflow single-task <intent>`: the explicit-selection front door. The
    // embedded pack's `single-task` is `creates-task: true`, so Form D mints and
    // composes it end-to-end through the dispatch arm.
    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "single-task", "add rate limiter"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow single-task \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The named creates-task workflow minted under .jigc/tasks/<slug>/.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("add-rate-limiter");
    assert!(
        task_dir.join("base.json").is_file(),
        "Form D over a creates-task workflow must open .jigc/tasks/add-rate-limiter/ with a base pin",
    );

    // The named workflow composed end-to-end, embedding the resolved intent and
    // ending with the routing footer (the same composed view `--format` renders).
    assert!(
        stdout.contains("add rate limiter"),
        "the composed view must embed the resolved intent; got:\n{stdout}",
    );
    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "Form-D agent-text composition must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn form_d_quick_fix_mints_and_composes_without_adr_or_supersedes() {
    let repo = TempDir::new("form-d-quick-fix");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `--workflow quick-fix <intent>`: the second selectable work-workflow. It is
    // `creates-task: true`, so Form D mints + composes it — but it is commit-only
    // (`allows-create: []`), so its composed text must carry no ADR/create
    // affordance and no superseded-context line, materially differing from
    // `single-task`.
    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "quick-fix", "fix typo in readme"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow quick-fix \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The work-workflow minted under .jigc/tasks/<slug>/.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("fix-typo-in-readme");
    assert!(
        task_dir.join("base.json").is_file(),
        "quick-fix is creates-task: true, so it must open .jigc/tasks/fix-typo-in-readme/ with a base pin",
    );

    // It composed end-to-end, embedding the resolved intent.
    assert!(
        stdout.contains("fix typo in readme"),
        "the composed view must embed the resolved intent; got:\n{stdout}",
    );

    // Materially different from single-task: no ADR/create affordance ...
    assert!(
        !stdout.contains("create-adr") && !stdout.to_lowercase().contains("adr"),
        "quick-fix is commit-only — its composed view must carry no ADR/create affordance; got:\n{stdout}",
    );
    // ... and no superseded-context line.
    assert!(
        !stdout.to_lowercase().contains("supersede"),
        "quick-fix must carry no superseded-context line; got:\n{stdout}",
    );
}

#[test]
fn form_d_unknown_workflow_rejects_before_minting() {
    let repo = TempDir::new("form-d-unknown");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "does-not-exist", "add rate limiter"],
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    assert!(
        !out.status.success(),
        "an unknown `--workflow` id must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("does-not-exist") && stderr.contains("route:"),
        "the rejection must name the unknown id and carry a route; got:\n{stderr}",
    );
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "the rejection must precede minting — no .jigc/tasks/ dir may be created",
    );
}

#[test]
fn serial_re_run_of_the_same_intent_blocks() {
    let repo = TempDir::new("compose-collision");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let first = run_start(repo.path(), home.path(), &["add rate limiter"]);
    assert!(
        first.status.success(),
        "the first mint+compose must succeed"
    );

    let second = run_start(repo.path(), home.path(), &["add rate limiter"]);
    assert!(
        !second.status.success(),
        "a serial re-run of the same intent must exit non-zero (serial collision)",
    );
    let stderr = String::from_utf8(second.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("add-rate-limiter"),
        "the serial-collision block must name the task; got:\n{stderr}",
    );
}
