//! The dev pack's `doc-code` probe executable (T1 skeleton) — a standalone program
//! that lives **outside** the engine workspace ([module-layout.md] → Probe boundary)
//! and binds to the locked JSON wire contract ([validation.md] → The wire contract).
//!
//! It cannot depend on `engine`, so it **re-declares** the request / response /
//! snapshot / finding serde shapes as its own structs matching the locked JSON form
//! (`engine::probe`). The engine's `response_findings_deserialize_into_the_one_finding_shape`
//! test already proves probe-authored JSON ingests into the engine's one `Finding`.
//!
//! ## T1 scope — wire-conformant I/O + bare-path/file-existence resolution
//!
//! 1. Read a [`ProbeRequest`] from **stdin** (JSON).
//! 2. Read the [`EffectiveStateSnapshot`] from `effective_state.snapshot_path` (JSON).
//! 3. For each anchor, resolve its **file** — the `<path>` before any `#` — against
//!    `working_tree_root`. A **missing** file yields one blocking
//!    `doc-code.<check_id>` finding (keyed on the anchor's `address`); a **present**
//!    file (or directory) yields none. A bare `<path>` (no `#symbol`) is exactly the
//!    file-existence check ([validation.md] → The anchor grammar: a bare path degrades
//!    to a file-existence check).
//! 4. Write a valid [`ProbeResponse`] JSON to **stdout** and exit `0`.
//!
//! **No tree-sitter, no symbol resolution** — `#symbol` resolution is T2,
//! the is-a-test predicate is T3. T1 resolves only the *file* before the `#`.

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

/// The wire-contract schema version — must match the engine's `result::SCHEMA_VERSION`.
const SCHEMA_VERSION: u32 = 2;

/// The **request** envelope the engine writes to this probe's stdin — the re-declared
/// projection of `engine::probe::ProbeRequest` (the locked field order).
#[derive(Debug, Deserialize)]
struct ProbeRequest {
    #[allow(dead_code)]
    probe_id: String,
    #[allow(dead_code)]
    target: String,
    effective_state: ProbeEffectiveState,
    #[allow(dead_code)]
    #[serde(default)]
    config: serde_json::Map<String, serde_json::Value>,
    #[allow(dead_code)]
    schema_version: u32,
}

/// The `effective_state` member — the path-ref to the materialized snapshot.
#[derive(Debug, Deserialize)]
struct ProbeEffectiveState {
    snapshot_path: PathBuf,
}

/// The read-only **effective-state snapshot** the engine materialized — the re-declared
/// projection of `engine::probe::EffectiveStateSnapshot` (the pinned field order
/// `anchors` then `working_tree_root`).
#[derive(Debug, Deserialize)]
struct EffectiveStateSnapshot {
    anchors: Vec<TargetAnchor>,
    working_tree_root: PathBuf,
}

/// One `(target-address, anchor-value, check-id)` pair — the re-declared projection of
/// `engine::target_surface::TargetAnchor`.
#[derive(Debug, Deserialize)]
struct TargetAnchor {
    address: String,
    anchor_value: String,
    check_id: String,
}

/// The **response** envelope this probe writes to stdout — the re-declared projection
/// of `engine::probe::ProbeResponse` (`findings` then `schema_version`).
#[derive(Debug, Serialize)]
struct ProbeResponse {
    findings: Vec<Finding>,
    schema_version: u32,
}

/// The engine's one [`Finding`] shape, re-declared. The engine deserializes `probe` /
/// `check` with `#[serde(default)]`, but this probe emits them explicitly (split from
/// the `code`) so the ingested finding carries the `(probe, check)` handle directly.
#[derive(Debug, Serialize)]
struct Finding {
    severity: Severity,
    probe: String,
    check: String,
    code: String,
    message: String,
    location: Option<Location>,
    route: Option<String>,
}

/// The finding severity — kebab-case, matching `engine::finding::Severity`.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Severity {
    Blocking,
}

/// Where a finding points — matching `engine::finding::Location` (an `address` plus a
/// 1-based source coordinate). A dangling anchor cites the doc that carries it, at the
/// doc's first coordinate (the engine owns precise line/col; the probe has only the
/// address it was handed).
#[derive(Debug, Serialize)]
struct Location {
    address: String,
    line: usize,
    col: usize,
}

impl Finding {
    /// One blocking `doc-code.<check_id>` finding for a dangling anchor, addressed at
    /// the target the engine enumerated. `probe` / `check` split from the dotted code.
    fn dangling_file(anchor: &TargetAnchor, file: &str) -> Self {
        let code = format!("doc-code.{}", anchor.check_id);
        Self {
            severity: Severity::Blocking,
            probe: "doc-code".to_string(),
            check: anchor.check_id.clone(),
            code,
            message: format!(
                "anchor `{}` resolves to no file (`{file}` is absent from the working tree)",
                anchor.anchor_value,
            ),
            location: Some(Location {
                address: anchor.address.clone(),
                line: 1,
                col: 1,
            }),
            route: None,
        }
    }
}

/// The file portion of an anchor value — everything before the first `#` (a bare path
/// has no `#`, so it is the whole value).
fn anchor_file(anchor_value: &str) -> &str {
    anchor_value.split('#').next().unwrap_or("")
}

/// Resolve every anchor's file against the working-tree root, emitting one blocking
/// finding per missing file and none for a present one.
fn check_anchors(snapshot: &EffectiveStateSnapshot) -> Vec<Finding> {
    snapshot
        .anchors
        .iter()
        .filter_map(|anchor| {
            let file = anchor_file(&anchor.anchor_value);
            if snapshot.working_tree_root.join(file).exists() {
                None
            } else {
                Some(Finding::dangling_file(anchor, file))
            }
        })
        .collect()
}

fn main() -> ExitCode {
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        return ExitCode::FAILURE;
    }
    let request: ProbeRequest = match serde_json::from_str(&input) {
        Ok(request) => request,
        Err(_) => return ExitCode::FAILURE,
    };
    let snapshot_bytes = match std::fs::read(&request.effective_state.snapshot_path) {
        Ok(bytes) => bytes,
        Err(_) => return ExitCode::FAILURE,
    };
    let snapshot: EffectiveStateSnapshot = match serde_json::from_slice(&snapshot_bytes) {
        Ok(snapshot) => snapshot,
        Err(_) => return ExitCode::FAILURE,
    };

    let response = ProbeResponse {
        findings: check_anchors(&snapshot),
        schema_version: SCHEMA_VERSION,
    };
    match serde_json::to_string(&response) {
        Ok(json) => {
            print!("{json}");
            ExitCode::SUCCESS
        }
        Err(_) => ExitCode::FAILURE,
    }
}
