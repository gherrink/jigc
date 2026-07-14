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
//! `milestones/` via the shared [`crate::gitignore::ensure`] writer.

use crate::cli::Format;
use crate::invocation_log::Outcome;
use crate::pack::make_pack;
use crate::render;
use crate::task::git_head;
use anyhow::{Context, Result, bail};
use engine::file_state::{FileStateRecord, hash_bytes, reconcile_committed};
use engine::finalize::plan_milestone_finalize;
use engine::finding::{Finding, Location, Severity};
use engine::index::{EdgeIndex, load_committed};
use engine::milestone::{
    JoinOutcome, MintedMilestone, add_from_spec, add_task, join, materialize, milestone_dir,
    mint_milestone, read_base_pin, read_task_list, render_fresh_record, synthesized_message,
    worktree_path,
};
use engine::packsource::PackResourceKind;
use engine::schema::Schema;
use engine::state::BasePin;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    /// Abandon the milestone: settle its committed record to the `discarded` terminal
    /// (a genuinely **joined** sub-task stays `joined` — it really did land) in one
    /// record-only commit, then tear the workbench down (the sub-task areas, the
    /// registered fan-out worktrees, and `.jigc/milestones/<id>/`). Refuses when any
    /// sub-task worktree holds uncommitted work, unless `--force`.
    Discard {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// Discard even when a sub-task worktree holds uncommitted work — the explicit
        /// consent to destroy it (without this, a dirty worktree refuses the abandon).
        #[arg(long)]
        force: bool,
    },
}

impl MilestoneCommand {
    /// Dispatch the parsed `milestone` verb against `cwd`, mapping the result to a
    /// process exit code. Success prints a summary on stdout and exits 0; a blocking
    /// finding (serial collision, unknown milestone) surfaces on stderr with its
    /// route and exits non-zero (`design/write-commands.md` → Minting a milestone).
    pub fn dispatch(self, cwd: &Path, format: Format) -> Outcome {
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
            MilestoneCommand::Discard {
                milestone_id,
                force,
            } => run_discard(cwd, &milestone_id, force),
            MilestoneCommand::Execute { .. } => unreachable!("`Execute` is handled above"),
            MilestoneCommand::Join { .. } => unreachable!("`Join` is handled above"),
            MilestoneCommand::Finalize { .. } => unreachable!("`Finalize` is handled above"),
        };
        match result {
            Ok(summary) => {
                println!("{}", render::milestone(format, &summary));
                Outcome::success()
            }
            Err(err) => {
                eprintln!("{}", render::operational_error(format, &err));
                Outcome::failure()
            }
        }
    }
}

/// `jigc milestone create "<title>"` — read HEAD, then mint the milestone area
/// with that single shared base and an empty task list. Ensures `.jigc/.gitignore`
/// lists `milestones/` (the area is disposable runtime state). Returns the summary
/// line; a serial collision surfaces as the engine's routed blocking finding.
///
/// **A milestone's id belongs to its committed record, not to its workbench** (M42
/// completion-audit HIGH; `design/team-ready-state.md` → The lifecycle). [`mint_milestone`]'s
/// collision check reads the gitignored `.jigc/milestones/<id>/` area — which is *legitimately
/// absent* both on a fresh clone of an in-flight milestone and after a terminal op tore it down —
/// so it cannot see that the slug is taken, and the record materialization below would then
/// **overwrite the committed record** of a milestone that was abandoned, or one whose work
/// landed. So `create` refuses **first** when a record already owns the slug
/// ([`guard_record_free`]), before HEAD is read or any area is minted.
fn run_create(cwd: &Path, title: &str) -> Result<String> {
    // The base pin is the *worktree* HEAD; the `.jigc/` area binds to jigc_home (the main
    // checkout), so all worktrees share one `.jigc/` (M31 Inc 2 / WF3).
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    crate::gitignore::ensure(&jigc_root)?;

    // The record-home split (`design/team-ready-state.md` → The `milestone-record` doctype;
    // The commit model): under a `[dev ▸ methodology]` project the composed cascade resolves
    // the methodology-pack `milestone-record` doctype, so materialize the committed record and
    // land a **record-only** path-scoped commit. Dev-only (no methodology pack) resolves no
    // such schema → degrade to today's no-record, no-extra-commit behavior.
    let schemas = shipped_schemas(&jigc_home)?;

    // The id-is-taken guard, ahead of every write: no area minted, no HEAD read, no record
    // overwritten. Inert dev-only (no record home exists to collide with).
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        guard_record_free(&jigc_home, schema, title)?;
    }

    let base = read_head(&repo_root)?;
    let minted = mint_milestone(&jigc_root, title, base).map_err(finding_to_err)?;

    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        // The record's schema-version stamp value: the doctype's manifest version (the
        // same authority `doc create`'s stamp deriver reads — milestone-record is
        // manifest-frozen since M40 A1), falling back to 1 for a manifest-less pack.
        let stamp = crate::pack::frozen_doctype_versions(make_pack()?.as_ref())
            .get(MILESTONE_RECORD_TYPE)
            .copied()
            .unwrap_or(1);
        materialize_and_commit_record(&jigc_home, schema, &minted, stamp)?;
    }

    Ok(format!(
        "minted milestone:{} (shared base {})",
        minted.id, minted.base.short
    ))
}

/// The methodology-pack doctype governing a milestone's committed team-ready state
/// (`design/team-ready-state.md` → The `milestone-record` doctype). Present in the resolved
/// schema set only under a `[dev ▸ methodology]` project; **absent** dev-only (no methodology
/// pack), which is what degrades `create` back to today's no-record behavior. `pub(crate)`
/// for the one cross-verb consumer: `rename`'s unconditional record reslug guard (M40 A4.4).
pub(crate) const MILESTONE_RECORD_TYPE: &str = "milestone-record";

/// Materialize the milestone's committed team-ready `milestone-record` and commit ONLY it —
/// the record-home split (`design/team-ready-state.md` → The commit model: path-scoped commit
/// at each milestone op). Renders the fresh record bytes ([`render_fresh_record`]: `base` +
/// `status: active` + the `schema_version` stamp + an empty `tasks` section), writes them to
/// the record's canonical committed home under docs-root (`docs/milestone-records/<id>.md`,
/// resolved by [`shipped_schemas`]' `apply_docs_root`), then lands a **record-only** commit —
/// never sweeping the agent's in-flight staged/untracked WIP (the M30/M31 path-scoped
/// discipline).
fn materialize_and_commit_record(
    jigc_home: &Path,
    schema: &Schema,
    minted: &MintedMilestone,
    schema_version: u32,
) -> Result<()> {
    let record_path = engine::store::canonical_path(jigc_home, schema, &minted.id)
        .context("the `milestone-record` doctype declares no committed location")?;
    if let Some(parent) = record_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("could not create the record home {parent:?}"))?;
    }
    let body = render_fresh_record(schema, &minted.id, &minted.base, schema_version);
    std::fs::write(&record_path, &body)
        .with_context(|| format!("could not write the milestone record {record_path:?}"))?;

    // The message temp file lands in the gitignored milestone area (never a tracked path).
    commit_record_only(
        jigc_home,
        &record_path,
        &minted.dir,
        &format!(
            "chore(milestone): open record for milestone:{}\n",
            minted.id
        ),
    )?;
    // Seed the record's `file-state` baseline to what `create` just wrote, so the first
    // `add-task`'s reconcile preflight compares against the CLI's own write (T6).
    baseline_record(
        &jigc_home.join(".jigc"),
        schema,
        &minted.id,
        body.as_bytes(),
    );
    Ok(())
}

/// Land a **record-only** commit for a milestone op (`design/team-ready-state.md` → The commit
/// model). Stages ONLY the record (`git add -- <record>`, index-restricted) and commits ONLY
/// that pathspec (`git commit -F <msg> -- <record>`), so any other staged/untracked change
/// stays out of the record commit and stays staged — the WIP-safety the whole-index
/// [`crate::task::git_commit`] cannot give, hence its narrowed sibling
/// [`git_commit_pathspec`]. The caller-supplied `message` is a CLI-synthesized structural
/// line (a record carries no authored prose) and is written into the gitignored `msg_dir`
/// (the milestone area). Reused by every per-op record commit (`create` opens, `add-task`
/// appends), each passing its own structural subject.
fn commit_record_only(
    repo_root: &Path,
    record_path: &Path,
    msg_dir: &Path,
    message: &str,
) -> Result<()> {
    let spec = record_path
        .strip_prefix(repo_root)
        .unwrap_or(record_path)
        .to_str()
        .with_context(|| format!("record path {record_path:?} is not valid UTF-8"))?;
    // Stage ONLY the record (a `git add -- <path>` never sweeps the ambient dirty tree).
    crate::task::git_run(repo_root, &["add", "--", spec])?;

    let msg_path = msg_dir.join("record-commit-msg.txt");
    std::fs::write(&msg_path, message)
        .with_context(|| format!("could not write the record commit message {msg_path:?}"))?;
    git_commit_pathspec(repo_root, &msg_path, spec)
}

