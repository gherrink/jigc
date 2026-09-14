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
        // No path at all: the subject IS the repository, whose repo-relative spelling is
        // `.`, and `resolves outside the repository at .` is noise. Naming the host root
        // here was the M50 completion audit's finding 3 — law 1 (`a surface prints no host
        // filesystem`) enforced at the four destroying doors and nowhere else, while this
        // one predicate hands its text to four more.
        return Some(format!("`{relative}` resolves outside the repository root"));
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
                // Rendered against the root, not printed as git handed it over: a git dir
                // reachable from a destination *under* the root is itself under the root
                // (`.git`, or a `--separate-git-dir` inside the tree), so this has a
                // repo-relative spelling. The helper's absolute fallback covers the
                // pathological case where it does not.
                let git_dir = engine::path::repo_relative(&root, Path::new(git_dir));
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
            // The owning repo was found by walking UP from a destination under the root and
            // stopping at the root, so its toplevel is under the root and has a
            // repo-relative spelling by construction.
            let toplevel = engine::path::repo_relative(&root, Path::new(&toplevel));
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

/// Adjudicate a caller-supplied path token that names a **source file inside this
/// repository**, answering with its clean repo-relative spelling — or with the reason it is
/// not one (M51 Increment 1 / T1; `completions/artifacts/M51/settle-record.md` → D1 parts 1+3,
/// as amended by §2).
///
/// **Three predicates, not two, and a resolve step before any of them.** D1 decided the door
/// would ask [`untrackable_reason`] + [`crate::config::is_workbench_root`] and called that
/// *"shipped predicates, no new capability"*. ***That is struck.*** The claim does not survive
/// being driven on the cell it was decided for: this module's own parameter is typed
/// *repo-root-relative* and [`untrackable_reason`] opens `relative.trim_matches('/')`
/// (`design/storage.md` says so in its own words), so `/private/tmp/victim/keepme.md` is
/// re-read as `<repo>/private/tmp/victim/keepme.md` and answers **trackable**. The
/// resolve-or-refuse step below is therefore a **new rule**, small and stated, and what the
/// `jigc config set` door actually refuses an absolute value with is a **third** predicate D1
/// never named — `config::unusable_root_reason`, whose symlink leg
/// ([`crate::config::symlinked_component`]) is the one this door reuses.
///
/// The four steps, in order:
///
///   1. **Resolve, or refuse** — the token is placed inside the repository
///      ([`place_inside`]); one that lands outside it is refused *as itself*, never folded
///      back in. A `..` that climbs above the root was previously folded **lexically**, so a
///      source one directory up was recorded as a repo-relative spelling naming a file that
///      is not the operator's.
///   2. **[`untrackable_reason`]** of the *resolved* value — git's own `.git`-component and
///      git-dir rules, ownership, and the index's gitlinks.
///   3. **[`crate::config::is_workbench_root`]** — jigc's own `.jigc/` tree, which git tracks
///      perfectly well and `jigc uninstall` removes whole.
///   4. **[`crate::config::symlinked_component`]** — the symlink leg of the root knobs'
///      usability rule, and only that leg: its file-shaped sibling refuses an existing
///      non-directory, which is exactly what a *source* is. The leg is shared; its sentence is
///      not, because the root knobs' names a consequence (`RD`-staged docs) that a source does
///      not have.
///
/// The reason is a sentence, not a code: one door, one code, the reason in the message — the
/// `config.untrackable-root` precedent, where the operator's fix is the same whichever leg
/// answered.
///
/// **The refusal quotes the token as typed**, including an absolute one. Law 1
/// (`design/surface-contract.md`) asks that every printed path be repo-real or a typed
/// identity; a source that resolves outside the repository *has no repo-real spelling* — the
/// `locate::not_in_repo_message` case, absolute by its subject — and the string is the
/// operator's own argument, which is the one thing they can edit.
pub(crate) fn resolve_source_token(repo_root: &Path, token: &str) -> Result<String, String> {
    let relative = place_inside(repo_root, token).ok_or_else(|| {
        format!(
            "`{token}` resolves outside the repository — `jigc migrate` reads its source and, \
             on `--approve`, retires it, so a source outside the tree would be deleted with no \
             copy in any commit of this repository"
        )
    })?;
    if let Some(reason) = untrackable_reason(repo_root, &relative) {
        return Err(reason);
    }
    if crate::config::is_workbench_root(&relative) {
        return Err(format!(
            "`{relative}` is inside jigc's own workbench (`.jigc/`) — the tree `jigc uninstall` \
             removes whole, so a source retired from there leaves bytes no index has a copy of"
        ));
    }
    if let Some(shown) = crate::config::symlinked_component(repo_root, &relative) {
        // The *fact* is the root knobs' — git records the link, never a path through it — but
        // the sentence is this door's: theirs names moved docs staging as `RD`, which is not
        // what happens to a source, and law 1 (`nothing lies`) is a rule about the sentence.
        return Err(format!(
            "`{shown}` is a symlink — git records the link, not a path through it, so \
             retiring the source would stage the removal of a path the worktree no longer \
             has, while the file it points at is untouched"
        ));
    }
    Ok(relative)
}

