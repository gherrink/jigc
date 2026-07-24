//! M45 Increment 9, T1 — the exit-code taxonomy suite
//! (`design/command-output-contract.md` → The exit-code taxonomy;
//! `implementation/pinning.md` → §2, one suite per pinned statement).
//!
//! The `0/1/2/3/4` vocabulary is the outermost layer of the machine contract — a
//! driver reads the exit code before a byte of output. This suite provokes each
//! outcome class through the real binary, across the verb families, and asserts the
//! severity→exit mapping **against the one minted table constant**
//! ([`cli::task::EXIT_CODES`], read via [`cli::task::exit_code_for`]) — never a hand
//! literal, so a code that moved in the table without moving here is unrepresentable:
//!
//!   - **success (0)** — a clean store-scope `jigc validate`;
//!   - **usage (2)** — an unknown subcommand, rejected by clap before jigc reads
//!     `--format` (the declared clap carve-out — its own arm, plain text, exit 2);
//!   - **blocking finding at a task-scope gate (3)** — `jigc task validate` over a task
//!     whose commit doc is unfilled;
//!   - **store-scope validate exit-flip (1)** — `jigc validate` over an unmigrated (v0)
//!     managed ADR corpus, the third store-scope exit-flipping exception;
//!   - **migration review hold (4)** — a migration `jigc task finalize` without
//!     `--approve`.
//!
//! Drives the built `jigc` binary against throwaway `git init` temp repos over the
//! shipped dev pack (`JIGC_PACK_DIR`), a self-cleaning `TempDir` per repo.

use cli::task::{ExitClass, exit_code_for};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-exitcodes-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` (byte-identical to
/// the binary-embedded pack, so setting it is harmless and pins the pack this suite reads).
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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

/// Initialize a real git repo with identity + one commit + the `.jigc/config/` layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = dev pack`,
/// optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", dev_pack())
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

/// Assert `out` exits 0, surfacing both streams on failure; return trimmed stdout.
fn ok_stdout(out: Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// The heart of the suite: assert `out`'s exit code is the taxonomy table's code for
/// `class` — read from [`EXIT_CODES`] via [`exit_code_for`], never a hand literal.
fn assert_exit_class(out: &Output, class: ExitClass, what: &str) {
    let expected = exit_code_for(class);
    assert_eq!(
        out.status.code(),
        Some(i32::from(expected)),
        "{what} must exit {expected} ({class:?}) per the EXIT_CODES table; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 0 — success: a clean store-scope `jigc validate` over a freshly-set-up repo.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn clean_store_validate_exits_success() {
    let repo = TempDir::new("success");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        jigc(repo.path(), home.path(), &["setup"], None),
        "jigc setup",
    );

    let out = jigc(repo.path(), home.path(), &["validate"], None);
    assert_exit_class(
        &out,
        ExitClass::Success,
        "a clean store-scope `jigc validate`",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 2 — usage: an unknown subcommand, rejected by clap (the declared carve-out).
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn unknown_subcommand_exits_usage() {
    let repo = TempDir::new("usage");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["definitely-not-a-real-verb"],
        None,
    );
    assert_exit_class(&out, ExitClass::Usage, "an unknown subcommand (clap usage)");
    assert!(
        out.stdout.is_empty(),
        "a usage error prints to stderr, never the JSON funnel on stdout; got:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 3 — blocking findings at a task-scope gate: `jigc task validate` over an unfilled
//     commit doc (its author-required fields empty → a blocking schema-conformance
//     finding). The task gate, not the store sweep.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn blocked_task_validate_exits_task_gate_blocked() {
    let repo = TempDir::new("taskgate");
    let home = TempDir::new("home");
    init_repo(repo.path());

    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "add a widget"],
            None,
        ),
        "jigc start",
    );

    // The freshly-provisioned commit doc has empty author-required fields — a blocking
    // finding at the task gate. No authoring, so the block is the only outcome.
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "add-a-widget"],
        None,
    );
    assert_exit_class(
        &out,
        ExitClass::TaskGateBlocked,
        "a blocking `jigc task validate`",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 1 — store-scope validate exit-flip: `jigc validate` over an unmigrated (v0) managed
//     ADR corpus flips the store sweep's exit to 1 (schema-conformance.schema-version-
//     current — the third store-scope exit-flipping exception). Maps to ExitClass::Error.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn store_validate_exit_flip_exits_error() {
    let repo = TempDir::new("flip");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        jigc(repo.path(), home.path(), &["setup"], None),
        "jigc setup",
    );

    // Commit an unstamped (v0) ADR at its canonical managed home — a managed corpus the
    // store sweep detects as unmigrated.
    let dir = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("single-node-cache.md"), UNSTAMPED_ADR).expect("write v0 adr");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed v0 adr"]);

    let out = jigc(repo.path(), home.path(), &["validate"], None);
    assert_exit_class(
        &out,
        ExitClass::Error,
        "a store-scope `jigc validate` over an unmigrated corpus (the exit-flip)",
    );
}

