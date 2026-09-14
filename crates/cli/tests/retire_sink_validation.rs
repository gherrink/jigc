//! M51 Increment 1 / T3 — **the retire sink re-validates in the same function as the unlink**
//! (`completions/artifacts/M51/settle-record.md` → §2, the sink; `design/finalize.md` →
//! Rollback discipline; `design/auto-migration.md` → Retire-the-foreign-original).
//!
//! T1 made the `jigc migrate` **door** adjudicate its `<path>` argument before anything mints.
//! That closes the door and nothing else: the value the door records is a plain file in a
//! **mutable working area** (`.jigc/tasks/<id>/source-path`), and one commit closure later
//! `jigc task finalize --approve` reads it back and hands it to `std::fs::remove_file` without
//! asking a second time. A door-only guard is therefore a guard on the *typing*, not on the
//! *deletion* — anything that can write a file under `.jigc/` between the two (a hand edit, a
//! sloppy script, a step that rewrites task state) substitutes the deletion target at exit 0.
//!
//! So the sink asks again, **immediately before the unlink and in the same function as it**,
//! and the three arms below are the three shapes that substitution takes. Each drives a
//! legitimate, authored, in-repo migration to the edge of `--approve`, rewrites the recorded
//! `source-path`, and then asserts the whole transaction refuses:
//!
//!   * (a) an **absolute path outside the repository** — the cell that makes this a data-loss
//!     class rather than a hygiene one: the planted canary lives on somebody else's
//!     filesystem, and `repo_root.join(<absolute>)` *is* that absolute path;
//!   * (b) a **leading-`:` pathspec-magic** spelling — `git add -- <path>` prevents option
//!     parsing, never magic, so a recorded `:(top)…` reaches `stage_migration` and stages a
//!     set of files nobody named. This is a **sink cell only**: at the door the same token is
//!     already refused, for the unrelated reason that it names no readable file;
//!   * (c) a **`.git/` component** — git records nothing there, so the deletion could only ever
//!     be one no index has a copy of.
//!
//! and (d) is the control that keeps the other three from being satisfied by a sink that
//! refuses everything: the identical spine, **untampered**, still lands.
//!
//! Every arm asserts the same five things, because the refusal fires *inside* the commit
//! closure and is therefore a refusal of the whole transaction, not of one step: exit **1**
//! with `finalize.retire-untrackable` and exactly one route (the `settle-record.md` → §10
//! mold), the canary byte-identical, `HEAD` unmoved, the promoted doc **gone** from the
//! worktree, and the **whole index** back where it was — the captured-pre-image rollback the
//! wave is extending, reached through a new refusal point. That last assertion is
//! deliberately unscoped: the property is *the index is byte-identical*, and a per-path scope
//! cannot see a rollback axis that drops paths nobody named.

use std::fs;
use std::path::PathBuf;

use crate::support::trial_corpus::{State, TrialCorpus};

/// The sink's own code (`settle-record.md` → §10's table). It is not the door's
/// `migrate.source-untrackable`: the door's subject is an argument the operator typed and can
/// retype, and this one's is task state that was written by a door which had already
/// adjudicated it — a different act, so a different identity.
const CODE: &str = "finalize.retire-untrackable";

/// The canonical home the `vision` doctype's `placement` declares — the file the promote
/// writes and the rollback must take back.
const PROMOTED: &str = "VISION.md";

/// The foreign source the spine migrates: an ordinary, in-repo, committed file, so nothing
/// about these arms rests on the source being unusual.
const FOREIGN_VISION: &str = "\
# Product Direction

We build a deterministic context compiler.

## Principles

Structure belongs to the CLI; prose belongs to the model.
";

/// The whole-doc payload that rewrites [`FOREIGN_VISION`] into the managed `vision`.
const VISION_PAYLOAD: &str = "\
title: Vision
sections:
  - id: thesis
    set:
      thesis: |-
        <<We build a deterministic context compiler.>>
  - id: invariants
    set:
      invariants: |-
        <<Structure belongs to the CLI; prose belongs to the model.>>
  - id: open-questions
    set:
      open-questions: |-
        <<Which domains earn a pack of their own.>>
