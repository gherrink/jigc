//! **F-11 — the `doc author` payload parse joins the write-miss floor** (M51 Increment 6,
//! T3; [roadmap](../../../implementation/roadmap.md) → Increment 6; the rc.14 trial's
//! [findings-verification](../../../completions/artifacts/RC-rc14/findings-verification.md)
//! row F-11).
//!
//! **What was driven, and why it is a defect.** `jigc doc author <doctype> --from-file`
//! refused a malformed payload with a bare sentence — *"malformed `doc author` payload:
//! unknown field `sectons` …"* — carrying **no severity, no code, no `at:`, no route and no
//! footer**, flattened to `{"error": …}` on `--format json`, while **the same door** answers
//! a declared-address miss with `blocking · write.unknown-field` + `at:` + a route. M50
//! Increment 9 closed the write-miss floor at the four target resolvers, and the payload
//! parse sits **upstream of every one of them**: a driver keying on the stable
//! `(code, target)` pair got nothing at the door it hits first.
//!
//! **The set this suite iterates, and what kind of set it is.** Not the reported repro: the
//! **class's defining case-set**, [`PayloadReject::ALL`] — every way
//! `cli::author::parse_author_payload` can refuse, read off a code-side enum that the
//! producer itself constructs through. [`every_refusal_exit_of_the_payload_parse_is_driven`]
//! closes it in both directions: every variant is driven by a cell here, and the production
//! source mints its refusals through the **one** constructor, so a fifth refusal exit cannot
//! be added without either taking a variant (which then owes a driven cell) or minting a
//! `Finding` outside that constructor (which reddens the same arm).
//!
//! **Declared bound.** The source-side half is a **text scan** of one production module, not
//! a type-level fence: it establishes that this module has one finding-minting seam and that
//! each variant is constructed exactly once. It cannot see a refusal that converts into
//! `Finding` through some future `From` impl — there is none today, and the error type is
//! what makes that visible (a bare `anyhow` cannot inhabit `Result<_, Finding>`).

use cli::author::PayloadReject;
use std::path::Path;

use crate::support;
use support::trial_corpus::{State, TrialCorpus};

/// The doctype every cell authors: a **non-singleton** (so the missing-`title:` cell is a
/// refusal rather than a defaulted id-source) whose schema declares both a prose slot
/// (`context`) and an inline field (`status`) — the two leaf kinds the `<<…>>` cross-check
/// discriminates.
const DOCTYPE: &str = "adr";

/// The workflow whose `allows-create:` grants `adr`, so a cell that is refused is refused by
/// the **payload parse** and not by a create-gate standing in front of it.
const WORKFLOW: &str = "record-decision";

/// One refusal site of `parse_author_payload`, as a payload that reaches it.
struct Cell {
    /// The refusal, as the done-criterion names it.
    label: &'static str,
    /// The code-side variant this payload must land on.
    reject: PayloadReject,
    /// The payload bytes, fed through `--from-file -`.
    payload: &'static str,
}

/// The five payloads — four refusal exits, the ungrammatical one reached by both of the
/// shapes a driver actually produces (a typo'd key and broken YAML).
const CELLS: &[Cell] = &[
    Cell {
        label: "unknown top-level field",
        reject: PayloadReject::Ungrammatical,
        payload: "title: A choice\nsectons: []\n",
    },
    Cell {
        label: "unparseable YAML",
        reject: PayloadReject::Ungrammatical,
        payload: "title: A choice\n  : :\n",
    },
    Cell {
        label: "missing `title:` on a non-singleton",
        reject: PayloadReject::MissingTitle,
        payload: "sections: []\n",
    },
    Cell {
        label: "a bare slot value",
        reject: PayloadReject::BareSlotValue,
        payload: "title: A choice\nsections:\n  - id: context\n    set:\n      context: plain prose\n",
    },
    Cell {
        label: "a wrapped field value",
        reject: PayloadReject::WrappedFieldValue,
        payload: "title: A choice\nsections:\n  - id: status\n    set:\n      status: \"<<accepted>>\"\n",
    },
];

/// A corpus with one live task minted from [`WORKFLOW`]. Nothing a cell drives persists —
/// the parse runs before any create — so one corpus serves every cell.
fn corpus() -> (TrialCorpus, String) {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow(WORKFLOW, "record the batch-payload decision");
    (corpus, task)
}

/// Drive one cell and return `(exit code, stderr)`.
fn drive(corpus: &TrialCorpus, task: &str, payload: &str, json: bool) -> (Option<i32>, String) {
    let mut args = vec!["doc", "author", DOCTYPE, "--from-file", "-", "--task", task];
    if json {
        args.extend(["--format", "json"]);
    }
    let out = corpus.jigc_stdin(&args, payload);
    (
        out.status.code(),
        String::from_utf8(out.stderr).expect("utf-8 stderr"),
    )
}

