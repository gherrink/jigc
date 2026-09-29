//! **A `jigc setup` that fails after its first write does not wedge the repository**
//! (M54 Increment 4 / T1; `DECISIONS.md` → 2026-09-28 M54 settled, S22;
//! `completions/artifacts/M54/planning-gate-record.md` → row 9).
//!
//! Driven at the M54 baseline: a first `jigc setup` that failed **between its first write
//! and its install commit** — a read-only in-repo `core.hooksPath`, a malformed committed
//! `.jigc/config/packs.yaml` — left its own writes uncommitted, and the plain re-run after
//! the fix refused them as the adopter's work with `setup.dirty-install-path`. The only
//! exit was `--force`, the consent the guard exists to make rare. M51's footprint record
//! ([`INSTALL_FOOTPRINT`]) was written only inside the install commit's own arms, so the
//! error returns before it left nothing to tell the next run those bytes were jigc's.
//!
//! **The fix, asserted here:** every error return in that span records the paths jigc
//! wrote in the failed run — clean against `HEAD` before it, dirty against `HEAD` at the
//! failure — and stages them, so a plain re-run completes (arms a–c). The guard stays armed
//! for everything else: an edit the adopter makes to a jigc-written file after the failure
//! is refused by name (d), a `--force` run records nothing (e), and a path dirty before the
//! run is still refused before anything is written (f).
//!
//! **And the settings cell is refused before the first write** (T2): a committed
//! `.claude/settings.json` the merges cannot parse (g) or take the shape of (j) is refused
//! with nothing written and a route to fix and commit it; after the committed fix a plain
//! re-run completes (h), and a fix left uncommitted is refused by name (i).
//!
//! Every arm drives the real binary (`CARGO_BIN_EXE_jigc`) over a throwaway repository with
//! one commit and no setup — the shape `dev/jigc-rig bare` builds.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The door's refusal over bytes it did not write (`engine::finalize::setup_dirty_install_finding`).
const DIRTY_CODE: &str = "setup.dirty-install-path";

/// The failed-run record M51 introduced (`cli::setup::INSTALL_FOOTPRINT_PATH`).
const INSTALL_FOOTPRINT: &str = ".jigc/state/setup-install-footprint";

/// The install commit's subject (`cli::setup::INSTALL_COMMIT_MESSAGE`).
const INSTALL_COMMIT: &str = "chore(jigc): install jigc workspace config";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-setup-failed-first-{tag}-{}-{:?}",
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

/// A directory made read-only (`0o555`) for as long as the guard lives, and writable again
/// (`0o755`) when it drops — so a panicking arm still leaves a tree [`TempDir`] can remove.
/// The `0o555`/`0o755` toggle is `tests/setup.rs`'s `set_dir_readonly` convention.
struct ReadOnly(PathBuf);

impl ReadOnly {
    fn new(dir: &Path) -> Self {
        set_mode(dir, 0o555);
        ReadOnly(dir.to_path_buf())
    }

    /// The adopter's fix: make the directory writable again.
    fn restore(self) {}
}

impl Drop for ReadOnly {
    fn drop(&mut self) {
        set_mode(&self.0, 0o755);
    }
}

fn set_mode(dir: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(dir).expect("stat dir for chmod").permissions();
    perms.set_mode(mode);
    fs::set_permissions(dir, perms).expect("chmod dir");
}

/// Run `git` in `repo`, asserting success, returning trimmed stdout.
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
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A real `git init` with a per-repo identity and one seed commit — a born `HEAD`, no setup.
fn bare_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    let path = repo.path();
    git(path, &["init", "-q"]);
    git(path, &["config", "user.email", "test@example.com"]);
    git(path, &["config", "user.name", "Test"]);
    write(path, "README.md", "hello\n");
    git(path, &["add", "README.md"]);
    git(path, &["commit", "-q", "-m", "initial"]);
    (repo, home)
}

/// Commit `contents` at `relative`.
fn commit_file(repo: &Path, relative: &str, contents: &str) {
    write(repo, relative, contents);
    git(repo, &["add", "--", relative]);
    git(
        repo,
        &[
            "commit",
            "-q",
            "-m",
            &format!("add {relative}"),
            "--",
            relative,
        ],
    );
}

/// A committed, in-repo `.githooks/` set as `core.hooksPath` — the hooks dir the install
/// writes its `pre-commit` hook into.
fn in_repo_hooks_path(repo: &Path) -> PathBuf {
    commit_file(repo, ".githooks/README", "project hooks\n");
    git(repo, &["config", "core.hooksPath", ".githooks"]);
    repo.join(".githooks")
}

