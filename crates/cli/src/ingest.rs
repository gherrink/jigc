//! `jigc ingest` — the existing-project ingestion scan (M9 Increment 3, T1).
//!
//! **Read-only** discover → classify → triage report. Locates the repo root +
//! project layer, loads the persisted schemas + the committed edge index
//! ([`index::load_committed`]) + the file-state record ([`FileStateRecord::load`]),
//! runs the engine's [`engine::ingest::discover_candidates`] then
//! [`engine::ingest::classify`] per candidate (in sorted candidate order), and
//! assembles the triage report (`file × best-match type × verdict`) the dispatch
//! renders through the global `--format`. No adopt yet — it rewrites nothing.
//!
//! ## Why the routed finding is net-new command-layer report-assembly
//!
//! The engine [`classify`](engine::ingest::classify) reduces to a **bare**
//! [`Verdict::NeedsReconcile`] — it discards *why* a candidate needs reconciling.
//! The triage surface (`project-setup.md` → Flow 2, bullet 4) needs each
//! `needs-reconcile` row to carry a routed [`Finding`] (blocking severity + located
//! message + route), exactly as an OOB conflict routes today. So this layer
//! **re-derives** the finding from the same substrate the verdict reduced over:
//!
//! - **near-miss** (under a schema's `location:` dir but fails parse/conformance) →
//!   re-run the home schema's [`parse_sections`] / [`schema_conformance`] and surface
//!   the first blocking finding, retargeted with a `reconcile <path>` route.
//! - **wrong location** (conformant against a schema but outside its `location:`) →
//!   a synthesized blocking finding routing the human to relocate it (jigc never
//!   auto-moves — detect-and-route).

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

use engine::file_state::FileStateRecord;
use engine::finding::{Finding, Location, Severity};
use engine::index;
use engine::ingest::{Verdict, classify, discover_candidates};
use engine::parse::parse_sections;
use engine::schema::{Schema, load_schema};
use engine::validate::schema_conformance;

use crate::pack::make_pack;
use crate::task::git_head;
use engine::packsource::PackResourceKind;

/// One triage row: a discovered candidate's path, its best-match doc-type (the home
/// schema's type, or the conformant schema's type for a wrong-location row; `None`
/// when it matches nothing), the rendered verdict label, and — for `needs-reconcile`
/// — the routed [`Finding`].
#[derive(Clone, Debug, serde::Serialize)]
pub struct TriageRow {
    /// The forward-slash repo-relative candidate path.
    pub file: String,
    /// The best-match doc-type, when one is known; `None` for `unmanaged`.
    pub best_match: Option<String>,
    /// The verdict label (`adoptable` / `needs-reconcile` / `unmanaged`).
    pub verdict: &'static str,
    /// The routed finding for a `needs-reconcile` row; `None` otherwise.
    pub finding: Option<Finding>,
    /// Whether this `adoptable` candidate was actually adopted on this run —
    /// schema-re-gated, indexed, and baselined (register-only). Always `false` for a
    /// `needs-reconcile` / `unmanaged` row; the render marks an adopted row distinctly.
    pub adopted: bool,
}

/// The triage report — the discovered candidates classified, in sorted candidate
/// order (the deterministic report order [`discover_candidates`] yields).
#[derive(Clone, Debug, serde::Serialize)]
pub struct IngestReport {
    pub rows: Vec<TriageRow>,
}

