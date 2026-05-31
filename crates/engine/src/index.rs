//! The edge index — a rebuildable map of cross-reference edges (forward, with
//! inverses derived) for cheap forward-ref integrity checks.
//!
//! See `design/storage.md` (edge-index lifecycle) and
//! `design/document-type-schema.md` (bidirectional, inverse derived not stored).
//!
//! ## What this module is (committed rebuild, site 1)
//!
//! This is the **committed-rebuild** site of the edge-index lifecycle
//! ([storage.md](../../../design/storage.md) → Edge index lifecycle, site 1): walk
//! the committed managed docs, parse each against its schema, and emit one forward
//! `(from, relation, to)` edge for every present **schema `ref` field**. The result
//! is stamped with the HEAD it was built against and persisted to
//! `.jigc/index/edges.json`; a [`load_committed`] whose stamp ≠ the current HEAD
//! rebuilds, a matching stamp loads as-is ([storage.md](../../../design/storage.md)
//! → Derived caches: each cache is stamped with the HEAD it was built against).
//!
//! ## Forward-only; the inverse is derived, never stored
//!
//! Only the **forward** edge is emitted and stored — the `supersedes: adr:a`
//! authored on doc B becomes `(adr:b, supersedes, adr:a)`. The back-edge
//! (`superseded-by`) is computed by *walking* this index at read-time, never
//! written ([document-type-schema.md](../../../design/document-type-schema.md) →
//! Bidirectional, but the inverse is derived). So this module stores a single
//! source of truth and the graph stays whole by construction.
//!
//! ## On-disk format + stamp (the pinned form)
//!
//! `{ "stamp": "<HEAD-sha>", "edges": [ { "from", "relation", "to" }, … ] }` —
//! pretty-JSON + one trailing newline, edges sorted by `(from, relation, to)` so the
//! bytes are deterministic. No `schema_version`: the index is a *rebuildable cache*,
//! re-derivable from the committed `.md`s at any time (the same stance as
//! [`crate::file_state::FileStateRecord`]). The HEAD stamp is **opaque text** the
//! caller supplies (the CLI reads it via `git`, keeping the engine shell-free —
//! `DECISIONS.md` 2026-05-31, inc-4 I/O split: CLI invokes git, engine resolves).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::field_block::Value;
use crate::schema::{FieldType, Schema, SectionBody};

/// The `edges.json` filename inside `<jigc_root>/index/`.
const EDGES_FILE: &str = "edges.json";

/// One forward cross-reference edge: doc `from` carries a `ref` field `relation`
/// pointing at `to`.
///
/// `from` is the source doc's identity (`<type>:<slug>`, identity-is-the-path);
/// `relation` is the schema `ref` field's id; `to` is the raw ref value (an address
/// like `adr:single-node-cache`). Forward-only — the inverse is derived by walking,
/// never stored.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Edge {
    /// The source doc's identity (`<type>:<slug>`).
    pub from: String,
    /// The `ref` field's id (the relation name).
    pub relation: String,
    /// The ref's raw target value (an address).
    pub to: String,
}

/// The persisted edge index: the HEAD `stamp` it was built against and the sorted
/// forward `edges`.
///
/// A rebuildable cache — it carries no `schema_version` of its own (re-derivable from
/// the committed docs at any time). `edges` are sorted by `(from, relation, to)` so
/// the on-disk bytes are deterministic.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeIndex {
    /// The HEAD (or doc-set fingerprint) this index was built against.
    pub stamp: String,
    /// The forward edges, sorted by `(from, relation, to)`.
    pub edges: Vec<Edge>,
}

impl EdgeIndex {
    /// The index's on-disk location under a `.jigc/` home.
    pub fn path_in(jigc_root: &Path) -> PathBuf {
        jigc_root.join("index").join(EDGES_FILE)
    }

