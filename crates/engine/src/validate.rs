//! The validation engine — read-only probes over a scope, severity via cascade,
//! the `finalize` gate.
//!
//! See `design/validation.md`. MVP engine-native probes: `workflow-refs`,
//! `file-state`, plus the synthetic `schema-conformance` integrity checks.
//!
//! ## `schema-conformance` (synthetic category)
//!
//! `schema-conformance` is a **synthetic** validate category (`validation.md` →
//! Synthetic categories): not a literal `check(target, ctx)` probe, but a namespace
//! for schema-driven integrity checks the engine runs over an already-parsed
//! instance. All four ship: inc-4 added the three per-instance checks below;
//! inc-5 adds the cross-doc `ref-resolves` (forward-ref / edge-index integrity),
//! which [`validate_task`] now runs once over the whole task (the `finalize` gate).
//!
//! - **`required-slot-present`** — every declared body slot must hold non-empty
//!   prose. The parser already requires the section *heading* (a missing heading is
//!   a `conformance.section-missing` parse error), so this check fires on the
//!   present-but-empty slot: the LLM left it unfilled. (`finalize.md` → phase 2:
//!   "validate blocks on required-slot presence".)
//! - **`required-field-present`** — every **author-required** declared field must be
//!   present in the instance. A field is author-required iff it carries neither a
//!   `default:` nor a `set:` — those are CLI-derived / defaulted, never the author's
//!   obligation. (`document-type-schema.md` → Field provenance.)
//! - **`field-value-conformant`** — every present field's value must pass its
//!   declared type, reusing [`crate::write::check_value`] (the same write-time
//!   adjudicator). A malformed date / non-member enum / empty string blocks.
//! - **`ref-resolves`** — every task-touched forward edge (a schema `ref` field) must
//!   resolve in one of the two reachable surfaces (committed store or this task's
//!   working area). Walked over the overlaid edge index ([`crate::index`]); a dangling
//!   target blocks. Unlike the three above, this is a *cross-doc* check run once over
//!   the whole task, not per parsed instance.
//!
//! Each violation is one intrinsic **blocking** `schema-conformance.*` [`Finding`]
//! (the inventory default severity; cascade tuning is not wired into engine probes
//! yet, paralleling `workflow-refs` / `file-state`). A fully-conformant instance
//! yields none.

use crate::field_block::Field;
use crate::file_state::{FileStateRecord, file_state};
use crate::finding::{Finding, Location, Severity};
use crate::parse::{Document, ParsedItem, ParsedSection, parse_sections};
use crate::probe::{EffectiveStateSnapshot, ProbeRequest, ProbeRun, ingest_probe_run};
use crate::result::ValidationReport;
use crate::schema::{Field as SchemaField, FieldType, Schema, Section, SectionBody};
use crate::target_surface::enumerate_target_surface;
use std::collections::BTreeMap;
use std::path::Path;

/// The working-area sub-directory holding the task's staged doc instances
/// (`DECISIONS.md` 2026-05-31 → Task working-area on-disk layout: a staged instance
/// lives at `.jigc/tasks/<id>/docs/<type>:<slug>.md`).
const DOCS_DIR: &str = "docs";

/// The pack probe the `code-anchor` field type binds to (`validation.md` → The
/// `doc-code` probe). The engine names it on the wire request; the CLI invoker resolves
/// the program. The engine ships no probe content (the engine-empty invariant) — this is
/// only the wire `probe_id`, never an embedded adjudicator.
const DOC_CODE_PROBE: &str = "doc-code";

/// The filename the engine materializes the effective-state snapshot to, under the task
/// working area — the path-ref the [`ProbeRequest`] carries to the subprocess probe
/// (`validation.md` → The wire contract: the engine materializes a read-only snapshot in
/// the probe's read scope and the request names its path). It rides in the gitignored
/// working area, never committed.
const SNAPSHOT_FILE: &str = "probe-snapshot.json";

/// The seam the engine drives a subprocess pack probe over — a `Fn(&ProbeRequest) ->
/// io::Result<ProbeRun>` the **CLI** supplies (the engine stays shell-free;
/// `module-layout.md` → Probe boundary). The engine enumerates the target surface,
/// materializes the snapshot, and builds the [`ProbeRequest`]; the invoker runs the
/// program and reports the raw outcome; the engine ingests it ([`ingest_probe_run`]).
/// This mirrors the `head` precedent — a capability the engine cannot produce
/// (shelling out) is passed in from the CLI (`DECISIONS.md` 2026-06-06, M10 inc-5 / T2).
pub type ProbeInvoker<'a> = dyn Fn(&ProbeRequest) -> std::io::Result<ProbeRun> + 'a;

/// The owner-assigned artifact home: every `owner-artifact` owned-location path must
/// be a repo-relative path **under** this prefix (`design/methodology-docs.md` → The
/// engine work, item 3 / the recording surface — a bare `exists(path)` would pass with
/// any pre-existing file, so the field is constrained to a path under an owned artifact
/// home, `completions/artifacts/<milestone>/…`). The trailing slash is significant — a
/// value of exactly `completions/artifacts` (no milestone segment) is *not* under it.
///
/// This is a **structural** location the intrinsic #5 gate mechanizes, not domain
/// content: `owned-location` is an engine-native field type (not a pack type), so the
/// home prefix the gate keys on is engine knowledge, mirroring how the engine owns the
/// `docs/`/`decisions/` working-area conventions.
const OWNED_ARTIFACT_HOME: &str = "completions/artifacts/";

/// The CLI-supplied git tracked-status predicate the engine threads through
/// [`validate_task`] into the #5 owner-artifact gate — a `Fn(&str) -> bool` taking a
/// **repo-relative** path and answering whether git tracks it (staged or committed),
/// the inverse of the CLI's `git ls-files --others` untracked set. The engine reads
/// existence under `repo_root` itself (a filesystem effect it already has) but **never
/// shells out** for tracked-status — that is the CLI's, mirroring the `ProbeInvoker`
/// shell-free seam (`design/methodology-docs.md` → The engine work, item 3; the planner
/// note's threaded-predicate shape). The engine reads the field string + the file's
/// presence + this predicate, **never the artifact's bytes** (presence, not content —
/// the determinism boundary).
pub type TrackedPredicate<'a> = dyn Fn(&str) -> bool + 'a;

