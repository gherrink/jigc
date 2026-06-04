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

use std::collections::BTreeMap;
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
}

impl FinalizePlan {
    /// Build a plan over a rendered message, a promote set, and a post-commit
    /// hash-update set, stamping the current [`SCHEMA_VERSION`].
    fn new(
        message: String,
        promotions: Vec<Promotion>,
        hash_updates: BTreeMap<String, String>,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            message,
            promotions,
            hash_updates,
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

    Ok(FinalizePlan::new(
        message,
        promote.promotions,
        promote.hash_updates,
    ))
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
        crate::schema::load_schema(ADR_YAML).expect("adr.yaml loads")
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
}
