//! M47 Increment 2 / T1 — **the record-commit transaction**: the four milestone
//! record-only doors capture the record's pre-image and restore it — worktree **and**
//! index — when the commit is rejected (`DECISIONS.md` 2026-07-26 → the M47 Settle,
//! Decision 4 — captured-pre-image rollback, the **fifth
//! staged-path family**; `design/finalize.md` → Rollback discipline;
//! `design/reconciliation.md` → Hash re-baselining, the fourth site;
//! `design/team-ready-state.md` → The commit model).
//!
//! Before this fix a single rejected `pre-commit` hook **bricked the milestone
//! permanently**: the record stayed written and staged, the baseline stayed at the
//! pre-write hash, and every later door (`add-task`, `discard --force`, `finalize`)
//! conflict-blocked forever while `jigc validate` reported a blocking
//! `file-state.hash-matches` at exit 0. The staged residue also tripped the **next
//! unrelated task's** carryover gate — a rejected milestone op silently blocking work
//! that has nothing to do with it.
//!
//! **The axis is the four doors** (`create` · `add-task` · `add-from-spec` ·
//! `discard`) × a rejecting `pre-commit` hook, each driven through the **real binary**
//! in its own throwaway `[dev ▸ methodology]` repo. Per door:
//!
//!   1. exit **non-zero**, with git's hook stderr **verbatim**;
//!   2. the record's on-disk bytes **byte-identical** to the pre-door state — **absent**
//!      for `create`, whose pre-image is `(bytes: absent, index: absent)`;
//!   3. `git ls-files --stage -- <record>` **byte-identical** to the pre-door entry
//!      (empty for `create`) — the index axis asserted as an **entry**, never as the
//!      weaker "not staged";
//!   4. `git status --porcelain` names **nothing** under the record's location;
//!   5. `jigc validate` exits 0 with **no** `file-state.hash-matches` and **no**
//!      `reconciliation.conflict-block` naming the record;
//!   6. the **carryover clause driven, not proxied** — with the hook removed,
//!      `jigc start` → author → `jigc task finalize` on an **unrelated** task lands at
//!      exit 0 with no `finalize.carried-staged` naming the record path.
//!
//! The baseline is deliberately **not** advanced by the rollback: nothing landed, so
//! nothing re-baselines and `design/reconciliation.md`'s re-baselining rule stands
//! untouched — assertion 5 is what proves the restored bytes still match the baseline
//! the *last landed* write recorded.

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
            "jigc-milestone-record-rollback-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning **raw** stdout (never
/// trimmed — the index-entry assertion is a byte comparison).
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
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Write the `[dev ▸ methodology]` compose marker and commit it — the composition that
/// resolves the `milestone-record` doctype (dev-only ships no record, so no door
/// commits and the axis is inert).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "compose marker"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs
/// (never inheriting a harness `JIGC_PACK_DIR` — the compose-marker path requires it
/// ABSENT).
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

/// The milestone under test — one title, one id, across every door.
const MILESTONE_TITLE: &str = "Cache rework";
const MILESTONE_ID: &str = "cache-rework";

/// The committed record's repo-relative path (docs-root-nested by the dev `docs-root`
/// knob the composition resolves).
const RECORD_SPEC: &str = "docs/milestone-records/cache-rework.md";

/// The record's location — the prefix `git status --porcelain` must never name after a
/// rejected door.
const RECORD_LOCATION: &str = "docs/milestone-records/";

/// The rejecting hook's own stderr — asserted **verbatim** in the door's output (the
/// hook channel is never wrapped or edited; `design/finalize.md` → the M40 refinement 3).
const HOOK_STDERR: &str = "record commit rejected by the test hook";

/// Install a `pre-commit` hook rejecting **any** commit that stages a milestone-record
/// path. Scoped to the record so the repo setup (and the carryover arm's ordinary task
/// finalize) is unaffected by installation order.
fn install_record_rejecting_hook(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nif git diff --cached --name-only | grep -q '^{RECORD_LOCATION}'; then\n  echo '{HOOK_STDERR}' >&2\n  exit 1\nfi\nexit 0\n"
        ),
    )
    .expect("write pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).expect("chmod hook");
    }
}

/// Remove the rejecting hook — the "fix the hook" half of the recoverability claim.
fn remove_hook(repo: &Path) {
    let _ = fs::remove_file(repo.join(".git").join("hooks").join("pre-commit"));
}

