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
use crate::target_surface::{enumerate_committed_surface, enumerate_target_surface};
use std::collections::{BTreeMap, BTreeSet};
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

/// The snapshot the **base** (HEAD) sweep of the newly-dangled comparison materializes —
/// distinct from [`SNAPSHOT_FILE`] so the index and base probes never clobber each other's
/// snapshot. Same gitignored working area; never committed.
const BASE_SNAPSHOT_FILE: &str = "base-probe-snapshot.json";

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
///
/// `code_tree_root` is the root the `doc-code` probe resolves cited **code** anchors
/// against — split from `repo_root` (M30 Inc 3, G4): the CLI's shared `validate` entry
/// passes the **materialized git index** (a temp checkout of the staged set) for both
/// `task validate` and `finalize`, so a cited symbol present on disk but **absent from
/// the index** blocks ("validated reality == committed reality"). Every other surface
/// (the committed-store reads, the edge index, the #5 owner-artifact gate) stays on
/// `repo_root`; only the doc-code snapshot's `working_tree_root` rides `code_tree_root`.
/// Engine unit tests pass `repo_root` for it (no index to materialize against).
///
/// `changed_code` is the task's **staged code change-set** — the repo-relative paths
/// `git diff --cached --name-only` reports (the CLI shells out; the engine stays
/// shell-free). It drives the **code-anchor blast radius** (the universal `finalize`
/// floor, `validation.md` → Scope = effective state): every committed doc's
/// `code-anchor` whose **target file is in this set** is re-resolved and **blocks** if
/// it has *newly* dangled — so a task that renames a symbol cannot commit a dangling
/// citation in a committed doc it never opened, *under any workflow* (the `doc→code`
/// analog of the inbound-edge blast-radius walk: "a task can't commit breakage
/// elsewhere"). Change-set scoping covers the right set because resolution is
/// **path-local** (`p#s` is a pure function of `p`'s bytes — see [`anchor_file`]) for
/// every shipped grammar over real files: a committed `p#s`'s resolution can change only
/// if `p` changed. (Two honest bounds: a non-local future resolver — imports / re-exports
/// / build config — would under-cover, guarded by `path_locality_*`; and a tracked
/// **symlink** anchor file breaks path-locality, so the probe advises rather than follows
/// it.) An **empty** set is the byte-identical pre-floor path (no committed anchor is in
/// scope), so a task touching no code is never blocked. `base_code_tree_root` is what
/// keeps this from re-attributing *pre-existing* drift: see its paragraph below.
///
/// `base_code_tree_root` is the **HEAD** code tree (the CLI materializes the changed
/// files' HEAD versions) the blast radius re-resolves committed anchors against to decide
/// *newly*-dangled. A blast anchor blocks only if it **resolved at base AND dangles at
/// `code_tree_root`** — so drift that was already present at HEAD (e.g. a prior
/// out-of-band edit) in a file this task merely touches is **not** attributed to it. This
/// is the cross-task-coupling guard the integrity/completeness split requires; without it,
/// scoping-by-file alone would wedge an unrelated task on pre-existing drift. Unused when
/// `changed_code` is empty (no blast set); engine unit tests with no blast pass
/// `code_tree_root` for it.
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
    code_tree_root: &Path,
    jigc_root: &Path,
    head: &str,
    resolved: &crate::cascade::Resolved,
    invoke_doc_code: &ProbeInvoker<'_>,
    tracked: &TrackedPredicate<'_>,
    changed_code: &BTreeSet<String>,
    base_code_tree_root: &Path,
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
    // probe). The engine enumerates the task's effective-state `code-anchor` leaves
    // **plus the code-anchor blast radius** (committed anchors whose target file is in
    // `changed_code` — the universal `finalize` floor), materializes one serializable
    // snapshot, builds the wire request, and ingests the probe's findings; the
    // CLI-supplied invoker runs the subprocess (the engine stays shell-free). A task
    // whose merged surface carries **no** anchor produces zero pairs, so the whole block
    // is skipped — the invoker is never called and no snapshot is written, leaving the
    // no-anchor path byte-identical to the pre-wiring sweep (the omitting-context guard,
    // hardening #5).
    findings.extend(schedule_doc_code(
        dir,
        repo_root,
        code_tree_root,
        base_code_tree_root,
        schemas,
        changed_code,
        invoke_doc_code,
    )?);

    // Severity assignment is the engine-owned post-pass at report construction
    // (`validation.md` → Severity assignment — the M6 post-pass): `resolved` is the
    // cascade the caller resolved, read per-finding by inventory `(probe, check)`
    // membership. A no-delta cascade leaves every emitted severity untouched.
    Ok(ValidationReport::new(findings, resolved))
}

/// The filename the engine materializes the store-sweep snapshot under — distinct
/// from [`SNAPSHOT_FILE`] (which rides the task working area) because the store sweep is
/// **task-less**: there is no working area, so the snapshot goes to a fresh **temp-dir
/// scratch** path (`validation.md` → Store-scope re-validation: the scratch path is a
/// temp dir, **never** under a managed `location:`). The engine writes it, drives the
/// probe over it, and removes it before returning — it is never committed and never
/// observed by anything but the one probe invocation.
const STORE_SNAPSHOT_FILE: &str = "store-probe-snapshot.json";

/// Re-validate the **committed store's** doc↔code surface — the engine entry point the
/// top-level `jigc validate` drives (`validation.md` → Store-scope re-validation). It is
/// the **store-scope twin** of [`schedule_doc_code`] wrapped in a [`ValidationReport`]:
/// **task-less and read-only** (no working area, no `FileStateRecord`, no edge index).
///
/// It enumerates every committed doc's `code-anchor` leaves over every active schema
/// `location:` ([`enumerate_committed_surface`]), materializes the serializable
/// [`EffectiveStateSnapshot`] (`working_tree_root = repo_root`) to a **temp-dir scratch**
/// path (never under a managed `location:`), drives the CLI-supplied subprocess
/// `invoke_doc_code` seam over it, and ingests the probe's findings + any
/// `pack-probe-integrity.*` meta-findings into the report. The engine stays shell-free —
/// the invoke step (the only thing that shells out) is the CLI's, exactly as at task
/// scope.
///
/// **Reuses the M10 spine wholesale** — `enumerate_committed_surface` (inc-1),
/// [`EffectiveStateSnapshot::new`], [`ProbeRequest::new`], and [`ingest_probe_run`] are
/// target-agnostic: the store sweep differs from the task sweep only in *which surface it
/// enumerates*, not in how it requests/ingests. One probe invocation covers the whole
/// store; the wire request names a representative `target` (the first address-sorted
/// anchor) but the probe reads the full set from the snapshot.
///
/// **Severity is the engine-owned post-pass** at [`ValidationReport::new`], keyed by
/// `(probe, check)` identically to the task gate (the M6 post-pass): `doc-code.*` content
/// findings are tunable, the `pack-probe-integrity.*` meta-findings intrinsic-blocking and
/// untunable. A no-delta `resolved` leaves every emitted severity untouched.
///
/// **No anchor → the loud guard only.** A store with no `code-anchor` leaf (or only
/// list-valued ones) never calls the invoker and writes no snapshot — the report carries
/// only the enumeration's guard findings, never silently dropping the multi-valued guard
/// (`validation.md` → Multi-valued anchors get a non-silent guard).
pub fn validate_store(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    resolved: &crate::cascade::Resolved,
    invoke_doc_code: &ProbeInvoker<'_>,
) -> std::io::Result<ValidationReport> {
    let (anchors, mut findings) = enumerate_committed_surface(repo_root, schemas)?;
    findings.extend(store_doc_code(repo_root, anchors, invoke_doc_code)?);
    Ok(ValidationReport::new(findings, resolved))
}

/// One cascade-resolved workflow definition fed to the store-scope `workflow↔refs` family
/// — its top-level `id`, its raw definition `bytes`, and the **origin-pack** command
/// `catalog` its `{{cli.X}}` refs resolve against. The CLI builds one per enumerated
/// workflow ([`validate_store_families`]), resolving each `catalog` against the workflow's
/// own origin pack (mirroring `jigc start`'s `origin_pack` → `load_catalog` idiom) so a
/// loser-pack workflow's command-refs are checked against ITS OWN pack's catalog, never the
/// precedence-winner's (`multi-pack.md` → Pack-local body-reference resolution). The `id`
/// also drives the per-workflow [`StepSource::scope_to_workflow`](crate::compose::StepSource::scope_to_workflow)
/// call so the same workflow's includes read its own pack's steps. For a single pack the
/// origin *is* the one pack, so each catalog/scope is byte-identical to a flat resolution.
pub struct StoreWorkflow {
    /// The workflow's top-level id — the scope key for origin resolution.
    pub id: String,
    /// The raw cascade-resolved definition bytes (a project whole-file shadow already won).
    pub bytes: Vec<u8>,
    /// The command catalog of this workflow's **origin pack** — what its `{{cli.X}}` refs
    /// resolve against (membership-only at store scope).
    pub catalog: crate::compose::CommandCatalog,
}

/// The store-scope **three-family** sweep — the [`validate_store`] superset the top-level
/// `jigc validate` drives, folding all three read-only store targets into one
/// [`ValidationReport`] (`validation.md` → Completing the envelope: host three families
/// under the uniform exit rule). **Task-less and read-only** throughout: no working area,
/// no `FileStateRecord` write, no edge-index mutation.
///
/// The three families, in a stable sweep order:
///
/// - **doc↔code** — every committed doc's `code-anchor` leaves resolved against the working tree via the CLI-supplied subprocess `invoke_doc_code` seam (the [`validate_store`] body, lifted to [`store_doc_code`]). The only family that can raise a `pack-probe-integrity.*` meta-finding (it is the one subprocess probe).
/// - **workflow↔refs** — each cascade-resolved workflow definition (the CLI enumerates + reads them, address-sorted, feeding each as a [`StoreWorkflow`] bundling its id, raw bytes, and **origin-pack** command catalog) run through the **task-independent** store-scope checks ([`crate::compose::workflow_refs_store`]): `include-resolves`, `include-cycle-absent`, `body-include-only`, the three marker-shadow checks, `fan-out-join-paired`, and the **catalog-membership-only** command-ref path. The task-data checks stay at `jigc start`. `workflow_source` is the CLI's layer-aware [`StepSource`](crate::compose::StepSource), **scoped per-workflow** to that definition's origin pack via [`StepSource::scope_to_workflow`](crate::compose::StepSource::scope_to_workflow) so a loser-pack workflow's includes + command-refs resolve against ITS OWN pack, never the precedence-winner's catalog (`multi-pack.md` → Pack-local body-reference resolution: `command-ref-resolves` and the include checks fire **per-definition against that definition's own pack**). For a single pack each origin *is* the one pack, so the resolution is byte-identical to a flat catalog (the no-composition floor).
/// - **file↔CLI-state** — the read-only committed-store hash twin ([`crate::file_state::detect_committed_store`]): each committed managed doc's on-disk hash against its `record` entry, **detect without absorb** (`record` is borrowed `&`, no write, never via `reconcile_committed_store`).
///
/// The engine stays **domain-empty**: the caller (CLI) resolves the cascade and feeds in
/// the schemas, the per-workflow definition bundles (id + bytes + origin catalog), the step
/// source, and the loaded `FileStateRecord`. Severity is the engine-owned post-pass at
/// [`ValidationReport::new`], keyed by `(probe, check)` identically to every other entry
/// point (a no-delta `resolved` leaves every emitted severity untouched).
// The CLI threads each store target's distinct determinism-boundary inputs in (the engine
// produces none of them): the committed-store root, the resolved schemas/cascade, the
// shell-free doc-code seam, the per-workflow definition bundles + their step source, and
// the loaded file-state record. Bundling them into a struct would only relocate the same
// arity, so the lint is allowed at this one composition point.
#[allow(clippy::too_many_arguments)]
pub fn validate_store_families(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    resolved: &crate::cascade::Resolved,
    invoke_doc_code: &ProbeInvoker<'_>,
    workflows: &[StoreWorkflow],
    workflow_source: &dyn crate::compose::StepSource,
    record: &FileStateRecord,
) -> std::io::Result<ValidationReport> {
    let mut findings = Vec::new();

    // Family 1 — doc↔code (the only subprocess probe; the one family that can raise a
    // `pack-probe-integrity.*` meta-finding).
    let (anchors, guard_findings) = enumerate_committed_surface(repo_root, schemas)?;
    findings.extend(guard_findings);
    findings.extend(store_doc_code(repo_root, anchors, invoke_doc_code)?);

    // Family 2 — workflow↔refs over each cascade-resolved definition (task-independent
    // checks + the membership-only command-ref path; the CLI enumerated + read them).
    // Each definition resolves **pack-locally**: the step source is scoped to this
    // workflow's origin pack (so its `{{include: step:X}}` reads its own pack's step) and
    // the membership check runs against this workflow's own origin catalog (so a loser-pack
    // workflow's `{{cli.X}}` resolves against ITS pack, never the precedence-winner's). For
    // a single pack the origin *is* the pack, so the scope is inert and the catalog is the
    // same — byte-identical to the pre-fix flat path (`multi-pack.md` → Pack-local
    // body-reference resolution).
    for workflow in workflows {
        workflow_source.scope_to_workflow(&workflow.id);
        findings.extend(crate::compose::workflow_refs_store(
            &workflow.bytes,
            workflow_source,
            &workflow.catalog,
        ));
    }

    // Family 3 — file↔CLI-state, the read-only detect-without-absorb committed-store twin.
    findings.extend(crate::file_state::detect_committed_store(
        record, schemas, repo_root,
    ));

    // Family 4 — cross-doc forward-ref integrity (the store-wide analog of the task-scope
    // `ref-resolves` finalize gate): every committed forward edge's target must resolve in
    // the committed store. Closes the doc↔code-vs-ref-resolves store-sweep asymmetry so the
    // salience-independent pre-commit backstop reaches a dangling cross-doc ref. The rebuild
    // is a pure in-memory builder (no save) — the stamp is inert at store scope.
    let committed_index = crate::index::rebuild_committed(repo_root, schemas, "store-sweep");
    findings.extend(crate::index::ref_resolves_store(
        &committed_index,
        repo_root,
        schemas,
    ));

    // Family 5 — schema-completeness (inverse / minimum-cardinality): the completeness twin
    // of Family 4's integrity walk. For every persisted `ref` declaring an `inverse-card`
    // minimum (`spec.derived-from → prd`, `inverse-card: "1..*"`), each committed target doc
    // below the minimum surfaces an **advisory** `schema-completeness.inverse-cardinality`
    // finding (a PRD with zero inbound `derived-from` edges → "needs ≥ 1 spec"). Reuses the
    // Family-4 `committed_index` to count inbound edges. This is **completeness, not
    // integrity** — it depends on *other* tasks, so it is **never** wired into
    // [`validate_task`] (the per-task `finalize` gate); only this store / milestone scope
    // sweep runs it (`design/document-type-schema.md` → Inverse-cardinality obligations are
    // never a per-task gate; `design/validation.md` → Integrity vs completeness). Advisory by
    // default — report-only, gating nothing.
    findings.extend(crate::index::inverse_cardinality_store(
        &committed_index,
        repo_root,
        schemas,
    ));

    // Family 6 — in-prose managed-mention integrity (`schema-conformance.mention-resolves`,
    // M33 Inc-3): the lighter, prose-embedded sibling of Family 4's `ref-resolves`. Scan
    // every committed doc's **slot prose** for managed mentions (`#<type>:<slug>`, `<type>`
    // a known doctype — the `:`-plus-known-doctype discriminator means a bare external
    // `#issue-42` is never a mention) and report each whose target is not
    // `committed_reachable`. **Advisory, store-scope only** — a mention dangles when
    // *another* doc is renamed/deleted, so store scope is its home; it is **never** wired
    // into [`validate_task`] (the per-task finalize gate). That advisory/store-scope
    // placement is exactly what "lighter than `field`-refs" means (`design/validation.md`
    // → the mention-resolves check; `design/document-type-schema.md` → In-prose mentions).
    findings.extend(crate::index::mention_resolves_store(repo_root, schemas));

    // The **fifth content family** (`validation.md` taxonomy → Store-scope schema-
    // conformance; the M34 detect-half). Re-parse **every committed instance against the
    // current resolved schema** and report each `schema-conformance.{required-slot-present,
    // required-field-present, field-value-conformant}` break. Unlike the hash-only
    // file↔CLI-state family (Family 3, above) this **re-parses**, so a doc made
    // non-conformant by a *schema-shape change* with its **bytes unchanged** (the M34
    // v1-corpus-under-v2-schema case) is detected rather than invisible. `ref-resolves` is
    // not re-emitted here — that is the index-based Family 4 ([`ref_resolves_store`]),
    // already at store scope (the four-check deliverable is 3-new + 1-existing). Report-only
    // at store scope like every content family (`corpus-migration.md` → the detect-half).
    findings.extend(schema_conformance_store(repo_root, schemas));

    Ok(ValidationReport::new(findings, resolved))
}

