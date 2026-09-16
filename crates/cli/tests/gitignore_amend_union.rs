//! **`gitignore::ensure` amends `.jigc/.gitignore` to the union instead of replacing
//! it** (M51 Increment 4 / T1; `completions/artifacts/M51/settle-record.md` → §6's byte
//! algorithm and EC-18 · `implementation/roadmap.md` → Milestone 51 Increment 4 ·
//! `completions/artifacts/M51/gap-findings.md` → G-46).
//!
//! **The base-red, driven at `1922c7f5`.** A repo whose `.jigc/.gitignore` carried a
//! user's own two lines (`# my private stuff` + `build-cache/`) and was one entry short
//! of the canonical set: `jigc setup` exited **0**, the file came back as exactly
//! [`ENTRIES`], `git status --short` was **empty** — the file had been rewritten to
//! match `HEAD` — and the user's two lines existed in no git object anywhere. The same
//! writer runs inside `task finalize`, where the replacement is committed.
//!
//! **The settled byte algorithm (§6, Codex 8).** *Preserve all existing bytes exactly;
//! append only missing canonical entries in fixed order; insert exactly one separator
//! newline only when required; never normalize or deduplicate existing content; reject
//! non-regular, symlinked or undecodable files.* Comments, blank lines, CRLF, a missing
//! final newline and pre-existing duplicates therefore survive untouched, and the result
//! is byte-idempotent **by construction** rather than by inspection.
//!
//! **Two axes, because one does not imply the other.**
//!
//! 1. **The byte axis** — eleven planted shapes over the function itself. It is a
//!    **manufactured** shape space and says so: the shapes a user's own `.gitignore` can
//!    take (a comment, a blank line, CRLF, a duplicate, a missing final newline) are a
//!    property of text files, not of anything jigc enumerates, so no code-side registry
//!    could generate this set. Every cell is built **from [`ENTRIES`]** rather than from a
//!    transcribed copy of it, so an entry joining the constant does not silently make a
//!    fixture assert yesterday's set (G-46's own warning).
//! 2. **The caller axis** — all four production callers, each driven **twice** through
//!    the real binary over the same planted file. Idempotency is the load-bearing half:
//!    the amend runs inside `finalize`'s commit transaction, so a writer that churned the
//!    file would make every finalize commit a diff nobody authored. Per-caller
//!    idempotency proved at the function does not imply it across callers — each one
//!    reaches `ensure` from a different `jigc_root` derivation (`repo_root` at `setup`,
//!    `jigc_home` at the three others), which is exactly the axis that had already
//!    drifted once (the pre-M39 three-literal divergence this module was minted to end).
//!
//! Plus the **three reject legs**, which are the algorithm's other half: a writer that
//! *appends* must refuse what it cannot read back byte-for-byte.

use std::fs;
use std::path::{Path, PathBuf};

use cli::gitignore::{self, ENTRIES, Ensured};

use crate::support;
use support::trial_corpus::{State, TrialCorpus};

/// The two lines the planted files carry that jigc has no opinion about — the user's
/// own, and the bytes the base-red destroyed.
const PRIVATE: &str = "# my private stuff\nbuild-cache/\n";

/// A throwaway directory that removes itself on drop (the no-tempfile-crate pattern).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-gitignore-union-{tag}-{}-{:?}",
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

/// The canonical set's **last** entry — the one every "missing entry" cell drops, read
/// from [`ENTRIES`] so the fixtures never transcribe the constant.
fn last_entry() -> &'static str {
    ENTRIES.lines().next_back().expect("ENTRIES is non-empty")
}

/// [`ENTRIES`] minus its last line — the "one entry short" body every amending cell
/// starts from.
fn entries_but_last() -> String {
    let last = last_entry();
    ENTRIES
        .lines()
        .filter(|line| *line != last)
        .map(|line| format!("{line}\n"))
        .collect()
}

