//! `engine::ingest` — repo-wide candidate discovery for the existing-project
//! ingestion flow (the brownfield detect-and-route slice).
//!
//! See `design/project-setup.md` → Flow 2 + Flow 2 hardening (the recursive scan,
//! G3) and `implementation/roadmap.md` (M9 increment 2, M21 increment 3).
//!
//! ## Why this is net-new
//!
//! Every existing committed-store sweep ([`crate::index::rebuild_committed`],
//! [`crate::file_state::reconcile_committed_store`]) globs **only** the declared
//! schema `location:` dirs — it never scans the repo root or `docs/`. Ingestion is
//! the *forward* problem: an arbitrary foreign `.md` with no type binding. So
//! discovery must reach beyond the location dirs.
//!
//! ## The recursive walk
//!
//! Discovery **recursively** walks the repo from the root down, collecting every
//! `*.md` file, so docs in `wiki/`, nested `docs/sub/`, `rfcs/`, etc. are reachable
//! (M21 G3 — M9's non-recursive top-level scan missed them). The internals dirs
//! (`.jigc/`, `.git/`) are pruned by name. The result is the repo-relative path list,
//! **sorted** and **deduped**, so the verdict order downstream is deterministic —
//! same repo in, same list out.

use std::collections::BTreeSet;
use std::path::Path;

use crate::file_state::{FileStateRecord, hash_bytes};
use crate::finding::Finding;
use crate::index::EdgeIndex;
use crate::parse::parse_sections;
use crate::schema::Schema;
use crate::validate::schema_conformance;

/// Internals dirs — never a candidate source, never descended into. `.jigc/` is the
/// adapter/state home; `.git/` is the VCS internals (its hooks/templates carry `.md`).
const INTERNALS_DIRS: [&str; 2] = [".jigc", ".git"];

/// Discover the sorted, deduped repo-relative `.md` candidate set by **recursively**
/// walking the repo from `repo_root` down, excluding the internals dirs (`.jigc/`,
/// `.git/`).
///
/// The walk descends into every subdirectory (`wiki/`, nested `docs/sub/`, `rfcs/`,
/// …) so a real repo's docs-elsewhere corpus is reachable — only `*.md` files are
/// collected. Paths are returned as forward-slash repo-relative strings, sorted
/// lexicographically and deduped. The internals dirs are pruned by name at the top
/// level, so nothing under them is ever read.
///
/// `schemas` is unused by the recursive walk (recursion reaches every dir a
/// `location:` could name) but kept in the signature: the caller pairs the candidate
/// list with the schemas for the downstream classify step. The walk is deterministic:
/// the `BTreeSet` makes the output order independent of `read_dir`'s natural
/// (filesystem) order (the hardening-#7 property: same repo in, byte-identical list
/// out).
pub fn discover_candidates(repo_root: &Path, _schemas: &[Schema]) -> Vec<String> {
    // A `BTreeSet` collects the candidates, so the output is sorted + deduped
    // regardless of the per-dir `read_dir` order.
    let mut candidates: BTreeSet<String> = BTreeSet::new();
    walk(repo_root, "", &mut candidates);
    candidates.into_iter().collect()
}

/// Recursively collect repo-relative `*.md` paths under `dir` (whose repo-relative
/// prefix is `prefix`, `""` at the root) into `candidates`, pruning the internals
/// dirs by name. A missing/unreadable dir contributes nothing.
fn walk(dir: &Path, prefix: &str, candidates: &mut BTreeSet<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return; // dir absent/unreadable: contributes nothing.
    };
    for entry in entries.flatten() {
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };
        let path = entry.path();
        if path.is_dir() {
            // Prune the internals dirs — never descended into.
            if INTERNALS_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(&path, &rel, candidates);
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            candidates.insert(rel);
        }
    }
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

