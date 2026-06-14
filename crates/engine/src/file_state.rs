//! The `file-state` hash record — the raw-byte drift hash, the
//! `.jigc/state/file-state.json` `path → hex-hash` map (load/save), the engine-native
//! `file-state` probe, and the full OOB reconciliation classifier.
//!
//! The store layer is a raw-byte [`blake3`] hash and byte-stable JSON I/O over the
//! record. Built atop it: [`file_state`], the inc-4 working-area/commit-only probe
//! (baseline-adopt + hash-matches); [`reconcile_committed`], the inc-5 OOB
//! reconciliation state machine over a *committed* managed doc — absorb /
//! conformance-block / conflict-block (`reconciliation.md` → The state machine;
//! `validation.md` → the `file-state` probe); and [`detect_rename`], the separate
//! rename classifier for a tracked path gone *missing* — strong signal (a
//! content-hash-matching untracked path → suspected `git mv`) / weak signal
//! (restore), routed to human-side revert, never auto-rewriting referrer refs
//! (`reconciliation.md` → Rename detection / No silent rename).
//!
//! - **Raw-byte hash** (`parsing.md` → Round-trip guarantees → Drift hash): the
//!   `file ↔ CLI-state` check hashes the file's *raw bytes*, so a conformant
//!   no-op read/write doesn't drift and first-touch canonicalizations re-baseline
//!   it. `blake3` is the chosen algorithm (`DECISIONS.md` 2026-05-31 →
//!   Cross-cutting build crates → Hashing = blake3; inc-4 planning pin).
//! - **The record** (`storage.md` → `.jigc/state/` hash records): one JSON map
//!   `path → hex-hash` at `<jigc_root>/state/file-state.json`, gitignored and
//!   rebuildable. Keyed to "last-known-good committed state"; the recorded hash
//!   updates at exactly three sites (baseline-adopt / absorb / commit —
//!   `reconciliation.md` → Hash re-baselining), all of which are *callers* of
//!   [`FileStateRecord::save`], not this module's concern.

use crate::finding::{Finding, Location, Severity};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The `file-state.json` filename inside `<jigc_root>/state/`.
const FILE_STATE_FILE: &str = "file-state.json";

/// Hash a file's **raw bytes** to its lowercase-hex `blake3` digest.
///
/// The drift hash is over raw bytes, never the canonicalized form, so a
/// conformant no-op read/write does not register as drift (`parsing.md` → Drift
/// hash). The output is the 64-char lowercase hex of the 32-byte `blake3` digest.
pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// The `file-state` record: a map from managed-doc path to its recorded raw-byte
/// hex hash (`storage.md` → `.jigc/state/` hash records).
///
/// A [`BTreeMap`] so serialization is **deterministic** (path-sorted) — the JSON
/// bytes are stable across runs regardless of insertion order, which the
/// byte-stable golden depends on. Paths are stored as their string form; the
/// record carries no schema version of its own (it is a rebuildable cache,
/// re-derivable from the committed docs at any time).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStateRecord {
    /// `path → hex-hash`, path-sorted for deterministic output.
    pub hashes: BTreeMap<String, String>,
}

impl FileStateRecord {
    /// An empty record.
    pub fn new() -> Self {
        Self::default()
    }

    /// The record's on-disk location under a `.jigc/` home.
    pub fn path_in(jigc_root: &Path) -> PathBuf {
        jigc_root.join("state").join(FILE_STATE_FILE)
    }

    /// Record `hash` for `path` (overwriting any prior hash for that path).
    pub fn record(&mut self, path: impl Into<String>, hash: impl Into<String>) {
        self.hashes.insert(path.into(), hash.into());
    }

    /// The recorded hash for `path`, if any.
    pub fn get(&self, path: &str) -> Option<&str> {
        self.hashes.get(path).map(String::as_str)
    }

    /// Serialize to the frozen on-disk byte form: pretty JSON, path-sorted, one
    /// trailing newline (golden-locked, matching the `base.json` convention).
    pub fn to_bytes(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("FileStateRecord serializes");
        s.push('\n');
        s
    }

    /// Save the record to `<jigc_root>/state/file-state.json`, creating the
    /// `state/` dir if absent.
    pub fn save(&self, jigc_root: &Path) -> std::io::Result<()> {
        let path = Self::path_in(jigc_root);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, self.to_bytes())
    }

    /// Load the record from `<jigc_root>/state/file-state.json`. A missing file
    /// is the *first-encounter* case (`reconciliation.md` → Absent-hash is not
    /// drift) and yields an empty record, never an error.
    pub fn load(jigc_root: &Path) -> std::io::Result<Self> {
        let path = Self::path_in(jigc_root);
        match std::fs::read(&path) {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Self::new()),
            Err(err) => Err(err),
        }
    }
}

/// The engine-native `file-state` probe over a set of files: classify each
/// `(path, bytes)` against the recorded baseline and emit the MVP findings.
///
/// Three states (the only two transitions the commit-only loop needs —
/// `validation.md` → Probes (`file-state`); `reconciliation.md` → Baseline
/// adoption; OOB absorb/conflict are inc-5):
///
/// - **`UNKNOWN`** (no recorded hash — first run / fresh checkout) → **baseline-adopt**:
///   the current on-disk content *is* the baseline. The probe records the hash on
///   `record` and emits an informational `file-state.baseline-adopt` finding
///   ([`Severity::Advisory`], no route). Absent-hash is not drift.
/// - **recorded + matching** → no finding (the file is `IN_SYNC`).
/// - **recorded + differing** → **drift**: a `file-state.hash-matches` finding —
///   blocking by default ([`Severity::Blocking`], the severity-inventory default),
///   tunable post-MVP — carrying a `reconcile <target>` route for the
///   reconciliation classifier ([`reconciliation.md`](../../../design/reconciliation.md)).
///   The recorded hash is **not** advanced on drift: re-baselining happens only at
///   the three named sites (adopt / absorb / commit), and drift is none of them.
///
/// Mutating: the `UNKNOWN → baseline` transition records into `record`, so the
/// caller persists the advanced record after the probe runs. The probe does no
/// I/O of its own — the caller supplies the raw bytes already read.
pub fn file_state(record: &mut FileStateRecord, files: &[(&str, &[u8])]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (path, bytes) in files {
        let current = hash_bytes(bytes);
        match record.get(path) {
            None => {
                record.record(*path, current);
                findings.push(baseline_adopt_finding(path));
            }
            Some(recorded) if recorded == current => {}
            Some(_) => findings.push(drift_finding(path)),
        }
    }
    findings
}

