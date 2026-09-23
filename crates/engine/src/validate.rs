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
use crate::file_state::FileStateRecord;
use crate::finding::{Finding, Location, Route, Severity};
use crate::parse::{Document, ParsedItem, ParsedSection, parse_sections};
use crate::probe::{EffectiveStateSnapshot, ProbeRequest, ProbeRun, RootKind, ingest_probe_run};
use crate::result::ValidationReport;
use crate::schema::{Field as SchemaField, FieldType, Schema, Section, SectionBody, SetKind};
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
pub(crate) const SNAPSHOT_FILE: &str = "probe-snapshot.json";

/// The snapshot the **base** (HEAD) sweep of the newly-dangled comparison materializes —
/// distinct from [`SNAPSHOT_FILE`] so the index and base probes never clobber each other's
/// snapshot. Same gitignored working area; never committed.
pub(crate) const BASE_SNAPSHOT_FILE: &str = "base-probe-snapshot.json";

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

/// The CLI-supplied git **history** predicate the engine threads through [`validate_task`]
/// into [`crate::file_state::reconcile_committed_store`] → [`crate::file_state::detect_rename`]
/// — a `Fn(&str) -> bool` taking a **repo-relative** path and answering whether HEAD carries
/// any history for it (`git log HEAD -1 -- <path>` is non-empty). Built on the same shell-free
/// seam as [`TrackedPredicate`]: the CLI owns the `git log` shell-out, the engine only consults
/// the boolean.
///
/// It gates the file-state weak-signal severity (M45, Decision 7; `design/storage.md` →
/// Derived caches): the file↔state hashes carry no stamp and no rebuild path, so a checkout
/// that moves underneath the gitignored cache (`git reset --hard` / branch switch / rebase
/// past a doc's creating commit) leaves a recorded baseline pointing at a path that no longer
/// exists. When HEAD has **no** history for the path, nothing was deleted — the dangling
/// baseline downgrades to advisory; when it **has** history, the path was genuinely deleted
/// and keeps blocking.
pub type HistoryPredicate<'a> = dyn Fn(&str) -> bool + 'a;

