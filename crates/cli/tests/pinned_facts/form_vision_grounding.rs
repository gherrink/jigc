//! **Pinned fact** (findings-verification.md §2, row 3; §1.5 path A):
//! *"form-vision grounding render regressed" — **REFUTED** as a regression.*
//!
//! The trial reported that form-vision's grounding render broke. The refutation: on
//! the **fresh-create path** — create the vision, set `grounded-in`, re-compose — the
//! grounded research's `findings` render in full (the M39 multi-source fix is intact;
//! rc.2's F1 fix stands). The *real* defect (§1.5) is a **role-binding trap on a
//! different, unpinned path** — a bare `set-field` on the *committed* vision that
//! skips `doc create`, so `task.vision` never binds — and that is closed and pinned
//! by the wave's role-binding fix (`flow_role_binding.rs`), not a regression.
//!
//! The repro block, made standing:
//!
//! ```yaml
//! claim: "form-vision fresh-create grounding render regressed"
//! verdict: REFUTED
//! setup: [ commit a research doc; start form-vision; doc create vision ]
//! repro: [ set-field vision#meta/grounded-in [research]; jigc start --task <t> ]
//! expect: the composed step renders the grounded research's findings
//! ```
//!
//! **Fact drift, not contract drift.** `flow_form_vision` is the M39 *acceptance*
//! flow; `flow_role_binding` pins the *fix* for the real (set-field-first) defect.
//! This pins the *refutation itself* — that the fresh-create path was never broken —
//! with a before/after contrast so the render is proven read, not assumed
//! (pinning.md §5).

use crate::support::trial_corpus::{State, TrialCorpus};

/// The distinctive grounding prose the re-composed form-vision step must echo — a
/// string that appears nowhere else in the composed workflow, so its presence is the
/// edge-walk slice actually reading the committed research.
const GROUNDING_FINDINGS: &str = "Static rules files go stale and are read once, not just in time.";

/// Commit one `research` doc through its real `do-research` workflow, returning the
/// minted `research:<slug>` id read off the `doc create` output (never reconstructed
/// test-side).
fn commit_research(corpus: &TrialCorpus) -> String {
    let task = corpus.start_workflow("do-research", "how agents lose context");
    let id = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "research",
            "--title",
            "Context Loss",
            "--task",
            &task,
        ])
        .trim_end_matches('\n')
        .to_string();
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{id}#question"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "How does a coding agent lose the context it was given?",
    );
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{id}#findings"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        GROUNDING_FINDINGS,
    );
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{id}#sources"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "The adoption trial records, 2026.",
    );
    corpus.finalize(&task, "research", "record the context-loss research", false);
    id
}

/// **The pin.** Fresh-create form-vision: create the vision, set `grounded-in` to the
/// committed research, re-compose — the grounded findings render. The **before**
/// compose (grounding unset) is empty of the findings; the **after** compose carries
/// them. The contrast is what makes "renders" a fact, not a bare happy path — and it
/// is exactly the render the trial claimed regressed.
#[test]
fn fresh_create_grounding_renders_the_research_findings() {
    let corpus = TrialCorpus::build(State::Fresh);
    let research = commit_research(&corpus);

    let task = corpus.start_workflow("form-vision", "form the grounded vision");
    corpus.jigc_ok(&[
        "doc", "create", "vision", "--title", "Vision", "--task", &task,
    ]);

    // Before grounding: the edge-walk slice resolves empty — the findings are ABSENT.
    let before = corpus.jigc_ok(&["start", "--task", &task]);
    assert!(
        !before.contains(GROUNDING_FINDINGS),
        "before `grounded-in` is set, the grounding findings must NOT render — else \
         the after-contrast proves nothing; got:\n{before}",
    );

    // Ground the fresh-created vision in the committed research (the fresh-create
    // path binds `task.vision` via the create-gate, so the slice resolves).
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        "vision:vision#meta/grounded-in",
        "--value",
        &format!("[{research}]"),
        "--task",
        &task,
    ]);

    // After grounding + re-compose: the grounded research's findings render in full.
    let after = corpus.jigc_ok(&["start", "--task", &task]);
    assert!(
        after.contains(GROUNDING_FINDINGS),
        "REFUTED (§2 row 3): on the fresh-create path the grounded research's findings \
         MUST render — the 'regression' claim is false (M39 multi-source fix intact); \
         got:\n{after}",
    );
}
