//! The adapter's first **owned artifact** — the guides, installed by `jigc setup` as a
//! version-stamped file jigc owns and replaces (M48 Increment 10, T1;
//! `design/assistant-adapter.md` → Generated, minimal, regenerated / The adapter profile;
//! `DECISIONS.md` → 2026-08-13 the Settle, *the adapter's own instruction files*).
//!
//! VISION commits the adapter to *"a small set of skill/command files that each just call
//! the CLI"*, and the install commit carried none — an adopter had no version-matched path
//! to the guides at all (the pre-1.0.0 trial had to seed `QUICKSTART.md`/`MIGRATING.md` into
//! a sibling directory by hand). This suite drives the **real binary** over the mechanism,
//! which is the load-bearing half:
//!
//! (a) `setup` writes the **profile-declared** path, carrying a self-describing header —
//!     the `jigc-version:` stamp convention plus the `blake3` of its own body — and the
//!     recorded hash **equals** the body's;
//! (b) a second `setup` is byte-identical (the `.jigc/AGENT.md` mold: rewritten whole,
//!     no in-file idempotency markers);
//! (c) a **pristine** copy stamped at an older version is replaced and re-stamped — the
//!     *replaced on upgrade* half, observable through the stamp rather than assumed;
//! (d) the install commit **names** it, so what `setup` lists as installed is what its
//!     commit carries;
//! (e) a profile declaring **no** guide target installs nothing and exits 0 — the
//!     omitting-context arm (`implementation/increment-workflow.md` → hardening #5): the
//!     target is optional, and an assistant without one must be inert, never an error;
//! (f) the shipped bytes carry **no in-repo relative link** — every relative link in the
//!     source guides would dangle from an adopter's repo, which is the exact stranding the
//!     trial pre-registered as a finding.
//!
//! No external test crates: the binary comes from `CARGO_BIN_EXE_jigc`, the temp repo is
//! built with `std::fs`, and a self-cleaning `TempDir` keeps this off the real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The Claude Code profile's declared guide target — the path this suite asserts against,
/// stated once here as the fixture's own premise (the profile is the authority; a change
/// there reddens this constant).
const GUIDE_PATH: &str = ".claude/skills/jigc/SKILL.md";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-guide-{tag}-{}-{:?}",
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

/// Make `root` a real git repo with a usable identity — `jigc setup` commits its own
/// install, so the fixture needs one.
fn mark_repo(root: &Path) {
    let ok = Command::new("git")
        .args(["init", "-q"])
        .current_dir(root)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git init")
        .status
        .success();
    assert!(ok, "git init failed");
    for kv in [
        ["user.email", "test@example.com"],
        ["user.name", "Test"],
        ["commit.gpgsign", "false"],
    ] {
        let ok = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["config", kv[0], kv[1]])
            .output()
            .expect("run git config")
            .status
            .success();
        assert!(ok, "git config {} failed", kv[0]);
    }
}

/// Run `git -C <root> <args>` with the ambient config neutralized, returning stdout.
fn git_capture(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .unwrap_or_else(|err| panic!("run git {args:?}: {err}"));
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Run the built `jigc setup` with `cwd = repo`, `$HOME = home`, and an optional
/// `JIGC_ADAPTERS_DIR` override (removed when `None`, so an ambient value never leaks in).
fn run_setup(repo: &Path, home: &Path, adapters_dir: Option<&Path>) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_jigc"));
    cmd.arg("setup")
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_ADAPTERS_DIR");
    if let Some(dir) = adapters_dir {
        cmd.env("JIGC_ADAPTERS_DIR", dir);
    }
    cmd.output().expect("run the jigc binary")
}

/// Assert `jigc setup` exited 0, printing stderr on failure.
fn assert_clean(out: &std::process::Output, label: &str) {
    assert!(
        out.status.success(),
        "{label}: `jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Split an installed artifact into (front-matter body, guide body) — the two halves the
/// stamp relates. Panics with the artifact's own bytes when the shape is wrong, so a
/// header regression reads as one failure rather than a slice panic.
fn split_artifact(text: &str) -> (&str, &str) {
    let rest = text.strip_prefix("---\n").unwrap_or_else(|| {
        panic!("the artifact must open with a `---` front matter; got:\n{text}")
    });
    let close = rest.find("\n---\n\n").unwrap_or_else(|| {
        panic!("the artifact's front matter must close with `---`; got:\n{text}")
    });
    (&rest[..close + 1], &rest[close + "\n---\n\n".len()..])
}

/// The value of a `key: value` line in the front matter.
fn header_value<'a>(front: &'a str, key: &str) -> &'a str {
    front
        .lines()
        .find_map(|line| line.strip_prefix(key))
        .map(str::trim)
        .unwrap_or_else(|| panic!("the front matter must carry `{key}`; got:\n{front}"))
}

