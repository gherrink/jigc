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
//!
//! **T2 — the mint unwinds with its record.** T1 restores the *committed* half; the
//! *workbench* half survived, so the re-run hit `milestone.serial-collision` /
//! `milestone.sub-task-collision` — honest, but not the approved recoverability
//! (*"fix the hook, re-run, it succeeds"*). Two assertions join the same axis, and
//! the axis card is the **what each door minted in the same call** enumeration:
//!
//!   2b. the door's own mint is **unwound** — `create` removed the milestone area it
//!       created; `add-task` removed the sub-task area and restored `tasks.json` to its
//!       pre-append bytes; `discard` mints nothing and its workbench is asserted to
//!       **survive** (the already-safe member, pinned so a future teardown-before-commit
//!       reddens here); `add-from-spec` is **excluded with its reason** — its k−1 landed
//!       record commits make its recovery a *resume*, not an unwind, and it is T3's;
//!   7. the **recovery arm** — with the hook removed the *identical* door argv re-runs at
//!      exit 0 and lands what it always would have: `create` lands its record commit,
//!      `add-task` leaves `tasks.json` + `.jigc/tasks/` + the committed record naming
//!      exactly one sub-task each (no orphan area, no duplicate item) with
//!      `jigc milestone list-tasks` and `jigc doc show … --format json` agreeing on the
//!      set, `discard` settles the record and removes the workbench — and `jigc validate`
//!      is clean at exit 0 afterwards.

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

/// The sub-task the `add-task` door mints — its intent, and the frozen slug that intent
/// mints to. The recovery arm counts **that one identity** in three places (the demoted
/// `tasks.json` cache, the `.jigc/tasks/` area, the committed record), so it is named once.
const ADD_TASK_INTENT: &str = "Evict cold entries";
const ADD_TASK_SUB_ID: &str = "evict-cold-entries";

/// The milestone's **gitignored workbench** `.jigc/milestones/<id>/` — what `create` mints
/// and what `discard`'s teardown removes.
fn milestone_area(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join(MILESTONE_ID)
}

/// The milestone's demoted task-list cache — the file `add-task` appends to and whose
/// pre-append bytes its unwind restores. Keyed on the engine's own constant, so the suite
/// cannot pin a second spelling of a name the engine decides.
fn task_list_path(repo: &Path) -> PathBuf {
    milestone_area(repo).join(engine::milestone::TASKS_FILE)
}

/// The sub-task working areas whose directory name starts with `prefix` — the orphan probe
/// (a leftover `evict-cold-entries` blocks the re-run; a suffixed `evict-cold-entries-2`
/// would be a *duplicate* identity, which the count catches).
fn sub_task_areas(repo: &Path, prefix: &str) -> Vec<String> {
    let mut names: Vec<String> = match fs::read_dir(repo.join(".jigc").join("tasks")) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().into_string().ok())
            .filter(|name| name.starts_with(prefix))
            .collect(),
        Err(_) => Vec::new(),
    };
    names.sort();
    names
}

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
        "add-task" => vec!["milestone", "add-task", MILESTONE_ID, ADD_TASK_INTENT],
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

/// **T2 assertion 2b — the door's own mint is unwound.** Each door unwinds exactly what
/// *it* minted in the same call, so the axis card is per-door and every member is
/// covered / excluded-with-reason / already-safe rather than brainstormed:
///
/// - `create` mints `.jigc/milestones/<id>/` → **removed** (never one it *found*: the
///   engine's serial collision refuses before the mint, so a returned area was created here);
/// - `add-task` mints `.jigc/tasks/<sub>/` and appends `tasks.json` → **both undone**, the
///   task list restored to its captured pre-append bytes;
/// - `add-from-spec` mints N sub-tasks with k−1 record commits already **landed**, so its
///   recovery is a resume rather than an unwind — **T3's**, excluded here with that reason;
/// - `discard` mints nothing (its teardown runs *after* the commit) → the workbench must
///   **survive**, asserted so a future teardown-before-commit reddens on this arm.
fn assert_mint_unwound(door: &str, repo: &Path, pre_task_list: Option<&Vec<u8>>) {
    match door {
        "create" => assert!(
            !milestone_area(repo).exists(),
            "door `create`: a rejected record commit must remove the milestone area it minted \
             (the re-run would otherwise block on `milestone.serial-collision`); {:?} survives",
            milestone_area(repo),
        ),
        "add-task" => {
            assert!(
                sub_task_areas(repo, ADD_TASK_SUB_ID).is_empty(),
                "door `add-task`: a rejected record commit must remove the sub-task area it \
                 minted (the re-run would otherwise block on `task.serial-collision`); got {:?}",
                sub_task_areas(repo, ADD_TASK_SUB_ID),
            );
            assert_eq!(
                fs::read(task_list_path(repo)).ok().as_ref(),
                pre_task_list,
                "door `add-task`: a rejected record commit must restore `tasks.json` to its \
                 captured pre-append bytes (the re-run would otherwise block on \
                 `milestone.sub-task-collision`, and the cache would name a sub-task the record \
                 does not)",
            );
        }
        // `add-from-spec` — T3 owns the mid-loop unwind + resume (the k−1 landed record commits
        // cannot be unwound), so this arm asserts nothing about its workbench here.
        "add-from-spec" => {}
        "discard" => assert!(
            milestone_area(repo).exists(),
            "door `discard` mints nothing and tears down only AFTER its record commit lands — a \
             rejected commit must leave the workbench {:?} intact for the re-run",
            milestone_area(repo),
        ),
        other => panic!("unknown door `{other}`"),
    }
}

