//! M53 Increment 2 / T1 — **the zero-false-fire controls for `engine::state::unwind_area`,
//! and the one transient the finalize transaction writes into the area it is about to
//! unwind** (`completions/artifacts/M53/settle-record.md` → D2 as amended by §13;
//! `acceptance-design.md` → Spikes owed, increment 2;
//! `implementation/roadmap.md` → M53 Increment 2).
//!
//! **The spike this is.** Increment 2 replaces `post_commit`'s and
//! `cleanup_subtask_areas`' `remove_dir_all` with `engine::state::unwind_area(area, kind)`
//! on the `Displace` arm, so that a byte jigc could not move aside is *kept* rather than
//! taken. Every cell of that decision's acceptance axis presupposes foreign bytes — and its
//! most dangerous regression is the opposite cell: `unwind_area` answering
//! `AreaUnwind::Foreign` over an **ordinary** area. That answer would leave every finalized
//! task's working area standing (listed by `jigc task list` as an active task that
//! finalized) and mint the increment's new advisory on every landed finalize. So §13 names
//! three controls, and they are driven **before** anything is built on them.
//!
//! **The controls, and where each is discharged** — [`CONTROLS`] is the set, stated once and
//! fenced, because one of the three is discharged by a test that already exists in the suite
//! that owns its fixture (M53 Increment 1 / T3) and re-building a real two-sub-task fan-out
//! here to assert the identical predicate would be a second fixture, not a second control.
//!
//! **The fourth arm — the transient — is why §13's wording is not enough on its own.** A
//! control that drives an area jigc's *own doors* never finalize proves nothing about the
//! area a finalize hands to phase 7: `try_execute_finalize_plan` writes
//! `<msg_tmp_dir>/finalize-message.tmp`, and at all three of its call sites `msg_tmp_dir`
//! **is** `cleanup_dir` — the very working area phase 7 tears down. The census below is
//! therefore driven rather than read: a `post-commit` hook copies the area at the latest
//! moment inside the transaction, and the production predicate
//! (`engine::state::foreign_area_paths`) is asked over that copy.
//!
//! **[Corrected 2026-09-22 (M53 completion audit, fix 1).** This arm shipped asserting the
//! census was *exactly* `["finalize-message.tmp"]` — one non-member — with the removal
//! statement offered as the reason that was safe. **Falsifying datum, driven on the debug
//! binary at `45427083`:** that removal is `let _ = std::fs::remove_file(…)`, and a
//! `pre-commit` hook that `chmod 0555`s the area makes it fail. `jigc task finalize` then
//! landed at exit 0 printing `finalize.foreign-bytes` over
//! `.jigc/tasks/<id>/finalize-message.tmp` — jigc's own transient, named as *a path jigc did
//! not write* — and `jigc task discard <id>` refused at **exit 1** over it, demanding
//! `--force`. A test that pins the defect as expected output is why the sweep stopped there,
//! so the census now asserts the complement is **empty**: every path the transaction writes
//! into the cleanup dir is a `WorkArea::jigc_written` member
//! (`engine::state::FINALIZE_MESSAGE_FILE` joined both rows), which is the only form of the
//! claim that does not depend on a best-effort statement running.**]
//!
//! **Declared bound on that arm.** The snapshot is taken inside `git commit`, so it sees
//! every write phases 1–6 made into the area; what happens *after* the commit returns is the
//! fault cell's subject and is driven in `flow54_acceptance`.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::support::trial_corpus::{State, TrialCorpus};

/// **§13's three zero-false-fire controls**, each with the test that discharges it: the
/// control as §13 words it, the suite file, and the test function.
///
/// The set is stated here because it is a set — three cells of one claim — and two of its
/// members live in this file while the third lives where its fixture does. A citation
/// nothing checks goes stale silently, which is why [`every_control_is_discharged_by_a_live_test`]
/// reads each file rather than trusting the row.
const CONTROLS: &[(&str, &str, &str)] = &[
    (
        "an ordinary task lifecycle (`start` → `doc create` → `doc rename --task` → \
         `task validate`)",
        "crates/cli/tests/finalize_area_unwind_controls.rs",
        "an_ordinary_task_lifecycle_area_unwinds_to_removed",
    ),
    (
        "a migrate task carrying `source` + `source-path`",
        "crates/cli/tests/finalize_area_unwind_controls.rs",
        "a_migrate_task_area_unwinds_to_removed",
    ),
    (
        "an ordinary post-join milestone area including `merged/` — discharged where its \
         fixture lives (M53 Increment 1 / T3), a real two-sub-task fan-out whose \
         `materialize` wrote the merged bodies",
        "crates/cli/tests/milestone_merged_complement.rs",
        "an_ordinary_post_join_area_unwinds_to_removed",
    ),
];

