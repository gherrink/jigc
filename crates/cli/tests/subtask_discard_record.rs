//! M49 Increment 2 / T3 — **a discarded sub-task stops being `active` in the record, and
//! stops being rebuildable** (`implementation/roadmap.md` → M49 Increment 2, bullet 4;
//! `design/team-ready-state.md` → The lifecycle / the per-op record-only-door transaction).
//!
//! Driven at `ddbd217`: `jigc task discard <sub-id>` removed the sub-task's working area,
//! exited 0, and left the committed record saying `- status: active` for it forever — the
//! team-ready record lying about work that was abandoned. And because
//! `engine::milestone::reseed_sub_task_areas` filtered on the **area's absence** alone,
//! never on the item's recorded `status`, the very next milestone door rebuilt the area the
//! discard had just removed: `jigc milestone list-tasks` — a *read* — resurrected it.
//!
//! So the fix is two halves in one commit, and this suite drives both through the **real
//! binary**:
//!
//!   1. **the record write** — `task discard` becomes the **fifth record-only door**,
//!      carrying the shipped discipline (capture the pre-image → commit the record alone
//!      through the transaction → advance the baseline), settling **that one item** to
//!      `discarded` while its siblings and the header stay byte-untouched;
//!   2. **the reseed skip** — a settled item is not rebuilt, over **every door that reaches
//!      the shared reseed site**, and that door set is *enumerated from its call sites in
//!      `crates/cli/src/milestone.rs`* rather than hand-listed, so a door added (or, at T4,
//!      removed) reddens [`DOOR_ARMS`]'s bijection instead of silently escaping the sweep.
//!
//! The omitting context is tested too (the context-scoped rule): an **ordinary** task —
//! no milestone names it — discards exactly as it always did, writing no record and landing
//! no commit.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-subtask-discard-record-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs
/// (never inheriting a harness `JIGC_PACK_DIR` — the compose-marker path requires it
/// ABSENT).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
}

/// Run `jigc <args>`, asserting exit 0, returning stdout.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(repo, home, args);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The milestone under test — one title, one id, across every arm.
const MILESTONE_TITLE: &str = "Cache rework";
const MILESTONE_ID: &str = "cache-rework";

/// The two sub-tasks: the one this suite discards, and the sibling that must survive
/// untouched on both axes (its record item, and its rebuildable working area).
const DISCARDED_INTENT: &str = "Warm the read cache";
const DISCARDED_SUB_ID: &str = "warm-the-read-cache";
const SIBLING_INTENT: &str = "Evict cold entries";
const SIBLING_SUB_ID: &str = "evict-cold-entries";

/// The committed record's repo-relative path (docs-root-nested by the dev `docs-root` knob
/// the `[dev ▸ methodology]` composition resolves).
const RECORD_SPEC: &str = "docs/milestone-records/cache-rework.md";

/// The record's location — the prefix `git status --porcelain` must never name after a
/// rejected door, and the prefix the rejecting hook keys on.
const RECORD_LOCATION: &str = "docs/milestone-records/";

