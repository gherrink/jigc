//! Real-binary regression — **the empty-diff backstop must fire at the shipped default
//! `docs-root: docs/`** (M42 Inc-5, validation finding).
//!
//! The backstop (`design/corpus-migration.md` → The empty-diff backstop — no silent bump) was
//! **inert in production**. `CascadeDefs::all_schemas` docs-root-resolves the current shape
//! (`location: docs/decisions/`) while the prior-schema snapshot the below-version arm loads
//! stores its `location:` **raw** (`decisions/`) — and `engine::schema_diff` compares the two
//! homes verbatim. So *every* below-version migration of a `location:`-bearing doctype carried a
//! **spurious** `SchemaChange::Relocated`, the residual is gated on an otherwise-empty diff, and
//! the backstop (plus its `PresentationOnly` sibling) could never be reached: an unclassifiable
//! structural change **silently restamped** the corpus at the new version while leaving every
//! instance non-conformant — the exact strand the backstop exists to close. It fired only under
//! `docs-root: .` (the flat layout the core unit fixtures happen to seed), so the same schema
//! change had opposite verdicts decided purely by a project-config knob.
//!
//! This drives the **shipped binary** over a real `JIGC_PACK_DIR` dev-pack copy at the **default**
//! docs-root: `adr` is bumped 2→3 with a change **no transform kind classifies** (a field's
//! `type:`, which the structural projection carries), and one committed v2-stamped ADR sits at
//! `docs/decisions/`. The migration must **refuse** it with the build-the-transform-kind route,
//! bytes untouched, stamp still 2.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-backstop-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — the faithful source the mutated copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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

/// A dev-pack copy in which `adr` is bumped **2 → 3** with an **unclassifiable** structural
/// change:
///
/// - `schema-snapshots/adr.v2.yaml` — the prior (v2) shape: the shipped `adr.yaml` with the
///   header `date` field declared `type: string`;
/// - `schemas/adr.yaml` — **shipped verbatim** (`type: date`), so its `schema-hash` still
///   matches the manifest and the pack-load freeze gate stays quiet;
/// - `config/schema-manifest.yaml` — the `adr` entry's `schema-version` bumped `2 → 3`.
///
/// A field's `type:` is **inside** the conformance-relevant structural projection
/// (`erase_field` keeps it) and **no** transform kind classifies a type change — the textbook
/// backstop shape.
fn pack_with_an_unclassifiable_adr_bump(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());

    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen(
        "{ id: date, type: date, set: on-create }",
        "{ id: date, type: string, set: on-create }",
        1,
    );
    assert_ne!(
        current, prior,
        "adr.yaml must declare the header `date` field as `type: date`",
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

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer the
/// cascade expects (the `docs-root` knob is left at its shipped default, `docs/`).
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

/// A conformant, **v2-stamped** ADR at the default docs-root home (`docs/decisions/`).
const V2_ADR: &str = "\
---
status: accepted
date: 2026-06-25
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

/// **The backstop fires at the DEFAULT `docs-root: docs/`.** An unclassifiable structural change
/// (a field's `type:`) between the doc's stamped version and the manifest's blocks the doc with
/// the *build the transform kind* route, and never restamps it.
///
/// Red before the fix: the docs-root-resolved `to` vs the raw snapshot `from` produced a spurious
/// `Relocated`, the residual was skipped, and the run reported `1 migrated` — the ADR's only byte
/// delta `schema-version: 2` → `3`, i.e. the silent stamp-bump over a non-conformant corpus the
/// backstop is *defined* to prevent.
#[test]
fn the_empty_diff_backstop_fires_at_the_default_docs_root() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_with_an_unclassifiable_adr_bump("pack");
    init_repo(repo.path());
    commit_v2_adr(repo.path());

    let out = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    // **A REFUSED MIGRATION EXITS NON-ZERO** (M42 completion audit, Finding 2). This asserted
    // `success()` on the rationale that *a routed block is an expected interim state, not a run
    // failure* — but the run exited 0 having migrated **nothing**, so the loudest refusal in the
    // verb was inaudible to a machine, and `validate` (exit 1, *run `jigc migrate-corpus`*) →
    // `migrate-corpus` (exit 0) → `validate` was an infinite CI loop. The doc is an interim
    // state; the *run* is a failure — it did not do what it was asked.
    assert!(
        !out.status.success(),
        "a refused migration must exit NON-ZERO — it migrated nothing; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        stdout.contains("0 migrated") && stdout.contains("1 blocked"),
        "the unclassifiable change is REFUSED, never migrated; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("blocked    docs/decisions/alpha.md"),
        "the refused doc is named at its docs-root home; stdout:\n{stdout}",
    );
    // The refusal is a real finding: a `migrate-corpus.*` code a driver keys on, the diagnosis,
    // and — separately — the route naming the real repair (build the kind), never a prose dead end.
    assert!(
        stdout.contains("migrate-corpus.unclassified-change"),
        "the refusal carries its machine code; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("no transform kind") && stdout.contains("build the transform kind"),
        "the route names the real repair — build the kind — not a prose-authoring dead end; \
         stdout:\n{stdout}",
    );

    // No silent bump: the doc is byte-identical and still stamped 2.
    let after = fs::read_to_string(repo.path().join("docs").join("decisions").join("alpha.md"))
        .expect("read the adr");
    assert_eq!(
        after, V2_ADR,
        "the refused doc is byte-identical — the stamp never flipped; stdout:\n{stdout}",
    );
}