    /// Serialize to the frozen on-disk byte form: pretty JSON + one trailing newline
    /// (the `file-state.json` / `base.json` convention). `edges` are already sorted.
    pub fn to_bytes(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("EdgeIndex serializes");
        s.push('\n');
        s
    }

    /// Save the index to `<jigc_root>/index/edges.json`, atomically (temp + rename),
    /// creating the `index/` dir on demand.
    pub fn save(&self, jigc_root: &Path) -> std::io::Result<()> {
        let path = Self::path_in(jigc_root);
        crate::state::persist(&path, self.to_bytes().as_bytes())
    }
}

/// Walk the committed managed docs and emit the sorted forward edge set, stamped
/// with `head` — the committed-rebuild (lifecycle site 1).
///
/// For every schema in `schemas` that declares a persisted `location:`, the
/// committed `<location>/<slug>.md` instances are parsed against the schema and each
/// present **`ref` field** emits a forward `(from = <type>:<slug>, relation =
/// <field-id>, to = <value>)` edge (a list-valued ref emits one edge per element).
/// Edges are sorted by `(from, relation, to)`; the inverse is never stored.
///
/// `head` is the opaque stamp the index is tagged with (the caller's HEAD sha). A
/// committed file that does not parse against its schema is **skipped** (its edges
/// are omitted) — the rebuild is best-effort over the committed store; per-doc
/// conformance is the parse-path / `finalize` gate's concern, not the index's.
pub fn rebuild_committed(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    head: &str,
) -> EdgeIndex {
    let mut edges = Vec::new();

    for (ty, schema) in schemas {
        let Some(location) = schema.location.as_deref() else {
            continue; // a transient (location-less) type has no committed docs.
        };
        let dir = repo_root.join(location);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue; // no committed docs of this type yet.
        };

        // The schema's `ref` field ids, by the section they live in.
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let Some(slug) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let Ok(source) = std::fs::read_to_string(&path) else {
                continue;
            };
            let Ok(doc) = crate::parse::parse_sections(schema, &source) else {
                continue; // unparseable committed file: skip; not the index's gate.
            };

            let from = format!("{ty}:{slug}");
            edges.extend(doc_edges(schema, &doc, &from));
        }
    }

    edges.sort();
    edges.dedup();
    EdgeIndex {
        stamp: head.to_string(),
        edges,
    }
}

/// Load the committed edge index at `<jigc_root>/index/edges.json`, rebuilding when
/// its stamp ≠ `head` (a branch switch, pull, or rebase moved the docs underneath
/// it) — the committed-rebuild's read entrypoint.
///
/// A present file whose `stamp == head` loads and returns as-is. A missing file, an
/// unreadable/corrupt file, or a stamp mismatch triggers a [`rebuild_committed`]
/// against the current store, which is then persisted with the new stamp. (Persist
/// failure is best-effort: the freshly rebuilt index is returned regardless, the
/// cache-stamp self-heals on the next read — `storage.md` → Derived caches.)
pub fn load_committed(
    repo_root: &Path,
    jigc_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    head: &str,
) -> EdgeIndex {
    let path = EdgeIndex::path_in(jigc_root);
    if let Ok(bytes) = std::fs::read(&path)
        && let Ok(index) = serde_json::from_slice::<EdgeIndex>(&bytes)
        && index.stamp == head
    {
        return index;
    }
    let rebuilt = rebuild_committed(repo_root, schemas, head);
    let _ = rebuilt.save(jigc_root);
    rebuilt
}