/// A sub-task's working area under the gitignored workbench.
fn task_area(repo: &Path, sub_id: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub_id)
}

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`) and
/// write the `[dev ▸ methodology]` compose marker that resolves the `milestone-record`
/// doctype (dev-only ships no record, so the whole record axis would be inert).
fn base_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    (repo, home)
}

/// A milestone carrying both sub-tasks, each recorded and each with a working area.
fn milestone_with_two_sub_tasks(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &["milestone", "create", MILESTONE_TITLE],
        "milestone create",
    );
    ok(
        repo,
        home,
        &["milestone", "add-task", MILESTONE_ID, DISCARDED_INTENT],
        "milestone add-task (the sub-task to discard)",
    );
    ok(
        repo,
        home,
        &["milestone", "add-task", MILESTONE_ID, SIBLING_INTENT],
        "milestone add-task (the sibling)",
    );
}

/// The rejecting hook's own stderr — asserted **verbatim** in the door's output (the hook
/// channel is never wrapped or edited; `design/finalize.md` → the M40 refinement 3).
const HOOK_STDERR: &str = "record commit rejected by the test hook";

/// Install a `pre-commit` hook rejecting **any** commit that stages a milestone-record path.
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

/// Remove the rejecting hook — the "fix the hook, re-run, it succeeds" half.
fn remove_hook(repo: &Path) {
    let _ = fs::remove_file(repo.join(".git").join("hooks").join("pre-commit"));
}

/// The record read back through the **pinned** read contract
/// (`jigc doc show milestone-record:<id> --format json`) — never off the raw bytes, so the
/// assertions bind to what an agent actually reads.
fn shown_record(repo: &Path, home: &Path) -> serde_json::Value {
    let stdout = ok(
        repo,
        home,
        &[
            "doc",
            "show",
            &format!("milestone-record:{MILESTONE_ID}"),
            "--format",
            "json",
        ],
        "doc show milestone-record --format json",
    );
    serde_json::from_str(&stdout).expect("the pinned `--format json` read parses")
}

/// The `(task-id, status)` pairs the shown record carries, in document order.
fn shown_statuses(record: &serde_json::Value) -> Vec<(String, String)> {
    record["sections"]["tasks"]
        .as_array()
        .expect("the record's `tasks` section is an array of items")
        .iter()
        .map(|item| {
            (
                item["task-id"]
                    .as_str()
                    .expect("item `task-id`")
                    .to_string(),
                item["status"].as_str().expect("item `status`").to_string(),
            )
        })
        .collect()
}

/// **Arm 1 — the record write.** A sub-task discard settles that one item to `discarded`,
/// leaves its sibling and the header untouched, and lands exactly one record-only commit.
#[test]
fn discarding_a_sub_task_settles_its_record_item_in_one_record_only_commit() {
    let (repo, home) = base_repo("settle");
    let (repo, home) = (repo.path(), home.path());
    milestone_with_two_sub_tasks(repo, home);

    let before = shown_record(repo, home);
    assert_eq!(
        shown_statuses(&before),
        vec![
            (DISCARDED_SUB_ID.to_string(), "active".to_string()),
            (SIBLING_SUB_ID.to_string(), "active".to_string()),
        ],
        "both sub-tasks start `active`",
    );
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    ok(
        repo,
        home,
        &["task", "discard", DISCARDED_SUB_ID],
        "task discard <sub-id>",
    );

    // (a) the item settled, the sibling and the header did not.
    let after = shown_record(repo, home);
    assert_eq!(
        shown_statuses(&after),
        vec![
            (DISCARDED_SUB_ID.to_string(), "discarded".to_string()),
            (SIBLING_SUB_ID.to_string(), "active".to_string()),
        ],
        "the discarded sub-task settles; its sibling stays `active`",
    );
    assert_eq!(
        after["fields"]["status"], "active",
        "the milestone itself is still in flight — only one sub-task was abandoned",
    );
    assert_eq!(
        before["fields"]["base"], after["fields"]["base"],
        "the discard touches no other leaf of the record",
    );

    // (b) exactly one commit landed, carrying the record and nothing else.
    let range = format!("{}..HEAD", head_before.trim());
    let landed: Vec<String> = git(repo, &["rev-list", &range])
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(
        landed.len(),
        1,
        "a sub-task discard lands exactly one record-only commit; got {landed:?}",
    );
    assert_eq!(
        git(repo, &["show", "--name-only", "--format=", "HEAD"]).trim(),
        RECORD_SPEC,
        "the commit carries the record alone (the path-scoped record-only discipline)",
    );

    // (c) the working area is gone, the sibling's is not.
    assert!(
        !task_area(repo, DISCARDED_SUB_ID).exists(),
        "the discarded sub-task's working area is removed",
    );
    assert!(
        task_area(repo, SIBLING_SUB_ID).is_dir(),
        "the sibling's working area is untouched",
    );
}

/// **Arm 2 — the rejection.** A rejecting `pre-commit` hook leaves the record and the index
/// at their pre-image, leaves the task itself intact, and frames the re-run; with the hook
/// removed the identical argv succeeds.
#[test]
fn a_rejected_sub_task_discard_restores_the_pre_image_and_frames_its_rerun() {
    let (repo, home) = base_repo("rejected");
    let (repo, home) = (repo.path(), home.path());
    milestone_with_two_sub_tasks(repo, home);

    let record_path = repo.join(RECORD_SPEC);
    let pre_bytes = fs::read(&record_path).expect("read the record");
    let pre_index = git(repo, &["ls-files", "--stage", "--", RECORD_SPEC]);
    let head_before = git(repo, &["rev-parse", "HEAD"]);
    install_record_rejecting_hook(repo);

    let rejected = jigc(repo, home, &["task", "discard", DISCARDED_SUB_ID]);
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();
    assert!(
        !rejected.status.success(),
        "a rejected record commit must exit non-zero; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(HOOK_STDERR),
        "the hook's own stderr rides verbatim; got:\n{stderr}",
    );
    assert!(
        stderr.contains("`jigc task discard warm-the-read-cache`"),
        "the frame carries this door's own copy-runnable re-run; got:\n{stderr}",
    );
    assert!(
        stderr.contains("nothing was committed"),
        "the frame states what survived; got:\n{stderr}",
    );

    // The pre-image, both axes, plus the workbench the frame claims is intact.
    assert_eq!(
        fs::read(&record_path).expect("read the record"),
        pre_bytes,
        "a rejected commit restores the record's bytes",
    );
    assert_eq!(
        git(repo, &["ls-files", "--stage", "--", RECORD_SPEC]),
        pre_index,
        "a rejected commit restores the record's index entry",
    );
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "nothing landed",
    );
    assert!(
        git(repo, &["status", "--porcelain"])
            .lines()
            .all(|line| !line.contains(RECORD_LOCATION)),
        "no record residue is left staged or dirty",
    );
    assert!(
        task_area(repo, DISCARDED_SUB_ID).is_dir(),
        "the task the frame calls intact still has its working area",
    );

    // The recovery: fix the hook, re-run the printed argv, it succeeds.
    remove_hook(repo);
    ok(
        repo,
        home,
        &["task", "discard", DISCARDED_SUB_ID],
        "the re-run after fixing the hook",
    );
    assert_eq!(
        shown_statuses(&shown_record(repo, home)),
        vec![
            (DISCARDED_SUB_ID.to_string(), "discarded".to_string()),
            (SIBLING_SUB_ID.to_string(), "active".to_string()),
        ],
        "the re-run lands what the rejected run would have",
    );
}

/// **Arm 3 — the omitting context.** An ordinary task (no milestone names it) discards
/// exactly as before: no record write, no commit.
#[test]
fn an_ordinary_task_discard_writes_no_record_and_lands_no_commit() {
    let (repo, home) = base_repo("ordinary");
    let (repo, home) = (repo.path(), home.path());
    // A milestone exists — so the door's resolution is exercised over a record that simply
    // does not name this task, not over a repo with no records at all.
    milestone_with_two_sub_tasks(repo, home);

    let minted = ok(
        repo,
        home,
        &["start", "--workflow", "quick-fix", "Tidy the cache module"],
        "jigc start --workflow quick-fix <intent>",
    );
    let ordinary = minted
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .expect("`jigc start` names the minted task id")
        .trim()
        .to_string();
    let ordinary = ordinary.as_str();
    assert!(
        task_area(repo, ordinary).is_dir(),
        "`jigc start` minted the ordinary task's area",
    );

    let head_before = git(repo, &["rev-parse", "HEAD"]);
    let record_before = fs::read(repo.join(RECORD_SPEC)).expect("read the record");

    ok(
        repo,
        home,
        &["task", "discard", ordinary, "--force"],
        "task discard <ordinary-id> --force",
    );

    assert!(
        !task_area(repo, ordinary).exists(),
        "the ordinary task's area is removed",
    );
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "an ordinary discard lands no commit",
    );
    assert_eq!(
        fs::read(repo.join(RECORD_SPEC)).expect("read the record"),
        record_before,
        "an ordinary discard writes no record",
    );
    assert_eq!(
        shown_statuses(&shown_record(repo, home)),
        vec![
            (DISCARDED_SUB_ID.to_string(), "active".to_string()),
            (SIBLING_SUB_ID.to_string(), "active".to_string()),
        ],
        "no sub-task item moved",
    );
}

/// One door of the reseed axis: the enclosing `run_*` function in
/// `crates/cli/src/milestone.rs` that reaches the shared reseed site, paired with the argv
/// that drives it through the real binary.
struct DoorArm {
    /// The enclosing function name the call-site enumeration reports.
    func: &'static str,
    /// The argv (after `jigc`) that drives this door.
    argv: &'static [&'static str],
}

/// **The arms of the reseed axis.** The *axis* is derived from the code
/// ([`doors_reaching_the_reseed_site`]); this table is only how each derived door is
/// *driven*, so the two are checked to biject: a door added to (or removed from) the reseed
/// site reddens the bijection rather than silently escaping the sweep.
///
/// `run_list_tasks` **left the axis** at M49 Increment 2, T4 — the one milestone verb
/// classified `VerbKind::Read` no longer re-seeds anything at all, so it has no rebuild to
/// skip. That it rebuilds nothing over exactly this state is asserted where the whole `Read`
/// set is swept (`crates/cli/tests/read_verb_acts_nothing.rs`).
const DOOR_ARMS: &[DoorArm] = &[
    DoorArm {
        func: "run_add_task",
        argv: &["milestone", "add-task", MILESTONE_ID, "Prefetch the index"],
    },
    DoorArm {
        func: "run_add_from_spec",
        argv: &[
            "milestone",
            "add-from-spec",
            MILESTONE_ID,
            "spec:rate-limit",
        ],
    },
    DoorArm {
        func: "run_provision",
        argv: &["milestone", "provision", MILESTONE_ID],
    },
    DoorArm {
        func: "run_execute",
        argv: &["milestone", "execute", MILESTONE_ID],
    },
    DoorArm {
        func: "run_join",
        argv: &["milestone", "join", MILESTONE_ID],
    },
    DoorArm {
        func: "run_milestone_finalize",
        argv: &["milestone", "finalize", MILESTONE_ID],
    },
    DoorArm {
        func: "run_discard",
        argv: &["milestone", "discard", MILESTONE_ID, "--force"],
    },
];

/// **The axis, read off the code**: every function in `crates/cli/src/milestone.rs` that
/// calls the shared `reseed_cache(` site. Enumerated from the source rather than hand-listed
/// — the sweep's subject is *the doors that reach the reseed*, and only the code knows that
/// set.
fn doors_reaching_the_reseed_site() -> BTreeSet<String> {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("milestone.rs"),
    )
    .expect("read crates/cli/src/milestone.rs");
    let mut current = String::new();
    let mut doors = BTreeSet::new();
    for line in source.lines() {
        if let Some(rest) = line.strip_prefix("fn ") {
            current = rest
                .split(['(', '<'])
                .next()
                .unwrap_or_default()
                .trim()
                .to_string();
            continue;
        }
        let code = line.trim_start();
        if code.starts_with("//") {
            continue;
        }
        if code.contains("reseed_cache(") && !current.is_empty() {
            doors.insert(current.clone());
        }
    }
    doors
}

/// A committed 2-criteria spec — `add-from-spec`'s seed substrate.
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

/// **Arm 4 — the reseed skip, over the whole door axis.** For every door that reaches the
/// shared reseed site: a sub-task the record settles to `discarded` is **never** rebuilt,
/// while a sibling the record still calls `active` **is**.
///
/// Each door runs under the record-rejecting hook, which is what makes the assertion uniform
/// across the axis: the reseed runs at the top of every door, so the hook stops the two
/// teardown doors (`milestone discard`, `milestone finalize`) *after* the reseed and *before*
/// they would remove the sibling's area for reasons of their own. The door's own exit status
/// is deliberately not asserted here — this arm's subject is the reseed, not the door.
#[test]
fn no_door_reaching_the_reseed_site_rebuilds_a_discarded_sub_task() {
    let derived = doors_reaching_the_reseed_site();
    let armed: BTreeSet<String> = DOOR_ARMS.iter().map(|a| a.func.to_string()).collect();
    assert_eq!(
        derived, armed,
        "the axis is the set of functions calling `reseed_cache(` in \
         crates/cli/src/milestone.rs — a door added there owes an arm in `DOOR_ARMS`, and a \
         door that no longer reaches the reseed site owes its arm's removal",
    );

    for arm in DOOR_ARMS {
        let (repo, home) = base_repo(&format!("axis-{}", arm.func.replace('_', "-")));
        let (repo, home) = (repo.path(), home.path());
        let specs = repo.join("docs").join("specs");
        fs::create_dir_all(&specs).expect("mk docs/specs/");
        fs::write(specs.join("rate-limit.md"), TWO_CRITERIA_SPEC).expect("write the spec");
        git(repo, &["add", "."]);
        git(repo, &["commit", "-q", "-m", "add spec"]);
        milestone_with_two_sub_tasks(repo, home);
        ok(
            repo,
            home,
            &["task", "discard", DISCARDED_SUB_ID],
            "task discard <sub-id>",
        );
        // The sibling's area goes too — a fresh clone's state, so "rebuilt" is observable.
        fs::remove_dir_all(task_area(repo, SIBLING_SUB_ID)).expect("remove the sibling's area");
        install_record_rejecting_hook(repo);

        let out = jigc(repo, home, arm.argv);
        let context = format!(
            "[{}] `jigc {}`\nstdout:\n{}\nstderr:\n{}",
            arm.func,
            arm.argv.join(" "),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !task_area(repo, DISCARDED_SUB_ID).exists(),
            "a discarded sub-task's working area must never be rebuilt\n{context}",
        );
        assert!(
            task_area(repo, SIBLING_SUB_ID).is_dir(),
            "a sibling the record still calls `active` must still be rebuilt\n{context}",
        );
    }
}

/// **The axis the door's silent `Ok(None)` swallowed** — the two ways a committed record
/// refuses to answer *"does any record name this task?"*, i.e. the two `continue` arms
/// `crate::milestone::recording_milestone` walked past: the bytes do not **read** (not UTF-8)
/// and the bytes do not **conform** (a leading BOM the `milestone-record` schema rejects).
///
/// Each is a byte prefix spliced onto the record jigc itself wrote, so the corruption is an
/// out-of-band edit to a machine-maintained doc — the state every sibling milestone door
/// already answers with `reconciliation.conflict-block`.
const UNREADABLE_RECORD_KINDS: &[(&str, &[u8])] = &[
    ("bytes that are not UTF-8", b"\xff\xfe"),
    ("a non-conforming leading BOM", b"\xef\xbb\xbf"),
];

/// The finding every milestone door raises over an out-of-band edit to the machine-maintained
/// record — asserted by code, so the arm binds to the shipped identity and not to prose.
const RECORD_CONFLICT_CODE: &str = "reconciliation.conflict-block";

/// Splice `prefix` onto the front of the committed record — one out-of-band edit, no commit.
fn corrupt_record(repo: &Path, prefix: &[u8]) {
    let path = repo.join(RECORD_SPEC);
    let mut bytes = prefix.to_vec();
    bytes.extend_from_slice(&fs::read(&path).expect("read the record"));
    fs::write(&path, bytes).expect("write the corrupted record");
}

/// Mint an ordinary (milestone-less) task and return its id.
fn ordinary_task(repo: &Path, home: &Path) -> String {
    let minted = ok(
        repo,
        home,
        &["start", "--workflow", "quick-fix", "Tidy the cache module"],
        "jigc start --workflow quick-fix <intent>",
    );
    minted
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .expect("`jigc start` names the minted task id")
        .trim()
        .to_string()
}

/// **Arm 4 — an unreadable record is never read as "no record names this task".** Driven at
/// `3778161`, where `recording_milestone` `continue`d past a record it could not read or
/// parse, returned `Ok(None)`, and `task discard` deleted the sub-task's working area **at
/// exit 0** while the committed record went on calling it `status: active` — the exact lie
/// this door exists to close, surviving on the door itself, and silently, while every sibling
/// milestone door blocked over the same bytes.
///
/// So the refusal is asserted over the whole cause axis ([`UNREADABLE_RECORD_KINDS`]): the
/// door exits non-zero with the shipped `reconciliation.conflict-block` and its route, the
/// record's bytes are untouched, nothing is committed, and — the half that makes the
/// rejection frame's state-truth clause true — the working area is still there.
#[test]
fn an_unreadable_record_refuses_the_sub_task_discard_instead_of_deleting_its_area() {
    for (what, prefix) in UNREADABLE_RECORD_KINDS {
        let (repo, home) = base_repo(&format!("unreadable-{}", prefix.len() + prefix[0] as usize));
        let (repo, home) = (repo.path(), home.path());
        milestone_with_two_sub_tasks(repo, home);
        corrupt_record(repo, prefix);

        let head_before = git(repo, &["rev-parse", "HEAD"]);
        let record_before = fs::read(repo.join(RECORD_SPEC)).expect("read the record");

        let out = jigc(repo, home, &["task", "discard", DISCARDED_SUB_ID]);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        let context = format!(
            "[{what}] stdout:\n{}\nstderr:\n{stderr}",
            String::from_utf8_lossy(&out.stdout),
        );

        assert!(
            !out.status.success(),
            "a record that cannot be read cannot clear a sub-task discard\n{context}",
        );
        assert!(
            stderr.contains(RECORD_CONFLICT_CODE),
            "the refusal must be the shipped `{RECORD_CONFLICT_CODE}` every sibling door \
             raises over the same bytes\n{context}",
        );
        assert!(
            stderr.contains("route:"),
            "a blocking finding carries a route\n{context}",
        );
        assert!(
            task_area(repo, DISCARDED_SUB_ID).is_dir(),
            "the working area must survive a refused discard — the rejection frame says it \
             does\n{context}",
        );
        assert_eq!(
            fs::read(repo.join(RECORD_SPEC)).expect("read the record"),
            record_before,
            "a refused discard never touches the record's bytes\n{context}",
        );
        assert_eq!(
            git(repo, &["rev-parse", "HEAD"]),
            head_before,
            "a refused discard lands no commit\n{context}",
        );
    }
}

/// **Arm 5 — the refusal is fail-closed, not owner-keyed.** An *ordinary* task's discard is
/// refused too while a record cannot be read, because "no record names this task" is exactly
/// the answer an unreadable record makes unavailable: the door cannot tell an ordinary task
/// from a sub-task of the record it could not parse, and guessing the benign branch is what
/// deleted the area at exit 0 in the first place. The route restores the record, after which
/// the ordinary discard proceeds — which this arm drives to the end.
#[test]
fn an_unreadable_record_fails_closed_for_an_ordinary_task_and_clears_once_restored() {
    let (repo, home) = base_repo("unreadable-ordinary");
    let (repo, home) = (repo.path(), home.path());
    milestone_with_two_sub_tasks(repo, home);
    let ordinary = ordinary_task(repo, home);
    let ordinary = ordinary.as_str();

    let pristine = fs::read(repo.join(RECORD_SPEC)).expect("read the record");
    corrupt_record(repo, UNREADABLE_RECORD_KINDS[1].1);

    let out = jigc(repo, home, &["task", "discard", ordinary, "--force"]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        !out.status.success() && stderr.contains(RECORD_CONFLICT_CODE),
        "an unreadable record fails closed at this door; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        task_area(repo, ordinary).is_dir(),
        "the ordinary task's area survives the refusal",
    );

    // The route: restore what jigc last wrote, then re-run.
    fs::write(repo.join(RECORD_SPEC), &pristine).expect("restore the record");
    ok(
        repo,
        home,
        &["task", "discard", ordinary, "--force"],
        "task discard <ordinary-id> --force once the record reads again",
    );
    assert!(
        !task_area(repo, ordinary).exists(),
        "the restored record clears the way and the ordinary discard proceeds",
    );
}

// ---------------------------------------------------------------------------------------
// Arm 5 — the ENUMERATION axis: no operating door acts on a settled sub-task.
// ---------------------------------------------------------------------------------------

/// A sub-task's provisioned fan-out worktree.
fn worktree(repo: &Path, sub_id: &str) -> PathBuf {
    repo.join(".jigc").join("worktrees").join(sub_id)
}

/// Settle the suite's discarded sub-task through the real door.
fn discard_the_sub_task(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &["task", "discard", DISCARDED_SUB_ID],
        "task discard <sub-id>",
    );
}

/// Every `` Spawn: `…` `` span of a composed milestone-execution view, **verbatim** — each is
/// a shell line (`cd <worktree> && jigc workflow <W> --task <id>`), run as emitted below
/// (the `milestone_record_fresh_clone` suite's shape: an agent-facing emitted artifact is
/// proven by running its bytes, never by reconstructing them).
fn spawn_spans(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("Spawn: `"))
        .filter_map(|rest| rest.strip_suffix('`'))
        .map(str::to_owned)
        .collect()
}

/// Install a `jigc` shim on a throwaway `PATH` entry, so an emitted `Spawn:` span — which
/// names the bare command `jigc`, as an agent would run it — resolves to the binary under
/// test.
fn install_jigc_shim(dir: &Path) -> PathBuf {
    let bin = dir.join("shim-bin");
    fs::create_dir_all(&bin).expect("mk the shim bin dir");
    let shim = bin.join("jigc");
    fs::write(
        &shim,
        format!("#!/bin/sh\nexec {:?} \"$@\"\n", env!("CARGO_BIN_EXE_jigc")),
    )
    .expect("write the jigc shim");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&shim).expect("shim metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&shim, perms).expect("chmod the shim");
    }
    bin
}