/// The **control**: the same pack copy, the same doc, migrated with `docs-root` set to the flat
/// repo-root layout (`.`) — the one topology the pre-fix code got right. The verdict must be
/// **identical** (blocked, same route): a project-config knob decides *where* a doc lives, never
/// *whether* its schema change is classifiable. Pre-fix these two arms disagreed — same change,
/// opposite verdicts.
#[test]
fn the_verdict_is_the_same_under_a_flat_docs_root() {
    let repo = TempDir::new("repo-flat");
    let home = TempDir::new("home");
    let pack = pack_with_an_unclassifiable_adr_bump("pack-flat");
    init_repo(repo.path());

    // The flat layout: `docs-root: .` — the ADR homes at `decisions/`.
    let set = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["config", "set", "docs-root", "."],
    );
    assert!(
        set.status.success(),
        "`jigc config set docs-root .`; stderr:\n{}",
        String::from_utf8_lossy(&set.stderr),
    );
    let dir = repo.path().join("decisions");
    fs::create_dir_all(&dir).expect("mk decisions/");
    fs::write(dir.join("alpha.md"), V2_ADR).expect("write the v2 adr");

    let out = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Same verdict, same exit: the flat layout refuses the doc, so it too exits non-zero.
    assert!(
        !out.status.success(),
        "the flat-layout run refuses the doc, so it exits NON-ZERO; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        stdout.contains("0 migrated") && stdout.contains("1 blocked"),
        "the flat layout reaches the same verdict as the default docs-root; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("blocked    decisions/alpha.md")
            && stdout.contains("build the transform kind"),
        "the flat layout carries the same build-the-kind route; stdout:\n{stdout}",
    );
    assert_eq!(
        fs::read_to_string(dir.join("alpha.md")).expect("read the adr"),
        V2_ADR,
        "the refused doc is byte-identical under the flat layout too",
    );
}

/// **A refusal is machine-readable, and the detect→migrate loop terminates** (M42 completion
/// audit, Finding 2).
///
/// The three Increment-5 refusal classes are the ones the roadmap calls *"refuse loudly"*, and
/// they were **inaudible to a machine**: the run exited **0** having migrated nothing, and the
/// blocked list was untyped `[path, route]` string tuples — no `code`, no `key`, no
/// `(code, target)` — i.e. entirely outside the finding-key contract Increment 9 exists to close.
/// Combined with Increment 4's `validate` exit flip that is an **infinite CI loop**: `validate` →
/// exit 1, *"run `jigc migrate-corpus`"* → `migrate-corpus` → exit **0 (success)** → `validate` →
/// exit 1 → forever, with **nothing machine-readable naming why**.
///
/// So a refused migration exits non-zero, and its blocked entries are real `Finding`s riding the
/// structural serialization seam — each with a `migrate-corpus.*` code, the stable `(code, target)`
/// key, a message and a route.
#[test]
fn a_refused_migration_exits_non_zero_and_its_blocked_entries_are_findings() {
    let repo = TempDir::new("repo-json");
    let home = TempDir::new("home");
    let pack = pack_with_an_unclassifiable_adr_bump("pack-json");
    init_repo(repo.path());
    commit_v2_adr(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);

    // 1. THE EXIT — a run that migrated nothing and refused a doc is not a success.
    assert!(
        !out.status.success(),
        "a refused migration exits NON-ZERO (pre-fix: 0, so validate→migrate→validate looped \
         forever); stdout:\n{stdout}",
    );

    // 2. THE ENVELOPE — the blocked entry is a Finding, not a `[path, route]` string tuple.
    let report: serde_json::Value =
        serde_json::from_str(&stdout).expect("`--format json` emits valid JSON");
    let blocked = report["blocked"]
        .as_array()
        .unwrap_or_else(|| panic!("`blocked` is an array; got: {report}"));
    assert_eq!(blocked.len(), 1, "one doc was refused; got: {report}");
    let finding = &blocked[0];

    assert_eq!(finding["severity"], "blocking");
    assert_eq!(finding["code"], "migrate-corpus.unclassified-change");
    // THE STABLE KEY — `(code, target)`, the pair a driver dedupes/tracks a finding by. `target`
    // is the doc's path: the subject of a migration refusal is the FILE the verb could not
    // rewrite, and during a relocation two files can contest one `<type>:<slug>` URI (which is
    // exactly what the destination-collision refusal reports), so the path is what discriminates.
    assert_eq!(finding["key"]["code"], "migrate-corpus.unclassified-change");
    assert_eq!(finding["key"]["target"], "docs/decisions/alpha.md");
    // The route is a route — the repair, not the diagnosis fused into it.
    assert!(
        finding["route"]
            .as_str()
            .expect("a refusal carries a route")
            .contains("build the transform kind"),
        "the route names the repair; got: {finding}",
    );
    assert!(
        finding["message"]
            .as_str()
            .expect("a refusal carries a message")
            .contains("no transform kind"),
        "the message carries the diagnosis; got: {finding}",
    );

    // 3. NOTHING SILENTLY MIGRATED — the refusal is the whole outcome.
    assert!(
        report["migrated"]
            .as_array()
            .expect("`migrated` is an array")
            .is_empty(),
        "nothing migrated; got: {report}",
    );
}