/// Write `contents` at `relative` under `repo`, creating parent directories.
fn write(repo: &Path, relative: &str, contents: &str) {
    let path = repo.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(path, contents).expect("write file");
}

/// Run `jigc <args>` with `cwd = repo` and a temp `$HOME`, with any developer-shell
/// `JIGC_PACK_DIR` removed so the run composes the **embedded** packs.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run jigc")
}

/// stdout + stderr of an invocation, joined for message assertions.
fn said(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// Assert a run failed at exit 1 carrying `code`.
fn assert_fails_with(out: &std::process::Output, code: &str, what: &str) {
    let said = said(out);
    assert_eq!(out.status.code(), Some(1), "{what} exits 1: {said}");
    assert!(said.contains(code), "{what} carries `{code}`: {said}");
}

/// Assert the plain re-run completed: exit 0, the install commit at `HEAD`, and nothing
/// left in `git status`.
fn assert_rerun_completes(repo: &Path, out: &std::process::Output) {
    assert_eq!(
        out.status.code(),
        Some(0),
        "a plain re-run after the fix completes, with no `--force`: {}",
        said(out)
    );
    assert_eq!(
        git(repo, &["log", "-1", "--format=%s"]),
        INSTALL_COMMIT,
        "the install commit is at HEAD"
    );
    let status = git(repo, &["status", "--short"]);
    assert!(status.is_empty(), "`git status --short` is empty: {status}");
}

/// The paths a `setup.dirty-install-path` refusal lists — its `` `path` `` lines.
fn refused_paths(out: &std::process::Output) -> Vec<String> {
    said(out)
        .lines()
        .map(str::trim)
        .filter(|line| line.len() > 2 && line.starts_with('`') && line.ends_with('`'))
        .map(|line| line.trim_matches('`').to_string())
        .collect()
}

/// (a) A read-only in-repo `core.hooksPath` fails the first run at its last write step,
/// after every other install file is on disk. The adopter makes the dir writable, and a
/// **plain** re-run completes.
#[test]
fn a_read_only_hooks_path_failure_leaves_a_repo_a_plain_rerun_completes() {
    let (repo, home) = bare_repo("hooks");
    let (repo, home) = (repo.path(), home.path());
    let hooks = ReadOnly::new(&in_repo_hooks_path(repo));

    let first = jigc(repo, home, &["setup"]);
    assert_fails_with(&first, "setup.install-hook", "a read-only hooks dir");
    assert!(
        repo.join(".jigc/AGENT.md").exists(),
        "the failure is after the first write — the fixture must exercise the wedge"
    );

    hooks.restore();
    let rerun = jigc(repo, home, &["setup"]);
    assert_rerun_completes(repo, &rerun);
}

/// (b) The same over a **committed `CLAUDE.md`**, which the install merges into: a tracked
/// path whose worktree now differs from the index. A record alone leaves it refused (the
/// footprint's index leg); the failure path stages what it records, so the re-run completes.
#[test]
fn a_committed_claude_md_does_not_stay_wedged_after_a_failed_first_run() {
    let (repo, home) = bare_repo("claude-md");
    let (repo, home) = (repo.path(), home.path());
    commit_file(repo, "CLAUDE.md", "# Project\n\nOur notes.\n");
    let hooks = ReadOnly::new(&in_repo_hooks_path(repo));

    let first = jigc(repo, home, &["setup"]);
    assert_fails_with(&first, "setup.install-hook", "a read-only hooks dir");
    assert!(
        fs::read_to_string(repo.join("CLAUDE.md"))
            .expect("read CLAUDE.md")
            .contains(".jigc/AGENT.md"),
        "the failed run injected the reference into the committed CLAUDE.md"
    );

    hooks.restore();
    let rerun = jigc(repo, home, &["setup"]);
    assert_rerun_completes(repo, &rerun);
    assert!(
        git(repo, &["show", "HEAD:CLAUDE.md"]).contains("Our notes."),
        "the adopter's committed prose rides on"
    );
}

/// (c) A **mid-span** return: a committed malformed `.jigc/config/packs.yaml` fails the
/// compose-marker step after the bootstrap file, the reference and the project layer are
/// written. The adopter commits the fix, and the re-run completes.
#[test]
fn a_mid_span_compose_marker_failure_leaves_a_repo_a_plain_rerun_completes() {
    let (repo, home) = bare_repo("packs");
    let (repo, home) = (repo.path(), home.path());
    commit_file(repo, ".jigc/config/packs.yaml", "packs: [unclosed\n");

    let first = jigc(repo, home, &["setup"]);
    assert_fails_with(&first, "setup.compose-marker", "a malformed packs.yaml");
    assert!(
        repo.join(".jigc/AGENT.md").exists(),
        "the failure is after the first write — the fixture must exercise the wedge"
    );
    assert!(
        !repo.join(".claude/settings.json").exists(),
        "and before the settings merge — a mid-span return, not the last one"
    );

    write(repo, ".jigc/config/packs.yaml", "");
    git(
        repo,
        &[
            "commit",
            "-q",
            "-m",
            "fix packs.yaml",
            "--",
            ".jigc/config/packs.yaml",
        ],
    );
    let rerun = jigc(repo, home, &["setup"]);
    assert_rerun_completes(repo, &rerun);
}

/// (d) **Re-arm control.** An adopter's edit to a jigc-written file *after* the failure is
/// their work, not jigc's: the record no longer describes those bytes, so the re-run
/// refuses over exactly that path — and over none of jigc's own.
#[test]
fn an_edit_after_the_failure_re_arms_the_guard_over_exactly_that_path() {
    let (repo, home) = bare_repo("re-arm");
    let (repo, home) = (repo.path(), home.path());
    let hooks = ReadOnly::new(&in_repo_hooks_path(repo));

    let first = jigc(repo, home, &["setup"]);
    assert_fails_with(&first, "setup.install-hook", "a read-only hooks dir");

    let agent = repo.join(".jigc/AGENT.md");
    let mut body = fs::read_to_string(&agent).expect("read AGENT.md");
    body.push_str("\nADOPTER NOTE\n");
    fs::write(&agent, body).expect("append to AGENT.md");

    hooks.restore();
    let rerun = jigc(repo, home, &["setup"]);
    assert_fails_with(&rerun, DIRTY_CODE, "an adopter edit after the failure");
    assert_eq!(
        refused_paths(&rerun),
        vec![".jigc/AGENT.md".to_string()],
        "the refusal names exactly the edited path: {}",
        said(&rerun)
    );
    assert!(
        fs::read_to_string(&agent)
            .expect("read AGENT.md")
            .contains("ADOPTER NOTE"),
        "and the adopter's bytes are still there"
    );
}

/// (e) A failed **`--force`** run records nothing: the consent never established the bytes
/// as jigc's own, and the record would otherwise exempt them on a later plain run.
#[test]
fn a_failed_force_run_records_no_footprint() {
    let (repo, home) = bare_repo("force");
    let (repo, home) = (repo.path(), home.path());
    let hooks = ReadOnly::new(&in_repo_hooks_path(repo));

    let first = jigc(repo, home, &["setup", "--force"]);
    assert_fails_with(
        &first,
        "setup.install-hook",
        "a read-only hooks dir under --force",
    );
    assert!(
        repo.join(".jigc/AGENT.md").exists(),
        "the failure is after the first write — the fixture must exercise the span"
    );
    assert!(
        !repo.join(INSTALL_FOOTPRINT).exists(),
        "a failed `--force` run leaves no `{INSTALL_FOOTPRINT}`"
    );
    drop(hooks);
}

/// (f) **Still refused.** An uncommitted adopter edit to a committed `CLAUDE.md` *before*
/// the run is refused by the pre-write gate, and nothing is written.
#[test]
fn a_path_dirty_before_the_run_is_still_refused_with_nothing_written() {
    let (repo, home) = bare_repo("dirty-before");
    let (repo, home) = (repo.path(), home.path());
    commit_file(repo, "CLAUDE.md", "# Project\n");
    write(repo, "CLAUDE.md", "# Project\n\nUNCOMMITTED WIP\n");

    let out = jigc(repo, home, &["setup"]);
    assert_fails_with(&out, DIRTY_CODE, "a path dirty before the run");
    assert_eq!(
        refused_paths(&out),
        vec!["CLAUDE.md".to_string()],
        "the refusal names the adopter's path: {}",
        said(&out)
    );
    assert!(
        !repo.join(".jigc").exists(),
        "the pre-write gate refuses before any write — `.jigc` is absent"
    );
    assert!(
        fs::read_to_string(repo.join("CLAUDE.md"))
            .expect("read CLAUDE.md")
            .contains("UNCOMMITTED WIP"),
        "the adopter's bytes are untouched"
    );
}

/// A committed `.claude/settings.json` whose JSON does not parse — the M54 baseline's
/// second wedge cell (S22).
const MALFORMED_SETTINGS: &str = "{ \"permissions\": { \"allow\": [ }\n";

/// The settings file the allowlist, hook and deny merges write into.
const SETTINGS: &str = ".claude/settings.json";

/// The refusal's `route:` line, as the text surface prints it.
fn route(out: &std::process::Output) -> String {
    said(out)
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("route:"))
        .unwrap_or_else(|| panic!("the refusal prints a route: {}", said(out)))
        .trim()
        .to_string()
}

