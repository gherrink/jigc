//! M48 Increment 7 / T3 — **the six cascade-authoring acks say the write is
//! uncommitted, in text and on the wire** (`DECISIONS.md` → 2026-08-13 the Settle,
//! F5's text half; [overrides.md](../../../design/overrides.md) → Authoring deltas;
//! [command-output-contract.md](../../../design/command-output-contract.md) →
//! Evolution posture).
//!
//! The measured defect, three times in the RC-pre-1.0 trial: `jigc config set` writes
//! `.jigc/config/` — a **committed** layer, unlike the rest of the gitignored `.jigc/`
//! workbench — and commits nothing, so the change rides an unrelated feature commit,
//! or a fresh clone inherits no config at all. Nothing on either surface said so.
//!
//! **The axis is all six write verbs, not the one the finding named.** The fix is a
//! clause every [`ConfigAck`] closes with and an envelope key every variant carries,
//! so the two surfaces state one fact — the parity rule this increment fences (*a
//! value the agent/human text prints but the envelope withholds is a gap*).
//!
//! **Iterated from [`ConfigAck::ALL`]**, the code-side axis this task mints: a
//! witness-constructor table on the `STORE_EXIT_FLIPS` mold (every variant carries
//! data, so each member supplies its own sample). A seventh authoring verb owes an arm
//! there, and [`config_ack_all_bijects_against_the_clap_config_verbs`] reddens until it
//! has one — the table cannot silently fall behind the verb tree.

use clap::CommandFactory;
use cli::cli::{Cli, Format};
use cli::render::{ConfigAck, config_ack};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The three facts the agent/human line must state, spelled here rather than read from
/// the renderer's own constant — a fence that quotes the code it fences proves only
/// that the code equals itself.
const TEXT_FACTS: &[&str] = &[
    // *where* the write landed — the committed project layer, not the gitignored workbench
    ".jigc/config/",
    // *that* it is not committed
    "uncommitted",
    // *what to do about it* — the prose the `docs-root` relocation lines already use
    "commit it with your next commit",
];

/// The `config` leaf verbs that **read** rather than write, so they render a knob
/// reading and not a [`ConfigAck`] (M48 Inc 6). The complement of [`ConfigAck::ALL`]
/// over the clap tree.
const READ_VERBS: &[&str] = &["get", "list"];

/// **The clause is on every arm, in both surfaces.** Iterates [`ConfigAck::ALL`],
/// feeding each member's witness to the real renderer in all three formats: the agent
/// and human lines state the three facts, and the JSON envelope carries the same fact
/// as a key (`committed: false`) beside the arm's declared `op`.
#[test]
fn every_config_ack_states_its_uncommitted_write_in_text_and_on_the_wire() {
    for arm in ConfigAck::ALL {
        let ack = (arm.witness)();

        for format in [Format::Agent, Format::Human] {
            let line = config_ack(format, &ack);
            assert!(
                line.starts_with("config: "),
                "`config {}`'s ack keeps its terse effect line; got:\n{line}",
                arm.verb,
            );
            for fact in TEXT_FACTS {
                assert!(
                    line.contains(fact),
                    "`config {}`'s {format:?} ack must state `{fact}` — the write lands in a \
                     committed layer and jigc commits nothing; got:\n{line}",
                    arm.verb,
                );
            }
        }

        let out = config_ack(Format::Json, &ack);
        let doc: Value = serde_json::from_str(&out).unwrap_or_else(|err| {
            panic!(
                "`config {}`'s JSON ack must be one document ({err}); got:\n{out}",
                arm.verb
            )
        });
        assert_eq!(
            doc["op"], arm.op,
            "`config {}`'s envelope carries its declared op; got:\n{out}",
            arm.verb,
        );
        assert_eq!(
            doc["committed"],
            Value::Bool(false),
            "`config {}`'s envelope carries the uncommitted state the text prints — a value \
             the text states and the envelope withholds is a gap; got:\n{out}",
            arm.verb,
        );
    }
}

