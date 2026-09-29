//! Real-binary regression — **a migration must not refuse an absence that already conforms**
//! (M46 Inc-4 T1; `design/corpus-migration.md` → The deterministic transform / The classifier's
//! holes).
//!
//! The transform's added-field arm had **two** value sources: a schema `default:` (spliced) and
//! a caller-threaded deriver (`with_stamp_default`). Everything else with no `default:` and no
//! `optional: true` was [`engine::transform::TransformError::Unsupported`] — including a field
//! declared `set:`, whose absence is **conformance-clean**: `validate::is_author_required` is
//! `default.is_none() && set.is_none()`, so `schema_conformance` never asks for it. The doc was
//! therefore blocked with the Framing-A route — *author the new required prose … then re-run* —
//! for a field **no author can write and no re-run can change**: a **permanent** dead end,
//! followed exactly.
//!
//! Driven here through the **shipped binary** over a real `JIGC_PACK_DIR` dev-pack copy at the
//! default docs-root: `adr` is bumped 2 → 3 by *adding back* its header `date` field
//! (`type: date, set: on-create`), and one committed, `date`-less, v2-stamped ADR sits at
//! `docs/decisions/`. The migration must **land** it — stamp 2 → 3, `date:` still absent, every
//! other byte untouched — never invent a date, and never block.
//!
//! Red at HEAD: `0 migrated, 0 already current, 1 blocked`, exit 1, `migrate-corpus.prose-needed`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-setfields-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — the faithful source the mutated copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// A dev-pack copy in which `adr` is bumped **2 → 3** by an **added `set:`-derived** header field:
///
/// - `schema-snapshots/adr.v2.yaml` — the prior (v2) shape: the shipped `adr.yaml` with the header
///   `date` line **deleted**;
/// - `schemas/adr.yaml` — **shipped verbatim**, so its `schema-hash` still matches the manifest and
///   the pack-load freeze gate stays quiet;
/// - `config/schema-manifest.yaml` — the `adr` entry's `schema-version` bumped `2 → 3`.
fn pack_adding_a_set_derived_adr_field(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());

    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen("      - { id: date, type: date, set: on-create }\n", "", 1);
    assert_ne!(
        current, prior,
        "adr.yaml must declare a header `date` field with `set: on-create`",
    );
    fs::write(
        dir.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");

    let manifest_path = dir.path().join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let bumped = manifest.replacen(
        "- type: adr\n    schema-version: 2",
        "- type: adr\n    schema-version: 3",
        1,
    );
    assert_ne!(
        manifest, bumped,
        "the manifest must carry `adr` at schema-version 2",
    );
    fs::write(&manifest_path, bumped).expect("write the bumped manifest");

    dir
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer the cascade
/// expects (the `docs-root` knob is left at its shipped default, `docs/`).
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// A conformant, **v2-stamped**, **`date`-less** ADR at the default docs-root home. Its absence of
/// a `date:` line is the whole fixture: under the v3 schema the field is `set:`-derived, so the
/// absence still conforms and the migration has nothing to write.
const V2_ADR: &str = "\
---
status: accepted
schema-version: 2
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// Commit [`V2_ADR`] at `docs/decisions/alpha.md` — the corpus the migration runs over.
fn commit_v2_adr(repo: &Path) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("alpha.md"), V2_ADR).expect("write the v2 adr");
    let out = Command::new("git")
        .args(["add", "."])
        .current_dir(repo)
        .output()
        .expect("git add");
    assert!(out.status.success(), "git add failed");
    let out = Command::new("git")
        .args(["commit", "-q", "-m", "seed a v2 adr"])
        .current_dir(repo)
        .output()
        .expect("git commit");
    assert!(out.status.success(), "git commit failed");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
}

/// **A `set:`-derived added field MIGRATES the doc, it does not block it.** The absence of a
/// `set:` field already conforms, and no author can fill it — so the Framing-A route the doc used
/// to collect (*author the prose … then re-run*) was a permanent dead end. The fold is a byte
/// no-op and the doc lands at the current stamp.
#[test]
fn a_set_derived_added_field_migrates_the_doc_instead_of_blocking_it() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_adding_a_set_derived_adr_field("pack");
    init_repo(repo.path());
    commit_v2_adr(repo.path());

    let out = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "the migration must succeed — nothing about this doc is un-migratable; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("1 migrated") && !stdout.contains("1 blocked"),
        "the doc migrates, and nothing is blocked; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("migrate-corpus.prose-needed"),
        "no prose is needed — a `set:` field is not the author's to write; stdout:\n{stdout}",
    );

    // The stamp is the ONLY byte delta: `date:` stays absent (never invented), and no other line
    // moves.
    let after = fs::read_to_string(repo.path().join("docs").join("decisions").join("alpha.md"))
        .expect("read the adr");
    assert_eq!(
        after,
        V2_ADR.replace("schema-version: 2", "schema-version: 3"),
        "the migrated doc differs from its v2 form ONLY in the stamp; stdout:\n{stdout}",
    );
    assert!(
        !after.contains("date:"),
        "the migration must not invent a date for a `set: on-create` field; got:\n{after}",
    );
}

