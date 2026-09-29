//! Real-binary acceptance for the M43 **catalog shape fence** (the style
//! guide's floor, `design/surface-contract.md` → The catalog shape fence):
//! the eager workflow-front-matter sweep requires every **selectable
//! work-workflow** (`creates-task: true && selectable: true`) of a
//! manifest-shipping pack to carry non-empty `when:`/`description:`/`usage:`,
//! and its `when:` line to be mechanically shaped — one line, period-less,
//! length-capped (the `workflow-dialect.md` craft rules' checkable half).
//!
//! Mutated shipped-tree copies (a dropped `usage:`; a multi-line /
//! period-terminated / over-cap `when:`) exit **non-zero at pack-load naming
//! the workflow**; a non-selectable or `creates-task: false` workflow is
//! outside the fence (`introspection.md` skip-on-absent, honored via scoping);
//! both shipped packs load clean. The when-NOT-clause assert is deliberately
//! absent — prose semantics, declined at Settle #7.
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract (the `suppression_fence.rs` / `freeze_enforcement.rs`
//! pattern).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-catshape-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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

/// Copy a shipped pack tree into a fresh temp dir and return the copy's root.
fn pack_copy(tag: &str, src: &Path) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(src, dir.path());
    dir
}

/// Drop the named workflow's `<field>:` front-matter line (plus any indented
/// continuation, so a future block-scalar shape stays droppable) from a copied
/// pack — the presence mutation the fence must catch.
fn drop_front_matter_field(pack: &Path, workflow: &str, field: &str) {
    let path = pack.join("workflows").join(format!("{workflow}.yaml"));
    let body = fs::read_to_string(&path).expect("read the copied workflow");
    let key = format!("{field}:");
    let mut out = String::new();
    let mut dropping = false;
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with(&key) {
            dropping = true;
            hit = true;
            continue;
        }
        if dropping {
            if line.starts_with(' ') {
                continue;
            }
            dropping = false;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `{workflow}` workflow must carry a `{field}:` line to drop"
    );
    fs::write(&path, out).expect("write the mutated workflow");
}

