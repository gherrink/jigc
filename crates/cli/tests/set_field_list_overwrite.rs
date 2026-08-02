//! Binary-level integration test for `jigc doc set-field` on a **list-cardinality**
//! (`0..*`) ref — the accumulate-or-block fix for the NGT-surfaced silent
//! last-write-wins footgun (`implementation/decisions-pending.md` → sub-part (b);
//! `DECISIONS.md` 2026-06-20).
//!
//! The defect: a single-value `set-field` onto a `0..*` ref that ALREADY carries
//! value(s) surgically replaces the whole list — silently dropping the prior
//! entries (no error, no warning, exit 0). The agent who calls `set-field` once per
//! value, per the composed guidance, keeps only the LAST value, and the dropped
//! edges never enter the forward-ref edge index. The only correct multi-value path
//! is the inline-list `--value "[a, b]"` form in ONE call.
//!
//! The fix (block-on-overwrite): a single-value `set-field` onto a populated `0..*`
//! ref is REJECTED with `write.list-overwrite` naming the field and showing the
//! inline-list form — the loss can no longer be silent. The explicit whole-list
//! replace via the bracket form `[a, b]` stays allowed (the blessed re-point/replace
//! idiom). A scalar / `0..1` ref re-set, and the first set on an empty `0..*` ref,
//! are unaffected.
//!
//! Cases:
//! - (a) two single-value `set-field`s on `adr.supersedes` (`0..*`): the first
//!   exits 0; the SECOND is blocked (non-zero, `write.list-overwrite`, names
//!   `supersedes`, shows the inline-list form), and the first value survives intact;
//! - (b) the inline-list `--value "[adr:a, adr:b]"` form sets BOTH in one call;
//! - (c) an explicit bracket-list replace on a populated `0..*` ref is allowed;
//! - (d) a scalar `0..1`-cardinality field re-set (`status`) still works unchanged.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-set-field-list-{tag}-{}-{:?}",
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

/// Initialize a real git repo with the `.jigc/config/` project layer, `jigc start`
/// a task, then `jigc doc create adr` so a freshly-created ADR (with NO `supersedes`
/// line) is staged in the task area. Returns (repo, home, task-id, adr-slug).
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
        &["start", "--workflow", "single-task", "supersede decisions"],
    );
    assert!(
        started.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&started.stderr)
    );

    let task = "supersede-decisions";
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

/// `jigc doc set-field adr:<slug>#status/supersedes --value <value>`.
fn set_supersedes(
    repo: &Path,
    home: &Path,
    task: &str,
    slug: &str,
    value: &str,
) -> std::process::Output {
    run(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/supersedes"),
            "--value",
            value,
            "--task",
            task,
            "--format",
            "json",
        ],
    )
}

#[test]
fn second_single_value_set_on_populated_list_ref_is_blocked() {
    let (repo, home, task, slug) = repo_with_created_adr();
    let staged = staged_adr(repo.path(), task, slug);

    // (a) First single-value set on the EMPTY `0..*` ref — exits 0 (the absent-field
    // generate path).
    let first = set_supersedes(repo.path(), home.path(), task, slug, "adr:alpha");
    assert!(
        first.status.success(),
        "the first `set-field` on an empty `0..*` ref must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let after_first = fs::read_to_string(&staged).expect("read staged adr after first set");
    assert!(
        after_first.contains("supersedes: adr:alpha"),
        "the first value must be present; got:\n{after_first}"
    );

    // The SECOND single-value set onto the now-populated `0..*` ref must be BLOCKED —
    // it would surgically replace the whole list and silently drop `adr:alpha`.
    let second = set_supersedes(repo.path(), home.path(), task, slug, "adr:beta");
    assert!(
        !second.status.success(),
        "a single-value `set-field` onto a populated `0..*` ref must block (non-zero exit)"
    );
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(
        stderr.contains("write.list-overwrite"),
        "the block must carry the `write.list-overwrite` code; got:\n{stderr}"
    );
    assert!(
        stderr.contains("supersedes"),
        "the block must name the offending field; got:\n{stderr}"
    );
    // The inline-list form must be surfaced (discoverability) — the bracket syntax.
    assert!(
        stderr.contains("[adr:alpha, adr:beta]") || stderr.contains("[adr:alpha,adr:beta]"),
        "the block must show the inline-list form combining existing + new; got:\n{stderr}"
    );

    // The first value SURVIVES — nothing was dropped or replaced by the rejected write.
    let after_second = fs::read_to_string(&staged).expect("read staged adr after rejected set");
    assert!(
        after_second.contains("supersedes: adr:alpha"),
        "a rejected overwrite must preserve the prior value; got:\n{after_second}"
    );
    assert!(
        !after_second.contains("adr:beta"),
        "a rejected overwrite must persist nothing of the new value; got:\n{after_second}"
    );
}

#[test]
fn inline_list_form_sets_both_values_in_one_call() {
    let (repo, home, task, slug) = repo_with_created_adr();
    let staged = staged_adr(repo.path(), task, slug);

    // (b) The inline-list `[a, b]` form sets BOTH values in one call (the correct
    // multi-value path) — on the empty ref.
    let out = set_supersedes(
        repo.path(),
        home.path(),
        task,
        slug,
        "[adr:alpha, adr:beta]",
    );
    assert!(
        out.status.success(),
        "the inline-list `set-field` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let after = fs::read_to_string(&staged).expect("read staged adr");
    assert!(
        after.contains("supersedes: [adr:alpha, adr:beta]"),
        "both values must be set in one call; got:\n{after}"
    );
}

#[test]
fn explicit_bracket_replace_on_populated_list_ref_is_allowed() {
    let (repo, home, task, slug) = repo_with_created_adr();
    let staged = staged_adr(repo.path(), task, slug);

    // First set a single value.
    let first = set_supersedes(repo.path(), home.path(), task, slug, "adr:alpha");
    assert!(first.status.success(), "first set must exit 0");

    // (c) An EXPLICIT bracket-list replace on the populated ref is allowed (the
    // blessed re-point/replace idiom — the whole list is named, no silent loss).
    let out = set_supersedes(
        repo.path(),
        home.path(),
        task,
        slug,
        "[adr:beta, adr:gamma]",
    );
    assert!(
        out.status.success(),
        "an explicit bracket-list replace must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let after = fs::read_to_string(&staged).expect("read staged adr");
    assert!(
        after.contains("supersedes: [adr:beta, adr:gamma]"),
        "the explicit replace must set the named list; got:\n{after}"
    );
    assert!(
        !after.contains("adr:alpha"),
        "the explicit replace replaces the whole list; got:\n{after}"
    );
}

#[test]
fn scalar_field_reset_is_unaffected() {
    let (repo, home, task, slug) = repo_with_created_adr();

    // (d) A scalar / `0..1`-cardinality field (`status`, an enum) re-set still works
    // — the list-overwrite guard is `0..*`-ref only.
    let first = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/status"),
            "--value",
            "accepted",
            "--task",
            task,
        ],
    );
    assert!(first.status.success(), "first scalar set must exit 0");
    let second = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/status"),
            "--value",
            "superseded",
            "--task",
            task,
        ],
    );
    assert!(
        second.status.success(),
        "re-setting a scalar field must exit 0 (replace semantics unchanged); stderr:\n{}",
        String::from_utf8_lossy(&second.stderr)
    );
}
