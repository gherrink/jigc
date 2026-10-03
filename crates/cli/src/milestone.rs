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
//!
//! # The root a workbench path is spelled against
//!
//! **`jigc_home`, never `repo_root`** (the independent review of `3c71da87`, MEDIUM 1;
//! 2026-09-22). Every path this module prints is a *workbench* path — a fan-out worktree, a
//! task or milestone area, the merged staging tree, `.jigc/` itself — or a scratch tree
//! outside the repository entirely, for which [`crate::render::repo_relative`]'s absolute
//! fallback is the honest answer whichever root it is handed. The workbench hangs off
//! `jigc_home`; `repo_root` is the checkout the **command was typed in**, and the two
//! coincide only while that checkout is not a linked worktree.
//!
//! Every door here binds both side by side, which is what let one rule ship in two
//! spellings: six sites already rendered against `jigc_home` (as `task.rs` does at every one
//! of its own, passing `task.jigc_home`) while twenty passed `repo_root`. Driven from inside
//! a sibling sub-task's worktree at `3c71da87`, `repo.operation-in-progress` and
//! `milestone.dirty-worktree` each put this machine's absolute path into the message, the
//! route **and** the pinned `(code, target)` key — law 1 (`design/surface-contract.md`),
//! whose one home is `render::repo_relative` and whose root is the caller's to get right.
//! `commit_seam_posture.rs`'s arm (h) reads the rule off this file's source; arm (g) drives
//! two of its members.
//!
//! ~~`crate::setup` is **out of this class**, not unswept: it binds no `jigc_home` at all —
//! its `repo_root` is the root it joins every `.jigc/` path off — so the two cannot diverge
//! inside that file. Whether `jigc uninstall` should bind `jigc_home` instead is a different
//! question about a different door.~~ **Struck 2026-09-23** (the cwd fixes' review, LOW 5 /
//! LOW 10). The first clause described the source correctly and disposed of it wrongly: a
//! file that binds *only* the standing checkout cannot diverge from itself, and is exactly
//! the file that acts on the wrong repository from a worktree. Falsifying datum, driven:
//! `jigc setup` from a linked worktree installed a worktree-local `.jigc/` nothing reads,
//! and `jigc uninstall` from a fan-out worktree removed the repository-wide `pre-commit`
//! hook at exit 0 while printing *"repo-local install removed"* over a main-checkout
//! install it left whole — with all four of its WIP guards inert, because they were handed
//! a checkout whose `.jigc/worktrees/` and `.jigc/tasks/` do not exist. Both doors now bind
//! [`crate::repo::jigc_home`] (`crate::setup`'s module header).

use crate::cli::Format;
use crate::invocation_log::Outcome;
use crate::pack::make_pack;
use crate::render;
use crate::repo::discover_repo_root;
use crate::task::git_head;
use anyhow::{Context, Result, bail};
use engine::data_value::SubTask;
use engine::file_state::{FileStateRecord, hash_bytes, reconcile_committed};
use engine::finalize::plan_milestone_finalize;
use engine::finding::{Finding, Location, Route, Severity};
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

/// The `jigc milestone create` long help. The work-unit half of the **argument
/// convention** — the verb takes its title as a positional where `jigc doc create`
/// takes a `--title` flag, and until M47 neither help said why, so an agent that had
/// met one verb read the other as an inconsistency (rc.9 trial, N20). Both sites
/// render the one [`crate::cli::ARGUMENT_CONVENTION`] statement, so the pair cannot
/// drift into describing two rules.
///
/// **It also says that it commits** (M52 Increment 10 / T2, D-2). This help volunteered
/// *"opening its **gitignored** area"* and nothing else, which an adopter reads as
/// *nothing is committed* — while the door writes the milestone's committed record and
/// moves `HEAD`. The claim is the door's own
/// [`MILESTONE_CREATE_COMMITS`](crate::invocation_log::MILESTONE_CREATE_COMMITS) clause,
/// rendered from the axis rather than retyped, and the misleading half is **corrected**
/// rather than merely extended: the gitignored subject is now named as the *working area*,
/// which is what is actually gitignored.
fn create_long_about() -> String {
    format!(
        "Mint a milestone work-unit (id = frozen slug from the title) with one shared \
         base pinned at HEAD and an empty task list.\n\n\
         Only the working area under `.jigc/` is gitignored — the milestone's record is \
         not. Where the composed cascade resolves the `milestone-record` doctype, this \
         door {}: it writes that record under docs-root, `HEAD` moves, and the commit is \
         path-scoped to the record, so anything else you had staged stays staged. A \
         project whose cascade resolves no such doctype opens the working area and \
         commits nothing.\n\n\
         {}",
        crate::invocation_log::MILESTONE_CREATE_COMMITS,
        crate::cli::ARGUMENT_CONVENTION,
    )
}

/// The `jigc milestone add-task` long help — **the door says that it commits** (M52
/// Increment 10 / T2, D-2). It named the gitignored task list and stopped, while under a
/// `[dev ▸ methodology]` project it appends to the milestone's committed record and lands a
/// record-only commit; driven, the door moved `HEAD` with neither its help nor its ack
/// saying so (`completions/artifacts/M52/baseline-surfaces.md` §4.4). The claim renders the
/// door's own [`MILESTONE_ADD_TASK_COMMITS`](crate::invocation_log::MILESTONE_ADD_TASK_COMMITS)
/// clause off the committing-door axis.
fn add_task_long_about() -> String {
    format!(
        "Mint a sub-task under an existing milestone (pinned to the milestone's shared \
         base, isolated `tasks/<sub>/` area) and append it to the task list.\n\n\
         Where the composed cascade resolves the `milestone-record` doctype, this door \
         also {}: `HEAD` moves, the commit is path-scoped to the record so anything else \
         you had staged stays staged, and the ack names the sha it landed. A project whose \
         cascade resolves no such doctype appends to the gitignored task list and commits \
         nothing.",
        crate::invocation_log::MILESTONE_ADD_TASK_COMMITS,
    )
}

/// The `jigc milestone add-from-spec` long help — **the door says that it commits** (M52
/// Increment 10 / T2, D-2). Its two "committed" hits were both *"a committed spec"*, so a
/// literal scan read as truthful while the door was as silent as its `add-task` sibling. It
/// lands **one record-only commit per seeded sub-task**, which is what the door's own
/// [`MILESTONE_ADD_FROM_SPEC_COMMITS`](crate::invocation_log::MILESTONE_ADD_FROM_SPEC_COMMITS)
/// clause says and what its ack now enumerates.
fn add_from_spec_long_about() -> String {
    format!(
        "Seed a milestone's task list from a committed spec: mint one sub-task per \
         repeatable `criterion` of the spec (criterion text as intent). A spec with zero \
         criteria blocks with `milestone.no-criteria` (\"nothing to seed from\").\n\n\
         Where the composed cascade resolves the `milestone-record` doctype, every seeded \
         sub-task is appended to the milestone's committed record and this door {}: `HEAD` \
         moves once per seed, each commit is path-scoped to the record, and the ack names \
         every sha it landed. A project whose cascade resolves no such doctype seeds the \
         gitignored task list and commits nothing.",
        crate::invocation_log::MILESTONE_ADD_FROM_SPEC_COMMITS,
    )
}