/// Validate one task working area — the single engine both `task validate` and
/// `finalize` phase 2 call (`validation.md` → How it gates `finalize`: one engine,
/// two entry points, so what `validate` reports and what `finalize` blocks on can
/// never diverge). No git, no commit.
///
/// Resolves the working area's staged doc instances (`<dir>/docs/*.md`, each named
/// `<type>:<slug>.md`), then over each instance runs the two MVP task-scope probes
/// and aggregates their severity-classified findings into a [`ValidationReport`]:
///
/// - **`file-state`** — a **persisted** instance gets the staged-copy advisory at its
///   repo-real committed destination ([`crate::file_state::staged_copy_finding`]);
///   `record` is neither consulted nor advanced for staged instances (M43 A14 — a
///   copied-in committed doc's in-flight edit must not drift). A transient-sink or
///   unknown-type instance yields no `file-state.*` finding at all.
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
/// `_tracked` is the CLI-supplied git tracked-status predicate ([`TrackedPredicate`]) —
/// **retained on the signature but no longer consulted here** (M45 Inc 8 T2): the #5
/// owner-artifact gate it fed moved out of this shared phase-2 entry to a post-stage site
/// ([`owner_artifacts_gate`], driven by the CLI finalize transaction), so `validate_task`
/// keeps the parameter for the shared two-entry-point signature while the gate relocates.
///
/// `history` is the CLI-supplied git history predicate ([`HistoryPredicate`]) the committed-
/// store rename detector consults to grade a **dangling baseline** — a recorded managed-doc
/// path now absent on disk with no content-matching suspect (M45, Decision 7). It is queried
/// only inside that already-cold path (a recorded-but-missing doc), so a healthy task never
/// shells out for it: history present (the path was genuinely deleted) keeps the blocking
/// weak-signal finding; history empty (the checkout moved underneath the gitignored cache)
/// downgrades to advisory with a `jigc unmanage` prune route.
///
/// `conflict` is the CLI-supplied [`ConflictBlock`](crate::file_state::ConflictBlock) the
/// committed-store sweep hands to its `DRIFTED + TOUCHED` classifier (M47 inc-2 / T4). The
/// engine sees only the working area's *path*, never whose it is — a per-task gate passes
/// [`ConflictBlock::task`](crate::file_state::ConflictBlock::task) with the real task id, the
/// milestone join gate passes its own (no single task owns the merged area) — so the naming
/// and the way out come from the caller that knows, never a placeholder minted here.
///
/// `adoption` is the CLI-supplied [`AdoptionInputs`] the same committed-store sweep hands to
/// its `UNKNOWN` + non-conformant classifier (M48 Inc 4 / T1), so a **never-adopted foreign**
/// file squatting at a managed home draws the store family's
/// `schema-conformance.unadopted-instance` and its adoption route here too, instead of being
/// graded an unvetted *managed* doc. The three facts it carries — the manifest version map,
/// the shipped prior shapes, the `migrate-<ty>`-bearing doctypes — are pack facts the engine
/// cannot produce; an empty set leaves this arm byte-identical to its pre-M48 behaviour.
///
/// `live_record` is the CLI-supplied [`LiveRecord`](crate::file_state::LiveRecord) the same
/// committed-store sweep hands to its dangling-baseline arm (M52 Inc 10 / T6): the committed
/// record of the work unit the validated task belongs to, if any. A sub-task stands at its
/// milestone's **base pin**, which by construction predates the record commit, so the record
/// reads absent-and-history-less there while it is live — and the shipped prune route would
/// unmanage the state the milestone is run from. The engine cannot know which work unit owns
/// a task, so the caller that does names the path;
/// [`LiveRecord::none`](crate::file_state::LiveRecord::none) leaves the arm byte-identical to
/// its shipped classification.
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
    _tracked: &TrackedPredicate<'_>,
    history: &HistoryPredicate<'_>,
    changed_code: &BTreeSet<String>,
    base_code_tree_root: &Path,
    conflict: &crate::file_state::ConflictBlock,
    adoption: &AdoptionInputs<'_>,
    live_record: &crate::file_state::LiveRecord,
) -> std::io::Result<ValidationReport> {
    let mut findings = Vec::new();
    for filename in staged_instances(dir)? {
        let bytes = std::fs::read(dir.join(DOCS_DIR).join(&filename))?;

        // The per-instance display identity (M43 A14; `surface-contract.md` → law 1:
        // every printed path is repo-real or a typed identity): a persisted instance
        // displays at its repo-relative committed destination, a transient-sink or
        // unknown-type one at its `<type>:<slug>` identity — never the
        // `docs/<type>:<slug>.md` working-area fiction the pre-M43 sweep printed.
        let display = staged_display(&filename, schemas);

        // `file-state` over this one instance: a persisted instance gets the
        // staged-copy advisory at its repo-real destination, emitted directly — the
        // record is neither consulted nor advanced (a copied-in committed doc's
        // in-flight edit legitimately differs from the committed baseline, so keying
        // the destination against the record would mint false blocking drift). A
        // transient-sink or unknown-type instance has no committed file to baseline
        // or drift, so no `file-state.*` finding fires at all — the A14 root cause.
        if let Some(dest) = staged_destination(&filename, schemas) {
            findings.push(crate::file_state::staged_copy_finding(&dest));
        }

        // `schema-conformance` over the instance, resolving its type from the
        // `<type>:<slug>.md` filename. A non-UTF-8 instance can't be a managed
        // Markdown doc; the parser owns that, so we require a UTF-8 read here.
        let source = String::from_utf8_lossy(&bytes);
        findings.extend(conformance_for(&filename, schemas, &display, &source));

        // The #5 owner-artifact presence gate is deliberately NOT run here (M45 Inc 8 T2,
        // `DECISIONS.md` → Decision 6). It asserts the named artifact is durably **tracked**
        // — a state phase-5 staging can satisfy, but only *after* the stage — so it moved out
        // of `validate_task` (the phase-2 site both `task validate` and finalize share) to a
        // dedicated post-stage site the CLI finalize transaction re-invokes over the
        // just-staged index ([`owner_artifacts_gate`]; `design/finalize.md` → 5. Stage).
        //
        // That relocation stands, and so does this omission: the gate is **not** an entry in
        // this shared phase-2 sweep. What M47 Inc 4 / T2 changed is the *CLI* wiring above it
        // — `jigc task validate` re-invokes the gate itself, once, under a constant-true
        // `tracked` predicate, so the six staging-independent causes preview while cause 7
        // (untracked) remains structurally unreachable from that door and stays post-stage
        // (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1). Running it here instead would
        // re-introduce exactly the pre-stage `tracked` false-positive Decision 6 retired,
        // because finalize's preflight shares this entry.
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
        history,
        conflict,
        adoption,
        live_record,
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

/// One family of the store-scope sweep `jigc validate` drives — the taxonomy's
/// **one home**, and the set [`crate::validate::STORE_FAMILIES`] is read at.
pub struct StoreFamily {
    /// The family's name, as `design/validation.md`'s taxonomy names it. Deliberately
    /// **not** a number: the arrival-order numbering is exactly what drifted (the code
    /// called inverse-cardinality *"Family 5"* while the design doc called
    /// schema-conformance *"the fifth family"*), so the registry carries sweep order
    /// and no ordinal at all.
    pub name: &'static str,
    /// What this family re-checks, in one clause — rendered verbatim into
    /// `jigc validate --help`, so it carries neither `;` nor `.` (the list's own
    /// separators).
    pub checks: &'static str,
}

/// The **store-sweep family axis** — every content family `jigc validate` reports, in
/// sweep order (M51 Increment 9, EC-11).
///
/// The seven are [`validate_store_families`]' own walks, with one carve-out stated
/// rather than left to the reader: the **orphan-strand** arm of the file↔CLI-state
/// member is produced CLI-side (it shells to `git ls-files`, which the domain-empty
/// engine never does), so it is named inside that member's clause instead of earning a
/// member of its own. The verb's other CLI-side advisory — store provenance,
/// `store-version.binary-mismatch` — is **not** a content family and is not a member
/// (`design/validation.md` → Store-scope re-validation).
///
/// It exists because the set had **no code-side home**: it lived in per-call comments
/// in this file and in prose in `design/validation.md`, and `jigc validate --help` —
/// the surface an agent actually reads before running the sweep — named exactly one
/// of them (*"Re-check every committed doc's code anchors"*, the doc↔code family).
/// So the door under-stated its own sweep by six families, and the two homes had
/// already drifted apart on the numbering (above).
///
/// The help is **generated** from this table rather than restating it — an unfenced
/// second home for one fact is the defect, not the fix — and
/// `crates/cli/tests/help_truth.rs` compares the emitted help bytes to this set as an
/// ordered equality, so a family missing from the help and a family in the help this
/// set does not carry are both red.
pub const STORE_FAMILIES: &[StoreFamily] = &[
    StoreFamily {
        name: "doc↔code",
        checks: "every committed doc's `code-anchor` leaves re-resolved against the \
                 working tree (the one subprocess probe)",
    },
    StoreFamily {
        name: "workflow↔refs",
        checks: "every cascade-resolved workflow definition's task-independent \
                 referential integrity, against its own origin pack",
    },
    StoreFamily {
        name: "file↔CLI-state",
        checks: "every committed managed doc's bytes against its recorded baseline, \
                 detect-without-absorb, plus the move a bare `git mv` left \
                 recorded-but-missing and the committed doc stranded outside its \
                 doctype's resolved home",
    },
    StoreFamily {
        name: "forward-ref integrity",
        checks: "every committed cross-doc `ref` edge's target resolving in the \
                 committed store",
    },
    StoreFamily {
        name: "schema-completeness",
        checks: "each committed doc below an `inverse-card` minimum its doctype \
                 declares (advisory)",
    },
    StoreFamily {
        name: "mention integrity",
        checks: "every `#<type>:<slug>` managed mention in committed slot prose \
                 resolving (advisory)",
    },
    StoreFamily {
        name: "schema-conformance",
        checks: "every committed instance re-parsed against the current resolved \
                 schema: presence, value and version currency, plus the two adoption \
                 advisories",
    },
];

/// One committed instance the CLI found at a **recorded prior home** of its doctype — a home
/// the versioned snapshot store says that doctype used to declare (M52 completion audit, fix
/// 2; `design/validation.md` → Store-scope schema-conformance).
///
/// The engine cannot derive these and must not try: the prior homes live in the pack's
/// snapshot store and resolve through the `docs-root` / `placement-root` cascade knobs, both
/// of which are the CLI's to read (the determinism boundary — the engine ships empty of
/// content by invariant). So the CLI enumerates them with
/// `cli::orphan::prior_home_instances` and hands them in, exactly as it hands in `versions`
/// and `priors`, and the fifth content family adjudicates them beside the resolved-home
/// census — same parse, same discriminator, same version-aware route.
///
/// The CLI hands in the **unmigrated** ones and only those (`cli::orphan` states the three
/// stamp readings and why a document *at* version at a prior home is a different family's), so
/// they need no code of their own — [`SCHEMA_VERSION_CURRENT_CODE`] is already the true
/// statement about them, and it already routes at `jigc migrate-corpus`, the verb that lands
/// them at the current home.
pub struct PriorHomeInstance {
    /// The doctype — the key the family adjudicates the instance under.
    pub ty: String,
    /// The `<type>:<slug>` identity the document carries **at that home**, derived by
    /// [`crate::index::instance_slug`] against the prior shape.
    pub identity: String,
    /// The absolute path of the committed file.
    pub path: std::path::PathBuf,
}

/// The store-scope **family sweep** — the [`validate_store`] superset the top-level
/// `jigc validate` drives, folding every read-only store target into one
/// [`ValidationReport`] (`validation.md` → Completing the envelope: host the families
/// under the uniform exit rule). **Task-less and read-only** throughout: no working area,
/// no `FileStateRecord` write, no edge-index mutation.
///
/// **Which families run is [`STORE_FAMILIES`], not a count here.** This comment read *"the
/// store-scope **three-family** sweep"* and then listed three, from M20 until M51, while
/// the body below has grown to **seven** — and `jigc validate --help`, generated from that
/// registry since M51, is the surface the drift was actually read off (M51 Increment 9,
/// EC-11).
///
/// The first three are detailed here because they are the ones whose
/// determinism-boundary inputs the CLI has to feed in (the seam this signature exists
/// for); the rest are engine-internal walks, commented at their call site:
///
/// - **doc↔code** — every committed doc's `code-anchor` leaves resolved against the working tree via the CLI-supplied subprocess `invoke_doc_code` seam (the [`validate_store`] body, lifted to [`store_doc_code`]). The only family that can raise a `pack-probe-integrity.*` meta-finding (it is the one subprocess probe).
/// - **workflow↔refs** — each cascade-resolved workflow definition (the CLI enumerates + reads them, address-sorted, feeding each as a [`StoreWorkflow`] bundling its id, raw bytes, and **origin-pack** command catalog) run through the **task-independent** store-scope checks ([`crate::compose::workflow_refs_store`]): `include-resolves`, `include-cycle-absent`, `body-include-only`, the three marker-shadow checks, `fan-out-join-paired`, the **catalog-membership-only** command-ref path, and the **doctype-membership-only** schema-ref path (`schema-ref-resolves`, M43 — resolved against the **composed cascade's** doctype set derived from `schemas`, deliberately NOT per-origin: a methodology step legitimately solicits a dev doctype, `surface-contract.md` → The schema projection). The task-data checks stay at `jigc start`. `workflow_source` is the CLI's layer-aware [`StepSource`](crate::compose::StepSource), **scoped per-workflow** to that definition's origin pack via [`StepSource::scope_to_workflow`](crate::compose::StepSource::scope_to_workflow) so a loser-pack workflow's includes + command-refs resolve against ITS OWN pack, never the precedence-winner's catalog (`multi-pack.md` → Pack-local body-reference resolution: `command-ref-resolves` and the include checks fire **per-definition against that definition's own pack**). For a single pack each origin *is* the one pack, so the resolution is byte-identical to a flat catalog (the no-composition floor).
/// - **file↔CLI-state** — the read-only committed-store hash twin ([`crate::file_state::detect_committed_store`]): each committed managed doc's on-disk hash against its `record` entry, **detect without absorb** (`record` is borrowed `&`, no write, never via `reconcile_committed_store`).
///
/// The engine stays **domain-empty**: the caller (CLI) resolves the cascade and feeds in
/// the schemas, the per-workflow definition bundles (id + bytes + origin catalog), the step
/// source, the loaded `FileStateRecord`, `versions` (the doctype → manifest schema-version
/// map the CLI reads from the pack's freeze manifest — the fifth family's version-aware
/// route keys on it; an empty map leaves every finding un-routed), and `priors` (the doctype
/// → **shipped prior-version schema shapes** the CLI reads from the pack's snapshot store —
/// the [managed-vs-foreign classifier](classify_provenance)'s parse-against-a-prior arm keys
/// on it; an empty map narrows the classifier to *stamp or parses-against-current*). Severity
/// is the engine-owned post-pass at [`ValidationReport::new`], keyed by `(probe, check)`
/// identically to every other entry point (a no-delta `resolved` leaves every emitted
/// severity untouched).
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
    versions: &BTreeMap<String, u32>,
    priors: &BTreeMap<String, Vec<Schema>>,
    prior_home_instances: &[PriorHomeInstance],
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
    // The loop is also where each definition's **id** exists — the `workflow-refs.*` checks are
    // pure helpers over bytes and hold none — so it is fed in and every finding is keyed at the
    // pack resource it was raised in (`workflow:<id>` / `step:<id>`;
    // `command-output-contract.md` → `workflow-refs.*` — the pack-resource form). Without it the
    // family projected `(code, null)`: two bad refs in one step were byte-identical keys in one
    // report array.
    // The schema-ref membership check (M43) resolves against the **composed cascade's**
    // doctype set — `schemas` is the same composed map every compose site feeds, so ONE
    // set serves every workflow, deliberately unlike the per-origin catalog: a
    // methodology step legitimately solicits a dev doctype (`surface-contract.md` →
    // The schema projection: the composition model, not per-origin doctype scoping).
    let doctypes: std::collections::BTreeSet<String> = schemas.keys().cloned().collect();
    for workflow in workflows {
        workflow_source.scope_to_workflow(&workflow.id);
        findings.extend(crate::compose::workflow_refs_store(
            &workflow.id,
            &workflow.bytes,
            workflow_source,
            &workflow.catalog,
            &doctypes,
        ));
    }

    // Family 3 — file↔CLI-state, the read-only detect-without-absorb committed-store twin.
    // It takes the same `versions` + `priors` as family 5 and asks the same committed-bytes
    // question (`is_unadopted_foreign`): a never-adopted foreign squatter at a managed home is
    // not an *un-baselined managed doc*, and must not be told it will be baselined on an
    // author or finalize that will never touch it (M42; `validation.md` → the same
    // discriminator applies to family 3's `un-baselined` advisory).
    findings.extend(crate::file_state::detect_committed_store(
        record, schemas, repo_root, versions, priors,
    ));

    // Family 3 (cont.) — recorded-but-missing OOB-rename detection (M35, Component A;
    // `validation.md` → file↔CLI-state: recorded-but-missing rename detection). The
    // read-only twin of `reconcile_committed_store`'s recorded-missing arm: it enumerates
    // recorded paths absent on disk and runs the pure, non-mutating `detect_rename`, so a
    // bare `git mv` committed without `jigc rename` is caught — including the referrer-less
    // move no dangling-ref check can find. Fires **before** Family 4 (`ref-resolves`) and
    // returns the renamed `<type>:<slug>` identities, whose inbound edges are then
    // **scope-subtracted** from the ref-resolves walk below — so a moved-with-referrers doc
    // surfaces one `reconciliation.rename` finding, never N competing dangling refs.
    let (rename_findings, renamed_targets) =
        crate::file_state::detect_committed_store_renames(record, schemas, repo_root);
    findings.extend(rename_findings);

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
        &renamed_targets,
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
    // **Version-aware routing (M34)**: each non-conformant doc's findings are labelled with a
    // `route` by the doc's schema-version stamp vs `versions[ty]` (the doctype's manifest
    // version) — `below-version`/`stamp-absent ⇒ migrate`, `at-version ⇒ corrupt`
    // (`validation.md` → Version-aware routing). Still report-only (exit 0); the route is a
    // direction, the blocking counterpart is the transform-transaction migration gate.
    // **Managed-vs-foreign (M42)**: the family now reaches placement homes, where a
    // brownfield repo's own `CHANGELOG.md` may be a never-adopted **foreign** file. Each
    // instance is classified from its committed bytes ([`classify_provenance`] — the stamp, a
    // parse against the current schema, or a parse against a shipped `priors` shape); a
    // foreign one is an **adoption** case, not an unmigrated corpus, and takes the
    // suppressed-structure advisory arm.
    findings.extend(schema_conformance_store(
        repo_root,
        schemas,
        versions,
        priors,
        workflows,
        prior_home_instances,
    ));

    // The two M40 hollow-and-surplus advisories (`schema-conformance.
    // {repeatable-populated, surplus-sections-absent}`, `validation.md` → Hollow and
    // surplus adoption) — their **own** walk, appended after family 5 so the
    // version-mismatch adjudication there stays intact, covering the placement
    // instances family 5 skips ([`hollow_surplus_store`]). `repeatable-populated`
    // exemptions ride the `….exempt` string knob (space-separated `doctype#section`
    // tokens), read from the resolved cascade the CLI seeded from the pack `knobs.yaml`.
    let exempt = resolved
        .scalar("validation.schema-conformance.repeatable-populated.exempt")
        .unwrap_or("");
    findings.extend(hollow_surplus_store(repo_root, schemas, exempt));

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
/// `FileStateRecord`, no edge index. It enumerates the committed store through
/// [`crate::index::committed_instances`], the **placement-aware** enumerator its sibling walk
/// ([`hollow_surplus_store`]) already uses: every located `<location>/<slug>.md` **and** a
/// placement doctype's single literal `placement.file`, each carrying its `<type>:<slug>`
/// identity (a placement singleton's `<type>:<type>`) — **not** the anchor-scoped
/// `enumerate_committed_surface`, which under-walks. It is emphatically **not** keyed on
/// `schema.location` alone: that guard (*"a transient type has no committed docs"*) was true
/// pre-M38 and false since, and it dropped the whole **placement** class — where both
/// placement doctypes at schema-version 2 live (`changelog`, `deferral-ledger`) — out of the
/// detect-half, so `jigc validate` exited 0 over exactly the corpus `migrate-corpus` exists
/// to upgrade (`design/storage.md` → The census; `design/validation.md` → Store-scope
/// schema-conformance: "every committed instance" means `index::committed_instances`). The
/// `rel_key` is the instance's repo-relative path, so a parse-level finding is attributed to
/// the doc's real home; `schema-conformance.unknown-type` can never fire here (the type comes
/// from schema iteration, never a filename prefix).
///
/// **And the instances at a doctype's *prior* homes** ([`PriorHomeInstance`], M52 completion
/// audit, fix 2). The census answers *where does this doctype live*, in the present tense, so
/// after a bump that also moved a home the committed document sat at a path no resolved schema
/// names and the sweep reported on a corpus that did not contain it — *"no findings — the
/// committed store validates clean"*, exit 0, while `jigc migrate-corpus --dry-run` walked the
/// snapshot store and answered `1 would migrate` about the same file. That is the M42
/// false-green class reopened through the one axis Increment 7 widened on the migrate side
/// alone. The CLI enumerates the prior-home set (the snapshot store and the two root knobs are
/// its to read) and hands it in; this family adjudicates it beside the census, and the
/// document draws the code that is true of it — below-version, routed at the verb that lands
/// it — rather than silence, or `schema-conformance.orphaned-instance`'s *no resolved doctype
/// claims this path* said of a doctype `jigc describe` still lists.
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
fn schema_conformance_store(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    versions: &BTreeMap<String, u32>,
    priors: &BTreeMap<String, Vec<Schema>>,
    workflows: &[StoreWorkflow],
    prior_home_instances: &[PriorHomeInstance],
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (ty, schema) in schemas {
        // Whether the doctype-directed adoption verb even exists for this type: `jigc migrate
        // <path> --as <ty>` composes the off-router `migrate-<ty>` workflow and hard-errors
        // ("not migratable") when the composed pack ships none — so the adoption route names
        // it only when it is there (the M40 two-tier route: never command a verb that
        // hard-errors; `design/validation.md` → the M40 two-tier route).
        let migratable = workflows.iter().any(|w| w.id == format!("migrate-{ty}"));
        // `identity` is the `<type>:<slug>` URI the enumerator already derives (a placement
        // doctype's `<type>:<type>` singleton included) — never re-derived from the path,
        // which does not round-trip for a case-preserved literal home (`CHANGELOG.md`).
        // The doctype's committed instances at its **current** home, followed by the ones the
        // CLI found at a home only a **prior** version declared ([`PriorHomeInstance`]). Both
        // are this doctype's documents and both are adjudicated by the body below — same
        // parse, same managed-vs-foreign discriminator, same version-aware route — because a
        // document's home says where it is, never whose it is. Enumerating only the first set
        // is what made `jigc validate` report *"the committed store validates clean"* over a
        // corpus `jigc migrate-corpus` could see and land (M52 completion audit, fix 2).
        let at_prior_homes = prior_home_instances
            .iter()
            .filter(|instance| &instance.ty == ty)
            .map(|instance| (instance.identity.clone(), instance.path.clone()));
        for (identity, path) in crate::index::committed_instances(repo_root, ty, schema)
            .into_iter()
            .chain(at_prior_homes)
        {
            let rel_key = path
                .strip_prefix(repo_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            let Ok(bytes) = std::fs::read(&path) else {
                continue; // read race: skip; the next sweep re-checks.
            };
            // Mirror the pure task-scope call exactly (no BOM strip, `from_utf8_lossy`):
            // a non-UTF-8 instance is the parser's concern.
            let source = String::from_utf8_lossy(&bytes);

            // **The managed-vs-foreign discriminator** (M42; `design/validation.md` → The
            // managed-vs-foreign discriminator). A **foreign** file — never adopted, squatting
            // at a managed home (the brownfield repo's own Keep-a-Changelog `CHANGELOG.md`,
            // which the placement-aware enumerator now reaches) — is the **in-location-squatter
            // adoption case** (`design/storage.md` → Placement), not an unmigrated corpus. Its
            // structural findings are **suppressed**: adjudicating a file the user never handed
            // to jigc against jigc's schema produces N blocking breaks routed at a verb that
            // does nothing for it. It surfaces as exactly one adoption **advisory**, addressed
            // at its **path** (it has no managed identity to claim).
            // The identity the enumerator derived — the `<slug>` half, never re-derived
            // from the path (which does not round-trip for a placement doctype's literal
            // home).
            let slug = identity.split_once(':').map_or("", |(_, slug)| slug);
            if let Some(cause) = unadopted_cause(ty, slug, schema, &source, versions, priors) {
                findings.push(unadopted_instance(
                    repo_root, ty, &rel_key, migratable, cause,
                ));
                continue;
            }
            // Parse once so the version-aware route can read the doc's stamp from the same
            // parse the conformance checks run over. Inlines the store half of
            // `conformance_for`: the type comes from schema iteration, so `unknown-type`
            // can never fire here; a parse failure surfaces its `conformance.*` findings
            // directly — version-routed `migrate` when the doc is below-version (a structural
            // v1→v2 change makes a historical doc non-canonical), else un-routed (Err arm).
            //
            // The doctype's manifest version, read **once** for both parse arms: it decides
            // the version-aware route, the two version breaks, and — where it is absent —
            // the M51 advisory that says so. An absent entry is not a quiet skip: it is the
            // state in which [`SCHEMA_VERSION_CURRENT_CODE`] can never fire, so the sweep
            // says which check it is not running rather than reporting a clean store
            // ([`unversioned_doctype`]). Minted here so the two arms cannot word one fact
            // two ways; each arm folds it into its own per-doc vector, so it takes the same
            // [`attribute_to_doc`] pass every other per-doc finding does.
            let current = versions.get(ty).copied();
            let unversioned = current.is_none().then(|| unversioned_doctype(ty, &rel_key));
            match parse_sections(schema, &source) {
                Ok(doc) => {
                    let mut doc_findings = schema_conformance(schema, &source, &doc);
                    let stamp = read_schema_version_stamp(&doc);
                    route_schema_conformance(&mut doc_findings, stamp, current, &rel_key);
                    // Version-currency is itself a surfaced break (M34 Inc-3), and since M42 it
                    // is **its own check id**, emitted **unconditionally** on every below-version
                    // managed instance — never only on an otherwise-clean one. The old
                    // `doc_findings.is_empty()` guard existed solely because the break reused
                    // `field-value-conformant` and would have double-reported alongside a real
                    // value break; with a distinct code there is no double-report, and the guard
                    // was actively harmful — a stale doc that *also* carries structural findings
                    // (the commonest kind) would leave any consumer of the staleness fact with
                    // nothing to key on. `route_schema_conformance` above only *labels* findings
                    // that already exist, so emission — not annotation — is what closes the
                    // silent-drift hole for the pure-stamp v0 corpus, the M34 dogfood's own
                    // headline case (`design/validation.md` → Version-currency is itself a
                    // surfaced break; the retraction of the no-new-check-id pin).
                    if let Some(current) = current
                        && stamp.is_none_or(|s| s < current)
                    {
                        doc_findings.push(version_currency_break(stamp, current, &rel_key));
                    }
                    // The **above-current** sibling (2026-07-24, the confidence-audit
                    // sibling-hunt item 1): an OOB-planted or foreign-future stamp is a
                    // *different fact* from a stale one — `jigc migrate-corpus` cannot fix
                    // it — so it gets its own break ([`version_ahead_break`]) rather than
                    // riding the below-version code. Without this arm the case was fully
                    // silent: no finding, exit 0 — the fixed failure through an unfixed door.
                    if let Some(current) = current
                        && let Some(s) = stamp
                        && s > current
                    {
                        doc_findings.push(version_ahead_break(s, current, &rel_key));
                    }
                    doc_findings.extend(unversioned);
                    attribute_to_doc(&mut doc_findings, &identity, &rel_key);
                    findings.extend(doc_findings);
                }
                Err(mut parse_findings) => {
                    // A below-version doc of a *versioned* doctype can fail to PARSE under the
                    // current schema: a structural v1→v2 change (the M36 adr `options` slot)
                    // makes a historical 3-heading ADR non-canonical (`section-renamed`/
                    // `section-missing`) *before* conformance runs. Route its parse-failure
                    // findings `migrate` when its front-matter stamp is below (or absent under)
                    // the manifest `current`, so the detector agrees with the verb — `jigc
                    // migrate-corpus` sources the prior shape and upgrades it — rather than the
                    // operator hitting a blocking-conformance dead-end with no repair direction
                    // (`design/corpus-migration.md` → Acceptance flows: the adr v1→v2 flow; the
                    // Finding-2 detector/verb agreement extended to the structural parse-failure
                    // case). The stamp is read from the raw front matter (a full parse is
                    // unavailable here). An **at**-version parse failure is genuine corruption
                    // — left un-routed, exactly the labeler's `corrupt`-vs-`migrate` split. An
                    // **above**-current stamp (2026-07-24, the sibling-hunt item 1) is a third
                    // fact: the doc was written to a schema this binary does not know, so the
                    // ahead break rides this arm too (its parse findings stay un-routed — no
                    // repair of *this binary's* making exists; the break's own route names the
                    // human ones).
                    let stamp = schema_version_from_front_matter(&source);
                    if let Some(current) = current
                        && let Some(s) = stamp
                        && s > current
                    {
                        parse_findings.push(version_ahead_break(s, current, &rel_key));
                    }
                    if let Some(current) = current
                        && stamp.is_none_or(|s| s < current)
                    {
                        route_schema_conformance(
                            &mut parse_findings,
                            stamp,
                            Some(current),
                            &rel_key,
                        );
                        // The version-currency break rides this arm too (M42) — and it is the
                        // arm that most needs it: a below-version doc of a structurally-changed
                        // doctype (a v1-stamped ADR under the v2 `options` shape — the commonest
                        // stale doc in a real corpus) fails to PARSE, so before M42 it emitted
                        // only routed `conformance.*` parse findings and **zero**
                        // `schema-conformance.*` — the staleness fact, the one thing a machine
                        // consumer must key on, was not in the report at all.
                        parse_findings.push(version_currency_break(stamp, current, &rel_key));
                    }
                    parse_findings.extend(unversioned);
                    attribute_to_doc(&mut parse_findings, &identity, &rel_key);
                    findings.extend(parse_findings);
                }
            }
        }
    }
    findings
}

/// The two M40 hollow-and-surplus advisories' **own store walk**
/// (`design/validation.md` → Hollow and surplus adoption): re-parse every committed
/// instance and run [`repeatable_populated`] + [`surplus_sections_absent`] over it.
/// A separate walk from [`schema_conformance_store`] — deliberately, twice over:
///
/// - **Placement coverage.** Family 5 walks only `location:`-bearing schemas, but the
///   headline hollow case (a zero-item roadmap) is a **placement** doctype
///   (`design/storage.md` → Placement), so this walk enumerates
///   [`crate::index::committed_instances`] — located `<location>/*.md` **and** a
///   placement type's single literal file — without wholesale-extending family 5's
///   presence/value checks to placement docs.
/// - **The version-mismatch adjudication stays intact.** Family 5 surfaces the
///   pure-stamp v0 case only when a doc has *no other* findings; an advisory folded
///   into its per-doc `doc_findings` would silently swallow that break.
///
/// Advisory-only and report-only like every store family; an unparseable committed
/// file is family 5's concern, not these advisories' (skipped here — which also
/// keeps [`surplus_sections_absent`] off a between-surplus doc, section-renamed's
/// territory).
fn hollow_surplus_store(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    exempt: &str,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (ty, schema) in schemas {
        // `identity` is the `<type>:<slug>` URI (a placement doctype's `<type>:<type>`
        // singleton included) the path→URI flip keys on — no longer discarded.
        for (identity, path) in crate::index::committed_instances(repo_root, ty, schema) {
            let Ok(mut source) = std::fs::read_to_string(&path) else {
                continue; // read race: skip; the next sweep re-checks.
            };
            crate::parse::strip_leading_bom(&mut source);
            let Ok(doc) = parse_sections(schema, &source) else {
                continue; // unparseable committed file: family 5's concern, not this one's.
            };
            let mut doc_findings = repeatable_populated(schema, &doc, exempt);
            doc_findings.extend(surplus_sections_absent(schema, &source));
            if doc_findings.is_empty() {
                continue;
            }
            let rel_key = path
                .strip_prefix(repo_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            attribute_to_doc(&mut doc_findings, &identity, &rel_key);
            findings.extend(doc_findings);
        }
    }
    findings
}

/// Whether a committed instance at a managed home is a **never-adopted foreign file** — the
/// one question **every** consumer asks of the discriminator (M42): family 5's adoption
/// arm ([`schema_conformance_store`]), family 3's un-baselined advisory
/// ([`crate::file_state::detect_committed_store`]), and — since it went `pub` — the CLI's
/// `jigc doc list` **registration state** (`design/doc-read-surface.md` → the fourth read
/// surface: a row is `managed` or `unregistered` by *this* answer, never a second rule). A
/// foreign squatter is **one fact — the adoption case** — so a second family must not also
/// call it *"an un-baselined managed doc, no action needed — baselined on its next author or
/// finalize"*: a promise about a file jigc will never author (`design/validation.md` → *the
/// same discriminator applies to family 3's `un-baselined` advisory*). Single-sourced here so
/// the **precondition** below is stated once and cannot drift between its callers.
///
/// **The precondition: it answers only where a stamp can exist.** Stamp-absence is
/// [`classify_provenance`]'s first signal, and it is only *evidence* for a doctype the CLI
/// stamps at all — the **versioned** set (`versions`, the manifest map; the stamp is injected
/// exactly there — `crates/cli/src/pack.rs` → `load_pack_schema`). For an **unversioned**
/// doctype (a freeze-exempt / project-defined type) NO instance ever carries a stamp, so
/// "unstamped" says nothing at all, and calling a doc of that type "never adopted by jigc" on
/// the strength of a failed parse would be exactly the kind of unfounded claim this
/// discriminator exists to remove: a corrupt *managed* doc routed at adoption instead of
/// surfacing its break. An unversioned doctype is therefore **never classified** — both
/// families keep their pre-M42 behavior over it (`false` here: not foreign, i.e. adjudicated
/// as managed, exactly as before).
pub fn is_unadopted_foreign(
    ty: &str,
    slug: &str,
    schema: &Schema,
    source: &str,
    versions: &BTreeMap<String, u32>,
    priors: &BTreeMap<String, Vec<Schema>>,
) -> bool {
    unadopted_cause(ty, slug, schema, source, versions, priors).is_some()
}

/// **Why** a committed instance at a managed home is a never-adopted foreign file, or
/// `None` when it is jigc's own — the discriminator's answer with its cause attached, so
/// the one advisory both consumers emit can state the fault it actually found rather than
/// one of the two verbatim (`design/surface-contract.md` → law 1).
fn unadopted_cause(
    ty: &str,
    slug: &str,
    schema: &Schema,
    source: &str,
    versions: &BTreeMap<String, u32>,
    priors: &BTreeMap<String, Vec<Schema>>,
) -> Option<UnadoptedCause> {
    // The stated precondition, unchanged and still first: an unversioned doctype is never
    // classified at all, so neither leg below speaks for it.
    if !versions.contains_key(ty) {
        return None;
    }
    // **The identity leg** (M50 Inc 2 / T1). jigc's own writer names every doc it wrote
    // `<slug>.md` — the mint, `jigc rename` and the finalize promote all derive the name
    // from a well-formed slug — so a file at a managed home whose identity is not a slug
    // was produced by no jigc operation, whatever bytes it carries. A hand-typed stamp is
    // not evidence of jigc's hand; the **path** is, and the path says no. Asked before the
    // byte legs because it is the stronger signal and the one the M50 address guard makes
    // load-bearing: every door now refuses that identity, so a surface calling it
    // *managed* would be claiming a doc nothing can address.
    if !crate::slug::is_slug(slug) {
        return Some(UnadoptedCause::UnaddressableIdentity);
    }
    let ty_priors: &[Schema] = priors.get(ty).map_or(&[], Vec::as_slice);
    (classify_provenance(schema, source, ty_priors) == Provenance::Foreign)
        .then_some(UnadoptedCause::ForeignBytes)
}

/// The two ways a committed file at a managed home turns out never to have been adopted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnadoptedCause {
    /// Its **bytes** belong to no version of the schema and it carries no stamp — the M42
    /// brownfield squatter (a real Keep-a-Changelog `CHANGELOG.md`).
    ForeignBytes,
    /// Its **identity** is not a doc id, so no address reaches it and no jigc operation
    /// produced its path — the M50 hand-dropped population.
    UnaddressableIdentity,
}

/// A committed file at a managed home is one of two things, and the fifth family must not
/// confuse them (`design/validation.md` → The managed-vs-foreign discriminator).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Provenance {
    /// **jigc's own doc** — possibly stale (below its manifest schema-version, or v0-era and
    /// unstamped), but adopted: it belongs to the corpus `jigc migrate-corpus` upgrades.
    Managed,
    /// A **never-adopted foreign file** squatting at a managed home (a brownfield repo's real
    /// Keep-a-Changelog `CHANGELOG.md`). It belongs to the *adoption* path (`jigc ingest` /
    /// `jigc migrate <path> --as <doctype>`), never the corpus migration.
    Foreign,
}

/// Classify a committed instance's **provenance** from **its own committed bytes** — the
/// three-arm discriminator (`design/validation.md` → The managed-vs-foreign discriminator):
///
/// 1. it carries a **schema-version stamp** ⇒ **managed** (jigc wrote it; the stamp is
///    engine-`set:`, never authored);
/// 2. else it **parses** against the **current** schema, or against **any shipped prior-version
///    shape** (`priors`, the pack's `schema-snapshots/<ty>.v<k>.yaml` set) ⇒ **managed, v0-era**
///    — the M34 headline detect case, the population that predates the stamp. This arm is
///    load-bearing: a v0-era ADR does *not* parse against the current (v2) `adr` shape but does
///    parse against `adr.v1`, so a current-shape-only test would call it foreign and route a
///    managed doc at adoption;
/// 3. else ⇒ **foreign** — it was never written by jigc under any version of this schema.
///
/// **It must key on the committed bytes, not on the file-state record.** The record lives in
/// gitignored, rebuildable `.jigc/state/` (`design/storage.md` → Derived caches), so on a
/// **fresh clone of a managed repo** it is empty — a record-keyed discriminator would read
/// every managed doc as foreign and tell a teammate to `ingest` their own committed corpus.
/// The stamp survives a clone; the record does not.
///
/// **The bound, declared with its expiry** (`design/validation.md`): stamp-absent means *both*
/// "never adopted" *and* "v0-era managed". The parse test separates them in the ordinary case;
/// what remains is bounded, not eliminated (a foreign file that happens to parse clean reads
/// managed; a v0-era managed doc corrupted past parsing reads foreign). M34/M40 shipped the
/// v0→v1 stamp migration for every persisted doctype, so the unstamped-managed population is
/// finite pre-1.0 and empty at the 1.0 pin, after which stamp-absent means foreign, full stop.
fn classify_provenance(schema: &Schema, source: &str, priors: &[Schema]) -> Provenance {
    // Arm 1 — the stamp, read from the raw front matter so a doc that fails to PARSE under the
    // current schema (the stale-but-managed case) is still recognized as jigc's own.
    if schema_version_from_front_matter(source).is_some() {
        return Provenance::Managed;
    }
    // Arms 2 + 3 — the parse test, current shape first, then each shipped prior shape.
    if parse_sections(schema, source).is_ok()
        || priors
            .iter()
            .any(|prior| parse_sections(prior, source).is_ok())
    {
        return Provenance::Managed;
    }
    Provenance::Foreign
}

/// The **adoption advisory** for a never-adopted foreign file squatting at a managed home
/// (`schema-conformance.unadopted-instance`, M42 — un-keyed, store-scope-only, gating nothing;
/// `design/validation.md` → Informational outcomes). It is addressed at the **file path**, not
/// a `<type>:<slug>` URI: a foreign file has **no managed identity to claim** (the
/// `file-state.*` path-keyed form — `design/command-output-contract.md` → the target-normal
/// forms), and a synthesized URI would name a doc that does not exist.
///
/// Its route is [`adoption_route`]'s — the one the read verb also serves (below).
fn unadopted_instance(
    repo_root: &Path,
    ty: &str,
    rel_key: &str,
    migratable: bool,
    cause: UnadoptedCause,
) -> Finding {
    // The tail names the fault the discriminator actually found. One code, one route, two
    // causes — stating the byte cause over an identity fault would be a law-1 lie about a
    // file that is stamped and parses clean.
    let tail = match cause {
        UnadoptedCause::ForeignBytes => format!(
            "it carries no schema-version stamp and parses against no known `{ty}` schema version"
        ),
        UnadoptedCause::UnaddressableIdentity => {
            "its name is not a doc id, so no `<type>:<slug>` address reaches it — jigc names \
             every doc it writes `<slug>.md`"
                .to_string()
        }
    };
    let message = format!(
        "committed file `{rel_key}` sits at the `{ty}` home but was never adopted by jigc — {tail}"
    );
    Finding::graded(
        Severity::Advisory,
        "schema-conformance.unadopted-instance",
        message,
        Some(Location::addressed(rel_key, 1, 1)),
        Some(adoption_route(repo_root, ty, rel_key, migratable).into()),
    )
}

/// **The route a never-adopted foreign file gets, from every surface that meets one** — the
/// store sweep's adoption advisory ([`unadopted_instance`], above) and — since it went `pub`
/// (M42, T7) — `jigc doc show`'s read-side block on the same file (`crates/cli/src/doc.rs` →
/// `reroute_unadopted`; `design/doc-read-surface.md` → `jigc doc list`: *"`doc show`'s block on
/// an unregistered instance routes to adoption, not to hand-repair"*). Single-sourced so the two
/// surfaces cannot tell two stories about one file — the very failure this discriminator exists
/// to close.
///
/// It obeys the **M40 two-tier rule — never command a verb that hard-errors**: the
/// doctype-directed `jigc migrate <path> --as <ty>` is named only when the composed pack ships
/// the `migrate-<ty>` workflow it composes (`migratable`); `jigc ingest`, the adoption front
/// door, always applies. It deliberately does **not** name `jigc migrate-corpus` — that verb
/// upgrades the *managed* corpus and would report this file `blocked`, doing nothing.
pub fn adoption_route(repo_root: &Path, ty: &str, rel_key: &str, migratable: bool) -> String {
    if migratable {
        format!(
            // **The operand is absolute** (M53 post-review-fix review, HIGH 2). `rel_key` is a
            // repo-relative store key and `jigc migrate <PATH>` roots its argument at the
            // *caller's* cwd, so the repo-relative spelling ran from the repository root and
            // nowhere else — driven from `docs/deep`, the emitted line answered *"could not
            // read the foreign `changelog` source at `CHANGELOG.md`"*. The message and the
            // `at:` locus above keep the repo-relative spelling: they are read, not run.
            "adopt — run `jigc ingest` to route it, or `{migrate} --as {ty}` to \
             rewrite it into the managed `{ty}` shape; it is a foreign file, not an unmigrated \
             managed doc",
            migrate = crate::finding::migrate_at(repo_root, rel_key),
        )
    } else {
        "adopt — run `jigc ingest` to route it; it is a foreign file, not an unmigrated \
         managed doc"
            .to_string()
    }
}

/// The **managed-vs-foreign discriminator's three CLI-supplied inputs**, bundled so the
/// *task-scope* reconciler can ask the same question the store family asks (M48 Inc 4 / T1).
///
/// M42 shipped the discriminator and swept it through the surfaces that adjudicate the
/// **committed store**: the fifth content family, family 3's un-baselined advisory, and `jigc
/// doc list`'s registration state. It never reached
/// [`crate::file_state::reconcile_committed`] — the primitive `jigc task validate` and the
/// `finalize` preflight route every committed doc through — so one foreign file answered
/// `schema-conformance.unadopted-instance` at one door and `reconciliation.conformance-block`
/// at the other, with a route that is prose about a class rather than about that file.
///
/// The engine produces none of these three: the manifest version map, the shipped
/// prior-version shapes and the set of doctypes whose `migrate-<ty>` workflow exists are all
/// **pack** facts the CLI resolves and threads in — the determinism boundary, exactly like the
/// caller-supplied [`crate::file_state::ConflictBlock`] beside it. Bundling them keeps one
/// value travelling through [`validate_task`] instead of three, and keeps the question
/// answerable in exactly one place ([`AdoptionInputs::unadopted`]).
///
/// **Inert by construction where it must be.** An empty `versions` map makes
/// [`is_unadopted_foreign`] answer `false` for every doctype (its stated precondition: the
/// discriminator runs only where a stamp can exist), so a caller that supplies nothing gets
/// the pre-M48 behaviour byte for byte.
#[derive(Clone, Copy)]
pub struct AdoptionInputs<'a> {
    /// `doctype → manifest schema-version` — the **versioned** set the discriminator is
    /// allowed to answer over at all.
    versions: &'a BTreeMap<String, u32>,
    /// `doctype → shipped prior-version shapes` — the parse-against-a-prior arm that keeps a
    /// v0-era **managed** doc from reading foreign.
    priors: &'a BTreeMap<String, Vec<Schema>>,
    /// The doctypes whose `migrate-<ty>` workflow ships — the M40 two-tier route's condition
    /// (never command a verb that hard-errors).
    migratable: &'a BTreeSet<String>,
    /// The door's own resolved repository root — the base the adoption route's `jigc migrate`
    /// operand is spelled against (M53 post-review-fix review, HIGH 2). A fourth CLI-supplied
    /// input for the same reason as the other three: the engine resolves no repository, and
    /// the route the two doors must tell one story with is now a *based* route, so the base
    /// has to travel with the bundle rather than be re-derived at each seam.
    repo_root: &'a Path,
}

impl<'a> AdoptionInputs<'a> {
    /// Bundle the three pack facts the CLI resolved.
    pub fn new(
        versions: &'a BTreeMap<String, u32>,
        priors: &'a BTreeMap<String, Vec<Schema>>,
        migratable: &'a BTreeSet<String>,
        repo_root: &'a Path,
    ) -> Self {
        Self {
            versions,
            priors,
            migratable,
            repo_root,
        }
    }

    /// The **adoption advisory** for a committed file at a managed home, or `None` when the
    /// discriminator adjudicates it **managed** — the single seam every non-store consumer
    /// asks through, so the code *and* the route are decided once.
    ///
    /// It is [`unadopted_instance`] verbatim, gated on [`is_unadopted_foreign`]: no new check
    /// id, no second producer, no second route (`design/validation.md` → The managed-vs-foreign
    /// discriminator). `rel_key` is the repo-relative path the finding addresses — a foreign
    /// file has no managed identity to claim.
    ///
    /// **`pub` since M46 Inc 3 / T2**, and the visibility is the whole point. `jigc
    /// migrate-corpus` excludes a never-adopted foreign file from its fold and must *report*
    /// it — as **this** advisory, verbatim, or the two doors tell two stories about one file
    /// again (which is the defect: the store door said *"adopt it"*, the corpus door said
    /// *"author the prose, then re-run"* over a file jigc never wrote). [`unadopted_instance`]
    /// stays private, so widening this seam is the only way to mirror it and there is no
    /// second constructor to drift from — read-only reach, the `STORE_EXIT_FLIPS` precedent.
    pub fn unadopted(
        &self,
        ty: &str,
        slug: &str,
        schema: &Schema,
        source: &str,
        rel_key: &str,
    ) -> Option<Finding> {
        unadopted_cause(ty, slug, schema, source, self.versions, self.priors).map(|cause| {
            unadopted_instance(
                self.repo_root,
                ty,
                rel_key,
                self.migratable.contains(ty),
                cause,
            )
        })
    }

    /// The doctype's **manifest schema-version**, or `None` for an **unversioned** doctype
    /// (M48 Inc 4 / T2) — the second thing the task-scope reconciler needs from this bundle,
    /// and the same map [`is_unadopted_foreign`]'s precondition keys on.
    ///
    /// `None` is what makes the unversioned population correct **by construction** rather than
    /// by a second branch: [`route_schema_conformance`] returns early on `None`, so a doctype
    /// with no notion of "below version" keeps the hand-repair sanction — and that is exactly
    /// the population the discriminator itself never classifies.
    pub(crate) fn current(&self, ty: &str) -> Option<u32> {
        self.versions.get(ty).copied()
    }
}

#[cfg(test)]
impl AdoptionInputs<'static> {
    /// The **inert** discriminator — the empty pack-fact set an engine unit test supplies when
    /// its subject is not adoption. `versions` is empty, so [`is_unadopted_foreign`]'s stated
    /// precondition refuses every doctype and [`AdoptionInputs::unadopted`] answers `None`
    /// throughout: the reconciler's `UNKNOWN` + non-conformant arm keeps its pre-M48
    /// `reconciliation.conformance-block` byte for byte. The foreign arm has its own binary-level
    /// acceptance (`crates/cli/tests/foreign_at_both_doors.rs`), where the real pack facts are in
    /// hand — the engine ships empty of content by invariant and cannot mint them here.
    pub(crate) fn inert() -> Self {
        use std::sync::OnceLock;
        static VERSIONS: OnceLock<BTreeMap<String, u32>> = OnceLock::new();
        static PRIORS: OnceLock<BTreeMap<String, Vec<Schema>>> = OnceLock::new();
        static MIGRATABLE: OnceLock<BTreeSet<String>> = OnceLock::new();
        Self {
            versions: VERSIONS.get_or_init(BTreeMap::new),
            priors: PRIORS.get_or_init(BTreeMap::new),
            migratable: MIGRATABLE.get_or_init(BTreeSet::new),
            // Unreachable by construction: an empty `versions` map makes the discriminator
            // answer `false` for every doctype, so no adoption route is ever composed from
            // this bundle and the root is never read.
            repo_root: Path::new("/"),
        }
    }
}

