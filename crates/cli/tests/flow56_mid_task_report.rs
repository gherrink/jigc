//! M55 Increment 11 / T2 — **flow 56 (B): mid-code-task reporting — the three staging
//! states, on the shipped `report-jigc-feedback`** (`design/worked-examples.md` → 56;
//! `design/findings-channel.md` §3, §11 → B).
//!
//! A `dev-task` is open in the same checkout, holding an edit to a tracked file and a new
//! file, while a finding is filed through the shipped `report-jigc-feedback`. Every write the
//! report and triage tasks make is a line their composed workflow emitted, run verbatim but
//! for its `<…>` fills (`findings_workflows`' helpers, reused, never copied):
//!
//! | state | the code task's files | the report's emitted finalize |
//! |---|---|---|
//! | 1 | unstaged throughout | lands the doc alone, exit 0 |
//! | 2 | staged **before** the report started | lands the doc alone, exit 0 — and again with `--carry-staged` appended, which is inert |
//! | 3 | staged **after** the report started | lands the doc alone, exit 0 |
//!
//! In every state the code task's `git diff --cached --name-status` is the same before and
//! after the report's finalize, the landed text names each of the code task's paths in its
//! left-out section, and the code task then lands its own two files at exit 0. Also driven:
//! a pending `.jigc/config` delta stays out of the report's commit (G8), and the **triage
//! edit shape** — the shipped `triage-jigc-feedback` editing a committed finding lands that
//! doc alone in each of the three states.
//!
//! **The omitting context** is the shipped `park-idea`, which composes the ordinary
//! `step:finalize`: over the same three states it behaves as it always has — state 2 refuses
//! `finalize.carried-staged` at exit 3, and state 3's commit carries the code task's files.
//!
//! **What this adds over `doc_only_finalize`**, which proves the same three states (M55
//! Increment 1): that suite drives a fixture workflow composing the shipped step and writes
//! through hand-built commands; this one drives the shipped report and triage workflows
//! through their emitted lines (§13's last row). It claims no first proof.
//!
//! **Red** is the mutant the doc-only step exists against: with `report-jigc-feedback`'s
//! body on `step:finalize` in place of `step:finalize-doc-only`, state 2's finalize is
//! refused `finalize.carried-staged` at exit 3 and the first assertion fails.

use crate::findings_workflows::{
    Composed, RESOLUTION, TITLE, argv, author_jigc_feedback, commit_count, emitted_line,
    fill_commit, finalize, head_files, listed_row, run_emitted, slug_of, start, start_triage, text,
};
use crate::support::trial_corpus::{State, TrialCorpus};

use std::fs;
use std::process::Output;

/// The code task's tracked file and its new one.
const TRACKED: &str = "src/lib.rs";
const NEW_FILE: &str = "src/new.rs";

/// Which of flow B's three states the code task's files are in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Staging {
    /// State 1 — unstaged throughout.
    Unstaged,
    /// State 2 — staged before the report task was minted.
    Before,
    /// State 3 — staged after the report task was minted.
    After,
}

const STATES: [Staging; 3] = [Staging::Unstaged, Staging::Before, Staging::After];

/// Flow B's three states, and state 2 again with `--carry-staged` (inert on this arm).
fn cells() -> impl Iterator<Item = (Staging, bool)> {
    STATES
        .iter()
        .map(|staging| (*staging, false))
        .chain([(Staging::Before, true)])
}

/// A `State::Fresh` corpus — jigc's own `pre-commit` hook asserted installed — with the code
/// task's tracked file committed.
fn corpus() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let hook = corpus.repo().join(".git/hooks/pre-commit");
    assert!(
        hook.is_file(),
        "the premise: `jigc setup` installed its pre-commit hook at {hook:?}",
    );
    fs::create_dir_all(corpus.repo().join("src")).expect("mk src/");
    fs::write(corpus.repo().join(TRACKED), "pub fn one() {}\n").expect("write the code file");
    corpus.git(&["add", "--", TRACKED]);
    corpus.git(&["commit", "-q", "-m", "chore: the code"]);
    corpus
}

/// Open the shipped `dev-task` and make its two edits — in the tree, not staged.
fn open_code_task(corpus: &TrialCorpus) -> Composed {
    let code = start(corpus, "dev-task", "change the code");
    fs::write(corpus.repo().join(TRACKED), "pub fn one() { two() }\n").expect("edit");
    fs::write(corpus.repo().join(NEW_FILE), "pub fn two() {}\n").expect("new file");
    code
}

fn stage_code(corpus: &TrialCorpus) {
    corpus.git(&["add", "--", TRACKED, NEW_FILE]);
}

/// The code task's view of the index — what must be identical before and after the
/// report's finalize.
fn cached(corpus: &TrialCorpus) -> String {
    corpus.git(&["diff", "--cached", "--name-status"])
}