/// **Adopt** an [`Verdict::Adoptable`] candidate — the net-new, schema-gated register
/// action (`project-setup.md` → Flow 2, bullet 3). Distinct from reconciliation's
/// `UNKNOWN → baseline-adopt` ([`crate::file_state::file_state`]), which records a hash
/// with **no schema check and no index population** — exactly the safety hole this
/// closes: ingestion registers only what re-conforms, and indexes its edges.
///
/// `rel_path` is the candidate's forward-slash repo-relative path (its `file-state`
/// record key); `bytes` are its raw on-disk bytes; `schema` is the matched doc-type the
/// classifier verdict named. Adopt **re-gates** rather than trusting the verdict:
///
/// 1. **parse → conformance** against `schema` (the same binary gate [`classify`] runs);
///    a non-conformant doc returns `Err(findings)` — **refused, never adopted** (the hole
///    closed).
/// 2. **`EdgeIndex::absorb_doc`** for the doc's identity `<type>:<slug>` (the `slug` is
///    `rel_path`'s filename stem), so an adopted `adr`'s `supersedes` edge enters the
///    forward index (unlike `baseline-adopt`, which indexes nothing).
/// 3. **`FileStateRecord::record(rel_path, hash_bytes(bytes))`** — the file-state
///    baseline for the now-managed doc.
///
/// **Register-only — it never moves or rewrites the file.** No I/O of its own: the caller
/// supplies the bytes already read and persists the advanced `record` + `index`. A
/// misplaced-but-conformant doc is the classifier's `needs-reconcile`, never adopted
/// here, so adopt never needs to relocate.
pub fn adopt(
    record: &mut FileStateRecord,
    index: &mut EdgeIndex,
    schema: &Schema,
    rel_path: &str,
    bytes: &[u8],
) -> Result<(), Vec<Finding>> {
    let mut source = String::from_utf8_lossy(bytes).into_owned();
    crate::parse::strip_leading_bom(&mut source);

    // Re-gate: parse → conformance. A non-conformant doc is refused (the hole closed).
    let doc = parse_sections(schema, &source)?;
    let conformance = schema_conformance(schema, &source, &doc);
    if !conformance.is_empty() {
        return Err(conformance);
    }

    // The `<type>:<slug>` identity — slug is the candidate's filename stem.
    let slug = rel_path.rsplit('/').next().unwrap_or(rel_path);
    let slug = slug.strip_suffix(".md").unwrap_or(slug);
    let from = format!("{}:{slug}", schema.ty);

    // Index the doc's forward edges, then baseline the file-state hash (register-only).
    index.absorb_doc(schema, &from, &doc);
    record.record(rel_path, hash_bytes(bytes));
    Ok(())
}

/// **Un-manage** a managed doc — the inverse of [`adopt`]'s register-only mutation
/// (M21 Increment 4; `project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5),
/// un-manage a doc). Drops the doc from jigc's index/state, **leaving the file bytes
/// on disk** (this function has no file-write path of its own):
///
/// 1. **`EdgeIndex::drop_doc(from)`** — remove every forward edge originating from the
///    doc's `<type>:<slug>` identity (the inverse of `adopt`'s `absorb_doc`).
/// 2. **`FileStateRecord::forget(rel_path)`** — drop the file-state baseline hash keyed
///    by the doc's `rel_path` (the inverse of `adopt`'s `record`).
///
/// `from` is the doc's `<type>:<slug>` identity (re-derived by the caller via the
/// classify location-match); `rel_path` is the file-state record key (symmetric with
/// `adopt`'s addressing). Returns `true` iff **either** surface changed — so a re-run on
/// an already-unmanaged doc returns `false` (a clean no-op, not an error), and the caller
/// can skip the persist. A `rel_path` under no schema location carries no edges, so only
/// the file-state entry drops; the index is left byte-identical (zero-edge case).
///
/// Register-only by construction (no I/O): the caller persists the mutated `record` +
/// `index` iff this returns `true`.
pub fn unmanage(
    record: &mut FileStateRecord,
    index: &mut EdgeIndex,
    from: &str,
    rel_path: &str,
) -> bool {
    // Order is irrelevant (the two surfaces are independent); `|` evaluates both so
    // neither short-circuits — a doc with edges but no baseline (or vice-versa) still
    // fully un-manages.
    let dropped_edges = index.drop_doc(from);
    let forgot = record.forget(rel_path);
    dropped_edges | forgot
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
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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

## Options
Alternatives were weighed and rejected.

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

    use crate::file_state::{FileStateRecord, hash_bytes};
    use crate::finding::Severity;
    use crate::index::{Edge, EdgeIndex};

    /// A conformant ADR carrying `supersedes: adr:single-node-cache` — the adopt
    /// fixture whose forward edge must enter the index on adoption.
    const CONFORMANT_ADR_SUPERSEDES: &str = "\
---
status: accepted
date: 2026-05-30
supersedes: adr:single-node-cache
---

# Distributed session cache

## Context
A single node is a single point of failure.

## Options
Alternatives were weighed and rejected.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";

    /// Adopt re-gates a conformant adr (carrying `supersedes: adr:single-node-cache`)
    /// and (a) populates the `(adr:<slug>, supersedes, adr:single-node-cache)` forward
    /// edge in the index + (b) records the file-state baseline hash for the rel-path.
    /// The slug is the candidate's filename stem.
    #[test]
    fn adopt_indexes_the_supersedes_edge_and_records_the_baseline() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let mut index = EdgeIndex::default();

        let rel_path = "decisions/distributed-cache.md";
        let bytes = CONFORMANT_ADR_SUPERSEDES.as_bytes();

        adopt(&mut record, &mut index, &schema, rel_path, bytes).expect("conformant adr adopts");

        // (a) The forward supersedes edge is indexed under the slug-derived identity.
        assert_eq!(
            index.edges,
            vec![Edge {
                from: "adr:distributed-cache".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }],
            "adopt populates the forward supersedes edge"
        );

        // (b) The file-state baseline hash for the rel-path is the raw-byte hash.
        assert_eq!(
            record.get(rel_path),
            Some(hash_bytes(bytes).as_str()),
            "adopt records the raw-byte file-state baseline keyed by rel-path"
        );
    }

    /// Register-only: adopt reads the file's bytes but **never moves or rewrites** it —
    /// the on-disk bytes are byte-identical before and after, and no new file appears.
    #[test]
    fn adopt_leaves_the_file_bytes_unchanged_on_disk() {
        let root = TempRoot::new("register-only");
        let rel_path = "decisions/distributed-cache.md";
        write(root.path(), rel_path, CONFORMANT_ADR_SUPERSEDES);

        let on_disk = root.path().join(rel_path);
        let before = std::fs::read(&on_disk).expect("read before");

        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let mut index = EdgeIndex::default();
        adopt(&mut record, &mut index, &schema, rel_path, &before).expect("adopts");

        // The file's bytes on disk are unchanged (register-only — no move, no rewrite).
        let after = std::fs::read(&on_disk).expect("read after");
        assert_eq!(before, after, "adopt never rewrites the adopted file");

        // No relocated/duplicated file: the decisions/ dir holds exactly the one file.
        let entries: Vec<_> = std::fs::read_dir(root.path().join("decisions"))
            .expect("read decisions/")
            .flatten()
            .map(|e| e.file_name())
            .collect();
        assert_eq!(entries.len(), 1, "adopt moves/creates no file: {entries:?}");
    }

    /// An ADR that parses but **fails conformance** (a malformed `date` value) — the
    /// fixture proving the conformance re-gate (not just the parse gate) refuses.
    const ADR_BAD_DATE: &str = "\
---
status: accepted
date: 2026/13/01
---