/// Validate one task working area — the single engine both `task validate` and
/// `finalize` phase 2 call (`validation.md` → How it gates `finalize`: one engine,
/// two entry points, so what `validate` reports and what `finalize` blocks on can
/// never diverge). No git, no commit.
///
/// Resolves the working area's staged doc instances (`<dir>/docs/*.md`, each named
/// `<type>:<slug>.md`), then over each instance runs the two MVP task-scope probes
/// and aggregates their severity-classified findings into a [`ValidationReport`]:
///
/// - **`file-state`** — hashes the staged bytes against `record` (baseline-adopt on
///   first encounter, blocking drift on a recorded mismatch). The probe advances the
///   record on adopt; the caller persists it.
/// - **`schema-conformance`** — parses each instance against its (caller-supplied)
///   schema. A *parse-level* conformance failure (missing/renamed heading, malformed
///   anchor, …) surfaces those findings directly; a clean parse then runs the
///   [`schema_conformance`] checks (required slot/field present, field-value
///   conformant) over the instance.
///
/// `schemas` maps a doc-type name to its resolved [`Schema`] — the engine stays
/// **domain-empty** (`CLAUDE.md` → engine ships empty of domain content): the caller
/// (CLI) resolves the cascade and feeds the schemas in. A staged file whose type
/// prefix has no schema in `schemas` raises a blocking `schema-conformance.unknown-type`
/// finding (the working area references a type the resolved cascade does not define).
///
/// Findings aggregate in a stable sweep order: docs by path-sorted filename, and
/// within each doc `file-state` before `schema-conformance`, then the cross-doc
/// `schema-conformance.ref-resolves` (forward-ref / edge-index integrity) sweep over
/// the whole task once.
///
/// `resolved` is the resolved cascade the caller already built; it feeds the
/// engine-owned severity post-pass at [`ValidationReport::new`] (`validation.md` →
/// Severity assignment — the M6 post-pass), so a project can tune a tunable check's
/// severity. A no-delta cascade leaves every emitted severity untouched (the
/// byte-identical no-override path).
///
/// `invoke_doc_code` is the CLI-supplied subprocess seam ([`ProbeInvoker`]) the engine
/// drives the `doc-code` probe over — the engine enumerates the effective-state
/// `code-anchor` surface, materializes the snapshot, and ingests the outcome, but the
/// invoke step (which shells out) is the CLI's, keeping the engine shell-free. A task
/// with no `code-anchor` leaf never calls it (the omitting-context inert path).
///
/// `tracked` is the CLI-supplied git tracked-status predicate ([`TrackedPredicate`]) the
/// #5 owner-artifact gate consults: for each staged instance's `owned-location` leaf, the
/// engine validates the path is repo-relative + under the owned artifact home + the file
/// is present (its own filesystem effect) and asks `tracked` whether git tracks it — the
/// engine never shells out for tracked-status. A task with no `owned-location` field never
/// consults it (the omitting-context inert path).
///
/// `repo_root` is the committed-store root, `jigc_root` is the `.jigc/` home (where the
/// edge index caches), and `head` is the opaque HEAD stamp the committed index is
/// tagged with (the CLI reads it via `git`, keeping the engine shell-free). These three
/// feed the `ref-resolves` walk: [`crate::index::load_committed`] builds/loads the
/// committed edge index, [`crate::index::overlay_working`] layers this task's staged
/// edges over it, and [`crate::index::ref_resolves`] walks every task-touched forward
/// edge — a dangling target (in neither the committed store nor this task's working
/// area) is a blocking finding (`validation.md` → Forward-ref resolution). So `finalize`
/// — which gates on exactly what `validate` reports — blocks on a dangling `supersedes`
/// and passes on a resolvable one (`worked-examples.md` → Superseding decision).
// Every parameter is a distinct determinism-boundary input the CLI threads in (the
// engine produces none of them): the working area, the resolved schemas/cascade, the
// committed-store + `.jigc/` roots, the git HEAD stamp, and the shell-free probe seam.
// Bundling them into a struct would only relocate the same arity, so the lint is allowed
// at this one composition point.
#[allow(clippy::too_many_arguments)]
pub fn validate_task(
    dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    record: &mut FileStateRecord,
    repo_root: &Path,
    jigc_root: &Path,
    head: &str,
    resolved: &crate::cascade::Resolved,
    invoke_doc_code: &ProbeInvoker<'_>,
    tracked: &TrackedPredicate<'_>,
) -> std::io::Result<ValidationReport> {
    let mut findings = Vec::new();
    for entry in staged_instances(dir)? {
        let StagedInstance { rel_key, filename } = entry;
        let bytes = std::fs::read(dir.join(DOCS_DIR).join(&filename))?;

        // `file-state` over this one instance, keyed by its docs-relative path.
        findings.extend(file_state(record, &[(rel_key.as_str(), &bytes)]));

        // `schema-conformance` over the instance, resolving its type from the
        // `<type>:<slug>.md` filename. A non-UTF-8 instance can't be a managed
        // Markdown doc; the parser owns that, so we require a UTF-8 read here.
        let source = String::from_utf8_lossy(&bytes);
        findings.extend(conformance_for(&filename, schemas, &rel_key, &source));

        // The #5 owner-artifact presence gate over this instance's `owned-location`
        // leaves: each named path must be a repo-relative path under the owned artifact
        // home and the artifact durably present + tracked (`design/methodology-docs.md`
        // → The engine work, item 3). An instance with no `owned-location` field yields
        // nothing — the omitting-context inert path (mirroring the doc-code surface).
        findings.extend(owner_artifact_present(
            &filename, schemas, &rel_key, &source, repo_root, tracked,
        ));
    }

    // Committed-store OOB reconciliation — sweep the committed managed docs and route
    // any out-of-band drift (`reconciliation.md` → The state machine / Detection timing:
    // the `task validate` full sweep, shared by `finalize`'s preflight). Runs over the
    // freshly-loaded committed edge index so a clean absorb's new edges feed the
    // forward-ref walk below; the record/index advance in place (the caller persists).
    let mut committed = crate::index::load_committed(repo_root, jigc_root, schemas, head);
    findings.extend(crate::file_state::reconcile_committed_store(
        record,
        &mut committed,
        schemas,
        repo_root,
        dir,
    ));

    // `schema-conformance.ref-resolves` — the cross-doc forward-ref / edge-index
    // integrity sweep, run once over the whole task (`validation.md` → Forward-ref
    // resolution). The committed edge index (rebuilt/loaded against `head`, with any
    // absorbed OOB edges folded in) is overlaid with this task's staged edges; every
    // task-touched forward edge must resolve in one of the two reachable surfaces
    // (committed store or this task's working area).
    let overlay = crate::index::overlay_working(&committed, dir, schemas);
    findings.extend(crate::index::ref_resolves(
        &overlay, repo_root, dir, schemas,
    ));

    // `doc-code` — the pack-provided doc↔code probe (`validation.md` → The `doc-code`
    // probe). The engine enumerates the task's effective-state `code-anchor` leaves,
    // materializes the serializable snapshot, builds the wire request, and ingests the
    // probe's findings; the CLI-supplied invoker runs the subprocess (the engine stays
    // shell-free). A task whose effective state carries **no** anchor produces zero
    // pairs, so the whole block is skipped — the invoker is never called and no snapshot
    // is written, leaving the no-anchor path byte-identical to the pre-wiring sweep (the
    // omitting-context guard, hardening #5).
    findings.extend(schedule_doc_code(dir, repo_root, schemas, invoke_doc_code)?);

    // Severity assignment is the engine-owned post-pass at report construction
    // (`validation.md` → Severity assignment — the M6 post-pass): `resolved` is the
    // cascade the caller resolved, read per-finding by inventory `(probe, check)`
    // membership. A no-delta cascade leaves every emitted severity untouched.
    Ok(ValidationReport::new(findings, resolved))
}

/// Schedule the `doc-code` probe over the task's effective-state target surface
/// (`validation.md` → The `doc-code` probe → Target surface). The engine owns three of
/// the four steps — **enumerate** ([`enumerate_target_surface`]), **materialize** the
/// serializable [`EffectiveStateSnapshot`] (written to `<dir>/probe-snapshot.json`, the
/// path-ref the wire carries), and **ingest** ([`ingest_probe_run`]) — and hands the
/// **invoke** step to the CLI-supplied `invoke` closure (the engine never shells out).
///
/// One probe invocation covers the whole surface: the snapshot itemizes every anchor,
/// and the probe adjudicates each against `working_tree_root` (`repo_root` — the
/// working-tree code the about-to-be-committed bytes live in). The wire request names a
/// representative `target` (the first anchor's address) so the envelope is well-formed;
/// the probe reads the full anchor set from the snapshot, not the request's `target`.
///
/// **Inert when the surface is empty** — a task carrying no `code-anchor` leaf yields no
/// pairs, so the invoker is never called and no snapshot is written: the returned findings
/// are empty and the sweep is byte-identical to the pre-wiring path (the omitting-context
/// guard, hardening #5). `config` is the empty object — doc-code reads no cascade config
/// beyond severity, which the engine assigns downstream in its post-pass.
fn schedule_doc_code(
    dir: &Path,
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    invoke: &ProbeInvoker<'_>,
) -> std::io::Result<Vec<Finding>> {
    let anchors = enumerate_target_surface(dir, repo_root, schemas)?;
    if anchors.is_empty() {
        return Ok(Vec::new());
    }

    // The representative target the wire envelope carries (the probe reads the full set
    // from the snapshot). `anchors` is non-empty here, so the first is always present.
    let target = anchors[0].address.clone();

    let snapshot = EffectiveStateSnapshot::new(anchors, repo_root.to_path_buf());
    let snapshot_path = dir.join(SNAPSHOT_FILE);
    std::fs::write(&snapshot_path, serde_json::to_vec(&snapshot)?)?;

    let request = ProbeRequest::new(
        DOC_CODE_PROBE,
        target,
        snapshot_path,
        serde_json::Map::new(),
    );
    let run = invoke(&request)?;
    Ok(ingest_probe_run(DOC_CODE_PROBE, &run))
}

/// One staged doc instance under `<dir>/docs/`: its `docs/<filename>` record key
/// and the bare `filename` (`<type>:<slug>.md`).
struct StagedInstance {
    rel_key: String,
    filename: String,
}

/// The task's staged doc instances under `<dir>/docs/`, in **path-sorted** filename
/// order (the stable sweep order). A working area with no `docs/` dir (nothing
/// staged yet) yields an empty list, not an error. Only `*.md` files are instances.
fn staged_instances(dir: &Path) -> std::io::Result<Vec<StagedInstance>> {
    let docs = dir.join(DOCS_DIR);
    let read = match std::fs::read_dir(&docs) {
        Ok(read) => read,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(err),
    };
    let mut names: Vec<String> = Vec::new();
    for entry in read {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".md") {
            names.push(name);
        }
    }
    names.sort();
    Ok(names
        .into_iter()
        .map(|filename| StagedInstance {
            rel_key: format!("{DOCS_DIR}/{filename}"),
            filename,
        })
        .collect())
}

