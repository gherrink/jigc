//! M42 Increment 11, T6 — a composed task names the gates it grants.
//!
//! Measured before the fix: **no** compose surface and **no** `describe` surface names
//! a task's create-gates anywhere. The only surface in the entire binary that does is
//! the `create.gate-blocked` **refusal** (`allowed doctypes: [adr]`) — an agent learns
//! its gates by tripping one.
//!
//! The fix is **presentation**, in the CLI frontend: a `create-gates:` line joins the
//! [`ROUTING_FOOTER`](../src/render.rs) mold on agent/human text, immediately before the
//! footer. It rides the composing workflow's `allows-create` declaration, so a gate-less
//! workflow (`quick-fix`, `allows-create: []`) prints **no** line at all — omitted, never
//! `none` (the context-scoped floor: the feature composed into an omitting context is
//! *inert*, never an error).
//!
//! The pinned composed-output JSON contract (`design/command-output-contract.md` §1)
//! stays **exactly** `{task, text}` — M42 revises no output contract, so the presentation
//! line must never reach the JSON projection, in either the key set or the `text` bytes.
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
            "jigc-compose-create-gates-{tag}-{}-{:?}",
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

/// Compose a workflow through the real binary: `jigc start --workflow <w> "<intent>"`,
/// in the given `--format`.
fn compose(
    repo: &Path,
    home: &Path,
    pack: &Path,
    workflow: &str,
    intent: &str,
    fmt: &str,
) -> String {
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["--format", fmt, "start", "--workflow", workflow, intent],
        ),
        &format!("jigc --format {fmt} start --workflow {workflow}"),
    )
}

/// A fresh repo with the adapter installed — the composing ground for both arms.
fn ground(repo: &Path, home: &Path, pack: &Path) {
    init_repo(repo);
    ok_stdout(run_jigc(repo, home, pack, &["setup"]), "jigc setup");
}

/// The granting arm: `single-task` declares `allows-create: [{type: adr}, {type: changelog}]`,
/// so its composed agent text must name both gates — on one line, immediately before the
/// routing footer (the frontend-appended presentation mold).
#[test]
fn a_granting_workflow_names_its_gates_immediately_before_the_footer() {
    let repo = TempDir::new("granting");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    let composed = compose(
        repo.path(),
        home.path(),
        &pack,
        "single-task",
        "add a rate limiter",
        "agent",
    );

    let lines: Vec<&str> = composed.lines().collect();
    let footer = lines
        .iter()
        .position(|l| *l == ROUTING_FOOTER)
        .unwrap_or_else(|| {
            panic!("the composed view ends with the routing footer; got:\n{composed}")
        });
    assert!(
        footer > 0,
        "the footer is never the first line:\n{composed}"
    );
    // Re-blessed at M47 Inc 10 / T6 **with** the fix that moved it: the line now carries
    // the clause defining the noun it lists (the meaning of `create-gates:` had stayed on
    // the `create.gate-blocked` refusal, learnable only by tripping one). The list and its
    // position are unchanged — this expectation still pins both.
    assert_eq!(
        lines[footer - 1],
        "create-gates: adr, changelog   — the doc-types this task is allowed to create; \
         any other type is refused",
        "`single-task` grants the adr + changelog create-gates — its composed text must \
         name them, and say what a gate is, on the line immediately before the routing \
         footer; got:\n{composed}",
    );
}

/// The omitting arm: `quick-fix` declares `allows-create: []`, so it grants nothing and
/// prints **no** gates line — omitted, not rendered as an empty list or `none`. The
/// feature composed into a context that omits the target is inert.
#[test]
fn a_gate_less_workflow_prints_no_gates_line() {
    let repo = TempDir::new("gateless");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    let composed = compose(
        repo.path(),
        home.path(),
        &pack,
        "quick-fix",
        "fix the typo",
        "agent",
    );

    assert!(
        composed.trim_end().ends_with(ROUTING_FOOTER),
        "the composed view ends with the routing footer; got:\n{composed}",
    );
    let offenders: Vec<&str> = composed
        .lines()
        .filter(|l| l.trim_start().starts_with("create-gates:"))
        .collect();
    assert!(
        offenders.is_empty(),
        "`quick-fix` grants no create-gate (`allows-create: []`) — it must print NO \
         `create-gates:` line (omitted, never `none`); got {offenders:?} in:\n{composed}",
    );
}

/// The pinned contract is untouched: `--format json` deserializes to **exactly**
/// `{task, text}` on **both** arms — no new key — and the presentation line never leaks
/// into the `text` bytes (it is appended in the frontend, never in the JSON projection).
#[test]
fn the_json_contract_stays_exactly_task_and_text_on_both_arms() {
    let repo = TempDir::new("json");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    for (workflow, intent) in [
        ("single-task", "add a rate limiter"),
        ("quick-fix", "fix the typo"),
    ] {
        let out = compose(repo.path(), home.path(), &pack, workflow, intent, "json");
        let value: serde_json::Value = serde_json::from_str(&out)
            .unwrap_or_else(|e| panic!("`{workflow}` --format json parses: {e}; got:\n{out}"));
        let object = value
            .as_object()
            .unwrap_or_else(|| panic!("`{workflow}` --format json is an object; got:\n{out}"));

        let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            ["task", "text"],
            "the composed-output contract is pinned at exactly `{{task, text}}` \
             (`design/command-output-contract.md` §1) — the gates line is presentation and \
             adds NO key; `{workflow}` emitted:\n{out}",
        );
        let text = object["text"].as_str().expect("`text` is a string");
        assert!(
            !text.contains("create-gates:"),
            "the gates line is appended in the CLI frontend, never in the JSON projection \
             — `{workflow}`'s `text` must not carry it; got:\n{text}",
        );
        assert!(
            !text.contains(ROUTING_FOOTER),
            "JSON carries no routing footer either (the mold this line joins); got:\n{text}",
        );
    }
}
