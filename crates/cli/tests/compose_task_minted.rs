//! M42 Increment 12, T2 — a composed task announces the id it minted.
//!
//! Measured before the fix: a work-minting `jigc start` reveals its task id in **agent
//! text** only *inside* a `Run:` command string (`Run: \`jigc task finalize <id>\``) —
//! the id an agent needs for every subsequent call is buried in a step body's prose. The
//! structural surface exists (`--format json` carries `task`, M41-V2), but the agent's
//! *reading* surface never states it. The composed text opens straight at the first
//! step's instruction.
//!
//! The fix is **presentation**, in the CLI frontend: a `task minted: <id>` header line
//! joins the [`ROUTING_FOOTER`](../src/render.rs) / `create-gates:` mold on agent/human
//! text — the frontend-appended presentation lines the JSON projection never carries.
//!
//! The header states a **mint**, so it appears exactly where an invocation *minted* the
//! id: a work-minting `start`. A `creates-task: false` compose (the router default) mints
//! nothing and carries no line; a **resume** (`jigc start --task <id>`, whose `task` key
//! is the *given* id, re-composed) minted nothing *in this invocation* either, so it
//! carries no line — announcing a mint that did not happen would be exactly the class of
//! false statement this increment exists to retire.
//!
//! The pinned composed-output JSON contract (`design/command-output-contract.md` §1)
//! stays **exactly** `{task, text}` — M42 revises no output contract, so the header line
//! must never reach the JSON projection, in either the key set or the `text` bytes.
//!
//! Everything here drives the **real binary** against the shipped dev pack, and asserts
//! the **emitted bytes** (`JIGC_PACK_DIR` = the tree that ships).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The one-line routing footer every agent-text composed view ends with
/// (`crates/cli/src/render.rs` → `ROUTING_FOOTER`, verbatim).
const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-compose-task-minted-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
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

/// A fresh repo with the adapter installed — the composing ground for every arm.
fn ground(repo: &Path, home: &Path, pack: &Path) {
    init_repo(repo);
    ok_stdout(run_jigc(repo, home, pack, &["setup"]), "jigc setup");
}

/// The `task minted:` header lines an emitted view carries (there must be at most one).
fn minted_lines(composed: &str) -> Vec<&str> {
    composed
        .lines()
        .filter(|l| l.trim_start().starts_with("task minted:"))
        .collect()
}

/// The minting arm: a work-minting `jigc start --workflow single-task "<intent>"` opens
/// its agent text with the id it just minted, on its own header line — the id every
/// subsequent call needs, stated rather than buried inside a `Run:` command string.
#[test]
fn a_work_minting_compose_opens_with_the_id_it_minted() {
    let repo = TempDir::new("mint");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    let composed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "--format",
                "agent",
                "start",
                "--workflow",
                "single-task",
                "add a rate limiter",
            ],
        ),
        "jigc start --workflow single-task",
    );

    assert_eq!(
        composed.lines().next(),
        Some("task minted: add-a-rate-limiter"),
        "a work-minting compose must open with the id it minted — today the id reaches \
         agent text only inside a `Run:` command string; got:\n{composed}",
    );
    assert_eq!(
        minted_lines(&composed).len(),
        1,
        "exactly one header line; got:\n{composed}",
    );
    // The header joins the presentation mold — it does not displace it.
    assert!(
        composed.trim_end().ends_with(ROUTING_FOOTER),
        "the composed view still ends with the routing footer; got:\n{composed}",
    );
    assert!(
        composed.contains("Run: `jigc task finalize add-a-rate-limiter`"),
        "the step bodies still compose below the header — and the id the header states is \
         the id they address; got:\n{composed}",
    );

    // Human carries the header too (a fresh intent — the slug above is now active).
    let human = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "--format",
                "human",
                "start",
                "--workflow",
                "single-task",
                "add a cache layer",
            ],
        ),
        "jigc --format human start --workflow single-task",
    );
    assert_eq!(
        human.lines().next(),
        Some("task minted: add-a-cache-layer"),
        "human carries the header too; got:\n{human}",
    );
}

