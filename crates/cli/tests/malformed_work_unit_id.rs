//! M50 Increment 1 / T1 — **a work-unit id is checked against the grammar before any
//! door touches the filesystem** (`completions/artifacts/M50/settle-record.md` → D2;
//! `design/structural-grammar.md` → Work-units and runtime identity;
//! `design/surface-contract.md` → the route floor).
//!
//! ## The class this closes
//!
//! `.jigc/tasks/<id>/` and `.jigc/milestones/<id>/` are built by **joining a
//! caller-supplied token onto a path**, and no door asked whether that token was an id
//! at all. Driven at `HEAD~`: `jigc task discard "../.."` resolved its "working area"
//! to the repository root and `remove_dir_all`'d it — `.git`, `.jigc`, every tracked
//! file — printing `discarded task ../..` and **exiting 0**. Its empty-string sibling
//! took `.jigc/tasks/` itself, `jigc task validate ""` and `jigc doc list --task ""`
//! reported a clean task that does not exist, and `jigc doc set-slot … --task ""`
//! minted a phantom `.jigc/tasks/docs/` that `jigc task list` then listed as live.
//!
//! The guard is one predicate (`engine::slug::is_slug`) carrying one code
//! (`work-unit.malformed-id`) and one route (the grammar), and it sits **below clap and
//! above the filesystem op**: a clap `value_parser` would exit 2 with no code and no
//! route, which is the M43 route-floor breach at every door that takes an id. It is
//! applied at the four task-family resolve seams (`TaskArea::resolve`,
//! `ActiveTask::resolve`, `resume_in_repo`, `reenter_in_repo`) and — the milestone side
//! having no single seam — at `MilestoneCommand::dispatch`'s top, above the shared
//! `reseed_cache` every operating milestone verb reaches first.
//!
//! ## What each arm drives
//!
//! Every arm drives the **real binary** over a `support::trial_corpus` fixture, and each
//! asserts the *tree* as well as the text — at `HEAD~` the text alone would have passed
//! over an exit-0 deletion. The milestone arm is the one that pins **position**: with a
//! malformed id at `HEAD~`, `jigc milestone execute` reached `reseed_cache` and
//! materialized both the milestone cache area and a sub-task working area before
//! composing successfully; the guard sits above that, so neither appears.
//!
//! `crates/cli/tests/no_such_task_route.rs` is this change's other half: a *well-formed
//! but unknown* id keeps the converged `jigc task list` route, unchanged.

use std::fs;
use std::path::Path;

use crate::support::trial_corpus::{State, TrialCorpus};

/// The finding code the whole family carries — task doors and milestone doors alike.
const CODE: &str = "work-unit.malformed-id";

/// The grammar the route states, byte-identical to the sentence the three `--slug`
/// doors already ship (`start.rs`, `doc.rs`, `migrate.rs`).
///
/// Spelled out here rather than imported from the production constant on purpose: a
/// contract test that compares the emitted bytes against the very constant that produced
/// them proves only that the constant equals itself.
const GRAMMAR: &str =
    "use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)";

/// Assert `out` is a refusal carrying the family's code and **exactly one** route line
/// that states the grammar — the route floor, counted rather than merely contained.
fn assert_refused(out: &std::process::Output, what: &str) {
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        !out.status.success(),
        "{what} must refuse a malformed work-unit id; exited {}\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
        out.status,
    );
    assert!(
        stderr.contains(CODE),
        "{what} must name `{CODE}`; got:\n{stderr}",
    );
    let routes: Vec<&str> = stderr
        .lines()
        .filter(|line| line.trim_start().starts_with("route:"))
        .collect();
    assert_eq!(
        routes.len(),
        1,
        "{what} must carry exactly one route line; got:\n{stderr}",
    );
    assert!(
        routes[0].contains(GRAMMAR),
        "{what}'s route must state the grammar verbatim; got:\n{}",
        routes[0],
    );
}

/// Every path `git ls-files` reports for `repo`, as repo-relative strings.
fn tracked(corpus: &TrialCorpus) -> Vec<String> {
    corpus
        .git(&["ls-files"])
        .lines()
        .map(str::to_string)
        .collect()
}

/// Assert every path in `paths` is still present under `repo`.
fn all_present(repo: &Path, paths: &[String], what: &str) {
    assert!(!paths.is_empty(), "the fixture must track files at all");
    for path in paths {
        assert!(
            repo.join(path).exists(),
            "{what}: the tracked file `{path}` must survive a refused run",
        );
    }
}

// ───────────────────────── (a) the traversal, with a live task ─────────────────────────

