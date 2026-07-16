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
use crate::finding::{Finding, Location, Severity};
use crate::schema::{Field, FieldType, Leaf, Schema, SectionBody};

/// The `edges.json` filename inside `<jigc_root>/index/`.
const EDGES_FILE: &str = "edges.json";

/// The working-area sub-directory holding the task's staged doc instances
/// (`<task_dir>/docs/<type>:<slug>.md` — the `:`-joined address slug, `DECISIONS.md`
/// 2026-05-31 → Task working-area on-disk layout).
const DOCS_DIR: &str = "docs";

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

    /// **Incrementally update** the committed index for one doc's edges — the
    /// OOB-absorb site (lifecycle site 3, `storage.md` → Edge index lifecycle: "the
    /// clean-absorb path incrementally updates the committed index for the absorbed
    /// doc's edges; atomic, in-place").
    ///
    /// Drops every edge whose `from == from` and re-inserts the freshly-parsed
    /// forward edges the absorbed `doc` now contributes (`schema`'s present `ref`
    /// fields), keeping the set sorted+deduped — so an edit that added, removed, or
    /// changed a cross-ref is reflected exactly, without a full store rebuild. The
    /// `stamp` is left untouched: absorb shifts one doc's edges in place, it does not
    /// re-baseline the whole index against a new HEAD (that is the finalize-commit
    /// site).
    pub fn absorb_doc(&mut self, schema: &Schema, from: &str, doc: &crate::parse::Document) {
        self.edges.retain(|e| e.from != from);
        self.edges.extend(doc_edges(schema, doc, from));
        self.edges.sort();
        self.edges.dedup();
    }

    /// **Drop every forward edge originating from `from`** — the inverse of
    /// [`absorb_doc`](Self::absorb_doc)'s register half (the `retain` with no re-add),
    /// the un-manage primitive (M21 Increment 4; `project-setup.md` → Flow 2 hardening
    /// → Teardown / cleanup (G5), un-manage a doc). Returns `true` iff at least one
    /// edge was removed, so a caller can report a no-op on an already-dropped doc
    /// (idempotency). The remaining edges stay sorted (a `retain` preserves order), so
    /// the on-disk bytes are unchanged for any doc that did not contribute an edge.
    pub fn drop_doc(&mut self, from: &str) -> bool {
        let before = self.edges.len();
        self.edges.retain(|e| e.from != from);
        self.edges.len() != before
    }
}

/// Enumerate `target`'s **inverse edges** — every forward edge in `index` whose
/// `to == target`, i.e. each persisted referrer that points at `target` across the
/// three persisted ref-fields (`adr.supersedes`, `arch-doc.cites`, `spec.derived-from`).
///
/// This is the **named reverse-walk** the `jigc rename` repoint step drives
/// ([write-commands.md](../../../design/write-commands.md) → `jigc rename`, step 3;
/// [storage.md](../../../design/storage.md) → Edge index lifecycle): the forward index
/// is the single source of truth, so the inverse is computed by *walking* it, never
/// stored (the inverse-is-derived rule). Returned in the index's `(from, relation, to)`
/// sort order (deterministic, since [`EdgeIndex::edges`] is kept sorted); a `target`
/// with no inbound edge yields an empty `Vec`. Pure over the in-memory edge set — no I/O.
pub fn referrers_of<'a>(index: &'a EdgeIndex, target: &str) -> Vec<&'a Edge> {
    index.edges.iter().filter(|e| e.to == target).collect()
}

