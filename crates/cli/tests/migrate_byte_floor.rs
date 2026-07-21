//! M44 Increment 5, T2 — the byte-floor triviality advisory on the migrate source.
//!
//! Done-criterion: `jigc migrate <below-floor-source> --as adr`
//!   - exits **0** and composes the migration workflow exactly as before (the foreign
//!     content surfaces through the source seam);
//!   - its **agent/human** output carries a `migrate.trivial-source` advisory that names
//!     the byte floor and routes to the from-knowledge `jigc start --workflow
//!     record-decision …` path (the honest alternative to migrating a placeholder source,
//!     `auto-migration.md` → The byte-floor advisory; `surface-contract.md` law 1);
//!   - under `--format json` the pinned `{task, text}` contract is **byte-identical** to
//!     the pre-rider projection — the advisory is presentation-only and never leaks into
//!     the JSON stdout an agent's tooling reads (`command-output-contract.md`);
//!   - a **normal-sized** foreign source emits **no** advisory (the floor is a mechanical
//!     byte count, Framing-A — not a content judgment).
//!
//! Drives the built `jigc` binary over the shipped dev pack (`JIGC_PACK_DIR` = the
//! embedded `pack/` tree) against throwaway `git init` temp repos.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-migrate-byte-floor-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
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

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer,
/// then `jigc setup`.
fn setup_repo(repo: &Path, home: &Path, pack: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    let out = run_jigc(repo, home, pack, &["setup"]);
    assert!(
        out.status.success(),
        "jigc setup must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
}

/// A near-empty placeholder source, well below the byte floor — the rc.7 loophole
/// (an agent minting a stub file just to reach the migrate authoring path).
const TRIVIAL_SOURCE: &str = "# Decision\n\nTBD.\n";

/// A realistic foreign ADR, comfortably above the byte floor — a genuine source to
/// migrate, so no advisory fires.
const NORMAL_SOURCE: &str = "\
# 1. Use PostgreSQL

Date: 2019-02-12

## Status

Accepted

## Context

We need a relational store with strong transactional guarantees.

## Decision

Use PostgreSQL as the primary datastore.

## Consequences

Operational familiarity is high; JSONB covers the semi-structured cases.
";

#[test]
fn below_floor_source_composes_and_carries_the_trivial_source_advisory() {
    let repo = TempDir::new("advisory");
    let home = TempDir::new("home");
    let pack = dev_pack();
    setup_repo(repo.path(), home.path(), &pack);

    fs::write(repo.path().join("stub.md"), TRIVIAL_SOURCE).expect("write trivial source");
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "stub.md", "--as", "adr"],
    );

    // Non-blocking: the advisory does not stop the migrate.
    assert!(
        out.status.success(),
        "a below-floor migrate must still exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // Composes the migration workflow exactly as before — the foreign content reaches
    // the source seam and a task is minted.
    assert!(
        stdout.contains("task minted:"),
        "the migrate must still mint + compose:\n{stdout}",
    );
    assert!(
        stdout.contains("TBD."),
        "the composed view must still surface the foreign source through the seam:\n{stdout}",
    );

    // The advisory rides the agent/human presentation surface (stdout, default format).
    assert!(
        stdout.contains("migrate.trivial-source"),
        "the below-floor migrate must carry the trivial-source advisory:\n{stdout}",
    );
    assert!(
        stdout.contains("advisory"),
        "the finding must render at advisory severity:\n{stdout}",
    );
    // Names the byte floor (the message states the concrete `<N>-byte` threshold; the
    // floor is calibrated below every shipped foreign fixture — see `migrate.rs`).
    assert!(
        stdout.contains("48-byte"),
        "the advisory must name the byte floor (48-byte):\n{stdout}",
    );
    // Routes to the from-knowledge record-decision path (T1's workflow id).
    assert!(
        stdout.contains("jigc start --workflow record-decision"),
        "the advisory must route to the from-knowledge record-decision path:\n{stdout}",
    );
}

#[test]
fn below_floor_json_contract_is_byte_identical_and_advisory_free_on_stdout() {
    let repo = TempDir::new("json");
    let home = TempDir::new("home");
    let pack = dev_pack();
    setup_repo(repo.path(), home.path(), &pack);

    fs::write(repo.path().join("stub.md"), TRIVIAL_SOURCE).expect("write trivial source");
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "stub.md", "--as", "adr", "--format", "json"],
    );
    assert!(out.status.success(), "the json migrate must exit 0");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // The pinned `{task, text}` contract is untouched: stdout parses to an object whose
    // key set is EXACTLY {task, text} — the advisory is presentation-only and never
    // leaks into the JSON an agent's tooling reads.
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("the json migrate stdout must be one JSON object ({e}):\n{stdout}")
    });
    let object = value.as_object().expect("the json contract is an object");
    let mut keys: Vec<&String> = object.keys().collect();
    keys.sort();
    assert_eq!(
        keys,
        vec!["task", "text"],
        "the pinned json contract stays exactly {{task, text}} — no advisory key:\n{stdout}",
    );
    assert!(
        !stdout.contains("migrate.trivial-source"),
        "the advisory must NOT leak into the json stdout contract:\n{stdout}",
    );
}

#[test]
fn below_floor_non_adr_source_does_not_route_to_the_adr_workflow() {
    // The byte-floor rider (S2) was scoped to the adr placeholder-source loophole — its
    // advice ("author the decision from knowledge") and route (`jigc start --workflow
    // record-decision`) produce an ADR, not the doctype being migrated. A trivial NON-adr
    // migration must therefore never carry that adr-producing route (surface-contract law
    // 1: nothing lies).
    let repo = TempDir::new("nonadr");
    let home = TempDir::new("home");
    let pack = dev_pack();
    setup_repo(repo.path(), home.path(), &pack);

    fs::write(repo.path().join("cl.md"), TRIVIAL_SOURCE).expect("write trivial source");
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "cl.md", "--as", "changelog"],
    );
    assert!(
        out.status.success(),
        "a below-floor non-adr migrate must still exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    // Still composes exactly as before — the trivial-source scope change is advisory-only.
    assert!(
        stdout.contains("task minted:"),
        "the non-adr migrate must still mint + compose:\n{stdout}",
    );

    // The lie: a changelog (or any non-adr) migration must NOT be routed to the
    // adr-producing `record-decision` workflow.
    assert!(
        !stdout.contains("record-decision") && !stderr.contains("record-decision"),
        "a non-adr migrate must not route to the adr `record-decision` workflow:\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    // The advisory is scoped to adr; a non-adr trivial source emits none.
    assert!(
        !stdout.contains("migrate.trivial-source") && !stderr.contains("migrate.trivial-source"),
        "the trivial-source advisory is adr-scoped — no non-adr migration carries it:\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

#[test]
fn normal_source_emits_no_advisory() {
    let repo = TempDir::new("normal");
    let home = TempDir::new("home");
    let pack = dev_pack();
    setup_repo(repo.path(), home.path(), &pack);

    fs::write(repo.path().join("adr.md"), NORMAL_SOURCE).expect("write normal source");
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "adr.md", "--as", "adr"],
    );
    assert!(out.status.success(), "the normal migrate must exit 0");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    assert!(
        !stdout.contains("migrate.trivial-source") && !stderr.contains("migrate.trivial-source"),
        "a normal-sized foreign source must emit no trivial-source advisory:\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}
