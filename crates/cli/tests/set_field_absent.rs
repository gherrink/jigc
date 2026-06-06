//! Binary-level integration test for `jigc doc set-field` populating an **absent**
//! optional header field in-place — the M10 Increment 1, T4 done-criterion
//! (`design/write-commands.md` → set-field; `design/parsing.md` → Absent structural
//! homes; `DECISIONS.md` 2026-06-06 → M10 Increment 1 planning grounding (c)).
//!
//! VERIFICATION-FIRST: `set_field_validated` already routes an absent optional field
//! to `insert_front_matter_field` (proven for `commit.implements` in the engine unit
//! test `absent_front_matter_ref_materializes_in_schema_order`), and the CLI verb
//! already calls it. This test drives the **adr/`status`** shape through the real
//! binary — the header id (`status`) differs from `commit`'s (`header`), and
//! `cites-code` is the new T3 pack-declared `code-anchor` field — so the path is
//! *proven*, never assumed:
//!
//! - (a) `jigc doc set-field adr:<slug>#status/cites-code --value <path>#<symbol>`
//!   exits zero against an ADR with no `cites-code` line;
//! - (b) exactly one `cites-code:` line is inserted, in schema order inside the
//!   `status` front-matter (after `date`/`supersedes`, before the `---` fence);
//! - (c) the edited buffer re-parses against the `adr` schema (the verb's own
//!   `validate_after` re-parse gate enforces this — a non-reparseable result would
//!   have blocked with a non-zero exit, so exit-zero is the proof; we additionally
//!   re-run the binary to read the field back as a structural confirmation);
//! - (d) a malformed value (control char) is blocked with `write.malformed-value`
//!   and nothing is persisted (no `cites-code:` line left behind).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-set-field-absent-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer,
/// `jigc start --workflow single-task` to mint a task, then `jigc doc create adr` so
/// a freshly-created ADR (with NO `cites-code` line) is staged in the task area.
/// Returns (repo, home, task-id, adr-slug).
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
        &["start", "--workflow", "single-task", "wire cites-code"],
    );
    assert!(
        started.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&started.stderr)
    );

    let task = "wire-cites-code";
    let created = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Anchor decision",
            "--task",
            task,
        ],
    );
    assert!(
        created.status.success(),
        "`jigc doc create adr` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr)
    );
    (repo, home, task, "anchor-decision")
}

/// The staged ADR instance path in the task's `docs/` area.
fn staged_adr(repo: &Path, task: &str, slug: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"))
}

#[test]
fn set_field_populates_absent_cites_code_in_schema_order_and_reparses() {
    let (repo, home, task, slug) = repo_with_created_adr();
    let staged = staged_adr(repo.path(), task, slug);

    // Precondition: the freshly-created ADR carries NO `cites-code` line.
    let before = fs::read_to_string(&staged).expect("read staged adr");
    assert!(
        !before.contains("cites-code"),
        "the freshly-created ADR must have no `cites-code` line; got:\n{before}"
    );

    // Populate an EARLIER-declared header field (`status`, the first field in the
    // `status` section) first, so the schema-order assertion is meaningful: a
    // schema-ordered insert must place `cites-code` (the LAST declared field) AFTER
    // `status`, not merely "somewhere in the fence". A naive head-of-fence insert
    // would fail this.
    let set_status = run(
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
    assert!(
        set_status.status.success(),
        "`set-field ...#status/status accepted` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&set_status.stderr)
    );

    // (a) `set-field` of the ABSENT optional `cites-code` header field exits zero.
    let value = "src/engine/write.rs#set_field_validated";
    let out = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/cites-code"),
            "--value",
            value,
            "--task",
            task,
        ],
    );
    assert!(
        out.status.success(),
        "`set-field ...#status/cites-code` of an absent optional field must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // (b) Exactly one `cites-code:` line, in schema order inside the `status`
    // front-matter (the header section is the `---`-fenced block; `cites-code` is
    // the last `status`-section field, so it lands after the present `status:` line
    // and before the closing `---`).
    let after = fs::read_to_string(&staged).expect("read staged adr after set-field");
    let occurrences = after.matches("cites-code:").count();
    assert_eq!(
        occurrences, 1,
        "exactly one `cites-code:` line must be inserted; got {occurrences} in:\n{after}"
    );
    assert!(
        after.contains(&format!("cites-code: {value}")),
        "the inserted line must carry the value; got:\n{after}"
    );
    // Schema order + front-matter containment: the `cites-code:` line sits between
    // the opening and closing `---` fences, and AFTER the earlier-declared `status:`
    // line (so the insert is schema-ordered, not head-of-fence).
    let fence_close = after
        .match_indices("---")
        .nth(1)
        .map(|(i, _)| i)
        .expect("the ADR has a closing front-matter fence");
    let cites_at = after.find("cites-code:").expect("cites-code line present");
    let status_at = after.find("status:").expect("status line present");
    assert!(
        cites_at < fence_close,
        "the `cites-code:` line must sit inside the front-matter (before the closing fence); got:\n{after}"
    );
    assert!(
        status_at < cites_at,
        "the `cites-code:` line must land after `status:` in schema order; got:\n{after}"
    );

    // (c) The edited buffer re-parses against the adr schema. The verb's own
    // `validate_after` re-parse gate already enforces this (exit-zero above is the
    // proof). Confirm structurally by reading the field back through the binary: a
    // non-reparseable buffer would not yield the value on a subsequent set-field
    // that re-edits the now-present field.
    let reedit = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/cites-code"),
            "--value",
            "src/engine/schema.rs#FieldType",
            "--task",
            task,
        ],
    );
    assert!(
        reedit.status.success(),
        "re-editing the now-present `cites-code` field must exit 0 (the buffer re-parsed); stderr:\n{}",
        String::from_utf8_lossy(&reedit.stderr)
    );
    let reedited = fs::read_to_string(&staged).expect("read staged adr after re-edit");
    assert_eq!(
        reedited.matches("cites-code:").count(),
        1,
        "the re-edit must surgically replace the present field, not insert a second; got:\n{reedited}"
    );
    assert!(
        reedited.contains("cites-code: src/engine/schema.rs#FieldType"),
        "the re-edit must carry the new value; got:\n{reedited}"
    );
}

#[test]
fn malformed_cites_code_value_blocks_and_persists_nothing() {
    let (repo, home, task, slug) = repo_with_created_adr();
    let staged = staged_adr(repo.path(), task, slug);

    // (d) A malformed value — a control char (a tab) — fails the `code-anchor`
    // write-time shape check and blocks with `write.malformed-value`. Nothing is
    // persisted: no `cites-code:` line is left behind.
    let out = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("adr:{slug}#status/cites-code"),
            "--value",
            "src/engine/write.rs#bad\tsymbol",
            "--task",
            task,
            "--format",
            "json",
        ],
    );
    assert!(
        !out.status.success(),
        "a control-char `code-anchor` value must block (non-zero exit)"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("write.malformed-value"),
        "the block must carry the `write.malformed-value` code; got:\n{stderr}"
    );

    let after = fs::read_to_string(&staged).expect("read staged adr after rejected write");
    assert!(
        !after.contains("cites-code"),
        "a rejected write must persist nothing — no `cites-code` line; got:\n{after}"
    );
}
