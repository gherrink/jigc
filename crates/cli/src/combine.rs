//! The off-line **code combine** for a `squash:true` fan-out finalize (M31 Inc 4 / WF2).
//!
//! Each fan-out sub-agent edits + `git add`s its code in an **isolated worktree**
//! ([storage.md](../../../design/storage.md) → the third combine-mode). At finalize the
//! CLI must fold the N worktree-staged code-sets into **one** tree — **disjoint-apply in
//! task-id order, never `git merge`** ([DECISIONS.md](../../../DECISIONS.md) 2026-06-20 →
//! M31 planning, combine = off-line temp-index build; WF2 collision granularity).
//!
//! [`combine_worktree_trees`] is that fold's pure-tree half: given the base tree and the
//! worktree list it either
//! - **blocks** — a rename-aware cross-worktree path collision returns a routed
//!   [`Finding`] naming the colliding path(s), building **no** tree (the join's
//!   never-text-merge discipline, applied to code); or
//! - **combines** — seeds a **temp index** from the base tree, `git apply --cached`s each
//!   worktree's staged patch in **task-id order**, and `git write-tree`s the result,
//!   returning the combined tree sha.
//!
//! The build is **off-line**: every index op is redirected through `GIT_INDEX_FILE` at a
//! throwaway path, so the **live index and working tree are never mutated** — a blocked
//! or failed combine needs no destructive reset, and unrelated main-checkout WIP survives
//! ([DECISIONS.md](../../../DECISIONS.md) 2026-06-20 → review S2, the M30 WIP-safety
//! hazard). The seam returns a **tree sha, never a commit** — the live-mutating
//! `commit-tree` / ref-update / checkout is its caller's job (M31 Inc 4 T2), keeping
//! WIP-safety provable in isolation here.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use engine::finding::{Finding, Location, Severity};

/// The outcome of an off-line combine: either a combined tree sha or a blocking
/// [`Finding`] for a cross-worktree code collision (never both, never a commit).
#[derive(Debug)]
pub enum CombineOutcome {
    /// The combined tree's sha — every worktree's staged code disjoint-applied onto the
    /// base tree, in task-id order.
    Combined(String),
    /// A blocking, route-bearing collision finding naming the contended path(s); no tree
    /// was built and the live index/worktree were never touched.
    Blocked(Finding),
}

/// Fold the `worktrees`' staged code-sets onto `base_tree` into one combined tree.
///
/// `repo_root` is the **main checkout** (the shared object database all the worktrees
/// write into); `base_tree` is the tree-ish the fan-out was pinned at; `worktrees` are the
/// per-sub-task worktree paths (each path's **final component is the task id**, so the
/// fold sorts by it — the result is **byte-identical regardless of input order**, the
/// determinism bar).
///
/// Returns [`CombineOutcome::Blocked`] when two worktrees touch the same path (a stage, a
/// delete, or **either** side of a rename — the block-set is rename-aware), else
/// [`CombineOutcome::Combined`]. Errors only on a genuine git/IO fault.
pub fn combine_worktree_trees(
    repo_root: &Path,
    base_tree: &str,
    worktrees: &[PathBuf],
) -> Result<CombineOutcome> {
    // Apply strictly in task-id order (the worktree dir's final component); the result is
    // then byte-identical regardless of how the caller ordered the input list.
    let mut ordered: Vec<&PathBuf> = worktrees.iter().collect();
    ordered.sort_by_key(|wt| task_id(wt));

    // Collision detect first — a non-empty cross-worktree path intersection blocks before
    // anything is built. `touched`: path -> the task ids that touch it, both sorted (so
    // the finding message is byte-stable across input orders, hardening #7).
    let mut touched: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for wt in &ordered {
        let id = task_id(wt);
        for path in block_set(wt)? {
            touched.entry(path).or_default().insert(id.clone());
        }
    }
    let colliding: Vec<(&String, &BTreeSet<String>)> =
        touched.iter().filter(|(_, ids)| ids.len() > 1).collect();
    if !colliding.is_empty() {
        return Ok(CombineOutcome::Blocked(collision_finding(&colliding)));
    }

    // No collision — build the combined tree off-line in a throwaway index, never
    // touching the live index/worktree.
    let index = TempIndex::new();
    git_with_index(repo_root, index.path(), &["read-tree", base_tree], None)
        .context("seed the combine index from the base tree")?;
    for wt in &ordered {
        let patch = git_stdout(wt, &["diff", "--cached", "--binary", "--no-renames"])
            .context("read a worktree's staged patch")?;
        if patch.is_empty() {
            continue; // a worktree with nothing staged contributes nothing.
        }
        git_with_index(
            repo_root,
            index.path(),
            &["apply", "--cached", "--whitespace=nowarn"],
            Some(&patch),
        )
        .context("disjoint-apply a worktree's staged patch into the combine index")?;
    }
    let tree = git_with_index(repo_root, index.path(), &["write-tree"], None)
        .context("write the combined tree")?;
    let tree = String::from_utf8(tree)
        .context("`git write-tree` produced non-UTF-8 output")?
        .trim()
        .to_string();
    Ok(CombineOutcome::Combined(tree))
}

