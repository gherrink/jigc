//! `jigc milestone <verb>` — the `milestone` work-unit front door
//! (`design/write-commands.md` → Minting a milestone + its task list).
//!
//! Two verbs over the engine's milestone primitives ([`engine::milestone`]):
//!
//! - **`create "<title>"`** — read HEAD (the **single shared base** every sub-task
//!   inherits — "CLI orchestrates, git executes"), then [`mint_milestone`] opens
//!   `.jigc/milestones/<slug>/` with that base pin and an empty task list. A serial
//!   slug collision rejects with the engine's routed blocking finding.
//! - **`add-task <milestone-id> "<intent>"`** — [`add_task`] mints a sub-task
//!   pinned to the milestone's shared base in an isolated `.jigc/tasks/<sub>/`
//!   area and appends it; an unknown milestone or a within-milestone serial slug
//!   collision rejects (the deterministic suffix runs only at the join).
//!
//! The CLI does the git I/O (reading HEAD) and locates the `.jigc/` home; the
//! engine performs no git I/O, so minting stays a pure function of its inputs. As
//! with `tasks/`, the milestone area is gitignored runtime state
//! (`design/storage.md` → repository layout) — a doc-elaboration pin
//! (`DECISIONS.md` 2026-06-04) — so create ensures `.jigc/.gitignore` lists
//! `milestones/`.

use crate::cli::Format;
use crate::pack::make_pack;
use crate::render;
use crate::task::{git_diff, git_head, git_untracked};
use anyhow::{Context, Result, bail};
use engine::finalize::plan_milestone_finalize;
use engine::finding::Finding;
use engine::index::load_committed;
use engine::milestone::{
    JoinOutcome, add_from_spec, add_task, join, materialize, milestone_dir, mint_milestone,
    read_base_pin, read_task_list, synthesized_message, worktree_path,
};
use engine::packsource::PackResourceKind;
use engine::schema::Schema;
use engine::state::BasePin;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// The default minting workflow for a milestone sub-task when `add-task` /
/// `add-from-spec` are given no `--workflow <id>`. A sub-task is a `task` work-unit
/// re-entered via `jigc workflow <W> --task <id>`; its minting workflow is recorded
/// per sub-task (read back by [`engine::state::read_workflow_id`]) and the re-entry
/// asserts equality against it (`design/write-commands.md` → Minting a milestone, the
/// `--workflow <id>` arg; `DECISIONS.md` 2026-06-04). The choice of workflow per
/// sub-task is the caller's; here we only record it.
const DEFAULT_SUB_TASK_WORKFLOW: &str = "sub-task";

/// The workflow `jigc milestone execute` composes over a milestone work-unit. The
/// pack's single `creates-task: false` milestone work-workflow today; a cascade knob
/// can choose among several when a second one earns it — the same evolution
/// `default-workflow`/the router followed (`design/write-commands.md` → Executing the
/// milestone — `jigc milestone execute <id>`).
const MILESTONE_EXECUTION_WORKFLOW: &str = "milestone-execution";

/// The `jigc milestone <verb>` subcommand tree.
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum MilestoneCommand {
    /// Mint a milestone work-unit (id = frozen slug from the title), opening its
    /// gitignored area with one shared base pinned at HEAD and an empty task list.
    Create {
        /// The milestone title — slugged into the `milestone:<slug>` id.
        title: String,
    },
    /// Mint a sub-task under an existing milestone (pinned to the milestone's
    /// shared base, isolated `tasks/<sub>/` area) and append it to the task list.
    AddTask {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// The sub-task intent — slugged into the sub-task id.
        intent: String,
        /// The minting workflow recorded for the sub-task (read back on re-entry,
        /// which asserts equality against it). Defaults to `sub-task`.
        #[arg(long, default_value = DEFAULT_SUB_TASK_WORKFLOW)]
        workflow: String,
    },
    /// Seed a milestone's task list from a committed spec: mint one sub-task per
    /// repeatable `criterion` of the spec (criterion text as intent). A spec with
    /// zero criteria blocks with `milestone.no-criteria` ("nothing to seed from").
    AddFromSpec {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// The committed spec's address (`spec:<slug>`) to enumerate criteria from.
        spec_addr: String,
        /// The minting workflow recorded for every seeded sub-task (read back on
        /// re-entry, which asserts equality against it). Defaults to `sub-task`.
        #[arg(long, default_value = DEFAULT_SUB_TASK_WORKFLOW)]
        workflow: String,
    },
    /// Emit a milestone's sub-task ids in canonical id-sorted order — the
    /// deterministic order the by-task-id join enumerates.
    ListTasks {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
    },
    /// Provision the milestone's fan-out worktrees: add one **detached** `git
    /// worktree` per sub-task at the milestone's recorded **base pin** under the
    /// gitignored `.jigc/worktrees/<sub-task-id>` path, so each fanned sub-agent gets
    /// an isolated code checkout. Idempotent — reuses a live worktree, clears a stale
    /// leftover from a crashed run. Run as a `Run:` step before the fan-out
    /// (`design/storage.md` → repository layout).
    Provision {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
    },
    /// Compose the `milestone-execution` workflow over the milestone, feeding its
    /// id-sorted sub-task list into `{{milestone.tasks}}` so the `fan-out` step
    /// resolves it (one `Spawn:` directive per sub-task). Mints nothing — the
    /// milestone and its sub-tasks already exist; an unknown milestone is rejected.
    Execute {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
    },
    /// Merge the milestone's sub-task areas into the parent working overlay by the
    /// by-task-id join — enumerate sub-areas by sorted task id, disjoint-union their
    /// staged docs (collision-suffixing distinct created instances), and report the
    /// merged outcome. Commits nothing. A blocking finding (a same-doc clash, an
    /// unknown milestone) surfaces on stderr with its route and exits non-zero.
    Join {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
    },
    /// The milestone commit boundary — run the by-task-id join, materialize its
    /// suffix-resolved doc bodies into the parent staging area, and commit them as
    /// one logical boundary with a CLI-synthesized message. A blocking join finding
    /// (a same-doc clash, an unknown milestone) routes to stderr and commits nothing.
    Finalize {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
    },
}