/// `git commit -F <message_file> -- <pathspec>` in `repo_root` — a **pathspec-restricted**
/// commit recording ONLY the listed path, leaving any other staged/untracked change untouched
/// (the WIP-safe milestone-op commit). [`crate::task::git_commit`] commits the whole index, so
/// the record path needs this narrowed sibling. Bails with git's stdout+stderr on a non-zero
/// exit (nothing was committed).
fn git_commit_pathspec(repo_root: &Path, message_file: &Path, pathspec: &str) -> Result<()> {
    let out = Command::new("git")
        .arg("commit")
        .arg("-F")
        .arg(message_file)
        .arg("--")
        .arg(pathspec)
        .current_dir(repo_root)
        .output()
        .context("could not run `git commit` (is git on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git commit` (record-only) was rejected (no commit was made):\n{}{}",
            String::from_utf8_lossy(&out.stdout).trim(),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    Ok(())
}

/// The committed record's **file-state key** — the repo-relative `<location><id>.md` path
/// [`reconcile_committed_store`](engine::file_state::reconcile_committed_store) keys the
/// baseline hash under (`format!("{location}{slug}.md")`). The `location:` here is already
/// docs-root-nested by [`shipped_schemas`] (`docs/milestone-records/`), so the key matches
/// the store-sweep's exactly — the same string the reconcile finding names.
fn record_key(schema: &Schema, milestone_id: &str) -> Option<String> {
    let location = schema.location.as_deref()?;
    Some(format!("{location}{milestone_id}.md"))
}

/// Advance the committed record's `file-state` baseline to `bytes` — recorded at each
/// milestone-op record WRITE (`create` materialize, `add-task` append) so the *next*
/// overwrite's [`reconcile_record_preflight`] compares against what the CLI last wrote, not a
/// stale hash (else a legitimate sequential op would false-drift against its own prior
/// append). The engine stays shell-free; the CLI persists the `.jigc/state/file-state.json`
/// baseline it owns. A record with no `location:` (never this doctype) or an unreadable
/// baseline store degrades to no-op — the guard then treats the next op as first-encounter
/// (baseline-adopt, no block), never a spurious error.
fn baseline_record(jigc_root: &Path, schema: &Schema, milestone_id: &str, bytes: &[u8]) {
    let Some(key) = record_key(schema, milestone_id) else {
        return;
    };
    let Ok(mut record) = FileStateRecord::load(jigc_root) else {
        return;
    };
    record.record(key, hash_bytes(bytes));
    let _ = record.save(jigc_root);
}

/// The **reconcile preflight** before a `set: on-transition` record overwrite — the
/// No-silent-overwrite discipline (`design/team-ready-state.md` → F3;
/// `design/reconciliation.md`). Because the `milestone-record` is machine-owned, an OOB human
/// edit to a machine-set field cannot be *merged*; it must **conflict-block**, not be silently
/// clobbered. So every overwrite site (`add-task` append, finalize status-flip) runs this
/// first and **routes a blocking finding** if the committed record drifted since the CLI last
/// wrote it, leaving the record untouched.
///
/// The CLI op IS the machine touching the record, so this drives the per-doc reconcile
/// primitive [`reconcile_committed`] with **`task_touched: true`** — the exact
/// `DRIFTED + TOUCHED → conflict-block` arm F3 mandates ("detected + conflict-blocked, **not
/// absorbed**"). The whole-store [`reconcile_committed_store`](engine::file_state::reconcile_committed_store)
/// wrapper cannot express this: it would *absorb* a conformant OOB edit (`DRIFTED + UNTOUCHED`)
/// — the silent clobber F3 forbids — and would sweep sibling records / unrelated committed
/// docs the milestone op has no business gating on. The per-doc primitive is scoped exactly to
/// this record (`DECISIONS.md` 2026-07-07 M39 T6). No edge-index mutation reaches disk — the
/// only mutator (absorb) is unreachable under `task_touched: true` — so a throwaway index
/// suffices; a first-encounter baseline-adopt is persisted so detection binds on the next op.
///
/// Inert where there is nothing to guard: dev-only (no `milestone-record` schema) never calls
/// this, and a record file absent on disk (nothing to overwrite) is a no-op.
fn reconcile_record_preflight(
    jigc_home: &Path,
    jigc_root: &Path,
    schema: &Schema,
    milestone_id: &str,
) -> Result<()> {
    let Some(record_path) = engine::store::canonical_path(jigc_home, schema, milestone_id) else {
        return Ok(());
    };
    let Ok(bytes) = std::fs::read(&record_path) else {
        return Ok(()); // no committed record yet → nothing to overwrite, nothing to guard.
    };
    let Some(key) = record_key(schema, milestone_id) else {
        return Ok(());
    };
    let from = format!("{MILESTONE_RECORD_TYPE}:{milestone_id}");

    let mut fs_record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;
    // No edge is ever written: absorb (the sole index mutator) is unreachable with
    // `task_touched: true`, so a throwaway index is never persisted.
    let mut index = EdgeIndex {
        stamp: String::new(),
        edges: Vec::new(),
    };
    let findings = reconcile_committed(
        &mut fs_record,
        &mut index,
        schema,
        &key,
        &from,
        &bytes,
        /* task_touched = */ true,
    );
    if let Some(blocking) = findings.iter().find(|f| f.severity == Severity::Blocking) {
        // Drift on a machine-owned record → conflict-block, the record untouched, routed.
        return Err(finding_to_err(blocking.clone()));
    }
    // A first-encounter baseline-adopt mutated the record in memory; persist it so the next
    // op's preflight has a baseline to detect drift against (a clean IN_SYNC yields no finding
    // and no mutation, so this skips the write there).
    if !findings.is_empty() {
        fs_record.save(jigc_root).with_context(|| {
            format!("could not persist the file-state baseline under {jigc_root:?}")
        })?;
    }
    Ok(())
}

/// Whether `base..head` is a linear-ancestor range whose **every** commit touches ONLY
/// milestone-record paths — the CLI half of the finalize base-guard refinement
/// (`design/team-ready-state.md` → The commit model: the finalize base-guard refinement, and
/// → What the base-guard is for). The engine does no git I/O, so the CLI computes the verdict
/// and feeds it to [`plan_milestone_finalize`]: (1) `base` must be a linear ancestor of `head`
/// (else genuine divergence — a rebase/rewrite — keeps the base-mismatch block); (2) every path
/// touched by any commit in `base..head` must be a milestone-record path (a foreign,
/// **code-or-doc** commit fails this, so external drift that would invalidate the M31
/// worktree-combine still blocks). Absent the `milestone-record` schema (dev-only, no
/// methodology pack) there is no committed record path, so the range is never treated as
/// record-only.
///
/// The predicate is **any** milestone's record path, not just this one's (M42 T2): what the
/// guard exists to prove is that **no code moved**, and another milestone's record is not code —
/// it cannot invalidate this milestone's combine base. Keyed on the *record path* alone, a
/// second live milestone's ordinary record-only bookkeeping (`create` / `add-task` — commits
/// that move nothing else) wedged the first at `finalize.base-mismatch`, and interleaved
/// creation wedged **both**, symmetrically and unrecoverably (even `git revert` fails: the
/// revert commit itself touches the other record). `milestone discard` depends on this too —
/// its record-settling commit is a *foreign* record path inside a concurrent milestone's range.
fn record_only_range(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    base_sha: &str,
    head_sha: &str,
) -> Result<bool> {
    let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) else {
        return Ok(false);
    };
    // The record doctype's `location:` is repo-relative and already docs-root-nested by
    // [`shipped_schemas`] (`docs/milestone-records/`) — the same prefix [`record_key`] builds a
    // record's file-state key on. A milestone-record path is a direct `<slug>.md` child of it
    // (exactly what [`engine::store::canonical_path`] mints), so nothing else parked under that
    // directory passes as bookkeeping.
    let Some(location) = schema.location.as_deref() else {
        return Ok(false);
    };
    let record_dir = location.trim_end_matches('/');
    let is_record_path = |p: &str| {
        p.strip_prefix(record_dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .is_some_and(|name| !name.contains('/') && name.ends_with(".md"))
    };

    // (1) `base` must be a linear ancestor of `head` (else genuine divergence).
    if !git_is_ancestor(repo_root, base_sha, head_sha)? {
        return Ok(false);
    }
    // (2) every path touched by any commit in `base..head` must be a record path. `git log
    // --name-only` lists the per-commit touched paths (a path touched then reverted still
    // appears), so the union ⊆ {records} iff every commit's fileset ⊆ {records}. `--no-renames`
    // pins the fact against the user's `diff.renames` config (the `git_changed_paths` precedent).
    let touched = crate::task::git_capture(
        repo_root,
        &[
            "log",
            "--format=",
            "--name-only",
            "--no-renames",
            &format!("{base_sha}..{head_sha}"),
        ],
    )?;
    let all_record = touched
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .all(is_record_path);
    Ok(all_record)
}

/// Whether `ancestor` is a linear ancestor of `descendant` (`git merge-base --is-ancestor`:
/// exit 0 = ancestor, exit 1 = not, other = error). The linearity half of the finalize
/// base-guard refinement — a non-ancestor base is genuine divergence and keeps the
/// base-mismatch block. No existing helper carries the exit-1-is-not-an-error semantics, so
/// it shells out directly (the [`git_commit_pathspec`] precedent).
fn git_is_ancestor(repo_root: &Path, ancestor: &str, descendant: &str) -> Result<bool> {
    let status = Command::new("git")
        .args(["merge-base", "--is-ancestor", ancestor, descendant])
        .current_dir(repo_root)
        .status()
        .context("could not run `git merge-base` (is git on PATH?)")?;
    match status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        other => bail!("`git merge-base --is-ancestor {ancestor} {descendant}` failed ({other:?})"),
    }
}