/// Run `schema-conformance` over one staged instance: resolve its type from the
/// `<type>:<slug>.md` filename, parse against the schema, and surface parse-level
/// findings or the [`schema_conformance`] checks. A type with no schema in the
/// resolved cascade raises a blocking `schema-conformance.unknown-type`.
fn conformance_for(
    filename: &str,
    schemas: &BTreeMap<String, Schema>,
    rel_key: &str,
    source: &str,
) -> Vec<Finding> {
    let ty = filename.split(':').next().unwrap_or(filename);
    let Some(schema) = schemas.get(ty) else {
        return vec![Finding::graded(
            Severity::Blocking,
            "schema-conformance.unknown-type",
            format!(
                "staged doc `{rel_key}` has type `{ty}`, which the resolved cascade does not define"
            ),
            Some(Location::addressed(rel_key, 1, 1)),
            None,
        )];
    };
    match parse_sections(schema, source) {
        Ok(doc) => schema_conformance(schema, source, &doc),
        Err(parse_findings) => parse_findings,
    }
}

/// The intrinsic #5 **owner-artifact presence gate** over one staged instance's
/// `owned-location` leaves (`design/methodology-docs.md` → The engine work, item 3).
///
/// For every header / simple-section field whose declared type is the engine-native
/// [`FieldType::OwnedLocation`], the gate reads the authored path string and emits one
/// blocking `owner-artifact.present` [`Finding`] unless **all** of:
///
/// - the path is repo-relative (not absolute) and carries no `..` component;
/// - it is under the owned artifact home ([`OWNED_ARTIFACT_HOME`] — not a bare
///   pre-existing file, the B-3 / Codex-blocking-1 anti-vacuity bar);
/// - the file is **present** under `repo_root` and does not symlink-escape the home
///   (the resolved real path stays under `<repo_root>/<home>`);
/// - git **tracks** it (`tracked` — the artifact is durably staged/committed, not a
///   present-but-untracked scratch file).
///
/// **Presence, not content** — the gate reads the field string, the file's existence,
/// and the tracked flag; it **never** reads the artifact's bytes (the determinism
/// boundary). The honest bound: it proves an artifact is present at the named owned path,
/// **not** that the genuine audit happened — the orchestrator writes both the field and
/// the file, so authenticity stays an orchestration-level recorded responsibility (the M8
/// orchestrator-responsibility analogue).
///
/// An instance whose schema declares no `owned-location` field, or which omits the field,
/// yields nothing — the omitting-context inert path. A type with no schema / an
/// unparseable instance is `conformance_for`'s concern, not this gate's, so it is skipped
/// here (best-effort, mirroring the target-surface enumeration).
fn owner_artifact_present(
    filename: &str,
    schemas: &BTreeMap<String, Schema>,
    rel_key: &str,
    source: &str,
    repo_root: &Path,
    tracked: &TrackedPredicate<'_>,
) -> Vec<Finding> {
    let ty = filename.split(':').next().unwrap_or(filename);
    let Some(schema) = schemas.get(ty) else {
        return Vec::new();
    };
    let Ok(doc) = parse_sections(schema, source) else {
        return Vec::new();
    };
    let mut findings = Vec::new();
    for section in &schema.sections {
        let SectionBody::Simple { fields, .. } = &section.body else {
            continue; // owned-location lives on the meta header / a simple section.
        };
        let Some(parsed) = doc.sections.iter().find(|s| s.id == section.id) else {
            continue;
        };
        for declared in fields {
            if declared.ty != FieldType::OwnedLocation {
                continue;
            }
            let Some(present) = parsed.fields.iter().find(|f| f.key == declared.id) else {
                continue; // omitted: nothing to adjudicate (the inert path).
            };
            let crate::field_block::Value::Scalar(path) = &present.value else {
                continue; // a list value is not an owned-location shape.
            };
            if let Some(why) = owned_location_violation(path, repo_root, tracked) {
                findings.push(blocking_conformance(
                    "owner-artifact.present",
                    format!(
                        "owner-artifact `{}` in section `{}` of `{rel_key}`: {why}",
                        declared.id, section.id
                    ),
                    Some(Location::addressed(
                        format!("{rel_key}#{}/{}", section.id, declared.id),
                        1,
                        1,
                    )),
                ));
            }
        }
    }
    findings
}

/// Adjudicate one `owned-location` path: `None` when the artifact is safe + durably
/// present + tracked, else `Some(reason)` (the human-readable cause the finding carries).
/// Reads only the path string, the file's presence under `repo_root`, and the `tracked`
/// flag — never the artifact's bytes (the determinism boundary).
fn owned_location_violation(
    path: &str,
    repo_root: &Path,
    tracked: &TrackedPredicate<'_>,
) -> Option<String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Some("the path is empty".to_string());
    }
    if Path::new(trimmed).is_absolute() || trimmed.starts_with('/') {
        return Some(format!("`{trimmed}` is absolute, not a repo-relative path"));
    }
    // A `..` component would let the path climb out of the owned home (and out of the
    // repo). Reject on the *textual* component, before any filesystem resolution.
    if Path::new(trimmed)
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Some(format!("`{trimmed}` contains a `..` component"));
    }
    if !trimmed.starts_with(OWNED_ARTIFACT_HOME) || trimmed.len() == OWNED_ARTIFACT_HOME.len() {
        return Some(format!(
            "`{trimmed}` is not under the owned artifact home `{OWNED_ARTIFACT_HOME}<milestone>/`"
        ));
    }
    // Presence under the repo root. The path is repo-relative + `..`-free, so the join
    // stays within the tree textually; a symlink could still escape, caught next.
    let full = repo_root.join(trimmed);
    if !full.exists() {
        return Some(format!("`{trimmed}` names no file under the repository"));
    }
    // Symlink-escape: the resolved real path must stay under the owned home. Canonicalize
    // both the home and the target (the home must resolve too — `..`-free + present).
    let home_real = repo_root.join(OWNED_ARTIFACT_HOME).canonicalize().ok();
    match (full.canonicalize().ok(), home_real) {
        (Some(real), Some(home)) if real.starts_with(&home) => {}
        _ => {
            return Some(format!(
                "`{trimmed}` resolves outside the owned artifact home (symlink escape)"
            ));
        }
    }
    if !tracked(trimmed) {
        return Some(format!(
            "`{trimmed}` is present but untracked — stage it so it is durably committed"
        ));
    }
    None
}

/// Run the synthetic `schema-conformance` checks over a parsed instance: every
/// declared body slot is non-empty (`required-slot-present`), every author-required
/// field is present (`required-field-present`), and every present field's value is
/// type-conformant (`field-value-conformant`). A **repeatable** section runs the same
/// three checks per item over its block leaves (the `id-from` heading field exempted),
/// findings addressed at `#section/item/leaf`. Returns one blocking [`Finding`] per
/// violation, in section-document order; a conformant instance yields an empty `Vec`.
///
/// `ref-resolves` (forward-ref / edge-index integrity) is **not** run here — it is a
/// *cross-doc* check [`validate_task`] runs once over the whole task, not a per-instance
/// check. `source` is needed to slice the opaque slot spans the parser recorded.
pub fn schema_conformance(schema: &Schema, source: &str, doc: &Document) -> Vec<Finding> {
    let mut findings = Vec::new();
    for section in &schema.sections {
        let Some(parsed) = doc.sections.iter().find(|s| s.id == section.id) else {
            // A section the parser did not map (a header section carries no
            // `ParsedSection`) — nothing slot/field/item-shaped to adjudicate here.
            continue;
        };
        match &section.body {
            SectionBody::Simple {
                slot: declared_slot,
                fields: declared_fields,
            } => {
                // required-slot-present: a declared body slot must hold non-empty prose.
                if declared_slot.is_some() {
                    check_slot_present(section, parsed, source, &mut findings);
                }
                // required-field-present + field-value-conformant over the declared fields.
                for declared in declared_fields {
                    check_field(section, declared, parsed, &mut findings);
                }
            }
            SectionBody::Repeatable { repeatable } => {
                check_repeatable(section, repeatable, parsed, source, &mut findings);
            }
        }
    }
    findings
}

/// Run the three synthetic checks over each item of a **repeatable** section's
/// block (`validation.md` → schema-conformance scope: the lifted MVP repeatable
/// limit). Per item, every declared block leaf is adjudicated — a `slot` leaf via
/// `required-slot-present`, a `field` leaf via `required-field-present` +
/// `field-value-conformant` — **except the `id-from` source field**, which renders
/// as the item heading not a trailing bullet (mirroring [`crate::parse`]'s
/// `ItemTemplate::from` heading-field exclusion); checking its presence as a bullet
/// would false-fail every conformant item. Findings address at `#section/item/leaf`
/// (the M13 fragment vocabulary).
fn check_repeatable(
    section: &Section,
    repeatable: &crate::schema::Repeatable,
    parsed: &ParsedSection,
    source: &str,
    findings: &mut Vec<Finding>,
) {
    for item in &parsed.items {
        for leaf in &repeatable.block {
            match leaf {
                crate::schema::Leaf::Slot { id, .. } => {
                    check_item_slot_present(section, item, id, source, findings);
                }
                crate::schema::Leaf::Field(field) => {
                    // The id-source field is the item heading, never a bullet.
                    if field.id == repeatable.id_from {
                        continue;
                    }
                    check_item_field(section, item, field, findings);
                }
            }
        }
    }
}

