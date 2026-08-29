//! M49 Increment 2 · T2 — a sub-task's `--workflow <id>` is checked against the loaded
//! packs **before anything mints**, at BOTH doors that mint a sub-task under one
//! (`implementation/roadmap.md` → M49 Increment 2; `completions/artifacts/M49/settle-record.md`
//! → T0-4; the `read_workflow` mint-after-validate discipline stated at
//! `crates/cli/src/migrate.rs:141`).
//!
//! **The defect, driven at `ddbd217`:** `jigc milestone add-task <m> "<intent>" --workflow
//! no-such-workflow` exits **0** — it writes the bogus id into the sub-task's area and commits
//! a record naming the sub-task `status: active`. The sub-task is then permanently unreachable:
//! `jigc workflow no-such-workflow --task <sub>` blocks on the unknown workflow, `jigc workflow
//! <real> --task <sub>` blocks on the W-equality guard, and no verb rewrites a recorded
//! workflow — so `jigc task discard` is the only exit. A door reported success for a state it
//! left broken.
//!
//! **The axis** ([`SUB_TASK_DOORS`]) is *every door that mints a milestone sub-task under a
//! caller-supplied `--workflow`*, taken from the verb surface rather than the reported repro:
//! `MilestoneCommand`'s two `workflow: String` arms, both defaulting to `sub-task` and both
//! reaching the same mint. The neighbouring `--workflow` doors that mint no *sub*-task
//! (`jigc start --workflow`, `jigc workflow <id>`) already reject an unknown id, and this
//! suite asserts that rather than leaving the axis boundary implied.
//!
//! Every arm drives the REAL binary against a throwaway `[dev ▸ methodology]` repo, so the
//! committed record — the thing the old exit 0 lied to — is real.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The **axis**: each door's `jigc milestone …` argv, *without* the `--workflow` flag the
/// arms append. Both rows mint a sub-task under a caller-supplied workflow.
const SUB_TASK_DOORS: &[&[&str]] = &[
    &["add-task", "rate-limit", "Bound the burst"],
    &["add-from-spec", "rate-limit", "spec:rate-limit"],
];

/// Workflow ids the composed `[dev ▸ methodology]` pack really provides — the members the
/// rejection must **name**, chosen so none of them is a substring of the message's own prose
/// (a bare "sub-task" would match the sentence, not the set).
const NAMED_MEMBERS: &[&str] = &["single-task", "migrate-adr", "planning"];

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-workflow-membership-{tag}-{}-{:?}",
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// A committed 2-criteria spec — `add-from-spec`'s seed substrate.
const TWO_CRITERIA_SPEC: &str = "\
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

/// A `[dev ▸ methodology]` repo with the spec committed and `milestone:rate-limit` open —
/// the shared substrate every arm starts from.
fn open_milestone(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    let root = repo.path();

    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(root.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        root.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    let specs = root.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join("rate-limit.md"), TWO_CRITERIA_SPEC).expect("write spec");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);

    let created = run(root, home.path(), &["milestone", "create", "Rate limit"]);
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
    (repo, home)
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// The committed record path under docs-root for milestone `rate-limit`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("rate-limit.md")
}

