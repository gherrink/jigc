//! M51 Increment 1 / T1 — **the `jigc migrate` door adjudicates its source path before
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

use std::fs;
use std::path::{Path, PathBuf};

use crate::support::trial_corpus::{State, TrialCorpus};

/// The one code every location leg of this door carries (`settle-record.md` → §10). One code
/// with the reason in the message, on `config.untrackable-root`'s five-reasons-one-code
/// precedent: the operator's fix is the same in all five cases — name a different source.
const CODE: &str = "migrate.source-untrackable";

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
        task_dirs(&corpus.repo()),
        Vec::<String>::new(),
        "{what} must strand no task directory — the refusal is asked BEFORE the mint",
    );
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
#[test]
fn an_absolute_in_repo_spelling_is_accepted_and_records_the_canonical_path() {
    let corpus = TrialCorpus::build(State::Fresh);
    let home = corpus.repo().join("docs").join("changelog");
    fs::create_dir_all(&home).expect("create the old changelog home");
    fs::write(home.join("changelog.md"), FOREIGN).expect("write the foreign changelog");

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