#[cfg(test)]
mod provenance_tests {
    //! The **managed-vs-foreign discriminator** ([`classify_provenance`], M42;
    //! `design/validation.md` → The managed-vs-foreign discriminator), pinned arm by arm over
    //! the real shipped shapes: the **current** (v2) `adr` — status header + the optional
    //! `## Options` slot — and its shipped **prior** (v1) snapshot shape, both carrying the
    //! injected schema-version stamp exactly as the CLI's `load_pack_schema` produces them.

    use super::*;

    /// The current `adr` shape (v2 — the `options` slot), stamp-injected like the CLI's load.
    fn current_adr() -> Schema {
        let yaml = br#"
type: adr
location: decisions/
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
  - id: context
    slot: { hint: Forces. }
  - id: options
    slot: { optional: true, hint: Alternatives. }
  - id: decision
    slot: { hint: What. }
  - id: consequences
    slot: { hint: Effects. }
"#;
        let mut schema = crate::schema::load_schema(yaml).expect("the current adr shape loads");
        crate::schema::inject_schema_version_stamp(&mut schema);
        schema
    }

    /// The shipped **prior** `adr.v1` shape (no `options` slot), stamp-injected identically.
    fn prior_adr() -> Schema {
        let yaml = br#"
type: adr
location: decisions/
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
  - id: context
    slot: { hint: Forces. }
  - id: decision
    slot: { hint: What. }
  - id: consequences
    slot: { hint: Effects. }
"#;
        let mut schema = crate::schema::load_schema(yaml).expect("the prior adr.v1 shape loads");
        crate::schema::inject_schema_version_stamp(&mut schema);
        schema
    }

    /// A v0-era ADR: the **prior** three-slot shape, **unstamped** (it predates the stamp).
    const V0_ERA_ADR: &str = "\
---
status: accepted
date: 2026-06-25
---

# Cache sessions in memory

## Context

Forces.

## Decision

What.

## Consequences

Effects.
";

    /// A **stamped** doc whose body parses against **nothing** — corrupted past parsing, yet
    /// unmistakably jigc's own: the stamp is engine-`set:`, never authored.
    const STAMPED_BUT_UNPARSEABLE: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Cache sessions in memory

## Backstory

Someone renamed every heading by hand.
";

    /// A real **Keep-a-Changelog** file — a foreign document that was never jigc's.
    const FOREIGN: &str = "\
# Changelog

## [Unreleased]

### Added

- A new thing.
";

    /// (arm 1 — stamped) A schema-version stamp means **managed**, whatever the body does: it
    /// is engine-written, never authored, so its presence is proof jigc wrote the doc.
    #[test]
    fn a_stamped_doc_is_managed_even_when_it_parses_against_nothing() {
        let current = current_adr();
        let priors = vec![prior_adr()];
        assert!(
            parse_sections(&current, STAMPED_BUT_UNPARSEABLE).is_err(),
            "precondition: the stamped doc must NOT parse — otherwise this arm is not tested",
        );
        assert_eq!(
            classify_provenance(&current, STAMPED_BUT_UNPARSEABLE, &priors),
            Provenance::Managed,
            "the stamp alone settles provenance — a stale/corrupt managed doc is still managed",
        );
    }

    /// (arm 2 — unstamped, parses against the CURRENT shape) A conformant unstamped doc of the
    /// current shape is a **managed, v0-era** doc, not a foreign file.
    #[test]
    fn an_unstamped_doc_that_parses_against_the_current_shape_is_managed() {
        let current = current_adr();
        let unstamped_current = V0_ERA_ADR.replacen(
            "## Decision",
            "## Options\n\nAlternatives.\n\n## Decision",
            1,
        );
        assert!(
            parse_sections(&current, &unstamped_current).is_ok(),
            "precondition: this doc parses against the current shape",
        );
        assert_eq!(
            classify_provenance(&current, &unstamped_current, &[]),
            Provenance::Managed,
            "a v0-era doc in the current shape is managed — even with no priors in hand",
        );
    }

    /// (arm 2 — unstamped, parses against a shipped PRIOR shape) **The load-bearing arm.** A
    /// v0-era ADR does **not** parse against the current (v2) shape — the optional `## Options`
    /// slot makes `## Decision` read as a renamed section — but it **does** parse against the
    /// shipped `adr.v1` snapshot. Without the priors arm the classifier would call jigc's own
    /// doc foreign and route the M34 headline detect case at *adoption*, so the same input is
    /// pinned **both** ways: managed with the prior in hand, foreign without it.
    #[test]
    fn an_unstamped_doc_that_parses_only_against_a_shipped_prior_is_managed() {
        let current = current_adr();
        let priors = vec![prior_adr()];
        assert!(
            parse_sections(&current, V0_ERA_ADR).is_err(),
            "precondition: a v0-era ADR does NOT parse against the current (v2) shape",
        );
        assert!(
            parse_sections(&priors[0], V0_ERA_ADR).is_ok(),
            "precondition: it DOES parse against the shipped adr.v1 shape",
        );
        assert_eq!(
            classify_provenance(&current, V0_ERA_ADR, &priors),
            Provenance::Managed,
            "the parse-against-a-prior arm keeps the v0-era managed corpus managed",
        );
        assert_eq!(
            classify_provenance(&current, V0_ERA_ADR, &[]),
            Provenance::Foreign,
            "and it is LOAD-BEARING: drop the priors and the same managed doc reads foreign",
        );
    }

    /// (arm 3 — unstamped, parses against nothing) A real Keep-a-Changelog file at a managed
    /// home is **foreign**: never adopted, and no version of the schema ever wrote it.
    #[test]
    fn an_unstamped_doc_that_parses_against_no_known_version_is_foreign() {
        let current = current_adr();
        let priors = vec![prior_adr()];
        assert_eq!(
            classify_provenance(&current, FOREIGN, &priors),
            Provenance::Foreign,
            "a file jigc never wrote, under any shipped version, is a foreign adoption case",
        );
    }
}

/// Read a committed instance's per-doc **schema-version stamp** value (the
/// engine-declared [`crate::schema::SCHEMA_VERSION_FIELD`] header leaf,
/// `design/corpus-migration.md` → The schema-version stamp), if present and integer-valued.
/// A stamp-absent doc (the v0 corpus state, before the stamp existed) yields `None` — which
/// the routing treats as below-version (`migrate`).
fn read_schema_version_stamp(doc: &Document) -> Option<u32> {
    doc.sections.iter().find_map(|section| {
        section
            .fields
            .iter()
            .find(|f| f.key == crate::schema::SCHEMA_VERSION_FIELD)
            .and_then(|f| match &f.value {
                crate::field_block::Value::Scalar(v) => v.trim().parse::<u32>().ok(),
                crate::field_block::Value::List(_) => None,
            })
    })
}

/// Read the schema-version stamp from a committed doc's leading `---` front-matter block
/// **without a full parse** — the parse-failure sibling of [`read_schema_version_stamp`]
/// (which needs a parsed [`Document`]). A below-version doc of a structurally-changed
/// doctype fails to parse under the current schema, so the version-aware route must source
/// its stamp from the raw front matter. A doc with no leading fence, or no integer
/// `schema-version:` line, yields `None` — the v0 corpus state the routing treats as
/// below-version.
///
/// `pub(crate)` since M48 Inc 4 / T2: the task-scope reconciler's advisory routes on the same
/// stamp, read the same way, so the two doors cannot disagree about what a doc is stamped.
///
/// **`pub` since M51 Inc 8 / T3**, for the one question that has no schema to ask it with: the
/// CLI's orphaned-instance enumerator (`cli::orphan`) meets a committed doc whose declared
/// doctype is defined by **no resolved schema**, so every stamp reader that takes a `&Schema`
/// is unavailable to it by construction. Read-only reach — the engine still ships empty of
/// content and still never shells to git for the tracked set the enumerator walks.
pub fn schema_version_from_front_matter(source: &str) -> Option<u32> {
    let body = source.strip_prefix("---\n")?;
    let end = body.find("\n---")?;
    body[..end].lines().find_map(|line| {
        line.strip_prefix(&format!("{}:", crate::schema::SCHEMA_VERSION_FIELD))
            .and_then(|v| v.trim().parse::<u32>().ok())
    })
}

/// The check id of the **version-currency** break — a *managed* committed instance whose
/// schema-version stamp is below its doctype's manifest version (M42).
pub const SCHEMA_VERSION_CURRENT_CODE: &str = "schema-conformance.schema-version-current";

/// Build the **version-currency** break for a committed doc of a versioned doctype, classified
/// **managed**, whose schema-version stamp is **absent** (the v0 corpus state) or **below** its
/// doctype's manifest `current` — the load-bearing M34 case a labeler-only path leaves silent
/// (`design/validation.md` → Version-currency is itself a surfaced break; DECISIONS 2026-06-25).
///
/// **Its own check id** (M42), retracting the M34 pin that reused `field-value-conformant` over
/// the stamp field: a stale stamp and an invalid enum value are *different facts with different
/// consequences*, and under the reuse the version break is **indistinguishable to any machine
/// consumer** — including the exit predicate, which sees only `(probe, code)`. Reusing the
/// value-conformance id was always a category error (the stamp is not a field whose *value* the
/// author got wrong — it is a fact about which schema the whole doc was written against); it
/// merely cost nothing until something needed to **act** on the distinction
/// (`design/validation.md` → the retraction, with its rationale engaged).
///
/// It carries **its own route** — `jigc migrate-corpus`, the verb that upgrades a managed corpus,
/// named verbatim — rather than the generic [`route_schema_conformance`] label the doc's *other*
/// findings take, and leads with the same `migrate` classification token. `Severity::Blocking`
/// like every conformance break; *blocking* is a task/finalize verdict, so under the store sweep
/// it is reported, never a gate — but it is the one content finding whose presence **flips
/// `jigc validate`'s exit non-zero** (M42, the third exit-flipping exception): an unmigrated
/// corpus means every other family in the sweep adjudicated docs against a schema they were
/// never written to, so the *result* is untrustworthy. The CLI's exit predicate keys on this
/// code (`crates/cli/src/render.rs` → `validation_store_exit_flips`), which is why the fact
/// needed its own id at all.
///
/// The stamp stays **author-exempt** (`is_author_required` untouched) — version-currency is a
/// store-scope corpus rule, never an authoring obligation, so task/finalize scope is unaffected.
fn version_currency_break(stamp: Option<u32>, current: u32, rel_key: &str) -> Finding {
    let field = crate::schema::SCHEMA_VERSION_FIELD;
    let message = match stamp {
        None => format!(
            "field `{field}` is absent; the committed doc predates the schema-version stamp \
             (below the current schema-version {current})"
        ),
        Some(s) => format!(
            "field `{field}` is schema-version {s}, below the current schema-version {current}"
        ),
    };
    let route = format!(
        "migrate — `{rel_key}` is a managed doc below the current schema-version {current}; run \
         `jigc migrate-corpus` to upgrade it"
    );
    Finding::graded(
        Severity::Blocking,
        SCHEMA_VERSION_CURRENT_CODE,
        message,
        None,
        Some(route.into()),
    )
}

/// The check id of the **version-ahead** break — a *managed* committed instance whose
/// schema-version stamp is **above** its doctype's manifest version (2026-07-24, the
/// confidence-audit sibling-hunt item 1).
pub const SCHEMA_VERSION_AHEAD_CODE: &str = "schema-conformance.schema-version-ahead";

/// Build the **version-ahead** break for a committed doc of a versioned doctype, classified
/// **managed**, whose schema-version stamp is **above** its doctype's manifest `current` — an
/// OOB-planted or foreign-future stamp (the sibling-hunt item 1: before this arm the case was
/// fully silent — no finding, exit 0, and `jigc migrate-corpus` skipped the doc as
/// already-current forever).
///
/// **Its own check id**, by the M42 retraction's own rationale ([`version_currency_break`]):
/// a stale stamp and a future stamp are *different facts with different consequences* — a
/// stale doc is mechanically repaired by `jigc migrate-corpus`, a future-stamped doc
/// **cannot be** (this binary has no schema to migrate it *to*) — and a machine consumer
/// keys on the code alone, so folding both into `schema-version-current` would route an
/// unfixable doc at a verb that blocks it.
///
/// The route is **Human-shaped** — no mechanical fix exists (`set-field` refuses the
/// machine-maintained stamp, the M45 forged-freeze-stamp guard): either a newer jigc wrote
/// the doc (upgrade jigc) or the stamp was edited out-of-band (restore it from git history).
/// `Severity::Blocking`, and — like its below sibling — the one **store-scope** consequence
/// is the exit flip: the doc was written to a schema this binary does not know, so the sweep
/// could not adjudicate it (the same untrustworthy-sweep criterion; the CLI's exit predicate
/// keys on this code too, `crates/cli/src/render.rs` → `validation_store_exit_flips`).
///
/// Deliberately **not** a `CHECK_INVENTORY` row (no severity knob): the break is the
/// trustworthiness floor stated for the inverse direction, a severity delta could not change
/// its consequence (the exit flip keys on the code, never the severity), and an un-keyed code
/// cannot be demoted at all — a floor stronger than `floor: blocking`.
fn version_ahead_break(stamp: u32, current: u32, rel_key: &str) -> Finding {
    let field = crate::schema::SCHEMA_VERSION_FIELD;
    let message = format!(
        "field `{field}` is schema-version {stamp}, above the current schema-version {current} \
         — the doc was written to a schema this jigc build does not know"
    );
    let route = ahead_route(stamp, current, rel_key);
    Finding::graded(
        Severity::Blocking,
        SCHEMA_VERSION_AHEAD_CODE,
        message,
        None,
        Some(route.into()),
    )
}

/// The **one** route wording for the future/foreign-stamp fact — shared by the ahead break
/// and [`route_schema_conformance`]'s ahead arm, so the two surfaces cannot tell two stories
/// about one doc. Human-shaped: no verb fixes a future stamp.
fn ahead_route(stamp: u32, current: u32, rel_key: &str) -> String {
    format!(
        "ahead — `{rel_key}` is stamped schema-version {stamp}, above the current {current} \
         this jigc build knows; either a newer jigc wrote it (upgrade jigc) or the stamp was \
         edited out-of-band (restore it from git history) — `jigc migrate-corpus` cannot fix \
         a future stamp"
    )
}

/// The check id of the **unversioned-doctype** advisory — a committed instance of a doctype
/// the composed set **defines** but whose owning pack ships **no `schema-manifest.yaml` entry**
/// for it (M51 Inc 8 / T5; `design/validation.md` → The M51 registrations — Increment 8, and
/// `completions/artifacts/M51/settle-record.md` → §18, which settles D10's second N26 question
/// *an unstamped managed corpus gets a store-surface answer*).
///
/// **The resolved half of one partition.** The store surface asks a single question — *does
/// the file's declared doctype resolve to a schema in the composed set?* — and each answer has
/// exactly one owner: *no* is `cli::orphan`'s `schema-conformance.orphaned-instance`, *yes* is
/// this code. Calling a doc of a **defined** doctype an orphan would be a law-1 lie, and
/// leaving it unsaid is what the base did.
pub const UNVERSIONED_DOCTYPE_CODE: &str = "schema-conformance.unversioned-doctype";

/// Build the **unversioned-doctype** advisory for one committed instance of a resolved but
/// manifest-less doctype.
///
/// **What it actually reports, stated so the wording can be held to it:** the *absence* the
/// sweep meets is `versions.get(ty) == None` — the per-doctype governed union
/// (`crates/cli/src/pack.rs` → `frozen_doctype_versions`) carries an entry only where the
/// doctype's **own origin pack** ships a manifest listing it. Where it does not, no
/// `schema-version` stamp is ever injected (the stamp is minted exactly there), so
/// [`SCHEMA_VERSION_CURRENT_CODE`] — the M42 check that exists to catch an unmigrated corpus —
/// **can never fire** against this doc, whatever its shape. The fact is about the *doctype*,
/// which is why the message leads with the doctype rather than with the file: it holds
/// identically for a foreign squatter sitting at that doctype's home, and a message keyed on
/// *this doc is unstamped* would be an unfounded claim about the file's provenance (the
/// managed-vs-foreign discriminator is stated inert over an unversioned doctype for exactly
/// that reason — [`unadopted_cause`]'s precondition).
///
/// **Advisory, report-only, and deliberately NOT a `cli::render::STORE_EXIT_FLIPS` member** —
/// the one row where it departs from its blocking sibling, on the human's confirmed reason
/// (§18): *a manifest-less pack means "unchecked" by the manifest header's own stated design,
/// and flipping would fail every manifest-less project pack's CI for a permitted choice*. The
/// header states the mechanism plainly — *an absent manifest = unchecked*
/// (`crates/cli/pack/config/schema-manifest.yaml`) — so a pack author who takes that choice
/// is told what they are not getting, never failed for taking it.
///
/// **Route: `Human`, two exits, the second a real one.** Declare the doctype's version by
/// adding a manifest entry, or state that the doctype is deliberately unfrozen. No argv
/// performs either — a pack's manifest is a pack-author edit, not a corpus operation — so a
/// `Mechanical` route here would name a verb that cannot do the work.
///
/// Un-keyed like its siblings (no `CHECK_INVENTORY` row, no severity knob): the inventory's
/// two-tier rule is defined over keyed rows, and this check has no knob to floor.
///
/// Doc-less and location-less by construction — it is emitted into the per-doc finding vector
/// and [`attribute_to_doc`] gives it the doc's display path and its `<type>:<slug>` identity
/// address, the address form every per-doc finding of this family takes (the *orphaned*
/// sibling keys at its path because, its doctype being undefined, it has no identity to claim).
fn unversioned_doctype(ty: &str, rel_key: &str) -> Finding {
    let field = crate::schema::SCHEMA_VERSION_FIELD;
    let message = format!(
        "doctype `{ty}` is defined by a pack that declares no `schema-manifest.yaml` entry \
         for it, so no `{field}` is ever stamped on its instances and the version-currency \
         check cannot tell a current doc from an unmigrated one"
    );
    let route = format!(
        "declare the version — add a `{ty}` entry to its pack's `config/schema-manifest.yaml`, \
         re-pinning the doctype's `schema-hash` — or state that `{ty}` is deliberately \
         unfrozen: an absent manifest entry means unchecked by design, so `{rel_key}` is \
         reported here and gated nowhere"
    );
    Finding::graded(
        Severity::Advisory,
        UNVERSIONED_DOCTYPE_CODE,
        message,
        None,
        Some(crate::finding::Route::human(route)),
    )
}

/// Label each `schema-conformance.*` finding over a non-conformant committed doc with its
/// **version-aware route** (`design/validation.md` → Store-scope schema-conformance →
/// Version-aware routing): a doc stamped **below** its doctype's `manifest` version — or
/// carrying **no** stamp (the v0 corpus state) — routes `migrate` (a known-old-version doc
/// the M34 transform can upgrade); one **at** the current version that still does not
/// conform routes `corrupt` (human review, not a version bump); one **above** the current
/// version routes `ahead` (2026-07-24 — the future/foreign-stamp fact, [`ahead_route`]:
/// upgrade jigc or restore the stamp, never `migrate` and never the at-current `corrupt`
/// lie). The classification leads the
/// `route` string so it is both a human-readable repair direction and a stable machine token
/// the agent reads; the engine never executes it (route is a direction, not a guarantee).
///
/// Routing applies only to a doctype the caller supplies a manifest version for (`current`
/// is `Some`): a doctype outside the versioned/frozen set has no notion of "below version",
/// so its findings stay un-routed (reported, never mislabeled). No new check id or knob — the
/// route rides the existing finding (M33 store-family pattern).
///
/// `pub(crate)` since M48 Inc 4 / T2: the **task-scope** reconciler's un-baselined advisory
/// ([`crate::file_state`] → `conformance_advisory_finding`) labels itself through this same
/// function, so *stale* is *stale* at both doors. That caller starts from the hand-repair
/// sanction and lets this override it, which is why the **at-version** arm — the one case the
/// sanction is the true advice — is the only arm it does not take.
pub(crate) fn route_schema_conformance(
    findings: &mut [Finding],
    stamp: Option<u32>,
    current: Option<u32>,
    rel_key: &str,
) {
    let Some(current) = current else {
        return; // an unversioned doctype has no migrate-vs-corrupt distinction.
    };
    let route = match stamp {
        None => format!(
            "migrate — `{rel_key}` carries no schema-version stamp; run the corpus migration \
             to stamp and upgrade it to schema-version {current}"
        ),
        Some(s) if s < current => format!(
            "migrate — `{rel_key}` is stamped schema-version {s}, below the current \
             {current}; run the corpus migration to upgrade it"
        ),
        // Above-current (2026-07-24, the sibling-hunt item 1): labelling these findings
        // `corrupt — at the current schema-version` would be a lie (the doc is at a
        // *future* one), and `migrate` would route at a verb that blocks it. Same wording
        // as the ahead break's own route.
        Some(s) if s > current => ahead_route(s, current, rel_key),
        Some(_) => format!(
            "corrupt — `{rel_key}` is at the current schema-version {current} but does not \
             conform; review it by hand"
        ),
    };
    for finding in findings {
        finding.route = Some(route.clone().into());
    }
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
    // The store sweep is task-less: there is no index to commit, so its root IS the
    // on-disk working tree — and its findings say so (`validation.md`:279).
    let snapshot =
        EffectiveStateSnapshot::new(anchors, repo_root.to_path_buf(), RootKind::WorkingTree);
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
///
/// Uniqueness must hold even for **concurrent sweeps in one process**: `pid` plus a *raw*
/// clock read collides when two threads sample the same instant, and a collision lets one
/// sweep's `remove_file` delete the other's snapshot mid-flight (a `NotFound` flake). So the
/// nanos component comes from [`crate::tempname::unique_nanos`], which is strictly increasing
/// per process and therefore closes that window regardless of clock resolution.
fn store_scratch_path() -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "jigc-{}-{}-{}",
        std::process::id(),
        crate::tempname::unique_nanos(),
        STORE_SNAPSHOT_FILE,
    ));
    path
}

#[cfg(test)]
mod store_scratch_path_tests {
    use super::{STORE_SNAPSHOT_FILE, store_scratch_path};

