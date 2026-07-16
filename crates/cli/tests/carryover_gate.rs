//! M43 Increment 2 / T3 — the **carryover gate** at `jigc task finalize`
//! (`design/surface-contract.md` → The carryover gate; the trial's #1-ranked v1 gate).
//!
//! A foreign change staged **before the task existed** (a pre-mint `git add` /
//! `git rm`) must not silently ride the task's whole-index commit. Finalize's
//! committing path refuses with **one blocking routed `finalize.carried-staged` per
//! carried path** (file-path target form — mid-carry the path may be foreign, with no
//! managed identity), overridable with `--carry-staged` (the `--approve` mold:
//! undecidable intent converted to a declared one; on a migration finalize it
//! **composes** with `--approve` — two independent declarations). The task's own
//! post-mint staging never trips the gate, a `--dry-run` still renders its forecast
//! (the refuse sits on the committing path only), and a migration's recorded retire
//! pathspec is exempt (that deletion is the task's own work).
//!
//! Every arm drives the real binary and asserts on the emitted findings envelope /
//! the landed git commit — never a reconstruction.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-carryover-gate-{tag}-{}-{:?}",
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit (a tracked `README.md`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        "doc set-field type",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "gate",
        ],
        "doc set-field scope",
    );
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_slot(&format!("commit:{task}#summary"), b"gate the carryover\n");
    set_slot(&format!("commit:{task}#body"), b"An M43 change.\n");
}

/// Mint a `single-task` task and do its work: write + `git add` a task edit
/// (post-mint staging — the task's own), fill the commit doc.
fn mint_and_work(repo: &Path, home: &Path, intent: &str, task: &str, edit: &str) {
    ok(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "jigc start",
    );
    fs::write(repo.join(edit), "pub fn work() {}\n").expect("write task edit");
    git(repo, &["add", edit]);
    fill_commit(repo, home, task);
}

/// The blocked finalize's pinned findings envelope (stdout, `--format json`),
/// asserting the run blocked with the validation exit (3).
fn blocked_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    assert_eq!(
        out.status.code(),
        Some(3),
        "`{what}` must block with the validation exit (3); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`{what}` blocks with the pinned findings envelope on stdout ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// The `finalize.carried-staged` findings in an envelope.
fn carried_staged(findings: &[serde_json::Value]) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["code"] == "finalize.carried-staged")
        .cloned()
        .collect()
}

/// Assert one carried-staged finding is blocking, keyed at `path` (the file-path
/// target form), and routed at both exits (unstage, or declare with `--carry-staged`).
fn assert_carried(finding: &serde_json::Value, path: &str) {
    assert_eq!(
        finding["severity"], "blocking",
        "the carryover refusal is blocking; got: {finding}"
    );
    assert_eq!(
        finding["key"]["target"], path,
        "the finding keys at the carried file path; got: {finding}"
    );
    let route = finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("the refusal carries a route; got: {finding}"));
    assert!(
        route.contains("--carry-staged") && route.contains("git restore --staged"),
        "the route names both exits (unstage, or `--carry-staged`); got: {route}"
    );
}

/// A **pre-mint staged add** blocks finalize — one routed blocking
/// `finalize.carried-staged` **per carried path** (two staged foreign files → exactly
/// two findings, each keyed at its own path) — while `--dry-run` still renders its
/// forecast (the refuse sits on the committing path only, after the dry-run branch).
#[test]
fn a_pre_mint_staged_add_blocks_with_one_routed_finding_per_path() {
    let repo = TempDir::new("add");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // The foreign pre-staged adds — staged BEFORE the task exists.
    fs::write(repo.path().join("foreign-a.txt"), "not this task's work\n").expect("write a");
    fs::write(repo.path().join("foreign-b.txt"), "also not\n").expect("write b");
    git(repo.path(), &["add", "foreign-a.txt", "foreign-b.txt"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    // The forecast still renders: the refuse must not block the `--dry-run` branch.
    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--dry-run"],
        "jigc task finalize --dry-run",
    );

    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--format", "json"],
            None,
        ),
        "jigc task finalize (pre-mint staged adds)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(
        carried.len(),
        2,
        "exactly ONE finding per carried path (two foreign paths staged); got:\n{findings:#?}"
    );
    assert_carried(&carried[0], "foreign-a.txt");
    assert_carried(&carried[1], "foreign-b.txt");
    assert!(
        !carried_staged(&findings)
            .iter()
            .any(|f| f["key"]["target"] == "feature.rs"),
        "the task's own post-mint staged edit is never carried; got:\n{findings:#?}"
    );

    // Nothing committed: the refusal precedes any side effect.
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        before,
        "the blocked finalize commits nothing"
    );
}

