//! M49 Increment 2 / T4 — **no `VerbKind::Read` leaf acts**, and the rows and the
//! doc-comment finally agree.
//!
//! [`VERB_KINDS`](cli::cli::VERB_KINDS) is the code-side registry of jigc's whole leaf
//! surface, and its doc-comment states the rule the rows are supposed to satisfy: a verb is
//! [`VerbKind::Read`] *only* when no invocation of it acts — it "reports what is there and
//! mutates nothing — neither repo files, nor the git index/history, nor the `.jigc/`
//! workbench". That was a claim about twelve rows and a measurement of none.
//!
//! **The subject is the registry, never a reported instance.** This suite enumerates the
//! `Read` rows out of `VERB_KINDS` in code, drives each one through the **real binary**
//! against a state built to *reveal* a write, each from its own pristine copy, and asserts
//! the whole repo tree — the gitignored `.jigc/` workbench included — is byte-identical
//! afterwards. A thirteenth `Read` row cannot ship without an answer here
//! ([`every_read_verb_has_an_invocation`]).
//!
//! **The revealing state** ([`Fixture`]) is the fresh-clone shape, because that is the shape
//! in which a re-seed is *reachable*: a committed `milestone-record` naming two sub-tasks
//! whose working areas are **absent** (one of them minted in the origin under a non-default
//! `--workflow`), a live ordinary task, and no persisted edge index. Over that state ten of
//! the twelve rows were already clean; `milestone list-tasks` wrote six files — rebuilding
//! both absent sub-task areas, each carrying a `workflow` file the committed record does not
//! carry and therefore cannot source, so the overridden sub-task came back under the pack
//! **default**. A read verb fabricating provenance a later door reads as authority.
//!
//! **The one carve-out, admitted by name and tied to what makes it safe.**
//! [`DERIVED_CACHE_PREFIX`] admits `.jigc/index/` — `jigc task validate` materializes
//! `edges.json` on its first run against a new HEAD (`crates/engine/src/index.rs` →
//! `load_committed`; [storage.md](../../../design/storage.md) → Edge index lifecycle, the
//! *committed rebuild* site). It is admitted because it is a **self-healing derived cache**
//! rather than authority, and the two properties that make it one are asserted here, not
//! assumed: the file it writes carries the repo's **current HEAD** as its stamp (a pure
//! function of the committed store), and a second run of the same verb changes **nothing**
//! (a fixed point, not an accumulating write).

use cli::cli::{VERB_KINDS, VerbKind};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The milestone the fixture's committed record names.
const MILESTONE_ID: &str = "cache-rework";
/// The sub-task minted in the origin under the **pack default** sub-task workflow.
const DEFAULT_SUB: &str = "warm-the-read-cache";
/// The sub-task minted in the origin under a **non-default** `--workflow` — the one whose
/// provenance a re-seed cannot source from the record, and therefore fabricates.
const OVERRIDDEN_SUB: &str = "evict-cold-entries";
/// The non-default workflow `OVERRIDDEN_SUB` was minted with.
const OVERRIDE_WORKFLOW: &str = "single-task";
/// The live ordinary task the fixture carries (`task diff` / `task validate` address it).
const ORDINARY_TASK: &str = "tidy-the-read-path";

/// The one path prefix a [`VerbKind::Read`] verb may materialize: the **derived edge-index
/// cache**, which is stamp-rebuildable by construction and is authority for nothing.
const DERIVED_CACHE_PREFIX: &str = ".jigc/index/";

/// The `<TASK>` marker in an [`INVOCATIONS`] row, substituted with the live ordinary task id.
const TASK_MARKER: &str = "<TASK>";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-read-verb-{tag}-{}-{:?}",
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
    String::from_utf8(out.stdout)
        .expect("utf-8 git stdout")
        .trim_end()
        .to_string()
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT, else the env pack supersedes
/// the marker).
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert an invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

// =======================================================================================
// The revealing fixture — built once by driving the binary, copied pristine per row.
// =======================================================================================

