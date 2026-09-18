//! M51 Increment 5 / T6 — **the `--task` read miss names the copy it looked in and
//! keeps `--task <id>` in its route** (N15) — **re-pointed at M52 Increment 6 / T2**,
//! when the address this suite was built on stopped reaching either read arm.
//!
//! Cited as the `pinned-by:` for the rc.14 trial's
//! [findings-verification](../../../completions/artifacts/RC-rc14/findings-verification.md)
//! row **F-7**, whose recorded reason (*"carried defect, as F-6"*) stopped being true the
//! moment this fix landed — N15 was carried at the trial and fixed in this wave, which is
//! why the conversion ledger's close re-read every row and not only the three the Settle
//! named (M51 Increment 11 / T3).
//!
//! ## The defect, driven
//!
//! `jigc doc show <miss-address> --task <id>` and the task-less read were **byte-identical**
//! at `9ae1f40b`:
//!
//! ```text
//! blocking · store.not-found — `vision:wrong-slug` names no committed doc: `vision` is a
//!   singleton, so its only address is `vision:vision`
//!   at: vision:wrong-slug
//!   route: read `vision:vision` — a singleton doctype has one instance at a fixed slug
//! ```
//!
//! Two lies in one block under `--task`: the word **committed** (the staged arm never
//! looked at the committed store), and a route that **drops `--task`** — followed exactly,
//! it serves the *committed* copy while the caller asked for the staged one
//! (`completions/artifacts/M51/baseline-prose.md` §6; the Settle's §12).
//!
//! ## Why the subject moved, and why the equality is no longer the defect
//!
//! M52 Increment 6 / T2 refuses a non-canonical **fixed-identity** address at the parse
//! boundary (`cli::doc::parse_verb_addr` → `cli::task::reject_fixed_identity_alias`), so
//! `vision:wrong-slug` never reaches `engine::store::resolve_read_schema`'s singleton-slug
//! guard through any `doc` door — and both reads are once again byte-identical.
//!
//! **That equality is legitimate where N15's was not, and the difference is checkable
//! rather than asserted.** N15's block *claimed a copy* — it said `committed` to a caller
//! who had asked for the staged one, and routed at the copy it had not been asked about.
//! The M52 refusal is **copy-blind**: it is a fact about the address, true in both copies
//! and before either is selected, which is the shape `store.unknown-type` has had since
//! M43 (`engine::store::resolve_read_schema`'s own note: *"a doctype id names nothing in
//! either copy, so the unknown-type block is copy-blind and stays shared verbatim"*). So
//! arm 1 below asserts what makes it legitimate — the block **names no copy at all** — and
//! not the equality, which is by itself neither right nor wrong.
//!
//! **N15's property is still live, and arm 2 drives it where it is still reachable**: an
//! address whose doctype does *not* have a fixed identity still reaches both read arms, and
//! the staged one still has to say which copy it looked in. The two arms are asserted to
//! differ there, which is the regression guard the original suite carried.
//!
//! The engine's singleton-slug guard is **kept, not retired** (M52 T2's scope note): the
//! committed arm still answers for `compose`, `doc list` and `validate`, none of which
//! passes through `parse_verb_addr`.
//!
//! ## What this suite drives
//!
//! Every read below runs through the real binary on a `committed-singletons` corpus with a
//! live task that stages a copy whose thesis differs from the committed one — so *which
//! copy a route served* stays a visible fact rather than an inference.

use crate::support;
use support::trial_corpus::{State, TrialCorpus};

/// The non-canonical singleton address the trial hit and M52 now refuses at the door.
const MISS: &str = "vision:wrong-slug";

/// A **committed** doc this task does not stage — the miss shape that still reaches both
/// read arms. It is addressed at the identity its doctype **has**, so M52's door guard is
/// silent and the two arms adjudicate exactly as they did before it: the miss is about the
/// *copy*, which is the question N15 was about.
const NOT_STAGED: &str = "roadmap:roadmap";

/// The thesis the task stages over the committed one, so *which copy the route served* is
/// a visible fact rather than an inference.
const STAGED_THESIS: &str = "The staged copy says something else entirely.";

