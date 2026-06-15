//! M23 Increment 1, T3 — the `migrate-changelog` workflow + its `author-migration`
//! step, shipped in the dev pack (no project shadow).
//!
//! Done-criterion (T3): `jigc migrate <foreign CHANGELOG.md> --as changelog` composes
//! the **shipped** `migrate-changelog` workflow (NOT a test shadow) and the emitted
//! view carries both
//!   - the resolved foreign content (the source seam — `{{ source }}` surfaces the
//!     staged foreign bytes verbatim), and
//!   - the author-spine command guidance (the create-changelog command-ref + the
//!     add-item / set-field date / set-slot / finalize directions),
//!
//! and the workflow + step load and pass the compose-time workflow-refs gate (a clean
//! exit 0 — a dangling command-ref or step-include would block composition).
//!
//! The seam feed itself is T2's contract; here the contract is that the **shipped pack
//! workflow** composes (so the verb works without the T2 shadow) and surfaces the
//! author spine — including the historical-`date` overwrite guidance (Grouped scope).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-migrate-wf-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
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
        .spawn()
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
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
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// A realistic multi-release foreign Keep-a-Changelog file carrying HISTORICAL dates.
const FOREIGN: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0] - 2023-01-15
### Added
- Device-code OAuth flow.
### Fixed
- Session fixation on logout.

## [1.1.0] - 2022-08-01
### Changed
- Bumped the default timeout to 30s.
";

#[test]
fn migrate_composes_the_shipped_workflow_with_seam_and_author_spine() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    ok_stdout(setup, "jigc setup");

    // No project shadow: the SHIPPED pack `migrate-changelog` workflow + its
    // `author-migration` step must compose on their own.
    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign CHANGELOG.md");

    // A clean exit 0 IS the workflow-refs gate passing — a dangling command-ref or a
    // missing step-include would block composition and exit non-zero.
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
    );
    let stdout = ok_stdout(out, "jigc migrate CHANGELOG.md --as changelog");

    // (1) The source seam resolved: the foreign content is surfaced verbatim.
    assert!(
        stdout.contains(FOREIGN.trim_end()),
        "the composed shipped workflow must surface the foreign content (the source seam); stdout:\n{stdout}",
    );

    // (2) The author spine is present — the create-changelog command-ref resolved to
    // its real command (proving the workflow-refs gate accepted it), plus the
    // add-item / set-slot directions through the write verbs.
    assert!(
        stdout.contains("jigc doc create changelog"),
        "the create-changelog command-ref must resolve into the composed view; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("jigc doc add-item changelog:changelog#releases"),
        "the release add-item guidance must be present; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("set-slot"),
        "the notes set-slot guidance must be present; stdout:\n{stdout}",
    );

    // (3) The historical-date clause: the step directs `set-field <release>/date` from
    // the FOREIGN historical date (overwriting the on-create stamp). Grouped scope.
    assert!(
        stdout.contains("set-field") && stdout.contains("/date"),
        "the historical-date set-field guidance must be present; stdout:\n{stdout}",
    );

    // (4) The finalize step composed in (the spine ends at finalize).
    assert!(
        stdout.contains("jigc task finalize") || stdout.contains("finalize"),
        "the finalize step must compose in; stdout:\n{stdout}",
    );
}
