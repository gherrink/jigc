//! M55 Increment 1 / T1 — **the doc-only finalize**: a code-less task whose recorded
//! composed workflow includes `step:finalize-doc-only` commits exactly its own docs and
//! recorded owner-artifacts, path-scoped, and nothing else
//! (`design/findings-channel.md` §3; `design/finalize.md` → The doc-only arm).
//!
//! **Flow B's three states** (§11) are driven through the real binary with jigc's own
//! `pre-commit` hook installed (`jigc setup`'s, asserted present), by a fixture code-less
//! workflow composing the shipped step over `[dev ▸ methodology]`, beside an open
//! `dev-task` holding an edit to a tracked file and a new file:
//!
//! | state | the code task's files | the report's finalize |
//! |---|---|---|
//! | 1 | unstaged throughout | lands the doc alone, exit 0 |
//! | 2 | staged **before** the report was minted | lands the doc alone, exit 0 — with and without `--carry-staged` (it was exit 3, `finalize.carried-staged`) |
//! | 3 | staged **after** the report was minted | lands the doc alone, exit 0 (it swept the code task's files in) |
//!
//! In every state the code task's `git diff --cached --name-status` is the same before and
//! after, and the code task then finalizes its own files at exit 0.
//!
//! **The omitting context** is `park-idea`, which composes the ordinary `step:finalize`:
//! over the same three states it behaves as it always has — state 2 refuses
//! `finalize.carried-staged` at exit 3, and state 3's commit carries the code files.
//!
//! **What stayed out is narrated on this commit model's spelling** (T2,
//! `cli::render::CommitModel::DocOnly`): in every state the landed left-out section and
//! `committed.left_out` name each of the code task's paths — staged ones included, one entry
//! per path — the pre-commit advisory says the commit is path-scoped and never *git add*, and
//! the `--dry-run` forecast is the path set, not the index.
//!
//! Every arm asserts on the landed git commit and the binary's emitted bytes, never on a
//! reconstruction.

use crate::support;

use std::fs;

use support::run_then_parse::stdout_json;
use support::trial_corpus::{State, TrialCorpus};

/// The fixture report workflow: code-less, granting `idea`, composing the shipped
/// `step:finalize-doc-only` in place of `step:finalize`. A project-layer definition, so it
/// composes over the default `[dev ▸ methodology]` cascade `jigc setup` writes.
const REPORT_WORKFLOW: &str = "\
---
when: file one finding as a doc while other work is open in the checkout
description: A fixture code-less report workflow — one idea doc, committed path-scoped.
usage: the fixture cell for the doc-only finalize, reached by name.
creates-task: true
selectable: false
suppressed:
  reason: fixture-only — the doc-only finalize cell, reached by name
  expires: never
allows-create:
  - { type: idea, as: idea }
---
{{ include: step:author-commit }}
{{ include: step:finalize-doc-only }}
";

const REPORT_WORKFLOW_ID: &str = "file-report";

/// The code task's tracked file and its new one.
const TRACKED: &str = "src/lib.rs";
const NEW_FILE: &str = "src/new.rs";

/// Which of Flow B's three states the code task's files are in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Staging {
    /// Unstaged throughout.
    Unstaged,
    /// Staged before the report task was minted.
    Before,
    /// Staged after the report task was minted.
    After,
}

const STATES: [Staging; 3] = [Staging::Unstaged, Staging::Before, Staging::After];

/// Flow B's three states, and state 2 again under `--carry-staged` (inert on this arm).
fn cells() -> impl Iterator<Item = (Staging, bool)> {
    STATES
        .iter()
        .map(|staging| (*staging, false))
        .chain([(Staging::Before, true)])
}

/// The doc-only model's pre-commit advisory stem, as the binary prints it.
const DOC_ONLY_STEM: &str = "finalize — about to commit only this task's docs, path-scoped; \
                             leaving out:";

/// The ordinary model's stem — what the omitting context still prints.
const INDEX_STEM: &str = "finalize — about to commit the index; leaving out:";

