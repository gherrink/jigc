//! The `hook_output` **producer axis** (confidence-audit sibling-hunt item 4, class:
//! JSON purity / `hook_output`) — every production site that runs a **hook-capable**
//! `git commit` on the user's behalf must surface a non-blocking hook's captured
//! stream to the caller, both ways where the surface is a JSON document
//! (`design/command-output-contract.md` → Stream discipline: the always-present
//! `hook_output` envelope key **and** the stderr relay carry the *same* captured
//! string) — never dropped (`design/surface-contract.md` law 2: nothing hides).
//!
//! The axis, enumerated from the code — every production caller of the one
//! hook-capable commit seam (`cli/task.rs` → `git_commit_capture`; git runs the
//! user's `pre-commit`/`commit-msg` hooks on every `git commit` not passed
//! `--no-verify`):
//!
//!   1. **task finalize** (whole-index per-task commits) — swept at M45 Inc 9
//!      (`machine_output.rs`, `finalize_outcome_surface.rs`).
//!   2. **milestone finalize aggregate** (squash:true combine / squash:false
//!      aggregate) — swept at M45 Inc 9 (`tests/milestone.rs`).
//!   3. **milestone finalize per-sub-task chain commits** (squash:false N+1) — every
//!      chain commit's stream folds into the same landed envelope; the arm lives in
//!      `tests/milestone.rs::milestone_finalize_json_envelope_folds_every_fan_out_commits_hook_output`
//!      (it needs that suite's fan-out machinery).
//!   4. **`jigc rename`** (the atomic identity-refactor self-commit) — the arm HERE.
//!   5. **`jigc migrate-corpus`** (the pathspec-limited self-commit — one commit per
//!      run) — the arm HERE.
//!   6. **milestone record-only commits** (`create` / `add-task` / `add-from-spec` /
//!      `discard`, all through `commit_record_only`) — the arms HERE (create /
//!      add-task / discard; `add-from-spec` shares the same `append_and_commit_record`
//!      seam `add-task` drives).
//!
//!   **Excluded with reason:** `jigc setup`'s install commit passes `--no-verify` by
//!   recorded design (the hook it installs must not self-trigger on the commit that
//!   installs it; `cli/setup.rs` → `commit_install`) — no hook runs, so there is no
//!   stream to surface. Every other `git commit` in the tree is `#[cfg(test)]`
//!   fixture machinery.
//!
//! A new commit site added later joins this axis: route it through
//! `task::git_commit_capture` (which *returns* the captured stream, so dropping it is
//! visible at the call site) and add its arm here.
//!
//! Each arm installs a **non-blocking** `pre-commit` hook that speaks a distinctive
//! marker and exits 0, drives the producer through the real binary, and asserts the
//! marker is visible to the caller — RED before the fix: rename discarded the captured
//! stream (`rename.rs` — `commit?;`), migrate-corpus ran its commit through the
//! output-discarding `git_run`, and the record-only commits captured stderr only for
//! the rejection message.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The marker the non-blocking hook speaks — distinctive enough that any surfacing of
/// it in an output stream can only have come from the hook.
const MARKER: &str = "NON-BLOCKING-HOOK-AXIS-MARKER";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-hook-output-axis-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker arm requires it absent).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Initialize a real git repo with one commit and an (empty) `.jigc/config` project
/// layer — the minimum `require_project_layer` accepts; the embedded pack supplies
/// everything else.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
}

/// Install a **non-blocking** `pre-commit` hook that prints [`MARKER`] to stderr and
/// exits 0 — git surfaces it on every hook-capable commit's stderr (the hook stream
/// `git_commit_capture` returns on success).
fn install_marker_hook(repo: &Path) {
    let hooks = repo.join(".git").join("hooks");
    fs::create_dir_all(&hooks).expect("mk hooks");
    let hook = hooks.join("pre-commit");
    fs::write(&hook, format!("#!/bin/sh\necho {MARKER} 1>&2\nexit 0\n"))
        .expect("write the non-blocking pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}

/// A conformant `adr` body under the pack adr schema, optionally carrying a
/// `schema-version` stamp (`None` = the unstamped v0 state the corpus migration lifts).
fn adr_body(title: &str, stamp: Option<u32>) -> String {
    let stamp_line = match stamp {
        Some(v) => format!("schema-version: {v}\n"),
        None => String::new(),
    };
    format!(
        "\
---
status: accepted
date: 2026-06-25
{stamp_line}---

# {title}

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
"
    )
}

/// Commit an `adr` at its canonical `docs/decisions/<slug>.md`.
fn commit_adr(repo: &Path, slug: &str, title: &str, stamp: Option<u32>) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_body(title, stamp)).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
}