/// Read `<root>/.gitignore` as raw bytes — raw, because half the cells exist to prove
/// that bytes `String` would normalize (CRLF, a missing final newline) come back
/// untouched.
fn body(root: &Path) -> Vec<u8> {
    fs::read(root.join(".gitignore")).expect("read the .gitignore")
}

// ---------------------------------------------------------------------------
// Axis 1 — the byte axis: eleven planted shapes (a MANUFACTURED shape space)
// ---------------------------------------------------------------------------

/// One planted shape: what is on disk before [`gitignore::ensure`], and which canonical
/// entries the amend is expected to append. The appended set is **declared per cell**
/// rather than recomputed the way the implementation computes it — a test that derives
/// its expectation from the algorithm under test asserts nothing.
struct Cell {
    label: &'static str,
    /// `None` is the absent file; `Some(bytes)` is planted verbatim.
    planted: Option<String>,
    /// The entries the amend must append, in [`ENTRIES`] order.
    appended: Vec<&'static str>,
    /// The report [`gitignore::ensure`] must return for this shape.
    report: Ensured,
}

/// The eleven cells, every one built from [`ENTRIES`].
fn cells() -> Vec<Cell> {
    let last = last_entry();
    let all: Vec<&'static str> = ENTRIES.lines().collect();
    let short = entries_but_last();
    vec![
        Cell {
            label: "absent",
            planted: None,
            appended: all.clone(),
            report: Ensured::Created,
        },
        Cell {
            label: "empty",
            planted: Some(String::new()),
            appended: all.clone(),
            report: Ensured::Amended {
                appended: all.clone(),
            },
        },
        Cell {
            label: "canonical",
            planted: Some(ENTRIES.to_string()),
            appended: Vec::new(),
            report: Ensured::Unchanged,
        },
        Cell {
            label: "canonical + a comment",
            planted: Some(format!("# jigc's transient workbench\n{ENTRIES}")),
            appended: Vec::new(),
            report: Ensured::Unchanged,
        },
        Cell {
            label: "a blank line",
            planted: Some(ENTRIES.replacen('\n', "\n\n", 1)),
            appended: Vec::new(),
            report: Ensured::Unchanged,
        },
        Cell {
            label: "CRLF",
            planted: Some(ENTRIES.replace('\n', "\r\n")),
            appended: Vec::new(),
            report: Ensured::Unchanged,
        },
        Cell {
            label: "no final newline",
            planted: Some(short.trim_end_matches('\n').to_string()),
            appended: vec![last],
            report: Ensured::Amended {
                appended: vec![last],
            },
        },
        Cell {
            label: "a pre-existing duplicate",
            planted: Some(format!("{ENTRIES}{}\n", all[0])),
            appended: Vec::new(),
            report: Ensured::Unchanged,
        },
        Cell {
            label: "entries out of order",
            planted: Some(
                all.iter()
                    .rev()
                    .map(|line| format!("{line}\n"))
                    .collect::<String>(),
            ),
            appended: Vec::new(),
            report: Ensured::Unchanged,
        },
        Cell {
            label: "an entry with trailing spaces",
            planted: Some(ENTRIES.replacen('\n', "   \n", 1)),
            appended: Vec::new(),
            report: Ensured::Unchanged,
        },
        Cell {
            label: "a missing entry with a private line",
            planted: Some(format!("{short}{PRIVATE}")),
            appended: vec![last],
            report: Ensured::Amended {
                appended: vec![last],
            },
        },
    ]
}

