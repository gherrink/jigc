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
//! **M48 Increment 3 / T3 — a task has a surface too.** `--task <id>` lists what that
//! task **stages** (the same row shape, staged-only), and a task-less listing served while
//! an open task stages docs routes at it on **stderr** — the `stale_read_hint` mold, stdout
//! byte-identical.
//!
//! **M55 Increment 6 / T3 — the row a triage reads.** Every row carries `title` (the `# H1`,
//! or `null`) and `fields` (`doc show`'s header-field map on a `managed` row that parses,
//! `null` on every other), each state driven by `doc_list_rows_carry_title_and_fields_by_state`.
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
            engine::tempname::unique_nanos(),
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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

/// The stderr of an invocation — where every advisory this verb emits rides.
fn stderr_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stderr.clone()).expect("utf-8 stderr")
}

/// Start a `single-task` task with `intent`, asserting success. The task id is the
/// slugified intent, which the caller names.
fn start_task(repo: &Path, home: &Path, intent: &str) {
    assert_ok(
        &jigc(repo, home, &["start", "--workflow", "single-task", intent]),
        "`jigc start --workflow single-task`",
    );
}

/// Lift the **emitted** backtick-quoted command out of an advisory line — the bytes an
/// agent would copy — so the route is run verbatim rather than reconstructed in test code.
fn lift_route(stderr: &str) -> Vec<String> {
    let start = stderr
        .find("`jigc ")
        .expect("the advisory carries a backtick-quoted `jigc …` command");
    let rest = &stderr[start + 1..];
    let end = rest.find('`').expect("the quoted command closes");
    rest[..end].split_whitespace().map(str::to_string).collect()
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
      "item-count": 0,
      "title": "Single-node cache",
      "fields": {
        "date": "2026-05-23",
        "schema-version": "2",
        "status": "accepted"
      }
    },
    {
      "id": "changelog:changelog",
      "path": "CHANGELOG.md",
      "state": "unregistered",
      "item-count": 0,
      "title": "Changelog",
      "fields": null
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
      "item-count": 0,
      "title": "Single-node cache",
      "fields": {
        "date": "2026-05-23",
        "schema-version": "2",
        "status": "accepted"
      }
    },
    {
      "id": "prd:habit-tracker",
      "path": "prds/habit-tracker.md",
      "state": "managed",
      "item-count": 2,
      "title": "Habit tracker",
      "fields": {
        "schema-version": "1"
      }
    }
  ]
}"#;

/// A committed `.md` jigc **stamped** that **no resolved doctype claims** — the orphaned
/// instance (M51 Increment 8 / T3, `design/validation.md` → the M51 registrations). Its front
/// matter carries the whole of what a jigc stamp is: a `schema-version:` number, and **no
/// type**. That is the datum the row's `id: null` rests on — there is nothing in the file, the
/// index or the commit that names what this doc was.
const STAMPED_ORPHAN: &str = "\
---
schema-version: 1
---

# An orphan

Prose jigc wrote and can no longer say anything about.
";

