//! M55 completion-audit CR3 — **a task whose staged doc is byte-identical to the committed
//! doc changes nothing, and finalize says so before git is asked.**
//!
//! The empty-commit guard's diff signal counted a staged doc whenever its doctype promotes,
//! without asking whether promoting it would change a byte. A task that copied a committed
//! doc in and wrote it back unchanged — a triage setting `status` to the value it already had,
//! a single-task re-setting an ADR's `status` — therefore passed the guard, reached
//! `git commit`, met git's own *"nothing to commit, working tree clean"*, and was routed as
//! `finalize.commit-rejected`: *"Fix the hook's complaint"*, when no hook had spoken.
//!
//! Both commit arms are pinned, each with the `--dry-run` forecast beside the real finalize
//! (the two read one diff signal), and each followed by its control: the same task, given a
//! real change, lands — so the guard refuses a no-op and nothing else.
//!
//! - **The doc-only arm** — `triage-inconsistency` over a filed record, its `status` set to
//!   the `open` it was filed with, every write through the composed text's emitted lines.
//! - **The ordinary arm** — `single-task` re-setting a committed ADR's `status` to the value
//!   it was committed with.

use crate::findings_workflows::{
    Composed, VISIBLE, author_inconsistency, commit_count, fill_commit, finalize, run_emitted,
    shown, slug_of, start, start_triage, text,
};
use crate::support;

use std::process::Output;

use support::trial_corpus::{State, TrialCorpus};

/// The blocked-finalize exit code (`design/command-output-contract.md` → exit codes).
const BLOCKED: i32 = 3;

/// Assert `out` is the honest empty-commit refusal: the engine's own code at the blocked exit,
/// and none of git's refusal or the hook route a commit-rejected misroute prints.
fn assert_empty_commit(out: &Output, door: &str) {
    let all = text(out);
    assert_eq!(
        out.status.code(),
        Some(BLOCKED),
        "{door} blocks at the empty-commit guard; {all}",
    );
    assert!(
        all.contains("finalize.empty-commit"),
        "{door} names `finalize.empty-commit`; {all}",
    );
    for misroute in [
        "nothing to commit",
        "hook",
        "commit-rejected",
        "was rejected",
    ] {
        assert!(
            !all.contains(misroute),
            "{door} never reaches git, so it never prints `{misroute}`; {all}",
        );
    }
}

/// The doc-only arm: a triage that sets a filed record's `status` to the `open` it already
/// carries stages a byte-identical copy, so the path-scoped commit would record nothing.
#[test]
fn a_triage_writing_the_filed_bytes_back_is_an_empty_commit_not_a_hook_rejection() {
    let corpus = TrialCorpus::build(State::Fresh);
    let report = start(&corpus, VISIBLE, "the cache docs disagree on eviction");
    let address = author_inconsistency(&corpus, &report);
    let landed = finalize(&corpus, &report);
    assert!(
        landed.status.success(),
        "the report lands; {}",
        text(&landed)
    );

    let triage = start_triage(&corpus, "triage-inconsistency", &address);
    let slug = slug_of(&address);
    let set_status = |value: &str| {
        run_emitted(
            &corpus,
            &triage,
            "jigc doc set-field inconsistency:<slug>#meta/status ",
            &[("<slug>", &slug), ("<resolved|intended|refuted>", value)],
            None,
        )
    };
    let first = set_status("open");
    assert!(
        first.contains("copied in for update"),
        "the write copies the committed record in; got:\n{first}",
    );
    fill_commit(&corpus, &triage, "inconsistencies");

    let before = commit_count(&corpus);
    assert_empty_commit(
        &corpus.jigc(&["task", "finalize", &triage.task, "--dry-run"]),
        "the doc-only `--dry-run`",
    );
    assert_empty_commit(&finalize(&corpus, &triage), "the doc-only finalize");
    assert_eq!(commit_count(&corpus), before, "no commit was made");

    // The control: the same task, given a real change, lands.
    set_status("intended");
    assert_lands(&corpus, &triage, before);
}

/// The ordinary arm: a single-task that re-sets a committed ADR's `status` to its committed
/// value stages a byte-identical copy, so the index-honoring commit would record nothing.
#[test]
fn a_single_task_writing_a_committed_doc_back_unchanged_is_an_empty_commit() {
    let corpus = TrialCorpus::build(State::Fresh);
    let record = corpus.start_workflow("single-task", "record a cache decision");
    let adr = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Use a cache",
            "--task",
            &record,
        ])
        .trim()
        .to_owned();
    for slot in ["context", "options", "decision", "consequences"] {
        corpus.set_slot(&format!("{adr}#{slot}"), &record, "Settled for this test.");
    }
    corpus.finalize(&record, "decisions", "record a cache decision", false);

    let touch = corpus.start_workflow("single-task", "touch the cache decision");
    let committed = shown(&corpus, &adr);
    let status = committed["fields"]["status"]
        .as_str()
        .unwrap_or_else(|| panic!("the committed ADR carries a status; got:\n{committed:#}"))
        .to_owned();
    corpus.set_field(&format!("{adr}#status"), &touch, &status);
    fill_commit_doc(&corpus, &touch);

    let before = commit_count(&corpus);
    assert_empty_commit(
        &corpus.jigc(&["task", "finalize", &touch, "--dry-run"]),
        "the ordinary `--dry-run`",
    );
    assert_empty_commit(
        &corpus.jigc(&["task", "finalize", &touch]),
        "the ordinary finalize",
    );
    assert_eq!(commit_count(&corpus), before, "no commit was made");

    // The control: the same task, given a real change, lands.
    let other = if status == "accepted" {
        "proposed"
    } else {
        "accepted"
    };
    corpus.set_field(&format!("{adr}#status"), &touch, other);
    let out = corpus.jigc(&["task", "finalize", &touch]);
    assert!(out.status.success(), "a real change lands; {}", text(&out));
    assert_eq!(commit_count(&corpus), before + 1, "exactly one commit");
}

/// Author `task`'s commit doc — the leaves the ordinary arm's finalize renders.
fn fill_commit_doc(corpus: &TrialCorpus, task: &str) {
    corpus.set_field(&format!("commit:{task}#type"), task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), task, "decisions");
    corpus.set_slot(&format!("commit:{task}#summary"), task, "touch a decision");
    corpus.set_slot(&format!("commit:{task}#body"), task, "Re-set its status.");
}

/// Finalize the doc-only task through its emitted line and assert it lands one commit.
fn assert_lands(corpus: &TrialCorpus, composed: &Composed, before: usize) {
    let out = finalize(corpus, composed);
    assert!(out.status.success(), "a real change lands; {}", text(&out));
    assert_eq!(commit_count(corpus), before + 1, "exactly one commit");
}
