//! M19 inc-1 / T3 — the end-to-end acceptance: `jigc setup` installs the
//! assistant-neutral warn-only `pre-commit` hook, and a **real** `git commit`
//! fires it against the **real** `doc-code` probe.
//!
//! This is the increment's Deliverable + Proves, exercised through the real binary
//! (`CARGO_BIN_EXE_jigc`) over a throwaway `git init` repo. T3 wires the neutral
//! install step (T2's [`cli::setup::install_precommit_hook`], pointed at the
//! installing `jigc`'s own absolute path via `current_exe`) into `setup::install`,
//! then drives genuine `git commit`s so the installed hook actually runs:
//!
//! - **(a)** a committed store with a valid code-anchor, the cited symbol renamed →
//!   `git commit` **succeeds** (warn-only) AND the doc↔code drift warning appears in
//!   the commit output;
//! - **(b)** a clean store → commit succeeds **silently** (no warning);
//! - **(c)** an unconfigured repo (no `.jigc/config/` layer) → commit succeeds
//!   silently, never broken;
//! - **(d)** `jigc setup` run twice leaves the `pre-commit` **byte-identical**
//!   (idempotent);
//! - **(e)** a repo with a pre-existing `pre-commit` keeps it after `setup`
//!   (non-destructive).
//!
//! The hook embeds the installing `jigc`'s absolute path (the stale-binary hazard);
//! the hook in turn runs `jigc validate`, which selects the real `doc-code` probe via
//! the `JIGC_DOC_CODE_PROBE` dev/test knob (the `validate_command.rs` idiom). That env
//! var is inherited by `git commit` → the hook → `jigc validate`, so the commit-time
//! sweep drives the genuine subprocess exactly as production would.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-precommit-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path —
/// the **real** tree-sitter subprocess the commit-time sweep drives, never a mock
/// (the `validate_command.rs` idiom).
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("pack")
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
}

/// Run a `git` command in `repo`, asserting success. Pins config to `/dev/null` so a
/// developer's global git config never leaks into the throwaway repo.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run `git commit -m <msg>` in `repo` with the real `doc-code` probe selected via
/// `JIGC_DOC_CODE_PROBE` (inherited by the hook → `jigc validate`). Returns the full
/// `Output` so a caller can assert exit status + the merged hook output.
fn git_commit(repo: &Path, msg: &str) -> std::process::Output {
    Command::new("git")
        .args(["commit", "-m", msg])
        .current_dir(repo)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run git commit")
}

/// Run the real `jigc` binary with `cwd = repo`, capturing output.
fn jigc(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run the jigc binary")
}

/// A committed `adr` citing `<rel>#<symbol>` from its `cites-code` header anchor
/// (the dev pack's `code-anchor` field type → `doc-code/symbol-exists`).
fn adr(rel: &str, symbol: &str) -> String {
    format!(
        "---\n\
         status: accepted\n\
         date: 2026-06-13\n\
         cites-code: {rel}#{symbol}\n\
         ---\n\
         \n\
         # The cache decision\n\
         \n\
         ## Context\n\
         Forces.\n\
         \n\
         ## Decision\n\
         Decided.\n\
         \n\
         ## Consequences\n\
         Effects.\n"
    )
}

/// Initialize a real git repo + identity, with the real doc-code probe seeded but **no**
/// `.jigc/config/` project layer yet — the caller decides whether to set it up.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
}

/// Seed a committed store with a valid code-anchor: an `adr` citing `evict_lru` in
/// `crates/engine/src/cache.rs`. Establishes a first commit so a later HEAD exists.
fn seed_store_with_anchor(repo: &Path) {
    let cache = repo.join("crates/engine/src/cache.rs");
    fs::create_dir_all(cache.parent().unwrap()).expect("mk code dir");
    fs::write(&cache, "pub fn evict_lru() {}\nfn helper() {}\n").expect("write cache.rs");

    fs::create_dir_all(repo.join("decisions")).expect("mk decisions");
    fs::write(
        repo.join("decisions/cache.md"),
        adr("crates/engine/src/cache.rs", "evict_lru"),
    )
    .expect("write adr");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed store"]);
}

