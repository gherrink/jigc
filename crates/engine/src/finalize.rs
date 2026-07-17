//! The `finalize` transaction **planner** — the engine half of the commit
//! boundary.
//!
//! `finalize ≡ validate(task) + commit` (`design/finalize.md` → What `finalize`
//! is). The transaction splits across the engine/CLI boundary by the determinism
//! invariant: the **engine** is a pure planner — given the task working area, its
//! base pin, the supplied HEAD, the already-computed `validate` report, and a
//! diff-presence signal, it runs the cheap rejections and produces the ordered
//! phase plan (the rendered commit message + the post-commit hash-update set). The
//! **CLI** executes the git steps (`add`, `commit -F`) the plan implies. The engine
//! performs **no git and no commit** and never shells out (`DECISIONS.md`
//! 2026-05-31 → inc-4 planning pins: "the engine `finalize` is a pure transaction
//! planner … keeping the engine LLM-free *and* shell-free").
//!
//! The planner covers the phases that are pure decisions over its inputs:
//!
//! - **Phase 1 — preflight** (`finalize.md` → 1. Preflight): the task working area
//!   must exist; the base pin must equal the supplied HEAD, else a divergence-routing
//!   blocking [`Finding`] (no plan) — the CLI never operates a task off its pinned
//!   base. (The "git committable" sub-check is a git-state query the CLI owns; it is
//!   not a pure decision, so it is not the planner's concern.)
//! - **Phase 2 — validate** (`finalize.md` → 2. Validate): if the supplied report
//!   holds any blocking finding, abort with exactly those findings. `finalize` has no
//!   private check path — what `validate` reports is what `finalize` blocks on.
//! - **Empty-commit guard** (`finalize.md` → Commit-doc rendering → Empty commit):
//!   "if validate passes but the staged diff … is empty, `finalize` aborts" — so the
//!   guard sits *after* validate. The CLI supplies `has_diff` (a `git diff` query it
//!   owns); the planner makes the abort decision.
//! - **Phase 3 — render** (`finalize.md` → 3. Render the commit message): the staged
//!   `commit:<id>` instance is rendered into the git-message string
//!   ([`crate::write::render_commit_message`]). No disk side effects.
//! - **Phase 4 — promote** (`finalize.md` → 4. Promote managed docs): for every staged
//!   managed-doc instance under `<task_dir>/docs/*.md`, the plan names a [`Promotion`]
//!   — the staged source path plus its canonical repo-relative destination
//!   (`<location>/<slug>.md`, the path from the doc type's `location:`). The **commit**
//!   doc (and any other transient, location-less type) is **excluded**: its sink is the
//!   git message rendered in phase 3, not a repo file. Promotion is **copy, not move**
//!   (the CLI copies); the plan only decides *what* lands *where*.
//! - **Phase 7 — post-commit** (`finalize.md` → 7. Post-commit): the plan carries the
//!   **hash-update set** — `canonical-path → blake3 hash` for every *promoted managed
//!   doc* the CLI records after the commit lands. The hash is over the staged bytes
//!   (byte-stable: the promoted copy equals the staged source), so `file-state` sees the
//!   committed doc in-sync on the next probe. The edge-index stamp invalidation +
//!   working-area removal are CLI-side side effects of the committed plan.
//!
//! The planner returns a [`FinalizePlan`] the CLI executes, or a `Vec<Finding>` it
//! routes to the agent. It performs no git and no commit; rollback discipline
//! (`finalize.md` → Rollback discipline) is the CLI's concern, but the planner's
//! aborts are deliberately ordered so that **no plan is produced** on any blocking
//! branch — there is nothing to roll back before phase 3.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::file_state::hash_bytes;
use crate::finding::{Finding, Location, Route, Severity};
use crate::result::{SCHEMA_VERSION, ValidationReport};
use crate::schema::Schema;
use crate::state::{BasePin, StagedSnapshot};
use crate::write;

/// The working-area sub-directory holding the task's staged doc instances
/// (`DECISIONS.md` 2026-05-31 → Task working-area on-disk layout).
const DOCS_DIR: &str = "docs";

/// One staged managed doc to promote (`finalize.md` → 4. Promote managed docs): copy
/// the [`source`] (the staged instance under `<task_dir>/docs/`) to the [`destination`]
/// (its canonical repo-relative path `<location>/<slug>.md`). **Copy, not move** — the
/// CLI performs the copy; the working area stays intact until phase 7.
///
/// [`source`]: Promotion::source
/// [`destination`]: Promotion::destination
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Promotion {
    /// The staged instance's on-disk path (`<task_dir>/docs/<type>:<slug>.md`).
    pub source: PathBuf,
    /// The canonical repo-relative destination (`<location>/<slug>.md`) — the key the
    /// CLI joins onto the repo root to write the copy, and the [`FinalizePlan::hash_updates`]
    /// key.
    pub destination: String,
}

/// The ordered `finalize` plan the engine hands the CLI to execute — the result of
/// a clean preflight + validate + empty-diff guard + render + promote.
///
/// It carries the rendered commit-message string (phase 3), the set of staged docs to
/// promote (phase 4: [`promotions`]), and the post-commit hash-update set (phase 7:
/// `committed-doc-path → blake3 hash`). The CLI is the only side that touches git: it
/// copies each promotion to its canonical path, stages, runs `git commit -F` over
/// [`message`], and — after the commit lands — records the [`hash_updates`] into the
/// `file-state` record.
///
/// [`message`]: FinalizePlan::message
/// [`promotions`]: FinalizePlan::promotions
/// [`hash_updates`]: FinalizePlan::hash_updates
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizePlan {
    /// The result-contract schema version (see [`SCHEMA_VERSION`]).
    pub schema_version: u32,
    /// The rendered git-commit message (phase 3) — no trailing newline; the CLI
    /// commits it via `git commit -F`.
    pub message: String,
    /// The staged managed docs to promote (phase 4), in canonical-destination order.
    /// The commit doc (and any transient, location-less type) is excluded. Empty when
    /// the task staged no persisted managed doc.
    pub promotions: Vec<Promotion>,
    /// The post-commit hash-update set (phase 7): `committed-doc-path → blake3 hex`
    /// for every promoted managed doc (hashed over the staged bytes — the copy is
    /// byte-stable). Empty when there is nothing to promote.
    pub hash_updates: BTreeMap<String, String>,
    /// The **retire set** (`design/auto-migration.md` → Retire-the-foreign-original):
    /// the repo-relative foreign original(s) a migration task replaces, removed
    /// **inside** the commit closure (before `git add --all`) so the deletion stages
    /// into the same commit as the promoted doc — the first byte-destructive write,
    /// transactional with promote + commit. Populated from the migration task's recorded
    /// `source-path` ([`crate::state::read_source_path`]); **empty** on every
    /// non-migration task (the milestone sibling never sets it).
    pub retirements: Vec<PathBuf>,
}

impl FinalizePlan {
    /// Build a plan over a rendered message, a promote set, a post-commit hash-update
    /// set, and a retire set, stamping the current [`SCHEMA_VERSION`].
    fn new(
        message: String,
        promotions: Vec<Promotion>,
        hash_updates: BTreeMap<String, String>,
        retirements: Vec<PathBuf>,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            message,
            promotions,
            hash_updates,
            retirements,
        }
    }
}

/// Plan the `finalize` transaction for a task — the pure engine-side decision over
/// the working area + base + supplied HEAD + validate report + diff signal.
///
/// Phases run in order, each aborting *before* any plan is produced:
///
/// 1. **Preflight** — `task_dir` must exist; `base.sha` must equal `head_sha`, else
///    a `finalize.base-mismatch` divergence-routing blocking [`Finding`].
/// 2. **Validate** — if `report` holds any blocking finding, abort with exactly
///    those findings (one engine, two entry points; no private check path).
/// 3. **Empty-commit guard** — if `has_diff` is false, abort with
///    `finalize.empty-commit` ("produced no diff").
/// 4. **Render** — read + parse the staged `commit:<commit_slug>` instance and render
///    it into the message ([`crate::write::render_commit_message`]).
///
/// `commit_schema` is the cascade-resolved `commit` doc schema (the engine stays
/// domain-empty — the CLI feeds it in); `commit_slug` is the task-derived id the
/// commit instance was provisioned under (`commit:<id>`). `schemas` is the full
/// cascade-resolved schema set (keyed by type), used in phase 4 to resolve each staged
/// doc's `location:`. `repo_root` is the repo root the planner probes for the promote
/// **clobber guard** (review S1) — a create-provenance doc whose canonical destination
/// already holds a committed managed doc blocks rather than silently overwriting it.
/// Returns the [`FinalizePlan`] on a clean run, or the blocking findings that aborted it.
///
/// Performs no git and no commit. Reads the task working area + stats `repo_root`
/// promote destinations for the clobber guard (the engine's existing filesystem effect);
/// never shells out.
// The planner is a pure decision over a deliberately explicit set of inputs (the
// determinism contract feeds every layer in rather than re-deriving it); bundling
// them into a params struct would be churn without clarifying the contract.
#[allow(clippy::too_many_arguments)]
pub fn plan_finalize(
    task_dir: &Path,
    repo_root: &Path,
    base: &BasePin,
    head_sha: &str,
    report: &ValidationReport,
    has_diff: bool,
    commit_schema: &Schema,
    commit_slug: &str,
    schemas: &BTreeMap<String, Schema>,
) -> Result<FinalizePlan, Vec<Finding>> {
    // The work unit every block whose subject is the *task* keys at (M42 inc-9 T4). The
    // commit slug IS the task id — a task's commit doc is provisioned as `commit:<task-id>`
    // — so the ref is in hand, not derived from the working-area path.
    let unit = Unit::Task(commit_slug);

    // Phase 1 — preflight: task exists, base pin == supplied HEAD.
    if !task_dir.exists() {
        return Err(vec![task_missing_finding(unit, task_dir)]);
    }
    if base.sha != head_sha {
        return Err(vec![base_mismatch_finding(unit, base, head_sha)]);
    }

    // Phase 2 — validate: abort on any blocking finding, surfacing exactly those.
    if report.has_blocking() {
        return Err(report
            .findings
            .iter()
            .filter(|f| f.severity == Severity::Blocking)
            .cloned()
            .collect());
    }

    // Empty-commit guard (after validate per finalize.md → Empty commit).
    if !has_diff {
        return Err(vec![empty_commit_finding(unit)]);
    }

    // Phase 3 — render the staged commit doc into the git-message string.
    let commit_path = task_dir
        .join(DOCS_DIR)
        .join(format!("{}:{commit_slug}.md", commit_schema.ty));
    let source = std::fs::read_to_string(&commit_path)
        .map_err(|err| vec![render_io_finding(&commit_path, &err)])?;
    let instance = write::instance_from_source(commit_schema, &source)?;
    let message = write::render_commit_message(commit_schema, &instance);

    // Phase 4 — promote: every staged managed doc with a `location:` lands at its
    // canonical `<location>/<slug>.md`. The commit doc (and any transient,
    // location-less type) is excluded. Phase 7's hash-update set carries each promoted
    // doc's blake3 over its staged bytes (byte-stable — the copy equals the source).
    let promote = plan_promotions(task_dir, schemas)?;

    // The clobber guard (review S1, `design/auto-migration.md` → Honest bounds, the
    // data-loss clobber guard): a **create-provenance** staged doc whose canonical
    // destination already holds a committed managed doc would silently overwrite it at
    // promote — irreversible data loss. Block before retire. The in-place migration
    // rewrite (the doc replacing the very foreign original at its own canonical path) is
    // excluded via the retire guard's source-path == destination discriminator.
    plan_clobber_guard(unit, task_dir, repo_root, &promote.promotions)?;

    // The retire set (`design/auto-migration.md` → Retire-the-foreign-original): a
    // migration task records its repo-relative foreign source path at mint; the planner
    // names it for retirement inside the commit closure. Empty on every non-migration
    // task (no `source-path`) — inert. The path-collision guard (→ Path-collision guard)
    // excludes a foreign source that equals a promote destination — the in-location
    // squatter the managed write rewrites in place is not a distinct original to retire.
    let retirements = plan_retirements(unit, task_dir, &promote.promotions)?;

    Ok(FinalizePlan::new(
        message,
        promote.promotions,
        promote.hash_updates,
        retirements,
    ))
}