/// The pinned listing shape when two stamped orphans sit beside the archetypal store. Three
/// declared facts ride this document: an orphan row's `id` is **`null`** (no resolved schema
/// defines the type its stamp was written under, and the stamp names none — a synthesized
/// `<type>:<slug>` would be an address `doc show` refuses), its `item-count` is **`null`**
/// (no schema to parse it against — distinct from the best-effort `0` a *parse failure*
/// yields), and the orphan rows sort **after** every resolved row, path-ordered among
/// themselves: `archive/old.md` sorts before `decisions/single-node-cache.md` by path, so a
/// row set merely path-sorted end-to-end would put it second.
const STORE_WITH_ORPHANS_JSON: &str = r#"{
  "docs": [
    {
      "id": "adr:single-node-cache",
      "path": "decisions/single-node-cache.md",
      "state": "managed",
      "item-count": 0,
      "title": "Single-node cache",
      "fields": {
        "date": "2026-05-23",
        "schema-version": "2",
        "status": "accepted"
      }
    },
    {
      "id": "changelog:changelog",
      "path": "CHANGELOG.md",
      "state": "unregistered",
      "item-count": 0,
      "title": "Changelog",
      "fields": null
    },
    {
      "id": null,
      "path": "decisions/archive/old.md",
      "state": "orphaned",
      "item-count": null,
      "title": "An orphan",
      "fields": null
    },
    {
      "id": null,
      "path": "decisions/notes/later.md",
      "state": "orphaned",
      "item-count": null,
      "title": "An orphan",
      "fields": null
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

    // (1) Plain — the column header, then `<id>  <path>  <state>` rows sorted by
    //     (type, slug). The foreign `CHANGELOG.md` is LISTED, flagged `unregistered`:
    //     omitting the very file jigc is telling the agent to adopt would send it
    //     straight to `cat`.
    let plain = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&plain, "`jigc doc list`");
    assert_eq!(
        stdout_of(&plain),
        "id  path  state\n\
         adr:single-node-cache  decisions/single-node-cache.md  managed\n\
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

    // (3) The doctype filter — one doctype's instances only, under the same header.
    let filtered = jigc(repo.path(), home.path(), &["doc", "list", "adr"]);
    assert_ok(&filtered, "`jigc doc list adr`");
    assert_eq!(
        stdout_of(&filtered),
        "id  path  state\n\
         adr:single-node-cache  decisions/single-node-cache.md  managed\n",
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

/// (M47 inc-10 T5 — P4-5/C3) The plain listing **names its columns**
/// (`design/doc-read-surface.md` → the fourth read surface; `design/surface-contract.md`
/// → law 2: nothing hides). Three bare columns left the reader to infer what the third
/// one meant; the header names them in **row order**, in the **pinned json's own key
/// spelling** (`id` · `path` · `state`), so the plain arm teaches the machine arm's
/// vocabulary — every header token is a real json key, checked against the json arm of
/// the same store rather than against a hand-written list.
///
/// Two **omitting contexts** ride here, because a header is a per-context claim: an
/// **empty** listing prints the empty-set line and **no** header (a header above nothing
/// names nothing), and the **`--format json`** arm carries none (the keys are the shape).
#[test]
fn doc_list_plain_names_its_columns_in_row_order() {
    let repo = TempDir::new("headers");
    let home = TempDir::new("home");
    init_repo(repo.path());
    seed_store(repo.path());

    // (1) The header is the first line, and every token it names is a real key of the
    //     pinned json row — the two arms share one vocabulary.
    let plain = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&plain, "`jigc doc list`");
    let out = stdout_of(&plain);
    let (header, rows) = out.split_once('\n').expect("a header line, then the rows");
    assert_eq!(
        header, "id  path  state",
        "the plain listing leads with its column header; got:\n{out}",
    );
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--format", "json"],
    );
    assert_ok(&json, "`jigc doc list --format json`");
    let json_out = stdout_of(&json);
    for column in header.split("  ") {
        assert!(
            json_out.contains(&format!("\"{column}\":")),
            "the header names the pinned json key `{column}`; json:\n{json_out}",
        );
    }

    // (2) It names them in ROW order: the first row's three fields line up with the
    //     three header tokens, positionally.
    let first = rows.lines().next().expect("at least one row");
    let cells: Vec<&str> = first.split("  ").collect();
    assert_eq!(
        cells,
        vec![
            "adr:single-node-cache",
            "decisions/single-node-cache.md",
            "managed",
        ],
        "the row's cells sit under the header tokens they are named by; got:\n{out}",
    );

    // (3) Omitting context A — an empty listing prints the empty-set line, no header.
    let empty_repo = TempDir::new("headers-empty");
    let empty_home = TempDir::new("home");
    init_repo(empty_repo.path());
    let empty = jigc(empty_repo.path(), empty_home.path(), &["doc", "list"]);
    assert_ok(&empty, "`jigc doc list` over an empty store");
    assert_eq!(
        stdout_of(&empty),
        "jigc doc list — no committed docs\n",
        "an empty listing carries the empty-set line alone — a header above nothing \
         names nothing",
    );

    // (4) Omitting context B — the json arm carries no header line (its keys are the
    //     shape); the pinned document is unchanged by the plain-arm header.
    assert_eq!(
        json_out.trim_end(),
        STORE_JSON,
        "the plain-arm header never reaches the pinned json shape",
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

/// (M51 inc-8 T4) **`jigc doc list` prints the orphaned row** — the store surface stops
/// dropping a file it is simultaneously refusing to validate over
/// (`design/doc-read-surface.md` → the fourth read surface, the third `state` value;
/// `completions/artifacts/M51/settle-record.md` → §20 fork 1). At the base this listing
/// carried the two resolved rows and nothing else, while `jigc validate` blocked on both
/// stamped files by name — which re-opens `doc list`'s founding argument, that an omitted
/// file sends the agent straight to `cat`, on the very files jigc has just exit-flipped over.
///
/// Four facts, each asserted on the emitted bytes:
///
/// - the row's **`id` is `null`** and its **`item-count` is `null`** — the pinned document
///   below is the declaration (and the reshape of two keys already on the wire, admissible
///   only while the pre-1.0 window is open — `design/command-output-contract.md` →
///   Evolution posture, the third pre-pin case);
/// - the plain arm renders that null identity as something **unpasteable** — never a
///   synthesized `<type>:<slug>` (an address `doc show` refuses) and never the path moved
///   into the identity column (a meaning change on a pinned key);
/// - orphan rows sort **after** every resolved row, path-ordered among themselves;
/// - a **doctype-filtered** listing carries none of them: an orphan belongs to no doctype,
///   so every filter excludes it — including the filter naming the type it was written
///   under, which resolves to nothing.
#[test]
fn doc_list_prints_the_orphaned_row_with_a_null_identity_and_no_item_count() {
    let repo = TempDir::new("orphans");
    let home = TempDir::new("home");
    init_repo(repo.path());
    seed_store(repo.path());
    // Both orphans sit INSIDE jigc's declared territory — under the `adr` doctype's own home
    // tree, one level below it — and not at any doctype's home (the census reads the home dir
    // non-recursively). Since the M51 completion audit that is what makes them this sweep's
    // subject at all: the `schema-version:` key is unnamespaced, so a stamped file outside
    // jigc's homes is a team document jigc may not speak for (`cli::orphan::Territory`).
    for rel in ["decisions/archive/old.md", "decisions/notes/later.md"] {
        let path = repo.path().join(rel);
        fs::create_dir_all(path.parent().expect("a parent dir")).expect("mk the orphan's dir");
        fs::write(&path, STAMPED_ORPHAN).expect("write the stamped orphan");
    }
    git(repo.path(), &["add", "decisions"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "docs jigc can no longer claim"],
    );

    // (1) The pinned json — `id: null`, `state: "orphaned"`, `item-count: null`, and the
    //     orphan rows last, path-ordered.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--format", "json"],
    );
    assert_ok(&json, "`jigc doc list --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        STORE_WITH_ORPHANS_JSON,
        "an orphaned instance is a row with a null identity and no item count, sorted \
         after the resolved rows",
    );

    // (2) The plain arm — the null identity renders unpasteable in the `id` column, and
    //     the path stays in the `path` column where it means what it has always meant.
    let plain = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&plain, "`jigc doc list`");
    let out = stdout_of(&plain);
    assert_eq!(
        out,
        "id  path  state\n\
         adr:single-node-cache  decisions/single-node-cache.md  managed\n\
         changelog:changelog  CHANGELOG.md  unregistered\n\
         (none)  decisions/archive/old.md  orphaned\n\
         (none)  decisions/notes/later.md  orphaned\n",
        "the plain listing prints the orphan rows with an unpasteable identity cell",
    );
    for row in out.lines().filter(|line| line.ends_with("orphaned")) {
        let id = row.split("  ").next().expect("an identity cell");
        assert!(
            !id.contains(':') && !id.contains('/'),
            "the identity cell of an orphan row must be neither an address `doc show` \
             refuses nor the path (the meaning change a pinned key cannot take); got \
             `{id}` in:\n{out}",
        );
    }

    // (3) A doctype-filtered listing carries no orphan row — an orphan belongs to no
    //     doctype, so it is out of every narrowed scope, including the one that would
    //     name the type its stamp was written under.
    let filtered = jigc(repo.path(), home.path(), &["doc", "list", "adr"]);
    assert_ok(&filtered, "`jigc doc list adr`");
    assert_eq!(
        stdout_of(&filtered),
        "id  path  state\n\
         adr:single-node-cache  decisions/single-node-cache.md  managed\n",
        "a doctype-narrowed listing carries no orphan rows",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// M55 Increment 6 / T3 — `title` and `fields` on every row
// ═════════════════════════════════════════════════════════════════════════════

/// A committed doc at the `adr` home, **stamped** — so the discriminator reads it `managed` —
/// whose body declares no section the `adr` schema does, so it does not parse. Its H1 is
/// still there, and the row's `title` needs no parse to read it.
const UNPARSEABLE_ADR: &str = "\
---
schema-version: 2
---

# Half an ADR

## Nonsense

No section the adr declares.
";

/// A stamped orphan with **no H1** — the one row state in which `title` reads `null`.
const UNTITLED_ORPHAN: &str = "\
---
schema-version: 1
---

Prose with no heading at all.
";

/// The one row whose `path` is `rel`, out of a parsed `doc list --format json` listing.
fn row_at<'a>(listing: &'a serde_json::Value, rel: &str) -> &'a serde_json::Value {
    listing["docs"]
        .as_array()
        .expect("`docs` is an array")
        .iter()
        .find(|row| row["path"] == rel)
        .unwrap_or_else(|| panic!("a row at `{rel}`; listing:\n{listing:#}"))
}

/// The value of `key` on `row`, asserting the key is **present** — a `null` that is really an
/// absent key would let a row that never grew the key pass every `null` assertion below.
fn key<'a>(row: &'a serde_json::Value, key: &str) -> &'a serde_json::Value {
    row.get(key)
        .unwrap_or_else(|| panic!("every row carries `{key}`; row:\n{row:#}"))
}

