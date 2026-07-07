//! M39 Increment 4 / T2 — the record-home split at `jigc milestone create`
//! (`design/team-ready-state.md` → The `milestone-record` doctype (home); The commit
//! model — path-scoped commit at each milestone op). `create` now materializes the
//! committed team-ready `milestone-record` under docs-root
//! (`docs/milestone-records/<id>.md`) — `base` + `status: active` + an empty `tasks`
//! section — and lands a **record-only** path-scoped commit, but ONLY under a
//! `[dev ▸ methodology]` project (whose composed cascade resolves the methodology-pack
//! `milestone-record` schema). Dev-only (no methodology pack, no such schema) degrades to
//! today's behavior: no record, no extra commit.
//!
//! Two proofs, driving the REAL binary against throwaway temp git repos:
//!
//!   (RED-i)  **Dev-only degrades.** With no `.jigc/config` (no compose marker), `create`
//!            resolves no `milestone-record` schema, so it materializes NO record and makes
//!            NO commit — exit 0, `docs/milestone-records/` absent, commit count unchanged.
//!
//!   (RED-ii) **`[dev ▸ methodology]` materializes + path-scoped-commits.** With the compose
//!            marker, `create` writes the committed `docs/milestone-records/<id>.md`
//!            (`base` pinning HEAD, `status: active`, empty `tasks`) and commits ONLY it:
//!            exactly one new commit naming ONLY the record — a **pre-staged unrelated file**
//!            stays out of the record commit and stays staged in the index (the M30/M31
//!            path-scoped staging discipline).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp repo
//! is a real `git init`, the composition is driven by the setup-written `packs.yaml` compose
//! marker (written directly here — the exact key `make_pack` reads), and a self-cleaning
//! `TempDir` keeps the test off the developer's real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use engine::milestone::read_back_record;
use engine::schema::load_schema;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-create-{tag}-{}-{:?}",
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
fn init_repo(repo: &Path) -> String {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    git(repo, &["rev-parse", "HEAD"]).trim().to_string()
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
/// parses committed record bytes against this (structure only; docs-root does not affect
/// the parse).
fn milestone_record_schema() -> engine::schema::Schema {
    let bytes = fs::read(
        methodology_pack_tree()
            .join("schemas")
            .join("milestone-record.yaml"),
    )
    .expect("read milestone-record.yaml");
    load_schema(&bytes).expect("milestone-record.yaml loads engine-native")
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

/// (RED-i) Dev-only `create` — no `.jigc/config`, no compose marker — resolves no
/// `milestone-record` schema, so it materializes NO record and makes NO commit: exit 0,
/// `docs/milestone-records/` absent, commit count unchanged (today's behavior preserved).
#[test]
fn dev_only_create_materializes_no_record_and_makes_no_commit() {
    let repo = TempDir::new("dev-only");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let before = commit_count(repo.path());

    let created = run_milestone(repo.path(), home.path(), &["create", "Cache rework"]);
    assert!(
        created.status.success(),
        "dev-only `jigc milestone create` must exit 0; got {:?}\nstderr:\n{}",
        created.status,
        String::from_utf8_lossy(&created.stderr),
    );

    assert!(
        !repo.path().join("docs").join("milestone-records").exists(),
        "dev-only create must materialize NO milestone record (no methodology pack)",
    );
    assert_eq!(
        commit_count(repo.path()),
        before,
        "dev-only create must make NO extra commit",
    );
}

/// (RED-ii) `[dev ▸ methodology]` `create` materializes the committed record under docs-root
/// and commits ONLY it — a pre-staged unrelated file stays out of the record commit and stays
/// staged (the path-scoped commit discipline).
#[test]
fn methodology_create_materializes_record_and_path_scoped_commits() {
    let repo = TempDir::new("compose");
    let home = TempDir::new("home");
    let head = init_repo(repo.path());

    // The compose marker — the exact key `make_pack` reads to assemble `[dev ▸ methodology]`
    // (dev-highest, so the dev `docs-root` knob applies → `docs/`). Written directly; no
    // `JIGC_PACK_DIR`, no listed packs.
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");

    // A pre-staged UNRELATED file — must NOT ride the record-only commit, and must stay
    // staged afterward (the WIP-safety the path-scoped commit guarantees).
    fs::write(repo.path().join("unrelated.txt"), "in-flight WIP\n").expect("write unrelated");
    git(repo.path(), &["add", "unrelated.txt"]);

    let before = commit_count(repo.path());

    let created = run_milestone(repo.path(), home.path(), &["create", "Cache rework"]);
    assert!(
        created.status.success(),
        "`[dev ▸ methodology]` `jigc milestone create` must exit 0; got {:?}\nstderr:\n{}",
        created.status,
        String::from_utf8_lossy(&created.stderr),
    );

    // The committed record exists under docs-root at the milestone work-unit id.
    let record = repo
        .path()
        .join("docs")
        .join("milestone-records")
        .join("cache-rework.md");
    assert!(
        record.is_file(),
        "create must materialize the committed record at {record:?}",
    );
    let body = fs::read_to_string(&record).expect("read the materialized record");

    // `base` pins HEAD, `status: active`, empty `tasks` — read back through the REAL schema
    // (drives the emitted committed bytes, not a reconstruction).
    assert!(
        body.contains("status: active"),
        "the fresh record seeds `status: active`; got:\n{body}",
    );
    let (base, tasks) =
        read_back_record(&milestone_record_schema(), &body).expect("the record reads back");
    assert_eq!(base.sha, head, "the record's `base` pins the repo HEAD");
    assert!(
        tasks.tasks.is_empty(),
        "a freshly-created record carries an EMPTY task list; got {:?}",
        tasks.tasks,
    );

    // Exactly one new commit, naming ONLY the record — the pre-staged unrelated file is NOT
    // in it (path-scoped commit).
    assert_eq!(
        commit_count(repo.path()),
        before + 1,
        "create lands exactly one record-only commit",
    );
    let files = commit_files(repo.path(), "HEAD");
    assert_eq!(
        files,
        vec!["docs/milestone-records/cache-rework.md".to_string()],
        "the record commit names ONLY the record — not the pre-staged unrelated file; got {files:?}",
    );

    // The unrelated file stays staged (still in the index, uncommitted).
    let staged = git(repo.path(), &["diff", "--cached", "--name-only"]);
    assert!(
        staged.lines().any(|l| l == "unrelated.txt"),
        "the pre-staged unrelated file must remain staged (path-scoped commit left it \
         untouched); got staged:\n{staged}",
    );
}
