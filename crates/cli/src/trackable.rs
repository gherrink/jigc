//! **Can git track a path here?** — the one predicate every door that computes a
//! *destination* for a managed doc asks before it moves or writes one there.
//!
//! The rule is M48 Increment 5's, generalized off the install commit's `pre-commit`
//! hook and onto any repo-relative path: a path is trackable **iff** it is under the
//! canonicalized repo root, outside git's own dirs, and owned by *this* repository
//! (not a submodule or an embedded repo). [`crate::setup::committable_hook_path`] is
//! the shape it was first written in, and now calls this — one rule, one home, so the
//! next door to ask cannot get a different answer than the install commit does.
//!
//! **Why the movers need it (M49).** `git mv <src> .git/<dest>` prints
//! `error: invalid path '.git/<dest>'` on stderr and **exits 0**: the file moves on
//! disk, the source leaves the index, and nothing is added — a staged deletion with no
//! matching add. Every doc-relocating door read that exit 0 as a successful move and
//! said so, so `jigc config set placement-root .git` reported the relocation, exited 0,
//! and left the doc surviving only in history — gone from the next clone. An exit code
//! is therefore not a trackability test; this is.
//!
//! **What is *not* asked: gitignore.** A gitignored destination is a perfectly
//! trackable path git has merely been told to skip — `git mv docs/x.md .jigc/x.md`
//! stages a real `R` rename — and the workbench relocation
//! ([`crate::relocate`]'s squatter displacement) depends on exactly that. Ignoring is a
//! policy about a path git *can* record; this predicate is about paths it cannot.

use std::path::{Path, PathBuf};

/// Why `relative` (a repo-root-relative path, existing or not) is a destination git
/// **cannot record** in the repository at `repo_root` — `None` when it can.
///
/// The five tests, in the order they can be answered most cheaply, and none of them
/// optional:
///
///   - **under the repo root** — a destination reached through `..` is outside the tree
///     git commits from. (`git mv` already rejects this one loudly, at exit 128; it is
///     tested here so the *door* can refuse before it moves anything.)
///   - **no `.git` path component** — git refuses to record any path with a `.git`
///     component (`error: invalid path`), whatever the git dir actually is, so this
///     holds in a linked worktree where `.git` is a *file* and the dirs below name a
///     different tree.
///   - **outside git's own dirs** — `--git-dir` *and* `--git-common-dir`, both printed
///     by one `rev-parse`. The literal-component test above does not subsume this: a
///     `--separate-git-dir` / `GIT_DIR` repo keeps its object store under a directory
///     that is not called `.git` at all.
///   - **owned by *this* repository** — location is not trackability. A directory under
///     the root can belong to another repo (a submodule, or a plain embedded repo), and
///     a path inside one is not this index's to take. Git is asked, from the nearest
///     ancestor that exists, which repo owns the destination.
///   - **not under a gitlink in the *index*** — the same ownership question asked of the
///     index, which is the only side that can see a submodule that is registered but not
///     checked out (a `git clone` without `--recursive`, the default clone).
///
/// **Conservative toward the move**: when git cannot be asked at all, the git-side tests
/// abstain and the answer rests on the two structural ones. A door that then moves is no
/// worse off than before — the `git mv` it is about to run needs the same git.
pub(crate) fn untrackable_reason(repo_root: &Path, relative: &str) -> Option<String> {
    let relative = relative.trim_matches('/');
    if relative.is_empty() || relative == "." {
        return None; // the repo root itself is always trackable.
    }

    // 1 — under the repo root. Resolved lexically (the destination need not exist yet);
    // an existing path is canonicalized so a symlinked ancestor cannot smuggle the
    // target out from under the tests below.
    let root = std::fs::canonicalize(repo_root).ok()?;
    let target = resolve(&root, relative)?;
    if !target.starts_with(&root) {
        return Some(format!(
            "`{relative}` resolves outside the repository at {}",
            root.display()
        ));
    }

    // 2 — git's own `invalid path` rule: no `.git` component, at any depth. Compared
    // case-insensitively because git's is (`core.protectNTFS`/`protectHFS` exist so the
    // rule cannot be dodged by case on the filesystems where case does not bind).
    if Path::new(relative)
        .components()
        .any(|c| c.as_os_str().eq_ignore_ascii_case(".git"))
    {
        return Some(format!(
            "`{relative}` is inside git's own directory — git refuses to track any path \
             with a `.git` component (`error: invalid path`), so the bytes would survive \
             only in history"
        ));
    }

    // 3 — git's own dirs, wherever they actually live.
    if let Some(out) = git_output(
        repo_root,
        [
            "rev-parse",
            "--path-format=absolute",
            "--git-dir",
            "--git-common-dir",
        ],
    ) && out.status.success()
    {
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            let git_dir = line.trim();
            if git_dir.is_empty() {
                continue;
            }
            // A git dir that does not resolve cannot contain the destination.
            if std::fs::canonicalize(git_dir).is_ok_and(|dir| target.starts_with(&dir)) {
                return Some(format!(
                    "`{relative}` is inside this repository's git directory ({git_dir}) — \
                     git records nothing there"
                ));
            }
        }
    }

    // 4 — ownership, asked from the nearest ancestor that exists on disk (the
    // destination itself may not).
    let mut probe = target.as_path();
    let owner = loop {
        if probe.is_dir() {
            break git_output(probe, ["rev-parse", "--show-toplevel"]);
        }
        match probe.parent() {
            Some(parent) if parent.starts_with(&root) || parent == root => probe = parent,
            _ => break None,
        }
    };
    if let Some(out) = owner
        && out.status.success()
    {
        let toplevel = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if std::fs::canonicalize(&toplevel).is_ok_and(|owner| owner != root) {
            return Some(format!(
                "`{relative}` is inside another repository ({toplevel}) — a submodule or an \
                 embedded repo, whose paths this index cannot record"
            ));
        }
    }

    // 5 — …and the same question of the index, the only side that can see a registered
    // but un-checked-out submodule.
    if index_gitlink_covers(repo_root, relative) {
        return Some(format!(
            "`{relative}` is inside a submodule registered in this repository's index — a \
             pathspec inside a submodule matches nothing"
        ));
    }

    None
}