";

/// A throwaway directory **outside** any repository — the home of the canary arm (a) proves
/// untouched. Minted with `mktemp -d`, so there is nothing to remove and no variable-path
/// delete anywhere in this suite.
fn outside_dir(label: &str) -> PathBuf {
    let template = std::env::temp_dir().join(format!(
        "jigc-retire-sink-{label}-{}-{:?}-XXXXXX",
        std::process::id(),
        engine::tempname::unique_nanos(),
    ));
    let out = std::process::Command::new("mktemp")
        .arg("-d")
        .arg(template.as_os_str())
        .output()
        .expect("run mktemp -d");
    assert!(
        out.status.success(),
        "mktemp -d failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    PathBuf::from(String::from_utf8_lossy(&out.stdout).trim())
}

/// A migration driven to the edge of `--approve`: the task exists, the managed `vision` is
/// authored and staged, the commit doc is conformant, and nothing has been promoted or
/// retired yet.
struct Staged {
    task: String,
    /// The working-area file holding the value the sink reads back — the thing these arms
    /// rewrite.
    source_path: PathBuf,
    /// `HEAD` before `finalize` ran, so an arm can assert the transaction moved nothing.
    head: String,
    /// The **whole** pre-finalize index (`git ls-files --stage`) and working-tree status
    /// (unscoped `git status --porcelain`). The axis a rolled-back transaction has to
    /// restore is *the index*, not the promote destination: a path-scoped assertion is
    /// satisfied by a rollback that leaves four other tracked files staged for deletion, so
    /// these two snapshots are compared whole.
    index: String,
    status: String,
}

/// Drive one committed in-repo `docs/direction.md` through migrate + author + commit-doc
/// authoring, stopping short of `finalize`.
fn staged_migration(corpus: &TrialCorpus) -> Staged {
    let repo = corpus.repo();
    fs::create_dir_all(repo.join("docs")).expect("create docs/");
    fs::write(repo.join("docs/direction.md"), FOREIGN_VISION).expect("write the foreign source");
    corpus.git(&["add", "--", "docs/direction.md"]);
    corpus.git(&["commit", "-q", "-m", "add the direction doc"]);

    let composed = corpus.jigc_ok(&["migrate", "docs/direction.md", "--as", "vision"]);
    let task = composed
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("the migrate mints a task; got:\n{composed}"))
        .trim()
        .to_owned();
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "vision",
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        VISION_PAYLOAD,
    );
    // The commit doc, authored exactly as `TrialCorpus::finalize` does — but not finalized,
    // because every arm below drives that step itself and three of them expect a refusal.
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "docs",
        "--task",
        &task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#scope"),
        "--value",
        "vision",
        "--task",
        &task,
    ]);
    corpus.set_slot(
        &format!("commit:{task}#summary"),
        &task,
        "migrate the direction doc",
    );
    corpus.set_slot(
        &format!("commit:{task}#body"),
        &task,
        "Built by the retire-sink fixture.",
    );

    let source_path = repo
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("source-path");
    assert!(
        source_path.is_file(),
        "the migrate door records the source path the sink reads back",
    );
    let head = corpus.git(&["rev-parse", "HEAD"]);
    let index = corpus.git(&["ls-files", "--stage"]);
    let status = corpus.git(&["status", "--porcelain"]);
    assert!(
        status.is_empty(),
        "the fixture stops with a clean tree, so any porcelain output after a refusal is \
         the refusal's own doing; got:\n{status}",
    );
    Staged {
        task,
        source_path,
        head,
        index,
        status,
    }
}

