//! M55 Increment 2 — **the create-only gate** (`design/findings-channel.md` §4, §10;
//! `design/workflow-dialect.md` → On-disk definition format).
//!
//! **T1 — strict `allows-create` entry keys (O3).** An entry key outside the closed set is
//! refused when the workflow loads, naming the key, under the front-matter envelope every
//! load door already refuses with (`workflow-refs.malformed-front-matter`, M55 pin P1).
//! Driven through the real binary on a committed project-layer shadow of `park-idea`
//! whose entry carries the misspelt `nwe: true`:
//!
//! - `jigc start --workflow park-idea` exits 1 naming `nwe` (it minted a task before);
//! - `jigc validate` lists the finding at `workflow:park-idea` naming `nwe`, and keeps its
//!   store-scope exit 0 (M55 pin P2; it reported no findings before);
//! - a task minted from the clean shadow, whose entry is misspelt afterwards, has
//!   `jigc doc create idea` refused at exit 1 naming `nwe` — the `doc` doors load the
//!   workflow too.
//!
//! **The omitting context** is the clean shadow, identical but for the typo: it loads and
//! mints, so the refusal is the key's and nothing else's.

use crate::support;

use std::fs;
use std::process::Output;

use support::run_then_parse::stdout_json;
use support::trial_corpus::{State, TrialCorpus};

/// The shipped `park-idea` definition — the shadow is these bytes with one entry changed,
/// so the only difference the binary can see is the one under test.
const SHIPPED_PARK_IDEA: &str = include_str!("../packs/methodology/workflows/park-idea.yaml");

/// The shipped entry, and the misspelt one the strict key set refuses.
const CLEAN_ENTRY: &str = "{ type: idea, as: idea }";
const MISSPELT_ENTRY: &str = "{ type: idea, as: idea, nwe: true }";

const SHADOW: &str = ".jigc/config/workflows/park-idea.yaml";
const MALFORMED: &str = "workflow-refs.malformed-front-matter";

/// `park-idea` with its entry replaced by `entry`.
fn park_idea_with(entry: &str) -> String {
    assert!(
        SHIPPED_PARK_IDEA.contains(CLEAN_ENTRY),
        "the premise: the shipped park-idea grants `{CLEAN_ENTRY}`; got:\n{SHIPPED_PARK_IDEA}",
    );
    SHIPPED_PARK_IDEA.replace(CLEAN_ENTRY, entry)
}

/// Write the project-layer shadow of `park-idea` carrying `entry`, and commit it.
fn commit_shadow(corpus: &TrialCorpus, entry: &str) {
    let path = corpus.repo().join(SHADOW);
    fs::create_dir_all(path.parent().expect("a parent")).expect("mk the workflows dir");
    fs::write(&path, park_idea_with(entry)).expect("write the shadow");
    corpus.git(&["add", "--", SHADOW]);
    corpus.git(&["commit", "-q", "-m", "chore: shadow park-idea"]);
}

fn text(out: &Output) -> String {
    format!(
        "--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Assert `out` is a load refusal: exit 1, the malformed-front-matter code, naming `nwe`.
fn assert_refused_naming_the_key(out: &Output, what: &str) {
    let shown = text(out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "{what} is refused at exit 1; {shown}"
    );
    assert!(
        shown.contains(MALFORMED),
        "{what} is refused with `{MALFORMED}`; {shown}"
    );
    assert!(
        shown.contains("nwe"),
        "{what}'s refusal names the unknown key `nwe`; {shown}"
    );
}

/// **(1)** `jigc start --workflow park-idea` over the misspelt shadow exits 1 naming `nwe`,
/// and mints nothing.
#[test]
fn start_refuses_a_misspelt_entry_key_naming_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    commit_shadow(&corpus, MISSPELT_ENTRY);

    let out = corpus.jigc(&["start", "--workflow", "park-idea", "park one idea"]);
    assert_refused_naming_the_key(&out, "`jigc start --workflow park-idea`");
    assert!(
        !String::from_utf8_lossy(&out.stdout).contains("task minted:"),
        "nothing is minted; {}",
        text(&out),
    );
}

/// **(2)** `jigc validate` lists the finding at `workflow:park-idea` naming `nwe`, and keeps
/// its report-only store-scope exit 0 (P2).
#[test]
fn validate_reports_a_misspelt_entry_key_and_exits_zero() {
    let corpus = TrialCorpus::build(State::Fresh);
    commit_shadow(&corpus, MISSPELT_ENTRY);

    let out = corpus.jigc(&["validate", "--format", "json"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "store-scope validate stays report-only, exit 0; {}",
        text(&out),
    );
    let report: serde_json::Value = stdout_json(&out, &[0], "`jigc validate --format json`");
    let findings = report["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the report carries a `findings` array; {}", text(&out)));
    let hits: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| f["code"] == MALFORMED && f["key"]["target"] == "workflow:park-idea")
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "one `{MALFORMED}` finding at `workflow:park-idea`; findings:\n{findings:#?}",
    );
    let message = hits[0]["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("nwe"),
        "the finding names the unknown key `nwe`; got: {message}",
    );
}

/// **(3)** A task minted from the clean shadow — the omitting context, which loads and
/// mints — has `jigc doc create idea` refused at exit 1 naming `nwe` once the entry is
/// misspelt: the `doc` doors load the workflow too.
#[test]
fn doc_create_refuses_a_misspelt_entry_key_naming_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    commit_shadow(&corpus, CLEAN_ENTRY);
    let task = corpus.start_workflow("park-idea", "park one idea");

    commit_shadow(&corpus, MISSPELT_ENTRY);
    let out = corpus.jigc(&[
        "doc",
        "create",
        "idea",
        "--title",
        "A Misspelt Gate",
        "--task",
        &task,
    ]);
    assert_refused_naming_the_key(&out, "`jigc doc create idea`");
}