/// The finding block a read prints — stderr up to (not including) the adapter trailer, so
/// the pin is over the finding and not over the footer every surface carries.
fn finding_block(out: &std::process::Output) -> String {
    let err = String::from_utf8_lossy(&out.stderr).into_owned();
    match err.find("— jigc ·") {
        Some(at) => err[..at].to_string(),
        None => err,
    }
}

/// A corpus with a live `form-vision` task whose staged vision differs from the committed
/// one. Returns `(corpus, task_id)`.
fn corpus_with_staged_vision() -> (TrialCorpus, String) {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    // `form-vision` is the workflow that gates the vision, and the write copies the
    // committed doc into the task — so the staged copy really exists and really differs.
    let task = corpus.start_workflow("form-vision", "revise the vision");
    corpus.set_slot("vision:vision#thesis", &task, STAGED_THESIS);
    (corpus, task)
}

/// **Arm 1** — the address N15 was reported on is refused before either copy is selected,
/// and the refusal claims no copy.
#[test]
fn the_fixed_identity_miss_is_refused_before_either_copy_is_selected() {
    let (corpus, task) = corpus_with_staged_vision();

    let staged = corpus.jigc(&["doc", "show", MISS, "--task", &task]);
    let committed = corpus.jigc(&["doc", "show", MISS]);

    for (label, out) in [("--task", &staged), ("task-less", &committed)] {
        assert!(
            !out.status.success(),
            "the {label} read of `{MISS}` names no doc — it must block non-zero",
        );
        assert_ne!(
            out.status.code(),
            Some(101),
            "the {label} read must refuse with a finding, never a panic",
        );
    }

    let staged_block = finding_block(&staged);
    let committed_block = finding_block(&committed);

    for (label, block) in [("--task", &staged_block), ("task-less", &committed_block)] {
        assert!(
            block.contains("store.fixed-identity"),
            "the {label} read is refused at the door, on the address rather than on a \
             copy: {block}",
        );
        // What makes the two arms' equality legitimate: the block claims neither copy.
        assert!(
            !block.contains("committed") && !block.contains("staged"),
            "a copy-blind refusal may not name a copy — that was N15's whole defect: \
             {block}",
        );
        assert!(
            block.contains("vision:vision"),
            "the {label} read names the identity this doctype does have: {block}",
        );
    }

    // The route, run verbatim, resolves — the identity it names is the one that exists.
    let repaired = corpus.jigc(&["doc", "show", "vision:vision", "--task", &task]);
    assert!(
        repaired.status.success(),
        "the refusal's route must be followable: {}",
        String::from_utf8_lossy(&repaired.stderr),
    );
}

/// **Arm 2** — N15's property, driven on the miss shape that still reaches both read arms:
/// the staged arm names the copy it looked in, and the two arms do not answer alike.
#[test]
fn the_task_read_miss_names_the_staged_copy_it_looked_in() {
    let (corpus, task) = corpus_with_staged_vision();

    let staged = corpus.jigc(&["doc", "show", NOT_STAGED, "--task", &task]);
    let committed = corpus.jigc(&["doc", "show", NOT_STAGED]);

    assert!(
        !staged.status.success(),
        "`{NOT_STAGED}` is committed but not staged in this task — the `--task` read blocks",
    );
    assert!(
        committed.status.success(),
        "`{NOT_STAGED}` is committed, so the task-less read serves it: {}",
        String::from_utf8_lossy(&committed.stderr),
    );

    let staged_block = finding_block(&staged);
    assert!(
        staged_block.contains("store.not-staged"),
        "the staged arm answers for the copy it looked in: {staged_block}",
    );
    assert!(
        staged_block.contains("not staged in this task"),
        "the staged arm must name the copy it looked in — the half of N15 that survives \
         the M52 door guard: {staged_block}",
    );

    // The route, run verbatim, serves the copy it names — the committed one, which is the
    // copy that exists. A route is followable or it is not a route.
    let repaired = corpus.jigc(&["doc", "show", NOT_STAGED]);
    assert!(
        repaired.status.success(),
        "the staged arm's route must be followable: {}",
        String::from_utf8_lossy(&repaired.stderr),
    );
}