/// The paths of a manifest-entry array, in order.
fn entry_paths(entries: &serde_json::Value) -> Vec<String> {
    entries
        .as_array()
        .unwrap_or_else(|| panic!("a manifest-entry array; got {entries}"))
        .iter()
        .map(|entry| {
            entry["path"]
                .as_str()
                .unwrap_or_else(|| panic!("an entry path; got {entry}"))
                .to_owned()
        })
        .collect()
}

/// The paths the **landed** text's left-out section lists — the indented lines under the
/// `  left-out (` header that follows the `finalized ` line, read off the emitted bytes.
fn landed_left_out_section(stdout: &str) -> Vec<String> {
    let landed = stdout
        .split_once("\nfinalized ")
        .unwrap_or_else(|| panic!("a landed summary; got:\n{stdout}"))
        .1;
    let mut lines = landed
        .lines()
        .skip_while(|line| !line.starts_with("  left-out ("));
    let header = lines
        .next()
        .unwrap_or_else(|| panic!("a left-out section; got:\n{landed}"));
    assert!(
        !header.contains("git add"),
        "the doc-only left-out header never routes at `git add`; got: {header}",
    );
    lines
        .map_while(|line| line.strip_prefix("    "))
        .map(str::to_owned)
        .collect()
}

/// A corpus with the fixture workflow and the code task's tracked file committed, jigc's
/// own `pre-commit` hook asserted installed.
fn corpus() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let hook = corpus.repo().join(".git/hooks/pre-commit");
    assert!(
        hook.is_file(),
        "the premise: `jigc setup` installed its pre-commit hook at {hook:?}",
    );
    let workflows = corpus.repo().join(".jigc/config/workflows");
    fs::create_dir_all(&workflows).expect("mk the project workflows dir");
    fs::write(
        workflows.join(format!("{REPORT_WORKFLOW_ID}.yaml")),
        REPORT_WORKFLOW,
    )
    .expect("write the fixture workflow");
    fs::create_dir_all(corpus.repo().join("src")).expect("mk src/");
    fs::write(corpus.repo().join(TRACKED), "pub fn one() {}\n").expect("write the code file");
    corpus.git(&["add", "--", ".jigc/config/workflows", TRACKED]);
    corpus.git(&[
        "commit",
        "-q",
        "-m",
        "chore: the fixture workflow and the code",
    ]);
    corpus
}

/// Open the code task and make its two edits — in the tree, not staged.
fn open_code_task(corpus: &TrialCorpus) -> String {
    let task = corpus.start_workflow("dev-task", "change the code");
    fs::write(corpus.repo().join(TRACKED), "pub fn one() { two() }\n").expect("edit");
    fs::write(corpus.repo().join(NEW_FILE), "pub fn two() {}\n").expect("new file");
    task
}

fn stage_code(corpus: &TrialCorpus) {
    corpus.git(&["add", "--", TRACKED, NEW_FILE]);
}

/// The code task's view of the index — what must be identical before and after the
/// report's finalize.
fn cached(corpus: &TrialCorpus) -> String {
    corpus.git(&["diff", "--cached", "--name-status"])
}

/// The paths HEAD's commit changed, one per line.
fn head_files(corpus: &TrialCorpus) -> Vec<String> {
    corpus
        .git(&["show", "--name-only", "--pretty=format:", "HEAD"])
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

fn commit_count(corpus: &TrialCorpus) -> usize {
    corpus
        .git(&["rev-list", "--count", "HEAD"])
        .parse()
        .expect("a count")
}

/// Author a fresh `idea` in `task` and fill its commit doc; returns the idea's promoted
/// path, from the address the binary **emitted**.
fn author_idea(corpus: &TrialCorpus, task: &str, title: &str) -> String {
    let address = corpus
        .jigc_ok(&["doc", "create", "idea", "--title", title, "--task", task])
        .trim()
        .to_string();
    corpus.set_field(&format!("{address}#trigger"), task, "a report comes back");
    corpus.set_slot(
        &format!("{address}#description"),
        task,
        "One finding, filed while other work is open.",
    );
    fill_commit(corpus, task);
    let slug = address
        .strip_prefix("idea:")
        .unwrap_or_else(|| panic!("an idea address; got {address}"));
    // `idea`'s `location: ideas/` resolves under the `docs-root` knob, `docs/` in this
    // corpus's pack-default.
    format!("docs/ideas/{slug}.md")
}

fn fill_commit(corpus: &TrialCorpus, task: &str) {
    corpus.set_field(&format!("commit:{task}#type"), task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), task, "ideas");
    corpus.set_slot(&format!("commit:{task}#summary"), task, "file a finding");
    corpus.set_slot(&format!("commit:{task}#body"), task, "A filed finding.");
}

