//! `jigc ingest` — the existing-project ingestion scan (M9 Increment 3, T1).
//!
//! Discover → classify → **adopt** → triage report. Locates the repo root +
//! project layer, loads the persisted schemas + the committed edge index
//! ([`index::load_committed`]) + the file-state record ([`FileStateRecord::load`]),
//! computes the candidate set from git ([`git_candidates`] — `git ls-files -z --cached
//! --others --exclude-standard -- '*.md'`, M40 / F8) then runs the engine's
//! [`engine::ingest::classify`] per candidate (in sorted candidate order), adopts
//! every conformant `adoptable` candidate **register-only** (records it into the
//! edge index + file-state baseline; it never moves or rewrites any candidate
//! file), and assembles the triage report (`file × best-match type × verdict`) the
//! dispatch renders through the global `--format`. Non-conformant / misplaced
//! candidates route to a human.
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
use engine::ingest::{Verdict, classify};
use engine::parse::parse_sections;
use engine::schema::Schema;
use engine::validate::{repeatable_populated, schema_conformance, surplus_sections_absent};

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
    /// The adopt-time triage annotations (M40 F4, `design/project-setup.md` → Flow 2;
    /// `design/validation.md` → Hollow and surplus adoption): the pinned shapes
    /// *"adopted — structurally empty: 0 \<items\>"* / *"adopted — N surplus trailing
    /// sections"*, computed for an `adoptable` row by re-parsing the in-hand source
    /// against the matched schema. **Fixed-advisory** — an annotation never flips a
    /// verdict, blocks adoption, or exits non-zero — and row-carried in EVERY output
    /// format (the agent path is the dominant consumer). Empty for an un-annotated row.
    pub annotations: Vec<String>,
}

/// The triage report — the discovered candidates classified, in sorted candidate
/// order (the deterministic report order [`git_candidates`] yields).
#[derive(Clone, Debug)]
pub struct IngestReport {
    pub rows: Vec<TriageRow>,
}

impl serde::Serialize for IngestReport {
    /// **The uniqueness half of the membership test, for the fourth emitting surface.** A row's
    /// `finding` projects as a [`Finding`], so its *presence* half rides that impl — but the
    /// emitted **slice** here is the report's rows, and the report holds its findings
    /// *indirectly* (one `Option<Finding>` per row), so it cannot project through
    /// [`engine::finding::Findings`]. It runs the same check over its own set, from its own
    /// `Serialize` — the guard is on the projection, not on a caller who must remember to call
    /// it (`design/command-output-contract.md` → The membership test).
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let findings: Vec<Finding> = self.rows.iter().filter_map(|r| r.finding.clone()).collect();
        engine::finding::debug_assert_keys_discriminate(&findings);
        let mut st = serializer.serialize_struct("IngestReport", 1)?;
        st.serialize_field("rows", &self.rows)?;
        st.end()
    }
}