/// **Every existing byte survives, the appended set is exactly the missing entries in
/// [`ENTRIES`] order, and a second call writes nothing.**
///
/// The three assertions are one claim in three parts: what was there is still there
/// *byte-for-byte* (so the private line, the comment, the CRLF and the duplicate all
/// live), what is new is exactly the missing canonical entries and nothing else, and the
/// result is a **fixed point** — which is what lets `finalize` commit the file without
/// authoring a diff of its own.
#[test]
fn every_planted_shape_keeps_its_bytes_and_gains_only_what_is_missing() {
    for cell in cells() {
        let dir = TempDir::new("bytes");
        let root = dir.path().join(".jigc");
        if let Some(planted) = &cell.planted {
            fs::create_dir_all(&root).expect("create the jigc root");
            fs::write(root.join(".gitignore"), planted).expect("plant the .gitignore");
        }

        let report = gitignore::ensure(&root).expect("the amend runs");
        assert_eq!(
            report, cell.report,
            "[{}] the report must name what changed",
            cell.label,
        );

        let after = body(&root);
        let planted = cell.planted.clone().unwrap_or_default();
        assert!(
            after.starts_with(planted.as_bytes()),
            "[{}] every existing byte must survive byte-for-byte; planted {planted:?}, got {:?}",
            cell.label,
            String::from_utf8_lossy(&after),
        );
        // Exactly one separator newline, and only when the existing bytes do not
        // already end in one.
        let separator = if planted.is_empty() || planted.ends_with('\n') || cell.appended.is_empty()
        {
            ""
        } else {
            "\n"
        };
        let expected = format!(
            "{planted}{separator}{}",
            cell.appended
                .iter()
                .map(|entry| format!("{entry}\n"))
                .collect::<String>(),
        );
        assert_eq!(
            String::from_utf8_lossy(&after),
            expected,
            "[{}] the appended set must be exactly the missing entries, in ENTRIES order",
            cell.label,
        );

        let again = gitignore::ensure(&root).expect("the second amend runs");
        assert_eq!(
            again,
            Ensured::Unchanged,
            "[{}] a second call has nothing to append",
            cell.label,
        );
        assert_eq!(
            body(&root),
            after,
            "[{}] a second call is byte-identical — the amend is a fixed point",
            cell.label,
        );
    }
}

// ---------------------------------------------------------------------------
// Axis 1b — the three reject legs
// ---------------------------------------------------------------------------

/// **A file jigc cannot read back byte-for-byte is refused, not replaced.** An amend
/// reads the existing bytes and writes them back; a directory, a symlink and an
/// undecodable file each break that contract in a different place, and the old
/// *replace* writer would have silently clobbered all three. The error carries the
/// offending path — §10 mints no finding code for an I/O fault, so nothing is minted
/// homeless.
#[test]
fn a_file_the_amend_cannot_read_back_is_refused() {
    // (1) Non-regular: a directory where the file belongs.
    let dir = TempDir::new("nonregular");
    let root = dir.path().join(".jigc");
    fs::create_dir_all(root.join(".gitignore")).expect("plant a directory");
    let err = gitignore::ensure(&root).expect_err("a directory is refused");
    assert!(
        err.to_string().contains(".gitignore"),
        "the refusal names the path: {err}",
    );
    assert!(
        root.join(".gitignore").is_dir(),
        "the refusal writes nothing",
    );

    // (2) A symlink — following it would rewrite a file outside `.jigc/`.
    let dir = TempDir::new("symlink");
    let root = dir.path().join(".jigc");
    fs::create_dir_all(&root).expect("create the jigc root");
    let target = dir.path().join("elsewhere");
    fs::write(&target, "not jigc's file\n").expect("write the link target");
    std::os::unix::fs::symlink(&target, root.join(".gitignore")).expect("plant a symlink");
    let err = gitignore::ensure(&root).expect_err("a symlink is refused");
    assert!(
        err.to_string().contains(".gitignore"),
        "the refusal names the path: {err}",
    );
    assert_eq!(
        fs::read_to_string(&target).expect("read the link target"),
        "not jigc's file\n",
        "the refusal leaves the link target untouched",
    );

    // (3) Undecodable — a lone 0x80 continuation byte is valid in no UTF-8 sequence.
    let dir = TempDir::new("undecodable");
    let root = dir.path().join(".jigc");
    fs::create_dir_all(&root).expect("create the jigc root");
    let raw = b"tasks/\n\x80\xff\n";
    fs::write(root.join(".gitignore"), raw).expect("plant undecodable bytes");
    let err = gitignore::ensure(&root).expect_err("undecodable bytes are refused");
    assert!(
        err.to_string().contains(".gitignore"),
        "the refusal names the path: {err}",
    );
    assert_eq!(
        body(&root),
        raw.to_vec(),
        "the refusal leaves the undecodable bytes exactly as they were",
    );
}