/// Whether the milestone's recorded base-SHA `sha` **no longer names a commit** in
/// `repo_root`'s object store — the stale-base detection (`design/team-ready-state.md` →
/// Stale-base edge (history rewrite)). A history rewrite can orphan the pinned base;
/// `git rev-parse --verify --quiet <sha>^{commit}` exits 0 when the sha resolves to a commit
/// and non-zero when it does not (a rewritten-away / fabricated sha). The zero-commit
/// sentinel ([`crate::task::EMPTY_TREE_SHA`]) is a valid pre-first-commit base, never a stale
/// pin, so it is exempt. The engine does no git I/O, so this CLI probe feeds the engine's
/// [`engine::milestone::stale_base_finding`] shape.
fn base_commit_missing(repo_root: &Path, sha: &str) -> Result<bool> {
    if sha == crate::task::EMPTY_TREE_SHA {
        return Ok(false);
    }
    let out = Command::new("git")
        .args(["rev-parse", "--verify", "--quiet"])
        .arg(format!("{sha}^{{commit}}"))
        .current_dir(repo_root)
        .output()
        .context("could not run `git rev-parse` (is git on PATH?)")?;
    Ok(!out.status.success())
}

/// Guard a milestone op against a **stale base pin** (`design/team-ready-state.md` →
/// Stale-base edge). If the milestone's recorded base SHA no longer names a commit — a
/// history rewrite orphaned it — route the human with the engine's blocking
/// [`stale_base_finding`](engine::milestone::stale_base_finding) and abort **before** any
/// git operation against the missing commit (a partial worktree provisioning, a corrupt
/// worktree-combine), never silent corruption. Called by every base-reading op
/// (`provision`/`join`/`finalize`) right after it reads the base pin. Inert when the base
/// still resolves (the common case) and for the zero-commit sentinel.
fn guard_base_live(repo_root: &Path, milestone_id: &str, base: &BasePin) -> Result<()> {
    if base_commit_missing(repo_root, &base.sha)? {
        return Err(finding_to_err(engine::milestone::stale_base_finding(
            milestone_id,
            &base.short,
        )));
    }
    Ok(())
}

/// `jigc milestone add-task <milestone-id> "<intent>"` — mint a sub-task pinned to
/// the milestone's shared base in its own isolated area and append it. Returns the
/// summary line; an unknown milestone or a within-milestone collision surfaces as
/// the engine's routed blocking finding.
fn run_add_task(cwd: &Path, milestone_id: &str, intent: &str, workflow: &str) -> Result<String> {
    // The `.jigc/` area binds to jigc_home (the main checkout); no git read here.
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    let schemas = shipped_schemas(&jigc_home)?;

    // Reconcile preflight (T6): before ANY mutation (the cache append below or the record
    // overwrite), conflict-block if the committed record drifted out-of-band, leaving both the
    // cache and the record untouched (`design/team-ready-state.md` → F3). Inert dev-only.
    //
    // **Ahead of the reseed's terminal guard, on purpose** (`design/team-ready-state.md` → The
    // lifecycle): drift is diagnosed BEFORE the drifted content is interpreted. A hand edit to
    // the machine-owned `status` leaf can write `joined`/`discarded` into the record, and reading
    // the lifecycle off tampered bytes would tell the operator *"this milestone is over"* about a
    // milestone nobody joined or abandoned — a route that lies, from the wave that exists to stop
    // routes lying. The drift block names the real cause and routes to reconcile.
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        reconcile_record_preflight(&jigc_home, &jigc_root, schema, milestone_id)?;
    }

    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the engine reads through it, so `add-task` on a fresh clone (no `.jigc/` WIP) continues the
    // milestone (`design/team-ready-state.md` → Engine capability 2 (read-back)). No-op once the
    // cache exists / dev-only (no record). Refuses a settled record (the terminal guard).
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

    let added = add_task(&jigc_root, milestone_id, intent, workflow).map_err(finding_to_err)?;

    // The record-home split (`design/team-ready-state.md` → Engine capability 1 (write), the
    // `add-task` — append arm; The commit model): under a `[dev ▸ methodology]` project the
    // composed cascade resolves the `milestone-record` doctype, so append the sub-task to the
    // committed record and land a **separate record-only** path-scoped commit (the JSON-cache
    // append above is retained as the demoted cache). Dev-only (no methodology pack) resolves
    // no such schema → degrade to today's no-record, no-extra-commit behavior.
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        append_and_commit_record(
            &jigc_home,
            &jigc_root,
            schema,
            milestone_id,
            &added.task.id,
            intent,
        )?;
    }

    Ok(format!(
        "added task:{} to milestone:{}",
        added.task.id, added.milestone_id
    ))
}

/// Append one sub-task to the committed `milestone-record` and commit ONLY it — the
/// `add-task` append arm of the record-home split (`design/team-ready-state.md` → Engine
/// capability 1 (write); The commit model: a **separate** record-only path-scoped commit per
/// `add-task`). Reads the committed record source at its canonical home under docs-root,
/// appends one `tasks` item (`task-id`/`intent`/`status: active`) via the byte-stable
/// [`engine::milestone::append_task_item`] primitive, writes it back, then lands a
/// record-only commit through the shared [`commit_record_only`] helper — never sweeping the
/// agent's in-flight staged/untracked WIP (the M30/M31 path-scoped discipline). A failed
/// append (a malformed record, a duplicate id) surfaces the engine's routed blocking finding.
fn append_and_commit_record(
    jigc_home: &Path,
    jigc_root: &Path,
    schema: &Schema,
    milestone_id: &str,
    task_id: &str,
    intent: &str,
) -> Result<()> {
    let record_path = engine::store::canonical_path(jigc_home, schema, milestone_id)
        .context("the `milestone-record` doctype declares no committed location")?;
    let source = std::fs::read_to_string(&record_path)
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;

    let appended = engine::milestone::append_task_item(schema, &source, task_id, intent)
        .map_err(|err| finding_to_err(engine::write::generate_error_finding(&err)))?;
    std::fs::write(&record_path, &appended)
        .with_context(|| format!("could not write the milestone record {record_path:?}"))?;

    // The message temp file lands in the gitignored milestone WIP area (never a tracked path).
    let msg_dir = milestone_dir(jigc_root, milestone_id);
    commit_record_only(
        jigc_home,
        &record_path,
        &msg_dir,
        &format!("chore(milestone): record task:{task_id} on milestone:{milestone_id}\n"),
    )?;
    // Advance the record's `file-state` baseline to the appended bytes, so the next overwrite's
    // reconcile preflight compares against this write, not the pre-append record (T6).
    baseline_record(jigc_root, schema, milestone_id, appended.as_bytes());
    Ok(())
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

    // Reconcile preflight (T6): before any mutation, conflict-block if the committed record
    // drifted out-of-band, leaving both the cache and the record untouched
    // (`design/team-ready-state.md` → F3). Inert dev-only. Ahead of the reseed's terminal guard —
    // drift is diagnosed before the drifted content is interpreted (the `add-task` note).
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        reconcile_record_preflight(&jigc_home, &jigc_root, schema, milestone_id)?;
    }

    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the mint reads through it, so `add-from-spec` on a fresh clone seeds onto the continued
    // milestone (`design/team-ready-state.md` → Engine capability 2 (read-back)). No-op once the
    // cache exists / dev-only (no record). Refuses a settled record (the terminal guard).
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

    let added = add_from_spec(
        &jigc_root,
        &jigc_home,
        &schemas,
        milestone_id,
        spec_addr,
        workflow,
    )
    .map_err(finding_to_err)?;

    // The record-home split (`design/team-ready-state.md` → Engine capability 1 (write), the
    // `add-task` append arm; The commit model): under a `[dev ▸ methodology]` project the composed
    // cascade resolves the `milestone-record` doctype, so each spec-seeded sub-task must ALSO land
    // in the committed record — exactly like `add-task` — else the sub-tasks are silently lost on
    // a fresh clone (the source-of-truth invariant). One separate record-only path-scoped commit
    // per seeded sub-task; the intent is read back from the minted task's working area (the
    // verbatim criterion text `add_from_spec` persisted). Dev-only resolves no schema → no record.
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        for a in &added {
            let intent = engine::state::read_intent(&a.task.dir).with_context(|| {
                format!(
                    "could not read the seeded sub-task intent for `{}`",
                    a.task.id
                )
            })?;
            append_and_commit_record(
                &jigc_home,
                &jigc_root,
                schema,
                milestone_id,
                &a.task.id,
                &intent,
            )?;
        }
    }

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
    let pack = make_pack()?;
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