/// Invalidate the persisted committed edge index — the `finalize` post-commit step
/// (lifecycle site 5, `storage.md` → Edge index lifecycle: "phase 7 invalidates the
/// stamp; next read rebuilds against the new HEAD").
///
/// Removes `<jigc_root>/index/edges.json` outright: a missing file is the strongest
/// invalidation — the next [`load_committed`] rebuilds against the current HEAD and
/// re-stamps (an absent file can never match a stamp). Best-effort by contract: a
/// failure self-heals (the stale on-disk stamp ≠ the new HEAD, so the next read rebuilds
/// regardless), so a not-found is success and any other error is returned for the caller
/// to log, never raise (`finalize.md` → 7. Post-commit: logged, not raised).
pub fn invalidate(jigc_root: &Path) -> std::io::Result<()> {
    let path = EdgeIndex::path_in(jigc_root);
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

/// Walk the committed managed docs and emit the sorted forward edge set, stamped
/// with `head` — the committed-rebuild (lifecycle site 1).
///
/// For every schema with committed instances — a located type's `<location>/<slug>.md`
/// files, or a **placement** type's single literal `placement.file` (addressed by the
/// fixed `<type>:<type>` singleton slug, [`committed_instances`]) — each instance is
/// parsed against the schema and each present **`ref` field** emits a forward `(from =
/// <type>:<slug>, relation = <field-id>, to = <value>)` edge (a list-valued ref emits one
/// edge per element). Edges are sorted by `(from, relation, to)`; the inverse is never stored.
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
        for (from, path) in committed_instances(repo_root, ty, schema) {
            let Ok(mut source) = std::fs::read_to_string(&path) else {
                continue; // no committed doc at this path yet.
            };
            crate::parse::strip_leading_bom(&mut source);
            let Ok(doc) = crate::parse::parse_sections(schema, &source) else {
                continue; // unparseable committed file: skip; not the index's gate.
            };
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

/// The working overlay (lifecycle site 2, `storage.md` → Edge index lifecycle): the
/// active task's working-area edges layered over the committed index, **in-memory,
/// never persisted** ("the index is never persisted with task deltas in it").
///
/// `committed` is the [`EdgeIndex`]'s edges (surface a's edge map); `task_edges` are
/// the forward edges the task's `docs/*.md` contribute (surface b — this task's
/// pending writes). `task_froms` records each `from` identity the task touched (the
/// `<type>:<slug>` of each staged instance), so [`ref_resolves`] walks only
/// **task-touched** edges — a committed-only edge is not this task's to fix.
///
/// Forward-ref integrity walks the *overlaid* graph (committed ∪ task); the overlay
/// is read-side only, derived per call and discarded — only `finalize` writes through
/// (indirectly, via stamp invalidation → next read rebuilds against the new HEAD).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkingOverlay {
    /// The committed forward edges (surface a).
    pub committed: Vec<Edge>,
    /// The task working-area forward edges (surface b — this task's pending writes).
    pub task_edges: Vec<Edge>,
    /// The `from` identities the task touched (`<type>:<slug>` per staged instance).
    pub task_froms: Vec<String>,
}

/// Derive the working overlay: parse the active task's `docs/*.md`, emit the same
/// `(from, relation, to)` forward edges the committed-rebuild path emits, and layer
/// them over `committed` — in-memory, never persisted (lifecycle site 2).
///
/// A staged instance lives at `<task_dir>/docs/<type>:<slug>.md`; its `from` identity
/// is the filename stem (already `<type>:<slug>`, the `:`-joined address slug). Its
/// type prefix (before the first `:`) resolves to a [`Schema`] in `schemas`; an
/// unparseable or unknown-type instance contributes no edges (best-effort, mirroring
/// the committed rebuild — per-doc conformance is the `schema-conformance` gate's
/// concern, not the overlay's). The task's `from` identities are recorded so
/// [`ref_resolves`] walks only task-touched edges.
///
/// A working area with no `docs/` dir (nothing staged yet) yields an empty overlay
/// over the committed edges.
pub fn overlay_working(
    committed: &EdgeIndex,
    task_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> WorkingOverlay {
    let mut task_edges = Vec::new();
    let mut task_froms = Vec::new();

    let docs = task_dir.join(DOCS_DIR);
    if let Ok(entries) = std::fs::read_dir(&docs) {
        let mut staged: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
            .collect();
        staged.sort();

        for path in staged {
            let Some(from) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            // The type prefix before the first `:` resolves the schema.
            let ty = from.split(':').next().unwrap_or(from);
            let Some(schema) = schemas.get(ty) else {
                continue; // unknown type: no edges (best-effort, mirrors rebuild).
            };
            task_froms.push(from.to_string());

            let Ok(mut source) = std::fs::read_to_string(&path) else {
                continue;
            };
            crate::parse::strip_leading_bom(&mut source);
            let Ok(doc) = crate::parse::parse_sections(schema, &source) else {
                continue; // unparseable staged file: skip; not the overlay's gate.
            };
            task_edges.extend(doc_edges(schema, &doc, from));
        }
    }

    task_edges.sort();
    task_edges.dedup();
    task_froms.sort();
    task_froms.dedup();

    WorkingOverlay {
        committed: committed.edges.clone(),
        task_edges,
        task_froms,
    }
}

impl WorkingOverlay {
    /// Walk one `.relation` edge from `from` (a `<type>:<slug>` identity) over the
    /// **overlaid** graph (this task's working edges layered over the committed
    /// edges), returning **every** target identity `to` — a `0..*` relation fans out
    /// to all its bound targets; an empty `Vec` means the relation is **unset** on
    /// `from` (the context-slice reads that as an absent value → empty text, not an
    /// error — `workflow-dialect.md` → Empty vs unresolvable).
    ///
    /// The task's working edges are listed first (an `adr:b` staged in this task with
    /// a `supersedes` edge surfaces its staged target ahead of any committed one).
    /// Within each surface the edges are already `(from, relation, to)`-sorted, so the
    /// returned order is deterministic.
    ///
    /// Pure over the overlay's in-memory edge sets — no I/O.
    pub fn walk_edge(&self, from: &str, relation: &str) -> Vec<String> {
        self.task_edges
            .iter()
            .chain(self.committed.iter())
            .filter(|e| e.from == from && e.relation == relation)
            .map(|e| e.to.clone())
            .collect()
    }
}

/// The synthetic `schema-conformance.ref-resolves` check (forward-ref integrity, the
/// inc-5 check deferred from inc-4) over the working `overlay` — walk every
/// **task-touched** forward edge and require its `to` target resolve in one of the
/// **two reachable surfaces** (`validation.md` → Forward-ref resolution):
///
/// - **(a) the committed store** — `<repo_root>/<location>/<slug>.md` exists; or
/// - **(b) this task's working area** — `<task_dir>/docs/<type>:<slug>.md` exists
///   (created by deltas in the *same* task).
///
/// A target in *neither* is a **blocking** `schema-conformance.ref-resolves`
/// [`Finding`] whose message names the three routing options (fix the ref / create
/// the target in this task / drop the field — `worked-examples.md` → Superseding
/// decision, the dangling variant). Cross-task forward-refs are unsupported (same
/// finding). A resolvable edge yields nothing; a fully-resolvable task yields an empty
/// `Vec`. Intrinsic blocking (severity inventory); no cascade tuning wired yet
/// (parallels the other intrinsic checks).
pub fn ref_resolves(
    overlay: &WorkingOverlay,
    repo_root: &Path,
    task_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for edge in &overlay.task_edges {
        // Only edges originating from a task-touched doc are this task's to resolve.
        if !overlay.task_froms.contains(&edge.from) {
            continue;
        }
        if !target_reachable(&edge.to, repo_root, task_dir, schemas) {
            findings.push(dangling(edge));
        }
    }
    findings
}

/// The **store-wide** analog of [`ref_resolves`]: every committed forward edge's target
/// must resolve in the committed store. For each `edge` in `committed.edges`, a `to` that
/// is not [`committed_reachable`] emits the same blocking `schema-conformance.ref-resolves`
/// [`dangling`] [`Finding`] the task-scope gate raises. Unlike [`ref_resolves`] there is no
/// task working area (no surface b) and no `task_froms` filter — the committed store is
/// closed, so *every* committed edge is in scope. Drive it over a committed [`EdgeIndex`]
/// (e.g. [`rebuild_committed`]); a fully-resolvable store yields an empty `Vec`. This closes
/// the store-sweep asymmetry (doc↔code swept store-wide since M18, `ref-resolves` only ever
/// task-scoped) so the salience-independent pre-commit backstop reaches a dangling cross-doc
/// ref in the committed store.
///
/// `exclude_targets` is the **scope-subtraction** set the store sweep threads in (M35,
/// Component A; `validation.md` → file↔CLI-state: the scope-subtraction dedup): every
/// `<type>:<slug>` identity the read-only rename detector
/// ([`crate::file_state::detect_committed_store_renames`]) flagged as renamed/missing. An
/// edge whose `to` is in this set is **skipped** — its dangle is the renamed slug's inbound
/// edge, already diagnosed as one `reconciliation.rename` finding, so re-reporting it as N
/// independent dangling refs would emit a competing diagnosis for one OOB event. Every
/// *other* dangling edge is still reported (no short-circuit — `jigc validate` stays a
/// report-all read command). An **empty** set leaves every edge in scope — byte-identical to
/// the pre-M35 walk (the `jigc rename` verb's own pre-commit assertion passes empty).
pub fn ref_resolves_store(
    committed: &EdgeIndex,
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    exclude_targets: &std::collections::BTreeSet<String>,
) -> Vec<Finding> {
    dangling_edges(committed, repo_root, schemas)
        .into_iter()
        .filter(|edge| !exclude_targets.contains(&edge.to)) // subtracted: diagnosed as a rename.
        .map(dangling)
        .collect()
}

/// The dangling forward edges of a committed [`EdgeIndex`]: every edge whose `to` is not
/// [`committed_reachable`] in `repo_root`. The edge-level primitive [`ref_resolves_store`]
/// renders into `schema-conformance.ref-resolves` [`Finding`]s, exposed so a transaction
/// can **diff** its before/after dangle sets and refuse only on a *newly-introduced* dangle
/// (the `jigc rename` verb's pre-commit integrity gate — it must never block on pre-existing
/// store rot it did not cause, the M18/M19 masking-trap guard). Returns the dangling edges
/// in the index's `(from, relation, to)` order.
pub fn dangling_edges<'a>(
    committed: &'a EdgeIndex,
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> Vec<&'a Edge> {
    committed
        .edges
        .iter()
        .filter(|edge| !committed_reachable(&edge.to, repo_root, schemas))
        .collect()
}

/// The store-scope **`schema-completeness.inverse-cardinality`** check
/// (`design/validation.md` → Integrity vs completeness; `design/document-type-schema.md`
/// → Inverse-cardinality and orphan obligations). For every persisted `ref` field that
/// declares an `inverse-card` **minimum** ≥ 1 (e.g. `spec.derived-from → prd`, `inverse:
/// has-specs, inverse-card: "1..*"`), enumerate the committed docs of the **target** type
/// and count each one's inbound edges of that relation over the committed index; a target
/// below the declared minimum surfaces one **advisory** [`Finding`].
///
/// This is **completeness, not integrity** — a PRD's specs are *other tasks'* jobs, so the
/// check **depends on the whole store**, never just one task: it is **never** a per-task
/// `finalize` gate ([`ref_resolves`] is the integrity twin the task gate runs). Only
/// [`validate_store_families`](crate::validate::validate_store_families) calls it. Advisory
/// by default; the cascade may re-grade it (the severity post-pass at
/// [`ValidationReport::new`](crate::result::ValidationReport) keyed on
/// `validation.schema-completeness.inverse-cardinality.severity`).
///
/// Deterministic by construction: `schemas` is a [`BTreeMap`] (type-sorted), sections walk
/// in schema order, and the target docs are enumerated slug-sorted ([`committed_instances`]).
pub fn inverse_cardinality_store(
    committed: &EdgeIndex,
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for schema in schemas.values() {
        for section in &schema.sections {
            // A `ref` carrying an inverse-card obligation lives on a header / simple
            // section (the same surface [`doc_edges`] extracts forward edges from).
            let SectionBody::Simple { fields, .. } = &section.body else {
                continue;
            };
            for field in fields {
                if field.ty != FieldType::Ref {
                    continue;
                }
                let (Some(target_ty), Some(inverse_card)) =
                    (field.to.as_deref(), field.inverse_card.as_deref())
                else {
                    continue; // not a ref-with-inverse-card obligation.
                };
                let min = inverse_card_min(inverse_card);
                if min == 0 {
                    continue; // `0..*` / `0..1` impose no completeness floor.
                }
                let Some(target_schema) = schemas.get(target_ty) else {
                    continue; // unknown target type: no committed docs to enumerate.
                };
                // Enumerate the target type's committed instances — a located type's
                // `<location>/*.md` or a placement type's single literal file (a placement
                // doctype can be a ref target with an inverse-card obligation too).
                for (identity, _path) in committed_instances(repo_root, target_ty, target_schema) {
                    let count = committed
                        .edges
                        .iter()
                        .filter(|e| e.relation == field.id && e.to == identity)
                        .count();
                    if count < min {
                        findings.push(below_inverse_minimum(
                            &identity,
                            &field.id,
                            field.inverse.as_deref(),
                            count,
                            min,
                        ));
                    }
                }
            }
        }
    }
    findings
}

/// The store-scope **`schema-conformance.mention-resolves`** check (M33 Inc-3;
/// `design/document-type-schema.md` → In-prose mentions; `design/validation.md` → the
/// mention-resolves check). Scans every committed doc's **slot prose** for **managed
/// mentions** — a `#<type>:<slug>` token whose `<type>` is a key in `schemas` (a known
/// managed doctype) — and reports each whose `<type>:<slug>` names no committed doc, by
/// the same [`committed_reachable`] rule the `ref-resolves` families use. The
/// `:`-plus-known-doctype is the deterministic discriminator: a bare external `#issue-42`
/// (no `<type>:`) is **never** a managed mention and is never flagged, so external
/// issue/PR mentions in prose are not noise.
///
/// This is the **lighter, prose-embedded sibling of `ref-resolves`** — a mention dangles
/// when *another* doc is renamed/deleted, so store scope (the cross-doc backstop) is its
/// home: it is **never** wired into [`crate::validate::validate_task`] (the per-task
/// `finalize` gate), only this store sweep. Default [`Severity::Advisory`] (a cascade
/// knob, re-graded by the post-pass keyed on
/// `validation.schema-conformance.mention-resolves.severity`). Doc-level resolution in v1
/// (a trailing `#<section>` anchor is left unparsed — section-anchor mentions deferred).
/// Only [`mention_resolves_store`] callers — [`validate_store_families`](crate::validate::validate_store_families) —
/// run it. The CLI scans the prose but never authors it: the determinism boundary holds.
///
/// Deterministic by construction: `schemas` is a [`BTreeMap`] (type-sorted), the committed
/// docs enumerate slug-sorted, and slot spans walk in document order.
pub fn mention_resolves_store(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (ty, schema) in schemas {
        // Committed instances slug-sorted — a located type's `<location>/*.md` or a
        // placement type's single literal file (its prose slots carry managed mentions too).
        for (from, path) in committed_instances(repo_root, ty, schema) {
            let Ok(mut source) = std::fs::read_to_string(&path) else {
                continue; // read race: skip; the next sweep re-checks.
            };
            crate::parse::strip_leading_bom(&mut source);
            let Ok(doc) = crate::parse::parse_sections(schema, &source) else {
                continue; // unparseable committed file: skip — not this check's gate.
            };
            // One keyed finding per `(doc, token)`: a token repeated across slots (or within
            // one) collapses to a single finding — occurrence index is unstable under edits,
            // so it is not part of the stable key ([command-output-contract.md] → the stable
            // finding key). `seen` dedupes in first-encounter order (document order).
            let mut seen = std::collections::BTreeSet::new();
            for span in slot_spans(&doc) {
                for mention in scan_mentions(span.slice(&source), schemas) {
                    if !committed_reachable(&mention, repo_root, schemas)
                        && seen.insert(mention.clone())
                    {
                        findings.push(dangling_mention(&from, &mention));
                    }
                }
            }
        }
    }
    findings
}

/// Every slot-prose span a parsed doc carries, in document order: each simple section's
/// slot, plus each repeatable item's slot(s), recursing into nested items. Slot prose is
/// the only surface a managed mention can live on — fields are typed leaves and headings
/// are structure, so the scan reads exactly these opaque spans.
fn slot_spans(doc: &crate::parse::Document) -> Vec<&crate::parse::Span> {
    let mut spans = Vec::new();
    for section in &doc.sections {
        if let Some(span) = &section.slot {
            spans.push(span);
        }
        for item in &section.items {
            collect_item_slot_spans(item, &mut spans);
        }
    }
    spans
}

/// Gather one repeatable item's slot span(s) — the bare single-slot `slot` or the
/// multi-slot `slots` entries — recursing into its nested items (the M22 multi-level
/// shape). The recursive twin of [`slot_spans`]'s top-level walk.
fn collect_item_slot_spans<'a>(
    item: &'a crate::parse::ParsedItem,
    spans: &mut Vec<&'a crate::parse::Span>,
) {
    if let Some(span) = &item.slot {
        spans.push(span);
    }
    for (_leaf, span) in &item.slots {
        spans.push(span);
    }
    for child in &item.items {
        collect_item_slot_spans(child, spans);
    }
}