/// A committed 2-criteria spec — `add-from-spec`'s seed substrate.
/// Carries the `schema-version` stamp so the store sweep at assertion 5 adjudicates a
/// **current** corpus — an unstamped fixture would exit non-zero on
/// `schema-conformance.schema-version-current` and mask the assertion.
const TWO_CRITERIA_SPEC: &str = "\
---
schema-version: 1
---

# Rate limit

## Goal

Bound per-client request volume.

## Context

Downstream services enforced limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Admits within the window  {#admits-within}

Requests under the cap are admitted unchanged.
";

/// Write + commit the spec at its canonical committed path under docs-root.
fn commit_spec(repo: &Path) {
    let specs = repo.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join("rate-limit.md"), TWO_CRITERIA_SPEC).expect("write spec");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "add spec"]);
}

/// The four doors of the axis.
const DOORS: [&str; 4] = ["create", "add-task", "add-from-spec", "discard"];

/// Bring the repo to the state each door is entered from — everything the door needs
/// **already landed** (so the only rejected commit is the door's own).
fn setup_for(door: &str, repo: &Path, home: &Path) {
    init_repo(repo);
    write_compose_marker(repo);
    match door {
        // `create`'s pre-image is the absent record: no milestone yet.
        "create" => {}
        "add-task" => ok(
            repo,
            home,
            &["milestone", "create", MILESTONE_TITLE],
            "milestone create",
        ),
        "add-from-spec" => {
            commit_spec(repo);
            ok(
                repo,
                home,
                &["milestone", "create", MILESTONE_TITLE],
                "milestone create",
            );
        }
        "discard" => {
            ok(
                repo,
                home,
                &["milestone", "create", MILESTONE_TITLE],
                "milestone create",
            );
            ok(
                repo,
                home,
                &["milestone", "add-task", MILESTONE_ID, "Warm the read cache"],
                "milestone add-task",
            );
        }
        other => panic!("unknown door `{other}`"),
    }
}

/// The door's own argv (the record-only commit under test).
fn door_argv(door: &str) -> Vec<&'static str> {
    match door {
        "create" => vec!["milestone", "create", MILESTONE_TITLE],
        "add-task" => vec!["milestone", "add-task", MILESTONE_ID, "Evict cold entries"],
        "add-from-spec" => vec![
            "milestone",
            "add-from-spec",
            MILESTONE_ID,
            "spec:rate-limit",
        ],
        "discard" => vec!["milestone", "discard", MILESTONE_ID],
        other => panic!("unknown door `{other}`"),
    }
}

/// `git status --porcelain` lines naming anything under the record's location.
fn porcelain_under_record_location(repo: &Path) -> Vec<String> {
    git(repo, &["status", "--porcelain"])
        .lines()
        .filter(|line| line.contains(RECORD_LOCATION))
        .map(str::to_owned)
        .collect()
}

/// Drive an unrelated task end-to-end through the real binary — the **carryover clause,
/// driven rather than proxied**. Any residue left staged by the rejected door is what a
/// task-minting door snapshots, so a leaked record entry blocks this finalize with
/// `finalize.carried-staged`.
fn unrelated_task_finalizes_clean(repo: &Path, home: &Path, door: &str) {
    const TASK: &str = "unrelated-tidy";
    ok(
        repo,
        home,
        &["start", "--workflow", "single-task", "Unrelated tidy"],
        "jigc start (the unrelated task)",
    );
    fs::write(repo.join("tidy.txt"), "tidy\n").expect("write the unrelated task's edit");
    git(repo, &["add", "tidy.txt"]);
    // Explicitly `--task`-scoped: a rejected door can leave its own sub-task area behind
    // (that unwind is T2/T3's), so more than one task may be active here.
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{TASK}#type"),
            "--value",
            "chore",
            "--task",
            TASK,
        ],
        "doc set-field type",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{TASK}#scope"),
            "--value",
            "tidy",
            "--task",
            TASK,
        ],
        "doc set-field scope",
    );
    for (addr, prose) in [
        (
            format!("commit:{TASK}#summary"),
            &b"tidy an unrelated file\n"[..],
        ),
        (
            format!("commit:{TASK}#body"),
            &b"Work with no relation to the milestone record.\n"[..],
        ),
    ] {
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", &addr, "--from-file", "-", "--task", TASK],
            Some(prose),
        );
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }

    let out = jigc(repo, home, &["task", "finalize", TASK], None);
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !combined.contains("finalize.carried-staged"),
        "door `{door}`: a rejected record commit must leave NO staged residue for the next \
         unrelated task's carryover gate; got:\n{combined}",
    );
    assert!(
        out.status.success(),
        "door `{door}`: the unrelated task's finalize must land at exit 0 after a rejected \
         record commit; got {:?}\n{combined}",
        out.status,
    );
}

