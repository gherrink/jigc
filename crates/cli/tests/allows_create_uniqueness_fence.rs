//! Real-binary acceptance for the **`allows-create` type-uniqueness fence**
//! (the confidence-audit wave, sibling-hunt item 8 — class: role-binding).
//!
//! The create-gate (`engine::state::create_gated`) and the copy-on-write role
//! binding (`doc.rs::bind_role_on_copy_in`) both resolve a workflow's
//! `allows-create` entry **by doctype, first match** — a precondition that was
//! true of every shipped workflow but fenced nowhere, so a future workflow
//! declaring two entries of one type would silently first-match-bind and
//! dead-letter the rest. The fence lives where the set is defined (the
//! `load_workflow_def` front-matter parse — the dev-workflow's *fence the
//! assumption where the set is defined* rule), so every load door blocks.
//!
//! A mutated shipped-tree copy whose `single-task` declares `adr` twice exits
//! **non-zero at pack-load naming the workflow and the type**; an unmutated
//! copy loads clean (the omitting context stays inert). This drives the **real
//! `jigc` binary** end-to-end — the `catalog_shape_fence.rs` pattern.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-acuniq-{tag}-{}-{:?}",
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

/// Copy the shipped dev pack tree into a fresh temp dir and return the copy.
fn dev_pack_copy(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());
    dir
}

/// Replace `single-task`'s `allows-create:` line in a copied pack with a list
/// declaring `adr` **twice** — the duplicate-type mutation the fence must catch.
fn duplicate_adr_in_single_task(pack: &Path) {
    let path = pack.join("workflows").join("single-task.yaml");
    let body = fs::read_to_string(&path).expect("read the copied workflow");
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with("allows-create:") {
            hit = true;
            out.push_str("allows-create: [{type: adr, as: decision}, {type: adr, as: change}]");
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `single-task` workflow must carry an `allows-create:` line to mutate"
    );
    fs::write(&path, out).expect("write the mutated workflow");
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` layer.
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

const START: &[&str] = &[
    "start",
    "--workflow",
    "single-task",
    "type-uniqueness probe",
];

/// The fence: a workflow declaring two `allows-create` entries of one type is
/// refused at pack-load, naming the workflow and the duplicated doctype.
#[test]
fn a_duplicate_allows_create_type_blocks_at_pack_load() {
    let repo = TempDir::new("dup-repo");
    let home = TempDir::new("dup-home");
    let pack = dev_pack_copy("dup");
    init_repo(repo.path());
    duplicate_adr_in_single_task(pack.path());

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a duplicate-type `allows-create:` must block at pack-load; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("single-task"),
        "stderr must name the offending `single-task` workflow; got:\n{stderr}",
    );
    assert!(
        stderr.contains("adr"),
        "stderr must name the duplicated `adr` doctype; got:\n{stderr}",
    );
}

/// The omitting context: an **unmutated** dev-pack copy — every shipped
/// multi-entry list carries distinct types — loads clean through the same door,
/// so the fence is inert on the shipped packs.
#[test]
fn the_unmutated_dev_pack_passes_the_uniqueness_fence() {
    let repo = TempDir::new("clean-repo");
    let home = TempDir::new("clean-home");
    let pack = dev_pack_copy("clean");
    init_repo(repo.path());

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must pass the type-uniqueness fence; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}