/// Parse an invocation's stdout as json.
fn json_of(out: &std::process::Output) -> serde_json::Value {
    serde_json::from_str(&stdout_of(out)).expect("the json arm parses")
}

/// (M55 inc-6 T3) **Every `doc list --format json` row carries `title` and `fields`, with the
/// value its state declares** (`design/findings-channel.md` → 5, R4 B5;
/// `design/doc-read-surface.md` → `jigc doc list`). `title` is the doc's `# H1`, or `null`
/// where it has none, on every row; `fields` is the header-field map `doc show` serves on a
/// `managed` row that parses, and `null` — never `{}` — on every other. Red at the base: no row
/// carried either key.
///
/// The states, each driven on the emitted bytes:
///
/// - **managed, parsing** — the H1, and the doc's fields, equal to `doc show`'s;
/// - **managed, not parsing** — the H1, and `null`;
/// - **unregistered** — the H1, and `null`;
/// - **orphaned** — the H1, and `null`; with no H1, `null` and `null`;
/// - **`--task`** — read from the staged copy, whose `status` differs from the committed one.
#[test]
fn doc_list_rows_carry_title_and_fields_by_state() {
    let repo = TempDir::new("title-fields");
    let home = TempDir::new("home");
    init_repo(repo.path());
    seed_store(repo.path());
    let decisions = repo.path().join("decisions");
    fs::write(decisions.join("half-an-adr.md"), UNPARSEABLE_ADR).expect("write the unparseable");
    fs::create_dir_all(decisions.join("archive")).expect("mk the orphans' dir");
    fs::write(decisions.join("archive").join("old.md"), STAMPED_ORPHAN).expect("write orphan");
    fs::write(
        decisions.join("archive").join("untitled.md"),
        UNTITLED_ORPHAN,
    )
    .expect("write the untitled orphan");
    git(repo.path(), &["add", "decisions"]);
    git(repo.path(), &["commit", "-q", "-m", "every row state"]);

    let listed = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--format", "json"],
    );
    assert_ok(&listed, "`jigc doc list --format json`");
    let listing = json_of(&listed);

    // Managed, parsing: the H1, and the very fields `doc show` serves for the same doc.
    let managed = row_at(&listing, "decisions/single-node-cache.md");
    assert_eq!(managed["state"], "managed");
    assert_eq!(key(managed, "title"), "Single-node cache");
    assert_eq!(
        key(managed, "fields"),
        &serde_json::json!({"date": "2026-05-23", "schema-version": "2", "status": "accepted"}),
    );
    let shown = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:single-node-cache", "--format", "json"],
    );
    assert_ok(
        &shown,
        "`jigc doc show adr:single-node-cache --format json`",
    );
    let shown = json_of(&shown);
    assert_eq!(
        (key(managed, "title"), key(managed, "fields")),
        (&shown["title"], &shown["fields"]),
        "a managed row that parses carries `doc show`'s own `title` and `fields`",
    );

    // Managed, not parsing: stamped, so managed — the H1 still read, `fields` null.
    let broken = row_at(&listing, "decisions/half-an-adr.md");
    assert_eq!(broken["state"], "managed");
    assert_eq!(key(broken, "title"), "Half an ADR");
    assert_eq!(key(broken, "fields"), &serde_json::Value::Null);

    // Unregistered: the H1, `fields` null — not adopted, whatever its bytes parse as.
    let foreign = row_at(&listing, "CHANGELOG.md");
    assert_eq!(foreign["state"], "unregistered");
    assert_eq!(key(foreign, "title"), "Changelog");
    assert_eq!(key(foreign, "fields"), &serde_json::Value::Null);

    // Orphaned: no schema to parse against, but the H1 needs none.
    let orphan = row_at(&listing, "decisions/archive/old.md");
    assert_eq!(orphan["state"], "orphaned");
    assert_eq!(key(orphan, "title"), "An orphan");
    assert_eq!(key(orphan, "fields"), &serde_json::Value::Null);
    let untitled = row_at(&listing, "decisions/archive/untitled.md");
    assert_eq!(untitled["state"], "orphaned");
    assert_eq!(key(untitled, "title"), &serde_json::Value::Null);
    assert_eq!(key(untitled, "fields"), &serde_json::Value::Null);

    // `--task`: the staged copy's values, which differ from the committed doc's.
    start_task(repo.path(), home.path(), "harden the cache");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                "adr:single-node-cache#status",
                "--value",
                "superseded",
                "--task",
                "harden-the-cache",
            ],
        ),
        "`jigc doc set-field` — the staged copy diverges",
    );
    let staged = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "list",
            "adr",
            "--task",
            "harden-the-cache",
            "--format",
            "json",
        ],
    );
    assert_ok(&staged, "`jigc doc list adr --task … --format json`");
    let staged = json_of(&staged);
    let copy = row_at(&staged, "decisions/single-node-cache.md");
    assert_eq!(key(copy, "title"), "Single-node cache");
    assert_eq!(
        key(copy, "fields")["status"],
        "superseded",
        "the staged row reads the staged copy; row:\n{copy:#}",
    );
    let committed = json_of(&jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "adr", "--format", "json"],
    ));
    assert_eq!(
        key(
            row_at(&committed, "decisions/single-node-cache.md"),
            "fields"
        )["status"],
        "accepted",
        "the committed row still reads the committed doc",
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

// ═════════════════════════════════════════════════════════════════════════════
// M48 Increment 3 / T3 — a task has a surface too
// ═════════════════════════════════════════════════════════════════════════════

/// The pinned `--format json` shape of the **staged** listing (`design/doc-read-surface.md`
/// → the fourth read surface, the staged arm). The row shape is the committed arm's,
/// key-for-key — the `--task` arm adds none:
///
/// - **`state`** is `managed` on every row: a staged working copy is jigc-written by
///   construction, never a foreign squatter (the same reason `doc show --task` runs no
///   adoption reroute);
/// - **`path`** is where the instance **promotes to at finalize** — the M43 A14 staged
///   display rule, so a printed path is repo-real rather than a working-area fiction;
///   a **transient** doctype (`commit:<task-id>`, sink = the git message) has no
///   committed home and therefore lists at its typed `<type>:<slug>` identity.
///
/// **`title` and `fields` are read from the staged copy** (M55): the copied-in adr carries
/// its staged `status: superseded`, never the committed `accepted`, the created adr its
/// `set: on-create` date — `<DATE>`, interpolated from the doc's own leaf read — and the
/// provisioned commit skeleton its H1 (the task id) and its two still-empty header fields.
const STAGED_JSON: &str = r#"{
  "docs": [
    {
      "id": "adr:cache-eviction",
      "path": "decisions/cache-eviction.md",
      "state": "managed",
      "item-count": 0,
      "title": "Cache eviction",
      "fields": {
        "date": "<DATE>",
        "schema-version": "2",
        "status": "proposed"
      }
    },
    {
      "id": "adr:single-node-cache",
      "path": "decisions/single-node-cache.md",
      "state": "managed",
      "item-count": 0,
      "title": "Single-node cache",
      "fields": {
        "date": "2026-05-23",
        "schema-version": "2",
        "status": "superseded"
      }
    },
    {
      "id": "commit:harden-the-cache",
      "path": "commit:harden-the-cache",
      "state": "managed",
      "item-count": 0,
      "title": "harden-the-cache",
      "fields": {
        "scope": "",
        "type": ""
      }
    }
  ]
}"#;

