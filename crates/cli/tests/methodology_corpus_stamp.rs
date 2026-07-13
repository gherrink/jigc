//! Acceptance — the **v0→stamped methodology corpus migration** through the real binary
//! (M40 Increment 5, T5; `design/corpus-migration.md` → M40 revises the dichotomy / the
//! stamp-absent v0 arm).
//!
//! The methodology pack's persisted doctypes joined the frozen manifest set at M40 A1
//! (stamp injection live), so an **existing committed methodology corpus authored before
//! the stamp existed** — the real keeper corpora — is a genuine stamp-absent (v0)
//! migration target. This drives `jigc migrate-corpus` on the shipped binary over a
//! fixture spanning **both home kinds**:
//!
//!   - a **placement singleton** (`roadmap` at the literal `docs/roadmap.md`) — a
//!     header-less doc, so the stamp *introduces* the front-matter fence; the placement
//!     arm had never run in-place before M40 (pre-fix, `resolve_migration_homes` treated
//!     every placement doctype as *relocated* and skipped one with no prior snapshot);
//!   - a **location doctype** (`research` under `docs/research/`) — header-bearing, so
//!     the stamp is appended into the existing header.
//!
//! Asserted per the stamp-absent arm's contract (`from = strip_stamp(to)`, the stamp
//! flips last, bytes otherwise unchanged): the migrated bytes are **exactly** the v0
//! bytes plus the stamp — nothing else moves — the transient `commit` shadow migrates
//! nothing, and a re-run reports both docs already current, byte-untouched.
//!
//! Not asserted here: the detect side (this suite's contract is the migrating verb's bytes).
//! The bound it used to record — *"the family-5 sweep walks only `location:`-bearing schemas,
//! so the unstamped placement roadmap is verb-migratable but not `validate`-routed"* — is
//! **gone** as of M42 Inc-2 T1: family 5 enumerates through `index::committed_instances`, so a
//! placement doc is detected and routed like any other (`design/validation.md` → "every
//! committed instance"). Its detect↔fix loop is covered on the real binary in
//! `corpus_migration.rs` → `validate_detects_the_stale_placement_changelog_then_migrate_clears_it`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-methodology-stamp-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home` on the shipped binary.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Make `repo` a real git repo, then run a **normal** `jigc setup` over it — which
/// writes the `compose-embedded-methodology` marker, so the composed `[dev ▸
/// methodology]` pack-set (both manifests, both frozen sets) governs every later verb.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// The **v0 (unstamped) roadmap** at its literal placement home — the byte form a real
/// pre-M40 keeper corpus has: header-less (no front-matter fence at all; the stamp is
/// what introduces it), the canonical minted skeleton otherwise.
const ROADMAP_V0: &str = "\
# Roadmap

## Milestones
";

/// The **v0 (unstamped) research doc** — header-bearing (the `meta` header carries the
/// on-create `date`), authored prose in all three slots, no `schema-version` line.
const RESEARCH_V0: &str = "\
---
date: 2026-07-10
---

# Cache Strategy

## Question

What cache strategy fits the session store.

## Findings

An in-memory single node wins on latency.

## Sources

Prior art in the issue tracker.
";

/// Commit the unstamped v0 methodology corpus: the placement singleton at its literal
/// file, the location doctype under `docs-root`.
fn commit_v0_corpus(repo: &Path) {
    fs::create_dir_all(repo.join("docs/research")).expect("mk docs/research/");
    fs::write(repo.join("docs/roadmap.md"), ROADMAP_V0).expect("write roadmap");
    fs::write(repo.join("docs/research/cache-strategy.md"), RESEARCH_V0).expect("write research");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed v0 methodology corpus"]);
}

/// Run `jigc migrate-corpus --format json` and parse the report.
fn migrate_json(repo: &Path, home: &Path) -> serde_json::Value {
    let out = jigc(repo, home, &["migrate-corpus", "--format", "json"]);
    assert_ok(&out, "`jigc migrate-corpus`");
    serde_json::from_slice(&out.stdout).expect("migrate-corpus --format json emits valid JSON")
}