/// The fresh-clone state every row is driven against, plus the origin it was cloned from
/// (which is what proves the overridden sub-task's *real* recorded workflow).
struct Fixture {
    /// The `TempDir`s the fixture lives in — held so the paths stay alive.
    _origin_dir: TempDir,
    _clone_dir: TempDir,
    _home: TempDir,
    /// The fresh clone every row copies.
    clone: PathBuf,
    /// The workflow the origin recorded for [`OVERRIDDEN_SUB`].
    recorded_override: String,
}

/// Initialize a `[dev ▸ methodology]` origin repo whose initial commit carries the compose
/// marker, so the milestone's base pin is that commit.
fn init_methodology_origin(repo: &Path) {
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

impl Fixture {
    fn build() -> Self {
        let origin_dir = TempDir::new("origin");
        let home = TempDir::new("home");
        let origin = origin_dir.path().to_path_buf();
        init_methodology_origin(&origin);

        assert_ok(
            &run_jigc(
                &origin,
                home.path(),
                &["milestone", "create", "Cache rework"],
            ),
            "`jigc milestone create`",
        );
        assert_ok(
            &run_jigc(
                &origin,
                home.path(),
                &["milestone", "add-task", MILESTONE_ID, "Warm the read cache"],
            ),
            "`jigc milestone add-task` (pack default workflow)",
        );
        assert_ok(
            &run_jigc(
                &origin,
                home.path(),
                &[
                    "milestone",
                    "add-task",
                    MILESTONE_ID,
                    "Evict cold entries",
                    "--workflow",
                    OVERRIDE_WORKFLOW,
                ],
            ),
            "`jigc milestone add-task --workflow` (the non-default override)",
        );

        // What the origin ACTUALLY recorded for the overridden sub-task — the provenance a
        // re-seed from the record cannot source, because the record does not carry it.
        let recorded_override = fs::read_to_string(
            origin
                .join(".jigc")
                .join("tasks")
                .join(OVERRIDDEN_SUB)
                .join("workflow"),
        )
        .expect("the origin recorded the overridden sub-task's workflow")
        .trim()
        .to_string();
        assert_eq!(
            recorded_override, OVERRIDE_WORKFLOW,
            "the fixture's overridden sub-task must record the non-default workflow",
        );

        // The fresh clone: exactly the tracked bytes a teammate gets.
        let clone_dir = TempDir::new("clone");
        let clone = clone_dir.path().join("clone");
        git(
            clone_dir.path(),
            &[
                "clone",
                "-q",
                &origin.display().to_string(),
                &clone.display().to_string(),
            ],
        );
        git(&clone, &["config", "user.email", "test@example.com"]);
        git(&clone, &["config", "user.name", "Test"]);

        // A live ordinary task, so `task diff` / `task validate` address real state.
        assert_ok(
            &run_jigc(
                &clone,
                home.path(),
                &[
                    "start",
                    "--workflow",
                    "single-task",
                    "tidy the read path",
                    "--slug",
                    ORDINARY_TASK,
                ],
            ),
            "`jigc start` (the live ordinary task)",
        );

        // The state is REVEALING — assert each property a write would disturb.
        assert!(
            clone
                .join("docs")
                .join("milestone-records")
                .join(format!("{MILESTONE_ID}.md"))
                .is_file(),
            "the clone carries the committed milestone record",
        );
        for sub in [DEFAULT_SUB, OVERRIDDEN_SUB] {
            assert!(
                !clone.join(".jigc").join("tasks").join(sub).exists(),
                "the fixture's sub-task area `{sub}` must be ABSENT — that absence is what a \
                 re-seeding read verb fills in",
            );
        }
        assert!(
            !clone.join(".jigc").join("milestones").exists(),
            "the fixture must carry no milestone cache",
        );
        assert!(
            !clone.join(".jigc").join("index").exists(),
            "the fixture must carry no persisted edge index",
        );
        assert!(
            clone
                .join(".jigc")
                .join("tasks")
                .join(ORDINARY_TASK)
                .is_dir(),
            "the fixture carries a live ordinary task",
        );

        Fixture {
            _origin_dir: origin_dir,
            _clone_dir: clone_dir,
            _home: home,
            clone,
            recorded_override,
        }
    }
}

// =======================================================================================
// Whole-tree snapshots — the assertion instrument.
// =======================================================================================

/// Every path under `root` with its bytes, `.git/` excluded (git's own index is compared
/// through [`git_state`] instead: a plain `git status` refreshes the index's stat cache, so
/// comparing those bytes would flag the *instrument*, not the verb). A directory is recorded
/// as an entry of its own, so creating an empty one is a difference.
fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("read {}: {err}", dir.display()))
        .map(|e| e.expect("dir entry"))
        .collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .expect("under root")
            .to_string_lossy()
            .replace('\\', "/");
        if rel == ".git" {
            continue;
        }
        let ft = entry.file_type().expect("file type");
        if ft.is_dir() {
            out.insert(format!("{rel}/"), b"<dir>".to_vec());
            walk(root, &path, out);
        } else if ft.is_file() {
            out.insert(rel, fs::read(&path).expect("read file"));
        } else {
            panic!("unexpected non-file, non-dir entry in the fixture: {rel}");
        }
    }
}