/// Scan opaque slot `prose` for **managed mentions** — `#<type>:<slug>` tokens whose
/// `<type>` is a key in `schemas` (a known managed doctype). The `:`-plus-known-doctype
/// is the deterministic discriminator: a bare `#issue-42` (no `<type>:`, an external
/// reference) is never a managed mention, so external issue/PR mentions are not flagged.
/// Returns each match's `<type>:<slug>` identity, in scan order; doc-level only (a
/// trailing `#<section>` anchor is left unparsed — section-anchor mentions are a v1
/// deferral).
fn scan_mentions(prose: &str, schemas: &BTreeMap<String, Schema>) -> Vec<String> {
    let bytes = prose.as_bytes();
    let mut mentions = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'#' {
            i += 1;
            continue;
        }
        // The `<type>` run: token bytes after `#`, up to a `:`.
        let type_start = i + 1;
        let mut j = type_start;
        while j < bytes.len() && is_mention_token_byte(bytes[j]) {
            j += 1;
        }
        // A managed mention needs a non-empty `<type>` immediately followed by `:`.
        if j == type_start || j >= bytes.len() || bytes[j] != b':' {
            i += 1;
            continue;
        }
        if !schemas.contains_key(&prose[type_start..j]) {
            i += 1; // `:` present but `<type>` is not a known doctype — not managed.
            continue;
        }
        // The `<slug>` run after the `:`.
        let slug_start = j + 1;
        let mut k = slug_start;
        while k < bytes.len() && is_mention_token_byte(bytes[k]) {
            k += 1;
        }
        if k == slug_start {
            i += 1; // `#<type>:` with no slug — not a mention.
            continue;
        }
        mentions.push(format!(
            "{}:{}",
            &prose[type_start..j],
            &prose[slug_start..k]
        ));
        i = k; // resume past the slug (a trailing `#<section>` anchor stays unparsed).
    }
    mentions
}

/// A managed-mention token byte: ASCII alphanumeric or `-` (the canonical slug +
/// doctype-name alphabet — `arch-doc`, `csrf-strict-origin`). A `:` separates `<type>`
/// from `<slug>` and is *not* a token byte; everything else ends a run.
fn is_mention_token_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-'
}

/// An **advisory** `schema-conformance.mention-resolves` [`Finding`] for an in-prose
/// managed mention that names no committed doc — located at the source doc's identity,
/// naming the dangling `<type>:<slug>` so the operator knows which mention to fix.
/// Carries a **repair route** (the human `route` kind — a real fix jigc cannot execute):
/// the prose author corrects or drops the mention. This is the advisory-route floor —
/// every finding routes, never `null`, and a dangling mention is a genuine repair, not the
/// informational "no action needed" ([command-output-contract.md] → §route, two kinds;
/// [validation.md] → the advisory-route floor; mirroring [`below_inverse_minimum`]'s
/// author-a-referrer repair route).
fn dangling_mention(from: &str, mention: &str) -> Finding {
    // `#<token>` — one keyed finding per `(doc, token)`; the caller collapses all
    // occurrences of a token in a doc to one (occurrence index is unstable under edits), so
    // the token fragment keeps distinct dangling mentions in one doc from colliding
    // ([command-output-contract.md](../../../design/command-output-contract.md) → the stable
    // finding key: mention → `#<token>`, one per `(doc, token)`).
    Finding::graded(
        Severity::Advisory,
        "schema-conformance.mention-resolves",
        format!(
            "schema-conformance — in-prose mention `#{mention}` in `{from}` resolves to no \
             committed doc (the renamed/deleted-doc case); correct or drop the mention",
        ),
        Some(Location::addressed(format!("{from}#{mention}"), 1, 1)),
        Some(format!("correct or drop the `#{mention}` mention in `{from}`").into()),
    )
}

/// The minimum of an `inverse-card` spelling — the integer before the `..` of a
/// `min..max` range (`"1..*"` → 1), or the whole value when there is no range
/// (`"1"` → 1). An unparseable minimum is treated as 0 (no completeness floor), so a
/// malformed declaration never fabricates a finding.
fn inverse_card_min(inverse_card: &str) -> usize {
    let head = inverse_card
        .split("..")
        .next()
        .unwrap_or(inverse_card)
        .trim();
    head.parse().unwrap_or(0)
}

