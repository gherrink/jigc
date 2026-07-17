//! M40 completion-audit fix — the **milestone-record** guard at the ITEM verb, the
//! A4.4 doc-level reslug refusal's item-level mirror (`design/write-commands.md` →
//! `jigc doc retitle-item` / Milestone-record reslug). A milestone-record's `tasks`
//! repeatable is `id-from: task-id`, a plain STRING — so the enum refusal is inert and
//! an unguarded `retitle-item` splices the heading, diverging the committed record
//! (and the `doc show` `task-id` leaf, keyed to item.title) from the real work-unit
//! id while committing clean.
//!
//! Two proofs, driving the REAL binary against a throwaway `[dev ▸ methodology]` repo:
//!
//!   (1) `jigc doc retitle-item milestone-record:<id>#tasks/<task-id>` exits
//!       **non-zero** with a blocking finding whose route points at the record's
//!       machine-maintained nature (the milestone verbs), and moves **no bytes**
//!       (no staged copy is even created).
//!
//!   (2) `jigc doc set-field …#tasks/<task-id>/task-id` (the id-from guard, already
//!       blocking) routes at the machine-maintained nature too — it must NOT name
//!       `retitle-item`, exactly the divergence-producing command (1) refuses.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-retitle-milestone-record-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Initialize a real git repo with one commit + the `[dev ▸ methodology]` compose
/// marker (the exact key `make_pack` reads — the composed cascade resolves the
/// methodology-pack `milestone-record` schema).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation exited 0, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("cache-rework.md")
}

/// Mint the committed record with one sub-task (`warm-the-read-cache`) and an active
/// jigc task whose working area the doc verbs stage into.
fn setup(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(repo, home, &["milestone", "create", "Cache rework"]),
        "jigc milestone create",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "milestone",
                "add-task",
                "cache-rework",
                "Warm the read cache",
            ],
        ),
        "jigc milestone add-task",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "single-task", "poke the record"],
        ),
        "jigc start --workflow single-task",
    );
}

/// (1) `retitle-item` on a milestone-record `tasks` item refuses: non-zero, blocking,
/// the route points at the machine-maintained milestone verbs, and no bytes move —
/// the committed record is byte-identical and no staged copy was created (the guard
/// fires before copy-in). Unguarded, this exits 0 and splices the heading, severing
/// the heading (the work-unit id) from the real sub-task id.
#[test]
fn retitle_item_on_a_milestone_record_refuses_with_the_machine_maintained_route() {
    let repo = TempDir::new("retitle");
    let home = TempDir::new("retitle-home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let before = fs::read_to_string(record_path(repo.path())).expect("committed record");

    let refused = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "retitle-item",
            "milestone-record:cache-rework#tasks/warm-the-read-cache",
            "--title",
            "Warm caches",
            "--task",
            "poke-the-record",
        ],
    );
    assert!(
        !refused.status.success(),
        "retitle-item on a milestone-record must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&refused.stdout),
    );
    let stderr = String::from_utf8_lossy(&refused.stderr).to_string();
    assert!(
        stderr.contains("blocking"),
        "the refusal is a blocking finding; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("machine-maintained"),
        "the finding names the record's machine-maintained nature; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc milestone"),
        "the route points at the milestone verbs; stderr:\n{stderr}",
    );
    // C0 (round-2 surface fixes): the route must NOT name `jigc task finalize` — the
    // record's sub-task statuses advance at `jigc milestone finalize`, and a per-sub-task
    // `task finalize` run in a fan-out worktree strands the sub-task's work.
    assert!(
        !stderr.contains("jigc task finalize"),
        "the route must not claim `jigc task finalize` advances a sub-task's status; \
         stderr:\n{stderr}",
    );

    // No bytes moved: the committed record is untouched and the guard fired before
    // any staged copy-in.
    assert_eq!(
        fs::read_to_string(record_path(repo.path())).expect("committed record after"),
        before,
        "the refused retitle left the committed record byte-identical",
    );
    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("poke-the-record")
        .join("docs")
        .join("milestone-record:cache-rework.md");
    assert!(
        !staged.exists(),
        "the refusal fires before copy-in — no staged milestone-record copy",
    );
}

/// (2) the set-field id-from guard's route, one step more type-aware: on a
/// milestone-record `task-id` it must NOT name `retitle-item` (the divergence-
/// producing command (1) refuses) — it points at the machine-maintained milestone
/// verbs instead.
#[test]
fn set_field_task_id_route_does_not_name_retitle_item_on_a_milestone_record() {
    let repo = TempDir::new("set-field");
    let home = TempDir::new("set-field-home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let refused = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "milestone-record:cache-rework#tasks/warm-the-read-cache/task-id",
            "--value",
            "evict-cold-entries",
            "--task",
            "poke-the-record",
        ],
    );
    assert!(
        !refused.status.success(),
        "set-field on the record's task-id must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&refused.stdout),
    );
    let stderr = String::from_utf8_lossy(&refused.stderr).to_string();
    assert!(
        !stderr.contains("retitle-item"),
        "the milestone-record route must NOT name retitle-item; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("machine-maintained"),
        "the route points at the machine-maintained nature; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc milestone"),
        "the route points at the milestone verbs; stderr:\n{stderr}",
    );
    // C0 (round-2 surface fixes): same truth here — no `jigc task finalize` in the route.
    assert!(
        !stderr.contains("jigc task finalize"),
        "the route must not claim `jigc task finalize` advances a sub-task's status; \
         stderr:\n{stderr}",
    );
}