    /// The store-sweep scratch path takes its disambiguator from the **shared**
    /// [`crate::tempname::unique_nanos`] mint, not a counter private to this module.
    ///
    /// Two claims, both load-bearing:
    ///
    /// 1. **The shape** is `jigc-<pid>-<nanos>-<snapshot-file>` under
    ///    [`std::env::temp_dir`] — the `pid` keeps concurrent *processes* apart and the
    ///    temp dir keeps the task-less sweep out of any managed `location:`. There is no
    ///    third component before the filename: the private sequence nonce this site used to
    ///    interpose is exactly what the shared mint replaces.
    /// 2. **The value comes from the shared counter** — it lies strictly between two
    ///    readings of `unique_nanos` taken around the call. A private counter reading the
    ///    raw clock cannot satisfy this on any host whose clock is coarser than a
    ///    nanosecond (macOS truncates to microseconds), because its reading ties the one
    ///    before it. Claim 1 is the platform-independent half; claim 2 is the direct one.
    ///
    /// Given both sites draw from the one mint, their cross-site distinctness follows from
    /// the mint's own proven injectivity ([`crate::tempname`] tests) — it is not re-proven
    /// here.
    #[test]
    fn the_store_scratch_path_draws_its_disambiguator_from_the_shared_mint() {
        let before = crate::tempname::unique_nanos();
        let scratch = store_scratch_path();
        let after = crate::tempname::unique_nanos();

        assert_eq!(
            scratch.parent(),
            Some(std::env::temp_dir().as_path()),
            "the task-less sweep's scratch lives in the temp dir, never a managed location",
        );
        let name = scratch
            .file_name()
            .and_then(|n| n.to_str())
            .expect("the scratch path has a UTF-8 filename");
        let middle = name
            .strip_prefix("jigc-")
            .and_then(|rest| rest.strip_suffix(&format!("-{STORE_SNAPSHOT_FILE}")))
            .unwrap_or_else(|| panic!("{name} is jigc-<pid>-<nanos>-{STORE_SNAPSHOT_FILE}"));

        let parts: Vec<&str> = middle.split('-').collect();
        assert_eq!(
            parts.len(),
            2,
            "{name} is jigc-<pid>-<nanos>-<file> — the shared mint replaces the private \
             sequence nonce, so there is no third component",
        );
        assert_eq!(
            parts[0],
            std::process::id().to_string(),
            "{name} carries this process's pid, which separates concurrent processes",
        );

        let nanos: u128 = parts[1]
            .parse()
            .unwrap_or_else(|_| panic!("{name} carries a numeric nanos component"));
        assert!(
            before < nanos && nanos < after,
            "the disambiguator {nanos} must come from the shared mint (between \
             {before} and {after}), not a private clock read",
        );
    }
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
    // Task scope always adjudicates a **materialized** tree, never the working tree: the
    // index sweep reads the staged set `finalize` commits, and the base sweep reads the
    // materialized HEAD versions of the same set (its content findings are only compared
    // by address — subtracted or kept — never rendered). So the root kind the probe reports
    // is the staged index, and a cited file present on disk but unstaged routes to `git add`
    // (`validation.md`:279).
    let snapshot = EffectiveStateSnapshot::new(
        anchors.to_vec(),
        code_tree_root.to_path_buf(),
        RootKind::StagedIndex,
    );
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

/// The task's staged doc instances under `<dir>/docs/` — the bare `<type>:<slug>.md`
/// filenames, in **path-sorted** order (the stable sweep order). A working area with
/// no `docs/` dir (nothing staged yet) yields an empty list, not an error. Only
/// `*.md` files are instances.
fn staged_instances(dir: &Path) -> std::io::Result<Vec<String>> {
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
    Ok(names)
}

/// The repo-relative committed destination a staged `<type>:<slug>.md` instance
/// promotes to at finalize — `<location>/<slug>.md`, or a **placement** doctype's
/// literal `placement.file` ([`crate::store::canonical_path`] rooted at the empty
/// path, so the placement branch is included) — or `None` for a transient-sink type
/// (no committed home) or a type the resolved cascade does not define.
fn staged_destination(filename: &str, schemas: &BTreeMap<String, Schema>) -> Option<String> {
    let identity = filename.strip_suffix(".md").unwrap_or(filename);
    let (ty, slug) = identity.split_once(':')?;
    let schema = schemas.get(ty)?;
    crate::store::canonical_path(Path::new(""), schema, slug)
        .map(|p| p.to_string_lossy().into_owned())
}

/// The per-instance **display identity** of one staged instance (M43 A14;
/// `design/surface-contract.md` → law 1's two legal forms): a **persisted** instance
/// displays at its repo-relative repo-real destination ([`staged_destination`]); a
/// **transient-sink** or **unknown-type** instance has no committed home, so it
/// displays at its `<type>:<slug>` identity — never the `docs/<type>:<slug>.md`
/// working-area fiction the pre-M43 sweep printed.
fn staged_display(filename: &str, schemas: &BTreeMap<String, Schema>) -> String {
    staged_destination(filename, schemas)
        .unwrap_or_else(|| filename.strip_suffix(".md").unwrap_or(filename).to_owned())
}

/// Run `schema-conformance` over one staged instance: resolve its type from the
/// `<type>:<slug>.md` filename, parse against the schema, and surface parse-level
/// findings or the [`schema_conformance`] checks. A type with no schema in the
/// resolved cascade raises a blocking `schema-conformance.unknown-type`, keyed —
/// message and target — at the `<type>:<slug>` identity derivable from the staged
/// filename (M43 A14: the typed-identity form; `display` equals that identity for an
/// unknown type by [`staged_display`]'s construction), routed at its recovery like
/// every gate block ([`conformance_route`] — the route floor).
fn conformance_for(
    filename: &str,
    schemas: &BTreeMap<String, Schema>,
    display: &str,
    source: &str,
) -> Vec<Finding> {
    let ty = filename.split(':').next().unwrap_or(filename);
    // Identity is the staged `<type>:<slug>.md` filename minus its extension — the URI the
    // path→URI flip keys on (never the `docs/<type>:<slug>.md` working-area path).
    let identity = filename.strip_suffix(".md").unwrap_or(filename);
    let Some(schema) = schemas.get(ty) else {
        return vec![blocking_conformance(
            "schema-conformance.unknown-type",
            format!(
                "staged doc `{identity}` has type `{ty}`, which the resolved cascade does not define"
            ),
            Some(Location::addressed(identity, 1, 1)),
        )];
    };
    let mut findings = match parse_sections(schema, source) {
        Ok(doc) => schema_conformance(schema, source, &doc),
        Err(parse_findings) => parse_findings,
    };
    attribute_to_doc(&mut findings, identity, display);
    findings
}

/// Thread the owning doc's `identity` + `display` into every `schema-conformance` /
/// `conformance.*` finding so each names the doc it came from — never a sibling's
/// (`design/validation.md` → Findings: `target` is the address the finding concerns). The
/// bare per-instance checks emit doc-less messages and fragment-only / absent addresses
/// (`section/field`, `section/item/leaf`, or `None`), so a multi-doc sweep produces
/// indistinguishable findings; this post-pass attributes each in place:
///
/// - **message** — prefixed with `` `<display>`:  `` so the rendered
///   `severity · code — message` line an operator reads names the doc. `display` is
///   the committed filesystem path at store scope, and the M43 A14 staged display
///   identity at task scope ([`staged_display`] — the repo-real destination for a
///   persisted instance, the `<type>:<slug>` identity for a transient one; law 1 of
///   `design/surface-contract.md`).
/// - **`Location.address`** — the JSON `target` channel, in **URI normal form**
///   (`command-output-contract.md` → the stable finding key): the **`path→URI` flip** — a
///   fragment-bearing address becomes `<identity>#<fragment>`, a fragment-less or
///   location-less finding is addressed at the bare `<identity>` (`<type>:<slug>`, the
///   addressing grammar that survives rename), **not** the `<location>/<slug>.md`
///   filesystem path it emitted before. **`line`/`col` are preserved** — attribution never
///   moves the source coordinate the check raised. The derived `key.target`
///   ([`Finding::key`]) reads through this URI address.
///
/// The single helper both the task-scope [`conformance_for`] and the store-scope
/// [`schema_conformance_store`] / [`hollow_surplus_store`] apply to their parse arms
/// (reuse-proven — every caller holds the `<type>:<slug>` identity, including a **placement**
/// doctype's `<type>:<type>` singleton identity, so the flip resolves placement docs too).
/// The task-scope `unknown-type` arm and the store `route`-labeler mint their own
/// message + address, so they are left untouched.
fn attribute_to_doc(findings: &mut [Finding], identity: &str, display: &str) {
    for finding in findings.iter_mut() {
        finding.message = format!("`{display}`: {}", finding.message);
    }
    // The address half is the shared `path→URI` flip ([`engine::finding::readdress_to_uri`]) —
    // the same one `jigc ingest`'s near-miss row applies, so a defect keys identically
    // whichever verb reports it.
    crate::finding::readdress_to_uri(findings, identity);
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
/// The **#5 owner-artifact presence gate** over a task's staged instances — the
/// finalize-time check that each staged completion-record's recorded `owned-location`
/// artifact is present, safe, and durably **tracked** (`design/methodology-docs.md` →
/// The engine work, item 3; `design/finalize.md` → 5. Stage). Since M45 Inc 8 T2 this is
/// the gate's home: it is **no longer run inside [`validate_task`]** (the shared phase-2
/// entry) but re-invoked by the CLI finalize transaction **after phase-5 staging**, with a
/// `tracked` predicate built over the just-staged index — so a finalize that stages the
/// recorded artifact in-transaction satisfies the gate in the same commit (the natural
/// pre-staged authoring order lands, `DECISIONS.md` → 2026-07-23 Decision 5).
///
/// **Two callers since M47 Inc 4 / T2** (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1):
/// the CLI finalize transaction's post-stage site above, and the `jigc task validate`
/// *preview*, which invokes this same function under a **constant-true** `tracked`
/// predicate. That predicate is the split: with it, `owned_location_violation`'s untracked
/// branch is unreachable, so the preview reports the six staging-independent causes (empty ·
/// absolute · `..` · not-under-home · names-no-file · symlink-escape) and cause 7 stays
/// post-stage — Decision 6's rationale (a *pre-stage* `tracked` check false-positives a
/// state phase-5 staging repairs) narrowed to the one cause it actually reaches, not
/// overturned. The whole axis is swept at `crates/cli/tests/owner_artifact_cause_axis.rs`.
///
/// Walks each staged `<type>:<slug>.md` instance's `owned-location` leaves via
/// [`owner_artifact_present`]; an instance carrying no such field yields nothing (the
/// omitting-context inert path). The `tracked` predicate is the engine's only
/// tracked-status channel (the engine never shells out — the determinism boundary).
pub fn owner_artifacts_gate(
    dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    repo_root: &Path,
    tracked: &TrackedPredicate<'_>,
) -> std::io::Result<Vec<Finding>> {
    let mut findings = Vec::new();
    for filename in staged_instances(dir)? {
        let bytes = std::fs::read(dir.join(DOCS_DIR).join(&filename))?;
        // The per-instance display identity (M43 A14): a persisted instance displays at its
        // repo-real committed destination — the same identity the phase-2 sweep used.
        let display = staged_display(&filename, schemas);
        let source = String::from_utf8_lossy(&bytes);
        findings.extend(owner_artifact_present(
            &filename, schemas, &display, &source, repo_root, tracked,
        ));
    }
    Ok(findings)
}

fn owner_artifact_present(
    filename: &str,
    schemas: &BTreeMap<String, Schema>,
    display: &str,
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
    // The subject is a **node inside a managed doc**, so the finding keys in URI normal form
    // — the same `<type>:<slug>` identity [`conformance_for`] flips to, never the
    // `<location>/<slug>.md` filesystem path (which breaks on `jigc rename` and is not the
    // addressing grammar; `command-output-contract.md` → the target forms). This gate emits
    // its own address, so the flip happens here rather than in [`attribute_to_doc`].
    let identity = filename.strip_suffix(".md").unwrap_or(filename);
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
            if let Some(violation) = owned_location_violation(path, repo_root, tracked) {
                let message = format!(
                    "owner-artifact `{}` in section `{}` of `{display}`: {}",
                    declared.id, section.id, violation.reason
                );
                let location = Some(Location::addressed(
                    format!("{identity}#{}/{}", section.id, declared.id),
                    1,
                    1,
                ));
                // Cause-aware route (M45 Inc 8 T1, Decision 5): the **untracked** cause is
                // the natural pre-staged authoring order's single missing step — the
                // artifact is present, safe, and at the recorded path, just not staged — so
                // the route names `git add <path>`. A `Route::mechanical` argv must lead
                // with `jigc` (`crates/cli/src/route_fence.rs`), so this is a `Route::human`
                // naming the command verbatim — the `finalize.carried-staged` precedent,
                // which names `git restore --staged` exactly that way. Every other cause
                // (absent / misplaced / unsafe) is a place-or-correct human judgment (the
                // default `conformance_route` for the code).
                findings.push(if violation.untracked {
                    Finding::graded(
                        Severity::Blocking,
                        "owner-artifact.present",
                        message,
                        location,
                        Some(Route::human(format!(
                            "the artifact is present but not staged — stage it with \
                             `{}` so it is durably committed with this task",
                            crate::finding::git_at(
                                repo_root,
                                &format!("add -- {}", crate::finding::shell_token(path.trim())),
                            )
                        ))),
                    )
                } else {
                    blocking_conformance("owner-artifact.present", message, location)
                });
            }
        }
    }
    findings
}

/// The adjudication of one `owned-location` path: the human-readable `reason` the finding
/// carries, plus whether the cause is the **untracked** one (the artifact is safe + present
/// at the recorded path but not staged). The emit site reads `untracked` to pick a
/// cause-aware route — `git add <path>` for the untracked cause (the natural pre-staged
/// order's single missing step), the default place-or-correct route otherwise.
struct OwnedLocationViolation {
    reason: String,
    untracked: bool,
}

impl OwnedLocationViolation {
    /// A non-untracked violation (absent / misplaced / unsafe) — the default route.
    fn misplaced(reason: String) -> Self {
        Self {
            reason,
            untracked: false,
        }
    }
}

/// Adjudicate one `owned-location` path: `None` when the artifact is safe + durably
/// present + tracked, else `Some(violation)` carrying the human-readable cause and the
/// untracked-cause flag ([`OwnedLocationViolation`]). Reads only the path string, the
/// file's presence under `repo_root`, and the `tracked` flag — never the artifact's bytes
/// (the determinism boundary).
fn owned_location_violation(
    path: &str,
    repo_root: &Path,
    tracked: &TrackedPredicate<'_>,
) -> Option<OwnedLocationViolation> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Some(OwnedLocationViolation::misplaced(
            "the path is empty".to_string(),
        ));
    }
    if Path::new(trimmed).is_absolute() || trimmed.starts_with('/') {
        return Some(OwnedLocationViolation::misplaced(format!(
            "`{trimmed}` is absolute, not a repo-relative path"
        )));
    }
    // A `..` component would let the path climb out of the owned home (and out of the
    // repo). Reject on the *textual* component, before any filesystem resolution.
    if Path::new(trimmed)
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Some(OwnedLocationViolation::misplaced(format!(
            "`{trimmed}` contains a `..` component"
        )));
    }
    if !trimmed.starts_with(OWNED_ARTIFACT_HOME) || trimmed.len() == OWNED_ARTIFACT_HOME.len() {
        return Some(OwnedLocationViolation::misplaced(format!(
            "`{trimmed}` is not under the owned artifact home `{OWNED_ARTIFACT_HOME}<milestone>/`"
        )));
    }
    // Presence under the repo root. The path is repo-relative + `..`-free, so the join
    // stays within the tree textually; a symlink could still escape, caught next.
    let full = repo_root.join(trimmed);
    if !full.exists() {
        return Some(OwnedLocationViolation::misplaced(format!(
            "`{trimmed}` names no file under the repository"
        )));
    }
    // Symlink-escape: the resolved real path must stay under the owned home. Canonicalize
    // both the home and the target (the home must resolve too — `..`-free + present).
    let home_real = repo_root.join(OWNED_ARTIFACT_HOME).canonicalize().ok();
    match (full.canonicalize().ok(), home_real) {
        (Some(real), Some(home)) if real.starts_with(&home) => {}
        _ => {
            return Some(OwnedLocationViolation::misplaced(format!(
                "`{trimmed}` resolves outside the owned artifact home (symlink escape)"
            )));
        }
    }
    if !tracked(trimmed) {
        // The untracked cause — safe + present, just not staged. The emit site routes it
        // to `git add <path>` (M45 Inc 8 T1).
        return Some(OwnedLocationViolation {
            reason: format!(
                "`{trimmed}` is present but untracked — stage it so it is durably committed"
            ),
            untracked: true,
        });
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
                check_repeatable(
                    section,
                    repeatable,
                    parsed,
                    source,
                    &schema.ty,
                    &mut findings,
                );
            }
        }
    }
    findings
}

/// The **`schema-conformance.repeatable-populated`** advisory (M40;
/// `design/validation.md` → Hollow and surplus adoption): a top-level repeatable
/// section that parses **zero items** is structurally hollow — the adoption trial
/// adopted a zero-item roadmap clean, so the emptiness must be *visible*, never a
/// gate. One [`Severity::Advisory`] finding per empty repeatable section, addressed
/// at the section fragment; a populated section, a simple section, or a schema with
/// no repeatable yields nothing (the omitting context stays inert).
///
/// `exempt` is the resolved value of the
/// `validation.schema-conformance.repeatable-populated.exempt` string knob — a
/// space-separated list of `doctype#section` tokens naming sections where zero items
/// **is** the steady state (pack-default: `changelog#unreleased-changes
/// milestone-record#tasks completion-record#findings`). A matching token suppresses
/// the finding. Top-level sections only — the token grammar addresses no nested
/// repeatable, and a hollow parent already surfaces.
///
/// Fire points are the store sweep ([`validate_store_families`]) and the CLI's
/// adopt-time triage re-parse — **never** [`validate_task`] / the finalize gate
/// (completeness, not integrity; the MVP minimum-cardinality posture preserved).
pub fn repeatable_populated(schema: &Schema, doc: &Document, exempt: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for section in &schema.sections {
        let SectionBody::Repeatable { .. } = &section.body else {
            continue; // a simple/header section has no items to be hollow of.
        };
        let token = format!("{}#{}", schema.ty, section.id);
        if exempt.split_whitespace().any(|t| t == token) {
            continue; // zero items is this section's declared steady state.
        }
        let is_empty = doc
            .sections
            .iter()
            .any(|s| s.id == section.id && s.items.is_empty());
        if !is_empty {
            continue;
        }
        findings.push(Finding::graded(
            Severity::Advisory,
            "schema-conformance.repeatable-populated",
            format!(
                "repeatable section `{}` parses zero items — structurally empty",
                section.id
            ),
            Some(Location::addressed(section.id.clone(), 1, 1)),
            Some(
                format!(
                    "populate the section, or exempt `{token}` via the \
                 `validation.schema-conformance.repeatable-populated.exempt` knob"
                )
                .into(),
            ),
        ));
    }
    findings
}

