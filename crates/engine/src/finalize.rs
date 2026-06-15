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
use crate::finding::{Finding, Location, Severity};
use crate::result::{SCHEMA_VERSION, ValidationReport};
use crate::schema::Schema;
use crate::state::BasePin;
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
/// doc's `location:`. Returns the [`FinalizePlan`] on a clean run, or the blocking
/// findings that aborted it.
///
/// Performs no git and no commit. Reads only the task working area (the engine's
/// existing filesystem effect); never shells out.
// The planner is a pure decision over a deliberately explicit set of inputs (the
// determinism contract feeds every layer in rather than re-deriving it); bundling
// them into a params struct would be churn without clarifying the contract.
#[allow(clippy::too_many_arguments)]
pub fn plan_finalize(
    task_dir: &Path,
    base: &BasePin,
    head_sha: &str,
    report: &ValidationReport,
    has_diff: bool,
    commit_schema: &Schema,
    commit_slug: &str,
    schemas: &BTreeMap<String, Schema>,
) -> Result<FinalizePlan, Vec<Finding>> {
    // Phase 1 — preflight: task exists, base pin == supplied HEAD.
    if !task_dir.exists() {
        return Err(vec![task_missing_finding(task_dir)]);
    }
    if base.sha != head_sha {
        return Err(vec![base_mismatch_finding(base, head_sha)]);
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
        return Err(vec![empty_commit_finding()]);
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

    // The retire set (`design/auto-migration.md` → Retire-the-foreign-original): a
    // migration task records its repo-relative foreign source path at mint; the planner
    // names it for retirement inside the commit closure. Empty on every non-migration
    // task (no `source-path`) — inert. The path-collision guard (→ Path-collision guard)
    // excludes a foreign source that equals a promote destination — the in-location
    // squatter the managed write rewrites in place is not a distinct original to retire.
    let retirements = plan_retirements(task_dir, &promote.promotions)?;

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
/// The **path-collision guard** (`design/auto-migration.md` → Path-collision guard, the
/// in-location squatter): when the recorded foreign source path **equals** a promote
/// destination (the canonical managed path `<location>/<slug>.md`), the managed write
/// *is* the in-place rewrite — the foreign file is not a distinct original, so the
/// retire is **skipped** (else the plan would delete the doc it just wrote). The common
/// root-`CHANGELOG.md` ≠ `changelog/changelog.md` case retires the distinct original.
fn plan_retirements(
    task_dir: &Path,
    promotions: &[Promotion],
) -> Result<Vec<PathBuf>, Vec<Finding>> {
    match crate::state::read_source_path(task_dir) {
        Ok(Some(path)) if !path.trim().is_empty() => {
            let foreign = PathBuf::from(path.trim());
            if promotions
                .iter()
                .any(|p| Path::new(&p.destination) == foreign)
            {
                Ok(Vec::new()) // in-location squatter — rewritten in place, not retired.
            } else {
                Ok(vec![foreign])
            }
        }
        Ok(_) => Ok(Vec::new()),
        Err(err) => Err(vec![source_path_io_finding(task_dir, &err)]),
    }
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
pub fn decide_base_repin(
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
    Err(vec![base_overlap_finding(base, head_sha, &overlapping)])
}

/// Plan the **milestone** `finalize` transaction — the thin sibling of
/// [`plan_finalize`] for the fan-out single-commit boundary (`design/finalize.md` →
/// `fan-out` finalize, single-commit form; `DECISIONS.md` 2026-06-04 → the inc-4
/// fork resolution: M7 ships one synthesized commit).
///
/// It shares [`plan_finalize`]'s phases — the preflight (`staging_dir` exists, `base`
/// == `head_sha`), the empty-commit guard, and the phase-4 promote / phase-7 hash sweep
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
pub fn plan_milestone_finalize(
    staging_dir: &Path,
    base: &BasePin,
    head_sha: &str,
    message: String,
    has_diff: bool,
    schemas: &BTreeMap<String, Schema>,
) -> Result<FinalizePlan, Vec<Finding>> {
    // Preflight (shared): the staging area exists, base pin == supplied HEAD.
    if !staging_dir.exists() {
        return Err(vec![task_missing_finding(staging_dir)]);
    }
    if base.sha != head_sha {
        return Err(vec![base_mismatch_finding(base, head_sha)]);
    }

    // Empty-commit guard (shared): validate-equivalent passed (the join adjudicated),
    // but a materialized-but-empty diff still aborts — no empty commits.
    if !has_diff {
        return Err(vec![empty_commit_finding()]);
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
/// instance whose type declares a `location:`, name a [`Promotion`] to its canonical
/// `<location>/<slug>.md` plus the blake3 of its staged bytes. Promotions are returned
/// in canonical-destination order (sorted, deterministic). The commit doc and any
/// unknown/transient type contribute nothing.
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
        // A type with no `location:` is transient (the commit doc) — never promoted.
        let Some(schema) = schemas.get(ty) else {
            continue;
        };
        let Some(location) = schema.location.as_deref() else {
            continue;
        };

        let bytes =
            std::fs::read(&source).map_err(|err| vec![promote_io_finding(&source, &err)])?;
        let destination = format!("{}/{slug}.md", location.trim_end_matches('/'));
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
/// doc during the promote phase.
fn promote_io_finding(path: &Path, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.promote-io",
        format!(
            "could not read the staged managed doc `{}` to promote it: {err}",
            path.display()
        ),
        None,
        None,
    )
}

/// A blocking finding for an I/O failure reading the migration task's recorded
/// `source-path` while planning the retire set.
fn source_path_io_finding(task_dir: &Path, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.source-path-io",
        format!(
            "could not read the recorded migration source path under `{}`: {err}",
            task_dir.display()
        ),
        None,
        None,
    )
}

/// The phase-1 task-missing reject (`finalize.md` → 1. Preflight: "Task exists").
fn task_missing_finding(task_dir: &Path) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.no-task",
        format!(
            "no task working area at `{}` — nothing to finalize",
            task_dir.display()
        ),
        None,
        Some("start a task with `jigc start \"<intent>\"`".to_string()),
    )
}

/// The phase-1 base-pin-divergence reject (`finalize.md` → 1. Preflight: "Base pin
/// matches HEAD"). A blocking finding carrying the divergence-routing prompt — the
/// CLI never operates a task off its pinned base.
fn base_mismatch_finding(base: &BasePin, head_sha: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.base-mismatch",
        format!(
            "the task was started at base `{}` but HEAD is now `{head_sha}`",
            base.short
        ),
        None,
        Some(format!(
            "switch back to `{}` or discard the task with `jigc task discard`",
            base.short
        )),
    )
}