/// Read the migration task's recorded foreign source path
/// ([`crate::state::read_source_path`]) into the retire set. A non-migration task has
/// no `source-path` → an empty set (inert). An I/O failure reading it is a blocking
/// [`Finding`] (the file is written at mint, so a read fault is a real fault — the
/// promote/render I/O precedent).
///
/// The **replacement precondition** (review F1, `design/auto-migration.md` →
/// Retire-the-foreign-original): the retire is byte-destructive, so it fires **only when
/// the migration actually produced its canonical replacement** — i.e. the promote set
/// names a persisted managed doc. A migration always stages a fillable `commit:<id>` doc
/// (transient, never promoted), so an empty promote set means the agent authored **no**
/// managed doc to replace the foreign original; deleting it then would be irreversible
/// data loss with no replacement. The planner **blocks** that case
/// (`finalize.migration-no-replacement`) rather than silently committing an empty
/// migration that wipes the foreign file.
///
/// The **path-collision guard** (`design/auto-migration.md` → Path-collision guard, the
/// in-location squatter): when the recorded foreign source path **equals** a promote
/// destination (the canonical managed path `<location>/<slug>.md`), the managed write
/// *is* the in-place rewrite — the foreign file is not a distinct original, so the
/// retire is **skipped** (else the plan would delete the doc it just wrote). Both sides
/// are [`crate::store::lexical_normalize`]d so the guard holds regardless of the recorded path's
/// spelling (review F2: a `./`-prefixed or redundant-component spelling of the canonical
/// path must still be recognized as the squatter). The common root-`CHANGELOG.md` ≠
/// `changelog/changelog.md` case retires the distinct original.
fn plan_retirements(
    unit: Unit,
    task_dir: &Path,
    promotions: &[Promotion],
) -> Result<Vec<PathBuf>, Vec<Finding>> {
    match crate::state::read_source_path(task_dir) {
        Ok(Some(path)) if !path.trim().is_empty() => {
            // F1: no managed replacement was promoted — refuse to retire (block), never
            // delete the foreign original with nothing to take its place.
            if promotions.is_empty() {
                return Err(vec![migration_no_replacement_finding(path.trim())]);
            }
            let foreign = crate::store::lexical_normalize(Path::new(path.trim()));
            if promotions
                .iter()
                .any(|p| crate::store::lexical_normalize(Path::new(&p.destination)) == foreign)
            {
                Ok(Vec::new()) // in-location squatter — rewritten in place, not retired.
            } else {
                Ok(vec![foreign])
            }
        }
        Ok(_) => Ok(Vec::new()),
        Err(err) => Err(vec![source_path_io_finding(unit, task_dir, &err)]),
    }
}

/// The finalize-promote **clobber guard** (review S1, `design/auto-migration.md` →
/// Honest bounds → the data-loss clobber guard): a **create-provenance** staged doc whose
/// canonical promote destination already holds a committed managed doc would silently
/// overwrite it at promote — irreversible data loss. The planner blocks it
/// (`finalize.promote-clobber`), covering the title-slug collision across a doctype dir
/// AND the off-canonical in-location squatter together, **regardless of conformance or
/// migration-ness** (the colliding-corpus case is conformant-over-conformant).
///
/// Gated to [`crate::state::Provenance::Created`] — the load-bearing regression hinge: an
/// [`crate::state::Provenance::EditedFromBase`] doc was copied in from the committed store
/// at base and edited, so re-promoting it over its own canonical path is the intended
/// copy-on-first-touch update (the NGT committed-field-update path), never a clobber.
///
/// The **in-place migration rewrite** is excluded — for **any** doctype (the M43
/// same-path carve-out, fork 5, `DECISIONS.md` → 2026-07-16 M43 Settle #5): when the
/// recorded foreign `source-path` IS this destination, the committed file is the very
/// foreign original being rewritten in place — overwriting it is the whole point, and
/// `--approve` stays the sole destructive gate (the review hold renders the fidelity
/// diff downstream, so the overwrite is review-before-destroy). The retire-skip
/// ([`plan_retirements`] skips source == destination) and this carve-out compose to an
/// in-place `M`-not-`D`+`A` landing. Both path sides are
/// [`crate::store::lexical_normalize`]d so a `./`-prefixed or redundant-component
/// spelling still matches. (Pre-M43 the exclusion was `singleton`-gated — mirroring the
/// M24 blank-seed copy-in gate — which bounded every non-singleton same-path migration
/// out of end-to-end authoring; the create side needed no change for the carve-out, as
/// a non-singleton create always seeds blank.)
///
/// Each staged promotion's `<type>:<slug>` address is recovered from its source file stem
/// (the [`plan_promotions`] naming convention) and looked up in the task's provenance
/// manifest ([`crate::state::ProvenanceRecord`]). A staged doc with no recorded
/// provenance (none was minted/copied-in here) is not create-provenance, so it never
/// trips the guard.
fn plan_clobber_guard(
    unit: Unit,
    task_dir: &Path,
    repo_root: &Path,
    promotions: &[Promotion],
) -> Result<(), Vec<Finding>> {
    let provenance = crate::state::ProvenanceRecord::load(task_dir)
        .map_err(|err| vec![provenance_io_finding(unit, task_dir, &err)])?;
    // The in-place migration rewrite's destination (if any) — the foreign source path the
    // managed write replaces at its own canonical path, normalized for the comparison.
    let in_place = crate::state::read_source_path(task_dir)
        .map_err(|err| vec![source_path_io_finding(unit, task_dir, &err)])?
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| crate::store::lexical_normalize(Path::new(&s)));

    let mut clobbers = Vec::new();
    for promotion in promotions {
        let Some(address) = promotion.source.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if provenance.get(address) != Some(crate::state::Provenance::Created) {
            continue; // edited-from-base / unrecorded → never a clobber.
        }
        let dest_norm = crate::store::lexical_normalize(Path::new(&promotion.destination));
        if in_place.as_ref() == Some(&dest_norm) {
            continue; // in-place rewrite — replacing the very foreign original (M43, fork 5).
        }
        if repo_root.join(&promotion.destination).is_file() {
            clobbers.push(clobber_finding(&promotion.destination));
        }
    }
    if clobbers.is_empty() {
        Ok(())
    } else {
        Err(clobbers)
    }
}

/// A blocking finding (review S1, `design/auto-migration.md` → Honest bounds → the
/// data-loss clobber guard) when a create-provenance doc's canonical promote destination
/// already holds a file — a committed managed doc that would collide, or a hand-authored /
/// foreign file (e.g. an existing `VISION.md` at a placement home) — promoting would silently
/// overwrite it (irreversible data loss). Names the destination it refused to clobber, and
/// [keys at it](file_location).
fn clobber_finding(destination: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.promote-clobber",
        format!(
            "promoting this task's doc to `{destination}` would overwrite a file already \
             there — refusing to clobber it"
        ),
        Some(file_location(destination)),
        Some(
            format!(
                "a file already occupies `{destination}`: if it is a hand-authored/foreign file, \
             remove or adopt it; if it is another managed doc, retitle this one so it slugs \
             differently or resolve the collision; then re-run `jigc task finalize`"
            )
            .into(),
        ),
    )
}

/// A blocking finding for an I/O failure loading the task's provenance manifest while
/// planning the clobber guard (the manifest is written at stage time, so a read fault is a
/// real fault — the promote/source-path I/O precedent). Its subject is the **task** (its
/// manifest), so it [keys at the work unit](Unit::location).
fn provenance_io_finding(unit: Unit, task_dir: &Path, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.provenance-io",
        format!(
            "could not read the task provenance manifest under `{}`: {err}",
            task_dir.display()
        ),
        Some(unit.location()),
        None,
    )
}

/// The phase-1 decision over a moved base (`design/finalize.md` → Parallel
/// hand-editing, the 2026-06-12 phase-1 amendment): does the divergence between the
/// recorded base pin and the supplied HEAD touch the task's work?
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepinDecision {
    /// `base == HEAD` — the pin holds; nothing moved, nothing to re-pin.
    NoDivergence,
    /// HEAD moved, but on history **disjoint** from the task's work — finalize
    /// proceeds with HEAD as the *effective* pin (in-memory only; `base.json` is
    /// never rewritten — a blocked-then-retried finalize re-derives the decision).
    Repin,
}

/// Decide whether a task whose base moved can **auto-re-pin** to the new HEAD —
/// the serial-task phase-1 amendment (`design/finalize.md` → Parallel hand-editing,
/// 2026-06-12): re-pin iff *(paths changed in commits between the recorded base and
/// HEAD)* ∩ *(currently-dirty working-tree paths ∪ the task's promote destinations)*
/// = ∅; any overlap keeps the block — now with a conflict route **naming the
/// overlapping paths** — which is exactly the parallel-hand-editing case the
/// unconditional rejection existed to catch.
///
/// A pure decision over CLI-supplied git facts (the `has_diff` precedent — the
/// engine never shells out): `changed_paths` is the repo-relative base→HEAD path
/// set (`git diff --name-only <base> <head>`), `dirty_paths` the repo-relative
/// dirty working-tree set (`git status --porcelain`, untracked included — `git add
/// --all` would commit them). The promote destinations come from the shared phase-4
/// sweep ([`plan_promotions`]) over `task_dir` + `schemas`, so a staged managed doc
/// whose canonical destination the moved history touched also blocks.
///
/// [`plan_finalize`]'s phase-1 equality stays as defense — on a re-pin the CLI
/// feeds it the effective pin. The milestone sibling ([`plan_milestone_finalize`])
/// is consciously unchanged: the amendment targets the serial-task phase 1.
/// `task_id` is the task the decision speaks about — the **work-unit ref** its overlap block
/// keys at (M42 inc-9 T4; [`Unit::location`]).
pub fn decide_base_repin(
    task_id: &str,
    task_dir: &Path,
    base: &BasePin,
    head_sha: &str,
    changed_paths: &[String],
    dirty_paths: &[String],
    schemas: &BTreeMap<String, Schema>,
) -> Result<RepinDecision, Vec<Finding>> {
    if base.sha == head_sha {
        return Ok(RepinDecision::NoDivergence);
    }

    // The task's footprint: dirty working-tree paths ∪ promote destinations (the
    // shared phase-4 sweep keeps the destination derivation in one place).
    let promote = plan_promotions(task_dir, schemas)?;
    let mut footprint: BTreeSet<&str> = dirty_paths.iter().map(String::as_str).collect();
    footprint.extend(promote.promotions.iter().map(|p| p.destination.as_str()));

    // Sorted + deduped: the overlap naming is a pure function of the path *set*,
    // byte-identical across feed orders (Validation hardening #7).
    let overlapping: Vec<&str> = changed_paths
        .iter()
        .map(String::as_str)
        .filter(|p| footprint.contains(p))
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .collect();
    if overlapping.is_empty() {
        return Ok(RepinDecision::Repin);
    }
    Err(vec![base_overlap_finding(
        Unit::Task(task_id),
        base,
        head_sha,
        &overlapping,
    )])
}

/// Which finalize boundary the carryover refuse speaks for. The **wording** differs
/// because the *facts* differ (surface-contract law 1 — say the truth): a task
/// finalize is a whole-index commit a carried entry WOULD silently ride; a milestone
/// finalize builds its aggregate from the sub-task worktrees over targeted pathspecs
/// (throwaway indexes, dedicated worktrees, an `--ff-only` land), so a live-index
/// entry structurally CANNOT ride it — it **stays staged across the boundary**, and
/// the refuse is the declare-at-the-boundary rule, not a leak fix. The decision
/// itself is identical; only the finding's message/route change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryoverBoundary {
    /// `jigc task finalize` — the whole-index commit.
    Task,
    /// `jigc milestone finalize` — the aggregate commit built from the sub-task
    /// worktrees; a carried entry stays staged, never committed here.
    Milestone,
}

/// The **carryover decision** at the finalize commit boundary
/// (`design/surface-contract.md` → The carryover gate; M43): which currently-staged
/// paths were *staged before this work unit existed* — one blocking routed [`Finding`]
/// per carried path (`finalize.carried-staged`, keyed at the **file path** — the
/// `migrate-corpus.*` one-refusal-per-candidate precedent), so a foreign pre-staged
/// change (add, modify, **or delete**) never crosses the boundary undeclared.
///
/// A pure decision over CLI-supplied git facts (the [`decide_base_repin`] mold — the
/// engine never shells out): `snapshot` is the pre-task staged state the task-minting
/// door persisted ([`crate::state::read_staged_snapshot`]); `current` the same probe
/// re-run at finalize preflight. A path is **carried** iff its `(path, blob)` pair
/// matches a snapshot entry (the same path restaged to a *different* blob is the
/// task's own work), or it is a snapshot staged-deletion still staged (an entry-only
/// comparison is structurally blind to a pre-task `git rm` — the trial's A7 case).
/// `retire_exempt` is a migration task's recorded retire pathspec
/// ([`crate::state::read_source_path`]) — that deletion is the task's own, never a
/// carryover. A **`None` snapshot yields no findings** — the declared fail-open bound
/// (a task minted pre-M43 finalizes as today). `boundary` selects the honest wording
/// for the refusing verb ([`CarryoverBoundary`]).
///
/// Findings come out **sorted by path** across both halves (one union `BTreeSet`) —
/// byte-identical whatever order the sets were built in (Validation hardening #7).
/// A separate decision fn rather than a planner phase, so the CLI can keep the refuse
/// on the **committing** path only — a planner-internal refuse would also block the
/// `--dry-run` forecast, which consumes the plan.
pub fn decide_carryover(
    snapshot: Option<&StagedSnapshot>,
    current: &StagedSnapshot,
    retire_exempt: Option<&str>,
    boundary: CarryoverBoundary,
) -> Vec<Finding> {
    // Missing snapshot ⇒ fail-open: a task minted before the gate existed finalizes
    // as today (the declared bound).
    let Some(snapshot) = snapshot else {
        return Vec::new();
    };
    // The recorded retire path is stored prose (may carry `./`); the git-fact paths
    // are already repo-relative canonical — normalize the exempt side only (the
    // `plan_clobber_guard` source-path precedent).
    let exempt = retire_exempt.map(|p| crate::store::lexical_normalize(Path::new(p)));

    // The carried union, path-sorted by construction: entries whose (path, blob)
    // still match, plus snapshot deletions still staged. A `bool` discriminates the
    // two halves so the finding can say *what* is carried.
    let mut carried: BTreeMap<&str, bool> = BTreeMap::new();
    for (path, blob) in &snapshot.entries {
        if current.entries.get(path) == Some(blob) {
            carried.insert(path, false);
        }
    }
    for path in &snapshot.deletions {
        if current.deletions.contains(path) {
            carried.insert(path, true);
        }
    }
    carried
        .into_iter()
        .filter(|(path, _)| exempt.as_deref() != Some(Path::new(path)))
        .map(|(path, is_deletion)| carried_staged_finding(path, is_deletion, boundary))
        .collect()
}