/// The `jigc milestone <verb>` subcommand tree.
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum MilestoneCommand {
    /// Mint a milestone work-unit (id = frozen slug from the title) with one shared
    /// base pinned at HEAD and an empty task list.
    #[command(long_about = create_long_about())]
    Create {
        /// The milestone title, taken as a positional — the work-unit form of the
        /// argument convention (`design/write-commands.md` → The argument
        /// convention). Slugged into the `milestone:<slug>` id.
        title: String,
    },
    /// Mint a sub-task under an existing milestone (pinned to the milestone's
    /// shared base, isolated `tasks/<sub>/` area) and append it to the task list.
    #[command(long_about = add_task_long_about())]
    AddTask {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// The sub-task intent — slugged into the sub-task id.
        intent: String,
        /// The minting workflow recorded for the sub-task (read back on re-entry,
        /// which asserts equality against it). Defaults to `sub-task`. Must name a
        /// workflow the loaded packs provide — an id they do not is refused before
        /// anything mints, naming the set.
        #[arg(long, default_value = DEFAULT_SUB_TASK_WORKFLOW)]
        workflow: String,
    },
    /// Seed a milestone's task list from a committed spec: mint one sub-task per
    /// repeatable `criterion` of the spec (criterion text as intent). A spec with
    /// zero criteria blocks with `milestone.no-criteria` ("nothing to seed from").
    #[command(long_about = add_from_spec_long_about())]
    AddFromSpec {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// The committed spec's address (`spec:<slug>`) to enumerate criteria from.
        spec_addr: String,
        /// The minting workflow recorded for every seeded sub-task (read back on
        /// re-entry, which asserts equality against it). Defaults to `sub-task`. Must
        /// name a workflow the loaded packs provide — an id they do not is refused
        /// before anything mints or seeds, naming the set.
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
    /// an isolated code checkout. Idempotent — reuses a live worktree untouched, and
    /// clears an **empty** leftover directory. A leftover that holds anything
    /// **refuses**: nothing can prove those bytes are disposable (a copied or moved
    /// repo's live worktrees land here), so the refusal names what would be deleted and
    /// `--force` is the consent to delete it. Run as a `Run:` step before the fan-out
    /// (`design/storage.md` → repository layout).
    Provision {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// Delete a leftover directory at a sub-task's worktree path even when it holds
        /// content — the explicit consent to destroy it (without this, a non-empty
        /// leftover refuses). Inert when every path is empty or a live worktree.
        #[arg(long)]
        force: bool,
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
    /// The milestone commit boundary — run the by-task-id join, then land BOTH halves
    /// of the fan-out as one logical boundary with a CLI-synthesized message: the
    /// join's suffix-resolved doc bodies, materialized into the parent staging area,
    /// and the code staged in each sub-task worktree, folded in by task id. A blocking
    /// join finding (a same-doc clash, an unknown milestone) routes to stderr and
    /// commits nothing.
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
    /// registered fan-out worktrees, and `.jigc/milestones/<id>/`). Refuses with
    /// `milestone.dirty-worktree` when any sub-task's path under `.jigc/worktrees/`
    /// holds content nothing can prove is disposable — a live worktree carrying
    /// uncommitted work, and equally a path this repository has not registered as a
    /// worktree (a `cp -R` or `mv` of the repo leaves the copy's worktrees registered
    /// at the source's path), whose contents no git here vouches for, committed or
    /// not. Refuses with `milestone.staged-prose` when a sub-task's working area under
    /// `.jigc/tasks/` stages a doc no commit has a copy of — a subject no worktree
    /// contains, so the worktree probe reads clean over it. Refuses with
    /// `milestone.foreign-bytes` when any of those areas, or `.jigc/milestones/<id>/`
    /// itself, holds a file jigc did not write: the whole tree is gitignored, so nothing
    /// else has a copy of it either. Get that content out and re-run, or pass `--force`,
    /// the single consent for all three guards.
    Discard {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// Settle the record and tear the workbench down even when a sub-task worktree
        /// path holds content, a sub-task's working area stages a doc no commit has a
        /// copy of, or the workbench holds files jigc did not write — the explicit
        /// consent for all three guards, and what it costs differs by path: a worktree
        /// this repository has registered is removed with everything uncommitted in it,
        /// a path registered nowhere is left orphaned on disk for you to deal with, and
        /// a staged doc or a foreign file under `.jigc/` is gone for good. Inert when
        /// the three guards are already clean; it never buys silence — the loss is
        /// narrated either way.
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
        // The work-unit id guard, at the door rather than at the seam it protects (M50
        // Inc 1 / T1). `milestone_dir` is a pure join, and every operating verb reaches
        // the shared `reseed_cache` — which reads the committed record and REBUILDS both
        // the milestone cache area and the sub-task working areas — *before* its own
        // `is_dir()` check. Driven at `HEAD~`, `jigc milestone execute "probe-"`
        // materialized both from a record filed under that malformed slug and composed at
        // exit 0. So the ask sits above all of it, where the token arrives.
        if let Some(id) = self.milestone_id()
            && let Err(err) = crate::task::reject_malformed_work_unit_id(id)
        {
            return crate::invocation_log::operational_failure(format, &err);
        }
        // **Is this a repository jigc was ever installed into?** — one ask for the whole
        // family, above every arm including `execute`'s, because a second reader is the
        // class re-opening (M52 Inc 8 / T1). Driven at `HEAD~` over a bare repo, six of
        // the nine doors ran at exit 0 — `provision` registering a real detached worktree
        // in `.git/worktrees/` — and `add-from-spec` resolved a docs-root path in an
        // unconfigured repo to refuse with `store.not-found`; only `execute` asked. The
        // whole arc then lived in a gitignored `.jigc/` no clone sees, which inverts
        // `design/team-ready-state.md`'s settlement that the committed record is the
        // source of truth. The answer is the SHIPPED `locate::not_set_up()` on the
        // `{error}` arm — the one every other door requiring the project layer already
        // gives (21 of the 47 leaves, measured at this wave's baseline, before this
        // change) — a deliberate consistency with the surface rather than an omission,
        // and no new finding code (`completions/artifacts/M52/settle-record.md` §13).
        //
        // It sits BELOW the malformed-id guard above and above everything else: that
        // guard is about the token the caller typed, and moving it would change a set-up
        // repository's answer to buy nothing.
        if let Err(err) = crate::migrate_corpus::require_project_layer(cwd) {
            return crate::invocation_log::operational_failure(format, &err);
        }
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
        // Which `--format json` arm this verb's success takes, read off `VERB_KINDS` —
        // the classification that already governs the whole clap tree — rather than
        // hand-cased on the variant (M51 Inc 5 / T3). A `Read` leaf runs no commit, so no
        // hook can ever speak into its envelope and `hook_output` is OMITTED; every other
        // leaf keeps the present-always key. Captured here, before the match consumes
        // `self`. An unclassified path cannot occur — `VERB_KINDS` is fenced total against
        // the clap tree (`cli_parse::every_leaf_verb_is_classified`) and `verb_path` is
        // matched exhaustively — and would take the committing arm, which is the status
        // quo rather than a new silence.
        let carries_hook_output = !matches!(
            crate::cli::verb_kind(self.verb_path()),
            Some(crate::cli::VerbKind::Read)
        );
        // Each committing verb returns `(summary, hook_output)` — the record-only
        // commit's captured non-blocking hook stream (the hook_output producer axis;
        // `design/command-output-contract.md` → Stream discipline). The read-only verbs
        // commit nothing, so their stream is the empty string (present-always).
        // The record-only doors' rollback conflicts, gathered on the failure arm and printed
        // beside the door's own frame (M52 Increment 5 / T4). Empty on every success and on
        // every read-only verb, so the `Err` arm below carries it unconditionally.
        let mut conflicts: Vec<Finding> = Vec::new();
        let result = match self {
            MilestoneCommand::Create { title } => run_create(cwd, &title, &mut conflicts),
            MilestoneCommand::AddTask {
                milestone_id,
                intent,
                workflow,
            } => run_add_task(cwd, &milestone_id, &intent, &workflow, &mut conflicts),
            MilestoneCommand::AddFromSpec {
                milestone_id,
                spec_addr,
                workflow,
            } => run_add_from_spec(
                cwd,
                format,
                &milestone_id,
                &spec_addr,
                &workflow,
                &mut conflicts,
            ),
            MilestoneCommand::ListTasks { milestone_id } => {
                run_list_tasks(cwd, &milestone_id).map(|summary| (summary, String::new()))
            }
            MilestoneCommand::Provision {
                milestone_id,
                force,
            } => run_provision(cwd, &milestone_id, force).map(|summary| (summary, String::new())),
            MilestoneCommand::Discard {
                milestone_id,
                force,
            } => run_discard(cwd, &milestone_id, force, &mut conflicts),
            MilestoneCommand::Execute { .. } => unreachable!("`Execute` is handled above"),
            MilestoneCommand::Join { .. } => unreachable!("`Join` is handled above"),
            MilestoneCommand::Finalize { .. } => unreachable!("`Finalize` is handled above"),
        };
        match result {
            Ok((summary, hook_output)) => {
                println!(
                    "{}",
                    render::milestone(
                        format,
                        &summary,
                        carries_hook_output.then_some(hook_output.as_str()),
                    )
                );
                // One capture, two channels: the same string rides the envelope above and
                // the delimited relay (stderr under `--format json`, stdout on agent-text).
                crate::task::relay_hook_output(format, &hook_output);
                Outcome::success()
            }
            Err(err) => match &frame {
                // A record-only door: a hook rejection is framed with what survived + this
                // door's own re-run, and names itself in the log. Every other failure keeps
                // the plain operational-error envelope.
                Some(frame) => {
                    crate::task::surface_commit_rejection(format, &err, frame, &conflicts)
                }
                // A read-only verb — it runs no commit, so no `CommitRejected` can reach here.
                None => crate::invocation_log::operational_failure(format, &err),
            },
        }
    }

    /// This verb's **leaf path** as [`crate::cli::VERB_KINDS`] spells it — the key the
    /// dispatch site looks its envelope arm up under (M51 Inc 5 / T3).
    ///
    /// It returns the path rather than a [`crate::cli::VerbKind`] on purpose: the kind is
    /// decided once, for the whole clap tree, in the table that a fence already holds
    /// total against it. A second hand-written classification here could disagree with the
    /// first, and the surface would then answer two ways about the same verb.
    ///
    /// Matched exhaustively on purpose, on [`MilestoneCommand::milestone_id`]'s mold: a
    /// verb added without a path here does not compile. Every row is driven through the
    /// real binary by `tests/milestone_envelope_arm.rs`, whose door table bijects the
    /// `milestone` leaves of `VERB_KINDS`.
    fn verb_path(&self) -> &'static [&'static str] {
        match self {
            MilestoneCommand::Create { .. } => &["milestone", "create"],
            MilestoneCommand::AddTask { .. } => &["milestone", "add-task"],
            MilestoneCommand::AddFromSpec { .. } => &["milestone", "add-from-spec"],
            MilestoneCommand::ListTasks { .. } => &["milestone", "list-tasks"],
            MilestoneCommand::Provision { .. } => &["milestone", "provision"],
            MilestoneCommand::Execute { .. } => &["milestone", "execute"],
            MilestoneCommand::Join { .. } => &["milestone", "join"],
            MilestoneCommand::Finalize { .. } => &["milestone", "finalize"],
            MilestoneCommand::Discard { .. } => &["milestone", "discard"],
        }
    }

    /// The caller-supplied milestone id this verb carries, or `None` for the one verb that
    /// takes none ([`MilestoneCommand::Create`] mints its id from a title).
    ///
    /// Matched exhaustively on purpose: a verb added with an id it forgot to declare here
    /// does not compile, so the guard's subject cannot silently shrink.
    fn milestone_id(&self) -> Option<&str> {
        match self {
            MilestoneCommand::Create { .. } => None,
            MilestoneCommand::AddTask { milestone_id, .. }
            | MilestoneCommand::AddFromSpec { milestone_id, .. }
            | MilestoneCommand::ListTasks { milestone_id }
            | MilestoneCommand::Provision { milestone_id, .. }
            | MilestoneCommand::Execute { milestone_id }
            | MilestoneCommand::Join { milestone_id }
            | MilestoneCommand::Finalize { milestone_id, .. }
            | MilestoneCommand::Discard { milestone_id, .. } => Some(milestone_id),
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
        let (code, target, survived, rerun) = match self {
            MilestoneCommand::Create { title } => (
                crate::invocation_log::ERROR_MILESTONE_CREATE_REJECTED,
                format!("milestone:{}", engine::milestone::mint_id(title)),
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
                format!("milestone:{milestone_id}"),
                format!(
                    "nothing was committed — the record append and the sub-task mint were both \
                     rolled back, so milestone:{milestone_id} is unchanged"
                ),
                format!(
                    "jigc milestone add-task {} {}{}",
                    shell_token(milestone_id),
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
                format!("milestone:{milestone_id}"),
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
                    "jigc milestone add-from-spec {} {}{}",
                    shell_token(milestone_id),
                    shell_token(spec_addr),
                    workflow_flag(workflow),
                ),
            ),
            MilestoneCommand::Discard {
                milestone_id,
                force,
            } => (
                crate::invocation_log::ERROR_MILESTONE_DISCARD_REJECTED,
                format!("milestone:{milestone_id}"),
                format!(
                    "nothing was committed — milestone:{milestone_id}'s record is still at its \
                     pre-discard state and its workbench is untouched"
                ),
                format!(
                    "jigc milestone discard {}{}",
                    shell_token(milestone_id),
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
            // The work-unit ref — the form the contract's `*.commit-rejected` row declares for
            // a door whose subject is a work unit (M52 Increment 1 / T1).
            target,
            survived,
            // Each of the four clauses describes what `commit_record_transaction`'s rollback
            // leaves behind, which is the same state on both cells of the axis.
            survived_non_hook: None,
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
        "jigc milestone finalize {}{}",
        crate::task::shell_token(milestone_id),
        if carry_staged { " --carry-staged" } else { "" },
    )
}

/// The ` --workflow <id>` suffix a re-run needs, empty when the door ran on the recorded
/// default (the flag `clap` fills in when it is absent).
fn workflow_flag(workflow: &str) -> String {
    if workflow == DEFAULT_SUB_TASK_WORKFLOW {
        String::new()
    } else {
        format!(" --workflow {}", crate::task::shell_token(workflow))
    }
}

/// The **resolve seam** the CLI-owned milestone verbs share (`list-tasks` / `provision` /
/// `execute` / `finalize` — each rejecting an absent `.jigc/milestones/<id>/` workbench;
/// M43 T7, `design/surface-contract.md` → The route fence, closing paragraph). The
/// quoted-title span rides the checked [`engine::finding::Route::mechanical`] constructor
/// inside [`engine::milestone::create_milestone_route`], so the CLI-seam parse fence asserts
/// it parses against the real CLI. `discard` calls the engine seam directly with its own
/// route (check the id, don't mint — a discard should never route to *creating* the
/// milestone).
///
/// **The identity is the engine's own, not a composed string** (M51 Increment 6 / T2).
/// `design/command-output-contract.md` declares a **work-unit** target form — `task:<id>` /
/// `milestone:<id>` — and this condition's finding has carried `milestone:<id>` at the
/// engine-raising doors since its first commit, which is a promise that `(code, target)`
/// resolves. At these four doors it resolved at none: the refusal was an `anyhow!` string,
/// so `--format json` answered the flattened `{"error": …}` with no code at all, and one
/// condition shipped **two** wire shapes. The identity is minted once, by the crate that
/// owns the condition, and travels on the envelope-projecting carrier.
///
/// **And the question is the base pin, not `is_dir()`** (M53 Increment 3). Each of this
/// crate's milestone doors carried its own existence check — four `!dir.is_dir()` guards
/// plus `list-tasks`' `dir.is_dir()` cache-branch selector — which asked whether the area
/// *exists*, so a bare `mkdir .jigc/milestones/<id>` passed and the door then faulted on a
/// later read of a milestone that does not exist.
/// [`engine::milestone::require_milestone_area`] is the one home of both answers; this
/// wrapper only supplies the family's shared mint route, which is why the two doors with
/// answers of their own — `discard`'s route and `list-tasks`' record fall-through — call
/// that seam directly instead of through here. It sits
/// **after** [`reseed_cache`] at every call site, so the fresh-clone rebuild has already
/// refilled the pin of any area a committed record still names: what reaches the residual
/// arm is a directory no record owns.
fn require_milestone_area(jigc_root: &Path, milestone_id: &str) -> Result<PathBuf> {
    engine::milestone::require_milestone_area(
        jigc_root,
        milestone_id,
        engine::milestone::create_milestone_route(),
    )
    .map_err(finding_to_err)
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
fn run_create(cwd: &Path, title: &str, conflicts: &mut Vec<Finding>) -> Result<(String, String)> {
    // **The base pin and the staged snapshot are jigc_home's, not the standing checkout's**
    // (M53 post-review-fix review, MEDIUM 4). This comment read *"the base pin is the
    // worktree HEAD"* and the code obeyed it, while `f919ea95` moved the **boundary** that
    // gates on both to `jigc_home` — so a milestone created from a linked worktree was born
    // un-finalizable. Driven on `committed-singletons` with a branch-attached worktree
    // `feat` at `9e7adc4` and main advanced: `milestone create` pinned `feat`'s HEAD, landed
    // its record commit on **main**, and the first `milestone finalize` refused
    // `finalize.base-mismatch` at exit 3 from every cwd, with a route asking the operator to
    // rewind the main checkout onto another branch's commit.
    //
    // `f919ea95`'s own reasoning — *"there was never a second subject to choose; every read
    // and write below is about the milestone"* — is the reasoning for the door that **sets**
    // what the gate compares, and this is that door. There is no `repo_root` binding left
    // here at all, which is the point: the two roots cannot diverge in a function that binds
    // one. `jigc_home_or_repo` carries the same not-in-repo refusal the walk-up did.
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");

    // **A title that yields no id is refused before the first write** (M53 Increment 5 / D5
    // as amended by §12; [`engine::state::reject_unslugable_title`]). The milestone's id is
    // the frozen slug of this title, and `mint_id`'s empty→type-name fallback would put the
    // work at `milestone:milestone` — an identity nobody typed, that the next such call
    // then serial-collides. Driven at `a53bc0a1~`, `jigc milestone create "日本語"` did
    // exactly that **and committed the record for it**, at exit 0.
    //
    // **Its position is the decision.** It sits after `jigc_home_or_repo` — a read — so a
    // caller standing outside a repository still
    // gets M49's one converged not-in-repo answer rather than a true sentence about a title
    // that is not their problem; and ahead of `shipped_schemas`, `guard_record_free` and
    // `gitignore::ensure`, so **no write precedes it**: no area, no record commit, and no
    // entry appended to the user's own `.gitignore` on the way out of a run that refused.
    engine::state::reject_unslugable_title("milestone", title, None).map_err(finding_to_err)?;

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

    // What the `.jigc/.gitignore` amend did, carried to the ack: `create` writes into a
    // file the user legitimately co-owns, so an appended entry is named rather than left
    // for `git diff` to discover (M51 Increment 4 / T2; `crate::gitignore::IGNORE_DOORS`).
    //
    // **Made here — after the refusal, before the first write that needs it** (M52
    // Increment 5 / T9, C4): the entry this door owes is `milestones/`, and it owes it
    // because [`mint_milestone`] below puts an area there. A `create` that refuses at
    // `guard_record_free` mints nothing, so it needs no entry — and amending anyway wrote
    // into the user's own file on the way out of a run that changed nothing else.
    let ignore = crate::gitignore::ensure(&jigc_root)?;

    // `jigc_home` — the checkout `milestone finalize` reads HEAD from (`git_head(&jigc_home)`).
    let base = read_head(&jigc_home)?;
    // The carryover gate's door half (M43 T1, `design/surface-contract.md` → The
    // carryover gate): `milestone create` is the shared checkout's aggregate-index
    // door — snapshot the pre-milestone staged state into the milestone area,
    // BEFORE the record commit below touches the index, so the milestone finalize
    // can tell "staged before this milestone existed" from the milestone's own
    // staging. This is one `Written` member of [`engine::state::MINT_DOORS`], which
    // is where the sub-task mints' *exemption* is stated and driven — this comment
    // used to carry a second, weaker reason for it ("worktrees are provisioned clean
    // and a missing snapshot fails open"), which is the fail-open bound, not the
    // premise (M49 Increment 12 / T1).
    //
    // And it is `jigc_home`'s index, for the same reason the base pin is: the boundary's own
    // probe is `git_staged_snapshot(&jigc_home)`, and a snapshot of a different index is a
    // carryover verdict about a checkout nobody is gating.
    let staged = crate::task::git_staged_snapshot(&jigc_home)?;
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
    // What the ack may name: the record this call landed, or `None` dev-only — there is no
    // record and no commit there, and naming one would be exactly the law-1 lie this ack
    // exists to close (M47 Inc 8 / T5).
    let mut record = None;
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
        let landed =
            match materialize_and_commit_record(&jigc_home, schema, &minted, stamp, conflicts) {
                Ok(landed) => landed,
                Err(err) => {
                    conflicts.extend(unwind_mint(
                        &jigc_home,
                        &minted.dir,
                        engine::state::WorkArea::Milestone,
                        None,
                    ));
                    return Err(err);
                }
            };
        hook_output = landed.hook_output;
        record = Some(landed.record);
    }

    Ok((
        render::milestone_created(&MilestoneCreated {
            id: minted.id.clone(),
            base_short: minted.base.short.clone(),
            record,
            ignore,
        }),
        hook_output,
    ))
}

/// What `jigc milestone create` landed, for its ack ([`crate::render::milestone_created`]).
///
/// The `record` half is `Option` **by the same discrimination the door itself makes**: under a
/// `[dev ▸ methodology]` project `create` materializes a committed record and lands a
/// record-only commit; dev-only it does neither. Carrying the absence as a value is what keeps
/// the dev-only ack from naming a path and a sha that do not exist (`design/surface-contract.md`
/// → law 1).
pub struct MilestoneCreated {
    /// The minted milestone's work-unit id.
    pub id: String,
    /// What this call's `.jigc/.gitignore` amend did — named on the ack when it appended
    /// anything, silent otherwise (M51 Increment 4 / T2;
    /// [`crate::render::gitignore_amend_line`], disposition in
    /// [`crate::gitignore::IGNORE_DOORS`]).
    pub ignore: crate::gitignore::Ensured,
    /// The short sha of the shared base every sub-task pins.
    pub base_short: String,
    /// The committed record this call landed — `None` dev-only.
    pub record: Option<CreatedRecord>,
}

/// The committed `milestone-record` a `[dev ▸ methodology]` `create` landed ([`MilestoneCreated`]).
pub struct CreatedRecord {
    /// The record's repo-relative path — the same string the commit staged.
    pub path: String,
    /// The record-only commit's short sha, or `None` when git could not be read back after the
    /// commit landed. The read is a convenience on top of a commit that already succeeded, so a
    /// failure degrades to naming no sha (the [`crate::setup::InstallCommit::Skipped`] posture)
    /// rather than reporting a landed `create` as failed.
    pub commit: Option<String>,
}

/// What [`materialize_and_commit_record`] landed: the record commit's captured non-blocking
/// hook stream (the hook_output producer axis) plus the record identity the ack names.
struct LandedRecord {
    hook_output: String,
    record: CreatedRecord,
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
/// discipline). Returns what landed ([`LandedRecord`]): the commit's captured non-blocking hook
/// stream plus the record path + short sha the ack names.
fn materialize_and_commit_record(
    jigc_home: &Path,
    schema: &Schema,
    minted: &MintedMilestone,
    schema_version: u32,
    conflicts: &mut Vec<Finding>,
) -> Result<LandedRecord> {
    let record_path = engine::store::canonical_path(jigc_home, schema, &minted.id)
        .context("the `milestone-record` doctype declares no committed location")?;
    if let Some(parent) = record_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("could not create the record home {parent:?}"))?;
    }
    // The pre-image, captured BEFORE the write — for `create` it is the absent record
    // (`bytes: absent, index: absent`), so a rejected commit **deletes** the write back out
    // ([`RecordPreImage`]).
    let mut pre =
        capture_record_pre_image(jigc_home, &record_path, crate::rollback::MILESTONE_DOOR)?;
    let body = render_fresh_record(schema, &minted.id, &minted.base, schema_version);
    std::fs::write(&record_path, &body)
        .with_context(|| format!("could not write the milestone record {record_path:?}"))?;
    // Read back what jigc just left here — the other half of the swap. Taken one statement
    // after the write, so a racer that edits the record during the commit below is a
    // mismatch rather than being mistaken for jigc's own bytes.
    pre.wrote();

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
        conflicts,
    )?;
    // Seed the record's `file-state` baseline to what `create` just wrote, so the first
    // `add-task`'s reconcile preflight compares against the CLI's own write (T6).
    baseline_record(
        &jigc_home.join(".jigc"),
        schema,
        &minted.id,
        body.as_bytes(),
    );
    let commit = landed_record_sha(jigc_home);
    Ok(LandedRecord {
        hook_output,
        record: CreatedRecord {
            path: record_pathspec(jigc_home, &record_path)?,
            commit,
        },
    })
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

    let msg_path = msg_dir.join(engine::milestone::RECORD_COMMIT_MSG_FILE);
    std::fs::write(&msg_path, message)
        .with_context(|| format!("could not write the record commit message {msg_path:?}"))?;
    git_commit_pathspec(repo_root, &msg_path, &spec)
}

/// The record-only commit a door **just landed**, as its short sha — read back the way
/// `setup`'s install commit reads its own (`crate::setup` → `commit_install`).
///
/// The commit is already in history, so a git hiccup here degrades to naming **no** sha,
/// never to failing an op that succeeded — which is why every caller's ack carries
/// `Option<String>` and omits the line rather than printing `unknown`.
///
/// One home since M52 Increment 10 / T2: the four record-only committing doors all read it,
/// and the two that gained their ack line there (`add-task`, `add-from-spec`) would otherwise
/// have been the fourth and fifth copy of the same three lines.
fn landed_record_sha(jigc_home: &Path) -> Option<String> {
    crate::task::git_capture(jigc_home, &["rev-parse", "--short", "HEAD"])
        .ok()
        .filter(|sha| !sha.is_empty())
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
/// - `worktree` — the file's content **or absent**, carried by the generic
///   [`crate::rollback::PreImage`] entry every `FileCas` population shares (M52 Increment 5 /
///   T4). This third axis is what the promotions / owner-artifact / config-layer families
///   never needed: `create` writes a record where none existed, so its pre-image is
///   `(bytes: absent, index: absent)` and the restore is a **delete**. A capture that models
///   only "present" silently leaves `create`'s record behind.
/// - `index` — the pre-write index entry, captured through the shared index primitive
///   ([`crate::task::capture_owner_artifact_index`], which already models "absent"), so this
///   family reuses the third axis's `(mode, blob-sha)` discipline rather than minting a
///   parallel one.
///
/// **The worktree half is compare-and-swap since M52 Increment 5 / T4**, on the discipline
/// `cli::rollback::ROLLBACK_POPULATIONS`' `milestone-record` row declares. It was the reason
/// that registry could point at two populations disagreeing about one cell inside one binary:
/// the config layer's absent-pre-image arm deleted a file only while it still held jigc's
/// bytes, and this one deleted it unconditionally — so a third party who wrote at the record
/// path during a rejected `jigc milestone create` lost the file at exit 1, named by nothing.
/// Now the restore asks the shared question (*is what is there now still what jigc left?*) and
/// a `No` preserves both copies and raises this door's own `<door>.rollback-conflict`.
struct RecordPreImage {
    /// The record's repo-relative pathspec — the identity the entry is keyed and parked under,
    /// and the string [`RecordPreImage::wrote`] reads the entry back by.
    spec: String,
    /// The worktree axis: one entry, plus the door whose identity a raced restore raises.
    worktree: crate::rollback::PreImageFamily,
    /// The record path's pre-write index entry (0 or 1 entries — the shared primitive's shape).
    index: Vec<crate::task::OwnerArtifactIndexEntry>,
}

impl RecordPreImage {
    /// Record that the door **just wrote** the record, reading back the bytes it left — the
    /// other half of the compare-and-swap, and the reason it must be called one statement
    /// after the write rather than at rollback time (at rollback time the read would return a
    /// racer's bytes and call them jigc's).
    ///
    /// An arm that captures and then fails **before** writing never calls it, which is the
    /// safe value: the entry stays `PostWrite::Untouched` and the rollback leaves the path
    /// alone however much it has changed.
    fn wrote(&mut self) {
        self.worktree.wrote(&self.spec);
    }
}

/// Capture the record's pre-write image — called **before** the door writes the record, so a
/// rejected commit can restore exactly what was there ([`RecordPreImage`]).
///
/// `door` is the caller's own [`crate::rollback::ConflictDoor`]: the four milestone
/// record-only doors pass [`crate::rollback::MILESTONE_DOOR`] and a sub-task
/// `jigc task discard` passes [`crate::rollback::TASK_DISCARD_DOOR`], so a raced restore names
/// the door it happened at rather than borrowing a sibling's identity.
///
/// **"Absent" means absent**, never "unreadable": only `NotFound` yields an absent pre-image,
/// because that value is what makes the rollback *delete* the file — swallowing a
/// permission/IO error into it would turn a rejected commit into a deletion of a record that
/// still exists.
fn capture_record_pre_image(
    repo_root: &Path,
    record_path: &Path,
    door: crate::rollback::ConflictDoor,
) -> Result<RecordPreImage> {
    let spec = record_pathspec(repo_root, record_path)?;
    let mut worktree = crate::rollback::PreImageFamily::empty(door);
    worktree.push(
        crate::rollback::PreImage::capture(spec.clone(), record_path.to_path_buf())
            .with_context(|| format!("could not read the milestone record {record_path:?}"))?,
    );
    Ok(RecordPreImage {
        index: crate::task::capture_owner_artifact_index(repo_root, std::slice::from_ref(&spec))?,
        spec,
        worktree,
    })
}

/// Restore a captured [`RecordPreImage`] — the worktree bytes first (**compare-and-swap**:
/// rewritten, or the file deleted when the pre-image was absent, but only while the path still
/// holds the bytes jigc wrote), then the index entry through the shared primitive
/// (`update-index --cacheinfo` for a present entry, `--force-remove` for an absent one — the
/// worktree is never reset to HEAD). Best-effort on the restore itself, exactly like every
/// sibling axis: the commit did **not** land, so a restore failure is swallowed rather than
/// replacing the door's real error (the hook's stderr stays the correction signal).
///
/// Returns one blocking `<door>.rollback-conflict` per path whose bytes are no longer jigc's —
/// at most one here, this family holding exactly one entry. The caller carries them beside its
/// own frame ([`crate::task::carry_rollback_conflicts`]); nothing is overwritten and nothing is
/// discarded on that arm.
///
/// The `file-state` baseline is deliberately **not** touched: nothing landed, so nothing
/// re-baselines and `design/reconciliation.md` → Hash re-baselining stands untouched — the
/// restored bytes match the baseline the last *landed* write recorded.
fn rollback_record_pre_image(
    repo_root: &Path,
    jigc_root: &Path,
    pre: &RecordPreImage,
) -> Vec<Finding> {
    let conflicts = pre.worktree.restore(repo_root, jigc_root);
    crate::task::rollback_owner_artifact_index(repo_root, &pre.index);
    conflicts
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
/// removal of jigc's own files safe: an `area` reaching here is one *this* call minted, not one
/// it found.
///
/// **It removes what jigc wrote, not the directory** (M52 Increment 5 / T7;
/// `cli::rollback::ROLLBACK_POPULATIONS` → the two `MintedSet` rows; `settle-record.md` → §2).
/// Until M52 this took the whole area with `remove_dir_all`, and the interval it runs in is
/// precisely the interval holding the door's rejecting hook — arbitrary code, with the
/// gitignored workbench in front of it. Driven at `8f0fb833`, a hook that wrote a file into
/// `.jigc/tasks/<id>/docs/` and exited 1 had that file destroyed at exit 1, named by nothing.
/// [`engine::state::unwind_area`] removes the area's own registry row and then the directory
/// **non-recursively**, so a third party's bytes survive by construction; what comes back is
/// which of the three things happened, and this function turns *"the area survives"* into the
/// one thing the operator can act on — a blocking finding naming it.
///
/// The restore goes through `engine::state::persist` (temp + `rename`), never a plain
/// `std::fs::write`: `tasks.json` is a **shared** workbench file every milestone door parses,
/// and this rollback runs on a live door's failure path — so truncating it in place opened
/// exactly the zero-byte window `engine::milestone::TASKS_FILE` → *Shared state* names, on the
/// file that comment is written on (M46 completion-audit F4). It runs on **every** outcome of
/// the removal above: the cache must stop naming a sub-task the record does not, whether or not
/// that sub-task's area could be taken.
///
/// **Every error branch is a carried finding, never a stderr note** (§2's *"any other
/// `remove_file`/`remove_dir` error ⇒ `operational_failure` naming the path, the area left as
/// found"*, built at the M52 Increment 5 validation). `operational_failure` is a *door outcome*
/// and this runs only on a failure path, so the door already has an error — usually a hook's
/// verbatim stderr, which `design/finalize.md` → the M40 refinement 3 forbids replacing. §2's
/// other half settles what is left: *a reject that carries a finding is emitted on the
/// findings arm, with the operational error itself rendered as a finding* — so each fault
/// rides [`crate::task::carry_rollback_conflicts`] beside the door's own frame, exactly as the
/// `Foreign` arm does.
///
/// **Shipped as three bare `eprintln!` notes until then, and all three were wrong in the same
/// three ways.** They said *"the mint was rolled back"* by silence while the thing stood — so
/// the identical re-run dead-ended on the id this call had already minted (driven at
/// `dcc8340f`: `task.serial-collision` at `add-task`) with nothing on any surface naming the
/// survivor; they printed a **host absolute** path (`design/surface-contract.md`
/// → law 1's printed-path rule); and they went to **stderr on a reject arm**, whose document
/// stderr already owns, so under `--format json` the stream stopped parsing at all
/// (`design/command-output-contract.md` → Stream discipline — the M52 Increment 1 sweep that
/// withheld [`print_resume_route`]'s notes reached the door's own narration and not this
/// family's faults). A finding is format-aware by construction and fixes all three at once.
fn unwind_mint(
    jigc_home: &Path,
    area: &Path,
    kind: engine::state::WorkArea,
    restore: Option<(&Path, &[u8])>,
) -> Vec<Finding> {
    let mut conflicts = Vec::new();
    match engine::state::unwind_area(area, kind) {
        Ok(engine::state::AreaUnwind::Absent | engine::state::AreaUnwind::Removed) => {}
        Ok(engine::state::AreaUnwind::Foreign) => {
            conflicts.push(mint_foreign_bytes_finding(jigc_home, area));
        }
        Err(err) => conflicts.push(mint_unwind_failed_finding(jigc_home, area, &err)),
    }
    if let Some((path, bytes)) = restore
        && let Err(err) = engine::state::persist(path, bytes)
    {
        conflicts.push(task_list_stale_finding(jigc_home, path, &err));
    }
    conflicts
}

/// The refusal a **mint unwind** raises over an area it may not remove: this call minted the
/// working area, its record commit was refused, and by the time the unwind ran the area held
/// bytes jigc did not write (M52 Increment 5 / T7; `settle-record.md` → §2, §14).
///
/// It is [`DISCARD_FOREIGN_BYTES_CODE`]'s third producer and takes that identity deliberately:
/// the operator's subject is the same one the `milestone discard` guard refuses over — *files
/// in the milestone workbench that jigc did not write* — and a second spelling of it would
/// make one state answer two ways (`design/surface-contract.md` → law 1).
///
/// **Located at the area**, unlike its two sibling producers. Those are door refusals that bail
/// on the first one, so [`engine::finding::is_declared_singleton`] admits them address-less;
/// this one can fire **once per seed area** in a single `add-from-spec` unwind, and a driver
/// branching on `(code, target)` needs the area to tell two of them apart.
///
/// The route is a [`engine::finding::Route::human`] one because no `jigc` argv reconciles it:
/// what is at that path is somebody else's, and only they can say whether it is worth keeping.
/// It names the honest cost rather than hiding it — an area left standing is an id already
/// minted, so the identical re-run collides until the path is gone.
///
/// **Its wording is shape-neutral on purpose.** [`engine::state::AreaUnwind::Foreign`] answers
/// for two shapes — the ordinary one, a minted directory a third party wrote into, and the
/// degenerate one, a non-directory squatting the area's own path — and *"holds files"* would
/// be false of the second at the moment an operator is recovering.
fn mint_foreign_bytes_finding(jigc_home: &Path, area: &Path) -> Finding {
    let listed = render::repo_relative(jigc_home, area);
    Finding::graded(
        Severity::Blocking,
        DISCARD_FOREIGN_BYTES_CODE,
        format!(
            "`{listed}` holds bytes jigc did not write, so the working area this call \
             minted was left standing rather than removed with them — `.jigc/` is \
             gitignored, so nothing else has a copy of what is there"
        ),
        Some(Location::addressed(listed.clone(), 1, 1)),
        Some(Route::human(format!(
            "nothing was committed. Keep what you need from `{listed}` and delete the \
             rest — until that path is gone, the identical re-run blocks on the id this \
             call already minted"
        ))),
    )
}

/// The identity a **workbench rollback step that could not complete** carries — the mint
/// unwind's non-`ENOTEMPTY` arm and its `tasks.json` restore sibling (M52 Increment 5 / T7;
/// `settle-record.md` → §2, *"any other `remove_file`/`remove_dir` error … ⇒ … naming the
/// path, the area left as found"*; built at the increment's validation, where all three of
/// its branches were found shipped as bare stderr notes).
///
/// **One code, three producers, discriminated by target.** The subject is one state — *a
/// piece of this call's workbench that the rollback could not put back* — and `(code, target)`
/// is what tells the instances apart: the area a removal stopped inside, or the task list a
/// restore could not rewrite. A second spelling per site would make a driver branch twice on
/// one fact, which is the rationale [`DISCARD_FOREIGN_BYTES_CODE`] already states for its own
/// three producers (`design/surface-contract.md` → law 1). It is **located** for the same
/// reason that one is: a single `add-from-spec` unwind walks N seed areas, so several can
/// coexist in one output and an address-less admission through
/// [`engine::finding::is_declared_singleton`] would be unkeyable.
///
/// Like every sibling at this door it is a door refusal carried beside the door's own error,
/// so it joins neither `engine::result::CHECK_INVENTORY` nor
/// [`crate::invocation_log::ERROR_CODE_REGISTRY`] — the door's error identity stays the
/// frame's (`milestone-<verb>.commit-rejected`), and this rides the log through
/// [`crate::task::carry_rollback_conflicts`]' `finding_codes` fold.
const MINT_UNWIND_FAILED_CODE: &str = "milestone.unwind-failed";

/// [`MINT_UNWIND_FAILED_CODE`]'s shared shape — blocking, located at `listed` (already
/// repo-relative), one message and one `Human` route.
///
/// The route is [`engine::finding::Route::human`] at both producers because no `jigc` argv
/// reconciles either state: what stopped the rollback is a filesystem the operator owns, and
/// jigc mints no verb that retries a rollback whose door has already returned.
fn unwind_failed_finding(listed: &str, message: String, route: String) -> Finding {
    Finding::graded(
        Severity::Blocking,
        MINT_UNWIND_FAILED_CODE,
        message,
        Some(Location::addressed(listed.to_owned(), 1, 1)),
        Some(Route::human(route)),
    )
}

/// The **area** producer: [`engine::state::unwind_area`] stopped on a path it could neither
/// remove nor account for, so the working area this call minted is standing.
///
/// Two paths are named because they are two facts: `err.path` is where the removal stopped —
/// [`engine::state::AreaUnwindError`] carries it precisely so the caller can — and `area` is
/// the thing that survives and that the honest cost is about. Both are rendered through
/// [`render::repo_relative`], law 1's one home for a printed path; `err.source` is an
/// [`std::io::Error`] whose `Display` carries no path of its own, so quoting it adds the cause
/// without re-admitting the host absolute path the shipped note printed.
///
/// The cost rides the route rather than being discovered later, exactly as
/// [`mint_foreign_bytes_finding`]'s does: an area left standing is an id already minted, so
/// the identical re-run collides until the path is gone. The **frame beside it says the mint
/// was rolled back** — true of every other outcome of this function — which is why this cell
/// may not be silent: the finding is what makes the pair honest.
fn mint_unwind_failed_finding(
    jigc_home: &Path,
    area: &Path,
    err: &engine::state::AreaUnwindError,
) -> Finding {
    let listed_area = render::repo_relative(jigc_home, area);
    let listed_path = render::repo_relative(jigc_home, &err.path);
    unwind_failed_finding(
        &listed_area,
        format!(
            "`{listed_path}` could not be removed ({}), so the working area \
             `{listed_area}` this call minted was left as found rather than unwound — the \
             mint did not roll back",
            err.source
        ),
        format!(
            "nothing was committed, but this call's mint survives. Clear \
             `{listed_area}` yourself once that path is writable again — until it is gone, \
             the identical re-run blocks on the id this call already minted"
        ),
    )
}

/// The **task-list** producer: the milestone's shared `tasks.json` could not be put back to
/// its pre-append bytes (the [`unwind_mint`] restore), or an `add-from-spec` mid-loop unwind
/// could not drop its un-recorded seeds from it ([`unwind_unrecorded_seeds`]).
///
/// One state, named once: the demoted cache lists sub-task(s) the committed record does not.
/// The route names the exit that actually exists, read off the seam rather than assumed —
/// [`engine::milestone::reseed_cache_from_record`] no-ops only while **both** `base.json` and
/// `tasks.json` are present, so deleting the stale list makes the next milestone op re-derive
/// it from the record. It stays a [`engine::finding::Route::human`] one because jigc mints no
/// argv for that deletion, and it does not offer the door's own re-run: the re-run collides on
/// the id the cache still names, which is the state this finding is about.
///
/// `err` is an [`std::io::Error`]: like its sibling above it carries no path in its `Display`,
/// so the only path on the surface is the repo-relative one this function renders.
fn task_list_stale_finding(jigc_home: &Path, path: &Path, err: &std::io::Error) -> Finding {
    let listed = render::repo_relative(jigc_home, path);
    unwind_failed_finding(
        &listed,
        format!(
            "`{listed}` could not be rewritten ({err}), so this milestone's task list \
             still names the sub-task(s) this call minted while the committed record does not"
        ),
        format!(
            "nothing was committed, and the committed `milestone-record` is the source of \
             truth for what this milestone holds — `{listed}` is a rebuildable cache that is \
             now ahead of it. Delete `{listed}` and the next milestone op re-derives it from \
             the record; until then the identical re-run blocks on the id this call already \
             minted"
        ),
    )
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
    conflicts: &mut Vec<Finding>,
) -> Result<String> {
    commit_record_only(repo_root, record_path, msg_dir, message)
        .inspect_err(|_| {
            // The restore may refuse a path whose bytes are no longer jigc's; those findings
            // travel out to the door's dispatch arm, which prints them beside the frame and
            // folds them into the log — never in place of the door's own error.
            conflicts.extend(rollback_record_pre_image(
                repo_root,
                &repo_root.join(".jigc"),
                pre,
            ));
        })
        // N20 — the rollback above has run, so the door's state-truth clause is true of what is
        // now on disk. Mark the failure as a commit-transaction failure so the record-only
        // doors' surface frames it with their code, clause and re-run instead of dropping the
        // frame it already built (a stale `.git/index.lock` meeting the stage's `git add` is
        // this cell's ordinary cause). A hook rejection passes through unmarked and keeps its
        // own verbatim frame.
        .map_err(crate::task::mark_commit_failure)
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
        &crate::repo::SeamSubject::live(repo_root),
        &[
            std::ffi::OsStr::new("-F"),
            message_file.as_os_str(),
            std::ffi::OsStr::new("--"),
            std::ffi::OsStr::new(pathspec),
        ],
    )
}

/// `git commit -F <message_file> -- <p1> <p2> …` in the checkout `subject` names — the
/// **multi-path** sibling of [`git_commit_pathspec`], recording exactly the listed paths and
/// leaving every other staged or untracked change where it was (M55; the doc-only finalize,
/// `design/findings-channel.md` §3, sized at R5). The record doors commit one path; a
/// doc-only task commits its docs plus their recorded owner-artifacts, so it needs several.
/// Same seam ([`crate::task::git_commit_capture`]): the user's hooks run, a rejection is
/// git's bytes verbatim with nothing committed, and a success returns the captured
/// non-blocking hook stream.
///
/// **An empty list is refused before git runs**, and the refusal is the helper's reason to
/// exist rather than a nicety: `git commit -F <msg> --` with no path after the separator
/// commits the **whole index**, which is precisely the sweep the path scope exists to
/// prevent. The callers never reach here with nothing to commit (the empty-commit guard
/// precedes the transaction), so this is the backstop, not a route.
pub(crate) fn git_commit_paths(
    subject: &crate::repo::SeamSubject,
    message_file: &Path,
    paths: &[String],
) -> Result<String> {
    if paths.is_empty() {
        anyhow::bail!(
            "refusing a path-scoped commit over no paths — `git commit --` with no path \
             commits the whole index"
        );
    }
    // Each path as the file it names, never a pattern (`crate::task::literal_pathspec`): a
    // `*` or `[…]` in a recorded owner-artifact would otherwise commit every staged path it
    // matches, which is the sweep this scope exists to prevent.
    let literals: Vec<String> = paths
        .iter()
        .map(|path| crate::task::literal_pathspec(path))
        .collect();
    let mut args = vec![
        std::ffi::OsStr::new("-F"),
        message_file.as_os_str(),
        std::ffi::OsStr::new("--"),
    ];
    args.extend(literals.iter().map(std::ffi::OsStr::new));
    crate::task::git_commit_capture(subject, &args)
}

/// The committed record's **file-state key** — the repo-relative `<location><id>.md` path
/// [`reconcile_committed_store`](engine::file_state::reconcile_committed_store) keys the
/// baseline hash under (`format!("{location}{slug}.md")`). The `location:` here is already
/// docs-root-nested by [`shipped_schemas`] (`docs/milestone-records/`), so the key matches
/// the store-sweep's exactly — the same string the reconcile finding names.
///
/// `pub(crate)` for the one cross-verb consumer: the task-scope sweep's live-record
/// carve-out ([`crate::task`], M52 Inc 10 / T6) keys on exactly this string, and a second
/// hand-built `format!("{location}{id}.md")` there is a drift waiting to happen.
pub(crate) fn record_key(schema: &Schema, milestone_id: &str) -> Option<String> {
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
fn record_conflict_block(jigc_home: &Path, key: &str) -> engine::file_state::ConflictBlock {
    engine::file_state::ConflictBlock::new(
        "the milestone record is machine-maintained and was edited out of band since jigc \
         last wrote it",
        engine::finding::Route::human(format!(
            "restore `{key}` to what jigc last wrote (`{restore}` for an uncommitted edit, \
             else revert the commit that changed it) and re-run this command — an external \
             edit to a machine-maintained record is never merged and never clobbered",
            restore = engine::finding::git_at(
                jigc_home,
                &format!("checkout -- {}", crate::task::shell_token(key)),
            ),
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
    // The managed-vs-foreign discriminator's three pack facts (M48 Inc 4 / T1). This door
    // holds no pack, so it resolves its own — the alternative is a `false` constant that
    // would silently make a foreign file squatting the record path read as an unvetted
    // *managed* record, the exact split this task closes.
    let pack = make_pack()?;
    let versions = crate::pack::frozen_doctype_versions(pack.as_ref());
    let priors = crate::pack::prior_doctype_schemas(pack.as_ref(), &versions);
    let migratable = crate::pack::migratable_doctypes(pack.as_ref());
    let findings = reconcile_committed(
        &mut fs_record,
        &mut index,
        schema,
        &key,
        &from,
        &bytes,
        /* task_touched = */ true,
        &record_conflict_block(jigc_home, &key),
        &engine::validate::AdoptionInputs::new(&versions, &priors, &migratable, jigc_home),
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

/// Validate a caller-supplied `--workflow <id>` against the loaded packs **before** the
/// door mutates anything — the `read_workflow` mint-after-validate discipline stated at
/// [`crate::migrate`]'s `ensure_migratable`, applied to the two doors that mint a milestone
/// sub-task under a caller-supplied workflow (`add-task` / `add-from-spec`).
///
/// Until M49 a bogus id was accepted at exit 0: the sub-task minted, its area recorded the
/// bogus workflow, and the committed record named it `status: active`. The sub-task was then
/// permanently unreachable — `jigc workflow <bogus> --task <sub>` blocks on the unknown
/// workflow, `jigc workflow <real> --task <sub>` blocks on the re-entry W-equality guard, and
/// no verb rewrites a recorded workflow — so `jigc task discard` was its only exit. A door
/// reported success for a state it had left broken.
///
/// Membership is the **cascade-resolved** definition read: the loaded packs' workflow ids
/// first (no I/O), then [`crate::start::CascadeDefs::read_workflow`], since a project layer's
/// whole-file definition shadow may own an id no pack ships. A read that fails for any other
/// reason (an unreadable project shadow) is not distinguished here — it is equally a workflow
/// this repository cannot compose, and the rejection names the pack-provided set it is absent
/// from, so the message stays true either way.
///
/// The rejection **names the provided set** rather than routing at a catalog: `jigc start`'s
/// catalog lists only `selectable: true` work-workflows, and a sub-task's own default
/// (`sub-task`) is deliberately not one of them, so routing there would name a set the door's
/// default is missing from (`design/surface-contract.md` → law 1: nothing lies). `rerun` is
/// the calling door's own argv — built into a checked [`Route`] only on the reject path, so
/// the parse fence costs nothing on the passing one.
fn ensure_workflow_provided(jigc_home: &Path, workflow: &str, rerun: Vec<String>) -> Result<()> {
    let pack = make_pack()?;
    let pack = pack.as_ref();
    let mut provided: Vec<String> = pack
        .list(PackResourceKind::Workflows)
        .iter()
        .map(|id| id.as_str().to_owned())
        .collect();
    if provided.iter().any(|id| id == workflow) {
        return Ok(());
    }
    let project_config = jigc_home.join(".jigc").join("config");
    let resolved = crate::start::resolve_severity_cascade(pack, &project_config)?;
    if crate::start::CascadeDefs::new(&resolved, &project_config)
        .read_workflow(pack, workflow)
        .is_ok()
    {
        return Ok(());
    }
    provided.sort();
    let set = provided.join(", ");
    Err(finding_to_err(Finding::block(
        "workflow-refs.unknown-workflow",
        format!(
            "no workflow `{workflow}` — a sub-task's `--workflow` must name a workflow the \
             loaded packs provide: {set}"
        ),
        Route::mechanical(rerun, " — nothing was minted, recorded or committed"),
    )))
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
    conflicts: &mut Vec<Finding>,
) -> Result<(String, String)> {
    // The `.jigc/` area binds to jigc_home (the main checkout); no git read here.
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    // The `--workflow` membership check (M49 Inc 2 T2), ahead of EVERY mutation this door
    // makes — the reconcile preflight, the fresh-clone reseed, the mint and the record commit
    // all sit below it, so a bogus id strands no sub-task area and leaves the committed record
    // byte-identical ([`ensure_workflow_provided`]).
    ensure_workflow_provided(
        &jigc_home,
        workflow,
        vec![
            "jigc".to_string(),
            "milestone".to_string(),
            "add-task".to_string(),
            crate::task::shell_token(milestone_id),
            "\"<intent>\"".to_string(),
            "--workflow".to_string(),
            "<workflow-id>".to_string(),
        ],
    )?;
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
    // The record-only commit this door lands, when it lands one — `None` dev-only (no record,
    // no commit) and when the post-commit read-back failed (M52 Increment 10 / T2).
    let mut record_commit: Option<String> = None;
    if let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) {
        // The mint unwinds with its record (M47 Inc 2 T2): on a rejected commit the sub-task
        // area this call minted goes, and the task list returns to its captured pre-append
        // bytes — so the demoted cache never names a sub-task the record does not, and the
        // identical re-run mints the same id instead of blocking on
        // `milestone.sub-task-collision`.
        let landed = append_and_commit_record(
            &jigc_home,
            &jigc_root,
            schema,
            milestone_id,
            &added.task.id,
            intent,
            workflow,
            conflicts,
        );
        hook_output = match landed {
            Ok(appended) => {
                record_commit = appended.commit;
                appended.hook_output
            }
            Err(err) => {
                conflicts.extend(unwind_mint(
                    &jigc_home,
                    &added.task.dir,
                    engine::state::WorkArea::Task,
                    pre_list
                        .as_deref()
                        .map(|bytes| (list_path.as_path(), bytes)),
                ));
                return Err(err);
            }
        };
    }

    // The ack names the commit it landed — the door moves `HEAD` and said so on no surface
    // (D-2; `completions/artifacts/M52/baseline-surfaces.md` §4.4). The mint line stays the
    // first line, which is what every caller reading the id back off this ack keys on.
    let mut summary = format!(
        "added task:{} to milestone:{}",
        added.task.id, added.milestone_id
    );
    if let Some(sha) = &record_commit {
        summary.push('\n');
        summary.push_str(&record_commit_ack(sha, &added.task.id));
    }
    Ok((summary, hook_output))
}

/// Append one sub-task to the committed `milestone-record` and commit ONLY it — the
/// `add-task` append arm of the record-home split (`design/team-ready-state.md` → Engine
/// capability 1 (write); The commit model: a **separate** record-only path-scoped commit per
/// `add-task`). Reads the committed record source at its canonical home under docs-root,
/// appends one `tasks` item (`task-id`/`intent`/`workflow`/`status: active`) via the
/// byte-stable [`engine::milestone::append_task_item`] primitive, writes it back, then lands a
/// record-only commit through the shared [`commit_record_only`] helper — never sweeping the
/// agent's in-flight staged/untracked WIP (the M30/M31 path-scoped discipline). A failed
/// append (a malformed record, a duplicate id) surfaces the engine's routed blocking finding.
/// Returns the record commit's captured non-blocking hook stream.
// One more than clippy's ceiling since T4, and the added argument is the one this door owes
// its caller: the rollback conflicts a refused record commit could not put back. The
// alternative — a struct bundling five unrelated scalars — would hide the out-param the
// `crate::task` finalize executor already carries in exactly this shape.
#[allow(clippy::too_many_arguments)]
fn append_and_commit_record(
    jigc_home: &Path,
    jigc_root: &Path,
    schema: &Schema,
    milestone_id: &str,
    task_id: &str,
    intent: &str,
    workflow: &str,
    conflicts: &mut Vec<Finding>,
) -> Result<AppendedRecord> {
    let record_path = engine::store::canonical_path(jigc_home, schema, milestone_id)
        .context("the `milestone-record` doctype declares no committed location")?;
    let source = std::fs::read_to_string(&record_path)
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;

    let appended = engine::milestone::append_task_item(schema, &source, task_id, intent, workflow)
        .map_err(|err| finding_to_err(engine::write::generate_error_finding(&err)))?;
    // The pre-image, captured BEFORE the append lands on disk — a rejected commit restores
    // the pre-append bytes AND the pre-append index entry ([`RecordPreImage`]). Under
    // `add-from-spec` this runs once per seeded sub-task, so the k-th rejection unwinds
    // exactly the k-th append.
    let mut pre =
        capture_record_pre_image(jigc_home, &record_path, crate::rollback::MILESTONE_DOOR)?;
    std::fs::write(&record_path, &appended)
        .with_context(|| format!("could not write the milestone record {record_path:?}"))?;
    pre.wrote();

    // The message temp file lands in the gitignored milestone WIP area (never a tracked path).
    let msg_dir = milestone_dir(jigc_root, milestone_id);
    let hook_output = commit_record_transaction(
        jigc_home,
        &record_path,
        &msg_dir,
        &format!("chore(milestone): record task:{task_id} on milestone:{milestone_id}\n"),
        &pre,
        conflicts,
    )?;
    // Advance the record's `file-state` baseline to the appended bytes, so the next overwrite's
    // reconcile preflight compares against this write, not the pre-append record (T6).
    baseline_record(jigc_root, schema, milestone_id, appended.as_bytes());
    Ok(AppendedRecord {
        hook_output,
        commit: landed_record_sha(jigc_home),
    })
}

/// What one record append hands back ([`append_and_commit_record`]) — the record-only commit
/// it landed, on the two axes the caller's surface needs: the captured non-blocking hook
/// stream it relays, and the sha its ack names. The shape [`SettledSubTask`] already carries
/// for the sibling door, reused so the two record-committing families describe one thing one
/// way (M52 Increment 10 / T2).
///
/// `commit` is `None` only when the post-commit read-back itself failed; a door that landed
/// no commit at all (dev-only, no `milestone-record` doctype) never calls this.
struct AppendedRecord {
    hook_output: String,
    commit: Option<String>,
}

/// The `record commit: <sha>` ack line the two **seeding** doors print — one per record-only
/// commit they land (M52 Increment 10 / T2, D-2).
///
/// Both doors were silent about a commit they had already made: driven, `add-task` and
/// `add-from-spec` moved `HEAD` with neither `--help` nor ack naming it, while the four
/// sibling record-committing doors all printed a sha
/// (`completions/artifacts/M52/baseline-surfaces.md` §4.4). The line's shape is the shipped
/// one ([`crate::render::record_commit_line`]), so the family speaks one way; the tail names
/// the sub-task, because `add-from-spec` lands one commit per seeded sub-task and a reader
/// otherwise cannot tell the N shas apart.
fn record_commit_ack(sha: &str, task_id: &str) -> String {
    crate::render::record_commit_line(
        sha,
        &format!(
            "task:{task_id} on the milestone record, committed on its own; anything else \
             you had staged stayed staged"
        ),
    )
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
    format: crate::cli::Format,
    milestone_id: &str,
    spec_addr: &str,
    workflow: &str,
    conflicts: &mut Vec<Finding>,
) -> Result<(String, String)> {
    // **The `<slug>` head is adjudicated at the door** — the fourth user-address parse
    // boundary, joining `doc::parse_verb_addr`, `TaskArea::bind` and `rename::parse_addr`
    // (M50 Inc 2 / T1; `crate::task::reject_malformed_slug_head`). A doc's slug *is* its
    // path component, and the engine's spec read joins it onto `<docs-root>/<location>/`
    // and opens the result: driven before this guard,
    // `jigc milestone add-from-spec <m> 'spec:../../../../<outside>/planted'` read a file
    // from **outside the repository**, seeded a sub-task from its criteria and landed a
    // commit naming the foreign source, at exit 0. Ahead of `jigc_home`, the workflow
    // check and every mutation: an address that names no doc anywhere is refused before
    // anything is resolved, read or minted. An address that does not *parse* falls
    // through untouched — the engine's own read owns that fault and answers it with
    // `store.unparseable`, so this guard adds a code rather than replacing one.
    if let Ok(address) = engine::address::Address::parse(spec_addr) {
        crate::task::reject_malformed_slug_head(spec_addr, address.slug.as_str())?;
    }
    // The committed spec read + the `.jigc/` sub-task mint both bind to jigc_home (the
    // main checkout); no git read here (sub-tasks pin to the milestone's stored base).
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    // The `--workflow` membership check (M49 Inc 2 T2), ahead of every mutation — the same
    // check the sibling `add-task` door runs, before the spec is even read, so a bogus id
    // seeds nothing ([`ensure_workflow_provided`]).
    ensure_workflow_provided(
        &jigc_home,
        workflow,
        vec![
            "jigc".to_string(),
            "milestone".to_string(),
            "add-from-spec".to_string(),
            crate::task::shell_token(milestone_id),
            crate::task::shell_token(spec_addr),
            "--workflow".to_string(),
            "<workflow-id>".to_string(),
        ],
    )?;
    let jigc_root = jigc_home.join(".jigc");
    let schemas = shipped_schemas(&jigc_home)?;

    // …and a well-formed slug still has to be an identity the doctype **can have** (M52
    // Increment 6 / T3; settle-record → D5.2 as amended by §9). Here, at this door's own
    // schema resolution — it is not a caller of the nine `doc` doors' shared funnel and
    // the predicate needs a `Schema` — and still **ahead of** the reconcile preflight,
    // the cache reseed and every mint. An unknown doctype is absent from `schemas`, so the
    // predicate is not applicable and the engine's own `store.unknown-type` still answers
    // first, unchanged.
    //
    // **The state it refuses.** `canonical_path`'s placement branch ignores the slug, so
    // driven at `9961013c` this read door resolved `vision:alpha` to the real committed
    // `VISION.md`, read it, enumerated its sections and refused with
    // `store.no-such-section` — routed at `jigc doc show vision:alpha`, an address its
    // sibling read door refuses (`completions/artifacts/M52/baseline-tokens.md` §4.2). A
    // dead end jigc printed itself, over a file the caller never named.
    if let Ok(address) = engine::address::Address::parse(spec_addr)
        && let Some(schema) = schemas.get(address.r#type.as_str())
    {
        crate::task::reject_fixed_identity_alias(schema, &address).map_err(finding_to_err)?;
    }

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
        conflicts.extend(unwind_unrecorded_seeds(
            &jigc_home,
            &jigc_root,
            milestone_id,
            &aborted.minted,
        ));
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
    // One `record commit:` ack line per landed record commit, in seed order — this door lands
    // N of them and named none (M52 Increment 10 / T2, D-2). Empty dev-only.
    let mut record_commits: Vec<String> = Vec::new();
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
                        workflow,
                        conflicts,
                    )
                });
            match recorded {
                Ok(appended) => {
                    streams.push(appended.hook_output);
                    if let Some(sha) = &appended.commit {
                        record_commits.push(record_commit_ack(sha, &a.task.id));
                    }
                }
                Err(err) => {
                    // The mid-loop unwind (M47 Inc 2 T3): the k−1 landed record commits are
                    // history and cannot be undone, so this call's atomicity is "the workbench
                    // names exactly what the record names" — every mint from here on goes.
                    conflicts.extend(unwind_unrecorded_seeds(
                        &jigc_home,
                        &jigc_root,
                        milestone_id,
                        &seeded.added[landed..],
                    ));
                    print_resume_route(
                        format,
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
    // …and then every commit it landed, one line each: a re-run that skipped every criterion
    // lands none and prints none, which is the same absent-vs-present rule the sibling acks
    // use for a sha they do not have.
    for line in &record_commits {
        summary.push('\n');
        summary.push_str(line);
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
/// the correction signal — but a failure is **named**, because what survives is a workbench the
/// operator may have to repair by hand. It is the third member of [`unwind_mint`]'s error-branch
/// axis and answers with that function's identity ([`task_list_stale_finding`]): the state a
/// failed `drop_sub_tasks` leaves is the same one a failed restore leaves one door over — the
/// demoted cache naming sub-tasks the record does not — so it is the same finding, keyed at the
/// same path. It shipped as a bare stderr note with the same three defects its two siblings had.
///
/// **Per seed, not per call** (M52 Increment 5 / T7): each area is unwound through
/// [`unwind_mint`], so a seed holding bytes jigc did not write is left standing and **named**
/// while its clean siblings are still taken. The returned findings are the union, one per such
/// area — which is why that producer is located at the area rather than riding
/// [`engine::finding::is_declared_singleton`]'s address-less admission like its two sibling
/// producers: this is the one cell where the family can emit more than one instance at a time.
fn unwind_unrecorded_seeds(
    jigc_home: &Path,
    jigc_root: &Path,
    milestone_id: &str,
    unrecorded: &[engine::milestone::AddedTask],
) -> Vec<Finding> {
    // Nothing minted — nothing to unwind, and no list to re-render (the failure may well be
    // that there is no milestone area at all).
    if unrecorded.is_empty() {
        return Vec::new();
    }
    let mut conflicts = Vec::new();
    for a in unrecorded {
        conflicts.extend(unwind_mint(
            jigc_home,
            &a.task.dir,
            engine::state::WorkArea::Task,
            None,
        ));
    }
    let ids: Vec<String> = unrecorded.iter().map(|a| a.task.id.clone()).collect();
    if let Err(err) = engine::milestone::drop_sub_tasks(jigc_root, milestone_id, &ids) {
        conflicts.push(task_list_stale_finding(
            jigc_home,
            &engine::milestone::milestone_dir(jigc_root, milestone_id)
                .join(engine::milestone::TASKS_FILE),
            &err,
        ));
    }
    conflicts
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
    format: crate::cli::Format,
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
    // **The stream rule** (M52 Increment 1 / T1; `design/command-output-contract.md` → Stream
    // discipline): these two notes print *beside* the refusal, and on a reject the refusal's
    // document owns stderr — so under `--format json` they are withheld rather than emitted
    // ahead of it, which is what stopped that stream parsing at all.
    //
    // **Withheld, not lost, and the carrier is named.** The rule they specialize is already in
    // the frame's own clause the document carries (*"any sub-task this run already recorded
    // stayed committed, and the re-run seeds only the remainder"*), and the exact counts are a
    // fact about the **committed record**, which is the source of truth for what landed and is
    // read back with `jigc milestone list-tasks <id>` — not a number only this print knew. The
    // re-run the second note names is the same argv the frame's route carries.
    if format == crate::cli::Format::Json {
        return;
    }
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
    crate::start::resolved_schemas(pack.as_ref(), &repo_root.join(".jigc/config"))
}

/// **Re-derive the demoted `.jigc` milestone cache from the committed record when absent** —
/// the fresh-clone resume read-path arm (`design/team-ready-state.md` → Engine capability 2
/// (read-back): "on a fresh clone (no `.jigc/` working state) the first milestone op parses the
/// record back into `BasePin` + `TaskList` and re-seeds the cache"; "Continue" means resume, not
/// WIP recovery; M39 T5). Every milestone-op **cache reader** that *operates* — `add-task` /
/// `add-from-spec` / `provision` / `execute` / `join` / `finalize` / `discard` — calls this
/// before it reads through the demoted cache
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
/// **The one read verb no longer reaches it** (M49 Increment 2, T4). `list-tasks` is
/// [`crate::cli::VerbKind::Read`] and used to call this like the rest, so a *read* rebuilt
/// every absent sub-task area — writing each one's `workflow` file, at the time, out of
/// `DEFAULT_SUB_TASK_WORKFLOW`: a value the record did not then carry, invented for an
/// `--workflow`-overridden sub-task, and read as authority by a later re-entry. M49 Increment 9
/// / T3 removed the *invention* — the record carries a per-item `workflow` leaf at
/// `schema-version` 3 and the re-seed sources it — but the objection is untouched, and it is
/// why `list-tasks` stays off this site: a `Read` leaf may not materialize a working area
/// whatever it would put in it. It reads the committed record directly now ([`read_record`])
/// and materializes nothing; the doors that go on to *operate* are unchanged.
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
/// / `provision` / `execute` / `join` / `finalize` / `discard` all block here with
/// `milestone.terminal`, routed to the committed record's read surface. (`list-tasks` refuses on
/// the same predicate, read-only, at its own site — [`run_list_tasks`].) Before that refusal
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

/// **Settle one sub-task's record item to `discarded` and commit ONLY the record** — the
/// `jigc task discard <sub-id>` half of the abandon path, and the **fifth record-only door**
/// (`design/team-ready-state.md` → The lifecycle / the per-op record-only-door transaction;
/// M49 Increment 2 / T3). Called by [`crate::task`]'s discard **before** it removes the
/// working area, so a rejected commit leaves the task exactly as the frame says it did.
///
/// It carries the family's shipped discipline unchanged, because the discipline is a property
/// of the seam and not of each caller remembering it: the reconcile preflight (an out-of-band
/// edit to the machine-maintained record conflict-blocks rather than being clobbered), the
/// captured pre-image, [`commit_record_transaction`] (which restores both axes when the hook
/// rejects), and [`baseline_record`] on the landed bytes.
///
/// `area` is the sub-task's **own working area**, in both of the roles this seam needs it for:
/// the gitignored scratch the commit message is written into (never the milestone area, which
/// a fresh clone may not have), and the leftover the settled-item refusal names when the
/// record says this sub-task is already over (M53 Increment 2 / T4). One path, because the two
/// roles are the same directory at this door — it is guaranteed to exist here, the discard is
/// about to remove it, and the refusal is about the very bytes the removal would have taken.
///
/// **Inert in the omitting contexts**, and both are ordinary rather than exceptional: a
/// dev-only project resolves no `milestone-record` doctype, and an ordinary task is named by no
/// record. Both return `Ok(None)` — nothing to record, no commit, and the discard proceeds.
/// Otherwise a [`SettledSubTask`] comes back: the landed record commit's captured non-blocking
/// hook stream for the caller to relay, and **the sha that commit landed as**, which the ack
/// names (M51 Increment 9 / T8).
///
/// **An absent answer is not the same as a negative one.** `Ok(None)` is reserved for
/// *"every record read, none names this task"*; when a record could not be read or did not
/// conform, [`recording_milestone`] hands back that set and this door **refuses**
/// ([`unreadable_record_refusal`]) rather than taking the benign branch. Before that split
/// existed, one out-of-band byte in the record made the whole door silent: the caller deleted
/// the sub-task's working area at exit 0 while the record went on calling it `active` — the
/// very lie this door exists to close, and the one state in which the door cannot see it.
pub(crate) fn settle_discarded_sub_task(
    jigc_home: &Path,
    jigc_root: &Path,
    task_id: &str,
    area: &Path,
    conflicts: &mut Vec<Finding>,
) -> Result<Option<SettledSubTask>> {
    let schemas = shipped_schemas(jigc_home)?;
    let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) else {
        return Ok(None);
    };
    let (recorded, unreadable) = recording_milestone(jigc_home, schema, task_id)?;
    let milestone_id = match recorded {
        Some(id) => id,
        // No record claims the task AND every record was read: an ordinary task, inert.
        None if unreadable.is_empty() => return Ok(None),
        // No record claims the task but one could not be read — fail closed, routed.
        None => {
            return Err(unreadable_record_refusal(
                jigc_home,
                jigc_root,
                schema,
                task_id,
                &unreadable,
            ));
        }
    };
    // The reconcile preflight, ahead of any mutation — the settle is a `set: on-transition`
    // overwrite like every other record write ([`reconcile_record_preflight`]).
    reconcile_record_preflight(jigc_home, jigc_root, schema, &milestone_id)?;
    let record_path = engine::store::canonical_path(jigc_home, schema, &milestone_id)
        .context("the `milestone-record` doctype declares no committed location")?;
    // Captured BEFORE the settle writes — a rejected commit restores the pre-settle bytes AND
    // the pre-settle index entry ([`RecordPreImage`]).
    let mut pre =
        capture_record_pre_image(jigc_home, &record_path, crate::rollback::TASK_DISCARD_DOOR)?;
    // The settled-item guard lives inside the engine call, so it is asked of the record this
    // door is about to splice and **after** the reconcile preflight — drift outranks the
    // terminal (`design/team-ready-state.md` → The terminal is terminal), and a terminal read
    // off tampered bytes would refuse with a sentence about a state nobody wrote.
    let settled = engine::milestone::discard_sub_task_item(
        &record_path,
        schema,
        &milestone_id,
        task_id,
        area,
        jigc_home,
    )
    .map_err(finding_to_err)?;
    // The engine writes the settled record; the read-back is the door's, one statement after.
    pre.wrote();
    let hook_output = commit_record_transaction(
        jigc_home,
        &record_path,
        area,
        &format!("chore(milestone): discard task:{task_id} on milestone:{milestone_id}\n"),
        &pre,
        conflicts,
    )?;
    baseline_record(jigc_root, schema, &milestone_id, settled.as_bytes());
    Ok(Some(SettledSubTask {
        hook_output,
        commit: landed_record_sha(jigc_home),
    }))
}

/// What a **settled** sub-task discard hands back ([`settle_discarded_sub_task`]) — the
/// record-only commit it landed, described on both axes the caller's surface needs: the
/// captured non-blocking hook stream it relays, and the sha the ack names.
///
/// `commit` is `None` only when the post-commit read-back itself failed; the *no commit at
/// all* case is the enclosing `Option`, so the two are never conflated.
pub(crate) struct SettledSubTask {
    pub(crate) hook_output: String,
    pub(crate) commit: Option<String>,
}

/// **The milestone whose COMMITTED RECORD names `task_id` as a sub-task** — the sub-task
/// discriminator [`settle_discarded_sub_task`] keys on, and the reason it keys on the record:
/// the committed `.md` is the source of truth and `.jigc/milestones/<id>/tasks.json` a
/// rebuildable cache (`design/team-ready-state.md` → Engine capability 2), so the
/// cache-reading [`engine::milestone::owning_milestone`] would answer *"no milestone"* for a
/// fresh clone whose workbench was rebuilt from the record — and a `task discard` there would
/// silently leave the record calling the sub-task `active`, which is the very defect this door
/// exists to close. The same rule `add-from-spec`'s resume skip set already applies.
///
/// Scans the record home in **sorted file order**, so a task id that somehow appeared in two
/// records resolves deterministically.
///
/// **A record that does not read or does not conform is collected, never skipped** — it rides
/// back in the second return as `(claiming id, unreadable ids)`. The earlier rule ("skipped
/// rather than propagated ... the record this door does need surfaces its own fault at the
/// splice") could not hold, and the code could not honour it: an unreadable record is exactly
/// the one whose task list is unknown, so walking past it turns *"I could not tell"* into
/// *"no milestone names this task"*, the caller never reaches a splice, and the discard
/// proceeds silently. The caller decides — [`unreadable_record_refusal`] — and it fails closed,
/// which does mean an unrelated unreadable record refuses an unrelated discard: that is the
/// price of the discrimination being unavailable, and it is paid loudly and with a route
/// instead of silently and destructively.
fn recording_milestone(
    jigc_home: &Path,
    schema: &Schema,
    task_id: &str,
) -> Result<(Option<String>, Vec<String>)> {
    let Some(location) = schema.location.as_deref() else {
        return Ok((None, Vec::new()));
    };
    let Ok(entries) = std::fs::read_dir(jigc_home.join(location)) else {
        return Ok((None, Vec::new()));
    };
    let mut ids: Vec<String> = entries
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| {
            entry
                .file_name()
                .to_str()
                .and_then(|name| name.strip_suffix(".md"))
                .map(str::to_owned)
        })
        .collect();
    ids.sort();
    let mut unreadable = Vec::new();
    for id in ids {
        let Some(path) = engine::store::canonical_path(jigc_home, schema, &id) else {
            unreadable.push(id);
            continue;
        };
        let Ok(source) = std::fs::read_to_string(&path) else {
            unreadable.push(id);
            continue;
        };
        let Ok((_, tasks)) = engine::milestone::read_back_record(schema, &source) else {
            unreadable.push(id);
            continue;
        };
        if tasks.tasks.iter().any(|t| t == task_id) {
            return Ok((Some(id), unreadable));
        }
    }
    Ok((None, unreadable))
}

/// **The refusal when no record claims the task and one could not be read** — the fail-closed
/// half of [`recording_milestone`]'s two-valued answer.
///
/// Names the record the demoted `.jigc/milestones/<id>/tasks.json` cache attributes the task to
/// when that record is itself one of the unreadable ones ([`engine::milestone::owning_milestone`]
/// — a hint, never the truth, so it only *chooses among* suspects and never clears one), else
/// the first in the same sorted order the scan walked, so the message is deterministic.
///
/// The diagnosis is the shipped one, not a new code: the overwhelmingly common cause is an
/// out-of-band edit to a machine-maintained record, which [`reconcile_record_preflight`] answers
/// with `reconciliation.conflict-block` and its restore route — the identical finding every
/// sibling milestone door raises over the same bytes, which is the whole point (this door was
/// the only one silent about it). A record that is unreadable *without* having drifted — a
/// hand-written file that jigc never wrote — falls through to the record's own read-back fault
/// at the splice ([`engine::milestone::read_back_record`]), and an unreadable-by-I/O record to a
/// plain operational error naming the path.
fn unreadable_record_refusal(
    jigc_home: &Path,
    jigc_root: &Path,
    schema: &Schema,
    task_id: &str,
    unreadable: &[String],
) -> anyhow::Error {
    let suspect = engine::milestone::owning_milestone(jigc_root, task_id)
        .filter(|id| unreadable.iter().any(|u| u == id))
        .unwrap_or_else(|| unreadable[0].clone());
    if let Err(drifted) = reconcile_record_preflight(jigc_home, jigc_root, schema, &suspect) {
        return drifted;
    }
    let path = engine::store::canonical_path(jigc_home, schema, &suspect);
    match path
        .as_ref()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|source| engine::milestone::read_back_record(schema, &source).err())
    {
        Some(finding) => finding_to_err(finding),
        None => anyhow::anyhow!(
            "could not read the milestone record `{suspect}`, so whether it names task \
             `{task_id}` as a sub-task is unknown — restore it to what jigc last wrote, \
             then re-run this command"
        ),
    }
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

/// **The committed record's schema and bytes — read, and nothing else.** The read-only half
/// of [`reseed_cache`], factored out because M49's T4 needs the *read* without the write:
/// `jigc milestone list-tasks` is a [`crate::cli::VerbKind::Read`] leaf and may not re-seed a
/// workbench, but it still has to answer from the source of truth and still has to see a
/// terminal record.
///
/// `None` covers the two cases in which there is no committed record to read, both ordinary:
/// a dev-only project resolves no `milestone-record` doctype (the demoted cache is then the
/// only home), and a genuinely-unknown milestone has no record file — so each caller's own
/// unknown-milestone guard still fires.
fn read_record<'a>(
    jigc_home: &Path,
    schemas: &'a BTreeMap<String, Schema>,
    milestone_id: &str,
) -> Result<Option<(&'a Schema, String)>> {
    let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) else {
        return Ok(None);
    };
    let Some(record_path) = engine::store::canonical_path(jigc_home, schema, milestone_id) else {
        return Ok(None);
    };
    if !record_path.exists() {
        return Ok(None);
    }
    let source = std::fs::read_to_string(&record_path)
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;
    Ok(Some((schema, source)))
}

/// **The sub-task ids a milestone's OPERATING doors act on** — the milestone's enumerated
/// sub-tasks minus the ones the **committed record** has settled
/// ([`engine::milestone::settled_sub_task_ids`]; M49 Increment 2, the enumeration half of the
/// discarded-sub-task sweep).
///
/// The reseed's rebuild skip closed one seam — a settled sub-task's working area is never
/// rebuilt — and that is exactly what makes this one blocking: a door still enumerating the
/// settled item emits work against an area that is deliberately absent. `milestone execute`
/// printed a `` Spawn: `cd .jigc/worktrees/<sub> && jigc workflow sub-task --task <sub>` ``
/// line for it that, run verbatim, dead-ends on *"no task"* and routes back at
/// `jigc milestone list-tasks`, which names that same sub-task — the very loop
/// `design/team-ready-state.md` says the reseed exists to close, reached through the other
/// seam. So the terminal predicate sweeps the enumeration too: one settled item, one answer,
/// at every operating door.
///
/// **Ordered by the caller's enumeration** (id-sorted at every site — the order the join
/// reads), and inert in the two ordinary omitting contexts [`read_record`] names: a dev-only
/// project resolves no `milestone-record` doctype and a milestone with no committed record
/// subtracts nothing, so the demoted cache's full list stands exactly as before.
///
/// **The teardown doors are NOT callers, and that is a checked exclusion**: a
/// `jigc task discard <sub-id>` removes the sub-task's working area and leaves its provisioned
/// worktree standing, so `milestone discard`'s and `milestone finalize`'s teardowns are the
/// last doors able to remove it — they keep enumerating the full recorded set.
fn live_sub_task_ids(
    jigc_home: &Path,
    schemas: &BTreeMap<String, Schema>,
    milestone_id: &str,
    ids: Vec<String>,
) -> Result<Vec<String>> {
    let Some((schema, source)) = read_record(jigc_home, schemas, milestone_id)? else {
        return Ok(ids);
    };
    let settled = engine::milestone::settled_sub_task_ids(schema, &source);
    Ok(ids.into_iter().filter(|id| !settled.contains(id)).collect())
}

/// Pair each live sub-task id with **the workflow it was minted against** — the value the
/// re-entry W-equality guard compares `<W>` to, so the `fan-out` emit can name a launch line
/// the binary accepts (M49 Increment 10 / T3; `design/write-commands.md` → Sub-agent re-entry).
///
/// It is read from the sub-task's own working area (`.jigc/tasks/<id>/workflow`) — *the same
/// file that guard reads*, and on a fresh clone the one the shared re-seed has just re-derived
/// from each record item's `workflow` leaf (M49 Increment 9). Reading the guard's own source is
/// what makes agreement structural rather than coincidental.
///
/// An area recording **no** workflow yields [`None`], and the `fan-out` step's own `run:` answers
/// that sub-task — the pre-bump case [`DEFAULT_SUB_TASK_WORKFLOW`] has covered since Increment 9.
/// The CLI does this read because the resolver does **no** work-unit I/O (the determinism
/// boundary): it is fed pairs.
fn recorded_workflows(
    jigc_home: &Path,
    jigc_root: &Path,
    ids: Vec<String>,
) -> Result<Vec<SubTask>> {
    ids.into_iter()
        .map(|id| {
            let dir = jigc_root.join("tasks").join(&id);
            let workflow = engine::state::read_workflow_id(&dir)
                .with_context(|| format!("could not read the recorded workflow for `{id}`"))?;
            // The absolute worktree the emitted `Spawn:` line `cd`s into (M53 — the cwd
            // census, C2-08 / C3-01). Built from `jigc_home`, which is where
            // `provision_worktrees` cuts them, and canonicalized so the pasted `cd` names
            // the same directory `git worktree list` does — a raw path is kept when
            // canonicalization fails, which is the honest degradation.
            let worktree = jigc_home.join(engine::milestone::worktree_path(&id));
            let worktree = worktree.canonicalize().unwrap_or(worktree);
            Ok(match worktree.to_str() {
                Some(path) => SubTask::in_worktree(id, workflow, path),
                // A non-UTF-8 worktree path cannot ride a composed line at all; the
                // repo-relative convention is what the emit falls back to.
                None => SubTask::new(id, workflow),
            })
        })
        .collect()
}

fn reseed_cache(
    jigc_home: &Path,
    jigc_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    milestone_id: &str,
) -> Result<()> {
    let Some((schema, source)) = read_record(jigc_home, schemas, milestone_id)? else {
        return Ok(());
    };
    let dir = milestone_dir(jigc_root, milestone_id);
    engine::milestone::reseed_cache_from_record(&dir, schema, &source).map_err(finding_to_err)?;
    // The sub-task working areas, rebuilt from the same record (M47 Inc 3 T3). Second, on
    // purpose: `reseed_cache_from_record` carries the terminal guard, so a settled milestone
    // refuses above this line and never has areas rebuilt for it. Each area is rebuilt under
    // the `workflow` leaf its own recorded item carries (`milestone-record` schema-version 3,
    // M49 Inc 9 T3), so an `--workflow` override IS fresh-clone durable.
    // `DEFAULT_SUB_TASK_WORKFLOW` is passed as the fall-back the engine may not know — the pack
    // fact reached only by a pre-bump item that carries no leaf, and exactly the value
    // `add-task` recorded absent an override.
    engine::milestone::reseed_sub_task_areas(jigc_root, schema, &source, DEFAULT_SUB_TASK_WORKFLOW)
        .map_err(finding_to_err)
}

/// `jigc milestone list-tasks <milestone-id>` — read the milestone's task list and emit its
/// sub-task ids in **canonical id-sorted order** (the deterministic order the by-task-id join
/// enumerates, surfaced through the binary, not just the internal enumerate fn). Returns the
/// summary line; an unknown milestone (no cache area, no committed record) surfaces as a
/// context-wrapped error and exits non-zero.
///
/// **It reads, and it writes nothing** (M49 Increment 2, T4). This is the one milestone verb
/// classified [`crate::cli::VerbKind::Read`], and it used to reach the shared
/// [`reseed_cache`] site like every operating door — so a *read* rebuilt the milestone cache
/// **and every absent sub-task working area**, each with a `workflow` file the committed
/// record did not then carry and could not source. An `--workflow`-overridden sub-task
/// came back under the pack default: a read verb fabricating provenance that a later
/// `jigc workflow <W> --task <id>` re-entry reads as authority. The record carries a per-item
/// `workflow` leaf since M49 Increment 9 / T3, so that value would be sourced rather than
/// invented today — which retires the example, not the conclusion, because the conclusion never
/// rested on it: a `Read` leaf may not materialize a working area whatever it would put in it
/// (see [`crate::cli::VerbKind`], which states what a `Read` leaf may and may not materialize).
///
/// So both behaviours the re-seed was carrying here are kept, read-only:
/// - **the fresh-clone answer** — with no `.jigc/` workbench the ids come straight from the
///   committed record ([`engine::milestone::read_back_record`], the same parse the re-seed
///   derives its cache with), so a teammate on a clone is answered without a workbench being
///   materialized for a question that needed none; and
/// - **the terminal refusal** — [`engine::milestone::terminal_status`] is a pure parse of the
///   record, so a `discarded`/`joined` milestone is still refused with `milestone.terminal`
///   and routed to the read surface that serves a settled record
///   (`design/team-ready-state.md` → The terminal is terminal).
///
/// The live cache stays authoritative when it is present, exactly as before: the re-seed was
/// itself a no-op in that case, so this changes nothing for a session that already has one.
fn run_list_tasks(cwd: &Path, milestone_id: &str) -> Result<String> {
    // The `.jigc/` area binds to jigc_home (the main checkout); no git read here.
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    let schemas = shipped_schemas(&jigc_home)?;
    let recorded = read_record(&jigc_home, &schemas, milestone_id)?;
    // The terminal predicate, ahead of everything: a milestone that is over is read through
    // its record, never operated — and that is a property of the record, not of the cache.
    if let Some((schema, source)) = &recorded
        && let Some(status) = engine::milestone::terminal_status(schema, source)
    {
        return Err(finding_to_err(
            engine::milestone::terminal_milestone_finding(milestone_id, &status),
        ));
    }
    // Enumeration is id-sorted on both paths — the order the join reads, not the recorded
    // insertion order.
    //
    // **The cache branch is taken through the resolve seam, not on `is_dir()`** (M53
    // Increment 3). A *task list* is what this door reads, and a directory carrying neither
    // the pin nor the list is not a cache: asking `is_dir()` took a bare `mkdir` down this
    // branch and answered the code-less `could not read the task list … (os error 2)`. The
    // seam separates the three states instead — a cache answers from the cache; a
    // record-without-a-cache answers from the **record**, which is the fresh-clone read this
    // door serves without re-seeding, being a `VerbKind::Read` leaf; and a directory no
    // record names is the seam's residual, one key and one route with the absence it is.
    let ids = match engine::milestone::require_milestone_area(
        &jigc_root,
        milestone_id,
        engine::milestone::create_milestone_route(),
    ) {
        Ok(dir) => read_task_list(&dir)
            .with_context(|| {
                format!("could not read the task list for milestone `{milestone_id}`")
            })?
            .enumerate(),
        Err(absence) => match &recorded {
            Some((schema, source)) => engine::milestone::read_back_record(schema, source)
                .map_err(finding_to_err)?
                .1
                .enumerate(),
            None => return Err(finding_to_err(absence)),
        },
    };
    // The **live** set on both branches: a sub-task the record has settled is not one of the
    // milestone's live sub-tasks, and this listing is the route every milestone dead end
    // prints — naming a settled sub-task here is how the operator is handed back the unit
    // that is over ([`live_sub_task_ids`]).
    let ids = live_sub_task_ids(&jigc_home, &schemas, milestone_id, ids)?;
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
/// a worktree the two coincide). **Idempotent**: a re-run reuses a live worktree, and an
/// **empty** leftover dir is pruned/cleared before the add; a leftover holding anything
/// refuses unless `force` ([`probe_leftover`]). An unknown milestone (no area) surfaces as
/// a context-wrapped error.
fn run_provision(cwd: &Path, milestone_id: &str, force: bool) -> Result<String> {
    let repo_root = discover_repo_root(cwd).ok_or_else(|| crate::locate::not_in_repo(cwd))?;
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the resolve seam + base-pin/task-list reads, so `provision` on a fresh clone
    // re-derives the milestone shape (`design/team-ready-state.md` → Engine capability 2).
    let schemas = shipped_schemas(&jigc_home)?;
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

    let dir = require_milestone_area(&jigc_root, milestone_id)?;
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
    // Id-sorted ids — the deterministic order the fan-out spawns its sub-agents — minus the
    // sub-tasks the record has settled: a settled unit owns no working area (the reseed never
    // rebuilds one), so a worktree for it is a checkout nothing can ever run in
    // ([`live_sub_task_ids`]).
    let ids = live_sub_task_ids(&jigc_home, &schemas, milestone_id, list.enumerate())?;

    // `.jigc/worktrees/` must be ignored or the linked worktrees pollute the main
    // checkout's `git status` / `git add --all`.
    //
    // This door **never commits**, so an entry appended here lives in the worktree alone —
    // which makes naming it the only channel there is (M51 Increment 4 / T2;
    // `crate::gitignore::IGNORE_DOORS`).
    //
    // **Made here — after the refusals, before the walk that needs it** (M52 Increment 5 /
    // T9, C4): `provision` owes `worktrees/` because [`provision_worktrees`] puts linked
    // worktrees there, and a run refused at `milestone.unknown` or `milestone.stale-base`
    // adds none. It used to amend at the top of the door, so `jigc milestone provision
    // <unknown-id>` appended to the user's own file on its way to exit 1.
    //
    // The one write that still precedes it is [`reseed_cache`]'s, which materializes the
    // cache under `milestones/` on a fresh clone — and it runs only when the record
    // resolves, which is the case this door does not refuse for. That cache survives the
    // stale-base refusal either way (nothing rolls it back), so the entry it needs is owed
    // to the re-run that follows the repair, which amends here.
    let ignore = crate::gitignore::ensure(&jigc_root)?;

    let paths = provision_worktrees(&jigc_home, milestone_id, &base.sha, &ids, force)?;
    let mut out = format!(
        "provisioned {} worktree(s) for milestone:{milestone_id} at base {} ({})",
        paths.len(),
        base.short,
        ids.join(", ")
    );
    // The ignore amend, when there was one — appended BELOW the provision line it is
    // incidental to, and absent entirely when nothing was appended.
    if let Some(line) = render::gitignore_amend_line(&ignore) {
        out.push('\n');
        out.push_str(&line);
    }
    Ok(out)
}

/// Add one **detached** worktree per sub-task at `base_sha` under
/// `<jigc_home>/.jigc/worktrees/<id>`, **idempotently**. The worktrees-parent is
/// created first (git makes only the leaf), then `git worktree prune` drops admin
/// records for any worktree whose dir was deleted by a crashed run. For each sub-task:
/// a worktree already registered at the exact path is reused (the crashed run's
/// worktree, or a prior provision — left untouched); otherwise the leftover directory at
/// that path is **probed** ([`probe_leftover`]) and cleared only when nothing there needs
/// keeping, before `git worktree add --detach` lands a fresh one. Returns the absolute
/// worktree paths in the input (id-sorted) order.
///
/// **A leftover holding anything refuses the whole provision** (`DECISIONS.md` 2026-08-13 →
/// the Settle, F3), naming the path and what would be deleted; `force` is the operator's
/// consent to delete it. The old unconditional `remove_dir_all` here was M47's declared
/// undischarged bound, and the pre-1.0.0 trial confirmed it destroys a copied repo's live,
/// uncommitted sub-agent work at exit 0 — the registered set cannot see it, because a copy's
/// admin record names the source's path. The refusal fires **before any removal or add**, so
/// a refused provision leaves every path exactly as it found it.
///
/// **That last sentence is what the two phases below buy**, and it did not hold while the
/// probe was asked *inside* the mutating loop (M48 completion audit): a leftover at the k-th
/// path refused only after paths `0..k` had been cleared and freshly added — a
/// half-provisioned set, which is precisely what a fail-closed guard exists to prevent. So
/// every path answers first, and the first byte moves only once none of them refused. The
/// sibling doors were already shaped this way ([`held_subtask_worktrees`] for `discard`,
/// `crate::setup::dirty_fanout_worktrees` for `uninstall`, both collect-then-refuse).
///
/// **The claim is about the refusal, and stays that narrow**: a `git worktree add` that fails
/// partway through phase 2 still leaves the earlier paths provisioned, and nothing here rolls
/// that back — an idempotent re-run reuses them. **Since M49 Increment 10 / T5 that state
/// is said rather than left to be discovered**: every fallible step of phase 2 — the
/// mutating half — surfaces as a [`PROVISION_FAILED_CODE`] block naming the path it stopped
/// at, how many worktrees landed first, and the [`provision_route`] re-run, through the same
/// [`finding_to_err`] funnel [`leftover_finding`] uses. Phase 1 keeps its plain `anyhow`
/// context: it mutates nothing, so its failures leave no half-provisioned state to explain
/// and no partial set for `jigc milestone execute` to report.
fn provision_worktrees(
    jigc_home: &Path,
    milestone_id: &str,
    base_sha: &str,
    sub_ids: &[String],
    force: bool,
) -> Result<Vec<PathBuf>> {
    // Canonicalize jigc_home so the per-id paths match `git worktree list`'s canonical
    // absolute paths (git resolves symlinks at `add` time) — the reuse comparison below.
    let canonical_home = jigc_home.canonicalize().with_context(|| {
        format!(
            // The workbench root spelled against itself — `.`, the repo-real spelling of
            // "here". The alternative, the cwd's checkout, is a different directory from
            // inside a linked worktree and would print this machine's absolute path.
            "could not canonicalize the jigc home `{}`",
            render::repo_relative(jigc_home, jigc_home),
        )
    })?;
    let worktrees_root = canonical_home.join(".jigc").join("worktrees");
    std::fs::create_dir_all(&worktrees_root).with_context(|| {
        format!(
            "could not create `{}`",
            render::repo_relative(jigc_home, &worktrees_root),
        )
    })?;

    // Drop admin records for any worktree dir deleted out from under git by a crashed
    // run, so a later `add` at that path is not rejected as a stale registration.
    //
    // **From `jigc_home`, like every other worktree-set helper** (M53 post-review-fix
    // review, LOW 11). This was the one member of the five left on the walk-up while
    // `subtask_worktrees`, `held_subtask_worktrees`, `provisioned_worktrees`,
    // `partial_worktree_advisories` and `remove_worktrees` all moved. **Behaviourally
    // equivalent** — `git worktree list/prune/add` answer for the whole repository from any
    // checkout, and the paths compared below are already built from `canonical_home` — so
    // this closes a split in a set the commit message said was made uniform, and buys
    // nothing else.
    git_worktree(jigc_home, &["worktree", "prune"])?;
    let registered = registered_worktrees(jigc_home)?;

    // Phase 1 — probe every path, mutate none. A path already registered as a worktree here
    // is reused untouched (idempotent), so it is neither probed nor cleared; every other one
    // is asked before the walk is allowed to move a byte anywhere.
    let mut plan = Vec::with_capacity(sub_ids.len());
    let mut held: Vec<(PathBuf, LeftoverHold)> = Vec::new();
    for id in sub_ids {
        // The absolute worktree path; `worktree_path(id)` is the shared relative
        // convention (`.jigc/worktrees/<id>`) the spawn line also renders.
        let path = canonical_home.join(worktree_path(id));
        let reuse = registered.iter().any(|w| w == &path);
        // A non-registered leftover would make `git worktree add` fail ("already exists"),
        // so it has to go — but only once the probe can prove it holds nothing (or
        // `--force` says so): the binary cannot tell `junk.txt` from `precious.txt`.
        if !reuse
            && !force
            && let Some(hold) = probe_leftover(jigc_home, &path)
        {
            // **Collected, not returned on the spot** (RC-m50 N9): stopping at the first
            // held path names one of the several things the walk would destroy, which is
            // the law-1 half-truth every door here exists to avoid.
            held.push((path.clone(), hold));
        }
        plan.push((id.clone(), path, reuse));
    }
    if !held.is_empty() {
        return Err(finding_to_err(leftover_finding(
            milestone_id,
            jigc_home,
            &held,
        )));
    }

    // Phase 2 — nothing refused, so clear and add. Every failure from here on has already
    // half-provisioned the milestone, so each one is answered with the door's own code, the
    // path it stopped at, the count that landed, and the idempotent re-run — never a bare
    // `anyhow` that a driver cannot tell from a missing repo.
    let total = plan.len();
    let mut paths = Vec::with_capacity(total);
    for (id, path, reuse) in plan {
        let stopped = |err: anyhow::Error, landed: usize| {
            finding_to_err(provision_failed_finding(
                &ProvisionStop {
                    milestone_id,
                    jigc_home,
                    sub_id: &id,
                    path: &path,
                    landed,
                    total,
                    force,
                },
                &err,
            ))
        };
        if !reuse {
            if path.exists() {
                // Read the loss BEFORE the removal, name what the removal actually TOOK
                // after it ([`PendingLoss`], law 1). Phase 1 refused on any content unless
                // `force`, so reaching here over a non-empty path means the operator
                // consented — and consent is a reason to proceed, never a reason to destroy
                // in silence, nor a licence to claim a destruction that then failed.
                let pending = pending_loss(jigc_home, &path);
                let cleared = remove_leftover(&path);
                pending.narrate_taken(jigc_home);
                cleared
                    .with_context(|| {
                        format!(
                            "could not clear the leftover at `{}`",
                            render::repo_relative(jigc_home, &path),
                        )
                    })
                    .map_err(|err| stopped(err, paths.len()))?;
            }
            let path_str = path
                .to_str()
                .with_context(|| {
                    format!(
                        "worktree path `{}` is not valid UTF-8",
                        render::repo_relative(jigc_home, &path),
                    )
                })
                .map_err(|err| stopped(err, paths.len()))?;
            git_worktree(
                jigc_home,
                &["worktree", "add", "--detach", path_str, base_sha],
            )
            .map_err(|err| stopped(err, paths.len()))?;
        }
        paths.push(path);
    }
    Ok(paths)
}

/// Remove the leftover at `path`, **whatever shape it is** — the removal half of the
/// narrate-then-remove pair, taught the shape its own narrator has known since M49.
///
/// `remove_dir_all` is a directory verb. Aimed at a plain file it fails with
/// `Not a directory (os error 20)` — and `jigc milestone provision --force` had already
/// printed *"removing the leftover file … they are not recoverable"* by then, so the door
/// narrated a destruction it then did not perform and routed at the argv that had just failed
/// (RC-m50 N8, reproduced 3/3). A door either performs the removal it narrated or refuses
/// before narrating it; there is no third honest outcome.
///
/// The shape is [`leftover_at`]'s — the one every destroying door reads, so the removal takes
/// exactly the thing the refusal refused over and the narration named.
fn remove_leftover(path: &Path) -> std::io::Result<()> {
    match leftover_at(path) {
        LeftoverAt::Directory => std::fs::remove_dir_all(path),
        // `Absent` and `Unreadable` included: at neither one is there a shape to dispatch on,
        // and `remove_file` surfaces the real errno (`NotFound`, `EACCES`) rather than this
        // function inventing one.
        LeftoverAt::Absent | LeftoverAt::Leaf | LeftoverAt::Unreadable(_) => {
            std::fs::remove_file(path)
        }
    }
}

/// The block a **phase-2** provision failure surfaces (M49 Increment 10 / T5) — the
/// [`leftover_finding`] mold, aimed at the other half of the same state.
///
/// The two are siblings and deliberately distinct codes: [`PROVISION_CODE`] refuses in
/// phase 1 having moved nothing, so its whole claim is *"every path is as you left it"*;
/// this one fires in phase 2, after `landed` of `total` worktrees are already on disk, so
/// its claim is the opposite and it says so. That state is not a dead end — it is exactly
/// the set [`partial_worktree_advisories`] reports at `jigc milestone execute`, and the
/// route repairs it: [`provision_route`]'s re-run reuses what landed and adds the rest.
///
/// The underlying git/IO failure rides the message verbatim (`{err:#}`, the whole `anyhow`
/// chain), because *"could not clear the stale worktree dir … Not a directory"* is the only
/// sentence that says what the operator has to deal with before the re-run can work.
fn provision_failed_finding(stop: &ProvisionStop<'_>, err: &anyhow::Error) -> Finding {
    let ProvisionStop {
        milestone_id,
        jigc_home,
        sub_id,
        path,
        landed,
        total,
        force,
    } = *stop;
    let address = render::repo_relative(jigc_home, path);
    Finding::graded(
        Severity::Blocking,
        PROVISION_FAILED_CODE,
        format!(
            "milestone:{milestone_id}: could not provision sub-task `{sub_id}`'s \
             worktree — {err:#}. {landed} of {total} worktree(s) landed before it, so \
             the milestone is now partially provisioned and \
             `jigc milestone execute {milestone_id}` will say so until the rest are \
             there",
        ),
        Some(Location::addressed(&address, 1, 1)),
        Some(provision_route(
            milestone_id,
            force,
            " — deal with what the message names at that path first; the re-run reuses \
             every worktree that landed and adds only the rest",
        )),
    )
}

/// The state a **phase-2** provision failure stopped in — [`provision_failed_finding`]'s
/// whole subject, in one value.
///
/// A struct rather than seven positional arguments: the block's claim is *"`landed` of
/// `total` are on disk and `sub_id`'s is not"*, and those counts only mean anything together.
/// It also keeps the finding's `jigc_home` — the block renders its own locus through
/// [`crate::render::repo_relative`], so the rule that a printed path is repo-real stays a
/// property of the producer rather than of whichever caller happened to pre-render it.
struct ProvisionStop<'a> {
    milestone_id: &'a str,
    /// The root the subject is spelled against — a worktree path lives under
    /// `<jigc_home>/.jigc/`, never under the cwd's own checkout (this module's header,
    /// *The root a workbench path is spelled against*).
    jigc_home: &'a Path,
    sub_id: &'a str,
    /// The worktree path the walk stopped at.
    path: &'a Path,
    /// How many worktrees landed before it, and how many the milestone asked for.
    landed: usize,
    total: usize,
    /// Whether the run that stopped carried `--force` — the re-run route echoes it.
    force: bool,
}

/// The canonical absolute paths of the repo's currently-registered worktrees, parsed
/// from `git worktree list --porcelain` (each `worktree <path>` line carries the
/// canonical path git stored at `add` time). The provision reuse check compares against
/// these.
///
/// `pub(crate)` for the sibling teardown: `jigc uninstall` removes `<repo>/.jigc/`
/// wholesale, and the fan-out worktrees live inside it, so it runs the same
/// registered-then-dirty probe this module's `discard` runs
/// (`crate::setup::dirty_fanout_worktrees`).
pub(crate) fn registered_worktrees(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let out = git_worktree(repo_root, &["worktree", "list", "--porcelain"])?;
    Ok(out
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(PathBuf::from)
        .collect())
}

/// What `git` can prove about a **worktree-shaped path under `.jigc/worktrees/`** that a
/// destroying door is about to remove — the three classes
/// `git -C <path> rev-parse --show-toplevel` separates (`DECISIONS.md` 2026-08-13 → the
/// Settle, F3).
///
/// **These are linkage classes, not directory shapes**, and the guard's question is *"can I
/// prove this path holds nothing precious?"* — never *"is this path ours?"*. A plain
/// directory with no `.git` at all is indistinguishable here from a worktree whose admin
/// record was pruned: git walks up and answers for the **enclosing** repo. Which is why the
/// two non-worktree verdicts share one **fail-closed** policy: the binary cannot tell
/// `junk.txt` from `precious.txt`, so a non-empty directory it cannot vouch for refuses.
///
/// The registered set is deliberately **not** the subject. The ordinary trigger is a `cp -R`
/// or `mv` of the whole repo (how every RC trial corpus is made): the copy's worktree admin
/// record names the **source's** path, so no path under the copy's own `.jigc/worktrees/` is
/// registered, `git worktree prune` removes nothing, and a registered-set guard is inert
/// exactly where the live work is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftoverVerdict {
    /// `rev-parse` exited non-zero — git can say **nothing** about the path (the
    /// copy-then-move-the-source shape: the worktree's `.git` file points at an admin
    /// directory that no longer exists). Refuses on any non-empty directory.
    Unverifiable,
    /// `rev-parse` printed **this path** — it is a live linked worktree of its own,
    /// registered here or not, and git reads its dirt correctly. The shipped
    /// [`dirty_worktrees`] probe decides: dirty refuses, clean clears.
    OwnWorktree,
    /// `rev-parse` printed **another** path (the enclosing repository's root) — the path
    /// carries no linkage of its own. Refuses on any non-empty directory.
    NoOwnLinkage,
}

/// The verdict axis, enumerated code-side so an acceptance suite **iterates** it instead of
/// hand-listing repros ([pinning.md](../../../implementation/pinning.md) → the enumeration
/// seam): a fourth verdict cannot be added without the guard's suite failing to compile.
pub const LEFTOVER_VERDICTS: [LeftoverVerdict; 3] = [
    LeftoverVerdict::Unverifiable,
    LeftoverVerdict::OwnWorktree,
    LeftoverVerdict::NoOwnLinkage,
];

/// What a destroying door does with a byte it finds at a path it is about to remove and
/// cannot prove is jigc's own — the axis's **disposition**, and the thing an acceptance
/// re-derives the rule per (M52 Increment 4, `settle-record.md` §7).
///
/// It replaces M46's `Option<code>` refuse-vs-narrate discriminator, which could express
/// only two of the three answers a door can honestly give. The rule every member satisfies
/// is one sentence per variant, and **silence is the arm no door may take**.
pub enum Disposition {
    /// **Refuse** first, naming every path, and take it only once the operator gives
    /// `consent` at this same door — M48's single-consent rule, so a reader never has to
    /// learn a second flag to get past a second guard.
    Refuse {
        /// The flag that consents, rendered in the refusal's own route.
        consent: &'static str,
    },
    /// **Take it, and name every path taken.** The arm for a door whose removal cannot be
    /// refused without firing on the ordinary success path — M46's measured warrant, where
    /// the loss is made *visible, not prevented*.
    ///
    /// **No member holds this disposition today.** [`FINALIZE_DOOR`] did until the
    /// Increment 4 plan drove what its sink actually takes (`settle-record.md` §18): the
    /// commit it stands behind carries nothing out of a sub-task's working area, so the
    /// landed-boundary warrant for narrating is unavailable there and the row became
    /// [`Disposition::Displace`]. The variant stays because the rule is stated over three
    /// arms and the acceptance derives its assertions **per disposition** — a door that
    /// takes bytes it cannot refuse over lands in a stated arm rather than a silent one.
    Narrate,
    /// **Keep it**: move it aside with its relative path preserved, and name where it went
    /// ([`crate::task::displace_foreign_area`], `.jigc/displaced/<unit-id>/<relative>`).
    /// The arm for a door with no consent to offer, because a consent there would be a new
    /// capability rather than a guard.
    Displace,
}

/// A **destroying door**: a verb that removes a path from disk that it did not write.
///
/// **The subject is the destroyed path, not a worktree.** Until M52 Increment 4 it was
/// *a worktree-shaped path under `.jigc/worktrees/`*, which is why the two doors that
/// destroy a **working area** — `jigc task discard` and `jigc task finalize` — stood
/// outside the very table minted to enumerate destroying doors, and why the working area's
/// foreign bytes died at exit 0 with nothing said. [`WORKTREE_DOORS`] is the worktree-shaped
/// subset, for the arms whose fixture is a leftover at a worktree path.
///
/// Every door **answers for what it removes** ([`PendingLoss`], read before the removal and
/// printed after it, so neither half of the claim can be false) — a door that destroys what
/// it never named is the law-1 half-truth (`design/surface-contract.md`) — and
/// [`DestroyingDoor::disposition`] says *how* it answers.
pub struct DestroyingDoor {
    /// The verb line the refusal names — the door the reader is standing at.
    pub verb: &'static str,
    /// How this door answers for a byte at the path it removes: refuse · narrate · displace.
    pub disposition: Disposition,
    /// Every **door-scoped blocking finding code** this door refuses with over a path it
    /// would destroy, so a reader can tell which door refused — and over which of its
    /// subjects — without parsing prose.
    ///
    /// **It is a set, not a single identity.** A door stands over more than one destroyable
    /// subject: `jigc milestone discard` answers for a fan-out worktree, a sub-task's staged
    /// prose and the foreign bytes in either area, each under its own code. Reading M46's
    /// single `code` as *"the door's one refusal"* is what left the abandon destroying
    /// staged prose at exit 0 until the M50 completion audit.
    ///
    /// Empty ⇔ the door refuses over nothing it destroys — a [`Disposition::Displace`]
    /// member, whose answer is the move rather than a refusal.
    pub codes: &'static [&'static str],
}

impl DestroyingDoor {
    /// The flag that consents to this door's removal, or `None` when the door offers none
    /// — the one question every arm asks of the axis before driving a member, answered
    /// here rather than re-matched at each caller.
    pub fn consent(&self) -> Option<&'static str> {
        match self.disposition {
            Disposition::Refuse { consent } => Some(consent),
            Disposition::Narrate | Disposition::Displace => None,
        }
    }
}

/// [`PROVISION_DOOR`]'s blocking identity, named separately so its refusal producer
/// ([`leftover_finding`]) reads the code without unwrapping the axis discriminator.
const PROVISION_CODE: &str = "milestone.leftover-holds-work";

/// [`DISCARD_DOOR`]'s blocking identity — [`dirty_worktree_finding`]'s, same reason.
const DISCARD_CODE: &str = "milestone.dirty-worktree";

/// A provision that **moved bytes and then stopped** — its own blocking identity, distinct
/// from [`PROVISION_CODE`]: the leftover refusal fires in phase 1 and changes nothing, this
/// one fires in phase 2 and leaves the milestone half provisioned (M49 Increment 10 / T5).
const PROVISION_FAILED_CODE: &str = "milestone.provision-failed";

/// The advisory `jigc milestone execute` carries when the milestone is **partially**
/// provisioned — the state [`PROVISION_FAILED_CODE`] leaves behind, and the only
/// provisioning state that is worth saying anything about (see [`partial_worktree_advisories`]).
const PARTIAL_WORKTREES_CODE: &str = "milestone.worktrees-partial";

/// The one route both halves of the provisioning state share: **re-run the provision**.
/// It is idempotent by construction — a worktree already registered at a sub-task's path is
/// reused untouched ([`provision_worktrees`]) — so the same argv repairs a half-provisioned
/// milestone whether the walk stopped on an error or a sub-task was added after it ran.
///
/// `--force` is echoed only when the failing run declared it, exactly as
/// [`MilestoneCommand::rejection_frame`]'s re-runs echo theirs: printed unasked it would
/// invite consent to a removal the operator never asked for, and dropped from a run that
/// carried it the re-run would refuse at the leftover guard before reaching the walk again.
fn provision_route(milestone_id: &str, force: bool, tail: &str) -> engine::finding::Route {
    let mut argv = vec![
        "jigc".to_owned(),
        "milestone".to_owned(),
        "provision".to_owned(),
        milestone_id.to_owned(),
    ];
    if force {
        argv.push("--force".to_owned());
    }
    engine::finding::Route::mechanical(argv, tail)
}

/// The one consent every refusing door takes — M48's rule that a reader learns one flag,
/// not one per guard, rendered into each refusal's own route.
const CONSENT_FLAG: &str = "--force";

/// `jigc milestone provision`'s door — it deletes a leftover at each sub-task's worktree
/// path before `git worktree add`.
pub const PROVISION_DOOR: DestroyingDoor = DestroyingDoor {
    verb: "jigc milestone provision",
    disposition: Disposition::Refuse {
        consent: CONSENT_FLAG,
    },
    codes: &[PROVISION_CODE],
};

/// `jigc milestone discard`'s door — the abandon teardown removes the fan-out worktrees,
/// every sub-task's working area and the milestone area itself, so it answers over all
/// three of the subjects those hold.
pub const DISCARD_DOOR: DestroyingDoor = DestroyingDoor {
    verb: "jigc milestone discard",
    disposition: Disposition::Refuse {
        consent: CONSENT_FLAG,
    },
    codes: &[
        DISCARD_CODE,
        DISCARD_STAGED_PROSE_CODE,
        DISCARD_FOREIGN_BYTES_CODE,
    ],
};

/// `jigc uninstall`'s door — `remove_dir_all(<jigc_home>/.jigc)` takes the worktrees, every
/// working area and the whole gitignored workbench with it, which is why it stands over
/// more destroyable subjects than any other member ([`crate::setup`]'s four refusals, all
/// over that one removal).
///
/// **The root is jigc_home, not the standing checkout** (M53 — the cwd fixes' review, LOW
/// 10), which is what makes all four of those refusals reachable from inside a fan-out
/// worktree: pointed at the caller's checkout they were handed a tree with no
/// `.jigc/worktrees/` and no `.jigc/tasks/` at all, so every one of them short-circuited
/// empty while the removal went ahead. The disposition is unchanged — `Refuse` with
/// `--force` the single consent — because binding the right subject changes *what* the door
/// stands over, never who consents to losing it.
pub const UNINSTALL_DOOR: DestroyingDoor = DestroyingDoor {
    verb: "jigc uninstall",
    disposition: Disposition::Refuse {
        consent: CONSENT_FLAG,
    },
    codes: &[
        "uninstall.dirty-worktree",
        "uninstall.staged-prose",
        "uninstall.foreign-bytes",
        "uninstall.untracked-workbench-file",
    ],
};

/// `jigc task discard`'s door — the abandon removes one task's whole working area
/// ([`crate::task`]'s two refusals over it).
///
/// It was outside the table while the subject was worktree-shaped, and is the door the
/// registry's own rule was hardest on: `jigc uninstall` refused over a task's staged prose
/// from M47 while this one took byte-identical bytes, the asymmetry the human adjudicated
/// at M50.
pub const TASK_DISCARD_DOOR: DestroyingDoor = DestroyingDoor {
    verb: "jigc task discard",
    disposition: Disposition::Refuse {
        consent: CONSENT_FLAG,
    },
    codes: &[
        crate::task::DISCARD_STAGED_PROSE,
        crate::task::DISCARD_FOREIGN_BYTES,
    ],
};

/// `jigc task finalize`'s door — phase 7 removes the task's working area once the commit
/// is in ([`crate::task::displace_foreign_area`] runs first).
///
/// **The first [`Disposition::Displace`] member.** The landed-boundary warrant for
/// *narrating* the loss (*the staged set is already in git*) is structurally unavailable
/// here: the commit takes the promoted docs and the index and nothing at all out of the
/// working area. A consent flag would be a new capability rather than a guard, so the
/// answer is the move — `.jigc/displaced/<task-id>/<relative>`, named on stderr and on the
/// landed envelope's `displaced` key.
///
/// **`codes` stays empty although this door now mints `finalize.foreign-bytes`** (M53
/// Increment 2 / T3; `settle-record.md` → D2's check scope). This field is *door-scoped
/// blocking codes the door refuses with*, and that advisory is not a refusal: it is raised
/// **after** the commit landed, at exit 0, over bytes the door **kept**. A displacing member's
/// answer is the move, so its code set is empty — the ⇔ `flow53_acceptance` asserts stays
/// true, and it stays true for a reason rather than by accident.
pub const TASK_FINALIZE_DOOR: DestroyingDoor = DestroyingDoor {
    verb: "jigc task finalize",
    disposition: Disposition::Displace,
    codes: &[],
};

/// `jigc milestone finalize`'s door — the **landed** boundary tears the fan-out worktrees
/// down once the commit is in ([`remove_worktrees`]) and removes every sub-task's working
/// area ([`cleanup_subtask_areas`]).
///
/// It was outside the table while the table's subject was *refusal*, which left the door
/// that destroys bytes on the ordinary success path unrepresented on the very axis minted
/// to enumerate destroying doors — so the subject became *destruction*. It then stood as
/// the one narrate-only member until the Increment 4 plan drove its area sink
/// (`settle-record.md` §18): the boundary's commit carries nothing out of a sub-task's
/// working area either, so the same warrant fails at the same seam one door over, and the
/// row is [`Disposition::Displace`] — the worktrees it still takes are narrated, the
/// measured `--ignored` bound of M46 unchanged.
///
/// **`codes` stays empty for the same reason its sibling's does** (M53 Increment 2 / T3):
/// `finalize.foreign-bytes` is a landed-arm advisory over kept bytes, not a code this door
/// refuses with — and here it does not even reach a `findings` key, the landed arm being
/// pinned at `Object(&["committed"])`, so it goes out on stderr.
pub const FINALIZE_DOOR: DestroyingDoor = DestroyingDoor {
    verb: "jigc milestone finalize",
    disposition: Disposition::Displace,
    codes: &[],
};

/// The destroying-door axis — the **six** verbs that remove a path they did not write,
/// minted code-side beside the classifiers they ask so an acceptance iterates the door ×
/// disposition matrix rather than a hand-written cell list. A cell is derived **per
/// [`Disposition`]**: a refusing member is driven twice (without its consent and with it),
/// a displacing member once, and the `jigc task finalize --force` cell the Settle refuses
/// is never enumerated because no member carries a consent it does not have.
pub const DESTROYING_DOORS: [&DestroyingDoor; 6] = [
    &PROVISION_DOOR,
    &DISCARD_DOOR,
    &UNINSTALL_DOOR,
    &TASK_DISCARD_DOOR,
    &TASK_FINALIZE_DOOR,
    &FINALIZE_DOOR,
];

/// The **worktree-shaped** subset of [`DESTROYING_DOORS`] — the four doors that remove a
/// path under `.jigc/worktrees/`, and therefore the only ones a [`LeftoverVerdict`] can be
/// asked about at all (`git rev-parse` inside a task's working area answers for the
/// enclosing repo, and no worktree verb clears one).
///
/// It is the domain of the arms whose fixture plants a leftover at a sub-task's worktree
/// path; the two working-area doors are reached through their own areas. Named here rather
/// than hand-listed in each suite, so a fifth worktree door lands in one place.
pub const WORKTREE_DOORS: [&DestroyingDoor; 4] = [
    &PROVISION_DOOR,
    &DISCARD_DOOR,
    &UNINSTALL_DOOR,
    &FINALIZE_DOOR,
];

/// Which [`LeftoverVerdict`] `path` falls in — one `git rev-parse --show-toplevel` run
/// **inside** it.
///
/// **The comparison canonicalizes both sides.** git prints realpaths (`/private/tmp/…` for a
/// `/tmp/…` argument on macOS), so a raw `PathBuf` compare misfiles a live worktree under a
/// symlinked temp root as [`LeftoverVerdict::NoOwnLinkage`] — the wrong policy, though still
/// a fail-closed one.
fn classify_leftover(path: &Path) -> LeftoverVerdict {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output();
    let Ok(out) = out else {
        // git is unrunnable (absent, or `path` is not a directory) — nothing is provable.
        return LeftoverVerdict::Unverifiable;
    };
    if !out.status.success() {
        return LeftoverVerdict::Unverifiable;
    }
    let printed = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim().to_string());
    let toplevel = printed.canonicalize().unwrap_or(printed);
    let own = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if toplevel == own {
        LeftoverVerdict::OwnWorktree
    } else {
        LeftoverVerdict::NoOwnLinkage
    }
}

/// How a destroying door may honestly **name** the path it found — the shape half of the
/// answer, beside [`LeftoverVerdict`]'s linkage half.
///
/// The two are orthogonal and both are needed. A leftover that is not a directory has no
/// linkage answer at all (git cannot be run inside a file) and no children to enumerate — it
/// **is** the bytes — and until M50 that shape reached every refusal as the failure of a
/// `read_dir` that was never going to work, which is how one screen came to tell a reader to
/// check that `git` was on PATH over a plain file (RC-m50 W-2).
pub enum LeftoverShape {
    /// A directory the probe read: `entries` is what the removal would take out of it.
    Directory,
    /// Not a directory — the path itself is the bytes, so there is nothing inside it to list
    /// and no worktree verb that clears it. **A symlink is one of these whatever it points
    /// at** ([`leftover_at`]): the bytes a removal takes there are the link's own, and the
    /// tree on the far side belongs to somebody else.
    File,
    /// A directory the probe could not read, carrying the failure verbatim. The
    /// **fail-closed** cell: the door still refuses, because *"found nothing"* and *"there is
    /// nothing"* are the same bytes to a caller about to remove it and only one of them is
    /// safe.
    Unreadable(String),
}

/// What a destroying door found at a path it was about to remove.
pub struct LeftoverHold {
    /// The linkage class git returned — what the refusal can honestly claim.
    pub verdict: LeftoverVerdict,
    /// What the path **is**, and therefore how a refusal may name it.
    pub shape: LeftoverShape,
    /// What the removal would take out of it: the `git status --porcelain` entries under
    /// [`LeftoverVerdict::OwnWorktree`], the directory's own child names (sorted, so the
    /// refusal is byte-reproducible) otherwise. **Empty for the two shapes with no inside** —
    /// a [`LeftoverShape::File`] is named by its path, and an [`LeftoverShape::Unreadable`]
    /// one by the failure, because listing an empty set beside either would read as *"it
    /// holds nothing"* (`design/surface-contract.md` → law 1).
    pub entries: Vec<String>,
    /// The git operation the checkout has left **un-concluded** — the probe's *second* leg,
    /// and the one that is not a question about bytes (the independent review of `3c71da87`,
    /// the HIGH; 2026-09-22).
    ///
    /// [`dirty_worktrees`] asks `git status --porcelain`, which is a question about
    /// **working-tree bytes**, while what a destroying door takes here is **repository state
    /// git holds**: a paused `rebase -i`, a `git bisect`, a dangling sequencer queue all
    /// leave a spotless tree, so every one of them cleared the guard and the removal took the
    /// operation's todo, its authored message, `ORIG_HEAD` and that checkout's reflog with
    /// it, at exit 0 and in silence. Driven at `3c71da87` at all three refusing doors.
    ///
    /// **Populated only under [`LeftoverVerdict::OwnWorktree`]**, which is the one verdict
    /// where git can vouch for the path at all. At the other two *any* content already
    /// refuses, so the leg would change no outcome — and there is no checkout to ask.
    pub operation: Option<crate::repo::InProgress>,
}

/// **The operation `worktree` has left un-concluded**, or `None` — [`LeftoverHold::operation`]'s
/// probe.
///
/// It is **not a second probe**: it asks [`crate::repo::adjudicated_breach`], the one
/// composition the door guard and the milestone boundary's fan-out preflight both ask, narrowed
/// to the one member this subject is about. [`crate::repo::PostureMember::HeadDetached`] is
/// deliberately not it — jigc provisions every fan-out worktree `--detach`, so a leg that
/// answered for it would refuse every ordinary fan-out teardown — and `HeadUnborn` cannot occur
/// in a checkout git created at a commit.
fn held_operation(worktree: &Path) -> Option<crate::repo::InProgress> {
    crate::repo::adjudicated_breach(
        worktree,
        &crate::repo::posture_subject(worktree),
        |member| matches!(member, crate::repo::PostureMember::OperationInProgress),
    )?
    .operation()
}

/// One refusal line for a held path: the repo-relative path, then what the door found there.
///
/// **All three refusing doors list through it**, so one state is described one way whichever
/// door refused (each appends its own per-path fate after it) — the [`narrate_removal`]
/// convention, applied to the refusals.
pub(crate) fn hold_line(jigc_home: &Path, path: &Path, hold: &LeftoverHold) -> String {
    let at = render::repo_relative(jigc_home, path);
    match &hold.shape {
        LeftoverShape::Directory => format!("{at}: {}", held_here(path, hold)),
        LeftoverShape::File => format!("{at}: the file itself"),
        LeftoverShape::Unreadable(err) => format!("{at}: unknown — {err}"),
    }
}

/// What a directory-shaped hold actually holds — the entries, the un-concluded operation, or
/// both, in that order.
///
/// **The operation carries the command that clears it, aimed at the checkout holding it**
/// ([`crate::repo::aim_at`]). The refusal's own route cannot: it is composed once for a set of
/// paths that may hold different operations, and *"commit or stash what a live worktree
/// holds"* — the route every one of these doors printed — resolves a bisect or a paused rebase
/// not at all. So the followable command rides the line it is true of.
fn held_here(abs: &Path, hold: &LeftoverHold) -> String {
    let entries = hold.entries.join(", ");
    let Some(operation) = hold.operation else {
        return entries;
    };
    // Parenthesised rather than dashed: [`because`]'s own clause is appended after this with
    // an em-dash, and a second one inside would leave the line with two in a row.
    let operation = format!(
        "{} git has left un-concluded (abandon it with `{}`)",
        operation.noun(),
        crate::repo::aim_at(abs, operation.abandon()),
    );
    if entries.is_empty() {
        operation
    } else {
        format!("{entries}; and {operation}")
    }
}

/// Why the door may not dispose of what it found — one sentence per (shape, verdict), shared
/// by the refusals so two doors cannot give two reasons for one state.
pub(crate) fn because(hold: &LeftoverHold) -> &'static str {
    match &hold.shape {
        LeftoverShape::File => {
            "it is a file, not a worktree, and nothing can say those bytes are disposable"
        }
        LeftoverShape::Unreadable(_) => {
            "nothing could be read there, so nothing can say those bytes are disposable"
        }
        // The operation leg first, and at every verdict it can be set at: it is the reason
        // that is NOT about bytes, so a worktree carrying one is refused over the operation
        // even where its tree is spotless and the sentence below would be false.
        LeftoverShape::Directory if hold.operation.is_some() => {
            "it is a live git worktree git has left mid-operation, and the removal takes the \
             checkout that operation can only be concluded or abandoned from"
        }
        LeftoverShape::Directory => match hold.verdict {
            LeftoverVerdict::OwnWorktree => {
                "it is a live git worktree holding uncommitted work, registered here or not"
            }
            LeftoverVerdict::NoOwnLinkage => {
                "git reports no worktree of its own there, so nothing can say those bytes are \
                 disposable"
            }
            LeftoverVerdict::Unverifiable => {
                "git cannot read a repository there, so nothing can say those bytes are \
                 disposable"
            }
        },
    }
}

/// What sits at a worktree-shaped path, read **before** anything is listed out of it — the
/// one shape derivation every destroying door reads, whichever surface it is about to speak
/// through: the refusal ([`probe_leftover`]), the narration ([`doomed_at`]) and the removal
/// ([`remove_leftover`]).
///
/// **It is read with `symlink_metadata`, and that is the whole of it** (M51 EC-17). Until this
/// existed the three readers each asked their own question: the refusal asked
/// `symlink_metadata`, the narration asked `path.is_dir()` and `path.exists()` — both of which
/// *follow* a symlink — and one planted state made them contradict each other in the one place
/// it costs most. Driven at `83924730` over a symlink at a sub-task's worktree path pointing
/// at the repository's own committed `docs/`: `jigc milestone provision` refused naming *"the
/// file itself — it is a file, not a worktree"*, and `jigc milestone provision --force` then
/// called the same path *"the leftover directory"*, enumerated **through** the link, named
/// `decisions-log.md`, `milestone-records` and `roadmap.md` as *"the only copy of these bytes
/// — they are not recoverable"*, removed the **link alone**, and left all three tracked and
/// committed exactly where they were. Identically at `jigc uninstall --force`. No bytes were
/// lost; what was spent is the credibility of the one warning standing between an operator and
/// a real `--force` deletion (`completions/artifacts/evidence-check-1.0/data-loss-hunt.md`
/// §2).
///
/// **[`DESTROYING_DOORS`]' fourth member cannot reach this shape at all**, and the acceptance
/// rests on that rather than on a walk: [`FINALIZE_DOOR`]'s teardown is [`remove_worktrees`],
/// whose subject is the worktrees *git has registered here* — and no registration names a
/// leaf, because `git worktree add` is the only thing that mints one. So the cell is driven at
/// all four doors and the fourth is asserted to say nothing
/// (`crates/cli/tests/leftover_probe_fail_closed.rs`).
///
/// **Absence is `NotFound`, and nothing else** (M52 Increment 4 / T1, defect L-1). This read
/// used to map *every* `symlink_metadata` failure to [`LeftoverAt::Absent`] — *"which is what
/// both readers already did with it"*, said the paragraph this one replaces — and `Absent` is
/// the single answer [`probe_leftover`] returns `None` for, i.e. **provably safe to delete**.
/// An `EACCES` on the parent is not absence. Driven at `4572ca7c` over a provisioned fan-out
/// with `chmod 000 .jigc/worktrees`, `jigc milestone discard <id>` exited **0** with an empty
/// stderr, settled the record as `discarded` and removed the milestone workbench, while both
/// sub-task worktrees sat on disk holding uncommitted work `git worktree list` no longer
/// named — an irreversible settle taken over bytes nothing could vouch for. So the question
/// *"is there a directory here to look inside"* has exactly three answers — there is, there
/// is not, and **the probe could not tell** — and the third is the one the guard exists for.
#[derive(Clone, PartialEq, Eq, Debug)]
enum LeftoverAt {
    /// Nothing is there: `symlink_metadata` said `NotFound`, the one failure that *is* an
    /// answer about the path rather than about the reader's access to it.
    Absent,
    /// A directory the door may look inside, and `remove_dir_all` may clear.
    Directory,
    /// A leaf whose own bytes the removal takes: a plain file, a socket, **or a symlink of
    /// any kind** — a link is never a door out of the tree, and a **dangling** one reads as
    /// absent through `exists()` while still sitting in the way of `git worktree add`.
    Leaf,
    /// The stat itself failed for a reason that is not absence — a permission wall, a broken
    /// mount — carrying the failure verbatim for the surface that renders it. The
    /// **fail-closed** answer: every reader here turns it into a hold, never a clearance.
    Unreadable(String),
}

/// Read [`LeftoverAt`] at `path`.
fn leftover_at(path: &Path) -> LeftoverAt {
    match std::fs::symlink_metadata(path) {
        Ok(meta) if meta.is_dir() => LeftoverAt::Directory,
        Ok(_) => LeftoverAt::Leaf,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => LeftoverAt::Absent,
        Err(err) => LeftoverAt::Unreadable(format!("{err}")),
    }
}

/// The **fail-closed leftover guard**: `None` when `path` is provably safe to delete (absent,
/// empty, or a clean **and concluded** worktree of its own), `Some(hold)` when it holds
/// something the door must not take — **including** the case where the probe could not read
/// the path at all, which is a hold like any other ([`LeftoverShape::Unreadable`]) rather than
/// a failure.
///
/// **"Clean" is two questions, not one** (the independent review of `3c71da87`, the HIGH;
/// 2026-09-22). This read *"a clean worktree of its own"* and asked
/// [`dirty_worktrees`] alone — `git status --porcelain`, a question about **working-tree
/// bytes** — while what these doors remove is also **repository state git holds**. A worktree
/// paused mid-`rebase -i`, mid-bisect or over a dangling sequencer queue has a spotless tree,
/// so it cleared, and the removal took the operation's todo, its authored message, `ORIG_HEAD`
/// and that checkout's reflog with it at exit 0 and in silence — driven at all three refusing
/// doors, `jigc milestone provision` included, where the path can belong to a **different
/// repository** and its foreign checkout was replaced by jigc's own. [`LeftoverHold::operation`]
/// is the second leg.
///
/// **It cannot fail, and that is the M50 repair** (RC-m50 N9). The unreadable case used to
/// come back as an `Err` and every door propagated it with `?`, so the *first* unreadable path
/// ended the walk and the door answered only for it: `jigc uninstall` over a directory holding
/// work **and** a leftover file named the file and never mentioned the directory. A door that
/// names one of the two things it would destroy is the law-1 half-truth
/// (`design/surface-contract.md`), so the failure now joins the hold it is about and every
/// door collects the whole set before it refuses.
///
/// **The refusal itself is unchanged**: an unreadable path still blocks, under the door's own
/// code, because removing on an unverified probe is the defect this guard closes. What changed
/// is that it blocks *beside* its siblings instead of *instead of* them.
///
/// `pub(crate)` for its sibling doors, and **the consumer set is three, stated because this
/// sentence read as four and was derived from** (2026-09-22). It said *"[`DESTROYING_DOORS`]
/// all remove worktree-shaped paths under `.jigc/worktrees/`"*, which is false of two of its
/// six members — `jigc task discard` and `jigc task finalize` stand at `.jigc/tasks/<id>/`,
/// where there is no checkout (driven: a `task discard` leaves a sub-task's worktree and every
/// marker of it standing) — and it made the review of `3c71da87` read the *four*
/// [`WORKTREE_DOORS`] as this probe's callers. They are not: [`FINALIZE_DOOR`] is the
/// [`Disposition::Displace`] member, whose teardown runs **after** its commit and is guarded a
/// phase earlier by [`fan_out_posture_findings`]. The callers are the **three refusing**
/// worktree doors — [`PROVISION_DOOR`] (phase 1), [`DISCARD_DOOR`]
/// ([`held_subtask_worktrees`]) and [`UNINSTALL_DOOR`] (`crate::setup::dirty_fanout_worktrees`)
/// — and they ask one probe rather than growing three that drift, which is what made the
/// second leg above one edit instead of three.
pub(crate) fn probe_leftover(jigc_home: &Path, path: &Path) -> Option<LeftoverHold> {
    match leftover_at(path) {
        LeftoverAt::Absent => return None,
        // A leaf leftover *is* the bytes: no linkage to classify (git cannot run inside it)
        // and no children to list, so it is named by its own name — [`doomed_at`]'s answer
        // since M49, which both surfaces now reach through one derivation instead of two that
        // contradicted each other at a symlink.
        LeftoverAt::Leaf => {
            return Some(LeftoverHold {
                verdict: LeftoverVerdict::Unverifiable,
                shape: LeftoverShape::File,
                entries: Vec::new(),
                operation: None,
            });
        }
        // The stat failed and the failure was not absence, so nothing here has been read at
        // all: the hold its own doc-comment has promised since M50, finally produced.
        LeftoverAt::Unreadable(err) => {
            return Some(LeftoverHold {
                verdict: LeftoverVerdict::Unverifiable,
                shape: LeftoverShape::Unreadable(err),
                entries: Vec::new(),
                operation: None,
            });
        }
        LeftoverAt::Directory => {}
    }
    let verdict = classify_leftover(path);
    // The second leg, asked wherever git can vouch for the checkout: a clean tree is not the
    // same question as a concluded repository, and it was the only one this probe asked.
    let operation = match verdict {
        LeftoverVerdict::OwnWorktree => held_operation(path),
        LeftoverVerdict::Unverifiable | LeftoverVerdict::NoOwnLinkage => None,
    };
    let read = match verdict {
        // git can read this worktree's dirt — so ask the shipped probe, and let a clean,
        // concluded worktree clear (the idempotent re-provision every fan-out depends on;
        // the concluded half is the `operation` leg above, asked at this same verdict).
        LeftoverVerdict::OwnWorktree => dirty_worktrees(&[path.to_path_buf()]).map(|dirty| {
            dirty
                .into_iter()
                .next()
                .map(|(_, entries)| entries)
                .unwrap_or_default()
        }),
        // Nothing can vouch for these bytes, so *any* content refuses — ignored files
        // included: an ignored file is not work to git, but the binary is not the one who
        // gets to decide that about a directory it cannot even place. That is the exact
        // opposite of `dirty_worktrees`' policy one screen down, and its sibling rather
        // than its contradiction, because the two run at different scopes: there git can
        // place the bytes, so the project's own ignore rules are readable and the ordinary
        // fan-out success path is full of build output; here git places nothing, so no
        // ignored/tracked distinction exists to scope a refusal with.
        LeftoverVerdict::Unverifiable | LeftoverVerdict::NoOwnLinkage => {
            child_names(jigc_home, path)
        }
    };
    match read {
        // Both legs empty is the only clearance — the clean, concluded worktree the
        // idempotent re-provision depends on.
        Ok(entries) if entries.is_empty() && operation.is_none() => None,
        Ok(entries) => Some(LeftoverHold {
            verdict,
            shape: LeftoverShape::Directory,
            entries,
            operation,
        }),
        Err(err) => Some(LeftoverHold {
            verdict,
            shape: LeftoverShape::Unreadable(format!("{err:#}")),
            entries: Vec::new(),
            operation,
        }),
    }
}

/// [`PROVISION_DOOR`]'s refusal: a blocking, route-bearing finding naming **every** leftover
/// path the walk would delete, what is at each one, and **why nothing can vouch for it** — the
/// [`dirty_worktree_finding`] mold, door-scoped so a reader can tell which door refused.
///
/// **Every held path, not the first** (RC-m50 N9). Phase 1 probes the whole ordered set before
/// this is composed, so a provision refused over two leftovers names two.
///
/// The route is the mechanical `--force` re-run (the M43 route fence checks that argv against
/// the real CLI at construction), **unconditionally now that it works at every shape**: it
/// was withheld from the fail-closed arm because phase 2's `remove_dir_all` could not clear a
/// leftover file, so consenting there failed one door further along and the route pointed at
/// the argv that had just failed. [`remove_leftover`] closed that, so the consent this door
/// offers is a consent it can honour (RC-m50 N8, W-2).
///
/// Its tail carries the honest first move: the refusal is not a puzzle to solve, it is a set
/// of paths to look at.
fn leftover_finding(
    milestone_id: &str,
    jigc_home: &Path,
    held: &[(PathBuf, LeftoverHold)],
) -> Finding {
    let listing: Vec<String> = held
        .iter()
        .map(|(path, hold)| format!("  {} — {}", hold_line(jigc_home, path, hold), because(hold),))
        .collect();
    let address = render::repo_relative(jigc_home, &held[0].0);
    let mut tail = String::from(
        " — but look at what is listed above first and move out anything you need; the removal \
         is permanent",
    );
    if held.iter().any(|(_, hold)| hold.operation.is_some()) {
        // *Move out anything you need* is inert over an un-concluded operation: there are no
        // bytes to move, and the consent below would take the checkout the operation lives
        // in. The command that clears each one is on its own listed line.
        tail.push_str(OPERATION_CLAUSE);
    }
    Finding::graded(
        Severity::Blocking,
        PROVISION_CODE,
        format!(
            "milestone:{milestone_id}: `{}` would delete {} path(s) it cannot prove are \
             disposable, so nothing was removed:\n{}",
            PROVISION_DOOR.verb,
            held.len(),
            listing.join("\n"),
        ),
        Some(Location::addressed(&address, 1, 1)),
        Some(provision_route(milestone_id, true, &tail)),
    )
}

/// The clause every refusal over an un-concluded operation appends to its route — one home,
/// because three doors print it and the reason is the same at all three: the door's shipped
/// route names the moves that get *bytes* out of the way, and none of them reaches a
/// repository state git is holding (the independent review of `3c71da87`, the HIGH).
///
/// The concrete command per path rides that path's own listed line
/// ([`held_here`]), since one route stands for a set of paths that may hold different
/// operations.
pub(crate) const OPERATION_CLAUSE: &str = ". A path listed above as mid-operation holds no bytes to move: conclude or abandon the \
     operation in that checkout — the command that abandons it is on its line — and the path \
     clears";

/// The sorted immediate child names of `path` — the "what would be deleted" listing for the
/// two verdicts with no git to ask. Sorted, so the refusal text does not vary with readdir
/// order.
fn child_names(jigc_home: &Path, path: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let at = render::repo_relative(jigc_home, path);
    for entry in std::fs::read_dir(path)
        .with_context(|| format!("could not read the leftover directory `{at}`"))?
    {
        let entry =
            entry.with_context(|| format!("could not read an entry of the directory `{at}`"))?;
        names.push(entry.file_name().to_string_lossy().into_owned());
    }
    names.sort();
    Ok(names)
}

/// What the milestone boundary found **at a sub-task's worktree path** — the subject the
/// boundary reads, credits and commits from.
///
/// The retired subject was the *registered* set (the task list ∩ `git worktree list`),
/// the same wrong subject M48 retired at the three [`DESTROYING_DOORS`]. A `cp -R` or `mv`
/// of the whole repo — how every RC trial corpus is made — leaves the copy's worktree admin
/// records naming the **source's** paths, so nothing under the copy's own
/// `.jigc/worktrees/` is registered there and the boundary saw no worktrees at all exactly
/// where the sub-agents' live work sat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorktreeState {
    /// Nothing on disk at that path — the sub-task genuinely never got a worktree, and
    /// contributed no code **by construction** (M47 Inc 3, call (b)(ii)).
    Absent,
    /// A live linked worktree of its own ([`LeftoverVerdict::OwnWorktree`]), registered
    /// here or not: git reads its index correctly, so the boundary reads it.
    Live,
    /// A directory git cannot read as a worktree of its own
    /// ([`LeftoverVerdict::Unverifiable`] after a `mv`, [`LeftoverVerdict::NoOwnLinkage`]
    /// for a plain directory) — **named**, never read for code.
    Unreadable,
}

/// One sub-task's worktree path, classified once for every consumer of the boundary.
struct SubtaskWorktree {
    /// The sub-task's work-unit id (the path's final component).
    id: String,
    /// The canonical `<jigc_home>/.jigc/worktrees/<id>` path.
    path: PathBuf,
    /// What is actually there.
    state: WorktreeState,
    /// Whether [`remove_worktrees`]'s teardown reaches this path — it removes **registered**
    /// worktrees and skips everything else. The loss narration's subject stays exactly that,
    /// so an unregistered leftover the teardown leaves standing is never narrated as lost.
    registered: bool,
}

/// Classify every sub-task's worktree path, id-ordered — one `git rev-parse` per existing
/// path ([`classify_leftover`], the classifier the destroying doors already ask).
///
/// **Membership is the verdict on an EXISTING path, never mere existence.** Driven at a
/// plain directory under `.jigc/worktrees/`, `git diff --cached --name-only` prints the
/// **enclosing** repo's staged set at exit 0 — so a mere-existence subject would attribute
/// the main checkout's staged files to a sub-task and commit them. And the `path.exists()`
/// pre-check comes first for the same reason [`probe_leftover`] keeps it: `classify_leftover`
/// on a missing directory answers [`LeftoverVerdict::Unverifiable`], which would turn every
/// genuinely never-provisioned sub-task into an unreadable one.
///
/// Best-effort on the `git worktree list` read (an unreadable list yields no registrations —
/// the teardown then removes nothing, never a spurious block).
fn subtask_worktrees(jigc_home: &Path, list: &engine::milestone::TaskList) -> Vec<SubtaskWorktree> {
    let registered = registered_worktrees(jigc_home).unwrap_or_default();
    // Match `provision_worktrees`' canonical-path convention (git stores canonical paths at
    // `add` time); fall back to the raw path if canonicalization fails (then nothing
    // matches the registered set, which is the fail-closed side).
    let canonical_home = jigc_home
        .canonicalize()
        .unwrap_or_else(|_| jigc_home.to_path_buf());
    list.enumerate()
        .into_iter()
        .map(|id| {
            let path = canonical_home.join(worktree_path(&id));
            let state = if !path.exists() {
                WorktreeState::Absent
            } else if classify_leftover(&path) == LeftoverVerdict::OwnWorktree {
                WorktreeState::Live
            } else {
                WorktreeState::Unreadable
            };
            SubtaskWorktree {
                registered: registered.iter().any(|w| w == &path),
                id,
                path,
                state,
            }
        })
        .collect()
}

/// The id-ordered paths of the milestone's **live** fan-out worktrees — the set the combine,
/// the collision read, the boundary gate and the staged-code signal all work over. A
/// never-provisioned (docs-only) milestone yields an empty list, so the `squash: true`
/// combine degrades to a docs-only commit; a path git cannot vouch for is excluded, so
/// nothing is ever read out of it.
/// **The posture breaches the boundary owes for the worktrees it commits *from*** — one
/// blocking finding per provisioned sub-task worktree git has left an operation
/// un-concluded in, each sited at that worktree (M53 post-review fix, 2026-09-22; the M53
/// per-axis review, axis 2 `DEFECT 1`).
///
/// `Cli::dispatch`'s guard adjudicates the posture of the checkout the command was **run
/// in**, and for this door that is the main checkout. It is not the checkout the bytes
/// come from: both commit models read `git diff --cached` out of each provisioned
/// worktree's own index — [`code_carrying_worktrees`] for `squash: false`, the combine for
/// `squash: true` — so a worktree mid-merge, mid-pick or mid-rebase had its operation's
/// staged payload committed under jigc's synthesized subject at exit 0 and its authored
/// message destroyed with the teardown, while `jigc task finalize` run *inside* that same
/// worktree refused. Driven on `1.0.0-rc.17` over every [`crate::repo::InProgress`] member a
/// linked worktree can hold, at both commit models.
///
/// It is **not a second probe**: it asks [`crate::repo::adjudicated_breach`], the one
/// composition the door guard asks, with `owes` = `true` — this door commits on behalf and
/// carries no exemption row — so the fan-out worktree's `HeadDetached` exemption arrives
/// from [`crate::repo::posture_subject`] rather than from a rule restated here (jigc
/// detached those worktrees itself, and refusing there would refuse its own provisioning).
///
/// **The subject is `worktrees`**, the same value both commit channels are handed, so the
/// set that is refused and the set that is committed cannot diverge — the
/// [`code_carrying_worktrees`] discipline, one phase earlier. A worktree with nothing
/// staged is still asked: whether it contributes is decided *after* this, and a boundary
/// that concluded an operation in it would be the same act either way.
///
/// One finding per breaching worktree rather than one listing them: each carries its own
/// route, aimed with `git -C <worktree>`, and its own located address — so two breaching
/// worktrees are two discriminating `(code, target)` keys rather than one.
fn fan_out_posture_findings(jigc_home: &Path, worktrees: &[PathBuf]) -> Vec<Finding> {
    worktrees
        .iter()
        .filter_map(|worktree| {
            // The subject is the **classifier's**, and here that is the whole point: this
            // door commits that worktree's index from a `SeamSubject::dedicated` handle, so
            // the `HeadDetached` exemption is about jigc's own provisioning and arrives from
            // `posture_subject` rather than from a rule restated here.
            let breach = crate::repo::adjudicated_breach(
                worktree,
                &crate::repo::posture_subject(worktree),
                |_| true,
            )?;
            let at = render::repo_relative(jigc_home, worktree);
            Some(breach.finding_at(crate::repo::BreachSite::FanOutWorktree {
                at: &at,
                abs: worktree,
            }))
        })
        .collect()
}

/// **The posture refusal `jigc milestone finalize` would make over sub-task `id`'s own
/// worktree** — the preview half of [`fan_out_posture_findings`] (M53 post-review fix,
/// 2026-09-22).
///
/// `jigc task validate <id>` already previews the posture of the checkout it is run in
/// ([`crate::cli::finalize_posture_refusal`], M52 Increment 3 / T6). For a milestone
/// sub-task that is only half the answer: an orchestrator runs `task validate <sub>` from
/// the **main** checkout, where the posture is clean, while the boundary that actually
/// commits that sub-task reads the sub-task's *worktree* index. Without this the preview
/// answers *"the task validates clean"* for a state the door now refuses — the law-1 lie
/// the fix itself would have created (`dev-workflow.md` → *widen a guard's trigger,
/// re-derive its response*).
///
/// It asks the **boundary's own producer** over the one worktree, so the finding, its
/// route and its site are the door's, byte for byte — **and it answers on the door's arm**
/// (the independent review of `3c71da87`, MEDIUM 2). It did not: the producer was shared
/// while the carrier was not, so one state answered two ways under `--format json` — the
/// door printing the pinned findings envelope on stdout at exit 3, the preview printing
/// `{"error": …}` on stderr at exit 1. `repo.operation-in-progress` projects a key at this
/// site ([`crate::repo::BreachSite::FanOutWorktree`], the filesystem-path form —
/// `design/command-output-contract.md`), and a code inside a message is not a key. Both
/// surfaces reach [`blocked`], which is the door's own carrier.
///
/// `None` when `id` is not a sub-task of
/// a milestone in this workbench, when its worktree path is not a provisioned worktree,
/// or when that worktree **is** the checkout the command was run in — there the cwd guard
/// has already answered, at its own site, and a second finding about the same state would
/// be two answers to one question.
/// **Is `id` a milestone sub-task?** — the one question the `task finalize` preview needs
/// before it can say which committing door it is forecasting
/// ([`crate::cli::finalize_posture_refusal`]).
///
/// It asks the shipped enumerator ([`engine::milestone::owning_milestone`]) over the shared
/// workbench, which is the same authority `TaskArea::finalize`'s own sub-task guard and
/// [`sub_task_fan_out_refusal`] read — so the preview cannot decide a task belongs to a
/// milestone that the door it forecasts disagrees about. A workbench it cannot locate reads
/// **not a sub-task**: the ordinary answer, and the one whose preview is stricter.
pub(crate) fn is_sub_task(cwd: &Path, id: &str) -> bool {
    crate::start::jigc_home_or_repo(cwd)
        .ok()
        .and_then(|home| engine::milestone::owning_milestone(&home.join(".jigc"), id))
        .is_some()
}

pub(crate) fn sub_task_fan_out_refusal(cwd: &Path, id: &str, format: Format) -> Option<Outcome> {
    let repo_root = discover_repo_root(cwd)?;
    let jigc_home = crate::start::jigc_home_or_repo(cwd).ok()?;
    let jigc_root = jigc_home.join(".jigc");
    engine::milestone::owning_milestone(&jigc_root, id)?;
    let worktree = jigc_home.join(engine::milestone::worktree_path(id));
    if !crate::repo::posture_subject(&worktree).is_dedicated() {
        return None;
    }
    // Canonicalized, because the worktree path is built from `jigc_home` while `repo_root`
    // came from the cwd walk-up: on macOS one of the two carries `/private` and a raw
    // comparison would answer *different checkout* about one directory.
    let same = match (worktree.canonicalize(), repo_root.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => worktree == repo_root,
    };
    if same {
        return None;
    }
    let finding = fan_out_posture_findings(&jigc_home, &[worktree])
        .into_iter()
        .next()?;
    match blocked(&jigc_home, format, vec![finding.clone()]) {
        Ok(outcome) => Some(outcome),
        // [`blocked`] reads the severity cascade to grade the report. A pack or cascade
        // that will not load is a different failure, and the honest answer to it is still a
        // refusal — the one thing this preview must never do is fall through to *the task
        // validates clean* over a state the door refuses. So the flattened arm is the
        // degraded fallback, never the ordinary one.
        Err(_) => Some(crate::invocation_log::operational_failure(
            format,
            &render::finding_error(&finding),
        )),
    }
}

fn live_worktrees(subtasks: &[SubtaskWorktree]) -> Vec<PathBuf> {
    subtasks
        .iter()
        .filter(|sub| sub.state == WorktreeState::Live)
        .map(|sub| sub.path.clone())
        .collect()
}

/// [`live_worktrees`] over a freshly classified task list — the single-use form for the
/// `join` channel, which needs the paths and none of the other facts.
fn provisioned_worktrees(jigc_home: &Path, list: &engine::milestone::TaskList) -> Vec<PathBuf> {
    live_worktrees(&subtask_worktrees(jigc_home, list))
}

/// The advisories `jigc milestone execute` carries when the milestone is **partially**
/// provisioned — one per live sub-task with no usable worktree, id-ordered (M49 Increment
/// 10 / T5; `design/team-ready-state.md` → The lifecycle).
///
/// **The trigger is the PARTIAL set, and only it.** Before this, `execute`'s bytes were
/// identical over every provisioning state, so a walk emitting `Spawn: cd
/// .jigc/worktrees/<id> && …` for a directory that is not there read exactly like one whose
/// worktrees are all present. The response is deliberately **not** a refusal and not a
/// per-missing-worktree complaint in the un-provisioned state: the composed walk's own first
/// step is `Run: jigc milestone provision …`, so `execute` is the orientation read taken
/// *before* provisioning, and a never-provisioned milestone is a **settled** state with
/// nothing to report. A fully provisioned one is the other settled state. What is worth
/// saying — what nothing else on the surface says — is that a provision **ran and did not
/// finish**, which is knowable only from the mixed set.
///
/// So both settled states stay byte-identical reads, and the partial one speaks. The subject
/// is the **path**, classified by the shipped [`subtask_worktrees`] — the same classifier
/// the boundary and the destroying doors ask, never the registered set, for the reason
/// recorded there.
///
/// One finding per missing sub-task rather than one summary line: each then carries its own
/// discriminating `(code, target)` key and its own locus, the shape the carryover gate
/// already uses for its per-path refusals (`design/command-output-contract.md` → the stable
/// finding key). The route is the shared idempotent [`provision_route`], with the tail
/// re-derived per verdict — a path holding a directory git cannot vouch for meets the
/// leftover refusal rather than a fresh `git worktree add`, and a route that promised
/// otherwise would be a law-1 lie one command deep.
fn partial_worktree_advisories(
    jigc_home: &Path,
    milestone_id: &str,
    live_ids: &[String],
) -> Vec<Finding> {
    let list = engine::milestone::TaskList {
        tasks: live_ids.to_vec(),
    };
    let subtasks = subtask_worktrees(jigc_home, &list);
    let missing: Vec<&SubtaskWorktree> = subtasks
        .iter()
        .filter(|sub| sub.state != WorktreeState::Live)
        .collect();
    // The two settled states — all provisioned, none provisioned — and the empty milestone,
    // which is both at once. Nothing to say in any of them.
    if missing.is_empty() || missing.len() == subtasks.len() {
        return Vec::new();
    }
    let provisioned = subtasks.len() - missing.len();
    missing
        .into_iter()
        .map(|sub| {
            let (found, tail) = match sub.state {
                WorktreeState::Absent => (
                    "nothing is there",
                    " — idempotent: it reuses every worktree that landed and adds \
                     only the missing ones",
                ),
                WorktreeState::Unreadable => (
                    "a directory git cannot read as a worktree of its own stands there",
                    " — it names what is at that path and refuses until you clear it",
                ),
                WorktreeState::Live => unreachable!("a live worktree is not missing"),
            };
            let address = render::repo_relative(jigc_home, &sub.path);
            Finding::graded(
                Severity::Advisory,
                PARTIAL_WORKTREES_CODE,
                format!(
                    "milestone:{milestone_id}: {provisioned} of {} sub-task worktrees \
                     are provisioned — sub-task `{}` has none ({found}), so its \
                     `Spawn:` line below would run in a working area that does not \
                     exist",
                    subtasks.len(),
                    sub.id,
                ),
                Some(Location::addressed(&address, 1, 1)),
                Some(provision_route(milestone_id, false, tail)),
            )
        })
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
/// silently clobbered ([`reconcile_record_preflight`]); (2) the **held-worktree guard** — the
/// teardown's `git worktree remove --force` is safe at *finalize* (the commit lands first) but on
/// the abandon path the sub-agents' work is **by definition uncommitted**, so a sub-task worktree
/// path holding content refuses the abandon unless `--force` names the consent
/// ([`held_subtask_worktrees`] — the subject is the path, not the registered set); (3) the record settle
/// ([`engine::milestone::discard_record`] — a genuinely **joined** sub-task stays `joined`);
/// (4) a **record-only** commit carrying a CLI-synthesized structural subject (never sweeping the
/// agent's in-flight WIP); (5) the teardown — the sub-task areas, the registered fan-out
/// worktrees, **and** `.jigc/milestones/<id>/` (which no verb removed before: teardown was
/// reachable only from the landed-finalize path, so an abandoned milestone left a cache
/// `provision` would happily re-provision worktrees from).
///
/// Dev-only (no methodology pack → no `milestone-record` schema) degrades exactly as every other
/// record arm does: no record to settle, no commit — and the workbench teardown still runs.
fn run_discard(
    cwd: &Path,
    milestone_id: &str,
    force: bool,
    conflicts: &mut Vec<Finding>,
) -> Result<(String, String)> {
    // The subject is the milestone's own store, so it is `jigc_home` throughout — including
    // the `git worktree` teardown, which acts on the shared admin records and must not be run
    // from a checkout it is about to remove (M53 — the cwd census, the verb class).
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
    // the resolve seam + task-list read, so a teammate on a fresh clone can abandon a
    // milestone whose workbench they never had (`design/team-ready-state.md` → Engine capability 2).
    // An **already-settled** record refuses here (the terminal guard): a milestone that is over is
    // not abandoned twice, and re-running the settle would land an empty record-only commit.
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

    // `<milestone-id>` is the declared dummy-table placeholder (the old `<id>` was
    // undeclared and ambiguous against the task-id sites).
    let list_tasks = engine::finding::Route::mechanical(
        ["jigc", "milestone", "list-tasks", "<milestone-id>"],
        "",
    );
    // The family's shared identity ([`no_such_milestone`]) with **this** door's route:
    // a teardown that answered a wrong id with *"create it first"* would point the
    // operator at the one act they did not ask for (M51 Increment 6 / T2). The **residual**
    // arm of the same seam keeps the family's one residual route, which is already a
    // teardown's — it says nothing was changed and names the path to clear (M53 Increment 3).
    let dir = engine::milestone::require_milestone_area(
        &jigc_root,
        milestone_id,
        engine::finding::Route::human(format!(
            "check the milestone id ({list_tasks} names a live milestone's sub-tasks); nothing was discarded"
        )),
    )
    .map_err(finding_to_err)?;
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;

    // (2) The held-worktree guard — the abandon path's WIP safety
    // (`design/team-ready-state.md` → Abandon refuses on a dirty worktree). The teardown's
    // [`remove_worktrees`] runs `git worktree remove --force`, which is safe at *finalize* (the
    // commit lands first, so every byte the worktree held is already in git) and **destroys
    // uncommitted work** here, where the sub-agents' work is by definition uncommitted. So a
    // sub-task worktree path holding content REFUSES the abandon, naming the paths; `--force` is
    // the human's explicit consent — on the one path whose premise is "throw this away", that
    // intent is exactly what must be confirmed rather than assumed.
    if !force {
        let held = held_subtask_worktrees(&jigc_home, &list);
        if !held.is_empty() {
            return Err(finding_to_err(dirty_worktree_finding(
                milestone_id,
                &jigc_home,
                &held,
            )));
        }
        // (2b) The foreign-byte guard — the abandon's THIRD uncommitted subject, over both
        // of this door's area kinds (M52 Increment 4 / T5). Ahead of the staged-prose guard
        // for the reason `crate::task::refuse_over_foreign_bytes` states: a re-entered
        // sub-task stages `commit:<sub-id>.md`, so asked second this guard would be inert on
        // that cell. Behind the worktree guard, which keeps the precedence it was given.
        refuse_over_foreign_bytes(&jigc_home, &jigc_root, milestone_id, &dir, &list)?;
        // (2c) The staged-prose guard — the abandon's OTHER uncommitted subject
        // (M50 completion audit, finding 4). Inside the same `!force` block, because
        // `--force` is the single consent for every arm of this door.
        refuse_over_subtask_staged_prose(&jigc_home, milestone_id, &list)?;
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
        let mut pre =
            capture_record_pre_image(&jigc_home, &record_path, crate::rollback::MILESTONE_DOOR)?;
        let settled = engine::milestone::discard_record(&record_path, schema, milestone_id)
            .map_err(finding_to_err)?;
        pre.wrote();
        // The message temp file lands in the (gitignored) milestone area — removed by the
        // teardown below, so it is written before the area goes.
        hook_output = commit_record_transaction(
            &jigc_home,
            &record_path,
            &dir,
            &format!("chore(milestone): discard record for milestone:{milestone_id}\n"),
            &pre,
            conflicts,
        )?;
        // Advance the record's file-state baseline to the settled bytes (the [`baseline_record`]
        // discipline every record write follows), so a later store sweep sees no drift.
        baseline_record(&jigc_root, schema, milestone_id, settled.as_bytes());
    }

    // (5) Teardown — the workbench outlives nothing: the sub-task areas, the registered fan-out
    // worktrees, and the milestone area itself.
    //
    // Read the authored prose FIRST and name it once the removal has actually taken it. The
    // sub-task areas hold `docs/*.md` that is in no object DB at all — on the abandon path
    // nothing landed, so the workbench is their only copy — and this door took them at exit 0
    // in silence while `jigc uninstall` refused over byte-identical state (M46 Inc 2
    // validation; the law-1 half-truth that increment removed). Scoped to THIS milestone's
    // sub-task ids, because that is exactly the set [`cleanup_subtask_areas`] removes: an
    // unrelated open task's prose survives and must not be named. Captured after the record
    // commit, so a rejected commit narrates no loss it never took — and narrated after the
    // removal, so an area the removal could not clear is not narrated as gone either
    // ([`PendingLoss`]; `cleanup_subtask_areas` is best-effort and logs its failures).
    let prose = crate::setup::pending_staged_prose(
        &jigc_home,
        &format!("discarding milestone:{milestone_id}"),
        Some(&list.enumerate()),
    );
    // The foreign population of the same areas, read before the removals and named after
    // them ([`crate::task::PendingForeign`], the outcome-keyed rule). Un-forced it is empty
    // by construction — guard (2b) returned — so this costs the ordinary abandon nothing.
    let foreign = crate::task::pending_foreign(
        &jigc_home,
        &discard_foreign_subject(&jigc_root, &dir, &list),
    );
    // The complement goes with the area here: this door's disposition over bytes jigc did
    // not write is a refusal with `--force` as the single consent, and it lives at the door
    // (M52 Increment 4 / T5), never in the sink — `SubtaskComplement`.
    // **The ack keys on the OUTCOME of all three sinks, never on having run them** (M52
    // Increment 4 / T7, D-2). Driven at `f8d5957a` with `.jigc/worktrees` unwritable, this
    // door warned twice that it could not remove a fan-out worktree and then printed
    // `workbench removed` at exit 0 with both worktrees still on disk — the loss narration
    // beside it having been outcome-keyed since M50 while the ack was not. Class size 1: the
    // sibling `milestone finalize` ack was driven on the identical failure and is honest.
    let mut workbench_gone = cleanup_subtask_areas(&jigc_root, &list, SubtaskComplement::Take);
    prose.narrate_taken(&jigc_home);
    workbench_gone &= remove_worktrees(&jigc_home, &list);
    workbench_gone &= remove_milestone_area(&jigc_home, &dir);
    foreign.narrate_taken(&jigc_home);

    // The warnings each sink already printed name *what* is left; the ack's job is to stop
    // claiming otherwise and to point at them.
    let workbench = if workbench_gone {
        "workbench removed"
    } else {
        "workbench NOT fully removed — the warnings above name what is left"
    };
    Ok((
        format!(
            "discarded milestone:{milestone_id} ({} sub-task(s); {workbench})",
            list.enumerate().len()
        ),
        hook_output,
    ))
}

/// [`DISCARD_DOOR`]'s **third** blocking identity — the foreign-byte arm (M52 Increment 4 /
/// T5). Its own code, for the reason the staged-prose split already states: a shared identity
/// would put the wrong door's re-run in front of the operator
/// (`design/surface-contract.md` → law 1).
///
/// Like its two siblings it is a door refusal, so it joins neither
/// `engine::result::CHECK_INVENTORY` nor [`crate::invocation_log::ERROR_CODE_REGISTRY`], and
/// like them it is logged through the `render::BlockedFinding` carrier.
///
/// **Three producers, one subject** (M52 Increment 5 / T7). Beside this door's two — the
/// listing and the fail-closed enumeration fault — [`mint_foreign_bytes_finding`] raises it
/// from the *mint unwind*, where the same state (a milestone workbench holding files jigc did
/// not write) stops a removal rather than a teardown. One state, one identity: a second
/// spelling would make a driver branch twice on one fact. The third producer differs in two
/// ways that are properties of its cell, not of the code — it is **located** at the area,
/// because a single `add-from-spec` unwind can raise one per seed, and it rides the door's
/// rollback-conflict carrier rather than [`finding_to_err`], because the door already has an
/// error the hook wrote and `design/finalize.md` → the M40 refinement 3 forbids replacing it.
const DISCARD_FOREIGN_BYTES_CODE: &str = "milestone.foreign-bytes";

/// **Exactly what this door's teardown removes**, in the order it removes it: each sub-task's
/// working area ([`cleanup_subtask_areas`]' own set, the milestone's id-sorted task list) and
/// then the milestone area itself ([`remove_milestone_area`]).
///
/// One derivation for the guard, the refusal and the narration, so the door cannot refuse over
/// a path its sink never reaches (G-15's *"a sink can be narrower than the guard that cleared
/// it"*). The fan-out worktrees are **not** here: they are `probe_leftover`'s subject, answered
/// through git under M48's `--ignored` adjudication (`settle-record.md` → §6), and a second
/// classifier over them would be two doors answering one question.
fn discard_foreign_subject(
    jigc_root: &Path,
    dir: &Path,
    list: &engine::milestone::TaskList,
) -> Vec<(PathBuf, crate::task::AreaKind)> {
    let tasks_root = jigc_root.join("tasks");
    let mut areas: Vec<(PathBuf, crate::task::AreaKind)> = list
        .enumerate()
        .into_iter()
        .map(|sub_id| (tasks_root.join(sub_id), crate::task::AreaKind::Task))
        .collect();
    areas.push((dir.to_path_buf(), crate::task::AreaKind::Milestone));
    areas
}

/// The `jigc milestone discard` door's guard over every byte jigc did not write in the
/// workbench it is about to tear down — the third of the three consenting doors.
///
/// **Both area kinds.** The milestone area is a second registry row
/// (`engine::state::MILESTONE_AREA_FILES`) and was reachable through no guard at all: driven
/// at `4f311cb1` a file beside `tasks.json` died with the abandon at exit 0, named by nothing.
fn refuse_over_foreign_bytes(
    jigc_home: &Path,
    jigc_root: &Path,
    milestone_id: &str,
    dir: &Path,
    list: &engine::milestone::TaskList,
) -> Result<()> {
    let subject = discard_foreign_subject(jigc_root, dir, list);
    let found = crate::task::foreign_areas(jigc_home, &subject)
        .map_err(|err| finding_to_err(unverified_foreign_finding(milestone_id, err)))?;
    let lines = crate::task::foreign_lines(&found);
    if lines.is_empty() {
        return Ok(());
    }
    Err(finding_to_err(foreign_bytes_finding(milestone_id, &lines)))
}

/// The abandon door's foreign-byte refusal: a blocking, route-bearing finding naming the
/// milestone and every path across its workbench that jigc did not write.
///
/// The route is this door's own — one consent, carrying the real milestone id — and it states
/// the fact that makes the paths worth stopping for rather than a claim about their content:
/// `.jigc/` is gitignored whole, so the workbench is their only copy.
fn foreign_bytes_finding(milestone_id: &str, lines: &[String]) -> Finding {
    Finding::block(
        DISCARD_FOREIGN_BYTES_CODE,
        format!(
            "milestone:{milestone_id}: its workbench holds {} path(s) jigc did not write — \
             `.jigc/` is gitignored, so abandoning the milestone would destroy bytes nothing \
             else has a copy of:\n{}",
            lines.len(),
            crate::task::foreign_listing(lines),
        ),
        format!(
            "move what you need out of the paths above, or delete what you do not, then \
             re-run `jigc milestone discard {milestone_id}` — or, once you have confirmed \
             they hold nothing you need, `jigc milestone discard {milestone_id} --force` \
             settles the record and tears the workbench down with them"
        ),
    )
}

/// The fail-closed half of [`refuse_over_foreign_bytes`], keyed to this door: the complement
/// could not be enumerated, so the workbench is not torn down with those bytes' existence
/// unknown.
fn unverified_foreign_finding(milestone_id: &str, err: std::io::Error) -> Finding {
    Finding::block(
        DISCARD_FOREIGN_BYTES_CODE,
        format!(
            "cannot check milestone:{milestone_id}'s workbench for files jigc did not write, \
             so abandoning it could destroy bytes nothing else has a copy of: {err}"
        ),
        format!(
            "make sure `.jigc/tasks/` and `.jigc/milestones/{milestone_id}/` are readable, \
             then re-run `jigc milestone discard {milestone_id}` — or, once you have \
             confirmed the milestone holds nothing you need, `jigc milestone discard \
             {milestone_id} --force` tears the workbench down unchecked"
        ),
    )
}

/// [`DISCARD_DOOR`]'s **second** blocking identity — the staged-prose arm, distinct from
/// [`DISCARD_CODE`]'s worktree arm because the two refusals have different subjects and
/// different exits, and a reader must be able to tell which one fired without parsing prose
/// (`design/surface-contract.md` → law 1; the shipped `uninstall.dirty-worktree` /
/// `uninstall.staged-prose` pair is the same split at the install door).
///
/// It is **not** [`DestroyingDoor::code`]: that field is the destroying-door table's
/// refuse-vs-narrate discriminator over *worktree-shaped paths*, which is the axis
/// [`DESTROYING_DOORS`] iterates. This refusal is about `.jigc/tasks/<sub-id>/docs/`, which
/// no worktree contains — which is precisely why the worktree guard could not see it.
///
/// A door refusal, not a probe result and not a commit-phase rejection, so it joins neither
/// `engine::result::CHECK_INVENTORY` nor [`crate::invocation_log::ERROR_CODE_REGISTRY`] —
/// exactly like the `task-discard.staged-prose` sibling it pairs with.
const DISCARD_STAGED_PROSE_CODE: &str = "milestone.staged-prose";

/// The `jigc milestone discard` door's WIP guard over its sub-tasks' **staged docs** — the
/// third door of the pair M50's D1 exists to reconcile (M50 completion audit, finding 4).
///
/// This door already read [`crate::task::staged_task_prose`] through
/// [`crate::setup::pending_staged_prose`], but only for **narration**: it named the bytes and
/// then took them at exit 0, while `jigc task discard` refused over the identical files under
/// `task-discard.staged-prose`. That is D1's own contradiction — one probe, two doors, opposite
/// answers — one door over.
///
/// **Why the worktree guard was not already enough**, which is the warrant that was retired
/// rather than reworded: `design/team-ready-state.md` justified the narrating arm here with
/// *"the refusal it does carry is the worktree one"*, and driven over the 2×2 that clause is
/// false in the only cell where bytes die. An agent authoring through jigc writes into
/// `.jigc/tasks/<sub-id>/docs/`, which is **not inside the worktree**, so
/// `git status --porcelain` reads clean while the sub-task's `commit:<sub-id>.md` holds
/// authored prose. The worktree probe cannot see this subject at all.
///
/// **Scoped to this milestone's sub-tasks** — [`cleanup_subtask_areas`]'s exact set, the same
/// scope the narration uses. A door must neither claim nor refuse over prose it will not
/// touch, so an unrelated open task's staged docs are none of this door's business.
///
/// **The guard is keyed on staged bytes, not on being a milestone.** `milestone add-task` and
/// `milestone provision` stage nothing, so a milestone abandoned before anyone re-entered a
/// sub-task still discards with no consent — the omitting context that keeps this from being
/// a `--force` trainer on the ordinary path (M46 Inc 2's measured objection, which is why the
/// *landed* [`FINALIZE_DOOR`] still narrates rather than refuses).
fn refuse_over_subtask_staged_prose(
    jigc_home: &Path,
    milestone_id: &str,
    list: &engine::milestone::TaskList,
) -> Result<()> {
    let only = list.enumerate();
    let staged = crate::task::staged_task_prose(jigc_home, Some(&only), &|err| {
        unverified_subtask_prose_finding(milestone_id, err)
    })
    .map_err(finding_to_err)?;
    if staged.is_empty() {
        return Ok(());
    }
    Err(finding_to_err(subtask_staged_prose_finding(
        milestone_id,
        &staged,
    )))
}

/// The abandon door's staged-doc refusal: a blocking, route-bearing finding naming each of
/// **this milestone's** sub-tasks and the doc identities its area stages.
///
/// **It claims "staged doc(s)", not "authored prose"** — the claim discipline both siblings
/// carry ([`crate::setup::staged_prose_finding`], `crate::task`'s discard finding): that the
/// docs are staged and in no commit is exactly what the probe measured, and whether a human
/// or the mint wrote their bytes is not something it can tell.
///
/// **Three exits, at this door's unit kind**, mirroring the task door's read → land → consent
/// order: `jigc doc show <address> --task <sub-id>` reads what is about to be lost (the listed
/// identities *are* the addresses it takes), `jigc milestone finalize <id>` lands the whole
/// boundary, and the consent comes last carrying the real milestone id. Naming
/// `jigc task finalize` here would be the wrong unit kind — a milestone sub-task's only commit
/// boundary is the milestone's (`design/write-commands.md` → Abandoning a milestone).
///
/// **The land exit's caveat is this door's, not the task door's** — driven, not copied. The
/// task door says *"refuses while a required slot is empty"*, which is true of `jigc task
/// finalize` over an empty commit skeleton; at *this* door the dominant cell (a sub-task
/// staging only its transient `commit:<sub-id>`, no promotable doc and no staged code)
/// refuses **earlier and for a different reason** — `milestone.zero-contribution`, driven
/// at `c344396`. Naming the task door's reason here would state a cause that does not fire,
/// which is the same law-1 defect this whole guard exists to close, so the caveat names
/// both conditions and claims neither exclusively.
fn subtask_staged_prose_finding(milestone_id: &str, staged: &[(String, Vec<String>)]) -> Finding {
    let listing: Vec<String> = staged
        .iter()
        .map(|(task, docs)| format!("  {task}: {}", docs.join(", ")))
        .collect();
    let docs: usize = staged.iter().map(|(_, docs)| docs.len()).sum();
    Finding::block(
        DISCARD_STAGED_PROSE_CODE,
        format!(
            "milestone:{milestone_id}: {} sub-task(s) stage {docs} doc(s) that no commit \
             has a copy of — abandoning the milestone would destroy them:\n{}",
            staged.len(),
            listing.join("\n"),
        ),
        format!(
            "read what is in them with `jigc doc show <address> --task <sub-task-id>` (the \
             sub-task ids are listed above), or land the milestone with \
             `jigc milestone finalize {milestone_id}` (which refuses, with its own route, \
             while the milestone has nothing to land or a required slot is empty) — or, \
             once you have confirmed the milestone holds nothing you need, \
             `jigc milestone discard {milestone_id} --force` settles the record and tears \
             the workbench down with them"
        ),
    )
}

/// The abandon door's fail-closed sibling, keyed to **this** door (D1's per-door-identity
/// rule): the staged set could not be enumerated, so the workbench is not torn down with its
/// sub-tasks' staged docs' existence unknown — "enumerated nothing" and "there is nothing" are
/// the same bytes to a caller about to `remove_dir_all`, and only one of them is safe.
///
/// Minting one identity for all three doors would put *"re-run `jigc uninstall`"* in front of
/// an operator standing at `jigc milestone discard` — the cross-door misdirection this family
/// exists to close, reintroduced by the very sharing that closes it.
///
/// Same claim discipline: the probe never ran, so it names only what it was looking for.
fn unverified_subtask_prose_finding(milestone_id: &str, err: std::io::Error) -> Finding {
    Finding::block(
        DISCARD_STAGED_PROSE_CODE,
        format!(
            "cannot check milestone:{milestone_id}'s sub-task working areas for staged \
             docs, so abandoning it could destroy work no commit has a copy of: {err}"
        ),
        format!(
            "make sure `.jigc/tasks/` is readable, then re-run \
             `jigc milestone discard {milestone_id}` — or, once you have confirmed the \
             milestone holds nothing you need, \
             `jigc milestone discard {milestone_id} --force` tears the workbench down \
             unchecked"
        ),
    )
}

/// The milestone's provisioned worktrees that hold **uncommitted work**, each paired with the
/// `git status --porcelain` entries that make it dirty — the abandon path's WIP probe
/// (`design/team-ready-state.md` → Abandon refuses on a dirty worktree).
///
/// **It answers about bytes, and that is the whole of what it answers** (the independent
/// review of `3c71da87`, the HIGH). A worktree git has left mid-operation over a *spotless*
/// tree is invisible here by construction, and the second leg that sees it lives at the
/// caller, beside this one: [`LeftoverHold::operation`]. Widening this probe instead would
/// have put a repository-state question behind a `git status` name.
///
/// **`--porcelain`, not `git diff --cached`.** Its sibling [`worktrees_have_staged_code`] reads
/// only the *staged* set, because at `finalize` the staged set is what the combine commits — but
/// what the abandon path destroys is **everything** in the worktree: staged, unstaged, and
/// **untracked** alike (`git worktree remove --force` deletes the checkout). So the guard's probe
/// is the union `git status --porcelain` reports.
///
/// **And that union stays without `--ignored`, deliberately** — this is the *refusal* probe, and
/// [`discarded_work`] is the *narration* probe that does read the ignored set and names those
/// bytes; the measured evidence for the split and the declared cost (**visible, not prevented**)
/// live there, its re-opening condition with the deferral it disposes
/// (`implementation/decisions-pending.md` → entry 11). The scope that makes the two policies in
/// this file siblings rather than a contradiction: **here git can place the bytes**, so the
/// project's own ignore rules are readable and the ordinary fan-out success path is full of build
/// output — whereas at a path git cannot place at all ([`probe_leftover`]'s fail-closed arm)
/// *any* child refuses, ignored or not, because there is nothing to scope a refusal with.
///
/// `pub(crate)` for the sibling teardown — `jigc uninstall`'s `remove_dir_all(.jigc)` destroys
/// exactly the same bytes (the worktrees live under `.jigc/worktrees/`), so it guards on this
/// same probe rather than growing a second, drifting one
/// (`crate::setup::dirty_fanout_worktrees`).
///
/// **Its failure names the host path, deliberately** (M50 Increment 12 / T1). Every other path
/// this module puts on a surface renders through [`crate::render::repo_relative`]; this one
/// does not, because the line is **quoting an invocation** rather than addressing a doc —
/// `` `git status --porcelain` in worktree <wt> failed: <git's own stderr> `` — and the path is
/// the argument jigc handed git, printed beside git's own words naming that same absolute.
/// Relativizing one half would misquote the command that failed. Disposed as
/// `DeclaredAbsolute` in `crates/cli/tests/repo_relative_paths.rs`.
pub(crate) fn dirty_worktrees(worktrees: &[PathBuf]) -> Result<Vec<(PathBuf, Vec<String>)>> {
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

/// What a sub-task's worktree-shaped path holds, and what [`remove_worktrees`] would actually do
/// with it — the two halves [`dirty_worktree_finding`] needs to refuse without lying.
struct HeldWorktree {
    /// The `.jigc/worktrees/<sub-task-id>` path, canonical (the [`registered_worktrees`] convention).
    path: PathBuf,
    /// What the probe found there — the entries a removal would take, the bare fact that the
    /// path is a file, or the reason nothing could be read ([`LeftoverHold`]).
    hold: LeftoverHold,
    /// Whether the teardown reaches this path at all: it removes **registered** worktrees and
    /// skips everything else, so an unregistered path is *orphaned*, never deleted.
    registered: bool,
}

/// Every sub-task worktree path of `list` that holds content the abandon must not step past —
/// [`DISCARD_DOOR`]'s guard set (`DECISIONS.md` 2026-08-13 → the Settle, F3).
///
/// **The subject is the path, not the registered set.** The retired subject was
/// [`provisioned_worktrees`] (the task list ∩ the *registered* worktrees), which is structurally
/// blind to the ordinary trigger: a `cp -R` or `mv` of the repo leaves the copy's worktrees
/// registered at the **source's** path, so nothing under the copy's own `.jigc/worktrees/` is
/// registered there and the guard saw nothing exactly where the live work was. Every sub-task's
/// path is now probed by the shared fail-closed [`probe_leftover`] — the same classifier
/// [`PROVISION_DOOR`] and [`UNINSTALL_DOOR`] ask — so a live worktree still clears when clean, and
/// a path git cannot vouch for refuses on any content.
///
/// Fail-closed: an unreadable directory or `git status` is a [`LeftoverShape::Unreadable`] hold
/// rather than an empty set, because *"found nothing"* and *"there is nothing"* are the same
/// bytes to the caller and only one of them is safe to abandon on. Since M50 it is collected
/// with its siblings instead of ending the walk, so the refusal names every path (RC-m50 N9) —
/// which is also why this **returns the set rather than a `Result`**: with the probe unable to
/// fail, an `Ok`-only signature would advertise an error path the door no longer has.
fn held_subtask_worktrees(
    jigc_home: &Path,
    list: &engine::milestone::TaskList,
) -> Vec<HeldWorktree> {
    let registered = registered_worktrees(jigc_home).unwrap_or_default();
    // Match `provision_worktrees`' canonical-path convention (git stores canonical paths at
    // `add` time); fall back to the raw path if canonicalization fails — the probe still runs,
    // and nothing matches the registered set, which is the fail-closed side.
    let canonical_home = jigc_home
        .canonicalize()
        .unwrap_or_else(|_| jigc_home.to_path_buf());
    let mut held = Vec::new();
    for sub_id in list.enumerate() {
        let path = canonical_home.join(worktree_path(&sub_id));
        // The probe's own failure is a hold like any other now, so it is **collected** with
        // its siblings rather than propagated with `?` — a `?` here answered for the first
        // unreadable path and named none of the rest (RC-m50 N9).
        if let Some(hold) = probe_leftover(jigc_home, &path) {
            held.push(HeldWorktree {
                registered: registered.iter().any(|w| w == &path),
                path,
                hold,
            });
        }
    }
    held
}

/// [`DISCARD_DOOR`]'s refusal: a blocking, route-bearing finding naming every sub-task worktree
/// path that holds content, what is in it, and **what the abandon would do to it**.
///
/// The per-path disposition is not decoration — it is law 1 (`design/surface-contract.md`). The
/// teardown removes a **registered** worktree and skips everything else, so claiming a removal for
/// a path [`remove_worktrees`] never reaches would be a lie, and so would calling a plain
/// directory's child names *uncommitted work* when no git vouched for them. Each line says only
/// what is true of that path.
///
/// The route names both honest exits — get the content out, or `--force` to abandon anyway (the
/// [`crate::combine::detect_code_collision`] finding idiom) — and states what `--force` really
/// costs on each disposition, since on the orphaning one it costs nothing on disk.
fn dirty_worktree_finding(milestone_id: &str, jigc_home: &Path, held: &[HeldWorktree]) -> Finding {
    let listing: Vec<String> = held
        .iter()
        .map(|w| {
            let fate = if w.registered {
                "registered here, so the teardown removes it and this content is destroyed"
            } else {
                "not registered here, so the teardown leaves it on disk with no milestone naming it"
            };
            format!(
                "  {} — {}; {fate}",
                hold_line(jigc_home, &w.path, &w.hold),
                because(&w.hold),
            )
        })
        .collect();
    let address = render::repo_relative(jigc_home, &held[0].path);
    let operation_clause = if held.iter().any(|w| w.hold.operation.is_some()) {
        OPERATION_CLAUSE
    } else {
        ""
    };
    Finding::graded(
        Severity::Blocking,
        DISCARD_CODE,
        format!(
            "milestone:{milestone_id}: {} sub-task worktree path(s) hold something the abandon \
             cannot prove is disposable, and `{}` would settle the record and tear the \
             workbench down over them:\n{}",
            held.len(),
            DISCARD_DOOR.verb,
            listing.join("\n"),
        ),
        Some(Location::addressed(&address, 1, 1)),
        Some(
            format!(
                "look at those paths and get out what you need (commit or stash what a live \
             worktree holds; a path listed as a file or as unreadable is not a worktree at \
             all, so move it aside or delete it yourself), then re-run \
             `jigc milestone discard {milestone_id}` — or run \
             `jigc milestone discard {milestone_id} --force` to abandon the milestone \
             anyway: a registered worktree is removed with everything \
             uncommitted in it, and a path nothing vouches for is left behind on disk for \
             you to deal with{operation_clause}"
            )
            .into(),
        ),
    )
}

/// Remove the milestone's gitignored workbench `.jigc/milestones/<id>/` on a settled discard —
/// the base pin + task-list cache the record no longer stands behind (`design/team-ready-state.md`
/// → The workbench is actually removed). The **only** remover outside the landed-finalize
/// executor: without it an abandoned milestone leaves a cache `provision` would re-provision
/// worktrees from. Best-effort, logged-not-raised — the record commit has already landed, so a
/// cleanup failure must not fail it (the [`cleanup_subtask_areas`] self-heal stance).
///
/// **Returns whether the area is gone** — the ack keys on the outcome, never on the attempt
/// (M52 Increment 4 / T7, D-2).
fn remove_milestone_area(jigc_home: &Path, dir: &Path) -> bool {
    if dir.exists()
        && let Err(err) = std::fs::remove_dir_all(dir)
    {
        eprintln!(
            "note: milestone workbench removal at `{}` failed (self-heals): {err:#}",
            render::repo_relative(jigc_home, dir),
        );
        return false;
    }
    true
}

/// Dispatch `jigc milestone execute <milestone-id>`: compose the milestone-execution
/// workflow over the milestone's id-sorted sub-task list, render the composed view on
/// stdout, and map it to the exit code. An unknown milestone (no area) or a blocking
/// compose finding routes to stderr **before** any output and exits non-zero
/// (`design/write-commands.md` → Executing the milestone).
fn dispatch_execute(cwd: &Path, format: Format, milestone_id: &str) -> Outcome {
    match run_execute(cwd, milestone_id) {
        Ok((view, advisories)) => {
            // The provisioning state FIRST, then the walk it is about to be read as (law 3 —
            // nothing ambushes: an agent that learns halfway down a `Spawn:` list that one of
            // the working areas is missing has already started). Presentation-only and
            // non-blocking either way: the composed `--format json` stays the pinned
            // `{task, text}` contract byte-for-byte, and stream discipline mirrors the
            // finalize and migrate advisories (`task.rs::emit_left_out_advisory`) — agent /
            // human text to **stdout** where the agent reads the walk, but under
            // `--format json` the structured envelope owns stdout, so the advisory goes to
            // **stderr** and the document's bytes never move.
            for advisory in &advisories {
                let line = render::advisory_line(advisory);
                if matches!(format, Format::Json) {
                    eprint!("{line}");
                } else {
                    print!("{line}");
                }
            }
            println!("{}", render::composed(format, &view));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// `jigc milestone execute <milestone-id>` — resolve the milestone work-unit (an
/// unknown id routes a blocking finding **before** any compose, the `dispatch_finalize`
/// precedent), read its **id-sorted** `TaskList::enumerate()`, and compose the
/// `creates-task: false` `milestone-execution` workflow over it — feeding the id-sorted
/// list into `{{milestone.tasks}}` so the `fan-out` step resolves it (one `Spawn:`
/// directive per sub-task, id-ordered, each naming **that sub-task's recorded minting
/// workflow** — [`recorded_workflows`]). The lone production site feeding
/// [`ComposeContext::milestone`] non-empty; **mints nothing** (the milestone and its
/// sub-tasks already exist).
///
/// Returns the composition paired with the **partial-provisioning** advisories
/// ([`partial_worktree_advisories`]) — the one thing this read knows that the composed walk
/// cannot say for itself, because the walk is compiled from the pack and the provisioning
/// state lives on disk.
fn run_execute(
    cwd: &Path,
    milestone_id: &str,
) -> Result<(crate::start::Composition, Vec<Finding>)> {
    // The `.jigc/` area binds to jigc_home (the main checkout); the compose feed resolves
    // the same split internally.
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    let jigc_root = jigc_home.join(".jigc");
    // Fresh-clone resume (M39 T5): re-derive the demoted cache from the committed record before
    // the resolve seam + task-list read, so `execute` on a fresh clone re-derives the
    // milestone shape and composes the fan-out (`design/team-ready-state.md` → Engine capability 2).
    let schemas = shipped_schemas(&jigc_home)?;
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;
    let dir = require_milestone_area(&jigc_root, milestone_id)?;
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;
    // Enumeration is id-sorted — the order the fan-out emits its `Spawn:` directives — and it
    // is the **live** set: a `Spawn:` line for a settled sub-task dead-ends on *"no task"* and
    // routes back at the listing that names it again ([`live_sub_task_ids`]).
    let ids = live_sub_task_ids(&jigc_home, &schemas, milestone_id, list.enumerate())?;
    // The provisioning state the fan-out is about to be told to run in — read before the
    // compose so the ids are the same live set the `Spawn:` lines are emitted over.
    let advisories = partial_worktree_advisories(&jigc_home, milestone_id, &ids);
    // Each id paired with its own recorded minting workflow, so the emitted `Spawn:` line names
    // the workflow the re-entry door will accept ([`recorded_workflows`]).
    let tasks = recorded_workflows(&jigc_home, &jigc_root, ids)?;
    // The compose door takes a **start path** and does its own walk-up + jigc_home split, so
    // it is handed the caller's cwd rather than a root resolved here: a compose's base pin is
    // deliberately the standing checkout's HEAD (M31 Inc 2 / WF3), and `cwd` and the walk-up
    // root it replaces resolve to the same checkout from every directory inside it.
    let view = crate::start::execute_milestone_in_repo(
        cwd,
        MILESTONE_EXECUTION_WORKFLOW,
        crate::start::MilestoneFeed::bound(milestone_id, &tasks),
    )?;
    Ok((view, advisories))
}

/// Dispatch `jigc milestone join <milestone-id>`: run the by-task-id join, render the
/// merged outcome, and map it to the exit code. A locator/IO error or an unknown
/// milestone (an `Err(Finding)`) routes to stderr and exits non-zero **before** any
/// summary. The outcome always renders on stdout (so the agent sees the suffix/rewrite
/// decisions); if any **blocking** finding rode inside it — a same-doc clash, an
/// isolation violation — the house finding line goes to stderr **first** and the process
/// exits non-zero, and the stdout headline says `join blocked:` rather than claiming a
/// join. The verb **commits nothing** (Increment 4 wires the suffix-resolved overlay
/// into finalize); a clash leaves the working tree untouched.
fn dispatch_join(cwd: &Path, format: Format, milestone_id: &str) -> Outcome {
    let (outcome, staged_by_sub_task) = match run_join(cwd, milestone_id) {
        Ok(outcome) => outcome,
        Err(err) => {
            return crate::invocation_log::operational_failure(format, &err);
        }
    };
    // A blocking finding inside the outcome (e.g. `join.same-doc-clash`) routes to
    // stderr and gates the exit code — nothing is committed regardless. It prints
    // **before** the stdout ack (M50 Increment 10 / T2): that ack ends in the routing
    // footer, and a reader that stops at the footer stopped before the verdict, so in
    // the merged read an agent actually gets the block used to arrive after the line
    // that reads as the end of the output. Stream discipline is unchanged — the finding
    // is on stderr, the ack on stdout; only the order moved.
    let blocking: Vec<&Finding> = outcome
        .findings
        .iter()
        .filter(|f| f.severity == engine::finding::Severity::Blocking)
        .collect();
    let mut finding_codes = Vec::with_capacity(blocking.len());
    for finding in &blocking {
        // The **house** findings line, whole — head, locus, route, in that order
        // ([`crate::render::finding_line`], M50 Increment 10 / T1). A blocked join
        // printed its message and pushed its code into the invocation log in the same
        // loop, while the reader got no code at all. `eprint!`, not `eprintln!`: the
        // house line already ends in `\n`.
        eprint!("{}", crate::render::finding_line(finding, false));
        finding_codes.push(finding.code.clone());
    }

    // The merged-overlay summary always prints (the agent reads the suffix/rewrite
    // decisions even when the join is clean). The milestone's full id-sorted sub-task
    // set rides along — each id paired with whether its own area staged anything — so the
    // ack can name the doc-less sub-tasks (C3) from what each sub-task actually staged
    // rather than from what survived the merge (M51 Increment 9 / T6). Its headline
    // states which verdict this run had — [`render::milestone_join`] opens `join
    // blocked:` over a blocking outcome, never `joined milestone:`.
    println!(
        "{}",
        render::milestone_join(format, milestone_id, &outcome, &staged_by_sub_task)
    );

    if blocking.is_empty() {
        Outcome::success()
    } else {
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
/// join outcome paired with the milestone's full **id-sorted** live sub-task set, each id
/// paired with **whether its own area staged any doc** — the input the C3 ack names the
/// doc-less members from ([`staged_by_sub_task`]).
fn run_join(
    cwd: &Path,
    milestone_id: &str,
) -> Result<(JoinOutcome, std::collections::BTreeMap<String, bool>)> {
    // The engine `join` itself performs no git I/O (the base is the milestone's *stored*
    // pin), but the CLI around it does: the stale-base guard shells to git, and the
    // cross-worktree collision read (below) reads each provisioned worktree's staged set.
    // The committed doc-store, the `.jigc/` index, the fan-out worktrees and their
    // registration all bind to jigc_home (the main checkout): `git worktree list` answers for
    // the whole repository from any of its checkouts, and the paths it is compared against are
    // built from jigc_home (M53 — the cwd census, the verb class).
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
    // The **live** set: a sub-task the record has settled is not a contributor that merely
    // staged nothing, and the code left in its worktree can never land (the finalize combine
    // folds the live set too), so it neither joins the doc-less line nor contends in the
    // cross-worktree collision read ([`live_sub_task_ids`]).
    let list = engine::milestone::TaskList {
        tasks: live_sub_task_ids(&jigc_home, &schemas, milestone_id, list.enumerate())?,
    };

    // Cross-worktree code collisions surface HERE, not only at finalize: two sub-tasks
    // staging the SAME path in their isolated fan-out worktrees is a clash the combine
    // never text-merges (`design/storage.md` → the join's never-blind-merge discipline).
    // A never-provisioned (docs-only) milestone yields no worktrees, so the read is inert.
    let worktrees = provisioned_worktrees(&jigc_home, &list);
    if let Some(finding) = crate::combine::detect_code_collision(&worktrees)? {
        outcome.findings.push(finding);
    }
    Ok((outcome, staged_by_sub_task(&jigc_root, &list.enumerate())?))
}

/// Each live sub-task id paired with **whether its own area staged any doc** — the
/// doc-less line's subject, read per sub-area from the same `provenance.json` the engine
/// join itself folds from ([`engine::state::ProvenanceRecord::load`], whose absent-file
/// case *is* the nothing-staged-yet case). `BTreeMap`-keyed, so the ack's listing is
/// id-sorted by construction and no enumeration order can reach the output.
///
/// **Why the disk and not the merge outcome** (M51 Increment 9 / T6; charter Tier 2
/// **EC-15**): a `join.same-doc-clash` keeps the contending address group out of the
/// merged overlay, so deriving *"who staged nothing"* from that overlay named every
/// contender — each of which had staged the clashing doc — as having staged nothing.
fn staged_by_sub_task(
    jigc_root: &Path,
    sub_ids: &[String],
) -> Result<std::collections::BTreeMap<String, bool>> {
    let mut staged = std::collections::BTreeMap::new();
    for sub_id in sub_ids {
        let sub_dir = jigc_root.join("tasks").join(sub_id);
        let record = engine::state::ProvenanceRecord::load(&sub_dir).with_context(|| {
            format!("could not read the provenance manifest of sub-task `{sub_id}`")
        })?;
        staged.insert(sub_id.clone(), !record.docs.is_empty());
    }
    Ok(staged)
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
        Err(err) => crate::invocation_log::operational_failure(format, &err),
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
    // **The whole boundary's subject is `jigc_home`, the main checkout — the cwd decides
    // nothing here** (M53 — the cwd census, rows C2-06 and C2-11). The comment this replaces
    // said HEAD and the commit/stage stay on the walk-up `repo_root`, and called the worktree
    // case *the deferred WF4/WF5 concern*; driven, that deferral was a hard fault and a
    // silent gate hole at once. From inside any provisioned fan-out worktree — the one cwd
    // jigc's own `milestone execute` spawn line puts a sub-agent in — the boundary composed a
    // mixed pathspec list (`.jigc/config`, `.jigc/.gitignore`, and the milestone record's
    // path, which could not be stripped against the worktree and came out ABSOLUTE) and ran
    // it with `current_dir` = that worktree: ``git add -- … failed: '…/<id>.md' is outside
    // repository at '…/.jigc/worktrees/<sub>'``, a raw git error carrying no code, no route
    // and no `at:`. Behind it, the commit seam's own subject was
    // `SeamSubject::live(<the worktree>)`, which refuses `repo.head-detached` because jigc
    // provisions every fan-out worktree `--detach`. And behind *that*, the base-mismatch gate
    // read the standing checkout's HEAD — a fan-out worktree is pinned to the base by
    // construction, so from inside one the gate could not fire at all, and only the git fault
    // above stopped the run, by accident rather than by guard.
    //
    // There was never a second subject to choose. Every read and write below is about the
    // milestone: the record lives under `jigc_home`'s docs-root, the carryover snapshot was
    // taken in `jigc_home`'s index by `milestone create`, the promote destination is
    // `jigc_home`'s store, the combine's dedicated worktree belongs under `jigc_home`'s
    // `.jigc/worktrees/`, and the commit this boundary lands is the main checkout's. Outside
    // a worktree the two paths are byte-identical by construction (`crate::repo::jigc_home`),
    // so nothing moves for the ordinary caller.
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
    // the resolve seam + base-pin/task-list reads + the engine `materialize`'s internal
    // cache reads, so `finalize` on a fresh clone re-derives the milestone shape and joins the
    // recorded sub-tasks (`design/team-ready-state.md` → Engine capability 2 (read-back)). A
    // **settled** record refuses here (the terminal guard): a landed milestone is not re-joined,
    // and an abandoned one is not finalized.
    reseed_cache(&jigc_home, &jigc_root, &schemas, milestone_id)?;

    let dir = require_milestone_area(&jigc_root, milestone_id)?;

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
    guard_base_live(&jigc_home, milestone_id, &base)?;
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
    // and the combine channel below (the M31 sibling-site shape — the gate's inputs sit
    // above the record flip, and the `squash` read it needs was hoisted to join them).
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;
    // The boundary lands the **live** sub-tasks: a settled one contributed nothing by
    // definition — its area is gone and never rebuilt — so it is credited in no manifest, named
    // in no synthesized message, and its worktree's staged code is not folded into the commit
    // ([`live_sub_task_ids`]). The **full** `list` stays the teardown's subject below: a
    // sub-task discard leaves the provisioned worktree standing, and this is one of the two
    // doors that can still remove it.
    let live = engine::milestone::TaskList {
        tasks: live_sub_task_ids(&jigc_home, &schemas, milestone_id, list.enumerate())?,
    };
    // The subject is the **path**, classified once (M46 Inc 2 T1) and shared by the gate,
    // the combine channel and the manifest's contribution facts below.
    let subtasks = subtask_worktrees(&jigc_home, &live);
    let worktrees = live_worktrees(&subtasks);

    // The repository posture of every checkout this boundary commits **from** (M53
    // post-review fix; [`fan_out_posture_findings`]). The door guard answered for the main
    // checkout before dispatch; these worktrees are the other subjects, and until this
    // landed nothing asked them at all. Placed here — as soon as the worktree set exists,
    // beside the boundary gate whose position rule it shares — because nothing durable has
    // been written yet: `materialize` rebuilds its dir every call and the `RecordFlipGuard`
    // is not armed until below, so a block here truly commits nothing and leaves the
    // milestone finalizable once the user has concluded or abandoned the operation.
    let breaches = fan_out_posture_findings(&jigc_home, &worktrees);
    if !breaches.is_empty() {
        return blocked(&jigc_home, format, breaches);
    }

    let staging_dir = materialized
        .docs_dir
        .parent()
        .expect("the materialized docs dir has a parent staging area")
        .to_path_buf();

    // The `finalize.fan-out.squash` knob (`design/finalize.md` → `fan-out` finalize) shapes
    // the commit. Resolve it from the project cascade (a missing `.jigc/config/` layer
    // yields the pack-default base — `squash` defaults `true`, the M7 single-aggregate form
    // whose bytes stay byte-identical to today, the read-side determinism guard). `false`
    // lays down one commit per sub-task in **id-sorted order** (rendering each sub-task's
    // own authored `commit:<sub-id>` doc) BEFORE the parent's synthesized aggregate — the
    // commit *sequence* is a pure function of the id set (hardening #7), even though each
    // sub-task message carries the sub-agent's authored prose (the CLI owns the ordering).
    // Read HERE — above the boundary gate, which needs it to know whether any transient
    // commit doc is part of what this boundary commits — and still before any sub-task
    // commit moves HEAD; the per-sub-task commits are laid down only AFTER the planner's
    // preflight validates base == HEAD below.
    let squash = resolve_squash(&jigc_home)?;

    // The milestone-boundary conformance gate (M45 — `design/finalize.md` → 2. Validate;
    // `design/validation.md` → The milestone-boundary gate): validate the **merged effective
    // state** — the by-task-id merged doc set (`materialize`'s output, on disk in
    // `merged/docs/`) overlaid on the committed store for the `schema-conformance.*` +
    // `ref-resolves` families, plus the `doc-code` code-anchor re-resolution over the merged
    // worktree code tree — and BLOCK (exit 3, committing nothing) on any blocking finding.
    // Positioned after `materialize` (whose output IS the merged doc set — a gate ahead of it
    // would re-implement the merge), after the base guard (the gate reads `base.sha`, not the
    // literal `HEAD`), after `provisioned_worktrees`, and BEFORE the record flip + the plan +
    // the `squash` branch. Nothing durable is written yet (`materialize` rebuilds its dir
    // every call, and the `RecordFlipGuard` is not yet armed), so a block here truly commits
    // nothing. A same-path collision (no single merged tree) is NOT the gate's concern — it
    // falls through to the knob branch's own collision handling below.
    if let Some(outcome) = milestone_boundary_gate(
        &jigc_home,
        &jigc_home,
        &jigc_root,
        &schemas,
        &staging_dir,
        &base,
        &worktrees,
        &materialized.sources,
        squash,
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
    // The sink the guard's destructor pushes a raced restore's conflicts into — owned here, so
    // both `squash` arms' refusal paths can drain it ([`drain_record_flip`]).
    let record_flip_sink: std::cell::RefCell<Vec<Finding>> = std::cell::RefCell::new(Vec::new());
    let mut record_flip = flip_record_for_finalize(
        &jigc_root,
        &jigc_home,
        &schemas,
        milestone_id,
        &record_flip_sink,
    )?;
    let record_changed = record_flip.as_ref().is_some_and(|f| f.changed);
    let record_pathspec = record_flip.as_ref().map(|f| f.pathspec.clone());

    // Step 2 — the CLI-synthesized message (a milestone has no commit doc to render).
    let message = synthesized_message(milestone_id, &live);

    // The diff-presence signal the planner's empty-commit guard needs, narrowed to the
    // worktree-isolation model (M31 Inc 4): the materialized docs that will be promoted, OR
    // any worktree's staged code (Σ `git diff --cached`). The main checkout no longer holds
    // the fan-out's code — it lives in the isolated worktrees — so a main-checkout
    // diff/untracked scan would both miss the real code and false-count unrelated WIP.
    let head = git_head(&jigc_home)?;
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
    let contributions = subtask_contributions(&subtasks, &materialized.sources)?;

    // The finalize base-guard refinement (`design/team-ready-state.md` → The commit model: the
    // finalize base-guard refinement; `DECISIONS.md` 2026-07-07). Per-op record commits advance
    // HEAD past the pinned base, so `base == HEAD` can never hold once record commits land. The
    // engine does no git I/O, so the CLI computes the verdict: `base..HEAD` is a linear-ancestor
    // range whose every commit touches ONLY milestone-record paths (any milestone's — record
    // bookkeeping is not code). Any non-record (external) commit fails the check → the engine
    // keeps the base-mismatch block, preserving the M31 worktree-combine guarantee. Only computed
    // when the base actually trails HEAD.
    let record_only_advance =
        base.sha != head && record_only_range(&jigc_home, &schemas, &base.sha, &head)?;

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
            &crate::task::git_staged_snapshot(&jigc_home)?,
            None,
            // No owner-artifact exemption at the milestone boundary — it applies only at
            // `CarryoverBoundary::Task` (M45 Inc 8; the milestone owner-artifact exemption
            // is a separate, deferred concern).
            &[],
            engine::finalize::CarryoverBoundary::Milestone,
            &jigc_home,
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
        let chain = subtask_patches_and_messages(&jigc_home, milestone_id, &worktrees, &schemas)?;
        let chain_subtask_ids = chain.ids;
        // The boundary's `.jigc/.gitignore` report, printed beside the landed manifest
        // below (M51 Increment 4 / T2) — this arm reaches the same shared writer the
        // per-task door does.
        let mut ignore_ack = None;
        // Any path the shared transaction's worktree rollback could not put back (M51
        // Increment 4 / T3) — this boundary reaches the same two rewrites the per-task door
        // does, through the same executor.
        let mut rollback_conflicts: Vec<Finding> = Vec::new();
        // What phase 7 moved out of the MILESTONE's own working area before tearing it down
        // (M53 Increment 1 / T4; `completions/artifacts/M53/settle-record.md` → D1.1). The
        // executor's `cleanup_dir` here is `dir`, the milestone area — so the kind travels
        // with the id and this boundary answers for the bytes it is about to remove. Empty on
        // every arm that never reached the teardown, because nothing was removed there either.
        let mut area_displaced: Vec<render::Displaced> = Vec::new();
        // One `finalize.foreign-bytes` advisory per area this boundary settles and cannot tear
        // down — its own, and each sub-task's (M53 Increment 2 / T3). Narrated on stderr
        // below: this door's landed arm is pinned at `Object(&["committed"])`, and the
        // additive-key window closed at M48.
        let mut kept_areas: Vec<Finding> = Vec::new();
        match crate::task::try_execute_finalize_plan(
            &jigc_home,
            &jigc_root,
            &dir,
            &plan,
            &dir,
            &schemas,
            None,
            crate::task::StagePolicy::ChainPerSubtask {
                subtasks: chain.patches,
                record: record_pathspec,
            },
            &mut ignore_ack,
            &mut rollback_conflicts,
            Some(crate::task::AreaTeardown {
                kind: engine::state::WorkArea::Milestone,
                unit_id: milestone_id,
                moved: &mut area_displaced,
                kept: &mut kept_areas,
            }),
        )? {
            Ok(hook_output) => {
                // The boundary landed — the flipped record rode the aggregate commit; disarm
                // its restore guard so the committed `joined` bytes are not reverted.
                disarm_record_flip(&mut record_flip);
                // Clean up the per-sub-task working areas (the executor only removed the
                // milestone area), **keeping** whatever jigc did not write in them — moved
                // aside and collected here so the manifest below can carry the pairs
                // (M52 Increment 4 / T4; `settle-record.md` → §18). Runs before the summary
                // for exactly that reason; the commit is already truth, so a summary that
                // then fails to render leaves no byte behind it. On a failed/rolled-back
                // finalize (below) this is never reached and the areas survive for retry.
                //
                // Seeded with what the executor already moved out of the MILESTONE area, so
                // the key below is the **union** over every area this boundary settled — its
                // own and each sub-task's (M53 Increment 1 / T4).
                let mut displaced = area_displaced;
                cleanup_subtask_areas(
                    &jigc_root,
                    &list,
                    SubtaskComplement::Displace {
                        jigc_home: &jigc_home,
                        moved: &mut displaced,
                        kept: &mut kept_areas,
                    },
                );
                // The key declares *sorted by `from`*, and with two kinds of area in the
                // union that is no longer true by construction: `.jigc/milestones/` sorting
                // before `.jigc/tasks/` is an accident of two directory names, not a rule
                // anything holds. Sorted here, once, where the union is complete (M53
                // Increment 1 / T4).
                displaced.sort_by(|a, b| a.from.cmp(&b.from));
                // C2 — the landing manifest, on the mold of the per-task landed summary:
                // `finalized <sha> — <subject>` + the whole boundary's landed-file set
                // (`git diff <pre-boundary-HEAD>..HEAD`, so the N+1 chain reads as one
                // set) + the per-sub-task contribution line.
                let landed = milestone_landed_summary(
                    &jigc_home,
                    &head,
                    &plan,
                    contributions,
                    &hook_output,
                    // The chain minted one commit per code-carrying sub-task, in this
                    // order — so each one's own sha is attributable (M50 Inc 11 / N12).
                    &chain_subtask_ids,
                    displaced,
                )?;
                print!("{}", render::milestone_finalized(format, &landed));
                if format != Format::Json {
                    println!();
                }
                // The amend this boundary made to the user's co-owned ignore file.
                crate::gitignore::emit_ack(format, &ignore_ack);
                // Relay the whole chain's folded non-blocking hook output — every fan-out
                // commit (the N per-sub-task code commits + the aggregate) runs the user's
                // hooks and `chain_commit` folds their captured streams into this one string,
                // the same string the envelope above carries (one capture, two channels;
                // `design/finalize.md` → 6. Commit; the hook_output producer axis).
                crate::task::relay_hook_output(format, &hook_output);
                // Every area this boundary settled and could not tear down, named once each
                // (M53 Increment 2 / T3).
                narrate_kept_areas(&kept_areas);
                // Tear down the fan-out worktrees the provision verb laid down (the heavier
                // A2 teardown — a non-blocking warning on a leaked worktree, never a block).
                remove_worktrees(&jigc_home, &list);
                Ok(Outcome::with_findings(0, &kept_areas))
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
                // The record flip's own restore is the guard's destructor, which can return
                // nothing — so force it here, before the rejection is composed, and carry its
                // conflicts beside the executor's (M52 Increment 5 / T5).
                rollback_conflicts.extend(drain_record_flip(&mut record_flip, &record_flip_sink));
                // A rejected chain names ITSELF in the invocation log — the `squash: false`
                // arm's own identity, not the task door's (M47 Inc 3 T7) — and states what
                // the abort above leaves behind, while git's stderr stays verbatim.
                Ok(crate::task::surface_commit_rejection(
                    format,
                    &err,
                    &crate::task::RejectionFrame {
                        code: crate::invocation_log::ERROR_MILESTONE_CHAIN_REJECTED,
                        target: format!("milestone:{milestone_id}"),
                        survived: format!(
                            "milestone:{milestone_id} is intact — nothing was committed, \
                             HEAD is at its pre-finalize commit, and every provisioned \
                             sub-task worktree still holds its staged code"
                        ),
                        survived_non_hook: None,
                        rerun: milestone_finalize_rerun(milestone_id, carry_staged),
                    },
                    &rollback_conflicts,
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
        // The boundary's `.jigc/.gitignore` report (M51 Increment 4 / T2), as above.
        let mut ignore_ack = None;
        // Any path the shared transaction's worktree rollback could not put back (M51
        // Increment 4 / T3) — this boundary reaches the same two rewrites the per-task door
        // does, through the same executor.
        let mut rollback_conflicts: Vec<Finding> = Vec::new();
        // The milestone area's own complement, as on the chain arm above — one executor, one
        // `cleanup_dir`, one subject (M53 Increment 1 / T4).
        let mut area_displaced: Vec<render::Displaced> = Vec::new();
        // As on the chain arm — one advisory per area this boundary could not tear down,
        // narrated on stderr (M53 Increment 2 / T3).
        let mut kept_areas: Vec<Finding> = Vec::new();
        match crate::task::try_execute_finalize_plan(
            &jigc_home,
            &jigc_root,
            &dir,
            &plan,
            &dir,
            &schemas,
            None,
            crate::task::StagePolicy::Combine(worktrees, record_pathspec),
            &mut ignore_ack,
            &mut rollback_conflicts,
            Some(crate::task::AreaTeardown {
                kind: engine::state::WorkArea::Milestone,
                unit_id: milestone_id,
                moved: &mut area_displaced,
                kept: &mut kept_areas,
            }),
        )? {
            // The boundary landed. Clean up the per-sub-task working areas too (the
            // executor only removed the milestone area). A failed/rolled-back finalize
            // exits non-zero and leaves the areas intact for retry.
            Ok(hook_output) => {
                // The flipped record rode the single combine commit — disarm its restore
                // guard so the committed `joined` bytes are not reverted.
                disarm_record_flip(&mut record_flip);
                // The per-sub-task areas, cleaned as on the chain arm and with the same
                // disposition over their complement: keep it (M52 Increment 4 / T4). Before
                // the summary, which carries the pairs it collects — seeded with the
                // milestone area's own moves, as on the chain arm (M53 Increment 1 / T4).
                let mut displaced = area_displaced;
                cleanup_subtask_areas(
                    &jigc_root,
                    &list,
                    SubtaskComplement::Displace {
                        jigc_home: &jigc_home,
                        moved: &mut displaced,
                        kept: &mut kept_areas,
                    },
                );
                // The key declares *sorted by `from`*, and with two kinds of area in the
                // union that is no longer true by construction: `.jigc/milestones/` sorting
                // before `.jigc/tasks/` is an accident of two directory names, not a rule
                // anything holds. Sorted here, once, where the union is complete (M53
                // Increment 1 / T4).
                displaced.sort_by(|a, b| a.from.cmp(&b.from));
                // C2 — the landing manifest (the per-task landed-summary mold): the
                // highest-stakes commit boundary must not succeed with empty stdout.
                let landed = milestone_landed_summary(
                    &jigc_home,
                    &head,
                    &plan,
                    contributions,
                    &hook_output,
                    // `squash: true` folds every sub-task into the one aggregate, so no
                    // sub-task owns a commit of its own to name.
                    &[],
                    displaced,
                )?;
                print!("{}", render::milestone_finalized(format, &landed));
                if format != Format::Json {
                    println!();
                }
                // The amend this boundary made to the user's co-owned ignore file.
                crate::gitignore::emit_ack(format, &ignore_ack);
                // T3 — relay the landed combine commit's non-blocking hook output (the
                // dedicated-worktree commit runs the user's hooks — M31 Inc 5).
                crate::task::relay_hook_output(format, &hook_output);
                // Every area this boundary settled and could not tear down, named once each
                // (M53 Increment 2 / T3).
                narrate_kept_areas(&kept_areas);
                // Tear down the fan-out worktrees on the landed default-path commit too
                // (the heavier A2 teardown — a non-blocking warning on a leaked worktree).
                remove_worktrees(&jigc_home, &list);
                Ok(Outcome::with_findings(0, &kept_areas))
            }
            // The commit-phase rejection: git's stderr stays verbatim-raw and the run names
            // itself in the invocation log — with the `squash: true` combine's OWN identity
            // since M47 Inc 3 T7 (it borrowed the task door's before). The failure arm is
            // inlined here so the landed arm can print the manifest. The executor already
            // rolled back its promoted-doc copies and the record flip's guard restores the
            // record, so the milestone is left exactly as the boundary found it.
            Err(err) => {
                // The record flip's restore is the guard's destructor, which can return
                // nothing — so force it here, before the rejection is composed, and carry its
                // conflicts beside the executor's (M52 Increment 5 / T5).
                rollback_conflicts.extend(drain_record_flip(&mut record_flip, &record_flip_sink));
                Ok(crate::task::surface_commit_rejection(
                    format,
                    &err,
                    &crate::task::RejectionFrame {
                        code: crate::invocation_log::ERROR_MILESTONE_FINALIZE_REJECTED,
                        target: format!("milestone:{milestone_id}"),
                        survived: format!(
                            "milestone:{milestone_id} is intact — nothing was committed, the \
                             merged docs were rolled back, and every provisioned sub-task \
                             worktree still holds its staged code"
                        ),
                        survived_non_hook: None,
                        rerun: milestone_finalize_rerun(milestone_id, carry_staged),
                    },
                    &rollback_conflicts,
                ))
            }
        }
    }
}

/// A transactional guard over the milestone record's `joined` flip that the finalize commit
/// folds in (`design/team-ready-state.md` → The commit model — join folds). Holds the record's
/// pre-flip bytes; on `Drop` (any early return / blocked / failed finalize) it puts them back, so
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
///
/// **The worktree half is compare-and-swap since M52 Increment 5 / T5**, on the discipline
/// `cli::rollback::ROLLBACK_POPULATIONS`' `fan-out-record-flip` row declares. The interval
/// this guard writes across is the widest one in the binary — promotion, retirement, the
/// live-index stage, the combine and the user's own `pre-commit` hook all run inside it — so
/// an unconditional rewrite of the captured pre-image took a third party's bytes at exit 1
/// with nothing saying so, the sixth write of the file whose other five doors T4 closed.
/// Now the restore asks the shared question (*is what is there now still what jigc left?*)
/// and a `No` preserves both copies and raises [`crate::rollback::MILESTONE_DOOR`]'s
/// `milestone.rollback-conflict` — the same identity the record's own doors raise, because
/// the subject is the same file and the operator's act is the same comparison.
struct RecordFlipGuard<'a> {
    /// The worktree axis: the record's pre-flip image plus the door a raced restore names.
    /// Keyed by [`RecordFlipGuard::pathspec`], which is also the park identity.
    worktree: crate::rollback::PreImageFamily,
    /// Whether the flip actually changed bytes — folded into the `has_diff` signal.
    changed: bool,
    /// The repo-relative pathspec the finalize commit path-adds.
    pathspec: String,
    /// The two roots the compare-and-swap's restore and park need, owned because the restore
    /// runs in a destructor that borrows nothing from its caller's frame.
    repo_root: PathBuf,
    jigc_root: PathBuf,
    /// Cleared by [`disarm`](RecordFlipGuard::disarm) once the commit lands.
    armed: bool,
    /// **The caller-owned sink.** A destructor returns nothing, so the conflicts a raced
    /// restore raises are pushed here and the boundary's refusal arm drains them
    /// ([`drain_record_flip`]) into the document it is composing, beside its own frame.
    sink: &'a std::cell::RefCell<Vec<Finding>>,
}

impl RecordFlipGuard<'_> {
    fn disarm(&mut self) {
        self.armed = false;
    }

    /// The flip's restore: the shared compare-and-swap, returning one
    /// `milestone.rollback-conflict` per path whose bytes are no longer jigc's (at most one —
    /// this family holds exactly one entry).
    ///
    /// A named unit rather than the destructor's body, because the registry's membership
    /// scan keys a population at `(file, unit)` and this is the row's site
    /// (`crates/cli/tests/rollback_population_registry.rs`).
    fn rollback_record_flip(&self) -> Vec<Finding> {
        self.worktree.restore(&self.repo_root, &self.jigc_root)
    }
}

impl Drop for RecordFlipGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            // Best-effort restore — the finalize did not land, so revert the working-tree
            // flip, but only while the record still holds the bytes the flip wrote. The
            // restore runs before the sink is borrowed, never inside the borrow.
            let conflicts = self.rollback_record_flip();
            self.sink.borrow_mut().extend(conflicts);
        }
    }
}

/// Disarm the (optional) record-flip guard on a landed finalize — a `None` (dev-only, no record)
/// is inert.
fn disarm_record_flip(guard: &mut Option<RecordFlipGuard<'_>>) {
    if let Some(guard) = guard {
        guard.disarm();
    }
}

/// Run the flip's restore **now** and take the conflicts it raised — the refusal arm's half of
/// the caller-owned sink.
///
/// The restore is the destructor's, so forcing the drop here is what makes its findings
/// available to the arm that is composing the refusal; afterwards the guard is `None` and the
/// end-of-scope drop is inert. Called on both `squash` arms' `Err` paths, before the rejection
/// is surfaced.
///
/// **Declared bound.** A failure *between* the flip and the boundary — a pre-commit block
/// (zero-contribution, carryover) or a propagated `?` — drops the guard without a drain, so the
/// restore still runs under the same compare-and-swap (no byte is overwritten and the pre-image
/// is parked) but the conflict is not carried onto that arm's document. Those arms commit
/// nothing and run no hook, so the racer there is an unrelated concurrent process rather than
/// the transaction's own interval.
fn drain_record_flip(
    guard: &mut Option<RecordFlipGuard<'_>>,
    sink: &std::cell::RefCell<Vec<Finding>>,
) -> Vec<Finding> {
    drop(guard.take());
    std::mem::take(&mut sink.borrow_mut())
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
/// pathspec the finalize commit path-adds. **Both the record home and the pathspec base are
/// `jigc_home`** (M53 — the cwd census, C2-06): they were split, the home under `jigc_home`'s
/// docs-root and the strip against the walk-up `repo_root`, on the stated ground that the two
/// coincide outside a worktree. Inside one they do not, `strip_prefix` fell through to its
/// `unwrap_or`, and the pathspec the boundary then `git add`ed was an **absolute host path**
/// against an index that had never heard of it.
///
/// The pre-flip bytes are carried as a [`crate::rollback::PreImage`] captured **before**
/// `join_record` writes and read back **one statement after** it — at rollback time the
/// read-back would return a racer's bytes and call them jigc's, which is the whole of the
/// compare-and-swap. `sink` is where the guard's destructor puts the conflicts a raced restore
/// raises ([`drain_record_flip`]).
fn flip_record_for_finalize<'a>(
    jigc_root: &Path,
    jigc_home: &Path,
    schemas: &BTreeMap<String, Schema>,
    milestone_id: &str,
    sink: &'a std::cell::RefCell<Vec<Finding>>,
) -> Result<Option<RecordFlipGuard<'a>>> {
    let Some(schema) = schemas.get(MILESTONE_RECORD_TYPE) else {
        return Ok(None);
    };
    // Reconcile preflight (T6): the status-flip is a `set: on-transition` overwrite, so
    // conflict-block if the committed record drifted out-of-band before we flip it — leaving
    // the record untouched (`design/team-ready-state.md` → F3). The `.jigc/` baseline home is
    // `jigc_home/.jigc`, which is also what every git call this guard makes now acts in.
    reconcile_record_preflight(jigc_home, &jigc_home.join(".jigc"), schema, milestone_id)?;

    let record_path = engine::store::canonical_path(jigc_home, schema, milestone_id)
        .context("the `milestone-record` doctype declares no committed location")?;
    let before = std::fs::read_to_string(&record_path)
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;
    let pathspec = record_path
        .strip_prefix(jigc_home)
        .unwrap_or(&record_path)
        .to_str()
        .with_context(|| format!("record path {record_path:?} is not valid UTF-8"))?
        .to_owned();
    // The rollback's own pre-image, captured before the flip writes. It re-reads rather than
    // reusing `before` because the entry owns its bytes (`before` is the `has_diff`
    // comparison below and nothing else), and this population's pre-image is always
    // **present** — the read above would have failed otherwise, so the absent arm the shared
    // entry models belongs to `milestone create`'s population and not to this one.
    let mut entry = crate::rollback::PreImage::capture(pathspec.clone(), record_path.clone())
        .with_context(|| format!("could not read the milestone record {record_path:?}"))?;
    let joined = engine::milestone::join_record(&record_path, schema, milestone_id)
        .map_err(finding_to_err)?;
    // One statement after the write, which is what makes the recorded image jigc's own.
    entry.wrote();
    let mut worktree = crate::rollback::PreImageFamily::empty(crate::rollback::MILESTONE_DOOR);
    worktree.push(entry);
    Ok(Some(RecordFlipGuard {
        changed: joined != before,
        worktree,
        pathspec,
        repo_root: jigc_home.to_path_buf(),
        jigc_root: jigc_root.to_path_buf(),
        armed: true,
        sink,
    }))
}

/// Remove each sub-task's working area (`.jigc/tasks/<sub-id>/`) once the milestone
/// **settles** — the gitignored runtime state the executor's milestone-area cleanup leaves
/// behind (it removes only the milestone area). Mirrors the single-task post-commit cleanup
/// discipline ([`crate::task::post_commit`]): best-effort, logged-not-raised — a cleanup
/// failure must not fail a commit that already landed (the "self-heal" stance). Enumerates
/// the milestone's id-sorted sub-task list, so the set cleaned is exactly its sub-tasks.
///
/// **Three call sites, on two kinds of path** — the doc-comment said *"called ONLY on the
/// success path"* until M46 Inc 2's validation caught the third falsifying it. The two
/// **landed** finalize arms (`squash: true` and the N+1 chain) run it after the commit, so
/// the areas' `docs/*.md` are promoted or rendered into the message and git holds them; no
/// failed or rolled-back finalize reaches it, which is what leaves the areas intact for a
/// retry (respecting the F1 rollback path). [`run_discard`] runs it on the **abandon** path,
/// where nothing landed and the area is the only copy of the sub-agent's authored prose — so
/// that caller, and only that caller, names those bytes first
/// ([`crate::setup::pending_staged_prose`]).
///
/// **What happens to each area's *complement* is the caller's**, which is why it arrives as
/// [`SubtaskComplement`] rather than being decided here (M52 Increment 4 / T4;
/// `settle-record.md` → §18): the two landed arms **keep** those bytes, and the abandon path
/// takes them under its own door's consent and narration. A disposition read off the
/// function instead of the call would silently re-decide `jigc milestone discard`.
///
/// **Returns whether every area it reached is gone** — a caller that acks the teardown keys
/// that ack on this, never on having run the loop (M52 Increment 4 / T7, D-2). The landed
/// finalize arms ignore it: their ack names the commit, not the workbench, and since M53
/// Increment 2 / T3 an area they left standing is named by its own `finalize.foreign-bytes`
/// advisory instead — so *not gone* is reported per area rather than as one boolean.
fn cleanup_subtask_areas(
    jigc_root: &Path,
    list: &engine::milestone::TaskList,
    mut complement: SubtaskComplement<'_>,
) -> bool {
    let tasks_root = jigc_root.join("tasks");
    let mut all_gone = true;
    for sub_id in list.enumerate() {
        let area = tasks_root.join(&sub_id);
        if !area.exists() {
            continue;
        }
        match &mut complement {
            SubtaskComplement::Displace {
                jigc_home,
                moved,
                kept,
            } => {
                // Per **area**, so the narration's subject is one working area — the sentence
                // [`crate::task::narrate_displacement`] writes — and the boundary's envelope
                // gets the union. The order the key declares is **not** this loop's to hold:
                // since M53 Increment 1 / T4 the union also carries the milestone area's own
                // moves, so each caller sorts the completed union rather than resting on an
                // ordering two directory names happen to give it.
                let outcome = crate::task::displace_foreign_area(
                    jigc_root,
                    &area,
                    engine::state::WorkArea::Task,
                    &sub_id,
                );
                crate::task::narrate_displacement(&outcome);
                // **The removal is conditioned on the move** (M53 Increment 2 / T3): what the
                // displacement above could not park stays where it is, because this arm's
                // removal is the registry-keyed unwind and not `remove_dir_all`. An area left
                // standing hands back one `finalize.foreign-bytes` advisory for the caller's
                // own surface.
                if !crate::task::unwind_settled_area(
                    jigc_home,
                    &area,
                    engine::state::WorkArea::Task,
                    &sub_id,
                    &outcome,
                    kept,
                ) {
                    all_gone = false;
                }
                // The union is the envelope's `displaced` key — the moves that landed. What
                // this area would not give up is on the narration above, per area, where the
                // count is that area's complement and not its move (M53 Increment 2 / T2).
                moved.extend(outcome.moved);
            }
            // **`Take` keeps `remove_dir_all`, and the disposition is read off the CALL.**
            // This function's own rule, quoted where the branch is: *"A disposition read off
            // the function instead of the call would silently re-decide `jigc milestone
            // discard`."* That door's `--force` **is** the consent for these bytes, and its
            // guard and narration have already run at the door; unwinding here instead would
            // leave the operator's own abandoned workbench standing after they consented to
            // its removal (`settle-record.md` → §3).
            SubtaskComplement::Take => {
                if let Err(err) = std::fs::remove_dir_all(&area) {
                    eprintln!(
                        "note: post-commit sub-task working-area removal for `{sub_id}` failed (self-heals): {err:#}"
                    );
                    all_gone = false;
                }
            }
        }
    }
    all_gone
}

/// What a [`cleanup_subtask_areas`] call does with each sub-task area's **complement** —
/// the bytes jigc did not write there (`engine::state::foreign_area_paths`), and, since M53
/// Increment 2 / T3, **which removal follows**. It differs by *door* (`settle-record.md` →
/// §18, §3).
///
/// The doc-comment read *"the removal itself is the same at every call; only this differs"*
/// until T3, and that is now false in the one way that matters: `Displace` removes what jigc
/// wrote and leaves the rest standing, `Take` still removes the directory whole. The two are
/// not a detail of the disposition — they **are** it.
enum SubtaskComplement<'a> {
    /// **Keep them** — move each entry to `.jigc/displaced/<sub-task-id>/<relative>` before
    /// the removal, narrate it, and collect the pairs for the landed envelope's
    /// `committed.displaced`. The two landed `jigc milestone finalize` arms, for the reason
    /// `jigc task finalize` has one door over: the boundary commit carries the promoted
    /// docs, the merged record and the sub-agents' staged code and takes **nothing** out of
    /// a working area, so the landed-boundary warrant that lets a sibling door merely
    /// narrate a loss is unavailable, and this door has no consent flag to offer instead.
    Displace {
        /// The root every path this arm prints is spelled against — the **main checkout**
        /// (`crate::task::workbench_home`), never the standing one. Named for what it is
        /// since M53 (row 6): it was spelled `repo_root`, both callers already passed
        /// `jigc_home`, and the sibling door that read the same field name literally printed
        /// host-absolute paths from a linked worktree.
        jigc_home: &'a Path,
        moved: &'a mut Vec<render::Displaced>,
        /// One `finalize.foreign-bytes` advisory per sub-task area the teardown left
        /// standing (M53 Increment 2 / T3). The caller decides where it goes: this door's
        /// landed arm is pinned at `Object(&["committed"])`, so it narrates them on stderr
        /// rather than growing a `findings` key.
        kept: &'a mut Vec<Finding>,
    },
    /// **Take them with the area.** [`run_discard`]'s abandon path, whose guard and
    /// narration live at that door (the staged-prose refusal + `--force`), never here.
    Take,
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
///
/// **Returns whether every registered worktree it reached is gone** — the warnings say *what*
/// is left, and this is what stops a caller's ack from claiming otherwise (M52 Increment 4 /
/// T7, D-2).
fn remove_worktrees(jigc_home: &Path, list: &engine::milestone::TaskList) -> bool {
    let mut all_gone = true;
    // **This function's two warnings name the host path, deliberately** (M50 Increment 12 /
    // T1). The second carries a `git worktree remove --force <path>` the operator pastes, and
    // git resolves a worktree path against the CALLER's cwd — so a repo-relative remedy works
    // only from the repo root, which nothing here can promise. The first warning then names
    // the same path the same way, because one screen spelling one path two ways is the law-1
    // break this pass closes, not a fix for it. Disposed as `DeclaredAbsolute` in
    // `crates/cli/tests/repo_relative_paths.rs`; the loss narration these warnings sit beside
    // goes through [`narrate_removal`], which is repo-relative like every other surface.
    //
    // **Both spans in that remedy now also name the checkout** (M53 — the cwd census, C1-08).
    // The census predicted this site shipped a repo-relative operand; driven, it did not —
    // `path` is `canonical_home.join(…)` and the operand has always been absolute. What was
    // cwd-fragile is its neighbour: `git worktree prune` is a repository operation, so pasted
    // from outside the repository it exits 128 while the `remove` beside it would have
    // worked. Both go through `engine::finding::git_at`, which is also what keeps this
    // non-`Finding` surface inside the one rule the route fence enforces everywhere else.
    let registered = registered_worktrees(jigc_home).unwrap_or_default();
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
            all_gone = false;
            continue;
        };
        // Read the loss BEFORE the removal and name what it actually TOOK after it (law 1
        // — nothing lies: a boundary that exits 0 must not have silently destroyed work,
        // and must not claim a destruction that did not happen either), through the
        // emitter every destroying door shares.
        let pending = pending_loss(jigc_home, &path);
        let removed = git_worktree(jigc_home, &["worktree", "remove", "--force", path_str]);
        pending.narrate_taken(jigc_home);
        if let Err(err) = removed {
            all_gone = false;
            // A2 — pinned non-blocking warning, naming the leaked path + the prune remedy.
            eprintln!(
                "warning: could not remove the fan-out worktree {path_str}: {err:#}\n  \
                 remedy: run `{prune}`, then `{remove}`",
                prune = engine::finding::git_at(jigc_home, "worktree prune"),
                remove = engine::finding::git_at(
                    jigc_home,
                    &format!(
                        "worktree remove --force {}",
                        crate::task::shell_token(path_str)
                    ),
                )
            );
        }
    }
    // Drop admin records for any worktree dir removed out-of-band (the provision prune
    // inverse) — best-effort; a prune failure is itself non-fatal to a landed commit.
    let _ = git_worktree(jigc_home, &["worktree", "prune"]);
    all_gone
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
            // The **house** findings line, whole — head, locus, route, in that order
            // ([`crate::render::finding_line`], M50 Increment 10 / T1). This loop used to
            // re-derive two of those three and drop the head, so a blocked milestone
            // finalize named the break and not its code. `eprint!`, not `eprintln!`: the
            // house line already ends in `\n`.
            eprint!("{}", crate::render::finding_line(finding, false));
        }
    }
    Ok(Outcome::with_findings(
        crate::task::EXIT_VALIDATION_BLOCKED,
        &report.findings,
    ))
}

/// Name on **stderr** every working area a landed boundary could not tear down — one
/// [`crate::render::finding_line`] each, the house renderer, head and locus and route in that
/// order (M53 Increment 2 / T3; `completions/artifacts/M53/settle-record.md` → D2.5).
///
/// **Stderr rather than the envelope, and that is a decision with a bound.** The sibling door
/// (`jigc task finalize`) puts the identical advisory on its landed `findings` array, because
/// that arm's pinned shape already carries one. This door's landed arm is pinned at
/// `Object(&["committed"])` (`crate::render::ENVELOPE_ARMS`) and the pre-1.0 additive-key
/// window closed at M48, so growing a `findings` key here is a 2.0 act. Until then the fact
/// reaches a driver through the side channel and the invocation log — the door records the
/// same findings via `Outcome::with_findings`, which moves no stdout byte — and its safety
/// does not rest on being heard: an area left standing is inert at every door.
///
/// Silent when nothing was kept: the omitting context prints no bytes at all.
fn narrate_kept_areas(kept: &[Finding]) {
    for finding in kept {
        // `eprint!`, not `eprintln!`: the house line already ends in `\n`.
        eprint!("{}", crate::render::finding_line(finding, false));
    }
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
    squash_of(&resolved)
}

/// Read `finalize.fan-out.squash` off a resolved cascade — the **one reader** the join
/// ([`resolve_squash`]) and the sub-task compose share (M55 Increment 3, P4), so the
/// compose that decides whether a sub-task is asked for its commit doc and the boundary
/// that decides whether to read it cannot answer from different layers.
pub(crate) fn squash_of(resolved: &engine::cascade::Resolved) -> Result<bool> {
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
/// the plan + the `squash` branch, so nothing durable is written yet and a block truly
/// commits nothing.
///
/// **Its subject is every doc this boundary commits** — the persisted merged docs, plus the
/// transient `commit:<sub-id>` docs the `squash: false` chain renders into commit messages.
/// The knob is therefore an *input* (`squash`) rather than something the gate is independent
/// of: which docs the boundary commits is exactly what the knob decides. The filter below
/// carries the derivation.
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
    sources: &BTreeMap<String, String>,
    squash: bool,
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

    // The gate's subject is **everything this boundary commits**, in the two shapes the merged
    // docs dir holds it:
    //
    //   * a **persisted** doc — one whose doctype declares a committed home (`location:` or
    //     `placement:`) — which the promote plan below lands as a repo file; and
    //   * a **rendered transient** doc — a sub-task's `commit:<sub-id>` skeleton, which
    //     declares no home but, under `squash: false`, IS the bytes of that sub-task's commit
    //     message, exactly as a task's commit doc is at `jigc task finalize`.
    //
    // The transient half was missing, and `design/validation.md` → the door list stated the
    // opposite ("that door gates as hard as the two task doors"): an author-required leaf left
    // unfilled — `type` and `summary` are the whole set — blocked `jigc task validate <sub-id>`
    // at exit 3 and landed a `: <subject>` / `feat:` commit through this door at exit 0.
    //
    // **The modes genuinely differ, and the reason is what the boundary reads.** Under
    // `squash: true` the aggregate message is CLI-synthesized (`synthesized_message` — "a
    // milestone has no commit doc to render") and NO commit doc is read at all, so a sub-task's
    // is not part of the committed state and gating it would refuse a milestone whose output is
    // entirely correct. Under `squash: false` only the **code-carrying** sub-tasks get a
    // commit, so only their commit docs are read — the subject is therefore keyed on the very
    // set the render uses ([`code_carrying_worktrees`]), never on a parallel derivation.
    //
    // Copy that subject into a fresh scratch staging area and gate over it; the promote plan
    // below still reads the full `staging_dir`.
    let rendered: std::collections::BTreeSet<String> = if squash {
        std::collections::BTreeSet::new()
    } else {
        code_carrying_worktrees(worktrees)?
            .into_iter()
            .map(|(id, _)| format!("{COMMIT_TYPE}:{id}"))
            .collect()
    };
    let gate_staging = crate::task::ScratchTree::new();
    let gate_docs = gate_staging.path().join("docs");
    std::fs::create_dir_all(&gate_docs).with_context(|| {
        format!(
            "could not open the gate staging area {}",
            crate::render::repo_relative(jigc_home, &gate_docs)
        )
    })?;
    let src_docs = staging_dir.join("docs");
    for entry in std::fs::read_dir(&src_docs).with_context(|| {
        format!(
            "could not read the merged docs dir {}",
            crate::render::repo_relative(jigc_home, &src_docs)
        )
    })? {
        let entry = entry?;
        let path = entry.path();
        let raw = entry.file_name();
        let name = raw.to_string_lossy();
        // **A body jigc wrote, or nothing** — [`engine::state::staged_doc_id`] on the name
        // AND the shape, which is the one membership question every other `merged/docs/`
        // seam asks: `engine::milestone::clear_staged_bodies` (the rebuild's remover),
        // `engine::state::foreign_area_paths`' milestone arm (the complement probe) and
        // `engine::state::unwind_merged` (the teardown) each ask
        // `shape.is_file() && staged_doc_id(..).is_some()`, and `engine::state`'s own rule
        // says shape is part of membership: jigc writes regular files and never a directory
        // or a link wearing a body's name, and the shape is read WITHOUT following symlinks
        // ([`std::fs::DirEntry::file_type`]).
        //
        // The name leg alone read `split(':').next()` until M53 Increment 1 / T1, which
        // reads the whole stem as the doctype when there is no `:` — so a foreign
        // `merged/docs/adr.md` resolved to the doctype `adr`, was copied into the gate
        // staging area, and blocked this boundary at exit 3 with
        // `schema-conformance.unknown-type` on a file jigc never wrote, on every re-run
        // (`settle-record.md` → §1). The shape leg was missed in the same motion, and the
        // selective clear that task landed is what made it reachable: a **directory** named
        // `adr:x.md` dead-ended the boundary at exit 1 with no finding code, no route and a
        // host-absolute path, and a **symlink** named `adr:y.md` was copied in and blocked at
        // exit 3 on a repo path that does not exist — verbatim the §1 defect, surviving one
        // axis over.
        //
        // Nothing under `merged/docs/` is walked for membership here: the gate's subject is
        // what this boundary COMMITS, and an entry jigc's own writer cannot emit is not that.
        // A skipped entry is left exactly where it is, for the complement probe and the
        // boundary's own displacement to answer for.
        let shape = entry.file_type().with_context(|| {
            format!(
                "could not read the shape of {}",
                crate::render::repo_relative(jigc_home, &path)
            )
        })?;
        if !shape.is_file() {
            continue;
        }
        let Some(address) = engine::state::staged_doc_id(&name) else {
            continue;
        };
        let Some((ty, _slug)) = address.split_once(':') else {
            continue;
        };
        // Persisted iff the doctype declares a committed home (`location:` or `placement:`)
        // — the transient-sink `commit` type declares neither.
        let persisted = schemas
            .get(ty)
            .is_some_and(|s| s.location.is_some() || s.placement.is_some());
        if persisted || rendered.contains(address) {
            std::fs::copy(&path, gate_docs.join(entry.file_name())).with_context(|| {
                format!(
                    "could not stage {} into the gate area",
                    crate::render::repo_relative(jigc_home, &path)
                )
            })?;
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
    let mut record = FileStateRecord::load(jigc_root).with_context(|| {
        format!(
            "could not load the file-state record under {}",
            crate::render::repo_relative(jigc_home, jigc_root)
        )
    })?;
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
    // The managed-vs-foreign discriminator's three pack facts (M48 Inc 4 / T1) — the merged
    // gate drives the same committed-store sweep the per-task door does, so a foreign
    // squatter must draw the store door's code and route here too.
    let versions = crate::pack::frozen_doctype_versions(pack.as_ref());
    let priors = crate::pack::prior_doctype_schemas(pack.as_ref(), &versions);
    let migratable = crate::pack::migratable_doctypes(pack.as_ref());

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
        &engine::validate::AdoptionInputs::new(&versions, &priors, &migratable, jigc_home),
        // No live-record carve-out at the join (M52 Inc 10 / T6). The carve-out names the
        // record of the work unit whose TASK is being swept, and this door sweeps no task:
        // the merged gate area belongs to none, which is why the `ConflictBlock` above cannot
        // name one either. The boundary's own milestone is not that subject, so naming its
        // record here would carve out a path on a different warrant than the one the value
        // carries.
        &engine::file_state::LiveRecord::none(),
    )
    .with_context(|| {
        format!(
            "validating the merged effective state under {}",
            crate::render::repo_relative(jigc_home, staging_dir)
        )
    })?;

    if report.has_blocking() {
        // `blocked()` re-applies the cascade (idempotent) and renders the pinned envelope —
        // exit 3. `merged_wt` + `base_tree` drop on return (the worktree/scratch teardown).
        return Ok(Some(blocked(
            jigc_home,
            format,
            scope_merged_repair_routes(report.findings.to_vec(), jigc_root, sources),
        )?));
    }
    Ok(None)
}

/// Scope each merged-state gate block's repair route to the **sub-task that contributed the
/// doc it names** (M49 Increment 8 / T3) — the third member of `crate::render::BOUNDARY_DOORS`
/// answering the same defect its two task siblings answer: a fan-out has ≥2 open sub-tasks by
/// construction, so the emitted `jigc doc set-slot <merged address> …` exited 1 on `more than
/// one active task` at the one door that *always* holds more than one.
///
/// **This is not the `<task-id>`-less case M47 inc-2 / T4 settled.** That decision governs the
/// [`engine::file_state::ConflictBlock`] a few lines above, whose subject is an external edit
/// against the merged set as a whole — genuinely no single task's. A conformance block names
/// exactly one merged doc, and [`engine::milestone::MaterializeOutcome::sources`] already
/// records which sub-task contributed it; the repair is to fix it *in that area* and re-run
/// the join, which is what the scoped route says and what re-running the door then clears.
///
/// **Declared bound — the suffixed instance.** When two areas each *create* the same slug the
/// join suffixes the loser, so the merged final address (`adr:foo-2`) is an address its own
/// contributing area does not stage. The staged-body existence check below is what keeps that
/// case honest: no body under the sub-area, no enrichment, and the route stays exactly the
/// one it was rather than becoming a confidently wrong one.
fn scope_merged_repair_routes(
    mut findings: Vec<Finding>,
    jigc_root: &Path,
    sources: &BTreeMap<String, String>,
) -> Vec<Finding> {
    for finding in &mut findings {
        let Some(head) = finding
            .location
            .as_ref()
            .and_then(|location| location.address.as_deref())
            .map(crate::doc::doc_head)
        else {
            continue;
        };
        let Some(task) = sources.get(head) else {
            continue;
        };
        let Some((ty, slug)) = head.split_once(':') else {
            continue;
        };
        if !engine::state::instance_path(&jigc_root.join("tasks").join(task), ty, slug).is_file() {
            continue;
        }
        crate::doc::scope_repair_route_to_task(finding, task);
    }
    findings
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
) -> Result<ChainSubtasks> {
    let commit_schema = schemas
        .get(COMMIT_TYPE)
        .with_context(|| format!("the embedded pack ships no `{COMMIT_TYPE}` schema"))?;
    // The sub-task working areas bind to jigc_home (the main checkout), not the per-task
    // worktree (M31 WF3); outside a worktree the two coincide.
    let tasks_root = jigc_home.join(".jigc").join("tasks");

    let coded = code_carrying_worktrees(worktrees)?;

    // Render each code-carrying sub-task's authored commit doc in id order (one engine
    // call) — the message each per-sub-task commit carries.
    let ids: Vec<String> = coded.iter().map(|(id, _)| id.clone()).collect();
    let messages =
        engine::finalize::render_subtask_messages(&ids, &tasks_root, commit_schema, milestone_id)
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

    // Pair each worktree's staged patch with its rendered message, in id order. The ids
    // ride out alongside: the chain mints one commit per member in exactly this order, so
    // the landing ack reads them back to attribute each sub-task's own sha (M50 Inc 11 /
    // N12; [`milestone_landed_summary`]).
    Ok(ChainSubtasks {
        ids,
        patches: coded
            .into_iter()
            .map(|(_, patch)| patch)
            .zip(messages)
            .collect(),
    })
}

/// The `squash: false` chain's per-sub-task inputs, id-ordered — what
/// [`subtask_patches_and_messages`] hands the boundary.
struct ChainSubtasks {
    /// The **code-carrying** sub-task ids, in the order the chain commits them. The
    /// landing ack reads them back to attribute each sub-task's own sha (M50 Inc 11 /
    /// N12; [`milestone_landed_summary`]).
    ids: Vec<String>,
    /// Each one's `(staged patch, rendered commit message)` pair, in the same order.
    patches: Vec<(Vec<u8>, String)>,
}

/// The **code-carrying** sub-tasks, id-ordered: each provisioned worktree that staged
/// something, paired with its staged patch. A worktree with **nothing staged** contributes
/// no per-sub-task commit (the retired `--allow-empty` tree-empty form is gone) and so is
/// absent; `worktrees` is already id-sorted (the provisioned set), which makes the commit
/// *sequence* a pure function of the id set (hardening #7).
///
/// **One producer, because two consumers must agree on the set.** The `squash: false` render
/// ([`subtask_patches_and_messages`]) reads it to decide which sub-tasks get a commit, and
/// the boundary gate ([`milestone_boundary_gate`]) reads it to decide whose transient
/// `commit:<sub-id>` doc is part of what the boundary commits — and therefore gets validated.
/// A gate keyed on a *different* set than the render is the hole this function exists to make
/// unstatable: it would either skip a doc that lands or block one that does not.
fn code_carrying_worktrees(worktrees: &[PathBuf]) -> Result<Vec<(String, Vec<u8>)>> {
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
    Ok(coded)
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
/// classified worktree set: its merged-doc count from the materialize's address→source map
/// (`sources`), its staged-code file count from its **live** fan-out worktree's index
/// (`git diff --cached --name-only`), and what the boundary found at its worktree path.
/// Computed **pre-commit** — a landed boundary tears the worktrees down.
///
/// A sub-task with an [`WorktreeState::Absent`] path counts 0 code files **by
/// construction**, not by measurement: since M31 Inc 4/5 the isolated worktree is the sole
/// place a sub-agent's code can live, so no worktree means no code was possible. That
/// degrade used to be indistinguishable from "the sub-agent staged nothing," which is why
/// `provisioned` rides the manifest (M47 Inc 3, call (b)(ii)).
///
/// An [`WorktreeState::Unreadable`] path is the third cell, and it is neither of those two
/// facts: something is there, and git cannot read it, so the boundary counted nothing out
/// of it and committed nothing from it. The manifest says exactly that rather than
/// borrowing the never-provisioned words (M46 Inc 2, T1).
fn subtask_contributions(
    subtasks: &[SubtaskWorktree],
    sources: &std::collections::BTreeMap<String, String>,
) -> Result<Vec<render::SubTaskContribution>> {
    let mut out = Vec::new();
    for sub in subtasks {
        let docs = sources.values().filter(|source| **source == sub.id).count();
        let live = sub.state == WorktreeState::Live;
        let code_files = if live {
            worktree_staged_file_count(&sub.path)?
        } else {
            0
        };
        // The loss half of the same pre-commit snapshot (M47 Inc 3, call (c)) — the
        // teardown removes the whole checkout, so everything the worktree holds beyond
        // the staged set dies with it. Read here, where the worktrees are still alive
        // and `code_files` is read, so the manifest's "landed" and "lost" halves come
        // from one observation of one state.
        //
        // **Gated on `registered`, not on `live`**: [`remove_worktrees`] removes registered
        // worktrees and skips everything else, so narrating a live-but-unregistered
        // worktree's content as *lost* — when the teardown leaves it standing — would be a
        // law-1 lie in the other direction (`design/surface-contract.md`).
        let discarded = if live && sub.registered {
            discarded_work(&sub.path)?
        } else {
            Vec::new()
        };
        out.push(render::SubTaskContribution {
            id: sub.id.clone(),
            docs,
            code_files,
            provisioned: live,
            worktree_unreadable: sub.state == WorktreeState::Unreadable,
            discarded,
            // Filled in AFTER the boundary lands, by [`milestone_landed_summary`] — the
            // sha does not exist yet at this pre-commit snapshot (M50 Inc 11 / N12).
            hash: None,
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
/// exit 0. The probe is `git status --porcelain` over the staged, unstaged, untracked
/// **and ignored** sets — **partitioned on the index column** so the narration stays true:
///
/// * index column set, worktree column clean (`A `, `M `, `R `) — **wholly staged**: every
///   byte is in the patch the boundary commits, so it is *not* reported. Reporting it
///   would be an over-report, which is what makes a loss warning untrustworthy.
/// * index column clear (`??`, ` M`, ` D`) — **never staged**: the commit carries none of
///   it.
/// * both columns set (`MM`, `AM`, an unmerged `UU`) — **partly staged**: the commit
///   carries the indexed version and the worktree's later edit dies.
/// * `!!` — **ignored**: no commit could ever carry it, and `git worktree remove --force`
///   deletes it exactly as hard as the rest (M46 Inc 2, `razor-ledger.md` §2f — driven at
///   exit 0 over a gitignored `secrets.env`). Falling through the two arms above would
///   have labelled it *"staged only in part"* over a file that was never staged.
///
/// **This is the narration probe, not the refusal probe.** Its [`dirty_worktrees`] sibling
/// deliberately stays `--porcelain` **without** `--ignored`: a provisioned worktree arrives
/// tracked-only while the sub-task walk tells the agent to build and test, so refusing on
/// the ignored axis would fire on the ordinary fan-out **success** path and train `--force`
/// into reflex. Declared cost, stated rather than implied: the loss is **visible, not
/// prevented**.
///
/// **`--ignored=matching`, not plain `--ignored`.** With `--untracked-files=all` the
/// traditional mode enumerates every file inside an ignored directory (measured: a
/// 43-entry `target/` inventory), which is the same over-report the wholly-staged
/// exclusion above exists to avoid. The matching mode names the level the ignore rule
/// matched — `target/`, `sub/node_modules/`, `secrets.env`. **Declared bound:** a precious
/// file nested inside an ignored directory is covered by its container's name, not listed
/// individually.
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
/// enumerates the files (the ignored set is the one deliberate exception, above).
fn discarded_work(worktree: &Path) -> Result<Vec<render::DiscardedWork>> {
    const PROBE: [&str; 5] = [
        "status",
        "--porcelain",
        "-z",
        "--untracked-files=all",
        "--ignored=matching",
    ];
    let out = Command::new("git")
        .args(PROBE)
        .current_dir(worktree)
        .output()
        .context("could not run `git status` (is git on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` in worktree {worktree:?} failed: {}",
            PROBE.join(" "),
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
        let state = if index == b'!' {
            // Ignored — no commit could carry it, and the teardown deletes it anyway.
            render::DiscardState::Ignored
        } else if index == b' ' || index == b'?' {
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

/// What a destroying door is about to destroy at a worktree-shaped path, in the shape the
/// narration prints — the **narration** counterpart of [`probe_leftover`], which is the
/// **refusal** one. Both dispatch on the same [`classify_leftover`] verdict, so neither
/// surface claims more about a path than git can say about it.
struct Doomed {
    /// How the path can honestly be named: git reads it as a worktree of its own, or it
    /// cannot read it at all. Calling a directory nothing vouches for a *worktree* is the
    /// law-1 lie the refusals already avoid ([`dirty_worktree_finding`]), and the narration
    /// is not exempt from it.
    subject: &'static str,
    /// One line per doomed item — the `git status` path plus its [`render::DiscardState`]
    /// label where git can read the worktree, the directory's sorted child names where it
    /// cannot.
    lines: Vec<DoomedLine>,
}

/// One doomed item: the line the narration prints, and the path on disk it stands for.
///
/// **The path is what makes the narration outcome-keyed** ([`PendingLoss::narrate_taken`]).
/// The line's *text* cannot be compared before and after a removal: a failed
/// `git worktree remove` drops the worktree's own linkage, so the identical byte comes back
/// through the other [`LeftoverVerdict`] arm rendered as a bare child name instead of
/// `path (never staged)`, and a text comparison reads that re-spelling as a destruction.
/// Whether the bytes are still there is a question about the filesystem, and it is asked of
/// the filesystem.
struct DoomedLine {
    /// What the narration prints for this item.
    text: String,
    /// Where those bytes are — gone afterwards iff the removal took them.
    at: PathBuf,
}

/// Probe what a removal at `path` would take. An empty `lines` means the narration has
/// nothing it may honestly print: an absent path, an empty directory, a worktree whose whole
/// content is staged — or a path this could not read at all, where the honest claim is that
/// there is none (see the arm below).
fn doomed_at(jigc_home: &Path, path: &Path) -> Result<Doomed> {
    const WORKTREE: &str = "fan-out worktree";
    // The shape is [`leftover_at`]'s, the refusal's own — never `exists()`/`is_dir()`, which
    // follow a symlink and had this surface enumerating through one (M51 EC-17).
    let at = leftover_at(path);
    // Nothing to name in either cell, for opposite reasons — and `Unreadable` must name
    // nothing *especially*: [`PendingLoss::narrate_taken`] calls a line taken when
    // `symlink_metadata` at it fails afterwards, which is the very condition that put the
    // path in this arm, so any line composed here would be reported as destroyed whether the
    // removal happened or not. A door that cannot read a path cannot honestly claim to have
    // taken what was under it.
    if matches!(at, LeftoverAt::Absent | LeftoverAt::Unreadable(_)) {
        return Ok(Doomed {
            subject: WORKTREE,
            lines: Vec::new(),
        });
    }
    // A **leaf** leftover has no children to enumerate — it *is* the bytes — and `child_names`
    // below would fail on it (or, at a symlink, succeed about somebody else's bytes), which
    // the narration swallows (best-effort), so the removal would take a leaf it never named.
    // It is named here instead, in the terms the probe can back: git vouches for nothing at a
    // path it cannot even enter.
    if at == LeftoverAt::Leaf {
        return Ok(Doomed {
            subject: "leftover file",
            // The bare file name, and — for the pathological path with none (a trailing
            // `..`) — the repo-relative spelling rather than `as_os_str()`'s host path,
            // which is the same law-1 leak one fallback deeper. The path it stands for is
            // the leftover itself: a non-directory leftover *is* the bytes.
            lines: vec![DoomedLine {
                text: path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| render::repo_relative(jigc_home, path)),
                at: path.to_path_buf(),
            }],
        });
    }
    match classify_leftover(path) {
        LeftoverVerdict::OwnWorktree => {
            let mut lines: Vec<DoomedLine> = discarded_work(path)?
                .into_iter()
                .map(|work| DoomedLine {
                    text: format!("{} ({})", work.path, work.state.label()),
                    at: path.join(&work.path),
                })
                .collect();
            // The un-concluded operation is a doomed item too — and on a clean tree it is the
            // ONLY one, which is why `--force` past this guard used to print nothing at all
            // (the independent review of `3c71da87`, the HIGH). Its `at` is the checkout's own
            // `.git` entry, so the line is outcome-keyed like every other: the two doors that
            // remove the checkout report it, and `jigc milestone discard` over a path this
            // repository never registered — which its teardown leaves on disk — does not.
            if let Some(operation) = held_operation(path) {
                lines.push(DoomedLine {
                    text: format!(
                        "{} git had left un-concluded — it can only be concluded or abandoned \
                         from this checkout",
                        operation.noun(),
                    ),
                    at: path.join(".git"),
                });
            }
            Ok(Doomed {
                subject: WORKTREE,
                lines,
            })
        }
        // Nothing vouches for these bytes, so nothing may be claimed about them beyond their
        // names — the [`child_names`] listing the two fail-closed refusals already print.
        LeftoverVerdict::Unverifiable | LeftoverVerdict::NoOwnLinkage => Ok(Doomed {
            subject: "leftover directory",
            lines: child_names(jigc_home, path)?
                .into_iter()
                .map(|name| DoomedLine {
                    at: path.join(&name),
                    text: name,
                })
                .collect(),
        }),
    }
}

/// What a removal at `path` would take, read **before** the removal runs so the narration
/// afterwards can name what it actually took — the capture half of the pair whose other half
/// is [`PendingLoss::narrate_taken`].
///
/// **The narration is keyed on the outcome, not on the intent** (M50 Increment 12 audit).
/// Until this pair existed, [`narrate_removal`] printed *"they are not recoverable"*
/// immediately **before** the removal, so a removal that then failed left the door claiming a
/// destruction it had not performed. T2 closed that for one *shape* — `remove_dir_all` aimed
/// at a plain file — and the class is not a shape, it is *any* cause a removal can fail for: a
/// read-only parent directory, a lock, a busy path. Driven at `1653d3d` it was live at three
/// of this pair's call sites at once, on one `chmod 555` state: `jigc milestone provision
/// --force` (exit 1), `jigc uninstall --force` (exit 1) and `jigc milestone discard --force`
/// (**exit 0**) each printed `not recoverable` over bytes still on disk afterwards. Keying the
/// narration on what the removal *did* is complete over every cause by construction, and it is
/// the rule [`remove_leftover`] already states: *a door either performs the removal it
/// narrated or refuses before narrating it; there is no third honest outcome.*
///
/// **Best-effort**, like the emitter: a path this cannot read yields no warning rather than
/// failing the door.
///
/// `pub(crate)` for `crate::setup::uninstall`, whose `remove_dir_all(<repo>/.jigc)` takes
/// exactly these paths with it.
pub(crate) struct PendingLoss {
    /// The path the removal is aimed at — re-read afterwards to tell taken from survived.
    path: PathBuf,
    /// What was there before it ran. `None` when the probe could not read the path at all.
    doomed: Option<Doomed>,
}

/// Read [`PendingLoss`] at `path`. Call it immediately before the removal.
pub(crate) fn pending_loss(jigc_home: &Path, path: &Path) -> PendingLoss {
    PendingLoss {
        path: path.to_path_buf(),
        doomed: doomed_at(jigc_home, path).ok(),
    }
}

impl PendingLoss {
    /// Name what the removal actually took, and nothing it did not. Call it immediately
    /// after the removal, on **both** its outcomes — the failure path is the whole point.
    ///
    /// The taken set is **per item**: a line is printed iff the bytes it stands for
    /// ([`DoomedLine::at`]) are no longer on disk. So a **partial** removal — the case that
    /// would make a plain narrate-on-success silent about a real loss — still names every
    /// line it took, and a removal that failed outright prints nothing at all.
    pub(crate) fn narrate_taken(&self, jigc_home: &Path) {
        let Some(before) = &self.doomed else {
            return;
        };
        let taken: Vec<String> = before
            .lines
            .iter()
            // `symlink_metadata`, not `exists()`: a dangling symlink the removal left behind
            // reads as absent through `exists()`, and the door would report bytes it still
            // has in the way.
            .filter(|line| std::fs::symlink_metadata(&line.at).is_err())
            .map(|line| line.text.clone())
            .collect();
        narrate_removal(jigc_home, &self.path, before.subject, &taken);
    }
}

/// Print the bytes a destroying door **took** at `path`, on stderr — the law-1 minimum every
/// member of [`DESTROYING_DOORS`] owes (`design/surface-contract.md`: a door that exits 0 must
/// not also have silently destroyed work, and none of them may claim a destruction that did
/// not happen).
///
/// **One emitter for all four doors**, not one per door. `finalize` and `discard` narrated
/// from M47 Inc 3 while `jigc milestone provision --force` cleared a leftover in silence and
/// `jigc uninstall --force` removed `.jigc/` reporting only `- removed .jigc/` — and three
/// call sites of one rule drift, so a fourth door would have arrived with a fourth phrasing.
/// Its callers reach it through [`PendingLoss::narrate_taken`] — or, at the **foreign-byte**
/// subject, through `crate::task::PendingForeign::narrate_taken` — never directly, because
/// *which* lines a door may print is the outcome question those pairs answer. The `subject`
/// noun travels with the caller for the same reason the pairs do: what a path *is* is the
/// classifier's answer, not this renderer's guess.
///
/// An empty `taken` prints nothing: the removal took none of what it was aimed at.
pub(crate) fn narrate_removal(jigc_home: &Path, path: &Path, subject: &str, taken: &[String]) {
    if taken.is_empty() {
        return;
    }
    let listing: Vec<String> = taken.iter().map(|line| format!("    {line}")).collect();
    eprintln!(
        "warning: removing the {subject} {} discards work that is not in git:\n{}\n  note: the \
         {subject} is the only copy of these bytes — they are not recoverable.",
        render::repo_relative(jigc_home, path),
        listing.join("\n"),
    );
}

/// Assemble the landed-boundary facts for [`render::milestone_finalized`] (C2), read
/// off the **committed bytes** after the boundary landed: the abbreviated HEAD hash +
/// subject, and the landed-file manifest as `git diff --name-status --no-renames
/// <pre-boundary-HEAD> HEAD` — the whole boundary's delta, so the `squash: false` N+1
/// chain reads as one set, identical to what the single `squash: true` commit shows. A
/// path among the plan's promotion destinations tags `Promoted`; otherwise the status
/// letter maps `A`→added / `D`→deleted / else modified (the
/// `crate::task::classify_landed_manifest` mapping).
///
/// **The manifest states membership; [`boundary_commits`] states attribution** (M50 Inc
/// 11 / N12). The whole-range diff is de-duplicating by construction — one entry per
/// path — while `squash: false` can land one path in more than one chain commit; the
/// stated resolution rule is that such an entry's **owning sha is the LAST commit that
/// lists it**, the one whose bytes are at HEAD (`render::MilestoneLanded::manifest`).
/// Until M50 the ack named the aggregate alone beside the boundary-wide manifest, so it
/// attributed each sub-task's files to a sha that does not contain them.
///
/// `chain_subtask_ids` is the id-ordered list of **code-carrying** sub-tasks the
/// `squash: false` chain minted a commit for (empty on the `squash: true` arm, which
/// mints none). The chain lays those commits down in exactly that order and then the
/// aggregate, so pairing them with the boundary's commits is the construction read back —
/// and it is **guarded by the length check**: if the landed chain is not `ids + 1` long
/// (a hook that committed something of its own, say), no sub-task hash is claimed at all
/// rather than a wrong one being asserted.
fn milestone_landed_summary(
    repo_root: &Path,
    pre_boundary_head: &str,
    plan: &engine::finalize::FinalizePlan,
    sub_tasks: Vec<render::SubTaskContribution>,
    hook_output: &str,
    chain_subtask_ids: &[String],
    displaced: Vec<render::Displaced>,
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
    let commits = boundary_commits(repo_root, pre_boundary_head)?;
    let mut sub_tasks = sub_tasks;
    if commits.len() == chain_subtask_ids.len() + 1 {
        for (id, commit) in chain_subtask_ids.iter().zip(&commits) {
            if let Some(sub) = sub_tasks.iter_mut().find(|sub| sub.id == *id) {
                sub.hash = Some(commit.hash.clone());
            }
        }
    }
    Ok(render::MilestoneLanded {
        hash,
        subject,
        files: manifest.len(),
        manifest,
        commits,
        sub_tasks,
        // M45 — the SAME captured boundary-commit hook string the stderr relay carries at
        // the call site (one capture, two channels; `design/command-output-contract.md` →
        // Stream discipline). The join path folds into this same envelope.
        hook_output: hook_output.to_owned(),
        // M52 Inc 4 T4 — what the sub-task-area teardown moved aside rather than destroyed,
        // as the same repo-relative pairs the caller already named on stderr. Empty on the
        // ordinary boundary, and present either way.
        displaced,
        // M52 Inc 10 T5 — the narration's second axis: what the SHARED checkout still holds
        // staged, read back POST-boundary from the index the boundary left behind.
        still_staged: shared_checkout_staged(repo_root)?,
    })
}

/// The **shared checkout's** staged set, read back **after** the boundary landed — the
/// second axis of the landed narration [`discarded_work`] sweeps (M52 Increment 10 / T5;
/// `completions/artifacts/M52/baseline-surfaces.md` §2.1 correction 1).
///
/// A sub-task's composed body invites work at the base pin, and a sub-agent that works in
/// the main checkout rather than in its provisioned worktree stages **there**. Both
/// aggregate channels build their commit from the sub-task worktrees over targeted
/// pathspecs and land by `--ff-only`, so a live-index entry structurally cannot ride the
/// boundary commit — the fact the carryover gate already states for the *pre-milestone*
/// staged set (`engine::finalize`'s `CarryoverBoundary::Milestone`). An entry staged
/// **after** the mint is past that gate's subject, so until this probe it crossed the
/// boundary named on no surface, at exit 0.
///
/// **Measured after the commit, not before, and by the same question the claim makes.**
/// The surface says *these stay staged*, and `git diff --cached` against the landed HEAD is
/// that sentence read back out of git: a path the boundary did carry is gone from it by
/// construction, so the block cannot over-report the way a pre-commit snapshot minus a
/// predicted landed set could. It is the `crate::task` landed-residual mold
/// (`design/finalize.md` → the post-commit `left-out` residual), asked one directory up.
///
/// **`--name-only -z`, not the plain form** — the [`discarded_work`] rule, same reason: git
/// display-quotes a path holding a space or a non-ASCII byte regardless of `core.quotePath`,
/// and a shared checkout holds arbitrary user code, so the plain form would name a path the
/// repo does not contain (`design/surface-contract.md` → law 1). Paths come back
/// repo-relative from git and are sorted here, so the surface is order-stable.
fn shared_checkout_staged(repo_root: &Path) -> Result<Vec<String>> {
    const PROBE: [&str; 4] = ["diff", "--cached", "--name-only", "-z"];
    let out = Command::new("git")
        .args(PROBE)
        .current_dir(repo_root)
        .output()
        .context("could not run `git diff --cached` (is git on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` in {repo_root:?} failed: {}",
            PROBE.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let mut staged: Vec<String> = out
        .stdout
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
        .map(|record| String::from_utf8_lossy(record).into_owned())
        .collect();
    staged.sort();
    staged.dedup();
    Ok(staged)
}

/// **Every** commit the boundary landed, oldest first, each with the paths it changed —
/// the attribution channel of the landing ack (M50 Inc 11 / N12).
///
/// Read from **git** over `<pre-boundary-HEAD>..HEAD` rather than reported by the commit
/// path, so completeness is a property of the range and not of every commit site
/// remembering to hand its sha back: the `squash: false` chain builds its commits in a
/// dedicated worktree and fast-forwards main, and this reads the shas that actually
/// survived that fast-forward.
///
/// A boundary that made the repo's **first** commit pins `pre_boundary_head` to the
/// empty-tree sentinel (`crate::task::EMPTY_TREE_SHA`, the unborn-HEAD case
/// [`crate::task::git_head`] returns), which is a tree and not a commit — so the range
/// degenerates to the whole history, which is exactly the set that boundary landed.
fn boundary_commits(
    repo_root: &Path,
    pre_boundary_head: &str,
) -> Result<Vec<render::LandedCommit>> {
    let range = if pre_boundary_head == crate::task::EMPTY_TREE_SHA {
        "HEAD".to_owned()
    } else {
        format!("{pre_boundary_head}..HEAD")
    };
    let listed = crate::task::git_capture(repo_root, &["rev-list", "--reverse", &range])?;
    let mut commits = Vec::new();
    for sha in listed.lines().map(str::trim).filter(|sha| !sha.is_empty()) {
        // Two reads rather than one combined `--format=%h%n%s --name-only`: an empty
        // subject would make the blank separator line ambiguous, and the paths must be
        // git's own unadorned `--name-only` set.
        let identity =
            crate::task::git_capture(repo_root, &["show", "--no-patch", "--format=%h%n%s", sha])?;
        let (hash, subject) = identity.split_once('\n').unwrap_or((identity.as_str(), ""));
        let names =
            crate::task::git_capture(repo_root, &["show", "--name-only", "--format=", sha])?;
        commits.push(render::LandedCommit {
            hash: hash.trim().to_owned(),
            subject: subject.trim().to_owned(),
            paths: names
                .lines()
                .map(str::trim)
                .filter(|path| !path.is_empty())
                .map(str::to_owned)
                .collect(),
        });
    }
    Ok(commits)
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

/// Map an engine [`Finding`] to an `anyhow` error carrying its **key** + message +
/// route — the `severity · code — message` line shape the findings envelope prints
/// (round-2 D6h: a milestone finding surfaced through the operational-error funnel
/// used to drop its stable `(code, target)` key, so `milestone.no-criteria` — unlike
/// every envelope-rendered sibling — was undiscriminable to a driver).
///
/// **One shape, one site** (M50 Increment 10 / T1): this delegates to
/// [`crate::render::finding_error`], whose [`BlockedFinding`](crate::render::BlockedFinding)
/// `Display` **is** the house findings line — `severity · code — message`, the `at:` locus,
/// the `route:`. Until M50 the six modules that own a funnel each re-derived that shape, and
/// four of them (`describe` / `doc` / `start` / `task`) rendered `finding.message` **alone**: the code a driver keys on
/// reached `--format json` and never the text (`design/command-output-contract.md` → The
/// stable finding key; `design/surface-contract.md` → law 1). Carrying the finding rather
/// than only its rendering also lets the dispatch log the identity it prints
/// ([`crate::render::blocked_finding`]).
fn finding_to_err(finding: Finding) -> anyhow::Error {
    // **The declared arms** ([`ENVELOPE_ARM_CODES`]). The engine returns a bare `Finding` —
    // it is clap-blind and knows nothing of envelopes — so the arm each condition's machine
    // surface takes is declared here, at the one seam where this surface's findings become
    // errors. Each member is asked of the producing crate's own constant, never of a copied
    // literal, so a renamed code cannot leave this seam quietly matching nothing. Every
    // other finding keeps the flattened default and its declared bound.
    if ENVELOPE_ARM_CODES.contains(&finding.code.as_str()) {
        return crate::render::envelope_finding_error(&finding);
    }
    crate::render::finding_error(&finding)
}

/// The codes this module's refusals answer on the **findings arm** rather than on
/// [`crate::render::finding_error`]'s flattened `{"error": …}` default — the one place that
/// choice is made for every [`finding_to_err`] caller, so two producers of one code cannot
/// ship two wire shapes.
///
/// It is `pub` so `crates/cli/tests/milestone_envelope_arm.rs` can fence the relation it
/// declares — *flattened is this surface's default, and this list is its exception set* —
/// by **reading** the list rather than restating it (M53 Increment 5 / T1, the owed spike),
/// the posture [`crate::render::ENVELOPE_OWED_CODES`] already ships.
///
/// - `UNKNOWN_MILESTONE_CODE` (M51 Increment 6 / T2) — the contract lists it under the
///   **work-unit** target form, and its sibling doors ([`no_such_milestone`]) answer the
///   envelope.
/// - [`DISCARD_FOREIGN_BYTES_CODE`] (M52 Increment 4) — the increment registers the whole
///   `*.foreign-bytes` family on the findings arm (`implementation/roadmap.md` → Increment 4,
///   *Codes it registers*), and `uninstall`'s member has answered there since it landed; this
///   door's two producers flattened it, so one registration shipped two shapes a driver
///   cannot discriminate.
///
/// **[Corrected 2026-09-21 (M52 completion audit, fix 4).** A third row read: *"
/// [`crate::task::FIXED_IDENTITY`] (M52 Increment 6 / T3) — T2 landed the code on the
/// findings arm at the nine `doc` doors' shared funnel; this door is one of the three
/// sibling producers T3 adds, and a flattened answer here would ship two wire shapes for
/// one registered code."* The reasoning held and was **under-applied**: it is an argument
/// about the *code*, taken at one module's funnel, and the audit drove seven `(door, cell)`
/// coordinates where the same argument had not been taken — this module's own
/// `add-from-spec` among them, flattening `store.unknown-type` and `store.not-found` while
/// enveloping `store.fixed-identity` one guard later. The set of codes the contract lists
/// under a declared target form now lives in one home the **carrier** asks
/// ([`crate::render::ENVELOPE_OWED_CODES`]), so that row is discharged there rather than
/// restated here, and this list keeps only what is genuinely this module's: two codes the
/// contract lists under no `store.*` target form.**]
pub const ENVELOPE_ARM_CODES: &[&str] = &[
    engine::milestone::UNKNOWN_MILESTONE_CODE,
    DISCARD_FOREIGN_BYTES_CODE,
];

#[cfg(test)]
mod tests {
    use super::{SubTask, unwind_mint};
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
    /// `` Spawn: `cd <worktree> && jigc workflow <W> --task <id>` `` per sub-task, **in
    /// id-sorted order**.
    /// The feed is given in NON-id order (zebra before alpha) so the id-sorted emit is
    /// not an accident of feed order — and (Validation hardening #7) the **reversed**
    /// feed emits the byte-identical block, proving the resolver sorts on resolve, not a
    /// caller pre-sort. The emit is read straight off the composed bytes the agent runs
    /// — never a reconstructed equivalent.
    ///
    /// Both `<W>` arms are fed here (M49 Increment 10 / T3): `alpha-fix` records
    /// `decided-task` and is spawned under it; `zebra-fix` records **nothing** and falls
    /// back to the step's own `run:` — and the sort still keys on the **id**, not the
    /// workflow, across both feed orders.
    #[test]
    fn milestone_feeding_emits_one_id_sorted_spawn_per_subtask() {
        let pack = fanout_pack();
        let source = PackStepSource { pack: &pack };
        // `repo_root` is unused on the no-task arm (it never reads HEAD); a throwaway
        // path suffices — the compose feeds only the injected milestone list.
        let repo_root = Path::new("/nonexistent-milestone-execute-repo");

        // The id-sorted sub-task list, FED in non-id order.
        let fed = vec![
            SubTask::new("zebra-fix", None),
            SubTask::new("alpha-fix", Some("decided-task".to_owned())),
        ];
        let composed = execute_milestone_core(
            repo_root,
            &pack,
            "milestone-execution",
            &source,
            crate::start::MilestoneFeed::bound("fanout-milestone", &fed),
        )
        .expect("milestone-execution composes over the fed sub-task list")
        .view;

        // Exactly one Spawn line per sub-task, each `cd`-ing into its own worktree
        // before the bare re-entry workflow + id — the recorded workflow where the
        // sub-task carries one, the step's `run:` where it does not.
        let alpha =
            "Spawn: `cd .jigc/worktrees/alpha-fix && jigc workflow decided-task --task alpha-fix`";
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
        let reversed = vec![
            SubTask::new("alpha-fix", Some("decided-task".to_owned())),
            SubTask::new("zebra-fix", None),
        ];
        let composed_rev = execute_milestone_core(
            repo_root,
            &pack,
            "milestone-execution",
            &source,
            crate::start::MilestoneFeed::bound("fanout-milestone", &reversed),
        )
        .expect("the reversed feed composes")
        .view;
        assert_eq!(
            composed.text, composed_rev.text,
            "the fan-out emit must be byte-identical across divergent feed orders (id-sorted on resolve)",
        );
    }

    /// **The fifth writer of `tasks.json`** (M46 completion-audit F4, the sweep's own
    /// un-swept sibling): [`unwind_mint`] restores the milestone task list's pre-append bytes
    /// when `add-task`'s record commit is rejected — a write into the *shared* milestone area
    /// that every milestone door parses through `engine::milestone::read_task_list`.
    ///
    /// `engine::milestone::TASKS_FILE`'s doc-comment states the hazard and claims **all four
    /// of its writers** persist through `engine::state::persist`; it enumerated the four
    /// *engine* writers and missed this CLI-side restore, which truncated the file in place.
    /// The rollback runs on the failure path of a live door, so a sibling door reading the
    /// task list at that moment saw the same zero-byte window the comment names.
    ///
    /// Witnessed deterministically by the write's mechanism: a truncating `std::fs::write`
    /// refills the existing inode, while temp + `rename` swaps a new one in — so `ino` must
    /// change across the restore.
    #[test]
    fn the_task_list_restore_replaces_rather_than_truncates() {
        use std::os::unix::fs::MetadataExt;

        // A throwaway area (the project's no-tempfile pattern).
        let area = std::env::temp_dir().join(format!(
            "jigc-unwind-mint-{}-{}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(&area).expect("open a temp milestone area");
        let list = area.join("tasks.json");
        std::fs::write(&list, br#"{"tasks":["task:a","task:b"]}"#).expect("seed the task list");
        // Held open across the restore for the reason its engine-side sibling
        // (`engine::milestone::tests::shared_area_writers_replace_rather_than_truncate`)
        // states in full: the handle is the concurrent `read_task_list` this claim is about,
        // and pinning the old inode is what stops a filesystem that recycles inode numbers
        // from handing one back and making a correct temp + `rename` read as a truncation.
        // One write stands between the two readings here, so the pin is what keeps that a
        // property of the test rather than of the arm count.
        let held_open_by_a_reader = std::fs::File::open(&list).expect("hold the seeded list open");
        let before = held_open_by_a_reader
            .metadata()
            .expect("the held handle answers metadata")
            .ino();

        // The mint half unwinds a sub-task area this call created; the restore half puts the
        // task list back to its captured pre-append bytes.
        let minted = area.join("minted-sub-task-area");
        std::fs::create_dir_all(&minted).expect("stage the minted area");
        let pre_append = br#"{"tasks":["task:a"]}"#;
        let conflicts = unwind_mint(
            &area,
            &minted,
            engine::state::WorkArea::Task,
            Some((list.as_path(), pre_append)),
        );

        assert!(!minted.exists(), "the minted area is unwound");
        assert!(
            conflicts.is_empty(),
            "an area holding nothing a third party wrote raises nothing (M52 Inc 5 / T7); \
             got {conflicts:?}",
        );
        assert_eq!(
            std::fs::read(&list).expect("the list survives"),
            pre_append,
            "the restore puts back the captured pre-append bytes",
        );
        assert_ne!(
            before,
            std::fs::metadata(&list).expect("the restored list").ino(),
            "the restore must REPLACE the shared task list (temp + rename), not truncate-and-\
             refill the inode a concurrent `read_task_list` may already have open",
        );
        drop(held_open_by_a_reader);

        let _ = std::fs::remove_dir_all(&area);
    }

    /// The multi-path commit helper (M55 Increment 1 / T1): it commits **exactly** the listed
    /// paths, leaves another staged path staged and out of the commit, and refuses an empty
    /// list before git runs — `git commit --` with no path would commit the whole index.
    #[test]
    fn git_commit_paths_commits_exactly_the_listed_paths_and_refuses_none() {
        let dir = std::env::temp_dir().join(format!(
            "jigc-commit-paths-{}-{}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(&dir).expect("mk temp repo");
        let git = |args: &[&str]| -> String {
            let out = std::process::Command::new("git")
                .args(args)
                .current_dir(&dir)
                .output()
                .expect("git runs");
            assert!(out.status.success(), "git {args:?} should succeed");
            String::from_utf8(out.stdout).expect("utf-8 git stdout")
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "t@example.com"]);
        git(&["config", "user.name", "Test"]);
        git(&["config", "commit.gpgsign", "false"]);
        git(&["commit", "-q", "--allow-empty", "--no-verify", "-m", "init"]);
        for name in ["a.txt", "b.txt", "other.txt"] {
            std::fs::write(dir.join(name), format!("{name}\n")).expect("write a file");
        }
        git(&["add", "--", "a.txt", "b.txt", "other.txt"]);
        let msg = dir.join("msg.txt");
        std::fs::write(&msg, "docs: two paths\n").expect("write msg");
        let subject = crate::repo::SeamSubject::live(&dir);

        let head = git(&["rev-parse", "HEAD"]);
        let refused = super::git_commit_paths(&subject, &msg, &[]);
        assert!(refused.is_err(), "an empty path list is refused");
        assert_eq!(
            git(&["rev-parse", "HEAD"]),
            head,
            "the refusal committed nothing — the whole index stayed staged",
        );

        super::git_commit_paths(&subject, &msg, &["a.txt".to_owned(), "b.txt".to_owned()])
            .expect("the path-scoped commit lands");
        assert_eq!(
            git(&["show", "--name-only", "--pretty=format:", "HEAD"]).trim(),
            "a.txt\nb.txt",
            "the commit holds exactly the listed paths",
        );
        assert_eq!(
            git(&["diff", "--cached", "--name-only"]).trim(),
            "other.txt",
            "the unlisted staged path stays staged",
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