/// The axis: **the four record-only doors × a rejecting `pre-commit` hook**. Each door
/// leaves the record's worktree bytes and its index entry byte-identical to the
/// pre-door state, names nothing under the record's location, keeps `jigc validate`
/// clean of the wreckage findings, and leaves no staged residue for the next unrelated
/// task's carryover gate.
#[test]
fn the_four_record_only_doors_restore_the_records_pre_image_on_a_rejected_commit() {
    for door in DOORS {
        let repo = TempDir::new(door);
        let home = TempDir::new(&format!("home-{door}"));
        let repo = repo.path();
        let home = home.path();
        setup_for(door, repo, home);

        // The pre-door image — worktree bytes (absent for `create`) and the index entry
        // (empty for `create`), both captured as raw bytes for a byte-identity compare.
        let pre_bytes: Option<Vec<u8>> = fs::read(repo.join(RECORD_SPEC)).ok();
        let pre_index = git(repo, &["ls-files", "--stage", "--", RECORD_SPEC]);
        if door == "create" {
            assert!(
                pre_bytes.is_none() && pre_index.is_empty(),
                "door `create` must be entered from the ABSENT pre-image (bytes: absent, \
                 index: absent); got bytes {:?} / index {pre_index:?}",
                pre_bytes.as_ref().map(|b| b.len()),
            );
        } else {
            assert!(
                pre_bytes.is_some() && !pre_index.is_empty(),
                "door `{door}` must be entered from a committed record; got bytes {:?} / \
                 index {pre_index:?}",
                pre_bytes.as_ref().map(|b| b.len()),
            );
        }
        let head_before = git(repo, &["rev-parse", "HEAD"]);

        install_record_rejecting_hook(repo);
        let out = jigc(repo, home, &door_argv(door), None);
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );

        // (1) Non-zero, with the hook's stderr verbatim.
        assert!(
            !out.status.success(),
            "door `{door}` must exit non-zero when its record commit is rejected; got \
             success:\n{combined}",
        );
        assert!(
            combined.contains(HOOK_STDERR),
            "door `{door}` must surface the hook's stderr VERBATIM; got:\n{combined}",
        );
        assert_eq!(
            git(repo, &["rev-parse", "HEAD"]),
            head_before,
            "door `{door}`: a rejected record commit lands nothing",
        );

        // (2) The worktree bytes are byte-identical to the pre-door image.
        let now_bytes: Option<Vec<u8>> = fs::read(repo.join(RECORD_SPEC)).ok();
        assert_eq!(
            now_bytes.as_deref().map(String::from_utf8_lossy),
            pre_bytes.as_deref().map(String::from_utf8_lossy),
            "door `{door}`: the record's on-disk bytes must be byte-identical to the \
             pre-door state (absent stays absent)",
        );

        // (3) The index axis — asserted as an ENTRY, never as "not staged".
        assert_eq!(
            git(repo, &["ls-files", "--stage", "--", RECORD_SPEC]),
            pre_index,
            "door `{door}`: the record's index entry must be byte-identical to the \
             pre-door entry",
        );

        // (4) Nothing under the record's location is dirty.
        assert!(
            porcelain_under_record_location(repo).is_empty(),
            "door `{door}`: `git status --porcelain` must name nothing under \
             `{RECORD_LOCATION}`; got {:?}",
            porcelain_under_record_location(repo),
        );

        // (5) `jigc validate` stops reporting the wreckage.
        let validate = jigc(repo, home, &["validate"], None);
        let report = format!(
            "{}{}",
            String::from_utf8_lossy(&validate.stdout),
            String::from_utf8_lossy(&validate.stderr)
        );
        assert!(
            validate.status.success(),
            "door `{door}`: `jigc validate` must exit 0 after the rollback; got {:?}\n{report}",
            validate.status,
        );
        assert!(
            !report.contains("file-state.hash-matches"),
            "door `{door}`: `jigc validate` must not report `file-state.hash-matches` over \
             the rolled-back record; got:\n{report}",
        );
        assert!(
            !(report.contains("reconciliation.conflict-block") && report.contains(RECORD_SPEC)),
            "door `{door}`: no `reconciliation.conflict-block` may name the record; got:\n{report}",
        );

        // (6) The carryover clause, driven through the real binary.
        remove_hook(repo);
        unrelated_task_finalizes_clean(repo, home, door);
    }
}
