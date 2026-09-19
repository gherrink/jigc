//! Cascade-layer location: pack-default (embedded), team (`~/.config/jigc/`),
//! project (in-repo `.jigc/config/`), plus repo-root discovery.
//!
//! The CLI locates layers and hands the engine its run context; the engine
//! resolves. See `implementation/module-layout.md` → The I/O boundary and
//! `design/overrides.md`.
//!
//! This module does *location* only: where each cascade source lives, which are
//! present, and — since M52 Inc 8 / T2 — why a missing one is missing. It reads no
//! file bytes and resolves no deltas — that is the engine's job (feed-layers-in /
//! assert-results-out).

use anyhow::{Context, Result};
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

impl RunContext {
    /// The path the project cascade layer occupies, **present or not** — the subject
    /// [`not_set_up`] needs to tell *absent* from *unreadable*. Spelled here rather than
    /// re-joined at each caller so the two facts (where the layer lives, what its absence
    /// means) stay in one home.
    pub(crate) fn project_config_path(&self) -> PathBuf {
        self.jigc_home.join(PROJECT_CONFIG_REL)
    }
}

/// The one project-layer-missing rejection every project-layer-requiring verb shares —
/// `jigc describe` / `ingest` / `migrate` / `migrate-corpus` / `upgrade` / `validate` /
/// the nine `milestone` doors / the compose paths all converge on this constructor when
/// `<jigc_home>/.jigc/config/` is not a readable directory (M43 T7: the anyhow-embedded
/// route rewrite, `design/surface-contract.md` → The route fence, closing paragraph). The
/// `jigc setup` span rides the checked [`engine::finding::Route::mechanical`]
/// constructor, so the CLI-seam parse fence asserts it parses against the real CLI.
///
/// # The two states, told apart here
///
/// Every caller reaches this after one `is_dir()`, and `is_dir()` answers `false` for
/// **absent** and for **unreadable** alike. Saying *"this project isn't set up"* over a
/// project that is set up — its `.jigc/` merely unreadable — is a law-1 lie the binary can
/// disprove with the same `stat` it just made, and the `jigc setup` route it prints would
/// fail for the identical reason (`completions/artifacts/M52/baseline-contracts.md` §4
/// LD-4, driven). So the discrimination lands **here, at the producer of the answer**,
/// rather than at each door: one `metadata` call, and every leaf sharing the constructor
/// shares the correction (`completions/artifacts/M52/settle-record.md` §13).
///
/// * a read fault (`Err` that is not `NotFound` — `EACCES` on a `chmod 000 .jigc`, an
///   unmounted home, an I/O error) names the fault and carries a
///   [`engine::finding::Route::human`] direction: no `jigc` argv repairs an unreadable
///   directory, so the constructor is not contorted into claiming a mechanical conversion
///   (the same judgment [`not_in_repo_route`] ships for its own axis);
/// * everything else — `NotFound`, and a path that resolves to a non-directory — keeps
///   the shipped not-set-up sentence **byte-for-byte**, which is what makes this a
///   discrimination rather than a replacement.
///
/// **Declared bound — the one arm this producer does not reach:** bare `jigc start`'s
/// orientation never calls it. [`crate::orient`] reads [`RunContext::project_config`]
/// itself and answers `OrientationView::UnsetProject` at exit 0 for `None`, so over an
/// unreadable `.jigc/` it still reports *unset-project*. Correcting that means a second
/// decision site or a fourth orientation variant — a pinned-envelope move this wave's
/// *fixes-and-understandability-only* boundary does not carry
/// (`tests/unreadable_project_layer.rs` states it too).
///
/// `pub` rather than `pub(crate)` so a suite sweeping either axis byte-compares against
/// **this** constructor instead of a copy typed into the test — the discipline
/// [`not_in_repo_message`] already ships for its own axis
/// (`tests/milestone_not_set_up_axis.rs`, `tests/not_in_repo_axis.rs`,
/// `tests/unreadable_project_layer.rs`).
pub fn not_set_up(project_config: &Path) -> anyhow::Error {
    match std::fs::metadata(project_config) {
        Err(err) if err.kind() != std::io::ErrorKind::NotFound => {
            let restore = engine::finding::Route::human(
                "restore read access to the `.jigc/` directory and run the command again",
            );
            anyhow::anyhow!("cannot read the `.jigc/config/` cascade layer: {err} — {restore}")
        }
        _ => {
            let setup = engine::finding::Route::mechanical(["jigc", "setup"], "");
            anyhow::anyhow!(
                "this project isn't set up — run {setup} (no `.jigc/config/` cascade layer found)"
            )
        }
    }
}