/// `required-slot-present` for one repeatable item's declared slot: its prose must
/// be non-empty. The parser records the item's slot span (the heading is present,
/// so the section parsed), so an all-whitespace slice is the unfilled-slot case.
/// Addressed at `#section/item/leaf`.
fn check_item_slot_present(
    section: &Section,
    item: &ParsedItem,
    leaf_id: &str,
    source: &str,
    findings: &mut Vec<Finding>,
) {
    let filled = item
        .slot
        .as_ref()
        .map(|span| !span.slice(source).trim().is_empty())
        .unwrap_or(false);
    if !filled {
        let line = item.slot.as_ref().map(|span| span.start_line).unwrap_or(1);
        findings.push(blocking_conformance(
            "schema-conformance.required-slot-present",
            format!(
                "required slot `{leaf_id}` in item `{}` of section `{}` is empty",
                item.id, section.id
            ),
            Some(Location::addressed(
                item_leaf_address(section, item, leaf_id),
                line,
                1,
            )),
        ));
    }
}

/// `required-field-present` + `field-value-conformant` for one repeatable item's
/// declared block field. An absent author-required field blocks; a present field
/// whose value fails its declared type blocks. Both address at `#section/item/leaf`.
fn check_item_field(
    section: &Section,
    item: &ParsedItem,
    declared: &SchemaField,
    findings: &mut Vec<Finding>,
) {
    let address = item_leaf_address(section, item, &declared.id);
    match item.fields.iter().find(|f| f.key == declared.id) {
        Some(present) => {
            if let Err(why) = crate::write::check_value(declared, &present.value) {
                findings.push(blocking_conformance(
                    "schema-conformance.field-value-conformant",
                    format!(
                        "field `{}` in item `{}` of section `{}`: {why}",
                        declared.id, item.id, section.id
                    ),
                    Some(Location::addressed(address, 1, 1)),
                ));
            }
        }
        None => {
            if is_author_required(declared) {
                findings.push(blocking_conformance(
                    "schema-conformance.required-field-present",
                    format!(
                        "required field `{}` is missing from item `{}` of section `{}`",
                        declared.id, item.id, section.id
                    ),
                    Some(Location::addressed(address, 1, 1)),
                ));
            }
        }
    }
}

/// The `section/item/leaf` address fragment for a repeatable-item finding (the M13
/// vocabulary; mirrors [`crate::target_surface`]'s per-item anchor address shape).
fn item_leaf_address(section: &Section, item: &ParsedItem, leaf_id: &str) -> String {
    format!("{}/{}/{}", section.id, item.id, leaf_id)
}

/// `required-slot-present`: the section declares a slot, so its prose must be
/// non-empty. The parser records the slot span even when empty (a present heading
/// with no prose under it), so an all-whitespace slice is the unfilled-slot case.
fn check_slot_present(
    section: &Section,
    parsed: &ParsedSection,
    source: &str,
    findings: &mut Vec<Finding>,
) {
    let filled = parsed
        .slot
        .as_ref()
        .map(|span| !span.slice(source).trim().is_empty())
        .unwrap_or(false);
    if !filled {
        findings.push(blocking_conformance(
            "schema-conformance.required-slot-present",
            format!("required slot in section `{}` is empty", section.id),
            slot_location(parsed),
        ));
    }
}

/// `required-field-present` + `field-value-conformant` for one declared field.
///
/// A field is **author-required** iff it carries neither a `default:` nor a `set:`
/// (those are CLI-derived / defaulted, never the author's obligation). An absent
/// author-required field blocks; a present field whose value fails its declared type
/// (via [`crate::write::check_value`]) blocks. A present field is *not* also reported
/// absent, and an absent field is *not* value-checked.
fn check_field(
    section: &Section,
    declared: &SchemaField,
    parsed: &ParsedSection,
    findings: &mut Vec<Finding>,
) {
    match parsed.fields.iter().find(|f| f.key == declared.id) {
        Some(present) => check_field_value(section, declared, present, findings),
        None => {
            if is_author_required(declared) {
                findings.push(blocking_conformance(
                    "schema-conformance.required-field-present",
                    format!(
                        "required field `{}` is missing from section `{}`",
                        declared.id, section.id
                    ),
                    None,
                ));
            }
        }
    }
}

/// `field-value-conformant`: a present field's value must pass its declared type.
fn check_field_value(
    section: &Section,
    declared: &SchemaField,
    present: &Field,
    findings: &mut Vec<Finding>,
) {
    if let Err(why) = crate::write::check_value(declared, &present.value) {
        findings.push(blocking_conformance(
            "schema-conformance.field-value-conformant",
            format!("field `{}` in section `{}`: {why}", declared.id, section.id),
            None,
        ));
    }
}

/// A field is author-required iff the author must supply it — no `default:`, no
/// `set:` (CLI-derived), not an **optional `ref`** (forward `card:` with a minimum
/// of 0, e.g. the ADR `supersedes` relation's `0..1`), and not a **pack-declared
/// field type** (e.g. `code-anchor`). An optional ref and a pack-declared anchor
/// both carry no author obligation: presence is the agent's choice and its
/// *adjudication* — not its presence — is the finalize-time probe's job (`ref-resolves`
/// for a ref, the type's bound adjudicator for a pack type, e.g. `code-anchor` → `doc-code`).
/// (`design/validation.md` → the synthetic `ref-resolves` check; `document-type-schema.md`
/// → `ref` cardinality + Pack-declared field types; `DECISIONS.md 2026-06-06` → M10 inc-1:
/// both shipped anchors are optional.)
fn is_author_required(field: &SchemaField) -> bool {
    if field.ty == FieldType::Ref && ref_min_cardinality_zero(field) {
        return false;
    }
    if matches!(field.ty, FieldType::Pack(_)) {
        return false;
    }
    // `owned-location` mirrors the pack-anchor exemption: its presence is the
    // agent's choice and its *adjudication* — path-safety + durable presence — is
    // the intrinsic finalize-time #5 owner-artifact gate, never a `required-field-
    // present` obligation here (`design/methodology-docs.md` → The engine work, item 3).
    if field.ty == FieldType::OwnedLocation {
        return false;
    }
    field.default.is_none() && field.set.is_none()
}

/// Whether a `ref` field's forward cardinality has a minimum of 0 (it is optional).
/// `card:` defaults to `"0..1"` when omitted (`schema.rs` → `SchemaField::card`); an
/// explicit form is optional iff it starts with `"0"` (`"0..1"` / `"0..*"`).
fn ref_min_cardinality_zero(field: &SchemaField) -> bool {
    match field.card.as_deref() {
        None => true,
        Some(card) => card.trim_start().starts_with('0'),
    }
}

/// The located coordinate for a slot finding: the slot span's start line when known.
fn slot_location(parsed: &ParsedSection) -> Option<Location> {
    parsed
        .slot
        .as_ref()
        .map(|span| Location::at(span.start_line, 1))
}

/// Build a blocking `schema-conformance.*` [`Finding`] (the inventory default
/// severity). These intrinsic checks carry no `route` — the agent fills the slot /
/// field directly.
fn blocking_conformance(code: &str, message: String, location: Option<Location>) -> Finding {
    Finding::graded(Severity::Blocking, code, message, location, None)
}

#[cfg(test)]
mod schema_conformance_tests {
    //! `schema-conformance.*` over a single parsed instance: required-slot-present,
    //! required-field-present, field-value-conformant. One blocking finding per
    //! violation; a conformant instance yields none.

    use super::*;
    use crate::parse::parse_sections;

    /// A purpose-built schema with the three things the checks adjudicate: a header
    /// with one author-required enum field (`kind`, no default/set) plus one
    /// CLI-defaulted enum field (`status`, with `default`), and one body slot section
    /// (`body`). Minimal — one of each lever, so a fixture can violate exactly one.
    fn schema() -> Schema {
        let yaml = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: kind, type: enum, of: [memo, brief] }
      - { id: status, type: enum, of: [open, closed], default: open }
  - id: body
    slot: { hint: \"The note body.\" }
";
        crate::schema::load_schema(yaml).expect("note schema loads")
    }

    /// Parse a fixture against the schema, panicking (with the parse findings) if it
    /// does not cleanly map — the conformance checks run *over a parsed instance*, so
    /// the fixture must parse first.
    fn parse(source: &str) -> Document {
        parse_sections(&schema(), source)
            .unwrap_or_else(|f| panic!("fixture must parse; got conformance findings: {f:?}"))
    }

    /// A fully-conformant instance: the slot is filled, both author-required fields
    /// present, every value type-conformant. Yields **zero** findings.
    const CONFORMANT: &str = "\
---
title: A note
kind: memo
status: open
---

# A note

## Body

The note body prose.
";