/// The committed instances of a doctype as `(from_identity, path)` pairs, **slug-sorted**
/// (the deterministic enumeration order) — the index-side placement census enumerator, the
/// sibling of [`crate::store::canonical_path`] / [`crate::file_state`]'s placement arms
/// ([storage.md](../../../design/storage.md) → Placement census).
///
/// A **located** doctype yields one entry per `<repo_root>/<location>/<slug>.md`, identity
/// `<type>:<slug>`. A **placement** doctype ([storage.md](../../../design/storage.md) →
/// Placement) yields its single committed instance at the literal `placement.file`
/// (repo-root-relative), addressed by the fixed singleton slug (= type id), identity
/// `<type>:<type>` — but only when that file is present on disk (an absent placement
/// singleton has no committed doc, matching the located-dir glob which lists only existing
/// files). A transient (location-less, non-placement) type yields none.
///
/// `pub` (not `pub(crate)`) because the census reaches outside the engine: the CLI's
/// compose-time `store` data-value resolver (`cli::start::committed_store` — the census's
/// 14th site) enumerates through this same walk, so **one** home resolution serves every
/// committed-instance consumer in both crates.
pub fn committed_instances(repo_root: &Path, ty: &str, schema: &Schema) -> Vec<(String, PathBuf)> {
    // A placement doctype's one instance lives at its literal repo-root-relative file,
    // addressed by the fixed `<type>:<type>` singleton slug — never a `<location>/*.md` glob.
    if let Some(placement) = &schema.placement {
        let path = repo_root.join(&placement.file);
        return if path.exists() {
            vec![(format!("{ty}:{ty}"), path)]
        } else {
            Vec::new() // an absent placement singleton has no committed doc.
        };
    }
    let Some(location) = schema.location.as_deref() else {
        return Vec::new(); // a transient (location-less) type has no committed docs.
    };
    let dir = repo_root.join(location);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new(); // no committed docs of this type yet.
    };
    let mut out: Vec<(String, PathBuf)> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .filter_map(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .map(|slug| (format!("{ty}:{slug}"), p.clone()))
        })
        .collect();
    out.sort();
    out
}

/// An **advisory** `schema-completeness.inverse-cardinality` [`Finding`] for a target doc
/// below its inverse-card minimum — located at the deficient target's identity, naming the
/// inverse relation (when declared) so the operator knows which obligation is unmet. Carries
/// a **repair route** (the human `route` kind — a real fix jigc cannot execute): author a
/// `<relation>` referrer of the target. This is the advisory-route floor — every finding
/// routes, never `null`, and an unmet completeness obligation is a genuine (another-task)
/// repair, not the informational "no action needed" ([command-output-contract.md] → §route,
/// two kinds; [validation.md] → the advisory-route floor).
fn below_inverse_minimum(
    target: &str,
    relation: &str,
    inverse: Option<&str>,
    count: usize,
    min: usize,
) -> Finding {
    let inverse_phrase = inverse
        .map(|i| format!(" (its `{i}` inverse)"))
        .unwrap_or_default();
    // `#<relation>` — a doc can be below-min on several inverse relations (the caller loops
    // per ref field), so the relation fragment keeps each finding's `(code, target)` key
    // distinct rather than colliding on the bare target identity
    // ([command-output-contract.md](../../../design/command-output-contract.md) → the stable
    // finding key: inverse-cardinality is per-relation, not one-per-doc).
    Finding::graded(
        Severity::Advisory,
        "schema-completeness.inverse-cardinality",
        format!(
            "schema-completeness — `{target}` has {count} inbound `{relation}` edge(s){inverse_phrase}, \
             below the inverse-card minimum of {min}",
        ),
        Some(Location::addressed(format!("{target}#{relation}"), 1, 1)),
        Some(format!("author a `{relation}` referrer of `{target}`").into()),
    )
}

