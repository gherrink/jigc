//! The one walker every **root-walking fence** reads a tracked tree of this repository
//! through (M54 S16).
//!
//! A root-walking fence asserts a property of each file it finds under a root it names
//! — `crates/cli/src`, a pack's `steps/`, `design/`. When that root is missing, or holds
//! nothing the fence reads, the property holds of every file found, because there are
//! none: the fence passes having checked nothing. That is how a move goes unfenced
//! (M54 Increment 3 moves the packs and the guides), so both cases panic here, naming
//! the root, instead of returning an empty list.
//!
//! **Emptiness is judged after `keep`.** A root holding only files the fence ignores is
//! as vacuous as an empty one, so it panics too. A caller that then filters on content
//! (a step that declares some code) filters the returned list, never `keep`, because
//! finding no match among real files is an answer and finding no files is not.
//!
//! Out of scope: walks of a throwaway repository and helpers that copy a pack into
//! one. A copied empty pack fails pack-load loudly, and a throwaway root is the test's
//! own fixture, not a tracked tree.

use std::path::{Path, PathBuf};

/// Every file under `root` that `keep` admits, recursively, skipping `target/` trees.
/// Sorted, so a fence's offender list is deterministic.
///
/// A `root` that is itself a file yields that file, so a fence may name a single
/// document beside its directory roots.
///
/// # Panics
///
/// When `root` does not exist, and when it yields no file `keep` admits.
pub fn files(root: &Path, keep: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if root.is_file() {
        if keep(root) {
            out.push(root.to_path_buf());
        }
    } else {
        require_dir(root);
        collect(root, &keep, &mut out);
    }
    finish(root, out)
}

/// Every file directly in `root` (no recursion) that `keep` admits. Sorted.
///
/// # Panics
///
/// When `root` is not a directory, and when it yields no file `keep` admits.
pub fn files_in(root: &Path, keep: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    require_dir(root);
    let out = entries(root)
        .into_iter()
        .filter(|path| path.is_file() && keep(path))
        .collect();
    finish(root, out)
}

/// A `keep` admitting the files whose extension is `want`.
pub fn ext(want: &'static str) -> impl Fn(&Path) -> bool {
    move |path| path.extension().is_some_and(|e| e == want)
}

fn require_dir(root: &Path) {
    assert!(
        root.is_dir(),
        "root_walk: the walked root `{}` is missing — a fence walking it would check \
         nothing and pass",
        root.display(),
    );
}

fn finish(root: &Path, mut out: Vec<PathBuf>) -> Vec<PathBuf> {
    assert!(
        !out.is_empty(),
        "root_walk: the walked root `{}` yields no file the fence reads — a fence walking \
         it would check nothing and pass",
        root.display(),
    );
    out.sort();
    out
}

fn entries(dir: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("root_walk: read `{}`: {e}", dir.display()))
        .map(|entry| {
            entry
                .unwrap_or_else(|e| panic!("root_walk: an entry of `{}`: {e}", dir.display()))
                .path()
        })
        .collect()
}

fn collect(dir: &Path, keep: &dyn Fn(&Path) -> bool, out: &mut Vec<PathBuf>) {
    for path in entries(dir) {
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            collect(&path, keep, out);
        } else if path.is_file() && keep(&path) {
            out.push(path);
        }
    }
}
