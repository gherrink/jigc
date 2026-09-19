//! M39 Increment 4 / T3 — `milestone add-task` appends to + path-scoped-commits the
//! record (`design/team-ready-state.md` → Engine capability 1 (write), the `add-task` —
//! append arm; The commit model — path-scoped commit at each milestone op). Under a
//! `[dev ▸ methodology]` project (whose composed cascade resolves the methodology-pack
//! `milestone-record` schema), each `add-task` reads the committed record, appends one
//! `tasks` item (`task-id`/`intent`/`status: active`), writes it back, and lands a
//! **separate record-only** path-scoped commit. Dev-only (no methodology pack, no such
//! schema) degrades to today's behavior: the JSON cache still appends, but no record and
//! no extra commit.
//!
//! Two proofs, driving the REAL binary against throwaway temp git repos:
//!
//!   (RED-i)  **`[dev ▸ methodology]` create → add-task ×2.** The committed record's
//!            `tasks` carries both sub-tasks (`task-id`/`intent`/`status: active`) in
//!            append order; the second append is **byte-stable** (the first record's bytes
//!            survive as a prefix); each `add-task` is a **separate** commit naming ONLY the
//!            record, leaving a pre-staged file staged and an untracked file untracked.
//!
//!   (RED-ii) **Dev-only degrades / is inert.** With no compose marker, `add-task` resolves
//!            no `milestone-record` schema, so it materializes NO record and makes NO extra
//!            commit — exit 0, `docs/milestone-records/` absent, commit count unchanged.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use engine::milestone::read_back_record;
use engine::parse::parse_sections;
use engine::schema::{Schema, load_schema};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-add-task-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(repo);
}

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The shipped `milestone-record.yaml` schema, loaded engine-native — the record read-back
/// / re-parse both parse committed record bytes against this (structure only; docs-root does
/// not affect the parse). The schema-version stamp is injected exactly as the production
/// load does (`load_pack_schema`: milestone-record is manifest-frozen since M40 A1), so the
/// stamped record bytes parse.
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

/// Write the `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads to
/// assemble the composition dev-highest (so the dev `docs-root` knob applies → `docs/`).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`, never inheriting a
/// harness `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT, else the env pack
/// supersedes the marker).
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

/// The repo-relative paths touched by a commit (`git diff-tree` name-only over the commit).
fn commit_files(repo: &Path, rev: &str) -> Vec<String> {
    git(
        repo,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", rev],
    )
    .lines()
    .map(str::to_string)
    .collect()
}

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("cache-rework.md")
}