/// **The centrepiece.** `jigc task discard "../.."` refuses, and the repository is still
/// there afterwards — `.git/`, the live task's own working area, and every tracked file.
///
/// The tree assertion is not decoration: at `HEAD~` this exact invocation exited **0**
/// having deleted all three, so a text-only assertion would have passed over the loss.
#[test]
fn task_discard_refuses_a_traversal_and_the_repository_survives() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let repo = corpus.repo();
    let live = corpus
        .live_task()
        .expect("refs-post-hoc leaves a task live");
    let files = tracked(&corpus);

    let out = corpus.jigc(&["task", "discard", "../.."]);
    assert_refused(&out, "`jigc task discard ../..`");

    assert!(repo.join(".git").is_dir(), "`.git/` must survive");
    assert!(
        repo.join(".jigc/tasks").join(live).is_dir(),
        "the live task's working area must survive",
    );
    all_present(&repo, &files, "task discard ../..");
}

// ───────────────────────── (b) the empty id at the destroying door ─────────────────────

/// `jigc task discard ""` refuses, and `.jigc/tasks/` — the directory the empty join
/// resolves to — is still there with its live task in it.
#[test]
fn task_discard_refuses_an_empty_id_and_the_task_roster_survives() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let repo = corpus.repo();
    let live = corpus
        .live_task()
        .expect("refs-post-hoc leaves a task live");

    let out = corpus.jigc(&["task", "discard", ""]);
    assert_refused(&out, "`jigc task discard \"\"`");

    assert!(
        repo.join(".jigc/tasks").is_dir(),
        "`.jigc/tasks/` must survive",
    );
    assert!(
        repo.join(".jigc/tasks").join(live).is_dir(),
        "the live task's working area must survive",
    );
}

// ───────────────────────── (c) the two exit-0 false greens ─────────────────────────────

/// `jigc task validate ""` and `jigc doc list --task ""` both **reported success** over a
/// task that does not exist — validate said *"the task validates clean"*, `doc list` said
/// *"no docs staged"*. Both now refuse with the family's code and route.
#[test]
fn the_read_doors_stop_reporting_a_clean_nonexistent_task() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);

    assert_refused(
        &corpus.jigc(&["task", "validate", ""]),
        "`jigc task validate \"\"`",
    );
    assert_refused(
        &corpus.jigc(&["doc", "list", "--task", ""]),
        "`jigc doc list --task \"\"`",
    );
}

// ───────────────────────── (d) the phantom working area ────────────────────────────────

/// A write verb with an empty `--task` used to resolve `.jigc/tasks/` itself as the
/// working area and stage into `.jigc/tasks/docs/` — a phantom task `jigc task list` then
/// listed as live. The write refuses, no `docs/` area appears, and the roster is the live
/// set it was.
#[test]
fn a_write_verb_with_an_empty_task_mints_no_phantom_area() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let repo = corpus.repo();
    let live = corpus
        .live_task()
        .expect("refs-post-hoc leaves a task live");

    let prose = repo.join("thesis.txt");
    fs::write(&prose, "A thesis this write must never place.\n").expect("write the payload");

    let out = corpus.jigc(&[
        "doc",
        "set-slot",
        "vision:vision#thesis",
        "--from-file",
        prose.to_str().expect("utf-8 payload path"),
        "--task",
        "",
    ]);
    assert_refused(&out, "`jigc doc set-slot … --task \"\"`");

    assert!(
        !repo.join(".jigc/tasks/docs").exists(),
        "no phantom `.jigc/tasks/docs/` working area may be minted",
    );
    // The roster is asserted by **count**, not only by the phantom's name: the claim is
    // that it is still exactly the live set, which a differently-named phantom would
    // break just as well.
    let roster = corpus.jigc_ok(&["task", "list"]);
    assert!(
        roster.contains("1 active task(s)"),
        "the roster must be exactly the live set; got:\n{roster}",
    );
    assert!(
        roster.contains(live),
        "the live task must still be listed; got:\n{roster}",
    );
}

// ───────────────────────── (e) the milestone doors ─────────────────────────────────────

/// Mint a milestone with one sub-task, and return the corpus.
fn milestone_corpus() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    corpus.jigc_ok(&["milestone", "create", "Probe milestone"]);
    corpus.jigc_ok(&["milestone", "add-task", "probe-milestone", "Do the thing"]);
    assert!(
        corpus
            .repo()
            .join(".jigc/milestones/probe-milestone")
            .is_dir(),
        "the fixture's premise is a present milestone cache area",
    );
    corpus
}