impl MilestoneCommand {
    /// Dispatch the parsed `milestone` verb against `cwd`, mapping the result to a
    /// process exit code. Success prints a summary on stdout and exits 0; a blocking
    /// finding (serial collision, unknown milestone) surfaces on stderr with its
    /// route and exits non-zero (`design/write-commands.md` → Minting a milestone).
    pub fn dispatch(self, cwd: &Path, format: Format) -> ExitCode {
        // The `join` verb reports a `JoinOutcome` (overlay + findings), not a one-line
        // summary, and a same-doc clash is a *blocking finding inside an Ok outcome*
        // (the merge ran, then routed the contention) — so it has its own dispatch arm.
        if let MilestoneCommand::Join { milestone_id } = self {
            return dispatch_join(cwd, format, &milestone_id);
        }
        // `finalize` is the commit boundary: it materializes the join and drives the
        // shared finalize-plan executor (git I/O), returning a process exit code rather
        // than a one-line summary — so it, too, has its own dispatch arm.
        if let MilestoneCommand::Finalize { milestone_id } = self {
            return dispatch_finalize(cwd, format, &milestone_id);
        }
        // `execute` composes a workflow and emits the composed view (not a one-line
        // summary), so — like `join`/`finalize` — it has its own dispatch arm.
        if let MilestoneCommand::Execute { milestone_id } = self {
            return dispatch_execute(cwd, format, &milestone_id);
        }
        let result = match self {
            MilestoneCommand::Create { title } => run_create(cwd, &title),
            MilestoneCommand::AddTask {
                milestone_id,
                intent,
                workflow,
            } => run_add_task(cwd, &milestone_id, &intent, &workflow),
            MilestoneCommand::AddFromSpec {
                milestone_id,
                spec_addr,
                workflow,
            } => run_add_from_spec(cwd, &milestone_id, &spec_addr, &workflow),
            MilestoneCommand::ListTasks { milestone_id } => run_list_tasks(cwd, &milestone_id),
            MilestoneCommand::Provision { milestone_id } => run_provision(cwd, &milestone_id),
            MilestoneCommand::Execute { .. } => unreachable!("`Execute` is handled above"),
            MilestoneCommand::Join { .. } => unreachable!("`Join` is handled above"),
            MilestoneCommand::Finalize { .. } => unreachable!("`Finalize` is handled above"),
        };
        match result {
            Ok(summary) => {
                println!("{}", render::milestone(format, &summary));
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("{}", render::operational_error(format, &err));
                ExitCode::FAILURE
            }
        }
    }
}

/// `jigc milestone create "<title>"` — read HEAD, then mint the milestone area
/// with that single shared base and an empty task list. Ensures `.jigc/.gitignore`
/// lists `milestones/` (the area is disposable runtime state). Returns the summary
/// line; a serial collision surfaces as the engine's routed blocking finding.
fn run_create(cwd: &Path, title: &str) -> Result<String> {
    // The base pin is the *worktree* HEAD; the `.jigc/` area binds to jigc_home (the main
    // checkout), so all worktrees share one `.jigc/` (M31 Inc 2 / WF3).
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_root = crate::start::jigc_home_or_repo(cwd)?.join(".jigc");
    ensure_jigc_gitignore(&jigc_root)?;
    let base = read_head(&repo_root)?;

    let minted = mint_milestone(&jigc_root, title, base).map_err(finding_to_err)?;
    Ok(format!(
        "minted milestone:{} (shared base {})",
        minted.id, minted.base.short
    ))
}

/// `jigc milestone add-task <milestone-id> "<intent>"` — mint a sub-task pinned to
/// the milestone's shared base in its own isolated area and append it. Returns the
/// summary line; an unknown milestone or a within-milestone collision surfaces as
/// the engine's routed blocking finding.
fn run_add_task(cwd: &Path, milestone_id: &str, intent: &str, workflow: &str) -> Result<String> {
    // The `.jigc/` area binds to jigc_home (the main checkout); no git read here.
    let jigc_root = crate::start::jigc_home_or_repo(cwd)?.join(".jigc");

    let added = add_task(&jigc_root, milestone_id, intent, workflow).map_err(finding_to_err)?;
    Ok(format!(
        "added task:{} to milestone:{}",
        added.task.id, added.milestone_id
    ))
}

/// `jigc milestone add-from-spec <milestone-id> <spec-addr>` — seed the milestone's
/// task list with one sub-task per repeatable `criterion` of a committed spec (the
/// criterion text as the sub-task intent). The CLI does the I/O — discover the repo
/// root, load the shipped schemas the engine resolves the spec address against — and
/// hands them to [`add_from_spec`], which performs no git I/O. Returns a summary
/// naming the seeded sub-tasks; an unknown milestone, an unknown/transient/
/// unparseable spec, or a **zero-criteria** spec (`milestone.no-criteria`) surfaces
/// as the engine's routed blocking finding and exits non-zero
/// (`design/write-commands.md` → Minting a milestone).
fn run_add_from_spec(
    cwd: &Path,
    milestone_id: &str,
    spec_addr: &str,
    workflow: &str,
) -> Result<String> {
    // The committed spec read + the `.jigc/` sub-task mint both bind to jigc_home (the
    // main checkout); no git read here (sub-tasks pin to the milestone's stored base).
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    let schemas = shipped_schemas(&jigc_home)?;

    let added = add_from_spec(
        &jigc_root,
        &jigc_home,
        &schemas,
        milestone_id,
        spec_addr,
        workflow,
    )
    .map_err(finding_to_err)?;

    let ids: Vec<&str> = added.iter().map(|a| a.task.id.as_str()).collect();
    Ok(format!(
        "seeded {} sub-task(s) into milestone:{milestone_id} from {spec_addr}: {}",
        ids.len(),
        ids.join(", ")
    ))
}