/// The **fifth content family** (`validation.md` → Store-scope schema-conformance — the
/// fifth family; `corpus-migration.md` → the detect-half): re-parse **every committed
/// instance against the current resolved schema** and report each `schema-conformance.*`
/// presence/value break — closing the silent-drift hole a schema-shape change opens. A doc
/// made non-conformant with its **bytes unchanged** is invisible to the hash-only
/// file↔CLI-state family ([`crate::file_state::detect_committed_store`], which compares
/// hashes, not structure), so it must be **re-parsed**, not re-hashed.
///
/// A **clean lift** of the pure task-scope [`conformance_for`] — no working area, no
/// `FileStateRecord`, no edge index. It walks the committed store with the
/// [`detect_committed_store`](crate::file_state::detect_committed_store) walk shape: per
/// persisted (`location:`-bearing) schema, slug-sorted `.md`
/// ([`committed_slugs`](crate::index::committed_slugs)), transient (location-less) types
/// skipped — **not** the anchor-scoped `enumerate_committed_surface`, which under-walks. Per
/// doc it synthesizes the `<type>:<slug>.md` filename + `<location>/<slug>.md` record key
/// [`conformance_for`] expects, so parse-level findings surface too;
/// `schema-conformance.unknown-type` can never fire here (the type comes from schema
/// iteration, never a filename prefix).
///
/// `ref-resolves` is **not** re-emitted — it is the index-based Family 4
/// ([`ref_resolves_store`](crate::index::ref_resolves_store)), already at store scope. So
/// this family is the three per-instance presence/value checks; the four-check deliverable
/// is 3-new + 1-existing, no scope dropped.
///
/// **Report-only at store scope** like every content family: the `schema-conformance.*`
/// findings carry their intrinsic-blocking severity, but *blocking* is a finalize/task-scope
/// verdict — under the read-only store sweep the same finding is *listed*, never a gate (the
/// CLI exit keys only on `pack-probe-integrity.*`). Read-only by construction: the only I/O
/// is reading the committed `.md` bytes.
///
/// Deterministic: `schemas` is a [`BTreeMap`] (type-sorted), the docs of each type enumerate
/// slug-sorted, and per doc the checks run in section-document order.
fn schema_conformance_store(repo_root: &Path, schemas: &BTreeMap<String, Schema>) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (ty, schema) in schemas {
        let Some(location) = schema.location.as_deref() else {
            continue; // a transient (location-less) type has no committed docs.
        };
        for slug in crate::index::committed_slugs(repo_root, location) {
            let rel_key = format!("{location}{slug}.md");
            let Ok(bytes) = std::fs::read(repo_root.join(&rel_key)) else {
                continue; // read race: skip; the next sweep re-checks.
            };
            // Mirror the pure task-scope call exactly (no BOM strip, `from_utf8_lossy`):
            // a non-UTF-8 instance is the parser's concern, surfaced via `conformance_for`.
            let source = String::from_utf8_lossy(&bytes);
            let filename = format!("{ty}:{slug}.md");
            findings.extend(conformance_for(&filename, schemas, &rel_key, &source));
        }
    }
    findings
}

/// Drive the `doc-code` subprocess probe over an already-enumerated committed-store anchor
/// surface — the doc↔code family shared by [`validate_store`] and [`validate_store_families`].
/// Materializes the serializable [`EffectiveStateSnapshot`] to a temp-dir scratch path
/// (never under a managed `location:`), drives the CLI-supplied seam over it, ingests the
/// probe's findings + any `pack-probe-integrity.*` meta-findings, and removes the scratch
/// before returning. An empty `anchors` (no `code-anchor` leaf) never calls the invoker and
/// writes no snapshot — the omitting-context inert path.
fn store_doc_code(
    repo_root: &Path,
    anchors: Vec<crate::target_surface::TargetAnchor>,
    invoke_doc_code: &ProbeInvoker<'_>,
) -> std::io::Result<Vec<Finding>> {
    if anchors.is_empty() {
        // No anchor to probe. The invoker is not called and no snapshot is written.
        return Ok(Vec::new());
    }

    // The representative target the wire envelope carries (the probe reads the full set
    // from the snapshot). `anchors` is non-empty here, so the first is always present.
    let target = anchors[0].address.clone();

    // Materialize the snapshot to a fresh temp-dir scratch file — task-less, so it must
    // NOT land under a managed `location:` (`validation.md` → the scratch path is a temp
    // dir). Removed before returning; never committed.
    let snapshot = EffectiveStateSnapshot::new(anchors, repo_root.to_path_buf());
    let scratch = store_scratch_path();
    std::fs::write(&scratch, serde_json::to_vec(&snapshot)?)?;

    let request = ProbeRequest::new(
        DOC_CODE_PROBE,
        target,
        scratch.clone(),
        serde_json::Map::new(),
    );
    let run = invoke_doc_code(&request);
    // Always clean the scratch file, whether the invoke succeeded or errored.
    let _ = std::fs::remove_file(&scratch);
    Ok(ingest_probe_run(DOC_CODE_PROBE, &run?))
}

/// A fresh, process-and-time-unique scratch path under [`std::env::temp_dir`] for the
/// store-sweep snapshot — never under a managed `location:` (the task-less sweep has no
/// working area to put it in). The caller writes it, drives the probe over it, and removes
/// it before returning.
fn store_scratch_path() -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "jigc-{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0),
        STORE_SNAPSHOT_FILE,
    ));
    path
}

/// The **file portion** of a `code-anchor` value (`<path>#<symbol>` → `<path>`; a bare
/// path with no `#` is itself the file). Split on the **first** `#`, byte-identical to
/// the `doc-code` probe's own `split_anchor` (`crates/cli/probes/doc-code/src/main.rs`),
/// so the engine's change-set scoping and the probe's resolution agree on what "the
/// file" is.
///
/// **This function is the path-locality chokepoint of the code-anchor blast radius.**
/// The universal `finalize` floor scopes its sweep to committed anchors whose file is in
/// the task's staged change-set, and that scoping is *complete* only because a
/// `code-anchor` resolves **path-locally** — `p#s` is a pure function of `p`'s bytes, so
/// a committed anchor can newly-dangle only when its file `p` changed. Every shipped
/// grammar (the six languages + CSS + YAML) resolves this way, and the reserved
/// `#kind:name` qualifier stays path-local. If a future resolver ever became **non-local**
/// (resolving a citation through imports / re-exports / build config, so changing file
/// `B` could dangle an anchor into file `A`), change-set scoping keyed on this function
/// would under-cover — so the invariant is guarded by `path_locality_*` tests that pin
/// "the scoped file is the literal pre-`#` path", forcing the model change to fail loud
/// rather than silently leak drift (`validation.md` → Scope = effective state, the
/// path-locality note; `DECISIONS.md` 2026-06-22 → Phase 2).
pub(crate) fn anchor_file(anchor_value: &str) -> &str {
    anchor_value
        .split_once('#')
        .map(|(file, _symbol)| file)
        .unwrap_or(anchor_value)
}

/// Schedule the `doc-code` probe over the task's effective-state target surface
/// (`validation.md` → The `doc-code` probe → Target surface). The engine owns three of
/// the four steps — **enumerate** ([`enumerate_target_surface`]), **materialize** the
/// serializable [`EffectiveStateSnapshot`] (written to `<dir>/probe-snapshot.json`, the
/// path-ref the wire carries), and **ingest** ([`ingest_probe_run`]) — and hands the
/// **invoke** step to the CLI-supplied `invoke` closure (the engine never shells out).
///
/// One probe invocation covers the whole surface: the snapshot itemizes every anchor,
/// and the probe adjudicates each against `working_tree_root` (`code_tree_root` — the
/// root the about-to-be-committed code lives in: the materialized git index at finalize
/// scope, the working tree at task-`validate` scope). `repo_root` (the committed-store
/// root) still drives the **enumeration** of the doc-side target surface; only the
/// code-side resolution rides `code_tree_root`. The wire request names a representative
/// `target` (the first anchor's address) so the envelope is well-formed; the probe reads
/// the full anchor set from the snapshot, not the request's `target`.
///
/// **Inert when the surface is empty** — a task carrying no `code-anchor` leaf yields no
/// pairs, so the invoker is never called and no snapshot is written: the returned findings
/// are empty and the sweep is byte-identical to the pre-wiring path (the omitting-context
/// guard, hardening #5). `config` is the empty object — doc-code reads no cascade config
/// beyond severity, which the engine assigns downstream in its post-pass.
fn schedule_doc_code(
    dir: &Path,
    repo_root: &Path,
    code_tree_root: &Path,
    base_code_tree_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    changed_code: &BTreeSet<String>,
    invoke: &ProbeInvoker<'_>,
) -> std::io::Result<Vec<Finding>> {
    let (mut anchors, guard_findings) = enumerate_target_surface(dir, repo_root, schemas)?;

    // The **code-anchor blast radius** — the universal `finalize` floor. A committed
    // doc's `code-anchor` whose target file the task changed (`changed_code`) is dragged
    // into the surface and re-resolved, so a rename that dangles a citation in a doc the
    // task never opened is caught under *any* workflow (the `doc→code` analog of the
    // inbound-edge blast-radius walk — `validation.md` → Scope = effective state). Scoping
    // by the change-set covers the right set because resolution is path-local
    // ([`anchor_file`]): a committed `p#s` can change resolution only if `p` is in the set.
    // **Both sides are lexically normalized** ([`crate::store::lexical_normalize`]) so a
    // committed anchor written `./src/foo.rs#X` matches git's canonical `src/foo.rs` —
    // without it a non-canonical anchor path silently escapes the floor (a false negative).
    // The blast set is deduped against the task surface by address (never double-reported);
    // an anchor the task itself authored stays a task-surface anchor (always its own
    // responsibility), so only the *committed, task-untouched* anchors take the
    // newly-dangled treatment below.
    let mut blast_addresses: BTreeSet<String> = BTreeSet::new();
    if !changed_code.is_empty() {
        let changed_norm: BTreeSet<std::path::PathBuf> = changed_code
            .iter()
            .map(|p| crate::store::lexical_normalize(Path::new(p)))
            .collect();
        // `enumerate_committed_surface` also returns a **multi-valued guard** for any
        // list-valued committed `code-anchor`. That guard is **intentionally not surfaced
        // here** (dropped with reason, not silently — Codex P2): scoping it to *this task*
        // would require parsing the list elements' files to test change-set membership, which
        // the design declines (no list-element check is built), and **no shipped doctype
        // declares a list-valued `code-anchor`**, so there is no per-task trigger. A
        // list-valued committed anchor is a **store-scope** concern, surfaced by `jigc
        // validate` (which keeps all of these guards), never re-attributed to a task here.
        let (committed, _committed_guard_is_store_scoped) =
            enumerate_committed_surface(repo_root, schemas)?;
        for anchor in committed {
            let file =
                crate::store::lexical_normalize(Path::new(anchor_file(&anchor.anchor_value)));
            if changed_norm.contains(&file) && !anchors.iter().any(|a| a.address == anchor.address)
            {
                blast_addresses.insert(anchor.address.clone());
                anchors.push(anchor);
            }
        }
        anchors.sort_by(|a, b| a.address.cmp(&b.address));
    }

    if anchors.is_empty() {
        // No anchor to probe, but a list-valued `code-anchor` still surfaces its loud
        // guard finding (the multi-valued non-silent guard, `validation.md` →
        // Store-scope re-validation) — never silently dropped, even at task scope.
        return Ok(guard_findings);
    }

    // The index probe over the merged surface (task ∪ blast), resolving against the
    // about-to-be-committed code (`code_tree_root` — the materialized git index).
    let index_findings =
        run_doc_code_probe(&anchors, code_tree_root, &dir.join(SNAPSHOT_FILE), invoke)?;

    // **Newly-dangled filter** — the cross-task-attribution guard. A *blast* finding is kept
    // (blocks) only if its anchor **affirmatively resolved at the base (HEAD) tree** — i.e.
    // the base probe returned **no finding at all** for it — *and* it dangles at the index.
    // So a task that merely touches a file already carrying stale committed citations is not
    // wedged on drift outside its control (`validation.md` → Scope = effective state, the
    // newly-dangled rule). "Affirmatively resolved" is stricter than "not blocking at base":
    // a base **advisory** (an *uncheckable* anchor — a symlink-anchor / unsupported-language)
    // is NOT proof the symbol was present, so it does not count as resolved and its index
    // dangle is not attributed here (the store sweep still surfaces it). Task-surface findings
    // are never filtered — the task authored those docs, so a dangling anchor there is always
    // its own integrity to fix. The base probe runs only over the blast set, only when there
    // is one; its `pack-probe-integrity.*` failures (a base comparison that did not actually
    // run) are **surfaced**, never swallowed — a broken base probe blocks loudly rather than
    // silently letting the comparison pass.
    let resolved_findings: Vec<Finding> = if blast_addresses.is_empty() {
        index_findings
    } else {
        let blast_anchors: Vec<crate::target_surface::TargetAnchor> = anchors
            .iter()
            .filter(|a| blast_addresses.contains(&a.address))
            .cloned()
            .collect();
        let base_findings = run_doc_code_probe(
            &blast_anchors,
            base_code_tree_root,
            &dir.join(BASE_SNAPSHOT_FILE),
            invoke,
        )?;
        // A base probe-integrity failure (timeout/crash/malformed) carries no address; surface
        // these so a base comparison that did not run blocks rather than passing silently.
        let base_meta: Vec<Finding> = base_findings
            .iter()
            .filter(|f| f.probe == "pack-probe-integrity")
            .cloned()
            .collect();
        // An anchor is "unresolved at base" if the base probe emitted **any** finding for it
        // (a pre-existing dangle, OR an uncheckable advisory) — only an anchor with *no* base
        // finding affirmatively resolved there.
        let base_unresolved: BTreeSet<&str> =
            base_findings.iter().filter_map(finding_address).collect();
        let mut kept: Vec<Finding> = index_findings
            .into_iter()
            .filter(|f| {
                !finding_address(f)
                    .is_some_and(|a| blast_addresses.contains(a) && base_unresolved.contains(a))
            })
            .collect();
        kept.extend(base_meta);
        kept
    };

    // The probe's (newly-dangled-filtered) findings, plus any multi-valued guard findings (a
    // list-valued `code-anchor` is never silently dropped, even alongside enumerable anchors).
    let mut findings = guard_findings;
    findings.extend(resolved_findings);
    Ok(findings)
}

