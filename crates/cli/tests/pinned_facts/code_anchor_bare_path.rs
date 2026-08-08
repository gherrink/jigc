//! **Pinned fact** (RC-alpha4 [findings-verification.md](../../../../completions/artifacts/RC-alpha4/findings-verification.md)
//! → **D10**, the refuted half): *"`implemented-by` rejects a bare path — the two
//! code-anchor fields accept two different shapes"* — **REFUTED at every gate.**
//!
//! P1 remembered a rejection of `implemented-by: apps/api/index.js` (no `#symbol`).
//! It did not reproduce: on rc.9 there is **one accepted grammar everywhere** —
//! a bare path is accepted at the write, at `task validate`, at `finalize` and at the
//! store-scope sweep, because the engine treats an anchor with no `#symbol` as
//! *"itself the file"* and `symbol-exists` degrades to **file existence**.
//!
//! **Only the refuted half is pinned here.** D10's CONFIRMED half — the two
//! `code-anchor` fields projecting indistinguishably in `doc schema`, with the
//! *check-activation* difference stated nowhere — is a separate finding, and pinning
//! the acceptance must not be read as blessing that gap.
//!
//! The repro block, made standing:
//!
//! ```yaml
//! claim: "implemented-by rejects a bare path (no #symbol)"
//! verdict: REFUTED
//! setup: [ { fixture: vendored } ]        # a committed arch-doc + real tracked source
//! repro:
//!   - ["jigc","doc","set-field","arch-doc:<slug>#components/<id>/implemented-by",
//!      "--value","src/pad.ts","--task","<task>"]
//!   - ["jigc","task","validate","<task>"]
//!   - ["jigc","task","finalize","<task>"]
//!   - ["jigc","validate"]
//! expect:
//!   exit: 0 at every gate — accepted at write, validate, finalize and the store sweep
//! negative-control:
//!   - bare path naming NO file → doc-code.symbol-exists BLOCKS, so the acceptance
//!     above is a resolving existence check, not an unchecked string field
//! ```
//!
//! **Fact drift, not contract drift.** The `doc-code` probe suites pin the *check*;
//! this pins the *trial claim* — the exact shape P1 believed refused, driven end to
//! end over the fixture state that carries a real tracked symbol (pinning.md §3).

use crate::support::trial_corpus::{State, TrialCorpus, VENDORED_CODE_FILE, VENDORED_CODE_SYMBOL};
use serde_json::Value;

/// The committed `arch-doc`'s `<id>#components/<component>/implemented-by` address,
/// read out of the read surface rather than reconstructed from the slug rule.
fn implemented_by_address(corpus: &TrialCorpus) -> String {
    let listed = corpus.jigc_ok(&["doc", "list", "arch-doc", "--format", "json"]);
    let listed: Value = serde_json::from_str(listed.trim())
        .unwrap_or_else(|e| panic!("`doc list --format json` parses ({e}); got:\n{listed}"));
    let doc_id = listed["docs"][0]["id"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("the vendored state ships one committed arch-doc; got:\n{listed:#}")
        })
        .to_string();

    let doc = corpus.jigc_ok(&["doc", "show", &doc_id, "--format", "json"]);
    let doc: Value = serde_json::from_str(doc.trim())
        .unwrap_or_else(|e| panic!("`doc show --format json` parses ({e}); got:\n{doc}"));
    let component = doc["sections"]["components"][0]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("the arch-doc carries one component; got:\n{doc:#}"));

    // The committed value is the SYMBOL-BEARING form, so the arm below is a real
    // change of shape rather than a re-write of the same bytes.
    let committed = doc["sections"]["components"][0]["implemented-by"]
        .as_str()
        .unwrap_or_else(|| panic!("the component carries an anchor; got:\n{doc:#}"));
    assert_eq!(
        committed,
        format!("{VENDORED_CODE_FILE}#{VENDORED_CODE_SYMBOL}"),
        "the fixture commits the `path#symbol` form, so switching to a bare path is \
         the shape change D10 claims is refused",
    );

    format!("{doc_id}#components/{component}/implemented-by")
}

/// Fill the task's transient `commit` doc, so a `task validate` in this task reports
/// on the **anchor** rather than on the unauthored commit message — the gate under
/// test has to be the only one talking.
fn author_commit(corpus: &TrialCorpus, task: &str, summary: &str) {
    for (leaf, value) in [("type", "docs"), ("scope", "arch-doc")] {
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
        summary,
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
        "Built by the pinned-facts code-anchor test.",
    );
}

/// **The pin.** A bare path — no `#symbol` — is accepted as an `implemented-by`
/// code anchor at **every** gate: the write, `task validate`, `finalize`, and the
/// store-scope sweep. Exactly the shape P1 reported as refused.
#[test]
fn a_bare_path_is_accepted_as_a_code_anchor_at_every_gate() {
    let corpus = TrialCorpus::build(State::Vendored);
    let address = implemented_by_address(&corpus);

    let task = corpus.start_workflow("single-task", "re-anchor the padding component");
    let ack = corpus.jigc(&[
        "doc",
        "set-field",
        &address,
        "--value",
        VENDORED_CODE_FILE,
        "--task",
        &task,
    ]);
    assert!(
        ack.status.success(),
        "REFUTED (D10): a bare path must be ACCEPTED at the write — P1's remembered \
         rejection does not reproduce; stderr:\n{}",
        String::from_utf8_lossy(&ack.stderr),
    );

    author_commit(&corpus, &task, "re-anchor the padding component");
    let validated = corpus.jigc(&["task", "validate", &task]);
    assert!(
        validated.status.success(),
        "a bare path passes `task validate` — `symbol-exists` degrades to file \
         existence, and the file exists; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&validated.stdout),
        String::from_utf8_lossy(&validated.stderr),
    );

    // Through the commit boundary, where `doc-code.symbol-exists` is blocking.
    corpus.jigc_ok(&["task", "finalize", &task]);
    let committed = corpus.jigc_ok(&["doc", "show", &address]);
    assert_eq!(
        committed.trim(),
        VENDORED_CODE_FILE,
        "the bare path is what landed — the anchor was not silently normalized",
    );

    let store = corpus.jigc(&["validate"]);
    assert!(
        store.status.success(),
        "the store-scope sweep accepts the bare path too; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&store.stdout),
        String::from_utf8_lossy(&store.stderr),
    );
}

/// **The negative control that makes the pin a fact.** A bare path naming **no
/// file** blocks — so the acceptance above is a check that resolved, not a field
/// nobody looks at. (This is also the case P1's block plausibly was: a dangling
/// path, a different finding.)
#[test]
fn a_bare_path_naming_no_file_still_blocks() {
    let corpus = TrialCorpus::build(State::Vendored);
    let address = implemented_by_address(&corpus);

    let task = corpus.start_workflow("single-task", "dangle the padding anchor");
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &address,
        "--value",
        "src/nowhere.ts",
        "--task",
        &task,
    ]);
    author_commit(&corpus, &task, "dangle the padding anchor");

    let validated = corpus.jigc(&["task", "validate", &task]);
    assert!(
        !validated.status.success(),
        "a bare path that names no file must BLOCK — otherwise the accepted bare \
         path above proves nothing; stdout:\n{}",
        String::from_utf8_lossy(&validated.stdout),
    );
    let surface = format!(
        "{}{}",
        String::from_utf8_lossy(&validated.stdout),
        String::from_utf8_lossy(&validated.stderr),
    );
    assert!(
        surface.contains("doc-code.symbol-exists"),
        "the block is the anchor check itself, not an unrelated gate; got:\n{surface}",
    );
}
