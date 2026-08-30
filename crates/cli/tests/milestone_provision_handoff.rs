//! M49 Increment 10 / T5 — **the provision → execute handoff reports the real state**
//! (`design/team-ready-state.md` → The lifecycle; `design/surface-contract.md` → law 1 +
//! the route floor; `design/command-output-contract.md` → Stream discipline).
//!
//! Two halves of **one** state — the set of worktrees that exist under
//! `.jigc/worktrees/` — so they share a finding family and a route (the idempotent
//! `jigc milestone provision <id>` re-run, which reuses what landed and adds the rest):
//!
//! 1. **`jigc milestone execute` was byte-identical over every provisioning state.** Fully
//!    provisioned, half provisioned and never provisioned all composed the same bytes at
//!    exit 0, so a walk that emits `Spawn: cd .jigc/worktrees/<id> && …` for a directory
//!    that does not exist read exactly like one whose worktrees are all there. The response
//!    is deliberately **not** a refusal: the composed walk's own first step is
//!    `Run: jigc milestone provision …`, so `execute` is the orientation read taken
//!    *before* provisioning, and blocking it would refuse the very read that teaches the
//!    repair. The guard's trigger is therefore the **PARTIAL** set alone — some sub-task
//!    has a live worktree and some has none, which is knowable only after a provision ran
//!    and did not finish — and the two settled states stay a silent read, byte-identical
//!    to each other. That equality is asserted, not merely left standing: it is the
//!    decision, and a future advisory that fires on a fully provisioned milestone must
//!    come back through this test.
//!
//! 2. **A provision that failed mid-phase-2 exited with a bare `anyhow`.** `provision`
//!    commits nothing, so it has no [`rejection_frame`] and its `Err` took the dispatch's
//!    `None` arm: no code, no route, no log identity — while the failure had already
//!    moved bytes (the earlier paths are provisioned), which is exactly the PARTIAL state
//!    half 1 now reports. The failure now carries `milestone.provision-failed` with the
//!    path it stopped at and the re-run route, through the same `finding_to_err` funnel
//!    `milestone.leftover-holds-work` already used one screen up.
//!
//! Every arm drives the **real binary** and reads the emitted bytes.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The advisory a partially provisioned `execute` carries.
const PARTIAL_CODE: &str = "milestone.worktrees-partial";
/// The blocking identity of a provision that moved bytes and then stopped.
const FAILED_CODE: &str = "milestone.provision-failed";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-provision-handoff-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success.
fn git_ok(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A real git repo with one commit (the milestone mint reads HEAD) and the project
/// cascade layer `execute` resolves against.
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    git_ok(root, &["config", "commit.gpgsign", "false"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("mk config layer");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Mint `milestone:cache-rework` with two sub-tasks — id-sorted `[alpha-fix, zebra-fix]`,
/// so the second path the provisioning loop reaches is `zebra-fix`.
fn mint_milestone(repo: &Path, home: &Path) {
    let created = run(repo, home, &["milestone", "create", "Cache rework"]);
    assert!(created.status.success(), "create must exit 0");
    for intent in ["Zebra fix", "Alpha fix"] {
        let added = run(
            repo,
            home,
            &["milestone", "add-task", "cache-rework", intent],
        );
        assert!(
            added.status.success(),
            "add-task `{intent}` must exit 0; stderr:\n{}",
            stderr_of(&added),
        );
    }
}

fn worktree_dir(repo: &Path, sub_id: &str) -> PathBuf {
    repo.join(".jigc").join("worktrees").join(sub_id)
}

/// **Half 1 — the byte-equality is broken deliberately, and only where the state is
/// genuinely different.** The same milestone is executed in three provisioning states;
/// the two settled ones stay byte-identical (the deliberate decision) and the PARTIAL
/// one is the one that says so, at exit 0, with the repair route.
#[test]
fn execute_speaks_only_in_the_partial_state() {
    let repo = TempDir::new("three-states");
    init_repo(repo.path());
    let home = TempDir::new("home");
    mint_milestone(repo.path(), home.path());

    // (a) never provisioned — the orientation read taken BEFORE provisioning.
    let none = run(
        repo.path(),
        home.path(),
        &["milestone", "execute", "cache-rework"],
    );
    assert!(
        none.status.success(),
        "execute over an unprovisioned milestone must stay a read at exit 0; stderr:\n{}",
        stderr_of(&none),
    );
    let none_out = stdout_of(&none);

    // (b) fully provisioned.
    let provisioned = run(
        repo.path(),
        home.path(),
        &["milestone", "provision", "cache-rework"],
    );
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        stderr_of(&provisioned),
    );
    let all = run(
        repo.path(),
        home.path(),
        &["milestone", "execute", "cache-rework"],
    );
    assert!(
        all.status.success(),
        "execute must exit 0; stderr:\n{}",
        stderr_of(&all)
    );
    let all_out = stdout_of(&all);

    // (c) exactly one present — one worktree taken back out from under the milestone.
    git_ok(
        repo.path(),
        &["worktree", "remove", "--force", ".jigc/worktrees/zebra-fix"],
    );
    assert!(
        !worktree_dir(repo.path(), "zebra-fix").exists(),
        "the fixture must leave exactly one worktree standing",
    );
    assert!(
        worktree_dir(repo.path(), "alpha-fix").exists(),
        "the fixture must leave `alpha-fix` provisioned",
    );
    let partial = run(
        repo.path(),
        home.path(),
        &["milestone", "execute", "cache-rework"],
    );
    assert!(
        partial.status.success(),
        "execute stays a read even over a half-provisioned milestone (its own step 1 is the \
         repair); stderr:\n{}",
        stderr_of(&partial),
    );
    let partial_out = stdout_of(&partial);

    // The decision, asserted rather than left standing: both SETTLED states are silent,
    // and silent in the same bytes.
    assert_eq!(
        none_out, all_out,
        "a fully provisioned and a never-provisioned `execute` are both settled states and \
         must stay byte-identical",
    );
    // The PARTIAL state is the one that says so.
    assert_ne!(
        partial_out, none_out,
        "a half-provisioned `execute` must not read identically to a settled one",
    );
    assert!(
        partial_out.contains(&format!("advisory · {PARTIAL_CODE}")),
        "the partial state must carry its advisory; got:\n{partial_out}",
    );
    assert!(
        partial_out.contains("zebra-fix"),
        "the advisory must name the sub-task with no worktree; got:\n{partial_out}",
    );
    assert!(
        partial_out.contains("route: `jigc milestone provision cache-rework`"),
        "the advisory must carry the idempotent re-run route; got:\n{partial_out}",
    );
    // Still the read it was: the composed walk is intact underneath the advisory.
    for id in ["alpha-fix", "zebra-fix"] {
        assert!(
            partial_out.contains(&format!("Spawn: `cd .jigc/worktrees/{id} &&")),
            "the composed walk must still emit every sub-task's `Spawn:` line; got:\n{partial_out}",
        );
    }
}

/// **Stream discipline** — under `--format json` the pinned `{task, text}` document keeps
/// stdout and the advisory rides stderr, exactly as the finalize advisories and the
/// migrate byte-floor advisory do (`design/command-output-contract.md` → Stream
/// discipline). Asserted on the partial state, where the advisory exists to be misplaced.
#[test]
fn the_partial_advisory_leaves_the_json_document_alone() {
    let repo = TempDir::new("stream");
    init_repo(repo.path());
    let home = TempDir::new("home");
    mint_milestone(repo.path(), home.path());
    assert!(
        run(
            repo.path(),
            home.path(),
            &["milestone", "provision", "cache-rework"]
        )
        .status
        .success(),
        "provision must exit 0",
    );
    git_ok(
        repo.path(),
        &["worktree", "remove", "--force", ".jigc/worktrees/zebra-fix"],
    );

    let out = run(
        repo.path(),
        home.path(),
        &["--format", "json", "milestone", "execute", "cache-rework"],
    );
    assert!(
        out.status.success(),
        "execute must exit 0; stderr:\n{}",
        stderr_of(&out)
    );
    let stdout = stdout_of(&out);
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!("stdout must parse as exactly one JSON document ({err}); got:\n{stdout}")
    });
    assert!(
        doc.get("text").is_some(),
        "the composed contract keeps stdout; got:\n{stdout}",
    );
    assert!(
        !stdout.contains(PARTIAL_CODE),
        "the advisory must not enter the pinned document; got:\n{stdout}",
    );
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains(&format!("advisory · {PARTIAL_CODE}")),
        "the advisory rides stderr under `--format json`; got:\n{stderr}",
    );
}