/// Mark this project as set up: the `.jigc/config/` cascade layer the `jigc validate`
/// locate-preamble gate requires. Without it, `jigc validate` reports not-a-project.
fn mark_set_up(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// The repo-relative path of the installed `pre-commit` hook in a default repo.
fn hook_path(repo: &Path) -> PathBuf {
    repo.join(".git/hooks/pre-commit")
}

/// The human-readable warning fragment the hook prints on a doc↔code content finding
/// (the B2 contract's emitted text).
const DRIFT_WARNING: &str = "doc<->code drift detected";

/// (a) The headline: a committed store with a valid code-anchor whose cited symbol is
/// then **renamed** → `git commit` SUCCEEDS (warn-only) AND the doc↔code drift warning
/// appears in the commit output. The backstop fires automatically with no agent action.
#[test]
fn commit_warns_on_stale_anchor_but_succeeds() {
    let repo = TempDir::new("stale");
    init_repo(repo.path());
    seed_store_with_anchor(repo.path());
    mark_set_up(repo.path());

    // Install the hook through the REAL `jigc setup` binary.
    let setup = jigc(repo.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    assert!(
        hook_path(repo.path()).is_file(),
        "`jigc setup` must install the pre-commit hook",
    );

    // Rename the cited symbol: the committed adr still cites `evict_lru`, now gone.
    fs::write(
        repo.path().join("crates/engine/src/cache.rs"),
        "pub fn evicted() {}\nfn helper() {}\n",
    )
    .expect("rename the cited symbol");

    // Commit the rename: the hook runs `jigc validate`, finds the stale anchor.
    git(repo.path(), &["add", "."]);
    let out = git_commit(repo.path(), "rename the cited symbol");
    let merged = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    assert!(
        out.status.success(),
        "the warn-only hook must NOT block the commit; output:\n{merged}",
    );
    assert!(
        merged.contains(DRIFT_WARNING),
        "the doc<->code drift warning must appear in the commit output; output:\n{merged}",
    );
    // The commit landed despite the warning.
    let head = Command::new("git")
        .args(["log", "--oneline", "-1"])
        .current_dir(repo.path())
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("git log");
    assert!(
        String::from_utf8_lossy(&head.stdout).contains("rename the cited symbol"),
        "the commit must have landed (warn-only): {}",
        String::from_utf8_lossy(&head.stdout),
    );
}

/// (b) A clean store (every cited symbol exists) → `git commit` succeeds **silently**:
/// no drift warning in the commit output.
#[test]
fn commit_is_silent_on_clean_store() {
    let repo = TempDir::new("clean");
    init_repo(repo.path());
    seed_store_with_anchor(repo.path());
    mark_set_up(repo.path());

    let setup = jigc(repo.path(), &["setup"]);
    assert!(setup.status.success(), "`jigc setup` must succeed");

    // A trivial, anchor-irrelevant change; the cited symbol still exists.
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    git(repo.path(), &["add", "."]);
    let out = git_commit(repo.path(), "clean change");
    let merged = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    assert!(
        out.status.success(),
        "a clean store must commit cleanly; output:\n{merged}",
    );
    assert!(
        !merged.contains(DRIFT_WARNING),
        "a clean store must warn nothing; output:\n{merged}",
    );
}

/// (c) An unconfigured repo (no `.jigc/config/` layer) → the hook's `jigc validate`
/// reports not-a-project (non-zero, on stderr), so the hook warns nothing and exits 0:
/// the commit succeeds **silently**, never broken. The hook keys on findings, never the
/// exit code (the exit code is wrong-way-round).
#[test]
fn commit_is_silent_on_unconfigured_repo() {
    let repo = TempDir::new("unconfigured");
    init_repo(repo.path());

    // Install the hook over a NOT-set-up repo. `jigc setup` itself sets up the project
    // layer, so to test the unconfigured path we install the hook directly via setup and
    // then commit — but setup marks the project set up. Instead: seed a file, install the
    // hook, then REMOVE the project layer to simulate an unconfigured repo at commit time.
    fs::write(repo.path().join("file.txt"), "x\n").expect("seed a file");
    let setup = jigc(repo.path(), &["setup"]);
    assert!(setup.status.success(), "`jigc setup` must succeed");
    assert!(
        hook_path(repo.path()).is_file(),
        "the hook must be installed",
    );
    // Tear the project layer back down: the repo is now unconfigured at commit time.
    fs::remove_dir_all(repo.path().join(".jigc")).expect("remove project layer");

    git(repo.path(), &["add", "."]);
    let out = git_commit(repo.path(), "first commit, unconfigured");
    let merged = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    assert!(
        out.status.success(),
        "an unconfigured repo must commit silently, never broken; output:\n{merged}",
    );
    assert!(
        !merged.contains(DRIFT_WARNING),
        "an unconfigured repo must warn nothing; output:\n{merged}",
    );
}

/// (d) `jigc setup` run twice leaves the `pre-commit` **byte-identical** (idempotent —
/// the jigc block is regenerated, never appended twice).
#[test]
fn setup_twice_leaves_hook_byte_identical() {
    let repo = TempDir::new("idempotent");
    init_repo(repo.path());

    let first_setup = jigc(repo.path(), &["setup"]);
    assert!(
        first_setup.status.success(),
        "first `jigc setup` must succeed"
    );
    let first = fs::read_to_string(hook_path(repo.path())).expect("hook written");

    let second_setup = jigc(repo.path(), &["setup"]);
    assert!(
        second_setup.status.success(),
        "second `jigc setup` must succeed",
    );
    let second = fs::read_to_string(hook_path(repo.path())).expect("hook still present");

    assert_eq!(
        first, second,
        "a re-run of `jigc setup` must leave the pre-commit byte-identical",
    );
}

/// (e) A repo with a pre-existing `pre-commit` keeps it after `setup` (non-destructive):
/// the foreign hook content is preserved verbatim and the jigc block is appended.
#[test]
fn setup_preserves_existing_precommit() {
    let repo = TempDir::new("preexisting");
    init_repo(repo.path());

    fs::create_dir_all(repo.path().join(".git/hooks")).expect("mk hooks dir");
    let existing = "#!/bin/sh\n# someone's hand-rolled hook\necho hello\nexit 0\n";
    fs::write(hook_path(repo.path()), existing).expect("seed a pre-existing pre-commit");

    let setup = jigc(repo.path(), &["setup"]);
    assert!(setup.status.success(), "`jigc setup` must succeed");

    let after = fs::read_to_string(hook_path(repo.path())).expect("hook still present");
    assert!(
        after.contains(existing),
        "the pre-existing hook content must be preserved verbatim; after:\n{after}",
    );
    assert!(
        after.contains("jigc-managed pre-commit hook"),
        "the wrapped hook must also carry the jigc block; after:\n{after}",
    );
}
