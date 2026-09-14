//! M51 Increment 1 / T1 + T2 — **the `jigc migrate` door adjudicates its source path before
//! anything mints** (`completions/artifacts/M51/settle-record.md` → D1 parts 1+3, as amended
//! by §2 · the §10 mold; `design/auto-migration.md` → The `jigc migrate` verb).
//!
//! The door took its `<path>` argument as an opaque token: it joined it onto the repository
//! root, read whatever came back, minted an off-router task and recorded the spelling as the
//! task's `source-path` — the value `finalize --approve` later **deletes**. Driven at
//! `abd81df`, every one of these landed at **exit 0**:
//!
//! ```text
//! jigc migrate /private/tmp/victim.XXXX/keepme.md --as changelog   # outside the repository
//! jigc migrate .git/config --as changelog                          # git's own directory
//! jigc migrate ../outside.md --as changelog                        # recorded as `outside.md`
//! jigc migrate link.md --as changelog                              # a symlink out of the tree
//! jigc migrate .jigc/scratch/n.md --as changelog                   # jigc's own workbench
//! ```
//!
//! The `../` cell is the sharpest of the five: `repo_relative_source_path` folded the `..`
//! **lexically**, so a source one directory above the repository was recorded as the
//! repo-relative `outside.md` — a spelling the retire sink would resolve back **inside** the
//! tree, at a file that is not the one the operator named.
//!
//! **Three predicates, not two** (§2, driven). `crate::trackable::untrackable_reason` opens
//! `let relative = relative.trim_matches('/')` and its parameter is typed *repo-root-relative*
//! — `design/storage.md` says so in its own words — so an absolute host path is re-read as
//! `<repo>/private/tmp/…` and answers **trackable**. The door therefore **resolves the caller
//! token to a repo-relative path or refuses an absolute one outright**, and only then asks
//! the shipped predicates: `untrackable_reason` · `is_workbench_root` ·
//! `unusable_root_reason`'s symlink leg.
//!
//! **A fourth question, and it is not about location** (§2, the trackedness leg — T2). All
//! three predicates above ask *where* the source is; none asks whether git has ever recorded
//! it. Driven end to end at `706f5f3f`: an **untracked** in-repo `HISTORY.md` migrated,
//! authored and `--approve`d landed the canonical doc, deleted the source from the worktree,
//! and left `git log --all -- HISTORY.md` **empty** — the bytes in no git object, the deletion
//! named on no surface. That falsified the recorded warrant under which the adapter deny floor
//! keeps its blanket permit for `migrate` + `finalize --approve` (*"a guarded migrate destroys
//! only a reviewed, in-repo, **git-recoverable** file"*). Arm (g) is that leg, and it drives
//! the route rather than reading it: the printed `git add` is extracted from the refusal and
//! run **verbatim**, and the identical `jigc migrate` then succeeds.
//!
//! Its admitting half is **two** arms, not one, because *git-recoverable* is a union: a source
//! in the **index** (staged, never committed — which is what makes the printed route
//! sufficient) and a source in **`HEAD`** but deliberately out of the index (the user's own
//! pre-staged `git rm --cached`, M40 F7) are both files git holds a copy of. The leg refuses
//! only a file git holds **no** copy of.
//!
//! Every arm drives the real binary in a [`support::trial_corpus`] fixture. The refusal shape
//! is the §10 mold, checked rather than assumed: one code, **exactly one** `route:` line, exit
//! **1** — and **no task directory**, because a door that strands a task dir has already
//! written the state the refusal claims it did not.
//!
//! The admitting half is arm (f), and it is the reason the resolve step exists rather than a
//! flat "refuse anything absolute": an absolute **in-repo** spelling stays accepted and still
//! records the canonical `docs/changelog/changelog.md`, which is the property
//! `migrate_retire_safety::migrate_records_a_canonical_source_path_for_redundant_spellings`
//! pins from the other side.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::support::trial_corpus::{State, TrialCorpus};

/// The one code every **location** leg of this door carries (`settle-record.md` → §10). One
/// code with the reason in the message, on `config.untrackable-root`'s five-reasons-one-code
/// precedent: the operator's fix is the same in all five cases — name a different source.
const CODE: &str = "migrate.source-untrackable";