/// **The table cannot fall behind the verb tree.** Every `config` leaf verb clap knows
/// is either an authoring verb with an [`ConfigAck::ALL`] arm or a declared read verb —
/// no missing member, no stale extra. A seventh authoring verb therefore reddens this
/// until it supplies a witness, and a deleted one reddens it until its arm goes.
#[test]
fn config_ack_all_bijects_against_the_clap_config_verbs() {
    let cli = Cli::command();
    let config = cli
        .get_subcommands()
        .find(|sub| sub.get_name() == "config")
        .expect("the clap tree carries a `config` subcommand");
    let leaves: BTreeSet<String> = config
        .get_subcommands()
        .filter(|sub| sub.get_name() != "help")
        .map(|sub| sub.get_name().to_string())
        .collect();

    let classified: BTreeSet<String> = ConfigAck::ALL
        .iter()
        .map(|arm| arm.verb.to_string())
        .chain(READ_VERBS.iter().map(|verb| (*verb).to_string()))
        .collect();

    assert_eq!(
        leaves, classified,
        "every `config` leaf verb is an authoring verb with a `ConfigAck::ALL` arm or a \
         declared read verb",
    );
    assert_eq!(
        ConfigAck::ALL.len(),
        leaves.len() - READ_VERBS.len(),
        "`ConfigAck::ALL` holds one arm per authoring verb (no duplicate arms)",
    );

    let ops: BTreeSet<&str> = ConfigAck::ALL.iter().map(|arm| arm.op).collect();
    assert_eq!(
        ops.len(),
        ConfigAck::ALL.len(),
        "each arm's `op` discriminates it on the wire",
    );
}

// ─────────────────────── the end-to-end arm, both formats ───────────────────────

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-config-ack-{tag}-{}-{:?}",
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

/// A real git repo with a seeded `.jigc/config/` project layer — the shape the
/// authoring verbs discover and require.
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

    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("create project layer");
    fs::write(config.join("manifest.yaml"), "scalar: {}\n").expect("write project manifest");
}

/// Run `jigc config <args>` with `cwd = repo` and `$HOME = home`.
fn run_config(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("config");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// **Through the real binary, in both formats.** `jigc config set` lands a `scalar-set`
/// in `.jigc/config/manifest.yaml`, leaves it uncommitted (the file is *untracked* in
/// git's eyes right here), and says so on the surface the caller asked for.
#[test]
fn config_set_through_the_binary_states_the_uncommitted_write_in_both_formats() {
    let home = TempDir::new("e2e-home");

    // The agent surface.
    {
        let repo = TempDir::new("e2e-text");
        init_repo(repo.path());
        let out = run_config(
            repo.path(),
            home.path(),
            &["set", "invocation-log", "true", "--format", "agent"],
        );
        let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
        let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
        assert!(
            out.status.success(),
            "`jigc config set` exits 0; stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        assert!(
            stdout.contains("config: set `invocation-log` = `true`"),
            "the ack keeps its effect line; got:\n{stdout}",
        );
        for fact in TEXT_FACTS {
            assert!(
                stdout.contains(fact),
                "the agent ack states `{fact}`; got:\n{stdout}",
            );
        }

        // The claim the clause makes is true of this repo: the write is not committed.
        let status = Command::new("git")
            .args(["status", "--porcelain", "--", ".jigc/config/manifest.yaml"])
            .current_dir(repo.path())
            .output()
            .expect("run git status");
        let porcelain = String::from_utf8(status.stdout).expect("utf-8 git status");
        assert!(
            porcelain.contains(".jigc/config/manifest.yaml"),
            "the manifest write is genuinely uncommitted after the ack; git status:\n{porcelain}",
        );
    }

    // The machine surface.
    {
        let repo = TempDir::new("e2e-json");
        init_repo(repo.path());
        let out = run_config(
            repo.path(),
            home.path(),
            &["set", "invocation-log", "true", "--format", "json"],
        );
        let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
        let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
        assert!(
            out.status.success(),
            "`jigc config set --format json` exits 0; stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        let doc: Value = serde_json::from_str(&stdout)
            .unwrap_or_else(|err| panic!("stdout is one JSON document ({err}); got:\n{stdout}"));
        assert_eq!(doc["op"], "config-set", "the envelope names the op");
        assert_eq!(
            doc["committed"],
            Value::Bool(false),
            "the envelope carries the uncommitted state the text prints; got:\n{stdout}",
        );
    }
}
