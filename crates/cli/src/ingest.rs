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
//!   the first blocking finding, routed by [`near_miss_route`] — the same runnable
//!   `jigc migrate <rel> --as <doctype>` the store sweep's sibling already hands over
//!   when the candidate is un-adopted and its doctype is migratable (M47 / N19).
//! - **wrong location** (conformant against a schema but outside its `location:`) →
//!   a synthesized blocking finding routing the human to relocate it (jigc never
//!   auto-moves — detect-and-route).

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use engine::file_state::FileStateRecord;
use engine::finding::{Finding, Location, Route, Severity};
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
/// order (the deterministic report order [`git_candidates`] yields), plus the run's
/// **non-blocking findings**.
#[derive(Clone, Debug)]
pub struct IngestReport {
    pub rows: Vec<TriageRow>,
    /// The scan's **non-blocking** findings — advisories about what the run *did*, as
    /// opposed to the per-row `finding` that says what a candidate *is*. Today's one
    /// member is `file-state.absorbed`, one per adopted candidate whose baseline this run
    /// advanced past an out-of-band edit (M52 Increment 8 / T3;
    /// `design/reconciliation.md` → The absorb surface). Empty on an ordinary scan.
    ///
    /// Findings-as-data on the success path, the `setup` precedent
    /// (`design/command-output-contract.md` → findings-as-data): a driver reads the
    /// advisory and its route as a value, never by grepping the triage prose.
    pub findings: engine::finding::Findings,
}

/// The verdict-class rollup + per-directory unmanaged breakdown that rides the
/// `--format json` projection **beside** `rows` (M44 Inc 7, T4). A machine consumer
/// reads the scan's shape without re-tallying the row array: the three verdict-class
/// counts, the adopted sub-count, and the per-directory unmanaged collapse — the same
/// keying the text arm renders ([`crate::render::ingest`]'s unmanaged-by-dir
/// aggregation), so the two agree byte-for-byte on the directory key. A pure projection
/// of `rows`; it flips no verdict and gates nothing.
#[derive(Clone, Debug, serde::Serialize)]
struct IngestSummary {
    /// Candidates conformant at their managed location.
    adoptable: usize,
    /// Adoptable candidates actually adopted this run (the schema re-gate passed).
    adopted: usize,
    /// Candidates that parse against a schema but conflict (near-miss / wrong-location).
    needs_reconcile: usize,
    /// Candidates matching no managed schema.
    unmanaged: usize,
    /// Per-directory unmanaged counts, keyed on the candidate's directory (the trailing
    /// slash form `docs/`, or `./` for a root candidate) — a [`std::collections::BTreeMap`],
    /// so the breakdown is sorted + order-invariant (same rows in → byte-identical block
    /// out, independent of row-encounter order).
    unmanaged_by_directory: std::collections::BTreeMap<String, usize>,
}

impl IngestSummary {
    /// Roll the triage rows up into the summary projection: tally each verdict class
    /// (plus the adopted sub-count) and collapse the unmanaged rows per directory with
    /// the identical key [`crate::render::ingest`]'s text arm aggregates on.
    fn of(rows: &[TriageRow]) -> Self {
        let mut summary = IngestSummary {
            adoptable: 0,
            adopted: 0,
            needs_reconcile: 0,
            unmanaged: 0,
            unmanaged_by_directory: std::collections::BTreeMap::new(),
        };
        for row in rows {
            match row.verdict {
                "adoptable" => summary.adoptable += 1,
                "needs-reconcile" => summary.needs_reconcile += 1,
                "unmanaged" => {
                    summary.unmanaged += 1;
                    let dir = match row.file.rfind('/') {
                        Some(i) => row.file[..=i].to_string(),
                        None => "./".to_string(),
                    };
                    *summary.unmanaged_by_directory.entry(dir).or_insert(0) += 1;
                }
                _ => {}
            }
            if row.adopted {
                summary.adopted += 1;
            }
        }
        summary
    }
}