/// Load every shipped schema keyed by doctype — the set [`add_from_spec`] resolves
/// the spec address's type against and [`run_join`]'s committed-index rebuild walks
/// (the engine stays domain-empty; the CLI feeds the pack in, the same idiom
/// `TaskArea::schemas` uses). Nests each `location:` under the resolved `docs-root`
/// (a schema-load surface — the committed-store reads must match the finalize-promote
/// write path).
fn shipped_schemas(repo_root: &Path) -> Result<BTreeMap<String, Schema>> {
    let pack = make_pack();
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .with_context(|| format!("the `{}` schema reads back", id.as_str()))?;
        let schema = crate::pack::load_pack_schema(pack.as_ref(), &bytes)
            .with_context(|| format!("the `{}` schema parses", id.as_str()))?;
        out.insert(schema.ty.clone(), schema);
    }
    let resolved =
        crate::start::resolve_severity_cascade(pack.as_ref(), &repo_root.join(".jigc/config"))?;
    crate::start::apply_docs_root(&resolved, out.values_mut());
    Ok(out)
}

/// `jigc milestone list-tasks <milestone-id>` — read the milestone's persisted
/// task list and emit its sub-task ids in **canonical id-sorted order** (the
/// deterministic order the by-task-id join enumerates, surfaced through the
/// binary, not just the internal enumerate fn). Returns the summary line; an
/// unknown milestone (no area / unreadable list) surfaces as a context-wrapped
/// error and exits non-zero.
fn run_list_tasks(cwd: &Path, milestone_id: &str) -> Result<String> {
    // The `.jigc/` area binds to jigc_home (the main checkout); no git read here.
    let jigc_root = crate::start::jigc_home_or_repo(cwd)?.join(".jigc");
    let dir = milestone_dir(&jigc_root, milestone_id);
    if !dir.is_dir() {
        bail!(
            "milestone `{milestone_id}` does not exist\n  route: create it first with `jigc milestone create \"<title>\"`"
        );
    }
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;
    // Enumeration is id-sorted — the order the join reads, not the recorded
    // insertion order.
    let ids = list.enumerate();
    Ok(format!(
        "milestone:{milestone_id} tasks ({}): {}",
        ids.len(),
        ids.join(", ")
    ))
}

/// `jigc milestone provision <milestone-id>` — add one **detached** `git worktree`
/// per sub-task at the milestone's recorded **base pin** (`read_base_pin().sha`, the
/// commit every sub-task inherited — **never** main HEAD, which may have advanced),
/// under the gitignored `.jigc/worktrees/<sub-task-id>` path, so each fanned sub-agent
/// runs against an isolated code checkout (`design/storage.md` → repository layout;
/// `DECISIONS.md` 2026-06-20 → M31 planning, WF4). Run as a `Run:` step before the
/// fan-out (T2 wires the emission).
///
/// The CLI does the git I/O: HEAD never enters here (the base is the milestone's
/// **stored** pin), but the worktree shell-outs run on the main checkout `repo_root`
/// while the `.jigc/worktrees/` parent binds to jigc_home (the M31 WF3 split — outside
/// a worktree the two coincide). **Idempotent**: a re-run reuses a live worktree, and a
/// stale leftover dir from a crashed run is pruned/cleared before the add. An unknown
/// milestone (no area) surfaces as a context-wrapped error.
fn run_provision(cwd: &Path, milestone_id: &str) -> Result<String> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    // `.jigc/worktrees/` must be ignored or the linked worktrees pollute the main
    // checkout's `git status` / `git add --all`.
    ensure_jigc_gitignore(&jigc_root)?;

    let dir = milestone_dir(&jigc_root, milestone_id);
    if !dir.is_dir() {
        bail!(
            "milestone `{milestone_id}` does not exist\n  route: create it first with `jigc milestone create \"<title>\"`"
        );
    }
    // The single shared base pin every sub-task inherited — the commit the worktrees
    // detach at, never a fresh HEAD.
    let base = read_base_pin(&dir).with_context(|| {
        format!("could not read the shared base pin for milestone `{milestone_id}`")
    })?;
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;
    // Id-sorted ids — the deterministic order the fan-out spawns its sub-agents.
    let ids = list.enumerate();

    let paths = provision_worktrees(&repo_root, &jigc_home, &base.sha, &ids)?;
    Ok(format!(
        "provisioned {} worktree(s) for milestone:{milestone_id} at base {} ({})",
        paths.len(),
        base.short,
        ids.join(", ")
    ))
}

/// Add one **detached** worktree per sub-task at `base_sha` under
/// `<jigc_home>/.jigc/worktrees/<id>`, **idempotently**. The worktrees-parent is
/// created first (git makes only the leaf), then `git worktree prune` drops admin
/// records for any worktree whose dir was deleted by a crashed run. For each sub-task:
/// a worktree already registered at the exact path is reused (the crashed run's
/// worktree, or a prior provision — left untouched); otherwise a stale non-registered
/// leftover dir is cleared and `git worktree add --detach` lands a fresh one. Returns
/// the absolute worktree paths in the input (id-sorted) order.
fn provision_worktrees(
    repo_root: &Path,
    jigc_home: &Path,
    base_sha: &str,
    sub_ids: &[String],
) -> Result<Vec<PathBuf>> {
    // Canonicalize jigc_home so the per-id paths match `git worktree list`'s canonical
    // absolute paths (git resolves symlinks at `add` time) — the reuse comparison below.
    let canonical_home = jigc_home
        .canonicalize()
        .with_context(|| format!("could not canonicalize {jigc_home:?}"))?;
    let worktrees_root = canonical_home.join(".jigc").join("worktrees");
    std::fs::create_dir_all(&worktrees_root)
        .with_context(|| format!("could not create {worktrees_root:?}"))?;

    // Drop admin records for any worktree dir deleted out from under git by a crashed
    // run, so a later `add` at that path is not rejected as a stale registration.
    git_worktree(repo_root, &["worktree", "prune"])?;
    let registered = registered_worktrees(repo_root)?;

    let mut paths = Vec::with_capacity(sub_ids.len());
    for id in sub_ids {
        // The absolute worktree path; `worktree_path(id)` is the shared relative
        // convention (`.jigc/worktrees/<id>`) the spawn line also renders.
        let path = canonical_home.join(worktree_path(id));
        if registered.iter().any(|w| w == &path) {
            // Already a registered worktree at this exact path — reuse it (idempotent).
            paths.push(path);
            continue;
        }
        // A stale, non-registered leftover dir would make `git worktree add` fail
        // ("already exists"); clear it first.
        if path.exists() {
            std::fs::remove_dir_all(&path)
                .with_context(|| format!("could not clear the stale worktree dir {path:?}"))?;
        }
        let path_str = path
            .to_str()
            .with_context(|| format!("worktree path {path:?} is not valid UTF-8"))?;
        git_worktree(
            repo_root,
            &["worktree", "add", "--detach", path_str, base_sha],
        )?;
        paths.push(path);
    }
    Ok(paths)
}

