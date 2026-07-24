//! M45 Increment 7 / T1 — **file-state history-gating** (`DECISIONS.md` → 2026-07-23
//! M45 Settle, Decision 7; `design/storage.md` → Derived caches). The file↔state hashes
//! carry no stamp and no rebuild path, so a checkout that moves underneath the gitignored
//! cache — `git reset --hard` / branch switch / rebase past a doc's creating commit —
//! leaves a recorded baseline pointing at a path that no longer exists in the working
//! tree, which the drift probe reads today as a **blocking** dangling baseline that wedges
//! every subsequent task.
//!
//! The fix history-gates the weak-signal severity: a dangling baseline downgrades to an
//! **advisory** with a route to the existing `jigc unmanage` **only when `git log HEAD -1
//! -- <path>` is empty** (HEAD has no history for the path — nothing was deleted).
//! Every genuine-deletion case (the path *has* history and is now gone) keeps blocking,
//! and the strong-signal (content-preserving `git mv`) arm is untouched.
//!
//! Three proofs, driving the REAL binary against throwaway git repos, each red before the
//! change:
//!
//!   (a) **reset --hard past the creating commit → advisory, no block.** An ADR is
//!       committed and baselined, then HEAD is moved back past its creating commit
//!       (`git reset --hard <root>`, so `git log HEAD -1 -- <path>` is empty). The next
//!       task's `jigc task validate` reports the dangling baseline **Advisory** with a
//!       `jigc unmanage` prune route and **exits 0** (red today: Blocking, exit 3).
//!
//!   (b) **`git rm` + commit (history present) → still blocks.** The same ADR is deleted
//!       via `git rm` and committed, so HEAD carries history for the path. The weak
//!       finding **still blocks** (`reconciliation.rename`, exit 3) — a genuine deletion
//!       is detected and routed, never silently downgraded.
//!
//!   (c) **bare content-preserving `git mv` → the strong finding is unchanged.** The ADR
//!       is `git mv`'d to a new path with identical content; the strong-signal
//!       `reconciliation.rename` still surfaces (names both paths, routes to `jigc
//!       rename`), untouched by the history gate.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The committed ADR at its canonical path / record key.
const ADR_PATH: &str = "docs/decisions/single-node-cache.md";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-fshistory-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success and returning trimmed stdout.
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
}