/// (a) + (b): `setup` writes the profile-declared path with a self-describing header whose
/// recorded hash **is** its body's, and a second `setup` is byte-identical.
///
/// The hash is asserted against a hash this test computes itself, never against the
/// header's own claim about itself — a header that records some other file's digest, or a
/// generator that stamps before assembling, fails here.
#[test]
fn setup_writes_the_version_stamped_guide_artifact_idempotently() {
    let repo = TempDir::new("stamp");
    mark_repo(repo.path());
    let home = TempDir::new("stamp-home");

    assert_clean(&run_setup(repo.path(), home.path(), None), "first setup");

    let installed = fs::read_to_string(repo.path().join(GUIDE_PATH))
        .unwrap_or_else(|err| panic!("`setup` must write `{GUIDE_PATH}`: {err}"));
    let (front, body) = split_artifact(&installed);

    assert_eq!(
        header_value(front, "jigc-version:"),
        env!("CARGO_PKG_VERSION"),
        "the artifact must be stamped with the running build's version; front matter:\n{front}",
    );
    assert_eq!(
        header_value(front, "jigc-body-blake3:"),
        engine::file_state::hash_bytes(body.as_bytes()),
        "the recorded hash must equal the body's own; front matter:\n{front}",
    );
    assert!(
        !body.trim().is_empty(),
        "the artifact must carry the guides, not an empty body",
    );
    // The guides really are the content — both of them, one file (the deliberate
    // one-artifact bound: a second file is the condition the delta discipline is owed on).
    for marker in ["jigc start", "jigc task finalize", "jigc migrate-corpus"] {
        assert!(
            body.contains(marker),
            "the artifact's body must carry the shipped guides (missing `{marker}`); got:\n{body}",
        );
    }

    // (b) A second setup is byte-identical — rewritten whole, no drifting stamp.
    assert_clean(&run_setup(repo.path(), home.path(), None), "second setup");
    let again = fs::read_to_string(repo.path().join(GUIDE_PATH)).expect("still installed");
    assert_eq!(
        installed, again,
        "a second `jigc setup` must leave the artifact byte-identical",
    );
}

/// (c) The *replaced on upgrade* half, observable through the stamp: a **pristine** copy
/// stamped at an older version — its recorded hash matching its own body, so it is jigc's
/// own artifact and not a user's edit — is replaced and re-stamped at the running build.
#[test]
fn a_pristine_stale_stamped_guide_is_replaced_and_restamped() {
    let repo = TempDir::new("stale");
    mark_repo(repo.path());
    let home = TempDir::new("stale-home");

    // A pristine artifact from an older build: an old body, an old stamp, and a hash that
    // is genuinely this body's — the state a `jigc setup` after a binary upgrade meets.
    let stale_body = "the guides as jigc 0.0.1-old shipped them\n";
    let stale = format!(
        "---\nname: jigc\njigc-version: 0.0.1-old\njigc-body-blake3: {}\n---\n\n{stale_body}",
        engine::file_state::hash_bytes(stale_body.as_bytes()),
    );
    let target = repo.path().join(GUIDE_PATH);
    fs::create_dir_all(target.parent().expect("the artifact has a parent dir"))
        .expect("seed the skill dir");
    fs::write(&target, &stale).expect("seed the stale artifact");

    assert_clean(&run_setup(repo.path(), home.path(), None), "setup");

    let installed = fs::read_to_string(&target).expect("still installed");
    let (front, body) = split_artifact(&installed);
    assert_eq!(
        header_value(front, "jigc-version:"),
        env!("CARGO_PKG_VERSION"),
        "a pristine stale copy must be re-stamped at the running build; got:\n{front}",
    );
    assert_eq!(
        header_value(front, "jigc-body-blake3:"),
        engine::file_state::hash_bytes(body.as_bytes()),
        "the re-stamped hash must equal the replaced body's own; got:\n{front}",
    );
    assert!(
        !installed.contains(stale_body),
        "the stale body must be replaced, not kept; got:\n{installed}",
    );
}