/// The canonical absolute paths of the repo's currently-registered worktrees, parsed
/// from `git worktree list --porcelain` (each `worktree <path>` line carries the
/// canonical path git stored at `add` time). The provision reuse check compares against
/// these.
fn registered_worktrees(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let out = git_worktree(repo_root, &["worktree", "list", "--porcelain"])?;
    Ok(out
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(PathBuf::from)
        .collect())
}

/// Run `git <args>` in `repo_root`, returning full stdout on success — the worktree
/// provisioning shell-outs ("CLI orchestrates, git executes"). Bails with git's stderr
/// on a non-zero exit (the `git_rev_parse` envelope, kept separate because the worktree
/// commands need the full multi-line stdout, not a single trimmed line).
fn git_worktree(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    String::from_utf8(out.stdout).context("`git` produced non-UTF-8 output")
}

/// Dispatch `jigc milestone execute <milestone-id>`: compose the milestone-execution
/// workflow over the milestone's id-sorted sub-task list, render the composed view on
/// stdout, and map it to the exit code. An unknown milestone (no area) or a blocking
/// compose finding routes to stderr **before** any output and exits non-zero
/// (`design/write-commands.md` → Executing the milestone).
fn dispatch_execute(cwd: &Path, format: Format, milestone_id: &str) -> ExitCode {
    match run_execute(cwd, milestone_id) {
        Ok(view) => {
            println!("{}", render::composed(format, &view));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
    }
}

/// `jigc milestone execute <milestone-id>` — resolve the milestone work-unit (an
/// unknown id routes a blocking finding **before** any compose, the `dispatch_finalize`
/// precedent), read its **id-sorted** `TaskList::enumerate()`, and compose the
/// `creates-task: false` `milestone-execution` workflow over it — feeding the id-sorted
/// list into `{{milestone.tasks}}` so the `fan-out` step resolves it (one `Spawn:`
/// directive per sub-task, id-ordered). The lone production site feeding
/// [`ComposeContext::milestone`] non-empty; **mints nothing** (the milestone and its
/// sub-tasks already exist).
fn run_execute(cwd: &Path, milestone_id: &str) -> Result<engine::compose::ComposedWorkflow> {
    // The `.jigc/` area binds to jigc_home (the main checkout); the compose feed resolves
    // the same split internally (it derives jigc_home from the worktree `repo_root`).
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_root = crate::start::jigc_home_or_repo(cwd)?.join(".jigc");
    let dir = milestone_dir(&jigc_root, milestone_id);
    if !dir.is_dir() {
        bail!(
            "milestone `{milestone_id}` does not exist\n  route: create it first with `jigc milestone create \"<title>\"`"
        );
    }
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;
    // Enumeration is id-sorted — the order the fan-out emits its `Spawn:` directives.
    let ids = list.enumerate();
    crate::start::execute_milestone_in_repo(&repo_root, MILESTONE_EXECUTION_WORKFLOW, &ids)
}

/// Dispatch `jigc milestone join <milestone-id>`: run the by-task-id join, render the
/// merged outcome, and map it to the exit code. A locator/IO error or an unknown
/// milestone (an `Err(Finding)`) routes to stderr and exits non-zero **before** any
/// summary. A successful merge always renders the outcome on stdout (so the agent sees
/// the suffix/rewrite decisions); if any **blocking** finding rode inside the outcome —
/// a same-doc clash, an isolation violation — its route also goes to stderr and the
/// process exits non-zero. The verb **commits nothing** (Increment 4 wires the
/// suffix-resolved overlay into finalize); a clash leaves the working tree untouched.
fn dispatch_join(cwd: &Path, format: Format, milestone_id: &str) -> ExitCode {
    let outcome = match run_join(cwd, milestone_id) {
        Ok(outcome) => outcome,
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            return ExitCode::FAILURE;
        }
    };
    // The merged-overlay summary always prints (the agent reads the suffix/rewrite
    // decisions even when the join is clean).
    println!("{}", render::milestone_join(format, milestone_id, &outcome));

    // A blocking finding inside the outcome (e.g. `join.same-doc-clash`) routes to
    // stderr and gates the exit code — nothing is committed regardless.
    let blocking: Vec<&Finding> = outcome
        .findings
        .iter()
        .filter(|f| f.severity == engine::finding::Severity::Blocking)
        .collect();
    if blocking.is_empty() {
        ExitCode::SUCCESS
    } else {
        for finding in blocking {
            eprintln!("{}", finding.message);
            if let Some(route) = &finding.route {
                eprintln!("  route: {route}");
            }
        }
        ExitCode::FAILURE
    }
}

