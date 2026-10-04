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
//! **A new `finalize.*` finding code joins one registry, wherever it is minted.**
//! The family is enumerated at `cli::render::FINALIZE_FAMILY` under a stated
//! predicate — *a code is a member iff a production constructor mints it as a
//! [`Finding`] in the `finalize.` namespace* — and
//! `crates/cli/tests/finalize_family_registry.rs` derives that set by scanning
//! production source, so a producer added here (or in `milestone`, or CLI-side)
//! that skips the registry reddens. Its size is deliberately stated in no
//! document: three homes carried three different wrong numerals for this family
//! before M49 Increment 11 / T11 replaced the counting with the table
//! (`design/command-output-contract.md` → The `finalize.*` family).
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
    /// `source-path` ([`crate::state::read_migration_source`]); **empty** on every
    /// non-migration task (the milestone sibling never sets it).
    pub retirements: Vec<PathBuf>,
    /// The **recorded owner-artifact paths** (`design/finalize.md` → 5. Stage; M45 Inc 8):
    /// every staged instance's `owned-location` field value, in staged-file then
    /// document order. An **in-process** field (this struct is never serialized to disk
    /// or the wire — the `--format json` contract is [`ValidationReport`], not
    /// `FinalizePlan`), so it carries no wire-contract weight. The CLI reads it to
    /// **exempt** these paths from the carryover gate (an agent stages the audit artifact
    /// before minting the recording task — the natural authoring order — so the recorded
    /// path is the task's own subject, never a foreign carry-over), and (T2) to **stage**
    /// them in-transaction. Empty when no staged instance carries an `owned-location`
    /// field (the omitting-context inert path).
    pub owner_artifacts: Vec<String>,
}

impl FinalizePlan {
    /// Build a plan over a rendered message, a promote set, a post-commit hash-update
    /// set, a retire set, and the recorded owner-artifact set, stamping the current
    /// [`SCHEMA_VERSION`].
    fn new(
        message: String,
        promotions: Vec<Promotion>,
        hash_updates: BTreeMap<String, String>,
        retirements: Vec<PathBuf>,
        owner_artifacts: Vec<String>,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            message,
            promotions,
            hash_updates,
            retirements,
            owner_artifacts,
        }
    }

    /// The **subject line** of the rendered [`message`] — everything up to its first
    /// newline, which is exactly what git will record as the commit's `%s` for a message
    /// whose body is separated by the blank line [`crate::write::render_commit_message`]
    /// always writes.
    ///
    /// Read by `jigc task finalize --dry-run` so the forecast names the subject the
    /// commit will carry (M50 Inc 12 / F-7). It is a **projection of the same render**,
    /// never a second composition: a dry-run-side re-spelling of `<type>(<scope>):
    /// <summary>` would print `feat(): …` for the optional scope the renderer omits.
    ///
    /// [`message`]: FinalizePlan::message
    pub fn subject(&self) -> &str {
        self.message.lines().next().unwrap_or("")
    }
}

/// **What git holds at the homes a plan promotes to** — the facts about a promote
/// destination that are not on the disk the planner stats, supplied by the CLI because the
/// engine does no git (the rc.24 fix pass, the completion audit's CPL-5;
/// `design/finalize.md` → 4. Promote, *what an occupied home is*).
///
/// The clobber guard asked one question of a home — *what is this directory entry?* — and
/// a committing door replaces more than directory entries. A path **git holds and the
/// worktree does not** read as a free home:
///
/// - a committed doc whose file was deleted from the worktree, uncommitted
///   ([`head`](Self::head)). A doc *minted* at that id then landed under it at exit 0, the
///   commit replacing the committed doc under its own identity — where a checkout holding a
///   `file-state` key for the doc blocked on `reconciliation.rename`, a checkout without one
///   (every fresh clone) confirmed nothing;
/// - a file staged and then taken out of the worktree ([`index`](Self::index)). The
///   promote's own `git add` replaced its index entry — bytes no commit holds.
///
/// Every member is a **repo-relative path as git prints it**, compared with a promotion's
/// [`lexical_normalize`](crate::store::lexical_normalize)d destination. The CLI asks git
/// itself — `git ls-tree`, `git ls-files` — over exactly the destinations
/// [`promote_destinations`] names, so the answer is git's under whatever conversion and
/// attribute settings the repository has: the questions are about which paths exist, never
/// about bytes. [`Default`] is *git holds none of them*, which is true of every plan whose
/// homes are new — and is what the engine's own tests, which have no repository, feed in.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HomeClaims {
    /// Promote destinations the commit the door builds on (`HEAD`) holds an entry at.
    pub head: BTreeSet<String>,
    /// Promote destinations the index of the checkout the promote writes into holds.
    pub index: BTreeSet<String>,
}

/// **The destinations the docs staged under `area_dir` promote to** — the promote plan's
/// own set, for the door that has to ask git about those paths *before* the planner runs
/// ([`HomeClaims`]). It is [`plan_promotions`] and nothing else, so the paths asked about
/// and the paths planned cannot differ. A staging area the sweep cannot read answers the
/// sweep's own blocking finding, which the planner then raises itself.
pub fn promote_destinations(
    area_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> Result<Vec<String>, Vec<Finding>> {
    Ok(plan_promotions(area_dir, schemas)?
        .promotions
        .into_iter()
        .map(|promotion| promotion.destination)
        .collect())
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
/// `claims` is what git holds at those destinations ([`HomeClaims`]): a home is occupied by
/// what the index or `HEAD` holds there as well as by what is on disk.
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
    claims: &HomeClaims,
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
    let source = std::fs::read_to_string(&commit_path).map_err(|err| {
        vec![render_io_finding(
            RenderSubject::Task { id: commit_slug },
            &commit_path,
            &err,
        )]
    })?;
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
    plan_clobber_guard(
        unit,
        task_dir,
        repo_root,
        &promote.promotions,
        schemas,
        claims,
    )?;

    // The retire set (`design/auto-migration.md` → Retire-the-foreign-original): a
    // migration task records its repo-relative foreign source path at mint; the planner
    // names it for retirement inside the commit closure. Empty on every non-migration
    // task (no `source-path`) — inert. The path-collision guard (→ Path-collision guard)
    // excludes a foreign source that equals a promote destination — the in-location
    // squatter the managed write rewrites in place is not a distinct original to retire.
    let retirements = plan_retirements(unit, task_dir, &promote.promotions)?;

    // The recorded owner-artifact set (M45 Inc 8): a second walk over the staged
    // instances' `owned-location` fields, mirroring the #5 presence gate's field walk but
    // *collecting* the recorded paths rather than adjudicating them — the CLI exempts them
    // from the carryover gate and (T2) stages them in-transaction. Best-effort: an
    // unparseable / typeless staged doc yields nothing, the gate's own skip-and-continue.
    let owner_artifacts = plan_owner_artifacts(task_dir, schemas);

    Ok(FinalizePlan::new(
        message,
        promote.promotions,
        promote.hash_updates,
        retirements,
        owner_artifacts,
    ))
}

/// Collect every staged instance's `owned-location` field value — the recorded
/// owner-artifact paths ([`FinalizePlan::owner_artifacts`]). Mirrors
/// [`crate::validate::owner_artifact_present`]'s section/field walk, but *collects* the
/// recorded path rather than adjudicating presence/safety: the carryover gate exempts
/// these paths and (M45 Inc 8 T2) finalize stages them in-transaction. A **best-effort**
/// read — an unparseable / typeless / location-less instance yields nothing, mirroring the
/// gate's skip-and-continue (an instance's own conformance is `conformance_for`'s concern,
/// surfaced through phase 2's report before the plan is produced). Repo-relative, trimmed,
/// non-empty values, in staged-file then document order.
///
/// **`pub` since M47 Inc 4**: the `task validate` carryover *preview*
/// ([`CarryoverBoundary::TaskPreview`]) needs the same owner-artifact exemption set the
/// committing door takes from [`FinalizePlan::owner_artifacts`], and it runs no planner —
/// so the exemption is computed from the one function rather than re-derived, and the two
/// doors cannot disagree about which paths are the task's own subject.
pub fn plan_owner_artifacts(task_dir: &Path, schemas: &BTreeMap<String, Schema>) -> Vec<String> {
    use crate::schema::{FieldType, SectionBody};

    let docs_dir = task_dir.join(DOCS_DIR);
    let Ok(entries) = std::fs::read_dir(&docs_dir) else {
        return Vec::new(); // no `docs/` dir means nothing was staged.
    };
    let mut staged: Vec<PathBuf> = entries
        .flatten()
        .filter(is_staged_shape)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .collect();
    staged.sort();

    let mut paths = Vec::new();
    for source_path in staged {
        let Some(stem) = source_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some((ty, _slug)) = stem.split_once(':') else {
            continue; // not a `<type>:<slug>` instance.
        };
        let Some(schema) = schemas.get(ty) else {
            continue;
        };
        let Ok(source) = std::fs::read_to_string(&source_path) else {
            continue;
        };
        let Ok(doc) = crate::parse::parse_sections(schema, &source) else {
            continue; // an unparseable instance is phase 2's concern, not this walk's.
        };
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
                    continue; // omitted: nothing recorded (the inert path).
                };
                if let crate::field_block::Value::Scalar(path) = &present.value {
                    let trimmed = path.trim();
                    if !trimmed.is_empty() {
                        paths.push(trimmed.to_string());
                    }
                }
            }
        }
    }
    paths
}

/// Read the migration task's recorded foreign source path
/// ([`crate::state::read_migration_source`]) into the retire set. A non-migration task has
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
    match crate::state::read_migration_source(task_dir) {
        Ok(Some(source)) => {
            // F1: no managed replacement was promoted — refuse to retire (block), never
            // delete the foreign original with nothing to take its place.
            if promotions.is_empty() {
                return Err(vec![migration_no_replacement_finding(source.recorded())]);
            }
            let foreign = source.normalized();
            if promotions
                .iter()
                .any(|p| crate::store::lexical_normalize(Path::new(&p.destination)) == foreign)
            {
                Ok(Vec::new()) // in-location squatter — rewritten in place, not retired.
            } else {
                Ok(vec![foreign])
            }
        }
        Ok(None) => Ok(Vec::new()),
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
/// then-`singleton`-gated blank-seed copy-in — which bounded every non-singleton
/// same-path migration out of end-to-end authoring. M43 inc-7 T1 then made the
/// create-side copy-in doctype-blind too: a create over a committed same-slug doc
/// copies in and records `EditedFromBase`, so it never reaches this guard — the
/// guard's reachable set narrows to **post-create drift**, a destination file that
/// appears only after the `Created` mint.)
///
/// Each staged promotion's `<type>:<slug>` address is recovered from its source file stem
/// (the [`plan_promotions`] naming convention) and looked up in the task's provenance
/// manifest ([`crate::state::ProvenanceRecord`]). A staged doc with no recorded
/// provenance (none was minted/copied-in here) is not create-provenance, so it never
/// trips the *occupied by a file* arm.
///
/// **The other arm asks no provenance at all** (the rc.24 fix pass, `(R6, D-7)`): a
/// destination that holds an entry which is **not a regular file** — a link, dangling or
/// live, a directory, a special file — refuses **every** promotion, minted or copied in,
/// carve-out or not ([`refused_promotions`]). A promote lands a regular file at exactly its
/// canonical path or the transaction refuses before anything is written; the provenance
/// only picks the exit the route hands back ([`ShapeExit`]).
fn plan_clobber_guard(
    unit: Unit,
    task_dir: &Path,
    repo_root: &Path,
    promotions: &[Promotion],
    schemas: &BTreeMap<String, Schema>,
    claims: &HomeClaims,
) -> Result<(), Vec<Finding>> {
    let provenance = crate::state::ProvenanceRecord::load(task_dir)
        .map_err(|err| vec![provenance_io_finding(unit, task_dir, &err)])?;
    // The migration's recorded foreign source path (if any) — kept as recorded for the
    // finding's naming, normalized for the in-place (source == destination) comparison.
    let source = crate::state::read_migration_source(task_dir)
        .map_err(|err| vec![source_path_io_finding(unit, task_dir, &err)])?;
    let in_place = source
        .as_ref()
        .map(crate::state::MigrationSource::normalized);
    let by = match source.as_ref() {
        Some(source) => ClobberedBy::Migration {
            source: source.recorded(),
        },
        None => ClobberedBy::Task,
    };
    let task_id = unit.id();

    let clobbers: Vec<Finding> = refused_promotions(
        repo_root,
        promotions,
        in_place.as_deref(),
        claims,
        |address| {
            // edited-from-base / unrecorded → never a clobber.
            provenance.get(address) == Some(crate::state::Provenance::Created)
        },
    )
    .into_iter()
    .map(|refused| {
        let exit = ShapeExit::of(provenance.get(refused.address), refused.address, schemas);
        let shape_unit = match source.as_ref() {
            Some(_) => ShapeUnit::Migration {
                id: task_id,
                address: refused.address,
            },
            None => ShapeUnit::Task {
                id: task_id,
                address: refused.address,
            },
        };
        let by = match refused.over {
            Occupant::File => by,
            Occupant::Foreign(shape) => ClobberedBy::Shape {
                shape,
                exit,
                unit: shape_unit,
            },
            Occupant::Held(holder) => ClobberedBy::Held {
                holder,
                exit,
                unit: shape_unit,
            },
        };
        clobber_finding(repo_root, &refused.promotion.destination, by)
    })
    .collect();
    if clobbers.is_empty() {
        Ok(())
    } else {
        Err(clobbers)
    }
}

/// **The clobber guard at the milestone commit boundary** — [`plan_clobber_guard`]'s
/// sibling for [`plan_milestone_finalize`], over the same predicate
/// ([`created_over_occupied`]) and the same finding ([`clobber_finding`]) (the rc.24 fix
/// pass, `(R6, D-1)`; `design/finalize.md` → 4. Promote).
///
/// The milestone planner ran the shared promote sweep and never the guard after it: the
/// guard arrived at M25 on the per-task planner alone, a week after the milestone planner
/// shipped, and nothing carried it across. So a sub-task's `created` doc landed on whatever
/// sat at its home at exit 0 with `findings` empty — a committed doc replaced under its own
/// id, an untracked file overwritten and left in no git object
/// (`completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R6-D-1.md`).
///
/// **What differs from the task arm is where the discriminator comes from, and nothing
/// else.** The merged staging area holds bodies only — no `provenance.json` — and a suffixed
/// body's final address is one no sub-area manifest names, so the join hands each
/// promotion's provenance on directly ([`crate::milestone::MergedOrigin`], keyed by final
/// address). A promotion with no origin is not a body the join wrote, and is not this
/// guard's subject — the task arm's *unrecorded → never a clobber*.
///
/// **It decides at the boundary, not at the join preview**, and that is the point: the
/// join's suffix is a pure function of the sub-area set (`design/storage.md` → The
/// by-task-id join, rule 4) and consults neither the committed store nor the worktree, and
/// an occupant can appear at a home between `jigc milestone join` and this call. Whatever
/// way a `created` doc came to face an occupied home — a suffix minted onto an id the store
/// already holds, or a file that turned up at its own id — the refusal is taken here, where
/// the write would happen.
///
/// There is **no in-place carve-out**: that is the migration rewrite's, a per-task verb, and
/// a milestone boundary retires nothing.
fn plan_milestone_clobber_guard(
    milestone_id: &str,
    repo_root: &Path,
    promotions: &[Promotion],
    origins: &BTreeMap<String, crate::milestone::MergedOrigin>,
    schemas: &BTreeMap<String, Schema>,
    claims: &HomeClaims,
) -> Result<(), Vec<Finding>> {
    let clobbers: Vec<Finding> =
        refused_promotions(repo_root, promotions, None, claims, |address| {
            origins
                .get(address)
                .is_some_and(|origin| origin.provenance == crate::state::Provenance::Created)
        })
        .into_iter()
        .filter_map(|refused| {
            let origin = origins.get(refused.address);
            // The exit the sub-task has, read off the address its doc is staged under.
            let exit = ShapeExit::of(
                origin.map(|origin| origin.provenance),
                origin.map_or(refused.address, |origin| origin.minted.as_str()),
                schemas,
            );
            let unit = ShapeUnit::Milestone {
                milestone: milestone_id,
                landing: refused.address,
                origin,
            };
            let by = match refused.over {
                // The file arm's subject is a body the join wrote — no origin, no clobber.
                Occupant::File => ClobberedBy::SubTask {
                    milestone: milestone_id,
                    landing: refused.address,
                    origin: origin?,
                },
                // The shape arm refuses whoever staged the body: with no origin there is no
                // sub-task to name, and the entry at the home is still not one to write
                // through.
                Occupant::Foreign(shape) => ClobberedBy::Shape { shape, exit, unit },
                // Asked of a `created` doc only, and `created` is read off an origin.
                Occupant::Held(holder) => ClobberedBy::Held { holder, exit, unit },
            };
            Some(clobber_finding(
                repo_root,
                &refused.promotion.destination,
                by,
            ))
        })
        .collect();
    if clobbers.is_empty() {
        Ok(())
    } else {
        Err(clobbers)
    }
}

/// **What a promotion would land on, when the planner refuses it** — the answer to the one
/// occupancy question both committing doors' clobber guards ask, over the checkout the
/// promote writes into.
///
/// It reads the **worktree**, which is where jigc's own notion of the committed store lives
/// (the create probe, [`crate::state::create_occupied`], and the in-task rename's
/// destination guard read the same place): a committed doc is a file at its home, and so is
/// an untracked one, which no reading of the index or of `HEAD` would see. An entry there
/// is an occupant whoever put it there — and it is read **without following a link**
/// ([`crate::store::home_entry`]), so what is asked is *what this directory entry is*,
/// never what it points at.
///
/// **And where the worktree shows nothing, it asks what git holds** ([`HomeClaims`]; the
/// completion audit's CPL-5). The worktree is where an occupant usually is, not the only
/// place one can be: a committed doc deleted from the worktree is still in `HEAD`, and a
/// file staged and then taken out of the worktree is still in the index — a merely-staged
/// file is seen above only while it is also on disk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Occupant {
    /// A regular file — refused only for a doc the unit **minted**.
    File,
    /// An entry that is not a regular file — refused for **every** promotion.
    Foreign(crate::store::ForeignEntry),
    /// **Nothing on disk, and a path git holds** ([`HomeClaims`]) — refused only for a doc
    /// the unit **minted**. The home reads free and is not: the commit would replace what
    /// `HEAD` holds there under its own path, or the promote's `git add` would replace an
    /// index entry no commit has.
    Held(Holder),
}

/// **Where git holds a file the worktree does not** — what [`Occupant::Held`] names.
/// `HEAD` wins when both do: a committed doc is the stronger claim, and the route that
/// brings it back (`git checkout HEAD -- <path>`) restores the index entry with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Holder {
    /// The commit the door builds on holds an entry at the path.
    Head,
    /// Only the index does — a file staged and never committed.
    Index,
}

/// One promotion the planner refuses: the staged `<type>:<slug>` address it promotes from
/// (the [`plan_promotions`] naming convention: the source file's stem), the promotion, and
/// what is at its destination.
struct RefusedPromotion<'p> {
    address: &'p str,
    promotion: &'p Promotion,
    over: Occupant,
}

/// **The promotions that may not be written** — the predicate [`plan_clobber_guard`] and
/// [`plan_milestone_clobber_guard`] share. Two arms over one observation of each home:
///
/// - a `created` doc over a **regular file** — the clobber the guard was built for.
///   `is_created` is each door's own answer to *"did this unit mint the doc?"* (the task
///   area's provenance manifest, or the join's per-body origin), and `in_place` is the
///   per-task migration carve-out's normalized source path (`None` at the milestone
///   boundary);
/// - **any** promotion over an entry that is **not a regular file** (the rc.24 fix pass,
///   `(R6, D-7)`). Neither discriminator applies: an `edited-from-base` doc was read
///   *through* a live link at copy-in and would be written through it, and the in-place
///   carve-out is a licence to rewrite a foreign *file*, not to write through whatever
///   stands at its path now;
/// - a `created` doc over a home with **nothing on disk that git still holds** (`claims`;
///   the completion audit's CPL-5). Same discriminators as the first arm, because it is the
///   first arm's question asked of the place the occupant actually is: an `edited-from-base`
///   doc is the committed doc's own update and re-promotes over its home whether or not the
///   file is in the worktree.
fn refused_promotions<'p>(
    repo_root: &Path,
    promotions: &'p [Promotion],
    in_place: Option<&Path>,
    claims: &HomeClaims,
    is_created: impl Fn(&str) -> bool,
) -> Vec<RefusedPromotion<'p>> {
    promotions
        .iter()
        .filter_map(|promotion| {
            let address = promotion.source.file_stem().and_then(|s| s.to_str())?;
            let dest_norm = crate::store::lexical_normalize(Path::new(&promotion.destination));
            // The two arms that ask *"did this unit mint the doc?"* share its answer, and
            // the in-place migration carve-out with it: a doc that replaces the very
            // foreign original it was migrated from (M43, fork 5) is not a clobber of it.
            let is_fresh_mint = || is_created(address) && in_place != Some(dest_norm.as_path());
            let over = match crate::store::home_entry(&repo_root.join(&promotion.destination)) {
                crate::store::HomeEntry::Foreign(shape) => Occupant::Foreign(shape),
                crate::store::HomeEntry::RegularFile => {
                    if !is_fresh_mint() {
                        return None;
                    }
                    Occupant::File
                }
                // Nothing on disk is not *nothing there*: what git holds at the path is an
                // occupant too ([`HomeClaims`]).
                crate::store::HomeEntry::Free => {
                    let git_path = dest_norm.to_string_lossy();
                    let holder = if claims.head.contains(git_path.as_ref()) {
                        Holder::Head
                    } else if claims.index.contains(git_path.as_ref()) {
                        Holder::Index
                    } else {
                        return None;
                    };
                    if !is_fresh_mint() {
                        return None;
                    }
                    Occupant::Held(holder)
                }
            };
            Some(RefusedPromotion {
                address,
                promotion,
                over,
            })
        })
        .collect()
}

/// **The exit a unit has from an entry at its doc's home that is not a regular file** —
/// what [`clobber_finding`]'s shape arm routes at. The entry is never jigc's to change, so
/// every exit is either a different home for the doc or the user's own act on the entry;
/// which of those *lands* depends on how the unit came to hold the doc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShapeExit {
    /// The unit **minted** the doc and its id is the author's to choose: re-slug it in the
    /// task onto a free home, or move the entry out of this one.
    RenameOrMoveOut,
    /// The unit minted the doc under a **fixed identity** (a `placement` / `display-title`
    /// singleton, whose in-task rename refuses outright), or nothing records how the doc
    /// was staged: the home has to be free.
    MoveOut,
    /// The unit **copied the doc in** — through a live link, the only entry of this class
    /// with a body — and edited it: the home has to hold the regular file itself, which the
    /// staged edit then lands over. A freed home would not do: the doc is `edited-from-base`.
    RegularFile,
}