/// The full **OOB reconciliation classifier** for a single *committed* managed doc —
/// the state machine [`file_state`] only baseline-adopted in inc-4
/// (`reconciliation.md` → The state machine; `DECISIONS.md` 2026-05-31 → inc-5
/// Reconciliation OOB classifier; `storage.md` → Edge index lifecycle, site 3).
///
/// Reads the `(committed-state, task-state)` pair for the doc at `path` (its
/// file-state key and `<type>:<slug>` identity `from`) and routes deterministically:
///
/// - **`UNKNOWN`** (no recorded hash) → **baseline-adopt**: record the current hash,
///   emit the advisory `file-state.baseline-adopt` finding (absent-hash is not drift).
/// - **`IN_SYNC`** (recorded hash matches) → no finding (clean / task-only change —
///   the working-area writes are reconciled elsewhere, not here).
/// - **`DRIFTED + TOUCHED`** (`task_touched`) → **conflict-block**: both sides moved.
///   A blocking `reconciliation.conflict-block` finding carrying the explicit-discard
///   route; no silent merge, the hash and edge index are left untouched.
/// - **`DRIFTED + UNTOUCHED`** → the **parse classifier**: re-parse + schema-validate
///   the on-disk bytes against `schema`.
///   - clean → **absorb**: re-hash the recorded baseline forward, incrementally
///     update the committed `index` for the doc's edges (lifecycle site 3), and emit
///     the advisory `reconciliation.absorb` "external edit absorbed" finding.
///   - parse/schema fail → **conformance-block**: a blocking
///     `reconciliation.conformance-block` finding naming the file + the first precise
///     conformance error; the hash is **not** advanced and the index is **not**
///     touched (no auto-repair).
///
/// Mutating: re-hash advances `record` on baseline-adopt and absorb; absorb mutates
/// `index` in place. No I/O of its own — the caller supplies the on-disk `bytes` and
/// persists the advanced record / index after the run.
#[allow(clippy::too_many_arguments)]
pub fn reconcile_committed(
    record: &mut FileStateRecord,
    index: &mut crate::index::EdgeIndex,
    schema: &crate::schema::Schema,
    path: &str,
    from: &str,
    bytes: &[u8],
    task_touched: bool,
) -> Vec<Finding> {
    let current = hash_bytes(bytes);
    match record.get(path) {
        // UNKNOWN → baseline-adopt (absent-hash is not drift).
        None => {
            record.record(path, current);
            vec![baseline_adopt_finding(path)]
        }
        // IN_SYNC → clean / task-only change: nothing to reconcile here.
        Some(recorded) if recorded == current => Vec::new(),
        // DRIFTED + TOUCHED → conflict-block (both sides moved; no silent merge).
        Some(_) if task_touched => vec![conflict_block_finding(path)],
        // DRIFTED + UNTOUCHED → the parse classifier.
        Some(_) => {
            let source = String::from_utf8_lossy(bytes);
            match crate::parse::parse_sections(schema, &source) {
                Ok(doc) => {
                    let conformance = crate::validate::schema_conformance(schema, &source, &doc);
                    if conformance.iter().any(|f| f.severity == Severity::Blocking) {
                        // Schema-invalid → conformance-block, naming the first error.
                        vec![conformance_block_finding(
                            path,
                            conformance.into_iter().next(),
                        )]
                    } else {
                        // Clean → absorb: re-hash + incrementally update the index.
                        record.record(path, current);
                        index.absorb_doc(schema, from, &doc);
                        vec![absorb_finding(path)]
                    }
                }
                // Parse fail → conformance-block, naming the first parse error.
                Err(parse_findings) => {
                    vec![conformance_block_finding(
                        path,
                        parse_findings.into_iter().next(),
                    )]
                }
            }
        }
    }
}

/// Sweep the **committed store** for out-of-band drift and route it — the command-
/// surface wiring of [`reconcile_committed`] + [`detect_rename`] over every tracked
/// committed managed doc (`reconciliation.md` → Detection timing: the `task validate`
/// full sweep, shared by `finalize`'s preflight).
///
/// For each persisted (`location:`-bearing) schema in `schemas`, walks
/// `<repo_root>/<location>/*.md`, derives each doc's `<type>:<slug>` identity and its
/// `<location>/<slug>.md` record key, and:
///
/// - **present on disk** → [`reconcile_committed`] against the recorded baseline.
///   `task_touched` is true iff this task's working area stages the same identity at
///   `<task_dir>/docs/<type>:<slug>.md` (a `DRIFTED + TOUCHED` conflict). Absorb
///   mutates `record` + `index` in place; a block leaves both pinned.
/// - **recorded but now absent on disk** → [`detect_rename`] over the untracked
///   candidates (the on-disk `.md` files of that type with no recorded hash), routing
///   a suspected `git mv` (strong signal) or a restore (weak signal).
///
/// Mutating: `record` (baseline-adopt / absorb) and `index` (absorb) advance in place;
/// the caller persists them. Findings aggregate in a stable order: persisted schemas
/// by type, then committed docs by path-sorted slug, then rename findings for each
/// recorded-but-missing path (also type-then-path sorted). No I/O beyond reading the
/// committed `.md` bytes — the engine stays shell-free.
pub fn reconcile_committed_store(
    record: &mut FileStateRecord,
    index: &mut crate::index::EdgeIndex,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    repo_root: &Path,
    task_dir: &Path,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    // Snapshot the recorded committed paths *before* the reconcile loop mutates the
    // record (baseline-adopt records fresh paths), so rename detection below can tell a
    // genuinely-tracked path from one this same sweep just adopted.
    let recorded_at_entry: std::collections::BTreeSet<String> = record
        .hashes
        .keys()
        .filter(|p| persisted_committed_path(p, schemas))
        .cloned()
        .collect();
    // Track which recorded paths we saw on disk, so a recorded-but-missing path can be
    // routed to rename detection afterward.
    let mut seen_on_disk: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    for (ty, schema) in schemas {
        let Some(location) = schema.location.as_deref() else {
            continue; // a transient (location-less) type has no committed docs.
        };
        let dir = repo_root.join(location);
        let mut slugs: Vec<String> = match std::fs::read_dir(&dir) {
            Ok(entries) => entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
                .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
                .collect(),
            Err(_) => Vec::new(), // no committed docs of this type yet.
        };
        slugs.sort();

        for slug in &slugs {
            let path = format!("{location}{slug}.md");
            seen_on_disk.insert(path.clone());
            let from = format!("{ty}:{slug}");
            let Ok(bytes) = std::fs::read(repo_root.join(location).join(format!("{slug}.md")))
            else {
                continue; // read race: skip; the next sweep re-checks.
            };
            let task_touched = task_dir.join("docs").join(format!("{from}.md")).exists();
            findings.extend(reconcile_committed(
                record,
                index,
                schema,
                &path,
                &from,
                &bytes,
                task_touched,
            ));
        }
    }

    // Rename detection: a recorded committed path of a persisted type that is no longer
    // on disk is a suspected rename/deletion (`reconciliation.md` → Rename detection).
    // "Missing" means absent from DISK, not absent from the walk (the 2026-06-12
    // amendment): the walk is a non-recursive `<location>/*.md` glob, so a baselined
    // path outside it (e.g. a promoted owner-artifact under `completions/artifacts/`)
    // is checked for genuine disk presence — present-but-unwalked yields no finding and
    // stays baselined as-is. The untracked candidates are the on-disk `.md` files (of
    // any persisted type) that carry no recorded hash, paired with their raw-byte hash.
    let recorded_missing: Vec<String> = recorded_at_entry
        .iter()
        .filter(|p| !seen_on_disk.contains(*p))
        .filter(|p| !repo_root.join(p).exists())
        .cloned()
        .collect();
    if !recorded_missing.is_empty() {
        let untracked = untracked_committed(&recorded_at_entry, schemas, repo_root);
        let untracked_refs: Vec<(&str, String)> = untracked
            .iter()
            .map(|(p, h)| (p.as_str(), h.clone()))
            .collect();
        for path in recorded_missing {
            let from = identity_of(&path, schemas).unwrap_or_else(|| path.clone());
            let recorded_hash = record.get(&path).unwrap_or("").to_string();
            findings.extend(detect_rename(&path, &from, &recorded_hash, &untracked_refs));
        }
    }

    findings
}

