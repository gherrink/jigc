//! End-to-end integration test for `jigc setup` — the adapter install.
//!
//! Drives the built `jigc` binary against a throwaway temp repo and asserts the
//! two injections from `design/assistant-adapter.md` → The three
//! responsibilities (inject the bootstrap line floor + allowlist `jigc`):
//!   1. the marker-fenced bootstrap block lands in `CLAUDE.md`;
//!   2. the `jigc *` permit lands in `.claude/settings.json`.
//!
//! It also asserts the command exits clean and is idempotent (a second run leaves
//! both files byte-identical), per `design/assistant-adapter.md`.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is built with `std::fs`, and a
//! self-cleaning `TempDir` keeps the test off the developer's real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-setup-{tag}-{}-{:?}",
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

/// Mark `root` as a git repo for repo-root discovery, without invoking git — a
/// bare `.git` directory is enough for `locate`.
fn mark_repo(root: &Path) {
    fs::create_dir_all(root.join(".git")).expect("create .git marker");
}

/// Run the built `jigc setup` binary with `cwd = repo` and `$HOME = home`.
fn run_setup(repo: &Path, home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("setup")
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn setup_installs_line_and_allowlist() {
    let repo = TempDir::new("install");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_setup(repo.path(), home.path());
    assert!(
        out.status.success(),
        "`jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // 1. The bootstrap block landed in CLAUDE.md, fenced by its markers.
    let claude_md =
        fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md was written");
    assert!(
        claude_md.contains("<!-- jigc:bootstrap:start -->"),
        "CLAUDE.md must carry the bootstrap start marker; got:\n{claude_md}",
    );
    assert!(
        claude_md.contains("<!-- jigc:bootstrap:end -->"),
        "CLAUDE.md must carry the bootstrap end marker; got:\n{claude_md}",
    );
    assert!(
        claude_md.contains("`jigc` is your interface to this project"),
        "CLAUDE.md must carry the bootstrap sentence; got:\n{claude_md}",
    );

    // 2. The `jigc *` permit landed in .claude/settings.json.
    let settings = fs::read_to_string(repo.path().join(".claude/settings.json"))
        .expect(".claude/settings.json was written");
    assert!(
        settings.contains("\"jigc *\""),
        ".claude/settings.json must permit `jigc *`; got:\n{settings}",
    );

    // Idempotent: a second run leaves both files byte-identical.
    let out2 = run_setup(repo.path(), home.path());
    assert!(out2.status.success(), "second `jigc setup` must exit 0");
    let claude_md2 =
        fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md still present");
    let settings2 = fs::read_to_string(repo.path().join(".claude/settings.json"))
        .expect(".claude/settings.json still present");
    assert_eq!(
        claude_md, claude_md2,
        "a second `jigc setup` must leave CLAUDE.md byte-identical",
    );
    assert_eq!(
        settings, settings2,
        "a second `jigc setup` must leave .claude/settings.json byte-identical",
    );
}