/// A **pre-mint staged modify** of a tracked file blocks the same way.
#[test]
fn a_pre_mint_staged_modify_blocks() {
    let repo = TempDir::new("modify");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // The foreign pre-staged modification of the tracked README.
    fs::write(repo.path().join("README.md"), "hello\nforeign edit\n").expect("modify README");
    git(repo.path(), &["add", "README.md"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--format", "json"],
            None,
        ),
        "jigc task finalize (pre-mint staged modify)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(carried.len(), 1, "one carried path; got:\n{findings:#?}");
    assert_carried(&carried[0], "README.md");
}

/// A **pre-mint staged deletion** (`git rm` before the mint) blocks too — the
/// staged-deletion half an entry-only snapshot is structurally blind to (the trial's
/// A7 case), and the message says it is a deletion.
#[test]
fn a_pre_mint_staged_deletion_blocks() {
    let repo = TempDir::new("delete");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // A second tracked file whose deletion the user pre-stages.
    fs::write(repo.path().join("doomed.txt"), "to be removed\n").expect("write doomed");
    git(repo.path(), &["add", "doomed.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "track doomed.txt"]);
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // The foreign pre-staged deletion — staged BEFORE the task exists.
    git(repo.path(), &["rm", "-q", "doomed.txt"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--format", "json"],
            None,
        ),
        "jigc task finalize (pre-mint staged deletion)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(carried.len(), 1, "one carried path; got:\n{findings:#?}");
    assert_carried(&carried[0], "doomed.txt");
    assert!(
        carried[0]["message"]
            .as_str()
            .expect("message is a string")
            .contains("deletion"),
        "the message names the deletion half; got: {}",
        carried[0]
    );
}

/// **Post-mint staging never trips the gate**: a task whose index was clean at mint
/// stages its own work and finalizes clean — no `--carry-staged` needed.
#[test]
fn post_mint_staging_never_trips_the_gate() {
    let repo = TempDir::new("post-mint");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", task],
        "jigc task finalize (post-mint staging only)",
    );
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|l| l == "feature.rs"),
        "the task's own staged edit lands; files:\n{committed}"
    );
}

/// `--carry-staged` **lands the declared carryover**: the same pre-mint staged add
/// that blocks above rides the whole-index commit once declared.
#[test]
fn carry_staged_lands_the_declared_carryover() {
    let repo = TempDir::new("declared");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    fs::write(repo.path().join("foreign-a.txt"), "deliberately carried\n").expect("write a");
    git(repo.path(), &["add", "foreign-a.txt"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--carry-staged"],
        "jigc task finalize --carry-staged",
    );
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|l| l == "foreign-a.txt")
            && committed.lines().any(|l| l == "feature.rs"),
        "the declared carryover AND the task's own edit land in the one commit; files:\n{committed}"
    );
}

// ───────────────────────── the migration composition ─────────────────────────

/// The foreign Keep-a-Changelog file the migration arms consume.
const FOREIGN_CHANGELOG: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

/// The declarative whole-doc payload that re-authors the canonical changelog.
const PAYLOAD_CHANGELOG: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 0.1.0
        set:
          date: 2021-03-09
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- First public release.>>"
"#;

/// Mint the changelog migration task over a committed `HISTORY.md` and author the
/// canonical doc; returns the migration task id.
fn mint_migration_and_author(repo: &Path, home: &Path) -> String {
    ok(
        repo,
        home,
        &["migrate", "HISTORY.md", "--as", "changelog"],
        "jigc migrate HISTORY.md --as changelog",
    );
    let task = "migrate-changelog-history";
    let out = jigc(
        repo,
        home,
        &[
            "doc",
            "author",
            "changelog",
            "--from-file",
            "-",
            "--task",
            task,
        ],
        Some(PAYLOAD_CHANGELOG.as_bytes()),
    );
    assert!(
        out.status.success(),
        "`jigc doc author changelog` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    task.to_string()
}

/// A migration finalize over a foreign pre-mint staged change needs **both**
/// declarations, independently: `--approve` alone still refuses on the carryover
/// (exit 3), `--carry-staged` alone still holds at the review gate (exit 4), and
/// the two together land the commit.
#[test]
fn a_migration_finalize_needs_approve_and_carry_staged_together() {
    let repo = TempDir::new("migration");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // The foreign original, committed; plus a tracked file whose foreign
    // modification is pre-staged before the migration mints.
    fs::write(repo.path().join("HISTORY.md"), FOREIGN_CHANGELOG).expect("write HISTORY.md");
    git(repo.path(), &["add", "HISTORY.md"]);
    git(repo.path(), &["commit", "-q", "-m", "track HISTORY.md"]);
    fs::write(repo.path().join("README.md"), "hello\nforeign edit\n").expect("modify README");
    git(repo.path(), &["add", "README.md"]);

    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    let task = mint_migration_and_author(repo.path(), home.path());

    // `--approve` alone: the fidelity declaration does NOT declare the carryover.
    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", &task, "--approve", "--format", "json"],
            None,
        ),
        "jigc task finalize --approve (carryover undeclared)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(carried.len(), 1, "one carried path; got:\n{findings:#?}");
    assert_carried(&carried[0], "README.md");

    // `--carry-staged` alone: the carryover declaration does NOT approve the
    // fidelity diff — the review gate still holds (exit 4, nothing committed).
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--carry-staged"],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(4),
        "`--carry-staged` alone holds at the review gate; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // Both declarations together land the one whole-index commit.
    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--approve", "--carry-staged"],
        "jigc task finalize --approve --carry-staged",
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.contains("A\tCHANGELOG.md")
            && name_status.contains("D\tHISTORY.md")
            && name_status.contains("M\tREADME.md"),
        "the landed commit carries the migration AND the declared carryover; got:\n{name_status}"
    );
}

/// The migration's **retire pathspec is exempt**: a user who pre-staged the source's
/// own deletion (`git rm --cached HISTORY.md` before `jigc migrate`) finalizes with
/// `--approve` alone — that deletion is the task's own work, never a carryover.
#[test]
fn the_migration_retire_pathspec_is_exempt() {
    let repo = TempDir::new("exempt");
    let home = TempDir::new("home");
    init_repo(repo.path());
    fs::write(repo.path().join("HISTORY.md"), FOREIGN_CHANGELOG).expect("write HISTORY.md");
    git(repo.path(), &["add", "HISTORY.md"]);
    git(repo.path(), &["commit", "-q", "-m", "track HISTORY.md"]);

    // The source's own deletion, pre-staged BEFORE the migration mints (`--cached`
    // keeps the worktree bytes for `jigc migrate` to read).
    git(repo.path(), &["rm", "-q", "--cached", "HISTORY.md"]);

    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    let task = mint_migration_and_author(repo.path(), home.path());

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--approve"],
        "jigc task finalize --approve (retire pathspec exempt)",
    );
    assert!(
        repo.path().join("CHANGELOG.md").exists() && !repo.path().join("HISTORY.md").exists(),
        "the migration lands: canonical doc promoted, foreign source retired"
    );
}
