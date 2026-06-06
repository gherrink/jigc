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

use crate::parse::parse_sections;
use crate::schema::Schema;
use crate::validate::schema_conformance;

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

/// The location-aware verdict for one discovered candidate (`project-setup.md` →
/// Flow 2, the N-candidate classifier). **Location is the discriminator**: the
/// committed store only ever *finds* a managed doc by globbing its schema's
/// `location:` dir, so a conformant doc at the wrong home is invisible to every
/// later sweep and is therefore *not* adoptable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Parses conformant against exactly that schema **and** already sits under
    /// that schema's `location:` dir — the only safe-to-register shape.
    Adoptable {
        /// The doc-type the file conforms to (and whose `location:` it sits under).
        ty: String,
    },
    /// Under a schema's `location:` dir but fails parse/conformance (a near-miss),
    /// **or** conformant against a schema but sitting at the wrong location. Routed
    /// to a human; never auto-relocated.
    NeedsReconcile,
    /// Outside every `location:` dir and conformant against nothing. Left untouched.
    Unmanaged,
}

/// Classify one discovered candidate against the persisted `schemas`, with location
/// as the discriminator (`project-setup.md` → Flow 2, bullet 2).
///
/// `rel_path` is the forward-slash repo-relative path [`discover_candidates`] yields;
/// `source` is the file's raw bytes as text. Each schema is run through
/// [`parse_sections`] + [`schema_conformance`] (binary parse-or-not, no fuzzy
/// mapping); a clean pass means the file conforms to that schema. The verdict then
/// reduces on whether the path sits under *some* schema's `location:` dir:
///
/// - **under** a schema `S`'s location → `Adoptable{S}` if it conforms to `S`, else
///   `NeedsReconcile` (a near-miss in the location dir, or a doc claiming a different
///   type than its home).
/// - **outside** every location dir → `NeedsReconcile` if it conforms to *some*
///   schema (a conformant doc at the wrong home — the location discriminator demotes
///   it from adoptable), else `Unmanaged`.
pub fn classify(rel_path: &str, source: &str, schemas: &[Schema]) -> Verdict {
    // The home schema: the one whose `location:` dir this path sits under, if any.
    // Schemas are scanned in declared order; a path lives under at most one
    // location dir in practice (locations are distinct dirs).
    let home = schemas.iter().find(|s| under_location(rel_path, s));

    // The schema (if any) this file parses conformant against. The first conformant
    // match suffices for the location-keyed reduction below.
    let conformant = schemas.iter().find(|s| conforms(s, source));

    match home {
        Some(home_schema) => {
            // Under a location dir: adoptable iff it conforms to *that* schema;
            // otherwise a near-miss (or wrong-type claim) routed to a human.
            if conforms(home_schema, source) {
                Verdict::Adoptable {
                    ty: home_schema.ty.clone(),
                }
            } else {
                Verdict::NeedsReconcile
            }
        }
        // Outside every location dir: a conformant doc is at the wrong home
        // (needs-reconcile, never adoptable); a doc conformant against nothing is
        // unmanaged.
        None => match conformant {
            Some(_) => Verdict::NeedsReconcile,
            None => Verdict::Unmanaged,
        },
    }
}

/// Whether `rel_path` sits directly under `schema`'s declared `location:` dir.
///
/// `location:` is matched as a `dir/` path prefix (`Schema.location`, normalized of
/// its trailing slash); a transient (location-less) schema is never a home.
fn under_location(rel_path: &str, schema: &Schema) -> bool {
    let Some(location) = schema.location.as_deref() else {
        return false;
    };
    let dir = location.trim_end_matches('/');
    !dir.is_empty() && rel_path.starts_with(&format!("{dir}/"))
}

/// Whether `source` parses *and* conforms cleanly against `schema` — the binary
/// parse-or-not gate (`parsing.md`): a non-empty parse-or-conformance finding set is
/// a non-conformance.
fn conforms(schema: &Schema, source: &str) -> bool {
    match parse_sections(schema, source) {
        Ok(doc) => schema_conformance(schema, source, &doc).is_empty(),
        Err(_) => false,
    }
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

    /// A conformant ADR body — the exact human-editable bytes the committed-store
    /// fixtures use (front-matter `status`/`date`, the three prose slot sections).
    const CONFORMANT_ADR: &str = "\
---
status: accepted
date: 2026-05-23
---

# Single-node session cache

## Context
Session lookups must stay sub-millisecond.

## Decision
A single in-memory node keeps lookups fast and avoids a network hop.

## Consequences
A cold node loses its sessions; clients re-authenticate.
";

    /// A near-miss freeform doc dropped into `decisions/`: no front-matter and none
    /// of the required ADR sections, so it fails parse/conformance against `adr`.
    const NON_CONFORMANT_NEAR_MISS: &str = "\
# Some loose notes

A few thoughts that are not an ADR at all.
";

    /// The four-shape classifier fixture: a conformant adr **at** its location, a
    /// non-conformant near-miss **in** the location dir, the same conformant adr at
    /// the **wrong** location (`docs/`), and a freeform doc conformant against
    /// nothing. The four verdicts must come out as Adoptable / NeedsReconcile /
    /// NeedsReconcile / Unmanaged respectively.
    #[test]
    fn classifies_the_four_candidate_shapes_with_location_as_discriminator() {
        let schemas = [adr_schema()];

        // 1. Conformant adr, under its `decisions/` location → Adoptable{adr}.
        assert_eq!(
            classify("decisions/single-node-cache.md", CONFORMANT_ADR, &schemas),
            Verdict::Adoptable {
                ty: "adr".to_string()
            },
        );

        // 2. Non-conformant near-miss, under `decisions/` → NeedsReconcile (claims
        //    the location's type, fails the parse/conformance gate).
        assert_eq!(
            classify("decisions/notes.md", NON_CONFORMANT_NEAR_MISS, &schemas),
            Verdict::NeedsReconcile,
        );

        // 3. Conformant adr at the WRONG location (`docs/`) → NeedsReconcile. This is
        //    the discriminator case: identical bytes to (1), but location alone
        //    demotes it from Adoptable — it is invisible to the `decisions/`-globbing
        //    committed-store sweep, so registering it where it sits would lose it.
        assert_eq!(
            classify("docs/architecture.md", CONFORMANT_ADR, &schemas),
            Verdict::NeedsReconcile,
        );

        // 4. Freeform doc, outside every location dir, conformant against nothing →
        //    Unmanaged.
        assert_eq!(
            classify("unmanaged.md", NON_CONFORMANT_NEAR_MISS, &schemas),
            Verdict::Unmanaged,
        );
    }

    /// The discriminator is load-bearing on its own: the *same conformant bytes*
    /// classify differently purely on path — Adoptable at the location, NeedsReconcile
    /// off it. (Guards against an implementation that decides on conformance alone.)
    #[test]
    fn identical_conformant_bytes_split_on_location_alone() {
        let schemas = [adr_schema()];
        let at_home = classify("decisions/x.md", CONFORMANT_ADR, &schemas);
        let off_home = classify("notes/x.md", CONFORMANT_ADR, &schemas);
        assert_eq!(
            at_home,
            Verdict::Adoptable {
                ty: "adr".to_string()
            },
        );
        assert_eq!(off_home, Verdict::NeedsReconcile);
        assert_ne!(at_home, off_home);
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
