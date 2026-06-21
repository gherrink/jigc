//! Cascade-layer location: pack-default (embedded), team (`~/.config/jigc/`),
//! project (in-repo `.jigc/config/`), plus repo-root discovery.
//!
//! The CLI locates layers and hands the engine its run context; the engine
//! resolves. See `implementation/module-layout.md` → The I/O boundary and
//! `design/overrides.md`.
//!
//! This module does *location* only: where each cascade source lives and which
//! are present. It reads no bytes and resolves no deltas — that is the engine's
//! job (feed-layers-in / assert-results-out).

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

/// What the CLI hands the engine after locating the cascade sources: the repo
/// root plus the located layer paths.
///
/// `pack-default` is not a path here — it is served in-binary by
/// [`crate::pack::EmbeddedPack`], so the only filesystem-located layers are
/// `project` (in-repo, present/absent) and `team` (external, a fixed path
/// whose population the engine checks).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunContext {
    /// The discovered **worktree** root (the directory holding `.git`) — where code,
    /// the git index, and `HEAD` live.
    pub repo_root: PathBuf,
    /// **jigc_home** — the main checkout the committed doc-store and `.jigc/` bind to. In
    /// a linked worktree this is the *main* checkout, not [`repo_root`](Self::repo_root);
    /// outside a worktree the two coincide (M31 Inc 2 / WF3).
    pub jigc_home: PathBuf,
    /// The in-repo project cascade layer at `<jigc_home>/.jigc/config/`, when that
    /// directory exists. `None` means the project layer is absent.
    pub project_config: Option<PathBuf>,
    /// The external team cascade layer at `~/.config/jigc/`. This is the path
    /// the layer *would* live at; the engine checks whether it is populated.
    pub team_config: PathBuf,
}

/// The in-repo project-layer sub-path under the repo root (`.jigc/config/`).
const PROJECT_CONFIG_REL: &str = ".jigc/config";

/// The team-layer sub-path under `$HOME` (`~/.config/jigc/`).
const TEAM_CONFIG_REL: &str = ".config/jigc";

/// Locate the cascade sources starting from `start`, resolving the team path
/// from `$HOME`.
pub fn locate(start: &Path) -> Result<RunContext> {
    let home = std::env::var_os("HOME")
        .context("cannot resolve the team config path: $HOME is not set")?;
    locate_from(start, Path::new(&home))
}

/// Locate the cascade sources with an explicit home directory (the testable
/// core of [`locate`]).
fn locate_from(start: &Path, home: &Path) -> Result<RunContext> {
    let Some(repo_root) = discover_repo_root(start) else {
        bail!(
            "not inside a git repository (no .git found from {})",
            start.display()
        );
    };
    // jigc_home — the main checkout the `.jigc/` layer + committed doc-store bind to;
    // outside a worktree it is the byte-identical walk-up root (M31 Inc 2 / WF3).
    let jigc_home = cli::repo::jigc_home(start).unwrap_or_else(|| repo_root.clone());

    let project_dir = jigc_home.join(PROJECT_CONFIG_REL);
    let project_config = project_dir.is_dir().then_some(project_dir);

    let team_config = home.join(TEAM_CONFIG_REL);

    Ok(RunContext {
        repo_root,
        jigc_home,
        project_config,
        team_config,
    })
}

/// Walk up from `start` until a directory containing a `.git` entry is found,
/// returning that directory (the repo root).
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// A throwaway directory that cleans itself up on drop, so the locate tests
    /// never touch a real repo or the developer's `~/.config`.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-locate-{tag}-{}-{:?}",
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

    fn mark_repo(root: &Path) {
        fs::create_dir_all(root.join(".git")).expect("create .git marker");
    }

    #[test]
    fn discovers_the_repo_root_from_a_nested_start_dir() {
        let tmp = TempDir::new("root");
        let root = tmp.path();
        mark_repo(root);
        let nested = root.join("crates").join("cli").join("src");
        fs::create_dir_all(&nested).expect("create nested dirs");
        let home = TempDir::new("home");

        let ctx = locate_from(&nested, home.path()).expect("locate succeeds inside a repo");

        assert_eq!(
            ctx.repo_root, root,
            "repo-root discovery must return the dir holding `.git`",
        );
    }

    #[test]
    fn present_project_config_dir_is_located_as_the_project_layer() {
        let tmp = TempDir::new("project-present");
        let root = tmp.path();
        mark_repo(root);
        let project = root.join(".jigc").join("config");
        fs::create_dir_all(&project).expect("create .jigc/config");
        let home = TempDir::new("home");

        let ctx = locate_from(root, home.path()).expect("locate succeeds");

        assert_eq!(
            ctx.project_config,
            Some(project),
            "a present `.jigc/config/` must be located as the project layer",
        );
    }

    #[test]
    fn absent_project_config_dir_is_reported_absent() {
        let tmp = TempDir::new("project-absent");
        let root = tmp.path();
        mark_repo(root);
        // No `.jigc/config/` created.
        let home = TempDir::new("home");

        let ctx = locate_from(root, home.path()).expect("locate succeeds");

        assert_eq!(
            ctx.project_config, None,
            "an absent `.jigc/config/` must be reported as no project layer",
        );
    }

    #[test]
    fn team_path_resolves_under_home_at_config_jigc() {
        let tmp = TempDir::new("team");
        let root = tmp.path();
        mark_repo(root);
        let home = TempDir::new("home");

        let ctx = locate_from(root, home.path()).expect("locate succeeds");

        assert_eq!(
            ctx.team_config,
            home.path().join(".config").join("jigc"),
            "the team layer path must resolve to `~/.config/jigc/`",
        );
    }

    #[test]
    fn no_git_marker_is_not_in_repo() {
        let tmp = TempDir::new("no-repo");
        let root = tmp.path();
        // No `.git` marker created anywhere up the temp-dir chain we control.
        let home = TempDir::new("home");

        let err = locate_from(root, home.path()).expect_err("no `.git` means not in a repo");
        assert!(
            err.to_string().contains("not inside a git repository"),
            "missing `.git` must surface as a not-in-repo error; got {err}",
        );
    }
}