/// One blocking `finalize.carried-staged` finding for one carried path — a staged
/// change (`is_deletion: false`) or a staged deletion (`true`) that predates the work
/// unit. Its subject is the **file** staged before the boundary's unit existed, so it
/// [keys at its path](file_location) (the file-path target form — mid-carry the path
/// may be foreign, with no managed identity). The route names both exits: unstage it,
/// or re-run finalize with `--carry-staged` to declare the carry-over deliberate (the
/// `--approve` mold — undecidable intent converted to a declared one). Which exit is
/// right is a judgment call, so the route is [`Route::human`]. The message states the
/// boundary's real consequence ([`CarryoverBoundary`], law 1): a task's whole-index
/// commit would silently absorb the entry; a milestone's aggregate cannot carry it —
/// the entry stays staged across the boundary either way.
fn carried_staged_finding(path: &str, is_deletion: bool, boundary: CarryoverBoundary) -> Finding {
    let what = if is_deletion {
        "staged for deletion"
    } else {
        "staged"
    };
    let kind = if is_deletion { "deletion" } else { "change" };
    let (message, route) = match boundary {
        CarryoverBoundary::Task => (
            format!(
                "`{path}` was already {what} before this task existed — refusing to let a \
                 pre-task staged {kind} silently ride this task's commit"
            ),
            format!(
                "unstage it (`git restore --staged -- {path}`) if it is not this task's work, \
                 or re-run the finalize with `--carry-staged` to declare the carry-over \
                 deliberate"
            ),
        ),
        CarryoverBoundary::Milestone => (
            format!(
                "`{path}` was already {what} before this milestone existed — the aggregate \
                 commit is built from the sub-task worktrees and cannot carry it, so the \
                 {kind} stays staged, undeclared, across this boundary"
            ),
            format!(
                "unstage it (`git restore --staged -- {path}`) if it is stale, or re-run \
                 the finalize with `--carry-staged` to declare it deliberate (it stays \
                 staged either way)"
            ),
        ),
    };
    Finding::graded(
        Severity::Blocking,
        "finalize.carried-staged",
        message,
        Some(file_location(path)),
        Some(Route::human(route)),
    )
}

/// Plan the **milestone** `finalize` transaction — the thin sibling of
/// [`plan_finalize`] for the fan-out single-commit boundary (`design/finalize.md` →
/// `fan-out` finalize, single-commit form; `DECISIONS.md` 2026-06-04 → the inc-4
/// fork resolution: M7 ships one synthesized commit).
///
/// It shares [`plan_finalize`]'s phases — the preflight (`staging_dir` exists, `base`
/// reconciled with `head_sha` — see `record_only_advance` below), the empty-commit guard,
/// and the phase-4 promote / phase-7 hash sweep
/// ([`plan_promotions`]) — with **two** structural differences a milestone forces:
///
/// - **No commit-doc render.** A milestone has no `commit:<slug>` doc to render (the
///   commit doc is per-task); its message is the **CLI-synthesized** structural
///   projection of the milestone id + its id-ordered sub-task list
///   ([`crate::milestone::synthesized_message`]). The caller passes that pre-rendered
///   `message` in and the planner carries it verbatim — substituting it for phase 3.
/// - **No validate report.** The join already adjudicated the merge (a same-doc clash
///   is a blocking finding the CLI blocks on *before* materializing, the `materialize`
///   precedent), so the planner takes no `report` — by the time it runs, the merged
///   overlay has been materialized into `staging_dir/docs/` clean.
///
/// `staging_dir` is the materialized parent staging area
/// ([`crate::milestone::MaterializeOutcome::docs_dir`]'s parent — the `merged/` dir),
/// whose `docs/` holds the suffix-resolved bodies in the same staging form a single
/// task's working area uses, so the shared promote sweep reads it unchanged. Performs
/// no git and no commit; reads only `staging_dir`.
// The determinism contract feeds every layer in explicitly (the [`plan_finalize`] precedent);
// bundling the inputs into a params struct would be churn without clarifying the contract.
#[allow(clippy::too_many_arguments)]
pub fn plan_milestone_finalize(
    milestone_id: &str,
    staging_dir: &Path,
    base: &BasePin,
    head_sha: &str,
    record_only_advance: bool,
    message: String,
    has_diff: bool,
    schemas: &BTreeMap<String, Schema>,
) -> Result<FinalizePlan, Vec<Finding>> {
    // The work unit every shared block keys at — `milestone:<id>`, never a guess derived from
    // `staging_dir` (M42 inc-9 T4; `design/command-output-contract.md` → the finalize
    // sub-table). The three constructors below are shared with [`plan_finalize`], so the
    // caller's unit is what tells them apart.
    let unit = Unit::Milestone(milestone_id);

    // Preflight (shared): the staging area exists, and the base pin is reconciled with the
    // supplied HEAD. The base-guard is refined for the per-op record-commit model
    // (`design/team-ready-state.md` → The commit model: the finalize base-guard refinement;
    // `DECISIONS.md` 2026-07-07 → the Inc-4/T4 fork): proceed when `base == HEAD` (the fast
    // path), OR when the CLI has verified `base..HEAD` is a linear-ancestor range whose every
    // commit touches only this milestone's record path (`record_only_advance`) — then the base
    // advances to HEAD. Any non-record commit in the range keeps the base-mismatch block, so
    // **external** drift (which would invalidate the M31 worktree-combine) is still caught; the
    // milestone's own record-only bookkeeping (touching no code) is tolerated. The engine does
    // no git I/O — the CLI computes the range verdict and supplies it here. The block it raises
    // is the MILESTONE arm of [`base_mismatch_finding`] (M42 T3): same code, a route that names
    // the milestone, the cause, and the two real (out-of-band git) options — never `discard`.
    if !staging_dir.exists() {
        return Err(vec![task_missing_finding(unit, staging_dir)]);
    }
    if base.sha != head_sha && !record_only_advance {
        return Err(vec![base_mismatch_finding(unit, base, head_sha)]);
    }

    // Empty-commit guard (shared): validate-equivalent passed (the join adjudicated),
    // but a materialized-but-empty diff still aborts — no empty commits.
    if !has_diff {
        return Err(vec![empty_commit_finding(unit)]);
    }

    // No commit-doc render — the synthesized message is substituted for phase 3. Phase 4
    // promote + phase 7 hashes are the SHARED sweep over the materialized `docs/`.
    let promote = plan_promotions(staging_dir, schemas)?;
    // A milestone boundary retires nothing — retire is migration-only (a per-task verb).
    Ok(FinalizePlan::new(
        message,
        promote.promotions,
        promote.hash_updates,
        Vec::new(),
    ))
}

/// Render the **per-sub-task** authored commit messages for the `squash: false`
/// fan-out finalize mode (`design/finalize.md` → `fan-out` finalize: "one commit
/// per sub-task in task-id order … the parent finalize renders each in id order").
///
/// Given the sub-task ids (the caller passes them **already id-sorted** —
/// [`crate::milestone::TaskList::enumerate`] order, the order the commit *sequence*
/// is laid down in), the `tasks/` namespace root they live under, and the
/// cascade-resolved `commit` doc schema, read each sub-task's authored
/// `commit:<sub-id>` doc (`<tasks_root>/<sub-id>/docs/commit:<sub-id>.md` — a
/// sub-task's commit slug is its own task id, the `commit:<task-id>` provisioning
/// convention) and render it into its git-message string via
/// [`crate::write::render_commit_message`]. Returns one message per sub-task in the
/// **given (id-sorted) order**, so the rendered *sequence* is a pure function of the
/// sub-task id *set* — byte-identical across feed/completion orders (Validation
/// hardening #7), even though each message carries the sub-agent's authored prose
/// (the CLI owns the ordering + placement; the prose is the sub-agent's — the
/// determinism boundary).
///
/// A missing or unparseable authored commit doc for any sub-task is a blocking
/// [`Finding`] (the `plan_finalize` render precedent) — a sub-task that fanned out
/// must have authored its commit doc before the parent finalize renders it.
/// Performs no git and no commit; reads only the sub-task working areas.
pub fn render_subtask_messages(
    sub_ids: &[String],
    tasks_root: &Path,
    commit_schema: &Schema,
) -> Result<Vec<String>, Vec<Finding>> {
    let mut messages = Vec::with_capacity(sub_ids.len());
    for sub_id in sub_ids {
        let commit_path = tasks_root
            .join(sub_id)
            .join(DOCS_DIR)
            .join(format!("{}:{sub_id}.md", commit_schema.ty));
        let source = std::fs::read_to_string(&commit_path)
            .map_err(|err| vec![render_io_finding(&commit_path, &err)])?;
        let instance = write::instance_from_source(commit_schema, &source)?;
        messages.push(write::render_commit_message(commit_schema, &instance));
    }
    Ok(messages)
}

/// The phase-4 promote decision: the staged docs to copy plus their phase-7 hashes.
struct PromotePlan {
    promotions: Vec<Promotion>,
    hash_updates: BTreeMap<String, String>,
}

/// Phase 4 + the phase-7 hash set: walk `<task_dir>/docs/*.md`, and for every staged
/// instance whose type declares a persisted home, name a [`Promotion`] to its canonical
/// destination plus the blake3 of its staged bytes. A `location:` doctype promotes to
/// `<location>/<slug>.md`; a **placement** doctype (`design/storage.md` → Placement)
/// promotes to its one literal `placement.file` repo-root-relative path (case-preserved,
/// no docs-root, no slug). Promotions are returned in canonical-destination order (sorted,
/// deterministic). The commit doc and any unknown/transient type contribute nothing.
///
/// A staged file lives at `<task_dir>/docs/<type>:<slug>.md`; its filename stem is the
/// `<type>:<slug>` address. An I/O failure reading the docs dir or a staged file is a
/// blocking [`Finding`] (the promote phase cannot proceed half-applied).
fn plan_promotions(
    task_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> Result<PromotePlan, Vec<Finding>> {
    let docs_dir = task_dir.join(DOCS_DIR);
    let entries = match std::fs::read_dir(&docs_dir) {
        Ok(entries) => entries,
        // No `docs/` dir means nothing was staged — an empty promote set.
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok(PromotePlan {
                promotions: Vec::new(),
                hash_updates: BTreeMap::new(),
            });
        }
        Err(err) => return Err(vec![promote_io_finding(&docs_dir, &err)]),
    };

    let mut staged: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .collect();
    staged.sort();

    let mut promotions = Vec::new();
    let mut hash_updates = BTreeMap::new();
    for source in staged {
        // The filename stem is the `<type>:<slug>` address; split on the first `:`.
        let Some(stem) = source.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some((ty, slug)) = stem.split_once(':') else {
            continue; // not a `<type>:<slug>` instance — skip.
        };
        let Some(schema) = schemas.get(ty) else {
            continue;
        };
        // Resolve the canonical destination: a **placement** doctype
        // (`design/storage.md` → Placement) lands at its one literal `placement.file`
        // repo-root-relative path (case-preserved, no docs-root, no slug); a `location:`
        // doctype lands at `<location>/<slug>.md`; a type with neither is transient (the
        // commit doc) and is never promoted.
        let destination = if let Some(placement) = &schema.placement {
            placement.file.clone()
        } else if let Some(location) = schema.location.as_deref() {
            format!("{}/{slug}.md", location.trim_end_matches('/'))
        } else {
            continue;
        };

        let bytes =
            std::fs::read(&source).map_err(|err| vec![promote_io_finding(&source, &err)])?;
        hash_updates.insert(destination.clone(), hash_bytes(&bytes));
        promotions.push(Promotion {
            source,
            destination,
        });
    }

    promotions.sort_by(|a, b| a.destination.cmp(&b.destination));
    Ok(PromotePlan {
        promotions,
        hash_updates,
    })
}

/// A blocking finding for an I/O failure walking the staged docs or reading a staged
/// doc during the promote phase. Its subject is the staged **file**, so it
/// [keys at its path](file_location).
fn promote_io_finding(path: &Path, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.promote-io",
        format!(
            "could not read the staged managed doc `{}` to promote it: {err}",
            path.display()
        ),
        Some(file_location(path.display())),
        None,
    )
}

/// A blocking finding (review F1) when a migration task recorded a foreign `source-path`
/// but staged **no** managed doc to promote in its place — the retire would delete the
/// foreign original with no canonical replacement. The migration must author its
/// canonical doc before finalize; refusing here keeps the first byte-destructive write
/// honest (never delete-with-no-replacement). Its subject is the **foreign source file**, so
/// it [keys at its path](file_location).
fn migration_no_replacement_finding(source_path: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.migration-no-replacement",
        format!(
            "this migration recorded the foreign source `{source_path}` but staged no \
             managed doc to replace it — the foreign original will not be retired with \
             nothing to take its place"
        ),
        Some(file_location(source_path)),
        Some(
            "author the canonical doc (e.g. `jigc doc create <doctype> --task <id>`), then \
             re-run `jigc task finalize <id> --approve`"
                .into(),
        ),
    )
}

