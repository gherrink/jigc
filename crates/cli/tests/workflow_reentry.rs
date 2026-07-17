//! End-to-end integration test for `jigc workflow <W> --task <id>` — the M8
//! Increment 2 sub-agent re-entry verb (`design/write-commands.md` → Sub-agent
//! re-entry). A fanned sub-agent re-enters its milestone sub-task by the
//! explicitly-named sub-workflow `<W>`; the CLI **asserts `<W>` equals the
//! sub-task's recorded mint workflow** before composing, so a stale launch
//! template that names the wrong workflow fails loudly rather than silently
//! composing the wrong thing.
//!
//! Two faces are covered here. The **read/compose** half (T2,
//! `workflow_reentry_composes_w_with_the_equality_guard`): compose `<W>` for the
//! sub-task with the W-equality guard. The **provision-on-first-entry** half (T3,
//! `first_reentry_provisions_the_write_ready_area` +
//! `reentry_of_a_creates_task_false_workflow_provisions_nothing`): the first
//! re-entry provisions the sub-workflow's deterministic commit doc into
//! `tasks/<sub>/docs/`, idempotently, `creates-task`-gated.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo: a milestone
//! with two sub-tasks (one minted `--workflow single-task`, one defaulting to
//! `sub-task`). The read/compose cases:
//!   (a) `jigc workflow single-task --task <single>` composes the single-task view
//!       (its create-gate `jigc doc create adr` line emitted) and exits 0.
//!   (b) `jigc workflow router --task <single>` (`<W>` != recorded) is rejected with
//!       a blocking finding naming the mismatch and exits non-zero.
//!   (c) `jigc workflow single-task --task <unknown>` rejects `no task`.
//!   (d) `jigc workflow sub-task --task <default>` surfaces the existing
//!       `workflow-refs.unknown-workflow` block (the `sub-task` pack workflow
//!       ships in increment 5 — fail-loud until then), exits non-zero.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-workflow-reentry-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves (milestone mint reads HEAD).
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
fn workflow_reentry_composes_w_with_the_equality_guard() {
    let repo = TempDir::new("guard");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A milestone with two sub-tasks: one minted `--workflow single-task` (so its
    // recorded mint workflow is single-task), one defaulting to `sub-task`.
    let created = run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
    let single = run(
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
    assert!(
        single.status.success(),
        "`add-task --workflow single-task` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&single.stderr),
    );
    let single_sub = "move-cache-to-redis";
    let default = run(
        repo.path(),
        home.path(),
        &["milestone", "add-task", "cache-rework", "Evict stale keys"],
    );
    assert!(
        default.status.success(),
        "`add-task` (default workflow) must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&default.stderr),
    );
    let default_sub = "evict-stale-keys";

    // (a) `jigc workflow single-task --task <single>` composes the single-task view —
    //     `<W>` == the recorded mint workflow, so the guard passes. The
    //     single-task-distinctive line is its create-gate `jigc doc create adr`
    //     affordance (from `allows-create: [{type: adr, as: decision}]`).
    let composed = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", single_sub],
    );
    assert!(
        composed.status.success(),
        "`jigc workflow single-task --task <single>` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&composed.stderr),
    );
    let composed_out = String::from_utf8(composed.stdout).expect("utf-8 stdout");
    assert!(
        composed_out.contains("jigc doc create adr"),
        "the composed single-task view must carry its create-gate ADR affordance; got:\n{composed_out}",
    );

    // (b) `jigc workflow router --task <single>` — `<W>` (router) != the recorded
    //     mint workflow (single-task), so the equality guard rejects it with a
    //     blocking finding naming both the requested and the recorded workflow.
    let mismatch = run(
        repo.path(),
        home.path(),
        &["workflow", "router", "--task", single_sub],
    );
    assert!(
        !mismatch.status.success(),
        "a `<W>` != recorded-workflow mismatch must exit non-zero",
    );
    let mismatch_err = String::from_utf8_lossy(&mismatch.stderr);
    assert!(
        mismatch_err.contains("router")
            && mismatch_err.contains("single-task")
            && mismatch_err.contains(single_sub),
        "the mismatch finding must name the requested `<W>`, the recorded workflow, and \
         the sub-task; got:\n{mismatch_err}",
    );

    // (c) `jigc workflow single-task --task <unknown>` rejects `no task`.
    let unknown = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", "no-such-sub"],
    );
    assert!(
        !unknown.status.success(),
        "an unknown sub-task must exit non-zero",
    );
    let unknown_err = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        unknown_err.contains("no task") && unknown_err.contains("no-such-sub"),
        "the unknown-task rejection must say `no task` and name the id; got:\n{unknown_err}",
    );

    // (d) `jigc workflow sub-task --task <default>` — `<W>` (sub-task) == the recorded
    //     mint workflow, so the equality guard passes; with the `sub-task` pack
    //     workflow now shipped (increment 5, T2) the compose succeeds, emitting the
    //     fan-out-free sub-task view (locate / implement / author-commit) — and,
    //     crucially, **no `jigc task finalize` line**, since the parent milestone's
    //     finalize is the only commit boundary. This asserts only that re-entry
    //     **composes** the `sub-task` view; that the same re-entry also **provisions**
    //     the `commit:<sub>` doc (`sub-task` is `creates-task: true`, so
    //     `should_provision_commit_doc` holds despite `selectable: false`) is the
    //     genuine worktree-active acceptance in `milestone.rs`
    //     (`milestone_finalize_squash_*_genuine_reentry_*`).
    let composed_sub = run(
        repo.path(),
        home.path(),
        &["workflow", "sub-task", "--task", default_sub],
    );
    assert!(
        composed_sub.status.success(),
        "`jigc workflow sub-task --task <default>` must exit 0 now sub-task ships; stderr:\n{}",
        String::from_utf8_lossy(&composed_sub.stderr),
    );
    let composed_sub_out = String::from_utf8(composed_sub.stdout).expect("utf-8 stdout");
    // The prose MAY name `jigc task finalize` — only to ban it (the C0 author-commit
    // wording); what must never appear is a `Run:` affordance for it.
    assert!(
        !composed_sub_out.contains("Run: `jigc task finalize"),
        "the fan-out-free sub-task view must emit no `jigc task finalize` Run line; got:\n{composed_sub_out}",
    );
}

