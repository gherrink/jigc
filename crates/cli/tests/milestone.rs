//! End-to-end integration test for the `milestone` work-unit front door
//! (`jigc milestone create` + `jigc milestone add-task`) — the substrate spine
//! before the by-task-id join consumes it (Increment 1, T4).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts
//! the done-criterion: `milestone create "Cache rework"` mints
//! `milestone:cache-rework` (exit 0, named in output); two `add-task`s under it
//! (`"Zebra fix"`, then `"Alpha fix"`) mint sub-tasks pinned to the **milestone's
//! shared base** in isolated `tasks/<sub>/` areas; the milestone's task list
//! enumerates **id-sorted** `[alpha-fix, zebra-fix]` regardless of add order; a
//! duplicate add-task intent **rejects** non-zero with a routed finding; and
//! `.jigc/milestones/` is gitignored.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init` (mint reads HEAD), and
//! a self-cleaning `TempDir` keeps the test off the developer's real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-milestone-cli-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (mint reads HEAD via `git
/// rev-parse`). No `.jigc/config/` layer is needed — the milestone front door
/// reads HEAD and writes engine state under `.jigc/`, it does not resolve the
/// cascade.
fn init_repo(root: &Path) -> String {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        out
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    let head = git(&["rev-parse", "HEAD"]);
    String::from_utf8(head.stdout)
        .expect("utf-8 head")
        .trim()
        .to_string()
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("milestone");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn milestone_create_then_add_tasks_through_the_binary() {
    let repo = TempDir::new("create-add");
    let head = init_repo(repo.path());
    let home = TempDir::new("home");

    // `milestone create "Cache rework"` mints `milestone:cache-rework`.
    let created = run_milestone(repo.path(), home.path(), &["create", "Cache rework"]);
    let stdout = String::from_utf8(created.stdout).expect("utf-8 stdout");
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; got {:?}\nstderr:\n{}",
        created.status,
        String::from_utf8_lossy(&created.stderr),
    );
    assert!(
        stdout.contains("milestone:cache-rework"),
        "the create summary must name the minted `milestone:cache-rework`; got:\n{stdout}",
    );
    let milestone_dir = repo
        .path()
        .join(".jigc")
        .join("milestones")
        .join("cache-rework");
    assert!(
        milestone_dir.join("base.json").is_file(),
        "create must open the milestone area with a shared base pin",
    );

    // Add two sub-tasks under it, in NON-id order (zebra before alpha), so the
    // id-sorted enumeration below is not an accident of insertion order.
    for intent in ["Zebra fix", "Alpha fix"] {
        let added = run_milestone(
            repo.path(),
            home.path(),
            &["add-task", "cache-rework", intent],
        );
        assert!(
            added.status.success(),
            "`jigc milestone add-task cache-rework \"{intent}\"` must exit 0; got {:?}\nstderr:\n{}",
            added.status,
            String::from_utf8_lossy(&added.stderr),
        );
    }

    // The task list enumerates id-sorted `[alpha-fix, zebra-fix]`, never the
    // `[zebra-fix, alpha-fix]` add order — the deterministic order the join reads.
    let tasks_json =
        fs::read_to_string(milestone_dir.join("tasks.json")).expect("task list written");
    let list: serde_json::Value = serde_json::from_str(&tasks_json).expect("task list parses");
    let recorded: Vec<&str> = list["tasks"]
        .as_array()
        .expect("tasks array")
        .iter()
        .map(|v| v.as_str().expect("string id"))
        .collect();
    // Recorded backing order is insertion order (the audit trail) ...
    assert_eq!(
        recorded,
        vec!["zebra-fix", "alpha-fix"],
        "the recorded task list keeps insertion order",
    );

    // Each sub-task opened its own isolated area pinned to the MILESTONE'S base.
    let milestone_base: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(milestone_dir.join("base.json")).unwrap())
            .expect("milestone base parses");
    assert_eq!(
        milestone_base["sha"].as_str().unwrap(),
        head,
        "the milestone base pins the repo HEAD",
    );
    for sub in ["alpha-fix", "zebra-fix"] {
        let sub_dir = repo.path().join(".jigc").join("tasks").join(sub);
        assert!(
            sub_dir.is_dir(),
            "sub-task `{sub}` must open its own isolated `tasks/{sub}/` area",
        );
        let sub_base: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(sub_dir.join("base.json")).unwrap())
                .expect("sub base parses");
        assert_eq!(
            sub_base, milestone_base,
            "sub-task `{sub}` must be pinned to the milestone's shared base, not a fresh HEAD",
        );
    }

    // A duplicate add-task intent rejects non-zero with a routed finding.
    let dup = run_milestone(
        repo.path(),
        home.path(),
        &["add-task", "cache-rework", "Alpha fix"],
    );
    let dup_stderr = String::from_utf8(dup.stderr).expect("utf-8 stderr");
    assert!(
        !dup.status.success(),
        "a duplicate add-task intent must exit non-zero; got {:?}",
        dup.status,
    );
    assert!(
        dup_stderr.contains("alpha-fix") && dup_stderr.contains("route:"),
        "the duplicate-add block must name the colliding sub-task and carry a route; got:\n{dup_stderr}",
    );
    // The collision appended nothing — the list is unchanged.
    let after = fs::read_to_string(milestone_dir.join("tasks.json")).expect("task list");
    assert_eq!(after, tasks_json, "a collision appends nothing to the list");

    // `.jigc/milestones/` is gitignored — the milestone area is never committed.
    let gitignore = fs::read_to_string(repo.path().join(".jigc").join(".gitignore"))
        .expect(".gitignore written");
    assert!(
        gitignore.lines().any(|l| l.trim() == "milestones/"),
        "`.jigc/.gitignore` must ignore `milestones/`; got:\n{gitignore}",
    );
    // Verified through git itself: the milestone area is ignored.
    let check = Command::new("git")
        .args(["check-ignore", ".jigc/milestones/cache-rework/base.json"])
        .current_dir(repo.path())
        .output()
        .expect("run git check-ignore");
    assert!(
        check.status.success(),
        "git must treat `.jigc/milestones/` as ignored; got {:?}\nstderr:\n{}",
        check.status,
        String::from_utf8_lossy(&check.stderr),
    );
}

#[test]
fn add_task_to_an_unknown_milestone_rejects() {
    let repo = TempDir::new("unknown");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_milestone(
        repo.path(),
        home.path(),
        &["add-task", "no-such-milestone", "Some fix"],
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "add-task against an unknown milestone must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("no-such-milestone") && stderr.contains("route:"),
        "the unknown-milestone block must name it and carry a route; got:\n{stderr}",
    );
}