/// The omitting arm: the cascade default is the **router** (`creates-task: false`) — it
/// mints nothing, so it announces nothing. The feature composed into a context that has
/// no mint is inert, never an error and never a `task minted: none`.
#[test]
fn a_router_compose_mints_nothing_and_announces_nothing() {
    let repo = TempDir::new("router");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    let composed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["--format", "agent", "start", "add a rate limiter"],
        ),
        "jigc start \"<intent>\"",
    );

    assert!(
        minted_lines(&composed).is_empty(),
        "the router mints no task — it must print NO header line (omitted, never \
         `none`); got:\n{composed}",
    );
    assert!(
        composed.trim_end().ends_with(ROUTING_FOOTER),
        "the composed view ends with the routing footer; got:\n{composed}",
    );

    // The router's `task` key is `null` — the structural fact the header reads.
    let json = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["--format", "json", "start", "add a rate limiter"],
        ),
        "jigc --format json start \"<intent>\"",
    );
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    assert!(
        value["task"].is_null(),
        "the router is `creates-task: false` — it mints no task; got:\n{json}",
    );
}

/// The honest-scope arm: a **resume** (`jigc start --task <id>`) re-composes a task that
/// was minted by an *earlier* invocation. Its `task` key is the given id, but this run
/// minted nothing — so it carries no header. Announcing a mint that did not happen is the
/// class of false statement this increment retires; the id is not "minted" twice.
#[test]
fn a_resume_re_composes_and_announces_no_mint() {
    let repo = TempDir::new("resume");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["start", "--workflow", "single-task", "add a rate limiter"],
        ),
        "jigc start --workflow single-task",
    );

    let resumed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["--format", "agent", "start", "--task", "add-a-rate-limiter"],
        ),
        "jigc start --task add-a-rate-limiter",
    );

    assert!(
        minted_lines(&resumed).is_empty(),
        "a resume mints nothing — it must not claim `task minted:`; got:\n{resumed}",
    );
    assert!(
        resumed.trim_end().ends_with(ROUTING_FOOTER),
        "the resumed view ends with the routing footer; got:\n{resumed}",
    );
}

/// The pinned contract is untouched: `--format json` deserializes to **exactly**
/// `{task, text}` on the minting **and** the router arm — no new key — and the header
/// line never leaks into the `text` bytes (it is appended in the frontend, never in the
/// JSON projection).
#[test]
fn the_json_contract_stays_exactly_task_and_text_on_both_arms() {
    let repo = TempDir::new("json");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    let arms: [(&str, Vec<&str>); 2] = [
        (
            "single-task",
            vec![
                "--format",
                "json",
                "start",
                "--workflow",
                "single-task",
                "add a rate limiter",
            ],
        ),
        (
            "router",
            vec!["--format", "json", "start", "route me somewhere"],
        ),
    ];

    for (arm, args) in arms {
        let out = ok_stdout(
            run_jigc(repo.path(), home.path(), &pack, &args),
            &format!("jigc --format json start ({arm})"),
        );
        let value: serde_json::Value = serde_json::from_str(&out)
            .unwrap_or_else(|e| panic!("`{arm}` --format json parses: {e}; got:\n{out}"));
        let object = value
            .as_object()
            .unwrap_or_else(|| panic!("`{arm}` --format json is an object; got:\n{out}"));

        let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            ["task", "text"],
            "the composed-output contract is pinned at exactly `{{task, text}}` \
             (`design/command-output-contract.md` §1) — the header line is presentation \
             and adds NO key; `{arm}` emitted:\n{out}",
        );
        let text = object["text"].as_str().expect("`text` is a string");
        assert!(
            !text.contains("task minted:"),
            "the header is appended in the CLI frontend, never in the JSON projection — \
             `{arm}`'s `text` must not carry it; got:\n{text}",
        );
        assert!(
            !text.contains(ROUTING_FOOTER),
            "JSON carries no routing footer either (the mold this header joins); got:\n{text}",
        );
    }
}