/// **Re-derive the demoted `.jigc` milestone cache from the committed record when absent** —
/// the fresh-clone resume read-path arm (`design/team-ready-state.md` → Engine capability 2
/// (read-back): "on a fresh clone (no `.jigc/` working state) the first milestone op parses the
/// record back into `BasePin` + `TaskList` and re-seeds the cache"; "Continue" means resume, not
/// WIP recovery; M39 T5). Every milestone-op **cache reader** (`add-task`/`provision`/`join`/
/// `finalize`/`list-tasks`/`execute`) calls this before it reads through the demoted cache
/// (`read_base_pin`/`read_task_list`) or its `dir.is_dir()` unknown-milestone guard, so a
/// teammate on a fresh clone (the gitignored `.jigc/milestones/<id>/` WIP gone, the committed
/// record present) re-derives the milestone shape from the source-of-truth record and resumes
/// its un-joined sub-tasks from scratch.
///
/// Gated twice so it stays inert where there is nothing to re-derive: (1) the `milestone-record`
/// schema must be resolved (a `[dev ▸ methodology]` project) — dev-only (no methodology pack) has
/// no committed record, so the demoted JSON cache is the only home and this no-ops; and (2) the
/// committed record file must exist at its canonical home — a genuinely-unknown milestone has no
/// record, so this no-ops and the caller's existing unknown-milestone guard still fires. The
/// engine reseed ([`engine::milestone::reseed_cache_from_record`]) is itself a no-op when both
/// cache files are already present (the live session's cache stays authoritative), so calling
/// this on every op is cheap and side-effect-free once the cache exists.
///
/// **It is also the terminal guard** (M42 completion-audit HIGH; `design/team-ready-state.md` →
/// The lifecycle). Because *every* milestone verb calls this before its own guards, the engine's
/// *"a record in a terminal state does not re-seed a workbench"* refusal is inherited by all of
/// them at one site: a `discarded` or `joined` milestone is over, and `add-task` / `add-from-spec`
/// / `list-tasks` / `provision` / `execute` / `join` / `finalize` / `discard` all block here with
/// `milestone.terminal`, routed to the committed record's read surface. Before that refusal
/// existed, this function was the **resurrection**: it rebuilt the workbench that `discard`'s
/// teardown (and `finalize`'s) had just removed, straight out of the settled record.
/// **Refuse a `create` whose id a committed record already owns** — the identity half of the
/// terminal predicate (M42 completion-audit HIGH; `design/team-ready-state.md` → The lifecycle).
/// Resolves the record home of the **exact id the mint will produce**
/// ([`engine::milestone::mint_id`] — never a second slug derivation) and blocks when a record
/// already lives there, naming its status so the route can distinguish a milestone to *continue*
/// from one that is *over* ([`engine::milestone::record_exists_finding`]).
///
/// Called by `create` only. Every other verb targets an id that must **already** exist, so their
/// guard is the opposite one ([`reseed_cache`]).
fn guard_record_free(jigc_home: &Path, schema: &Schema, title: &str) -> Result<()> {
    let milestone_id = engine::milestone::mint_id(title);
    let Some(record_path) = engine::store::canonical_path(jigc_home, schema, &milestone_id) else {
        return Ok(());
    };
    if !record_path.exists() {
        return Ok(());
    }
    // The status is decoration on the block (the *existence* is the refusal), so an unreadable
    // record degrades to a status-less message rather than masking the collision.
    let status = std::fs::read_to_string(&record_path)
        .ok()
        .and_then(|source| engine::milestone::record_status(schema, &source));
    Err(finding_to_err(engine::milestone::record_exists_finding(
        &milestone_id,
        status.as_deref(),
    )))
}

fn reseed_cache(
    jigc_home: &Path,
    jigc_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    milestone_id: &str,
) -> Result<()> {
    let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) else {
        return Ok(());
    };
    let Some(record_path) = engine::store::canonical_path(jigc_home, schema, milestone_id) else {
        return Ok(());
    };
    if !record_path.exists() {
        return Ok(());
    }
    let source = std::fs::read_to_string(&record_path)
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;
    let dir = milestone_dir(jigc_root, milestone_id);
    engine::milestone::reseed_cache_from_record(&dir, schema, &source).map_err(finding_to_err)
}

