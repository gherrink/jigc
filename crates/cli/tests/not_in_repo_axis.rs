//! M49 Increment 11 / T2 — **the not-a-git-repository axis, one row per leaf verb.**
//!
//! The class: *what jigc says when you run it outside a git repository*. Every leaf verb
//! needs one — the workbench, the committed store, the base pin and the cascade's project
//! layer all hang off the `.git`-bearing root — so the precondition is universal and the
//! answer had better be too. At HEAD it was said **three** ways in **two** contract shapes
//! (`DECISIONS.md` → 2026-08-31 M49 Increment 11 planning, the basis):
//!
//! * `locate.rs` bailed ``not inside a git repository (no .git found from X)`` — reached by
//!   `start`, `validate`, `migrate`, `migrate-corpus`, `relocate`;
//! * **17** production `with_context` sites carried ``not inside a git repository (from X)``
//!   — reached by everything else, `describe` / `doc list` / `doc show` / `task list` /
//!   `ingest` / `config list` / `upgrade` among them (the baseline's *"19"* is corrected on
//!   the conversion's own diff, which removes 18 lines carrying the sentence, one of them
//!   the `bail!`; `DECISIONS.md` → 2026-08-31 M49 Increment 11 / T2);
//! * and `jigc setup` alone minted a routed `Finding` (`setup.repo-root`), joined by
//!   `jigc uninstall`'s `uninstall.repo-root`.
//!
//! Two of the three told a first-time user the **state** and never the **move**: 45 of the
//! 47 leaf verbs printed a bare fact with no route at all. This suite is the fix's
//! acceptance — *one shared precondition text, carrying one route, at every door*.
//!
//! **The route is a `Human` route, and says so rather than manufacturing an argv.** There
//! is no `jigc` command that repairs this state from where the user is standing: the moves
//! are `cd` and `git init`, and `git init` is git's to run, not jigc's. So it rides
//! [`engine::finding::Route::human`] — `design/surface-contract.md` → The route fence:
//! *"a route whose command sits mid-sentence does not fit the constructor, and the
//! constructor is not to be contorted to claim a conversion."*
//!
//! **The door set is derived, not remembered.** The axis is
//! [`cli::cli::VERB_KINDS`](cli::cli::VERB_KINDS) — the table already fenced ⇔ against the
//! real clap tree (`cli_parse::every_leaf_verb_is_classified`), so this suite enumerates
//! *the binary's* leaf verbs rather than the seven doors an audit happened to walk. Every
//! member needs an argv row here or [`arms_cover_every_leaf_verb`] panics: a verb added to
//! the surface cannot ship without an author deciding what it says outside a repository.
//! The measured fact that closes the derivation is that **every one of the 47 requires a
//! repository** — driven, per member, below.
//!
//! Per cell, through the **real binary**, from a directory with no `.git` anywhere above it:
//!
//! 1. the run exits **1** — a clean operational error, never clap's 2 (the argv parsed) and
//!    never a debug-fence panic's 101;
//! 2. stderr carries the **one shared precondition text**, byte-compared against the shared
//!    constructor [`cli::locate::not_in_repo_message`] rather than a copy typed here;
//! 3. it carries the **one shared route**, exactly once;
//! 4. it carries the retired spellings **nowhere**, and says "not inside a git repository"
//!    exactly once — one answer, not a layered one.
//!
//! Arm 2 is the **source** arm: no production site composes the sentence except the shared
//! constructor, so a new door cannot re-mint a fourth spelling by copying a neighbour.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::leaf_argv;

/// The sentence stem every spelling of this refusal has always shared — the needle the
/// source arm counts and the emitted-bytes arm asserts appears exactly once.
const STEM: &str = "not inside a git repository";

/// The retired spelling the 17 `with_context` sites carried. No door may emit it again.
const RETIRED: &str = "not inside a git repository (from ";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-not-in-repo-axis-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The axis input — **the shared minimal-argv table**, not a copy of one
/// ([`crate::support::leaf_argv`]). It moved out of this file at M52 Increment 1 / T3,
/// when the pre-dispatch fault axis needed the same 47 rows; a second copy would rot the
/// first time a leaf gained a required argument, and the suite that was not edited would
/// die at clap with an exit 2 that reads like a declared carve-out.
const ARMS: &[leaf_argv::Arm] = leaf_argv::MINIMAL_ARGV;