/// Git's *content* state — HEAD, the staged/worktree diff, and the reachable commit count.
/// Between them these move if a verb touched the index or the history.
///
/// The `.jigc/` workbench is **excluded from the pathspec**, not from the assertion: it is
/// gitignored in a set-up repo and merely untracked in this one, so leaving it in would report
/// the workbench through two instruments at once and let the carve-out below look like a git
/// change. [`snapshot`] is what watches `.jigc/`, byte for byte.
fn git_state(repo: &Path) -> String {
    format!(
        "HEAD {}\ncommits {}\nstatus\n{}\n",
        git(repo, &["rev-parse", "HEAD"]),
        git(repo, &["rev-list", "--all", "--count"]),
        git(
            repo,
            &["status", "--porcelain=v1", "--", ".", ":(exclude).jigc"],
        ),
    )
}

/// The paths whose presence or bytes differ between two snapshots.
fn differing(before: &BTreeMap<String, Vec<u8>>, after: &BTreeMap<String, Vec<u8>>) -> Vec<String> {
    let keys: BTreeSet<&String> = before.keys().chain(after.keys()).collect();
    keys.into_iter()
        .filter(|k| before.get(*k) != after.get(*k))
        .cloned()
        .collect()
}

/// Copy a whole directory tree, bytes and structure — how each row gets its pristine copy.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create the copy root");
    for entry in fs::read_dir(src).expect("read the source dir") {
        let entry = entry.expect("dir entry");
        let to = dst.join(entry.file_name());
        let ft = entry.file_type().expect("file type");
        if ft.is_dir() {
            copy_tree(&entry.path(), &to);
        } else if ft.is_file() {
            fs::copy(entry.path(), &to).expect("copy file");
        } else {
            panic!("unexpected non-file, non-dir entry: {}", to.display());
        }
    }
}

// =======================================================================================
// The rows — enumerated from VERB_KINDS, driven from this table.
// =======================================================================================

/// How each [`VerbKind::Read`] leaf is **driven** against the fixture. The *axis* is
/// [`VERB_KINDS`] itself; this table only says what arguments each row takes, and
/// [`every_read_verb_has_an_invocation`] fences the two against each other in both
/// directions.
const INVOCATIONS: &[(&[&str], &[&str])] = &[
    (&["upgrade"], &["upgrade"]),
    (&["describe"], &["describe"]),
    (&["validate"], &["validate"]),
    (
        &["doc", "show"],
        &["doc", "show", "milestone-record:cache-rework"],
    ),
    (&["doc", "schema"], &["doc", "schema", "adr"]),
    (&["doc", "list"], &["doc", "list"]),
    (&["task", "list"], &["task", "list"]),
    (&["task", "diff"], &["task", "diff", TASK_MARKER]),
    (&["task", "validate"], &["task", "validate", TASK_MARKER]),
    (&["config", "get"], &["config", "get", "docs-root"]),
    (&["config", "list"], &["config", "list"]),
    (
        &["milestone", "list-tasks"],
        &["milestone", "list-tasks", MILESTONE_ID],
    ),
];

/// The `Read` rows of [`VERB_KINDS`], read off the registry in code.
fn read_rows() -> Vec<Vec<String>> {
    VERB_KINDS
        .iter()
        .filter(|(_, kind)| *kind == VerbKind::Read)
        .map(|(path, _)| path.iter().map(|s| (*s).to_string()).collect())
        .collect()
}