/// Run an emitted shell span verbatim through `sh -c`, with the `jigc` shim first on `PATH`.
fn run_span(cwd: &Path, home: &Path, shim_bin: &Path, span: &str) -> std::process::Output {
    let path = match std::env::var("PATH") {
        Ok(rest) => format!("{}:{rest}", shim_bin.display()),
        Err(_) => shim_bin.display().to_string(),
    };
    Command::new("sh")
        .arg("-c")
        .arg(span)
        .current_dir(cwd)
        .env("HOME", home)
        .env("PATH", path)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the emitted span")
}

/// One door of the **enumeration** axis: the enclosing `run_*` function in
/// `crates/cli/src/milestone.rs` that reads the milestone's sub-task list, paired with what
/// that door owes over an item the record has settled. Each `check` drives its own door
/// through the real binary from the shared two-sub-task starting state, and does its own
/// `task discard` at the moment its door needs it.
struct EnumArm {
    /// The enclosing function name the call-site enumeration reports.
    func: &'static str,
    /// The door's obligation, driven and asserted through the binary.
    check: fn(&Path, &Path),
}

/// **The arms of the enumeration axis.** The *axis* is derived from the code
/// ([`doors_reading_the_task_list`]); this table is only how each derived door is *driven*,
/// so the two are checked to biject: a door added to (or removed from) the task-list read
/// site reddens the bijection rather than silently escaping the sweep.
///
/// Five doors **operate** on the sub-task set and owe the settled item's exclusion. The
/// sixth, `run_discard`, is the milestone's **teardown** and owes the opposite — a sub-task
/// discard leaves the provisioned worktree standing (verified in its arm), so the abandon
/// path has to keep covering the settled member or the worktree is orphaned.
const ENUM_ARMS: &[EnumArm] = &[
    EnumArm {
        func: "run_list_tasks",
        check: list_tasks_omits_the_settled_sub_task,
    },
    EnumArm {
        func: "run_provision",
        check: provision_omits_the_settled_sub_task,
    },
    EnumArm {
        func: "run_execute",
        check: execute_omits_the_settled_sub_task,
    },
    EnumArm {
        func: "run_join",
        check: join_omits_the_settled_sub_task,
    },
    EnumArm {
        func: "run_milestone_finalize",
        check: finalize_omits_the_settled_sub_task,
    },
    EnumArm {
        func: "run_discard",
        check: discard_still_tears_down_the_settled_sub_task,
    },
];