/// (d) The install commit **names** the artifact — what `setup` reports as installed is
/// what its own commit carries, so a clone gets the guides that match the binary that
/// wrote them.
#[test]
fn the_install_commit_carries_the_guide_artifact() {
    let repo = TempDir::new("commit");
    mark_repo(repo.path());
    let home = TempDir::new("commit-home");
    // A born HEAD, so the install commit is the ordinary (non-first-commit) shape.
    fs::write(repo.path().join("README.md"), "hi\n").expect("seed README");
    git_capture(repo.path(), &["add", "README.md"]);
    git_capture(repo.path(), &["commit", "-q", "-m", "initial"]);

    let out = run_setup(repo.path(), home.path(), None);
    assert_clean(&out, "setup");

    let committed = git_capture(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|line| line == GUIDE_PATH),
        "the install commit must carry `{GUIDE_PATH}`; got:\n{committed}",
    );
    // Nothing of the install is left untracked behind the summary that lists it.
    let porcelain = git_capture(repo.path(), &["status", "--porcelain"]);
    assert!(
        !porcelain.contains(".claude/skills"),
        "the guide artifact must not be left untracked; `git status` says:\n{porcelain}",
    );
    // The summary names it on both surfaces the driver reads.
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains(GUIDE_PATH),
        "`jigc setup`'s summary must name the installed guide; got:\n{stdout}",
    );
}

/// (e) The omitting-context arm: a profile declaring **no** guide target installs nothing
/// at that path and exits 0. The target is optional by design — an assistant with no place
/// to put a guide must be *inert* here, never an install error — and a green pass over the
/// one composing profile would hide a required-field regression in every omitting one.
#[test]
fn a_profile_with_no_guide_target_installs_nothing_and_exits_zero() {
    let repo = TempDir::new("omit");
    mark_repo(repo.path());
    let home = TempDir::new("omit-home");

    // The shipped profile minus its `guide:` block, everything else byte-faithful — so
    // this fixture tracks the profile as it evolves rather than re-typing it.
    let shipped = include_str!("../adapters/claude-code.yaml");
    let (head, _) = shipped
        .split_once("\nguide:")
        .expect("the shipped profile declares a `guide:` block");
    let adapters = TempDir::new("omit-adapters");
    fs::write(
        adapters.path().join("claude-code.yaml"),
        format!("{head}\n"),
    )
    .expect("write the guide-less profile");

    let out = run_setup(repo.path(), home.path(), Some(adapters.path()));
    assert_clean(&out, "guide-less setup");
    assert!(
        !repo.path().join(GUIDE_PATH).exists(),
        "a profile declaring no guide target must install no guide",
    );
    assert!(
        !repo.path().join(".claude/skills").exists(),
        "a profile declaring no guide target must not create the skill tree either",
    );
    // The rest of the install is unaffected — inert means inert, not degraded.
    assert!(
        repo.path().join(".jigc/AGENT.md").exists() && repo.path().join("CLAUDE.md").exists(),
        "the guide-less install must still write the rest of the adapter",
    );
    let committed = git_capture(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        !committed.contains(".claude/skills"),
        "a guide-less install must not name a guide in its commit; got:\n{committed}",
    );
}

/// (f) The shipped bytes carry **no in-repo relative link**. Every relative link in the
/// source guides points at a file of the *jigc* repository — `design/storage.md`,
/// `implementation/roadmap.md`, the archived migration method — and dangles from an
/// adopter's tree, which is the stranding the pre-1.0.0 trial pre-registered as a finding.
/// Shipping them unresolved would author a law-1 defect in the same motion that fixes one.
///
/// The check is over the **installed bytes**, not over the generator: it is the file an
/// adopter opens that has to be link-clean.
#[test]
fn the_installed_guide_carries_no_in_repo_relative_link() {
    let repo = TempDir::new("links");
    mark_repo(repo.path());
    let home = TempDir::new("links-home");
    assert_clean(&run_setup(repo.path(), home.path(), None), "setup");

    let installed = fs::read_to_string(repo.path().join(GUIDE_PATH)).expect("installed");
    let mut rest = installed.as_str();
    let mut dangling: Vec<&str> = Vec::new();
    while let Some(at) = rest.find("](") {
        let tail = &rest[at + 2..];
        let end = tail.find(')').unwrap_or(tail.len());
        let target = &tail[..end];
        // Absolute URLs are fine from anywhere; anything else is a path into a repo the
        // adopter does not have.
        if !target.contains("://") && !target.starts_with('#') {
            dangling.push(target);
        }
        rest = &tail[end.min(tail.len())..];
    }
    assert!(
        dangling.is_empty(),
        "the shipped guide must carry no in-repo relative link; found: {dangling:?}",
    );
}