/// Resolve `relative` against the canonicalized `root`: the real path when it exists
/// (symlinks followed), else the lexical join with `.`/`..` folded out — so a
/// destination that does not exist yet is still placed. `None` when `..` walks off the
/// front of the path entirely.
fn resolve(root: &Path, relative: &str) -> Option<PathBuf> {
    let joined = root.join(relative);
    if let Ok(real) = std::fs::canonicalize(&joined) {
        return Some(real);
    }
    let mut out = root.to_path_buf();
    for component in Path::new(relative).components() {
        use std::path::Component;
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    return None;
                }
            }
            Component::Normal(part) => out.push(part),
            // An absolute component restarts the path — `join` already honoured it.
            Component::RootDir | Component::Prefix(_) => out = joined.clone(),
        }
    }
    Some(out)
}

/// Whether the index holds a **gitlink** (mode `160000`) at any ancestor directory of
/// `relative` — i.e. whether the path lies inside a submodule as far as *this* index is
/// concerned, checked out or not.
///
/// Asks about the ancestors rather than the path itself, because a pathspec *inside* a
/// submodule matches nothing (that is the whole problem). The gitlink's own reported path
/// is then checked to be a proper ancestor: a sibling submodule under a shared parent
/// (`my-hooks/vendored` beside `my-hooks/pre-commit`) matches the ancestor pathspec but
/// does not contain the path, and refusing on it would drop a perfectly committable one.
/// `-z` so paths arrive unquoted whatever `core.quotePath` says. Conservative on failure:
/// if git cannot be asked, the path keeps whatever the other tests granted it.
pub(crate) fn index_gitlink_covers(repo_root: &Path, relative: &str) -> bool {
    let mut ancestors: Vec<String> = Vec::new();
    let mut prefix = String::new();
    // Every proper ancestor DIRECTORY of the path (its own component dropped).
    let mut components: Vec<&str> = relative.split('/').collect();
    components.pop();
    for component in components {
        if !prefix.is_empty() {
            prefix.push('/');
        }
        prefix.push_str(component);
        ancestors.push(prefix.clone());
    }
    if ancestors.is_empty() {
        return false;
    }

    let mut args: Vec<String> = vec![
        "ls-files".into(),
        "--stage".into(),
        "-z".into(),
        "--".into(),
    ];
    args.extend(ancestors);
    let Some(out) = git_output(repo_root, args) else {
        return false;
    };
    if !out.status.success() {
        return false;
    }
    String::from_utf8_lossy(&out.stdout)
        .split('\0')
        .filter_map(|entry| entry.strip_prefix("160000 "))
        .filter_map(|entry| entry.split_once('\t'))
        .any(|(_, path)| relative.starts_with(&format!("{path}/")))
}

