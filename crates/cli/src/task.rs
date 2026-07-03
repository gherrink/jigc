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
use engine::finalize::{Promotion, RepinDecision, decide_base_repin, plan_finalize};
use engine::finding::Finding;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::probe::{ProbeRequest, ProbeRun, ProbeRunStatus};
use engine::schema::Schema;
use engine::state::{self, BasePin, RolesRecord};
use engine::store::canonical_path;
use engine::validate::validate_task;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

/// The validation-blocked exit code: a blocking-`ValidationReport` /
/// `plan_finalize`-findings outcome — distinct from an operational error (1) and
/// clap's usage error (2), so a harness-side tally can discriminate outcomes from
/// the exit code alone (`design/measurement.md` → The capture substrate, item 2:
/// drift-caught and the `validate-blocks` paired count key on it). Setup/ingest/join
/// blocks and git/hook commit rejections are NOT validation outcomes and stay 1.
pub(crate) const EXIT_VALIDATION_BLOCKED: u8 = 3;

/// The exit code a **migration** `finalize` returns when it blocks at the review gate
/// (`design/auto-migration.md` → The review gate): the human has not yet `--approve`d
/// the fidelity diff, so nothing is committed. Distinct from [`EXIT_VALIDATION_BLOCKED`]
/// (3) — the rewrite is structurally conformant; this is a *human-fidelity* hold, not a
/// validation outcome — and from the operational error (1), so a harness-side tally can
/// discriminate a pending review from a real block by the exit code alone
/// (`design/measurement.md` → exit-code hygiene).
pub(crate) const EXIT_REVIEW_PENDING: u8 = 4;

/// The `jigc task <verb>` subcommand tree. Each verb names a task by its `<id>`.
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum TaskCommand {
    /// Enumerate the active tasks (id + minting workflow + intent); no id needed.
    List,
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
        /// Approve a migration task's fidelity diff and proceed through the commit
        /// transaction. Without it, a migration `finalize` renders the diff and blocks
        /// (exit 4, nothing committed). Inert on a non-migration task.
        #[arg(long)]
        approve: bool,
        /// Print the pre-commit manifest (the file-set the commit would carry, untracked
        /// sweeps flagged) and stop — commit nothing, no destructive side effect (B1
        /// dirty-tree sweep). A dry-run never requires `--approve`.
        #[arg(long)]
        dry_run: bool,
    },
    /// Bind an already-committed doc to one of the task's declared context roles,
    /// so `task.<role>` resolves to it on the resume re-compose.
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
            TaskCommand::List => run_list(cwd, format),
            TaskCommand::Diff { id } => run_diff(cwd, &id),
            TaskCommand::Validate { id } => return run_validate(cwd, &id, format),
            TaskCommand::Discard { id } => run_discard(cwd, &id),
            TaskCommand::Finalize {
                id,
                approve,
                dry_run,
            } => {
                return run_finalize(cwd, &id, format, approve, dry_run);
            }
            TaskCommand::Bind { role, addr, id } => run_bind(cwd, &role, &addr, &id),
        };
        match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{}", render::operational_error(format, &err));
                ExitCode::FAILURE
            }
        }
    }
}

/// One active task's at-a-glance row for `jigc task list` — its id plus the cheaply
/// available context recorded in its working area (the minting workflow and the
/// original intent). Serializes to the `--format json` array element.
#[derive(Debug, serde::Serialize)]
pub struct TaskListRow {
    /// The task id (the working-area slug under `.jigc/tasks/`).
    pub id: String,
    /// The minting workflow id (`<task_dir>/workflow`); `None` for a task minted
    /// before that file existed.
    pub workflow: Option<String>,
    /// The original intent the task was minted from (`<task_dir>/intent`); empty when
    /// none was recorded.
    pub intent: String,
}