impl ShapeExit {
    /// `provenance` is how the unit holds the doc staged at `address` (its `<type>:<slug>`
    /// in the area that staged it), `schemas` the resolved set its doctype is read from.
    fn of(
        provenance: Option<crate::state::Provenance>,
        address: &str,
        schemas: &BTreeMap<String, Schema>,
    ) -> Self {
        match provenance {
            Some(crate::state::Provenance::EditedFromBase) => ShapeExit::RegularFile,
            Some(crate::state::Provenance::Created) => {
                let fixed = address
                    .split_once(':')
                    .and_then(|(ty, _)| schemas.get(ty))
                    .is_none_or(|schema| schema.fixed_title().is_some());
                if fixed {
                    ShapeExit::MoveOut
                } else {
                    ShapeExit::RenameOrMoveOut
                }
            }
            None => ShapeExit::MoveOut,
        }
    }
}

/// **Which unit the shape refusal speaks to** — the work unit, and the address its doc is
/// staged under there (the one an in-task rename takes).
#[derive(Clone, Copy)]
enum ShapeUnit<'a> {
    /// An ordinary task.
    Task { id: &'a str, address: &'a str },
    /// A migration task — same exits, and the continuation ends at `--approve`.
    Migration { id: &'a str, address: &'a str },
    /// The milestone commit boundary. `origin` is the join's record of the body — `None`
    /// for a promotion the join did not write.
    Milestone {
        milestone: &'a str,
        landing: &'a str,
        origin: Option<&'a crate::milestone::MergedOrigin>,
    },
    /// The promote **sink** — the write itself, re-asking one statement before it copies.
    /// It knows no unit: it is reached only when the entry appeared after the planner
    /// looked, and the next run's planner is the one that routes by unit.
    Sink,
    /// A **store door** — one that rewrites or moves a committed doc where it stands, with
    /// no work unit and no promote (`jigc rename`, the relocation primitive;
    /// [`store_home_refusal`]). `withheld` is the clause that says what the door did not
    /// do, `rerun` the clause that says what to run once the home holds the doc itself.
    Store { withheld: &'a str, rerun: &'a str },
}

impl ShapeUnit<'_> {
    /// The doc the refusal is about, as a message names it — the unit, and the address the
    /// doc is **staged under** there (with the join's suffix beside it when it took one).
    fn whose(&self) -> String {
        match self {
            ShapeUnit::Task { address, .. } => format!("this task's doc `{address}`"),
            ShapeUnit::Migration { address, .. } => format!("this migration's doc `{address}`"),
            ShapeUnit::Milestone {
                landing,
                origin: Some(origin),
                ..
            } => {
                let minted = origin.minted.as_str();
                let sub_task = origin.source_task();
                if *landing == minted {
                    format!("sub-task `{sub_task}`'s doc `{minted}`")
                } else {
                    format!(
                        "sub-task `{sub_task}`'s doc `{minted}` — which the join suffixed to \
                         `{landing}` —"
                    )
                }
            }
            ShapeUnit::Milestone {
                landing,
                origin: None,
                ..
            } => format!("this milestone's doc `{landing}`"),
            ShapeUnit::Sink | ShapeUnit::Store { .. } => "the doc".to_owned(),
        }
    }

    /// The route's opening clause: what the refusal left untouched.
    fn intact(&self) -> &'static str {
        match self {
            ShapeUnit::Milestone { .. } => {
                "nothing was committed and every sub-task's staged work is intact"
            }
            _ => "nothing was committed and this task's staged docs are intact",
        }
    }

    /// The route's closing clause: the unit's own boundary, re-run.
    fn rerun(&self) -> String {
        use crate::finding::shell_operand;
        match self {
            ShapeUnit::Migration { id, .. } => {
                let id = shell_operand(id);
                format!(
                    "re-run `jigc task finalize {id}` to review the fidelity diff and `jigc \
                     task finalize {id} --approve` to land it"
                )
            }
            ShapeUnit::Task { id, .. } => {
                format!("re-run `jigc task finalize {}`", shell_operand(id))
            }
            ShapeUnit::Milestone { milestone, .. } => format!(
                "re-run `jigc milestone finalize {}`",
                shell_operand(milestone)
            ),
            ShapeUnit::Sink | ShapeUnit::Store { .. } => "re-run the command".to_owned(),
        }
    }

    /// **The exit that gives the unit's doc another id** — the in-task rename, addressed at
    /// the id the doc is staged under; at the milestone boundary, one per sub-task of the
    /// collision group from the doc's position on ([`sub_task_renames`]). `None` where the
    /// unit names no doc to rename.
    fn rename(&self) -> Option<String> {
        use crate::finding::shell_operand;
        match self {
            ShapeUnit::Task { id, address } | ShapeUnit::Migration { id, address } => {
                Some(format!(
                    "give this task's doc an id whose home is free (`jigc doc rename {} --to \
                     \"<title>\" --task {}`)",
                    shell_operand(address),
                    shell_operand(id),
                ))
            }
            ShapeUnit::Milestone {
                origin: Some(origin),
                ..
            } => Some(sub_task_renames(origin)),
            _ => None,
        }
    }

    /// **The exit that drops the mint** — the one a doc under a fixed identity has, since
    /// no rename can move it: the work unit that staged it is discarded, by consent
    /// (`--force`), and the clause says what goes with it and how to read it first. The
    /// unit is the task itself, or the sub-task the body came from. `None` where the unit
    /// names no work unit that staged the doc.
    fn drop_the_mint(&self) -> Option<String> {
        use crate::finding::shell_operand;
        let (task, address, what, with_it) = match self {
            ShapeUnit::Task { id, address } | ShapeUnit::Migration { id, address } => {
                (*id, *address, "this task", "its staged docs go with it")
            }
            ShapeUnit::Milestone {
                origin: Some(origin),
                ..
            } => (
                origin.source_task(),
                origin.minted.as_str(),
                "the sub-task that minted it",
                "that sub-task's whole staged work goes with it",
            ),
            _ => return None,
        };
        let task = shell_operand(task);
        Some(format!(
            "drop {what} (`jigc task discard {task} --force` — {with_it}, so read what you \
             want kept first: `jigc doc show {} --task {task}`)",
            shell_operand(address),
        ))
    }
}

/// **Whose doc the refused promote was** — what [`clobber_finding`] words its message and
/// its route from. One finding identity (`finalize.promote-clobber`, keyed at the
/// destination), three units whose exits differ.
#[derive(Clone, Copy)]
enum ClobberedBy<'a> {
    /// An ordinary task's `created` doc.
    Task,
    /// A migration task's doc, whose recorded foreign source is a *different* file than the
    /// occupied destination.
    Migration {
        /// The recorded repo-relative migration source.
        source: &'a str,
    },
    /// A sub-task's `created` doc, at the milestone commit boundary.
    SubTask {
        /// The milestone whose boundary refused.
        milestone: &'a str,
        /// The doc's **final** address — the one the join resolved it to, whose home is
        /// the occupied destination.
        landing: &'a str,
        /// Where the body came from: its sub-task, the address it is staged under there,
        /// and the rest of its collision group.
        origin: &'a crate::milestone::MergedOrigin,
    },
    /// **Any** unit's doc, over a destination that holds an entry which is not a regular
    /// file (`(R6, D-7)`) — the arm that is about the entry and not about the doc.
    Shape {
        /// What is at the destination.
        shape: crate::store::ForeignEntry,
        /// The exit this unit has from it.
        exit: ShapeExit,
        /// Who is told.
        unit: ShapeUnit<'a>,
    },
    /// A unit's `created` doc, over a destination with nothing on disk that **git still
    /// holds** (CPL-5) — a committed doc missing from the worktree, or a file staged and
    /// taken out of it.
    Held {
        /// Where git holds it.
        holder: Holder,
        /// The exit this unit has: another id for its doc, or — under a fixed identity —
        /// none but dropping the mint.
        exit: ShapeExit,
        /// Who is told.
        unit: ShapeUnit<'a>,
    },
}

/// A blocking finding (review S1, `design/auto-migration.md` → Honest bounds → the
/// data-loss clobber guard) when a create-provenance doc's canonical promote destination
/// already holds a file — a committed managed doc that would collide, or a hand-authored /
/// foreign file (e.g. an existing `VISION.md` at a placement home) — promoting would silently
/// overwrite it (irreversible data loss). Names the destination it refused to clobber, and
/// [keys at it](file_location) — the `(code, target)` key is one for both arms below.
///
/// The route discriminates on the recorded migration source (M43 inc-6 T3; RC-lacon
/// findings-verification → A4; `design/surface-contract.md` → law 2/3) and **never
/// teaches raw removal** — the pre-M43 "remove or adopt it" was verbatim the A4/A7 trap
/// that had a user pre-stage a silent `git rm`:
///
/// - **[`ClobberedBy::Migration`]** — a migration whose recorded foreign source is a
///   *different* file than the occupied destination (the title-slug collision; the
///   source == destination case is the in-place rewrite [`plan_clobber_guard`] carves
///   out, so it never reaches here). The finding names **both** paths and the route ends
///   at the migration's own continuation: plain finalize → review the fidelity diff →
///   `--approve` (the sole destructive gate, which also retires the recorded source).
/// - **[`ClobberedBy::Task`]** — an ordinary create collision; the route repairs through
///   jigc verbs only (retitle / an explicit `--slug` / adopt the occupant via `jigc
///   migrate`).
/// - **[`ClobberedBy::SubTask`]** — the milestone commit boundary's arm
///   ([`sub_task_clobber_text`]), whose unit has neither of the exits above: a sub-task has
///   no boundary of its own, and adopting the occupant first moves `HEAD` off the
///   milestone's base.
/// - **[`ClobberedBy::Shape`]** — the destination holds an entry that is **not a regular
///   file** ([`shape_clobber_text`]; the rc.24 fix pass, `(R6, D-7)`). One identity with the
///   arms above — the same code, keyed at the same destination path — because it is the
///   same refusal: something a third party put at the doc's home, which a promote would
///   write over or, here, *through*. It routes at no `jigc migrate`: a link is not a file
///   to adopt (driven: `migrate.source-untrackable`).
/// - **[`ClobberedBy::Held`]** — nothing is on disk at the destination and **git still
///   holds a file there** ([`held_clobber_text`]; the completion audit's CPL-5). The same
///   refusal once more, asked of the place the occupant is: the commit would replace it
///   under its own path.
fn clobber_finding(repo_root: &Path, destination: &str, by: ClobberedBy) -> Finding {
    let (message, route) = match by {
        ClobberedBy::Migration { source } => (
            format!(
                "promoting this migration's doc to `{destination}` would overwrite a file \
                 already there — and that file is not the migration's recorded source \
                 (`{source}`); refusing to clobber it"
            ),
            format!(
                "a file already occupies `{destination}`, and this migration's recorded \
                 source is the different file `{source}`: if the occupant is another \
                 managed doc, land this migration under a different id — re-author with a \
                 title that slugs differently, or `jigc task discard <id> --force` and re-mint \
                 with `{source_migrate} --as <doctype> --slug <different-slug>`; if \
                 the occupant is itself foreign, adopt it through its own `jigc migrate` \
                 task first; then re-run `jigc task finalize <id>` to review the fidelity \
                 diff and `jigc task finalize <id> --approve` to land it (`--approve` also \
                 retires the recorded source)",
                // **Absolute** (M53 post-review-fix review, HIGH 2): `source` is the recorded
                // repo-relative migration source, and `jigc migrate <PATH>` roots its argument
                // at the caller's cwd.
                source_migrate = crate::finding::migrate_at(repo_root, source),
            ),
        ),
        ClobberedBy::Task => (
            format!(
                "promoting this task's doc to `{destination}` would overwrite a file already \
                 there — refusing to clobber it"
            ),
            format!(
                "a file already occupies `{destination}`: if it is another managed doc, \
                 retitle this task's doc so it slugs differently, or re-create it with an \
                 explicit `--slug` (`jigc doc create <doctype> --title <title> --slug \
                 <slug> --task <id>`); if it is a hand-authored/foreign file, bring it \
                 under management with `{destination_migrate} --as <doctype>` in \
                 its own task; then re-run `jigc task finalize`",
                destination_migrate = crate::finding::migrate_at(repo_root, destination),
            ),
        ),
        ClobberedBy::SubTask {
            milestone,
            landing,
            origin,
        } => sub_task_clobber_text(destination, milestone, landing, origin),
        ClobberedBy::Shape { shape, exit, unit } => {
            shape_clobber_text(destination, shape, exit, unit)
        }
        ClobberedBy::Held { holder, exit, unit } => {
            held_clobber_text(repo_root, destination, holder, exit, unit)
        }
    };
    Finding::graded(
        Severity::Blocking,
        "finalize.promote-clobber",
        message,
        Some(file_location(destination)),
        Some(route.into()),
    )
}

/// The message and the route of [`clobber_finding`]'s **sub-task arm** — the milestone
/// commit boundary refusing to promote a sub-task's `created` doc over an occupied home
/// (the rc.24 fix pass, `(R6, D-1)`).
///
/// **The route has to land the milestone with every sub-task's work in it**, because a
/// blocked boundary leaves N sub-tasks pending and none of them has a boundary of its own.
/// Three things shape it, each driven on the built binary:
///
/// - **The exit is the in-task rename, addressed at the sub-area.** `jigc doc rename
///   <minted> --to "<title>" --task <sub-task>` re-slugs the staged doc inside that
///   sub-task's own area — body, provenance entry, role binding and staged referrers — so
///   the next join produces a different final address. The address it takes is the one the
///   doc is **staged under** ([`crate::milestone::MergedOrigin::minted`]), not the suffixed
///   one this finding is about: `<type>:<slug>-2` names nothing in any sub-area.
/// - **A suffixed landing names every later member of its group.** The join numbers the
///   sub-tasks that minted one slug by position in task-id order, so a doc renamed out of
///   the group moves each later one up a suffix — renaming only the blocked doc hands its
///   occupied suffix to the next sub-task, and the re-run refuses again. The route therefore
///   carries one rename per sub-task from this doc's position on
///   ([`crate::milestone::MergedOrigin::group_from_here`]): with those docs out of the
///   group, nothing lands on this suffix or a later one, and the earlier members keep the
///   ids they had.
/// - **It never routes at adopting the occupant first.** The task arm's *"bring it under
///   management … in its own task"* is a commit, and a commit made before this boundary
///   moves `HEAD` off the milestone's pinned base — the milestone then blocks on
///   `finalize.base-mismatch`, whose own exits are out-of-band git. So the occupant is named
///   as untouched and the order is stated: the milestone lands first.
///
/// A `Human` route, like the in-task rename's own occupancy refusal it sends the reader to:
/// the new title is the author's to write, and a mechanical argv is one command.
fn sub_task_clobber_text(
    destination: &str,
    milestone: &str,
    landing: &str,
    origin: &crate::milestone::MergedOrigin,
) -> (String, String) {
    use crate::finding::shell_operand;
    let sub_task = origin.source_task();
    let minted = origin.minted.as_str();
    let suffixed = landing != minted;
    let message = if suffixed {
        format!(
            "promoting sub-task `{sub_task}`'s doc `{minted}` — which the join suffixed to \
             `{landing}` — to `{destination}` would overwrite a file already there; refusing \
             to clobber it"
        )
    } else {
        format!(
            "promoting sub-task `{sub_task}`'s doc `{minted}` to `{destination}` would \
             overwrite a file already there — refusing to clobber it"
        )
    };
    let rename = sub_task_renames(origin);
    let route = format!(
        "nothing was committed and every sub-task's staged work is intact: {rename}, then \
         re-run `jigc milestone finalize {milestone}`. The file at `{destination}` is left \
         exactly as it is — deal with it after the milestone has landed, not before: a \
         commit made first moves `HEAD` off this milestone's base and blocks it on \
         `finalize.base-mismatch`",
        milestone = shell_operand(milestone),
    );
    (message, route)
}

/// **The in-task renames that move a sub-task's doc off a home** — the clause both
/// milestone-boundary arms of [`clobber_finding`] hand back ([`sub_task_clobber_text`]'s
/// route, and the shape arm's rename exit): one `jigc doc rename … --task <sub-task>` per
/// sub-task of the collision group from this doc's position on
/// ([`crate::milestone::MergedOrigin::group_from_here`]), addressed at the id the doc is
/// **staged under** in its sub-area. Why a suffixed landing names every later member is
/// [`sub_task_clobber_text`]'s to say.
fn sub_task_renames(origin: &crate::milestone::MergedOrigin) -> String {
    use crate::finding::shell_operand;
    let minted = origin.minted.as_str();
    let renames: Vec<String> = origin
        .group_from_here
        .iter()
        .map(|sub| {
            format!(
                "`jigc doc rename {} --to \"<title>\" --task {}`",
                shell_operand(minted),
                shell_operand(sub),
            )
        })
        .collect();
    match renames.as_slice() {
        [one] => format!("give the doc a title that slugs to an id nothing holds ({one})"),
        many => format!(
            "the join numbers the sub-tasks that minted `{minted}` in task-id order, so \
             renaming only this doc would move the next one onto the same id — give the doc \
             of each of the {} sub-tasks from this one on its own title, slugging to an id \
             nothing holds ({})",
            many.len(),
            many.join(", "),
        ),
    }
}

/// The message and the route of [`clobber_finding`]'s **shape arm** — a promote refused
/// because its destination holds an entry that is not a regular file (the rc.24 fix pass,
/// `(R6, D-7)`; `design/finalize.md` → 4. Promote).
///
/// **The contract it states**: a promote lands a regular file at exactly its canonical
/// path, or the transaction refuses before anything is written. jigc writes regular files
/// and real directories and never a link, so the entry is a third party's — and the route
/// therefore never says what to do *to* it beyond the one thing every exit needs: that it
/// is not at the doc's home when the unit is re-run. It teaches no removal (the A4/A7 trap,
/// `design/surface-contract.md` → law 2): the entry is moved, or replaced by the file it
/// stands for, by the person whose entry it is.
///
/// **Each exit is one that lands, per unit and per provenance** ([`ShapeExit`]), driven on
/// the built binary at both committing doors:
///
/// - a doc the unit **minted** under an id the author chooses is re-slugged in the task
///   (`jigc doc rename … --task <unit>` — addressed at the id the doc is *staged under*, as
///   at the milestone boundary's file arm) or lands once the home is free;
/// - a doc under a **fixed identity** has no other id, so only the freed home is named — a
///   rename there refuses, and a route must not hand back a command that refuses;
/// - a doc the unit **copied in through a live link** and edited is `edited-from-base`: it
///   lands over the regular file itself, put where the link was.
///
/// At the milestone boundary every exit is stated as **no commit**, because the file arm's
/// rule holds here too: a commit made before the boundary moves `HEAD` off the milestone's
/// pinned base and blocks it on `finalize.base-mismatch`.
fn shape_clobber_text(
    destination: &str,
    shape: crate::store::ForeignEntry,
    exit: ShapeExit,
    unit: ShapeUnit,
) -> (String, String) {
    use crate::finding::shell_operand;
    let noun = shape.noun();
    let bare = shape.bare();
    let whose = unit.whose();
    let message = match unit {
        // No unit and no promote: the doc is committed at this home, and the door would
        // have rewritten it there or carried the entry somewhere else.
        ShapeUnit::Store { withheld, .. } => format!(
            "`{destination}` is {noun}, not a regular file — jigc keeps a managed doc as a \
             regular file at exactly its home, and neither writes through a link nor moves \
             one, so {withheld}"
        ),
        _ => format!(
            "`{destination}` is {noun}, not a regular file — jigc lands a managed doc as a \
             regular file at exactly its home and never writes through a link, so {whose} \
             is not promoted there"
        ),
    };

    let yours = format!(
        "jigc writes regular files only, so the {bare} at `{destination}` is not one it put \
         there, and what becomes of it is yours to decide"
    );
    let move_out =
        format!("move the {bare} out of the doc's home, so that `{destination}` is free");
    let regular = format!(
        "put the regular file itself at `{destination}` — for a link, a copy of the file it \
         points at, in the link's place"
    );
    let route = match unit {
        ShapeUnit::Task { id, address } | ShapeUnit::Migration { id, address } => {
            let id = shell_operand(id);
            let finalize = match unit {
                ShapeUnit::Migration { .. } => format!(
                    "re-run `jigc task finalize {id}` to review the fidelity diff and `jigc \
                     task finalize {id} --approve` to land it"
                ),
                _ => format!("re-run `jigc task finalize {id}`"),
            };
            let act = match exit {
                ShapeExit::RenameOrMoveOut => format!(
                    "give this task's doc an id whose home is free (`jigc doc rename {} --to \
                     \"<title>\" --task {id}`), or {move_out}; then {finalize}",
                    shell_operand(address),
                ),
                ShapeExit::MoveOut => format!("{move_out}; then {finalize}"),
                ShapeExit::RegularFile => {
                    format!("{regular}; then {finalize} — this task's staged edit lands over it")
                }
            };
            format!("nothing was committed and this task's staged docs are intact. {yours}: {act}")
        }
        ShapeUnit::Milestone {
            milestone, origin, ..
        } => {
            let finalize = format!(
                "re-run `jigc milestone finalize {}`",
                shell_operand(milestone)
            );
            let no_commit = "that is no commit, so `HEAD` stays on this milestone's base";
            let act = match (exit, origin) {
                (ShapeExit::RenameOrMoveOut, Some(origin)) => format!(
                    "{}, or {move_out} — {no_commit}; then {finalize}",
                    sub_task_renames(origin),
                ),
                (ShapeExit::RegularFile, _) => format!(
                    "{regular} — {no_commit}; then {finalize}, which lands the sub-task's \
                     staged edit over it"
                ),
                _ => format!("{move_out} — {no_commit}; then {finalize}"),
            };
            format!(
                "nothing was committed and every sub-task's staged work is intact. {yours}: \
                 {act}"
            )
        }
        ShapeUnit::Sink => format!(
            "nothing was committed and the promote was rolled back — the {bare} appeared at \
             `{destination}` after this finalize had planned its promote. {yours}: re-run \
             the finalize, which names the exit this unit has from it"
        ),
        // The copied-in arm's exit, at a door that acts on the committed store: the regular
        // file itself at the home is a **commit** here, and the door's own command follows.
        ShapeUnit::Store { rerun, .. } => format!(
            "nothing was written through `{destination}` and it stands exactly as it was. \
             {yours}: put the doc itself at `{destination}` as a regular file — for a link, \
             a copy of the file it points at, in the link's place — and commit that; then \
             {rerun}"
        ),
    };
    (message, route)
}

