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
//!
//! The T3 up-front validation gate (collision blocks · no-op reslug → retitle-only ·
//! mid-fan-out guard) rides the same committed store:
//! - **collision (#3):** a rename onto a *different* existing slug blocks (exit != 0, the
//!   collision message), the store untouched.
//! - **no-op reslug → retitle-only:** a new title that slugs to the doc's *own* current
//!   slug rewrites the H1 + commits once, with **no** `git mv` and **no** referrer edit.
//! - **mid-fan-out guard (#4):** an active task working area **or** an in-flight milestone
//!   blocks the rename (the marker present case); its absence is the happy path proceeding
//!   (the marker-absent case — increment-workflow #5, the guard proven not inert).

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

/// Collision (#3): a rename whose new title slugs to a **different** existing doc's slug
/// blocks (exit != 0, the collision message), leaving the store untouched — an identity
/// refactor never silently suffixes onto a live doc.
#[test]
fn rename_onto_a_different_existing_slug_blocks() {
    let repo = TempDir::new("collision");
    seed_store(repo.path());
    let before_count = commit_count(repo.path());

    // "Keeper" slugs to `keeper`, the third adr's existing slug — a collision.
    let out = jigc(
        repo.path(),
        &["rename", "adr:single-node-cache", "--to", "Keeper"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a rename onto a different existing slug must block; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("a different doc already exists"),
        "the block must carry the collision message; stderr:\n{stderr}",
    );

    // The store is untouched: the target keeps its slug, the colliding doc is unchanged,
    // no new commit landed.
    let decisions = repo.path().join("docs/decisions");
    assert!(
        decisions.join("single-node-cache.md").is_file(),
        "the blocked rename must leave the target at its old slug",
    );
    assert!(
        fs::read_to_string(decisions.join("keeper.md"))
            .unwrap()
            .contains("# Keeper"),
        "the colliding doc must be untouched",
    );
    assert_eq!(
        commit_count(repo.path()),
        before_count,
        "a blocked rename must land no commit",
    );
}

/// No-op reslug → retitle-only: a new title that slugs to the doc's **own** current slug
/// (`single-node-cache`) rewrites the H1 + commits once, with **no** `git mv` and **no**
/// referrer edit (identity unchanged, so nothing dangles).
#[test]
fn noop_reslug_degrades_to_a_retitle_only() {
    let repo = TempDir::new("retitle");
    seed_store(repo.path());
    let before_count = commit_count(repo.path());

    // "Single Node Cache" slugs to `single-node-cache` — the doc's own current slug.
    let out = jigc(
        repo.path(),
        &[
            "rename",
            "adr:single-node-cache",
            "--to",
            "Single Node Cache",
        ],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "a no-op reslug must succeed as a retitle-only; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // No file move: the doc stays at its old slug; no new slug was minted.
    let decisions = repo.path().join("docs/decisions");
    assert!(
        decisions.join("single-node-cache.md").is_file(),
        "a retitle-only must not move the file",
    );

    // The H1 was rewritten to the new title.
    let moved = fs::read_to_string(decisions.join("single-node-cache.md")).unwrap();
    assert!(
        moved.contains("# Single Node Cache") && !moved.contains("# Single node cache"),
        "the H1 must be rewritten to the new title; got:\n{moved}",
    );

    // No referrer edit: both referrers still point at the unchanged slug.
    let superseder = fs::read_to_string(decisions.join("revisit-caching.md")).unwrap();
    assert!(
        superseder.contains("supersedes: adr:single-node-cache"),
        "a retitle-only must not edit referrers; got:\n{superseder}",
    );
    let arch = fs::read_to_string(repo.path().join("docs/architecture/cache-layer.md")).unwrap();
    assert!(
        arch.contains("cites: [adr:single-node-cache, adr:keeper]"),
        "a retitle-only must not edit referrers; got:\n{arch}",
    );

    // Exactly one new commit.
    assert_eq!(
        commit_count(repo.path()),
        before_count + 1,
        "a retitle-only must emit exactly one commit",
    );
}

/// Mid-fan-out guard (#4): an active task working area blocks the rename. Pairs with the
/// happy path (no marker → proceeds) to prove the guard is not inert (increment-workflow
/// #5).
#[test]
fn mid_fan_out_active_task_blocks() {
    let repo = TempDir::new("fanout-task");
    seed_store(repo.path());
    let before_count = commit_count(repo.path());

    // An in-flight task working area — the by-task-id join key the rename would invalidate.
    fs::create_dir_all(repo.path().join(".jigc/tasks/some-task")).expect("mk task area");

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
        "a rename mid-fan-out (active task) must block; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("in flight"),
        "the block must carry the mid-fan-out message; stderr:\n{stderr}",
    );
    assert!(
        repo.path()
            .join("docs/decisions/single-node-cache.md")
            .is_file()
            && !repo
                .path()
                .join("docs/decisions/distributed-cache.md")
                .exists(),
        "a blocked rename must leave the store untouched",
    );
    assert_eq!(
        commit_count(repo.path()),
        before_count,
        "a blocked rename must land no commit",
    );
}

/// Mid-fan-out guard (#4): an in-flight milestone blocks the rename, the second marker the
/// coarse guard keys on (separate from the active-task case above).
#[test]
fn mid_fan_out_in_flight_milestone_blocks() {
    let repo = TempDir::new("fanout-milestone");
    seed_store(repo.path());
    let before_count = commit_count(repo.path());

    // An in-flight milestone dir — the second mid-fan-out marker.
    fs::create_dir_all(repo.path().join(".jigc/milestones/some-milestone")).expect("mk milestone");

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
        "a rename mid-fan-out (milestone) must block; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("in flight"),
        "the block must carry the mid-fan-out message; stderr:\n{stderr}",
    );
    assert_eq!(
        commit_count(repo.path()),
        before_count,
        "a blocked rename must land no commit",
    );
}