/// Is `to` (`<type>:<slug>`) reachable in either surface — the committed store
/// (`<location>/<slug>.md` exists) or this task's working area
/// (`docs/<type>:<slug>.md` exists)?
fn target_reachable(
    to: &str,
    repo_root: &Path,
    task_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> bool {
    // Surface b: this task's working area, keyed by the `:`-joined identity.
    if task_dir.join(DOCS_DIR).join(format!("{to}.md")).exists() {
        return true;
    }
    // Surface a: the committed store at the target type's canonical path.
    committed_reachable(to, repo_root, schemas)
}

/// Is `to` (`<type>:<slug>`) reachable in the **committed store** alone — its target
/// type's canonical `<location>/<slug>.md` exists? This is surface-a of
/// [`target_reachable`], extracted so the store-wide forward-ref sweep
/// ([`ref_resolves_store`]) — which holds no task working area — reuses the identical
/// committed-reachability rule. An unknown type or a non-`<type>:<slug>` identity is
/// unreachable.
fn committed_reachable(to: &str, repo_root: &Path, schemas: &BTreeMap<String, Schema>) -> bool {
    let (ty, slug) = match to.split_once(':') {
        Some(pair) => pair,
        None => return false, // not a `<type>:<slug>` identity: unreachable.
    };
    let Some(schema) = schemas.get(ty) else {
        return false; // unknown type: no committed location to reach.
    };
    crate::store::canonical_path(repo_root, schema, slug)
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// A blocking `schema-conformance.ref-resolves` [`Finding`] for a dangling forward
/// edge — located at the source doc's identity **plus a `#<relation>/<type>:<to-slug>` fragment**,
/// naming the three routing options. The fragment makes the finding's stable
/// `(code, target)` key collision-free: a `0..*` ref with several dangling targets fans one
/// keyed finding per dangling target ([command-output-contract.md](../../../design/command-output-contract.md)
/// → the stable finding key), rather than colliding on the bare source identity.
fn dangling(edge: &Edge) -> Finding {
    // `#<relation>/<type>:<to-slug>` — the target's **full identity** (`edge.to`), so the key
    // names its subject unambiguously and collision-freedom holds without assuming a relation's
    // targets never span two doctypes (a 1.0-pinned stable key must identify unambiguously).
    Finding::graded(
        Severity::Blocking,
        "schema-conformance.ref-resolves",
        format!(
            "forward-ref integrity — `{from}#{relation}` target `{to}` resolves in \
             neither the committed store nor this task's working area; resolution: \
             fix the reference to an existing target, create the target in this task, \
             or drop the `{relation}` field",
            from = edge.from,
            relation = edge.relation,
            to = edge.to,
        ),
        Some(Location::addressed(
            format!("{}#{}/{}", edge.from, edge.relation, edge.to),
            1,
            1,
        )),
        Some("fix the reference, create the target in this task, or drop the field".into()),
    )
}

/// Emit the sorted forward edges a doc `source` contributes under identity `from` —
/// the same `(from, relation, to)` extraction the committed-rebuild / working-overlay
/// paths perform, exposed for callers that hold a doc as bytes rather than a file (the
/// by-task-id join's self-ref rewrite re-derives a suffixed instance's edges from its
/// rewritten body, `storage.md` → The by-task-id join, step 4). An unparseable source
/// contributes no edges (best-effort, mirroring the rebuild). Edges are sorted by
/// `(from, relation, to)`.
pub fn edges_from_source(schema: &Schema, source: &str, from: &str) -> Vec<Edge> {
    let mut src = source.to_string();
    crate::parse::strip_leading_bom(&mut src);
    let Ok(doc) = crate::parse::parse_sections(schema, &src) else {
        return Vec::new();
    };
    let mut edges = doc_edges(schema, &doc, from);
    edges.sort();
    edges
}

/// Emit the forward edges a single parsed doc contributes: one per present schema
/// `ref` field, on **both** a simple section's field group and a **repeatable
/// section's item bodies** (a list-valued ref → one edge per element; a
/// repeatable-item ref → one edge per item that carries it). Non-ref leaves (slots,
/// `code-anchor` fields, nested repeatables) contribute nothing.
fn doc_edges(schema: &Schema, doc: &crate::parse::Document, from: &str) -> Vec<Edge> {
    let mut edges = Vec::new();

    for section in &schema.sections {
        let Some(parsed) = doc.sections.iter().find(|s| s.id == section.id) else {
            continue;
        };
        match &section.body {
            SectionBody::Simple { fields, .. } => {
                let ref_ids = simple_ref_ids(fields);
                for field in &parsed.fields {
                    if ref_ids.contains(&field.key.as_str()) {
                        push_field_edges(field, from, &mut edges);
                    }
                }
            }
            // A repeatable section: each item's ref-typed leaf contributes a forward
            // edge from the containing doc. The general edge-extractor completion
            // (M33 inc-2 / T3) — previously a bare skip. No shipped schema carries a
            // repeatable-item ref, so production edge sets are unchanged.
            SectionBody::Repeatable { repeatable } => {
                let ref_ids = repeatable_ref_ids(&repeatable.block);
                if ref_ids.is_empty() {
                    continue;
                }
                for item in &parsed.items {
                    for field in &item.fields {
                        if ref_ids.contains(&field.key.as_str()) {
                            push_field_edges(field, from, &mut edges);
                        }
                    }
                }
            }
        }
    }

    edges
}

/// Push the forward edge(s) one ref-typed parsed field contributes under `from` —
/// one per value (a list-valued ref → one edge per element, in value order). The
/// shared value fan-out for both the simple-section and repeatable-item paths.
fn push_field_edges(field: &crate::field_block::Field, from: &str, edges: &mut Vec<Edge>) {
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

/// The `ref`-typed field ids declared by a simple section's field group.
fn simple_ref_ids(fields: &[Field]) -> Vec<&str> {
    fields
        .iter()
        .filter(|f| f.ty == FieldType::Ref)
        .map(|f| f.id.as_str())
        .collect()
}

/// The `ref`-typed field ids declared by a repeatable item block's `Leaf::Field`
/// leaves — slot and nested-repeatable leaves carry no edge.
fn repeatable_ref_ids(block: &[Leaf]) -> Vec<&str> {
    block
        .iter()
        .filter_map(|leaf| match leaf {
            Leaf::Field(f) if f.ty == FieldType::Ref => Some(f.id.as_str()),
            _ => None,
        })
        .collect()
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
            crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
                .expect("adr.yaml loads"),
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

## Options
Alternatives were weighed and rejected.

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

## Options
Alternatives were weighed and rejected.

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

    /// REGRESSION (M38 placement census): a **placement** doctype's committed instance
    /// lives at its literal `placement.file` (repo-root-relative), addressed by the fixed
    /// singleton slug (= type id) — so `rebuild_committed` must extract its forward edges
    /// from that literal file, not skip it for lacking a `location:`. Pre-fix, the
    /// `location.as_deref() else continue` guard dropped it, so `vision`'s `grounded-in`
    /// edge went unindexed and `jigc rename` repointed nothing on it.
    #[test]
    fn rebuild_extracts_placement_doctype_edges() {
        let root = TempRoot::new("placement-edges");
        // An ADR schema re-homed as a placement doctype at the repo-root literal FOO.md:
        // one committed instance, no `location:`, identity is the fixed `<type>:<type>`.
        let mut schema =
            crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
                .expect("adr.yaml loads");
        schema.location = None;
        schema.placement = Some(crate::schema::Placement {
            file: "FOO.md".to_string(),
        });
        let mut schemas = BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        // The committed placement instance carries a `supersedes` ref (mirrors vision's
        // `grounded-in`): a placement doctype's edge must be indexed exactly as a located
        // doctype's.
        std::fs::write(root.path().join("FOO.md"), ADR_B).expect("write placement doc");

        let index = rebuild_committed(root.path(), &schemas, "HEADSHA");
        assert_eq!(
            index.edges,
            vec![Edge {
                from: "adr:adr".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }],
            "the placement doc's forward edge is extracted from its literal file",
        );

        // The rename repoint drives `referrers_of` over this index — it must now find the
        // placement doc as a referrer of the target (pre-fix: dropped → repoints nothing).
        let referrers = referrers_of(&index, "adr:single-node-cache");
        assert_eq!(
            referrers.len(),
            1,
            "the placement doc is a referrer of the target"
        );
        assert_eq!(referrers[0].from, "adr:adr");
        assert_eq!(referrers[0].relation, "supersedes");
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

    /// `finalize` post-commit invalidation (lifecycle site 5): after a commit lands at a
    /// new HEAD, [`invalidate`] leaves the on-disk index **stale** so the next
    /// [`load_committed`] against the new HEAD rebuilds + re-stamps rather than serving a
    /// pre-commit edge set. Asserts the persisted stamp ≠ the new HEAD post-invalidate,
    /// and that the next load returns the new stamp.
    #[test]
    fn post_commit_invalidates_edge_stamp() {
        let repo = TempRoot::new("invalidate-repo");
        let jigc = TempRoot::new("invalidate-jigc");
        write_committed_adrs(repo.path());
        let schemas = schemas();

        // Pre-commit: the index is built + persisted against the old HEAD "before".
        let before = load_committed(repo.path(), jigc.path(), &schemas, "before");
        assert_eq!(before.stamp, "before");
        assert!(
            EdgeIndex::path_in(jigc.path()).exists(),
            "the index is persisted pre-commit"
        );

        // Post-commit: HEAD is now "after". Invalidation removes the persisted index.
        invalidate(jigc.path()).expect("invalidation succeeds");
        assert!(
            !EdgeIndex::path_in(jigc.path()).exists(),
            "invalidation removes the persisted index so the next read rebuilds"
        );

        // The next load against the new HEAD rebuilds and re-stamps with "after" — it
        // cannot serve the stale pre-commit stamp.
        let after = load_committed(repo.path(), jigc.path(), &schemas, "after");
        assert_eq!(
            after.stamp, "after",
            "the next read rebuilds against the new HEAD"
        );
        let on_disk: EdgeIndex = serde_json::from_slice(
            &std::fs::read(EdgeIndex::path_in(jigc.path())).expect("read re-stamped index"),
        )
        .expect("persisted index parses");
        assert_ne!(
            on_disk.stamp, "before",
            "the persisted stamp is no longer the pre-commit HEAD"
        );

        // Invalidation is idempotent / best-effort: a second call on an absent file is Ok.
        invalidate(jigc.path()).expect("invalidate is idempotent");

        // Re-invalidate so the post-commit contract holds even after the re-stamp above:
        // the on-disk index is gone, the next read rebuilds against whatever HEAD it gets.
        invalidate(jigc.path()).expect("invalidation succeeds");
        assert!(
            !EdgeIndex::path_in(jigc.path()).exists(),
            "the index is gone after invalidation"
        );
    }

    /// A minimal `prd` doctype (the `derived-from` target) + a `spec` carrying the
    /// `derived-from → prd` inverse-card `1..*` obligation, alongside `adr` — the
    /// schema map the advisory-route-floor corpus drives.
    fn route_floor_schemas() -> BTreeMap<String, Schema> {
        const PRD_YAML: &[u8] = b"\
type: prd
location: prds/
id-from: title
sections:
  - id: overview
    slot: { hint: The product requirement. }
";
        const SPEC_YAML: &[u8] = b"\
type: spec
location: specs/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: prd, card: \"0..1\", inverse: has-specs, inverse-card: \"1..*\" }
  - id: goal
    slot: { hint: One sentence. }
";
        let types = crate::schema::dev_pack_field_types();
        let mut m = schemas(); // adr
        m.insert(
            "prd".to_string(),
            crate::schema::load_schema_with_types(PRD_YAML, &types).expect("prd fixture loads"),
        );
        m.insert(
            "spec".to_string(),
            crate::schema::load_schema_with_types(SPEC_YAML, &types).expect("spec fixture loads"),
        );
        m
    }

    /// A committed ADR whose Decision slot carries a **dangling managed mention**
    /// `#adr:ghost` (`adr` is a known doctype; `decisions/ghost.md` is never committed).
    const ADR_WITH_DANGLING_MENTION: &str = "\
---
status: accepted
date: 2026-07-11
---

# Note with a mention

## Context
Some context.

## Options
Alternatives were weighed and rejected.

## Decision
This supersedes the prior approach; see #adr:ghost for the record.

## Consequences
None.
";

    /// A committed PRD with **zero inbound `derived-from` edges** — below the
    /// `1..*` inverse-card minimum.
    const PRD_UNREFERENCED: &str = "\
# Real product requirement

## Overview
The product must do the thing.
";

    /// The **advisory-route floor** (`command-output-contract.md` → §route; `validation.md`
    /// → the advisory-route floor): a store sweep over a corpus carrying a dangling mention
    /// **and** a below-inverse-min doc emits **no finding with `route: null`**, and the
    /// dangling-mention finding's route names a **repair** action (human: correct or drop the
    /// mention), never the informational "no action needed". Drives the emitted findings of
    /// the two store families whose constructors carried no route.
    #[test]
    fn store_advisories_all_carry_a_route_and_the_mention_route_is_a_repair() {
        let root = TempRoot::new("route-floor");
        let schemas = route_floor_schemas();
        std::fs::create_dir_all(root.path().join("decisions")).expect("mk decisions/");
        std::fs::write(
            root.path().join("decisions/note.md"),
            ADR_WITH_DANGLING_MENTION,
        )
        .expect("write adr");
        std::fs::create_dir_all(root.path().join("prds")).expect("mk prds/");
        std::fs::write(root.path().join("prds/real.md"), PRD_UNREFERENCED).expect("write prd");

        let committed = rebuild_committed(root.path(), &schemas, "route-floor");
        let mut findings = inverse_cardinality_store(&committed, root.path(), &schemas);
        findings.extend(mention_resolves_store(root.path(), &schemas));

        // The corpus produces exactly the two families under test.
        assert!(
            findings
                .iter()
                .any(|f| f.code == "schema-completeness.inverse-cardinality"),
            "the unreferenced PRD must surface a below-inverse-min advisory: {findings:?}",
        );
        let mention = findings
            .iter()
            .find(|f| f.code == "schema-conformance.mention-resolves")
            .expect("the dangling mention must surface a mention-resolves advisory");

        // The floor: no advisory carries `route: null`.
        for f in &findings {
            assert!(
                f.route.is_some(),
                "every advisory carries a route (the floor, never null): {f:?}",
            );
        }

        // The mention route is a REPAIR action, not the informational no-op.
        let route = mention.route.as_deref().expect("mention carries a route");
        assert!(
            route.contains("correct") && route.contains("drop"),
            "the dangling-mention route names the repair (correct or drop the mention): {route:?}",
        );
        assert!(
            !route.contains("no action needed"),
            "a dangling mention is a real fix, never the informational 'no action needed': {route:?}",
        );
    }
}

#[cfg(test)]
mod overlay_tests {
    //! The working overlay (lifecycle site 2) + `ref-resolves` forward-ref integrity
    //! over the two reachable surfaces. The overlay is in-memory, never persisted; a
    //! task-touched edge whose target resolves in neither surface blocks.

    use super::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-overlay-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
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
            crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
                .expect("adr.yaml loads"),
        );
        m
    }

    /// A committed ADR `A` (the supersede target), with no outgoing ref.
    const ADR_A: &str = "\
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
A single in-memory node keeps session lookups sub-millisecond.

## Consequences
A cold node loses its sessions; clients re-authenticate.
";

    /// A working-area ADR `B` whose `supersedes` points at `to`.
    fn adr_b_superseding(to: &str) -> String {
        format!(
            "\
---
status: accepted
date: 2026-05-30
supersedes: {to}
---

# Shared redis session cache

## Context
A single node is a single point of failure.

## Options
Alternatives were weighed and rejected.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
"
        )
    }

    /// Commit ADR `A` at its canonical `decisions/single-node-cache.md`.
    fn commit_adr_a(repo_root: &Path) {
        let dir = repo_root.join("decisions");
        std::fs::create_dir_all(&dir).expect("mk decisions/");
        std::fs::write(dir.join("single-node-cache.md"), ADR_A).expect("write A");
    }

    /// Stage a working-area ADR `B` at `<task_dir>/docs/adr:<slug>.md`.
    fn stage_adr_b(task_dir: &Path, slug: &str, supersedes: &str) {
        let docs = task_dir.join(DOCS_DIR);
        std::fs::create_dir_all(&docs).expect("mk docs/");
        std::fs::write(
            docs.join(format!("adr:{slug}.md")),
            adr_b_superseding(supersedes),
        )
        .expect("stage B");
    }

    /// PASSES when the supersede target is in the committed store: committed `A`,
    /// task working area holds `B` with `supersedes: adr:single-node-cache` → the
    /// overlay layers B's edge over the committed index and `ref_resolves` returns
    /// no findings (surface a).
    #[test]
    fn ref_resolves_passes_when_target_committed() {
        let repo = TempRoot::new("pass-repo");
        let task = TempRoot::new("pass-task");
        commit_adr_a(repo.path());
        stage_adr_b(
            task.path(),
            "shared-redis-session-cache",
            "adr:single-node-cache",
        );

        // The committed index over A (no outgoing edge); the overlay adds B→A.
        let committed = rebuild_committed(repo.path(), &schemas(), "HEAD");
        assert!(committed.edges.is_empty(), "A has no outgoing ref");

        let overlay = overlay_working(&committed, task.path(), &schemas());
        assert_eq!(
            overlay.task_edges,
            vec![Edge {
                from: "adr:shared-redis-session-cache".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }],
            "the overlay layers B's supersedes edge"
        );

        let findings = ref_resolves(&overlay, repo.path(), task.path(), &schemas());
        assert!(
            findings.is_empty(),
            "the supersede target is committed → no findings, got {findings:?}"
        );

        // The overlay was NOT persisted: no edges.json under the task area.
        assert!(
            !task.path().join("index").join("edges.json").exists(),
            "the working overlay is never persisted"
        );
    }

    /// Surface b: the supersede target is created in the SAME task's working area
    /// (not committed) → still resolves.
    #[test]
    fn ref_resolves_passes_when_target_in_same_task() {
        let repo = TempRoot::new("same-repo");
        let task = TempRoot::new("same-task");
        // No committed A. The target `adr:single-node-cache` is staged in this task.
        let docs = task.path().join(DOCS_DIR);
        std::fs::create_dir_all(&docs).expect("mk docs/");
        std::fs::write(docs.join("adr:single-node-cache.md"), ADR_A).expect("stage target");
        stage_adr_b(
            task.path(),
            "shared-redis-session-cache",
            "adr:single-node-cache",
        );

        let committed = rebuild_committed(repo.path(), &schemas(), "HEAD");
        let overlay = overlay_working(&committed, task.path(), &schemas());

        let findings = ref_resolves(&overlay, repo.path(), task.path(), &schemas());
        assert!(
            findings.is_empty(),
            "the target created in the same task resolves (surface b), got {findings:?}"
        );
    }

    /// BLOCKS when the supersede target dangles: `supersedes: adr:typo-nonexistent`
    /// resolves in neither surface → exactly one blocking
    /// `schema-conformance.ref-resolves` finding whose message names the three
    /// routing options (fix / create-in-task / drop). The overlay is not persisted.
    #[test]
    fn ref_resolves_blocks_when_target_dangles() {
        let repo = TempRoot::new("dangle-repo");
        let task = TempRoot::new("dangle-task");
        commit_adr_a(repo.path());
        stage_adr_b(
            task.path(),
            "shared-redis-session-cache",
            "adr:typo-nonexistent",
        );

        let committed = rebuild_committed(repo.path(), &schemas(), "HEAD");
        let overlay = overlay_working(&committed, task.path(), &schemas());

        let findings = ref_resolves(&overlay, repo.path(), task.path(), &schemas());
        assert_eq!(
            findings.len(),
            1,
            "a dangling forward-ref yields exactly one finding, got {findings:?}"
        );
        let f = &findings[0];
        assert_eq!(f.severity, Severity::Blocking);
        assert_eq!(f.code, "schema-conformance.ref-resolves");
        assert!(
            f.message.contains("adr:typo-nonexistent"),
            "names the unresolved target: {}",
            f.message
        );
        // The three routing options are named in the message.
        assert!(
            f.message.contains("fix"),
            "names the fix option: {}",
            f.message
        );
        assert!(
            f.message.contains("create the target in this task"),
            "names the create-in-task option: {}",
            f.message
        );
        assert!(
            f.message.contains("drop"),
            "names the drop option: {}",
            f.message
        );
        assert!(
            f.location.is_some(),
            "the block is located at the source doc"
        );

        // The overlay was NOT persisted to .jigc/index/edges.json (byte-unchanged: it
        // never existed). No edges.json was written under the task area at all.
        assert!(
            !task.path().join("index").join("edges.json").exists(),
            "the working overlay is never persisted"
        );
    }

    /// (M41 Inc-3 T1, the done-criterion) A **`0..*` ref with two dangling targets** fans
    /// **two** blocking `ref-resolves` findings, each with a **distinct** stable
    /// `(code, target)` key — `adr:<from>#supersedes/<type>:<to-slug>` — so a driver dedupes them
    /// apart rather than collapsing two real breaks onto one bare-source key
    /// (`command-output-contract.md` → the stable finding key: `ref-resolves` fans one keyed
    /// finding per dangling target).
    #[test]
    fn dangling_list_ref_keys_each_target_distinctly() {
        let repo = TempRoot::new("dangle-list-repo");
        let task = TempRoot::new("dangle-list-task");
        // A `0..*` supersedes with two targets, both absent from the store and this task.
        let body = "\
---
status: accepted
date: 2026-05-30
supersedes: [adr:ghost-one, adr:ghost-two]
---

# Shared redis session cache

## Context
A single node is a single point of failure.

## Options
Alternatives were weighed and rejected.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";
        let docs = task.path().join(DOCS_DIR);
        std::fs::create_dir_all(&docs).expect("mk docs/");
        std::fs::write(docs.join("adr:shared-redis-session-cache.md"), body).expect("stage B");

        let committed = rebuild_committed(repo.path(), &schemas(), "HEAD");
        let overlay = overlay_working(&committed, task.path(), &schemas());
        let findings = ref_resolves(&overlay, repo.path(), task.path(), &schemas());

        assert_eq!(
            findings.len(),
            2,
            "two dangling targets fan two findings, got {findings:?}"
        );
        let keys: Vec<crate::finding::FindingKey> = findings.iter().map(Finding::key).collect();
        // Both are `ref-resolves`, distinguished only by their `#supersedes/<type>:<to-slug>`.
        assert!(
            keys.iter()
                .all(|k| k.code == "schema-conformance.ref-resolves"),
            "both carry the ref-resolves code: {keys:?}",
        );
        let targets: std::collections::BTreeSet<Option<String>> =
            keys.iter().map(|k| k.target.clone()).collect();
        assert_eq!(
            targets,
            [
                Some("adr:shared-redis-session-cache#supersedes/adr:ghost-one".to_string()),
                Some("adr:shared-redis-session-cache#supersedes/adr:ghost-two".to_string()),
            ]
            .into_iter()
            .collect(),
            "each dangling target gets a distinct #supersedes/<type>:<to-slug> key: {keys:?}",
        );
        // The two keys are genuinely distinct (the collision the fragment prevents).
        assert_ne!(keys[0].target, keys[1].target, "keys must not collide");
    }
}

#[cfg(test)]
mod commit_overlay_tests {
    //! The **first transient-source edge** (`DECISIONS.md` 2026-06-01 → M3 Increment 1
    //! T4): a working-area `commit` instance whose `implements` ref points at a
    //! `spec:<slug>` emits a forward edge into the overlay, and `ref_resolves` walks the
    //! committed ∪ working surfaces exactly as the `supersedes` overlay tests do — the
    //! only new surface is a `commit`-typed `from`. PASSES when the committed
    //! `specs/<slug>.md` exists; BLOCKS when the target dangles.

    use super::*;

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");
    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-commit-overlay-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
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

    /// `commit` (transient source) + `spec` (committed target type) — the two schemas
    /// this transient-source edge spans.
    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "commit".to_string(),
            crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads"),
        );
        m.insert(
            "spec".to_string(),
            crate::schema::load_schema_with_types(
                SPEC_YAML,
                &crate::schema::dev_pack_field_types(),
            )
            .expect("spec.yaml loads"),
        );
        m
    }

    /// A working-area `commit` instance whose `header` carries `implements: <to>` —
    /// front matter (`type` + the `implements` ref), the `# H1`, then the slot
    /// sections. Mirrors the canonical commit render shape.
    fn commit_implementing(to: &str) -> String {
        format!(
            "\
---
type: feat
implements: {to}
---

# Add the rate limiter

## Summary

Add a per-client rate limit at the gateway.

## Body

Centralize limiting at the gateway.

## Trailers
"
        )
    }

    /// A committed `spec` instance (the `implements` target). It carries no outgoing
    /// ref, so it contributes no edges; only its existence at the canonical path matters
    /// for `ref_resolves` surface a.
    const SPEC_BODY: &str = "\
---
---

# Rate limiting

## Goal

Bound per-client request rate at the gateway.

## Context

Unbounded clients exhaust gateway capacity.

## Criteria
";

    /// Commit a `spec` at its canonical `specs/<slug>.md`.
    fn commit_spec(repo_root: &Path, slug: &str) {
        let dir = repo_root.join("specs");
        std::fs::create_dir_all(&dir).expect("mk specs/");
        std::fs::write(dir.join(format!("{slug}.md")), SPEC_BODY).expect("write spec");
    }

    /// Stage the working-area `commit` at `<task_dir>/docs/commit:<id>.md`.
    fn stage_commit(task_dir: &Path, id: &str, implements: &str) {
        let docs = task_dir.join(DOCS_DIR);
        std::fs::create_dir_all(&docs).expect("mk docs/");
        std::fs::write(
            docs.join(format!("commit:{id}.md")),
            commit_implementing(implements),
        )
        .expect("stage commit");
    }

    /// PASSES (surface a): the transient `commit` source's `implements` edge points at
    /// a committed `spec:rate-limiting`; the overlay layers the edge over the (empty)
    /// committed index and `ref_resolves` returns no findings.
    #[test]
    fn ref_resolves_passes_when_spec_target_committed() {
        let repo = TempRoot::new("pass-repo");
        let task = TempRoot::new("pass-task");
        commit_spec(repo.path(), "rate-limiting");
        stage_commit(task.path(), "t-001", "spec:rate-limiting");

        // The committed index has no edges (the spec carries no outgoing ref).
        let committed = rebuild_committed(repo.path(), &schemas(), "HEAD");
        assert!(committed.edges.is_empty(), "the spec has no outgoing ref");

        let overlay = overlay_working(&committed, task.path(), &schemas());
        assert_eq!(
            overlay.task_edges,
            vec![Edge {
                from: "commit:t-001".to_string(),
                relation: "implements".to_string(),
                to: "spec:rate-limiting".to_string(),
            }],
            "the overlay layers the transient commit's implements edge"
        );

        let findings = ref_resolves(&overlay, repo.path(), task.path(), &schemas());
        assert!(
            findings.is_empty(),
            "the implements target is committed → no findings, got {findings:?}"
        );

        // The overlay was NOT persisted under the task area.
        assert!(
            !task.path().join("index").join("edges.json").exists(),
            "the working overlay is never persisted"
        );
    }

    /// BLOCKS: the transient `commit` source's `implements` points at a non-existent
    /// `spec:typo-nonexistent` — resolves in neither surface → exactly one blocking
    /// `schema-conformance.ref-resolves` finding naming the three routing options.
    #[test]
    fn ref_resolves_blocks_when_spec_target_dangles() {
        let repo = TempRoot::new("dangle-repo");
        let task = TempRoot::new("dangle-task");
        commit_spec(repo.path(), "rate-limiting");
        stage_commit(task.path(), "t-001", "spec:typo-nonexistent");

        let committed = rebuild_committed(repo.path(), &schemas(), "HEAD");
        let overlay = overlay_working(&committed, task.path(), &schemas());

        let findings = ref_resolves(&overlay, repo.path(), task.path(), &schemas());
        assert_eq!(
            findings.len(),
            1,
            "a dangling transient-source forward-ref yields exactly one finding, got {findings:?}"
        );
        let f = &findings[0];
        assert_eq!(f.severity, Severity::Blocking);
        assert_eq!(f.code, "schema-conformance.ref-resolves");
        assert!(
            f.message.contains("spec:typo-nonexistent"),
            "names the unresolved target: {}",
            f.message
        );
        assert!(
            f.message.contains("fix"),
            "names the fix option: {}",
            f.message
        );
        assert!(
            f.message.contains("create the target in this task"),
            "names the create-in-task option: {}",
            f.message
        );
        assert!(
            f.message.contains("drop"),
            "names the drop option: {}",
            f.message
        );
        assert!(
            f.location.is_some(),
            "the block is located at the source commit doc"
        );

        assert!(
            !task.path().join("index").join("edges.json").exists(),
            "the working overlay is never persisted"
        );
    }
}