/// The message and the route of [`clobber_finding`]'s **held arm** — a `created` doc refused
/// because its destination, with nothing on disk, is a path **git still holds** (the rc.24
/// fix pass, the completion audit's CPL-5; `design/finalize.md` → 4. Promote).
///
/// **The state.** A committed doc deleted from the worktree and not committed as deleted, or
/// a file staged and then taken out of the worktree. Both read as a free home to every probe
/// that asks the disk — the create that minted the doc included — so the doc is `created`,
/// and landing it replaces what git holds under its own path: the committed doc under its
/// own id, at exit 0, or the staged blob, which no commit then has.
///
/// **Each exit is one that lands, and none of them is a commit made first** — a commit
/// before a unit's boundary moves `HEAD` off its base pin, at either door:
///
/// - a doc minted under an id its author chooses takes **another id** in the unit
///   ([`ShapeUnit::rename`]), and lands beside what git holds;
/// - a doc under a **fixed identity** has no other id, so the mint itself is dropped
///   ([`ShapeUnit::drop_the_mint`]) — the work is then done again over the file that is
///   there, which the create copies in for update instead of minting over.
///
/// The file git holds is never jigc's to decide about: the route names the one command that
/// brings it back into the worktree ([`crate::finding::git_at`], so it runs from any
/// directory) and says it is left as it is. It teaches no removal and no commit.
fn held_clobber_text(
    repo_root: &Path,
    destination: &str,
    holder: Holder,
    exit: ShapeExit,
    unit: ShapeUnit,
) -> (String, String) {
    use crate::finding::{git_at, shell_operand};
    let whose = unit.whose();
    let (holds, restore) = match holder {
        Holder::Head => (
            "the committed file is still in `HEAD`",
            git_at(
                repo_root,
                &format!("checkout HEAD -- {}", shell_operand(destination)),
            ),
        ),
        Holder::Index => (
            "a file is staged there in git's index, in no commit",
            git_at(
                repo_root,
                &format!("checkout -- {}", shell_operand(destination)),
            ),
        ),
    };
    let message = format!(
        "`{destination}` is missing from the worktree but not from git — {holds} — and \
         {whose} was minted as a new doc, not as an edit of that file: promoting it would \
         replace the file git holds under its own path, so it is not promoted there"
    );
    let intact = unit.intact();
    let rerun = unit.rerun();
    let left = format!(
        "The file git holds at `{destination}` is left exactly as it is: `{restore}` brings \
         it back into the worktree, and that is no commit"
    );
    let route = match (exit, unit.rename(), unit.drop_the_mint()) {
        (ShapeExit::RenameOrMoveOut, Some(rename), _) => {
            format!("{intact}: {rename}, then {rerun}. {left}")
        }
        (_, _, Some(drop)) => {
            let again = match unit {
                ShapeUnit::Milestone { .. } => format!(
                    "{drop}, then {rerun}; a task started once the milestone has landed \
                     meets the file that is there, and its create copies a managed doc in \
                     for update instead of minting over it"
                ),
                _ => format!(
                    "{drop}, bring the file back, and start the work again — the create \
                     then meets the file that is there, and copies a managed doc in for \
                     update instead of minting over it"
                ),
            };
            format!(
                "{intact}. A doc of this doctype has one fixed home, so it cannot be given \
                 another id: {again}. {left}"
            )
        }
        // No work unit to name (a body the join did not write): the home has to hold the
        // file git holds before anything lands over it.
        _ => format!("{intact}: bring the file back, then {rerun}. {left}"),
    };
    (message, route)
}

/// **The promote sink's refusal** — [`clobber_finding`]'s shape arm, raised by the write
/// itself (`cli::task::promote`) when the entry at a destination is not a regular file at
/// the moment it would be written.
///
/// The planner's guard is the routed refusal; this is the backstop behind it, for an entry
/// that appeared between the plan and the write — the pattern the retire sink set at M51
/// (`cli::task::ValidatedRetirement`): a guard that only ran at the door guards the plan,
/// not the write. Same code and key as the planner's, so a driver reads one identity
/// whichever of the two caught it.
#[must_use]
pub fn promote_sink_refusal(
    repo_root: &Path,
    destination: &str,
    shape: crate::store::ForeignEntry,
) -> Finding {
    clobber_finding(
        repo_root,
        destination,
        ClobberedBy::Shape {
            shape,
            exit: ShapeExit::MoveOut,
            unit: ShapeUnit::Sink,
        },
    )
}

