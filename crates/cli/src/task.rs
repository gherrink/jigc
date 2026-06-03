//! `jigc task <verb> <id>` — the task lifecycle surface over a named task's working
//! area (`design/write-commands.md` → Lifecycle: `diff` · `validate` · `discard`).
//!
//! Unlike `jigc doc` (which resolves the *single* active task from cwd), the
//! lifecycle verbs take the task `<id>` **explicitly** — the explicit selector the
//! `doc` surface deferred here. Three preview/abandon verbs land in this task
//! (`finalize` is the next one):
//!
//! - **`diff`** — the working changeset vs base: the git diff of the working tree
//!   against the task's pinned base SHA (code) plus the staged managed-doc instances
//!   (which live in the working area, outside the tree). Read-only, always exit 0.
//! - **`validate`** — run `engine::validate::validate_task` over the working area
//!   and render the findings; the exit code tracks `has_blocking()` so validate
//!   *previews what finalize blocks on* (`design/validation.md` → How it gates
//!   `finalize`; `design/reconciliation.md` → Detection timing: the full sweep).
//! - **`discard`** — remove `.jigc/tasks/<id>/`, abandoning the task. Idempotent on
//!   an already-absent area; always exit 0 on success.
//! - **`finalize`** — the commit boundary: read HEAD, ask the engine planner for the
//!   ordered phase plan (preflight → validate → empty-diff guard → render), and on a
//!   clean plan execute the git steps the plan implies — write the rendered message to
//!   a temp file, `git add` the working-tree code changes, `git commit -F <tmp>`
//!   (**never** `--no-verify`), then post-commit (advance the file-state hashes, remove
//!   the working area). A blocking finding or a hook/git rejection aborts cleanly with
//!   the findings / stderr surfaced and **no** commit (`design/finalize.md` → 5. Stage
//!   / 6. Commit / 7. Post-commit; `design/worked-examples.md` → flow #4).
//!
//! Findings render through the shared `crate::render` finding/format renderers (the
//! determinism boundary unaffected — the engine carries the data, the CLI formats).