/// Emit the forward edges a single parsed doc contributes: one per present schema
/// `ref` field (a list-valued ref → one edge per element).
fn doc_edges(schema: &Schema, doc: &crate::parse::Document, from: &str) -> Vec<Edge> {
    let mut edges = Vec::new();

    for section in &schema.sections {
        let SectionBody::Simple { fields, .. } = &section.body else {
            continue; // repeatable-section refs are not an MVP target.
        };
        let ref_ids: Vec<&str> = fields
            .iter()
            .filter(|f| f.ty == FieldType::Ref)
            .map(|f| f.id.as_str())
            .collect();
        if ref_ids.is_empty() {
            continue;
        }

        let Some(parsed) = doc.sections.iter().find(|s| s.id == section.id) else {
            continue;
        };
        for field in &parsed.fields {
            if !ref_ids.contains(&field.key.as_str()) {
                continue;
            }
            match &field.value {
                Value::Scalar(v) => edges.push(Edge {
                    from: from.to_string(),
                    relation: field.key.clone(),
                    to: v.clone(),
                }),
                Value::List(vs) => {
                    for v in vs {
                        edges.push(Edge {
                            from: from.to_string(),
                            relation: field.key.clone(),
                            to: v.clone(),
                        });
                    }
                }
            }
        }
    }

    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    /// A throwaway directory that removes itself on drop — keeps index tests off any
    /// real repo tree.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-index-{tag}-{}-{:?}",
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

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "adr".to_string(),
            crate::schema::load_schema(ADR_YAML).expect("adr.yaml loads"),
        );
        m
    }

    /// A committed ADR `A` with no outgoing ref.
    const ADR_A: &str = "\
---
status: accepted
date: 2026-05-23
---

# Single-node session cache

## Context
Session lookups must stay sub-millisecond.

## Decision
A single in-memory node keeps session lookups sub-millisecond.

## Consequences
A cold node loses its sessions; clients re-authenticate.
";

    /// A committed ADR `B` that supersedes `A`.
    const ADR_B: &str = "\
---
status: accepted
date: 2026-05-30
supersedes: adr:single-node-cache
---

# Distributed session cache