/// `jigc milestone join <milestone-id>` — the CLI I/O around the engine's by-task-id
/// [`join`]: discover the repo root + `.jigc/` home, read the milestone's **shared
/// base** pin (the stamp the committed edge index is keyed to), load the shipped
/// schemas the per-area overlay derivation resolves staged-doc types against, and load
/// the committed [`EdgeIndex`](engine::index::EdgeIndex) the per-sub-area cross-area
/// ref walk resolves against. The engine performs no git I/O; the CLI feeds it the
/// resolved inputs (`design/storage.md` → The by-task-id join). An unknown milestone
/// (no area) surfaces as the engine's routed `milestone.unknown` block.
fn run_join(cwd: &Path, milestone_id: &str) -> Result<JoinOutcome> {
    // The committed doc-store + `.jigc/` index bind to jigc_home (the main checkout); the
    // join performs no git I/O (the base is the milestone's *stored* pin) (M31 Inc 2).
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    let schemas = shipped_schemas(&jigc_home)?;

    // The committed edge index is keyed to the milestone's shared base — the commit
    // every sub-task inherited — so the cross-area ref walk resolves against the store
    // as it stood at that base. An unknown milestone has no base pin; fall back to the
    // engine's routed `milestone.unknown` block by leaving the join to detect it.
    let head = match read_base_pin(&milestone_dir(&jigc_root, milestone_id)) {
        Ok(pin) => pin.sha,
        Err(_) => String::new(),
    };
    let committed = load_committed(&jigc_home, &jigc_root, &schemas, &head);

    join(&jigc_root, &jigc_home, milestone_id, &schemas, &committed).map_err(finding_to_err)
}

/// Dispatch `jigc milestone finalize <milestone-id>`: run the materialized join +
/// single-commit boundary and map it to the exit code. A blocking join finding (a
/// same-doc clash, an unknown milestone) or an orchestration error routes to stderr and
/// exits non-zero **before** any commit. A landed commit prints a summary and exits 0.
/// `design/finalize.md` → `fan-out` finalize (single-commit form).
fn dispatch_finalize(cwd: &Path, format: Format, milestone_id: &str) -> ExitCode {
    match run_milestone_finalize(cwd, format, milestone_id) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
    }
}

