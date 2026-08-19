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
