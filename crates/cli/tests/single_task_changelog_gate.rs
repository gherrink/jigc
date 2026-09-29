//! M22 Increment 5, T2 — the `single-task` fold-in: `{type: changelog, as: change}`
//! beside the existing `{type: adr, as: decision}` in `single-task`'s `allows-create`.
//!
//! Over a `git init` temp repo against the **shipped** dev pack (selected via
//! `JIGC_PACK_DIR` = the embedded `pack/` tree, so it is the bytes that ship), prove:
//!   - `jigc start` (the cascade default `single-task`) mints a task;
//!   - `jigc doc create changelog` is **admitted** through `single-task`'s create-gate
//!     (the singleton mints at the fixed slug, bound to `task.change`), and an
//!     unreleased change-group authored via `add-item changelog:changelog#unreleased-changes`
//!     round-trips byte-stable;
//!   - the gate is **not widened to admit everything** — `jigc doc create spec` (a
//!     shipped doctype NOT in `single-task`'s gate) is still REFUSED (the M4
//!     omitting-context discipline: prove admission AND continued refusal).
//!
//! The existing `single-task` ADR-create + superseding-decision acceptance
//! (`superseding_decision.rs`, `finalize_to_git.rs`) stand UNCHANGED as the
//! regression watch; the flow-24 bar-5 end-to-end (the fold-in appending an
//! unreleased entry through finalize) rides in T3.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-st-changelog-gate-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
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
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

/// Run a `jigc` subcommand piping `stdin`.
fn run_jigc_stdin(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: &[u8],
) -> std::process::Output {
    use std::io::Write;
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
}

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both
/// streams.
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

/// The staged `changelog:changelog` instance in the task working area.
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The shipped changelog schema, loaded for the round-trip assertion.
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped changelog schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

#[test]
fn single_task_gate_admits_changelog_and_still_refuses_a_non_gated_type() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "jigc setup",
    );

    // Start `single-task` explicitly (the cascade default is the `router`, which is
    // `creates-task: false`; `single-task` is the task-minting work-workflow whose
    // gate this task exercises).
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "start",
                "--workflow",
                "single-task",
                "add an OAuth login button",
            ],
        ),
        "jigc start --workflow single-task",
    );
    let task = "add-an-oauth-login-button";

    // ADMITTED: `jigc doc create changelog` passes single-task's create-gate
    // (bound to `task.change`). The singleton mints at the fixed slug = the type id.
    let created = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["doc", "create", "changelog", "--title", "Changelog"],
        ),
        "jigc doc create changelog (single-task gate)",
    );
    assert_eq!(
        created, "changelog:changelog",
        "the singleton mints at the fixed slug = the type id",
    );

    // Author an unreleased change-group through the multi-word section, then its notes.
    let group = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#unreleased-changes",
                "--title",
                "Added",
            ],
        ),
        "add-item unreleased #added",
    );
    assert_eq!(group, "changelog:changelog#unreleased-changes/added");
    ok_stdout(
        run_jigc_stdin(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
            ],
            b"OAuth login button on the sign-in page.\n",
        ),
        "set-slot unreleased #added notes",
    );

    // The staged doc round-trips byte-stable.
    let staged = staged_changelog(repo.path(), task);
    let schema = shipped_changelog_schema(&pack);
    let parsed =
        engine::write::instance_from_source(&schema, &staged).expect("staged changelog re-parses");
    assert!(
        staged.contains("## Unreleased Changes"),
        "the multi-word section heading is intact; staged:\n{staged}",
    );
    assert!(
        staged.contains("OAuth login button on the sign-in page."),
        "the authored notes are present; staged:\n{staged}",
    );
    assert_eq!(
        engine::write::render(&schema, &parsed),
        staged,
        "the fold-in-authored changelog is byte-stable across parse → render",
    );

    // REFUSED: the gate is NOT widened to admit everything. `spec` is a shipped
    // doctype that single-task's `allows-create` does NOT list, so the create-gate
    // blocks it (the M4 omitting-context discipline — admission must not bleed into
    // every doctype).
    let refused = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["doc", "create", "spec", "--title", "Auth flow"],
    );
    assert!(
        !refused.status.success(),
        "`jigc doc create spec` must be REFUSED by single-task's gate; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
}
