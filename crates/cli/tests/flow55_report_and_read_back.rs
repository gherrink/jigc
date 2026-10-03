//! M55 Increment 11 / T1 — **flow 55 (A): report and read back, then triage — one corpus,
//! the whole lifecycle, on the shipped workflows** (`design/worked-examples.md` → 55;
//! `design/findings-channel.md` §11 → A).
//!
//! One `State::Fresh` corpus walks the lifecycle in order, every write a line the composed
//! workflow emitted, run verbatim but for its `<…>` fills (`findings_workflows`' helpers,
//! widened to `pub(crate)` and reused, never copied):
//!
//! 1. `report-jigc-feedback`, started by name, emits a create line that carries no flag
//!    beyond `--title` and `--task` — the gate entry's `new: true` does the create-only work.
//!    `kind`, `found-in`, `jigc-version`, `description` and a fenced `repro` with a `#`-led
//!    line are written.
//! 2. A foreign path is staged with plain git **after** the report started. The finalize
//!    exits 0, its commit holds only the doc, and the foreign path stays staged — and stays
//!    staged through every later landing of the walk.
//! 3. `doc list --format json` and `doc show --format json` carry the title and
//!    `fields.status == "open"`; with `status:` hand-deleted and committed by plain git,
//!    both still project `"open"`.
//! 4. `report-inconsistency` is reached from the bare `jigc start` catalog — its id read off
//!    the catalog line — and lands alone with three sides, each added by `add-item --slug`.
//!    The catalog lists none of the three hidden workflows, and each composes by name.
//! 5. `triage-jigc-feedback`, named in plain words, moves **the row this corpus filed** —
//!    read back through the real read surface, its `status:` line hand-deleted — to
//!    `resolved` with a `resolution` and a `pinned-by`, and lands that doc alone; the
//!    `doc list` row then reads `resolved`.
//!
//! **What this adds over `findings_workflows`**, whose arms (a), (b), (c) and (e) each prove
//! one of these steps in a corpus of its own: the triage reads the row the report filed,
//! through the real read surface between them, over the hand edit the read-back step made —
//! not a row filed in a separate arm.
//!
//! **Red** is the mutant the doc-only step exists against: a project shadow of
//! `report-jigc-feedback` whose body includes `step:finalize` in place of
//! `step:finalize-doc-only` sweeps the foreign staged path into the report's commit, and the
//! one-doc assertion of step 2 fails.

use crate::findings_workflows::{
    Composed, FOREIGN, HIDDEN, RESOLUTION, SIDES, TITLE, VISIBLE, argv, assert_lands_alone,
    assert_reads_open, assert_triage_text, author_inconsistency, author_jigc_feedback, catalog,
    commit_count, emitted_line, fill_commit, finalize, head_files, listed_row, minted, run_emitted,
    shown, slug_of, start, start_triage, text,
};
use crate::support::trial_corpus::{State, TrialCorpus};

use std::fs;

use serde_json::json;

/// The test this flow's triage names as pinning the fix it resolves.
const PINNED_BY: &str =
    "flow55_report_and_read_back::one_corpus_files_reads_back_and_triages_its_own_finding";

/// Run `composed`'s emitted finalize and assert it lands exactly one commit holding the doc
/// at `address` (in `ty`'s store) alone, the foreign path still staged. Returns that doc's
/// committed path, read back off `doc list`.
fn land_alone(corpus: &TrialCorpus, composed: &Composed, ty: &str, address: &str) -> String {
    let before = commit_count(corpus);
    let out = finalize(corpus, composed);
    assert!(
        out.status.success(),
        "`{}`'s emitted finalize lands at exit 0; {}",
        composed.task,
        text(&out),
    );
    assert_eq!(commit_count(corpus), before + 1, "exactly one commit");
    let path = listed_row(corpus, ty, address)["path"]
        .as_str()
        .expect("a row has a path")
        .to_owned();
    assert_eq!(
        head_files(corpus),
        vec![path.clone()],
        "`{}`'s commit holds `{address}` alone; {}",
        composed.task,
        text(&out),
    );
    assert_eq!(
        corpus.git(&["diff", "--cached", "--name-only"]),
        FOREIGN,
        "the foreign path stays staged for the task it belongs to",
    );
    path
}