/// Both malformed shapes refuse at a milestone door — the empty id at a read door, the
/// traversal at the **destroying** one — over a corpus that really has a milestone area,
/// and nothing on disk moves.
#[test]
fn the_milestone_doors_refuse_both_malformed_shapes() {
    let corpus = milestone_corpus();
    let repo = corpus.repo();
    let files = tracked(&corpus);

    assert_refused(
        &corpus.jigc(&["milestone", "list-tasks", ""]),
        "`jigc milestone list-tasks \"\"`",
    );
    assert_refused(
        &corpus.jigc(&["milestone", "discard", "../.."]),
        "`jigc milestone discard ../..`",
    );

    assert!(repo.join(".git").is_dir(), "`.git/` must survive");
    assert!(
        repo.join(".jigc/milestones/probe-milestone").is_dir(),
        "the milestone's cache area must survive",
    );
    assert!(
        repo.join(".jigc/tasks/do-the-thing").is_dir(),
        "the sub-task's working area must survive",
    );
    all_present(&repo, &files, "milestone discard ../..");
}

/// **Position, driven.** The guard sits **above `reseed_cache`**, which every operating
/// milestone door reaches before its own `is_dir()` check.
///
/// The fixture makes that observable rather than structural: a committed
/// `milestone-record` is moved to a **malformed** slug (`probe-`, a trailing hyphen — no
/// traversal, purely a grammar violation) and both cache areas are cleared. At `HEAD~`,
/// `jigc milestone execute "probe-"` read that record, **rebuilt** `.jigc/milestones/probe-/`
/// and `.jigc/tasks/do-the-thing/` from it, and composed the workflow at exit 0. With the
/// guard above the re-seed, the door refuses and neither directory comes back.
#[test]
fn the_guard_sits_above_the_milestone_cache_reseed() {
    let corpus = milestone_corpus();
    let repo = corpus.repo();

    // The record under a malformed identity — moved on disk, which is the shape the
    // re-seed reads (it parses the store, not HEAD).
    let records = repo.join("docs/milestone-records");
    fs::rename(
        records.join("probe-milestone.md"),
        records.join("probe-.md"),
    )
    .expect("move the record onto a malformed slug");

    // Clear both areas the re-seed would rebuild, so their reappearance is the signal.
    let cache = repo.join(".jigc/milestones/probe-milestone");
    let area = repo.join(".jigc/tasks/do-the-thing");
    fs::remove_dir_all(&cache).expect("clear the milestone cache area");
    fs::remove_dir_all(&area).expect("clear the sub-task working area");

    let out = corpus.jigc(&["milestone", "execute", "probe-"]);
    assert_refused(&out, "`jigc milestone execute probe-`");

    assert!(
        !repo.join(".jigc/milestones/probe-").exists(),
        "the refused door must materialize no milestone cache area",
    );
    assert!(
        !area.exists(),
        "the refused door must materialize no sub-task working area",
    );
}

// ───────────────────────── (f) the cleanup path still lands ────────────────────────────

/// **N24 disposed by driving.** `post_commit`'s `remove_dir_all(cleanup_dir)` is fed from
/// exactly two production sources — a `TaskArea`'s own `dir` and `milestone_dir(…)` — both
/// of which now sit behind the guard. It is silent by design, so the proof is that both
/// boundaries still land green and both working areas are still removed.
#[test]
fn the_task_boundary_still_lands_and_removes_its_working_area() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let repo = corpus.repo();
    let task = corpus
        .live_task()
        .expect("refs-post-hoc leaves a task live")
        .to_string();
    assert!(
        repo.join(".jigc/tasks").join(&task).is_dir(),
        "the fixture's premise is a live working area",
    );

    corpus.finalize(&task, "vision", "Ground the vision in research", false);

    assert!(
        !repo.join(".jigc/tasks").join(&task).exists(),
        "a landed `jigc task finalize` removes the working area (post_commit's cleanup)",
    );
}

/// The milestone half of the same disposal: a real fan-out boundary — provision, stage
/// code inside the sub-task's worktree, finalize — lands green and removes
/// `.jigc/milestones/<id>/`.
#[test]
fn the_milestone_boundary_still_lands_and_removes_its_area() {
    let corpus = milestone_corpus();
    let repo = corpus.repo();
    corpus.jigc_ok(&["milestone", "provision", "probe-milestone"]);

    let worktree = repo.join(".jigc/worktrees/do-the-thing");
    fs::create_dir_all(worktree.join("src")).expect("mk the worktree src dir");
    fs::write(worktree.join("src/thing.rs"), "pub fn thing() {}\n").expect("write the code");
    corpus.git(&[
        "-C",
        worktree.to_str().expect("utf-8 worktree path"),
        "add",
        "src/thing.rs",
    ]);

    corpus.jigc_ok(&["milestone", "finalize", "probe-milestone"]);

    assert!(
        !repo.join(".jigc/milestones/probe-milestone").exists(),
        "a landed `jigc milestone finalize` removes the milestone area (post_commit's cleanup)",
    );
}
