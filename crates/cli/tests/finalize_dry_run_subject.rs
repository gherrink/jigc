//! **F-7 — `jigc task finalize --dry-run` forecasts the composed commit subject.**
//!
//! The dry run's whole job is *"tell me what this finalize will do"*, and the single
//! most-read fact about a commit — its subject line — was computed and thrown away:
//! `plan_finalize` renders the staged commit doc into the git message at phase 3
//! (`engine::finalize::FinalizePlan::message`), **before** the `--dry-run` branch returns,
//! and the branch printed the file manifest alone. The landed ack names the subject
//! (`finalized <hash> — <subject>`); the forecast of that same commit did not
//! (RC-m50 `findings-verification.md` → F-7, CONFIRMED · capability gap).
//!
//! The arm below drives the real binary over one authored task with a staged code file
//! and asserts the **emitted bytes** of the forecast carry the *same string* the landed
//! ack later prints — the forecast and the commit are the same render, not two spellings.
//! The commit doc leaves `scope` (optional) **unset** on purpose: a second spelling at the
//! dry-run site would compose `feat(): …`, so the empty-scope shape is what proves the
//! subject comes from `write::render_commit_message` and nowhere else.
//!
//! **Declared bound:** the surface claims the *forecast* — the subject jigc composes and
//! hands to git — never what git ends up with. A `commit-msg` hook may rewrite the message
//! after the forecast printed; that is the same bound the manifest already carries.
//!
//! Drives the built `jigc` binary against a throwaway git repo: the binary path comes from
//! `CARGO_BIN_EXE_jigc`, the pack from `JIGC_PACK_DIR`, and a self-cleaning `TempDir` with
//! an isolated `$HOME` keeps the test off the developer's machine.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-dry-run-subject-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    use std::io::Write;
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", dev_pack())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
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

/// Run `jigc <args>`, asserting exit 0, returning trimmed stdout.
fn ok_stdout(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = run_jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// Mint a `single-task`, author its commit doc **leaving `scope` unset**, and stage one
/// code file. Returns the task id.
fn seed_task(repo: &Path, home: &Path, intent: &str) -> String {
    ok_stdout(repo, home, &["setup"], "jigc setup");
    ok_stdout(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "jigc start",
    );
    let task = intent.replace(' ', "-");
    let out = run_jigc(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        None,
    );
    assert!(out.status.success(), "set-field #type must succeed");
    // `scope` stays UNSET (it is `optional: true`): the renderer omits the parens, so a
    // second spelling at the dry-run site would show up as `feat(): …`.
    let out = run_jigc(
        repo,
        home,
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
        ],
        Some(b"forecast the composed subject\n"),
    );
    assert!(out.status.success(), "set-slot #summary must succeed");
    fs::write(repo.join("src.txt"), "the task's code\n").expect("write src.txt");
    git(repo, &["add", "src.txt"]);
    task
}

/// The `subject` value the dry-run JSON envelope carries.
fn json_subject(stdout: &str) -> String {
    let value: serde_json::Value = serde_json::from_str(stdout).expect("the dry-run JSON parses");
    assert_eq!(
        value.get("dry_run"),
        Some(&serde_json::Value::Bool(true)),
        "the forecast envelope is the dry-run one; got:\n{stdout}",
    );
    value
        .get("subject")
        .and_then(|s| s.as_str())
        .unwrap_or_else(|| {
            panic!("`--format json --dry-run` must carry the forecast subject; got:\n{stdout}")
        })
        .to_string()
}

/// **The arm.** One authored task with a staged code file: the dry run's forecast subject
/// and the subsequent real finalize's landed subject are the SAME string, on both surfaces.
#[test]
fn the_dry_run_forecasts_the_subject_the_commit_lands() {
    let repo = TempDir::new("forecast");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let task = seed_task(repo.path(), home.path(), "forecast subject");
    let head_subject_before = git(repo.path(), &["log", "-1", "--pretty=format:%s"]);

    let text = ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--dry-run"],
        "jigc task finalize --dry-run",
    );
    let json = ok_stdout(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", &task, "--dry-run"],
        "jigc --format json task finalize --dry-run",
    );
    let forecast = json_subject(&json);

    // The empty-scope shape: composed by the renderer, never re-spelled at the dry-run site.
    assert_eq!(
        forecast, "feat: forecast the composed subject",
        "the forecast subject is the renderer's own composition (no `()` for an unset scope)",
    );
    assert!(
        text.contains(&forecast),
        "the agent-text forecast must print the subject it forecasts; stdout:\n{text}",
    );
    // Still a forecast: nothing landed (HEAD is `jigc setup`'s own install commit).
    assert_eq!(
        git(repo.path(), &["log", "-1", "--pretty=format:%s"]),
        head_subject_before,
        "a dry run commits nothing",
    );

    let landed = ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        "jigc task finalize",
    );
    let landed_subject = git(repo.path(), &["log", "-1", "--pretty=format:%s"]);
    assert_eq!(
        forecast, landed_subject,
        "the forecast and the landed subject are the SAME string; landed ack:\n{landed}",
    );
    assert!(
        landed.contains(&format!("— {landed_subject}")),
        "the landed ack names that same subject; stdout:\n{landed}",
    );
}