/// (RED-i) `[dev ▸ methodology]` create → add-task ×2 — both sub-tasks land in the
/// committed record in append order, the append byte-stable, each op a separate
/// record-only commit that leaves unrelated staged/untracked WIP untouched.
#[test]
fn methodology_add_task_appends_record_byte_stable_and_path_scoped_commits() {
    let repo = TempDir::new("compose");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Cache rework"]),
        "`[dev ▸ methodology]` `jigc milestone create`",
    );
    let after_create = fs::read_to_string(record_path(repo.path())).expect("record after create");

    // Ambient in-flight WIP that must NOT ride either record-only commit: one pre-staged
    // (in the index), one untracked (only in the worktree).
    fs::write(repo.path().join("staged.txt"), "staged WIP\n").expect("write staged");
    git(repo.path(), &["add", "staged.txt"]);
    fs::write(repo.path().join("untracked.txt"), "untracked WIP\n").expect("write untracked");

    // --- add-task #1 --------------------------------------------------------------------
    let before_1 = commit_count(repo.path());
    assert_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-task", "cache-rework", "Warm the read cache"],
        ),
        "add-task #1",
    );
    assert_eq!(
        commit_count(repo.path()),
        before_1 + 1,
        "add-task #1 lands exactly one record-only commit",
    );
    assert_eq!(
        commit_files(repo.path(), "HEAD"),
        vec!["docs/milestone-records/cache-rework.md".to_string()],
        "add-task #1's commit names ONLY the record",
    );
    let after_one = fs::read_to_string(record_path(repo.path())).expect("record after add #1");

    // --- add-task #2 --------------------------------------------------------------------
    let before_2 = commit_count(repo.path());
    assert_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-task", "cache-rework", "Evict cold entries"],
        ),
        "add-task #2",
    );
    assert_eq!(
        commit_count(repo.path()),
        before_2 + 1,
        "add-task #2 lands a SEPARATE record-only commit",
    );
    assert_eq!(
        commit_files(repo.path(), "HEAD"),
        vec!["docs/milestone-records/cache-rework.md".to_string()],
        "add-task #2's commit names ONLY the record",
    );
    let after_two = fs::read_to_string(record_path(repo.path())).expect("record after add #2");

    // Both sub-tasks present, in append order, through the REAL schema (the emitted bytes,
    // not a reconstruction).
    let schema = milestone_record_schema();
    let (_, tasks) = read_back_record(&schema, &after_two).expect("the record reads back");
    assert_eq!(
        tasks.tasks,
        vec![
            "warm-the-read-cache".to_string(),
            "evict-cold-entries".to_string()
        ],
        "the record's `tasks` carries both sub-tasks in append order",
    );

    // Each item carries its machine-set `intent` + `status: active`, in order.
    let doc = parse_sections(&schema, &after_two).expect("the record re-parses");
    let tasks_section = doc
        .sections
        .iter()
        .find(|s| s.id == "tasks")
        .expect("the parsed record carries the `tasks` section");
    let seen: Vec<(String, Option<String>, Option<String>)> = tasks_section
        .items
        .iter()
        .map(|item| {
            let leaf = |key: &str| {
                item.fields
                    .iter()
                    .find(|f| f.key == key)
                    .map(|f| f.value.render())
            };
            (item.title.clone(), leaf("intent"), leaf("status"))
        })
        .collect();
    assert_eq!(
        seen,
        vec![
            (
                "warm-the-read-cache".to_string(),
                Some("Warm the read cache".to_string()),
                Some("active".to_string()),
            ),
            (
                "evict-cold-entries".to_string(),
                Some("Evict cold entries".to_string()),
                Some("active".to_string()),
            ),
        ],
        "both sub-tasks carry their machine-set intent + status: active in append order",
    );

    // Byte-stable append: the second append is confined to the new item's span — the whole
    // one-item record survives byte-identical as a prefix of the two-item record. And the
    // first append never disturbed the freshly-created header/H1/`## Tasks` — `after_create`
    // (trimmed of its trailing empty-section newline) is itself a prefix of `after_one`.
    assert!(
        after_two.starts_with(after_one.trim_end_matches('\n')),
        "add-task #2 leaves the first record's bytes untouched outside the appended span:\n\
         --- after_one ---\n{after_one}\n--- after_two ---\n{after_two}",
    );
    assert!(
        after_one.starts_with(after_create.trim_end_matches('\n')),
        "add-task #1 leaves the created record's bytes untouched outside the appended span:\n\
         --- after_create ---\n{after_create}\n--- after_one ---\n{after_one}",
    );

    // The ambient WIP survived both path-scoped commits untouched: `staged.txt` still staged,
    // `untracked.txt` still untracked.
    let staged = git(repo.path(), &["diff", "--cached", "--name-only"]);
    assert!(
        staged.lines().any(|l| l == "staged.txt"),
        "the pre-staged file must remain staged after both record commits; got staged:\n{staged}",
    );
    let untracked = git(repo.path(), &["ls-files", "--others", "--exclude-standard"]);
    assert!(
        untracked.lines().any(|l| l == "untracked.txt"),
        "the untracked file must remain untracked after both record commits; got:\n{untracked}",
    );
}

/// (RED-ii) Dev-only `add-task` — no compose marker, no `milestone-record` schema — is
/// inert on the record: it materializes NO record and makes NO extra commit (the JSON cache
/// still appends; today's behavior preserved). The omitting-context proof (the feature must
/// not error or over-commit where its target doctype is absent).
#[test]
fn dev_only_add_task_materializes_no_record_and_makes_no_commit() {
    let repo = TempDir::new("dev-only");
    let home = TempDir::new("home");
    init_repo(repo.path());

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Cache rework"]),
        "dev-only `jigc milestone create`",
    );

    let before = commit_count(repo.path());
    assert_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-task", "cache-rework", "Warm the read cache"],
        ),
        "dev-only add-task",
    );

    assert!(
        !repo.path().join("docs").join("milestone-records").exists(),
        "dev-only add-task must materialize NO milestone record (no methodology pack)",
    );
    assert_eq!(
        commit_count(repo.path()),
        before,
        "dev-only add-task must make NO extra commit",
    );
}