/// A blocking finding for an I/O failure reading the migration task's recorded
/// `source-path` while planning the retire set. Its subject is the **task** (its recorded
/// source path), so it [keys at the work unit](Unit::location).
fn source_path_io_finding(unit: Unit, task_dir: &Path, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.source-path-io",
        format!(
            "could not read the recorded migration source path under `{}`: {err}",
            task_dir.display()
        ),
        Some(unit.location()),
        None,
    )
}

/// The phase-1 task-missing reject (`finalize.md` → 1. Preflight: "Task exists"). Its
/// subject is the work unit whose area is absent, so it
/// [keys at the work unit](Unit::location) — never at the missing path (the *unit* is what
/// a driver tracks; the path is an artifact of where the workbench happens to sit).
fn task_missing_finding(unit: Unit, task_dir: &Path) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.no-task",
        format!(
            "no task working area at `{}` — nothing to finalize",
            task_dir.display()
        ),
        Some(unit.location()),
        Some("start a task with `jigc start \"<intent>\"`".into()),
    )
}

/// The work unit a `finalize.*` block speaks about — one `code`, two units whose cause and
/// whose exits are different (M42 T3; `design/write-commands.md` → The
/// `finalize.base-mismatch` route is unit-aware), and whose **stable key target** is the
/// unit's own ref (M42 inc-9 T4; [`Unit::location`]).
///
/// It carries the unit's **id**, because the shared constructors ([`task_missing_finding`],
/// [`base_mismatch_finding`], [`empty_commit_finding`]) serve both planners and a
/// path-derived guess would be exactly the degenerate target the contract forbids.
#[derive(Clone, Copy)]
enum Unit<'a> {
    /// A serial task ([`plan_finalize`]) — `task:<id>`.
    Task(&'a str),
    /// A milestone ([`plan_milestone_finalize`]) — `milestone:<id>`.
    Milestone(&'a str),
}

impl Unit<'_> {
    /// The unit's [`Location`] — the **work-unit ref** target form `task:<id>` /
    /// `milestone:<id>` a `finalize.*` block whose subject is the work unit keys at
    /// ([command-output-contract.md](../../../design/command-output-contract.md) → the form
    /// table, the work-unit row; the finalize sub-table). Without it every diverged task in
    /// every repo collided on the one degenerate key `(finalize.base-mismatch, null)`. A
    /// container address, no fragment (`structural-grammar.md` → Work-units and runtime
    /// identity).
    fn location(&self) -> Location {
        let address = match self {
            Self::Task(id) => format!("task:{id}"),
            Self::Milestone(id) => format!("milestone:{id}"),
        };
        Location::addressed(address, 1, 1)
    }
}

/// The [`Location`] of a `finalize.*` block whose subject is a **file** — the promote
/// destination it refused to clobber, the staged doc it could not read, the foreign original
/// it would not retire. It keys at the **filesystem path** form, the `file-state.*` exception
/// generalized: the subject may be a *foreign* file carrying no committed URI identity, so a
/// `<type>:<slug>` target would name a doc that does not exist
/// ([command-output-contract.md](../../../design/command-output-contract.md) → the form table,
/// the file row; the finalize sub-table).
fn file_location(path: impl std::fmt::Display) -> Location {
    Location::addressed(path.to_string(), 1, 1)
}

/// The phase-1 base-pin-divergence reject (`finalize.md` → 1. Preflight: "Base pin
/// matches HEAD"). A blocking finding carrying the divergence-routing prompt — the
/// CLI never operates a task (or a milestone) off its pinned base.
///
/// The route is **unit-aware** (M42 T3 — `design/team-ready-state.md` → What the base-guard
/// is for → The route; `design/write-commands.md` → the unit-aware route). Both units share
/// the code; nothing else about them is the same:
///
/// - **Task** — a serial task pinned to `<A>` while HEAD moved: switch back to `<A>`, or
///   discard the task (a cheap, honest exit for a task — `jigc task discard`, which also
///   re-mints). Unchanged.
/// - **Milestone** — a milestone's base is pinned once at `create` and **never** re-pinned
///   ([`decide_base_repin`] is consciously task-only: a task's footprint is its dirty paths +
///   promote destinations, which the overlap test can see; a milestone's is code sitting in
///   worktrees cut from the base, which it cannot). So any commit that moved code after the
///   base invalidates it — the M31 worktree-combine cannot be proven sound against a HEAD
///   whose code has moved. The route therefore names the **cause** and the two **real**
///   options (land the milestone first, or re-cut onto the new base — both honestly labelled
///   out-of-band git; there is no `jigc milestone rebase` at M42, deliberately deferred). It
///   never offers `discard`: `jigc milestone discard` settles the record of an **abandoned**
///   milestone, so routing a still-wanted one there tells the operator to destroy the work to
///   satisfy a guard.
fn base_mismatch_finding(unit: Unit, base: &BasePin, head_sha: &str) -> Finding {
    let (message, route) = match unit {
        Unit::Task(_) => (
            format!(
                "the task was started at base `{}` but HEAD is now `{head_sha}`",
                base.short
            ),
            format!(
                "switch back to `{}` or discard the task with `jigc task discard`",
                base.short
            ),
        ),
        Unit::Milestone(_) => (
            format!(
                "the milestone was pinned to base `{}` but HEAD is now `{head_sha}`, and the \
                 commits landed since move more than milestone-record bookkeeping — the \
                 sub-task worktrees were cut from `{}`, so combining them onto HEAD cannot be \
                 proven sound",
                base.short, base.short
            ),
            format!(
                "land this milestone's work first (out-of-band git: return HEAD to `{}`, run \
                 `jigc milestone finalize`, then re-land the newer commits on top), or re-cut \
                 this milestone's work onto the new base (out-of-band git: re-provision the \
                 sub-task worktrees from HEAD and re-apply each sub-task's staged changes)",
                base.short
            ),
        ),
    };
    Finding::graded(
        Severity::Blocking,
        "finalize.base-mismatch",
        message,
        Some(unit.location()),
        Some(route.into()),
    )
}

/// The overlap form of the phase-1 base-divergence block (`finalize.md` → Parallel
/// hand-editing, the 2026-06-12 amendment): HEAD moved on history that **touches the
/// task's work**, so no auto-re-pin — the finding names the overlapping paths and
/// carries the resolve-or-discard conflict route. `overlapping` is sorted + deduped
/// by [`decide_base_repin`].
///
/// The second constructor of the one `finalize.base-mismatch` code — mutually exclusive with
/// [`base_mismatch_finding`]'s pin form (one instance per finalize), and
/// [keyed at the same work unit](Unit::location).
fn base_overlap_finding(
    unit: Unit,
    base: &BasePin,
    head_sha: &str,
    overlapping: &[&str],
) -> Finding {
    let paths = overlapping.join("`, `");
    Finding::graded(
        Severity::Blocking,
        "finalize.base-mismatch",
        format!(
            "the task was started at base `{}` but HEAD is now `{head_sha}`, and the moved \
             history overlaps the task's work on `{paths}`",
            base.short
        ),
        Some(unit.location()),
        Some(
            format!(
                "resolve the overlap on `{paths}` against the new history, or discard the task \
             with `jigc task discard`"
            )
            .into(),
        ),
    )
}

/// The empty-commit abort (`finalize.md` → Commit-doc rendering → Empty commit):
/// validate passed but the staged diff is empty. No empty commits. Its subject is the work
/// unit that produced nothing, so it [keys at it](Unit::location) — shared by both planners,
/// so the unit is passed in, never guessed from a path.
fn empty_commit_finding(unit: Unit) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.empty-commit",
        "task validated but produced no diff — nothing to finalize",
        Some(unit.location()),
        Some("make a change, then re-run `jigc task finalize`".into()),
    )
}

