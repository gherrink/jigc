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
//! hand list of workflow ids — a migrate-shaped workflow declaring no `suppressed.door`
//! (the cell T2's refusal deliberately leaves open, since `door` is what T2 keys on) is
//! precisely what reaches this arm. *(This read "a **project-layer** migrate-shaped
//! workflow" and was corrected 2026-09-20: the project layer was the one home the seam
//! could not reach — see the layer table below.)*
//!
//! The axis is the cross of *(the body composes `step:migration-finalize`)* ×
//! *(a source seam is staged)* × **the layer the workflow lives in**. Three of the
//! first cross's four cells are driven below; the fourth — a staged source under a body
//! that composes no such promise, which a project pack could ship as a `migrate-<ty>`
//! workflow omitting the step — is the one cell this change does **not** touch: it held
//! on the seam before and still does, because `staged_migration` alone already satisfies
//! the predicate.
//!
//! | composes the step | staged source | expectation |
//! |---|---|---|
//! | yes | yes | the hold, with the fidelity diff — `jigc migrate … --as adr` |
//! | yes | no  | the hold, saying it has no fidelity diff — the off-verb cell |
//! | no  | no  | no hold — an ordinary task lands at exit 0 |
//!
//! **The third dimension, added 2026-09-20 (the validate→fix loop).** T4 shipped the
//! `(yes, no)` cell over a **project pack** — `FixturePack`, a pack constituent — and
//! described it as *"a project-layer migrate-shaped workflow"*. Those are two different
//! homes, and only the pack one reached the arm: `TaskArea::recorded_workflow` resolved
//! the recorded id through `PackSource::read` while the compose door resolves it through
//! `CascadeDefs::read_workflow`, where a project `.jigc/config/workflows/<id>.yaml`
//! whole-file shadow wins and a project-layer-only id resolves at all. So the same
//! `(composes: yes, staged: no)` cell answered **exit 0 with HEAD moved** through the
//! project *layer* while answering exit 4 through the project *pack*. The layer is
//! therefore an axis of its own, and its two members are driven here:
//!
//! | layer | how the definition is reached | cell |
//! |---|---|---|
//! | embedded pack | the shipped `migrate-*` members | the verb-routed arm below |
//! | project pack | `FixturePack`, a `packs.yaml` constituent | the off-verb arm below |
//! | project cascade layer — shadow | `.jigc/config/workflows/single-task.yaml` over a shipped id | [`a_project_layer_shadow_composing_the_step_holds_like_its_pack_siblings`] |
//! | project cascade layer — only | `.jigc/config/workflows/<id>.yaml` for an id no pack ships | [`a_project_layer_only_migrate_shaped_workflow_holds_rather_than_dying_unreadable`] |
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

/// The body a **project cascade layer** shadow of the shipped `single-task` id carries —
/// migrate-shaped, so its composed text promises the exit-4 hold, under an id every pack
/// already ships (which is what makes it a *shadow* rather than a new definition).
const SHADOW_OF_A_SHIPPED_ID: &str = "\
---
when: implement a single scoped change
description: A project-layer shadow whose body composes the migration review hold.
usage: the project cascade layer's shadow cell.
creates-task: true
allows-create: [{type: adr, as: decision}]
---
{{ include: step:locate }}
{{ include: step:migration-finalize }}
";

/// The body a **project-layer-only** workflow carries — the same migrate shape under an
/// id no pack ships, so the definition exists in exactly one home: the cascade's project
/// layer.
const PROJECT_LAYER_ONLY: &str = "\
---
when: migrate a decision record reached by name rather than by verb
description: A migrate-shaped workflow no pack ships — the project-layer-only cell.
usage: the project cascade layer's only-home cell.
creates-task: true
allows-create: [{type: adr, as: decision}]
---
{{ include: step:author-migration-adr }}
{{ include: step:migration-finalize }}
";

/// Install a project cascade-layer workflow definition at
/// `.jigc/config/workflows/<id>.yaml` — the whole-file shadow `design/overrides.md`
/// declares, written directly because no verb installs one.
fn write_project_layer_workflow(corpus: &TrialCorpus, id: &str, yaml: &str) {
    let dir = corpus.repo().join(".jigc/config/workflows");
    std::fs::create_dir_all(&dir).expect("create the project layer's workflows dir");
    std::fs::write(dir.join(format!("{id}.yaml")), yaml).expect("write the project shadow");
}