/// Every `task-id` value the pinned `doc show --format json` shape carries, in document
/// order — walked recursively rather than read at a fixed path, so the set assertion binds
/// to the *values* the contract emits and not to this suite's model of the envelope.
fn record_task_ids(value: &serde_json::Value) -> Vec<String> {
    let mut out = Vec::new();
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if key == "task-id"
                    && let Some(id) = child.as_str()
                {
                    out.push(id.to_string());
                }
                out.extend(record_task_ids(child));
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                out.extend(record_task_ids(item));
            }
        }
        _ => {}
    }
    out
}

/// The committed record's sub-task set, read through the **pinned** read contract
/// (`jigc doc show milestone-record:<id> --format json`).
fn recorded_task_ids(repo: &Path, home: &Path) -> Vec<String> {
    let show = jigc(
        repo,
        home,
        &[
            "doc",
            "show",
            &format!("milestone-record:{MILESTONE_ID}"),
            "--format",
            "json",
        ],
        None,
    );
    let stdout = String::from_utf8_lossy(&show.stdout).into_owned();
    assert!(
        show.status.success(),
        "`jigc doc show milestone-record:{MILESTONE_ID} --format json` must exit 0; stdout:\n\
         {stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&show.stderr),
    );
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("the pinned `--format json` read parses");
    record_task_ids(&json)
}

/// The workbench's sub-task set, read through `jigc milestone list-tasks` (whose line is
/// `milestone:<id> tasks (<n>): <id>, <id>`).
fn listed_task_ids(repo: &Path, home: &Path) -> Vec<String> {
    let out = jigc(repo, home, &["milestone", "list-tasks", MILESTONE_ID], None);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        out.status.success(),
        "`jigc milestone list-tasks {MILESTONE_ID}` must exit 0; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let line = stdout
        .lines()
        .find(|line| line.starts_with(&format!("milestone:{MILESTONE_ID} tasks")))
        .unwrap_or_else(|| panic!("`list-tasks` must print its enumeration line; got:\n{stdout}"));
    let (_, ids) = line
        .split_once("): ")
        .unwrap_or_else(|| panic!("`list-tasks` must print `(<n>): <ids>`; got:\n{line}"));
    ids.split(", ")
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .collect()
}