    /// Missing required slot: the `## Body` heading is present (so it parses) but the
    /// slot prose is empty. Exactly one `schema-conformance.required-slot-present`.
    const MISSING_SLOT: &str = "\
---
title: A note
kind: memo
status: open
---

# A note

## Body
";

    /// Missing required field: the author-required `kind` enum (no default) is absent
    /// from the front-matter. Exactly one `schema-conformance.required-field-present`.
    /// (`status` is absent too but is CLI-defaulted, so it is *not* author-required.)
    const MISSING_FIELD: &str = "\
---
title: A note
---

# A note

## Body

The note body prose.
";

    /// Malformed field value: `kind` is present but not an enum member. Exactly one
    /// `schema-conformance.field-value-conformant`.
    const MALFORMED_VALUE: &str = "\
---
title: A note
kind: wormhole
status: open
---

# A note

## Body

The note body prose.
";

    /// The clean fixture yields no findings; each minimal negative fixture yields
    /// exactly one matching blocking `schema-conformance.{…}` finding. The table is
    /// the done-criterion: one violation each, asserted by `code` + `severity`.
    #[test]
    fn schema_conformance_flags_missing_and_malformed() {
        let schema = schema();

        // The conformant instance: zero findings.
        let doc = parse(CONFORMANT);
        let findings = schema_conformance(&schema, CONFORMANT, &doc);
        assert!(
            findings.is_empty(),
            "a fully-conformant instance must yield no findings, got {findings:?}"
        );

        // Each negative fixture: exactly one finding with the expected code + severity.
        let cases: &[(&str, &str)] = &[
            (MISSING_SLOT, "schema-conformance.required-slot-present"),
            (MISSING_FIELD, "schema-conformance.required-field-present"),
            (MALFORMED_VALUE, "schema-conformance.field-value-conformant"),
        ];
        for (source, expected_code) in cases {
            let doc = parse(source);
            let findings = schema_conformance(&schema, source, &doc);
            assert_eq!(
                findings.len(),
                1,
                "fixture for {expected_code} must yield exactly one finding, got {findings:?}"
            );
            let finding = &findings[0];
            assert_eq!(finding.code, *expected_code, "finding code");
            assert_eq!(
                finding.severity,
                Severity::Blocking,
                "{expected_code} must be blocking"
            );
        }
    }

    /// A pack-declared field type (`code-anchor`) is **author-optional**: its
    /// presence is the agent's choice and its *adjudication* is the bound
    /// finalize-time probe (`doc-code`), exactly as an optional `ref`'s resolution
    /// is `ref-resolves`' job — never its presence. So an instance that **omits**
    /// the optional `cites-code` anchor yields **zero** `required-field-present`
    /// findings (M10 inc-1: both shipped anchors are optional;
    /// `DECISIONS.md 2026-06-06`). The red step proving the exemption: without it,
    /// the shipped `adr` (whose `cites-code` is always absent at create) would
    /// blocking-fail conformance on every task.
    #[test]
    fn an_omitted_pack_anchor_field_is_not_author_required() {
        let yaml = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: cites-code, type: code-anchor }
  - id: body
    slot: { hint: \"The note body.\" }
";
        let schema =
            crate::schema::load_schema_with_types(yaml, &crate::schema::dev_pack_field_types())
                .expect("note schema with a pack anchor loads");

        // No `cites-code` line in the front-matter — the optional anchor is absent.
        let source = "\
---
title: A note
---

# A note

## Body

The note body prose.
";
        let doc = parse_sections(&schema, source)
            .unwrap_or_else(|f| panic!("fixture must parse; got {f:?}"));
        let findings = schema_conformance(&schema, source, &doc);
        assert!(
            findings.is_empty(),
            "an omitted optional `code-anchor` field must yield no findings, got {findings:?}",
        );
    }

    /// (M16 inc-3 T1) The engine-native `owned-location` field is **exempt** from
    /// `schema-conformance`'s author-required + value-conformant checks, mirroring
    /// the pack-anchor exemption: its presence is the agent's choice and its
    /// *adjudication* (path-safety + presence) is the finalize-time #5 gate (T2),
    /// never `field-value-conformant`. So an instance is clean both ways: when the
    /// `owner-artifact` field is **omitted** (no `required-field-present`) and when
    /// it carries an arbitrary value (no `field-value-conformant`), regardless of
    /// that value. The red step proving recognition: without the native arm,
    /// `owned-location` would resolve to an unresolved `Pack` and `load_schema`
    /// (engine-empty set) would reject the schema with `UnknownFieldType`.
    #[test]
    fn an_owned_location_field_is_exempt_from_conformance() {
        let yaml = b"\
type: completion-record
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: owner-artifact, type: owned-location }
  - id: body
    slot: { hint: \"The record body.\" }
";
        // Loads with the engine-empty type set — `owned-location` is engine-native.
        let schema = crate::schema::load_schema(yaml)
            .expect("completion-record schema with owned-location loads");

        // Omitted: no `owner-artifact` line — must not fire `required-field-present`.
        let omitted = "\
---
title: A record
---

# A record

## Body

The record body prose.
";
        let doc = parse_sections(&schema, omitted)
            .unwrap_or_else(|f| panic!("fixture must parse; got {f:?}"));
        let findings = schema_conformance(&schema, omitted, &doc);
        assert!(
            findings.is_empty(),
            "an omitted `owned-location` field must yield no findings, got {findings:?}",
        );

        // Present with an arbitrary value — must not fire `field-value-conformant`
        // (recognition only; path-safety/presence is the T2 finalize-time gate).
        let present = "\
---
title: A record
owner-artifact: ../wildly/unsafe/../path
---

# A record

## Body

The record body prose.
";
        let doc = parse_sections(&schema, present)
            .unwrap_or_else(|f| panic!("fixture must parse; got {f:?}"));
        let findings = schema_conformance(&schema, present, &doc);
        assert!(
            findings.is_empty(),
            "a present `owned-location` value must not fire field-value-conformant, got {findings:?}",
        );
    }
}

#[cfg(test)]
mod repeatable_conformance_tests {
    //! `schema-conformance.*` over a **repeatable** section's items: per item, the
    //! three synthetic checks run over the block leaves (`required-slot-present`,
    //! `required-field-present`, `field-value-conformant`), with the `id-from`
    //! heading field exempted (it is the heading text, never a trailing bullet, so
    //! it must not false-fail as a missing required field). Findings address at
    //! `#section/item/leaf`. A conformant repeatable instance yields none; a
    //! hand-malformed entry fires a blocking finding (the M16 inc-1 red obligation).

    use super::*;
    use crate::parse::parse_sections;