/// A directory with **no `.git` anywhere above it**, plus the files the argvs point at
/// (so a cell that somehow got past the precondition would fail on something else, loudly,
/// rather than on a missing file that looks like the same refusal). The file set is the
/// argv table's own ([`leaf_argv::FIXTURE_FILES`]) — the tails name them, so they belong
/// beside the tails.
fn outside_a_repo(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    for (name, body) in leaf_argv::FIXTURE_FILES {
        fs::write(dir.path().join(name), body).expect("write an argv-table fixture file");
    }
    assert!(
        dir.path().ancestors().all(|a| !a.join(".git").exists()),
        "the axis fixture must have no `.git` above it; got {}",
        dir.path().display(),
    );
    dir
}

/// Run `jigc <argv>` with `cwd = dir` and `$HOME = home`.
fn jigc(dir: &Path, home: &Path, argv: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(argv)
        .current_dir(dir)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

// ---------------------------------------------------------------------------------
// The fence — the arm set is the binary's leaf-verb set
// ---------------------------------------------------------------------------------

/// [`ARMS`] ⇔ [`cli::cli::VERB_KINDS`]: every leaf verb the clap tree carries has exactly one arm,
/// and no arm names a path the tree no longer has. A verb added to the surface reddens
/// here until someone decides what it says outside a repository.
#[test]
fn arms_cover_every_leaf_verb() {
    leaf_argv::assert_covers_every_leaf_verb();
}

// ---------------------------------------------------------------------------------
// Arm 1 — every door, driven outside a repository, answers once with one text + one route
// ---------------------------------------------------------------------------------

/// The axis itself. Every leaf verb, driven from a directory with no `.git` above it,
/// emits the shared precondition text and the shared route — and nothing else about the
/// repository.
#[test]
fn every_door_outside_a_repo_answers_with_the_one_text_and_route() {
    let dir = outside_a_repo("axis");
    let home = TempDir::new("home");
    // The binary reports the cwd the OS hands *it*, which on macOS is the canonical path
    // (`/var/…` is a symlink to `/private/var/…`). Canonicalize here so the arm compares
    // the same directory, not two spellings of it.
    let cwd = fs::canonicalize(dir.path()).expect("canonicalize the fixture dir");
    let expected_message = cli::locate::not_in_repo_message(&cwd);
    let expected_route = cli::locate::not_in_repo_route();

    for (path, tail) in ARMS {
        let argv: Vec<&str> = path.iter().copied().chain(tail.iter().copied()).collect();
        let label = argv.join(" ");
        let out = jigc(dir.path(), home.path(), &argv);
        let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");

        assert_eq!(
            out.status.code(),
            Some(1),
            "`jigc {label}` outside a repo must be a clean operational error (clap's 2 means \
             the argv did not parse; 101 is a debug-fence panic).\nstdout:\n{}\nstderr:\n{stderr}",
            String::from_utf8_lossy(&out.stdout),
        );
        assert!(
            stderr.contains(&expected_message),
            "`jigc {label}` must carry the one shared precondition text.\nexpected:\n\
             {expected_message}\ngot:\n{stderr}",
        );
        assert_eq!(
            stderr.matches(expected_route.as_str()).count(),
            1,
            "`jigc {label}` must carry the one shared route, exactly once.\nexpected:\n{}\n\
             got:\n{stderr}",
            expected_route.as_str(),
        );
        assert_eq!(
            stderr.matches(STEM).count(),
            1,
            "`jigc {label}` must say it once — a layered answer is two answers.\ngot:\n{stderr}",
        );
        assert!(
            !stderr.contains(RETIRED),
            "`jigc {label}` still carries the retired spelling.\ngot:\n{stderr}",
        );
    }
}

// ---------------------------------------------------------------------------------
// Arm 2 — the source arm: one constructor composes the sentence, and only it
// ---------------------------------------------------------------------------------

/// No production site composes the precondition sentence except the shared constructor in
/// `locate.rs`. This is what keeps the axis closed going forward: the fourth spelling would
/// arrive the way the first three did — by a new door copying the neighbour it was written
/// beside — and that copy reddens here before it can reach a user.
#[test]
fn only_the_shared_constructor_composes_the_precondition_text() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut carriers: Vec<String> = Vec::new();
    for entry in fs::read_dir(&src).expect("read crates/cli/src") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("read a source file");
        if text.contains(STEM) {
            carriers.push(
                path.file_name()
                    .expect("a file name")
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }
    carriers.sort();
    assert_eq!(
        carriers,
        vec!["locate.rs".to_string()],
        "only `locate.rs` may compose the not-in-a-repository sentence — every other door \
         reaches it through `locate::not_in_repo` / `locate::not_in_repo_finding`",
    );
}