/// One `doc-code` probe invocation over `anchors`, resolving each against `code_tree_root`
/// with the serializable snapshot materialized at `snapshot_path`. Empty `anchors` → no
/// invocation, no snapshot, empty findings (the omitting-context inert path). Shared by the
/// index sweep and the newly-dangled **base** sweep so both materialize + invoke + ingest
/// identically.
fn run_doc_code_probe(
    anchors: &[crate::target_surface::TargetAnchor],
    code_tree_root: &Path,
    snapshot_path: &Path,
    invoke: &ProbeInvoker<'_>,
) -> std::io::Result<Vec<Finding>> {
    if anchors.is_empty() {
        return Ok(Vec::new());
    }
    // The representative target the wire envelope carries (the probe reads the full set from
    // the snapshot). `anchors` is non-empty here, so the first is always present.
    let target = anchors[0].address.clone();
    let snapshot = EffectiveStateSnapshot::new(anchors.to_vec(), code_tree_root.to_path_buf());
    std::fs::write(snapshot_path, serde_json::to_vec(&snapshot)?)?;
    let request = ProbeRequest::new(
        DOC_CODE_PROBE,
        target,
        snapshot_path.to_path_buf(),
        serde_json::Map::new(),
    );
    let run = invoke(&request)?;
    Ok(ingest_probe_run(DOC_CODE_PROBE, &run))
}