    /// A purpose-built repeatable doctype (a test fixture, not pack content — no
    /// shipped doctype carries a required field inside a repeatable item). The
    /// `entries` block carries the three levers: the `id-from: title` heading field
    /// (exempted), a required `detail` slot, and an author-required `kind` enum.
    fn schema() -> Schema {
        let yaml = b"\
type: note
id-from: title
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: kind, type: enum, of: [memo, brief] }
        - { id: detail, slot: { hint: \"The entry detail.\" } }
";
        crate::schema::load_schema(yaml).expect("note repeatable schema loads")
    }

    fn parse(source: &str) -> Document {
        parse_sections(&schema(), source)
            .unwrap_or_else(|f| panic!("fixture must parse; got conformance findings: {f:?}"))
    }

    /// A fully-conformant repeatable instance: one item whose detail slot is filled,
    /// whose `kind` enum is present and a member, and whose `title` (the id-source
    /// heading) carries no trailing bullet. Yields **zero** findings — the id-from
    /// heading field must NOT be flagged `required-field-present` (the exemption trap).
    const CONFORMANT: &str = "\
---
---

# A note

## Entries

### First entry  {#first-entry}

The first entry's detail prose.

<!-- fields -->
- kind: memo
";

    /// A repeatable entry with an **empty required slot**: the `### …` item heading
    /// parses, but its `detail` slot prose is blank. Exactly one blocking
    /// `schema-conformance.required-slot-present`, addressed at `#entries/first-entry/detail`.
    const EMPTY_SLOT: &str = "\
---
---

# A note

## Entries

### First entry  {#first-entry}

<!-- fields -->
- kind: memo
";

    /// A repeatable entry with a **malformed required field value**: `kind` is present
    /// but not an enum member. Exactly one blocking
    /// `schema-conformance.field-value-conformant`, addressed at `#entries/first-entry/kind`.
    const MALFORMED_VALUE: &str = "\
---
---

# A note

## Entries

### First entry  {#first-entry}

The first entry's detail prose.

<!-- fields -->
- kind: wormhole
";

    /// (a) A fully-conformant repeatable instance yields ZERO findings — proving the
    /// id-from heading field is not spuriously flagged `required-field-present` (the
    /// exemption trap). The load-bearing assumption the planner flagged.
    #[test]
    fn conformant_repeatable_instance_yields_no_findings() {
        let schema = schema();
        let doc = parse(CONFORMANT);
        let findings = schema_conformance(&schema, CONFORMANT, &doc);
        assert!(
            findings.is_empty(),
            "a fully-conformant repeatable instance must yield no findings, got {findings:?}"
        );
    }

    /// (b) The red obligation FIRES: a hand-malformed entry with an empty required
    /// slot produces a blocking `schema-conformance.required-slot-present`, and a
    /// separate entry with a malformed required field value produces a blocking
    /// `schema-conformance.field-value-conformant`. Each addressed at `#section/item/leaf`.
    #[test]
    fn malformed_repeatable_entry_fires_a_blocking_finding() {
        let schema = schema();

        let cases: &[(&str, &str, &str)] = &[
            (
                EMPTY_SLOT,
                "schema-conformance.required-slot-present",
                "entries/first-entry/detail",
            ),
            (
                MALFORMED_VALUE,
                "schema-conformance.field-value-conformant",
                "entries/first-entry/kind",
            ),
        ];
        for (source, expected_code, expected_address) in cases {
            let doc = parse(source);
            let findings = schema_conformance(&schema, source, &doc);
            assert_eq!(
                findings.len(),
                1,
                "fixture for {expected_code} must yield exactly one finding, got {findings:?}"
            );
            let finding = &findings[0];
            assert_eq!(finding.code, *expected_code, "finding code");
            assert_eq!(
                finding.severity,
                Severity::Blocking,
                "{expected_code} must be blocking"
            );
            let address = finding
                .location
                .as_ref()
                .and_then(|l| l.address.as_deref())
                .unwrap_or_else(|| panic!("{expected_code} must carry an address"));
            assert_eq!(
                address, *expected_address,
                "{expected_code} must address at #section/item/leaf"
            );
        }
    }

    /// A missing author-required field inside an item fires `required-field-present`,
    /// addressed at `#section/item/leaf` — and the id-from heading field, absent as a
    /// bullet by construction, is exempted (else every conformant item false-fails).
    #[test]
    fn missing_required_field_in_an_item_fires() {
        let schema = schema();
        let source = "\
---
---

# A note

## Entries

### First entry  {#first-entry}

The first entry's detail prose.
";
        let doc = parse(source);
        let findings = schema_conformance(&schema, source, &doc);
        assert_eq!(
            findings.len(),
            1,
            "a missing author-required item field must yield exactly one finding, got {findings:?}"
        );
        assert_eq!(
            findings[0].code,
            "schema-conformance.required-field-present"
        );
        assert_eq!(findings[0].severity, Severity::Blocking);
        assert_eq!(
            findings[0]
                .location
                .as_ref()
                .and_then(|l| l.address.as_deref()),
            Some("entries/first-entry/kind"),
        );
    }
}

#[cfg(test)]
mod validate_task_tests {
    //! The task-scope sweep: `validate_task` resolves a working area's staged doc
    //! instances, runs `file-state` + `schema-conformance` over them, and aggregates
    //! the severity-classified findings into one report exposing `has_blocking()`.

    use super::*;
    use crate::file_state::hash_bytes;
    use std::path::PathBuf;

    /// A throwaway working-area root that removes itself on drop.
    struct TempArea(PathBuf);

    impl TempArea {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-validate-task-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
            std::fs::create_dir_all(path.join(DOCS_DIR)).expect("create docs dir");
            TempArea(path)
        }

        fn dir(&self) -> &Path {
            &self.0
        }

        /// Stage a doc instance at `docs/<filename>` and return its docs-relative
        /// path key (the form the `file-state` record is keyed by).
        fn stage(&self, filename: &str, bytes: &[u8]) -> String {
            let rel = format!("{DOCS_DIR}/{filename}");
            std::fs::write(self.0.join(&rel), bytes).expect("stage instance");
            rel
        }
    }

    impl Drop for TempArea {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The `note` schema used by inc-4's conformance fixtures: one author-required
    /// enum field plus one body slot — enough to make an instance conformance-broken
    /// by leaving the slot empty.
    fn note_schema() -> Schema {
        let yaml = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: kind, type: enum, of: [memo, brief] }
  - id: body
    slot: { hint: \"The note body.\" }
";
        crate::schema::load_schema(yaml).expect("note schema loads")
    }

    /// A fully-conformant `note` instance: slot filled, required field present.
    const CONFORMANT: &str = "\
---
title: A note
kind: memo
---

# A note

## Body

The note body prose.
";

    /// A conformance-broken `note` instance: the `## Body` heading parses but the
    /// slot is empty — one `schema-conformance.required-slot-present`.
    const BROKEN: &str = "\
---
title: A note
kind: memo
---

# A note

## Body
";

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert("note".to_string(), note_schema());
        m
    }

    /// A no-delta resolved cascade — the post-pass leaves every emitted severity
    /// untouched, so these sweeps assert the byte-identical no-override path.
    fn no_delta_resolved() -> crate::cascade::Resolved {
        crate::cascade::resolve(
            &crate::cascade::PackDefaultLayer::new(
                "dev-pack",
                "0.1.0",
                BTreeMap::new(),
                Vec::new(),
            ),
            None,
            None,
        )
        .expect("resolves")
    }

    /// An invoker that **must not run** — these fixtures (the `note` schema) carry no
    /// `code-anchor`, so the target surface is empty and the doc-code block is skipped
    /// before any invocation (the omitting-context inert path, hardening #5). A call
    /// here would be a scope bug, so it panics.
    fn unused_invoker() -> impl Fn(&ProbeRequest) -> std::io::Result<ProbeRun> {
        |_req| panic!("doc-code invoker must not run when the surface is empty")
    }

    /// A tracked-predicate for fixtures carrying **no** `owned-location` field — the #5
    /// owner-artifact gate never consults it (the omitting-context inert path), so its
    /// answer is irrelevant; `false` is the inert default.
    fn never_tracked() -> impl Fn(&str) -> bool {
        |_path| false
    }

    /// The done-criterion. Over a working area with **one drifted file** and **one
    /// conformance-broken instance**, `validate_task` returns *both* findings and
    /// `has_blocking() == true`; over a **clean** area it returns an empty report and
    /// `has_blocking() == false`.
    #[test]
    fn validate_task_aggregates_probe_findings() {
        // --- The broken area: a drifted instance + a conformance-broken instance.
        let area = TempArea::new("broken");

        // `note:drift.md` was committed with the conformant bytes (its hash is in the
        // record), but the working area now holds *different* bytes → file-state drift.
        let drift_rel = area.stage("note:drift.md", BROKEN.as_bytes());
        let mut record = FileStateRecord::new();
        record.record(drift_rel.clone(), hash_bytes(CONFORMANT.as_bytes()));

        // `note:broken.md` is a fresh instance (no recorded hash → baseline-adopt,
        // advisory) whose slot is empty → schema-conformance blocks.
        area.stage("note:broken.md", BROKEN.as_bytes());

        let report = validate_task(
            area.dir(),
            &schemas(),
            &mut record,
            area.dir(),
            area.dir(),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
        )
        .expect("sweep runs");

        // Both blocking findings are present in the aggregate.
        let codes: Vec<&str> = report.findings.iter().map(|f| f.code.as_str()).collect();
        assert!(
            codes.contains(&"file-state.hash-matches"),
            "the drifted file must surface a file-state drift finding, got {codes:?}"
        );
        assert!(
            codes.contains(&"schema-conformance.required-slot-present"),
            "the broken instance must surface a conformance finding, got {codes:?}"
        );
        assert!(
            report.has_blocking(),
            "a drifted + conformance-broken area must block, got {:?}",
            report.findings
        );

        // --- The clean area: a single conformant instance, freshly baselined.
        let clean = TempArea::new("clean");
        clean.stage("note:ok.md", CONFORMANT.as_bytes());
        let mut clean_record = FileStateRecord::new();

        let clean_report = validate_task(
            clean.dir(),
            &schemas(),
            &mut clean_record,
            clean.dir(),
            clean.dir(),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
        )
        .expect("clean sweep runs");

        // A fresh file baselines (advisory, non-blocking) and conforms → no blocker.
        assert!(
            !clean_report.has_blocking(),
            "a clean area must not block, got {:?}",
            clean_report.findings
        );
        assert!(
            clean_report
                .findings
                .iter()
                .all(|f| f.severity != Severity::Blocking),
            "no blocking findings over a clean area, got {:?}",
            clean_report.findings
        );
    }
}

#[cfg(test)]
mod ref_resolves_in_sweep_tests {
    //! The `schema-conformance.ref-resolves` forward-ref / edge-index integrity check
    //! is now part of the `validate_task` sweep, so `finalize` (which gates on exactly
    //! what `validate` reports) blocks on a dangling `supersedes` and passes on a
    //! resolvable one (`validation.md` → Forward-ref resolution; `worked-examples.md` →
    //! Superseding decision, the `[2 validate]` line and its dangling variant).

