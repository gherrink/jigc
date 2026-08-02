//! Binary-level integration test for the **N2 `[…]` non-ref-scalar reject** — the
//! ref-clear idiom's silent footgun (`DECISIONS.md` 2026-07-11 M41 Increment 7;
//! `design/write-commands.md` → the `[]` reject).
//!
//! The defect: `--value "[]"` is the clear idiom for a **list-cardinality** (`0..*`)
//! ref, but on a **non-ref** scalar (`string` / `code-anchor` / …) the CLI happily
//! spliced the literal `[]` (or `[a, b]`) as the value — silent garbage, no error.
//!
//! The fix: a `[…]`-wrapped value on a non-ref scalar is REJECTED at the write-time
//! floor (`write.malformed-value`), routed to the real clear verb (`--unset`); the
//! prior value is untouched. The `ref` accept-path is unaffected — a `0..*` ref still
//! clears via `[]` — the in-task `check_ref` reorder.
//!
//! Cases:
//! - (a) a valid `code-anchor` (`adr.cites-code`) then `--value "[]"`: rejected
//!   (`write.malformed-value`, route names `--unset`), the prior anchor survives;
//! - (b) a `0..*` ref (`adr.supersedes`) carrying a value clears via `--value "[]"`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-set-field-bracket-{tag}-{}-{:?}",
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

/// Run `git` in `repo`, asserting success.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    command.output().expect("run the jigc binary")
}

/// Initialize a real git repo + project layer, `jigc start` a task, `jigc doc create
/// adr` so a freshly-created ADR is staged. Returns (repo, home, task-id, adr-slug).
fn repo_with_created_adr() -> (TempDir, TempDir, &'static str, &'static str) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let started = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "clear a field"],
    );
    assert!(
        started.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&started.stderr)
    );

    let task = "clear-a-field";
    let created = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "New decision",
            "--task",
            task,
        ],
    );
    assert!(
        created.status.success(),
        "`jigc doc create adr` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr)
    );
    (repo, home, task, "new-decision")
}

/// The staged ADR instance path in the task's `docs/` area.
fn staged_adr(repo: &Path, task: &str, slug: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"))
}

/// `jigc doc set-field adr:<slug>#<leaf> --value <value>`.
fn set_field(
    repo: &Path,
    home: &Path,
    task: &str,
    slug: &str,
    leaf: &str,
    value: &str,
) -> std::process::Output {
    run(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#{leaf}"),
            "--value",
            value,
            "--task",
            task,
        ],
    )
}

#[test]
fn bracket_value_on_code_anchor_is_rejected_and_prior_value_survives() {
    let (repo, home, task, slug) = repo_with_created_adr();
    let staged = staged_adr(repo.path(), task, slug);

    // Seed a valid `code-anchor` value (shape-only write-time check — the path need not
    // exist; the real adjudication is the finalize `doc-code` probe).
    let seed = set_field(
        repo.path(),
        home.path(),
        task,
        slug,
        "status/cites-code",
        "src/foo.rs#bar",
    );
    assert!(
        seed.status.success(),
        "seeding a valid code-anchor must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&seed.stderr)
    );
    let seeded = fs::read_to_string(&staged).expect("read staged adr after seed");
    assert!(
        seeded.contains("cites-code: src/foo.rs#bar"),
        "the seed value must be present; got:\n{seeded}"
    );

    // (a) `--value "[]"` on the non-ref code-anchor is REJECTED — it is the ref-clear
    // idiom misapplied; splicing it would write literal `[]` garbage.
    let rejected = set_field(
        repo.path(),
        home.path(),
        task,
        slug,
        "status/cites-code",
        "[]",
    );
    assert!(
        !rejected.status.success(),
        "`--value \"[]\"` on a non-ref scalar must block (non-zero exit)"
    );
    let stderr = String::from_utf8_lossy(&rejected.stderr);
    assert!(
        stderr.contains("write.malformed-value"),
        "the block must carry the `write.malformed-value` code; got:\n{stderr}"
    );
    assert!(
        stderr.contains("jigc doc set-field <addr> --unset"),
        "the reject must route to the real clear verb `--unset`; got:\n{stderr}"
    );

    // The prior anchor value SURVIVES — nothing was replaced by the rejected write.
    let after = fs::read_to_string(&staged).expect("read staged adr after rejected set");
    assert!(
        after.contains("cites-code: src/foo.rs#bar"),
        "a rejected write must preserve the prior value; got:\n{after}"
    );
    assert!(
        !after.contains("cites-code: []"),
        "the literal `[]` garbage must never reach disk; got:\n{after}"
    );
}

#[test]
fn empty_bracket_still_clears_a_list_ref() {
    let (repo, home, task, slug) = repo_with_created_adr();
    let staged = staged_adr(repo.path(), task, slug);

    // Seed the `0..*` ref with a value.
    let seed = set_field(
        repo.path(),
        home.path(),
        task,
        slug,
        "status/supersedes",
        "adr:alpha",
    );
    assert!(
        seed.status.success(),
        "seeding the list ref must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&seed.stderr)
    );

    // (b) `--value "[]"` on the `0..*` ref CLEARS it — the accept-path the `check_ref`
    // reorder preserves.
    let cleared = set_field(
        repo.path(),
        home.path(),
        task,
        slug,
        "status/supersedes",
        "[]",
    );
    assert!(
        cleared.status.success(),
        "`--value \"[]\"` on a `0..*` ref must clear it (exit 0); stderr:\n{}",
        String::from_utf8_lossy(&cleared.stderr)
    );
    let after = fs::read_to_string(&staged).expect("read staged adr after clear");
    assert!(
        !after.contains("adr:alpha"),
        "the cleared value must be gone; got:\n{after}"
    );
}