/// Task 0 — create + finalize `adr:single-node-cache`, the committed managed doc whose
/// landed finalize posts its file-state baseline (finalize phase 7).
fn commit_prior_adr(repo: &Path, home: &Path) {
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start` (task 0)");
    let task = "cache-sessions-in-a-single";

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (task 0)");

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot(
        "adr:single-node-cache#context",
        b"Session lookups must stay sub-millisecond.\n",
    );
    set_slot(
        "adr:single-node-cache#decision",
        b"A single in-memory node keeps lookups fast.\n",
    );
    set_slot(
        "adr:single-node-cache#consequences",
        b"A cold node loses its sessions.\n",
    );
    fill_commit(repo, home, task);

    let out = jigc(repo, home, &["task", "finalize", task, "--format", "json"]);
    assert_ok(&out, "`jigc task finalize` (task 0)");
    assert!(
        repo.join(ADR_PATH).exists(),
        "task 0 must promote {ADR_PATH}"
    );
}

/// Mint a commit-only task with one staged code file + a filled commit doc.
fn stage_commit_only(repo: &Path, home: &Path, task: &str, intent: &str) {
    let out = jigc(repo, home, &["start", "--workflow", "single-task", intent]);
    assert_ok(&out, &format!("`jigc start` ({task})"));
    fs::write(repo.join(format!("{task}.txt")), "the code change\n").expect("write code change");
    git(repo, &["add", &format!("{task}.txt")]);
    fill_commit(repo, home, task);
}

/// The findings array of a parsed JSON report envelope.
fn parse_envelope(stdout: &str, what: &str) -> Vec<serde_json::Value> {
    let value: serde_json::Value = serde_json::from_str(stdout).unwrap_or_else(|err| {
        panic!("{what}: stdout must parse as the report envelope ({err}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("{what}: the envelope must carry a `findings` array; got:\n{stdout}")
        })
        .clone()
}

/// The single `reconciliation.rename` finding naming `path`, or a panic if absent.
fn rename_finding(findings: &[serde_json::Value], path: &str, what: &str) -> serde_json::Value {
    let matches: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| {
            f["code"] == "reconciliation.rename"
                && f["message"].as_str().is_some_and(|m| m.contains(path))
        })
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "{what}: exactly one `reconciliation.rename` finding must name {path}; got:\n{findings:#?}",
    );
    matches[0].clone()
}

/// (a) A `git reset --hard` past the ADR's creating commit — the path has no HEAD history,
/// so the dangling baseline downgrades to an **advisory** with a `jigc unmanage` prune
/// route, and the next task's `task validate` does **not** block.
#[test]
fn reset_hard_past_creating_commit_downgrades_to_advisory() {
    let repo = TempDir::new("reset");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let root = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();

    // Land + baseline the ADR (its finalize is the only commit on top of the root).
    commit_prior_adr(repo.path(), home.path());

    // Move HEAD back past the creating commit: the ADR leaves disk AND leaves HEAD's
    // history, while the gitignored file-state baseline survives the reset.
    git(repo.path(), &["reset", "--hard", &root]);
    assert!(
        !repo.path().join(ADR_PATH).exists(),
        "the reset removes the ADR from the working tree",
    );
    assert!(
        git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
            .trim()
            .is_empty(),
        "HEAD carries no history for the ADR path after the reset",
    );
    let record = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    assert!(
        fs::read_to_string(&record)
            .expect("the file-state baseline survives the reset")
            .contains(ADR_PATH),
        "the dangling baseline is still recorded",
    );

    // The next task previews the store sweep: the dangling baseline is advisory, not a block.
    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task, "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "a history-less dangling baseline must not block `task validate`; \
         got {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status,
    );
    let findings = parse_envelope(&stdout, "history-less validate");
    let rename = rename_finding(&findings, ADR_PATH, "history-less validate");
    assert_eq!(
        rename["severity"], "advisory",
        "the history-less dangling baseline downgrades to advisory; got:\n{rename:#?}",
    );
    assert!(
        rename["route"]
            .as_str()
            .expect("the advisory carries a route")
            .contains(&format!("jigc unmanage {ADR_PATH}")),
        "the advisory routes prune-first to `jigc unmanage {ADR_PATH}`; got:\n{rename:#?}",
    );
}

/// (b) A `git rm <ADR>` + commit leaves the path with HEAD history — the weak-signal
/// finding still **blocks** (`reconciliation.rename`, exit 3), never silently downgraded.
#[test]
fn git_rm_with_history_still_blocks() {
    let repo = TempDir::new("gitrm");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    // Delete the committed ADR through git — the deletion commit gives the path history.
    git(repo.path(), &["rm", "-q", ADR_PATH]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "docs: drop the cache ADR"],
    );
    assert!(
        !git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
            .trim()
            .is_empty(),
        "HEAD carries history for the deleted ADR path",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task, "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !out.status.success(),
        "a genuine deletion (history present) must still block; got success\nstdout:\n{stdout}",
    );
    assert_eq!(
        out.status.code(),
        Some(3),
        "a blocking validate exits 3; got {:?}",
        out.status,
    );
    let findings = parse_envelope(&stdout, "genuine-deletion validate");
    let rename = rename_finding(&findings, ADR_PATH, "genuine-deletion validate");
    assert_eq!(
        rename["severity"], "blocking",
        "a genuine deletion keeps blocking; got:\n{rename:#?}",
    );
}

/// (c) A bare content-preserving `git mv` is the strong-signal arm the history gate leaves
/// untouched — the `reconciliation.rename` finding names both paths and routes to `jigc
/// rename`, and still blocks.
#[test]
fn bare_git_mv_still_surfaces_the_strong_finding() {
    const MOVED_PATH: &str = "docs/decisions/renamed-cache.md";
    let repo = TempDir::new("gitmv");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    // A content-preserving move: the moved file carries the recorded hash (strong signal).
    git(repo.path(), &["mv", ADR_PATH, MOVED_PATH]);
    assert!(
        repo.path().join(MOVED_PATH).exists() && !repo.path().join(ADR_PATH).exists(),
        "the git mv relocates the ADR with its content intact",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task, "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !out.status.success(),
        "a strong-signal rename still blocks; got success\nstdout:\n{stdout}",
    );
    let findings = parse_envelope(&stdout, "git-mv validate");
    let rename = rename_finding(&findings, ADR_PATH, "git-mv validate");
    assert_eq!(
        rename["severity"], "blocking",
        "the strong-signal rename is untouched by the history gate; got:\n{rename:#?}",
    );
    assert!(
        rename["message"]
            .as_str()
            .expect("the strong finding carries a message")
            .contains("git mv"),
        "the strong finding names the suspected `git mv`; got:\n{rename:#?}",
    );
    assert!(
        rename["route"]
            .as_str()
            .expect("the strong finding carries a route")
            .contains("jigc rename"),
        "the strong finding routes to `jigc rename`; got:\n{rename:#?}",
    );
}