/// The report's string array under `key`, as owned strings.
fn paths(report: &serde_json::Value, key: &str) -> Vec<String> {
    report[key]
        .as_array()
        .unwrap_or_else(|| panic!("`{key}` is an array; got: {report}"))
        .iter()
        .map(|v| {
            // `blocked` entries are `(path, route)` pairs; the others are bare paths.
            v.as_str()
                .map(str::to_string)
                .unwrap_or_else(|| v[0].as_str().expect("path").to_string())
        })
        .collect()
}

/// The M40 stamp-absent (v0) arm over the methodology corpus, on the real binary:
/// `jigc migrate-corpus` stamps **both home kinds** deterministically — the placement
/// singleton in place at its literal file (the fence introduced on the header-less
/// doc), the location doctype in place under `docs-root` (the stamp appended into the
/// existing header) — with the bytes otherwise **exactly** unchanged, the transient
/// `commit` shadow untouched (it has no persisted home, so the exact-set report proves
/// nothing else was walked or written), and a re-run reporting both already current.
#[test]
fn migrate_corpus_stamps_the_unstamped_methodology_corpus_across_both_home_kinds() {
    let repo = TempDir::new("v0");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v0_corpus(repo.path());

    // MIGRATE — the exact migrated set is the two seeded docs, nothing blocked, nothing
    // else touched (the transient `commit` manifest entry contributes no candidate).
    let report = migrate_json(repo.path(), home.path());
    assert_eq!(
        paths(&report, "migrated"),
        vec![
            "docs/research/cache-strategy.md".to_string(),
            "docs/roadmap.md".to_string(),
        ],
        "exactly the two v0 methodology docs migrate (placement + location); got: {report}",
    );
    assert!(
        paths(&report, "blocked").is_empty() && paths(&report, "already_current").is_empty(),
        "a pure stamp add blocks nothing and skips nothing; got: {report}",
    );

    // The placement singleton, in place at its literal file: the stamp INTRODUCES the
    // front-matter fence on the header-less doc — every other byte unchanged.
    let roadmap = fs::read_to_string(repo.path().join("docs/roadmap.md")).expect("read roadmap");
    assert_eq!(
        roadmap,
        format!("---\nschema-version: 1\n---\n\n{ROADMAP_V0}"),
        "the migrated roadmap is the v0 bytes plus the introduced fence, nothing else",
    );

    // The location doctype: the stamp is APPENDED into the existing header — every
    // other byte (the date, the H1, the authored prose) unchanged.
    let research = fs::read_to_string(repo.path().join("docs/research/cache-strategy.md"))
        .expect("read research");
    assert_eq!(
        research,
        RESEARCH_V0.replacen(
            "date: 2026-07-10\n",
            "date: 2026-07-10\nschema-version: 1\n",
            1
        ),
        "the migrated research doc is the v0 bytes plus the appended stamp line, nothing else",
    );

    // RE-RUN — idempotent: both docs report already current, byte-untouched.
    let rerun = migrate_json(repo.path(), home.path());
    assert!(
        paths(&rerun, "migrated").is_empty() && paths(&rerun, "blocked").is_empty(),
        "a re-run migrates nothing; got: {rerun}",
    );
    assert_eq!(
        paths(&rerun, "already_current"),
        vec![
            "docs/research/cache-strategy.md".to_string(),
            "docs/roadmap.md".to_string(),
        ],
        "a re-run reports both docs already current; got: {rerun}",
    );
    let roadmap_after =
        fs::read_to_string(repo.path().join("docs/roadmap.md")).expect("read roadmap");
    let research_after = fs::read_to_string(repo.path().join("docs/research/cache-strategy.md"))
        .expect("read research");
    assert_eq!(roadmap_after, roadmap, "the re-run is byte-inert");
    assert_eq!(research_after, research, "the re-run is byte-inert");
}