/// **T2 assertion 7 — the recovery arm**: *fix the hook, re-run, it succeeds*. The door is
/// re-run with the **identical argv** (never a repaired one) and must land what it always
/// would have, leaving `jigc validate` clean at exit 0.
fn assert_door_recovers(door: &str, repo: &Path, home: &Path) {
    let head_before = git(repo, &["rev-parse", "HEAD"]);
    match door {
        "create" => {
            ok(
                repo,
                home,
                &door_argv(door),
                "the identical `milestone create` re-run, hook removed",
            );
            assert_ne!(
                git(repo, &["rev-parse", "HEAD"]),
                head_before,
                "door `create`: the recovered re-run lands its record commit",
            );
            assert_eq!(
                git(repo, &["show", "--name-only", "--format=", "HEAD"]).trim(),
                RECORD_SPEC,
                "door `create`: the recovered commit is record-only",
            );
            assert!(
                milestone_area(repo).exists(),
                "door `create`: the recovered re-run mints the workbench it unwound",
            );
        }
        "add-task" => {
            ok(
                repo,
                home,
                &door_argv(door),
                "the identical `milestone add-task` re-run, hook removed",
            );
            // Exactly one `evict-cold-entries` in all three homes — no orphan area, no
            // duplicate item, no duplicate cache entry.
            assert_eq!(
                sub_task_areas(repo, ADD_TASK_SUB_ID),
                vec![ADD_TASK_SUB_ID.to_string()],
                "door `add-task`: exactly one sub-task area survives the reject → re-run pair",
            );
            let cache = fs::read_to_string(task_list_path(repo)).expect("the task-list cache");
            assert_eq!(
                cache.matches(ADD_TASK_SUB_ID).count(),
                1,
                "door `add-task`: the task-list cache names `{ADD_TASK_SUB_ID}` exactly once; \
                 got:\n{cache}",
            );
            let recorded = recorded_task_ids(repo, home);
            assert_eq!(
                recorded,
                vec![ADD_TASK_SUB_ID.to_string()],
                "door `add-task`: the committed record names `{ADD_TASK_SUB_ID}` exactly once",
            );
            let mut listed = listed_task_ids(repo, home);
            listed.sort();
            let mut recorded_sorted = recorded;
            recorded_sorted.sort();
            assert_eq!(
                listed, recorded_sorted,
                "door `add-task`: `jigc milestone list-tasks` and the committed record must agree \
                 on the sub-task set",
            );
        }
        // `add-from-spec`'s recovery is a **resume**, not a re-run of the identical argv — T3's.
        "add-from-spec" => return,
        "discard" => {
            ok(
                repo,
                home,
                &door_argv(door),
                "the identical `milestone discard` re-run, hook removed",
            );
            let show = jigc(
                repo,
                home,
                &[
                    "doc",
                    "show",
                    &format!("milestone-record:{MILESTONE_ID}"),
                    "--format",
                    "json",
                ],
                None,
            );
            let json = String::from_utf8_lossy(&show.stdout).into_owned();
            assert!(
                show.status.success() && json.contains("\"status\": \"discarded\""),
                "door `discard`: the recovered re-run settles the record at `discarded`; got:\n\
                 {json}",
            );
            assert!(
                !milestone_area(repo).exists(),
                "door `discard`: the recovered re-run removes the workbench",
            );
        }
        other => panic!("unknown door `{other}`"),
    }

    let validate = jigc(repo, home, &["validate"], None);
    assert!(
        validate.status.success(),
        "door `{door}`: `jigc validate` must exit 0 after the recovered re-run; got {:?}\n{}{}",
        validate.status,
        String::from_utf8_lossy(&validate.stdout),
        String::from_utf8_lossy(&validate.stderr),
    );
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
    // Explicitly `--task`-scoped: the `add-from-spec` door still leaves its own sub-task areas
    // behind (that unwind is T3's), so more than one task may be active here.
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
        // The **workbench** half of the pre-door image (T2): the task-list bytes `add-task`
        // appends to. Absent for `create`, whose milestone area does not exist yet.
        let pre_task_list: Option<Vec<u8>> = fs::read(task_list_path(repo)).ok();
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

        // (2b) The door's own mint is unwound — the workbench half (T2).
        assert_mint_unwound(door, repo, pre_task_list.as_ref());

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

        // (7) The recovery arm — fix the hook, re-run, it succeeds (T2). Deliberately AFTER
        // the carryover arm: the recovered door stages the record path itself, so running it
        // first would commit any residue and mask (6).
        assert_door_recovers(door, repo, home);
    }
}

/// A record at the doctype's committed home that was **never committed** — an operator's
/// hand-drafted file, or (before T1) the residue of a rejected `create`. Its shape is what
/// `render_fresh_record` writes, so the door's status probe reads it back.
const UNCOMMITTED_RECORD: &str = "\
---
base: 4d1df0f0c90610a03d98c4199c6f6088952731ff 4d1df0f
status: active
schema-version: 2
---

# cache-rework

## Tasks
";

/// **T2's law-1 repair**: `create`'s id-is-taken refusal states what it actually found.
///
/// [`guard_record_free`](../../src/milestone.rs) probes the record's **existence on disk** —
/// it runs no git read and cannot know whether the file was ever committed — yet
/// `milestone.record-exists` said *"already has a **committed** record"*. The repro that
/// surfaced it was the rejected `create` (a record written, staged, and never landed, over
/// which the re-run then claimed a commit that never happened); T1 removed *that* instance,
/// but the lie survives wherever the record is present-and-uncommitted, which a hand-drafted
/// record is. The refusal itself is correct and its route already resolves — only the claim
/// about provenance goes (`design/surface-contract.md` → law 1: nothing lies).
#[test]
fn create_refuses_over_an_uncommitted_record_without_claiming_it_is_committed() {
    let repo = TempDir::new("uncommitted-record");
    let home = TempDir::new("home-uncommitted-record");
    let repo = repo.path();
    let home = home.path();
    init_repo(repo);
    write_compose_marker(repo);

    // The record exists on disk and is **untracked** — never committed, by construction.
    fs::create_dir_all(repo.join(RECORD_LOCATION)).expect("mk the record home");
    fs::write(repo.join(RECORD_SPEC), UNCOMMITTED_RECORD).expect("write the untracked record");
    assert!(
        git(repo, &["ls-files", "--", RECORD_SPEC]).is_empty(),
        "the fixture record must be untracked — the whole point is that nothing committed it",
    );

    let out = jigc(repo, home, &["milestone", "create", MILESTONE_TITLE], None);
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !out.status.success() && combined.contains("milestone.record-exists"),
        "the id-is-taken refusal still fires over a record that already owns the slug; got:\n\
         {combined}",
    );
    assert!(
        !combined.contains("committed record"),
        "law 1: the refusal must not claim the record was COMMITTED — it probed the file's \
         existence, not git; got:\n{combined}",
    );
    assert!(
        combined.contains(&format!("milestone `{MILESTONE_ID}` already has a record")),
        "the refusal states what it actually found — a record already owns the slug; got:\n\
         {combined}",
    );
    assert!(
        combined.contains("(its record reads `active`)")
            && combined.contains(&format!(
                "jigc milestone add-task {MILESTONE_ID} \"<intent>\""
            )),
        "the status decoration and the continue-it route survive the repair; got:\n{combined}",
    );
    assert!(
        !milestone_area(repo).exists(),
        "the refusal runs ahead of every write — no milestone area is minted",
    );
}

