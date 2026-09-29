//! The **pack-manifest locator** — each pack's freeze manifest found **by pack
//! identity**, never by a fixed path (M54 Increment 3, S16).
//!
//! A fence that reads `crates/cli/pack/config/schema-manifest.yaml` at every
//! revision passes **vacuously** the moment that path stops existing: the base side
//! reads *absent*, the comparator's clean cell, and a hash re-pinned inside the very
//! push that moves the pack ships unchecked ([DECISIONS.md](../../../../DECISIONS.md)
//! → the M54 Settle, S16). So a manifest is found here by what it **is**, not where
//! it sits: a pack's manifest is the `config/schema-manifest.yaml` whose sibling
//! `config/defaults.yaml` carries that pack's `pack-id`.
//!
//! **The three outcomes per pack, each stated:**
//!
//! - **exactly one** — the pack's manifest path at that tree;
//! - **zero** — the pack is *absent there*: it is simply not a key of the map, and a
//!   caller that compares two trees states the absence rather than inventing a
//!   path;
//! - **more than one** — a [`Duplicate`], a fence error. The locator **never picks**:
//!   choosing one of two same-id manifests would let the other re-pin anything.
//!
//! **Excluded:** everything under [`FIXTURES`] — test fixtures carry deliberately
//! reshaped pack copies, and a fixture is not the pack it imitates. A manifest whose
//! sibling `defaults.yaml` names no `pack-id` identifies no pack and is not listed;
//! a `defaults.yaml` that does not parse fails **loudly**, since an unreadable
//! identity is not evidence of no identity.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// A pack's freeze manifest, relative to the pack root.
pub const MANIFEST: &str = "config/schema-manifest.yaml";

/// The sibling that carries the pack's identity, relative to the pack root.
pub const DEFAULTS: &str = "config/defaults.yaml";

/// The tree the locator never looks into: fixtures imitate packs, they are not packs.
pub const FIXTURES: &str = "crates/cli/tests/fixtures/";

/// Which tree of a repo to locate in.
#[derive(Clone, Copy, Debug)]
pub enum Tree<'a> {
    /// A committed revision, read through git — the fence's history side.
    Rev(&'a str),
    /// The working tree as it stands, tracked and untracked-but-not-ignored — the
    /// state about to be committed.
    Working,
}

/// Two or more manifests claiming one pack id in one tree — a fence error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Duplicate {
    /// The pack id claimed more than once.
    pub pack: String,
    /// Every manifest path claiming it, sorted.
    pub paths: Vec<String>,
}

/// Every pack's manifest in `tree`, keyed by pack id: `pack-id → repo-relative
/// manifest path`. A pack absent from the map is **absent from that tree**.
///
/// `Err` carries every pack id claimed by more than one manifest, sorted by id — the
/// fence error, never a pick. A `BTreeMap` throughout, so the result is independent
/// of the order git lists paths in.
pub fn locate(repo: &Path, tree: Tree<'_>) -> Result<BTreeMap<String, String>, Vec<Duplicate>> {
    let mut claims: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for manifest in list(repo, tree)
        .into_iter()
        .filter(|path| !path.starts_with(FIXTURES))
        .filter(|path| path == MANIFEST || path.ends_with(&format!("/{MANIFEST}")))
    {
        let root = &manifest[..manifest.len() - MANIFEST.len()];
        let defaults = format!("{root}{DEFAULTS}");
        let Some(text) = read(repo, tree, &defaults) else {
            continue;
        };
        if let Some(pack) = pack_id(&text, &defaults) {
            claims.entry(pack).or_default().push(manifest);
        }
    }

    let duplicates: Vec<Duplicate> = claims
        .iter()
        .filter(|(_, paths)| paths.len() > 1)
        .map(|(pack, paths)| {
            let mut paths = paths.clone();
            paths.sort();
            Duplicate {
                pack: pack.clone(),
                paths,
            }
        })
        .collect();
    if !duplicates.is_empty() {
        return Err(duplicates);
    }
    Ok(claims
        .into_iter()
        .map(|(pack, mut paths)| (pack, paths.remove(0)))
        .collect())
}

/// One file's bytes in `tree`, or `None` when the path does not exist there.
pub fn read(repo: &Path, tree: Tree<'_>, path: &str) -> Option<String> {
    match tree {
        Tree::Rev(rev) => {
            let out = Command::new("git")
                .args(["show", &format!("{rev}:{path}")])
                .current_dir(repo)
                .output()
                .expect("run git show");
            out.status
                .success()
                .then(|| String::from_utf8(out.stdout).expect("a pack config file is utf-8"))
        }
        Tree::Working => fs::read_to_string(repo.join(path)).ok(),
    }
}

/// Every file path in `tree`, repo-relative.
fn list(repo: &Path, tree: Tree<'_>) -> Vec<String> {
    let args: &[&str] = match tree {
        Tree::Rev(rev) => &["ls-tree", "-r", "--name-only", "-z", rev],
        Tree::Working => &[
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ],
    };
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git to list the tree");
    assert!(
        out.status.success(),
        "git {args:?} failed in {}: {}",
        repo.display(),
        String::from_utf8_lossy(&out.stderr),
    );
    let listing = String::from_utf8(out.stdout).expect("utf-8 paths");
    let paths = listing
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_string);
    match tree {
        Tree::Rev(_) => paths.collect(),
        // The index still lists a file deleted from disk but not yet staged — the
        // working tree does not carry it.
        Tree::Working => paths.filter(|p| repo.join(p).is_file()).collect(),
    }
}

/// The `pack-id` a `defaults.yaml` text declares, or `None` when it declares none.
fn pack_id(text: &str, path: &str) -> Option<String> {
    let value: serde_yaml_ng::Value = serde_yaml_ng::from_str(text).unwrap_or_else(|e| {
        panic!(
            "{path} does not parse ({e}), so the pack identity of its sibling manifest \
             cannot be read — an unreadable identity is not evidence of no identity"
        )
    });
    value.get("pack-id")?.as_str().map(str::to_string)
}