## Context
A single node is a single point of failure.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";

    /// Commit ADRs A and B to `decisions/` under `repo_root`.
    fn write_committed_adrs(repo_root: &Path) {
        let dir = repo_root.join("decisions");
        std::fs::create_dir_all(&dir).expect("mk decisions/");
        std::fs::write(dir.join("single-node-cache.md"), ADR_A).expect("write A");
        std::fs::write(dir.join("distributed-cache.md"), ADR_B).expect("write B");
    }

    /// GOLDEN: committing two ADRs where B supersedes A and rebuilding the committed
    /// index produces exactly one forward edge `(adr:distributed-cache, supersedes,
    /// adr:single-node-cache)`, stamped with HEAD, serialized to the frozen
    /// `edges.json` bytes (sorted, stamp normalized).
    #[test]
    fn edge_index_rebuilds_from_committed_supersedes_edge() {
        let root = TempRoot::new("rebuild");
        write_committed_adrs(root.path());

        let index = rebuild_committed(root.path(), &schemas(), "HEADSHA");

        assert_eq!(
            index.edges,
            vec![Edge {
                from: "adr:distributed-cache".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }],
            "exactly the forward supersedes edge, no inverse",
        );

        // The serialized on-disk bytes, with the stamp normalized for the snapshot.
        let normalized = EdgeIndex {
            stamp: "<HEAD>".to_string(),
            ..index.clone()
        };
        insta::assert_snapshot!(normalized.to_bytes(), @r#"
        {
          "stamp": "<HEAD>",
          "edges": [
            {
              "from": "adr:distributed-cache",
              "relation": "supersedes",
              "to": "adr:single-node-cache"
            }
          ]
        }
        "#);
    }

    /// A stale stamp triggers a rebuild on load; a matching stamp loads the persisted
    /// bytes as-is.
    #[test]
    fn load_rebuilds_on_stale_stamp_and_loads_on_match() {
        let repo = TempRoot::new("load-repo");
        let jigc = TempRoot::new("load-jigc");
        write_committed_adrs(repo.path());
        let schemas = schemas();

        // First load: no cache present → rebuild against HEAD "v1", persisted.
        let first = load_committed(repo.path(), jigc.path(), &schemas, "v1");
        assert_eq!(first.stamp, "v1");
        assert_eq!(first.edges.len(), 1);
        assert!(
            EdgeIndex::path_in(jigc.path()).exists(),
            "the rebuilt index is persisted"
        );

        // A matching stamp loads the persisted index as-is (no rebuild needed).
        let same = load_committed(repo.path(), jigc.path(), &schemas, "v1");
        assert_eq!(
            same, first,
            "matching stamp loads the persisted bytes as-is"
        );

        // A new HEAD ("v2") is a stamp mismatch → rebuild and re-stamp.
        let rebuilt = load_committed(repo.path(), jigc.path(), &schemas, "v2");
        assert_eq!(rebuilt.stamp, "v2", "a stale stamp triggers a rebuild");
        assert_eq!(
            rebuilt.edges, first.edges,
            "same committed edges, new stamp"
        );

        // The persisted file now carries the new stamp.
        let on_disk: EdgeIndex = serde_json::from_slice(
            &std::fs::read(EdgeIndex::path_in(jigc.path())).expect("read persisted index"),
        )
        .expect("persisted index parses");
        assert_eq!(on_disk.stamp, "v2");
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use crate::write::{self, Instance, SectionContent};
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert("adr".to_string(), adr_schema());
        m
    }

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-index-prop-{}-{:?}",
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

    /// A slug fragment for a committed ADR filename.
    fn slug_strategy() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9-]{0,12}"
    }

    proptest! {
        /// EXTRACTION IS TOTAL + FORWARD-ONLY: for an arbitrary set of committed ADRs,
        /// each optionally carrying a `supersedes` ref, the rebuilt edge set equals
        /// **exactly** the present ref-field values across the doc set — no edge for an
        /// absent ref, no inverse edge, one edge per present value. And the index
        /// serialize→deserialize round-trips byte-for-byte.
        #[test]
        fn extraction_is_total_forward_only_and_round_trips(
            specs in prop::collection::vec(
                (slug_strategy(), proptest::option::of(slug_strategy())),
                1..6,
            ),
        ) {
            // Distinct slugs so filenames don't collide; build the committed store.
            let mut specs = specs;
            specs.sort();
            specs.dedup_by(|a, b| a.0 == b.0);

            let schema = adr_schema();
            let root = TempRoot::new();
            let dir = root.path().join("decisions");
            std::fs::create_dir_all(&dir).expect("mk decisions/");

            // The expected forward edge set, built independently of the walker.
            let mut expected: Vec<Edge> = Vec::new();
            for (slug, supersedes) in &specs {
                let mut sections = vec![
                    SectionContent { id: "status".to_string(), ..Default::default() },
                    SectionContent {
                        id: "context".to_string(),
                        slot: Some("forces".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "decision".to_string(),
                        slot: Some("the call".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "consequences".to_string(),
                        slot: Some("tradeoffs".to_string()),
                        ..Default::default()
                    },
                ];
                if let Some(target) = supersedes {
                    let to = format!("adr:{target}");
                    sections[0].fields = vec![crate::field_block::Field {
                        key: "supersedes".to_string(),
                        value: crate::field_block::Value::Scalar(to.clone()),
                    }];
                    expected.push(Edge {
                        from: format!("adr:{slug}"),
                        relation: "supersedes".to_string(),
                        to,
                    });
                }
                let instance = Instance {
                    title: format!("ADR {slug}"),
                    sections,
                };
                let bytes = write::render(&schema, &instance);
                std::fs::write(dir.join(format!("{slug}.md")), &bytes).expect("write adr");
            }
            expected.sort();

            let index = rebuild_committed(root.path(), &schemas(), "STAMP");
            prop_assert_eq!(&index.edges, &expected);

            // serialize → deserialize round-trips.
            let bytes = index.to_bytes();
            let back: EdgeIndex = serde_json::from_str(&bytes).expect("round-trips");
            prop_assert_eq!(back, index);
        }
    }
}
