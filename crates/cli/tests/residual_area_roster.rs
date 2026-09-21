//! M53 Increment 3 / T2 — **the enumerators take the residual predicate**
//! (`completions/artifacts/M53/settle-record.md` → D3, the first of the predicate's three
//! homes; `implementation/roadmap.md` → M53 Increment 3).
//!
//! **The lie this suite retires.** A directory under `.jigc/tasks/` was a task because it
//! was a directory. `engine::state::list_active_task_ids` filtered on `entry.path().is_dir()`
//! and nothing else, so a bare `mkdir .jigc/tasks/anything` — reachable on the shipped binary
//! with one command, and reachable *without* one wherever a teardown faulted partway — was
//! listed by `jigc task list` as an active task, rendered by `jigc start` as work in progress
//! with a resume route, and named in the `also open:` block a work-starting `jigc start`
//! appends. Every one of those surfaces was reporting a work unit that does not exist.
//!
//! **What replaces it.** A task is an area that carries its base pin:
//! `symlink_metadata(<area>/base.json).is_file()` — existence, never a parse, and read
//! through the writer registry's own member 0 rather than a spelled-out filename
//! ([`engine::state::WorkArea::jigc_written`]). Its converse — *every legitimate area
//! carries the pin at the first moment a door can observe it* — is the load-bearing half,
//! and it is fenced per production mint door in `mint_door_base_pin.rs` (T1), not assumed
//! here.
//!
//! **The axis this suite iterates**, the one D3 names: `{empty dir · a foreign file · a
//! foreign docs/<ty>:<slug>.md} × {a plain id · a sub-task of a joined milestone · a
//! sub-task of an open milestone}`, over every door that *enumerates* (the roster in both
//! formats, orientation in both formats, and the `also open:` block). The by-id doors are
//! the predicate's second home and are fenced by their own suites.
//!
//! **Why three shapes rather than one.** They differ in what the *destroying* doors think
//! of the same directory, which is the property a roster fix must not quietly change: the
//! foreign file is `uninstall.foreign-bytes`' subject and must stay named after this change
//! ([`a_foreign_file_in_a_residual_is_still_named_by_the_destroying_door`]), while the
//! `docs/<ty>:<slug>.md` shape is the *worst* cell precisely because it is the one shape
//! jigc's own writer could have produced — `engine::state::staged_doc_id` recognises it, so
//! it is **not** foreign, and an area holding nothing else still has no pin and is still not
//! a task.
//!
//! **Why one corpus carries three residuals.** The cells are independent at the enumerator
//! (it reads names under one directory), and each milestone placement costs a real mint,
//! record commit and — for the joined row — a real docs-only boundary. Three corpora × three
//! planted ids covers the nine cells while driving each placement's arc once.

use crate::support;

use serde_json::Value;
use std::fs;
use std::path::Path;
use support::trial_corpus::{State, TrialCorpus};

// ───────────────────────────── the three residual shapes ─────────────────────────────

/// What a residual directory holds. The pin is absent in all three — that is what makes
/// them residuals; the shapes differ in what *else* is in there.
#[derive(Clone, Copy, Debug)]
enum Shape {
    /// Nothing at all — the bare `mkdir`, and the shape an interrupted teardown leaves.
    EmptyDir,
    /// One file jigc did not write, at the area root — the destroying doors' subject.
    ForeignFile,
    /// One file under `docs/` wearing a **staged-instance name** (`<type>:<slug>.md`): the
    /// only shape here that jigc's own writer could have produced, so the foreign-bytes
    /// probe does not name it and the pin is the only thing that distinguishes it from a
    /// live task's staged area.
    ForeignStagedName,
}

impl Shape {
    /// Every shape. A sweep that iterates this picks up a fourth with no edit.
    const ALL: &'static [Shape] = &[
        Shape::EmptyDir,
        Shape::ForeignFile,
        Shape::ForeignStagedName,
    ];

    /// The shape's label, for a failure message that says which cell fell over.
    fn label(self) -> &'static str {
        match self {
            Shape::EmptyDir => "empty dir",
            Shape::ForeignFile => "a foreign file",
            Shape::ForeignStagedName => "a foreign docs/<ty>:<slug>.md",
        }
    }

    /// Plant this shape at `.jigc/tasks/<id>/`, replacing whatever is there.
    ///
    /// The area is **removed first**, so a placement that mints a real sub-task area and
    /// then residualizes it cannot leave the pin behind and pass by accident.
    fn plant(self, repo: &Path, id: &str) {
        let area = repo.join(".jigc").join("tasks").join(id);
        let _ = fs::remove_dir_all(&area);
        fs::create_dir_all(&area).expect("plant the residual area");
        match self {
            Shape::EmptyDir => {}
            Shape::ForeignFile => {
                fs::write(area.join("notes.txt"), "a third party's bytes\n")
                    .expect("plant the foreign file");
            }
            Shape::ForeignStagedName => {
                let docs = area.join("docs");
                fs::create_dir_all(&docs).expect("plant the residual docs/ tree");
                fs::write(docs.join("adr:leftover.md"), "# Leftover\n")
                    .expect("plant the staged-looking body");
            }
        }
        assert!(
            !area.join("base.json").exists(),
            "a planted residual must carry no base pin, or this suite proves nothing",
        );
    }
}