impl serde::Serialize for IngestReport {
    /// **The uniqueness half of the membership test, for the fourth emitting surface.** A row's
    /// `finding` projects as a [`Finding`], so its *presence* half rides that impl — but the
    /// emitted **slice** here is the report's rows, and the report holds its findings
    /// *indirectly* (one `Option<Finding>` per row), so it cannot project through
    /// [`engine::finding::Findings`]. It runs the same check over its own set, from its own
    /// `Serialize` — the guard is on the projection, not on a caller who must remember to call
    /// it (`design/command-output-contract.md` → The membership test).
    ///
    /// The emitted struct carries three fields: the full per-row `rows` projection, the
    /// derived [`IngestSummary`] rollup (T4) — a pure projection of `rows`, no gate — and
    /// the run's own `findings` (M52 Increment 8 / T3), whose uniqueness half rides
    /// [`engine::finding::Findings`]' own `Serialize`.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let findings: Vec<Finding> = self.rows.iter().filter_map(|r| r.finding.clone()).collect();
        engine::finding::debug_assert_keys_discriminate(&findings);
        let mut st = serializer.serialize_struct("IngestReport", 3)?;
        st.serialize_field("rows", &self.rows)?;
        st.serialize_field("summary", &IngestSummary::of(&self.rows))?;
        // The run's own advisories, checked by `Findings`' own `Serialize` over its own set
        // — a separate array from the per-row findings above, because the two answer
        // different questions about different subjects and a shared key would collide them.
        st.serialize_field("findings", &self.findings)?;
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
    let worktree = discover_repo_root(cwd).ok_or_else(|| crate::locate::not_in_repo(cwd))?;
    let pack = make_pack()?;
    let project_config = jigc_home.join(".jigc/config");
    let resolved = crate::start::resolve_severity_cascade(pack.as_ref(), &project_config)?;
    let schemas = load_schemas(pack.as_ref(), &project_config)?;

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

    // The migratable doctype set — every `migrate-<doctype>` workflow the composed pack
    // ships, derived exactly as the store sweep's `unregistered-doc` route derives it
    // (`cli.rs`), so the two surfaces can only offer `jigc migrate` where it runs.
    let migratable: std::collections::BTreeSet<String> = pack
        .list(PackResourceKind::Workflows)
        .iter()
        .filter_map(|id| id.as_str().strip_prefix("migrate-").map(str::to_owned))
        .collect();

