//! The `file-state` hash record — the raw-byte drift hash and the
//! `.jigc/state/file-state.json` `path → hex-hash` map (load/save).
//!
//! This is the *store* layer only: a raw-byte [`blake3`] hash and byte-stable
//! JSON I/O over the record. No classifier, no probe, no drift logic — those
//! consume this store and land in inc-5 (`reconciliation.md` → the committed-state
//! axis; `validation.md` → the `file-state` probe).
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
