//! Acceptance — **`milestone-record` 2 → 3: the record gains the per-task `workflow`
//! leaf** (M49 Increment 9, T2; `completions/artifacts/M49/settle-record.md` → Tier 3;
//! `design/team-ready-state.md` → The lifecycle / the committed record;
//! `design/corpus-migration.md` → the classifier's kinds (`AddedItemField`) ·
//! Prior-schema sourcing; `implementation/doctype-authoring.md` → Freeze / versioning).
//!
//! The committed record is what a teammate on a **fresh clone** resumes a milestone
//! from — and the one thing it never carried is *which workflow* each sub-task was
//! minted against. That value lives only in the gitignored `.jigc/` workbench, so it is
//! **not fresh-clone durable**, and a guard that compares recorded workflows then
//! asserts provenance it cannot actually see. Schema-version 3 declares the leaf; T3
//! delivers the behaviour it exists for. The bump lands **alone** here so its migration
//! is its own red step.
//!
//! The leaf is `{ id: workflow, type: string, set: on-transition }` — machine-maintained
//! like every other leaf on this doctype. That makes the bump a **byte no-op** on every
//! committed record, and the two source facts that make it one are **driven here, not
//! trusted**:
//!
//!   * the diff classifies `AddedItemField`, whose driver returns the source
//!     **unchanged** for a `default`-less, non-author-required leaf
//!     (`engine::transform` → `apply_added_item_field`) — so a committed record with
//!     real `tasks` items folds with the stamp as its only byte delta; and
//!   * `schema-conformance.required-field-present` fires **only** on an author-required
//!     field (`engine::validate` → `is_author_required`) — so the migrated record, which
//!     carries no `workflow` bullet on any item, still validates clean.
//!
//! Every arm drives the **shipped binary** against a throwaway `[dev ▸ methodology]`
//! repo — the real embedded packs, the real freeze manifest, the real snapshot store —
//! and asserts the **bytes on disk** or the **emitted JSON**, never a reconstruction.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-v3-{tag}-{}-{:?}",
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
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A real git repo with one commit and the `[dev ▸ methodology]` compose marker — the
/// exact project-layer key that composes the embedded methodology pack over the dev
/// pack, so the `milestone-record` under test is the **shipped** one.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation exited 0, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The committed record's repo-relative path under the composed `docs-root`.
const RECORD_REL: &str = "docs/milestone-records/cache-rework.md";

/// Mint a milestone and one sub-task through the **real** verbs, returning the committed
/// record's bytes — the exact shape every real `milestone-record` on disk is in.
fn minted_record(repo: &Path, home: &Path) -> String {
    assert_ok(
        &jigc(repo, home, &["milestone", "create", "Cache rework"]),
        "jigc milestone create",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &["milestone", "add-task", "cache-rework", "Zebra fix"],
        ),
        "jigc milestone add-task",
    );
    fs::read_to_string(repo.join(RECORD_REL)).expect("read the committed milestone record")
}

// ---------------------------------------------------------------------------------------------
// (1) The mint — a fresh record stamps the new version.
// ---------------------------------------------------------------------------------------------

/// **A fresh mint carries `schema-version: 3`.** The stamp the CLI materializes is the
/// manifest's declared version, so the bump reaches the mint site with no code change.
///
/// RED before the bump: the record stamped `schema-version: 2`.
#[test]
fn a_fresh_milestone_record_mints_at_schema_version_3() {
    let repo = TempDir::new("mint");
    let home = TempDir::new("mint-home");
    init_repo(repo.path());

    let record = minted_record(repo.path(), home.path());
    assert!(
        record.contains("schema-version: 3\n"),
        "a freshly minted milestone-record stamps the current schema-version; got:\n{record}"
    );
}

// ---------------------------------------------------------------------------------------------
// (2) The corpus migration — the stamp is the only byte the fold moves.
// ---------------------------------------------------------------------------------------------

