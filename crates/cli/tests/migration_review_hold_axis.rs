//! M52 Increment 9, T4 — **the exit-4 review hold fires wherever the composed body
//! promises it.**
//!
//! `step:migration-finalize` is the one step whose text states the hold: *"a plain
//! finalize commits NOTHING — it renders the foreign source against the canonical
//! rewrite and holds (exit 4)"*. Any workflow whose body includes that step composes
//! that promise, and until this increment `task.rs` decided whether to keep it by
//! asking a **different** question — whether the task dir holds a staged source seam,
//! which only `jigc migrate` writes. The two agree on the twelve verb-routed members
//! and disagree everywhere else: a migrate-shaped workflow composed **by name** minted
//! a task whose own composed text promised a hold, and whose plain `jigc task finalize`
//! landed a commit at exit 0.
//!
//! The subject is therefore the **composed contract**, not the staged source, and not a
//! hand list of workflow ids — a project-layer migrate-shaped workflow declaring no
//! `suppressed.door` (the cell T2's refusal deliberately leaves open, since `door` is
//! what T2 keys on) is precisely what reaches this arm.
//!
//! The axis is the cross of *(the body composes `step:migration-finalize`)* ×
//! *(a source seam is staged)*. Three of its four cells are driven below; the fourth
//! — a staged source under a body that composes no such promise, which a project pack
//! could ship as a `migrate-<ty>` workflow omitting the step — is the one cell this
//! change does **not** touch: it held on the seam before and still does, because
//! `staged_migration` alone already satisfies the predicate.
//!
//! | composes the step | staged source | expectation |
//! |---|---|---|
//! | yes | yes | the hold, with the fidelity diff — `jigc migrate … --as adr` |
//! | yes | no  | the hold, saying it has no fidelity diff — the off-verb cell |
//! | no  | no  | no hold — an ordinary task lands at exit 0 |
//!
//! Every arm drives the real binary through [`support::trial_corpus`].

use crate::support;

use support::trial_corpus::{FixturePack, State, TrialCorpus};

/// A **project-layer migrate-shaped workflow**: its body includes
/// `step:migration-finalize`, so the composed text promises the exit-4 hold, and it
/// declares no `suppressed.door`, so T2's verb-routed refusal lets it compose and
/// nothing stages a foreign source for it.
const OFF_VERB_WORKFLOW: &str = "\
---
when: migrate a decision record reached by name rather than by verb
description: A migrate-shaped workflow that declares no door — the project-layer cell.
usage: the fixture cell for a migrate-shaped workflow composed by name.
creates-task: true
selectable: false
suppressed:
  reason: fixture-only — the off-verb migrate-shaped cell, reached by name
  expires: never
allows-create: [{type: adr, as: decision}]
---
{{ include: step:author-migration-adr }}
{{ include: step:migration-finalize }}
";

/// The foreign ADR the verb-routed arm migrates.
const FOREIGN_ADR: &str = "\
# Use Postgres

## Status
Accepted

## Context
The prototype's flat files stopped answering the queries the reports needed.

## Decision
We will use PostgreSQL as the primary datastore.

## Consequences
Every deployment now carries a database to operate.
";

const FOREIGN_ADR_PATH: &str = "docs/adr/0001-use-postgres.md";

/// Author an `adr` in `task` to a finalizable state and return the address the binary
/// **emitted** from `doc create` — never a test-side re-spelling of the slug rule.
fn author_adr(corpus: &TrialCorpus, task: &str, title: &str) -> String {
    let address = corpus
        .jigc_ok(&["doc", "create", "adr", "--title", title, "--task", task])
        .trim_end_matches('\n')
        .to_string();
    for (slot, prose) in [
        ("context", "The forces that made the decision necessary."),
        (
            "decision",
            "We will use PostgreSQL as the primary datastore.",
        ),
        ("consequences", "Every deployment now carries a database."),
    ] {
        corpus.set_slot(&format!("{address}#{slot}"), task, prose);
    }
    address
}

/// Author the task's `commit` doc without finalizing — the fixture builder's
/// [`TrialCorpus::finalize`] does both, and an arm that expects a non-zero finalize
/// needs the two apart.
fn author_commit(corpus: &TrialCorpus, task: &str, summary: &str) {
    corpus.set_field(&format!("commit:{task}#type"), task, "docs");
    corpus.set_slot(&format!("commit:{task}#summary"), task, summary);
}

fn head_sha(corpus: &TrialCorpus) -> String {
    corpus.git(&["rev-parse", "HEAD"])
}