/// The **read-only file↔CLI-state store twin** — `jigc validate`'s store-scope
/// file-state target (`validation.md` → Completing the envelope → read-only
/// file↔CLI-state at store scope; M20). The *detect-without-absorb* complement of
/// [`reconcile_committed_store`]: it compares each committed managed doc's on-disk
/// content hash to its recorded [`FileStateRecord`] and **reports** drift, opening
/// **no** record write and **never** routing through the mutating reconcile path
/// (no absorb, no re-baseline, no edge-index touch) — so it cannot silently
/// re-baseline the very drift the sweep exists to surface.
///
/// Mirrors only the [`reconcile_committed_store`] *walk shape* — per persisted
/// (`location:`-bearing) schema, the path-sorted `<location>/*.md` glob, the
/// `<location>/<slug>.md` record key — and routes each **present** doc by
/// [`FileStateRecord::get`] vs [`hash_bytes`] of the on-disk bytes:
///
/// - **content drift** (recorded hash ≠ on-disk hash) → exactly one blocking
///   `file-state.hash-matches` finding (the reused check id) carrying a
///   **store-scope route** (review / re-author through the owning workflow), never
///   the task-scope `reconcile <path>` route the mutating path emits.
/// - **un-baselined** (no recorded hash) → exactly one **advisory**
///   `file-state.un-baselined` finding — a distinct *not-yet-tracked* outcome,
///   neither drift nor silent-clean (informational on a fresh / pre-baseline
///   store, so it does not flip the exit code).
/// - **in-sync** (recorded hash matches) → no finding.
///
/// **Content drift on *present* docs only.** Recorded-but-now-missing docs
/// (rename / deletion — [`detect_rename`]'s task-scope concern) are **out of scope**:
/// the twin walks on-disk docs and never enumerates recorded-but-absent paths.
///
/// Read-only by construction: `record` is borrowed `&` (no mutation possible) and
/// the only I/O is reading the committed `.md` bytes. Findings aggregate in a stable
/// order — persisted schemas by type, then committed docs by path-sorted slug.
pub fn detect_committed_store(
    record: &FileStateRecord,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    repo_root: &Path,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for schema in schemas.values() {
        let Some(location) = schema.location.as_deref() else {
            continue; // a transient (location-less) type has no committed docs.
        };
        let dir = repo_root.join(location);
        let mut slugs: Vec<String> = match std::fs::read_dir(&dir) {
            Ok(entries) => entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
                .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
                .collect(),
            Err(_) => Vec::new(), // no committed docs of this type yet.
        };
        slugs.sort();

        for slug in &slugs {
            let path = format!("{location}{slug}.md");
            let Ok(bytes) = std::fs::read(dir.join(format!("{slug}.md"))) else {
                continue; // read race: skip; the next sweep re-checks.
            };
            let current = hash_bytes(&bytes);
            match record.get(&path) {
                None => findings.push(unbaselined_finding(&path)),
                Some(recorded) if recorded == current => {}
                Some(_) => findings.push(drift_store_finding(&path)),
            }
        }
    }
    findings
}

/// The store-scope drift finding: the on-disk content no longer matches the
/// recorded hash, surfaced read-only by [`detect_committed_store`]. Reuses the
/// `file-state.hash-matches` check id (no new id / knob) but carries the
/// **store-scope route** — there is no task and the mutating reconcile is barred, so
/// it routes to *review / re-author through the owning workflow*, never
/// `reconcile <path>` (`validation.md` → Completing the envelope: the reused
/// `hash-matches` id carries a store-scope route, not `reconcile <path>`).
fn drift_store_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "file-state.hash-matches",
        format!("on-disk content of `{path}` differs from the recorded state"),
        Some(Location::addressed(path, 1, 1)),
        Some(format!(
            "review the out-of-band edit to `{path}` and re-author it through the owning workflow"
        )),
    )
}

/// The advisory **un-baselined** finding: a committed managed doc with no recorded
/// hash, surfaced by [`detect_committed_store`]. A distinct *not-yet-tracked*
/// outcome — neither drift nor silent-clean — so a read-only twin that cannot adopt
/// a baseline (the mutating path's UNKNOWN resolution) never falsely reports an
/// untracked doc as clean. Advisory + no route: informational on a fresh /
/// pre-baseline store, it does not flip the exit code (`validation.md` → Completing
/// the envelope: UNKNOWN (un-baselined) ≠ clean).
fn unbaselined_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "file-state.un-baselined",
        format!("committed doc `{path}` is not yet baselined in the file-state record"),
        Some(Location::addressed(path, 1, 1)),
        None,
    )
}

/// Whether `path` (a `file-state` record key like `decisions/x.md`) lives under a
/// persisted schema's `location:` — i.e. it is a committed managed doc, not a staged
/// working-area key (`docs/<type>:<slug>.md`) or a code path. The `:` discriminates
/// the staged namespace even when a schema's `location:` is itself `docs/`: every
/// staged key carries one ([`crate::state`] mints `<type>:<slug>.md`), no committed
/// path can (committed docs are `<location>/<slug>.md`, slugs `[a-z0-9-]` per
/// [`crate::slug::slugify`]).
fn persisted_committed_path(
    path: &str,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
) -> bool {
    !path.contains(':')
        && schemas
            .values()
            .filter_map(|s| s.location.as_deref())
            .any(|loc| path.starts_with(loc) && path.ends_with(".md"))
}

