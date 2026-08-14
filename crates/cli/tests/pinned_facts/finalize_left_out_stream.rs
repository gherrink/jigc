//! **Pinned fact** (RC-alpha4 [findings-verification.md](../../../../completions/artifacts/RC-alpha4/findings-verification.md)
//! → **A2**): *"`task finalize --format json` emitted three lines of human text
//! before the JSON object"* — **REFUTED as stated: stdout is pure JSON; the three
//! lines are the `left-out` advisory, and it rides stderr.**
//!
//! P2's "three lines" are the pre-commit `left-out (unstaged/untracked — git add to
//! include)` block naming the files the index commit leaves behind. Under
//! `--format json` the structured envelope owns stdout, so the advisory is routed to
//! **stderr** (`task.rs::emit_left_out_advisory`) — a merged-stream capture (`2>&1`,
//! or an agent harness that interleaves the two fds) reproduces P2's picture exactly,
//! and a merged `| jq` does fail. The content is not lost either way: the same set
//! rides the one stdout document as `committed.left_out`.
//!
//! **This is the half that was unpinned** ([M47 baseline](../../../../completions/artifacts/M47/baseline.md)
//! §2 premise 6): the non-empty-`left_out` **stdout** arm stands at
//! `crates/cli/tests/finalize_manifest.rs:313` (`json_manifest_on_dry_run_and_landed_run`),
//! and A2's finalize-JSON *purity under a chatty hook* stands at
//! [`super::finalize_json`] — neither drives the **stderr** arm. Re-checked at HEAD
//! before this file was written: no suite asserted the advisory's stream.
//!
//! The repro block, made standing:
//!
//! ```yaml
//! claim: "finalize --format json emits three lines of human text before the JSON object"
//! verdict: REFUTED as stated (the lines are the left-out advisory, on stderr)
//! setup:
//!   - fixture: fresh
//!   - a tracked file modified unstaged + an untracked file, beside the task's staged work
//! repro:
//!   - ["jigc","task","finalize","<task>","--format","json"]   # 1> fin.json 2> fin.err
//! expect:
//!   stdout: exactly one JSON document (byte 1 is `{`); no `left-out (` block;
//!           `committed.left_out` names the unstaged + untracked paths
//!   stderr: the `finalize — about to commit the index; leaving out:` block naming them
//! contrast:
//!   a finalize with nothing left behind prints the block on NEITHER stream
//! ```
//!
//! **Fact drift, not contract drift.** `machine_output.rs` pins the *statement* —
//! stream discipline as a per-outcome-class property. This pins the *trial claim*:
//! the specific agent-text block P2 saw "before the JSON", shown to be stderr's, with
//! the empty-`left_out` contrast that keeps the marker from being vacuously absent
//! (pinning.md §3).

use crate::support::trial_corpus::{State, TrialCorpus};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;

/// The verbatim first line of the pre-commit advisory (`render::left_out_advisory`).
/// **Reworded at M48 Inc 9 / T3** (RC-pre-1.0 → F13 — the header stated the deed above a
/// rejection); the stream discipline this file pins is untouched by the wording, and the
/// header's own intent-not-deed property is pinned at
/// `finalize_message_truth::both_pre_commit_headers_state_the_intent_and_a_rejected_finalize_never_reads_as_done`.
const ADVISORY_TITLE: &str = "finalize — about to commit the index; leaving out:";
/// The verbatim section header the advisory and the landed residual share
/// (`render::left_out_lines`).
const LEFT_OUT_HEADER: &str = "left-out (unstaged/untracked — git add to include):";

/// Fill the task's transient `commit` doc without the `TrialCorpus::finalize` helper,
/// which asserts success and swallows the streams — both streams are the subject here.
fn author_commit(corpus: &TrialCorpus, task: &str, summary: &str) {
    for (leaf, value) in [("type", "docs"), ("scope", "pinning")] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#{leaf}"),
            "--value",
            value,
            "--task",
            task,
        ]);
    }
    for (leaf, prose) in [
        ("summary", summary),
        ("body", "Built by the pinned-facts left-out stream test."),
    ] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#{leaf}"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            prose,
        );
    }
}

/// The `path` of every entry in a manifest-shaped JSON array.
fn paths(entries: &Value) -> BTreeSet<String> {
    entries
        .as_array()
        .unwrap_or_else(|| panic!("a manifest key is an array; got:\n{entries:#}"))
        .iter()
        .map(|e| {
            e["path"]
                .as_str()
                .unwrap_or_else(|| panic!("a manifest entry carries a `path`; got:\n{e:#}"))
                .to_string()
        })
        .collect()
}