// ---------------------------------------------------------------------------
// Axis 2 — the caller axis: four production callers, each driven twice
// ---------------------------------------------------------------------------

/// Plant the one-entry-short body carrying [`PRIVATE`] at `<repo>/.jigc/.gitignore`,
/// returning the planted bytes.
fn plant(repo: &Path) -> String {
    let planted = format!("{}{PRIVATE}", entries_but_last());
    fs::write(repo.join(".jigc").join(".gitignore"), &planted).expect("plant the .gitignore");
    planted
}

/// [`plant`], then **commit it** — the shape a team that keeps its own `.jigc/.gitignore`
/// lines actually has, and the only one that reaches `setup`'s amend since Increment 3's
/// guard refuses over an install path whose bytes are in no commit.
fn plant_committed(corpus: &TrialCorpus) -> String {
    let planted = plant(&corpus.repo());
    corpus.git(&["add", ".jigc/.gitignore"]);
    corpus.git(&["commit", "-q", "-m", "keep our own .jigc ignores"]);
    planted
}

/// What every caller must leave behind: the planted bytes plus the one missing entry.
fn amended(planted: &str) -> String {
    format!("{planted}{}\n", last_entry())
}

/// Assert `<repo>/.jigc/.gitignore` is the amended body — the user's lines intact, the
/// canonical set complete.
fn assert_amended(repo: &Path, planted: &str, caller: &str, run: &str) {
    let after = fs::read_to_string(repo.join(".jigc").join(".gitignore")).expect("read");
    assert!(
        after.contains("build-cache/"),
        "[{caller} · {run}] the user's own line must survive; got:\n{after}",
    );
    assert_eq!(
        after,
        amended(planted),
        "[{caller} · {run}] the amend appends the missing entry and nothing else",
    );
}

/// **`jigc setup` (`adapter.rs` → `init_project_layer`).** The caller that justifies the
/// amend being a precondition rather than a rider: `setup` *commits* what it writes, so
/// before this fix the user's lines were replaced **and landed** — `git status` clean,
/// the bytes in no git object (§6's declared bound, driven).
///
/// **The plant is committed here, and that is a narrowing the M51 completion audit
/// forced.** Increment 3's `setup.dirty-install-path` now refuses *before any write* over
/// an install path carrying bytes in no commit, so an **uncommitted** plant no longer
/// reaches the amend at all — it is refused, with the user's lines intact, which is the
/// stronger answer. What the amend is still owed over is the shape a team actually has:
/// the private lines **committed**, and a later jigc adding an entry to `ENTRIES`.
#[test]
fn setup_amends_and_is_idempotent() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let planted = plant_committed(&corpus);
    for run in ["first", "second"] {
        corpus.jigc(&["setup"]);
        assert_amended(&repo, &planted, "setup", run);
    }
}

/// **The sibling cell the narrowing above creates**: an *uncommitted* private line at an
/// install path is not amended-over at all — `setup` refuses before writing a byte, and the
/// user's exact planted bytes are still there (Increment 3's guard, M51 completion audit).
#[test]
fn setup_refuses_rather_than_amending_an_uncommitted_plant() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let planted = plant(&repo);
    let out = corpus.jigc(&["setup"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "an install path carrying bytes in no commit refuses: {}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert_eq!(
        read_ignore(&repo),
        planted,
        "and the refusal leaves the user's planted bytes exactly as they were",
    );
}