/// Assert the whole transaction refused: the §10 mold on the printed surface, plus the four
/// state facts that make the refusal a *refusal* rather than a message.
fn assert_transaction_refused(
    corpus: &TrialCorpus,
    staged: &Staged,
    out: &std::process::Output,
    what: &str,
) {
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(
        out.status.code(),
        Some(1),
        "{what} must be refused at exit 1 (the destroying-door mold); got {}\
         \n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
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
        "{what} must carry exactly one route line (the route floor); got:\n{stderr}",
    );
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        staged.head,
        "{what} must leave HEAD unmoved — the refusal is of the whole transaction",
    );
    assert!(
        !corpus.repo().join(PROMOTED).exists(),
        "{what} must roll the promote back out of the worktree",
    );
    // The axis is the index, not one path (M51 Inc 1 validation): the captured-pre-image
    // discipline's claim is that a refused finalize leaves the index byte-identical to its
    // pre-finalize state, so the assertion is over the WHOLE index and the WHOLE porcelain —
    // a `-- VISION.md` scope is satisfied by a rollback that stages four `.jigc/` files for
    // deletion on its way past.
    assert_eq!(
        corpus.git(&["ls-files", "--stage"]),
        staged.index,
        "{what} must leave the whole index byte-identical to its pre-finalize state",
    );
    assert_eq!(
        corpus.git(&["status", "--porcelain"]),
        staged.status,
        "{what} must leave the working tree and index exactly as it found them",
    );
    assert!(
        corpus.repo().join("docs/direction.md").is_file(),
        "{what} must retire nothing at all — the real source is still on disk",
    );
}

// ───────────────────── (a) an absolute path outside the repository ─────────────────────

/// **The centrepiece.** A recorded `source-path` rewritten to an absolute path outside the
/// repository is refused at the sink, and the planted canary is byte-identical afterwards.
///
/// Without the sink check `retire` computes `repo_root.join(<absolute>)`, which in Rust *is*
/// the absolute path — so the unlink lands on a file in another tree, at exit 0, inside a
/// commit that says it migrated a document.
#[test]
fn an_absolute_recorded_source_is_refused_at_the_sink_and_the_canary_survives() {
    let corpus = TrialCorpus::build(State::Fresh);
    let staged = staged_migration(&corpus);

    let outside = outside_dir("absolute");
    let canary = outside.join("keepme.md");
    fs::write(&canary, FOREIGN_VISION).expect("plant the canary");
    let before = fs::read(&canary).expect("read the canary");
    fs::write(
        &staged.source_path,
        canary.to_str().expect("utf-8 canary path"),
    )
    .expect("rewrite the recorded source path");

    let out = corpus.jigc(&["task", "finalize", &staged.task, "--approve"]);
    assert_transaction_refused(
        &corpus,
        &staged,
        &out,
        "finalize --approve over a recorded absolute host path",
    );
    assert_eq!(
        fs::read(&canary).expect("read the canary back"),
        before,
        "the file outside the repository must be byte-identical after the refusal",
    );
}

// ───────────────────── (b) leading-`:` pathspec magic ─────────────────────