/// A finding's located target address, if it carries one.
fn finding_address(f: &Finding) -> Option<&str> {
    f.location.as_ref().and_then(|l| l.address.as_deref())
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
/// recursing into a **nested** repeatable's items at every level (the M22 multi-level
/// lift), findings addressed at `#section/item/.../leaf` — one segment deeper per
/// nesting level. Returns one blocking [`Finding`] per violation, in section-document
/// order; a conformant instance yields an empty `Vec`.
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
                // required-slot-present: a declared, non-optional body slot must hold
                // non-empty prose. An `optional:` slot is exempt — its absence never
                // blocks finalize (`design/changelog.md` → engine work #3).
                if declared_slot.as_ref().is_some_and(|s| !s.optional) {
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
/// limit), recursing into a **nested** repeatable's items at every level (the M22
/// multi-level lift; `design/changelog.md` → engine work #1). Per item, every
/// declared block leaf is adjudicated — a `slot` leaf via `required-slot-present`, a
/// `field` leaf via `required-field-present` + `field-value-conformant`, a nested
/// `repeatable` leaf by recursing into the item's nested items one level deeper —
/// **except the `id-from` source field**, which renders as the item heading not a
/// trailing bullet (mirroring [`crate::parse`]'s `ItemTemplate::from` heading-field
/// exclusion at **every** level); checking its presence as a bullet would false-fail
/// every conformant item. Findings address at `#section/item/.../leaf` (the M13
/// fragment vocabulary, one segment deeper per nesting level).
fn check_repeatable(
    section: &Section,
    repeatable: &crate::schema::Repeatable,
    parsed: &ParsedSection,
    source: &str,
    findings: &mut Vec<Finding>,
) {
    for item in &parsed.items {
        // The address path up to and including this item: `section/item` at the top
        // level, deepening one segment per nesting level as the recursion descends.
        let item_path = format!("{}/{}", section.id, item.id);
        check_item_leaves(&item_path, repeatable, item, source, findings);
    }
}

/// Adjudicate one repeatable item's declared block leaves, addressed under
/// `item_path` (the `section/item/.../item` path up to and including this item).
/// Recurses into a nested `Leaf::Repeatable`'s items one segment deeper. The shared
/// per-item body of [`check_repeatable`], lifted so the top level and every nested
/// level run the identical conformance with a deepening address.
fn check_item_leaves(
    item_path: &str,
    repeatable: &crate::schema::Repeatable,
    item: &ParsedItem,
    source: &str,
    findings: &mut Vec<Finding>,
) {
    for leaf in &repeatable.block {
        match leaf {
            // A non-optional slot leaf must hold non-empty prose; an `optional:`
            // slot leaf is exempt (its absence never blocks), mirroring the simple
            // arm (`design/changelog.md` → engine work #3).
            crate::schema::Leaf::Slot { id, slot } if !slot.optional => {
                check_item_slot_present(item_path, item, id, source, findings);
            }
            crate::schema::Leaf::Slot { .. } => {}
            crate::schema::Leaf::Field(field) => {
                // The id-source field is the item heading, never a bullet — exempt
                // from the bullet-based `check_item_field` at this level just as at
                // the top level. But when the id-from declares an `enum`, the heading
                // value is still schema-constrained: re-slug it (the same re-slug the
                // parser's `heading_matches` uses) and require membership in the
                // (slug-form) enum members, so `Fixed`→`fixed` passes while a foreign
                // `Performance`→`performance` ∉ enum blocks — `migrate`'s "adopted iff
                // conformant" guarantee for the one enum id-from the changelog has
                // (`design/auto-migration.md` → Engine/validation work #1). A non-enum
                // id-from carries no such constraint and stays exempt — the shared
                // [`id_from_enum_violation`] adjudicator yields `None` for it.
                if field.id == repeatable.id_from {
                    check_id_from_enum(item_path, repeatable, item, findings);
                    continue;
                }
                check_item_field(item_path, item, field, findings);
            }
            // A nested repeatable: recurse over this item's nested items one segment
            // deeper, each adjudicated against the nested block's own `id-from`
            // exemption and addressed at the **section-qualified**
            // `…/parent/<nested-section>/child/leaf` (review finding S1,
            // `design/changelog.md` → engine work #1) — the nested-section id is included
            // so the address the gate names is exactly the canonical form an agent types
            // and `set-slot`/`add-item` emit (consistent across every surface).
            crate::schema::Leaf::Repeatable {
                id: nested_section,
                repeatable: nested,
            } => {
                for child in &item.items {
                    let child_path = format!("{item_path}/{nested_section}/{}", child.id);
                    check_item_leaves(&child_path, nested, child, source, findings);
                }
            }
        }
    }
}

/// `required-slot-present` for one repeatable item's declared slot: its prose must
/// be non-empty. The parser records the item's slot span (the heading is present,
/// so the section parsed), so an all-whitespace slice is the unfilled-slot case.
/// Addressed at `item_path/leaf` (one segment deeper per nesting level).
fn check_item_slot_present(
    item_path: &str,
    item: &ParsedItem,
    leaf_id: &str,
    source: &str,
    findings: &mut Vec<Finding>,
) {
    // The addressed slot leaf's own span — single-slot via the bare `slot`,
    // multi-slot via the named `slots` entry (`slot_span` resolves either). Each
    // declared slot leaf is checked against ITS span, so a multi-slot item with one
    // empty required slot yields one blocking finding for that leaf alone.
    let span = item.slot_span(leaf_id);
    let filled = span
        .map(|span| !span.slice(source).trim().is_empty())
        .unwrap_or(false);
    if !filled {
        let line = span.map(|span| span.start_line).unwrap_or(1);
        findings.push(blocking_conformance(
            "schema-conformance.required-slot-present",
            format!("required slot `{leaf_id}` in item `{item_path}` is empty"),
            Some(Location::addressed(
                format!("{item_path}/{leaf_id}"),
                line,
                1,
            )),
        ));
    }
}

/// `required-field-present` + `field-value-conformant` for one repeatable item's
/// declared block field. An absent author-required field blocks; a present field
/// whose value fails its declared type blocks. Both address at `item_path/leaf`.
fn check_item_field(
    item_path: &str,
    item: &ParsedItem,
    declared: &SchemaField,
    findings: &mut Vec<Finding>,
) {
    let address = format!("{item_path}/{}", declared.id);
    match item.fields.iter().find(|f| f.key == declared.id) {
        Some(present) => {
            if let Err(why) = crate::write::check_value(declared, &present.value) {
                findings.push(blocking_conformance(
                    "schema-conformance.field-value-conformant",
                    format!("field `{}` in item `{item_path}`: {why}", declared.id),
                    Some(Location::addressed(address, 1, 1)),
                ));
            }
        }
        None => {
            if is_author_required(declared) {
                findings.push(blocking_conformance(
                    "schema-conformance.required-field-present",
                    format!(
                        "required field `{}` is missing from item `{item_path}`",
                        declared.id
                    ),
                    Some(Location::addressed(address, 1, 1)),
                ));
            }
        }
    }
}

/// The finding code both the `add-item` write-time pre-check and finalize's
/// [`check_id_from_enum`] emit for a foreign id-from enum heading — one shared constant
/// so the two call sites cannot drift (review S3: M25 copies this discipline to four
/// doctypes, so the code lives in one place, not a mirrored literal). The leaf-suffix of
/// the *address* may differ by call site (no parsed item yet at add-item time); only the
/// **code** is identical across the two check times (`design/write-commands.md` → Two
/// check times).
pub const ID_FROM_ENUM_CODE: &str = "schema-conformance.field-value-conformant";

/// The shared id-from-enum adjudicator — the single owner of the re-slug + membership
/// discipline both check times route through (review S3). A repeatable's `id-from` value
/// lives in the item *heading* (rendered `### <Title>  {#slug}`), so when that id-from
/// field is an `enum` the heading is itself schema-constrained: re-slug `title` (the same
/// re-slug the parser's `heading_matches` applies — **not** [`crate::write::check_value`],
/// whose literal compare would reject `Fixed` for the `fixed` member) and test membership
/// in the declared enum members. Returns the slug-cased id when it is **not** a member
/// (the violation the caller addresses), else `None`.
///
/// A **non-enum** id-from (every shipped doctype's `title`/`key`/`version`) carries no
/// such constraint and yields `None` (the exempt path); a malformed enum schema (no `of`)
/// names no members, so any value is non-conformant — surfaced, not silently passed.
/// (`design/auto-migration.md` → Engine/validation work #1 / Hardening #3.)
pub fn id_from_enum_violation(
    repeatable: &crate::schema::Repeatable,
    title: &str,
) -> Option<String> {
    let field = repeatable.block.iter().find_map(|leaf| match leaf {
        crate::schema::Leaf::Field(f) if f.id == repeatable.id_from => Some(f),
        _ => None,
    })?;
    if field.ty != FieldType::Enum {
        return None;
    }
    let slug = crate::slug::slugify(title);
    let members = field.of.as_deref().unwrap_or(&[]);
    if members.iter().any(|m| m == &slug) {
        None
    } else {
        Some(slug)
    }
}

/// `field-value-conformant` for a repeatable item's **`id-from` enum** field — the
/// one case where the heading text (not a bullet in `item.fields`) is itself a
/// schema-constrained value (`design/auto-migration.md` → Engine/validation work #1).
/// The id-from heading is exempt from the ordinary bullet-based field check; the shared
/// [`id_from_enum_violation`] adjudicator owns the re-slug + membership test (a non-enum
/// id-from yields `None`, staying exempt). `Fixed`→`fixed` / `Added`→`added` pass; a
/// foreign `Performance`→`performance` ∉ enum blocks. The finding addresses
/// `item_path/<id-from>` (the slug-cased item id is already in `item_path`).
fn check_id_from_enum(
    item_path: &str,
    repeatable: &crate::schema::Repeatable,
    item: &ParsedItem,
    findings: &mut Vec<Finding>,
) {
    if let Some(slug) = id_from_enum_violation(repeatable, &item.title) {
        findings.push(blocking_conformance(
            ID_FROM_ENUM_CODE,
            format!(
                "id-from field `{}` in item `{item_path}`: `{}` is not an enum member",
                repeatable.id_from, slug
            ),
            Some(Location::addressed(
                format!("{item_path}/{}", repeatable.id_from),
                1,
                1,
            )),
        ));
    }
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
        // An **optional** field materialized present-but-empty (the front-matter key
        // is emitted, no value — the field analog of an optional slot whose heading is
        // present with no prose) is the absent-content case the `optional:` flag
        // governs: treat it as absent, not as an empty value that fails its type.
        // (M22 inc-3 `optional:` semantics — "headings present, the flag governs
        // whether absent content blocks"; the provisioned `commit` doc materializes an
        // empty `scope` key, so a first-task minimal commit must finalize clean.)
        Some(present) if declared.optional && value_is_empty(&present.value) => {}
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

/// Whether a parsed field value carries no content — an empty scalar (after trim) or
/// an empty list. Used to recognize a present-but-empty **optional** field as the
/// absent-content case its `optional:` flag exempts.
fn value_is_empty(value: &crate::field_block::Value) -> bool {
    match value {
        crate::field_block::Value::Scalar(s) => s.trim().is_empty(),
        crate::field_block::Value::List(items) => items.is_empty(),
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
/// of 0, e.g. the ADR `supersedes` relation's `0..*`), and not a **pack-declared
/// field type** (e.g. `code-anchor`). An optional ref and a pack-declared anchor
/// both carry no author obligation: presence is the agent's choice and its
/// *adjudication* — not its presence — is the finalize-time probe's job (`ref-resolves`
/// for a ref, the type's bound adjudicator for a pack type, e.g. `code-anchor` → `doc-code`).
/// (`design/validation.md` → the synthetic `ref-resolves` check; `document-type-schema.md`
/// → `ref` cardinality + Pack-declared field types; `DECISIONS.md 2026-06-06` → M10 inc-1:
/// both shipped anchors are optional.)
fn is_author_required(field: &SchemaField) -> bool {
    // An `optional:` field is never author-required — its absence does not block
    // finalize, while a required field still does (`design/changelog.md` → engine
    // work #3). Covers both the simple-section and repeatable-item field arms.
    if field.optional {
        return false;
    }
    if field.ty == FieldType::Ref && ref_min_cardinality_zero(field) {
        return false;
    }
    if matches!(field.ty, FieldType::Pack(_)) {
        return false;
    }
    // `owned-location` carries NO exemption (M17 reversed the M16 one, which made
    // the #5 gate bypassable by omission — a fresh mint that never set the field
    // finalized clean): a declared owned-location field is author-required like any
    // other no-default/no-set field. *Conformance* owns field-absence; the intrinsic
    // finalize-time #5 owner-artifact gate keeps owning the named file's path-safety
    // + durable presence (`design/methodology-docs.md` → The engine work, item 3
    // amendment 2026-06-12).
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

    /// (M17 inc-3 T3, inverting M16 inc-3 T1) A **declared** `owned-location` field
    /// is **author-required** like any other no-`default:`/no-`set:` field: omitting
    /// it fires exactly one blocking `required-field-present`. The M16 exemption made
    /// the #5 gate's "each run recorded as an owner-artifact" silently voidable — a
    /// fresh mint that never set `owner-artifact:` finalized clean with no finding
    /// anywhere (omission being the default state of a fresh mint). M17 reverses it:
    /// *conformance* owns field-**absence**; the #5 gate keeps owning the named
    /// *file's* presence/safety/trackedness. A **present** arbitrary value still
    /// fires zero `field-value-conformant` (recognition only at this layer — the
    /// opaque-scalar floor; the value's safety is the gate's job).
    /// (`design/methodology-docs.md` → The engine work, item 3 amendment 2026-06-12.)
    #[test]
    fn a_declared_owned_location_field_is_author_required() {
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

        // Omitted: no `owner-artifact` line — exactly one blocking
        // `required-field-present` (the M17 flip closing the omission bypass).
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
        assert_eq!(
            findings.len(),
            1,
            "an omitted declared `owned-location` field must fire exactly one finding, got {findings:?}",
        );
        assert_eq!(findings[0].severity, Severity::Blocking);
        assert_eq!(
            findings[0].code,
            "schema-conformance.required-field-present"
        );
        assert!(
            findings[0].message.contains("owner-artifact"),
            "the finding must name the missing field, got: {}",
            findings[0].message,
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

    /// A present `ref` value that fails the write-time shape check (a bare slug —
    /// no `<type>:` prefix) fires exactly one blocking
    /// `schema-conformance.field-value-conformant` at the `supersedes` leaf — the
    /// conformance gate routing through the same [`crate::write::check_value`] as
    /// the write verb, so a malformed migrated edge is caught here, not deferred to
    /// a misleading finalize dangle (`design/auto-migration.md` → Write-time
    /// ref-shape check; review S2 — the second of the two call sites).
    #[test]
    fn malformed_supersedes_ref_fires_one_field_value_conformant() {
        let yaml = b"\
type: adr
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: supersedes, type: ref, to: adr, card: \"0..1\" }
  - id: body
    slot: { hint: \"The decision body.\" }
";
        let schema = crate::schema::load_schema(yaml).expect("adr-like schema loads");
        let source = "\
---
supersedes: use-postgres
---

# A decision

## Body

The body.
";
        let doc = parse_sections(&schema, source)
            .unwrap_or_else(|f| panic!("fixture must parse; got {f:?}"));
        let findings = schema_conformance(&schema, source, &doc);
        assert_eq!(
            findings.len(),
            1,
            "a malformed `supersedes` ref must fire exactly one finding, got {findings:?}",
        );
        assert_eq!(findings[0].severity, Severity::Blocking);
        assert_eq!(
            findings[0].code,
            "schema-conformance.field-value-conformant"
        );
        assert!(
            findings[0].message.contains("supersedes"),
            "the finding must name the `supersedes` leaf, got: {}",
            findings[0].message,
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
mod nested_repeatable_conformance_tests {
    //! (M22 inc-1 T4) `schema-conformance.*` over a **two-level** repeatable: the
    //! three synthetic checks recurse into each nested item's leaves (the changelog's
    //! `release → change-group` shape), the `id-from` heading field exempted at
    //! **every** level, and a nested-leaf finding addressed one hop deeper
    //! (`<section>/<release>/<change-group>/<leaf>`). A conformant two-level instance
    //! yields none; a hand-malformed nested entry fires a blocking finding naming the
    //! **nested** leaf, not the parent. See `design/changelog.md` → engine work #1
    //! (the validate bullet: recurse, conformance at every level, nested address).

    use super::*;
    use crate::parse::parse_sections;

    /// A `changelog`-shaped two-level fixture (a test fixture, not pack content): a
    /// `releases` repeatable (depth 1, items `###`) whose block carries the `version`
    /// id-source field then a **nested** `changes` repeatable (depth 2, items `####`).
    /// Each change-group carries the `category` id-source field (exempt), a required
    /// `notes` slot, and an author-required `severity` enum — the three nested levers,
    /// so a fixture can violate exactly one at the nested level.
    fn schema() -> Schema {
        let yaml = b"\
type: changelog
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: severity, type: enum, of: [minor, major] }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";
        crate::schema::load_schema(yaml).expect("nested changelog schema loads")
    }

    fn parse(source: &str) -> Document {
        parse_sections(&schema(), source)
            .unwrap_or_else(|f| panic!("fixture must parse; got conformance findings: {f:?}"))
    }

    /// A fully-conformant two-level instance: one release with one change-group whose
    /// `notes` slot is filled, whose `severity` enum is a member, and whose `category`
    /// (the nested id-source heading) carries no trailing bullet.
    const CONFORMANT: &str = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

- OAuth device-code flow

<!-- fields -->
- severity: minor
";

    /// A nested change-group with an **empty required slot**: the `#### …` group
    /// heading parses, but its `notes` slot prose is blank.
    const NESTED_EMPTY_SLOT: &str = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

<!-- fields -->
- severity: minor
";

    /// A nested change-group with a **malformed required field value**: `severity` is
    /// present but not an enum member.
    const NESTED_MALFORMED_VALUE: &str = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

- OAuth device-code flow

<!-- fields -->
- severity: catastrophic
";

    /// (ii) A conformant two-level instance validates **clean at every level** — the
    /// nested `category` id-from heading is not false-failed as an absent bullet (the
    /// exemption must hold at the nested level too).
    #[test]
    fn conformant_two_level_instance_yields_no_findings() {
        let schema = schema();
        let doc = parse(CONFORMANT);
        let findings = schema_conformance(&schema, CONFORMANT, &doc);
        assert!(
            findings.is_empty(),
            "a conformant two-level instance must yield no findings, got {findings:?}"
        );
    }

    /// (i) An empty required nested slot **blocks** at
    /// `schema-conformance.required-slot-present`, the finding address naming the
    /// **nested** leaf (`<section>/<release>/<change-group>/<leaf>`), not the parent.
    #[test]
    fn empty_nested_slot_blocks_naming_the_nested_leaf() {
        let schema = schema();
        let doc = parse(NESTED_EMPTY_SLOT);
        let findings = schema_conformance(&schema, NESTED_EMPTY_SLOT, &doc);
        assert_eq!(
            findings.len(),
            1,
            "an empty nested slot must yield exactly one finding, got {findings:?}"
        );
        let finding = &findings[0];
        assert_eq!(finding.code, "schema-conformance.required-slot-present");
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.location.as_ref().and_then(|l| l.address.as_deref()),
            // The canonical SECTION-QUALIFIED nested address (review finding S1): the
            // `changes` nested-section segment is included — the same form `add-item`
            // emits and `set-slot` accepts, so the gate names the address an agent types.
            Some("releases/1-2-0/changes/added/notes"),
            "the finding must address the NESTED leaf (section-qualified), not the parent",
        );
    }

    /// (iii) A nested field whose value fails its declared type **blocks** at
    /// `schema-conformance.field-value-conformant`, addressed at the nested leaf.
    #[test]
    fn malformed_nested_field_value_blocks_at_the_nested_address() {
        let schema = schema();
        let doc = parse(NESTED_MALFORMED_VALUE);
        let findings = schema_conformance(&schema, NESTED_MALFORMED_VALUE, &doc);
        assert_eq!(
            findings.len(),
            1,
            "a malformed nested field value must yield exactly one finding, got {findings:?}"
        );
        let finding = &findings[0];
        assert_eq!(finding.code, "schema-conformance.field-value-conformant");
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.location.as_ref().and_then(|l| l.address.as_deref()),
            // Section-qualified nested address (review finding S1) — includes `changes`.
            Some("releases/1-2-0/changes/added/severity"),
            "the finding must address the NESTED leaf (section-qualified)",
        );
    }
}

#[cfg(test)]
mod id_from_enum_conformance_tests {
    //! (M23 inc-2 T1) The conformance gate enforces the `enum` on an `id-from`
    //! field, the one case the changelog has (the change-group's `category` *is* its
    //! `id-from` and *is* an enum). The id-from value lives in the parsed item
    //! **heading**, not `item.fields`, so a new check re-slugs the heading text and
    //! tests membership against the (slug-form) enum members: `Fixed`→`fixed` /
    //! `Added`→`added` pass, `Performance`→`performance` ∉ enum is rejected at one
    //! blocking `schema-conformance.field-value-conformant` addressed
    //! `…/<item>/category`. A **non-enum** `id-from` (a `string` `version`/`title`)
    //! stays exempt — no new finding (the no-blast-radius regression watch). See
    //! `design/auto-migration.md` → Engine/validation work #1.

    use super::*;
    use crate::parse::parse_sections;

    /// A `changelog`-shaped two-level fixture whose **both** repeatable id-from
    /// fields the check could touch are exercised: the top-level `groups` repeatable
    /// keys off an `enum` `category` (the change-group itself promoted to top level),
    /// and a nested `subgroups` repeatable also keys off an `enum` `category`. The
    /// nested level proves the check fires recursively. (The non-id-from `severity`
    /// enum is the control that the existing per-field check already covers.)
    fn schema() -> Schema {
        let yaml = b"\
type: changelog
sections:
  - id: groups
    repeatable:
      id-from: category
      block:
        - { id: category, type: enum, of: [added, fixed, removed] }
        - { id: notes, slot: { hint: \"One bullet per change.\" } }
        - id: subgroups
          repeatable:
            id-from: category
            block:
              - { id: category, type: enum, of: [added, fixed, removed] }
              - { id: detail, slot: { hint: \"Sub-detail.\" } }
";
        crate::schema::load_schema(yaml).expect("changelog enum-id-from schema loads")
    }

    /// A **non-enum** `id-from` variant of the same shape: `category` is a plain
    /// `string` (the regression-watch control standing in for every shipped
    /// doctype's `title`/`key`/`version` id-from). The check must leave it exempt.
    fn string_id_from_schema() -> Schema {
        let yaml = b"\
type: changelog
sections:
  - id: groups
    repeatable:
      id-from: category
      block:
        - { id: category, type: string }
        - { id: notes, slot: { hint: \"One bullet per change.\" } }
";
        crate::schema::load_schema(yaml).expect("string-id-from schema loads")
    }

    fn parse(schema: &Schema, source: &str) -> Document {
        parse_sections(schema, source)
            .unwrap_or_else(|f| panic!("fixture must parse; got conformance findings: {f:?}"))
    }

    /// (b) Headings that re-slug to a valid enum member at **both** levels fire
    /// **zero** id-from-enum findings — `Fixed`→`fixed`, `Added`→`added` pass.
    #[test]
    fn valid_member_headings_fire_no_finding() {
        let schema = schema();
        let source = "\
# Changelog

## Groups

### Fixed  {#fixed}

A top-level fix.

#### Added  {#added}

A nested addition.
";
        let doc = parse(&schema, source);
        let findings = schema_conformance(&schema, source, &doc);
        assert!(
            findings.is_empty(),
            "headings that re-slug to valid enum members must fire no findings, got {findings:?}"
        );
    }

    /// (a) top level — a heading re-slugging OUTSIDE the enum (`### Performance` →
    /// `performance` ∉ {added, fixed, removed}) fires exactly one blocking
    /// `schema-conformance.field-value-conformant` addressed `groups/performance/category`.
    #[test]
    fn top_level_foreign_category_blocks_at_the_slug_address() {
        let schema = schema();
        let source = "\
# Changelog

## Groups

### Performance  {#performance}

A perf change.
";
        let doc = parse(&schema, source);
        let findings = schema_conformance(&schema, source, &doc);
        assert_eq!(
            findings.len(),
            1,
            "a foreign top-level category must fire exactly one finding, got {findings:?}"
        );
        let finding = &findings[0];
        // BUILD-PIN: code + slug-cased leaf-suffix pinned against the real binary.
        assert_eq!(finding.code, "schema-conformance.field-value-conformant");
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("groups/performance/category"),
            "the finding must address the slug-cased id-from leaf",
        );
    }

    /// (a) nested level — a nested change-group whose heading re-slugs OUTSIDE the
    /// enum fires exactly one blocking `field-value-conformant` at the nested,
    /// section-qualified slug address `groups/fixed/subgroups/performance/category`.
    #[test]
    fn nested_foreign_category_blocks_at_the_nested_slug_address() {
        let schema = schema();
        let source = "\
# Changelog

## Groups

### Fixed  {#fixed}

A top-level fix.

#### Performance  {#performance}

A nested perf change.
";
        let doc = parse(&schema, source);
        let findings = schema_conformance(&schema, source, &doc);
        assert_eq!(
            findings.len(),
            1,
            "a foreign nested category must fire exactly one finding, got {findings:?}"
        );
        let finding = &findings[0];
        assert_eq!(finding.code, "schema-conformance.field-value-conformant");
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("groups/fixed/subgroups/performance/category"),
            "the finding must address the section-qualified nested slug leaf",
        );
    }

    /// (c) regression watch — a **non-enum** `id-from` (a `string` `category`
    /// standing in for every shipped doctype's `title`/`key`/`version`) stays
    /// exempt: an arbitrary foreign heading fires **zero** new findings.
    #[test]
    fn non_enum_id_from_stays_exempt() {
        let schema = string_id_from_schema();
        let source = "\
# Changelog

## Groups

### Performance  {#performance}

A perf change.
";
        let doc = parse(&schema, source);
        let findings = schema_conformance(&schema, source, &doc);
        assert!(
            findings.is_empty(),
            "a non-enum string id-from must stay exempt, got {findings:?}"
        );
    }
}

#[cfg(test)]
mod optional_slot_field_tests {
    //! (M22 inc-3 T1) The `optional:` flag exempts a slot/field from the
    //! requiredness check at **all four** enforcement arms (simple slot, simple
    //! field, repeatable slot, repeatable field). An absent optional slot/field
    //! finalizes **clean**; an absent **required** slot/field still **blocks** (the
    //! masking-test guard: the flag is the only thing suppressing the finding); and
    //! an absent optional field renders with **no stray `<!-- fields -->` line**
    //! (the byte-stable absent form, proven through the writer). See
    //! `design/changelog.md` → engine work #3.

    use super::*;
    use crate::parse::parse_sections;

    /// A simple-section fixture with both optional levers: an optional `link` field
    /// (no default/set) and an optional `note` slot section — plus a **required**
    /// `kind` field and a **required** `body` slot as the masking-test controls.
    fn simple_schema(optional: bool) -> Schema {
        let opt = if optional { ", optional: true" } else { "" };
        let yaml = format!(
            "\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - {{ id: title, type: string }}
      - {{ id: kind, type: enum, of: [memo, brief] }}
      - {{ id: link, type: string{opt} }}
  - id: body
    slot: {{ hint: \"The body.\" }}
  - id: note
    slot: {{ hint: \"An optional note.\"{opt} }}
"
        );
        crate::schema::load_schema(yaml.as_bytes()).expect("simple schema loads")
    }

    /// A source that **omits** the optional `link` field (no bullet) and leaves the
    /// optional `note` slot **empty** (heading present, no prose), while the required
    /// `kind` field and `body` slot are filled. `required-slot-present` /
    /// `required-field-present` adjudicate *content presence within a present
    /// structure* — the section/leaf headings must be present (parse-layer
    /// requirement), the optional `optional:` flag governs whether their absent
    /// *content* blocks.
    const SIMPLE_OMITS_OPTIONAL: &str = "\
---
title: A note
kind: memo
---

# A note

## Body

The body prose.

## Note
";

    /// (i, simple arm) An absent optional field AND optional slot yield ZERO
    /// findings when the flag is set.
    #[test]
    fn absent_optional_simple_slot_and_field_are_clean() {
        let schema = simple_schema(true);
        let doc = parse_sections(&schema, SIMPLE_OMITS_OPTIONAL)
            .unwrap_or_else(|f| panic!("fixture must parse; got {f:?}"));
        let findings = schema_conformance(&schema, SIMPLE_OMITS_OPTIONAL, &doc);
        assert!(
            findings.is_empty(),
            "absent optional simple slot + field must yield no findings, got {findings:?}",
        );
    }

    /// (ii, simple arm — masking-test guard) The SAME fixture with the flag removed
    /// STILL blocks: the absent `link` field fires `required-field-present` and the
    /// absent `note` slot section fires `required-slot-present`. The flag is the only
    /// thing suppressing those findings.
    #[test]
    fn without_the_flag_the_same_simple_slot_and_field_block() {
        let schema = simple_schema(false);
        let doc = parse_sections(&schema, SIMPLE_OMITS_OPTIONAL)
            .unwrap_or_else(|f| panic!("fixture must parse; got {f:?}"));
        let findings = schema_conformance(&schema, SIMPLE_OMITS_OPTIONAL, &doc);
        let codes: Vec<&str> = findings.iter().map(|f| f.code.as_str()).collect();
        assert!(
            codes.contains(&"schema-conformance.required-field-present"),
            "the absent required field must block, got {findings:?}",
        );
        assert!(
            codes.contains(&"schema-conformance.required-slot-present"),
            "the absent required slot must block, got {findings:?}",
        );
        assert!(
            findings.iter().all(|f| f.severity == Severity::Blocking),
            "the suppressed-by-flag findings must be blocking, got {findings:?}",
        );
    }

    /// A repeatable fixture whose item block carries both optional levers — an
    /// optional `link` field and an optional `note` slot leaf — plus a **required**
    /// `kind` field and a **required** `detail` slot as the masking-test controls.
    /// The `title` id-from field is the heading (exempted).
    fn repeatable_schema(optional: bool) -> Schema {
        let opt = if optional { ", optional: true" } else { "" };
        let yaml = format!(
            "\
type: note
id-from: title
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - {{ id: title, type: string }}
        - {{ id: kind, type: enum, of: [memo, brief] }}
        - {{ id: link, type: string{opt} }}
        - {{ id: detail, slot: {{ hint: \"The detail.\" }} }}
        - {{ id: note, slot: {{ hint: \"An optional note.\"{opt} }} }}
"
        );
        crate::schema::load_schema(yaml.as_bytes()).expect("repeatable schema loads")
    }

    /// A repeatable item that OMITS the optional `link` field (no bullet) and leaves
    /// the optional `note` slot leaf **empty** (its `#### Note` sub-heading present,
    /// no prose) — the M4 omitting-context arm — while the required `kind` field and
    /// `detail` slot are filled. The multi-slot item renders both `#### Detail` /
    /// `#### Note` sub-labels (a parse-layer requirement); the `optional:` flag
    /// governs whether the empty `note` prose blocks.
    const REPEATABLE_OMITS_OPTIONAL: &str = "\
---
---

# A note

## Entries

### First entry  {#first-entry}

#### Detail

The detail prose.

#### Note

<!-- fields -->
- kind: memo
";

    /// (i, repeatable arm — the M4 omitting-context arm) A repeatable item that omits
    /// the optional slot leaf AND optional field yields ZERO findings when the flag
    /// is set.
    #[test]
    fn absent_optional_repeatable_slot_and_field_are_clean() {
        let schema = repeatable_schema(true);
        let doc = parse_sections(&schema, REPEATABLE_OMITS_OPTIONAL)
            .unwrap_or_else(|f| panic!("fixture must parse; got {f:?}"));
        let findings = schema_conformance(&schema, REPEATABLE_OMITS_OPTIONAL, &doc);
        assert!(
            findings.is_empty(),
            "absent optional repeatable slot + field must yield no findings, got {findings:?}",
        );
    }

    /// (ii, repeatable arm — masking-test guard) The SAME item with the flag removed
    /// STILL blocks: the absent `link` field fires `required-field-present` and the
    /// absent `note` slot leaf fires `required-slot-present`, each addressed at the
    /// item leaf. The flag is the only thing suppressing them.
    #[test]
    fn without_the_flag_the_same_repeatable_slot_and_field_block() {
        let schema = repeatable_schema(false);
        let doc = parse_sections(&schema, REPEATABLE_OMITS_OPTIONAL)
            .unwrap_or_else(|f| panic!("fixture must parse; got {f:?}"));
        let findings = schema_conformance(&schema, REPEATABLE_OMITS_OPTIONAL, &doc);
        let codes: Vec<&str> = findings.iter().map(|f| f.code.as_str()).collect();
        assert!(
            codes.contains(&"schema-conformance.required-field-present"),
            "the absent required item field must block, got {findings:?}",
        );
        assert!(
            codes.contains(&"schema-conformance.required-slot-present"),
            "the absent required item slot must block, got {findings:?}",
        );
        assert!(
            findings.iter().all(|f| f.severity == Severity::Blocking),
            "the suppressed-by-flag findings must be blocking, got {findings:?}",
        );
    }

    /// (iii) The byte-stable absent form, proven through the **writer** (C2): a
    /// repeatable item whose ONLY field is the optional `link` renders with NO
    /// `<!-- fields -->` bullet (and no sentinel) when the field is absent, while
    /// the present form renders the bullet. Both directions asserted byte-for-byte
    /// via `render(parse(src)) == src` — the emitted artifact is the contract.
    #[test]
    fn optional_field_renders_byte_stable_in_both_absent_and_present_forms() {
        // A schema whose item block carries ONLY the id-from heading + the optional
        // `link` field, so an absent `link` means an item with NO trailing field
        // group at all (the sole-field sentinel-suppression case).
        let yaml = b"\
type: note
id-from: title
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: link, type: string, optional: true }
";
        let schema = crate::schema::load_schema(yaml).expect("schema loads");

        // Absent form: the item carries no `<!-- fields -->` block.
        let absent = "\
# A note

## Entries

### First entry  {#first-entry}
";
        let absent_instance =
            crate::write::instance_from_source(&schema, absent).expect("absent form parses");
        let rendered_absent = crate::write::render(&schema, &absent_instance);
        assert_eq!(
            rendered_absent, absent,
            "the absent optional field renders with no stray fields-block line",
        );
        assert!(
            !rendered_absent.contains("<!-- fields -->"),
            "no sentinel when the sole field is absent, got: {rendered_absent}",
        );

        // Present form: the item renders the optional field as a single bullet.
        let present = "\
# A note

## Entries

### First entry  {#first-entry}

<!-- fields -->
- link: https://example.com/compare/1.0.0...1.1.0
";
        let present_instance =
            crate::write::instance_from_source(&schema, present).expect("present form parses");
        let rendered_present = crate::write::render(&schema, &present_instance);
        assert_eq!(
            rendered_present, present,
            "the present optional field renders its bullet byte-for-byte",
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
            area.dir(),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &BTreeSet::new(),
            area.dir(),
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
            clean.dir(),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &BTreeSet::new(),
            clean.dir(),
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

    /// A committed ADR `C` (a second supersede target), distinct from `A`, so a
    /// 2-element `supersedes` list can name two real committed docs.
    const ADR_C: &str = "\
---
status: accepted
date: 2026-05-24
---

# Round-robin session router

## Context
Routing must spread session load evenly.

## Decision
A round-robin router spreads session load across nodes.

## Consequences
A failed node's sessions are re-routed on next request.
";

    /// Commit ADR `C` at `decisions/round-robin-router.md`.
    fn commit_adr_c(repo_root: &Path) {
        let dir = repo_root.join("decisions");
        std::fs::create_dir_all(&dir).expect("mk decisions/");
        std::fs::write(dir.join("round-robin-router.md"), ADR_C).expect("write C");
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
            repo.path(),
            &jigc,
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &BTreeSet::new(),
            repo.path(),
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
            repo.path(),
            &jigc,
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &BTreeSet::new(),
            repo.path(),
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

    /// `supersedes` is `0..*`: a staged ADR may supersede MORE than one committed
    /// decision. A 2-element list (`[adr:single-node-cache, adr:round-robin-router]`)
    /// whose every target is committed resolves CLEAN — both edges find their target
    /// in the committed store, so `validate_task` surfaces no `ref-resolves` finding
    /// and does not block (the multi-element resolve path the single-supersedes tests
    /// never exercised). (`auto-migration.md` → Edge migration; `supersedes` widened
    /// 0..1 → 0..*.)
    #[test]
    fn validate_task_passes_on_resolvable_multi_supersedes() {
        let repo = TempRoot::new("multi-resolve-repo");
        let jigc = repo.path().join(".jigc");
        let task_dir = jigc.join("tasks").join("supersede-cache");
        commit_adr_a(repo.path());
        commit_adr_c(repo.path());
        stage_adr_b(
            &task_dir,
            "shared-redis-session-cache",
            "[adr:single-node-cache, adr:round-robin-router]",
        );

        let mut record = FileStateRecord::new();
        let report = validate_task(
            &task_dir,
            &schemas(),
            &mut record,
            repo.path(),
            repo.path(),
            &jigc,
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &BTreeSet::new(),
            repo.path(),
        )
        .expect("sweep runs");

        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.code == "schema-conformance.ref-resolves"),
            "both elements of a 2-element supersedes resolve in the committed store, \
             so no ref-resolves finding, got {:?}",
            report.findings
        );
        assert!(
            !report.has_blocking(),
            "a clean task with a fully-resolvable multi-element supersedes must not \
             block, got {:?}",
            report.findings
        );
    }

    /// The partial-dangle variant of the `0..*` list: one element of the 2-element
    /// `supersedes` is committed, the other dangles. The ref-resolves walk aggregates
    /// EXACTLY ONE blocking finding (for the dangling element only — the resolvable
    /// one yields nothing), naming the missing target, so `finalize` blocks.
    #[test]
    fn validate_task_blocks_on_partially_dangling_multi_supersedes() {
        let repo = TempRoot::new("multi-dangle-repo");
        let jigc = repo.path().join(".jigc");
        let task_dir = jigc.join("tasks").join("supersede-cache");
        commit_adr_a(repo.path());
        stage_adr_b(
            &task_dir,
            "shared-redis-session-cache",
            "[adr:single-node-cache, adr:typo-nonexistent]",
        );

        let mut record = FileStateRecord::new();
        let report = validate_task(
            &task_dir,
            &schemas(),
            &mut record,
            repo.path(),
            repo.path(),
            &jigc,
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &BTreeSet::new(),
            repo.path(),
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
            "exactly one ref-resolves finding — only the dangling element of the list, \
             got {:?}",
            report.findings
        );
        assert!(
            refresolves[0].message.contains("adr:typo-nonexistent"),
            "the finding names the dangling element: {}",
            refresolves[0].message
        );
        assert!(
            report.has_blocking(),
            "a partially-dangling forward-ref list must make the task block, got {:?}",
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
            repo.path(),
            &repo.path().join(".jigc"),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &|_p| true, // tracked is irrelevant — the file is absent.
            &BTreeSet::new(),
            repo.path(),
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
            repo.path(),
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
            repo.path(),
            &repo.path().join(".jigc"),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &|_p| true, // the artifact is tracked.
            &BTreeSet::new(),
            repo.path(),
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

#[cfg(test)]
mod validate_store_tests {
    //! The store-scope sweep ([`validate_store`]): enumerate every committed doc's
    //! `code-anchor` leaves, materialize the snapshot to a temp-dir scratch path, drive
    //! the CLI-supplied `doc-code` invoker over it, then ingest the probe's findings plus
    //! the `pack-probe-integrity.*` meta-findings into one [`ValidationReport`]. The
    //! invoker is an in-process double here (the real probe is `T2`'s acceptance).

    use super::*;
    use crate::file_state::hash_bytes;
    use crate::probe::{ProbeRequest, ProbeResponse, ProbeRun, ProbeRunStatus};
    use crate::schema::{dev_pack_field_types, load_schema_with_types};
    use std::cell::RefCell;
    use std::path::PathBuf;

    /// A throwaway committed-store root that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-validate-store-{tag}-{}-{:?}",
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
        /// Commit a doc at `<repo_root>/<location>/<slug>.md`.
        fn commit(&self, location: &str, slug: &str, body: &str) {
            let dir = self.0.join(location);
            std::fs::create_dir_all(&dir).expect("mk location");
            std::fs::write(dir.join(format!("{slug}.md")), body).expect("commit");
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    /// The two code-anchor doctypes the store sweep walks — `adr` (decisions/) with a
    /// header `cites-code`, and an inline `spec` (specs/) with a criterion `maps-to-test`.
    fn schemas() -> BTreeMap<String, Schema> {
        const SPEC_YAML: &[u8] = b"\
type: spec
location: specs/
id-from: title
sections:
  - id: goal
    slot: { hint: One sentence. }
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: maps-to-test, type: code-anchor, check: criterion-maps-to-test }
";
        let mut m = BTreeMap::new();
        m.insert(
            "adr".to_string(),
            load_schema_with_types(ADR_YAML, &dev_pack_field_types()).expect("adr.yaml loads"),
        );
        m.insert(
            "spec".to_string(),
            load_schema_with_types(SPEC_YAML, &dev_pack_field_types()).expect("spec fixture loads"),
        );
        m
    }

    /// A committed ADR with a **valid** anchor (a real symbol — `validate_task` exists in
    /// this very file).
    const ADR_VALID: &str = "\
---
status: accepted
date: 2026-05-23
cites-code: crates/engine/src/validate.rs#validate_task
---

# Valid decision

## Context
Forces.

## Decision
Decided.

## Consequences
Effects.
";

    /// A committed ADR with a **dangling** anchor — a well-formed `<path>#<symbol>` whose
    /// symbol does not exist (the renamed/deleted-symbol case the store sweep must catch).
    const ADR_DANGLING: &str = "\
---
status: accepted
date: 2026-05-23
cites-code: crates/engine/src/validate.rs#vanished_symbol
---

# Dangling decision

## Context
Forces.

## Decision
Decided.

## Consequences
Effects.
";

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

    /// An in-process `doc-code` invoker double (the `InProcessDocCodeProbe` pattern as an
    /// `impl Fn(&ProbeRequest) -> io::Result<ProbeRun>`): it reads the snapshot the engine
    /// materialized at the request's `snapshot_path`, and emits one blocking
    /// `doc-code.symbol-exists` content finding for every anchor whose value names a
    /// `vanished_symbol` (the dangling fixture). A valid anchor yields nothing. It records
    /// each snapshot path it was handed so the test can assert scratch-path placement +
    /// cleanup. Exits 0 with a well-formed [`ProbeResponse`].
    fn dangling_aware_invoker(
        seen: &RefCell<Vec<PathBuf>>,
    ) -> impl Fn(&ProbeRequest) -> std::io::Result<ProbeRun> + '_ {
        move |req| {
            seen.borrow_mut()
                .push(req.effective_state.snapshot_path.clone());
            let snapshot: EffectiveStateSnapshot =
                serde_json::from_slice(&std::fs::read(&req.effective_state.snapshot_path)?)?;
            let findings: Vec<Finding> = snapshot
                .anchors
                .iter()
                .filter(|a| a.anchor_value.contains("vanished_symbol"))
                .map(|a| {
                    Finding::graded(
                        Severity::Blocking,
                        "doc-code.symbol-exists",
                        format!("symbol does not resolve: {}", a.anchor_value),
                        Some(Location::addressed(a.address.clone(), 1, 1)),
                        None,
                    )
                })
                .collect();
            Ok(ProbeRun {
                stdout: serde_json::to_vec(&ProbeResponse::new(findings)).unwrap(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            })
        }
    }

    /// (Test 1, the done-criterion) A committed store with one **valid** and one
    /// **dangling** anchor: the real-shaped invoker emits exactly one blocking
    /// `doc-code.symbol-exists` content finding (for the dangling anchor), the valid one is
    /// clean, and `report.has_blocking()` reflects it. A clean store (valid anchor only)
    /// yields no content finding and does not block.
    #[test]
    fn validate_store_reports_dangling_anchor_clean_on_valid() {
        // --- The drifted store: one valid + one dangling committed anchor.
        let repo = TempRoot::new("dangling");
        repo.commit("decisions", "valid", ADR_VALID);
        repo.commit("decisions", "dangling", ADR_DANGLING);

        let seen = RefCell::new(Vec::new());
        let report = validate_store(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
        )
        .expect("store sweep runs");

        let content: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "doc-code.symbol-exists")
            .collect();
        assert_eq!(
            content.len(),
            1,
            "exactly one dangling anchor must surface one doc-code content finding, got {:?}",
            report.findings,
        );
        assert_eq!(content[0].severity, Severity::Blocking);
        assert!(
            content[0].message.contains("vanished_symbol"),
            "the finding must name the dangling anchor, got {}",
            content[0].message,
        );
        assert!(
            report.has_blocking(),
            "a dangling anchor must make the report block, got {:?}",
            report.findings,
        );

        // --- The clean store: only the valid anchor. No content finding, no block.
        let clean = TempRoot::new("clean");
        clean.commit("decisions", "valid", ADR_VALID);
        let seen_clean = RefCell::new(Vec::new());
        let clean_report = validate_store(
            clean.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen_clean),
        )
        .expect("clean store sweep runs");
        assert!(
            clean_report.findings.is_empty(),
            "a store with only valid anchors must yield an empty report, got {:?}",
            clean_report.findings,
        );
        assert!(!clean_report.has_blocking());
    }

    /// (Test 2) An invoker that fails — non-zero exit (crash), timeout, or exit-0 garbage
    /// (malformed-output) — yields **exactly one** blocking `pack-probe-integrity.*`
    /// meta-finding in the report (the input to inc-3's exit rule). The probe didn't
    /// validate anything, so the report cannot claim a trustworthy clean result.
    #[test]
    fn validate_store_failure_yields_one_probe_integrity_meta_finding() {
        // Each failure mode drives `validate_store` over a real-anchor store (so the
        // invoker is actually called) and asserts exactly one blocking meta-finding.
        fn assert_one_meta(label: &str, invoke: &ProbeInvoker<'_>) {
            let repo = TempRoot::new(label);
            repo.commit("decisions", "valid", ADR_VALID);

            let report = validate_store(repo.path(), &schemas(), &no_delta_resolved(), invoke)
                .expect("store sweep runs even when the probe misbehaves");

            let meta: Vec<&Finding> = report
                .findings
                .iter()
                .filter(|f| f.code.starts_with("pack-probe-integrity"))
                .collect();
            assert_eq!(
                meta.len(),
                1,
                "a {label} invocation must yield exactly one pack-probe-integrity meta-finding, \
                 got {:?}",
                report.findings,
            );
            assert_eq!(
                meta[0].severity,
                Severity::Blocking,
                "{label} meta-finding blocks"
            );
            assert!(
                report.has_blocking(),
                "{label}: the report must block (the probe could not be trusted)",
            );
        }

        // crash — non-zero exit with no usable output.
        assert_one_meta("crash", &|_req| {
            Ok(ProbeRun {
                stdout: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(2) },
            })
        });
        // timeout — killed for exceeding the budget.
        assert_one_meta("timeout", &|_req| {
            Ok(ProbeRun {
                stdout: Vec::new(),
                status: ProbeRunStatus::TimedOut,
            })
        });
        // malformed-output — exit 0 but unparseable stdout.
        assert_one_meta("malformed-output", &|_req| {
            Ok(ProbeRun {
                stdout: b"not json".to_vec(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            })
        });
    }

    /// (Test 3) The snapshot scratch path resolves under [`std::env::temp_dir`] — **not**
    /// under `repo_root` nor any managed `location:` — and is **cleaned up** after the
    /// sweep returns. The invoker captures the path the engine handed it; the test
    /// inspects it.
    #[test]
    fn store_snapshot_scratch_is_under_temp_dir_and_cleaned_up() {
        let repo = TempRoot::new("scratch");
        repo.commit("decisions", "valid", ADR_VALID);

        let seen = RefCell::new(Vec::new());
        let report = validate_store(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
        )
        .expect("store sweep runs");
        // The valid-only store does not block, but the invoker WAS called.
        assert!(!report.has_blocking());

        let paths = seen.borrow();
        assert_eq!(
            paths.len(),
            1,
            "the invoker is called exactly once for the whole store"
        );
        let scratch = &paths[0];

        assert!(
            scratch.starts_with(std::env::temp_dir()),
            "the snapshot scratch path must resolve under std::env::temp_dir(), got {scratch:?}",
        );
        assert!(
            !scratch.starts_with(repo.path()),
            "the scratch path must NEVER be under repo_root / any location:, got {scratch:?}",
        );
        assert!(
            !scratch.exists(),
            "the scratch snapshot must be cleaned up after the sweep returns, still at {scratch:?}",
        );
    }

    /// A `StepSource` that resolves **no** step id — so a workflow that includes any
    /// step id surfaces a `workflow-refs.include-resolves` dangling-include finding (the
    /// task-independent break the store-scope workflow↔refs family catches).
    struct EmptyStepSource;
    impl crate::compose::StepSource for EmptyStepSource {
        fn step(&self, _id: &str) -> Option<crate::compose::StepDef> {
            None
        }
    }

    /// An empty command catalog — the membership-only command-ref path needs one but
    /// these fixtures carry no command-refs, so it is intentionally bare.
    fn empty_catalog() -> crate::compose::CommandCatalog {
        crate::compose::CommandCatalog {
            commands: BTreeMap::new(),
        }
    }

    /// (T3, the done-criterion) **One** `validate_store_families` run folds all three
    /// content families into one [`ValidationReport`]: a stale **doc↔code** anchor, a
    /// **workflow↔refs** dangling include, and a **file↔CLI-state** committed-doc drift —
    /// each surfaces its own content finding in the single report. And with a forced
    /// `pack-probe-integrity.*` meta-finding present (a crashing invoker), the three
    /// content families still surface while the report carries the meta-finding too —
    /// the input the CLI exit rule keys on (only the meta-finding flips the exit, never a
    /// content finding; `validation.md` → Exit semantics — uniform across all three families).
    #[test]
    fn validate_store_folds_three_content_families() {
        // (i) doc↔code: a committed ADR with a dangling anchor (`vanished_symbol`).
        let repo = TempRoot::new("three-families");
        repo.commit("decisions", "dangling", ADR_DANGLING);

        // (iii) file↔CLI-state: the committed ADR's recorded baseline differs from its
        // on-disk bytes → a `file-state.hash-matches` store-scope drift finding.
        let drift_path = "decisions/dangling.md";
        let mut record = FileStateRecord::new();
        record.record(drift_path, hash_bytes(b"a different baseline"));

        // (ii) workflow↔refs: a workflow whose `{{include: step:not-a-step}}` resolves
        // to no step in the (empty) source → a `workflow-refs.include-resolves` finding.
        let dangling_wf: Vec<u8> = b"---\nwhen: x\n---\n{{ include: step:not-a-step }}\n".to_vec();
        let workflows = vec![StoreWorkflow {
            id: "dangling".to_string(),
            bytes: dangling_wf,
            catalog: empty_catalog(),
        }];
        let source = EmptyStepSource;

        // --- A healthy invoker first: all three content families surface, no meta-finding.
        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &workflows,
            &source,
            &record,
        )
        .expect("three-family store sweep runs");

        let has = |code: &str| report.findings.iter().any(|f| f.code == code);
        assert!(
            has("doc-code.symbol-exists"),
            "the doc↔code family must surface the dangling anchor: {:?}",
            report.findings,
        );
        assert!(
            has("workflow-refs.include-resolves"),
            "the workflow↔refs family must surface the dangling include: {:?}",
            report.findings,
        );
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.code == "file-state.hash-matches" && f.message.contains(drift_path)),
            "the file↔CLI-state family must surface the committed-doc drift: {:?}",
            report.findings,
        );
        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.probe == "pack-probe-integrity"),
            "a healthy probe yields no meta-finding: {:?}",
            report.findings,
        );

        // --- A crashing invoker: the meta-finding is present AND the content families
        // still surface (the meta-finding never suppresses content findings).
        let crashing = |_req: &ProbeRequest| {
            Ok(ProbeRun {
                stdout: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(2) },
            })
        };
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &crashing,
            &workflows,
            &source,
            &record,
        )
        .expect("three-family store sweep runs with a crashing probe");

        assert!(
            report
                .findings
                .iter()
                .any(|f| f.probe == "pack-probe-integrity"),
            "a crashing probe must surface a pack-probe-integrity meta-finding: {:?}",
            report.findings,
        );
        let has = |code: &str| report.findings.iter().any(|f| f.code == code);
        assert!(
            has("workflow-refs.include-resolves")
                && report
                    .findings
                    .iter()
                    .any(|f| f.code == "file-state.hash-matches"),
            "the two engine-native content families must still surface alongside the \
             meta-finding (it never suppresses them): {:?}",
            report.findings,
        );

        // The record is unchanged (the file-state twin opens no write).
        assert_eq!(
            record.get(drift_path),
            Some(hash_bytes(b"a different baseline").as_str()),
            "the file-state twin must not re-baseline the drift it reports",
        );
    }

    /// A committed ADR whose `supersedes` names `to` (an `adr:<slug>` identity). Required
    /// slots are filled so the *only* possible blocking finding is a forward-ref one.
    fn adr_superseding(to: &str) -> String {
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

    /// Family 4 — cross-doc forward-ref integrity at **store scope**: a committed ADR whose
    /// `supersedes` target is absent from the committed store surfaces exactly one blocking
    /// `schema-conformance.ref-resolves` finding. This is the store-wide analog of the
    /// task-scope `ref_resolves` finalize gate — closing the asymmetry (doc↔code swept
    /// store-wide since M18, `ref-resolves` never) so the salience-independent pre-commit
    /// backstop reaches a dangling cross-doc ref in the committed store.
    #[test]
    fn validate_store_surfaces_dangling_cross_doc_supersedes() {
        let repo = TempRoot::new("dangling-supersedes");
        // adr-b supersedes `adr:absent-target`, which is NOT committed → a dangling edge.
        let body = adr_superseding("adr:absent-target");
        repo.commit("decisions", "shared-redis-cache", &body);

        // Baseline the committed doc so the file-state family stays silent — isolate Family 4.
        let mut record = FileStateRecord::new();
        record.record(
            "decisions/shared-redis-cache.md",
            hash_bytes(body.as_bytes()),
        );

        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
        )
        .expect("store sweep runs");

        let ref_resolves: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "schema-conformance.ref-resolves")
            .collect();
        assert_eq!(
            ref_resolves.len(),
            1,
            "exactly one store-wide ref-resolves finding for the dangling supersedes, got {:?}",
            report.findings,
        );
        let f = ref_resolves[0];
        assert_eq!(
            f.severity,
            Severity::Blocking,
            "store-wide ref-resolves mirrors the finalize gate's blocking severity",
        );
        assert!(
            f.message.contains("adr:absent-target"),
            "the finding must name the dangling target, got: {}",
            f.message,
        );
    }

    /// A committed store whose `supersedes` targets all resolve emits **no** ref-resolves
    /// finding — the clean store-wide path (the false-positive guard for Family 4).
    #[test]
    fn validate_store_clean_on_resolvable_cross_doc_supersedes() {
        let repo = TempRoot::new("resolvable-supersedes");
        // The target ADR is committed, so the `supersedes` edge resolves store-wide.
        repo.commit("decisions", "absent-target", &adr_superseding_target());
        let body = adr_superseding("adr:absent-target");
        repo.commit("decisions", "shared-redis-cache", &body);

        let mut record = FileStateRecord::new();
        record.record(
            "decisions/shared-redis-cache.md",
            hash_bytes(body.as_bytes()),
        );
        record.record(
            "decisions/absent-target.md",
            hash_bytes(adr_superseding_target().as_bytes()),
        );

        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
        )
        .expect("store sweep runs");

        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.code == "schema-conformance.ref-resolves"),
            "a resolvable cross-doc supersedes must not surface a ref-resolves finding: {:?}",
            report.findings,
        );
    }

    /// A committed ADR with no forward ref — the resolvable `supersedes` target.
    fn adr_superseding_target() -> String {
        "\
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
"
        .to_string()
    }

    /// The [`schemas`] set with the `adr` doctype **shadowed** by a stricter schema that
    /// adds a required `owner` header field — the engine-level analog of a project-layer
    /// schema shadow. A committed ADR conformant under the un-shadowed schema is
    /// non-conformant under this one **with its bytes unchanged** (the silent-drift case
    /// the fifth content family detects, which the hash-only file↔CLI-state family cannot).
    fn shadowed_schemas() -> BTreeMap<String, Schema> {
        const ADR_SHADOW_YAML: &[u8] = b"\
type: adr
location: decisions/
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
      - { id: supersedes, type: ref, to: adr, card: \"0..*\", inverse: superseded-by }
      - { id: cites-code, type: code-anchor }
      - { id: owner, type: string }
  - id: context
    slot: { hint: Forces. }
  - id: decision
    slot: { hint: What. }
  - id: consequences
    slot: { hint: Effects. }
";
        let mut m = schemas();
        m.insert(
            "adr".to_string(),
            load_schema_with_types(ADR_SHADOW_YAML, &dev_pack_field_types())
                .expect("adr shadow schema loads"),
        );
        m
    }

    /// The **fifth content family** (`validation.md` → Store-scope schema-conformance): a
    /// committed doc made non-conformant by a *schema-shape change* with its **bytes
    /// unchanged** is re-parsed against the current resolved schema and its break surfaced
    /// at store scope — the silent-drift hole the hash-only file↔CLI-state family (Family 3)
    /// cannot see. The shadow adds a required `owner` header field the committed ADR's
    /// (unchanged) bytes never carried, so `schema-conformance.required-field-present`
    /// surfaces; under the un-shadowed schema the same store is fully conformant and
    /// surfaces none. The family is wired into [`validate_store_families`] only.
    #[test]
    fn validate_store_surfaces_schema_conformance_break_under_shadow() {
        let repo = TempRoot::new("conformance-shadow");
        // A committed ADR conformant under the real adr schema — its bytes never change.
        repo.commit("decisions", "valid", ADR_VALID);

        // Baseline the doc so the hash-only file↔CLI-state family stays silent: this proves
        // the break is caught by RE-PARSE, not a hash mismatch (the bytes match baseline).
        let mut record = FileStateRecord::new();
        record.record("decisions/valid.md", hash_bytes(ADR_VALID.as_bytes()));

        // (i) Under the un-shadowed schema the store is fully conformant — no break.
        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
        )
        .expect("store sweep runs over a conformant store");
        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.probe == "schema-conformance"),
            "a fully conformant store surfaces no schema-conformance break: {:?}",
            report.findings,
        );

        // (ii) The schema shadow adds a required `owner` header field the committed ADR's
        // unchanged bytes do not carry → the silent-drift case the re-parse detects.
        let report = validate_store_families(
            repo.path(),
            &shadowed_schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
        )
        .expect("store sweep runs under the schema shadow");
        let breaks: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "schema-conformance.required-field-present")
            .collect();
        assert_eq!(
            breaks.len(),
            1,
            "the re-parse must surface exactly one required-field-present break for the \
             now-non-conformant committed ADR: {:?}",
            report.findings,
        );
        assert!(
            breaks[0].message.contains("owner") && breaks[0].message.contains("status"),
            "the break must name the missing required `owner` field: {}",
            breaks[0].message,
        );
        // Read-only at store scope: the file-state record is untouched (no re-baseline).
        assert_eq!(
            record.get("decisions/valid.md"),
            Some(hash_bytes(ADR_VALID.as_bytes()).as_str()),
            "the store sweep opens no record write",
        );
    }

    /// (Test 4) A store with **no** `code-anchor` leaf never calls the invoker and writes
    /// no snapshot — the report is empty and does not block (the omitting-context inert
    /// path; the invoker panics if it runs). The list-valued / loud-guard half is the
    /// enumeration's own test (inc-1); this asserts the entry point's empty-surface skip.
    #[test]
    fn no_anchor_store_skips_the_invoker() {
        const ADR_NO_ANCHOR: &str = "\
---
status: accepted
date: 2026-05-23
---

# No-anchor decision

## Context
Forces.

## Decision
Decided.

## Consequences
Effects.
";
        let repo = TempRoot::new("no-anchor");
        repo.commit("decisions", "plain", ADR_NO_ANCHOR);

        let report = validate_store(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &(|_req: &ProbeRequest| {
                panic!("invoker must not run when the store carries no anchor")
            }),
        )
        .expect("store sweep runs");
        assert!(
            report.findings.is_empty(),
            "a no-anchor store yields an empty report, got {:?}",
            report.findings,
        );
        assert!(!report.has_blocking());
    }

    /// (T2b) The store-sweep `workflow↔refs` family resolves each workflow's command-refs
    /// against ITS OWN [`StoreWorkflow::catalog`] — **per-definition, not one flat catalog**
    /// (`multi-pack.md` → Pack-local body-reference resolution). Two workflows, each with a
    /// lone `{{cli.<id>}}` body line, fed in the SAME sweep with **divergent** catalogs:
    ///
    /// - workflow A references `alpha-cmd` and carries a catalog that DEFINES `alpha-cmd`
    ///   (but NOT `beta-cmd`) → clean, no finding.
    /// - workflow B references `beta-cmd` and carries a catalog that DEFINES `beta-cmd`
    ///   (but NOT `alpha-cmd`) → clean, no finding.
    ///
    /// Under a single flat catalog (the pre-fix bug) NEITHER catalog is a superset, so
    /// whichever flat catalog were chosen, the OTHER workflow's ref would surface a false
    /// `command-ref-resolves` finding. Per-workflow catalogs make both clean. And the
    /// no-over-correction guard: workflow C references `gamma-cmd` with an EMPTY own
    /// catalog → it IS still caught (a genuinely-dangling ref in the workflow's own pack).
    #[test]
    fn store_sweep_resolves_command_refs_per_workflow_catalog() {
        // A catalog defining exactly one command id (the minimal shape
        // `load_command_catalog` accepts: an `args` list + a `hint`).
        let catalog_with = |id: &str| {
            let yaml = format!(
                "commands:\n  - id: {id}\n    command: jigc\n    args: [\"noop\"]\n    hint: \"{id}\"\n"
            );
            crate::compose::load_command_catalog(yaml.as_bytes())
                .unwrap_or_else(|f| panic!("catalog for `{id}` loads: {f:?}"))
        };
        // A workflow that includes one step (the `{{cli.X}}` ref lives in a STEP body, not
        // the workflow body, which is include-only) — the step id is the workflow id.
        let workflow_with = |id: &str, catalog: crate::compose::CommandCatalog| StoreWorkflow {
            id: id.to_string(),
            bytes: format!("---\nwhen: x\n---\n{{{{ include: step:{id} }}}}\n").into_bytes(),
            catalog,
        };
        // A step source mapping each step id to a body carrying its own `{{cli.<id>-cmd}}`
        // ref — so the expanded step body reaches the membership check (the engine-test
        // `MapSource` idiom, local to this test).
        struct CliStepSource;
        impl crate::compose::StepSource for CliStepSource {
            fn step(&self, id: &str) -> Option<crate::compose::StepDef> {
                let body = format!("---\n---\n{{{{ cli.{id}-cmd }}}}\n");
                Some(crate::compose::load_step_def(id, body.as_bytes()).expect("step loads"))
            }
        }

        let repo = TempRoot::new("per-workflow-catalog");
        let record = FileStateRecord::new();
        let source = CliStepSource;

        // A + B: each step's `{{cli.<id>-cmd}}` ref resolves against ITS workflow's OWN
        // catalog (neither catalog is a superset of the other).
        let workflows = vec![
            workflow_with("alpha", catalog_with("alpha-cmd")),
            workflow_with("beta", catalog_with("beta-cmd")),
        ];
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &(|_req: &ProbeRequest| panic!("no-anchor store: invoker must not run")),
            &workflows,
            &source,
            &record,
        )
        .expect("per-workflow-catalog sweep runs");
        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.code == "workflow-refs.command-ref-resolves"),
            "each workflow's command-ref must resolve against its own catalog — no false \
             command-ref finding: {:?}",
            report.findings,
        );

        // C: a genuinely-dangling ref in the workflow's OWN (empty) catalog IS caught —
        // the fix must not over-correct into silence.
        let dangling = vec![workflow_with("gamma", empty_catalog())];
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &(|_req: &ProbeRequest| panic!("no-anchor store: invoker must not run")),
            &dangling,
            &source,
            &record,
        )
        .expect("dangling-ref sweep runs");
        assert!(
            report.findings.iter().any(|f| {
                f.code == "workflow-refs.command-ref-resolves" && f.message.contains("gamma-cmd")
            }),
            "a ref absent from the workflow's own catalog must still be caught: {:?}",
            report.findings,
        );
    }

    /// A committed ADR citing `anchor` from its header `cites-code` — the doc a task
    /// never opens but whose citation its code change may dangle.
    fn adr_citing(anchor: &str) -> String {
        format!(
            "---\nstatus: accepted\ndate: 2026-05-23\ncites-code: {anchor}\n---\n\n\
             # Cited decision\n\n## Context\nForces.\n\n## Decision\nDecided.\n\n\
             ## Consequences\nEffects.\n"
        )
    }

    /// A repo-relative change-set from a path slice.
    fn change_set(paths: &[&str]) -> BTreeSet<String> {
        paths.iter().map(|p| p.to_string()).collect()
    }

    /// A code tree at `<temp>/<rel>` for each `(rel, content)` — the materialized index /
    /// base root the content-aware invoker resolves anchors against. Self-cleaning.
    fn code_tree(tag: &str, files: &[(&str, &str)]) -> TempRoot {
        let tree = TempRoot::new(tag);
        for (rel, content) in files {
            let dest = tree.path().join(rel);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).expect("mk code dir");
            }
            std::fs::write(dest, content).expect("write code file");
        }
        tree
    }

    /// A **content-aware** `doc-code` invoker double — unlike [`dangling_aware_invoker`] it
    /// reads each anchor's file from the snapshot's `working_tree_root` and reports dangling
    /// iff the `#symbol` is absent from that file's bytes. This lets a test give the **base**
    /// (HEAD) and **index** trees different contents, exercising the newly-dangled comparison
    /// the keyword-matching double cannot.
    fn content_aware_invoker(
        seen: &RefCell<Vec<PathBuf>>,
    ) -> impl Fn(&ProbeRequest) -> std::io::Result<ProbeRun> + '_ {
        move |req| {
            seen.borrow_mut()
                .push(req.effective_state.snapshot_path.clone());
            let snapshot: EffectiveStateSnapshot =
                serde_json::from_slice(&std::fs::read(&req.effective_state.snapshot_path)?)?;
            let findings: Vec<Finding> = snapshot
                .anchors
                .iter()
                .filter_map(|a| {
                    let (file, sym) = match a.anchor_value.split_once('#') {
                        Some((f, s)) => (f, Some(s)),
                        None => (a.anchor_value.as_str(), None),
                    };
                    let path = snapshot.working_tree_root.join(file);
                    let dangles = match sym {
                        Some(s) => std::fs::read_to_string(&path)
                            .map(|c| !c.contains(s))
                            .unwrap_or(true),
                        None => !path.exists(),
                    };
                    dangles.then(|| {
                        Finding::graded(
                            Severity::Blocking,
                            "doc-code.symbol-exists",
                            format!("dangling: {}", a.anchor_value),
                            Some(Location::addressed(a.address.clone(), 1, 1)),
                            None,
                        )
                    })
                })
                .collect();
            Ok(ProbeRun {
                stdout: serde_json::to_vec(&ProbeResponse::new(findings)).unwrap(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            })
        }
    }

    fn blast_blocks(r: &ValidationReport) -> bool {
        r.findings
            .iter()
            .any(|f| f.code == "doc-code.symbol-exists" && f.severity == Severity::Blocking)
    }

    /// (Phase-2 floor — the universal `finalize` floor, with the newly-dangled guard) A task
    /// that changes a code file dangles a `code-anchor` in a **committed** doc it never
    /// opened, and `validate_task` (== what `finalize` gates on) **blocks** — but ONLY when
    /// the anchor *newly* dangled (resolved at HEAD, gone at the staged index). Pre-existing
    /// drift in a touched file, an unrelated file, and a no-code task all stay clean — no
    /// cross-task false attribution (`validation.md` → Scope = effective state).
    #[test]
    fn finalize_floor_blocks_newly_dangled_not_pre_existing_drift() {
        let repo = TempRoot::new("blast-repo");
        repo.commit(
            "decisions",
            "cited",
            &adr_citing("src/foo.rs#vanished_symbol"),
        );
        // Empty task working area: the only anchor reaching the probe is the committed one.
        let task = TempRoot::new("blast-task");
        let no_op_tracked = |_: &str| false;
        // The staged index: `src/foo.rs` no longer defines `vanished_symbol`.
        let index = code_tree("blast-index", &[("src/foo.rs", "pub fn other() {}\n")]);

        let run = |changed: &BTreeSet<String>, base: &Path| {
            let seen = RefCell::new(Vec::new());
            validate_task(
                task.path(),
                &schemas(),
                &mut FileStateRecord::new(),
                repo.path(),
                index.path(),
                repo.path(),
                "HEAD",
                &no_delta_resolved(),
                &content_aware_invoker(&seen),
                &no_op_tracked,
                changed,
                base,
            )
            .expect("sweep runs")
        };
        let changed = change_set(&["src/foo.rs"]);

        // (1) NEWLY dangled — HEAD has the symbol, the index doesn't → BLOCK (the hole closed).
        let base_clean = code_tree(
            "blast-base-ok",
            &[("src/foo.rs", "pub fn vanished_symbol() {}\n")],
        );
        assert!(
            blast_blocks(&run(&changed, base_clean.path())),
            "a NEWLY-dangled committed anchor MUST block",
        );

        // (4) PRE-EXISTING drift — HEAD already lacked the symbol → NOT this task's fault → clean.
        let base_drifted = code_tree("blast-base-drift", &[("src/foo.rs", "pub fn other() {}\n")]);
        assert!(
            !blast_blocks(&run(&changed, base_drifted.path())),
            "pre-existing drift in a touched file must NOT be re-attributed to this task",
        );

        // (2) Unrelated file changed → not in the blast radius → clean (base irrelevant).
        assert!(
            !blast_blocks(&run(&change_set(&["src/other.rs"]), base_clean.path())),
            "an anchor whose file the task didn't change must NOT be re-attributed",
        );

        // (3) No code changed → blast radius inert → clean.
        assert!(
            !blast_blocks(&run(&BTreeSet::new(), base_clean.path())),
            "a task that changes no code must NOT block on committed drift",
        );
    }

    /// The path-normalization fix (Codex P1 false-negative): a committed anchor written with
    /// a non-canonical `./` path must still be caught — the change-set is canonical
    /// (`src/foo.rs`), so without lexically normalizing both sides the anchor would silently
    /// escape the floor.
    #[test]
    fn floor_normalizes_non_canonical_anchor_paths() {
        let repo = TempRoot::new("blast-norm");
        repo.commit(
            "decisions",
            "cited",
            &adr_citing("./src/foo.rs#vanished_symbol"),
        );
        let task = TempRoot::new("blast-norm-task");
        let index = code_tree("norm-index", &[("src/foo.rs", "pub fn other() {}\n")]);
        let base = code_tree(
            "norm-base",
            &[("src/foo.rs", "pub fn vanished_symbol() {}\n")],
        );
        let seen = RefCell::new(Vec::new());
        let report = validate_task(
            task.path(),
            &schemas(),
            &mut FileStateRecord::new(),
            repo.path(),
            index.path(),
            repo.path(),
            "HEAD",
            &no_delta_resolved(),
            &content_aware_invoker(&seen),
            &|_: &str| false,
            &change_set(&["src/foo.rs"]),
            base.path(),
        )
        .expect("sweep runs");
        assert!(
            blast_blocks(&report),
            "a non-canonical `./src/foo.rs` anchor must still be caught, got {:?}",
            report.findings,
        );
    }

    /// (Codex round-2 P2b) "Resolved at base" must mean **affirmatively clean** (no base
    /// finding), not merely "not blocking at base". An anchor that is only *uncheckable* at
    /// base (an advisory — a symlink-anchor / unsupported-language) is NOT proof the symbol
    /// resolved there, so its index dangle must NOT be attributed to this task.
    #[test]
    fn base_advisory_is_not_resolved_so_no_false_attribution() {
        let repo = TempRoot::new("adv-repo");
        repo.commit(
            "decisions",
            "cited",
            &adr_citing("src/foo.rs#vanished_symbol"),
        );
        let task = TempRoot::new("adv-task");
        let index = code_tree("adv-index", &[("src/foo.rs", "pub fn other() {}\n")]);
        let base = code_tree("adv-base", &[("src/foo.rs", "anything\n")]);
        let base_path = base.path().to_path_buf();
        // Against the BASE tree: an uncheckable advisory (as a symlink would yield). Against
        // the INDEX tree: a blocking dangle. The advisory must not count as resolved-at-base.
        let invoker = |req: &ProbeRequest| -> std::io::Result<ProbeRun> {
            let snapshot: EffectiveStateSnapshot =
                serde_json::from_slice(&std::fs::read(&req.effective_state.snapshot_path)?)?;
            let is_base = snapshot.working_tree_root == base_path;
            let findings: Vec<Finding> = snapshot
                .anchors
                .iter()
                .map(|a| {
                    let (sev, code) = if is_base {
                        (Severity::Advisory, "doc-code.symlink-anchor")
                    } else {
                        (Severity::Blocking, "doc-code.symbol-exists")
                    };
                    Finding::graded(
                        sev,
                        code,
                        a.anchor_value.clone(),
                        Some(Location::addressed(a.address.clone(), 1, 1)),
                        None,
                    )
                })
                .collect();
            Ok(ProbeRun {
                stdout: serde_json::to_vec(&ProbeResponse::new(findings)).unwrap(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            })
        };
        let report = validate_task(
            task.path(),
            &schemas(),
            &mut FileStateRecord::new(),
            repo.path(),
            index.path(),
            repo.path(),
            "HEAD",
            &no_delta_resolved(),
            &invoker,
            &|_: &str| false,
            &change_set(&["src/foo.rs"]),
            base.path(),
        )
        .expect("sweep runs");
        assert!(
            !blast_blocks(&report),
            "an anchor only uncheckable (advisory) at base must NOT be attributed, got {:?}",
            report.findings,
        );
    }

    /// (Codex round-2 P2a) A **base probe failure** (a base comparison that did not actually
    /// run — crash/timeout/malformed) must be **surfaced** as a blocking `pack-probe-integrity`
    /// finding, never swallowed: a broken base probe blocks loudly rather than silently
    /// letting the newly-dangled comparison pass.
    #[test]
    fn base_probe_failure_is_surfaced_not_swallowed() {
        let repo = TempRoot::new("bfail-repo");
        repo.commit(
            "decisions",
            "cited",
            &adr_citing("src/foo.rs#vanished_symbol"),
        );
        let task = TempRoot::new("bfail-task");
        let index = code_tree("bfail-index", &[("src/foo.rs", "pub fn other() {}\n")]);
        let base = code_tree("bfail-base", &[("src/foo.rs", "x\n")]);
        let base_path = base.path().to_path_buf();
        // The base probe CRASHES (non-zero exit, no output); the index probe blocks normally.
        let invoker = |req: &ProbeRequest| -> std::io::Result<ProbeRun> {
            let snapshot: EffectiveStateSnapshot =
                serde_json::from_slice(&std::fs::read(&req.effective_state.snapshot_path)?)?;
            if snapshot.working_tree_root == base_path {
                return Ok(ProbeRun {
                    stdout: Vec::new(),
                    status: ProbeRunStatus::Exited { code: Some(2) },
                });
            }
            let findings: Vec<Finding> = snapshot
                .anchors
                .iter()
                .map(|a| {
                    Finding::graded(
                        Severity::Blocking,
                        "doc-code.symbol-exists",
                        "dangling".to_string(),
                        Some(Location::addressed(a.address.clone(), 1, 1)),
                        None,
                    )
                })
                .collect();
            Ok(ProbeRun {
                stdout: serde_json::to_vec(&ProbeResponse::new(findings)).unwrap(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            })
        };
        let report = validate_task(
            task.path(),
            &schemas(),
            &mut FileStateRecord::new(),
            repo.path(),
            index.path(),
            repo.path(),
            "HEAD",
            &no_delta_resolved(),
            &invoker,
            &|_: &str| false,
            &change_set(&["src/foo.rs"]),
            base.path(),
        )
        .expect("sweep runs");
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.probe == "pack-probe-integrity"),
            "a base probe failure must surface a pack-probe-integrity finding, got {:?}",
            report.findings,
        );
    }

    /// The **path-locality guard** of the blast radius (`validation.md` → Scope = effective
    /// state; [`anchor_file`]). Change-set scoping is *complete* only because a `code-anchor`
    /// resolves path-locally — its scoped file is the literal pre-`#` path. This pins that
    /// extraction so a future non-local resolver (one where changing file B could dangle an
    /// anchor into file A) trips a red test instead of silently leaking drift.
    #[test]
    fn path_locality_scopes_blast_radius_by_anchor_file() {
        assert_eq!(anchor_file("src/a.rs#Foo"), "src/a.rs");
        assert_eq!(anchor_file("dir/sub/b.ts#Bar"), "dir/sub/b.ts");
        // A bare path (no `#symbol`) is itself the file.
        assert_eq!(anchor_file("src/c.css"), "src/c.css");
        // The split is on the FIRST `#`, byte-identical to the probe's `split_anchor`.
        assert_eq!(anchor_file("src/d.rs#a#b"), "src/d.rs");
        // A committed anchor is in scope iff its file is the literal path the task changed.
        let changed = change_set(&["src/a.rs"]);
        assert!(changed.contains(anchor_file("src/a.rs#Foo")));
        assert!(!changed.contains(anchor_file("src/b.rs#Foo")));
    }
}