// ---------------------------------------------------------------------------------------
// T3 — `add-from-spec`'s mid-loop becomes atomic and resumable.
//
// The three sibling doors mint once and commit once, so a rejection unwinds the whole call.
// `add-from-spec` mints N sub-tasks and lands **one record-only commit per sub-task**, so a
// rejected k-th commit has k−1 commits already in history: they cannot be unwound, and the
// approved *"fix the hook, re-run, it succeeds"* is therefore a **resume**, not a re-run of an
// untouched state. Two halves, one invariant:
//
//   (a) **atomic** — the k-th rejection unwinds the mints the record never named (`k..N`), so
//       the committed record and the demoted `tasks.json` cache name the identical k−1
//       sub-tasks in the identical order, the record is unstaged and byte-identical to its
//       k−1 state, and no orphan task area survives for an un-recorded criterion;
//   (b) **resumable** — a criterion whose sub-task id the milestone's task list already
//       carries is *skipped*, never collided, so the re-run seeds exactly the remaining
//       N−k+1 and acks the already-seeded count.
//
// The axis is **k ∈ 1..=N over an N=3 spec** (never one reported repro): k=1 exercises the
// nothing-landed edge, k=N the everything-but-the-last. The re-run is **lifted verbatim from
// the rejection's own printed line** — the emitted bytes are the contract, and a hand-built
// equivalent would pass over a route that lies. A fully-seeded re-run closes the loop:
// nothing to seed is a true ack, never a collision.
// ---------------------------------------------------------------------------------------

/// A committed **3-criteria** spec — the N of T3's axis. The titles are the engine suite's
/// (the slugs they mint are pinned there too) and are deliberately not in id-sorted physical
/// order, so "the same k−1 sub-tasks in the same order" is a real assertion rather than one a
/// sorted list would satisfy accidentally. Stamped, so the store sweep adjudicates a current
/// corpus.
const THREE_CRITERIA_SPEC: &str = "\
---
schema-version: 1
---

# Gateway rate limiting

## Goal

Bound per-client request volume at the gateway.

## Context

Downstream services were each enforcing limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Admits within the window  {#admits-within}

Requests under the cap are admitted unchanged.

### Recovers after the window  {#recovers}

The next window admits requests again.
";

/// The 3-criteria spec's committed slug — its address is `spec:<slug>`.
const THREE_CRITERIA_SLUG: &str = "gateway-rate-limiting";

/// The sub-task ids the three criteria mint, in **physical criterion order** (the order both
/// the record and the task-list cache record them in).
const THREE_CRITERIA_IDS: [&str; 3] = [
    "rejects-the-101st-request",
    "admits-within-the-window",
    "recovers-after-the-window",
];

/// Write + commit the 3-criteria spec at its canonical committed path under docs-root.
fn commit_three_criteria_spec(repo: &Path) {
    let specs = repo.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(
        specs.join(format!("{THREE_CRITERIA_SLUG}.md")),
        THREE_CRITERIA_SPEC,
    )
    .expect("write the 3-criteria spec");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "add the 3-criteria spec"]);
}

/// Install a `pre-commit` hook rejecting **exactly the k-th** record commit (1-based),
/// counting record-staging commits in a `.git/`-local counter. Installed *after* the
/// milestone's `create` commit, so the count is `add-from-spec`'s own loop.
fn install_kth_record_rejecting_hook(repo: &Path, k: usize) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\n\
             if git diff --cached --name-only | grep -q '^{RECORD_LOCATION}'; then\n\
             \x20 n=$(cat .git/record-commit-count 2>/dev/null || echo 0)\n\
             \x20 n=$((n+1))\n\
             \x20 echo $n > .git/record-commit-count\n\
             \x20 if [ \"$n\" -eq {k} ]; then\n\
             \x20   echo '{HOOK_STDERR}' >&2\n\
             \x20   exit 1\n\
             \x20 fi\n\
             fi\n\
             exit 0\n"
        ),
    )
    .expect("write the k-th-rejecting pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).expect("chmod hook");
    }
}