/// `jigc milestone finalize <milestone-id>` — the milestone commit boundary
/// (`design/finalize.md` → `fan-out` finalize, single-commit form; `design/storage.md` →
/// one logical commit boundary).
///
/// The flow reuses the engine primitives end-to-end: (1) [`materialize`] runs the
/// by-task-id join and writes the suffix-resolved doc bodies into the parent staging area
/// `<.jigc>/milestones/<id>/merged/docs/` — **blocking** (surfacing the routed finding,
/// committing nothing) if the join holds any blocking finding (a same-doc clash, an
/// unknown milestone), the `dispatch_join` precedent; (2) the message is the
/// CLI-[`synthesized_message`] structural projection of the milestone id + its id-ordered
/// sub-task list (a milestone has no commit doc to render — planner-note (b)); (3)
/// [`plan_milestone_finalize`] runs the shared preflight (`base` == HEAD) + empty-commit
/// guard + promote/hash sweep over the materialized staging area; (4) the **shared**
/// [`crate::task::execute_finalize_plan`] executor promotes, stages, and commits in **one**
/// boundary, removing the milestone area on success. The engine performs no git; the CLI
/// reads HEAD and locates `.jigc/`.
fn run_milestone_finalize(cwd: &Path, format: Format, milestone_id: &str) -> Result<ExitCode> {
    // The committed doc-store + `.jigc/` bind to jigc_home (the main checkout); HEAD + the
    // git commit/stage stay on the worktree `repo_root` (M31 Inc 2 / WF3). The promote
    // transaction (`try_execute_finalize_plan` / `execute_finalize_plan`) is kept on
    // `repo_root` — the worktree-finalize promote placement is the deferred WF4/WF5 concern.
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    let schemas = shipped_schemas(&jigc_home)?;

    let dir = milestone_dir(&jigc_root, milestone_id);
    if !dir.is_dir() {
        bail!(
            "milestone `{milestone_id}` does not exist\n  route: create it first with `jigc milestone create \"<title>\"`"
        );
    }

    // The committed edge index is keyed to the milestone's shared base (the commit every
    // sub-task inherited) — the cross-area ref walk inside the join resolves against the
    // store as it stood at that base.
    let base = read_base_pin(&dir).with_context(|| {
        format!("could not read the shared base pin for milestone `{milestone_id}`")
    })?;
    let committed = load_committed(&jigc_home, &jigc_root, &schemas, &base.sha);

    // Step 1 — materialize the join's suffix-resolved bodies. A blocking join finding (a
    // same-doc clash) surfaces here and commits nothing (the materialize blocks before it
    // writes — the `dispatch_join` precedent, planner-note (c)).
    let materialized = materialize(&jigc_root, &jigc_home, milestone_id, &schemas, &committed)
        .map_err(finding_to_err)?;

    // Step 2 — the CLI-synthesized message (a milestone has no commit doc to render).
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;
    let message = synthesized_message(milestone_id, &list);

    // The `finalize.fan-out.squash` knob (`design/finalize.md` → `fan-out` finalize) shapes
    // the commit. Resolve it from the project cascade (a missing `.jigc/config/` layer
    // yields the pack-default base — `squash` defaults `true`, the M7 single-aggregate form
    // whose bytes stay byte-identical to today, the read-side determinism guard). `false`
    // lays down one commit per sub-task in **id-sorted order** (rendering each sub-task's
    // own authored `commit:<sub-id>` doc) BEFORE the parent's synthesized aggregate — the
    // commit *sequence* is a pure function of the id set (hardening #7), even though each
    // sub-task message carries the sub-agent's authored prose (the CLI owns the ordering).
    // Read here (before any sub-task commit moves HEAD); the per-sub-task commits are laid
    // down only AFTER the planner's preflight validates base == HEAD below.
    let squash = resolve_squash(&jigc_home)?;

    // The diff-presence signal the planner's empty-commit guard needs: the materialized
    // docs that will be promoted, plus any working-tree change / untracked file from base.
    let head = git_head(&repo_root)?;
    let has_diff = !materialized.addresses.is_empty()
        || !git_diff(&repo_root, &base.sha)?.trim().is_empty()
        || !git_untracked(&repo_root)?.trim().is_empty();

    // Step 3 — the thin sibling planner over the materialized staging area (the parent of
    // `merged/docs/`): shared preflight + empty-commit guard + promote/hash sweep.
    let staging_dir = materialized
        .docs_dir
        .parent()
        .expect("the materialized docs dir has a parent staging area")
        .to_path_buf();
    let plan =
        match plan_milestone_finalize(&staging_dir, &base, &head, message, has_diff, &schemas) {
            Ok(plan) => plan,
            Err(findings) => {
                for finding in &findings {
                    eprintln!("{}", finding.message);
                    if let Some(route) = &finding.route {
                        eprintln!("  route: {route}");
                    }
                }
                // The same outcome class as the task-finalize planner block: a
                // `plan_*_finalize`-findings block is a validation outcome, exit 3
                // (`design/measurement.md` → The capture substrate, item 2).
                return Ok(ExitCode::from(crate::task::EXIT_VALIDATION_BLOCKED));
            }
        };

    // `squash: false` — lay down the per-sub-task commits in id order now (the planner's
    // preflight has validated base == HEAD against the pre-boundary HEAD; these commits then
    // advance HEAD, and the parent aggregate that follows re-checks nothing — the boundary's
    // base invariant was the planner's call). On `true` this is skipped, leaving the single
    // CLI-synthesized aggregate exactly as M7 shipped it (the default-path determinism guard).
    //
    // The N sub-task commits advance HEAD BEFORE the parent aggregate lands, so the boundary
    // would not be all-or-nothing unless we can undo them: capture the pre-finalize HEAD here
    // (before any sub-task commit moves it). If the aggregate finalize fails, `git reset
    // --hard` to this sha returns HEAD + index + working tree to as-if-finalize-was-never-
    // called — no orphaned sub-task commits (`CLAUDE.md` "Writes are transactional";
    // `design/finalize.md` → Rollback discipline). `execute_finalize_plan` already rolls back
    // its own promotions; this reset additionally undoes the per-sub-task commits the
    // milestone boundary laid down ahead of it.
    if !squash {
        let pre_finalize_head = git_head(&repo_root)?;
        commit_per_subtask_messages(&repo_root, &dir, milestone_id, &list, &schemas)?;

        // Step 4 — the SHARED executor: promote + stage + commit (one boundary) + post-commit.
        // The milestone boundary runs no reconcile sweep → no post-sweep record (`None`),
        // and keeps the whole-tree `git add --all` sweep (M30 G1 — sub-agent code is
        // unstaged-by-design; narrowing it would drop it, the M31 worktree redesign owns it).
        match crate::task::try_execute_finalize_plan(
            &repo_root,
            &jigc_root,
            &dir,
            &plan,
            &dir,
            &schemas,
            None,
            crate::task::StagePolicy::Sweep,
        )? {
            Ok(hook_output) => {
                // T3 — relay ONLY the aggregate `git_commit`'s non-blocking hook output
                // (the N per-sub-task `commit_empty_message` commits laid down above
                // relay nothing — `design/finalize.md` → 6. Commit, review B1: fan-out
                // relays only the aggregate). Same placement discipline as T2.
                crate::task::relay_hook_output(format, &hook_output);
                // The boundary landed — clean up the per-sub-task working areas too (the
                // executor only removed the milestone area). On a failure (below) the areas
                // survive for retry.
                cleanup_subtask_areas(&jigc_root, &list);
                Ok(ExitCode::SUCCESS)
            }
            // Aggregate rejected (a commit/hook rejection). The executor already rolled
            // back its promotions; now undo the per-sub-task commits + any staging so HEAD
            // returns to the pre-finalize sha (all-or-nothing), then surface git's stderr
            // verbatim and exit `FAILURE` — the same signal `execute_finalize_plan` gives.
            // The sub-task areas are left intact (not cleaned) so a retry works.
            Err(err) => {
                crate::task::git_reset_hard(&repo_root, &pre_finalize_head)?;
                eprintln!("{}", render::operational_error(format, &err));
                Ok(ExitCode::FAILURE)
            }
        }
    } else {
        // Step 4 — the SHARED executor: promote + stage + commit (one boundary) + post-commit.
        // The message temp file is written into the (gitignored) milestone area; the milestone
        // area is the cleanup dir removed on a landed commit. The `squash: true` default path
        // lands only the single CLI-synthesized aggregate (no per-sub-task commits), so it is
        // byte-identical to what M7 shipped — left untouched.
        let code = crate::task::execute_finalize_plan(
            &repo_root, &jigc_root, &dir, &plan, &dir, &schemas, format,
        )?;
        // On a landed commit, clean up the per-sub-task working areas too (the executor only
        // removed the milestone area). A failed/rolled-back finalize exits non-zero and leaves
        // the areas intact for retry.
        if code == ExitCode::SUCCESS {
            cleanup_subtask_areas(&jigc_root, &list);
        }
        Ok(code)
    }
}

/// Remove each sub-task's working area (`.jigc/tasks/<sub-id>/`) after a **landed**
/// milestone finalize — the gitignored runtime state the executor's milestone-area
/// cleanup leaves behind (it removes only the milestone area). Mirrors the single-task
/// post-commit cleanup discipline ([`crate::task::post_commit`]): best-effort,
/// logged-not-raised — a cleanup failure must not fail a commit that already landed (the
/// "self-heal" stance). Called ONLY on the success path, so a failed/rolled-back finalize
/// leaves the areas intact for a retry (respecting the F1 rollback path). Enumerates the
/// milestone's id-sorted sub-task list, so the set cleaned is exactly its sub-tasks.
fn cleanup_subtask_areas(jigc_root: &Path, list: &engine::milestone::TaskList) {
    let tasks_root = jigc_root.join("tasks");
    for sub_id in list.enumerate() {
        let area = tasks_root.join(&sub_id);
        if area.exists()
            && let Err(err) = std::fs::remove_dir_all(&area)
        {
            eprintln!(
                "note: post-commit sub-task working-area removal for `{sub_id}` failed (self-heals): {err:#}"
            );
        }
    }
}