/// Run `composed`'s emitted finalize line verbatim, `--carry-staged` appended when `carry`.
fn run_finalize(corpus: &TrialCorpus, composed: &Composed, carry: bool) -> Output {
    let mut words = argv(
        &emitted_line(
            &composed.text,
            &format!("jigc task finalize {}", composed.task),
        ),
        &[],
    );
    if carry {
        words.push("--carry-staged".to_owned());
    }
    let args: Vec<&str> = words[1..].iter().map(String::as_str).collect();
    corpus.jigc_stdin_from(&composed.cwd, &args, "")
}

/// The committed path of the doc at `address` in `ty`'s store, read off `doc list`.
fn committed_path(corpus: &TrialCorpus, ty: &str, address: &str) -> String {
    listed_row(corpus, ty, address)["path"]
        .as_str()
        .expect("a row has a path")
        .to_owned()
}

/// Assert the landed text's left-out section — after the `finalized` line — names each of
/// the code task's paths, staged or not.
fn assert_left_out_named(out: &Output, label: &str) {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let landed = stdout
        .split_once("\nfinalized ")
        .unwrap_or_else(|| panic!("{label} a landed summary; {}", text(out)))
        .1;
    let section = landed
        .split_once("  left-out (")
        .unwrap_or_else(|| panic!("{label} a left-out section; {}", text(out)))
        .1;
    for path in [TRACKED, NEW_FILE] {
        assert!(
            section.contains(path),
            "{label} the left-out section names `{path}`; {}",
            text(out),
        );
    }
}

/// The code task fills its commit doc through its emitted lines and lands its own two files
/// at exit 0 — the second half of every state.
fn code_task_lands_its_own_files(corpus: &TrialCorpus, code: &Composed, label: &str) {
    if cached(corpus).is_empty() {
        stage_code(corpus);
    }
    fill_commit(corpus, code, "code");
    let out = finalize(corpus, code);
    assert!(
        out.status.success(),
        "{label} the code task then finalizes its own files at exit 0; {}",
        text(&out),
    );
    let mut files = head_files(corpus);
    files.sort();
    assert_eq!(
        files,
        vec![TRACKED.to_owned(), NEW_FILE.to_owned()],
        "{label} the code task's commit holds its own files",
    );
}

/// **(1)** Flow B's three states on the shipped `report-jigc-feedback`, and state 2 again
/// with `--carry-staged`: each emitted finalize lands one commit holding the finding alone
/// at exit 0, the code task's index unchanged and its paths narrated as left out, and the
/// code task then lands its own files.
#[test]
fn every_state_lands_the_report_doc_alone_on_the_shipped_workflow() {
    for (staging, carry) in cells() {
        let label = format!("[{staging:?}, carry={carry}]");
        let corpus = corpus();
        let code = open_code_task(&corpus);
        if staging == Staging::Before {
            stage_code(&corpus);
        }
        let (report, address) = author_jigc_feedback(&corpus, TITLE);
        if staging == Staging::After {
            stage_code(&corpus);
        }
        let before_index = cached(&corpus);
        let before_count = commit_count(&corpus);

        let out = run_finalize(&corpus, &report, carry);
        assert!(
            out.status.success(),
            "{label} the report's emitted finalize lands at exit 0; {}",
            text(&out),
        );
        assert_eq!(
            commit_count(&corpus),
            before_count + 1,
            "{label} exactly one commit",
        );
        let path = committed_path(&corpus, "jigc-feedback", &address);
        assert_eq!(
            head_files(&corpus),
            vec![path],
            "{label} the report's commit holds its finding alone; {}",
            text(&out),
        );
        assert_eq!(
            cached(&corpus),
            before_index,
            "{label} the code task's index is what it was",
        );
        assert_left_out_named(&out, &label);
        code_task_lands_its_own_files(&corpus, &code, &label);
    }
}

/// **(2)** A pending `.jigc/config` delta is outside the doc-only path set (G8): state 3's
/// report commit does not carry it, and it is still pending afterwards.
#[test]
fn a_pending_config_delta_stays_out_of_the_reports_commit() {
    let corpus = corpus();
    let _code = open_code_task(&corpus);
    let (report, address) = author_jigc_feedback(&corpus, TITLE);
    stage_code(&corpus);
    corpus.jigc_ok(&[
        "config",
        "set",
        "validation.doc-code.title-names-symbol.severity",
        "advisory",
    ]);
    let pending = || {
        corpus.git(&[
            "status",
            "--porcelain",
            "--untracked-files=all",
            "--",
            ".jigc/config",
        ])
    };
    let before = pending();
    assert!(
        !before.is_empty(),
        "the premise: a `.jigc/config` delta is pending"
    );

    let out = run_finalize(&corpus, &report, false);
    assert!(out.status.success(), "the report lands; {}", text(&out));
    assert_eq!(
        head_files(&corpus),
        vec![committed_path(&corpus, "jigc-feedback", &address)],
        "the report's commit carries no config delta",
    );
    assert_eq!(pending(), before, "the config delta is still pending");
}

