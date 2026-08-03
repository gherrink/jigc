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
    JoinOutcome, MintedMilestone, TASKS_FILE, add_from_spec, add_task, join, materialize,
    milestone_dir, mint_milestone, read_base_pin, read_task_list, render_fresh_record,
    synthesized_message, worktree_path,
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
        /// Declare the pre-milestone staged index state deliberate: proceed past the
        /// carryover refuse (`finalize.carried-staged`). The aggregate commit is built
        /// from the sub-task worktrees, so the carried entries never ride it — they
        /// stay staged across the boundary either way. Inert when nothing is carried.
        #[arg(long)]
        carry_staged: bool,
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
        if let MilestoneCommand::Finalize {
            milestone_id,
            carry_staged,
        } = self
        {
            return dispatch_finalize(cwd, format, &milestone_id, carry_staged);
        }
        // `execute` composes a workflow and emits the composed view (not a one-line
        // summary), so — like `join`/`finalize` — it has its own dispatch arm.
        if let MilestoneCommand::Execute { milestone_id } = self {
            return dispatch_execute(cwd, format, &milestone_id);
        }
        // The four record-only committing doors share ONE `Err` arm below, so each one's
        // half of the survivable frame — its state-truth clause, its own re-run argv, and
        // its own error identity — is captured HERE, before the match consumes `self`
        // (M47 Inc 3 T7). `None` for the read-only verbs: they commit nothing, so no hook
        // can reject them.
        let frame = self.rejection_frame();
        // Each committing verb returns `(summary, hook_output)` — the record-only
        // commit's captured non-blocking hook stream (the hook_output producer axis;
        // `design/command-output-contract.md` → Stream discipline). The read-only verbs
        // commit nothing, so their stream is the empty string (present-always).
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
            MilestoneCommand::ListTasks { milestone_id } => {
                run_list_tasks(cwd, &milestone_id).map(|summary| (summary, String::new()))
            }
            MilestoneCommand::Provision { milestone_id } => {
                run_provision(cwd, &milestone_id).map(|summary| (summary, String::new()))
            }
            MilestoneCommand::Discard {
                milestone_id,
                force,
            } => run_discard(cwd, &milestone_id, force),
            MilestoneCommand::Execute { .. } => unreachable!("`Execute` is handled above"),
            MilestoneCommand::Join { .. } => unreachable!("`Join` is handled above"),
            MilestoneCommand::Finalize { .. } => unreachable!("`Finalize` is handled above"),
        };
        match result {
            Ok((summary, hook_output)) => {
                println!("{}", render::milestone(format, &summary, &hook_output));
                // One capture, two channels: the same string rides the envelope above and
                // the delimited relay (stderr under `--format json`, stdout on agent-text).
                crate::task::relay_hook_output(format, &hook_output);
                Outcome::success()
            }
            Err(err) => match &frame {
                // A record-only door: a hook rejection is framed with what survived + this
                // door's own re-run, and names itself in the log. Every other failure keeps
                // the plain operational-error envelope.
                Some(frame) => crate::task::surface_commit_rejection(format, &err, frame),
                // A read-only verb — it runs no commit, so no `CommitRejected` can reach here.
                None => {
                    eprintln!("{}", render::operational_error(format, &err));
                    Outcome::failure()
                }
            },
        }
    }

    /// This verb's half of the **survivable frame** — its state-truth clause, its own
    /// copy-runnable re-run argv, and its own error identity — or `None` for a verb that
    /// commits nothing (M47 Inc 3 T7; `design/finalize.md` → 6. Commit).
    ///
    /// The four record-only doors each write the record, `git add` it, and commit it, and
    /// since M47 Inc 2 a rejected commit restores the captured pre-image on **both** axes
    /// (worktree bytes + index entry) and unwinds any mint the door made — which is what
    /// makes a state-truth clause statable at all here rather than a bricking notice. Each
    /// clause is nevertheless written to *its own* door's truth, and they differ:
    /// `create` / `add-task` leave nothing of the op behind; `discard` settles an existing
    /// record, so the record and the workbench both stay as they were; and `add-from-spec`
    /// is **resumable**, so its earlier record commits are history no rollback takes back
    /// (see its inline note — the one clause that must not say "unchanged").
    ///
    /// The re-run echoes the flags the repeat run genuinely needs (`--workflow` when it was
    /// steered off the default, `--force` on a discard), so the printed line is followable
    /// verbatim rather than merely recognizable.
    fn rejection_frame(&self) -> Option<crate::task::RejectionFrame> {
        use crate::task::shell_token;
        let (code, survived, rerun) = match self {
            MilestoneCommand::Create { title } => (
                crate::invocation_log::ERROR_MILESTONE_CREATE_REJECTED,
                format!(
                    "nothing was committed — the record write and the milestone workbench were \
                     both rolled back, so nothing of milestone:{} survives",
                    engine::milestone::mint_id(title),
                ),
                format!("jigc milestone create {}", shell_token(title)),
            ),
            MilestoneCommand::AddTask {
                milestone_id,
                intent,
                workflow,
            } => (
                crate::invocation_log::ERROR_MILESTONE_ADD_TASK_REJECTED,
                format!(
                    "nothing was committed — the record append and the sub-task mint were both \
                     rolled back, so milestone:{milestone_id} is unchanged"
                ),
                format!(
                    "jigc milestone add-task {milestone_id} {}{}",
                    shell_token(intent),
                    workflow_flag(workflow),
                ),
            ),
            MilestoneCommand::AddFromSpec {
                milestone_id,
                spec_addr,
                workflow,
            } => (
                crate::invocation_log::ERROR_MILESTONE_ADD_FROM_SPEC_REJECTED,
                // NOT "the milestone is unchanged": seeding is resumable (M47 Inc 2 T3), so a
                // rejection at the k-th criterion leaves the k−1 already-landed record commits
                // as history — only the refused append and the mints it never recorded are
                // unwound. Claiming otherwise would be a law-1 lie for every k > 1, and the
                // `note:` lines the seeding loop prints above carry the counts.
                format!(
                    "nothing was committed for the sub-task being recorded — its record append \
                     and every not-yet-recorded mint were rolled back, so \
                     milestone:{milestone_id}'s task list names exactly what its record names; \
                     any sub-task this run already recorded stayed committed, and the re-run \
                     seeds only the remainder"
                ),
                format!(
                    "jigc milestone add-from-spec {milestone_id} {spec_addr}{}",
                    workflow_flag(workflow),
                ),
            ),
            MilestoneCommand::Discard {
                milestone_id,
                force,
            } => (
                crate::invocation_log::ERROR_MILESTONE_DISCARD_REJECTED,
                format!(
                    "nothing was committed — milestone:{milestone_id}'s record is still at its \
                     pre-discard state and its workbench is untouched"
                ),
                format!(
                    "jigc milestone discard {milestone_id}{}",
                    if *force { " --force" } else { "" },
                ),
            ),
            MilestoneCommand::ListTasks { .. }
            | MilestoneCommand::Provision { .. }
            | MilestoneCommand::Execute { .. }
            | MilestoneCommand::Join { .. }
            | MilestoneCommand::Finalize { .. } => return None,
        };
        Some(crate::task::RejectionFrame {
            code,
            survived,
            rerun,
        })
    }
}

/// The milestone boundary's **own** re-run command line — shared by both commit-model arms
/// (the route is the same argv either way; only the logged identity differs). `--carry-staged`
/// is echoed when it was declared, else the re-run would refuse at the carryover gate before
/// it ever reached the commit phase again.
fn milestone_finalize_rerun(milestone_id: &str, carry_staged: bool) -> String {
    format!(
        "jigc milestone finalize {milestone_id}{}",
        if carry_staged { " --carry-staged" } else { "" },
    )
}

/// The ` --workflow <id>` suffix a re-run needs, empty when the door ran on the recorded
/// default (the flag `clap` fills in when it is absent).
fn workflow_flag(workflow: &str) -> String {
    if workflow == DEFAULT_SUB_TASK_WORKFLOW {
        String::new()
    } else {
        format!(" --workflow {workflow}")
    }
}

