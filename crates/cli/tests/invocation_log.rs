//! Acceptance — the M36 **opt-in invocation log** (`design/measurement.md` → The
//! in-repo invocation log; `roadmap.md` → M36 Increment 1, bullet 3).
//!
//! A `bool` cascade knob `invocation-log` (default **OFF**) turns on an append-only JSONL
//! log at `.jigc/logs/invocations.jsonl` — one record `{timestamp, argv, exit_code,
//! duration_ms, finding_codes}` per `jigc` invocation, written by a `main()` wrapper that
//! straddles `Cli::try_parse()` (so a clap-rejected usage error is logged too) and resolves
//! the knob from the project cascade independent of argv. Knob OFF writes nothing; a run
//! outside a jigc repo writes nothing; `.jigc/logs/` is git-ignored while `.jigc/version`
//! (the T2 provenance stamp) stays committed.
//!
//! This drives the built `jigc` binary end-to-end with **no** `JIGC_PACK_DIR` (the real
//! embedded pack) in a self-cleaning `TempDir`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-invocation-log-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home` against the real embedded pack.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Make `root` a real git repo with identity, then run `jigc setup` over it.
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

/// Turn the invocation-log knob ON in the project cascade.
fn enable_log(repo: &Path, home: &Path) {
    let out = jigc(repo, home, &["config", "set", "invocation-log", "true"]);
    assert!(
        out.status.success(),
        "`jigc config set invocation-log true` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The parsed JSONL records at `.jigc/logs/invocations.jsonl` (empty when the file is absent).
fn log_records(repo: &Path) -> Vec<serde_json::Value> {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    match fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).expect("each log line is valid JSON"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// The last record whose `argv` array contains `needle`.
fn record_with_arg<'a>(
    records: &'a [serde_json::Value],
    needle: &str,
) -> Option<&'a serde_json::Value> {
    records.iter().rev().find(|r| {
        r["argv"]
            .as_array()
            .is_some_and(|a| a.iter().any(|v| v.as_str() == Some(needle)))
    })
}

/// Knob ON: a successful invocation appends exactly one well-formed JSONL record carrying
/// every field of the record shape.
#[test]
fn knob_on_appends_one_well_formed_record() {
    let repo = TempDir::new("on");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    let before = log_records(repo.path()).len();
    let out = jigc(repo.path(), home.path(), &["describe"]);
    assert!(out.status.success(), "`jigc describe` must succeed");

    let records = log_records(repo.path());
    assert_eq!(
        records.len(),
        before + 1,
        "exactly one record is appended per invocation; got {records:#?}",
    );
    let rec = record_with_arg(&records, "describe").expect("the describe invocation is logged");
    assert!(
        rec["timestamp"].as_str().is_some(),
        "record carries a string timestamp; got {rec}",
    );
    assert_eq!(rec["exit_code"].as_u64(), Some(0), "describe exits 0");
    assert!(
        rec["duration_ms"].as_u64().is_some(),
        "record carries a numeric duration_ms; got {rec}",
    );
    assert!(
        rec["finding_codes"].as_array().is_some(),
        "record carries a finding_codes array; got {rec}",
    );
}

/// Knob ON: a clap-rejected invocation (unknown subcommand → exit 2) is logged too — the
/// wrapper straddles `Cli::try_parse()`, so the usage error never escapes uncaptured.
#[test]
fn clap_usage_error_is_logged() {
    let repo = TempDir::new("usage");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["frobnicate-nonexistent"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "an unknown subcommand is a clap usage error (exit 2)",
    );

    let records = log_records(repo.path());
    let rec = record_with_arg(&records, "frobnicate-nonexistent")
        .expect("the clap-rejected invocation is logged");
    assert_eq!(
        rec["exit_code"].as_u64(),
        Some(2),
        "the logged usage error records exit_code 2; got {rec}",
    );
}

/// Knob ON: a failing verb records its finding codes — a validation-blocked `task validate`
/// (exit 3, unfilled required commit slots) logs a record whose `finding_codes` is non-empty.
#[test]
fn failing_verb_records_finding_codes() {
    let repo = TempDir::new("findings");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    // Mint a task; its provisioned commit form has empty required slots/fields.
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "block me"],
    );
    assert!(
        out.status.success(),
        "`jigc start` must mint the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let out = jigc(repo.path(), home.path(), &["task", "validate", "block-me"]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "an unfilled task validates as blocking (exit 3); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    let records = log_records(repo.path());
    let rec = records
        .iter()
        .rev()
        .find(|r| r["exit_code"].as_u64() == Some(3))
        .expect("the blocked validate is logged with exit_code 3");
    let codes = rec["finding_codes"]
        .as_array()
        .expect("finding_codes is an array");
    assert!(
        !codes.is_empty(),
        "a failing verb records its finding_codes; got {rec}",
    );
}

/// Knob OFF (the default): nothing is written — no `.jigc/logs/` file appears.
#[test]
fn knob_off_writes_nothing() {
    let repo = TempDir::new("off");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["describe"]);
    assert!(out.status.success(), "`jigc describe` must succeed");

    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("logs")
            .join("invocations.jsonl")
            .exists(),
        "the default (knob OFF) writes no invocation log",
    );
}

/// Outside a jigc repo (a git repo with no `.jigc/` project layer): the wrapper no-ops —
/// nothing is written even though the knob's default read would be attempted.
#[test]
fn outside_jigc_repo_writes_nothing() {
    let repo = TempDir::new("bare");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);

    let out = jigc(repo.path(), home.path(), &["describe"]);
    // describe errors without a project layer, but the wrapper must still no-op silently.
    let _ = out;
    assert!(
        !repo.path().join(".jigc").join("logs").exists(),
        "a run outside a jigc project layer writes no invocation log",
    );
}

/// The store hygiene split: `.jigc/logs/` is git-ignored (never committed), while the T2
/// `.jigc/version` provenance stamp stays committed.
#[test]
fn logs_ignored_version_committed() {
    let repo = TempDir::new("ignore");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());
    let _ = jigc(repo.path(), home.path(), &["describe"]);

    // `.jigc/logs/` is check-ignore-ignored (git prints the path + exits 0).
    let out = Command::new("git")
        .args(["check-ignore", ".jigc/logs/invocations.jsonl"])
        .current_dir(repo.path())
        .output()
        .expect("run git check-ignore");
    assert!(
        out.status.success(),
        "`.jigc/logs/` must be git-ignored; check-ignore stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );

    // `.jigc/version` is NOT ignored (exit 1) and IS tracked.
    let out = Command::new("git")
        .args(["check-ignore", ".jigc/version"])
        .current_dir(repo.path())
        .output()
        .expect("run git check-ignore");
    assert!(
        !out.status.success(),
        "`.jigc/version` must NOT be git-ignored (it is committed provenance)",
    );
    let tracked = git(repo.path(), &["ls-files"]);
    assert!(
        tracked.lines().any(|p| p == ".jigc/version"),
        "`.jigc/version` stays committed; tracked:\n{tracked}",
    );
}
