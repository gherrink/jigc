//! **Pinned fact — the poster child** (findings-verification.md §2, row 1):
//! *"Workflow-printed `#type`/`#scope` commit addresses error" — **REFUTED**.*
//!
//! The project-alpha-3.0 trial reported that the bare single-hop commit addresses the
//! composed workflows print (`commit:<t>#type`, `commit:<t>#scope`) error. The
//! refutation: both the bare single-hop **and** the section-qualified forms resolve
//! (a deliberate write-path alias) — the trial's own log held ~40 successful
//! bare-form calls and **zero** failures; "errors" was an inference from surfaces
//! disagreeing, not an observation.
//!
//! This is the repro block in `pinning.md` §3, made a standing test verbatim:
//!
//! ```yaml
//! claim: "set-field commit:<t>#type errors (bare single-hop form)"
//! verdict: REFUTED
//! repro:
//!   - ["jigc","doc","set-field","commit:<task>#type","--value","docs","--task","<task>"]
//! expect: { exit: 0, stdout_json: { "findings": [] } }
//! ```
//!
//! **Fact drift, not contract drift.** `doc_read_surface::documented_alias_forms_round_trip`
//! pins the *contract* — the address grammar as a registry-derived round-trip
//! property. This pins the *trial claim* — the exact bare forms the trial ran, over
//! the trial-shaped fixture — so the specific refutation cannot drift out from under
//! the property (pinning.md §5).

use crate::support::trial_corpus::{State, TrialCorpus};
use serde_json::Value;

/// The `set-field --format json` ack for `commit:<task>#<leaf>`: exit 0, `findings`
/// empty, and the bare single-hop leaf decomposed to `header/<leaf>` — which is the
/// alias resolving. The decomposition is what proves "resolves" rather than merely
/// "did not error".
fn set_bare_commit_field(corpus: &TrialCorpus, task: &str, leaf: &str, value: &str) -> Value {
    let out = corpus.jigc(&[
        "doc",
        "set-field",
        &format!("commit:{task}#{leaf}"),
        "--value",
        value,
        "--format",
        "json",
        "--task",
        task,
    ]);
    assert!(
        out.status.success(),
        "REFUTED (§2 row 1): the bare single-hop `commit:{task}#{leaf}` write must \
         exit 0 — the trial's `errors` claim is false; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("the ack must be one JSON document ({e}); got:\n{stdout}"))
}

/// **The pin.** Both bare single-hop commit fields the composed workflows print —
/// `#type` and `#scope` — resolve on the write path: exit 0, `findings: []`, and the
/// ack shows each bare leaf resolved through the section-qualified `header/<leaf>`
/// node (the deliberate alias). The negative control at the end is what keeps
/// "resolves" a discriminating fact rather than "everything passes".
#[test]
fn bare_single_hop_commit_type_and_scope_resolve() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "probe the bare commit addresses");

    for (leaf, value) in [("type", "docs"), ("scope", "pinning")] {
        let ack = set_bare_commit_field(&corpus, &task, leaf, value);
        assert_eq!(
            ack["findings"],
            serde_json::json!([]),
            "the refuted repro's `stdout_json: {{ findings: [] }}` — a clean bare write",
        );
        // The bare single-hop leaf resolved to the section-qualified `header/<leaf>`
        // node: the alias, not a coincidence of a same-named top-level field.
        assert_eq!(
            ack["target"]["section"], "header",
            "the bare `#{leaf}` resolves through the `header` section (the alias); got:\n{ack:#}",
        );
        assert_eq!(
            ack["target"]["leaf"], leaf,
            "the bare `#{leaf}` addresses the `{leaf}` leaf; got:\n{ack:#}",
        );
        assert_eq!(ack["value"], value, "the write took effect; got:\n{ack:#}");
    }

    // Negative control: a genuinely non-existent leaf IS refused, so the two passes
    // above are the alias resolving, not a write path that accepts any string.
    let bogus = corpus.jigc(&[
        "doc",
        "set-field",
        &format!("commit:{task}#nonesuch"),
        "--value",
        "x",
        "--task",
        &task,
    ]);
    assert!(
        !bogus.status.success(),
        "a non-existent commit leaf must be refused — otherwise `#type`/`#scope` \
         resolving proves nothing; the write accepted `#nonesuch`",
    );
}