    let candidates = git_candidates(&jigc_home)?;
    let mut rows = Vec::with_capacity(candidates.len());
    let mut findings = engine::finding::Findings::from(Vec::new());
    let mut adopted_any = false;
    for rel_path in candidates {
        let bytes = read_candidate_bytes(&jigc_home, &rel_path)?;
        let source = String::from_utf8_lossy(&bytes).into_owned();
        let registered = record.get(&rel_path).is_some();
        let mut row = classify_row(
            &rel_path,
            &source,
            &schemas,
            exempt,
            &migratable,
            registered,
        );

        // Would adopting this candidate carry an out-of-band edit forward? Asked **before**
        // `adopt` re-keys the record, because afterwards the evidence is gone — the whole
        // defect this closes is that the advance was invisible (M52 Increment 8 / T3).
        // `adopt` is what decides whether the absorb happens at all (its parse + conformance
        // re-gate refuses a non-conformant edit), so the answer is only kept on a row that
        // actually adopted.
        let absorbing = engine::file_state::absorbed_drift(&record, &rel_path, &bytes);

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
            if let Some(absorbed) = absorbing {
                findings.push(absorbed);
            }
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

    Ok(IngestReport { rows, findings })
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
fn classify_row(
    rel_path: &str,
    source: &str,
    schemas: &[Schema],
    exempt: &str,
    migratable: &std::collections::BTreeSet<String>,
    registered: bool,
) -> TriageRow {
    match classify(rel_path, source, schemas) {
        Verdict::Adoptable { ty } => {
            let home = schemas.iter().find(|s| s.ty == ty);
            // **The identity gate, asked before the verdict is kept** (M51 Inc 9 / T4).
            // Conformant bytes at a managed home are only half of adoptable: the file must
            // also bear an identity a `<type>:<slug>` address reaches, or adopting it
            // records a baseline + edge-index entry no door can name. The subject is
            // exactly `home_identity`'s `None`, so the refusal and the identity every other
            // consumer mints come from one function rather than two agreeing predicates.
            if let Some(home) = home
                && home_identity(rel_path, home).is_none()
            {
                return TriageRow {
                    file: rel_path.to_string(),
                    best_match: Some(home.ty.clone()),
                    verdict: "needs-reconcile",
                    finding: Some(unaddressable_identity_finding(rel_path, home)),
                    adopted: false,
                    annotations: Vec::new(),
                };
            }
            let annotations = home
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
                let finding = near_miss_finding(rel_path, source, home, migratable, registered);
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
fn near_miss_finding(
    rel_path: &str,
    source: &str,
    home: &Schema,
    migratable: &std::collections::BTreeSet<String>,
    registered: bool,
) -> Finding {
    let detail = match parse_sections(home, source) {
        Err(findings) => findings.into_iter().next(),
        Ok(doc) => schema_conformance(home, source, &doc).into_iter().next(),
    };
    let route = near_miss_route(rel_path, &home.ty, migratable, registered);
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

/// The repair direction a **near-miss** carries (M47 inc-10 / N19). `jigc validate`'s
/// store-sweep sibling ([`crate::orphan::unregistered_route`]) has handed over the runnable
/// `jigc migrate <rel> --as <doctype>` since M40 while this surface, over the very same
/// file, offered the bare prose *"reconcile `<rel>` against the `<ty>` schema"* — two
/// surfaces, one repair, only one of them followable. It is the same argv here, and because
/// the text is a single backticked real command it is built through [`Route::mechanical`],
/// so the CLI parse fence proves it runs.
///
/// **Gated exactly like the sibling, over the whole domain this route reaches.** The sibling
/// speaks only for the *unregistered* orphan tier; this one fires for every non-conformant
/// candidate at a managed home, which includes an **already-adopted** doc (a stale
/// schema-version stamp is the M42 case) — for which adoption is not the repair at all. So
/// `jigc migrate` is offered only when the candidate is both unregistered **and** its doctype
/// ships a `migrate-<doctype>` workflow (`jigc migrate --as <doctype>` hard-rejects without
/// one). Each other arm is a human direction that says which of the two it is: a route that
/// cannot be run, or one that repairs the wrong thing, is the defect one level down.
fn near_miss_route(
    rel_path: &str,
    ty: &str,
    migratable: &std::collections::BTreeSet<String>,
    registered: bool,
) -> Route {
    if registered {
        Route::human(format!(
            "`{rel_path}` is already managed, so adoption is not the repair — reconcile it \
             against the `{ty}` schema (a corpus left on an older schema-version migrates with \
             `jigc migrate-corpus`), then re-run `jigc ingest`"
        ))
    } else if migratable.contains(ty) {
        Route::mechanical(
            [
                "jigc",
                "migrate",
                &crate::task::shell_token(rel_path),
                "--as",
                ty,
            ],
            format!(
                " — it opens the `migrate-{ty}` workflow, which rewrites the file to \
                 conformant shape and adopts it at finalize"
            ),
        )
    } else {
        Route::human(format!(
            "reconcile `{rel_path}` against the `{ty}` schema by hand, then re-run \
             `jigc ingest` — no `migrate-{ty}` workflow ships to rewrite it for you"
        ))
    }
}

/// The candidate's **managed identity** (`<type>:<slug>`) when `rel_path` sits at `schema`'s
/// **canonical home** — the same identity the store enumerator would mint for it
/// ([`engine::index::committed_instances`]), so a finding raised over the candidate keys
/// exactly as the store sweep's finding over the committed doc does.
///
/// - a **placement** doctype's literal `placement.file` → the `<type>:<type>` singleton;
/// - a **direct child** `.md` of the doctype's `location:` dir **whose stem is a
///   well-formed slug** → `<type>:<file-stem>`;
/// - anything else → `None`: a candidate at a managed home that no `<type>:<slug>` address
///   reaches, which keeps the path-form target — and, when it is otherwise *conformant*,
///   is refused adoption outright ([`unaddressable_identity_finding`]).
///
/// **The `<slug>` this mints is the only component the filesystem supplies, so it is the
/// only one checked** (M51 Inc 9 / T4). A placement doctype's identity is `<type>:<type>`,
/// both halves read off the schema; a location doctype's is `<type>:<file-stem>`, whose
/// second half is whatever a human typed into a filename. The [`engine::slug::is_slug`] leg
/// is therefore on that branch alone — the same leg
/// [`engine::validate::is_unadopted_foreign`]'s identity arm applies to a *committed*
/// instance (M50 Inc 2), asked here at the *adoption* door so the two surfaces stop telling
/// two stories about one file.
fn home_identity(rel_path: &str, schema: &Schema) -> Option<String> {
    let ty = &schema.ty;
    if let Some(placement) = &schema.placement {
        return (rel_path == placement.file).then(|| format!("{ty}:{ty}"));
    }
    let dir = schema.location.as_deref()?.trim_end_matches('/');
    let rest = rel_path.strip_prefix(&format!("{dir}/"))?;
    let slug = rest.strip_suffix(".md")?;
    // `is_slug` subsumes the old non-empty + no-`/` pair (a `/` is outside `[a-z0-9-]`),
    // so the *nested* candidate the doc comment already excluded still resolves to `None`
    // — by the same predicate that now also excludes `My Decision`.
    engine::slug::is_slug(slug).then(|| format!("{ty}:{slug}"))
}

/// The adoption refusal for a candidate that is **conformant at a managed home but bears no
/// managed identity** — `ingest.unaddressable-identity` (M51 Inc 9 / T4;
/// `design/validation.md` → The M51 registrations — Increment 9).
///
/// Its subject is exactly the set [`home_identity`] answers `None` over, which is one class
/// with two shapes and one consequence: **no `<type>:<slug>` address reaches the file**, so
/// adopting it records a file-state baseline and an edge-index entry under an identity no
/// door can name. Driven at the base, both shapes adopted at exit 0:
///
/// - **the name is not a doc id** (`docs/decisions/My Decision.md`) — `jigc doc list` then
///   called it `unregistered`, `jigc doc show` refused it `store.malformed-slug`, and `jigc
///   validate` exited 1 with `schema-conformance.unadopted-instance` **routed back at `jigc
///   ingest`** — a loop, the door the sweep names having just claimed the file was adopted;
/// - **the path is nested below the flat home** (`docs/decisions/sub/nested-one.md`) — worse,
///   because [`engine::index::committed_instances`] enumerates a location home with a flat
///   `read_dir`, so after the silent adoption *no* surface mentioned the file again.
///
/// **The route is a [`Route::human`] naming `git mv`**, on M45's `owner-artifact.present`
/// precedent (a `git add` route rendered the same way): `jigc doc rename` is not the repair —
/// it repoints a doc jigc **manages**, and this file by definition is not one. The `git mv`
/// operands render through [`crate::task::shell_token`], so the printed line survives a shell
/// as itself over a name holding a space or a metachar — which is the very class of name that
/// lands here. The destination is named, never performed (jigc does not auto-move), and git
/// refuses an occupied destination loudly rather than clobbering it.
fn unaddressable_identity_finding(rel_path: &str, schema: &Schema) -> Finding {
    let ty = &schema.ty;
    let dir = schema
        .location
        .as_deref()
        .map(|location| location.trim_end_matches('/'))
        .unwrap_or_default();
    // The stem the file already carries, and the addressable name it would carry at the
    // home — `slugify` is the mint rule, so the destination is the name jigc itself would
    // have written. An empty mint (a stem that normalizes to nothing) names no destination
    // rather than an empty one.
    let stem = rel_path
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".md"))
        .unwrap_or_default();
    let minted = engine::slug::slugify(stem);
    let destination = (!dir.is_empty() && !minted.is_empty()).then(|| format!("{dir}/{minted}.md"));
    let nested = !dir.is_empty()
        && rel_path
            .strip_prefix(&format!("{dir}/"))
            .is_some_and(|rest| rest.contains('/'));

    let cause = if nested {
        format!(
            "it sits below `{dir}/` rather than directly in it, and jigc homes \
             every `{ty}` as a direct child of that directory, so no \
             `<type>:<slug>` address reaches it"
        )
    } else {
        "its name is not a doc id, so no `<type>:<slug>` address reaches it — jigc \
         names every doc it writes `<slug>.md`"
            .to_string()
    };
    let act = if nested {
        format!("move it onto the `{ty}` home")
    } else {
        "rename it to a doc id".to_string()
    };
    let repair = match &destination {
        Some(destination) => format!(
            "{act} — `git mv {} {}` — then re-run `jigc ingest`",
            crate::task::shell_token(rel_path),
            crate::task::shell_token(destination),
        ),
        // No destination can be minted (the name normalizes to nothing), so the route
        // names the grammar the new name must satisfy instead of an empty path.
        None => format!(
            "{act}: give it a `<slug>.md` name directly under the `{ty}` home — a \
             lowercase `[a-z0-9-]` stem with no leading, trailing or doubled `-` \
             — then re-run `jigc ingest`"
        ),
    };
    Finding::graded(
        Severity::Blocking,
        "ingest.unaddressable-identity",
        format!("conformant `{ty}` at `{rel_path}` is not adopted: {cause}"),
        // The path form, and necessarily so: the whole fault is that this file has no
        // `<type>:<slug>` identity to key at (`design/command-output-contract.md` → the
        // target-normal forms, the declared `ingest.*` / `file-state.*` exception).
        Some(Location::addressed(rel_path, 1, 1)),
        Some(Route::human(format!(
            "{repair}; `jigc doc rename` is not the repair — it repoints a doc jigc \
             already manages, and this file has never been adopted"
        ))),
    )
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
        Some(format!("move {rel_path} into {location}, then re-run `jigc ingest`").into()),
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

/// Every **cascade-resolved** schema — the persisted set the classifier runs each
/// candidate against (the engine stays domain-empty; the CLI feeds the cascade in).
/// Returned doctype-sorted (the engine's discovery dedups + sorts independently, so the
/// order is the resolver's, not this surface's, concern).
///
/// Surface B: this is what makes `jigc ingest` / `jigc unmanage` discover and classify
/// managed docs at the same parent the write surfaces promote them to. Read pack-only,
/// `ingest` reported a document `jigc validate` was adjudicating as an `adr` — its
/// fields, its sections, its baseline — as `unmanaged`, matching no schema at all; once
/// the doc was conformant it did worse, routing the operator to move it **out of** the
/// home the resolved cascade declares for it (M49 Increment 3 T2).
pub(crate) fn load_schemas(
    pack: &dyn engine::packsource::PackSource,
    project_config: &Path,
) -> Result<Vec<Schema>> {
    Ok(crate::start::resolved_schemas(pack, project_config)?
        .into_values()
        .collect())
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
        return Err(crate::locate::not_set_up(&project_config));
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

#[cfg(test)]
mod tests {
    use super::*;
    use engine::finding::RouteKind;

    /// An `adr`-typed schema fixture (the shipped shape minus the pack-declared
    /// `code-anchor` leaf, which needs the pack's field-type table to resolve) — the
    /// near-miss route reads only the doctype name and the parse/conformance detail.
    fn adr_schema() -> Schema {
        engine::schema::load_schema(
            b"\
type: adr
location: decisions/
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
  - id: context
    slot: { hint: Forces. }
  - id: decision
    slot: { hint: What was decided. }
",
        )
        .expect("the adr fixture schema loads")
    }

    /// An ADR-shaped doc sitting at the `adr` home that does **not** conform (its
    /// `## Decision` section is missing) — the near-miss the triage re-derives a route
    /// for.
    const NEAR_MISS_ADR: &str = "\
---
status: accepted
---

# Auth choice

## Context
Sessions must survive a restart.
";

    /// (M47 inc-10 T4 · N19) `jigc ingest`'s near-miss route is no weaker than the one
    /// `jigc validate` already hands over for the same file. The shipped route was the
    /// bare prose *"reconcile decisions/auth-choice.md against the `adr` schema"* while
    /// the store sweep's sibling (`orphan::unregistered_route`) has emitted the runnable
    /// `jigc migrate <rel> --as <doctype>` since M40 — two surfaces, one file, one
    /// repair, and only one of them followable. Reshaped to that argv it is a single
    /// backticked real command, so it is constructed through [`Route::mechanical`] and
    /// buys the CLI parse fence.
    #[test]
    fn ingest_near_miss_route_hands_over_the_migrate_argv() {
        crate::route_fence::install();
        let migratable = ["adr".to_string()].into_iter().collect();

        let finding = near_miss_finding(
            "decisions/auth-choice.md",
            NEAR_MISS_ADR,
            &adr_schema(),
            &migratable,
            false,
        );

        let route = finding
            .route
            .expect("a needs-reconcile row carries a route");
        assert!(
            matches!(route.kind(), RouteKind::Mechanical { .. }),
            "the reshaped route is a copy-runnable command: {route:?}",
        );
        assert!(
            route.starts_with("`jigc migrate decisions/auth-choice.md --as adr`"),
            "it leads with the same adoption argv the store sweep's sibling emits: {route}",
        );
    }

    /// **The route is a command, so its operand is quoted** (M51 completion audit). The
    /// argv above is the emitted bytes — `Route::mechanical` composes its text as
    /// `argv.join(" ")` — and the path in it comes off the filesystem, not out of a
    /// grammar. Driven at `befdbf93` a non-conformant `docs/decisions/my notes.md` panicked
    /// the whole `jigc ingest` run at exit 101 on the fence's own token check; in release it
    /// would have printed `jigc migrate docs/decisions/my notes.md --as adr`, which parses
    /// as a migrate of `docs/decisions/my`.
    ///
    /// Its sibling one function over ([`unaddressable_identity_finding`]'s `git mv`) had
    /// rendered both operands through `shell_token` since M51 Increment 9 — the same wave —
    /// so this is the un-swept half of one rule, not a new one.
    #[test]
    fn ingest_near_miss_route_quotes_a_path_a_shell_would_re_lex() {
        crate::route_fence::install();
        let migratable = ["adr".to_string()].into_iter().collect();

        let finding = near_miss_finding(
            "decisions/my notes.md",
            NEAR_MISS_ADR,
            &adr_schema(),
            &migratable,
            false,
        );

        let route = finding
            .route
            .expect("a needs-reconcile row carries a route");
        assert!(
            route.starts_with("`jigc migrate 'decisions/my notes.md' --as adr`"),
            "the operand is the bytes a shell re-lexes as the path: {route}",
        );
        assert!(
            engine::finding::command_spans_are_shell_safe(&route),
            "and the whole span is runnable: {route}",
        );
    }

    /// The migratable axis: the sibling route is **gated** on a shipped
    /// `migrate-<doctype>` workflow, because `jigc migrate --as <doctype>` hard-rejects
    /// without one. A doctype that ships no migrate workflow therefore keeps a human
    /// direction that says so — a mechanical route here would be a route that cannot be
    /// run, which is the defect one level down.
    #[test]
    fn ingest_near_miss_route_stays_human_without_a_migrate_workflow() {
        crate::route_fence::install();
        let migratable = std::collections::BTreeSet::new();

        let finding = near_miss_finding(
            "decisions/auth-choice.md",
            NEAR_MISS_ADR,
            &adr_schema(),
            &migratable,
            false,
        );

        let route = finding
            .route
            .expect("a needs-reconcile row carries a route");
        assert!(
            matches!(route.kind(), RouteKind::Human),
            "no migrate workflow ships, so no command is offered: {route:?}",
        );
        assert!(
            route.contains("migrate-adr") && !route.contains("jigc migrate "),
            "and the route says why it offers none: {route}",
        );
    }

    /// The registration axis — the half of this route's domain the shipped
    /// `unregistered_route` sibling never reaches. A near-miss can be an **already
    /// adopted** doc that stopped conforming (the M42 stale-schema-version case), and
    /// for it adoption is not the repair at all: `jigc migrate` would try to adopt what
    /// is already managed. That arm therefore stays a human direction and says so.
    #[test]
    fn ingest_near_miss_route_offers_no_adoption_for_an_already_managed_doc() {
        crate::route_fence::install();
        let migratable = ["adr".to_string()].into_iter().collect();

        let finding = near_miss_finding(
            "decisions/auth-choice.md",
            NEAR_MISS_ADR,
            &adr_schema(),
            &migratable,
            true,
        );

        let route = finding
            .route
            .expect("a needs-reconcile row carries a route");
        assert!(
            matches!(route.kind(), RouteKind::Human),
            "an adopted doc is not re-adopted: {route:?}",
        );
        assert!(
            !route.contains("jigc migrate decisions/auth-choice.md"),
            "the adoption argv is never offered for a managed doc: {route}",
        );
        assert!(
            route.contains("already managed"),
            "and the route says why: {route}",
        );
    }
}