/// Replace the named workflow's one-line `when:` with the given raw YAML text
/// (which may span lines — the shape mutations).
fn replace_when(pack: &Path, workflow: &str, replacement: &str) {
    let path = pack.join("workflows").join(format!("{workflow}.yaml"));
    let body = fs::read_to_string(&path).expect("read the copied workflow");
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with("when:") {
            hit = true;
            out.push_str(replacement);
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `{workflow}` workflow must carry a `when:` line to replace"
    );
    fs::write(&path, out).expect("write the mutated workflow");
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

/// Run `jigc <args>` with NO `JIGC_PACK_DIR` (the embedded / listed-pack path).
fn run_embedded(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

const START: &[&str] = &["start", "--workflow", "single-task", "catalog-shape probe"];

/// Mutate a dev-pack copy, run the composing `jigc start` against it, and
/// assert pack-load **blocks** with stderr naming the workflow + the field.
fn assert_blocks(tag: &str, mutate: impl FnOnce(&Path), workflow: &str, field: &str, why: &str) {
    let repo = TempDir::new(&format!("{tag}-repo"));
    let home = TempDir::new(&format!("{tag}-home"));
    let pack = pack_copy(tag, &embedded_pack_tree());
    init_repo(repo.path());
    mutate(pack.path());

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "{why} must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains(workflow),
        "stderr must name the offending `{workflow}` workflow; got:\n{stderr}",
    );
    assert!(
        stderr.contains(field),
        "stderr must name the offending `{field}` field; got:\n{stderr}",
    );
}

/// The presence fence: a selectable work-workflow with its `usage:` dropped is
/// blocked at pack-load.
#[test]
fn a_dropped_usage_on_a_selectable_work_workflow_blocks() {
    assert_blocks(
        "usage",
        |pack| drop_front_matter_field(pack, "single-task", "usage"),
        "single-task",
        "usage",
        "a selectable work-workflow with no `usage:`",
    );
}

/// The shape fence, line-count half: a multi-line `when:` (a block scalar) is
/// blocked at pack-load.
#[test]
fn a_multi_line_when_blocks() {
    assert_blocks(
        "multiline",
        |pack| {
            replace_when(
                pack,
                "single-task",
                "when: |\n  implement one scoped change\n  end-to-end, over two lines",
            )
        },
        "single-task",
        "when",
        "a multi-line `when:`",
    );
}

/// The shape fence, punctuation half: a period-terminated `when:` is blocked
/// at pack-load (the line interpolates mid-sentence in the router catalog).
#[test]
fn a_period_terminated_when_blocks() {
    assert_blocks(
        "period",
        |pack| {
            replace_when(
                pack,
                "single-task",
                "when: implement one scoped change end-to-end.",
            )
        },
        "single-task",
        "when",
        "a period-terminated `when:`",
    );
}

/// The shape fence, length half: a `when:` over the named char cap is blocked
/// at pack-load.
#[test]
fn an_over_cap_when_blocks() {
    let long = format!("when: implement {}", "a very long clause ".repeat(10));
    assert!(long.len() > 130, "the mutation must overshoot the cap");
    assert_blocks(
        "overcap",
        |pack| replace_when(pack, "single-task", &long),
        "single-task",
        "when",
        "an over-cap `when:`",
    );
}

/// The sweep covers **every** manifest-shipping constituent: a *listed*
/// methodology-pack copy whose selectable `decided-task` gains a
/// period-terminated `when:` blocks at pack-load too.
#[test]
fn a_mutated_methodology_workflow_blocks_through_the_listed_pack_path() {
    let repo = TempDir::new("m-repo");
    let home = TempDir::new("m-home");
    let pack = pack_copy("m-period", &methodology_pack_tree());
    init_repo(repo.path());
    replace_when(
        pack.path(),
        "decided-task",
        "when: implement one scoped change test-first, recording the design decision it makes.",
    );
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    let out = run_embedded(repo.path(), home.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a listed pack's period-terminated `when:` must block; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("decided-task") && stderr.contains("when"),
        "stderr must name the offending `decided-task` workflow and its `when` field; got:\n{stderr}",
    );
}

/// The omitting contexts: the fence is scoped to selectable work-workflows, so
/// the **same mutations** on out-of-scope workflows load clean — a dropped
/// `usage:` on the hidden `sub-task` (`selectable: false`) and on the
/// `creates-task: false` `router` (`introspection.md` skip-on-absent, honored
/// via the factory scoping). Never an error.
#[test]
fn out_of_scope_workflows_are_outside_the_fence() {
    let repo = TempDir::new("scope-repo");
    let home = TempDir::new("scope-home");
    let pack = pack_copy("scope", &embedded_pack_tree());
    init_repo(repo.path());
    drop_front_matter_field(pack.path(), "sub-task", "usage");
    drop_front_matter_field(pack.path(), "router", "usage");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "a non-selectable / non-work workflow must be outside the catalog shape fence; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The controls: an **unmutated** dev-pack copy and the composed
/// **`[dev ▸ methodology]`** pair (the setup-written compose marker) both load
/// clean — every shipped selectable work-workflow already conforms, so the
/// fence is inert on every production composition.
#[test]
fn both_shipped_packs_pass_the_catalog_shape_asserts() {
    // The unmutated JIGC_PACK_DIR copy.
    let repo = TempDir::new("clean-repo");
    let home = TempDir::new("clean-home");
    let pack = pack_copy("clean", &embedded_pack_tree());
    init_repo(repo.path());
    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must pass the catalog shape asserts; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // The composed [dev ▸ methodology] pair via the setup-written marker.
    let repo = TempDir::new("pair-repo");
    let home = TempDir::new("pair-home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let out = run_embedded(repo.path(), home.path(), START);
    assert!(
        out.status.success(),
        "the composed [dev ▸ methodology] pair must pass the catalog shape asserts; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}