/// **Half 2 — a provision that fails mid-phase-2 blocks with a code and a route, and the
/// state it leaves is exactly what half 1 reports.** A regular *file* planted at the
/// second sub-task's worktree path makes phase 2's `remove_dir_all` fail after the first
/// path has already been provisioned: the failure is real, it is mid-walk, and it leaves
/// the PARTIAL set behind.
#[test]
fn a_provision_that_fails_mid_walk_blocks_with_a_code_and_a_route() {
    let repo = TempDir::new("mid-walk");
    init_repo(repo.path());
    let home = TempDir::new("home");
    mint_milestone(repo.path(), home.path());

    // A file, not a directory, at the LAST path the walk reaches: `remove_dir_all` fails
    // on it (ENOTDIR) after `alpha-fix` has already been added. `--force` skips phase 1's
    // probe, so the walk genuinely reaches the mutating phase.
    fs::create_dir_all(repo.path().join(".jigc").join("worktrees")).expect("mk worktrees root");
    fs::write(worktree_dir(repo.path(), "zebra-fix"), b"not a directory\n").expect("plant file");

    let out = run(
        repo.path(),
        home.path(),
        &["milestone", "provision", "cache-rework", "--force"],
    );
    assert!(
        !out.status.success(),
        "a provision that could not provision must not exit 0; stdout:\n{}",
        stdout_of(&out),
    );
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains(&format!("blocking · {FAILED_CODE}")),
        "the failure must carry its own code, not a bare anyhow; got:\n{stderr}",
    );
    assert!(
        stderr.contains("at: ") && stderr.contains("zebra-fix"),
        "the block must name the path it stopped at; got:\n{stderr}",
    );
    assert!(
        stderr.contains("route: `jigc milestone provision cache-rework --force`"),
        "the block must route to the idempotent re-run, carrying the flag the re-run needs; \
         got:\n{stderr}",
    );

    // The state the failure left is the PARTIAL one — and `execute` says so.
    assert!(
        worktree_dir(repo.path(), "alpha-fix").exists(),
        "the earlier path really was provisioned before the walk stopped",
    );
    let executed = run(
        repo.path(),
        home.path(),
        &["milestone", "execute", "cache-rework"],
    );
    assert!(
        executed.status.success(),
        "execute stays a read; stderr:\n{}",
        stderr_of(&executed),
    );
    let composed = stdout_of(&executed);
    assert!(
        composed.contains(&format!("advisory · {PARTIAL_CODE}")) && composed.contains("zebra-fix"),
        "the handoff reports the state the failed provision left; got:\n{composed}",
    );
}
