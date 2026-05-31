//! End-to-end integration test for the `jigc doc <verb> <addr>` write surface.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo with a
//! started task, exercising the MVP write loop's primitives
//! (`design/write-commands.md` → The verbs / Content handoff / Worked example):
//! `set-field <addr> --value` (inline, adjudicated) and `set-slot <addr>
//! --from-file -` (prose via stdin). A malformed `--value` blocks with the typed
//! finding message + route on stderr and a non-zero exit.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init` (`jigc start` reads
//! HEAD), and a self-cleaning `TempDir` keeps the test off the developer's repo.
//! Staged bytes are asserted by reading the working-area instance directly (the
//! sibling `jigc task diff` surface lands in a later task; the staged buffer on
//! disk *is* what `task diff` would render).

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
            "jigc-doc-write-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves, then `jigc start "<intent>"` to mint a task and
/// provision its commit doc. Returns the repo + a `$HOME` temp dir.
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
        .args(["start", intent])
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

/// Read the staged commit instance from the task working area.
fn staged_commit(repo: &Path, task: &str) -> String {
    let path = repo
        .path_join(task)
        .unwrap_or_else(|| panic!("compute staged path"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// Helper trait sugar: the staged commit-doc path for a task.
trait PathJoin {
    fn path_join(&self, task: &str) -> Option<PathBuf>;
}
impl PathJoin for Path {
    fn path_join(&self, task: &str) -> Option<PathBuf> {
        Some(
            self.join(".jigc")
                .join("tasks")
                .join(task)
                .join("docs")
                .join(format!("commit:{task}.md")),
        )
    }
}

#[test]
fn set_field_inline_stages_the_field_value() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

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
    assert!(
        out.status.success(),
        "`set-field ... --value feat` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let staged = staged_commit(repo.path(), task);
    assert!(
        staged.contains("type: feat"),
        "the staged commit doc must carry `type: feat`; got:\n{staged}"
    );
}

#[test]
fn set_slot_from_stdin_stages_the_prose() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";
    let prose = b"Add a per-client rate limiter at the gateway.\n";

    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
        ],
        Some(prose),
    );
    assert!(
        out.status.success(),
        "`set-slot ... --from-file -` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let staged = staged_commit(repo.path(), task);
    assert!(
        staged.contains("Add a per-client rate limiter at the gateway."),
        "the staged commit doc must carry the piped slot prose; got:\n{staged}"
    );
}

#[test]
fn malformed_field_value_blocks_with_a_routed_finding() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "not-a-member",
        ],
        None,
    );
    assert!(
        !out.status.success(),
        "a non-member enum value must exit non-zero"
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("not-a-member") || stderr.to_lowercase().contains("enum"),
        "the block must name the rejected value or the enum constraint; got:\n{stderr}"
    );
    assert!(
        stderr.contains("route:"),
        "the block must carry a route directing the agent's next action; got:\n{stderr}"
    );

    // The malformed write must NOT have mutated the staged doc (no `type: not-a-member`).
    let staged = staged_commit(repo.path(), task);
    assert!(
        !staged.contains("not-a-member"),
        "a rejected write must persist nothing; got:\n{staged}"
    );
}