/// The shared per-producer assertion for a `--format json` run: stdout parses as
/// **exactly one** JSON document whose `hook_output` string (at `pointer`) carries the
/// hook's marker, and stderr carries the delimited relay of the *same* captured text
/// (one capture, two channels — `design/command-output-contract.md` → Stream
/// discipline).
fn assert_hook_surfaced_json(out: &std::process::Output, pointer: &str, what: &str) {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "{what} must exit 0 under a non-blocking hook; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    let value: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap_or_else(|err| {
        panic!("{what}: stdout must parse as exactly one JSON document ({err}); got:\n{stdout}")
    });
    let hook_output = value
        .pointer(pointer)
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| {
            panic!("{what}: the envelope must carry a `hook_output` string at `{pointer}`; got:\n{stdout}")
        });
    assert!(
        hook_output.contains(MARKER),
        "{what}: `hook_output` must carry the hook's captured text; got:\n{stdout}",
    );
    assert!(
        stderr.contains(&format!("--- hook output ---\n{hook_output}")),
        "{what}: the stderr relay must carry the same captured string as `hook_output`; \
         stderr:\n{stderr}",
    );
}

/// **Producer 4 — `jigc rename`.** The atomic identity-refactor commit runs the user's
/// hooks; a non-blocking hook's stream must reach the caller — the `hook_output` key on
/// the JSON report + the stderr relay, and the delimited stdout section on agent-text.
/// RED before: `rename.rs` bound the captured stream and dropped it (`commit?;`).
#[test]
fn rename_surfaces_hook_output_on_envelope_and_stderr() {
    let repo = TempDir::new("rename");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(2));
    install_marker_hook(repo.path());

    // The JSON surface: envelope key + stderr relay.
    let renamed = jigc(
        repo.path(),
        home.path(),
        &[
            "--format",
            "json",
            "rename",
            "adr:alpha-decision",
            "--to",
            "Beta decision",
        ],
    );
    assert_hook_surfaced_json(&renamed, "/hook_output", "`jigc --format json rename`");

    // The agent-text surface: the delimited section rides stdout with the summary.
    let back = jigc(
        repo.path(),
        home.path(),
        &["rename", "adr:beta-decision", "--to", "Alpha decision"],
    );
    let stdout = String::from_utf8_lossy(&back.stdout);
    assert!(
        back.status.success(),
        "the agent-text rename must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&back.stderr),
    );
    assert!(
        stdout.contains("--- hook output ---") && stdout.contains(MARKER),
        "the agent-text rename must relay the hook stream in the delimited section; got:\n{stdout}",
    );
}

/// **Producer 5 — `jigc migrate-corpus`.** The verb's pathspec-limited self-commit
/// (one commit per run) runs the user's hooks; the captured stream must reach the
/// report — the always-present `hook_output` key + the stderr relay. RED before: the
/// commit ran through the output-discarding `git_run`.
#[test]
fn migrate_corpus_surfaces_hook_output_on_envelope_and_stderr() {
    let repo = TempDir::new("migrate-corpus");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // An unstamped (v0) committed ADR — the stamp add-field migration writes it back
    // and the commit boundary lands it, so the hook genuinely fires.
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);
    install_marker_hook(repo.path());

    let migrated = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "migrate-corpus"],
    );
    assert_hook_surfaced_json(
        &migrated,
        "/hook_output",
        "`jigc --format json migrate-corpus`",
    );
    // The run genuinely committed (the hook had a commit to fire on).
    let value: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&migrated.stdout).trim())
            .expect("stdout parses");
    assert!(
        value["commit"].is_string(),
        "the migration must have landed its self-commit; got:\n{}",
        String::from_utf8_lossy(&migrated.stdout),
    );
}

/// **Producer 6 — the milestone record-only commits.** `create` / `add-task` /
/// `discard` each land a record-only path-scoped commit through `commit_record_only`
/// with the user's hooks live; each ack's JSON summary envelope must carry the
/// captured stream beside `text`, with the stderr relay alongside. RED before:
/// `git_commit_pathspec` captured stderr only for the rejection message and returned
/// `()` on success.
#[test]
fn milestone_record_ops_surface_hook_output_on_envelope_and_stderr() {
    let repo = TempDir::new("milestone-record");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // The `[dev ▸ methodology]` compose marker — the record-home split materializes +
    // commits the record only when the `milestone-record` schema resolves.
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    install_marker_hook(repo.path());

    let created = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "milestone", "create", "Cache rework"],
    );
    assert_hook_surfaced_json(&created, "/hook_output", "`jigc milestone create`");

    let added = jigc(
        repo.path(),
        home.path(),
        &[
            "--format",
            "json",
            "milestone",
            "add-task",
            "cache-rework",
            "Alpha fix",
        ],
    );
    assert_hook_surfaced_json(&added, "/hook_output", "`jigc milestone add-task`");

    let discarded = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "milestone", "discard", "cache-rework"],
    );
    assert_hook_surfaced_json(&discarded, "/hook_output", "`jigc milestone discard`");
}