/// Run `git <args>` in `dir`, handing back the captured output — `None` when git could
/// not be spawned at all. The predicate form: a git that cannot answer abstains.
fn git_output<I, S>(dir: &Path, args: I) -> Option<std::process::Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway git repo that removes itself on drop (the project's no-tempfile pattern).
    struct TempRepo(PathBuf);

    impl TempRepo {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-trackable-{tag}-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp repo");
            let repo = TempRepo(path);
            repo.git(&["init", "-q"]);
            repo.git(&["config", "user.email", "t@t"]);
            repo.git(&["config", "user.name", "t"]);
            repo
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn git(&self, args: &[&str]) {
            let out = std::process::Command::new("git")
                .arg("-C")
                .arg(&self.0)
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
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// **The refusing half, over the two shapes git cannot record**, each at more than one
    /// depth and whether or not it exists on disk — the destination of a move need not
    /// exist yet, which is exactly why the answer cannot be read off the filesystem.
    #[test]
    fn every_shape_git_cannot_record_is_named_as_such() {
        let repo = TempRepo::new("refuse");
        std::fs::create_dir_all(repo.path().join(".git/hooks")).expect("hooks dir");

        for relative in [
            ".git",
            ".git/roadmap.md",
            ".git/hooks/roadmap.md",
            ".git/jigc-docs/decisions/one.md",
            ".GIT/roadmap.md",
            "docs/.git/roadmap.md",
            "../escaped.md",
            "../../escaped.md",
            "docs/../../escaped.md",
        ] {
            assert!(
                untrackable_reason(repo.path(), relative).is_some(),
                "`{relative}` is a destination git cannot record — it must be refused",
            );
        }
    }

    /// **The admitting half**, and the bound the fix is scoped by: everything git *can*
    /// record is admitted, including a **gitignored** path. Ignoring is a policy about a
    /// path git can track and has been told to skip — `git mv docs/x.md .jigc/x.md` stages
    /// a real `R` rename — so refusing it would have broken the workbench relocation while
    /// closing nothing.
    #[test]
    fn an_ordinary_or_merely_gitignored_destination_is_admitted() {
        let repo = TempRepo::new("admit");
        std::fs::write(repo.path().join(".gitignore"), ".jigc/\n").expect("write gitignore");
        std::fs::create_dir_all(repo.path().join(".jigc")).expect("workbench dir");

        for relative in [
            "",
            ".",
            "docs/roadmap.md",
            "notes/roadmap.md",
            "ROADMAP.md",
            ".jigc/roadmap.md",
            ".jigc/displaced/roadmap.md",
            ".github/workflows/ci.yml",
            "docs/./decisions/one.md",
            "deep/nested/never/created/one.md",
        ] {
            assert_eq!(
                untrackable_reason(repo.path(), relative),
                None,
                "`{relative}` is a path git can record — refusing it would break a \
                 legitimate move",
            );
        }
    }

    /// **Location is not trackability.** A directory under the root can belong to another
    /// repository, and a path inside one is not this index's to take — `git add` refuses it
    /// fatally (a submodule) or stages nothing at exit 0 (an embedded repo). This is the
    /// ownership half the M48 install-commit rule already carried; it comes along because
    /// the rule is asked here rather than restated.
    #[test]
    fn a_path_inside_an_embedded_repository_is_not_this_index_s_to_take() {
        let repo = TempRepo::new("embedded");
        let inner = repo.path().join("vendored");
        std::fs::create_dir_all(&inner).expect("inner dir");
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(&inner)
            .args(["init", "-q"])
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .expect("run git init");
        assert!(out.status.success(), "the embedded repo initializes");

        assert!(
            untrackable_reason(repo.path(), "vendored/roadmap.md").is_some(),
            "a destination inside an embedded repository is refused",
        );
        assert_eq!(
            untrackable_reason(repo.path(), "roadmap.md"),
            None,
            "…and its sibling outside is unaffected",
        );
    }
}
