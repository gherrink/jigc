//! The engine's test-only walker for **root-walking fences** over a tracked tree of this
//! repository (M54 S16) — the engine-side counterpart of the cli's
//! `tests/support/root_walk.rs`, which the engine cannot reach: that file sits outside
//! this crate, and naming it by `#[path]` would ship an engine whose own tests cannot
//! build from its package.
//!
//! A fence that walks a root asserts a property of each file it finds, so a root that is
//! missing, or holds nothing the fence reads, lets it pass having checked nothing. Both
//! cases panic here, naming the root. Emptiness is judged after `keep`.

use std::path::{Path, PathBuf};

/// Every file under `root` that `keep` admits, recursively, sorted.
///
/// # Panics
///
/// When `root` is not a directory, and when it yields no file `keep` admits.
pub(crate) fn files(root: &Path, keep: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    assert!(
        root.is_dir(),
        "root_walk: the walked root `{}` is missing — a fence walking it would check \
         nothing and pass",
        root.display(),
    );
    let mut out = Vec::new();
    collect(root, &keep, &mut out);
    assert!(
        !out.is_empty(),
        "root_walk: the walked root `{}` yields no file the fence reads — a fence walking \
         it would check nothing and pass",
        root.display(),
    );
    out.sort();
    out
}

/// A `keep` admitting the files whose extension is `want`.
pub(crate) fn ext(want: &'static str) -> impl Fn(&Path) -> bool {
    move |path| path.extension().is_some_and(|e| e == want)
}

fn collect(dir: &Path, keep: &dyn Fn(&Path) -> bool, out: &mut Vec<PathBuf>) {
    let entries = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("root_walk: read `{}`: {e}", dir.display()));
    for entry in entries {
        let path = entry
            .unwrap_or_else(|e| panic!("root_walk: an entry of `{}`: {e}", dir.display()))
            .path();
        if path.is_dir() {
            collect(&path, keep, out);
        } else if path.is_file() && keep(&path) {
            out.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh directory under the system temp dir, removed on drop.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(tag: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "jigc-engine-root-walk-{tag}-{}-{:?}",
                std::process::id(),
                crate::tempname::unique_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create the scratch dir");
            Scratch(path)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn panic_text(root: PathBuf, keep: fn(&Path) -> bool) -> String {
        let payload = std::panic::catch_unwind(move || drop(files(&root, keep)))
            .expect_err("the walker must panic");
        payload
            .downcast_ref::<String>()
            .cloned()
            .unwrap_or_default()
    }

    #[test]
    fn a_missing_root_panics_naming_it() {
        let dir = Scratch::new("missing");
        let root = dir.0.join("gone");
        let text = panic_text(root.clone(), |_| true);
        assert!(
            text.contains("is missing") && text.contains(&root.display().to_string()),
            "got: {text}"
        );
    }

    #[test]
    fn an_empty_root_panics_naming_it() {
        let dir = Scratch::new("empty");
        let text = panic_text(dir.0.clone(), |_| true);
        assert!(
            text.contains("yields no file") && text.contains(&dir.0.display().to_string()),
            "got: {text}"
        );
    }

    #[test]
    fn a_root_with_nothing_kept_panics() {
        let dir = Scratch::new("unkept");
        std::fs::write(dir.0.join("notes.txt"), "x\n").expect("write a file");
        let text = panic_text(dir.0.clone(), |p| p.extension().is_some_and(|e| e == "rs"));
        assert!(text.contains("yields no file"), "got: {text}");
    }

    #[test]
    fn a_populated_root_yields_its_kept_files_sorted() {
        let dir = Scratch::new("populated");
        std::fs::create_dir_all(dir.0.join("sub")).expect("mk sub");
        for rel in ["b.rs", "a.rs", "sub/c.rs", "readme.md"] {
            std::fs::write(dir.0.join(rel), "x\n").expect("write a file");
        }
        let got: Vec<PathBuf> = files(&dir.0, ext("rs"))
            .into_iter()
            .map(|p| p.strip_prefix(&dir.0).unwrap().to_path_buf())
            .collect();
        assert_eq!(
            got,
            [
                PathBuf::from("a.rs"),
                PathBuf::from("b.rs"),
                PathBuf::from("sub/c.rs")
            ]
        );
    }
}
