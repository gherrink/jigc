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
//! - **Phase 4 — promote** is a **structural no-op** in the commit-only case
//!   (`DECISIONS.md` 2026-05-31 → inc-4 planning pin: "`finalize` promote phase is a
//!   structural no-op in the commit-only case"): the commit doc's sink is the git
//!   message, never a repo file, and inc-4 has no persisted managed doc to promote
//!   (ADR create + promotion is inc-5).
//! - **Phase 7 — post-commit** (`finalize.md` → 7. Post-commit): the plan carries the
//!   **hash-update set** — `canonical-path → blake3 hash` for every *committed managed
//!   doc* the CLI records after the commit lands. In the commit-only case there are no
//!   promoted docs, so the set is empty (the commit doc is not a repo file). The
//!   edge-index stamp invalidation + working-area removal are CLI-side side effects of
//!   the committed plan (edge-index invalidation is inc-5 — no index yet).
//!
//! The planner returns a [`FinalizePlan`] the CLI executes, or a `Vec<Finding>` it
//! routes to the agent. It performs no git and no commit; rollback discipline
//! (`finalize.md` → Rollback discipline) is the CLI's concern, but the planner's
//! aborts are deliberately ordered so that **no plan is produced** on any blocking
//! branch — there is nothing to roll back before phase 3.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::finding::{Finding, Location, Severity};
use crate::result::{SCHEMA_VERSION, ValidationReport};
use crate::schema::Schema;
use crate::state::BasePin;
use crate::write;

/// The working-area sub-directory holding the task's staged doc instances
/// (`DECISIONS.md` 2026-05-31 → Task working-area on-disk layout).
const DOCS_DIR: &str = "docs";

/// The ordered `finalize` plan the engine hands the CLI to execute — the result of
/// a clean preflight + validate + empty-diff guard + render.
///
/// It carries the rendered commit-message string (phase 3) and the post-commit
/// hash-update set (phase 7: `committed-doc-path → blake3 hash`). The CLI is the
/// only side that touches git: it stages, runs `git commit -F` over [`message`],
/// and — after the commit lands — records the [`hash_updates`] into the `file-state`
/// record. Promote is a no-op in the commit-only case, so [`hash_updates`] is empty
/// here (the commit doc's sink is the git message, not a repo file).
///
/// [`message`]: FinalizePlan::message
/// [`hash_updates`]: FinalizePlan::hash_updates
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizePlan {
    /// The result-contract schema version (see [`SCHEMA_VERSION`]).
    pub schema_version: u32,
    /// The rendered git-commit message (phase 3) — no trailing newline; the CLI
    /// commits it via `git commit -F`.
    pub message: String,
    /// The post-commit hash-update set (phase 7): `committed-doc-path → blake3 hex`
    /// for every committed managed doc. Empty in the commit-only case (promote is a
    /// no-op; the commit doc is not a repo file).
    pub hash_updates: BTreeMap<String, String>,
}

impl FinalizePlan {
    /// Build a plan over a rendered message and a post-commit hash-update set,
    /// stamping the current [`SCHEMA_VERSION`].
    fn new(message: String, hash_updates: BTreeMap<String, String>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            message,
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
/// commit instance was provisioned under (`commit:<id>`). Returns the [`FinalizePlan`]
/// on a clean run, or the blocking findings that aborted it.
///
/// Performs no git and no commit. Reads only the task working area (the engine's
/// existing filesystem effect); never shells out.
pub fn plan_finalize(
    task_dir: &Path,
    base: &BasePin,
    head_sha: &str,
    report: &ValidationReport,
    has_diff: bool,
    commit_schema: &Schema,
    commit_slug: &str,
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

    // Phase 4 — promote: a structural no-op in the commit-only case, so the
    // phase-7 hash-update set is empty (no committed repo file to re-baseline).
    let hash_updates = BTreeMap::new();

    Ok(FinalizePlan::new(message, hash_updates))
}

/// The phase-1 task-missing reject (`finalize.md` → 1. Preflight: "Task exists").
fn task_missing_finding(task_dir: &Path) -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: "finalize.no-task".to_string(),
        message: format!(
            "no task working area at `{}` — nothing to finalize",
            task_dir.display()
        ),
        location: None,
        route: Some("start a task with `jigc start \"<intent>\"`".to_string()),
    }
}

/// The phase-1 base-pin-divergence reject (`finalize.md` → 1. Preflight: "Base pin
/// matches HEAD"). A blocking finding carrying the divergence-routing prompt — the
/// CLI never operates a task off its pinned base.
fn base_mismatch_finding(base: &BasePin, head_sha: &str) -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: "finalize.base-mismatch".to_string(),
        message: format!(
            "the task was started at base `{}` but HEAD is now `{head_sha}`",
            base.short
        ),
        location: None,
        route: Some(format!(
            "switch back to `{}` or discard the task with `jigc task discard`",
            base.short
        )),
    }
}

/// The empty-commit abort (`finalize.md` → Commit-doc rendering → Empty commit):
/// validate passed but the staged diff is empty. No empty commits.
fn empty_commit_finding() -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: "finalize.empty-commit".to_string(),
        message: "task validated but produced no diff — nothing to finalize".to_string(),
        location: None,
        route: Some("make a change, then re-run `jigc task finalize`".to_string()),
    }
}

/// A blocking finding for an I/O failure reading the staged commit doc during render.
fn render_io_finding(path: &Path, err: &std::io::Error) -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: "finalize.render-io".to_string(),
        message: format!(
            "could not read the staged commit doc `{}`: {err}",
            path.display()
        ),
        location: Some(Location {
            address: None,
            line: 1,
            col: 1,
        }),
        route: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state;
    use std::path::PathBuf;

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

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
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
        let clean = ValidationReport::new(Vec::new());

        // (Preflight) base-pin != supplied HEAD → divergence block, no plan.
        let err = plan_finalize(
            &task_dir,
            &base(),
            "ffffffffffffffffffffffffffffffffffffffff",
            &clean,
            true,
            &schema,
            "add-rate-limiter",
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
        let blocking = Finding {
            severity: Severity::Blocking,
            code: "schema-conformance.required-slot-present".into(),
            message: "required slot `body` is empty".into(),
            location: None,
            route: None,
        };
        let advisory = Finding {
            severity: Severity::Advisory,
            code: "commit-rendering.line-limit-subject".into(),
            message: "subject is 71 chars".into(),
            location: None,
            route: None,
        };
        let report = ValidationReport::new(vec![advisory, blocking.clone()]);
        let err = plan_finalize(
            &task_dir,
            &base(),
            &base().sha,
            &report,
            true,
            &schema,
            "add-rate-limiter",
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
        )
        .expect("a clean task yields a plan");
        assert_eq!(
            plan.message, "feat: add a per-client rate limiter",
            "the plan carries the rendered commit message (phase 3)"
        );
        assert!(
            plan.hash_updates.is_empty(),
            "promote is a no-op in the commit-only case, so the hash set is empty"
        );
        assert_eq!(plan.schema_version, SCHEMA_VERSION);
    }

    /// A missing task working area aborts preflight before anything else.
    #[test]
    fn finalize_planner_rejects_a_missing_task() {
        let root = TempRoot::new("missing");
        let task_dir = root.path().join("tasks").join("nonexistent");
        let clean = ValidationReport::new(Vec::new());

        let err = plan_finalize(
            &task_dir,
            &base(),
            &base().sha,
            &clean,
            true,
            &commit_schema(),
            "nonexistent",
        )
        .expect_err("a missing task aborts preflight");
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "finalize.no-task");
        assert_eq!(err[0].severity, Severity::Blocking);
    }
}
