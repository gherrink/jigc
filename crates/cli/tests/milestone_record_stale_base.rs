//! M39 Increment 4 / T7 — the **stale-base edge**: a milestone whose recorded base-SHA pin
//! names a commit that **no longer exists** (a history rewrite orphaned it) must route the
//! human with a blocking finding, never git-operate against the missing commit
//! (`design/team-ready-state.md` → Stale-base edge (history rewrite)). Under a
//! `[dev ▸ methodology]` project whose composed cascade resolves the `milestone-record`
//! schema.
//!
//! Two proofs, driving the REAL binary against throwaway git repos. Both set the committed
//! record's `base` to a **non-existent** SHA (the done-criterion's first option), then
//! simulate a fresh clone (`rm -rf .jigc`, restore the tracked bits) so the demoted cache is
//! re-derived from the drifted record — the file-state baseline is gone too, so the T6
//! reconcile preflight adopts (no drift block) and the *stale-base* guard is what fires:
//!
//!   (a) **provision** — reads the base to detach one worktree per sub-task → blocks with a
//!       routed finding **before** any `git worktree add`, so no worktree is provisioned
//!       (no partial write).
//!   (b) **finalize** — reads the base for the worktree-combine boundary → blocks the same
//!       way **before** any commit, so no commit lands and the record is not flipped.
//!
//! Pre-fix, the op reaches `git` with the bogus SHA and fails with a raw git error (or, for
//! provision, a partially-provisioned worktree set) — never a routed, human-facing block.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-stale-base-{tag}-{}-{:?}",
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
/// the composition dev-highest (so the dev `docs-root` knob applies → `docs/`).
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

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("cache-rework.md")
}

/// The all-zeros SHA — a well-formed object name that names **no** commit in the repo, the
/// rewritten-away base pin the guard must route on.
const MISSING_SHA: &str = "0000000000000000000000000000000000000000";

/// Rewrite the committed record's `meta` header `base:` line to the non-existent
/// [`MISSING_SHA`] (leaving `docs/` uncommitted in the working tree — the fresh-clone
/// re-derive reads it). Returns the drifted bytes for the not-corrupted assertion.
fn set_record_base_missing(repo: &Path) -> String {
    let path = record_path(repo);
    let before = fs::read_to_string(&path).expect("read record before base rewrite");
    let mut out = String::with_capacity(before.len());
    let mut rewrote = false;
    for line in before.lines() {
        if line.trim_start().starts_with("base:") {
            out.push_str(&format!("base: {MISSING_SHA} {}", &MISSING_SHA[..7]));
            rewrote = true;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(
        rewrote,
        "the record must carry a `base:` line; got:\n{before}"
    );
    fs::write(&path, &out).expect("write base-rewritten record");
    out
}

/// create → add-task → rewrite the record base to a non-existent SHA → simulate a fresh
/// clone (drop ALL of `.jigc/`, restore only the tracked bits). The demoted cache is then
/// re-derived from the drifted record on the next op; the file-state baseline is gone, so
/// the T6 reconcile preflight adopts (no drift block) — the stale-base guard is what fires.
/// Returns the drifted record bytes for the not-corrupted assertion.
fn seed_stale_base(repo: &Path, home: &Path) -> String {
    init_repo(repo);
    write_compose_marker(repo);
    assert!(
        run_milestone(repo, home, &["create", "Cache rework"])
            .status
            .success(),
        "`[dev ▸ methodology]` `jigc milestone create` must exit 0",
    );
    // Commit the tracked `.jigc` bits (compose marker + generated `.gitignore`) the way a
    // clone carries them — the WIP under `.jigc/` does not survive the clone.
    git(
        repo,
        &["add", ".jigc/config/packs.yaml", ".jigc/.gitignore"],
    );
    git(repo, &["commit", "-q", "-m", "jigc config"]);
    assert!(
        run_milestone(
            repo,
            home,
            &["add-task", "cache-rework", "Warm the read cache"]
        )
        .status
        .success(),
        "add-task must exit 0",
    );

    let drifted = set_record_base_missing(repo);

    // Fresh-clone simulation: drop `.jigc/`, restore only the tracked bits. The gitignored
    // WIP (`.jigc/milestones/…`, `.jigc/state/…`) is gone — the next op re-derives from the
    // committed (now base-drifted) record.
    fs::remove_dir_all(repo.join(".jigc")).expect("rm -rf .jigc");
    git(repo, &["checkout", "--", ".jigc"]);
    drifted
}

/// A routed stale-base block names the record's identity and its human route (re-pin /
/// restore the base) — the human-facing surface, not a raw git error.
fn assert_routed_stale_base(stderr: &str) {
    assert!(
        stderr.contains("cache-rework")
            && stderr.contains("no longer exists")
            && stderr.to_lowercase().contains("route:"),
        "the block must be a routed stale-base finding (naming the milestone + a re-pin \
         route), not a raw git error; got stderr:\n{stderr}",
    );
}

#[test]
fn stale_base_routes_provision_without_provisioning() {
    let repo = TempDir::new("provision");
    let home = TempDir::new("home");
    let drifted = seed_stale_base(repo.path(), home.path());

    let out = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        !out.status.success(),
        "provision over a stale base must block (non-zero); got success\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert_routed_stale_base(&String::from_utf8_lossy(&out.stderr));

    // No partial write: not one worktree was provisioned (only the main checkout registered).
    let worktrees = git(repo.path(), &["worktree", "list", "--porcelain"]);
    assert_eq!(
        worktrees
            .lines()
            .filter(|l| l.starts_with("worktree "))
            .count(),
        1,
        "a blocked provision must register NO fan-out worktree; got:\n{worktrees}",
    );
    // The committed record is untouched.
    assert_eq!(
        fs::read_to_string(record_path(repo.path())).expect("record readable"),
        drifted,
        "a blocked provision must not rewrite the record",
    );
}

#[test]
fn stale_base_routes_finalize_without_committing() {
    let repo = TempDir::new("finalize");
    let home = TempDir::new("home");
    let drifted = seed_stale_base(repo.path(), home.path());
    let commits_before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        !out.status.success(),
        "finalize over a stale base must block (non-zero); got success\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert_routed_stale_base(&String::from_utf8_lossy(&out.stderr));

    // No corruption: no commit landed and the record was not flipped/overwritten.
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        commits_before,
        "a blocked finalize makes no commit",
    );
    assert_eq!(
        fs::read_to_string(record_path(repo.path())).expect("record readable"),
        drifted,
        "a blocked finalize must leave the record byte-identical (not flipped)",
    );
}