/// A worktree's task id — its path's final component (the `.jigc/worktrees/<id>`
/// convention; [`engine::milestone::worktree_path`]).
fn task_id(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// A worktree's **block-set** — every path its staged changes touch, read from a
/// **rename-aware** `git diff --cached --name-status -M`: the union of {staged path,
/// rename old-path, delete path}. Rename-awareness is load-bearing — a rename's *old*
/// path must enter the set or a collision against another worktree's edit of that path
/// slips the guard ([DECISIONS.md](../../../DECISIONS.md) 2026-06-20 → WF2 collision
/// granularity).
fn block_set(worktree: &Path) -> Result<BTreeSet<String>> {
    let out = git_stdout(
        worktree,
        &["diff", "--cached", "--name-status", "--find-renames"],
    )
    .context("read a worktree's staged name-status")?;
    let text = String::from_utf8(out).context("`git diff --name-status` produced non-UTF-8")?;
    let mut set = BTreeSet::new();
    for line in text.lines() {
        let mut fields = line.split('\t');
        let Some(status) = fields.next() else {
            continue;
        };
        // R<score> / C<score> carry an old (source) and a new (dest) path — both block;
        // A / M / D carry a single path.
        if status.starts_with('R') || status.starts_with('C') {
            if let Some(old) = fields.next() {
                set.insert(old.to_string());
            }
            if let Some(new) = fields.next() {
                set.insert(new.to_string());
            }
        } else if let Some(path) = fields.next() {
            set.insert(path.to_string());
        }
    }
    Ok(set)
}

/// A blocking, route-bearing finding naming the cross-worktree colliding path(s) and the
/// contending sub-tasks. The combine never text-merges code — the join's never-blind-merge
/// discipline ([storage.md](../../../design/storage.md)) applied to the code substrate.
fn collision_finding(colliding: &[(&String, &BTreeSet<String>)]) -> Finding {
    let listing: Vec<String> = colliding
        .iter()
        .map(|(path, ids)| {
            let tasks: Vec<&str> = ids.iter().map(String::as_str).collect();
            format!("`{path}` (sub-tasks [{}])", tasks.join(", "))
        })
        .collect();
    let address = colliding[0].0;
    Finding::graded(
        Severity::Blocking,
        "combine.code-collision",
        format!(
            "code collision — {} staged by more than one worktree; the combine \
             disjoint-applies code and never text-merges a shared file",
            listing.join(", ")
        ),
        Some(Location::addressed(address, 1, 1)),
        Some(
            "have the contending sub-tasks touch distinct files, or combine their \
             overlapping changes by hand"
                .to_string(),
        ),
    )
}

/// Run `git <args>` in `dir`, returning raw stdout (binary-safe — patches are not UTF-8).
fn git_stdout(dir: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    Ok(out.stdout)
}

/// Run `git <args>` in `dir` with `GIT_INDEX_FILE` redirected to `index` (the off-line
/// build's throwaway index — the live index is never touched), optionally feeding `stdin`.
fn git_with_index(
    dir: &Path,
    index: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> Result<Vec<u8>> {
    let mut cmd = Command::new("git");
    cmd.args(args)
        .current_dir(dir)
        .env("GIT_INDEX_FILE", index)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        });
    let mut child = cmd
        .spawn()
        .context("could not run `git` (is it on PATH?)")?;
    if let Some(data) = stdin {
        // Take the handle out and drop it after the write so git sees EOF.
        child
            .stdin
            .take()
            .expect("stdin was piped")
            .write_all(data)
            .context("write the patch to `git apply`")?;
    }
    let out = child
        .wait_with_output()
        .context("wait for `git` to finish")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    Ok(out.stdout)
}

/// A throwaway index file for the off-line build, removed on drop — never the live
/// `.git/index`, so the combine cannot mutate the main checkout's staging state.
struct TempIndex(PathBuf);

