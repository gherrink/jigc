//! M24 Increment 6, T2 — the migration finalize `git add` is **narrowed** to the
//! migration's own changes: the promoted canonical doc, the retired original's deletion,
//! and jigc's git-tracked config layer (`.jigc/config/`, `.jigc/.gitignore`) — **never**
//! arbitrary user WIP ([auto-migration.md](../../../design/auto-migration.md) →
//! Hardening #9a/#9; `DECISIONS.md` B2).
//!
//! `setup` writes the `.jigc/config/` cascade layer but never commits, so the first
//! migration commit is what lands it — hence it must be in the narrowed set (B2). A
//! migration touches no code, so the {promote + retire + jigc-config} set is complete.
//!
//! Red before the fix: the blanket `git add --all` swept an unrelated untracked file
//! (`scratch.txt`) into the migration commit. The assertions inspect the **landed** git
//! commit (its tree + its name-status delta + the post-commit `git status`), never a
//! reconstructed equivalent — the committed bytes are the contract.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-migrate-gitadd-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`,
/// optionally piping `stdin`.
fn run_jigc(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
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

/// Assert a `jigc` invocation exits 0, returning its trimmed stdout.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// The off-router migration task id — `migrate` mints a per-file `migrate-<doctype>-<slug(path)>`.
const TASK: &str = "migrate-changelog-history";

/// The foreign Keep-a-Changelog file (committed, so its retire lands a tracked deletion).
const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

/// The declarative whole-doc payload that re-authors the canonical changelog in ONE
/// `doc author --from-file-file` batch — no commit doc leaf anywhere.
const PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 0.1.0
        set:
          date: 2021-03-09
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- First public release.>>"
"#;

#[test]
fn migration_finalize_stages_only_its_own_changes_not_user_wip() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // Track the foreign original so its retirement lands as a real (staged) deletion.
    fs::write(repo.path().join("HISTORY.md"), FOREIGN).expect("write foreign HISTORY.md");
    git(repo.path(), &["add", "HISTORY.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track foreign changelog"],
    );

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // `setup` now commits its own install (M26), so the `.jigc/config/` layer is already
    // tracked at HEAD going into the migration — confirm it, so the tree assertions below
    // read against the cumulative HEAD tree (the migration commit no longer lands it).
    let tracked_before = git(repo.path(), &["ls-tree", "-r", "--name-only", "HEAD"]);
    assert!(
        tracked_before
            .lines()
            .any(|l| l.starts_with(".jigc/config/")),
        "precondition: `setup` commits the `.jigc/config/` layer (M26); tree:\n{tracked_before}",
    );

    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["migrate", "HISTORY.md", "--as", "changelog"],
            None,
        ),
        "jigc migrate",
    );

    // An unrelated untracked file the user happened to leave in the tree — it must NOT be
    // swept into the migration commit.
    fs::write(repo.path().join("scratch.txt"), "private WIP\n").expect("write scratch.txt");

    // Author the canonical changelog in ONE declarative batch — nothing against the
    // commit doc (it is auto-provisioned filled, T1).
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "author",
                "changelog",
                "--from-file",
                "-",
                "--task",
                TASK,
            ],
            Some(PAYLOAD.as_bytes()),
        ),
        "jigc doc author changelog --from-file -",
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
        None,
    );
    assert!(
        out.status.success(),
        "migration finalize --approve must land (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // The landed commit's tree: it CONTAINS the promoted canonical doc, the jigc-config
    // layer, and `.jigc/.gitignore`.
    let tree = git(repo.path(), &["ls-tree", "-r", "--name-only", "HEAD"]);
    let tree_paths: Vec<&str> = tree.lines().collect();
    assert!(
        tree_paths.contains(&"CHANGELOG.md"),
        "the migration commit promotes `CHANGELOG.md`; tree:\n{tree}",
    );
    assert!(
        tree_paths.iter().any(|p| p.starts_with(".jigc/config/")),
        "the migration commit lands the jigc-tracked config layer `.jigc/config/`; tree:\n{tree}",
    );
    assert!(
        tree_paths.contains(&".jigc/.gitignore"),
        "the migration commit lands `.jigc/.gitignore`; tree:\n{tree}",
    );

    // It STAGES the retire deletion of the foreign original: gone from the tree, and the
    // commit's own delta records it as a deletion (`D`).
    assert!(
        !tree_paths.contains(&"HISTORY.md"),
        "the migration commit retires the foreign `HISTORY.md` (absent from the tree); tree:\n{tree}",
    );
    let delta = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        delta
            .lines()
            .any(|l| l.starts_with('D') && l.ends_with("HISTORY.md")),
        "the migration commit's delta records the foreign original as a deletion; delta:\n{delta}",
    );

    // It does NOT contain the unrelated untracked file — and that file is STILL untracked
    // in `git status` after the commit (it was never swept in).
    assert!(
        !tree_paths.contains(&"scratch.txt"),
        "the unrelated `scratch.txt` is NOT in the migration commit; tree:\n{tree}",
    );
    let status = git(repo.path(), &["status", "--porcelain"]);
    assert!(
        status.lines().any(|l| l == "?? scratch.txt"),
        "`scratch.txt` is still untracked after the migration commit; status:\n{status}",
    );
}