use crate::cli::Format;
use crate::pack::make_pack;
use crate::render;
use anyhow::{Context, Result, bail};
use engine::address::Address;
use engine::compose::{WorkflowDef, load_workflow_def};
use engine::file_state::{self, FileStateRecord};
use engine::finalize::{Promotion, plan_finalize};
use engine::finding::Finding;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::{Schema, load_schema};
use engine::state::{self, BasePin, RolesRecord};
use engine::store::canonical_path;
use engine::validate::validate_task;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// The `jigc task <verb>` subcommand tree. Each verb names a task by its `<id>`
/// (`design/write-commands.md` → Lifecycle).
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum TaskCommand {
    /// Show the working changeset vs base (code diff + staged managed docs).
    Diff {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
    /// Run `validate(task)` and render the findings; exit non-zero iff any blocks.
    Validate {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
    /// Abandon the task — remove its working area `.jigc/tasks/<id>/`.
    Discard {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
    /// The commit boundary — validate, render, stage, `git commit`, post-commit.
    Finalize {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
    /// Bind an already-committed doc to one of the task's declared context roles,
    /// so `task.<role>` resolves to it on the resume re-compose
    /// (`design/write-commands.md` → Binding a context role).
    Bind {
        /// The context role to bind — one of the workflow's declared `reads` roles.
        role: String,
        /// The `<type>:<slug>` address of the committed doc to bind.
        addr: String,
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
}

impl TaskCommand {
    /// Dispatch the parsed `task` verb against `cwd`, mapping the result to a process
    /// exit code. `validate` maps a blocking report to a non-zero exit (the gate
    /// preview); `diff` / `discard` exit 0 on success. An orchestration error
    /// surfaces on stderr with a non-zero exit.
    pub fn dispatch(self, cwd: &Path, format: Format) -> ExitCode {
        let result = match self {
            TaskCommand::Diff { id } => run_diff(cwd, &id),
            TaskCommand::Validate { id } => return run_validate(cwd, &id, format),
            TaskCommand::Discard { id } => run_discard(cwd, &id),
            TaskCommand::Finalize { id } => return run_finalize(cwd, &id, format),
            TaskCommand::Bind { role, addr, id } => run_bind(cwd, &role, &addr, &id),
        };
        match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err:#}");
                ExitCode::FAILURE
            }
        }
    }
}

/// `jigc task diff <id>` — print the working changeset vs base.
///
/// Two parts: (1) the git diff of the working tree against the task's pinned base
/// SHA (the code changes — "CLI orchestrates, git executes"), and (2) the staged
/// managed-doc instances under the working area's `docs/` (which live outside the
/// tree, so git does not see them). Read-only.
fn run_diff(cwd: &Path, id: &str) -> Result<()> {
    let task = TaskArea::resolve(cwd, id)?;
    let base = task.base()?;

    let diff = git_diff(&task.repo_root, &base.sha)?;
    if !diff.trim().is_empty() {
        println!("# code changes vs base {}", base.short);
        print!("{diff}");
        if !diff.ends_with('\n') {
            println!();
        }
    }

    let staged = task.staged_docs()?;
    if !staged.is_empty() {
        println!("# staged docs");
        for (name, body) in staged {
            println!("--- {name}");
            print!("{body}");
            if !body.ends_with('\n') {
                println!();
            }
        }
    }
    Ok(())
}

/// `jigc task validate <id>` — run the task-scope sweep and render its findings; the
/// exit code tracks `has_blocking()` (`design/validation.md` → How it gates
/// `finalize`: validate previews what finalize blocks on). The findings render
/// through `crate::render::validation` in the selected format.
fn run_validate(cwd: &Path, id: &str, format: Format) -> ExitCode {
    let task = match TaskArea::resolve(cwd, id) {
        Ok(task) => task,
        Err(err) => {
            eprintln!("{err:#}");
            return ExitCode::FAILURE;
        }
    };
    match task.validate() {
        Ok(report) => {
            print!("{}", render::validation(format, &report));
            if format != Format::Json {
                println!();
            }
            if report.has_blocking() {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("{err:#}");
            ExitCode::FAILURE
        }
    }
}

/// `jigc task discard <id>` — remove the task's working area, abandoning it
/// (`design/write-commands.md` → Lifecycle: abandon). Idempotent: an already-absent
/// area is not an error.
fn run_discard(cwd: &Path, id: &str) -> Result<()> {
    let task = TaskArea::resolve(cwd, id)?;
    if task.dir.exists() {
        std::fs::remove_dir_all(&task.dir)
            .with_context(|| format!("could not discard task `{id}` at {:?}", task.dir))?;
    }
    Ok(())
}

/// `jigc task bind <role> <addr> <id>` — bind an already-committed doc to one of the
/// task's declared context roles (`design/write-commands.md` → Binding a context
/// role: the five-step enforcement; `workflow-dialect.md` → `reads`).
///
/// Five-step enforcement: (1) the task `<id>` resolves to a live working area
/// (`TaskArea::resolve` bails with the start-a-task route otherwise); (2) `<role>`
/// is one of the task's workflow-declared `reads` roles, else reject listing the
/// declared roles; (3) `<addr>` resolves in the committed store — its canonical
/// `<location>/<slug>.md` exists at HEAD — else `no such doc <addr>`; (4) the
/// target's doctype (the address's `<type>`) equals the role's declared `type`,
/// else reject with the mismatch; (5) record the binding in the task's
/// `roles.json` (last-write-wins). The agent supplies only the which-doc choice;
/// recording the binding stays the CLI's (the determinism boundary holds).
fn run_bind(cwd: &Path, role: &str, addr: &str, id: &str) -> Result<()> {
    let task = TaskArea::resolve(cwd, id)?;
    task.bind(role, addr)
}

/// The commit doctype name — the task's workflow-provisioned doc whose sink is the
/// git message (`design/finalize.md` → Commit-doc rendering). The commit instance is
/// provisioned under `commit:<task-id>`, so the commit slug is the task id.
const COMMIT_TYPE: &str = "commit";

/// `jigc task finalize <id>` — execute the commit boundary (`design/finalize.md` →
/// 5. Stage / 6. Commit / 7. Post-commit; `design/worked-examples.md` → flow #4).
///
/// The engine is a pure planner ([`plan_finalize`]); the CLI owns every git step. The
/// loop: read HEAD (`git rev-parse`), run `validate(task)`, compute the diff-presence
/// signal, load the commit schema, and ask the planner for the ordered plan. On a
/// blocking finding (base-mismatch, validate block, empty diff) the planner returns the
/// findings — rendered, non-zero exit, **no** commit. On a clean plan: write the
/// rendered message to a temp file, `git add` the working-tree code changes between base
/// and tree, `git commit -F <tmp>` (**never** `--no-verify`). A hook / git rejection
/// aborts non-zero with git's stderr surfaced and no working-area change. On a successful
/// commit, run post-commit (advance the file-state hashes, remove the working area) —
/// best-effort: a failure there is logged, not raised (the commit is already truth).
fn run_finalize(cwd: &Path, id: &str, format: Format) -> ExitCode {
    let task = match TaskArea::resolve(cwd, id) {
        Ok(task) => task,
        Err(err) => {
            eprintln!("{err:#}");
            return ExitCode::FAILURE;
        }
    };
    match task.finalize(id, format) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{err:#}");
            ExitCode::FAILURE
        }
    }
}

/// A named task's working area: the repo root, the `.jigc/` home, and the task dir.
struct TaskArea {
    repo_root: PathBuf,
    jigc_root: PathBuf,
    dir: PathBuf,
    pack: Box<dyn PackSource>,
}

impl TaskArea {
    /// Resolve the task `id`'s working area from `cwd`. The repo root is the
    /// nearest `.git` ancestor; the task dir is `<repo>/.jigc/tasks/<id>/`. A
    /// task that does not exist rejects with the start-a-task route.
    fn resolve(cwd: &Path, id: &str) -> Result<Self> {
        let repo_root = discover_repo_root(cwd)
            .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
        let jigc_root = repo_root.join(".jigc");
        let dir = jigc_root.join("tasks").join(id);
        if !dir.is_dir() {
            bail!("no task `{id}` — start one with `jigc start \"<intent>\"`");
        }
        Ok(Self {
            repo_root,
            jigc_root,
            dir,
            pack: make_pack(),
        })
    }

    /// Read the task's pinned base commit.
    fn base(&self) -> Result<BasePin> {
        let path = self.dir.join("base.json");
        let bytes = std::fs::read(&path)
            .with_context(|| format!("could not read the base pin at {path:?}"))?;
        serde_json::from_slice(&bytes).with_context(|| format!("malformed base pin at {path:?}"))
    }

    /// Load every shipped schema, keyed by doctype — the set the conformance sweep
    /// resolves staged instances against (the engine stays domain-empty; the CLI
    /// feeds the cascade in).
    fn schemas(&self) -> Result<BTreeMap<String, Schema>> {
        let mut out = BTreeMap::new();
        for id in self.pack.list(PackResourceKind::Schemas) {
            let bytes = self
                .pack
                .read(PackResourceKind::Schemas, &id)
                .with_context(|| format!("the `{}` schema reads back", id.as_str()))?;
            let schema = load_schema(&bytes)
                .with_context(|| format!("the `{}` schema parses", id.as_str()))?;
            out.insert(schema.ty.clone(), schema);
        }
        Ok(out)
    }

    /// Run the task-scope validation sweep against the committed-state `file-state`
    /// record (`design/validation.md` → How it gates `finalize`; `design/reconciliation.md`
    /// → Detection timing: the `task validate` full sweep).
    ///
    /// The record is **loaded** (so drift of a committed doc against its recorded
    /// baseline is detected) but **not persisted** back from `validate`: the record
    /// advances only at adopt/absorb/commit, and a `validate` is none of those for
    /// committed state — `finalize` phase 7 re-derives the committed hashes from the
    /// just-landed commit. So a working-area instance with no committed baseline
    /// baseline-adopts fresh (advisory) each run rather than drifting against a stale
    /// staged hash — clean validate stays clean as the agent fills it.
    ///
    /// The sweep runs `engine::file_state::reconcile_committed_store` over the committed
    /// store (`reconciliation.md` → Detection timing: the `task validate` full sweep,
    /// shared by `finalize`'s preflight) — routing OOB drift on a committed `decisions/*.md`
    /// (absorb the conformant, conformance/conflict-block the rest, detect renames) — and
    /// `schema-conformance.ref-resolves` (forward-ref / edge-index integrity). Both need
    /// the committed-store root, the `.jigc/` home (where the edge index caches), and the
    /// current HEAD (read via `git`, keeping the engine shell-free). The two reachable
    /// surfaces (committed store + this task's working area) make `validate` preview
    /// exactly the forward-ref block `finalize` gates on.
    fn validate(&self) -> Result<engine::result::ValidationReport> {
        let schemas = self.schemas()?;
        let head = git_head(&self.repo_root)?;
        let mut record = FileStateRecord::load(&self.jigc_root).with_context(|| {
            format!(
                "could not load the file-state record under {:?}",
                self.jigc_root
            )
        })?;
        validate_task(
            &self.dir,
            &schemas,
            &mut record,
            &self.repo_root,
            &self.jigc_root,
            &head,
        )
        .with_context(|| format!("validating task at {:?}", self.dir))
    }

    /// Execute the `finalize` transaction (`design/finalize.md` → 5–7). Returns the
    /// process exit code: `SUCCESS` on a landed commit, `FAILURE` on any planner block
    /// (rendered + surfaced) or hook/git rejection (git's stderr surfaced). An
    /// orchestration error (git unavailable, malformed pin) bubbles as `Err`.
    fn finalize(&self, id: &str, format: Format) -> Result<ExitCode> {
        let base = self.base()?;
        let head = git_head(&self.repo_root)?;

        // The validate report the planner gates on — one engine, two entry points
        // (`design/finalize.md` → 2. Validate: no private check path).
        let report = self.validate()?;

        // The diff-presence signal the planner's empty-commit guard needs: any
        // working-tree change from base, any staged managed doc, or any untracked
        // file. `git diff <base>` lists only tracked changes, but the stage step
        // (`git add --all`) also commits untracked files — so they count too, else
        // an untracked-only task would abort as falsely "empty".
        let has_diff = !git_diff(&self.repo_root, &base.sha)?.trim().is_empty()
            || !self.staged_docs()?.is_empty()
            || !git_untracked(&self.repo_root)?.trim().is_empty();

        let schemas = self.schemas()?;
        let commit_schema = schemas
            .get(COMMIT_TYPE)
            .with_context(|| format!("the embedded pack ships no `{COMMIT_TYPE}` schema"))?;

        // The engine plans; the CLI executes. A blocking branch returns the findings.
        let plan = match plan_finalize(
            &self.dir,
            &base,
            &head,
            &report,
            has_diff,
            commit_schema,
            id,
            &schemas,
        ) {
            Ok(plan) => plan,
            Err(findings) => {
                let report = engine::result::ValidationReport::new(findings);
                eprint!("{}", render::validation(format, &report));
                if format != Format::Json {
                    eprintln!();
                }
                return Ok(ExitCode::FAILURE);
            }
        };

        // Phase 4–6: promote + stage + commit. Write the rendered message to a temp
        // file, copy each promoted managed doc to its canonical path (`design/finalize.md`
        // → 4. Promote managed docs — copy, not move), stage the working-tree code
        // changes + the promoted docs (everything differing from base —
        // `design/finalize.md` → Dirty-tree policy / 5. Stage), and `git commit -F <tmp>`
        // (never `--no-verify`). A hook/git rejection surfaces git's stderr verbatim,
        // rolls back the promoted copies, and lands no commit.
        let msg_path = self.dir.join("finalize-message.tmp");
        std::fs::write(&msg_path, &plan.message)
            .with_context(|| format!("could not write the commit message to {msg_path:?}"))?;
        let commit_result = (|| -> Result<()> {
            // Phase 4 — promote: copy each staged managed doc to `<repo>/<destination>`.
            self.promote(&plan.promotions)?;
            // The transient `.jigc/` subdirs (`tasks/`/`index/`/`state/`) are gitignored
            // via `.jigc/.gitignore` — the working area is never committed
            // (`design/storage.md` → repository layout). Ensure it exists so the
            // `git add --all` stage below picks up `config/` + the promoted docs + the
            // code changes.
            self.ensure_jigc_gitignore()?;
            git_run(&self.repo_root, &["add", "--all"])?;
            git_commit(&self.repo_root, &msg_path)?;
            Ok(())
        })();
        let _ = std::fs::remove_file(&msg_path);
        if let Err(err) = commit_result {
            // Roll back phases 4–5 (`design/finalize.md` → Rollback discipline): restore
            // HEAD content for the promoted paths and delete the promoted copies. The
            // working area is untouched; the agent re-runs after fixing.
            self.rollback_promotions(&plan.promotions);
            eprintln!("{err:#}");
            return Ok(ExitCode::FAILURE);
        }

        // Phase 7 — post-commit (best-effort: a failure here is logged, not raised; the
        // commit is already truth). Advance the file-state hashes for the committed
        // working set, then remove the working area.
        self.post_commit(&plan.hash_updates);

        Ok(ExitCode::SUCCESS)
    }

    /// Phase 4 (`design/finalize.md` → 4. Promote managed docs). Copy each staged
    /// managed doc from its working-area source to its canonical repo path
    /// (`<repo_root>/<destination>`) — **copy, not move**, so rollback is a removal of
    /// the copies and the working area stays intact. Creates the destination's parent
    /// directory (e.g. `decisions/`) when absent.
    fn promote(&self, promotions: &[Promotion]) -> Result<()> {
        for promotion in promotions {
            let dest = self.repo_root.join(&promotion.destination);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("could not create {parent:?} to promote into"))?;
            }
            std::fs::copy(&promotion.source, &dest).with_context(|| {
                format!(
                    "could not promote {:?} to {dest:?}",
                    promotion.source.display()
                )
            })?;
        }
        Ok(())
    }

    /// Roll back phase 4–5 on a commit failure (`design/finalize.md` → Rollback
    /// discipline / 6. Commit). For each promoted path, restore HEAD's content in the
    /// index + worktree (undoing the stage) and delete the promoted copy. Best-effort:
    /// a failure is logged, never raised — the commit did not land, so the worst case is
    /// a stray copy the next `finalize`/`discard` overwrites.
    fn rollback_promotions(&self, promotions: &[Promotion]) {
        for promotion in promotions {
            let _ = git_run(
                &self.repo_root,
                &["restore", "--staged", "--worktree", &promotion.destination],
            );
            let dest = self.repo_root.join(&promotion.destination);
            // `git restore` recreates the path only if it existed at HEAD; a freshly
            // promoted (new) doc has no HEAD content, so remove the copy outright.
            if !path_at_head(&self.repo_root, &promotion.destination) {
                let _ = std::fs::remove_file(&dest);
            }
        }
    }

    /// Ensure `.jigc/.gitignore` ignores the transient subdirs (`tasks/`/`index/`/
    /// `state/`) so a `finalize` stage never commits the working area or the
    /// rebuildable caches (`design/storage.md` → repository layout: `.jigc/` is one home
    /// whose `config/` is committed while `tasks/`/`index/`/`state/` are gitignored).
    /// Idempotent — written only when absent.
    fn ensure_jigc_gitignore(&self) -> Result<()> {
        let path = self.jigc_root.join(".gitignore");
        if path.exists() {
            return Ok(());
        }
        std::fs::create_dir_all(&self.jigc_root)
            .with_context(|| format!("could not create {:?}", self.jigc_root))?;
        std::fs::write(&path, "tasks/\nindex/\nstate/\n")
            .with_context(|| format!("could not write {path:?}"))?;
        Ok(())
    }

    /// Phase 7 (`design/finalize.md` → 7. Post-commit, best-effort). Three updates, none
    /// of which can affect commit truth, all self-healing: advance the `file-state`
    /// record for every committed file (the plan's managed-doc hash set plus the
    /// committed working-tree files), **invalidate the edge-index stamp** (lifecycle
    /// site 5, `design/storage.md` → Edge index lifecycle — remove the persisted index
    /// so the next read rebuilds against the new HEAD), then remove the working area.
    /// Each step self-heals on failure — a stale `.jigc/tasks/<id>/` is cleaned by the
    /// next `discard`/`finalize`, a stale hash re-baselines on the next probe, and the
    /// edge index re-derives from the committed `.md`s — so a failure is logged to
    /// stderr, never raised (the commit is already truth).
    fn post_commit(&self, hash_updates: &BTreeMap<String, String>) {
        if let Err(err) = self.advance_file_state(hash_updates) {
            eprintln!("note: post-commit file-state update failed (self-heals): {err:#}");
        }
        if let Err(err) = engine::index::invalidate(&self.jigc_root) {
            eprintln!("note: post-commit edge-index invalidation failed (self-heals): {err:#}");
        }
        if let Err(err) = std::fs::remove_dir_all(&self.dir) {
            eprintln!("note: post-commit working-area removal failed (self-heals): {err:#}");
        }
    }

    /// Record the committed working set into the `file-state` record and save it. The
    /// plan's managed-doc hash set is empty in the commit-only case; the committed
    /// code files (the working-tree changes that just landed) are hashed from `HEAD`'s
    /// tree so the next `file-state` probe sees them in-sync.
    fn advance_file_state(&self, hash_updates: &BTreeMap<String, String>) -> Result<()> {
        let mut record = FileStateRecord::load(&self.jigc_root)
            .with_context(|| format!("loading the file-state record under {:?}", self.jigc_root))?;
        for (path, hash) in hash_updates {
            record.record(path.clone(), hash.clone());
        }
        // Hash the files the just-landed commit touched, reading their committed bytes.
        for path in git_commit_files(&self.repo_root)? {
            if let Ok(bytes) = git_show_file(&self.repo_root, &path) {
                record.record(path, file_state::hash_bytes(&bytes));
            }
        }
        record
            .save(&self.jigc_root)
            .with_context(|| format!("saving the file-state record under {:?}", self.jigc_root))?;
        Ok(())
    }

    /// Steps 2–5 of the bind enforcement (`design/write-commands.md` → Binding a
    /// context role). Step 1 (active task) is `TaskArea::resolve`, run by the caller.
    ///
    /// Loads the task's own minting workflow def (the `reads` declaration lives on
    /// its front-matter, the same source the resume re-compose reads), then:
    /// 2. `role` ∈ `def.reads` else reject, listing the declared roles;
    /// 3. `<addr>` parses + its canonical committed path exists else `no such doc <addr>`;
    /// 4. the address's `<type>` equals the role's declared `type` else the mismatch;
    /// 5. record `role -> <addr>` in `roles.json` (last-write-wins).
    fn bind(&self, role: &str, addr: &str) -> Result<()> {
        let def = self.workflow_def()?;

        // Step 2 — the role must be one the workflow declares as a `reads` role.
        let Some(reads) = def.reads.iter().find(|r| r.role == role) else {
            let declared: Vec<&str> = def.reads.iter().map(|r| r.role.as_str()).collect();
            let declared = if declared.is_empty() {
                "none".to_string()
            } else {
                declared.join(", ")
            };
            bail!("role `{role}` is not a declared read-role of this task (declared: {declared})");
        };

        // Parse the address (it must name a `<type>:<slug>`).
        let address = Address::parse(addr)
            .map_err(|err| anyhow::anyhow!("malformed address `{addr}`: {err}"))?;

        // Step 3 — the addr must resolve in the committed store: its canonical
        // `<location>/<slug>.md` must exist (identity is the path). An unknown type
        // or a transient (location-less) type has no committed path → unresolved.
        let schemas = self.schemas()?;
        let resolves = schemas
            .get(address.r#type.as_str())
            .and_then(|schema| canonical_path(&self.repo_root, schema, address.slug.as_str()))
            .is_some_and(|path| path.is_file());
        if !resolves {
            bail!("no such doc `{addr}`");
        }

        // Step 4 — the target's doctype must equal the role's declared `type`.
        if address.r#type.as_str() != reads.doc_type {
            bail!(
                "doctype mismatch: `{addr}` is a `{}` but role `{role}` declares type `{}`",
                address.r#type.as_str(),
                reads.doc_type,
            );
        }

        // Step 5 — record the binding (last-write-wins) and persist it.
        let mut roles = RolesRecord::load(&self.dir)
            .with_context(|| format!("could not read roles for task at {:?}", self.dir))?;
        roles.bind(role, addr);
        roles
            .save(&self.dir)
            .with_context(|| format!("could not record the binding for task at {:?}", self.dir))?;
        Ok(())
    }

    /// Load the task's own minting workflow definition — the source the `reads`
    /// declaration lives on, read back the same way the resume re-compose does
    /// (`crate::start::resume_in_repo`). A working area with no recorded workflow id
    /// is a clear fault, never a silent fall-through to the cascade default.
    fn workflow_def(&self) -> Result<WorkflowDef> {
        let workflow_id = state::read_workflow_id(&self.dir)
            .with_context(|| format!("could not read the recorded workflow for {:?}", self.dir))?
            .with_context(|| {
                format!(
                    "task at {:?} has no recorded workflow — discard it and re-start with `jigc start`",
                    self.dir
                )
            })?;
        let bytes = self
            .pack
            .read(
                PackResourceKind::Workflows,
                &ResourceId::from(workflow_id.as_str()),
            )
            .with_context(|| format!("the recorded workflow `{workflow_id}` reads back"))?;
        load_workflow_def(&bytes).map_err(finding_to_err)
    }

    /// The staged managed-doc instances under the working area's `docs/`, in
    /// path-sorted filename order, as `(filename, body)` pairs. An absent `docs/`
    /// yields an empty list (nothing staged yet).
    fn staged_docs(&self) -> Result<Vec<(String, String)>> {
        let docs = self.dir.join("docs");
        let read = match std::fs::read_dir(&docs) {
            Ok(read) => read,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => {
                return Err(err).with_context(|| format!("reading staged docs in {docs:?}"));
            }
        };
        let mut names: Vec<String> = Vec::new();
        for entry in read {
            let entry = entry.context("reading a staged-docs entry")?;
            if !entry
                .file_type()
                .context("staged-docs entry type")?
                .is_file()
            {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.ends_with(".md") {
                names.push(name);
            }
        }
        names.sort();
        let mut out = Vec::with_capacity(names.len());
        for name in names {
            let body = std::fs::read_to_string(docs.join(&name))
                .with_context(|| format!("reading staged doc {name}"))?;
            out.push((name, body));
        }
        Ok(out)
    }
}

/// Run `git diff <base>` in `repo_root`, returning the unified diff of the working
/// tree against the pinned base commit (`storage.md` → CLI and git: "CLI
/// orchestrates, git executes"). Shells out to the user's `git`
/// (`DECISIONS.md` 2026-05-31 → Git invocation).
fn git_diff(repo_root: &Path, base_sha: &str) -> Result<String> {
    let out = Command::new("git")
        .args(["diff", base_sha])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git diff {base_sha}` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    String::from_utf8(out.stdout).context("`git diff` produced non-UTF-8 output")
}

/// List untracked, non-ignored files via `git ls-files --others --exclude-standard`.
/// `git diff <base>` never reports these, but the finalize stage (`git add --all`)
/// commits them — so the empty-commit guard counts them as a diff signal.
fn git_untracked(repo_root: &Path) -> Result<String> {
    let out = Command::new("git")
        .args(["ls-files", "--others", "--exclude-standard"])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git ls-files --others` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    String::from_utf8(out.stdout).context("`git ls-files` produced non-UTF-8 output")
}

/// Read HEAD's full SHA via `git rev-parse HEAD` (the supplied HEAD the planner
/// checks the base pin against — `design/finalize.md` → 1. Preflight). Shells out to
/// the user's `git` (`DECISIONS.md` 2026-05-31 → Git invocation).
fn git_head(repo_root: &Path) -> Result<String> {
    git_capture(repo_root, &["rev-parse", "HEAD"])
}

/// Run `git <args>` in `repo_root` for its side effect (e.g. `add`), bailing with
/// git's stderr on a non-zero exit. Shells out to the user's `git`.
fn git_run(repo_root: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// `git commit -F <message-file>` (`design/finalize.md` → 6. Commit). **Never** passes
/// `--no-verify`: the user's `pre-commit` / `commit-msg` hooks are policy and the CLI
/// respects them — a hook rejection surfaces git's stderr verbatim (the correction
/// signal), and no commit lands.
fn git_commit(repo_root: &Path, message_file: &Path) -> Result<()> {
    let out = Command::new("git")
        .arg("commit")
        .arg("-F")
        .arg(message_file)
        .current_dir(repo_root)
        .output()
        .context("could not run `git commit` (is git on PATH?)")?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stdout = String::from_utf8_lossy(&out.stdout);
        bail!(
            "`git commit` was rejected (no commit was made):\n{}{}",
            stdout.trim(),
            stderr.trim()
        );
    }
    Ok(())
}

/// The paths the just-landed `HEAD` commit touched (`git show --name-only`). Used by
/// post-commit to advance the `file-state` hashes for the committed working set.
fn git_commit_files(repo_root: &Path) -> Result<Vec<String>> {
    let out = git_capture(
        repo_root,
        &["show", "--name-only", "--format=", "--no-renames", "HEAD"],
    )?;
    Ok(out
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Whether `path` (repo-relative) exists at `HEAD` (`git cat-file -e HEAD:<path>`).
/// Used by rollback to tell a newly promoted doc (no HEAD content — delete the copy)
/// from an overwrite of an existing committed doc (`git restore` already reverted it).
fn path_at_head(repo_root: &Path, path: &str) -> bool {
    Command::new("git")
        .arg("cat-file")
        .arg("-e")
        .arg(format!("HEAD:{path}"))
        .current_dir(repo_root)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Read a file's committed bytes at `HEAD` (`git show HEAD:<path>`). Returns an error
/// for a path absent at HEAD (e.g. a deletion); the caller skips those.
fn git_show_file(repo_root: &Path, path: &str) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .arg("show")
        .arg(format!("HEAD:{path}"))
        .current_dir(repo_root)
        .output()
        .context("could not run `git show`")?;
    if !out.status.success() {
        bail!("`git show HEAD:{path}` failed");
    }
    Ok(out.stdout)
}

/// Run `git <args>` in `repo_root`, returning trimmed stdout, bailing on non-zero.
fn git_capture(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    String::from_utf8(out.stdout)
        .context("`git` produced non-UTF-8 output")
        .map(|s| s.trim().to_string())
}

/// Map an engine [`Finding`] (a malformed workflow def) to an `anyhow` error
/// carrying its message + route — the same envelope `crate::doc` / `crate::start`
/// use for a load-time block.
fn finding_to_err(finding: Finding) -> anyhow::Error {
    match finding.route {
        Some(route) => anyhow::anyhow!("{}\n  route: {route}", finding.message),
        None => anyhow::anyhow!("{}", finding.message),
    }
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the same
/// discovery `crate::start` / `crate::doc` do.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `git_untracked` is the empty-commit guard's untracked signal: a brand-new
    /// file that `git diff <base>` would miss but `git add --all` would commit must
    /// register, so an untracked-only task is not falsely treated as empty.
    #[test]
    fn git_untracked_reports_new_files_and_nothing_when_clean() {
        let dir = std::env::temp_dir().join(format!("jigc-untracked-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mk temp repo");
        let run = |args: &[&str]| {
            Command::new("git")
                .args(args)
                .current_dir(&dir)
                .output()
                .expect("git runs");
        };
        run(&["init", "-q"]);
        // A fresh repo with no files: nothing untracked.
        assert!(git_untracked(&dir).expect("clean").trim().is_empty());
        // A new, unstaged file is untracked.
        std::fs::write(dir.join("new.rs"), "fn main() {}\n").expect("write");
        assert!(
            git_untracked(&dir).expect("dirty").contains("new.rs"),
            "an untracked file is reported"
        );
        // Once tracked (added), it is no longer "untracked".
        run(&["add", "new.rs"]);
        assert!(git_untracked(&dir).expect("staged").trim().is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
