//! Binary-level integration test for `jigc doc set-field <addr> --unset` (M41 Increment
//! 7, V5 — `design/write-commands.md` → the `--unset` verb). Drives the real binary:
//!
//! - an optional header scalar (`adr#cites-code`) clears cleanly — the `cites-code:`
//!   line is gone, the doc still re-conforms (a follow-up write over the now-absent
//!   field succeeds), and the buffer is byte-identical to its pre-`cites-code` state
//!   (byte-stable clear);
//! - `#status --unset` on a **defaulted** enum blocks with `write.unset-ineligible` +
//!   a route;
//! - `#schema-version --unset` on the engine-injected stamp blocks (unsetting it would
//!   corrupt the freeze gate);
//! - `--value ""` (the clear footgun) rejects with a route that now names
//!   `jigc doc set-field <addr> --unset`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-set-field-unset-{tag}-{}-{:?}",
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

/// Run `git` in `repo`, asserting success.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    command.output().expect("run the jigc binary")
}

/// Initialize a repo + project layer, mint a task, create an ADR. Returns
/// (repo, home, task-id, adr-slug).
fn repo_with_created_adr() -> (TempDir, TempDir, &'static str, &'static str) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let started = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "clear cites-code"],
    );
    assert!(
        started.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&started.stderr)
    );

    let task = "clear-cites-code";
    let created = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Anchor decision",
            "--task",
            task,
        ],
    );
    assert!(
        created.status.success(),
        "`jigc doc create adr` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr)
    );
    (repo, home, task, "anchor-decision")
}

/// The staged ADR instance path in the task's `docs/` area.
fn staged_adr(repo: &Path, task: &str, slug: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"))
}

/// Set `cites-code` to `value` on the staged ADR, asserting success.
fn set_cites(repo: &Path, home: &Path, task: &str, slug: &str, value: &str) {
    let out = run(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/cites-code"),
            "--value",
            value,
            "--task",
            task,
        ],
    );
    assert!(
        out.status.success(),
        "`set-field cites-code` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn unset_clears_an_optional_scalar_byte_stable_and_reconforms() {
    let (repo, home, task, slug) = repo_with_created_adr();
    let staged = staged_adr(repo.path(), task, slug);

    // Snapshot the buffer BEFORE cites-code is ever added — the byte-stable clear must
    // return to exactly this state.
    let before = fs::read_to_string(&staged).expect("read staged adr");
    assert!(
        !before.contains("cites-code"),
        "precondition: no cites-code"
    );

    set_cites(
        repo.path(),
        home.path(),
        task,
        slug,
        "src/engine/write.rs#unset_field",
    );
    let with_cites = fs::read_to_string(&staged).expect("read after set");
    assert!(with_cites.contains("cites-code:"), "cites-code present now");

    // `--unset` clears it; the JSON ack carries `unset: true`.
    let out = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/cites-code"),
            "--unset",
            "--task",
            task,
            "--format",
            "json",
        ],
    );
    assert!(
        out.status.success(),
        "`set-field cites-code --unset` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"unset\": true"),
        "the JSON ack names the clear; got:\n{stdout}"
    );

    // The `cites-code:` line is gone, and the buffer is byte-identical to its
    // pre-cites-code state (a byte-stable clear, not a re-canonicalized rewrite).
    let after = fs::read_to_string(&staged).expect("read after unset");
    assert!(
        !after.contains("cites-code"),
        "the cleared line is gone:\n{after}"
    );
    assert_eq!(
        after, before,
        "the clear must be byte-stable — identical to the pre-cites-code buffer"
    );

    // Re-conforms: a fresh write over the now-absent field succeeds (proves the buffer
    // still parses against the schema after the removal).
    set_cites(
        repo.path(),
        home.path(),
        task,
        slug,
        "src/engine/write.rs#unset_field",
    );
    assert!(
        fs::read_to_string(&staged).unwrap().contains("cites-code:"),
        "the field re-populates cleanly after the clear"
    );
}

#[test]
fn unset_a_defaulted_field_blocks_with_a_route() {
    let (repo, home, task, slug) = repo_with_created_adr();

    // `status` carries `default: proposed` — clearing it drops a guaranteed value.
    let out = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/status"),
            "--unset",
            "--task",
            task,
        ],
    );
    assert!(
        !out.status.success(),
        "unsetting a defaulted field must block (non-zero exit)"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("write.unset-ineligible"),
        "the block carries the ineligible code; got:\n{stderr}"
    );
    assert!(
        stderr.contains("route:"),
        "the ineligible block carries a route; got:\n{stderr}"
    );
}

#[test]
fn unset_the_schema_version_stamp_blocks() {
    let (repo, home, task, slug) = repo_with_created_adr();

    // The engine-injected `schema-version` stamp is `set:`-derived — unsetting it would
    // corrupt the freeze gate, so it is refused.
    let out = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/schema-version"),
            "--unset",
            "--task",
            task,
        ],
    );
    assert!(
        !out.status.success(),
        "unsetting the schema-version stamp must block"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("write.unset-ineligible"),
        "the stamp block carries the ineligible code; got:\n{stderr}"
    );
}

#[test]
fn empty_value_reject_routes_to_unset() {
    let (repo, home, task, slug) = repo_with_created_adr();

    // `--value ""` is the clear footgun: it rejects (the opaque floor forbids an empty
    // value), and the route now names `--unset`.
    let out = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/cites-code"),
            "--value",
            "",
            "--task",
            task,
        ],
    );
    assert!(
        !out.status.success(),
        "an empty `--value` must reject (non-zero exit)"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--unset"),
        "the empty-value reject's route names `--unset`; got:\n{stderr}"
    );
}