/// **The migration.** A committed `schema-version: 2` record — one carrying a real
/// `tasks` item, the only place the new leaf could ever splice — folds to 3 under
/// `jigc migrate-corpus`, reported as migrated and **byte-identical except the stamp**.
///
/// The v2 source is not hand-written: it is the binary's own v3 mint with the stamp
/// wound back, which is exactly the v2 shape (the new leaf is machine-maintained, so
/// the writer materializes no bullet for it) and therefore the exact bytes every record
/// committed before this bump holds.
///
/// RED before the bump: the shipped doctype was itself at v2, so the doc was reported
/// `already-current` and `migrated[]` was empty.
#[test]
fn a_committed_v2_record_migrates_with_the_stamp_as_its_only_byte_delta() {
    let repo = TempDir::new("migrate");
    let home = TempDir::new("migrate-home");
    init_repo(repo.path());

    let at_v3 = minted_record(repo.path(), home.path());
    let at_v2 = at_v3.replace("schema-version: 3", "schema-version: 2");
    assert_ne!(
        at_v2, at_v3,
        "the mint must carry a stamp for the wind-back to have anything to move"
    );
    fs::write(repo.path().join(RECORD_REL), &at_v2).expect("wind the stamp back to v2");
    git(repo.path(), &["add", RECORD_REL]);
    git(repo.path(), &["commit", "-q", "-m", "seed the v2 record"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let stdout = assert_ok(&out, "jigc migrate-corpus over a v2 milestone-record");
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the report is JSON ({e}); stdout:\n{stdout}"));
    let migrated: Vec<&str> = report["migrated"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `migrated[]`; got:\n{report:#}"))
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect();
    assert_eq!(
        migrated,
        [RECORD_REL],
        "exactly the v2 record migrates; report:\n{report:#}"
    );

    let after = fs::read_to_string(repo.path().join(RECORD_REL)).expect("read the migrated record");
    assert_eq!(
        after, at_v3,
        "the stamp is the migration's only byte delta: `AddedItemField` for a \
         `default`-less, machine-maintained leaf writes nothing on any item",
    );
}

/// **The migrated record still validates.** No item carries a `workflow` bullet, and
/// none has to: the leaf is `set:`-bearing, so it is not author-required and
/// `schema-conformance.required-field-present` never names it.
///
/// RED before the bump: the corpus held a v2 doc under a v2 schema, so the arm proved
/// nothing about the added leaf.
#[test]
fn the_migrated_record_validates_with_no_workflow_bullet_on_any_item() {
    let repo = TempDir::new("validate");
    let home = TempDir::new("validate-home");
    init_repo(repo.path());

    let at_v3 = minted_record(repo.path(), home.path());
    fs::write(
        repo.path().join(RECORD_REL),
        at_v3.replace("schema-version: 3", "schema-version: 2"),
    )
    .expect("wind the stamp back to v2");
    git(repo.path(), &["add", RECORD_REL]);
    git(repo.path(), &["commit", "-q", "-m", "seed the v2 record"]);
    assert_ok(
        &jigc(repo.path(), home.path(), &["migrate-corpus"]),
        "jigc migrate-corpus over a v2 milestone-record",
    );

    let after = fs::read_to_string(repo.path().join(RECORD_REL)).expect("read the migrated record");
    assert!(
        !after.contains("workflow"),
        "the migration writes no `workflow` bullet — the leaf is machine-maintained, and \
         T3 is what fills it; got:\n{after}"
    );
    let stdout = assert_ok(
        &jigc(repo.path(), home.path(), &["validate"]),
        "jigc validate over the migrated corpus",
    );
    assert!(
        !stdout.contains("required-field-present"),
        "a `set:`-bearing leaf is never the author's obligation; validate said:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------------------------
// (3) The declared shape — the leaf is on the item block, and it is machine-maintained.
// ---------------------------------------------------------------------------------------------

/// **The read surface shows the leaf where it lives.** `jigc doc schema` projects
/// `workflow` inside the `tasks` **item block** (not the header), declared
/// `set: on-transition` and therefore not author-required — the projection an agent
/// reads to learn what it may write.
///
/// RED before the bump: no `workflow` leaf existed to project.
#[test]
fn doc_schema_projects_the_workflow_leaf_on_the_tasks_item_block() {
    let repo = TempDir::new("schema");
    let home = TempDir::new("schema-home");
    init_repo(repo.path());

    let stdout = assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "schema", "milestone-record", "--format", "json"],
        ),
        "jigc doc schema milestone-record --format json",
    );
    let projection: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the projection is JSON ({e}); stdout:\n{stdout}"));
    assert_eq!(
        projection["schema-version"], 3,
        "the projection states the shipped schema-version; got:\n{projection:#}"
    );

    let tasks = projection["sections"]
        .as_array()
        .unwrap_or_else(|| panic!("the projection carries `sections[]`; got:\n{projection:#}"))
        .iter()
        .find(|s| s["id"] == "tasks")
        .unwrap_or_else(|| {
            panic!("the projection carries the `tasks` section; got:\n{projection:#}")
        });
    let leaf = tasks["item"]["fields"]
        .as_array()
        .unwrap_or_else(|| panic!("the `tasks` item block carries `fields[]`; got:\n{tasks:#}"))
        .iter()
        .find(|f| f["id"] == "workflow")
        .unwrap_or_else(|| {
            panic!("the `tasks` item block declares the `workflow` leaf; got:\n{tasks:#}")
        });
    assert_eq!(
        leaf["type"], "string",
        "the leaf is a plain string: {leaf:#}"
    );
    assert_eq!(
        leaf["set"], "on-transition",
        "the leaf is machine-maintained, like every other leaf on this doctype: {leaf:#}"
    );
    assert_eq!(
        leaf["author-required"], false,
        "a `set:`-bearing leaf is never the author's obligation: {leaf:#}"
    );

    // The header is NOT where it lives: a milestone's workflow is per sub-task.
    let header: Vec<&str> = projection["fields"]
        .as_array()
        .map(|fields| {
            fields
                .iter()
                .filter_map(|f| f["id"].as_str())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    assert!(
        !header.contains(&"workflow"),
        "the leaf is per sub-task, not per milestone; header fields: {header:?}"
    );
}
