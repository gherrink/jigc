//! M45 Increment 9 / T2 — the **JSON-purity property suite** pinning *stream
//! discipline* ([command-output-contract.md](../../../design/command-output-contract.md)
//! → Stream discipline). One suite per pinned statement, its module doc-comment
//! naming and linking the section it fences (`implementation/pinning.md` §2).
//!
//! The statement, verbatim from the contract: **under `--format json` the JSON-bearing
//! stream parses as exactly one document, and the other stream carries no JSON at all.**
//! *Which* stream is JSON-bearing is decided by the **outcome class**, never the exit
//! code, so the driver's discrimination predicate is *parse stdout; if stdout is empty,
//! parse stderr* (contract → Stream discipline, the discrimination predicate):
//!
//!   * an **adjudication** (a landed finalize, a validation report) writes exactly one
//!     document to **stdout**, every agent-text side channel (the hook relay, advisories)
//!     to stderr;
//!   * a **reject** (an operational error, a blocked write) leaves **stdout empty** and
//!     writes exactly one document to **stderr**;
//!   * a **clap-resolved usage error** is the declared carve-out — clap formats and prints
//!     its own message before any jigc code reads `--format`, so it is **plain text, exit
//!     2, no envelope on either stream** (contract → the clap carve-out is declared).
//!
//! **Verbs enumerate from the clap tree** via `CommandFactory` (`--format` is a *global*
//! arg, so every verb is in scope by construction) — a new verb is swept the day it lands,
//! no hand list. The **error/usage surface is asserted for every enumerated verb
//! fixture-free**: run in a bare non-git, non-project directory, where a verb with required
//! args clap-errors (exit 2) and every other verb operational-errors (exit 1) — no verb can
//! succeed, so the predicate's two reject branches are exercised across the whole tree. The
//! **success arm** runs one real landed finalize under [`State::ChattyHooks`] — a foreign
//! non-blocking `pre-commit` whose stdout git folds onto its own stderr — and pins the
//! merged-fd reality the M45 `hook_output` key answers: the captured hook text rides the one
//! stdout document while the verbatim `--- hook output ---` relay rides stderr.

mod support;

use clap::CommandFactory;
use cli::cli::Cli;
use cli::task::{EXIT_SUCCESS, EXIT_USAGE};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use support::trial_corpus::{CHATTY_HOOK_MARKER, State, TrialCorpus};

/// Whether `s` parses as **exactly one** JSON document — the pinnable form of "this
/// stream carries the document" (and, negated, of "this stream carries no JSON at all").
/// `serde_json::from_str` accepts leading/trailing whitespace but rejects trailing
/// non-whitespace, so a stream holding a document plus a plain-text side channel fails.
fn is_one_json_doc(s: &str) -> bool {
    serde_json::from_str::<Value>(s).is_ok()
}

/// Every **leaf** verb's argv path, walked from the clap `Command` tree — the real
/// enumeration seam, so a subcommand added anywhere auto-joins this sweep. clap's own
/// auto-generated `help` subcommand is skipped (it is not a jigc verb).
fn leaf_verb_paths() -> Vec<Vec<String>> {
    fn walk(cmd: &clap::Command, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
        let mut had_child = false;
        for sub in cmd.get_subcommands() {
            if sub.get_name() == "help" {
                continue;
            }
            had_child = true;
            let mut child = prefix.clone();
            child.push(sub.get_name().to_string());
            walk(sub, child, out);
        }
        // A leaf verb (no jigc subcommands of its own) is one invocation to sweep.
        if !had_child && !prefix.is_empty() {
            out.push(prefix);
        }
    }
    let mut out = Vec::new();
    walk(&Cli::command(), Vec::new(), &mut out);
    out
}

/// A throwaway **bare** directory — not a git repo, not a jigc project — with an isolated
/// `$HOME`, in which every verb is guaranteed to fail (a required-arg verb at clap, every
/// other verb operationally). Removes itself on drop.
struct BareDir {
    root: PathBuf,
}

impl BareDir {
    fn new() -> Self {
        let mut root = std::env::temp_dir();
        root.push(format!(
            "jigc-machine-output-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock after the epoch")
                .as_nanos(),
        ));
        fs::create_dir_all(root.join("home")).expect("create the bare corpus home");
        BareDir { root }
    }

    /// Run `jigc --format json <verb...>` in the bare dir with no extra args, the isolated
    /// `$HOME`, and `JIGC_PACK_DIR` scrubbed (so the embedded pack is always the one loaded).
    fn run_json(&self, verb: &[String]) -> Output {
        let mut args: Vec<&str> = vec!["--format", "json"];
        args.extend(verb.iter().map(String::as_str));
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(&args)
            .current_dir(&self.root)
            .env("HOME", self.root.join("home"))
            .env_remove("JIGC_PACK_DIR")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("spawn jigc")
    }
}

