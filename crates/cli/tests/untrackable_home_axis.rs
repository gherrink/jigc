//! **A home git cannot track is not a home** — the axis of doors that re-point where
//! managed docs live (M49 completion triage, the HIGH data-loss finding).
//!
//! `jigc config set placement-root .git` used to print
//!
//! ```text
//! relocating the committed doc(s) stranded by the `placement-root` re-point to `.git` …
//!   - docs/roadmap.md → .git/roadmap.md
//! ```
//!
//! and exit **0**, after which `git status` held a staged **deletion with no matching
//! add**, `jigc validate` reported no findings, `jigc doc list` named `.git/roadmap.md`
//! as managed — and the next clone had no roadmap at all. The bytes survived only in
//! history.
//!
//! The mechanism is one exit code that lies: `git mv <src> .git/<dst>` prints
//! `error: invalid path '.git/<dst>'` and **exits 0**. It moves the file on disk, drops
//! the source from the index, and adds nothing, because git refuses to record any path
//! with a `.git` component. Every mover in the codebase read that 0 as success.
//!
//! **The axis is the destination, not the knob.** These arms drive both re-pointing
//! doors (`docs-root`, the M39 mover; `placement-root`, this milestone's) through the
//! two shapes git cannot record — a path inside its own directory, and a path outside
//! the repository — and pin the **bound the auditor measured**: a *gitignored*
//! destination is a perfectly trackable path git has merely been told to skip, so
//! `placement-root .jigc` still stages a real rename and must keep working. Refusing
//! "unusual" roots would have broken that; refusing untrackable ones does not.
//!
//! Each refusal is asserted on three things at once, because a door that refuses but
//! half-acts is the same defect wearing a different exit code: the finding is raised,
//! **nothing moved**, and the knob did **not** land.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-untrackable-home-{tag}-{}-{:?}",
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

/// A real git repo with `jigc setup` run and one committed, ingested `roadmap`
/// singleton at its declared home — the doc every arm below tries to lose.
struct Corpus {
    repo: TempDir,
    home: TempDir,
}

/// A conformant, v1-stamped `roadmap` at its declared nested home — the shape `jigc
/// ingest` adopts, so the corpus these arms guard is genuinely managed and `jigc
/// validate`'s verdict on it means something.
const ROADMAP: &str = "\
---
schema-version: 1
---

# Roadmap

## Milestones

### First milestone  {#first-milestone}

#### Proves

The loop closes.

#### Decomposition

One increment.
";

impl Corpus {
    fn new(tag: &str) -> Self {
        let repo = TempDir::new(tag);
        let home = TempDir::new(&format!("{tag}-home"));
        let corpus = Self { repo, home };
        corpus.git(&["init", "-q"]);
        corpus.git(&["config", "user.email", "test@example.com"]);
        corpus.git(&["config", "user.name", "Test"]);
        fs::write(corpus.repo.path().join("README.md"), "hello\n").expect("write file");
        corpus.git(&["add", "."]);
        corpus.git(&["commit", "-q", "-m", "initial"]);
        corpus.ok(&["setup"]);
        corpus.commit_file("docs/roadmap.md", ROADMAP);
        corpus.ok(&["ingest"]);
        corpus
    }

    fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(self.repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout).expect("utf-8 git stdout")
    }

    fn jigc(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    }

    /// `jigc <args>`, asserting exit 0 and returning stdout.
    fn ok(&self, args: &[&str]) -> String {
        let out = self.jigc(args);
        assert!(
            out.status.success(),
            "`jigc {}` must exit 0; stdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout).expect("utf-8 stdout")
    }

    fn commit_file(&self, rel: &str, body: &str) {
        let abs = self.repo.path().join(rel);
        if let Some(parent) = abs.parent() {
            fs::create_dir_all(parent).expect("create parent dir");
        }
        fs::write(&abs, body).expect("write file");
        self.git(&["add", rel]);
        self.git(&["commit", "-q", "-m", &format!("add {rel}")]);
    }

    fn exists(&self, rel: &str) -> bool {
        self.repo.path().join(rel).exists()
    }

    /// Every path `git` currently tracks in the index — the set a clone would receive.
    fn tracked(&self) -> Vec<String> {
        self.git(&["ls-files"]).lines().map(str::to_owned).collect()
    }

    /// The project manifest's raw bytes (`.jigc/config/manifest.yaml`), or `""`.
    fn manifest(&self) -> String {
        fs::read_to_string(
            self.repo
                .path()
                .join(".jigc")
                .join("config")
                .join("manifest.yaml"),
        )
        .unwrap_or_default()
    }
}