    use super::*;
    use std::path::PathBuf;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-validate-refresolves-{tag}-{}-{:?}",
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

    /// A no-delta resolved cascade — the post-pass is a no-op, so these sweeps
    /// exercise the byte-identical no-override path.
    fn no_delta_resolved() -> crate::cascade::Resolved {
        crate::cascade::resolve(
            &crate::cascade::PackDefaultLayer::new(
                "dev-pack",
                "0.1.0",
                BTreeMap::new(),
                Vec::new(),
            ),
            None,
            None,
        )
        .expect("resolves")
    }

    /// An invoker that **must not run** — these ADR fixtures carry no `cites-code`
    /// anchor, so the doc-code surface is empty and the block is skipped (the inert
    /// path). A call would be a scope bug, so it panics.
    fn unused_invoker() -> impl Fn(&ProbeRequest) -> std::io::Result<ProbeRun> {
        |_req| panic!("doc-code invoker must not run when the surface is empty")
    }

    /// A tracked-predicate for the ADR fixtures (no `owned-location` field) — the #5
    /// owner-artifact gate never consults it; `false` is the inert default.
    fn never_tracked() -> impl Fn(&str) -> bool {
        |_path| false
    }

    /// A committed ADR `A` (the supersede target), with no outgoing ref. Its required
    /// slots are filled so the committed file parses cleanly.
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

    /// A working-area ADR `B` whose `supersedes` points at `to`. Its required slots are
    /// filled so the *only* possible blocking finding is the ref-resolves one.
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

    /// The dangling variant (`worked-examples.md` → The dangling variant): a staged ADR
    /// supersedes `adr:typo-nonexistent`, which exists in neither the committed store
    /// nor this task's working area. `validate_task` must aggregate the
    /// `schema-conformance.ref-resolves` blocking finding so `finalize` blocks.
    #[test]
    fn validate_task_blocks_on_dangling_supersedes() {
        let repo = TempRoot::new("dangle-repo");
        let jigc = repo.path().join(".jigc");
        let task_dir = jigc.join("tasks").join("supersede-cache");
        commit_adr_a(repo.path());
        stage_adr_b(
            &task_dir,
            "shared-redis-session-cache",
            "adr:typo-nonexistent",
        );

        let mut record = FileStateRecord::new();
        let report = validate_task(
            &task_dir,
            &schemas(),
            &mut record,
            repo.path(),
            &jigc,
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
        )
        .expect("sweep runs");

        let refresolves: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "schema-conformance.ref-resolves")
            .collect();
        assert_eq!(
            refresolves.len(),
            1,
            "exactly one ref-resolves finding for the dangling supersedes, got {:?}",
            report.findings
        );
        assert!(
            refresolves[0].message.contains("adr:typo-nonexistent"),
            "the finding names the dangling target: {}",
            refresolves[0].message
        );
        assert!(
            report.has_blocking(),
            "a dangling forward-ref must make the task block, got {:?}",
            report.findings
        );
    }

    /// The resolvable case (`worked-examples.md` → `[2 validate] forward-ref … ✓
    /// (committed store)`): a staged ADR supersedes a committed target, so the
    /// ref-resolves walk finds it in the committed store — `validate_task` surfaces no
    /// `ref-resolves` finding and does not block on it.
    #[test]
    fn validate_task_passes_on_resolvable_supersedes() {
        let repo = TempRoot::new("resolve-repo");
        let jigc = repo.path().join(".jigc");
        let task_dir = jigc.join("tasks").join("supersede-cache");
        commit_adr_a(repo.path());
        stage_adr_b(
            &task_dir,
            "shared-redis-session-cache",
            "adr:single-node-cache",
        );

        let mut record = FileStateRecord::new();
        let report = validate_task(
            &task_dir,
            &schemas(),
            &mut record,
            repo.path(),
            &jigc,
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
        )
        .expect("sweep runs");

        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.code == "schema-conformance.ref-resolves"),
            "a resolvable supersedes yields no ref-resolves finding, got {:?}",
            report.findings
        );
        assert!(
            !report.has_blocking(),
            "a clean task with a resolvable supersedes must not block, got {:?}",
            report.findings
        );
    }
}

#[cfg(test)]
mod owner_artifact_gate_tests {
    //! (M16 inc-3 T2) The intrinsic #5 **owner-artifact presence gate** over a FIXTURE
    //! `completion-record` doctype carrying an engine-native `owned-location` field
    //! (`design/methodology-docs.md` → The engine work, item 3). The gate **fires**
    //! (one blocking `owner-artifact.present`) on each unsafe / absent / untracked path
    //! and is **silent** on a path durably staged under `completions/artifacts/<milestone>/`,
    //! so a `finalize` (gating on exactly what `validate_task` reports — `plan_finalize`
    //! phase 2) BLOCKS on the absent case and LANDS on the staged case (one engine, two
    //! entry points). An instance that **omits** the field is inert (the omitting-context
    //! guard) — paired with the firing cases so a green pass can't hide a scope bug.
    //!
    //! Path-safety + presence + tracked are proven over a real temp repo; the gate reads
    //! the field string + the file's presence + the tracked flag, **never the artifact's
    //! bytes** (the determinism boundary — presence, not content).

    use super::*;
    use crate::file_state::FileStateRecord;
    use std::path::PathBuf;

    /// A throwaway repo root that removes itself on drop.
    struct TempRepo(PathBuf);

