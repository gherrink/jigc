//! M42 Increment 8 / T6 — `jigc doc list [<doctype>]`, the **fourth read surface**:
//! the committed store surface projected **by identity**, each row carrying its
//! **registration state** (`design/doc-read-surface.md` → `jigc doc list` — the fourth
//! read surface (M42); `design/validation.md` → The managed-vs-foreign discriminator).
//!
//! `doc show` answers *"what does this doc say"* and presupposes you already know the doc
//! exists; nothing answered *"which docs exist"* — an agent's only route to the corpus was
//! to guess a slug or read the filesystem, the raw read the adapter rule forbids. The verb
//! rides `engine::index::committed_instances` (the placement-aware census enumerator the
//! store sweep already uses — one primitive, two consumers) and adjudicates each row's state
//! with the **one** discriminator `engine::validate::is_unadopted_foreign` owns.
//!
//! **The posture is declared as it ships** (the M41 lesson — an undeclared output calcifies
//! into a de-facto contract): `doc show`'s posture, **no in-band version integer**, additive
//! keys pre-1.0 only — so the `--format json` shape is **golden-pinned here, verbatim**:
//! `{"docs":[{"id","path","state"}]}`, an object wrapper (a bare array can never take an
//! additive key), sorted by (type, slug).
//!
//! Everything is asserted on the EMITTED bytes of the real binary (`CARGO_BIN_EXE_jigc`).
//! No external test crates.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-doc-list-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        path.push(unique);
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer, naming
/// the methodology pack in `packs.yaml` (the `[dev ▸ methodology]` composition — so the
/// listing spans both packs' doctypes).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run jigc")
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

/// The stdout of an invocation.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

/// A canonical committed ADR — jigc's own doc: **stamped** at the adr's frozen
/// `schema-version: 2`, so the discriminator reads it `managed` on arm 1.
const COMMITTED_ADR: &str = "\
---
status: accepted
date: 2026-05-23
schema-version: 2
---

# Single-node cache

## Context

Forces.

## Options

Alternatives were weighed and rejected.

## Decision

A single in-memory node.

## Consequences

None.
";

/// A stock brownfield repo's **real Keep-a-Changelog `CHANGELOG.md`** — a file the user never
/// handed to jigc, sitting at the `changelog` doctype's canonical placement home. Unstamped,
/// and it parses against no shipped `changelog` shape ⇒ **foreign, never adopted**.
const FOREIGN_CHANGELOG: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0] - 2026-05-01

### Added

- A thing.
";

/// The pinned `--format json` shape (the posture: no in-band version integer, additive keys
/// pre-1.0 only) — the object wrapper, rows sorted by (type, slug).
const STORE_JSON: &str = r#"{
  "docs": [
    {
      "id": "adr:single-node-cache",
      "path": "decisions/single-node-cache.md",
      "state": "managed",
      "item-count": 0
    },
    {
      "id": "changelog:changelog",
      "path": "CHANGELOG.md",
      "state": "unregistered",
      "item-count": 0
    }
  ]
}"#;

/// A canonical committed **managed** `prd` — jigc's own doc (stamped `schema-version: 1`),
/// carrying a `requirements` **repeatable** section with **two** items. Captured verbatim
/// from the real `doc author prd` write path, so it parses clean against the current `prd`
/// schema ⇒ the listing counts its two items (`item-count: 2` — the nonzero managed count,
/// so the new parse-and-count path is genuinely exercised, not masked by all-zero rows).
const MANAGED_PRD: &str = "\
---
schema-version: 1
---

# Habit tracker

## Vision

A tracker that turns intentions into daily streaks.

## Requirements

### Log a habit in one tap  {#log-a-habit-in-one}

Logging a habit takes a single tap from the home screen.

### Show the current streak  {#show-the-current-streak}

The current streak is shown front and center.

## Context

Built for solo users who abandon heavyweight planners.
";

/// The pinned listing shape when the store holds a repeatable-less managed adr (`item-count`
/// 0) beside a two-requirement managed prd (`item-count` 2) — the additive key present on
/// every row, its value the parsed top-level repeatable-item count.
const STORE_WITH_ITEMS_JSON: &str = r#"{
  "docs": [
    {
      "id": "adr:single-node-cache",
      "path": "decisions/single-node-cache.md",
      "state": "managed",
      "item-count": 0
    },
    {
      "id": "prd:habit-tracker",
      "path": "prds/habit-tracker.md",
      "state": "managed",
      "item-count": 2
    }
  ]
}"#;

/// Seed the archetypal store: one committed (managed) ADR + one foreign root `CHANGELOG.md`.
fn seed_store(repo: &Path) {
    let decisions = repo.join("decisions");
    fs::create_dir_all(&decisions).expect("mk decisions/");
    fs::write(decisions.join("single-node-cache.md"), COMMITTED_ADR).expect("write committed adr");
    fs::write(repo.join("CHANGELOG.md"), FOREIGN_CHANGELOG).expect("write foreign changelog");
    git(repo, &["add", "decisions", "CHANGELOG.md"]);
    git(repo, &["commit", "-q", "-m", "the store"]);
}