// ---------------------------------------------------------------------------------------------
// T2 — the report NAMES what it left unfilled (the loudness rider).
//
// A byte no-op that says nothing is its own hazard: a doctype author who adds a `set:`-bearing
// field expecting the corpus to carry a value gets a clean `1 migrated` and silence, because the
// doc conforms without it. So the run reports every added leaf whose declared field carries a
// `set:` deriver and no `default:` — one advisory per leaf, on **both** surfaces, targeted at
// `<path>#<section>/<field>` so two unfilled leaves in one doc discriminate — while `blocked`
// stays empty and the exit stays 0 (the advisory is a report, never a refusal:
// `design/command-output-contract.md` → the `migrate-corpus.*` sub-table + Evolution posture,
// *The M46 additive key: `unfilled`*).
// ---------------------------------------------------------------------------------------------

/// The advisory's code — one per unfilled `set:`-derived leaf of a migrated doc.
const UNFILLED_CODE: &str = "migrate-corpus.set-field-unfilled";

/// A dev-pack copy in which `changelog` is bumped **2 → 3** by an **added `set:`-derived leaf of
/// its `releases` item block** — the T1 pack's twin at the **item locus** (the second of the two
/// loops whose disagreement produced the M42 holes):
///
/// - `schema-snapshots/changelog.v2.yaml` — the prior (v2) shape: the shipped `changelog.yaml`
///   with the item-block `date` line **deleted**;
/// - `schemas/changelog.yaml` — shipped verbatim (the freeze gate stays quiet);
/// - `config/schema-manifest.yaml` — the `changelog` entry's `schema-version` bumped `2 → 3`.
fn pack_adding_a_set_derived_changelog_leaf(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());

    let current = fs::read_to_string(dir.path().join("schemas").join("changelog.yaml"))
        .expect("read the copied changelog.yaml");
    let prior = current.replacen(
        "        - { id: date, type: date, set: on-create }\n",
        "",
        1,
    );
    assert_ne!(
        current, prior,
        "changelog.yaml must declare an item-block `date` field with `set: on-create`",
    );
    fs::write(
        dir.path()
            .join("schema-snapshots")
            .join("changelog.v2.yaml"),
        prior,
    )
    .expect("write the changelog.v2 snapshot");

    bump_manifest(dir.path(), "changelog", 2, 3);
    dir
}

/// A dev-pack copy in which `adr` is bumped **2 → 3** by an added leaf that carries **no `set:`
/// at all** — the optional `supersedes` ref. The **control**: the fold is the same byte no-op
/// (an optional ref's absence conforms, M46 Inc-4 T1), so if the report spoke about *every*
/// no-op leaf rather than the `set:`-derived ones it would speak here too.
fn pack_adding_an_optional_ref(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());

    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen(
        "      - { id: supersedes, type: ref, to: adr, card: \"0..*\", inverse: superseded-by }\n",
        "",
        1,
    );
    assert_ne!(
        current, prior,
        "adr.yaml must declare the optional `supersedes` ref",
    );
    fs::write(
        dir.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");

    bump_manifest(dir.path(), "adr", 2, 3);
    dir
}

/// Bump `ty`'s manifest `schema-version` `from → to` in the copied pack at `pack`.
fn bump_manifest(pack: &Path, ty: &str, from: u32, to: u32) {
    let manifest_path = pack.join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let bumped = manifest.replacen(
        &format!("- type: {ty}\n    schema-version: {from}"),
        &format!("- type: {ty}\n    schema-version: {to}"),
        1,
    );
    assert_ne!(
        manifest, bumped,
        "the manifest must carry `{ty}` at schema-version {from}",
    );
    fs::write(&manifest_path, bumped).expect("write the bumped manifest");
}

/// A conformant, **v2-stamped** `changelog` at its literal root placement home, whose one cut
/// release carries **no `date:` bullet** — the item-locus twin of [`V2_ADR`].
const V2_CHANGELOG: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

## Releases

### 1.0.0  {#1-0-0}

#### added  {#added}

- the first cut
";