/// The `<type>:<slug>` identity for a committed record path (`decisions/x.md` → its
/// persisted type's `adr:x`), or `None` when no persisted schema owns it.
fn identity_of(
    path: &str,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
) -> Option<String> {
    for (ty, schema) in schemas {
        let Some(location) = schema.location.as_deref() else {
            continue;
        };
        if let Some(rest) = path.strip_prefix(location)
            && let Some(slug) = rest.strip_suffix(".md")
        {
            return Some(format!("{ty}:{slug}"));
        }
    }
    None
}

/// The on-disk committed `.md` files (under any persisted `location:`) that carry **no**
/// recorded hash — the untracked rename candidates, each paired with its raw-byte hash
/// (`reconciliation.md` → Rename detection: an untracked path with a content-hash match
/// is a suspected `git mv`).
fn untracked_committed(
    recorded_at_entry: &std::collections::BTreeSet<String>,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    repo_root: &Path,
) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for schema in schemas.values() {
        let Some(location) = schema.location.as_deref() else {
            continue;
        };
        let dir = repo_root.join(location);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for path in entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        {
            let Some(slug) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let key = format!("{location}{slug}.md");
            if recorded_at_entry.contains(&key) {
                continue; // tracked at entry, not an untracked candidate.
            }
            if let Ok(bytes) = std::fs::read(&path) {
                out.push((key, hash_bytes(&bytes)));
            }
        }
    }
    out.sort();
    out
}

/// **Rename detection** — the separate classifier for a tracked managed-doc path
/// gone *missing* on disk (`reconciliation.md` → Rename detection; `storage.md` →
/// Identity: a path change is an identity change). Runs at the same trigger points
/// as [`reconcile_committed`], but only when the recorded path is **absent** — the
/// state machine there sees the old path as missing and a moved file as a fresh
/// untracked file, so neither catches a `git mv`.
///
/// Given the missing tracked `path`, its `<type>:<slug>` identity `from`, its
/// `recorded_hash`, and the on-disk `untracked` candidates (`(path, raw-byte hash)`
/// pairs the caller collected), it emits exactly one blocking `reconciliation.rename`
/// finding routed for human-side revert:
///
/// - **Strong signal** — some untracked path carries the **same** `recorded_hash`: a
///   suspected `git mv`. The finding names both paths and routes to
///   `git mv <suspect> <tracked>` (revert the move). The first hash-matching
///   candidate in `untracked` order is named.
/// - **Weak signal** — no untracked path matches: the file is simply gone. The
///   finding names the missing path and routes to **restore** it (or, post-MVP,
///   confirm the deletion via `jigc doc delete`).
///
/// **No auto-rewrite.** A path rename is an identity change; the MVP blocks and routes
/// to revert, and **never** rewrites referrer refs or mutates the edge index
/// (`reconciliation.md` → No silent rename). This function is pure of I/O and of any
/// edge/referrer mutation by construction — it reads its inputs and returns findings.
pub fn detect_rename(
    path: &str,
    from: &str,
    recorded_hash: &str,
    untracked: &[(&str, String)],
) -> Vec<Finding> {
    match untracked.iter().find(|(_, hash)| hash == recorded_hash) {
        Some((suspect, _)) => vec![rename_strong_finding(path, from, suspect)],
        None => vec![rename_weak_finding(path, from)],
    }
}

/// The blocking **strong-signal** rename finding (`reconciliation.md` → Rename
/// detection → strong signal): the missing tracked doc and the content-matching
/// suspect, routed to revert the suspected `git mv`. Referrer refs are untouched —
/// the route hands the identity change back to the human.
fn rename_strong_finding(path: &str, from: &str, suspect: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "reconciliation.rename",
        format!(
            "tracked managed doc {from} ({path}) is missing; {suspect} has the same content hash — likely renamed via `git mv`"
        ),
        Some(Location::addressed(path, 1, 1)),
        Some(format!(
            "revert the move: `git mv {suspect} {path}` (post-MVP: `jigc doc rename` will re-key file-state and rewrite referrer refs)"
        )),
    )
}

/// The blocking **weak-signal** rename finding (`reconciliation.md` → Rename
/// detection → weak signal): the tracked doc is simply gone, no content-matching
/// suspect, routed to restore the file.
fn rename_weak_finding(path: &str, from: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "reconciliation.rename",
        format!("tracked managed doc {from} ({path}) is missing"),
        Some(Location::addressed(path, 1, 1)),
        Some(format!(
            "restore {path} (post-MVP: `jigc doc delete {from}` to confirm deletion)"
        )),
    )
}

/// The advisory **absorb** finding (`reconciliation.md` → OOB edit → absorb: "external
/// edit absorbed: `<doc>`"). Informational, no route — a clean external edit is honored,
/// not a problem to repair; the absorb already re-hashed + updated the edge index.
fn absorb_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "reconciliation.absorb",
        format!("external edit absorbed: `{path}`"),
        Some(Location::addressed(path, 1, 1)),
        None,
    )
}

/// The blocking **conformance-block** finding (`reconciliation.md` → OOB edit →
/// conformance-block: "a precise conformance error — file, line, expected shape"). The
/// underlying parse/schema `cause` (re-located onto the file) names exactly what is
/// wrong; the MVP never auto-repairs (`reconciliation.md` → Auto-repair scope).
fn conformance_block_finding(path: &str, cause: Option<Finding>) -> Finding {
    let (detail, line) = match &cause {
        Some(f) => (
            f.message.clone(),
            f.location.as_ref().map(|l| l.line).unwrap_or(1),
        ),
        None => ("the edit is not schema-conformant".to_string(), 1),
    };
    Finding::graded(
        Severity::Blocking,
        "reconciliation.conformance-block",
        format!("nonconformant edit on `{path}`: {detail}"),
        Some(Location::addressed(path, line, 1)),
        Some("fix the file to restore conformance, or revert the edit".to_string()),
    )
}

/// The blocking **conflict-block** finding (`reconciliation.md` → Conflict — block at
/// file level): both the on-disk file and the task's working area moved. File
/// granularity, explicit-discard route, never a silent merge (three-way merge is
/// deferred).
fn conflict_block_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "reconciliation.conflict-block",
        format!(
            "conflict on `{path}`: an external edit and this task's staged writes both changed it"
        ),
        Some(Location::addressed(path, 1, 1)),
        Some(format!(
            "discard the task's writes (`jigc task discard-write {path}`) or revert the file on disk"
        )),
    )
}