/// Place `token` inside `repo_root`, answering with its clean repo-relative spelling — `None`
/// when it lands anywhere else.
///
/// **Canonicalization-safe on both sides, and asymmetric on purpose.** The repository root and
/// the token's *directory* chain are both canonicalized, because a repo legitimately sits under
/// a symlinked ancestor (every macOS temp corpus lives under `/var` → `/private/var`) and a
/// caller types whichever spelling their shell handed them — comparing the two raw would refuse
/// an ordinary in-repo absolute path. The **final component is never canonicalized**: resolving
/// it would silently rewrite a symlinked source into its target, and whether the token names a
/// link is step 4's question, not this step's.
///
/// That asymmetry between the leaf and its ancestors is the rule, not an accident of the
/// implementation. An ancestor link is only a *route* to the bytes — resolving it records the
/// same file under a spelling git can record — while the leaf **is** the subject: recording a
/// link there would retire the link and leave the bytes, which is why step 4 refuses it.
fn place_inside(repo_root: &Path, token: &str) -> Option<String> {
    let root = std::fs::canonicalize(repo_root).unwrap_or_else(|_| repo_root.to_path_buf());
    let joined = if Path::new(token).is_absolute() {
        PathBuf::from(token)
    } else {
        root.join(token)
    };
    let folded = fold_lexically(&joined)?;
    let placed = match (folded.parent(), folded.file_name()) {
        (Some(parent), Some(leaf)) => real_dir(parent).join(leaf),
        // No file name at all (the token is the root itself) — nothing to keep literal.
        _ => real_dir(&folded),
    };
    let relative = placed
        .strip_prefix(&root)
        .or_else(|_| placed.strip_prefix(repo_root))
        .ok()?;
    Some(relative.to_string_lossy().into_owned())
}

/// Fold `.` and `..` out of an absolute path **lexically** — `None` when `..` walks off the
/// front of the filesystem. No filesystem access: this is the placement step, and the
/// components that exist are canonicalized by [`real_dir`] afterwards.
fn fold_lexically(path: &Path) -> Option<PathBuf> {
    use std::path::Component;

    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    return None;
                }
            }
            Component::Normal(part) => out.push(part),
            other => out.push(other.as_os_str()),
        }
    }
    Some(out)
}

/// `dir` with its **existing** prefix canonicalized and the rest re-appended — so a directory
/// chain that does not exist yet is still placed against the real filesystem rather than
/// compared raw.
fn real_dir(dir: &Path) -> PathBuf {
    if let Ok(real) = std::fs::canonicalize(dir) {
        return real;
    }
    let mut tail: Vec<&std::ffi::OsStr> = Vec::new();
    let mut probe = dir;
    while let Some(parent) = probe.parent() {
        tail.push(probe.file_name().unwrap_or(probe.as_os_str()));
        if let Ok(real) = std::fs::canonicalize(parent) {
            let mut out = real;
            for part in tail.iter().rev() {
                out.push(part);
            }
            return out;
        }
        probe = parent;
    }
    dir.to_path_buf()
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

    /// **No refusal names the host filesystem** (the M50 completion audit, finding 3).
    ///
    /// Three of the five reasons composed an absolute path — the repo root, git's own dir,
    /// and the owning repository's toplevel — and this is the *shared* predicate four doors
    /// ask (`config set placement-root`, `jigc rename`, `jigc setup`, `jigc relocate`). Law 1
    /// is stated universally (`design/surface-contract.md`: *every printed path is repo-real
    /// or a typed identity*), so the rule belongs to the predicate, not to whichever caller
    /// last got a bug report. Each reason is reached through the shape that is the ONLY way
    /// to reach it — the `.git`-component reason answers first for every ordinary layout, so
    /// the git-dir reason needs a `--separate-git-dir` inside the tree.
    #[test]
    fn no_refusal_names_the_host_path_of_the_machine_it_ran_on() {
        let repo = TempRepo::new("host-path");
        // The owning-repository reason.
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
        // The git-dir reason: a git dir inside the tree that is not called `.git`.
        repo.git(&["init", "-q", "--separate-git-dir=gitstore"]);

        // Both spellings of the root, because a canonicalizing filesystem (macOS
        // `/var` → `/private/var`) can put either one on the surface.
        let mut prefixes = vec![repo.path().to_string_lossy().into_owned()];
        if let Ok(real) = repo.path().canonicalize() {
            let real = real.to_string_lossy().into_owned();
            if !prefixes.contains(&real) {
                prefixes.push(real);
            }
        }

        for relative in [
            "../escaped.md",
            "gitstore/roadmap.md",
            "vendored/roadmap.md",
        ] {
            let reason = untrackable_reason(repo.path(), relative).unwrap_or_else(|| {
                panic!("`{relative}` must be refused — the cell reached no reason to check")
            });
            for prefix in &prefixes {
                assert!(
                    !reason.contains(prefix.as_str()),
                    "the refusal for `{relative}` prints the host path `{prefix}`, which is \
                     neither repo-real nor a typed identity: {reason}",
                );
            }
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