/// **(3)** The triage edit shape: in each state, the shipped `triage-jigc-feedback` moves a
/// committed finding to `resolved` through its emitted lines and lands that doc alone — the
/// code task's index unchanged — and the code task then lands its own files.
#[test]
fn a_triage_lands_the_edited_finding_alone_in_every_state() {
    for staging in STATES {
        let label = format!("[{staging:?}]");
        let corpus = corpus();
        let (report, address) = author_jigc_feedback(&corpus, TITLE);
        let filed = run_finalize(&corpus, &report, false);
        assert!(
            filed.status.success(),
            "{label} the report lands; {}",
            text(&filed)
        );
        let path = committed_path(&corpus, "jigc-feedback", &address);

        let code = open_code_task(&corpus);
        if staging == Staging::Before {
            stage_code(&corpus);
        }
        let triage = start_triage(&corpus, "triage-jigc-feedback", &address);
        if staging == Staging::After {
            stage_code(&corpus);
        }
        let slug = slug_of(&address);
        run_emitted(
            &corpus,
            &triage,
            "jigc doc set-field jigc-feedback:<slug>#meta/status ",
            &[
                ("<slug>", &slug),
                ("<resolved|declined|duplicate|refuted>", "resolved"),
            ],
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
        let before_index = cached(&corpus);
        let before_count = commit_count(&corpus);

        let out = run_finalize(&corpus, &triage, false);
        assert!(
            out.status.success(),
            "{label} the triage's emitted finalize lands at exit 0; {}",
            text(&out),
        );
        assert_eq!(
            commit_count(&corpus),
            before_count + 1,
            "{label} exactly one commit",
        );
        assert_eq!(
            head_files(&corpus),
            vec![path.clone()],
            "{label} the triage's commit holds the edited finding alone; {}",
            text(&out),
        );
        let landed = corpus.git(&["show", &format!("HEAD:{path}")]);
        assert!(
            landed.contains("status: resolved") && landed.contains(RESOLUTION),
            "{label} the edit is what landed; got:\n{landed}",
        );
        assert_eq!(
            cached(&corpus),
            before_index,
            "{label} the code task's index is what it was",
        );
        code_task_lands_its_own_files(&corpus, &code, &label);
    }
}

/// **(4) The omitting context.** The shipped `park-idea` composes the ordinary
/// `step:finalize`, so over the same three states it behaves as it always has: state 1 lands
/// the doc alone, state 2 refuses `finalize.carried-staged` at exit 3 committing nothing, and
/// state 3 sweeps the code task's staged files into its commit.
#[test]
fn park_idea_over_the_same_states_commits_the_index() {
    for staging in STATES {
        let label = format!("[{staging:?}]");
        let corpus = corpus();
        let _code = open_code_task(&corpus);
        if staging == Staging::Before {
            stage_code(&corpus);
        }
        let park = start(&corpus, "park-idea", "park a thought");
        if staging == Staging::After {
            stage_code(&corpus);
        }
        let address = run_emitted(
            &corpus,
            &park,
            "jigc doc create idea ",
            &[("<TITLE>", "A Parked Thought")],
            None,
        )
        .trim()
        .to_owned();
        let slug = slug_of(&address);
        run_emitted(
            &corpus,
            &park,
            "jigc doc set-field idea:<slug>#trigger ",
            &[
                ("<slug>", &slug),
                ("<what would resurface it>", "a report comes back"),
            ],
            None,
        );
        run_emitted(
            &corpus,
            &park,
            "jigc doc set-slot idea:<slug>#description ",
            &[("<slug>", &slug)],
            Some("One thought, parked while other work is open."),
        );
        fill_commit(&corpus, &park, "ideas");
        let before_count = commit_count(&corpus);

        let out = run_finalize(&corpus, &park, false);
        match staging {
            Staging::Unstaged => {
                assert!(out.status.success(), "{label} lands; {}", text(&out));
                assert_eq!(
                    head_files(&corpus),
                    vec![committed_path(&corpus, "idea", &address)],
                    "{label} nothing staged, so the index is the doc alone",
                );
            }
            Staging::Before => {
                assert_eq!(
                    out.status.code(),
                    Some(3),
                    "{label} the carryover gate refuses; {}",
                    text(&out),
                );
                assert!(
                    String::from_utf8_lossy(&out.stderr).contains("finalize.carried-staged"),
                    "{label} refused as finalize.carried-staged; {}",
                    text(&out),
                );
                assert_eq!(
                    commit_count(&corpus),
                    before_count,
                    "{label} nothing committed"
                );
            }
            Staging::After => {
                assert!(out.status.success(), "{label} lands; {}", text(&out));
                let mut files = head_files(&corpus);
                files.sort();
                let mut expected = vec![
                    committed_path(&corpus, "idea", &address),
                    NEW_FILE.to_owned(),
                    TRACKED.to_owned(),
                ];
                expected.sort();
                assert_eq!(
                    files, expected,
                    "{label} the ordinary model commits the index, the code task's files included",
                );
            }
        }
    }
}
