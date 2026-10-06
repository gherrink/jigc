//! Where this workspace's integration suites live — **derived from the `[[test]]`
//! targets `crates/cli/Cargo.toml` declares**, never listed.
//!
//! A *home* is a directory that holds suite files (`<home>/*.rs`) and the group roots that
//! compile them (`<home>/groups/*.rs`). There are two since 2026-10-06: the product's,
//! `crates/cli/tests/`, and the repository's own tooling suites', outside `crates/`
//! ([dev-workflow.md](../../../../implementation/dev-workflow.md) → Gate, *Where the
//! tooling's own tests live*). Until then "every suite file" and "every test file" meant
//! `crates/cli/tests/` by assumption, in the registration fence and in the source fences
//! that walk the crates; a suite moved out of that directory left each of them without a
//! word. So the set is read where it is decided — the manifest's `[[test]]` paths — and a
//! third home joins every reader by its manifest entry.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

/// One `[[test]]` target of the `jigc` package.
pub struct TestTarget {
    /// The target's name — the group, and the binary nextest reports.
    pub name: String,
    /// Its root file, absolute and free of `..`.
    pub root: PathBuf,
}

/// The `jigc` package's directory, canonical: the base every path here is built from, so
/// two paths to one file compare equal.
fn crate_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .expect("the cli crate's directory is reachable")
}

/// `path` with every `.` dropped and every `..` resolved against the component before it
/// — lexically, so a path to a file that does not exist still has one spelling.
pub fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

/// The `[[test]]` targets the manifest declares, in declaration order — the authority on
/// what `cargo test` compiles.
pub fn declared_test_targets() -> Vec<TestTarget> {
    let dir = crate_dir();
    let body =
        fs::read_to_string(dir.join("Cargo.toml")).expect("crates/cli/Cargo.toml is readable");
    let mut out: Vec<TestTarget> = Vec::new();
    let mut current: Option<(Option<String>, Option<String>)> = None;
    let flush = |cur: Option<(Option<String>, Option<String>)>, out: &mut Vec<TestTarget>| {
        if let Some((name, path)) = cur {
            out.push(TestTarget {
                name: name.expect("a [[test]] target declares a name"),
                root: normalize(&dir.join(path.expect("a [[test]] target declares a path"))),
            });
        }
    };
    for line in body.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            flush(current.take(), &mut out);
            if line == "[[test]]" {
                current = Some((None, None));
            }
            continue;
        }
        if let Some((name, path)) = current.as_mut()
            && let Some((key, value)) = line.split_once('=')
        {
            let value = value.trim().trim_matches('"').to_string();
            match key.trim() {
                "name" => *name = Some(value),
                "path" => *path = Some(value),
                _ => {}
            }
        }
    }
    flush(current.take(), &mut out);
    out
}

/// The home a group root belongs to: the directory above its `groups/`.
///
/// # Panics
///
/// When the root does not sit in a directory named `groups` — the one layout the
/// registration fence knows how to read, so a target declared in any other shape is
/// named here instead of being fenced by nothing.
pub fn home_of(target: &TestTarget) -> PathBuf {
    let groups = target.root.parent().expect("a group root has a parent");
    assert!(
        groups.file_name().is_some_and(|name| name == "groups"),
        "`[[test]]` target `{}` is rooted at {}, which is not `<home>/groups/<root>.rs` — \
         the layout `tests/test_target_registration.rs` reads a suite home by",
        target.name,
        target.root.display(),
    );
    groups
        .parent()
        .expect("a `groups/` directory has a parent")
        .to_path_buf()
}

/// Every suite home, sorted — each directory some declared `[[test]]` root sits under.
pub fn suite_homes() -> Vec<PathBuf> {
    let homes: BTreeSet<PathBuf> = declared_test_targets().iter().map(home_of).collect();
    homes.into_iter().collect()
}

/// The suite homes that lie outside `crates/` — what a fence that walks the crates' trees
/// must walk as well to keep reading every test file. Canonical, like [`suite_homes`].
pub fn homes_outside_the_crates() -> Vec<PathBuf> {
    let crates = normalize(&crate_dir().join(".."));
    suite_homes()
        .into_iter()
        .filter(|home| !home.starts_with(&crates))
        .collect()
}