#[cfg(test)]
mod arch_doc_cites_tests {
    //! The n→n `arch-doc —cites→ adr` edge (M13 Increment 4 / T1): a header
    //! `cites` ref list emits **one forward edge per element** through `doc_edges`,
    //! exactly as `supersedes` does — but list-valued, so the per-element fan-out is
    //! the contract under test. The repeatable `components` anchors carry
    //! `implemented-by` (a `code-anchor`, not a `ref`, in a repeatable section), so
    //! they contribute **no** edges — only the doc-level `cites` does.

    use super::*;

    const ARCH_DOC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/arch-doc.yaml");

    fn arch_doc_schema() -> Schema {
        crate::schema::load_schema_with_types(ARCH_DOC_YAML, &crate::schema::dev_pack_field_types())
            .expect("arch-doc.yaml loads")
    }

    /// A canonical `arch-doc` instance: a `meta` header carrying `cites` over **two**
    /// adr targets, an `overview` slot, and **two** `components` items each anchoring
    /// `implemented-by` at distinct code. Authored in the frozen byte form the
    /// canonical writer emits.
    const ARCH_DOC_SRC: &str = "\
---
cites: [adr:single-node-cache, adr:distributed-cache]
---

# The edge index

## Overview

The edge index is a rebuildable map of forward cross-reference edges.

## Components

### The rebuild  {#rebuild}

Walks the committed docs and emits one forward edge per present ref field.

<!-- fields -->
- implemented-by: crates/engine/src/index.rs#rebuild_committed

### The overlay  {#overlay}

Layers the active task's working-area edges over the committed index in memory.

<!-- fields -->
- implemented-by: crates/engine/src/index.rs#overlay_working
";

