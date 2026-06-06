//! `engine::ingest` — repo-wide candidate discovery for the existing-project
//! ingestion flow (the brownfield detect-and-route slice).
//!
//! See `design/project-setup.md` → Flow 2 (the bounded slice: root + `docs/` +
//! every schema `location:` dir) and `implementation/roadmap.md` (M9 increment 2).
//!
//! ## Why this is net-new
//!
//! Every existing committed-store sweep ([`crate::index::rebuild_committed`],
//! [`crate::file_state::reconcile_committed_store`]) globs **only** the declared
//! schema `location:` dirs — it never scans the repo root or `docs/`. Ingestion is
//! the *forward* problem: an arbitrary foreign `.md` with no type binding. So
//! discovery must reach beyond the location dirs.
//!
//! ## The bounded slice
//!
//! Discovery is conventional, **non-recursive** file-walking over a fixed, bounded
//! set of directories — the repo root, `docs/`, and every schema's `location:` dir.
//! Each directory is read flat (matching the existing per-location `read_dir`
//! discipline); only `*.md` files are collected. `.jigc/` internals are excluded.
//! The result is the repo-relative path list, **sorted** and **deduped** (the same
//! schema may appear at a dir already covered, e.g. `docs/` as a `location:`), so
//! the verdict order downstream is deterministic — same repo in, same list out.

use std::collections::BTreeSet;
use std::path::Path;

use crate::schema::Schema;

/// The `.jigc/` internals dir — never a candidate source.
const INTERNALS_DIR: &str = ".jigc";

/// Discover the sorted, deduped repo-relative `.md` candidate set across the repo
/// root, `docs/`, and every schema `location:` dir, excluding `.jigc/` internals.
///
/// Each directory is scanned **flat** (non-recursive); a missing directory
/// contributes nothing (no committed docs of that shape yet). Paths are returned as
/// forward-slash repo-relative strings, sorted lexicographically and deduped — a
/// `location:` dir that coincides with the root or `docs/` yields each file once.
///
/// `schemas` supplies the `location:` dirs; a transient (location-less) type
/// contributes none. The walk is deterministic: the `BTreeSet` makes the output
/// order independent of `read_dir`'s natural (filesystem) order.
pub fn discover_candidates(repo_root: &Path, schemas: &[Schema]) -> Vec<String> {
    // The bounded dir set: root (the empty relative prefix), `docs/`, and every
    // declared `location:`. A `BTreeSet` dedups dirs that coincide (e.g. `docs/`
    // declared as a `location:`).
    let mut dirs: BTreeSet<&str> = BTreeSet::new();
    dirs.insert(""); // the repo root itself.
    dirs.insert("docs");
    for schema in schemas {
        if let Some(location) = schema.location.as_deref() {
            // Normalize a trailing slash off the declared `location:` (`decisions/`).
            dirs.insert(location.trim_end_matches('/'));
        }
    }

    // A `BTreeSet` collects the candidates, so the output is sorted + deduped
    // regardless of the per-dir `read_dir` order (the hardening-#7 determinism
    // property: same repo in, byte-identical list out).
    let mut candidates: BTreeSet<String> = BTreeSet::new();
    for dir in dirs {
        let abs = if dir.is_empty() {
            repo_root.to_path_buf()
        } else {
            repo_root.join(dir)
        };
        let Ok(entries) = std::fs::read_dir(&abs) else {
            continue; // dir absent: contributes nothing.
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            if !path.is_file() {
                continue; // a `*.md` directory is not a candidate.
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let rel = if dir.is_empty() {
                name.to_string()
            } else {
                format!("{dir}/{name}")
            };
            // Guard the `.jigc/` exclusion even if a stray location pointed inside it.
            if rel == INTERNALS_DIR || rel.starts_with(&format!("{INTERNALS_DIR}/")) {
                continue;
            }
            candidates.insert(rel);
        }
    }

    candidates.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    /// A throwaway directory that removes itself on drop — keeps ingest tests off any
    /// real repo tree.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-ingest-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp root");
            TempRoot(path)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn adr_schema() -> Schema {
        crate::schema::load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    fn write(root: &Path, rel: &str, body: &str) {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("mk parent dir");
        }
        std::fs::write(path, body).expect("write candidate");
    }

    /// The four candidate shapes the increment fixtures around: a doc at the
    /// `location:` dir, one at the repo root, one under `docs/`, plus noise (a
    /// non-`.md` file, a `.jigc/` internal) that must be excluded.
    fn write_four_file_fixture(root: &Path) {
        write(root, "decisions/single-node-cache.md", "adr at location");
        write(root, "README.md", "root readme");
        write(root, "docs/architecture.md", "docs note");
        write(root, "unmanaged.md", "freeform root doc");
        // Noise that must NOT be discovered:
        write(root, "Cargo.toml", "not markdown");
        write(root, ".jigc/AGENT.md", "internal adapter, excluded");
        write(root, "decisions/notes.txt", "not markdown");
    }

    #[test]
    fn discovers_the_exact_sorted_candidate_set() {
        let root = TempRoot::new("exact-set");
        write_four_file_fixture(root.path());

        let got = discover_candidates(root.path(), &[adr_schema()]);

        assert_eq!(
            got,
            vec![
                "README.md".to_string(),
                "decisions/single-node-cache.md".to_string(),
                "docs/architecture.md".to_string(),
                "unmanaged.md".to_string(),
            ],
        );
    }

    /// Determinism (hardening #7): a fixture whose natural `read_dir` order differs
    /// from sorted order must still yield the byte-identical sorted list. The names
    /// are chosen so the filesystem's natural enumeration is unlikely to be sorted,
    /// and the assertion is byte-for-byte against the sorted expectation.
    #[test]
    fn yields_byte_identical_sorted_list_regardless_of_read_dir_order() {
        let root = TempRoot::new("determinism");
        // Insert in a deliberately scrambled order across multiple dirs.
        write(root.path(), "zebra.md", "z");
        write(root.path(), "alpha.md", "a");
        write(root.path(), "decisions/yak.md", "y");
        write(root.path(), "decisions/ant.md", "n");
        write(root.path(), "docs/mid.md", "m");
        write(root.path(), "docs/aardvark.md", "k");

        let got = discover_candidates(root.path(), &[adr_schema()]);

        let expected = vec![
            "alpha.md".to_string(),
            "decisions/ant.md".to_string(),
            "decisions/yak.md".to_string(),
            "docs/aardvark.md".to_string(),
            "docs/mid.md".to_string(),
            "zebra.md".to_string(),
        ];
        // Byte-identical: the produced join must equal the sorted-expected join.
        assert_eq!(got, expected);
        assert_eq!(got.join("\n"), expected.join("\n"));
    }

    /// A `docs/` dir that is *also* a declared `location:` yields each file once —
    /// the dedup is real, not incidental.
    #[test]
    fn dedups_a_location_that_coincides_with_a_walked_dir() {
        let root = TempRoot::new("dedup");
        write(
            root.path(),
            "docs/overlap.md",
            "lives in docs and is the location",
        );

        // A schema whose location IS `docs/` — the dir is walked twice, file once.
        let mut schema = adr_schema();
        schema.location = Some("docs/".to_string());

        let got = discover_candidates(root.path(), &[schema]);

        assert_eq!(got, vec!["docs/overlap.md".to_string()]);
    }
}