/// The one route a not-inside-a-git-repository refusal carries.
///
/// It is a [`engine::finding::Route::human`] direction, and says so rather than
/// manufacturing an argv to satisfy the route fence: standing outside a repository there
/// is no `jigc` command that repairs the state — the moves are `cd` and `git init`, and
/// `git init` is git's to run, not jigc's (`design/surface-contract.md` → The route
/// fence: *"a route whose command sits mid-sentence does not fit the constructor, and
/// the constructor is not to be contorted to claim a conversion"*).
pub fn not_in_repo_route() -> engine::finding::Route {
    engine::finding::Route::human(
        "run jigc from inside the target git repository; if this project isn't one yet, \
         `git init` here first",
    )
}

/// The one precondition text every door reachable from outside a repository shares.
///
/// It carries `start` because *where jigc looked* is the fact a user in the wrong
/// directory needs: the walk-up found no `.git` from there, and the path says which
/// "there" the shell was in.
pub fn not_in_repo_message(start: &Path) -> String {
    format!(
        "not inside a git repository (no `.git` found from {})",
        start.display()
    )
}

/// The shared not-inside-a-git-repository rejection, `anyhow` shape — every leaf verb but
/// the two that answer in `Finding`s converges here (M49 Inc 11 T2).
///
/// Before it there were three spellings across two contract shapes and 20 sites, and 45 of
/// the 47 leaf verbs stated the state with no route at all
/// (`design/surface-contract.md` → law 2; `tests/not_in_repo_axis.rs` drives the axis and
/// fences this constructor as the only composer of the sentence).
pub(crate) fn not_in_repo(start: &Path) -> anyhow::Error {
    anyhow::anyhow!("{} — {}", not_in_repo_message(start), not_in_repo_route())
}

/// Map a [`locate`] failure onto a blocking `<verb>.repo-root` finding — the `Finding`
/// shape of the same answer, for the two doors (`jigc setup` / `jigc uninstall`) whose
/// whole surface is a `Result<_, Finding>`.
///
/// The not-inside-a-repository cause — the only one a user standing in the wrong directory
/// can reach — answers with the **shared** message and route, so the two finding-shaped
/// doors say exactly what the anyhow-shaped doors say.
///
/// [`locate`] has exactly one other failure mode: an unset `$HOME`, which is a different
/// precondition (no `cd` repairs it) and is *not* on this axis — no directory you can stand
/// in produces it. It keeps its shipped message and its shipped `other_route` byte-for-byte.
/// **Declared bound:** that arm is a law-1 wobble this task did not own — it reports a
/// `<verb>.repo-root` code and a "run it from inside the repository" route for a fault that
/// is neither — and repairing it means minting a code, which is a contract move
/// (`DECISIONS.md` → 2026-08-31 M49 Increment 11 / T2).
pub(crate) fn locate_finding(
    code: &'static str,
    start: &Path,
    err: &anyhow::Error,
    other_route: &'static str,
) -> engine::finding::Finding {
    if discover_repo_root(start).is_none() {
        return engine::finding::Finding::block(
            code,
            not_in_repo_message(start),
            not_in_repo_route(),
        );
    }
    engine::finding::Finding::block(
        code,
        format!("cannot locate the repository root: {err:#}"),
        other_route,
    )
}

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
        return Err(not_in_repo(start));
    };
    // jigc_home — the main checkout the `.jigc/` layer + committed doc-store bind to;
    // outside a worktree it is the byte-identical walk-up root (M31 Inc 2 / WF3).
    let jigc_home = crate::repo::jigc_home(start).unwrap_or_else(|| repo_root.clone());

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