/// Cell `(composes: yes, staged source: no)` — the off-verb migrate-shaped task.
///
/// Plain `jigc task finalize` must hold at exit 4 with HEAD unmoved and nothing
/// promoted; `--approve` must then land. Before this increment the plain run committed
/// at exit 0, contradicting the task's own composed body.
#[test]
fn an_off_verb_migrate_shaped_task_holds_on_a_plain_finalize_and_lands_on_approve() {
    let pack = FixturePack::from_dev_pack("offverb-hold");
    pack.write_workflow("offverb", OFF_VERB_WORKFLOW);
    let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);
    let task = corpus.start_workflow("offverb", "migrate the old decision");

    author_adr(&corpus, &task, "Off Verb Decision");
    author_commit(&corpus, &task, "migrate the old decision");

    let before = head_sha(&corpus);
    let held = corpus.jigc(&["task", "finalize", &task]);
    let stdout = String::from_utf8_lossy(&held.stdout).to_string();
    assert_eq!(
        held.status.code(),
        Some(4),
        "the composed body promises a review hold at exit 4; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&held.stderr),
    );
    assert!(
        stdout.contains("migration review required"),
        "the hold names itself on the same surface the verb-routed hold uses:\n{stdout}",
    );
    // The hold cannot render a fidelity diff — there is no foreign source to diff
    // against — so it says so rather than showing an empty one.
    assert!(
        stdout.contains("no foreign source"),
        "the source-less hold states its empty case rather than rendering a blank diff:\n\
         {stdout}",
    );
    assert!(
        !stdout.contains("--- foreign source (staged seam)"),
        "the source-less hold renders no foreign-source block:\n{stdout}",
    );
    assert_eq!(
        before,
        head_sha(&corpus),
        "the review hold commits NOTHING — HEAD must be unmoved",
    );
    assert!(
        !corpus.repo().join("docs/decisions").exists(),
        "the review hold promotes nothing",
    );

    // …and the same task approves.
    let landed = corpus.jigc_ok(&["task", "finalize", &task, "--approve"]);
    assert_ne!(
        before,
        head_sha(&corpus),
        "`--approve` lands the commit the hold withheld; stdout:\n{landed}",
    );
    assert!(
        corpus
            .repo()
            .join("docs/decisions/off-verb-decision.md")
            .exists(),
        "`--approve` promotes the canonical doc",
    );
}

/// Cell `(composes: no, staged source: no)` — the control.
///
/// A task on the very same corpus whose workflow does **not** include
/// `step:migration-finalize` must be untouched by the widened predicate: its plain
/// finalize lands at exit 0.
#[test]
fn a_non_migration_task_on_the_same_corpus_is_unaffected() {
    let pack = FixturePack::from_dev_pack("offverb-control");
    pack.write_workflow("offverb", OFF_VERB_WORKFLOW);
    let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);
    let task = corpus.start_workflow("record-decision", "decide the datastore");

    author_adr(&corpus, &task, "Datastore Choice");
    author_commit(&corpus, &task, "record the datastore decision");

    let before = head_sha(&corpus);
    let landed = corpus.jigc(&["task", "finalize", &task]);
    assert!(
        landed.status.success(),
        "a workflow that composes no migration-finalize step holds nothing; stdout:\n{}\n\
         stderr:\n{}",
        String::from_utf8_lossy(&landed.stdout),
        String::from_utf8_lossy(&landed.stderr),
    );
    assert_ne!(
        before,
        head_sha(&corpus),
        "the control task's plain finalize lands its commit",
    );
}

/// Cell `(composes: yes, staged source: yes)` — the shipped verb-routed happy path.
///
/// The widened predicate must leave it byte-identical: the plain finalize still holds
/// at exit 4 **with** the fidelity diff, and `--approve` still lands the canonical doc
/// byte-for-byte as the working area staged it, retiring the foreign original.
#[test]
fn the_verb_routed_migration_still_holds_with_its_diff_and_approves_byte_identically() {
    let corpus = TrialCorpus::build(State::Fresh);
    let foreign = corpus.repo().join(FOREIGN_ADR_PATH);
    std::fs::create_dir_all(foreign.parent().expect("the foreign source has a parent"))
        .expect("create the foreign adr dir");
    std::fs::write(&foreign, FOREIGN_ADR).expect("write the foreign adr");
    corpus.git(&["add", FOREIGN_ADR_PATH]);
    corpus.git(&["commit", "-q", "-m", "add the foreign adr"]);

    let minted = corpus.jigc_ok(&["migrate", FOREIGN_ADR_PATH, "--as", "adr"]);
    let task = minted
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .expect("the migrate door prints the minted task id")
        .trim()
        .to_string();

    let address = author_adr(&corpus, &task, "Use Postgres");
    author_commit(&corpus, &task, "migrate the postgres decision");

    let staged = corpus
        .repo()
        .join(".jigc/tasks")
        .join(&task)
        .join("docs")
        .join(format!("{address}.md"));
    let staged_bytes = std::fs::read_to_string(&staged)
        .unwrap_or_else(|e| panic!("read the staged canonical doc at {staged:?}: {e}"));

    let before = head_sha(&corpus);
    let held = corpus.jigc(&["task", "finalize", &task]);
    let stdout = String::from_utf8_lossy(&held.stdout).to_string();
    assert_eq!(
        held.status.code(),
        Some(4),
        "the verb-routed migration still holds; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&held.stderr),
    );
    assert!(
        stdout.contains("migration review required")
            && stdout.contains("--- foreign source (staged seam)")
            && stdout.contains("The prototype's flat files stopped answering"),
        "the staged-source hold still renders the fidelity diff:\n{stdout}",
    );
    assert_eq!(before, head_sha(&corpus), "the hold commits nothing");

    corpus.jigc_ok(&["task", "finalize", &task, "--approve"]);
    let landed = support::trial_corpus::read(&corpus.repo(), "docs/decisions/use-postgres.md");
    assert_eq!(
        staged_bytes, landed,
        "`--approve` promotes the staged canonical doc BYTE-IDENTICALLY",
    );
    assert!(
        !corpus.repo().join(FOREIGN_ADR_PATH).exists(),
        "`--approve` retires the foreign original",
    );
}