impl Drop for BareDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// **The enumeration sweep.** Every leaf verb, driven fixture-free under `--format json`,
/// obeys the discrimination predicate's two reject branches:
///
///   * a **clap usage error** (exit [`EXIT_USAGE`]) is plain text on **both** streams —
///     neither parses as a JSON document — and stdout is empty (the clap carve-out);
///   * any **operational reject** (exit ≠ 0, ≠ 2) leaves **stdout empty** and writes
///     **exactly one** JSON document to **stderr**.
///
/// No verb may succeed fixture-free (that would be a side-effect-free success in a bare
/// dir), so the success branch is left to the [`State::ChattyHooks`] arm below.
#[test]
fn every_verb_obeys_stream_discipline_on_its_reject_surface() {
    let bare = BareDir::new();
    let verbs = leaf_verb_paths();
    assert!(
        verbs.len() >= 40,
        "the clap tree should enumerate the whole verb surface; got {} verbs: {verbs:?}",
        verbs.len(),
    );

    for verb in &verbs {
        let label = verb.join(" ");
        let out = bare.run_json(verb);
        let code = out.status.code().unwrap_or(-1);
        let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
        let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

        assert_ne!(
            code, EXIT_SUCCESS as i32,
            "`jigc {label}` must not SUCCEED fixture-free (bare non-project dir); \
             stdout:\n{stdout}\nstderr:\n{stderr}",
        );

        if code == EXIT_USAGE as i32 {
            // The clap carve-out: adjudicated before jigc reads `--format`, so plain text.
            assert!(
                stdout.is_empty(),
                "`jigc {label}` clap usage error must leave stdout empty; stdout:\n{stdout}",
            );
            assert!(
                !is_one_json_doc(&stdout) && !is_one_json_doc(&stderr),
                "`jigc {label}` clap usage error is plain text on both streams, no envelope; \
                 stdout:\n{stdout}\nstderr:\n{stderr}",
            );
            assert!(
                !stderr.trim().is_empty(),
                "`jigc {label}` clap usage error must say what was wrong on stderr",
            );
        } else {
            // An operational reject: stdout empty, stderr the sole document.
            assert!(
                stdout.is_empty(),
                "`jigc {label}` reject (exit {code}) must leave stdout empty; stdout:\n{stdout}",
            );
            assert!(
                is_one_json_doc(&stderr),
                "`jigc {label}` reject (exit {code}) must write exactly one JSON document to \
                 stderr; stderr:\n{stderr}",
            );
        }
    }
}

/// **The clap carve-out, as its own arm** (contract → the clap carve-out is declared): a
/// usage error is plain text, exit 2, and carries **no** envelope on either stream — even
/// under `--format json`, because `--format` is jigc's own flag, parsed by the same clap
/// pass that rejects the bad argv. `jigc doc show` with no address is the canonical case
/// (the contract names it: *the first `jigc doc show` with no argument reddens it*).
#[test]
fn clap_usage_error_is_plain_text_exit_2_with_no_envelope() {
    let bare = BareDir::new();
    let out = bare.run_json(&["doc".to_string(), "show".to_string()]);
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    assert_eq!(
        code, EXIT_USAGE as i32,
        "a missing-required-arg usage error exits {EXIT_USAGE}; got {code}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.is_empty(),
        "clap prints nothing to stdout; got:\n{stdout}"
    );
    assert!(
        !is_one_json_doc(&stdout) && !is_one_json_doc(&stderr),
        "a clap usage error is plain text — never a JSON envelope; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    // Plain text really means clap's own usage message — not jigc's operational-error prose.
    assert!(
        stderr.contains("Usage:") || stderr.to_lowercase().contains("error"),
        "clap's own usage message rides stderr; got:\n{stderr}",
    );
}

/// **The success arm** (contract → Stream discipline; the M45 `hook_output` key). A real
/// landed `task finalize --format json` under a chatty, non-blocking foreign `pre-commit`:
///
///   * **stdout parses as exactly one JSON document**, and its `committed.hook_output`
///     carries the captured hook text — so a driver that merged git's two fds still reads
///     the hook output from the one document it already parses;
///   * the **verbatim `--- hook output ---` relay rides stderr**, which carries **no** JSON
///     document (the M19 discipline unqualified — the relay never touches the stdout stream);
///   * **one capture, two channels** — the stderr relay carries the *same* captured string
///     the `hook_output` field does (the field is not re-run, re-trimmed, or re-formatted).
#[test]
fn landed_finalize_under_chatty_hooks_carries_hook_output_on_stdout_and_relays_on_stderr() {
    let corpus = TrialCorpus::build(State::ChattyHooks);
    let task = corpus.start_workflow("planning", "plan the first wave");
    corpus.jigc_ok(&[
        "doc", "create", "roadmap", "--title", "Roadmap", "--task", &task,
    ]);
    // Author the transient commit doc so finalize has a subject to render (the fields the
    // `TrialCorpus::finalize` helper sets, driven here so the finalize itself is `--format
    // json` with both streams captured raw).
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "docs",
        "--task",
        &task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#scope"),
        "--value",
        "planning",
        "--task",
        &task,
    ]);
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "mint the roadmap",
    );
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#body"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "Built by the machine-output suite.",
    );

    let out = corpus.jigc(&["--format", "json", "task", "finalize", &task]);
    assert!(
        out.status.success(),
        "the chatty (non-blocking) hook must not block the finalize; status {}",
        out.status,
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    // Stdout is exactly one JSON document — the whole finalize envelope.
    let value: Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("stdout must be exactly one JSON document ({e}); got:\n{stdout}")
    });

    // The captured hook text rides the one stdout document, in `committed.hook_output`.
    let hook_output = value["committed"]["hook_output"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("committed.hook_output must be a present string; got:\n{stdout}")
        });
    assert_eq!(
        hook_output, CHATTY_HOOK_MARKER,
        "the captured hook text is the chatty hook's line, inside the stdout document",
    );

    // The verbatim relay rides stderr, and stderr carries no JSON document.
    assert!(
        !is_one_json_doc(&stderr),
        "stderr must carry no JSON document — only the agent-text relay; got:\n{stderr}",
    );
    // One capture, two channels: the stderr relay carries the SAME captured string the
    // `hook_output` field does, in its delimited section (the field is not re-formatted).
    assert!(
        stderr.contains(&format!("--- hook output ---\n{hook_output}")),
        "the stderr relay must carry the same captured string as `hook_output`; got:\n{stderr}",
    );
}