/// (M48 inc-3 T3) **`jigc doc list --task <id>` lists what the task stages** — the
/// fourth read surface learns that a task has a surface too
/// (`design/doc-read-surface.md` → the fourth read surface, the staged arm;
/// `design/surface-contract.md` → law 2: nothing hides).
///
/// Six consecutive trials went to the filesystem to read their own in-flight work. `doc
/// show --task <id>` has served the staged copy since M43 — but it presupposes you know
/// the address, and the *index* read was committed-only, so nothing answered *"which
/// docs does my task hold"*. Red at HEAD: `--task` died at clap with a bare parse error.
///
/// Staged-**only**, matching `doc show --task` (which blocks `store.not-staged` rather
/// than falling back to committed): the foreign `CHANGELOG.md` the committed listing
/// flags `unregistered` is absent here, because the task does not stage it.
#[test]
fn doc_list_task_lists_what_the_task_stages() {
    let repo = TempDir::new("staged");
    let home = TempDir::new("home");
    init_repo(repo.path());
    seed_store(repo.path());

    start_task(repo.path(), home.path(), "harden the cache");
    let created = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache eviction",
            "--task",
            "harden-the-cache",
        ],
    );
    assert_ok(&created, "`jigc doc create adr`");
    // A committed doc **copied in** by a staged write — it must be listed ONCE, from the
    // staged surface, never twice (the two views are not merged).
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                "adr:single-node-cache#status",
                "--value",
                "superseded",
                "--task",
                "harden-the-cache",
            ],
        ),
        "`jigc doc set-field` — the copy-in",
    );

    // (1) Plain — the same header, then the staged rows: the created adr, the copied-in
    //     committed adr (once), and the `start`-provisioned transient commit skeleton.
    let plain = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--task", "harden-the-cache"],
    );
    assert_ok(&plain, "`jigc doc list --task harden-the-cache`");
    assert_eq!(
        stdout_of(&plain),
        "id  path  state\n\
         adr:cache-eviction  decisions/cache-eviction.md  managed\n\
         adr:single-node-cache  decisions/single-node-cache.md  managed\n\
         commit:harden-the-cache  commit:harden-the-cache  managed\n",
        "the staged listing carries the task's staged docs, each once",
    );
    assert!(
        !stdout_of(&plain).contains("CHANGELOG"),
        "the staged arm is staged-ONLY — the committed store's foreign squatter is not \
         in it; got:\n{}",
        stdout_of(&plain),
    );

    // (2) `--format json` — the row shape key-for-key, the `--task` arm adding none.
    let json = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "list",
            "--task",
            "harden-the-cache",
            "--format",
            "json",
        ],
    );
    assert_ok(&json, "`jigc doc list --task … --format json`");
    let date = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "adr:cache-eviction#status/date",
            "--task",
            "harden-the-cache",
        ],
    );
    assert_ok(&date, "`jigc doc show adr:cache-eviction#status/date`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        STAGED_JSON.replace("<DATE>", stdout_of(&date).trim()),
        "the staged listing's pinned shape",
    );

    // (3) The doctype positional narrows the staged listing exactly as it narrows the
    //     committed one — the transient commit skeleton drops out.
    let narrowed = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "adr", "--task", "harden-the-cache"],
    );
    assert_ok(&narrowed, "`jigc doc list adr --task harden-the-cache`");
    assert_eq!(
        stdout_of(&narrowed),
        "id  path  state\n\
         adr:cache-eviction  decisions/cache-eviction.md  managed\n\
         adr:single-node-cache  decisions/single-node-cache.md  managed\n",
        "`doc list <doctype> --task <id>` filters the staged surface",
    );

    // (4) The **omitting context**: a doctype the task stages nothing of. Not an error
    //     and not zero bytes — the empty-set line, naming both the scope and the task.
    let empty = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "prd", "--task", "harden-the-cache"],
    );
    assert_ok(&empty, "`jigc doc list prd --task harden-the-cache`");
    assert_eq!(
        stdout_of(&empty),
        "jigc doc list — no `prd` docs staged in task harden-the-cache\n",
        "an empty staged listing states its empty set, naming the doctype and the task",
    );
    let empty_json = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "list",
            "prd",
            "--task",
            "harden-the-cache",
            "--format",
            "json",
        ],
    );
    assert_ok(&empty_json, "`jigc doc list prd --task … --format json`");
    assert_eq!(
        stdout_of(&empty_json).trim_end(),
        "{\n  \"docs\": []\n}",
        "an empty staged listing is still the pinned object wrapper",
    );

    // (5) An unknown task id gets the shared wrong-id route, exactly like `doc show
    //     --task` — never a bare clap parse error (the HEAD behaviour this closes).
    let bad = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--task", "no-such-task"],
    );
    assert!(
        !bad.status.success(),
        "an unknown task id must not succeed; stdout:\n{}",
        stdout_of(&bad),
    );
    let stderr = stderr_of(&bad);
    assert!(
        stderr.contains("no-such-task") && stderr.contains("jigc task list"),
        "the unknown task id routes at `jigc task list`; got:\n{stderr}",
    );
}