/// **The pin.** A landed `task finalize --format json` that leaves files behind:
/// **stdout alone parses as exactly one JSON document** and carries no agent-text
/// block, while the `left-out` advisory — P2's "three lines of human text" — rides
/// **stderr** naming each left-behind path. The same set is recoverable from the one
/// stdout document as `committed.left_out`, so a merged-stream driver loses nothing
/// but the ability to `jq` its capture.
#[test]
fn the_left_out_advisory_rides_stderr_while_stdout_stays_one_json_document() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();

    // Repo furniture committed BEFORE the mint, so nothing is staged at mint time
    // (a pre-staged file is the carryover gate's business, not this pin's).
    fs::write(repo.join("keep.txt"), "the tracked file\n").expect("write keep.txt");
    corpus.git(&["add", "keep.txt"]);
    corpus.git(&["commit", "-q", "-m", "seed the tracked file"]);

    let task = corpus.start_workflow("single-task", "leave two files behind");

    // The task's own staged work...
    fs::write(repo.join("worked.txt"), "the task's change\n").expect("write worked.txt");
    corpus.git(&["add", "worked.txt"]);
    // ...beside exactly what P2 had: one unstaged modification, one untracked file.
    fs::write(repo.join("keep.txt"), "the tracked file, edited\n").expect("edit keep.txt");
    fs::write(repo.join("scratch.txt"), "private WIP\n").expect("write scratch.txt");

    author_commit(&corpus, &task, "leave two files behind");
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    assert!(
        out.status.success(),
        "the finalize lands — a left-out set surfaces, it never blocks; status {}\n\
         stderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    // (1) stdout ALONE parses as exactly one JSON document — the refutation of
    //     "three lines of human text before the JSON object".
    let value: Value = serde_json::from_str(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "REFUTED (A2): `finalize --format json` stdout must be exactly one JSON \
             document ({e}) — the human lines the trial saw were the MERGED stream.\n\
             stdout:\n{stdout}\nstderr:\n{stderr}"
        )
    });
    assert!(
        !stdout.contains(ADVISORY_TITLE) && !stdout.contains(LEFT_OUT_HEADER),
        "the agent-text advisory must NOT be on stdout — that is exactly what breaks \
         a naive `| jq`; stdout:\n{stdout}",
    );

    // (2) The advisory is stderr's, and it names each left-behind path.
    assert!(
        stderr.contains(ADVISORY_TITLE) && stderr.contains(LEFT_OUT_HEADER),
        "the `left-out` advisory rides stderr under `--format json`; stderr:\n{stderr}",
    );
    for path in ["keep.txt", "scratch.txt"] {
        assert!(
            stderr.contains(path),
            "the advisory names `{path}` — the unstaged/untracked file the commit \
             leaves behind; stderr:\n{stderr}",
        );
    }

    // (3) Nothing is lost to a merged-stream driver: the same set rides the one
    //     stdout document, and the task's own staged work is NOT in it.
    let left_out = paths(&value["committed"]["left_out"]);
    assert_eq!(
        left_out,
        BTreeSet::from(["keep.txt".to_string(), "scratch.txt".to_string()]),
        "`committed.left_out` names the same set the stderr advisory printed; \
         stdout:\n{stdout}",
    );
    assert!(
        paths(&value["committed"]["manifest"]).contains("worked.txt"),
        "the task's staged work landed in the commit, never in the left-out set; \
         stdout:\n{stdout}",
    );
}

/// **The contrast that makes the pin a fact.** With nothing left behind, the block
/// appears on **neither** stream and `committed.left_out` is empty — so the stderr
/// assertion above is the advisory being routed, not a marker that is always there.
#[test]
fn a_finalize_that_leaves_nothing_behind_prints_the_block_on_neither_stream() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();

    let task = corpus.start_workflow("single-task", "leave nothing behind");
    fs::write(repo.join("worked.txt"), "the task's change\n").expect("write worked.txt");
    corpus.git(&["add", "worked.txt"]);

    author_commit(&corpus, &task, "leave nothing behind");
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    assert!(
        out.status.success(),
        "the clean finalize lands; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    assert!(
        !stdout.contains(LEFT_OUT_HEADER) && !stderr.contains(LEFT_OUT_HEADER),
        "with nothing left behind the block is printed nowhere;\nstdout:\n{stdout}\n\
         stderr:\n{stderr}",
    );
    let value: Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("stdout is one JSON document ({e}); got:\n{stdout}"));
    assert_eq!(
        value["committed"]["left_out"],
        serde_json::json!([]),
        "`committed.left_out` is empty in the same state; stdout:\n{stdout}",
    );
}
