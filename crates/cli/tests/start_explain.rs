//! End-to-end integration test for `jigc start --explain "<intent>"` (M4 Inc-6,
//! T3): the resolution-tree view, driven through the built `jigc` binary against a
//! throwaway git repo and asserted on the **emitted bytes**.
//!
//! `--explain` is task-independent: it resolves the cascade, builds the resolution
//! tree over the resolved default (or `--workflow`-named) workflow, and prints it
//! WITHOUT minting — no `.jigc/tasks/<id>/` working area appears
//! (`design/workflow-dialect.md` → `--explain` output contract;
//! `design/worked-examples.md` → 3a). Three behaviours pinned: the unmodified-pack
//! tree (`overrides applied: none` + the include tree with each step's source
//! layer), the post-`replace-step` tree (`overrides applied: 1` + the `← replaces …
//! at position` annotation), and the structured `--format json` projection.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-start-explain-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
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

/// Run `jigc start <args>` with `cwd = repo` and `$HOME = home`.
fn run_start(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("start");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn explain_over_the_unmodified_pack_prints_the_tree_and_mints_nothing() {
    // (a) `jigc start --explain --workflow single-task "<intent>"` over the
    // unmodified pack prints the resolution tree — the workflow line, `overrides
    // applied: none`, and the include tree with each step tagged by source layer —
    // and MINTS NOTHING (no `.jigc/tasks/<id>/` appears).
    let repo = TempDir::new("unmodified");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(
        repo.path(),
        home.path(),
        &["--explain", "--workflow", "single-task", "add rate limiter"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "`jigc start --explain` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // INERT-FLAG GUARD: `--explain` changes observable output. The tree names the
    // workflow + its winning layer.
    assert!(
        stdout.contains("workflow:single-task"),
        "the tree must name the workflow; got:\n{stdout}",
    );
    assert!(
        stdout.contains("pack-default"),
        "the workflow line must show its winning cascade layer (pack-default); got:\n{stdout}",
    );
    // No structural override applied → `overrides applied: none`.
    assert!(
        stdout.contains("overrides applied: none"),
        "the unmodified pack must report `overrides applied: none`; got:\n{stdout}",
    );
    // The pack-default include list, each step tagged by source layer.
    for step in [
        "step:locate",
        "step:implement",
        "step:superseded-context",
        "step:finalize",
    ] {
        assert!(
            stdout.contains(step),
            "the include tree must list `{step}`; got:\n{stdout}",
        );
    }

    // --explain is task-independent: it mints NOTHING.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "`--explain` must not mint a task — no .jigc/tasks/ dir may appear; got:\n{stdout}",
    );
}

#[test]
fn explain_after_a_replace_step_reports_one_override_with_the_replaces_annotation() {
    // (b) After a `replace-step workflow:single-task#implement → step:project-
    // implement`, `--explain` prints `overrides applied: 1` with the `← replaces …
    // at position` annotation (`worked-examples.md` → 3a).
    let repo = TempDir::new("replace-step");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    fs::write(
        config.join("manifest.yaml"),
        "deltas:\n\
         \x20 - kind: replace-step\n\
         \x20   target: workflow:single-task#implement\n\
         \x20   with: step:project-implement\n",
    )
    .expect("write project manifest with a replace-step delta");
    fs::create_dir_all(config.join("steps")).expect("mk steps/");
    fs::write(
        config.join("steps").join("project-implement.yaml"),
        "{{ include: step:implement }}\n\n\
         Before you finalize, run the project lint probe and fix any findings.\n",
    )
    .expect("write the native project-implement step");

    let out = run_start(
        repo.path(),
        home.path(),
        &["--explain", "--workflow", "single-task", "add rate limiter"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "`jigc start --explain` after a replace-step must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // One structural override applied.
    assert!(
        stdout.contains("overrides applied: 1"),
        "a single replace-step must report `overrides applied: 1`; got:\n{stdout}",
    );
    // The project step won its slot, tagged `project`.
    assert!(
        stdout.contains("step:project-implement"),
        "the resolved include list must carry the project step; got:\n{stdout}",
    );
    assert!(
        stdout.contains("project"),
        "the project step's source layer must show as `project`; got:\n{stdout}",
    );
    // The `← replaces <id> at position N` annotation (position 2 in the list).
    assert!(
        stdout.contains("replaces step:implement at position 2"),
        "the replace-step slot must carry the `← replaces … at position` annotation; got:\n{stdout}",
    );

    // Still mints nothing.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "`--explain` must not mint a task; got:\n{stdout}",
    );
}

#[test]
fn explain_format_json_emits_the_structured_tree() {
    // (c) `--explain --format json` emits the structured tree — parseable, same
    // provenance (the winning layer per step + the override count + the replacement
    // annotation).
    let repo = TempDir::new("json");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    fs::write(
        config.join("manifest.yaml"),
        "deltas:\n\
         \x20 - kind: replace-step\n\
         \x20   target: workflow:single-task#implement\n\
         \x20   with: step:project-implement\n",
    )
    .expect("write project manifest with a replace-step delta");
    fs::create_dir_all(config.join("steps")).expect("mk steps/");
    fs::write(
        config.join("steps").join("project-implement.yaml"),
        "{{ include: step:implement }}\n",
    )
    .expect("write the native project-implement step");

    let out = run_start(
        repo.path(),
        home.path(),
        &[
            "--explain",
            "--format",
            "json",
            "--workflow",
            "single-task",
            "add rate limiter",
        ],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "`jigc start --explain --format json` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("--format json must emit parseable JSON");

    assert_eq!(value["workflow"], "single-task", "got:\n{stdout}");
    assert_eq!(value["workflow_layer"], "pack-default", "got:\n{stdout}");
    assert_eq!(value["overrides_applied"], 1, "got:\n{stdout}");

    let steps = value["steps"].as_array().expect("steps is an array");
    // The slot the replace swapped: id = the project step, layer = project, with a
    // `replaces` annotation naming the swapped id + position.
    let swapped = steps
        .iter()
        .find(|s| s["id"] == "project-implement")
        .expect("the project step is present in the JSON tree");
    assert_eq!(swapped["layer"], "project", "got:\n{stdout}");
    assert_eq!(
        swapped["replaces"]["replaced"], "implement",
        "got:\n{stdout}"
    );
    assert_eq!(swapped["replaces"]["position"], 2, "got:\n{stdout}");

    // JSON carries no routing footer (tooling-consumed).
    assert!(
        !stdout.contains("— jigc ·"),
        "JSON --explain output must carry no routing footer; got:\n{stdout}",
    );
}
