//! **The post-sweep hand-off cell of the file-state merge axis** (M46 Increment 1 /
//! T1 — `DECISIONS.md` 2026-08-18 M46 planned, N-3; `design/reconciliation.md` →
//! Persistence of the shifted baseline).
//!
//! The engine-side cells of the mutation-kind axis (`record` × `forget`, both ways)
//! live next to the code they exercise, in `engine::file_state`'s unit tests. The
//! third axis member cannot: it is the **post-sweep hand-off**, and it only exists
//! through the real binary, across a real `git commit`, with a real hook process in
//! the middle.
//!
//! The hand-off is the wave's most important path and the longest-lived base in the
//! product. `task finalize` loads the file-state record in its preflight sweep
//! (`cli/task.rs` → `Task::validate`), carries the swept record **by value** through
//! staging and the `git commit` — during which git runs the user's `pre-commit`
//! hook, a whole other process free to write the very same shared file — and only
//! then saves it in phase 7 (`post_commit` → `advance_file_state`). Before the
//! merge, that phase-7 save wrote the long-held record verbatim and silently
//! resurrected whatever the hook had just retired.
//!
//! The cell: a `pre-commit` hook runs **`jigc unmanage`** — a shipped, register-only,
//! git-index-free record writer (`cli/unmanage.rs`) — against a committed managed doc
//! the task never touches, retiring its file-state baseline mid-commit. After the
//! finalize lands, that baseline must **stay** retired.
//!
//! Non-vacuity is asserted, not assumed: the subject path is checked to be outside
//! the finalize's two own update sources — `hash_updates` (the plan's promoted
//! managed-doc set) and `git_commit_files` (the paths the landed commit touched) —
//! so the cell cannot pass because the finalize happened to re-record the subject
//! itself, and the record is checked to have genuinely advanced for the task's own
//! code file, so it cannot pass because phase 7 silently did nothing.
//!
//! RED before the merge: the subject's baseline is back on disk at exit 0.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The committed managed doc whose baseline the hook retires mid-commit — the
/// subject of the hand-off. The task below never opens it.
const SUBJECT: &str = "docs/decisions/planted-baseline.md";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-fs-hand-off-{tag}-{}-{:?}",
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
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run a `git` command in `repo`, returning its stdout.
fn git_out(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("git stdout is utf-8")
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`.
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
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// A conformant, current-stamp `adr` body.
fn adr_body(title: &str) -> String {
    format!(
        "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

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

/// Commit the subject ADR with plain git, then adopt it through `jigc ingest` so its
/// **file-state baseline is already on disk** before the finalize starts. That matters:
/// the merge takes the other writer's decision only for a key we did not touch
/// ourselves, and a baseline the preflight sweep had to *adopt* would be our own
/// deliberate write. Seeding it first puts the subject in the loaded base, which is
/// exactly the hand-off's real shape.
fn plant_baselined_subject(repo: &Path, home: &Path) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(
        dir.join("planted-baseline.md"),
        adr_body("Planted baseline"),
    )
    .expect("write the subject adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "plant the subject adr"]);

    let out = jigc(repo, home, &["ingest"]);
    assert_ok(&out, "`jigc ingest` (adopt + baseline the subject)");
    assert!(
        file_state(repo).contains_key(SUBJECT),
        "precondition: `jigc ingest` must baseline `{SUBJECT}`; record:\n{:?}",
        file_state(repo),
    );
}

/// Install a `pre-commit` hook that runs `jigc unmanage <SUBJECT>` — a register-only
/// write to the shared `.jigc/state/file-state.json`, landed while `jigc task
/// finalize` is holding its own long-lived copy of that same record. The hook stages
/// nothing and touches no git index, so the commit's file set is unaffected by it.
fn install_unmanage_hook(repo: &Path) {
    let hooks = repo.join(".git").join("hooks");
    fs::create_dir_all(&hooks).expect("mk hooks");
    let hook = hooks.join("pre-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\n\"{}\" unmanage {SUBJECT} || exit 1\nexit 0\n",
            env!("CARGO_BIN_EXE_jigc"),
        ),
    )
    .expect("write the pre-commit hook");
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
}

/// The persisted file-state map, as `path → hash`.
fn file_state(repo: &Path) -> std::collections::BTreeMap<String, String> {
    let path = repo.join(".jigc").join("state").join("file-state.json");
    let bytes = fs::read(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).expect("file-state.json is parseable JSON");
    value
        .get("hashes")
        .and_then(|h| h.as_object())
        .expect("file-state.json carries a `hashes` object")
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
        .collect()
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

/// **The post-sweep hand-off cell.** A `pre-commit` hook retires the subject's
/// file-state baseline while `jigc task finalize` is mid-commit; the landed
/// finalize's phase-7 save must merge that retirement rather than overwrite it with
/// the record it loaded before the hook ever ran.
#[test]
fn a_hooks_record_write_survives_the_finalize_post_sweep_hand_off() {
    let repo = TempDir::new("handoff");
    let home = TempDir::new("home");
    init_repo(repo.path());
    plant_baselined_subject(repo.path(), home.path());
    install_unmanage_hook(repo.path());

    let task = "add-rate-limiter";
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "quick-fix", "add rate limiter"],
    );
    assert_ok(&out, "`jigc start`");

    // A staged code change so the per-task narrowing has something to commit.
    let code = format!("{task}.txt");
    fs::write(repo.path().join(&code), "the code change\n").expect("write the code change");
    git(repo.path(), &["add", &code]);
    fill_commit(repo.path(), home.path(), task);

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize`");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    // ── non-vacuity 1: the subject is outside `git_commit_files` ────────────────
    let committed: Vec<String> = git_out(
        repo.path(),
        &["diff-tree", "--no-commit-id", "-r", "--name-only", "HEAD"],
    )
    .lines()
    .map(str::to_string)
    .collect();
    assert!(
        !committed.iter().any(|p| p == SUBJECT),
        "the landed commit must not touch `{SUBJECT}` — otherwise phase 7 re-records it \
         from the commit and the cell passes vacuously; committed: {committed:?}",
    );

    // ── non-vacuity 2: the subject is outside `hash_updates` ────────────────────
    // `hash_updates` is the finalize plan's promoted managed-doc set, and every promoted
    // doc is named on the landed manifest. The manifest is finalize's *own* speech, so
    // the relayed hook stream is cut off first — the hook naturally names the doc it
    // just unmanaged, and that is the other writer talking, not this plan.
    let manifest = stdout
        .split("--- hook output ---")
        .next()
        .expect("stdout has a first segment");
    assert!(
        !manifest.contains(SUBJECT),
        "the finalize must not promote or name `{SUBJECT}` — it is a bystander doc, not \
         part of this task's plan; manifest:\n{manifest}",
    );

    // ── non-vacuity 3: phase 7 genuinely advanced the record ────────────────────
    let after = file_state(repo.path());
    assert!(
        after.contains_key(&code),
        "phase 7 must have advanced the record for the task's own committed code file \
         — otherwise the cell would pass on a save that never happened; record:\n{after:?}",
    );

    // ── the claim: the hook's retirement survived the hand-off ──────────────────
    assert!(
        !after.contains_key(SUBJECT),
        "the hook retired `{SUBJECT}`'s baseline mid-commit; the post-sweep record \
         finalize had been holding since its preflight must merge that retirement, \
         never resurrect the key; record:\n{after:?}",
    );
}
