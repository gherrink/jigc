//! **Pinned fact** (findings-verification.md §2, row 2):
//! *"finalize `--format json` emits non-JSON" — **REFUTED** as an fd-routing bug; the
//! field experience is real.*
//!
//! Three trial sessions saw `task finalize --format json` "emit non-JSON". The
//! refutation: **stdout is pure JSON**; the `--- hook output ---` relay rides
//! **stderr** (the M19/S1 stream discipline). The parse failures came from agent
//! harnesses **merging the two streams** — a driver bug, not a contract breach. The
//! fragility is real (every merged-stream driver hits it), which is why the M45
//! `hook_output` key folds the captured hook text into the one stdout document a
//! merged-stream driver already parses.
//!
//! The repro block, made standing:
//!
//! ```yaml
//! claim: "finalize --format json emits non-JSON under a chatty hook"
//! verdict: REFUTED
//! setup: [ { fixture: chatty-hooks } ]
//! repro:  [ ["jigc","--format","json","task","finalize","<task>"] ]
//! expect:
//!   stdout: exactly one JSON document (no --- hook output --- delimiter)
//!   stderr: carries the --- hook output --- relay and the hook's line
//! ```
//!
//! **Fact drift, not contract drift.** `machine_output::landed_finalize_under_chatty_hooks_…`
//! pins the *contract* (stream discipline as a per-outcome-class property, plus the
//! `hook_output` key). This pins the *trial claim* — that the "non-JSON" bytes the
//! sessions saw are stderr's, and stdout **alone** parses (pinning.md §5).

use crate::support::trial_corpus::{CHATTY_HOOK_MARKER, State, TrialCorpus};
use serde_json::Value;

/// Author the transient commit doc so finalize has a subject, without the
/// `TrialCorpus::finalize` helper (which asserts success and hides the streams) — the
/// point here is to capture both streams of a **raw** `--format json` finalize.
fn author_commit(corpus: &TrialCorpus, task: &str) {
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "docs",
        "--task",
        task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#scope"),
        "--value",
        "planning",
        "--task",
        task,
    ]);
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
            "--task",
            task,
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
            task,
        ],
        "Built by the pinned-facts finalize-json test.",
    );
}

/// **The pin.** A real landed `task finalize --format json` under the `chatty-hooks`
/// state — a foreign, non-blocking `pre-commit` whose stdout git folds onto its own
/// stderr. **stdout alone parses as exactly one JSON document, carrying no
/// `--- hook output ---` delimiter**; the relay and the hook's line ride **stderr**.
/// The `hook_output` key carries the captured text inside that one document, so a
/// merged-stream driver still recovers it. That whole shape is the refutation.
#[test]
fn finalize_json_stdout_is_pure_json_hook_relay_on_stderr() {
    let corpus = TrialCorpus::build(State::ChattyHooks);
    let task = corpus.start_workflow("planning", "plan the first wave");
    corpus.jigc_ok(&[
        "doc", "create", "roadmap", "--title", "Roadmap", "--task", &task,
    ]);
    author_commit(&corpus, &task);

    let out = corpus.jigc(&["--format", "json", "task", "finalize", &task]);
    assert!(
        out.status.success(),
        "the chatty (non-blocking) hook must not block the finalize; status {}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    // (1) stdout ALONE parses as exactly one JSON document — the whole refutation.
    let value: Value = serde_json::from_str(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "REFUTED (§2 row 2): finalize `--format json` stdout must be exactly one \
             JSON document ({e}) — the 'non-JSON' the trial saw was the MERGED stream. \
             stdout:\n{stdout}\nstderr:\n{stderr}"
        )
    });

    // (2) The `--- hook output ---` relay — the non-JSON bytes — is stderr's, never
    //     stdout's. This is the exact discrimination a merged-stream driver loses.
    assert!(
        !stdout.contains("--- hook output ---"),
        "the hook relay delimiter must NOT be on stdout — that is what breaks a naive \
         JSON.parse(stdout); stdout:\n{stdout}",
    );
    assert!(
        stderr.contains("--- hook output ---") && stderr.contains(CHATTY_HOOK_MARKER),
        "the verbatim hook relay + the hook's line ride stderr; got:\n{stderr}",
    );

    // (3) The M45 answer: the captured hook text is recoverable from the one stdout
    //     document, so a driver that merged git's two fds still reads it.
    assert_eq!(
        value["committed"]["hook_output"].as_str(),
        Some(CHATTY_HOOK_MARKER),
        "`committed.hook_output` folds the captured hook text into the stdout document; \
         got:\n{stdout}",
    );
}