/// (M48 inc-3 T3) **The task-less listing routes at the staged read** — the
/// `stale_read_hint` mold applied to the index read (`design/doc-read-surface.md` → the
/// fourth read surface, the staged arm; `design/surface-contract.md` → the route floor).
///
/// The committed listing is the honest answer to *"which docs exist"*, and it stays
/// **byte-identical**: the advisory rides **stderr** on both the empty and the non-empty
/// listing, so no second key and no second line reaches the pin. The route is **lifted
/// out of the emitted bytes and run verbatim**.
///
/// The **omitting context** rides here too: with nothing staged anywhere, there is no
/// advisory at all — an unconditional note would be noise on every read of a store with
/// no open work.
#[test]
fn a_task_less_listing_routes_at_the_staged_read() {
    let repo = TempDir::new("route");
    let home = TempDir::new("home");
    init_repo(repo.path());
    seed_store(repo.path());

    // (1) Omitting context — nothing staged, so nothing to route to: stderr is silent.
    let quiet = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&quiet, "`jigc doc list` with no open task");
    assert_eq!(
        stderr_of(&quiet),
        "",
        "with nothing staged the listing carries no advisory at all",
    );

    start_task(repo.path(), home.path(), "harden the cache");

    // (2) The non-empty listing — stdout byte-identical to the pre-change bytes (the
    //     shape the committed arms above pin), the advisory on stderr.
    let plain = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&plain, "`jigc doc list` with an open staging task");
    assert_eq!(
        stdout_of(&plain),
        "id  path  state\n\
         adr:single-node-cache  decisions/single-node-cache.md  managed\n\
         changelog:changelog  CHANGELOG.md  unregistered\n",
        "the advisory never reaches stdout — the committed listing is byte-identical",
    );
    let stderr = stderr_of(&plain);
    assert!(
        stderr.contains("staged in open task harden-the-cache"),
        "the advisory names the staging task; got:\n{stderr}",
    );
    assert!(
        stderr.contains("this listing is the committed store"),
        "it says which surface this listing was, not what the reader should fear; \
         got:\n{stderr}",
    );
    assert!(
        stderr.contains("if that task is yours, list what it stages:"),
        "the staged listing is handed over under an explicit `if that task is yours` \
         clause; got:\n{stderr}",
    );

    // (3) The pinned json is untouched too — the advisory rides stderr on both arms.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "list", "--format", "json"],
    );
    assert_ok(
        &json,
        "`jigc doc list --format json` with an open staging task",
    );
    assert_eq!(
        stdout_of(&json).trim_end(),
        STORE_JSON,
        "the pinned json shape is byte-identical with an open staging task",
    );
    assert!(
        stderr_of(&json).contains("harden-the-cache"),
        "the json arm carries the same stderr advisory; got:\n{}",
        stderr_of(&json),
    );

    // (4) The EMITTED route, lifted and run verbatim — it must exit 0 and answer.
    let argv = lift_route(&stderr);
    assert_eq!(
        argv,
        vec!["jigc", "doc", "list", "--task", "harden-the-cache"],
        "the route is the staged listing of the named task",
    );
    let run: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    let followed = jigc(repo.path(), home.path(), &run);
    assert_ok(&followed, "the emitted route, run verbatim");
    assert!(
        stdout_of(&followed).contains("commit:harden-the-cache"),
        "following the route lands on the task's staged surface; got:\n{}",
        stdout_of(&followed),
    );

    // (5) The EMPTY committed listing routes too — the case the finding is sharpest in:
    //     a store with nothing committed yet, while the reader's task holds staged work.
    let fresh = TempDir::new("route-empty");
    let fresh_home = TempDir::new("home");
    init_repo(fresh.path());
    start_task(fresh.path(), fresh_home.path(), "harden the cache");
    let empty = jigc(fresh.path(), fresh_home.path(), &["doc", "list"]);
    assert_ok(
        &empty,
        "`jigc doc list` over an empty store with an open task",
    );
    assert_eq!(
        stdout_of(&empty),
        "jigc doc list — no committed docs\n",
        "the empty-set line is byte-identical — the route rides stderr beside it",
    );
    let empty_route = lift_route(&stderr_of(&empty));
    let run: Vec<&str> = empty_route[1..].iter().map(String::as_str).collect();
    assert_ok(
        &jigc(fresh.path(), fresh_home.path(), &run),
        "the empty listing's emitted route, run verbatim",
    );

    // (6) The doctype scope the reader asked for survives into the route — a narrowed
    //     listing routes at the narrowed staged listing, not a wider one. The task stages
    //     its `commit` doc, so `doc list commit` is the narrowing it answers to (a doctype
    //     it stages nothing of is `a_narrowed_listing_names_only_tasks_staging_that_doctype`).
    let narrowed = jigc(repo.path(), home.path(), &["doc", "list", "commit"]);
    assert_ok(
        &narrowed,
        "`jigc doc list commit` with an open staging task",
    );
    assert_eq!(
        lift_route(&stderr_of(&narrowed)),
        vec![
            "jigc",
            "doc",
            "list",
            "commit",
            "--task",
            "harden-the-cache"
        ],
        "the route carries the doctype the reader scoped the listing to",
    );
}