/// Run the ingestion scan against `cwd`: locate the repo + project layer, load the
/// schemas + committed index + file-state record (the adopt substrate), discover +
/// classify every candidate, **adopt every `adoptable` candidate** (schema-re-gate →
/// index → baseline, register-only), then persist the advanced index + record and
/// assemble the triage report. Adopt never moves or rewrites a candidate file.
pub(crate) fn run(cwd: &Path) -> Result<IngestReport> {
    let repo_root = require_project_layer(cwd)?;
    let pack = make_pack();
    let schemas = load_schemas(pack.as_ref())?;

    // The adopt substrate: the committed edge index keyed to the current HEAD and the
    // file-state record. Adopt advances these in memory (`adopt` persists nothing
    // itself), then `run` saves the result — register-only, no candidate file touched.
    let jigc_root = repo_root.join(".jigc");
    let head = git_head(&repo_root)?;
    let schema_map: std::collections::BTreeMap<String, Schema> =
        schemas.iter().map(|s| (s.ty.clone(), s.clone())).collect();
    let mut index = index::load_committed(&repo_root, &jigc_root, &schema_map, &head);
    let mut record = FileStateRecord::load(&jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;

    let candidates = discover_candidates(&repo_root, &schemas);
    let mut rows = Vec::with_capacity(candidates.len());
    let mut adopted_any = false;
    for rel_path in candidates {
        let bytes = read_candidate_bytes(&repo_root, &rel_path)?;
        let source = String::from_utf8_lossy(&bytes).into_owned();
        let mut row = classify_row(&rel_path, &source, &schemas);

        // Adopt every `adoptable` candidate (schema-gated, register-only). The verdict
        // already named the conformant-at-location type; `adopt` re-gates over the same
        // substrate and refuses anything non-conformant — so a row is only marked
        // `adopted` when the re-gate *also* passes (nothing adopted without a schema
        // check). A refusal leaves the row un-adopted, never erroring the whole scan.
        if row.verdict == "adoptable"
            && let Some(ty) = row.best_match.as_deref()
            && let Some(schema) = schema_map.get(ty)
            && engine::ingest::adopt(&mut record, &mut index, schema, &rel_path, &bytes).is_ok()
        {
            row.adopted = true;
            adopted_any = true;
        }

        rows.push(row);
    }

    // Persist the advanced index + record once, after the whole scan, iff anything was
    // adopted — adopt is register-only (it touches no candidate file); the caller saves
    // the surfaces it mutated.
    if adopted_any {
        index
            .save(&jigc_root)
            .with_context(|| format!("could not save the edge index under {jigc_root:?}"))?;
        record
            .save(&jigc_root)
            .with_context(|| format!("could not save the file-state record under {jigc_root:?}"))?;
    }

    Ok(IngestReport { rows })
}

/// Classify one candidate into a [`TriageRow`], re-deriving the routed finding for a
/// `needs-reconcile` verdict from the same `parse_sections` / `schema_conformance`
/// substrate the engine reduced over (the engine discards it; the triage surface
/// needs it).
fn classify_row(rel_path: &str, source: &str, schemas: &[Schema]) -> TriageRow {
    match classify(rel_path, source, schemas) {
        Verdict::Adoptable { ty } => TriageRow {
            file: rel_path.to_string(),
            best_match: Some(ty),
            verdict: "adoptable",
            finding: None,
            // The caller adopts the row (re-gate → index → baseline) and flips this.
            adopted: false,
        },
        Verdict::Unmanaged => TriageRow {
            file: rel_path.to_string(),
            best_match: None,
            verdict: "unmanaged",
            finding: None,
            adopted: false,
        },
        Verdict::NeedsReconcile => {
            // Split the two needs-reconcile shapes the same way the engine's verdict
            // reduction does: a near-miss sits under a schema's location dir; a
            // wrong-location doc conforms to a schema it does not live under.
            if let Some(home) = schemas.iter().find(|s| under_location(rel_path, s)) {
                let finding = near_miss_finding(rel_path, source, home);
                TriageRow {
                    file: rel_path.to_string(),
                    best_match: Some(home.ty.clone()),
                    verdict: "needs-reconcile",
                    finding: Some(finding),
                    adopted: false,
                }
            } else {
                let conformant = schemas.iter().find(|s| conforms(s, source));
                let ty = conformant.map(|s| s.ty.clone());
                let finding = wrong_location_finding(rel_path, conformant);
                TriageRow {
                    file: rel_path.to_string(),
                    best_match: ty,
                    verdict: "needs-reconcile",
                    finding: Some(finding),
                    adopted: false,
                }
            }
        }
    }
}

/// Re-derive the routed finding for a **near-miss** (a doc under a schema's
/// `location:` dir that fails parse/conformance). Surface the first blocking finding
/// the home schema's parse/conformance produces — its located message names the exact
/// failure — and route it to reconcile the candidate against its claimed type.
fn near_miss_finding(rel_path: &str, source: &str, home: &Schema) -> Finding {
    let detail = match parse_sections(home, source) {
        Err(findings) => findings.into_iter().next(),
        Ok(doc) => schema_conformance(home, source, &doc).into_iter().next(),
    };
    let route = format!("reconcile {rel_path} against the `{}` schema", home.ty);
    match detail {
        Some(finding) => Finding::graded(
            Severity::Blocking,
            finding.code,
            finding.message,
            finding
                .location
                .map(|loc| Location::addressed(rel_path, loc.line, loc.col))
                .or_else(|| Some(Location::addressed(rel_path, 1, 1))),
            Some(route),
        ),
        // Defensive: a needs-reconcile under a location dir always has a failing gate,
        // so this branch is not reached in practice. Surface a generic routed block
        // rather than panic, keeping the report total.
        None => Finding::graded(
            Severity::Blocking,
            "ingest.needs-reconcile",
            format!("`{rel_path}` does not conform to the `{}` schema", home.ty),
            Some(Location::addressed(rel_path, 1, 1)),
            Some(route),
        ),
    }
}

/// Re-derive the routed finding for a **wrong-location** doc: conformant against a
/// schema but sitting outside that schema's `location:` dir, so the committed-store
/// sweep would never find it. Route the human to relocate it — jigc never auto-moves.
fn wrong_location_finding(rel_path: &str, conformant: Option<&Schema>) -> Finding {
    let (ty, location) = match conformant {
        Some(schema) => (
            schema.ty.as_str(),
            schema.location.as_deref().unwrap_or("its location dir"),
        ),
        None => ("(unknown)", "its location dir"),
    };
    Finding::graded(
        Severity::Blocking,
        "ingest.wrong-location",
        format!(
            "conformant `{ty}` at `{rel_path}` sits outside `{location}` — relocate to adopt (jigc never auto-moves)"
        ),
        Some(Location::addressed(rel_path, 1, 1)),
        Some(format!(
            "move {rel_path} into {location}, then re-run `jigc ingest`"
        )),
    )
}

/// Whether `rel_path` sits directly under `schema`'s declared `location:` dir — the
/// command-layer mirror of the engine's location discriminator, used to split the two
/// `needs-reconcile` shapes for finding re-derivation. A transient (location-less)
/// schema is never a home.
fn under_location(rel_path: &str, schema: &Schema) -> bool {
    let Some(location) = schema.location.as_deref() else {
        return false;
    };
    let dir = location.trim_end_matches('/');
    !dir.is_empty() && rel_path.starts_with(&format!("{dir}/"))
}

/// Whether `source` parses *and* conforms cleanly against `schema` — the binary
/// parse-or-not gate, mirrored from the engine so the wrong-location row can name the
/// conformant type.
fn conforms(schema: &Schema, source: &str) -> bool {
    match parse_sections(schema, source) {
        Ok(doc) => schema_conformance(schema, source, &doc).is_empty(),
        Err(_) => false,
    }
}

/// Read a discovered candidate's raw on-disk bytes — the exact bytes `adopt` hashes
/// for the file-state baseline, and (lossy-decoded by the caller) the source the
/// classifier + finding re-derivation parse. A read failure (a file that vanished
/// between discovery and classification) surfaces as a routed error.
fn read_candidate_bytes(repo_root: &Path, rel_path: &str) -> Result<Vec<u8>> {
    let path = repo_root.join(rel_path);
    std::fs::read(&path).with_context(|| format!("could not read the candidate at {path:?}"))
}

/// Load every shipped schema from the embedded pack — the persisted set the
/// classifier runs each candidate against (the engine stays domain-empty; the CLI
/// feeds the cascade in). Returned in pack-list order (the engine's discovery dedups
/// + sorts independently).
fn load_schemas(pack: &dyn engine::packsource::PackSource) -> Result<Vec<Schema>> {
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .with_context(|| format!("the `{}` schema reads back", id.as_str()))?;
        let schema =
            load_schema(&bytes).with_context(|| format!("the `{}` schema parses", id.as_str()))?;
        out.push(schema);
    }
    Ok(out)
}

/// Locate the repo root and its `.jigc/config/` project layer — the same locate
/// preamble the `jigc upgrade` seam uses. Errors with routed messages when the repo
/// or the project layer is absent.
fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }
    Ok(repo_root)
}

/// Walk up from `start` to the directory holding `.git` (the repo root).
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}