/// Assert a settings refusal came **before the first write** (M54 Increment 4 / T2): exit 1
/// with `setup.inject-allowlist`, a message naming the file, a route that never sends the
/// adopter to a permission they already have, and nothing on disk or in the index for the
/// next run to meet — which is also why no footprint is recorded.
fn assert_refused_before_any_write(repo: &Path, out: &std::process::Output, what: &str) {
    assert_fails_with(out, "setup.inject-allowlist", what);
    let said = said(out);
    assert!(
        said.contains(SETTINGS),
        "{what}: the message names `{SETTINGS}`: {said}"
    );
    let route = route(out);
    assert!(
        !route.contains("writable"),
        "{what}: the route is not a permission fix: {route}"
    );
    assert!(
        route.contains("commit"),
        "{what}: the route says to commit the fix, since an uncommitted one is refused: {route}"
    );
    let status = git(repo, &["status", "--short"]);
    assert!(
        status.is_empty(),
        "{what}: `git status --short` is empty: {status}"
    );
    assert!(!repo.join(".jigc").exists(), "{what}: `.jigc/` is absent");
    assert!(
        !repo.join("CLAUDE.md").exists(),
        "{what}: `CLAUDE.md` is absent"
    );
    assert!(
        !repo.join(INSTALL_FOOTPRINT).exists(),
        "{what}: no `{INSTALL_FOOTPRINT}` is recorded"
    );
}