/// An `arch-doc` that **cites** the target through a ref-field *and* mentions the old slug
/// in its **slot prose** (the Overview line). The rename rewrites the structured `cites`
/// ref but, by the determinism boundary, never the prose — it only *reports* it.
const ARCH_DOC_WITH_PROSE: &str = "\
---
cites: [adr:single-node-cache, adr:keeper]
schema-version: 1
---

# Cache layer

## Overview

The single-node-cache decision shapes this subsystem.

## Components
";

/// An unmanaged `README.md`: line 3 mentions the old slug as a standalone token (reported),
/// line 4 carries a `caches`-style near-match (`single-node-caches`) that the word-boundary
/// matcher must NOT report.
const README: &str = "\
# Project

See the single-node-cache ADR for the caching rationale.
We considered single-node-caches but kept the singular form.
";

/// An unmanaged source file whose comment mentions the old slug (line 1, reported).
const SRC_CACHE: &str = "\
// single-node-cache: legacy module name kept for back-compat.
pub fn cache() {}
";

/// Seed a store whose old slug additionally appears in an arch-doc's slot prose, a
/// `README.md`, and a source comment — plus a `caches`-style near-match in the README.
fn seed_store_with_prose(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    let decisions = repo.join("docs/decisions");
    fs::create_dir_all(&decisions).expect("mk decisions");
    fs::write(decisions.join("single-node-cache.md"), ADR_TARGET).expect("write target");
    fs::write(decisions.join("keeper.md"), ADR_KEEPER).expect("write keeper");

    let architecture = repo.join("docs/architecture");
    fs::create_dir_all(&architecture).expect("mk architecture");
    fs::write(architecture.join("cache-layer.md"), ARCH_DOC_WITH_PROSE).expect("write arch-doc");

    fs::write(repo.join("README.md"), README).expect("write README");
    let src = repo.join("src");
    fs::create_dir_all(&src).expect("mk src");
    fs::write(src.join("cache.rs"), SRC_CACHE).expect("write src");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc/config")).expect("create project layer");
}

/// Advisory prose/unmanaged-mention report (#2, the honest line): the old slug appears in an
/// arch-doc's slot prose, a `README.md`, and a source comment — the rename **reports** all
/// three in the verb output, **rewrites none** of them, the report **never** changes exit
/// status, and a `caches`-style near-match is **not** reported. The word-boundary
/// discrimination is *proven* (a naive substring scan would over-report the near-match,
/// yielding 4 mentions, not 3) — not asserted.
#[test]
fn advisory_report_lists_prose_and_unmanaged_mentions_word_bounded() {
    let repo = TempDir::new("advisory");
    seed_store_with_prose(repo.path());

    // Pre-rename bytes of every file the report must NOT rewrite.
    let arch_path = repo.path().join("docs/architecture/cache-layer.md");
    let readme_path = repo.path().join("README.md");
    let src_path = repo.path().join("src/cache.rs");
    let before_readme = fs::read(&readme_path).unwrap();
    let before_src = fs::read(&src_path).unwrap();

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

    // Advisory: the prose report never changes exit status — the rename still exits 0.
    assert!(
        out.status.success(),
        "the advisory report must not change the rename's exit status; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // All three mentions are listed in the verb output (path:line each).
    assert!(
        stdout.contains("docs/architecture/cache-layer.md:"),
        "the arch-doc slot-prose mention must be reported; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("README.md:3"),
        "the README mention (line 3) must be reported; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("src/cache.rs:1"),
        "the source-comment mention (line 1) must be reported; stdout:\n{stdout}",
    );

    // Word-boundary discrimination: the README's `single-node-caches` near-match (line 4)
    // is NOT reported. A naive substring scan would over-count to 4 mentions; the report
    // states exactly 3, so the near-match is genuinely excluded (proven, not asserted).
    assert!(
        stdout.contains("3 prose/unmanaged mention"),
        "exactly 3 mentions (the near-match excluded) must be reported; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("README.md:4"),
        "the `single-node-caches` near-match (line 4) must NOT be reported; stdout:\n{stdout}",
    );

    // Rewrites none: the structured `cites` ref repointed, but the prose + unmanaged files
    // are byte-untouched.
    let arch = fs::read_to_string(&arch_path).unwrap();
    assert!(
        arch.contains("cites: [adr:distributed-cache, adr:keeper]"),
        "the structured cites ref must still repoint old -> new; got:\n{arch}",
    );
    assert!(
        arch.contains("The single-node-cache decision shapes this subsystem."),
        "the arch-doc slot prose must be left untouched (reported, never rewritten); got:\n{arch}",
    );
    assert_eq!(
        fs::read(&readme_path).unwrap(),
        before_readme,
        "the README must be left byte-untouched (advisory report, never a rewrite)",
    );
    assert_eq!(
        fs::read(&src_path).unwrap(),
        before_src,
        "the source file must be left byte-untouched (advisory report, never a rewrite)",
    );
}