/// A conformant `adr` body under the pack schema, with **no** `schema-version` stamp —
/// the unmigrated v0 state the store sweep flags + exit-flips.
const UNSTAMPED_ADR: &str = "\
---
status: accepted
date: 2026-06-25
---

# Single-node cache

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

// ─────────────────────────────────────────────────────────────────────────────
// 4 — migration review hold: a migration `jigc task finalize` without `--approve`
//     renders the fidelity diff and stops (exit 4). Reaches the review gate only over a
//     conformant staged migration (an unconformant one would exit 3 first).
// ─────────────────────────────────────────────────────────────────────────────

/// The off-router migration task id `jigc migrate HISTORY.md --as changelog` mints — the
/// empty intent slugs the `migrate-<doctype>-<slug(path)>` fallback (mirrors
/// `migrate_review_gate.rs`).
const MIGRATION_TASK: &str = "migrate-changelog-history-3268e06b69e1";

const FOREIGN_CHANGELOG: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

#[test]
fn migration_finalize_without_approve_exits_review_hold() {
    let repo = TempDir::new("review");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        jigc(repo.path(), home.path(), &["setup"], None),
        "jigc setup",
    );

    fs::write(repo.path().join("HISTORY.md"), FOREIGN_CHANGELOG).expect("write foreign");
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["migrate", "HISTORY.md", "--as", "changelog"],
            None,
        ),
        "jigc migrate",
    );
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "create",
                "changelog",
                "--title",
                "Changelog",
                "--task",
                MIGRATION_TASK,
            ],
            None,
        ),
        "doc create changelog",
    );

    // Author the single release oldest-to-newest with a historical date + one change
    // group, so the staged changelog is conformant and finalize reaches the review gate.
    let release = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "0.1.0",
                "--task",
                MIGRATION_TASK,
            ],
            None,
        ),
        "add-item release",
    );
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{release}/date"),
                "--value",
                "2021-03-09",
                "--task",
                MIGRATION_TASK,
            ],
            None,
        ),
        "set-field date",
    );
    let group = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
                "--task",
                MIGRATION_TASK,
            ],
            None,
        ),
        "add-item change-group",
    );
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
                "--task",
                MIGRATION_TASK,
            ],
            Some(b"First public release.\n"),
        ),
        "set-slot notes",
    );
    make_commit_conformant(repo.path(), home.path(), MIGRATION_TASK);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", MIGRATION_TASK],
        None,
    );
    assert_exit_class(
        &out,
        ExitClass::MigrationReview,
        "a migration finalize without --approve",
    );
    // The hold committed nothing — HEAD is unmoved, the foreign original byte-intact.
    assert!(
        repo.path().join("HISTORY.md").exists(),
        "a review hold adopts nothing and retires nothing",
    );
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "a review hold writes no canonical doc",
    );
}

/// Fill every author-required field/slot of `task`'s provisioned commit doc so a
/// finalize over it validates clean (the review gate sits behind the validation gate).
fn make_commit_conformant(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        ok_stdout(
            jigc(
                repo,
                home,
                &["doc", "set-field", addr, "--value", value, "--task", task],
                None,
            ),
            "set-field commit",
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        ok_stdout(
            jigc(
                repo,
                home,
                &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
                Some(prose),
            ),
            "set-slot commit",
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "changelog");
    set_slot(
        &format!("commit:{task}#summary"),
        b"adopt the migrated changelog\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Migrate the foreign HISTORY.md into managed shape.\n",
    );
}