/// Commit [`V2_CHANGELOG`] at the root `CHANGELOG.md` placement home.
fn commit_v2_changelog(repo: &Path) {
    fs::write(repo.join("CHANGELOG.md"), V2_CHANGELOG).expect("write the v2 changelog");
    let out = Command::new("git")
        .args(["add", "."])
        .current_dir(repo)
        .output()
        .expect("git add");
    assert!(out.status.success(), "git add failed");
    let out = Command::new("git")
        .args(["commit", "-q", "-m", "seed a v2 changelog"])
        .current_dir(repo)
        .output()
        .expect("git commit");
    assert!(out.status.success(), "git commit failed");
}

/// The one `unfilled[]` member of a `--format json` report, asserted down to its stable
/// `(code, target)` key, its severity, and the presence of a route — plus the two facts that
/// make it a *report* and not a refusal: `blocked` empty and the exit 0.
fn sole_unfilled(out: &std::process::Output, target: &str) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "an unfilled `set:` leaf is reported, never blocking — the run exits 0; stdout:\n{stdout}\
         \nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let report: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|err| panic!("the report is JSON: {err}"));
    assert_eq!(
        report["blocked"].as_array().map(Vec::len),
        Some(0),
        "the advisory rides its own key — `blocked`, whose emptiness IS the exit rule, stays \
         empty; report:\n{report:#}",
    );
    let unfilled = report["unfilled"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries an `unfilled[]` member; got:\n{report:#}"));
    assert_eq!(
        unfilled.len(),
        1,
        "exactly one leaf was left unfilled; report:\n{report:#}",
    );
    let finding = unfilled[0].clone();
    assert_eq!(finding["code"], UNFILLED_CODE);
    assert_eq!(finding["severity"], "advisory");
    assert_eq!(
        finding["key"],
        serde_json::json!({ "code": UNFILLED_CODE, "target": target }),
        "the stable `(code, target)` key names the leaf, so two unfilled leaves in one doc \
         discriminate; finding:\n{finding:#}",
    );
    assert!(
        finding["route"].as_str().is_some_and(|r| !r.is_empty()),
        "every finding routes (the advisory-route floor); finding:\n{finding:#}",
    );
    finding
}

/// **The simple/header locus — the report names the `date` it left unfilled.** The same run T1
/// pins as *migrating* (byte no-op, exit 0) must also **say** what it did not write: the
/// `--format json` envelope carries one `unfilled[]` member keyed at
/// `docs/decisions/alpha.md#status/date`, and the text surface names the same leaf on its own
/// line with the code and the route. `blocked` stays empty and the exit stays 0.
#[test]
fn the_report_names_the_set_field_it_left_unfilled_at_the_simple_locus() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_adding_a_set_derived_adr_field("pack");
    init_repo(repo.path());
    commit_v2_adr(repo.path());

    // The machine surface first, over the **dry run** — which prints the identical triage an
    // applying run prints, so the applying run below is left to prove the text half over the
    // real write.
    let json = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--dry-run", "--format", "json"],
    );
    let finding = sole_unfilled(&json, "docs/decisions/alpha.md#status/date");
    // `set: on-create` is **author-overridable** (`engine::schema::is_machine_maintained_absolute`
    // is false for it), so the route is a human direction at the write path — not the
    // "no action needed" an absolute takes, and not a mechanical argv: this report holds no
    // task id, so no runnable command can be composed from it.
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("jigc doc set-field"),
        "an author-overridable `set:` leaf routes at the write path; route: {route}",
    );

    // The text surface, over the applying run.
    let out = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "the run exits 0; stdout:\n{stdout}");
    assert!(
        stdout.contains("docs/decisions/alpha.md#status/date"),
        "the text surface names the unfilled leaf; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains(UNFILLED_CODE),
        "with the code a driver keys on; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("route:"),
        "and the route on its own line; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("1 left unfilled"),
        "and the headline counts it, so a reader scanning one line is not told the run wrote \
         everything the schema declares; stdout:\n{stdout}",
    );
}

