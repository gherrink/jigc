//! M42 Increment 11, T2 — the changelog paragraph becomes its own step, included
//! only where the create-gate is granted, and the false "lists the gates" sentence
//! is retired.
//!
//! The rc.5 adoption trial found `step:implement` instructing
//! `jigc doc create changelog` under **every** workflow that includes it — while
//! only `single-task` grants the `{type: changelog, as: change}` create-gate. Under
//! `implement-from-spec` and `sub-task` the binary **refuses** the very command the
//! composed prompt tells the agent to run (`create.gate-blocked`), and the prompt's
//! one discovery hint — *"bare `jigc start` lists the gates it grants"* — is false
//! (bare `start` prints the workflow catalog and names no gate anywhere).
//!
//! The fix is **step-subset inclusion** ([workflow-dialect.md](../../../design/workflow-dialect.md)),
//! never a conditional (the dialect bars them): the paragraph moves to
//! `step:record-changelog`, included by `single-task` alone.
//!
//! Proven on the **emitted bytes** through the real binary against the shipped dev
//! pack (`JIGC_PACK_DIR` = the tree that ships):
//!   - `single-task` composes exactly **one** `jigc doc create changelog` line, and
//!     that emitted line — run **verbatim** — is *admitted* by the gate (exit 0);
//!   - `implement-from-spec` and `sub-task` compose **zero** such lines (the
//!     omitting contexts: the step is absent where the gate is absent);
//!   - no composed output — and no source byte of **either** shipped pack — carries
//!     the retired `lists the gates` sentence.
//!
//! The gate's own admit/refuse behaviour is unchanged and stays pinned by
//! `single_task_changelog_gate.rs`; this suite pins *what the pack tells the agent
//! to run*.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-changelog-step-subset-{tag}-{}-{:?}",
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

/// The workspace root — both shipped pack trees hang off it.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root is two levels above crates/cli")
        .to_path_buf()
}

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
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

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

/// The stdout of a successful `jigc` invocation, or a panic carrying both streams.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// Compose a workflow through the real binary: `jigc start --workflow <w> <intent>`.
fn compose(repo: &Path, home: &Path, pack: &Path, workflow: &str, intent: &str) -> String {
    ok_stdout(
        run_jigc(repo, home, pack, &["start", "--workflow", workflow, intent]),
        &format!("jigc start --workflow {workflow}"),
    )
}

/// Every emitted line that instructs a `jigc doc create changelog`, verbatim.
fn changelog_create_lines(composed: &str) -> Vec<&str> {
    composed
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("jigc doc create changelog"))
        .collect()
}

/// Walk a pack tree, yielding every file's text.
fn pack_files(root: &Path, out: &mut Vec<(PathBuf, String)>) {
    for entry in fs::read_dir(root).unwrap_or_else(|e| panic!("read {root:?}: {e}")) {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            pack_files(&path, out);
        } else if let Ok(text) = fs::read_to_string(&path) {
            out.push((path, text));
        }
    }
}

/// The core done-criterion: the changelog instruction composes **only** where the
/// create-gate is granted, and the emitted line is the one the binary admits.
#[test]
fn the_changelog_instruction_composes_only_where_the_gate_is_granted() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "jigc setup",
    );

    // ── The granting context: `single-task` alone carries `{type: changelog, as: change}`.
    let single = compose(
        repo.path(),
        home.path(),
        &pack,
        "single-task",
        "add a rate limiter",
    );
    let lines = changelog_create_lines(&single);
    assert_eq!(
        lines.len(),
        1,
        "`single-task` grants the changelog gate — it must compose exactly one \
         `jigc doc create changelog` line; got {lines:?} in:\n{single}",
    );

    // The EMITTED line is the contract: run it verbatim. A prompt whose literal
    // command the binary refuses is the defect this task exists to kill.
    let emitted = lines[0];
    let argv: Vec<&str> = emitted.split_whitespace().collect();
    assert_eq!(argv[0], "jigc", "the emitted line invokes the binary");
    let created = run_jigc(repo.path(), home.path(), &pack, &argv[1..]);
    assert!(
        created.status.success(),
        "the emitted line `{emitted}` must be ADMITTED by the gate it is composed \
         under; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );

    // ── The omitting contexts: neither workflow grants the gate, so neither may
    // instruct the create at all (today both do — and the binary refuses it).
    for workflow in ["implement-from-spec", "sub-task"] {
        let composed = compose(
            repo.path(),
            home.path(),
            &pack,
            workflow,
            &format!("work the {workflow} arm"),
        );
        let lines = changelog_create_lines(&composed);
        assert!(
            lines.is_empty(),
            "`{workflow}` does not grant the changelog create-gate — it must compose \
             ZERO `jigc doc create changelog` lines (the binary refuses them with \
             `create.gate-blocked`); got {lines:?} in:\n{composed}",
        );
    }
}

/// The false discovery hint is **retired**, not rewritten: bare `jigc start` prints
/// the workflow catalog and names no gate anywhere, and a step included only where
/// the gate exists needs no hint. Pinned on the emitted bytes of the three
/// `step:implement` workflows *and* exhaustively over both shipped pack trees — the
/// only bytes from which the sentence could ever reach a composed view.
#[test]
fn no_composed_workflow_and_no_pack_source_claims_start_lists_the_gates() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "jigc setup",
    );

    for workflow in ["single-task", "implement-from-spec", "sub-task"] {
        let composed = compose(
            repo.path(),
            home.path(),
            &pack,
            workflow,
            &format!("compose the {workflow} view"),
        );
        assert!(
            !composed.contains("lists the gates"),
            "`{workflow}` must not compose the false gates-discovery sentence; got:\n{composed}",
        );
    }

    let root = workspace_root();
    let mut files = Vec::new();
    pack_files(&root.join("crates").join("cli").join("pack"), &mut files);
    pack_files(&root.join("packs").join("methodology"), &mut files);
    let offenders: Vec<&PathBuf> = files
        .iter()
        .filter(|(_, text)| text.contains("lists the gates"))
        .map(|(path, _)| path)
        .collect();
    assert!(
        offenders.is_empty(),
        "no shipped pack source may claim bare `jigc start` lists the gates it grants \
         (it does not); offenders: {offenders:?}",
    );
}