/// **`jigc task finalize` (`task.rs`, inside the commit transaction).** The churn cell:
/// a writer that rewrote the file on every finalize would put a diff nobody authored
/// into every commit, which is why byte-idempotency is what this axis proves.
#[test]
fn task_finalize_amends_and_is_idempotent() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let planted = plant(&repo);
    for (run, tag) in [("first", "one"), ("second", "two")] {
        let task = corpus.start_workflow("single-task", &format!("Change {tag}"));
        // The code change is written and staged AFTER the mint, so it is the task's own
        // work rather than carryover the gate would refuse.
        let file = format!("work-{tag}.txt");
        fs::write(repo.join(&file), "work\n").expect("write the code change");
        corpus.git(&["add", &file]);
        corpus.finalize(&task, "core", &format!("change {tag}"), false);
        assert_amended(&repo, &planted, "task finalize", run);
    }
}

/// **`jigc milestone create` (`milestone.rs`).** Reaches `ensure` from `jigc_home`, not
/// `repo_root` — the derivation that makes cross-caller idempotency a claim rather than
/// a corollary.
#[test]
fn milestone_create_amends_and_is_idempotent() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let planted = plant(&repo);
    for (run, title) in [("first", "Cache rework"), ("second", "Cache polish")] {
        corpus.jigc_ok(&["milestone", "create", title]);
        assert_amended(&repo, &planted, "milestone create", run);
    }
}

/// **`jigc milestone provision` (`milestone.rs`).** The caller that **never commits**, so
/// on an `ENTRIES` upgrade a private line died here with no transaction at all — visibly
/// (` M .jigc/.gitignore`) but destroyed all the same (§6's declared bound).
#[test]
fn milestone_provision_amends_and_is_idempotent() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    corpus.jigc_ok(&["milestone", "add-task", "cache-rework", "Area zed"]);
    let planted = plant(&repo);
    for run in ["first", "second"] {
        corpus.jigc_ok(&["milestone", "provision", "cache-rework"]);
        assert_amended(&repo, &planted, "milestone provision", run);
    }
}

/// The four callers agree **byte-for-byte** over the same planted file. Per-caller
/// idempotency does not imply this: each reaches `ensure` from its own root derivation,
/// and the divergence this module was minted to end (`worktrees/` present in one writer
/// and absent from two) was exactly a cross-caller disagreement nothing measured.
#[test]
fn the_four_callers_leave_identical_bytes() {
    let mut bodies: Vec<(&str, String)> = Vec::new();

    // The `setup` arm's plant is **committed**: since the M51 completion audit an
    // uncommitted install path refuses before the amend can run (see
    // `setup_refuses_rather_than_amending_an_uncommitted_plant`), and the bytes the four
    // callers must agree on are the amend's, not the guard's.
    let setup = TrialCorpus::build(State::Fresh);
    let planted = plant_committed(&setup);
    setup.jigc(&["setup"]);
    bodies.push(("setup", read_ignore(&setup.repo())));

    let finalize = TrialCorpus::build(State::Fresh);
    plant(&finalize.repo());
    let task = finalize.start_workflow("single-task", "Change one");
    fs::write(finalize.repo().join("work.txt"), "work\n").expect("write the code change");
    finalize.git(&["add", "work.txt"]);
    finalize.finalize(&task, "core", "change one", false);
    bodies.push(("task finalize", read_ignore(&finalize.repo())));

    let create = TrialCorpus::build(State::Fresh);
    plant(&create.repo());
    create.jigc_ok(&["milestone", "create", "Cache rework"]);
    bodies.push(("milestone create", read_ignore(&create.repo())));

    let provision = TrialCorpus::build(State::Fresh);
    provision.jigc_ok(&["milestone", "create", "Cache rework"]);
    provision.jigc_ok(&["milestone", "add-task", "cache-rework", "Area zed"]);
    plant(&provision.repo());
    provision.jigc_ok(&["milestone", "provision", "cache-rework"]);
    bodies.push(("milestone provision", read_ignore(&provision.repo())));

    for (caller, got) in &bodies {
        assert_eq!(
            got,
            &amended(&planted),
            "[{caller}] must leave the identical amended bytes",
        );
    }
}

/// `<repo>/.jigc/.gitignore`, as a string.
fn read_ignore(repo: &Path) -> String {
    fs::read_to_string(repo.join(".jigc").join(".gitignore")).expect("read the .gitignore")
}
