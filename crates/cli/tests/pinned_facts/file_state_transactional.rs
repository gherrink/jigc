//! **Pinned fact** (findings-verification.md §2, row 4 / §1.3 attribution correction):
//! *Bug C's attribution corrected — the transactional invariant holds in the letter.*
//!
//! The trial blamed a discarded/abandoned task for a dangling file-state entry that
//! blocked every later task. The correction: **false** — a plain task writes **no**
//! file-state entries before finalize (the baseline is recorded at finalize by
//! `advance_file_state`), so `run_discard` has nothing to roll back and no writer is
//! un-paired from a commit. "Writes are transactional" holds in the letter. The real
//! §1.3 defect was a **git-level history discard** of a commit whose baseline had been
//! recorded — a decoupled cache lifetime, closed and pinned by
//! `file_state_history_gate.rs`, not by anything on the task-discard path.
//!
//! The repro block, made standing:
//!
//! ```yaml
//! claim: "task discard orphans file-state and blocks future tasks"
//! verdict: REFUTED (attribution corrected)
//! setup: [ start a plain task; author a managed doc ]
//! repro: [ jigc task discard <t> ; then a fresh full task+finalize ]
//! expect:
//!   discard: exit 0, no file-state baseline for the discarded docs
//!   fresh task: finalizes clean (never a reconciliation block, exit 3)
//! ```
//!
//! **Fact drift, not contract drift.** `file_state_history_gate.rs` pins the *fix* for
//! the actual cache-lifetime defect. This pins the *corrected attribution* — that the
//! discard path never orphaned state to begin with — so the trial's mistaken blame
//! cannot silently re-enter the record (pinning.md §5).

use crate::support::trial_corpus::{State, TrialCorpus};

/// The gitignored file-state cache — the store's baseline records, written at finalize.
const FILE_STATE_JSON: &str = ".jigc/state/file-state.json";

/// **The pin.** A plain task, authored and then **discarded**, leaves no file-state
/// baseline behind (there was none to leave — the baseline is a finalize-time
/// record), and a subsequent fresh full task finalizes **clean**: never blocked by a
/// reconciliation entry the discard could have orphaned. The clean landing is the
/// transactional-invariant-holds proof; the empty-cache assertion is why it lands.
#[test]
fn a_discarded_plain_task_orphans_no_file_state_and_never_blocks_the_next() {
    let corpus = TrialCorpus::build(State::Fresh);

    // A plain task that authors a managed doc, then is discarded before finalize.
    let doomed = corpus.start_workflow("planning", "plan a wave, then abandon it");
    corpus.jigc_ok(&[
        "doc", "create", "roadmap", "--title", "Roadmap", "--task", &doomed,
    ]);
    let discard = corpus.jigc(&["task", "discard", &doomed, "--force"]);
    assert!(
        discard.status.success(),
        "`task discard` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&discard.stderr),
    );

    // No file-state baseline was recorded for the discarded doc — a plain task writes
    // none before finalize, so there is nothing for the discard to orphan.
    let cache = corpus.repo().join(FILE_STATE_JSON);
    if cache.exists() {
        let body = std::fs::read_to_string(&cache).expect("read the file-state cache");
        assert!(
            !body.contains("roadmap"),
            "REFUTED (§2 row 4): a discarded plain task must orphan NO file-state \
             baseline — the discard path never wrote one; got:\n{body}",
        );
    }

    // A fresh full task lands clean — the discarded task left nothing that blocks it.
    // A dangling reconciliation entry would exit 3 here; `finalize` asserts success,
    // so a clean return IS the transactional-invariant-holds pin.
    let fresh = corpus.start_workflow("planning", "plan the wave for real");
    corpus.jigc_ok(&[
        "doc", "create", "roadmap", "--title", "Roadmap", "--task", &fresh,
    ]);
    let landed = corpus.finalize(&fresh, "planning", "mint the roadmap", false);
    assert!(
        landed.contains("task minted") || !landed.is_empty(),
        "the fresh task must finalize without a reconciliation block; got:\n{landed}",
    );
    // The commit really landed — proof the fresh loop was not silently gated.
    let tracked = corpus.git(&["ls-files"]);
    assert!(
        tracked.lines().any(|line| line == "docs/roadmap.md"),
        "the fresh task's finalize must have landed its commit; git ls-files:\n{tracked}",
    );
}
