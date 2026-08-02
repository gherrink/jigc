//! Release-artifact smoke test — proves the shipped `jigc` binary is real.
//!
//! Increment 6 (`implementation/roadmap.md` → Increment 6: *Release build + a
//! quickstart*; **Proves:** the path of least resistance exists on a real
//! machine). Three assertions over the *built* binary + the shipped doc:
//!   1. `jigc --version` reports the workspace version (the artifact identifies
//!      itself as `jigc <version>`, the settled name — `DECISIONS.md`
//!      2026-05-31 → Product name);
//!   2. `jigc setup` in a fresh temp repo exits 0 and leaves *both* adapter
//!      files in place (the install end-to-end over the real binary);
//!   3. `QUICKSTART.md` exists and names the three loop commands
//!      (`jigc setup` → `jigc start "<intent>"` → `jigc task finalize`).
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the version from `CARGO_PKG_VERSION`, the temp repo is
//! built with `std::fs`, and a self-cleaning `TempDir` keeps the test off the
//! developer's real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-smoke-{tag}-{}-{:?}",
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

/// Make `root` a real git repo. `jigc setup` now installs a `pre-commit` hook that
/// resolves the repo's real hooks dir via git, so a bare `.git` marker no longer
/// suffices — the install runs `git rev-parse` against an actual repo.
fn mark_repo(root: &Path) {
    let out = Command::new("git")
        .args(["init", "-q"])
        .current_dir(root)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git init");
    assert!(
        out.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    configure_identity(root);
}

/// `jigc setup` commits its own install footprint (M30 audit finding 1) — even on an
/// unborn HEAD it mints the repo's first commit — so a test repo needs a usable identity
/// and signing off for that commit to land deterministically.
fn configure_identity(root: &Path) {
    for kv in [
        ["user.email", "test@example.com"],
        ["user.name", "Test"],
        ["commit.gpgsign", "false"],
    ] {
        let ok = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["config", kv[0], kv[1]])
            .output()
            .expect("run git config")
            .status
            .success();
        assert!(ok, "git config {} failed", kv[0]);
    }
}

/// The repo root, derived from this test crate's manifest dir
/// (`<root>/crates/cli`).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

#[test]
fn version_reports_the_workspace_version() {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("--version")
        .output()
        .expect("run the jigc binary");
    assert!(out.status.success(), "`jigc --version` must exit 0");

    let stdout = String::from_utf8(out.stdout).expect("version output is UTF-8");
    let expected = format!("jigc {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(
        stdout, expected,
        "`jigc --version` must report the workspace version as `jigc <version>`",
    );
}

#[test]
fn setup_runs_end_to_end_over_the_built_binary() {
    let repo = TempDir::new("setup");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("setup")
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // Both adapter files are in place after the install.
    assert!(
        repo.path().join("CLAUDE.md").is_file(),
        "`jigc setup` must leave CLAUDE.md in place",
    );
    assert!(
        repo.path().join(".claude/settings.json").is_file(),
        "`jigc setup` must leave .claude/settings.json in place",
    );
}

#[test]
fn quickstart_documents_the_three_loop_commands() {
    let quickstart = repo_root().join("QUICKSTART.md");
    let body = fs::read_to_string(&quickstart)
        .unwrap_or_else(|_| panic!("QUICKSTART.md must exist at the repo root: {quickstart:?}"));

    for cmd in ["jigc setup", "jigc start", "jigc task finalize"] {
        assert!(
            body.contains(cmd),
            "QUICKSTART.md must name the loop command `{cmd}`",
        );
    }
}