/// The `commit` doc type the per-sub-task render addresses — a sub-task's authored
/// commit doc is `commit:<sub-id>` (the `commit:<task-id>` provisioning convention),
/// the transient type whose sink is the git message.
const COMMIT_TYPE: &str = "commit";

/// Resolve the `finalize.fan-out.squash` knob for the project at `repo_root` —
/// `true` (the pack default) keeps the single CLI-synthesized aggregate commit; `false`
/// opts into per-sub-task commits. A missing `.jigc/config/` layer resolves to the
/// pack-default base (`true`), so the no-knob/default path is byte-identical to today
/// (`design/overrides.md` → Read-side determinism; the `task validate`/`finalize`
/// cascade-resolve idiom, `crate::start::resolve_severity_cascade`).
fn resolve_squash(repo_root: &Path) -> Result<bool> {
    let project_config = repo_root.join(".jigc").join("config");
    let pack = make_pack();
    let resolved = crate::start::resolve_severity_cascade(pack.as_ref(), &project_config)?;
    // The knob is a declared `bool`; the closed surface guarantees it resolves. Anything
    // other than `true` is the opt-in `false` (the knob's enum-of-bool is `true`/`false`).
    Ok(resolved.scalar_required("finalize.fan-out.squash")? == "true")
}

/// Lay down **one commit per sub-task in id-sorted order** for the `squash: false` mode
/// (`design/finalize.md` → `fan-out` finalize). The engine renders each sub-task's
/// authored `commit:<sub-id>` doc into its message in id order
/// ([`engine::finalize::render_subtask_messages`] over the **id-sorted**
/// [`engine::milestone::TaskList::enumerate`] ids); the CLI commits each with
/// `--allow-empty` (the merged tree lands in the parent aggregate that follows, so a
/// sub-task commit is intentionally tree-empty yet a real commit in the deterministic
/// sequence). A sub-task missing its authored commit doc routes the engine's blocking
/// render finding and commits nothing further.
fn commit_per_subtask_messages(
    repo_root: &Path,
    msg_tmp_dir: &Path,
    milestone_id: &str,
    list: &engine::milestone::TaskList,
    schemas: &BTreeMap<String, Schema>,
) -> Result<()> {
    let commit_schema = schemas
        .get(COMMIT_TYPE)
        .with_context(|| format!("the embedded pack ships no `{COMMIT_TYPE}` schema"))?;
    let tasks_root = repo_root.join(".jigc").join("tasks");
    // Id-sorted ids — the deterministic order the commit sequence is laid down in.
    let ids = list.enumerate();
    let messages = engine::finalize::render_subtask_messages(&ids, &tasks_root, commit_schema)
        .map_err(|findings| {
            // The first blocking finding carries the route (the `dispatch_finalize` envelope).
            findings
                .into_iter()
                .next()
                .map(finding_to_err)
                .unwrap_or_else(|| {
                    anyhow::anyhow!(
                        "could not render a sub-task commit for milestone `{milestone_id}`"
                    )
                })
        })?;
    for message in &messages {
        crate::task::commit_empty_message(repo_root, msg_tmp_dir, message)?;
    }
    Ok(())
}

