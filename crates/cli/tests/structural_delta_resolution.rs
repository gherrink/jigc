//! The M55 completion audit — **a project `structural-op` delta resolves one workflow at
//! every door that reads one**.
//!
//! `jigc config replace-step` / `insert-step` / `remove-step` record the project layer's
//! deltas against a pack workflow's include list (`design/overrides.md` → Resolution
//! algorithm phase 4). The fresh compose applied them; the resume (`jigc start --task`),
//! the verb-minted compose (`jigc task amend`, `jigc migrate`) and the finalize-time arms
//! keyed on a composed step (the doc-only commit, the migration review hold) all read the
//! workflow **without** them. So a delta swapping the commit model's step composed the
//! minted text on one model and finalized on the other — the CR1 text-vs-commit mismatch
//! through a different door — and a resumed task's text was not its minted text.
//!
//! Every cell installs its delta through the real verb, drives the real binary, and asserts
//! on the emitted bytes and the landed git commit:
//!
//! - **(1)** `replace-step` of `report-inconsistency`'s `finalize-doc-only` with a step that
//!   wraps the ordinary `step:finalize`: the minted `what's-left:` line states the index
//!   model, and the finalize — after a foreign path is staged — commits the index, foreign
//!   path included (the state-3 sweep the text now promises).
//! - **(2)** the reverse, `replace-step` of `park-idea`'s `finalize` with a step that wraps
//!   `step:finalize-doc-only`: the minted line states the doc-only model, and the finalize
//!   lands the idea alone, the foreign path still staged.
//! - **(3)** under both deltas, the resumed task's composed `text` equals its minted `text`
//!   byte for byte, and so does its `what's-left:` line.
//! - **(4)** an `insert-step` into `amend` (minted by `jigc task amend`, the verb-minted
//!   door) composes the inserted step, and the resume composes it again.
//! - **(5)** `replace-step` of `single-task`'s `finalize` with a step that wraps
//!   `step:migration-finalize`: the composed text promises the review hold, and the plain
//!   finalize holds at exit 4 with HEAD unmoved.

use crate::findings_workflows::{
    FOREIGN, author_inconsistency, commit_count, fill_commit, finalize, head_files, start, text,
};
use crate::support::trial_corpus::{State, TrialCorpus};

use std::fs;

use cli::gate_coverage::whats_left_coverage;
use cli::render::CommitModel;

/// A native step wrapping the ordinary index-model finalize.
const ORDINARY_WRAPPER: (&str, &str) = ("land-the-index", "{{ include: step:finalize }}\n");

/// A native step wrapping the doc-only finalize.
const DOC_ONLY_WRAPPER: (&str, &str) =
    ("land-docs-alone", "{{ include: step:finalize-doc-only }}\n");

/// A native step wrapping the migration review hold.
const HOLD_WRAPPER: (&str, &str) = (
    "hold-for-review",
    "{{ include: step:migration-finalize }}\n",
);

/// A native step carrying one sentence no shipped step says.
const AMEND_NOTE: (&str, &str) = (
    "amend-house-note",
    "House rule: an amended subject keeps its ticket prefix.\n",
);
const AMEND_NOTE_SENTENCE: &str = "House rule: an amended subject keeps its ticket prefix.";

/// The `replace-step` cells: `(target, wrapper, the model the wrapper composes)`.
const REPLACE_CELLS: [(&str, &str, (&str, &str), CommitModel); 2] = [
    (
        "report-inconsistency",
        "workflow:report-inconsistency#finalize-doc-only",
        ORDINARY_WRAPPER,
        CommitModel::Index,
    ),
    (
        "park-idea",
        "workflow:park-idea#finalize",
        DOC_ONLY_WRAPPER,
        CommitModel::DocOnly,
    ),
];

/// Write the native step's source file where nothing tracks it (`.jigc/` is gitignored)
/// and return its path — the `<FILE>` every delta verb takes.
fn step_source(corpus: &TrialCorpus, (id, body): (&str, &str)) -> String {
    let path = corpus.repo().join(".jigc").join(format!("{id}.yaml"));
    fs::write(&path, body).expect("write the native step's source");
    path.to_str().expect("a utf-8 path").to_owned()
}

