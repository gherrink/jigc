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
use crate::invocation_log::{self, Outcome};
use crate::pack::make_pack;
use crate::render;
use crate::repo::discover_repo_root;
use crate::repo::{SeamAct, SeamSubject};
use anyhow::{Context, Result, bail};
use engine::address::Address;
use engine::compose::{WorkflowDef, load_workflow_def};
use engine::file_state::{self, FileStateRecord};
use engine::finalize::{
    CarryoverBoundary, Promotion, RepinDecision, decide_base_repin, decide_carryover, plan_finalize,
};
use engine::finding::{Finding, Findings, Location, Severity};
use engine::packsource::PackSource;
use engine::probe::{ProbeRequest, ProbeRun, ProbeRunStatus};
use engine::schema::Schema;
use engine::state::{self, BasePin, RolesRecord};
use engine::store::canonical_path;
use engine::validate::{owner_artifacts_gate, validate_task};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The five outcome classes of the exit-code taxonomy
/// (`design/command-output-contract.md` → The exit-code taxonomy). The stable identity
/// of each [`EXIT_CODES`] row, so a consumer selects a code by *what it means* — never a
/// hand literal (the AGENT.md one-liner render below; the exit-code suite in `tests/`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitClass {
    /// The run did what was asked. A store-scope `jigc validate` carrying content
    /// findings (report-only) and clap's `--help`/`--version` also land here.
    Success,
    /// Operational error, and every reject that is not a task-scope gate: an absent
    /// task id, a git/IO failure, a blocked write (`write.*`), a hook-rejected finalize,
    /// and the store-scope exit flips [`crate::render::STORE_EXIT_FLIPS`] enumerates.
    /// The count is deliberately not restated — the table **is** the enumeration
    /// (`design/validation.md` → Exit semantics); the hand-count that stood here went
    /// stale against it (M49 Increment 8 / T6).
    Error,
    /// Usage error — clap's own convention, emitted before jigc reads `--format`.
    Usage,
    /// Blocking findings at a task-scope gate: `jigc task validate` / `task finalize` /
    /// `milestone finalize` only — never the store sweep, never a rejected write.
    TaskGateBlocked,
    /// Migration review hold — a migration `finalize` without `--approve`.
    MigrationReview,
}

/// One row of the exit-code taxonomy — the single code-side origin of the `0/1/2/3/4`
/// vocabulary the AGENT.md one-liner and the exit-code suite both assert against
/// (statement == constant). Homed in `task.rs` because it is the crate's exit-code home
/// (the two named codes 3/4 live here) and it is reachable from `tests/*.rs`
/// (`implementation/pinning.md` → §2, the exit-code taxonomy suite).
pub struct ExitCode {
    /// The outcome class this row identifies.
    pub class: ExitClass,
    /// The numeric process exit code.
    pub code: u8,
    /// What the code means — the outcome class in words.
    pub meaning: &'static str,
    /// The task-scope-vs-store-scope qualifier that disambiguates it.
    pub scope: &'static str,
    /// The short label the AGENT.md bootstrap one-liner names this code by, or `None`
    /// for a code that line does not enumerate (0 — success is not an exit caution).
    pub bootstrap_label: Option<&'static str>,
}

/// Exit 0 — a clean run.
pub const EXIT_SUCCESS: u8 = 0;

/// Exit 1 — an operational error, and every reject that is not a task-scope gate.
pub const EXIT_ERROR: u8 = 1;

/// Exit 2 — a clap usage error (clap's own convention, emitted before jigc reads
/// `--format`).
pub const EXIT_USAGE: u8 = 2;

/// The validation-blocked exit code: a blocking-`ValidationReport` /
/// `plan_finalize`-findings outcome — distinct from an operational error (1) and
/// clap's usage error (2), so a harness-side tally can discriminate outcomes from
/// the exit code alone (`design/measurement.md` → The capture substrate, item 2:
/// drift-caught and the `validate-blocks` paired count key on it). Setup/ingest/join
/// blocks and git/hook commit rejections are NOT validation outcomes and stay 1.
pub const EXIT_VALIDATION_BLOCKED: u8 = 3;

/// The exit code a **migration** `finalize` returns when it blocks at the review gate
/// (`design/auto-migration.md` → The review gate): the human has not yet `--approve`d
/// the fidelity diff, so nothing is committed. Distinct from [`EXIT_VALIDATION_BLOCKED`]
/// (3) — the rewrite is structurally conformant; this is a *human-fidelity* hold, not a
/// validation outcome — and from the operational error (1), so a harness-side tally can
/// discriminate a pending review from a real block by the exit code alone
/// (`design/measurement.md` → exit-code hygiene).
pub const EXIT_REVIEW_PENDING: u8 = 4;

/// The exit-code taxonomy table — one row per outcome class, the design of record
/// (`design/command-output-contract.md` → The exit-code taxonomy) rendered as a
/// code-side constant. It folds in the two named codes above (3/4) and the three
/// bare-literal codes (0/1/2) the dispatch sites used to hand-write, giving the
/// vocabulary a single origin.
pub const EXIT_CODES: &[ExitCode] = &[
    ExitCode {
        class: ExitClass::Success,
        code: EXIT_SUCCESS,
        meaning: "the run did what was asked",
        scope: "includes a store-scope `jigc validate` carrying content findings \
                (report-only) and clap's `--help`/`--version`",
        bootstrap_label: None,
    },
    ExitCode {
        class: ExitClass::Error,
        code: EXIT_ERROR,
        meaning: "operational error, and every reject that is not a task-scope gate",
        scope: "an absent task id, a git/IO failure, a blocked write, a hook-rejected \
                finalize, and the store-scope exit flips `render::STORE_EXIT_FLIPS` enumerates",
        bootstrap_label: Some("error"),
    },
    ExitCode {
        class: ExitClass::Usage,
        code: EXIT_USAGE,
        meaning: "usage error",
        scope: "clap's own convention, emitted before jigc reads `--format`",
        bootstrap_label: Some("usage"),
    },
    ExitCode {
        class: ExitClass::TaskGateBlocked,
        code: EXIT_VALIDATION_BLOCKED,
        meaning: "blocking findings at a task-scope gate",
        scope: "the transaction gates only — `task validate`/`task finalize`/`milestone \
                finalize`; never the store sweep, never a rejected write",
        bootstrap_label: Some("blocking findings at a task-scope gate"),
    },
    ExitCode {
        class: ExitClass::MigrationReview,
        code: EXIT_REVIEW_PENDING,
        meaning: "migration review hold",
        scope: "a migration `finalize` without `--approve`: the fidelity diff rendered, \
                nothing committed",
        bootstrap_label: Some("migration review hold"),
    },
];

/// The exit code for an outcome `class`, read from [`EXIT_CODES`] — never a hand
/// literal. The exit-code suite and any consumer assert against this, not a `Some(3)`.
pub fn exit_code_for(class: ExitClass) -> u8 {
    EXIT_CODES
        .iter()
        .find(|row| row.class == class)
        .expect("every ExitClass has a taxonomy row")
        .code
}

/// Render the exit-code taxonomy as the AGENT.md bootstrap one-liner — each code the
/// line names, `<code> <label>`, joined by ` · ` — seam-generated from [`EXIT_CODES`]
/// so the stated line cannot drift from the table (statement == constant, the M43
/// corollary; the `bootstrap` context selects which rows the line names, so code 0 is
/// omitted). Asserted against `BOOTSTRAP_OUTPUT_CONTRACT` in `adapter.rs`.
pub fn bootstrap_taxonomy_line() -> String {
    EXIT_CODES
        .iter()
        .filter_map(|row| {
            row.bootstrap_label
                .map(|label| format!("{} {label}", row.code))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

/// The `task finalize` long help — the gate split stated **generated**, from the one
/// table that already states it (M51 Increment 9, EC-11).
///
/// [`crate::gate_coverage::whats_left_coverage`] renders the coverage sentence onto the
/// composed `what's-left:` line, and seven further surfaces are fenced per token against
/// the same table. The commit boundary's own help said nothing about it, so the door an
/// agent runs *after* a clean preview never named the gates that preview had not reached.
/// A sentence typed here would have been a ninth home; this renders the generated one.
fn finalize_long_about() -> String {
    format!(
        "The commit boundary — validate, render, stage, `git commit`, post-commit.\n\n\
         `jigc task validate <id>` {}. Those later-phase gates are decided here, at the \
         real finalize — so a clean preview is not a promise the commit lands.\n\n\
         On a task minted by `jigc task amend` it takes its **second commit model**: it {} \
         rather than adding one. That arm stages nothing and refuses over a non-empty index \
         (`git commit --amend` rewrites `HEAD` from the index, so anything staged would be \
         folded into a commit that never carried it); there is no flag that declares that \
         carry-over deliberate, so on that arm the index gate above is this refusal and not \
         the carryover gate.",
        // One help text serves both arms, so the coverage sentence renders the **ordinary**
        // model's spelling and the paragraph below names the amend arm's own index gate in
        // its own words (the F-10 review's MEDIUM-3). Rendering the amend spelling here
        // instead would state an amend-only name on the door every ordinary task runs.
        crate::gate_coverage::whats_left_coverage(crate::render::CommitModel::Index),
        crate::invocation_log::TASK_FINALIZE_AMEND_COMMITS,
    )
}

/// **The amend arm's index gate** (F-10) — `git commit --amend` rewrites `HEAD` from the
/// index, so anything staged when it runs is folded into the rewritten commit, silently, at
/// exit 0. Driven on the baseline: `HEAD` held one file, an unrelated `unrelated.txt` was
/// staged, `git commit --amend -F <msg>` exited 0, and the amended commit carried two.
///
/// That is [`finalize.carried-staged`](engine::finalize) re-enacted **on a commit that
/// already landed**, which is why this refusal is stricter than its sibling rather than a
/// copy of it: the carryover gate compares against a pre-task snapshot and `--carry-staged`
/// declares the carry deliberate, while here *every* staged path is wrong by construction —
/// the arm's whole contract is that the committed tree does not move — so the subject is the
/// index itself and the door offers no flag that waves it through.
///
/// One finding per staged path, keyed at that path (the file form, since the subject carries
/// no `<type>:<slug>` identity), so a driver reading `findings[].key` gets the set rather
/// than a count. The route unstages exactly the path it names, aimed through
/// [`engine::finding::git_at`] so it runs from any cwd.
fn amend_index_dirty_finding(path: &str, repo_root: &Path) -> Finding {
    let unstage = engine::finding::git_at(
        repo_root,
        &format!("restore --staged -- {}", engine::finding::shell_token(path)),
    );
    Finding::graded(
        Severity::Blocking,
        "finalize.amend-index-dirty",
        format!(
            "`{path}` is staged, and an amend rewrites `HEAD` from the index — finalizing now \
             would fold it into the commit whose message this task is repairing, a change \
             that commit never carried"
        ),
        Some(Location::addressed(path.to_string(), 1, 1)),
        Some(engine::finding::Route::human(format!(
            "unstage it (`{unstage}`) and re-run the finalize — this arm takes no \
             `--carry-staged`, because an amend that carried anything would change a tree it \
             promised not to touch"
        ))),
    )
}

/// Which position raised [`amend_staged_doc_finding`]. The two differ **only** in the
/// route, and they differ because the state differs — the finding, its code and its
/// message are one identity for one condition.
pub(crate) enum AmendDocDoor<'a> {
    /// A **`jigc doc` write verb**, asked at the copy-on-first-touch seam before anything
    /// is staged. The committed doc is untouched and this task still has its message to
    /// repair, so the repair is to author the doc's change where it can land.
    Write,
    /// The **finalize arm**, over a doc the task has already staged. No verb un-stages one
    /// doc from a task, so the task itself is what goes — and `jigc task discard` refuses
    /// over staged prose without `--force`, which is why the consent is named.
    Finalize { task: &'a str },
}

/// **The amend arm's staged-doc gate** (the F-10 review's HIGH-1) — an amend task cannot
/// carry a doc that **promotes**, because its commit model changes no tree.
///
/// Driven at `d9c4bd84`: inside an amend task an ordinary `jigc doc set-slot
/// vision:vision#thesis` acked *"copied in for update — … re-promoted at finalize"* at exit
/// 0, and `jigc task finalize` then **did** promote it — `try_execute_finalize_plan`'s
/// promote phase runs over `plan.promotions` on every arm — while `StagePolicy::Amend`
/// staged nothing and `git commit --amend` rewrote only the message. After exit 0:
/// ` M VISION.md` in the worktree, `git show HEAD:VISION.md` still the old bytes, the landed
/// manifest empty, the pinned `jigc doc show vision:vision` **committed**-doc read serving
/// the uncommitted bytes, and `jigc validate` **exit 0** because the transaction had
/// baselined file-state to its own un-committed promotion. Exit-0 divergence behind a
/// committing door, plus a false green on the CI-gated verb. The asymmetry that showed it
/// was unintended: a *rejected* amend rolls that promotion back, so the transaction
/// discarded a promotion it refused to commit and kept the one it did.
///
/// **The axis is the promote predicate, not a doctype name.** Membership is
/// [`engine::finalize::promote_destination`] answering `Some` — the very function the
/// promote plan is built from — so the gate and the promote cannot disagree about which
/// docs diverge. In both shipped packs `commit` is the only doctype it answers `None` for,
/// which is the corollary `design/finalize.md` states as *an amend task carries exactly one
/// doc, its own transient commit doc*; the rule itself is derived rather than counted, so a
/// pack that ships a second transient doctype stays correct with no edit here.
///
/// One finding per staged doc, keyed at the **destination** — the file that would be left
/// diverged — on the [`FinalizeSubject::FilePath`](crate::render::FinalizeSubject) precedent
/// its `promote-io` / `promote-clobber` siblings already take, so a driver reading
/// `findings[].key` gets the set rather than a count.
pub(crate) fn amend_staged_doc_finding(
    address: &str,
    home: &str,
    door: AmendDocDoor<'_>,
) -> Finding {
    let route = match door {
        AmendDocDoor::Write => engine::finding::Route::mechanical(
            ["jigc", "start", "\"<intent>\""],
            " mints an ordinary task, whose finalize commits the promoted doc; this amend \
             task keeps its own job, repairing `HEAD`'s message",
        ),
        AmendDocDoor::Finalize { task } => engine::finding::Route::mechanical(
            ["jigc", "task", "discard", task, "--force"],
            " discards this amend task — the committed doc is untouched, so the store loses \
             nothing; then author the doc's change in an ordinary task (`jigc start \
             \"<intent>\"`) and repair the message with a fresh `jigc task amend`",
        ),
    };
    Finding::graded(
        Severity::Blocking,
        "finalize.amend-staged-doc",
        // One message for one condition, worded to be true at **both** positions: the
        // write door is asked before the doc is staged and the finalize arm after, so a
        // sentence built around *staged* would be false at one of them.
        format!(
            "`{address}` is a managed doc, and it promotes to `{home}` — but this task's \
             commit model is an amend, which changes no tree: its finalize would write \
             `{home}` into the worktree and commit none of it, leaving the file diverged \
             from the commit it just rewrote"
        ),
        Some(Location::addressed(home.to_string(), 1, 1)),
        Some(route),
    )
}

/// The `jigc task amend` long help.
///
/// **This door commits nothing, and says so** — the surface-contract's law 1 read the other
/// way round: `jigc task amend` is `BEHALF_DOORS`' `Neither`, so a help text implying it
/// rewrites HEAD would be as much a lie as a committing door that stayed silent. What it
/// does is mint; `jigc task finalize <id>` is what rewrites the commit, and the help names it.
///
/// It also states the two facts an agent cannot discover from the door's own output: that
/// the tree is not the subject (so a wrong *change* is a new task, not an amend), and that
/// amending a commit which has been pushed rewrites shared history. The second is **advice,
/// never a fence** — the baseline drove `git branch -r --contains HEAD` empty for a
/// never-fetched remote as well as a never-pushed commit, so a guard here would be a
/// heuristic wearing a fence's clothes.
fn amend_long_about() -> String {
    format!(
        "Start a task that re-authors the commit message at HEAD — the tree is untouched.\n\n\
         This mints a task with an empty `commit` doc pinned to HEAD and composes the \
         `amend` workflow; it commits nothing itself. Author the doc through the ordinary \
         `jigc doc` verbs, then `jigc task finalize <id>` renders it and rewrites the \
         commit with `git commit --amend`, leaving every byte of the committed tree as it \
         is — so a wrong *change* needs a new task, not an amend.\n\n\
         HEAD's message is not read back into the doc: you author the new message from \
         scratch. {ADVISORY_PUSHED_HISTORY}\n\n\
         The amend refuses over a non-empty index (`git commit --amend` would otherwise \
         fold the whole of it into the rewritten commit) and there is no flag that \
         declares that carry-over deliberate.",
    )
}

/// The pushed-history sentence — the `jigc task amend` mint ack's advisory tail, and the
/// same words in the door's long help.
///
/// One home for both, because they are one statement: an ack that warned and a help that
/// did not (or that worded it as a refusal) is the drift law 1 fences. It carries **no
/// finding code and no route** — it is not a `Finding`, because a finding claims jigc
/// checked something, and jigc cannot check this.
pub(crate) const ADVISORY_PUSHED_HISTORY: &str = "If this commit has already been pushed, amending it rewrites shared history — jigc \
     cannot tell whether it has.";

/// The `jigc task discard` long help — **the door says that it commits** (M52 Increment 10 /
/// T2, D-2). Its two "commit" hits were both inside `--force`'s *"no commit has a copy of"*,
/// so a literal scan read as truthful while the door never said it moves `HEAD`: a sub-task's
/// discard settles its milestone's committed record and lands a record-only commit
/// ([`crate::milestone::settle_discarded_sub_task`]) — this is
/// [`COMMITTING_DOORS`](crate::invocation_log::COMMITTING_DOORS)' tenth member, and the claim
/// renders that row's own
/// [`TASK_DISCARD_COMMITS`](crate::invocation_log::TASK_DISCARD_COMMITS) clause.
///
/// The **conditional** is stated beside it rather than left to be discovered: an ordinary
/// task's discard is workbench-local and commits nothing, which is the common cell.
fn discard_long_about() -> String {
    format!(
        "Abandon the task — remove its working area `.jigc/tasks/<id>/`.\n\n\
         A **sub-task** of a milestone is not workbench-local: discarding one {} before \
         the area goes, so `HEAD` moves and the ack names the sha. That commit is \
         path-scoped to the record — anything else you had staged stays staged. An \
         ordinary task's discard commits nothing.",
        crate::invocation_log::TASK_DISCARD_COMMITS,
    )
}

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
        /// Declare the carry-over of pre-task staged changes deliberate, so this
        /// preview mirrors the `finalize --carry-staged` you intend to run: the
        /// carryover gate's findings (`finalize.carried-staged`) are omitted, exactly
        /// as the declared finalize omits them. Inert when nothing is carried.
        #[arg(long)]
        carry_staged: bool,
    },
    /// Start a task that re-authors the commit message at HEAD — the tree is untouched.
    #[command(long_about = amend_long_about())]
    Amend {
        /// What this repair is for, in your own words — the line the task id slugs from
        /// and `{{task.intent}}` composes with. Omitted, the task is named after the
        /// commit it rewrites (`amend-<sha7>`).
        intent: Option<String>,
    },
    /// Abandon the task — remove its working area `.jigc/tasks/<id>/`.
    #[command(long_about = discard_long_about())]
    Discard {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
        /// Remove the working area even when it holds staged docs no commit has a copy
        /// of, or files jigc did not write there at all — the explicit consent to
        /// destroy them, and the single consent for both guards. `jigc start` stages the
        /// task's `commit:<id>` doc at mint, so an ordinary discard needs this from the
        /// moment the task exists. Inert when the area stages nothing and holds nothing
        /// foreign (a `milestone add-task` sub-task before its first re-entry); it never
        /// buys silence — what it takes is named as it goes.
        #[arg(long)]
        force: bool,
    },
    /// The commit boundary — validate, render, stage, `git commit`, post-commit.
    #[command(long_about = finalize_long_about())]
    Finalize {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
        /// Approve a migration task's fidelity diff and proceed through the commit
        /// transaction. Without it, a migration `finalize` renders the diff and blocks
        /// (exit 4, nothing committed). Inert on a non-migration task.
        #[arg(long)]
        approve: bool,
        /// Print the commit's forecast subject line and the pre-commit manifest (the
        /// file-set the commit would carry, untracked sweeps flagged) and stop — commit
        /// nothing, no destructive side effect (B1 dirty-tree sweep). The subject is the
        /// one jigc will hand git, not what git ends up with: a `commit-msg` hook may
        /// still rewrite it. A dry-run never requires `--approve`. It *refuses*, rather
        /// than printing a manifest, on **every gate decided before the transaction** —
        /// which is more than `jigc task validate` reports: the repository posture, this
        /// task's validation findings, the **base pin** (`finalize.base-mismatch`), the
        /// empty-commit guard (`finalize.empty-commit`, or `finalize.nothing-staged` where
        /// the working tree has changes and the index is empty), the **sub-task boundary**
        /// (`finalize.milestone-sub-task` — a milestone's sub-task has no per-task commit
        /// door at all), the planner's two collision gates
        /// (`finalize.promote-clobber` over a file already sitting at a promote
        /// destination, `finalize.migration-no-replacement` over a recorded migration
        /// source with nothing staged to replace it), and the carryover gate, where an
        /// undeclared carry-over is
        /// reported (exit 3) instead of the manifest (add `--carry-staged` to forecast the
        /// carry). On an **amend** task three of those read differently: the amend adds no
        /// commit, so the empty-commit guard does not apply; the index gate is the arm's
        /// own `finalize.amend-index-dirty` over *any* staged path, which `--carry-staged`
        /// cannot forecast past, joined by `finalize.amend-staged-doc` over a staged doc
        /// that would promote; and the base pin is the commit it rewrites having moved out
        /// from under it. When it **does** print a manifest, its `findings` are the set
        /// `jigc task validate <id>` reports, the staging-independent `owner-artifact`
        /// causes included — reported here, decided at the real finalize. The gates named
        /// above are **outside** that set by design — the base pin, the empty-commit /
        /// nothing-staged guard, the sub-task boundary and the planner's two collision gates
        /// — so a refusal here can name a code no `jigc task validate <id>` will ever print;
        /// the amend arm's two are the exception, being the index gate the preview answers on
        /// that arm. Every other gate —
        /// staging, promotion, the untracked
        /// `owner-artifact` cause, the commit hook — is decided only by the real finalize,
        /// so a printed manifest is not a promise the commit lands.
        #[arg(long)]
        dry_run: bool,
        /// Declare the carry-over of pre-task staged changes deliberate: land index
        /// entries staged before this task existed instead of refusing
        /// (`finalize.carried-staged`). On a migration task it composes with
        /// `--approve` — two independent declarations. Inert when nothing is carried,
        /// and inert on an **amend** task in every state: that arm refuses over *any*
        /// non-empty index (`finalize.amend-index-dirty`) because `git commit --amend`
        /// rewrites HEAD from the index, so there is nothing here to declare deliberate
        /// and this flag declares nothing there.
        #[arg(long)]
        carry_staged: bool,
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
    pub fn dispatch(self, cwd: &Path, format: Format) -> Outcome {
        let result = match self {
            TaskCommand::List => run_list(cwd, format),
            TaskCommand::Diff { id } => run_diff(cwd, &id, format),
            TaskCommand::Validate { id, carry_staged } => {
                return run_validate(cwd, &id, format, carry_staged);
            }
            TaskCommand::Discard { id, force } => {
                // Its own arm: a sub-task discard commits the record, so a rejecting hook is
                // framed with this door's state-truth clause + re-run and names itself in the
                // invocation log, instead of falling to the plain operational envelope.
                let frame = discard_rejection_frame(&id, force);
                // The sub-task record's rollback conflicts, gathered on the failure arm and
                // printed beside this door's frame (M52 Increment 5 / T4). Empty on success
                // and on every ordinary (non-sub-task) discard, which settles no record.
                let mut conflicts: Vec<Finding> = Vec::new();
                return match run_discard(cwd, &id, format, force, &mut conflicts) {
                    Ok(()) => Outcome::success(),
                    Err(err) => {
                        // The **staged-prose refusal** names itself (M50 Inc 3 / T2, the
                        // `rename` precedent): it travels as a `render::BlockedFinding`, so
                        // the identity the surface prints is the identity the invocation log
                        // records — a refused discard is legible there rather than one more
                        // exit-1-with-nothing. The read-back is no longer written out here:
                        // this door and `rename` were the only two that had it, which is
                        // precisely why the other ~29 dropped the identity (M50 completion
                        // audit, Finding 2). It rides the one funnel
                        // `surface_commit_rejection` falls through to
                        // ([`crate::invocation_log::operational_failure`]); the printed bytes
                        // are unchanged.
                        surface_commit_rejection(format, &err, &frame, &conflicts)
                    }
                };
            }
            TaskCommand::Finalize {
                id,
                approve,
                dry_run,
                carry_staged,
            } => {
                return run_finalize(cwd, &id, format, approve, dry_run, carry_staged);
            }
            TaskCommand::Amend { intent } => run_amend(cwd, intent.as_deref(), format),
            TaskCommand::Bind { role, addr, id } => run_bind(cwd, &role, &addr, &id, format),
        };
        match result {
            Ok(()) => Outcome::success(),
            Err(err) => crate::invocation_log::operational_failure(format, &err),
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

/// `jigc task amend ["<intent>"]` — mint a task that re-authors the commit message at HEAD
/// and print its composed workflow (F-10).
///
/// It prints on **stdout** at exit 0 and renders through [`render::composed`] like every
/// other composing door, so the pinned `{task, text}` envelope is byte-identical in shape to
/// `jigc start`'s — the commit it pins is named on the presentation block, not on a new key
/// (`design/command-output-contract.md` §1).
///
/// A refused HEAD shape, a repository with no project layer, and an unslugable intent all
/// surface through the ordinary operational funnel with their routes, having minted nothing.
fn run_amend(cwd: &Path, intent: Option<&str>, format: Format) -> Result<()> {
    let composition = crate::start::amend_in_repo(cwd, intent)?;
    println!("{}", render::composed(format, &composition));
    Ok(())
}

/// `jigc task diff <id>` — print the working changeset vs base.
///
/// Two parts: (1) the git diff of the working tree against the task's pinned base
/// SHA (the code changes — "CLI orchestrates, git executes"), and (2) the staged
/// managed-doc instances under the working area's `docs/` (which live outside the
/// tree, so git does not see them). Read-only.
///
/// **`format` reaches the render (M47 Inc 7).** This was the one dispatch arm of the
/// six that dropped it, so `--format json` printed markdown here and — over a working
/// area that staged nothing, with no code change — printed **zero bytes on both
/// streams at exit 0**. Both halves close through [`render::task_diff`]: JSON emits the
/// settled `task-diff` envelope, whose present-always `code_diff` / `staged_docs` keys
/// make the empty state non-empty; agent-text is byte-unchanged.
///
/// **The diff's subject is the checkout that holds the task's code, not the one the
/// caller stands in** ([`TaskArea::code_checkout`]; M53 — the cwd census, row C2-02). It
/// read `repo_root` — the bare walk-up — so an orchestrator at the repository root asking
/// what a fan-out sub-task had changed was answered about the *main* checkout: the
/// milestone-record commits came back as the sub-task's work and the file the sub-task had
/// actually staged in its worktree was **omitted**, at exit 0, on a verb whose
/// `--format json` shape is 1.0-pinned. The envelope's keys do not move here; the values
/// become true.
fn run_diff(cwd: &Path, id: &str, format: Format) -> Result<()> {
    let task = TaskArea::resolve(cwd, id)?;
    let base = task.base()?;

    let code_diff = git_diff(&task.code_checkout(), &base.sha)?;
    // The working area's `docs/<type>:<slug>.md` set, stripped to the `<type>:<slug>`
    // identity exactly as `dropped_staged_docs` derives the `task-discard` ack's list —
    // and that identity *is* the address `jigc doc show <id> --task <id>` takes, which
    // is why the envelope carries one key and never the bodies.
    let staged: Vec<render::StagedDoc> = task
        .staged_docs()?
        .into_iter()
        .map(|(name, body)| render::StagedDoc {
            id: name.strip_suffix(".md").unwrap_or(&name).to_string(),
            body,
        })
        .collect();

    let view = render::TaskDiffView {
        task: id,
        base: &base,
        code_diff: &code_diff,
        staged: &staged,
    };
    print!("{}", render::task_diff(format, &view));
    if format == Format::Json {
        println!();
    }
    Ok(())
}

/// `jigc task validate <id>` — run the task-scope sweep and render its findings; the
/// exit code tracks `has_blocking()` (`design/validation.md` → How it gates
/// `finalize`: validate previews what finalize blocks on) — a blocking report exits
/// [`EXIT_VALIDATION_BLOCKED`], an operational error 1. The findings render
/// through `crate::render::validation` in the selected format.
///
/// The sweep runs with the previewable finalize-time gates **on**
/// ([`GatePreview::On`]; M47 Inc 4, Settle Decision 1) — so a state the committing
/// door refuses is reported here rather than discovered there. `carry_staged` is the
/// preview's half of finalize's consent flag: with it declared, the carryover gate is
/// omitted at both doors, so a driver that always intends to carry reads a preview of
/// *its own* finalize instead of a permanently-red one.
///
/// **The repository posture is previewed first, and separately** (M52 Increment 3 / T6;
/// `completions/artifacts/M52/settle-record.md` → D2.6 as amended by §4). Under an
/// operation git has left un-concluded — a merge, a rebase, a half-finished pick — or a
/// detached HEAD, `finalize` refuses outright before it reaches this phase at all, and
/// through M52 Increment 2 the preview reported the task's content findings over that
/// state and exited 0 or 3 while the door it forecasts would not act. So it asks
/// [`crate::cli::finalize_posture_refusal`] — the **committing** door's own producer,
/// with that door's own exemptions — and renders the identical finding at the identical
/// exit code. It is **not** folded into [`TaskArea::preview_gates`]: the family is a fact
/// about the repository rather than about the task's content, and the separate
/// invocation is what [`crate::gate_coverage::Invocation::SeparatelyAtDoor`] states on
/// every surface that carries the coverage claim.
fn run_validate(cwd: &Path, id: &str, format: Format, carry_staged: bool) -> Outcome {
    // The one pre-commit phase a caller can resolve BEFORE finalizing, asked here and
    // answered by the committing door's own producer (M52 Increment 3 / T6) — against the
    // subject that door's own seam will have (M53, the rc.20 review `(2, A2-2)`).
    if let Some(refusal) =
        crate::cli::finalize_posture_refusal(cwd, crate::milestone::is_sub_task(cwd, id), format)
    {
        return refusal;
    }
    // …and, for a milestone sub-task, the posture of the checkout ITS boundary commits
    // from (M53 post-review fix). `jigc milestone finalize` reads the sub-task's own
    // worktree index, so a preview that asked only about the cwd answered *validates
    // clean* from the main checkout for a state that door refuses. Inert for every task
    // that is not a sub-task, and for a sub-task validated from inside its own worktree,
    // where the call above has already answered.
    if let Some(refusal) = crate::milestone::sub_task_fan_out_refusal(cwd, id, format) {
        return refusal;
    }
    let task = match TaskArea::resolve(cwd, id) {
        Ok(task) => task,
        Err(err) => {
            return crate::invocation_log::operational_failure(format, &err);
        }
    };
    // The post-sweep record is dropped: a standalone `validate` is a pure reader —
    // the durable baseline advances only at a landed `finalize`
    // (`design/reconciliation.md` → Persistence of the shifted baseline).
    match task.validate(GatePreview::On { carry_staged }) {
        Ok((report, _record)) => {
            print!("{}", render::validation(format, &report));
            if format != Format::Json {
                println!();
            }
            let code = if report.has_blocking() {
                EXIT_VALIDATION_BLOCKED
            } else {
                0
            };
            Outcome::with_findings(code, &report.findings)
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// The task-scope sweep's findings for one live task, run for the **orientation** door
/// (`crate::orient`; M50 → the Settle, D3).
///
/// One sweep, one answer: this is the same [`TaskArea::validate`] `jigc task validate`
/// drives, with the previewable finalize-time gates **on**, so what bare `jigc start`
/// reports about a task and what the route it prints beside it reports cannot diverge.
/// The post-sweep record is dropped — orientation is a pure reader of the baseline, which
/// advances only at a landed `finalize` (`design/reconciliation.md` → Persistence of the
/// shifted baseline).
///
/// **The error is a reason, not a failure.** The sweep can genuinely fail to run — a
/// `JIGC_DOC_CODE_PROBE` override naming no file over an anchored task, an unreadable
/// working area — and the bootstrap door must still name the task. The whole
/// context chain is returned as the string the caller carries as
/// `findings_unavailable`, so *unknown* is never rendered as *clean*.
pub(crate) fn sweep_for_orientation(cwd: &Path, id: &str) -> std::result::Result<Findings, String> {
    let task = TaskArea::resolve(cwd, id).map_err(|err| format!("{err:#}"))?;
    task.validate(GatePreview::On {
        carry_staged: false,
    })
    .map(|(report, _record)| report.findings)
    .map_err(|err| format!("{err:#}"))
}

/// `jigc task discard <id>` — remove the task's working area, abandoning it
/// (`design/write-commands.md` → Lifecycle: abandon), and ack the removal
/// (`design/command-output-contract.md` §2 → The task-state verbs join the envelope).
///
/// **Not idempotent** — an already-absent area rejects at [`TaskArea::resolve`] (exit 1,
/// the task-list route), which runs *before* any removal could. The prior claim of
/// idempotence here was false, and it stood behind a `discarded` effect key on the ack
/// that could only ever have held the constant `true`; the distinction it wanted —
/// *removed* vs *was already gone* — already lives one layer up, in **exit 0 + this ack**
/// vs **exit 1 + the error envelope** (`design/command-output-contract.md` §2 → the
/// ⚠ correction).
///
/// **A milestone sub-task settles its committed record here** (M49 Increment 2 / T3;
/// `design/team-ready-state.md` → The lifecycle). This door was workbench-local and knew
/// nothing of milestones, so abandoning a sub-task removed its area at exit 0 while the
/// committed record went on calling it `active` — the team-ready record lying about
/// abandoned work. So it delegates to the record-only-door seam
/// ([`crate::milestone::settle_discarded_sub_task`]), which is inert for an ordinary task
/// and dev-only, and it runs **before** the removal: a rejected commit must leave the task
/// exactly as this door's rejection frame says it did.
///
/// The captured non-blocking hook stream is **relayed** (`design/command-output-contract.md`
/// → Stream discipline) rather than added to the ack envelope: the pre-1.0 additive-key
/// window is closed (M48), and the relay delivers the hook's words on both surfaces without
/// minting new contract shape.
///
/// **The settled record commit's sha rides the ack** (M51 Increment 9 / T8). This is a
/// [`COMMITTING_DOORS`](crate::invocation_log::COMMITTING_DOORS) member whose success line
/// named no commit, while `setup`, `milestone create` and `migrate-corpus` all name theirs —
/// so the two cells of this one door, *settled a record* and *removed a workbench directory*,
/// printed the identical sentence. The seam returns the sha it landed and both surfaces carry
/// it; an ordinary task names none, because there is none.
///
/// **It refuses over the task's own staged docs unless `force`** (M50 → the Settle, D1;
/// `design/team-ready-state.md` → The lifecycle). `jigc uninstall` has refused over exactly
/// these bytes since M48 while this door removed them at exit 0 — two destroying doors
/// answering opposite ways about the identical files, with the refusing one's route pointing
/// at the destroying one. Both now ask the **one** probe
/// ([`staged_task_prose`], scoped to this task's area), so they cannot disagree.
///
/// **It also refuses over bytes jigc did not write there** ([`refuse_over_foreign_bytes`],
/// M52 Increment 4 / T5) — the working area's other population — and that guard is asked
/// **first**, because the mint stages the commit doc and a guard asked after the staged-prose
/// one could never fire at this door. `--force` is the single consent for both.
///
/// **Placement is load-bearing** and the order is `resolve` → guards → settle → remove:
/// [`TaskArea::resolve`] carries Increment 1's malformed-id refusal and stays outermost, and
/// the guard must sit **before** [`crate::milestone::settle_discarded_sub_task`], which
/// *commits* the milestone record — a refusal after it would leave a committed record
/// settling a sub-task whose working area survives, which is the lying-record defect M49
/// closed, re-opened by the fix for it.
fn run_discard(
    cwd: &Path,
    id: &str,
    format: Format,
    force: bool,
    conflicts: &mut Vec<Finding>,
) -> Result<()> {
    let task = TaskArea::resolve(cwd, id)?;
    refuse_over_foreign_bytes(&task, id, force)?;
    refuse_over_staged_prose(&task, id, force)?;
    let dropped = dropped_staged_docs(&task);
    let settled = crate::milestone::settle_discarded_sub_task(
        &task.jigc_home,
        &task.jigc_root,
        id,
        &task.dir,
        conflicts,
    )?;
    // Absent settle ⇒ no commit at all (an ordinary task, or a dev-only project): the ack
    // names no sha, which is the fact, not a withheld value.
    let commit = settled.as_ref().and_then(|s| s.commit.clone());
    let hook_output = settled.map(|s| s.hook_output).unwrap_or_default();
    // Read what the removal would take of the OTHER population before it runs, and name what
    // it actually took afterwards ([`PendingForeign`], law 1). Un-forced this is empty by
    // construction — the guard above returned — so the capture costs the ordinary discard one
    // `read_dir` of an area it is about to delete anyway.
    let pending = pending_foreign(&task.jigc_home, &[(task.dir.clone(), AreaKind::Task)]);
    let removed = std::fs::remove_dir_all(&task.dir);
    pending.narrate_taken(&task.jigc_home);
    removed.with_context(|| format!("could not discard task `{id}` at {:?}", task.dir))?;
    println!(
        "{}",
        render::task_ack(
            format,
            &render::TaskAck::Discarded {
                task: id.to_string(),
                dropped,
                commit,
            },
        )
    );
    relay_hook_output(format, &hook_output);
    Ok(())
}

/// The `task discard` door's half of the **survivable frame** (M47 Inc 3 T7;
/// `design/finalize.md` → 6. Commit) — a sub-task discard runs a hook-capable record-only
/// commit, so it is a [`COMMITTING_DOORS`](invocation_log::COMMITTING_DOORS) member and owes
/// its own state-truth clause, its own copy-runnable re-run, and its own error identity.
///
/// The clause is written to *this* door's truth: the record commit runs **before** the area
/// is removed and restores its captured pre-image on rejection, so both the record and the
/// task survive — which is what makes the re-run land what the rejected run would have.
///
/// The re-run **echoes the consent the run carried** (M50 Inc 3 / T2). Since the
/// staged-prose guard, a bare `jigc task discard <id>` refuses in exactly the state that
/// printed this frame — a sub-task whose record commit was rejected has a working area, and
/// a re-entered sub-task's area stages its commit doc — so a re-run line without `--force`
/// would be a route that cannot land what the rejected run would have.
fn discard_rejection_frame(id: &str, force: bool) -> RejectionFrame {
    RejectionFrame {
        code: crate::invocation_log::ERROR_TASK_DISCARD_REJECTED,
        target: work_unit_ref(id),
        survived: format!(
            "nothing was committed — the milestone record still names task:{id} as it did, \
             and the task's working area is intact"
        ),
        // `commit_record_transaction` rolls the record back on both cells of the axis, and the
        // area is removed only after the commit lands — so one clause is true of both.
        survived_non_hook: None,
        rerun: format!(
            "jigc task discard {}{}",
            shell_token(id),
            if force { " --force" } else { "" },
        ),
    }
}

/// The sorted `<type>:<slug>` identities staged in a working area's `docs/` dir — the
/// `docs/<type>:<slug>.md` set with the suffix stripped, which *is* the address
/// `jigc doc show <addr> --task <id>` takes.
///
/// **The name rule is [`engine::state::staged_doc_id`]'s**, shared with the writer
/// registry's `docs/` cell ([`engine::state::foreign_area_paths`]) so that the probe
/// which says *"this is staged prose"* and the probe which says *"jigc wrote this"*
/// cannot disagree about which names are identities (M52 Increment 4 / T2). The **shape**
/// question stays here and is deliberately the other one — see below.
///
/// **The `*.md` set, never directory-non-emptiness**: `docs/` also holds
/// `provenance.json`, so an emptiness probe over the directory answers "there is prose
/// here" for every task that ever existed.
///
/// **The name is not enough: the entry has to be a file** (M52 Increment 4 / T1, defect
/// L-3). This asked `strip_suffix(".md")` and no shape question at all, so a *directory*
/// named `<type>:<slug>.md` became a staged identity at every surface reading this probe.
/// Driven at `4572ca7c` with `mkdir '.jigc/tasks/<id>/docs/fake:thing.md'`: `jigc task
/// discard` refused naming `fake:thing` and routed at `jigc doc show fake:thing --task <id>`,
/// which dead-ends at `store.unknown-type`; the forced discard acked *"dropped staged edits
/// to: fake:thing"*; and `jigc doc list --task` — a 1.0-pinned contract — listed it
/// `managed`. Four surfaces claiming a staged doc that no doc read can open.
///
/// The shape question is asked through `std::fs::metadata`, which **follows** a symlink: the
/// id this yields *is* the address `jigc doc show` takes, and that read follows one too, so a
/// link to a real `.md` is a doc this can honestly name while a link to a directory is not.
/// What a non-file entry there *is* — a byte jigc did not write into a working area — is the
/// destroying doors' question about their own subject, not this probe's question about staged
/// prose, and a probe that answers the wrong one of those answers it wrongly.
///
/// `Ok(vec![])` for an absent dir; an unreadable *present* dir is an `Err`, so a caller
/// that must fail closed can ([`staged_task_prose`], the destroying doors' guard —
/// enumerating nothing must never be indistinguishable from finding nothing). An entry whose
/// own shape cannot be read is an `Err` for the same reason. Callers for whom the list is
/// decoration take `.unwrap_or_default()`.
pub(crate) fn staged_doc_ids(docs_dir: &Path) -> std::io::Result<Vec<String>> {
    if !docs_dir.exists() {
        return Ok(Vec::new());
    }
    let mut ids: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(docs_dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let Some(id) = engine::state::staged_doc_id(&name.to_string_lossy()).map(str::to_owned)
        else {
            continue;
        };
        if !std::fs::metadata(entry.path())?.is_file() {
            continue;
        }
        ids.push(id);
    }
    ids.sort();
    Ok(ids)
}

/// The **open tasks whose staged docs exist only in the workbench** — each task id paired
/// with the sorted `<type>:<slug>` identities staged in its `.jigc/tasks/<id>/docs/`.
/// [`crate::setup::uninstall`]'s second WIP guard, and the reproduced pre-1.0.0 loss:
/// `setup` → `start` → `doc set-slot` → `uninstall` removed `.jigc/` at exit 0 and the
/// authored summary was in no object DB (`DECISIONS.md` 2026-08-13 → the Settle, F3).
///
/// **One probe, one home, for every door that destroys a task area** (M50 → the Settle, D1).
/// It lives here rather than in `crate::setup` because its subject is a task's working
/// area, and two doors that answer about the same bytes from two copies of one probe are
/// two doors that will eventually disagree — which is exactly the contradiction `uninstall`
/// refusing over bytes `jigc task discard` destroyed in silence already was.
///
/// `only` scopes the enumeration to the task ids the caller's removal actually reaches —
/// `None` for a door that takes the whole workbench (`uninstall`), `Some(&[id])` for a door
/// that takes one area. A door must neither claim nor refuse over prose it will not touch,
/// so an out-of-scope area is skipped before it is probed at all: its readability is not
/// this call's business.
///
/// `unverified` is the **caller's** fail-closed refusal (M50 Inc 3 / T2). The probe measures
/// bytes and holds no opinion about which door asked, but a refusal names a door: the
/// install door's is [`unverified_prose_finding`], the task door's
/// [`discard_unverified_prose_finding`]. Minting one identity here for both would put
/// *"re-run `jigc uninstall`"* in front of an operator who ran `jigc task discard` — the
/// cross-door misdirection this pair exists to close (`design/surface-contract.md` → law 1),
/// reintroduced by the very sharing that closes it.
///
/// **It measures staging, not authorship.** A staged `*.md` is a doc no commit has a copy
/// of; whether a human or the mint wrote its bytes is not something this probe (or any
/// other) can tell, and the guard deliberately fires either way — the pristine skeleton
/// `jigc start` leaves behind is refused exactly like typed prose, because "the binary
/// cannot prove these bytes are disposable" is the whole basis of the refusal. What the
/// probe cannot distinguish, its findings must not claim
/// ([`crate::setup::staged_prose_finding`]).
///
/// **The subject is the staged `*.md` set** ([`staged_doc_ids`]), never
/// directory-non-emptiness — `docs/` always also holds `provenance.json` — and it does
/// **not** filter on the transient mark: the doc destroyed in the reproduced loss is the
/// task's `commit:<id>`, a transient doctype whose prose is exactly what the operator wrote.
///
/// Fail-closed like its sibling: a present-but-unreadable `tasks/` or `docs/` dir refuses
/// rather than reporting an empty set, because "enumerated nothing" and "there is nothing"
/// are the same bytes to the caller and only one of them is safe.
pub(crate) fn staged_task_prose(
    jigc_home: &Path,
    only: Option<&[String]>,
    unverified: &dyn Fn(std::io::Error) -> Finding,
) -> Result<Vec<(String, Vec<String>)>, Finding> {
    let tasks_root = jigc_home.join(".jigc").join("tasks");
    if !tasks_root.is_dir() {
        return Ok(Vec::new());
    }
    let mut staged = Vec::new();
    let entries = std::fs::read_dir(&tasks_root).map_err(unverified)?;
    for entry in entries {
        let entry = entry.map_err(unverified)?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let task = entry.file_name().to_string_lossy().into_owned();
        // Scope BEFORE probing: an area this door will not remove is neither named nor
        // refused over, and its readability is not this call's business either.
        if only.is_some_and(|only| !only.contains(&task)) {
            continue;
        }
        let docs = staged_doc_ids(&path.join("docs")).map_err(|err| {
            // Law 1's printed-path rule binds a blocking finding surface, and this fault is
            // one at three doors (M52 Increment 4 / T7, D-3). The subject is always under
            // `<repo>/.jigc/tasks/`, so a repo-relative spelling exists by construction.
            unverified(std::io::Error::other(format!(
                "{}: {err}",
                crate::render::repo_relative(jigc_home, &path.join("docs"))
            )))
        })?;
        if !docs.is_empty() {
            staged.push((task, docs));
        }
    }
    // Sorted by task id — the refusal's listing is byte-reproducible.
    staged.sort();
    Ok(staged)
}

/// The **install** door's fail-closed refusal: the staged set could not be enumerated, so
/// the teardown refuses rather than remove `.jigc/` with the staged docs' existence unknown.
/// Same code as the refusal it stands in for ([`crate::setup::staged_prose_finding`]) — the
/// operator's next action is identical, and the same claim discipline binds: the probe never
/// ran, so it can name only what it was looking for (staged docs), never what they contain.
///
/// Its sibling at the other destroying door is [`discard_unverified_prose_finding`]; the two
/// exist separately because a refusal names a door and this one names `jigc uninstall`.
pub(crate) fn unverified_prose_finding(err: std::io::Error) -> Finding {
    Finding::block(
        "uninstall.staged-prose",
        format!(
            "cannot check `.jigc/tasks/` for open tasks' staged docs, so removing `.jigc/` \
             could destroy work no commit has a copy of: {err}"
        ),
        "make sure `.jigc/tasks/` is readable, then re-run `jigc uninstall` — or, once you \
         have confirmed the open tasks hold nothing you need, `jigc uninstall --force` \
         deletes them with the install",
    )
}

/// The **task** door's foreign-byte code — its own, never a sibling door's, on the
/// [`DISCARD_STAGED_PROSE`] mold and for the same reason: the two other doors that refuse
/// over this subject destroy different things and their routes lead different ways.
///
/// A door refusal, not a probe result and not a commit-phase rejection, so it joins neither
/// `engine::result::CHECK_INVENTORY` nor [`crate::invocation_log::ERROR_CODE_REGISTRY`] —
/// exactly like the staged-prose sibling it pairs with. It is still **logged**: the refusal
/// travels as a `render::BlockedFinding`, so the identity the surface prints is the identity
/// the invocation log records.
///
/// **And on the declared arm.** M52 Increment 4 registers this code, `milestone.foreign-bytes`
/// and `uninstall.foreign-bytes` on the **findings arm** (`implementation/roadmap.md` →
/// Increment 4, *Codes it registers*), so both of this door's producers raise through
/// [`crate::render::envelope_finding_error`], never the flattened `{"error": …}` default.
/// Registering one family and then shipping two wire shapes of it is the defect the arm
/// declaration exists to prevent: a driver cannot key on a code that lives inside a message
/// (`design/command-output-contract.md` → The membership test), and the `uninstall` sibling
/// answered the envelope from the day it landed.
pub(crate) const DISCARD_FOREIGN_BYTES: &str = "task-discard.foreign-bytes";

/// The `jigc task discard` door's guard over the working area's **other** population: every
/// byte jigc did not write there (`engine::state::foreign_area_paths`), refused unless the
/// operator consented with `--force` (M52 Increment 4 / T5).
///
/// **Why it is asked BEFORE [`refuse_over_staged_prose`].** Every `jigc start` stages
/// `commit:<id>.md`, so the staged-prose subject is non-empty on essentially every task this
/// door is pointed at; asked second, this guard would be structurally unable to fire at its
/// own door — a guard that never runs. Asked first it costs the staged-prose refusal nothing
/// on any state whose complement is empty, and T2 drove the complement empty over a full
/// lifecycle (`crates/cli/tests/task_area_writer_registry.rs`), which is the ordinary path.
///
/// **The subject is exactly what the removal takes** — this task's area, the same path
/// `run_discard` hands `remove_dir_all` — so the door cannot refuse over bytes it would not
/// destroy, nor destroy bytes it did not check (G-15).
fn refuse_over_foreign_bytes(task: &TaskArea, id: &str, force: bool) -> Result<()> {
    if force {
        return Ok(());
    }
    let found =
        foreign_areas(&task.jigc_home, &[(task.dir.clone(), AreaKind::Task)]).map_err(|err| {
            crate::render::envelope_finding_error(&discard_unverified_foreign_finding(id, err))
        })?;
    let lines = foreign_lines(&found);
    if lines.is_empty() {
        return Ok(());
    }
    Err(crate::render::envelope_finding_error(
        &discard_foreign_bytes_finding(id, &lines),
    ))
}

/// Render a foreign-byte listing as the indented block every refusal in this family prints —
/// one home, so the three doors' refusals cannot drift into three spellings of one listing.
pub(crate) fn foreign_listing(lines: &[String]) -> String {
    lines
        .iter()
        .map(|line| format!("  {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The task door's foreign-byte refusal: a blocking, route-bearing finding naming the task
/// and every path in its working area that jigc did not write.
///
/// **It claims "jigc did not write these", not "you authored them"** — the same claim
/// discipline the staged-prose sibling carries. What the walk measured is membership of
/// jigc's own writer registry; who put the bytes there, and whether they matter, is not
/// something any probe here can tell, so the refusal states the one fact that makes them
/// worth stopping for: `.jigc/` is gitignored whole, so nothing else has a copy.
///
/// **A foreign directory is named, not walked.** It is the unit the removal takes and the
/// unit the displacing doors move, so naming the entry keeps every surface in this family
/// talking about the same thing.
fn discard_foreign_bytes_finding(id: &str, lines: &[String]) -> Finding {
    Finding::block(
        DISCARD_FOREIGN_BYTES,
        format!(
            "task `{id}`'s working area holds {} path(s) jigc did not write — `.jigc/` is \
             gitignored, so discarding the task would destroy bytes nothing else has a copy \
             of:\n{}",
            lines.len(),
            foreign_listing(lines),
        ),
        format!(
            "move what you need out of `.jigc/tasks/{id}/`, or delete what you do not, then \
             re-run `jigc task discard {id}` — or, once you have confirmed they hold nothing \
             you need, `jigc task discard {id} --force` removes the working area with them"
        ),
    )
}

/// The fail-closed half of [`refuse_over_foreign_bytes`], keyed to this door: the complement
/// could not be enumerated, so the area is not removed with those bytes' existence unknown.
fn discard_unverified_foreign_finding(id: &str, err: std::io::Error) -> Finding {
    Finding::block(
        DISCARD_FOREIGN_BYTES,
        format!(
            "cannot check task `{id}`'s working area for files jigc did not write, so \
             discarding it could destroy bytes nothing else has a copy of: {err}"
        ),
        format!(
            "make sure `.jigc/tasks/{id}/` is readable, then re-run `jigc task discard {id}` \
             — or, once you have confirmed the task holds nothing you need, `jigc task \
             discard {id} --force` removes the working area unchecked"
        ),
    )
}

/// The **task** door's staged-prose code — its own, never the install door's
/// (`uninstall.staged-prose`). The two doors destroy different things and their routes lead
/// different ways, so one code for both would name the wrong subject at one of them
/// ([surface-contract.md](../../design/surface-contract.md) → law 1); the per-door precedent
/// is the shipped `uninstall.dirty-worktree` / `milestone.dirty-worktree` pair, and the
/// `task-discard.` namespace is the one [`crate::invocation_log::ERROR_TASK_DISCARD_REJECTED`]
/// already uses.
///
/// It is a **door refusal**, not a probe result and not a commit-phase rejection, so it joins
/// neither `engine::result::CHECK_INVENTORY` nor
/// [`crate::invocation_log::ERROR_CODE_REGISTRY`] — exactly like the sibling it pairs with.
pub(crate) const DISCARD_STAGED_PROSE: &str = "task-discard.staged-prose";

/// The `jigc task discard` door's WIP guard: refuse while the working area holds staged docs
/// no commit has a copy of, unless the operator consented with `--force`
/// (M50 → the Settle, D1; `design/write-commands.md` → Abandoning a task).
///
/// **The subject is staging, never authorship.** [`staged_task_prose`] cannot tell prose
/// someone typed from the pristine skeleton `jigc start` mints — and the skeleton is the
/// *dominant* cell, since the mint itself stages `commit:<id>.md` — so this refuses over both
/// on the one ground it can prove: the bytes exist nowhere else. That is the shipped, doc-of-
/// record posture of the install door's identical guard (`design/project-setup.md` → the two
/// states `uninstall` refuses), and applying it here is what makes the pair agree.
///
/// **Scoped to this task's area.** A door must neither claim nor refuse over prose it will
/// not touch, so a sibling task's staged docs are none of this door's business — which is
/// why the probe carries the scope rather than the caller filtering afterwards.
fn refuse_over_staged_prose(task: &TaskArea, id: &str, force: bool) -> Result<()> {
    if force {
        return Ok(());
    }
    let only = [id.to_string()];
    let staged = staged_task_prose(&task.jigc_home, Some(&only), &|err| {
        discard_unverified_prose_finding(id, err)
    })
    .map_err(|finding| crate::render::finding_error(&finding))?;
    let docs: Vec<String> = staged.into_iter().flat_map(|(_, docs)| docs).collect();
    if docs.is_empty() {
        return Ok(());
    }
    // The sub-task discriminator, asked at the finding's CONSTRUCTION SITE (M52 Increment 4 /
    // T7, D-1): the route this finding composes is only true of a task the per-task finalize
    // will accept, and `engine::milestone::owning_milestone` is the same membership answer
    // the `finalize.milestone-sub-task` guard itself keys on.
    let owner = engine::milestone::owning_milestone(&task.jigc_root, id);
    Err(crate::render::finding_error(&discard_staged_prose_finding(
        id,
        &docs,
        owner.as_deref(),
    )))
}

/// The task door's staged-doc refusal: a blocking, route-bearing finding naming the task and
/// the doc identities its area stages.
///
/// **It claims "staged doc(s)", not "authored prose"** — the claim discipline its install-door
/// sibling ([`crate::setup::staged_prose_finding`]) already carries: that the docs are staged
/// and in no commit is exactly what the probe measured, and whether a human or the mint wrote
/// their bytes is not something it can tell.
///
/// **The route names the reachable exits in the order they are reachable.** The listed
/// identities are the addresses `jigc doc show <addr> --task <id>` takes, so reading what you
/// are about to lose is followable as printed; `jigc task finalize` follows with the condition
/// that makes it available (over the dominant skeleton cell it cannot succeed — the empty doc
/// fails `schema-conformance`); and the consent lands last, carrying the real id.
///
/// **`owner` is the sub-task discriminator** (M52 Increment 4 / T7, D-1). Over a milestone
/// sub-task the landing exit above is a **dead end**: `jigc task finalize <sub>` answers
/// `finalize.milestone-sub-task` at exit 3, because the parent milestone's boundary is the
/// only one that commits it. So on that branch the route names the boundary that does accept
/// it and the consent that does work there, and it states the consent's **record
/// side-effect** — `jigc task discard <sub> --force` settles that sub-task as `discarded` in
/// the committed milestone record, which the ack prints and the route previously left for the
/// operator to discover after the fact.
fn discard_staged_prose_finding(id: &str, docs: &[String], owner: Option<&str>) -> Finding {
    let exits = match owner {
        Some(milestone) => format!(
            "or land them with `jigc milestone finalize {milestone}` — this task is a \
             sub-task of milestone `{milestone}`, whose boundary is the only one that \
             commits it, so `jigc task finalize` refuses it — or, once you have confirmed \
             the task holds nothing you need, `jigc task discard {id} --force` removes the \
             working area with them and settles this sub-task as `discarded` in the \
             committed milestone record"
        ),
        None => format!(
            "or land them with `jigc task finalize {id}` (which refuses while a required \
             slot is empty) — or, once you have confirmed the task holds nothing you need, \
             `jigc task discard {id} --force` removes the working area with them"
        ),
    };
    Finding::block(
        DISCARD_STAGED_PROSE,
        format!(
            "task `{id}` stages {} doc(s) that no commit has a copy of — discarding it would \
             destroy them: {}",
            docs.len(),
            docs.join(", "),
        ),
        format!("read what is in them with `jigc doc show <address> --task {id}`, {exits}"),
    )
}

/// The task door's fail-closed refusal — the [`unverified_prose_finding`] sibling, keyed to
/// *this* door. The staged set could not be enumerated, so the area is not removed with its
/// staged docs' existence unknown: "enumerated nothing" and "there is nothing" are the same
/// bytes to a caller about to `remove_dir_all`, and only one of them is safe.
///
/// Same claim discipline: the probe never ran, so it names only what it was looking for.
fn discard_unverified_prose_finding(id: &str, err: std::io::Error) -> Finding {
    Finding::block(
        DISCARD_STAGED_PROSE,
        format!(
            "cannot check task `{id}`'s working area for staged docs, so discarding it could \
             destroy work no commit has a copy of: {err}"
        ),
        format!(
            "make sure `.jigc/tasks/{id}/` is readable, then re-run `jigc task discard {id}` \
             — or, once you have confirmed the task holds nothing you need, `jigc task \
             discard {id} --force` removes the working area unchecked"
        ),
    )
}

/// Enumerate the staged docs a `task discard` is about to throw away — the working
/// area's `docs/<type>:<slug>.md` set, as sorted `<type>:<slug>` identities — so the
/// ack can state what the removal dropped (B3, 2026-07-17 surface review; the style
/// guide's ack rule: "what a discard threw away"). Read **before** `remove_dir_all`.
///
/// The transient bit rides the resolved schema (no `location:`/`placement:` — the
/// commit doc): its staged copy was never going to land as a file, so the ack marks
/// it rather than implying committed content was lost. Best-effort by design: an
/// unreadable `docs/` dir yields the empty list and a schema-load failure just
/// leaves the marks off — enumeration must never block the discard itself.
fn dropped_staged_docs(task: &TaskArea) -> Vec<render::DroppedStaged> {
    let ids = staged_doc_ids(&task.dir.join("docs")).unwrap_or_default();
    let schemas = task.schemas().unwrap_or_default();
    ids.into_iter()
        .map(|doc| {
            let ty = doc.split(':').next().unwrap_or("");
            let transient = schemas
                .get(ty)
                .is_some_and(|s| s.location.is_none() && s.placement.is_none());
            render::DroppedStaged { doc, transient }
        })
        .collect()
}

/// `jigc task bind <role> <addr> <id>` — bind an already-committed doc to one of the
/// task's declared context roles (`design/write-commands.md` → Binding a context
/// role: the five-step enforcement; `workflow-dialect.md` → `reads`).
///
/// Five-step enforcement: (1) the task `<id>` resolves to a live working area
/// (`TaskArea::resolve` bails with the task-list route otherwise); (2) `<role>`
/// is one of the task's workflow-declared `reads` roles, else reject listing the
/// declared roles; (3) `<addr>` resolves in the committed store — its canonical
/// `<location>/<slug>.md` exists at HEAD — else the blocking `store.not-found`
/// [`no_committed_doc`] raises; (4) the
/// target's doctype (the address's `<type>`) equals the role's declared `type`,
/// else reject with the mismatch; (5) record the binding in the task's
/// `roles.json` (last-write-wins). The agent supplies only the which-doc choice;
/// recording the binding stays the CLI's (the determinism boundary holds).
///
/// The landed binding is acked (`design/command-output-contract.md` §2 → The task-state
/// verbs join the envelope): the ack's `target` is stamped from the **parsed** address
/// [`TaskArea::bind`] hands back, never the raw `addr` argument — the raw string may be a
/// bare singleton address, which the parse normalizes to the `<type>:<slug>` URI form.
fn run_bind(cwd: &Path, role: &str, addr: &str, id: &str, format: Format) -> Result<()> {
    let task = TaskArea::resolve(cwd, id)?;
    let address = task.bind(role, addr)?;
    println!(
        "{}",
        render::task_ack(
            format,
            &render::TaskAck::Bound {
                task: id.to_string(),
                role: role.to_string(),
                address: format!("{}:{}", address.r#type.as_str(), address.slug.as_str()),
                target: render::AckTarget {
                    doctype: address.r#type.as_str().to_string(),
                    slug: address.slug.as_str().to_string(),
                    section: None,
                    item: None,
                    leaf: None,
                },
            },
        )
    );
    Ok(())
}

/// The commit doctype name — the task's workflow-provisioned doc whose sink is the
/// git message (`design/finalize.md` → Commit-doc rendering). The commit instance is
/// provisioned under `commit:<task-id>`, so the commit slug is the task id.
pub(crate) const COMMIT_TYPE: &str = "commit";

/// The changelog doctype name — the create-gate the
/// [`CHANGELOG_GATE_CODE`] check keys on (`design/validation.md` → The changelog-gate
/// advisory). A `singleton`, so its slug is fixed to the type id.
const CHANGELOG_TYPE: &str = "changelog";

/// The step whose composed body **states** the migration review hold — *"a plain
/// finalize commits NOTHING … and holds (exit 4)"*. Every migrate-shaped workflow of
/// both shipped packs includes it, and so may a project-layer one.
///
/// It is the review hold's subject because it is the sentence the hold has to keep
/// ([`TaskArea::composes_review_hold`]; `design/auto-migration.md` → The review gate).
const MIGRATION_FINALIZE_STEP: &str = "migration-finalize";

/// The step whose composed body states the **path-scoped doc-only commit** (M55;
/// `design/findings-channel.md` §3): a task whose recorded composed workflow includes it
/// commits exactly its own docs and recorded owner-artifacts, and every other staged path
/// stays staged. Keyed on the composed contract, on [`MIGRATION_FINALIZE_STEP`]'s mold.
const DOC_ONLY_FINALIZE_STEP: &str = "finalize-doc-only";

/// Whether a composed workflow promises the doc-only commit — its own `includes` name
/// [`DOC_ONLY_FINALIZE_STEP`]. The **one** predicate every door asks of a definition, so
/// the arm a committing door takes and the arm a preview describes cannot be decided by two
/// different readings of the same workflow.
pub(crate) fn composes_doc_only_finalize(def: &WorkflowDef) -> bool {
    def.includes
        .iter()
        .any(|step| step == DOC_ONLY_FINALIZE_STEP)
}

/// The changelog-gate advisory's finding code — one source for the finding it grades,
/// the cascade key its route names as the not-user-facing exit, and the severity probe
/// that decides which route that is ([`TaskArea::changelog_gate_refuses`]).
const CHANGELOG_GATE_CODE: &str = "changelog-recording.gate-granted-unused";

/// Which **door** the changelog-gate advisory speaks for — the [`CarryoverBoundary`]
/// mold (one decision, two doors, and the *route* differs because the verbs that run
/// differ). Since M46 Inc 6 / T3 the advisory is reported at both doors, and a route
/// naming an in-task verb at a door where the working area is already gone is not a
/// hint but a dead end: `jigc doc create changelog --task <id>` printed after a landed
/// finalize answers ``no task `<id>` `` at exit 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AdvisoryDoor {
    /// `jigc task validate` — the preview. The working area is open by construction
    /// here, so the in-task write verbs run and the route leads with them.
    TaskPreview,
    /// `jigc task finalize` — the committing door. When the advisory is **not**
    /// promoted, this invocation lands the commit and deletes the working area, so the
    /// route names only the landed-state form.
    Finalize,
}

/// **`jigc task bind` named a role this task's workflow does not declare** (M52 completion
/// audit, fix 5). The door's own mint — no other verb enforces a `reads:` declaration — on
/// the `task-discard.*` namespace precedent, the door-verb family a refusal takes when the
/// state is the door's own. Keyed at the **work unit** (`task:<id>`), the
/// [command-output-contract](../../../design/command-output-contract.md) target form whose
/// subject is a task rather than a document: the role is a property of *this* task's
/// workflow, and the address the caller passed is not the thing that is wrong.
const UNDECLARED_READ_ROLE: &str = "task-bind.undeclared-role";

/// **The bound address's doctype is not the one the role declares** (M52 completion audit,
/// fix 5) — step 4 of the bind enforcement, coded with its step-2 sibling above and keyed at
/// the same work unit for the same reason.
const ROLE_TYPE_MISMATCH: &str = "task-bind.role-type-mismatch";

/// The **stale-commit-summary** advisory's finding code (M51 Increment 11 / T2;
/// `design/validation.md` → The M51 registrations — Increment 11; the rc.14 trial's
/// F-9). One spelling, read by both producers.
///
/// The namespace is the **subject's** topic family, per the shipped producing-door
/// rule: an advisory the task-scope sweep re-raises takes the family of the thing it
/// speaks about, exactly as [`CHANGELOG_GATE_CODE`] does. `write.*` is refused — that
/// family is the write-time address-reject taxonomy with its own declared route split
/// — and `rename.*` is refused too: that is the task-less top-level `jigc rename`
/// door's family, and the trial's F-4 records those two verbs already being confused.
const STALE_TITLE_CODE: &str = "commit-recording.stale-title";

/// **The staged commit summary still names a title this task renamed a doc away from**
/// — one shared predicate, two producers (M51 Increment 11 / T2; the rc.14 trial's
/// F-9, `completions/artifacts/M51/settle-record.md` → §21).
///
/// `jigc doc rename --task` moves a staged doc's `# H1`; the task's staged `commit`
/// doc may already carry the old one in its `summary` slot, which
/// [`engine::write::render_commit_message`] projects as the **commit subject** — which
/// is why [`engine::write::COMMIT_SECTION_SUMMARY`] is `pub` and read from there rather
/// than re-spelled here: two literals for one id is the *statement == constant* drift
/// this wave exists to close. Nothing
/// noticed: the trial's only adapter bypass was a worker that renamed at invocation 18,
/// finalized at 23, and rewrote jigc's commit with raw git because the product never
/// connected the two. The answer is a `Finding` re-raised **from durable state**
/// ([`state::RenameRecord`], written at the rename) rather than a notice printed once —
/// *an instrument fires reliably iff its trigger is a state and its consequence is
/// re-raised by the product* (`RC-1.0-gate/cue-card-postmortem.md`).
///
/// **Advisory, and un-keyed** — neither tier of the two-tier inventory rule, so the M6
/// severity post-pass leaves it alone (`design/validation.md` → Severity assignment). A
/// stale subject is a real cost and never a reason to refuse a commit: the prose is the
/// author's, and this reports it.
///
/// **One finding per task, never one per rename.** The subject is one stale summary and
/// the repair is one write, so the target is the summary's own address
/// (`commit:<task-id>#summary`) and the message names every recorded title still present,
/// sorted — a per-rename finding would key several findings at one address and hand the
/// agent the same repair three times.
///
/// **A recorded title another staged doc still carries is not stale.** Every correct
/// summary naming `Rate limiting` also names `Rate`, so a superstring rename (and the
/// rename-back cell, `A → B → A`) would otherwise mint a finding whose own route,
/// followed exactly, changes nothing — M46's PT-1 defect. The live-title clause is what
/// keeps the check silenceable by the thing it prints.
///
/// `None` — never an error — for every absence: a task that renamed nothing, no staged
/// commit doc, a commit doc that does not parse (its conformance findings speak for it,
/// and a second diagnosis over the same bytes would report one fault twice), or a
/// `commit` schema with no `summary` slot.
pub(crate) fn stale_commit_summary_finding(
    task_dir: &Path,
    task_id: &str,
    commit_schema: &Schema,
) -> Result<Option<Finding>> {
    let recorded = state::RenameRecord::load(task_dir)
        .with_context(|| format!("could not read the rename record of task `{task_id}`"))?
        .pre_rename_titles;
    if recorded.is_empty() {
        return Ok(None);
    }

    let commit_path = state::instance_path(task_dir, COMMIT_TYPE, task_id);
    let source = match std::fs::read_to_string(&commit_path) {
        Ok(source) => source,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => {
            return Err(err)
                .with_context(|| format!("reading the staged commit doc at {commit_path:?}"));
        }
    };
    let Ok(document) = engine::parse::parse_sections(commit_schema, &source) else {
        return Ok(None);
    };
    let Some(span) = document
        .sections
        .iter()
        .find(|section| section.id == engine::write::COMMIT_SECTION_SUMMARY)
        .and_then(|section| section.slot.as_ref())
    else {
        return Ok(None);
    };
    let summary = span.slice(&source);

    let live = live_staged_titles(task_dir, task_id)?;
    let mut stale: Vec<&str> = recorded
        .iter()
        .map(String::as_str)
        .filter(|title| {
            summary.contains(*title) && !live.iter().any(|current| current.contains(*title))
        })
        .collect();
    stale.sort_unstable();
    stale.dedup();
    if stale.is_empty() {
        return Ok(None);
    }

    let address = format!(
        "{COMMIT_TYPE}:{task_id}#{}",
        engine::write::COMMIT_SECTION_SUMMARY
    );
    let named = stale
        .iter()
        .map(|title| format!("`{title}`"))
        .collect::<Vec<_>>()
        .join(", ");
    let moved = if stale.len() == 1 {
        "a title this task renamed a doc away from"
    } else {
        "titles this task renamed docs away from"
    };
    Ok(Some(Finding::graded(
        Severity::Advisory,
        STALE_TITLE_CODE,
        format!(
            "the staged commit summary still names {named} — {moved}, so the commit \
             subject would land naming a title no staged doc carries",
        ),
        Some(Location::addressed(address.clone(), span.start_line, 1)),
        Some(engine::finding::Route::mechanical(
            [
                "jigc",
                "doc",
                "set-slot",
                &address,
                "--task",
                task_id,
                "--from-file",
                "-",
            ],
            " to re-write the summary against the titles the docs now carry",
        )),
    )))
}

/// The `# H1` every **other** staged doc in the task area currently carries — the
/// live-title set [`stale_commit_summary_finding`] measures a recorded title against.
///
/// The commit doc itself is excluded: it is the finding's *subject*, not an explainer,
/// and its `# H1` is the machine-derived task id rather than an authored title.
/// A staged file with no `# H1` contributes nothing.
fn live_staged_titles(task_dir: &Path, task_id: &str) -> Result<Vec<String>> {
    // The staged-docs directory is read off the instance path rather than re-spelled: the
    // commit instance lives in it, so its parent IS the home, and there is no second
    // literal to drift from `engine::state`'s own.
    let commit_file = state::instance_path(task_dir, COMMIT_TYPE, task_id);
    let Some(docs) = commit_file.parent().map(Path::to_path_buf) else {
        return Ok(Vec::new());
    };
    let entries = match std::fs::read_dir(&docs) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(err).with_context(|| format!("reading the staged docs at {docs:?}"));
        }
    };
    let mut titles = Vec::new();
    for entry in entries {
        let path = entry
            .with_context(|| format!("reading the staged docs at {docs:?}"))?
            .path();
        if path == commit_file || path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        if let Some(title) = crate::rename::read_h1(&source) {
            titles.push(title.to_string());
        }
    }
    Ok(titles)
}

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
fn run_finalize(
    cwd: &Path,
    id: &str,
    format: Format,
    approve: bool,
    dry_run: bool,
    carry_staged: bool,
) -> Outcome {
    let task = match TaskArea::resolve(cwd, id) {
        Ok(task) => task,
        Err(err) => {
            return crate::invocation_log::operational_failure(format, &err);
        }
    };
    // **The posture, against this task's own seam subject** (M53, the rc.20 per-axis review
    // `(2, A2-2)`). `Cli::dispatch`'s guard has already adjudicated the cwd with the
    // classifier's subject and let this door through, which in a provisioned fan-out worktree
    // means it let a detached HEAD through — and `SeamSubject::live` then refused it deep
    // inside the transaction, after the promote/retire/stage phases had run and rolled back,
    // while `--dry-run` forecast a clean manifest. Asked here it is the same finding at the
    // same exit from the same producer, one phase earlier and on both arms, so the forecast
    // and the act agree and the door refuses before it writes anything. Inert everywhere the
    // dispatch guard already answered: outside a fan-out worktree the two subjects are the
    // same value.
    if let Some(refusal) =
        crate::cli::finalize_posture_refusal(cwd, crate::milestone::is_sub_task(cwd, id), format)
    {
        return refusal;
    }
    match task.finalize(id, format, approve, dry_run, carry_staged) {
        Ok(outcome) => outcome,
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// The CLI side of the `doc-code` probe seam (`engine::validate::ProbeInvoker`): run the
/// resolved `doc-code` program over the engine-built [`ProbeRequest`] and report the raw
/// [`ProbeRun`] (`design/validation.md` → Architecture — the CLI owns the subprocess
/// invoker; the engine stays shell-free). The engine enumerated the surface, materialized
/// the snapshot, and built the request; this serializes it to the child's stdin, enforces
/// the wall-clock budget via [`crate::invoke::invoke_probe`], and maps the raw outcome
/// into the engine type the engine ingests.
///
/// A **spawn failure** (the program is missing / not executable) maps to
/// [`ProbeRunStatus::CouldNotStart`], carrying the program it tried and the io error, so a
/// misconfigured probe surfaces a blocking `pack-probe-integrity.crash` meta-finding that
/// says *could not start `<program>`* rather than silently passing or aborting the whole
/// validate — an unresolvable invocation is never a clean run (M54 Inc 2 T2; the program
/// named since the M54 audit, which completes S4). When the program itself cannot be
/// resolved — the running `jigc`'s own image path is unreadable — there is no program to
/// name, and the error says so. A serialization failure of the engine-built request is the
/// only `Err` raised (an internal fault, not a probe outcome).
pub(crate) fn doc_code_invoker(request: &ProbeRequest) -> std::io::Result<ProbeRun> {
    let bytes = serde_json::to_vec(request)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    let (program, args) = match crate::invoke::doc_code_command() {
        Ok(command) => command,
        Err(err) => {
            return Ok(could_not_start(
                None,
                format!("the running `jigc` image's path is unreadable: {err}"),
            ));
        }
    };
    match crate::invoke::invoke_probe(&program, &args, &bytes, crate::invoke::DOC_CODE_BUDGET) {
        Ok(outcome) => Ok(ProbeRun {
            stdout: outcome.stdout,
            stderr: outcome.stderr,
            status: match outcome.status {
                crate::invoke::ProbeStatus::Exited { code } => ProbeRunStatus::Exited { code },
                crate::invoke::ProbeStatus::TimedOut => ProbeRunStatus::TimedOut,
            },
        }),
        // The program could not be spawned (absent / not executable) — a crash candidate,
        // not an orchestration error: the engine synthesizes the blocking meta-finding,
        // and its message names the program and carries this error.
        Err(err) => Ok(could_not_start(
            Some(program.display().to_string()),
            err.to_string(),
        )),
    }
}

/// The [`ProbeRun`] of a probe that never ran: no output, and why it could not start.
fn could_not_start(program: Option<String>, error: String) -> ProbeRun {
    ProbeRun {
        stdout: Vec::new(),
        stderr: Vec::new(),
        status: ProbeRunStatus::CouldNotStart { program, error },
    }
}

/// The converged roster route a **top-level** task's absence carries — `jigc task
/// <verb> <id>` ([`TaskArea::resolve`]), `jigc start --task <id>`
/// (`crate::start::resume_in_repo`) and `jigc doc <verb> … --task <id>`
/// (`crate::doc`'s active-task resolution) all hand [`require_task_area`] the one factual
/// recovery, `jigc task list` (`DECISIONS.md` 2026-07-16 M43 Settle, cross-cutting:
/// wrong-id routes converge — the old `start --task` route claimed `jigc start` lists live
/// tasks, which it never did). The route span goes through the checked
/// [`engine::finding::Route::mechanical`] constructor, so the CLI-seam parse fence asserts
/// it parses against the real CLI.
///
/// The fourth seam, `jigc workflow <W> --task <id>`, is a milestone sub-agent's re-entry
/// and passes its **own** roster instead; nothing else does.
///
/// The **no-active-task** state (no `--task` given, no task exists) is a different state
/// with a correct start-route and does not converge here.
///
/// It is a function rather than a `const` because `Route::mechanical` runs the parse fence
/// at construction. It was `no_such_task`'s body until M53 Increment 3 / T3 folded that
/// wrapper into [`require_task_area`], which answers two cells rather than one.
pub(crate) fn task_list_route() -> engine::finding::Route {
    engine::finding::Route::mechanical(["jigc", "task", "list"], " lists the live tasks")
}

/// **Resolve a task working area by id — the one predicate every by-id door asks**
/// (M53 Increment 3 / T3; `completions/artifacts/M53/settle-record.md` → D3.2 as amended
/// by §9, the second of the residual rule's three homes).
///
/// Four seams used to ask `!dir.is_dir()` in four copies — [`TaskArea::resolve`],
/// `crate::start::resume_in_repo`, `crate::start::reenter_in_repo` and
/// `crate::doc::ActiveTask::resolve` — and a directory was therefore a task because it was
/// a directory. Driven at `2e491967` over a bare `mkdir .jigc/tasks/leftover`, the four
/// disagreed four ways: `jigc task validate` reported *"the task validates clean"* at exit
/// **0**, `jigc task discard` composed `task-discard.foreign-bytes` (a refusal about bytes
/// *in a task*), `jigc doc show … --task` answered `store.not-staged` (a statement about a
/// task's *staged set*), and `jigc start --task` fell through to a code-less
/// `could not read the base pin for task …`. Each door had already accepted the directory
/// as a work unit and was answering a later question about it.
///
/// **Two answers, and the enumerator's own rule decides between them**
/// ([`engine::state::carries_base_pin`], the predicate `engine::state::list_active_task_ids`
/// took at T2): a directory that is not there is the shipped absence, whose recovery is the
/// caller's own roster and therefore arrives as `absent_route`; a directory that is there
/// but carries no base pin is a **leftover**, whose recovery is one act at every door and is
/// composed by the producer itself
/// ([`engine::finalize::residual_task_area_finding`]). The **malformed** id is a third
/// answer and is refused earlier still, at each door's own
/// [`reject_malformed_work_unit_id`] call, before any token joins a path (M50 Inc 1 / T1).
///
/// **The pin is read for existence, never parsed**, so a *legitimate* area whose pin is
/// torn, corrupt or unreadable passes this and still fails at its own read — which is why
/// §9 keeps those three producers rather than deleting them. What this closes is the
/// residual's path to them, not the window they exist for.
///
/// **`jigc task discard` is not an exception.** A residual is a task at no door, so the
/// teardown door answers exactly as the other sixteen do — and the false-flip path M53
/// Increment 2's settled-item guard watches at the *record* is thereby closed one layer
/// earlier, at resolution, before the door ever looks for an item to settle.
///
/// **Both answers are typed [`engine::finding::Finding`]s, not `anyhow!` strings** (M51
/// Increment 6 / T1, carried forward). The contract lists `finalize.no-task` under the
/// **work-unit** target form — i.e. as a finding a driver may key on — and before M51 it
/// projected no key at any of these doors: `--format json` answered the flattened
/// `{"error": …}`, whose code lives inside a message. Both identities are minted in the
/// engine (the producer `cli::render::FINALIZE_FAMILY` declares for this code) and travel
/// on the envelope-projecting carrier, so every door that raises either answers the findings
/// envelope through the one funnel.
///
/// Takes **jigc_home** — the main checkout the `.jigc/` workbench binds to — so a door
/// called from a fanned-out worktree resolves the shared area and renders its path
/// repo-relative rather than host-absolute (M31 Inc 2 / WF3; `design/surface-contract.md`
/// law 1).
pub(crate) fn require_task_area(
    jigc_home: &Path,
    id: &str,
    absent_route: engine::finding::Route,
) -> Result<PathBuf> {
    let dir = jigc_home.join(".jigc").join("tasks").join(id);
    if !dir.is_dir() {
        return Err(crate::render::envelope_finding_error(
            &engine::finalize::no_such_task_finding(id, absent_route),
        ));
    }
    if !engine::state::carries_base_pin(&dir, engine::state::WorkArea::Task) {
        return Err(crate::render::envelope_finding_error(
            &engine::finalize::residual_task_area_finding(id, jigc_home, &dir),
        ));
    }
    Ok(dir)
}

/// The **grammar** every work-unit id obeys — the one sentence the three `--slug` doors
/// already state (`crate::start`'s mint, `crate::doc::reject_malformed_slug`,
/// `crate::migrate`'s adoption), reused verbatim so a fourth spelling of *"what an id
/// is"* never enters the surface (`design/surface-contract.md` → law 1).
///
/// Since M50 Increment 2 it also carries [`reject_malformed_slug_head`]'s route: a work-unit
/// id and a doc slug obey the **same** grammar (both are `engine::slug::is_slug`), so the
/// second family reuses the literal rather than spelling it again. The name still says
/// which family minted it; it moves — it is not copied — the day a third family needs it
/// (`DECISIONS.md` → 2026-09-05 M50 Increment 2 planning, declared bound iii).
pub(crate) const WORK_UNIT_ID_GRAMMAR: &str =
    "use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)";

/// The blocking finding code every malformed work-unit id carries — **one** code for the
/// whole family, task doors and milestone doors alike, because it is one fault.
pub(crate) const MALFORMED_WORK_UNIT_ID: &str = "work-unit.malformed-id";

/// Refuse a caller-supplied work-unit id that is not a well-formed slug — the guard at
/// every **resolve seam**, below clap and above the filesystem op
/// (`completions/artifacts/M50/settle-record.md` → D2).
///
/// `.jigc/tasks/<id>/` and `.jigc/milestones/<id>/` are built by joining a caller token
/// onto a path, and until M50 no door asked whether that token was an id at all: driven,
/// `jigc task discard "../.."` resolved the *repository root* as a working area and
/// `remove_dir_all`'d it — `.git` included — at **exit 0**. So membership is decided
/// where the token becomes a path component, never on a remembered list of shapes.
///
/// **Not a clap `value_parser`**: clap rejects with exit 2 carrying no code and no route,
/// which is the M43 route-floor breach at every door that takes an id. This produces a
/// [`Finding`] instead, flattened through the shared [`crate::render::finding_error`]
/// carrier, so the refusal has an identity the surface prints and the log can record.
///
/// The predicate is [`engine::slug::is_slug`] — the **recognition** rule, the one every
/// frozen id read back from disk is checked against. `slugify(x) == x` would be the wrong
/// test: the mint-time word cap and edge-stopword drop make it reject ids the mint itself
/// produced.
///
/// The route **states the grammar** and deliberately does *not* name `jigc task list`: a
/// malformed token was never an id, so routing to the roster would be a law-1
/// misdirection. A *well-formed but unknown* id keeps that route ([`no_such_task`]).
pub(crate) fn reject_malformed_work_unit_id(id: &str) -> Result<()> {
    if engine::slug::is_slug(id) {
        return Ok(());
    }
    Err(crate::render::finding_error(&Finding::graded(
        Severity::Blocking,
        MALFORMED_WORK_UNIT_ID,
        format!("{id:?} is not a valid work-unit id"),
        None,
        Some(engine::finding::Route::human(WORK_UNIT_ID_GRAMMAR)),
    )))
}

/// The blocking finding code every malformed **address slug head** carries — **one** code
/// for the whole family, the read doors and the write doors alike, because it is one
/// fault: the `<slug>` half of a `<type>:<slug>` address is what names the file, and a
/// token that is not a slug names no doc anywhere.
pub(crate) const MALFORMED_SLUG_HEAD: &str = "store.malformed-slug";

/// Refuse a caller-typed address whose `<slug>` head is not a well-formed slug — the guard
/// at the **four** user-address parse boundaries (`crate::doc::parse_verb_addr`,
/// [`TaskArea::bind`]'s own parse, `crate::rename::parse_addr`, and
/// `crate::milestone::run_add_from_spec`), covering the ten `DoctypeArg::Address` doors
/// (`completions/artifacts/M50/settle-record.md` → D2, family 4; `DECISIONS.md` →
/// 2026-09-05 M50 Increment 2 planning).
///
/// The fourth boundary joined at the M50 completion audit: the door set was derived from a
/// hand-kept allowlist of clap **argument names**, and `jigc milestone add-from-spec` takes
/// its address through `spec_addr` — a name nobody had listed — so it carried no
/// `DOCTYPE_DOORS` row and no guard. Driven, its `<slug>` head reached
/// `engine::store::canonical_path` and **read a file outside the repository** at exit 0.
/// The registry is now derived from `crate::cli::ARG_TOKENS`, a *total* classification of
/// every clap argument, so an unlisted name cannot hide a door again.
///
/// A doc's slug *is* its path component — `<docs-root>/<location>/<slug>.md` — and
/// [`engine::address`] splits on `:` / `#` / `/` and sanitizes nothing, so until M50 the
/// slug reached that join with no door asking whether it was a slug. Driven at
/// `ce5b015`: `jigc doc show "research:<absolute path>" --format json` served a file from
/// **outside the repository** through the 1.0-pinned read contract at exit 0, and
/// `jigc rename 'research:../../src/planted' --to "Captured Doc"` committed an arbitrary
/// source file into the docs root.
///
/// **Not inside `engine::address::Address::parse`**, and that is measured rather than
/// preferred: `jigc doc schema --format json` advertises **type-level** addresses whose
/// instance parts are placeheld (`changelog:<slug>#…`), and a shipped law-1 fence asserts
/// every advertised address parses — so the engine grammar must stay placeholder-tolerant.
/// The guard belongs at the verb boundary, which is where a human or an agent types an
/// address.
///
/// The refusal is the [`reject_malformed_work_unit_id`] shape one family over: a blocking
/// [`Finding`] through the shared [`crate::render::finding_error`] carrier, naming the
/// token the caller typed, and **one** route — the family's discovery verb `jigc doc list`
/// (never `jigc describe`, which lists doctypes and answers nothing about a slug) plus the
/// grammar in [`WORK_UNIT_ID_GRAMMAR`]'s one shipped spelling. Reused, not copied: a
/// second spelling of *what a slug is* is exactly the drift the surface contract forbids.
///
/// **Two faults, one boundary** (M52 Increment 6 / T5). Since the name-ceiling guard rides
/// beside the grammar reject ([`reject_slug_head_over_name_ceiling`]), this function is the
/// four boundaries' whole *is this head usable* door: the grammar fault
/// ([`MALFORMED_SLUG_HEAD`]) first, because a token that is not a slug is not a question
/// about length; the ceiling fault ([`SLUG_NAME_CEILING_CODE`]) second. Both are facts
/// about the **token**, so both precede the doctype question a resolved schema answers
/// ([`reject_fixed_identity_alias`]) — the order every caller inherits by calling this.
pub(crate) fn reject_malformed_slug_head(addr: &str, slug: &str) -> Result<()> {
    if !engine::slug::is_slug(slug) {
        return Err(crate::render::finding_error(&Finding::graded(
            Severity::Blocking,
            MALFORMED_SLUG_HEAD,
            format!("{slug:?} is not a valid doc slug — the `<slug>` head of address `{addr}`"),
            None,
            Some(engine::finding::Route::mechanical(
                ["jigc", "doc", "list"],
                format!(
                    " lists the committed docs and the identity each one carries; \
                     {WORK_UNIT_ID_GRAMMAR}"
                ),
            )),
        )));
    }
    reject_slug_head_over_name_ceiling(addr, slug)
}

/// Refuse a caller-typed address whose `<slug>` head is a well-formed slug the filesystem
/// cannot **name** — the ceiling family's second half, at the same four boundaries
/// [`reject_malformed_slug_head`] guards (M52 Increment 6 / T5;
/// `completions/artifacts/M52/settle-record.md` → D5.6).
///
/// **The state it refuses.** A head becomes one path component — `<slug>.md` committed,
/// `<type>:<slug>.md` staged in the task working area — which is exactly the path shape
/// [`crate::cli::SLUG_NAME_CEILING`] is derived for, so the head takes that constant and
/// that code rather than a second ceiling of its own. Driven at the wave's baseline
/// (`completions/artifacts/M52/baseline-tokens.md` §2.5 row 1), a 300-byte head at the five
/// `doc` write doors reached the copy-in and reported the discovery as I/O —
/// ``could not copy in `vision:aaaa…#thesis` for editing: File name too long (os error 63)``,
/// with no code, no `at:` and no route, and the same sentence inside `{"error": …}` under
/// `--format json`.
///
/// **Why it rides the boundary and not those five doors.** An over-long head is unusable
/// wherever it is typed: it resolves nothing at the read doors, names no file for `rename`
/// to move, and no spec for `milestone add-from-spec` to seed from. Guarding the five that
/// fail loudly would key the fix on today's symptom — the mistake
/// [`reject_slug_over_name_ceiling`]'s own *one flag, one ceiling, six doors* reading names
/// one family over. One edit here lands it at all four parse boundaries.
///
/// **It refuses rather than truncating**, for that sibling's reason: a truncated identity
/// names a different doc.
///
/// The two halves of the family differ only in locus and route, and each is right for its
/// own caller. The flag half carries **no** location — the fault is the value, and five of
/// its six doors have no doc, task or path to key it at — and routes at re-running with a
/// shorter `--slug`. This half carries the address the caller typed as its locus, because
/// that address *is* where the fault is, and routes at the family's discovery verb
/// `jigc doc list`, the same one [`reject_malformed_slug_head`] names.
fn reject_slug_head_over_name_ceiling(addr: &str, slug: &str) -> Result<()> {
    let ceiling = crate::cli::SLUG_NAME_CEILING;
    if slug.len() <= ceiling {
        return Ok(());
    }
    Err(crate::render::finding_error(&Finding::graded(
        Severity::Blocking,
        SLUG_NAME_CEILING_CODE,
        format!(
            "the `<slug>` head of address `{addr}` is {} bytes — over the {ceiling}-byte \
             ceiling a doc identity carries",
            slug.len(),
        ),
        Some(Location::addressed(addr, 1, 1)),
        Some(engine::finding::Route::mechanical(
            ["jigc", "doc", "list"],
            format!(
                " lists the committed docs and the identity each one carries; a slug is one \
                 filesystem path component, so an identity is at most {ceiling} bytes — jigc \
                 refuses rather than truncating, because a truncated identity names a \
                 different doc"
            ),
        )),
    )))
}

/// The blocking finding code an address carries whose `<slug>` head is **not the identity
/// its doctype has** — M52 Increment 6 / T2 (settle-record → D5.2; `design/validation.md`
/// → The M52 registrations). **One** code for the family on [`MALFORMED_SLUG_HEAD`]'s
/// reading: it is one fault at every door, and which verb the caller typed does not change
/// what they have to do about it.
///
/// Deliberately **not** `store.not-found`, the answer this address used to get at the two
/// read doors: *not found* is a fact about the corpus and invites the caller to create the
/// thing, while a fixed-identity doctype's slug is **supplied by the CLI**, so the address
/// names nothing that could ever exist and creating it is not an act jigc offers. And
/// deliberately not `store.malformed-slug`, M50's *this token is not a slug*: `bogus` **is**
/// a slug — the grammar has nothing to say about it.
pub(crate) const FIXED_IDENTITY: &str = "store.fixed-identity";

/// Refuse a caller-typed address whose `<slug>` head is not the one identity a
/// **fixed-identity** doctype has ([`engine::schema::Schema::has_fixed_identity`] — the
/// predicate's one engine home, M52 Increment 6 / T1).
///
/// **The state it refuses, driven at `cbf7d833` on the shipped packs.** `jigc doc set-field
/// vision:alpha#meta/grounded-in --value "[research:x]" --task <id>` exited **0** and minted
/// `.jigc/tasks/<id>/docs/vision:alpha.md` — a staged instance at an identity the store
/// cannot hold — while `jigc doc list` carried no `alpha` row and `jigc doc show
/// vision:alpha` refused. Taken end to end on the released `1.0.0-rc.15` over the sibling
/// `roadmap:bogus` (`completions/artifacts/M52/advocates/F5.md` §1) it **finalized and
/// committed**: the real `docs/roadmap.md` rewritten, `jigc validate` clean at exit 0, and
/// the pinned write ack handing the driver `target.slug: "bogus"` with `findings: []`.
/// Between that write and the finalize there is **no address at which the agent can read
/// what it wrote**, so M48's read-back fence is defeated structurally rather than by
/// omission.
///
/// Returned as a bare [`Finding`] rather than an error, so each door declares its own arm:
/// the nine `doc` doors wrap it in `DocFailure::block` (the findings envelope), and the
/// three sibling doors that resolve their schema separately carry their own.
///
/// **The route is built from the schema alone** — `jigc doc show <ty>:<ty>`, the identity
/// the doctype does have. It carries no `--task`: this guard sits at the parse boundary,
/// *before* any door selects a copy, and a `--task` it does not hold would be a guess —
/// which is the law-1 lie this wave closes, not a convenience.
pub(crate) fn reject_fixed_identity_alias(
    schema: &Schema,
    address: &Address,
) -> Result<(), Finding> {
    reject_fixed_identity_slug(schema, &address.to_string(), address.slug.as_str())
}

/// [`reject_fixed_identity_alias`]'s core, addressed by `<slug>` rather than by a parsed
/// [`Address`] — the form `jigc rename` needs (M52 Increment 6 / T3).
///
/// That door splits its target with its own local parser into a `(ty, slug)` pair and
/// never builds an [`Address`], so an `Address`-only entry point would make it re-parse a
/// string it has already validated — a fallible call for an infallible fact, and a second
/// place the address could be spelled. `typed` is what the refusal is **about**: the
/// address as the door holds it, so the locus and the `(code, target)` key name that.
///
/// One message, one route, one home: the two entry points differ only in how the caller
/// already holds the address, never in what the refusal says.
pub(crate) fn reject_fixed_identity_slug(
    schema: &Schema,
    typed: &str,
    slug: &str,
) -> Result<(), Finding> {
    if !schema.has_fixed_identity() || slug == schema.ty {
        return Ok(());
    }
    let canonical = format!("{ty}:{ty}", ty = schema.ty);
    Err(Finding::graded(
        Severity::Blocking,
        FIXED_IDENTITY,
        format!(
            "`{typed}` is not an address `{ty}` can have: its identity is fixed — the CLI \
             supplies the slug, so `{canonical}` is the one instance this doctype has",
            ty = schema.ty,
        ),
        Some(Location::addressed(typed, 1, 1)),
        Some(engine::finding::Route::mechanical(
            ["jigc", "doc", "show", canonical.as_str()],
            " — a fixed-identity doctype has one instance at a fixed slug",
        )),
    ))
}

/// The blocking finding `jigc task bind` raises when its address names no **committed**
/// doc — step 3b of the five-step enforcement (`design/write-commands.md` → Binding a
/// context role), which shipped as a bare ``no such doc `<addr>` `` bail: no code, no
/// locus, no route, and a flattened `{"error": …}` on `--format json` (M52 Increment 10 /
/// T7; per-axis-review axis-6 §4 lead 2; settle-record D13 lead 6a).
///
/// **The code is reused, not minted.** `store.not-found` is what every read door already
/// raises for this exact fact, and it is a code the contract lists under a declared target
/// form — so a driver is promised its `(code, target)` key resolves.
///
/// **The arm is selection, not a decision taken here.** `design/command-output-contract.md`
/// → *Which reject arm a run takes, in one rule* (M52 Increment 1): *a reject that carries
/// a finding takes the findings arm; a reject with no finding behind it takes `{error}`*.
/// This refusal carries one, so it takes that arm; the posture that admits the move from
/// the flattened shape this door used to emit is the still-open pre-pin window, declared at
/// that doc's *Evolution posture* — cited, not re-decided.
///
/// The message is this door's rather than the read doors': what a bind resolves is the
/// committed store, so the sentence names that rule instead of inviting the caller to
/// create the doc. `canonical` is `None` for a transient doctype, which has no committed
/// home to name at all — the one cell where the *expected at* clause would be a guess.
fn no_committed_doc(jigc_home: &Path, address: &Address, canonical: Option<&Path>) -> Finding {
    let typed = address.to_string();
    let expected = canonical
        .map(|path| {
            format!(
                " (expected at `{}`)",
                crate::render::repo_relative(jigc_home, path)
            )
        })
        .unwrap_or_default();
    Finding::graded(
        Severity::Blocking,
        engine::store::NOT_FOUND,
        format!(
            "no committed doc `{typed}` to bind{expected} — a bind resolves in the committed \
             store, and a doc that is only staged in an open task is not bindable"
        ),
        Some(Location::addressed(&typed, 1, 1)),
        Some(engine::finding::Route::mechanical(
            crate::doc::doctype_argv("list", address.r#type.as_str()),
            " — the committed docs this doctype has",
        )),
    )
}

/// The blocking finding code a caller token longer than the OS name ceiling carries —
/// **one** code for the whole **name-ceiling** family, on [`MALFORMED_SLUG_HEAD`]'s
/// reading: it is one fault, and which door the caller typed does not change what they have
/// to do about it.
///
/// Two halves, one code, because one thing is wrong in both: a token jigc would turn into a
/// single filesystem path component is longer than one can be. The `--slug` **override**,
/// at every [`crate::cli::SLUG_DOORS`] row (M51 Increment 9 / T3, EC-28;
/// [`reject_slug_over_name_ceiling`]), and the `<slug>` **head** of a caller-typed address,
/// at the four user-address parse boundaries (M52 Increment 6 / T5;
/// [`reject_slug_head_over_name_ceiling`]) — which differ only in locus and route, each
/// stated at its own predicate.
///
/// Deliberately **not** `write.malformed-slug`, M50's *"this token is not a slug"*: the
/// override here **is** a slug — the grammar has nothing to say about it — it is simply too
/// long to be a filename. Answering with the grammar code would send the caller to re-read a
/// rule their value already obeys.
pub(crate) const SLUG_NAME_CEILING_CODE: &str = "write.slug-name-ceiling";

/// Refuse a `--slug` override longer than [`crate::cli::SLUG_NAME_CEILING`] — the predicate
/// every [`crate::cli::SLUG_DOORS`] row runs, beside the grammar reject it cannot answer
/// for.
///
/// **The state it refuses.** A `--slug` value drives a minted identity **verbatim**, and
/// jigc turns that identity into a single filesystem path component — so a value the OS
/// cannot name is a value no door can mint from. Driven at `422032b6` with a 300-byte slug,
/// three of the six doors discovered that at the `write`/`create_dir_all`, after the door
/// had accepted the value:
///
///   * `jigc start … --slug <300>` and `jigc doc create adr --slug <300>` blocked with
///     `task.working-area-io` — *"File name too long (os error 63)"* — routed at *"resolve
///     the underlying I/O condition (a disk or permissions problem on the `.jigc/` task
///     working area)"*. **There is no disk or permissions problem**, and no act that route
///     names moves the caller forward: a law-1 lie
///     (`design/surface-contract.md` → law 1, nothing lies).
///   * `jigc doc rename adr:keeper --to Y --slug <300>` printed the OS error bare — no code,
///     no route, no `at:` (`completions/artifacts/M51/baseline-tokens.md` → row 12).
///
/// **Why it runs at every row and not at those three.** `jigc migrate … --slug <300>` exited
/// **0** in the same probe, the override inert because its target is a singleton, and
/// `jigc doc add-item --slug <300>` landed a 300-byte `{#…}` anchor, an item id being no
/// filename. Guarding the three that fail loudly would key the fix on today's *symptom*
/// rather than on the rule, and an override that is inert at one door today is an identity
/// at that door tomorrow. One flag, one ceiling, six doors.
///
/// **It refuses rather than truncating.** A truncated identity is a different doc — the same
/// reason [`reject_malformed_slug_head`] never re-slugifies, one family over.
pub(crate) fn reject_slug_over_name_ceiling(slug: &str) -> Result<()> {
    let ceiling = crate::cli::SLUG_NAME_CEILING;
    if slug.len() <= ceiling {
        return Ok(());
    }
    Err(crate::render::finding_error(&Finding::graded(
        Severity::Blocking,
        SLUG_NAME_CEILING_CODE,
        format!(
            "`--slug {slug:?}` is {} bytes — over the {ceiling}-byte ceiling a minted id \
             carries",
            slug.len(),
        ),
        None,
        Some(engine::finding::Route::human(format!(
            "re-run with a `--slug` of at most {ceiling} bytes — jigc refuses rather than \
             truncating, because a truncated id names a different doc. The ceiling is the \
             filesystem's per-component limit less what jigc wraps around an id when it \
             becomes a filename"
        ))),
    )))
}

/// Which finalize-time gates a [`TaskArea::validate`] sweep additionally **previews**
/// (M47 Inc 4, `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1).
///
/// The seam is a parameter and not a shared default precisely because
/// [`TaskArea::validate`] has two callers: the read door (`jigc task validate`) wants
/// the preview, while `finalize`'s preflight must **not** take it — the committing path
/// runs each of these gates itself at its own position in the phase order
/// (`design/finalize.md` → 5. Stage), and previewing them in the preflight would move
/// finalize's block position, which is the one thing this change may not do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GatePreview {
    /// No preview — the caller runs these gates itself, in position. `finalize`'s
    /// preflight passes this, so its observable behaviour is unchanged.
    Off,
    /// Preview them at the read door. `carry_staged` mirrors finalize's consent flag:
    /// a declared carry-over is a finding at neither door.
    On { carry_staged: bool },
    /// The `--dry-run` **forecast** (M51 Inc 5 / T5, EC-20): the previewable gates the
    /// committing preflight has not already run for itself. It is [`Self::On`] minus the
    /// changelog advisory — and only that — because `finalize` computes that one at its
    /// own position and merged it into the report the forecast is extending, so running
    /// it here would emit the member twice.
    ///
    /// Sharing the seam rather than re-listing the members is the point: a fifth member
    /// joining [`TaskArea::preview_gates`] reaches the forecast the day it lands, which is
    /// the drift that put `--dry-run` a whole advisory tier behind `task validate` in the
    /// first place.
    Forecast { carry_staged: bool },
}

/// A named task's working area. `repo_root` is the **worktree** (code, the git index,
/// HEAD — every `git` shell-out routes here); `jigc_root` (and the doc-store base
/// `jigc_home`) bind to **jigc_home**, the main checkout, so all worktrees of one
/// project share a single `.jigc/` (M31 Inc 2 / WF3). Outside a worktree the two
/// coincide.
struct TaskArea {
    /// The task id this area belongs to — the substitution source for every route a
    /// finding emits about *this* task (M47 inc-2 / T4: the conflict route names the real
    /// id, never a `<task-id>` placeholder the engine cannot fill).
    id: String,
    repo_root: PathBuf,
    jigc_home: PathBuf,
    jigc_root: PathBuf,
    dir: PathBuf,
    pack: Box<dyn PackSource>,
}

impl TaskArea {
    /// Resolve the task `id`'s working area from `cwd`. The repo root is the nearest
    /// `.git` ancestor (the worktree); jigc_home is the main checkout, and the task dir is
    /// `<jigc_home>/.jigc/tasks/<id>/`.
    ///
    /// **Three answers, in this order** (`design/structural-grammar.md` → Work-units and
    /// runtime identity, resolution): a **malformed** `id` is not a wrong id but *not an
    /// id*, and is refused by [`reject_malformed_work_unit_id`] before it can become the
    /// path component below; then [`require_task_area`] — the one predicate all four by-id
    /// seams share — answers a **well-formed but absent** id with the converged task-list
    /// route, and a directory that carries no base pin as the **leftover** it is (M53
    /// Increment 3 / T3).
    fn resolve(cwd: &Path, id: &str) -> Result<Self> {
        // Before anything joins `id` onto a path (M50 Inc 1 / T1).
        reject_malformed_work_unit_id(id)?;
        let repo_root = discover_repo_root(cwd).ok_or_else(|| crate::locate::not_in_repo(cwd))?;
        let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
        let jigc_root = jigc_home.join(".jigc");
        let dir = require_task_area(&jigc_home, id, task_list_route())?;
        Ok(Self {
            id: id.to_string(),
            repo_root,
            jigc_home,
            jigc_root,
            dir,
            pack: make_pack()?,
        })
    }

    /// **The checkout that holds this task's code** — the subject of a read about what the
    /// task changed, which is not always the checkout the caller stands in (M53 — the cwd
    /// census, row C2-02).
    ///
    /// The rule has two cases and no third:
    ///
    /// * a **milestone sub-task** with a provisioned fan-out worktree — its code lives in
    ///   that worktree and nowhere else (M31 Inc 4/5: since the isolation split, the
    ///   provisioned worktree is the *sole* copy of a sub-agent's staged code), so the
    ///   worktree is the subject from every cwd, the repository root included;
    /// * **a plain task** — its code is authored in whatever checkout the agent is working
    ///   in, and [`crate::task::run_finalize`] commits it *there*, so `repo_root` is both
    ///   the honest subject and the one that agrees with the commit door.
    ///
    /// The sub-task leg is [`crate::repo::posture_subject`]'s discriminator, unchanged and
    /// unrestated: `.git` is a file, the path is `<jigc_home>/.jigc/worktrees/<id>`, and
    /// `<id>` is a sub-task of a milestone in this workbench. Asking it rather than
    /// re-deriving it is what keeps this read and the fan-out doors from classifying one
    /// directory two ways.
    fn code_checkout(&self) -> PathBuf {
        let worktree = self
            .jigc_home
            .join(engine::milestone::worktree_path(&self.id));
        if crate::repo::posture_subject(&worktree).is_dedicated() {
            worktree
        } else {
            self.repo_root.clone()
        }
    }

    /// **Where this door's commit will land, when that is not the checkout the `.jigc/`
    /// workbench binds to** — `None` on the ordinary path (M53 — the cwd census, row C2-09;
    /// [`crate::render::CommitSite`] carries the rule and the declared bound).
    ///
    /// The comparison itself lives on [`crate::render::CommitSite::differing`] — one producer
    /// shared with `jigc task amend`'s mint ack, which asks the same question about the same
    /// pair and answers it in its own tense (the F-10 review's LOW-7). Both paths are
    /// canonicalized there, because one of the two can carry macOS's `/private` prefix and
    /// the other not, the same reason [`crate::milestone::sub_task_fan_out_refusal`]
    /// canonicalizes its own pair.
    fn commit_site(&self) -> Option<crate::render::CommitSite> {
        crate::render::CommitSite::differing(&self.jigc_home, &self.repo_root)
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
    ///
    /// **Both fault arms name the pin repo-relative** (M53 Increment 3 / T5;
    /// `completions/artifacts/M53/settle-record.md` → §9; `design/surface-contract.md` →
    /// law 1: *every printed path is repo-real or a typed identity*). They spelled it
    /// `{path:?}` — the host path of the machine the door ran on — one line away from
    /// [`require_task_area`], which already renders the same area repo-relative.
    ///
    /// **The arms are kept, not deleted, and D3 is why they still matter.** The predicate
    /// every roster and every by-id door now asks is *existence, never a parse*
    /// ([`engine::state::carries_base_pin`]), so a **residual** no longer reaches here at
    /// all — but a *legitimate* area whose pin is torn, corrupt or unreadable passes that
    /// door and still fails this read, which is the exact window D3 argues exists.
    ///
    /// **The root is `jigc_home`, not `repo_root`.** The `.jigc/` workbench binds to the
    /// main checkout (M31 Inc 2 / WF3), so a door called from a fanned-out worktree would
    /// find no repo-relative spelling against the worktree's own root and fall back to the
    /// absolute — the very thing this renders away. It is also the root
    /// [`require_task_area`] renders the residual against, so one area is spelled one way.
    fn base(&self) -> Result<BasePin> {
        let path = self.dir.join("base.json");
        let named = crate::render::repo_relative(&self.jigc_home, &path);
        let bytes = std::fs::read(&path)
            .with_context(|| format!("could not read the base pin at `{named}`"))?;
        serde_json::from_slice(&bytes).with_context(|| format!("malformed base pin at `{named}`"))
    }

    /// Every **cascade-resolved** schema, keyed by doctype — the set the conformance
    /// sweep resolves staged instances against and the finalize promote destination is
    /// read from (the engine stays domain-empty; the CLI feeds the cascade in).
    ///
    /// Surface C — the finalize-promote **write** path, and the one that made the
    /// shadow-blind read destructive: read pack-only, `task validate` named the staged
    /// copy at the pack's home, finalize promoted it there, and the read surfaces then
    /// looked at the resolved home — `doc show` blocked `store.not-found` on a document
    /// that had just been committed, while `jigc validate` called that store clean
    /// (M49 Increment 3 T2).
    fn schemas(&self) -> Result<BTreeMap<String, Schema>> {
        crate::start::resolved_schemas(self.pack.as_ref(), &self.project_config())
    }

    /// The task-scope **probe pre-flight** — the task twin of `cli.rs`'s store-scope
    /// `require_doc_code_probe`. When the task's effective state carries `code-anchor`
    /// work (the same surface `schedule_doc_code` invokes the probe over) and the
    /// `JIGC_DOC_CODE_PROBE` override is set, require the override to be an existing file
    /// **before** the engine sweep reaches the probe. Without the override the probe is
    /// the running `jigc` itself (M54 S4), so there is nothing to check. A missing probe
    /// is one misconfiguration to report once — not an N-per-anchor
    /// `pack-probe-integrity.crash` floor — so this bails with **one**
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
        crate::invoke::require_doc_code_override()
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
    /// `task validate` drops the record — read verbs stay pure readers. Staged
    /// working-area instances never touch the record (M43 A14): a persisted instance
    /// is reported as an informational staged-copy advisory at its repo-real
    /// destination, a transient one yields no `file-state.*` finding at all, and no
    /// `docs/<type>:<slug>.md` key is ever minted.
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
    fn validate(
        &self,
        preview: GatePreview,
    ) -> Result<(engine::result::ValidationReport, FileStateRecord)> {
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
        let history = self.history_predicate();
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
        // The migration source this task is replacing, when it is a migration — the same
        // recorded path the carryover gate reads as its retire exemption.
        let migration_source = state::read_migration_source(&self.dir).with_context(|| {
            format!("could not read the source path for task at {:?}", self.dir)
        })?;
        // The managed-vs-foreign discriminator's three pack facts (M48 Inc 4 / T1): the
        // committed-store sweep this call drives must answer a foreign squatter with the
        // *store* door's code and route, and the engine produces none of them.
        let versions = crate::pack::frozen_doctype_versions(self.pack.as_ref());
        let priors = crate::pack::prior_doctype_schemas(self.pack.as_ref(), &versions);
        let migratable = crate::pack::migratable_doctypes(self.pack.as_ref());
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
            &history,
            &changed_code,
            base_tree.path(),
            // The conflict route is the caller's, and this caller is a named task: a
            // `DRIFTED + TOUCHED` block routes at the REAL id (M47 inc-2 / T4). A
            // **migration** task also holds the one path whose general route cannot be
            // followed — its own recorded source (M46 inc-5 / T2): discarding retires the
            // migration, and the revert the route's other exit names is the act the adapter
            // forbids over a managed doc, on a file that was already hand-broken out of
            // band. So the source path rides along and carries its own exit; every other
            // conflicting path, here and at a non-migration task, keeps the general one.
            &engine::file_state::ConflictBlock::task(
                &self.id,
                migration_source
                    .as_ref()
                    .map(state::MigrationSource::recorded),
            ),
            &engine::validate::AdoptionInputs::new(
                &versions,
                &priors,
                &migratable,
                // The adoption route's base: the committed store binds to `jigc_home`, so the
                // `jigc migrate` operand it emits is spelled against the same checkout the
                // `rel_key` beside it is relative to (M53 post-review-fix review, HIGH 2).
                &self.jigc_home,
            ),
            // The live-record carve-out is this caller's too (M52 Inc 10 / T6): only a task
            // door knows which milestone owns the task it is sweeping.
            &self.live_milestone_record(&schemas),
        )
        .with_context(|| format!("validating task at {:?}", self.dir))?;
        let report = self.preview_gates(report, preview, &schemas)?;
        // The stale-commit-summary advisory (M51 Inc 11 / T2 — the rc.14 trial's F-9).
        // Sited HERE, outside [`Self::preview_gates`] and ahead of the route scoping, for
        // two reasons that are decisions rather than placement: it is a **content**
        // finding, not a gate — `task validate` already promises this task's content
        // findings, so joining the previewed set would propagate a `GATE_COVERAGE` row to
        // all eight enumerating surfaces for something no boundary gate decides — and
        // being unconditional (no [`GatePreview`] arm) it reaches `task validate`, the
        // `--dry-run` forecast, the committing preflight and `start`'s orientation from
        // this one computation, which is what makes the advisory *re-raised* rather than
        // printed once.
        let report = match self.stale_commit_summary(&schemas)? {
            None => report,
            Some(finding) => {
                let mut findings = report.findings.into_vec();
                findings.push(finding);
                engine::result::ValidationReport::new(findings, &self.severity_cascade()?)
            }
        };
        let report = self.scope_repair_routes(report)?;
        Ok((report, record))
    }

    /// Scope every gate-block repair route in `report` to **this** task (M49 Increment 8 /
    /// T3). Both task-scope members of [`crate::render::BOUNDARY_DOORS`] reach this — `jigc
    /// task validate` and `finalize`'s preflight share the one [`Self::validate`] entry — and
    /// both were *given* the id they emit a route without: with a second task open, the
    /// emitted `jigc doc set-slot …` exited 1 on `more than one active task`, so the only
    /// stated exit from a blocked gate could not be run.
    ///
    /// The transform is [`crate::doc::scope_repair_route_to_task`], applied **after**
    /// [`Self::preview_gates`] so a previewed finalize-time gate is scoped identically to an
    /// engine one, and it is **door-scoped by position**: the store sweep (`jigc validate`)
    /// runs no task, resolves no id, and never passes through here, so its routes stay
    /// task-free exactly as they were.
    ///
    /// Rebuilt through [`engine::result::ValidationReport::new`] — the sanctioned
    /// re-construction its sibling merge already uses; the severity post-pass is idempotent
    /// and this pass touches routes only, so grading and order are unmoved.
    fn scope_repair_routes(
        &self,
        report: engine::result::ValidationReport,
    ) -> Result<engine::result::ValidationReport> {
        let mut findings = report.findings.into_vec();
        for finding in &mut findings {
            crate::doc::scope_repair_route_to_task(finding, &self.id);
        }
        Ok(engine::result::ValidationReport::new(
            findings,
            &self.severity_cascade()?,
        ))
    }

    /// This task's [`stale_commit_summary_finding`], resolved against the composed
    /// `commit` schema. `None` when the pack-set ships no `commit` doctype — the same
    /// *absent means inert* reading [`Self::changelog_gate_advisory`] takes for a
    /// pack-set with no `changelog`.
    fn stale_commit_summary(&self, schemas: &BTreeMap<String, Schema>) -> Result<Option<Finding>> {
        let Some(schema) = schemas.get(COMMIT_TYPE) else {
            return Ok(None);
        };
        stale_commit_summary_finding(&self.dir, &self.id, schema)
    }

    /// Merge the **previewable finalize-time gates** into a `task validate` report
    /// (M47 Inc 4, `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1). Inert under
    /// [`GatePreview::Off`], which is what `finalize`'s preflight passes — the
    /// committing path keeps running these gates itself, at their own position in the
    /// phase order (`design/finalize.md` → 5. Stage), so its exits, its rendered
    /// findings and its block order are untouched by this door existing.
    ///
    /// The first member is the **carryover** decision: a pure decision over
    /// CLI-supplied git facts ([`decide_carryover`]) — the mint-time staged snapshot,
    /// the same probe re-run now, the migration retire pathspec, and the recorded
    /// owner-artifact paths, all read-only. It stages nothing and mutates nothing, so
    /// the preview stays a pure reader; the owner exemption comes from the *same*
    /// [`engine::finalize::plan_owner_artifacts`] the planner feeds the committing
    /// door, so the two cannot disagree about which paths are the task's own subject.
    /// A snapshot-less (pre-M43) task yields nothing — the declared fail-open bound
    /// holds at this door exactly as it does at the committing one.
    ///
    /// `carry_staged` is finalize's consent flag, mirrored: a declared carry-over is
    /// not a finding at either door, so a driver that always intends to carry reads a
    /// preview of *its own* finalize rather than a permanently-red one (the
    /// cross-model review's scoping of the non-breaking argument).
    ///
    /// The second member is the **#5 owner-artifact gate**
    /// ([`owner_artifacts_gate`]) — run here under a **constant-true `tracked`
    /// predicate**, which is the whole construction. M45 Decision 6 relocated this gate
    /// off the shared phase-2 entry because a *pre-stage* `tracked` check false-positives
    /// a state phase-5 staging repairs, and that rationale is preserved exactly: with
    /// `tracked` pinned true the untracked branch of `owned_location_violation` is
    /// structurally unreachable from this door, so cause 7 stays post-stage. What the
    /// preview reaches is the other six causes — empty · absolute · `..` ·
    /// not-under-home · names-no-file · symlink-escape — none of which consults
    /// `tracked`, so no stage can change their verdict and previewing them cannot
    /// false-positive (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1: Decision 6
    /// narrowed to its own stated grounds, not overturned). The trial's two live blocks
    /// were causes 4 and 5. The gate reads the task's staged instances and the named
    /// paths' presence only — no git shell-out, no staging, no mutation.
    ///
    /// The merge goes through [`engine::result::ValidationReport::new`] with the
    /// resolved severity cascade — like the finalize-scope changelog advisory — so a
    /// project's severity delta applies to a previewed finding and the envelope stays
    /// one report.
    /// **The index gate of the amend arm**, as one producer both positions ask (F-10).
    ///
    /// Empty unless this area carries the `amend` marker: the ordinary commit model stages
    /// the agent's index deliberately, and only `--amend` rewrites `HEAD` **from** it.
    ///
    /// It is a method rather than two call-sites' worth of the same three git lines because
    /// it is asked at two positions — the committing door, ahead of `plan_finalize`, and the
    /// preview inside [`TaskArea::preview_gates`] — and the promise
    /// [`crate::gate_coverage::Door::Previewed`] makes is *same check, same severity, same
    /// exit code*. Two spellings of one check is how that promise stops being true.
    fn amend_index_findings(&self) -> Result<Vec<Finding>> {
        if state::read_amend_pin(&self.dir)
            .with_context(|| format!("could not read the amend marker for task `{}`", self.id))?
            .is_none()
        {
            return Ok(Vec::new());
        }
        let staged = git_capture(
            &self.repo_root,
            &["diff", "--cached", "--name-only", "HEAD"],
        )?;
        Ok(staged
            .lines()
            .map(str::trim)
            .filter(|path| !path.is_empty())
            .map(|path| amend_index_dirty_finding(path, &self.repo_root))
            .collect())
    }

    /// **The amend arm's staged-doc gate**, as one producer both positions ask (the F-10
    /// review's HIGH-1) — the direct sibling of [`Self::amend_index_findings`], and a
    /// method for the same reason: it is asked at the committing door ahead of
    /// `plan_finalize` **and** inside [`Self::preview_gates`], and what
    /// [`crate::gate_coverage::Door::Previewed`] promises is *same check, same severity,
    /// same exit code*. Two spellings of one check is how that promise stops being true.
    ///
    /// Empty unless this area carries the `amend` marker: every other commit model stages
    /// and commits its promotions, so a staged managed doc there is the ordinary case.
    ///
    /// The subject is [`Self::staged_docs`] — the one enumeration of *what this task
    /// staged* — filtered by [`engine::finalize::promote_destination`], the promote plan's
    /// own membership predicate. A staged file whose stem is not an address, or whose
    /// doctype this pack does not resolve, is skipped here exactly as `plan_promotions`
    /// skips it: it promotes nothing, so it diverges nothing, and the doors that own an
    /// unresolvable staged doc answer for it at their own positions.
    fn amend_staged_doc_findings(
        &self,
        schemas: &BTreeMap<String, Schema>,
    ) -> Result<Vec<Finding>> {
        if state::read_amend_pin(&self.dir)
            .with_context(|| format!("could not read the amend marker for task `{}`", self.id))?
            .is_none()
        {
            return Ok(Vec::new());
        }
        let mut findings = Vec::new();
        for (name, _) in self.staged_docs()? {
            let Some(address) = engine::state::staged_doc_id(&name) else {
                continue;
            };
            let Some((ty, slug)) = address.split_once(':') else {
                continue;
            };
            let Some(schema) = schemas.get(ty) else {
                continue;
            };
            let Some(home) = engine::finalize::promote_destination(schema, slug) else {
                continue;
            };
            findings.push(amend_staged_doc_finding(
                address,
                &home,
                AmendDocDoor::Finalize { task: &self.id },
            ));
        }
        Ok(findings)
    }

    fn preview_gates(
        &self,
        report: engine::result::ValidationReport,
        preview: GatePreview,
        schemas: &BTreeMap<String, Schema>,
    ) -> Result<engine::result::ValidationReport> {
        // `changelog` discriminates the two previewing doors: the read door owes the
        // advisory, the `--dry-run` forecast has it already (see [`GatePreview::Forecast`]).
        let (carry_staged, changelog) = match preview {
            GatePreview::Off => return Ok(report),
            GatePreview::On { carry_staged } => (carry_staged, true),
            GatePreview::Forecast { carry_staged } => (carry_staged, false),
        };
        let mut previewed = Vec::new();
        // **The `carryover` member's check is arm-dependent, and the preview asks the arm's
        // own** (F-10; `crate::gate_coverage::GATE_COVERAGE`'s `carryover` row). The member
        // is *the index gate* — `finalize.carried-staged` on the ordinary commit model,
        // `finalize.amend-index-dirty` on the amend model, which refuses the whole index
        // rather than the pre-task subset and takes no `--carry-staged`. Asking the wrong
        // one here would re-open the class M52 Increment 3 closed for posture: a preview
        // reading clean over a state its own committing door refuses at exit 3.
        //
        // The amend arm's other refusal, `finalize.base-mismatch`, deliberately does **not**
        // preview: the base pin is finalize-only by the M47 Settle, Decision 1, and the
        // amend form is that same member (the pin *is* the commit it rewrites).
        let amending = state::read_amend_pin(&self.dir)
            .with_context(|| format!("could not read the amend marker for task `{}`", self.id))?
            .is_some();
        if amending {
            previewed.extend(self.amend_index_findings()?);
            // **The arm's second gate previews beside its first** (the F-10 review's
            // HIGH-1). It is not a new [`crate::gate_coverage::GATE_COVERAGE`] member: it
            // is a finding about **this task's staged content**, computed by this same
            // task-scope sweep and reported in this same report at the same
            // [`EXIT_VALIDATION_BLOCKED`] the committing door takes — the `content-findings`
            // member, answered arm-dependently, exactly as the `carryover` row's own
            // comment records for the index gate. A separate member would put an
            // amend-only clause into the composed `what's-left:` line of every ordinary
            // task, for a check that is inert on every one of them.
            previewed.extend(self.amend_staged_doc_findings(schemas)?);
        }
        // The doc-only arm is the carryover gate's second exemption (M55), and the preview
        // asks the same precedence the committing door does ([`Self::commits_doc_only`]): a
        // path-scoped commit takes no path outside its set, so a path staged before the task
        // existed cannot cross the boundary and there is nothing for the gate to decide.
        if !carry_staged && !amending && !self.commits_doc_only(&self.id)? {
            let snapshot = state::read_staged_snapshot(&self.dir).with_context(|| {
                format!(
                    "could not read the staged snapshot for task at {:?}",
                    self.dir
                )
            })?;
            let retire_exempt = state::read_migration_source(&self.dir).with_context(|| {
                format!("could not read the source path for task at {:?}", self.dir)
            })?;
            previewed.extend(decide_carryover(
                snapshot.as_ref(),
                &git_staged_snapshot(&self.repo_root)?,
                retire_exempt.as_ref().map(state::MigrationSource::recorded),
                &engine::finalize::plan_owner_artifacts(&self.dir, schemas),
                CarryoverBoundary::TaskPreview,
                &self.repo_root,
            ));
        }
        // `&|_| true` is load-bearing, not a shortcut: it is what keeps cause 7 (untracked)
        // post-stage while the six staging-independent causes preview.
        previewed.extend(
            owner_artifacts_gate(&self.dir, schemas, &self.repo_root, &|_| true).with_context(
                || {
                    format!(
                        "previewing the owner-artifact gate for task at {:?}",
                        self.dir
                    )
                },
            )?,
        );
        // The third member: the **changelog-gate advisory** (M46 Inc 6 / T3). It reads
        // the task's recorded minting workflow and its staged changelog against that
        // doc's un-authored baseline — no git shell-out, no staging, no mutation — and
        // it is the one finalize-scope check whose whole state exists *before* the
        // commit, so excluding it made `task validate` silent about a finding the very
        // next invocation would print (and, promoted with one cascade line, refuse on).
        // The committing door keeps computing it itself, at its own position — never
        // merged twice ([`GatePreview::Off`], and [`GatePreview::Forecast`] for the
        // `--dry-run` door that extends that door's own report).
        if changelog {
            previewed.extend(self.changelog_gate_advisory(
                &self.id,
                schemas,
                AdvisoryDoor::TaskPreview,
            )?);
        }
        if previewed.is_empty() {
            return Ok(report);
        }
        let mut findings = report.findings.into_vec();
        findings.extend(previewed);
        Ok(engine::result::ValidationReport::new(
            findings,
            &self.severity_cascade()?,
        ))
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

    /// Build the git **history** predicate the engine's committed-store rename detector
    /// consults to grade a dangling baseline ([`engine::validate::HistoryPredicate`]; M45,
    /// Decision 7). The CLI owns the shell-out, the engine stays shell-free.
    ///
    /// A repo-relative path **has history** iff `git log HEAD -1 -- <path>` produces a line
    /// (some HEAD-reachable commit touched it). So a genuine deletion (the path was committed
    /// then removed) reads history-present → the weak finding keeps blocking; a checkout that
    /// moved underneath the gitignored file-state cache (`git reset --hard` / branch switch /
    /// rebase past the creating commit) leaves the path history-less → the baseline downgrades
    /// to advisory. The query runs only inside the already-cold recorded-but-missing arm, so a
    /// healthy task never shells out for it. A `git log` failure (a broken/unborn HEAD) is
    /// treated as **history-present** — the conservative default that keeps blocking rather
    /// than silently downgrading a possible deletion.
    fn history_predicate(&self) -> impl Fn(&str) -> bool {
        let repo_root = self.repo_root.clone();
        move |path: &str| git_path_has_history(&repo_root, path).unwrap_or(true)
    }

    /// The **live milestone record** the committed-store sweep's dangling-baseline arm
    /// carves out ([`engine::file_state::LiveRecord`]; M52 Inc 10 / T6) — the committed
    /// record of the milestone **this** task belongs to, or
    /// [`LiveRecord::none`](engine::file_state::LiveRecord::none) when it belongs to none.
    ///
    /// A sub-task is pinned to its milestone's **base**, which by construction predates the
    /// record commit that `jigc milestone create` lands. So wherever the checkout stands at
    /// that pin — the provisioned worktree does by construction, the shared checkout whenever
    /// it is put there — the record is absent on disk with no HEAD history: the
    /// dangling-baseline cell exactly, whose shipped route prunes the baseline with `jigc
    /// unmanage`. Followed, that unmanages the record the milestone is run from. The engine
    /// cannot see the membership, so the fact is supplied here, from the two enumerators that
    /// already own it: [`engine::milestone::owning_milestone`] and
    /// [`crate::milestone::record_key`] (the same string the sweep keys the baseline under).
    ///
    /// Path-keyed, never blanket: a record of some *other* milestone, and every non-record
    /// managed doc, keep the shipped advisory and its prune route byte-identically — and
    /// M45's history gate is untouched above this arm, so a genuine deletion of the record
    /// (history present) still blocks.
    fn live_milestone_record(
        &self,
        schemas: &BTreeMap<String, Schema>,
    ) -> engine::file_state::LiveRecord {
        let Some(milestone_id) = engine::milestone::owning_milestone(&self.jigc_root, &self.id)
        else {
            return engine::file_state::LiveRecord::none();
        };
        match schemas
            .get(crate::milestone::MILESTONE_RECORD_TYPE)
            .and_then(|schema| crate::milestone::record_key(schema, &milestone_id))
        {
            Some(key) => engine::file_state::LiveRecord::milestone(key, milestone_id),
            None => engine::file_state::LiveRecord::none(),
        }
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
    fn blocked(&self, findings: Vec<Finding>, format: Format) -> Result<Outcome> {
        let report = engine::result::ValidationReport::new(findings, &self.severity_cascade()?);
        if format == Format::Json {
            print!("{}", render::validation(format, &report));
        } else {
            eprint!("{}", render::validation(format, &report));
            eprintln!();
        }
        Ok(Outcome::with_findings(
            EXIT_VALIDATION_BLOCKED,
            &report.findings,
        ))
    }

    /// Execute the `finalize` transaction (`design/finalize.md` → 5–7). Returns the
    /// process exit code: `SUCCESS` on a landed commit — which also emits the
    /// preflight findings envelope on stdout, symmetric with `task validate`
    /// (`design/measurement.md` → The capture substrate, item 2) —
    /// [`EXIT_VALIDATION_BLOCKED`] on a planner block (rendered + surfaced), `FAILURE`
    /// on a hook/git rejection (git's stderr surfaced, no envelope — not a validation
    /// outcome). An orchestration error (git unavailable, malformed pin) bubbles as
    /// `Err`.
    fn finalize(
        &self,
        id: &str,
        format: Format,
        approve: bool,
        dry_run: bool,
        carry_staged: bool,
    ) -> Result<Outcome> {
        // The sub-task membership guard (round-2 D1; the batch-C C0 verdict): a task
        // that is a milestone sub-task refuses the per-task finalize BEFORE anything
        // else runs — the parent milestone's finalize is the only commit boundary, and
        // a per-sub-task finalize in a fan-out worktree lands a commit on the detached
        // HEAD, empties the staged index the combine folds, and strands the work while
        // the record claims it joined. The promote-clobber refusal class: the guarded
        // op has zero legitimate use (the milestone folds staged indexes directly).
        if let Some(milestone_id) = engine::milestone::owning_milestone(&self.jigc_root, id) {
            return self.blocked(
                vec![engine::milestone::sub_task_finalize_finding(
                    &milestone_id,
                    id,
                )],
                format,
            );
        }
        let base = self.base()?;
        let head = git_head(&self.repo_root)?;
        let schemas = self.schemas()?;

        // **The commit model this task takes** (F-10) — the `amend` marker `jigc task amend`
        // wrote, holding the sha of the commit it pins. `None` on every other task, which is
        // what keeps the ordinary model the default and moves no existing task's behaviour.
        let amend = state::read_amend_pin(&self.dir)
            .with_context(|| format!("could not read the amend marker for task `{id}`"))?;

        // Phase 1 (amended) — the moved-base re-pin decision runs BEFORE anything
        // else reads the pin: `has_diff` must diff against the *effective* base,
        // else a no-work task reads disjoint moved history as its own diff and
        // slips the empty-commit guard. The CLI supplies the git facts; the engine
        // owns the decision (`design/finalize.md` → Parallel hand-editing, the
        // 2026-06-12 phase-1 amendment).
        let base = match decide_base_repin(
            id,
            &self.dir,
            &base,
            &head,
            &git_changed_paths(&self.repo_root, &base.sha, &head)?,
            &git_dirty_paths(&self.repo_root)?,
            &schemas,
        ) {
            // **The amend arm never re-pins** (F-10). `Repin` exists so a task whose history
            // moved disjointly can still land its own work on the new HEAD; an amend's whole
            // subject IS the commit it pinned, so silently re-pinning would rewrite a
            // different commit than the one the agent read the subject line of. Its own
            // mismatch refusal is below, keyed on the marker rather than on this decision.
            _ if amend.is_some() => base,
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

        // **The amend arm's own two gates** (F-10), ahead of everything the ordinary model
        // runs. They sit here, before `plan_finalize`, and the position is forced rather than
        // chosen: the engine raises its OWN `finalize.base-mismatch` inside that call, worded
        // for an ordinary task (*"switch back to `<A>`"* — a real exit when `<A>` is the
        // history your work sits on, and a different repository state when it is the commit
        // you are rewriting), so a re-worded refusal placed after it would be unreachable.
        // Driven: it was, and the arm printed the ordinary task's message.
        //
        // Ahead of the conformance gate too, which is the safer order on the one cell where
        // both apply: the dirty index is the data-loss cell, and it is the one an agent can
        // act on without first re-authoring anything.
        //
        // 1. `finalize.amend-index-dirty` — `git commit --amend` rewrites HEAD from the
        //    INDEX, so anything staged when it runs is folded into the rewritten commit,
        //    silently, at exit 0 (driven on the baseline: HEAD held one file, an unrelated
        //    path was staged, the amended commit carried two). This is
        //    `finalize.carried-staged` re-enacted on a commit that already landed.
        // 2. `finalize.amend-staged-doc` — the task staged a doc that **promotes**, and
        //    this arm commits no tree change, so the promote phase would write that file
        //    into the worktree and commit none of it (the F-10 review's HIGH-1; driven at
        //    exit 0, with `jigc validate` false-green over it because the transaction
        //    baselined its own un-committed promotion). Ahead of the conformance gate
        //    deliberately: a half-authored staged doc answers there first, and a
        //    conformance finding about a doc that cannot land in this task at all routes
        //    the agent at filling it in.
        // 3. `finalize.base-mismatch` — HEAD is no longer the commit the marker pinned, so
        //    the subject line the agent read and re-authored against belongs to a different
        //    commit. The shipped code, re-worded for this arm.
        //
        // **The carryover gate is exempt here, and this is the reason stated at its row**
        // (`design/finalize.md` → The amend arm): an amend stages nothing, so the gate has
        // nothing to decide, and gate 1 above is its stricter replacement — it refuses the
        // WHOLE index, declared or not, where `--carry-staged` would wave a subset through.
        if amend.is_some() {
            let dirty = self.amend_index_findings()?;
            if !dirty.is_empty() {
                return self.blocked(dirty, format);
            }
            let staged_docs = self.amend_staged_doc_findings(&schemas)?;
            if !staged_docs.is_empty() {
                return self.blocked(staged_docs, format);
            }
        }
        if let Some(pinned) = amend.as_deref()
            && pinned != head
        {
            return self.blocked(
                vec![engine::finalize::amend_base_mismatch_finding(
                    id, pinned, &head,
                )],
                format,
            );
        }

        // The validate report the planner gates on — one engine, two entry points
        // (`design/finalize.md` → 2. Validate: no private check path). The post-sweep
        // record rides along: a *landed* commit persists it (the absorb baseline-advance,
        // `design/reconciliation.md` → Persistence of the shifted baseline); a blocked
        // branch drops it.
        // `GatePreview::Off` (M47 Inc 4): the committing path runs the previewable
        // gates itself, each at its own position in the phase order — the carryover
        // refuse below sits inside the `--dry-run` branch and again ahead of the
        // migration review gate. Previewing them here would move those block positions,
        // and finalize's observable behaviour must be unchanged by the preview door.
        let (report, swept) = self.validate(GatePreview::Off)?;

        // The finalize-scope `changelog-recording` check (M42 Settle fork 6;
        // `design/validation.md` → The changelog-gate advisory) — merged into the report
        // BEFORE `plan_finalize` reads it, so a project that promotes the key to
        // `blocking` with one cascade line actually gates on it (the M6 post-pass runs at
        // the single `ValidationReport::new` construction point, and is idempotent over
        // already-assigned findings).
        let mut report = match self.changelog_gate_advisory(id, &schemas, AdvisoryDoor::Finalize)? {
            None => report,
            Some(finding) => {
                let mut findings = report.findings.into_vec();
                findings.push(finding);
                engine::result::ValidationReport::new(findings, &self.severity_cascade()?)
            }
        };

        // A **staged** migration is identified once by the source seam: it selects the
        // stage policy (`MigrationFixed`), feeds the fidelity diff, and — read here, ahead
        // of the empty-commit signal — keeps the migration `has_diff` on the proven
        // whole-tree probe (its blocks, e.g. the missing-replacement F1 retire-safety
        // gate, must precede the empty-commit guard inside `plan_finalize`).
        let source_seam = self.dir.join(engine::state::SOURCE_FILE);
        let staged_migration = source_seam.exists();

        // The **review hold** keys on something else, and M52 Inc 9 / T4 is where the two
        // part company: the hold is a promise the composed body makes, so its subject is
        // the composed contract ([`Self::composes_review_hold`]), not the artifact one
        // door happens to stage. Keyed on the seam alone, a migrate-shaped workflow
        // composed by name landed a commit at exit 0 under its own text's "a plain
        // finalize commits NOTHING … holds (exit 4)". The staged-source cell keeps every
        // mechanic it had; the source-less cell gets the hold and nothing else — it stages
        // no fixed pathspec, retires nothing, and cannot render a fidelity diff, which is
        // the one thing the hold below has to say out loud.
        let holds_for_review = staged_migration || self.composes_review_hold(id)?;

        // **The doc-only commit model** (M55; `design/findings-channel.md` §3), decided once
        // under the arm precedence [`Self::commits_doc_only`] owns, and read by the diff
        // signal, the empty-commit recolor, the carryover decision and the stage policy below
        // — so no two of them can disagree about which commit this task makes.
        let doc_only = self.commits_doc_only(id)?;

        // The diff-presence signal the planner's empty-commit guard needs. On the per-task
        // `IndexHonoring` path it is the NARROWED stage set the commit actually lands (M30
        // G2; `design/finalize.md` → Dirty-tree policy) — never the ambient dirty tree: the
        // agent's staged code (`git diff --cached`), any staged doc that will PROMOTE (the
        // transient commit doc never promotes, so a commit-only task that staged no code
        // reads empty), and the git-tracked config layer a first finalize must land;
        // unstaged/untracked WIP is excluded. A migration task keeps the original
        // whole-tree probe (`MigrationFixed` stages its own fixed set regardless).
        // **The amend arm's change is the message, which no diff probe can see** (F-10). Every
        // branch below asks *"is there anything to commit?"* of the TREE, and the honest
        // answer for an amend is always yes: the commit doc the agent authored is what
        // changes, and it is a transient that promotes nowhere. Left to the probes, a
        // message-only amend trips `finalize.empty-commit` — the one guard whose premise
        // ("git refuses a commit recording no change") is simply false of `--amend`, which
        // mints a new sha every time, driven, even with nothing staged.
        let has_diff = if amend.is_some() {
            true
        } else if staged_migration {
            !git_diff(&self.repo_root, &base.sha)?.trim().is_empty()
                || !self.staged_docs()?.is_empty()
                || !git_untracked(&self.repo_root)?.trim().is_empty()
        } else {
            // A staged doc counts toward the diff exactly when it will PROMOTE — mirroring
            // `plan_promotions`' promotability: a `location:` doctype OR a **placement**
            // doctype (`design/storage.md` → Placement, `location: None`). Testing only
            // `location` here would drop a placement singleton (e.g. the `vision` managed at
            // root `VISION.md`) through the transient arm, tripping the empty-commit guard on
            // a doc-only task whose sole diff IS that promotion. The transient commit doc
            // (neither location nor placement) is still excluded.
            let staged_promotable = self.staged_docs()?.iter().any(|(name, _)| {
                name.strip_suffix(".md")
                    .and_then(|stem| stem.split_once(':'))
                    .and_then(|(ty, _)| schemas.get(ty))
                    .is_some_and(|schema| schema.location.is_some() || schema.placement.is_some())
            });
            // **The doc-only arm's diff is its path set and nothing else** (M55): a doc that
            // promotes, or a recorded owner-artifact. Anyone's staged code and a pending
            // `.jigc/config` delta are outside the set the commit takes, so counting them would
            // let a task with no doc of its own reach a path-scoped commit over nothing.
            if doc_only {
                staged_promotable
                    || !engine::finalize::plan_owner_artifacts(&self.dir, &schemas).is_empty()
            } else {
                let staged_code = !git_capture(&self.repo_root, &["diff", "--cached"])?.is_empty();
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
            }
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
            //
            // Skipped on the doc-only arm (M55): its route is `git add`, and no `git add` can
            // bring a path into a path-scoped commit, so an empty doc-only task keeps the
            // engine's own `finalize.empty-commit`.
            Err(findings)
                if !doc_only
                    && findings.iter().any(|f| f.code == "finalize.empty-commit")
                    && !git_dirty_paths(&self.repo_root)?.is_empty() =>
            {
                return self.blocked(vec![nothing_staged_finding(id)], format);
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

        // M43 — the carryover decision (`design/surface-contract.md` → The carryover
        // gate), computed ONCE pre-commit: `post_commit` deletes the working area (and
        // the mint-time snapshot in it) before the landed manifest is classified, and
        // the one computation threaded to every render site is what makes the
        // forecast/landed identical-set invariant hold by construction. The engine
        // decides over CLI-supplied git facts: the pre-task snapshot every minting door
        // persisted, the same probe re-run now, and the migration's recorded retire
        // pathspec (that deletion is the task's own work, exempt). A task minted before
        // the snapshot existed reads `None` and fails open (the declared bound).
        //
        // **Not on the doc-only arm** (M55; `design/findings-channel.md` §3), the amend
        // arm's precedent: the gate exists because the ordinary commit IS the index, and a
        // path-scoped commit never takes a path outside its set, so a path staged before
        // this task existed cannot cross the boundary — declared or not. The decision is
        // empty there, so the `--dry-run` refusal and the committing refusal below never
        // fire and `--carry-staged` carries nothing: it is inert in every state.
        let carried_findings = if doc_only {
            Vec::new()
        } else {
            let snapshot = state::read_staged_snapshot(&self.dir).with_context(|| {
                format!(
                    "could not read the staged snapshot for task at {:?}",
                    self.dir
                )
            })?;
            let retire_exempt = state::read_migration_source(&self.dir).with_context(|| {
                format!("could not read the source path for task at {:?}", self.dir)
            })?;
            decide_carryover(
                snapshot.as_ref(),
                &git_staged_snapshot(&self.repo_root)?,
                retire_exempt.as_ref().map(state::MigrationSource::recorded),
                // The recorded owner-artifact paths are exempt: an agent stages the audit
                // artifact before minting this recording task (the natural authoring
                // order), so the pre-task staged entry is the task's own subject, never a
                // foreign carry-over (M45 Inc 8; `design/finalize.md` → 5. Stage).
                &plan.owner_artifacts,
                CarryoverBoundary::Task,
                &self.repo_root,
            )
        };
        // The carried path set the manifest labels `carried-over` (labeling changes no
        // set membership) — extracted from the decision's file-path targets, so the
        // label set and the refuse set can never diverge (one decision fn).
        let carried_paths: BTreeSet<String> = carried_findings
            .iter()
            .filter_map(|finding| {
                finding
                    .location
                    .as_ref()
                    .and_then(|location| location.address.clone())
            })
            .collect();

        // B1 dirty-tree sweep — `--dry-run` surfaces the commit file-set and stops, with no
        // commit and no destructive side effect. It is placed BEFORE the migration review
        // gate: a dry-run commits nothing, so `--approve` must never be required. The plan
        // above is computed read-only (`plan_finalize` only reads), so deriving the
        // prediction from it is side-effect-free.
        if dry_run {
            // M47 Inc 4 T3 — the forecast obeys the gate it forecasts. An undeclared
            // carry-over is a state the committing run below refuses at
            // [`EXIT_VALIDATION_BLOCKED`], so forecasting it at exit 0 was a false green
            // — and, because `relabel_carried` stamped `carried-over` regardless, the
            // refusing-state and committing-state JSON documents came out BYTE-IDENTICAL:
            // `--dry-run` was information-free on the one axis `--carry-staged` decides.
            // The refusal IS the forecast (the carried paths and their two exits are the
            // finding's own bytes; the manifest's `carried-over` discriminator survives in
            // `findings[].key.target`), and `--dry-run --carry-staged` forecasts the
            // declared carry instead. Still a pure reader: `blocked` renders and returns.
            if !carry_staged && !carried_findings.is_empty() {
                return self.blocked(carried_findings, format);
            }
            let (mut included, left_out) =
                self.predict_manifest(&plan, staged_migration, doc_only)?;
            // Reached only under a declared `--carry-staged` or an empty carried set, so
            // the label can no longer claim a consent this run never carried.
            relabel_carried(&mut included, &carried_paths);
            // M51 Inc 5 / T5 (EC-20) — the forecast carries the findings the door it
            // forecasts reports. Driven, `task validate` and the LANDED `task finalize`
            // emitted an identical set while `--dry-run` emitted no `findings` key at all,
            // dropping every advisory: the report was computed three statements above this
            // branch and discarded. QUICKSTART presents the two as one preview surface, so
            // the forecast was silent about exactly what the preview exists to show.
            //
            // The extension is [`GatePreview::Forecast`], not a hand-picked gate: the
            // preflight above ran with the preview OFF (it must, or finalize's block
            // positions move), so the owner-artifact causes that need no staging are the
            // one previewable member this report is still missing — and asking the shared
            // seam for them is what keeps a future fifth member from going missing here
            // too. Carryover contributes nothing by construction: this line is reached
            // only with the carried set empty or `--carry-staged` declared, which is the
            // same condition under which the read door omits it.
            //
            // `scope_repair_routes` runs after the merge, mirroring [`Self::validate`]'s
            // order, so a previewed route reads identically at both doors (it is
            // idempotent — an argv already carrying `--task` is left alone).
            let forecast = self.scope_repair_routes(self.preview_gates(
                report,
                GatePreview::Forecast { carry_staged },
                &schemas,
            )?)?;
            // **Which sentence the forecast makes is the commit model's** (F-10): the amend
            // arm rewrites `HEAD` rather than adding to it, so it names the commit it would
            // replace and the subject that commit carries **now** — read here, where `HEAD`
            // is still that commit. Both models hand the envelope the same `subject`
            // ([`render::ForecastSubject`]).
            let rewrites = match amend.as_deref() {
                Some(pinned) => Some((
                    git_capture(&self.repo_root, &["rev-parse", "--short", pinned])?,
                    git_capture(
                        &self.repo_root,
                        &["log", "-1", "--pretty=format:%s", pinned],
                    )?,
                )),
                None => None,
            };
            print!(
                "{}",
                // The subject is the plan's OWN render (phase 3, already computed above),
                // never a dry-run-side re-spelling — M50 Inc 12 / F-7.
                render::finalize_manifest(
                    format,
                    match rewrites.as_ref() {
                        Some((sha, from)) => render::ForecastSubject::Rewrites {
                            sha,
                            from,
                            to: plan.subject(),
                        },
                        // The doc-only arm adds a commit too, path-scoped (M55): the
                        // variant carries the model the left-out section is spelled on.
                        None if doc_only => render::ForecastSubject::AddsPathScoped(plan.subject()),
                        None => render::ForecastSubject::Adds(plan.subject()),
                    },
                    &included,
                    &left_out,
                    &forecast.findings,
                    self.commit_site().as_ref(),
                )
            );
            if format != Format::Json {
                println!();
            }
            // The forecast's findings reach the invocation log too, exactly as the read
            // door's and the landed door's do — a door that prints a finding and records
            // none leaves the log unable to answer what the run saw.
            return Ok(Outcome::with_findings(0, &forecast.findings));
        }

        // M43 — the carryover gate: refuse to let index entries staged BEFORE this task
        // existed silently ride its whole-index commit — one blocking routed finding per
        // carried path. The COMMITTING path's copy of the refusal the `--dry-run` branch
        // above already made (M47 Inc 4 T3 — one decision, two doors), placed ahead of the
        // migration review gate (a blocking refusal precedes the human-fidelity hold, like
        // the planner's validation blocks). `--carry-staged` converts the undecidable
        // intent to a declared one (the `--approve` mold; on a migration the two compose,
        // each gating its own concern).
        if !carry_staged && !carried_findings.is_empty() {
            return self.blocked(carried_findings, format);
        }

        if holds_for_review && !approve {
            // `None` is the source-less cell: no seam was staged, so there is nothing to
            // diff the rewrite against and the render says so. It is not the same as an
            // empty source — an empty diff would read as *"the rewrite dropped
            // everything"*, which is a claim this state has no basis to make.
            let foreign = if staged_migration {
                Some(std::fs::read_to_string(&source_seam).with_context(|| {
                    format!("could not read the staged source seam at {source_seam:?}")
                })?)
            } else {
                None
            };
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
            // The human gate names what `--approve` will delete (M51 Increment 1 / T7).
            // The value comes off the SAME [`ValidatedRetirement`] the sink unlinks from,
            // so the sentence a human consents to and the path `remove_file` receives
            // cannot be different files — naming the raw recorded string here would
            // reintroduce exactly the divergence T3's typed value exists to close.
            //
            // An inadmissible recorded value REFUSES here rather than being named: the
            // hold's whole job is to state what approving does, and what approving does
            // in that state is refuse (`finalize.retire-untrackable`, one code, one
            // route, the same identity the sink raises). Reachable only if something
            // rewrote `source-path` after the `jigc migrate` door adjudicated it.
            let retires = plan
                .retirements
                .iter()
                .map(|retirement| {
                    ValidatedRetirement::adjudicate(
                        &self.repo_root,
                        retirement,
                        StateTruth::ReviewHold,
                    )
                    .map(|validated| validated.path)
                    .map_err(|finding| crate::render::finding_error(&finding))
                })
                .collect::<Result<Vec<_>>>()?;
            print!(
                "{}",
                render::migration_review(format, id, foreign.as_deref(), &rewrites, &retires)
            );
            if format != Format::Json {
                println!();
            }
            // The hold names itself in the log (M43): exit 4 alone said a coded stop
            // happened, never why (`design/finalize.md` → "A failed finalize must be
            // legible in the invocation log").
            return Ok(Outcome::coded_error(
                EXIT_REVIEW_PENDING,
                invocation_log::ERROR_REVIEW_PENDING,
            ));
        }

        // M42 — say what the commit is about to leave out, BEFORE it commits
        // (`design/finalize.md` → "The `left-out` advisory prints BEFORE the commit too").
        // The landed residual below names the same set, but only once the partial commit
        // is already history. The forecast is the `--dry-run` one (side-effect-free, and a
        // `MigrationFixed` migration correctly forecasts none — it sweeps no WIP). It
        // SURFACES only: the commit still lands, the block stays reserved for the
        // empty-index case (`nothing_staged_finding`).
        //
        // Under `--format json` both pre-commit prints are **held** rather than emitted here:
        // which stream carries this run's document is not known until the commit has been
        // attempted, and on a reject the document is stderr's ([`emit_or_defer`]).
        let mut deferred_advisories = String::new();
        let (_, pending_left_out) = self.predict_manifest(&plan, staged_migration, doc_only)?;
        // The model this run is on, so the print describes the commit it is about to make
        // rather than the ordinary one (the F-10 review's MEDIUM-3). Read off the same `amend`
        // marker and `doc_only` answer `stage` is chosen from three statements below, so the
        // sentence and the act cannot disagree.
        emit_left_out_advisory(
            format,
            &mut deferred_advisories,
            &pending_left_out,
            render::CommitModel::of(amend.as_deref(), doc_only),
        );

        // M43 — the carried-over half of the pre-commit print: a `--carry-staged` run
        // names what it is about to carry, BEFORE it commits, with the same
        // `carried-over` label the other three sites render. Built from the carried set
        // directly (not the forecast): a migration's narrowed forecast never lists a
        // carried entry, but its whole-index commit still lands it. Empty on every
        // undeclared run — a non-empty carried set refused above.
        emit_carried_advisory(format, &mut deferred_advisories, &carried_paths);

        // Phases 4–7: the shared transactional core — promote + stage + commit +
        // rollback + post-commit. The working area is the cleanup dir removed on a
        // landed commit.
        // M30 G1 — the per-task stage policy: a migration keeps its proven fixed-pathspec
        // stage; every other per-task finalize honors the agent's existing index.
        let stage = if amend.is_some() {
            StagePolicy::Amend
        } else if staged_migration {
            StagePolicy::MigrationFixed
        } else if doc_only {
            StagePolicy::DocOnly
        } else {
            StagePolicy::IndexHonoring
        };
        // The transaction's `.jigc/.gitignore` report, filled by the executor's shared
        // writer and printed beside the landed render below (M51 Increment 4 / T2).
        let mut ignore_ack = None;
        // Any path the transaction's worktree rollback could not put back (M51 Increment 4 /
        // T3) — empty on a landed commit, and empty on a refused one unless something else
        // rewrote a file jigc had rewritten while the transaction was still running.
        let mut rollback_conflicts: Vec<Finding> = Vec::new();
        // What phase 7 moved out of this task's working area before tearing it down (M52
        // Increment 4 / T3) — empty on the ordinary path, and empty on every arm that never
        // reached the teardown, because nothing was removed there either.
        let mut displaced_foreign: Vec<render::Displaced> = Vec::new();
        // One `finalize.foreign-bytes` advisory per working area phase 7 could not tear down
        // (M53 Increment 2 / T3). Empty on the ordinary path and on every arm that never
        // reached the teardown; at THIS door it rides the landed envelope's `findings`, which
        // the arm below folds it into before rendering.
        let mut kept_areas: Vec<Finding> = Vec::new();
        match try_execute_finalize_plan(
            &self.repo_root,
            &self.jigc_root,
            &self.dir,
            &plan,
            &self.dir,
            &schemas,
            Some(swept),
            stage,
            &mut ignore_ack,
            &mut rollback_conflicts,
            Some(AreaTeardown {
                kind: state::WorkArea::Task,
                unit_id: &self.id,
                moved: &mut displaced_foreign,
                kept: &mut kept_areas,
            }),
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
                // **The manifest is what THIS RUN committed** — the commit's own delta on
                // the ordinary arm. On the amend arm that delta is the superseded commit's
                // file list, unchanged by this run, so reporting it would answer a question
                // nobody asked with a set that reads as *what I just committed*. The amend
                // arm's honest manifest is empty, and its text says the tree is unchanged
                // instead of counting files into it (F-10).
                let (mut manifest, left_out) = if amend.is_some() {
                    (Vec::new(), Vec::new())
                } else {
                    classify_landed_manifest(
                        git_commit_name_status(&self.repo_root)?,
                        git_status_entries(&self.repo_root)?,
                        &promoted_dests,
                        doc_only,
                    )
                };
                // M43 — label the landed carried entries from the PRE-commit-computed
                // set (the working area holding the snapshot is already gone here).
                relabel_carried(&mut manifest, &carried_paths);
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
                    // M45 — the SAME captured hook string the stderr relay carries below
                    // (one capture, two channels; `design/command-output-contract.md` →
                    // Stream discipline). Cloned here for the JSON `committed.hook_output`
                    // key; `relay_hook_output` still trims the same source for stderr.
                    hook_output: hook_output.clone(),
                    // M52 Inc 4 T3 — what phase 7 moved aside rather than destroyed, as the
                    // same repo-relative pairs it already named on stderr. Empty on the
                    // ordinary path, and present either way.
                    displaced: displaced_foreign,
                    // F-10 — the superseded sha, present only on the amend arm, which is its
                    // own `ENVELOPE_ARMS` row. `--amend` has already moved `HEAD` by here, so
                    // the marker jigc wrote at the mint is the only place this still is.
                    //
                    // **Abbreviated to the same length `hash` above carries**, and the reason
                    // is the object it rides: two commit shas sit in one `committed` object,
                    // and the door's own mint ack already named this commit `amending:
                    // <sha7>`. A 40-character `amended` beside a 7-character `hash` would
                    // make a reader ask what the difference means, and the answer would be
                    // *nothing* (`design/surface-contract.md` → law 3). The marker holds the
                    // full sha, which is what `git rev-parse --short` is asked to shorten
                    // here rather than a truncation this code invents.
                    amended: amend
                        .as_deref()
                        .map(|full| git_capture(&self.repo_root, &["rev-parse", "--short", full]))
                        .transpose()?,
                    // M53 — the cwd census, C2-09: the checkout this commit landed in, when
                    // it is not the one the workbench binds to. Text-only by declared bound.
                    site: self.commit_site(),
                    // M55 — the doc-only model's left-out spelling on the landed text, from
                    // the same answer the stage policy was chosen from.
                    doc_only,
                };
                // M53 Increment 2 / T3 — the working area(s) phase 7 could not tear down,
                // folded into the landed envelope's own `findings` array. `Findings::push`
                // is the sanctioned post-report append: the uniqueness seam rides the
                // projection, not the construction, and this code joins no
                // `engine::result::CHECK_INVENTORY`, so re-running the severity post-pass
                // over it would change nothing observable.
                for finding in kept_areas {
                    report.findings.push(finding);
                }
                print!("{}", render::finalize_landed(format, &report, &landed));
                if format != Format::Json {
                    println!();
                }
                // The amend this transaction made to the user's co-owned ignore file.
                crate::gitignore::emit_ack(format, &ignore_ack);
                // T2 — relay any non-blocking hook output the commit produced
                // (`design/finalize.md` → 6. Commit, success-relay).
                relay_hook_output(format, &hook_output);
                // The landed arm's document owns stdout, so the held pre-commit prints go
                // where they always went — stderr, naming the same sets the document carries
                // as `committed.left_out` and the manifest's `carried-over` labels.
                eprint!("{deferred_advisories}");
                // A landed commit still surfaces its preflight advisories in the log record
                // (`design/measurement.md` → item 2: absorb evidence, never swallowed).
                Ok(Outcome::with_findings(0, &report.findings))
            }
            Err(err) => {
                // M45 Inc 8 T2 — the relocated #5 owner-artifact gate blocked AFTER the
                // stage (a recorded artifact absent / unsafe / present-but-untracked). It is
                // a validation block (exit 3, a real `ValidationReport`), distinct from a
                // stage-git failure or a hook rejection; the rollback (promotions + the third
                // index axis) already ran in the executor. Checked first — it owns its own
                // typed marker (`design/finalize.md` → 5. Stage; the validate-0/finalize-3
                // split, which since M47 Inc 4 / T2 is **cause 7's alone** — the other six
                // are previewable, so a driver that ran `task validate` has already seen
                // them; the untracked cause is the only one it could not have).
                if let Some(block) = err.downcast_ref::<OwnerArtifactBlock>() {
                    let findings =
                        fold_rollback_conflicts(format, block.0.clone(), &rollback_conflicts);
                    return Ok(carry_rollback_conflicts(
                        format,
                        self.blocked(findings, format)?,
                        &rollback_conflicts,
                    ));
                }
                // M51 Increment 1 / T3 — the **retire sink** refused: the recorded
                // `source-path` is not a path this repository can unlink, asked one statement
                // above the `remove_file` that would have done it. Re-raised rather than
                // reframed, because it is neither of the two things the frames below say: not
                // a git failure in jigc's own stage phase, and emphatically not a hook
                // rejection — no hook ran, and dressing it as one would tell the operator to
                // satisfy a hook that was never consulted (law 1). The rollback already ran in
                // the executor, so the transaction is whole; the shared error funnel
                // (`invocation_log::operational_failure`) prints the house findings line and
                // records the code, at [`EXIT_ERROR`] — the §10 destroying-door mold's exit.
                //
                // It carries no rollback conflict, and that is a property rather than an
                // omission: the sink adjudicates one statement above its `remove_file`, which
                // runs BEFORE `gitignore::ensure` and before the stage's stamp refresh — so on
                // this arm jigc has rewritten neither file and the worktree family is
                // uniformly *never written*.
                if render::blocked_finding(&err).is_some() {
                    return Err(err);
                }
                // M40 F7 item 2 — a failure in jigc's OWN stage phase (the marked
                // `git add` in `stage_migration`/`stage_index_honoring`) surfaces as a
                // routed blocking finding through the findings envelope, the git
                // stderr embedded verbatim; the rollback already ran in the executor.
                // A commit-phase rejection (no marker) stays verbatim-raw below — the
                // recorded hook decision honored (`design/finalize.md` → 6. Commit,
                // M40 item 3): the hook output IS the correction signal.
                if let Some(stage) = err.downcast_ref::<StageGitFailure>() {
                    let findings = fold_rollback_conflicts(
                        format,
                        vec![stage_failed_finding(id, &stage.0, approve, carry_staged)],
                        &rollback_conflicts,
                    );
                    return Ok(carry_rollback_conflicts(
                        format,
                        self.blocked(findings, format)?,
                        &rollback_conflicts,
                    ));
                }
                // M42 — the commit-phase rejection keeps git's stderr verbatim-raw AND
                // gains the recoverability half it never said: the task survives intact,
                // so the same re-run lands the commit once the hook is satisfied
                // (`design/finalize.md` → 6. Commit). Framed HERE because the task `id` and
                // the run's own flags are in hand — `git_commit` never sees them.
                // It also carries a **route-exempt error identity** into the invocation log
                // (M42 T7, `design/finalize.md` → "A failed finalize must be legible in the
                // invocation log"): without it this record is byte-identical to a
                // `finalize <absent-id>` (exit 1, `finding_codes: []`), so the one failed-
                // finalize class the RC adoption trial hit is the one the log cannot name.
                // **[Revised at M52 Increment 1 / T1.]** This read *"Log-only, deliberately
                // NOT a `Finding` — a Finding would force a mandatory route (the M41
                // advisory-route floor) and wrap the hook's stderr, which the design pins as
                // verbatim and unwrapped."* Both halves were answerable rather than binding: a
                // route is what the frame's own closing sentence already *is*, and the hook's
                // bytes are the finding's `message` — escaped by the envelope, never wrapped.
                // What the reservation bought was a machine surface on which a driver could
                // not read the code it had to branch on, nor the path to its own raced bytes
                // (`advocates/F6.md` spike 1). The identity is still the log's; on
                // `--format json` it is now also the reject document's.
                // M47 Inc 3 T7 — the framing moved into the shared `surface_commit_rejection`
                // so every committing door says the same three things; this door's bytes
                // are unchanged, and the re-run echoes the flags a repeat run genuinely needs
                // (`--approve` on a migration hold, `--carry-staged` past the carryover gate)
                // so the printed line is followable, not just recognizable.
                //
                // M46 Inc 8 T1 — the clause is **scoped to what this door actually holds**
                // (`design/surface-contract.md` → the style guide, "quantified over what that
                // surface actually serves"; RC-1.0-gate B1-1). It used to end *"your staged
                // changes are still staged"*, one word covering two mechanisms: on a
                // docs-only task the work lives in `.jigc/tasks/<id>/docs/` and `git diff
                // --cached` prints nothing, so a reader who took the word in git's sense went
                // looking for a state git could not show. Naming both areas separately is a
                // scope repair, not a behaviour change — nothing about the rollback moved,
                // and each half is driven: the docs-only branch by
                // `commit_rejected_axis::the_task_door_names_its_own_staged_docs_over_an_empty_git_index`
                // (index empty at the moment the frame prints, the ADR still in the task
                // area), the index half by `finalize_message_truth::
                // hook_rejection_says_the_task_is_intact_and_the_rerun_lands` (the re-run
                // commits exactly the `git add`-ed path and nothing else).
                let mut rerun = format!("jigc task finalize {}", shell_token(id));
                if approve {
                    rerun.push_str(" --approve");
                }
                if carry_staged {
                    rerun.push_str(" --carry-staged");
                }
                // F-10 — the amend arm's own identity and its own state-truth clause. It is
                // the cheapest clause in the registry, and driven: `git commit --amend` is
                // atomic w.r.t. `HEAD`, so a rejected amend leaves the commit byte-identical
                // — there is no rollback to scope it against, because nothing was promoted,
                // nothing was staged, and `HEAD` never moved.
                let (code, survived) = match amend.as_deref() {
                    Some(_) => (
                        invocation_log::ERROR_AMEND_REJECTED,
                        format!(
                            "`HEAD` is unchanged — the commit this task is repairing still \
                             carries the message it had, and task {id}'s authored commit doc \
                             is still in `.jigc/tasks/{id}/docs/`"
                        ),
                    ),
                    None => (
                        invocation_log::ERROR_COMMIT_REJECTED,
                        format!(
                            "task {id} is intact — nothing was committed, your task's \
                             staged docs are still in `.jigc/tasks/{id}/docs/`, and \
                             anything you had `git add`-ed is still in git's index"
                        ),
                    ),
                };
                Ok(surface_commit_rejection(
                    format,
                    &err,
                    &RejectionFrame {
                        code,
                        target: work_unit_ref(id),
                        survived,
                        survived_non_hook: None,
                        rerun,
                    },
                    &rollback_conflicts,
                ))
            }
        }
    }

    /// Predict the pre-commit manifest for `--dry-run` — a side-effect-free forecast of
    /// the file-set the commit would carry (B1 dirty-tree sweep), built from repo state
    /// alone (no stage, no commit). Mirrors the two stage paths:
    ///
    /// - **Non-migration** (`IndexHonoring`): start from the working tree (`git status
    ///   --porcelain --untracked-files=all`) and split each dirty path by its porcelain
    ///   columns — X (index) → `included`, Y (worktree) → `left_out` — **except** the paths
    ///   [`stage_index_honoring`] `git add`s itself (the `.jigc/config` / `.jigc/.gitignore`
    ///   config layer + the `.jigc/version` stamp), which are `included` whichever column is
    ///   dirty: jigc stages them, so they are never left behind (M42). Then ADD each
    ///   promotion `destination` as `promoted` (the staged docs live in the gitignored
    ///   working area, absent from `git status`) and each tracked retirement as `deleted`.
    ///   A promotion wins the dedup over any same-path working-tree entry.
    /// - **Migration** (`stage_migration`'s narrowed pathspec): exactly its set — promotions
    ///   (`promoted`), tracked retirements (`deleted`), and the jigc-tracked config layer
    ///   `.jigc/config` / `.jigc/.gitignore` (`modified`). No user WIP.
    /// - **Doc-only** (`stage_doc_only`'s path set, M55): the path set, never the index —
    ///   each promotion `destination` (`promoted`) and each dirty path under a stageable
    ///   recorded owner-artifact (by the rule jigc's own stage uses). Every other porcelain
    ///   entry is `left_out`, **staged ones included**, one entry per path
    ///   ([`left_out_entry`]): the path-scoped commit leaves it where it is.
    ///
    /// Accepted prediction bound: on a first-ever finalize, the transaction's
    /// `crate::gitignore::ensure` may create `.jigc/.gitignore` that `git add --all` would
    /// then sweep but this prediction won't show (it doesn't exist yet) — setup typically
    /// already writes it, so the gap is rare.
    fn predict_manifest(
        &self,
        plan: &engine::finalize::FinalizePlan,
        staged_migration: bool,
        doc_only: bool,
    ) -> Result<(Vec<render::ManifestEntry>, Vec<render::ManifestEntry>)> {
        use render::{ManifestEntry, ManifestKind};

        let promoted: Vec<String> = plan
            .promotions
            .iter()
            .map(|promotion| promotion.destination.clone())
            .collect();

        if staged_migration {
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

        if doc_only {
            // The doc-only arm commits its path set alone (M55): the promotions, added below,
            // and whatever its owner-artifact stage adds. Nothing else is included, whichever
            // column is dirty — and nothing else is staged by jigc either (no config layer, no
            // version stamp), so every remaining entry is left where it is.
            let owner_specs = owner_artifact_stage_specs(&self.repo_root, plan);
            let mut included: Vec<ManifestEntry> = Vec::new();
            let mut left_out: Vec<ManifestEntry> = Vec::new();
            for (code, path) in git_status_entries(&self.repo_root)? {
                if promoted.contains(&path) {
                    continue;
                }
                if owner_specs.iter().any(|spec| pathspec_covers(spec, &path)) {
                    included.push(ManifestEntry {
                        kind: self_staged_kind(&code),
                        path,
                    });
                    continue;
                }
                left_out.extend(left_out_entry(&code, path, true));
            }
            included.extend(promoted.into_iter().map(|path| ManifestEntry {
                path,
                kind: ManifestKind::Promoted,
            }));
            return Ok((included, left_out));
        }

        // Non-migration `IndexHonoring`: the commit lands the INDEX, so split each dirty
        // path by its porcelain column (M30 G3) — the X (index) column is **included** in
        // the commit, the Y (worktree) column is **left out** (unstaged/untracked WIP the
        // agent must `git add` to include). A staged-then-further-modified (`MM`) path is
        // non-blank in both columns, so it appears in **both** sets.
        //
        // …with ONE exception the columns alone cannot see: [`stage_index_honoring`] adds
        // **jigc's own contributions** to the index before the commit, so a dirty path under
        // that narrowed pathspec is *included*, never left behind — predicting it "left-out
        // (git add to include)" would state a falsehood AND tell the agent to stage a path
        // the CLI owns and is staging itself (M42; the migration branch above already
        // accounts for its own stage — this is the un-swept sibling).
        let promoted_set: std::collections::HashSet<&str> =
            promoted.iter().map(String::as_str).collect();
        // The stage's own pathspecs, guarded exactly as it guards them — except the version
        // stamp, which `stage_index_honoring` REFRESHES before the existence guard, so it is
        // present at stage time whatever this prediction sees now.
        let mut jigc_staged =
            existing_pathspecs(&self.repo_root, &[".jigc/config", ".jigc/.gitignore"]);
        jigc_staged.push(crate::setup::VERSION_STAMP_PATH.to_owned());
        let mut included: Vec<ManifestEntry> = Vec::new();
        let mut left_out: Vec<ManifestEntry> = Vec::new();
        for (code, path) in git_status_entries(&self.repo_root)? {
            // A promotion lands at its canonical path and wins the dedup; it is added below.
            if promoted_set.contains(path.as_str()) {
                continue;
            }
            let x = code.chars().next().unwrap_or(' ');
            // jigc stages this one itself (a `git add -- <spec>` matches the file and, for a
            // directory spec like `.jigc/config`, everything under it) — so it rides the
            // commit whichever column is dirty: an untracked one is `added`, otherwise the
            // index column when it is already staged, else the worktree column jigc will stage.
            if jigc_staged.iter().any(|spec| pathspec_covers(spec, &path)) {
                included.push(ManifestEntry {
                    kind: self_staged_kind(&code),
                    path,
                });
                continue;
            }
            // X names the staged change the commit carries. `?` (untracked) is not in the
            // index, so it never counts as included.
            if x != ' ' && x != '?' {
                included.push(ManifestEntry {
                    path: path.clone(),
                    kind: column_kind(x),
                });
            }
            // Y names the un-staged worktree residual left out of the commit.
            left_out.extend(left_out_entry(&code, path, false));
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
    /// 3. `<addr>` parses (a grammar fault answers with `cli::doc`'s shared guidance and
    ///    route) + its canonical committed path exists, else [`no_committed_doc`]'s
    ///    blocking `store.not-found`;
    /// 4. the address's `<type>` equals the role's declared `type` else the mismatch;
    /// 5. record `role -> <addr>` in `roles.json` (last-write-wins).
    ///
    /// Hands the **parsed** address back to the caller, which stamps the ack's `target`
    /// from it (the URI normal form), never from the raw `addr` argument.
    fn bind(&self, role: &str, addr: &str) -> Result<Address> {
        let def = self.workflow_def()?;

        // Step 2 — the role must be one the workflow declares as a `reads` role.
        //
        // **Coded at the M52 completion audit (fix 5).** The route-floor sweep that closed
        // `jigc relocate`'s six bare refusals drove this door too, and found the ledger's
        // count of what was left here wrong in the honest direction: M52 Increment 10 / T7
        // discharged the **two** refusals `decisions-pending.md`'s axis-6 lead 6a named
        // (the address grammar fault and the committed-store miss), and this one — which
        // fires *before* both, on an ordinary wrong role — was in no ledger, brief or
        // increment. It is the same defect: a `bail!` carries no code, so the route floor
        // (`design/surface-contract.md` → law 2) cannot reach it at all.
        let Some(reads) = def.reads.iter().find(|r| r.role == role) else {
            let declared: Vec<&str> = def.reads.iter().map(|r| r.role.as_str()).collect();
            let route = if declared.is_empty() {
                engine::finding::Route::mechanical(
                    ["jigc", "start", "--task", &self.id],
                    " re-composes this task's step text — a workflow that declares no read \
                     role binds nothing, and the context it reads is composed for you",
                )
            } else {
                engine::finding::Route::human(format!(
                    "re-run with one of this task's declared read-roles: {}",
                    declared.join(", "),
                ))
            };
            let declared = if declared.is_empty() {
                "none".to_string()
            } else {
                declared.join(", ")
            };
            return Err(crate::render::finding_error(&Finding::graded(
                Severity::Blocking,
                UNDECLARED_READ_ROLE,
                format!(
                    "role `{role}` is not a declared read-role of this task (declared: \
                     {declared})"
                ),
                Some(Location::addressed(format!("task:{}", self.id), 1, 1)),
                Some(route),
            )));
        };

        // Parse the address (it must name a `<type>:<slug>`), then ask whether the slug
        // head is a slug at all — the guard the ten `DoctypeArg::Address` doors share
        // (M50 Inc 2 / T1). This door's refusal was route-less; it gains the route with it.
        //
        // The grammar fault composes through `cli::doc`'s single guidance home (M52
        // Increment 10 / T7; per-axis-review axis-6 §4 lead 2 — both of this door's
        // refusals were bare `anyhow` bails, *"the last inch of DEFECT A6-3's dead end"*),
        // so the explanation and the route are the ones `jigc doc show` emits for the same
        // fault and cannot drift from them. The address is handed over **twice** because
        // this door does not expand a bare fixed-identity head: what it parses *is* what
        // the caller typed, and `BareHead::Verbatim` is what keeps the head sentence from
        // offering a spelling `Address::parse` refuses here.
        let address = Address::parse(addr).map_err(|err| {
            crate::doc::malformed_address_error(addr, addr, &err, crate::doc::BareHead::Verbatim)
        })?;
        reject_malformed_slug_head(addr, address.slug.as_str())?;

        // Step 3a — an **unknown doctype** is its own fault, and it used to be folded into
        // the not-found below: `jigc task bind spec nosuch:thing <id>` answered ``no such
        // doc `nosuch:thing` ``, naming the *doc* when the *doctype* is what does not exist,
        // with no finding code and no route to the surface that would have shown the caller
        // the real doctypes (M49 Increment 11 / T1 — the axis's fifteenth door).
        let schemas = self.schemas()?;
        let Some(schema) = schemas.get(address.r#type.as_str()) else {
            return Err(crate::render::finding_error(
                &engine::store::unknown_doctype(address.r#type.as_str()),
            ));
        };

        // …and a well-formed slug still has to be an identity this doctype **can have**
        // (M52 Increment 6 / T3; settle-record → D5.2 as amended by §9). Here rather than
        // at the parse above, because the predicate needs a resolved `Schema` and this
        // door is not a caller of `cli::doc::parse_verb_addr`'s funnel — and **after**
        // the unknown-doctype answer above, so that precedence is unchanged.
        //
        // Driven at `9961013c` over a manufactured `reads:` role, `jigc task bind vision
        // vision:alpha <task>` exited **0** and wrote `"vision": "vision:alpha"` into
        // `roles.json` — because step 3b below resolves through `canonical_path`, whose
        // placement branch ignores the slug and finds the real committed `VISION.md`. The
        // task is then carrying a recorded read role at an identity every read surface
        // refuses (`jigc doc show vision:alpha` blocks). The `location:`-homed sibling
        // refused instead, but with step 3b's **code-less** `no such doc <addr>` bail: a
        // fact about the corpus, inviting the caller to create a doc this doctype cannot
        // have.
        reject_fixed_identity_alias(schema, &address)
            .map_err(|finding| crate::render::envelope_finding_error(&finding))?;

        // Step 3b — the addr must resolve in the committed store: its canonical
        // `<location>/<slug>.md` must exist (identity is the path). A transient
        // (location-less) type has no committed path → unresolved.
        let committed = canonical_path(&self.jigc_home, schema, address.slug.as_str());
        if !committed.as_deref().is_some_and(Path::is_file) {
            return Err(crate::render::envelope_finding_error(&no_committed_doc(
                &self.jigc_home,
                &address,
                committed.as_deref(),
            )));
        }

        // Step 4 — the target's doctype must equal the role's declared `type`. Coded with
        // step 2 above (M52 completion audit, fix 5), and routed **mechanically**: the
        // declared type is in hand, and `jigc doc list <ty>` names every committed instance
        // of it, so the reader's next command is the one that answers.
        if address.r#type.as_str() != reads.doc_type {
            return Err(crate::render::finding_error(&Finding::graded(
                Severity::Blocking,
                ROLE_TYPE_MISMATCH,
                format!(
                    "doctype mismatch: `{addr}` is a `{}` but role `{role}` declares type `{}`",
                    address.r#type.as_str(),
                    reads.doc_type,
                ),
                Some(Location::addressed(format!("task:{}", self.id), 1, 1)),
                Some(engine::finding::Route::mechanical(
                    ["jigc", "doc", "list", &reads.doc_type],
                    " names every committed instance of the type this role reads",
                )),
            )));
        }

        // Step 5 — record the binding (last-write-wins) and persist it.
        let mut roles = RolesRecord::load(&self.dir)
            .with_context(|| format!("could not read roles for task at {:?}", self.dir))?;
        roles.bind(role, addr);
        roles
            .save(&self.dir)
            .with_context(|| format!("could not record the binding for task at {:?}", self.dir))?;
        Ok(address)
    }

    /// Load the task's own minting workflow definition — the source the `reads`
    /// declaration lives on, read back the same way the resume re-compose does
    /// (`crate::start::resume_in_repo`), which since M52 is literally true: both go
    /// through [`crate::start::resolved_workflow`], so a project `workflows/<id>.yaml`
    /// shadow's `reads` roles are the ones `jigc task bind` enforces. Until M52 this
    /// read was pack-only while resume read the cascade — driven, `jigc task bind` on a
    /// task minted from a project-layer-only workflow id answered *"the recorded workflow
    /// `offverb` reads back: no pack resource of kind workflows with id `offverb`"* at
    /// exit 1 over a workflow the compose door had just resolved at exit 0.
    ///
    /// A working area with no recorded workflow id is a clear fault, never a silent
    /// fall-through to the cascade default.
    fn workflow_def(&self) -> Result<WorkflowDef> {
        let workflow_id = state::read_workflow_id(&self.dir)
            .with_context(|| format!("could not read the recorded workflow for {:?}", self.dir))?
            .with_context(|| {
                format!(
                    "task at {:?} has no recorded workflow — discard it and re-start with {}",
                    self.dir,
                    engine::finding::Route::mechanical(["jigc", "start"], ""),
                )
            })?;
        let bytes = crate::start::resolved_workflow(
            self.pack.as_ref(),
            &self.project_config(),
            workflow_id.as_str(),
        )?;
        load_workflow_def(&bytes).map_err(finding_to_err)
    }

    /// The workflow definition this task **recorded at mint**, with its id — or `None`
    /// when the task recorded none.
    ///
    /// Absence is never a fault: a task minted before the workflow record existed
    /// carries no id, and both callers read a *property of the composed body*, not a
    /// state assertion — so a task with nothing recorded simply has no such property
    /// ([`Self::changelog_gate_advisory`]'s shipped posture, now shared).
    ///
    /// One home for the read, because two call sites deciding finalize behaviour from
    /// the same recorded id must not diverge on how they resolve it — and the read is
    /// the **cascade's** ([`crate::start::resolved_workflow`]), because not diverging
    /// from each other is worth nothing while both diverge from the door that composed
    /// the body. [`workflow_def`]
    /// stays separate rather than wrapping this: its caller needs the *binding* source
    /// and a missing record there is a clear fault with its own route, not an absence to
    /// skip over.
    ///
    /// [`workflow_def`]: Self::workflow_def
    fn recorded_workflow(&self, id: &str) -> Result<Option<(String, WorkflowDef)>> {
        let Some(workflow_id) = state::read_workflow_id(&self.dir)
            .with_context(|| format!("could not read the recorded workflow of task `{id}`"))?
        else {
            return Ok(None);
        };
        let bytes = crate::start::resolved_workflow(
            self.pack.as_ref(),
            &self.project_config(),
            workflow_id.as_str(),
        )?;
        let def = load_workflow_def(&bytes).map_err(finding_to_err)?;
        Ok(Some((workflow_id, def)))
    }

    /// Whether this task's **composed body promises the migration review hold** — its
    /// recorded workflow includes [`MIGRATION_FINALIZE_STEP`], whose text states that a
    /// plain finalize commits nothing and holds at exit 4.
    ///
    /// **The subject is the composed contract, not the staged source.** Until M52 the
    /// hold keyed on the source seam alone, which only `jigc migrate` writes — so a
    /// migrate-shaped workflow composed *by name* minted a task whose own composed text
    /// promised a hold and whose plain `jigc task finalize` landed a commit at exit 0
    /// (`design/auto-migration.md` → The review gate). It is also deliberately **not**
    /// keyed on `suppressed.door`: the cell that reaches this arm is precisely a
    /// migrate-shaped workflow declaring none, which is the cell the verb-routed compose
    /// refusal ([`crate::start::VERB_ROUTED_CODE`]) leaves open.
    ///
    /// **[Corrected 2026-09-20 (M52 Increment 9, the validate→fix loop):** that cell was
    /// named *"a **project-layer** migrate-shaped workflow"*, and as written it was
    /// **false at the layer it names** — [`Self::recorded_workflow`] resolved through the
    /// pack while the compose door resolved through the cascade, so the project layer was
    /// the one layer this arm could not see. Driven at the T4 commit: a project
    /// `.jigc/config/workflows/single-task.yaml` shadow whose body includes
    /// `step:migration-finalize` composed *"commits NOTHING — … holds (exit 4)"* and its
    /// plain `jigc task finalize` landed a commit at **exit 0**; a project-layer-only id
    /// died at exit 1 with `no pack resource of kind workflows with id`. What actually
    /// reached the arm was a **project *pack*** member — which is what T4's own fixture
    /// (`FixturePack`) exercises. The claim is now true of the layer it names, because
    /// the read moved to [`crate::start::resolved_workflow`].**]**
    ///
    /// A task that recorded no workflow composes no such promise — `false`, never a
    /// fault.
    fn composes_review_hold(&self, id: &str) -> Result<bool> {
        Ok(self.recorded_workflow(id)?.is_some_and(|(_, def)| {
            def.includes
                .iter()
                .any(|step| step == MIGRATION_FINALIZE_STEP)
        }))
    }

    /// Whether this task's finalize takes the **doc-only commit** (M55;
    /// `design/findings-channel.md` §3; `design/finalize.md` → The doc-only arm) — the one
    /// place the arm's **precedence** is decided, asked by the committing door and by the
    /// preview alike: **the amend marker, then a staged migration seam, then the doc-only
    /// step, then the ordinary model.** An amend task commits no tree change and a staged
    /// migration stages its own fixed set, so neither can also be path-scoped to its docs;
    /// the shipped migrate workflows never compose the step, so the second clause is a
    /// precedence statement rather than a reachable conflict.
    ///
    /// The subject is the composed contract, as for [`Self::composes_review_hold`]: the
    /// recorded workflow, cascade-resolved, includes [`DOC_ONLY_FINALIZE_STEP`]. A task that
    /// recorded no workflow composes no such promise — `false`, never a fault.
    fn commits_doc_only(&self, id: &str) -> Result<bool> {
        if state::read_amend_pin(&self.dir)
            .with_context(|| format!("could not read the amend marker for task `{id}`"))?
            .is_some()
            || self.dir.join(engine::state::SOURCE_FILE).exists()
        {
            return Ok(false);
        }
        Ok(self
            .recorded_workflow(id)?
            .is_some_and(|(_, def)| composes_doc_only_finalize(&def)))
    }

    /// The **granted-and-unused changelog gate** finding (M42 Settle fork 6;
    /// `design/validation.md` → The changelog-gate advisory): the task's minting
    /// workflow **grants** the `changelog` create-gate and the task authored **no**
    /// changelog entry. `None` when the gate is absent (the omitting context — the check
    /// is inert, never an error) or when an entry was authored.
    ///
    /// It keys on the **gate**, never on the diff: whether a change is "user-facing" is
    /// judgment, and the determinism boundary forbids the CLI from making it
    /// (`VISION.md` → the determinism boundary). The pack decides which workflows record
    /// a changelog by granting the gate; this only reports that a granted gate went
    /// unused. Advisory by default, but an **inventory row** — a project that means it
    /// (the trial's *"if it matters, gate it"*) promotes it to `blocking` with one
    /// cascade line, and the M6 post-pass gates on it with no code change.
    ///
    /// **"Authored" is a write-touch, not an item count.** The staged changelog is
    /// compared against its **un-authored baseline** ([`TaskArea::changelog_baseline`]
    /// — the bytes the staging primitive materialized at first touch), so *any* staged
    /// write that lands bytes counts: a new change-group, a retitled release, a
    /// corrected `date`, rewritten `notes`, a retracted entry. The predicate it
    /// replaces compared **item counts**, so the three writes that leave the count
    /// alone recorded an entry the check refused to see — the trial's F-1
    /// ([`RC-1.0-gate/findings-verification.md`] §1), and a straight contradiction of
    /// the design's own *"it keys on the gate, never on the diff"*.
    ///
    /// A bare `jigc doc create changelog` materializes exactly that baseline (a
    /// copy-in of the committed body, or the pristine skeleton when none is committed),
    /// so it is **not** an entry and the advisory still fires: create-and-abandon is
    /// precisely the silent skip this check exists to surface. A **refused** write
    /// leaves the same copied-in bytes behind, and reads the same way.
    ///
    /// [Keys at the task](work_unit_location) — the finding's subject is the *work
    /// unit*, so a `null` target would collapse every skipped changelog in the corpus
    /// onto one `(code, target)` key (`command-output-contract.md` → the form table, the
    /// work-unit row).
    ///
    /// **Reported at both doors, routed per door.** Since M46 Inc 6 / T3 the advisory
    /// is a member of the previewed set ([`TaskArea::preview_gates`]), so `door`
    /// selects the recovery whose verbs actually run there
    /// ([`changelog_gate_route`]) — the M47 claim *`task validate` previews what
    /// finalize gates on*, applied to the one finalize-scope check that was outside it.
    fn changelog_gate_advisory(
        &self,
        id: &str,
        schemas: &BTreeMap<String, Schema>,
        door: AdvisoryDoor,
    ) -> Result<Option<Finding>> {
        // A pack-set shipping no `changelog` doctype cannot grant the gate.
        let Some(schema) = schemas.get(CHANGELOG_TYPE) else {
            return Ok(None);
        };
        // The gate lives on the task's MINTING workflow. A task minted before the
        // workflow record existed carries none — no gate, no finding (never a fault: this
        // check is an advisory, not a state assertion).
        let Some((workflow_id, def)) = self.recorded_workflow(id)? else {
            return Ok(None);
        };
        if !def
            .allows_create
            .iter()
            .any(|gate| gate.doc_type == CHANGELOG_TYPE)
        {
            return Ok(None);
        }

        // The singleton stages at its fixed slug (= the type id); its committed home is
        // the placement literal (`CHANGELOG.md`), absent until a first entry lands.
        let staged_path = state::instance_path(&self.dir, CHANGELOG_TYPE, CHANGELOG_TYPE);
        let staged = match std::fs::read_to_string(&staged_path) {
            Ok(body) => Some(body),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(err) => {
                return Err(err)
                    .with_context(|| format!("reading the staged changelog at {staged_path:?}"));
            }
        };
        if let Some(staged) = staged
            && let Some(baseline) = self.changelog_baseline(schema)?
            && staged != baseline
        {
            return Ok(None);
        }

        Ok(Some(Finding::graded(
            Severity::Advisory,
            CHANGELOG_GATE_CODE,
            format!(
                "workflow `{workflow_id}` grants the `changelog` create-gate and this \
                 task recorded no changelog entry",
            ),
            Some(work_unit_location(id)),
            Some(changelog_gate_route(id, door, self.changelog_gate_refuses()?).into()),
        )))
    }

    /// Whether the resolved cascade has promoted this advisory to **blocking** — the
    /// premise the route turns on at *either* door: a promoted gate refuses the
    /// commit, which leaves the working area open, so the in-task verbs run and a
    /// change that is *not* user-facing needs a stated exit (an advisory's *"no action
    /// is needed"* is no exit at all from a blocking finding).
    ///
    /// Resolved through the report's **own** construction point — `ValidationReport::new`
    /// runs the M6 post-pass including its inventory-membership test
    /// (`design/validation.md` → Severity assignment) — over a throwaway probe finding,
    /// never a second copy of the three-step key lookup: a route promising the wrong
    /// recovery because it re-derived the severity differently is exactly the drift the
    /// single construction point exists to prevent.
    fn changelog_gate_refuses(&self) -> Result<bool> {
        let probe = Finding::graded(
            Severity::Advisory,
            CHANGELOG_GATE_CODE,
            String::new(),
            None,
            None,
        );
        Ok(
            engine::result::ValidationReport::new(vec![probe], &self.severity_cascade()?)
                .has_blocking(),
        )
    }

    /// The **un-authored baseline** of this task's staged changelog: the bytes the
    /// staging primitive materialized at first touch, before any authoring landed on
    /// them. The write-touch predicate above is `staged != baseline`.
    ///
    /// The two arms are the two staging primitives, discriminated by the provenance
    /// the working area already records at first touch (`storage.md` → The by-task-id
    /// join → classification by provenance), never re-derived from what happens to be
    /// on disk — a migration task seeds **blank** over an in-location squatter, so
    /// "a committed file exists" is not the discriminator:
    ///
    /// * [`Provenance::Created`](state::Provenance::Created) — the doc was minted here,
    ///   so the baseline is the pristine create skeleton, rebuilt through
    ///   [`engine::state::provisioned_bytes`] (the mint's own renderer) from the same
    ///   `on_create` seeds `jigc doc create` computes. The changelog's only doc-level
    ///   header field is the machine-maintained `schema-version` stamp — a
    ///   [`set:`-derived leaf the write path refuses](crate::doc) — so this rebuild is
    ///   clock-free and byte-exact.
    /// * otherwise — the doc was copied in from the committed store, so the baseline is
    ///   the committed bytes under the one canonicalization
    ///   [`copy_in`](engine::state::copy_in) applies.
    ///
    /// `None` is *"no baseline to compare against"* — a committed source that is gone,
    /// or a pack whose `changelog` is not a fixed-title singleton and whose skeleton
    /// title is therefore unknown. The caller keeps the advisory on: an unprovable
    /// write is not a recorded entry, and the finding is an advisory either way.
    fn changelog_baseline(&self, schema: &Schema) -> Result<Option<String>> {
        let address = format!("{CHANGELOG_TYPE}:{CHANGELOG_TYPE}");
        let provenance = state::ProvenanceRecord::load(&self.dir)
            .with_context(|| {
                format!(
                    "could not read the staged-doc provenance of task `{}`",
                    self.id
                )
            })?
            .get(&address);
        if provenance == Some(state::Provenance::Created) {
            let Some(title) = schema.fixed_title() else {
                return Ok(None);
            };
            let migration = state::read_migration_source(&self.dir)
                .context("could not read the task's migration source path")?
                .is_some();
            let on_create = crate::doc::on_create_doc_fields(
                schema,
                migration,
                crate::doc::stamp_schema_version(self.pack.as_ref(), CHANGELOG_TYPE),
            );
            return Ok(Some(state::provisioned_bytes(schema, &title, &on_create)));
        }
        let Some(path) = canonical_path(&self.jigc_home, schema, CHANGELOG_TYPE) else {
            return Ok(None);
        };
        if !path.is_file() {
            return Ok(None);
        }
        let committed = std::fs::read_to_string(&path)
            .with_context(|| format!("reading the committed changelog at {path:?}"))?;
        Ok(Some(engine::write::first_touch_canonicalize(&committed)))
    }

    /// The staged managed-doc instances under the working area's `docs/`, in
    /// path-sorted filename order, as `(filename, body)` pairs. An absent `docs/`
    /// yields an empty list (nothing staged yet).
    ///
    /// **The name rule is [`engine::state::staged_doc_id`]'s**, the one home
    /// ([`staged_doc_ids`]' rule, and the inverse of the writer's own
    /// `<type>:<slug>.md`). It asked `ends_with(".md")` until M52 Increment 5's audit fix,
    /// which was the same looseness the `docs/` membership predicate carried — here it lies
    /// rather than destroys: `jigc task diff --format json` reported a third party's
    /// `docs/agent-notes.md` as a staged doc under an `id` no `jigc doc show` can open.
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
            if engine::state::staged_doc_id(&name).is_some() {
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

/// The changelog-gate advisory's **route**, composed for the door that prints it
/// (M46 Inc 6 / T3; `design/surface-contract.md` → law 1 and the universal
/// advisory-route floor).
///
/// Three shapes, and each names only verbs that run where it is printed:
///
/// * **the preview door** — `.jigc/tasks/<id>/` is open by construction, so the
///   in-task write verbs lead;
/// * **the committing door, unpromoted** — this invocation lands the commit and
///   removes the working area, so the entry becomes a task of its own and the in-task
///   form is **not offered**: printed here it would answer ``no task `<id>` `` at exit
///   1, which is the dead end M46's F-E names (the shipped text offered it as the
///   *"before finalize"* option, on the false premise that it was live on a preview
///   the advisory did not then reach);
/// * **promoted to `blocking`, either door** — the gate refuses the commit, so the
///   task survives and the in-task verbs run; the not-user-facing case gets the one
///   exit that actually clears a blocking finding, since *"no action is needed"* does
///   not clear one.
fn changelog_gate_route(id: &str, door: AdvisoryDoor, refuses: bool) -> String {
    let in_task = format!(
        "`jigc doc create changelog --title Changelog --task {id}`, then \
         `jigc doc add-item changelog:changelog#unreleased-changes --title <category> \
         --task {id}`"
    );
    if refuses {
        return format!(
            "this project has promoted the gate to `blocking`, so record the entry in \
             this task — {in_task}; if the change is not user-facing, lower the gate \
             back with `jigc config set validation.{CHANGELOG_GATE_CODE}.severity \
             advisory`"
        );
    }
    let record = match door {
        AdvisoryDoor::TaskPreview => format!("record it in this task — {in_task}"),
        AdvisoryDoor::Finalize => {
            "record it — `jigc start --workflow record-change \"<what changed>\"`".to_owned()
        }
    };
    format!("if the change is user-facing, {record}; if it is not user-facing, no action is needed")
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
    /// The **amend** arm (F-10) — stage nothing at all, and commit through
    /// [`git_commit_amend`]. It is the one policy whose staging step is the *empty* act
    /// rather than a narrowed one, and that is the arm's contract rather than an
    /// optimization: its own gate has already refused a non-empty index, so there is nothing
    /// left that could be staged, and adding jigc's promoted destinations (which the two
    /// per-task policies do) would put a file into a commit whose tree this arm exists to
    /// leave alone. An amend task promotes nothing anyway — the only doc it stages is the
    /// transient `commit`, which never promotes.
    Amend,
    /// The **doc-only** arm (M55; `design/findings-channel.md` §3) — stage exactly the
    /// task's promoted docs (created, or copied in and edited) and its stageable recorded
    /// owner-artifacts ([`stage_doc_only`]), and commit **those paths alone** through
    /// [`crate::milestone::git_commit_paths`], never the index. Whatever else is staged in the
    /// checkout stays staged. No config-layer stage and no `.jigc/version` refresh: both sit
    /// outside the path set by construction and wait for the next ordinary finalize (gap G8;
    /// `design/storage.md` → the version stamp's doc-only exception).
    DocOnly,
    /// The `squash: true` fan-out boundary (M31 Inc 4) — the id-ordered list of the
    /// milestone's still-provisioned worktree paths, plus the optional repo-relative
    /// `milestone-record` pathspec to path-add into the same commit (M39 T4: the `join`
    /// status-flip folds into the one finalize commit; `None` dev-only / no record). The
    /// sub-agent code lives in those isolated worktrees, NOT this checkout, so a `git add
    /// --all` sweep would drop it; instead [`combine_commit`] folds each worktree's staged
    /// code-set onto the base tree **off-line**, overlays the promoted docs + config + the
    /// record, and commits the combined tree with the user's hooks running from a dedicated
    /// worktree (M31 Inc 5). An empty list (a docs-only / never-provisioned milestone)
    /// degrades to a docs-only commit, byte-identical to the M7 single-aggregate form —
    /// and since M47 Inc 3 that degrade **has a floor**: the caller refuses a boundary
    /// whose sole change is the milestone record's own `joined` flip
    /// (`milestone.zero-contribution`; `design/finalize.md` → The zero-contribution
    /// refusal), so an empty list reaching here always has merged docs to commit.
    Combine(Vec<PathBuf>, Option<String>),
    /// The `squash: false` fan-out boundary (M31 — WIP-safe rework). The id-ordered
    /// `(staged-patch, rendered-commit-message)` pairs for the code-carrying sub-tasks.
    /// [`chain_commit`] builds the whole N+1 commit chain (one commit per sub-task carrying
    /// THAT sub-task's code with the user's hooks running, then the merged-docs aggregate)
    /// in a **dedicated worktree**, then fast-forwards main — the live checkout is never the
    /// commit site and is never `git reset --hard`ed, so an abort (a hook rejection) leaves
    /// unrelated main-checkout WIP intact (review S2; the squash:true
    /// [`Combine`](StagePolicy::Combine) WIP-safety, mirrored onto the honest-rework path).
    /// Every chain commit's captured hook stream folds into the returned `hook_output`
    /// ([`fold_hook_streams`]), so the caller's one relay + envelope carry all N+1 (the
    /// hook_output producer axis). An empty list degrades to a docs-only aggregate — with
    /// the same M47 Inc 3 floor as [`Combine`](StagePolicy::Combine): the caller has
    /// already refused a boundary carrying neither merged docs nor staged code.
    ChainPerSubtask {
        subtasks: Vec<(Vec<u8>, String)>,
        /// The optional repo-relative `milestone-record` pathspec to path-add into the
        /// merged-docs aggregate commit (M39 T4: the `join` status-flip folds into the one
        /// finalize commit; `None` dev-only / no record).
        record: Option<String>,
    },
}

impl StagePolicy {
    /// The milestone-record pathspecs this arm `git add`s into the **LIVE** index — the
    /// fifth staged-path family's *fan-out* member (M47 Inc 3 T5; `design/finalize.md` →
    /// Rollback discipline).
    ///
    /// Both fan-out arms land through [`overlay_docs_commit_and_ff`], which stages the
    /// record alongside the promoted docs + config layer into the live index before its
    /// `git merge --ff-only`. The promotions / owner-artifact / config-layer axes each
    /// capture their own contribution; the record's was the one gap, so a boundary that
    /// refused **after** that stage (an `--ff-only` refusal over ordinary untracked WIP)
    /// left a `joined` blob staged for a milestone that never finalized — which a later
    /// plain `git commit` would land as a lying record.
    ///
    /// The match is **exhaustive by construction** (the set-fence discipline —
    /// `implementation/dev-workflow.md` → *a defect at a distance*): a new arm that stages
    /// a path into the live index must decide here, so the rollback provably covers exactly
    /// what the stage adds.
    fn live_index_record_pathspecs(&self) -> Vec<String> {
        match self {
            // The per-task stages touch no milestone record.
            StagePolicy::MigrationFixed
            | StagePolicy::IndexHonoring
            | StagePolicy::Amend
            | StagePolicy::DocOnly => Vec::new(),
            StagePolicy::Combine(_, record) | StagePolicy::ChainPerSubtask { record, .. } => {
                record.iter().cloned().collect()
            }
        }
    }
}

/// The **shared** transactional executor — a [`FinalizePlan`]'s commit phases 4–7: promote + stage + commit +
/// post-commit, returning `Ok(Ok(hook_output))` when the aggregate landed and
/// `Ok(Err(_))` when the commit was rejected (the promotions already rolled back). The
/// `hook_output` is the boundary's captured non-blocking-hook stream (empty when no hook
/// spoke): the single commit's on the per-task/`Combine` arms, and on `ChainPerSubtask`
/// every chain commit's stream folded in commit order ([`fold_hook_streams`]) — threaded
/// up so the success-relay sites (per-task T2, milestone T3) can surface it to the agent
/// (`design/finalize.md` → 6. Commit). The
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
///
/// `ignore_ack` is the transaction's **outbound** report of what the shared
/// `.jigc/.gitignore` writer did (M51 Increment 4 / T2) — filled in the closure on the
/// `displaced` / `retired` idiom, so a mid-transaction failure still hands the caller
/// whatever the writer had already done. Three doors reach this one writer (`jigc task
/// finalize` and both `jigc milestone finalize` arms) and each prints on its own surface,
/// which is why the fact leaves by the parameter list rather than by the return: the
/// landed structs those surfaces render serialize verbatim as the pinned `committed`
/// envelope, and declaring a key there is not this task's act
/// ([`crate::gitignore::emit_ack`]).
///
/// `displace` is phase 7's **keep-what-jigc-did-not-write** arm (M52 Increment 4 / T3; M53
/// Increment 1 / T4; M53 Increment 2 / T3) — an [`AreaTeardown`]: the **kind** and id of the
/// work unit whose area `cleanup_dir` is, plus the two sinks its outcome leaves by. The kind
/// travels with the id because `cleanup_dir` is a *task* area at one door and a *milestone*
/// area at two others, and the two rows of `engine::state::WorkArea` are what decides
/// membership: passing the id alone left the kind to be assumed, and the assumption was
/// hard-coded ([`post_commit`]). `None` only where no area is torn down at all.
// The shared executor threads many distinct, independent facts (repo/jigc/tmp/cleanup
// roots, the plan, schemas, the post-sweep record, the stage policy, the ignore-amend
// report it hands back); each is a real input, not incidental coupling, so an allow is
// clearer here than a parameter struct.
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
    ignore_ack: &mut Option<crate::gitignore::Ensured>,
    rollback_conflicts: &mut Vec<Finding>,
    displace: Option<AreaTeardown<'_>>,
) -> Result<Result<String>> {
    // A registry member of **both** rows (`engine::state::FINALIZE_MESSAGE_FILE`), never a
    // literal: `msg_tmp_dir` is `cleanup_dir` at all three call sites, so this transient lands
    // in the very area phase 7 tears down, and the `remove_file` below is best-effort. On the
    // arm where that removal faults — a `pre-commit` hook that `chmod 0555`s the area — a
    // non-member here is reported as a path jigc did not write and then refuses every later
    // destroying door over jigc's own file (M53 completion audit, fix 1).
    let msg_path = msg_tmp_dir.join(state::FINALIZE_MESSAGE_FILE);
    std::fs::write(&msg_path, &plan.message).with_context(|| {
        // Law 1's printed-path rule, through its one home: this fault reaches the operator,
        // and `{msg_path:?}` spelled the host path of the machine jigc ran on.
        format!(
            "could not write the commit message to `{}`",
            // The transient lands in `cleanup_dir`, which is a workbench area at all three
            // call sites — so it is spelled against the workbench root (M53, row 6).
            render::repo_relative(workbench_home(jigc_root), &msg_path),
        )
    })?;
    // The promote and retire **worktree** axes, on the shared compare-and-swap entry (M52
    // Increment 5 / T3; `crate::rollback::ROLLBACK_POPULATIONS` → `promote-destination` and
    // `retired-original`, both `Discipline::FileCas`). One family, because the two
    // populations are two disciplines only in the registry's bookkeeping sense — on disk they
    // ask the identical question, *is what is there now still what jigc left?*, and answering
    // it in one place is what keeps them from disagreeing the way the class's populations did
    // before it was enumerated. Each entry carries the destination's pre-promote bytes (or
    // *absent* for a doc the promotion creates) or the retirement's pre-deletion bytes
    // against *absence*, plus what jigc left there read back one statement after the act.
    // Empty — and therefore inert — on every arm that promotes and retires nothing.
    let mut promote_worktree =
        crate::rollback::PreImageFamily::empty(crate::rollback::FINALIZE_DOOR);
    // The pathspecs the migration stage actually `git add`ed (M40 F7) — the rollback's
    // index-axis key, so only a deletion jigc itself staged is un-staged on failure.
    // Empty on every non-migration stage arm (those have no retirements to un-stage).
    let mut staged: Vec<String> = Vec::new();
    // The third rollback axis (M45 Inc 8 T2, `design/finalize.md` → Rollback discipline, the
    // owner-artifact row): each recorded owner-artifact path's PRE-finalize index entry,
    // captured before the stage `git add` overwrites it. On a stage/commit failure the axis
    // restores exactly this — never "un-stage what jigc staged", which loses a user's
    // pre-staged blob `A` that jigc's `git add` overwrote with `B`. Empty when the plan
    // records no owner-artifact (the omitting-context inert path — every non-completion task).
    let mut owner_index: Vec<OwnerArtifactIndexEntry> = Vec::new();
    // The promotions index axis (confidence-audit sibling-hunt item 2): each promotion
    // destination's PRE-finalize index entry, captured with the same third-axis primitive
    // before the stage `git add`s the destination — so a rejected finalize restores a blob
    // the user staged at the destination mid-task (post-mint, outside the carryover
    // snapshot) instead of resetting the path to HEAD and destroying it. Empty when the
    // plan promotes nothing.
    let mut promo_index: Vec<OwnerArtifactIndexEntry> = Vec::new();
    // The FOURTH rollback axis (M45 milestone-audit fix, `design/finalize.md` → Rollback
    // discipline): the pre-finalize index state of jigc's own config-layer stage — the
    // `.jigc/version` stamp and the `.jigc/config/` + `.jigc/.gitignore` layer the stage
    // functions `git add` (and `refresh_version_stamp` rewrites). Captured before the stage,
    // restored on a stage/commit failure, so a rejected finalize leaves the index byte-identical
    // to its pre-finalize state — never a stray-staged stamp/config the owner-artifact axis (and
    // the promotions axis) did not cover.
    //
    // `Option`, not a bare `Vec`, and the distinction is load-bearing (M51 Inc 1 validation):
    // unlike the other four axes — each keyed per captured path, so an empty capture restores
    // nothing — this one's drop arm is a SET DIFFERENCE taken at rollback time ("under the
    // pathspecs now, absent from the capture → `--force-remove`"). An empty `Vec` therefore
    // reads as *the whole config layer was absent from the index pre-finalize* and drops every
    // tracked `.jigc/` entry; `None` says *the capture never ran*, which the rollback must
    // treat as "nothing staged here is mine", never as "everything here is mine".
    let mut config_index: Option<Vec<ConfigLayerIndexEntry>> = None;
    // The FIFTH staged-path family's fan-out member (M47 Inc 3 T5, `design/finalize.md` →
    // Rollback discipline, the record row): the milestone record's pre-finalize index entry.
    // Both fan-out arms reach `overlay_docs_commit_and_ff`, which `git add`s the record into
    // the LIVE index before its `--ff-only`; every other path that stage adds is covered by
    // the promotions / config-layer axes, and the record was the gap. Empty (inert) on every
    // per-task arm and dev-only (no `milestone-record` schema → no pathspec).
    let record_paths = stage.live_index_record_pathspecs();
    let mut record_index: Vec<OwnerArtifactIndexEntry> = Vec::new();
    // The config layer's WORKTREE axis (M51 Inc 4 / T3, `design/finalize.md` → Rollback
    // discipline): the pre-image bytes of the two files this transaction *rewrites* —
    // `.jigc/.gitignore` (the shared amend) and `.jigc/version` (the stamp refresh). The four
    // axes above are all index axes, so a refused finalize put the index back and left both
    // rewrites standing on disk. Unlike the config-layer INDEX axis this one needs no `Option`
    // wrapper: it is keyed per entry, so an unread family restores nothing rather than acting
    // destructively on an empty set (the type's own note).
    let mut config_worktree =
        crate::rollback::PreImageFamily::empty(crate::rollback::FINALIZE_DOOR);
    let commit_result = (|| -> Result<String> {
        // Phase 4a — the pre-images, taken FIRST (M51 Inc 1 validation; `design/finalize.md`
        // → Rollback discipline). **Every index axis's capture precedes every failure point
        // inside this closure.** The rule is over the failure-POINT axis, not over one
        // refusal: a rollback must never run against an axis that has no pre-image, because
        // "no pre-image" and "a genuinely empty pre-image" are the same value and the
        // config-layer axis acts *destructively* on the second. These four reads touch only
        // the INDEX, while promote / retire / `gitignore::ensure` below touch only the
        // WORKTREE — so taking them here is exactly as genuine a pre-finalize read as taking
        // them after, and it is genuine at every failure point, which it was not: the
        // in-closure `retire` sink refusal (and every other `?` that fired above the old
        // capture site) left the config axis holding an empty `Vec`, which then
        // `--force-remove`d the whole tracked `.jigc/` layer out of the index.
        //
        // The THIRD rollback axis: each owner-artifact path's pre-finalize index entry, taken
        // before the stage below stages (and possibly overwrites) it. Inert (empty) unless the
        // plan records an owner-artifact.
        owner_index = capture_owner_artifact_index(repo_root, &plan.owner_artifacts)?;
        // The PROMOTIONS index axis, on the same capture/restore primitive as the third: each
        // promotion destination's pre-finalize index entry, before any stage arm `git add`s it
        // (and before `promote` writes that destination in the worktree).
        let promo_paths: Vec<String> = plan
            .promotions
            .iter()
            .map(|p| p.destination.clone())
            .collect();
        promo_index = capture_owner_artifact_index(repo_root, &promo_paths)?;
        // The FOURTH axis: jigc's config-layer index, before any stage arm refreshes the stamp
        // / (re)stages the config layer. Covers every arm — the two per-task stages add these
        // paths to the live index, and the fan-out `overlay_docs_commit_and_ff` stages the
        // config layer into the live index before its `--ff-only` (where a collision aborts to
        // this same rollback). Assigned as `Some(..)`, so a failure of the capture ITSELF —
        // the one failure point a hoist cannot precede — leaves `None` and makes the rollback
        // a no-op rather than a set difference against nothing.
        config_index = Some(capture_config_layer_index(repo_root)?);
        // The FIFTH family's fan-out member: the milestone record's pre-finalize index entry,
        // on the same shared index primitive (which already models "absent", so a record not
        // yet in the index is restored to *absent*, never conjured). The record's WORKTREE
        // bytes are the `RecordFlipGuard`'s half; this is the index half the guard never
        // covered.
        record_index = capture_owner_artifact_index(repo_root, &record_paths)?;
        // …and the one WORKTREE capture, taken here for the same reason the four above are:
        // it must precede every failure point that can follow a write of its own. Promote and
        // retire run below it and touch neither file, so this read is as genuine here as it
        // would be one statement before each write — and genuine at the failure points in
        // between, which a later capture would not be.
        config_worktree = capture_config_layer_worktree(repo_root, jigc_root)?;
        // Phase 4b — promote: copy each staged managed doc to `<repo>/<destination>`,
        // capturing any displaced pre-existing destination bytes for the rollback. The
        // captures accumulate into the outer `displaced` (&mut, not returned-on-`Ok`),
        // so a MID-promote failure still hands the `Err` arm every capture taken so far
        // (confidence-audit code-review MEDIUM — the failure-POINT axis).
        promote(repo_root, &plan.promotions, &mut promote_worktree)?;
        // Retire each foreign original (`design/auto-migration.md` →
        // Retire-the-foreign-original) — the first byte-destructive write, inside the
        // commit closure so `git add --all` stages the deletion into the same commit as
        // the promoted doc. Empty (inert) on every non-migration finalize. Same
        // caller-owned capture discipline: a MID-retire failure keeps the bytes already
        // captured for the rollback's worktree axis.
        retire(repo_root, &plan.retirements, &mut promote_worktree)?;
        // The transient `.jigc/` subdirs are gitignored via `.jigc/.gitignore` — the
        // working area is never committed (`design/storage.md` → repository layout).
        // Ensure it exists so the `git add --all` stage picks up `config/` + the promoted
        // docs + the code changes.
        //
        // What the amend did goes out through the caller's `&mut` (the `displaced` /
        // `retired` idiom above), because this executor is shared by three doors and each
        // one prints on its own surface — and because the landed struct those surfaces
        // render IS the pinned `committed` envelope, which this task does not add a key to
        // (M51 Increment 4 / T2; `crate::gitignore::IGNORE_DOORS`).
        let ensured = crate::gitignore::ensure(jigc_root)?;
        // The post-write image, read back one statement after the write (M51 Inc 4 / T3) — the
        // compare-and-swap's other half, which does not exist at Phase 4a because jigc has not
        // written yet. `Unchanged` wrote not a byte, so it records nothing and the entry stays
        // *never written*: a file jigc did not touch is never rolled back, however much it
        // changes.
        if !matches!(ensured, crate::gitignore::Ensured::Unchanged) {
            config_worktree.wrote(".jigc/.gitignore");
        }
        *ignore_ack = Some(ensured);
        // The commit seam's typed subject: this finalize commits in the USER's checkout,
        // and the subject records which ref that was when the act was decided
        // (`crate::repo::SeamSubject`). The two fan-out arms below never reach this one —
        // they commit in a dedicated worktree and say so from its handle.
        let live = SeamSubject::live(repo_root);
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
                staged = stage_migration(repo_root, plan, &mut config_worktree)?;
                gate_owner_artifacts_post_stage(repo_root, cleanup_dir, schemas, plan)?;
                git_commit(&live, &msg_path)
            }
            // Per-task IndexHonoring (M30 G6): honor the agent's existing index and add
            // ONLY jigc's promoted docs + the config layer into it; never sweep the
            // ambient dirty tree. The whole-index `git_commit` lands the lot.
            StagePolicy::IndexHonoring => {
                stage_index_honoring(repo_root, plan, &mut config_worktree)?;
                // M45 Inc 8 T2 — the #5 owner-artifact gate runs HERE, after the stage
                // (`design/finalize.md` → 5. Stage: the gate moves after the stage, not
                // relaxes its `tracked` clause). A block returns `OwnerArtifactBlock`, so the
                // shared `Err` arm below rolls back (promotions + the third index axis) and
                // the per-task surface routes it through `self.blocked()` at exit 3.
                gate_owner_artifacts_post_stage(repo_root, cleanup_dir, schemas, plan)?;
                git_commit(&live, &msg_path)
            }
            // The amend arm (F-10): stage nothing, and rewrite `HEAD` from the index the
            // gate has already required to be empty. No `gate_owner_artifacts_post_stage`
            // either — that gate adjudicates what a stage put in the commit, and this arm
            // stages nothing.
            StagePolicy::Amend => git_commit_amend(&live, &msg_path),
            // The doc-only arm (M55): stage the task's own path set, run the same post-stage
            // owner-artifact gate the index-honoring arm runs, and commit exactly that set —
            // every other staged path stays staged, and the rollback axes above (promotions,
            // owner-artifacts) restore exactly what this stage touched.
            StagePolicy::DocOnly => {
                let paths = stage_doc_only(repo_root, plan)?;
                gate_owner_artifacts_post_stage(repo_root, cleanup_dir, schemas, plan)?;
                crate::milestone::git_commit_paths(&live, &msg_path, &paths)
            }
            // The `squash: true` fan-out boundary (M31 Inc 4 / Inc 5): fold the N
            // worktree-staged code-sets onto the base tree off-line, overlay the promoted docs
            // + config, and commit the combined tree with the user's hooks running from a
            // dedicated worktree — never `git add --all` (the code lives in the isolated
            // worktrees, not this checkout, so a sweep would drop it).
            StagePolicy::Combine(worktrees, record) => {
                combine_commit(repo_root, &worktrees, record.as_deref(), plan, &msg_path)
            }
            // The `squash: false` honest-rework boundary (M31 — WIP-safe): build the N+1
            // commit chain in a dedicated worktree (one commit per sub-task carrying its own
            // code with hooks running, then the merged-docs aggregate) and fast-forward main
            // — never touching the live checkout, so an abort leaves unrelated WIP intact
            // (no `git reset --hard`). Returns EVERY chain commit's captured hook stream
            // folded into one string (the hook_output producer axis), so the success-relay
            // site's one relay + envelope carry all N+1.
            StagePolicy::ChainPerSubtask { subtasks, record } => {
                chain_commit(repo_root, &subtasks, record.as_deref(), plan, &msg_path)
            }
        }
    })();
    let _ = std::fs::remove_file(&msg_path);
    let hook_output = match commit_result {
        Ok(hook_output) => hook_output,
        Err(err) => {
            // Roll back phases 4–5 (`design/finalize.md` → Rollback discipline): restore
            // each promotion destination's captured pre-finalize index entry + undo
            // promote's worktree write, and restore what the retire/stage themselves
            // touched (review B1; scoped M40 F7) — so an approved-but-failed commit never
            // leaves the foreign file deleted with no commit, never resurrects a user's
            // own pre-staged deletion, and never destroys a blob the user staged at a
            // destination mid-task (the promotions index axis).
            let mut conflicts = rollback_promotions(
                repo_root,
                jigc_root,
                &plan.promotions,
                &promo_index,
                &promote_worktree,
                &plan.retirements,
                &staged,
            );
            // The third scoped axis (M45 Inc 8 T2): restore each owner-artifact path's
            // captured pre-finalize index entry, so a failed finalize leaves the index
            // byte-identical to its pre-finalize state — including a user's pre-staged blob.
            rollback_owner_artifact_index(repo_root, &owner_index);
            // The fourth scoped axis (M45 milestone-audit fix): restore jigc's own config-layer
            // stage — the `.jigc/version` stamp `refresh_version_stamp` rewrote and the config
            // layer the stage (re)added — so the stamp/config entries are never left staged.
            rollback_config_layer_index(repo_root, config_index.as_deref());
            // The fifth family's fan-out member (M47 Inc 3 T5): restore the milestone
            // record's captured pre-finalize index entry, so a refused boundary leaves no
            // `joined` blob staged for a milestone that never finalized (the
            // `RecordFlipGuard` restores the worktree bytes to `active`; without this the
            // index kept the flipped blob and the next plain `git commit` landed a lying
            // record). Inert on every arm that stages no record.
            rollback_owner_artifact_index(repo_root, &record_index);
            // The config layer's WORKTREE axis (M51 Inc 4 / T3): put the two rewritten files
            // back — compare-and-swap, so a file something else changed inside the transaction
            // is left exactly as that editor left it and its pre-image is preserved beside it.
            // Each such path hands back one blocking `finalize.rollback-conflict`, which the
            // door prints beside its own frame; the door's error is never replaced.
            // …joined by the promote/retire worktree axis's own conflicts, gathered above:
            // one blocking finding per raced path, whichever population the path belongs to,
            // because the operator's act is the same comparison in every case.
            conflicts.extend(config_worktree.restore(repo_root, jigc_root));
            *rollback_conflicts = conflicts;
            // Every rollback above has run, so the state each door's frame describes is the
            // state that is now on disk — and the error is marked as a commit-transaction
            // failure so the door frames it instead of dropping the frame (N20). A hook
            // rejection, a stage failure and a routed refusal pass through unmarked.
            return Ok(Err(mark_commit_failure(err)));
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
        displace,
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

/// Fold N captured hook streams — one per landed commit of a multi-commit boundary
/// (the `squash: false` N+1 chain, `add-from-spec`'s per-sub-task record commits) —
/// into the ONE string the caller relays and envelopes: trimmed non-empty chunks
/// joined by a newline, commit order preserved. Folding (rather than relaying each
/// chunk as it lands) keeps the one-capture-two-channels property
/// (`design/command-output-contract.md` → Stream discipline): the `hook_output`
/// envelope key and the stderr relay carry the *same* string.
pub(crate) fn fold_hook_streams<'a, I: IntoIterator<Item = &'a str>>(streams: I) -> String {
    streams
        .into_iter()
        .map(str::trim)
        .filter(|chunk| !chunk.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Phase 4 (`design/finalize.md` → 4. Promote managed docs). Copy each staged managed
/// doc from its source to its canonical repo path (`<repo_root>/<destination>`) —
/// **copy, not move**, so rollback is a removal of the copies and the working area stays
/// intact. Creates the destination's parent directory (e.g. `decisions/`) when absent.
///
/// **Captures the destination's pre-image** keyed by repo-relative destination
/// (confidence-audit minor item 10 — the retire byte-capture discipline, applied to the
/// promote): a **same-path** migration (`destination == source`, the M43 carve-out)
/// plans no retirement, so when the foreign original is **untracked** the promotion
/// destination is the user's only copy of its bytes and `git restore` has nothing to
/// recover from — the captured bytes are what [`rollback_promotions`] restores so a
/// rejected commit never deletes them.
///
/// **Since M52 Increment 5 / T3 the capture is a [`crate::rollback::PreImage`] and the
/// restore is compare-and-swap** (`rollback::ROLLBACK_POPULATIONS` → `promote-destination`,
/// `Discipline::FileCas`). Two things follow, and both are the point. The **post-image is
/// read back one statement after the copy**, which is what lets the rollback tell *jigc's own
/// promoted bytes* from a third party's edit made inside the transaction — an unconditional
/// rewrite destroyed that edit silently. And an **unreadable** destination now refuses the
/// transaction instead of promoting over it: recording `pre` as *absent* there would make the
/// rollback **delete** a file this run did not create, which is the loss in the other
/// direction (`PreImage::capture`'s own rule — absent means absent, never unreadable).
///
/// The entries accumulate into the **caller-owned** family (confidence-audit
/// code-review MEDIUM — the failure-POINT axis): a mid-promote failure at promotion *k*
/// (disk full, permissions, a directory squatting the destination) must not discard the
/// captures already taken for promotions 1..k — a returned-only-on-`Ok` collection did,
/// and the shared `Err` arm's rollback then deleted a same-path untracked foreign it had
/// no bytes to restore. Each capture is pushed **before** its `fs::copy`, so even the
/// failing promotion's own pre-image reaches the rollback.
fn promote(
    repo_root: &Path,
    promotions: &[Promotion],
    worktree: &mut crate::rollback::PreImageFamily,
) -> Result<()> {
    for promotion in promotions {
        let dest = repo_root.join(&promotion.destination);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("could not create {parent:?} to promote into"))?;
        }
        worktree.push(
            crate::rollback::PreImage::capture(promotion.destination.clone(), dest.clone())
                .with_context(|| format!("could not read {dest:?} before promoting over it"))?,
        );
        let copied = std::fs::copy(&promotion.source, &dest);
        // The post-image, read back one statement after the write — the swap's other half,
        // and the only moment at which the bytes on disk are provably jigc's. Read on the
        // FAILING arm too, before the `?`: a `fs::copy` that errs part-way through leaves a
        // truncated file at the destination, and those bytes are as much jigc's doing as a
        // whole one's. Read only after an `Ok` they would be `Untouched` — *jigc cannot prove
        // what it put there* — and the swap would leave the truncation standing. A copy that
        // failed without touching the file reads back as its own pre-image, which the
        // `pre == post` rule already treats as nothing having happened.
        worktree.wrote(&promotion.destination);
        copied.with_context(|| {
            format!(
                "could not promote {:?} to {dest:?}",
                promotion.source.display()
            )
        })?;
    }
    Ok(())
}

/// A retirement whose recorded path has been **adjudicated against the repository it is about
/// to be unlinked from** — the only value [`retire`] will act on (M51 Increment 1 / T3;
/// `completions/artifacts/M51/settle-record.md` → §2, the sink).
///
/// **Why a second adjudication at all.** T1 made the `jigc migrate` door refuse a `<path>` it
/// cannot record, which closes the door and nothing else. What the door produces is a line in
/// a **mutable, gitignored working area** (`.jigc/tasks/<id>/source-path`), and one commit
/// closure later [`retire`] reads it back and hands it to `std::fs::remove_file`. Between
/// those two moments the value is caller-supplied *again*, so a guard that only ran at the
/// door is a guard on the typing, not on the deletion. Driven at `93095bd`: rewriting that
/// file to an absolute path outside the repository and running `finalize --approve` deleted a
/// file in another tree at **exit 0**, inside a commit that said it had migrated a document.
///
/// **The two fields are what makes it a value and not a re-check.** The *normalized literal
/// path* is the repo-relative spelling [`crate::trackable::resolve_source_token`] answered
/// with — not the recorded string, which may spell the same file differently or no file at
/// all — and the *bound repository identity* is the root that answer was computed against, so
/// [`Self::target`] joins against the root that adjudicated it rather than whichever root
/// happens to be in scope at the call. `engine::state::MigrationSource` is the other half of
/// the pair: it carries provenance (*these bytes were recorded as a migration source*) and
/// deliberately not admissibility, because admissibility needs `git` and a symlink syscall and
/// the engine hosts neither.
///
/// **Why the predicate lives CLI-side**, stated rather than assumed: `crates/engine` contains
/// no `Command::new` and no `symlink_metadata`/`read_link` — it is the deterministic core, and
/// asking git who owns a path is not a deterministic-core act. The engine's own precedent for
/// a CLI-supplied predicate is `engine::validate`'s injected `tracked`.
#[derive(Debug)]
struct ValidatedRetirement {
    /// The normalized, repo-relative literal path to unlink.
    path: String,
    /// The repository this adjudication was bound to — the root [`Self::target`] joins against.
    repo_root: PathBuf,
}

impl ValidatedRetirement {
    /// Adjudicate one planned retirement, answering with the value [`retire`] may unlink — or
    /// with the blocking [`Finding`] that refuses the whole transaction.
    ///
    /// **Two questions, in this order.**
    ///
    ///   1. **Git pathspec magic** — a recorded path git reads as a pattern rather than a
    ///      name: the `:` prefix (`:(top)`, `:!`) or a wildmatch byte (`*`, `?`, `[`, `\`)
    ///      anywhere in it. `git add -- <path>` prevents **option** parsing and nothing else,
    ///      so such a token reaching [`stage_migration`] is a pathspec and stages a set nobody
    ///      named.
    ///   2. **[`crate::trackable::resolve_source_token`]** — the door's own rule, asked again
    ///      of the recorded value: the magic question above, then resolve inside the
    ///      repository or refuse an absolute one outright, then git's `.git`-component and
    ///      git-dir rules, ownership and the index's gitlinks, then jigc's own workbench, then
    ///      a symlinked component. One home, two callers: the door and the sink cannot get
    ///      different answers about the same path, which is the property that makes "the door
    ///      already checked it" a safe thing for a reader to believe.
    ///
    ///      **The magic leg moved INTO that home** (M51 Increment 1, the axis fix). It was a
    ///      sink-local `recorded.starts_with(':')` on the stated ground that the door refuses
    ///      the same token anyway *"for the unrelated reason that it names no readable file"*.
    ///      That ground is false for **every readable spelling of the class**, and it is a
    ///      claim about a *fixture* rather than about the rule: it held only of the one token
    ///      the axis suite planted, `:(top)README.md`, which names nothing. A file whose name
    ///      is literally `*.md` **is** readable, so the door read it, asked its trackedness
    ///      leg a `git ls-files -- '*.md'` that answered about six other files, and minted at
    ///      exit 0 — after which `--approve` unlinked bytes no git object held. So is a file
    ///      named `:colon.md`: staged, the door asked the same leg and reported
    ///      `migrate.source-untracked` — *"in neither this repository's index nor its HEAD"* —
    ///      about a path `git ls-files --stage` printed, routing to a `git add -- :colon.md`
    ///      that exits **128**; committed, it was admitted outright and the refusal arrived
    ///      one authoring later, here. One spelling of a rule is not the rule, so the class
    ///      is asked once, in the home both callers share — and the axis suite's colon cell
    ///      now plants a readable colon-named file, so the premise cannot be restated as a
    ///      green cell.
    fn adjudicate(repo_root: &Path, recorded: &Path, state: StateTruth) -> Result<Self, Finding> {
        let recorded = recorded.to_string_lossy();
        // **The base is `repo_root`, not a cwd** (M53 — the cwd census, C2-03). The value
        // adjudicated here is the one the migrate door already *recorded*, and what it
        // recorded is the clean repo-relative spelling — an identity, never the operator's
        // typed token. Re-joining it against a cwd would resolve a stored identity as if it
        // were a fresh argument, and the sink would then unlink a path nobody named.
        let path = crate::trackable::resolve_source_token(repo_root, repo_root, &recorded)
            .map_err(|reason| retire_untrackable_finding(&recorded, state, reason))?;
        Ok(Self {
            path,
            repo_root: repo_root.to_path_buf(),
        })
    }

    /// The absolute path to unlink — the adjudicated repo-relative spelling joined against the
    /// **bound** root. Joining the *recorded* string instead is the hole itself: in Rust
    /// `root.join(<absolute>)` **is** that absolute path, so a recorded host path silently
    /// escaped the repository at the one call that mattered.
    fn target(&self) -> PathBuf {
        self.repo_root.join(&self.path)
    }
}

/// Which door raised [`retire_untrackable_finding`] — and therefore **what is true of the
/// repository at the moment it prints** (M47's per-door state-truth clause). One code and one
/// repair, two states: the review hold has promoted nothing to roll back, and saying otherwise
/// would be the door narrating a transaction it never opened.
#[derive(Debug, Clone, Copy)]
enum StateTruth {
    /// [`TaskArea::finalize`]'s exit-4 review hold — a pure read that writes nothing.
    ReviewHold,
    /// [`retire`], inside the commit closure — the promote is already undone by the caller's
    /// rollback when this finding travels out.
    RolledBack,
}

impl StateTruth {
    /// The leading clause of the route: what this run did, in this run's own terms.
    fn clause(self) -> &'static str {
        match self {
            Self::ReviewHold => {
                "nothing was committed — this run is the review hold, which writes nothing"
            }
            Self::RolledBack => "nothing was committed and the promote was rolled back",
        }
    }
}

/// The refusal raised by the sink and by the review hold that forecasts it
/// (`settle-record.md` → §10's table row; `design/validation.md` → the
/// finding inventory): one blocking code for every reason, the reason carried in the message
/// on `config.untrackable-root`'s precedent, and a [`engine::finding::Route::human`] because
/// no `jigc` argv resolves this state — the recorded value is task state the operator repairs
/// or abandons, and a route the state refuses is worse than none (M50's `write.not-present`
/// lesson).
///
/// It is **not** the door's `migrate.source-untrackable`. That code's subject is an argument
/// the operator just typed and can retype; this one's is task state written by a door which
/// had already adjudicated it, so reaching this finding means something changed the value
/// afterwards. A different act repairs it, so it is a different identity.
///
/// **It keys at the recorded path** ([`FinalizeSubject::FilePath`](crate::render::FinalizeSubject)),
/// which is the one form that discriminates: two inadmissible retirements in one plan are two
/// findings, not one `(code, null)`. The quoted value may have no repo-real spelling — that is
/// precisely what the finding says about it — and it is also the only string an operator can
/// search their working area for, so law 1's printed-path rule is met the way the `jigc
/// migrate` door meets it (`design/surface-contract.md`; the `locate::not_in_repo_message`
/// case), by naming the subject that has no relative form rather than inventing one.
fn retire_untrackable_finding(
    recorded: &str,
    state: StateTruth,
    reason: impl Into<String>,
) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.retire-untrackable",
        format!(
            "this migration's recorded source is not a path this repository can retire: {}",
            reason.into()
        ),
        Some(Location::addressed(recorded.to_string(), 1, 1)),
        Some(engine::finding::Route::human(format!(
            "{}. The retire target is recorded in the task's working area at \
             `source-path`: restore it to the path `jigc migrate` recorded, or abandon \
             the migration and re-run `jigc migrate` against a file inside this repository",
            state.clause(),
        ))),
    )
}

/// The retire step (`design/auto-migration.md` → Retire-the-foreign-original) — the
/// first byte-destructive write on a repo file. Adjudicate each planned retirement
/// ([`ValidatedRetirement`]) and remove it so the subsequent `git add --all` stages the
/// deletion into the same commit as the promoted managed doc.
///
/// **The adjudication is inside this loop on purpose** (M51 Increment 1 / T3,
/// `settle-record.md` → §2): one statement above the unlink, not once before the commit
/// closure is entered — a check at the closure's mouth spans promotion, staging and hooks
/// and widens the window in which the recorded value can be substituted for another. An
/// inadmissible retirement is a refusal of the **whole transaction**, not of one step: the
/// `Err` propagates out of the closure, the caller's rollback restores the promote and the
/// captured pre-images, and nothing is committed. Runs inside the commit closure, so a failure
/// aborts the transaction (rolling back the promotions). An already-absent path is not
/// an error (idempotent — the goal is the file gone). Empty on every non-migration
/// finalize, so this is inert there.
///
/// **Captures the deleted bytes** keyed by repo-relative path (review F3): a foreign
/// original that was **untracked** at HEAD has no committed bytes for `git restore` to
/// recover on a rollback, so the captured bytes are what `rollback_promotions` rewrites
/// to keep an approved-but-failed commit from permanently losing it.
///
/// **Since M52 Increment 5 / T3 the capture is a [`crate::rollback::PreImage`] whose
/// post-image is *absence*** (`crate::rollback::PostWrite::Removed`), so the restore is the
/// same compare-and-swap every other `FileCas` population runs: the bytes come back **only
/// while the path is still absent**, and a third party who re-created it inside the
/// transaction keeps their file *and is told* — before, that racer kept their file silently
/// while jigc's own capture was dropped on the floor and the deletion of the original bytes
/// survived, named by nothing (`rollback::ROLLBACK_POPULATIONS` → `retired-original`;
/// baseline-rollback.md §1 table A row 3, *"jigc's deletion survives silently"*).
///
/// The captures accumulate into the **caller-owned** family (confidence-audit
/// code-review MEDIUM — the same failure-POINT axis as [`promote`]): a mid-retire
/// failure at retirement *k* must not discard the captures for retirements already
/// deleted — a returned-only-on-`Ok` collection did, leaving the rollback's worktree
/// axis nothing to rewrite for an untracked foreign it had just deleted.
fn retire(
    repo_root: &Path,
    retirements: &[PathBuf],
    worktree: &mut crate::rollback::PreImageFamily,
) -> Result<()> {
    for retirement in retirements {
        // The sink's own adjudication (M51 Increment 1 / T3), asked HERE — one statement
        // above the unlink, in the same function as it, so nothing runs between the answer
        // and the act it authorizes.
        let validated =
            ValidatedRetirement::adjudicate(repo_root, retirement, StateTruth::RolledBack)
                .map_err(|finding| crate::render::finding_error(&finding))?;
        let path = validated.target();
        match std::fs::read(&path) {
            Ok(bytes) => {
                std::fs::remove_file(&path)
                    .with_context(|| format!("could not retire the foreign original {path:?}"))?;
                // Pushed AFTER the unlink succeeded, carrying the bytes the read above
                // already holds — the two halves of one act. The entry's post-image is
                // *absence*, so an entry pushed before a removal that then FAILED would
                // claim jigc had emptied a path still holding its own bytes, and the swap
                // would raise a conflict and park a pre-image over a file jigc never touched.
                // The failure-POINT discipline is untouched: this family is the caller's, so
                // a mid-loop failure at retirement *k* keeps every entry 1..k-1, and there is
                // nothing at *k* to restore.
                worktree.push(crate::rollback::PreImage::removed(
                    retirement.to_string_lossy().into_owned(),
                    path.clone(),
                    bytes,
                ));
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
    Ok(())
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
///
/// **Returns the staged-pathspec set** it fed to `git add` (M40 F7) — the key for
/// [`rollback_promotions`]'s index axis: on a commit failure, only a deletion *this
/// stage* staged is un-staged, so a user's pre-staged `git rm` (skipped above, never in
/// this set) is never resurrected in the index.
fn stage_migration(
    repo_root: &Path,
    plan: &engine::finalize::FinalizePlan,
    config_worktree: &mut crate::rollback::PreImageFamily,
) -> Result<Vec<String>> {
    let mut pathspecs: Vec<String> = Vec::new();
    for promotion in &plan.promotions {
        pathspecs.push(promotion.destination.clone());
    }
    for retirement in &plan.retirements {
        let spec = retirement
            .to_str()
            .with_context(|| format!("retirement path {retirement:?} is not valid UTF-8"))?;
        // [`retire`] has already deleted the original from the worktree. Stage that
        // deletion only when the path is still in the **index** (M40 F7,
        // `design/finalize.md` → M40 refinement item 1) — a `git add` pathspec that
        // matches nothing is a fatal error, whereas the blanket `git add --all`
        // tolerated it. Two skip shapes, both lossless: an untracked foreign was never
        // in the index (nothing to stage), and a user's pre-staged `git rm` already
        // removed the index entry (the commit is whole-index, so the staged deletion
        // lands regardless). A HEAD discriminator can't see the pre-staged shape — HEAD
        // still carries the path — and kept the fatal pathspec.
        if path_in_index(repo_root, spec) {
            pathspecs.push(spec.to_owned());
        }
    }
    refresh_version_stamp(repo_root, config_worktree)?;
    pathspecs.extend(existing_pathspecs(
        repo_root,
        &jigc_config_layer_pathspecs(),
    ));
    // M45 Inc 8 T2 — stage each recorded owner-artifact too (existence + non-ignore guarded;
    // see [`stage_index_honoring`]). These join the returned set — which
    // [`rollback_promotions`] consults only for *retirement* paths, so an owner-artifact in it
    // is never un-staged by the retire index axis; its own rollback is the executor's third
    // axis (the pre-finalize index capture/restore).
    pathspecs.extend(owner_artifact_stage_specs(repo_root, plan));
    let mut args: Vec<&str> = vec!["add", "--"];
    args.extend(pathspecs.iter().map(String::as_str));
    git_run(repo_root, &args).map_err(mark_stage_failure)?;
    Ok(pathspecs)
}

/// The recorded owner-artifact paths ([`engine::finalize::FinalizePlan::owner_artifacts`])
/// that are safe to `git add` — a **stageable shape** (repo-relative, `..`-free — see
/// [`owner_artifact_is_stageable_shape`]), present on disk, **and not gitignored** (M45 Inc 8
/// T2). Each skipped shape stays unstaged and the post-stage [`owner_artifacts_gate`] blocks
/// on it (absolute / `..` → misplaced; absent → names-no-file; gitignored → untracked), so the
/// gate — not this stage — is the authority on path safety. Skipping the unsafe shapes is also
/// what keeps `git add` from fataling on an out-of-repo pathspec. Duplicate paths are harmless
/// to `git add`, so no dedup.
fn owner_artifact_stage_specs(
    repo_root: &Path,
    plan: &engine::finalize::FinalizePlan,
) -> Vec<String> {
    plan.owner_artifacts
        .iter()
        .filter(|path| {
            owner_artifact_is_stageable_shape(path)
                && repo_root.join(path).exists()
                && !path_is_ignored(repo_root, path)
        })
        .cloned()
        .collect()
}

/// Whether an owner-artifact path is shaped for a safe `git add` / index probe — **repo-
/// relative** (not absolute) and free of `..` components. finalize's stage + third-axis
/// capture skip any other shape: an absolute or climbing path is outside the index and would
/// make `git add` / `git ls-files` **fatal** (`git ls-files -- /etc/passwd` → "outside
/// repository"), erroring the transaction before the gate can speak. The post-stage
/// [`owner_artifacts_gate`] is the authority that BLOCKS such a path
/// (`owned_location_violation` → misplaced), so this guard only keeps the plumbing from
/// crashing ahead of that verdict — it never *replaces* the gate's safety adjudication.
fn owner_artifact_is_stageable_shape(path: &str) -> bool {
    let p = Path::new(path);
    !p.is_absolute()
        && !path.starts_with('/')
        && !p
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
}

/// Whether `path` (repo-relative) is ignored by a `.gitignore` rule — `git check-ignore -q`
/// exits 0 when ignored, 1 when not. On any other outcome (git error) default **not
/// ignored**: the subsequent `git add` then decides, keeping today's behavior. Used to keep
/// the owner-artifact stage from feeding `git add` a gitignored path (fatal without `-f`).
fn path_is_ignored(repo_root: &Path, path: &str) -> bool {
    Command::new("git")
        .args(["check-ignore", "-q", "--", path])
        .current_dir(repo_root)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Run the engine's #5 owner-artifact presence gate
/// ([`engine::validate::owner_artifacts_gate`]) over the task's just-staged working area,
/// with a fresh `tracked` predicate built over the **post-stage** index (M45 Inc 8 T2). This
/// is the gate's relocated home — after phase-5 staging, so a finalize that stages the
/// recorded artifact satisfies it in the same transaction (`design/finalize.md` → 5. Stage).
/// A blocking result becomes an [`OwnerArtifactBlock`] error, so the shared executor rolls
/// back (promotions + the third index axis) and the per-task surface routes it through
/// `self.blocked()` (exit 3). Still the gate's only *authoritative* home — it is the one
/// run over the real post-stage index. Since M47 Inc 4 / T2 the six staging-independent
/// causes are also **previewed** at `task validate` under a constant-true predicate
/// ([`TaskArea::preview_gates`]), so the validate-0 / finalize-3 split is now **cause 7's
/// alone**: untracked is the one verdict no preview could have shown. Inert (a no-op, no git
/// shell-out) when the plan records no owner-artifact — every non-completion task.
fn gate_owner_artifacts_post_stage(
    repo_root: &Path,
    task_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    plan: &engine::finalize::FinalizePlan,
) -> Result<()> {
    if plan.owner_artifacts.is_empty() {
        return Ok(());
    }
    // The post-stage tracked-status predicate: the inverse of `git ls-files --others` (all
    // untracked, ignored included — a gitignored path reads untracked, so the gate blocks).
    // Read AFTER the stage, so a just-staged owner-artifact reads tracked and the gate passes
    // (mirrors `Task::tracked_predicate`, rebuilt here because the executor is a free fn).
    let untracked: std::collections::HashSet<String> = git_untracked_all(repo_root)?
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect();
    let tracked = move |path: &str| !untracked.contains(path);
    let findings = owner_artifacts_gate(task_dir, schemas, repo_root, &tracked)?;
    if findings.iter().any(|f| f.severity == Severity::Blocking) {
        return Err(anyhow::Error::new(OwnerArtifactBlock(findings)));
    }
    Ok(())
}

/// Typed marker for a **post-stage owner-artifact block** (M45 Inc 8 T2) — the relocated #5
/// gate found a recorded artifact absent / unsafe / present-but-untracked *after* the stage.
/// Carried through [`try_execute_finalize_plan`]'s error channel so the executor's shared
/// `Err` arm runs the rollback (promotions + the third index axis) and the per-task surface
/// (which holds the task id) can route it through `self.blocked()` at exit 3 — a **validation**
/// block, distinct from a [`StageGitFailure`] (git error) or a [`CommitRejected`] (hook).
#[derive(Debug)]
struct OwnerArtifactBlock(Vec<Finding>);

impl std::fmt::Display for OwnerArtifactBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} owner-artifact finding(s) blocked the finalize after staging",
            self.0.len()
        )
    }
}

impl std::error::Error for OwnerArtifactBlock {}

/// One path's pre-finalize index entry — the third rollback axis's capture (M45 Inc 8 T2,
/// `design/finalize.md` → Rollback discipline, the owner-artifact row), reused by the
/// promotions index axis for each promotion destination (confidence-audit sibling-hunt
/// item 2 — one primitive, two staged-path families).
/// `entry` is `Some((mode, blob-sha))` when the path was in the index before finalize staged
/// it (e.g. a user's pre-staged blob `A`), `None` when it was absent from the index.
pub(crate) struct OwnerArtifactIndexEntry {
    path: String,
    entry: Option<(String, String)>,
}

/// Capture each path's pre-finalize index entry via `git ls-files --stage` (M45 Inc 8 T2 —
/// the third rollback axis; also fed the promotion destinations for the promotions index
/// axis, and — since M47 Inc 2 T1 — the **fifth** staged-path family, the milestone
/// record-only doors ([`crate::milestone`]'s record pre-image), which is why this is
/// `pub(crate)` rather than a parallel primitive). The output is `<mode> <sha> <stage>\t<path>`
/// when the path is in the index, empty when absent. Called before the stage `git add`
/// overwrites the entry, so a stage/commit failure can restore *exactly* what was there —
/// not drop to HEAD, not keep jigc's overwrite. Empty `paths` → empty capture (inert).
pub(crate) fn capture_owner_artifact_index(
    repo_root: &Path,
    paths: &[String],
) -> Result<Vec<OwnerArtifactIndexEntry>> {
    let mut captured = Vec::with_capacity(paths.len());
    for path in paths {
        // An unsafe shape (absolute / `..`) is never in the index and would make `git
        // ls-files -- <path>` fatal ("outside repository"); it is never staged either, so
        // there is nothing to capture or restore — skip it (the gate blocks it as misplaced).
        if !owner_artifact_is_stageable_shape(path) {
            continue;
        }
        let line = git_capture(repo_root, &["ls-files", "--stage", "--", path])?;
        let fields: Vec<&str> = line.split_whitespace().collect();
        let entry = if fields.len() >= 2 {
            Some((fields[0].to_owned(), fields[1].to_owned()))
        } else {
            None // absent from the index pre-finalize.
        };
        captured.push(OwnerArtifactIndexEntry {
            path: path.clone(),
            entry,
        });
    }
    Ok(captured)
}

/// Restore each captured path's pre-finalize index entry on a stage/commit failure — the
/// third scoped rollback axis (M45 Inc 8 T2), also the index half of [`rollback_promotions`]
/// (the promotions index axis). Best-effort (the commit did not
/// land, so a restore failure is logged, never raised — mirroring [`rollback_promotions`]).
///
/// - **Present** → `git update-index --cacheinfo <mode> <sha> <path>` sets the index back to
///   the captured blob **without touching the worktree** — so a user's on-disk edit `B`
///   survives while `git show :<path>` == the pre-staged blob `A` (the exact case the
///   carryover exemption exists for; a plain `git restore --staged` would drop to HEAD and
///   lose `A`).
/// - **Absent** → `git update-index --force-remove <path>` drops the entry jigc's stage
///   added, returning the index to its pre-finalize "not staged" state (the worktree file
///   stays).
pub(crate) fn rollback_owner_artifact_index(
    repo_root: &Path,
    captured: &[OwnerArtifactIndexEntry],
) {
    for item in captured {
        match &item.entry {
            Some((mode, sha)) => {
                let _ = git_run(
                    repo_root,
                    &["update-index", "--cacheinfo", mode, sha, &item.path],
                );
            }
            None => {
                let _ = git_run(repo_root, &["update-index", "--force-remove", &item.path]);
            }
        }
    }
}

/// Which root a rewritten config-layer file hangs off — because the **two writers do not
/// agree**, and a pre-image taken from the wrong root would restore a file nobody wrote.
/// `gitignore::ensure` is handed `jigc_root` (the `.jigc/` the CLI resolved, which in a
/// fan-out is the MAIN checkout's, the worktree binding to it), while
/// [`refresh_version_stamp`] writes under `repo_root`. In the ordinary single-checkout case
/// the two coincide; the enum exists so the case where they do not is decided rather than
/// assumed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigLayerHome {
    /// `<jigc_root>/<file name>` — the path `crate::gitignore::ensure` writes.
    JigcRoot,
    /// `<repo_root>/<spec>` — the path `crate::setup::write_version_stamp` writes.
    RepoRoot,
}

/// Whether `finalize` **rewrites** this config-layer path in the worktree — and therefore
/// owes it a worktree pre-image — or does not, carrying the reason it does not.
///
/// The disposition is the point (M51 Increment 4 / T3; `settle-record.md` → §6): the family's
/// subject is *the files finalize rewrites*, and the Settle names the trap by name — capture
/// a **directory** and the family silently becomes a different shape (absent-means-delete
/// over N files). So every pathspec the stage adds answers this question, and a pathspec that
/// arrives without an answer reddens
/// `crates/cli/tests/config_layer_preimage.rs::every_config_layer_pathspec_carries_a_disposition`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigLayerWrite {
    /// `finalize` writes this file inside the commit closure, so the transaction captures its
    /// pre-image before the write and restores it (compare-and-swap) on a failure.
    Rewritten {
        /// Which root the writer hangs it off ([`ConfigLayerHome`]).
        home: ConfigLayerHome,
    },
    /// `finalize` stages this path but writes none of its bytes, with the reason.
    Untouched(&'static str),
}

/// One jigc-owned config-layer pathspec: what the stage `git add`s, and whether `finalize`
/// rewrites it.
pub struct ConfigLayerSpec {
    /// The repo-relative pathspec, exactly as the stage passes it to `git add`.
    pub spec: &'static str,
    /// Whether `finalize` rewrites it in the worktree.
    pub write: ConfigLayerWrite,
}

/// The **single source of truth** for the jigc-owned config-layer paths every
/// non-migration/migration stage `git add`s into the index: the git-tracked config layer
/// (`.jigc/config/` + `.jigc/.gitignore`, which `setup` writes but never commits, so the
/// first finalize lands them) and the `.jigc/version` binary-provenance stamp (which
/// [`refresh_version_stamp`] rewrites to the running build before staging). Defined once so
/// the rollback (the **fourth axis** — [`capture_config_layer_index`] /
/// [`rollback_config_layer_index`]) provably restores *exactly* the set the two stage
/// functions ([`stage_migration`] / [`stage_index_honoring`]) contribute; growing the staged
/// set here grows what the rollback covers (the set-fence discipline — `implementation/
/// dev-workflow.md` → *a defect at a distance*).
///
/// **Since M51 it carries the worktree disposition too**, because the set-fence above was
/// only ever about the *index*: two of these three paths are also **rewritten on disk** by
/// the transaction, and for a whole milestone nothing rolled that back. The rows are what
/// [`capture_config_layer_worktree`] reads, so the two families share one enumeration and a
/// fourth pathspec cannot join one without answering the other.
pub const CONFIG_LAYER_SPECS: &[ConfigLayerSpec] = &[
    ConfigLayerSpec {
        spec: ".jigc/config",
        write: ConfigLayerWrite::Untouched(
            "a DIRECTORY, and the only one here: the stage adds its files, the transaction \
             writes none of them. A worktree pre-image over a directory is a different shape \
             — absent-means-delete over N files, including files this run never saw — so it \
             is declared out rather than captured as if it were a file \
             (`settle-record.md` → §6/A1)",
        ),
    },
    ConfigLayerSpec {
        spec: ".jigc/.gitignore",
        write: ConfigLayerWrite::Rewritten {
            home: ConfigLayerHome::JigcRoot,
        },
    },
    ConfigLayerSpec {
        spec: crate::setup::VERSION_STAMP_PATH,
        write: ConfigLayerWrite::Rewritten {
            home: ConfigLayerHome::RepoRoot,
        },
    },
];

/// The pathspecs of [`CONFIG_LAYER_SPECS`], in declaration order — the argument list the two
/// stage functions and both index-axis halves pass to git.
fn jigc_config_layer_pathspecs() -> Vec<&'static str> {
    CONFIG_LAYER_SPECS.iter().map(|row| row.spec).collect()
}

/// Capture the **worktree pre-image** of every [`ConfigLayerWrite::Rewritten`] row — the axis
/// M45's four index axes left open (M51 Increment 4 / T3; `design/finalize.md` → Rollback
/// discipline; M51's `settle-record.md` → §6 / D4), carried since M52 Increment 5 / T2 by the
/// generic [`crate::rollback::PreImageFamily`] every `FileCas` population shares.
///
/// The four index axes restore the *index*; nothing restored the **bytes on disk**. So a
/// refused `finalize` left the file the amend appended to and the stamp the refresh rewrote
/// standing in the worktree while the frame said *"nothing was committed"* — and before the
/// amend landed one task earlier, the ignore file's rewrite matched `HEAD`, so `git status`
/// read **clean** and a user's uncommitted private line existed in no git object at all.
///
/// Called at **Phase 4a**, beside the four index captures and before `gitignore::ensure` and
/// the stamp refresh — promote and retire run between, and neither touches these two files,
/// so the read is exactly as genuine there and it is genuine at every failure point below it
/// (`design/finalize.md` → *Every capture precedes every failure point*).
///
/// Each entry is keyed by the row's **pathspec**, which is both the family's key (what
/// [`crate::rollback::PreImageFamily::wrote`] matches when a write site reports what it
/// wrote) and the name its pre-image is parked under on a conflict. Its absolute path is
/// resolved through the row's [`ConfigLayerHome`], because the two writers hang off different
/// roots and a pre-image taken from the wrong one would restore a file nobody wrote.
fn capture_config_layer_worktree(
    repo_root: &Path,
    jigc_root: &Path,
) -> Result<crate::rollback::PreImageFamily> {
    let mut family = crate::rollback::PreImageFamily::empty(crate::rollback::FINALIZE_DOOR);
    for row in CONFIG_LAYER_SPECS {
        let ConfigLayerWrite::Rewritten { home } = row.write else {
            continue;
        };
        let path = match home {
            // `gitignore::ensure` is handed `jigc_root` and joins the file name onto it.
            ConfigLayerHome::JigcRoot => {
                jigc_root.join(row.spec.rsplit('/').next().unwrap_or(row.spec))
            }
            ConfigLayerHome::RepoRoot => repo_root.join(row.spec),
        };
        family.push(
            crate::rollback::PreImage::capture(row.spec, path).with_context(|| {
                format!(
                    "could not read the pre-finalize `{}` — the transaction will not rewrite \
                     a file it cannot put back",
                    row.spec,
                )
            })?,
        );
    }
    Ok(family)
}

/// Print each rollback conflict **beside** the door's own frame, and fold its identity into
/// the door's [`Outcome`] so the invocation log names it.
///
/// **One renderer, every door that commits through the shared executor** — the
/// `crate::gitignore::emit_ack` shape this increment already ships, for the same reason: the
/// fact is one fact, and a second copy of it is a second place for it to drift. Always
/// **stderr**, whatever the format: it is presentation beside the result, so under
/// `--format json` the document on stdout still parses as exactly one JSON value
/// (`design/command-output-contract.md` → Stream discipline).
///
/// It **adds to** the door's surface and replaces nothing. A hook rejection keeps its verbatim
/// stderr and its frame; a stage failure keeps its own routed `finalize.stage-failed`. Only
/// the log record grows — by the codes of the paths the rollback could not put back.
pub(crate) fn carry_rollback_conflicts(
    format: Format,
    mut outcome: Outcome,
    conflicts: &[Finding],
) -> Outcome {
    // **The stream rule** (M52 Increment 1 / T1): under `--format json` the stream carrying
    // the door's document carries nothing else, so the conflicts are *in* the document —
    // [`fold_rollback_conflicts`] put them there before it was rendered — and printing them
    // here as well would be both a duplicate and, on the reject arm, the trailing bytes that
    // stop the stream parsing at all. The agent/human arm is unchanged.
    if format != Format::Json {
        for finding in conflicts {
            eprint!("{}", crate::render::finding_line(finding, false));
        }
    }
    outcome
        .finding_codes
        .extend(conflicts.iter().map(|finding| finding.code.clone()));
    outcome
}

/// The document half of the rule [`carry_rollback_conflicts`] states: under `--format json`
/// the rollback conflicts join the findings the door is about to render, so the one document
/// carries them; under agent/human they are left to print beside the result, which is where
/// that surface has always shown them and where its bytes are pinned.
///
/// Both halves are called at every site that has conflicts in hand — this one before the
/// render, that one after — so a door cannot fold without recording, or record without
/// folding.
pub(crate) fn fold_rollback_conflicts(
    format: Format,
    mut findings: Vec<Finding>,
    conflicts: &[Finding],
) -> Vec<Finding> {
    if format == Format::Json {
        findings.extend(conflicts.iter().cloned());
    }
    findings
}

/// One config-layer index entry's pre-finalize state — the fourth rollback axis's capture
/// (M45 milestone-audit fix, `design/finalize.md` → Rollback discipline). `path` is a real
/// file under [`jigc_config_layer_pathspecs`] (the directory `.jigc/config` expands to its
/// files); `(mode, blob-sha)` is what the index held before the stage overwrote/added it.
struct ConfigLayerIndexEntry {
    path: String,
    mode: String,
    sha: String,
}

/// Capture the pre-finalize index entries for the jigc config-layer pathspecs
/// ([`jigc_config_layer_pathspecs`]) via `git ls-files --stage` — the fourth rollback axis's
/// pre-image (M45 milestone-audit fix; the sibling the owner-artifact axis
/// ([`capture_owner_artifact_index`]) left un-swept). Every real file under the pathspecs records
/// its `(mode, sha)`. Called **before** the stage's `git add` refreshes `.jigc/version` /
/// (re)stages the config layer, so a stage/commit failure restores *exactly* what was there — not
/// jigc's refreshed overwrite, not a first-finalize's newly-added entry. The captured path set
/// also serves as the "present pre-finalize" witness the rollback diffs against to drop
/// newly-added entries. Absent files (e.g. an untracked `.jigc/.gitignore`, or a stamp not yet
/// committed) simply don't appear — the rollback force-removes any such entry the stage adds.
fn capture_config_layer_index(repo_root: &Path) -> Result<Vec<ConfigLayerIndexEntry>> {
    let specs = jigc_config_layer_pathspecs();
    let mut args: Vec<&str> = vec!["ls-files", "--stage", "--"];
    args.extend(specs.iter().copied());
    let out = git_capture(repo_root, &args)?;
    let mut captured = Vec::new();
    for line in out.lines() {
        // `<mode> <sha> <stage>\t<path>` — the path follows a TAB, the metadata is
        // whitespace-separated before it.
        let Some((meta, path)) = line.split_once('\t') else {
            continue;
        };
        let fields: Vec<&str> = meta.split_whitespace().collect();
        if fields.len() >= 2 {
            captured.push(ConfigLayerIndexEntry {
                path: path.to_owned(),
                mode: fields[0].to_owned(),
                sha: fields[1].to_owned(),
            });
        }
    }
    Ok(captured)
}

/// Restore the jigc config-layer index to its captured pre-finalize state on a stage/commit
/// failure — the fourth scoped rollback axis (M45 milestone-audit fix). Best-effort (the commit
/// did not land, so a restore failure is logged, never raised — mirroring
/// [`rollback_owner_artifact_index`] / [`rollback_promotions`]). Two moves, together returning the
/// index (restricted to the config-layer pathspecs) to exactly its pre-finalize state:
///
/// - **Restore** each captured entry to its pre-finalize `(mode, sha)` via `git update-index
///   --cacheinfo` — **without touching the worktree**, so a `.jigc/version` [`refresh_version_stamp`]
///   rewrote on disk survives while `git show :.jigc/version` == the pre-finalize blob (the same
///   index-only discipline the owner-artifact axis holds). A same-build refresh captured the same
///   sha, so this is a no-op; a differing build's refreshed stage is undone.
/// - **Drop** any entry now under the pathspecs that was **absent** pre-finalize (a first-finalize
///   `git add` of a not-yet-tracked `.jigc/.gitignore` / stamp / config file) via `git update-index
///   --force-remove`, returning the index to its pre-finalize "not staged" state (the worktree file
///   stays). A user who *pre-staged* `.jigc/version` themselves captured a present entry, so it is
///   restored, never dropped — the "restore only what jigc's stage changed, never clobber a user's
///   pre-staged blob" discipline the third axis established.
fn rollback_config_layer_index(repo_root: &Path, captured: Option<&[ConfigLayerIndexEntry]>) {
    // `None` — the capture never ran, so nothing under the pathspecs is this finalize's doing
    // and the set difference below has no subject (M51 Inc 1 validation). Distinct from
    // `Some(&[])`, which says the layer was genuinely absent from the index pre-finalize, so
    // anything under the pathspecs now IS the stage's and is dropped.
    let Some(captured) = captured else {
        return;
    };
    for item in captured {
        let _ = git_run(
            repo_root,
            &[
                "update-index",
                "--cacheinfo",
                &item.mode,
                &item.sha,
                &item.path,
            ],
        );
    }
    let present_before: std::collections::HashSet<&str> =
        captured.iter().map(|item| item.path.as_str()).collect();
    let specs = jigc_config_layer_pathspecs();
    let mut args: Vec<&str> = vec!["ls-files", "--stage", "--"];
    args.extend(specs.iter().copied());
    if let Ok(out) = git_capture(repo_root, &args) {
        for line in out.lines() {
            if let Some((_, path)) = line.split_once('\t')
                && !present_before.contains(path)
            {
                let _ = git_run(repo_root, &["update-index", "--force-remove", path]);
            }
        }
    }
}

/// Refresh the committed binary-provenance stamp (`.jigc/version`) to the running build —
/// a store-writing op keeps the stamp current (`design/storage.md` → Store provenance:
/// setup writes it, store-writing ops refresh it). A same-build refresh writes identical
/// bytes (so `git add` stages nothing and the commit is unchanged); a newer build lands the
/// bumped stamp alongside the work, exactly as the git-tracked config layer is (re)staged.
///
/// **It reports what it wrote** into the transaction's worktree pre-image family (M51 Inc 4 /
/// T3), because this is one of the two writes a failed transaction has to put back — and the
/// write site is the only place the bytes jigc produced can be read back as jigc's own. The
/// two fan-out arms never reach this function, which is exactly why the family distinguishes
/// *"jigc wrote nothing here"* from *"jigc wrote identical bytes"*.
fn refresh_version_stamp(
    repo_root: &Path,
    config_worktree: &mut crate::rollback::PreImageFamily,
) -> Result<()> {
    crate::setup::write_version_stamp(repo_root)
        .with_context(|| "refreshing the binary-provenance stamp `.jigc/version`")?;
    config_worktree.wrote(crate::setup::VERSION_STAMP_PATH);
    Ok(())
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
fn stage_index_honoring(
    repo_root: &Path,
    plan: &engine::finalize::FinalizePlan,
    config_worktree: &mut crate::rollback::PreImageFamily,
) -> Result<()> {
    let mut pathspecs: Vec<String> = Vec::new();
    for promotion in &plan.promotions {
        pathspecs.push(promotion.destination.clone());
    }
    refresh_version_stamp(repo_root, config_worktree)?;
    pathspecs.extend(existing_pathspecs(
        repo_root,
        &jigc_config_layer_pathspecs(),
    ));
    // M45 Inc 8 T2 — stage each recorded owner-artifact so the artifact lands in the SAME
    // commit as the completion-record naming it (`design/finalize.md` → 5. Stage: finalize
    // stages the recorded paths, and the gate moves after the stage). Existence + non-ignore
    // guarded: an absent or gitignored path stays unstaged (an explicit `git add` of an
    // ignored path is fatal — exit 128 — and would abort the whole stage), so the post-stage
    // gate then blocks on it (absent / present-but-untracked). The pre-finalize index entry of
    // each path is captured by the executor before this add, so a stage/commit failure
    // restores it (the third rollback axis).
    pathspecs.extend(owner_artifact_stage_specs(repo_root, plan));
    // Nothing jigc-owned to add — the agent's existing index stands alone (a `git add --`
    // with no pathspec is an error, so guard it).
    if pathspecs.is_empty() {
        return Ok(());
    }
    let mut args: Vec<&str> = vec!["add", "--"];
    args.extend(pathspecs.iter().map(String::as_str));
    git_run(repo_root, &args).map_err(mark_stage_failure)
}

/// The doc-only stage (M55; `design/findings-channel.md` §3): `git add` exactly the task's
/// promotion destinations and its stageable recorded owner-artifacts
/// ([`owner_artifact_stage_specs`]), and return that path set — sorted and de-duplicated, so
/// the pathspec the commit takes is a function of the plan alone. Unlike
/// [`stage_index_honoring`] it refreshes no `.jigc/version` stamp and stages no config
/// layer: the commit is path-scoped, and those paths are not this task's docs. An empty set
/// stages nothing and is returned empty for the commit helper to refuse.
fn stage_doc_only(repo_root: &Path, plan: &engine::finalize::FinalizePlan) -> Result<Vec<String>> {
    let mut paths: Vec<String> = plan
        .promotions
        .iter()
        .map(|promotion| promotion.destination.clone())
        .collect();
    paths.extend(owner_artifact_stage_specs(repo_root, plan));
    paths.sort();
    paths.dedup();
    if paths.is_empty() {
        return Ok(paths);
    }
    let mut args: Vec<&str> = vec!["add", "--"];
    args.extend(paths.iter().map(String::as_str));
    git_run(repo_root, &args).map_err(mark_stage_failure)?;
    Ok(paths)
}

/// Typed marker for a git failure during jigc's **own stage phase** — the `git add` in
/// [`stage_migration`] / [`stage_index_honoring`] (M40 F7, `design/finalize.md` → M40
/// refinement item 2). Carried through [`try_execute_finalize_plan`]'s error channel so
/// the per-task surface can tell a stage-phase failure (routed blocking finding, the
/// git stderr embedded verbatim — [`stage_failed_finding`]) from a commit-phase hook
/// rejection (verbatim-raw, the recorded hook decision — item 3). The executor's shared
/// `Err` arm runs the rollback either way, before the surface discriminates.
#[derive(Debug)]
struct StageGitFailure(String);

impl std::fmt::Display for StageGitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for StageGitFailure {}

/// Wrap a stage-phase [`git_run`] failure in the [`StageGitFailure`] marker, flattening
/// the chain (`{:#}`) so the message keeps `git_run`'s embedded verbatim stderr.
fn mark_stage_failure(err: anyhow::Error) -> anyhow::Error {
    anyhow::Error::new(StageGitFailure(format!("{err:#}")))
}

/// Typed marker for a **commit-phase** rejection — the user's `pre-commit` / `commit-msg`
/// hook (or git itself) refused the commit in [`git_commit_capture`] (M42,
/// `design/finalize.md` → 6. Commit). It carries git's message **verbatim**, so the door
/// that raised it — which knows what survived and what its own re-run is — can frame it with
/// the recoverability it declares without editing the hook's own bytes
/// ([`surface_commit_rejection`]). Its [`Display`](std::fmt::Display) is that verbatim
/// message, so any caller that only prints `{err:#}` is byte-unchanged.
///
/// `pub(crate)` since M47 Inc 3 T7: the frame swept off the one task door onto the whole
/// nine-door committing axis, so `milestone` / `rename` / `migrate_corpus` discriminate on it
/// too (through [`surface_commit_rejection`], the one place that downcasts).
///
/// **It carries who refused** (M52 Increment 3 / T5): `by_hook` is false when git itself
/// refused the commit, which is the same *state* — everything before the commit succeeded,
/// nothing landed — but not the same *diagnosis*. See [`git_commit_capture`] for the exit-code
/// discriminator that sets it and for the driven basis of that discriminator.
#[derive(Debug)]
pub(crate) struct CommitRejected {
    /// git's own bytes, verbatim, under the seam's one-line frame.
    cause: String,
    /// Whether a hook could have caused this — see [`git_commit_capture`].
    by_hook: bool,
}

impl std::fmt::Display for CommitRejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.cause)
    }
}

impl std::error::Error for CommitRejected {}

/// Typed marker for a **non-hook** failure inside a door's commit transaction — the class
/// N20 names (`completions/artifacts/M51/charter.md` → N20; `gap-findings.md` → G-47).
///
/// `git merge --ff-only` refusing to overwrite ordinary untracked main-checkout WIP, a stale
/// `.git/index.lock` meeting the stage's `git add`, a promote destination that is a regular
/// file: every one of them fails **inside** the transaction a [`RejectionFrame`] was built for,
/// after its rollback has run — and every one of them used to reach
/// [`surface_commit_rejection`], miss the [`CommitRejected`] downcast, and fall through to the
/// plain operational envelope, discarding the frame's code, state clause and re-run
/// (`error_code: null` in the invocation log, no route, no statement of what survived).
///
/// It is deliberately a **separate** marker from [`CommitRejected`] rather than a widening of
/// it, and since M52 Increment 3 / T5 — when `CommitRejected` learned to carry *who* refused —
/// what separates them is no longer the diagnosis but the **state**. Both take
/// [`crate::render::commit_failed`]'s hook-free wording; they differ in *where* the failure
/// happened, and therefore in which clause is true of the repository:
///
///   * a `CommitRejected` (hook or git) fails **at** the commit, with everything before it
///     done — [`RejectionFrame::survived`];
///   * a `CommitFailed` fails **before** it, so the door's own stage may be exactly what did
///     not complete — [`RejectionFrame::survived_non_hook`].
///
/// Its [`Display`](std::fmt::Display) is the flattened cause, so git's own bytes stay verbatim.
#[derive(Debug)]
pub(crate) struct CommitFailed(String);

impl std::fmt::Display for CommitFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CommitFailed {}

/// Whether `err` is a **typed** error a door's surface already discriminates *before* it
/// reaches [`surface_commit_rejection`] — the exhaustive passthrough rule for
/// [`mark_commit_failure`].
///
/// Each of these is something better than a framed non-hook failure, and each is read back by
/// a `downcast_ref` further up, so wrapping one would both replace a precise diagnosis with a
/// generic one *and* make that `downcast_ref` miss:
///
///   * [`CommitRejected`] — a hook (or git) refused the commit; that is the *first* cell of the
///     axis, and it keeps its own frame and its own verbatim relay;
///   * [`StageGitFailure`] — jigc's own stage phase failed, which the per-task door routes as a
///     first-class blocking `finalize.stage-failed` finding with its own state clause and route
///     (M40 F7 item 2);
///   * [`OwnerArtifactBlock`] — the post-stage owner-artifact gate blocked: a real
///     `ValidationReport` at exit 3, not an operational failure at all;
///   * a [`BlockedFinding`](crate::render::BlockedFinding) — a routed refusal carrying its own
///     code and route, including the `repo.*` posture refusals the commit seam raises
///     (M51 Increment 2 / T4).
///
/// **A typed marker added to the executor's error channel owes a row here**, and
/// [`typed_errors_pass_through_the_commit_failure_marker`] is where that is asserted rather
/// than assumed — a list this size is only safe while something reddens when it goes stale.
fn already_typed(err: &anyhow::Error) -> bool {
    err.is::<CommitRejected>()
        || err.is::<StageGitFailure>()
        || err.is::<OwnerArtifactBlock>()
        || crate::render::blocked_finding(err).is_some()
}

/// Mark an error escaping a door's **commit transaction** as a [`CommitFailed`], so the door's
/// surface frames it instead of dropping the frame it already built (N20).
///
/// Applied at each door's transaction boundary, **after** its rollback has run — which is what
/// makes the frame's state clause true of what is on disk when it prints. An error
/// [`already_typed`] recognizes passes through unchanged.
pub(crate) fn mark_commit_failure(err: anyhow::Error) -> anyhow::Error {
    if already_typed(&err) {
        return err;
    }
    anyhow::Error::new(CommitFailed(format!("{err:#}")))
}

/// One committing door's half of the **survivable frame** (M47 Inc 3 T7; `DECISIONS.md` →
/// 2026-07-26 M47 the Settle, Decision 6). The door owns all three fields because only the
/// door knows them: `git_commit_capture` sees no verb, no id, and no argv.
pub(crate) struct RejectionFrame {
    /// This door's route-exempt error identity — its
    /// [`COMMITTING_DOORS`](invocation_log::COMMITTING_DOORS) member's `error_code`, so the
    /// invocation log names *which* door refused instead of every door borrowing the task
    /// door's `finalize.commit-rejected`.
    pub(crate) code: &'static str,
    /// What the refusal is **about**, in one of the contract's declared target forms — the
    /// work unit (`task:<id>` / `milestone:<id>`) where the door acts on one, the doc identity
    /// where it acts on one doc, and the door's own verb where its subject is the whole corpus
    /// (`design/command-output-contract.md` → the `*.commit-rejected` row).
    ///
    /// It exists because the machine arm keys on it (M52 Increment 1 / T1): the same refusal
    /// that names itself in the invocation log now projects `(code, target)` on the reject
    /// document, and a code with no target is not a key. The door owns it for the reason it
    /// owns the other three — `git_commit_capture` sees no verb, no id and no argv.
    pub(crate) target: String,
    /// What survived the rejection, as one clause with **no trailing period** (the renderer
    /// adds it). Must be true of *this* door: some doors leave their write staged, some
    /// unwind it entirely.
    ///
    /// **It states the rollback's INTENT, and the renderer scopes it to the rollback's
    /// outcome** (M52 completion audit). This is a constant composed before the transaction
    /// runs, so it cannot know that the compare-and-swap declined to overwrite a racer
    /// (`<door>.rollback-conflict`) or that the minted-area unwind declined to remove an area
    /// holding a third party's file (`<door>.foreign-bytes`) — the two outcomes M52
    /// Increment 5 minted, and the two in which this sentence is false. Those arrive at
    /// [`surface_commit_rejection`] as `conflicts`, one finding per path left standing, and
    /// [`crate::render::commit_rejection_route`] opens the clause with that exception rather
    /// than letting this absolute stand alone. So a door author writes the clause that is
    /// true when the rollback completed, and nothing here needs a second string for when it
    /// did not.
    pub(crate) survived: String,
    /// The state clause for the **non-hook** cell ([`CommitFailed`]), where `survived` is not
    /// true of it — `None` when one clause is true of both (the usual case: every rollback the
    /// hook cell describes runs on this path too).
    ///
    /// It exists because G-47's axis is `COMMITTING_DOORS × {hook rejection, non-hook failure}
    /// × {is the clause true?}`, and the answer is not uniform: `migrate-corpus`' hook-cell
    /// clause says the migrated bytes are *"written and staged"*, and in the non-hook cell the
    /// **stage is what failed**. Reusing one clause across both cells there would ship a law-1
    /// lie at the moment an operator is recovering.
    pub(crate) survived_non_hook: Option<String>,
    /// This door's **own** copy-runnable re-run command line, including any flag the re-run
    /// genuinely needs to reach the same commit phase again.
    pub(crate) rerun: String,
}

impl RejectionFrame {
    /// The state clause for the non-hook cell — the door's own where it stated one, else the
    /// clause it states for a hook rejection.
    fn non_hook_clause(&self) -> &str {
        self.survived_non_hook.as_deref().unwrap_or(&self.survived)
    }
}

/// Surface a failed committing door: a **commit-phase rejection** ([`CommitRejected`]) is
/// framed with `frame`'s state-truth clause + the door's own re-run and carries `frame.code`
/// into the invocation log; every other (unstructured `anyhow`) failure keeps the plain
/// operational-error envelope and carries no identity.
///
/// **Three render arms, not two** (M52 Increment 3 / T5): a hook's rejection, git's own
/// refusal of the same commit, and a failure *before* the commit ([`CommitFailed`]). Only the
/// first may name a hook; the first two share the state clause, because they share the state.
///
/// The one place that downcasts [`CommitRejected`], shared by every member of
/// [`COMMITTING_DOORS`](invocation_log::COMMITTING_DOORS) — the per-task `finalize` arm, both
/// milestone-finalize commit-model arms, `rename`, `migrate-corpus`, and the record-only doors
/// (the four milestone ops, joined at M49 by a sub-task `jigc task discard`). Before M47 only the task door framed anything and
/// the two milestone-finalize arms logged the *task* door's code; the requirement is about
/// *a commit that did not land*, and the log's job is to say which door it was
/// (`design/finalize.md` → "A failed finalize must be legible in the invocation log", widened
/// to the family). The rendering discipline is untouched: git's hook stderr stays verbatim.
pub(crate) fn surface_commit_rejection(
    format: Format,
    err: &anyhow::Error,
    frame: &RejectionFrame,
    conflicts: &[Finding],
) -> Outcome {
    // N20 — the **non-hook** cell: the frame was built in full and then dropped here, so a
    // `git merge --ff-only` refusal (or a stale index lock, or a blocked promote destination)
    // exited 1 with `error_code: null`, no route and no word about what survived. It keeps the
    // same three things the rejection cell states, in its own render arm — nothing in it blames
    // a hook, because in this cell none spoke.
    let framed = match err.downcast_ref::<CommitRejected>() {
        // A hook refused it: the hook's complaint IS the correction signal, so the frame
        // routes the operator at it.
        Some(rejected) if rejected.by_hook => Some((
            rejected.cause.as_str(),
            render::commit_rejected(
                &rejected.cause,
                &frame.survived,
                &frame.rerun,
                conflicts.len(),
            ),
            render::commit_rejection_route(&frame.survived, &frame.rerun, conflicts.len()),
        )),
        // **git itself** refused it (M52 Increment 3 / T5) — the third cell of the axis. It
        // takes the non-hook *diagnosis* and the **hook cell's state clause**, and that pairing
        // is the whole reason it is an arm rather than a second caller of the one below:
        // `survived_non_hook` was written for a failure **before** the commit (a stale index
        // lock met the stage, a blocked promote destination, a refused `--ff-only`), so
        // `migrate-corpus` says there *"the stage did not complete"*. Here everything up to the
        // commit succeeded, exactly as under a rejecting hook, so that sentence would be false
        // at the moment an operator is recovering.
        Some(rejected) => Some((
            rejected.cause.as_str(),
            render::commit_failed(
                &rejected.cause,
                &frame.survived,
                &frame.rerun,
                conflicts.len(),
            ),
            render::commit_failure_route(&frame.survived, &frame.rerun, conflicts.len()),
        )),
        None => err.downcast_ref::<CommitFailed>().map(|failed| {
            (
                failed.0.as_str(),
                render::commit_failed(
                    &failed.0,
                    frame.non_hook_clause(),
                    &frame.rerun,
                    conflicts.len(),
                ),
                render::commit_failure_route(
                    frame.non_hook_clause(),
                    &frame.rerun,
                    conflicts.len(),
                ),
            )
        }),
    };
    let Some((cause, text, route)) = framed else {
        // Neither typed cell: the shared operational funnel, with the conflicts beside it
        // exactly as the two gate arms place them.
        return carry_rollback_conflicts(
            format,
            crate::invocation_log::operational_failure(format, err),
            conflicts,
        );
    };

    // **The reject document owns stderr** (`design/command-output-contract.md` → Stream
    // discipline, the reject row): under `--format json` the refusal and every rollback
    // conflict are one findings envelope, because a driver reading the stream this door
    // writes to must find exactly one document there. The agent-text arm is byte-unchanged —
    // the frame, then each conflict beside it.
    if format == Format::Json {
        let mut findings = vec![render::commit_rejection_finding(
            frame.code,
            &frame.target,
            cause,
            route,
        )];
        findings.extend(conflicts.iter().cloned());
        // A no-delta cascade: this is a refusal, not an inventory sweep, so the severity
        // post-pass is a no-op over it. A cascade that cannot be resolved at all degrades to
        // the framed text below rather than failing twice — the same fall-through
        // `invocation_log::operational_failure` takes on its own envelope arm.
        if let Ok(resolved) = crate::cascade_util::no_delta_resolved() {
            let report = engine::result::ValidationReport::new(findings, &resolved);
            eprint!("{}", render::validation(format, &report));
            return carry_rollback_conflicts(format, Outcome::error(frame.code), conflicts);
        }
    }
    eprintln!("{text}");
    carry_rollback_conflicts(format, Outcome::error(frame.code), conflicts)
}

/// The one home of the emitted-command quoting rule is [`engine::finding::shell_token`];
/// this crate reaches it by its historical name.
///
/// It moved to the engine at the M51 completion audit: the engine mints routes of its own
/// (`file_state`'s conflict block, `validate`'s owner-artifact `git add`, `finalize`'s
/// `git restore --staged`), so a CLI-only home meant every engine-side producer
/// interpolated its path raw — a `Route::human` naming `` `git add -- my notes.md` ``
/// exits 128 when followed, and a `Route::mechanical` carrying the same token panicked the
/// debug binary at the fence instead of being quoted at the producer.
pub(crate) use engine::finding::shell_token;

/// Emit the **pre-commit** `left-out` advisory (M42, `design/finalize.md` → "The `left-out`
/// advisory prints BEFORE the commit too") — nothing at all when the commit leaves nothing
/// behind. Placement follows the [`relay_hook_output`] discipline: agent/human text goes to
/// **stdout** (where the agent reads the finalize surface), but under `--format json` the
/// structured envelope owns stdout and must not be corrupted, so the advisory goes to
/// **stderr**.
/// `model` is this finalize's commit model ([`render::CommitModel`]) — the advisory's stem and
/// its guidance clause are both the model's, because on the amend arm *"about to commit the
/// index"* is false and *"git add to include"* is a route this same run refuses (the F-10
/// review's MEDIUM-3).
fn emit_left_out_advisory(
    format: Format,
    deferred: &mut String,
    left_out: &[render::ManifestEntry],
    model: render::CommitModel,
) {
    emit_or_defer(format, deferred, render::left_out_advisory(left_out, model));
}

/// Emit the **pre-commit** carried-over print (M43, `design/surface-contract.md` → The
/// carryover gate) — nothing at all when nothing is carried. Stream discipline as
/// [`emit_left_out_advisory`]: agent/human text to **stdout**, `--format json` to
/// **stderr** (the structured envelope owns stdout).
fn emit_carried_advisory(format: Format, deferred: &mut String, carried_paths: &BTreeSet<String>) {
    let carried: Vec<render::ManifestEntry> = carried_paths
        .iter()
        .map(|path| render::ManifestEntry {
            path: path.clone(),
            kind: render::ManifestKind::CarriedOver,
        })
        .collect();
    emit_or_defer(format, deferred, render::carried_over_advisory(&carried));
}

/// **The stream rule for a pre-commit print** (M52 Increment 1 / T1;
/// `design/command-output-contract.md` → Stream discipline).
///
/// Both advisories above are emitted **before** the commit, when nobody yet knows which arm
/// this finalize will take — and that is exactly what made them a defect on the machine
/// surface. Under `--format json` a landed finalize's document owns stdout and these print
/// beside it on stderr (unchanged, and the set they name also rides `committed.left_out` and
/// the manifest's `carried-over` labels); a **rejected** one's document owns stderr, and prose
/// printed there ahead of it is why `json.loads(stderr)` failed at char 0.
///
/// So under json the text is **held** and flushed only on the arm whose document is not
/// stderr's. On the reject arm it is withheld, and that is a fold rather than a loss: both
/// advisories forecast what a commit *would* leave behind, and on that arm no commit was made
/// — the frame's own clause says so. The agent/human arm is untouched: it prints to stdout,
/// in place, exactly as before.
fn emit_or_defer(format: Format, deferred: &mut String, advisory: String) {
    if advisory.is_empty() {
        return;
    }
    if format == Format::Json {
        deferred.push_str(&advisory);
    } else {
        print!("{advisory}");
    }
}

/// Relabel manifest entries whose path is in the pre-commit **carried set** to
/// [`render::ManifestKind::CarriedOver`] (M43 — the labeled manifest, all four render
/// sites over one set). Labeling changes no set membership: a carried path keeps its
/// place in the included set; the label says only how it got there (staged before this
/// task existed, riding under a declared `--carry-staged`).
fn relabel_carried(entries: &mut [render::ManifestEntry], carried: &BTreeSet<String>) {
    for entry in entries {
        if carried.contains(&entry.path) {
            entry.kind = render::ManifestKind::CarriedOver;
        }
    }
}

/// The M30 G2 block-on-empty guidance: the working tree is dirty but the narrowed index
/// is empty, so a per-task `IndexHonoring` commit would land nothing the agent staged. A
/// routed blocking finding pointing at `git add` — distinct from the genuinely-clean
/// engine `finalize.empty-commit` ("produced no diff"), selected CLI-side
/// (`design/finalize.md` → Dirty-tree policy, revised M30).
///
/// [Keys at the task](work_unit_location) — one of the two `finalize.*` members living in the
/// CLI, which the design's file-scoped census missed (M42 inc-9 T4).
fn nothing_staged_finding(task_id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.nothing-staged",
        "you staged nothing — the working tree has changes but the index is empty",
        Some(work_unit_location(task_id)),
        Some("`git add` your changes, then re-run `jigc task finalize`".into()),
    )
}

/// The **work-unit ref** [`Location`] a `finalize.*` block whose subject is the task keys at
/// — `task:<id>`, the container address the engine's `finalize.rs` siblings carry
/// ([command-output-contract.md](../../../design/command-output-contract.md) → the form
/// table, the work-unit row). Without it both CLI members projected the degenerate key
/// `(code, null)`: every staged-nothing block in every repo was one key.
fn work_unit_location(task_id: &str) -> Location {
    Location::addressed(work_unit_ref(task_id), 1, 1)
}

/// **The root a workbench path is spelled against** — `jigc_home`, never `repo_root`
/// (M53 — the pre-v1 usability batch, row 6 / the rc.19 per-axis review's `(3, F-A)`; the
/// rule `crate::milestone`'s module header states, and `a6711cee` applied there).
///
/// Everything under `.jigc/` — a working area, a displaced entry, the transient
/// `COMMIT_MSG` — belongs to the **main checkout**, which is what `jigc_root`'s parent is by
/// construction. `repo_root` is a different thing with the same shape: *the checkout the
/// command was typed in*, which from a branch-attached linked worktree is another directory
/// entirely. `render::repo_relative` falls back to the honest absolute when its strip fails,
/// so handing it `repo_root` does not error — it silently prints the host filesystem, on the
/// stderr note, on the 1.0-pinned `committed.displaced` keys and inside
/// `finalize.foreign-bytes`' message and route.
///
/// Taking `jigc_root` (rather than `jigc_home`) is deliberate: it is the argument the
/// workbench functions already carry, so the correct root is derivable at every one of them
/// without a caller being trusted to pass it. The fallback is the degenerate root-directory
/// case and is unreachable for a real `<home>/.jigc`.
pub(crate) fn workbench_home(jigc_root: &Path) -> &Path {
    jigc_root.parent().unwrap_or(jigc_root)
}

/// The **work-unit ref** itself — `task:<id>`, the contract's declared target form — so the
/// two consumers that need it as a string (the located family above, and the committing
/// door's [`RejectionFrame::target`]) spell it once (M52 Increment 1 / T1).
pub(crate) fn work_unit_ref(task_id: &str) -> String {
    format!("task:{task_id}")
}

/// The M40 F7 routed stage-failure block (`design/finalize.md` → M40 refinement item
/// 2): a git failure during jigc's **own stage phase**, surfaced as a blocking finding
/// with the git stderr embedded verbatim — previously a raw enveloped operational error
/// with no code and no route. The rollback has already run when this surfaces (the
/// executor's shared `Err` arm). Code executor-chosen in the `finalize.*` family (the
/// design names no code; `DECISIONS.md` 2026-07-10 M40 Inc 1 T3). [Keys at the
/// task](work_unit_location) — the second CLI-resident member of the family.
///
/// **The route is a [`Route::mechanical`]** (M53 — the pre-v1 usability batch, row 5 / the
/// M52 per-axis review's `(4, DEFECT 1)`). M47 swept the survivable frame over the
/// committing axis with, in the roadmap's words, *a shell-safe copy-runnable re-run argv*
/// per door, and M51's route floor widened to blocking findings so that a blocked finalize
/// names its recovery. Driven, this one named `jigc task finalize` with **no `<ID>`**: the
/// task id was in hand — this function takes it and spends it on the locus — while the
/// route was a bare `&'static str`, so pasting it exited **2** with a clap usage error,
/// one step from the sibling frame below that prints `re-run
/// `jigc task finalize <id>``. `Route::mechanical` is what makes that structural: its
/// debug-build fence parses the argv against the real CLI, so a route that stops running
/// stops compiling the test suite, and `Route::human` cannot be reached by accident.
///
/// The argv leads and the condition trails, because that is the order the reader acts in:
/// the command is the thing to copy, and *once the git failure is resolved* is when to run
/// it. The **message** keeps git's own stderr verbatim, absolute path and all
/// (`design/surface-contract.md` — the *quoting an invocation* case); only the route moved.
///
/// The door is always `jigc task finalize`: this finding's one producer is
/// [`TaskArea::run_finalize`]'s `StageGitFailure` arm, and a sub-task — whose boundary is
/// the milestone door — is refused by that door long before a stage phase runs.
///
/// **And it echoes the run's own flags**, exactly as the [`RejectionFrame`] one match arm
/// away does and for the same reason: *the same re-run* means the same invocation. Driven,
/// the argv without them is copy-runnable and still does not land — on a migration hold
/// `jigc task finalize <id>` exits **4** at the review gate the original run had passed
/// with `--approve`. A route that parses but does not do what its own sentence says is the
/// half-fix, so the two flags a repeat run genuinely needs ride it
/// (`design/finalize.md`; `crates/cli/tests/migrate_rollback.rs` runs the emitted argv).
fn stage_failed_finding(
    task_id: &str,
    git_error: &str,
    approve: bool,
    carry_staged: bool,
) -> Finding {
    let mut argv = vec![
        "jigc".to_string(),
        "task".to_string(),
        "finalize".to_string(),
        task_id.to_string(),
    ];
    if approve {
        argv.push("--approve".to_string());
    }
    if carry_staged {
        argv.push("--carry-staged".to_string());
    }
    Finding::graded(
        Severity::Blocking,
        "finalize.stage-failed",
        format!(
            "jigc could not stage its own changes — no commit was made and the \
             promotions were rolled back: {git_error}"
        ),
        Some(work_unit_location(task_id)),
        Some(engine::finding::Route::mechanical(
            argv,
            " once the embedded git failure is resolved (e.g. remove a stale \
             `.git/index.lock`) — the task survives intact, so the same re-run lands the \
             commit",
        )),
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
/// 6. Commit; `design/auto-migration.md` → The transaction mechanism, the scoped
/// rollback obligation). Promoted paths are restored **two-axis scoped** (confidence-audit
/// sibling-hunt item 2): the **index** back to each destination's captured pre-finalize
/// entry via the shared third-axis primitive ([`rollback_owner_artifact_index`]) — never a
/// `git restore --staged` reset to HEAD, which destroyed a blob the user staged at the
/// destination mid-task (post-mint, so outside the carryover snapshot; for a destination
/// new at HEAD the reset dropped the entry entirely) — and the **worktree** back to what it
/// held before [`promote`] wrote it and what [`retire`] deleted.
///
/// **The worktree axis is `worktree`'s, and it is compare-and-swap** (M52 Increment 5 / T3;
/// `crate::rollback::ROLLBACK_POPULATIONS` → `promote-destination` / `retired-original`).
/// Both populations restore **only while the path is still as jigc left it** — the promoted
/// bytes at a destination, absence at a retirement — and a path a third party changed inside
/// the transaction keeps that third party's version, with jigc's pre-image parked in the
/// gitignored workbench and one blocking `finalize.rollback-conflict` per raced path. Before
/// that, both arms acted unconditionally on their own half: the promote rewrote its capture
/// over a racer's edit, and the retire's *"restore only while absent"* silently dropped its
/// capture and left jigc's deletion of the original bytes standing (baseline-rollback.md §1
/// table A rows 2 and 3). What each entry restores is unchanged: the captured pre-promote
/// bytes for a destination that pre-existed (the same-path untracked-foreign cell,
/// confidence-audit minor item 10, and for a tracked destination a capture strictly more
/// faithful than `--source=HEAD`, which destroyed an uncommitted pre-promote modification —
/// reviewer LOW-2), the promoted copy removed for a genuinely new doc, and the retire's own
/// byte-capture rewritten (which recovers an **untracked** foreign `git restore` cannot —
/// review F3).
///
/// Two arms stay **here**, both keyed on a destination this promotion **created** — asked of
/// the family rather than of a list beside it, so the two cannot disagree — and both reached
/// only once the swap has actually removed the copy, never over a racer's file:
///
/// - a destination **tracked at HEAD** with no pre-image (deleted from the worktree
///   pre-promote) is restored from git (`--source=HEAD --worktree` — index untouched, that
///   axis is restored above). It is not a pre-image population: it has no third-party byte to
///   lose, because the bytes it writes come out of the object DB;
/// - a genuinely new doc's now-empty parent directories are swept.
///
/// Retirements keep their **index axis, keyed on the staged-pathspec set** (`staged`, what
/// [`stage_migration`] actually `git add`ed): only a deletion jigc's own stage staged is
/// un-staged (`git restore --staged`). A planned-but-untouched retirement — the user's
/// pre-finalize `git rm` (in neither set) — is restored on neither axis: the prior
/// unconditional per-planned-retirement `git restore --staged --worktree` resurrected it on
/// any commit failure.
///
/// Best-effort on the restores: a failure is logged, never raised — the commit did not land,
/// so the worst case is a stray copy the next `finalize`/`discard` overwrites. The returned
/// findings are the raced paths, which the door prints beside its own frame.
fn rollback_promotions(
    repo_root: &Path,
    jigc_root: &Path,
    promotions: &[Promotion],
    promo_index: &[OwnerArtifactIndexEntry],
    worktree: &crate::rollback::PreImageFamily,
    retirements: &[PathBuf],
    staged: &[String],
) -> Vec<Finding> {
    // Index axis: restore each destination's captured pre-finalize index entry — a user's
    // mid-task staged blob comes back byte-exact, and a pre-staged deletion (entry absent
    // pre-finalize) is NOT resurrected (`--force-remove` drops the stage's overwrite).
    rollback_owner_artifact_index(repo_root, promo_index);
    for retirement in retirements {
        // Index axis: un-stage the deletion ONLY when jigc's own stage staged it — a
        // user's pre-staged `git rm` is not in `staged` and stays staged.
        let path = retirement.to_string_lossy();
        if staged.iter().any(|spec| *spec == path) {
            let _ = git_run(repo_root, &["restore", "--staged", &path]);
        }
    }
    // The worktree axis, both populations, compare-and-swap.
    let conflicts = worktree.restore(repo_root, jigc_root);
    for promotion in promotions {
        // Only a destination the promotion CREATED reaches the two arms below — a
        // pre-existing one was restored by the swap above, or left to its racer.
        if !worktree
            .entry(&promotion.destination)
            .is_some_and(crate::rollback::PreImage::created)
        {
            continue;
        }
        let dest = repo_root.join(&promotion.destination);
        // Still there: the swap declined to remove it because the bytes are somebody else's,
        // and a conflict already names both copies. Neither arm below may touch it.
        if dest.exists() {
            continue;
        }
        if path_at_head(repo_root, &promotion.destination) {
            let _ = git_run(
                repo_root,
                &[
                    "restore",
                    "--source=HEAD",
                    "--worktree",
                    "--",
                    &promotion.destination,
                ],
            );
            continue;
        }
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
    conflicts
}

/// **Which registry row a destroying door's subject is read through**, and the noun the
/// narration uses for that kind of area — never guessed from the path, because the two
/// callers that would guess are the two doors that must not disagree.
///
/// [`Displaced`](AreaKind::Displaced) has **no registry row**: `.jigc/displaced/` is where
/// jigc parks bytes it moved aside rather than destroy ([`displace_foreign_area`];
/// `crate::relocate::relocate_stranded`), so its whole tree is the subject. The row
/// deliberately **does not discriminate** a pre-image jigc parked from a foreign byte a
/// human dropped there — both carry the identical claim, that nothing else has a copy
/// (`completions/artifacts/M52/settle-record.md` → §8).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum AreaKind {
    /// `.jigc/tasks/<task-id>/` — `engine::state::TASK_AREA_FILES` plus the `docs/` rule.
    Task,
    /// `.jigc/milestones/<milestone-id>/` — `engine::state::MILESTONE_AREA_FILES`.
    Milestone,
    /// `.jigc/displaced/` — every leaf under it.
    Displaced,
}

impl AreaKind {
    /// How a narration names this kind of area.
    fn subject(self) -> &'static str {
        match self {
            AreaKind::Task | AreaKind::Milestone => "working area",
            AreaKind::Displaced => "relocation workbench",
        }
    }
}

/// One area a destroying door is about to take, paired with the bytes in it **jigc did not
/// write** — the subject the three consenting doors refuse over and narrate (M52 Increment 4
/// / T5; `design/team-ready-state.md` → The working area's two populations).
///
/// The refusal's listing and the narration's listing are the same lines from the same walk,
/// so a door cannot refuse over one set and take another — the *"a sink can be narrower than
/// the guard that cleared it"* shape G-15 names.
pub(crate) struct ForeignArea {
    /// The area itself — what the narration says it is removing.
    dir: PathBuf,
    /// The noun for this kind of area ([`AreaKind::subject`]).
    subject: &'static str,
    /// The complement's entries: the absolute path (re-read after the removal, so the
    /// narration is keyed on the outcome) and the repo-relative line every surface prints.
    entries: Vec<(PathBuf, String)>,
}

/// **The complement of every area in `areas`**, fail-closed — the one derivation the guards,
/// the refusals and the narrations at `jigc task discard`, `jigc milestone discard` and
/// `jigc uninstall` all read.
///
/// **Shape decides membership of the subject, not just of the set.** An area that is absent,
/// a plain file, or a **symlink** is not a working area: the sinks' `remove_dir_all` destroys
/// nothing at such a path (a plain file at `.jigc/tasks` is `crate::setup`'s
/// `workbench_paths` subject instead, and a symlinked area would have this walk enumerating
/// *somebody else's* directory and refusing over bytes the door will never touch). A door
/// must neither claim nor refuse over what it will not take.
///
/// **Fail-closed**, like every probe a destroying door reads: a present area that cannot be
/// read is an `Err`, never an empty complement — "enumerated nothing" and "there is nothing"
/// are the same empty vector to a caller about to delete, and only one of them is safe.
///
/// Areas whose complement is empty are dropped, so the ordinary corpus carries no rows at all.
pub(crate) fn foreign_areas(
    jigc_home: &Path,
    areas: &[(PathBuf, AreaKind)],
) -> std::io::Result<Vec<ForeignArea>> {
    let mut found = Vec::new();
    for (dir, kind) in areas {
        match std::fs::symlink_metadata(dir) {
            Ok(shape) if shape.is_dir() => {}
            Ok(_) => continue,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(err) => return Err(err),
        }
        let relative = match kind {
            AreaKind::Task => state::foreign_area_paths(dir, state::WorkArea::Task)?,
            AreaKind::Milestone => state::foreign_area_paths(dir, state::WorkArea::Milestone)?,
            AreaKind::Displaced => displaced_leaves(dir)?,
        };
        if relative.is_empty() {
            continue;
        }
        found.push(ForeignArea {
            dir: dir.clone(),
            subject: kind.subject(),
            entries: relative
                .into_iter()
                .map(|rel| {
                    let abs = dir.join(&rel);
                    let line = render::repo_relative(jigc_home, &abs);
                    (abs, line)
                })
                .collect(),
        });
    }
    Ok(found)
}

/// Every leaf under `.jigc/displaced/`, relative to it and sorted — the [`AreaKind::Displaced`]
/// row's walk.
///
/// Leaves rather than top-level entries, because the parking home is keyed by the *unit* whose
/// bytes were moved (`displaced/<task-id>/<relative>`), so a top-level listing would name task
/// ids where the operator needs file paths. Shape is read with `symlink_metadata`, so a symlink
/// is a leaf rather than a door out of the tree — the `crate::setup::workbench_paths` rule.
fn displaced_leaves(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut stack = vec![root.to_path_buf()];
    let mut leaves = Vec::new();
    while let Some(path) = stack.pop() {
        if path != root && !std::fs::symlink_metadata(&path)?.is_dir() {
            leaves.push(path.strip_prefix(root).unwrap_or(&path).to_path_buf());
            continue;
        }
        for entry in std::fs::read_dir(&path)? {
            stack.push(entry?.path());
        }
    }
    leaves.sort();
    Ok(leaves)
}

/// The repo-relative lines of every area's complement, in area order — the listing a refusal
/// prints and the set it is refusing over.
pub(crate) fn foreign_lines(areas: &[ForeignArea]) -> Vec<String> {
    areas
        .iter()
        .flat_map(|area| area.entries.iter().map(|(_, line)| line.clone()))
        .collect()
}

/// What a destroying door's removal would take that jigc did not write, read **before** the
/// removal so the narration afterwards can name what it actually took — the foreign-byte
/// counterpart of `crate::milestone::PendingLoss`, and bound by the same rule: the narration
/// is keyed on the **outcome**, never on the intent, so a removal that failed claims nothing.
///
/// Best-effort, like every narration: an area that cannot be read yields no warning rather
/// than failing a removal the guards (or `--force`) already cleared.
pub(crate) struct PendingForeign(Vec<ForeignArea>);

/// Read [`PendingForeign`] over `areas`. Call it immediately before the removal.
pub(crate) fn pending_foreign(jigc_home: &Path, areas: &[(PathBuf, AreaKind)]) -> PendingForeign {
    PendingForeign(foreign_areas(jigc_home, areas).unwrap_or_default())
}

impl PendingForeign {
    /// Name what the removal actually took, per area and per path. Call it immediately after
    /// the removal, on **both** its outcomes.
    pub(crate) fn narrate_taken(&self, jigc_home: &Path) {
        for area in &self.0 {
            // `symlink_metadata`, not `exists()`: a dangling symlink the removal left behind
            // reads as absent through `exists()`, and the door would report bytes still in
            // the way as taken.
            let taken: Vec<String> = area
                .entries
                .iter()
                .filter(|(abs, _)| std::fs::symlink_metadata(abs).is_err())
                .map(|(_, line)| line.clone())
                .collect();
            crate::milestone::narrate_removal(jigc_home, &area.dir, area.subject, &taken);
        }
    }
}

/// **The bytes jigc did not write into `area`, kept** — every entry of the working area's
/// complement (`engine::state::foreign_area_paths`, whose registry decides membership) moved
/// to `<jigc_root>/displaced/<unit_id>/<relative>`, with its relative path preserved, before
/// the caller removes the area (M52 Increment 4 / T3; `settle-record.md` → D3.3 as amended by
/// §8; `design/storage.md` → The per-task working area).
///
/// **Why a new primitive rather than `relocate::displace_foreign_squatter`.** That one
/// classifies *one* destination against the file-state record, frees its index slot with `git
/// rm --cached`, and parks it by **basename** into a flat directory. Every one of those is
/// wrong here: the subject is an enumerated set inside a **gitignored** tree no index knows,
/// and a flat park by basename would collapse `analysis/perf.txt` and `docs/perf.txt` onto one
/// name. Only the same-filesystem `fs::rename` technique carries over — `.jigc/displaced/` and
/// `.jigc/tasks/` are siblings, so the move is a rename and never a copy.
///
/// **The unit is the complement's entry, not a file.** A foreign directory arrives whole and
/// is moved whole, which preserves its subtree by construction and is also what the door
/// *names*: one pair per entry.
///
/// **A pre-existing destination is never overwritten.** The parking home is never cleaned by
/// jigc, so a later task minted from the same intent — the same id — can meet its own earlier
/// `NOTES.md` there; the move then lands at `<name>.2`, `<name>.3`, … and the returned `to`
/// says where it actually went. Silently replacing it would be this door's own loss cell, one
/// directory over.
///
/// **Best-effort, and honest about it — which takes both halves of the outcome** (M53
/// Increment 2 / T2; `completions/artifacts/M53/settle-record.md` → D2.1). A move that cannot
/// be made is never raised: the commit is already truth and the teardown after this is what
/// the caller depends on. Until M53 the return carried *only* what actually moved, and each
/// failure went out as a lone `note:` of its own — so [`narrate_displacement`]'s count came
/// from `moved.len()`, and an area that held two entries and moved one announced itself as
/// *held 1 entry* while the second was destroyed (driven,
/// `completions/artifacts/M53/baseline-a3-2-failed-displacement.md` §4.1). The return now
/// carries the entries it could **not** move beside the ones it did, each with the reason its
/// move failed — `create_dir_all` at the parking home and `fs::rename` are two distinct
/// failure points and both belong in that set — and the narration is the one place either set
/// is printed. The outcome-keyed rule `crate::milestone::PendingLoss` states is unchanged:
/// name what the act *took*, never what it intended.
///
/// **Declared bound:** the complement's entries reach here through `to_string_lossy`
/// (`engine::state::foreign_area_paths`), so an **undecodable** filename arrives
/// U+FFFD-substituted and its rename fails — the move is then narrated as a failure and the
/// teardown takes the byte. Visible, not kept; the repair belongs where the name is lost.
pub(crate) fn displace_foreign_area(
    jigc_root: &Path,
    area: &Path,
    kind: state::WorkArea,
    unit_id: &str,
) -> Displacement {
    // Every path this function prints lives under `.jigc/`, so it spells them against the
    // workbench root, which it derives from the one argument that *is* that root (M53 — the
    // pre-v1 usability batch, row 6). It used to take a `repo_root` beside it and render
    // against that: at the milestone door the caller passed `jigc_home` and the surfaces were
    // clean, at the task door it passed the standing checkout and, from a branch-attached
    // linked worktree, every one of them went host-absolute — the stderr note, the 1.0-pinned
    // `committed.displaced[].from`/`.to`, and each park-failure reason. Dropping the parameter
    // is the fix: there is now no argument a caller could get wrong.
    let jigc_home = workbench_home(jigc_root);
    let foreign = match state::foreign_area_paths(area, kind) {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!(
                "note: could not read {} to move aside what jigc did not write there \
                 (the removal below takes whatever is in it): {err}",
                render::repo_relative(jigc_home, area),
            );
            // Nothing was enumerated, so there is no entry to report on either side — the
            // note above is the whole answer this call has.
            return Displacement {
                moved: Vec::new(),
                unmoved: Vec::new(),
            };
        }
    };
    let home = jigc_root
        .join(crate::relocate::WORKBENCH_SUBDIR)
        .join(unit_id);
    let mut moved: Vec<render::Displaced> = Vec::new();
    let mut unmoved: Vec<Unmoved> = Vec::new();
    for entry in foreign {
        let from = area.join(&entry);
        let from_line = render::repo_relative(jigc_home, &from);
        let to = free_displacement_path(home.join(&entry));
        let parent = to.parent().unwrap_or(&home);
        if let Err(err) = std::fs::create_dir_all(parent) {
            unmoved.push(Unmoved {
                path: from_line,
                reason: format!(
                    "could not open {} to park it: {err}",
                    render::repo_relative(jigc_home, parent),
                ),
            });
            continue;
        }
        let to_line = render::repo_relative(jigc_home, &to);
        match std::fs::rename(&from, &to) {
            Ok(()) => moved.push(render::Displaced {
                from: from_line,
                to: to_line,
            }),
            Err(err) => unmoved.push(Unmoved {
                path: from_line,
                reason: format!("could not move it to {to_line}: {err}"),
            }),
        }
    }
    moved.sort_by(|a, b| a.from.cmp(&b.from));
    unmoved.sort_by(|a, b| a.path.cmp(&b.path));
    Displacement { moved, unmoved }
}

/// What one [`displace_foreign_area`] call did with a working area's complement — **both**
/// halves, because either alone lets a surface under-report the area (M53 Increment 2 / T2).
pub(crate) struct Displacement {
    /// The entries that reached `.jigc/displaced/<unit-id>/`, sorted by `from` — the pairs
    /// the landed envelope's `committed.displaced` carries, which is declared as what moved.
    pub(crate) moved: Vec<render::Displaced>,
    /// The entries whose move failed — still in the area when this call returns, and whose
    /// fate from there is the **caller's** teardown, not this function's. Sorted by path.
    /// Empty on the ordinary path, where the parking home is jigc's own sibling directory
    /// and every move is a rename between two directories inside `.jigc/`.
    pub(crate) unmoved: Vec<Unmoved>,
}

impl Displacement {
    /// How many entries the area's complement held — `moved` **and** `unmoved`, which is the
    /// only count a surface may print about the area: `moved.len()` is a count of the act.
    fn held(&self) -> usize {
        self.moved.len() + self.unmoved.len()
    }
}

/// One complement entry [`displace_foreign_area`] could not move, and why.
pub(crate) struct Unmoved {
    /// Where the entry was asked to move **from** — its working-area path, repo-relative
    /// (law 1: a surface prints no host filesystem).
    pub(crate) path: String,
    /// Why the move did not happen, naming the failure point: the parking home could not be
    /// opened, or the rename itself failed.
    pub(crate) reason: String,
}

/// `wanted`, or the first `<wanted>.<n>` (n ≥ 2) nothing occupies — the no-clobber rule
/// [`displace_foreign_area`] states, asked with `symlink_metadata` so a dangling symlink
/// counts as occupied (it is a name in the way, not an absence).
fn free_displacement_path(wanted: PathBuf) -> PathBuf {
    if std::fs::symlink_metadata(&wanted).is_err() {
        return wanted;
    }
    let mut n = 2u32;
    loop {
        let candidate = PathBuf::from(format!("{}.{n}", wanted.to_string_lossy()));
        if std::fs::symlink_metadata(&candidate).is_err() {
            return candidate;
        }
        n += 1;
    }
}

/// Name on **stderr** what [`displace_foreign_area`] did with one working area's complement —
/// the side channel, so the landed document still owns stdout undiluted on every format
/// (`design/command-output-contract.md` → Stream discipline). Silent when the complement was
/// empty: the omitting context prints no bytes at all.
///
/// **The count is the area's, never the act's** (M53 Increment 2 / T2;
/// `completions/artifacts/M53/settle-record.md` → D2.2). It is [`Displacement::held`] — every
/// entry the complement held — and both sets are named under it: the moves that landed, and
/// the entries that did not move with the reason each did not. Keyed on `moved.len()` this
/// sentence read *held 1 entry* over an area that held two and lost the second in silence
/// (driven, `completions/artifacts/M53/baseline-a3-2-failed-displacement.md` §4.1); a door
/// that under-reports its own subject is the law-1 half-truth
/// `crate::milestone::DESTROYING_DOORS` names.
///
/// Both displacing doors call it over **one working area's** outcome: the milestone boundary
/// narrates per sub-task area rather than once for the boundary (M52 Increment 4 / T4), so
/// the sentence's *"the working area"* stays the true singular it is here.
pub(crate) fn narrate_displacement(outcome: &Displacement) {
    let held = outcome.held();
    if held == 0 {
        return;
    }
    let mut message = format!(
        "note: the working area held {held} entr{} jigc did not write, and removing it would \
         have destroyed bytes no commit has a copy of",
        if held == 1 { "y" } else { "ies" },
    );
    if !outcome.moved.is_empty() {
        // The whole complement moved ⇒ the plain sentence, unchanged since M52: the count is
        // already the area's, and *they* is every entry it held.
        if outcome.unmoved.is_empty() {
            message.push_str(" — they were moved aside, not taken:\n");
        } else {
            message.push_str(&format!(
                " — {} of them moved aside, not taken:\n",
                outcome.moved.len(),
            ));
        }
        message.push_str(
            &outcome
                .moved
                .iter()
                .map(|m| format!("    {} → {}", m.from, m.to))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
    if !outcome.unmoved.is_empty() {
        if outcome.moved.is_empty() {
            message.push_str(" — not one of them could be moved aside:\n");
        } else {
            message.push_str(&format!(
                "\n  and {} could not be moved:\n",
                outcome.unmoved.len(),
            ));
        }
        message.push_str(
            &outcome
                .unmoved
                .iter()
                .map(|u| format!("    {} — {}", u.path, u.reason))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
    eprintln!("{message}");
}

/// Phase 7 (`design/finalize.md` → 7. Post-commit, best-effort). Three updates, none of
/// which can affect commit truth, all self-healing: advance the `file-state` record for
/// every committed file (the plan's managed-doc hash set plus the committed working-tree
/// files), **invalidate the edge-index stamp** (lifecycle site 5, `design/storage.md` →
/// Edge index lifecycle — remove the persisted index so the next read rebuilds against
/// the new HEAD), then remove the `cleanup_dir` working area. Each step self-heals on
/// failure, so a failure is logged to stderr, never raised (the commit is already truth).
///
/// `displace` is the removal's **keep** arm ([`AreaTeardown`]): given, the complement is moved
/// aside and narrated **before** the removal runs, and the removal itself is
/// [`engine::state::unwind_area`] rather than `remove_dir_all`, so the door destroys only
/// bytes it wrote (M53 Increment 2 / T3).
///
/// **`None` keeps `remove_dir_all`, and that is structural rather than a choice.** The unwind
/// is keyed on the area's own `engine::state::WorkArea` registry row, so without a kind there
/// is no membership to remove *by*. It is production-dead — all three call sites hand a
/// subject — and reached only by the unit test that drives the rejection arm, which never
/// gets to phase 7 at all.
///
/// **The kind is a parameter because this door has two kinds of subject** (M53 Increment 1 /
/// T4; `completions/artifacts/M53/settle-record.md` → D1.1). It shipped from M52 Increment 4 /
/// T3 taking the id alone and hard-coding `WorkArea::Task` here, with the two milestone
/// boundaries passing `None` on the stated ground that *"this door must not answer for a
/// subject it was not given"*. The subject **was** given: `cleanup_dir` at both milestone call
/// sites IS the milestone area, and the removal below is what tears it down. So the
/// sentence did not decline a subject — it declined to *look* at one it was already
/// destroying, and driven at `a4ce1d97` a landed `jigc milestone finalize` took an operator's
/// `merged/docs/deep.txt` out of a gitignored tree at **exit 0**, with an empty stderr, while
/// printing `"displaced": []` on a 1.0-pinned envelope.
fn post_commit(
    repo_root: &Path,
    jigc_root: &Path,
    cleanup_dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    hash_updates: &BTreeMap<String, String>,
    post_sweep: Option<FileStateRecord>,
    displace: Option<AreaTeardown<'_>>,
) {
    if let Err(err) = advance_file_state(repo_root, jigc_root, schemas, hash_updates, post_sweep) {
        eprintln!("note: post-commit file-state update failed (self-heals): {err:#}");
    }
    if let Err(err) = engine::index::invalidate(jigc_root) {
        eprintln!("note: post-commit edge-index invalidation failed (self-heals): {err:#}");
    }
    let Some(teardown) = displace else {
        if let Err(err) = std::fs::remove_dir_all(cleanup_dir) {
            eprintln!("note: post-commit working-area removal failed (self-heals): {err:#}");
        }
        return;
    };
    let outcome = displace_foreign_area(jigc_root, cleanup_dir, teardown.kind, teardown.unit_id);
    narrate_displacement(&outcome);
    // The area's fate is the caller's to report, not this function's to act on: the commit
    // is truth either way, and an area left standing is named by the advisory the unwind
    // pushes into `teardown.kept`.
    let _gone = unwind_settled_area(
        workbench_home(jigc_root),
        cleanup_dir,
        teardown.kind,
        teardown.unit_id,
        &outcome,
        teardown.kept,
    );
    // The sink is the landed envelope's `displaced` key, declared as the moves that
    // landed — so it takes `moved` alone, while the narration above answers for the
    // whole complement (M53 Increment 2 / T2).
    *teardown.moved = outcome.moved;
}

/// Everything phase 7 needs to answer for the working area it is about to tear down — the
/// **subject** and the two sinks its outcome leaves by (M52 Increment 4 / T3; M53 Increment 1 /
/// T4; M53 Increment 2 / T3).
///
/// It is a struct rather than a tuple because it grew a second sink and the two are not
/// interchangeable: one is the landed envelope's declared `displaced` key, the other is a
/// findings channel whose disposition differs **by door** — `jigc task finalize` puts its
/// entries on the landed `findings` array, and `jigc milestone finalize`, whose landed arm is
/// pinned at `Object(&["committed"])`, narrates them on stderr instead
/// (`completions/artifacts/M53/settle-record.md` → D2.5).
pub(crate) struct AreaTeardown<'a> {
    /// Which registry row `cleanup_dir`'s membership is decided by.
    pub(crate) kind: state::WorkArea,
    /// The work unit whose area it is — the advisory's target, and the parking home's name.
    pub(crate) unit_id: &'a str,
    /// The moves that landed, for `committed.displaced`.
    pub(crate) moved: &'a mut Vec<render::Displaced>,
    /// One `finalize.foreign-bytes` advisory per area this teardown left standing
    /// ([`kept_area_finding`]).
    pub(crate) kept: &'a mut Vec<Finding>,
}

/// **The removal is conditioned on the move** — remove what jigc wrote into `area` and then
/// the area, and where that leaves the area standing, say so (M53 Increment 2 / T3;
/// `completions/artifacts/M53/settle-record.md` → D2.3/D2.4 as amended by §6, §7).
///
/// This shipped as `remove_dir_all`, which is the one call that makes the displacement above
/// decorative: an entry whose move failed was taken a statement later by the very teardown the
/// move existed to spare it from, out of a gitignored tree with no second copy, at exit 0.
/// [`engine::state::unwind_area`] removes the area's own registry row and then the directory
/// through a non-recursive `remove_dir`, so a third party's bytes survive **by construction**
/// rather than by a check — the same discipline M52 Increment 5 already applied one door over
/// (`design/finalize.md` → Rollback discipline).
///
/// **Both non-clean answers raise the advisory**, `Foreign` and `Err` alike: the operator's
/// subject is one state — *a landed commit, and a working area kept because jigc could not
/// take it apart* — and the recovery is the same act in both. What differs is what the area
/// still holds, which is why the message is composed from a **re-read after the unwind**
/// rather than from the move's failure set (§6): the `Foreign` variant carries no paths, and
/// the cell that motivates the whole rule reports **zero** move failures — a `pre-commit` hook
/// that writes into the area during the commit can also leave it unreadable, so the
/// displacement enumerates nothing at all and the area still survives its own teardown.
///
/// Returns whether the area is **gone** — the fact a caller that acks a teardown keys its ack
/// on, never on having run the loop.
pub(crate) fn unwind_settled_area(
    jigc_home: &Path,
    area: &Path,
    kind: state::WorkArea,
    unit_id: &str,
    outcome: &Displacement,
    kept: &mut Vec<Finding>,
) -> bool {
    match state::unwind_area(area, kind) {
        Ok(state::AreaUnwind::Absent | state::AreaUnwind::Removed) => true,
        Ok(state::AreaUnwind::Foreign) => {
            kept.push(kept_area_finding(
                jigc_home, area, kind, unit_id, outcome, None,
            ));
            false
        }
        Err(err) => {
            kept.push(kept_area_finding(
                jigc_home,
                area,
                kind,
                unit_id,
                outcome,
                Some(&err),
            ));
            false
        }
    }
}

/// `finalize.foreign-bytes` — the identity a **landed** committing door raises over a working
/// area it could not tear down (M53 Increment 2 / T3; `settle-record.md` → D2.4 as amended by
/// §4, §5). The code is written here as a literal rather than through a const because
/// `crates/cli/tests/finalize_family_registry.rs` derives the family by scanning production
/// source for `Finding` constructors and reading their **code string literals**; a member
/// built from a const is invisible to it, which is that suite's own declared bound (a).
///
/// **One code for one state, at both displacing doors.** `crate::milestone`'s sibling rule —
/// *a second spelling would make one state answer two ways* — keys a code on the state **and
/// its route**, and the two `Disposition::Displace` doors reach one state through one seam
/// ([`unwind_settled_area`]) with one route: move the path out by hand. It is emphatically not
/// the mint-unwind state (`milestone.foreign-bytes` — nothing was committed there, and that
/// route says so) nor the discard state (a refusal *before* anything is removed).
///
/// **Advisory, and the exit stays 0** (`design/command-output-contract.md` → the exit
/// taxonomy): the commit is truth, the landed arm is exit 0, and a kept byte is not a reason
/// to tell a driver the run failed. It joins [`crate::render::FINALIZE_FAMILY`] and **not**
/// `crate::invocation_log::ERROR_CODE_REGISTRY`, which is the committing doors' commit-phase
/// *outcome* vocabulary — the rule this file already states one family over: a finding carried
/// beside a door's own result joins neither that registry nor `CHECK_INVENTORY`.
fn kept_area_finding(
    jigc_home: &Path,
    area: &Path,
    kind: state::WorkArea,
    unit_id: &str,
    outcome: &Displacement,
    fault: Option<&state::AreaUnwindError>,
) -> Finding {
    let listed = render::repo_relative(jigc_home, area);
    let target = match kind {
        state::WorkArea::Task => work_unit_ref(unit_id),
        state::WorkArea::Milestone => format!("milestone:{unit_id}"),
    };
    // §6 — the subject is what is in the area NOW. `AreaUnwind::Foreign` carries no paths,
    // and the move's failure set is empty on the cell that motivates the rule.
    let mut message = match state::foreign_area_paths(area, kind) {
        Ok(held) if held.is_empty() => format!(
            "`{listed}` was left standing after the commit landed, holding nothing but jigc's \
             own working files — `.jigc/` is gitignored, so nothing else has a copy of what \
             is there"
        ),
        Ok(held) => format!(
            "`{listed}` holds {} path(s) jigc did not write, so the working area was left \
             standing rather than removed with them — `.jigc/` is gitignored, so nothing else \
             has a copy of what is there: {}",
            held.len(),
            // `foreign_area_paths` answers **relative to the area**, and law 1's printed-path
            // rule has one home: re-root each entry and render it through
            // `render::repo_relative`, which is also the spelling the operator needs — a bare
            // `notes.txt` names nothing they can act on.
            held.iter()
                .map(|path| format!("`{}`", render::repo_relative(jigc_home, &area.join(path))))
                .collect::<Vec<_>>()
                .join(", "),
        ),
        Err(err) => format!(
            "`{listed}` was left standing after the commit landed and could not be read to \
             say what is in it ({err}) — `.jigc/` is gitignored, so nothing else has a copy of \
             what is there"
        ),
    };
    if let Some(err) = fault {
        message.push_str(&format!(
            "; the teardown stopped at `{}` ({}), leaving the rest of the area as found",
            render::repo_relative(jigc_home, &err.path),
            err.source,
        ));
    }
    // The failing move's reason, named **where one exists** (§6): the `Foreign` cell reached
    // by a hook's write reports none, and a sentence asserting one would be false there.
    if !outcome.unmoved.is_empty() {
        message.push_str(&format!(
            "; {} of them could not be moved aside: {}",
            outcome.unmoved.len(),
            outcome
                .unmoved
                .iter()
                .map(|entry| format!("`{}` — {}", entry.path, entry.reason))
                .collect::<Vec<_>>()
                .join(", "),
        ));
    }
    Finding::graded(
        Severity::Advisory,
        "finalize.foreign-bytes",
        message,
        Some(Location::addressed(target, 1, 1)),
        Some(engine::finding::Route::human(format!(
            "the commit landed and nothing in it is affected. Keep what you need from \
             `{listed}` and delete the rest — jigc mints no verb that clears it, because what \
             is in there is not jigc's to judge"
        ))),
    )
}

/// Record the committed working set into the `file-state` record and save it. The plan's
/// managed-doc hash set is empty in the commit-only case; the committed code files (the
/// working-tree changes that just landed) are hashed from `HEAD`'s tree so the next
/// `file-state` probe sees them in-sync.
///
/// When the caller carries a `post_sweep` record (the per-task preflight's post-sweep
/// state), it is the **base** the commit's updates land on, so an absorbed OOB baseline
/// the commit itself never touched persists too — fixing the verified re-fire defect
/// (`design/reconciliation.md` → Persistence of the shifted baseline). The sweep mints
/// **committed-store keys only** by construction (M43 A14 — staged working-area
/// instances are record-silent, [`TaskArea::validate`]; the pre-M43 per-run
/// `docs/<type>:<slug>.md` strip is retired as dead code), so the post-sweep record
/// lands as-is. The plan's hash set and the landed commit's re-hash apply on top,
/// winning on overlap. `None` (the milestone boundary, no sweep) loads the durable
/// record as before.
///
/// **The carried record's merge base must travel with it — never re-load here** (M46
/// Increment 1). `post_sweep` was loaded back at [`Task::validate`] and moved by value
/// through staging, the `git commit`, and the pre-commit hook's whole separate `jigc`
/// process, any of which may have written `.jigc/state/file-state.json` in the meantime.
/// `FileStateRecord::save` merges *our* per-key deltas onto whatever is on disk now,
/// relative to the base the load stashed — so a hook's `jigc unmanage` mid-commit stays
/// unmanaged. Swapping this `Some(swept)` for a fresh `FileStateRecord::load` would
/// compute the delta against a base that already contains the concurrent write and
/// silently void the merge on the most important path in the product, without breaking
/// a single type (`engine::file_state::FileStateRecord::load` carries the same note).
fn advance_file_state(
    repo_root: &Path,
    jigc_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    hash_updates: &BTreeMap<String, String>,
    post_sweep: Option<FileStateRecord>,
) -> Result<()> {
    let mut record = match post_sweep {
        Some(swept) => swept,
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

/// Probe the repo's **staged state** as a carryover-gate [`engine::state::StagedSnapshot`]
/// — the one git probe every task-minting door runs (`design/surface-contract.md` →
/// The carryover gate; M43 T1): `git diff --cached --raw -z`, whose raw records carry
/// both halves the gate needs — the staged blob hash per changed path (adds +
/// modifications become `entries`) *and* the `D` status a `--name-only` listing is
/// blind to (staged deletions become `deletions`). Flag pins: `--no-renames` so a
/// staged rename reports as its D + A halves (the deletion half is exactly what the
/// entry-only view misses) and the parse is independent of the user's `diff.renames`
/// config; `--abbrev=40` because raw output abbreviates object names by default and
/// the snapshot's blobs must compare stably at finalize; `-z` so a path containing a
/// space or newline survives verbatim (the [`git_staged_paths`] rationale).
pub(crate) fn git_staged_snapshot(repo_root: &Path) -> Result<engine::state::StagedSnapshot> {
    let out = Command::new("git")
        .args([
            "diff",
            "--cached",
            "--raw",
            "-z",
            "--no-renames",
            "--abbrev=40",
        ])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git diff --cached --raw -z` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let text =
        String::from_utf8(out.stdout).context("`git diff --cached` produced non-UTF-8 output")?;
    // With `-z` each record is `:<oldmode> <newmode> <oldsha> <newsha> <status>` NUL
    // `<path>` NUL — alternating meta/path tokens, plus one empty trailing token.
    let mut snapshot = engine::state::StagedSnapshot::default();
    let mut tokens = text.split('\0').filter(|t| !t.is_empty());
    while let Some(meta) = tokens.next() {
        let path = tokens
            .next()
            .context("`git diff --cached --raw -z` emitted a meta record without its path")?;
        let fields: Vec<&str> = meta.trim_start_matches(':').split(' ').collect();
        let [_, _, _, new_sha, status] = fields[..] else {
            bail!("`git diff --cached --raw -z` emitted an unrecognized record: {meta:?}");
        };
        if status == "D" {
            snapshot.deletions.insert(path.to_owned());
        } else {
            snapshot.entries.insert(path.to_owned(), new_sha.to_owned());
        }
    }
    Ok(snapshot)
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
pub(crate) fn git_changed_paths(
    repo_root: &Path,
    base_sha: &str,
    head_sha: &str,
) -> Result<Vec<String>> {
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
pub(crate) fn git_dirty_paths(repo_root: &Path) -> Result<Vec<String>> {
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

/// Whether HEAD carries any history for `path` — `git log HEAD -1 -- <path>` prints at
/// least one line. The M45 file-state history gate ([`engine::validate::HistoryPredicate`],
/// Decision 7) consults this to distinguish a genuine deletion (path has history, still gone
/// → block) from a dangling baseline the checkout moved out from under the gitignored
/// file-state cache (no history → advisory + `jigc unmanage` prune route). `-1` bounds the
/// walk to the first touching commit (presence is all the gate needs).
pub(crate) fn git_path_has_history(repo_root: &Path, path: &str) -> Result<bool> {
    let out = Command::new("git")
        .args(["log", "HEAD", "-1", "--format=%H", "--", path])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git log HEAD -1 -- {path}` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(!String::from_utf8_lossy(&out.stdout).trim().is_empty())
}

/// The bytes of the **last committed version** of `path`, or `None` when HEAD carries no
/// history for it at all.
///
/// `git log HEAD -1 --format=%H -- <path>` names the most recent commit reachable from HEAD
/// that touched the path — [`git_path_has_history`]'s own query, asked for *which* commit
/// rather than *whether one exists*. The blob is then read from that commit when the path
/// still exists there (the commit added or modified it, including the uncommitted-deletion
/// cell where HEAD itself still carries the file) and from its **first parent** when that
/// commit removed it — a `git rm`, or the source half of a `git mv`.
///
/// **A failure is a failure, not an absence.** Every unanswerable shape — git missing, a
/// blob readable at neither revision (a merge whose first parent never had the path), a
/// malformed revision — is an `Err`, so a caller that must read conservatively can, and one
/// that must not cannot mistake it for *there was nothing there*. Only the empty `git log`
/// is `Ok(None)`.
pub(crate) fn git_last_committed_blob(repo_root: &Path, path: &str) -> Result<Option<Vec<u8>>> {
    let out = Command::new("git")
        .args(["log", "HEAD", "-1", "--format=%H", "--", path])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git log HEAD -1 -- {path}` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let commit = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if commit.is_empty() {
        return Ok(None);
    }
    for rev in [commit.clone(), format!("{commit}^")] {
        let show = Command::new("git")
            .args(["cat-file", "blob", &format!("{rev}:{path}")])
            .current_dir(repo_root)
            .output()
            .context("could not run `git` (is it on PATH?)")?;
        if show.status.success() {
            return Ok(Some(show.stdout));
        }
    }
    bail!(
        "`git cat-file blob {commit}:{path}` and its first parent both failed — the commit \
         that last touched `{path}` carries no readable blob for it"
    )
}

/// Whether the tree at `rev` carries `path` — `git ls-tree -r --name-only <rev> -- <path>`
/// prints at least one line. The path is passed as a **pathspec** after `--`, never spliced
/// into a revision spec, so a home carrying a `:` or a leading `-` is still asked about the
/// file it names.
///
/// Used by the vacated-home check to tell a removal the repository **committed** from one it
/// only holds in the worktree or the index (`crate::orphan::Removal`).
pub(crate) fn git_rev_tracks_path(repo_root: &Path, rev: &str, path: &str) -> Result<bool> {
    let out = Command::new("git")
        .args(["ls-tree", "-r", "--name-only", rev, "--", path])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git ls-tree -r --name-only {rev} -- {path}` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(!String::from_utf8_lossy(&out.stdout).trim().is_empty())
}

/// Whether the **index** carries `path` — `git ls-files -- <path>` prints at least one line.
/// The complement of [`git_rev_tracks_path`] at `HEAD`: together they separate a staged
/// deletion (in `HEAD`, not in the index) from a worktree-only one (in both).
pub(crate) fn git_index_tracks_path(repo_root: &Path, path: &str) -> Result<bool> {
    let out = Command::new("git")
        .args(["ls-files", "--", path])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git ls-files -- {path}` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(!String::from_utf8_lossy(&out.stdout).trim().is_empty())
}

/// The abbreviated sha of the most recent commit reachable from `HEAD` that **removed**
/// `path`, or `None` when no such commit is named — `git log --diff-filter=D -1 --format=%h`.
///
/// **It is run so that its answer, rather than the command, can be printed.** A route that
/// hands the reader a locator is claiming the locator names something; on every cell where
/// the removal is not committed, and on a merge-only removal or a shallow clone where it is,
/// this query answers nothing — so the caller asks it first and drops the clause when the
/// answer is empty (M52 completion audit, fix 6). A git failure is `None` for the same
/// reason: an unanswerable question names no commit either.
pub(crate) fn git_deleting_commit(repo_root: &Path, path: &str) -> Option<String> {
    let out = Command::new("git")
        .args(["log", "--diff-filter=D", "-1", "--format=%h", "--", path])
        .current_dir(repo_root)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let sha = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!sha.is_empty()).then_some(sha)
}

/// The canonical git empty-tree SHA — the sentinel base a **zero-commit** (unborn
/// HEAD) repo pins to so every `creates-task` workflow runs pre-first-commit
/// (`design/project-setup.md` → Flow 2 hardening — zero-commit;
/// `implementation/roadmap.md` → M21 Increment 2).
///
/// **The mint half stands; the finalize half is struck** (M51 Increment 2 / T3). This
/// comment also claimed *"and the first finalize diffs against the empty tree"*, and
/// since the repository-posture guard that is false: `jigc task finalize` is
/// commit-on-behalf ([`crate::cli::BEHALF_DOORS`]) with no stated exemption, so an
/// unborn HEAD refuses with `repo.head-unborn` before the pin is read. Driven, the
/// falsifying datum: on a fresh `git init`, `jigc task finalize <id>` now exits non-zero
/// naming *land the repository's first commit with `git commit`*. The mint side is
/// untouched — `jigc start` is `Neither`, adjudicates no posture, and still pins this
/// sentinel on a zero-commit repo — and the documented on-ramp runs `jigc setup` first,
/// whose install commit (exempt from the unborn member, by the M30 audit rationale)
/// births HEAD before any work is finalized.
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
/// On a **zero-commit** repo (unborn HEAD) returns the [`EMPTY_TREE_SHA`] sentinel, so a
/// task minted pre-first-commit pins and compares sentinel-vs-sentinel
/// (`design/project-setup.md` → Flow 2 hardening — zero-commit). The *"so the first
/// finalize diffs against the empty tree"* half of this sentence is **struck** — see
/// [`EMPTY_TREE_SHA`] for the falsifying datum: since M51 Increment 2 an unborn HEAD
/// refuses at every commit-on-behalf door, so no finalize reaches this branch.
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

/// `git commit -F <message-file>` (`design/finalize.md` → 6. Commit) — the message-file
/// form of the one hook-capable commit seam [`git_commit_capture`]; see there for the
/// hook posture, the captured-stream contract, and the typed subject.
pub fn git_commit(subject: &SeamSubject, message_file: &Path) -> Result<String> {
    git_commit_capture(
        subject,
        &[std::ffi::OsStr::new("-F"), message_file.as_os_str()],
    )
}

/// `git commit --amend -F <message-file>` (F-10; `design/finalize.md` → The amend arm) —
/// [`git_commit`]'s second commit model, through the identical hook-capable seam, so the
/// posture re-probe, the captured hook stream and the rejection typing are the same on both.
///
/// **The tree is not a parameter and cannot be.** `git commit --amend` rewrites `HEAD` from
/// **the index**, and this arm reaches here having staged nothing ([`StagePolicy::Amend`])
/// over an index its own gate already refused to let be non-empty
/// (`finalize.amend-index-dirty`). That gate is what makes this a message-only rewrite:
/// without it git folds the whole index in silently at exit 0, which is the driven hazard
/// the arm exists to close rather than a corner of it.
pub fn git_commit_amend(subject: &SeamSubject, message_file: &Path) -> Result<String> {
    git_commit_capture(
        subject,
        &[
            std::ffi::OsStr::new("--amend"),
            std::ffi::OsStr::new("-F"),
            message_file.as_os_str(),
        ],
    )
}

/// The **only** exit code a hook failure can reach the seam with — git normalizes every one of
/// them to `1`, so a code that is not this one was git's own refusal and no hook can be blamed
/// for it (M52 Increment 3 / T5; `completions/artifacts/M52/baseline-posture.md` §2.9, L8;
/// `settle-record.md` → D2.3).
///
/// **Driven on git 2.54.0 (Apple Git-157)**, in both directions and over every hook failure
/// mode reachable: a `pre-commit` hook exiting 1, 42 or **128** (git's own fatal code), a
/// `commit-msg` hook exiting 7, a hook whose shebang cannot be executed
/// (`fatal: cannot exec '.git/hooks/pre-commit'`), and a hook killed by `SIGKILL`
/// (`error: … died of signal 9`) — every one arrives as exit **1**. git's own refusals arrive
/// as **128** (`gpg failed to sign the data`, a partial commit during a pick, an unmerged
/// index) or **129** (a usage error, which would be jigc's own argv).
///
/// **The hook cell is keyed on `== 1`, not the git cell on `== 128`**, and the difference is a
/// third code: keyed the other way, a 129 would inherit the hook diagnosis — the same lie one
/// code over. The normalization this rests on is pinned against the host's real git by
/// `commit_rejected_axis::a_hook_exiting_gits_own_code_is_still_framed_as_a_hook_rejection`,
/// because a git that stopped normalizing would flip a genuine hook rejection into the
/// non-hook frame.
///
/// **Declared bound:** exit 1 is *also* git's refusal to record an empty commit, which is why
/// that cell is discriminated by each door **before** the seam (M48 Increment 8, `nothing_staged`)
/// rather than here — an exit code cannot separate those two, and the door can.
const GIT_HOOK_EXIT: i32 = 1;

/// git's two streams, relayed **apart**: each trimmed, joined by a newline, and an empty one
/// dropped rather than left as a blank line.
///
/// It was `format!("{}{}", stdout.trim(), stderr.trim())`, which glued git's last stdout token
/// to its first stderr token whenever git wrote to both — driven at the baseline as
/// `U\tsq.txterror: Committing is not possible because you have unmerged files.`
/// (`completions/artifacts/M52/baseline-posture.md` §2.9, L6). The relay is supposed to be
/// verbatim (law 1), and two tokens welded into one word are not git's bytes.
fn relay_git_streams(stdout: &str, stderr: &str) -> String {
    [stdout.trim(), stderr.trim()]
        .into_iter()
        .filter(|stream| !stream.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The ONE **hook-capable commit seam**: `git commit <args…>` in the checkout `subject`
/// names. **Never**
/// passes `--no-verify`: the user's `pre-commit` / `commit-msg` hooks are policy and the
/// CLI respects them — a hook rejection surfaces git's stdout+stderr verbatim in the
/// typed [`CommitRejected`] (the correction signal), and no commit lands.
///
/// **A non-zero exit is not proof a hook spoke** (M52 Increment 3 / T5): git's own refusals
/// arrive here identically, and until this increment every one of them was relayed under
/// *"`git commit` was rejected"* and closed with *"Fix the hook's complaint"* — a law-1 lie
/// and an unfollowable route at ten doors, driven with an ordinary `commit.gpgsign`
/// (`completions/artifacts/M52/baseline-posture.md` §2.9). [`GIT_HOOK_EXIT`] is the
/// discriminator and carries its driven basis; [`relay_git_streams`] keeps git's two streams
/// apart while doing it.
///
/// On a **successful** commit returns the captured hook output so the caller can surface
/// a non-blocking hook's warning — e.g. the M19 doc↔code backstop, which warns but exits
/// 0 (`design/finalize.md` → 6. Commit, success-relay). git redirects a hook's own stdout
/// to stderr and writes only its own commit summary ("[branch sha] message", file stats)
/// to stdout, so **stderr is the hook stream**: capturing it is general (any non-blocking
/// hook, not just jigc's backstop) and a no-hook commit yields empty. The bytes are
/// merely captured here — placement (the `hook_output` envelope key + the stderr relay,
/// `design/command-output-contract.md` → Stream discipline) is the caller's job.
///
/// **The producer-axis fence** (confidence-audit sibling-hunt item 4): every production
/// site that runs a hook-capable `git commit` on the user's behalf funnels through here —
/// the finalize paths via [`git_commit`], the milestone record-only commits
/// (`milestone::git_commit_pathspec`), `jigc rename`, and `jigc migrate-corpus`'s
/// self-commit — so the captured stream is *returned* at every site and dropping it is
/// visible at the call site, never a silent `git_run` discard. The one exclusion is
/// `setup`'s install commit, `--no-verify` by recorded design (its hook must not
/// self-trigger on the commit that installs it). A new commit site joins the axis by
/// calling this and surfacing the returned stream (`tests/hook_output_axis.rs`).
///
/// **The subject is typed, and re-probed here — immediately before the act** (M51
/// Increment 2; `settle-record.md` → Review amendments §3). The door adjudicated the
/// repository posture before anything was resolved; a per-task finalize then validates,
/// promotes, retires and stages, and the fan-out boundary runs the user's hooks in a
/// worktree sharing this `.git` — so a hook, a concurrent process or an earlier phase of
/// this same run can move HEAD in between. [`SeamSubject`] also records **which checkout
/// this is**, because the seam cannot tell: a fan-out worktree answers `git symbolic-ref
/// -q HEAD` exactly as a user's detached HEAD does, so a caller must say, from a live
/// `DedicatedWorktree` handle. A breach refuses here and **no commit is made**.
pub(crate) fn git_commit_capture(
    subject: &SeamSubject,
    args: &[&std::ffi::OsStr],
) -> Result<String> {
    subject.verify(SeamAct::Commit)?;
    let out = Command::new("git")
        .arg("commit")
        .args(args)
        .current_dir(subject.path())
        .output()
        .context("could not run `git commit` (is git on PATH?)")?;
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let relayed = relay_git_streams(&stdout, &stderr);
        // M42 — the rejection is a TYPED error (`CommitRejected`) carrying git's bytes
        // verbatim, so the per-task surface (which knows the task id) can frame it with
        // the recoverability route. Its `Display` is this same message, so the callers
        // that only print `{err:#}` are byte-unchanged.
        //
        // **Who refused decides the headline and the route** (M52 Increment 3 / T5) — the
        // discriminator is [`GIT_HOOK_EXIT`], see there.
        let by_hook = out.status.code() == Some(GIT_HOOK_EXIT);
        let headline = if by_hook {
            "`git commit` was rejected (no commit was made):"
        } else {
            "`git commit` failed (no commit was made):"
        };
        return Err(anyhow::Error::new(CommitRejected {
            cause: format!("{headline}\n{relayed}"),
            by_hook,
        }));
    }
    Ok(stderr.trim_end().to_owned())
}

/// Whether **nothing under `pathspec` is staged** — `git diff --cached --quiet -- <pathspec>`
/// exits 0 exactly when the index matches HEAD there (with no HEAD it diffs the empty tree,
/// so a first commit still reports changes).
///
/// The committing axis's **empty-commit discriminator**, the pre-flight side of
/// [`git_commit_capture`]: git refuses a commit that would record no change, and that refusal
/// arrives at the seam as a non-zero exit indistinguishable from a hook's — so a door that
/// commits unconditionally frames *"`git commit` was rejected"* over a run nobody rejected,
/// routes to a re-run that can only fail identically, and relays git's unrelated untracked-file
/// listing as if it were the cause (M48 Increment 8; `completions/artifacts/RC-pre-1.0/v1-walk.md`
/// → Arm 6). A door asks this **before** committing and acks its own no-op instead. The
/// predicate is the shipped shape of `migrate_corpus`' and `setup`'s own skips, lifted beside
/// the commit seam the axis shares so a door inherits it rather than re-deriving it.
///
/// A git that cannot be spawned reads as **`false`** (something may be staged): the commit
/// then runs and fails loudly at the seam, never silently claiming a no-op.
pub(crate) fn nothing_staged(repo_root: &Path, pathspec: &[&str]) -> bool {
    let mut args: Vec<&str> = vec!["diff", "--cached", "--quiet", "--"];
    args.extend(pathspec);
    Command::new("git")
        .args(&args)
        .current_dir(repo_root)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
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
    record: Option<&str>,
    plan: &engine::finalize::FinalizePlan,
    msg_path: &Path,
) -> Result<String> {
    // base == HEAD (the milestone preflight guaranteed it, or advanced over a record-only
    // range, before we got here), so the fan-out's pin tree is this checkout's HEAD tree.
    let code_tree = match crate::combine::combine_worktree_trees(repo_root, "HEAD", worktrees)? {
        crate::combine::CombineOutcome::Combined(tree) => tree,
        crate::combine::CombineOutcome::Blocked(finding) => return Err(finding_to_err(finding)),
    };
    // Overlay the promoted docs + config + the flipped milestone record onto the combined code
    // tree and commit it (with the user's hooks) as a child of HEAD, then fast-forward main —
    // the shared WIP-safe aggregate.
    let head = git_head(repo_root)?;
    overlay_docs_commit_and_ff(repo_root, &code_tree, &head, record, plan, msg_path)
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
    record: Option<&str>,
    plan: &engine::finalize::FinalizePlan,
    msg_path: &Path,
) -> Result<String> {
    // The live checkout's expectation, recorded BEFORE the boundary's own commit: the
    // dedicated commit below runs the user's hooks, which share this `.git`, so the ref
    // this fast-forward was decided for can move between here and the `--ff-only`.
    let live = SeamSubject::live(repo_root);
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
    // The milestone record's in-place `joined` flip folds into THIS finalize commit
    // (`design/team-ready-state.md` → The commit model: join folds) — path-added alongside the
    // promoted docs. The record is a committed managed doc outside `.jigc/`, already flipped on
    // disk by [`engine::milestone::join_record`], so a targeted `git add` stages exactly its
    // joined bytes into both the off-line tree and (below) the live index for the fast-forward.
    if let Some(record) = record {
        pathspecs.push(record.to_owned());
    }
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
    // Re-probe immediately before the act: a fast-forward advances whatever ref HEAD is
    // on, so landing it on a ref that moved under us would put the boundary's commit on a
    // branch nobody asked for. The dedicated commit already ran; refusing here lands
    // nothing on the live checkout and the caller's rollback restores the promoted docs.
    live.verify(SeamAct::Commit)?;
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
/// tree, never the retired `--allow-empty` form) and the user's hooks run. The disjoint
/// patches (the caller's up-front collision block guarantees disjointness) apply cleanly in
/// sequence.
///
/// Then overlay the merged docs + config onto the last sub-task commit's tree and commit the
/// aggregate (hooks running) as its child, and fast-forward main onto the whole chain — the
/// shared [`overlay_docs_commit_and_ff`]. The dedicated worktree is torn down on drop; on ANY
/// abort (a per-sub-task or the aggregate hook rejection) it returns `Err` having committed
/// NOTHING to main — HEAD stays at the pre-finalize sha and unrelated WIP is intact.
///
/// Returns **every** chain commit's captured non-blocking hook stream — the N per-sub-task
/// streams + the aggregate's, folded in commit order ([`fold_hook_streams`]) — for the
/// caller's ONE success-relay + envelope (the hook_output producer axis: relaying each chunk
/// as it landed put the per-sub-task streams on stderr only, so a JSON driver reading the
/// landed envelope never saw them; `design/command-output-contract.md` → Stream discipline).
fn chain_commit(
    repo_root: &Path,
    subtasks: &[(Vec<u8>, String)],
    record: Option<&str>,
    plan: &engine::finalize::FinalizePlan,
    msg_path: &Path,
) -> Result<String> {
    let head = git_head(repo_root)?;
    // The per-sub-task commits accrue in a dedicated worktree at HEAD — never the live
    // checkout, so an abort leaves main untouched (no `git reset --hard`).
    let dedicated = DedicatedWorktree::add(repo_root, &head)?;
    let wt = dedicated.path();
    // The seam's subject, built from the live handle — the only way the dedicated variant
    // exists (`SeamSubject::dedicated`). Every commit in this loop runs in a
    // worktree jigc detached itself, so `repo.head-detached` is exempt here and nowhere
    // else; a merge left un-concluded INSIDE it still refuses.
    let subject = SeamSubject::dedicated(&dedicated);
    let sub_msg = wt.join(".jigc-subtask-message.tmp");
    let mut streams: Vec<String> = Vec::new();
    for (patch, message) in subtasks {
        git_apply_index(wt, patch)?;
        std::fs::write(&sub_msg, message).with_context(|| {
            format!("could not write the sub-task commit message to {sub_msg:?}")
        })?;
        let commit_result = git_commit(&subject, &sub_msg);
        let _ = std::fs::remove_file(&sub_msg);
        // Every fan-out commit runs the user's hooks (M31 Inc 5; `design/finalize.md` →
        // 6. Commit); each captured stream folds into the one returned string below.
        streams.push(commit_result?);
    }
    // The aggregate carries the merged docs (a milestone-level merge artifact, review B1/B2),
    // built off the last sub-task commit's tree and fast-forwarded onto main with the chain.
    let subtask_head = git_head(wt)?;
    streams.push(overlay_docs_commit_and_ff(
        repo_root,
        &subtask_head,
        &subtask_head,
        record,
        plan,
        msg_path,
    )?);
    Ok(fold_hook_streams(streams.iter().map(String::as_str)))
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
    // The seam is TOLD this is a worktree jigc provisioned, from the handle itself — it
    // cannot be sniffed: this throwaway worktree is not a registered sub-task, so
    // `repo::posture_subject` would classify it live and refuse jigc's own commit site.
    let hook_output = git_commit(&SeamSubject::dedicated(&dedicated), msg_path)?;
    let commit = git_head(wt)?;
    Ok((commit, hook_output))
}

/// Check an off-line-built `tree` out into a **throwaway detached worktree** (added at
/// `base`, then `read-tree --reset -u`ed to `tree`), returning the worktree handle whose
/// [`path`](DedicatedWorktree::path) is the tree on disk. The milestone-boundary
/// conformance gate points the `doc-code` probe's `working_tree_root` at it, because the
/// probe resolves anchors by **filesystem path** while the combine returns only a tree SHA
/// (`design/finalize.md` → 2. Validate — How the `doc-code` arm reaches the merged code).
/// The returned handle tears the worktree down on drop, so a blocked *and* a clean gate exit
/// both leave `.jigc/worktrees/` clean. Never mutates the live index/worktree — the
/// dedicated worktree is a separate checkout sharing only the object DB.
pub(crate) fn checkout_tree_worktree(
    repo_root: &Path,
    base: &str,
    tree: &str,
) -> Result<DedicatedWorktree> {
    let dedicated = DedicatedWorktree::add(repo_root, base)?;
    git_run(dedicated.path(), &["read-tree", "--reset", "-u", tree])?;
    Ok(dedicated)
}

/// A throwaway **detached** git worktree for the squash:true hook-running combine commit,
/// removed on drop (`git worktree remove --force` + `prune`). A linked worktree shares the
/// main repo's `.git` (object DB + hooks), so a commit made here runs the user's shared
/// `pre-commit`/`commit-msg` hooks; committing here instead of the main checkout keeps the
/// combine WIP-safe (the main index/worktree are never the commit site). Lives under the
/// gitignored `.jigc/worktrees/` parent so a leaked dir never pollutes `git status`.
pub(crate) struct DedicatedWorktree {
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

    pub(crate) fn path(&self) -> &Path {
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
///   `included` and shows its worktree residual here, so it appears in **both**. On the
///   doc-only arm (`doc_only`, M55) the commit took named paths and left the index's other
///   entries staged, so **every** residual entry is left out, one per path
///   ([`left_out_entry`]) — the same rule the forecast applies, so the two sets are one.
fn classify_landed_manifest(
    name_status: Vec<(char, String)>,
    porcelain: Vec<(String, String)>,
    promoted: &std::collections::HashSet<String>,
    doc_only: bool,
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
    let left_out = porcelain
        .into_iter()
        .filter_map(|(code, path)| left_out_entry(&code, path, doc_only))
        .collect();
    (included, left_out)
}

/// The **left-out** entry one porcelain line contributes, or none — the one rule the
/// `--dry-run` forecast ([`TaskArea::predict_manifest`]) and the landed residual
/// ([`classify_landed_manifest`]) share, so the two sets cannot be classified two ways.
///
/// On the index-committing models only the **Y (worktree)** column is left out: the commit
/// took the index, so a blank Y means the path matches what was committed. On the
/// **doc-only** model (`doc_only`, M55) the commit took named paths, so every entry outside
/// them is left out — **one entry per path**, staged ones included — tagged by its **X
/// (index)** column when the path is staged (the fact this arm keeps intact for the task it
/// belongs to) and by its Y column otherwise.
fn left_out_entry(code: &str, path: String, doc_only: bool) -> Option<render::ManifestEntry> {
    let mut columns = code.chars();
    let x = columns.next().unwrap_or(' ');
    let y = columns.next().unwrap_or(' ');
    let column = if doc_only && x != ' ' && x != '?' {
        x
    } else if y != ' ' {
        y
    } else {
        return None;
    };
    Some(render::ManifestEntry {
        path,
        kind: column_kind(column),
    })
}

/// Whether the pathspec `spec` jigc hands `git add` covers the repo-relative `path` — the
/// file itself, or anything under it when `spec` names a directory (with or without its
/// trailing `/`). The membership test of the two stages that `git add` paths of their own:
/// the index-honoring stage's config layer and stamp, and the doc-only stage's
/// owner-artifacts.
fn pathspec_covers(spec: &str, path: &str) -> bool {
    let spec = spec.trim_end_matches('/');
    path == spec
        || path
            .strip_prefix(spec)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// The manifest kind of a dirty path **jigc's own stage** will `git add` — it rides the
/// commit whichever porcelain column is dirty: an untracked one is `added`, otherwise the
/// index column when it is already staged, else the worktree column the stage will stage.
fn self_staged_kind(code: &str) -> render::ManifestKind {
    let mut columns = code.chars();
    let x = columns.next().unwrap_or(' ');
    let y = columns.next().unwrap_or(' ');
    match (x, y) {
        ('?', _) => render::ManifestKind::Added,
        (' ', _) => column_kind(y),
        _ => column_kind(x),
    }
}

/// Whether `path` (repo-relative) is in the **index** (`git ls-files -- <path>` prints
/// it). The staging discriminator for a retirement pathspec (M40 F7): a user who
/// pre-staged the deletion (`git rm` before finalize) has removed the index entry —
/// while HEAD still carries the path — so only the index tells the stage whether the
/// pathspec would match anything.
///
/// **Also the `jigc migrate` door's trackedness leg** (M51 Increment 1 / T2), and shared
/// rather than copied on purpose: [`stage_migration`] skips a retirement this predicate
/// refuses — *"an untracked foreign was never in the index (nothing to stage)"* — which is
/// precisely the silent-loss cell the door now refuses up front. Two spellings of one
/// question would let the door admit a source the stage then drops.
///
/// **The path is handed to git as `:(literal)`, and that is the difference between asking
/// about this file and asking about a pattern** (M51 Increment 1, the axis fix). `--` prevents
/// option parsing and nothing else: driven, `git ls-files -- '*.md'` printed six tracked files
/// for a token naming a file git had never seen, so this predicate answered **tracked** for an
/// untracked source and the door's trackedness leg passed the one cell it exists to catch.
/// `crate::trackable::pathspec_magic_reason` refuses such a token at both doors that reach
/// here; the magic prefix is the belt to that pair of braces, so a caller that has not asked —
/// including a future one — gets an answer about the file it named.
pub(crate) fn path_in_index(repo_root: &Path, path: &str) -> bool {
    let literal = format!(":(literal){path}");
    Command::new("git")
        .args(["ls-files", "--", literal.as_str()])
        .current_dir(repo_root)
        .output()
        .map(|out| out.status.success() && !out.stdout.is_empty())
        .unwrap_or(false)
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

/// Map an engine [`Finding`] to an `anyhow` error carrying its **key** + message +
/// route — the same envelope the front door and the write verbs use.
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
    crate::render::finding_error(&finding)
}

/// Materialize the versions of `changed` files at an arbitrary `treeish` into a fresh,
/// self-cleaning scratch tree — the general form of [`Task::materialize_head_subset`], keyed
/// to any commit/tree rather than `HEAD`. The milestone-boundary conformance gate feeds the
/// milestone's **shared base** (`base.sha`, not `HEAD`) so the merged code-anchor blast
/// radius decides *newly*-dangled against the base every sub-task inherited
/// (`design/finalize.md` → 2. Validate; `design/validation.md` → The milestone-boundary
/// gate). Only files present as a **blob** at `treeish` are written (a path absent or a
/// directory there → `git cat-file blob` fails → skipped, so it correctly counts as
/// not-resolved-at-base). An empty `changed` set yields an empty tree (the blast radius is
/// then inert). The [`ScratchTree`] removes itself on drop, so a clean *or* blocked gate
/// leaks nothing.
pub(crate) fn materialize_treeish_subset(
    repo_root: &Path,
    treeish: &str,
    changed: &BTreeSet<String>,
) -> Result<ScratchTree> {
    let tree = ScratchTree::new();
    std::fs::create_dir_all(tree.path())
        .with_context(|| format!("could not create the base scratch tree {:?}", tree.path()))?;
    for path in changed {
        let out = Command::new("git")
            .args(["cat-file", "blob", &format!("{treeish}:{path}")])
            .current_dir(repo_root)
            .output()
            .context("could not run `git cat-file` for the base tree")?;
        if !out.status.success() {
            continue; // absent / non-blob at treeish → not resolved at base
        }
        let dest = tree.path().join(path);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(&dest, &out.stdout)
            .with_context(|| format!("could not write base blob {dest:?}"))?;
    }
    Ok(tree)
}

/// A process-and-time-unique temp directory that removes itself on drop — the scratch
/// tree [`Task::materialize_index`] checks the git index out into (the `store_scratch_path`
/// / `describe::TempDir` idiom). Self-cleaning so a clean *or* blocked validate leaks
/// nothing.
pub(crate) struct ScratchTree(PathBuf);

impl ScratchTree {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ScratchTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Every typed error the executor's channel carries survives the N20 marking** — the
    /// fence under [`already_typed`]'s list (M51 Increment 2 / T5).
    ///
    /// The list is a hand-written set, which this repo only tolerates while something reddens
    /// when it goes stale: each of these is read back by a `downcast_ref` in a door's surface
    /// *after* [`mark_commit_failure`] has run, so wrapping one makes that read miss and the
    /// door prints a generic frame over a refusal that had its own diagnosis, code and route.
    /// The owner-artifact gate is the driven instance — marking it once turned
    /// `flow20`'s `owner-artifact.present` block into a bare frame at exit 1.
    #[test]
    fn typed_errors_pass_through_the_commit_failure_marker() {
        let typed: Vec<(&str, anyhow::Error)> = vec![
            (
                "CommitRejected",
                anyhow::Error::new(CommitRejected {
                    cause: "hook said no".to_string(),
                    by_hook: true,
                }),
            ),
            (
                "StageGitFailure",
                anyhow::Error::new(StageGitFailure("git add failed".to_string())),
            ),
            (
                "OwnerArtifactBlock",
                anyhow::Error::new(OwnerArtifactBlock(vec![
                    engine::finding::Finding::blocking(
                        "owner-artifact.present",
                        "the recorded artifact is absent",
                        Location::addressed("task:probe".to_string(), 1, 1),
                    ),
                ])),
            ),
            (
                "BlockedFinding",
                crate::render::finding_error(&engine::finding::Finding::blocking(
                    "repo.head-detached",
                    "HEAD is detached",
                    Location::addressed("task:probe".to_string(), 1, 1),
                )),
            ),
        ];
        for (name, err) in typed {
            let marked = mark_commit_failure(err);
            assert!(
                !marked.is::<CommitFailed>(),
                "`{name}` is discriminated by a door's surface after the marking, so it must \
                 pass through unwrapped",
            );
        }
        // …and a bare operational error IS marked — else the fence passes vacuously.
        let bare = mark_commit_failure(anyhow::anyhow!("`git merge --ff-only` failed"));
        assert!(
            bare.is::<CommitFailed>(),
            "an untyped commit-transaction failure must be marked, or N20's whole cell is inert",
        );
    }

    /// **Every pathspec-magic spelling is refused at the sink** — the magic leg of
    /// [`ValidatedRetirement::adjudicate`], swept over git's own magic forms rather than
    /// pinned at the one an end-to-end arm happens to drive.
    ///
    /// **The axis is git's two magics, not its prefix one.** The leg shipped as
    /// `starts_with(':')` and the wildmatch half — `*`, `?`, `[`, `\` — was open at every
    /// site: driven, a recorded `*.md` reached `git add -- '*.md'` and staged an unrelated
    /// worktree edit into the migration commit while the literal file it named was unlinked
    /// with no git object holding it. Both halves are asked in one home
    /// ([`crate::trackable::pathspec_magic_reason`]) and swept here in one list.
    ///
    /// It is a unit test and not a second integration arm for a stated reason: the
    /// parenthesised forms cannot *reach* this predicate through the binary, because
    /// `engine::file_state::ConflictBlock::task` composes `jigc unmanage <source>` as a
    /// `Route::mechanical` eagerly on every migration finalize and the route fence's
    /// `shell_safe` leg refuses `(`/`)` long before the commit closure opens — a pre-existing
    /// defect on its own trigger (`implementation/decisions-pending.md`), whose subject is a
    /// route's quoting rather than this sink. Asking the predicate directly is what keeps the
    /// leg swept over its whole axis anyway.
    ///
    /// The refusal is decided **before any filesystem access**, which is why an arbitrary root
    /// is enough here: a token git reads as a pattern is not a path this door will resolve at
    /// all.
    #[test]
    fn every_pathspec_magic_spelling_is_refused_at_the_sink() {
        let root = Path::new("/nonexistent-repo-root");
        for (spelling, names) in [
            (":(top)keepme.md", "pathspec magic"),
            (":(exclude)keepme.md", "pathspec magic"),
            (":!keepme.md", "pathspec magic"),
            (":/keepme.md", "pathspec magic"),
            (":", "pathspec magic"),
            ("*.md", "pathspec wildmatch"),
            ("keepme.?d", "pathspec wildmatch"),
            ("docs/*.md", "pathspec wildmatch"),
            ("keep[me].md", "pathspec wildmatch"),
            ("keep\\me.md", "pathspec wildmatch"),
        ] {
            let finding =
                ValidatedRetirement::adjudicate(root, Path::new(spelling), StateTruth::RolledBack)
                    .expect_err(&format!("`{spelling}` must be refused at the sink"));
            assert_eq!(finding.code, "finalize.retire-untrackable");
            assert!(
                finding.message.contains(names),
                "`{spelling}` must be refused AS {names}, not as some other reason; got: {}",
                finding.message,
            );
        }
    }

    /// …and an ordinary repo-relative spelling is **not** refused, so the leg above cannot be
    /// satisfied by a predicate that says no to everything. Driven against **this** repository,
    /// because every step after the `:` test asks the filesystem and git, and a bare temp
    /// directory answers neither question the way a repository does.
    #[test]
    fn an_ordinary_in_repo_spelling_survives_the_sink() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("crates/cli sits two levels under the repo root");
        let validated = ValidatedRetirement::adjudicate(
            root,
            Path::new("docs/direction.md"),
            StateTruth::RolledBack,
        )
        .expect("an ordinary repo-relative source is admissible");
        assert_eq!(validated.target(), root.join("docs/direction.md"));
    }

    /// The **shell-safety fence on the re-run seam** (M47 Inc 3 T7 fix). The frame's re-run
    /// line is bytes an operator or agent pastes into a shell, and the tokens it embeds are
    /// **author-owned prose** — a milestone title, a sub-task intent, a rename target — i.e.
    /// LLM-written by the determinism boundary, so `$`, a backtick, `;`, `&`, a quote and a
    /// glob are all inputs this door is *designed* to receive. The fence is the shell itself,
    /// swept over the metachar axis rather than the whitespace case the door fixtures happen
    /// to feed: for every sample `sh` must split the emitted token into **exactly one** word
    /// whose bytes are **identical** to the input. Anything else — an expansion, a command
    /// substitution, a second command — means the printed re-run silently does something
    /// other than what the frame says it does.
    #[test]
    fn shell_token_round_trips_through_a_real_shell_over_the_metachar_axis() {
        let samples = [
            "Cache rework",
            "Cache $HOME rework",
            "Cache `rework` now",
            "Cache;touch-PWNED",
            "Cache && touch PWNED",
            "Cache | tee PWNED",
            "Cache > PWNED",
            "Cache 'quoted' rework",
            "Cache \"quoted\" rework",
            "Cache \\ rework",
            "Cache *.md rework",
            "Cache ${HOME} $(id) rework",
            "Cache #1 rework!",
            "Cache ~rework",
            "Cache\nrework",
            "Cache\trework",
            "",
        ];
        for sample in samples {
            let token = shell_token(sample);
            // `set --` is the shell's own word splitter: `$#` is how many words the token
            // became, `$1` is the bytes of the first one.
            let script = format!("set -- {token}\nprintf %s \"$#\"\nprintf :\nprintf %s \"$1\"");
            let out = Command::new("sh")
                .arg("-c")
                .arg(&script)
                .output()
                .expect("run sh");
            assert!(
                out.status.success(),
                "the emitted token `{token}` for {sample:?} must parse as a shell word; \
                 stderr:\n{}",
                String::from_utf8_lossy(&out.stderr),
            );
            assert_eq!(
                String::from_utf8_lossy(&out.stdout),
                format!("1:{sample}"),
                "the emitted token `{token}` must be exactly one shell word carrying \
                 {sample:?} byte-for-byte",
            );
        }
    }

    /// The readable half of the same seam: an id, a slug, an address or a flag value that
    /// carries no shell-special byte stays **bare**, so the printed re-run reads like the
    /// command an operator would have typed (the quoting exists for prose, not for ids).
    #[test]
    fn shell_token_leaves_an_id_or_address_bare() {
        for plain in [
            "jigc",
            "milestone",
            "add-from-spec",
            "cache-rework",
            "adr:alpha-decision",
            "spec:rate-limit",
            "--to",
            "docs/decisions/alpha.md",
            "v1.0.0",
            "single-task",
        ] {
            assert_eq!(shell_token(plain), plain, "`{plain}` needs no quoting");
        }
    }

    /// `git_untracked` is the empty-commit guard's untracked signal: a brand-new
    /// file that `git diff <base>` would miss but `git add --all` would commit must
    /// register, so an untracked-only task is not falsely treated as empty.
    #[test]
    fn git_untracked_reports_new_files_and_nothing_when_clean() {
        let dir = std::env::temp_dir().join(format!(
            "jigc-untracked-{}-{}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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
        let dir = std::env::temp_dir().join(format!(
            "jigc-commit-hook-{}-{}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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
        // Birth HEAD before the seam is ever entered. Since M51 Increment 2 the seam
        // re-probes the repository posture and refuses `repo.head-unborn`, and this
        // fixture is about the captured **hook stream**, not about committing onto an
        // unborn HEAD — that cell is `posture_door_axis`'s, where `jigc setup` is the one
        // exempt door. `--no-verify` so this birth commit never runs the hooks installed
        // below and cannot contribute a stream of its own.
        run(&["commit", "-q", "--allow-empty", "--no-verify", "-m", "init"]);
        let msg = dir.join("msg.txt");
        std::fs::write(&msg, "test: a commit\n").expect("write msg");

        // Control: no hook, a real (non-empty) commit — returns empty captured output.
        std::fs::write(dir.join("a.txt"), "a\n").expect("write a");
        run(&["add", "--all"]);
        let captured = git_commit(&SeamSubject::live(&dir), &msg).expect("no-hook commit lands");
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
        let captured = git_commit(&SeamSubject::live(&dir), &msg).expect("hook commit lands");
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
        let err = git_commit(&SeamSubject::live(&dir), &msg).expect_err("a rejecting hook bails");
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
        let dir = std::env::temp_dir().join(format!(
            "jigc-pathspec-guard-{}-{}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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

    /// The pre-promote bytes of the failure-POINT axis tests below — an untracked
    /// same-path foreign whose only copy is what the capture-and-restore discipline
    /// protects.
    const AXIS_FOREIGN: &str = "# History\n\nthe user's only copy of these bytes\n";

    /// Shared harness for the failure-POINT axis tests: a temp git repo with one seed
    /// commit (the rollback shells out to git, so HEAD must exist).
    fn finalize_axis_repo(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jigc-{tag}-{}-{}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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
        std::fs::write(dir.join("seed.txt"), "seed\n").expect("write seed");
        run(&["add", "--all"]);
        run(&["commit", "-q", "-m", "seed"]);
        dir
    }

    /// Drive [`try_execute_finalize_plan`] over a crafted plan and assert the
    /// transaction was rejected (the inner `Err` — the shared rollback arm ran).
    fn execute_plan_expect_rejection(repo: &Path, plan: &engine::finalize::FinalizePlan) {
        let msg_tmp = repo.join("msg-tmp");
        std::fs::create_dir_all(&msg_tmp).expect("mk msg tmp");
        let schemas: BTreeMap<String, Schema> = BTreeMap::new();
        let result = try_execute_finalize_plan(
            repo,
            &repo.join(".jigc"),
            &msg_tmp,
            plan,
            &repo.join("cleanup-unused"),
            &schemas,
            None,
            StagePolicy::MigrationFixed,
            &mut None,
            &mut Vec::new(),
            // The rejection arm never reaches phase 7, so no area is removed and no
            // displacement subject exists to hand it.
            None,
        )
        .expect("no setup I/O error");
        assert!(
            result.is_err(),
            "the seeded mid-transaction failure must reject the finalize"
        );
    }

    fn axis_plan(
        promotions: Vec<Promotion>,
        retirements: Vec<PathBuf>,
    ) -> engine::finalize::FinalizePlan {
        engine::finalize::FinalizePlan {
            schema_version: engine::result::SCHEMA_VERSION,
            message: "test: seeded mid-transaction failure".into(),
            promotions,
            hash_updates: BTreeMap::new(),
            retirements,
            owner_artifacts: Vec::new(),
        }
    }

    /// Failure-POINT axis, the **mid-promote** member (confidence-audit code-review
    /// MEDIUM — the residual of the class minor item 10 fixed): [`promote`] fails at
    /// promotion *k* (here: the destination is a directory, so `fs::copy` errors) after
    /// promotion 1 already displaced a same-path **untracked** foreign. Pre-fix the
    /// displaced capture was returned **only on `Ok`**, so the `?` discarded it; the
    /// shared `Err` arm's rollback saw an untracked destination with no capture and
    /// **deleted the user's only copy**. The capture must survive the mid-promote
    /// failure and the rollback must restore the foreign byte-intact.
    #[test]
    fn mid_promote_failure_restores_the_earlier_displaced_untracked_foreign() {
        let repo = finalize_axis_repo("mid-promote");
        // Promotion 1's destination: an untracked same-path foreign — its bytes exist
        // nowhere else (not at HEAD, and a same-path migration plans no retirement).
        std::fs::write(repo.join("CHANGELOG.md"), AXIS_FOREIGN).expect("write foreign");
        // The staged sources promote copies from.
        std::fs::create_dir_all(repo.join("staged")).expect("mk staged");
        std::fs::write(
            repo.join("staged").join("one.md"),
            "# Changelog\n\npromoted\n",
        )
        .expect("write source 1");
        std::fs::write(repo.join("staged").join("two.md"), "# Blocked\n").expect("write source 2");
        // Promotion 2's destination is a DIRECTORY — `fs::copy` into it fails
        // deterministically (portable: no permission bits, no disk-full simulation).
        std::fs::create_dir_all(repo.join("blocked.md")).expect("mk blocking dir");
        let plan = axis_plan(
            vec![
                Promotion {
                    source: repo.join("staged").join("one.md"),
                    destination: "CHANGELOG.md".into(),
                },
                Promotion {
                    source: repo.join("staged").join("two.md"),
                    destination: "blocked.md".into(),
                },
            ],
            Vec::new(),
        );

        execute_plan_expect_rejection(&repo, &plan);

        let restored = repo.join("CHANGELOG.md");
        assert!(
            restored.exists(),
            "the displaced untracked foreign must survive a MID-PROMOTE failure — the \
             capture taken at promotion 1 must reach the rollback even though promotion 2 \
             errored"
        );
        assert_eq!(
            std::fs::read_to_string(&restored).expect("read restored foreign"),
            AXIS_FOREIGN,
            "the displaced untracked foreign must be restored byte-intact"
        );
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// Failure-POINT axis, the **mid-retire** member (the reviewer-noted same-class
    /// shape in [`retire`]): retirement *k* fails (here: the path is a directory, so
    /// `fs::read` errors with a non-`NotFound` kind) after retirement 1 already deleted
    /// an **untracked** foreign — captured pre-deletion, but pre-fix returned only on
    /// `Ok`, so the `?` discarded the capture and the rollback's worktree axis had no
    /// bytes to rewrite: the foreign was lost permanently.
    #[test]
    fn mid_retire_failure_restores_the_earlier_retired_untracked_foreign() {
        let repo = finalize_axis_repo("mid-retire");
        // Retirement 1: an untracked foreign — no committed bytes for `git restore`.
        std::fs::write(repo.join("HISTORY.md"), AXIS_FOREIGN).expect("write foreign");
        // Retirement 2 is a DIRECTORY — `fs::read` errors with `IsADirectory`
        // (non-`NotFound`, so retire raises rather than skipping).
        std::fs::create_dir_all(repo.join("blocked-retire")).expect("mk blocking dir");
        let plan = axis_plan(
            Vec::new(),
            vec![PathBuf::from("HISTORY.md"), PathBuf::from("blocked-retire")],
        );

        execute_plan_expect_rejection(&repo, &plan);

        let restored = repo.join("HISTORY.md");
        assert!(
            restored.exists(),
            "the retired untracked foreign must survive a MID-RETIRE failure — the \
             capture taken at retirement 1 must reach the rollback even though \
             retirement 2 errored"
        );
        assert_eq!(
            std::fs::read_to_string(&restored).expect("read restored foreign"),
            AXIS_FOREIGN,
            "the retired untracked foreign must be restored byte-intact"
        );
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// The reviewer's LOW-2, pinned: when captured displaced bytes exist for a
    /// destination **tracked at HEAD**, the rollback's worktree axis restores the
    /// captured pre-promote bytes — not `git restore --source=HEAD`, which would
    /// destroy an uncommitted worktree modification the user held at the destination
    /// before the promotion overwrote it.
    #[test]
    fn rollback_restores_a_tracked_destinations_uncommitted_pre_promote_bytes() {
        let repo = finalize_axis_repo("tracked-displaced");
        let run = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(&repo)
                .output()
                .expect("git runs");
            assert!(out.status.success(), "git {args:?} should succeed");
        };
        // CHANGELOG.md is tracked at HEAD as "committed", then modified (uncommitted)
        // to "modified" — the pre-promote worktree bytes the capture protects.
        std::fs::write(repo.join("CHANGELOG.md"), "committed\n").expect("write committed");
        run(&["add", "CHANGELOG.md"]);
        run(&["commit", "-q", "-m", "track changelog"]);
        std::fs::write(repo.join("CHANGELOG.md"), "modified\n").expect("write modified");
        std::fs::create_dir_all(repo.join("staged")).expect("mk staged");
        std::fs::write(repo.join("staged").join("one.md"), "promoted\n").expect("write source 1");
        std::fs::write(repo.join("staged").join("two.md"), "blocked\n").expect("write source 2");
        std::fs::create_dir_all(repo.join("blocked.md")).expect("mk blocking dir");
        let plan = axis_plan(
            vec![
                Promotion {
                    source: repo.join("staged").join("one.md"),
                    destination: "CHANGELOG.md".into(),
                },
                Promotion {
                    source: repo.join("staged").join("two.md"),
                    destination: "blocked.md".into(),
                },
            ],
            Vec::new(),
        );

        execute_plan_expect_rejection(&repo, &plan);

        assert_eq!(
            std::fs::read_to_string(repo.join("CHANGELOG.md")).expect("read restored"),
            "modified\n",
            "the rollback must restore the captured pre-promote worktree bytes — \
             `--source=HEAD` would destroy the user's uncommitted modification"
        );
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// A workbench for the staged-prose probe: `<root>/.jigc/tasks/<id>/docs/` per entry,
    /// each carrying the named `<type>:<slug>.md` staged docs plus the `provenance.json`
    /// every real working area holds (so an emptiness probe over the *directory* would
    /// answer "prose here" for every one of them).
    fn staged_workbench(tag: &str, tasks: &[(&str, &[&str])]) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "jigc-staged-prose-{tag}-{}-{}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        let _ = std::fs::remove_dir_all(&root);
        for (task, docs) in tasks {
            let dir = root.join(".jigc").join("tasks").join(task).join("docs");
            std::fs::create_dir_all(&dir).expect("mk task docs dir");
            std::fs::write(dir.join("provenance.json"), "{}\n").expect("write provenance");
            for doc in *docs {
                std::fs::write(dir.join(format!("{doc}.md")), "# staged\n").expect("write doc");
            }
        }
        root
    }

    /// The **scope** half of the shared probe (M50 → the Settle, D1): a door that removes
    /// one task's area must be able to ask about *that* area's staged prose and nothing
    /// else. Until the probe carried `only`, the single-task question could only be asked
    /// after the fact, by the narration filtering a whole-workbench enumeration — which is
    /// no use to a guard that has to refuse *before* the removal.
    ///
    /// So: over a two-task workbench, the scoped call returns exactly the named task's
    /// sorted `<type>:<slug>` identities and nothing of the sibling's, while the unscoped
    /// call (the `uninstall` door, which takes the whole workbench) still returns both.
    /// A door that named the sibling's prose would be claiming a destruction it does not
    /// perform — the same law-1 lie as performing one it never named.
    #[test]
    fn the_staged_prose_probe_answers_for_one_task_without_naming_its_sibling() {
        let root = staged_workbench(
            "scope",
            &[
                ("t-alpha", &["commit:t-alpha", "adr:cache-rework"][..]),
                ("t-beta", &["commit:t-beta"][..]),
            ],
        );

        let scoped = staged_task_prose(
            &root,
            Some(&["t-alpha".to_string()]),
            &unverified_prose_finding,
        )
        .expect("scoped");
        assert_eq!(
            scoped,
            vec![(
                "t-alpha".to_string(),
                vec!["adr:cache-rework".to_string(), "commit:t-alpha".to_string()],
            )],
            "the scoped probe answers for the named task alone, sorted, and says nothing \
             about `t-beta`",
        );

        let whole = staged_task_prose(&root, None, &unverified_prose_finding).expect("unscoped");
        assert_eq!(
            whole
                .iter()
                .map(|(task, _)| task.as_str())
                .collect::<Vec<_>>(),
            vec!["t-alpha", "t-beta"],
            "the whole-workbench door still sees both tasks",
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    /// The **fail-closed** half, unpinned until now: a present-but-unreadable `docs/` dir
    /// inside the scope must be an `Err`, never an empty set. "Enumerated nothing" and
    /// "there is nothing" are the same bytes to a caller about to `remove_dir_all` the
    /// area, and only one of them is safe — so the probe refuses and the door renders the
    /// refusal ([`unverified_prose_finding`]).
    ///
    /// Out of scope, the same unreadable dir is *not* the caller's business: a door that
    /// will not touch that area must not be blocked by it either.
    #[test]
    fn an_unreadable_docs_dir_refuses_in_scope_and_is_none_of_the_doors_business_outside_it() {
        use std::os::unix::fs::PermissionsExt;
        let root = staged_workbench(
            "unreadable",
            &[
                ("t-alpha", &["commit:t-alpha"][..]),
                ("t-beta", &["commit:t-beta"][..]),
            ],
        );
        let sealed = root.join(".jigc/tasks/t-beta/docs");
        std::fs::set_permissions(&sealed, std::fs::Permissions::from_mode(0o000))
            .expect("seal t-beta's docs dir");

        let whole = staged_task_prose(&root, None, &unverified_prose_finding);
        // The *task* door's own fail-closed identity over the same unreadable dir — the
        // refusal names the door that asked, never its sibling's (M50 Inc 3 / T2).
        let at_task_door = staged_task_prose(&root, Some(&["t-beta".to_string()]), &|err| {
            discard_unverified_prose_finding("t-beta", err)
        });
        let scoped_in = staged_task_prose(
            &root,
            Some(&["t-beta".to_string()]),
            &unverified_prose_finding,
        );
        let scoped_out = staged_task_prose(
            &root,
            Some(&["t-alpha".to_string()]),
            &unverified_prose_finding,
        );

        // Restore before asserting, so a failure does not leave an unremovable temp tree.
        std::fs::set_permissions(&sealed, std::fs::Permissions::from_mode(0o755))
            .expect("unseal t-beta's docs dir");

        let unverified = whole.expect_err("the whole-workbench probe fails closed");
        assert_eq!(
            unverified.code, "uninstall.staged-prose",
            "the fail-closed arm carries the door's own code, not a bare IO error",
        );
        scoped_in.expect_err("the in-scope probe fails closed on the same dir");
        let at_task_door = at_task_door.expect_err("the task door fails closed too");
        assert_eq!(
            at_task_door.code, DISCARD_STAGED_PROSE,
            "and it fails closed under the DOOR's code — an operator who ran `jigc task \
             discard` is never sent to re-run `jigc uninstall`",
        );
        assert!(
            at_task_door
                .route
                .as_deref()
                .is_some_and(|route| route.contains("jigc task discard t-beta --force")),
            "the task door's fail-closed route names its own consent; got {:?}",
            at_task_door.route,
        );
        assert_eq!(
            scoped_out.expect("an out-of-scope area is never probed"),
            vec![("t-alpha".to_string(), vec!["commit:t-alpha".to_string()])],
            "a door that will not touch `t-beta` is not blocked by `t-beta`",
        );

        let _ = std::fs::remove_dir_all(&root);
    }
}