/// Ensure `.jigc/.gitignore` ignores the transient runtime subdirs, including
/// `milestones/` and the fan-out `worktrees/` (`design/storage.md` → repository layout;
/// `DECISIONS.md` 2026-06-04 → `milestones/` gitignored like `tasks/`; `DECISIONS.md`
/// 2026-06-21 → M31 Inc 3 adds `worktrees/`). Idempotent — the file is (re)written only
/// when it is absent or does not already list **both** `milestones/` and `worktrees/`,
/// so an adapter-written `.gitignore` (which predates either) is amended once.
fn ensure_jigc_gitignore(jigc_root: &Path) -> Result<()> {
    const ENTRIES: &str = "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\n";
    let path = jigc_root.join(".gitignore");
    let needs_write = match std::fs::read_to_string(&path) {
        Ok(existing) => {
            let lines: Vec<&str> = existing.lines().map(str::trim).collect();
            !lines.contains(&"milestones/") || !lines.contains(&"worktrees/")
        }
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

/// Read HEAD as a [`BasePin`] (full + short SHA) via the user's `git` — the same
/// idiom `crate::start::read_head` uses ("CLI orchestrates, git executes";
/// `DECISIONS.md` 2026-05-31 → Git invocation).
///
/// A **zero-commit** repo (unborn HEAD) pins the milestone mint to the canonical
/// empty-tree sentinel rather than bailing, so the milestone mint runs pre-first-commit
/// like the single-task mint (`design/project-setup.md` → Flow 2 hardening —
/// zero-commit). The unborn case is detected distinctly via
/// [`crate::task::head_is_unborn`] so a real git failure still bails.
fn read_head(repo_root: &Path) -> Result<BasePin> {
    if crate::task::head_is_unborn(repo_root)? {
        return Ok(BasePin::new(
            crate::task::EMPTY_TREE_SHA,
            crate::task::EMPTY_TREE_SHORT,
        ));
    }
    let sha = git_rev_parse(repo_root, &["rev-parse", "HEAD"])?;
    let short = git_rev_parse(repo_root, &["rev-parse", "--short", "HEAD"])?;
    Ok(BasePin::new(sha, short))
}

/// Run `git <args>` in `repo_root` and return the single trimmed line of stdout.
fn git_rev_parse(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    String::from_utf8(out.stdout)
        .context("`git` produced non-UTF-8 output")
        .map(|s| s.trim().to_string())
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the same
/// discovery `crate::start` / `crate::task` do.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route —
/// the same envelope minting already uses (a hard block is a blocking-severity
/// finding carrying a route, `DECISIONS.md` 2026-05-31).
fn finding_to_err(finding: Finding) -> anyhow::Error {
    match finding.route {
        Some(route) => anyhow::anyhow!("{}\n  route: {route}", finding.message),
        None => anyhow::anyhow!("{}", finding.message),
    }
}

#[cfg(test)]
mod tests {
    use crate::start::{PackStepSource, execute_milestone_core};
    use engine::packsource::{PackError, PackResourceKind, PackSource, ResourceId};
    use std::collections::HashMap;
    use std::path::Path;

    /// An in-memory [`PackSource`] seeded from `(kind, id, bytes)` triples — the
    /// `FixturePack` idiom the `start.rs` compose tests use, lets the milestone-feeding
    /// dispatch drive [`execute_milestone_core`] over a fixture `fan-out` workflow with
    /// no embedded pack and no real `milestone-execution` workflow (T3).
    struct FixturePack(HashMap<(PackResourceKind, ResourceId), Vec<u8>>);

    impl FixturePack {
        fn with(triples: Vec<(PackResourceKind, &str, &str)>) -> Self {
            FixturePack(
                triples
                    .into_iter()
                    .map(|(kind, id, body)| {
                        ((kind, ResourceId::from(id)), body.as_bytes().to_vec())
                    })
                    .collect(),
            )
        }
    }

    impl PackSource for FixturePack {
        fn pack_version(&self) -> String {
            "0.0.0".to_owned()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            let mut ids: Vec<ResourceId> = self
                .0
                .keys()
                .filter(|(k, _)| *k == kind)
                .map(|(_, id)| id.clone())
                .collect();
            ids.sort();
            ids
        }

        fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
            self.0
                .get(&(kind, id.clone()))
                .cloned()
                .ok_or_else(|| PackError::NotFound {
                    kind,
                    id: id.clone(),
                })
        }
    }

    /// A `creates-task: false` fixture workflow whose lone step is a `fan-out` over
    /// `{{ milestone.tasks }}` running `workflow:sub-task` — the surface the
    /// milestone-feeding dispatch composes, the M8 idiom for the (not-yet-built, T3)
    /// real `milestone-execution` workflow.
    fn fanout_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: milestone-execution\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "milestone-execution",
                "---\nwhen: fan out over a milestone's sub-tasks\ncreates-task: false\n---\n{{ include: step:implement-tasks }}\n{{ include: step:join-tasks }}\n",
            ),
            (
                PackResourceKind::Steps,
                "implement-tasks",
                "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run:  workflow:sub-task\n---\nSpawn one sub-agent per sub-task.\n",
            ),
            (
                PackResourceKind::Steps,
                "join-tasks",
                "---\njoin: true\n---\nMerge the fanned sub-task areas by task id.\n",
            ),
        ])
    }

    /// The T1 milestone-feeding contract: composing the `creates-task: false`
    /// `milestone-execution` fixture workflow through [`execute_milestone_core`] with a
    /// milestone's id-sorted sub-task list fed into `{{milestone.tasks}}` emits **one**
    /// `` Spawn: `cd <worktree> && jigc workflow sub-task --task <id>` `` per id, **in
    /// id-sorted order**.
    /// The feed is given in NON-id order (zebra before alpha) so the id-sorted emit is
    /// not an accident of feed order — and (Validation hardening #7) the **reversed**
    /// feed emits the byte-identical block, proving the resolver sorts on resolve, not a
    /// caller pre-sort. The emit is read straight off the composed bytes the agent runs
    /// — never a reconstructed equivalent.
    #[test]
    fn milestone_feeding_emits_one_id_sorted_spawn_per_subtask() {
        let pack = fanout_pack();
        let source = PackStepSource { pack: &pack };
        // `repo_root` is unused on the no-task arm (it never reads HEAD); a throwaway
        // path suffices — the compose feeds only the injected milestone list.
        let repo_root = Path::new("/nonexistent-milestone-execute-repo");

        // The id-sorted sub-task list, FED in non-id order.
        let fed = vec!["zebra-fix".to_owned(), "alpha-fix".to_owned()];
        let composed =
            execute_milestone_core(repo_root, &pack, "milestone-execution", &source, &fed)
                .expect("milestone-execution composes over the fed sub-task list");

        // Exactly one Spawn line per sub-task, each `cd`-ing into its own worktree
        // before the bare re-entry workflow + id.
        let alpha =
            "Spawn: `cd .jigc/worktrees/alpha-fix && jigc workflow sub-task --task alpha-fix`";
        let zebra =
            "Spawn: `cd .jigc/worktrees/zebra-fix && jigc workflow sub-task --task zebra-fix`";
        assert!(
            composed.text.contains(alpha) && composed.text.contains(zebra),
            "the fan-out must emit one Spawn directive per sub-task; got:\n{}",
            composed.text,
        );
        assert_eq!(
            composed.text.matches("Spawn: `cd .jigc/worktrees/").count(),
            2,
            "exactly one Spawn per sub-task (no duplicates / extras); got:\n{}",
            composed.text,
        );
        // id-sorted: alpha-fix must precede zebra-fix in the emitted bytes, even though
        // it was fed zebra-then-alpha.
        let alpha_at = composed.text.find(alpha).expect("alpha Spawn present");
        let zebra_at = composed.text.find(zebra).expect("zebra Spawn present");
        assert!(
            alpha_at < zebra_at,
            "the Spawn directives must be id-sorted (alpha before zebra), not in feed order; got:\n{}",
            composed.text,
        );

        // Order-invariance (Validation hardening #7): the REVERSED feed emits the
        // byte-identical composed output — the resolver sorts on resolve.
        let reversed = vec!["alpha-fix".to_owned(), "zebra-fix".to_owned()];
        let composed_rev =
            execute_milestone_core(repo_root, &pack, "milestone-execution", &source, &reversed)
                .expect("the reversed feed composes");
        assert_eq!(
            composed.text, composed_rev.text,
            "the fan-out emit must be byte-identical across divergent feed orders (id-sorted on resolve)",
        );
    }
}