/// Run the ingestion scan against `cwd`: locate the repo + project layer, load the
/// schemas + committed index + file-state record (the adopt substrate), discover +
/// classify every candidate, **adopt every `adoptable` candidate** (schema-re-gate →
/// index → baseline, register-only), then persist the advanced index + record and
/// assemble the triage report. Adopt never moves or rewrites a candidate file.
pub(crate) fn run(cwd: &Path) -> Result<IngestReport> {
    // The committed doc-store + `.jigc/` bind to jigc_home (the main checkout); only the
    // edge-index HEAD stamp reads the worktree HEAD (M31 Inc 2 / WF3). Outside a worktree
    // the two coincide, so the sweep is byte-identical.
    let jigc_home = require_project_layer(cwd)?;
    let worktree = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let pack = make_pack()?;
    let resolved =
        crate::start::resolve_severity_cascade(pack.as_ref(), &jigc_home.join(".jigc/config"))?;
    let schemas = load_schemas(pack.as_ref(), &resolved)?;

    // The adopt substrate: the committed edge index keyed to the current HEAD and the
    // file-state record. Adopt advances these in memory (`adopt` persists nothing
    // itself), then `run` saves the result — register-only, no candidate file touched.
    let jigc_root = jigc_home.join(".jigc");
    let head = git_head(&worktree)?;
    let schema_map: std::collections::BTreeMap<String, Schema> =
        schemas.iter().map(|s| (s.ty.clone(), s.clone())).collect();
    let mut index = index::load_committed(&jigc_home, &jigc_root, &schema_map, &head);
    let mut record = FileStateRecord::load(&jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;

    // The `repeatable-populated` exemption knob — the same resolved value the store
    // sweep reads, so the adopt-time annotation and the store advisory agree on which
    // `doctype#section` tokens are a declared zero-item steady state.
    let exempt = resolved
        .scalar("validation.schema-conformance.repeatable-populated.exempt")
        .unwrap_or("");

    let candidates = git_candidates(&jigc_home)?;
    let mut rows = Vec::with_capacity(candidates.len());
    let mut adopted_any = false;
    for rel_path in candidates {
        let bytes = read_candidate_bytes(&jigc_home, &rel_path)?;
        let source = String::from_utf8_lossy(&bytes).into_owned();
        let mut row = classify_row(&rel_path, &source, &schemas, exempt);

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

/// Discover the sorted, deduped repo-relative `.md` candidate set from **git** (M40 /
/// F8; `design/project-setup.md` → Flow-2 discovery, the M40 re-base paragraph):
/// `git ls-files -z --cached --others --exclude-standard -- '*.md'` — tracked plus
/// untracked-but-not-ignored, gitignored files excluded (the adoption trial listed
/// `node_modules` `.md`s as 91% of the logged triage output). The internals prune
/// stays on top of the git listing (`.jigc/AGENT.md` is deliberately un-gitignored,
/// so it *does* appear in `ls-files`), and the unsorted listing re-sorts + dedups
/// through a `BTreeSet` (same-repo-in → same-verdicts-out). **No no-git fallback**:
/// ingest hard-requires a git repo, so a `git` failure is a hard error — the engine's
/// filesystem walk ([`engine::ingest::discover_candidates`]) survives as engine-test
/// substrate only.
fn git_candidates(jigc_home: &Path) -> Result<Vec<String>> {
    // `-z` NUL-terminates the listing so git never C-quotes a pathname — the default
    // `core.quotepath=true` octal-escapes any non-ASCII byte in line-oriented output
    // (`"docs/r\303\251sum\303\251.md"`), and consuming that quoted line as a literal
    // path would abort the whole scan on an ordinary `café.md` (M40 F8 hardening).
    let listing = crate::task::git_capture(
        jigc_home,
        &[
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
            "*.md",
        ],
    )
    .context("could not enumerate the `.md` candidate set via `git ls-files`")?;
    let candidates: std::collections::BTreeSet<String> = listing
        .split('\0')
        .filter(|rel| !rel.is_empty())
        .filter(|rel| {
            !rel.split('/')
                .any(|component| component == ".jigc" || component == ".git")
        })
        .map(str::to_string)
        .collect();
    Ok(candidates.into_iter().collect())
}

/// Classify one candidate into a [`TriageRow`], re-deriving the routed finding for a
/// `needs-reconcile` verdict from the same `parse_sections` / `schema_conformance`
/// substrate the engine reduced over (the engine discards it; the triage surface
/// needs it). An `adoptable` row additionally re-parses against the matched schema to
/// compute its adopt-time annotations ([`adopt_annotations`]); `exempt` is the
/// resolved `…repeatable-populated.exempt` knob value.
fn classify_row(rel_path: &str, source: &str, schemas: &[Schema], exempt: &str) -> TriageRow {
    match classify(rel_path, source, schemas) {
        Verdict::Adoptable { ty } => {
            let annotations = schemas
                .iter()
                .find(|s| s.ty == ty)
                .map(|schema| adopt_annotations(schema, source, exempt))
                .unwrap_or_default();
            TriageRow {
                file: rel_path.to_string(),
                best_match: Some(ty),
                verdict: "adoptable",
                finding: None,
                // The caller adopts the row (re-gate → index → baseline) and flips this.
                adopted: false,
                annotations,
            }
        }
        Verdict::Unmanaged => TriageRow {
            file: rel_path.to_string(),
            best_match: None,
            verdict: "unmanaged",
            finding: None,
            adopted: false,
            annotations: Vec::new(),
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
                    annotations: Vec::new(),
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
                    annotations: Vec::new(),
                }
            }
        }
    }
}

/// Compute an `adoptable` candidate's **adopt-time triage annotations** (M40 F4,
/// `design/validation.md` → Hollow and surplus adoption): re-parse the in-hand source
/// against the matched schema and run the two engine advisory checks —
/// [`repeatable_populated`] (a required repeatable parsing zero items; suppressed per
/// `doctype#section` token by the resolved `exempt` knob value) and
/// [`surplus_sections_absent`] (trailing H2s beyond the schema's body sections) —
/// mapping each finding to its pinned row-annotation shape: *"adopted — structurally
/// empty: 0 \<items\>"* / *"adopted — N surplus trailing sections"*
/// (`design/project-setup.md` → Flow 2). **Fixed-advisory**: the annotation is
/// visibility only — it never flips the verdict, blocks the adopt, or exits non-zero.
fn adopt_annotations(schema: &Schema, source: &str, exempt: &str) -> Vec<String> {
    let Ok(doc) = parse_sections(schema, source) else {
        // Defensive: an `adoptable` verdict already gated parse+conformance, so this
        // branch is unreached in practice; an un-parseable doc simply has no annotation.
        return Vec::new();
    };
    let mut annotations = Vec::new();
    for finding in repeatable_populated(schema, &doc, exempt) {
        // The engine addresses the finding at the hollow section's id — the `<items>`
        // noun of the pinned shape (e.g. `0 milestones`).
        let section = finding
            .location
            .and_then(|loc| loc.address)
            .unwrap_or_default();
        annotations.push(format!("adopted — structurally empty: 0 {section}"));
    }
    for finding in surplus_sections_absent(schema, source) {
        // The engine's pinned message shape leads with the surplus count
        // (`"{N} trailing surplus section heading(s) …"` — engine-tested), so the
        // first token is the N of the pinned annotation shape.
        let count = finding.message.split_whitespace().next().unwrap_or("1");
        annotations.push(format!("adopted — {count} surplus trailing sections"));
    }
    annotations
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
        Some(finding) => {
            // The re-derived finding keeps the **code** the parse/conformance sweep raised, so
            // it must keep that code's **target form** too: `jigc validate` reports the same
            // defect in the same doc, and one defect projects one `(code, target)` key —
            // whichever verb reports it (`command-output-contract.md` → the stable finding
            // key). So the raw fragment address rides the shared `path→URI` flip against the
            // candidate's managed identity, exactly as the store sweep's `attribute_to_doc`
            // does; it is **not** re-stamped at the file path, which produced a second key for
            // one defect (M42 Inc 9 fix).
            let mut finding = Finding::graded(
                Severity::Blocking,
                finding.code,
                finding.message,
                finding
                    .location
                    .or_else(|| Some(Location::at(1, 1)))
                    .map(|loc| match loc.address {
                        Some(fragment) => Location::addressed(fragment, loc.line, loc.col),
                        None => Location::at(loc.line, loc.col),
                    }),
                Some(route),
            );
            match home_identity(rel_path, home) {
                Some(identity) => {
                    engine::finding::readdress_to_uri(std::slice::from_mut(&mut finding), &identity)
                }
                // A candidate under a `location:` dir but not **at** the canonical home (a
                // nested subdirectory the store enumerator never reaches) has no managed
                // identity to claim, so it keeps the **path form** — the declared exception
                // `file-state.*` / `ingest.*` already use.
                None => {
                    let (line, col) = finding.location.map_or((1, 1), |loc| (loc.line, loc.col));
                    finding.location = Some(Location::addressed(rel_path, line, col));
                }
            }
            finding
        }
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

/// The candidate's **managed identity** (`<type>:<slug>`) when `rel_path` sits at `schema`'s
/// **canonical home** — the same identity the store enumerator would mint for it
/// ([`engine::index::committed_instances`]), so a finding raised over the candidate keys
/// exactly as the store sweep's finding over the committed doc does.
///
/// - a **placement** doctype's literal `placement.file` → the `<type>:<type>` singleton;
/// - a **direct child** `.md` of the doctype's `location:` dir → `<type>:<file-stem>`;
/// - anything else (a *nested* `.md` under the location dir, which the store enumerator's
///   flat `read_dir` never reaches, so no committed instance would ever bear this identity)
///   → `None`: a genuinely unidentifiable candidate, which keeps the path-form target.
fn home_identity(rel_path: &str, schema: &Schema) -> Option<String> {
    let ty = &schema.ty;
    if let Some(placement) = &schema.placement {
        return (rel_path == placement.file).then(|| format!("{ty}:{ty}"));
    }
    let dir = schema.location.as_deref()?.trim_end_matches('/');
    let rest = rel_path.strip_prefix(&format!("{dir}/"))?;
    let slug = rest.strip_suffix(".md")?;
    (!slug.is_empty() && !slug.contains('/')).then(|| format!("{ty}:{slug}"))
}

/// Re-derive the routed finding for a **wrong-location** doc: conformant against a
/// schema but sitting outside that schema's `location:` dir, so the committed-store
/// sweep would never find it. Route the human to relocate it — jigc never auto-moves.
fn wrong_location_finding(rel_path: &str, conformant: Option<&Schema>) -> Finding {
    let (ty, location) = match conformant {
        Some(schema) => (
            schema.ty.as_str(),
            // A placement doctype names its literal home; else its `location:` dir.
            schema
                .placement
                .as_ref()
                .map(|p| p.file.as_str())
                .or(schema.location.as_deref())
                .unwrap_or("its location dir"),
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

/// Whether `rel_path` sits at `schema`'s canonical home — under its declared
/// `location:` dir, or exactly at its literal `placement.file` (a placement doctype's
/// single home; `design/storage.md` → Placement) — the command-layer mirror of the
/// engine's home discriminator, used to split the two `needs-reconcile` shapes for
/// finding re-derivation. A transient (neither location nor placement) schema is never
/// a home.
fn under_location(rel_path: &str, schema: &Schema) -> bool {
    if let Some(placement) = &schema.placement {
        return rel_path == placement.file;
    }
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
pub(crate) fn load_schemas(
    pack: &dyn engine::packsource::PackSource,
    resolved: &engine::cascade::Resolved,
) -> Result<Vec<Schema>> {
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .with_context(|| format!("the `{}` schema reads back", id.as_str()))?;
        let schema = crate::pack::load_pack_schema(pack, &bytes)
            .with_context(|| format!("the `{}` schema parses", id.as_str()))?;
        out.push(schema);
    }
    // Surface B: nest every persisted doctype's `location:` under the resolved
    // `docs-root` so `jigc ingest` / `jigc unmanage` discover + classify managed docs at
    // the same parent the write surfaces promote them to.
    crate::start::apply_docs_root(resolved, out.iter_mut());
    Ok(out)
}

/// Locate the repo root and its `.jigc/config/` project layer — the same locate
/// preamble the `jigc upgrade` seam uses. Errors with routed messages when the repo
/// or the project layer is absent.
pub(crate) fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    // The committed doc-store + `.jigc/` bind to jigc_home (the main checkout); a worktree
    // resolves the project's shared layer (M31 Inc 2 / WF3).
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let project_config = jigc_home.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }
    Ok(jigc_home)
}

/// Walk up from `start` to the directory holding `.git` (the repo root).
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}