/// **The axis, read off the code**: every function in `crates/cli/src/milestone.rs` that
/// reads the milestone's persisted sub-task list (`read_task_list(`). Enumerated from the
/// source rather than hand-listed — the sweep's subject is *the doors that enumerate the
/// sub-tasks*, and only the code knows that set.
fn doors_reading_the_task_list() -> BTreeSet<String> {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("milestone.rs"),
    )
    .expect("read crates/cli/src/milestone.rs");
    let mut current = String::new();
    let mut doors = BTreeSet::new();
    for line in source.lines() {
        if let Some(rest) = line.strip_prefix("fn ") {
            current = rest
                .split(['(', '<'])
                .next()
                .unwrap_or_default()
                .trim()
                .to_string();
            continue;
        }
        let code = line.trim_start();
        if code.starts_with("//") {
            continue;
        }
        if code.contains("read_task_list(") && !current.is_empty() {
            doors.insert(current.clone());
        }
    }
    doors
}

/// `jigc milestone list-tasks` answers with the **live** sub-tasks: a settled item is neither
/// named nor counted, so the route every dead end prints cannot hand the operator back the
/// sub-task that is over.
fn list_tasks_omits_the_settled_sub_task(repo: &Path, home: &Path) {
    discard_the_sub_task(repo, home);
    let out = ok(
        repo,
        home,
        &["milestone", "list-tasks", MILESTONE_ID],
        "milestone list-tasks",
    );
    assert!(
        out.contains(&format!(
            "milestone:{MILESTONE_ID} tasks (1): {SIBLING_SUB_ID}"
        )),
        "the live sub-task is named and counted; got:\n{out}",
    );
    assert!(
        !out.contains(DISCARDED_SUB_ID),
        "a settled sub-task is not a live sub-task; got:\n{out}",
    );
}