/// The foreign Keep-a-Changelog file the migrate control adopts.
const FOREIGN_CHANGELOG: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The working area of `task` in `corpus`.
fn task_area(corpus: &TrialCorpus, task: &str) -> PathBuf {
    corpus.repo().join(".jigc").join("tasks").join(task)
}

/// Every entry name directly under `area`, sorted — the failure message's evidence, so a
/// red assertion says what the area actually held rather than only what it wanted.
fn entry_names(area: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(area)
        .unwrap_or_else(|err| panic!("read {}: {err}", area.display()))
        .map(|entry| {
            entry
                .expect("a working-area entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

/// Assert that `area` unwinds whole and is gone — the one predicate all three controls make.
fn assert_unwinds_to_removed(area: &Path, control: &str) {
    let held = entry_names(area);
    assert_eq!(
        engine::state::unwind_area(area, engine::state::WorkArea::Task)
            .expect("the unwind reports"),
        engine::state::AreaUnwind::Removed,
        "{control}: an area holding nothing but jigc's own writes must unwind whole — a \
         `Foreign` here leaves every finalized task's area standing and mints an advisory on \
         every landed finalize; the area held {held:?}",
    );
    assert!(
        !area.exists(),
        "{control}: …and the area itself is gone, so no door lists a task that finalized",
    );
}

// ---------------------------------------------------------------------------
// The control set
// ---------------------------------------------------------------------------

/// **The set is enumerable and every row names a test that exists.** A control discharged
/// nowhere is a control nobody runs, and the third row points out of this file.
#[test]
fn every_control_is_discharged_by_a_live_test() {
    for (control, file, function) in CONTROLS {
        let path = workspace_root().join(file);
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read the suite that discharges `{control}`: {err}"));
        assert!(
            body.contains(&format!("fn {function}(")),
            "§13's control — {control} — is discharged by `{function}` in `{file}`, and that \
             function is not there: a renamed or deleted control is a control nobody runs",
        );
    }
}

/// **(a) An ordinary task lifecycle unwinds to `Removed`.** The lifecycle is the one the
/// writer registry's own control drives — it is what reaches `roles.json` (the create gate's
/// binding) and `renames.json` (the member a hand-drawn list missed) — asked here of the
/// *removal* half rather than of the probe.
#[test]
fn an_ordinary_task_lifecycle_area_unwinds_to_removed() {
    let corpus = TrialCorpus::build(State::Fresh);
    // `record-decision` is the workflow whose `allows-create:` grants the ADR; `quick-fix`
    // grants none, so it could not reach `roles.json` at all.
    let task = corpus.start_workflow("record-decision", "record a decision about caching");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Cache the thing",
        "--task",
        &task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "adr:cache-the-thing",
        "--to",
        "Cache the other thing",
        "--task",
        &task,
    ]);
    // Report-only: `task validate` over an unfilled ADR exits non-zero on its own content
    // findings, and its verdict is not this control's subject — the snapshots it lands in
    // the area are.
    let _ = corpus.jigc(&["task", "validate", &task]);

    let area = task_area(&corpus, &task);
    let held = entry_names(&area);
    for member in ["roles.json", "renames.json", "docs"] {
        assert!(
            held.contains(&member.to_string()),
            "the lifecycle must really reach `{member}`, else this control proves nothing; \
             the area holds {held:?}",
        );
    }

    assert_unwinds_to_removed(&area, "an ordinary task lifecycle");
}

/// **(b) A migrate task's area unwinds to `Removed`.** Its own two members —
/// `engine::state::SOURCE_FILE` and `SOURCE_PATH_FILE`, the staged foreign bytes and the
/// path they came from — are written by no other door, so an area carrying them is a shape
/// the lifecycle control above never produces.
#[test]
fn a_migrate_task_area_unwinds_to_removed() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::write(corpus.repo().join("HISTORY.md"), FOREIGN_CHANGELOG).expect("write HISTORY.md");
    // Tracked, because M51 Increment 1 made an untracked in-repo migrate source a refusal.
    corpus.git(&["add", "HISTORY.md"]);
    corpus.git(&["commit", "-q", "-m", "add HISTORY.md"]);

    corpus.jigc_ok(&["migrate", "HISTORY.md", "--as", "changelog"]);
    let tasks = corpus.repo().join(".jigc").join("tasks");
    let minted = entry_names(&tasks);
    assert_eq!(
        minted.len(),
        1,
        "the migrate mint is this corpus's only task; `.jigc/tasks/` holds {minted:?}",
    );
    let area = tasks.join(&minted[0]);

    let held = entry_names(&area);
    for member in [engine::state::SOURCE_FILE, engine::state::SOURCE_PATH_FILE] {
        assert!(
            held.contains(&member.to_string()),
            "a migration area must really carry `{member}`, else this control is the \
             lifecycle control again; the area holds {held:?}",
        );
    }

    assert_unwinds_to_removed(&area, "a migrate task");
}

// ---------------------------------------------------------------------------
// The transient the finalize transaction writes into the area it unwinds
// ---------------------------------------------------------------------------

/// **The whole census, driven: the transaction writes NO non-member into the area it is about
/// to unwind.**
///
/// The enumeration is taken by a `post-commit` hook — the latest moment inside the
/// transaction — and classified by the production predicate, so the set is the area's own
/// rather than a source reading of it. Then the landed run answers the second half: nothing
/// was displaced and nothing was narrated.
///
/// The empty census is the assertion, and it is deliberately stronger than the one this test
/// shipped with (see the module note): a census of *one non-member, removed a statement
/// later* is only safe while that statement runs, and it is best-effort.
#[test]
fn the_finalize_transaction_writes_no_non_member_into_the_area_it_unwinds() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
    let area = task_area(&corpus, &task);
    let snapshot = corpus.home().join("area-inside-the-transaction");

    // The hooks directory git will actually run, asked of git rather than assumed — the
    // install's own `pre-commit` lives there too and is left exactly as it is.
    let hooks = corpus
        .repo()
        .join(corpus.git(&["rev-parse", "--git-path", "hooks"]));
    fs::create_dir_all(&hooks).expect("the hooks dir");
    let hook = hooks.join("post-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\ncp -R '{}' '{}'\n",
            area.display(),
            snapshot.display(),
        ),
    )
    .expect("write the post-commit snapshot hook");
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("make the hook runnable");

    // A real diff for the commit to carry: a commit-only task with nothing staged blocks at
    // `finalize.empty-commit`, and a blocked finalize never reaches the commit at all.
    fs::write(
        corpus.repo().join("README.md"),
        "trial corpus\nretry cap = 3\n",
    )
    .expect("edit the tracked file");
    corpus.git(&["add", "README.md"]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "fix",
        "--task",
        &task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#scope"),
        "--value",
        "cli",
        "--task",
        &task,
    ]);
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "cap the retries");

    let out = corpus.jigc(&["task", "finalize", &task]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "the finalize must LAND — the transient only exists on the arm that commits;\n\
         --- stdout ---\n{}\n--- stderr ---\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );

    // (1) the enumeration really happened, over a populated area.
    assert!(
        snapshot.is_dir(),
        "the `post-commit` hook must have run — without its copy this arm enumerates \
         nothing and proves nothing",
    );
    assert!(
        snapshot.join("base.json").is_file(),
        "…over the real area: the copy holds {:?}",
        entry_names(&snapshot),
    );

    // (1b) …and over an area that really holds the transient, so the census below is a
    //      statement about a populated set rather than a vacuous one.
    assert!(
        snapshot
            .join(engine::state::FINALIZE_MESSAGE_FILE)
            .is_file(),
        "the snapshot must catch the commit-message transient IN the area — without it this \
         census proves nothing about the name that motivated it; the copy holds {:?}",
        entry_names(&snapshot),
    );

    // (2) the census: EVERY path production writes into the cleanup dir is a registry member.
    assert_eq!(
        engine::state::foreign_area_paths(&snapshot, engine::state::WorkArea::Task)
            .expect("the copied area enumerates"),
        Vec::<PathBuf>::new(),
        "every path the transaction writes into the cleanup dir must be a \
         `WorkArea::jigc_written` member — a non-member here is reported as a path jigc did \
         not write and refuses every later destroying door over jigc's own file; the area \
         held {:?} at the last moment inside the transaction",
        entry_names(&snapshot),
    );

    // (3) …and the transient is gone before phase 7 reads the area on the ordinary arm too.
    //     Were it still there, the shipped displacement would have moved it aside and said so.
    assert!(
        !corpus.repo().join(".jigc").join("displaced").exists(),
        "an ordinary landed finalize displaces nothing — `.jigc/displaced/` exists, so \
         phase 7 found a non-member in the area; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("moved aside"),
        "…and narrates nothing; stderr:\n{stderr}",
    );
    assert!(
        !area.exists(),
        "…and the teardown still took jigc's own area"
    );
}