/// The demoted task-list cache's ids in **recorded (physical) order** — read from the JSON
/// the engine writes, so the order assertion binds to the persisted state and not to
/// `list-tasks`' id-sorted projection. An absent cache is the empty list.
fn tasks_json_ids(repo: &Path) -> Vec<String> {
    let Ok(bytes) = fs::read(task_list_path(repo)) else {
        return Vec::new();
    };
    let json: serde_json::Value =
        serde_json::from_slice(&bytes).expect("the task-list cache parses as JSON");
    json.get("tasks")
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

/// Every sub-task working area under `.jigc/tasks/`, sorted — the **orphan probe**: an area
/// for a criterion the record does not name is exactly the residue that blocks the resume.
fn all_sub_task_dirs(repo: &Path) -> Vec<String> {
    let mut names: Vec<String> = match fs::read_dir(repo.join(".jigc").join("tasks")) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect(),
        Err(_) => Vec::new(),
    };
    names.sort();
    names
}

/// The record's bytes **as of HEAD** — the k−1 state, since the only commits touching the
/// record are its own record-only commits.
fn head_record_bytes(repo: &Path) -> String {
    git(repo, &["show", &format!("HEAD:{RECORD_SPEC}")])
}

/// The re-run command the mid-loop rejection printed, **lifted verbatim** from the emitted
/// line (the emitted bytes are the contract — a re-run rebuilt in test code would pass over a
/// route that lies about how to recover). Returns the full argv, leading `jigc`.
fn lifted_rerun_argv(combined: &str) -> Vec<String> {
    let line = combined
        .lines()
        .find(|line| line.contains("`jigc milestone add-from-spec"))
        .unwrap_or_else(|| {
            panic!(
                "the mid-loop rejection must PRINT the re-run command that recovers it; \
                 got:\n{combined}"
            )
        });
    let span = line.split('`').nth(1).unwrap_or_else(|| {
        panic!("the printed re-run command must be a backticked span; got:\n{line}")
    });
    let argv: Vec<String> = span.split_whitespace().map(str::to_owned).collect();
    assert_eq!(
        argv.first().map(String::as_str),
        Some("jigc"),
        "the lifted re-run argv leads with `jigc`; got {argv:?}",
    );
    argv
}

/// Assert the milestone's three homes name `expected` — the committed record and the demoted
/// cache in the identical order, and `.jigc/tasks/` exactly once each (no orphan, no duplicate).
fn assert_seeded_set(repo: &Path, home: &Path, expected: &[&str], what: &str) {
    let expected_ids: Vec<String> = expected.iter().map(|id| (*id).to_string()).collect();
    assert_eq!(
        recorded_task_ids(repo, home),
        expected_ids,
        "{what}: the committed record must name exactly {expected_ids:?}, in that order",
    );
    assert_eq!(
        tasks_json_ids(repo),
        expected_ids,
        "{what}: the demoted `tasks.json` cache must name exactly {expected_ids:?}, in that \
         order — the same set as the record",
    );
    let mut sorted = expected_ids.clone();
    sorted.sort();
    assert_eq!(
        all_sub_task_dirs(repo),
        sorted,
        "{what}: `.jigc/tasks/` must hold exactly one area per recorded sub-task — an area for \
         an un-recorded criterion is the orphan that blocks the resume",
    );
}