/// Cell `(composes: yes, staged source: no, layer: project cascade layer — shadow)`.
///
/// A project `workflows/single-task.yaml` shadow whose body includes
/// `step:migration-finalize` composes the exit-4 promise, so the seam owes the hold. It
/// did not: the seam read the **pack's** `single-task`, which composes no such step, and
/// the plain finalize landed a commit at exit 0 with HEAD moved — the exact cell T4's own
/// entry named and did not reach.
#[test]
fn a_project_layer_shadow_composing_the_step_holds_like_its_pack_siblings() {
    let corpus = TrialCorpus::build(State::Fresh);
    write_project_layer_workflow(&corpus, "single-task", SHADOW_OF_A_SHIPPED_ID);
    let task = corpus.start_workflow("single-task", "shadow the hold");

    // The composed body is what makes the promise — assert it before asserting the hold,
    // so a pack edit that drops the step fails here rather than silently hollowing the arm.
    let composed = corpus.jigc_ok(&["start", "--task", &task]);
    assert!(
        composed.contains("commits NOTHING"),
        "the project-layer shadow composes the exit-4 promise:\n{composed}",
    );

    author_adr(&corpus, &task, "Shadowed Decision");
    author_commit(&corpus, &task, "shadow the hold");

    let before = head_sha(&corpus);
    let held = corpus.jigc(&["task", "finalize", &task]);
    let stdout = String::from_utf8_lossy(&held.stdout).to_string();
    assert_eq!(
        held.status.code(),
        Some(4),
        "a promise made by the project cascade layer is still a promise; stdout:\n{stdout}\n\
         stderr:\n{}",
        String::from_utf8_lossy(&held.stderr),
    );
    assert!(
        stdout.contains("migration review required") && stdout.contains("no foreign source"),
        "the source-less hold names itself and states its empty case:\n{stdout}",
    );
    assert_eq!(
        before,
        head_sha(&corpus),
        "the review hold commits NOTHING — HEAD must be unmoved",
    );

    corpus.jigc_ok(&["task", "finalize", &task, "--approve"]);
    assert_ne!(
        before,
        head_sha(&corpus),
        "`--approve` lands the commit the hold withheld",
    );
}

/// Cell `(composes: yes, staged source: no, layer: project cascade layer — only)`.
///
/// The compose door resolves a project-layer-only id at exit 0, so every door downstream
/// owes the same resolution. Before the fix the task was **bricked**: `doc create` and
/// `doc author` refused with `pack.resource-missing`, `task bind` and `task finalize`
/// died with *"no pack resource of kind workflows"* — a task the product minted and then
/// could not be authored in, let alone held.
#[test]
fn a_project_layer_only_migrate_shaped_workflow_holds_rather_than_dying_unreadable() {
    let corpus = TrialCorpus::build(State::Fresh);
    write_project_layer_workflow(&corpus, "offverb", PROJECT_LAYER_ONLY);
    let task = corpus.start_workflow("offverb", "offverb only");

    // The create-gate is the first door downstream of the mint, and it reads the same
    // recorded id — a refusal here is the brick, not the hold.
    author_adr(&corpus, &task, "Off Verb Only");
    author_commit(&corpus, &task, "offverb only");

    let before = head_sha(&corpus);
    let held = corpus.jigc(&["task", "finalize", &task]);
    let stdout = String::from_utf8_lossy(&held.stdout).to_string();
    let stderr = String::from_utf8_lossy(&held.stderr).to_string();
    assert!(
        !stderr.contains("no pack resource"),
        "a definition the compose door resolved is not missing at the finalize seam:\n{stderr}",
    );
    assert_eq!(
        held.status.code(),
        Some(4),
        "the project-layer-only body promises the hold too; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(before, head_sha(&corpus), "the hold commits nothing");

    corpus.jigc_ok(&["task", "finalize", &task, "--approve"]);
    assert!(
        corpus
            .repo()
            .join("docs/decisions/off-verb-only.md")
            .exists(),
        "`--approve` promotes the canonical doc",
    );
}