/// The whole lifecycle — report, read back, a hand edit, the router-visible report, and
/// the triage of the row this corpus filed — in one corpus, on the shipped workflows.
#[test]
fn one_corpus_files_reads_back_and_triages_its_own_finding() {
    let corpus = TrialCorpus::build(State::Fresh);

    // ── 1 · report-jigc-feedback by name; its create line carries no flag. ──
    let (report, address) = author_jigc_feedback(&corpus, TITLE);
    let create = emitted_line(&report.text, "jigc doc create jigc-feedback ");
    let flags: Vec<&str> = create
        .split_whitespace()
        .filter(|word| word.starts_with("--"))
        .collect();
    assert_eq!(
        flags,
        ["--title", "--task"],
        "the emitted create carries no create-only flag — the gate entry's `new: true` \
         does that work; got `{create}`",
    );

    // ── 2 · a foreign path staged after the report started stays out of its commit. ──
    fs::create_dir_all(corpus.repo().join("src")).expect("mk src/");
    fs::write(corpus.repo().join(FOREIGN), "pub fn other_task() {}\n").expect("foreign file");
    corpus.git(&["add", "--", FOREIGN]);
    let path = land_alone(&corpus, &report, "jigc-feedback", &address);

    // ── 3 · read back open; still open once `status:` is hand-deleted and committed. ──
    assert_reads_open(&corpus, "jigc-feedback", &address, TITLE, "as filed");
    let committed = fs::read_to_string(corpus.repo().join(&path)).expect("read the finding");
    let statusless: String = committed
        .split_inclusive('\n')
        .filter(|line| !line.starts_with("status:"))
        .collect();
    assert_ne!(statusless, committed, "`{path}` carried a `status:` line");
    fs::write(corpus.repo().join(&path), &statusless).expect("hand edit");
    corpus.git(&["add", "--", &path]);
    corpus.git(&[
        "commit",
        "-q",
        "-m",
        "drop the status line by hand",
        "--",
        &path,
    ]);
    assert_reads_open(
        &corpus,
        "jigc-feedback",
        &address,
        TITLE,
        "status hand-deleted",
    );

    // ── 4 · report-inconsistency from the catalog; the hidden three compose by name. ──
    let (router, ids) = catalog(&corpus);
    let chosen = ids
        .iter()
        .find(|id| *id == VISIBLE)
        .unwrap_or_else(|| panic!("the catalog lists `{VISIBLE}`; got:\n{router}"));
    for hidden in HIDDEN {
        assert!(
            !ids.iter().any(|id| id == hidden),
            "the catalog leaves `{hidden}` out; got:\n{router}",
        );
    }
    let start_line = argv(
        &emitted_line(&router, "jigc start --workflow <chosen>"),
        &[
            ("<chosen>", chosen),
            ("<intent>", "the cache docs disagree on eviction"),
        ],
    );
    let args: Vec<&str> = start_line[1..].iter().map(String::as_str).collect();
    let reached = corpus.jigc_ok(&args);
    let reporter = Composed {
        task: minted(&reached),
        text: reached,
        cwd: corpus.repo(),
    };
    let record = author_inconsistency(&corpus, &reporter);
    land_alone(&corpus, &reporter, "inconsistency", &record);
    let row = listed_row(&corpus, "inconsistency", &record);
    assert_eq!(
        (&row["item-count"], &row["fields"]["status"]),
        (&json!(SIDES.len()), &json!("open")),
        "three sides, filed open; row:\n{row:#}",
    );
    let by_name = start(
        &corpus,
        "triage-inconsistency",
        "reach triage-inconsistency by name",
    );
    assert!(
        by_name
            .text
            .contains(&format!("jigc task finalize {}", by_name.task)),
        "`triage-inconsistency` composes by name to a finalizing task; got:\n{}",
        by_name.text,
    );

    // ── 5 · triage the row this corpus filed, read back through the real surface. ──
    let filed = fs::read_to_string(corpus.repo().join(&path)).expect("read the finding");
    let triage = start_triage(&corpus, "triage-jigc-feedback", &address);
    assert_triage_text(
        &triage,
        "jigc-feedback",
        "four leaves — `status`, `duplicate-of`, `pinned-by` and `resolution`",
    );
    let slug = slug_of(&address);
    let first = run_emitted(
        &corpus,
        &triage,
        "jigc doc set-field jigc-feedback:<slug>#meta/status ",
        &[
            ("<slug>", &slug),
            ("<resolved|declined|duplicate|refuted>", "resolved"),
        ],
        None,
    );
    assert!(
        first.contains("copied in for update"),
        "the first write acks the committed finding's copy-in; got:\n{first}",
    );
    run_emitted(
        &corpus,
        &triage,
        "jigc doc set-field jigc-feedback:<slug>#meta/pinned-by ",
        &[("<slug>", &slug), ("<module>::<test_name>", PINNED_BY)],
        None,
    );
    run_emitted(
        &corpus,
        &triage,
        "jigc doc set-slot jigc-feedback:<slug>#resolution ",
        &[("<slug>", &slug)],
        Some(RESOLUTION),
    );
    fill_commit(&corpus, &triage, "feedback");
    assert_lands_alone(&corpus, &triage, &path);
    let row = listed_row(&corpus, "jigc-feedback", &address);
    assert_eq!(
        (
            &row["title"],
            &row["fields"]["status"],
            &row["fields"]["pinned-by"]
        ),
        (&json!(TITLE), &json!("resolved"), &json!(PINNED_BY)),
        "the row this corpus filed now reads resolved and pinned; row:\n{row:#}",
    );
    assert!(
        shown(&corpus, &address).to_string().contains(RESOLUTION),
        "the committed finding carries its resolution",
    );
    let triaged = fs::read_to_string(corpus.repo().join(&path)).expect("re-read");
    assert!(
        filed
            .lines()
            .all(|line| triaged.lines().any(|kept| kept == line)),
        "every line of the hand-edited finding is kept by the triage; filed:\n{filed}\n\
         triaged:\n{triaged}",
    );
}