/// The **trackedness** leg's own code (`settle-record.md` → §2, the trackedness leg; §10's
/// table row). It is a second code rather than a fifth reason under [`CODE`] because the
/// operator's fix is *not* the same: every location leg says *name a different source*, and
/// this one says *keep this source and stage it* — a different act, so a different identity.
const UNTRACKED: &str = "migrate.source-untracked";

/// A foreign changelog body, comfortably above the byte floor so no arm can be refused for
/// being trivial instead of for being where it is.
const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release, migrated from the foreign file.
";

/// A throwaway directory **outside** any repository — the home of the planted canary the
/// absolute-outside arm proves untouched. Minted with `mktemp -d`, so there is nothing to
/// remove and no variable-path delete anywhere in this suite.
fn outside_dir(label: &str) -> PathBuf {
    let template = std::env::temp_dir().join(format!(
        "jigc-migrate-source-{label}-{}-{:?}-XXXXXX",
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

/// Every task id the corpus holds — the set a refusal must leave **empty**.
fn task_dirs(repo: &Path) -> Vec<String> {
    let tasks = repo.join(".jigc").join("tasks");
    let Ok(entries) = fs::read_dir(&tasks) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

/// Assert `out` is the §10 refusal: **exit 1**, the door's code, **exactly one** route line —
/// and that the run minted nothing.
///
/// The exit code is the mold's, asserted as the number rather than as *non-zero*: a refusal
/// that came out at 3 would be a *validation* verdict (`design/validation.md` → Exit
/// semantics), and this is an operational refusal at the door, before any document exists to
/// have a verdict about.
fn assert_refused(corpus: &TrialCorpus, out: &std::process::Output, what: &str) {
    assert_refused_with(corpus, out, CODE, what);
}

/// [`assert_refused`] over an explicit `code` — the same mold, asked of whichever leg
/// answered. The location legs share [`CODE`]; the trackedness leg carries [`UNTRACKED`].
fn assert_refused_with(
    corpus: &TrialCorpus,
    out: &std::process::Output,
    code: &str,
    what: &str,
) -> String {
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(
        out.status.code(),
        Some(1),
        "{what} must be refused at exit 1 (the destroying-door mold); \
         got {}\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
        out.status,
    );
    assert!(
        stderr.contains(code),
        "{what} must name `{code}`; got:\n{stderr}",
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
        task_dirs(&corpus.repo()),
        Vec::<String>::new(),
        "{what} must strand no task directory — the refusal is asked BEFORE the mint",
    );
    stderr
}

// ───────────────────── (a) absolute, outside the repository ─────────────────────

/// **The centrepiece.** An absolute path to a file outside the repository is refused at the
/// door, and the planted canary is byte-identical afterwards.
///
/// At `abd81df` this exited 0, recorded the **absolute host path** as the task's
/// `source-path`, and handed `finalize --approve` a deletion target on somebody else's
/// filesystem. The canary assertion is the whole point of the arm: the refusal is not merely
/// a message, it is the reason the bytes are still there.
#[test]
fn an_absolute_source_outside_the_repository_is_refused_and_the_canary_survives() {
    let corpus = TrialCorpus::build(State::Fresh);
    let outside = outside_dir("absolute");
    let canary = outside.join("keepme.md");
    fs::write(&canary, FOREIGN).expect("plant the canary");
    let before = fs::read(&canary).expect("read the canary");

    let out = corpus.jigc(&[
        "migrate",
        canary.to_str().expect("utf-8 canary path"),
        "--as",
        "changelog",
    ]);
    assert_refused(&corpus, &out, "migrate <absolute path outside the repo>");

    assert_eq!(
        fs::read(&canary).expect("read the canary back"),
        before,
        "the file outside the repository must be byte-identical after the refusal",
    );
}

// ───────────────────── (b) inside git's own directory ─────────────────────

/// `.git/config` is not a migration source. Git refuses to record any path with a `.git`
/// component, so a migration whose source lives there could only ever end in a deletion no
/// index has a copy of — and the driven harm at `abd81df` was the file's **contents** reaching
/// the composed step text.
#[test]
fn a_source_inside_gits_own_directory_is_refused_and_the_file_survives() {
    let corpus = TrialCorpus::build(State::Fresh);
    let config = corpus.repo().join(".git").join("config");
    let before = fs::read(&config).expect("the fixture repo has a git config");

    let out = corpus.jigc(&["migrate", ".git/config", "--as", "changelog"]);
    assert_refused(&corpus, &out, "migrate .git/config");

    assert_eq!(
        fs::read(&config).expect("read the git config back"),
        before,
        "git's own config must be byte-identical after the refusal",
    );
}

// ───────────────────── (c) `../` out of the tree ─────────────────────

/// A `../` source that lands above the repository root is refused — and the arm asserts the
/// **ack and the absent task dir**, deliberately not a canary.
///
/// The canary belongs to arm (a). What is specific here is that the pre-fix door did not merely
/// accept this source: `repo_relative_source_path` folded the `..` lexically and recorded
/// `outside.md`, a repo-relative spelling naming a file that is not the operator's. A canary at
/// the real path would have survived at `abd81df` too, so it would prove nothing about this
/// cell; the recorded identity is what was wrong, and a refusal that mints nothing is the only
/// state in which no identity is recorded at all.
#[test]
fn a_dotdot_source_above_the_repository_is_refused_and_mints_nothing() {
    let corpus = TrialCorpus::build(State::Fresh);
    // The corpus root holds `repo/` and `home/`, so `../outside.md` is a real readable file
    // one level above the repository — the cell is the *location*, never an unreadable path.
    let above = corpus
        .repo()
        .parent()
        .expect("the corpus repo has a parent")
        .join("outside.md");
    fs::write(&above, FOREIGN).expect("write the file above the repo");

    let out = corpus.jigc(&["migrate", "../outside.md", "--as", "changelog"]);
    assert_refused(&corpus, &out, "migrate ../outside.md");
}

// ───────────────────── (d) an in-repo symlink out of the tree ─────────────────────

/// A symlink **inside** the repository is refused — at **both** of its targets, because the
/// two are refused by different legs and only one of them is the leg this task added.
///
/// A link out of the tree is caught by `untrackable_reason`: the predicate canonicalizes the
/// path it resolves, so the target lands outside the root. A link at an **in-repo** file is
/// not — it resolves to a perfectly trackable path — and it is the worse cell of the two: the
/// recorded source would be the *link*, so `finalize --approve` would stage the removal of a
/// path the worktree no longer has while the bytes the operator meant to migrate sit untouched
/// beside it. That cell is what `unusable_root_reason`'s symlink leg answers, asked here of
/// every existing component of the resolved value.
#[test]
fn an_in_repo_symlink_is_refused_whichever_side_of_the_root_it_points_at() {
    // (i) out of the tree.
    let corpus = TrialCorpus::build(State::Fresh);
    let outside = outside_dir("symlink");
    let target = outside.join("elsewhere.md");
    fs::write(&target, FOREIGN).expect("write the outside target");
    std::os::unix::fs::symlink(&target, corpus.repo().join("link.md")).expect("plant the symlink");

    let out = corpus.jigc(&["migrate", "link.md", "--as", "changelog"]);
    assert_refused(&corpus, &out, "migrate <in-repo symlink to outside>");
    assert!(
        target.exists(),
        "the symlink's target outside the repository must survive the refusal",
    );

    // (ii) at a file inside the tree — the symlink leg's own cell.
    let corpus = TrialCorpus::build(State::Fresh);
    let docs = corpus.repo().join("docs");
    fs::create_dir_all(&docs).expect("create the docs dir");
    fs::write(docs.join("real.md"), FOREIGN).expect("write the in-repo target");
    std::os::unix::fs::symlink("docs/real.md", corpus.repo().join("leaflink.md"))
        .expect("plant the in-repo symlink");

    let out = corpus.jigc(&["migrate", "leaflink.md", "--as", "changelog"]);
    assert_refused(
        &corpus,
        &out,
        "migrate <in-repo symlink to an in-repo file>",
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("is a symlink"),
        "the in-repo cell must be refused BY THE SYMLINK LEG, not by a location leg that \
         happens to answer first; got:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        docs.join("real.md").exists(),
        "the file the link points at must survive the refusal",
    );
}

// ───────────────────── (e) inside jigc's own workbench ─────────────────────

/// A source under `.jigc/` is refused: the workbench is the tree `jigc uninstall` removes
/// whole, and a migration that retires a source from there would delete bytes no index has a
/// copy of. It is the third predicate's own question — the workbench is a path git tracks
/// perfectly well, so `untrackable_reason` says nothing about it.
#[test]
fn a_source_inside_the_workbench_is_refused() {
    let corpus = TrialCorpus::build(State::Fresh);
    let scratch = corpus.repo().join(".jigc").join("scratch");
    fs::create_dir_all(&scratch).expect("create the workbench scratch dir");
    let source = scratch.join("notes.md");
    fs::write(&source, FOREIGN).expect("write the workbench source");

    let out = corpus.jigc(&["migrate", ".jigc/scratch/notes.md", "--as", "changelog"]);
    assert_refused(&corpus, &out, "migrate .jigc/scratch/notes.md");

    assert!(
        source.exists(),
        "the workbench file must survive the refusal",
    );
}

// ───────────────────── (f) the admitting half ─────────────────────

/// An absolute **in-repo** spelling is still accepted, and still records the canonical
/// repo-relative `docs/changelog/changelog.md`.
///
/// The absolute path is built from the corpus's **own** repo path — the uncanonicalized
/// spelling a caller actually types on macOS, where `/var` is a symlink to `/private/var` —
/// so the arm pins that the resolve step is canonicalization-safe on both sides. A door that
/// refused every absolute token, or that recorded the host spelling, fails here.
///
/// **The `git add` is T2's, and it is the fixture being made admissible rather than
/// decoration**: the trackedness leg refuses a source git has never recorded, so an arm whose
/// subject is the *spelling* has to stage its source or it stops testing spelling at all. It
/// stages without committing on purpose — the predicate is the **index**, so this arm is also
/// the staged-not-committed cell of that leg.
#[test]
fn an_absolute_in_repo_spelling_is_accepted_and_records_the_canonical_path() {
    let corpus = TrialCorpus::build(State::Fresh);
    let home = corpus.repo().join("docs").join("changelog");
    fs::create_dir_all(&home).expect("create the old changelog home");
    fs::write(home.join("changelog.md"), FOREIGN).expect("write the foreign changelog");
    corpus.git(&["add", "--", "docs/changelog/changelog.md"]);

    let absolute = home.join("changelog.md");
    corpus.jigc_ok(&[
        "migrate",
        absolute.to_str().expect("utf-8 absolute path"),
        "--as",
        "changelog",
    ]);

    let minted = task_dirs(&corpus.repo());
    assert_eq!(
        minted.len(),
        1,
        "an admissible source mints exactly one migration task; got {minted:?}",
    );
    let recorded = fs::read_to_string(
        corpus
            .repo()
            .join(".jigc")
            .join("tasks")
            .join(&minted[0])
            .join("source-path"),
    )
    .expect("read the recorded source-path");
    assert_eq!(
        recorded, "docs/changelog/changelog.md",
        "an absolute in-repo spelling records the canonical repo-relative path",
    );
}

// ───────────────────── (g) the trackedness leg ─────────────────────

/// Extract the `git …` command the route prints, **verbatim**, from the one refusal line that
/// carries one.
///
/// The route is read out of the emitted bytes rather than rebuilt here, because the emitted
/// bytes are the contract: a route the agent cannot run is a route floor breach, and a test
/// that hand-builds the command it then runs can pass over a refusal that printed something
/// else entirely.
fn printed_git_command(stderr: &str) -> String {
    let route = stderr
        .lines()
        .find(|line| line.trim_start().starts_with("route:"))
        .unwrap_or_else(|| panic!("the refusal carries a route line; got:\n{stderr}"));
    let mut parts = route.split('`');
    parts.next();
    parts
        .find(|candidate| candidate.starts_with("git "))
        .unwrap_or_else(|| panic!("the route names a `git …` command; got: {route}"))
        .to_owned()
}

/// **The trackedness leg.** An untracked in-repo source is refused with its own code, a locus
/// and one route — and the route *works*: running the printed `git add` verbatim and re-running
/// the **identical** `jigc migrate` succeeds.
///
/// This is what makes the leg a narrowing rather than a dead end (M50's `write.not-present`
/// lesson): the predicate is membership of the **index**, not of `HEAD`, so the single `git add`
/// the route names is sufficient — the operator does not have to commit a foreign file they are
/// about to retire in order to be allowed to retire it.
///
/// The re-run is byte-identical to the refused invocation on purpose. A leg whose escape needs a
/// *different* jigc command would be a second door, and the refusal does not name one.
#[test]
fn an_untracked_in_repo_source_is_refused_and_the_printed_git_add_makes_it_admissible() {
    let corpus = TrialCorpus::build(State::Fresh);
    let source = corpus.repo().join("HISTORY.md");
    fs::write(&source, FOREIGN).expect("plant the untracked foreign source");

    let out = corpus.jigc(&["migrate", "HISTORY.md", "--as", "changelog"]);
    let stderr = assert_refused_with(
        &corpus,
        &out,
        UNTRACKED,
        "migrate <an untracked in-repo source>",
    );
    assert!(
        stderr.contains("at: HISTORY.md"),
        "the refusal names WHERE — the repo-relative source it is about (M49's located-finding \
         rule); got:\n{stderr}",
    );
    assert!(
        source.exists(),
        "the untracked source must survive a refusal that never read past the door",
    );

    // The route, run as printed. `sh -c` rather than a split argv, because *verbatim* is the
    // claim: the operator copies the line out of the terminal.
    let command = printed_git_command(&stderr);
    assert!(
        command.starts_with("git add "),
        "the trackedness leg routes at `git add`, the one act that resolves the state \
         (M45's owner-artifact precedent); got: {command}",
    );
    let ran = std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .current_dir(corpus.repo())
        .env("HOME", corpus.home())
        .output()
        .expect("run the printed git command");
    assert!(
        ran.status.success(),
        "the printed `{command}` must run as printed; got {}:\n{}",
        ran.status,
        String::from_utf8_lossy(&ran.stderr),
    );

    corpus.jigc_ok(&["migrate", "HISTORY.md", "--as", "changelog"]);
    let minted = task_dirs(&corpus.repo());
    assert_eq!(
        minted.len(),
        1,
        "the re-run after the printed route mints exactly one migration task; got {minted:?}",
    );
    let recorded = fs::read_to_string(
        corpus
            .repo()
            .join(".jigc")
            .join("tasks")
            .join(&minted[0])
            .join("source-path"),
    )
    .expect("read the recorded source-path");
    assert_eq!(
        recorded, "HISTORY.md",
        "the admitted source records the spelling the operator staged",
    );
}

/// …and a **committed** source is unaffected — the ordinary case the leg must not touch.
///
/// Stated as its own arm rather than folded into the one above, because the two prove different
/// things: that one proves the escape is reachable, this one proves the leg has a complement at
/// all. A predicate that refused every source would pass every assertion in arm (g) up to the
/// re-run and still have broken migration outright.
#[test]
fn a_committed_source_is_unaffected_by_the_trackedness_leg() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::write(corpus.repo().join("HISTORY.md"), FOREIGN).expect("write the foreign source");
    corpus.git(&["add", "--", "HISTORY.md"]);
    corpus.git(&["commit", "-q", "-m", "vendor the foreign changelog"]);

    corpus.jigc_ok(&["migrate", "HISTORY.md", "--as", "changelog"]);
    assert_eq!(
        task_dirs(&corpus.repo()).len(),
        1,
        "a committed foreign source migrates exactly as it did before the leg",
    );
}

// ───────────── the step text, checked against both admissible cells ─────────────

/// The foreign `vision` source the two cells below migrate — a non-conformant document
/// with real prose, so the author step has something to route.
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

/// Drive one foreign `vision` source at `path` all the way through `finalize --approve`,
/// and answer with the sha of the commit it landed.
fn migrate_and_approve(corpus: &TrialCorpus, path: &str) -> String {
    let composed = corpus.jigc_ok(&["migrate", path, "--as", "vision"]);
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
    corpus.finalize(&task, "vision", "migrate the direction doc", true);
    corpus.git(&["rev-parse", "HEAD"])
}

/// **The pack step's own sentence, checked against what the two admissible cells do**
/// (`crates/cli/pack/steps/migration-finalize.yaml` and its methodology twin; the Settle's
/// owed G-5 re-drive, discharged here rather than restated).
///
/// The Settle predicted that the trackedness leg would make the old sentence — *"the
/// deletion staged, so the removal lands in the same commit as the canonical doc (a `git rm`
/// in effect)"* — true by refusing the one cell that falsified it, and that **no pack repair
/// was owed**. Driven at this task, that prediction is false: the leg's predicate is the
/// **index**, so a source that was `git add`ed and never committed is *admitted*, and on
/// `--approve` it leaves no row in the commit and nothing in history — `git log --all --
/// <path>` is empty, exactly as it was for the untracked cell. The old sentence was
/// therefore repaired rather than left standing, and this arm is what keeps the repair
/// honest: it asserts the **behaviour** of both cells and the **statement** in the same test,
/// so prose and binary cannot drift apart again.
///
/// Both cells are admissible on purpose. The staged-only one is not a defect to fix here —
/// it is the price of the predicate the Settle chose deliberately, so that the printed `git
/// add` route is *sufficient* and nobody has to commit a foreign file in order to be allowed
/// to retire it (M50's `write.not-present` lesson). What the wave owes such a cell is that no
/// surface claim about it be false.
#[test]
fn the_migration_finalize_step_states_what_both_admissible_cells_actually_do() {
    // (i) committed — the removal lands in the commit, the original survives in history.
    let corpus = TrialCorpus::build(State::Fresh);
    fs::create_dir_all(corpus.repo().join("docs")).expect("create docs/");
    fs::write(corpus.repo().join("docs/direction.md"), FOREIGN_VISION).expect("write the source");
    corpus.git(&["add", "--", "docs/direction.md"]);
    corpus.git(&["commit", "-q", "-m", "add the direction doc"]);

    let sha = migrate_and_approve(&corpus, "docs/direction.md");
    let landed = corpus.git(&["show", "--name-status", "--format=", &sha]);
    assert!(
        landed
            .lines()
            .any(|line| line.starts_with('D') && line.contains("docs/direction.md")),
        "a COMMITTED source retires as a deletion in the migration commit — the step's \
         `git rm` sentence; got:\n{landed}",
    );
    assert!(
        !corpus
            .git(&["log", "--all", "--format=%H", "--", "docs/direction.md"])
            .is_empty(),
        "…and the original stays recoverable from history",
    );

    // (ii) staged, never committed — admitted, and the removal reaches no commit at all.
    let corpus = TrialCorpus::build(State::Fresh);
    fs::create_dir_all(corpus.repo().join("docs")).expect("create docs/");
    fs::write(corpus.repo().join("docs/direction.md"), FOREIGN_VISION).expect("write the source");
    corpus.git(&["add", "--", "docs/direction.md"]);

    let sha = migrate_and_approve(&corpus, "docs/direction.md");
    let landed = corpus.git(&["show", "--name-status", "--format=", &sha]);
    assert!(
        !landed.contains("docs/direction.md"),
        "a STAGED-ONLY source has no committed copy for the deletion to point at, so the \
         migration commit carries no row for it — the sentence the repair added; got:\n{landed}",
    );
    assert!(
        corpus
            .git(&["log", "--all", "--format=%H", "--", "docs/direction.md"])
            .is_empty(),
        "…and no history holds the original — which is why the step now says to commit the \
         source first if you want one",
    );

    // The statement itself, read off the composed surface the agent actually sees.
    let composed = corpus.jigc_ok(&["workflow", "--preview", "migrate-vision"]);
    let flat = composed.split_whitespace().collect::<Vec<_>>().join(" ");
    for fragment in [
        "the deletion staged",
        "had already committed the removal lands in the same commit",
        "only `git add`ed and never committed has no committed copy",
        "refused at `jigc migrate`",
    ] {
        assert!(
            flat.contains(fragment),
            "the migration-finalize step must state `{fragment}`; got:\n{composed}",
        );
    }
}

/// …and a source git holds in **`HEAD` but not in the index** is admitted too.
///
/// This is the cell the Settle's shorthand (*"the index, not HEAD"*) would have refused, and
/// refusing it would have been a regression rather than a narrowing: the user who committed a
/// foreign file and then pre-staged its own deletion (`git rm --cached`, so the worktree bytes
/// survive for `migrate` to read) is walking a **documented** path — M40 F7 discriminates the
/// retirement pathspec on the index precisely so that pre-staged deletion still lands
/// (`design/finalize.md`; `crates/cli/tests/carryover_gate.rs` drives it end to end). Git holds
/// a perfectly good copy of the bytes, which is what the warrant asks; `git ls-files` alone
/// cannot see it.
#[test]
fn a_source_committed_then_unstaged_is_admitted_because_git_still_holds_a_copy() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::write(corpus.repo().join("HISTORY.md"), FOREIGN).expect("write the foreign source");
    corpus.git(&["add", "--", "HISTORY.md"]);
    corpus.git(&["commit", "-q", "-m", "vendor the foreign changelog"]);
    corpus.git(&["rm", "-q", "--cached", "--", "HISTORY.md"]);
    assert!(
        corpus.git(&["ls-files", "--", "HISTORY.md"]).is_empty(),
        "the fixture must leave the source OUT of the index — otherwise this arm is the \
         committed one over again",
    );

    corpus.jigc_ok(&["migrate", "HISTORY.md", "--as", "changelog"]);
    assert_eq!(
        task_dirs(&corpus.repo()).len(),
        1,
        "a source `HEAD` still holds migrates — the leg refuses *no copy anywhere*, never \
         *not in the index*",
    );
}

// ───────── (h) the finding inventory registers exactly this increment's codes ─────────

/// The design doc that owns the finding inventory — `validation.md`'s **Severity inventory**
/// is the one home a check's severity class, its intrinsic-or-tunable classification and its
/// keyed-or-unkeyed status are stated in, and the M45 / M49 registration sections are the
/// precedent for a wave that mints findings without minting keyed check ids.
const INVENTORY_DOC: &str = "design/validation.md";

/// The section heading this increment's registration table sits under, verbatim.
///
/// **Keyed per increment, deliberately.** M51 mints ten codes across the wave
/// (`completions/artifacts/M51/settle-record.md` → §10's table), and each increment's own
/// arm asserts **equality** over its own mints. One shared section would make every such arm
/// red the moment a sibling increment registered its codes — equality over a set that is not
/// the arm's subject. So each increment registers under its own sibling heading, and this
/// arm's subject is exactly the four Increment 1 ships.
const INVENTORY_HEADING: &str = "### The M51 registrations — Increment 1: the path-argument rules";

/// **The four codes Increment 1 mints**, spelled here and deliberately **not** imported from
/// the production constants — the `design/structural-grammar.md` literal-equality precedent
/// (`crates/cli/tests/malformed_work_unit_id.rs` →
/// `the_design_doc_states_the_shipped_grammar_verbatim`).
///
/// The chain that makes this a fence rather than a spell-check has three links, and the arm
/// below asserts all three:
///
/// 1. **doc == these literals.** The registration table's code column is *scraped* and
///    compared as a **set**, so a fifth row, a missing row or a re-spelled code reddens —
///    `contains` alone would pass a table that registered a code the binary never emits.
/// 2. **these literals == what the binary emits.** Every member is asserted to be a code some
///    shipped production registry declares — `cli::cli::PATH_ARG_OCCURRENCES` for the three
///    minted at a door argument (itself ⇔-fenced against the real clap tree, so it cannot
///    drift into documentation) and `cli::render::FINALIZE_FAMILY` for the sink's, which is
///    minted one statement above the `remove_file` it guards and therefore belongs to no
///    argument. On the wire: arms (a)–(g) above drive `migrate.source-untrackable` and
///    `migrate.source-untracked` through the real binary, and the two siblings in this group
///    target drive the others (`retire_sink_validation.rs`, `step_source_rules.rs`).
/// 3. **none of them is an `ERROR_CODE_REGISTRY` member** (`settle-record.md` → §10). That
///    registry mirrors **door identities** derived from `COMMITTING_DOORS`, and a blocking
///    `Finding` is not an `Outcome` identity; registering one there would put a finding in a
///    set whose own fence is a doc mirror of a different thing.
const INCREMENT_CODES: [&str; 4] = [
    "config.step-source-untrackable",
    "finalize.retire-untrackable",
    "migrate.source-untrackable",
    "migrate.source-untracked",
];

/// The repository root, two levels above `crates/cli`.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the repo root sits two levels above crates/cli")
        .to_path_buf()
}

/// The block of [`INVENTORY_DOC`] under [`INVENTORY_HEADING`], up to the next heading of the
/// same or a higher level — the section, never the rest of the file.
fn registration_section(doc: &str) -> &str {
    let start = doc.find(INVENTORY_HEADING).unwrap_or_else(|| {
        panic!(
            "{INVENTORY_DOC} must carry the section `{INVENTORY_HEADING}` — this increment \
             mints four blocking findings, and `validation.md`'s Severity inventory is the \
             home the wave's Settle named for registering them (settle-record.md → §10)",
        )
    });
    let body = &doc[start + INVENTORY_HEADING.len()..];
    let end = body
        .match_indices('\n')
        .map(|(at, _)| at + 1)
        .find(|at| body[*at..].starts_with("## ") || body[*at..].starts_with("### "))
        .unwrap_or(body.len());
    &body[..end]
}

/// Every code the registration table's **first column** names, as a set.
fn registered_codes(section: &str) -> BTreeSet<String> {
    section
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        // Drop the header row and the `|---|` separator: neither carries a backticked code.
        .filter_map(|line| line.trim_start_matches('|').split('|').next())
        .flat_map(|first_cell| {
            first_cell
                .split('`')
                .skip(1)
                .step_by(2)
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect()
}

/// **The inventory names exactly the codes this increment's binary emits, and none of them
/// joins the door-identity registry** (`settle-record.md` → §10; the roadmap's *Codes it
/// registers*).
///
/// Red at this task's start, and at the section-absent panic rather than the equality: T9
/// began with `validation.md` carrying no M51 registration at all, so the codes T1–T4 had
/// already minted were named in no inventory. It is the one mechanical check on a task whose
/// deliverable is otherwise prose — the rest of T9's loci state *rules*, and a byte-assert
/// over a rule's wording would pin editorial phrasing rather than a contract, which is why
/// they are verified by reading against the stated command in `DECISIONS.md` instead.
#[test]
fn the_finding_inventory_registers_exactly_this_increments_codes() {
    let root = repo_root();
    let doc = fs::read_to_string(root.join(INVENTORY_DOC))
        .unwrap_or_else(|err| panic!("read {INVENTORY_DOC}: {err}"));
    let section = registration_section(&doc);

    // 1 — doc == these literals, as a SET.
    let registered = registered_codes(section);
    let expected: BTreeSet<String> = INCREMENT_CODES
        .iter()
        .map(|code| (*code).to_owned())
        .collect();
    assert_eq!(
        registered, expected,
        "{INVENTORY_DOC} → `{INVENTORY_HEADING}` must register EXACTLY the codes this \
         increment mints — one table row per code, the code alone in the first column. A \
         row for a code the binary never emits is a lie on the inventory that calls itself \
         *the single source of truth* for what the engine emits; a missing row is the \
         silence §10 exists to end.\nsection read:\n{section}",
    );

    // 2 — these literals == what the binary emits, sourced from the shipped registries.
    let door_codes: BTreeSet<&str> = cli::cli::PATH_ARG_OCCURRENCES
        .iter()
        .flat_map(|occurrence| occurrence.arms)
        .filter_map(|arm| match arm.disposition {
            cli::cli::PathArgDisposition::Adjudicated { codes, .. } => Some(codes),
            cli::cli::PathArgDisposition::NoRule { .. } => None,
        })
        .flatten()
        .copied()
        .collect();
    let finalize_codes: BTreeSet<&str> = cli::render::FINALIZE_FAMILY
        .iter()
        .map(|member| member.code)
        .collect();
    for code in INCREMENT_CODES {
        assert!(
            door_codes.contains(code) || finalize_codes.contains(code),
            "`{code}` must be declared by a shipped production registry — \
             `cli::cli::PATH_ARG_OCCURRENCES` for a code minted at a door argument, \
             `cli::render::FINALIZE_FAMILY` for one minted inside the finalize transaction. \
             A code the inventory registers and no registry declares is a doc entry with \
             nothing behind it.",
        );
    }

    // 3 — and none of them joins the door-identity registry (§10).
    for code in INCREMENT_CODES {
        assert!(
            !cli::invocation_log::ERROR_CODE_REGISTRY.contains(&code),
            "`{code}` must stay OUT of `ERROR_CODE_REGISTRY`: that registry mirrors door \
             identities derived from `COMMITTING_DOORS`, and a blocking `Finding` is not an \
             `Outcome` identity (settle-record.md → §10)",
        );
    }
    assert!(
        section.contains("ERROR_CODE_REGISTRY"),
        "…and the section must SAY so — the reason a blocking finding is not a door identity \
         is the half a reader cannot derive from the table, and §10 decided it once for the \
         whole family",
    );
}
