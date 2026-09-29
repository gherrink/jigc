//! M45 Increment 10 / T2 — the **revise-safe** "safe whether or not it already
//! exists" sentence ported to the three named author steps: `author-vision`
//! (methodology, via `form-vision`), `author-arch-doc` (dev, via
//! `architecture-documentation`), and `author-adr` (dev, via `record-decision`).
//!
//! Finding §62 / §1.5's walk-in: the sentence exists on `author-roadmap` /
//! `-ledger` / `-decisions` but was missing from these three — so an agent
//! re-entering the workflow over an *already-committed* doc could not tell from
//! the composed guidance that `create` is safe (the committed instance is copied
//! in for revise, `existed: true`), the root of the trial's create-refused walk-in.
//!
//! The contract is the EMITTED bytes: this drives the real `jigc` binary and
//! asserts the composed step text of each workflow — the guidance an agent
//! actually reads — carries the revise-safe sentence at its create step. No
//! reconstructed equivalent: the assertion runs over the composed stdout verbatim.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-revise-safe-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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

/// Initialize a real git repo with the `[dev ▸ methodology]` composition, so all
/// three workflows (`form-vision` from methodology, `architecture-documentation`
/// and `record-decision` from the embedded dev base) compose in one repo.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, returning captured output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jigc");
    child.stdin.take();
    child.wait_with_output().expect("wait for jigc")
}

/// Compose `--workflow <wf>` with an intent and return the composed stdout, asserting exit 0.
fn compose(repo: &Path, home: &Path, workflow: &str, intent: &str) -> String {
    let out = jigc(repo, home, &["start", "--workflow", workflow, intent]);
    assert!(
        out.status.success(),
        "`jigc start --workflow {workflow}` must compose; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 composed stdout")
}

const REVISE_SAFE: &str = "safe whether or not it already exists";

#[test]
fn form_vision_create_step_states_the_revise_safe_sentence() {
    let repo = TempDir::new("vision");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let composed = compose(
        repo.path(),
        home.path(),
        "form-vision",
        "form the project vision",
    );
    assert!(
        composed.contains("jigc doc create vision"),
        "the composed form-vision guidance must carry the resolved create line; got:\n{composed}",
    );
    assert!(
        composed.contains(REVISE_SAFE),
        "the composed form-vision create step must state the revise-safe sentence \
         (`{REVISE_SAFE}`) — an already-committed vision is copied in for revise; got:\n{composed}",
    );
}

#[test]
fn architecture_documentation_create_step_states_the_revise_safe_sentence() {
    let repo = TempDir::new("arch");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let composed = compose(
        repo.path(),
        home.path(),
        "architecture-documentation",
        "document the ingest path",
    );
    assert!(
        composed.contains("jigc doc create arch-doc"),
        "the composed architecture-documentation guidance must carry the resolved create line; \
         got:\n{composed}",
    );
    assert!(
        composed.contains(REVISE_SAFE),
        "the composed architecture-documentation create step must state the revise-safe sentence \
         (`{REVISE_SAFE}`) — an already-committed arch-doc is copied in for revise; got:\n{composed}",
    );
}

#[test]
fn record_decision_create_step_states_the_revise_safe_sentence() {
    let repo = TempDir::new("adr");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let composed = compose(
        repo.path(),
        home.path(),
        "record-decision",
        "adopt the blackboard pattern",
    );
    assert!(
        composed.contains("jigc doc create adr"),
        "the composed record-decision guidance must carry the resolved create line; got:\n{composed}",
    );
    assert!(
        composed.contains(REVISE_SAFE),
        "the composed record-decision create step must state the revise-safe sentence \
         (`{REVISE_SAFE}`) — an already-committed ADR is copied in for revise; got:\n{composed}",
    );
}