    /// `doc_edges` emits **one** `(arch-doc:<slug>, cites, adr:<t>)` edge per `cites`
    /// element — and nothing for the repeatable `implemented-by` anchors (a
    /// repeatable-section non-ref leaf). Two cites elements → exactly two edges, in
    /// element order.
    #[test]
    fn doc_edges_emits_one_cites_edge_per_element() {
        let schema = arch_doc_schema();
        let mut src = ARCH_DOC_SRC.to_string();
        crate::parse::strip_leading_bom(&mut src);
        let doc = crate::parse::parse_sections(&schema, &src).expect("arch-doc parses");

        let edges = doc_edges(&schema, &doc, "arch-doc:the-edge-index");
        assert_eq!(
            edges,
            vec![
                Edge {
                    from: "arch-doc:the-edge-index".to_string(),
                    relation: "cites".to_string(),
                    to: "adr:single-node-cache".to_string(),
                },
                Edge {
                    from: "arch-doc:the-edge-index".to_string(),
                    relation: "cites".to_string(),
                    to: "adr:distributed-cache".to_string(),
                },
            ],
            "one cites edge per element, in order; the repeatable implemented-by \
             anchors contribute no edges",
        );
    }
}

#[cfg(test)]
mod repeatable_ref_tests {
    //! T3 (M33 inc-2): the edge-extractor recurses repeatable item bodies — a
    //! ref-typed `Leaf::Field` in a repeatable item contributes a forward edge,
    //! exactly as a simple-section ref does. No shipped schema carries a
    //! repeatable-item ref (so production edge sets are byte-unchanged), so this
    //! exercises the branch over a synthetic schema. Non-ref repeatable leaves
    //! (slots, code-anchors) still contribute nothing — the `arch_doc_cites_tests`
    //! invariant.

