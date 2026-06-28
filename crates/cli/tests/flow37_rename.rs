//! M35 Increment 1 — flow 37, the `jigc rename` verb + its committed-in-place atomic
//! transaction (T2, the risk spike). Drives the **real** `jigc` binary over a committed
//! store, so the bytes an operator would actually run are the contract (the
//! invocation-path-masking lesson).
//!
//! The store: an `adr` (`docs/decisions/single-node-cache.md`) that is **superseded by**
//! a second `adr` (`supersedes: adr:single-node-cache`, scalar ref) and **cited by** an
//! `arch-doc` (`cites: [adr:single-node-cache, adr:keeper]`, list ref with a sibling). A
//! third `adr` (`keeper`) keeps the cites sibling resolvable so `jigc validate` stays
//! clean.
//!
//! - **happy path (#1, #2-boundary):** `jigc rename adr:single-node-cache --to
//!   "Distributed cache"` exits 0, emits **one** commit, repoints **both** referrers
//!   (the scalar `supersedes` and the list `cites` — whole-list re-emit, the `keeper`
//!   sibling + order byte-preserved), rewrites the moved doc's H1, `git mv`s the file
//!   (`git log --follow` survives), and a follow-up `jigc validate` exits 0 with **zero**
//!   `ref-resolves` findings.
//! - **forced pre-commit failure (#5, RED):** a rejecting `pre-commit` hook makes the
//!   commit fail → the store is left **byte-identical to pre-rename** (old slug present,
//!   new slug absent, referrers untouched, the index clean, no new commit). Proven by
//!   driving the interrupted transaction, not asserted.

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
            "jigc-rename-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) so the `jigc validate` pre-flight
/// resolves (the `validate_command.rs` idiom).
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
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

