//! Repo-root vs **jigc_home** resolution (M31 worktree fan-out, Inc 2 / WF3).
//!
//! `discover_repo_root` (the walk-up that every command module already owns) finds the
//! **worktree** root — the dir where code, the git index, and `HEAD` live. In a linked
//! git worktree that is *not* where `.jigc/` and the committed doc-store live: those
//! bind to **jigc_home**, the *main* checkout, so all worktrees of one project share a
//! single `.jigc/`. [`jigc_home`] is that separate resolver.
//!
//! It is **layered** so the ~15 fake-`.git` unit fixtures stay green: it shells to git
//! (`dirname(git rev-parse --git-common-dir)`) **only** when a *real linked-worktree*
//! `.git` is present — which is exactly the case where `.git` is a *file* (a `gitdir:`
//! pointer). A main checkout and a fake `create_dir_all(".git")` fixture both keep
//! `.git` as a *directory*, and for both the answer is the `.git`-bearing dir itself —
//! resolved by walk-up, never by shelling out (so a fake `.git` can never walk up to,
//! and bind against, a real ancestor repo).

use std::path::{Path, PathBuf};

/// Resolve **jigc_home** — the dir `.jigc/` state and the committed doc-store bind to.
///
/// Returns the same path [`discover_repo_root`](crate::repo::discover_repo_root) would
/// **outside** a worktree (byte-identical, by construction — both return the
/// `.git`-bearing ancestor directly); inside a linked worktree it returns the *main*
/// checkout. `None` when no `.git` is found walking up from `start`.
pub fn jigc_home(start: &Path) -> Option<PathBuf> {
    let repo_root = discover_repo_root(start)?;
    // Only a *linked worktree* keeps `.git` as a FILE (a `gitdir:` pointer); its
    // jigc_home is the MAIN checkout, reachable only through git. A main checkout — and
    // a fake `.git` fixture (an empty dir with no git internals) — keeps `.git` as a
    // directory, whose jigc_home is the `.git`-bearing dir itself. Shell to git only
    // for the file case, so a fake `.git` never walks up to a real ancestor repo.
    if !repo_root.join(".git").is_file() {
        return Some(repo_root);
    }
    git_common_dir_parent(&repo_root).or(Some(repo_root))
}

/// Walk up from `start` until a directory containing a `.git` entry is found,
/// returning that directory (the worktree root). The same walk-up every command module
/// owns; co-located here so [`jigc_home`] can layer over it.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

/// `dirname(git -C <repo_root> rev-parse --path-format=absolute --git-common-dir)` —
/// the *main* checkout behind a linked worktree. `None` if git fails or returns empty.
fn git_common_dir_parent(repo_root: &Path) -> Option<PathBuf> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let common = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if common.is_empty() {
        return None;
    }
    Path::new(&common).parent().map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway directory that removes itself on drop (the project's no-tempfile
    /// pattern). Cleans up worktrees too — `remove_dir_all` flattens the tree.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-repo-unit-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn git(dir: &Path, args: &[&str]) {
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    }

    /// (a) Outside any worktree, `jigc_home` returns the byte-identical path the
    /// walk-up resolver does — the behavior-preserving guarantee.
    #[test]
    fn jigc_home_matches_walk_up_outside_worktree() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let start = dir.path();

        assert_eq!(
            jigc_home(start),
            discover_repo_root(start),
            "outside a worktree jigc_home must equal the walk-up repo root",
        );
        assert_eq!(jigc_home(start).as_deref(), Some(dir.path()));
    }

    /// (b) Inside a linked worktree (`.git` is a file), `jigc_home` returns the MAIN
    /// checkout, not the worktree root.
    #[test]
    fn jigc_home_resolves_to_main_checkout_from_worktree() {
        let main = TempDir::new();
        git(main.path(), &["init", "-q"]);
        git(main.path(), &["config", "user.email", "t@t"]);
        git(main.path(), &["config", "user.name", "t"]);
        git(
            main.path(),
            &["commit", "-q", "--allow-empty", "-m", "init"],
        );

        let linked = main.path().join("linked");
        git(
            main.path(),
            &["worktree", "add", "-q", linked.to_str().unwrap()],
        );
        assert!(
            linked.join(".git").is_file(),
            "a linked worktree's .git must be a file",
        );

        // The worktree's walk-up root is the worktree itself...
        assert_eq!(
            discover_repo_root(&linked).as_deref(),
            Some(linked.as_path())
        );
        // ...but jigc_home redirects to the main checkout.
        let home = jigc_home(&linked).expect("jigc_home resolves from a worktree");
        assert_eq!(
            std::fs::canonicalize(&home).unwrap(),
            std::fs::canonicalize(main.path()).unwrap(),
            "jigc_home from a worktree must be the main checkout, not the worktree",
        );
        assert_ne!(
            std::fs::canonicalize(&home).unwrap(),
            std::fs::canonicalize(&linked).unwrap(),
        );
    }

    /// (c) The load-bearing red: a fake `.git` fixture (empty `create_dir_all(".git")`,
    /// no `git init`) nested *inside a real git repo* resolves to the fake-bearing dir
    /// itself — the walk-up fallback fires and never shells out to the real ancestor.
    #[test]
    fn jigc_home_fake_git_does_not_shell_to_ancestor() {
        let ancestor = TempDir::new();
        // A real repo as the ancestor — what git WOULD bind to if we shelled out.
        git(ancestor.path(), &["init", "-q"]);

        let fake = ancestor.path().join("nested");
        std::fs::create_dir_all(fake.join(".git")).expect("seed a fake .git dir");

        assert_eq!(
            jigc_home(&fake).as_deref(),
            Some(fake.as_path()),
            "a fake .git must resolve to itself (walk-up fallback), not the ancestor",
        );
        assert_ne!(
            jigc_home(&fake).as_deref(),
            Some(ancestor.path()),
            "a fake .git must NOT shell out and bind to the real ancestor repo",
        );
    }
}