/// The informational baseline-adopt finding (`reconciliation.md` → Baseline
/// adoption: "baseline adopted: `<doc>`"). Advisory, no route — first encounter
/// is the normal case, not a problem to repair.
fn baseline_adopt_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "file-state.baseline-adopt",
        format!("baseline adopted: `{path}`"),
        Some(Location::addressed(path, 1, 1)),
        None,
    )
}

/// The drift block: the on-disk content no longer matches the recorded hash. A
/// blocking `file-state.hash-matches` finding carrying a `reconcile <target>`
/// route the engine never executes (`validation.md` → Findings: the `reconcile`
/// route; severity inventory: `hash-matches` default blocking).
fn drift_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "file-state.hash-matches",
        format!("on-disk content of `{path}` differs from the recorded state"),
        Some(Location::addressed(path, 1, 1)),
        Some(format!("reconcile {path}")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::{Edge, EdgeIndex};
    use crate::schema::Schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }

    /// A committed ADR `B` (superseding nothing) — the recorded baseline before any
    /// out-of-band edit.
    const ADR_B_BASE: &str = "\
---
status: accepted
date: 2026-05-30
---

# Distributed session cache

## Context
A single node is a single point of failure.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";

    /// The same ADR `B`, edited out-of-band to *add* a `supersedes` ref — a clean,
    /// schema-conformant edit (the absorb case).
    const ADR_B_EDITED_SUPERSEDES: &str = "\
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

    /// ADR `B`, edited out-of-band to *malform* the `date` field value — a
    /// non-conformant edit (the conformance-block case).
    const ADR_B_EDITED_BAD_DATE: &str = "\
---
status: accepted
date: 2026/13/01
---

# Distributed session cache