impl TempIndex {
    fn new() -> Self {
        let name = format!(
            "jigc-combine-index-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default(),
        );
        let path = std::env::temp_dir().join(name);
        // `git read-tree` creates the file; clear any stale leftover first.
        let _ = std::fs::remove_file(&path);
        TempIndex(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempIndex {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway directory that removes itself (and any worktrees nested under it) on
    /// drop — the project's no-tempfile pattern (mirrors `crate::repo` tests).
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-combine-test-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            // Detach worktrees first so `remove_dir_all` does not race git's admin files.
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// Run `git`, asserting success, returning trimmed stdout. Isolated from the host's
    /// user/global git config so the suite is deterministic.
    fn git(dir: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@t")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@t")
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    fn write(dir: &Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mkdir");
        }
        std::fs::write(p, body).expect("write file");
    }

    /// Init a main checkout with a base commit carrying the given files, returning
    /// `(main_path, base_tree_sha)`.
    fn init_main(td: &TempDir, base_files: &[(&str, &str)]) -> (PathBuf, String) {
        let main = td.path().join("main");
        std::fs::create_dir_all(&main).expect("mkdir main");
        git(&main, &["init", "-q"]);
        git(&main, &["config", "user.email", "t@t"]);
        git(&main, &["config", "user.name", "t"]);
        for (rel, body) in base_files {
            write(&main, rel, body);
        }
        git(&main, &["add", "-A"]);
        git(&main, &["commit", "-q", "-m", "base"]);
        let tree = git(&main, &["rev-parse", "HEAD^{tree}"]);
        (main, tree)
    }

    /// Add a detached worktree named `id` (so its path's final component is the task id)
    /// at the base, returning its path.
    fn add_worktree(main: &Path, td: &TempDir, id: &str) -> PathBuf {
        let path = td.path().join("worktrees").join(id);
        std::fs::create_dir_all(path.parent().unwrap()).expect("mkdir worktrees");
        git(
            main,
            &[
                "worktree",
                "add",
                "-q",
                "--detach",
                path.to_str().unwrap(),
                "HEAD",
            ],
        );
        path
    }

    /// Collect a tree's `path -> blob-sha` map via `git ls-tree -r`.
    fn ls_tree(main: &Path, tree: &str) -> BTreeMap<String, String> {
        let out = git(main, &["ls-tree", "-r", tree]);
        out.lines()
            .map(|line| {
                // "<mode> blob <sha>\t<path>"
                let (meta, path) = line.split_once('\t').expect("ls-tree tab");
                let sha = meta
                    .split_whitespace()
                    .nth(2)
                    .expect("blob sha")
                    .to_string();
                (path.to_string(), sha)
            })
            .collect()
    }

    /// (a) Two worktrees with **disjoint** staged sets combine into one tree carrying
    /// both worktrees' blobs **plus** the base tree's files (drops none) — and unrelated
    /// main-checkout WIP survives the (successful) off-line build untouched (review S2).
    #[test]
    fn combines_disjoint_staged_sets_dropping_none() {
        let td = TempDir::new();
        let (main, base_tree) = init_main(&td, &[("base.txt", "base\n")]);

        let wt_a = add_worktree(&main, &td, "task-aaa");
        write(&wt_a, "alpha.rs", "fn a() {}\n");
        git(&wt_a, &["add", "alpha.rs"]);

        let wt_b = add_worktree(&main, &td, "task-bbb");
        write(&wt_b, "beta.rs", "fn b() {}\n");
        git(&wt_b, &["add", "beta.rs"]);

        // Seed unrelated WIP in the MAIN checkout: an untracked file + an unstaged edit
        // to a tracked file. The off-line build must not touch either.
        write(&main, "wip-untracked.txt", "scratch\n");
        write(&main, "base.txt", "base + local WIP\n");
        let status_before = git(&main, &["status", "--porcelain"]);

        let outcome = combine_worktree_trees(&main, &base_tree, &[wt_a.clone(), wt_b.clone()])
            .expect("combine succeeds");
        let tree = match outcome {
            CombineOutcome::Combined(sha) => sha,
            CombineOutcome::Blocked(f) => panic!("disjoint sets must combine, got {f:?}"),
        };

        let entries = ls_tree(&main, &tree);
        assert!(entries.contains_key("base.txt"), "base file kept");
        assert!(
            entries.contains_key("alpha.rs"),
            "worktree A's staged blob kept"
        );
        assert!(
            entries.contains_key("beta.rs"),
            "worktree B's staged blob kept"
        );
        // The combined tree carries the BASE blob of base.txt (the main checkout's WIP
        // edit never entered the off-line build).
        let base_entries = ls_tree(&main, &base_tree);
        assert_eq!(
            entries.get("base.txt"),
            base_entries.get("base.txt"),
            "combine took the base blob, not the main checkout's WIP edit",
        );

        // WIP survives untouched.
        assert_eq!(
            std::fs::read_to_string(main.join("wip-untracked.txt")).unwrap(),
            "scratch\n",
        );
        assert_eq!(
            std::fs::read_to_string(main.join("base.txt")).unwrap(),
            "base + local WIP\n",
        );
        assert_eq!(
            git(&main, &["status", "--porcelain"]),
            status_before,
            "the live index/worktree are byte-identical after an off-line combine",
        );
    }

    /// (b) Two worktrees touching the **same path** — one of them via a **rename** of it
    /// (the rename's old-path is in the block-set) — block with a finding naming the
    /// collision; no tree is built.
    #[test]
    fn blocks_same_path_collision_including_a_rename() {
        let td = TempDir::new();
        let (main, base_tree) = init_main(&td, &[("shared.txt", "shared\n")]);

        // Worktree A modifies shared.txt.
        let wt_a = add_worktree(&main, &td, "task-aaa");
        write(&wt_a, "shared.txt", "shared, edited by A\n");
        git(&wt_a, &["add", "shared.txt"]);

        // Worktree B RENAMES shared.txt -> moved.txt (its old-path collides with A).
        let wt_b = add_worktree(&main, &td, "task-bbb");
        git(&wt_b, &["mv", "shared.txt", "moved.txt"]);

        let outcome = combine_worktree_trees(&main, &base_tree, &[wt_a, wt_b])
            .expect("collision detection runs cleanly");
        match outcome {
            CombineOutcome::Blocked(f) => {
                assert_eq!(f.severity, Severity::Blocking);
                assert_eq!(f.code, "combine.code-collision");
                assert!(
                    f.message.contains("shared.txt"),
                    "the finding names the colliding path, got: {}",
                    f.message,
                );
                assert!(f.route.is_some(), "a block carries a repair route");
            }
            CombineOutcome::Combined(t) => {
                panic!("a same-path collision (incl. a rename old-path) must block, got tree {t}")
            }
        }
    }

    /// (c) The combined tree is byte-identical regardless of the worktree-list input
    /// ordering (the fold sorts by task id — the determinism bar, hardening #7).
    #[test]
    fn combined_tree_is_byte_identical_across_input_orders() {
        let td = TempDir::new();
        let (main, base_tree) = init_main(&td, &[("base.txt", "base\n")]);

        let wt_a = add_worktree(&main, &td, "task-aaa");
        write(&wt_a, "alpha.rs", "fn a() {}\n");
        git(&wt_a, &["add", "alpha.rs"]);

        let wt_b = add_worktree(&main, &td, "task-bbb");
        write(&wt_b, "beta.rs", "fn b() {}\n");
        git(&wt_b, &["add", "beta.rs"]);

        let forward = combine_worktree_trees(&main, &base_tree, &[wt_a.clone(), wt_b.clone()])
            .expect("combine (forward)");
        let reverse =
            combine_worktree_trees(&main, &base_tree, &[wt_b, wt_a]).expect("combine (reverse)");
        let (f, r) = match (forward, reverse) {
            (CombineOutcome::Combined(f), CombineOutcome::Combined(r)) => (f, r),
            other => panic!("both orders must combine, got {other:?}"),
        };
        assert_eq!(f, r, "the combined tree sha must be order-invariant");
    }

    /// (d) Unrelated main-checkout WIP is byte-identical untouched after a **blocked**
    /// call (the off-line build never even runs on a collision — review S2 literal).
    #[test]
    fn wip_survives_a_blocked_combine() {
        let td = TempDir::new();
        let (main, base_tree) = init_main(&td, &[("shared.txt", "shared\n")]);

        let wt_a = add_worktree(&main, &td, "task-aaa");
        write(&wt_a, "shared.txt", "A\n");
        git(&wt_a, &["add", "shared.txt"]);
        let wt_b = add_worktree(&main, &td, "task-bbb");
        write(&wt_b, "shared.txt", "B\n");
        git(&wt_b, &["add", "shared.txt"]);

        write(&main, "wip-untracked.txt", "scratch\n");
        write(&main, "shared.txt", "local WIP on shared\n");
        let status_before = git(&main, &["status", "--porcelain"]);

        let outcome =
            combine_worktree_trees(&main, &base_tree, &[wt_a, wt_b]).expect("collision detect");
        assert!(
            matches!(outcome, CombineOutcome::Blocked(_)),
            "a same-path collision must block",
        );

        assert_eq!(
            std::fs::read_to_string(main.join("wip-untracked.txt")).unwrap(),
            "scratch\n",
        );
        assert_eq!(
            std::fs::read_to_string(main.join("shared.txt")).unwrap(),
            "local WIP on shared\n",
        );
        assert_eq!(
            git(&main, &["status", "--porcelain"]),
            status_before,
            "a blocked combine leaves the live index/worktree byte-identical",
        );
    }
}