/// `jigc milestone list-tasks <milestone-id>` — read the milestone's persisted
/// task list and emit its sub-task ids in **canonical id-sorted order** (the
/// deterministic order the by-task-id join enumerates, surfaced through the
/// binary, not just the internal enumerate fn). Returns the summary line; an
/// unknown milestone (no area / unreadable list) surfaces as a context-wrapped
/// error and exits non-zero.
fn run_list_tasks(cwd: &Path, milestone_id: &str) -> Result<String> {
    // The `.jigc/` area binds to jigc_home (the main checkout); no git read here.
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the `dir.is_dir()` guard, so `list-tasks` on a fresh clone re-derives the milestone shape
    // and continues (`design/team-ready-state.md` → Engine capability 2 (read-back)).
    let schemas = shipped_schemas(&jigc_home)?;
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;
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
    crate::gitignore::ensure(&jigc_root)?;

    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the `dir.is_dir()` guard + base-pin/task-list reads, so `provision` on a fresh clone
    // re-derives the milestone shape (`design/team-ready-state.md` → Engine capability 2).
    let schemas = shipped_schemas(&jigc_home)?;
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

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
    // Stale-base edge (T7): a rewritten-away base pin routes the human BEFORE any
    // `git worktree add` against the missing commit (else a partial, half-provisioned
    // worktree set) (`design/team-ready-state.md` → Stale-base edge).
    guard_base_live(&repo_root, milestone_id, &base)?;
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

/// The id-ordered paths of the milestone's still-provisioned fan-out worktrees — each
/// sub-task whose `<jigc_home>/.jigc/worktrees/<id>` is a currently-registered git
/// worktree (the [`remove_worktrees`] enumeration, reused for the combine channel). A
/// never-provisioned (docs-only) milestone yields an empty list, so the `squash: true`
/// combine degrades to a docs-only commit. Best-effort on the `git worktree list` read (an
/// unreadable list yields no worktrees — the combine then commits the docs alone, never a
/// spurious block).
fn provisioned_worktrees(
    repo_root: &Path,
    jigc_home: &Path,
    list: &engine::milestone::TaskList,
) -> Vec<PathBuf> {
    let registered = registered_worktrees(repo_root).unwrap_or_default();
    // Match `provision_worktrees`' canonical-path convention (git stores canonical paths at
    // `add` time); fall back to the raw path if canonicalization fails (then nothing
    // matches and the worktree is treated as not-provisioned).
    let canonical_home = jigc_home
        .canonicalize()
        .unwrap_or_else(|_| jigc_home.to_path_buf());
    list.enumerate()
        .into_iter()
        .map(|id| canonical_home.join(worktree_path(&id)))
        .filter(|path| registered.iter().any(|w| w == path))
        .collect()
}

/// Whether any of the milestone's provisioned worktrees has staged code — Σ `git diff
/// --cached --name-only` over the worktree list (the narrowed empty-commit signal for the
/// worktree-isolation model, M31 Inc 4). An empty list (a docs-only milestone) yields
/// `false`, so the empty-commit guard then rests on the materialized-docs signal alone.
fn worktrees_have_staged_code(worktrees: &[PathBuf]) -> Result<bool> {
    for wt in worktrees {
        let out = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .current_dir(wt)
            .output()
            .context("could not run `git` (is it on PATH?)")?;
        if !out.status.success() {
            bail!(
                "`git diff --cached` in worktree {wt:?} failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        if !String::from_utf8_lossy(&out.stdout).trim().is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
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

/// `jigc milestone discard <milestone-id> [--force]` — the milestone family's terminal verb
/// (`design/team-ready-state.md` → `jigc milestone discard <id>`; `design/write-commands.md` →
/// Abandoning a milestone). The milestone is being **abandoned**: settle its committed record to
/// the `discarded` terminal and tear the workbench down.
///
/// The op order is the settled one: (1) the **reconcile preflight** — the record settle is a
/// `set: on-transition` overwrite, so an out-of-band edit conflict-blocks rather than being
/// silently clobbered ([`reconcile_record_preflight`]); (2) the **dirty-worktree guard** — the
/// teardown's `git worktree remove --force` is safe at *finalize* (the commit lands first) but on
/// the abandon path the sub-agents' work is **by definition uncommitted**, so a dirty worktree
/// refuses the abandon unless `--force` names the consent to destroy it; (3) the record settle
/// ([`engine::milestone::discard_record`] — a genuinely **joined** sub-task stays `joined`);
/// (4) a **record-only** commit carrying a CLI-synthesized structural subject (never sweeping the
/// agent's in-flight WIP); (5) the teardown — the sub-task areas, the registered fan-out
/// worktrees, **and** `.jigc/milestones/<id>/` (which no verb removed before: teardown was
/// reachable only from the landed-finalize path, so an abandoned milestone left a cache
/// `provision` would happily re-provision worktrees from).
///
/// Dev-only (no methodology pack → no `milestone-record` schema) degrades exactly as every other
/// record arm does: no record to settle, no commit — and the workbench teardown still runs.
fn run_discard(cwd: &Path, milestone_id: &str, force: bool) -> Result<String> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    let schemas = shipped_schemas(&jigc_home)?;

    // (1) The reconcile preflight — a drifted committed record conflict-blocks before ANY
    // mutation, leaving the record and the workbench untouched (`design/team-ready-state.md` → F3).
    // Inert dev-only. Ahead of the reseed's terminal guard — drift is diagnosed before the drifted
    // content is interpreted (the `add-task` note).
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        reconcile_record_preflight(&jigc_home, &jigc_root, schema, milestone_id)?;
    }

    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the `dir.is_dir()` guard + task-list read, so a teammate on a fresh clone can abandon a
    // milestone whose workbench they never had (`design/team-ready-state.md` → Engine capability 2).
    // An **already-settled** record refuses here (the terminal guard): a milestone that is over is
    // not abandoned twice, and re-running the settle would land an empty record-only commit.
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

    let dir = milestone_dir(&jigc_root, milestone_id);
    if !dir.is_dir() {
        bail!(
            "milestone `{milestone_id}` does not exist\n  route: check the milestone id (`jigc milestone list-tasks <id>` names a live milestone's sub-tasks); nothing was discarded"
        );
    }
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;

    // (2) The dirty-worktree guard — the abandon path's WIP safety
    // (`design/team-ready-state.md` → Abandon refuses on a dirty worktree). The teardown's
    // [`remove_worktrees`] runs `git worktree remove --force`, which is safe at *finalize* (the
    // commit lands first, so every byte the worktree held is already in git) and **destroys
    // uncommitted work** here, where the sub-agents' work is by definition uncommitted. So a
    // dirty worktree REFUSES the abandon, naming the paths; `--force` is the human's explicit
    // consent to destroy the work — on the one path whose premise is "throw this away", that
    // intent is exactly what must be confirmed rather than assumed.
    if !force {
        let worktrees = provisioned_worktrees(&repo_root, &jigc_home, &list);
        let dirty = dirty_worktrees(&worktrees)?;
        if !dirty.is_empty() {
            return Err(finding_to_err(dirty_worktree_finding(milestone_id, &dirty)));
        }
    }

    // (3) Settle the committed record + (4) commit ONLY it. Dev-only resolves no schema → no
    // record, no commit (the omitting context).
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        let record_path = engine::store::canonical_path(&jigc_home, schema, milestone_id)
            .context("the `milestone-record` doctype declares no committed location")?;
        let settled = engine::milestone::discard_record(&record_path, schema, milestone_id)
            .map_err(finding_to_err)?;
        // The message temp file lands in the (gitignored) milestone area — removed by the
        // teardown below, so it is written before the area goes.
        commit_record_only(
            &jigc_home,
            &record_path,
            &dir,
            &format!("chore(milestone): discard record for milestone:{milestone_id}\n"),
        )?;
        // Advance the record's file-state baseline to the settled bytes (the [`baseline_record`]
        // discipline every record write follows), so a later store sweep sees no drift.
        baseline_record(&jigc_root, schema, milestone_id, settled.as_bytes());
    }

    // (5) Teardown — the workbench outlives nothing: the sub-task areas, the registered fan-out
    // worktrees, and the milestone area itself.
    cleanup_subtask_areas(&jigc_root, &list);
    remove_worktrees(&repo_root, &jigc_home, &list);
    remove_milestone_area(&dir);

    Ok(format!(
        "discarded milestone:{milestone_id} ({} sub-task(s); workbench removed)",
        list.enumerate().len()
    ))
}

/// The milestone's provisioned worktrees that hold **uncommitted work**, each paired with the
/// `git status --porcelain` entries that make it dirty — the abandon path's WIP probe
/// (`design/team-ready-state.md` → Abandon refuses on a dirty worktree).
///
/// **`--porcelain`, not `git diff --cached`.** Its sibling [`worktrees_have_staged_code`] reads
/// only the *staged* set, because at `finalize` the staged set is what the combine commits — but
/// what the abandon path destroys is **everything** in the worktree: staged, unstaged, and
/// **untracked** alike (`git worktree remove --force` deletes the checkout). So the guard's probe
/// is the union `git status --porcelain` reports; an ignored file is not work and never appears.
fn dirty_worktrees(worktrees: &[PathBuf]) -> Result<Vec<(PathBuf, Vec<String>)>> {
    let mut dirty = Vec::new();
    for wt in worktrees {
        let out = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(wt)
            .output()
            .context("could not run `git status` (is git on PATH?)")?;
        if !out.status.success() {
            bail!(
                "`git status --porcelain` in worktree {wt:?} failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        let entries: Vec<String> = String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        if !entries.is_empty() {
            dirty.push((wt.clone(), entries));
        }
    }
    Ok(dirty)
}

/// A blocking, route-bearing finding naming every dirty sub-task worktree and the uncommitted
/// entries inside it — the abandon's refusal. The route names both honest exits: get the work out
/// (commit / stash / copy it), or re-run with `--force` to say the work is genuinely being thrown
/// away (the [`crate::combine::detect_code_collision`] finding idiom).
fn dirty_worktree_finding(milestone_id: &str, dirty: &[(PathBuf, Vec<String>)]) -> Finding {
    let listing: Vec<String> = dirty
        .iter()
        .map(|(path, entries)| format!("  {}: {}", path.display(), entries.join(", ")))
        .collect();
    let address = dirty[0].0.display().to_string();
    Finding::graded(
        Severity::Blocking,
        "milestone.dirty-worktree",
        format!(
            "milestone:{milestone_id} has uncommitted work in {} sub-task worktree(s) — \
             discarding it would destroy that work:\n{}",
            dirty.len(),
            listing.join("\n"),
        ),
        Some(Location::addressed(&address, 1, 1)),
        Some(format!(
            "get the work out of those worktrees first (commit, stash, or copy it), then re-run \
             `jigc milestone discard {milestone_id}` — or re-run with `--force` to abandon the \
             milestone and destroy the uncommitted work"
        )),
    )
}

/// Remove the milestone's gitignored workbench `.jigc/milestones/<id>/` on a settled discard —
/// the base pin + task-list cache the record no longer stands behind (`design/team-ready-state.md`
/// → The workbench is actually removed). The **only** remover outside the landed-finalize
/// executor: without it an abandoned milestone leaves a cache `provision` would re-provision
/// worktrees from. Best-effort, logged-not-raised — the record commit has already landed, so a
/// cleanup failure must not fail it (the [`cleanup_subtask_areas`] self-heal stance).
fn remove_milestone_area(dir: &Path) {
    if dir.exists()
        && let Err(err) = std::fs::remove_dir_all(dir)
    {
        eprintln!("note: milestone workbench removal at {dir:?} failed (self-heals): {err:#}");
    }
}

/// Dispatch `jigc milestone execute <milestone-id>`: compose the milestone-execution
/// workflow over the milestone's id-sorted sub-task list, render the composed view on
/// stdout, and map it to the exit code. An unknown milestone (no area) or a blocking
/// compose finding routes to stderr **before** any output and exits non-zero
/// (`design/write-commands.md` → Executing the milestone).
fn dispatch_execute(cwd: &Path, format: Format, milestone_id: &str) -> Outcome {
    match run_execute(cwd, milestone_id) {
        Ok(view) => {
            println!("{}", render::composed(format, &view));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
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
fn run_execute(cwd: &Path, milestone_id: &str) -> Result<crate::start::Composition> {
    // The `.jigc/` area binds to jigc_home (the main checkout); the compose feed resolves
    // the same split internally (it derives jigc_home from the worktree `repo_root`).
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the `dir.is_dir()` guard + task-list read, so `execute` on a fresh clone re-derives the
    // milestone shape and composes the fan-out (`design/team-ready-state.md` → Engine capability 2).
    let schemas = shipped_schemas(&jigc_home)?;
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;
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
fn dispatch_join(cwd: &Path, format: Format, milestone_id: &str) -> Outcome {
    let outcome = match run_join(cwd, milestone_id) {
        Ok(outcome) => outcome,
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            return Outcome::failure();
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
        Outcome::success()
    } else {
        let mut finding_codes = Vec::with_capacity(blocking.len());
        for finding in blocking {
            eprintln!("{}", finding.message);
            if let Some(route) = &finding.route {
                eprintln!("  route: {route}");
            }
            finding_codes.push(finding.code.clone());
        }
        Outcome {
            code: 1,
            finding_codes,
            error_code: None,
        }
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

    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the base-pin read + the engine `join`'s internal cache reads, so `join` on a fresh clone
    // re-derives the milestone shape (`design/team-ready-state.md` → Engine capability 2).
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

    // The committed edge index is keyed to the milestone's shared base — the commit
    // every sub-task inherited — so the cross-area ref walk resolves against the store
    // as it stood at that base. An unknown milestone has no base pin; fall back to the
    // engine's routed `milestone.unknown` block by leaving the join to detect it.
    let head = match read_base_pin(&milestone_dir(&jigc_root, milestone_id)) {
        Ok(pin) => {
            // Stale-base edge (T7): a rewritten-away base pin routes the human rather than
            // resolving the committed index against a missing commit — a silently-empty
            // committed store would mis-report the cross-area ref walk
            // (`design/team-ready-state.md` → Stale-base edge). The main checkout is a git
            // dir, so probe there. An unknown milestone (no pin, the `Err` arm) is left to
            // the engine's routed `milestone.unknown` block below.
            guard_base_live(&jigc_home, milestone_id, &pin)?;
            pin.sha
        }
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
fn dispatch_finalize(cwd: &Path, format: Format, milestone_id: &str) -> Outcome {
    match run_milestone_finalize(cwd, format, milestone_id) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            crate::task::finalize_failure_outcome(&err)
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
/// unknown milestone), the `dispatch_join` precedent; (1b) under a `[dev ▸ methodology]`
/// project [`flip_record_for_finalize`] flips the committed `milestone-record` to `joined`
/// (the `join` in-place-mutate arm) and folds that write into THIS commit — path-added
/// alongside the promoted docs, its change gating `has_diff`, guarded so a blocked/failed
/// finalize restores it (`design/team-ready-state.md` → The commit model — join folds); (2)
/// the message is the CLI-[`synthesized_message`] structural projection of the milestone id +
/// its id-ordered sub-task list (a milestone has no commit doc to render — planner-note (b));
/// (3) [`plan_milestone_finalize`] runs the shared preflight (`base` == HEAD, or advanced over
/// a record-only range) + empty-commit guard + promote/hash sweep over the materialized staging
/// area; (4) the **shared** [`crate::task::execute_finalize_plan`] executor promotes, stages
/// (incl. the flipped record), and commits in **one** boundary, removing the milestone area on
/// success. The engine performs no git; the CLI reads HEAD and locates `.jigc/`.
fn run_milestone_finalize(cwd: &Path, format: Format, milestone_id: &str) -> Result<Outcome> {
    // The committed doc-store + `.jigc/` bind to jigc_home (the main checkout); HEAD + the
    // git commit/stage stay on the worktree `repo_root` (M31 Inc 2 / WF3). The promote
    // transaction (`try_execute_finalize_plan` / `execute_finalize_plan`) is kept on
    // `repo_root` — the worktree-finalize promote placement is the deferred WF4/WF5 concern.
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    let schemas = shipped_schemas(&jigc_home)?;

    // Reconcile preflight (T6), hoisted ahead of the reseed's terminal guard — drift is diagnosed
    // before the drifted content is interpreted (the `add-task` note). The finalize status-flip
    // runs this again at its own write site ([`flip_record_for_finalize`]), which is where T6
    // places it; a clean record makes the second call a no-op.
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        reconcile_record_preflight(&jigc_home, &jigc_root, schema, milestone_id)?;
    }

    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the `dir.is_dir()` guard + base-pin/task-list reads + the engine `materialize`'s internal
    // cache reads, so `finalize` on a fresh clone re-derives the milestone shape and joins the
    // recorded sub-tasks (`design/team-ready-state.md` → Engine capability 2 (read-back)). A
    // **settled** record refuses here (the terminal guard): a landed milestone is not re-joined,
    // and an abandoned one is not finalized.
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

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
    // Stale-base edge (T7): a rewritten-away base pin routes the human BEFORE any git work
    // against the missing commit (the worktree-combine base, the base-guard `merge-base`) —
    // so nothing is materialized, flipped, or committed against a base that no longer exists
    // (`design/team-ready-state.md` → Stale-base edge).
    guard_base_live(&repo_root, milestone_id, &base)?;
    let committed = load_committed(&jigc_home, &jigc_root, &schemas, &base.sha);

    // Step 1 — materialize the join's suffix-resolved bodies. A blocking join finding (a
    // same-doc clash) surfaces here and commits nothing (the materialize blocks before it
    // writes — the `dispatch_join` precedent, planner-note (c)).
    let materialized = materialize(&jigc_root, &jigc_home, milestone_id, &schemas, &committed)
        .map_err(finding_to_err)?;

    // The `join` in-place-mutate arm, CLI side (`design/team-ready-state.md` → Engine capability
    // 1 (write), the `join` — in-place mutate arm; The commit model — join folds): under a
    // `[dev ▸ methodology]` project flip the committed record's every `tasks` item + the header
    // `status` active → joined via [`engine::milestone::join_record`] and FOLD the write into
    // THIS single finalize commit (never a second commit). Dev-only (no methodology pack, no
    // `milestone-record` schema) resolves no record → `None`, the finalize path byte-untouched.
    // The returned [`RecordFlipGuard`] restores the pre-flip bytes on ANY blocked/failed
    // finalize below (the flip must not persist on a non-landed commit — "Writes are
    // transactional"); the success paths [`disarm`](RecordFlipGuard::disarm) it once the commit
    // has landed.
    let mut record_flip = flip_record_for_finalize(&repo_root, &jigc_home, &schemas, milestone_id)?;
    let record_changed = record_flip.as_ref().is_some_and(|f| f.changed);
    let record_pathspec = record_flip.as_ref().map(|f| f.pathspec.clone());

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

    // The id-ordered still-provisioned fan-out worktrees (empty for a docs-only milestone)
    // — both the empty-commit signal below and the `squash: true` combine channel below
    // read this set.
    let worktrees = provisioned_worktrees(&repo_root, &jigc_home, &list);

    // The diff-presence signal the planner's empty-commit guard needs, narrowed to the
    // worktree-isolation model (M31 Inc 4): the materialized docs that will be promoted, OR
    // any worktree's staged code (Σ `git diff --cached`). The main checkout no longer holds
    // the fan-out's code — it lives in the isolated worktrees — so a main-checkout
    // diff/untracked scan would both miss the real code and false-count unrelated WIP.
    let head = git_head(&repo_root)?;
    // `record_changed` folds the milestone record's `join` status-flip into the diff signal
    // (M39 T4 — "has_diff gated on the record change"): a docs-only milestone whose only change
    // is the record flip is NOT an empty commit.
    let has_diff = !materialized.addresses.is_empty()
        || worktrees_have_staged_code(&worktrees)?
        || record_changed;

    // The finalize base-guard refinement (`design/team-ready-state.md` → The commit model: the
    // finalize base-guard refinement; `DECISIONS.md` 2026-07-07). Per-op record commits advance
    // HEAD past the pinned base, so `base == HEAD` can never hold once record commits land. The
    // engine does no git I/O, so the CLI computes the verdict: `base..HEAD` is a linear-ancestor
    // range whose every commit touches ONLY milestone-record paths (any milestone's — record
    // bookkeeping is not code). Any non-record (external) commit fails the check → the engine
    // keeps the base-mismatch block, preserving the M31 worktree-combine guarantee. Only computed
    // when the base actually trails HEAD.
    let record_only_advance =
        base.sha != head && record_only_range(&repo_root, &schemas, &base.sha, &head)?;

    // Step 3 — the thin sibling planner over the materialized staging area (the parent of
    // `merged/docs/`): shared preflight + empty-commit guard + promote/hash sweep.
    let staging_dir = materialized
        .docs_dir
        .parent()
        .expect("the materialized docs dir has a parent staging area")
        .to_path_buf();
    let plan = match plan_milestone_finalize(
        milestone_id,
        &staging_dir,
        &base,
        &head,
        record_only_advance,
        message,
        has_diff,
        &schemas,
    ) {
        Ok(plan) => plan,
        Err(findings) => return blocked(&jigc_home, format, findings),
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
        // A cross-worktree code collision blocks **up front**, before any commit — the
        // disjoint per-sub-task patches must apply cleanly in sequence, so a same-path
        // collision blocks naming the path and commits nothing (the combine's
        // never-text-merge discipline, applied to the honest-rework path;
        // `crate::combine::detect_code_collision`).
        if let Some(finding) = crate::combine::detect_code_collision(&worktrees)? {
            return Err(finding_to_err(finding));
        }

        // Build the id-ordered `(staged-patch, rendered-commit-message)` pairs for the
        // code-carrying sub-tasks (a render finding blocks here, before any commit). The
        // SHARED executor's `ChainPerSubtask` channel then lays down one commit per sub-task
        // (carrying THAT sub-task's code, hooks run + relayed) followed by the merged-docs
        // aggregate — all in a **dedicated worktree**, then a fast-forward of main. The live
        // checkout is never the commit site and is never `git reset --hard`ed, so an abort
        // (a hook rejection) leaves unrelated main-checkout WIP intact (review S2 — the
        // squash:true WIP-safety, mirrored onto the honest-rework path). The merged docs ride
        // the aggregate (review B1/B2: the by-task-id doc-join is a milestone-level merge
        // artifact, not cleanly partitionable per sub-task). No reconcile sweep → `None`.
        let subtasks =
            subtask_patches_and_messages(&jigc_home, milestone_id, &worktrees, &schemas)?;
        match crate::task::try_execute_finalize_plan(
            &repo_root,
            &jigc_root,
            &dir,
            &plan,
            &dir,
            &schemas,
            None,
            crate::task::StagePolicy::ChainPerSubtask {
                subtasks,
                format,
                record: record_pathspec,
            },
        )? {
            Ok(hook_output) => {
                // The boundary landed — the flipped record rode the aggregate commit; disarm
                // its restore guard so the committed `joined` bytes are not reverted.
                disarm_record_flip(&mut record_flip);
                // Relay the aggregate commit's non-blocking hook output (each per-sub-task
                // commit already relayed its own inside `chain_commit` — every fan-out commit
                // runs the user's hooks, `design/finalize.md` → 6. Commit).
                crate::task::relay_hook_output(format, &hook_output);
                // The boundary landed — clean up the per-sub-task working areas too (the
                // executor only removed the milestone area). On a failure (below) the areas
                // survive for retry.
                cleanup_subtask_areas(&jigc_root, &list);
                // Tear down the fan-out worktrees the provision verb laid down (the heavier
                // A2 teardown — a non-blocking warning on a leaked worktree, never a block).
                remove_worktrees(&repo_root, &jigc_home, &list);
                Ok(Outcome::success())
            }
            // The chain was aborted (a per-sub-task or the aggregate hook rejection). The
            // chain built every commit in a dedicated worktree and never fast-forwarded main,
            // so the live checkout is untouched (HEAD at the pre-finalize sha, unrelated WIP
            // intact) and there is nothing to reset. The executor already rolled back its
            // promoted-doc copies; tear down the fan-out worktrees (the sub-task areas survive
            // for retry), surface git's stderr verbatim, and exit `FAILURE`.
            Err(err) => {
                remove_worktrees(&repo_root, &jigc_home, &list);
                eprintln!("{}", render::operational_error(format, &err));
                // A rejected chain names itself in the invocation log (the route-exempt
                // commit-phase identity), while git's stderr above stays verbatim.
                Ok(crate::task::finalize_failure_outcome(&err))
            }
        }
    } else {
        // Step 4 — the SHARED executor: promote + combine + commit (one boundary) +
        // post-commit. The message temp file is written into the (gitignored) milestone
        // area; the milestone area is the cleanup dir removed on a landed commit. The
        // `squash: true` boundary folds the N still-provisioned worktrees' staged code-sets
        // into the single commit (M31 Inc 4) via the executor's `Combine` channel — never a
        // whole-tree sweep (the code lives in the isolated worktrees, not this checkout). A
        // docs-only (never-provisioned) milestone yields an empty list and degrades to a
        // docs-only commit, byte-identical to what M7 shipped.
        let code = crate::task::execute_finalize_plan(
            &repo_root,
            &jigc_root,
            &dir,
            &plan,
            &dir,
            &schemas,
            format,
            crate::task::StagePolicy::Combine(worktrees, record_pathspec),
        )?;
        // On a landed commit, clean up the per-sub-task working areas too (the executor only
        // removed the milestone area). A failed/rolled-back finalize exits non-zero and leaves
        // the areas intact for retry.
        if code.code == 0 {
            // The flipped record rode the single combine commit — disarm its restore guard so
            // the committed `joined` bytes are not reverted.
            disarm_record_flip(&mut record_flip);
            cleanup_subtask_areas(&jigc_root, &list);
            // Tear down the fan-out worktrees on the landed default-path commit too (the
            // heavier A2 teardown — a non-blocking warning on a leaked worktree).
            remove_worktrees(&repo_root, &jigc_home, &list);
        }
        Ok(code)
    }
}

/// A transactional guard over the milestone record's `joined` flip that the finalize commit
/// folds in (`design/team-ready-state.md` → The commit model — join folds). Holds the record's
/// pre-flip bytes; on `Drop` (any early return / blocked / failed finalize) it rewrites them, so
/// a non-landed finalize leaves the committed record at its pre-flip `active` state ("Writes are
/// transactional" — the flip must not persist without the commit). The success paths call
/// [`disarm`](RecordFlipGuard::disarm) once the commit has landed, so the committed `joined`
/// bytes are kept.
struct RecordFlipGuard {
    /// The committed record's on-disk path (outside `.jigc/`).
    path: PathBuf,
    /// The pre-flip bytes to restore on an aborted finalize.
    before: String,
    /// Whether the flip actually changed bytes — folded into the `has_diff` signal.
    changed: bool,
    /// The repo-relative pathspec the finalize commit path-adds.
    pathspec: String,
    /// Cleared by [`disarm`](RecordFlipGuard::disarm) once the commit lands.
    armed: bool,
}

impl RecordFlipGuard {
    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for RecordFlipGuard {
    fn drop(&mut self) {
        if self.armed {
            // Best-effort restore — the finalize did not land, so revert the working-tree flip.
            let _ = std::fs::write(&self.path, &self.before);
        }
    }
}

/// Disarm the (optional) record-flip guard on a landed finalize — a `None` (dev-only, no record)
/// is inert.
fn disarm_record_flip(guard: &mut Option<RecordFlipGuard>) {
    if let Some(guard) = guard {
        guard.disarm();
    }
}

/// Flip the committed `milestone-record` to `joined` in place and set up its fold into the
/// finalize commit — the CLI half of the `join` in-place-mutate arm
/// (`design/team-ready-state.md` → Engine capability 1 (write); The commit model — join folds).
///
/// Returns `None` dev-only (no `milestone-record` schema resolved → the finalize path stays
/// byte-identical to today — the omitting-context inert case). Under `[dev ▸ methodology]` it
/// reads the committed record, calls [`engine::milestone::join_record`] (the byte-stable
/// in-place item-leaf + header `status` splice, active → joined), and returns an **armed**
/// [`RecordFlipGuard`] carrying the pre-flip bytes (for a transactional restore on a
/// blocked/failed finalize), the change signal (fed into `has_diff`), and the repo-relative
/// pathspec the finalize commit path-adds. The record home is under `jigc_home`'s docs-root; the
/// pathspec is stripped against the git-commit `repo_root` (they coincide outside a worktree,
/// and milestone-finalize-in-a-worktree is the deferred WF4/WF5 concern).
fn flip_record_for_finalize(
    repo_root: &Path,
    jigc_home: &Path,
    schemas: &BTreeMap<String, Schema>,
    milestone_id: &str,
) -> Result<Option<RecordFlipGuard>> {
    let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) else {
        return Ok(None);
    };
    // Reconcile preflight (T6): the status-flip is a `set: on-transition` overwrite, so
    // conflict-block if the committed record drifted out-of-band before we flip it — leaving
    // the record untouched (`design/team-ready-state.md` → F3). `repo_root` and `jigc_home`
    // coincide outside a worktree (milestone-finalize-in-a-worktree is the deferred WF4/WF5
    // concern), so the `.jigc/` baseline home is `jigc_home/.jigc`.
    reconcile_record_preflight(jigc_home, &jigc_home.join(".jigc"), schema, milestone_id)?;

    let record_path = engine::store::canonical_path(jigc_home, schema, milestone_id)
        .context("the `milestone-record` doctype declares no committed location")?;
    let before = std::fs::read_to_string(&record_path)
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;
    let joined = engine::milestone::join_record(&record_path, schema, milestone_id)
        .map_err(finding_to_err)?;
    let pathspec = record_path
        .strip_prefix(repo_root)
        .unwrap_or(&record_path)
        .to_str()
        .with_context(|| format!("record path {record_path:?} is not valid UTF-8"))?
        .to_owned();
    Ok(Some(RecordFlipGuard {
        changed: joined != before,
        path: record_path,
        before,
        pathspec,
        armed: true,
    }))
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

/// Tear down the milestone's fan-out worktrees once the commit boundary settles — a
/// landed finalize (success) or an abort reset. For each sub-task id whose
/// `<jigc_home>/.jigc/worktrees/<id>` checkout is still a **registered** worktree,
/// `git worktree remove --force` it, then `git worktree prune` the admin records (the
/// [`provision_worktrees`] inverse). A never-provisioned (or already-removed) sub-task
/// has nothing registered and is skipped, so a non-fan-out finalize tears down nothing.
///
/// Best-effort, but **heavier than** [`cleanup_subtask_areas`]' silent self-heal
/// (review A2, `DECISIONS.md` 2026-06-20 → M31 planning): a removal that fails surfaces
/// a **non-blocking warning** naming the leaked worktree path + the `git worktree prune`
/// remedy — a leaked worktree is a registered git object, not gitignored scratch — yet
/// it never blocks a commit that already landed (the F1 rollback/landed-commit stance).
fn remove_worktrees(repo_root: &Path, jigc_home: &Path, list: &engine::milestone::TaskList) {
    let registered = registered_worktrees(repo_root).unwrap_or_default();
    // Match `provision_worktrees`' canonical-path convention (git stores canonical paths at
    // `add` time); fall back to the raw path if canonicalization fails (then nothing matches
    // and the worktree is left registered — surfaced by the prune-only no-op below).
    let canonical_home = jigc_home
        .canonicalize()
        .unwrap_or_else(|_| jigc_home.to_path_buf());
    for sub_id in list.enumerate() {
        let path = canonical_home.join(worktree_path(&sub_id));
        if !registered.iter().any(|w| w == &path) {
            // Never provisioned (or already torn down) — nothing to remove.
            continue;
        }
        let Some(path_str) = path.to_str() else {
            eprintln!("warning: fan-out worktree path {path:?} is not valid UTF-8 (left in place)");
            continue;
        };
        if let Err(err) = git_worktree(repo_root, &["worktree", "remove", "--force", path_str]) {
            // A2 — pinned non-blocking warning, naming the leaked path + the prune remedy.
            eprintln!(
                "warning: could not remove the fan-out worktree {path_str}: {err:#}\n  \
                 remedy: run `git worktree prune`, then `git worktree remove --force {path_str}`"
            );
        }
    }
    // Drop admin records for any worktree dir removed out-of-band (the provision prune
    // inverse) — best-effort; a prune failure is itself non-fatal to a landed commit.
    let _ = git_worktree(repo_root, &["worktree", "prune"]);
}

/// The `commit` doc type the per-sub-task render addresses — a sub-task's authored
/// commit doc is `commit:<sub-id>` (the `commit:<task-id>` provisioning convention),
/// the transient type whose sink is the git message.
const COMMIT_TYPE: &str = "commit";

/// Surface a **blocked** milestone `finalize` — the funnel the per-task planner block already
/// rides ([`crate::task`]'s `blocked`): `--format json` prints the pinned findings envelope on
/// stdout (keys and all), the agent/human view keeps its message + route lines on stderr, and
/// the exit is the validation outcome (3) either way (`design/measurement.md` → The capture
/// substrate, item 2).
///
/// **The envelope, not prose (M42 inc-9 T4).** §3's consumer list is *descriptive, not
/// constitutive* (`design/command-output-contract.md` → the findings envelope): a verb that
/// hands a driver a `Finding` **is** a consumer of the envelope. A blocked milestone finalize
/// handed one to nobody — it printed the message and the route as text in every format — so
/// the milestone arm of `finalize.base-mismatch` reached no driver at all. It now rides the
/// same envelope as its task sibling, keyed at `milestone:<id>`.
fn blocked(jigc_home: &Path, format: Format, findings: Vec<Finding>) -> Result<Outcome> {
    let pack = make_pack()?;
    let cascade = crate::start::resolve_severity_cascade(
        pack.as_ref(),
        &jigc_home.join(".jigc").join("config"),
    )?;
    let report = engine::result::ValidationReport::new(findings, &cascade);
    if format == Format::Json {
        print!("{}", render::validation(format, &report));
    } else {
        for finding in &report.findings {
            eprintln!("{}", finding.message);
            if let Some(route) = &finding.route {
                eprintln!("  route: {route}");
            }
        }
    }
    Ok(Outcome::with_findings(
        crate::task::EXIT_VALIDATION_BLOCKED,
        &report.findings,
    ))
}

/// Resolve the `finalize.fan-out.squash` knob for the project at `repo_root` —
/// `true` (the pack default) keeps the single CLI-synthesized aggregate commit; `false`
/// opts into per-sub-task commits. A missing `.jigc/config/` layer resolves to the
/// pack-default base (`true`), so the no-knob/default path is byte-identical to today
/// (`design/overrides.md` → Read-side determinism; the `task validate`/`finalize`
/// cascade-resolve idiom, `crate::start::resolve_severity_cascade`).
fn resolve_squash(repo_root: &Path) -> Result<bool> {
    let project_config = repo_root.join(".jigc").join("config");
    let pack = make_pack()?;
    let resolved = crate::start::resolve_severity_cascade(pack.as_ref(), &project_config)?;
    // The knob is a declared `bool`; the closed surface guarantees it resolves. Anything
    // other than `true` is the opt-in `false` (the knob's enum-of-bool is `true`/`false`).
    Ok(resolved.scalar_required("finalize.fan-out.squash")? == "true")
}

/// Build the id-ordered `(staged-patch, rendered-commit-message)` pairs for the
/// `squash: false` **honest-rework** mode (M31; `design/finalize.md` → `fan-out` finalize) —
/// the inputs [`crate::task::StagePolicy::ChainPerSubtask`] turns into **one commit per
/// sub-task in id-sorted order**, each carrying THAT sub-task's worktree-attributed code.
/// For each provisioned worktree (already id-sorted) that staged code, capture its staged
/// patch (`git diff --cached --binary` in the worktree) and pair it with its authored
/// `commit:<sub-id>` doc ([`engine::finalize::render_subtask_messages`]). A worktree with
/// **nothing staged** contributes no pair (the retired `--allow-empty` tree-empty form is
/// gone), so the commit *sequence* is a pure function of the id set of code-carrying
/// sub-tasks (hardening #7). The merged docs land in the parent aggregate, not here (review
/// B1/B2). A sub-task missing its authored commit doc routes the engine's blocking render
/// finding — surfaced here, before any commit.
fn subtask_patches_and_messages(
    jigc_home: &Path,
    milestone_id: &str,
    worktrees: &[PathBuf],
    schemas: &BTreeMap<String, Schema>,
) -> Result<Vec<(Vec<u8>, String)>> {
    let commit_schema = schemas
        .get(COMMIT_TYPE)
        .with_context(|| format!("the embedded pack ships no `{COMMIT_TYPE}` schema"))?;
    // The sub-task working areas bind to jigc_home (the main checkout), not the per-task
    // worktree (M31 WF3); outside a worktree the two coincide.
    let tasks_root = jigc_home.join(".jigc").join("tasks");

    // The id-ordered worktrees that staged code, paired with their staged patch — a
    // worktree with nothing staged contributes no per-sub-task commit (no `--allow-empty`);
    // its docs ride the parent aggregate. `worktrees` is already id-sorted (the provisioned
    // set), the deterministic order the commit sequence is laid down in.
    let mut coded: Vec<(String, Vec<u8>)> = Vec::new();
    for wt in worktrees {
        let id = wt
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let patch = worktree_staged_patch(wt)?;
        if patch.is_empty() {
            continue;
        }
        coded.push((id, patch));
    }

    // Render each code-carrying sub-task's authored commit doc in id order (one engine
    // call) — the message each per-sub-task commit carries.
    let ids: Vec<String> = coded.iter().map(|(id, _)| id.clone()).collect();
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

    // Pair each worktree's staged patch with its rendered message, in id order.
    Ok(coded
        .into_iter()
        .map(|(_, patch)| patch)
        .zip(messages)
        .collect())
}

/// A provisioned worktree's staged patch (`git diff --cached --binary`) as raw bytes —
/// binary-safe (a patch is not guaranteed UTF-8) and empty when nothing is staged. The
/// `squash: false` honest-rework reads it per worktree, then [`crate::task::chain_commit`]
/// applies it in a dedicated worktree.
fn worktree_staged_patch(worktree: &Path) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .args(["diff", "--cached", "--binary"])
        .current_dir(worktree)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git diff --cached` in worktree {worktree:?} failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(out.stdout)
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
                .expect("milestone-execution composes over the fed sub-task list")
                .view;

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
                .expect("the reversed feed composes")
                .view;
        assert_eq!(
            composed.text, composed_rev.text,
            "the fan-out emit must be byte-identical across divergent feed orders (id-sorted on resolve)",
        );
    }
}