/// **T3's axis — k ∈ 1..=N over an N=3 spec.** For every k: the k-th record commit is
/// rejected, the un-recorded mints are unwound (record ≡ cache ≡ areas at k−1, the record
/// byte-identical to its k−1 state and unstaged), and the re-run **lifted from the
/// rejection's own printed line** resumes — seeding exactly the remaining N−k+1, acking the
/// already-seeded count, and leaving all N named once each. A final fully-seeded re-run
/// closes it: nothing to seed acks, never collides.
#[test]
fn add_from_spec_unwinds_its_un_recorded_mints_and_resumes_at_every_k() {
    const N: usize = THREE_CRITERIA_IDS.len();
    let spec_addr = format!("spec:{THREE_CRITERIA_SLUG}");

    for k in 1..=N {
        let repo = TempDir::new(&format!("from-spec-k{k}"));
        let home = TempDir::new(&format!("home-from-spec-k{k}"));
        let repo = repo.path();
        let home = home.path();
        init_repo(repo);
        write_compose_marker(repo);
        commit_three_criteria_spec(repo);
        ok(
            repo,
            home,
            &["milestone", "create", MILESTONE_TITLE],
            "milestone create",
        );

        // The k-th of `add-from-spec`'s N record commits is rejected — the hook is installed
        // AFTER `create`'s own record commit, so the count is this door's loop.
        install_kth_record_rejecting_hook(repo, k);
        let out = jigc(
            repo,
            home,
            &["milestone", "add-from-spec", MILESTONE_ID, &spec_addr],
            None,
        );
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            !out.status.success(),
            "k={k}: a rejected record commit must exit non-zero; got success:\n{combined}",
        );
        assert!(
            combined.contains(HOOK_STDERR),
            "k={k}: the hook's stderr stays VERBATIM; got:\n{combined}",
        );

        // (a) atomic — the record, the cache, and the areas all name the k−1 that landed.
        let landed: Vec<&str> = THREE_CRITERIA_IDS[..k - 1].to_vec();
        assert_seeded_set(repo, home, &landed, &format!("k={k}, after the rejection"));
        assert_eq!(
            fs::read_to_string(repo.join(RECORD_SPEC)).expect("read the record"),
            head_record_bytes(repo),
            "k={k}: the record on disk must be byte-identical to its k−1 state (HEAD)",
        );
        assert!(
            porcelain_under_record_location(repo).is_empty(),
            "k={k}: nothing under `{RECORD_LOCATION}` may be left dirty or staged; got {:?}",
            porcelain_under_record_location(repo),
        );

        // (b) resumable — the re-run is LIFTED from the rejection's own printed line.
        remove_hook(repo);
        let argv = lifted_rerun_argv(&combined);
        let rerun_args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
        let rerun = jigc(repo, home, &rerun_args, None);
        let rerun_out = format!(
            "{}{}",
            String::from_utf8_lossy(&rerun.stdout),
            String::from_utf8_lossy(&rerun.stderr)
        );
        assert!(
            rerun.status.success(),
            "k={k}: the re-run lifted from the rejection's own line must exit 0; \
             argv {argv:?}\n{rerun_out}",
        );
        assert!(
            rerun_out.contains(&format!("seeded {} sub-task(s)", N - k + 1)),
            "k={k}: the resume seeds exactly the remaining {}; got:\n{rerun_out}",
            N - k + 1,
        );
        if k > 1 {
            assert!(
                rerun_out.contains(&format!("{} already seeded", k - 1)),
                "k={k}: the resume acks the {} already-seeded criteri(on/a); got:\n{rerun_out}",
                k - 1,
            );
        } else {
            assert!(
                !rerun_out.contains("already seeded"),
                "k=1: nothing had landed, so the resume must claim no already-seeded \
                 criteria; got:\n{rerun_out}",
            );
        }
        assert_seeded_set(
            repo,
            home,
            &THREE_CRITERIA_IDS,
            &format!("k={k}, after the resume"),
        );
        ok(repo, home, &["validate"], "jigc validate after the resume");

        // The fully-seeded re-run — nothing to seed is a true ack, never a collision.
        let head_before = git(repo, &["rev-parse", "HEAD"]);
        let again = jigc(repo, home, &rerun_args, None);
        let again_out = format!(
            "{}{}",
            String::from_utf8_lossy(&again.stdout),
            String::from_utf8_lossy(&again.stderr)
        );
        assert!(
            again.status.success(),
            "k={k}: a fully-seeded re-run has nothing to seed and must exit 0; got:\n{again_out}",
        );
        assert!(
            !again_out.contains("collision"),
            "k={k}: a fully-seeded re-run must ACK, never collide; got:\n{again_out}",
        );
        assert!(
            again_out.contains("seeded 0 sub-task(s)")
                && again_out.contains(&format!("{N} already seeded")),
            "k={k}: the fully-seeded ack states both halves (0 seeded, {N} already seeded); \
             got:\n{again_out}",
        );
        assert_eq!(
            git(repo, &["rev-parse", "HEAD"]),
            head_before,
            "k={k}: a fully-seeded re-run mints nothing and commits nothing",
        );
        assert_seeded_set(
            repo,
            home,
            &THREE_CRITERIA_IDS,
            &format!("k={k}, after the fully-seeded re-run"),
        );
    }
}

// ---------------------------------------------------------------------------
// T3 fix — **the resume skip set is keyed on the committed record**, and the
// mid-mint abort unwinds like every other door.
// ---------------------------------------------------------------------------

/// A 3-criteria spec whose **third criterion slugs like its second** — the word cap keeps
/// the first [`engine::slug::MAX_WORDS`] words, so two criteria opening with the same words
/// mint the same sub-task id and the second one aborts the mint mid-loop. The producer T3's
/// record-commit unwind deliberately does not cover.
const COLLIDING_CRITERIA_SPEC: &str = "\
---
schema-version: 1
---

