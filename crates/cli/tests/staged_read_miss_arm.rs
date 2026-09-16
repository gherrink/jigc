//! M51 Increment 5 / T6 — **the `--task` read miss names the copy it looked in and
//! keeps `--task <id>` in its route** (N15).
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
//! ## The producer is one guard, not the `--task` arm in general
//!
//! Both arms share `engine::store::resolve_read_schema`, and its **singleton-slug guard**
//! fires *before* either selects a source — so the staged arm inherited a block phrased for
//! the committed store. Driven on a non-singleton, the staged arm already answers
//! `store.not-staged` correctly, naming its copy and routing at the right door: the correct
//! shape was three inches away in the same file.
//!
//! ## What this suite drives
//!
//! Both reads of one non-canonical singleton address on a `committed-singletons` corpus with
//! a live task, through the real binary:
//!
//! * the two surfaces are **no longer byte-identical** — the regression guard for the whole
//!   finding, since the defect *was* their equality;
//! * the `--task` arm **names the staged copy and the task**, never says *committed*, and
//!   carries `--task <id>` in a route that runs;
//! * the route, **run verbatim**, serves the **staged** bytes — which is the fact the
//!   dropped `--task` destroyed. The task stages a copy whose thesis differs from the
//!   committed one, so a route that quietly served the committed copy would be visible
//!   here rather than indistinguishable;
//! * the task-less arm is **byte-identical to HEAD's** — pinned as a literal, because the
//!   fix's scope is the staged arm and a committed-arm byte move would be a silent
//!   contract change on a 1.0-pinned read surface.

use crate::support;
use support::trial_corpus::{State, TrialCorpus};

/// The non-canonical singleton address both arms are driven with — `vision` is a
/// `placement` singleton, so every slug but `vision` names no instance in either copy.
const MISS: &str = "vision:wrong-slug";

/// The thesis the task stages over the committed one, so *which copy the route served*
/// is a visible fact rather than an inference.
const STAGED_THESIS: &str = "The staged copy says something else entirely.";

/// The task-less arm's finding block, verbatim at HEAD. Spelled out rather than derived:
/// the claim is that these bytes **do not move**, and a derivation would move with them.
const COMMITTED_ARM: &str = "\
blocking · store.not-found — `vision:wrong-slug` names no committed doc: `vision` is a singleton, so its only address is `vision:vision`
  at: vision:wrong-slug
  route: read `vision:vision` — a singleton doctype has one instance at a fixed slug
";

/// The finding block a read prints — stderr up to (not including) the adapter trailer, so
/// the pin is over the finding and not over the footer every surface carries.
fn finding_block(out: &std::process::Output) -> String {
    let err = String::from_utf8_lossy(&out.stderr).into_owned();
    match err.find("— jigc ·") {
        Some(at) => err[..at].to_string(),
        None => err,
    }
}

/// **The arm** — the two reads of one miss address, and the three facts that separate them.
#[test]
fn the_task_read_miss_names_the_staged_copy_and_keeps_the_task_in_its_route() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    // `form-vision` is the workflow that gates the vision, and the write copies the
    // committed doc into the task — so the staged copy really exists and really differs.
    let task = corpus.start_workflow("form-vision", "revise the vision");
    corpus.set_slot("vision:vision#thesis", &task, STAGED_THESIS);

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

    // The defect *was* the equality: one block answered for two different copies.
    assert_ne!(
        staged_block, committed_block,
        "the `--task` read looked in the staged copy and the task-less read in the \
         committed one — one block cannot answer for both",
    );

    // The staged arm: which copy it looked in, and a route that keeps `--task`.
    assert!(
        staged_block.contains("store.not-found"),
        "the staged arm keeps the family's code: {staged_block}",
    );
    assert!(
        staged_block.contains("staged") && staged_block.contains(task.as_str()),
        "the staged arm must name the copy it looked in and the task it looked in it for: \
         {staged_block}",
    );
    assert!(
        !staged_block.contains("committed"),
        "the staged arm never consulted the committed store, so it may not say so: \
         {staged_block}",
    );
    assert!(
        staged_block.contains(&format!("--task {task}")),
        "the staged arm's route must keep `--task {task}` — dropped, it serves the other \
         copy than the one the caller asked for: {staged_block}",
    );

    // The route, run verbatim, serves the STAGED copy — the fact the dropped `--task`
    // destroyed: followed exactly, the old route served committed prose to a caller who
    // had asked for the staged one.
    let repaired = corpus.jigc(&["doc", "show", "vision:vision#thesis", "--task", &task]);
    assert!(
        repaired.status.success(),
        "the staged arm's route must be followable: {}",
        String::from_utf8_lossy(&repaired.stderr),
    );
    assert_eq!(
        String::from_utf8_lossy(&repaired.stdout).trim(),
        STAGED_THESIS,
        "the route keeps the caller in the copy they asked for",
    );

    // The committed arm's bytes do not move.
    assert_eq!(
        committed_block, COMMITTED_ARM,
        "the task-less arm is out of this fix's scope — its bytes are a 1.0-pinned surface",
    );
}
