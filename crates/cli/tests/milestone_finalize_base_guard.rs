//! M39 Increment 4 / T4-prelude — the milestone-finalize base-guard refinement
//! (`design/team-ready-state.md` → The commit model: the finalize base-guard refinement;
//! `DECISIONS.md` 2026-07-07 → the Inc-4/T4 fork). Per-op record commits (`create` /
//! `add-task` each commit `docs/milestone-records/<id>.md`) advance HEAD past the milestone's
//! pinned base, so the old `plan_milestone_finalize` preflight `base == HEAD` (the M31
//! worktree-combine external-drift guard) can never pass in this model. The guard is refined,
//! not removed: it advances the base over a linear-ancestor range whose every commit touches
//! ONLY the record path — but **any non-record (external) commit in the range still blocks**.
//!
//! Two proofs, driving the REAL binary against throwaway `[dev ▸ methodology]` git repos:
//!
//!   (RED)   **A FOREIGN commit still blocks.** create → add-task ×2 → land a commit touching
//!           a NON-record code path → `finalize` blocks with the base-mismatch route and exits
//!           `EXIT_VALIDATION_BLOCKED` (3). External drift must stay caught (the guarantee that
//!           must not regress) — this passes-by-blocking both before and after the fix.
//!
//!   (GREEN) **A record-only range proceeds.** create → add-task ×2 (only the milestone's own
//!           record commits in `base..HEAD`) → `finalize` no longer emits base-mismatch — the
//!           base advances and the preflight proceeds (it then blocks downstream on the
//!           empty-commit guard, since no sub-task authored a doc — that is past the base-guard,
//!           which is what this task refines). Before the fix this blocks with base-mismatch.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-finalize-base-guard-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Write the `[dev ▸ methodology]` compose marker — the key `make_pack` reads to assemble
/// the composition dev-highest (so the `milestone-record` schema + dev `docs-root` apply).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`, never inheriting a
/// harness `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// create → add-task ×2 on a `[dev ▸ methodology]` repo — the shared setup that lands the
/// milestone's own record-only commits (create opens, each add-task appends) in `base..HEAD`.
fn setup_milestone(repo: &Path, home: &Path) {
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`[dev ▸ methodology]` `jigc milestone create`",
    );
    assert_ok(
        &run_milestone(
            repo,
            home,
            &["add-task", "cache-rework", "Warm the read cache"],
        ),
        "add-task #1",
    );
    assert_ok(
        &run_milestone(
            repo,
            home,
            &["add-task", "cache-rework", "Evict cold entries"],
        ),
        "add-task #2",
    );
}

/// (RED — the guarantee that must not regress) A FOREIGN commit touching a non-record code
/// path in `base..HEAD` keeps the base-mismatch block: `finalize` routes the divergence and
/// exits `EXIT_VALIDATION_BLOCKED` (3). External drift stays caught.
#[test]
fn foreign_commit_in_range_still_blocks_finalize_with_base_mismatch() {
    let repo = TempDir::new("foreign");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_milestone(repo.path(), home.path());

    // A foreign, non-record commit — external drift that would invalidate the worktree-combine.
    fs::write(repo.path().join("src.rs"), "fn main() {}\n").expect("write code file");
    git(repo.path(), &["add", "src.rs"]);
    git(repo.path(), &["commit", "-q", "-m", "foreign code change"]);

    let out = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a foreign commit in range blocks finalize at the base-guard (exit 3); stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("HEAD is now"),
        "the block routes the base-mismatch divergence (external drift caught); stderr:\n{stderr}",
    );
}

/// (GREEN — the refinement) With ONLY the milestone's own record commits in `base..HEAD`,
/// `finalize` no longer emits base-mismatch — the base advances over the record-only range and
/// the preflight proceeds. Before the fix this blocked with base-mismatch, so the absence of
/// "HEAD is now" is the red→green witness. Since M39 T4 folds the record's `join` status-flip
/// into the commit, that flip is a real diff, so the finalize now **lands** (exit 0) rather than
/// blocking on the empty-commit guard — the stronger witness that it advanced past the base-guard.
#[test]
fn record_only_range_advances_base_past_the_guard() {
    let repo = TempDir::new("record-only");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_milestone(repo.path(), home.path());

    let out = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("HEAD is now"),
        "a record-only `base..HEAD` range must NOT block at the base-guard — the base advances; \
         stderr:\n{stderr}",
    );
    assert!(
        out.status.success(),
        "the guard advanced and the finalize landed the record's join-flip (T4); exit {:?}, \
         stderr:\n{stderr}",
        out.status.code(),
    );
}
