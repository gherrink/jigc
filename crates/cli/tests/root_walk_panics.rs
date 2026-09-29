//! The shared root walker's own arms (M54 Inc 3 T3, S16): a root-walking fence whose
//! root is missing, or yields nothing it reads, **panics** rather than passing having
//! checked nothing. The walker is `support::root_walk`; every root-walking fence over
//! a tracked tree of this repository reads through it.

use std::fs;
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};

use crate::support::root_walk::{ext, files, files_in};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-root-walk-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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

/// The panic message of `f`, which must panic.
fn panic_text(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let payload = catch_unwind(f).expect_err("the walker must panic");
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default()
}

fn write(path: &Path) {
    fs::create_dir_all(path.parent().expect("a parent")).expect("mk parent");
    fs::write(path, "x\n").expect("write a file");
}

#[test]
fn a_missing_root_panics_naming_it() {
    let dir = TempDir::new("missing");
    let root = dir.path().join("gone");
    for walk in [
        Box::new(|r: PathBuf| drop(files(&r, |_| true))) as Box<dyn Fn(PathBuf)>,
        Box::new(|r: PathBuf| drop(files_in(&r, |_| true))),
    ] {
        let r = root.clone();
        let text = panic_text(std::panic::AssertUnwindSafe(move || walk(r)));
        assert!(
            text.contains("is missing") && text.contains(&root.display().to_string()),
            "a missing root must panic naming it; got: {text}",
        );
    }
}

#[test]
fn an_empty_root_panics_naming_it() {
    let dir = TempDir::new("empty");
    let root = dir.path().to_path_buf();
    for walk in [
        Box::new(|r: PathBuf| drop(files(&r, |_| true))) as Box<dyn Fn(PathBuf)>,
        Box::new(|r: PathBuf| drop(files_in(&r, |_| true))),
    ] {
        let r = root.clone();
        let text = panic_text(std::panic::AssertUnwindSafe(move || walk(r)));
        assert!(
            text.contains("yields no file") && text.contains(&root.display().to_string()),
            "an empty root must panic naming it; got: {text}",
        );
    }
}

/// A root holding only files the fence ignores checks as little as an empty one.
#[test]
fn a_root_with_nothing_kept_panics() {
    let dir = TempDir::new("unkept");
    write(&dir.path().join("notes.txt"));
    write(&dir.path().join("sub/deeper.txt"));
    let root = dir.path().to_path_buf();
    let r = root.clone();
    let text = panic_text(move || drop(files(&r, ext("rs"))));
    assert!(text.contains("yields no file"), "got: {text}");
    let text = panic_text(move || drop(files_in(&root, ext("rs"))));
    assert!(text.contains("yields no file"), "got: {text}");
}

/// The green arm: a populated root yields its kept files, sorted — `files` recursing
/// past `target/`, `files_in` staying at one level — and a file root yields itself.
#[test]
fn a_populated_root_yields_its_kept_files_sorted() {
    let dir = TempDir::new("populated");
    let root = dir.path();
    for rel in [
        "b.rs",
        "a.rs",
        "sub/c.rs",
        "target/debug/skip.rs",
        "readme.md",
    ] {
        write(&root.join(rel));
    }
    let rel = |paths: Vec<PathBuf>| -> Vec<String> {
        paths
            .iter()
            .map(|p| p.strip_prefix(root).unwrap().display().to_string())
            .collect()
    };
    assert_eq!(rel(files(root, ext("rs"))), ["a.rs", "b.rs", "sub/c.rs"]);
    assert_eq!(rel(files_in(root, ext("rs"))), ["a.rs", "b.rs"]);
    assert_eq!(
        files(&root.join("readme.md"), |_| true),
        [root.join("readme.md")]
    );
}