# Gateway rate limiting

## Goal

Bound per-client request volume at the gateway.

## Context

Downstream services were each enforcing limits ad hoc.

## Criteria

### Admits requests under the cap  {#admits}

Requests under the cap are admitted unchanged.

### The gateway rejects the 101st request in a rolling window  {#rejects-window}

The 101st request in a rolling 60s window is rejected.

### The gateway rejects the 101st request without a body  {#rejects-body}

A rejection carries no response body.
";

/// The same spec with the colliding third criterion **repaired** — the operator's fix, the
/// repair half of the mint-abort arm.
const REPAIRED_CRITERIA_SPEC: &str = "\
---
schema-version: 1
---

# Gateway rate limiting

## Goal

Bound per-client request volume at the gateway.

## Context

Downstream services were each enforcing limits ad hoc.

## Criteria

### Admits requests under the cap  {#admits}

Requests under the cap are admitted unchanged.

### The gateway rejects the 101st request in a rolling window  {#rejects-window}

The 101st request in a rolling 60s window is rejected.

### Body-less rejections carry a Retry-After header  {#rejects-body}

A rejection carries no response body.
";

/// The sub-task ids [`REPAIRED_CRITERIA_SPEC`] mints, in physical criterion order. Its first
/// two are what [`COLLIDING_CRITERIA_SPEC`] mints before its third aborts the loop.
const REPAIRED_CRITERIA_IDS: [&str; 3] = [
    "admits-requests-under-the-cap",
    "gateway-rejects-the-101st",
    "body-less-rejections-carry",
];

/// Write + commit `body` as the spec at `docs/specs/<THREE_CRITERIA_SLUG>.md`.
fn commit_spec_body(repo: &Path, body: &str, message: &str) {
    let specs = repo.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join(format!("{THREE_CRITERIA_SLUG}.md")), body).expect("write the spec");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", message]);
}