/// A blocking finding for an I/O failure reading the staged commit doc during render. Its
/// subject is the staged **file**, so it [keys at its path](file_location).
fn render_io_finding(path: &Path, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.render-io",
        format!(
            "could not read the staged commit doc `{}`: {err}",
            path.display()
        ),
        Some(file_location(path.display())),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state;
    use std::path::PathBuf;

    /// The finding's **key target** — the address its stable `(code, target)` key derives
    /// from ([`Finding::key`]). Every `finalize.*` block declares one: the work-unit ref
    /// where the subject is the unit, the file path where it is a file
    /// (`design/command-output-contract.md` → the finalize sub-table).
    fn target(finding: &Finding) -> Option<&str> {
        finding
            .location
            .as_ref()
            .and_then(|location| location.address.as_deref())
    }

    /// A no-delta resolved cascade — the post-pass leaves every emitted severity
    /// untouched, so a report built over it carries exactly its emitted findings.
    fn no_delta_resolved() -> crate::cascade::Resolved {
        crate::cascade::resolve(
            &crate::cascade::PackDefaultLayer::new(
                "dev-pack",
                "0.1.0",
                std::collections::BTreeMap::new(),
                Vec::new(),
            ),
            None,
            None,
        )
        .expect("resolves")
    }

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-finalize-{tag}-{}-{:?}",
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

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");
    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const CHANGELOG_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/changelog.yaml");

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }

    /// The `changelog` singleton (fixed slug = the type id, location `changelog/`) — the
    /// singleton arm of the clobber guard's doctype-blind in-place-rewrite carve-out
    /// (M43, fork 5), exercised on a real singleton schema.
    fn changelog_schema() -> Schema {
        crate::schema::load_schema_with_types(
            CHANGELOG_YAML,
            &crate::schema::dev_pack_field_types(),
        )
        .expect("changelog.yaml loads")
    }

    /// The cascade-resolved schema set the planner reads in phase 4: the `commit`
    /// (transient, location-less) + `adr` (persisted to `decisions/`) + `changelog`
    /// (a persisted singleton) types.
    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert("commit".to_string(), commit_schema());
        m.insert("adr".to_string(), adr_schema());
        m.insert("changelog".to_string(), changelog_schema());
        m
    }

    /// A throwaway **placement** doctype (M38 inc-1): a singleton whose one instance
    /// lives at the literal repo-root-relative `file` — **no `location`**, docs-root
    /// never applies. A single prose slot keeps a staged instance trivially conformant.
    fn placement_schema(ty: &str, file: &str) -> Schema {
        let yaml = format!(
            "\
type: {ty}
placement: {{ file: {file} }}
singleton: true
sections:
  - id: body
    slot: {{ hint: the placement body }}
"
        );
        crate::schema::load_schema(yaml.as_bytes()).expect("placement schema loads")
    }

    /// Stage a filled placement instance (one body slot) at `<task_dir>/docs/<ty>:<slug>.md`
    /// — a persisted managed doc the promote phase carries to its literal `placement.file`.
    /// The slug is the singleton's fixed slug (= type id). Returns the staged bytes (the
    /// byte-stable source the hash is taken over).
    fn stage_filled_placement(task_dir: &Path, schema: &Schema, slug: &str) -> Vec<u8> {
        use crate::write::SectionContent;

        let instance = write::Instance {
            title: slug.to_string(),
            sections: vec![SectionContent {
                id: "body".to_string(),
                slot: Some("A placement singleton at its literal home.".to_string()),
                ..Default::default()
            }],
        };
        let bytes = write::render(schema, &instance);
        let path = state::instance_path(task_dir, &schema.ty, slug);
        state::persist(&path, bytes.as_bytes()).expect("persist staged placement");
        bytes.into_bytes()
    }

    fn base() -> BasePin {
        BasePin::new("0123456789abcdef0123456789abcdef01234567", "0123456")
    }

    /// Stage a filled `commit:<slug>` doc in `task_dir` (a `feat` type + a summary)
    /// by rendering a populated instance to the canonical bytes the planner re-parses
    /// and re-renders. Returns the commit schema.
    fn stage_filled_commit(task_dir: &Path, slug: &str) -> Schema {
        use crate::field_block::Value;
        use crate::write::SectionContent;

        let schema = commit_schema();
        let instance = write::Instance {
            title: slug.to_string(),
            sections: vec![
                SectionContent {
                    id: "header".to_string(),
                    fields: vec![crate::field_block::Field {
                        key: "type".to_string(),
                        value: Value::Scalar("feat".to_string()),
                    }],
                    ..Default::default()
                },
                SectionContent {
                    id: "summary".to_string(),
                    slot: Some("add a per-client rate limiter".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    ..Default::default()
                },
                SectionContent {
                    id: "trailers".to_string(),
                    ..Default::default()
                },
            ],
        };
        let bytes = write::render(&schema, &instance);
        let path = state::instance_path(task_dir, &schema.ty, slug);
        state::persist(&path, bytes.as_bytes()).expect("persist staged commit");
        schema
    }

    /// Stage a filled `adr:<slug>` doc in `task_dir` (status `accepted` + a decision
    /// slot) — a persisted managed doc the promote phase carries to `decisions/`.
    /// Returns the staged bytes (the byte-stable source the hash is taken over).
    fn stage_filled_adr(task_dir: &Path, slug: &str) -> Vec<u8> {
        use crate::field_block::{Field, Value};
        use crate::write::SectionContent;

        let schema = adr_schema();
        let instance = write::Instance {
            title: slug.to_string(),
            sections: vec![
                SectionContent {
                    id: "status".to_string(),
                    fields: vec![
                        Field {
                            key: "status".to_string(),
                            value: Value::Scalar("accepted".to_string()),
                        },
                        Field {
                            key: "date".to_string(),
                            value: Value::Scalar("2026-05-31".to_string()),
                        },
                    ],
                    ..Default::default()
                },
                SectionContent {
                    id: "context".to_string(),
                    slot: Some("Session lookups must stay sub-millisecond.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "options".to_string(),
                    slot: Some("Alternatives were weighed and rejected.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "decision".to_string(),
                    slot: Some("A single in-memory node keeps lookups fast.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "consequences".to_string(),
                    slot: Some("A cold node loses its sessions.".to_string()),
                    ..Default::default()
                },
            ],
        };
        let bytes = write::render(&schema, &instance);
        let path = state::instance_path(task_dir, &schema.ty, slug);
        state::persist(&path, bytes.as_bytes()).expect("persist staged ADR");
        bytes.into_bytes()
    }

    /// The done-criterion: the planner orders the phases and aborts on each blocking
    /// branch, only producing a plan on a clean run.
    ///
    /// - **base-pin != HEAD** → the divergence-routing blocking finding, **no plan**.
    /// - **blocking validate findings** → abort at phase 2 carrying exactly those.
    /// - **clean task** → a plan with the rendered message + the (empty, commit-only)
    ///   post-commit hash set.
    /// - **empty diff** → the "produced no diff" abort.
    #[test]
    fn finalize_planner_orders_phases_and_aborts_on_block() {
        let root = TempRoot::new("orders");
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        let schema = stage_filled_commit(&task_dir, "add-rate-limiter");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        // (Preflight) base-pin != supplied HEAD → divergence block, no plan.
        let err = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            &clean,
            true,
            &schema,
            "add-rate-limiter",
            &schemas(),
        )
        .expect_err("a base mismatch aborts preflight");
        assert_eq!(err.len(), 1, "one preflight finding");
        assert_eq!(err[0].code, "finalize.base-mismatch");
        assert_eq!(err[0].severity, Severity::Blocking);
        assert_eq!(
            target(&err[0]),
            Some("task:add-rate-limiter"),
            "the diverged TASK keys at its own work-unit ref — else every diverged task in \
             every repo collides on `(finalize.base-mismatch, null)`",
        );
        assert!(
            err[0].route.is_some(),
            "the divergence block carries a switch-back/discard route"
        );

        // (Phase 2) blocking validate findings → abort with exactly those.
        // Both fixtures carry a declared target — the doc URI form — because a report is a
        // serialization funnel and every finding reaching one owes a key
        // (`command-output-contract.md` → The membership test).
        let blocking = Finding::graded(
            Severity::Blocking,
            "schema-conformance.required-slot-present",
            "required slot `body` is empty",
            Some(Location::addressed("commit:add-rate-limiter#body", 1, 1)),
            None,
        );
        let advisory = Finding::graded(
            Severity::Advisory,
            "commit-rendering.line-limit-subject",
            "subject is 71 chars",
            Some(Location::addressed("commit:add-rate-limiter#subject", 1, 1)),
            None,
        );
        let report = ValidationReport::new(vec![advisory, blocking.clone()], &no_delta_resolved());
        let err = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &report,
            true,
            &schema,
            "add-rate-limiter",
            &schemas(),
        )
        .expect_err("blocking validate findings abort at phase 2");
        assert_eq!(
            err,
            vec![blocking],
            "phase 2 surfaces exactly the blocking findings (advisory dropped)"
        );

        // (Empty-commit guard) validate clean but no diff → produced-no-diff abort.
        let err = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            false,
            &schema,
            "add-rate-limiter",
            &schemas(),
        )
        .expect_err("an empty diff aborts");
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "finalize.empty-commit");
        assert_eq!(
            target(&err[0]),
            Some("task:add-rate-limiter"),
            "the empty-commit abort keys at the work unit that produced nothing",
        );
        assert!(
            err[0].message.contains("produced no diff"),
            "the empty-commit message is the produced-no-diff abort: {:?}",
            err[0]
        );

        // (Clean) base matches, validate clean, diff present → a plan with the
        // rendered message + the empty (commit-only) post-commit hash set.
        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "add-rate-limiter",
            &schemas(),
        )
        .expect("a clean task yields a plan");
        assert_eq!(
            plan.message, "feat: add a per-client rate limiter",
            "the plan carries the rendered commit message (phase 3)"
        );
        assert!(
            plan.promotions.is_empty() && plan.hash_updates.is_empty(),
            "a commit-only task stages no persisted doc — nothing to promote"
        );
        assert_eq!(plan.schema_version, SCHEMA_VERSION);
    }

    /// A **migration** task records the repo-relative foreign source path
    /// (`<task_dir>/source-path`, written by `jigc migrate`); the planner reads it back
    /// and names it in [`FinalizePlan::retirements`] — the retire-the-foreign-original
    /// target (`design/auto-migration.md` → Retire-the-foreign-original). The omitting
    /// context — a **non-migration** task with no `source-path` — retires **nothing**
    /// (inert, never an error).
    #[test]
    fn finalize_plan_retires_the_recorded_foreign_path() {
        let root = TempRoot::new("retire");
        let task_dir = root.path().join("tasks").join("changelog");
        let schema = stage_filled_commit(&task_dir, "changelog");
        // A promotable managed doc stands in for the migration's authored canonical
        // replacement (review F1: the retire fires only when one was produced).
        stage_filled_adr(&task_dir, "single-node-cache");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        // Omitting context: no `source-path` file → an empty retire set.
        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "changelog",
            &schemas(),
        )
        .expect("a clean non-migration task yields a plan");
        assert!(
            plan.retirements.is_empty(),
            "a non-migration task retires nothing (inert omitting context)"
        );

        // A migration task records its foreign source path → the plan retires exactly it.
        crate::state::persist(&task_dir.join("source-path"), b"CHANGELOG.md")
            .expect("record the foreign source path");
        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "changelog",
            &schemas(),
        )
        .expect("a clean migration task yields a plan");
        assert_eq!(
            plan.retirements,
            vec![PathBuf::from("CHANGELOG.md")],
            "the recorded repo-relative foreign path is the retire target",
        );
    }

    /// Review F1: a migration task that recorded a foreign `source-path` but staged
    /// **no** managed doc to promote in its place must **block** — never retire (delete)
    /// the foreign original with no canonical replacement. The fillable `commit:<id>` doc
    /// is transient (location-less, never promoted), so a commit-doc-only migration has an
    /// empty promote set; that is exactly the data-loss case the precondition closes.
    #[test]
    fn finalize_plan_blocks_a_migration_that_promoted_no_replacement() {
        let root = TempRoot::new("retire-no-replacement");
        let task_dir = root.path().join("tasks").join("changelog");
        // Only the transient commit doc is staged — no managed (persisted) doc.
        let schema = stage_filled_commit(&task_dir, "changelog");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        // The migration recorded the foreign original it intends to replace …
        crate::state::persist(&task_dir.join("source-path"), b"CHANGELOG.md")
            .expect("record the foreign source path");

        // … but authored nothing to replace it → the planner blocks (no plan, so the CLI
        // never reaches the byte-destructive retire).
        let findings = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "changelog",
            &schemas(),
        )
        .expect_err("a migration that promoted no replacement must block");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, "finalize.migration-no-replacement");
        assert_eq!(findings[0].severity, Severity::Blocking);
        assert_eq!(
            target(&findings[0]),
            Some("CHANGELOG.md"),
            "the refused retire keys at the foreign source FILE — it has no managed identity",
        );
        assert!(
            findings[0].message.contains("CHANGELOG.md"),
            "the finding names the foreign original it refused to retire: {:?}",
            findings[0]
        );
    }

    /// The path-collision guard (`design/auto-migration.md` → Path-collision guard,
    /// the in-location squatter): when the recorded foreign source path **equals** the
    /// canonical managed path a staged doc promotes to, the managed write *is* the
    /// in-place rewrite — the planner **skips** the retire (it would otherwise delete
    /// the doc it just wrote). A foreign source at a **distinct** path still retires
    /// (the common root-`CHANGELOG.md` case — T2 — is unaffected). One mechanism, both
    /// cases. Review F2: the guard normalizes both sides, so a `./`-prefixed or
    /// redundant-component spelling of the canonical path is still recognized as the
    /// squatter (it would otherwise slip the guard and delete the just-written doc).
    #[test]
    fn finalize_plan_skips_retire_when_source_is_the_promote_destination() {
        let root = TempRoot::new("retire-collision");
        let task_dir = root.path().join("tasks").join("changelog");
        let schema = stage_filled_commit(&task_dir, "changelog");
        // A staged doc whose canonical destination is `decisions/single-node-cache.md`.
        stage_filled_adr(&task_dir, "single-node-cache");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        // Collision: the recorded foreign source IS the staged doc's promote
        // destination — the in-location squatter rewritten in place.
        crate::state::persist(
            &task_dir.join("source-path"),
            b"decisions/single-node-cache.md",
        )
        .expect("record the in-location foreign source path");
        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "changelog",
            &schemas(),
        )
        .expect("a clean in-place migration yields a plan");
        assert!(
            plan.retirements.is_empty(),
            "no retire fires against the just-written canonical path (in-place rewrite)",
        );
        assert_eq!(
            plan.promotions.len(),
            1,
            "the squatter is still promoted (written in place)",
        );
        assert_eq!(
            plan.promotions[0].destination,
            "decisions/single-node-cache.md",
        );

        // Review F2: a redundantly-spelled foreign source path (`./`-prefixed, or with a
        // `..` round-trip) of the canonical destination must STILL fire the guard — it is
        // the same in-location squatter, so retire is skipped (not delete the just-written
        // doc). Pre-fix these slipped the exact-match guard and deleted the canonical doc.
        for spelling in [
            "./decisions/single-node-cache.md",
            "decisions/../decisions/single-node-cache.md",
        ] {
            crate::state::persist(&task_dir.join("source-path"), spelling.as_bytes())
                .expect("record a redundantly-spelled in-location foreign source path");
            let plan = plan_finalize(
                &task_dir,
                root.path(),
                &base(),
                &base().sha,
                &clean,
                true,
                &schema,
                "changelog",
                &schemas(),
            )
            .expect("a clean in-place migration yields a plan");
            assert!(
                plan.retirements.is_empty(),
                "the guard normalizes the spelling `{spelling}` — no retire fires against \
                 the just-written canonical path",
            );
        }

        // A distinct foreign path still retires — the root-`CHANGELOG.md` case (T2).
        crate::state::persist(&task_dir.join("source-path"), b"CHANGELOG.md")
            .expect("record a distinct foreign source path");
        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "changelog",
            &schemas(),
        )
        .expect("a clean distinct-path migration yields a plan");
        assert_eq!(
            plan.retirements,
            vec![PathBuf::from("CHANGELOG.md")],
            "a distinct foreign original still retires (the T2 root-file case)",
        );
    }

    /// A missing task working area aborts preflight before anything else.
    #[test]
    fn finalize_planner_rejects_a_missing_task() {
        let root = TempRoot::new("missing");
        let task_dir = root.path().join("tasks").join("nonexistent");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        let err = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &commit_schema(),
            "nonexistent",
            &schemas(),
        )
        .expect_err("a missing task aborts preflight");
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "finalize.no-task");
        assert_eq!(err[0].severity, Severity::Blocking);
        assert_eq!(
            target(&err[0]),
            Some("task:nonexistent"),
            "the absent-area block keys at the WORK UNIT, not at the path it looked in",
        );
    }

    /// The finalize-promote **clobber guard** (review S1, `design/auto-migration.md` →
    /// Honest bounds → the data-loss clobber guard): a **Created**-provenance staged doc
    /// whose canonical destination already holds a committed managed doc **blocks** with
    /// exactly one `finalize.promote-clobber` — never silently overwriting it. This is the
    /// title-slug-collision / off-canonical-squatter data-loss case (a 2nd ADR slugging to
    /// an already-committed slug).
    #[test]
    fn finalize_plan_blocks_a_created_doc_that_clobbers_a_committed_doc() {
        let root = TempRoot::new("clobber");
        let task_dir = root.path().join("tasks").join("migrate-adr-second");
        let schema = stage_filled_commit(&task_dir, "migrate-adr-second");
        // A doc minted in THIS task (Created provenance) targeting `decisions/<slug>.md`.
        stage_filled_adr(&task_dir, "single-node-cache");
        state::record_doc_provenance(
            &task_dir,
            "adr:single-node-cache",
            state::Provenance::Created,
        )
        .expect("record created provenance");
        // A managed doc is ALREADY committed at that canonical destination.
        state::persist(
            &root.path().join("decisions").join("single-node-cache.md"),
            b"# a different decision already committed here\n",
        )
        .expect("commit a prior managed doc at the destination");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        let findings = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "migrate-adr-second",
            &schemas(),
        )
        .expect_err("a created doc clobbering a committed managed doc must block");
        assert_eq!(findings.len(), 1, "exactly one clobber finding");
        assert_eq!(findings[0].code, "finalize.promote-clobber");
        assert_eq!(findings[0].severity, Severity::Blocking);
        assert_eq!(
            target(&findings[0]),
            Some("decisions/single-node-cache.md"),
            "the clobber refusal keys at the destination FILE it refused to overwrite",
        );
        assert!(
            findings[0]
                .message
                .contains("decisions/single-node-cache.md"),
            "the block names the destination it refused to clobber: {:?}",
            findings[0].message
        );
    }

    /// Regression: an **EditedFromBase** staged doc whose destination already holds a
    /// committed file does **not** block — it was copied in from the committed store at
    /// base and edited, so re-promoting it over its own canonical path is the intended
    /// copy-on-first-touch update (the NGT committed-field-update path), never a clobber.
    /// `Provenance::Created` gating is the load-bearing regression hinge.
    #[test]
    fn finalize_plan_allows_an_edited_from_base_doc_over_a_committed_doc() {
        let root = TempRoot::new("clobber-edited");
        let task_dir = root.path().join("tasks").join("update-adr");
        let schema = stage_filled_commit(&task_dir, "update-adr");
        stage_filled_adr(&task_dir, "single-node-cache");
        // Copy-on-first-touch: the doc existed at base and was copied in for editing.
        state::record_doc_provenance(
            &task_dir,
            "adr:single-node-cache",
            state::Provenance::EditedFromBase,
        )
        .expect("record edited-from-base provenance");
        state::persist(
            &root.path().join("decisions").join("single-node-cache.md"),
            b"# the committed base of this very doc\n",
        )
        .expect("commit the base doc at the destination");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "update-adr",
            &schemas(),
        )
        .expect("an edited-from-base re-promote does not clobber");
        assert_eq!(plan.promotions.len(), 1, "the edited doc still promotes");
        assert_eq!(
            plan.promotions[0].destination,
            "decisions/single-node-cache.md"
        );
    }

    /// Regression: a **Created** doc whose canonical destination is **absent** does not
    /// block — the normal first-time migration / fresh-mint promote (nothing to clobber).
    #[test]
    fn finalize_plan_allows_a_created_doc_to_an_absent_destination() {
        let root = TempRoot::new("clobber-absent");
        let task_dir = root.path().join("tasks").join("migrate-adr-first");
        let schema = stage_filled_commit(&task_dir, "migrate-adr-first");
        stage_filled_adr(&task_dir, "single-node-cache");
        state::record_doc_provenance(
            &task_dir,
            "adr:single-node-cache",
            state::Provenance::Created,
        )
        .expect("record created provenance");
        // No committed file at `decisions/single-node-cache.md` → first-time promote.
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "migrate-adr-first",
            &schemas(),
        )
        .expect("a first-time created promote does not clobber");
        assert_eq!(plan.promotions.len(), 1);
        assert_eq!(
            plan.promotions[0].destination,
            "decisions/single-node-cache.md"
        );
    }

    /// Regression (the M24 **singleton** in-location squatter, `auto-migration.md` →
    /// Path-collision guard / Hardening #8): a **Created** doc of a **`singleton`** doctype
    /// whose destination already holds a committed file does **not** block when the
    /// migration's recorded `source-path` IS that destination — the committed file is the
    /// very foreign original being rewritten in place (the M24 blank-seed path), not a
    /// distinct managed doc. Since M43 (fork 5) the exclusion is doctype-blind — the
    /// singleton is one instance of the same-path carve-out — and matches the retire
    /// guard's source-path == destination discriminator; a redundantly-spelled source-path
    /// (`./`-prefixed) still matches. Without this exclusion the M24 changelog-squatter
    /// rewrite (flow 26) regresses.
    #[test]
    fn finalize_plan_allows_the_in_place_singleton_rewrite() {
        let root = TempRoot::new("clobber-in-place");
        let task_dir = root.path().join("tasks").join("migrate-in-place");
        let schema = stage_filled_commit(&task_dir, "migrate-in-place");
        // A staged singleton instance (fixed slug = type id) → the literal root
        // `CHANGELOG.md` (changelog is a `placement` doctype at schema v2).
        state::persist(
            &state::instance_path(&task_dir, "changelog", "changelog"),
            b"# Changelog\n\nblank-seeded then authored in place\n",
        )
        .expect("stage the singleton instance");
        state::record_doc_provenance(&task_dir, "changelog:changelog", state::Provenance::Created)
            .expect("record created provenance (blank-seeded squatter)");
        // The committed file at the canonical destination IS the foreign original.
        state::persist(
            &root.path().join("CHANGELOG.md"),
            b"non-conformant foreign squatter at the canonical path\n",
        )
        .expect("commit the foreign squatter at the destination");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        for source_path in ["CHANGELOG.md", "./CHANGELOG.md"] {
            state::persist(&task_dir.join("source-path"), source_path.as_bytes())
                .expect("record the in-place migration source path");
            let plan = plan_finalize(
                &task_dir,
                root.path(),
                &base(),
                &base().sha,
                &clean,
                true,
                &schema,
                "migrate-in-place",
                &schemas(),
            )
            .unwrap_or_else(|_| panic!("the in-place rewrite `{source_path}` must not clobber"));
            assert_eq!(
                plan.promotions[0].destination, "CHANGELOG.md",
                "the singleton squatter is rewritten in place (source-path `{source_path}`)",
            );
        }
    }

    /// The M43 same-path carve-out (fork 5, `DECISIONS.md` → 2026-07-16 M43 Settle #5):
    /// a **non-singleton** (adr) Created doc whose recorded `source-path` IS its own
    /// promote destination — the same-path migration — is the in-place rewrite for
    /// **any** doctype, not a clobber. The plan carries the promotion, **no** clobber
    /// finding, **no** retirement ([`plan_retirements`] skips source == destination), so
    /// the two compose to an in-place `M`-not-`D+A` landing; `--approve` stays the sole
    /// destructive gate downstream. A redundantly-spelled source-path (`./`-prefixed)
    /// still matches.
    #[test]
    fn finalize_plan_allows_the_non_singleton_in_place_rewrite() {
        let root = TempRoot::new("clobber-nonsingleton-inplace");
        let task_dir = root.path().join("tasks").join("migrate-adr-inplace");
        let schema = stage_filled_commit(&task_dir, "migrate-adr-inplace");
        stage_filled_adr(&task_dir, "single-node-cache");
        state::record_doc_provenance(
            &task_dir,
            "adr:single-node-cache",
            state::Provenance::Created,
        )
        .expect("record created provenance");
        // The committed file at the canonical destination IS the foreign original.
        state::persist(
            &root.path().join("decisions").join("single-node-cache.md"),
            b"non-conformant foreign adr at the canonical path\n",
        )
        .expect("commit the foreign original at the destination");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        for source_path in [
            "decisions/single-node-cache.md",
            "./decisions/single-node-cache.md",
        ] {
            state::persist(&task_dir.join("source-path"), source_path.as_bytes())
                .expect("record the same-path migration source path");
            let plan = plan_finalize(
                &task_dir,
                root.path(),
                &base(),
                &base().sha,
                &clean,
                true,
                &schema,
                "migrate-adr-inplace",
                &schemas(),
            )
            .unwrap_or_else(|findings| {
                panic!("the same-path rewrite `{source_path}` must not clobber: {findings:?}")
            });
            assert_eq!(
                plan.promotions.len(),
                1,
                "the plan carries the in-place promotion (source-path `{source_path}`)",
            );
            assert_eq!(
                plan.promotions[0].destination, "decisions/single-node-cache.md",
                "the non-singleton squatter is rewritten in place (source-path `{source_path}`)",
            );
            assert!(
                plan.retirements.is_empty(),
                "the same-path rewrite retires nothing — the promote IS the replacement \
                 (source-path `{source_path}`)",
            );
        }
    }

    /// GOLDEN: a task that staged both a commit doc and an `adr:single-node-cache`
    /// yields a plan whose promote set names exactly the ADR — copied to its canonical
    /// `decisions/single-node-cache.md` — with the same path keying a `hash_updates`
    /// blake3 entry. The commit doc (transient, location-less) appears in **neither**
    /// (its sink is the rendered git message). `finalize.md` → 4. Promote / 7. Post-commit.
    #[test]
    fn finalize_plan_promotes_a_staged_adr() {
        let root = TempRoot::new("promote");
        let task_dir = root.path().join("tasks").join("cache-sessions");
        let schema = stage_filled_commit(&task_dir, "cache-sessions");
        let adr_bytes = stage_filled_adr(&task_dir, "single-node-cache");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "cache-sessions",
            &schemas(),
        )
        .expect("a clean task with a staged ADR yields a plan");

        // The promote set names exactly the ADR, copied to its canonical path; the
        // commit doc is excluded (transient sink).
        assert_eq!(plan.promotions.len(), 1, "exactly the one persisted ADR");
        assert_eq!(
            plan.promotions[0].destination,
            "decisions/single-node-cache.md"
        );
        assert_eq!(
            plan.promotions[0].source,
            task_dir.join("docs").join("adr:single-node-cache.md"),
            "the source is the staged instance",
        );

        // The hash-update set keys the canonical path to the blake3 of the staged bytes
        // (byte-stable copy), and the commit doc is absent from it.
        assert_eq!(plan.hash_updates.len(), 1, "one promoted doc -> one hash");
        assert_eq!(
            plan.hash_updates.get("decisions/single-node-cache.md"),
            Some(&hash_bytes(&adr_bytes)),
            "the hash is over the staged ADR bytes",
        );
        assert!(
            !plan.hash_updates.keys().any(|k| k.contains("commit")),
            "the commit doc is never promoted",
        );

        // Freeze the plan's promote-relevant shape (source path normalized to the
        // working-area-relative tail, which is stable across temp roots).
        let promotions: Vec<(String, String)> = plan
            .promotions
            .iter()
            .map(|p| {
                let tail = p
                    .source
                    .strip_prefix(&task_dir)
                    .expect("source under the task dir")
                    .to_string_lossy()
                    .replace('\\', "/");
                (tail, p.destination.clone())
            })
            .collect();
        insta::assert_debug_snapshot!(
            (promotions, &plan.hash_updates),
            @r#"
        (
            [
                (
                    "docs/adr:single-node-cache.md",
                    "decisions/single-node-cache.md",
                ),
            ],
            {
                "decisions/single-node-cache.md": "eb5e3b6151ba4edaa8b9e507c61628d029a6018975dd765bc15a949e9742dad1",
            },
        )
        "#
        );
    }

    /// (M38 inc-1 T2) `plan_promotions` carries a staged **placement** doctype to its
    /// literal `placement.file` — case-preserved, bypassing docs-root and the
    /// `<location>/<slug>.md` join — for both a root `FOO.md` and a `docs/bar.md`. The
    /// destination is the literal file and it keys the `hash_updates` entry to the
    /// blake3 of the staged bytes (same Promotion/hash shape as a `location:` doctype).
    /// (`design/storage.md` → Placement — census site `plan_promotions`.)
    #[test]
    fn plan_promotions_places_a_placement_doctype_at_its_literal_file() {
        let root = TempRoot::new("placement-promote");
        let task_dir = root.path().join("tasks").join("author-pages");

        // Two placement doctypes: a root `FOO.md` and a direct `docs/bar.md`.
        let root_schema = placement_schema("foo", "FOO.md");
        let docs_schema = placement_schema("bar", "docs/bar.md");
        let foo_bytes = stage_filled_placement(&task_dir, &root_schema, &root_schema.ty);
        let bar_bytes = stage_filled_placement(&task_dir, &docs_schema, &docs_schema.ty);

        let mut with_placement: BTreeMap<String, Schema> = BTreeMap::new();
        with_placement.insert(root_schema.ty.clone(), root_schema.clone());
        with_placement.insert(docs_schema.ty.clone(), docs_schema.clone());

        let plan = plan_promotions(&task_dir, &with_placement).expect("placement promote plan");

        // Destinations are the literal `placement.file` paths, case-preserved, no docs-root
        // prefix; sorted by destination (`FOO.md` < `docs/bar.md` — uppercase sorts first).
        let dests: Vec<&str> = plan
            .promotions
            .iter()
            .map(|p| p.destination.as_str())
            .collect();
        assert_eq!(dests, vec!["FOO.md", "docs/bar.md"]);

        // Each literal destination keys the blake3 of its staged bytes (byte-stable copy).
        assert_eq!(
            plan.hash_updates.len(),
            2,
            "two placement docs -> two hashes"
        );
        assert_eq!(
            plan.hash_updates.get("FOO.md"),
            Some(&hash_bytes(&foo_bytes)),
            "the hash is over the staged FOO.md bytes",
        );
        assert_eq!(
            plan.hash_updates.get("docs/bar.md"),
            Some(&hash_bytes(&bar_bytes)),
            "the hash is over the staged docs/bar.md bytes",
        );
        // The source is the staged instance under the working area.
        assert_eq!(
            plan.promotions[0].source,
            task_dir.join("docs").join("foo:foo.md"),
        );
    }

    /// (M38 inc-1 T2) A **placement** doctype rides the milestone-join promote path —
    /// the shared `plan_promotions` inherits the literal-file branch, so the join
    /// produces a plan promoting the instance to its literal `placement.file`.
    #[test]
    fn milestone_finalize_promotes_a_placement_doctype() {
        let root = TempRoot::new("milestone-placement");
        let staging = root
            .path()
            .join("milestones")
            .join("page-rework")
            .join("merged");
        let schema = placement_schema("foo", "FOO.md");
        let bytes = stage_filled_placement(&staging, &schema, &schema.ty);

        let mut with_placement = schemas();
        with_placement.insert(schema.ty.clone(), schema.clone());

        let plan = plan_milestone_finalize(
            "page-rework",
            &staging,
            &base(),
            &base().sha,
            false,
            "Finalize milestone page-rework (1 sub-task)\n".to_string(),
            true,
            &with_placement,
        )
        .expect("a placement doctype rides the milestone join (placement != root-render)");

        assert_eq!(plan.promotions.len(), 1, "exactly the one placement doc");
        assert_eq!(
            plan.promotions[0].destination, "FOO.md",
            "promoted to its literal placement.file, not blocked as a root render",
        );
        assert_eq!(
            plan.hash_updates.get("FOO.md"),
            Some(&hash_bytes(&bytes)),
            "the join path keys the literal destination to the staged bytes' hash",
        );
    }

    /// Stage a filled `commit:<sub-id>` doc in a sub-task working area
    /// `<tasks_root>/<sub-id>/docs/` (a `feat` type + a per-sub-task summary), the
    /// authored commit doc a fanned-out sub-task produces. Returns the slug for
    /// convenience.
    fn stage_subtask_commit(tasks_root: &Path, sub_id: &str, summary: &str) {
        use crate::field_block::Value;
        use crate::write::SectionContent;

        let schema = commit_schema();
        let instance = write::Instance {
            title: sub_id.to_string(),
            sections: vec![
                SectionContent {
                    id: "header".to_string(),
                    fields: vec![crate::field_block::Field {
                        key: "type".to_string(),
                        value: Value::Scalar("feat".to_string()),
                    }],
                    ..Default::default()
                },
                SectionContent {
                    id: "summary".to_string(),
                    slot: Some(summary.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    ..Default::default()
                },
                SectionContent {
                    id: "trailers".to_string(),
                    ..Default::default()
                },
            ],
        };
        let bytes = write::render(&schema, &instance);
        let task_dir = tasks_root.join(sub_id);
        let path = state::instance_path(&task_dir, &schema.ty, sub_id);
        state::persist(&path, bytes.as_bytes()).expect("persist staged sub-task commit");
    }

    /// `squash: false` renders **one** authored commit message per sub-task, in the
    /// **given (id-sorted) order**, each read off that sub-task's own `commit:<sub-id>`
    /// doc via the shared `render_commit_message`. The rendered *sequence* is a pure
    /// function of the id *set* — feeding the ids in id-order vs its REVERSE yields the
    /// **byte-identical** message list (Validation hardening #7), because the caller
    /// passes id-sorted ids and the renderer preserves that order. A sub-task missing
    /// its authored commit doc is a blocking finding (the render precedent).
    #[test]
    fn render_subtask_messages_is_id_ordered_and_order_invariant() {
        let root = TempRoot::new("subtask-messages");
        let tasks_root = root.path().join("tasks");
        // Two sub-tasks authored distinct commit prose. id-sorted: [area-low, area-zed].
        stage_subtask_commit(&tasks_root, "area-zed", "rework the zed cache path");
        stage_subtask_commit(&tasks_root, "area-low", "rework the low cache path");

        let schema = commit_schema();
        let id_sorted = vec!["area-low".to_string(), "area-zed".to_string()];
        let messages = render_subtask_messages(&id_sorted, &tasks_root, &schema)
            .expect("both sub-tasks authored a commit doc");
        assert_eq!(
            messages,
            vec![
                "feat: rework the low cache path".to_string(),
                "feat: rework the zed cache path".to_string(),
            ],
            "one rendered authored message per sub-task, in id-sorted order",
        );

        // Order-invariance: the REVERSED id list still renders the SAME (id-keyed) bodies
        // in the SAME positions the caller feeds — the caller (TaskList::enumerate) sorts,
        // so the committed sequence is byte-identical across feed orders.
        let reversed = vec!["area-zed".to_string(), "area-low".to_string()];
        let rev_messages = render_subtask_messages(&reversed, &tasks_root, &schema)
            .expect("the reversed feed renders");
        // Re-sorting the reversed feed reproduces the id-sorted sequence byte-for-byte.
        let mut paired: Vec<(String, String)> = reversed.into_iter().zip(rev_messages).collect();
        paired.sort_by(|a, b| a.0.cmp(&b.0));
        let resorted: Vec<String> = paired.into_iter().map(|(_, m)| m).collect();
        assert_eq!(
            messages, resorted,
            "the rendered messages are id-keyed — re-sorting any feed order reproduces the \
             byte-identical id-sorted sequence",
        );

        // A sub-task missing its authored commit doc is a blocking render finding.
        let missing = render_subtask_messages(
            &["area-low".to_string(), "no-such-task".to_string()],
            &tasks_root,
            &schema,
        )
        .expect_err("a sub-task with no authored commit doc blocks");
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].code, "finalize.render-io");
        assert_eq!(missing[0].severity, Severity::Blocking);
    }

    /// The **milestone** planner ([`plan_milestone_finalize`]) is the thin sibling
    /// of [`plan_finalize`] for the fan-out single-commit boundary: it shares the
    /// preflight (staging area exists, base == HEAD), the empty-commit guard, and the
    /// phase-4 promote/phase-7 hash sweep, but **substitutes** the caller-supplied
    /// pre-rendered (T1-synthesized) message for the commit-doc render a milestone has
    /// no doc for (planner-note (b)), and adjudicates no validate report (the join's
    /// findings — a same-doc clash — are blocked CLI-side before materialize, per
    /// planner-note (c)).
    #[test]
    fn milestone_finalize_planner_shares_promote_substitutes_message_and_aborts() {
        let root = TempRoot::new("milestone-finalize");
        // The staging area is the materialized `merged/` dir whose `docs/` holds the
        // suffix-resolved bodies — the same staging form `plan_promotions` already reads.
        let staging = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged");
        let adr_bytes = stage_filled_adr(&staging, "cache-strategy");
        let message = "Finalize milestone cache-rework (2 sub-tasks)\n\n- area-low\n- area-zed\n";

        // (Preflight) base != supplied HEAD, and NOT a record-only advance → the same
        // divergence-routing CODE, no plan (its route is the milestone arm's — asserted in
        // `base_mismatch_route_is_unit_aware`).
        let err = plan_milestone_finalize(
            "cache-rework",
            &staging,
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            false,
            message.to_string(),
            true,
            &schemas(),
        )
        .expect_err("a base mismatch aborts preflight");
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "finalize.base-mismatch");

        // (Preflight) base != supplied HEAD, but the CLI verified `base..HEAD` is a
        // record-only linear-ancestor range (`record_only_advance = true`) → the base advances
        // and the preflight proceeds (the finalize base-guard refinement). The milestone's own
        // record-only bookkeeping is tolerated; only external drift keeps the block above.
        let plan = plan_milestone_finalize(
            "cache-rework",
            &staging,
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            true,
            message.to_string(),
            true,
            &schemas(),
        )
        .expect("a record-only-range advance clears the base-guard and yields a plan");
        assert_eq!(
            plan.message, message,
            "the advanced-base plan still carries the synthesized message verbatim",
        );

        // (Preflight) a missing staging area aborts before anything.
        let err = plan_milestone_finalize(
            "none",
            &root.path().join("milestones").join("none").join("merged"),
            &base(),
            &base().sha,
            false,
            message.to_string(),
            true,
            &schemas(),
        )
        .expect_err("a missing staging area aborts preflight");
        assert_eq!(err[0].code, "finalize.no-task");
        assert_eq!(
            target(&err[0]),
            Some("milestone:none"),
            "the shared constructor keys at the MILESTONE — the caller's unit tells it apart",
        );

        // (Empty-commit guard) base matches but no diff → produced-no-diff abort.
        let err = plan_milestone_finalize(
            "cache-rework",
            &staging,
            &base(),
            &base().sha,
            false,
            message.to_string(),
            false,
            &schemas(),
        )
        .expect_err("an empty diff aborts");
        assert_eq!(err[0].code, "finalize.empty-commit");
        assert_eq!(
            target(&err[0]),
            Some("milestone:cache-rework"),
            "the milestone's empty-commit abort keys at the milestone",
        );

        // (Clean) the plan carries the SUBSTITUTED synthesized message verbatim (never a
        // commit-doc render) and the SHARED promote/hash set over the materialized doc.
        let plan = plan_milestone_finalize(
            "cache-rework",
            &staging,
            &base(),
            &base().sha,
            false,
            message.to_string(),
            true,
            &schemas(),
        )
        .expect("a clean milestone staging area yields a plan");
        assert_eq!(
            plan.message, message,
            "the milestone plan carries the synthesized message verbatim, not a commit-doc render",
        );
        assert_eq!(plan.promotions.len(), 1, "the materialized ADR is promoted");
        assert_eq!(
            plan.promotions[0].destination, "decisions/cache-strategy.md",
            "the materialized ADR lands at its canonical path",
        );
        assert_eq!(
            plan.promotions[0].source,
            staging.join("docs").join("adr:cache-strategy.md"),
            "the source is the materialized staging body (the shared promote sweep)",
        );
        assert_eq!(
            plan.hash_updates.get("decisions/cache-strategy.md"),
            Some(&hash_bytes(&adr_bytes)),
            "the hash is over the materialized body bytes (the shared phase-7 set)",
        );
    }

    /// The `finalize.base-mismatch` route is **unit-aware** (M42 T3 — `design/write-commands.md`
    /// → The `finalize.base-mismatch` route is unit-aware; `design/team-ready-state.md` → What
    /// the base-guard is for → The route — the actual defect). One code, two work-unit kinds,
    /// and until M42 one route: the **task** route (*"switch back to `<A>` or discard the task
    /// with `jigc task discard`"*) was served to a blocked **milestone** too — where
    /// `jigc task discard <milestone-id>` is a dead end (`no task '<id>'`, exit 1) that calls a
    /// milestone "the task", and where the only honest reading of `discard` is *throw the work
    /// away*.
    ///
    /// - The **task** arm stays byte-identical (its switch-back-or-discard route is correct for
    ///   a task, whose `discard` is a genuine, cheap exit).
    /// - The **milestone** arm names the *milestone*, names the **cause** (commits landed after
    ///   this milestone's base, so the sub-task worktrees were cut from a base HEAD no longer
    ///   reflects and the combine cannot be proven sound against it), and names the two **real**
    ///   options — land the milestone first, or re-cut onto the new base — both honestly labelled
    ///   out-of-band git (there is no `jigc milestone rebase` at M42, deliberately). It **never**
    ///   offers `discard`: that settles the record of an *abandoned* milestone, so routing a
    ///   still-wanted milestone there tells the operator to destroy work to satisfy a guard.
    #[test]
    fn base_mismatch_route_is_unit_aware() {
        let root = TempRoot::new("unit-aware-route");
        let head = "ffffffffffffffffffffffffffffffffffffffff";

        // The task arm — unchanged: it names the task and offers `jigc task discard`.
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        let schema = stage_filled_commit(&task_dir, "add-rate-limiter");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());
        let err = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            head,
            &clean,
            true,
            &schema,
            "add-rate-limiter",
            &schemas(),
        )
        .expect_err("a base mismatch aborts the task preflight");
        assert_eq!(err[0].code, "finalize.base-mismatch");
        assert_eq!(
            err[0].message,
            format!(
                "the task was started at base `{}` but HEAD is now `{head}`",
                base().short
            ),
            "the task arm's message is unchanged",
        );
        assert_eq!(
            err[0].route.as_deref(),
            Some(
                format!(
                    "switch back to `{}` or discard the task with `jigc task discard`",
                    base().short
                )
                .as_str()
            ),
            "the task arm's route is unchanged",
        );

        // The milestone arm — the same code, a different unit, a different route.
        let staging = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged");
        stage_filled_adr(&staging, "cache-strategy");
        let err = plan_milestone_finalize(
            "cache-rework",
            &staging,
            &base(),
            head,
            false,
            "Finalize milestone cache-rework (1 sub-task)\n".to_string(),
            true,
            &schemas(),
        )
        .expect_err("a base mismatch aborts the milestone preflight");
        assert_eq!(err.len(), 1, "one preflight finding");
        assert_eq!(err[0].code, "finalize.base-mismatch");
        assert_eq!(err[0].severity, Severity::Blocking);
        assert_eq!(
            target(&err[0]),
            Some("milestone:cache-rework"),
            "same code, a different unit — and a different KEY: the milestone's own ref",
        );

        let message = &err[0].message;
        assert!(
            message.contains("milestone") && !message.contains("the task"),
            "the milestone arm names the milestone, never \"the task\": {message:?}",
        );
        assert!(
            message.contains("HEAD is now") && message.contains(&base().short),
            "the milestone arm still names the pinned base and the advanced HEAD: {message:?}",
        );
        assert!(
            message.contains("worktree"),
            "the milestone arm names the CAUSE — the worktrees were cut from the pinned base: \
             {message:?}",
        );

        let route = err[0].route.as_deref().expect("the block carries a route");
        assert!(
            !route.contains("discard"),
            "`discard` is never the exit from a base-mismatched milestone you still want: \
             {route:?}",
        );
        assert!(
            route.contains("jigc milestone finalize") && route.contains("re-cut"),
            "the route names BOTH real options — land the milestone first, or re-cut onto the \
             new base: {route:?}",
        );
        assert!(
            route.contains("out-of-band git"),
            "both options are honestly labelled out-of-band git (there is no verb for either): \
             {route:?}",
        );
    }

    /// An unmoved base (`base == HEAD`) is **no-divergence** — the pin holds and
    /// the decision never blocks, whatever the task's dirty footprint looks like.
    #[test]
    fn repin_unmoved_base_is_no_divergence() {
        let root = TempRoot::new("repin-unmoved");
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        stage_filled_adr(&task_dir, "keep-sessions-in-memory");

        let decision = decide_base_repin(
            "add-rate-limiter",
            &task_dir,
            &base(),
            &base().sha,
            &[],
            &["src/limiter.rs".to_string()],
            &schemas(),
        )
        .expect("an unmoved base never blocks");
        assert_eq!(decision, RepinDecision::NoDivergence);
    }

    /// The phase-1 amendment (`finalize.md` → Parallel hand-editing, 2026-06-12):
    /// a base moved on history **disjoint** from the task's work — the changed
    /// paths touch neither a dirty working-tree path nor a staged doc's promote
    /// destination — **re-pins** instead of blocking.
    #[test]
    fn repin_disjoint_moved_history_repins() {
        let root = TempRoot::new("repin-disjoint");
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        stage_filled_adr(&task_dir, "keep-sessions-in-memory");

        let decision = decide_base_repin(
            "add-rate-limiter",
            &task_dir,
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            &["other.txt".to_string(), "README.md".to_string()],
            &["src/limiter.rs".to_string()],
            &schemas(),
        )
        .expect("disjoint moved history re-pins, never blocks");
        assert_eq!(decision, RepinDecision::Repin);
    }

    /// A moved-history path that is also a **dirty working-tree path** keeps the
    /// block — the parallel-hand-editing case — and the finding **names the
    /// overlapping paths** (sorted: a pure function of the path *set*, byte-identical
    /// across feed orders) plus the resolve-or-discard route.
    #[test]
    fn repin_dirty_path_overlap_blocks_naming_the_path() {
        let root = TempRoot::new("repin-dirty-overlap");
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        stage_filled_adr(&task_dir, "keep-sessions-in-memory");

        let changed = [
            "src/limiter.rs".to_string(),
            "other.txt".to_string(),
            "src/auth.rs".to_string(),
        ];
        let dirty = ["src/auth.rs".to_string(), "src/limiter.rs".to_string()];
        let err = decide_base_repin(
            "add-rate-limiter",
            &task_dir,
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            &changed,
            &dirty,
            &schemas(),
        )
        .expect_err("a dirty-path overlap keeps the block");
        assert_eq!(err.len(), 1, "one overlap finding");
        assert_eq!(err[0].code, "finalize.base-mismatch");
        assert_eq!(err[0].severity, Severity::Blocking);
        assert!(
            err[0].message.contains("src/limiter.rs") && err[0].message.contains("src/auth.rs"),
            "the block names every overlapping path: {:?}",
            err[0].message
        );
        assert!(
            !err[0].message.contains("other.txt"),
            "a non-overlapping moved path is not named: {:?}",
            err[0].message
        );
        let route = err[0].route.as_deref().expect("the block carries a route");
        assert!(
            route.contains("jigc task discard"),
            "the route offers the discard half of resolve-or-discard: {route:?}"
        );

        // Order-invariance: the same path SETS fed in reversed order yield the
        // byte-identical finding — the naming is a function of the set, not the feed.
        let mut changed_rev = changed.to_vec();
        changed_rev.reverse();
        let mut dirty_rev = dirty.to_vec();
        dirty_rev.reverse();
        let err_rev = decide_base_repin(
            "add-rate-limiter",
            &task_dir,
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            &changed_rev,
            &dirty_rev,
            &schemas(),
        )
        .expect_err("the reversed feed blocks identically");
        assert_eq!(
            err, err_rev,
            "the finding is byte-identical across feed orders"
        );
    }

    /// A moved-history path that collides with a staged doc's **promote
    /// destination** (the phase-4 sweep's canonical `<location>/<slug>.md`) also
    /// keeps the block — the human's commit already touched where the task's doc
    /// will land.
    #[test]
    fn repin_promote_destination_overlap_blocks() {
        let root = TempRoot::new("repin-promote-overlap");
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        stage_filled_adr(&task_dir, "keep-sessions-in-memory");

        let err = decide_base_repin(
            "add-rate-limiter",
            &task_dir,
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            &[
                "other.txt".to_string(),
                "decisions/keep-sessions-in-memory.md".to_string(),
            ],
            &["src/limiter.rs".to_string()],
            &schemas(),
        )
        .expect_err("a promote-destination overlap keeps the block");
        assert_eq!(err.len(), 1, "one overlap finding");
        assert_eq!(err[0].code, "finalize.base-mismatch");
        assert!(
            err[0]
                .message
                .contains("decisions/keep-sessions-in-memory.md"),
            "the block names the colliding promote destination: {:?}",
            err[0].message
        );
    }

    /// Build a [`StagedSnapshot`] from literal `(path, blob)` entries + deletion paths.
    fn staged(entries: &[(&str, &str)], deletions: &[&str]) -> StagedSnapshot {
        let mut snapshot = StagedSnapshot::default();
        for (path, blob) in entries {
            snapshot.entries.insert(path.to_string(), blob.to_string());
        }
        for path in deletions {
            snapshot.deletions.insert(path.to_string());
        }
        snapshot
    }

    /// The carryover gate's core discrimination (`design/surface-contract.md` → The
    /// carryover gate): an index entry whose `(path, blob)` **both** match the
    /// pre-task snapshot is carried — one blocking `finalize.carried-staged` finding,
    /// keyed at the **file path** (the file-path target form), carrying a route that
    /// names both exits (unstage, or declare with `--carry-staged`). A path staged
    /// only *after* mint is the task's own work and never flagged.
    #[test]
    fn carryover_matching_entry_is_carried_and_routed() {
        let snapshot = staged(&[("src/foreign.rs", "aaaa1111")], &[]);
        let current = staged(
            &[("src/foreign.rs", "aaaa1111"), ("src/mine.rs", "bbbb2222")],
            &[],
        );
        let findings = decide_carryover(Some(&snapshot), &current, None, CarryoverBoundary::Task);
        assert_eq!(findings.len(), 1, "one finding per carried path — only one");
        let finding = &findings[0];
        assert_eq!(finding.code, "finalize.carried-staged");
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.key().target.as_deref(),
            Some("src/foreign.rs"),
            "keyed at the carried file path (the file-path target form)",
        );
        assert!(
            finding.message.contains("src/foreign.rs"),
            "the message names the carried path: {:?}",
            finding.message
        );
        let route = finding.route.as_deref().expect("blocking ⇒ routed");
        assert!(
            route.contains("--carry-staged"),
            "the route names the declare-it-deliberate override: {route:?}"
        );
        assert!(
            route.contains("git restore --staged"),
            "the route names the unstage exit: {route:?}"
        );
        // Through the serialization seam: the route floor + declared-target asserts hold.
        serde_json::to_string(&crate::finding::Findings::from(findings))
            .expect("the carried-staged finding passes the finding-key seam");
    }

    /// The same path **restaged to a different blob** is the task's own work — the
    /// snapshot pair no longer matches, so nothing is carried. Likewise a snapshot
    /// entry the agent unstaged entirely, and a snapshot **deletion** since restored.
    #[test]
    fn carryover_restaged_or_cleared_paths_are_not_carried() {
        let snapshot = staged(
            &[("src/a.rs", "aaaa1111"), ("src/b.rs", "cccc3333")],
            &["gone.md"],
        );
        // a.rs restaged to new content; b.rs unstaged; gone.md's deletion restored.
        let current = staged(&[("src/a.rs", "dddd4444")], &[]);
        assert_eq!(
            decide_carryover(Some(&snapshot), &current, None, CarryoverBoundary::Task),
            Vec::new(),
            "a restaged / cleared / restored path is not a carryover",
        );
    }

    /// A snapshot **staged deletion still staged** at finalize is carried — the
    /// entry-blind A7 case: a pre-task `git rm` must not silently ride the commit.
    #[test]
    fn carryover_snapshot_deletion_still_staged_is_carried() {
        let snapshot = staged(&[], &["legacy/OLD.md"]);
        let current = staged(&[], &["legacy/OLD.md"]);
        let findings = decide_carryover(Some(&snapshot), &current, None, CarryoverBoundary::Task);
        assert_eq!(findings.len(), 1, "the staged deletion is carried");
        assert_eq!(findings[0].code, "finalize.carried-staged");
        assert_eq!(findings[0].key().target.as_deref(), Some("legacy/OLD.md"));
        assert!(
            findings[0].message.contains("deletion"),
            "the message says what is carried — a staged deletion: {:?}",
            findings[0].message
        );
    }

    /// The migration's **own retire pathspec is exempt** — that deletion is the
    /// task's — while every other carried path still blocks. The exempt path is
    /// compared lexically normalized (the recorded `source-path` precedent).
    #[test]
    fn carryover_retire_pathspec_is_exempt_others_still_block() {
        let snapshot = staged(&[("notes.md", "aaaa1111")], &["legacy/CHANGES.md"]);
        let current = staged(&[("notes.md", "aaaa1111")], &["legacy/CHANGES.md"]);
        let findings = decide_carryover(
            Some(&snapshot),
            &current,
            Some("./legacy/CHANGES.md"),
            CarryoverBoundary::Task,
        );
        assert_eq!(
            findings.len(),
            1,
            "the retire path is exempt; the entry is not"
        );
        assert_eq!(findings[0].key().target.as_deref(), Some("notes.md"));
    }

    /// **Missing snapshot ⇒ fail-open** (the declared bound: a task minted pre-M43
    /// finalizes as today) — and, distinctly, an **empty** snapshot carries nothing
    /// (everything staged since mint is the task's own).
    #[test]
    fn carryover_missing_snapshot_fails_open_empty_snapshot_carries_nothing() {
        let current = staged(&[("src/foreign.rs", "aaaa1111")], &["legacy/OLD.md"]);
        assert_eq!(
            decide_carryover(None, &current, None, CarryoverBoundary::Task),
            Vec::new(),
            "no snapshot (pre-M43 mint) ⇒ fail-open, no findings",
        );
        assert_eq!(
            decide_carryover(
                Some(&StagedSnapshot::default()),
                &current,
                None,
                CarryoverBoundary::Task
            ),
            Vec::new(),
            "an empty snapshot (clean index at mint) carries nothing",
        );
    }

    /// The finding list is **sorted by path across both halves** (entries ∪
    /// deletions) — a pure function of the carried *set*, byte-identical however the
    /// input sets were built (Validation hardening #7: the sets are fed here in
    /// id-order and in reverse, asserting identical output).
    #[test]
    fn carryover_findings_sort_by_path_and_are_order_invariant() {
        let entries = [("b.md", "aaaa1111"), ("d.md", "cccc3333")];
        let deletions = ["a.md", "c.md"];
        let snapshot = staged(&entries, &deletions);
        let current = staged(&entries, &deletions);
        let findings = decide_carryover(Some(&snapshot), &current, None, CarryoverBoundary::Task);
        let targets: Vec<Option<String>> = findings.iter().map(|f| f.key().target).collect();
        assert_eq!(
            targets,
            ["a.md", "b.md", "c.md", "d.md"].map(|p| Some(p.to_string())),
            "one finding per carried path, path-sorted across entries and deletions",
        );

        // Divergent build order: the same sets inserted in reverse yield the
        // byte-identical finding list.
        let mut entries_rev = entries;
        entries_rev.reverse();
        let mut deletions_rev = deletions;
        deletions_rev.reverse();
        let snapshot_rev = staged(&entries_rev, &deletions_rev);
        let current_rev = staged(&entries_rev, &deletions_rev);
        assert_eq!(
            findings,
            decide_carryover(
                Some(&snapshot_rev),
                &current_rev,
                None,
                CarryoverBoundary::Task
            ),
            "the decision is a function of the sets — identical across build orders",
        );
    }

    /// The **milestone boundary's wording is law-1 honest** (`design/surface-contract.md`
    /// → The carryover gate; the T4 honest-wording bound): the aggregate commit is built
    /// from the sub-task worktrees over targeted pathspecs, so a live-index entry
    /// structurally cannot ride it — the finding says the entry **stays staged** across
    /// the boundary and never claims it would ride the commit. The task boundary keeps
    /// its whole-index-commit truth.
    #[test]
    fn carryover_milestone_boundary_wording_says_stays_staged_never_ride() {
        let snapshot = staged(&[("foreign.txt", "aaaa1111")], &["gone.md"]);
        let current = staged(&[("foreign.txt", "aaaa1111")], &["gone.md"]);
        let findings = decide_carryover(
            Some(&snapshot),
            &current,
            None,
            CarryoverBoundary::Milestone,
        );
        assert_eq!(findings.len(), 2, "both halves still carry — same decision");
        for finding in &findings {
            assert_eq!(finding.code, "finalize.carried-staged");
            assert!(
                finding.message.contains("milestone") && finding.message.contains("stays staged"),
                "the milestone message names the boundary and the stays-staged truth: {:?}",
                finding.message
            );
            let route = finding.route.as_deref().expect("blocking ⇒ routed");
            assert!(
                !finding.message.contains("ride") && !route.contains("ride"),
                "the milestone wording never claims the entry would ride the aggregate \
                 commit; message: {:?}\nroute: {route:?}",
                finding.message
            );
            assert!(
                route.contains("--carry-staged") && route.contains("git restore --staged"),
                "the route names both exits: {route:?}"
            );
        }
    }
}