/// The shared adjudication of one refused re-point: the door raises the routed finding,
/// the doc is **still where it was and still tracked**, and the knob did not land.
fn assert_refused_and_inert(corpus: &Corpus, key: &str, value: &str) {
    let before = corpus.manifest();
    let out = corpus.jigc(&["config", "set", key, value]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    assert!(
        !out.status.success(),
        "`jigc config set {key} {value}` must NOT exit 0 — a move git cannot record is not \
         a move; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("config.untrackable-root"),
        "the refusal carries its finding code; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(value),
        "the refusal names the value it refused; stderr:\n{stderr}",
    );

    assert!(
        corpus.exists("docs/roadmap.md"),
        "nothing moved — the doc is still at its home",
    );
    assert!(
        corpus.tracked().iter().any(|p| p == "docs/roadmap.md"),
        "the doc is still TRACKED: the defect's signature was a staged deletion with no \
         matching add, so the index is the assertion, not the filesystem; tracked:\n{:#?}",
        corpus.tracked(),
    );
    assert_eq!(
        corpus.manifest(),
        before,
        "a refused set records nothing — a landed knob with no move leaves the store \
         pointing at a home no doc is at",
    );
}

/// **The reported defect.** `placement-root .git` moved the committed roadmap into git's
/// own directory, called it a successful move, and exited 0.
#[test]
fn a_placement_root_inside_the_git_dir_is_refused_and_the_doc_stays_tracked() {
    let corpus = Corpus::new("placement-git");
    assert_refused_and_inert(&corpus, "placement-root", ".git");
    assert!(
        !corpus.exists(".git/roadmap.md"),
        "no copy is left inside the git directory",
    );

    // The store is not merely *reported* clean — it is clean, because nothing moved.
    let report = corpus.ok(&["validate"]);
    assert!(
        !report.contains("file-state."),
        "the refused re-point leaves no file-state finding behind; report:\n{report}",
    );
}

/// **The sibling axis.** The same shape on M39's pre-existing `docs-root` mover — a
/// pre-existing defect this milestone's own flow exercises, so it is fixed here, not
/// deferred.
#[test]
fn a_docs_root_inside_the_git_dir_is_refused_and_the_doc_stays_tracked() {
    let corpus = Corpus::new("docs-git");
    assert_refused_and_inert(&corpus, "docs-root", ".git");
}

/// A nested path inside the git directory is the same answer — the rule is git's
/// (`error: invalid path` on any `.git` component), not a match against one literal.
#[test]
fn a_root_nested_inside_the_git_dir_is_refused_on_both_knobs() {
    let corpus = Corpus::new("nested-git");
    assert_refused_and_inert(&corpus, "docs-root", ".git/jigc-docs");
    assert_refused_and_inert(&corpus, "placement-root", ".git/hooks");
}

/// The second untrackable shape: a root that resolves **outside the repository**. `git
/// mv` already refuses this one loudly (exit 128) — which is why it never lost bytes —
/// but the door refused nothing, printed a per-doc `could not relocate` line, and landed
/// the knob anyway, leaving the store pointing at a home outside the repo.
#[test]
fn a_root_outside_the_repository_is_refused_on_both_knobs() {
    let corpus = Corpus::new("outside");
    assert_refused_and_inert(&corpus, "docs-root", "../elsewhere");
    assert_refused_and_inert(&corpus, "placement-root", "../elsewhere");
}

/// **The bound, pinned.** A *gitignored* destination is a path git can record perfectly
/// well and has merely been told to skip: `.jigc` stages a real `R` rename and loses
/// nothing. The fix refuses destinations git **cannot track**, never roots that merely
/// look unusual — so this arm is what separates the two, and it must keep passing.
#[test]
fn a_gitignored_root_still_relocates_because_ignoring_is_not_untrackable() {
    let corpus = Corpus::new("ignored");
    corpus.ok(&["config", "set", "placement-root", ".jigc"]);

    assert!(
        corpus.exists(".jigc/roadmap.md") && !corpus.exists("docs/roadmap.md"),
        "the move lands — a gitignored home is trackable, just ignored",
    );
    let status = corpus.git(&["status", "--porcelain"]);
    assert!(
        status
            .lines()
            .any(|l| l.starts_with('R') && l.contains(".jigc/roadmap.md")),
        "and it stages as a real rename, not a bare deletion; status:\n{status}",
    );
}

/// **The ordinary re-point is untouched.** The guard is a refusal of two shapes, not a
/// new gate on the door: a plain directory root still detects, routes and moves.
#[test]
fn an_ordinary_root_still_moves_the_doc_it_would_strand() {
    let corpus = Corpus::new("ordinary");
    corpus.ok(&["config", "set", "placement-root", "notes"]);
    assert!(
        corpus.exists("notes/roadmap.md") && !corpus.exists("docs/roadmap.md"),
        "the floor still relocates a stranded committed doc",
    );
    assert!(
        corpus.tracked().iter().any(|p| p == "notes/roadmap.md"),
        "…to a home git actually tracks; tracked:\n{:#?}",
        corpus.tracked(),
    );
}