    impl TempRepo {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-owner-artifact-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp repo");
            TempRepo(path)
        }
        fn path(&self) -> &Path {
            &self.0
        }
        /// Materialize a file at `rel` (repo-relative), creating parent dirs.
        fn write(&self, rel: &str, bytes: &[u8]) {
            let full = self.0.join(rel);
            std::fs::create_dir_all(full.parent().unwrap()).expect("mk parents");
            std::fs::write(full, bytes).expect("write file");
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The fixture `completion-record` doctype: a `meta` header carrying an
    /// engine-native `owned-location` `owner-artifact` field (the #5 gate target) plus a
    /// body slot. A FIXTURE, never pack content — the real completion-record rides this
    /// gate in inc-5.
    fn schemas() -> BTreeMap<String, Schema> {
        let yaml = b"\
type: completion-record
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: owner-artifact, type: owned-location }
  - id: body
    slot: { hint: \"The record body.\" }
";
        let mut m = BTreeMap::new();
        m.insert(
            "completion-record".to_string(),
            crate::schema::load_schema(yaml).expect("completion-record fixture loads"),
        );
        m
    }

    /// A fixture completion-record naming `owner-artifact: <path>`; its required slot is
    /// filled so the *only* possible blocking finding is the owner-artifact one.
    fn record_with_owner_artifact(path: &str) -> String {
        format!(
            "\
---
title: M16 completion
owner-artifact: {path}
---

# M16 completion

## Body

The audit landed green.
"
        )
    }

    /// A fixture completion-record that **omits** the `owner-artifact` field entirely
    /// (the omitting context). Its required slot is filled.
    const RECORD_NO_OWNER_ARTIFACT: &str = "\
---
title: M16 completion
---

# M16 completion

## Body

The audit landed green.
";

    /// Run the gate over a single parsed completion-record instance with the given path
    /// + tracked predicate, over `repo_root`.
    fn gate(repo_root: &Path, source: &str, tracked: &TrackedPredicate<'_>) -> Vec<Finding> {
        owner_artifact_present(
            "completion-record:m16.md",
            &schemas(),
            "docs/completion-record:m16.md",
            source,
            repo_root,
            tracked,
        )
    }

    /// A tracked predicate that tracks every path (so a present-but-safe artifact is the
    /// pass case — only path-safety / presence can fire).
    fn always_tracked() -> impl Fn(&str) -> bool {
        |_p| true
    }

    /// (RED — fires) Each unsafe / absent / untracked owner-artifact path produces
    /// **exactly one** blocking `owner-artifact.present` finding. The cases span every
    /// bar the design names: absent-path (the file is missing), absolute, `..`-containing,
    /// not-under-home (the anti-vacuity bar — a real, tracked, present file that is NOT a
    /// durable owner-artifact), out-of-repo (an absolute escape), and present-but-untracked.
    #[test]
    fn gate_fires_on_each_unsafe_or_absent_or_untracked_path() {
        let repo = TempRepo::new("fires");
        // A real, tracked, present file OUTSIDE the owned home — the anti-vacuity case:
        // a bare exists() would pass it; the under-home constraint must reject it.
        repo.write("README.md", b"# readme\n");
        // A present + (claimed-)tracked artifact UNDER the home — the untracked case
        // flips its tracked flag to false; the absent case points elsewhere.
        repo.write("completions/artifacts/M16/audit.md", b"audit transcript\n");

        // (path, tracked-predicate, label) — each must yield exactly one finding.
        let cases: Vec<(&str, Box<TrackedPredicate>, &str)> = vec![
            (
                // absent-path: a well-shaped home path naming a file that does not exist.
                "completions/artifacts/M16/missing.md",
                Box::new(|_p: &str| true),
                "absent path",
            ),
            ("/etc/passwd", Box::new(|_p: &str| true), "absolute path"),
            (
                "completions/artifacts/M16/../../etc/passwd",
                Box::new(|_p: &str| true),
                "`..`-containing path",
            ),
            (
                // not under the owned home, though present + tracked (anti-vacuity).
                "README.md",
                Box::new(|_p: &str| true),
                "not under owned home",
            ),
            (
                // present under the home but UNTRACKED.
                "completions/artifacts/M16/audit.md",
                Box::new(|_p: &str| false),
                "present-but-untracked",
            ),
        ];

        for (path, tracked, label) in cases {
            let source = record_with_owner_artifact(path);
            let findings = gate(repo.path(), &source, tracked.as_ref());
            assert_eq!(
                findings.len(),
                1,
                "the gate must fire exactly once for the {label} case ({path}), got {findings:?}"
            );
            let f = &findings[0];
            assert_eq!(f.code, "owner-artifact.present", "{label}: finding code");
            assert_eq!(f.severity, Severity::Blocking, "{label}: must block");
            assert_eq!(
                f.location.as_ref().and_then(|l| l.address.as_deref()),
                Some("docs/completion-record:m16.md#meta/owner-artifact"),
                "{label}: the finding addresses the owner-artifact field",
            );
        }
    }

    /// (GREEN — passes) A path durably staged under `completions/artifacts/<milestone>/`
    /// (present + tracked, repo-relative, `..`-free) yields **zero** findings — the gate
    /// is silent on a real recorded owner-artifact.
    #[test]
    fn gate_passes_on_a_durably_staged_owned_artifact() {
        let repo = TempRepo::new("passes");
        repo.write(
            "completions/artifacts/M16/audit.md",
            b"the genuine audit transcript\n",
        );
        let source = record_with_owner_artifact("completions/artifacts/M16/audit.md");
        let findings = gate(repo.path(), &source, &always_tracked());
        assert!(
            findings.is_empty(),
            "a durably-staged owned-location artifact must yield no finding, got {findings:?}"
        );
    }

    /// (Inert — the omitting-context guard) A completion-record that omits the
    /// `owner-artifact` field consults nothing and yields zero findings — byte-identical
    /// to the pre-gate sweep. Paired with the firing cases so a green pass over a single
    /// composing context can't hide a scope bug in every omitting context.
    #[test]
    fn gate_is_inert_when_the_field_is_omitted() {
        let repo = TempRepo::new("inert");
        // A tracked predicate that PANICS if consulted — the omitting context must never
        // reach the tracked check (or any path adjudication).
        let must_not_run: Box<TrackedPredicate> =
            Box::new(|_p: &str| panic!("tracked must not be consulted when the field is omitted"));
        let findings = gate(repo.path(), RECORD_NO_OWNER_ARTIFACT, must_not_run.as_ref());
        assert!(
            findings.is_empty(),
            "an omitted owner-artifact field must yield no finding, got {findings:?}"
        );
    }

    /// (Symlink-escape — fires) An `owner-artifact` path under the owned home that is a
    /// **symlink pointing outside** the home resolves out of the artifact home; the gate
    /// rejects it (the real path must stay under `<repo>/completions/artifacts/`). Unix-only
    /// (symlink creation), the platform the build + CI runs on.
    #[cfg(unix)]
    #[test]
    fn gate_fires_on_a_symlink_escape() {
        let repo = TempRepo::new("symlink");
        // A real file OUTSIDE the home, and a symlink under the home pointing at it.
        repo.write("outside/secret.md", b"out of the owned home\n");
        std::fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk home");
        std::os::unix::fs::symlink(
            repo.path().join("outside/secret.md"),
            repo.path().join("completions/artifacts/M16/escape.md"),
        )
        .expect("make escaping symlink");

        let source = record_with_owner_artifact("completions/artifacts/M16/escape.md");
        // Tracked = true so the ONLY lever is the symlink-escape check.
        let findings = gate(repo.path(), &source, &always_tracked());
        assert_eq!(
            findings.len(),
            1,
            "a symlink escaping the owned home must fire exactly once, got {findings:?}"
        );
        assert_eq!(findings[0].code, "owner-artifact.present");
        assert!(
            findings[0].message.contains("symlink"),
            "the finding names the symlink escape: {}",
            findings[0].message
        );
    }

    /// A no-delta resolved cascade — `owner-artifact.present` is not an inventory row, so
    /// the post-pass is doubly inert; this confirms the emitted blocking severity survives.
    fn no_delta_resolved() -> crate::cascade::Resolved {
        crate::cascade::resolve(
            &crate::cascade::PackDefaultLayer::new(
                "dev-pack",
                "0.1.0",
                BTreeMap::new(),
                Vec::new(),
            ),
            None,
            None,
        )
        .expect("resolves")
    }

    /// The doc-code invoker must not run (the completion-record carries no `code-anchor`).
    fn unused_invoker() -> impl Fn(&ProbeRequest) -> std::io::Result<ProbeRun> {
        |_req| panic!("doc-code invoker must not run when the surface is empty")
    }

    /// Stage the fixture completion-record at `<task_dir>/docs/completion-record:m16.md`.
    fn stage_record(task_dir: &Path, source: &str) {
        let docs = task_dir.join(DOCS_DIR);
        std::fs::create_dir_all(&docs).expect("mk docs/");
        std::fs::write(docs.join("completion-record:m16.md"), source).expect("stage record");
    }

    /// (Integration — one engine, two entry points) The gate's finding flows through
    /// `validate_task` into `plan_finalize` phase 2: a completion-record naming an
    /// **absent** owner-artifact makes `validate_task` report a blocker, so `plan_finalize`
    /// BLOCKS; the same task with the artifact durably staged + tracked yields no blocker,
    /// so `plan_finalize` proceeds past phase 2 (it LANDS — the gate does not stop it).
    #[test]
    fn finalize_blocks_on_absent_owner_artifact_and_lands_on_staged() {
        use crate::finalize::plan_finalize;
        use crate::state::BasePin;

        // --- The absent case: validate_task reports the owner-artifact blocker.
        let repo = TempRepo::new("finalize-absent");
        let task_dir = repo.path().join(".jigc").join("tasks").join("complete-m16");
        stage_record(
            &task_dir,
            &record_with_owner_artifact("completions/artifacts/M16/missing.md"),
        );
        let mut record = FileStateRecord::new();
        let report = validate_task(
            &task_dir,
            &schemas(),
            &mut record,
            repo.path(),
            &repo.path().join(".jigc"),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &|_p| true, // tracked is irrelevant — the file is absent.
        )
        .expect("validate runs");
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.code == "owner-artifact.present" && f.severity == Severity::Blocking),
            "the absent owner-artifact must surface a blocking finding, got {:?}",
            report.findings
        );
        assert!(report.has_blocking(), "the absent case must block validate");

        // plan_finalize phase 2 aborts on the report's blocking findings (it needs no
        // commit doc — phase 2 precedes the render). Base == HEAD so the preflight passes.
        let base = BasePin::new("HEAD", "HEAD");
        let plan = plan_finalize(
            &task_dir,
            &base,
            "HEAD",
            &report,
            true,
            schemas().get("completion-record").unwrap(),
            "m16",
            &schemas(),
        );
        let Err(findings) = plan else {
            panic!("finalize must BLOCK on the absent owner-artifact, got a plan");
        };
        assert!(
            findings.iter().any(|f| f.code == "owner-artifact.present"),
            "the finalize block surfaces the owner-artifact finding, got {findings:?}"
        );

        // --- The staged case: the artifact is present + tracked → no owner-artifact blocker.
        let repo = TempRepo::new("finalize-staged");
        let task_dir = repo.path().join(".jigc").join("tasks").join("complete-m16");
        repo.write(
            "completions/artifacts/M16/audit.md",
            b"the genuine audit transcript\n",
        );
        stage_record(
            &task_dir,
            &record_with_owner_artifact("completions/artifacts/M16/audit.md"),
        );
        let mut record = FileStateRecord::new();
        let report = validate_task(
            &task_dir,
            &schemas(),
            &mut record,
            repo.path(),
            &repo.path().join(".jigc"),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &|_p| true, // the artifact is tracked.
        )
        .expect("validate runs");
        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.code == "owner-artifact.present"),
            "a staged + tracked owner-artifact yields no owner-artifact finding, got {:?}",
            report.findings
        );
        assert!(
            !report.has_blocking(),
            "the staged case must not block validate, got {:?}",
            report.findings
        );
    }
}