## Context
A single node is a single point of failure.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";

    const ADR_B_PATH: &str = "decisions/distributed-cache.md";
    const ADR_B_FROM: &str = "adr:distributed-cache";

    /// The DRIFTED+UNTOUCHED clean-reparse branch: a committed ADR with a recorded
    /// baseline hash, edited on disk to add a `supersedes`, reconciles to **absorb** —
    /// the recorded hash advances to the new content's hash, the committed edge index
    /// gains the new forward edge, and the finding is an **advisory**
    /// `reconciliation.absorb` carrying the "external edit absorbed" message.
    #[test]
    fn oob_clean_edit_absorbs_and_updates_index() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        // Baseline: the committed bytes before the OOB edit.
        record.record(ADR_B_PATH, hash_bytes(ADR_B_BASE.as_bytes()));

        // The committed index has no edge for B yet (it superseded nothing).
        let mut index = EdgeIndex::default();

        let edited = ADR_B_EDITED_SUPERSEDES.as_bytes();
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            edited,
            /* task_touched */ false,
        );

        // Exactly one advisory absorb finding carrying the message.
        assert_eq!(findings.len(), 1, "absorb emits exactly one finding");
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.absorb");
        assert_eq!(f.severity, Severity::Advisory, "absorb is informational");
        assert!(
            f.message.contains("external edit absorbed") && f.message.contains(ADR_B_PATH),
            "absorb names the absorbed doc: {f:?}"
        );

        // Re-hash: the recorded baseline advanced to the new content's hash.
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(hash_bytes(edited).as_str()),
            "absorb re-hashes the recorded baseline to the edited content"
        );

        // The committed edge index gained the new forward edge (incremental update).
        assert_eq!(
            index.edges,
            vec![Edge {
                from: ADR_B_FROM.to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }],
            "absorb incrementally adds the edited doc's new forward edge"
        );
    }

    /// The DRIFTED+UNTOUCHED parse/schema-fail branch: an edit malforming the `date`
    /// value **conformance-blocks** — a blocking `reconciliation.conformance-block`
    /// finding naming the file (and an expected-shape hint), the recorded hash is
    /// **not** advanced (no re-baseline on a block), and the edge index is **not**
    /// touched (no auto-repair).
    #[test]
    fn oob_nonconformant_edit_blocks() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let baseline = hash_bytes(ADR_B_BASE.as_bytes());
        record.record(ADR_B_PATH, baseline.clone());

        let mut index = EdgeIndex::default();
        let edges_before = index.edges.clone();

        let edited = ADR_B_EDITED_BAD_DATE.as_bytes();
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            edited,
            /* task_touched */ false,
        );

        assert!(
            findings
                .iter()
                .any(|f| f.code == "reconciliation.conformance-block"
                    && f.severity == Severity::Blocking),
            "a nonconformant edit conformance-blocks: {findings:?}"
        );
        let block = findings
            .iter()
            .find(|f| f.code == "reconciliation.conformance-block")
            .expect("a conformance-block finding");
        assert!(
            block.message.contains(ADR_B_PATH),
            "the block names the offending file: {block:?}"
        );
        assert!(
            block.route.is_some(),
            "the conformance-block carries a route"
        );

        // The recorded hash is NOT advanced — a block is not a re-baseline site.
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(baseline.as_str()),
            "conformance-block does not advance the recorded hash (no auto-repair)"
        );
        // The edge index is untouched.
        assert_eq!(
            index.edges, edges_before,
            "conformance-block does not update the edge index"
        );
    }

    /// The DRIFTED+TOUCHED branch: the same committed doc both drifted on disk **and**
    /// staged by the active task **conflict-blocks** — a blocking
    /// `reconciliation.conflict-block` finding carrying the explicit-discard route, no
    /// silent merge, the recorded hash unadvanced and the edge index untouched.
    #[test]
    fn oob_conflict_blocks() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let baseline = hash_bytes(ADR_B_BASE.as_bytes());
        record.record(ADR_B_PATH, baseline.clone());

        let mut index = EdgeIndex::default();

        // The on-disk content drifted (a clean edit, even) — but the task also touched it.
        let edited = ADR_B_EDITED_SUPERSEDES.as_bytes();
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            edited,
            /* task_touched */ true,
        );

        assert_eq!(findings.len(), 1, "conflict emits exactly one finding");
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.conflict-block");
        assert_eq!(f.severity, Severity::Blocking);
        let route = f
            .route
            .as_deref()
            .expect("conflict carries a discard route");
        assert!(
            route.contains("discard") || route.contains("revert"),
            "the conflict route names the explicit-discard / revert paths: {route:?}"
        );

        // No silent merge: neither the recorded hash nor the edge index moved.
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(baseline.as_str()),
            "conflict-block does not advance the recorded hash (no silent merge)"
        );
        assert!(
            index.edges.is_empty(),
            "conflict-block does not update the edge index"
        );
    }

    /// **Strong signal** — a tracked managed-doc path is missing **and** an untracked
    /// path carries the **same recorded content hash**: a suspected `git mv`. The
    /// detector emits exactly one blocking `reconciliation.rename` finding naming both
    /// the missing tracked path and the suspect path, routed to a `git mv … revert`,
    /// and rewrites **no** referrer refs / edges (`reconciliation.md` → Rename
    /// detection → strong signal).
    #[test]
    fn rename_strong_signal_routes_to_git_mv_revert() {
        const TRACKED: &str = "decisions/rate-limit.md";
        const MOVED: &str = "decisions/gateway-rate-limit.md";
        const FROM: &str = "adr:rate-limit";

        // The recorded baseline hash for the (now-missing) tracked doc.
        let recorded = hash_bytes(ADR_B_BASE.as_bytes());

        // The untracked candidates on disk: the moved file carries the *same* content,
        // so its raw-byte hash matches the recorded baseline (a suspected `git mv`).
        let untracked: Vec<(&str, String)> = vec![(MOVED, hash_bytes(ADR_B_BASE.as_bytes()))];

        let findings = detect_rename(TRACKED, FROM, &recorded, &untracked);

        assert_eq!(findings.len(), 1, "strong signal emits exactly one finding");
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.rename");
        assert_eq!(f.severity, Severity::Blocking);
        assert!(
            f.message.contains(TRACKED) && f.message.contains(MOVED),
            "the strong-signal finding names both the missing tracked path and the suspect path: {f:?}"
        );
        let route = f
            .route
            .as_deref()
            .expect("strong signal carries a revert route");
        assert!(
            route.contains("git mv") && route.contains(MOVED) && route.contains(TRACKED),
            "the route directs a `git mv … revert` of the moved file back to the tracked path: {route:?}"
        );
    }

    /// **Weak signal** — a tracked managed-doc path is missing and **no** untracked
    /// path carries the recorded content hash: the file is simply gone (deleted). The
    /// detector emits exactly one blocking `reconciliation.rename` finding routed to
    /// **restore**, naming the missing path and rewriting **no** referrer refs / edges
    /// (`reconciliation.md` → Rename detection → weak signal).
    #[test]
    fn rename_weak_signal_routes_to_restore() {
        const TRACKED: &str = "decisions/rate-limit.md";
        const FROM: &str = "adr:rate-limit";

        let recorded = hash_bytes(ADR_B_BASE.as_bytes());

        // Untracked candidates exist, but none matches the recorded hash (different
        // content) — so there is no rename suspect, only a missing file.
        let untracked: Vec<(&str, String)> =
            vec![("decisions/unrelated.md", hash_bytes(b"some other body\n"))];

        let findings = detect_rename(TRACKED, FROM, &recorded, &untracked);

        assert_eq!(findings.len(), 1, "weak signal emits exactly one finding");
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.rename");
        assert_eq!(f.severity, Severity::Blocking);
        assert!(
            f.message.contains(TRACKED) && f.message.contains("missing"),
            "the weak-signal finding names the missing tracked path: {f:?}"
        );
        let route = f
            .route
            .as_deref()
            .expect("weak signal carries a restore route");
        assert!(
            route.contains("restore"),
            "the weak-signal route directs a restore of the missing file: {route:?}"
        );
    }

    /// The command-surface sweep [`reconcile_committed_store`] routes committed-store
    /// drift end-to-end: it walks `<repo_root>/decisions/*.md`, reconciles each against
    /// the recorded baseline, and aggregates the classifier's findings — a conformant
    /// OOB edit **absorbs** (advisory, record advances) while a nonconformant one
    /// **conformance-blocks** (record pinned). This is the contract `validate_task`
    /// relies on (`reconciliation.md` → Detection timing: the `task validate` full sweep).
    #[test]
    fn reconcile_committed_store_sweeps_and_routes_drift() {
        let schema = adr_schema();
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        // A committed store with two ADRs at their canonical paths.
        let root = TempRoot::new("sweep");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");

        // `clean`: a conformant OOB edit (a `## Decision` prose change) → absorb.
        let clean_path = "decisions/clean.md";
        std::fs::write(
            decisions.join("clean.md"),
            ADR_B_EDITED_SUPERSEDES, // differs from ADR_B_BASE (adds a supersedes)
        )
        .expect("write clean ADR");
        // `broken`: a structural nonconformance (renamed heading) → conformance-block.
        let broken_path = "decisions/broken.md";
        let broken = ADR_B_BASE.replace("## Decision", "## Decisionz");
        std::fs::write(decisions.join("broken.md"), &broken).expect("write broken ADR");

        // Record both baselines as the pre-edit content (so both DRIFT).
        let mut record = FileStateRecord::new();
        record.record(clean_path, hash_bytes(ADR_B_BASE.as_bytes()));
        record.record(broken_path, hash_bytes(ADR_B_BASE.as_bytes()));

        let mut index = EdgeIndex::default();
        // No active task touches either doc (a separate empty task dir).
        let task = TempRoot::new("sweep-task");

        let findings =
            reconcile_committed_store(&mut record, &mut index, &schemas, root.path(), task.path());

        // The clean ADR absorbed (advisory) and its baseline advanced + edge folded in.
        assert!(
            findings
                .iter()
                .any(|f| f.code == "reconciliation.absorb" && f.message.contains(clean_path)),
            "the conformant edit absorbs: {findings:?}"
        );
        assert_eq!(
            record.get(clean_path),
            Some(hash_bytes(ADR_B_EDITED_SUPERSEDES.as_bytes()).as_str()),
            "absorb advances the clean doc's recorded baseline"
        );
        assert!(
            index.edges.contains(&Edge {
                from: "adr:clean".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }),
            "absorb folds the clean doc's new forward edge into the index: {:?}",
            index.edges
        );

        // The broken ADR conformance-blocked and its baseline stayed pinned.
        let block = findings
            .iter()
            .find(|f| f.code == "reconciliation.conformance-block")
            .expect("the nonconformant edit conformance-blocks");
        assert_eq!(block.severity, Severity::Blocking);
        assert!(
            block.message.contains(broken_path),
            "the block names the offending file: {block:?}"
        );
        assert_eq!(
            record.get(broken_path),
            Some(hash_bytes(ADR_B_BASE.as_bytes()).as_str()),
            "conformance-block does not advance the recorded hash"
        );
    }

    /// [`reconcile_committed_store`] routes a recorded committed doc that has gone
    /// **missing** on disk to rename detection: a content-hash-matching untracked file
    /// is a strong-signal `git mv` suspect (`reconciliation.md` → Rename detection).
    #[test]
    fn reconcile_committed_store_routes_a_missing_recorded_doc_to_rename() {
        let schema = adr_schema();
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        let root = TempRoot::new("rename-sweep");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");

        // The tracked path is recorded but absent on disk; a content-matching file
        // exists at a new (untracked) path — a suspected `git mv`.
        let tracked = "decisions/old-name.md";
        let moved_slug = "new-name";
        std::fs::write(decisions.join(format!("{moved_slug}.md")), ADR_B_BASE)
            .expect("write moved ADR");

        let mut record = FileStateRecord::new();
        record.record(tracked, hash_bytes(ADR_B_BASE.as_bytes()));

        let mut index = EdgeIndex::default();
        let task = TempRoot::new("rename-task");

        let findings =
            reconcile_committed_store(&mut record, &mut index, &schemas, root.path(), task.path());

        let rename = findings
            .iter()
            .find(|f| f.code == "reconciliation.rename")
            .expect("the missing recorded doc routes to rename detection");
        assert_eq!(rename.severity, Severity::Blocking);
        assert!(
            rename.message.contains(tracked) && rename.message.contains("new-name"),
            "the strong-signal rename names both the missing tracked path and the suspect: {rename:?}"
        );
    }

    /// The 2026-06-12 rename amendment (`reconciliation.md` → Rename detection:
    /// "missing means absent from disk, not absent from the walk"): a baselined path
    /// under a persisted `location:` prefix but outside the non-recursive
    /// `<location>/*.md` walk — the promoted owner-artifact case,
    /// `completions/artifacts/<run>/x.md`, baselined by its finalize — is **not**
    /// missing while it exists on disk: zero `reconciliation.rename`, hash untouched.
    /// Genuinely deleted from disk, the weak-signal blocking rename fires.
    #[test]
    fn baselined_path_outside_walk_is_not_missing() {
        let mut schema = adr_schema();
        schema.location = Some("completions/".to_string());
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("completion-record".to_string(), schema);

        let root = TempRoot::new("outside-walk");
        let artifact_rel = "completions/artifacts/run-1/x.md";
        let artifact_abs = root.path().join(artifact_rel);
        std::fs::create_dir_all(artifact_abs.parent().expect("artifact has a parent"))
            .expect("mk completions/artifacts/run-1/");
        let artifact_bytes: &[u8] = b"the genuine audit transcript\n";
        std::fs::write(&artifact_abs, artifact_bytes).expect("write the owner-artifact");

        // The artifact was baselined at its finalize (git_commit_files re-hash).
        let mut record = FileStateRecord::new();
        let baseline = hash_bytes(artifact_bytes);
        record.record(artifact_rel, baseline.clone());

        let mut index = EdgeIndex::default();
        let task = TempRoot::new("outside-walk-task");

        // Present on disk but never walked → NOT missing: zero rename, hash untouched.
        let findings =
            reconcile_committed_store(&mut record, &mut index, &schemas, root.path(), task.path());
        assert!(
            findings.iter().all(|f| f.code != "reconciliation.rename"),
            "a baselined path present on disk outside the walk is not missing: {findings:?}"
        );
        assert_eq!(
            record.get(artifact_rel),
            Some(baseline.as_str()),
            "the present-outside-the-walk path stays baselined as-is (hash untouched)"
        );

        // Genuinely deleted from disk → the weak-signal blocking rename fires.
        std::fs::remove_file(&artifact_abs).expect("delete the owner-artifact");
        let findings =
            reconcile_committed_store(&mut record, &mut index, &schemas, root.path(), task.path());
        let rename = findings
            .iter()
            .find(|f| f.code == "reconciliation.rename")
            .expect("a genuinely deleted baselined path still routes to rename detection");
        assert_eq!(rename.severity, Severity::Blocking);
        assert!(
            rename.message.contains(artifact_rel) && rename.message.contains("missing"),
            "the weak-signal rename names the missing artifact path: {rename:?}"
        );
        assert!(
            rename
                .route
                .as_deref()
                .is_some_and(|r| r.contains("restore")),
            "no content-matching suspect exists, so the weak signal routes to restore: {rename:?}"
        );
    }

    /// The read-only file↔CLI-state **store twin** ([`detect_committed_store`])
    /// over a committed store carrying a baselined-and-drifted doc, an
    /// un-baselined doc, and an in-sync doc (`validation.md` → Completing the
    /// envelope → read-only file↔CLI-state at store scope):
    ///
    /// - the drifted doc → exactly one `file-state.hash-matches` finding whose
    ///   route is the **store-scope variant** (NOT `reconcile <path>`);
    /// - the un-baselined committed doc → exactly one distinct **advisory**
    ///   un-baselined finding (not drift, not silent-clean);
    /// - the in-sync doc → no finding;
    /// - the passed `record` is **byte-identical before and after** (the twin
    ///   takes it by `&` and opens no record write — mutation-free).
    #[test]
    fn detect_committed_store_reports_drift_and_unbaselined_without_absorb() {
        let schema = adr_schema();
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        let root = TempRoot::new("detect-twin");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");

        // `drifted`: recorded baseline differs from on-disk bytes → hash-matches.
        let drifted_path = "decisions/drifted.md";
        std::fs::write(decisions.join("drifted.md"), ADR_B_EDITED_SUPERSEDES)
            .expect("write drifted ADR");
        // `synced`: on-disk bytes equal the recorded baseline → no finding.
        let synced_path = "decisions/synced.md";
        std::fs::write(decisions.join("synced.md"), ADR_B_BASE).expect("write synced ADR");
        // `fresh`: a committed doc with NO recorded hash → un-baselined advisory.
        let fresh_path = "decisions/fresh.md";
        std::fs::write(decisions.join("fresh.md"), ADR_B_BASE).expect("write fresh ADR");

        let mut record = FileStateRecord::new();
        record.record(drifted_path, hash_bytes(ADR_B_BASE.as_bytes())); // pre-edit baseline
        record.record(synced_path, hash_bytes(ADR_B_BASE.as_bytes())); // matches on-disk
        // (no record for fresh_path)

        // Clone the record to assert byte-for-byte mutation-freedom after the call.
        let record_before = record.clone();

        let findings = detect_committed_store(&record, &schemas, root.path());

        // (i) the drifted doc → exactly one hash-matches finding, store-scope route.
        let drift: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.code == "file-state.hash-matches")
            .collect();
        assert_eq!(
            drift.len(),
            1,
            "exactly one hash-matches finding (the drifted doc): {findings:?}"
        );
        let drift = drift[0];
        assert_eq!(drift.severity, Severity::Blocking);
        assert!(
            drift.message.contains(drifted_path),
            "the drift finding names the drifted doc: {drift:?}"
        );
        let route = drift
            .route
            .as_deref()
            .expect("the store-scope drift carries a route");
        assert!(
            route != format!("reconcile {drifted_path}") && !route.starts_with("reconcile"),
            "the store-scope route is NOT the task-scope `reconcile <path>` variant: {route:?}"
        );

        // (iii) the un-baselined doc → exactly one distinct advisory finding.
        let unbaselined: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.message.contains(fresh_path))
            .collect();
        assert_eq!(
            unbaselined.len(),
            1,
            "exactly one finding for the un-baselined doc: {findings:?}"
        );
        let unbaselined = unbaselined[0];
        assert_eq!(
            unbaselined.severity,
            Severity::Advisory,
            "un-baselined is informational, neither drift nor a block: {unbaselined:?}"
        );
        assert_ne!(
            unbaselined.code, "file-state.hash-matches",
            "un-baselined is a DISTINCT outcome from drift, not silent-clean"
        );

        // (iv) the in-sync doc → no finding.
        assert!(
            findings.iter().all(|f| !f.message.contains(synced_path)),
            "the in-sync doc emits no finding: {findings:?}"
        );

        // (ii) mutation-free: the record is byte-identical before and after.
        assert_eq!(
            record, record_before,
            "detect_committed_store must not mutate the record (no absorb, no re-baseline)"
        );
    }

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-file-state-{tag}-{}-{:?}",
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

    /// The done-criterion: (a) the hash of a fixed *raw-byte* input equals a
    /// pinned `blake3` hex (golden), proving raw-byte (non-canonicalized) hashing
    /// over a non-ASCII, multi-byte input; and (b) save→load of a multi-entry
    /// record round-trips byte-stably (path-sorted, identical bytes).
    #[test]
    fn file_state_hash_is_raw_byte_blake3_and_round_trips() {
        // (a) Raw-byte blake3 golden. The input carries a UTF-8 check mark (✓)
        // and an é plus a trailing newline — bytes that a canonicalizer would
        // touch; the drift hash must not, so the digest is pinned to the raw
        // bytes verbatim.
        let input: &[u8] = b"jigc file-state drift hash \xe2\x9c\x93 caf\xc3\xa9\n";
        assert_eq!(
            hash_bytes(input),
            "4c4d3fd9e60ea710965b8f8c2b6bbc63974af6f803baf098cd10d84e55c5aceb",
            "raw-byte blake3 hex of the fixed input must match the pinned golden",
        );

        // (b) Multi-entry record, recorded out of path order, saved and loaded.
        let mut record = FileStateRecord::new();
        record.record("decisions/rate-limit.md", hash_bytes(b"adr body\n"));
        record.record("specs/gateway.md", hash_bytes(b"spec body\n"));
        record.record(
            "decisions/auth.md",
            hash_bytes(b"another adr\xc3\xa9 body\n"),
        );

        let root = TempRoot::new("roundtrip");
        record.save(root.path()).expect("save the record");

        // The serialized bytes are deterministic: path-sorted, pretty, one
        // trailing newline. Golden-pin the exact on-disk byte form.
        let on_disk =
            std::fs::read_to_string(FileStateRecord::path_in(root.path())).expect("record written");
        insta::assert_snapshot!(on_disk, @r#"
        {
          "hashes": {
            "decisions/auth.md": "f1217111b9f36d9883a5924a7c7cbc216bd3062fb25282d31a0e47654976871d",
            "decisions/rate-limit.md": "874feccab2c6ac7ce7e7c0fe96442ad9144871796370127452bd7b21bfc78ee3",
            "specs/gateway.md": "b2eec3f1ab19c18fe2324d109745dd543f0e7d68a772f9b0162cf71e34303ae2"
          }
        }
        "#);

        // save → load round-trips the in-memory record exactly.
        let loaded = FileStateRecord::load(root.path()).expect("load the record");
        assert_eq!(loaded, record, "save → load must round-trip byte-stably");
    }

    /// A missing `file-state.json` is the first-encounter case: an empty record,
    /// never an error (`reconciliation.md` → Absent-hash is not drift).
    #[test]
    fn load_missing_record_is_empty_not_error() {
        let root = TempRoot::new("missing");
        let loaded = FileStateRecord::load(root.path()).expect("missing file loads empty");
        assert_eq!(loaded, FileStateRecord::new());
    }

    /// The done-criterion for the `file-state` probe across the three MVP states:
    ///
    /// 1. **Fresh file (`UNKNOWN`)** → exactly one informational
    ///    `file-state.baseline-adopt` finding (advisory, no route) **and** the
    ///    record advances: the file's hash is now recorded.
    /// 2. **Unchanged second run** → zero findings (the file is `IN_SYNC`).
    /// 3. **Mutated file** → exactly one blocking `file-state.hash-matches`
    ///    finding carrying a non-`None` `reconcile` route, and the recorded hash
    ///    is **not** advanced (re-baselining is not a drift-site).
    ///
    /// All findings are the one [`Finding`] envelope.
    #[test]
    fn file_state_probe_baselines_and_detects_drift() {
        const PATH: &str = "decisions/rate-limit.md";
        let original: &[u8] = b"## Decision\n\nadopt a token bucket\n";
        let mutated: &[u8] = b"## Decision\n\nadopt a leaky bucket\n";

        let mut record = FileStateRecord::new();

        // (1) Fresh file: baseline-adopt + the record advances.
        let findings = file_state(&mut record, &[(PATH, original)]);
        assert_eq!(findings.len(), 1, "first encounter emits one finding");
        let adopt = &findings[0];
        assert_eq!(adopt.code, "file-state.baseline-adopt");
        assert_eq!(adopt.severity, Severity::Advisory);
        assert_eq!(adopt.route, None, "baseline adoption is not a repair");
        assert_eq!(
            record.get(PATH),
            Some(hash_bytes(original).as_str()),
            "baseline-adopt records the current hash (the record advances)",
        );

        // (2) Unchanged second run: zero findings, no spurious re-adoption.
        let findings = file_state(&mut record, &[(PATH, original)]);
        assert!(
            findings.is_empty(),
            "a matching hash is IN_SYNC — no finding, got {findings:?}",
        );

        // (3) Mutated file: exactly one blocking hash-matches drift finding with a
        // non-None reconcile route; the recorded hash stays pinned (not advanced).
        let findings = file_state(&mut record, &[(PATH, mutated)]);
        assert_eq!(findings.len(), 1, "drift emits exactly one finding");
        let drift = &findings[0];
        assert_eq!(drift.code, "file-state.hash-matches");
        assert_eq!(drift.severity, Severity::Blocking);
        let route = drift
            .route
            .as_deref()
            .expect("drift carries a reconcile route");
        assert!(
            route.starts_with("reconcile"),
            "the drift route is a reconcile direction, got {route:?}",
        );
        assert_eq!(
            record.get(PATH),
            Some(hash_bytes(original).as_str()),
            "drift does not advance the recorded hash (re-baselining is not a drift-site)",
        );
    }
}
