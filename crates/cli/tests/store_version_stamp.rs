//! Acceptance — the M36 **binary-provenance stamp** (`design/storage.md` → Store
//! provenance; `design/validation.md` → un-keyed findings / Exit semantics).
//!
//! `jigc setup` writes a **committed** `.jigc/version` recording which `jigc` build wrote
//! the store (the running `CARGO_PKG_VERSION`, one line `jigc-version: <semver>`), so the
//! stamp travels with the repo to a clone. The store-scope `jigc validate` sweep compares
//! the recorded stamp to the running binary and, on a mismatch, emits a **report-only,
//! un-keyed** `store-version.binary-mismatch` advisory at exit 0 — it never gates. A
//! matching stamp and an **absent** stamp (a pre-M36 hand-built store, never false-flagged)
//! each emit nothing.
//!
//! This drives the built `jigc` binary end-to-end (the invocation-path-masking lesson:
//! drive the bytes an operator would actually run) with **no** `JIGC_PACK_DIR` — the real
//! embedded pack. The probe is the real `doc-code` (so the `jigc validate` pre-flight
//! resolves) and a self-cleaning `TempDir` keeps the test off the developer's repo.

use crate::support::run_then_parse::stdout_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-version-stamp-{tag}-{}-{:?}",
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

/// The running build's version — the value `jigc setup` stamps and `jigc validate`
/// compares against. The test crate is part of the `cli` crate's build, so this is the
/// workspace version (`1.0.0-rc.2`).
const RUNNING_VERSION: &str = env!("CARGO_PKG_VERSION");

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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe
/// resolved through the production path (no `JIGC_DOC_CODE_PROBE` override).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Make `root` a real git repo with identity, then run `jigc setup` over it.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// `jigc setup` writes a `.jigc/version` stamp recording the running build in the
/// `jigc-version: <semver>` one-line format, and **commits** it (present in the tree, so
/// it travels to a clone).
#[test]
fn setup_writes_committed_binary_provenance_stamp() {
    let repo = TempDir::new("write");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    let stamp = repo.path().join(".jigc").join("version");
    let body = fs::read_to_string(&stamp).expect("`jigc setup` must write `.jigc/version`");
    assert_eq!(
        body,
        format!("jigc-version: {RUNNING_VERSION}\n"),
        "the stamp records the running build in the one-line `jigc-version: <semver>` format",
    );

    // Committed — present in the tree (so a clone carries it). `git ls-files` lists tracked
    // paths; the stamp must be among them.
    let tracked = git(repo.path(), &["ls-files"]);
    assert!(
        tracked.lines().any(|p| p == ".jigc/version"),
        "`.jigc/version` must be committed (tracked), so it travels with the repo; tracked:\n{tracked}",
    );
}

/// `jigc validate` over a store whose stamp records a **different** version emits a
/// report-only, un-keyed `store-version.binary-mismatch` advisory naming both versions,
/// and exits **0** (it never gates).
#[test]
fn validate_advisory_on_binary_mismatch_exits_zero() {
    let repo = TempDir::new("mismatch");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // Rewrite the committed stamp to a different build (a store written elsewhere).
    let stamp = repo.path().join(".jigc").join("version");
    let other = "0.9.0-elsewhere";
    fs::write(&stamp, format!("jigc-version: {other}\n")).expect("rewrite the stamp");

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a binary-mismatch advisory must NOT gate `jigc validate` (exit 0); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        count(&stdout, "store-version.binary-mismatch"),
        1,
        "a store stamped by a different build must surface EXACTLY ONE \
         `store-version.binary-mismatch` advisory; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("advisory"),
        "the finding must be advisory severity; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains(other) && stdout.contains(RUNNING_VERSION),
        "the finding must name both the recorded ({other}) and running ({RUNNING_VERSION}) \
         versions; stdout:\n{stdout}",
    );
}

/// The advisory-route floor (`design/validation.md` → The advisory-route floor): a
/// `store-version.binary-mismatch` advisory is a `Severity::Advisory` finding, so it is
/// inside the floor and MUST carry a non-null `route` on the pinned `--format json`
/// surface. Its message names a real action (align versions / re-run `jigc setup`), so the
/// route is a repair route, never `null`.
#[test]
fn binary_mismatch_advisory_carries_a_route() {
    let repo = TempDir::new("mismatch-route");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    let stamp = repo.path().join(".jigc").join("version");
    fs::write(&stamp, "jigc-version: 0.0.1-test\n").expect("rewrite the stamp");

    let out = jigc(repo.path(), home.path(), &["validate", "--format", "json"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "a binary-mismatch advisory must NOT gate `jigc validate --format json` (exit 0); \
         stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let report: serde_json::Value = stdout_json(&out, &[0], "`jigc validate --format json`");
    let findings = report["findings"]
        .as_array()
        .expect("the report carries a findings array");
    let mismatch = findings
        .iter()
        .find(|f| f["code"] == "store-version.binary-mismatch")
        .expect("the mismatch advisory is present in the JSON report");
    assert!(
        mismatch["route"].is_string(),
        "an advisory finding must carry a non-null route (the advisory-route floor); got:\n{mismatch:#}",
    );
}

/// A **matching** stamp (the store written by this very build) and an **absent** stamp (a
/// pre-M36 hand-built store) each emit **no** `store-version.binary-mismatch` finding, and
/// `jigc validate` exits 0.
#[test]
fn validate_silent_on_matching_and_absent_stamp() {
    let repo = TempDir::new("silent");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    let stamp = repo.path().join(".jigc").join("version");

    // (matching) The stamp `setup` wrote records this build — no mismatch.
    let matching = jigc(repo.path(), home.path(), &["validate"]);
    let matching_out = String::from_utf8_lossy(&matching.stdout);
    assert!(
        matching.status.success(),
        "a matching stamp must exit 0; stdout:\n{matching_out}",
    );
    assert!(
        !matching_out.contains("store-version.binary-mismatch"),
        "a stamp recording the running build must emit NO mismatch finding; \
         stdout:\n{matching_out}",
    );

    // (absent) A store with no stamp (pre-M36) must never be false-flagged.
    fs::remove_file(&stamp).expect("remove the stamp");
    let absent = jigc(repo.path(), home.path(), &["validate"]);
    let absent_out = String::from_utf8_lossy(&absent.stdout);
    assert!(
        absent.status.success(),
        "an absent stamp must exit 0; stdout:\n{absent_out}",
    );
    assert!(
        !absent_out.contains("store-version.binary-mismatch"),
        "an absent stamp (a pre-M36 store) must never be false-flagged; stdout:\n{absent_out}",
    );
}
