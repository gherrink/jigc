//! M24 Increment 6, T1 — the migration commit doc is **auto-provisioned filled**, so
//! a migration finalizes with **no manual commit authoring**.
//!
//! On a migration task ([auto-migration.md](../../../design/auto-migration.md) →
//! Hardening #4), `jigc migrate` provisions the transient `commit:<id>` doc already
//! filled: `type`/`scope` as field values (`docs` / the doctype) and `summary`/`body`
//! as templated slot prose that is a pure deterministic function of (source path +
//! doctype) — the bounded slot-fill rule (`DECISIONS.md` S2). So the agent authors only
//! the canonical changelog (one `doc author --from` batch) and never touches the commit
//! doc; `jigc task finalize --approve` lands the formulaic `docs(changelog): …` commit.
//!
//! Red before the fix: with no commit authoring the migration mint provisioned an
//! **empty** fillable form, so finalize blocked on the empty author-required `type`
//! field + the empty `summary` slot. The subject is asserted on the **landed** git
//! commit (`git log -1 --format=%s`), never a reconstructed equivalent — the emitted
//! bytes are the contract.

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
            "jigc-migrate-commit-{tag}-{}-{:?}",
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
        .trim()
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

/// The off-router migration task id — `migrate` mints `migrate-<doctype>`.
const TASK: &str = "migrate-changelog";

/// The foreign Keep-a-Changelog file (committed, so its retire lands a tracked deletion).
const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

/// The declarative whole-doc payload that re-authors the canonical changelog in ONE
/// `doc author --from` batch — no commit doc leaf anywhere.
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
fn migration_finalizes_with_autoprovisioned_commit_doc() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // Track the foreign original so its retirement lands as a real deletion.
    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign CHANGELOG.md");
    git(repo.path(), &["add", "CHANGELOG.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track foreign changelog"],
    );

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["migrate", "CHANGELOG.md", "--as", "changelog"],
            None,
        ),
        "jigc migrate",
    );

    // Author the canonical changelog in ONE declarative batch — and NOTHING against the
    // commit doc (no `set-field`/`set-slot` against `commit:<id>`).
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["doc", "author", "changelog", "--from", "-", "--task", TASK],
            Some(PAYLOAD.as_bytes()),
        ),
        "jigc doc author changelog --from -",
    );

    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    // Finalize with NO manual commit authoring — the auto-provisioned filled commit doc
    // carries the formulaic message, so this lands clean (red before the fix: the empty
    // author-required `type` field + `summary` slot blocked finalize).
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
        None,
    );
    assert!(
        out.status.success(),
        "finalize --approve on a migration with NO commit authoring must land (exit 0); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // Exactly ONE new commit (the all-or-nothing transaction).
    let count_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        count_after,
        count_before + 1,
        "the approved migration lands exactly ONE commit"
    );

    // The LANDED commit subject is the formulaic `docs(changelog): <templated summary>`
    // — asserted on the emitted bytes (`git log`), never a reconstructed equivalent.
    let subject = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert_eq!(
        subject, "docs(changelog): adopt CHANGELOG.md as a managed changelog",
        "the auto-provisioned migration commit lands the formulaic subject",
    );
    // The body is present (the templated `body` slot).
    let body = git(repo.path(), &["log", "-1", "--format=%b"]);
    assert!(
        !body.trim().is_empty(),
        "the auto-provisioned migration commit carries a non-empty body; got: {body:?}",
    );
}