/// The **`schema-conformance.surplus-sections-absent`** advisory (M40;
/// `design/validation.md` → Hollow and surplus adoption): **trailing** H2 headings
/// beyond the schema's body sections are invisible to the positional parse —
/// body sections map positionally onto H2s and the parser never visits a trailing
/// surplus (`implementation/parsing.md` → Surplus-section tolerance), so a
/// byte-faithful adoption carries them silently. A raw-block scan
/// ([`crate::parse::scan_blocks`]) closes the visibility gap: count the source's
/// H2 headings against the schema's body-section count and emit one
/// [`Severity::Advisory`] finding naming the surplus count. The parsed [`Document`]
/// still **never carries surplus sections** — the advisory is visibility, never a
/// structural admission (the surplus bytes stay on disk and survive splices
/// untouched).
///
/// **Trailing only.** A surplus H2 *between* required sections shifts the
/// positional mapping and is `conformance.section-renamed`'s territory (a
/// parse-level block); this check stays inert there — when any of the first N H2s
/// does not slug-match its positional section id, the scan yields nothing, so the
/// two never double-fire.
///
/// Fire points are the store sweep ([`validate_store_families`]) and the CLI's
/// adopt-time triage re-parse — **never** [`validate_task`] / the finalize gate
/// (completeness, not integrity).
pub fn surplus_sections_absent(schema: &Schema, source: &str) -> Vec<Finding> {
    let body_ids: Vec<&str> = schema
        .sections
        .iter()
        .filter(|s| !s.header)
        .map(|s| s.id.as_str())
        .collect();
    let h2s: Vec<(String, usize)> = crate::parse::scan_blocks(source)
        .into_iter()
        .filter_map(|block| match block {
            crate::parse::Block::Heading {
                level: pulldown_cmark::HeadingLevel::H2,
                text,
                line,
                ..
            } => Some((text, line)),
            _ => None,
        })
        .collect();
    if h2s.len() <= body_ids.len() {
        return Vec::new(); // no surplus (a missing section is section-missing's territory).
    }
    // Trailing only: when any of the first N H2s does not match its positional section
    // id, the surplus sits BETWEEN required sections and the parse trips
    // `conformance.section-renamed` — this check must not double-fire. The compare is
    // the *recognition* rule (`slug::renormalize`, the same inverse `parse::heading_matches`
    // uses), never the *mint* rule: `slugify` caps and drops edge stopwords, so it would
    // read a conformant `## In Scope` as a mismatch and silently drop this advisory for
    // every doc of that doctype.
    if body_ids
        .iter()
        .zip(&h2s)
        .any(|(id, (text, _))| crate::slug::renormalize(text) != **id)
    {
        return Vec::new();
    }
    let surplus = &h2s[body_ids.len()..];
    let (first_text, first_line) = &surplus[0];
    vec![Finding::graded(
        Severity::Advisory,
        "schema-conformance.surplus-sections-absent",
        format!(
            "{} trailing surplus section heading(s) beyond the schema's {} body \
             section(s), starting at `## {first_text}` — the positional parse never \
             visits them, so the content is carried byte-faithful but unmanaged",
            surplus.len(),
            body_ids.len(),
        ),
        // `#<surplus-heading>` (the first surplus H2's slug) so the flip keys this advisory
        // distinctly from a same-doc section/field finding (`command-output-contract.md` →
        // surplus → `#<surplus-heading>`); `line` stays the first surplus heading's.
        Some(Location::addressed(
            crate::slug::slugify(first_text),
            *first_line,
            1,
        )),
        Some(
            "fold the surplus content into a schema section or remove it — jigc \
             never reads or splices it"
                .into(),
        ),
    )]
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
    doctype: &str,
    findings: &mut Vec<Finding>,
) {
    for item in &parsed.items {
        // The address path up to and including this item: `section/item` at the top
        // level, deepening one segment per nesting level as the recursion descends.
        let item_path = format!("{}/{}", section.id, item.id);
        check_item_leaves(&item_path, repeatable, item, source, doctype, findings);
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
    doctype: &str,
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
                    check_id_from_enum(item_path, repeatable, item, doctype, findings);
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
                    check_item_leaves(&child_path, nested, child, source, doctype, findings);
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

/// The verdict of the shared id-from adjudicator [`id_from_enum_violation`]: a heading
/// that is malformed **as an id source** (independent of the field's type), or one that
/// re-slugs **outside** the id-from's declared enum. Both doors (`add-item` write-time
/// and the task gate) match on it to compose their message; the finding **code** is the
/// same [`ID_FROM_ENUM_CODE`] for either arm (no new check minted — M45 inc-4).
#[derive(Debug)]
pub enum IdFromViolation {
    /// The heading text cannot serve as a stable id source — empty, whitespace-only,
    /// an embedded newline, or leading/trailing whitespace. Carries a self-contained
    /// human clause (`"is empty"` / `"is whitespace-only"` / …). The **universal** rule:
    /// it fires whatever the id-from field's declared type, and reads the heading RAW.
    Shape(&'static str),
    /// The heading is not a well-shaped **git trailer token** — it carries internal
    /// whitespace (`BREAKING CHANGE`) or a colon, either of which breaks the
    /// `%(trailers)` block the `commit` doctype renders each `key: value` trailer into
    /// ([`crate::write`] → `trailer_lines`). Carries a self-contained human clause
    /// (`"contains whitespace"` / `"contains a colon"`). **Commit-scoped** (M45 inc-4 /
    /// T4): the doctype whose sink is the git message is a documented special case (the
    /// `COMMIT_*` well-known-id coupling). Reads the heading RAW — the slug
    /// `breaking-change` is a well-shaped token, which is why the named enum seam is
    /// blind to it.
    TrailerKeyShape(&'static str),
    /// The re-slugged heading is not a member of the id-from's declared `enum`. Carries
    /// the slug-cased id the caller addresses.
    NotEnumMember(String),
}

/// The **commit-trailer** key-shape rule (M45 inc-4 / T4; `design/validation.md` → the
/// M45 registrations; `design/finalize.md` → Commit-doc rendering). A `commit` trailer
/// key renders into a git trailer footer line `key: value`; git recognizes a trailer
/// only when its token carries no whitespace, and a colon would be read as the
/// separator — so a key with either **breaks the whole `%(trailers)` block**. Read the
/// key **RAW**, never the slug (`BREAKING CHANGE` slugs to the well-shaped token
/// `breaking-change`, so a slug-based test is blind to exactly the input this rejects).
/// Returns the human clause, else `None`.
fn commit_trailer_key_violation(title: &str) -> Option<&'static str> {
    if title.chars().any(char::is_whitespace) {
        Some("contains whitespace")
    } else if title.contains(':') {
        Some("contains a colon")
    } else {
        None
    }
}

/// The **universal** id-from shape rule (M45 inc-4 / T3; `design/validation.md`;
/// `implementation/pinning.md` §2) — applied to every id-from's raw heading text, whatever
/// the field's declared type. A repeatable's `id-from` value *is* the item heading, and the
/// item id is slugged from it, so a heading that is empty, whitespace-only, carries an
/// embedded newline, or has surrounding whitespace cannot serve as a stable id source.
/// Read the heading **RAW**, never the slug: the slug drops exactly the characters this
/// catches (` Foo ` and `Foo` slug identically, and `BREAKING CHANGE` slugging past a
/// slug-based enum test is the same blindness), so a slug-based test would be blind to
/// three of the four cases. Returns the human clause, else `None`.
fn id_from_shape_violation(title: &str) -> Option<&'static str> {
    if title.is_empty() {
        Some("is empty")
    } else if title.trim().is_empty() {
        Some("is whitespace-only")
    } else if title.contains('\n') || title.contains('\r') {
        Some("contains an embedded newline")
    } else if title != title.trim() {
        Some("has leading or trailing whitespace")
    } else {
        None
    }
}

/// The shared id-from adjudicator — the single owner of the shape + enum-membership
/// discipline both check times route through (review S3). A repeatable's `id-from` value
/// lives in the item *heading* (rendered `### <Title>  {#slug}`), and both rules read that
/// heading:
///
/// 1. The **universal shape rule** ([`id_from_shape_violation`]) — every id-from heading
///    must be a stable single-line non-blank string, **whatever** the field's type. This
///    is checked first and reads the heading RAW.
/// 2. The **commit-trailer key-shape rule** ([`commit_trailer_key_violation`]) — when
///    `doctype` is `commit` and the id-from is the trailers `key`, the heading additionally
///    must be a well-shaped git trailer token (no internal whitespace, no colon), since it
///    renders into a `key: value` footer line. Commit-scoped (the git-message-sink special
///    case), reads the heading RAW (the slug hides exactly this).
/// 3. The **enum-membership rule** — when the id-from field is an `enum` the heading is
///    additionally schema-constrained: re-slug `title` (the same re-slug the parser's
///    `heading_matches` applies — **not** [`crate::write::check_value`], whose literal
///    compare would reject `Fixed` for the `fixed` member) and test membership in the
///    declared members. A malformed enum schema (no `of`) names no members, so any value
///    is non-conformant — surfaced, not silently passed.
///
/// Returns the [`IdFromViolation`] the caller addresses, else `None`. A **non-enum**
/// id-from (every shipped doctype's `title`/`key`/`version`/`task-id`/`category`) carries
/// no membership constraint but is still subject to the shape rule (and, for `commit`'s
/// trailer `key`, the trailer-token rule). `doctype` is the doc's `type` — the only signal
/// that scopes the commit-special-case rule; every other doctype ignores it.
/// (`design/auto-migration.md` → Engine/validation work #1 / Hardening #3; the M45
/// commit-trailer registration in `design/validation.md`.)
pub fn id_from_enum_violation(
    repeatable: &crate::schema::Repeatable,
    title: &str,
    doctype: &str,
) -> Option<IdFromViolation> {
    // The universal shape rule runs first and is type-independent — it fires even when the
    // id-from field is absent from the block (a mis-declared schema), so it is not gated
    // behind the field lookup below.
    if let Some(reason) = id_from_shape_violation(title) {
        return Some(IdFromViolation::Shape(reason));
    }
    // The commit-trailer key-shape rule (T4) — commit-scoped (the doctype whose sink is
    // the git message is a documented special case) over the `trailers` id-from `key`,
    // whose value renders into a `key: value` git footer line. It runs after the
    // universal shape rule (so an already-blank/edge-whitespace key is caught there) and
    // catches the internal-whitespace / colon case the universal rule does not. Non-enum
    // by schema, so the enum branch below never fires for it.
    if doctype == "commit"
        && repeatable.id_from == "key"
        && let Some(reason) = commit_trailer_key_violation(title)
    {
        return Some(IdFromViolation::TrailerKeyShape(reason));
    }
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
        Some(IdFromViolation::NotEnumMember(slug))
    }
}

/// `field-value-conformant` for a repeatable item's **`id-from`** heading — the one place
/// the heading text (not a bullet in `item.fields`) is itself a schema-constrained value
/// (`design/auto-migration.md` → Engine/validation work #1). The id-from heading is exempt
/// from the ordinary bullet-based field check; the shared [`id_from_enum_violation`]
/// adjudicator owns both the **universal shape** rule (empty / whitespace-only /
/// embedded-newline / leading-trailing-whitespace, applied to every id-from) and the
/// **enum-membership** rule (`Fixed`→`fixed` / `Added`→`added` pass; a foreign
/// `Performance`→`performance` ∉ enum blocks). The finding addresses `item_path/<id-from>`
/// (the slug-cased item id is already in `item_path`).
fn check_id_from_enum(
    item_path: &str,
    repeatable: &crate::schema::Repeatable,
    item: &ParsedItem,
    doctype: &str,
    findings: &mut Vec<Finding>,
) {
    let Some(violation) = id_from_enum_violation(repeatable, &item.title, doctype) else {
        return;
    };
    let detail = match &violation {
        IdFromViolation::Shape(reason) => format!("the heading {reason}"),
        IdFromViolation::TrailerKeyShape(reason) => format!("the trailer key {reason}"),
        IdFromViolation::NotEnumMember(slug) => format!("`{slug}` is not an enum member"),
    };
    findings.push(blocking_conformance(
        ID_FROM_ENUM_CODE,
        format!(
            "id-from field `{}` in item `{item_path}`: {detail}",
            repeatable.id_from
        ),
        Some(Location::addressed(
            format!("{item_path}/{}", repeatable.id_from),
            1,
            1,
        )),
    ));
}

/// `required-slot-present`: the section declares a slot, so its prose must be
/// non-empty. The parser records the slot span even when empty (a present heading
/// with no prose under it), so an all-whitespace slice is the unfilled-slot case.
///
/// Addressed at `#<section>` (`command-output-contract.md` → schema-conformance): a doc has
/// **one required slot per section** but many sections, so a fragment-less finding collapses
/// every empty slot in a doc onto one key under the path→URI flip — a pristine `jigc doc
/// create adr` has three, and emitted three byte-identical keys in one envelope. The section
/// id is the discriminator, and it was already in hand (spent on the message alone). Its
/// siblings in this family were normalized the same way: [`check_field`] /
/// [`check_field_value`] at `#<section>/<field>`, [`check_item_slot_present`] at
/// `#<item-path>/<leaf>`.
fn check_slot_present(
    section: &Section,
    parsed: &ParsedSection,
    source: &str,
    findings: &mut Vec<Finding>,
) {
    let span = parsed.slot.as_ref();
    let filled = span
        .map(|span| !span.slice(source).trim().is_empty())
        .unwrap_or(false);
    if !filled {
        // `line`/`col` stay the slot's source coordinate (the human's pointer); the address
        // is what the key reads.
        let line = span.map(|span| span.start_line).unwrap_or(1);
        findings.push(blocking_conformance(
            "schema-conformance.required-slot-present",
            format!("required slot in section `{}` is empty", section.id),
            Some(Location::addressed(section.id.clone(), line, 1)),
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
                    // `#<section>/<field>` so several missing/invalid fields in one doc key
                    // distinctly under the path→URI flip (`command-output-contract.md` →
                    // schema-conformance → `#<section>/<field>`).
                    Some(Location::addressed(
                        format!("{}/{}", section.id, declared.id),
                        1,
                        1,
                    )),
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
            // `#<section>/<field>` so several invalid fields in one doc key distinctly under
            // the path→URI flip (`command-output-contract.md` → schema-conformance).
            Some(Location::addressed(
                format!("{}/{}", section.id, declared.id),
                1,
                1,
            )),
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
///
/// `pub` since M40 F1: this predicate is the **shared** authority the mint and the
/// read surface consume — [`crate::state`]'s create skeleton pre-stamps exactly the
/// author-required header fields, and the CLI's `doc schema` projection reports
/// `author-required` per field — so provisioning, validation, and introspection can
/// never drift apart (`DECISIONS.md` 2026-07-10 → M40 Settle #4).
pub fn is_author_required(field: &SchemaField) -> bool {
    // An `optional:` field is never author-required — its absence does not block
    // finalize, while a required field still does (`design/changelog.md` → engine
    // work #3). Covers both the simple-section and repeatable-item field arms.
    if field.optional {
        return false;
    }
    if is_optional_ref(field) {
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
    // The `set:` exemption is keyed on the **closed vocabulary**, exhaustively: every
    // honored [`SetKind`] names a deriver the CLI actually runs, so a field carrying one
    // is filled without the author. Before M49 this read `field.set.is_none()`, and a
    // typo bought the exemption forever — `set: on-creat` names no deriver, nothing fills
    // the field, and `required-field-present` stopped asking (`implementation/roadmap.md`
    // → M49 Increment 2). `load_schema_with_types` now refuses that spelling outright;
    // matching the kind rather than presence means the predicate states the truth on its
    // own, and a fourth kind must be dispositioned here.
    if let Some(kind) = field.set.as_deref().and_then(SetKind::parse) {
        match kind {
            SetKind::OnCreate | SetKind::OnTransition | SetKind::SchemaVersion => return false,
        }
    }
    field.default.is_none()
}

/// Whether `field` is an **optional `ref`** — a `ref` whose forward cardinality has
/// a minimum of 0 (`card:` absent ⇒ the `"0..1"` default per `schema.rs` →
/// `SchemaField::card`, or an explicit form starting with `0` — `"0..1"` / `"0..*"`).
/// Such a ref carries no author obligation ([`is_author_required`] exempts it), and
/// the commit doc's fillable form omits its line for the same reason — `pub` so the
/// CLI consumes this one predicate instead of a hand-maintained mirror (M40 F1;
/// `DECISIONS.md` 2026-07-10 → M40 Settle #4).
pub fn is_optional_ref(field: &SchemaField) -> bool {
    field.ty == FieldType::Ref
        && match field.card.as_deref() {
            None => true,
            Some(card) => card.trim_start().starts_with('0'),
        }
}

/// Build a blocking `schema-conformance.*` [`Finding`] (the inventory default
/// severity), **routed** at its repair ([`conformance_route`]) — the route floor,
/// widened to the gate blocks at M43 (`design/surface-contract.md` → The route fence):
/// a blocked finalize names its recovery. "The agent fills the slot / field directly"
/// is now said *by the finding*, never assumed.
fn blocking_conformance(code: &str, message: String, location: Option<Location>) -> Finding {
    assert!(
        CONFORMANCE_ROUTE_CODES.contains(&code),
        "blocking_conformance mints `{code}`, which is absent from CONFORMANCE_ROUTE_CODES \
         — the enumerable domain of conformance_route; add it there too"
    );
    let route = conformance_route(code);
    Finding::graded(Severity::Blocking, code, message, location, Some(route))
}

/// The **enumerable domain of [`conformance_route`]** — every code a
/// [`blocking_conformance`] gate block can mint, as a value a consumer can iterate rather
/// than a shape only the `match` below knows. Total over that domain by assertion, not by
/// census: [`blocking_conformance`] refuses a code absent from this list at its own source,
/// so a seventh arm cannot reach an output without joining the list.
///
/// It exists because the gate-block repair routes are a **class with an axis** — the codes
/// crossed with the doors that emit them (`crate::render::BOUNDARY_DOORS`) — and a test over
/// that axis must read both sides from code (M49 Increment 8 / T3; `implementation/pinning.md`
/// §1, the enumeration seam).
pub const CONFORMANCE_ROUTE_CODES: &[&str] = &[
    "schema-conformance.required-slot-present",
    "schema-conformance.required-field-present",
    "schema-conformance.field-value-conformant",
    "schema-conformance.unknown-type",
    "owner-artifact.present",
];

/// The **gate repair routes**: the [`CONFORMANCE_ROUTE_CODES`] whose declared repair is a
/// copy-runnable `jigc` write, derived by asking [`conformance_route`] rather than by
/// re-listing them. The two-branch-judgment codes (`unknown-type`, `owner-artifact.present`)
/// route [`Route::human`] and are excluded by that derivation, so promoting one to a
/// mechanical route later widens this set with no edit here.
pub fn conformance_repair_codes() -> Vec<&'static str> {
    CONFORMANCE_ROUTE_CODES
        .iter()
        .copied()
        .filter(|code| {
            matches!(
                conformance_route(code).kind(),
                crate::finding::RouteKind::Mechanical { .. }
            )
        })
        .collect()
}

/// The per-code repair route of a [`blocking_conformance`] gate block — one declared map,
/// so every call site minting a code routes it identically. The mechanical routes carry
/// the `<address>` / `<value>` placeholders of the CLI-seam dummy table
/// (`crates/cli/src/route_fence.rs` → `DUMMY_SUBSTITUTIONS`); `<address>` is the finding's
/// own `key.target`, and it is **rendered concrete** at the attribution flip
/// ([`crate::finding::readdress_to_uri`] — B1, 2026-07-17 surface review), so the route
/// an agent reads carries the real copy-runnable write address, never the placeholder
/// (`<value>` stays a placeholder: the value is the agent's to supply). A new
/// `blocking_conformance` code must declare its route here — the
/// loud panic is the same posture as the seam assert it feeds (`Finding`'s `Serialize`
/// would refuse the route-less finding anyway; this names the omission at its source).
pub(crate) fn conformance_route(code: &str) -> Route {
    match code {
        "schema-conformance.required-slot-present" => Route::mechanical(
            ["jigc", "doc", "set-slot", "<address>", "--from-file", "-"],
            " to fill the empty slot",
        ),
        "schema-conformance.required-field-present" => Route::mechanical(
            [
                "jigc",
                "doc",
                "set-field",
                "<address>",
                "--value",
                "<value>",
            ],
            " to supply the missing field",
        ),
        // `ID_FROM_ENUM_CODE` shares this code (the id-from-enum violation is a
        // field-value non-conformance), so it shares the route.
        "schema-conformance.field-value-conformant" => Route::mechanical(
            [
                "jigc",
                "doc",
                "set-field",
                "<address>",
                "--value",
                "<value>",
            ],
            " to correct the value",
        ),
        // Two-branch judgment: either a pack/config change dropped the doctype from
        // the resolved cascade mid-task, or the staged file is a stray out-of-band
        // write into the working area — which side is broken is the human's call, so
        // the route is human (`jigc describe` is the check, not the repair).
        "schema-conformance.unknown-type" => Route::human(
            "check the type against `jigc describe` — restore the doctype's cascade \
             entry if a pack/config change removed it, or remove or re-type the stray \
             staged file",
        ),
        // Two-branch judgment: either the recorded path is wrong or the named file is
        // missing — which side is broken is the human's call, so the route is human.
        "owner-artifact.present" => Route::human(
            "place the owned artifact at the recorded path, or correct the field with \
             `jigc doc set-field` to where the file really lives",
        ),
        other => panic!(
            "blocking_conformance mints `{other}` with no declared route — add it to \
             conformance_route AND to CONFORMANCE_ROUTE_CODES (the route floor, \
             design/surface-contract.md → The route fence)"
        ),
    }
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

    /// The route floor, widened to the gate blocks (M43, `design/surface-contract.md` →
    /// The route fence): every `schema-conformance.*` block the finalize/task gate mints
    /// **carries a route** naming the write verb that repairs it — the empty slot routes
    /// `jigc doc set-slot`, the missing/malformed field routes `jigc doc set-field`. A
    /// blocked finalize names its recovery; "the agent fills the slot directly" is now
    /// said *by the finding*, not assumed.
    #[test]
    fn gate_schema_conformance_blocks_carry_a_route() {
        let schema = schema();
        let cases: &[(&str, &str)] = &[
            (MISSING_SLOT, "jigc doc set-slot"),
            (MISSING_FIELD, "jigc doc set-field"),
            (MALFORMED_VALUE, "jigc doc set-field"),
        ];
        for (source, expected_verb) in cases {
            let doc = parse(source);
            let findings = schema_conformance(&schema, source, &doc);
            assert_eq!(findings.len(), 1, "one finding, got {findings:?}");
            let route = findings[0].route.as_deref().unwrap_or_else(|| {
                panic!(
                    "a gate schema-conformance block must carry a route, got {:?}",
                    findings[0]
                )
            });
            assert!(
                route.contains(expected_verb),
                "the `{}` route must name `{expected_verb}`, got `{route}`",
                findings[0].code,
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
mod id_from_shape_guard_tests {
    //! (M45 inc-4 / T3) The **universal** id-from shape rule. Every id-from's heading
    //! *is* the id source (the slug is derived from it), so — whatever the field's
    //! declared type — it must be a stable single-line non-blank string. An empty,
    //! whitespace-only, embedded-newline, or leading/trailing-whitespace heading is
    //! rejected at BOTH doors, naming the shared
    //! `schema-conformance.field-value-conformant`. Unlike the enum-membership rule the
    //! shape rule applies to a **non-enum** id-from too, and reads the heading **RAW**
    //! (the slug drops the very characters this must catch — the leading/trailing-ws
    //! case slugs *identically* to the clean title, so a slug-based test is blind to it).
    //! A legitimate milestone title passes untouched. See `design/validation.md`;
    //! `implementation/pinning.md` §2.

    use super::*;
    use crate::parse::{Document, ParsedItem, ParsedSection};

    /// A string-`id-from` repeatable — the non-enum class every shipped doctype's
    /// `title`/`key`/`version`/`task-id`/`category` id-from belongs to — whose block
    /// carries ONLY the id-from field (no required slot), so the ONLY finding a
    /// malformed heading can raise is the shape one.
    fn string_repeatable_schema() -> Schema {
        let yaml = b"\
type: roadmap
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
";
        crate::schema::load_schema(yaml).expect("string-id-from schema loads")
    }

    fn repeatable_of(schema: &Schema) -> &crate::schema::Repeatable {
        match &schema.sections[0].body {
            SectionBody::Repeatable { repeatable } => repeatable,
            _ => unreachable!("fixture section is repeatable"),
        }
    }

    /// The shared adjudicator both doors route through rejects EVERY malformed shape
    /// over a **non-enum** id-from, and reads the heading RAW — the leading/trailing-ws
    /// title slugs identically to the clean one, so its rejection (contrasted with the
    /// clean title's pass) proves the guard does not consult the slug — while a
    /// legitimate milestone title passes untouched (the over-rejection guard).
    #[test]
    fn adjudicator_rejects_every_malformed_shape_and_passes_clean() {
        let schema = string_repeatable_schema();
        let r = repeatable_of(&schema);
        for bad in ["", "   ", "line1\nline2", " Milestone 45 — the rc.9 wave "] {
            assert!(
                id_from_enum_violation(r, bad, "roadmap").is_some(),
                "a malformed id-from heading {bad:?} must be rejected over a non-enum id-from",
            );
        }
        assert!(
            id_from_enum_violation(r, "Milestone 45 — the rc.9 wave", "roadmap").is_none(),
            "a legitimate milestone title must pass the shape guard untouched",
        );
    }

    /// A two-level string-`id-from` schema (a milestone repeatable holding a nested
    /// task repeatable), so the task-gate test proves the shape guard fires at **every**
    /// site the conformance seam walks — top level AND nested — not just one.
    fn nested_string_schema() -> Schema {
        let yaml = b"\
type: roadmap
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - id: tasks
          repeatable:
            id-from: title
            block:
              - { id: title, type: string }
";
        crate::schema::load_schema(yaml).expect("nested string-id-from schema loads")
    }

    /// The task gate (`schema_conformance`) blocks a malformed heading at BOTH a
    /// top-level and a nested id-from site, each a blocking
    /// `schema-conformance.field-value-conformant` addressed at that site's id-from
    /// leaf. The heading text is read raw off `item.title`, so the whitespace-only top
    /// item and the embedded-newline nested item are both caught.
    #[test]
    fn task_gate_blocks_top_and_nested_shape_violations() {
        let schema = nested_string_schema();
        // A hand-built parsed document: the parser trims headings, so a shape-broken
        // title is injected directly (the raw heading text a write door would accept).
        let doc = Document {
            sections: vec![ParsedSection {
                id: "milestones".into(),
                slot: None,
                fields: vec![],
                items: vec![ParsedItem {
                    id: "top".into(),
                    title: "   ".into(),
                    slot: None,
                    slots: vec![],
                    fields: vec![],
                    items: vec![ParsedItem {
                        id: "child".into(),
                        title: "line1\nline2".into(),
                        slot: None,
                        slots: vec![],
                        fields: vec![],
                        items: vec![],
                    }],
                }],
            }],
        };
        let findings = schema_conformance(&schema, "", &doc);
        let addresses: Vec<&str> = findings
            .iter()
            .filter(|f| {
                f.code == "schema-conformance.field-value-conformant"
                    && f.severity == Severity::Blocking
            })
            .filter_map(|f| f.location.as_ref().and_then(|l| l.address.as_deref()))
            .collect();
        assert!(
            addresses.contains(&"milestones/top/title"),
            "the top-level shape violation must block at its id-from leaf; got {findings:?}",
        );
        assert!(
            addresses.contains(&"milestones/top/tasks/child/title"),
            "the nested shape violation must block at its section-qualified id-from leaf; \
             got {findings:?}",
        );
    }
}

#[cfg(test)]
mod commit_trailer_key_shape_tests {
    //! (M45 inc-4 / T4) The **commit-trailer** key-shape rule. The `commit` doctype's
    //! `trailers` repeatable is `id-from: key`, and each key renders as a `key: value`
    //! git trailer footer line ([`crate::write`] → `trailer_lines`), so a key that is
    //! not a well-shaped git trailer token — internal whitespace (`BREAKING CHANGE`) or
    //! a colon — breaks the `%(trailers)` block. The shared [`id_from_enum_violation`]
    //! adjudicator rejects it at BOTH doors under the existing
    //! `schema-conformance.field-value-conformant`, reading `item.title` **RAW** (the
    //! slug `breaking-change` is a well-shaped token — why the named enum seam was
    //! blind). The rule is **commit-scoped**: a non-commit id-from with internal
    //! whitespace passes untouched. See `design/validation.md` → the M45 registrations;
    //! `DECISIONS.md` → 2026-07-23 Decision 4.

    use super::*;
    use crate::field_block::{Field, Value};
    use crate::parse::{Document, ParsedItem, ParsedSection};

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    fn trailers_repeatable(schema: &Schema) -> &crate::schema::Repeatable {
        let section = schema
            .sections
            .iter()
            .find(|s| s.id == "trailers")
            .expect("commit has a trailers section");
        match &section.body {
            SectionBody::Repeatable { repeatable } => repeatable,
            _ => unreachable!("trailers is repeatable"),
        }
    }

    /// A `commit` [`Document`] carrying a single trailer item whose heading (the
    /// id-from `key`) is `title` and whose `value` field is `value`. Only the
    /// `trailers` section is materialized — the other schema sections are absent from
    /// the parsed doc, so [`schema_conformance`] skips them (a `continue` on the
    /// missing map), isolating the trailer-key adjudication.
    fn commit_doc_with_trailer(title: &str, value: &str) -> Document {
        Document {
            sections: vec![ParsedSection {
                id: "trailers".into(),
                slot: None,
                fields: vec![],
                items: vec![ParsedItem {
                    id: "the-trailer".into(),
                    title: title.into(),
                    slot: None,
                    slots: vec![],
                    fields: vec![Field {
                        key: "value".into(),
                        value: Value::Scalar(value.into()),
                    }],
                    items: vec![],
                }],
            }],
        }
    }

    fn conformant_key_findings(doc: &Document) -> Vec<Finding> {
        let schema = commit_schema();
        schema_conformance(&schema, "", doc)
            .into_iter()
            .filter(|f| {
                f.code == "schema-conformance.field-value-conformant"
                    && f.severity == Severity::Blocking
            })
            .collect()
    }

    /// The task gate ([`schema_conformance`], the finalize seam) blocks a commit-trailer
    /// key bearing internal whitespace, addressed at the id-from leaf
    /// `trailers/<item>/key`, under the shared `field-value-conformant`.
    #[test]
    fn task_gate_blocks_a_whitespace_trailer_key() {
        let doc = commit_doc_with_trailer("BREAKING CHANGE", "the api changed");
        let addresses: Vec<String> = conformant_key_findings(&doc)
            .iter()
            .filter_map(|f| f.location.as_ref().and_then(|l| l.address.clone()))
            .collect();
        assert!(
            addresses.iter().any(|a| a == "trailers/the-trailer/key"),
            "a whitespace commit-trailer key must block field-value-conformant at its \
             id-from leaf; got addresses {addresses:?}",
        );
    }

    /// A colon-bearing key would break the `%(trailers)` block at the separator; it is
    /// rejected the same way.
    #[test]
    fn task_gate_blocks_a_colon_bearing_trailer_key() {
        let doc = commit_doc_with_trailer("Co:lon", "value");
        assert!(
            !conformant_key_findings(&doc).is_empty(),
            "a colon-bearing commit-trailer key must block field-value-conformant",
        );
    }

    /// A well-shaped hyphenated trailer key (`Co-Authored-By`) is untouched — the rule
    /// must not over-reject the realistic multi-word-but-hyphenated key.
    #[test]
    fn task_gate_passes_a_well_shaped_trailer_key() {
        let doc = commit_doc_with_trailer("Co-Authored-By", "Ada <ada@example.com>");
        assert!(
            conformant_key_findings(&doc).is_empty(),
            "a well-shaped hyphenated trailer key must not fire field-value-conformant; \
             got {:?}",
            conformant_key_findings(&doc),
        );
    }

    /// The rule is **commit-scoped**, proven at the shared adjudicator: the identical
    /// whitespace-bearing key that yields `TrailerKeyShape` under the `commit` doctype
    /// passes untouched under a non-commit doctype (a `roadmap`-shaped id-from) — so the
    /// trailer-token rule cannot leak onto every other doctype's `title`/`key` id-from.
    #[test]
    fn the_trailer_rule_is_commit_scoped() {
        let commit = commit_schema();
        let trailers = trailers_repeatable(&commit);
        assert!(
            matches!(
                id_from_enum_violation(trailers, "BREAKING CHANGE", "commit"),
                Some(IdFromViolation::TrailerKeyShape(_))
            ),
            "a whitespace key must be a TrailerKeyShape violation under `commit`",
        );
        assert!(
            id_from_enum_violation(trailers, "BREAKING CHANGE", "roadmap").is_none(),
            "the same key must pass untouched under a non-commit doctype (commit-scoped)",
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
mod repeatable_populated_tests {
    //! (M40 F4 half 1) The pure `schema-conformance.repeatable-populated` check
    //! (`design/validation.md` → Hollow and surplus adoption): a top-level repeatable
    //! section parsing zero items yields one **advisory** finding; a populated
    //! section, a schema with no repeatable (the omitting context), or a matching
    //! `doctype#section` exempt token yields nothing. Never a gate.

    use super::*;
    use crate::schema::load_schema;

    /// A located doctype with one simple + one repeatable section — the check's
    /// happy-path substrate.
    const LOG_YAML: &[u8] = b"\
type: log
location: logs/
id-from: title
sections:
  - id: intro
    slot: { hint: One sentence. }
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: notes, slot: { hint: The notes. } }
";

    /// A doctype with **no** repeatable section — the omitting context.
    const FLAT_YAML: &[u8] = b"\
type: note
location: notes/
id-from: title
sections:
  - id: body
    slot: { hint: The body. }
";

    fn parse(schema: &Schema, source: &str) -> Document {
        parse_sections(schema, source).expect("fixture parses")
    }

    const HOLLOW_LOG: &str = "\
# Log

## Intro
Prose.

## Entries
";

    const POPULATED_LOG: &str = "\
# Log

## Intro
Prose.

## Entries

### First  {#first}

Some notes.
";

    /// A zero-item repeatable fires exactly one advisory, addressed at the section
    /// fragment, carrying the `(schema-conformance, repeatable-populated)` handle and
    /// a route naming the exempt knob; a populated one is silent.
    #[test]
    fn zero_item_repeatable_fires_one_advisory_populated_is_silent() {
        let schema = load_schema(LOG_YAML).expect("log schema loads");

        let doc = parse(&schema, HOLLOW_LOG);
        let findings = repeatable_populated(&schema, &doc, "");
        assert_eq!(
            findings.len(),
            1,
            "a zero-item repeatable fires exactly one advisory, got {findings:?}"
        );
        let finding = &findings[0];
        assert_eq!(
            finding.severity,
            Severity::Advisory,
            "advisory, never a gate"
        );
        assert_eq!(finding.probe, "schema-conformance");
        assert_eq!(finding.check, "repeatable-populated");
        assert_eq!(finding.code, "schema-conformance.repeatable-populated");
        assert_eq!(
            finding.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("entries"),
            "addressed at the hollow section's fragment",
        );
        assert!(
            finding.route.as_deref().is_some_and(|r| r
                .contains("validation.schema-conformance.repeatable-populated.exempt")
                && r.contains("log#entries")),
            "the route names the exempt knob + this section's token, got {:?}",
            finding.route,
        );

        let doc = parse(&schema, POPULATED_LOG);
        assert!(
            repeatable_populated(&schema, &doc, "").is_empty(),
            "a populated repeatable stays silent",
        );
    }

    /// The omitting context stays **inert**: a schema with no repeatable section
    /// yields nothing (never an error) — the check composed into a doctype that
    /// omits the target must not fire.
    #[test]
    fn schema_without_repeatable_is_inert() {
        let schema = load_schema(FLAT_YAML).expect("flat schema loads");
        let doc = parse(&schema, "# Note\n\n## Body\nProse.\n");
        assert!(
            repeatable_populated(&schema, &doc, "").is_empty(),
            "a doctype with no repeatable section is inert",
        );
    }

    /// A matching `doctype#section` exempt token suppresses the advisory; a
    /// non-matching token list (wrong doctype, wrong section) leaves it firing —
    /// the exemption is exact-token, never substring.
    #[test]
    fn exempt_token_suppresses_only_exact_matches() {
        let schema = load_schema(LOG_YAML).expect("log schema loads");
        let doc = parse(&schema, HOLLOW_LOG);

        assert!(
            repeatable_populated(&schema, &doc, "log#entries").is_empty(),
            "the exact `log#entries` token suppresses the advisory",
        );
        assert!(
            repeatable_populated(
                &schema,
                &doc,
                "changelog#unreleased-changes milestone-record#tasks log#entries"
            )
            .is_empty(),
            "a matching token anywhere in the space-separated list suppresses",
        );
        assert_eq!(
            repeatable_populated(&schema, &doc, "other#entries log#other log#entrie").len(),
            1,
            "non-matching tokens never suppress (exact-token, not substring)",
        );
    }
}

#[cfg(test)]
mod surplus_sections_tests {
    //! (M40 F4 half 2) The pure `schema-conformance.surplus-sections-absent` check
    //! (`design/validation.md` → Hollow and surplus adoption): **trailing** H2
    //! headings beyond the schema's body sections yield one **advisory** finding
    //! carrying the surplus count; a conformant doc stays inert; a surplus H2
    //! *between* required sections is `conformance.section-renamed`'s territory and
    //! never double-fires this check. Never a gate.

    use super::*;
    use crate::schema::load_schema;

    /// A located doctype with one simple + one repeatable section — two body
    /// sections for the positional H2 mapping.
    const LOG_YAML: &[u8] = b"\
type: log
location: logs/
id-from: title
sections:
  - id: intro
    slot: { hint: One sentence. }
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: notes, slot: { hint: The notes. } }
";

    /// A conformant log — exactly the schema's two body-section H2s.
    const CONFORMANT: &str = "\
# Log

## Intro
Prose.

## Entries

### First  {#first}

Some notes.
";

    /// The conformant log plus **two trailing** surplus H2s — the adoption-trial
    /// shape (a `## Legacy planning notes` carried structurally silent).
    const TRAILING_SURPLUS: &str = "\
# Log

## Intro
Prose.

## Entries

### First  {#first}

Some notes.

## Legacy Planning Notes
Old notes the positional parse never visits.

## Scratch
More.
";

    /// A surplus H2 **between** the required sections — it shifts the positional
    /// mapping, so it is section-renamed's territory, not this check's.
    const BETWEEN_SURPLUS: &str = "\
# Log

## Intro
Prose.

## Legacy Planning Notes
Old notes.

## Entries
";

    /// (The done-criterion) Two trailing surplus H2s fire exactly one advisory
    /// carrying the surplus count, the `(schema-conformance,
    /// surplus-sections-absent)` handle, and the first surplus heading's line —
    /// while the doc still **parses clean** and the parsed [`Document`] carries
    /// only the schema's sections (visibility, never structural admission).
    #[test]
    fn trailing_surplus_fires_one_advisory_with_the_surplus_count() {
        let schema = load_schema(LOG_YAML).expect("log schema loads");

        let doc =
            parse_sections(&schema, TRAILING_SURPLUS).expect("a trailing surplus parses clean");
        assert_eq!(
            doc.sections.len(),
            2,
            "the parsed Document never carries surplus sections",
        );

        let findings = surplus_sections_absent(&schema, TRAILING_SURPLUS);
        assert_eq!(findings.len(), 1, "one advisory per doc; got {findings:?}");
        let finding = &findings[0];
        assert_eq!(
            finding.severity,
            Severity::Advisory,
            "visibility, never a gate"
        );
        assert_eq!(finding.probe, "schema-conformance");
        assert_eq!(finding.check, "surplus-sections-absent");
        assert_eq!(finding.code, "schema-conformance.surplus-sections-absent");
        assert!(
            finding.message.contains("2 trailing surplus"),
            "the surplus count is in the message: {}",
            finding.message,
        );
        let location = finding.location.as_ref().expect("located");
        assert_eq!(
            location.line, 12,
            "located at the first surplus heading (`## Legacy Planning Notes`)",
        );
    }

    /// M42 inc 10, the validate-site arm: the positional heading compare that
    /// decides whether the surplus is *trailing* recognizes a heading by
    /// **renormalizing** it, never by re-running the *mint* rule. Under a
    /// `slugify`-based compare, a schema whose first body section id carries a
    /// leading edge stopword (`in-scope` → `## In Scope` → `slugify` yields
    /// `scope`) reads as a positional mismatch, so the check returns empty and the
    /// trailing-surplus advisory is **silently lost** on every doc of that doctype.
    #[test]
    fn edge_stopword_section_id_still_reports_trailing_surplus() {
        const BRIEF_YAML: &[u8] = b"\
type: brief
location: briefs/
id-from: title
sections:
  - id: in-scope
    slot: { hint: What this covers. }
";
        let schema = load_schema(BRIEF_YAML).expect("brief schema loads");
        let src = "\
# Brief

## In Scope
The read path only.

## Legacy Planning Notes
Old notes the positional parse never visits.
";
        let findings = surplus_sections_absent(&schema, src);
        assert_eq!(
            findings.len(),
            1,
            "the trailing surplus must still be reported over an edge-stopword \
             section id; got {findings:?}",
        );
        assert!(
            findings[0].message.contains("1 trailing surplus"),
            "the surplus count is in the message: {}",
            findings[0].message,
        );
    }

    /// A doc with exactly the schema's H2s stays inert — the omitting context.
    #[test]
    fn conformant_doc_stays_inert() {
        let schema = load_schema(LOG_YAML).expect("log schema loads");
        assert!(
            surplus_sections_absent(&schema, CONFORMANT).is_empty(),
            "no surplus, no finding",
        );
    }

    /// (The done-criterion) A surplus H2 **between** required sections still trips
    /// `conformance.section-renamed` at parse level and this check stays inert —
    /// the two never double-fire on one doc.
    #[test]
    fn between_surplus_is_section_renamed_territory_never_a_double_fire() {
        let schema = load_schema(LOG_YAML).expect("log schema loads");

        let err = parse_sections(&schema, BETWEEN_SURPLUS)
            .expect_err("a between-surplus breaks the positional mapping");
        assert!(
            err.iter().any(|f| f.code == "conformance.section-renamed"),
            "the parse trips section-renamed: {err:?}",
        );

        assert!(
            surplus_sections_absent(&schema, BETWEEN_SURPLUS).is_empty(),
            "the raw-block scan stays inert on a between-surplus — no double fire",
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
                crate::tempname::unique_nanos(),
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

    /// The caller-supplied conflict presentation the CLI hands `validate_task` (M47 inc-2 /
    /// T4) — a task-scope caller names its real task id, never a placeholder.
    fn test_conflict() -> crate::file_state::ConflictBlock {
        crate::file_state::ConflictBlock::task("drift-the-cache", None)
    }

    /// The done-criterion. Over a working area with **two conformance-broken
    /// instances**, `validate_task` aggregates one finding per instance and
    /// `has_blocking() == true`; over a **clean** area it returns an empty report and
    /// `has_blocking() == false`. (The pre-M43 drift arm over a staged instance is
    /// retired — A14: staged instances never key against the record; see
    /// `staged_transient_instance_yields_no_file_state_findings` for that pin.)
    #[test]
    fn validate_task_aggregates_probe_findings() {
        // --- The broken area: two conformance-broken staged instances.
        let area = TempArea::new("broken");
        area.stage("note:also-broken.md", BROKEN.as_bytes());
        area.stage("note:broken.md", BROKEN.as_bytes());
        let mut record = FileStateRecord::new();

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
            &|_| true,
            &BTreeSet::new(),
            area.dir(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
        )
        .expect("sweep runs");

        // Both instances' blocking findings are present in the aggregate.
        let codes: Vec<&str> = report.findings.iter().map(|f| f.code.as_str()).collect();
        assert_eq!(
            codes,
            vec![
                "schema-conformance.required-slot-present",
                "schema-conformance.required-slot-present",
            ],
            "one conformance finding per broken instance, in path-sorted order"
        );
        assert!(
            report.has_blocking(),
            "a conformance-broken area must block, got {:?}",
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
            &|_| true,
            &BTreeSet::new(),
            clean.dir(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
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

    // ── M43 A14: the honest staged-sweep display + the transient file-state skip ──

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "adr".to_string(),
            crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
                .expect("adr.yaml loads"),
        );
        m
    }

    /// A staged ADR whose required `consequences` slot is empty — one
    /// `schema-conformance.required-slot-present`, nothing else.
    const ADR_BROKEN: &str = "\
---
status: accepted
date: 2026-07-16
---

# Rate limit the gateway

## Context
Clients can flood the gateway.

## Options
Alternatives were weighed and rejected.

## Decision
Throttle per client id.

## Consequences
";

    /// A fully-conformant ADR (every required slot filled, no outgoing ref).
    const ADR_OK: &str = "\
---
status: accepted
date: 2026-07-16
---

# Rate limit the gateway

## Context
Clients can flood the gateway.

## Options
Alternatives were weighed and rejected.

## Decision
Throttle per client id.

## Consequences
Bursty-but-honest clients see occasional 429s.
";

    /// The finding's carried target address, if any.
    fn address(f: &Finding) -> Option<&str> {
        f.location.as_ref().and_then(|l| l.address.as_deref())
    }

    /// (M43 A14, done-criterion 1) A staged **persisted** ADR's staged-sweep findings
    /// name its repo-real committed destination `decisions/<slug>.md` — in the
    /// staged-copy advisory's message AND target, and as the conformance message
    /// prefix — never the fictional `docs/adr:<slug>.md` working-area key
    /// (`design/surface-contract.md` → law 1: every printed path is repo-real or a
    /// typed identity). The record is neither consulted nor advanced.
    #[test]
    fn staged_persisted_adr_reports_at_its_repo_real_destination() {
        let area = TempArea::new("persisted");
        area.stage("adr:rate-limit-the-gateway.md", ADR_BROKEN.as_bytes());
        let mut record = FileStateRecord::new();

        let report = validate_task(
            area.dir(),
            &adr_schemas(),
            &mut record,
            area.dir(),
            area.dir(),
            area.dir(),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &|_| true,
            &BTreeSet::new(),
            area.dir(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
        )
        .expect("sweep runs");

        // Exactly one file-state finding: the staged-copy advisory at the repo-real
        // destination — message AND target name `decisions/<slug>.md`.
        let staged: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code.starts_with("file-state."))
            .collect();
        assert_eq!(
            staged.len(),
            1,
            "one staged-copy advisory and no other file-state finding: {:?}",
            report.findings
        );
        let advisory = staged[0];
        assert_eq!(advisory.code, "file-state.staged-copy");
        assert_eq!(advisory.severity, Severity::Advisory);
        assert!(
            advisory
                .message
                .contains("`decisions/rate-limit-the-gateway.md`"),
            "the advisory's message names the repo-real destination: {advisory:?}",
        );
        assert_eq!(
            address(advisory),
            Some("decisions/rate-limit-the-gateway.md"),
            "the advisory's target is the repo-real path (the file-path form, value \
             corrected): {advisory:?}",
        );
        assert!(
            advisory
                .route
                .as_deref()
                .is_some_and(|r| r.contains("no action needed")),
            "the advisory-route floor: an ignorable advisory says so: {advisory:?}",
        );

        // The record was neither consulted nor advanced — no key minted.
        assert!(
            record.hashes.is_empty(),
            "the staged loop must not mint record keys; got {:?}",
            record.hashes
        );

        // The conformance finding displays the repo-real path in its message prefix;
        // its target stays the URI identity (the address grammar is untouched).
        let slot = report
            .findings
            .iter()
            .find(|f| f.code == "schema-conformance.required-slot-present")
            .expect("the empty consequences slot fires");
        assert!(
            slot.message
                .starts_with("`decisions/rate-limit-the-gateway.md`: "),
            "the conformance message prefix is the repo-real destination: {}",
            slot.message
        );
        assert_eq!(
            address(slot),
            Some("adr:rate-limit-the-gateway#consequences"),
            "the conformance target stays the URI identity: {slot:?}",
        );
    }

    /// (M43 A14, the do-NOT-do-it guard) A task editing an **existing committed** doc —
    /// the copied-in staged instance whose bytes legitimately differ from the committed
    /// baseline — must NOT drift at its repo-real key: the staged sweep never keys the
    /// destination against the record (that would mint false blocking drift on every
    /// in-flight edit; `DECISIONS.md` 2026-07-16 Inc 5 T4).
    #[test]
    fn a_copied_in_committed_doc_never_drifts_at_its_repo_real_key() {
        let area = TempArea::new("copied-in");
        // The committed doc at its canonical home, baselined in the record…
        let decisions = area.dir().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");
        std::fs::write(decisions.join("rate-limit-the-gateway.md"), ADR_OK).expect("commit");
        let mut record = FileStateRecord::new();
        record.record(
            "decisions/rate-limit-the-gateway.md",
            hash_bytes(ADR_OK.as_bytes()),
        );
        let before = record.clone();
        // …and this task's staged copy, mid-edit (different bytes).
        let edited = ADR_OK.replace("occasional", "rare");
        assert_ne!(edited, ADR_OK);
        area.stage("adr:rate-limit-the-gateway.md", edited.as_bytes());

        let report = validate_task(
            area.dir(),
            &adr_schemas(),
            &mut record,
            area.dir(),
            area.dir(),
            area.dir(),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &|_| true,
            &BTreeSet::new(),
            area.dir(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
        )
        .expect("sweep runs");

        assert!(
            report
                .findings
                .iter()
                .all(|f| f.code != "file-state.hash-matches"),
            "an in-flight edit of a committed doc must not drift at the repo-real key: {:?}",
            report.findings
        );
        assert!(
            !report.has_blocking(),
            "the conformant mid-edit copy must not block: {:?}",
            report.findings
        );
        assert_eq!(
            record, before,
            "the staged sweep neither consults nor advances the record",
        );
    }

    /// (M43 A14, done-criterion 2) A staged **transient-sink** instance (no committed
    /// home — the `note`/`commit` shape) yields **zero** `file-state.*` findings — there
    /// is no committed file to baseline or drift, the A14 root cause — while its
    /// `schema-conformance` still fires, displayed AND keyed at the `<type>:<slug>`
    /// identity. A stale legacy-shaped record key is neither consulted nor dropped.
    #[test]
    fn staged_transient_instance_yields_no_file_state_findings() {
        let area = TempArea::new("transient");
        let rel = area.stage("note:broken.md", BROKEN.as_bytes());
        let mut record = FileStateRecord::new();
        record.record(rel.clone(), hash_bytes(CONFORMANT.as_bytes()));
        let before = record.clone();

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
            &|_| true,
            &BTreeSet::new(),
            area.dir(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
        )
        .expect("sweep runs");

        assert!(
            report
                .findings
                .iter()
                .all(|f| !f.code.starts_with("file-state.")),
            "a transient-sink staged instance is file-state-silent: {:?}",
            report.findings
        );
        let slot = report
            .findings
            .iter()
            .find(|f| f.code == "schema-conformance.required-slot-present")
            .expect("the empty body slot still fires");
        assert!(
            slot.message.starts_with("`note:broken`: "),
            "a transient instance displays at its `<type>:<slug>` identity: {}",
            slot.message
        );
        assert_eq!(address(slot), Some("note:broken#body"));
        assert_eq!(
            record, before,
            "the stale legacy key is neither consulted nor dropped",
        );
    }

    /// (M43 A14, the `(code, target)` value correction) The `unknown-type` arm keys —
    /// message AND target — at the `<type>:<slug>` identity derivable from the staged
    /// filename, never the `docs/<type>:<slug>.md` working-area fiction.
    #[test]
    fn unknown_type_keys_at_the_type_slug_identity() {
        let area = TempArea::new("unknown");
        area.stage("mystery:zed.md", b"whatever\n");
        let mut record = FileStateRecord::new();

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
            &|_| true,
            &BTreeSet::new(),
            area.dir(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
        )
        .expect("sweep runs");

        let unknown = report
            .findings
            .iter()
            .find(|f| f.code == "schema-conformance.unknown-type")
            .expect("the unknown type blocks");
        assert_eq!(
            address(unknown),
            Some("mystery:zed"),
            "the target is the typed identity, not a working-area path: {unknown:?}",
        );
        assert!(
            unknown.message.contains("`mystery:zed`") && !unknown.message.contains("docs/"),
            "the message names the identity, never the docs/ fiction: {}",
            unknown.message
        );
        // No file-state finding either — an unknown type has no committed home.
        assert!(
            report
                .findings
                .iter()
                .all(|f| !f.code.starts_with("file-state.")),
            "an unknown-type instance is file-state-silent: {:?}",
            report.findings
        );
    }

    /// (M43 completion audit) The route floor over the `unknown-type` block: the
    /// finding must survive the serialization seam — `Finding`'s `Serialize` asserts
    /// blocking ⇒ route present, and `schema-conformance.` is **not** route-exempt —
    /// and the route must name the recovery for the trigger states (a pack/config
    /// change dropped the doctype from the resolved cascade mid-task, or the staged
    /// file is a stray out-of-band write into the working area): check the type
    /// against `jigc describe`, then restore the cascade entry or remove/re-type the
    /// stray staged file.
    #[test]
    fn unknown_type_block_routes_through_the_serialization_seam() {
        let area = TempArea::new("unknown-route");
        area.stage("mystery:zed.md", b"whatever\n");
        let mut record = FileStateRecord::new();

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
            &|_| true,
            &BTreeSet::new(),
            area.dir(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
        )
        .expect("sweep runs");

        let unknown = report
            .findings
            .iter()
            .find(|f| f.code == "schema-conformance.unknown-type")
            .expect("the unknown type blocks");
        // The seam itself: serializing a route-less, non-exempt blocking finding
        // fires the route-floor assert (`finding.rs` → `Serialize`), so this line
        // alone is the floor's presence proof.
        let wire = serde_json::to_value(unknown).expect("the finding serializes");
        let route = wire["route"]
            .as_str()
            .expect("blocking implies a route, never null");
        assert!(
            route.contains("`jigc describe`"),
            "the route names the doctype check: {route}"
        );
    }

    /// (M43 A14, the placement branch) A staged **placement** doctype instance displays
    /// at its literal `placement.file` — the repo-real destination `canonical_path`
    /// resolves for it — not a `docs/<type>:<type>.md` fiction.
    #[test]
    fn staged_placement_instance_displays_at_its_literal_file() {
        let roadmap = crate::schema::load_schema(
            b"\
type: roadmap
id-from: title
placement: { file: ROADMAP.md }
sections:
  - id: body
    slot: { hint: \"The roadmap.\" }
",
        )
        .expect("placement schema loads");
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("roadmap".to_string(), roadmap);

        let area = TempArea::new("placement");
        area.stage("roadmap:roadmap.md", b"# Roadmap\n\n## Body\n\nThe plan.\n");
        let mut record = FileStateRecord::new();

        let report = validate_task(
            area.dir(),
            &schemas,
            &mut record,
            area.dir(),
            area.dir(),
            area.dir(),
            "HEAD",
            &no_delta_resolved(),
            &unused_invoker(),
            &never_tracked(),
            &|_| true,
            &BTreeSet::new(),
            area.dir(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
        )
        .expect("sweep runs");

        let advisory = report
            .findings
            .iter()
            .find(|f| f.code == "file-state.staged-copy")
            .expect("a placement instance is persisted — the staged-copy advisory fires");
        assert_eq!(
            address(advisory),
            Some("ROADMAP.md"),
            "the placement branch displays at the literal file: {advisory:?}",
        );
        assert!(
            advisory.message.contains("`ROADMAP.md`"),
            "the message names the literal file: {}",
            advisory.message
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
                crate::tempname::unique_nanos(),
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

    /// The caller-supplied conflict presentation the CLI hands `validate_task` (M47 inc-2 /
    /// T4) — a task-scope caller names its real task id, never a placeholder.
    fn test_conflict() -> crate::file_state::ConflictBlock {
        crate::file_state::ConflictBlock::task("drift-the-cache", None)
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

## Options
Alternatives were weighed and rejected.

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

## Options
Alternatives were weighed and rejected.

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
            &|_| true,
            &BTreeSet::new(),
            repo.path(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
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
            &|_| true,
            &BTreeSet::new(),
            repo.path(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
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
            &|_| true,
            &BTreeSet::new(),
            repo.path(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
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
            &|_| true,
            &BTreeSet::new(),
            repo.path(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
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
                crate::tempname::unique_nanos(),
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

    /// The caller-supplied conflict presentation the CLI hands `validate_task` (M47 inc-2 /
    /// T4) — a task-scope caller names its real task id, never a placeholder.
    fn test_conflict() -> crate::file_state::ConflictBlock {
        crate::file_state::ConflictBlock::task("drift-the-cache", None)
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
                Some("completion-record:m16#meta/owner-artifact"),
                "{label}: the finding addresses the owner-artifact field in **URI normal \
                 form** — the doc-node target form, never the `<location>/<slug>.md` \
                 filesystem path (M42 Inc 9 T5; `command-output-contract.md` → the target \
                 forms: a node inside a managed doc takes the URI)",
            );
        }
    }

    /// (M45 Inc 8 T1) The **untracked cause is cause-aware routed**: an owner-artifact
    /// safe + present at the recorded path but not staged routes to `git add <path>` as a
    /// [`RouteKind::Human`] direction (a mechanical route's argv must lead with `jigc`, so
    /// the command is named in human text — the `finalize.carried-staged` precedent). Every
    /// **other** cause (here: absent) keeps the default place-or-correct route, never naming
    /// `git add` — so the cause-awareness is proven at both poles, not just the new arm.
    #[test]
    fn gate_routes_the_untracked_cause_to_git_add() {
        use crate::finding::RouteKind;
        let repo = TempRepo::new("route");
        repo.write("completions/artifacts/M16/audit.md", b"audit transcript\n");

        // The untracked cause: present + safe, tracked-predicate says false.
        let untracked: Box<TrackedPredicate> = Box::new(|_p: &str| false);
        let source = record_with_owner_artifact("completions/artifacts/M16/audit.md");
        let findings = gate(repo.path(), &source, untracked.as_ref());
        assert_eq!(
            findings.len(),
            1,
            "the untracked case fires once, got {findings:?}"
        );
        let route = findings[0].route.as_ref().expect("blocking ⇒ routed");
        assert!(
            matches!(route.kind(), RouteKind::Human),
            "the untracked route is a human direction (git add cannot be a mechanical route \
             — its argv does not lead with `jigc`), got {:?}",
            route.kind()
        );
        assert!(
            route.as_str().contains(&format!(
                "git -C {} add -- completions/artifacts/M16/audit.md",
                repo.path().display()
            )),
            "the untracked route names `git add -- <path>`, aimed at the checkout the task \
             commits in (M53 — the cwd census, C1-05; the `--` is git's own \
             option-parsing guard, which this span had been missing): got: {}",
            route.as_str()
        );

        // The absent cause keeps the default place-or-correct route (never `git add`).
        let absent_source = record_with_owner_artifact("completions/artifacts/M16/missing.md");
        let absent = gate(repo.path(), &absent_source, &always_tracked());
        assert_eq!(
            absent.len(),
            1,
            "the absent case fires once, got {absent:?}"
        );
        let absent_route = absent[0].route.as_ref().expect("blocking ⇒ routed");
        assert!(
            !absent_route.as_str().contains("git add"),
            "a non-untracked cause keeps the place/correct route, never `git add`: {}",
            absent_route.as_str()
        );
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

    /// (M45 Inc 8 T2 — Decision 6) The owner-artifact gate is **no longer** an entry in
    /// `validate_task`'s report — a completion-record naming an **absent** owner-artifact
    /// makes `validate_task` report **no** `owner-artifact.present` finding and **does not
    /// block** — while the relocated [`owner_artifacts_gate`], the post-stage site the CLI
    /// finalize transaction re-invokes, **does** fire the blocker. The staged + tracked case
    /// keeps the gate silent (it lands).
    ///
    /// **Basis narrowed (M47 Inc 4 / T2, `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1).**
    /// The assertions below are unchanged and still hold: this shared phase-2 entry does not
    /// run the gate, which is the property that keeps finalize's preflight free of the
    /// pre-stage `tracked` false-positive. What no longer follows from them is the sentence
    /// this doc used to draw — *"so `jigc task validate` exits 0 on this state"*. The wiring
    /// is CLI-side, and `jigc task validate` now re-invokes [`owner_artifacts_gate`] itself
    /// under a constant-true `tracked` predicate, so an **absent** artifact (the
    /// names-no-file cause, which never consults `tracked`) blocks that door at exit 3.
    /// Cause 7 (untracked) is the one that stays validate-0 / finalize-3; the seven-cause ×
    /// two-surface axis is swept at `crates/cli/tests/owner_artifact_cause_axis.rs`.
    #[test]
    fn owner_artifact_gate_relocated_off_validate_task_to_post_stage() {
        // --- The absent case: validate_task is SILENT (Decision 6), the gate FIRES.
        let repo = TempRepo::new("relocated-absent");
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
            &|_p| true, // the _tracked param is no longer consulted here.
            &|_| true,
            &BTreeSet::new(),
            repo.path(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
        )
        .expect("validate runs");
        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.code == "owner-artifact.present"),
            "validate_task must NOT report the owner-artifact gate any more (Decision 6 — \
             the gate relocated post-stage), got {:?}",
            report.findings
        );
        assert!(
            !report.has_blocking(),
            "this shared phase-2 entry reports no blocking finding for the absent artifact — \
             finalize's preflight must stay free of the pre-stage `tracked` check (the CLI \
             preview door invokes the gate itself; M47 Inc 4 / T2), got {:?}",
            report.findings
        );

        // The relocated post-stage gate DOES fire the blocker over the same task working area.
        let gate_findings = owner_artifacts_gate(&task_dir, &schemas(), repo.path(), &|_p| true)
            .expect("gate runs");
        assert!(
            gate_findings
                .iter()
                .any(|f| f.code == "owner-artifact.present" && f.severity == Severity::Blocking),
            "the post-stage gate blocks on the absent owner-artifact, got {gate_findings:?}"
        );

        // --- The staged + tracked case: the post-stage gate is silent (it lands).
        let repo = TempRepo::new("relocated-staged");
        let task_dir = repo.path().join(".jigc").join("tasks").join("complete-m16");
        repo.write(
            "completions/artifacts/M16/audit.md",
            b"the genuine audit transcript\n",
        );
        stage_record(
            &task_dir,
            &record_with_owner_artifact("completions/artifacts/M16/audit.md"),
        );
        let gate_findings = owner_artifacts_gate(&task_dir, &schemas(), repo.path(), &|_p| true)
            .expect("gate runs");
        assert!(
            gate_findings.is_empty(),
            "a staged + tracked owner-artifact yields no gate finding, got {gate_findings:?}"
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

    /// The caller-supplied conflict presentation the CLI hands `validate_task` (M47 inc-2 /
    /// T4) — a task-scope caller names its real task id, never a placeholder.
    fn test_conflict() -> crate::file_state::ConflictBlock {
        crate::file_state::ConflictBlock::task("drift-the-cache", None)
    }

    /// Regression: the store-sweep scratch path must be unique per call even under
    /// concurrent sweeps in one process. Before the per-call sequence nonce,
    /// [`store_scratch_path`] keyed uniqueness on `pid + nanos` only, so two threads that
    /// sampled the clock in the same nanosecond produced the same path — and one sweep's
    /// `remove_file` then deleted the other's snapshot mid-flight, surfacing as a `NotFound`
    /// flake in the whole-suite gate. Generate many paths across threads; every one must be
    /// distinct.

    #[test]
    fn store_scratch_path_is_unique_under_concurrency() {
        use std::sync::{Arc, Mutex};
        let all = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        for _ in 0..8 {
            let all = Arc::clone(&all);
            handles.push(std::thread::spawn(move || {
                let mut local = Vec::with_capacity(5000);
                for _ in 0..5000 {
                    local.push(store_scratch_path());
                }
                all.lock().unwrap().extend(local);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let paths = all.lock().unwrap();
        let unique: std::collections::HashSet<_> = paths.iter().collect();
        assert_eq!(
            unique.len(),
            paths.len(),
            "store_scratch_path collided under concurrent load: {} of {} paths were duplicates",
            paths.len() - unique.len(),
            paths.len(),
        );
    }

    /// A throwaway committed-store root that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-validate-store-{tag}-{}-{:?}",
                std::process::id(),
                crate::tempname::unique_nanos(),
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

    /// The real methodology `roadmap` — a **placement** doctype (its one instance
    /// lives at the literal `docs/roadmap.md`, `location: None`), the headline
    /// hollow-adoption case the family-5 location walk cannot reach.
    const ROADMAP_YAML: &[u8] = include_bytes!("../../../packs/methodology/schemas/roadmap.yaml");

    /// A `milestone-record`-shaped located doctype whose `tasks` section is a
    /// pack-default exempt token (`milestone-record#tasks` — zero tasks is a valid
    /// just-created state, `design/validation.md` → Hollow and surplus adoption).
    const MREC_YAML: &[u8] = b"\
type: milestone-record
location: docs/milestone-records/
id-from: title
sections:
  - id: tasks
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: notes, slot: { hint: Notes. } }
";

    /// A committed roadmap whose `milestones` repeatable parses **zero items** —
    /// exactly what the adoption trial adopted structurally silent.
    const HOLLOW_ROADMAP: &str = "\
# roadmap

## Milestones
";

    /// A committed spec whose `criteria` repeatable parses **zero items**.
    const HOLLOW_SPEC: &str = "\
# Hollow spec

## Goal
One sentence.

## Criteria
";

    /// The store-test schemas plus the placement `roadmap` — the map the
    /// hollow-adoption sweep tests drive.
    fn schemas_with_roadmap() -> BTreeMap<String, Schema> {
        let mut m = schemas();
        m.insert(
            "roadmap".to_string(),
            crate::schema::load_schema(ROADMAP_YAML).expect("roadmap.yaml loads"),
        );
        m
    }

    /// A resolved cascade whose base carries the `…repeatable-populated.exempt`
    /// string knob at `value` — the shape `jigc validate`'s cascade resolves from the
    /// embedded `knobs.yaml` pack default.
    fn resolved_with_exempt(value: &str) -> crate::cascade::Resolved {
        let mut base = BTreeMap::new();
        base.insert(
            "validation.schema-conformance.repeatable-populated.exempt".to_string(),
            value.to_string(),
        );
        crate::cascade::resolve(
            &crate::cascade::PackDefaultLayer::new("dev-pack", "0.1.0", base, Vec::new()),
            None,
            None,
        )
        .expect("resolves")
    }

    /// (M40 F4 half 1, the done-criterion) The store sweep advises a committed
    /// zero-item **placement** instance (the roadmap at its literal `docs/roadmap.md`
    /// — family 5 skips placement doctypes, so this walk must cover it) AND a
    /// zero-item **located** instance (`specs/hollow.md`), each addressed at
    /// `<rel-key>#<section>`, advisory severity, never flipping the gate.
    #[test]
    fn store_sweep_advises_zero_item_placement_and_located_repeatables() {
        let repo = TempRoot::new("hollow-adoption");
        repo.commit("docs", "roadmap", HOLLOW_ROADMAP);
        repo.commit("specs", "hollow", HOLLOW_SPEC);

        // Baseline the located doc so the file↔CLI-state family stays quiet.
        let mut record = FileStateRecord::new();
        record.record("specs/hollow.md", hash_bytes(HOLLOW_SPEC.as_bytes()));

        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas_with_roadmap(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs");

        let hollow: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "schema-conformance.repeatable-populated")
            .collect();
        let addresses: Vec<&str> = hollow
            .iter()
            .filter_map(|f| f.location.as_ref().and_then(|l| l.address.as_deref()))
            .collect();
        assert_eq!(
            addresses,
            vec!["roadmap:roadmap#milestones", "spec:hollow#criteria"],
            "the placement roadmap AND the located spec each surface one hollow \
             advisory, addressed at their URI identity + section; got {:?}",
            report.findings,
        );
        for finding in &hollow {
            assert_eq!(
                finding.severity,
                Severity::Advisory,
                "hollow adoption is visibility, never a gate: {finding:?}"
            );
        }
        assert!(
            !report.has_blocking(),
            "the advisories alone must not block: {:?}",
            report.findings,
        );
    }

    /// (M40 F4 half 1, the done-criterion) A **pack-default exempt token**
    /// (`milestone-record#tasks`) suppresses the advisory for that `doctype#section`
    /// while a non-exempt hollow section (`roadmap#milestones`) still fires — the
    /// knob read through the resolved cascade, exact-token matching.
    #[test]
    fn store_sweep_exempt_token_suppresses_the_matching_section_only() {
        let repo = TempRoot::new("hollow-exempt");
        repo.commit("docs", "roadmap", HOLLOW_ROADMAP);
        let mrec = "# m40\n\n## Tasks\n";
        repo.commit("docs/milestone-records", "m40", mrec);

        let mut schemas = schemas_with_roadmap();
        schemas.insert(
            "milestone-record".to_string(),
            crate::schema::load_schema(MREC_YAML).expect("milestone-record fixture loads"),
        );
        let mut record = FileStateRecord::new();
        record.record("docs/milestone-records/m40.md", hash_bytes(mrec.as_bytes()));

        // The shipped pack-default token list (`knobs.yaml`).
        let resolved = resolved_with_exempt(
            "changelog#unreleased-changes changelog#releases milestone-record#tasks completion-record#findings",
        );

        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas,
            &resolved,
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs");

        let addresses: Vec<&str> = report
            .findings
            .iter()
            .filter(|f| f.code == "schema-conformance.repeatable-populated")
            .filter_map(|f| f.location.as_ref().and_then(|l| l.address.as_deref()))
            .collect();
        assert_eq!(
            addresses,
            vec!["roadmap:roadmap#milestones"],
            "the exempt `milestone-record#tasks` token suppresses its advisory; the \
             non-exempt roadmap still fires; got {:?}",
            report.findings,
        );
    }

    /// A committed spec carrying **one trailing** surplus H2 — parses clean, so
    /// only the raw-block scan can see it.
    const SURPLUS_SPEC: &str = "\
# Surplus spec

## Goal
One sentence.

## Criteria

## Legacy Planning Notes
Old notes the positional parse never visits.
";

    /// A committed spec with a surplus H2 **between** required sections — the
    /// positional mapping shifts, so family 5's re-parse trips
    /// `conformance.section-renamed` and the surplus advisory must stay quiet.
    const BETWEEN_SPEC: &str = "\
# Between spec

## Goal
One sentence.

## Legacy Planning Notes
Old notes.

## Criteria
";

    /// (M40 F4 half 2, the done-criterion) The store sweep advises a committed
    /// doc's **trailing** surplus H2 (advisory, addressed at the doc, the surplus
    /// count in the message) — while a **between**-surplus doc surfaces through
    /// family 5's `conformance.section-renamed` re-parse instead, with **zero**
    /// surplus advisories (no double fire).
    #[test]
    fn store_sweep_advises_trailing_surplus_and_stays_quiet_on_between_surplus() {
        let repo = TempRoot::new("surplus");
        repo.commit("specs", "surplus", SURPLUS_SPEC);
        repo.commit("specs", "between", BETWEEN_SPEC);

        // Baseline both docs so the file↔CLI-state family stays quiet.
        let mut record = FileStateRecord::new();
        record.record("specs/surplus.md", hash_bytes(SURPLUS_SPEC.as_bytes()));
        record.record("specs/between.md", hash_bytes(BETWEEN_SPEC.as_bytes()));

        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs");

        let surplus: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "schema-conformance.surplus-sections-absent")
            .collect();
        let addresses: Vec<&str> = surplus
            .iter()
            .filter_map(|f| f.location.as_ref().and_then(|l| l.address.as_deref()))
            .collect();
        assert_eq!(
            addresses,
            vec!["spec:surplus#legacy-planning-notes"],
            "only the trailing-surplus doc fires — the between-surplus doc is \
             section-renamed's territory; got {:?}",
            report.findings,
        );
        assert_eq!(
            surplus[0].severity,
            Severity::Advisory,
            "surplus adoption is visibility, never a gate: {:?}",
            surplus[0],
        );
        assert!(
            surplus[0].message.contains("1 trailing surplus"),
            "the surplus count is in the message: {}",
            surplus[0].message,
        );

        assert!(
            report
                .findings
                .iter()
                .any(|f| f.code == "conformance.section-renamed"
                    && f.message.contains("specs/between.md")),
            "the between-surplus doc surfaces through family 5's re-parse: {:?}",
            report.findings,
        );
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

## Options
Alternatives were weighed and rejected.

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

## Options
Alternatives were weighed and rejected.

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
                        // Routed, as the real probe routes it (the route floor).
                        Some("update the citation, or restore the cited symbol".into()),
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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
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

    /// A `StepSource` resolving exactly one step, `only`, with the given body — the
    /// minimal source a schema-ref-bearing store workflow needs.
    struct OnlyStepSource(&'static str);
    impl crate::compose::StepSource for OnlyStepSource {
        fn step(&self, id: &str) -> Option<crate::compose::StepDef> {
            (id == "only").then(|| crate::compose::StepDef {
                id: "only".to_string(),
                body: self.0.to_string(),
                kind: crate::compose::StepKind::Plain,
                states_constraints: Vec::new(),
            })
        }
    }

    /// (M43 T2, the done-criterion) The store sweep's workflow↔refs family resolves
    /// `{{schema:<doctype>}}` refs against the **composed cascade's** doctype set (the
    /// `schemas` map the sweep already receives): a [`StoreWorkflow`] whose step body
    /// solicits a ghost doctype yields the blocking `schema-ref-resolves` finding
    /// **keyed at the pack resource**; `{{schema:adr}}` in a workflow whose **origin
    /// pack does not ship `adr`** (its origin catalog is empty — nothing adr-shaped)
    /// resolves CLEAN against the composed set — the not-per-origin pin, the
    /// deliberate divergence from the command-ref membership path
    /// (`surface-contract.md` → The schema projection).
    #[test]
    fn store_sweep_resolves_schema_refs_against_the_composed_doctype_set() {
        let repo = TempRoot::new("schema-refs");
        let wf_bytes: Vec<u8> = b"---\nwhen: x\n---\n{{ include: step:only }}\n".to_vec();
        let record = FileStateRecord::new();

        // Arm 1 — a ghost doctype: the blocking finding, keyed at the step resource.
        let workflows = vec![StoreWorkflow {
            id: "author-doc".to_string(),
            bytes: wf_bytes.clone(),
            catalog: empty_catalog(),
        }];
        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &workflows,
            &OnlyStepSource("author the doc:\n{{ schema:ghost }}\n"),
            &record,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs");
        let ghost: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "workflow-refs.schema-ref-resolves")
            .collect();
        assert_eq!(
            ghost.len(),
            1,
            "the ghost schema-ref trips exactly one finding: {:?}",
            report.findings,
        );
        assert_eq!(ghost[0].severity, Severity::Blocking);
        assert_eq!(
            ghost[0].key().target.as_deref(),
            Some("step:only"),
            "the finding keys at the pack resource it was raised in, got {:?}",
            ghost[0],
        );
        assert!(report.has_blocking(), "the dangling schema-ref blocks");

        // Arm 2 — `{{schema:adr}}` in a workflow whose origin pack ships no `adr`
        // (empty origin catalog): CLEAN, because membership is asked of the composed
        // set (`schemas()` carries `adr`), never the per-origin surface.
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &workflows,
            &OnlyStepSource("{{schema:adr}}\n"),
            &record,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs");
        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.code == "workflow-refs.schema-ref-resolves"),
            "a composed-set doctype resolves clean regardless of the origin pack: {:?}",
            report.findings,
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

## Options
Alternatives were weighed and rejected.

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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
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

    /// (T1, the done-criterion) An OOB `git mv` of a managed doc — the file moved on disk
    /// while its `record` baseline + every referrer still key the **old** slug — surfaces
    /// **exactly one** `reconciliation.rename` finding (the read-only store-scope rename
    /// detector, firing in the file↔CLI-state family before `ref-resolves`) and **zero**
    /// `ref-resolves` findings for that slug's inbound edge (the scope-subtraction dedup),
    /// while a *separate* unrelated dangling ref is **still** reported (no short-circuit).
    /// The `record` is byte-identical after the sweep (read-only proven).
    ///
    /// RED without the subtraction: the same OOB event would emit a `reconciliation.rename`
    /// finding AND a competing `ref-resolves` dangle for the renamed slug's inbound edge.
    #[test]
    fn validate_store_oob_rename_subtracts_inbound_refs_keeps_unrelated_dangle() {
        let repo = TempRoot::new("oob-rename");

        // The target doc was committed at decisions/moved.md, then OOB `git mv`d to
        // decisions/moved-renamed.md with its bytes preserved (the strong-signal case). Only
        // the new path is on disk; decisions/moved.md is gone (moved away).
        let moved_body = adr_superseding_target();
        repo.commit("decisions", "moved-renamed", &moved_body);

        // A referrer still pointing at the OLD slug → its inbound edge to `adr:moved`.
        let referrer_body = adr_superseding("adr:moved");
        repo.commit("decisions", "referrer", &referrer_body);

        // A SEPARATE, unrelated dangling ref → `adr:ghost` is never committed.
        let unrelated_body = adr_superseding("adr:ghost");
        repo.commit("decisions", "unrelated", &unrelated_body);

        // The record baselines the committed-at-entry paths — crucially decisions/moved.md
        // (now missing on disk) at the moved doc's content hash, so the untracked
        // decisions/moved-renamed.md is a strong-signal content match.
        let mut record = FileStateRecord::new();
        record.record("decisions/moved.md", hash_bytes(moved_body.as_bytes()));
        record.record(
            "decisions/referrer.md",
            hash_bytes(referrer_body.as_bytes()),
        );
        record.record(
            "decisions/unrelated.md",
            hash_bytes(unrelated_body.as_bytes()),
        );
        let record_before = record.clone();

        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs");

        // Exactly one rename finding (the OOB move), strong-signal naming both paths.
        let renames: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "reconciliation.rename")
            .collect();
        assert_eq!(
            renames.len(),
            1,
            "exactly one rename finding for the OOB move, got {:?}",
            report.findings,
        );
        assert!(
            renames[0].message.contains("decisions/moved.md")
                && renames[0].message.contains("decisions/moved-renamed.md"),
            "the strong-signal rename names both old and new paths: {}",
            renames[0].message,
        );

        // Scope-subtraction: ZERO ref-resolves findings for the renamed slug's inbound edge.
        let refs: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.code == "schema-conformance.ref-resolves")
            .collect();
        assert!(
            !refs.iter().any(|f| f.message.contains("adr:moved")),
            "the renamed slug's inbound edge must be subtracted (no competing dangle), got {:?}",
            refs,
        );

        // No short-circuit: the unrelated dangling ref is STILL reported.
        assert!(
            refs.iter().any(|f| f.message.contains("adr:ghost")),
            "an unrelated dangling ref must still be reported (report-all), got {:?}",
            refs,
        );

        // Read-only proven: the record is byte-identical after the sweep.
        assert_eq!(
            record.hashes, record_before.hashes,
            "the store-scope rename detector must not mutate the record",
        );
    }

    /// (T1) The referrer-less case — an OOB `git mv` of a doc **no other doc references**
    /// still surfaces the rename finding. This is the only detector that catches it: there
    /// is no dangling ref for `ref-resolves` to find, so without the strong-signal rename
    /// classifier the move would be entirely invisible to `jigc validate`.
    #[test]
    fn validate_store_detects_referrerless_oob_rename() {
        let repo = TempRoot::new("oob-rename-referrerless");
        let moved_body = adr_superseding_target();
        repo.commit("decisions", "moved-renamed", &moved_body);

        let mut record = FileStateRecord::new();
        record.record("decisions/moved.md", hash_bytes(moved_body.as_bytes()));

        let seen = RefCell::new(Vec::new());
        let report = validate_store_families(
            repo.path(),
            &schemas(),
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs");

        assert_eq!(
            report
                .findings
                .iter()
                .filter(|f| f.code == "reconciliation.rename")
                .count(),
            1,
            "a referrer-less OOB rename still surfaces the rename finding: {:?}",
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

## Options
Alternatives were weighed and rejected.

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
  - id: options
    slot: { optional: true, hint: Alternatives. }
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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs over a conformant store");
        // The family's `schema-conformance.*` output over this store is **accounted for**,
        // not filtered: the empty `versions` map this fixture passes makes `adr` an
        // unversioned doctype, which since M51 Inc 8 / T5 is a *stated* fact rather than
        // silence ([`unversioned_doctype`]) — one advisory for the one committed instance.
        // What the arm claims is that the re-parse surfaces **no break**, and dropping the
        // advisory from the filter without asserting it would be the masking shape.
        // (The local is deliberately not named after the family: `finding.rs`'s producer
        // fence scans this source for check-id spellings, and a method call on a local of
        // that name reads to it as an undisposed id — driven, it reddened that fence.)
        let rows: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.probe == "schema-conformance")
            .collect();
        assert_eq!(
            rows.len(),
            1,
            "a fully conformant store surfaces no schema-conformance break: {:?}",
            report.findings,
        );
        assert_eq!(
            rows[0].code, UNVERSIONED_DOCTYPE_CODE,
            "the one row is the unversioned-doctype advisory this fixture's empty `versions` \
             map earns, never a conformance break: {:?}",
            rows[0],
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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
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

    /// The adr schema carrying the engine-injected schema-version stamp — the shape the
    /// pack loader produces for a persisted frozen doctype. Without it a committed
    /// `schema-version:` header line has no declared home and the parser would drop it, so a
    /// version-stamp test must run against the injected schema, not the bare YAML.
    fn stamped_schemas() -> BTreeMap<String, Schema> {
        let mut m = schemas();
        crate::schema::inject_schema_version_stamp(m.get_mut("adr").expect("adr schema present"));
        m
    }

    /// A committed ADR conformant under the real adr schema, stamped `schema-version: 0`
    /// (below the manifest version 1 — a known-old-version doc the transform can upgrade).
    const ADR_STAMP_0: &str = "\
---
status: accepted
date: 2026-05-23
cites-code: crates/engine/src/validate.rs#validate_task
schema-version: 0
---

# Stamped-zero decision

## Context
Forces.

## Options
Alternatives were weighed and rejected.

## Decision
Decided.

## Consequences
Effects.
";

    /// A committed ADR conformant under the real adr schema, stamped `schema-version: 1`
    /// (at the manifest version — current, so it carries no version-mismatch break).
    const ADR_STAMP_1: &str = "\
---
status: accepted
date: 2026-05-23
cites-code: crates/engine/src/validate.rs#validate_task
schema-version: 1
---

# Stamped-current decision

## Context
Forces.

## Options
Alternatives were weighed and rejected.

## Decision
Decided.

## Consequences
Effects.
";

    /// A committed ADR stamped `schema-version: 0` in the **pre-`options` 3-heading form**
    /// (`context`/`decision`/`consequences`, no `## Options`). Under the current (options-
    /// bearing) adr schema it **fails to parse** — the parser matches body sections strictly
    /// by index, so `decision` at the `options` offset trips `section-renamed`/`section-
    /// missing`. The below-version structural case the M36 adr v1→v2 migration upgrades.
    const ADR_STAMP_0_NO_OPTIONS: &str = "\
---
status: accepted
date: 2026-05-23
cites-code: crates/engine/src/validate.rs#validate_task
schema-version: 0
---

# Pre-options decision

## Context
Forces.

## Decision
Decided.

## Consequences
Effects.
";

    /// (M36 Inc-3) A below-version doc that fails to **parse** under the current schema — a
    /// structural v1→v2 change (the adr `options` slot) makes a historical 3-heading ADR
    /// non-canonical (`section-renamed`/`section-missing`) *before* conformance runs — has
    /// its parse-failure findings routed `migrate`, so `jigc validate` **agrees with**
    /// `jigc migrate-corpus` (which sources the prior shape and upgrades it) instead of
    /// surfacing a blocking-conformance dead-end with no repair direction
    /// (`design/corpus-migration.md` → Acceptance flows: the adr v1→v2 flow; the Finding-2
    /// detector/verb agreement extended to the structural parse-failure case). An
    /// at/above-version parse failure is genuine corruption and stays un-routed.
    #[test]
    fn store_sweep_routes_below_version_parse_failures_migrate() {
        let schemas = stamped_schemas(); // the real, options-bearing (v2) adr schema
        let versions: BTreeMap<String, u32> = [("adr".to_string(), 2u32)].into_iter().collect();

        let route_of = |tag: &str, body: &str| -> Vec<Finding> {
            let repo = TempRoot::new(tag);
            repo.commit("decisions", "doc", body);
            let mut record = FileStateRecord::new();
            record.record("decisions/doc.md", hash_bytes(body.as_bytes()));
            let seen = RefCell::new(Vec::new());
            validate_store_families(
                repo.path(),
                &schemas,
                &no_delta_resolved(),
                &dangling_aware_invoker(&seen),
                &[],
                &EmptyStepSource,
                &record,
                &versions,
                &BTreeMap::new(),
                &[],
            )
            .expect("store sweep runs")
            .findings
            .into_vec()
        };

        // The pre-`options` v0 ADR fails to parse under v2 (section-renamed/missing); every
        // surfaced conformance finding is routed `migrate` (the detector agrees with the verb).
        let below = route_of("below", ADR_STAMP_0_NO_OPTIONS);
        let structural: Vec<&Finding> = below
            .iter()
            .filter(|f| f.code.starts_with("conformance."))
            .collect();
        assert!(
            !structural.is_empty(),
            "the pre-options doc must fail to parse under v2 (structural findings), got {below:?}",
        );
        assert!(
            structural
                .iter()
                .all(|f| f.route.as_deref().is_some_and(|r| r.starts_with("migrate"))),
            "every parse-failure finding of a below-version doc routes migrate, got {structural:?}",
        );

        // Guard: an ADR stamped **at** the current version that still fails to parse is
        // genuine corruption — its parse-failure findings stay UN-routed (not migrate).
        let versions_v1: BTreeMap<String, u32> = [("adr".to_string(), 1u32)].into_iter().collect();
        let repo = TempRoot::new("at-version-corrupt");
        // Stamp this pre-options body at v1 == current, so it is at-version yet parse-failing.
        let at_body = ADR_STAMP_0_NO_OPTIONS.replace("schema-version: 0", "schema-version: 1");
        repo.commit("decisions", "doc", &at_body);
        let mut record = FileStateRecord::new();
        record.record("decisions/doc.md", hash_bytes(at_body.as_bytes()));
        let seen = RefCell::new(Vec::new());
        let at = validate_store_families(
            repo.path(),
            &schemas,
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
            &versions_v1,
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs")
        .findings;
        let at_structural: Vec<&Finding> = at
            .iter()
            .filter(|f| f.code.starts_with("conformance."))
            .collect();
        // An at-version parse failure is genuine corruption — a purely-positional parser
        // diagnostic under the route floor's one-home EXEMPTION (M43: the located message
        // *is* the repair; `design/surface-contract.md` → The route fence), so its
        // route-lessness is a declared exemption, not an un-swept hole.
        assert!(
            !at_structural.is_empty()
                && at_structural
                    .iter()
                    .all(|f| f.route.is_none() && crate::finding::is_route_exempt(&f.code)),
            "an at-version parse failure is corruption — route-exempt parser diagnostics, \
             got {at_structural:?}",
        );
    }

    /// (M34 Inc-3; the code minted at M42 Inc-3 T2) **Version-currency is itself a surfaced
    /// break** — *emitted*, not only routed. A committed persisted doc of a *versioned* doctype
    /// whose schema-version stamp is **absent** (the v0 corpus state) or **below** the manifest
    /// `current` is non-conformant on its stamp **even when otherwise structurally clean**, so
    /// the fifth family EMITS one finding for it — the pure-stamp v0 dogfood a labeler-only path
    /// leaves silent (`design/validation.md` → Version-currency is itself a surfaced break;
    /// DECISIONS 2026-06-25). A doc stamped **at** `current` stays clean. The break carries
    /// **its own check id** — [`SCHEMA_VERSION_CURRENT_CODE`], no longer the reused
    /// `field-value-conformant` (M42: the fact must be distinguishable to a machine consumer) —
    /// and routes `migrate`, naming `jigc migrate-corpus`. Store-scope, report-only.
    #[test]
    fn store_sweep_emits_version_mismatch_break_for_below_or_absent_stamp() {
        let schemas = stamped_schemas();
        let versions: BTreeMap<String, u32> = [("adr".to_string(), 1u32)].into_iter().collect();

        // Run the store sweep over a single committed, baselined ADR `body`, returning the
        // version-currency findings it surfaces over the schema-version stamp.
        let version_findings = |tag: &str, body: &str| -> Vec<Finding> {
            let repo = TempRoot::new(tag);
            repo.commit("decisions", "doc", body);
            // Baseline the doc so the hash-only file↔CLI-state family stays silent — the
            // version break is the only schema-conformance signal under test.
            let mut record = FileStateRecord::new();
            record.record("decisions/doc.md", hash_bytes(body.as_bytes()));
            let seen = RefCell::new(Vec::new());
            let report = validate_store_families(
                repo.path(),
                &schemas,
                &no_delta_resolved(),
                &dangling_aware_invoker(&seen),
                &[],
                &EmptyStepSource,
                &record,
                &versions,
                &BTreeMap::new(),
                &[],
            )
            .expect("store sweep runs");
            report
                .findings
                .into_iter()
                .filter(|f| f.code == SCHEMA_VERSION_CURRENT_CODE)
                .collect()
        };

        // (i) Stamp absent (the v0 corpus state), otherwise conformant ⇒ exactly one version
        // break routed `migrate` — the headline pure-stamp case a labeler leaves silent.
        let absent = version_findings("absent", ADR_VALID);
        assert_eq!(
            absent.len(),
            1,
            "an unstamped v0 doc must surface exactly one version break, got {absent:?}",
        );
        assert!(
            absent[0]
                .route
                .as_deref()
                .is_some_and(|r| r.starts_with("migrate") && r.contains("jigc migrate-corpus")),
            "the stamp-absent version break must route migrate at the corpus migration, got {:?}",
            absent[0].route,
        );

        // (ii) Stamp below current (0 < 1), otherwise conformant ⇒ one version break, migrate.
        let below = version_findings("below", ADR_STAMP_0);
        assert_eq!(
            below.len(),
            1,
            "a below-version doc must surface exactly one version break, got {below:?}",
        );
        assert!(
            below[0]
                .route
                .as_deref()
                .is_some_and(|r| r.starts_with("migrate")),
            "the below-version break must route migrate, got {:?}",
            below[0].route,
        );

        // (iii) Stamp at current (1) ⇒ no version break (the false-positive guard).
        let at = version_findings("at", ADR_STAMP_1);
        assert!(
            at.is_empty(),
            "an at-version doc must surface NO version break, got {at:?}",
        );
    }

    /// (2026-07-24 — the confidence-audit sibling-hunt item 1) **An above-current stamp is a
    /// surfaced break of its own** — [`SCHEMA_VERSION_AHEAD_CODE`], never the below-version
    /// code and never silence. Before this arm an OOB-planted `schema-version: 99` produced
    /// **no finding at all** (`stamp.is_none_or(|s| s < current)` has no `s > current` case) —
    /// the fixed failure through an unfixed door. Both detector arms are pinned: the
    /// parse-success arm (an otherwise-conformant doc) and the parse-failure arm (a doc
    /// written to a future schema this binary cannot parse). The route is Human-shaped, `ahead`
    /// leading — no verb fixes a future stamp.
    #[test]
    fn store_sweep_emits_version_ahead_break_for_above_current_stamp() {
        let schemas = stamped_schemas();
        let versions: BTreeMap<String, u32> = [("adr".to_string(), 1u32)].into_iter().collect();

        let findings_of = |tag: &str, body: &str| -> Vec<Finding> {
            let repo = TempRoot::new(tag);
            repo.commit("decisions", "doc", body);
            let mut record = FileStateRecord::new();
            record.record("decisions/doc.md", hash_bytes(body.as_bytes()));
            let seen = RefCell::new(Vec::new());
            validate_store_families(
                repo.path(),
                &schemas,
                &no_delta_resolved(),
                &dangling_aware_invoker(&seen),
                &[],
                &EmptyStepSource,
                &record,
                &versions,
                &BTreeMap::new(),
                &[],
            )
            .expect("store sweep runs")
            .findings
            .into_vec()
        };

        // (i) The parse-success arm: an otherwise-conformant ADR stamped 7 under manifest
        // version 1 ⇒ exactly one AHEAD break, routed `ahead` at the human repairs — and
        // no below-version break (the two facts stay distinguishable by code).
        let above_body = ADR_STAMP_1.replace("schema-version: 1", "schema-version: 7");
        let above = findings_of("above", &above_body);
        let ahead: Vec<&Finding> = above
            .iter()
            .filter(|f| f.code == SCHEMA_VERSION_AHEAD_CODE)
            .collect();
        assert_eq!(
            ahead.len(),
            1,
            "an above-current doc must surface exactly one ahead break, got {above:?}",
        );
        assert!(
            ahead[0]
                .route
                .as_deref()
                .is_some_and(|r| r.starts_with("ahead")
                    && r.contains("upgrade jigc")
                    && r.contains("git history")),
            "the ahead break routes `ahead` at the human repairs, got {:?}",
            ahead[0].route,
        );
        assert!(
            !above.iter().any(|f| f.code == SCHEMA_VERSION_CURRENT_CODE),
            "a future stamp must never surface as the below-version break, got {above:?}",
        );

        // (ii) The parse-failure arm: a doc written to a future schema shape (sections this
        // binary's schema does not match) still surfaces the ahead break — the stamp is read
        // from the raw front matter.
        let broken_above = ADR_STAMP_0_NO_OPTIONS.replace("schema-version: 0", "schema-version: 7");
        let broken = findings_of("broken-above", &broken_above);
        assert_eq!(
            broken
                .iter()
                .filter(|f| f.code == SCHEMA_VERSION_AHEAD_CODE)
                .count(),
            1,
            "an above-current doc that fails to parse must still surface the ahead break, \
             got {broken:?}",
        );
    }

    /// (2026-07-24 mutation audit, findings #12+13) The labeler's **ahead arm** on the
    /// cell only a future-stamped *and* non-conformant doc reaches: a doc stamped above
    /// current that parses but breaks conformance (an empty required slot) must carry the
    /// `ahead — …` label on its **accompanying** `schema-conformance.*` findings — never
    /// `corrupt — at the current schema-version {current}`, the documented lie (the doc is
    /// at a *future* version, and no hand-review of "corruption" repairs a version skew).
    /// The break's own route was already pinned; this pins the labeler's arm
    /// (`route_schema_conformance`, `Some(s) if s > current`), which is observable only
    /// on this cell — a clean future-stamped doc has no other findings to label.
    #[test]
    fn store_sweep_labels_a_broken_above_current_docs_findings_ahead_not_corrupt() {
        let schemas = stamped_schemas();
        let versions: BTreeMap<String, u32> = [("adr".to_string(), 1u32)].into_iter().collect();

        // Future-stamped (7 > 1) AND non-conformant: parse succeeds (structure intact),
        // but the emptied `## Consequences` slot breaks `required-slot-present`.
        let body = ADR_STAMP_1
            .replace("schema-version: 1", "schema-version: 7")
            .replace("## Consequences\nEffects.\n", "## Consequences\n");

        let repo = TempRoot::new("ahead-label");
        repo.commit("decisions", "doc", &body);
        let mut record = FileStateRecord::new();
        record.record("decisions/doc.md", hash_bytes(body.as_bytes()));
        let seen = RefCell::new(Vec::new());
        let findings = validate_store_families(
            repo.path(),
            &schemas,
            &no_delta_resolved(),
            &dangling_aware_invoker(&seen),
            &[],
            &EmptyStepSource,
            &record,
            &versions,
            &BTreeMap::new(),
            &[],
        )
        .expect("store sweep runs")
        .findings
        .into_vec();

        // The accompanying conformance findings exist (the cell is non-degenerate) …
        let accompanying: Vec<&Finding> = findings
            .iter()
            .filter(|f| {
                f.code.starts_with("schema-conformance.") && f.code != SCHEMA_VERSION_AHEAD_CODE
            })
            .collect();
        assert!(
            !accompanying.is_empty(),
            "the emptied slot must surface an accompanying conformance finding, \
             got {findings:?}",
        );
        // … and every one carries the ahead label, never the at-current corrupt lie.
        for f in &accompanying {
            let route = f.route.as_deref().unwrap_or("");
            assert!(
                route.starts_with("ahead"),
                "an accompanying finding of a future-stamped doc is labeled `ahead — …`, \
                 got {f:?}",
            );
            assert!(
                !route.starts_with("corrupt"),
                "the `corrupt — at the current schema-version` label is the documented \
                 lie for a future-stamped doc, got {f:?}",
            );
        }
        // The ahead break itself still rides alongside (one fact per code).
        assert_eq!(
            findings
                .iter()
                .filter(|f| f.code == SCHEMA_VERSION_AHEAD_CODE)
                .count(),
            1,
            "the ahead break accompanies the labeled findings, got {findings:?}",
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

## Options
Alternatives were weighed and rejected.

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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
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
             # Cited decision\n\n## Context\nForces.\n\n## Options\nAlternatives were weighed and rejected.\n\n## Decision\nDecided.\n\n\
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
                            // Routed, as the real probe routes it (the route floor).
                            Some("update the citation, or restore the cited symbol".into()),
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
                &|_| true,
                changed,
                base,
                &test_conflict(),
                &AdoptionInputs::inert(),
                &crate::file_state::LiveRecord::none(),
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
            &|_| true,
            &change_set(&["src/foo.rs"]),
            base.path(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
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
                        // Routed, as the real probe routes it (the route floor).
                        Some("update the citation, or restore the cited symbol".into()),
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
            &|_| true,
            &change_set(&["src/foo.rs"]),
            base.path(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
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
                        // Routed, as the real probe routes it (the route floor).
                        Some("update the citation, or restore the cited symbol".into()),
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
            &|_| true,
            &change_set(&["src/foo.rs"]),
            base.path(),
            &test_conflict(),
            &AdoptionInputs::inert(),
            &crate::file_state::LiveRecord::none(),
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

    /// One non-conformant doc's finding, for the routing unit (a `required-field-present`
    /// break, route initially `None`).
    fn one_break() -> Vec<Finding> {
        vec![Finding::graded(
            Severity::Blocking,
            "schema-conformance.required-field-present",
            "required field `owner` is missing",
            Some(Location::addressed("decisions/x.md", 1, 1)),
            None,
        )]
    }

    /// (T3, the done-criterion) Version-aware migrate-vs-corrupt routing: a stamp **below**
    /// the manifest version OR **absent** routes `migrate`; a stamp **at** the version routes
    /// `corrupt`. The route is the existing finding's `route` field — no new id.
    #[test]
    fn route_classifies_below_absent_migrate_and_at_corrupt() {
        // stamp absent (the v0 corpus state) ⇒ migrate.
        let mut f = one_break();
        route_schema_conformance(&mut f, None, Some(1), "decisions/x.md");
        assert!(
            f[0].route.as_deref().unwrap().starts_with("migrate"),
            "stamp-absent must route migrate, got {:?}",
            f[0].route,
        );

        // stamp below the current version ⇒ migrate.
        let mut f = one_break();
        route_schema_conformance(&mut f, Some(0), Some(1), "decisions/x.md");
        assert!(
            f[0].route.as_deref().unwrap().starts_with("migrate"),
            "below-version must route migrate, got {:?}",
            f[0].route,
        );

        // stamp at the current version ⇒ corrupt.
        let mut f = one_break();
        route_schema_conformance(&mut f, Some(1), Some(1), "decisions/x.md");
        assert!(
            f[0].route.as_deref().unwrap().starts_with("corrupt"),
            "at-version must route corrupt, got {:?}",
            f[0].route,
        );
    }

    /// (T3, the omitting-context inert path) A doctype the caller supplies **no** manifest
    /// version for (`current = None`) has no migrate-vs-corrupt distinction, so its finding
    /// stays **un-routed** — reported, never mislabeled, never an error. This is the scope
    /// guard: version-aware routing fires only over the versioned/frozen set, and an empty
    /// `versions` map (a non-freeze pack) leaves every finding **un-routed**. Inert is a
    /// claim about the *route*, not about the report: since M51 Inc 8 / T5 the sweep says
    /// out loud that it is not version-checking such a doctype ([`unversioned_doctype`]).
    #[test]
    fn route_is_inert_when_no_manifest_version() {
        let mut f = one_break();
        route_schema_conformance(&mut f, None, None, "decisions/x.md");
        assert_eq!(
            f[0].route, None,
            "an unversioned doctype must leave the finding un-routed",
        );

        // And a present stamp with no manifest version is equally inert (no false corrupt).
        let mut f = one_break();
        route_schema_conformance(&mut f, Some(7), None, "decisions/x.md");
        assert_eq!(
            f[0].route, None,
            "a stamped doc whose doctype has no manifest version must stay un-routed",
        );
    }
}

#[cfg(test)]
mod attribution_tests {
    //! File-attribution (M36): every `schema-conformance` / `conformance.*` finding names
    //! the doc it came from — at both the task-scope [`conformance_for`] and the store-scope
    //! [`schema_conformance_store`] loci. The bare per-instance checks emit doc-less messages
    //! and fragment-only / absent addresses, so a multi-doc sweep would otherwise produce
    //! findings indistinguishable across sibling docs. The `line`/`col` the check raised is
    //! preserved verbatim; the `message` prefix gains the `rel_key` filesystem path, while
    //! the `Location.address` gains the doc's **URI identity** (`<type>:<slug>`, the path→URI
    //! flip — `command-output-contract.md` → the stable finding key), not the path.

    use super::*;
    use crate::parse::parse_sections;

    /// A minimal persisted `note` doctype with one required body slot — the one lever a
    /// fixture violates (an empty slot ⇒ `schema-conformance.required-slot-present`). Carries
    /// a `location:` so the store-scope sweep walks it.
    fn note_schema() -> Schema {
        let yaml = b"\
type: note
location: notes/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: body
    slot: { hint: \"The note body.\" }
";
        crate::schema::load_schema(yaml).expect("note schema loads")
    }

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert("note".to_string(), note_schema());
        m
    }

    /// A `note` instance whose required body slot is **empty** (heading present, no prose) —
    /// the unfilled-slot case. `pad` shifts the slot's source line so the two sibling
    /// fixtures raise findings at *different* coordinates, making line-preservation observable.
    fn empty_slot_note(title: &str, pad: &str) -> String {
        format!(
            "\
---
title: {title}
---
{pad}
# {title}

## Body
"
        )
    }

    /// The single bare (un-attributed) finding the checks raise over one empty-slot note —
    /// the baseline the attributed finding must match on `line`/`col` and (prefix-stripped)
    /// `message`.
    fn bare_finding(source: &str) -> Finding {
        let schema = note_schema();
        let doc = parse_sections(&schema, source).expect("fixture parses");
        let mut findings = schema_conformance(&schema, source, &doc);
        assert_eq!(
            findings.len(),
            1,
            "the empty-slot fixture must raise exactly one bare finding: {findings:?}"
        );
        findings.pop().unwrap()
    }

    /// (task scope) Two docs, each an empty-slot note, run through [`conformance_for`]: each
    /// finding's message AND `Location.address` name **its own** `rel_key` (never the
    /// sibling's), with `line`/`col` byte-identical to the bare baseline.

    #[test]
    fn conformance_for_attributes_each_finding_to_its_own_doc() {
        let schemas = schemas();
        for (slug, pad) in [("alpha", ""), ("beta", "\n")] {
            let source = empty_slot_note(slug, pad);
            let filename = format!("note:{slug}.md");
            let rel_key = format!("docs/note:{slug}.md");
            let identity = format!("note:{slug}");
            let sibling = if slug == "alpha" { "beta" } else { "alpha" };

            let bare = bare_finding(&source);
            let bare_loc = bare.location.clone().expect("bare finding is located");

            let findings = conformance_for(&filename, &schemas, &rel_key, &source);
            assert_eq!(findings.len(), 1, "one finding for {slug}: {findings:?}");
            let f = &findings[0];

            assert_eq!(
                f.message,
                format!("`{rel_key}`: {}", bare.message),
                "the message is prefixed with the owning doc",
            );
            assert!(
                !f.message.contains(sibling),
                "{slug}'s message must not name the sibling `{sibling}`: {}",
                f.message,
            );
            let loc = f.location.as_ref().expect("attributed finding is located");
            assert_eq!(
                loc.address.as_deref(),
                Some(format!("{identity}#body").as_str()),
                "the address names the owning doc's URI identity, fragmented at the empty \
                 slot's section (M42 Inc 9 — a doc's several empty slots must not collapse \
                 onto one key)",
            );
            assert!(
                !loc.address.as_deref().unwrap().contains(sibling),
                "{slug}'s address must not name the sibling `{sibling}`",
            );
            assert_eq!(
                (loc.line, loc.col),
                (bare_loc.line, bare_loc.col),
                "line/col are preserved verbatim (attribution never moves the coordinate)",
            );
        }
    }

    /// (store scope, the done-criterion) A committed store of **two** empty-slot notes: the
    /// store sweep surfaces one break per doc, each attributed to its own `notes/<slug>.md`
    /// — never the sibling's — with `line`/`col` unchanged. This is where the adoption
    /// ingest→validate path names every finding's doc.
    #[test]
    fn schema_conformance_store_attributes_each_finding_to_its_own_doc() {
        let root = std::env::temp_dir().join(format!(
            "jigc-attribution-{}-{:?}",
            std::process::id(),
            crate::tempname::unique_nanos(),
        ));
        let notes = root.join("notes");
        std::fs::create_dir_all(&notes).expect("mk notes/");
        let alpha = empty_slot_note("alpha", "");
        let beta = empty_slot_note("beta", "\n");
        std::fs::write(notes.join("alpha.md"), &alpha).expect("commit alpha");
        std::fs::write(notes.join("beta.md"), &beta).expect("commit beta");

        let findings = schema_conformance_store(
            &root,
            &schemas(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
            &[],
        );
        let _ = std::fs::remove_dir_all(&root);

        let breaks: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.code == "schema-conformance.required-slot-present")
            .collect();
        assert_eq!(
            breaks.len(),
            2,
            "one required-slot-present break per committed note: {findings:?}",
        );

        for (slug, source) in [("alpha", &alpha), ("beta", &beta)] {
            let rel_key = format!("notes/{slug}.md");
            let identity = format!("note:{slug}");
            let sibling_key = if slug == "alpha" { "beta" } else { "alpha" };
            let bare_loc = bare_finding(source).location.expect("bare located");

            let f = breaks
                .iter()
                .find(|f| f.message.contains(&rel_key))
                .unwrap_or_else(|| panic!("no finding attributed to `{rel_key}`: {breaks:?}"));

            assert!(
                !f.message.contains(sibling_key),
                "{slug}'s message must not name the sibling `{sibling_key}`: {}",
                f.message,
            );
            let loc = f.location.as_ref().expect("attributed finding is located");
            assert_eq!(
                loc.address.as_deref(),
                Some(format!("{identity}#body").as_str()),
                "the address names the owning doc's URI identity, fragmented at the empty \
                 slot's section (M42 Inc 9)",
            );
            assert_eq!(
                (loc.line, loc.col),
                (bare_loc.line, bare_loc.col),
                "line/col preserved verbatim for {slug}",
            );
        }
    }

    /// A `note` whose front matter carries **two undeclared keys** and whose required
    /// `## Body` section is **absent** — three parse-`conformance.*` findings over one doc,
    /// the exact input the family's degenerate key collapsed.
    const NOTE_TWO_UNKNOWN_FIELDS: &str = "\
---
title: Cache it
alpha: 1
beta: 2
---

# Cache it
";

    /// (M42 Inc 9 T5 — the collision, at the key) **Two `conformance.unknown-field`
    /// findings in one doc carry DISTINCT stable keys.** Before the fragment work the
    /// family set no address below the doc, so [`attribute_to_doc`] addressed both at the
    /// bare `note:cache-it` and a driver deserializing the findings array saw
    /// `(conformance.unknown-field, note:cache-it)` **twice, byte-identical** — it could
    /// neither dedupe them nor tell them apart (`command-output-contract.md` → the
    /// parse-conformance sub-table: `unknown-field` → `#<section>/<field-key>`).
    ///
    /// Asserted on the **emitted** key ([`Finding::key`] — the `(code, target)` the envelope
    /// projects), not on a reconstructed address.
    #[test]
    fn two_unknown_fields_in_one_doc_carry_distinct_keys() {
        let findings = conformance_for(
            "note:cache-it.md",
            &schemas(),
            "notes/cache-it.md",
            NOTE_TWO_UNKNOWN_FIELDS,
        );
        let keys: Vec<crate::finding::FindingKey> = findings
            .iter()
            .filter(|f| f.code == "conformance.unknown-field")
            .map(|f| f.key())
            .collect();
        let targets: Vec<Option<&str>> = keys.iter().map(|k| k.target.as_deref()).collect();
        assert_eq!(
            targets,
            [
                Some("note:cache-it#meta/alpha"),
                Some("note:cache-it#meta/beta")
            ],
            "each unknown key keys at its own `<type>:<slug>#<section>/<field-key>`: \
             {findings:#?}",
        );
        assert_ne!(keys[0], keys[1], "and the two keys are therefore distinct");
    }

    /// (M42 Inc 9 T5 — HAZARD 2, the gate regression) The fragment work must **keep the
    /// `<type>:<slug>` identity hop** at the head of every address. The CLI's store trailer
    /// (`crates/cli/src/render.rs` → `gates_at_task`) splits a per-doc conformance finding's
    /// address on `#` and joins the **identity half** against the un-baselined committed
    /// docs, to decide whether a gate exists for it: an un-baselined doc's conformance break
    /// is graded *advisory* by the reconciler at task scope, so the trailer must not claim a
    /// gate. Adding fragments is safe (the join splits them off); **dropping the identity
    /// prefix would silently break the join** — every un-baselined doc's break would start
    /// claiming a gate that never fires.
    ///
    /// So: every parse-`conformance.*` finding the emit path produces addresses its doc,
    /// identity-first — fragment or no fragment.
    #[test]
    fn every_conformance_finding_keeps_the_doc_identity_at_the_head_of_its_address() {
        let findings = conformance_for(
            "note:cache-it.md",
            &schemas(),
            "notes/cache-it.md",
            NOTE_TWO_UNKNOWN_FIELDS,
        );
        assert!(
            findings.len() >= 3,
            "two unknown keys + the missing `## Body` section: {findings:#?}",
        );
        for f in &findings {
            let target = f
                .key()
                .target
                .unwrap_or_else(|| panic!("every parse-conformance finding is keyed: {f:#?}"));
            let identity = target.split('#').next().expect("split yields the head");
            assert_eq!(
                identity, "note:cache-it",
                "the un-baselined join is on the identity half of the address — dropping the \
                 `<type>:<slug>` hop would silently break the gate claim: {f:#?}",
            );
        }
    }
}