# Distributed session cache

## Context
A single node is a single point of failure.

## Options
Alternatives were weighed and rejected.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";

    /// The hole closed: a non-conformant doc dropped under a `location:` dir is
    /// **never adopted** — adopt re-gates and refuses, returning blocking findings, and
    /// touches neither the index nor the file-state record (no silent baseline-adopt).
    /// Both refusal paths are covered: a parse failure (a freeform near-miss) and a
    /// schema-conformance failure (a parseable doc with a malformed value).
    #[test]
    fn adopt_refuses_a_non_conformant_doc() {
        let schema = adr_schema();

        for (rel_path, source) in [
            ("decisions/notes.md", NON_CONFORMANT_NEAR_MISS), // fails parse
            ("decisions/bad-date.md", ADR_BAD_DATE),          // parses, fails conformance
        ] {
            let mut record = FileStateRecord::new();
            let mut index = EdgeIndex::default();
            let bytes = source.as_bytes();

            let err = adopt(&mut record, &mut index, &schema, rel_path, bytes)
                .expect_err("a non-conformant doc is refused, never adopted");
            assert!(!err.is_empty(), "refusal carries the conformance findings");
            assert!(
                err.iter().any(|f| f.severity == Severity::Blocking),
                "the refusal is blocking for {rel_path}: {err:?}"
            );

            // The hole closed: neither the index nor the file-state record was touched.
            assert!(
                index.edges.is_empty(),
                "a refused doc populates no edges for {rel_path}: {:?}",
                index.edges
            );
            assert_eq!(
                record.get(rel_path),
                None,
                "a refused doc records no file-state baseline for {rel_path} (no silent adopt)"
            );
        }
    }

    /// The inverse of adopt (M21 Increment 4 / T1): after adopt-ing a conformant adr
    /// carrying a `supersedes` edge, [`unmanage`] drops **both** the file-state hash
    /// (`record.get(rel_path) == None`) **and** the `from == adr:<slug>` edges; it
    /// returns `true` on the first run, and a **second** `unmanage` returns `false` with
    /// the record + index left **byte-identical** (idempotent no-op).
    #[test]
    fn unmanage_drops_baseline_and_edges_and_is_idempotent() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let mut index = EdgeIndex::default();

        let rel_path = "decisions/distributed-cache.md";
        let from = "adr:distributed-cache";
        let bytes = CONFORMANT_ADR_SUPERSEDES.as_bytes();

        // Adopt: the doc is now managed (indexed + baselined).
        adopt(&mut record, &mut index, &schema, rel_path, bytes).expect("conformant adr adopts");
        assert!(record.get(rel_path).is_some(), "adopt baselined the doc");
        assert!(
            index.edges.iter().any(|e| e.from == from),
            "adopt indexed the supersedes edge"
        );

        // First un-manage: drops both surfaces, returns true.
        let dropped = unmanage(&mut record, &mut index, from, rel_path);
        assert!(dropped, "un-managing a managed doc reports a change");
        assert_eq!(
            record.get(rel_path),
            None,
            "un-manage drops the file-state hash"
        );
        assert!(
            !index.edges.iter().any(|e| e.from == from),
            "un-manage drops the doc's forward edges: {:?}",
            index.edges
        );

        // Snapshot the post-unmanage bytes; a second run must leave them byte-identical.
        let record_bytes = record.to_bytes();
        let index_bytes = index.to_bytes();

        // Second un-manage: a clean no-op — returns false, both surfaces unchanged.
        let again = unmanage(&mut record, &mut index, from, rel_path);
        assert!(
            !again,
            "re-running on an already-unmanaged doc is a no-op (false)"
        );
        assert_eq!(
            record.to_bytes(),
            record_bytes,
            "a no-op un-manage leaves the record byte-identical"
        );
        assert_eq!(
            index.to_bytes(),
            index_bytes,
            "a no-op un-manage leaves the index byte-identical"
        );
    }

    /// A `rel_path` under **no schema location** (the type re-derivation yields a `from`
    /// matching no edge) drops **only** the file-state entry — zero edges, the index
    /// left untouched — and is still a clean no-op on re-run.
    #[test]
    fn unmanage_with_no_indexed_edges_drops_only_the_baseline() {
        let mut record = FileStateRecord::new();
        let mut index = EdgeIndex::default();

        // A baselined path that contributed no edges (e.g. a doc carrying no `ref`).
        let rel_path = "decisions/edgeless.md";
        let from = "adr:edgeless";
        record.record(rel_path, hash_bytes(b"some bytes\n"));
        let index_before = index.to_bytes();

        let dropped = unmanage(&mut record, &mut index, from, rel_path);
        assert!(
            dropped,
            "dropping a baselined-but-edgeless doc reports a change"
        );
        assert_eq!(record.get(rel_path), None, "the baseline is dropped");
        assert_eq!(
            index.to_bytes(),
            index_before,
            "no edges existed → the index is byte-identical"
        );

        assert!(
            !unmanage(&mut record, &mut index, from, rel_path),
            "a second run is a clean no-op"
        );
    }

    /// Recursive reach (G3): a fixture with docs in nested subdirectories — `wiki/`,
    /// `docs/sub/`, `rfcs/`, plus a root `README.md` and a `decisions/` doc — must
    /// surface **all five** repo-relative paths, sorted byte-identically regardless of
    /// `read_dir` order, while **excluding** both internals dirs (`.jigc/`, `.git/`).
    #[test]
    fn discovers_nested_docs_excluding_both_internals_dirs() {
        let root = TempRoot::new("recursive");
        // Five real candidates spread across nested dirs (deliberately unsorted writes).
        write(root.path(), "wiki/page.md", "wiki page");
        write(root.path(), "docs/sub/deep.md", "deeply nested note");
        write(root.path(), "rfcs/0001.md", "an rfc");
        write(root.path(), "README.md", "root readme");
        write(root.path(), "decisions/x.md", "a decision");
        // Internals that must NEVER be discovered, even though they are `.md`.
        write(root.path(), ".jigc/internal.md", "adapter internal");
        write(root.path(), ".git/hooks/x.md", "git internal");

        let got = discover_candidates(root.path(), &[adr_schema()]);

        assert_eq!(
            got,
            vec![
                "README.md".to_string(),
                "decisions/x.md".to_string(),
                "docs/sub/deep.md".to_string(),
                "rfcs/0001.md".to_string(),
                "wiki/page.md".to_string(),
            ],
        );
        // Both internals dirs are excluded by name (not by `.md` extension).
        assert!(
            !got.iter().any(|p| p.starts_with(".jigc/")),
            "the .jigc/ internals dir is excluded: {got:?}"
        );
        assert!(
            !got.iter().any(|p| p.starts_with(".git/")),
            "the .git/ internals dir is excluded: {got:?}"
        );
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