/// **The agent-text surface.** Every refusal site answers with the located, coded, routed
/// shape the same door already gives a declared-address miss: `blocking · <code> — `, an
/// `at:` naming the doctype, and a route.
#[test]
fn every_payload_refusal_answers_with_a_severity_a_code_an_at_and_a_route() {
    let (corpus, task) = corpus();
    for cell in CELLS {
        let (code, stderr) = drive(&corpus, &task, cell.payload, false);
        assert_eq!(code, Some(1), "{}: exit code\n{stderr}", cell.label);
        let lead = format!("blocking · {} — ", cell.reject.code());
        assert!(
            stderr.contains(&lead),
            "{}: stderr carries no `{lead}`:\n{stderr}",
            cell.label,
        );
        assert!(
            stderr.contains(&format!("\n  at: {DOCTYPE}\n")),
            "{}: stderr names no `at: {DOCTYPE}`:\n{stderr}",
            cell.label,
        );
        assert!(
            stderr.contains("\n  route: "),
            "{}: stderr carries no route:\n{stderr}",
            cell.label,
        );
    }
}

/// **The machine surface.** The same refusals reach `--format json` as the declared
/// `Reject::Findings` envelope — `{findings, schema_version}` — with the stable
/// `(code, target)` key resolving to `{<code>, <doctype>}`, not as the flattened
/// `{"error": …}` a driver cannot key on.
#[test]
fn every_payload_refusal_emits_the_findings_envelope_with_a_resolving_key() {
    let (corpus, task) = corpus();
    for cell in CELLS {
        let (exit, stderr) = drive(&corpus, &task, cell.payload, true);
        assert_eq!(exit, Some(1), "{}: exit code\n{stderr}", cell.label);
        let doc: serde_json::Value = serde_json::from_str(&stderr)
            .unwrap_or_else(|e| panic!("{}: {e}\n{stderr}", cell.label));
        let object = doc.as_object().expect("a JSON object envelope");
        let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            ["findings", "schema_version"],
            "{}: the declared `Reject::Findings` key set\n{stderr}",
            cell.label,
        );
        let findings = object["findings"].as_array().expect("a findings array");
        assert_eq!(findings.len(), 1, "{}: one finding\n{stderr}", cell.label);
        assert_eq!(
            findings[0]["key"],
            serde_json::json!({ "code": cell.reject.code(), "target": DOCTYPE }),
            "{}: the stable finding key\n{stderr}",
            cell.label,
        );
        assert_eq!(
            findings[0]["severity"], "blocking",
            "{}: severity\n{stderr}",
            cell.label,
        );
        assert!(
            findings[0]["route"].is_string(),
            "{}: route\n{stderr}",
            cell.label,
        );
    }
}

/// **The completeness arm, in both directions.** `PayloadReject::ALL` is the set; every
/// member is driven by a cell above, and the production module constructs every refusal
/// through the one shared constructor, exactly once per member — so a fifth refusal exit
/// cannot land silently.
#[test]
fn every_refusal_exit_of_the_payload_parse_is_driven() {
    let driven: Vec<PayloadReject> = CELLS.iter().map(|cell| cell.reject).collect();
    for reject in PayloadReject::ALL {
        assert!(
            driven.contains(reject),
            "declared refusal exit `{}` is driven by no cell",
            reject.variant(),
        );
    }
    for reject in &driven {
        assert!(
            PayloadReject::ALL.contains(reject),
            "cell drives `{}`, which the declared set does not carry",
            reject.variant(),
        );
    }

    let source =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/author.rs"))
            .expect("read the payload parser's source");
    let code = production_code(&source);
    assert_eq!(
        code.matches("Finding::graded(").count(),
        1,
        "the payload parser mints its refusals through exactly one constructor — a second \
         mint is a refusal exit outside `PayloadReject`",
    );
    for reject in PayloadReject::ALL {
        let construction = format!("reject(PayloadReject::{},", reject.variant());
        assert_eq!(
            code.matches(&construction).count(),
            1,
            "`{}` is constructed exactly once in the payload parser",
            reject.variant(),
        );
    }
    assert_eq!(
        code.matches("reject(PayloadReject::").count(),
        PayloadReject::ALL.len(),
        "every `reject(…)` call site names a declared variant",
    );
}

/// `source` with its `#[cfg(test)]` module, its comment lines and **all** whitespace
/// removed — so the scan above reads the module's *code* and is blind to how rustfmt broke
/// a call across lines (the first spelling of it was, and passed a construction the
/// formatter had wrapped).
fn production_code(source: &str) -> String {
    source
        .split_once("\n#[cfg(test)]")
        .map_or(source, |(head, _)| head)
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .flat_map(str::chars)
        .filter(|c| !c.is_whitespace())
        .collect()
}