/// The one does-not-exist rejection the milestone verbs share (`list-tasks` /
/// `provision` / `execute` / `finalize` — each rejecting an absent
/// `.jigc/milestones/<id>/` workbench; M43 T7, `design/surface-contract.md`
/// → The route fence, closing paragraph). The quoted-title span rides the checked
/// [`engine::finding::Route::mechanical`] constructor, so the CLI-seam parse fence
/// asserts it parses against the real CLI. `discard` keeps its own variant (check the
/// id, don't mint — a discard should never route to *creating* the milestone).
fn no_such_milestone(milestone_id: &str) -> anyhow::Error {
    let create =
        engine::finding::Route::mechanical(["jigc", "milestone", "create", "\"<title>\""], "");
    anyhow::anyhow!(
        "milestone `{milestone_id}` does not exist\n  route: create it first with {create}"
    )
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
fn run_create(cwd: &Path, title: &str) -> Result<(String, String)> {
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
    // The carryover gate's door half (M43 T1, `design/surface-contract.md` → The
    // carryover gate): `milestone create` is the shared checkout's aggregate-index
    // door — snapshot the pre-milestone staged state into the milestone area,
    // BEFORE the record commit below touches the index, so the milestone finalize
    // can tell "staged before this milestone existed" from the milestone's own
    // staging. (Sub-task mints write no snapshot: worktrees are provisioned clean
    // and a missing snapshot fails open.)
    let staged = crate::task::git_staged_snapshot(&repo_root)?;
    let minted = mint_milestone(&jigc_root, title, base).map_err(finding_to_err)?;
    engine::state::write_staged_snapshot(&minted.dir, &staged).with_context(|| {
        format!(
            "could not write the staged snapshot for milestone `{}`",
            minted.id
        )
    })?;

    // The record commit's captured non-blocking hook stream (the hook_output producer
    // axis) — empty dev-only (no record, no commit, no hook ran).
    let mut hook_output = String::new();
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        // The record's schema-version stamp value: the doctype's manifest version (the
        // same authority `doc create`'s stamp deriver reads — milestone-record is
        // manifest-frozen since M40 A1), falling back to 1 for a manifest-less pack.
        let stamp = crate::pack::frozen_doctype_versions(make_pack()?.as_ref())
            .get(MILESTONE_RECORD_TYPE)
            .copied()
            .unwrap_or(1);
        // The mint unwinds with its record (M47 Inc 2 T2): the record transaction restores the
        // *committed* half, and this restores the *workbench* half — the milestone area minted
        // three lines up. Removing it is safe precisely because [`mint_milestone`] refuses on a
        // pre-existing area, so `minted.dir` is one this call created, never one it found.
        // Without it the re-run blocks on `milestone.serial-collision` forever, and the approved
        // recoverability ("fix the hook, re-run, it succeeds") is unreachable.
        hook_output = materialize_and_commit_record(&jigc_home, schema, &minted, stamp)
            .inspect_err(|_| unwind_mint(&minted.dir, None))?;
    }

    Ok((
        format!(
            "minted milestone:{} (shared base {})",
            minted.id, minted.base.short
        ),
        hook_output,
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
/// discipline). Returns the record commit's captured non-blocking hook stream.
fn materialize_and_commit_record(
    jigc_home: &Path,
    schema: &Schema,
    minted: &MintedMilestone,
    schema_version: u32,
) -> Result<String> {
    let record_path = engine::store::canonical_path(jigc_home, schema, &minted.id)
        .context("the `milestone-record` doctype declares no committed location")?;
    if let Some(parent) = record_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("could not create the record home {parent:?}"))?;
    }
    // The pre-image, captured BEFORE the write — for `create` it is the absent record
    // (`bytes: absent, index: absent`), so a rejected commit **deletes** the write back out
    // ([`RecordPreImage`]).
    let pre = capture_record_pre_image(jigc_home, &record_path)?;
    let body = render_fresh_record(schema, &minted.id, &minted.base, schema_version);
    std::fs::write(&record_path, &body)
        .with_context(|| format!("could not write the milestone record {record_path:?}"))?;

    // The message temp file lands in the gitignored milestone area (never a tracked path).
    let hook_output = commit_record_transaction(
        jigc_home,
        &record_path,
        &minted.dir,
        &format!(
            "chore(milestone): open record for milestone:{}\n",
            minted.id
        ),
        &pre,
    )?;
    // Seed the record's `file-state` baseline to what `create` just wrote, so the first
    // `add-task`'s reconcile preflight compares against the CLI's own write (T6).
    baseline_record(
        &jigc_home.join(".jigc"),
        schema,
        &minted.id,
        body.as_bytes(),
    );
    Ok(hook_output)
}

/// Land a **record-only** commit for a milestone op (`design/team-ready-state.md` → The commit
/// model). Stages ONLY the record (`git add -- <record>`, index-restricted) and commits ONLY
/// that pathspec (`git commit -F <msg> -- <record>`), so any other staged/untracked change
/// stays out of the record commit and stays staged — the WIP-safety the whole-index
/// [`crate::task::git_commit`] cannot give, hence its narrowed sibling
/// [`git_commit_pathspec`]. The caller-supplied `message` is a CLI-synthesized structural
/// line (a record carries no authored prose) and is written into the gitignored `msg_dir`
/// (the milestone area). Reused by every per-op record commit (`create` opens, `add-task`
/// appends), each passing its own structural subject. Returns the landed commit's captured
/// non-blocking hook stream (the hook_output producer axis) for the verb's ack.
fn commit_record_only(
    repo_root: &Path,
    record_path: &Path,
    msg_dir: &Path,
    message: &str,
) -> Result<String> {
    let spec = record_pathspec(repo_root, record_path)?;
    // Stage ONLY the record (a `git add -- <path>` never sweeps the ambient dirty tree).
    crate::task::git_run(repo_root, &["add", "--", &spec])?;

    let msg_path = msg_dir.join("record-commit-msg.txt");
    std::fs::write(&msg_path, message)
        .with_context(|| format!("could not write the record commit message {msg_path:?}"))?;
    git_commit_pathspec(repo_root, &msg_path, &spec)
}

/// The record's **repo-relative pathspec** — the one string the stage, the commit, and the
/// index pre-image capture all key on, defined once so the rollback provably covers exactly
/// the path the commit stages (the set-fence discipline).
fn record_pathspec(repo_root: &Path, record_path: &Path) -> Result<String> {
    record_path
        .strip_prefix(repo_root)
        .unwrap_or(record_path)
        .to_str()
        .map(str::to_owned)
        .with_context(|| format!("record path {record_path:?} is not valid UTF-8"))
}

/// One record path's **pre-write image** — the fifth staged-path family's capture (M47 Inc 2
/// T1, `design/finalize.md` → Rollback discipline, the record-only-door row; `DECISIONS.md`
/// 2026-07-26 → the M47 Settle, Decision 4).
///
/// The record-only doors write the record, `git add` it, and commit it, so a rejected commit
/// left the write **and** the index entry behind: the milestone bricked (every later door
/// conflict-blocked against a baseline the rejected write never advanced) and the staged
/// residue tripped the next unrelated task's carryover gate. The pre-image carries **both**
/// axes so the restore can put back exactly what was there:
///
/// - `bytes` — the worktree file's content **or `None`** when the record did not exist. This
///   third axis is what the promotions / owner-artifact / config-layer families never needed:
///   `create` writes a record where none existed, so its pre-image is
///   `(bytes: absent, index: absent)` and the restore is a **delete**. A capture that models
///   only "present" silently leaves `create`'s record behind.
/// - `index` — the pre-write index entry, captured through the shared index primitive
///   ([`crate::task::capture_owner_artifact_index`], which already models "absent"), so this
///   family reuses the third axis's `(mode, blob-sha)` discipline rather than minting a
///   parallel one.
struct RecordPreImage {
    /// The record's absolute on-disk path (the worktree axis's restore target).
    path: PathBuf,
    /// The record's pre-write bytes, `None` when the record did not exist.
    bytes: Option<Vec<u8>>,
    /// The record path's pre-write index entry (0 or 1 entries — the shared primitive's shape).
    index: Vec<crate::task::OwnerArtifactIndexEntry>,
}

/// Capture the record's pre-write image — called **before** the door writes the record, so a
/// rejected commit can restore exactly what was there ([`RecordPreImage`]).
///
/// **"Absent" means absent**, never "unreadable": only `NotFound` yields `bytes: None`, because
/// that value is what makes the rollback *delete* the file — swallowing a permission/IO error
/// into it would turn a rejected commit into a deletion of a record that still exists.
fn capture_record_pre_image(repo_root: &Path, record_path: &Path) -> Result<RecordPreImage> {
    let spec = record_pathspec(repo_root, record_path)?;
    let bytes = match std::fs::read(record_path) {
        Ok(bytes) => Some(bytes),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(err) => {
            return Err(err)
                .with_context(|| format!("could not read the milestone record {record_path:?}"));
        }
    };
    Ok(RecordPreImage {
        path: record_path.to_path_buf(),
        bytes,
        index: crate::task::capture_owner_artifact_index(repo_root, std::slice::from_ref(&spec))?,
    })
}

/// Restore a captured [`RecordPreImage`] — the worktree bytes first (rewritten, or the file
/// **deleted** when the pre-image was absent), then the index entry through the shared
/// primitive (`update-index --cacheinfo` for a present entry, `--force-remove` for an absent
/// one — the worktree is never reset to HEAD). Best-effort, exactly like every sibling axis:
/// the commit did **not** land, so a restore failure is swallowed rather than replacing the
/// door's real error (the hook's stderr stays the correction signal).
///
/// The `file-state` baseline is deliberately **not** touched: nothing landed, so nothing
/// re-baselines and `design/reconciliation.md` → Hash re-baselining stands untouched — the
/// restored bytes match the baseline the last *landed* write recorded.
fn rollback_record_pre_image(repo_root: &Path, pre: &RecordPreImage) {
    match &pre.bytes {
        Some(bytes) => {
            let _ = std::fs::write(&pre.path, bytes);
        }
        None => {
            let _ = std::fs::remove_file(&pre.path);
        }
    }
    crate::task::rollback_owner_artifact_index(repo_root, &pre.index);
}

/// **Unwind exactly what this door minted in the same call** — the *workbench* half of a
/// rejected record-only door (M47 Inc 2 T2; `DECISIONS.md` 2026-07-26 → the M47 Settle,
/// Decision 4, the approved *"fix the hook, re-run, it succeeds"*).
///
/// [`commit_record_transaction`] restores the *committed* half, but the door's own mint survived
/// it — so the identical re-run blocked on `milestone.serial-collision` (`create`) or
/// `milestone.sub-task-collision` (`add-task`) **forever**. `area` is the directory this call
/// created (`create`'s milestone area, `add-task`'s sub-task area) and `restore` is a file this
/// call appended to, paired with its captured pre-append bytes (`add-task`'s `tasks.json`).
///
/// **The unwind is the CLI door's, never a relaxation of the engine's identity guard.** Both
/// engine mints refuse on a pre-existing id before creating anything, which is what makes the
/// removal safe: an `area` reaching here is one *this* call minted, not one it found.
///
/// Best-effort, like every sibling rollback: the commit did **not** land, so a cleanup failure
/// must not replace the door's real error (the hook's stderr stays the correction signal) — but
/// it is noted on stderr rather than swallowed, because what survives is a workbench the operator
/// may have to remove by hand.
fn unwind_mint(area: &Path, restore: Option<(&Path, &[u8])>) {
    if area.exists()
        && let Err(err) = std::fs::remove_dir_all(area)
    {
        eprintln!("note: could not unwind the minted working area at {area:?}: {err:#}");
    }
    if let Some((path, bytes)) = restore
        && let Err(err) = std::fs::write(path, bytes)
    {
        eprintln!("note: could not restore {path:?} to its pre-append bytes: {err:#}");
    }
}

/// [`commit_record_only`] as a **transaction**: on any failure of the stage/commit (a
/// rejecting hook or a git error) restore the captured pre-image and propagate the error
/// **unchanged** — the hook's stderr stays verbatim (`design/finalize.md` → the M40
/// refinement 3). Every record-only door commits through this, so the family's rollback
/// coverage is a property of the seam rather than of each caller remembering.
fn commit_record_transaction(
    repo_root: &Path,
    record_path: &Path,
    msg_dir: &Path,
    message: &str,
    pre: &RecordPreImage,
) -> Result<String> {
    commit_record_only(repo_root, record_path, msg_dir, message).inspect_err(|_| {
        rollback_record_pre_image(repo_root, pre);
    })
}

/// `git commit -F <message_file> -- <pathspec>` in `repo_root` — a **pathspec-restricted**
/// commit recording ONLY the listed path, leaving any other staged/untracked change untouched
/// (the WIP-safe milestone-op commit). [`crate::task::git_commit`] commits the whole index, so
/// the record path needs this narrowed sibling — routed through the same hook-capable commit
/// seam ([`crate::task::git_commit_capture`], the hook_output producer axis): the user's
/// hooks run (never `--no-verify`), a rejection surfaces git's stdout+stderr verbatim with
/// nothing committed, and a success returns the captured non-blocking hook stream.
fn git_commit_pathspec(repo_root: &Path, message_file: &Path, pathspec: &str) -> Result<String> {
    crate::task::git_commit_capture(
        repo_root,
        &[
            std::ffi::OsStr::new("-F"),
            message_file.as_os_str(),
            std::ffi::OsStr::new("--"),
            std::ffi::OsStr::new(pathspec),
        ],
    )
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

/// The record door's own **conflict-block presentation** (M47 inc-2 / T4) — the caller half
/// of [`ConflictBlock`](engine::file_state::ConflictBlock).
///
/// There is **no task at this door**: a milestone-record op writes the record directly, so
/// the classifier's task-scope default (`this task's staged writes` + `jigc task discard
/// <task-id>`) was an **inapplicable verb** carrying an **unsubstituted placeholder** on a
/// *blocking* finding — the pair the M43 route floor exists to prevent
/// (`design/surface-contract.md` → The route fence; `DECISIONS.md` 2026-07-26 the Settle,
/// item 8/P6: `<task-id>` is not derivable here — it needs a different source, and here that
/// source does not exist at all). So the message names the **record** and the route is the
/// only resolution that exists: restore what jigc last wrote, then re-run. It is a
/// [`Route::human`](engine::finding::Route::human) — reverting an out-of-band edit is a
/// human judgment over git, not a `jigc` verb (the record is machine-maintained, so the
/// edit is never merged and never clobbered; `design/team-ready-state.md` → the
/// No-silent-overwrite discipline).
fn record_conflict_block(key: &str) -> engine::file_state::ConflictBlock {
    engine::file_state::ConflictBlock::new(
        "the milestone record is machine-maintained and was edited out of band since jigc \
         last wrote it",
        engine::finding::Route::human(format!(
            "restore `{key}` to what jigc last wrote (`git checkout -- {key}` for an \
             uncommitted edit, else revert the commit that changed it) and re-run this \
             command — an external edit to a machine-maintained record is never merged and \
             never clobbered",
        )),
    )
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
        &record_conflict_block(&key),
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
fn run_add_task(
    cwd: &Path,
    milestone_id: &str,
    intent: &str,
    workflow: &str,
) -> Result<(String, String)> {
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

    // The task list's pre-append bytes, captured BEFORE the mint appends to it — the
    // workbench half of this door's pre-image (M47 Inc 2 T2). Captured as raw bytes, so the
    // restore puts back what was there rather than a re-render of what was parsed.
    let list_path = milestone_dir(&jigc_root, milestone_id).join(TASKS_FILE);
    let pre_list = std::fs::read(&list_path).ok();

    let added = add_task(&jigc_root, milestone_id, intent, workflow).map_err(finding_to_err)?;

    // The record-home split (`design/team-ready-state.md` → Engine capability 1 (write), the
    // `add-task` — append arm; The commit model): under a `[dev ▸ methodology]` project the
    // composed cascade resolves the `milestone-record` doctype, so append the sub-task to the
    // committed record and land a **separate record-only** path-scoped commit (the JSON-cache
    // append above is retained as the demoted cache). Dev-only (no methodology pack) resolves
    // no such schema → degrade to today's no-record, no-extra-commit behavior.
    // The record commit's captured non-blocking hook stream (the hook_output producer
    // axis) — empty dev-only (no record, no commit, no hook ran).
    let mut hook_output = String::new();
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        // The mint unwinds with its record (M47 Inc 2 T2): on a rejected commit the sub-task
        // area this call minted goes, and the task list returns to its captured pre-append
        // bytes — so the demoted cache never names a sub-task the record does not, and the
        // identical re-run mints the same id instead of blocking on
        // `milestone.sub-task-collision`.
        hook_output = append_and_commit_record(
            &jigc_home,
            &jigc_root,
            schema,
            milestone_id,
            &added.task.id,
            intent,
        )
        .inspect_err(|_| {
            unwind_mint(
                &added.task.dir,
                pre_list
                    .as_deref()
                    .map(|bytes| (list_path.as_path(), bytes)),
            );
        })?;
    }

    Ok((
        format!(
            "added task:{} to milestone:{}",
            added.task.id, added.milestone_id
        ),
        hook_output,
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
/// Returns the record commit's captured non-blocking hook stream.
fn append_and_commit_record(
    jigc_home: &Path,
    jigc_root: &Path,
    schema: &Schema,
    milestone_id: &str,
    task_id: &str,
    intent: &str,
) -> Result<String> {
    let record_path = engine::store::canonical_path(jigc_home, schema, milestone_id)
        .context("the `milestone-record` doctype declares no committed location")?;
    let source = std::fs::read_to_string(&record_path)
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;

    let appended = engine::milestone::append_task_item(schema, &source, task_id, intent)
        .map_err(|err| finding_to_err(engine::write::generate_error_finding(&err)))?;
    // The pre-image, captured BEFORE the append lands on disk — a rejected commit restores
    // the pre-append bytes AND the pre-append index entry ([`RecordPreImage`]). Under
    // `add-from-spec` this runs once per seeded sub-task, so the k-th rejection unwinds
    // exactly the k-th append.
    let pre = capture_record_pre_image(jigc_home, &record_path)?;
    std::fs::write(&record_path, &appended)
        .with_context(|| format!("could not write the milestone record {record_path:?}"))?;

    // The message temp file lands in the gitignored milestone WIP area (never a tracked path).
    let msg_dir = milestone_dir(jigc_root, milestone_id);
    let hook_output = commit_record_transaction(
        jigc_home,
        &record_path,
        &msg_dir,
        &format!("chore(milestone): record task:{task_id} on milestone:{milestone_id}\n"),
        &pre,
    )?;
    // Advance the record's `file-state` baseline to the appended bytes, so the next overwrite's
    // reconcile preflight compares against this write, not the pre-append record (T6).
    baseline_record(jigc_root, schema, milestone_id, appended.as_bytes());
    Ok(hook_output)
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
) -> Result<(String, String)> {
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

    // Resumable seeding (M47 Inc 2 T3): a criterion whose sub-task the milestone already
    // carries is skipped, not collided — so the re-run after a rejected k-th record commit
    // seeds exactly the remainder instead of dead-ending on `milestone.sub-task-collision`.
    // **The skip set is what the COMMITTED RECORD names** (the T3 fix): the record is the
    // source of truth and `tasks.json` a rebuildable cache, so a criterion that reached the
    // workbench but never the record must never be absorbed as "already seeded" — that is a
    // criterion silently absent from the team-ready record forever, at exit 0.
    let recorded = recorded_sub_tasks(&jigc_home, &schemas, milestone_id)?;
    let seeded = add_from_spec(
        &jigc_root,
        &jigc_home,
        &schemas,
        milestone_id,
        spec_addr,
        workflow,
        recorded.as_deref(),
    )
    .map_err(|aborted| {
        // An abort mid-loop minted sub-tasks the record will never name — the same
        // divergence a rejected k-th record commit leaves, through the same unwind.
        unwind_unrecorded_seeds(&jigc_root, milestone_id, &aborted.minted);
        finding_to_err(aborted.finding)
    })?;

    // The record-home split (`design/team-ready-state.md` → Engine capability 1 (write), the
    // `add-task` append arm; The commit model): under a `[dev ▸ methodology]` project the composed
    // cascade resolves the `milestone-record` doctype, so each spec-seeded sub-task must ALSO land
    // in the committed record — exactly like `add-task` — else the sub-tasks are silently lost on
    // a fresh clone (the source-of-truth invariant). One separate record-only path-scoped commit
    // per seeded sub-task; the intent is read back from the minted task's working area (the
    // verbatim criterion text `add_from_spec` persisted). Dev-only resolves no schema → no record.
    // One record-only commit per seeded sub-task — each captured non-blocking hook
    // stream folds into the one acked string (the hook_output producer axis). Empty
    // dev-only (no record, no commits, no hook ran).
    let mut streams: Vec<String> = Vec::new();
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        for (landed, a) in seeded.added.iter().enumerate() {
            let recorded = engine::state::read_intent(&a.task.dir)
                .with_context(|| {
                    format!(
                        "could not read the seeded sub-task intent for `{}`",
                        a.task.id
                    )
                })
                .and_then(|intent| {
                    append_and_commit_record(
                        &jigc_home,
                        &jigc_root,
                        schema,
                        milestone_id,
                        &a.task.id,
                        &intent,
                    )
                });
            match recorded {
                Ok(stream) => streams.push(stream),
                Err(err) => {
                    // The mid-loop unwind (M47 Inc 2 T3): the k−1 landed record commits are
                    // history and cannot be undone, so this call's atomicity is "the workbench
                    // names exactly what the record names" — every mint from here on goes.
                    unwind_unrecorded_seeds(&jigc_root, milestone_id, &seeded.added[landed..]);
                    print_resume_route(
                        milestone_id,
                        spec_addr,
                        workflow,
                        landed,
                        seeded.added.len(),
                    );
                    return Err(err);
                }
            }
        }
    }

    let ids: Vec<&str> = seeded.added.iter().map(|a| a.task.id.as_str()).collect();
    // The ack states **both halves** of a resumable pass — what this call seeded, and what it
    // found already seeded (`design/surface-contract.md` → law 1: a re-run that skipped N
    // criteria may not report itself as a fresh seeding of nothing).
    let mut summary = format!(
        "seeded {} sub-task(s) into milestone:{milestone_id} from {spec_addr}",
        ids.len()
    );
    if !ids.is_empty() {
        summary.push_str(&format!(": {}", ids.join(", ")));
    }
    if !seeded.already_seeded.is_empty() {
        summary.push_str(&format!(
            " ({} already seeded, skipped: {})",
            seeded.already_seeded.len(),
            seeded.already_seeded.join(", ")
        ));
    }
    Ok((
        summary,
        crate::task::fold_hook_streams(streams.iter().map(String::as_str)),
    ))
}

/// Unwind the sub-tasks this `add-from-spec` call minted but never recorded — the **mid-loop**
/// member of the mint-unwinds-with-its-record discipline (M47 Inc 2 T3; the T2 sibling
/// [`unwind_mint`]).
///
/// `add-from-spec` mints N sub-tasks up front and lands one record-only commit per sub-task,
/// so a rejected k-th commit cannot unwind the call: the k−1 landed record commits are
/// history. What it *can* — and must — restore is the agreement between the two homes: each
/// un-recorded mint's area goes and its id leaves the demoted `tasks.json` cache
/// ([`engine::milestone::drop_sub_tasks`], which re-renders the surviving ids in order, so the
/// cache is byte-identical to its k−1 state). Without it the cache names sub-tasks the record
/// does not, and the resume dead-ends on `milestone.sub-task-collision`.
///
/// Best-effort, like every sibling rollback — the door's real error (the hook's stderr) stays
/// the correction signal — but a failure is noted on stderr rather than swallowed, because
/// what survives is a workbench the operator may have to repair by hand.
fn unwind_unrecorded_seeds(
    jigc_root: &Path,
    milestone_id: &str,
    unrecorded: &[engine::milestone::AddedTask],
) {
    // Nothing minted — nothing to unwind, and no list to re-render (the failure may well be
    // that there is no milestone area at all).
    if unrecorded.is_empty() {
        return;
    }
    for a in unrecorded {
        unwind_mint(&a.task.dir, None);
    }
    let ids: Vec<String> = unrecorded.iter().map(|a| a.task.id.clone()).collect();
    if let Err(err) = engine::milestone::drop_sub_tasks(jigc_root, milestone_id, &ids) {
        eprintln!(
            "note: could not drop the un-recorded sub-task(s) {ids:?} from milestone \
             `{milestone_id}`'s task list: {err:#}"
        );
    }
}

/// Print what a mid-loop rejection actually left behind, and the **command that recovers it**
/// (M47 Inc 2 T3). The recovery is a *resume*: `landed` of this call's `total` mints are in
/// the record for good, the rest were unwound, and re-running the same door seeds only the
/// remainder — so the route is the door's own argv, built through the checked
/// [`Route::mechanical`] constructor (the M43 route fence: a command span that stopped parsing
/// against the real CLI cannot be constructed). `--workflow` rides along when it is not the
/// default, else the resumed mints would silently record a different minting workflow.
///
/// The hook-rejection channel itself stays **verbatim** (`design/finalize.md` → the M40
/// refinement 3); this is a note *beside* it, never a rewrite of it.
fn print_resume_route(
    milestone_id: &str,
    spec_addr: &str,
    workflow: &str,
    landed: usize,
    total: usize,
) {
    let mut argv = vec![
        "jigc".to_string(),
        "milestone".to_string(),
        "add-from-spec".to_string(),
        milestone_id.to_string(),
        spec_addr.to_string(),
    ];
    if workflow != DEFAULT_SUB_TASK_WORKFLOW {
        argv.push("--workflow".to_string());
        argv.push(workflow.to_string());
    }
    let route = engine::finding::Route::mechanical(
        argv,
        " — already-seeded criteria are skipped, not re-minted.",
    );
    eprintln!(
        "note: {landed} of {total} newly seeded sub-task(s) landed in the record; the {} \
         un-recorded mint(s) were unwound, so the task list names exactly what the record does.",
        total - landed
    );
    eprintln!(
        "note: after clearing the rejection, re-run {}",
        route.as_str()
    );
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
/// **Two halves, one site** (M47 Inc 3 T3): the milestone cache
/// ([`engine::milestone::reseed_cache_from_record`]) **and** every sub-task's working area
/// ([`engine::milestone::reseed_sub_task_areas`]). The first alone told the clone *which*
/// sub-tasks the milestone carries while leaving `.jigc/tasks/<sub>/` absent, so the
/// milestone-execution workflow's own emitted `Spawn:` line dead-ended on *"no task"* — routed
/// straight back at `jigc milestone list-tasks`, a loop. Both halves hang off this one site, so
/// the promise above holds for every verb that reaches it rather than for a hand-kept subset.
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
/// **Refuse a `create` whose id a record already owns** — the identity half of the
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

/// **The sub-task ids the COMMITTED RECORD names** — the source of truth for what a milestone
/// already carries (`design/team-ready-state.md` → Engine capability 2 (read-back): the
/// committed `.md` is the record, the `.jigc` JSON a rebuildable cache), read through the same
/// [`engine::milestone::read_back_record`] the cache re-derives itself with.
///
/// `None` means the milestone has **no committed record home at all** — a dev-only project,
/// which resolves no `milestone-record` schema and whose doors land no record commit, so the
/// demoted cache is the only home and is therefore the truth. Under a `[dev ▸ methodology]`
/// composition a *missing* record file is not that case: the record is the truth and it names
/// nothing, so this answers with the empty set rather than falling back to the cache — which
/// is exactly the fallback that let an un-recorded workbench id pass as "already seeded".
///
/// Called by `add-from-spec`'s resume skip set, after [`reseed_cache`] (so a fresh clone has
/// already re-derived its cache from this same record).
fn recorded_sub_tasks(
    jigc_home: &Path,
    schemas: &BTreeMap<String, Schema>,
    milestone_id: &str,
) -> Result<Option<Vec<String>>> {
    let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) else {
        return Ok(None);
    };
    let Some(record_path) = engine::store::canonical_path(jigc_home, schema, milestone_id) else {
        return Ok(None);
    };
    if !record_path.exists() {
        return Ok(Some(Vec::new()));
    }
    let source = std::fs::read_to_string(&record_path)
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;
    let (_, tasks) =
        engine::milestone::read_back_record(schema, &source).map_err(finding_to_err)?;
    Ok(Some(tasks.tasks))
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
    engine::milestone::reseed_cache_from_record(&dir, schema, &source).map_err(finding_to_err)?;
    // The sub-task working areas, rebuilt from the same record (M47 Inc 3 T3). Second, on
    // purpose: `reseed_cache_from_record` carries the terminal guard, so a settled milestone
    // refuses above this line and never has areas rebuilt for it. `DEFAULT_SUB_TASK_WORKFLOW`
    // is the pack fact the engine may not know — the record carries no minting workflow, so an
    // `--workflow` override is workbench-local and not fresh-clone-durable (the engine fn's
    // declared bound).
    engine::milestone::reseed_sub_task_areas(jigc_root, schema, &source, DEFAULT_SUB_TASK_WORKFLOW)
        .map_err(finding_to_err)
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
        return Err(no_such_milestone(milestone_id));
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
        return Err(no_such_milestone(milestone_id));
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
fn run_discard(cwd: &Path, milestone_id: &str, force: bool) -> Result<(String, String)> {
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
        // `<milestone-id>` is the declared dummy-table placeholder (the old `<id>` was
        // undeclared and ambiguous against the task-id sites).
        let list_tasks = engine::finding::Route::mechanical(
            ["jigc", "milestone", "list-tasks", "<milestone-id>"],
            "",
        );
        bail!(
            "milestone `{milestone_id}` does not exist\n  route: check the milestone id ({list_tasks} names a live milestone's sub-tasks); nothing was discarded"
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
    // record, no commit (the omitting context — `hook_output` stays empty).
    let mut hook_output = String::new();
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        let record_path = engine::store::canonical_path(&jigc_home, schema, milestone_id)
            .context("the `milestone-record` doctype declares no committed location")?;
        // The pre-image, captured BEFORE the settle overwrites the record — a rejected commit
        // restores the pre-settle bytes AND the index entry, so the abandon is re-runnable
        // once the hook's complaint is fixed ([`RecordPreImage`]).
        let pre = capture_record_pre_image(&jigc_home, &record_path)?;
        let settled = engine::milestone::discard_record(&record_path, schema, milestone_id)
            .map_err(finding_to_err)?;
        // The message temp file lands in the (gitignored) milestone area — removed by the
        // teardown below, so it is written before the area goes.
        hook_output = commit_record_transaction(
            &jigc_home,
            &record_path,
            &dir,
            &format!("chore(milestone): discard record for milestone:{milestone_id}\n"),
            &pre,
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

    Ok((
        format!(
            "discarded milestone:{milestone_id} ({} sub-task(s); workbench removed)",
            list.enumerate().len()
        ),
        hook_output,
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
        ).into()),
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
        return Err(no_such_milestone(milestone_id));
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
    let (outcome, sub_tasks) = match run_join(cwd, milestone_id) {
        Ok(outcome) => outcome,
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            return Outcome::failure();
        }
    };
    // The merged-overlay summary always prints (the agent reads the suffix/rewrite
    // decisions even when the join is clean). The milestone's full id-sorted sub-task
    // list rides along so the ack can name the doc-less sub-tasks (C3).
    println!(
        "{}",
        render::milestone_join(format, milestone_id, &outcome, &sub_tasks)
    );

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
        // A blocked join is a reject that is not a task-scope gate — the taxonomy
        // constant, never a bare literal (the exit-code table's one origin).
        Outcome {
            code: crate::task::EXIT_ERROR,
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
/// (no area) surfaces as the engine's routed `milestone.unknown` block. Returns the
/// join outcome paired with the milestone's full **id-sorted** sub-task list — the
/// set the C3 ack names the doc-less members of.
fn run_join(cwd: &Path, milestone_id: &str) -> Result<(JoinOutcome, Vec<String>)> {
    // The engine `join` itself performs no git I/O (the base is the milestone's *stored*
    // pin), but the CLI around it does: the stale-base guard shells to git, and the
    // cross-worktree collision read (below) reads each provisioned worktree's staged set.
    // The committed doc-store + `.jigc/` index bind to jigc_home (the main checkout); the
    // fan-out worktrees + their registration bind to the worktree repo_root (M31 Inc 2).
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
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

    let mut outcome =
        join(&jigc_root, &jigc_home, milestone_id, &schemas, &committed).map_err(finding_to_err)?;
    // The join succeeded, so the milestone area exists — read its full sub-task list
    // (id-sorted) for the ack's doc-less-member line (C3).
    let list = read_task_list(&milestone_dir(&jigc_root, milestone_id))
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;

    // Cross-worktree code collisions surface HERE, not only at finalize: two sub-tasks
    // staging the SAME path in their isolated fan-out worktrees is a clash the combine
    // never text-merges (`design/storage.md` → the join's never-blind-merge discipline).
    // A never-provisioned (docs-only) milestone yields no worktrees, so the read is inert.
    let worktrees = provisioned_worktrees(&repo_root, &jigc_home, &list);
    if let Some(finding) = crate::combine::detect_code_collision(&worktrees)? {
        outcome.findings.push(finding);
    }
    Ok((outcome, list.enumerate()))
}

/// Dispatch `jigc milestone finalize <milestone-id>`: run the materialized join +
/// single-commit boundary and map it to the exit code. A blocking join finding (a
/// same-doc clash, an unknown milestone) or an orchestration error routes to stderr and
/// exits non-zero **before** any commit. A landed commit prints a summary and exits 0.
/// `design/finalize.md` → `fan-out` finalize (single-commit form).
fn dispatch_finalize(
    cwd: &Path,
    format: Format,
    milestone_id: &str,
    carry_staged: bool,
) -> Outcome {
    match run_milestone_finalize(cwd, format, milestone_id, carry_staged) {
        Ok(code) => code,
        // Orchestration only — every commit this boundary runs happens inside
        // `try_execute_finalize_plan`, whose rejection is the *inner* `Result` each
        // commit-model arm frames itself (M47 Inc 3 T7). A `CommitRejected` therefore
        // cannot reach here, and the arm-specific identity is minted where the arm is
        // known; a plain failure carries no identity, as before.
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
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
/// area; (4) the **shared** [`crate::task::try_execute_finalize_plan`] executor promotes, stages
/// (incl. the flipped record), and commits in **one** boundary, removing the milestone area on
/// success. The engine performs no git; the CLI reads HEAD and locates `.jigc/`.
fn run_milestone_finalize(
    cwd: &Path,
    format: Format,
    milestone_id: &str,
    carry_staged: bool,
) -> Result<Outcome> {
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
        return Err(no_such_milestone(milestone_id));
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

    // The milestone's id-ordered sub-task list + its still-provisioned fan-out worktrees
    // (empty for a docs-only milestone) — hoisted above the record flip because the
    // boundary conformance gate below reads the worktree set, and it must run before any
    // durable write. Both feed the message, the empty-commit signal, the contribution facts,
    // and the combine channel below (the M31 sibling-site shape — the gate is
    // knob-independent, so its inputs sit above the record flip and the `squash` read).
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;
    let worktrees = provisioned_worktrees(&repo_root, &jigc_home, &list);
    let staging_dir = materialized
        .docs_dir
        .parent()
        .expect("the materialized docs dir has a parent staging area")
        .to_path_buf();

    // The milestone-boundary conformance gate (M45 — `design/finalize.md` → 2. Validate;
    // `design/validation.md` → The milestone-boundary gate): validate the **merged effective
    // state** — the by-task-id merged doc set (`materialize`'s output, on disk in
    // `merged/docs/`) overlaid on the committed store for the `schema-conformance.*` +
    // `ref-resolves` families, plus the `doc-code` code-anchor re-resolution over the merged
    // worktree code tree — and BLOCK (exit 3, committing nothing) on any blocking finding.
    // Positioned after `materialize` (whose output IS the merged doc set — a gate ahead of it
    // would re-implement the merge), after the base guard (the gate reads `base.sha`, not the
    // literal `HEAD`), after `provisioned_worktrees`, and BEFORE the record flip + the plan +
    // the `squash` branch — knob-independent. Nothing durable is written yet (`materialize`
    // rebuilds its dir every call, and the `RecordFlipGuard` is not yet armed), so a block
    // here truly commits nothing. A same-path collision (no single merged tree) is NOT the
    // gate's concern — it falls through to the knob branch's own collision handling below.
    if let Some(outcome) = milestone_boundary_gate(
        &repo_root,
        &jigc_home,
        &jigc_root,
        &schemas,
        &staging_dir,
        &base,
        &worktrees,
        format,
    )? {
        return Ok(outcome);
    }

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

    // The diff-presence signal the planner's empty-commit guard needs, narrowed to the
    // worktree-isolation model (M31 Inc 4): the materialized docs that will be promoted, OR
    // any worktree's staged code (Σ `git diff --cached`). The main checkout no longer holds
    // the fan-out's code — it lives in the isolated worktrees — so a main-checkout
    // diff/untracked scan would both miss the real code and false-count unrelated WIP.
    let head = git_head(&repo_root)?;
    // `record_changed` folds the milestone record's `join` status-flip into the diff signal
    // (M39 T4 — "has_diff gated on the record change"): a docs-only milestone whose only change
    // is the record flip is NOT an empty commit. This stays the *planner's* empty-commit input;
    // whether the boundary lands any actual WORK is the separate, stricter zero-contribution
    // question asked below, after the plan resolves what would be promoted.
    let contributed_code = worktrees_have_staged_code(&worktrees)?;
    let has_diff = !materialized.addresses.is_empty() || contributed_code || record_changed;

    // C2 — the landing manifest's per-sub-task contribution facts, computed PRE-commit
    // while the fan-out worktrees are still provisioned (a landed boundary tears them
    // down): each sub-task's merged-doc count (the materialize's address→source map),
    // its worktree's staged-code file count, and whether it HAS a provisioned worktree
    // at all. The no-work sub-task reads `docs: 0, code_files: 0` and renders visibly as
    // `nothing staged`; a sub-task with no worktree is named too, since it could not have
    // contributed code even in principle (M47 Inc 3, call (b)(ii)).
    let contributions = subtask_contributions(&list, &materialized.sources, &worktrees)?;

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

    // Step 3 — the thin sibling planner over the materialized staging area (`staging_dir`,
    // the parent of `merged/docs/`, hoisted with the gate above): shared preflight +
    // empty-commit guard + promote/hash sweep.
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

    // The **zero-contribution** refusal (M47 Inc 3; `DECISIONS.md` → 2026-07-26 M47 Increment 3
    // halt resolution, call (b)(i)): refuse a boundary that would land **no work at all** — no
    // doc in the plan's promote set, no staged code in any sub-task worktree — leaving only
    // jigc's own bookkeeping (the record's `active → joined` flip + the config layer) to commit.
    // The motivating case is the fresh clone, reproduced live: a clone carries the committed
    // record but none of the gitignored workbench, so `finalize` landed at exit 0 having landed
    // zero work and flipped the record to the TERMINAL `joined`, after which the milestone could
    // never be finalized again. It replaces the settled "refuse when a milestone recorded as
    // provisioned has no live worktrees", withdrawn as unbuildable (no such record exists, and a
    // `.jigc`-local marker fails open by construction on exactly that fresh-clone case).
    //
    // **The predicate is the plan's promote set, not `materialized.addresses`** (M47 Inc 3 T2,
    // recorded elaboration): `has_diff`'s doc term counts every materialized address, and a
    // **transient** doc — the sub-task's `commit:<sub-id>` — is materialized but never promoted
    // (`plan_promotions`: a doctype with neither `location` nor `placement` is transient). So a
    // milestone whose sub-agents authored their commit docs and staged no code passed the
    // `has_diff`-only reading and landed the identical zero-work commit, verified live. Keying
    // on what would actually land closes the whole contribution-channel axis at one seam
    // instead of the one repro. It cannot false-fire on a genuine docs-only milestone: that one
    // promotes its merged docs.
    //
    // Placed immediately AFTER the planner (whose promote set it reads), beside the
    // empty-commit guard it refines: the planner's staging / base-mismatch / empty-commit
    // blocks keep precedence (the carryover gate's precedent below), so a drifted base still
    // reports the base mismatch, and an all-empty dev-only milestone still reports the plain
    // empty-commit block. Blocking here drops the `RecordFlipGuard`, restoring the record to
    // `active` — the milestone stays finalizable, which is the whole point.
    if plan.promotions.is_empty() && !contributed_code {
        return blocked(
            &jigc_home,
            format,
            vec![zero_contribution_finding(milestone_id, record_changed)],
        );
    }

    // M43 — the carryover gate's milestone arm (`design/surface-contract.md` → The
    // carryover gate): `milestone create` was the shared checkout's aggregate-index
    // door, and this is where its snapshot is consumed — refuse to finalize over
    // index state staged BEFORE the milestone existed, one blocking routed finding
    // per carried path. Honest-wording bound (law 1): both aggregate channels below
    // (Combine, ChainPerSubtask) build from throwaway indexes / dedicated worktrees
    // over targeted pathspecs and land via `--ff-only`, so a live-index foreign
    // entry structurally CANNOT ride the milestone commit — the refuse is the
    // declare-at-the-boundary rule, not a leak fix, and the finding says the entry
    // STAYS STAGED across the boundary (`CarryoverBoundary::Milestone`).
    // `--carry-staged` declares it deliberate; a milestone created pre-M43 has no
    // snapshot and fails open (the declared bound). No retire exemption — only a
    // migration task retires a source. Placed after the planner (its validation /
    // base-mismatch / empty-commit blocks keep precedence, the task-finalize
    // precedent) and before either commit channel; a block here drops the
    // `RecordFlipGuard`, restoring the pre-flip record bytes.
    if !carry_staged {
        let snapshot = engine::state::read_staged_snapshot(&dir).with_context(|| {
            format!("could not read the staged snapshot for milestone `{milestone_id}`")
        })?;
        let carried = engine::finalize::decide_carryover(
            snapshot.as_ref(),
            &crate::task::git_staged_snapshot(&repo_root)?,
            None,
            // No owner-artifact exemption at the milestone boundary — it applies only at
            // `CarryoverBoundary::Task` (M45 Inc 8; the milestone owner-artifact exemption
            // is a separate, deferred concern).
            &[],
            engine::finalize::CarryoverBoundary::Milestone,
        );
        if !carried.is_empty() {
            return blocked(&jigc_home, format, carried);
        }
    }

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
    // `design/finalize.md` → Rollback discipline). `try_execute_finalize_plan` already rolls back
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
                record: record_pathspec,
            },
        )? {
            Ok(hook_output) => {
                // The boundary landed — the flipped record rode the aggregate commit; disarm
                // its restore guard so the committed `joined` bytes are not reverted.
                disarm_record_flip(&mut record_flip);
                // C2 — the landing manifest, on the mold of the per-task landed summary:
                // `finalized <sha> — <subject>` + the whole boundary's landed-file set
                // (`git diff <pre-boundary-HEAD>..HEAD`, so the N+1 chain reads as one
                // set) + the per-sub-task contribution line.
                let landed = milestone_landed_summary(
                    &repo_root,
                    &head,
                    &plan,
                    contributions,
                    &hook_output,
                )?;
                print!("{}", render::milestone_finalized(format, &landed));
                if format != Format::Json {
                    println!();
                }
                // Relay the whole chain's folded non-blocking hook output — every fan-out
                // commit (the N per-sub-task code commits + the aggregate) runs the user's
                // hooks and `chain_commit` folds their captured streams into this one string,
                // the same string the envelope above carries (one capture, two channels;
                // `design/finalize.md` → 6. Commit; the hook_output producer axis).
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
            // The chain was aborted. The `Err` is untyped, so this arm catches EVERY way the
            // boundary refuses: a per-sub-task hook rejection, the aggregate hook's, and a
            // `git merge --ff-only` refusal over ordinary untracked main-checkout WIP with no
            // hook installed at all. The chain built every commit in a dedicated worktree and
            // never fast-forwarded main, so the live checkout is untouched (HEAD at the
            // pre-finalize sha, unrelated WIP intact) and there is nothing to reset. The
            // executor already rolled back its promoted-doc copies.
            //
            // **The fan-out worktrees are NOT torn down here** (M47 Inc 3; `finalize.md` →
            // 6. Commit: *"working area intact"*). `remove_worktrees` runs
            // `git worktree remove --force`, whose safety rests on the commit having landed
            // **first** — every byte the worktree held is then already in git
            // (`design/team-ready-state.md` → Abandon refuses on a dirty worktree). On THIS
            // arm the commit did not land, and since M31 Inc 4/5 the provisioned worktree is
            // the **sole copy** of a sub-agent's staged code, so the teardown destroyed exactly
            // the work a retry needs. The `squash: true` abort sibling below already tears down
            // nothing; both arms now leave the whole working area — sub-task areas AND
            // worktrees — intact for the re-run. (A landed boundary still tears down, above;
            // an abandoned milestone is torn down by `jigc milestone discard`, which refuses on
            // a dirty worktree unless `--force`.) Surface git's stderr verbatim and exit
            // `FAILURE`.
            Err(err) => {
                // A rejected chain names ITSELF in the invocation log — the `squash: false`
                // arm's own identity, not the task door's (M47 Inc 3 T7) — and states what
                // the abort above leaves behind, while git's stderr stays verbatim.
                Ok(crate::task::surface_commit_rejection(
                    format,
                    &err,
                    &crate::task::RejectionFrame {
                        code: crate::invocation_log::ERROR_MILESTONE_CHAIN_REJECTED,
                        survived: format!(
                            "milestone:{milestone_id} is intact — nothing was committed, HEAD is \
                             at its pre-finalize commit, and every provisioned sub-task worktree \
                             still holds its staged code"
                        ),
                        rerun: milestone_finalize_rerun(milestone_id, carry_staged),
                    },
                ))
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
        match crate::task::try_execute_finalize_plan(
            &repo_root,
            &jigc_root,
            &dir,
            &plan,
            &dir,
            &schemas,
            None,
            crate::task::StagePolicy::Combine(worktrees, record_pathspec),
        )? {
            // The boundary landed. Clean up the per-sub-task working areas too (the
            // executor only removed the milestone area). A failed/rolled-back finalize
            // exits non-zero and leaves the areas intact for retry.
            Ok(hook_output) => {
                // The flipped record rode the single combine commit — disarm its restore
                // guard so the committed `joined` bytes are not reverted.
                disarm_record_flip(&mut record_flip);
                // C2 — the landing manifest (the per-task landed-summary mold): the
                // highest-stakes commit boundary must not succeed with empty stdout.
                let landed = milestone_landed_summary(
                    &repo_root,
                    &head,
                    &plan,
                    contributions,
                    &hook_output,
                )?;
                print!("{}", render::milestone_finalized(format, &landed));
                if format != Format::Json {
                    println!();
                }
                // T3 — relay the landed combine commit's non-blocking hook output (the
                // dedicated-worktree commit runs the user's hooks — M31 Inc 5).
                crate::task::relay_hook_output(format, &hook_output);
                cleanup_subtask_areas(&jigc_root, &list);
                // Tear down the fan-out worktrees on the landed default-path commit too
                // (the heavier A2 teardown — a non-blocking warning on a leaked worktree).
                remove_worktrees(&repo_root, &jigc_home, &list);
                Ok(Outcome::success())
            }
            // The commit-phase rejection: git's stderr stays verbatim-raw and the run names
            // itself in the invocation log — with the `squash: true` combine's OWN identity
            // since M47 Inc 3 T7 (it borrowed the task door's before). The failure arm is
            // inlined here so the landed arm can print the manifest. The executor already
            // rolled back its promoted-doc copies and the record flip's guard restores the
            // record, so the milestone is left exactly as the boundary found it.
            Err(err) => Ok(crate::task::surface_commit_rejection(
                format,
                &err,
                &crate::task::RejectionFrame {
                    code: crate::invocation_log::ERROR_MILESTONE_FINALIZE_REJECTED,
                    survived: format!(
                        "milestone:{milestone_id} is intact — nothing was committed, the merged \
                         docs were rolled back, and every provisioned sub-task worktree still \
                         holds its staged code"
                    ),
                    rerun: milestone_finalize_rerun(milestone_id, carry_staged),
                },
            )),
        }
    }
}

/// A transactional guard over the milestone record's `joined` flip that the finalize commit
/// folds in (`design/team-ready-state.md` → The commit model — join folds). Holds the record's
/// pre-flip bytes; on `Drop` (any early return / blocked / failed finalize) it rewrites them, so
/// a non-landed finalize leaves the committed record at its pre-flip `active` state ("Writes are
/// transactional" — the flip must not persist without the commit). The success paths call
/// [`disarm`](RecordFlipGuard::disarm) once the commit has landed, so the committed `joined`
/// bytes are kept.
///
/// **The guard covers the WORKTREE axis only.** The finalize commit path-adds the record's
/// pathspec into the **live** index (both fan-out arms, through
/// `overlay_docs_commit_and_ff`), and restoring the file's bytes does not un-stage that blob —
/// a refused boundary was left with a `joined` blob staged for a milestone that never
/// finalized. The **index** half is the executor's fifth-family capture/restore
/// ([`crate::task::StagePolicy::live_index_record_pathspecs`]; M47 Inc 3 T5,
/// `design/finalize.md` → Rollback discipline). The two halves together are the transaction.
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

/// Tear down the milestone's fan-out worktrees once the milestone actually settles — a
/// **landed** finalize (both fan-out modes) or a `discard`. Never an **aborted** finalize:
/// the removal is `--force`, which is safe only because the commit landed first, so every
/// byte the worktree held is already in git; on an abort nothing landed and the worktree is
/// the sole copy of the sub-agent's staged code (M47 Inc 3; `design/finalize.md` → 6. Commit,
/// *"working area intact"*). `discard` is the one un-landed caller, and it refuses on a dirty
/// worktree unless the human declares `--force`. For each sub-task id whose
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
///
/// **The loss is narrated before it happens** (M47 Inc 3, call (c)): the removal destroys
/// everything the boundary did not commit — the staged set landed, the rest did not — so
/// each worktree holding such work prints a non-blocking warning naming it
/// ([`discarded_work`]). Applies to every caller, including `discard --force`, where it
/// names what the human's `--force` threw away (the un-forced `discard` refuses earlier,
/// so the two never both speak). **Honest bound:** this makes the loss *visible*, not
/// *prevented* — a `discard`-style refusal on the landed path is new surface, chartered
/// to M46 (`implementation/decisions-pending.md` → the capability wave).
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
        // Name the loss BEFORE the removal (law 1 — nothing lies: a boundary that exits 0
        // must not also have silently destroyed work). Best-effort: an unreadable status
        // yields no warning and never blocks a commit that already landed.
        let discarded = discarded_work(&path).unwrap_or_default();
        if !discarded.is_empty() {
            let listing: Vec<String> = discarded
                .iter()
                .map(|work| format!("    {} ({})", work.path, work.state.label()))
                .collect();
            eprintln!(
                "warning: removing the fan-out worktree {path_str} discards work that is not in \
                 git:\n{}\n  note: the worktree is the only copy of these bytes — they are not \
                 recoverable.",
                listing.join("\n"),
            );
        }
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

/// The **zero-contribution** refusal (M47 Inc 3; `DECISIONS.md` → 2026-07-26 M47 Increment 3
/// halt resolution, call (b)(i)): a blocking, route-bearing finding for a `finalize` that would
/// land no work — nothing to promote, nothing staged in any sub-task worktree. The route names
/// **both** honest exits: give the sub-tasks a working area and do the work (the mechanical
/// span — the step a fresh clone is missing), or settle the record as abandoned via `jigc
/// milestone discard`. Keyed at `milestone:<id>`, the milestone target form
/// ([`crate::milestone`]'s sibling producers; `design/command-output-contract.md` → the
/// declared target forms).
///
/// `record_flip` is the **context the message needs to stay true**: only a `[dev ▸ methodology]`
/// project has a committed record to flip, so the "burns the terminal `joined`" clause — the
/// sharpest half of the harm — is stated exactly where it holds and omitted where it does not
/// (a context-independent statement here would render the same lie on every dev-only boundary).
fn zero_contribution_finding(milestone_id: &str, record_flip: bool) -> Finding {
    let consequence = if record_flip {
        " — the boundary would commit only jigc's own bookkeeping and flip the milestone record \
         to the terminal `joined`, after which the milestone could never be finalized again"
    } else {
        " — the boundary would commit only jigc's own bookkeeping"
    };
    Finding::graded(
        Severity::Blocking,
        "milestone.zero-contribution",
        format!(
            "milestone:{milestone_id} would land no work: no sub-task's docs promote to the \
             store, and no sub-task worktree holds staged code{consequence}"
        ),
        Some(Location::addressed(
            format!("milestone:{milestone_id}"),
            1,
            1,
        )),
        Some(engine::finding::Route::mechanical(
            ["jigc", "milestone", "provision", milestone_id],
            format!(
                " gives every sub-task a working area (a fresh clone has none; an existing one \
                 is reused); execute the sub-tasks, `git add` their work inside their worktrees, \
                 then re-run `jigc milestone finalize {milestone_id}` — or settle the milestone \
                 as abandoned with `jigc milestone discard {milestone_id}`"
            ),
        )),
    )
}

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

/// The **milestone-boundary conformance gate** (M45 — `design/finalize.md` → 2. Validate;
/// `design/validation.md` → The milestone-boundary gate; `DECISIONS.md` → 2026-07-23 M45
/// Settle, Decision 2). Validate the **merged effective state** — the by-task-id merged doc
/// set (`materialize`'s output, on disk at `staging_dir/docs/`) overlaid on the committed
/// store, plus the `doc-code` re-resolution over the merged worktree code tree — and return
/// `Some(blocked-outcome)` (exit 3, committing nothing) on any blocking finding, else `None`
/// (the boundary proceeds). This is the per-task gate's families, unchanged, with *"the
/// task's working deltas"* read as *"the merged sub-areas"* — deleting the divergence
/// [finalize.md](../../../design/finalize.md) says cannot exist, not new capability.
///
/// The families are `schema-conformance.*` + `ref-resolves` over the merged docs, plus the
/// `doc-code` code-anchor re-resolution — one validate run, never N per-area. The gate is
/// **read-only**: the loaded [`FileStateRecord`] is never persisted (the `task validate`
/// idiom — the record advances only at a landed commit). It runs BEFORE the record flip +
/// the plan + the `squash` branch, so it is knob-independent, and nothing durable is written
/// yet, so a block truly commits nothing.
///
/// The `doc-code` arm needs the merged tree **on disk** (the probe resolves anchors by
/// filesystem path while [`combine_worktree_trees`](crate::combine::combine_worktree_trees)
/// returns only a tree SHA), so the merged tree is checked out into a throwaway detached
/// worktree the probe's `working_tree_root` points at, **torn down on both exits** (the
/// `DedicatedWorktree` drop). A same-path collision yields no single merged tree — that is
/// the join/knob-branch's concern, not the conformance gate's, so it falls through (`None`).
#[allow(clippy::too_many_arguments)]
fn milestone_boundary_gate(
    repo_root: &Path,
    jigc_home: &Path,
    jigc_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    staging_dir: &Path,
    base: &BasePin,
    worktrees: &[PathBuf],
    format: Format,
) -> Result<Option<Outcome>> {
    // Fold the N worktree-staged code-sets onto the milestone's shared base (`base.sha`, NOT
    // `HEAD`) into ONE merged tree — off-line (touches neither the live index nor the
    // worktree), which is what makes it callable from a gate that may block. A same-path
    // collision builds no tree; leave it to the branch below and proceed.
    let tree = match crate::combine::combine_worktree_trees(repo_root, &base.sha, worktrees)? {
        crate::combine::CombineOutcome::Combined(sha) => sha,
        crate::combine::CombineOutcome::Blocked(_) => return Ok(None),
    };

    // The merged docs the gate validates are the **persisted** ones — those with a committed
    // home. A sub-task's transient `commit:<id>` skeleton is materialized into `merged/docs/`
    // too, but it is NOT part of the committed merged state (it renders into a message,
    // validated at its own render/finalize path, and is unused under the `squash: true`
    // synthesized aggregate), so validating it would false-block a clean milestone whose
    // aggregate never uses it. Copy the persisted subset into a fresh scratch staging area and
    // gate over that; the promote plan below still reads the full `staging_dir`.
    let gate_staging = crate::task::ScratchTree::new();
    let gate_docs = gate_staging.path().join("docs");
    std::fs::create_dir_all(&gate_docs)
        .with_context(|| format!("could not open the gate staging area {gate_docs:?}"))?;
    let src_docs = staging_dir.join("docs");
    for entry in std::fs::read_dir(&src_docs)
        .with_context(|| format!("could not read the merged docs dir {src_docs:?}"))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|x| x.to_str()) != Some("md") {
            continue;
        }
        let Some(ty) = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|stem| stem.split(':').next().unwrap_or(stem))
        else {
            continue;
        };
        // Persisted iff the doctype declares a committed home (`location:` or `placement:`)
        // — the transient-sink `commit` type declares neither.
        let persisted = schemas
            .get(ty)
            .is_some_and(|s| s.location.is_some() || s.placement.is_some());
        if persisted {
            std::fs::copy(&path, gate_docs.join(entry.file_name()))
                .with_context(|| format!("could not stage {path:?} into the gate area"))?;
        }
    }

    // Check the merged tree out into a throwaway detached worktree — the `doc-code` probe's
    // `working_tree_root`. Torn down on drop (this blocked path AND the clean fall-through).
    let merged_wt = crate::task::checkout_tree_worktree(repo_root, &base.sha, &tree)?;

    // The merged code change-set (`base.sha`..merged tree) drives the code-anchor blast
    // radius — a COMMITTED doc's anchor whose target file a worktree changed is re-resolved
    // here (the cross-worktree class no per-area check can see). `base_tree` is the `base.sha`
    // version of those files: the "resolved at base" side of the newly-dangled comparison.
    let changed_code = merged_changed_code(repo_root, &base.sha, &tree)?;
    let base_tree = crate::task::materialize_treeish_subset(repo_root, &base.sha, &changed_code)?;

    // The read-only determinism-boundary feeds the shared `validate_task` entry needs — the
    // record is loaded but never persisted, and the severity cascade + tracked-status
    // predicate + `doc-code` invoker are byte-identical to the per-task gate.
    let mut record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;
    let pack = make_pack()?;
    let cascade = crate::start::resolve_severity_cascade(
        pack.as_ref(),
        &jigc_home.join(".jigc").join("config"),
    )?;
    let untracked: std::collections::HashSet<String> = crate::task::git_untracked_all(repo_root)?
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect();
    let tracked = move |path: &str| !untracked.contains(path);
    // The file-state history gate (M45, Decision 7): a dangling baseline with no HEAD history
    // is graded advisory, a genuine deletion keeps blocking. Built from `repo_root` via the
    // same shell-free-seam `git log` helper the per-task gate uses; a `git` failure defaults
    // history-present (conservative — keep blocking).
    let repo_root_for_history = repo_root.to_path_buf();
    let history = move |path: &str| {
        crate::task::git_path_has_history(&repo_root_for_history, path).unwrap_or(true)
    };

    // ONE validate over the persisted merged docs (the filtered `gate_staging` area) + the
    // merged code root — the committed edge index is keyed to the shared base (`base.sha`),
    // the committed-store reads bind to `jigc_home`, the code anchors resolve against the
    // merged-tree worktree.
    let report = engine::validate::validate_task(
        gate_staging.path(),
        schemas,
        &mut record,
        jigc_home,
        merged_wt.path(),
        jigc_root,
        &base.sha,
        &cascade,
        &crate::task::doc_code_invoker,
        &tracked,
        &history,
        &changed_code,
        base_tree.path(),
        // The merged gate area belongs to no single task, so the conflict route cannot name
        // one (M47 inc-2 / T4): it routes at the sub-task listing the human picks from,
        // never at a `<task-id>` placeholder.
        &engine::file_state::ConflictBlock::new(
            "an external edit and a joined sub-task's staged writes both changed it",
            engine::finding::Route::mechanical(
                ["jigc", "task", "list"],
                " names the live tasks — discard the sub-task that staged this doc, or revert \
                 the external edit on disk, then re-run the join",
            ),
        ),
    )
    .with_context(|| format!("validating the merged effective state under {staging_dir:?}"))?;

    if report.has_blocking() {
        // `blocked()` re-applies the cascade (idempotent) and renders the pinned envelope —
        // exit 3. `merged_wt` + `base_tree` drop on return (the worktree/scratch teardown).
        return Ok(Some(blocked(jigc_home, format, report.findings.to_vec())?));
    }
    Ok(None)
}

/// The merged code change-set — the repo-relative paths that differ between the milestone's
/// shared base (`base_sha`) and the off-line-combined merged `tree`, feeding the boundary
/// gate's code-anchor blast radius (the [`crate::task::git_staged_paths`] `-z` NUL-parse
/// idiom, keyed on two tree-ishes rather than the cached index — `--no-renames` so a rename
/// reports as its delete + add halves, path-stable).
fn merged_changed_code(
    repo_root: &Path,
    base_sha: &str,
    tree: &str,
) -> Result<std::collections::BTreeSet<String>> {
    let out = Command::new("git")
        .args(["diff", "--name-only", "-z", "--no-renames", base_sha, tree])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git diff --name-only -z {base_sha} {tree}` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    // `-z` NUL-terminates each entry (the last too), so the trailing split yields an empty
    // string — filtered. No `trim()`: a path's own whitespace is significant.
    Ok(String::from_utf8(out.stdout)
        .context("`git diff --name-only` produced non-UTF-8 output")?
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect())
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

/// Each sub-task's landing-manifest contribution (C2), id-sorted over the milestone's
/// full task `list`: its merged-doc count from the materialize's address→source map
/// (`sources`), its staged-code file count from its still-provisioned fan-out
/// worktree's index (`git diff --cached --name-only`), and **whether it has a
/// provisioned worktree at all**. Computed **pre-commit** — a landed boundary tears the
/// worktrees down.
///
/// A never-provisioned sub-task counts 0 code files **by construction**, not by
/// measurement: since M31 Inc 4/5 the isolated worktree is the sole place a sub-agent's
/// code can live, so no worktree means no code was possible. That degrade used to be
/// indistinguishable from "the sub-agent staged nothing," which is why `provisioned`
/// rides the manifest (M47 Inc 3, call (b)(ii)).
fn subtask_contributions(
    list: &engine::milestone::TaskList,
    sources: &std::collections::BTreeMap<String, String>,
    worktrees: &[PathBuf],
) -> Result<Vec<render::SubTaskContribution>> {
    let mut out = Vec::new();
    for id in list.enumerate() {
        let docs = sources.values().filter(|source| **source == id).count();
        let worktree = worktrees
            .iter()
            .find(|path| path.file_name().is_some_and(|name| name == id.as_str()));
        let code_files = match worktree {
            Some(path) => worktree_staged_file_count(path)?,
            None => 0,
        };
        // The loss half of the same pre-commit snapshot (M47 Inc 3, call (c)) — the
        // teardown removes the whole checkout, so everything the worktree holds beyond
        // the staged set dies with it. Read here, where the worktrees are still alive
        // and `code_files` is read, so the manifest's "landed" and "lost" halves come
        // from one observation of one state.
        let discarded = match worktree {
            Some(path) => discarded_work(path)?,
            None => Vec::new(),
        };
        out.push(render::SubTaskContribution {
            id,
            docs,
            code_files,
            provisioned: worktree.is_some(),
            discarded,
        });
    }
    Ok(out)
}

/// The number of files staged in a fan-out worktree's index (`git diff --cached
/// --name-only`, non-empty lines) — the code-file half of a sub-task's contribution.
fn worktree_staged_file_count(worktree: &Path) -> Result<usize> {
    let out = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(worktree)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git diff --cached --name-only` in worktree {worktree:?} failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count())
}

/// The work in a fan-out worktree the teardown **destroys** — path-sorted, empty when
/// everything the worktree holds is staged (M47 Inc 3, call (c); `DECISIONS.md` →
/// 2026-07-26 M47 Increment 3 halt resolution).
///
/// The boundary commits only [`worktree_staged_patch`] and then `git worktree remove
/// --force`s the whole checkout, so the difference between those two sets is destroyed at
/// exit 0. The probe is `git status --porcelain`, the same union its
/// [`dirty_worktrees`] sibling reads (staged, unstaged and untracked alike; an ignored
/// file is not work and never appears) — **partitioned on the index column** so the
/// narration stays true:
///
/// * index column set, worktree column clean (`A `, `M `, `R `) — **wholly staged**: every
///   byte is in the patch the boundary commits, so it is *not* reported. Reporting it
///   would be an over-report, which is what makes a loss warning untrustworthy.
/// * index column clear (`??`, ` M`, ` D`) — **never staged**: the commit carries none of
///   it.
/// * both columns set (`MM`, `AM`, an unmerged `UU`) — **partly staged**: the commit
///   carries the indexed version and the worktree's later edit dies.
///
/// **`--porcelain -z`, not `--porcelain`.** Git display-quotes a path holding a space,
/// a quote or a non-ASCII byte **regardless of `core.quotePath`**, and a fan-out worktree
/// holds arbitrary user code — so the plain form would name a path the repo does not
/// contain. The `-z` form emits verbatim paths in NUL-terminated records; a rename/copy
/// entry appends its origin path as an extra record, consumed here so the stream stays
/// aligned.
///
/// **`--untracked-files=all`, not git's default collapse.** The default reports a wholly
/// untracked directory as the single entry `notes/`, naming a *directory* where this
/// surface promises the paths being destroyed. A loss narration read after the fact
/// enumerates the files (an ignored file still never appears).
fn discarded_work(worktree: &Path) -> Result<Vec<render::DiscardedWork>> {
    let out = Command::new("git")
        .args(["status", "--porcelain", "-z", "--untracked-files=all"])
        .current_dir(worktree)
        .output()
        .context("could not run `git status` (is git on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git status --porcelain -z --untracked-files=all` in worktree {worktree:?} failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }

    let mut records = out.stdout.split(|byte| *byte == 0);
    let mut discarded = Vec::new();
    while let Some(record) = records.next() {
        // `XY <path>` — two status columns, one space, then the verbatim path.
        if record.len() < 4 {
            continue;
        }
        let index = record[0];
        let tree = record[1];
        let path = String::from_utf8_lossy(&record[3..]).into_owned();
        if index == b'R' || index == b'C' {
            // A rename/copy carries its origin path as the next record — consume it so
            // the origin is never mistaken for a status entry.
            let _ = records.next();
        }
        let state = if index == b' ' || index == b'?' {
            render::DiscardState::NeverStaged
        } else if tree == b' ' {
            // Wholly staged — the boundary commits it.
            continue;
        } else {
            render::DiscardState::PartlyStaged
        };
        discarded.push(render::DiscardedWork { path, state });
    }
    discarded.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(discarded)
}

/// Assemble the landed-boundary facts for [`render::milestone_finalized`] (C2), read
/// off the **committed bytes** after the boundary landed: the abbreviated HEAD hash +
/// subject, and the landed-file manifest as `git diff --name-status --no-renames
/// <pre-boundary-HEAD> HEAD` — the whole boundary's delta, so the `squash: false` N+1
/// chain reads as one set, identical to what the single `squash: true` commit shows. A
/// path among the plan's promotion destinations tags `Promoted`; otherwise the status
/// letter maps `A`→added / `D`→deleted / else modified (the
/// `crate::task::classify_landed_manifest` mapping).
fn milestone_landed_summary(
    repo_root: &Path,
    pre_boundary_head: &str,
    plan: &engine::finalize::FinalizePlan,
    sub_tasks: Vec<render::SubTaskContribution>,
    hook_output: &str,
) -> Result<render::MilestoneLanded> {
    let hash = crate::task::git_capture(repo_root, &["rev-parse", "--short", "HEAD"])?;
    let subject = crate::task::git_capture(repo_root, &["log", "-1", "--pretty=format:%s"])?;
    let name_status = crate::task::git_capture(
        repo_root,
        &[
            "diff",
            "--name-status",
            "--no-renames",
            pre_boundary_head,
            "HEAD",
        ],
    )?;
    let promoted: std::collections::HashSet<&str> = plan
        .promotions
        .iter()
        .map(|promotion| promotion.destination.as_str())
        .collect();
    let mut manifest = Vec::new();
    for line in name_status.lines() {
        let Some((status, path)) = line.trim().split_once('\t') else {
            continue;
        };
        let path = path.trim().to_owned();
        let kind = if promoted.contains(path.as_str()) {
            render::ManifestKind::Promoted
        } else {
            match status.trim().chars().next().unwrap_or('M') {
                'A' => render::ManifestKind::Added,
                'D' => render::ManifestKind::Deleted,
                _ => render::ManifestKind::Modified,
            }
        };
        manifest.push(render::ManifestEntry { path, kind });
    }
    Ok(render::MilestoneLanded {
        hash,
        subject,
        files: manifest.len(),
        manifest,
        sub_tasks,
        // M45 — the SAME captured boundary-commit hook string the stderr relay carries at
        // the call site (one capture, two channels; `design/command-output-contract.md` →
        // Stream discipline). The join path folds into this same envelope.
        hook_output: hook_output.to_owned(),
    })
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

/// Map an engine [`Finding`] to an `anyhow` error carrying its **key** + message +
/// route — the `severity · code — message` line shape the findings envelope prints
/// (round-2 D6h: a milestone finding surfaced through the operational-error funnel
/// used to drop its stable `(code, target)` key, so `milestone.no-criteria` — unlike
/// every envelope-rendered sibling — was undiscriminable to a driver; the funnel now
/// carries the key for every milestone finding it converts).
fn finding_to_err(finding: Finding) -> anyhow::Error {
    let severity = match finding.severity {
        engine::finding::Severity::Blocking => "blocking",
        engine::finding::Severity::Warning => "warning",
        engine::finding::Severity::Advisory => "advisory",
    };
    let head = format!("{severity} · {} — {}", finding.code, finding.message);
    match finding.route {
        Some(route) => anyhow::anyhow!("{head}\n  route: {route}"),
        None => anyhow::anyhow!("{head}"),
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