/// Run a `git` command in `repo`, asserting success and returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Run `jigc <args>` with `cwd = repo`, the real `doc-code` probe selected via
/// `JIGC_DOC_CODE_PROBE`.
fn jigc(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// The `adr` being renamed — H1 "Single node cache", slug `single-node-cache`.
const ADR_TARGET: &str = "\
---
status: accepted
date: 2026-06-28
schema-version: 1
---

# Single node cache

## Context

Lookups must stay fast.

## Decision

Cache on one node.

## Consequences

A cold node loses cache.
";

/// A second `adr` that supersedes the target via a **scalar** `supersedes` ref.
const ADR_SUPERSEDER: &str = "\
---
status: accepted
date: 2026-06-28
supersedes: adr:single-node-cache
schema-version: 1
---

# Revisit caching

## Context

The single node is a bottleneck.

## Decision

Revisit the caching approach.

## Consequences

Migration needed.
";

/// A third `adr` kept only so the `cites` list sibling stays resolvable.
const ADR_KEEPER: &str = "\
---
status: accepted
date: 2026-06-28
schema-version: 1
---

# Keeper

## Context

Context.

## Decision

Decided.

## Consequences

Effects.
";

/// An `arch-doc` that cites the target through a **list** `cites` ref carrying a sibling
/// (`adr:keeper`) — the whole-list re-emit must preserve the sibling + order.
const ARCH_DOC: &str = "\
---
cites: [adr:single-node-cache, adr:keeper]
schema-version: 1
---

# Cache layer

## Overview

The caching subsystem.

## Components
";

/// Seed a real git repo with a committed store + the `.jigc/config/` project layer.
fn seed_store(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    let decisions = repo.join("docs/decisions");
    fs::create_dir_all(&decisions).expect("mk decisions");
    fs::write(decisions.join("single-node-cache.md"), ADR_TARGET).expect("write target");
    fs::write(decisions.join("revisit-caching.md"), ADR_SUPERSEDER).expect("write superseder");
    fs::write(decisions.join("keeper.md"), ADR_KEEPER).expect("write keeper");

    let architecture = repo.join("docs/architecture");
    fs::create_dir_all(&architecture).expect("mk architecture");
    fs::write(architecture.join("cache-layer.md"), ARCH_DOC).expect("write arch-doc");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc/config")).expect("create project layer");
}

/// `git rev-list --count HEAD` — the commit count.
fn commit_count(repo: &Path) -> usize {
    git(repo, &["rev-list", "--count", "HEAD"])
        .parse()
        .expect("commit count parses")
}

/// Happy path (#1, #2-boundary): the rename repoints both referrers, rewrites the H1,
/// `git mv`s, commits once, and leaves a store `jigc validate` reports clean.
#[test]
fn rename_repoints_referrers_moves_file_and_commits_once() {
    let repo = TempDir::new("happy");
    seed_store(repo.path());
    let before = commit_count(repo.path());

    let out = jigc(
        repo.path(),
        &[
            "rename",
            "adr:single-node-cache",
            "--to",
            "Distributed cache",
        ],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "`jigc rename` must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // The file moved: new slug present, old slug gone.
    let decisions = repo.path().join("docs/decisions");
    assert!(
        decisions.join("distributed-cache.md").is_file(),
        "the renamed doc must land at its new slug",
    );
    assert!(
        !decisions.join("single-node-cache.md").exists(),
        "the old slug must be gone after the move",
    );

    // The H1 was rewritten to the new title.
    let moved = fs::read_to_string(decisions.join("distributed-cache.md")).unwrap();
    assert!(
        moved.contains("# Distributed cache") && !moved.contains("# Single node cache"),
        "the moved doc's H1 must be the new title; got:\n{moved}",
    );
    // The schema-version stamp is preserved (rename is not a migration).
    assert!(
        moved.contains("schema-version: 1"),
        "the schema-version stamp must be preserved; got:\n{moved}",
    );

    // The scalar `supersedes` referrer repointed old -> new.
    let superseder = fs::read_to_string(decisions.join("revisit-caching.md")).unwrap();
    assert!(
        superseder.contains("supersedes: adr:distributed-cache")
            && !superseder.contains("single-node-cache"),
        "the scalar supersedes ref must repoint old -> new; got:\n{superseder}",
    );

    // The list `cites` referrer repointed only the one element — the sibling + order +
    // separator are byte-preserved (the whole-list re-emit contract).
    let arch = fs::read_to_string(repo.path().join("docs/architecture/cache-layer.md")).unwrap();
    assert!(
        arch.contains("cites: [adr:distributed-cache, adr:keeper]")
            && !arch.contains("single-node-cache"),
        "the list cites ref must swap only the one element, preserving the keeper sibling \
         and order; got:\n{arch}",
    );

    // Exactly one new commit.
    assert_eq!(
        commit_count(repo.path()),
        before + 1,
        "`jigc rename` must emit exactly one commit",
    );

    // `git log --follow` survives the move (the H1 one-liner stays within similarity).
    let follow = git(
        repo.path(),
        &[
            "log",
            "--follow",
            "--format=%H",
            "--",
            "docs/decisions/distributed-cache.md",
        ],
    );
    assert!(
        follow.lines().count() >= 2,
        "git log --follow must trace the renamed doc through the move; got:\n{follow}",
    );

    // The store is clean: `jigc validate` exits 0 with zero ref-resolves findings.
    let v = jigc(repo.path(), &["validate"]);
    let vout = String::from_utf8_lossy(&v.stdout);
    let verr = String::from_utf8_lossy(&v.stderr);
    assert!(
        v.status.success(),
        "`jigc validate` must exit 0 after the rename; stdout:\n{vout}\nstderr:\n{verr}",
    );
    assert!(
        !vout.contains("ref-resolves"),
        "no dangling ref may remain after the rename; stdout:\n{vout}",
    );
}

/// Forced pre-commit failure (#5, RED): a rejecting `pre-commit` hook makes the commit
/// fail → `rollback_rename` restores the store byte-identical to pre-rename. Proven by
/// driving the interrupted transaction, not asserted.
#[test]
fn rename_rolls_back_byte_identical_on_precommit_failure() {
    let repo = TempDir::new("rollback");
    seed_store(repo.path());

    // A rejecting `pre-commit` hook — the user's hook is policy and the verb never
    // bypasses it (`--no-verify` is forbidden), so the commit fails.
    let hooks = repo.path().join(".git/hooks");
    fs::create_dir_all(&hooks).expect("mk hooks");
    let hook = hooks.join("pre-commit");
    fs::write(&hook, "#!/bin/sh\nexit 1\n").expect("write rejecting hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }

    // Pre-rename snapshot of every managed doc + the commit count.
    let target = repo.path().join("docs/decisions/single-node-cache.md");
    let superseder = repo.path().join("docs/decisions/revisit-caching.md");
    let arch = repo.path().join("docs/architecture/cache-layer.md");
    let before_target = fs::read(&target).unwrap();
    let before_superseder = fs::read(&superseder).unwrap();
    let before_arch = fs::read(&arch).unwrap();
    let before_count = commit_count(repo.path());

    let out = jigc(
        repo.path(),
        &[
            "rename",
            "adr:single-node-cache",
            "--to",
            "Distributed cache",
        ],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a rejected pre-commit hook must fail the rename; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // The store is byte-identical to before: old slug present + unchanged, new slug
    // absent, both referrers untouched.
    assert!(
        target.is_file() && fs::read(&target).unwrap() == before_target,
        "the old slug must be restored byte-identical (no half-moved file)",
    );
    assert!(
        !repo
            .path()
            .join("docs/decisions/distributed-cache.md")
            .exists(),
        "no new slug may be left behind by a rolled-back rename",
    );
    assert_eq!(
        fs::read(&superseder).unwrap(),
        before_superseder,
        "the supersedes referrer must be restored byte-identical",
    );
    assert_eq!(
        fs::read(&arch).unwrap(),
        before_arch,
        "the cites referrer must be restored byte-identical",
    );

    // No commit landed, and the index is clean — the rollback unstaged everything.
    assert_eq!(
        commit_count(repo.path()),
        before_count,
        "a rolled-back rename must land no commit",
    );
    // No managed doc is left staged or modified — the rollback unstaged the `git mv` and
    // reverted every referrer edit (the gitignored `.jigc/` caches are not the store and
    // are normally ignored by `jigc setup`).
    let status = git(repo.path(), &["status", "--porcelain"]);
    let doc_entries: Vec<&str> = status
        .lines()
        .filter(|line| line.contains("docs/"))
        .collect();
    assert!(
        doc_entries.is_empty(),
        "a rolled-back rename must leave no managed doc staged or modified; got:\n{status}",
    );
}
