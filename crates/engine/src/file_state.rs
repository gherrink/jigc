//! The `file-state` hash record — the raw-byte drift hash, the
//! `.jigc/state/file-state.json` `path → hex-hash` map (load/save), the engine-native
//! `file-state` probe, and the full OOB reconciliation classifier.
//!
//! The store layer is a raw-byte [`blake3`] hash and byte-stable JSON I/O over the
//! record. Built atop it: [`file_state`], the inc-4 working-area/commit-only probe
//! (baseline-adopt + hash-matches); and [`reconcile_committed`], the inc-5 OOB
//! reconciliation state machine over a *committed* managed doc — absorb /
//! conformance-block / conflict-block (`reconciliation.md` → The state machine;
//! `validation.md` → the `file-state` probe).
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

/// The advisory **absorb** finding (`reconciliation.md` → OOB edit → absorb: "external
/// edit absorbed: `<doc>`"). Informational, no route — a clean external edit is honored,
/// not a problem to repair; the absorb already re-hashed + updated the edge index.
fn absorb_finding(path: &str) -> Finding {
    Finding {
        severity: Severity::Advisory,
        code: "reconciliation.absorb".to_string(),
        message: format!("external edit absorbed: `{path}`"),
        location: Some(Location::addressed(path, 1, 1)),
        route: None,
    }
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
    Finding {
        severity: Severity::Blocking,
        code: "reconciliation.conformance-block".to_string(),
        message: format!("nonconformant edit on `{path}`: {detail}"),
        location: Some(Location::addressed(path, line, 1)),
        route: Some("fix the file to restore conformance, or revert the edit".to_string()),
    }
}

/// The blocking **conflict-block** finding (`reconciliation.md` → Conflict — block at
/// file level): both the on-disk file and the task's working area moved. File
/// granularity, explicit-discard route, never a silent merge (three-way merge is
/// deferred).
fn conflict_block_finding(path: &str) -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: "reconciliation.conflict-block".to_string(),
        message: format!(
            "conflict on `{path}`: an external edit and this task's staged writes both changed it"
        ),
        location: Some(Location::addressed(path, 1, 1)),
        route: Some(format!(
            "discard the task's writes (`jigc task discard-write {path}`) or revert the file on disk"
        )),
    }
}

/// The informational baseline-adopt finding (`reconciliation.md` → Baseline
/// adoption: "baseline adopted: `<doc>`"). Advisory, no route — first encounter
/// is the normal case, not a problem to repair.
fn baseline_adopt_finding(path: &str) -> Finding {
    Finding {
        severity: Severity::Advisory,
        code: "file-state.baseline-adopt".to_string(),
        message: format!("baseline adopted: `{path}`"),
        location: Some(Location::addressed(path, 1, 1)),
        route: None,
    }
}

/// The drift block: the on-disk content no longer matches the recorded hash. A
/// blocking `file-state.hash-matches` finding carrying a `reconcile <target>`
/// route the engine never executes (`validation.md` → Findings: the `reconcile`
/// route; severity inventory: `hash-matches` default blocking).
fn drift_finding(path: &str) -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: "file-state.hash-matches".to_string(),
        message: format!("on-disk content of `{path}` differs from the recorded state"),
        location: Some(Location::addressed(path, 1, 1)),
        route: Some(format!("reconcile {path}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::{Edge, EdgeIndex};
    use crate::schema::Schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema(ADR_YAML).expect("adr.yaml loads")
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