    use super::*;

    /// A synthetic schema whose repeatable `criteria` block carries a `ref` leaf
    /// (`traces-to → adr`) alongside the id-source `title` field and a `statement`
    /// slot. No shipped schema has this shape; the `ref` type is engine-native, so
    /// the bare loader resolves it without any pack types.
    fn schema_with_repeatable_ref() -> Schema {
        let yaml = b"\
type: spec
location: specs/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"The criterion, testably phrased.\" } }
        - { id: traces-to, type: ref, to: adr }
";
        crate::schema::load_schema(yaml).expect("synthetic repeatable-ref schema loads")
    }

    /// An instance with two `criteria` items, each carrying the repeatable-item
    /// `traces-to` ref in its sentinelled per-item field block.
    const SRC: &str = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Rate limit holds  {#rate-limit}
The gateway rejects the 101st request in a 60s window.

<!-- fields -->
- traces-to: adr:single-node-cache

### Burst allowance  {#burst-allowance}
A short burst above the limit is tolerated for 2s.

<!-- fields -->
- traces-to: adr:distributed-cache
";

    /// The repeatable-item ref enters the edge index: `doc_edges` emits one forward
    /// edge per item's `traces-to` value, from the containing doc — the branch that
    /// was a bare `continue` before this task (silently inert). Edges are in item
    /// order (the same physical order the simple-section path emits in).
    #[test]
    fn doc_edges_emits_an_edge_per_repeatable_item_ref() {
        let schema = schema_with_repeatable_ref();
        let mut src = SRC.to_string();
        crate::parse::strip_leading_bom(&mut src);
        let doc = crate::parse::parse_sections(&schema, &src).expect("synthetic instance parses");

        let edges = doc_edges(&schema, &doc, "spec:auth-flow");
        assert_eq!(
            edges,
            vec![
                Edge {
                    from: "spec:auth-flow".to_string(),
                    relation: "traces-to".to_string(),
                    to: "adr:single-node-cache".to_string(),
                },
                Edge {
                    from: "spec:auth-flow".to_string(),
                    relation: "traces-to".to_string(),
                    to: "adr:distributed-cache".to_string(),
                },
            ],
            "one forward edge per repeatable-item ref value, in item order",
        );
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use crate::write::{self, Instance, SectionContent};
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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
                        id: "options".to_string(),
                        slot: Some("Alternatives were weighed and rejected.".to_string()),
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

#[cfg(test)]
mod referrers_tests {
    //! The **inverse-edge reverse-walk** ([`referrers_of`]) the `jigc rename` repoint
    //! step drives — every forward edge whose `to == target`, across the three persisted
    //! ref-fields, deterministic order, empty for a doc with no inbound edge.

    use super::*;

    fn edge(from: &str, relation: &str, to: &str) -> Edge {
        Edge {
            from: from.to_string(),
            relation: relation.to_string(),
            to: to.to_string(),
        }
    }

    /// An index spanning all three persisted ref-fields: `adr:old` is referenced by an
    /// `adr.supersedes` edge **and** an `arch-doc.cites` edge; `prd:old` by a
    /// `spec.derived-from` edge; plus unrelated edges that must NOT match.
    fn index() -> EdgeIndex {
        let mut edges = vec![
            edge("adr:b", "supersedes", "adr:old"),
            edge("arch-doc:overview", "cites", "adr:old"),
            edge("spec:auth-flow", "derived-from", "prd:old"),
            // unrelated: a referrer of a different target, and a self-less doc.
            edge("adr:c", "supersedes", "adr:other"),
            edge("arch-doc:overview", "cites", "adr:unrelated"),
        ];
        edges.sort();
        edges.dedup();
        EdgeIndex {
            stamp: "STAMP".to_string(),
            edges,
        }
    }

    /// `referrers_of` returns **exactly** the inbound edges of the target — across
    /// the `supersedes` and `cites` ref-fields for `adr:old`, and the `derived-from`
    /// ref-field for `prd:old` — in the index's `(from, relation, to)` sort order, and
    /// nothing pointing elsewhere.
    #[test]
    fn referrers_of_returns_exactly_the_inbound_edges() {
        let index = index();

        let adr_old = referrers_of(&index, "adr:old");
        assert_eq!(
            adr_old,
            vec![
                &edge("adr:b", "supersedes", "adr:old"),
                &edge("arch-doc:overview", "cites", "adr:old"),
            ],
            "both the supersedes and cites referrers, sorted, nothing else"
        );

        let prd_old = referrers_of(&index, "prd:old");
        assert_eq!(
            prd_old,
            vec![&edge("spec:auth-flow", "derived-from", "prd:old")],
            "the derived-from referrer (the third persisted ref-field)"
        );
    }

    /// A target with no inbound edge — every doc with no referrer — yields an empty
    /// `Vec` (the no-op rename case: nothing to repoint).
    #[test]
    fn referrers_of_is_empty_for_no_inbound() {
        let index = index();
        assert!(
            referrers_of(&index, "adr:never-referenced").is_empty(),
            "a doc with no inbound edge has no referrers"
        );
        // An empty index has no referrers for anything.
        assert!(referrers_of(&EdgeIndex::default(), "adr:old").is_empty());
    }
}