/// `jigc milestone provision` lays down a worktree per **live** sub-task — a settled one owns
/// no work, and its area is deliberately never rebuilt, so a worktree for it is a checkout
/// nothing can ever run in.
fn provision_omits_the_settled_sub_task(repo: &Path, home: &Path) {
    discard_the_sub_task(repo, home);
    let out = ok(
        repo,
        home,
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    assert!(
        out.contains("provisioned 1 worktree(s)") && !out.contains(DISCARDED_SUB_ID),
        "provision names and counts the live sub-tasks only; got:\n{out}",
    );
    assert!(
        !worktree(repo, DISCARDED_SUB_ID).exists(),
        "a settled sub-task gets no worktree",
    );
    assert!(
        worktree(repo, SIBLING_SUB_ID).is_dir(),
        "the live sibling still gets its worktree",
    );
}

/// `jigc milestone execute` emits one `Spawn:` line per **live** sub-task — and the emitted
/// bytes are proven by running them, not by reading them: the settled sub-task's line is
/// absent (it would dead-end on *"no task"* and route straight back at `list-tasks`, which
/// names it again — the loop the reseed exists to close), and the line that IS emitted runs
/// verbatim at exit 0.
fn execute_omits_the_settled_sub_task(repo: &Path, home: &Path) {
    discard_the_sub_task(repo, home);
    ok(
        repo,
        home,
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    let view = ok(
        repo,
        home,
        &["milestone", "execute", MILESTONE_ID],
        "milestone execute",
    );
    let spans = spawn_spans(&view);
    assert_eq!(
        spans,
        vec![format!(
            "cd .jigc/worktrees/{SIBLING_SUB_ID} && jigc workflow sub-task --task {SIBLING_SUB_ID}"
        )],
        "exactly one `Spawn:` line, for the live sub-task; the composed view was:\n{view}",
    );
    let shim = install_jigc_shim(home);
    let out = run_span(repo, home, &shim, &spans[0]);
    assert!(
        out.status.success(),
        "the emitted `Spawn:` line must run verbatim; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// `jigc milestone join` reports over the **live** sub-tasks — a settled one is not a
/// contributor that merely staged nothing.
fn join_omits_the_settled_sub_task(repo: &Path, home: &Path) {
    discard_the_sub_task(repo, home);
    let out = ok(
        repo,
        home,
        &["milestone", "join", MILESTONE_ID],
        "milestone join",
    );
    assert!(
        out.contains(SIBLING_SUB_ID),
        "the live sub-task is still named; got:\n{out}",
    );
    assert!(
        !out.contains(DISCARDED_SUB_ID),
        "a settled sub-task is not a doc-less contributor; got:\n{out}",
    );
}

/// `jigc milestone finalize` lands the boundary over the **live** sub-tasks: the synthesized
/// message credits only what landed, the manifest names only live members, and — the sharpest
/// half — the record's settled item is **not** flipped back to `joined`, which would make the
/// committed record lie about abandoned work all over again.
fn finalize_omits_the_settled_sub_task(repo: &Path, home: &Path) {
    discard_the_sub_task(repo, home);
    ok(
        repo,
        home,
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    let wt = worktree(repo, SIBLING_SUB_ID);
    fs::write(wt.join("feature.txt"), "work\n").expect("write the sub-task's code");
    git(&wt, &["add", "feature.txt"]);

    let out = ok(
        repo,
        home,
        &["milestone", "finalize", MILESTONE_ID],
        "milestone finalize",
    );
    assert!(
        !out.contains(DISCARDED_SUB_ID),
        "the landing manifest never credits a settled sub-task; got:\n{out}",
    );
    let message = git(repo, &["log", "-1", "--format=%B"]);
    assert!(
        message.contains("(1 sub-task)") && message.contains(&format!("- {SIBLING_SUB_ID}")),
        "the synthesized message lists the sub-tasks that landed; got:\n{message}",
    );
    assert!(
        !message.contains(DISCARDED_SUB_ID),
        "the synthesized message never lists a settled sub-task; got:\n{message}",
    );
    assert_eq!(
        shown_statuses(&shown_record(repo, home)),
        vec![
            (DISCARDED_SUB_ID.to_string(), "discarded".to_string()),
            (SIBLING_SUB_ID.to_string(), "joined".to_string()),
        ],
        "the join flip settles the live sub-task and leaves the settled one settled",
    );
}

/// `jigc milestone discard` is the **teardown**, and owes the opposite obligation: a sub-task
/// discard leaves its provisioned worktree standing (asserted here), so the abandon path has
/// to keep covering the settled member — excluding it would orphan a registered worktree the
/// milestone's own teardown is the last door able to remove.
fn discard_still_tears_down_the_settled_sub_task(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    discard_the_sub_task(repo, home);
    assert!(
        worktree(repo, DISCARDED_SUB_ID).is_dir(),
        "a sub-task discard removes the working area, not the provisioned worktree — which is \
         why the teardown below must still cover it",
    );
    ok(
        repo,
        home,
        &["milestone", "discard", MILESTONE_ID, "--force"],
        "milestone discard --force",
    );
    assert!(
        !worktree(repo, DISCARDED_SUB_ID).exists(),
        "the abandon path tears down the settled member's worktree too",
    );
    assert!(
        !worktree(repo, SIBLING_SUB_ID).exists(),
        "and its live sibling's",
    );
}

/// **Arm 5 — the enumeration skip, over the whole door axis.** For every door that reads the
/// milestone's sub-task list: an item the record settles to `discarded` is not enumerated as
/// a live sub-task — it gets no worktree, no `Spawn:` line, no place in the count, no credit
/// in the landing manifest and no flip back to `joined` — while the teardown door still
/// covers it.
///
/// The axis is derived from the call sites rather than hand-listed, so a new door reading the
/// task list reddens the bijection instead of silently escaping the sweep.
#[test]
fn no_door_reading_the_task_list_enumerates_a_settled_sub_task() {
    let derived = doors_reading_the_task_list();
    let armed: BTreeSet<String> = ENUM_ARMS.iter().map(|a| a.func.to_string()).collect();
    assert_eq!(
        derived, armed,
        "the axis is the set of functions calling `read_task_list(` in \
         crates/cli/src/milestone.rs — a door added there owes an arm in `ENUM_ARMS`, and a \
         door that no longer reads the task list owes its arm's removal",
    );

    for arm in ENUM_ARMS {
        let (repo, home) = base_repo(&format!("live-{}", arm.func.replace('_', "-")));
        let (repo, home) = (repo.path(), home.path());
        milestone_with_two_sub_tasks(repo, home);
        (arm.check)(repo, home);
    }
}