/// (M42 inc-8 T6) **`jigc doc list` projects the store surface by identity, with each row's
/// registration state** — and its `--format json` shape is golden-pinned at ship.
#[test]
fn doc_list_projects_the_store_surface_with_its_registration_state() {
    let repo = TempDir::new("surface");
    let home = TempDir::new("home");
    init_repo(repo.path());
    seed_store(repo.path());

    // (1) Plain — `<id>  <path>  <state>`, sorted by (type, slug). The foreign
    //     `CHANGELOG.md` is LISTED, flagged `unregistered`: omitting the very file jigc is
    //     telling the agent to adopt would send it straight to `cat`.
    let plain = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&plain, "`jigc doc list`");
    assert_eq!(
        stdout_of(&plain),
        "adr:single-node-cache  decisions/single-node-cache.md  managed\n\
         changelog:changelog  CHANGELOG.md  unregistered\n",
        "the listing is the store surface by identity, each row carrying its state",
    );

    // (2) `--format json` — the pinned shape, verbatim.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--format", "json"],
    );
    assert_ok(&json, "`jigc doc list --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        STORE_JSON,
        "the pinned `doc list --format json` shape",
    );

    // (3) The doctype filter — one doctype's instances only.
    let filtered = jigc(repo.path(), home.path(), &["doc", "list", "adr"]);
    assert_ok(&filtered, "`jigc doc list adr`");
    assert_eq!(
        stdout_of(&filtered),
        "adr:single-node-cache  decisions/single-node-cache.md  managed\n",
        "`doc list <doctype>` filters to that doctype",
    );

    // (4) A **transient** (location-less, non-placement) doctype has no committed instance,
    //     so it yields no rows — at exit 0, never a block, and never zero bytes: the
    //     doctype-filtered empty case prints the empty-set line naming its scope (M43
    //     Inc 7 / T5; `design/surface-contract.md` → the style guide).
    let transient = jigc(repo.path(), home.path(), &["doc", "list", "commit"]);
    assert_ok(&transient, "`jigc doc list commit`");
    assert_eq!(
        stdout_of(&transient),
        "jigc doc list — no committed `commit` docs\n",
        "a doctype-filtered empty listing prints the empty-set line naming the doctype",
    );
    let transient_json = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "commit", "--format", "json"],
    );
    assert_ok(&transient_json, "`jigc doc list commit --format json`");
    assert_eq!(
        stdout_of(&transient_json).trim_end(),
        "{\n  \"docs\": []\n}",
        "an empty listing is still the pinned object wrapper",
    );
}

/// (M43 inc-7 T5) An **empty listing prints an empty-set line, never zero bytes**
/// (`design/surface-contract.md` → the style guide; the `task list` empty-roster mold):
/// the whole-store empty case at exit 0, and the `--format json` shape unchanged (the
/// pinned object wrapper — the empty-set line is agent-arm prose only).
#[test]
fn doc_list_prints_an_empty_set_line_on_an_empty_store() {
    let repo = TempDir::new("empty");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // (1) The empty store — the empty-set line, exit 0.
    let plain = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&plain, "`jigc doc list` over an empty store");
    assert_eq!(
        stdout_of(&plain),
        "jigc doc list — no committed docs\n",
        "the empty store prints the empty-set line, never zero bytes",
    );

    // (2) The pinned `--format json` shape is unchanged — no prose line rides it.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--format", "json"],
    );
    assert_ok(&json, "`jigc doc list --format json` over an empty store");
    assert_eq!(
        stdout_of(&json).trim_end(),
        "{\n  \"docs\": []\n}",
        "the empty listing keeps the pinned object wrapper",
    );
}

/// (M44 inc-7 T3) Each `--format json` row carries the additive **`item-count`** key — the
/// parsed count of the doc's top-level repeatable items (`design/doc-read-surface.md` → the
/// item-count additive key). `doc list` did not parse instances before this, so the count is
/// a new parse: a repeatable-less managed adr counts 0, a two-requirement managed prd counts
/// 2, and a foreign/unparseable instance degrades to 0 (proven by the archetypal store above).
#[test]
fn doc_list_json_carries_the_item_count_of_each_doc() {
    let repo = TempDir::new("counts");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let decisions = repo.path().join("decisions");
    fs::create_dir_all(&decisions).expect("mk decisions/");
    fs::write(decisions.join("single-node-cache.md"), COMMITTED_ADR).expect("write managed adr");
    let prds = repo.path().join("prds");
    fs::create_dir_all(&prds).expect("mk prds/");
    fs::write(prds.join("habit-tracker.md"), MANAGED_PRD).expect("write managed prd");
    git(repo.path(), &["add", "decisions", "prds"]);
    git(repo.path(), &["commit", "-q", "-m", "the store"]);

    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--format", "json"],
    );
    assert_ok(&json, "`jigc doc list --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        STORE_WITH_ITEMS_JSON,
        "each row carries item-count: the adr 0, the two-requirement prd 2",
    );
}

/// (M42 inc-8 T6) An **unknown doctype blocks** `store.unknown-type`, routed at
/// `jigc describe` — the same routed block the read-side `doc show` / `doc schema` raise.
#[test]
fn doc_list_blocks_an_unknown_doctype_routed_at_describe() {
    let repo = TempDir::new("unknown");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["doc", "list", "nonesuch"]);
    assert!(
        !out.status.success(),
        "an unknown doctype blocks; stdout:\n{}",
        stdout_of(&out),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("store.unknown-type") && stderr.contains("nonesuch"),
        "the block names the code + the unknown doctype; got:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc describe"),
        "the block routes at `jigc describe`; got:\n{stderr}",
    );
}