/// `jigc task list` — enumerate the active tasks so an agent that started one and came
/// back can find its id (M26 post-completion shakedown: there was no in-tool way to
/// discover a live task id). Reads the **same** `state::list_active_task_ids`
/// enumeration the active-task resolution and the ambiguous-task error use — so the
/// three never disagree — and surfaces each task's minting workflow + intent from its
/// working area. Always exit 0; the empty roster is a clean message, not an error.
fn run_list(cwd: &Path, format: Format) -> Result<()> {
    // The `.jigc/` working area binds to jigc_home (the main checkout), so a worktree
    // lists the project's shared task roster (M31 Inc 2 / WF3).
    let jigc_root = crate::start::jigc_home_or_repo(cwd)?.join(".jigc");
    let mut rows = Vec::new();
    for id in state::list_active_task_ids(&jigc_root) {
        let dir = jigc_root.join("tasks").join(&id);
        let workflow = state::read_workflow_id(&dir)
            .with_context(|| format!("could not read the workflow of task `{id}`"))?;
        let intent = state::read_intent(&dir)
            .with_context(|| format!("could not read the intent of task `{id}`"))?;
        rows.push(TaskListRow {
            id,
            workflow,
            intent: intent.trim().to_string(),
        });
    }
    println!("{}", render::task_list(format, &rows));
    Ok(())
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
/// `finalize`: validate previews what finalize blocks on) — a blocking report exits
/// [`EXIT_VALIDATION_BLOCKED`], an operational error 1. The findings render
/// through `crate::render::validation` in the selected format.
fn run_validate(cwd: &Path, id: &str, format: Format) -> ExitCode {
    let task = match TaskArea::resolve(cwd, id) {
        Ok(task) => task,
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            return ExitCode::FAILURE;
        }
    };
    // The post-sweep record is dropped: a standalone `validate` is a pure reader —
    // the durable baseline advances only at a landed `finalize`
    // (`design/reconciliation.md` → Persistence of the shifted baseline).
    match task.validate() {
        Ok((report, _record)) => {
            print!("{}", render::validation(format, &report));
            if format != Format::Json {
                println!();
            }
            if report.has_blocking() {
                ExitCode::from(EXIT_VALIDATION_BLOCKED)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
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
fn run_finalize(cwd: &Path, id: &str, format: Format, approve: bool, dry_run: bool) -> ExitCode {
    let task = match TaskArea::resolve(cwd, id) {
        Ok(task) => task,
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            return ExitCode::FAILURE;
        }
    };
    match task.finalize(id, format, approve, dry_run) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
    }
}

/// The CLI side of the `doc-code` probe seam (`engine::validate::ProbeInvoker`): run the
/// resolved `doc-code` program over the engine-built [`ProbeRequest`] and report the raw
/// [`ProbeRun`] (`design/validation.md` → Architecture — the CLI owns the subprocess
/// invoker; the engine stays shell-free). The engine enumerated the surface, materialized
/// the snapshot, and built the request; this serializes it to the child's stdin, enforces
/// the wall-clock budget via [`::cli::invoke::invoke_probe`], and maps the raw outcome
/// into the engine type the engine ingests.
///
/// A **spawn failure** (the program is missing / not executable) maps to a `crash`
/// candidate ([`ProbeRunStatus::Exited`] with `code: None`) so a misconfigured probe
/// surfaces a blocking `pack-probe-integrity.crash` meta-finding rather than silently
/// passing or aborting the whole validate — an unresolvable invocation is never a clean
/// run. A serialization failure of the engine-built request is the only `Err` raised (an
/// internal fault, not a probe outcome).
pub(crate) fn doc_code_invoker(request: &ProbeRequest) -> std::io::Result<ProbeRun> {
    let bytes = serde_json::to_vec(request)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    let program = ::cli::invoke::doc_code_program();
    match ::cli::invoke::invoke_probe(&program, &bytes, ::cli::invoke::DOC_CODE_BUDGET) {
        Ok(outcome) => Ok(ProbeRun {
            stdout: outcome.stdout,
            status: match outcome.status {
                ::cli::invoke::ProbeStatus::Exited { code } => ProbeRunStatus::Exited { code },
                ::cli::invoke::ProbeStatus::TimedOut => ProbeRunStatus::TimedOut,
            },
        }),
        // The program could not be spawned (absent / not executable) — a crash candidate,
        // not an orchestration error: the engine synthesizes the blocking meta-finding.
        Err(_) => Ok(ProbeRun {
            stdout: Vec::new(),
            status: ProbeRunStatus::Exited { code: None },
        }),
    }
}

/// A named task's working area. `repo_root` is the **worktree** (code, the git index,
/// HEAD — every `git` shell-out routes here); `jigc_root` (and the doc-store base
/// `jigc_home`) bind to **jigc_home**, the main checkout, so all worktrees of one
/// project share a single `.jigc/` (M31 Inc 2 / WF3). Outside a worktree the two
/// coincide.
struct TaskArea {
    repo_root: PathBuf,
    jigc_home: PathBuf,
    jigc_root: PathBuf,
    dir: PathBuf,
    pack: Box<dyn PackSource>,
}

impl TaskArea {
    /// Resolve the task `id`'s working area from `cwd`. The repo root is the nearest
    /// `.git` ancestor (the worktree); jigc_home is the main checkout, and the task dir is
    /// `<jigc_home>/.jigc/tasks/<id>/`. A task that does not exist rejects with the
    /// start-a-task route.
    fn resolve(cwd: &Path, id: &str) -> Result<Self> {
        let repo_root = discover_repo_root(cwd)
            .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
        let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
        let jigc_root = jigc_home.join(".jigc");
        let dir = jigc_root.join("tasks").join(id);
        if !dir.is_dir() {
            bail!("no task `{id}` — start one with `jigc start \"<intent>\"`");
        }
        Ok(Self {
            repo_root,
            jigc_home,
            jigc_root,
            dir,
            pack: make_pack(),
        })
    }

    /// The project cascade layer's config dir (`<repo>/.jigc/config`) — the override
    /// surface `task validate` / `finalize` resolve to feed the engine's severity
    /// post-pass (`design/validation.md` → Every finding-emitting entry point must
    /// resolve the cascade). A missing dir is the no-override case
    /// ([`crate::start::resolve_severity_cascade`] yields the pack-default base).
    fn project_config(&self) -> PathBuf {
        self.jigc_root.join("config")
    }

    /// Resolve the project cascade for the engine's M6 severity post-pass — the
    /// `Resolved` `validate` / `finalize` thread through `ValidationReport::new`, so a
    /// recorded `validation.<probe>.<check>.severity` scalar-set tunes their findings
    /// (`design/validation.md` → Severity assignment — the M6 post-pass). A no-override
    /// project layer resolves to the base scalars → the post-pass is inert (the
    /// no-override path stays byte-identical).
    fn severity_cascade(&self) -> Result<engine::cascade::Resolved> {
        crate::start::resolve_severity_cascade(self.pack.as_ref(), &self.project_config())
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
            let schema = crate::pack::load_pack_schema(self.pack.as_ref(), &bytes)
                .with_context(|| format!("the `{}` schema parses", id.as_str()))?;
            out.insert(schema.ty.clone(), schema);
        }
        // Surface C — the finalize-promote write path: nest every persisted doctype's
        // `location:` under the resolved `docs-root` so the conformance sweep + promotion
        // destination agree with the read surfaces.
        let resolved = self.severity_cascade()?;
        crate::start::apply_docs_root(&resolved, out.values_mut());
        Ok(out)
    }

    /// The task-scope **probe pre-flight** — the task twin of `cli.rs`'s store-scope
    /// `require_doc_code_probe`. When the task's effective state carries `code-anchor`
    /// work (the same surface `schedule_doc_code` invokes the probe over), resolve the
    /// `doc-code` program (the `JIGC_DOC_CODE_PROBE` override else the `<bin-dir>/doc-code`
    /// sibling) and require it to be an existing file **before** the engine sweep reaches
    /// the probe. A missing probe is one misconfiguration to report once — not an
    /// N-per-anchor `pack-probe-integrity.crash` floor — so this bails with **one**
    /// operational error naming the resolved path + the override knob, which the verb
    /// handler routes to stderr with a non-zero exit (`module-layout.md` → Probe
    /// distribution, the task-scope absence fix; `design/validation.md` → Distribution
    /// bound). A task whose effective state carries **no** anchor enumerates an empty
    /// surface, so this is inert — a commit-only no-anchor task is unaffected, matching the
    /// no-anchor sweep, which never invokes the probe.
    fn require_doc_code_probe(&self, schemas: &BTreeMap<String, Schema>) -> Result<()> {
        let (anchors, _guard) =
            engine::target_surface::enumerate_target_surface(&self.dir, &self.jigc_home, schemas)
                .with_context(|| {
                format!(
                    "enumerating the task's code-anchor surface at {:?}",
                    self.dir
                )
            })?;
        if anchors.is_empty() {
            return Ok(());
        }
        let program = ::cli::invoke::doc_code_program();
        if !program.is_file() {
            anyhow::bail!(
                "`doc-code` probe not found at {program:?} — place the `doc-code` binary beside `jigc` or set `JIGC_DOC_CODE_PROBE` to its path"
            );
        }
        Ok(())
    }

    /// Run the task-scope validation sweep against the committed-state `file-state`
    /// record (`design/validation.md` → How it gates `finalize`; `design/reconciliation.md`
    /// → Detection timing: the `task validate` full sweep).
    ///
    /// The record is **loaded** (so drift of a committed doc against its recorded
    /// baseline is detected) and the **post-sweep** record is returned alongside the
    /// report — but never persisted here: the record advances durably only at a
    /// **landed `finalize`** (`design/reconciliation.md` → Persistence of the shifted
    /// baseline, the M17 amendment), whose post-commit threads this record through so
    /// an absorbed OOB baseline stops re-firing in every later task. A standalone
    /// `task validate` drops the record — read verbs stay pure readers. A
    /// working-area instance with no committed baseline baseline-adopts fresh
    /// (advisory) each run rather than drifting against a stale staged hash — clean
    /// validate stays clean as the agent fills it; those `docs/…` staged keys are
    /// per-run and stripped before any persistence.
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
    fn validate(&self) -> Result<(engine::result::ValidationReport, FileStateRecord)> {
        let schemas = self.schemas()?;
        // Pre-flight the `doc-code` probe before the engine sweep reaches it, so a missing
        // probe on an anchored task is one operational error — never an N-per-anchor
        // `pack-probe-integrity.crash` floor (the store-scope `require_doc_code_probe`
        // ported to the task path; `module-layout.md` → Probe distribution).
        self.require_doc_code_probe(&schemas)?;
        let head = git_head(&self.repo_root)?;
        let mut record = FileStateRecord::load(&self.jigc_root).with_context(|| {
            format!(
                "could not load the file-state record under {:?}",
                self.jigc_root
            )
        })?;
        let tracked = self.tracked_predicate()?;
        // Materialize the current git index into a self-cleaning temp tree and resolve
        // cited code anchors against it (M30 Inc 3, G4): the `doc-code` probe validates
        // what *commits*, not the ambient working tree, so a symbol present on disk but
        // left unstaged blocks ("validated reality == committed reality"). This is the
        // single shared `validate` entry (`design/finalize.md` → no private check path),
        // so `task validate` and `finalize` both gate on the index. The index is a pure
        // function of the staged set, so the snapshot stays deterministic.
        let index_tree = self.materialize_index()?;
        // The staged code change-set — the engine's code-anchor blast radius re-resolves
        // every committed anchor whose target file is in this set (the universal `finalize`
        // floor: a rename can't dangle a citation in a committed doc the task never opened,
        // under any workflow). Computed against the same staged index `materialize_index`
        // checks out, so the floor validates exactly the code that commits.
        let changed_code = git_staged_paths(&self.repo_root)?;
        // The HEAD versions of the changed files — the base tree the blast radius's
        // newly-dangled comparison resolves committed anchors against, so a task that merely
        // touches a file already carrying pre-existing committed drift is not wedged on drift
        // it did not cause (only anchors that resolved at HEAD but dangle at the index block).
        let base_tree = self.materialize_head_subset(&changed_code)?;
        let report = validate_task(
            &self.dir,
            &schemas,
            &mut record,
            // The committed doc-store reads bind to jigc_home; the code anchors resolve
            // against the materialized index tree (the worktree's staged set) (M31 Inc 2).
            &self.jigc_home,
            index_tree.path(),
            &self.jigc_root,
            &head,
            &self.severity_cascade()?,
            &doc_code_invoker,
            &tracked,
            &changed_code,
            base_tree.path(),
        )
        .with_context(|| format!("validating task at {:?}", self.dir))?;
        Ok((report, record))
    }

    /// Build the git tracked-status predicate the engine's #5 owner-artifact gate
    /// consults ([`engine::validate::TrackedPredicate`]) — the CLI owns the shell-out, the
    /// engine stays shell-free (`design/methodology-docs.md` → The engine work, item 3).
    ///
    /// A repo-relative path is **durably present** iff git does *not* list it among *all*
    /// untracked files — `git ls-files --others` **without** `--exclude-standard`, via
    /// [`git_untracked_all`]: a present file outside that set is in git's index/HEAD (staged
    /// or committed), the durable-presence the gate requires; a present-but-just-created file
    /// is in the set, so the predicate answers `false` and the gate blocks (the
    /// present-but-untracked case). Dropping `--exclude-standard` is deliberate: with it, a
    /// present-but-**gitignored** file is in neither the index nor the `--others` output, so
    /// it would read as tracked — yet `git add` skips it, so it is *not* durably committable.
    /// Listing ignored files here too catches that gap (a gitignored artifact blocks). The
    /// engine resolves existence itself, so this predicate answers durable-presence only.
    fn tracked_predicate(&self) -> Result<impl Fn(&str) -> bool> {
        let untracked: std::collections::HashSet<String> = git_untracked_all(&self.repo_root)?
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_owned)
            .collect();
        Ok(move |path: &str| !untracked.contains(path))
    }

    /// Materialize the current git **index** into a fresh, self-cleaning temp tree — the
    /// `doc-code` probe's code-resolution root (M30 Inc 3, G4). `git checkout-index -a`
    /// writes every staged blob under `<temp>/`, so a cited symbol present on disk but
    /// absent from the index does not resolve (the about-to-be-committed code is exactly
    /// the staged set). An empty index yields an empty tree (no anchor resolves); the
    /// [`ScratchTree`] removes itself on drop, so neither a clean nor a blocked validate
    /// leaks scratch.
    fn materialize_index(&self) -> Result<ScratchTree> {
        let tree = ScratchTree::new();
        std::fs::create_dir_all(tree.path()).with_context(|| {
            format!("could not create the index scratch tree {:?}", tree.path())
        })?;
        let prefix = format!("--prefix={}/", tree.path().display());
        git_run(&self.repo_root, &["checkout-index", "-a", &prefix])?;
        Ok(tree)
    }

    /// Materialize the **HEAD** versions of the task's `changed` files into a fresh,
    /// self-cleaning temp tree — the `base_code_tree_root` the engine's blast-radius
    /// newly-dangled comparison resolves committed anchors against (so a blast anchor blocks
    /// only when it RESOLVED at HEAD but dangles at the staged index — never on pre-existing
    /// drift in a file the task merely touches). Only the change-set is materialized (a blast
    /// anchor's file is always in it), and only files **present at HEAD**: a file the task
    /// *adds* is absent here, so an anchor into it correctly counts as not-resolved-at-base
    /// (it was already dangling). An empty change-set yields an empty tree (the blast radius
    /// is then inert anyway). The [`ScratchTree`] removes itself on drop.
    fn materialize_head_subset(
        &self,
        changed: &std::collections::BTreeSet<String>,
    ) -> Result<ScratchTree> {
        let tree = ScratchTree::new();
        std::fs::create_dir_all(tree.path())
            .with_context(|| format!("could not create the base scratch tree {:?}", tree.path()))?;
        for path in changed {
            if !path_at_head(&self.repo_root, path) {
                continue; // added-in-task (absent at HEAD) → not resolved at base
            }
            let out = Command::new("git")
                .args(["cat-file", "blob", &format!("HEAD:{path}")])
                .current_dir(&self.repo_root)
                .output()
                .context("could not run `git cat-file` for the base tree")?;
            if !out.status.success() {
                continue; // not a blob at HEAD (e.g. a directory path) — nothing to resolve
            }
            let dest = tree.path().join(path);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            std::fs::write(&dest, &out.stdout)
                .with_context(|| format!("could not write base blob {:?}", dest))?;
        }
        Ok(tree)
    }

    /// Render blocking `findings` through the shared validation funnel and return the
    /// validation-blocked exit (3) — the planner-block surface shared by the phase-1
    /// re-pin decision and [`plan_finalize`].
    ///
    /// The `--format json` report is the primary machine output: it rides **stdout**
    /// regardless of the exit code (so `jigc --format json task finalize > report.json`
    /// captures the findings on a block, not an empty file), symmetric with the landed
    /// path and `task validate`. The exit code (0 vs 3) carries pass/block; the report
    /// carries the findings. The agent-text rendering stays a human-oriented diagnostic
    /// on stderr.
    fn blocked(&self, findings: Vec<Finding>, format: Format) -> Result<ExitCode> {
        let report = engine::result::ValidationReport::new(findings, &self.severity_cascade()?);
        if format == Format::Json {
            print!("{}", render::validation(format, &report));
        } else {
            eprint!("{}", render::validation(format, &report));
            eprintln!();
        }
        Ok(ExitCode::from(EXIT_VALIDATION_BLOCKED))
    }

    /// Execute the `finalize` transaction (`design/finalize.md` → 5–7). Returns the
    /// process exit code: `SUCCESS` on a landed commit — which also emits the
    /// preflight findings envelope on stdout, symmetric with `task validate`
    /// (`design/measurement.md` → The capture substrate, item 2) —
    /// [`EXIT_VALIDATION_BLOCKED`] on a planner block (rendered + surfaced), `FAILURE`
    /// on a hook/git rejection (git's stderr surfaced, no envelope — not a validation
    /// outcome). An orchestration error (git unavailable, malformed pin) bubbles as
    /// `Err`.
    fn finalize(&self, id: &str, format: Format, approve: bool, dry_run: bool) -> Result<ExitCode> {
        let base = self.base()?;
        let head = git_head(&self.repo_root)?;
        let schemas = self.schemas()?;

        // Phase 1 (amended) — the moved-base re-pin decision runs BEFORE anything
        // else reads the pin: `has_diff` must diff against the *effective* base,
        // else a no-work task reads disjoint moved history as its own diff and
        // slips the empty-commit guard. The CLI supplies the git facts; the engine
        // owns the decision (`design/finalize.md` → Parallel hand-editing, the
        // 2026-06-12 phase-1 amendment).
        let base = match decide_base_repin(
            &self.dir,
            &base,
            &head,
            &git_changed_paths(&self.repo_root, &base.sha, &head)?,
            &git_dirty_paths(&self.repo_root)?,
            &schemas,
        ) {
            Ok(RepinDecision::NoDivergence) => base,
            // Disjoint moved history: the effective pin is HEAD — in-memory ONLY.
            // `base.json` is never rewritten: a landed finalize deletes the working
            // area, and a blocked run re-derives the decision next invocation (the
            // inc-2 landed-only discipline).
            Ok(RepinDecision::Repin) => BasePin::new(
                head.clone(),
                git_capture(&self.repo_root, &["rev-parse", "--short", "HEAD"])?,
            ),
            // The moved history overlaps the task's work — the block names the
            // overlapping paths and carries the resolve-or-discard route.
            Err(findings) => return self.blocked(findings, format),
        };

        // The validate report the planner gates on — one engine, two entry points
        // (`design/finalize.md` → 2. Validate: no private check path). The post-sweep
        // record rides along: a *landed* commit persists it (the absorb baseline-advance,
        // `design/reconciliation.md` → Persistence of the shifted baseline); a blocked
        // branch drops it.
        let (report, swept) = self.validate()?;

        // A migration task is identified once by the staged source seam: it selects the
        // stage policy (`MigrationFixed`), gates the review block, and — read here, ahead
        // of the empty-commit signal — keeps the migration `has_diff` on the proven
        // whole-tree probe (its blocks, e.g. the missing-replacement F1 retire-safety
        // gate, must precede the empty-commit guard inside `plan_finalize`).
        let source_seam = self.dir.join(crate::migrate::SOURCE_FILE);
        let is_migration = source_seam.exists();

        // The diff-presence signal the planner's empty-commit guard needs. On the per-task
        // `IndexHonoring` path it is the NARROWED stage set the commit actually lands (M30
        // G2; `design/finalize.md` → Dirty-tree policy) — never the ambient dirty tree: the
        // agent's staged code (`git diff --cached`), any staged doc that will PROMOTE (the
        // transient commit doc never promotes, so a commit-only task that staged no code
        // reads empty), and the git-tracked config layer a first finalize must land;
        // unstaged/untracked WIP is excluded. A migration task keeps the original
        // whole-tree probe (`MigrationFixed` stages its own fixed set regardless).
        let has_diff = if is_migration {
            !git_diff(&self.repo_root, &base.sha)?.trim().is_empty()
                || !self.staged_docs()?.is_empty()
                || !git_untracked(&self.repo_root)?.trim().is_empty()
        } else {
            let staged_code = !git_capture(&self.repo_root, &["diff", "--cached"])?.is_empty();
            let staged_promotable = self.staged_docs()?.iter().any(|(name, _)| {
                name.strip_suffix(".md")
                    .and_then(|stem| stem.split_once(':'))
                    .and_then(|(ty, _)| schemas.get(ty))
                    .is_some_and(|schema| schema.location.is_some())
            });
            let config_pending = !git_capture(
                &self.repo_root,
                &[
                    "status",
                    "--porcelain",
                    "--",
                    ".jigc/config",
                    ".jigc/.gitignore",
                ],
            )?
            .is_empty();
            staged_code || staged_promotable || config_pending
        };

        let commit_schema = schemas
            .get(COMMIT_TYPE)
            .with_context(|| format!("the embedded pack ships no `{COMMIT_TYPE}` schema"))?;

        // The engine plans; the CLI executes. A blocking branch returns the findings.
        let plan = match plan_finalize(
            &self.dir,
            // The committed doc-store base (supersede targets, promotion destinations,
            // retirements) binds to jigc_home (M31 Inc 2 / WF3).
            &self.jigc_home,
            &base,
            &head,
            &report,
            has_diff,
            commit_schema,
            id,
            &schemas,
        ) {
            Ok(plan) => plan,
            // M30 G2 — recolor the engine's clean-empty block as the staged-nothing block
            // when the narrowed set is empty BUT the working tree is dirty: the agent has
            // changes it never `git add`ed, so point at `git add` rather than "produced no
            // diff". CLI-side message selection — the shared engine finding is left
            // untouched (it also serves the whole-tree milestone planner). Validation /
            // forward-ref / base-mismatch blocks keep their precedence (plan_finalize
            // surfaces them ahead of the empty-commit guard, so they fall through here).
            Err(findings)
                if findings.iter().any(|f| f.code == "finalize.empty-commit")
                    && !git_dirty_paths(&self.repo_root)?.is_empty() =>
            {
                return self.blocked(vec![nothing_staged_finding()], format);
            }
            Err(findings) => return self.blocked(findings, format),
        };

        // The migration review gate (`design/auto-migration.md` → The review gate). On a
        // migration task (the source seam is staged, determined above), finalize without
        // `--approve` renders the fidelity diff — the staged source-seam bytes vs each
        // staged canonical doc, both read pre-commit — and blocks (exit 4, committing
        // nothing), because the strict parse guarantees structure, never
        // content-faithfulness; the human is its only check. `--approve` falls through to
        // the transaction. Inert on a non-migration task: no source seam.

        // B1 dirty-tree sweep — `--dry-run` surfaces the commit file-set and stops, with no
        // commit and no destructive side effect. It is placed BEFORE the migration review
        // gate: a dry-run commits nothing, so `--approve` must never be required. The plan
        // above is computed read-only (`plan_finalize` only reads), so deriving the
        // prediction from it is side-effect-free.
        if dry_run {
            let (included, left_out) = self.predict_manifest(&plan, is_migration)?;
            print!(
                "{}",
                render::finalize_manifest(format, &included, &left_out)
            );
            if format != Format::Json {
                println!();
            }
            return Ok(ExitCode::SUCCESS);
        }

        if is_migration && !approve {
            let foreign = std::fs::read_to_string(&source_seam).with_context(|| {
                format!("could not read the staged source seam at {source_seam:?}")
            })?;
            let mut rewrites = Vec::with_capacity(plan.promotions.len());
            for promotion in &plan.promotions {
                let rendered = std::fs::read_to_string(&promotion.source).with_context(|| {
                    format!(
                        "could not read the staged canonical doc at {:?}",
                        promotion.source
                    )
                })?;
                rewrites.push((promotion.destination.clone(), rendered));
            }
            print!(
                "{}",
                render::migration_review(format, id, &foreign, &rewrites)
            );
            if format != Format::Json {
                println!();
            }
            return Ok(ExitCode::from(EXIT_REVIEW_PENDING));
        }

        // Phases 4–7: the shared transactional core — promote + stage + commit +
        // rollback + post-commit. The working area is the cleanup dir removed on a
        // landed commit.
        // M30 G1 — the per-task stage policy: a migration keeps its proven fixed-pathspec
        // stage; every other per-task finalize honors the agent's existing index.
        let stage = if is_migration {
            StagePolicy::MigrationFixed
        } else {
            StagePolicy::IndexHonoring
        };
        match try_execute_finalize_plan(
            &self.repo_root,
            &self.jigc_root,
            &self.dir,
            &plan,
            &self.dir,
            &schemas,
            Some(swept),
            stage,
        )? {
            // T1 captures the aggregate hook output; the per-task relay site (T2) consumes it.
            Ok(hook_output) => {
                // The landed surface: emit the preflight findings envelope on stdout,
                // symmetric with `task validate` and advisories included — absorb
                // evidence observed by the sweep is surfaced, never swallowed
                // (`design/measurement.md` → The capture substrate, item 2). Only a
                // landed commit emits it: a hook/git rejection took the branch below.
                //
                // M26 shakedown — pair the envelope with a success summary confirming
                // what landed (the commit + each promoted doc): a successful finalize was
                // near-silent, leaving a user to run `git log` to tell it worked. The
                // commit is already truth here, so HEAD names the landed hash; the plan
                // names what promoted where.
                // M30 G3 — derive the manifest split from the LANDED commit: the **included**
                // set is the commit's own delta (`git show --name-status HEAD`); the
                // **left-out** residual is the post-commit `git status --porcelain` worktree
                // column (the unstaged/untracked WIP the index commit left behind, symmetric
                // with the dry-run forecast). The committed bytes are the contract, not a
                // reconstruction.
                let promoted_dests: std::collections::HashSet<String> = plan
                    .promotions
                    .iter()
                    .map(|promotion| promotion.destination.clone())
                    .collect();
                let (manifest, left_out) = classify_landed_manifest(
                    git_commit_name_status(&self.repo_root)?,
                    git_status_entries(&self.repo_root)?,
                    &promoted_dests,
                );
                let landed = render::Landed {
                    hash: git_capture(&self.repo_root, &["rev-parse", "--short", "HEAD"])?,
                    subject: git_capture(&self.repo_root, &["log", "-1", "--pretty=format:%s"])?,
                    promoted: plan
                        .promotions
                        .iter()
                        .map(|promotion| promotion.destination.clone())
                        .collect(),
                    files: manifest.len(),
                    manifest,
                    left_out,
                };
                print!("{}", render::finalize_landed(format, &report, &landed));
                if format != Format::Json {
                    println!();
                }
                // T2 — relay any non-blocking hook output the commit produced
                // (`design/finalize.md` → 6. Commit, success-relay).
                relay_hook_output(format, &hook_output);
                Ok(ExitCode::SUCCESS)
            }
            Err(err) => {
                eprintln!("{}", render::operational_error(format, &err));
                Ok(ExitCode::FAILURE)
            }
        }
    }

    /// Predict the pre-commit manifest for `--dry-run` — a side-effect-free forecast of
    /// the file-set the commit would carry (B1 dirty-tree sweep), built from repo state
    /// alone (no stage, no commit). Mirrors the two stage paths:
    ///
    /// - **Non-migration** (`git add --all`): start from the working tree (`git status
    ///   --porcelain --untracked-files=all`), mapping `??`→untracked / modified / deleted;
    ///   then ADD each promotion `destination` as `promoted` (the staged docs live in the
    ///   gitignored working area, absent from `git status`) and each tracked retirement as
    ///   `deleted`. A promotion wins the dedup over any same-path working-tree entry.
    /// - **Migration** (`stage_migration`'s narrowed pathspec): exactly its set — promotions
    ///   (`promoted`), tracked retirements (`deleted`), and the jigc-tracked config layer
    ///   `.jigc/config` / `.jigc/.gitignore` (`modified`). No user WIP.
    ///
    /// Accepted prediction bound: on a first-ever finalize, the transaction's
    /// `ensure_jigc_gitignore` may create `.jigc/.gitignore` that `git add --all` would
    /// then sweep but this prediction won't show (it doesn't exist yet) — setup typically
    /// already writes it, so the gap is rare.
    fn predict_manifest(
        &self,
        plan: &engine::finalize::FinalizePlan,
        is_migration: bool,
    ) -> Result<(Vec<render::ManifestEntry>, Vec<render::ManifestEntry>)> {
        use render::{ManifestEntry, ManifestKind};

        let promoted: Vec<String> = plan
            .promotions
            .iter()
            .map(|promotion| promotion.destination.clone())
            .collect();

        if is_migration {
            let mut entries: Vec<ManifestEntry> = promoted
                .iter()
                .map(|path| ManifestEntry {
                    path: path.clone(),
                    kind: ManifestKind::Promoted,
                })
                .collect();
            for retirement in &plan.retirements {
                if let Some(spec) = retirement.to_str()
                    && path_at_head(&self.repo_root, spec)
                {
                    entries.push(ManifestEntry {
                        path: spec.to_owned(),
                        kind: ManifestKind::Deleted,
                    });
                }
            }
            // The jigc-tracked config layer the narrowed `git add` also stages (B2) — the
            // first migration commit lands it. Tracked config being (re)staged → modified.
            for spec in existing_pathspecs(&self.repo_root, &[".jigc/config", ".jigc/.gitignore"]) {
                entries.push(ManifestEntry {
                    path: spec,
                    kind: ManifestKind::Modified,
                });
            }
            // A migration is `MigrationFixed` — it stages its own narrowed pathspec and
            // never sweeps user WIP, so there is no left-out set.
            return Ok((entries, Vec::new()));
        }

        // Non-migration `IndexHonoring`: the commit lands the INDEX, so split each dirty
        // path by its porcelain column (M30 G3) — the X (index) column is **included** in
        // the commit, the Y (worktree) column is **left out** (unstaged/untracked WIP the
        // agent must `git add` to include). A staged-then-further-modified (`MM`) path is
        // non-blank in both columns, so it appears in **both** sets.
        let promoted_set: std::collections::HashSet<&str> =
            promoted.iter().map(String::as_str).collect();
        let mut included: Vec<ManifestEntry> = Vec::new();
        let mut left_out: Vec<ManifestEntry> = Vec::new();
        for (code, path) in git_status_entries(&self.repo_root)? {
            // A promotion lands at its canonical path and wins the dedup; it is added below.
            if promoted_set.contains(path.as_str()) {
                continue;
            }
            let mut columns = code.chars();
            let x = columns.next().unwrap_or(' ');
            let y = columns.next().unwrap_or(' ');
            // X names the staged change the commit carries. `?` (untracked) is not in the
            // index, so it never counts as included.
            if x != ' ' && x != '?' {
                included.push(ManifestEntry {
                    path: path.clone(),
                    kind: column_kind(x),
                });
            }
            // Y names the un-staged worktree residual left out of the commit.
            if y != ' ' {
                left_out.push(ManifestEntry {
                    path,
                    kind: column_kind(y),
                });
            }
        }
        for path in &promoted {
            included.push(ManifestEntry {
                path: path.clone(),
                kind: ManifestKind::Promoted,
            });
        }
        // Empty on a non-migration task (only a migration populates retirements), but kept
        // for symmetry with the stage path.
        for retirement in &plan.retirements {
            if let Some(spec) = retirement.to_str()
                && path_at_head(&self.repo_root, spec)
            {
                included.push(ManifestEntry {
                    path: spec.to_owned(),
                    kind: ManifestKind::Deleted,
                });
            }
        }
        Ok((included, left_out))
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
            .and_then(|schema| canonical_path(&self.jigc_home, schema, address.slug.as_str()))
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

/// Which working-tree changes the finalize stage commits (`design/finalize.md` →
/// Dirty-tree policy, revised M30; `DECISIONS.md` 2026-06-20 M30 planning, G1). The
/// per-task path narrows to the agent's own index; the migration path keeps its proven
/// fixed-pathspec stage; the `squash: true` milestone boundary folds the isolated
/// worktrees' code off-line ([`StagePolicy::Combine`]). The `squash: false` boundary builds
/// its N+1 commit chain (one per sub-task carrying its own code, then the merged-docs
/// aggregate) in a dedicated worktree and fast-forwards main ([`StagePolicy::ChainPerSubtask`])
/// — WIP-safe like the `squash: true` path, never `git reset --hard`ing the live checkout (the
/// old whole-tree `Sweep` is retired: under worktree isolation it dropped every line of
/// sub-agent code, the data-loss the redesign fixes).
pub(crate) enum StagePolicy {
    /// Migration finalize — stage exactly its own paths ([`stage_migration`]): the
    /// promoted canonical doc(s), each retired original's deletion, and the config layer.
    MigrationFixed,
    /// Per-task non-migration finalize (M30) — honor the agent's existing index, adding
    /// only jigc's promoted-doc destinations + the first-commit config layer
    /// ([`stage_index_honoring`]); unstaged/untracked WIP stays uncommitted.
    IndexHonoring,
    /// The `squash: true` fan-out boundary (M31 Inc 4) — the id-ordered list of the
    /// milestone's still-provisioned worktree paths. The sub-agent code lives in those
    /// isolated worktrees, NOT this checkout, so a `git add --all` sweep would drop it;
    /// instead [`combine_commit`] folds each worktree's staged code-set onto the base tree
    /// **off-line**, overlays the promoted docs + config, and commits the combined tree with
    /// the user's hooks running from a dedicated worktree (M31 Inc 5). An empty list (a
    /// docs-only / never-provisioned milestone) degrades to a docs-only commit, byte-identical
    /// to the M7 single-aggregate form.
    Combine(Vec<PathBuf>),
    /// The `squash: false` fan-out boundary (M31 — WIP-safe rework). The id-ordered
    /// `(staged-patch, rendered-commit-message)` pairs for the code-carrying sub-tasks, plus
    /// the output [`Format`] for the per-commit hook relay. [`chain_commit`] builds the whole
    /// N+1 commit chain (one commit per sub-task carrying THAT sub-task's code with the user's
    /// hooks running + relayed, then the merged-docs aggregate) in a **dedicated worktree**,
    /// then fast-forwards main — the live checkout is never the commit site and is never `git
    /// reset --hard`ed, so an abort (a hook rejection) leaves unrelated main-checkout WIP
    /// intact (review S2; the squash:true [`Combine`](StagePolicy::Combine) WIP-safety, mirrored
    /// onto the honest-rework path). An empty list degrades to a docs-only aggregate.
    ChainPerSubtask {
        subtasks: Vec<(Vec<u8>, String)>,
        format: Format,
    },
}

/// Execute a [`FinalizePlan`]'s commit phases 4–7 (`design/finalize.md` → 4. Promote /
/// 5. Stage / 6. Commit / 7. Post-commit) — the **shared** executor the per-task
/// [`TaskArea::finalize`] and the milestone single-commit boundary
/// (`crate::milestone`) both drive, so the promote/stage/commit/rollback/hash logic is
/// implemented once, never divergently (the inc-4 doc-elaboration pin, `DECISIONS.md`
/// 2026-06-04). The engine planner already decided *what* lands *where*; this owns only
/// the git I/O.
///
/// `msg_tmp_dir` is where the rendered message temp file is written (the task working
/// area / the milestone staging area — both gitignored); `cleanup_dir` is the working
/// area removed on a landed commit (the task dir / the milestone area). The flow: write
/// the message to a temp file, copy each promoted doc to its canonical repo path, ensure
/// `.jigc/.gitignore`, stage per `stage`, commit (**never** `--no-verify`). A hook/git
/// rejection surfaces git's stderr verbatim, rolls back the promoted copies, and lands no
/// commit (exit `FAILURE`). On success, post-commit (best-effort: advance the file-state
/// hashes, invalidate the edge-index stamp, remove the working area).
///
/// `stage` selects the stage/commit mechanism ([`StagePolicy`]). The `squash: true`
/// milestone boundary passes [`StagePolicy::Combine`] (fold the worktree code-sets +
/// commit the combined tree with the user's hooks running); the migration path its fixed
/// pathspec.
// Each argument is a distinct, independent fact (repo/jigc/tmp/cleanup roots, the plan,
// schemas, the output format, the stage policy) threaded straight to the shared executor;
// an allow is clearer here than a parameter struct (matching `try_execute_finalize_plan`).
#[allow(clippy::too_many_arguments)]
pub(crate) fn execute_finalize_plan(
    repo_root: &Path,
    jigc_root: &Path,
    msg_tmp_dir: &Path,
    plan: &engine::finalize::FinalizePlan,
    cleanup_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    format: Format,
    stage: StagePolicy,
) -> Result<ExitCode> {
    // Map the landed/failed `Result` onto the historic `Ok(ExitCode)` contract the
    // per-task and `squash: true` milestone callers expect (a failure surfaces git's
    // stderr verbatim — through the shared operational-error funnel, so `--format
    // json` gets the error envelope — and exits `FAILURE`; success exits `SUCCESS`).
    // The milestone boundary runs no reconcile sweep, so it carries no post-sweep
    // record to persist (`None` — post-commit loads the durable record as before).
    match try_execute_finalize_plan(
        repo_root,
        jigc_root,
        msg_tmp_dir,
        plan,
        cleanup_dir,
        schemas,
        None,
        stage,
    )? {
        // T3 — relay the landed commit's non-blocking hook output (the `squash: true`
        // milestone boundary; the combine commits the combined tree from a dedicated worktree
        // where the user's hooks run, so this stream carries that hook output — M31 Inc 5;
        // `design/finalize.md` → 6. Commit, review B1).
        Ok(hook_output) => {
            relay_hook_output(format, &hook_output);
            Ok(ExitCode::SUCCESS)
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Ok(ExitCode::FAILURE)
        }
    }
}

/// The transactional core of [`execute_finalize_plan`]: promote + stage + commit +
/// post-commit, returning `Ok(Ok(hook_output))` when the aggregate landed and
/// `Ok(Err(_))` when the commit was rejected (the promotions already rolled back). The
/// `hook_output` is the aggregate `git_commit`'s captured non-blocking-hook stream
/// (empty when no hook spoke) — threaded up so the success-relay sites (per-task T2,
/// milestone T3) can surface it to the agent (`design/finalize.md` → 6. Commit). The
/// outer `Result` carries only setup I/O errors (writing the message temp file). The
/// `squash: false` milestone boundary calls this directly with [`StagePolicy::ChainPerSubtask`]
/// — [`chain_commit`] builds the whole N+1 chain in a dedicated worktree and fast-forwards main,
/// so an aborted chain commits nothing to the live checkout (no `git reset --hard`) and the
/// `Ok(Err(_))` carries the original error so the caller can still surface it.
///
/// `post_sweep` is the per-task preflight's post-sweep file-state record — persisted
/// by post-commit **only when the commit lands**, so an absorbed OOB baseline advances
/// durably exactly once (`design/reconciliation.md` → Persistence of the shifted
/// baseline). The milestone callers run no sweep and pass `None`.
///
/// `stage` selects the stage step (M30 G1 — [`StagePolicy`]): a migration stages only its
/// own fixed paths ([`stage_migration`]); a per-task non-migration finalize honors the
/// agent's existing index ([`stage_index_honoring`]); the `squash: true` milestone boundary
/// folds the isolated worktrees off-line ([`combine_commit`]); the `squash: false` boundary
/// builds its N+1 commit chain in a dedicated worktree and fast-forwards main
/// ([`chain_commit`]).
// The shared executor threads many distinct, independent facts (repo/jigc/tmp/cleanup
// roots, the plan, schemas, the post-sweep record, the stage policy); each is a real
// input, not incidental coupling, so an allow is clearer here than a parameter struct.
#[allow(clippy::too_many_arguments)]
pub(crate) fn try_execute_finalize_plan(
    repo_root: &Path,
    jigc_root: &Path,
    msg_tmp_dir: &Path,
    plan: &engine::finalize::FinalizePlan,
    cleanup_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    post_sweep: Option<FileStateRecord>,
    stage: StagePolicy,
) -> Result<Result<String>> {
    let msg_path = msg_tmp_dir.join("finalize-message.tmp");
    std::fs::write(&msg_path, &plan.message)
        .with_context(|| format!("could not write the commit message to {msg_path:?}"))?;
    // The bytes retire deleted, captured pre-deletion so a rollback can rewrite an
    // untracked foreign original `git restore` cannot recover (review F3). Empty unless a
    // migration retire ran.
    let mut retired: Vec<(PathBuf, Vec<u8>)> = Vec::new();
    let commit_result = (|| -> Result<String> {
        // Phase 4 — promote: copy each staged managed doc to `<repo>/<destination>`.
        promote(repo_root, &plan.promotions)?;
        // Retire each foreign original (`design/auto-migration.md` →
        // Retire-the-foreign-original) — the first byte-destructive write, inside the
        // commit closure so `git add --all` stages the deletion into the same commit as
        // the promoted doc. Empty (inert) on every non-migration finalize.
        retired = retire(repo_root, &plan.retirements)?;
        // The transient `.jigc/` subdirs are gitignored via `.jigc/.gitignore` — the
        // working area is never committed (`design/storage.md` → repository layout).
        // Ensure it exists so the `git add --all` stage picks up `config/` + the promoted
        // docs + the code changes.
        ensure_jigc_gitignore(jigc_root)?;
        match stage {
            // Narrowed migration stage (`design/auto-migration.md` → Hardening #9a/#9;
            // `DECISIONS.md` B2): a migration touches no code, so stage ONLY its own
            // changes — the promoted canonical doc(s), each retired original's deletion,
            // and jigc's git-tracked config layer (`.jigc/config/` + `.jigc/.gitignore`,
            // which `setup` writes but never commits, so this first migration commit must
            // land them) — never arbitrary user WIP. The whole-index `git commit -F`
            // (never `-- <pathspec>`) then lands it, so an agent-`git add`ed but
            // non-promoted artifact (the `owner-artifact`) still rides the commit (G6).
            StagePolicy::MigrationFixed => {
                stage_migration(repo_root, plan)?;
                git_commit(repo_root, &msg_path)
            }
            // Per-task IndexHonoring (M30 G6): honor the agent's existing index and add
            // ONLY jigc's promoted docs + the config layer into it; never sweep the
            // ambient dirty tree. The whole-index `git_commit` lands the lot.
            StagePolicy::IndexHonoring => {
                stage_index_honoring(repo_root, plan)?;
                git_commit(repo_root, &msg_path)
            }
            // The `squash: true` fan-out boundary (M31 Inc 4 / Inc 5): fold the N
            // worktree-staged code-sets onto the base tree off-line, overlay the promoted docs
            // + config, and commit the combined tree with the user's hooks running from a
            // dedicated worktree — never `git add --all` (the code lives in the isolated
            // worktrees, not this checkout, so a sweep would drop it).
            StagePolicy::Combine(worktrees) => {
                combine_commit(repo_root, &worktrees, plan, &msg_path)
            }
            // The `squash: false` honest-rework boundary (M31 — WIP-safe): build the N+1
            // commit chain in a dedicated worktree (one commit per sub-task carrying its own
            // code with hooks running + relayed, then the merged-docs aggregate) and
            // fast-forward main — never touching the live checkout, so an abort leaves
            // unrelated WIP intact (no `git reset --hard`). Returns the aggregate's hook
            // output (the success-relay site surfaces it; the per-sub-task outputs are relayed
            // inside).
            StagePolicy::ChainPerSubtask { subtasks, format } => {
                chain_commit(repo_root, &subtasks, plan, &msg_path, format)
            }
        }
    })();
    let _ = std::fs::remove_file(&msg_path);
    let hook_output = match commit_result {
        Ok(hook_output) => hook_output,
        Err(err) => {
            // Roll back phases 4–5 (`design/finalize.md` → Rollback discipline): restore
            // HEAD content for the promoted paths and delete the promoted copies, and
            // restore each retired foreign original (review B1) — so an approved-but-failed
            // commit never leaves the foreign file deleted with no commit; no commit landed.
            rollback_promotions(repo_root, &plan.promotions, &plan.retirements, &retired);
            return Ok(Err(err));
        }
    };
    // Phase 7 — post-commit (best-effort; the commit is already truth).
    post_commit(
        repo_root,
        jigc_root,
        cleanup_dir,
        schemas,
        &plan.hash_updates,
        post_sweep,
    );
    Ok(Ok(hook_output))
}

/// Relay a landed commit's captured non-blocking hook output to the agent
/// (`design/finalize.md` → 6. Commit, success-relay; M19 review S1). git surfaces a
/// *non-blocking* `pre-commit`/`commit-msg` hook's stream on a **successful** commit
/// (the warn-only doc↔code backstop, a linter/formatter that warns-but-exits-0); today
/// finalize swallowed it on success and only surfaced it on rejection, so the warning
/// never reached an agent committing through jigc. This closes that loop.
///
/// **Deliberate, general behavior change** (recorded so it is not a surprise): the
/// relay is *not* jigc-backstop-specific — git exposes one combined hook stream and the
/// CLI cannot single out its own backstop from a user's hook, so **any** non-blocking
/// hook's output now surfaces. Empty output (no hook spoke) emits nothing — no
/// delimiter.
///
/// **Placement discipline (review S1):** a clearly delimited section that *follows* the
/// success result, never inside the routing footer or the structured envelope. On
/// `--format json` the structured envelope owns stdout (an agent parsing it must not
/// have it corrupted), so the relay goes to **stderr**; on agent-text it is a delimited
/// section after `render::validation`'s footer.
pub(crate) fn relay_hook_output(format: Format, hook_output: &str) {
    let hook_output = hook_output.trim();
    if hook_output.is_empty() {
        return;
    }
    // `section` already ends with one `\n`; `print!`/`eprint!` (not the `ln` variants)
    // keep the relayed section's shape clean — no spurious trailing blank line.
    let section = format!("--- hook output ---\n{hook_output}\n");
    if format == Format::Json {
        eprint!("{section}");
    } else {
        print!("{section}");
    }
}

/// Phase 4 (`design/finalize.md` → 4. Promote managed docs). Copy each staged managed
/// doc from its source to its canonical repo path (`<repo_root>/<destination>`) —
/// **copy, not move**, so rollback is a removal of the copies and the working area stays
/// intact. Creates the destination's parent directory (e.g. `decisions/`) when absent.
fn promote(repo_root: &Path, promotions: &[Promotion]) -> Result<()> {
    for promotion in promotions {
        let dest = repo_root.join(&promotion.destination);
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

/// The retire step (`design/auto-migration.md` → Retire-the-foreign-original) — the
/// first byte-destructive write on a repo file. Remove each recorded foreign original
/// (repo-relative) so the subsequent `git add --all` stages the deletion into the same
/// commit as the promoted managed doc. Runs inside the commit closure, so a failure
/// aborts the transaction (rolling back the promotions). An already-absent path is not
/// an error (idempotent — the goal is the file gone). Empty on every non-migration
/// finalize, so this is inert there.
///
/// **Captures the deleted bytes** keyed by repo-relative path (review F3): a foreign
/// original that was **untracked** at HEAD has no committed bytes for `git restore` to
/// recover on a rollback, so the captured bytes are what `rollback_promotions` rewrites
/// to keep an approved-but-failed commit from permanently losing it.
fn retire(repo_root: &Path, retirements: &[PathBuf]) -> Result<Vec<(PathBuf, Vec<u8>)>> {
    let mut captured = Vec::new();
    for retirement in retirements {
        let path = repo_root.join(retirement);
        match std::fs::read(&path) {
            Ok(bytes) => {
                captured.push((retirement.clone(), bytes));
                std::fs::remove_file(&path)
                    .with_context(|| format!("could not retire the foreign original {path:?}"))?;
            }
            // Already absent — idempotent (the goal is the file gone); nothing to capture.
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => {
                return Err(err).with_context(|| {
                    format!("could not read the foreign original {path:?} to retire it")
                });
            }
        }
    }
    Ok(captured)
}

/// The narrowed migration stage (`design/auto-migration.md` → Hardening #9a/#9;
/// `DECISIONS.md` B2). A migration touches no code, so instead of the blanket `git add
/// --all` the non-migration finalize uses, stage exactly the migration's own paths:
///
/// - each promoted canonical doc (`plan.promotions[*].destination`);
/// - each retired foreign original (`plan.retirements[*]`) — already deleted from the
///   worktree by [`retire`], so `git add -- <path>` stages the **removal** (git ≥ 2.0
///   stages deletions for a pathspec);
/// - jigc's git-*tracked* config layer (`.jigc/config/` + `.jigc/.gitignore`) — `jigc
///   setup` writes it but never commits, so the first migration commit is what lands it
///   (review B2); without it a naive {promote + retire} narrowing would strand the setup.
///   Each fixed pathspec is guarded on existence ([`existing_pathspecs`]): a `git add`
///   pathspec that matches no file is fatal (exit 128) and stages **nothing**, so an
///   absent layer would abort the whole byte-destructive migration commit — skipping it
///   keeps the present paths staging while never aborting (Inc 6 advisory / review LOW).
///
/// This deliberately excludes arbitrary user WIP (the whole point of #9a). The general
/// dirty-tree-sweep redesign for non-migration tasks stays deferred.
fn stage_migration(repo_root: &Path, plan: &engine::finalize::FinalizePlan) -> Result<()> {
    let mut pathspecs: Vec<String> = Vec::new();
    for promotion in &plan.promotions {
        pathspecs.push(promotion.destination.clone());
    }
    for retirement in &plan.retirements {
        let spec = retirement
            .to_str()
            .with_context(|| format!("retirement path {retirement:?} is not valid UTF-8"))?;
        // [`retire`] has already deleted the original from the worktree. Stage that
        // deletion only when the file was **tracked** at HEAD — a `git add` pathspec that
        // matches nothing (an untracked-then-deleted foreign original) is a fatal error,
        // whereas the blanket `git add --all` tolerated it. An untracked deletion needs no
        // staging (it was never in the index), so skipping it is correct, not a loss.
        if path_at_head(repo_root, spec) {
            pathspecs.push(spec.to_owned());
        }
    }
    refresh_version_stamp(repo_root)?;
    pathspecs.extend(existing_pathspecs(
        repo_root,
        &[
            ".jigc/config",
            ".jigc/.gitignore",
            crate::setup::VERSION_STAMP_PATH,
        ],
    ));
    let mut args: Vec<&str> = vec!["add", "--"];
    args.extend(pathspecs.iter().map(String::as_str));
    git_run(repo_root, &args)
}

/// Refresh the committed binary-provenance stamp (`.jigc/version`) to the running build —
/// a store-writing op keeps the stamp current (`design/storage.md` → Store provenance:
/// setup writes it, store-writing ops refresh it). A same-build refresh writes identical
/// bytes (so `git add` stages nothing and the commit is unchanged); a newer build lands the
/// bumped stamp alongside the work, exactly as the git-tracked config layer is (re)staged.
fn refresh_version_stamp(repo_root: &Path) -> Result<()> {
    crate::setup::write_version_stamp(repo_root)
        .with_context(|| "refreshing the binary-provenance stamp `.jigc/version`")
}

/// The per-task non-migration stage (M30 G6, `DECISIONS.md` 2026-06-20). Honor the
/// agent's **existing index** (the code it `git add`ed) and add ONLY jigc's own
/// contributions — each promoted canonical doc destination + the git-tracked config
/// layer (`.jigc/config` / `.jigc/.gitignore`, which `setup` writes but never commits, so
/// the first finalize must land them) — INTO that index. The caller's whole-index
/// `git_commit` then lands the lot, so an agent-`git add`ed but non-promoted artifact
/// (the `owner-artifact`) still rides the commit; a curated `git commit -- <pathspec>`
/// would silently drop it. It NEVER sweeps the ambient dirty tree (unstaged/untracked
/// WIP). A subset of [`stage_migration`] — a non-migration task has no retirements — and
/// shares its [`existing_pathspecs`] existence guard so an absent config layer never makes
/// the `git add` fatal (exit 128).
fn stage_index_honoring(repo_root: &Path, plan: &engine::finalize::FinalizePlan) -> Result<()> {
    let mut pathspecs: Vec<String> = Vec::new();
    for promotion in &plan.promotions {
        pathspecs.push(promotion.destination.clone());
    }
    refresh_version_stamp(repo_root)?;
    pathspecs.extend(existing_pathspecs(
        repo_root,
        &[
            ".jigc/config",
            ".jigc/.gitignore",
            crate::setup::VERSION_STAMP_PATH,
        ],
    ));
    // Nothing jigc-owned to add — the agent's existing index stands alone (a `git add --`
    // with no pathspec is an error, so guard it).
    if pathspecs.is_empty() {
        return Ok(());
    }
    let mut args: Vec<&str> = vec!["add", "--"];
    args.extend(pathspecs.iter().map(String::as_str));
    git_run(repo_root, &args)
}

/// The M30 G2 block-on-empty guidance: the working tree is dirty but the narrowed index
/// is empty, so a per-task `IndexHonoring` commit would land nothing the agent staged. A
/// routed blocking finding pointing at `git add` — distinct from the genuinely-clean
/// engine `finalize.empty-commit` ("produced no diff"), selected CLI-side
/// (`design/finalize.md` → Dirty-tree policy, revised M30).
fn nothing_staged_finding() -> Finding {
    Finding::block(
        "finalize.nothing-staged",
        "you staged nothing — the working tree has changes but the index is empty",
        "`git add` your changes, then re-run `jigc task finalize`",
    )
}

/// Keep only the fixed `candidates` (repo-relative) that actually exist on disk under
/// `repo_root`. A `git add -- <pathspec>` that matches no file is fatal (exit 128) and
/// stages **nothing**, so feeding a `git add` an absent fixed pathspec would abort the
/// entire stage; an absent path has nothing to stage anyway, so dropping it is correct,
/// not a loss. Returns the survivors in input order (review LOW / Inc 6 advisory).
fn existing_pathspecs(repo_root: &Path, candidates: &[&str]) -> Vec<String> {
    candidates
        .iter()
        .filter(|spec| repo_root.join(spec).exists())
        .map(|spec| (*spec).to_owned())
        .collect()
}

/// Roll back phase 4–5 on a commit failure (`design/finalize.md` → Rollback discipline /
/// 6. Commit; `design/auto-migration.md` → Retire-the-foreign-original, the rollback).
/// For each promoted path, restore HEAD's content in the index + worktree (undoing the
/// stage) and delete the promoted copy. For each **retired** foreign original (review
/// B1), restore HEAD's content in the index + worktree — the retire deleted a file
/// present at HEAD and `git add --all` staged that deletion, so `git restore --staged
/// --worktree` brings its bytes back, ensuring an approved-but-failed commit never leaves
/// the foreign file deleted with no commit. When the foreign was **untracked** at HEAD
/// (review F3), `git restore` has no committed bytes to recover and is a no-op — so if the
/// path is still absent afterward, rewrite the bytes `retire` captured pre-deletion
/// (`retired`), keyed by repo-relative path, so an untracked foreign is never permanently
/// lost. Best-effort: a failure is logged, never raised — the commit did not land, so the
/// worst case is a stray copy the next `finalize`/`discard` overwrites.
fn rollback_promotions(
    repo_root: &Path,
    promotions: &[Promotion],
    retirements: &[PathBuf],
    retired: &[(PathBuf, Vec<u8>)],
) {
    for promotion in promotions {
        let _ = git_run(
            repo_root,
            &["restore", "--staged", "--worktree", &promotion.destination],
        );
        let dest = repo_root.join(&promotion.destination);
        // `git restore` recreates the path only if it existed at HEAD; a freshly promoted
        // (new) doc has no HEAD content, so remove the copy outright.
        if !path_at_head(repo_root, &promotion.destination) {
            let _ = std::fs::remove_file(&dest);
            // Sweep up any now-empty parent dirs `promote`'s `create_dir_all` opened (e.g.
            // an untracked `docs/decisions/`), up to — but never including — repo_root, so the
            // rollback leaves no empty scratch dir behind ("as-if-finalize-was-never-called").
            // `remove_dir` only succeeds on an EMPTY dir, so a dir holding other ADRs survives.
            let mut parent = dest.parent();
            while let Some(dir) = parent {
                if dir == repo_root || std::fs::remove_dir(dir).is_err() {
                    break;
                }
                parent = dir.parent();
            }
        }
    }
    for retirement in retirements {
        let path = retirement.to_string_lossy();
        let _ = git_run(repo_root, &["restore", "--staged", "--worktree", &path]);
        // `git restore` is a no-op for a foreign that was untracked at HEAD — there are no
        // committed bytes to recover. If it is still gone, rewrite the captured bytes so an
        // untracked foreign is never permanently lost on a rolled-back commit (review F3).
        let abs = repo_root.join(retirement);
        if !abs.exists()
            && let Some((_, bytes)) = retired.iter().find(|(p, _)| p == retirement)
        {
            let _ = std::fs::write(&abs, bytes);
        }
    }
}

/// Ensure `.jigc/.gitignore` ignores the transient subdirs so a `finalize` stage never
/// commits the working area or the rebuildable caches (`design/storage.md` → repository
/// layout: `.jigc/` is one home whose `config/` is committed while `tasks/`/`index/`/
/// `state/`/`milestones/` are gitignored). Idempotent — (re)written only when absent or
/// not already listing `milestones/` (so an adapter-written `.gitignore` predating the
/// milestone area is amended once, matching `crate::milestone::ensure_jigc_gitignore`).
fn ensure_jigc_gitignore(jigc_root: &Path) -> Result<()> {
    const ENTRIES: &str = "tasks/\nindex/\nstate/\nmilestones/\n";
    let path = jigc_root.join(".gitignore");
    let needs_write = match std::fs::read_to_string(&path) {
        Ok(existing) => !existing.lines().any(|l| l.trim() == "milestones/"),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
        Err(err) => return Err(err).with_context(|| format!("could not read {path:?}")),
    };
    if needs_write {
        std::fs::create_dir_all(jigc_root)
            .with_context(|| format!("could not create {jigc_root:?}"))?;
        std::fs::write(&path, ENTRIES).with_context(|| format!("could not write {path:?}"))?;
    }
    Ok(())
}

/// Phase 7 (`design/finalize.md` → 7. Post-commit, best-effort). Three updates, none of
/// which can affect commit truth, all self-healing: advance the `file-state` record for
/// every committed file (the plan's managed-doc hash set plus the committed working-tree
/// files), **invalidate the edge-index stamp** (lifecycle site 5, `design/storage.md` →
/// Edge index lifecycle — remove the persisted index so the next read rebuilds against
/// the new HEAD), then remove the `cleanup_dir` working area. Each step self-heals on
/// failure, so a failure is logged to stderr, never raised (the commit is already truth).
fn post_commit(
    repo_root: &Path,
    jigc_root: &Path,
    cleanup_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    hash_updates: &BTreeMap<String, String>,
    post_sweep: Option<FileStateRecord>,
) {
    if let Err(err) = advance_file_state(repo_root, jigc_root, schemas, hash_updates, post_sweep) {
        eprintln!("note: post-commit file-state update failed (self-heals): {err:#}");
    }
    if let Err(err) = engine::index::invalidate(jigc_root) {
        eprintln!("note: post-commit edge-index invalidation failed (self-heals): {err:#}");
    }
    if let Err(err) = std::fs::remove_dir_all(cleanup_dir) {
        eprintln!("note: post-commit working-area removal failed (self-heals): {err:#}");
    }
}

/// Record the committed working set into the `file-state` record and save it. The plan's
/// managed-doc hash set is empty in the commit-only case; the committed code files (the
/// working-tree changes that just landed) are hashed from `HEAD`'s tree so the next
/// `file-state` probe sees them in-sync.
///
/// When the caller carries a `post_sweep` record (the per-task preflight's post-sweep
/// state), it is the **base** the commit's updates land on, so an absorbed OOB baseline
/// the commit itself never touched persists too — fixing the verified re-fire defect
/// (`design/reconciliation.md` → Persistence of the shifted baseline). Committed-store
/// keys only: the sweep's staged working-area baselines (`docs/<type>:<slug>.md`) are
/// per-run ([`TaskArea::validate`]) and stripped — discriminated by the `:` every
/// staged key carries ([`engine::state`] mints `<type>:<slug>.md`) and no committed
/// path can (committed docs are `<location>/<slug>.md`, slugs `[a-z0-9-]`), so a
/// committed baseline under a schema whose `location:` is itself `docs/` survives and
/// its drift detection stays live. The plan's hash set and the landed commit's
/// re-hash apply on top, winning on overlap. `None` (the milestone boundary, no sweep)
/// loads the durable record as before.
fn advance_file_state(
    repo_root: &Path,
    jigc_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    hash_updates: &BTreeMap<String, String>,
    post_sweep: Option<FileStateRecord>,
) -> Result<()> {
    let mut record = match post_sweep {
        Some(mut swept) => {
            swept
                .hashes
                .retain(|path, _| !(path.starts_with("docs/") && path.contains(':')));
            swept
        }
        None => FileStateRecord::load(jigc_root)
            .with_context(|| format!("loading the file-state record under {jigc_root:?}"))?,
    };
    for (path, hash) in hash_updates {
        record.record(path.clone(), hash.clone());
    }
    // Hash the files the just-landed commit touched, reading their committed bytes — but
    // gate each through the G4 baseline-adopt check (`engine::file_state`): a foreign
    // non-conformant `.md` that `git add --all` swept into the aggregate commit (a freeform
    // file dropped into a `location:` dir) is **not** recorded, so it stays `UNKNOWN` and
    // the store-sweep advisory re-fires every finalize until the human resolves it
    // (`design/project-setup.md` → Flow 2 hardening — the routed-but-not-recorded
    // recurrence the post-commit path must not defeat). Non-managed paths (code, configs)
    // are always recorded.
    for path in git_commit_files(repo_root)? {
        if let Ok(bytes) = git_show_file(repo_root, &path)
            && file_state::committed_path_recordable(schemas, &path, &bytes)
        {
            record.record(path, file_state::hash_bytes(&bytes));
        }
    }
    record
        .save(jigc_root)
        .with_context(|| format!("saving the file-state record under {jigc_root:?}"))?;
    Ok(())
}

/// The repo-relative paths in the **staged** set — the task's code change-set the engine's
/// code-anchor blast radius scopes its committed-anchor re-resolution to
/// (`design/validation.md` → Scope = effective state, the universal `finalize` floor).
/// Mirrors what [`Task::materialize_index`] checks out, so "the files the floor re-validates"
/// and "the code that commits" are the same staged set. The CLI owns the shell-out; the
/// engine stays shell-free. `--no-renames` so a rename reports both its old and new path (the
/// old path is exactly the file a committed anchor may now dangle against). **`-z`**
/// (NUL-separated) so a path containing a space, a leading/trailing space, or a newline
/// survives verbatim — without it git quotes such paths and a line/`trim()` split would
/// corrupt the change-set (so the blast radius would scope against the wrong file).
pub(crate) fn git_staged_paths(repo_root: &Path) -> Result<std::collections::BTreeSet<String>> {
    let out = Command::new("git")
        .args(["diff", "--cached", "--no-renames", "--name-only", "-z"])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git diff --cached --name-only -z` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    // `-z` terminates each entry with a NUL (the last one too), so the trailing split yields
    // an empty string — filtered out. No `trim()`: a path's own leading/trailing whitespace
    // is significant and must survive.
    Ok(String::from_utf8(out.stdout)
        .context("`git diff --cached` produced non-UTF-8 output")?
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Run `git diff <base>` in `repo_root`, returning the unified diff of the working
/// tree against the pinned base commit (`storage.md` → CLI and git: "CLI
/// orchestrates, git executes"). Shells out to the user's `git`
/// (`DECISIONS.md` 2026-05-31 → Git invocation).
pub(crate) fn git_diff(repo_root: &Path, base_sha: &str) -> Result<String> {
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

/// List the repo-relative paths changed in commits between `base_sha` and `head_sha`
/// (`git diff --no-renames --name-only <base> <head>`) — the moved-history fact the
/// phase-1 re-pin decision ([`decide_base_repin`]) intersects with the task's footprint
/// (`design/finalize.md` → Parallel hand-editing, the 2026-06-12 amendment).
/// `--no-renames`: under rename detection a renamed file reports only its NEW name, so
/// the deleted OLD path would never enter the changed set and a rename of a footprint
/// path would auto-re-pin instead of blocking; it also pins the decision against the
/// user's `diff.renames` config (Validation hardening #7 — a pure function of repo state).
fn git_changed_paths(repo_root: &Path, base_sha: &str, head_sha: &str) -> Result<Vec<String>> {
    let out = git_capture(
        repo_root,
        &["diff", "--no-renames", "--name-only", base_sha, head_sha],
    )?;
    Ok(out
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect())
}

/// List the repo-relative dirty working-tree paths — `git status --porcelain
/// --untracked-files=all`, untracked included (the finalize stage is `git add --all`,
/// which commits them, so they are part of the task's footprint; `=all` lists files
/// inside untracked directories individually, else a collapsed `dir/` entry could
/// never match a changed file path and an overlap would slip). A rename line names
/// both sides; both count.
fn git_dirty_paths(repo_root: &Path) -> Result<Vec<String>> {
    let out = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git status --porcelain` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let text = String::from_utf8(out.stdout).context("`git status` produced non-UTF-8 output")?;
    let mut paths = Vec::new();
    for line in text.lines() {
        // Porcelain v1: two status columns + a space, then the path; a rename
        // reads `R  old -> new`.
        let Some(path) = line.get(3..) else { continue };
        match path.split_once(" -> ") {
            Some((old, new)) => {
                paths.push(old.to_string());
                paths.push(new.to_string());
            }
            None => paths.push(path.to_string()),
        }
    }
    Ok(paths)
}

/// The status-preserving sibling of [`git_dirty_paths`] (B1 dirty-tree sweep): each dirty
/// working-tree path paired with its **two-column** porcelain status code (`git status
/// --porcelain --untracked-files=all`). The full XY is preserved (never collapsed) so the
/// `--dry-run` manifest prediction can split the X (index → included) and Y (worktree →
/// left-out) columns independently (M30 G3). A rename `R old -> new` splits to `old`
/// (`D `, staged delete) + `new` (`A `, staged add) — the shape the staged index carries.
pub(crate) fn git_status_entries(repo_root: &Path) -> Result<Vec<(String, String)>> {
    let out = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git status --porcelain` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let text = String::from_utf8(out.stdout).context("`git status` produced non-UTF-8 output")?;
    let mut entries = Vec::new();
    for line in text.lines() {
        // Porcelain v1: two status columns + a space, then the path.
        let Some(code) = line.get(..2) else { continue };
        let Some(path) = line.get(3..) else { continue };
        match path.split_once(" -> ") {
            Some((old, new)) => {
                entries.push(("D ".to_string(), old.to_string()));
                entries.push(("A ".to_string(), new.to_string()));
            }
            None => entries.push((code.to_string(), path.to_string())),
        }
    }
    Ok(entries)
}

/// Map a single porcelain status-column char to its manifest kind (M30 G3 — each column
/// classified independently so the dry-run forecast can split included/left-out): `D`→
/// deleted, `A`→added (a deliberately-staged new file; only ever the X/index column, so
/// always an *included* member), `?`→untracked (an untracked file; only the worktree side,
/// so always *left-out*), everything else (`M`/`T`/`C`/…)→modified.
fn column_kind(c: char) -> render::ManifestKind {
    match c {
        'D' => render::ManifestKind::Deleted,
        'A' => render::ManifestKind::Added,
        '?' => render::ManifestKind::Untracked,
        _ => render::ManifestKind::Modified,
    }
}

/// List untracked, non-ignored files via `git ls-files --others --exclude-standard`.
/// `git diff <base>` never reports these, but the finalize stage (`git add --all`)
/// commits them — so the empty-commit guard counts them as a diff signal.
pub(crate) fn git_untracked(repo_root: &Path) -> Result<String> {
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

/// List **all** untracked files — including gitignored ones — via
/// `git ls-files --others` (no `--exclude-standard`). A path absent from this set is in
/// git's index/HEAD (durably present); a present path *in* it is untracked or gitignored,
/// i.e. not durably committable. The owner-artifact #5 gate's durable-presence predicate
/// consults this so a present-but-gitignored artifact (which `git add` would skip) blocks
/// rather than passing as falsely "tracked".
pub(crate) fn git_untracked_all(repo_root: &Path) -> Result<String> {
    let out = Command::new("git")
        .args(["ls-files", "--others"])
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

/// The canonical git empty-tree SHA — the sentinel base a **zero-commit** (unborn
/// HEAD) repo pins to so every `creates-task` workflow runs pre-first-commit and the
/// first finalize diffs against the empty tree (`design/project-setup.md` → Flow 2
/// hardening — zero-commit; `implementation/roadmap.md` → M21 Increment 2). `git diff
/// <empty-tree>` emits the add-everything diff and the first commit lands cleanly.
pub(crate) const EMPTY_TREE_SHA: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

/// The short form of [`EMPTY_TREE_SHA`] (for the [`crate::state::BasePin`]'s
/// human-facing divergence messages).
pub(crate) const EMPTY_TREE_SHORT: &str = "4b825dc";

/// Whether `repo_root`'s HEAD is **unborn** — a real repo with no commits yet.
/// `git rev-parse --verify -q HEAD` exits **1** for an unborn HEAD but **128** for a
/// genuinely broken/missing git, so the zero-commit sentinel fires for the former
/// while a real git failure still bails (`design/project-setup.md` → Flow 2 hardening
/// — zero-commit). Shells out to the user's `git` (`DECISIONS.md` 2026-05-31).
pub(crate) fn head_is_unborn(repo_root: &Path) -> Result<bool> {
    let out = Command::new("git")
        .args(["rev-parse", "--verify", "-q", "HEAD"])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    match out.status.code() {
        // HEAD resolves: a real commit — not unborn.
        Some(0) => Ok(false),
        // Unborn HEAD: a real repo with no commits.
        Some(1) => Ok(true),
        // 128 (broken/missing git) or any other code: a genuine failure — bail.
        _ => bail!(
            "`git rev-parse --verify -q HEAD` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ),
    }
}

/// Read HEAD's full SHA via `git rev-parse HEAD` (the supplied HEAD the planner
/// checks the base pin against — `design/finalize.md` → 1. Preflight). Shells out to
/// the user's `git` (`DECISIONS.md` 2026-05-31 → Git invocation).
///
/// On a **zero-commit** repo (unborn HEAD) returns the [`EMPTY_TREE_SHA`] sentinel so
/// the first finalize diffs against the empty tree and the pin guard compares
/// sentinel-vs-sentinel (`design/project-setup.md` → Flow 2 hardening — zero-commit).
pub(crate) fn git_head(repo_root: &Path) -> Result<String> {
    if head_is_unborn(repo_root)? {
        return Ok(EMPTY_TREE_SHA.to_string());
    }
    git_capture(repo_root, &["rev-parse", "HEAD"])
}

/// Run `git <args>` in `repo_root` for its side effect (e.g. `add`), bailing with
/// git's stderr on a non-zero exit. Shells out to the user's `git`.
pub(crate) fn git_run(repo_root: &Path, args: &[&str]) -> Result<()> {
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
///
/// On a **successful** commit returns the captured hook output so the relay sites can
/// surface a non-blocking hook's warning to the agent — e.g. the M19 doc↔code backstop,
/// which warns but exits 0 (`design/finalize.md` → 6. Commit, success-relay). git
/// redirects a hook's own stdout to stderr and writes only its own commit summary
/// ("[branch sha] message", file stats) to stdout, so **stderr is the hook stream**:
/// capturing it is general (any non-blocking hook, not just jigc's backstop) and a
/// no-hook commit yields empty. The bytes are merely captured here — placement/printing
/// is the relay sites' job (M19 increment 2, T2/T3).
pub(crate) fn git_commit(repo_root: &Path, message_file: &Path) -> Result<String> {
    let out = Command::new("git")
        .arg("commit")
        .arg("-F")
        .arg(message_file)
        .current_dir(repo_root)
        .output()
        .context("could not run `git commit` (is git on PATH?)")?;
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() {
        let stdout = String::from_utf8_lossy(&out.stdout);
        bail!(
            "`git commit` was rejected (no commit was made):\n{}{}",
            stdout.trim(),
            stderr.trim()
        );
    }
    Ok(stderr.trim_end().to_owned())
}

/// Apply a worktree's staged `patch` (`git diff --cached --binary`) onto `repo_root`'s index
/// **and** working tree — `git apply --index --whitespace=nowarn`, feeding the patch on stdin
/// (the `squash: false` honest-rework, M31; [`chain_commit`] runs it in the dedicated worktree,
/// never the live checkout). `--index` updates both so the working tree stays consistent with
/// the commit that follows (a `--cached`/index-only apply would leave the code missing from the
/// tree, dirtying the checkout); `--binary` produced the patch so binary blobs apply. A disjoint
/// patch (the up-front collision block guarantees disjointness) applies cleanly atop the prior
/// per-sub-task commits. Bails with git's stderr on a non-zero exit.
fn git_apply_index(repo_root: &Path, patch: &[u8]) -> Result<()> {
    let mut child = Command::new("git")
        .args(["apply", "--index", "--whitespace=nowarn"])
        .current_dir(repo_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("could not run `git apply` (is git on PATH?)")?;
    child
        .stdin
        .take()
        .expect("stdin was piped")
        .write_all(patch)
        .context("could not write the worktree patch to `git apply`")?;
    let out = child
        .wait_with_output()
        .context("wait for `git apply` to finish")?;
    if !out.status.success() {
        bail!(
            "`git apply` of a sub-task's worktree code failed (no commit was made): {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// The `squash: true` fan-out commit (M31 Inc 4 T2 / Inc 5 hook restoration): fold the N
/// worktree-staged code-sets onto the base tree off-line via the [combine engine](crate::combine),
/// overlay the promoted docs + jigc's git-tracked config layer into a throwaway index, then
/// commit the combined tree **with the user's hooks running** from a dedicated detached
/// worktree — never `git add --all` (the sub-agent code lives in the isolated worktrees, not
/// this checkout, so a sweep would drop it, the data-loss the redesign fixes). An empty
/// `worktrees` list (a docs-only / never-provisioned milestone) folds nothing and degrades to
/// a docs-only commit, byte-identical to the M7 single-aggregate form.
///
/// **WIP-safe (review S2):** the off-line tree build (a throwaway index, `GIT_INDEX_FILE`) and
/// the hook-running commit both happen **away from the main checkout** — the commit lands in a
/// clean dedicated worktree ([`commit_combined_tree_with_hooks`]), so the live index/worktree
/// are untouched until the final **non-destructive** fast-forward (`git merge --ff-only`,
/// never `git reset --hard` — which would wipe unrelated unstaged WIP). A blocked combine **or
/// a hook rejection** lands nothing on main and leaves unrelated main-checkout WIP intact; even
/// a successful land carries unrelated WIP (and refuses, never destroys, on a true collision).
///
/// A cross-worktree code collision blocks with a routed [`Finding`] (the combine never
/// text-merges code) surfaced as an `Err` — the same shape a same-doc join clash takes;
/// the caller's rollback restores the promoted docs and lands no commit.
///
/// Returns the dedicated-worktree commit's captured non-blocking hook stream (empty when no
/// hook spoke) so the caller's success-relay surfaces it (`design/finalize.md` → 6. Commit;
/// `DECISIONS.md` 2026-06-21 — the never-bypass-hooks contract on both fan-out commit paths).
fn combine_commit(
    repo_root: &Path,
    worktrees: &[PathBuf],
    plan: &engine::finalize::FinalizePlan,
    msg_path: &Path,
) -> Result<String> {
    // base == HEAD (the milestone preflight guaranteed it before we got here), so the
    // fan-out's pin tree is this checkout's HEAD tree.
    let code_tree = match crate::combine::combine_worktree_trees(repo_root, "HEAD", worktrees)? {
        crate::combine::CombineOutcome::Combined(tree) => tree,
        crate::combine::CombineOutcome::Blocked(finding) => return Err(finding_to_err(finding)),
    };
    // Overlay the promoted docs + config onto the combined code tree and commit it (with the
    // user's hooks) as a child of HEAD, then fast-forward main — the shared WIP-safe aggregate.
    let head = git_head(repo_root)?;
    overlay_docs_commit_and_ff(repo_root, &code_tree, &head, plan, msg_path)
}

/// Build the merged-docs aggregate commit **off the live checkout** and fast-forward main —
/// the WIP-safe commit-and-land both fan-out modes share (the `squash: true`
/// [`combine_commit`] and the `squash: false` [`chain_commit`]).
///
/// Overlay the promoted docs + the git-tracked config layer onto `base_treeish` in a throwaway
/// index (`GIT_INDEX_FILE` — the live index is never touched; the promote step already copied
/// each doc into this checkout's working tree, so a targeted `git add` stages exactly jigc's
/// own contributions, never the ambient dirty tree), write the tree, then commit it as a child
/// of `parent` with the user's hooks running from a clean **dedicated worktree**
/// ([`commit_combined_tree_with_hooks`] — the main checkout is never the commit site).
///
/// Land it on main with a **non-destructive** fast-forward, NOT `git reset --hard` (the M30
/// hazard the redesign rejects — a `reset --hard` resets the live working tree to the commit,
/// silently wiping any unrelated unstaged WIP a human is editing; `design/finalize.md` →
/// `fan-out` finalize, `design/worked-examples.md` flow 33). First stage jigc's own
/// contributions (the promoted docs + first-commit config, already in the working tree from
/// `promote`) into the LIVE index — the same `pathspecs` overlaid above — so the fast-forward
/// sees them as matching the target, not as untracked/dirty paths it would refuse to overwrite;
/// then `git merge --ff-only` advances the ref + brings the fan-out code into the checkout while
/// CARRYING unrelated WIP. If WIP genuinely collides with a committed path, `--ff-only` refuses
/// (surfaced as an `Err` → rollback) rather than silently discarding it — the contract is
/// carry-or-refuse, never destroy (review S2). Returns the dedicated-worktree commit's captured
/// non-blocking hook stream (empty when no hook spoke) for the caller's success-relay.
fn overlay_docs_commit_and_ff(
    repo_root: &Path,
    base_treeish: &str,
    parent: &str,
    plan: &engine::finalize::FinalizePlan,
    msg_path: &Path,
) -> Result<String> {
    let index = CombineIndex::new();
    git_index(repo_root, index.path(), &["read-tree", base_treeish])?;
    let mut pathspecs: Vec<String> = plan
        .promotions
        .iter()
        .map(|promotion| promotion.destination.clone())
        .collect();
    pathspecs.extend(existing_pathspecs(
        repo_root,
        &[".jigc/config", ".jigc/.gitignore"],
    ));
    if !pathspecs.is_empty() {
        let mut args: Vec<&str> = vec!["add", "--"];
        args.extend(pathspecs.iter().map(String::as_str));
        git_index(repo_root, index.path(), &args)?;
    }
    let tree = git_index(repo_root, index.path(), &["write-tree"])?;
    let (commit, hook_output) =
        commit_combined_tree_with_hooks(repo_root, &tree, parent, msg_path)?;
    if !pathspecs.is_empty() {
        let mut args: Vec<&str> = vec!["add", "--"];
        args.extend(pathspecs.iter().map(String::as_str));
        git_run(repo_root, &args)?;
    }
    git_run(repo_root, &["merge", "--ff-only", &commit])?;
    Ok(hook_output)
}

/// The `squash: false` honest-rework fan-out commit (M31 — WIP-safe rework): build the whole
/// N+1 commit chain **away from the live checkout** and fast-forward main, so an abort never
/// touches main (no `git reset --hard`) and unrelated main-checkout WIP survives (review S2;
/// the squash:true [`combine_commit`] WIP-safety, mirrored onto the per-sub-task path that the
/// pre-M31 code reintroduced the hazard on).
///
/// `subtasks` is the id-ordered `(staged-patch, rendered-commit-message)` list for the
/// code-carrying sub-tasks (the caller dropped the empty ones). In a **dedicated detached
/// worktree** at HEAD (a linked worktree shares `.git`, so the user's hooks fire), apply each
/// sub-task's staged patch onto the worktree's index + tree and `git commit -F` its rendered
/// `commit:<sub-id>` doc — so each per-sub-task commit carries THAT sub-task's code (a real
/// tree, never the retired `--allow-empty` form), the user's hooks run, and each commit's
/// non-blocking hook output is relayed. The disjoint patches (the caller's up-front collision
/// block guarantees disjointness) apply cleanly in sequence.
///
/// Then overlay the merged docs + config onto the last sub-task commit's tree and commit the
/// aggregate (hooks running) as its child, and fast-forward main onto the whole chain — the
/// shared [`overlay_docs_commit_and_ff`]. The dedicated worktree is torn down on drop; on ANY
/// abort (a per-sub-task or the aggregate hook rejection) it returns `Err` having committed
/// NOTHING to main — HEAD stays at the pre-finalize sha and unrelated WIP is intact. Returns
/// the aggregate's captured non-blocking hook stream for the caller's success-relay.
fn chain_commit(
    repo_root: &Path,
    subtasks: &[(Vec<u8>, String)],
    plan: &engine::finalize::FinalizePlan,
    msg_path: &Path,
    format: Format,
) -> Result<String> {
    let head = git_head(repo_root)?;
    // The per-sub-task commits accrue in a dedicated worktree at HEAD — never the live
    // checkout, so an abort leaves main untouched (no `git reset --hard`).
    let dedicated = DedicatedWorktree::add(repo_root, &head)?;
    let wt = dedicated.path();
    let sub_msg = wt.join(".jigc-subtask-message.tmp");
    for (patch, message) in subtasks {
        git_apply_index(wt, patch)?;
        std::fs::write(&sub_msg, message).with_context(|| {
            format!("could not write the sub-task commit message to {sub_msg:?}")
        })?;
        let commit_result = git_commit(wt, &sub_msg);
        let _ = std::fs::remove_file(&sub_msg);
        // Relay each per-sub-task commit's non-blocking hook output (M31 Inc 5 — every fan-out
        // commit runs the user's hooks; `design/finalize.md` → 6. Commit).
        relay_hook_output(format, &commit_result?);
    }
    // The aggregate carries the merged docs (a milestone-level merge artifact, review B1/B2),
    // built off the last sub-task commit's tree and fast-forwarded onto main with the chain.
    let subtask_head = git_head(wt)?;
    overlay_docs_commit_and_ff(repo_root, &subtask_head, &subtask_head, plan, msg_path)
}

/// Commit the off-line-built combined `tree` as a child of `parent`, **running the repo's
/// shared `pre-commit`/`commit-msg` hooks against it**, without touching the main checkout
/// (M31 Inc 5 — the squash:true hook restoration; `design/finalize.md` → never bypass hooks).
///
/// `git commit-tree` (the Inc 4 mechanism this replaces) writes a commit object directly and
/// runs **no** hook — bypassing the never-`--no-verify` contract. Instead this adds a clean
/// **dedicated detached worktree** at `parent` (a linked worktree shares the main `.git`, so
/// the user's hooks fire), resets its index + working tree to the combined `tree` (`read-tree
/// --reset -u` — HEAD stays at `parent`, so the commit's parent is `parent` and its tree is
/// the combined tree), and runs the shared [`git_commit`] (`git commit -F`, never
/// `--no-verify`) there. A hook rejection surfaces git's stderr verbatim and lands nothing
/// (the dedicated worktree is the only site mutated — the main checkout is untouched). Returns
/// the new commit sha + the captured non-blocking hook stream. The dedicated worktree is torn
/// down on drop regardless of outcome.
fn commit_combined_tree_with_hooks(
    repo_root: &Path,
    tree: &str,
    parent: &str,
    msg_path: &Path,
) -> Result<(String, String)> {
    let dedicated = DedicatedWorktree::add(repo_root, parent)?;
    let wt = dedicated.path();
    // Set the dedicated worktree's index + working tree to the combined tree while leaving
    // its detached HEAD at `parent` — so `git commit` records `parent` as the parent and the
    // combined tree as the commit's tree. `--reset -u` forces both (the worktree was freshly
    // checked out at `parent` and has no local changes to preserve).
    git_run(wt, &["read-tree", "--reset", "-u", tree])?;
    let hook_output = git_commit(wt, msg_path)?;
    let commit = git_head(wt)?;
    Ok((commit, hook_output))
}

/// A throwaway **detached** git worktree for the squash:true hook-running combine commit,
/// removed on drop (`git worktree remove --force` + `prune`). A linked worktree shares the
/// main repo's `.git` (object DB + hooks), so a commit made here runs the user's shared
/// `pre-commit`/`commit-msg` hooks; committing here instead of the main checkout keeps the
/// combine WIP-safe (the main index/worktree are never the commit site). Lives under the
/// gitignored `.jigc/worktrees/` parent so a leaked dir never pollutes `git status`.
struct DedicatedWorktree {
    repo_root: PathBuf,
    path: PathBuf,
}

impl DedicatedWorktree {
    /// Add a detached worktree at `base` under `.jigc/worktrees/.combine-<pid>-<nanos>`,
    /// clearing any stale leftover dir + pruning admin records first (a crashed prior run).
    fn add(repo_root: &Path, base: &str) -> Result<Self> {
        let name = format!(
            ".combine-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        );
        let path = repo_root.join(".jigc").join("worktrees").join(name);
        std::fs::create_dir_all(path.parent().expect("worktree path has a parent"))
            .with_context(|| format!("could not create the worktrees parent for {path:?}"))?;
        // A stale leftover dir / admin record from a crashed run would make `add` fail.
        let _ = std::fs::remove_dir_all(&path);
        git_run(repo_root, &["worktree", "prune"])?;
        let path_str = path
            .to_str()
            .with_context(|| format!("worktree path {path:?} is not valid UTF-8"))?;
        git_run(repo_root, &["worktree", "add", "--detach", path_str, base])?;
        Ok(DedicatedWorktree {
            repo_root: repo_root.to_path_buf(),
            path,
        })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for DedicatedWorktree {
    fn drop(&mut self) {
        if let Some(path_str) = self.path.to_str() {
            let _ = git_run(
                &self.repo_root,
                &["worktree", "remove", "--force", path_str],
            );
        }
        let _ = git_run(&self.repo_root, &["worktree", "prune"]);
    }
}

/// Run `git <args>` in `repo_root` with `GIT_INDEX_FILE` redirected to `index` — the
/// off-line combine overlay's throwaway index, so the live index/worktree are never
/// touched. Returns trimmed stdout (the tree sha for `write-tree`, empty for
/// `read-tree`/`add`); bails with git's stderr on a non-zero exit.
fn git_index(repo_root: &Path, index: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .env("GIT_INDEX_FILE", index)
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

/// A throwaway index file for the off-line combine overlay, removed on drop — never the
/// live `.git/index`, so the overlay cannot mutate this checkout's staging state (the
/// `crate::combine::TempIndex` idiom, kept private to its own off-line build).
struct CombineIndex(PathBuf);

impl CombineIndex {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "jigc-combine-overlay-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        ));
        // `git read-tree` creates the file; clear any stale leftover first.
        let _ = std::fs::remove_file(&path);
        CombineIndex(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for CombineIndex {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
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

/// The just-landed `HEAD` commit's name-status delta (`git show --name-status --no-renames
/// HEAD`) as `(status-char, path)` pairs — `A`/`M`/`D` (renames split to `A`+`D` under
/// `--no-renames`). Feeds the landed pre-commit manifest (B1 dirty-tree sweep).
fn git_commit_name_status(repo_root: &Path) -> Result<Vec<(char, String)>> {
    let out = git_capture(
        repo_root,
        &["show", "--name-status", "--format=", "--no-renames", "HEAD"],
    )?;
    let mut entries = Vec::new();
    for line in out.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Tab-separated: `<status>\t<path>`.
        let Some((status, path)) = line.split_once('\t') else {
            continue;
        };
        let code = status.trim().chars().next().unwrap_or('M');
        entries.push((code, path.trim().to_owned()));
    }
    Ok(entries)
}

/// Classify the landed commit into the manifest split (M30 G3 — dry-run/landed symmetry):
///
/// - **included** — the commit's own delta (`name_status` from `git show --name-status HEAD`):
///   a path in `promoted` is a `Promoted` managed doc; otherwise the status char maps
///   `A`→added (a deliberately-staged new file), `M`→modified, `D`→deleted, any other
///   (`C`/`T`/…)→modified.
/// - **left_out** — the post-commit `git status --porcelain` worktree residual (`porcelain`):
///   the index commit landed the staged set, so each entry's **Y (worktree) column** names the
///   unstaged/untracked WIP it left behind (the agent `git add`s to include it), classified by
///   [`column_kind`]. A staged-then-further-modified (`MM`) path commits its staged side into
///   `included` and shows its worktree residual here, so it appears in **both**.
fn classify_landed_manifest(
    name_status: Vec<(char, String)>,
    porcelain: Vec<(String, String)>,
    promoted: &std::collections::HashSet<String>,
) -> (Vec<render::ManifestEntry>, Vec<render::ManifestEntry>) {
    let included = name_status
        .into_iter()
        .map(|(code, path)| {
            let kind = if promoted.contains(&path) {
                render::ManifestKind::Promoted
            } else {
                match code {
                    'A' => render::ManifestKind::Added,
                    'D' => render::ManifestKind::Deleted,
                    _ => render::ManifestKind::Modified,
                }
            };
            render::ManifestEntry { path, kind }
        })
        .collect();
    let mut left_out = Vec::new();
    for (code, path) in porcelain {
        // The Y (worktree) column names the residual the index commit left behind; a blank
        // Y means the worktree matches the index (nothing left out for that path).
        let y = code.chars().nth(1).unwrap_or(' ');
        if y != ' ' {
            left_out.push(render::ManifestEntry {
                path,
                kind: column_kind(y),
            });
        }
    }
    (included, left_out)
}

/// Whether `path` (repo-relative) exists at `HEAD` (`git cat-file -e HEAD:<path>`).
/// Used by rollback to tell a newly promoted doc (no HEAD content — delete the copy)
/// from an overwrite of an existing committed doc (`git restore` already reverted it).
pub(crate) fn path_at_head(repo_root: &Path, path: &str) -> bool {
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
pub(crate) fn git_capture(repo_root: &Path, args: &[&str]) -> Result<String> {
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

/// A process-and-time-unique temp directory that removes itself on drop — the scratch
/// tree [`Task::materialize_index`] checks the git index out into (the `store_scratch_path`
/// / `describe::TempDir` idiom). Self-cleaning so a clean *or* blocked validate leaks
/// nothing.
struct ScratchTree(PathBuf);

impl ScratchTree {
    fn new() -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-index-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        ));
        ScratchTree(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ScratchTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
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

    /// The M19 capture layer: a successful `git commit` carries any non-blocking hook
    /// output back to the caller so the relay sites (per-task / aggregate) can surface
    /// it (`design/finalize.md` → 6. Commit). `git_commit` returns the combined captured
    /// stdout/stderr on success; a no-hook control returns empty; a rejecting hook still
    /// `bail!`s with its stderr (the correction signal, no commit lands).
    #[test]
    fn git_commit_returns_hook_output_on_success_and_bails_on_rejection() {
        let dir = std::env::temp_dir().join(format!("jigc-commit-hook-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mk temp repo");
        let run = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(&dir)
                .output()
                .expect("git runs");
            assert!(out.status.success(), "git {args:?} should succeed");
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "t@example.com"]);
        run(&["config", "user.name", "Test"]);
        run(&["config", "commit.gpgsign", "false"]);
        let msg = dir.join("msg.txt");
        std::fs::write(&msg, "test: a commit\n").expect("write msg");

        // Control: no hook, a real (non-empty) commit — returns empty captured output.
        std::fs::write(dir.join("a.txt"), "a\n").expect("write a");
        run(&["add", "--all"]);
        let captured = git_commit(&dir, &msg).expect("no-hook commit lands");
        assert!(
            captured.trim().is_empty(),
            "a no-hook commit returns empty captured output, got {captured:?}"
        );

        // A non-blocking `pre-commit` hook that prints to stdout and exits 0: the commit
        // lands and `git_commit` returns the hook's output.
        let hooks = dir.join(".git").join("hooks");
        std::fs::create_dir_all(&hooks).expect("mk hooks");
        let hook = hooks.join("pre-commit");
        std::fs::write(
            &hook,
            "#!/bin/sh\necho 'WARN: doc-code backstop says hi'\nexit 0\n",
        )
        .expect("write hook");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755))
                .expect("chmod hook");
        }
        std::fs::write(dir.join("b.txt"), "b\n").expect("write b");
        run(&["add", "--all"]);
        let captured = git_commit(&dir, &msg).expect("hook commit lands");
        assert!(
            captured.contains("doc-code backstop says hi"),
            "a successful non-blocking hook's output is captured, got {captured:?}"
        );

        // A rejecting hook (exit 1) still bails with its stderr; no commit lands.
        std::fs::write(
            &hook,
            "#!/bin/sh\necho 'BLOCK: this is rejected' 1>&2\nexit 1\n",
        )
        .expect("write rejecting hook");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755))
                .expect("chmod hook");
        }
        std::fs::write(dir.join("c.txt"), "c\n").expect("write c");
        run(&["add", "--all"]);
        let err = git_commit(&dir, &msg).expect_err("a rejecting hook bails");
        assert!(
            format!("{err:#}").contains("this is rejected"),
            "the rejection surfaces the hook's stderr, got {err:#}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The migration stage guards its fixed jigc-config pathspecs on existence (Inc 6
    /// advisory / review LOW): a `git add -- <existing> <absent>` is **fatal (exit 128)
    /// and stages nothing**, which would abort the byte-destructive migration commit. The
    /// first half reproduces that abort directly; the second proves `existing_pathspecs`
    /// drops the absent member so the present one still stages cleanly.
    #[test]
    fn existing_pathspecs_drops_absent_so_git_add_never_aborts() {
        let dir = std::env::temp_dir().join(format!("jigc-pathspec-guard-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mk temp repo");
        let run = |args: &[&str]| {
            Command::new("git")
                .args(args)
                .current_dir(&dir)
                .output()
                .expect("git runs")
        };
        assert!(run(&["init", "-q"]).status.success());
        // The jigc-config layer is present; `.jigc/.gitignore` is **absent** (the missing
        // fixed pathspec the fix defends against).
        std::fs::create_dir_all(dir.join(".jigc").join("config")).expect("mk config layer");
        std::fs::write(dir.join(".jigc").join("config").join("packs.yaml"), "{}\n")
            .expect("write config");
        assert!(!dir.join(".jigc").join(".gitignore").exists());

        // Reproduce the abort: feeding `git add --` the absent pathspec is fatal and
        // stages nothing — the failure mode the byte-destructive migration commit must
        // never hit.
        let unfiltered = run(&["add", "--", ".jigc/config", ".jigc/.gitignore"]);
        assert!(
            !unfiltered.status.success(),
            "a `git add` with an absent pathspec is fatal (the abort the fix defends against)"
        );
        let staged_after_abort = run(&["diff", "--cached", "--name-only"]);
        assert!(
            String::from_utf8_lossy(&staged_after_abort.stdout)
                .trim()
                .is_empty(),
            "the aborted `git add` stages nothing"
        );

        // The fix: `existing_pathspecs` drops the absent member, keeping input order.
        let specs = existing_pathspecs(&dir, &[".jigc/config", ".jigc/.gitignore"]);
        assert_eq!(
            specs,
            vec![".jigc/config".to_string()],
            "only the existing fixed pathspec survives"
        );

        // Staging just the survivors succeeds and lands the config layer in the index.
        let mut args: Vec<String> = vec!["add".into(), "--".into()];
        args.extend(specs);
        let filtered = run(&args.iter().map(String::as_str).collect::<Vec<_>>());
        assert!(
            filtered.status.success(),
            "staging only the existing pathspec succeeds (no abort)"
        );
        let staged = run(&["diff", "--cached", "--name-only"]);
        assert!(
            String::from_utf8_lossy(&staged.stdout).contains(".jigc/config/packs.yaml"),
            "the present config layer is staged"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