/// The path a sub-task's provisioned commit doc lives at: `.jigc/tasks/<sub>/docs/commit:<sub>.md`.
fn commit_doc_path(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(sub)
        .join("docs")
        .join(format!("commit:{sub}.md"))
}

/// T3 — provision-on-first-entry: the first `jigc workflow <W> --task <id>` provisions
/// the sub-workflow's deterministic commit doc into `tasks/<sub>/docs/`, mirroring
/// `jigc start`'s mint-time provisioning but deferred to first entry, idempotent
/// (first-entry-only), and never run for a `creates-task: false` `<W>`
/// (`design/write-commands.md` → Sub-agent re-entry; roadmap M8 Increment 2 bullet 3).
#[test]
fn first_reentry_provisions_the_write_ready_area() {
    let repo = TempDir::new("provision");
    init_repo(repo.path());
    let home = TempDir::new("home");

    run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    let intent = "Move cache to redis";
    let sub = "move-cache-to-redis";
    let added = run(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            intent,
            "--workflow",
            "single-task",
        ],
    );
    assert!(added.status.success(), "add-task must exit 0");
    // A never-re-entered sibling sub-task, to prove provisioning is keyed to the
    // re-entered sub-area only (criterion e).
    let sibling = "evict-stale-keys";
    let added_sibling = run(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Evict stale keys",
            "--workflow",
            "single-task",
        ],
    );
    assert!(
        added_sibling.status.success(),
        "add-task sibling must exit 0"
    );

    // (a) After `add-task` the sub-area has NO docs/ — provisioning is deferred to
    //     first re-entry, not done at mint.
    let docs_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(sub)
        .join("docs");
    assert!(
        !docs_dir.exists(),
        "`add-task` must NOT provision docs/ — provisioning is deferred to first entry",
    );

    // (b) The first `jigc workflow single-task --task <sub>` provisions
    //     `docs/commit:<sub>.md` as the fillable empty form, byte-identical to what
    //     `jigc start --workflow single-task "<intent>"` mints for the same intent.
    let composed = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", sub],
    );
    assert!(
        composed.status.success(),
        "first re-entry must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&composed.stderr),
    );
    let provisioned = fs::read(commit_doc_path(repo.path(), sub))
        .expect("first re-entry must provision docs/commit:<sub>.md");

    // The reference bytes: a top-level `jigc start --workflow single-task "<intent>"`
    // in a fresh repo mints the same slug id, so its provisioned commit form is the
    // shape re-entry must reproduce (the provisioned bytes are the contract).
    let ref_repo = TempDir::new("ref");
    init_repo(ref_repo.path());
    let ref_home = TempDir::new("ref-home");
    let started = run(
        ref_repo.path(),
        ref_home.path(),
        &["start", "--workflow", "single-task", intent],
    );
    assert!(
        started.status.success(),
        "reference `jigc start` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&started.stderr),
    );
    let reference = fs::read(commit_doc_path(ref_repo.path(), sub))
        .expect("reference `jigc start` must provision docs/commit:<sub>.md");
    assert_eq!(
        provisioned, reference,
        "re-entry-provisioned bytes must equal the shape `jigc start` mints",
    );

    // (e) The never-re-entered sibling sub-area still has NO docs/.
    let sibling_docs = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(sibling)
        .join("docs");
    assert!(
        !sibling_docs.exists(),
        "a never-re-entered sibling sub-area must still have no docs/",
    );

    // (d) Re-entering a second time after a simulated agent edit leaves the edited
    //     bytes intact — provisioning is first-entry-only, keyed on the skeleton's
    //     absence, so a sub-agent's in-progress edits survive re-entry.
    let edited = b"## edited by the sub-agent\n";
    fs::write(commit_doc_path(repo.path(), sub), edited).expect("simulate an agent edit");
    let reentered = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", sub],
    );
    assert!(
        reentered.status.success(),
        "second re-entry must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&reentered.stderr),
    );
    let after = fs::read(commit_doc_path(repo.path(), sub)).expect("the doc must still exist");
    assert_eq!(
        after, edited,
        "a second re-entry must NOT overwrite the edited bytes (first-entry-only)",
    );
}

/// (c) A `creates-task: false` `<W>` provisions nothing on re-entry. The `router`
/// ships `creates-task: false`; a sub-task minted `--workflow router` re-enters via
/// `jigc workflow router --task <sub>` (the W-equality guard passes) and the compose
/// must leave the sub-area docs-less (the no-task arm provisions nothing).
#[test]
fn reentry_of_a_creates_task_false_workflow_provisions_nothing() {
    let repo = TempDir::new("notask");
    init_repo(repo.path());
    let home = TempDir::new("home");

    run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    let sub = "pick-a-workflow";
    let added = run(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Pick a workflow",
            "--workflow",
            "router",
        ],
    );
    assert!(
        added.status.success(),
        "add-task --workflow router must exit 0"
    );

    let composed = run(
        repo.path(),
        home.path(),
        &["workflow", "router", "--task", sub],
    );
    assert!(
        composed.status.success(),
        "re-entering a `creates-task: false` workflow must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&composed.stderr),
    );
    let docs_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(sub)
        .join("docs");
    assert!(
        !docs_dir.exists(),
        "a `creates-task: false` `<W>` must provision nothing on re-entry",
    );
}