/// The adopter's fix to the settings file: valid JSON jigc merges into.
const FIXED_SETTINGS: &str = "{ \"permissions\": { \"allow\": [] } }\n";

/// (g) A committed `.claude/settings.json` that does not parse is refused **before the
/// first write**, with the parser's own line and column and a route to fix and commit it.
#[test]
fn a_malformed_committed_settings_file_is_refused_before_any_write() {
    let (repo, home) = bare_repo("settings-malformed");
    let (repo, home) = (repo.path(), home.path());
    commit_file(repo, SETTINGS, MALFORMED_SETTINGS);

    let out = jigc(repo, home, &["setup"]);
    assert_refused_before_any_write(repo, &out, "a malformed settings file");
    let said = said(&out);
    assert!(
        said.contains("line 1") && said.contains("column"),
        "the message carries the parser's line and column: {said}"
    );
}

/// (h) After the adopter **commits** the fix, a plain re-run completes.
#[test]
fn after_the_settings_fix_is_committed_a_plain_rerun_completes() {
    let (repo, home) = bare_repo("settings-fixed");
    let (repo, home) = (repo.path(), home.path());
    commit_file(repo, SETTINGS, MALFORMED_SETTINGS);

    let first = jigc(repo, home, &["setup"]);
    assert_fails_with(
        &first,
        "setup.inject-allowlist",
        "a malformed settings file",
    );

    commit_file(repo, SETTINGS, FIXED_SETTINGS);
    let rerun = jigc(repo, home, &["setup"]);
    assert_rerun_completes(repo, &rerun);
}

/// (i) **Re-arm control.** The fix left **uncommitted** is the adopter's work on a tracked
/// install path, so the re-run refuses it by name — which is why the route says commit.
#[test]
fn an_uncommitted_settings_fix_is_refused_by_name() {
    let (repo, home) = bare_repo("settings-uncommitted");
    let (repo, home) = (repo.path(), home.path());
    commit_file(repo, SETTINGS, MALFORMED_SETTINGS);

    let first = jigc(repo, home, &["setup"]);
    assert_fails_with(
        &first,
        "setup.inject-allowlist",
        "a malformed settings file",
    );

    write(repo, SETTINGS, FIXED_SETTINGS);
    let rerun = jigc(repo, home, &["setup"]);
    assert_fails_with(&rerun, DIRTY_CODE, "an uncommitted settings fix");
    assert_eq!(
        refused_paths(&rerun),
        vec![SETTINGS.to_string()],
        "the refusal names exactly the settings file: {}",
        said(&rerun)
    );
}

/// (j) A committed settings file that **parses but has a shape the merge cannot take** —
/// `permissions` an array — is refused the same way, before any write.
#[test]
fn a_settings_file_of_the_wrong_shape_is_refused_before_any_write() {
    let (repo, home) = bare_repo("settings-shape");
    let (repo, home) = (repo.path(), home.path());
    commit_file(repo, SETTINGS, "{\"permissions\": []}\n");

    let out = jigc(repo, home, &["setup"]);
    assert_refused_before_any_write(repo, &out, "a wrong-shape settings file");
    let said = said(&out);
    assert!(
        said.contains("`permissions` is not a JSON object"),
        "the message carries the shape error: {said}"
    );
}