/// The sub-task ids the working area carries (absent `tasks/` reads as the empty set).
fn task_areas(repo: &Path) -> BTreeSet<String> {
    let dir = repo.join(".jigc").join("tasks");
    let Ok(entries) = fs::read_dir(&dir) else {
        return BTreeSet::new();
    };
    entries
        .map(|e| e.expect("read task dir entry").file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .collect()
}

/// The whole committed history, as one string — `git log` gaining no commit is asserted on
/// the log itself, not on a count that a rewrite could keep equal.
fn commit_log(repo: &Path) -> String {
    git(repo, &["log", "--format=%H %s"])
}

/// A bogus `--workflow` is refused at EVERY sub-task door, before anything mints: exit
/// non-zero, a routed rejection naming the provided set, no task area, a byte-identical
/// record, and no new commit.
#[test]
fn a_bogus_workflow_is_refused_at_every_sub_task_door() {
    for (i, door) in SUB_TASK_DOORS.iter().enumerate() {
        let verb = door[0];
        let (repo, home) = open_milestone(&format!("bogus-{i}"));
        let root = repo.path();

        let areas_before = task_areas(root);
        let record_before = fs::read(record_path(root)).expect("the record exists after create");
        let log_before = commit_log(root);

        let mut args = vec!["milestone"];
        args.extend_from_slice(door);
        args.extend_from_slice(&["--workflow", "no-such-workflow"]);
        let out = run(root, home.path(), &args);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

        assert!(
            !out.status.success(),
            "`milestone {verb} --workflow no-such-workflow` must exit non-zero; stdout:\n{}",
            String::from_utf8_lossy(&out.stdout),
        );
        assert!(
            stderr.contains("no-such-workflow"),
            "the {verb} rejection must name the rejected id; got:\n{stderr}",
        );
        for member in NAMED_MEMBERS {
            assert!(
                stderr.contains(member),
                "the {verb} rejection must NAME the provided workflow set (missing `{member}`); got:\n{stderr}",
            );
        }
        assert!(
            stderr.contains("route:"),
            "the {verb} rejection must be routed; got:\n{stderr}",
        );

        assert_eq!(
            task_areas(root),
            areas_before,
            "`milestone {verb}` must mint no sub-task area on a rejected --workflow",
        );
        assert_eq!(
            fs::read(record_path(root)).expect("the record survives the rejection"),
            record_before,
            "the committed record must be byte-identical after a rejected `milestone {verb}`",
        );
        assert_eq!(
            commit_log(root),
            log_before,
            "a rejected `milestone {verb}` must land no commit",
        );
    }
}

/// A valid **non-default** `--workflow` still lands at every sub-task door and is recorded —
/// the membership check refuses the bogus id without narrowing the door.
#[test]
fn a_valid_non_default_workflow_still_lands_at_every_sub_task_door() {
    for (i, door) in SUB_TASK_DOORS.iter().enumerate() {
        let verb = door[0];
        let (repo, home) = open_milestone(&format!("valid-{i}"));
        let root = repo.path();

        let mut args = vec!["milestone"];
        args.extend_from_slice(door);
        args.extend_from_slice(&["--workflow", "single-task"]);
        let out = run(root, home.path(), &args);
        assert!(
            out.status.success(),
            "`milestone {verb} --workflow single-task` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );

        let areas = task_areas(root);
        assert!(
            !areas.is_empty(),
            "`milestone {verb} --workflow single-task` must mint at least one sub-task",
        );
        let record = fs::read_to_string(record_path(root)).expect("read the record");
        for id in &areas {
            let recorded =
                engine::state::read_workflow_id(&root.join(".jigc").join("tasks").join(id))
                    .expect("read the recorded workflow")
                    .expect("a minted sub-task records its workflow");
            assert_eq!(
                recorded, "single-task",
                "`milestone {verb}` must record the caller's non-default workflow for `{id}`",
            );
            assert!(
                record.contains(id.as_str()),
                "the committed record must name sub-task `{id}`; got:\n{record}",
            );
        }
    }
}

/// The axis **boundary**, asserted rather than implied: the two neighbouring doors that take a
/// caller-supplied workflow but mint no *sub*-task already refuse an unknown id, so T2 fixes
/// the two `SUB_TASK_DOORS` rows and re-fixes nothing.
#[test]
fn the_other_workflow_doors_already_refuse_an_unknown_id() {
    let (repo, home) = open_milestone("boundary");
    let root = repo.path();

    for args in [
        vec!["start", "--workflow", "no-such-workflow", "Bound the burst"],
        vec!["workflow", "no-such-workflow", "--preview"],
    ] {
        let out = run(root, home.path(), &args);
        assert!(
            !out.status.success(),
            "`jigc {}` must exit non-zero on an unknown workflow; stdout:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
    }
    assert!(
        task_areas(root).is_empty(),
        "neither neighbouring door may mint a task on an unknown workflow",
    );
}