#[test]
fn every_read_verb_has_an_invocation() {
    let rows: BTreeSet<Vec<String>> = read_rows().into_iter().collect();
    let driven: BTreeSet<Vec<String>> = INVOCATIONS
        .iter()
        .map(|(path, _)| path.iter().map(|s| (*s).to_string()).collect())
        .collect();
    assert_eq!(
        rows, driven,
        "every `VerbKind::Read` leaf must be driven here, and nothing else may be — the axis \
         is the registry, not this table",
    );
    assert_eq!(
        INVOCATIONS.len(),
        rows.len(),
        "the invocation table carries no duplicate rows",
    );
}

/// The sweep: every `Read` row, over the revealing state, from a pristine copy.
#[test]
fn no_read_verb_acts_over_the_revealing_state() {
    let fx = Fixture::build();
    let mut acted: Vec<String> = Vec::new();
    let mut admitted_rows: Vec<String> = Vec::new();

    for (path, argv) in INVOCATIONS {
        let label = path.join(" ");
        let tag = path.join("-");
        let workdir = TempDir::new(&format!("row-{tag}"));
        let repo = workdir.path().join("repo");
        let home = workdir.path().join("home");
        fs::create_dir_all(&home).expect("create the row home");
        copy_tree(&fx.clone, &repo);

        let args: Vec<String> = argv
            .iter()
            .map(|a| {
                if *a == TASK_MARKER {
                    ORDINARY_TASK.to_string()
                } else {
                    (*a).to_string()
                }
            })
            .collect();
        let argv_ref: Vec<&str> = args.iter().map(String::as_str).collect();

        let before = snapshot(&repo);
        let git_before = git_state(&repo);
        let out = run_jigc(&repo, &home, &argv_ref);
        let after = snapshot(&repo);
        let git_after = git_state(&repo);

        if git_before != git_after {
            acted.push(format!(
                "`jigc {label}` moved git's own state — a read verb touches neither the index \
                 nor the history:\n     before: {git_before}\n     after:  {git_after}",
            ));
        }

        let changed = differing(&before, &after);
        let (admitted, offending): (Vec<String>, Vec<String>) = changed
            .into_iter()
            .partition(|p| p.starts_with(DERIVED_CACHE_PREFIX));

        if !offending.is_empty() {
            acted.push(format!(
                "`jigc {label}` wrote {} path(s) outside `{DERIVED_CACHE_PREFIX}`: {}\n     \
                 (exit {:?}; stderr: {})",
                offending.len(),
                offending.join(", "),
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim(),
            ));
        }

        if !admitted.is_empty() {
            admitted_rows.push(label.clone());
            // The carve-out is admitted for what it IS, so both properties are asserted here.
            // (1) a pure function of the committed store at HEAD: the index it wrote carries
            //     this repo's current HEAD as its stamp.
            let head = git(&repo, &["rev-parse", "HEAD"]);
            let edges = repo.join(".jigc").join("index").join("edges.json");
            if edges.is_file() {
                let bytes = fs::read_to_string(&edges).expect("read the edge index");
                let parsed: serde_json::Value =
                    serde_json::from_str(&bytes).expect("the edge index is JSON");
                assert_eq!(
                    parsed.get("stamp").and_then(serde_json::Value::as_str),
                    Some(head.as_str()),
                    "`jigc {label}`'s derived cache must be stamped with the committed HEAD it \
                     was built from; got:\n{bytes}",
                );
            }
            // (2) a fixed point, not an accumulating write: a second run changes nothing at all.
            let again_before = snapshot(&repo);
            let _ = run_jigc(&repo, &home, &argv_ref);
            let again = differing(&again_before, &snapshot(&repo));
            assert!(
                again.is_empty(),
                "`jigc {label}` must be inert once its derived cache exists; the second run \
                 changed: {}",
                again.join(", "),
            );
        }
    }

    assert!(
        acted.is_empty(),
        "a `VerbKind::Read` leaf may report state and materialize the stamp-rebuildable \
         `{DERIVED_CACHE_PREFIX}` cache — nothing else:\n  - {}",
        acted.join("\n  - "),
    );
    assert!(
        !admitted_rows.is_empty(),
        "the `{DERIVED_CACHE_PREFIX}` carve-out must stay exercised — no row reached it, so the \
         admission is now dead and should be removed rather than carried",
    );
    // The record cannot source a minting workflow, so nothing may have invented one.
    assert_eq!(
        fx.recorded_override, OVERRIDE_WORKFLOW,
        "the origin's recorded provenance is what a re-seed would have overwritten with the \
         pack default",
    );
}