/// A recorded path beginning with `:` is refused. `git add -- <path>` prevents *option*
/// parsing and nothing else, so `:/keepme.md` reaching `stage_migration` is a pathspec, not a
/// file name — `:/` is git's own *from the top of the tree* magic, and it stages whatever it
/// matches from wherever that puts it.
///
/// **The spelling is `:/…` rather than `:(top)…`, and the reason is stated rather than
/// convenient.** Both are leading-`:` magic and the sink's rule refuses the two identically —
/// `cli::task::tests::every_leading_colon_spelling_is_refused_at_the_sink` drives the
/// parenthesised forms directly at the predicate. What `:(top)…` additionally trips, **before
/// this sink is reached**, is an unrelated pre-existing defect this task neither introduces nor
/// owns: `engine::file_state::ConflictBlock::task` builds `jigc unmanage <source>` as a
/// `Route::mechanical` **eagerly**, on every migration finalize, so any recorded source
/// carrying a shell-special byte fails the route fence's `shell_safe` leg — driven at this
/// task on a perfectly ordinary committed source named `docs/old (draft).md`, which panics
/// `finalize --approve` at exit 101 in a debug build and emits an unrunnable route in a
/// release one. Carried with a trigger (`implementation/decisions-pending.md`) rather than
/// fixed here, because its subject is a route's quoting and not the retire sink.
#[test]
fn a_recorded_pathspec_magic_source_is_refused_at_the_sink() {
    let corpus = TrialCorpus::build(State::Fresh);
    let keepme = corpus.repo().join("keepme.md");
    fs::write(&keepme, "keep me\n").expect("plant the in-repo canary");
    corpus.git(&["add", "--", "keepme.md"]);
    corpus.git(&["commit", "-q", "-m", "add keepme"]);
    let before = fs::read(&keepme).expect("read the in-repo canary");

    let staged = staged_migration(&corpus);
    fs::write(&staged.source_path, ":/keepme.md").expect("rewrite the recorded source path");

    let out = corpus.jigc(&["task", "finalize", &staged.task, "--approve"]);
    assert_transaction_refused(
        &corpus,
        &staged,
        &out,
        "finalize --approve over a recorded `:/…` pathspec",
    );
    assert_eq!(
        fs::read(&keepme).expect("read the in-repo canary back"),
        before,
        "the file the pathspec would have matched must be byte-identical",
    );
}

// ───────────────────── (c) a `.git/` component ─────────────────────

/// A recorded path with a `.git` component is refused. Git records nothing under its own
/// directory, so a retirement there is a deletion with no copy in any commit — and the unlink
/// itself lands on the repository's own machinery.
#[test]
fn a_recorded_source_inside_gits_own_directory_is_refused_at_the_sink() {
    let corpus = TrialCorpus::build(State::Fresh);
    let canary = corpus.repo().join(".git").join("jigc-canary.md");
    fs::write(&canary, "keep me\n").expect("plant the canary inside .git/");
    let before = fs::read(&canary).expect("read the canary");

    let staged = staged_migration(&corpus);
    fs::write(&staged.source_path, ".git/jigc-canary.md")
        .expect("rewrite the recorded source path");

    let out = corpus.jigc(&["task", "finalize", &staged.task, "--approve"]);
    assert_transaction_refused(
        &corpus,
        &staged,
        &out,
        "finalize --approve over a recorded `.git/` source",
    );
    assert_eq!(
        fs::read(&canary).expect("read the canary back"),
        before,
        "the file inside git's own directory must be byte-identical after the refusal",
    );
}

// ───────────────────── (d) the control ─────────────────────

/// The identical spine, **untampered**, still lands: the managed doc is committed at its
/// placement home, the foreign original is retired, and the deletion rides the same commit.
///
/// Without this arm every assertion above is satisfied by a sink that refuses everything,
/// which is the failure mode a re-validating sink is most likely to ship with.
#[test]
fn an_untampered_migration_still_finalizes_and_retires() {
    let corpus = TrialCorpus::build(State::Fresh);
    let staged = staged_migration(&corpus);

    let out = corpus.jigc(&["task", "finalize", &staged.task, "--approve"]);
    assert!(
        out.status.success(),
        "an adjudicated, in-repo migration must still land; \
         \n--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ne!(
        corpus.git(&["rev-parse", "HEAD"]),
        staged.head,
        "the approved migration lands a commit",
    );
    assert!(
        corpus.repo().join(PROMOTED).is_file(),
        "the managed doc lands at its placement home",
    );
    assert!(
        !corpus.repo().join("docs/direction.md").exists(),
        "the foreign original is retired from the worktree",
    );
    let landed = corpus.git(&["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        landed
            .lines()
            .any(|line| line.starts_with('D') && line.contains("docs/direction.md")),
        "the retirement rides the same commit as the promoted doc; got:\n{landed}",
    );
}
