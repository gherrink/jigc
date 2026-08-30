//! Real-binary acceptance for the M49 Increment 3 **schema-load strictness
//! repair** (T3): a mis-keyed schema leaf is *refused*, not absorbed.
//!
//! Four mappings in the schema model cannot carry `deny_unknown_fields` —
//! `Section` (serde forbids it beside the flattened `SectionBody`,
//! `crates/engine/src/schema.rs`) and the untagged struct variants it flattens
//! into. So a stray key was either swallowed silently, or — worse — failed the
//! `Repeatable` variant and fell through to `SectionBody::Simple { slot: None,
//! fields: [] }`, **erasing the whole section from every surface at exit 0**.
//! Verified at HEAD before the fix: a `patern:` typo on `commit.trailers.key`
//! made `jigc doc schema commit` print `summary`, `body` and *no* `trailers`,
//! exit 0.
//!
//! The fixture is a **manifest-less** `JIGC_PACK_DIR` pack copy — the exposure
//! the deferral names verbatim (*a third-party or project-layer pack is
//! authored*), and the state where nothing else backstops the typo: all 16
//! shipped doctypes are manifest-governed, so on a shipped pack the freeze
//! gate would fire first and hide the hole.
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract (the `freeze_enforcement.rs` /
//! `catalog_shape_fence.rs` pattern). Design:
//! `design/document-type-schema.md` → On-disk definition format;
//! `implementation/decisions-pending.md` → the mis-keyed-leaf entry, discharged
//! as D5's precondition.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-schemastrict-{tag}-{}-{:?}",
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

/// The embedded dev pack tree — the faithful source the on-disk copies mirror.
fn dev_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
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

/// Copy the dev pack into a fresh temp dir and **drop its freeze manifest**, so
/// the copy stands in for the pack a third party or the project layer authors:
/// nothing but the loader itself refuses a typo there.
fn manifest_less_pack(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&dev_pack_tree(), dir.path());
    fs::remove_file(dir.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");
    dir
}

/// Replace `needle` with `replacement` (exactly once) in the copied pack's
/// `schemas/<doctype>.yaml` — the mis-key mutations.
fn mutate_schema(pack: &Path, doctype: &str, needle: &str, replacement: &str) {
    let path = pack.join("schemas").join(format!("{doctype}.yaml"));
    let body = fs::read_to_string(&path).expect("read the copied schema");
    let mutated = body.replacen(needle, replacement, 1);
    assert_ne!(
        body, mutated,
        "the shipped `{doctype}` schema must carry `{needle}` to mutate"
    );
    fs::write(&path, mutated).expect("write the mutated schema");
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project
/// layer the cascade expects.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_with_pack(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// Mutate a manifest-less dev-pack copy, run `jigc doc schema <doctype>` against
/// it, and assert the load **blocks** with stderr naming the section locus and
/// the offending key — and never falling back on serde's untagged non-message.
fn assert_blocks(
    tag: &str,
    doctype: &str,
    needle: &str,
    replacement: &str,
    locus: &str,
    key: &str,
) {
    let repo = TempDir::new(&format!("{tag}-repo"));
    let home = TempDir::new(&format!("{tag}-home"));
    let pack = manifest_less_pack(tag);
    init_repo(repo.path());
    mutate_schema(pack.path(), doctype, needle, replacement);

    let out = run_with_pack(
        repo.path(),
        home.path(),
        pack.path(),
        &["doc", "schema", doctype],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a mis-keyed `{doctype}` schema leaf must exit non-zero; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains(locus),
        "stderr must name the `{locus}` locus; got:\n{stderr}",
    );
    assert!(
        stderr.contains(key),
        "stderr must name the offending `{key}` key; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("did not match any variant"),
        "stderr must not fall back on serde's untagged non-message; got:\n{stderr}",
    );
}

/// **The reported repro.** A `patern:` typo on `commit.trailers.key` erased the
/// whole `trailers` section from `jigc doc schema commit` at exit 0. It now
/// exits non-zero naming the section and the key.
#[test]
fn mis_keyed_repeatable_field_leaf_blocks_instead_of_erasing_the_section() {
    assert_blocks(
        "repeatable-field",
        "commit",
        "- { id: key, type: string }",
        "- { id: key, type: string, patern: \"^[A-Z]\" }",
        "trailers",
        "patern",
    );
}

/// The sibling found by driving, in no ledger: an untagged **struct variant**
/// (`Leaf::Slot`) silently ignoring a stray key — `spec.criteria/statement`
/// loaded clean at exit 0 with the typo intact.
#[test]
fn stray_key_on_a_slot_leaf_blocks() {
    assert_blocks(
        "slot-leaf",
        "spec",
        "- { id: statement, slot: { hint: \"The criterion, testably phrased.\" } }",
        "- { id: statement, slot: { hint: \"The criterion, testably phrased.\" }, bogus: 1 }",
        "criteria",
        "bogus",
    );
}

/// The **simple-section** case: the same `patern:` typo on a header field errored
/// before, but as serde's twice-repeated *"data did not match any variant of
/// untagged enum SectionBody"*, naming neither the section nor the key. It now
/// reports the same located message the repeatable case does.
#[test]
fn mis_keyed_simple_section_field_reports_the_located_message() {
    assert_blocks(
        "simple-field",
        "commit",
        "- { id: scope, type: string, optional: true }",
        "- { id: scope, type: string, optional: true, patern: \"^[a-z]\" }",
        "header",
        "patern",
    );
}

/// The **omitting context**: an *unmutated* manifest-less copy loads clean, and
/// `include:` still resolves — the shape check runs on the include-expanded
/// value, so `changelog`'s shared `change-group` fragment still splices in at
/// both its sites.
#[test]
fn include_still_resolves_and_an_unmutated_pack_loads_clean() {
    let repo = TempDir::new("clean-repo");
    let home = TempDir::new("clean-home");
    let pack = manifest_less_pack("clean");
    init_repo(repo.path());

    let out = run_with_pack(
        repo.path(),
        home.path(),
        pack.path(),
        &["doc", "schema", "changelog"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "an unmutated manifest-less pack must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    // The fragment's two leaves, spliced in at the staging site and again inside
    // each release's nested `changes` group.
    assert!(
        stdout.contains("category") && stdout.contains("notes"),
        "the included `change-group` fragment's leaves must still resolve; got:\n{stdout}",
    );
}

/// **No re-pin.** Every schema both packs ship — and every versioned snapshot
/// the corpus migration reads — loads unchanged under the strict check: 17
/// schemas + 6 snapshots, all `Ok`. A stray key in any of them would redden
/// here rather than at an adopter.
#[test]
fn every_shipped_schema_and_snapshot_loads_unchanged() {
    let field_types: Vec<engine::schema::PackTypeDecl> = serde_yaml_ng::from_str(
        &fs::read_to_string(dev_pack_tree().join("config").join("field-types.yaml"))
            .expect("read the dev pack's field-type declarations"),
    )
    .expect("parse the dev pack's field-type declarations");

    let mut loaded = 0usize;
    for pack in [dev_pack_tree(), methodology_pack_tree()] {
        for dir in ["schemas", "schema-snapshots"] {
            let home = pack.join(dir);
            for entry in fs::read_dir(&home).expect("read the schema directory") {
                let path = entry.expect("dir entry").path();
                if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
                    continue;
                }
                let bytes = fs::read(&path).expect("read the shipped schema");
                engine::schema::load_schema_with_types(&bytes, &field_types)
                    .unwrap_or_else(|e| panic!("{} must load unchanged: {e}", path.display()));
                loaded += 1;
            }
        }
    }
    assert_eq!(
        loaded, 23,
        "both packs ship 17 schemas + 6 versioned snapshots",
    );
}

/// The **freeze** is unmoved: an unmutated, *manifest-bearing* dev-pack copy
/// still matches every `schema-hash` its `config/schema-manifest.yaml` pins, so
/// the strict check re-pinned nothing. `jigc doc schema commit` composes clean
/// and still lists the `trailers` section the repro erased.
#[test]
fn the_shipped_pack_still_matches_its_frozen_manifest() {
    let repo = TempDir::new("frozen-repo");
    let home = TempDir::new("frozen-home");
    let pack = TempDir::new("frozen-pack");
    copy_tree(&dev_pack_tree(), pack.path());
    init_repo(repo.path());

    let out = run_with_pack(
        repo.path(),
        home.path(),
        pack.path(),
        &["doc", "schema", "commit"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "the shipped pack must still match its frozen manifest; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        stdout.contains("trailers"),
        "the `trailers` section must be present, not erased; got:\n{stdout}",
    );
}
