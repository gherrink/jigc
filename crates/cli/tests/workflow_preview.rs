//! End-to-end integration test for `jigc workflow <id> --preview` — the M44
//! Increment 3 read surface (`design/surface-contract.md` → law 2: nothing hides;
//! the read-surface spine of `introspection.md` / `doc-read-surface.md`). It composes
//! a **`creates-task: true`** workflow's step text **without minting a task**, so a
//! mutation-cautious agent can read what a work-minting workflow will ask *before*
//! consenting to mint — the capability half of change 2 (Inc 4).
//!
//! This suite drives the **real binary** (T2 lesson: the capability is proven THROUGH
//! the shipped verb, not the core fn) against the shipped dev pack, and asserts the
//! **emitted bytes**:
//!   (a) `jigc workflow single-task --preview` exits 0; its agent output leads with
//!       the mint-first banner (law 3 — nothing ambushes), names the mint command, and
//!       renders `--task your-task-id` as an identity (law 1 — never a fictional real
//!       id); it carries **no** `task minted:` / `resume:` / `what's-left:` line, and
//!       provisions **no** `.jigc/tasks/*` directory (nothing minted).
//!   (b) `--format json` stays the pinned `{task: null, text}` contract — `task` is
//!       `null`, `text` carries the composed body, and the banner never reaches JSON.
//!   (c) `jigc workflow router --preview` (`creates-task: false`) exits non-zero with
//!       the routed rejection (there is nothing to preview — the router mints nothing).
//!   (d) the pre-existing `jigc workflow <W> --task <id>` re-entry path still composes
//!       (exit 0, unchanged).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-workflow-preview-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// so the cascade resolves.
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn preview_composes_a_creates_task_workflow_without_minting() {
    let repo = TempDir::new("compose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // (a) `jigc workflow single-task --preview` composes `single-task`
    //     (`creates-task: true`) without minting a task.
    let preview = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--preview"],
    );
    assert!(
        preview.status.success(),
        "`jigc workflow single-task --preview` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&preview.stderr),
    );
    let out = String::from_utf8(preview.stdout).expect("utf-8 stdout");

    // The mint-first banner (law 3): it states no task is minted and names the mint
    // command with the workflow id.
    assert!(
        out.contains("no task minted"),
        "the preview must lead with the mint-first banner stating no task minted; got:\n{out}",
    );
    assert!(
        out.contains("jigc start --workflow single-task"),
        "the banner must name the mint command with the workflow id; got:\n{out}",
    );
    // The synthetic identity (law 1): the composed body renders `--task your-task-id`,
    // an identity/placeholder — never a fictional real id.
    assert!(
        out.contains("your-task-id"),
        "the preview must render `your-task-id` as an identity in the composed body; got:\n{out}",
    );
    // The composed body is really present — the single-task create-gate line, its
    // distinctive affordance.
    assert!(
        out.contains("jigc doc create adr"),
        "the preview must carry the composed single-task body; got:\n{out}",
    );

    // No minted-task presentation: nothing was minted, so none of the id-carrying
    // affordance lines may appear.
    assert!(
        !out.contains("task minted:"),
        "a preview mints nothing — no `task minted:` line; got:\n{out}",
    );
    assert!(
        !out.contains("resume:"),
        "a preview has no task to resume — no `resume:` line; got:\n{out}",
    );
    assert!(
        !out.contains("what's-left:"),
        "a preview has no task to validate — no `what's-left:` line; got:\n{out}",
    );

    // No task store touched: a preview provisions no `.jigc/tasks/*` directory.
    let tasks_dir = repo.path().join(".jigc").join("tasks");
    if tasks_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&tasks_dir)
            .expect("read tasks dir")
            .collect::<Result<Vec<_>, _>>()
            .expect("read tasks entries");
        assert!(
            entries.is_empty(),
            "a preview must create no `.jigc/tasks/*` directory; found {} entries",
            entries.len(),
        );
    }
}

#[test]
fn preview_json_stays_the_pinned_task_null_text_contract() {
    let repo = TempDir::new("json");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let preview = run(
        repo.path(),
        home.path(),
        &["--format", "json", "workflow", "single-task", "--preview"],
    );
    assert!(
        preview.status.success(),
        "`--format json workflow single-task --preview` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&preview.stderr),
    );
    let out = String::from_utf8(preview.stdout).expect("utf-8 stdout");
    let value: serde_json::Value = serde_json::from_str(&out).expect("preview JSON parses");

    // The pinned `{task, text}` contract: `task` is null (nothing minted, law 1), and
    // `text` carries the composed body.
    let obj = value.as_object().expect("JSON object");
    assert_eq!(
        obj.keys().collect::<Vec<_>>(),
        vec!["task", "text"],
        "the composed-output JSON contract is exactly {{task, text}}; got keys {:?}",
        obj.keys().collect::<Vec<_>>(),
    );
    assert!(
        value["task"].is_null(),
        "a preview mints no task — `task` must be null; got: {}",
        value["task"],
    );
    let text = value["text"].as_str().expect("`text` is a string");
    assert!(
        text.contains("your-task-id"),
        "the composed body must render the `your-task-id` identity; got:\n{text}",
    );
    // The banner is presentation-only — it never reaches the JSON contract.
    assert!(
        !text.contains("no task minted"),
        "the mint-first banner must never reach the JSON `text`; got:\n{text}",
    );
}

#[test]
fn preview_of_a_creates_task_false_workflow_is_rejected() {
    let repo = TempDir::new("router");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // (c) `jigc workflow router --preview` — `router` is `creates-task: false`, so it
    //     mints nothing to begin with; there is nothing to preview, and the command is
    //     rejected with the route to run it directly.
    let rejected = run(
        repo.path(),
        home.path(),
        &["workflow", "router", "--preview"],
    );
    assert!(
        !rejected.status.success(),
        "previewing a `creates-task: false` workflow must exit non-zero",
    );
    let err = String::from_utf8_lossy(&rejected.stderr);
    assert!(
        err.contains("router")
            && err.contains("nothing to preview")
            && err.contains("jigc start --workflow router"),
        "the rejection must name the workflow, say there is nothing to preview, and route \
         to running it directly; got:\n{err}",
    );
}

#[test]
fn workflow_task_reentry_still_composes() {
    let repo = TempDir::new("reentry");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A milestone sub-task minted `--workflow single-task`, so the re-entry
    // W-equality guard passes.
    let created = run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert!(created.status.success(), "milestone create must exit 0");
    let added = run(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Move cache to redis",
            "--workflow",
            "single-task",
        ],
    );
    assert!(added.status.success(), "add-task must exit 0");
    let sub = "move-cache-to-redis";

    // (d) The pre-existing `jigc workflow <W> --task <id>` re-entry path still composes.
    let composed = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", sub],
    );
    assert!(
        composed.status.success(),
        "`jigc workflow single-task --task <sub>` must still exit 0; stderr:\n{}",
        String::from_utf8_lossy(&composed.stderr),
    );
    let out = String::from_utf8(composed.stdout).expect("utf-8 stdout");
    assert!(
        out.contains("jigc doc create adr"),
        "the re-entry compose must carry the single-task body; got:\n{out}",
    );
    // A real re-entry names its task id — not the preview placeholder.
    assert!(
        !out.contains("your-task-id"),
        "a real re-entry composes the real id, never the preview placeholder; got:\n{out}",
    );
}