/// Commit the project layer the delta verb wrote, so the tree is clean around the cell.
fn commit_config(corpus: &TrialCorpus) {
    corpus.git(&["add", "--", ".jigc/config"]);
    corpus.git(&["commit", "-q", "-m", "chore: a project structural delta"]);
}

/// A fresh corpus carrying one `replace-step` delta, recorded through the real verb.
fn with_replace(target: &str, step: (&str, &str)) -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let source = step_source(&corpus, step);
    corpus.jigc_ok(&["config", "replace-step", target, &source]);
    commit_config(&corpus);
    corpus
}

/// The emitted `what's-left:` line of an agent-format compose.
fn whats_left(composed: &str) -> String {
    composed
        .lines()
        .find(|line| line.starts_with("what's-left: "))
        .unwrap_or_else(|| panic!("a composed what's-left line; got:\n{composed}"))
        .to_owned()
}

/// Write and stage a path no task owns.
fn stage_foreign(corpus: &TrialCorpus) {
    fs::create_dir_all(corpus.repo().join("src")).expect("mk src/");
    fs::write(corpus.repo().join(FOREIGN), "pub fn foreign() {}\n").expect("write");
    corpus.git(&["add", "--", FOREIGN]);
}

/// Author a finalizable `idea` in `task` (the commit doc is filled by the caller); returns
/// its promoted path, from the address the binary emitted.
fn author_idea(corpus: &TrialCorpus, task: &str) -> String {
    let address = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "idea",
            "--title",
            "Delta Parked",
            "--task",
            task,
        ])
        .trim()
        .to_owned();
    corpus.set_field(&format!("{address}#trigger"), task, "a delta comes back");
    corpus.set_slot(
        &format!("{address}#description"),
        task,
        "One idea, parked under a project delta.",
    );
    let slug = address
        .strip_prefix("idea:")
        .unwrap_or_else(|| panic!("an idea address; got {address}"));
    format!("docs/ideas/{slug}.md")
}

/// **(1)** A `replace-step` off `finalize-doc-only` composes the index model, and the
/// finalize commits the index — the staged foreign path included, as the text says. It
/// was decided off the pack's include list, so the finalize took the doc-only arm beneath a
/// composed body that promised the index.
#[test]
fn a_replace_step_off_the_doc_only_step_composes_and_commits_the_index() {
    let (workflow, target, wrapper, model) = REPLACE_CELLS[0];
    let corpus = with_replace(target, wrapper);
    let composed = start(&corpus, workflow, "file a disagreement");
    assert!(
        whats_left(&composed.text).contains(&whats_left_coverage(model)),
        "the minted line states the index model:\n{}",
        whats_left(&composed.text),
    );
    stage_foreign(&corpus);
    let doc = author_inconsistency(&corpus, &composed);
    let before = commit_count(&corpus);

    let out = finalize(&corpus, &composed);
    assert!(out.status.success(), "the finalize lands; {}", text(&out));
    assert_eq!(commit_count(&corpus), before + 1, "one commit");
    let files = head_files(&corpus);
    assert!(
        files.iter().any(|path| path == FOREIGN),
        "the index model commits the staged foreign path, as the text promised; \
         landed {files:?} for {doc}",
    );
}

/// **(2)** The reverse: a `replace-step` of `park-idea`'s `finalize` with the doc-only step
/// composes the doc-only model, and the finalize lands the idea alone, the foreign path
/// still staged.
#[test]
fn a_replace_step_into_the_doc_only_step_composes_and_commits_path_scoped() {
    let (workflow, target, wrapper, model) = REPLACE_CELLS[1];
    let corpus = with_replace(target, wrapper);
    let composed = start(&corpus, workflow, "park a thought");
    assert!(
        whats_left(&composed.text).contains(&whats_left_coverage(model)),
        "the minted line states the doc-only model:\n{}",
        whats_left(&composed.text),
    );
    stage_foreign(&corpus);
    let doc = author_idea(&corpus, &composed.task);
    fill_commit(&corpus, &composed, "ideas");

    let out = finalize(&corpus, &composed);
    assert!(out.status.success(), "the finalize lands; {}", text(&out));
    assert_eq!(
        head_files(&corpus),
        vec![doc],
        "the doc-only model commits the idea alone, as the text promised",
    );
    assert_eq!(
        corpus.git(&["diff", "--cached", "--name-only"]).trim(),
        FOREIGN,
        "the foreign path is still staged",
    );
}