/// **The item locus — the same statement about a leaf inside a repeatable item block.** The
/// twin loop that produced the M42 holes by disagreeing with its sibling: `changelog`'s
/// `releases/date`, added back at schema-version 3 over a committed release that predates it.
#[test]
fn the_report_names_the_set_leaf_it_left_unfilled_at_the_item_locus() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_adding_a_set_derived_changelog_leaf("pack");
    init_repo(repo.path());
    commit_v2_changelog(repo.path());

    let json = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--dry-run", "--format", "json"],
    );
    let finding = sole_unfilled(&json, "CHANGELOG.md#releases/date");

    // **The advisory must not compose an address the item locus cannot have** (M46 completion
    // audit, finding F2). `date` is declared per **item** of `releases`, so its write address is
    // `#<section>/<item-id>/<field>` — a `#releases/date` names a section-level leaf that does
    // not exist, and a reader copying it out of the route reaches nothing. The report holds no
    // item id (and must not emit one per item: a repeatable section with **zero** items still has
    // an unfilled leaf, and a per-item finding would go silent on exactly the corpus a doctype
    // author most needs to hear about). So the route names the write-address **form** and routes
    // at the read that enumerates the real ids — it fabricates no address.
    let route = finding["route"].as_str().expect("a route");
    assert!(
        !route.contains("set-field <doc-address>#releases/date"),
        "an item-locus leaf has no section-level write address — the route must not compose \
         one; route: {route}",
    );
    assert!(
        route.contains("#<section>/<item-id>/date") || route.contains("#releases/<item-id>/date"),
        "the route names the item-qualified write-address form, so a reader knows the id hop \
         is theirs to fill; route: {route}",
    );
    assert!(
        route.contains("jigc doc show"),
        "and routes at the read that enumerates the real item ids, since the report holds \
         none; route: {route}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains("item"),
        "the message states the locus — the absence is per item of `releases`, not a \
         section-level one; message: {message}",
    );

    let out = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success() && stdout.contains("1 migrated"),
        "the release migrates — no item gains a fabricated `date:` bullet; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("CHANGELOG.md#releases/date") && stdout.contains(UNFILLED_CODE),
        "the text surface names the unfilled item leaf; stdout:\n{stdout}",
    );

    // The stamp is the ONLY byte delta at the item locus too.
    let after = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md");
    assert_eq!(
        after,
        V2_CHANGELOG.replace("schema-version: 2", "schema-version: 3"),
        "the migrated changelog differs from its v2 form ONLY in the stamp; stdout:\n{stdout}",
    );
}

/// **The control — a migration with no `set:` leaf says nothing about unfilled fields.** The
/// added leaf here is the optional `supersedes` ref: the same conformance-clean absence, the
/// same byte no-op, and **no** `set:` deriver — so the word is absent from the text entirely and
/// an ordinary run's bytes are unchanged.
#[test]
fn a_migration_with_no_set_leaf_says_nothing_about_unfilled_fields() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_adding_an_optional_ref("pack");
    init_repo(repo.path());
    commit_v2_adr(repo.path());

    let out = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success() && stdout.contains("1 migrated"),
        "the optional ref folds to zero bytes and the doc migrates; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("unfilled") && !stdout.contains(UNFILLED_CODE),
        "nothing was left unfilled, so the report says nothing about it; stdout:\n{stdout}",
    );

    let json = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--dry-run", "--format", "json"],
    );
    let report: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&json.stdout)).expect("the report is JSON");
    assert_eq!(
        report["unfilled"].as_array().map(Vec::len),
        Some(0),
        "the key is present-always and empty — absent-versus-empty is not a discrimination a \
         driver should infer; report:\n{report:#}",
    );
}

/// A conformant **v0** (stamp-less) ADR — the pre-stamp corpus state. It parses against the
/// shipped `adr.v1` prior shape *and* the current one, so the discriminator reads it **managed**,
/// and its migration's one change is the schema-version stamp itself.
const V0_ADR: &str = "\
---
status: accepted
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// **The stamp is a `set:` field with a threaded-in value — and is never reported unfilled.**
/// The `schema-version` field declares `set: schema-version` and no `default:` **in the pack**;
/// the CLI hands the manifest version in *as* a `default:` before the fold (`with_stamp_default`),
/// which is the whole extension point T1 left untouched. So the derivation must run against the
/// schema the fold used, not the raw pack shape — fed the raw one it would report the stamp it
/// had just written, on every v0 doc in every corpus.
///
/// Driven over the **stock** pack (nothing mutated): a v0 ADR whose only migration is the stamp
/// add lands at `schema-version: 2` with `unfilled` empty.
#[test]
fn the_stamp_the_cli_threads_a_value_into_is_never_reported_unfilled() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = embedded_pack_tree();
    init_repo(repo.path());
    let dir = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("alpha.md"), V0_ADR).expect("write the v0 adr");
    for args in [
        &["add", "."][..],
        &["commit", "-q", "-m", "seed a v0 adr"][..],
    ] {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
            .output()
            .expect("run git");
        assert!(out.status.success(), "git {args:?} failed");
    }

    let out = jigc(repo.path(), home.path(), &pack, &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success() && stdout.contains("1 migrated"),
        "the v0 doc migrates — the stamp is added at the manifest version; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("unfilled"),
        "the stamp was FILLED — the CLI threaded its value in as a `default:`; stdout:\n{stdout}",
    );
    let after = fs::read_to_string(dir.join("alpha.md")).expect("read the adr");
    assert!(
        after.contains("schema-version: 2"),
        "and the value it was filled with is the manifest version; got:\n{after}",
    );
}