/// The overlap form of the phase-1 base-divergence block (`finalize.md` → Parallel
/// hand-editing, the 2026-06-12 amendment): HEAD moved on history that **touches the
/// task's work**, so no auto-re-pin — the finding names the overlapping paths and
/// carries the resolve-or-discard conflict route. `overlapping` is sorted + deduped
/// by [`decide_base_repin`].
fn base_overlap_finding(base: &BasePin, head_sha: &str, overlapping: &[&str]) -> Finding {
    let paths = overlapping.join("`, `");
    Finding::graded(
        Severity::Blocking,
        "finalize.base-mismatch",
        format!(
            "the task was started at base `{}` but HEAD is now `{head_sha}`, and the moved \
             history overlaps the task's work on `{paths}`",
            base.short
        ),
        None,
        Some(format!(
            "resolve the overlap on `{paths}` against the new history, or discard the task \
             with `jigc task discard`"
        )),
    )
}

/// The empty-commit abort (`finalize.md` → Commit-doc rendering → Empty commit):
/// validate passed but the staged diff is empty. No empty commits.
fn empty_commit_finding() -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.empty-commit",
        "task validated but produced no diff — nothing to finalize",
        None,
        Some("make a change, then re-run `jigc task finalize`".to_string()),
    )
}

/// A blocking finding for an I/O failure reading the staged commit doc during render.
fn render_io_finding(path: &Path, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.render-io",
        format!(
            "could not read the staged commit doc `{}`: {err}",
            path.display()
        ),
        Some(Location::at(1, 1)),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state;
    use std::path::PathBuf;

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

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }

    /// The cascade-resolved schema set the planner reads in phase 4: the `commit`
    /// (transient, location-less) + `adr` (persisted to `decisions/`) types.
    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert("commit".to_string(), commit_schema());
        m.insert("adr".to_string(), adr_schema());
        m
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
        assert!(
            err[0].route.is_some(),
            "the divergence block carries a switch-back/discard route"
        );

        // (Phase 2) blocking validate findings → abort with exactly those.
        let blocking = Finding::graded(
            Severity::Blocking,
            "schema-conformance.required-slot-present",
            "required slot `body` is empty",
            None,
            None,
        );
        let advisory = Finding::graded(
            Severity::Advisory,
            "commit-rendering.line-limit-subject",
            "subject is 71 chars",
            None,
            None,
        );
        let report = ValidationReport::new(vec![advisory, blocking.clone()], &no_delta_resolved());
        let err = plan_finalize(
            &task_dir,
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
        assert!(
            err[0].message.contains("produced no diff"),
            "the empty-commit message is the produced-no-diff abort: {:?}",
            err[0]
        );

        // (Clean) base matches, validate clean, diff present → a plan with the
        // rendered message + the empty (commit-only) post-commit hash set.
        let plan = plan_finalize(
            &task_dir,
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
        let clean = ValidationReport::new(Vec::new(), &no_delta_resolved());

        // Omitting context: no `source-path` file → an empty retire set.
        let plan = plan_finalize(
            &task_dir,
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

    /// The path-collision guard (`design/auto-migration.md` → Path-collision guard,
    /// the in-location squatter): when the recorded foreign source path **equals** the
    /// canonical managed path a staged doc promotes to, the managed write *is* the
    /// in-place rewrite — the planner **skips** the retire (it would otherwise delete
    /// the doc it just wrote). A foreign source at a **distinct** path still retires
    /// (the common root-`CHANGELOG.md` case — T2 — is unaffected). One mechanism, both
    /// cases.
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

        // A distinct foreign path still retires — the root-`CHANGELOG.md` case (T2).
        crate::state::persist(&task_dir.join("source-path"), b"CHANGELOG.md")
            .expect("record a distinct foreign source path");
        let plan = plan_finalize(
            &task_dir,
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
                "decisions/single-node-cache.md": "1bd77717762c2872ef9b2db8ccb6126552044ed761a90d74a8866f48f9d1c5f9",
            },
        )
        "#
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

        // (Preflight) base != supplied HEAD → the SAME divergence-routing block, no plan.
        let err = plan_milestone_finalize(
            &staging,
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            message.to_string(),
            true,
            &schemas(),
        )
        .expect_err("a base mismatch aborts preflight");
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "finalize.base-mismatch");

        // (Preflight) a missing staging area aborts before anything.
        let err = plan_milestone_finalize(
            &root.path().join("milestones").join("none").join("merged"),
            &base(),
            &base().sha,
            message.to_string(),
            true,
            &schemas(),
        )
        .expect_err("a missing staging area aborts preflight");
        assert_eq!(err[0].code, "finalize.no-task");

        // (Empty-commit guard) base matches but no diff → produced-no-diff abort.
        let err = plan_milestone_finalize(
            &staging,
            &base(),
            &base().sha,
            message.to_string(),
            false,
            &schemas(),
        )
        .expect_err("an empty diff aborts");
        assert_eq!(err[0].code, "finalize.empty-commit");

        // (Clean) the plan carries the SUBSTITUTED synthesized message verbatim (never a
        // commit-doc render) and the SHARED promote/hash set over the materialized doc.
        let plan = plan_milestone_finalize(
            &staging,
            &base(),
            &base().sha,
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

    /// An unmoved base (`base == HEAD`) is **no-divergence** — the pin holds and
    /// the decision never blocks, whatever the task's dirty footprint looks like.
    #[test]
    fn repin_unmoved_base_is_no_divergence() {
        let root = TempRoot::new("repin-unmoved");
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        stage_filled_adr(&task_dir, "keep-sessions-in-memory");

        let decision = decide_base_repin(
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
}