fn text(out: &std::process::Output) -> String {
    format!(
        "--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// The code task finalizes its own two files at exit 0 — the second half of every state.
fn code_task_lands_its_own_files(corpus: &TrialCorpus, code: &str, staging: Staging) {
    if staging == Staging::Unstaged {
        stage_code(corpus);
    }
    fill_commit(corpus, code);
    let out = corpus.jigc(&["task", "finalize", code]);
    assert!(
        out.status.success(),
        "[{staging:?}] the code task then finalizes its own files at exit 0; {}",
        text(&out),
    );
    let mut files = head_files(corpus);
    files.sort();
    assert_eq!(
        files,
        vec![TRACKED.to_owned(), NEW_FILE.to_owned()],
        "[{staging:?}] the code task's commit holds its own files",
    );
}

/// Build one state up to the report's finalize: the code task open with its edits, the
/// report task minted and authored, the code staged where the state says.
fn arrange(staging: Staging) -> (TrialCorpus, String, String, String) {
    let corpus = corpus();
    let code = open_code_task(&corpus);
    if staging == Staging::Before {
        stage_code(&corpus);
    }
    let report = corpus.start_workflow(REPORT_WORKFLOW_ID, "file a finding");
    if staging == Staging::After {
        stage_code(&corpus);
    }
    let doc = author_idea(&corpus, &report, "Reports Land Alone");
    (corpus, code, report, doc)
}

/// **(1)** Flow B, states 1, 2 and 3 — and state 2 again under `--carry-staged`, which is
/// inert on this arm: each lands one commit holding the report doc alone, at exit 0, the
/// code task's index unchanged, and the code task then lands its own files.
#[test]
fn every_flow_b_state_lands_the_report_doc_alone() {
    for (staging, carry) in cells() {
        let (corpus, code, report, doc) = arrange(staging);
        let before_index = cached(&corpus);
        let before_count = commit_count(&corpus);

        let mut args = vec!["task", "finalize", report.as_str()];
        if carry {
            args.push("--carry-staged");
        }
        let out = corpus.jigc(&args);
        assert!(
            out.status.success(),
            "[{staging:?}, carry={carry}] the report's finalize lands at exit 0; {}",
            text(&out),
        );
        assert_eq!(
            commit_count(&corpus),
            before_count + 1,
            "[{staging:?}, carry={carry}] exactly one commit",
        );
        assert_eq!(
            head_files(&corpus),
            vec![doc.clone()],
            "[{staging:?}, carry={carry}] the report's commit holds its doc alone",
        );
        assert_eq!(
            cached(&corpus),
            before_index,
            "[{staging:?}, carry={carry}] the code task's index is what it was",
        );
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            stdout.contains(DOC_ONLY_STEM),
            "[{staging:?}, carry={carry}] the pre-commit advisory states the path-scoped \
             commit; {}",
            text(&out),
        );
        assert_eq!(
            landed_left_out_section(&stdout),
            vec![TRACKED.to_owned(), NEW_FILE.to_owned()],
            "[{staging:?}, carry={carry}] the landed left-out section names each of the code \
             task's paths, staged or not, once; {}",
            text(&out),
        );
        code_task_lands_its_own_files(&corpus, &code, staging);
    }
}

/// **(1b)** The same states under `--format json`, each forecast first: `committed.left_out`
/// names each of the code task's paths once, the pre-commit advisory prints on stderr, and
/// the `--dry-run` forecast is the path set — its `manifest` paths are the landed commit's
/// paths and its `left_out` is the landed `committed.left_out`, entry for entry.
#[test]
fn every_flow_b_state_forecasts_the_path_set_and_narrates_what_stays() {
    for (staging, carry) in cells() {
        let (corpus, _code, report, _doc) = arrange(staging);
        let label = format!("[{staging:?}, carry={carry}]");
        let mut args = vec!["task", "finalize", report.as_str(), "--format", "json"];
        if carry {
            args.push("--carry-staged");
        }
        let mut dry = args.clone();
        dry.push("--dry-run");
        let forecast: serde_json::Value =
            stdout_json(&corpus.jigc(&dry), &[0], &format!("{label} the forecast"));

        let out = corpus.jigc(&args);
        let landed: serde_json::Value =
            stdout_json(&out, &[0], &format!("{label} the landed finalize"));
        let committed = &landed["committed"];
        let left_out = entry_paths(&committed["left_out"]);
        for path in [TRACKED, NEW_FILE] {
            assert_eq!(
                left_out.iter().filter(|p| *p == path).count(),
                1,
                "{label} committed.left_out names `{path}` exactly once; got {left_out:?}",
            );
        }
        let stderr = String::from_utf8_lossy(&out.stderr);
        let advisory = stderr
            .split_once(DOC_ONLY_STEM)
            .unwrap_or_else(|| panic!("{label} the advisory prints on stderr; {}", text(&out)))
            .1;
        assert!(
            !advisory.contains("git add")
                && advisory.contains(TRACKED)
                && advisory.contains(NEW_FILE),
            "{label} the advisory names each path and never routes at `git add`; {}",
            text(&out),
        );

        let mut forecast_paths = entry_paths(&forecast["manifest"]);
        forecast_paths.sort();
        let mut landed_paths = head_files(&corpus);
        landed_paths.sort();
        assert_eq!(
            forecast_paths, landed_paths,
            "{label} the forecast manifest is the path set the commit took",
        );
        assert_eq!(
            forecast["left_out"], committed["left_out"],
            "{label} the forecast's left-out set is the landed one",
        );
    }
}

/// **(2)** A pending `.jigc/config` delta is outside the path set by construction (G8): it
/// is not in the report's commit and is still pending after it.
#[test]
fn a_pending_config_delta_is_not_in_the_reports_commit() {
    let (corpus, _code, report, doc) = arrange(Staging::After);
    corpus.jigc_ok(&[
        "config",
        "set",
        "validation.doc-code.title-names-symbol.severity",
        "advisory",
    ]);
    let pending = || {
        corpus
            .git(&[
                "status",
                "--porcelain",
                "--untracked-files=all",
                "--",
                ".jigc/config",
            ])
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    let before = pending();
    assert!(
        !before.is_empty(),
        "the premise: a `.jigc/config` delta is pending",
    );

    let out = corpus.jigc(&["task", "finalize", &report]);
    assert!(out.status.success(), "the report lands; {}", text(&out));
    assert_eq!(
        head_files(&corpus),
        vec![doc],
        "the report's commit carries no config delta",
    );
    assert_eq!(pending(), before, "the config delta is still pending");
}

/// **(3)** The triage shape: a task that **edits** a committed `idea` (copied in by an
/// ordinary `doc set-slot`) lands that doc alone, beside the code task's staged files.
#[test]
fn an_edited_committed_doc_lands_alone() {
    let corpus = corpus();
    let filing = corpus.start_workflow(REPORT_WORKFLOW_ID, "file the first finding");
    let doc = author_idea(&corpus, &filing, "Triage Me Later");
    let filed = corpus.jigc(&["task", "finalize", &filing]);
    assert!(filed.status.success(), "the filing lands; {}", text(&filed));

    let code = open_code_task(&corpus);
    let triage = corpus.start_workflow(REPORT_WORKFLOW_ID, "triage the finding");
    stage_code(&corpus);
    let slug = doc
        .strip_prefix("docs/ideas/")
        .and_then(|rest| rest.strip_suffix(".md"))
        .expect("the idea's promoted path");
    corpus.set_slot(
        &format!("idea:{slug}#description"),
        &triage,
        "Triaged: resolved by the doc-only finalize.",
    );
    fill_commit(&corpus, &triage);
    let before_index = cached(&corpus);

    let out = corpus.jigc(&["task", "finalize", &triage]);
    assert!(out.status.success(), "the triage lands; {}", text(&out));
    assert_eq!(
        head_files(&corpus),
        vec![doc.clone()],
        "the triage's commit holds the edited doc alone",
    );
    assert!(
        corpus
            .git(&["show", &format!("HEAD:{doc}")])
            .contains("Triaged: resolved"),
        "the edit is what landed",
    );
    assert_eq!(
        cached(&corpus),
        before_index,
        "the code task's index is unchanged"
    );
    code_task_lands_its_own_files(&corpus, &code, Staging::After);
}

/// **(4) The omitting context.** `park-idea` composes the ordinary `step:finalize`, so
/// over the same three states it behaves as it always has: state 1 lands the doc alone,
/// state 2 refuses `finalize.carried-staged` at exit 3, and state 3 sweeps the code
/// task's staged files into its commit.
#[test]
fn park_idea_over_the_same_states_behaves_as_today() {
    for staging in STATES {
        let corpus = corpus();
        let _code = open_code_task(&corpus);
        if staging == Staging::Before {
            stage_code(&corpus);
        }
        let park = corpus.start_workflow("park-idea", "park a thought");
        if staging == Staging::After {
            stage_code(&corpus);
        }
        let doc = author_idea(&corpus, &park, "A Parked Thought");
        let before_count = commit_count(&corpus);

        let out = corpus.jigc(&["task", "finalize", &park, "--format", "json"]);
        match staging {
            Staging::Unstaged => {
                assert!(out.status.success(), "[{staging:?}] lands; {}", text(&out));
                assert_eq!(head_files(&corpus), vec![doc]);
                let stderr = String::from_utf8_lossy(&out.stderr);
                assert!(
                    stderr.contains(INDEX_STEM) && !stderr.contains(DOC_ONLY_STEM),
                    "[{staging:?}] the ordinary model's advisory, on its own spelling; {}",
                    text(&out),
                );
            }
            Staging::Before => {
                assert_eq!(
                    out.status.code(),
                    Some(3),
                    "[{staging:?}] the carryover gate refuses; {}",
                    text(&out),
                );
                assert!(
                    String::from_utf8_lossy(&out.stdout).contains("finalize.carried-staged"),
                    "[{staging:?}] refused as finalize.carried-staged; {}",
                    text(&out),
                );
                assert_eq!(commit_count(&corpus), before_count, "nothing committed");
            }
            Staging::After => {
                assert!(out.status.success(), "[{staging:?}] lands; {}", text(&out));
                let mut files = head_files(&corpus);
                files.sort();
                let mut expected = vec![doc, NEW_FILE.to_owned(), TRACKED.to_owned()];
                expected.sort();
                assert_eq!(
                    files, expected,
                    "[{staging:?}] the ordinary model commits the index, code included",
                );
            }
        }
    }
}

/// **(5)** A `pre-commit` hook exiting 1 commits nothing, leaves the code task's index
/// unchanged, and frames the rejection as `finalize.commit-rejected`.
#[test]
fn a_rejecting_hook_commits_nothing_and_leaves_the_index_alone() {
    use std::os::unix::fs::PermissionsExt;

    let (corpus, _code, report, doc) = arrange(Staging::After);
    let hook = corpus.repo().join(".git/hooks/pre-commit");
    fs::write(
        &hook,
        "#!/bin/sh\necho 'policy: no commits today' >&2\nexit 1\n",
    )
    .expect("write the rejecting hook");
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod the hook");
    let before_index = cached(&corpus);
    let before_count = commit_count(&corpus);

    let out = corpus.jigc(&["task", "finalize", &report, "--format", "json"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a hook rejection exits 1; {}",
        text(&out)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("finalize.commit-rejected") && stderr.contains("policy: no commits today"),
        "framed as finalize.commit-rejected, the hook's bytes verbatim; {}",
        text(&out),
    );
    assert_eq!(commit_count(&corpus), before_count, "nothing committed");
    assert_eq!(
        cached(&corpus),
        before_index,
        "the code task's index is unchanged"
    );
    assert!(
        !corpus.repo().join(&doc).exists(),
        "the promotion is rolled back",
    );
}

/// **(6)** A finalize during an un-concluded merge refuses at the existing posture code,
/// committing nothing and leaving the merge in place.
#[test]
fn an_unconcluded_merge_refuses_at_the_posture_code() {
    let corpus = corpus();
    corpus.git(&["checkout", "-q", "-b", "side"]);
    fs::write(corpus.repo().join("side.txt"), "side\n").expect("write side");
    corpus.git(&["add", "side.txt"]);
    corpus.git(&["commit", "-q", "-m", "side"]);
    corpus.git(&["checkout", "-q", "main"]);

    let report = corpus.start_workflow(REPORT_WORKFLOW_ID, "file a finding");
    author_idea(&corpus, &report, "Mid Merge");
    corpus.git(&["merge", "-q", "--no-ff", "--no-commit", "side"]);
    let before_count = commit_count(&corpus);

    let out = corpus.jigc(&["task", "finalize", &report]);
    assert!(!out.status.success(), "refused; {}", text(&out));
    assert!(
        text(&out).contains("repo.operation-in-progress"),
        "refused at the existing posture code; {}",
        text(&out),
    );
    assert_eq!(commit_count(&corpus), before_count, "nothing committed");
    assert!(
        corpus.repo().join(".git/MERGE_HEAD").exists(),
        "the merge is left in place",
    );
}

/// **(7)** A milestone sub-task minted from the fixture workflow still refuses the
/// per-task finalize with `finalize.milestone-sub-task` at exit 3 — so `jigc milestone
/// finalize` never reaches the doc-only arm.
#[test]
fn a_milestone_sub_task_still_refuses_the_per_task_finalize() {
    let corpus = corpus();
    corpus.jigc_ok(&["milestone", "create", "file findings"]);
    let ack = corpus.jigc_ok(&[
        "milestone",
        "add-task",
        "file-findings",
        "file a finding",
        "--workflow",
        REPORT_WORKFLOW_ID,
    ]);
    let (_, rest) = ack
        .split_once("added task:")
        .unwrap_or_else(|| panic!("`milestone add-task` names the task it minted; got:\n{ack}"));
    let sub = rest.split_whitespace().next().expect("the sub-task id");

    let out = corpus.jigc(&["task", "finalize", sub]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "refused at exit 3; {}",
        text(&out)
    );
    assert!(
        text(&out).contains("finalize.milestone-sub-task"),
        "refused as finalize.milestone-sub-task; {}",
        text(&out),
    );
}

/// **(8)** `jigc task validate` previews the committing door's decision: in state 2 the
/// doc-only task reports no `finalize.carried-staged`, while the omitting `park-idea` task
/// over the same state still does.
#[test]
fn task_validate_in_state_two_previews_no_carryover() {
    let (corpus, _code, report, _doc) = arrange(Staging::Before);
    let out = corpus.jigc(&["task", "validate", &report, "--format", "json"]);
    assert!(
        out.status.success(),
        "the doc-only preview is clean at exit 0; {}",
        text(&out),
    );
    assert!(
        !text(&out).contains("finalize.carried-staged"),
        "no carryover finding on the doc-only arm; {}",
        text(&out),
    );

    let park = corpus.start_workflow("park-idea", "park a thought");
    author_idea(&corpus, &park, "A Parked Thought");
    let control = corpus.jigc(&["task", "validate", &park, "--format", "json"]);
    assert!(
        text(&control).contains("finalize.carried-staged"),
        "the control: an ordinary task over a pre-staged index still previews the gate; {}",
        text(&control),
    );
}

/// **(9) The index-gate member in this arm's spelling** (T3, `cli::gate_coverage`). The
/// composed `what's-left:` line of a doc-only task — read off the emitted bytes — carries the
/// table's sentence on [`CommitModel::DocOnly`] and never names the carryover gate, which is
/// skipped here; and in state 2 both previews, `jigc task validate` and `--dry-run`, decide the
/// member as the committing door does: no `finalize.carried-staged`, exit 0.
///
/// **The omitting context:** a `park-idea` task over the same pre-staged index composes the
/// ordinary sentence, and its `--dry-run` still refuses at the carryover gate.
#[test]
fn the_composed_line_and_both_previews_state_this_arms_index_gate() {
    use cli::gate_coverage::{GATE_COVERAGE, whats_left_coverage};
    use cli::render::CommitModel;

    let index_gate = GATE_COVERAGE
        .iter()
        .find(|row| row.id == "carryover")
        .expect("the index-gate member");
    let whats_left = |composed: &str| -> String {
        composed
            .lines()
            .find(|line| line.starts_with("what's-left: "))
            .unwrap_or_else(|| panic!("a composed what's-left line; got:\n{composed}"))
            .to_owned()
    };
    let minted = |composed: &str| -> String {
        composed
            .lines()
            .find_map(|line| line.strip_prefix("task minted: "))
            .unwrap_or_else(|| panic!("a minted task id; got:\n{composed}"))
            .trim()
            .to_owned()
    };

    let corpus = corpus();
    let _code = open_code_task(&corpus);
    stage_code(&corpus);

    let composed = corpus.jigc_ok(&["start", "--workflow", REPORT_WORKFLOW_ID, "file a finding"]);
    let line = whats_left(&composed);
    assert!(
        line.contains(&whats_left_coverage(CommitModel::DocOnly))
            && line.contains(index_gate.token(CommitModel::DocOnly)),
        "the doc-only task's composed line states the table's sentence on its own model:\n  \
         line: {line}",
    );
    assert!(
        !line.contains("carryover"),
        "…and never names the carryover gate this arm skips:\n  line: {line}",
    );
    let report = minted(&composed);
    author_idea(&corpus, &report, "Previews Agree");

    for (door, args) in [
        (
            "task validate",
            vec!["task", "validate", report.as_str(), "--format", "json"],
        ),
        (
            "--dry-run",
            vec![
                "task",
                "finalize",
                report.as_str(),
                "--dry-run",
                "--format",
                "json",
            ],
        ),
    ] {
        let out = corpus.jigc(&args);
        assert!(
            out.status.success() && !text(&out).contains("finalize.carried-staged"),
            "[{door}] state 2 previews no carryover finding on the doc-only arm, exit 0; {}",
            text(&out),
        );
    }

    let park = corpus.jigc_ok(&["start", "--workflow", "park-idea", "park a thought"]);
    assert!(
        whats_left(&park).contains(&whats_left_coverage(CommitModel::Index)),
        "the control: an ordinary task composes the ordinary sentence:\n{}",
        whats_left(&park),
    );
    let park_task = minted(&park);
    author_idea(&corpus, &park_task, "A Parked Thought");
    let control = corpus.jigc(&[
        "task",
        "finalize",
        &park_task,
        "--dry-run",
        "--format",
        "json",
    ]);
    assert!(
        control.status.code() == Some(3) && text(&control).contains("finalize.carried-staged"),
        "the control: an ordinary task's --dry-run still refuses at the carryover gate; {}",
        text(&control),
    );
}