/// **The invariant this axis exists for**: `add-from-spec` never exits 0 while a criterion of
/// the spec it was pointed at is absent from the **committed record**. A positive ack over a
/// record that names fewer criteria than the spec is the silent, durable, fresh-clone-visible
/// loss the wave is named for — so this is asserted after *every* exit-0 run of every arm.
fn assert_no_criterion_lost(
    repo: &Path,
    home: &Path,
    out: &std::process::Output,
    criteria: &[&str],
    what: &str,
) {
    if !out.status.success() {
        return;
    }
    let recorded = recorded_task_ids(repo, home);
    let missing: Vec<&str> = criteria
        .iter()
        .copied()
        .filter(|id| !recorded.iter().any(|got| got == id))
        .collect();
    assert!(
        missing.is_empty(),
        "{what}: `add-from-spec` acked success while the committed record does not name \
         {missing:?} (it names {recorded:?}); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The producers of the divergence — **the axis**: every way the gitignored `.jigc` workbench
/// can come to name a sub-task the committed record does not.
const DIVERGENCE_PRODUCERS: [&str; 3] =
    ["record-commit-rejection", "mint-abort", "planted-residue"];

/// **The producer axis** (the M47 Inc 2 T3 fix). T3 made `add-from-spec` resumable by skipping
/// criteria the milestone "already carries" — and keyed that skip set on the **gitignored
/// `tasks.json` cache**, so any criterion that reached the cache but never the record was
/// silently dropped from the record forever, at exit 0 with a positive ack. The skip set is
/// now keyed on what the **committed record** names (the source of truth —
/// `design/team-ready-state.md`), and the mid-mint abort unwinds its own mints through the
/// same primitive a rejected record commit uses, so the divergence is not produced either.
///
/// Iterated over the **producers** of the divergence rather than over the one reported repro:
/// a rejected k-th record commit (T3's own, already unwound — pinned so it stays covered), an
/// engine-side mint abort (the reported producer), and a planted cache residue standing for an
/// unwind that could not complete or a process killed between the mint and its record commit.
/// Every arm asserts the same invariant: no exit-0 run leaves a criterion out of the record.
#[test]
fn add_from_spec_never_drops_a_criterion_from_the_record_at_any_divergence_producer() {
    for producer in DIVERGENCE_PRODUCERS {
        let repo = TempDir::new(&format!("divergence-{producer}"));
        let home = TempDir::new(&format!("home-divergence-{producer}"));
        let repo = repo.path();
        let home = home.path();
        init_repo(repo);
        write_compose_marker(repo);
        let spec_addr = format!("spec:{THREE_CRITERIA_SLUG}");

        // Each producer: bring the workbench to a state where it names a sub-task the record
        // does not (or would, before the fix), then let the operator repair and re-run.
        let (criteria, resumed): (Vec<&str>, std::process::Output) = match producer {
            "record-commit-rejection" => {
                commit_three_criteria_spec(repo);
                ok(
                    repo,
                    home,
                    &["milestone", "create", MILESTONE_TITLE],
                    "milestone create",
                );
                install_kth_record_rejecting_hook(repo, 2);
                let first = jigc(
                    repo,
                    home,
                    &["milestone", "add-from-spec", MILESTONE_ID, &spec_addr],
                    None,
                );
                assert!(
                    !first.status.success(),
                    "{producer}: the rejected record commit must exit non-zero",
                );
                // The operator's repair: clear the rejection, re-run.
                remove_hook(repo);
                let resumed = jigc(
                    repo,
                    home,
                    &["milestone", "add-from-spec", MILESTONE_ID, &spec_addr],
                    None,
                );
                (THREE_CRITERIA_IDS.to_vec(), resumed)
            }
            "mint-abort" => {
                commit_spec_body(repo, COLLIDING_CRITERIA_SPEC, "add the colliding spec");
                ok(
                    repo,
                    home,
                    &["milestone", "create", MILESTONE_TITLE],
                    "milestone create",
                );
                let first = jigc(
                    repo,
                    home,
                    &["milestone", "add-from-spec", MILESTONE_ID, &spec_addr],
                    None,
                );
                let first_out = format!(
                    "{}{}",
                    String::from_utf8_lossy(&first.stdout),
                    String::from_utf8_lossy(&first.stderr)
                );
                assert!(
                    !first.status.success(),
                    "{producer}: two criteria slugging alike must block; got:\n{first_out}",
                );
                assert!(
                    first_out.contains("milestone.sub-task-collision"),
                    "{producer}: the authoring collision stays the loud block; got:\n{first_out}",
                );
                // The abort unwinds its own mints — the workbench names exactly what the
                // record names (nothing), so the divergence is never produced.
                assert_eq!(
                    tasks_json_ids(repo),
                    Vec::<String>::new(),
                    "{producer}: the aborted mint leaves no id in the demoted cache",
                );
                assert_eq!(
                    all_sub_task_dirs(repo),
                    Vec::<String>::new(),
                    "{producer}: the aborted mint leaves no orphan sub-task area",
                );
                // The operator's repair: fix the colliding criterion, commit, re-run.
                commit_spec_body(
                    repo,
                    REPAIRED_CRITERIA_SPEC,
                    "repair the colliding criterion",
                );
                let resumed = jigc(
                    repo,
                    home,
                    &["milestone", "add-from-spec", MILESTONE_ID, &spec_addr],
                    None,
                );
                (REPAIRED_CRITERIA_IDS.to_vec(), resumed)
            }
            "planted-residue" => {
                commit_three_criteria_spec(repo);
                ok(
                    repo,
                    home,
                    &["milestone", "create", MILESTONE_TITLE],
                    "milestone create",
                );
                // An unwind that could not complete (or a process killed between the mint and
                // its record commit): the cache names a sub-task the record never got.
                fs::write(
                    task_list_path(repo),
                    format!(
                        "{{\n  \"tasks\": [\n    \"{}\"\n  ]\n}}\n",
                        THREE_CRITERIA_IDS[0]
                    ),
                )
                .expect("plant the cache residue");
                let resumed = jigc(
                    repo,
                    home,
                    &["milestone", "add-from-spec", MILESTONE_ID, &spec_addr],
                    None,
                );
                (THREE_CRITERIA_IDS.to_vec(), resumed)
            }
            other => panic!("unknown divergence producer `{other}`"),
        };

        let resumed_out = format!(
            "{}{}",
            String::from_utf8_lossy(&resumed.stdout),
            String::from_utf8_lossy(&resumed.stderr)
        );
        // The invariant — over every producer, on every exit-0 run.
        assert_no_criterion_lost(
            repo,
            home,
            &resumed,
            &criteria,
            &format!("{producer}, after the repair"),
        );
        // A producer whose residue the repair could not clear must stay LOUD: a skipped
        // criterion is never absorbed as "already seeded".
        if producer == "planted-residue" {
            assert!(
                !resumed.status.success(),
                "{producer}: a cache id the record does not name must block, never be absorbed \
                 as already-seeded; got:\n{resumed_out}",
            );
            assert!(
                !resumed_out.contains("already seeded"),
                "{producer}: the residue is not an already-seeded criterion — the record names \
                 nothing; got:\n{resumed_out}",
            );
        } else {
            assert!(
                resumed.status.success(),
                "{producer}: the repaired re-run must land every criterion; got:\n{resumed_out}",
            );
            assert_seeded_set(
                repo,
                home,
                &criteria,
                &format!("{producer}, after the repair"),
            );
            ok(repo, home, &["validate"], "jigc validate after the repair");
        }
    }
}
