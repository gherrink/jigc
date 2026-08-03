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
