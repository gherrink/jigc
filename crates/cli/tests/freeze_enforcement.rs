//! Real-binary acceptance for the M33 **pack-load freeze gate** (T3): a
//! `JIGC_PACK_DIR` on-disk dev-pack copy whose schema *shape* drifted from the
//! shipped `config/schema-manifest.yaml` without a version bump makes a `jigc`
//! command exit **non-zero**, naming the schema-hash mismatch; an unmutated copy
//! composes clean; a manifest-less copy is unaffected (the same drift passes once
//! the freeze manifest is dropped).
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract, not a reconstructed equivalent. It is the runtime
//! sibling of the build-time freeze gate
//! (`pack::shipped_schema_manifest_matches_the_frozen_doctype_set`); the design is
//! `design/corpus-migration.md` → The enforcement gate fires at pack-load
//! (review Finding 3) + `design/worked-examples.md` → the freeze-enforcement flow.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-freeze-{tag}-{}-{:?}",
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

/// The embedded dev pack tree — the faithful source the on-disk copy mirrors.
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

/// Copy the dev pack into a fresh temp dir and return the copy's root.
fn dev_pack_copy(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());
    dir
}

/// Mutate the copied `adr` schema's **shape** (its `location:`) — a hash-affecting
/// change that bumps no `schema-version`, the un-migrated schema change the freeze
/// forbids. The manifest is deliberately left unbumped.
fn drift_adr_schema(pack: &Path) {
    let schema = pack.join("schemas").join("adr.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied adr.yaml");
    let drifted = body.replacen("location: decisions/", "location: adr-records/", 1);
    assert_ne!(
        body, drifted,
        "adr.yaml must declare `location: decisions/`"
    );
    fs::write(&schema, drifted).expect("write the drifted adr.yaml");
}

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
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

/// Run `jigc start --workflow single-task "<intent>"` with `cwd = repo`,
/// `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_start(repo: &Path, home: &Path, pack: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", "freeze-gate probe"])
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// A schema-shape change with no manifest bump is **blocked at pack-load**: the
/// composing `jigc start` exits non-zero, and stderr names the schema-hash
/// mismatch on the drifted doctype.
#[test]
fn schema_shape_drift_without_manifest_bump_is_blocked() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("drift");
    init_repo(repo.path());
    drift_adr_schema(pack.path());

    let out = run_start(repo.path(), home.path(), pack.path());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a drifted schema must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch"),
        "stderr must name the schema-hash mismatch; got:\n{stderr}",
    );
    assert!(
        stderr.contains("adr"),
        "stderr must name the drifted `adr` doctype; got:\n{stderr}",
    );
}

/// The control: an **unmutated** dev-pack copy matches its shipped manifest, so the
/// freeze gate is inert and `jigc start` composes clean.
#[test]
fn unmutated_pack_copy_composes_clean() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("clean");
    init_repo(repo.path());

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must compose clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The omitting context: a **manifest-less** pack is unchecked. The *same* schema
/// drift that the manifest-bearing copy blocks composes clean once
/// `config/schema-manifest.yaml` is dropped — proving the manifest is the gate, and
/// that a seeded / composed pack with no manifest stays inert (never errors).
#[test]
fn manifest_less_pack_is_unaffected() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("nomanifest");
    init_repo(repo.path());
    drift_adr_schema(pack.path());
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "a manifest-less pack must be unaffected by the freeze gate; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}