/// (M55 audit O24) **A narrowed listing names only the tasks that stage that doctype.**
/// The note's claim is *"`<doctype>` docs are also staged in open task …"* and its route is
/// that task's narrowed staged listing — so a task staging nothing of the doctype is not
/// named: before the fix, `doc list adr` named a task staging only its commit doc, and the
/// route it handed over answered *"no `adr` docs staged"*. The note reads the staged arm's
/// own row predicate, so the route it hands over, **run verbatim**, lists a row.
///
/// Over two open tasks — one staging only its `commit` skeleton, one an `adr` beside its
/// own — on both arms (the note rides stderr on `--format json` too, stdout untouched):
/// `doc list adr` names the adr-staging task alone; with that task discarded it is silent;
/// `doc list commit` and the unfiltered `doc list` keep naming every staging task.
#[test]
fn a_narrowed_listing_names_only_tasks_staging_that_doctype() {
    let repo = TempDir::new("narrowed");
    let home = TempDir::new("home");
    init_repo(repo.path());
    seed_store(repo.path());
    start_task(repo.path(), home.path(), "harden the cache");
    start_task(repo.path(), home.path(), "trim the cache");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "create",
                "adr",
                "--title",
                "Cache eviction",
                "--task",
                "trim-the-cache",
            ],
        ),
        "`jigc doc create adr` in the second task",
    );

    for format in [&[][..], &["--format", "json"][..]] {
        let mut args = vec!["doc", "list", "adr"];
        args.extend_from_slice(format);
        let out = jigc(repo.path(), home.path(), &args);
        assert_ok(&out, "`jigc doc list adr` with two open tasks");
        let stderr = stderr_of(&out);
        assert!(
            stderr.contains("`adr` docs are also staged in open task trim-the-cache ")
                && !stderr.contains("harden-the-cache"),
            "{args:?}: only the task staging an adr is named; got:\n{stderr}",
        );
        let route = lift_route(&stderr);
        assert_eq!(
            route,
            vec!["jigc", "doc", "list", "adr", "--task", "trim-the-cache"]
        );
        let run: Vec<&str> = route[1..].iter().map(String::as_str).collect();
        let followed = jigc(repo.path(), home.path(), &run);
        assert_ok(&followed, "the emitted route, run verbatim");
        assert!(
            stdout_of(&followed).contains("adr:cache-eviction"),
            "{args:?}: the route lands on a listing with a row; got:\n{}",
            stdout_of(&followed),
        );
    }

    // The unfiltered listing and a doctype both tasks stage keep naming both.
    for args in [&["doc", "list"][..], &["doc", "list", "commit"][..]] {
        let stderr = stderr_of(&jigc(repo.path(), home.path(), args));
        assert!(
            stderr.contains("open tasks harden-the-cache, trim-the-cache"),
            "{args:?}: every task staging a listed doc is named; got:\n{stderr}",
        );
    }

    // With the adr-staging task gone, `doc list adr` has nothing staged to point at.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "discard", "trim-the-cache", "--force"],
        ),
        "`jigc task discard`",
    );
    for format in [&[][..], &["--format", "json"][..]] {
        let mut args = vec!["doc", "list", "adr"];
        args.extend_from_slice(format);
        let out = jigc(repo.path(), home.path(), &args);
        assert_ok(&out, "`jigc doc list adr` with no adr staged");
        assert_eq!(
            stderr_of(&out),
            "",
            "{args:?}: a task staging no `adr` is not named"
        );
    }
}