/// **A store door's refusal over a home that is not a regular file** — [`clobber_finding`]'s
/// shape arm, raised by a door that writes or moves a committed doc **where it stands**
/// rather than promoting one (the rc.24 fix pass; `design/finalize.md` → 4. Promote,
/// *the doors that write a committed home in place*).
///
/// `(R6, D-7)` made the shape arm true of every door that promotes. The doors that do not
/// promote shared the mechanism and were left declared: `jigc rename` moved a link with
/// `git mv` and wrote the retitle — and every referrer's repoint — *through* it, and the
/// relocation primitive carried a link to a new home. They refuse the same state of the
/// same doc under the **same code, keyed at the same path**: one fault owes one identity
/// (`cli::rename::RefusalKind::code`'s rule), and a driver that has learnt what
/// `finalize.promote-clobber` at a doc's home means has nothing new to learn at these
/// doors. What differs is the unit — there is none — so the message says what the door
/// withheld (`withheld`) and the route ends at the door's own command (`rerun`) once the
/// regular file is committed at the home.
///
/// `home` is the repo-relative path of the entry.
#[must_use]
pub fn store_home_refusal(
    repo_root: &Path,
    home: &str,
    shape: crate::store::ForeignEntry,
    withheld: &str,
    rerun: &str,
) -> Finding {
    clobber_finding(
        repo_root,
        home,
        ClobberedBy::Shape {
            shape,
            // The home has to hold the regular file itself — the one exit a committed doc
            // has from it. The store arm words it; the variant records which exit it is.
            exit: ShapeExit::RegularFile,
            unit: ShapeUnit::Store { withheld, rerun },
        },
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
        Some(
            format!(
                "resolve the read fault on the task provenance manifest under `{}` (a disk \
                 or permissions problem), then re-run the finalize",
                task_dir.display()
            )
            .into(),
        ),
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
/// is consciously unchanged: the amendment targets the **per-task** doors. Since M47
/// (N7) those doors are two — the commit door and the read-only `jigc start --task`
/// resume door, which called a blanket `base != HEAD` refusal and was therefore
/// stricter than the commit door it precedes; both now make this one decision.
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

/// Which **door** the carryover decision speaks for. The **wording** differs because
/// the *facts* differ (surface-contract law 1 — say the truth): a task finalize is a
/// whole-index commit a carried entry WOULD silently ride; a milestone finalize builds
/// its aggregate from the sub-task worktrees over targeted pathspecs (throwaway
/// indexes, dedicated worktrees, an `--ff-only` land), so a live-index entry
/// structurally CANNOT ride it — it **stays staged across the boundary**, and the
/// refuse is the declare-at-the-boundary rule, not a leak fix; and a `task validate`
/// **preview** has refused nothing at all — it reports what the finalize will refuse.
/// The decision itself is identical at every door; only the finding's message/route
/// change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryoverBoundary {
    /// `jigc task finalize` — the whole-index commit.
    Task,
    /// `jigc task validate` — the **preview** of the [`CarryoverBoundary::Task`]
    /// refusal (M47 Inc 4, Settle Decision 1): the same task boundary, the same
    /// decision and the same exemptions, read from a door that commits nothing. A
    /// route text presuming the finalize invocation would be a law-1 lie here, so the
    /// wording names the door that *will* refuse and never claims one did.
    TaskPreview,
    /// `jigc milestone finalize` — the aggregate commit built from the sub-task
    /// worktrees; a carried entry stays staged, never committed here.
    Milestone,
    /// `jigc setup` — the **install commit** (M51 Increment 3; `settle-record.md` → D3,
    /// amended by §1). The family's fourth member and the one that is not a *task* door
    /// at all: the install commit is a pathspec commit of the files `setup` just wrote,
    /// and a path in that pathspec carrying bytes `setup` did not write would ride it
    /// undeclared — the same class of refusal ("a door committing paths it does not
    /// own"), asked over a different predicate and worded for a door with no task.
    ///
    /// **It is not a [`decide_carryover`] door.** Its predicate is **worktree-vs-HEAD,
    /// per path, asked before `setup` writes anything** — not the staged-snapshot pair,
    /// whose subject is the *index* vs HEAD and which therefore yields zero findings on
    /// this door's own cell (a pathspec commit takes the **worktree** contents, and a
    /// restaged different blob reads as the door's own work). The door asks git itself
    /// ([`crate::finalize`] shells out to nothing) and hands the dirty set here to be
    /// worded: **one** finding over the whole set ([`setup_dirty_install_finding`]),
    /// never one per path.
    Setup,
}

impl CarryoverBoundary {
    /// The **finding code** this door's refusal keys at — the door's identity, derived
    /// from the boundary rather than re-spelled at each producer. The three
    /// [`decide_carryover`] doors share `finalize.carried-staged` (one decision, one
    /// subject — a path staged before the work unit existed); [`CarryoverBoundary::Setup`]
    /// carries its own, because that family's task-shaped vocabulary (*"before this task
    /// existed"*, `--carry-staged`) would lie at a door with no task
    /// ([surface-contract.md](../../../design/surface-contract.md) → law 1).
    fn code(self) -> &'static str {
        match self {
            CarryoverBoundary::Task
            | CarryoverBoundary::TaskPreview
            | CarryoverBoundary::Milestone => "finalize.carried-staged",
            CarryoverBoundary::Setup => "setup.dirty-install-path",
        }
    }

    /// Whether this door speaks for the **task** boundary — the committing door and
    /// its preview alike. The owner-artifact exemption keys on the boundary, not on
    /// the door (the milestone-boundary exemption is a separate, deferred concern), so
    /// widening the door axis must never widen the exemption's.
    fn is_task_boundary(self) -> bool {
        matches!(
            self,
            CarryoverBoundary::Task | CarryoverBoundary::TaskPreview
        )
    }
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
/// ([`crate::state::read_migration_source`]) — that deletion is the task's own, never a
/// carryover. `owner_exempt` is the task's recorded owner-artifact paths
/// ([`FinalizePlan::owner_artifacts`]; M45 Inc 8): an agent stages the audit artifact
/// **before** minting the recording task (the natural authoring order), so the recorded
/// path is the task's own subject, never a foreign carry-over — it is exempt **only at the
/// [`CarryoverBoundary::Task`] boundary** (the milestone-boundary owner-artifact exemption
/// is a separate, deferred concern). A **`None` snapshot yields no findings** — the declared
/// fail-open bound (a task minted pre-M43 finalizes as today). `boundary` selects the honest
/// wording for the refusing verb ([`CarryoverBoundary`]).
///
/// Findings come out **sorted by path** across both halves (one union `BTreeSet`) —
/// byte-identical whatever order the sets were built in (Validation hardening #7).
/// A separate decision fn rather than a planner phase, so the CLI can keep the refuse
/// on the **committing** path only — a planner-internal refuse would also block the
/// `--dry-run` forecast, which consumes the plan.
///
/// `home` is the **absolute root of the checkout whose index this decision is about** — the
/// caller's own repo root, which at `jigc task finalize` is the standing checkout (the
/// checkout the door commits in, `crates/cli/src/render.rs` → the commit-site line) and at
/// the milestone boundary is the workbench's. The route's `git restore --staged` names it, so
/// the unstage reaches the index the refusal is about from any directory
/// ([`crate::finding::git_at`]).
pub fn decide_carryover(
    snapshot: Option<&StagedSnapshot>,
    current: &StagedSnapshot,
    retire_exempt: Option<&str>,
    owner_exempt: &[String],
    boundary: CarryoverBoundary,
    home: &Path,
) -> Vec<Finding> {
    // Missing snapshot ⇒ fail-open: a task minted before the gate existed finalizes
    // as today (the declared bound).
    let Some(snapshot) = snapshot else {
        return Vec::new();
    };
    // The exempt paths are stored prose (a retire path may carry `./`; an owner-artifact
    // path is the recorded field value); the git-fact paths are already repo-relative
    // canonical — normalize the exempt side only (the `plan_clobber_guard` source-path
    // precedent). The retire path is a migration task's own deletion; the owner-artifact
    // paths are the task's recorded subject, exempt only at the TASK boundary (the
    // milestone owner-artifact exemption is a separate, deferred concern).
    let mut exempt: BTreeSet<PathBuf> = BTreeSet::new();
    if let Some(path) = retire_exempt {
        exempt.insert(crate::store::lexical_normalize(Path::new(path)));
    }
    if boundary.is_task_boundary() {
        for path in owner_exempt {
            exempt.insert(crate::store::lexical_normalize(Path::new(path)));
        }
    }

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
        .filter(|(path, _)| !exempt.contains(Path::new(path)))
        .map(|(path, is_deletion)| carried_staged_finding(path, is_deletion, boundary, home))
        .collect()
}

/// One blocking `finalize.carried-staged` finding for one carried path — a staged
/// change (`is_deletion: false`) or a staged deletion (`true`) that predates the work
/// unit. Its subject is the **file** staged before the boundary's unit existed, so it
/// [keys at its path](file_location) (the file-path target form — mid-carry the path
/// may be foreign, with no managed identity). The route names both exits: unstage it,
/// or declare the carry-over deliberate with `--carry-staged` (the `--approve` mold —
/// undecidable intent converted to a declared one), phrased for the door it is read
/// from — the committing doors say *re-run*, the preview door cannot (nothing was run).
/// Which exit is right is a judgment call, so the route is [`Route::human`]. The message
/// states the door's real consequence ([`CarryoverBoundary`], law 1): a task's
/// whole-index commit would silently absorb the entry; a milestone's aggregate cannot
/// carry it — the entry stays staged across the boundary either way; and the `task
/// validate` preview names the finalize that *will* refuse rather than claiming one did.
fn carried_staged_finding(
    path: &str,
    is_deletion: bool,
    boundary: CarryoverBoundary,
    home: &Path,
) -> Finding {
    let what = if is_deletion {
        "staged for deletion"
    } else {
        "staged"
    };
    let kind = if is_deletion { "deletion" } else { "change" };
    // The unstage names the index it acts on; the message, the `at:` locus and the
    // `(code, target)` key stay repo-relative (M53 — the cwd census, the route class).
    let unstage = crate::finding::git_at(
        home,
        &format!("restore --staged -- {}", crate::finding::shell_token(path)),
    );
    let (message, route) = match boundary {
        CarryoverBoundary::Task => (
            format!(
                "`{path}` was already {what} before this task existed — refusing to let a \
                 pre-task staged {kind} silently ride this task's commit"
            ),
            format!(
                "unstage it (`{unstage}`) if it is not this task's work, or re-run the \
                 finalize with `--carry-staged` to declare the carry-over deliberate"
            ),
        ),
        // The preview door (M47 Inc 4): nothing has been refused, so the message says
        // what *will* be — and the consent flag is accepted at both doors, so the route
        // says so rather than naming a finalize re-run the reader never ran.
        CarryoverBoundary::TaskPreview => (
            format!(
                "`{path}` was already {what} before this task existed — `jigc task finalize` \
                 will refuse to let a pre-task staged {kind} silently ride this task's commit"
            ),
            format!(
                "unstage it (`{unstage}`) if it is not this task's work, or pass \
                 `--carry-staged` — accepted here and at the finalize — to declare the \
                 carry-over deliberate"
            ),
        ),
        // The `Setup` door is not a per-path door: its refusal is ONE finding over the
        // whole dirty set ([`setup_dirty_install_finding`]), and its predicate is the
        // pre-write worktree-vs-HEAD query the door asks itself, never this staged-snapshot
        // decision (`settle-record.md` → Review amendments §1). Reached only if a caller
        // hands this boundary to the per-path decision anyway; the wording stays the door's
        // own, over the one-path set, so no door can borrow another's words.
        CarryoverBoundary::Setup => {
            return setup_dirty_install_finding(&[path.to_string()], true, SetupHead::Born, None);
        }
        CarryoverBoundary::Milestone => (
            format!(
                "`{path}` was already {what} before this milestone existed — the aggregate \
                 commit is built from the sub-task worktrees and cannot carry it, so the \
                 {kind} stays staged, undeclared, across this boundary"
            ),
            format!(
                "unstage it (`{unstage}`) if it is stale, or re-run the finalize with \
                 `--carry-staged` to declare it deliberate (it stays staged either way)"
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

/// The **[`CarryoverBoundary::Setup`] door's refusal**: one blocking, routed
/// `setup.dirty-install-path` finding over the whole set of install-footprint paths that
/// carried bytes `jigc setup` did not write (M51 Increment 3; `settle-record.md` → D3 with
/// its §1 amendment, and §10's mold — a blocking [`Finding`], a [`Route::human`] route,
/// exit 1, registered in `validation.md`'s inventory and outside `ERROR_CODE_REGISTRY`,
/// which mirrors door *outcome* identities).
///
/// `dirty` is the CLI's per-path **worktree-vs-HEAD** answer taken **before `setup` wrote
/// anything** (the engine shells out to nothing — the [`decide_carryover`] mold: the door
/// supplies the git facts, the engine words the finding). The set is the subject, so the
/// finding carries **no [`Location`]** — no single listed path keys it — and lists its paths
/// **sorted**, so the door's pathspec order can never reach the printed surface
/// (order-invariant by construction).
///
/// **What the message may not claim** (the load-bearing honesty constraint): the guard binds
/// the **commit**, not the install — the install files stay written and staged — so for a
/// path `setup` regenerates whole (`.jigc/AGENT.md`, `.jigc/version`, `.jigc/.gitignore`) it
/// does **not** put the user's bytes back on disk. The message therefore states only what is
/// certain at this point: the install files are written and staged, no install commit was
/// made, and `HEAD` is untouched — so every listed path's pre-run bytes are still in it. It
/// claims nothing about the worktree.
///
/// **The route names the resolving act first and `--force` second.** `--force` at the first
/// command an adopter runs is D3's declared reflex-training risk, so the exit that keeps the
/// work leads and the single consent follows, stated as what it spends. It names `-u` on the
/// stash because a listed path may be one git holds **no** copy of (M51 Increment 3
/// completion audit — untracked is a subject of this door): a bare `git stash push -- <path>`
/// there exits 1 with *"did not match any file(s) known to git"*, and a route that does not
/// run is not a route.
///
/// **And both the state clause and the route are worded for the `HEAD` they are read at**
/// ([`SetupHead`]; the rc.24 fix pass, `(R1, F1)`). Every sentence above is about a
/// repository that has a commit. On an **unborn** `HEAD` two of them are false: there is no
/// `HEAD` for the pre-run bytes to *still be in*, and `git stash` — the route's first
/// resolving act — exits 1 with *"You do not have the initial commit yet"*, with or without
/// `-u`. So that arm states what is true there (the bytes exist only where the adopter left
/// them) and routes at acts that run with no commit: move the file out of the install path,
/// or commit it — naming the `riding` paths that commit must carry — then `--force`.
///
/// **And for the paths git's own status could not see** ([`SetupUnseen`]; the rc.24 fix
/// pass, the ignored sibling of `(R1, F1)`). Every route arm above is an act git performs
/// on a path it reports. An **ignored** file is taken by neither `git stash` nor
/// `git commit`, and a tracked file whose index entry is flagged **assume-unchanged** or
/// **skip-worktree** is one git believes is clean — so over such a path the born route's
/// *commit or stash* is a no-op that leaves the refusal standing, and the unborn route's
/// *commit them* exits 1. Each class is therefore routed at the act that works for it: an
/// ignored file is moved out of the path, a flagged one has its flag cleared (both
/// commands, since one invocation clears only the first flag it names) and is then
/// committed or stashed like any other change. The message says which paths these are and
/// why nothing else warned about them, and its state clause stops saying `HEAD` holds the
/// pre-run bytes — for an ignored file nothing in git does.
pub fn setup_dirty_install_finding(
    dirty: &[String],
    install_written: bool,
    head: SetupHead<'_>,
    unseen: Option<SetupUnseen<'_>>,
) -> Finding {
    let paths: BTreeSet<&str> = dirty.iter().map(String::as_str).collect();
    // The unseen classes, each cut to the paths this finding lists and sorted with them.
    // They exist only at the pre-write ask: by the commit-time backstop the install has
    // run, so a replaced file holds jigc's bytes and there is nothing left to say of it.
    let unseen = unseen.filter(|_| !install_written);
    let (named_ignored, named_flagged): (&[String], &[String]) =
        unseen.map_or((&[], &[]), |unseen| (unseen.ignored, unseen.flagged));
    let listed = |named: &[String]| -> Vec<&str> {
        paths
            .iter()
            .copied()
            .filter(|path| named.iter().any(|name| name == path))
            .collect()
    };
    let ignored = listed(named_ignored);
    let flagged: Vec<&str> = listed(named_flagged)
        .into_iter()
        .filter(|path| !ignored.contains(path))
        .collect();
    let seen = paths.len() - ignored.len() - flagged.len();
    let ticked = |paths: &[&str]| -> String {
        let named: Vec<String> = paths.iter().map(|path| format!("`{path}`")).collect();
        named.join(", ")
    };
    // Why nothing else warned about them — appended to either `HEAD`'s message, and empty
    // (so every pre-existing byte stands) when git's status named every listed path.
    let unreported = if ignored.is_empty() && flagged.is_empty() {
        String::new()
    } else {
        let mut lines = vec![format!(
            "\n`git status` reports nothing at {} of those path(s), so nothing else would \
             have warned you:",
            ignored.len() + flagged.len(),
        )];
        for path in &paths {
            if ignored.contains(path) {
                lines.push(format!(
                    "  `{path}`: ignored — the install replaces this file whole, and no \
                     commit, stash or index entry holds what is in it (an unedited copy an \
                     older jigc wrote reads the same once the generated text has changed, \
                     and `--force` loses nothing over one)"
                ));
            } else if flagged.contains(path) {
                lines.push(format!(
                    "  `{path}`: tracked, with the change hidden by its index entry \
                     (assume-unchanged or skip-worktree)"
                ));
            }
        }
        lines.join("\n")
    };
    let listing: Vec<String> = paths.iter().map(|path| format!("  `{path}`")).collect();
    let listing = listing.join("\n");
    // What the door actually did, per shape — the same refusal is reachable before the
    // install's first write (the gate) and after it (the pre-commit backstop), and a
    // sentence true of one is false of the other.
    let state = if install_written {
        "the install files are written and staged, and no install commit was made"
    } else {
        "nothing was installed and no install commit was made"
    };
    // The one consent, and what it spends — the same sentence at either `HEAD`.
    const FORCE: &str = "`jigc setup --force` is the single consent, and it lets the install \
                         run and commit those paths as it leaves them — which at a path jigc \
                         regenerates whole is jigc's own content, not yours";
    // At an ignored path the consent buys a replacement and no commit — said where one is
    // listed, so the sentence above does not promise a commit git will not make.
    let force = if ignored.is_empty() {
        FORCE.to_string()
    } else {
        format!(
            "{FORCE} — and at {} it replaces the file while no commit carries either version",
            ticked(&ignored),
        )
    };
    let (message, route) = match head {
        SetupHead::Born if unreported.is_empty() => (
            format!(
                "{} path(s) in the install footprint carried changes that were in no commit \
                 before this run, so committing the install would sweep work `jigc setup` did \
                 not write into `chore(jigc): install jigc workspace config`:\n{listing}\n\
                 {state} — `HEAD` is untouched, so every path listed above still has its \
                 pre-run bytes there",
                paths.len(),
            ),
            format!(
                "commit or stash the work at those path(s) — `git stash -u` where git does \
                 not track them yet — then re-run `jigc setup`; {FORCE}"
            ),
        ),
        // A listed path git's status does not report. The sentence above would say `HEAD`
        // holds its pre-run bytes, which is false of an ignored file, and its route would
        // name two acts git does not perform on one — so this arm says where the bytes
        // actually are (on disk, untouched: the ask is pre-write) and routes each class at
        // the act that works for it.
        SetupHead::Born => {
            let mut acts: Vec<String> = Vec::new();
            if seen > 0 {
                acts.push(
                    "commit or stash the work at the path(s) `git status` does report — \
                     `git stash -u` where git does not track them yet"
                        .to_string(),
                );
            }
            // A flagged path is only ever named by `unseen`, so its home is in hand.
            if let (false, Some(home)) = (flagged.is_empty(), unseen.map(|unseen| unseen.home)) {
                let operands: Vec<String> = flagged
                    .iter()
                    .map(|path| crate::finding::shell_operand(path))
                    .collect();
                let operands = operands.join(" ");
                acts.push(format!(
                    "let git see the change at {} with `{}` and `{}` (one call clears only \
                     one of the two flags), and commit or stash it",
                    ticked(&flagged),
                    crate::finding::git_at(
                        home,
                        &format!("update-index --no-assume-unchanged -- {operands}")
                    ),
                    crate::finding::git_at(
                        home,
                        &format!("update-index --no-skip-worktree -- {operands}")
                    ),
                ));
            }
            if !ignored.is_empty() {
                acts.push(format!(
                    "move the file out of {}, which git ignores and so neither commits \
                     nor stashes",
                    ticked(&ignored),
                ));
            }
            (
                format!(
                    "{} path(s) in the install footprint hold bytes that are in no commit, \
                     so installing would write over work `jigc setup` did not write, or \
                     sweep it into `chore(jigc): install jigc workspace config`:\n{listing}\n\
                     {state} — `HEAD` is untouched, and so is every path listed above: its \
                     bytes are exactly where you left them{unreported}",
                    paths.len(),
                ),
                format!("{} — then re-run `jigc setup`; {force}", acts.join("; ")),
            )
        }
        SetupHead::Unborn { home, riding } => {
            // What the door left alone, said of the place the bytes actually are: before
            // the first write that is the worktree and the index, untouched; at the
            // backstop the install has run, and what is certain is that this run neither
            // staged nor committed the paths it names.
            let left = if install_written {
                "none of the paths listed above was staged or committed by this run"
            } else {
                "the bytes at every path listed above exist only where you left them, and \
                 they are still there"
            };
            // The commit arm has to say what else that commit must carry. Committing only
            // the named path gives the repository a `HEAD`, and from then on an untracked
            // file at an install path is a subject — so a re-run would refuse again, over
            // paths this refusal never named.
            let riding: BTreeSet<&str> = riding.iter().map(String::as_str).collect();
            let with = if riding.is_empty() {
                String::new()
            } else {
                let named: Vec<String> = riding.iter().map(|path| format!("`{path}`")).collect();
                format!(
                    " in one commit with {} — untracked at an install path too: the install \
                     merges into such a file today, and `jigc setup` asks about it as well \
                     once the repository has a commit —",
                    named.join(", "),
                )
            };
            // The unstage names the index it acts on (M53 — the cwd census, the route
            // class): a route is read from wherever the reader stands.
            let unstage = crate::finding::git_at(home, "rm --cached -- <path>");
            // The commit arm is an act git performs, and git does not add an ignored path
            // unless forced to (`git add` exits 1 over one). So where every listed path is
            // ignored there is no commit arm to offer, and where only some are it names
            // the ones it does not cover. Only *ignored* can be unreported here: with no
            // commit, a flagged index entry is still a staged addition `git status` names.
            let route = if ignored.is_empty() {
                format!(
                    "move the file(s) out of those path(s) — `{unstage}` first where you \
                     had staged one — then re-run `jigc setup`; or commit \
                     them{with} and re-run, so git holds your copy before the install runs \
                     over the path; {FORCE}"
                )
            } else if seen == 0 {
                format!(
                    "move the file(s) out of those path(s), then re-run `jigc setup` — git \
                     ignores them, so no commit would take a copy first; {force}"
                )
            } else {
                format!(
                    "move the file(s) out of those path(s) — `{unstage}` first where you \
                     had staged one — then re-run `jigc setup`; or commit the one(s) git does \
                     not ignore{with} and move {} out, since git will not commit an ignored \
                     file, then re-run, so git holds your copy before the install runs over \
                     the path; {force}",
                    ticked(&ignored),
                )
            };
            (
                format!(
                    "{} path(s) in the install footprint hold bytes that are in no commit, \
                     so installing would write over work `jigc setup` did not write, or \
                     sweep it into `chore(jigc): install jigc workspace config`:\n{listing}\n\
                     {state} — this repository has no commit yet, so {left}{unreported}",
                    paths.len(),
                ),
                route,
            )
        }
    };
    Finding::block(
        CarryoverBoundary::Setup.code(),
        message,
        Route::human(route),
    )
}

/// **The listed paths git's own status did not report** — what the
/// [`CarryoverBoundary::Setup`] door found by asking the bytes rather than `git status`
/// (the rc.24 fix pass, the ignored sibling of `(R1, F1)`), and the input
/// [`setup_dirty_install_finding`] words the unreported paths' own sentence and route arms
/// from.
///
/// Two classes, because the act that resolves each is different and neither is the one
/// the reported paths are routed at. The door supplies the facts, the engine words them
/// (the [`decide_carryover`] mold). A path named here that the finding does not list is
/// ignored, so the door may hand over the whole answer it took.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetupUnseen<'a> {
    /// The absolute root of the checkout — the flag-clearing commands name it
    /// ([`crate::finding::git_at`]) on a born `HEAD`, where [`SetupHead`] carries none.
    pub home: &'a Path,
    /// Untracked **and** unreported: a file git ignores, at a path the install replaces
    /// whole. No commit, stash or index entry holds its bytes, and neither `git commit` nor
    /// `git stash` will take them.
    pub ignored: &'a [String],
    /// Tracked, unreported, and not the bytes the index holds: the entry is flagged
    /// assume-unchanged or skip-worktree, so git reports the path clean while the file on
    /// disk says otherwise.
    pub flagged: &'a [String],
}

/// **Which `HEAD` the [`CarryoverBoundary::Setup`] door's refusal is read at** — the one
/// input [`setup_dirty_install_finding`] words its state clause and its route from (the
/// rc.24 fix pass, `(R1, F1)`).
///
/// It is a parameter rather than a second function because the refusal is **one** finding
/// identity with one predicate; what differs is which sentences about it are true. A
/// repository with no commit has no `HEAD` to be untouched and nothing for `git stash` to
/// stand on, so a route written for a born `HEAD` names an act that exits 1 there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetupHead<'a> {
    /// `HEAD` resolves to a commit — every ordinary repository.
    Born,
    /// **No commit yet**: a fresh `git init`, or an orphan branch whose index was emptied.
    Unborn {
        /// The absolute root of the checkout the refusal is about — the route's unstage
        /// names it ([`crate::finding::git_at`]), so the command reaches that index from any
        /// directory.
        home: &'a Path,
        /// The install paths that are **untracked and exempt today** — a file the install
        /// merges into, which rides the repository's first commit with the adopter's bytes
        /// intact (`design/validation.md` → the `setup.dirty-install-path` row). The exemption
        /// ends with the first commit, so the route's *commit* arm has to name them: a commit
        /// that leaves them out turns each into a refusal on the re-run. Empty at the
        /// commit-time backstop, where the install has already merged into and staged them.
        riding: &'a [String],
    },
}

/// Plan the **milestone** `finalize` transaction — the thin sibling of
/// [`plan_finalize`] for the fan-out single-commit boundary (`design/finalize.md` →
/// `fan-out` finalize, single-commit form; `DECISIONS.md` 2026-06-04 → the inc-4
/// fork resolution: M7 ships one synthesized commit).
///
/// It shares [`plan_finalize`]'s phases — the preflight (`staging_dir` exists, `base`
/// reconciled with `head_sha` — see `record_only_advance` below), the empty-commit guard,
/// the phase-4 promote / phase-7 hash sweep ([`plan_promotions`]) and the promote clobber
/// guard — with **two** structural differences a milestone forces:
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
/// task's working area uses, so the shared promote sweep reads it unchanged.
///
/// **The promote clobber guard is shared too** (the rc.24 fix pass, `(R6, D-1)`): after the
/// sweep, a `created` doc whose destination under `repo_root` already holds a file blocks
/// with `finalize.promote-clobber`, exactly as [`plan_finalize`] does
/// ([`plan_milestone_clobber_guard`]). `origins` is the join's per-body provenance
/// ([`crate::milestone::MaterializeOutcome::origins`]) — the merged area carries no
/// manifest of its own — and `claims` what git holds at the promote destinations
/// ([`HomeClaims`]), which the CLI asks of the main checkout. Performs no git and no commit;
/// reads `staging_dir`, and stats the promote destinations under `repo_root`.
// The determinism contract feeds every layer in explicitly (the [`plan_finalize`] precedent);
// bundling the inputs into a params struct would be churn without clarifying the contract.
#[allow(clippy::too_many_arguments)]
pub fn plan_milestone_finalize(
    milestone_id: &str,
    staging_dir: &Path,
    repo_root: &Path,
    base: &BasePin,
    head_sha: &str,
    record_only_advance: bool,
    message: String,
    has_diff: bool,
    schemas: &BTreeMap<String, Schema>,
    origins: &BTreeMap<String, crate::milestone::MergedOrigin>,
    claims: &HomeClaims,
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
    // The clobber guard, at this boundary exactly as at the task one — a sub-task's
    // `created` doc never promotes over an entry already at its destination, however it
    // came to face one ([`plan_milestone_clobber_guard`]).
    plan_milestone_clobber_guard(
        milestone_id,
        repo_root,
        &promote.promotions,
        origins,
        schemas,
        claims,
    )?;
    // A milestone boundary retires nothing — retire is migration-only (a per-task verb) —
    // and carries no owner-artifact set: the owner-artifact exemption/stage is a per-task
    // concern, the milestone-boundary case a separate, deferred one (M45 Inc 8).
    Ok(FinalizePlan::new(
        message,
        promote.promotions,
        promote.hash_updates,
        Vec::new(),
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
/// must have authored its commit doc before the parent finalize renders it. `milestone_id`
/// is carried for that refusal alone: a **never-entered** sub-task has no commit doc as its
/// ordinary state, and `jigc milestone execute <milestone-id>` is the door that prints the
/// launch line which provisions one ([`RenderSubject::SubTask`], M49 Increment 10 / T6).
/// Performs no git and no commit; reads only the sub-task working areas.
pub fn render_subtask_messages(
    sub_ids: &[String],
    tasks_root: &Path,
    commit_schema: &Schema,
    milestone_id: &str,
) -> Result<Vec<String>, Vec<Finding>> {
    let mut messages = Vec::with_capacity(sub_ids.len());
    for sub_id in sub_ids {
        let commit_path = tasks_root
            .join(sub_id)
            .join(DOCS_DIR)
            .join(format!("{}:{sub_id}.md", commit_schema.ty));
        let source = std::fs::read_to_string(&commit_path).map_err(|err| {
            vec![render_io_finding(
                RenderSubject::SubTask {
                    id: sub_id,
                    milestone: milestone_id,
                },
                &commit_path,
                &err,
            )]
        })?;
        let instance = write::instance_from_source(commit_schema, &source)?;
        messages.push(write::render_commit_message(commit_schema, &instance));
    }
    Ok(messages)
}

/// **Shape is part of membership** — the leg every walk over a staging `docs/` owes, asked
/// here so the walks that *act on* what they find cannot disagree with the walks that
/// answer for its complement.
///
/// jigc's writers ([`crate::state::instance_path`] + `fs::write`,
/// [`crate::milestone::materialize`]) emit **regular files**, so a *directory* or a
/// *symlink* wearing a staged instance's `<type>:<slug>.md` name is a third party's. That is
/// already the question [`crate::state::foreign_area_paths`] asks when it enumerates an
/// area's complement, and the question [`crate::state::unwind_docs`],
/// [`crate::state::unwind_merged`] and [`crate::milestone::clear_staged_bodies`] ask before
/// removing a byte — all of them reading the shape **without following symlinks**
/// ([`std::fs::DirEntry::file_type`]), because the question is what jigc wrote here, not
/// what a link points at.
///
/// Both walks asked only the *name*, and at [`plan_promotions`] both shapes were driven at
/// **both** committing doors — `jigc task finalize` and `jigc milestone finalize` — before
/// any fix (M53 Increment 1, the T1 follow-up):
///
/// * a **directory** named `adr:x.md` dead-ended the door at `finalize.promote-io` —
///   *"resolve the read fault … (a disk or permissions problem)"*, which no disk problem
///   caused and no re-run could clear, printed with a host-absolute path;
/// * a **symlink** named `adr:y.md` was read **through**, and its target's bytes were
///   promoted into the commit as a managed doc at **exit 0** — foreign prose landing at
///   `<location>/<slug>.md` under jigc's own name (`promoted docs/decisions/foreign-link.md`,
///   driven at the task door).
///
/// [`plan_owner_artifacts`] is the *same walk over the same directory*, and it takes the
/// leg for seam-unity rather than on a repro: two walks in one file answering one question
/// two ways is how the miss above happened. **Declared bound:** its own cell — a linked
/// `completion-record:x.md` contributing a foreign `owned-location` to the transaction's
/// staged set and the carryover exemption — is *reasoned, not driven*.
///
/// A skipped entry is left exactly where it is, for the door's own complement disposition —
/// refuse · narrate · displace — to answer for, which is the answer those doors already
/// give it.
///
/// **A shape that cannot be stat'd is skipped**, for the same reason the `flatten()` above
/// each call drops an unreadable entry: *not acted on* is this walk's conservative answer,
/// and the complement probe — which propagates its I/O errors rather than swallowing them —
/// is the seam that answers for a path nothing here can read.
fn is_staged_shape(entry: &std::fs::DirEntry) -> bool {
    entry.file_type().is_ok_and(|shape| shape.is_file())
}

/// **Where a staged `<type>:<slug>` doc promotes to** — its canonical repo-relative
/// destination — or `None` for a **transient** doctype, one declaring neither a
/// `placement:` file nor a `location:` and therefore promoted by nothing (the `commit`
/// doc is the only shipped member of that class in either pack).
///
/// `Some(…)` is exactly the predicate *"a finalize of this task writes this doc into the
/// worktree"*, which is why it is a function and not two `if let`s in two files. Two doors
/// ask it: [`plan_promotions`] builds the promote plan from it, and the **amend arm's
/// staged-doc gate** (`cli::task`) refuses over every staged doc it answers `Some` for —
/// the F-10 review's HIGH-1, where `StagePolicy::Amend` stages nothing while the promote
/// phase still wrote each promotion into the worktree, leaving it diverged from `HEAD` at
/// exit 0. A gate keyed on a second spelling of *would this doc promote?* could disagree
/// with the planner about precisely the doc that diverges, which is the drift the one-home
/// rule exists to stop.
#[must_use]
pub fn promote_destination(schema: &Schema, slug: &str) -> Option<String> {
    if let Some(placement) = &schema.placement {
        return Some(placement.file.clone());
    }
    let location = schema.location.as_deref()?;
    Some(format!("{}/{slug}.md", location.trim_end_matches('/')))
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
        .filter(is_staged_shape)
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
        // Resolve the canonical destination through the shared predicate: a **placement**
        // doctype (`design/storage.md` → Placement) lands at its one literal
        // `placement.file` repo-root-relative path (case-preserved, no docs-root, no
        // slug); a `location:` doctype lands at `<location>/<slug>.md`; a type with
        // neither is transient (the commit doc) and is never promoted.
        let Some(destination) = promote_destination(schema, slug) else {
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
        Some(
            format!(
                "resolve the read fault on `{}` (a disk or permissions problem), then \
                 re-run the finalize",
                path.display()
            )
            .into(),
        ),
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
        Some(
            format!(
                "resolve the read fault on the recorded migration source path under `{}` (a \
                 disk or permissions problem), then re-run the finalize",
                task_dir.display()
            )
            .into(),
        ),
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

/// The **unknown work-unit id** refusal every door that resolves a task by id shares —
/// `jigc task <verb> <id>`, `jigc doc <verb> … --task <id>`, `jigc start --task <id>` and
/// `jigc workflow <W> --task <id>` (M51 Increment 6 / T1).
///
/// Same **cause** as [`task_missing_finding`] — a task working area that is not there — so
/// same `code` and same [work-unit key](Unit::location) `task:<id>`, which is the form
/// [command-output-contract.md](../../../design/command-output-contract.md) declares for
/// `finalize.no-task`. It is a separate constructor rather than a reuse because the
/// **message** and the **route** differ and may not be borrowed: the phase-1 sibling speaks
/// about a *path* it was about to finalize, and routes to `jigc start`; these doors were
/// handed an *id* and their recovery is the roster their own family reads.
///
/// **Why the route is a parameter.** Two families of door reach this — a top-level task's
/// roster is `jigc task list`, a milestone sub-task's is `jigc milestone list-tasks` — and
/// the engine is clap-blind by layering, so which roster answers is the CLI's fact, not
/// the engine's. The code, the severity and the target are what transfer; inventing one
/// route here would make one of the two doors lie.
///
/// **Why it lives in the engine.** `finalize.no-task`'s producer is declared
/// `engine::finalize` in `cli::render::FINALIZE_FAMILY`, and that registry is *derived*
/// from a scan of production constructors — a CLI-side mint of the same code would add a
/// second `(code, module)` pair and redden
/// `finalize_family_registry::the_registry_equals_the_production_producer_set`.
pub fn no_such_task_finding(id: &str, route: Route) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.no-task",
        format!("no task `{id}`"),
        Some(Unit::Task(id).location()),
        Some(route),
    )
}

/// The **residual** cell of the same absence — a directory under `.jigc/tasks/` that
/// carries no base pin (M53 Increment 3 / T3;
/// `completions/artifacts/M53/settle-record.md` → D3 as amended by §9 and §14).
///
/// **Same code, same key, a different sentence.** *A task is an area that carries its base
/// pin* ([`crate::state::carries_base_pin`]), so a pin-less directory is not a task that is
/// somehow broken — it is **no task**, exactly like a name nothing answers to. That makes
/// this the second shape of one condition, not a second condition: the code stays
/// `finalize.no-task` and the key stays `(finalize.no-task, task:<id>)`, which is what lets
/// a driver keying on the contract's work-unit form see both shapes without learning a
/// second pair. Only the message and the route differ, because only the **recovery**
/// differs: an unknown id is recovered by reading the roster, a leftover directory by
/// clearing a path.
///
/// **Why the route is composed here and is not a parameter.** Its sibling
/// [`no_such_task_finding`] takes its route from the caller because *which roster answers*
/// is a CLI fact the engine is layering-blind to. This cell has no such split: there is one
/// recovery at all four resolve seams — take what you need out of the directory and delete
/// the rest — and jigc mints **no verb** that clears it, so nothing about it is door-shaped.
/// A parameter here would only create room for four doors to disagree about one act.
///
/// **Why it names no count.** The obvious sentence — *"holding N path(s) jigc did not
/// write"* — is the shipped `finalize.foreign-bytes` mould, and driven over this class's own
/// three shapes that count is `0 / 1 / 0`: an empty leftover holds none, and a
/// `docs/<type>:<slug>.md` is a name jigc's own writer produces, so
/// [`crate::state::foreign_area_paths`] does not claim it either. A number that is zero in
/// two of three cells would tell the reader *there is nothing in there* about a directory
/// that may hold their only copy of something. The sentence names the **path**, and lets
/// them look.
///
/// `area` renders through [`crate::path::repo_relative`] against **`jigc_home`** — the main
/// checkout the `.jigc/` workbench binds to, never the worktree `repo_root` a fanned-out
/// sub-agent calls from, which would print a host-absolute path for every door reached from
/// a worktree (`design/surface-contract.md` law 1).
pub fn residual_task_area_finding(id: &str, jigc_home: &Path, area: &Path) -> Finding {
    let listed = crate::path::repo_relative(jigc_home, area);
    Finding::graded(
        Severity::Blocking,
        "finalize.no-task",
        format!(
            "no task `{id}`: {}",
            crate::state::residual_area_note(&listed, "task")
        ),
        Some(Unit::Task(id).location()),
        Some(crate::state::residual_area_route(&listed)),
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

impl<'a> Unit<'a> {
    /// The unit's bare id — what a route's re-run names.
    fn id(&self) -> &'a str {
        match self {
            Self::Task(id) | Self::Milestone(id) => id,
        }
    }

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
///   discard the task (a cheap, honest exit for a task — `jigc task discard <id> --force`,
///   which also re-mints). Since M50 the exit names the task's **own id** rather than the bare
///   verb, and carries the consent the staged-prose guard requires: the arm binds the id
///   already, and a route whose argv cannot be run is a route floor breach (M43 P6).
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
///
/// **One sha form** (M47 inc-10 / N22). Both constructors of this code used to name the
/// pinned base by its git-abbreviated `short` and HEAD by the full 40-char sha in the same
/// sentence, so the two identities being compared did not look comparable. The engine is
/// fed only HEAD's full sha (the planners take `head_sha: &str`), and abbreviating it here
/// would print a prefix git never minted — so the one form available at this seam, and the
/// one both sides now render in, is [`BasePin::sha`].
fn base_mismatch_finding(unit: Unit, base: &BasePin, head_sha: &str) -> Finding {
    let base_sha = &base.sha;
    let (message, route) = match unit {
        Unit::Task(id) => (
            format!("the task was started at base `{base_sha}` but HEAD is now `{head_sha}`"),
            format!(
                "switch back to `{base_sha}` or discard the task with \
                 `jigc task discard {id} --force`"
            ),
        ),
        Unit::Milestone(_) => (
            format!(
                "the milestone was pinned to base `{base_sha}` but HEAD is now `{head_sha}`, and \
                 the commits landed since move more than milestone-record bookkeeping — the \
                 sub-task worktrees were cut from `{base_sha}`, so combining them onto HEAD \
                 cannot be proven sound"
            ),
            format!(
                "land this milestone's work first (out-of-band git: return HEAD to `{base_sha}`, \
                 run `jigc milestone finalize`, then re-land the newer commits on top), or \
                 re-cut this milestone's work onto the new base (out-of-band git: re-provision \
                 the sub-task worktrees from HEAD and re-apply each sub-task's staged changes)"
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

/// **The amend arm's moved-HEAD refusal** (F-10) — the third constructor of the one
/// `finalize.base-mismatch` code, mutually exclusive with the two above, and here rather
/// than in the CLI because the registry keys a code to **one** producer module.
///
/// The pin form's message says *"the task was started at base `<A>` but HEAD is now `<B>`"*
/// and routes at switching back or discarding. Both halves are wrong for an amend in the
/// same way: an ordinary task's base is the history its work sits **on**, so switching back
/// is a real exit; an amend's pin is the commit it **rewrites**, so once `HEAD` has moved
/// the commit whose subject line the agent read and re-authored against is no longer the one
/// `--amend` would rewrite, and *"switch back"* would silently mean a different repository
/// state.
pub fn amend_base_mismatch_finding(id: &str, pinned: &str, head: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.base-mismatch",
        format!(
            "`HEAD` is no longer the commit this amend was minted against — it pinned \
             `{pinned}` and `HEAD` is now `{head}`, so the message this task authored would \
             rewrite a different commit"
        ),
        Some(Unit::Task(id).location()),
        Some(crate::finding::Route::human(format!(
            "start the repair again against the commit that is there now (`jigc task discard \
             {} --force`, then `jigc task amend`), or return `HEAD` to `{pinned}` first",
            crate::finding::shell_token(id),
        ))),
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
/// [keyed at the same work unit](Unit::location). It renders the same **one sha form** as
/// the pin constructor (M47 inc-10 / N22).
fn base_overlap_finding(
    unit: Unit,
    base: &BasePin,
    head_sha: &str,
    overlapping: &[&str],
) -> Finding {
    let paths = overlapping.join("`, `");
    // Task-only by its own message (*"the task was started at base …"*) and by its one call
    // site, so the abandon exit carries the real id rather than a placeholder the caller can
    // derive (M43 P6 — a derivable placeholder on a blocking route is a route floor breach).
    let (Unit::Task(id) | Unit::Milestone(id)) = unit;
    Finding::graded(
        Severity::Blocking,
        "finalize.base-mismatch",
        format!(
            "the task was started at base `{}` but HEAD is now `{head_sha}`, and the moved \
             history overlaps the task's work on `{paths}`",
            base.sha
        ),
        Some(unit.location()),
        Some(
            format!(
                "resolve the overlap on `{paths}` against the new history, or discard the task \
             with `jigc task discard {id} --force`"
            )
            .into(),
        ),
    )
}

/// The empty-commit abort (`finalize.md` → Commit-doc rendering → Empty commit):
/// validate passed but the staged diff is empty. No empty commits. Its subject is the work
/// unit that produced nothing, so it [keys at it](Unit::location) — shared by both planners,
/// so the unit is passed in, never guessed from a path.
///
/// **Both exits, and the unit's own verb family** (M47 inc-10 / N17). The shipped route was
/// *"make a change, then re-run `jigc task finalize`"* — one exit for a block whose other
/// honest answer is *this task produced nothing; abandon it* (`jigc task discard`, already
/// the recorded abandon verb on the write path), and the **task** verb served to a blocked
/// milestone too. That is the same unit-blindness M42 removed from this file's sibling
/// [`base_mismatch_finding`], swept here: the task arm re-runs `jigc task finalize <id>`, the
/// milestone arm `jigc milestone finalize <id>`, each with its own abandon verb.
fn empty_commit_finding(unit: Unit) -> Finding {
    let (message, route) = match unit {
        Unit::Task(id) => (
            "task validated but produced no diff — nothing to finalize".to_string(),
            format!(
                "make a change, then re-run `jigc task finalize {id}` — or, if the task is done \
                 with nothing to show, abandon it with `jigc task discard {id} --force`"
            ),
        ),
        Unit::Milestone(id) => (
            "the milestone validated but produced no diff — nothing to finalize".to_string(),
            format!(
                "make a change in a sub-task, then re-run `jigc milestone finalize {id}` — or, \
                 if the milestone is being abandoned, settle its record with \
                 `jigc milestone discard {id}`"
            ),
        ),
    };
    Finding::graded(
        Severity::Blocking,
        "finalize.empty-commit",
        message,
        Some(unit.location()),
        Some(route.into()),
    )
}

/// Whose staged commit doc a render was reading — the discriminator the **absent** arm of
/// [`render_io_finding`] keys on (M49 Increment 10 / T6).
///
/// One code, two units, and an absence that means something different in each: a serial task's
/// commit doc is provisioned once, at compose, so its absence is a working area that lost a
/// file; a fan-out sub-task's is provisioned on its **first re-entry**, so its absence is the
/// ordinary state of a sub-task nobody ever entered. The recoveries differ accordingly, which
/// is why the subject is carried rather than guessed from the path — the [`Unit`] precedent
/// (M42 T3), applied to the render.
#[derive(Clone, Copy)]
enum RenderSubject<'a> {
    /// The finalizing task's own commit doc — [`plan_finalize`] phase 3.
    Task { id: &'a str },
    /// A fan-out sub-task's commit doc — [`render_subtask_messages`]. It carries the
    /// **milestone** id too, because `jigc milestone execute <m>` is the door that prints the
    /// sub-task's launch line (`design/workflow-dialect.md` → Emitted format rule 4), and that
    /// line is the act the operator is owed here.
    SubTask { id: &'a str, milestone: &'a str },
}

impl RenderSubject<'_> {
    /// The message + route for a commit doc that is simply **not there**. Neither route names
    /// a `.jigc/tasks/…` path: that tree is jigc's own gitignored workbench, so handing it to
    /// the operator as their subject is a direction nobody can take
    /// (`design/surface-contract.md` → law 1, the route floor).
    fn absent(self) -> (String, Route) {
        match self {
            RenderSubject::Task { id } => (
                format!(
                    "task `{id}` stages no commit doc — `commit:{id}` is not in its working \
                     area, so there is nothing to render into the git message"
                ),
                Route::mechanical(
                    ["jigc", "doc", "list", "--task", id],
                    format!(
                        " — it lists what the task still stages; a commit doc is provisioned \
                         once, at compose, so a task whose copy is gone is abandoned with \
                         `jigc task discard {id} --force` and the work re-started with \
                         `jigc start`"
                    ),
                ),
            ),
            RenderSubject::SubTask { id, milestone } => (
                format!(
                    "no commit doc for sub-task `{id}` — a sub-task's is provisioned on its \
                     first re-entry and authored there, and its working area holds none"
                ),
                Route::mechanical(
                    ["jigc", "milestone", "execute", milestone],
                    format!(
                        " — it prints sub-task `{id}`'s launch line; run that in the \
                         sub-task's worktree, author the commit doc, then re-run the finalize \
                         — or settle the sub-task with `jigc task discard {id} --force`"
                    ),
                ),
            ),
        }
    }
}

/// A blocking finding for a failed read of the staged commit doc during render. Its subject is
/// the staged **file**, so it [keys at its path](file_location) on both arms.
///
/// **The two read outcomes are not one fact** (M49 Increment 10 / T6). A genuine
/// permissions/disk fault keeps the shipped text — the read really did fail, and resolving the
/// fault really is the recovery. `NotFound` is not that: nothing is faulty, the doc was never
/// written, and the shipped text told the operator to fix a disk problem on a path in jigc's
/// own workbench — a law-1 lie in both halves, driven live on the `squash: false` fan-out path
/// where a never-entered sub-task reaches it as its *ordinary* state. So absence answers per
/// [`RenderSubject`], with a route the operator can run verbatim.
fn render_io_finding(subject: RenderSubject<'_>, path: &Path, err: &std::io::Error) -> Finding {
    let (message, route) = if err.kind() == std::io::ErrorKind::NotFound {
        subject.absent()
    } else {
        (
            format!(
                "could not read the staged commit doc `{}`: {err}",
                path.display()
            ),
            format!(
                "resolve the read fault on `{}` (a disk or permissions problem), then \
                 re-run the finalize",
                path.display()
            )
            .into(),
        )
    };
    Finding::graded(
        Severity::Blocking,
        "finalize.render-io",
        message,
        Some(file_location(path.display())),
        Some(route),
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

    /// The route-floor seam-sweep, exercised through the real `finalize.*` I/O producers
    /// (M43 surface census): each I/O fault carries a recovery route and drives cleanly
    /// through the [`Findings`](crate::finding::Findings) serialization seam — the traffic
    /// whose absence let all four ship route-less (`DECISIONS.md` 2026-07-17 → the
    /// seam-sweep rule). A route-less finding would panic the route-floor assert here.
    #[test]
    fn finalize_io_findings_carry_a_recovery_route_through_the_seam() {
        use crate::finding::Findings;
        let err = || std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let task_dir = Path::new(".jigc/tasks/t1");
        let findings = vec![
            render_io_finding(
                RenderSubject::Task { id: "t1" },
                Path::new(".jigc/tasks/t1/commit.md"),
                &err(),
            ),
            promote_io_finding(Path::new(".jigc/tasks/t1/docs/adr:x.md"), &err()),
            provenance_io_finding(Unit::Task("t1"), task_dir, &err()),
            source_path_io_finding(Unit::Task("t1"), task_dir, &err()),
        ];
        for f in &findings {
            assert_eq!(f.severity, Severity::Blocking);
            let route = f.route.as_ref().expect("an I/O fault names its recovery");
            assert!(
                route.as_str().contains("re-run the finalize"),
                "route names the re-run: {route}"
            );
        }
        // Drives the route-floor + key-uniqueness seams; a route-less finding panics here.
        let json = serde_json::to_string(&Findings::from(findings)).expect("findings serialize");
        assert!(json.contains("finalize.render-io"));
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
                crate::tempname::unique_nanos(),
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

    const COMMIT_YAML: &[u8] = include_bytes!(pack_path!(dev, "schemas/commit.yaml"));
    const ADR_YAML: &[u8] = include_bytes!(pack_path!(dev, "schemas/adr.yaml"));
    const CHANGELOG_YAML: &[u8] = include_bytes!(pack_path!(dev, "schemas/changelog.yaml"));

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

    /// A throwaway `completion-record` doctype carrying an engine-native `owned-location`
    /// `owner-artifact` field on its `meta` header (the #5 gate target) plus a body slot,
    /// persisted to `completions/` — the substrate for the `owner_artifacts` collection
    /// walk. A FIXTURE, never pack content.
    fn completion_record_schema() -> Schema {
        let yaml = b"\
type: completion-record
location: completions/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: owner-artifact, type: owned-location }
  - id: body
    slot: { hint: \"The record body.\" }
";
        crate::schema::load_schema(yaml).expect("completion-record fixture loads")
    }

    /// Stage a filled `completion-record:<slug>` doc naming `owner_artifact` on its meta
    /// header at `<task_dir>/docs/completion-record:<slug>.md` (a body slot filled so the
    /// instance parses cleanly). The engine-native owned-location value is what
    /// [`plan_owner_artifacts`] collects.
    fn stage_filled_completion_record(task_dir: &Path, schema: &Schema, slug: &str, owner: &str) {
        use crate::field_block::{Field, Value};
        use crate::write::SectionContent;

        let instance = write::Instance {
            title: slug.to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![Field {
                        key: "owner-artifact".to_string(),
                        value: Value::Scalar(owner.to_string()),
                    }],
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    slot: Some("The audit landed green.".to_string()),
                    ..Default::default()
                },
            ],
        };
        let bytes = write::render(schema, &instance);
        let path = state::instance_path(task_dir, &schema.ty, slug);
        state::persist(&path, bytes.as_bytes()).expect("persist staged completion-record");
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
                &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
            &HomeClaims::default(),
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
                &HomeClaims::default(),
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
                &HomeClaims::default(),
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

    /// The **migration-case** clobber route (M43 inc-6 T3; RC-lacon findings-verification
    /// → A4; `design/surface-contract.md` → law 2/3): a migration whose recorded foreign
    /// `source-path` is a **different** file than the occupied destination still blocks —
    /// and the finding names **both** the occupied destination and the recorded source,
    /// teaches **no raw removal** (never "remove"/`git rm` — verbatim the A4 trap that
    /// staged a silent `git rm`), routes through jigc verbs only, and names the
    /// post-resolution continuation ending at `--approve` (plain finalize → review →
    /// approve). The `(code, target)` key is unchanged: `(finalize.promote-clobber,
    /// <destination>)`.
    #[test]
    fn a_migration_clobber_names_the_recorded_source_and_the_approve_continuation() {
        let root = TempRoot::new("clobber-migration-route");
        let task_dir = root.path().join("tasks").join("migrate-adr-collide");
        let schema = stage_filled_commit(&task_dir, "migrate-adr-collide");
        stage_filled_adr(&task_dir, "single-node-cache");
        state::record_doc_provenance(
            &task_dir,
            "adr:single-node-cache",
            state::Provenance::Created,
        )
        .expect("record created provenance");
        // The recorded foreign source is a DIFFERENT file than the promote destination —
        // the title-slug-collision migration (the same-path case never reaches the guard).
        state::persist(&task_dir.join("source-path"), b"notes/old-decision.md")
            .expect("record the migration source path");
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
            "migrate-adr-collide",
            &schemas(),
            &HomeClaims::default(),
        )
        .expect_err("a colliding migration (source != destination) must still block");
        assert_eq!(findings.len(), 1, "exactly one clobber finding");
        assert_eq!(findings[0].code, "finalize.promote-clobber");
        assert_eq!(
            target(&findings[0]),
            Some("decisions/single-node-cache.md"),
            "the (code, target) key is unchanged — it keys at the destination file",
        );
        assert!(
            findings[0]
                .message
                .contains("decisions/single-node-cache.md")
                && findings[0].message.contains("notes/old-decision.md"),
            "the block names BOTH the occupied destination and the recorded source: {:?}",
            findings[0].message
        );
        let route = findings[0].route.as_deref().expect("blocking ⇒ routed");
        assert!(
            route.contains("notes/old-decision.md"),
            "the route discriminates on the recorded source, naming it: {route:?}"
        );
        assert!(
            route.contains("--approve"),
            "the route names the post-resolution continuation including `--approve`: {route:?}"
        );
        assert!(
            route.contains("--slug") && route.contains("jigc migrate"),
            "the route repairs through jigc verbs (`--slug`, adopt via `jigc migrate`): {route:?}"
        );
        for surface in [&findings[0].message, route] {
            assert!(
                !surface.to_lowercase().contains("remove")
                    && !surface.to_lowercase().contains("delete")
                    && !surface.contains("git rm"),
                "the clobber block never teaches raw removal (RC-lacon A4/A7): {surface:?}"
            );
        }
    }

    /// The **non-migration** clobber route (M43 inc-6 T3; `design/surface-contract.md` →
    /// law 2): an ordinary create collision (no recorded `source-path`) routes through
    /// jigc verbs only — retitle / an explicit `--slug` / adopt the occupant via
    /// `jigc migrate` — ending at the `jigc task finalize` re-run, never a raw-removal
    /// instruction.
    #[test]
    fn a_non_migration_clobber_routes_through_jigc_verbs_only() {
        let root = TempRoot::new("clobber-plain-route");
        let task_dir = root.path().join("tasks").join("record-decision");
        let schema = stage_filled_commit(&task_dir, "record-decision");
        stage_filled_adr(&task_dir, "single-node-cache");
        state::record_doc_provenance(
            &task_dir,
            "adr:single-node-cache",
            state::Provenance::Created,
        )
        .expect("record created provenance");
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
            "record-decision",
            &schemas(),
            &HomeClaims::default(),
        )
        .expect_err("a created doc clobbering a committed managed doc must block");
        assert_eq!(findings.len(), 1, "exactly one clobber finding");
        let route = findings[0].route.as_deref().expect("blocking ⇒ routed");
        assert!(
            route.contains("retitle") && route.contains("--slug"),
            "the managed-collision arm routes through retitle / `--slug`: {route:?}"
        );
        assert!(
            route.contains("jigc migrate"),
            "the foreign-occupant arm routes through adoption via `jigc migrate`: {route:?}"
        );
        assert!(
            route.contains("jigc task finalize"),
            "the route ends at the finalize re-run: {route:?}"
        );
        for surface in [&findings[0].message, route] {
            assert!(
                !surface.to_lowercase().contains("remove")
                    && !surface.to_lowercase().contains("delete")
                    && !surface.contains("git rm"),
                "the clobber block never teaches raw removal (RC-lacon A4/A7): {surface:?}"
            );
        }
    }

    /// Plant an entry that is **not a regular file** at `path` — the shape axis of
    /// `(R6, D-7)`. Returns the path a write *through* the entry would create or change,
    /// when it has one.
    #[cfg(unix)]
    fn plant_foreign(path: &Path, shape: &str) -> Option<PathBuf> {
        std::fs::create_dir_all(path.parent().expect("a home directory")).expect("mk home dir");
        let beside = |name: &str| path.parent().unwrap().join(name);
        match shape {
            "dangling-link" => {
                std::os::unix::fs::symlink("nowhere.md", path).expect("link");
                Some(beside("nowhere.md"))
            }
            "live-link" => {
                let target = beside("the-real-file.md");
                std::fs::write(&target, b"somebody else's bytes\n").expect("the link's target");
                std::os::unix::fs::symlink("the-real-file.md", path).expect("link");
                Some(target)
            }
            "device-link" => {
                std::os::unix::fs::symlink("/dev/null", path).expect("link");
                None
            }
            "directory" => {
                std::fs::create_dir(path).expect("mk a directory at the home");
                None
            }
            "fifo" => {
                let made = std::process::Command::new("mkfifo")
                    .arg(path)
                    .status()
                    .expect("spawn mkfifo");
                assert!(made.success(), "mkfifo");
                None
            }
            other => panic!("not a shape: {other}"),
        }
    }

    /// The shapes, each with the noun the refusal names it by.
    #[cfg(unix)]
    const FOREIGN_SHAPES: [(&str, &str); 5] = [
        ("dangling-link", "a symbolic link"),
        ("live-link", "a symbolic link"),
        ("device-link", "a symbolic link"),
        ("directory", "a directory"),
        ("fifo", "a special file"),
    ];

    /// Assert a shape refusal's surfaces teach no raw removal and name the entry.
    #[cfg(unix)]
    fn assert_shape_surfaces(finding: &Finding, destination: &str, noun: &str, cell: &str) {
        assert_eq!(finding.code, "finalize.promote-clobber", "{cell}");
        assert_eq!(finding.severity, Severity::Blocking, "{cell}");
        assert_eq!(
            target(finding),
            Some(destination),
            "{cell}: keyed at the destination path — the key of every other occupant",
        );
        assert!(
            finding.message.contains(noun) && finding.message.contains(destination),
            "{cell}: the message names the entry and the home; got: {}",
            finding.message,
        );
        let route = finding.route.as_deref().expect("blocking ⇒ routed");
        for surface in [finding.message.as_str(), route] {
            let lower = surface.to_lowercase();
            assert!(
                !lower.contains("remove")
                    && !lower.contains("delete")
                    && !surface.contains("git rm"),
                "{cell}: never teaches raw removal (RC-lacon A4/A7): {surface:?}",
            );
            assert!(
                !surface.contains("jigc migrate"),
                "{cell}: a link is not a file to adopt — no `jigc migrate` span: {surface:?}",
            );
        }
    }

    /// **A promote lands a regular file at exactly its home, or the plan refuses** (the
    /// rc.24 fix pass, `(R6, D-7)`; `design/finalize.md` → 4. Promote). Over every entry
    /// that is not a regular file × every way a task can hold the doc — minted, copied in,
    /// unrecorded — the planner blocks with `finalize.promote-clobber` keyed at the
    /// destination and writes nothing, at the home or through it.
    ///
    /// Until then the guard asked `is_file()`, which follows links: a dangling link read as
    /// a free home (the promote then wrote through it), and an `edited-from-base` doc was
    /// never asked at all (the promote wrote its edit through a live link into the target).
    ///
    /// The route is the exit **this** provenance has: a minted doc is re-slugged in the
    /// task or lands once the home is free; a copied-in one lands over the regular file
    /// itself; an unrecorded one is offered only the freed home.
    #[cfg(unix)]
    #[test]
    fn finalize_plan_refuses_every_promotion_over_an_entry_that_is_not_a_regular_file() {
        let provenances = [
            Some(state::Provenance::Created),
            Some(state::Provenance::EditedFromBase),
            None,
        ];
        for (shape, noun) in FOREIGN_SHAPES {
            for provenance in provenances {
                let cell = format!("{shape} · {provenance:?}");
                let root = TempRoot::new("shape-task");
                let task_dir = root.path().join("tasks").join("record-decision");
                let schema = stage_filled_commit(&task_dir, "record-decision");
                stage_filled_adr(&task_dir, "single-node-cache");
                if let Some(provenance) = provenance {
                    state::record_doc_provenance(&task_dir, "adr:single-node-cache", provenance)
                        .expect("record provenance");
                }
                let destination = "decisions/single-node-cache.md";
                let through = plant_foreign(&root.path().join(destination), shape);
                let through_before = through.as_ref().and_then(|p| std::fs::read(p).ok());
                let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

                let findings = plan_finalize(
                    &task_dir,
                    root.path(),
                    &base(),
                    &base().sha,
                    &clean,
                    true,
                    &schema,
                    "record-decision",
                    &schemas(),
                    &HomeClaims::default(),
                )
                .expect_err("an entry that is not a regular file refuses the promotion");
                assert_eq!(findings.len(), 1, "{cell}: exactly one finding");
                let finding = &findings[0];
                assert_shape_surfaces(finding, destination, noun, &cell);
                assert!(
                    finding.message.contains("adr:single-node-cache"),
                    "{cell}: the message names the doc; got: {}",
                    finding.message,
                );
                let route = finding.route.as_deref().expect("a route");
                let rename = "`jigc doc rename adr:single-node-cache --to \"<title>\" --task record-decision`";
                assert_eq!(
                    route.contains(rename),
                    provenance == Some(state::Provenance::Created),
                    "{cell}: the in-task rename is handed back exactly for a doc this task \
                     minted — a committed identity is not re-slugged in a task; got: {route}",
                );
                assert_eq!(
                    route.contains("regular file itself"),
                    provenance == Some(state::Provenance::EditedFromBase),
                    "{cell}: a copied-in doc lands over the regular file itself; got: {route}",
                );
                assert_eq!(
                    route.contains("out of the doc's home"),
                    provenance != Some(state::Provenance::EditedFromBase),
                    "{cell}: a freed home is an exit for every doc but a copied-in one; got: \
                     {route}",
                );
                assert!(
                    route.contains("`jigc task finalize record-decision`"),
                    "{cell}: the route ends at this task's own boundary, by id; got: {route}",
                );
                if let Some(through) = &through {
                    assert_eq!(
                        std::fs::read(through).ok(),
                        through_before,
                        "{cell}: the planner writes nothing through the entry",
                    );
                }
            }
        }
    }

    /// **A fixed identity has no other id to take**: a `placement` singleton minted over a
    /// link at its literal home is refused with the freed home as its only exit — the
    /// in-task rename refuses a singleton outright, and a route never hands back a command
    /// that refuses.
    #[cfg(unix)]
    #[test]
    fn a_fixed_identity_over_a_link_is_routed_at_the_freed_home_only() {
        let root = TempRoot::new("shape-singleton");
        let task_dir = root.path().join("tasks").join("record-change");
        let schema = stage_filled_commit(&task_dir, "record-change");
        state::persist(
            &state::instance_path(&task_dir, "changelog", "changelog"),
            b"# Changelog\n\nauthored\n",
        )
        .expect("stage the singleton instance");
        state::record_doc_provenance(&task_dir, "changelog:changelog", state::Provenance::Created)
            .expect("record created provenance");
        plant_foreign(&root.path().join("CHANGELOG.md"), "dangling-link");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        let findings = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &schema,
            "record-change",
            &schemas(),
            &HomeClaims::default(),
        )
        .expect_err("a link at a placement home refuses");
        assert_eq!(findings.len(), 1);
        assert_shape_surfaces(&findings[0], "CHANGELOG.md", "a symbolic link", "singleton");
        let route = findings[0].route.as_deref().expect("a route");
        assert!(
            !route.contains("jigc doc rename") && route.contains("out of the doc's home"),
            "the freed home is the only exit; got: {route}",
        );
    }

    /// **A migration is refused the same way, carve-out or not.** A migration whose doc
    /// would land on a link refuses with its own continuation (`--approve`) named — and the
    /// in-place carve-out, which licenses rewriting the foreign *file* at the doc's own
    /// home, is no licence to write through a link standing at that path: source ==
    /// destination over a link refuses too.
    #[cfg(unix)]
    #[test]
    fn a_migration_over_a_link_is_refused_even_in_place() {
        for (what, source_path) in [
            ("a different source", "notes/old-decision.md"),
            ("the in-place source", "decisions/single-node-cache.md"),
        ] {
            let root = TempRoot::new("shape-migration");
            let task_dir = root.path().join("tasks").join("migrate-adr");
            let schema = stage_filled_commit(&task_dir, "migrate-adr");
            stage_filled_adr(&task_dir, "single-node-cache");
            state::record_doc_provenance(
                &task_dir,
                "adr:single-node-cache",
                state::Provenance::Created,
            )
            .expect("record created provenance");
            state::persist(&task_dir.join("source-path"), source_path.as_bytes())
                .expect("record the migration source path");
            let destination = "decisions/single-node-cache.md";
            let through = plant_foreign(&root.path().join(destination), "dangling-link");
            let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

            let findings = plan_finalize(
                &task_dir,
                root.path(),
                &base(),
                &base().sha,
                &clean,
                true,
                &schema,
                "migrate-adr",
                &schemas(),
                &HomeClaims::default(),
            )
            .expect_err("a migration never lands through a link");
            assert_eq!(findings.len(), 1, "{what}");
            assert_shape_surfaces(&findings[0], destination, "a symbolic link", what);
            let route = findings[0].route.as_deref().expect("a route");
            assert!(
                route.contains("`jigc task finalize migrate-adr --approve`")
                    && route.contains("fidelity diff"),
                "{what}: the route ends at the migration's own continuation; got: {route}",
            );
            assert!(
                !through.expect("the link's target path").exists(),
                "{what}: nothing was written through the link",
            );
        }
    }

    /// **The sink's refusal is the planner's identity** ([`promote_sink_refusal`]): the same
    /// code keyed at the same destination path, so a driver reads one key whichever of the
    /// two caught the entry — and its route says what the sink can truthfully say, that the
    /// entry appeared after the plan and the re-run names the unit's exit.
    #[test]
    fn the_sink_refusal_carries_the_planners_code_and_key() {
        let finding = promote_sink_refusal(
            Path::new("/repo"),
            "decisions/single-node-cache.md",
            crate::store::ForeignEntry::Symlink,
        );
        assert_eq!(finding.code, "finalize.promote-clobber");
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(target(&finding), Some("decisions/single-node-cache.md"));
        let route = finding.route.as_deref().expect("a route");
        assert!(
            route.contains("nothing was committed and the promote was rolled back")
                && route.contains("re-run the finalize"),
            "the sink speaks for its own state; got: {route}",
        );
        for surface in [finding.message.as_str(), route] {
            assert!(
                !surface.contains("/repo"),
                "repo-relative paths only: {surface:?}",
            );
        }
    }

    /// **A store door's refusal is the planner's identity too** ([`store_home_refusal`]):
    /// the same code keyed at the same home, over every shape — so a doc whose home is not
    /// a regular file reads as one fault at `jigc task finalize`, at `jigc rename` and at a
    /// relocation. What differs is what the door says it withheld and where its route ends:
    /// there is no unit, the exit is the regular file **committed** at the home, and the
    /// door's own command follows it. It teaches no removal and prints no host path.
    #[test]
    fn the_store_door_refusal_carries_the_planners_code_and_key() {
        use crate::store::ForeignEntry;
        for (shape, noun, bare) in [
            (ForeignEntry::Symlink, "a symbolic link", "link"),
            (ForeignEntry::Directory, "a directory", "directory"),
            (ForeignEntry::Other, "a special file", "special file"),
        ] {
            let finding = store_home_refusal(
                Path::new("/repo"),
                "decisions/single-node-cache.md",
                shape,
                "`adr:single-node-cache` is not renamed",
                "re-run `jigc rename adr:single-node-cache --to Cache`",
            );
            assert_eq!(finding.code, "finalize.promote-clobber", "{shape:?}");
            assert_eq!(finding.severity, Severity::Blocking, "{shape:?}");
            assert_eq!(
                target(&finding),
                Some("decisions/single-node-cache.md"),
                "{shape:?}: keyed at the home, like the planner's and the sink's",
            );
            assert!(
                finding.message.contains(noun)
                    && finding.message.contains("decisions/single-node-cache.md")
                    && finding
                        .message
                        .ends_with("so `adr:single-node-cache` is not renamed"),
                "{shape:?}: the message names the entry, the home and what the door \
                 withheld; got: {}",
                finding.message,
            );
            assert!(
                !finding.message.contains("promoted"),
                "{shape:?}: nothing is promoted at a store door; got: {}",
                finding.message,
            );
            let route = finding.route.as_deref().expect("a route");
            assert!(
                route.contains(&format!("the {bare} at `decisions/single-node-cache.md`"))
                    && route.contains("as a regular file")
                    && route.contains("commit that")
                    && route
                        .ends_with("then re-run `jigc rename adr:single-node-cache --to Cache`"),
                "{shape:?}: the route names the state the home has to be in — committed — \
                 and ends at the door's own command; got: {route}",
            );
            for surface in [finding.message.as_str(), route] {
                let lower = surface.to_lowercase();
                assert!(
                    !lower.contains("remove") && !lower.contains("delete"),
                    "{shape:?}: never teaches raw removal: {surface:?}",
                );
                assert!(
                    !surface.contains("/repo"),
                    "{shape:?}: repo-relative paths only: {surface:?}",
                );
            }
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
            &HomeClaims::default(),
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

    /// (M45 Inc 8 T1) `plan_finalize` populates [`FinalizePlan::owner_artifacts`] with each
    /// staged instance's recorded `owned-location` value — the second walk mirroring the #5
    /// presence gate's field walk. A clean report + a diff signal drive the planner past
    /// phase 2, and the plan carries the recorded owner-artifact path the CLI exempts from
    /// the carryover gate (and, T2, stages in-transaction). A staged doc with **no**
    /// owned-location field contributes nothing (the omitting-context inert path).
    #[test]
    fn plan_finalize_collects_owner_artifacts_from_a_staged_owned_location_record() {
        let root = TempRoot::new("owner-artifacts");
        let task_dir = root.path().join("tasks").join("record-completion");

        // A transient commit doc (no owned-location — contributes nothing) + a
        // completion-record naming an owner-artifact under the owned home.
        stage_filled_commit(&task_dir, "record-completion");
        let cr = completion_record_schema();
        stage_filled_completion_record(&task_dir, &cr, "m45", "completions/artifacts/M45/audit.md");

        let mut schemas = schemas();
        schemas.insert(cr.ty.clone(), cr.clone());

        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());
        let plan = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &clean,
            true,
            &commit_schema(),
            "record-completion",
            &schemas,
            &HomeClaims::default(),
        )
        .expect("a clean report + diff drive the planner past phase 2");

        assert_eq!(
            plan.owner_artifacts,
            vec!["completions/artifacts/M45/audit.md".to_string()],
            "the plan collects exactly the recorded owned-location path — the carryover \
             exemption + (T2) stage set; the commit doc's no-owned-location contributes none",
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
            root.path(),
            &base(),
            &base().sha,
            false,
            "Finalize milestone page-rework (1 sub-task)\n".to_string(),
            true,
            &with_placement,
            &no_origins(),
            &HomeClaims::default(),
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
        let messages = render_subtask_messages(&id_sorted, &tasks_root, &schema, "cache-rework")
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
        let rev_messages = render_subtask_messages(&reversed, &tasks_root, &schema, "cache-rework")
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
            "cache-rework",
        )
        .expect_err("a sub-task with no authored commit doc blocks");
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].code, "finalize.render-io");
        assert_eq!(missing[0].severity, Severity::Blocking);
    }

    /// M49 Increment 10 / T6 — **the render's two read outcomes are two different facts**,
    /// over both subjects: a `NotFound` names the unit and routes at an act the operator can
    /// run, and every other I/O fault keeps the shipped read-fault text verbatim.
    ///
    /// The three cells reachable through the real binary are driven there
    /// (`crates/cli/tests/finalize_render_io_absent.rs`). **The sub-task's unreadable cell is
    /// only reachable here**: through the binary the milestone `join` reads every staged doc
    /// body before the per-sub-task render is called, so an unreadable `commit:<sub>` is
    /// answered by `milestone.area-io` first — this is that cell's standing home, driven
    /// through the production [`render_subtask_messages`] over a real unreadable path.
    #[test]
    fn the_render_finding_tells_an_absent_commit_doc_from_a_read_fault() {
        let root = TempRoot::new("render-io-subject");
        let tasks_root = root.path().join("tasks");
        stage_subtask_commit(&tasks_root, "alpha-area", "rework the alpha path");
        let schema = commit_schema();
        let ids = vec!["alpha-area".to_string(), "beta-area".to_string()];

        // --- the sub-task, absent: the never-entered state, not a fault -------------------
        let absent = render_subtask_messages(&ids, &tasks_root, &schema, "cache-rework")
            .expect_err("a sub-task with no commit doc blocks");
        assert_eq!(absent.len(), 1);
        assert_eq!(absent[0].code, "finalize.render-io");
        assert_eq!(
            absent[0].message,
            "no commit doc for sub-task `beta-area` — a sub-task's is provisioned on its \
             first re-entry and authored there, and its working area holds none",
        );
        let route = absent[0]
            .route
            .as_ref()
            .expect("the refusal carries a route")
            .as_str()
            .to_string();
        assert!(
            route.starts_with("`jigc milestone execute cache-rework`"),
            "the route leads with the door that prints the launch line; got: {route}",
        );
        assert!(
            route.contains("jigc task discard beta-area"),
            "the route names the settle exit; got: {route}",
        );
        assert!(
            !route.contains(".jigc/tasks") && !route.contains("a disk or permissions problem"),
            "an absent doc is neither a disk fault nor a workbench path to fix; got: {route}",
        );

        // --- the sub-task, unreadable: the shipped read-fault text, verbatim --------------
        // A directory in the doc's place fails the read with something other than `NotFound`
        // on every platform — the permissions/disk class the shipped text was written for.
        let blocked_path =
            state::instance_path(&tasks_root.join("beta-area"), &schema.ty, "beta-area");
        std::fs::create_dir_all(&blocked_path).expect("put a directory in the doc's place");
        let unreadable = render_subtask_messages(&ids, &tasks_root, &schema, "cache-rework")
            .expect_err("an unreadable commit doc blocks");
        assert_eq!(unreadable.len(), 1);
        assert_eq!(unreadable[0].code, "finalize.render-io");
        assert!(
            unreadable[0]
                .message
                .starts_with("could not read the staged commit doc `"),
            "a genuine fault keeps the shipped message; got: {}",
            unreadable[0].message,
        );
        let fault_route = unreadable[0]
            .route
            .as_ref()
            .expect("the fault carries a route")
            .as_str()
            .to_string();
        assert!(
            fault_route.contains("(a disk or permissions problem), then re-run the finalize"),
            "a genuine fault keeps the shipped route; got: {fault_route}",
        );

        // --- the serial task, absent: the other subject's own message + route -------------
        let missing = std::io::Error::from(std::io::ErrorKind::NotFound);
        let task = render_io_finding(
            RenderSubject::Task {
                id: "warm-the-cache",
            },
            Path::new(".jigc/tasks/warm-the-cache/docs/commit:warm-the-cache.md"),
            &missing,
        );
        assert_eq!(
            task.message,
            "task `warm-the-cache` stages no commit doc — `commit:warm-the-cache` is not in \
             its working area, so there is nothing to render into the git message",
        );
        let task_route = task
            .route
            .as_ref()
            .expect("the refusal carries a route")
            .as_str()
            .to_string();
        assert!(
            task_route.starts_with("`jigc doc list --task warm-the-cache`"),
            "the route leads with the read of what the task stages; got: {task_route}",
        );
        assert!(
            !task_route.contains(".jigc/tasks"),
            "no route names jigc's own workbench path as the operator's subject; \
             got: {task_route}",
        );
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
            root.path(),
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            false,
            message.to_string(),
            true,
            &schemas(),
            &no_origins(),
            &HomeClaims::default(),
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
            root.path(),
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            true,
            message.to_string(),
            true,
            &schemas(),
            &no_origins(),
            &HomeClaims::default(),
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
            root.path(),
            &base(),
            &base().sha,
            false,
            message.to_string(),
            true,
            &schemas(),
            &no_origins(),
            &HomeClaims::default(),
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
            root.path(),
            &base(),
            &base().sha,
            false,
            message.to_string(),
            false,
            &schemas(),
            &no_origins(),
            &HomeClaims::default(),
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
            root.path(),
            &base(),
            &base().sha,
            false,
            message.to_string(),
            true,
            &schemas(),
            &no_origins(),
            &HomeClaims::default(),
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

    /// No join origins — the milestone planner's promote set is unrecorded, which is the
    /// guard's *not a body the join wrote* arm. The preflight/empty-commit/promote tests
    /// above and below pass it: none of them stages a doc over an occupied home.
    fn no_origins() -> BTreeMap<String, crate::milestone::MergedOrigin> {
        BTreeMap::new()
    }

    /// One join origin: `final_address` came from `group[0]`, staged there as `minted`.
    fn origin_of(
        final_address: &str,
        provenance: state::Provenance,
        minted: &str,
        group: &[&str],
    ) -> (String, crate::milestone::MergedOrigin) {
        (
            final_address.to_string(),
            crate::milestone::MergedOrigin {
                provenance,
                minted: minted.to_string(),
                group_from_here: group.iter().map(|s| (*s).to_string()).collect(),
            },
        )
    }

    /// Run the milestone planner over `staging` with `origins`, every other input clean.
    fn plan_milestone_with(
        root: &Path,
        staging: &Path,
        origins: &BTreeMap<String, crate::milestone::MergedOrigin>,
    ) -> Result<FinalizePlan, Vec<Finding>> {
        plan_milestone_finalize(
            "cache-rework",
            staging,
            root,
            &base(),
            &base().sha,
            false,
            "Finalize milestone cache-rework (2 sub-tasks)\n".to_string(),
            true,
            &schemas(),
            origins,
            &HomeClaims::default(),
        )
    }

    /// **The milestone planner runs the clobber guard** (the rc.24 fix pass, `(R6, D-1)`;
    /// `design/finalize.md` → 4. Promote): a sub-task's `created` doc whose destination
    /// already holds a file blocks with `finalize.promote-clobber`, keyed at the destination,
    /// exactly as [`plan_finalize`] does. Until then the milestone planner ran the promote
    /// sweep and returned — the guard had only ever been wired to the task planner.
    ///
    /// The axis is the class, not the reported instance: the doc reaches the occupied home
    /// **suffixed by the join** (the record's repro) or **under its own id** (the
    /// verification's V2 — no collision at all), and the occupant is a managed-looking doc or
    /// a foreign file. Every cell refuses, names the sub-task and the address the doc is
    /// staged under in its sub-area, and routes at the in-task rename and this milestone's
    /// own boundary — never at a per-task finalize (a sub-task has none) and never at
    /// adopting the occupant first (that commit moves `HEAD` off the milestone's base).
    #[test]
    fn milestone_plan_blocks_a_created_doc_over_an_occupied_home() {
        // (final slug, the slug it is staged under in the sub-area)
        let landings = [
            ("cache-strategy-2", "cache-strategy"),
            ("cache-strategy", "cache-strategy"),
        ];
        let occupants: [&[u8]; 2] = [
            b"---\nstatus: accepted\n---\n\n# An earlier decision\n\n## Decision\n\nKept.\n",
            b"a hand-written note\n",
        ];
        for (final_slug, minted_slug) in landings {
            for occupant in occupants {
                let root = TempRoot::new("milestone-clobber");
                let staging = root
                    .path()
                    .join("milestones")
                    .join("cache-rework")
                    .join("merged");
                stage_filled_adr(&staging, final_slug);
                let destination = format!("decisions/{final_slug}.md");
                state::persist(&root.path().join(&destination), occupant)
                    .expect("an occupant at the destination");
                let final_address = format!("adr:{final_slug}");
                let minted = format!("adr:{minted_slug}");
                let origins = BTreeMap::from([origin_of(
                    &final_address,
                    state::Provenance::Created,
                    &minted,
                    &["area-zed"],
                )]);

                let findings = plan_milestone_with(root.path(), &staging, &origins)
                    .expect_err("a created doc over an occupied home must block");
                let cell = format!("{final_address} staged as {minted}");
                assert_eq!(findings.len(), 1, "{cell}: exactly one clobber finding");
                let finding = &findings[0];
                assert_eq!(finding.code, "finalize.promote-clobber", "{cell}");
                assert_eq!(finding.severity, Severity::Blocking, "{cell}");
                assert_eq!(
                    target(finding),
                    Some(destination.as_str()),
                    "{cell}: keyed at the destination FILE, the task arm's key exactly",
                );
                assert!(
                    finding.message.contains("`area-zed`") && finding.message.contains(&minted),
                    "{cell}: the message names the sub-task and the address its doc is staged \
                     under; got: {}",
                    finding.message,
                );
                assert_eq!(
                    finding.message.contains("suffixed"),
                    final_slug != minted_slug,
                    "{cell}: the suffix is named exactly when the join applied one; got: {}",
                    finding.message,
                );
                let route = finding.route.as_deref().expect("a blocking finding routes");
                assert!(
                    route.contains(&format!(
                        "`jigc doc rename {minted} --to \"<title>\" --task area-zed`"
                    )),
                    "{cell}: the route hands back the in-task rename, addressed at the \
                     sub-area's own address; got: {route}",
                );
                assert!(
                    route.contains("`jigc milestone finalize cache-rework`"),
                    "{cell}: the route ends at this milestone's boundary; got: {route}",
                );
                assert!(
                    !route.contains("jigc task finalize") && !route.contains("jigc migrate"),
                    "{cell}: no per-task boundary, and no adoption before the boundary (it \
                     moves HEAD off the milestone's base); got: {route}",
                );
                assert!(
                    route.contains("finalize.base-mismatch"),
                    "{cell}: the route says why the occupant waits; got: {route}",
                );
                assert_eq!(
                    std::fs::read(root.path().join(&destination)).expect("the occupant"),
                    occupant,
                    "{cell}: the planner writes nothing",
                );
            }
        }
    }

    /// **A blocked suffix routes every later member of its group** — the join numbers a
    /// collision group by position in task-id order, so renaming only the blocked doc hands
    /// its occupied suffix to the next sub-task. The finding for `-2` of a group of three
    /// names the renames of the second **and** the third sub-task, and not the first, which
    /// keeps the bare id either way; two occupied suffixes are two findings, each keyed at
    /// its own destination.
    #[test]
    fn milestone_clobber_route_names_every_sub_task_from_the_blocked_suffix_on() {
        let root = TempRoot::new("milestone-clobber-group");
        let staging = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged");
        for slug in ["cache-strategy", "cache-strategy-2", "cache-strategy-3"] {
            stage_filled_adr(&staging, slug);
        }
        let created = state::Provenance::Created;
        let minted = "adr:cache-strategy";
        let origins = BTreeMap::from([
            origin_of(
                "adr:cache-strategy",
                created,
                minted,
                &["low", "mid", "zed"],
            ),
            origin_of("adr:cache-strategy-2", created, minted, &["mid", "zed"]),
            origin_of("adr:cache-strategy-3", created, minted, &["zed"]),
        ]);
        let rename =
            |sub: &str| format!("`jigc doc rename {minted} --to \"<title>\" --task {sub}`");

        // Only `-2` is occupied.
        state::persist(
            &root.path().join("decisions").join("cache-strategy-2.md"),
            b"# an earlier decision\n",
        )
        .expect("occupy -2");
        let findings = plan_milestone_with(root.path(), &staging, &origins)
            .expect_err("the occupied suffix blocks");
        assert_eq!(findings.len(), 1, "`-3` is free: {findings:#?}");
        let route = findings[0].route.as_deref().expect("a route");
        assert!(
            route.contains(&rename("mid")) && route.contains(&rename("zed")),
            "both sub-tasks from the blocked suffix on are named; got: {route}",
        );
        assert!(
            !route.contains(&rename("low")),
            "the first sub-task keeps the bare id and is not asked to move; got: {route}",
        );

        // `-3` occupied as well: one finding per occupied home, distinct keys.
        state::persist(
            &root.path().join("decisions").join("cache-strategy-3.md"),
            b"a hand-written note\n",
        )
        .expect("occupy -3");
        let findings = plan_milestone_with(root.path(), &staging, &origins)
            .expect_err("both occupied suffixes block");
        let targets: Vec<Option<&str>> = findings.iter().map(target).collect();
        assert_eq!(
            targets,
            vec![
                Some("decisions/cache-strategy-2.md"),
                Some("decisions/cache-strategy-3.md"),
            ],
            "one finding per occupied destination, in destination order",
        );
        let last = findings[1].route.as_deref().expect("a route");
        assert!(
            last.contains(&rename("zed")) && !last.contains(&rename("mid")),
            "the last member's finding names its own rename only; got: {last}",
        );
    }

    /// **The regression hinge, at the milestone boundary too.** A sub-task that copied a
    /// committed doc in holds it `edited-from-base`; re-promoting it over its own home is the
    /// update path (`design/storage.md` → copy-on-first-touch), never a clobber. And a body
    /// with no origin at all is not one the join wrote — the task arm's *unrecorded → never a
    /// clobber*.
    #[test]
    fn milestone_plan_lets_an_edited_from_base_doc_re_promote() {
        for origins in [
            BTreeMap::from([origin_of(
                "adr:cache-strategy",
                state::Provenance::EditedFromBase,
                "adr:cache-strategy",
                &["area-low"],
            )]),
            no_origins(),
        ] {
            let root = TempRoot::new("milestone-update");
            let staging = root
                .path()
                .join("milestones")
                .join("cache-rework")
                .join("merged");
            stage_filled_adr(&staging, "cache-strategy");
            state::persist(
                &root.path().join("decisions").join("cache-strategy.md"),
                b"# the committed version this sub-task edited\n",
            )
            .expect("the committed doc at its home");
            let plan = plan_milestone_with(root.path(), &staging, &origins)
                .expect("an update of a committed doc re-promotes over its own home");
            assert_eq!(plan.promotions.len(), 1);
        }
    }

    /// **Control — a free home passes.** A `created` doc, suffixed or not, whose destination
    /// holds nothing is the join working; an occupant at a *different* home is not its
    /// concern.
    #[test]
    fn milestone_plan_passes_a_created_doc_at_a_free_home() {
        let root = TempRoot::new("milestone-free");
        let staging = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged");
        stage_filled_adr(&staging, "cache-strategy");
        stage_filled_adr(&staging, "cache-strategy-2");
        state::persist(
            &root.path().join("decisions").join("cache-strategy-3.md"),
            b"# a neighbour, at a home nobody promotes to\n",
        )
        .expect("an unrelated doc");
        let created = state::Provenance::Created;
        let minted = "adr:cache-strategy";
        let origins = BTreeMap::from([
            origin_of("adr:cache-strategy", created, minted, &["low", "zed"]),
            origin_of("adr:cache-strategy-2", created, minted, &["zed"]),
        ]);
        let plan =
            plan_milestone_with(root.path(), &staging, &origins).expect("free homes yield a plan");
        assert_eq!(plan.promotions.len(), 2, "both created docs are promoted");
    }

    /// The claims of a repository that holds `path` — in `HEAD`, or in the index alone.
    fn held(path: &str, in_head: bool) -> HomeClaims {
        let mut claims = HomeClaims::default();
        if in_head {
            claims.head.insert(path.to_string());
        } else {
            claims.index.insert(path.to_string());
        }
        claims
    }

    /// Run the task planner over a staged `adr:single-node-cache` recorded as `provenance`,
    /// with `claims` — every other input clean.
    fn plan_task_holding(
        root: &Path,
        provenance: Option<state::Provenance>,
        claims: &HomeClaims,
    ) -> Result<FinalizePlan, Vec<Finding>> {
        let task_dir = root.join("tasks").join("record-decision");
        let schema = stage_filled_commit(&task_dir, "record-decision");
        stage_filled_adr(&task_dir, "single-node-cache");
        if let Some(provenance) = provenance {
            state::record_doc_provenance(&task_dir, "adr:single-node-cache", provenance)
                .expect("record provenance");
        }
        plan_finalize(
            &task_dir,
            root,
            &base(),
            &base().sha,
            &ValidationReport::new(Vec::new(), &no_delta_resolved()),
            true,
            &schema,
            "record-decision",
            &schemas(),
            claims,
        )
    }

    /// **A home with nothing on disk that git still holds is occupied** (the rc.24 fix pass,
    /// the completion audit's CPL-5; `design/finalize.md` → 4. Promote). A `created` doc is
    /// refused under `finalize.promote-clobber`, keyed at the destination, whichever of
    /// `HEAD` and the index holds the path; the route gives the doc another id, ends at the
    /// task's own boundary, and names the aimed restore of what git holds — never a removal.
    ///
    /// The cells that must **not** refuse sit beside it: the same claims over a doc the task
    /// copied in (`edited-from-base` — the committed doc's own update), over a doc with no
    /// recorded provenance, over an in-place migration of that very path, and a claim on a
    /// path the plan does not promote to.
    #[test]
    fn a_created_doc_is_refused_over_a_home_git_holds_and_the_disk_does_not() {
        let destination = "decisions/single-node-cache.md";
        for in_head in [true, false] {
            let root = TempRoot::new("held-home");
            let findings = plan_task_holding(
                root.path(),
                Some(state::Provenance::Created),
                &held(destination, in_head),
            )
            .expect_err("a created doc over a home git holds must block");
            assert_eq!(findings.len(), 1, "one finding; {findings:#?}");
            let finding = &findings[0];
            assert_eq!(finding.code, "finalize.promote-clobber");
            assert_eq!(finding.severity, Severity::Blocking);
            assert_eq!(target(finding), Some(destination), "keyed at the home");
            assert!(
                finding.message.contains("missing from the worktree")
                    && finding.message.contains("`adr:single-node-cache`"),
                "the message says the home is not free and whose doc it is: {}",
                finding.message,
            );
            let route = finding.route.as_deref().expect("blocking ⇒ routed");
            assert!(
                route.contains(
                    "`jigc doc rename adr:single-node-cache --to \"<title>\" --task \
                     record-decision`"
                ) && route.contains("`jigc task finalize record-decision`"),
                "the route renames in the task and ends at its boundary: {route}",
            );
            let restore = if in_head {
                "checkout HEAD -- decisions/single-node-cache.md`"
            } else {
                "checkout -- decisions/single-node-cache.md`"
            };
            assert!(
                route.contains(&format!("`git -C {}", root.path().display()))
                    && route.contains(restore),
                "the restore is aimed at the checkout and names what holds the file: {route}",
            );
            for surface in [finding.message.as_str(), route] {
                let lower = surface.to_lowercase();
                assert!(
                    !lower.contains("remove")
                        && !lower.contains("delete")
                        && !lower.contains("git rm"),
                    "the refusal never teaches a removal: {surface}",
                );
            }
            assert!(
                !root.path().join(destination).exists(),
                "the planner writes nothing",
            );
        }

        // ── MUST NOT REFUSE ──
        let claims = held(destination, true);
        for (cell, provenance) in [
            ("edited-from-base", Some(state::Provenance::EditedFromBase)),
            ("no recorded provenance", None),
        ] {
            let root = TempRoot::new("held-home-control");
            plan_task_holding(root.path(), provenance, &claims)
                .unwrap_or_else(|f| panic!("{cell}: not a mint, so not a clobber; {f:#?}"));
        }
        let root = TempRoot::new("held-home-elsewhere");
        plan_task_holding(
            root.path(),
            Some(state::Provenance::Created),
            &held("decisions/another-doc.md", true),
        )
        .expect("a claim on a path the plan does not promote to refuses nothing");
        let root = TempRoot::new("held-home-in-place");
        state::persist(
            &root
                .path()
                .join("tasks")
                .join("record-decision")
                .join("source-path"),
            destination.as_bytes(),
        )
        .expect("record the migration source");
        plan_task_holding(root.path(), Some(state::Provenance::Created), &claims)
            .expect("the in-place migration of that very path is its own carve-out");
    }

    /// **A fixed identity has no other id** — the held arm's route for a placement singleton
    /// prints no rename (it would refuse) and hands back the drop of the mint, at both doors.
    #[test]
    fn the_held_arm_routes_a_fixed_identity_at_dropping_the_mint() {
        let schema = placement_schema("foo", "FOO.md");
        let mut with_placement = schemas();
        with_placement.insert(schema.ty.clone(), schema.clone());
        let claims = held("FOO.md", true);

        // The task door.
        let root = TempRoot::new("held-fixed-task");
        let task_dir = root.path().join("tasks").join("form-foo");
        let commit = stage_filled_commit(&task_dir, "form-foo");
        stage_filled_placement(&task_dir, &schema, &schema.ty);
        state::record_doc_provenance(&task_dir, "foo:foo", state::Provenance::Created)
            .expect("record created provenance");
        let findings = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            &base().sha,
            &ValidationReport::new(Vec::new(), &no_delta_resolved()),
            true,
            &commit,
            "form-foo",
            &with_placement,
            &claims,
        )
        .expect_err("a minted singleton over a home git holds must block");
        let route = findings[0].route.as_deref().expect("a route");
        assert!(
            !route.contains("jigc doc rename")
                && route.contains("`jigc task discard form-foo --force`")
                && route.contains("`jigc doc show foo:foo --task form-foo`"),
            "the task door drops the mint, and says how to read it first: {route}",
        );

        // The milestone door.
        let root = TempRoot::new("held-fixed-milestone");
        let staging = root
            .path()
            .join("milestones")
            .join("page-rework")
            .join("merged");
        stage_filled_placement(&staging, &schema, &schema.ty);
        let origins = BTreeMap::from([origin_of(
            "foo:foo",
            state::Provenance::Created,
            "foo:foo",
            &["area-zed"],
        )]);
        let plan = |origins: &BTreeMap<String, crate::milestone::MergedOrigin>| {
            plan_milestone_finalize(
                "page-rework",
                &staging,
                root.path(),
                &base(),
                &base().sha,
                false,
                "Finalize milestone page-rework (1 sub-task)\n".to_string(),
                true,
                &with_placement,
                origins,
                &claims,
            )
        };
        let findings = plan(&origins).expect_err("the boundary refuses it too");
        assert_eq!(findings.len(), 1);
        assert_eq!(target(&findings[0]), Some("FOO.md"));
        let route = findings[0].route.as_deref().expect("a route");
        assert!(
            !route.contains("jigc doc rename")
                && route.contains("`jigc task discard area-zed --force`")
                && route.contains("`jigc milestone finalize page-rework`"),
            "the boundary drops the sub-task that minted it and lands the rest: {route}",
        );
        // MUST NOT REFUSE: a body the join did not write is not a mint.
        plan(&no_origins()).expect("no origin, no clobber — the file arm's rule");
    }

    /// **The milestone boundary refuses the same entries** (the rc.24 fix pass, `(R6, D-7)`)
    /// — the promote is shared, so the contract is. Over every shape × every way the join
    /// can hand a body on — a sub-task's fresh doc (kept its id · suffixed), a sub-task's
    /// edit of a committed doc, and a body with no origin at all — the planner blocks with
    /// `finalize.promote-clobber` keyed at the destination. The file arm lets the last two
    /// through by design (*edited-from-base re-promotes*, *unrecorded is not a clobber*);
    /// the shape arm asks neither, because it is about the entry.
    ///
    /// Every route ends at this milestone's own boundary and states its exit as **no
    /// commit** — a commit made first moves `HEAD` off the milestone's base.
    #[cfg(unix)]
    #[test]
    fn milestone_plan_refuses_every_promotion_over_an_entry_that_is_not_a_regular_file() {
        let created = state::Provenance::Created;
        let edited = state::Provenance::EditedFromBase;
        // (final slug, origin: provenance + the slug it is staged under)
        let landings: [(&str, Option<(state::Provenance, &str)>); 4] = [
            ("cache-strategy", Some((created, "cache-strategy"))),
            ("cache-strategy-2", Some((created, "cache-strategy"))),
            ("cache-strategy", Some((edited, "cache-strategy"))),
            ("cache-strategy", None),
        ];
        for (shape, noun) in FOREIGN_SHAPES {
            for (final_slug, origin) in landings {
                let cell = format!("{shape} · adr:{final_slug} · {origin:?}");
                let root = TempRoot::new("shape-milestone");
                let staging = root
                    .path()
                    .join("milestones")
                    .join("cache-rework")
                    .join("merged");
                stage_filled_adr(&staging, final_slug);
                let destination = format!("decisions/{final_slug}.md");
                let through = plant_foreign(&root.path().join(&destination), shape);
                let through_before = through.as_ref().and_then(|p| std::fs::read(p).ok());
                let final_address = format!("adr:{final_slug}");
                let origins = match origin {
                    Some((provenance, minted_slug)) => BTreeMap::from([origin_of(
                        &final_address,
                        provenance,
                        &format!("adr:{minted_slug}"),
                        &["area-zed"],
                    )]),
                    None => no_origins(),
                };

                let findings = plan_milestone_with(root.path(), &staging, &origins)
                    .expect_err("an entry that is not a regular file refuses the promotion");
                assert_eq!(findings.len(), 1, "{cell}: exactly one finding");
                let finding = &findings[0];
                assert_shape_surfaces(finding, &destination, noun, &cell);
                let route = finding.route.as_deref().expect("a route");
                assert!(
                    route.contains("`jigc milestone finalize cache-rework`")
                        && !route.contains("jigc task finalize"),
                    "{cell}: the route ends at this milestone's boundary and names no \
                     per-task one; got: {route}",
                );
                assert!(
                    route.contains("no commit"),
                    "{cell}: every exit is stated as no commit; got: {route}",
                );
                let rename =
                    "`jigc doc rename adr:cache-strategy --to \"<title>\" --task area-zed`";
                assert_eq!(
                    route.contains(rename),
                    matches!(origin, Some((state::Provenance::Created, _))),
                    "{cell}: the in-task rename — at the id the doc is staged under — exactly \
                     for a doc a sub-task minted; got: {route}",
                );
                assert_eq!(
                    route.contains("regular file itself"),
                    matches!(origin, Some((state::Provenance::EditedFromBase, _))),
                    "{cell}: a copied-in doc lands over the regular file itself; got: {route}",
                );
                assert_eq!(
                    finding.message.contains("`area-zed`"),
                    origin.is_some(),
                    "{cell}: the sub-task is named exactly when the join recorded one; got: {}",
                    finding.message,
                );
                assert_eq!(
                    finding.message.contains("suffixed"),
                    final_slug == "cache-strategy-2",
                    "{cell}: the suffix is named exactly when the join applied one; got: {}",
                    finding.message,
                );
                if let Some(through) = &through {
                    assert_eq!(
                        std::fs::read(through).ok(),
                        through_before,
                        "{cell}: the planner writes nothing through the entry",
                    );
                }
            }
        }
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
    /// - The **task** arm keeps its shape (its switch-back-or-discard route is correct for a
    ///   task, whose `discard` is a genuine, cheap exit); since M50 its discard span is
    ///   substituted with the real id and carries `--force`.
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
            &HomeClaims::default(),
        )
        .expect_err("a base mismatch aborts the task preflight");
        assert_eq!(err[0].code, "finalize.base-mismatch");
        assert_eq!(
            err[0].message,
            format!(
                "the task was started at base `{}` but HEAD is now `{head}`",
                base().sha
            ),
            "the task arm's message names the base and HEAD (in the one sha form N22 settled)",
        );
        assert_eq!(
            err[0].route.as_deref(),
            Some(
                format!(
                    "switch back to `{}` or discard the task with \
                     `jigc task discard add-rate-limiter --force`",
                    base().sha
                )
                .as_str()
            ),
            "the task arm's route still offers switch-back-or-discard",
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
            root.path(),
            &base(),
            head,
            false,
            "Finalize milestone cache-rework (1 sub-task)\n".to_string(),
            true,
            &schemas(),
            &no_origins(),
            &HomeClaims::default(),
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
            message.contains("HEAD is now") && message.contains(&base().sha),
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

    /// The backticked sha-shaped tokens of a finding's text, in order — the probe the
    /// one-sha-form assertion iterates over (M47 inc-10 T4 / N22). A route or message
    /// renders every sha inside backticks, so splitting on the delimiter and keeping the
    /// odd (quoted) runs that are all-hex and ≥7 long enumerates exactly the sha forms a
    /// reader sees.
    fn sha_tokens(text: &str) -> Vec<&str> {
        text.split('`')
            .skip(1)
            .step_by(2)
            .filter(|t| t.len() >= 7 && t.chars().all(|c| c.is_ascii_hexdigit()))
            .collect()
    }

    /// (M47 inc-10 T4 · N22) `finalize.base-mismatch` renders **one** sha form. It used
    /// to name the pinned base by its git-abbreviated `short` and HEAD by its full
    /// 40-char sha in the same sentence, so the two identities being *compared* did not
    /// look comparable. The sweep runs the code's whole producer axis — the pin form ×
    /// {task, milestone} and the overlap form — and asserts, per finding, that every
    /// sha-shaped token in message **and** route is the same width.
    #[test]
    fn base_mismatch_renders_one_sha_form_across_both_constructors() {
        let root = TempRoot::new("one-sha-form");
        let head = "ffffffffffffffffffffffffffffffffffffffff";
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        // The pin form, task arm.
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        let schema = stage_filled_commit(&task_dir, "add-rate-limiter");
        let pin_task = plan_finalize(
            &task_dir,
            root.path(),
            &base(),
            head,
            &clean,
            true,
            &schema,
            "add-rate-limiter",
            &schemas(),
            &HomeClaims::default(),
        )
        .expect_err("a base mismatch aborts the task preflight");

        // The pin form, milestone arm.
        let staging = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged");
        stage_filled_adr(&staging, "cache-strategy");
        let pin_milestone = plan_milestone_finalize(
            "cache-rework",
            &staging,
            root.path(),
            &base(),
            head,
            false,
            "Finalize milestone cache-rework (1 sub-task)\n".to_string(),
            true,
            &schemas(),
            &no_origins(),
            &HomeClaims::default(),
        )
        .expect_err("a base mismatch aborts the milestone preflight");

        // The overlap form: moved history that touches the task's own dirty footprint.
        let overlap_dir = root.path().join("tasks").join("overlapping");
        stage_filled_adr(&overlap_dir, "keep-sessions-in-memory");
        let overlap = decide_base_repin(
            "overlapping",
            &overlap_dir,
            &base(),
            head,
            &["src/limiter.rs".to_string()],
            &["src/limiter.rs".to_string()],
            &schemas(),
        )
        .expect_err("overlapping moved history blocks");

        for (arm, findings) in [
            ("pin/task", pin_task),
            ("pin/milestone", pin_milestone),
            ("overlap", overlap),
        ] {
            let finding = &findings[0];
            assert_eq!(finding.code, "finalize.base-mismatch", "{arm}");
            let route = finding.route.as_deref().expect("blocking ⇒ a route");
            let mut widths: Vec<usize> = sha_tokens(&finding.message)
                .into_iter()
                .chain(sha_tokens(route))
                .map(str::len)
                .collect();
            assert!(
                widths.len() >= 2,
                "{arm}: the finding names at least the base and HEAD",
            );
            widths.dedup();
            assert_eq!(
                widths,
                vec![base().sha.len()],
                "{arm}: every sha the reader sees is rendered in one form — \
                 message {:?}, route {route:?}",
                finding.message,
            );
        }
    }

    /// (M47 inc-10 T4 · N17) `finalize.empty-commit` routes **both** exits, and names
    /// the verb family of the unit that produced nothing. The shipped route was
    /// *"make a change, then re-run `jigc task finalize`"* — one exit, no abandon path,
    /// and the **task** verb served to a blocked milestone too (the same unit-blindness
    /// M42 fixed on this file's sibling `base-mismatch` route). `jigc task discard` was
    /// already the recorded abandon verb one file over (`write.rs` → the discard route);
    /// the block that most needs it never named it.
    #[test]
    fn empty_commit_route_names_the_abandon_exit_for_both_units() {
        let root = TempRoot::new("empty-commit-abandon");
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        let schema = stage_filled_commit(&task_dir, "add-rate-limiter");
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
            &HomeClaims::default(),
        )
        .expect_err("an empty diff aborts");
        assert_eq!(err[0].code, "finalize.empty-commit");
        let route = err[0].route.as_deref().expect("blocking ⇒ a route");
        assert!(
            route.contains("jigc task finalize add-rate-limiter"),
            "the task arm re-runs the task door, with the id filled in: {route:?}",
        );
        assert!(
            route.contains("jigc task discard add-rate-limiter"),
            "the task arm names the abandon exit: {route:?}",
        );

        let staging = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged");
        stage_filled_adr(&staging, "cache-strategy");
        let err = plan_milestone_finalize(
            "cache-rework",
            &staging,
            root.path(),
            &base(),
            &base().sha,
            false,
            "Finalize milestone cache-rework (1 sub-task)\n".to_string(),
            false,
            &schemas(),
            &no_origins(),
            &HomeClaims::default(),
        )
        .expect_err("an empty diff aborts");
        assert_eq!(err[0].code, "finalize.empty-commit");
        assert!(
            err[0].message.starts_with("the milestone validated"),
            "the milestone arm names the unit that produced nothing: {:?}",
            err[0].message,
        );
        let route = err[0].route.as_deref().expect("blocking ⇒ a route");
        assert!(
            route.contains("jigc milestone finalize cache-rework"),
            "the milestone arm names the MILESTONE door, never `jigc task finalize`: {route:?}",
        );
        assert!(
            route.contains("jigc milestone discard cache-rework"),
            "the milestone arm names the milestone abandon exit: {route:?}",
        );
        assert!(
            !route.contains("jigc task "),
            "no task verb is offered for a milestone (the M42 unit-awareness, swept onto \
             this code): {route:?}",
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
        let findings = decide_carryover(
            Some(&snapshot),
            &current,
            None,
            &[],
            CarryoverBoundary::Task,
            std::path::Path::new("/repo"),
        );
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
            route.contains("git -C /repo restore --staged"),
            "the route names the unstage exit, aimed at the index it is about (M53 — the \
             cwd census, C1-01): {route:?}"
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
            decide_carryover(
                Some(&snapshot),
                &current,
                None,
                &[],
                CarryoverBoundary::Task,
                std::path::Path::new("/repo"),
            ),
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
        let findings = decide_carryover(
            Some(&snapshot),
            &current,
            None,
            &[],
            CarryoverBoundary::Task,
            std::path::Path::new("/repo"),
        );
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
            &[],
            CarryoverBoundary::Task,
            std::path::Path::new("/repo"),
        );
        assert_eq!(
            findings.len(),
            1,
            "the retire path is exempt; the entry is not"
        );
        assert_eq!(findings[0].key().target.as_deref(), Some("notes.md"));
    }

    /// The task's **recorded owner-artifact paths are exempt at the [`CarryoverBoundary::Task`]
    /// boundary** (M45 Inc 8 T1, Decision 5): an agent stages the audit artifact before
    /// minting the recording task (the natural authoring order), so the pre-task staged entry
    /// is the task's own subject, never a foreign carry-over — while every *other* carried
    /// path still blocks. The exempt path is compared lexically normalized (the recorded
    /// `owned-location` field value may carry `./`). **Context-scoped guard:** the same set
    /// at the **milestone** boundary is **not** exempt (the milestone-boundary owner-artifact
    /// exemption is a separate, deferred concern) — a green over the composing (Task) context
    /// must not hide the omitting (Milestone) context.
    ///
    /// **The exemption keys on the boundary, so it iterates the boundary's whole door axis**
    /// (M47 Inc 4): the committing door and its [`CarryoverBoundary::TaskPreview`] read-door
    /// preview must exempt the identical set, else the preview would report a block the
    /// finalize does not have. A door added to the task boundary later joins this loop.
    #[test]
    fn carryover_owner_artifact_paths_exempt_at_task_boundary_only() {
        let snapshot = staged(
            &[
                ("completions/artifacts/M45/audit.md", "aaaa1111"),
                ("src/mine.rs", "bbbb2222"),
            ],
            &[],
        );
        let current = snapshot.clone();
        let owner_exempt = vec!["./completions/artifacts/M45/audit.md".to_string()];

        // Every task-boundary door: the owner-artifact is exempt; the foreign entry blocks.
        for boundary in [CarryoverBoundary::Task, CarryoverBoundary::TaskPreview] {
            let task = decide_carryover(
                Some(&snapshot),
                &current,
                None,
                &owner_exempt,
                boundary,
                std::path::Path::new("/repo"),
            );
            assert_eq!(
                task.len(),
                1,
                "the owner-artifact is exempt at {boundary:?}; the foreign entry is not: {task:?}"
            );
            assert_eq!(task[0].key().target.as_deref(), Some("src/mine.rs"));
        }

        // Milestone boundary: the SAME set is not exempt — both entries carry (the
        // omitting-context guard).
        let milestone = decide_carryover(
            Some(&snapshot),
            &current,
            None,
            &owner_exempt,
            CarryoverBoundary::Milestone,
            std::path::Path::new("/repo"),
        );
        assert_eq!(
            milestone.len(),
            2,
            "no owner-artifact exemption at the milestone boundary — both carry: {milestone:?}"
        );
    }

    /// **Missing snapshot ⇒ fail-open** (the declared bound: a task minted pre-M43
    /// finalizes as today) — and, distinctly, an **empty** snapshot carries nothing
    /// (everything staged since mint is the task's own).
    #[test]
    fn carryover_missing_snapshot_fails_open_empty_snapshot_carries_nothing() {
        let current = staged(&[("src/foreign.rs", "aaaa1111")], &["legacy/OLD.md"]);
        assert_eq!(
            decide_carryover(
                None,
                &current,
                None,
                &[],
                CarryoverBoundary::Task,
                std::path::Path::new("/repo"),
            ),
            Vec::new(),
            "no snapshot (pre-M43 mint) ⇒ fail-open, no findings",
        );
        assert_eq!(
            decide_carryover(
                Some(&StagedSnapshot::default()),
                &current,
                None,
                &[],
                CarryoverBoundary::Task,
                std::path::Path::new("/repo"),
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
        let findings = decide_carryover(
            Some(&snapshot),
            &current,
            None,
            &[],
            CarryoverBoundary::Task,
            std::path::Path::new("/repo"),
        );
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
                &[],
                CarryoverBoundary::Task,
                std::path::Path::new("/repo"),
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
            &[],
            CarryoverBoundary::Milestone,
            std::path::Path::new("/repo"),
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
                route.contains("--carry-staged") && route.contains("git -C /repo restore --staged"),
                "the route names both exits, the unstage aimed at the index it is about: \
                 {route:?}"
            );
        }
    }

    /// The **preview door's wording is law-1 honest** (M47 Inc 4): read from
    /// `jigc task validate`, nothing has been refused — so the message may not say
    /// *"refusing"* and the route may not tell the reader to *re-run the finalize* they
    /// never ran. It names the door that **will** refuse and a flag accepted at both.
    /// The [`CarryoverBoundary::Task`] door's own bytes are asserted unchanged in the same
    /// breath: the preview must not have been bought by re-wording the committing door.
    #[test]
    fn carryover_preview_door_names_the_finalize_that_will_refuse() {
        let snapshot = staged(&[("foreign.txt", "aaaa1111")], &["gone.md"]);
        let current = snapshot.clone();

        let preview = decide_carryover(
            Some(&snapshot),
            &current,
            None,
            &[],
            CarryoverBoundary::TaskPreview,
            std::path::Path::new("/repo"),
        );
        let committing = decide_carryover(
            Some(&snapshot),
            &current,
            None,
            &[],
            CarryoverBoundary::Task,
            std::path::Path::new("/repo"),
        );
        assert_eq!(preview.len(), 2, "the same decision, both halves");
        assert_eq!(
            preview
                .iter()
                .map(|f| f.key().target.clone())
                .collect::<Vec<_>>(),
            committing
                .iter()
                .map(|f| f.key().target.clone())
                .collect::<Vec<_>>(),
            "same decision, same keys — only the wording differs"
        );
        for finding in &preview {
            assert_eq!(finding.code, "finalize.carried-staged");
            assert_eq!(finding.severity, Severity::Blocking);
            let route = finding.route.as_deref().expect("blocking ⇒ routed");
            assert!(
                finding.message.contains("jigc task finalize")
                    && finding.message.contains("will refuse"),
                "the preview names the door that will refuse: {:?}",
                finding.message
            );
            assert!(
                !finding.message.contains("refusing") && !route.contains("re-run the finalize"),
                "the preview never claims a finalize refused or was run; message: {:?}\nroute: {route:?}",
                finding.message
            );
            assert!(
                route.contains("--carry-staged") && route.contains("git -C /repo restore --staged"),
                "the route names both exits, the unstage aimed at the index it is about: \
                 {route:?}"
            );
        }
        // The committing door is untouched by the preview existing.
        for finding in &committing {
            assert!(
                finding.message.contains("refusing to let"),
                "the committing door keeps its own truth: {:?}",
                finding.message
            );
            assert!(
                finding
                    .route
                    .as_deref()
                    .is_some_and(|r| r.contains("re-run the finalize with `--carry-staged`")),
                "the committing door keeps its own route: {:?}",
                finding.route
            );
        }
    }

    /// The **`Setup` boundary's own refusal identity** (M51 Increment 3 / T1;
    /// `settle-record.md` → D3, amended by §1 and §10): `jigc setup` refuses its install
    /// commit when a path in its own pathspec carries bytes it did not write. The door is
    /// a [`CarryoverBoundary`] member — same family, *"a door committing paths it does not
    /// own"* — but neither the task doors' code nor their shape fits it:
    ///
    /// - **Its own code**, `setup.dirty-install-path`: `finalize.carried-staged`'s
    ///   task-shaped vocabulary (*"before this task existed"*, `--carry-staged`) would lie
    ///   at a door with no task (law 1).
    /// - **One finding over the whole dirty set** — the `uninstall.dirty-worktree` mold at
    ///   this same door, not `finalize.carried-staged`'s one-per-path: `setup::run` returns
    ///   a single [`Finding`], so the set is the subject and no one path is the key.
    /// - **The route names the resolving act first** and `--force` second: `--force` at the
    ///   first command an adopter runs is the reflex-training risk D3 carries as a declared
    ///   bound, so the exit that keeps the work leads.
    ///
    /// **What the message may not claim** (the load-bearing honesty constraint): it states
    /// only what the door actually did, which is why `install_written` picks the clause
    /// rather than a single sentence covering both arms. The M51 completion audit is the
    /// reason there are two: the *"written and staged"* wording was universal while the
    /// door's ordinary refusal moved **before** the first write, where it is false.
    ///
    /// **Order-invariant by construction** (Validation hardening #7): the same dirty set in
    /// two divergent orders words one byte-identical finding, so the door's pathspec order
    /// can never reach the printed surface.
    #[test]
    fn setup_boundary_finding_words_the_doors_own_refusal() {
        let dirty: Vec<String> = ["CLAUDE.md", ".claude/settings.json", ".jigc/AGENT.md"]
            .iter()
            .map(|p| (*p).to_string())
            .collect();
        let finding = setup_dirty_install_finding(&dirty, true, SetupHead::Born, None);

        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(finding.code, "setup.dirty-install-path");
        assert_eq!(
            finding.code,
            CarryoverBoundary::Setup.code(),
            "the code is the boundary's own identity, derived from it and not re-spelled",
        );
        let route = finding.route.as_ref().expect("blocking ⇒ routed");
        assert!(
            matches!(route.kind(), crate::finding::RouteKind::Human),
            "which exit is right is the adopter's judgment: {:?}",
            route.kind()
        );
        assert!(
            finding.location.is_none(),
            "the subject is the dirty set, not one path — no single location can key it",
        );

        // Law 1, the message: every path it was given, and the state it left behind.
        for path in &dirty {
            assert!(
                finding.message.contains(path.as_str()),
                "the message names every dirty path it is given; missing {path}: {:?}",
                finding.message
            );
        }
        assert!(
            finding.message.contains("written and staged")
                && finding.message.contains("no install commit"),
            "the backstop arm states the install files were written and staged with no \
             install commit made: {:?}",
            finding.message
        );
        // …and the pre-write arm — the door's ordinary refusal since the M51 completion
        // audit — says the opposite, because it installed nothing. One predicate, two
        // truthful state clauses; a single universal wording made one of them a law-1 lie.
        let pre_write = setup_dirty_install_finding(&dirty, false, SetupHead::Born, None);
        assert!(
            pre_write.message.contains("nothing was installed")
                && !pre_write.message.contains("written and staged"),
            "the pre-write arm claims no install it did not perform: {:?}",
            pre_write.message
        );
        assert_eq!(
            pre_write.route, finding.route,
            "the route is the same either way — what differs is the state clause",
        );

        // The route's ORDER is the deliverable: the act that keeps the work, then the
        // consent that spends it.
        let repair = route
            .as_str()
            .find("re-run `jigc setup`")
            .expect("the route names the resolving re-run");
        let force = route
            .as_str()
            .find("jigc setup --force")
            .expect("the route names the single consent");
        assert!(
            route.as_str().contains("commit or stash the work"),
            "the route names the resolving act: {route}"
        );
        assert!(
            repair < force,
            "the resolving act is named BEFORE `--force`, which is the single consent and \
             not the first thing offered: {route}"
        );

        // Order-invariance: the same set, reversed, words the same bytes.
        let reversed: Vec<String> = dirty.iter().rev().cloned().collect();
        assert_eq!(
            setup_dirty_install_finding(&reversed, true, SetupHead::Born, None),
            finding,
            "the dirty set is order-invariant — the door's pathspec order never reaches the \
             printed surface",
        );
    }

    /// **The same refusal on an unborn `HEAD` claims nothing a repository with no commit
    /// makes false, and routes at acts that run there** (the rc.24 fix pass, `(R1, F1)`).
    ///
    /// The born wording says *"`HEAD` is untouched, so every path listed above still has its
    /// pre-run bytes there"* and routes at `git stash` first. With no commit there is no
    /// `HEAD` for the bytes to be in, and `git stash` exits 1 (*"You do not have the initial
    /// commit yet"*) — so the refusal the door printed on an unborn `HEAD` named an act that
    /// could not run. This arm names the three that can, in the order that keeps the work:
    /// move the file out, commit it, then the consent.
    ///
    /// **The commit arm names what else that commit must carry.** A first commit ends the
    /// unborn exemption, so an untracked file the install would have merged into becomes a
    /// subject on the re-run; a route that left it out would be followed into a second
    /// refusal over a path the first never named.
    #[test]
    fn setup_boundary_finding_on_an_unborn_head_routes_at_acts_that_run_there() {
        let dirty = vec![".jigc/AGENT.md".to_string()];
        let riding = vec![".gitignore".to_string(), "CLAUDE.md".to_string()];
        let home = Path::new("/somewhere/repo");
        let unborn = |riding| SetupHead::Unborn { home, riding };
        let finding = setup_dirty_install_finding(&dirty, false, unborn(&riding), None);

        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.code,
            CarryoverBoundary::Setup.code(),
            "one refusal, one identity — the `HEAD` it is read at changes its words only",
        );
        assert!(finding.location.is_none(), "the set is still the subject");
        assert!(
            finding.message.contains("1 path(s)") && finding.message.contains("  `.jigc/AGENT.md`"),
            "the count and the listing keep the born arm's shape: {:?}",
            finding.message
        );
        assert!(
            finding.message.contains("nothing was installed")
                && finding.message.contains("no commit yet"),
            "the state clause is the pre-write one, said of a repository with no commit: {:?}",
            finding.message
        );
        assert!(
            !finding.message.contains("`HEAD` is untouched"),
            "there is no `HEAD` to be untouched: {:?}",
            finding.message
        );

        let route = finding.route.as_ref().expect("blocking ⇒ routed");
        assert!(
            matches!(route.kind(), crate::finding::RouteKind::Human),
            "which exit is right is still the adopter's judgment: {:?}",
            route.kind()
        );
        let route = route.as_str();
        assert!(
            !route.contains("stash"),
            "`git stash` has nothing to stand on without a commit, so it is not offered: {route}"
        );
        let at = |needle: &str| {
            route
                .find(needle)
                .unwrap_or_else(|| panic!("the route names `{needle}`: {route}"))
        };
        let (moved, commit, force) = (
            at("move the file(s) out"),
            at("or commit them"),
            at("jigc setup --force"),
        );
        assert!(
            moved < commit && commit < force,
            "the one-step exit leads, the commit follows, the consent is last: {route}"
        );
        assert!(
            route.contains("git -C /somewhere/repo rm --cached -- <path>"),
            "moving a staged file out leaves its index entry behind, so the unstage is named: \\
             {route}"
        );
        for path in &riding {
            assert!(
                route[commit..force].contains(&format!("`{path}`")),
                "the commit arm names `{path}`, which that commit must carry: {route}"
            );
        }

        // With nothing riding, the commit arm names no other path at all.
        let alone = setup_dirty_install_finding(&dirty, false, unborn(&[]), None);
        let alone = alone.route.as_ref().expect("routed").as_str().to_string();
        assert!(
            alone.contains("or commit them and re-run"),
            "the bare commit arm: {alone}"
        );

        // The backstop arm: the install *is* written, and the sentence says only what this
        // run certainly did not do to the paths it names.
        let written = setup_dirty_install_finding(&dirty, true, unborn(&[]), None);
        assert!(
            written.message.contains("written and staged")
                && written.message.contains("staged or committed by this run")
                && !written.message.contains("exist only where you left them"),
            "the backstop arm claims nothing about the worktree: {:?}",
            written.message
        );

        // Order-invariant over both sets, like the born arm.
        let reversed: Vec<String> = riding.iter().rev().cloned().collect();
        assert_eq!(
            setup_dirty_install_finding(&dirty, false, unborn(&reversed), None),
            finding,
            "the riding set's order never reaches the printed surface",
        );
    }

    /// **A path git's status does not report is routed at an act git performs on it** (the
    /// rc.24 fix pass, the ignored sibling of `(R1, F1)`).
    ///
    /// Every arm of the reported routes is an act on a path git can see: *commit or stash*
    /// on a born `HEAD`, *commit them* on an unborn one. Over an **ignored** file both are
    /// no-ops or failures, and over a tracked file whose index entry is **flagged**
    /// assume-unchanged or skip-worktree, *commit or stash* finds nothing to take — so a
    /// refusal that listed such a path beside the old route could not be cleared by
    /// following it. Each class gets its own arm, the message says why nothing else warned
    /// about the path, and the born state clause stops claiming `HEAD` holds the bytes.
    ///
    /// **With no unseen path listed, not a byte moves** — the refusal an adopter has always
    /// read is the one they still read.
    #[test]
    fn setup_boundary_finding_routes_an_unreported_path_at_an_act_that_reaches_it() {
        let home = Path::new("/somewhere/repo");
        let paths = |names: &[&str]| -> Vec<String> {
            names.iter().map(|name| (*name).to_string()).collect()
        };
        let dirty = paths(&["CLAUDE.md", ".jigc/AGENT.md", ".jigc/version"]);
        let ignored = paths(&[".jigc/AGENT.md"]);
        let flagged = paths(&[".jigc/version", "not/listed.md"]);
        fn unseen<'a>(
            home: &'a Path,
            ignored: &'a [String],
            flagged: &'a [String],
        ) -> Option<SetupUnseen<'a>> {
            Some(SetupUnseen {
                home,
                ignored,
                flagged,
            })
        }

        // Nothing unseen among the listed paths ⇒ the pre-existing bytes, whichever way
        // that is said.
        let plain = setup_dirty_install_finding(&dirty, false, SetupHead::Born, None);
        assert_eq!(
            setup_dirty_install_finding(&dirty, false, SetupHead::Born, unseen(home, &[], &[])),
            plain,
            "an empty answer words nothing",
        );
        assert_eq!(
            setup_dirty_install_finding(
                &dirty,
                false,
                SetupHead::Born,
                unseen(home, &[], &paths(&["not/listed.md"])),
            ),
            plain,
            "nor does a class naming only paths the refusal does not list",
        );

        // Born, one path of each kind.
        let finding = setup_dirty_install_finding(
            &dirty,
            false,
            SetupHead::Born,
            unseen(home, &ignored, &flagged),
        );
        assert_eq!(finding.code, CarryoverBoundary::Setup.code());
        assert!(finding.location.is_none(), "the set is still the subject");
        let message = finding.message.as_str();
        assert!(
            message.contains("3 path(s)")
                && message.contains("reports nothing at 2 of those path(s)")
                && message.contains("  `.jigc/AGENT.md`: ignored")
                && message.contains("  `.jigc/version`: tracked")
                && !message.contains("not/listed.md"),
            "the message says which listed paths git does not report, and why: {message:?}"
        );
        assert!(
            !message.contains("still has its pre-run bytes there")
                && message.contains("exactly where you left them"),
            "and does not say `HEAD` holds an ignored file's bytes: {message:?}"
        );
        let route = finding.route.as_ref().expect("blocking ⇒ routed").as_str();
        let at = |needle: &str| {
            route
                .find(needle)
                .unwrap_or_else(|| panic!("the route names `{needle}`: {route}"))
        };
        let (seen, clear, skip, moved, rerun, force) = (
            at("commit or stash the work at the path(s) `git status` does report"),
            at("git -C /somewhere/repo update-index --no-assume-unchanged -- .jigc/version"),
            at("git -C /somewhere/repo update-index --no-skip-worktree -- .jigc/version"),
            at("move the file out of `.jigc/AGENT.md`"),
            at("re-run `jigc setup`"),
            at("jigc setup --force"),
        );
        assert!(
            seen < clear && clear < skip && skip < moved && moved < rerun && rerun < force,
            "every resolving act, then the re-run, then the consent: {route}"
        );
        assert!(
            route[force..].contains("at `.jigc/AGENT.md` it replaces the file"),
            "the consent says what it buys at a path no commit will carry: {route}"
        );

        // Only unseen paths ⇒ no *commit or stash* arm at all: nothing listed is reported.
        let only = setup_dirty_install_finding(
            &ignored,
            false,
            SetupHead::Born,
            unseen(home, &ignored, &[]),
        );
        let only = only.route.as_ref().expect("routed").as_str().to_string();
        assert!(
            only.starts_with("move the file out of `.jigc/AGENT.md`") && !only.contains("stash -u"),
            "an ignored path alone is routed at the one act that works for it: {only}"
        );

        // Unborn: only *ignored* can be unreported, and git will not commit it.
        let unborn = SetupHead::Unborn { home, riding: &[] };
        let all = setup_dirty_install_finding(&ignored, false, unborn, unseen(home, &ignored, &[]));
        let all_route = all.route.as_ref().expect("routed").as_str();
        assert!(
            all_route.starts_with("move the file(s) out of those path(s), then re-run")
                && !all_route.contains("or commit"),
            "with every listed path ignored there is no commit arm to offer: {all_route}"
        );
        assert!(
            all.message.contains("exist only where you left them")
                && all.message.contains("  `.jigc/AGENT.md`: ignored"),
            "the unborn state clause stands, with the class named: {:?}",
            all.message
        );
        let some = paths(&[".jigc/AGENT.md", ".jigc/config/packs.yaml"]);
        let mixed = setup_dirty_install_finding(&some, false, unborn, unseen(home, &ignored, &[]));
        let mixed = mixed.route.as_ref().expect("routed").as_str().to_string();
        assert!(
            mixed.contains("or commit the one(s) git does not ignore")
                && mixed.contains("move `.jigc/AGENT.md` out"),
            "the commit arm names the path it does not cover: {mixed}"
        );

        // The classes exist at the pre-write ask alone.
        assert_eq!(
            setup_dirty_install_finding(
                &dirty,
                true,
                SetupHead::Born,
                unseen(home, &ignored, &flagged)
            ),
            setup_dirty_install_finding(&dirty, true, SetupHead::Born, None),
            "at the backstop the install has run, and nothing is said of a replaced file",
        );

        // Order-invariant over all three sets.
        let reversed = |set: &[String]| -> Vec<String> { set.iter().rev().cloned().collect() };
        assert_eq!(
            setup_dirty_install_finding(
                &reversed(&dirty),
                false,
                SetupHead::Born,
                unseen(home, &reversed(&ignored), &reversed(&flagged)),
            ),
            finding,
            "no set's order reaches the printed surface",
        );
    }
}