// ───────────────────────────── the three placements ─────────────────────────────

/// Plant one residual per [`Shape`] at the given ids, in order, and hand back the ids.
fn plant_each_shape(repo: &Path, ids: &[String]) {
    assert_eq!(
        ids.len(),
        Shape::ALL.len(),
        "one id per shape — the cell count is the axis",
    );
    for (shape, id) in Shape::ALL.iter().zip(ids) {
        shape.plant(repo, id);
    }
}

/// `jigc milestone add-task <milestone> "<intent>"`, returning the **minted** sub-task id
/// read off the ack rather than re-slugged in test code.
fn add_sub_task(corpus: &TrialCorpus, milestone: &str, intent: &str) -> String {
    let ack = corpus.jigc_ok(&["milestone", "add-task", milestone, intent]);
    let (_, rest) = ack
        .split_once("added task:")
        .unwrap_or_else(|| panic!("`milestone add-task` names the task it minted; got:\n{ack}"));
    rest.split_whitespace()
        .next()
        .expect("the ack's task id is one token")
        .to_string()
}

/// A corpus carrying one milestone with three real, minted sub-task areas — the substrate
/// both milestone placements start from, before either residualizes anything.
fn milestone_with_three_sub_tasks() -> (TrialCorpus, Vec<String>) {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    let ids: Vec<String> = ["Warm the read cache", "Shard the index", "Trim the log"]
        .iter()
        .map(|intent| add_sub_task(&corpus, "cache-rework", intent))
        .collect();
    (corpus, ids)
}

/// A corpus whose milestone is **open** (`status: active`) and whose three recorded
/// sub-tasks' working areas have each been replaced by a residual.
fn open_milestone_corpus() -> (TrialCorpus, Vec<String>) {
    let (corpus, ids) = milestone_with_three_sub_tasks();
    plant_each_shape(&corpus.repo(), &ids);
    (corpus, ids)
}

/// A corpus whose milestone is **joined** — a real docs-only boundary landed, so the record
/// is terminal and jigc removed every sub-task area — with a residual then planted at each
/// recorded sub-task id. The cell D2.6 calls the false-flip path.
fn joined_milestone_corpus() -> (TrialCorpus, Vec<String>) {
    let (corpus, ids) = milestone_with_three_sub_tasks();
    // One sub-task contributes a promotable doc, so the boundary is not a
    // `milestone.zero-contribution` refusal. Staged the way a fanned-out sub-agent leaves
    // it: the body plus its provenance bit. The residuals are planted **after** the
    // boundary, so the join sees three ordinary sub-tasks and nothing else.
    let docs = corpus
        .repo()
        .join(".jigc")
        .join("tasks")
        .join(&ids[0])
        .join("docs");
    fs::create_dir_all(&docs).expect("mk the contributing sub-task's docs/");
    fs::write(
        docs.join("adr:warm-policy.md"),
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# Warm policy\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n",
    )
    .expect("stage the contributed doc");
    fs::write(
        docs.join("provenance.json"),
        "{\"docs\":{\"adr:warm-policy\":\"created\"}}",
    )
    .expect("stage the provenance bit");

    corpus.jigc_ok(&["milestone", "finalize", "cache-rework"]);
    let record = corpus.jigc_ok(&["doc", "show", "milestone-record:cache-rework"]);
    assert!(
        record.contains("status: joined"),
        "the milestone must really be joined, or this placement is the open one again; got:\n{record}",
    );
    assert!(
        !corpus
            .repo()
            .join(".jigc")
            .join("tasks")
            .join(&ids[0])
            .exists(),
        "the boundary removes its sub-task areas — the residual below must be planted, not left",
    );

    plant_each_shape(&corpus.repo(), &ids);
    (corpus, ids)
}

// ───────────────────────────── what every enumerating door must say ────────────────────