// =======================================================================================
// `milestone list-tasks` — the two behaviours the reseed was carrying, both read-only.
// =======================================================================================

/// A fresh clone still gets its answer: both sub-tasks, exit 0 — read out of the committed
/// record, with no workbench written for a read.
#[test]
fn list_tasks_serves_a_fresh_clone_from_the_record_without_writing() {
    let fx = Fixture::build();
    let workdir = TempDir::new("fresh-clone-answer");
    let repo = workdir.path().join("repo");
    let home = workdir.path().join("home");
    fs::create_dir_all(&home).expect("create the home");
    copy_tree(&fx.clone, &repo);

    let out = run_jigc(&repo, &home, &["milestone", "list-tasks", MILESTONE_ID]);
    assert_ok(&out, "`jigc milestone list-tasks` on a fresh clone");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains(DEFAULT_SUB) && stdout.contains(OVERRIDDEN_SUB),
        "the fresh-clone answer names both recorded sub-tasks; got:\n{stdout}",
    );
    assert!(
        stdout.contains("tasks (2)"),
        "the fresh-clone answer counts both recorded sub-tasks; got:\n{stdout}",
    );

    assert!(
        !repo.join(".jigc").join("milestones").exists(),
        "the read must not materialize the milestone cache",
    );
    for sub in [DEFAULT_SUB, OVERRIDDEN_SUB] {
        assert!(
            !repo.join(".jigc").join("tasks").join(sub).exists(),
            "the read must not rebuild `{sub}`'s working area — the area's `workflow` file is \
             authority for what a re-entry composes, and the record carries no workflow to \
             source it from (the origin recorded `{}`)",
            fx.recorded_override,
        );
    }
}

/// And a milestone that is over is still refused — `terminal_status` is a pure parse of the
/// committed record, so the refusal survives the read-only rewrite intact.
#[test]
fn list_tasks_still_refuses_a_settled_milestone() {
    let origin_dir = TempDir::new("settled-origin");
    let home = TempDir::new("settled-home");
    let origin = origin_dir.path().to_path_buf();
    init_methodology_origin(&origin);

    assert_ok(
        &run_jigc(
            &origin,
            home.path(),
            &["milestone", "create", "Cache rework"],
        ),
        "`jigc milestone create`",
    );
    assert_ok(
        &run_jigc(
            &origin,
            home.path(),
            &["milestone", "add-task", MILESTONE_ID, "Warm the read cache"],
        ),
        "`jigc milestone add-task`",
    );
    assert_ok(
        &run_jigc(
            &origin,
            home.path(),
            &["milestone", "discard", MILESTONE_ID, "--force"],
        ),
        "`jigc milestone discard` (settle the record)",
    );

    let clone_dir = TempDir::new("settled-clone");
    let clone = clone_dir.path().join("clone");
    git(
        clone_dir.path(),
        &[
            "clone",
            "-q",
            &origin.display().to_string(),
            &clone.display().to_string(),
        ],
    );

    let out = run_jigc(
        &clone,
        home.path(),
        &["milestone", "list-tasks", MILESTONE_ID],
    );
    assert!(
        !out.status.success(),
        "a settled milestone is over — `list-tasks` must refuse it; got exit 0\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        stderr.contains("milestone.terminal") && stderr.contains("discarded"),
        "the refusal is `milestone.terminal`, naming the terminal; got:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("jigc doc show milestone-record:{MILESTONE_ID}")),
        "the refusal routes to the read surface that DOES serve a settled record; got:\n{stderr}",
    );
    assert!(
        !clone.join(".jigc").join("milestones").exists(),
        "and no workbench is materialized for the settled milestone",
    );
}