/// **(3)** Under either delta the resumed task's composed `text` is its minted `text`, and
/// its `what's-left:` line states the model the minted one did. The resume read the pack's
/// include list.
#[test]
fn a_resumed_task_composes_its_minted_text_under_a_structural_delta() {
    for (workflow, target, wrapper, model) in REPLACE_CELLS {
        let corpus = with_replace(target, wrapper);
        let (task, minted) = json_text(&corpus.jigc_ok(&[
            "--format",
            "json",
            "start",
            "--workflow",
            workflow,
            "resume me",
        ]));
        let task = task.expect("a minted task id");
        let (_, resumed) =
            json_text(&corpus.jigc_ok(&["--format", "json", "start", "--task", &task]));
        assert_eq!(
            resumed, minted,
            "[{workflow}] the resumed text is the minted text"
        );
        let line = whats_left(&corpus.jigc_ok(&["start", "--task", &task]));
        assert!(
            line.contains(&whats_left_coverage(model)),
            "[{workflow}] the resumed what's-left line states the minted model:\n{line}",
        );
    }
}

/// `(task, text)` of a `--format json` compose.
fn json_text(stdout: &str) -> (Option<String>, String) {
    let value: serde_json::Value = serde_json::from_str(stdout).expect("valid JSON");
    (
        value["task"].as_str().map(str::to_owned),
        value["text"].as_str().expect("a text string").to_owned(),
    )
}

/// **(4)** The verb-minted door: an `insert-step` into `amend` composes the inserted step on
/// `jigc task amend`, and the resume composes the same text.
#[test]
fn an_insert_step_into_amend_composes_at_the_verb_and_on_resume() {
    let corpus = TrialCorpus::build(State::Fresh);
    let source = step_source(&corpus, AMEND_NOTE);
    corpus.jigc_ok(&[
        "config",
        "insert-step",
        "--workflow",
        "amend",
        "--after",
        "amend-message",
        &source,
    ]);
    commit_config(&corpus);

    let (task, minted) =
        json_text(&corpus.jigc_ok(&["--format", "json", "task", "amend", "repair the message"]));
    assert!(
        minted.contains(AMEND_NOTE_SENTENCE),
        "the verb-minted compose applies the project's insert-step:\n{minted}",
    );
    let task = task.expect("a minted task id");
    let (_, resumed) = json_text(&corpus.jigc_ok(&["--format", "json", "start", "--task", &task]));
    assert_eq!(resumed, minted, "the resumed text is the minted text");
}

/// **(5)** The review hold: a `replace-step` of `single-task`'s `finalize` with a step that
/// wraps `step:migration-finalize` composes the exit-4 promise, and the plain finalize keeps
/// it. The hold's predicate read the pack's include list, so it landed at exit 0.
#[test]
fn a_replace_step_into_the_review_hold_holds() {
    let corpus = with_replace("workflow:single-task#finalize", HOLD_WRAPPER);
    let task = corpus.start_workflow("single-task", "hold for review");
    let composed = corpus.jigc_ok(&["start", "--task", &task]);
    assert!(
        composed.contains("commits NOTHING"),
        "the delta composes the hold's promise:\n{composed}",
    );
    let address = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Held Decision",
            "--task",
            &task,
        ])
        .trim()
        .to_owned();
    for (slot, prose) in [
        ("context", "The forces that made the decision necessary."),
        ("decision", "We will hold every migration for review."),
        ("consequences", "Every migration waits for a reviewer."),
    ] {
        corpus.set_slot(&format!("{address}#{slot}"), &task, prose);
    }
    corpus.set_field(&format!("commit:{task}#type"), &task, "docs");
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "hold for review");

    let before = corpus.git(&["rev-parse", "HEAD"]);
    let held = corpus.jigc(&["task", "finalize", &task]);
    assert_eq!(
        held.status.code(),
        Some(4),
        "the composed promise is kept; {}",
        text(&held),
    );
    assert_eq!(corpus.git(&["rev-parse", "HEAD"]), before, "HEAD unmoved");
}