/// (M48 inc-3 T3) The advisory's **plural branch** — two open tasks staging docs. Both
/// ids are listed, the clause asks whether *one of them* is the reader's, and the route
/// carries the shared `<task-id>` placeholder (no single id could be right). The axis is
/// the number of staging tasks, because the sentence branches on it — the same axis the
/// `doc show` stale-read note is swept over.
#[test]
fn the_listing_advisory_branches_on_the_number_of_staging_tasks() {
    let repo = TempDir::new("plural");
    let home = TempDir::new("home");
    init_repo(repo.path());
    seed_store(repo.path());

    start_task(repo.path(), home.path(), "harden the cache");
    start_task(repo.path(), home.path(), "trim the cache");

    let out = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&out, "`jigc doc list` with two open staging tasks");
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains("open tasks")
            && stderr.contains("harden-the-cache")
            && stderr.contains("trim-the-cache"),
        "both staging tasks are listed; got:\n{stderr}",
    );
    assert!(
        stderr.contains("if one of them is yours, list what it stages:"),
        "the plural clause asks whether one of them is the reader's; got:\n{stderr}",
    );
    assert_eq!(
        lift_route(&stderr),
        vec!["jigc", "doc", "list", "--task", "<task-id>"],
        "the plural route carries the shared placeholder — no single id could be right",
    );
}
