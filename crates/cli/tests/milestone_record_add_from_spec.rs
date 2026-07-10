//! M39 Increment 4 — `milestone add-from-spec` seeds the committed record (`design/
//! team-ready-state.md` → Engine capability 1 (write), the `add-task` append arm; The
//! commit model — path-scoped commit at each milestone op; Engine capability 2 (read-back) —
//! the committed record is the source of truth, rebuilt on a fresh clone). Under a
//! `[dev ▸ methodology]` project, `add-from-spec` mints one sub-task per spec criterion, and
//! — exactly like `add-task` — each seeded sub-task must land in the committed
//! `milestone-record` (a separate record-only path-scoped commit), so a teammate on a fresh
//! clone re-derives the seeded task list. A bypass here silently loses spec-seeded sub-tasks
//! on clone (the source-of-truth invariant broken).
//!
//! Two proofs, driving the REAL binary against throwaway temp git repos:
//!
//!   (RED-i)  **`[dev ▸ methodology]` create → add-from-spec.** The committed record's
//!            `tasks` carries every seeded sub-task (`task-id`/`intent`/`status: active`),
//!            and a **fresh clone** (delete `.jigc/`, restore the committed tree) re-derives
//!            the full seeded task list from the record — nothing lost.
//!
//!   (RED-ii) **Dev-only degrades / is inert.** With no compose marker, `add-from-spec`
//!            resolves no `milestone-record` schema, so it materializes NO record and makes NO
//!            extra commit beyond the seed — exit 0, `docs/milestone-records/` absent.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use engine::milestone::read_back_record;
use engine::schema::{Schema, load_schema};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-add-from-spec-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The shipped `milestone-record.yaml` schema, loaded engine-native, with the
/// schema-version stamp injected exactly as the production load does
/// (`load_pack_schema`: milestone-record is manifest-frozen since M40 A1).
fn milestone_record_schema() -> Schema {
    let bytes = fs::read(
        methodology_pack_tree()
            .join("schemas")
            .join("milestone-record.yaml"),
    )
    .expect("read milestone-record.yaml");
    let mut schema = load_schema(&bytes).expect("milestone-record.yaml loads engine-native");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// Write the `[dev ▸ methodology]` compose marker.
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`, never inheriting a
/// harness `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The number of commits reachable from HEAD.
fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("commit count parses")
}

/// A committed 2-criteria spec — the seed substrate.
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

/// Write + commit the spec at its canonical committed path under docs-root.
fn commit_spec(repo: &Path, slug: &str, body: &str) {
    let specs = repo.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join(format!("{slug}.md")), body).expect("write spec");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "add spec"]);
}

/// The committed record path under docs-root for milestone `rate-limit`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("rate-limit.md")
}

/// (RED-i) `[dev ▸ methodology]` create → add-from-spec — every seeded sub-task lands in the
/// committed record, and a **fresh clone** (delete `.jigc/`, restore the tree) re-derives the
/// full seeded task list from the record. The regression witness: a bypass loses the
/// spec-seeded sub-tasks on clone.
#[test]
fn methodology_add_from_spec_seeds_the_record_and_survives_a_fresh_clone() {
    let repo = TempDir::new("compose");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    // The compose marker + config must be committed so the fresh-clone restore keeps them.
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "compose marker"]);

    commit_spec(repo.path(), "rate-limit", TWO_CRITERIA_SPEC);

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Rate limit"]),
        "`[dev ▸ methodology]` `jigc milestone create`",
    );

    let before = commit_count(repo.path());
    assert_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-from-spec", "rate-limit", "spec:rate-limit"],
        ),
        "add-from-spec",
    );

    // One record-only commit per seeded sub-task (2 criteria → 2 commits).
    assert_eq!(
        commit_count(repo.path()),
        before + 2,
        "add-from-spec lands one record-only commit per seeded sub-task",
    );

    // The committed record carries both seeded sub-tasks (the emitted bytes, real schema).
    let schema = milestone_record_schema();
    let source = fs::read_to_string(record_path(repo.path())).expect("record after seed");
    let (_, tasks) = read_back_record(&schema, &source).expect("the record reads back");
    assert_eq!(
        tasks.tasks,
        vec![
            "rejects-the-101st-request".to_string(),
            "admits-within-the-window".to_string(),
        ],
        "the committed record's `tasks` carries both spec-seeded sub-tasks",
    );

    // --- fresh-clone resume: delete `.jigc/`, restore the committed tree, re-derive ---------
    fs::remove_dir_all(repo.path().join(".jigc")).expect("rm .jigc");
    git(repo.path(), &["checkout", "--", ".jigc"]);

    let listed = run_milestone(repo.path(), home.path(), &["list-tasks", "rate-limit"]);
    assert_ok(&listed, "list-tasks on a fresh clone");
    let listed_stdout = String::from_utf8(listed.stdout).expect("utf-8 stdout");
    // Both spec-seeded sub-tasks re-derive from the record — nothing lost on clone.
    for id in ["rejects-the-101st-request", "admits-within-the-window"] {
        assert!(
            listed_stdout.contains(id),
            "fresh-clone list-tasks must re-derive spec-seeded sub-task `{id}` from the record; got:\n{listed_stdout}",
        );
    }
    assert!(
        listed_stdout.contains("tasks (2)"),
        "fresh-clone list-tasks must re-derive both seeded sub-tasks; got:\n{listed_stdout}",
    );
}

/// (RED-ii) Dev-only `add-from-spec` — no compose marker, no `milestone-record` schema — is
/// inert on the record: it materializes NO record (today's behavior preserved), exit 0.
#[test]
fn dev_only_add_from_spec_materializes_no_record() {
    let repo = TempDir::new("dev-only");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_spec(repo.path(), "rate-limit", TWO_CRITERIA_SPEC);

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Rate limit"]),
        "dev-only `jigc milestone create`",
    );
    assert_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-from-spec", "rate-limit", "spec:rate-limit"],
        ),
        "dev-only add-from-spec",
    );

    assert!(
        !repo.path().join("docs").join("milestone-records").exists(),
        "dev-only add-from-spec must materialize NO milestone record (no methodology pack)",
    );
}