/// Every door that **enumerates** the active set, asked of a corpus holding `ids` as
/// residuals and no legitimate task: none of them may name one.
///
/// The roster's own emptiness is asserted too, not merely the ids' absence — a surface that
/// printed nothing at all would satisfy a bare `!contains`.
fn assert_no_enumerating_door_names(corpus: &TrialCorpus, ids: &[String], placement: &str) {
    let names = |surface: &str, text: &str| {
        for (shape, id) in Shape::ALL.iter().zip(ids) {
            assert!(
                !text.contains(id.as_str()),
                "[{placement} · {}] `{surface}` named the residual `{id}` — a directory with \
                 no base pin is not a task at any door; got:\n{text}",
                shape.label(),
            );
        }
    };

    let roster = corpus.jigc_ok(&["task", "list"]);
    names("jigc task list", &roster);
    assert!(
        roster.contains("no active tasks"),
        "[{placement}] the roster over residuals alone is the empty roster, not a silent one; \
         got:\n{roster}",
    );

    let raw = corpus.jigc_ok(&["task", "list", "--format", "json"]);
    names("jigc task list --format json", &raw);
    let rows: Value = serde_json::from_str(&raw).expect("the roster envelope is valid JSON");
    assert_eq!(
        rows.as_array().map(Vec::len),
        Some(0),
        "[{placement}] the machine roster must be the empty array; got:\n{rows:#}",
    );

    let orientation = corpus.jigc_ok(&["start"]);
    names("jigc start", &orientation);
    assert!(
        !orientation.contains("Active task:"),
        "[{placement}] orientation must render the clean view over residuals; got:\n{orientation}",
    );

    let raw = corpus.jigc_ok(&["start", "--format", "json"]);
    names("jigc start --format json", &raw);
    let view: Value = serde_json::from_str(&raw).expect("the orientation envelope is valid JSON");
    assert_eq!(
        view["state"], "clean",
        "[{placement}] the versioned envelope must tag the state clean; got:\n{view:#}",
    );

    // Last, because it mints: the `also open:` block a work-starting form appends. A
    // residual was *already open* work as far as the shipped enumerator was concerned.
    let started = corpus.jigc_ok(&["start", "Add a rate limiter"]);
    names("jigc start \"<intent>\"", &started);
    assert!(
        !started.contains("also open:"),
        "[{placement}] nothing was open, so the block renders no bytes at all; got:\n{started}",
    );
}

// ───────────────────────────── the three placement cells ─────────────────────────────

/// Placement 1 — **a plain id**: three residuals under `.jigc/tasks/`, belonging to no
/// milestone. The cell one `mkdir` reaches on the shipped binary.
#[test]
fn a_plain_residual_is_a_task_at_no_enumerating_door() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let ids: Vec<String> = ["stray-alpha", "stray-beta", "stray-gamma"]
        .iter()
        .map(|id| (*id).to_string())
        .collect();
    plant_each_shape(&corpus.repo(), &ids);

    assert_no_enumerating_door_names(&corpus, &ids, "plain id");
}

/// Placement 2 — **a sub-task of an open milestone**: the record still lists all three as
/// `active`, and their areas are residuals. The record's word is not the roster's: the
/// roster answers on the area, and the area has no pin.
#[test]
fn a_sub_task_residual_of_an_open_milestone_is_a_task_at_no_enumerating_door() {
    let (corpus, ids) = open_milestone_corpus();

    assert_no_enumerating_door_names(&corpus, &ids, "sub-task of an open milestone");
}

/// Placement 3 — **a sub-task of a joined milestone**: the boundary landed, the record is
/// terminal, and a directory reappeared at each settled sub-task's id. This is the cell
/// where the lie was worst — the roster offered live work under a milestone that can never
/// be finalized again.
#[test]
fn a_sub_task_residual_of_a_joined_milestone_is_a_task_at_no_enumerating_door() {
    let (corpus, ids) = joined_milestone_corpus();

    assert_no_enumerating_door_names(&corpus, &ids, "sub-task of a joined milestone");
}

// ───────────────────────────── the control the fix must not move ───────────────────────

/// The **destroying** doors' subject is unchanged: a residual holding a foreign file is
/// still named by `uninstall.foreign-bytes`, at the path it lives at.
///
/// This is the control for the whole change. The predicate decides what is a *work unit*;
/// it decides nothing about whose *bytes* are in a directory, and a roster fix that
/// quietly narrowed the destroying doors' subject would turn "not a task" into "safe to
/// delete" — which is the one conversion this increment must not make.
#[test]
fn a_foreign_file_in_a_residual_is_still_named_by_the_destroying_door() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    Shape::ForeignFile.plant(&corpus.repo(), "stray-beta");
    Shape::ForeignStagedName.plant(&corpus.repo(), "stray-gamma");

    let out = corpus.jigc(&["uninstall"]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        text.contains("uninstall.foreign-bytes"),
        "a foreign byte in a residual still refuses the teardown; got:\n{text}",
    );
    assert!(
        text.contains(".jigc/tasks/stray-beta/notes.txt"),
        "and the refusal still names the path it would have destroyed; got:\n{text}",
    );
    assert!(
        corpus
            .repo()
            .join(".jigc")
            .join("tasks")
            .join("stray-beta")
            .join("notes.txt")
            .is_file(),
        "a refused teardown destroys nothing",
    );
    // The staged-looking shape is jigc's own writer's form, so it is NOT foreign — stated
    // here so the asymmetry is a pinned property rather than a surprise in the field.
    assert!(
        !text.contains("adr:leftover.md"),
        "a `<type>:<slug>.md` under `docs/` is a name jigc's own writer produces, so the \
         foreign-bytes probe does not claim it; got:\n{text}",
    );
}
