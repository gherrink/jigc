//! The `jigc` clap command tree + format selection.
//!
//! Argument parsing only: the command tree (`start`, …), the global
//! `--format=agent|json|human` selector, and dispatch to a handler stub. The
//! renderers themselves live in `render`; format *selection* is here, the
//! mapping `Format → renderer` is presentation. See
//! `implementation/module-layout.md` → Renderers (format selection) and
//! `design/write-commands.md` → Task origination (bare `jigc start`).

use crate::config::ConfigCommand;
use crate::describe;
use crate::doc::DocCommand;
use crate::ingest;
use crate::invocation_log::Outcome;
use crate::migrate;
use crate::migrate_corpus;
use crate::milestone::MilestoneCommand;
use crate::orient;
use crate::relocate;
use crate::rename;
use crate::render;
use crate::repo::PostureMember;
use crate::setup;
use crate::start;
use crate::task::TaskCommand;
use crate::unmanage;
use crate::upgrade;
use anyhow::{Context, Result};
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

/// The **argument convention**, stated once and rendered into both `create` verbs'
/// long help (`crate::doc::create_long_about`, `crate::milestone::create_long_about`).
///
/// The verb surface has taken its id-source in two spellings since M7 — `jigc doc
/// create --title "…"` beside `jigc milestone create "…"` — and the rule behind the
/// split was recorded in `design/write-commands.md` → The argument convention at M42
/// but stated on **neither** help, so an agent that had met one verb read the other as
/// an inconsistency and guessed (the rc.9 trial's N20). One `const` feeds both sites:
/// the two surfaces cannot drift into describing two different rules.
pub const ARGUMENT_CONVENTION: &str = "The argument convention: a doc verb takes its \
     id-source as the `--title` flag, a work-unit verb takes its id-source as a \
     positional (`design/write-commands.md` → The argument convention).";

/// The `migrate-corpus` long help — **generated from the kind set it classifies**, not
/// a second home for it (M51 Increment 9, EC-11).
///
/// The help named *"the deterministic v0→v1 transform"*: one transform, at one version
/// step. The verb has classified a closed set of kinds since M50
/// ([`engine::schema_diff::SchemaChangeKind::ALL`], eighteen members), and a refusal
/// prints one of those wire names verbatim (`crate::migrate_corpus` → the `Unsupported`
/// fold), so the kind an agent meets on the refusal surface was a kind the door's own
/// help had never admitted existed.
///
/// Rendering [`SchemaChangeKind::ALL`](engine::schema_diff::SchemaChangeKind::ALL)
/// through the same `as_str` home the refusal prints from is what keeps that from
/// recurring: a nineteenth kind reaches this help by existing. The prose around the
/// list states the three *dispositions* as a rule rather than enumerating which members
/// take which — a hand-listed subset would be exactly the second home this replaces.
fn migrate_corpus_long_about() -> String {
    let kinds = engine::schema_diff::SchemaChangeKind::ALL
        .iter()
        .map(|kind| kind.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "Migrate the committed managed corpus onto the current schema.\n\n\
         `jigc migrate-corpus` re-parses every committed doc of a frozen persisted \
         doctype, folds the deterministic diff between the schema version that doc is \
         stamped at and the current one into its bytes byte-stable, and writes each doc \
         back only on a clean conformance gate (the stamp flips last). Detect-with `jigc \
         validate`; this migrates. Disjoint from `jigc migrate` (foreign adoption) and \
         `jigc upgrade` (config). The verb lands its own migration in a pathspec-limited \
         commit; `--no-commit` leaves the writes unstaged, `--dry-run` writes nothing at \
         all.\n\n\
         The classification is closed and a refusal names the member it stopped at, so \
         here is the whole set — every change classifies as exactly one of these kinds: \
         {kinds}. A kind the transform driver folds byte-stable is applied. One that \
         needs authored prose mints its slot empty and is routed to the agent to author, \
         then re-run. One the driver refuses by design — applying it would destroy \
         committed bytes — blocks naming that kind and the locus it refused at, never a \
         silent no-op; `unclassified` is the backstop for a shape that moved with no kind \
         to apply, and refuses rather than restamping a doc at a version it does not \
         conform to."
    )
}

/// The `validate` long help — **generated from the family set the sweep runs**, not a
/// second home for it (M51 Increment 9, EC-11).
///
/// The help named one family and called it the verb: *"Re-check every committed doc's
/// code anchors against the codebase"*. That is the doc↔code family — one of the seven
/// [`engine::validate::STORE_FAMILIES`] the sweep drives — so the door under-stated its
/// own report by six families, and an agent reading it before running the sweep learned
/// nothing about the checks that decide most of what it will see (a dangling cross-doc
/// `ref`, an unmigrated stamp, a workflow whose step file moved).
///
/// Rendering the registry is what keeps that from recurring: an eighth family reaches
/// this help by joining the set. The sweep's exit rule is deliberately not restated here
/// — it is a function of which codes fire, and that set has its own home
/// ([`crate::render::STORE_EXIT_FLIPS`]).
fn validate_long_about() -> String {
    let families = engine::validate::STORE_FAMILIES
        .iter()
        .map(|family| format!("{} — {}", family.name, family.checks))
        .collect::<Vec<_>>()
        .join("; ");
    format!(
        "Re-check the committed store and report drift — the store-wide, read-only \
         sweep.\n\n\
         `jigc validate` is **task-less**: it re-reads the committed store and reports \
         what it finds, where `jigc task validate <id>` previews one task's gate. \
         Detect-and-report — it repairs nothing.\n\n\
         It sweeps every content family, in sweep order: {families}."
    )
}

/// The `jigc` CLI — a context compiler for coding agents.
#[derive(Debug, Parser, PartialEq, Eq)]
#[command(name = "jigc", version, about)]
pub struct Cli {
    /// Output format: `agent` (default, terse text for a coding agent) · `json`
    /// (machine-readable) · `human` (formatted for a terminal).
    #[arg(long, value_enum, default_value_t = Format::Agent, global = true)]
    pub format: Format,

    #[command(subcommand)]
    pub command: Command,
}

/// Output format selector. `agent` (default, non-TTY agent-text) · `json`
/// (generic, tooling-consumed) · `human` (TTY-pretty).
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Agent,
    Json,
    Human,
}

/// The `jigc` command tree.
#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Command {
    /// Start or resume work. Bare `jigc start` orients (read-only); an `<intent>`
    /// composes the cascade's `default-workflow` — shipped as the `router`, a
    /// `creates-task: false` selection pass that routes the intent to a
    /// work-workflow and mints **nothing**; `jigc start --workflow <X> "<intent>"`
    /// composes `<X>` and mints the task iff `<X>` declares `creates-task: true`;
    /// `--task <id>` resumes an existing task and re-composes it.
    Start {
        /// Optional task intent. Absent → orient (read-only); present → compose the
        /// cascade's `default-workflow` with `{{task.intent}}` = `<intent>`, minting
        /// a task iff that workflow declares `creates-task: true` (the shipped
        /// default, the `router`, does not — it routes to the workflow that does).
        intent: Option<String>,

        /// Compose a named workflow explicitly, bypassing the cascade default.
        /// Mints a task iff the workflow declares `creates-task: true`. Combines
        /// with the `<intent>` positional; mutually exclusive with `--task`.
        #[arg(long, conflicts_with = "task")]
        workflow: Option<String>,

        /// Resume an existing task by id: re-compose its default workflow over
        /// the task's persisted state (its base + bound context roles), so the
        /// agent picks up context bound since the task was minted (e.g. an ADR
        /// created in-task, now reachable as `task.<role>`). Mutually exclusive
        /// with `<intent>`.
        #[arg(long, conflicts_with = "intent")]
        task: Option<String>,

        /// Show the resolution tree instead of composing — the cascade
        /// provenance, the resolved include list with each step's winning layer,
        /// and the `replaces … at position` annotations — without minting a task.
        /// Combines with the `<intent>` positional and `--workflow <X>`; mutually
        /// exclusive with `--task`.
        #[arg(long, conflicts_with = "task")]
        explain: bool,

        /// Override the minted task id (only meaningful when a task is minted).
        /// Taken **verbatim** and validated as a well-formed slug — a malformed value
        /// is rejected, never silently re-slugified. Mutually exclusive with `--task`
        /// (a resume mints nothing); inert on a non-minting compose (`design/write-
        /// commands.md` → `jigc rename`'s `--slug` precedent).
        #[arg(long, conflicts_with = "task")]
        slug: Option<String>,
    },

    /// Re-enter a milestone sub-task, or preview a work-minting workflow — compose the
    /// named `<W>`. `--task <id>` re-enters the sub-task `<id>` as a fanned sub-agent
    /// (`<W>` must equal the sub-task's recorded mint workflow, else rejected —
    /// distinct from `jigc start --task`, which recomposes a top-level task's own
    /// workflow). `--preview` composes a `creates-task: true` `<W>` **without minting a
    /// task**, so an agent can read what it will ask before consenting to mint. Exactly
    /// one of the two is required.
    #[command(group(ArgGroup::new("workflow_mode").required(true).args(["task", "preview"])))]
    Workflow {
        /// The workflow to compose — with `--task`, the fan-out step's `run:`
        /// workflow (must equal the sub-task's recorded mint workflow); with
        /// `--preview`, any `creates-task: true` workflow.
        workflow: String,

        /// Re-enter the named milestone sub-task. Mutually exclusive with `--preview`.
        #[arg(long)]
        task: Option<String>,

        /// Preview the workflow's composed step text **without minting a task** — the
        /// read surface for a mutation-cautious agent. The workflow must declare
        /// `creates-task: true` (a `creates-task: false` workflow mints nothing, so it
        /// has nothing to preview — run it directly). Mutually exclusive with `--task`.
        #[arg(long)]
        preview: bool,
    },

    /// Read and write managed docs — the `jigc doc <verb>` surface.
    ///
    /// The write verbs (`create`/`add-item`/`set-field`/`set-slot`/`author`/…)
    /// work the active task's working area; the read verbs
    /// (`show`/`schema`/`list`) serve the committed store.
    Doc {
        #[command(subcommand)]
        verb: DocCommand,
    },

    /// The task lifecycle surface — `jigc task <verb> <id>` over a named task's
    /// working area.
    Task {
        #[command(subcommand)]
        verb: TaskCommand,
    },

    /// The cascade surface — `jigc config <verb>` reads what the knobs currently
    /// resolve to (`get` / `list`) and records deltas into the project layer
    /// (`.jigc/config/`).
    Config {
        #[command(subcommand)]
        verb: ConfigCommand,
    },

    /// The milestone work-unit surface — `jigc milestone <verb>` mints a
    /// milestone, builds its sub-task list, and executes, joins, and finalizes it —
    /// or, when the work is abandoned, discards it.
    Milestone {
        #[command(subcommand)]
        verb: MilestoneCommand,
    },

    /// Install the Claude Code adapter — writes the `.jigc/AGENT.md` bootstrap and
    /// a reference into `CLAUDE.md`, initializes the project layer (`.jigc/config/`),
    /// allowlists `Bash(jigc:*)`, and installs the `SessionStart` and warn-only git
    /// `pre-commit` hooks. Idempotent. Refuses its own install commit when a path in
    /// that footprint carries work no commit has a copy of, rather than sweeping it in.
    Setup {
        /// Commit the install footprint even where it carries bytes `jigc setup` did
        /// not write — the explicit consent to sweep uncommitted work into
        /// `chore(jigc): install jigc workspace config`. Inert when the footprint is
        /// clean, which is every ordinary install.
        #[arg(long)]
        force: bool,
    },

    /// Reverse this project's jigc install — removes `.jigc/`, unwires the
    /// `CLAUDE.md` reference, and drops the `Bash(jigc:*)` permit from
    /// `.claude/settings.json`. Leaves the machine-global `doc-code` probe (shared
    /// across repos) in place. Idempotent: a second run is a clean no-op, and the
    /// host files it edits (`CLAUDE.md`, `.claude/settings.json`, a wrapped
    /// `pre-commit` hook) keep your own content byte-for-byte. Four states it
    /// refuses instead of destroying, because `.jigc/` is their only copy — a
    /// fan-out sub-task path under `.jigc/worktrees/` that holds content blocks
    /// with `uninstall.dirty-worktree` (get the work out — or, for a path this
    /// repository has registered as a worktree, abandon the milestone with
    /// `jigc milestone discard <milestone-id> --force`), an open task
    /// under `.jigc/tasks/` holding a staged doc no commit has a copy of blocks
    /// with `uninstall.staged-prose` (throw the task away with `jigc task discard
    /// <task-id> --force`, or land it with `jigc task finalize <task-id>` once its doc
    /// is complete — a milestone sub-task lands only through its milestone's own
    /// boundary, and the refusal's listing names each sub-task's two exits), and any
    /// other file under `.jigc/` that no index has a copy of —
    /// a recorded config delta among them — blocks with
    /// `uninstall.untracked-workbench-file` (`git add <path>` is enough to make it
    /// recoverable), and a file jigc did not write inside a working area under
    /// `.jigc/tasks/` or `.jigc/milestones/`, or anything parked under
    /// `.jigc/displaced/`, blocks with `uninstall.foreign-bytes` (move it out, or
    /// `rm -r` what you do not need — jigc has no verb that clears the parking home).
    /// Any of them removes nothing until you re-run — or pass `--force`, which deletes
    /// all four with the install.
    Uninstall {
        /// Remove `.jigc/` even when it holds a fan-out worktree with content, an
        /// open task's staged docs, or a workbench file no index has a copy of —
        /// the explicit consent to destroy work no commit has a copy of. Inert when
        /// all three guards are already clean.
        #[arg(long)]
        force: bool,
    },

    /// Re-check every recorded config delta against the current pack and report
    /// what needs attention. Report-and-route only — it changes nothing, and exits
    /// non-zero if any finding blocks.
    Upgrade,

    /// Scan the project for existing markdown docs, adopt every conformant one
    /// (register-only — never moving or rewriting a file), and print a triage
    /// report. Misplaced or non-conformant files are flagged for a human; exits 0.
    /// A flagged non-conformant file is rewritten into managed shape with `jigc
    /// migrate <path> --as <doctype>`.
    Ingest,

    /// Rewrite a foreign, non-conformant document into managed shape — `jigc
    /// migrate <path> --as <doctype>`. Mints a migration task and composes the
    /// `migrate-<doctype>` workflow so the agent re-authors the content through the
    /// write verbs. Works for any doctype with a shipped `migrate-<doctype>` workflow
    /// — see the error message for the live set. An already-conformant file needs no
    /// rewrite: adopt it with `jigc ingest` instead.
    Migrate {
        /// The repo-relative path of the foreign document to migrate (e.g.
        /// `CHANGELOG.md`).
        path: String,

        /// The target managed doctype the foreign document is rewritten into — any
        /// doctype with a shipped `migrate-<doctype>` workflow — see the error
        /// message for the live set.
        #[arg(long = "as")]
        r#as: String,

        /// Override the migrated doc's slug (`<doctype>:<slug>`), decoupling the id
        /// from the authored title. Taken **verbatim** and validated as a well-formed
        /// slug before any task is minted — a malformed value is rejected, never
        /// silently re-slugified (`jigc doc create --slug`'s discipline, recorded in
        /// the migration task and applied when the agent authors the target doc).
        #[arg(long)]
        slug: Option<String>,
    },

    /// Migrate the committed managed corpus onto the current schema.
    #[command(long_about = migrate_corpus_long_about())]
    MigrateCorpus {
        /// Migrate and write, but stage and commit **nothing** — leave the migration in the
        /// working tree, to review it or fold it into a larger commit yourself.
        #[arg(long = "no-commit")]
        no_commit: bool,

        /// Print the triage report the migration *would* produce and change nothing: no
        /// migrated bytes, no relocation move, no commit. Implies `--no-commit`.
        #[arg(long = "dry-run")]
        dry_run: bool,
    },

    /// Drop a single managed doc from jigc's index and state — `jigc unmanage
    /// <path>` removes its edges and file-state baseline, leaving the file bytes on
    /// disk (the inverse of `ingest`'s adopt). Idempotent: a re-run is a clean no-op.
    Unmanage {
        /// The repo-relative path of the managed doc to un-manage (e.g.
        /// `decisions/single-node-cache.md`).
        path: String,
    },

    /// Rename a managed doc — `jigc rename <type>:<slug> --to "<New Title>"` is the
    /// CLI-owned identity refactor: it derives the new slug from the title, repoints
    /// every persisted referrer old→new, rewrites the moved doc's H1, `git mv`s it, and
    /// commits as one atomic transaction (rolling back cleanly on any failure). `--to` is
    /// required; `--slug` (only valid alongside `--to`) overrides the derived slug.
    Rename {
        /// The `<type>:<slug>` address of the doc to rename (e.g.
        /// `adr:single-node-cache`).
        #[arg(value_name = "type:slug")]
        old_slug: String,

        /// The new title — the moved doc's H1, and the slug source unless `--slug`
        /// overrides. Required.
        #[arg(long)]
        to: String,

        /// Override the derived slug (only valid alongside `--to`). Taken **verbatim**
        /// and validated as a well-formed slug — a malformed value is rejected before
        /// the destination path is built, never silently re-slugified (the discipline
        /// `jigc start` / `jigc migrate` / `jigc doc create` already state).
        #[arg(long, requires = "to")]
        slug: Option<String>,
    },

    /// Relocate a **freeze-exempt** doctype's pre-existing committed instance(s) from a
    /// human-supplied prior home to its current schema home — `jigc relocate <type> --from
    /// <prior-home>`. The sibling of the version-gated `jigc migrate-corpus` relocation for
    /// frozen doctypes: a freeze-exempt doctype carries no prior-home snapshot, so the prior
    /// home is supplied by hand and each stranded instance is `git mv`d byte-faithful to the
    /// current home (never silently stranded). A frozen doctype is refused (use
    /// `migrate-corpus`).
    Relocate {
        /// The freeze-exempt doctype id whose schema home moved (e.g. `vision`).
        r#type: String,

        /// The prior home the committed instance(s) sit at — a directory (`docs/vision/`) or
        /// a literal file (`docs/vision.md`). Required (no snapshot records it).
        #[arg(long)]
        from: String,
    },

    /// Print a prose tour of what's available — every workflow and doc-type with
    /// their descriptions, plus command-ref hints — reflecting the resolved
    /// cascade.
    ///
    /// The prose is a human menu, not a stable API: read it, don't build on its
    /// wording.
    ///
    /// `--workflows` / `--doctypes` / `--commands` select which kinds of entry the
    /// menu returns; they combine, and passing none returns the whole tour. The
    /// single-item form (`jigc describe <id>`) is not built — describe is the menu;
    /// `jigc start --explain` is the resolution trace.
    ///
    /// The global `--format json` arm emits the same tour as a keyed object.
    /// It parses — it is simply as unpinned as the prose, and may change with
    /// any pack edit. For a structural read you can depend on, reach for
    /// `jigc doc schema` — the separately versioned contract.
    Describe {
        /// Return the workflow entries.
        #[arg(long)]
        workflows: bool,

        /// Return the doc-type entries.
        #[arg(long)]
        doctypes: bool,

        /// Return the command-ref entries.
        #[arg(long)]
        commands: bool,
    },

    /// Re-check the committed store and report drift — the store-wide, read-only sweep.
    #[command(long_about = validate_long_about())]
    Validate,
}

impl Command {
    /// **The leaf verb this parsed command is** — the key [`BEHALF_DOORS`] is read at, and
    /// the subject the repository-posture guard adjudicates (M51 Increment 2 / T3).
    ///
    /// **Read off the parsed command, never off the process argv.** The identity has to
    /// survive every spelling clap accepts (an abbreviation, a flag before the verb, a
    /// value that happens to look like a subcommand), and a string scan of `argv` answers
    /// about the *bytes typed* rather than about the *door reached*. The match is
    /// exhaustive on purpose — the M50 `milestone_id()` precedent: a verb added anywhere in
    /// the tree **cannot compile** until someone says which leaf it is, and
    /// [`BEHALF_DOORS`]' own ⇔ fence then makes them say what it acts on.
    pub fn leaf(&self) -> &'static [&'static str] {
        match self {
            Command::Start { .. } => &["start"],
            Command::Workflow { .. } => &["workflow"],
            Command::Setup { .. } => &["setup"],
            Command::Uninstall { .. } => &["uninstall"],
            Command::Upgrade => &["upgrade"],
            Command::Ingest => &["ingest"],
            Command::Migrate { .. } => &["migrate"],
            Command::MigrateCorpus { .. } => &["migrate-corpus"],
            Command::Unmanage { .. } => &["unmanage"],
            Command::Rename { .. } => &["rename"],
            Command::Relocate { .. } => &["relocate"],
            Command::Describe { .. } => &["describe"],
            Command::Validate => &["validate"],
            Command::Doc { verb } => match verb {
                DocCommand::Create { .. } => &["doc", "create"],
                DocCommand::AddItem { .. } => &["doc", "add-item"],
                DocCommand::RemoveItem { .. } => &["doc", "remove-item"],
                DocCommand::RetitleItem { .. } => &["doc", "retitle-item"],
                DocCommand::Rename { .. } => &["doc", "rename"],
                DocCommand::SetField { .. } => &["doc", "set-field"],
                DocCommand::SetSlot { .. } => &["doc", "set-slot"],
                DocCommand::Author { .. } => &["doc", "author"],
                DocCommand::Show { .. } => &["doc", "show"],
                DocCommand::Schema { .. } => &["doc", "schema"],
                DocCommand::List { .. } => &["doc", "list"],
            },
            Command::Task { verb } => match verb {
                TaskCommand::List => &["task", "list"],
                TaskCommand::Diff { .. } => &["task", "diff"],
                TaskCommand::Validate { .. } => &["task", "validate"],
                TaskCommand::Discard { .. } => &["task", "discard"],
                TaskCommand::Finalize { .. } => &["task", "finalize"],
                TaskCommand::Bind { .. } => &["task", "bind"],
            },
            Command::Config { verb } => match verb {
                ConfigCommand::Set { .. } => &["config", "set"],
                ConfigCommand::InsertStep { .. } => &["config", "insert-step"],
                ConfigCommand::ReplaceStep { .. } => &["config", "replace-step"],
                ConfigCommand::RemoveStep { .. } => &["config", "remove-step"],
                ConfigCommand::Fill { .. } => &["config", "fill"],
                ConfigCommand::Get { .. } => &["config", "get"],
                ConfigCommand::List => &["config", "list"],
                ConfigCommand::Fork { .. } => &["config", "fork"],
            },
            Command::Milestone { verb } => match verb {
                MilestoneCommand::Create { .. } => &["milestone", "create"],
                MilestoneCommand::AddTask { .. } => &["milestone", "add-task"],
                MilestoneCommand::AddFromSpec { .. } => &["milestone", "add-from-spec"],
                MilestoneCommand::ListTasks { .. } => &["milestone", "list-tasks"],
                MilestoneCommand::Provision { .. } => &["milestone", "provision"],
                MilestoneCommand::Execute { .. } => &["milestone", "execute"],
                MilestoneCommand::Join { .. } => &["milestone", "join"],
                MilestoneCommand::Finalize { .. } => &["milestone", "finalize"],
                MilestoneCommand::Discard { .. } => &["milestone", "discard"],
            },
        }
    }
}

/// **The repository-posture guard** — one gate, in front of every door that commits or
/// moves on the user's behalf (M51 Increment 2 / T3; `settle-record.md` → D2 · Review
/// amendments §3).
///
/// Its subject is the [`BEHALF_DOORS`] row for the **parsed** leaf ([`Command::leaf`]), so
/// the three classes need no per-door wiring and a new verb joins the guard the moment
/// someone classifies it:
///
///   * **commit-on-behalf** — the full posture family, minus the members that row states
///     as [`PostureExemption`] (`jigc setup` and *unborn*, and nothing else);
///   * **move-on-behalf** — [`PostureMember::OperationInProgress`] only. A `git mv` lands
///     in the index and which commit it joins stays the user's to decide, so a detached or
///     unborn HEAD is none of the mover's business; moving a **tracked** file out from
///     under a half-finished merge is.
///   * **neither** — nothing. A door that neither commits nor moves a committed file owes
///     the user no posture verdict, and answering one would be a refusal with no damage
///     behind it.
///
/// **It never fires outside a git repository.** The walk-up answers `None` there and this
/// returns `None`, so `locate::not_in_repo`'s single answer keeps arriving from the door
/// that was asked (`tests/not_in_repo_axis.rs`) rather than being pre-empted by a posture
/// the probe could not have read anyway.
///
/// **A working directory it cannot read is a refusal, not a stand-down** (M52 Increment 3
/// / T3; `settle-record.md` → D2.4). Through rc.15 this read the cwd as
/// `current_dir().ok()?` and answered `None` on the fault — indistinguishable, from here,
/// from *nothing to refuse* — so a door that commits or moves on the user's behalf went on
/// to act for a repository nobody could name. It now goes through [`cwd_or_refusal`], the
/// module's one reader of the process cwd, so the fault takes the same `{error}` arm at
/// the same exit code it takes at every dispatch arm, one step earlier and from the guard
/// rather than past it. The [`ActsOnBehalf::Neither`] doors return above this line and are
/// untouched: they still meet the fault at their own arm.
///
/// **The refusal carries no override.** The route names the git command that resolves the
/// state — a posture is a repository state the user can resolve, not bytes only they can
/// value — so `--force` at `jigc setup` keeps exactly one meaning, and a flag that consents
/// to something else (`--carry-staged`: *carry my staged work*) cannot carry a door past a
/// merge it never consented to conclude.
///
/// It routes through [`crate::invocation_log::operational_failure`], the one seam that
/// pairs the printed finding with the logged identity, so a posture refusal is as legible
/// in the invocation log as it is on stderr.
fn refuse_on_posture(command: &Command, format: Format) -> Option<Outcome> {
    // The ⇔ fence (`tests::every_leaf_verb_says_what_it_acts_on`) makes the miss
    // impossible; answering `None` rather than panicking keeps a table hole a missing
    // guard instead of an exit-101 on top of the user's command.
    let acts = &BEHALF_DOORS
        .iter()
        .find(|row| row.door == command.leaf())?
        .acts;
    if matches!(acts, ActsOnBehalf::Neither) {
        return None;
    }
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return Some(refusal),
    };
    posture_refusal_in(&cwd, acts, format)
}

/// The refusal a door acting as `acts` owes in `cwd`'s repository, or `None` when the
/// posture is committable, unanswerable, or outside a git repository.
///
/// Split out of [`refuse_on_posture`] so the **preview** can ask the identical question
/// ([`finalize_posture_refusal`]): one producer, so the door and the surface that
/// forecasts it cannot answer differently.
fn posture_refusal_in(cwd: &Path, acts: &ActsOnBehalf, format: Format) -> Option<Outcome> {
    let repo_root = crate::repo::discover_repo_root(cwd)?;
    let subject = crate::repo::posture_subject(&repo_root);
    let breach = crate::repo::posture(&repo_root)
        .into_iter()
        .find(|breach| {
            subject.adjudicates(breach.member())
                && match acts {
                    ActsOnBehalf::CommitsOnBehalf { exempt, .. } => {
                        !exempt.iter().any(|row| row.member == breach.member())
                    }
                    ActsOnBehalf::MovesOnBehalf { .. } => {
                        breach.member() == PostureMember::OperationInProgress
                    }
                    ActsOnBehalf::Neither => false,
                }
        })?;
    Some(crate::invocation_log::operational_failure(
        format,
        &render::finding_error(&breach.finding()),
    ))
}

/// **The refusal `jigc task finalize` would make in `cwd`'s repository** — the posture
/// `jigc task validate` previews (M52 Increment 3 / T6;
/// `completions/artifacts/M52/settle-record.md` → D2.6 as amended by §4).
///
/// The preview is a **separate invocation at the door**, not a member of
/// [`crate::task::TaskArea::preview_gates`]: the posture family is a fact about the
/// repository, not about the task's content, and folding it into the task-scope sweep
/// would report it at the sweep's severity and exit code
/// ([`crate::gate_coverage::Invocation::SeparatelyAtDoor`] carries that distinction for
/// every surface that states the coverage claim). What it must *not* be is a second
/// answer: it asks this module's one producer with the **`task finalize` row's own**
/// [`ActsOnBehalf`], so the finding, its route and its exit code are the committing
/// door's, byte for byte, and an exemption added to that row reaches the preview with no
/// edit anywhere else.
///
/// A sub-task's commit boundary is `jigc milestone finalize`, whose row is
/// commit-on-behalf with the same empty exemption set — so one lookup answers for both
/// doors rather than the preview having to know which one a task belongs to.
///
/// `None` — the miss arm — keeps a table hole a missing guard rather than an exit-101 on
/// top of the user's command, exactly as [`refuse_on_posture`] does.
pub(crate) fn finalize_posture_refusal(cwd: &Path, format: Format) -> Option<Outcome> {
    let acts = &BEHALF_DOORS
        .iter()
        .find(|row| row.door == ["task", "finalize"])?
        .acts;
    posture_refusal_in(cwd, acts, format)
}

impl Cli {
    /// Dispatch the parsed command: bare `start` (no `intent`) runs the
    /// read-only orientation end-to-end; an `<intent>` composes the cascade's
    /// default workflow, minting a task iff that workflow declares
    /// `creates-task: true` (`design/write-commands.md` → Task origination).
    pub fn dispatch(self) -> Outcome {
        // Before any door runs: no repository posture reaches a door that commits or
        // moves on the user's behalf without that door having adjudicated it.
        if let Some(refusal) = refuse_on_posture(&self.command, self.format) {
            return refusal;
        }
        match self.command {
            // `--explain` short-circuits the compose path: it renders the
            // task-independent resolution tree and mints nothing (clap forbids it
            // with `--task`, so only the orient/intent forms reach here). The
            // `<intent>` is the tree's id-slug input; absent, the default-workflow
            // tree is shown.
            Command::Start {
                intent,
                workflow,
                task: None,
                explain: true,
                slug: _,
            } => run_explain(self.format, intent.as_deref(), workflow.as_deref()),
            Command::Start {
                intent: None,
                workflow: None,
                task: None,
                explain: false,
                slug: _,
            } => run_orient(self.format),
            // `--workflow <X>` with no `<intent>`: a `creates-task: false` `<X>`
            // composes (no mint); a `creates-task: true` `<X>` is rejected (it slugs
            // its task id from the intent), instead of silently orienting.
            Command::Start {
                intent: None,
                workflow: Some(workflow),
                task: None,
                explain: false,
                slug: _,
            } => run_compose_named_no_intent(self.format, &workflow),
            Command::Start {
                intent: Some(intent),
                workflow: None,
                task: None,
                explain: false,
                slug,
            } => run_compose(self.format, &intent, slug.as_deref()),
            Command::Start {
                intent: Some(intent),
                workflow: Some(workflow),
                task: None,
                explain: false,
                slug,
            } => run_compose_named(self.format, &intent, &workflow, slug.as_deref()),
            Command::Start {
                intent: None,
                workflow: _,
                task: Some(id),
                explain: false,
                slug: _,
            } => run_resume(self.format, &id),
            // `conflicts_with` makes clap reject `<intent>` + `--task` and
            // `--explain` + `--task` together, so these arms are unreachable.
            Command::Start {
                intent: Some(_),
                workflow: _,
                task: Some(_),
                explain: _,
                slug: _,
            }
            | Command::Start {
                task: Some(_),
                explain: true,
                ..
            } => unreachable!("clap rejects `<intent>`/`--explain` together with `--task`"),
            Command::Workflow {
                workflow,
                task: Some(task),
                preview: false,
            } => run_reenter(self.format, &workflow, &task),
            Command::Workflow {
                workflow,
                task: None,
                preview: true,
            } => run_preview(self.format, &workflow),
            Command::Workflow { .. } => {
                unreachable!(
                    "the `workflow_mode` arg-group requires exactly one of --task/--preview"
                )
            }
            Command::Doc { verb } => run_doc(self.format, verb),
            Command::Task { verb } => run_task(self.format, verb),
            Command::Config { verb } => run_config(self.format, verb),
            Command::Milestone { verb } => run_milestone(self.format, verb),
            Command::Setup { force } => run_setup(self.format, force),
            Command::Uninstall { force } => run_uninstall(self.format, force),
            Command::Upgrade => run_upgrade(self.format),
            Command::Ingest => run_ingest(self.format),
            Command::Migrate { path, r#as, slug } => {
                run_migrate(self.format, &path, &r#as, slug.as_deref())
            }
            Command::MigrateCorpus { no_commit, dry_run } => {
                run_migrate_corpus(self.format, migrate_corpus::Options { no_commit, dry_run })
            }
            Command::Unmanage { path } => run_unmanage(self.format, &path),
            Command::Rename { old_slug, to, slug } => {
                run_rename(self.format, &old_slug, &to, slug.as_deref())
            }
            Command::Relocate { r#type, from } => run_relocate(self.format, &r#type, &from),
            Command::Describe {
                workflows,
                doctypes,
                commands,
            } => run_describe(
                self.format,
                describe::Kinds::from_flags(workflows, doctypes, commands),
            ),
            Command::Validate => run_validate_store(self.format),
        }
    }
}

/// The process working directory, or **the one refusal every leaf gives for a working
/// directory it cannot read** — the pre-dispatch funnel's first half (M52 Increment 1 /
/// T3; `design/surface-contract.md` → The pre-dispatch stream rule).
///
/// The fault is real and ordinary: a shell sitting in a directory another process removed,
/// a worktree pruned under an agent, a build step that deletes its own scratch dir. It
/// fires **before** jigc has located a repository, a project layer or a pack, so it
/// precedes every per-verb reject funnel — and through rc.15 each of the 23 dispatch arms
/// answered it with its own `eprintln!` of plain prose and a bare `Outcome::failure()`. A
/// `--format json` driver therefore got, at **all 47 leaves**, bytes it cannot parse and an
/// exit code with nothing behind it: neither reject arm, no `(code, target)` key, and
/// nothing in the invocation log.
///
/// Routing it through [`crate::invocation_log::operational_failure`] settles all three at
/// once — `{"error": …}` under `--format json`, the same sentence as before under the text
/// formats, and the run's identity recorded — because that seam is where this codebase
/// already pairs a printed operational error with its logged outcome. There is **no
/// finding behind this fault**, so it takes the `{error}` arm by the rule T1 of this
/// increment wrote down ([command-output-contract.md](../../../design/command-output-contract.md)
/// → *Which reject arm a run takes*): a reject that carries a finding takes the findings
/// arm, a reject with none takes `{error}`.
///
/// **Every reader of the cwd in this module comes here.** Increment 1 left one that did
/// not — [`refuse_on_posture`]'s `current_dir().ok()?`, which **silently** skipped the
/// whole M51 posture family on this same fault — and declared it rather than omitting it,
/// because closing it is a control-flow change on an `Option<Outcome>` and not an
/// expression substitution; Increment 3 / T3 closed it (D2.4, settle-record §12). The
/// arithmetic is measured, not asserted, by `crates/cli/tests/pre_dispatch_faults.rs`'s
/// count fence: at rc.15 this module held **24** readers of the process cwd, one per
/// dispatch arm; it now holds **one** — this seam — with **24** callers reaching it, the
/// 23 dispatch arms plus the posture guard. A 25th caller added tomorrow, or a second raw
/// reader, reddens the fence rather than quietly reviving the prose.
fn cwd_or_refusal(format: Format) -> Result<std::path::PathBuf, Outcome> {
    std::env::current_dir().map_err(|err| {
        crate::invocation_log::operational_failure(
            format,
            &anyhow::anyhow!("cannot determine the current directory: {err}"),
        )
    })
}

/// Run `jigc describe` against the current working directory: locate the repo +
/// project layer, build the **pack-only** resolved definitions (the unfiltered
/// workflow set + the full doctype set + the command catalog), assemble the
/// whole-menu projection, keep the `kinds` the caller asked for, render it through
/// the selected `format`, and print it. A clean run exits 0; a locator error (no repo
/// / no project layer) routes to stderr and exits non-zero
/// (`design/introspection.md` → Command surface). Reads-only — it composes nothing
/// and writes nothing.
fn run_describe(format: Format, kinds: describe::Kinds) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match describe::run(&cwd, kinds) {
        Ok(description) => {
            println!("{}", render::describe(format, &description));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Run `jigc setup` (the adapter install) against the current working directory:
/// locate the repo root, install the Claude Code adapter (bootstrap reference +
/// project-layer init + `Bash(jigc:*)` allowlist), render the outcome through the
/// selected `format`, and map
/// it to the exit code. Success prints a summary on stdout and exits 0; a write
/// failure prints a blocking `setup.*` finding (with its route) on stderr and
/// exits non-zero (`design/assistant-adapter.md` → Generated, minimal,
/// regenerated; the block-payload envelope, `DECISIONS.md` 2026-05-31).
fn run_setup(format: Format, force: bool) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match setup::run(&cwd, force) {
        Ok((summary, ignore)) => {
            println!("{}", render::setup_success(format, &summary));
            // What the install's `.jigc/.gitignore` amend did (M51 Increment 4 / T2) —
            // beside the summary, because `setup` writes into a file the user
            // legitimately co-owns AND commits it, so an entry appended to an older
            // build's committed set must be said rather than landed in silence.
            crate::gitignore::emit_ack(format, &Some(ignore));
            Outcome::success()
        }
        // The refusal takes the **declared** reject arm (M52 Increment 1 / T2): raised as an
        // envelope-projecting carrier and rendered at the one reject funnel, so
        // `--format json` gets the `{findings, schema_version}` document that keeps the
        // `(code, target)` key — and the invocation log gets the identity this door prints.
        // The agent/human bytes are the funnel's one-finding render, which is the house
        // finding line plus the routing footer: what this door has always emitted.
        Err(finding) => crate::invocation_log::operational_failure(
            format,
            &render::envelope_finding_error(&finding),
        ),
    }
}

/// Run `jigc uninstall` (the repo-local teardown) against the current working
/// directory: locate the repo root, reverse the enumerated repo-local install
/// (remove `.jigc/`, unwire the `CLAUDE.md` reference, drop the `Bash(jigc:*)` allowlist
/// permit — never the machine-global `doc-code` probe), render the outcome through the
/// selected `format`, and map it to the exit code. Success prints a summary on stdout
/// and exits 0; a write failure — or either WIP guard, unless `force` — prints a blocking
/// `uninstall.*` finding (with its route) on stderr and exits non-zero
/// (`design/project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5), bullet (b)).
fn run_uninstall(format: Format, force: bool) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match setup::run_uninstall(&cwd, force) {
        Ok(summary) => {
            println!("{}", render::uninstall_success(format, &summary));
            Outcome::success()
        }
        // `run_setup`'s arm exactly — the same producer, the same declared reject arm, and
        // for this door the stakes the guard exists for: `uninstall.untracked-workbench-file`
        // names the unsaved bytes a teardown would destroy, and its route carries the move
        // that saves them. Under the bare-`Finding` root a driver reached neither.
        Err(finding) => crate::invocation_log::operational_failure(
            format,
            &render::envelope_finding_error(&finding),
        ),
    }
}

/// Dispatch a `jigc task <verb> <id>` lifecycle verb against the current working
/// directory. The selected `--format` flows through to the finding renderer for
/// `validate`; `diff` / `discard` produce plain output.
fn run_task(format: Format, verb: TaskCommand) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    verb.dispatch(&cwd, format)
}

/// Dispatch a `jigc config <verb>` cascade-authoring write against the current
/// working directory. A `set` records a `scalar-set`; `insert-step` / `replace-step`
/// / `remove-step` each record a `structural-op` (insert/replace also writing the
/// native step file) — each adjudicated at write time; a blocking adjudication
/// finding surfaces on stderr (with its route, through the shared
/// format-honoring operational-error funnel) and exits non-zero
/// (`design/overrides.md` → Authoring deltas).
fn run_config(format: Format, verb: ConfigCommand) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    verb.dispatch(&cwd, format)
}

/// Dispatch a `jigc milestone <verb>` action against the current working
/// directory. `create` reads HEAD and mints the milestone area; `add-task` mints a
/// sub-task pinned to the milestone's shared base and appends it. A blocking finding
/// (serial collision, unknown milestone) surfaces on stderr with its route and
/// exits non-zero (`design/write-commands.md` → Minting a milestone).
fn run_milestone(format: Format, verb: MilestoneCommand) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    verb.dispatch(&cwd, format)
}

/// Run `jigc upgrade` against the current working directory: locate the project
/// layer, run the `override-default` classifier over every recorded delta against
/// the current pack, render the findings + routes through the selected `format`,
/// and **gate the exit code on `report.has_blocking()`** — any blocking finding
/// exits non-zero. **Report-and-route only**: nothing is written. The rendered
/// report (the standard findings+routes view, or the positive no-findings line on
/// a clean cascade) prints on stdout; a locator error (no repo / no project layer)
/// routes to stderr and exits non-zero (`design/overrides.md` → The `jigc upgrade`
/// command, step 3: report through the standard renderer, blocking-by-default).
fn run_upgrade(format: Format) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    // The adapter's **owned guide artifact**, read before the classifier runs (M48
    // Increment 10 / T2): a copy the user has edited is reported here with its route and
    // replaced nowhere — this verb writes nothing. `None`/`None` when there is no artifact
    // to read, so the clean line claims no check it did not run.
    let (guide, guide_finding) = upgrade::adapter_guide_reading(&cwd);
    match upgrade::upgrade_in_repo(&cwd) {
        Ok(mut report) => {
            // Appended after the classifier's own report, the shipped un-keyed-advisory
            // idiom (`store-version.binary-mismatch` at the store sweep): it is not a
            // `CHECK_INVENTORY` row, so the severity post-pass leaves it advisory and the
            // exit stays 0.
            if let Some(finding) = guide_finding {
                report.findings.push(finding);
            }
            // The clean line names what the sweep checked (round-2 D4): the recorded
            // config deltas re-applied against the current pack, and — since M48 — the
            // adapter artifact it read. A separate count read so the report seam stays
            // report-and-route only.
            let checked = upgrade::recorded_delta_count(&cwd).unwrap_or(0);
            println!(
                "{}",
                render::validation_upgrade(format, &report, checked, guide.as_deref())
            );
            let code = if report.has_blocking() { 1 } else { 0 };
            Outcome::with_findings(code, &report.findings)
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Run `jigc ingest` against the current working directory: locate the repo root,
/// discover candidate markdown, classify each against the persisted schemas, adopt
/// every conformant `adoptable` candidate **register-only** (record it into the edge
/// index + file-state baseline — never moving or rewriting a candidate file), and
/// render the triage report through the selected `format`. A clean scan exits 0; a
/// locator error (no repo / no project layer) routes to stderr and exits non-zero
/// (`design/project-setup.md` → Flow 2; `design/worked-examples.md` → flow 12).
fn run_ingest(format: Format) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match ingest::run(&cwd) {
        Ok(report) => {
            println!("{}", render::ingest(format, &report));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Run `jigc migrate <path> --as <doctype>` against the current working directory:
/// locate the repo + project layer, read the foreign file, mint the off-router
/// migration task, stage the foreign bytes, compose the `migrate-<doctype>` workflow
/// over the source seam, render the composed view through the selected `format`, and
/// print it. A clean run prints on stdout and exits 0; an unknown doctype, a missing
/// foreign file, a serial collision, or a blocking compose finding surfaces on stderr
/// (with its route) and exits non-zero (`design/auto-migration.md` → The `jigc migrate`
/// verb / The source seam).
fn run_migrate(format: Format, path: &str, doctype: &str, slug_override: Option<&str>) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    migrate::run(&cwd, path, doctype, slug_override, format)
}

/// Run `jigc migrate-corpus` against the current working directory: locate the repo +
/// project layer, build the frozen persisted doctypes' v0→v1 migration jobs from the pack,
/// migrate the committed corpus (per-doc gated, byte-stable, the schema-version stamp
/// flipped last), render the report through the selected `format`, and print it. A clean
/// run exits 0 — even with docs routed to the agent to author prose (a blocked doc is an
/// expected interim state of the detect→block→migrate loop, not a failure). A locator error
/// (no repo / no project layer) routes to stderr and exits non-zero
/// (`design/corpus-migration.md` → Acceptance flows; `design/worked-examples.md` → flow 35).
///
/// `options` carries the commit boundary's two opt-outs (`design/corpus-migration.md` → The
/// commit boundary): `--no-commit` (write, land nothing) and `--dry-run` (write nothing, land
/// nothing — the report an applying run would print).
fn run_migrate_corpus(format: Format, options: migrate_corpus::Options) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    migrate_corpus::run(&cwd, format, options)
}

/// Run `jigc relocate <type> --from <prior-home>` against the current working directory: the
/// freeze-exempt relocation path (`design/corpus-migration.md` → Relocation: freeze-exempt
/// sibling). Locate the repo + project layer, resolve the doctype's current home, detect the
/// committed instances stranded at the supplied prior home, `git mv` each to the current home
/// via the T1 move primitive, render the report, and print it. A clean run exits 0 (even with
/// nothing to move — an idempotent no-op); a locator error or a frozen-doctype refusal routes
/// to stderr and exits non-zero.
fn run_relocate(format: Format, doctype: &str, from: &str) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    relocate::run(&cwd, doctype, from, format)
}

/// Run `jigc unmanage <rel_path>` against the current working directory: locate the
/// repo + project layer, load the index + file-state record, drop the doc's forward
/// edges + its file-state baseline (register-only — the file bytes are left on disk),
/// and render the outcome through the selected `format`. A clean run (a real drop *or*
/// an idempotent no-op) exits 0; a locator error (no repo / no project layer) routes to
/// stderr and exits non-zero (`design/project-setup.md` → Flow 2 hardening → Teardown /
/// cleanup (G5)).
fn run_unmanage(format: Format, path: &str) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match unmanage::run(&cwd, path) {
        Ok(report) => {
            println!("{}", render::unmanage(format, &report));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Run `jigc rename <old-slug> --to "<New Title>"` (optional `--slug`) against the current
/// working directory: locate the repo + project layer, repoint every persisted referrer
/// old→new, rewrite the moved doc's H1, `git mv`, and commit as one atomic transaction —
/// rendering the outcome through the selected `format`. A clean rename prints a summary on
/// stdout and exits 0; a missing target, a malformed address, or a pre-commit failure (the
/// transaction rolled back) surfaces on stderr (with its route) and exits non-zero
/// (`design/write-commands.md` → `jigc rename`).
fn run_rename(format: Format, old_slug: &str, to: &str, slug: Option<&str>) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    // The rollback's compare-and-swap conflicts, gathered on the failure arm and printed
    // beside this door's own frame (M52 Increment 5 / T6). Empty on every success and on every
    // pre-transaction refusal, so the `Err` arm below carries it unconditionally.
    let mut conflicts: Vec<engine::finding::Finding> = Vec::new();
    match rename::run(&cwd, old_slug, to, slug, &mut conflicts) {
        Ok(report) => {
            println!("{}", render::rename(format, &report));
            // The atomic rename commit's captured non-blocking hook stream — the same
            // string the report's `hook_output` key carries, relayed on the other channel
            // (stderr under `--format json`, the delimited stdout section on agent-text;
            // the hook_output producer axis).
            crate::task::relay_hook_output(format, &report.hook_output);
            Outcome::success()
        }
        Err(err) => {
            // A **pre-transaction refusal** (`rename::RefusalKind`) names itself: it travels
            // as a `render::BlockedFinding`, so the identity the surface prints is the
            // identity the invocation log records — a refused rename is legible there rather
            // than one more exit-1-with-nothing (M49 Inc 11 T4, PT-A). That read-back used to
            // be written out here, and being written out *here* was the defect: two doors had
            // it and ~29 did not (M50 completion audit, Finding 2). It now lives in the one
            // funnel `surface_commit_rejection` falls through to
            // ([`crate::invocation_log::operational_failure`]), so this door keeps the
            // behaviour without owning it — and the printed bytes are unchanged.
            //
            // The atomic rename transaction rolls back in full on any pre-commit failure
            // (`rollback_rename`), so a hook rejection leaves the store exactly as it was —
            // stated here as this door's half of the survivable frame, with its own re-run
            // and its own log identity (M47 Inc 3 T7).
            crate::task::surface_commit_rejection(
                format,
                &err,
                &crate::task::RejectionFrame {
                    code: crate::invocation_log::ERROR_RENAME_REJECTED,
                    // The doc address the caller named — this door acts on **one** doc, and
                    // its identity is what an operator reads the refusal back by. Declared at
                    // `design/command-output-contract.md` → the `*.commit-rejected` row.
                    target: old_slug.to_string(),
                    survived: format!(
                        "nothing was committed — the rename was rolled back, so `{old_slug}` \
                         still holds its original identity and every referrer still points \
                         at it"
                    ),
                    // `rollback_rename` restores the store byte-and-record identical on
                    // every pre-commit failure, hook or not, so one clause is true of both.
                    survived_non_hook: None,
                    rerun: format!(
                        "jigc rename {} --to {}{}",
                        crate::task::shell_token(old_slug),
                        crate::task::shell_token(to),
                        match slug {
                            Some(slug) => format!(" --slug {}", crate::task::shell_token(slug)),
                            None => String::new(),
                        },
                    ),
                },
                // A pre-transaction refusal and a rejected commit both roll the rename back
                // in full — and since M52 Increment 5 / T6 the two worktree paths HEAD cannot
                // answer for (the landing path, the gitignored file-state record) roll back
                // compare-and-swap, so a raced restore travels here as a blocking
                // `rename.rollback-conflict` printed beside the frame rather than in place of
                // it. A pre-transaction refusal never reaches a write, so its set is empty.
                &conflicts,
            )
        }
    }
}

/// Run `jigc validate` (the store-scope doc-code re-validation sweep) against the
/// current working directory: locate the repo + project layer, build the resolved
/// schemas + severity cascade, run the read-only committed-store walk
/// (`engine::validate::validate_store`) driving the CLI's production `doc-code` invoker,
/// and render the report through the selected `format`. The sweep is task-less and
/// **detect-and-report** — it gates no transaction.
///
/// The exit code follows the report-only rule with the **exit-flipping exceptions**
/// [`render::STORE_EXIT_FLIPS`] enumerates (`design/validation.md` → Exit semantics): a
/// content-only run (stale-anchor findings, or none) exits **0** (the `jigc ingest`
/// precedent — a blocking *content* finding still exits 0); a report matching any member of
/// that table exits **non-zero** (`Outcome::failure()`, *not* the task-gate
/// `EXIT_VALIDATION_BLOCKED` — this is not a transaction gate). The table is named here
/// rather than re-listed: it has grown three times (M35, M42, 2026-07-24, M46) and every
/// hand-count written beside it went stale. The decision is the shared
/// [`render::validation_store_exit_flips`] (keyed on the probe id / check id **directly**,
/// never on `report.has_blocking()` and never on a route string), so the exit code, the JSON
/// `report_only` field, and the trailer stay truthful in lockstep — a deliberate divergence
/// from the `run_upgrade` / `task validate` idiom.
///
/// The probe **pre-flight** runs before the sweep: an unresolvable `doc-code` probe bails
/// with one operational error here. A locator error (no repo / no project layer) likewise
/// routes to stderr and exits non-zero.
fn run_validate_store(format: Format) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match validate_store_in_repo(&cwd) {
        Ok(StoreSweep {
            report,
            unbaselined,
        }) => {
            println!(
                "{}",
                render::validation_store(format, &report, &unbaselined)
            );
            // The exit rule honors the exit-flipping exceptions (`validation.md` → Exit
            // semantics), and asks [`render::STORE_EXIT_FLIPS`] which they are rather than
            // re-enumerating them here: each member declares its own matcher, closing line
            // and criterion beside the others, and the members are not one criterion —
            // `pack-probe-integrity.*` and the two schema-version breaks mean the sweep
            // could not produce a trustworthy result, while `reconciliation.rename` and the
            // never-adopted squatter (M46) are sweeps that **worked** and are reporting a
            // real event they found. This comment enumerated three of them and was stale at
            // four. Otherwise (content-only or clean) detect-and-report exits 0, even when a
            // content finding blocks. Keyed on the probe id / check id directly, mirrored by
            // [`render::validation_store`]'s `report_only` field + trailer.
            let code = if render::validation_store_exit_flips(&report) {
                1
            } else {
                0
            };
            Outcome::with_findings(code, &report.findings)
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// One store sweep's result: the [`engine::result::ValidationReport`] **and** the set of
/// committed-doc identities the file-state record has **not** baselined — the discriminator the
/// report-only trailer's gate claim keys on ([`render::validation_store`]; `validation.md` → The
/// trailer must not claim a gate that does not exist).
///
/// The set rides alongside the report rather than inside it because it is not a *finding*: it is
/// the store-scope fact that decides, per finding, whether a task-scope gate exists for the doc it
/// is addressed at. It is derived here — the one place that holds the loaded record, the resolved
/// schemas, and the repo root at once.
struct StoreSweep {
    report: engine::result::ValidationReport,
    /// `<type>:<slug>` identities (the address a store-scope per-doc finding carries) of the
    /// committed instances with **no** `FileStateRecord` entry.
    unbaselined: std::collections::BTreeSet<String>,
}

/// The `<type>:<slug>` identities of every committed instance the file-state record has **not**
/// baselined (M42 Inc 4 — `validation.md` → The trailer must not claim a gate that does not
/// exist). The same `record.get(<repo-relative path>)` membership test M40's two-tier orphan
/// route (below) already discriminates on — and the key is the enumerator's own path, never a
/// path re-derived from the identity (a placement doctype's case-preserved literal home does not
/// round-trip through slug derivation).
///
/// Un-baselined is **not** a defect — it is the state of every committed doc in a fresh clone
/// (`.jigc/state/` is gitignored), in a brownfield adoption, and in any hand-authored corpus. It
/// is load-bearing here for one reason only: at **task** scope a committed doc is adjudicated by
/// `file_state::reconcile_committed_store`, whose UNKNOWN arm grades a nonconformant file
/// **advisory** (*routed, not recorded*) — so the store sweep's `conformance.*` /
/// `schema-conformance.*` break over it gates **nowhere**, and the trailer must not say it gates.
fn unbaselined_identities(
    repo_root: &Path,
    schemas: &std::collections::BTreeMap<String, engine::schema::Schema>,
    record: &engine::file_state::FileStateRecord,
) -> std::collections::BTreeSet<String> {
    let mut unbaselined = std::collections::BTreeSet::new();
    for (ty, schema) in schemas {
        for (identity, path) in engine::index::committed_instances(repo_root, ty, schema) {
            let key = path
                .strip_prefix(repo_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            if record.get(&key).is_none() {
                unbaselined.insert(identity);
            }
        }
    }
    unbaselined
}

/// Locate the repo + project layer from `cwd`, **pre-flight the `doc-code` probe**, then
/// build the resolved schemas (keyed by doctype) + severity cascade and run the
/// committed-store sweep against the production `doc-code` invoker. The pre-flight bails
/// before any sweep work when the probe is unresolvable, so a missing probe is one
/// operational error, never N per-anchor crash meta-findings. Mirrors `run_ingest`'s
/// locate preamble + `task.rs`'s `schemas()` / `resolve_severity_cascade` idiom; the
/// engine stays domain-empty (the CLI feeds the pack in).
fn validate_store_in_repo(cwd: &Path) -> Result<StoreSweep> {
    let jigc_home = require_project_layer(cwd)?;
    require_doc_code_probe()?;
    let pack = crate::pack::make_pack()?;
    let pack = pack.as_ref();
    let project_config = jigc_home.join(".jigc").join("config");
    let resolved = crate::start::resolve_severity_cascade(pack, &project_config)?;

    // workflow↔refs family: enumerate every cascade-resolved workflow definition as a
    // per-workflow bundle (id + bytes + **origin-pack** catalog, address-sorted, layer-aware)
    // + the layer-aware step source the engine scopes per-workflow — the proven `describe`
    // enumeration idiom (`validation.md` → Completing the envelope). Resolving each catalog
    // against the workflow's origin pack (mirroring `jigc start`) is what makes the
    // command-refs resolve **pack-locally**: once two packs compose, a loser-pack workflow's
    // `{{cli.X}}` is checked against ITS OWN pack's catalog, never the precedence-winner's
    // (`multi-pack.md` → Pack-local body-reference resolution).
    let defs = crate::start::CascadeDefs::new(&resolved, &project_config);

    // The schema map fed to BOTH the doc↔code family and the file↔CLI-state twin resolves
    // **through the cascade**, not pack-only — a project `schemas/<id>.yaml` whole-file
    // shadow (e.g. one that relocates a doctype's `location:`) must be honored, the same
    // `project > team > pack-default` rule every other definition resolves by (the
    // workflow↔refs family already reads cascade-aware via `defs.read_workflow`, and its
    // field-types per-origin via `all_schemas`'s `origin_pack` lookup).
    let schemas = defs.all_schemas(pack)?;
    let workflows = crate::start::enumerate_store_workflows(pack, &defs)?;
    let workflow_source = crate::start::CascadeStepSource::new(pack, &resolved, &project_config);

    // file↔CLI-state family: the loaded file-state record, read **read-only** (the
    // detect-without-absorb twin borrows it `&`, opens no write).
    let jigc_root = jigc_home.join(".jigc");
    let record = engine::file_state::FileStateRecord::load(&jigc_root)
        .with_context(|| format!("loading the file-state record at {jigc_root:?}"))?;

    // The doctype → manifest schema-version map the fifth content family's version-aware
    // route keys on (`design/validation.md` → Version-aware routing): a non-conformant
    // committed doc stamped below its doctype's manifest version routes `migrate`, one at the
    // version routes `corrupt`. Read from the pack's freeze manifest; a non-freeze pack
    // yields the empty map (every finding un-routed).
    let versions = crate::pack::frozen_doctype_versions(pack);

    // The doctype → **shipped prior-version schema shapes** map the fifth family's
    // managed-vs-foreign classifier keys its parse-against-a-prior arm on (`design/validation.md`
    // → The managed-vs-foreign discriminator): an **unstamped** committed doc that parses against
    // a shape jigc once shipped is a **managed, v0-era** doc (route: the corpus migration), not a
    // foreign file squatting at a managed home (route: adoption). Read from the pack's snapshot
    // store; a pack that ships none yields the empty map.
    let priors = crate::pack::prior_doctype_schemas(pack, &versions);

    let mut report = engine::validate::validate_store_families(
        &jigc_home,
        &schemas,
        &resolved,
        &crate::task::doc_code_invoker,
        &workflows,
        &workflow_source,
        &record,
        &versions,
        &priors,
    )
    .with_context(|| format!("validating the committed store at {jigc_home:?}"))?;

    // Binary-provenance stamp advisory (M36, `design/storage.md` → Store provenance): a
    // store stamped by a different `jigc` build than the one now reading it gets a
    // report-only, **un-keyed** `store-version.binary-mismatch` advisory — a cross-machine
    // heads-up that two builds may resolve the cascade differently, never a gate (its
    // `(store-version, binary-mismatch)` handle is not a `CHECK_INVENTORY` row, and it does
    // not match `validation_store_exit_flips`, so the exit stays 0). An absent stamp (a
    // pre-M36 store) is never false-flagged. CLI-side because `.jigc/version` is a
    // store-level fact the engine (empty by invariant) never reads.
    //
    // **Schema-version-aware since M42** (`design/storage.md` → Store provenance — the check):
    // its route was a **false all-clear** over a stale corpus — it named only "re-run `jigc
    // setup`", which re-stamps `.jigc/version` and thereby self-clears this advisory while the
    // committed docs stay stale. The fix keys on the **machine handle** the fifth family mints
    // for exactly this fact — a `schema-conformance.schema-version-current` break in the
    // just-built report — never on a route-string prefix (a coupling the Settle rejected). A
    // stale corpus routes at `jigc migrate-corpus` first; a current one keeps the plain
    // align-or-re-stamp route.
    let corpus_stale = report
        .findings
        .iter()
        .any(|f| f.code == engine::validate::SCHEMA_VERSION_CURRENT_CODE);
    if let Some(finding) = crate::setup::binary_mismatch_finding(&jigc_home, corpus_stale) {
        report.findings.push(finding);
    }

    // The home-re-point strand advisory (M36 for `docs-root`, M49 Increment 7 for
    // `placement-root`; `design/validation.md` → Orphan detection): a committed `.md` left
    // outside its doctype's current resolved home by a re-point of the knob that resolves that
    // home. The engine's file-state twin only sees docs it has a `FileStateRecord` for; this
    // fills the coverage hole by enumerating `git ls-files` (committed truth that survives a
    // fresh clone), matched on the part the home's knob leaves invariant — never a bare `.md`
    // match (`README.md` stays unflagged).
    //
    // **Two-tier since M40** (`design/validation.md` → M40 two-tier route): the classify
    // heuristic is unchanged and emission is NEVER gated on `FileStateRecord` membership
    // (a fresh clone still fires — the M36 never-silent property), but the *finding*
    // discriminates on it:
    //  - **registered** (the record knows the path) → a genuine strand:
    //    `file-state.orphaned-doc` + the existing relocate / `jigc unmanage` route;
    //  - **unregistered** (never adopted) → the looks-managed-but-unregistered advisory
    //    `file-state.unregistered-doc`, routed `jigc migrate <path> --as <doctype>`-or-
    //    ignore when that workflow ships (the `migrate.rs` prefix idiom), else
    //    ignore-or-human — `unmanage` on a never-registered doc is a proven no-op loop
    //    (the adoption trial's `research/notes.md`), and a route must never command a
    //    verb that hard-errors.
    // Both tiers stay report-only + **un-keyed**: neither code is a `CHECK_INVENTORY`
    // row nor matches `validation_store_exit_flips`, so the exit stays 0. CLI-side
    // because the engine ships domain-empty and never shells to git for tracked status.
    //
    // **Both home shapes since M49 Increment 7.** The arm above was `location:`-only because a
    // `placement:` home was a constant no knob could move; `placement-root` ends that, so the
    // detector follows — else `jigc validate` prints *the committed store validates clean* over
    // a baselined doc stranded at the doctype's prior home, while the `location:` twin of that
    // exact state has been reported since M36. Each tier's wording keys on the matched
    // doctype's **current home shape**, so neither names the knob that did not move it.
    let declared = defs.declared_schemas(pack)?;
    // The paths the strand walk speaks for — the other half of M51 Increment 8's partition
    // (below): a strand's doctype RESOLVES, so the orphaned-instance break must not also
    // claim it.
    let mut spoken_for = std::collections::BTreeSet::new();
    for strand in crate::orphan::orphaned_docs(&jigc_home, &declared, &schemas) {
        let crate::orphan::Strand {
            rel,
            doctype,
            current,
        } = strand;
        spoken_for.insert(rel.clone());
        let finding = if record.get(&rel).is_some() {
            let (diagnosis, route) = match &current {
                crate::orphan::Home::Location(_) => (
                    format!(
                        "committed doc `{rel}` sits outside the resolved doctype roots — a \
                         `docs-root` change likely stranded it (it looks managed but resolves \
                         under no doctype location)"
                    ),
                    "move it under the current resolved root (re-point `docs-root` to cover \
                     it) or drop it with `jigc unmanage`"
                        .to_string(),
                ),
                crate::orphan::Home::Placement(file) => (
                    format!(
                        "committed doc `{rel}` sits outside `{doctype}`'s resolved home \
                         `{file}` — a `placement-root` change likely stranded it (the home \
                         moved, the committed instance did not)"
                    ),
                    format!(
                        "move it to `{file}` and re-run `jigc ingest`, re-point \
                         `placement-root` to cover where it sits, or drop it with \
                         `jigc unmanage {token}`",
                        token = crate::task::shell_token(&rel)
                    ),
                ),
            };
            engine::finding::Finding::graded(
                engine::finding::Severity::Advisory,
                "file-state.orphaned-doc",
                diagnosis,
                Some(engine::finding::Location::addressed(rel, 1, 1)),
                Some(route.into()),
            )
        } else {
            let migrate_workflow = format!("migrate-{doctype}");
            let migratable = pack
                .list(engine::packsource::PackResourceKind::Workflows)
                .iter()
                .any(|id| *id == engine::packsource::ResourceId::from(migrate_workflow.as_str()));
            let route = crate::orphan::unregistered_route(&rel, &doctype, migratable);
            let looks_like = match &current {
                crate::orphan::Home::Location(_) => {
                    format!("it sits under a `{doctype}`-style directory")
                }
                crate::orphan::Home::Placement(file) => {
                    format!(
                        "it carries `{doctype}`'s home filename, whose resolved home is `{file}`"
                    )
                }
            };
            engine::finding::Finding::graded(
                engine::finding::Severity::Advisory,
                "file-state.unregistered-doc",
                format!(
                    "committed doc `{rel}` looks managed ({looks_like}) but was never \
                     adopted — a basename coincidence or an un-ingested foreign doc, not a \
                     tracked strand"
                ),
                Some(engine::finding::Location::addressed(rel, 1, 1)),
                Some(route.into()),
            )
        };
        report.findings.push(finding);
    }

    // The orphaned-instance break (M51 Increment 8 / T3; `design/validation.md` → The M51
    // registrations — Increment 8): a committed doc jigc stamped whose declared doctype is
    // defined by **no** resolved schema — the instances a doctype leaves behind when the pack
    // that owns it leaves the composition. The partition against the strand advisory above is
    // asked once, on whether the doctype resolves; `spoken_for` carries that answer for the
    // paths the strand walk already named. Blocking, and `render::STORE_EXIT_FLIPS`' sixth
    // member, so the sweep refuses the exit-0 green rather than reporting that it found nothing
    // in files it has no schema to read. CLI-side, like every arm above it: the subject is the
    // committed set, which only `git ls-files` knows.
    //
    // **The subject is jigc's declared territory, not the repository** (M51 completion audit):
    // the stamp is the unnamespaced key `schema-version:`, so it cannot tell jigc's own stamp
    // from a team document using that key for its own purposes — the *home* is what discriminates.
    // `Territory` derives that from the two resolved knobs and the resolved doctype homes, and
    // carries the two residuals it leaves.
    let territory = crate::orphan::Territory::resolve(
        crate::start::docs_root_prefix(&resolved),
        crate::start::placement_root(&resolved),
        &schemas,
    );
    for rel in crate::orphan::orphaned_instances(&jigc_home, &schemas, &territory, &spoken_for) {
        report
            .findings
            .push(crate::orphan::orphaned_instance_finding(&rel));
    }

    // The gate-claim discriminator (M42 Inc 4): which of the committed docs this sweep just
    // adjudicated carry no file-state baseline. Derived from the same `record` + `schemas` the
    // families ran against, so the trailer's claim and the sweep's verdicts describe one store.
    let unbaselined = unbaselined_identities(&jigc_home, &schemas, &record);
    Ok(StoreSweep {
        report,
        unbaselined,
    })
}

/// The store sweep's **probe pre-flight**: resolve the `doc-code` probe program (the
/// `JIGC_DOC_CODE_PROBE` override else the `<bin-dir>/doc-code` sibling) and require it
/// to be an existing file before the sweep runs. A missing probe is a single
/// misconfiguration to report once — not a `crash` meta-finding per anchor — so this
/// bails with **one** operational error naming the resolved path + the override knob,
/// and the handler routes it to stderr with a non-zero exit (`design/validation.md` →
/// Distribution bound; review S2).
fn require_doc_code_probe() -> Result<()> {
    let program = crate::invoke::doc_code_program();
    if !program.is_file() {
        anyhow::bail!(
            "`doc-code` probe not found at {program:?} — place the `doc-code` binary beside `jigc` or set `JIGC_DOC_CODE_PROBE` to its path"
        );
    }
    Ok(())
}

/// Locate the repo root and its `.jigc/config/` project layer — the store-walk locate
/// preamble shared with `jigc ingest` / `jigc upgrade`. Errors with routed messages when
/// the repo or the project layer is absent.
fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    let ctx = crate::locate::locate(cwd)?;
    if ctx.project_config.is_none() {
        return Err(crate::locate::not_set_up());
    }
    // The committed doc-store + `.jigc/` bind to jigc_home (the main checkout); the store
    // sweep does no git I/O, so jigc_home is the single base it needs (M31 Inc 2 / WF3).
    Ok(ctx.jigc_home)
}

/// Dispatch a `jigc doc <verb>` write against the active task in the current
/// working directory. The `doc` surface is agent-facing structured I/O (the
/// staged buffer + the route on a block); a **blocking finding** renders through
/// the global `--format` (a JSON envelope under `--format json`), so an agent on
/// `--format json` gets a parseable block (`design/write-commands.md` → The verbs).
fn run_doc(format: Format, verb: DocCommand) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    verb.dispatch(&cwd, format)
}

/// Mint a task from `intent` and compose the cascade's default workflow against
/// the current working directory, render the composed view through the selected
/// `format`, and print it — mapping success/failure to the process exit code. A
/// blocking gate / minting finding surfaces on stderr (with its route) and exits
/// non-zero; nothing is emitted past a block.
fn run_compose(format: Format, intent: &str, slug: Option<&str>) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match start::compose_in_repo(&cwd, intent, slug) {
        Ok(view) => {
            println!("{}", render::composed(format, &view));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Compose the explicitly-named `workflow` from `intent` against the current
/// working directory — the `jigc start --workflow <X> "<intent>"` front door
/// (Form D, `design/write-commands.md` → Task origination). Bypasses the cascade
/// default and composes the *named* workflow, minting iff it declares
/// `creates-task: true`. Renders the composed view through the selected `format`
/// and maps success/failure to the exit code: a clean compose prints on stdout
/// and exits 0; an unknown `<X>` or a blocking gate finding surfaces on stderr
/// (with its route) and exits non-zero — nothing is emitted past a block, and an
/// unknown id is rejected before any mint.
fn run_compose_named(format: Format, intent: &str, workflow: &str, slug: Option<&str>) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match start::compose_named_in_repo(&cwd, intent, workflow, slug) {
        Ok(view) => {
            println!("{}", render::composed(format, &view));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Compose the explicitly-named `workflow` with **no `<intent>`** against the
/// current working directory — the `jigc start --workflow <X>` form with no
/// positional. A `creates-task: false` `<X>` composes (no mint); a
/// `creates-task: true` `<X>` is rejected with an actionable message (it slugs its
/// task id from the intent), rather than silently falling through to orientation
/// (`design/write-commands.md` → Task origination). Renders the composed view on
/// success (exit 0); a rejection or blocking finding surfaces on stderr (with its
/// route) and exits non-zero.
fn run_compose_named_no_intent(format: Format, workflow: &str) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match start::compose_named_no_intent_in_repo(&cwd, workflow) {
        Ok(view) => {
            println!("{}", render::composed(format, &view));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Render the `--explain` resolution tree against the current working directory:
/// resolve the cascade, build the tree over the resolved default workflow (or the
/// `--workflow <X>`-named one) — WITHOUT minting (the tree is task-independent) —
/// render it through the selected `format`, and print it. A blocking resolution
/// finding (an orphaned anchor, an unknown workflow) surfaces on stderr (with its
/// route) and exits non-zero (`design/workflow-dialect.md` → `--explain` output
/// contract; `design/worked-examples.md` → 3a).
fn run_explain(format: Format, intent: Option<&str>, workflow: Option<&str>) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match start::compose_explain_in_repo(&cwd, intent, workflow) {
        Ok((tree, pack_label)) => {
            println!("{}", render::explain(format, &tree, &pack_label));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Resume an existing task by id and re-compose its default workflow against the
/// current working directory, render the composed view through the selected
/// `format`, and print it. The re-compose reads the task's persisted bound
/// context roles (`.jigc/tasks/<id>/roles.json`), so a role bound since the task
/// was minted (e.g. an in-task ADR) resolves in the composed view. A missing task
/// or a blocking gate finding surfaces on stderr (with its route) and exits
/// non-zero.
fn run_resume(format: Format, id: &str) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match start::resume_in_repo(&cwd, id) {
        Ok(view) => {
            println!("{}", render::composed(format, &view));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Re-enter a milestone sub-task as a fanned sub-agent: compose the
/// explicitly-named `workflow` for sub-task `task` against the current working
/// directory, render the composed view through the selected `format`, and print
/// it. The CLI asserts `workflow` equals the sub-task's recorded mint workflow
/// before composing — a mismatch (a stale launch template) is a blocking finding
/// on stderr (with its route) and exits non-zero; an unknown sub-task or a
/// blocking gate finding likewise surfaces on stderr and exits non-zero
/// (`design/write-commands.md` → Sub-agent re-entry).
fn run_reenter(format: Format, workflow: &str, task: &str) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match start::reenter_in_repo(&cwd, workflow, task) {
        Ok(view) => {
            println!("{}", render::composed(format, &view));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Run `jigc workflow <W> --preview` against the current working directory: compose the
/// `creates-task: true` workflow `<W>`'s step text **without minting a task**, and
/// render it through the mint-first [`render::composed_preview`] surface. A
/// `creates-task: false` `<W>` (the router and its kind) mints nothing to begin with,
/// so [`start::preview_in_repo`] rejects it with the route to run it directly.
fn run_preview(format: Format, workflow: &str) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match start::preview_in_repo(&cwd, workflow) {
        Ok(view) => {
            println!("{}", render::composed_preview(format, &view, workflow));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Run the bare-`start` orientation against the current working directory,
/// render the structured result through the selected `format`, and print it —
/// mapping success/failure to the process exit code.
fn run_orient(format: Format) -> Outcome {
    let cwd = match cwd_or_refusal(format) {
        Ok(cwd) => cwd,
        Err(refusal) => return refusal,
    };
    match orient::orient(&cwd) {
        Ok(view) => {
            println!("{}", render::orientation(format, &view));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Which side of the read/write line a leaf verb sits on.
///
/// The classification is by the verb's **job**, and a verb is [`VerbKind::Read`] only
/// when *no* invocation of it acts: it reports what is there and mutates nothing —
/// neither repo files, nor the git index/history, nor the `.jigc/` workbench (minting a
/// task included). Anything that can act is [`VerbKind::Write`], including the mixed
/// verbs whose read-shaped form is one flag among several (`jigc start` orients bare and
/// mints with an intent; `jigc milestone execute` mints nothing yet reseeds the cache).
///
/// **One carve-out, and it is what lets the rule above be enforced rather than aspired to**
/// (M49 Increment 2, T4). A `Read` verb may materialize a **self-healing derived cache**:
/// state that is a pure function of the committed store at HEAD, that carries the HEAD it was
/// built against, and that is rebuilt from the store on any stamp miss. `jigc task validate`
/// writes `.jigc/index/edges.json` on its first run against a new HEAD and is inert after
/// (`engine::index::load_committed`, *"stamp-rebuildable by construction"*;
/// `design/storage.md` → Edge index lifecycle, the *committed rebuild* site) — losing that
/// file costs one rebuild, never a wrong answer. What a `Read` verb may **never** do is
/// create, alter or resurrect state a later door reads as **authority**. A sub-task's
/// `.jigc/tasks/<id>/` working area is authority — its `workflow` file decides what a
/// re-entry composes — and `jigc milestone list-tasks` rebuilt exactly that from a committed
/// record that carried no minting workflow at the time, so an `--workflow`-overridden sub-task
/// came back under the pack default: a read verb fabricating provenance. The record carries a
/// per-item `workflow` leaf since M49 Increment 9 / T3, so that value would be sourced rather
/// than invented today — which retires the example and leaves the rule exactly where it was: a
/// `Read` leaf may not materialize a working area whatever it would put in it. It reads the record
/// and writes nothing now (`crate::milestone`'s `run_list_tasks`), and the whole `Read` set is
/// swept over a state built to reveal a write by `tests/read_verb_acts_nothing.rs`.
///
/// It exists because *"a read intent must never be answered with a write verb"* is a
/// claim about **every** leaf of the clap tree, not about the handful of guesses a trial
/// happened to record — so the rule that enforces it reads this table rather than a
/// curated `(parent, guess)` list, and the table is fenced total against the tree
/// (`cli_parse::every_leaf_verb_is_classified`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbKind {
    /// Reports state; mutates nothing. The honest answer to a read intent.
    Read,
    /// Acts — on the repo, on git, or on the workbench. Never offered to a read intent.
    Write,
}

/// Every **leaf** verb's argv path with its [`VerbKind`] — total over the clap tree by
/// fence, so a verb added anywhere must classify itself before it can ship.
pub const VERB_KINDS: &[(&[&str], VerbKind)] = &[
    // Top level.
    (&["start"], VerbKind::Write),
    (&["workflow"], VerbKind::Write),
    (&["setup"], VerbKind::Write),
    (&["uninstall"], VerbKind::Write),
    (&["upgrade"], VerbKind::Read),
    (&["ingest"], VerbKind::Write),
    (&["migrate"], VerbKind::Write),
    (&["migrate-corpus"], VerbKind::Write),
    (&["unmanage"], VerbKind::Write),
    (&["rename"], VerbKind::Write),
    (&["relocate"], VerbKind::Write),
    (&["describe"], VerbKind::Read),
    (&["validate"], VerbKind::Read),
    // `jigc doc` — the managed-doc surface.
    (&["doc", "create"], VerbKind::Write),
    (&["doc", "add-item"], VerbKind::Write),
    (&["doc", "remove-item"], VerbKind::Write),
    (&["doc", "retitle-item"], VerbKind::Write),
    (&["doc", "rename"], VerbKind::Write),
    (&["doc", "set-field"], VerbKind::Write),
    (&["doc", "set-slot"], VerbKind::Write),
    (&["doc", "author"], VerbKind::Write),
    (&["doc", "show"], VerbKind::Read),
    (&["doc", "schema"], VerbKind::Read),
    (&["doc", "list"], VerbKind::Read),
    // `jigc task` — the task lifecycle.
    (&["task", "list"], VerbKind::Read),
    (&["task", "diff"], VerbKind::Read),
    (&["task", "validate"], VerbKind::Read),
    (&["task", "discard"], VerbKind::Write),
    (&["task", "finalize"], VerbKind::Write),
    (&["task", "bind"], VerbKind::Write),
    // `jigc config` — the cascade surface.
    (&["config", "set"], VerbKind::Write),
    (&["config", "insert-step"], VerbKind::Write),
    (&["config", "replace-step"], VerbKind::Write),
    (&["config", "remove-step"], VerbKind::Write),
    (&["config", "fill"], VerbKind::Write),
    (&["config", "fork"], VerbKind::Write),
    (&["config", "get"], VerbKind::Read),
    (&["config", "list"], VerbKind::Read),
    // `jigc milestone` — the work-unit surface.
    (&["milestone", "create"], VerbKind::Write),
    (&["milestone", "add-task"], VerbKind::Write),
    (&["milestone", "add-from-spec"], VerbKind::Write),
    (&["milestone", "list-tasks"], VerbKind::Read),
    (&["milestone", "provision"], VerbKind::Write),
    (&["milestone", "execute"], VerbKind::Write),
    (&["milestone", "join"], VerbKind::Write),
    (&["milestone", "finalize"], VerbKind::Write),
    (&["milestone", "discard"], VerbKind::Write),
];

/// **What a leaf verb does on the user's behalf** — the third classification of the clap
/// tree, beside [`VerbKind`] and [`DoctypeArg`], and the one a **repository posture** is
/// adjudicated against ([`crate::repo::PostureMember`]).
///
/// The two acting classes are separate because they adjudicate **different** families, not
/// because one is a stronger flavour of the other:
///
///   * [`ActsOnBehalf::CommitsOnBehalf`] — the door runs a `git commit` the user did not
///     type, so **every** member of the posture family applies: a commit made on a detached
///     HEAD belongs to no branch, a commit made on an unborn HEAD pins git's empty tree as a
///     base, and a commit made under a live `MERGE_HEAD` concludes a merge the user started,
///     under jigc's own subject.
///   * [`ActsOnBehalf::MovesOnBehalf`] — the door `git mv`s a **committed** file through the
///     one move primitive ([`crate::relocate::move_doc`], whose own doc-comment names its
///     four doors) and commits nothing. Only *operation in progress* applies: the move lands
///     in the index for the user to commit, so which commit it joins stays theirs to decide,
///     while moving a tracked file out from under a half-finished merge is not.
///   * [`ActsOnBehalf::Neither`] — a **stated verdict, not an absence**. `jigc migrate` mints
///     a task and writes into the workbench; `jigc doc rename` moves **staged** bytes inside
///     `.jigc/tasks/<id>/` ([`crate::doc`]'s `move_staged_identity`, an `fs::rename` of a file
///     no commit has); `jigc milestone provision` adds worktrees. None of them commits or moves
///     a committed file, so none of them owes the user a posture verdict.
///
/// **One door, one class.** [`crate::rename`] both `git mv`s and commits, and
/// [`crate::migrate_corpus`] relocates (by `fs::rename`, *not* `git mv`) and commits: both are
/// `CommitsOnBehalf`, because the commit is the act that makes the whole posture family
/// relevant and `COMMITTING_DOORS ⊆ commit-on-behalf` is asserted here
/// (`cli_parse::every_committing_door_acts_on_the_users_behalf`).
pub enum ActsOnBehalf {
    /// The door lands a commit the user did not type — the **full** posture family, minus
    /// whatever this row states as [`PostureExemption`].
    CommitsOnBehalf {
        /// A **runnable** argv for this door with [`WORK_UNIT_ID_SLOT`] standing in for a
        /// work-unit id where one is needed — every other argument present and well-formed,
        /// so the posture is the only thing the door can fault on. The axis suite drives the
        /// row's own argv rather than a hand-written cell list.
        argv: &'static [&'static str],
        /// The members this door does **not** adjudicate, each with its reason. Empty on
        /// every row but one: a hole in a family is a decision or it is a bug, and this is
        /// how a door states which.
        exempt: &'static [PostureExemption],
    },
    /// The door `git mv`s a committed file and commits nothing — *operation in progress*
    /// only. The variant carries **no** exemption field on purpose: a mover that was exempt
    /// from the one member it adjudicates would adjudicate nothing, and the type refuses to
    /// express it.
    MovesOnBehalf {
        /// A runnable argv, as [`ActsOnBehalf::CommitsOnBehalf`] carries.
        argv: &'static [&'static str],
    },
    /// The door acts on nobody's behalf — it carries no argv and no exemption, because it
    /// adjudicates no posture.
    Neither,
}

/// One **stated exemption** from the repository-posture family, carried on a
/// [`ActsOnBehalf::CommitsOnBehalf`] row.
pub struct PostureExemption {
    /// The member this door does not adjudicate.
    pub member: PostureMember,
    /// **Why** — quoted from the record that decided it, never paraphrased.
    pub reason: &'static str,
}

/// **Why `jigc setup` is exempt from [`PostureMember::HeadUnborn`]** — the M30 audit
/// rationale, **quoted** from [`crate::setup`]'s `commit_install` (`setup.rs:1617-1625`,
/// the repo's basis-unchanged form) rather than paraphrased, with only the comment's line
/// wrapping removed.
///
/// Driven, `git init -q . && jigc setup` lands `chore(jigc): install jigc workspace config`
/// at exit 0 today. Applying the family uniformly would turn **the QUICKSTART on-ramp — the
/// first command an adopter runs** — into a refusal routed at `jigc setup --force`, training
/// the very `--force` reflex the wave's own staged-set guard prices as its honest weakness
/// (`completions/artifacts/M51/settle-record.md` → Review amendments §3). The **detached** and
/// **operation-in-progress** members still apply at `setup`; only *unborn* is exempt.
pub const SETUP_UNBORN_EXEMPTION: &str = "Require a git work tree — but DO mint on an \
     **unborn HEAD** (a brand-new repo with no commits). Setup owns committing its own \
     install footprint regardless of HEAD state (M30 audit finding 1): on a cold-start repo \
     the first `finalize` since M30 stages only the task's change-set \
     ([`crate::task::stage_index_honoring`]), so if setup skipped the install here, \
     `CLAUDE.md`/`.claude/settings.json`/`.jigc/AGENT.md` would be left untracked after the \
     first managed commit.";

/// **One leaf verb and what it does on the user's behalf** — the row [`BEHALF_DOORS`]
/// carries.
pub struct BehalfDoor {
    /// The leaf verb path, as an operator types it after `jigc`.
    pub door: &'static [&'static str],
    /// What this door does on the user's behalf — and, for the two acting classes, the
    /// runnable argv that reaches it.
    pub acts: ActsOnBehalf,
}

/// **Every leaf verb, classified by what it does on the user's behalf** — a **total**
/// classification over the clap tree, on [`VERB_KINDS`]' mold and fenced ⇔ against it
/// (`cli_parse::every_leaf_verb_says_what_it_acts_on`), so a verb added anywhere **reddens
/// until someone answers it**.
///
/// It is total rather than a curated door list because a **lower bound plus one named
/// element is not a membership rule**: `COMMITTING_DOORS` says which doors log a
/// commit-rejection identity — ten rows over **nine** leaves (`milestone finalize` is two
/// commit models), saying nothing at all about the other thirty-eight — and `jigc setup`,
/// the one committing door whose install commit passes `--no-verify` and therefore carries
/// no rejection identity to log, is invisible to it. Four consumers read this one table
/// instead of hand-enumerating four different sets: the posture guard (M51 Increment 2 /
/// T3), the staged-set guard (Increment 3), the survivable frame's cause vocabulary
/// (N20), and — since Increment 8 / T1 — the **ambush-class source set**
/// ([`crate::pack::AMBUSH_CONTRACTS`]), whose derivation is *blocking codes minted by a
/// door in the commit-on-behalf class* and whose every row is looked up here.
///
/// **It carries no error identity.** A posture breach is a blocking `Finding`
/// ([`PostureMember::code`]), not an [`Outcome`] identity, so this table feeds no
/// `ERROR_CODE_REGISTRY` derivation and duplicates no fence
/// (`crate::invocation_log::ERROR_CODE_REGISTRY`; `settle-record.md` → §10).
pub const BEHALF_DOORS: &[BehalfDoor] = &[
    // Top level.
    //
    // `start` and `workflow` mint a task directory under the gitignored `.jigc/` workbench
    // and compose text; `ingest` is register-only ("never moving or rewriting a file");
    // `migrate` mints a migration task — every commit of that work is `task finalize`'s.
    BehalfDoor {
        door: &["start"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["workflow"],
        acts: ActsOnBehalf::Neither,
    },
    // The install commit (`crate::setup`'s `commit_install`) — the one committing door no
    // `COMMITTING_DOORS` row supplies, because it passes `--no-verify` by recorded design
    // and therefore has no hook rejection to carry an identity.
    BehalfDoor {
        door: &["setup"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["setup"],
            exempt: &[PostureExemption {
                member: PostureMember::HeadUnborn,
                reason: SETUP_UNBORN_EXEMPTION,
            }],
        },
    },
    // `uninstall` deletes the install footprint and commits nothing — a destroying door
    // (`DESTROYING_DOORS`), which is a different registry and a different consent.
    BehalfDoor {
        door: &["uninstall"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["upgrade"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["ingest"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["migrate"],
        acts: ActsOnBehalf::Neither,
    },
    // The pathspec-limited self-commit (`crate::migrate_corpus`). Its relocation arm moves
    // by `fs::rename`, **not** `git mv` — the commit boundary stages the move once, at the
    // end — so this door is commit-on-behalf and is no member of the mover class.
    BehalfDoor {
        door: &["migrate-corpus"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["migrate-corpus"],
            exempt: &[],
        },
    },
    BehalfDoor {
        door: &["unmanage"],
        acts: ActsOnBehalf::Neither,
    },
    // The atomic identity refactor: it `git mv`s through [`crate::relocate::move_doc`] AND
    // commits, so the commit decides the class (one door, one class).
    BehalfDoor {
        door: &["rename"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["rename", "adr:keeper", "--to", "Axis"],
            exempt: &[],
        },
    },
    // The freeze-exempt relocation: `git mv` per stranded instance, no commit — the user
    // commits the move themselves, so only an operation in progress is theirs to conclude
    // first.
    BehalfDoor {
        door: &["relocate"],
        acts: ActsOnBehalf::MovesOnBehalf {
            argv: &["relocate", "vision", "--from", "docs/vision/"],
        },
    },
    BehalfDoor {
        door: &["describe"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["validate"],
        acts: ActsOnBehalf::Neither,
    },
    // `jigc doc` — the managed-doc surface. Every verb writes into the task's working area
    // under `.jigc/tasks/<id>/`, which no commit has a copy of until `task finalize`
    // promotes it; `doc rename` moves those **staged** bytes with `fs::rename` and routes a
    // committed identity change to `jigc rename`.
    BehalfDoor {
        door: &["doc", "create"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "add-item"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "remove-item"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "retitle-item"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "rename"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "set-field"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "set-slot"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "author"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "show"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "schema"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["doc", "list"],
        acts: ActsOnBehalf::Neither,
    },
    // `jigc task` — the lifecycle. Two of the six land a commit.
    BehalfDoor {
        door: &["task", "list"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["task", "diff"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["task", "validate"],
        acts: ActsOnBehalf::Neither,
    },
    // A milestone sub-task's discard lands a record-only settle commit before it removes
    // the working area (`crate::milestone`'s `settle_discarded_sub_task`, M49 Inc 2 T3).
    BehalfDoor {
        door: &["task", "discard"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["task", "discard", WORK_UNIT_ID_SLOT],
            exempt: &[],
        },
    },
    // The commit boundary itself.
    BehalfDoor {
        door: &["task", "finalize"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["task", "finalize", WORK_UNIT_ID_SLOT],
            exempt: &[],
        },
    },
    BehalfDoor {
        door: &["task", "bind"],
        acts: ActsOnBehalf::Neither,
    },
    // `jigc config` — the cascade surface. Only `set` can move a committed file: the
    // `docs-root` and `placement-root` knobs carry a detect+route+**move** floor through
    // [`crate::relocate::move_doc`] (`crate::config`'s relocation arm), which is why the
    // row's argv names a root knob — a `config set` of any other key moves nothing, and a
    // row written that way would drive a cell that proves nothing (the `SLUG_DOORS`
    // `start`-row precedent).
    BehalfDoor {
        door: &["config", "set"],
        acts: ActsOnBehalf::MovesOnBehalf {
            argv: &["config", "set", "docs-root", "docs"],
        },
    },
    BehalfDoor {
        door: &["config", "insert-step"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["config", "replace-step"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["config", "remove-step"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["config", "fill"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["config", "fork"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["config", "get"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["config", "list"],
        acts: ActsOnBehalf::Neither,
    },
    // `jigc milestone` — the fan-out surface. The four record-only doors each land a
    // pathspec-limited commit of the milestone record (`crate::milestone`'s
    // `commit_record_only`); `finalize` lands the boundary's own commit under either
    // commit model. `provision` adds worktrees, `execute` reseeds the cache and `join`
    // folds into the workbench — none of the three commits or moves a committed file.
    BehalfDoor {
        door: &["milestone", "create"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["milestone", "create", "Axis milestone"],
            exempt: &[],
        },
    },
    BehalfDoor {
        door: &["milestone", "add-task"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["milestone", "add-task", WORK_UNIT_ID_SLOT, "axis intent"],
            exempt: &[],
        },
    },
    BehalfDoor {
        door: &["milestone", "add-from-spec"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["milestone", "add-from-spec", WORK_UNIT_ID_SLOT, "spec:axis"],
            exempt: &[],
        },
    },
    BehalfDoor {
        door: &["milestone", "list-tasks"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["milestone", "provision"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["milestone", "execute"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["milestone", "join"],
        acts: ActsOnBehalf::Neither,
    },
    BehalfDoor {
        door: &["milestone", "finalize"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["milestone", "finalize", WORK_UNIT_ID_SLOT],
            exempt: &[],
        },
    },
    BehalfDoor {
        door: &["milestone", "discard"],
        acts: ActsOnBehalf::CommitsOnBehalf {
            argv: &["milestone", "discard", WORK_UNIT_ID_SLOT],
            exempt: &[],
        },
    },
];

/// **How a doctype id reaches a leaf verb** — the second classification of the clap
/// tree, beside [`VerbKind`].
///
/// It exists because *"one unknown doctype, one answer"* is a claim about **every**
/// door that takes a doctype, and the doors were never enumerated: five different
/// shapes shipped across seven of them, two carrying no finding code and one carrying
/// no route at all, and a sixth leaked `PackResourceKind`'s `{:?}` at six `doc` write
/// verbs (M49 Increment 11 / T1; `DECISIONS.md` → 2026-08-31).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoctypeArg {
    /// A **bare** doctype id — the whole argument is the doctype (`jigc doc schema
    /// <DOCTYPE>`, `jigc migrate <path> --as <DOCTYPE>`, `jigc relocate <TYPE> …`).
    Bare,
    /// The `<type>` **head of a doc address** (`jigc doc set-slot <type>:<slug>#<slot>`,
    /// `jigc rename <type>:<slug> …`, `jigc task bind <role> <type>:<slug> <id>`).
    Address,
}

/// **What one clap argument's value is** — the classification the three door registries
/// take their vocabularies from.
///
/// Three of the four variants name a *caller-supplied token jigc turns into an identity
/// or a path component*; [`ArgToken::Plain`] is everything else, and it is a **stated
/// verdict, not an absence**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgToken {
    /// A **doctype id**, in the shape [`DoctypeArg`] names — the vocabulary
    /// [`DOCTYPE_DOORS`] is derived from.
    Doctype(DoctypeArg),
    /// A **work-unit id** — a task id or a milestone id; [`WORK_UNIT_ID_DOORS`].
    WorkUnitId,
    /// A **mint-time slug override** (`--slug`), taken verbatim; [`SLUG_DOORS`].
    SlugOverride,
    /// **None of the three identity families** — with the *path* half split out rather
    /// than left as a sentence.
    ///
    /// Until M51 this variant was payload-free and its doc-comment said, of the five
    /// arguments that do become paths, that *"a path argument is a path the caller
    /// **means** as one, adjudicated by the filesystem and by their own doors"*. Driven,
    /// the second half of that claim was false at `abd81df`: `jigc migrate` joined its
    /// `<path>` onto the repository root, read whatever came back and recorded the
    /// spelling as the value `finalize --approve` **deletes** — an absolute path outside
    /// the repository included. Nothing adjudicated it, at that door or at the sink.
    ///
    /// So the path half is now a [`PlainValue`] the author must answer, and answering
    /// [`PlainValue::PathBearing`] reddens `cli_parse::every_path_arg_occurrence_is_registered`
    /// until the argument's **occurrences** join [`PATH_ARG_OCCURRENCES`] with a rule —
    /// or with a stated no-rule-and-why.
    Plain(PlainValue),
}

/// **Does this argument's value become a path component?** — the answer every
/// [`ArgToken::Plain`] argument owes, and the vocabulary [`PATH_ARG_OCCURRENCES`] is
/// derived from.
///
/// **Two members, and the second is a verdict rather than a default.** The six
/// path-bearing ids are `path`, `file`, `from_file`, `from`, `target` and `value`
/// (`completions/artifacts/M51/settle-record.md` → D1 part 3, as amended by §2). Every
/// other argument is [`PlainValue::Other`], and the two **near misses** are named here
/// because classifying them by silence is what this split exists to stop:
///
///   * **`title`** does reach a path — but not as the caller typed it: a title is
///     *slugified* by jigc ([`engine::slug`]), and the minted slug is the path component.
///     The token the caller supplies is prose. The **verbatim** override of that mint is
///     `--slug`, which is why it carries its own family ([`ArgToken::SlugOverride`]) and
///     its own door registry.
///   * **`workflow`** selects a pack resource by id and a task working area is then named
///     after the **task id**, not after it; an id no pack declares is refused against the
///     loaded model before anything is minted (M49 Increment 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlainValue {
    /// The value **becomes a path component** at at least one of the argument's
    /// occurrences — so every occurrence owes a rule or a stated no-rule-and-why.
    PathBearing,
    /// The value becomes no path component anywhere: prose, a flag, a closed-vocabulary
    /// id resolved against the loaded model.
    Other,
}

/// **Every argument of every leaf verb, classified** — a *total* function over the clap
/// tree, and the one vocabulary [`DOCTYPE_DOORS`], [`WORK_UNIT_ID_DOORS`] and
/// [`SLUG_DOORS`] are derived from.
///
/// **Why total, and what it replaced.** Each of the three door registries used to be
/// derived from its own hand-kept list of argument *names* — `DOCTYPE_ARG_IDS =
/// ["type","doctype","as","addr","old_slug"]` and two siblings — with a ⇔ fence against
/// that list. An allowlist of names is a **finder, not a fence**: a door whose address
/// argument is named anything else is invisible to it, and one was. `jigc milestone
/// add-from-spec` takes its address through `spec_addr`, so it carried no
/// [`DOCTYPE_DOORS`] row, no `<slug>`-head guard ran on it, and driven at `d9e91f1` the
/// address `spec:../../../../<outside>/planted` **read a file from outside the
/// repository**, seeded a sub-task from its criteria and landed a commit naming the
/// foreign source — at exit 0, on the wave whose claim is that no caller-supplied token
/// becomes a path component unvalidated (the M50 completion audit, finding 1).
///
/// **A strictly-derived membership rule is not expressible against the clap tree**, and
/// that is measured rather than assumed: clap's introspection offers an argument's id,
/// its help and its value name — never the Rust type of the field it fills or where the
/// value flows, and every argument here is a `String`. Membership therefore cannot be
/// *computed*; it can only be *asked of the author*. **Totality is what forces the
/// question.** The complement is erased, so a new argument — address-shaped or not —
/// reddens `cli_parse::every_clap_argument_is_classified` until someone says what it
/// carries; saying [`ArgToken::Doctype`] reddens `every_doctype_door_is_registered`
/// until the door joins [`DOCTYPE_DOORS`]; and joining that reddens the axis suites
/// (`address_slug_head_axis.rs`, `unknown_doctype_axis.rs`, flow 51 arm 1) until the
/// door is actually driven. The old shape asked none of those questions of a door whose
/// argument nobody had thought to list.
///
/// One id, one classification: an id that appears on several leaves carries the same
/// kind of value at each of them, which is what makes the per-id table (rather than a
/// per-leaf one) sound — and `every_clap_argument_is_classified` holds the ⇔ against the
/// real tree in both directions, so neither a new argument nor a deleted one can pass.
///
/// **Declared bound.** The table can be answered *wrongly* — an author who classifies a
/// new address argument `Plain` gets a green gate, exactly as before. What it cannot be
/// is answered *silently*: the question is now asked of every argument that ships, which
/// is the strongest thing expressible against a tree that carries no types. The
/// [`DoctypeArg`] half is checked rather than believed —
/// `every_doctype_door_is_registered` asserts a door's registered shape equals the shape
/// its own argument carries.
///
/// **The `Plain` half is no longer outside that.** It used to be: the bound read *"nothing
/// can check `Plain` from the outside"*, and M51's driven damage sat inside exactly that
/// blind spot — `jigc migrate` recorded an absolute outside-the-repository `<path>` as the
/// value `--approve` deletes. `Plain` now carries [`PlainValue`], and a
/// [`PlainValue::PathBearing`] answer is checked twice over: every **occurrence** of the
/// argument must carry a [`PATH_ARG_OCCURRENCES`] row (`cli_parse::
/// every_path_arg_occurrence_is_registered`), and every row's arms are **driven** over the
/// whole escape-shape axis (`crates/cli/tests/path_arg_occurrence_axis.rs`). What stays
/// unchecked is an argument classified [`PlainValue::Other`] that does reach a path — the
/// same shape of bound, one family narrower.
pub const ARG_TOKENS: &[(&str, ArgToken)] = &[
    // ── The doctype family: a bare id, or the `<type>` head of an address ──
    ("addr", ArgToken::Doctype(DoctypeArg::Address)),
    ("as", ArgToken::Doctype(DoctypeArg::Bare)),
    ("doctype", ArgToken::Doctype(DoctypeArg::Bare)),
    ("old_slug", ArgToken::Doctype(DoctypeArg::Address)),
    // `jigc milestone add-from-spec <milestone> <spec-addr>` — the door the name-list
    // derivation could not see.
    ("spec_addr", ArgToken::Doctype(DoctypeArg::Address)),
    ("type", ArgToken::Doctype(DoctypeArg::Bare)),
    // ── The work-unit family: a task id or a milestone id ──
    ("id", ArgToken::WorkUnitId),
    ("milestone_id", ArgToken::WorkUnitId),
    ("task", ArgToken::WorkUnitId),
    // ── The mint-override family ──
    ("slug", ArgToken::SlugOverride),
    // ── Plain: no identity family — each answering whether its value becomes a path ──
    ("after", ArgToken::Plain(PlainValue::Other)),
    ("approve", ArgToken::Plain(PlainValue::Other)),
    ("before", ArgToken::Plain(PlainValue::Other)),
    ("carry_staged", ArgToken::Plain(PlainValue::Other)),
    ("commands", ArgToken::Plain(PlainValue::Other)),
    ("doctypes", ArgToken::Plain(PlainValue::Other)),
    ("dry_run", ArgToken::Plain(PlainValue::Other)),
    ("explain", ArgToken::Plain(PlainValue::Other)),
    ("file", ArgToken::Plain(PlainValue::PathBearing)),
    ("force", ArgToken::Plain(PlainValue::Other)),
    ("from", ArgToken::Plain(PlainValue::PathBearing)),
    ("from_file", ArgToken::Plain(PlainValue::PathBearing)),
    ("intent", ArgToken::Plain(PlainValue::Other)),
    ("key", ArgToken::Plain(PlainValue::Other)),
    ("no_commit", ArgToken::Plain(PlainValue::Other)),
    ("path", ArgToken::Plain(PlainValue::PathBearing)),
    ("preview", ArgToken::Plain(PlainValue::Other)),
    ("role", ArgToken::Plain(PlainValue::Other)),
    ("target", ArgToken::Plain(PlainValue::PathBearing)),
    ("title", ArgToken::Plain(PlainValue::Other)),
    ("to", ArgToken::Plain(PlainValue::Other)),
    ("unset", ArgToken::Plain(PlainValue::Other)),
    ("value", ArgToken::Plain(PlainValue::PathBearing)),
    ("workflow", ArgToken::Plain(PlainValue::Other)),
    ("workflows", ArgToken::Plain(PlainValue::Other)),
];

/// The [`ArgToken`] one clap argument id carries — `None` for an id the table does not
/// classify, which `cli_parse::every_clap_argument_is_classified` proves the real tree
/// never contains.
pub fn arg_token(id: &str) -> Option<ArgToken> {
    ARG_TOKENS
        .iter()
        .find(|(known, _)| *known == id)
        .map(|(_, token)| *token)
}

/// The clap argument ids that carry a **doctype** — [`ARG_TOKENS`]' `Doctype` projection,
/// and the vocabulary [`DOCTYPE_DOORS`] is fenced ⇔ against. `type` (`doc create`,
/// `relocate`), `doctype` (`doc author`/`schema`/`list`), `as` (`migrate`), `addr` (every
/// addressed `doc` verb and `task bind`), `old_slug` (`rename`) and `spec_addr`
/// (`milestone add-from-spec`).
pub fn doctype_arg_ids() -> Vec<&'static str> {
    ARG_TOKENS
        .iter()
        .filter(|(_, token)| matches!(token, ArgToken::Doctype(_)))
        .map(|(id, _)| *id)
        .collect()
}

/// The **shape** a doctype-carrying argument's value arrives in — `None` for an id that
/// carries no doctype. The door registry's own `DoctypeArg` is asserted equal to this, so
/// a row cannot describe its door's argument wrongly.
pub fn doctype_arg_shape(id: &str) -> Option<DoctypeArg> {
    match arg_token(id) {
        Some(ArgToken::Doctype(shape)) => Some(shape),
        _ => None,
    }
}

/// The clap argument ids that carry a **work-unit id** — `id` (`task diff`/`validate`/
/// `discard`/`finalize`/`bind`), `task` (the `--task <id>` scope flag on `start`,
/// `workflow` and every `doc` verb) and `milestone_id` (every operating `milestone`
/// verb).
///
/// The family exists because `.jigc/tasks/<id>/` and `.jigc/milestones/<id>/` are built
/// by joining a caller token onto a path, and *"every door asks whether that token is an
/// id"* is a claim about **every** door that takes one — which nobody had counted until
/// `jigc task discard "../.."` removed the repository (`DECISIONS.md` → 2026-09-05).
pub fn work_unit_id_arg_ids() -> Vec<&'static str> {
    ARG_TOKENS
        .iter()
        .filter(|(_, token)| matches!(token, ArgToken::WorkUnitId))
        .map(|(id, _)| *id)
        .collect()
}

/// The clap argument ids that carry a **mint-time slug override**. One member: a `--slug`
/// value drives a minted identity **verbatim** — it is never re-slugified — and a doc's
/// slug *is* its path component (`<docs-root>/<location>/<slug>.md`), so an override that
/// is not a slug names a file nobody addressed. `old_slug` on `jigc rename` is a doctype
/// **address**, classified in that family and guarded as its slug head.
pub fn slug_arg_ids() -> Vec<&'static str> {
    ARG_TOKENS
        .iter()
        .filter(|(_, token)| matches!(token, ArgToken::SlugOverride))
        .map(|(id, _)| *id)
        .collect()
}

/// **Every leaf verb that takes a doctype**, with the shape it arrives in — derived
/// from the clap tree by [`doctype_arg_ids`] and fenced against it, so the set is the
/// binary's, not a remembered one.
///
/// The **answer** each door owes is decided by what the doctype is *for*, not by which
/// verb asked — two shapes, and the split is deliberate:
///
///   * a **create-gate** door (`doc create` / `doc author`) is naming a doctype to
///     *mint*, so its refusal is `create.unknown-doctype` and names the authorable set;
///   * every other door is naming a doctype that must already **exist** in the resolved
///     cascade, so its refusal is `store.unknown-type` — one fault, one code, whether
///     the id arrived bare or as an address head.
///
/// A door whose usable set is *narrower* than the cascade's (only `jigc migrate --as`,
/// which needs a shipped `migrate-<doctype>` workflow) says so in its **message**; the
/// code stays the fault and the route stays the runnable doctype surface.
pub const DOCTYPE_DOORS: &[(&[&str], DoctypeArg)] = &[
    // Top level.
    (&["migrate"], DoctypeArg::Bare),
    (&["relocate"], DoctypeArg::Bare),
    (&["rename"], DoctypeArg::Address),
    // `jigc doc` — the managed-doc surface.
    (&["doc", "create"], DoctypeArg::Bare),
    (&["doc", "author"], DoctypeArg::Bare),
    (&["doc", "schema"], DoctypeArg::Bare),
    (&["doc", "list"], DoctypeArg::Bare),
    (&["doc", "add-item"], DoctypeArg::Address),
    (&["doc", "remove-item"], DoctypeArg::Address),
    (&["doc", "retitle-item"], DoctypeArg::Address),
    (&["doc", "rename"], DoctypeArg::Address),
    (&["doc", "set-field"], DoctypeArg::Address),
    (&["doc", "set-slot"], DoctypeArg::Address),
    (&["doc", "show"], DoctypeArg::Address),
    // `jigc task` — the task lifecycle.
    (&["task", "bind"], DoctypeArg::Address),
    // `jigc milestone` — the fan-out surface. `add-from-spec` names the committed spec it
    // seeds sub-tasks from, so it takes an address exactly as the `doc` read doors do.
    (&["milestone", "add-from-spec"], DoctypeArg::Address),
];

/// The token a [`WORK_UNIT_ID_DOORS`] row's `argv` carries **in place of** the work-unit
/// id, so one row serves every cell of the axis (`""`, a traversal, an absolute path, a
/// well-formed-but-unknown id) instead of four hand-written argvs per door.
///
/// The [`BEHALF_DOORS`] rows carry it too, for the same reason and with the same meaning —
/// a task id or a milestone id, whichever the door takes — so the two registries substitute
/// into one vocabulary rather than two spellings of the same slot.
pub const WORK_UNIT_ID_SLOT: &str = "<id>";

/// The payload path the two `--from-file` rows of [`WORK_UNIT_ID_DOORS`] name.
///
/// Those rows must carry a `--from-file` that is *not itself* a fault, or the door would
/// answer about a missing file instead of about the id. The literal lives beside the
/// table rather than in the suite that runs it, so the argv a row declares and the file
/// the suite plants cannot drift apart.
pub const WORK_UNIT_ID_DOOR_PAYLOAD: &str = "payload.yaml";

/// **One door that takes a work-unit id** — a leaf verb, the argument the id arrives
/// through, and a runnable argv carrying [`WORK_UNIT_ID_SLOT`] where the id goes.
pub struct WorkUnitIdDoor {
    /// The leaf verb path, as an operator types it after `jigc`.
    pub door: &'static [&'static str],
    /// The clap argument id the work-unit id arrives through — a member of
    /// [`work_unit_id_arg_ids`], fenced against what `argv` actually parses into, so a
    /// renamed argument reddens rather than reading as documentation.
    pub arg: &'static str,
    /// A **runnable** argv for this door with [`WORK_UNIT_ID_SLOT`] standing in for the
    /// id — every other argument present and well-formed, so the id is the *only* thing
    /// the door can fault on.
    pub argv: &'static [&'static str],
}

/// **Every leaf verb that takes a work-unit id** — derived from the clap tree by
/// [`work_unit_id_arg_ids`] and fenced ⇔ against it
/// (`cli_parse::every_work_unit_id_door_is_registered`), so the set is the binary's, not
/// a remembered one.
///
/// Twenty-five doors. The guard that refuses a malformed id
/// (`crate::task::reject_malformed_work_unit_id`) sits at five *resolve seams*, and a
/// seam is not a door: the M50 baseline counted the seams by reading the source, which is
/// exactly the shape that has left a class half-swept in four waves running. This table
/// is the door side of that claim, and `crates/cli/tests/work_unit_id_axis.rs` drives
/// every row over the whole token axis.
pub const WORK_UNIT_ID_DOORS: &[WorkUnitIdDoor] = &[
    // Top level — the two composing doors, whose `--task <id>` resumes or re-enters.
    WorkUnitIdDoor {
        door: &["start"],
        arg: "task",
        argv: &["start", "--task", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["workflow"],
        arg: "task",
        argv: &["workflow", "single-task", "--task", WORK_UNIT_ID_SLOT],
    },
    // `jigc doc` — every managed-doc verb scopes to a task's working area.
    WorkUnitIdDoor {
        door: &["doc", "create"],
        arg: "task",
        argv: &[
            "doc",
            "create",
            "adr",
            "--title",
            "Axis",
            "--task",
            WORK_UNIT_ID_SLOT,
        ],
    },
    WorkUnitIdDoor {
        door: &["doc", "author"],
        arg: "task",
        argv: &[
            "doc",
            "author",
            "adr",
            "--from-file",
            WORK_UNIT_ID_DOOR_PAYLOAD,
            "--task",
            WORK_UNIT_ID_SLOT,
        ],
    },
    WorkUnitIdDoor {
        door: &["doc", "add-item"],
        arg: "task",
        argv: &[
            "doc",
            "add-item",
            "adr:axis#entries",
            "--title",
            "Axis",
            "--task",
            WORK_UNIT_ID_SLOT,
        ],
    },
    WorkUnitIdDoor {
        door: &["doc", "remove-item"],
        arg: "task",
        argv: &[
            "doc",
            "remove-item",
            "adr:axis#entries/one",
            "--task",
            WORK_UNIT_ID_SLOT,
        ],
    },
    WorkUnitIdDoor {
        door: &["doc", "retitle-item"],
        arg: "task",
        argv: &[
            "doc",
            "retitle-item",
            "adr:axis#entries/one",
            "--title",
            "Axis",
            "--task",
            WORK_UNIT_ID_SLOT,
        ],
    },
    WorkUnitIdDoor {
        door: &["doc", "rename"],
        arg: "task",
        argv: &[
            "doc",
            "rename",
            "adr:axis",
            "--to",
            "Axis",
            "--task",
            WORK_UNIT_ID_SLOT,
        ],
    },
    WorkUnitIdDoor {
        door: &["doc", "set-field"],
        arg: "task",
        argv: &[
            "doc",
            "set-field",
            "adr:axis#status",
            "--value",
            "accepted",
            "--task",
            WORK_UNIT_ID_SLOT,
        ],
    },
    WorkUnitIdDoor {
        door: &["doc", "set-slot"],
        arg: "task",
        argv: &[
            "doc",
            "set-slot",
            "adr:axis#context",
            "--from-file",
            WORK_UNIT_ID_DOOR_PAYLOAD,
            "--task",
            WORK_UNIT_ID_SLOT,
        ],
    },
    WorkUnitIdDoor {
        door: &["doc", "show"],
        arg: "task",
        argv: &["doc", "show", "adr:axis", "--task", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["doc", "list"],
        arg: "task",
        argv: &["doc", "list", "adr", "--task", WORK_UNIT_ID_SLOT],
    },
    // `jigc task` — the task lifecycle, where the id is the positional subject.
    WorkUnitIdDoor {
        door: &["task", "diff"],
        arg: "id",
        argv: &["task", "diff", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["task", "validate"],
        arg: "id",
        argv: &["task", "validate", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["task", "discard"],
        arg: "id",
        argv: &["task", "discard", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["task", "finalize"],
        arg: "id",
        argv: &["task", "finalize", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["task", "bind"],
        arg: "id",
        argv: &["task", "bind", "spec", "adr:axis", WORK_UNIT_ID_SLOT],
    },
    // `jigc milestone` — every verb that operates on an existing milestone. `create` is
    // absent because it MINTS its id from a title rather than taking one.
    WorkUnitIdDoor {
        door: &["milestone", "add-task"],
        arg: "milestone_id",
        argv: &["milestone", "add-task", WORK_UNIT_ID_SLOT, "Axis intent"],
    },
    WorkUnitIdDoor {
        door: &["milestone", "add-from-spec"],
        arg: "milestone_id",
        argv: &["milestone", "add-from-spec", WORK_UNIT_ID_SLOT, "spec:axis"],
    },
    WorkUnitIdDoor {
        door: &["milestone", "list-tasks"],
        arg: "milestone_id",
        argv: &["milestone", "list-tasks", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["milestone", "provision"],
        arg: "milestone_id",
        argv: &["milestone", "provision", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["milestone", "execute"],
        arg: "milestone_id",
        argv: &["milestone", "execute", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["milestone", "join"],
        arg: "milestone_id",
        argv: &["milestone", "join", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["milestone", "finalize"],
        arg: "milestone_id",
        argv: &["milestone", "finalize", WORK_UNIT_ID_SLOT],
    },
    WorkUnitIdDoor {
        door: &["milestone", "discard"],
        arg: "milestone_id",
        argv: &["milestone", "discard", WORK_UNIT_ID_SLOT],
    },
];

/// The token a [`SlugDoor`] row's `argv` carries **in place of** the `--slug` value, so
/// one row serves every cell of the axis instead of three hand-written argvs per door.
pub const SLUG_OVERRIDE_SLOT: &str = "<slug>";

/// The foreign source path the `jigc migrate` row of [`SLUG_DOORS`] names.
///
/// That row must carry a source that is *not itself* a fault, or the door could answer
/// about a missing file instead of about the override. The literal lives beside the table
/// rather than in the suite that runs it, so the argv a row declares and the file the
/// suite plants cannot drift apart — [`WORK_UNIT_ID_DOOR_PAYLOAD`]'s rule, one family
/// over.
pub const SLUG_DOOR_SOURCE: &str = "foreign-changelog.md";

/// **One door that takes a `--slug` override** — a leaf verb, the argument the override
/// arrives through, and a runnable argv carrying [`SLUG_OVERRIDE_SLOT`] where the value
/// goes.
pub struct SlugDoor {
    /// The leaf verb path, as an operator types it after `jigc`.
    pub door: &'static [&'static str],
    /// The clap argument id the override arrives through — a member of [`slug_arg_ids`],
    /// fenced against what `argv` actually parses into, so a renamed argument reddens
    /// rather than reading as documentation.
    pub arg: &'static str,
    /// A **runnable** argv for this door with [`SLUG_OVERRIDE_SLOT`] standing in for the
    /// override — every other argument present and well-formed, so the override is the
    /// only thing the door can fault on. The `doc` rows carry no `--task`: they run under
    /// the shipped single-active-task default, which the suite's fixture provides.
    pub argv: &'static [&'static str],
}

/// **Every leaf verb that takes a `--slug` override** — derived from the clap tree by
/// [`slug_arg_ids`] and fenced ⇔ against it (`cli_parse::every_slug_door_is_registered`),
/// so the set is the binary's, not a remembered one.
///
/// Six doors. Five of them refused a malformed override byte-identically from M39
/// onward; the sixth — `jigc rename` — did not, and driven at `b32def1`
/// `jigc rename adr:keeper --to "New Title" --slug '../../src/pwned'` exited **0** and
/// committed `docs/decisions/keeper.md => src/pwned.md`, after which no `doc list` row,
/// no `doc show` and no `validate` finding could name the doc again. `crates/cli/tests/
/// slug_override_axis.rs` drives every row over the whole malformed-token axis.
///
/// The `start` row is the **minting** form on purpose: driven, bare
/// `jigc start "<intent>" --slug '../../x'` exits 0 with the override inert, because the
/// cascade default composes the router and mints nothing — a row written that way would
/// pass while proving nothing (`DECISIONS.md` → 2026-09-05 M50 Increment 2 planning).
pub const SLUG_DOORS: &[SlugDoor] = &[
    SlugDoor {
        door: &["start"],
        arg: "slug",
        argv: &[
            "start",
            "--workflow",
            "single-task",
            "axis intent",
            "--slug",
            SLUG_OVERRIDE_SLOT,
        ],
    },
    SlugDoor {
        door: &["migrate"],
        arg: "slug",
        argv: &[
            "migrate",
            SLUG_DOOR_SOURCE,
            "--as",
            "changelog",
            "--slug",
            SLUG_OVERRIDE_SLOT,
        ],
    },
    SlugDoor {
        door: &["rename"],
        arg: "slug",
        argv: &[
            "rename",
            "adr:keeper",
            "--to",
            "Axis",
            "--slug",
            SLUG_OVERRIDE_SLOT,
        ],
    },
    SlugDoor {
        door: &["doc", "create"],
        arg: "slug",
        argv: &[
            "doc",
            "create",
            "adr",
            "--title",
            "Axis",
            "--slug",
            SLUG_OVERRIDE_SLOT,
        ],
    },
    SlugDoor {
        door: &["doc", "add-item"],
        arg: "slug",
        argv: &[
            "doc",
            "add-item",
            "adr:keeper#entries",
            "--title",
            "Axis",
            "--slug",
            SLUG_OVERRIDE_SLOT,
        ],
    },
    SlugDoor {
        door: &["doc", "rename"],
        arg: "slug",
        argv: &[
            "doc",
            "rename",
            "adr:keeper",
            "--to",
            "Axis",
            "--slug",
            SLUG_OVERRIDE_SLOT,
        ],
    },
];

/// The **per-component** byte ceiling a filename may not exceed. `NAME_MAX` is 255 on every
/// filesystem jigc supports (APFS, HFS+, ext4, btrfs, XFS, NTFS), and it bounds a single
/// path *component*, never the whole path.
///
/// **Two rules read it, and they take different subjects** (M52 Increment 6 / T6). The slug
/// family below reserves what jigc wraps around an identity on its way to becoming a
/// *filename*, which is why its subject is the slug and not the directory it lands in
/// ([`SLUG_NAME_CEILING`]); a **root knob**'s value becomes one or more *directory*
/// components, and nothing is appended to a directory name, so
/// `config::unusable_root_reason`'s nameability leg takes this constant bare.
///
/// Public so that leg's own suite (`crates/cli/tests/root_knob_rules.rs`) builds its cells
/// from the constant the refusal is derived from instead of restating 255 — M49's
/// *statement == constant* lesson, applied to the test as well as to the code.
pub const NAME_MAX_BYTES: usize = 255;

/// Decimal digits in `n` — the width its `Display` renders, used to reserve room for the
/// two numbers [`TEMP_SIBLING_RESERVE`] cannot know in advance.
const fn decimal_width(mut n: u128) -> usize {
    let mut width = 1;
    while n >= 10 {
        n /= 10;
        width += 1;
    }
    width
}

/// What a staged doc's **atomic write** appends to the filename before the `rename`:
/// `engine::state`'s temp sibling is `<filename>.<pid>.<nanos>.tmp`, in the target's own
/// directory. The two numbers are reserved at their **types'** widths — `std::process::id`
/// is a `u32`, and `engine::tempname::unique_nanos` is a `u64` reading widened to `u128` —
/// so a fatter pid or a clock further from the epoch cannot quietly eat the reserve.
///
/// This is the **longest** suffix any [`SLUG_DOORS`] row appends, and it is the one that
/// matters: the committed path's bare `.md` is shorter, and a task directory appends
/// nothing at all.
const TEMP_SIBLING_RESERVE: usize = ".".len()
    + decimal_width(u32::MAX as u128)
    + ".".len()
    + decimal_width(u64::MAX as u128)
    + ".tmp".len();

/// What every doc home appends: a managed instance is `<slug>.md` committed and
/// `<type>:<slug>.md` staged (`engine::store::canonical_path` / `engine::state`'s
/// `instance_filename`).
const DOC_EXTENSION_RESERVE: usize = ".md".len();

/// What a **staged** instance prepends: `<type>:`. Reserved at
/// [`engine::slug::MAX_CHARS`] — jigc's own stated filesystem-safety backstop for a minted
/// identifier — plus the `:` joiner, rather than at the length of any one doctype id, so
/// the reserve does not move when a pack ships a new doctype.
///
/// **Declared bound:** a doctype id is authored, not minted, so nothing *enforces*
/// `MAX_CHARS` on it. Measured at HEAD, the longest id either shipped pack defines is
/// `completion-record` — **17** bytes, a third of the reserve — so the budget holds for the
/// whole shipped population, and a project pack declaring a doctype id longer than
/// [`engine::slug::MAX_CHARS`] is the one shape that could still reach the OS ceiling.
const DOCTYPE_PREFIX_RESERVE: usize = engine::slug::MAX_CHARS + ":".len();

/// **The ceiling a `--slug` override may not exceed**, in bytes — one constant, read by the
/// predicate every [`SLUG_DOORS`] row runs (`crate::task::reject_slug_over_name_ceiling`).
///
/// Derived, never restated: it is [`NAME_MAX_BYTES`] less everything jigc wraps around a
/// minted id on its way to becoming one path component — the staged instance's `<type>:`
/// prefix, its `.md` extension, and the atomic write's temp suffix. Three literals at three
/// doors would re-enact the failure M45 named: a statement that is a *copy* of the constant
/// it describes drifts from it silently.
///
/// The value is **165**. The mint-time cap on a *derived* slug is
/// [`engine::slug::MAX_CHARS`] (50), which `--slug` deliberately bypasses (a migration task
/// id carries a `blake3` disambiguator past it), so this ceiling binds only the override —
/// and it refuses rather than truncating, because a truncated identity names a different
/// doc.
pub const SLUG_NAME_CEILING: usize =
    NAME_MAX_BYTES - DOCTYPE_PREFIX_RESERVE - DOC_EXTENSION_RESERVE - TEMP_SIBLING_RESERVE;

/// The clap argument ids whose value **becomes a path component** — [`ARG_TOKENS`]'
/// [`PlainValue::PathBearing`] projection, and the vocabulary [`PATH_ARG_OCCURRENCES`] is
/// fenced ⇔ against. Six: `path` · `file` · `from_file` · `from` · `target` · `value`.
pub fn path_arg_ids() -> Vec<&'static str> {
    ARG_TOKENS
        .iter()
        .filter(|(_, token)| matches!(token, ArgToken::Plain(PlainValue::PathBearing)))
        .map(|(id, _)| *id)
        .collect()
}

/// The token a [`PathArgArm`]'s `argv` carries **in place of** the caller-supplied path
/// token, so one arm serves every cell of the escape-shape axis instead of eight
/// hand-written argvs per occurrence — [`WORK_UNIT_ID_SLOT`]'s rule, one family over.
pub const PATH_ARG_SLOT: &str = "<path>";

/// **What shape the caller's token takes at one occurrence** — what the axis suite has to
/// substitute for [`PATH_ARG_SLOT`] to be asking that occurrence its own question.
///
/// It is part of the registry rather than of the suite because it is a statement about the
/// *argument*, not about a test: an occurrence whose subject is a **home** answers about a
/// directory, and handing it a file spelling would be asking it something else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathArgSubject {
    /// A **file jigc reads** (`jigc migrate <path>`, `jigc config insert-step <file>`,
    /// every `--from-file`), or — at `jigc unmanage` — a file it looks up by path.
    SourceFile,
    /// A **directory a home resolves to** (`jigc config set docs-root <value>`,
    /// `jigc relocate --from <prior-home>`).
    Home,
    /// The **`#` tail of an address** whose head is held valid, so the door answers about
    /// the tail — the component that would name `.jigc/config/steps/<id>.yaml` or
    /// `.jigc/config/fills/<id>.md`. Carries the head, verbatim.
    AddressTail(&'static str),
    /// A **field value**, adjudicated against the field's declared type.
    FieldValue,
}

/// **Which tokens one arm answers for** — `Caller` for an arm the caller's own spelling
/// reaches, `Literal` for an arm the door's declared grammar reserves for one exact token.
///
/// The one `Literal` today is `-`, the **stdin sentinel** of `--from-file`. It is not a
/// path and never becomes one, and it is exactly why this registry is keyed by
/// `(leaf, argument id, conditional arm)` rather than by argument id: the same argument at
/// the same door reads two different kinds of thing depending on the token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmToken {
    /// Any token the caller supplies — minus whatever a sibling `Literal` arm reserves.
    Caller,
    /// This exact token, and no other.
    Literal(&'static str),
}

/// **What one occurrence-arm does about its token** — one stated rule, or one stated
/// no-rule-and-why. A *typed* value, so a disposition cannot be a comment somebody deletes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathArgDisposition {
    /// The token is **adjudicated** before it reaches a filesystem op: the predicate that
    /// answers, and every blocking code the arm can earn.
    Adjudicated {
        /// The predicate asked, named so a reader can go and read it.
        predicate: &'static str,
        /// Every blocking finding code this arm refuses with — non-empty.
        codes: &'static [&'static str],
    },
    /// The token is **not** adjudicated as a path, and this is why it needs no rule. The
    /// reason is the row: a no-rule with no reason is the silence this registry exists to
    /// replace.
    NoRule {
        /// Why the token becomes no path component — the claim the axis suite drives.
        why: &'static str,
    },
}

/// **One arm of one occurrence** — the condition it answers under, the tokens it answers
/// for, a runnable argv, and its disposition.
pub struct PathArgArm {
    /// The condition this arm answers under, as a reader would state it
    /// (*"`key ∈ ROOT_KNOBS`"*, *"the value is `-`"*). Unique within its occurrence: it is
    /// the third component of the registry's key.
    pub when: &'static str,
    /// Which tokens reach this arm.
    pub token: ArmToken,
    /// What shape the token takes here.
    pub subject: PathArgSubject,
    /// A **runnable** argv for this arm with [`PATH_ARG_SLOT`] standing in for the caller
    /// token — every other argument present and well-formed, so the token is the only
    /// thing the door can fault on.
    pub argv: &'static [&'static str],
    /// The rule, or the stated absence of one.
    pub disposition: PathArgDisposition,
}

/// **One occurrence of a path-bearing argument** — a leaf verb, the argument, and its arms.
pub struct PathArgOccurrence {
    /// The leaf verb path, as an operator types it after `jigc`.
    pub door: &'static [&'static str],
    /// The clap argument id — a member of [`path_arg_ids`], fenced against what `argv`
    /// actually parses into, so a renamed argument reddens rather than reading as
    /// documentation.
    pub arg: &'static str,
    /// This occurrence's arms — at least one, each with its own `when`.
    pub arms: &'static [PathArgArm],
}

/// **Every occurrence of every path-bearing argument, with its rule** — keyed by
/// `(leaf, argument id, conditional arm)` and ⇔-fenced against the clap tree
/// (`cli_parse::every_path_arg_occurrence_is_registered`), driven whole by
/// `crates/cli/tests/path_arg_occurrence_axis.rs`.
///
/// **Why occurrences and not ids.** One argument id means different things at different
/// leaves, and a registry that deduplicated them would state one rule where two are owed:
/// `path` is `jigc migrate`'s **source to be read and retired** *and* `jigc unmanage`'s
/// **lookup key**; `file` occurs at `insert-step` and at `replace-step`; `value` is
/// path-like only when `key ∈ ROOT_KNOBS`; and `from_file` carries the `-` stdin sentinel,
/// which is not a path at all. Each of those is a separate row with its own answer
/// (`completions/artifacts/M51/settle-record.md` → D1 part 3, as amended by §2, Codex 6).
///
/// **What a row buys.** M50's audit found `DOCTYPE_DOORS` derived from a hand-kept
/// allowlist of argument *names*, which cannot see an argument nobody listed, and one
/// unseen argument read a file from outside the repository at exit 0. The answer was
/// totality: erase the complement, so a new argument reddens until someone answers it.
/// [`ArgToken::Plain`] was total over *ids* and silent about *path-ness* — and that silence
/// is where M51's own driven damage sat, at `jigc migrate <absolute-path>`. This registry
/// is the second half of that totality: an argument answered [`PlainValue::PathBearing`]
/// reddens until **every one of its occurrences** carries a rule or a stated no-rule, and
/// the axis suite reddens until every arm is driven over every escape shape.
pub const PATH_ARG_OCCURRENCES: &[PathArgOccurrence] = &[
    // ── `path` ────────────────────────────────────────────────────────────────────────
    PathArgOccurrence {
        door: &["migrate"],
        arg: "path",
        arms: &[PathArgArm {
            when: "always — the argument is the source `jigc migrate` reads and \
                   `jigc task finalize --approve` retires",
            token: ArmToken::Caller,
            subject: PathArgSubject::SourceFile,
            argv: &["migrate", PATH_ARG_SLOT, "--as", "changelog"],
            disposition: PathArgDisposition::Adjudicated {
                predicate: "crate::trackable::resolve_source_token (git pathspec magic — \
                            the `:` prefix and wildmatch alike — then resolve-or-refuse, then \
                            untrackable_reason · is_workbench_root · the symlink leg of \
                            unusable_root_reason), then the index-or-HEAD trackedness leg",
                codes: &["migrate.source-untrackable", "migrate.source-untracked"],
            },
        }],
    },
    PathArgOccurrence {
        door: &["unmanage"],
        arg: "path",
        arms: &[PathArgArm {
            when: "always",
            token: ArmToken::Caller,
            subject: PathArgSubject::SourceFile,
            argv: &["unmanage", PATH_ARG_SLOT],
            disposition: PathArgDisposition::NoRule {
                why: "the token is a LOOKUP KEY, matched against the spellings the store \
                      already recorded (the file-state baseline and the edge index) and \
                      never joined onto a path jigc reads, writes or unlinks — `unmanage` \
                      leaves the bytes on disk by definition, which is the whole verb. A \
                      spelling no record carries selects nothing and earns the idempotent \
                      no-op the verb documents.",
            },
        }],
    },
    // ── `from` ────────────────────────────────────────────────────────────────────────
    PathArgOccurrence {
        door: &["relocate"],
        arg: "from",
        arms: &[PathArgArm {
            when: "always",
            token: ArmToken::Caller,
            subject: PathArgSubject::Home,
            argv: &["relocate", "adr", "--from", PATH_ARG_SLOT],
            disposition: PathArgDisposition::NoRule {
                why: "the prior home is a PREFIX matched against the committed spellings \
                      `git ls-files` reports (`crate::orphan::is_stranded`) — never a path \
                      opened, written, or handed to git as a pathspec. A value that is not \
                      one of those spellings selects no instance, so the door relocates \
                      nothing. The DESTINATION is the schema's own home, and it is the \
                      destination M49's trackability rule guards.",
            },
        }],
    },
    // ── `file` ────────────────────────────────────────────────────────────────────────
    PathArgOccurrence {
        door: &["config", "insert-step"],
        arg: "file",
        arms: &[PathArgArm {
            when: "always — `-` is read as a file name here, not as stdin",
            token: ArmToken::Caller,
            subject: PathArgSubject::SourceFile,
            argv: &[
                "config",
                "insert-step",
                "--workflow",
                "single-task",
                "--after",
                "implement",
                PATH_ARG_SLOT,
            ],
            disposition: PathArgDisposition::Adjudicated {
                predicate: "crate::trackable::source_read_reason (a SOURCE rule: no `.git` \
                            component, not reached through jigc's transient workbench — an \
                            out-of-repo source is admitted and copied in)",
                codes: &["config.step-source-untrackable"],
            },
        }],
    },
    PathArgOccurrence {
        door: &["config", "replace-step"],
        arg: "file",
        arms: &[PathArgArm {
            when: "always — `-` is read as a file name here, not as stdin",
            token: ArmToken::Caller,
            subject: PathArgSubject::SourceFile,
            argv: &[
                "config",
                "replace-step",
                "workflow:single-task#locate",
                PATH_ARG_SLOT,
            ],
            disposition: PathArgDisposition::Adjudicated {
                predicate: "crate::trackable::source_read_reason — the same predicate as the \
                            `insert-step` occurrence, which is why this registry is keyed by \
                            occurrence: one home, two rows",
                codes: &["config.step-source-untrackable"],
            },
        }],
    },
    // ── `from_file` ───────────────────────────────────────────────────────────────────
    PathArgOccurrence {
        door: &["config", "fill"],
        arg: "from_file",
        arms: &[
            PathArgArm {
                when: "the value is `-`",
                token: ArmToken::Literal("-"),
                subject: PathArgSubject::SourceFile,
                argv: &[
                    "config",
                    "fill",
                    "step:implement#extra-guidance",
                    "--from-file",
                    PATH_ARG_SLOT,
                ],
                disposition: PathArgDisposition::NoRule {
                    why: "the declared stdin sentinel is not a path — the content arrives on \
                          the stream and no token reaches the filesystem at all.",
                },
            },
            PathArgArm {
                when: "the value is a path",
                token: ArmToken::Caller,
                subject: PathArgSubject::SourceFile,
                argv: &[
                    "config",
                    "fill",
                    "step:implement#extra-guidance",
                    "--from-file",
                    PATH_ARG_SLOT,
                ],
                disposition: PathArgDisposition::NoRule {
                    why: "stated once, in full, at `crate::doc::read_handoff`: the door's own \
                          declared grammar hands the identical bytes through `-`, so a path \
                          rule here would refuse a SPELLING and not an outcome — a fence the \
                          door's `--help` defeats is a false completeness claim. The \
                          destination is a declared `{{fill:}}` point and the content lands \
                          in a file the next diff shows.",
                },
            },
        ],
    },
    PathArgOccurrence {
        door: &["doc", "set-slot"],
        arg: "from_file",
        arms: &[
            PathArgArm {
                when: "the value is `-`",
                token: ArmToken::Literal("-"),
                subject: PathArgSubject::SourceFile,
                argv: &[
                    "doc",
                    "set-slot",
                    "adr:probe#context",
                    "--from-file",
                    PATH_ARG_SLOT,
                ],
                disposition: PathArgDisposition::NoRule {
                    why: "the declared stdin sentinel is not a path — the prose arrives on the \
                          stream and no token reaches the filesystem at all.",
                },
            },
            PathArgArm {
                when: "the value is a path",
                token: ArmToken::Caller,
                subject: PathArgSubject::SourceFile,
                argv: &[
                    "doc",
                    "set-slot",
                    "adr:probe#context",
                    "--from-file",
                    PATH_ARG_SLOT,
                ],
                disposition: PathArgDisposition::NoRule {
                    why: "stated once, in full, at `crate::doc::read_handoff` — the bytes are \
                          read as SLOT PROSE into a schema-declared slot, readable back \
                          through `jigc doc show … --task <id>`, and `-` hands the same bytes \
                          through the same door.",
                },
            },
        ],
    },
    PathArgOccurrence {
        door: &["doc", "author"],
        arg: "from_file",
        arms: &[
            PathArgArm {
                when: "the value is `-`",
                token: ArmToken::Literal("-"),
                subject: PathArgSubject::SourceFile,
                argv: &["doc", "author", "adr", "--from-file", PATH_ARG_SLOT],
                disposition: PathArgDisposition::NoRule {
                    why: "the declared stdin sentinel is not a path — the payload arrives on \
                          the stream and no token reaches the filesystem at all.",
                },
            },
            PathArgArm {
                when: "the value is a path",
                token: ArmToken::Caller,
                subject: PathArgSubject::SourceFile,
                argv: &["doc", "author", "adr", "--from-file", PATH_ARG_SLOT],
                disposition: PathArgDisposition::NoRule {
                    why: "stated once, in full, at `crate::doc::read_handoff` — the bytes are \
                          read as a declarative PAYLOAD and every leaf it writes is \
                          schema-declared, and `-` hands the same bytes through the same door.",
                },
            },
        ],
    },
    // ── `target` ──────────────────────────────────────────────────────────────────────
    PathArgOccurrence {
        door: &["config", "replace-step"],
        arg: "target",
        arms: &[PathArgArm {
            when: "always",
            token: ArmToken::Caller,
            subject: PathArgSubject::AddressTail("workflow:single-task#"),
            argv: &[
                "config",
                "replace-step",
                PATH_ARG_SLOT,
                "replacement-step.yaml",
            ],
            disposition: PathArgDisposition::NoRule {
                why: "the target names the include-list POSITION being swapped, and the native \
                      file's id is the SOURCE FILE's basename — so this token names no path \
                      component at any spelling. It must also resolve in the workflow's \
                      resolved include list before any write (`config.anchor-absent`).",
            },
        }],
    },
    PathArgOccurrence {
        door: &["config", "remove-step"],
        arg: "target",
        arms: &[PathArgArm {
            when: "always",
            token: ArmToken::Caller,
            subject: PathArgSubject::AddressTail("workflow:single-task#"),
            argv: &["config", "remove-step", PATH_ARG_SLOT],
            disposition: PathArgDisposition::NoRule {
                why: "the verb writes no native file at all — there is nothing to add — so the \
                      token names no path component; and the position must resolve in the \
                      workflow's resolved include list before the delta is recorded \
                      (`config.anchor-absent`).",
            },
        }],
    },
    PathArgOccurrence {
        door: &["config", "fill"],
        arg: "target",
        arms: &[PathArgArm {
            when: "always",
            token: ArmToken::Caller,
            subject: PathArgSubject::AddressTail("step:implement#"),
            argv: &["config", "fill", PATH_ARG_SLOT, "--from-file", "-"],
            disposition: PathArgDisposition::NoRule {
                why: "the `#<fill-id>` tail DOES name `.jigc/config/fills/<fill-id>.md` — but \
                      only a tail that `check_fill_point_present` has resolved against the \
                      `{{fill:<id>}}` points the RESOLVED STEP BODY declares, a closed \
                      vocabulary the pack authors and no caller extends. A tail the body does \
                      not declare is refused before any write (`config.fill-point-absent`), so \
                      no caller spelling reaches the write.",
            },
        }],
    },
    PathArgOccurrence {
        door: &["config", "fork"],
        arg: "target",
        arms: &[PathArgArm {
            when: "always",
            token: ArmToken::Caller,
            subject: PathArgSubject::AddressTail("workflow:single-task#"),
            argv: &["config", "fork", PATH_ARG_SLOT],
            disposition: PathArgDisposition::NoRule {
                why: "the `#<step-id>` tail DOES name `.jigc/config/steps/<step-id>.yaml` — but \
                      only a tail `check_anchor_present` has resolved against the workflow's \
                      resolved include list, a closed vocabulary no caller extends. An id the \
                      list does not carry is refused before any write \
                      (`config.anchor-absent`).",
            },
        }],
    },
    // ── `value` ───────────────────────────────────────────────────────────────────────
    PathArgOccurrence {
        door: &["config", "set"],
        arg: "value",
        arms: &[
            PathArgArm {
                when: "`key ∈ ROOT_KNOBS` — the value is resolved as a HOME",
                token: ArmToken::Caller,
                subject: PathArgSubject::Home,
                argv: &["config", "set", "docs-root", PATH_ARG_SLOT],
                disposition: PathArgDisposition::Adjudicated {
                    predicate: "crate::trackable::untrackable_reason · \
                                crate::config::is_workbench_root · \
                                crate::config::unusable_root_reason (absolute · edge \
                                whitespace · git pathspec magic · unnameable component · \
                                symlinked or file-shaped component)",
                    codes: &[
                        "config.untrackable-root",
                        "config.workbench-root",
                        "config.unusable-root",
                    ],
                },
            },
            PathArgArm {
                when: "`key ∉ ROOT_KNOBS`",
                token: ArmToken::Caller,
                subject: PathArgSubject::FieldValue,
                argv: &["config", "set", "default-workflow", PATH_ARG_SLOT],
                disposition: PathArgDisposition::NoRule {
                    why: "a non-root knob's value is a SCALAR, adjudicated against the knob's \
                          declared type before it lands (`config.value-rejected`) and read back \
                          by the resolver as that scalar. Only `ROOT_KNOBS`' values are \
                          resolved as a home, which is why the two arms of one argument carry \
                          two different answers.",
                },
            },
        ],
    },
    PathArgOccurrence {
        door: &["doc", "set-field"],
        arg: "value",
        arms: &[PathArgArm {
            when: "always",
            token: ArmToken::Caller,
            subject: PathArgSubject::FieldValue,
            argv: &[
                "doc",
                "set-field",
                "commit:probe#scope",
                "--value",
                PATH_ARG_SLOT,
            ],
            disposition: PathArgDisposition::NoRule {
                why: "the value is adjudicated against the FIELD'S DECLARED TYPE and lands as \
                      document content — a `string` field's value is prose on a line. The two \
                      types whose values name something outside the document carry their own \
                      rules, neither of them this argument's: a `ref` resolves through the \
                      address grammar M50 fenced, and an `owned-location` is adjudicated whole \
                      by the owner-artifact gate (absolute · `..` · home confinement · symlink \
                      escape · presence · trackedness).",
            },
        }],
    },
];

/// The [`VerbKind`] of a leaf verb path — `None` for a path that names no leaf verb
/// (a parent node, an unknown token, clap's builtin `help`).
pub fn verb_kind<S: AsRef<str>>(path: &[S]) -> Option<VerbKind> {
    VERB_KINDS
        .iter()
        .find(|(known, _)| {
            known.len() == path.len()
                && known
                    .iter()
                    .zip(path)
                    .all(|(known, token)| *known == token.as_ref())
        })
        .map(|(_, kind)| *kind)
}

/// The guessed tokens that **mean "show me"** — the read-intent lexicon.
///
/// This is a lexicon of English, not an enumeration of jigc's surface: it decides only
/// whether the *guess* asked to see something. What may be offered back is decided by
/// [`VERB_KINDS`] over the whole clap tree, so widening the surface can never re-open
/// the law-1 hole this rule closes.
pub const READ_INTENT_GUESSES: &[&str] = &[
    "read", "cat", "view", "show", "get", "list", "ls", "print", "display", "dump", "info",
    "inspect", "head", "status", "peek", "open",
];

/// One parent node's **read answer**: the tip a read-shaped miss under that parent earns
/// when no near sibling of clap's reads.
///
/// Fenced total over the clap tree's parent nodes, the root included
/// (`cli_parse::every_parent_node_carries_a_read_answer`), so a new verb *group* cannot
/// ship without an answer to "how do I see what is there?" — and every command span in
/// every answer is asserted [`VerbKind::Read`] by that same fence.
pub struct ReadAnswer {
    /// The parent node's argv path (`&[]` is the root — `jigc <guess>`).
    pub parent: &'static [&'static str],
    /// Builds the answer's tip text (leading `tip: `).
    pub tip: fn() -> String,
}

/// The per-parent read answers — see [`ReadAnswer`].
pub const PARENT_READ_ANSWERS: &[ReadAnswer] = &[
    ReadAnswer {
        parent: &[],
        tip: tip_root_read_shaped,
    },
    ReadAnswer {
        parent: &["doc"],
        tip: tip_doc_read_shaped,
    },
    ReadAnswer {
        parent: &["task"],
        tip: tip_task_read_shaped,
    },
    ReadAnswer {
        parent: &["config"],
        tip: tip_config_read_shaped,
    },
    ReadAnswer {
        parent: &["milestone"],
        tip: tip_milestone_read_shaped,
    },
];

/// One curated row of the unknown-subcommand table: the `(parent, guess)` key an agent
/// actually reached for in a trial, and the honest tip it earns.
///
/// `tip` is a **function**, not a literal, because every command span inside it rides
/// the checked [`Route::mechanical`](engine::finding::Route) constructor — built at
/// call time, under the live parse fence.
pub struct SiblingTip {
    /// The parent verb the guess sat under (`jigc <parent> <guess>`).
    pub parent: &'static str,
    /// The guessed verb — one that does not exist.
    pub guess: &'static str,
    /// Builds this row's tip text (leading `tip: `).
    pub tip: fn() -> String,
}

/// The curated unknown-subcommand table — **the axis every fence over these tips
/// iterates**, rather than a list re-typed at each fence (M48 Inc 6 T2; the
/// receipt-outlives-contract failure the wave's razor guards against).
///
/// Keyed on `(parent, guessed verb)` — the guesses agents actually reached for in the
/// trials — each row supplies a tip naming what the real sibling **does** (its effect),
/// because clap's bare did-you-mean can steer wrong: a `task discard-write` guesser
/// suggested toward `discard` would destroy the whole task. Where a row matches, jigc's
/// own render of the block drops clap's suggestion and carries this tip in its place
/// ([`unknown_subcommand_block`]); everywhere else clap's suggestion stands verbatim.
pub const CURATED_SIBLING_TIPS: &[SiblingTip] = &[
    SiblingTip {
        parent: "task",
        guess: "discard-write",
        tip: tip_task_discard_write,
    },
    SiblingTip {
        parent: "task",
        guess: "status",
        tip: tip_task_read_shaped,
    },
    SiblingTip {
        parent: "doc",
        guess: "read",
        tip: tip_doc_read_shaped,
    },
    SiblingTip {
        parent: "doc",
        guess: "get",
        tip: tip_doc_read_shaped,
    },
    SiblingTip {
        parent: "config",
        guess: "show",
        tip: tip_config_read_shaped,
    },
];

/// The ghost verb the reconciliation route used to name: clap's did-you-mean steers to
/// `discard`, which abandons the whole task — so the tip states that effect out loud.
fn tip_task_discard_write() -> String {
    format!(
        "tip: no per-write discard exists — {}",
        engine::finding::Route::mechanical(
            ["jigc", "task", "discard", "<task-id>", "--force"],
            " abandons the WHOLE task (removes its working area and every staged \
             write); to back out a single external edit, revert that file on disk \
             instead",
        )
        .as_str()
    )
}

/// The `task` parent's read answer (the `status` guess among them): name what each real
/// sibling *does*, not a bare did-you-mean.
fn tip_task_read_shaped() -> String {
    format!(
        "tip: {}; {}",
        engine::finding::Route::mechanical(
            ["jigc", "task", "list"],
            " enumerates the active tasks (id + minting workflow + intent)",
        )
        .as_str(),
        engine::finding::Route::mechanical(
            ["jigc", "task", "validate", "<task-id>"],
            " previews part of the finalize gate for one task — the repository posture \
             finalize refuses under, content findings, carryover, the owner-artifact \
             causes that need no staging, and the granted-but-unused changelog gate",
        )
        .as_str(),
    )
}

/// The read-shaped `doc` guesses (`read` / `get`): both ask to *see* a managed doc, and
/// both must be answered with the read surface. clap answered `doc read` with `tip: some
/// similar subcommands exist: 'create', 'rename'` — a read intent steered at two writes,
/// the plainest law-1 failure the surface can make — and `doc get` with nothing at all.
///
/// The enumerating read leads because it is placeholder-free (an agent that guessed the
/// verb rarely holds an address yet), and the addressed read follows.
fn tip_doc_read_shaped() -> String {
    format!(
        "tip: reading a managed doc is its own verb — {}; {}",
        engine::finding::Route::mechanical(
            ["jigc", "doc", "list"],
            " enumerates the managed docs (identity, repo path, registration state)",
        )
        .as_str(),
        engine::finding::Route::mechanical(
            ["jigc", "doc", "show", "<address>"],
            " prints one committed doc, or the addressed slice of it, on stdout \
             (--format json for the pinned machine shape; --task <task-id> to read a \
             task's staged copy)",
        )
        .as_str(),
    )
}

/// The read-shaped `config` guess (`show`): it asks what the cascade currently resolves
/// to. clap found no near sibling and said nothing, so the surface hid the read rung it
/// has. Both spans are read verbs; the enumerating one leads (a guesser at `config show`
/// wants the whole surface, and it needs no key to be runnable).
fn tip_config_read_shaped() -> String {
    format!(
        "tip: {}; {}",
        engine::finding::Route::mechanical(
            ["jigc", "config", "list"],
            " prints every declared knob with its resolved value and the cascade layer \
             that won it",
        )
        .as_str(),
        engine::finding::Route::mechanical(
            ["jigc", "config", "get", "<key>"],
            " reads one knob by name",
        )
        .as_str(),
    )
}

/// The **root**'s read answer: a read-shaped guess with no parent (`jigc read`, `jigc
/// cat`) asked to see the project, and clap's nearest siblings are `relocate`/`rename` —
/// two writes, one of them the identity-mutating verb. The docs are what a read intent at
/// this level is after, so the two doc reads lead and the tour follows.
fn tip_root_read_shaped() -> String {
    format!(
        "tip: reading is its own verb — {}; {}; {}",
        engine::finding::Route::mechanical(
            ["jigc", "doc", "list"],
            " enumerates the managed docs (identity, repo path, registration state)",
        )
        .as_str(),
        engine::finding::Route::mechanical(
            ["jigc", "doc", "show", "<address>"],
            " prints one committed doc, or the addressed slice of it, on stdout",
        )
        .as_str(),
        engine::finding::Route::mechanical(
            ["jigc", "describe"],
            " tours the workflows and doc-types the resolved cascade offers",
        )
        .as_str(),
    )
}

/// The `milestone` parent's read answer: the one milestone verb that only reports.
fn tip_milestone_read_shaped() -> String {
    format!(
        "tip: {}; {}",
        engine::finding::Route::mechanical(
            ["jigc", "milestone", "list-tasks", "<milestone-id>"],
            " emits a milestone's sub-task ids in the canonical id-sorted order",
        )
        .as_str(),
        engine::finding::Route::mechanical(
            ["jigc", "doc", "show", "<address>"],
            " reads the milestone's own committed record (`milestone-record:<slug>`)",
        )
        .as_str(),
    )
}

/// The honest sibling tip for an unknown-subcommand **semantic guess** — never a
/// silent alias (M43 law 2, `DECISIONS.md` 2026-07-16 Settle, cross-cutting; trial
/// provenance: A1 papercut, log rec 254).
///
/// Looks the guess up in [`CURATED_SIBLING_TIPS`]. The tip never dispatches anything —
/// the guess stays a genuine usage error (exit 2, the block still says what was wrong
/// and prints the parent's usage). An uncurated guess (or a curated verb under the
/// wrong parent) returns `None`: inert, never an error.
///
/// `argv` is the raw process argv; the parent is the token immediately preceding the
/// guessed verb, so the map fires only where the guess actually sat (a global flag
/// between parent and guess misses the tip — best-effort by design, the clap error
/// still prints).
pub fn unknown_subcommand_tip(err: &clap::Error, argv: &[String]) -> Option<String> {
    let guess = invalid_subcommand_guess(err)?;
    let parent = argv
        .windows(2)
        .find(|pair| pair[1] == guess)
        .map(|pair| pair[0].as_str())?;
    CURATED_SIBLING_TIPS
        .iter()
        .find(|row| row.parent == parent && row.guess == guess)
        .map(|row| (row.tip)())
}

/// jigc's own render of the **unknown-subcommand** error block — `None` for every other
/// clap error kind, which keeps clap's own render untouched (`DECISIONS.md` 2026-08-13
/// the Settle, F8 part 3).
///
/// **Why jigc renders this one kind.** clap's block is emitted whole before anything of
/// jigc's can speak, so a curated tip could only *append* — and for the curated rows the
/// two contradict: `jigc task discard-write` printed clap's `tip: a similar subcommand
/// exists: 'discard'` directly above jigc's warning that `discard` abandons the whole
/// task, and a read-shaped miss is answered with a write verb. A surface that prints the
/// correction below the lie has not removed the lie (`design/surface-contract.md` → law
/// 1). So for this kind jigc composes the block from the error's own context — the same
/// four parts in the same order, plain text on stderr at the same exit 2
/// ([`crate::task::EXIT_USAGE`]) — and the **tip slot** carries the curated tip where a
/// row matches, the parent's read answer where the guess asked to *see* something, and
/// clap's own did-you-mean otherwise. did-you-mean is dropped only where one of those
/// two replaces it, never globally.
///
/// **The read-intent layer is the axis, not a curated pair.** A curated `(parent, guess)`
/// table can only answer the misses a trial happened to record: at M48 Inc 6's first HEAD
/// `jigc doc read` was repaired while `jigc doc cat` still drew `'create'` and `jigc read`
/// still drew `'relocate', 'rename'` — the identical law-1 lie one uncurated key away. So
/// when the guess is a [`READ_INTENT_GUESSES`] token, clap's suggestions are classified
/// against [`VERB_KINDS`] and every **write** verb is dropped; whatever reads is kept, and
/// if nothing does, the parent's [`ReadAnswer`] takes the slot. A read intent therefore
/// cannot reach clap's did-you-mean at all, and both tables are fenced against the clap
/// tree — so a verb added anywhere joins the rule the day it lands.
///
/// The parts, all read from the error clap already built (never re-derived): the guessed
/// token (`ContextKind::InvalidSubcommand`), clap's suggestion list
/// (`ContextKind::SuggestedSubcommand` — absent when clap found no near sibling, and
/// then no tip slot is printed at all), and the parent's usage (`ContextKind::Usage`).
pub fn unknown_subcommand_block(err: &clap::Error, argv: &[String]) -> Option<String> {
    use clap::error::{ContextKind, ContextValue};

    let guess = invalid_subcommand_guess(err)?;
    let mut block = format!("error: unrecognized subcommand '{guess}'\n");
    let suggested = match err.get(ContextKind::SuggestedSubcommand) {
        Some(ContextValue::Strings(near)) => near.clone(),
        _ => Vec::new(),
    };
    let tip = match unknown_subcommand_tip(err, argv) {
        Some(curated) => Some(curated),
        None if READ_INTENT_GUESSES.contains(&guess) => {
            read_intent_tip(&parent_path(argv, guess), &suggested)
        }
        None if suggested.is_empty() => None,
        None => Some(did_you_mean(&suggested)),
    };
    if let Some(tip) = tip {
        block.push_str(&format!("\n  {tip}\n"));
    }
    if let Some(ContextValue::StyledStr(usage)) = err.get(ContextKind::Usage) {
        block.push_str(&format!("\n{usage}\n"));
    }
    block.push_str("\nFor more information, try '--help'.\n");
    Some(block)
}

/// The argument shape a [`ForeclosedArgumentTip`] row answers.
pub enum ForeclosedArgument {
    /// **Any** positional typed at this node — the node declares none at all, so every
    /// positional reaching it is the one foreclosed form.
    Positional,
    /// One named flag the node does not declare (`--task` at `jigc rename`).
    Flag(&'static str),
}

/// A leaf verb whose **foreclosed argument form** already has an answer on record: the
/// node's argv path below `jigc`, the argument shape that reaches the row, an argv that
/// produces it, and the tip that names the answer.
pub struct ForeclosedArgumentTip {
    /// The leaf node the argument was typed at (`["describe"]` for `jigc describe adr`).
    pub node: &'static [&'static str],
    /// The argument shape this row answers.
    pub arg: ForeclosedArgument,
    /// An argv (after the binary name) that reaches this row — driven through the real
    /// binary by `crates/cli/tests/clap_error_kind_axis.rs`, so a row whose node has since
    /// grown the argument stops erroring and fails there.
    pub probe: &'static [&'static str],
    /// Builds the tip served at that node.
    pub tip: fn() -> String,
}

/// The curated set — two rows, by derivation rather than by convenience, one per
/// [`ForeclosedArgument`] shape.
///
/// **The positional row.** Nine leaf verbs take no positional at all (`task list`, `config
/// list`, `setup`, `uninstall`, `upgrade`, `ingest`, `migrate-corpus`, `describe`,
/// `validate`), and `describe` is the only one whose own definition **records the answer to
/// the positional form**: [`Command::Describe`]'s doc states the single-item form is not
/// built and names what answers it instead. The eight siblings record no such answer — the
/// nearest, `validate`'s "distinct from `jigc task validate <id>`", is a two-verb
/// disambiguation, not a recovery for a typed positional — so law 2 leaves them with
/// nothing to name and clap's own render stands.
///
/// **The flag row** (M49, S-4). A rejected flag earns a row only where two things hold at
/// once: the node does not declare it, and the surface records a **sibling form that does
/// take it**, so the tip routes somewhere that runs. Measured over the whole tree by
/// reading each leaf's own help: twelve leaves declare `--task` and thirty-five do not,
/// and exactly three definitions name a verb across that line at all — `jigc doc rename` →
/// `jigc rename`, `jigc doc list` → `jigc ingest`/`jigc migrate`, and `jigc describe` →
/// `jigc start`. Only the first is a **task-scope** split: [`crate::doc::DocCommand::Rename`]
/// is defined as *"the in-task sibling of the top-level `jigc rename` (which is task-less
/// and self-committing, and refuses outright while any task is in flight)"*. The other two
/// fail the second leg rather than the first — `doc list`'s pair routes an unregistered
/// doc, and `start --task` resumes a task rather than answering a menu — as does the
/// nearest positional-side near-miss, `validate`'s "distinct from `jigc task validate
/// <id>`", whose sibling takes its id as a positional and would reject `--task` in turn.
pub const FORECLOSED_ARGUMENT_TIPS: &[ForeclosedArgumentTip] = &[
    ForeclosedArgumentTip {
        node: &["describe"],
        arg: ForeclosedArgument::Positional,
        probe: &["describe", "adr"],
        tip: tip_describe_positional,
    },
    ForeclosedArgumentTip {
        node: &["rename"],
        arg: ForeclosedArgument::Flag("--task"),
        probe: &["rename", "adr:x", "--to", "A New Title", "--task", "t1"],
        tip: tip_rename_task_flag,
    },
];

/// The answer `describe`'s foreclosed single-item form has carried on record since M11:
/// the kind filters narrow the menu, and `jigc start --explain` is the resolution trace
/// (the two affordances [`Commands::Describe`]'s own doc names — said, now, where the
/// refusal actually happens).
fn tip_describe_positional() -> String {
    format!(
        "tip: describe is the whole menu — the single-item form is not built. Narrow it by \
         kind: {}; {}; {}. For how one workflow resolves, {}",
        engine::finding::Route::mechanical(
            ["jigc", "describe", "--workflows"],
            " tours the workflows alone",
        )
        .as_str(),
        engine::finding::Route::mechanical(["jigc", "describe", "--doctypes"], " the doc-types")
            .as_str(),
        engine::finding::Route::mechanical(["jigc", "describe", "--commands"], " the command refs")
            .as_str(),
        engine::finding::Route::mechanical(
            ["jigc", "start", "--explain"],
            " is the resolution trace",
        )
        .as_str(),
    )
}

/// The answer the identity split records for a `--task` typed at the top-level
/// `jigc rename` (M49, S-4).
///
/// `jigc rename` is the **committed-store** identity refactor: it is task-less and
/// self-committing, and refuses outright while any task is in flight. So a `--task` here
/// is an agent reaching for its in-task sibling, and clap's own answer — *"tip: to pass
/// `--task` as a value, use `-- --task`"* — is a misdirection twice over: the node's only
/// positional is already filled by the address, and no value form of `--task` exists at
/// this node in any case. The recovery is on record at
/// [`crate::doc::DocCommand::Rename`]'s own definition; law 2 puts it where the state is
/// produced.
fn tip_rename_task_flag() -> String {
    format!(
        "tip: `jigc rename` is the committed-store identity refactor — task-less and \
         self-committing, and it refuses while any task is in flight, so it takes no \
         `--task`. The in-task title change is its sibling: {}",
        engine::finding::Route::mechanical(
            [
                "jigc",
                "doc",
                "rename",
                "<address>",
                "--to",
                "<title>",
                "--task",
                "<task-id>",
            ],
            " retitles the doc your task has staged, and re-slugs it while its identity \
             is still uncommitted",
        )
        .as_str(),
    )
}

/// jigc's own render of an **unexpected argument** typed at a leaf whose foreclosed form
/// has a recorded answer — `None` for every other clap error, and for every rejected
/// argument [`FORECLOSED_ARGUMENT_TIPS`] carries no row for, which keeps clap's own render
/// untouched.
///
/// Same reasoning, shape and stream as [`unknown_subcommand_block`]: clap emits its block
/// whole before anything of jigc's can speak, so a tip could only be appended *below* the
/// bare refusal, and `design/surface-contract.md` → law 2 asks the surface that **produces**
/// the state to name the designated recovery — not a later one the reader has to go find.
/// `jigc describe adr` was a bare exit 2 while [`Command::Describe`] recorded the answer
/// three lines away (`completions/artifacts/M46/razor-ledger.md` §1, S-2), and
/// `jigc rename <type>:<slug> --to X --task <id>` drew clap's *"to pass `--task` as a
/// value, use `-- --task`"* while naming `jigc doc rename` nowhere (M49, S-4).
///
/// Neither foreclosed form is **built** here — `describe`'s single-item lookup stays
/// foreclosed (`DECISIONS.md` → 2026-08-13 the M48 Settle) and `rename` stays task-less:
/// this names the answer, and the run still fails at [`crate::task::EXIT_USAGE`].
///
/// The parts are read from the error clap already built, never re-derived: the rejected
/// token (`ContextKind::InvalidArg`) and the node's usage (`ContextKind::Usage`).
pub fn unexpected_argument_block(err: &clap::Error, argv: &[String]) -> Option<String> {
    use clap::error::{ContextKind, ContextValue};

    if err.kind() != clap::error::ErrorKind::UnknownArgument {
        return None;
    }
    let token = match err.get(ContextKind::InvalidArg)? {
        ContextValue::String(token) => token.clone(),
        _ => return None,
    };
    let node = parent_path(argv, &token);
    let row = FORECLOSED_ARGUMENT_TIPS.iter().find(|row| {
        row.node.len() == node.len()
            && row.node.iter().zip(&node).all(|(a, b)| *a == b.as_str())
            && match row.arg {
                ForeclosedArgument::Positional => !token.starts_with('-'),
                ForeclosedArgument::Flag(flag) => token == flag,
            }
    })?;
    let mut block = format!("error: unexpected argument '{token}' found\n");
    block.push_str(&format!("\n  {}\n", (row.tip)()));
    if let Some(ContextValue::StyledStr(usage)) = err.get(ContextKind::Usage) {
        block.push_str(&format!("\n{usage}\n"));
    }
    block.push_str("\nFor more information, try '--help'.\n");
    Some(block)
}

/// The **parent node's argv path** the guess was typed under — `[]` for a top-level
/// guess (`jigc read`), `["doc"]` for `jigc doc cat`.
///
/// Walked down the real clap tree rather than read off the token before the guess, so a
/// global flag and its value (`jigc --format json doc cat`) cannot be mistaken for the
/// parent. Stops at the guess itself; an unresolvable argv yields the deepest node it
/// did resolve.
fn parent_path(argv: &[String], guess: &str) -> Vec<String> {
    use clap::CommandFactory;

    let mut cmd = Cli::command();
    let mut path = Vec::new();
    for token in argv.iter().skip(1) {
        if token == guess {
            break;
        }
        if let Some(sub) = cmd.find_subcommand(token).cloned() {
            path.push(token.clone());
            cmd = sub;
        }
    }
    path
}

/// The tip a **read-shaped** guess earns: clap's near siblings minus every write verb,
/// and — when that leaves nothing — the parent's own [`ReadAnswer`].
///
/// `None` only when the parent resolves to no node carrying an answer, and even then the
/// caller prints no tip rather than falling back to a suggestion that might act: a read
/// intent is answered with a read verb or with silence, never with a write verb.
fn read_intent_tip(parent: &[String], suggested: &[String]) -> Option<String> {
    let readable: Vec<String> = suggested
        .iter()
        .filter(|name| {
            let mut path = parent.to_vec();
            path.push((*name).clone());
            verb_kind(&path) == Some(VerbKind::Read)
        })
        .cloned()
        .collect();
    if !readable.is_empty() {
        return Some(did_you_mean(&readable));
    }
    PARENT_READ_ANSWERS
        .iter()
        .find(|answer| {
            answer.parent.len() == parent.len() && answer.parent.iter().eq(parent.iter())
        })
        .map(|answer| (answer.tip)())
}

/// clap's own did-you-mean wording for a suggestion list, re-emitted verbatim so an
/// **uncurated** guess loses nothing to the render takeover (law 2 — nothing hides).
fn did_you_mean(near: &[String]) -> String {
    let opening = if near.len() == 1 {
        "tip: a similar subcommand exists: "
    } else {
        "tip: some similar subcommands exist: "
    };
    let quoted: Vec<String> = near.iter().map(|name| format!("'{name}'")).collect();
    format!("{opening}{}", quoted.join(", "))
}

/// The guessed token an `InvalidSubcommand` error carries — `None` for every other
/// clap error kind, which keeps clap's own render.
fn invalid_subcommand_guess(err: &clap::Error) -> Option<&str> {
    use clap::error::{ContextKind, ContextValue};

    if err.kind() != clap::error::ErrorKind::InvalidSubcommand {
        return None;
    }
    match err.get(ContextKind::InvalidSubcommand)? {
        ContextValue::String(guess) => Some(guess.as_str()),
        _ => None,
    }
}

#[cfg(test)]
mod cli_parse {
    use super::*;
    use clap::Parser;

    /// Every curated sibling-map entry builds its tip **with the T2 parse fence
    /// installed**: each command span rides `Route::mechanical`, so a map entry
    /// naming a verb that does not parse against the real CLI panics here — the
    /// map cannot grow a ghost verb, independent of which guesses the integration
    /// arms happen to drive (`design/surface-contract.md` → The route fence).
    ///
    /// The rows are **iterated from [`CURATED_SIBLING_TIPS`]** (M48 Inc 6 T2): a row
    /// added to the table joins this fence the day it lands, where the earlier
    /// hand-copied pair list let the receipt outlive the contract.
    #[test]
    fn every_curated_sibling_tip_passes_the_route_parse_fence() {
        crate::route_fence::install();
        for row in CURATED_SIBLING_TIPS {
            let (parent, guess) = (row.parent, row.guess);
            let err = Cli::try_parse_from(["jigc", parent, guess])
                .expect_err("a curated guess is an unknown subcommand");
            let argv: Vec<String> = ["jigc", parent, guess].map(String::from).into();
            let tip = unknown_subcommand_tip(&err, &argv)
                .unwrap_or_else(|| panic!("the curated guess `{parent} {guess}` must yield a tip"));
            assert_eq!(
                tip,
                (row.tip)(),
                "the tip served for `{parent} {guess}` is the row's own",
            );
            assert!(
                tip.starts_with("tip: "),
                "the tip is identifiable as a tip; got: {tip}"
            );
        }
    }

    /// Every [`FORECLOSED_ARGUMENT_TIPS`] row fires **at the node it names, for the
    /// argument shape it names**, with the route parse fence installed: the row's own probe
    /// argv is parsed, clap's real error is the input, and the block carries the row's own
    /// tip. A row naming a node that has since grown the argument stops erroring and fails
    /// here; a row whose tip names a ghost verb panics on construction.
    ///
    /// The two legs of the row set are checked as well as its members: the node really does
    /// **not** declare a `Flag` row's flag (else the row is answering a form that works),
    /// and the probe really does reject at the row's own token.
    ///
    /// Iterated from the table, so a third row joins the fence the day it lands — the
    /// integration arms in `tests/describe.rs` and `tests/clap_error_kind_axis.rs` drive
    /// the emitted bytes end to end, but each knows only its own node.
    #[test]
    fn every_foreclosed_argument_row_fires_at_its_node() {
        use clap::CommandFactory;

        crate::route_fence::install();
        for row in FORECLOSED_ARGUMENT_TIPS {
            let argv: Vec<String> = std::iter::once("jigc".to_string())
                .chain(row.probe.iter().map(|token| (*token).to_string()))
                .collect();
            let shown = argv.join(" ");
            if let ForeclosedArgument::Flag(flag) = row.arg {
                let mut node = Cli::command();
                for token in row.node {
                    node = node
                        .find_subcommand(token)
                        .unwrap_or_else(|| panic!("`{token}` is a node of the clap tree"))
                        .clone();
                }
                assert!(
                    !node
                        .get_arguments()
                        .any(|arg| arg.get_long().map(|long| format!("--{long}"))
                            == Some(flag.to_string())),
                    "`jigc {}` declares `{flag}`, so the foreclosed row answers a form \
                     that works",
                    row.node.join(" "),
                );
            }
            let err = Cli::try_parse_from(&argv)
                .expect_err("a foreclosed-argument probe must be rejected");
            let block = unexpected_argument_block(&err, &argv)
                .unwrap_or_else(|| panic!("`{shown}` must render jigc's own block"));
            assert!(
                block.contains(&(row.tip)()),
                "`{shown}` must carry the row's own tip; got:\n{block}",
            );
        }
    }

    /// **Every clap argument is classified — the complement is erased.**
    ///
    /// [`ARG_TOKENS`] is a *total* function over the tree's argument ids, and this is the
    /// ⇔ that keeps it total in both directions: no argument reaches a leaf verb without a
    /// classification, and no classification names an argument the tree no longer has.
    ///
    /// It exists because the three door registries were each derived from a hand-kept
    /// **allowlist of argument names**, which cannot see an argument nobody listed —
    /// `jigc milestone add-from-spec`'s `spec_addr` was one, and its unguarded `<slug>`
    /// head read a file from outside the repository at exit 0 (the M50 completion audit,
    /// finding 1). Clap exposes no type information for a `String` argument, so
    /// address-shapedness cannot be *computed* from the tree; making the classification
    /// total is what makes it *asked*. This assertion is the question: a new argument
    /// reddens here until an author says what its value is, and answering
    /// [`ArgToken::Doctype`] reddens [`every_doctype_door_is_registered`] until the door
    /// joins the registry the axis suites sweep.
    #[test]
    fn every_clap_argument_is_classified() {
        let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        for (id, _) in ARG_TOKENS {
            assert!(
                seen.insert(id),
                "`{id}` is classified twice — one id, one classification",
            );
        }
        let mut in_tree: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for (path, args) in clap_leaves_with_args() {
            for id in args {
                assert!(
                    arg_token(&id).is_some(),
                    "`jigc {}` takes the argument `{id}`, which ARG_TOKENS does not \
                     classify — say what its value is (a doctype, a work-unit id, a slug \
                     override, or Plain), because an unclassified argument is invisible to \
                     every door registry derived from it",
                    path.join(" "),
                );
                in_tree.insert(id);
            }
        }
        for (id, _) in ARG_TOKENS {
            assert!(
                in_tree.contains(*id),
                "ARG_TOKENS classifies `{id}`, which no leaf verb of the clap tree takes",
            );
        }
    }

    /// **The doctype-door set is derived from the clap tree, not remembered.**
    ///
    /// A leaf verb carries a [`DOCTYPE_DOORS`] row **iff** one of its clap arguments is
    /// named in [`doctype_arg_ids`] — the ⇔ that makes *"one unknown doctype, one
    /// answer, at every door that takes one"* a claim about the whole surface rather
    /// than about the seven doors an audit happened to walk. A new verb taking a
    /// doctype reddens here until it declares how the id arrives, and a row for a verb
    /// that lost its doctype argument cannot linger.
    #[test]
    fn every_doctype_door_is_registered() {
        for (path, args) in clap_leaves_with_args() {
            let carries = args
                .iter()
                .any(|id| doctype_arg_ids().contains(&id.as_str()));
            let registered = DOCTYPE_DOORS
                .iter()
                .any(|(known, _)| known.iter().eq(path.iter()));
            assert_eq!(
                carries,
                registered,
                "`jigc {}` carries a doctype argument ({carries}) but its DOCTYPE_DOORS \
                 membership is {registered} — its arguments are [{}]",
                path.join(" "),
                args.join(", "),
            );
        }
        let leaves: Vec<Vec<String>> = clap_leaves_with_args()
            .into_iter()
            .map(|(path, _)| path)
            .collect();
        for (path, _) in DOCTYPE_DOORS {
            let path: Vec<String> = path.iter().map(|token| (*token).to_string()).collect();
            assert!(
                leaves.contains(&path),
                "DOCTYPE_DOORS carries `jigc {}`, which the clap tree no longer has",
                path.join(" "),
            );
        }
        // …and a row's declared SHAPE is the shape its own argument carries, so a door
        // cannot be registered `Bare` while taking an address (or the reverse) and be
        // swept by the wrong axis.
        for (path, args) in clap_leaves_with_args() {
            let Some((_, shape)) = DOCTYPE_DOORS
                .iter()
                .find(|(known, _)| known.iter().eq(path.iter()))
            else {
                continue;
            };
            let carried: Vec<DoctypeArg> =
                args.iter().filter_map(|id| doctype_arg_shape(id)).collect();
            assert_eq!(
                carried,
                vec![*shape],
                "`jigc {}` is registered {shape:?}, but its arguments [{}] carry {carried:?}",
                path.join(" "),
                args.join(", "),
            );
        }
    }

    /// **The work-unit-id door set is derived from the clap tree, not remembered.**
    ///
    /// A leaf verb carries a [`WORK_UNIT_ID_DOORS`] row **iff** one of its clap arguments
    /// is named in [`work_unit_id_arg_ids`] — the same ⇔ that fences the doctype doors,
    /// applied to the second identity a caller hands jigc. Without it *"a malformed
    /// work-unit id is refused at every door"* would be a claim about the five resolve
    /// **seams** an audit read in the source, and a door is not a seam: the guard could
    /// be complete over the seams and still leave a door that reaches the filesystem by
    /// some other path with nothing to say.
    ///
    /// Three assertions, and the second is the one a renamed argument trips:
    ///
    ///   1. the ⇔ over every leaf — a new verb taking a work-unit id reddens here until
    ///      it declares how the id arrives, and a row for a verb that lost its id
    ///      argument cannot linger;
    ///   2. each row's `argv` **actually parses** against the real CLI, lands on the leaf
    ///      the row names, and delivers [`WORK_UNIT_ID_SLOT`]'s stand-in into the row's
    ///      declared `arg` — so a row cannot describe a door it does not reach, and
    ///      `arg` is checked rather than believed;
    ///   3. every declared `arg` is a member of [`work_unit_id_arg_ids`], so the
    ///      vocabulary the derivation reads and the vocabulary the rows use are one set.
    #[test]
    fn every_work_unit_id_door_is_registered() {
        // (1) The ⇔ against the real clap tree.
        for (path, args) in clap_leaves_with_args() {
            let carries = args
                .iter()
                .any(|id| work_unit_id_arg_ids().contains(&id.as_str()));
            let registered = WORK_UNIT_ID_DOORS
                .iter()
                .any(|row| row.door.iter().eq(path.iter()));
            assert_eq!(
                carries,
                registered,
                "`jigc {}` carries a work-unit-id argument ({carries}) but its \
                 WORK_UNIT_ID_DOORS membership is {registered} — its arguments are [{}]",
                path.join(" "),
                args.join(", "),
            );
        }

        // (2) + (3) Each row's argv reaches the leaf it names, through the arg it names.
        const SENTINEL: &str = "fence-id";
        for row in WORK_UNIT_ID_DOORS {
            let shown = row.door.join(" ");
            assert!(
                work_unit_id_arg_ids().contains(&row.arg),
                "`jigc {shown}` declares `{}`, which is not a work-unit-id argument",
                row.arg,
            );
            assert_eq!(
                row.argv
                    .iter()
                    .filter(|token| **token == WORK_UNIT_ID_SLOT)
                    .count(),
                1,
                "`jigc {shown}`: the argv must carry the id slot exactly once",
            );
            let mut argv: Vec<String> = vec!["jigc".to_string()];
            argv.extend(row.argv.iter().map(|token| {
                if *token == WORK_UNIT_ID_SLOT {
                    SENTINEL.to_string()
                } else {
                    (*token).to_string()
                }
            }));
            let matches = <Cli as clap::CommandFactory>::command()
                .try_get_matches_from(&argv)
                .unwrap_or_else(|err| panic!("`jigc {shown}`: the row's argv must parse: {err}"));
            let mut leaf = &matches;
            let mut reached: Vec<String> = Vec::new();
            while let Some((name, sub)) = leaf.subcommand() {
                reached.push(name.to_string());
                leaf = sub;
            }
            assert_eq!(
                reached,
                row.door,
                "`jigc {shown}`: the row's argv lands on `jigc {}`",
                reached.join(" "),
            );
            assert_eq!(
                leaf.get_one::<String>(row.arg).map(String::as_str),
                Some(SENTINEL),
                "`jigc {shown}`: the argv must deliver the id through `{}`",
                row.arg,
            );
        }
    }

    /// **The `--slug` door set is derived from the clap tree, not remembered.**
    ///
    /// The same ⇔ that fences the doctype and work-unit-id doors, applied to the third
    /// token a caller hands jigc: a leaf verb carries a [`SLUG_DOORS`] row **iff** one of
    /// its clap arguments is named in [`slug_arg_ids`]. Without it *"every `--slug` door
    /// refuses a value that is not a slug"* would be a claim about the five doors M39
    /// happened to build plus the sixth this increment fixed — and the sixth is precisely
    /// what a remembered list missed.
    ///
    /// Three assertions, the second being the one a renamed argument trips: the ⇔ over
    /// every leaf; each row's `argv` actually parsing against the real CLI, landing on the
    /// leaf the row names and delivering [`SLUG_OVERRIDE_SLOT`]'s stand-in into the row's
    /// declared `arg`; and every declared `arg` being a member of [`slug_arg_ids`].
    #[test]
    fn every_slug_door_is_registered() {
        // (1) The ⇔ over every leaf of the real clap tree.
        for (path, args) in clap_leaves_with_args() {
            let carries = args.iter().any(|id| slug_arg_ids().contains(&id.as_str()));
            let registered = SLUG_DOORS.iter().any(|row| row.door.iter().eq(path.iter()));
            assert_eq!(
                carries,
                registered,
                "`jigc {}` carries a `--slug` argument ({carries}) but its SLUG_DOORS \
                 membership is {registered} — its arguments are [{}]",
                path.join(" "),
                args.join(", "),
            );
        }

        // (2) + (3) Each row's argv reaches the leaf it names, through the arg it names.
        const SENTINEL: &str = "fence-slug";
        for row in SLUG_DOORS {
            let shown = row.door.join(" ");
            assert!(
                slug_arg_ids().contains(&row.arg),
                "`jigc {shown}` declares `{}`, which is not a slug-override argument",
                row.arg,
            );
            assert_eq!(
                row.argv
                    .iter()
                    .filter(|token| **token == SLUG_OVERRIDE_SLOT)
                    .count(),
                1,
                "`jigc {shown}`: the argv must carry the slug slot exactly once",
            );
            let mut argv: Vec<String> = vec!["jigc".to_string()];
            argv.extend(row.argv.iter().map(|token| {
                if *token == SLUG_OVERRIDE_SLOT {
                    SENTINEL.to_string()
                } else {
                    (*token).to_string()
                }
            }));
            let matches = <Cli as clap::CommandFactory>::command()
                .try_get_matches_from(&argv)
                .unwrap_or_else(|err| panic!("`jigc {shown}`: the row's argv must parse: {err}"));
            let mut leaf = &matches;
            let mut reached: Vec<String> = Vec::new();
            while let Some((name, sub)) = leaf.subcommand() {
                reached.push(name.to_string());
                leaf = sub;
            }
            assert_eq!(
                reached,
                row.door,
                "`jigc {shown}`: the row's argv lands on `jigc {}`",
                reached.join(" "),
            );
            assert_eq!(
                leaf.get_one::<String>(row.arg).map(String::as_str),
                Some(SENTINEL),
                "`jigc {shown}`: the argv must deliver the override through `{}`",
                row.arg,
            );
        }
    }

    /// **Every occurrence of every path-bearing argument is registered, with a rule.**
    ///
    /// The fourth ⇔ against the real clap tree, and the one that closes
    /// [`ArgToken::Plain`]'s old blind spot: an argument classified
    /// [`PlainValue::PathBearing`] must carry a [`PATH_ARG_OCCURRENCES`] row **at every
    /// leaf it occurs at**, and no row may name an occurrence the tree no longer has.
    ///
    /// **Occurrences, not ids** (`completions/artifacts/M51/settle-record.md` → §2): one id
    /// means different things at different leaves — `path` is `jigc migrate`'s source to be
    /// read and retired and `jigc unmanage`'s lookup key — so a fence over deduplicated ids
    /// would let a new leaf reuse an existing id and inherit a rule nobody asked about it.
    ///
    /// Five assertions:
    ///
    ///   1. the ⇔ over every `(leaf, argument)` pair of the real tree;
    ///   2. every row names a leaf the tree has, and a [`path_arg_ids`] member;
    ///   3. each row has at least one arm, and the arms' `when` clauses are distinct — the
    ///      `when` is the third component of the registry's key;
    ///   4. each arm's `argv` **actually parses**, lands on the leaf the row names, and
    ///      delivers its token through the row's declared `arg` — so a row cannot describe
    ///      a door it does not reach, and `arg` is checked rather than believed;
    ///   5. each disposition says something: an `Adjudicated` arm names a predicate and at
    ///      least one blocking code, a `NoRule` arm states why. A no-rule with no reason is
    ///      the silence this registry replaces.
    #[test]
    fn every_path_arg_occurrence_is_registered() {
        // (1) The ⇔ over every (leaf, argument) pair of the real clap tree.
        for (path, args) in clap_leaves_with_args() {
            for id in &args {
                let bearing = path_arg_ids().contains(&id.as_str());
                let registered = PATH_ARG_OCCURRENCES
                    .iter()
                    .any(|row| row.door.iter().eq(path.iter()) && row.arg == id);
                assert_eq!(
                    bearing,
                    registered,
                    "`jigc {}` takes `{id}`, which is path-bearing ({bearing}) but its \
                     PATH_ARG_OCCURRENCES membership is {registered} — every occurrence of a \
                     path-bearing argument owes one stated rule or one stated \
                     no-rule-and-why",
                    path.join(" "),
                );
            }
        }

        // (2) Every row names an occurrence the tree has.
        let leaves = clap_leaves_with_args();
        for row in PATH_ARG_OCCURRENCES {
            let shown = row.door.join(" ");
            let Some((_, args)) = leaves
                .iter()
                .find(|(path, _)| path.iter().eq(row.door.iter()))
            else {
                panic!(
                    "PATH_ARG_OCCURRENCES carries `jigc {shown}`, which the clap tree no \
                     longer has"
                );
            };
            assert!(
                args.iter().any(|id| id == row.arg),
                "`jigc {shown}` no longer takes `{}` — its arguments are [{}]",
                row.arg,
                args.join(", "),
            );
            assert!(
                path_arg_ids().contains(&row.arg),
                "`jigc {shown}` declares `{}`, which ARG_TOKENS does not classify PathBearing",
                row.arg,
            );

            // (3) At least one arm, and the `when` clauses are the key's third component.
            assert!(
                !row.arms.is_empty(),
                "`jigc {shown}`'s `{}` carries no arm — an occurrence with no arm states \
                 nothing",
                row.arg,
            );
            let mut whens: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
            for arm in row.arms {
                assert!(
                    !arm.when.trim().is_empty(),
                    "`jigc {shown}`'s `{}` carries an arm with no condition",
                    row.arg,
                );
                assert!(
                    whens.insert(arm.when),
                    "`jigc {shown}`'s `{}` carries two arms keyed `{}`",
                    row.arg,
                    arm.when,
                );
            }

            // (4) + (5) Each arm reaches the door it names, through the argument it names,
            //           and says what it does about the token.
            for arm in row.arms {
                let token = match arm.token {
                    ArmToken::Caller => "fence-path",
                    ArmToken::Literal(literal) => literal,
                };
                assert_eq!(
                    arm.argv
                        .iter()
                        .filter(|part| **part == PATH_ARG_SLOT)
                        .count(),
                    1,
                    "`jigc {shown}` [{}]: the argv must carry the path slot exactly once",
                    arm.when,
                );
                let mut argv: Vec<String> = vec!["jigc".to_string()];
                argv.extend(arm.argv.iter().map(|part| {
                    if *part == PATH_ARG_SLOT {
                        token.to_string()
                    } else {
                        (*part).to_string()
                    }
                }));
                let matches = <Cli as clap::CommandFactory>::command()
                    .try_get_matches_from(&argv)
                    .unwrap_or_else(|err| {
                        panic!(
                            "`jigc {shown}` [{}]: the arm's argv must parse: {err}",
                            arm.when
                        )
                    });
                let mut leaf = &matches;
                let mut reached: Vec<String> = Vec::new();
                while let Some((name, sub)) = leaf.subcommand() {
                    reached.push(name.to_string());
                    leaf = sub;
                }
                assert_eq!(
                    reached,
                    row.door,
                    "`jigc {shown}` [{}]: the arm's argv lands on `jigc {}`",
                    arm.when,
                    reached.join(" "),
                );
                // Read the value **raw**: the six path-bearing arguments are not one Rust
                // type (`file` is a `PathBuf`, the rest are `String`), and a typed read
                // panics on the mismatch rather than answering.
                let delivered: Vec<String> = leaf
                    .get_raw(row.arg)
                    .map(|values| {
                        values
                            .map(|value| value.to_string_lossy().into_owned())
                            .collect()
                    })
                    .unwrap_or_default();
                assert_eq!(
                    delivered,
                    vec![token.to_string()],
                    "`jigc {shown}` [{}]: the argv must deliver the token through `{}`",
                    arm.when,
                    row.arg,
                );
                match arm.disposition {
                    PathArgDisposition::Adjudicated { predicate, codes } => {
                        assert!(
                            !predicate.trim().is_empty(),
                            "`jigc {shown}` [{}]: an adjudicated arm names the predicate that \
                             answers",
                            arm.when,
                        );
                        assert!(
                            !codes.is_empty(),
                            "`jigc {shown}` [{}]: an adjudicated arm names at least one \
                             blocking code",
                            arm.when,
                        );
                        for code in codes {
                            assert!(
                                code.contains('.') && !code.ends_with('.'),
                                "`jigc {shown}` [{}]: `{code}` is not a `<family>.<cause>` \
                                 finding code",
                                arm.when,
                            );
                        }
                    }
                    PathArgDisposition::NoRule { why } => assert!(
                        why.trim().len() > 40,
                        "`jigc {shown}` [{}]: a no-rule arm states WHY the token becomes no \
                         path component — a no-rule with no reason is the silence this \
                         registry replaces",
                        arm.when,
                    ),
                }
            }
        }
    }

    /// Every leaf verb path paired with its clap **argument ids** — the derivation
    /// substrate [`every_doctype_door_is_registered`] reads. The same walk as
    /// [`clap_tree`], carrying the arguments its callers do not need.
    fn clap_leaves_with_args() -> Vec<(Vec<String>, Vec<String>)> {
        fn walk(
            cmd: &clap::Command,
            prefix: Vec<String>,
            out: &mut Vec<(Vec<String>, Vec<String>)>,
        ) {
            let subs: Vec<&clap::Command> = cmd
                .get_subcommands()
                .filter(|sub| sub.get_name() != "help")
                .collect();
            if subs.is_empty() {
                if !prefix.is_empty() {
                    let args = cmd
                        .get_arguments()
                        .map(|arg| arg.get_id().to_string())
                        .collect();
                    out.push((prefix, args));
                }
                return;
            }
            for sub in subs {
                let mut child = prefix.clone();
                child.push(sub.get_name().to_string());
                walk(sub, child, out);
            }
        }
        let mut out = Vec::new();
        walk(
            &<Cli as clap::CommandFactory>::command(),
            Vec::new(),
            &mut out,
        );
        out
    }

    /// Walk the clap tree, returning `(leaf verb paths, parent node paths)` — the root
    /// (`vec![]`) is a parent node. `help` is clap's builtin, not a jigc verb.
    fn clap_tree() -> (Vec<Vec<String>>, Vec<Vec<String>>) {
        fn walk(
            cmd: &clap::Command,
            prefix: Vec<String>,
            leaves: &mut Vec<Vec<String>>,
            parents: &mut Vec<Vec<String>>,
        ) {
            let subs: Vec<&clap::Command> = cmd
                .get_subcommands()
                .filter(|sub| sub.get_name() != "help")
                .collect();
            if subs.is_empty() {
                if !prefix.is_empty() {
                    leaves.push(prefix);
                }
                return;
            }
            parents.push(prefix.clone());
            for sub in subs {
                let mut child = prefix.clone();
                child.push(sub.get_name().to_string());
                walk(sub, child, leaves, parents);
            }
        }
        let (mut leaves, mut parents) = (Vec::new(), Vec::new());
        walk(
            &<Cli as clap::CommandFactory>::command(),
            Vec::new(),
            &mut leaves,
            &mut parents,
        );
        (leaves, parents)
    }

    /// **The read/write classification is total over the clap tree** — a bijection, so a
    /// verb added anywhere must classify itself before it can ship, and a row for a verb
    /// that no longer exists cannot linger.
    ///
    /// This is what makes *"a read intent is never answered with a write verb"* a claim
    /// about the whole surface rather than about the `(parent, guess)` pairs a trial
    /// happened to record: the rule reads [`VERB_KINDS`], and membership is decided here,
    /// at the tree (`implementation/dev-workflow.md` → a grep is not a fence).
    #[test]
    fn every_leaf_verb_is_classified() {
        let (leaves, _) = clap_tree();
        for leaf in &leaves {
            assert!(
                verb_kind(leaf).is_some(),
                "`jigc {}` is a leaf verb and must carry a VERB_KINDS row — is it a read \
                 (it only reports) or a write (it can act)?",
                leaf.join(" "),
            );
        }
        for (path, _) in VERB_KINDS {
            let path: Vec<String> = path.iter().map(|token| (*token).to_string()).collect();
            assert!(
                leaves.contains(&path),
                "VERB_KINDS carries `jigc {}`, which the clap tree no longer has",
                path.join(" "),
            );
        }
        assert_eq!(
            VERB_KINDS.len(),
            leaves.len(),
            "the classification is a bijection with the clap tree's leaves",
        );
    }

    /// **Every leaf verb says what it does on the user's behalf** — a bijection with the
    /// clap tree, on [`every_leaf_verb_is_classified`]'s mold, so a verb added anywhere
    /// **reddens until someone answers it** and a row for a verb that no longer exists
    /// cannot linger.
    ///
    /// Without it, *"no repository posture reaches a door that commits or moves on the
    /// user's behalf"* would be a claim about the doors somebody remembered — the shape
    /// that has left a class half-swept in five waves running. [`ActsOnBehalf::Neither`]
    /// is a stated verdict here, not an absence: a leaf with no row is a red test, never a
    /// silent *nothing to adjudicate*.
    #[test]
    fn every_leaf_verb_says_what_it_acts_on() {
        let (leaves, _) = clap_tree();
        for leaf in &leaves {
            let rows = BEHALF_DOORS
                .iter()
                .filter(|row| row.door.iter().eq(leaf.iter()))
                .count();
            assert_eq!(
                rows,
                1,
                "`jigc {}` is a leaf verb and must carry exactly one BEHALF_DOORS row — \
                 does it commit on the user's behalf, move a committed file on their \
                 behalf, or neither?",
                leaf.join(" "),
            );
        }
        for row in BEHALF_DOORS {
            let path: Vec<String> = row.door.iter().map(|token| (*token).to_string()).collect();
            assert!(
                leaves.contains(&path),
                "BEHALF_DOORS carries `jigc {}`, which the clap tree no longer has",
                path.join(" "),
            );
        }
        assert_eq!(
            BEHALF_DOORS.len(),
            leaves.len(),
            "the on-behalf classification is a bijection with the clap tree's leaves",
        );
    }

    /// **One parseable argv per leaf of the clap tree** — the fixture behind
    /// [`leaf_reads_the_parsed_command_back_to_its_own_door`].
    ///
    /// Only well-formedness matters: nothing here runs, so a required value is filled with
    /// whatever parses. The table is ⇔-fenced against the real clap tree by the test
    /// below, so a leaf added anywhere reddens here until someone supplies its argv — the
    /// table cannot quietly stop covering the classification the posture guard reads.
    const LEAF_ARGV: &[&[&str]] = &[
        &["start", "an intent"],
        &["workflow", "single-task", "--preview"],
        &["setup"],
        &["uninstall"],
        &["upgrade"],
        &["ingest"],
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
        &["migrate-corpus"],
        &["unmanage", "docs/x.md"],
        &["rename", "adr:keeper", "--to", "Axis"],
        &["relocate", "vision", "--from", "docs/vision/"],
        &["describe"],
        &["validate"],
        &["doc", "create", "adr", "--title", "Axis"],
        &["doc", "add-item", "roadmap#milestones", "--title", "Axis"],
        &["doc", "remove-item", "roadmap#milestones.m1"],
        &[
            "doc",
            "retitle-item",
            "roadmap#milestones.m1",
            "--title",
            "Axis",
        ],
        &["doc", "rename", "adr:keeper", "--to", "Axis"],
        &["doc", "set-field", "commit:t#type", "--value", "feat"],
        &["doc", "set-slot", "commit:t#summary", "--from-file", "-"],
        &["doc", "author", "adr", "--from-file", "-"],
        &["doc", "show", "adr:keeper"],
        &["doc", "schema", "adr"],
        &["doc", "list"],
        &["task", "list"],
        &["task", "diff", "axis-unit"],
        &["task", "validate", "axis-unit"],
        &["task", "discard", "axis-unit"],
        &["task", "finalize", "axis-unit"],
        &["task", "bind", "decision", "adr:keeper", "axis-unit"],
        &["config", "set", "docs-root", "docs"],
        &[
            "config",
            "insert-step",
            "step.yaml",
            "--workflow",
            "single-task",
            "--after",
            "implement",
        ],
        &[
            "config",
            "replace-step",
            "single-task:implement",
            "step.yaml",
        ],
        &["config", "remove-step", "single-task:implement"],
        &[
            "config",
            "fill",
            "single-task:implement",
            "--from-file",
            "-",
        ],
        &["config", "get", "docs-root"],
        &["config", "list"],
        &["config", "fork", "single-task"],
        &["milestone", "create", "Axis milestone"],
        &["milestone", "add-task", "axis-unit", "axis intent"],
        &["milestone", "add-from-spec", "axis-unit", "spec:axis"],
        &["milestone", "list-tasks", "axis-unit"],
        &["milestone", "provision", "axis-unit"],
        &["milestone", "execute", "axis-unit"],
        &["milestone", "join", "axis-unit"],
        &["milestone", "finalize", "axis-unit"],
        &["milestone", "discard", "axis-unit"],
    ];

    /// **[`Command::leaf`] answers each leaf's own door, for every leaf** — the totality
    /// the posture guard rests on.
    ///
    /// The exhaustive match compiler-fences a *new* variant, and nothing more: a variant
    /// mapped to the **wrong** door still compiles, and would hand the guard another door's
    /// class — a committing door's family applied to a read verb (a refusal with no damage
    /// behind it), or a `Neither` verdict over a door that commits. So every leaf is parsed
    /// through the real clap tree and its answer checked against the door its own argv
    /// reaches, on the longest-classified-prefix mold [`BEHALF_DOORS`] already uses.
    #[test]
    fn leaf_reads_the_parsed_command_back_to_its_own_door() {
        let (leaves, _) = clap_tree();
        let mut answered: Vec<Vec<String>> = Vec::new();
        for argv in LEAF_ARGV {
            let shown = argv.join(" ");
            let cli = Cli::try_parse_from(std::iter::once("jigc").chain(argv.iter().copied()))
                .unwrap_or_else(|err| panic!("`jigc {shown}` must parse: {err}"));
            // The door this argv names: its longest leading prefix that is a classified
            // leaf — derived from the argv rather than written beside it, so the row
            // cannot assert its own expectation.
            let expected: &[&str] = BEHALF_DOORS
                .iter()
                .map(|row| row.door)
                .filter(|door| argv.len() >= door.len() && argv[..door.len()] == **door)
                .max_by_key(|door| door.len())
                .unwrap_or_else(|| panic!("`jigc {shown}` names no classified leaf"));
            assert_eq!(
                cli.command.leaf(),
                expected,
                "`jigc {shown}` parses to a command whose leaf is not its own door",
            );
            answered.push(expected.iter().map(|token| (*token).to_string()).collect());
        }
        for leaf in &leaves {
            assert!(
                answered.contains(leaf),
                "`jigc {}` is a leaf verb with no row in LEAF_ARGV — the leaf-identity \
                 fence cannot see it",
                leaf.join(" "),
            );
        }
        assert_eq!(
            answered.len(),
            leaves.len(),
            "LEAF_ARGV is a bijection with the clap tree's leaves",
        );
    }

    /// **Every acting row's argv is runnable and reaches the door it names** — the
    /// [`WORK_UNIT_ID_DOORS`] mold, applied to the rows the posture axis drives.
    ///
    /// A row whose argv did not parse, or parsed onto a *different* leaf, would hand that
    /// axis a cell proving something about another door — so the argv is checked against
    /// the real clap tree rather than believed.
    #[test]
    fn every_acting_row_carries_a_runnable_argv() {
        const SENTINEL: &str = "fence-id";
        for row in BEHALF_DOORS {
            let shown = row.door.join(" ");
            let argv = match &row.acts {
                ActsOnBehalf::CommitsOnBehalf { argv, .. } => *argv,
                ActsOnBehalf::MovesOnBehalf { argv } => *argv,
                ActsOnBehalf::Neither => continue,
            };
            assert!(
                !argv.is_empty(),
                "`jigc {shown}` acts on the user's behalf, so its row owes a runnable argv",
            );
            assert!(
                argv.iter()
                    .filter(|token| **token == WORK_UNIT_ID_SLOT)
                    .count()
                    <= 1,
                "`jigc {shown}`: the argv may carry the id slot at most once",
            );
            let mut full: Vec<String> = vec!["jigc".to_string()];
            full.extend(argv.iter().map(|token| {
                if *token == WORK_UNIT_ID_SLOT {
                    SENTINEL.to_string()
                } else {
                    (*token).to_string()
                }
            }));
            let matches = <Cli as clap::CommandFactory>::command()
                .try_get_matches_from(&full)
                .unwrap_or_else(|err| panic!("`jigc {shown}`: the row's argv must parse: {err}"));
            let mut leaf = &matches;
            let mut reached: Vec<String> = Vec::new();
            while let Some((name, sub)) = leaf.subcommand() {
                reached.push(name.to_string());
                leaf = sub;
            }
            assert_eq!(
                reached,
                row.door
                    .iter()
                    .map(|token| (*token).to_string())
                    .collect::<Vec<String>>(),
                "`jigc {shown}`: the row's argv reaches `jigc {}` instead",
                reached.join(" "),
            );
        }
    }

    /// **`COMMITTING_DOORS ⊆ commit-on-behalf`** — the membership rule, asserted rather
    /// than assumed.
    ///
    /// Each `CommittingDoor::verb` is mapped back to its leaf path by dropping the leading
    /// `jigc` and any parenthesised commit-model arm (`jigc milestone finalize (squash:
    /// true)`), then resolved against [`VERB_KINDS`] on `freeze_enforcement.rs`'s
    /// longest-classified-prefix mold — so a door label naming no leaf reddens here too.
    /// The ⊆ is one-directional by design: `jigc setup` commits on the user's behalf and
    /// carries **no** rejection identity (its install commit passes `--no-verify`), so the
    /// superset is strictly larger and
    /// [`jigc_setup_commits_on_behalf_and_states_its_unborn_exemption`] names it.
    #[test]
    fn every_committing_door_acts_on_the_users_behalf() {
        for door in crate::invocation_log::COMMITTING_DOORS {
            let label = door.verb;
            let tokens: Vec<&str> = label
                .split(" (")
                .next()
                .expect("a split always yields a first part")
                .split_whitespace()
                .collect();
            let (leading, path) = tokens.split_first().expect("a door label is not empty");
            assert_eq!(
                *leading, "jigc",
                "the COMMITTING_DOORS label `{label}` must name the binary it is typed after",
            );
            let leaf = VERB_KINDS
                .iter()
                .map(|(leaf, _)| *leaf)
                .filter(|leaf| path.len() >= leaf.len() && path[..leaf.len()] == **leaf)
                .max_by_key(|leaf| leaf.len())
                .unwrap_or_else(|| panic!("`{label}` reaches no VERB_KINDS leaf"));
            let row = BEHALF_DOORS
                .iter()
                .find(|row| row.door.iter().eq(leaf.iter()))
                .unwrap_or_else(|| {
                    panic!(
                        "`{label}` reaches `jigc {}`, which carries no BEHALF_DOORS row",
                        leaf.join(" "),
                    )
                });
            assert!(
                matches!(row.acts, ActsOnBehalf::CommitsOnBehalf { .. }),
                "`{label}` runs a hook-capable commit on the user's behalf, so `jigc {}` \
                 must be classified CommitsOnBehalf — COMMITTING_DOORS ⊆ commit-on-behalf",
                leaf.join(" "),
            );
        }
    }

    /// **`jigc setup` commits on the user's behalf, and states its one exemption** —
    /// asserted **by name**, because `setup` is the one member of the class no
    /// `COMMITTING_DOORS` row supplies (its install commit passes `--no-verify`, so there
    /// is no hook rejection to carry an identity) and a ⊆ assertion cannot see a member
    /// its subset never names.
    ///
    /// The exemption is checked as a **value**: the member it names, and a reason that
    /// quotes the M30 audit rationale. An exemption is how a door states that a hole in a
    /// family is a decision; a silent hole is what this increment exists to remove.
    #[test]
    fn jigc_setup_commits_on_behalf_and_states_its_unborn_exemption() {
        let row = BEHALF_DOORS
            .iter()
            .find(|row| row.door.iter().eq(["setup"].iter()))
            .expect("`jigc setup` must carry a BEHALF_DOORS row");
        let ActsOnBehalf::CommitsOnBehalf { exempt, .. } = &row.acts else {
            panic!(
                "`jigc setup` lands `chore(jigc): install jigc workspace config` on the \
                 user's behalf — it must be classified CommitsOnBehalf",
            );
        };
        let exemptions: Vec<PostureMember> = exempt.iter().map(|row| row.member).collect();
        assert_eq!(
            exemptions,
            vec![PostureMember::HeadUnborn],
            "`jigc setup` is exempt from the unborn member and from nothing else",
        );
        assert_eq!(
            exempt[0].reason, SETUP_UNBORN_EXEMPTION,
            "the exemption's reason is the quoted M30 audit rationale",
        );
        assert!(
            SETUP_UNBORN_EXEMPTION.contains("M30 audit finding 1"),
            "the reason quotes `commit_install`'s rationale rather than paraphrasing it",
        );
    }

    /// **Every parent node can answer "how do I see what is there?"** — the root
    /// included — and every command span in every answer is a [`VerbKind::Read`] verb
    /// that parses (the tips are built here with the route fence installed).
    ///
    /// Without this, a new verb *group* could ship with no read answer, and a
    /// read-shaped miss under it would fall to silence.
    #[test]
    fn every_parent_node_carries_a_read_answer() {
        crate::route_fence::install();
        let (_, parents) = clap_tree();
        for parent in &parents {
            let answer = PARENT_READ_ANSWERS
                .iter()
                .find(|answer| answer.parent.iter().eq(parent.iter()))
                .unwrap_or_else(|| {
                    panic!(
                        "the `jigc {}` node must carry a read answer for a read-shaped miss",
                        parent.join(" "),
                    )
                });
            let tip = (answer.tip)();
            assert!(
                tip.starts_with("tip: "),
                "the answer is identifiable as a tip; got: {tip}",
            );
            let named = spans_named(&tip);
            assert!(
                !named.is_empty(),
                "the `jigc {}` read answer must name a verb; got: {tip}",
                parent.join(" "),
            );
            for path in named {
                assert_eq!(
                    verb_kind(&path),
                    Some(VerbKind::Read),
                    "the `jigc {}` read answer names `jigc {}`, which acts",
                    parent.join(" "),
                    path.join(" "),
                );
            }
        }
        for answer in PARENT_READ_ANSWERS {
            let path: Vec<String> = answer
                .parent
                .iter()
                .map(|token| (*token).to_string())
                .collect();
            assert!(
                parents.contains(&path),
                "a read answer is declared for `jigc {}`, which is no node of the clap tree",
                path.join(" "),
            );
        }
    }

    /// The leaf-verb paths a tip's backticked `jigc …` spans name — each token run walked
    /// down the clap tree for as far as it matches.
    fn spans_named(tip: &str) -> Vec<Vec<String>> {
        tip.split('`')
            .skip(1)
            .step_by(2)
            .filter_map(|span| {
                let mut tokens = span.split_whitespace();
                if tokens.next() != Some("jigc") {
                    return None;
                }
                let mut cmd = <Cli as clap::CommandFactory>::command();
                let mut path = Vec::new();
                for token in tokens {
                    match cmd.find_subcommand(token).cloned() {
                        Some(sub) => {
                            path.push(token.to_string());
                            cmd = sub;
                        }
                        None => break,
                    }
                }
                (!path.is_empty()).then_some(path)
            })
            .collect()
    }

    /// A clap error that is not `InvalidSubcommand` never yields a tip — the map
    /// intercepts semantic guesses only, not ordinary usage errors — **and jigc
    /// renders no block for it either**, so clap keeps that error surface whole,
    /// did-you-mean included (M48 Inc 6 T2: the takeover is one error kind wide).
    #[test]
    fn a_non_subcommand_usage_error_yields_no_tip() {
        let err = Cli::try_parse_from(["jigc", "task", "validate"])
            .expect_err("`task validate` with no id is a usage error");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
        let argv: Vec<String> = ["jigc", "task", "validate"].map(String::from).into();
        assert_eq!(unknown_subcommand_tip(&err, &argv), None);
        assert_eq!(unknown_subcommand_block(&err, &argv), None);
    }

    /// The re-emitted did-you-mean matches clap's own wording in **both** arities —
    /// singular and plural — since jigc now writes that line for every uncurated
    /// guess and a drift here would silently reword clap's suggestion.
    #[test]
    fn the_re_emitted_did_you_mean_matches_claps_wording_in_both_arities() {
        assert_eq!(
            did_you_mean(&["validate".to_string()]),
            "tip: a similar subcommand exists: 'validate'",
        );
        assert_eq!(
            did_you_mean(&["create".to_string(), "rename".to_string()]),
            "tip: some similar subcommands exist: 'create', 'rename'",
        );
    }

    #[test]
    fn start_defaults_to_agent_format() {
        let cli = Cli::try_parse_from(["jigc", "start"]).expect("`jigc start` parses");
        assert_eq!(cli.format, Format::Agent);
        assert_eq!(
            cli.command,
            Command::Start {
                intent: None,
                workflow: None,
                task: None,
                explain: false,
                slug: None,
            }
        );
    }

    #[test]
    fn start_with_intent_parses_the_positional() {
        let cli =
            Cli::try_parse_from(["jigc", "start", "add rate limiter"]).expect("intent parses");
        assert_eq!(
            cli.command,
            Command::Start {
                intent: Some("add rate limiter".to_string()),
                workflow: None,
                task: None,
                explain: false,
                slug: None,
            }
        );
    }

    #[test]
    fn start_task_parses_the_resume_form() {
        let cli = Cli::try_parse_from(["jigc", "start", "--task", "add-rate-limiter"])
            .expect("`--task <id>` parses");
        assert_eq!(
            cli.command,
            Command::Start {
                intent: None,
                workflow: None,
                task: Some("add-rate-limiter".to_string()),
                explain: false,
                slug: None,
            }
        );
    }

    #[test]
    fn start_workflow_combines_with_the_intent_positional() {
        let cli = Cli::try_parse_from(["jigc", "start", "--workflow", "single-task", "an intent"])
            .expect("`--workflow <X> <intent>` parses");
        assert_eq!(
            cli.command,
            Command::Start {
                intent: Some("an intent".to_string()),
                workflow: Some("single-task".to_string()),
                task: None,
                explain: false,
                slug: None,
            }
        );
    }

    #[test]
    fn start_explain_combines_with_intent_and_workflow() {
        let cli = Cli::try_parse_from([
            "jigc",
            "start",
            "--explain",
            "--workflow",
            "single-task",
            "an intent",
        ])
        .expect("`--explain --workflow <X> <intent>` parses");
        assert_eq!(
            cli.command,
            Command::Start {
                intent: Some("an intent".to_string()),
                workflow: Some("single-task".to_string()),
                task: None,
                explain: true,
                slug: None,
            }
        );
    }

    #[test]
    fn start_rejects_explain_and_task_together() {
        let err = Cli::try_parse_from(["jigc", "start", "--explain", "--task", "some-id"])
            .expect_err("`--explain` together with `--task` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
    }

    #[test]
    fn start_rejects_workflow_and_task_together() {
        let err =
            Cli::try_parse_from(["jigc", "start", "--workflow", "single-task", "--task", "id"])
                .expect_err("`--workflow` together with `--task` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
    }

    #[test]
    fn start_rejects_intent_and_task_together() {
        let err = Cli::try_parse_from(["jigc", "start", "an intent", "--task", "some-id"])
            .expect_err("`<intent>` together with `--task` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
    }

    #[test]
    fn start_parses_the_slug_override() {
        let cli = Cli::try_parse_from([
            "jigc",
            "start",
            "--workflow",
            "single-task",
            "an intent",
            "--slug",
            "explicit-id",
        ])
        .expect("`jigc start --workflow <X> <intent> --slug <id>` parses");
        assert_eq!(
            cli.command,
            Command::Start {
                intent: Some("an intent".to_string()),
                workflow: Some("single-task".to_string()),
                task: None,
                explain: false,
                slug: Some("explicit-id".to_string()),
            }
        );
    }

    #[test]
    fn start_rejects_slug_and_task_together() {
        let err = Cli::try_parse_from(["jigc", "start", "--task", "some-id", "--slug", "explicit"])
            .expect_err(
                "`--slug` together with `--task` must be rejected (a resume mints nothing)",
            );
        assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
    }

    #[test]
    fn format_json_is_selected() {
        let cli = Cli::try_parse_from(["jigc", "start", "--format", "json"])
            .expect("--format json parses");
        assert_eq!(cli.format, Format::Json);
    }

    #[test]
    fn format_human_is_selected() {
        let cli = Cli::try_parse_from(["jigc", "start", "--format", "human"])
            .expect("--format human parses");
        assert_eq!(cli.format, Format::Human);
    }

    #[test]
    fn unknown_format_value_is_rejected() {
        let err = Cli::try_parse_from(["jigc", "start", "--format", "xml"])
            .expect_err("unknown format must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::InvalidValue);
    }

    #[test]
    fn setup_parses() {
        let cli = Cli::try_parse_from(["jigc", "setup"]).expect("`jigc setup` parses");
        assert_eq!(cli.command, Command::Setup { force: false });
        let forced = Cli::try_parse_from(["jigc", "setup", "--force"])
            .expect("`jigc setup --force` parses — the door's single consent");
        assert_eq!(forced.command, Command::Setup { force: true });
    }

    #[test]
    fn uninstall_parses() {
        let cli = Cli::try_parse_from(["jigc", "uninstall"]).expect("`jigc uninstall` parses");
        assert_eq!(cli.command, Command::Uninstall { force: false });
    }

    #[test]
    fn uninstall_takes_no_positional() {
        let err = Cli::try_parse_from(["jigc", "uninstall", "extra"])
            .expect_err("`jigc uninstall` takes no positional argument");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn upgrade_parses() {
        let cli = Cli::try_parse_from(["jigc", "upgrade"]).expect("`jigc upgrade` parses");
        assert_eq!(cli.command, Command::Upgrade);
    }

    #[test]
    fn upgrade_format_json_is_selected() {
        let cli = Cli::try_parse_from(["jigc", "upgrade", "--format", "json"])
            .expect("`jigc upgrade --format json` parses");
        assert_eq!(cli.format, Format::Json);
        assert_eq!(cli.command, Command::Upgrade);
    }

    #[test]
    fn upgrade_takes_no_positional() {
        let err = Cli::try_parse_from(["jigc", "upgrade", "extra"])
            .expect_err("`jigc upgrade` takes no positional argument");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn ingest_parses() {
        let cli = Cli::try_parse_from(["jigc", "ingest"]).expect("`jigc ingest` parses");
        assert_eq!(cli.command, Command::Ingest);
    }

    #[test]
    fn ingest_format_json_is_selected() {
        let cli = Cli::try_parse_from(["jigc", "ingest", "--format", "json"])
            .expect("`jigc ingest --format json` parses");
        assert_eq!(cli.format, Format::Json);
        assert_eq!(cli.command, Command::Ingest);
    }

    #[test]
    fn ingest_takes_no_positional() {
        let err = Cli::try_parse_from(["jigc", "ingest", "extra"])
            .expect_err("`jigc ingest` takes no positional argument");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn migrate_parses_the_path_and_doctype() {
        let cli = Cli::try_parse_from(["jigc", "migrate", "CHANGELOG.md", "--as", "changelog"])
            .expect("`jigc migrate <path> --as <doctype>` parses");
        assert_eq!(
            cli.command,
            Command::Migrate {
                path: "CHANGELOG.md".to_string(),
                r#as: "changelog".to_string(),
                slug: None,
            }
        );
    }

    /// (M43 inc-6 T2) `--slug` on `jigc migrate` parses as the optional doc-id
    /// override — recorded at mint and applied when the agent authors the target doc.
    #[test]
    fn migrate_parses_the_slug_override() {
        let cli = Cli::try_parse_from([
            "jigc",
            "migrate",
            "CHANGELOG.md",
            "--as",
            "changelog",
            "--slug",
            "release-log",
        ])
        .expect("`jigc migrate <path> --as <doctype> --slug <s>` parses");
        assert_eq!(
            cli.command,
            Command::Migrate {
                path: "CHANGELOG.md".to_string(),
                r#as: "changelog".to_string(),
                slug: Some("release-log".to_string()),
            }
        );
    }

    #[test]
    fn migrate_requires_the_as_flag() {
        let err = Cli::try_parse_from(["jigc", "migrate", "CHANGELOG.md"])
            .expect_err("`jigc migrate <path>` with no `--as` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn migrate_requires_a_path() {
        let err = Cli::try_parse_from(["jigc", "migrate", "--as", "changelog"])
            .expect_err("`jigc migrate --as <doctype>` with no path must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    /// The bare verb, and the commit boundary's two opt-outs (M42 Inc-1 T3): `--no-commit`
    /// (write, land nothing) and `--dry-run` (write nothing) — with `--dry-run`'s **implication**
    /// pinned: a dry run never commits, because nothing was written for it to stage.
    #[test]
    fn migrate_corpus_parses_with_the_commit_boundary_flags() {
        let cli =
            Cli::try_parse_from(["jigc", "migrate-corpus"]).expect("`jigc migrate-corpus` parses");
        assert_eq!(
            cli.command,
            Command::MigrateCorpus {
                no_commit: false,
                dry_run: false,
            },
        );
        // The default applies AND lands its writes — the standing behaviour.
        assert!(migrate_corpus::Options::default().writes());
        assert!(migrate_corpus::Options::default().commits());

        let cli = Cli::try_parse_from(["jigc", "migrate-corpus", "--no-commit"])
            .expect("`jigc migrate-corpus --no-commit` parses");
        assert_eq!(
            cli.command,
            Command::MigrateCorpus {
                no_commit: true,
                dry_run: false,
            },
        );
        let opts = migrate_corpus::Options {
            no_commit: true,
            dry_run: false,
        };
        assert!(opts.writes(), "`--no-commit` still writes the migration");
        assert!(!opts.commits(), "`--no-commit` lands nothing");

        let cli = Cli::try_parse_from(["jigc", "migrate-corpus", "--dry-run"])
            .expect("`jigc migrate-corpus --dry-run` parses");
        assert_eq!(
            cli.command,
            Command::MigrateCorpus {
                no_commit: false,
                dry_run: true,
            },
        );
        let opts = migrate_corpus::Options {
            no_commit: false,
            dry_run: true,
        };
        assert!(!opts.writes(), "`--dry-run` suppresses the write");
        assert!(
            !opts.commits(),
            "`--dry-run` IMPLIES no commit — nothing was written to stage",
        );
    }

    #[test]
    fn relocate_parses_type_and_from() {
        let cli = Cli::try_parse_from(["jigc", "relocate", "vision", "--from", "docs/vision/"])
            .expect("`jigc relocate <type> --from <prior>` parses");
        assert_eq!(
            cli.command,
            Command::Relocate {
                r#type: "vision".to_string(),
                from: "docs/vision/".to_string(),
            }
        );
    }

    #[test]
    fn relocate_requires_the_from_flag() {
        let err = Cli::try_parse_from(["jigc", "relocate", "vision"])
            .expect_err("`jigc relocate <type>` with no `--from` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn migrate_corpus_takes_no_positional() {
        let err = Cli::try_parse_from(["jigc", "migrate-corpus", "extra"])
            .expect_err("`jigc migrate-corpus` takes no positional argument");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn validate_parses() {
        let cli = Cli::try_parse_from(["jigc", "validate"]).expect("`jigc validate` parses");
        assert_eq!(cli.command, Command::Validate);
    }

    #[test]
    fn validate_format_json_is_selected() {
        let cli = Cli::try_parse_from(["jigc", "validate", "--format", "json"])
            .expect("`jigc validate --format json` parses");
        assert_eq!(cli.format, Format::Json);
        assert_eq!(cli.command, Command::Validate);
    }

    #[test]
    fn validate_takes_no_positional() {
        let err = Cli::try_parse_from(["jigc", "validate", "extra"])
            .expect_err("`jigc validate` takes no positional argument");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn rename_parses_the_address_and_title() {
        let cli = Cli::try_parse_from(["jigc", "rename", "adr:old-cache", "--to", "New cache"])
            .expect("`jigc rename <addr> --to <title>` parses");
        assert_eq!(
            cli.command,
            Command::Rename {
                old_slug: "adr:old-cache".to_string(),
                to: "New cache".to_string(),
                slug: None,
            }
        );
    }

    #[test]
    fn rename_parses_the_explicit_slug_override() {
        let cli = Cli::try_parse_from([
            "jigc",
            "rename",
            "adr:old-cache",
            "--to",
            "New cache",
            "--slug",
            "fast-cache",
        ])
        .expect("`jigc rename <addr> --to <title> --slug <slug>` parses");
        assert_eq!(
            cli.command,
            Command::Rename {
                old_slug: "adr:old-cache".to_string(),
                to: "New cache".to_string(),
                slug: Some("fast-cache".to_string()),
            }
        );
    }

    #[test]
    fn rename_requires_the_to_flag() {
        let err = Cli::try_parse_from(["jigc", "rename", "adr:old-cache"])
            .expect_err("`jigc rename <addr>` with no `--to` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn rename_requires_an_address() {
        let err = Cli::try_parse_from(["jigc", "rename", "--to", "New cache"])
            .expect_err("`jigc rename --to <title>` with no address must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn describe_parses() {
        let cli = Cli::try_parse_from(["jigc", "describe"]).expect("`jigc describe` parses");
        // Bare `describe` selects no kind — `Kinds::from_flags` reads that as the whole
        // menu, so the default tour is unchanged.
        assert_eq!(
            cli.command,
            Command::Describe {
                workflows: false,
                doctypes: false,
                commands: false,
            }
        );
    }

    #[test]
    fn describe_format_json_is_selected() {
        let cli = Cli::try_parse_from(["jigc", "describe", "--format", "json"])
            .expect("`jigc describe --format json` parses");
        assert_eq!(cli.format, Format::Json);
        assert_eq!(
            cli.command,
            Command::Describe {
                workflows: false,
                doctypes: false,
                commands: false,
            }
        );
    }

    #[test]
    fn describe_kind_flags_combine() {
        // The filter is a selection, not a mode: the three flags combine freely
        // (`design/introspection.md` → Command surface).
        let cli = Cli::try_parse_from(["jigc", "describe", "--workflows", "--commands"])
            .expect("`jigc describe --workflows --commands` parses");
        assert_eq!(
            cli.command,
            Command::Describe {
                workflows: true,
                doctypes: false,
                commands: true,
            }
        );
    }

    #[test]
    fn describe_rejects_a_positional() {
        // The whole-menu surface takes no positional — a single-item `describe <id>`
        // form is not built (`design/introspection.md` → Command surface).
        let err = Cli::try_parse_from(["jigc", "describe", "single-task"])
            .expect_err("`jigc describe` takes no positional argument");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn config_set_parses_the_key_and_value() {
        let cli = Cli::try_parse_from(["jigc", "config", "set", "default-workflow", "single-task"])
            .expect("`jigc config set <key> <value>` parses");
        assert_eq!(
            cli.command,
            Command::Config {
                verb: ConfigCommand::Set {
                    key: "default-workflow".to_string(),
                    value: "single-task".to_string(),
                },
            }
        );
    }

    #[test]
    fn config_insert_step_parses_workflow_anchor_and_file() {
        let cli = Cli::try_parse_from([
            "jigc",
            "config",
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "implement",
            "./extra.yaml",
        ])
        .expect("`jigc config insert-step ...` parses");
        assert_eq!(
            cli.command,
            Command::Config {
                verb: ConfigCommand::InsertStep {
                    workflow: "single-task".to_string(),
                    after: Some("implement".to_string()),
                    before: None,
                    file: "./extra.yaml".into(),
                },
            }
        );
    }

    #[test]
    fn config_insert_step_rejects_after_and_before_together() {
        let err = Cli::try_parse_from([
            "jigc",
            "config",
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "implement",
            "--before",
            "finalize",
            "./extra.yaml",
        ])
        .expect_err("`--after` together with `--before` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
    }

    #[test]
    fn config_replace_step_parses_the_target_and_file() {
        let cli = Cli::try_parse_from([
            "jigc",
            "config",
            "replace-step",
            "workflow:single-task#implement",
            "./project-implement.yaml",
        ])
        .expect("`jigc config replace-step <target> <file>` parses");
        assert_eq!(
            cli.command,
            Command::Config {
                verb: ConfigCommand::ReplaceStep {
                    target: "workflow:single-task#implement".to_string(),
                    file: "./project-implement.yaml".into(),
                },
            }
        );
    }

    #[test]
    fn config_remove_step_parses_the_target() {
        let cli = Cli::try_parse_from([
            "jigc",
            "config",
            "remove-step",
            "workflow:single-task#superseded-context",
        ])
        .expect("`jigc config remove-step <target>` parses");
        assert_eq!(
            cli.command,
            Command::Config {
                verb: ConfigCommand::RemoveStep {
                    target: "workflow:single-task#superseded-context".to_string(),
                },
            }
        );
    }

    #[test]
    fn config_fill_parses_the_target_and_from_file() {
        let cli = Cli::try_parse_from([
            "jigc",
            "config",
            "fill",
            "step:implement#extra-guidance",
            "--from-file",
            "-",
        ])
        .expect("`jigc config fill <target> --from-file <f>` parses");
        assert_eq!(
            cli.command,
            Command::Config {
                verb: ConfigCommand::Fill {
                    target: "step:implement#extra-guidance".to_string(),
                    from_file: "-".to_string(),
                },
            }
        );
    }

    #[test]
    fn config_fill_requires_from_file() {
        let err = Cli::try_parse_from(["jigc", "config", "fill", "step:implement#extra-guidance"])
            .expect_err("`config fill` with no `--from-file` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn config_fork_parses_the_target() {
        let cli = Cli::try_parse_from(["jigc", "config", "fork", "workflow:single-task#implement"])
            .expect("`jigc config fork <target>` parses");
        assert_eq!(
            cli.command,
            Command::Config {
                verb: ConfigCommand::Fork {
                    target: "workflow:single-task#implement".to_string(),
                },
            }
        );
    }

    #[test]
    fn config_fork_requires_a_target() {
        let err = Cli::try_parse_from(["jigc", "config", "fork"])
            .expect_err("`config fork` with no target must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn config_replace_step_requires_a_file() {
        let err = Cli::try_parse_from(["jigc", "config", "replace-step", "workflow:single-task#x"])
            .expect_err("`replace-step` with no file must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn config_insert_step_requires_an_anchor() {
        let err = Cli::try_parse_from([
            "jigc",
            "config",
            "insert-step",
            "--workflow",
            "single-task",
            "./extra.yaml",
        ])
        .expect_err("`insert-step` with neither `--after` nor `--before` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn config_set_requires_both_key_and_value() {
        let err = Cli::try_parse_from(["jigc", "config", "set", "default-workflow"])
            .expect_err("`config set` with no value must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn milestone_create_parses_the_title() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "create", "Cache rework"])
            .expect("`jigc milestone create <title>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Create {
                    title: "Cache rework".to_string(),
                },
            }
        );
    }

    #[test]
    fn milestone_create_requires_a_title() {
        let err = Cli::try_parse_from(["jigc", "milestone", "create"])
            .expect_err("`milestone create` with no title must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn milestone_add_task_parses_the_id_and_intent() {
        let cli =
            Cli::try_parse_from(["jigc", "milestone", "add-task", "cache-rework", "Zebra fix"])
                .expect("`jigc milestone add-task <id> <intent>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::AddTask {
                    milestone_id: "cache-rework".to_string(),
                    intent: "Zebra fix".to_string(),
                    workflow: "sub-task".to_string(),
                },
            }
        );
    }

    #[test]
    fn milestone_add_task_workflow_parses_explicit_value() {
        let cli = Cli::try_parse_from([
            "jigc",
            "milestone",
            "add-task",
            "cache-rework",
            "Zebra fix",
            "--workflow",
            "single-task",
        ])
        .expect("`jigc milestone add-task <id> <intent> --workflow <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::AddTask {
                    milestone_id: "cache-rework".to_string(),
                    intent: "Zebra fix".to_string(),
                    workflow: "single-task".to_string(),
                },
            }
        );
    }

    #[test]
    fn milestone_add_task_requires_both_id_and_intent() {
        let err = Cli::try_parse_from(["jigc", "milestone", "add-task", "cache-rework"])
            .expect_err("`milestone add-task` with no intent must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn milestone_list_tasks_parses_the_id() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "list-tasks", "cache-rework"])
            .expect("`jigc milestone list-tasks <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::ListTasks {
                    milestone_id: "cache-rework".to_string(),
                },
            }
        );
    }

    #[test]
    fn milestone_provision_parses_the_id() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "provision", "cache-rework"])
            .expect("`jigc milestone provision <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Provision {
                    milestone_id: "cache-rework".to_string(),
                    force: false,
                },
            }
        );
    }

    #[test]
    fn milestone_provision_requires_a_milestone_id() {
        let err = Cli::try_parse_from(["jigc", "milestone", "provision"])
            .expect_err("`milestone provision` with no id must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn milestone_execute_parses_the_id() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "execute", "cache-rework"])
            .expect("`jigc milestone execute <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Execute {
                    milestone_id: "cache-rework".to_string(),
                },
            }
        );
    }

    #[test]
    fn milestone_execute_requires_a_milestone_id() {
        let err = Cli::try_parse_from(["jigc", "milestone", "execute"])
            .expect_err("`milestone execute` with no id must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn milestone_join_parses_the_id() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "join", "cache-rework"])
            .expect("`jigc milestone join <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Join {
                    milestone_id: "cache-rework".to_string(),
                },
            }
        );
    }

    #[test]
    fn milestone_join_requires_a_milestone_id() {
        let err = Cli::try_parse_from(["jigc", "milestone", "join"])
            .expect_err("`milestone join` with no id must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn milestone_finalize_parses_the_id() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "finalize", "cache-rework"])
            .expect("`jigc milestone finalize <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Finalize {
                    milestone_id: "cache-rework".to_string(),
                    carry_staged: false,
                },
            }
        );
    }

    /// The carryover override is opt-in (M43 T4): `--carry-staged` defaults false —
    /// pre-milestone staged state must be *declared*, never implied.
    #[test]
    fn milestone_finalize_parses_carry_staged() {
        let cli = Cli::try_parse_from([
            "jigc",
            "milestone",
            "finalize",
            "cache-rework",
            "--carry-staged",
        ])
        .expect("`jigc milestone finalize <id> --carry-staged` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Finalize {
                    milestone_id: "cache-rework".to_string(),
                    carry_staged: true,
                },
            }
        );
    }

    #[test]
    fn milestone_finalize_requires_a_milestone_id() {
        let err = Cli::try_parse_from(["jigc", "milestone", "finalize"])
            .expect_err("`milestone finalize` with no id must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    /// The abandon path's default is SAFE: `--force` (the consent to destroy a dirty sub-task
    /// worktree's uncommitted work) is opt-in, never implied (M42 Inc 7 / T5).
    #[test]
    fn milestone_discard_parses_the_id_with_force_defaulting_false() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "discard", "cache-rework"])
            .expect("`jigc milestone discard <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Discard {
                    milestone_id: "cache-rework".to_string(),
                    force: false,
                },
            }
        );
    }

    #[test]
    fn milestone_discard_parses_the_force_flag() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "discard", "cache-rework", "--force"])
            .expect("`jigc milestone discard <id> --force` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Discard {
                    milestone_id: "cache-rework".to_string(),
                    force: true,
                },
            }
        );
    }

    #[test]
    fn milestone_discard_requires_a_milestone_id() {
        let err = Cli::try_parse_from(["jigc", "milestone", "discard"])
            .expect_err("`milestone discard` with no id must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn task_finalize_parses_the_id_with_flags_defaulting_false() {
        let cli = Cli::try_parse_from(["jigc", "task", "finalize", "move-cache"])
            .expect("`jigc task finalize <id>` parses");
        assert_eq!(
            cli.command,
            Command::Task {
                verb: TaskCommand::Finalize {
                    id: "move-cache".to_string(),
                    approve: false,
                    dry_run: false,
                    carry_staged: false,
                },
            }
        );
    }

    #[test]
    fn task_finalize_parses_the_dry_run_flag() {
        let cli = Cli::try_parse_from(["jigc", "task", "finalize", "move-cache", "--dry-run"])
            .expect("`jigc task finalize <id> --dry-run` parses");
        assert_eq!(
            cli.command,
            Command::Task {
                verb: TaskCommand::Finalize {
                    id: "move-cache".to_string(),
                    approve: false,
                    dry_run: true,
                    carry_staged: false,
                },
            }
        );
    }

    #[test]
    fn milestone_add_from_spec_parses_the_id_and_spec_address() {
        let cli = Cli::try_parse_from([
            "jigc",
            "milestone",
            "add-from-spec",
            "cache-rework",
            "spec:gateway-rate-limiting",
        ])
        .expect("`jigc milestone add-from-spec <id> <spec-addr>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::AddFromSpec {
                    milestone_id: "cache-rework".to_string(),
                    spec_addr: "spec:gateway-rate-limiting".to_string(),
                    workflow: "sub-task".to_string(),
                },
            }
        );
    }

    #[test]
    fn milestone_add_from_spec_workflow_parses_explicit_value() {
        let cli = Cli::try_parse_from([
            "jigc",
            "milestone",
            "add-from-spec",
            "cache-rework",
            "spec:gateway-rate-limiting",
            "--workflow",
            "single-task",
        ])
        .expect("`jigc milestone add-from-spec <id> <spec-addr> --workflow <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::AddFromSpec {
                    milestone_id: "cache-rework".to_string(),
                    spec_addr: "spec:gateway-rate-limiting".to_string(),
                    workflow: "single-task".to_string(),
                },
            }
        );
    }

    #[test]
    fn milestone_add_from_spec_requires_both_id_and_spec_address() {
        let err = Cli::try_parse_from(["jigc", "milestone", "add-from-spec", "cache-rework"])
            .expect_err("`milestone add-from-spec` with no spec address must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn milestone_rejects_an_unknown_verb() {
        let err = Cli::try_parse_from(["jigc", "milestone", "destroy", "cache-rework"])
            .expect_err("an unknown milestone verb must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::InvalidSubcommand);
    }

    #[test]
    fn workflow_parses_the_id_and_task() {
        let cli = Cli::try_parse_from(["jigc", "workflow", "single-task", "--task", "move-cache"])
            .expect("`jigc workflow <W> --task <id>` parses");
        assert_eq!(
            cli.command,
            Command::Workflow {
                workflow: "single-task".to_string(),
                task: Some("move-cache".to_string()),
                preview: false,
            }
        );
    }

    #[test]
    fn workflow_parses_the_id_and_preview() {
        let cli = Cli::try_parse_from(["jigc", "workflow", "single-task", "--preview"])
            .expect("`jigc workflow <W> --preview` parses");
        assert_eq!(
            cli.command,
            Command::Workflow {
                workflow: "single-task".to_string(),
                task: None,
                preview: true,
            }
        );
    }

    #[test]
    fn workflow_requires_exactly_one_mode() {
        // Neither `--task` nor `--preview` → the required arg-group is unsatisfied.
        let missing = Cli::try_parse_from(["jigc", "workflow", "single-task"])
            .expect_err("`jigc workflow <W>` with neither mode must be rejected");
        assert_eq!(
            missing.kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );

        // Both at once → mutually exclusive within the group.
        let both = Cli::try_parse_from([
            "jigc",
            "workflow",
            "single-task",
            "--task",
            "move-cache",
            "--preview",
        ])
        .expect_err("`--task` and `--preview` together must be rejected");
        assert_eq!(both.kind(), clap::error::ErrorKind::ArgumentConflict);
    }

    #[test]
    fn doc_verbs_take_an_optional_task_selector() {
        // `--task <id>` parses on all three `doc` verbs and is optional (its
        // absence parses too) — the write-time barrier selector
        // (`design/write-commands.md` → The write-time `--task`-scoped barrier).
        let create = Cli::try_parse_from([
            "jigc",
            "doc",
            "create",
            "adr",
            "--title",
            "Pick redis",
            "--task",
            "move-cache",
        ])
        .expect("`doc create … --task <id>` parses");
        assert_eq!(
            create.command,
            Command::Doc {
                verb: DocCommand::Create {
                    r#type: "adr".to_string(),
                    title: "Pick redis".to_string(),
                    slug: None,
                    task: Some("move-cache".to_string()),
                },
            }
        );

        let set_field = Cli::try_parse_from([
            "jigc",
            "doc",
            "set-field",
            "commit:move-cache#type",
            "--value",
            "feat",
            "--task",
            "move-cache",
        ])
        .expect("`doc set-field … --task <id>` parses");
        assert_eq!(
            set_field.command,
            Command::Doc {
                verb: DocCommand::SetField {
                    addr: "commit:move-cache#type".to_string(),
                    value: Some("feat".to_string()),
                    unset: false,
                    task: Some("move-cache".to_string()),
                },
            }
        );

        let set_slot = Cli::try_parse_from([
            "jigc",
            "doc",
            "set-slot",
            "commit:move-cache#summary",
            "--from-file",
            "-",
            "--task",
            "move-cache",
        ])
        .expect("`doc set-slot … --task <id>` parses");
        assert_eq!(
            set_slot.command,
            Command::Doc {
                verb: DocCommand::SetSlot {
                    addr: "commit:move-cache#summary".to_string(),
                    from_file: "-".to_string(),
                    task: Some("move-cache".to_string()),
                },
            }
        );

        let retitle_item = Cli::try_parse_from([
            "jigc",
            "doc",
            "retitle-item",
            "arch-doc:cache-layer#components/session-store",
            "--title",
            "Session vault",
            "--task",
            "move-cache",
        ])
        .expect("`doc retitle-item … --task <id>` parses");
        assert_eq!(
            retitle_item.command,
            Command::Doc {
                verb: DocCommand::RetitleItem {
                    addr: "arch-doc:cache-layer#components/session-store".to_string(),
                    title: "Session vault".to_string(),
                    task: Some("move-cache".to_string()),
                },
            }
        );

        // The selector is optional: omitting it parses, leaving `task: None`.
        let no_task = Cli::try_parse_from([
            "jigc",
            "doc",
            "set-field",
            "commit:x#type",
            "--value",
            "feat",
        ])
        .expect("`doc set-field` with no `--task` parses");
        assert_eq!(
            no_task.command,
            Command::Doc {
                verb: DocCommand::SetField {
                    addr: "commit:x#type".to_string(),
                    value: Some("feat".to_string()),
                    unset: false,
                    task: None,
                },
            }
        );

        // `--unset` parses (no `--value`), leaving `value: None`.
        let unset = Cli::try_parse_from([
            "jigc",
            "doc",
            "set-field",
            "adr:pick-redis#cites-code",
            "--unset",
        ])
        .expect("`doc set-field … --unset` parses");
        assert_eq!(
            unset.command,
            Command::Doc {
                verb: DocCommand::SetField {
                    addr: "adr:pick-redis#cites-code".to_string(),
                    value: None,
                    unset: true,
                    task: None,
                },
            }
        );

        // Exactly one of `--value`/`--unset`: both together conflict, neither is rejected.
        assert!(
            Cli::try_parse_from([
                "jigc",
                "doc",
                "set-field",
                "adr:x#cites-code",
                "--value",
                "y",
                "--unset",
            ])
            .is_err(),
            "`--value` and `--unset` are mutually exclusive"
        );
        assert!(
            Cli::try_parse_from(["jigc", "doc", "set-field", "adr:x#cites-code"]).is_err(),
            "one of `--value`/`--unset` is required"
        );
    }

    #[test]
    fn doc_schema_parses_the_doctype_with_the_global_format() {
        // `jigc doc schema <doctype>` — the third read surface (M40 F1). The global
        // `--format json` selects the separately-pinned contract projection.
        let schema = Cli::try_parse_from(["jigc", "doc", "schema", "adr", "--format", "json"])
            .expect("`doc schema <doctype> --format json` parses");
        assert_eq!(schema.format, Format::Json);
        assert_eq!(
            schema.command,
            Command::Doc {
                verb: DocCommand::Schema {
                    doctype: "adr".to_string(),
                },
            }
        );
    }

    #[test]
    fn doc_list_parses_with_an_optional_doctype_filter() {
        // `jigc doc list [<doctype>] [--task <id>]` — the fourth read surface (M42 T6).
        // The doctype is optional (omit → every persisted doctype); the global
        // `--format json` selects the pinned listing shape; `--task <id>` (M48 inc-3 T3)
        // selects that task's staged surface and is always an explicit id, never inferred.
        let all = Cli::try_parse_from(["jigc", "doc", "list", "--format", "json"])
            .expect("`doc list --format json` parses");
        assert_eq!(all.format, Format::Json);
        assert_eq!(
            all.command,
            Command::Doc {
                verb: DocCommand::List {
                    doctype: None,
                    task: None,
                },
            }
        );
        let one = Cli::try_parse_from(["jigc", "doc", "list", "adr"]).expect("`doc list adr`");
        assert_eq!(
            one.command,
            Command::Doc {
                verb: DocCommand::List {
                    doctype: Some("adr".to_string()),
                    task: None,
                },
            }
        );
        let staged = Cli::try_parse_from(["jigc", "doc", "list", "adr", "--task", "t1"])
            .expect("`doc list adr --task t1`");
        assert_eq!(
            staged.command,
            Command::Doc {
                verb: DocCommand::List {
                    doctype: Some("adr".to_string()),
                    task: Some("t1".to_string()),
                },
            }
        );
    }

    #[test]
    fn start_help_runs_cleanly() {
        let err = Cli::try_parse_from(["jigc", "start", "--help"])
            .expect_err("--help short-circuits parsing");
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    /// Render the long help (`--help`) of a nested subcommand — the emitted text an
    /// agent/human actually reads, not a reconstruction of the doc comments.
    fn long_help(path: &[&str]) -> String {
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        let mut sub = &mut cmd;
        for name in path {
            sub = sub
                .find_subcommand_mut(name)
                .unwrap_or_else(|| panic!("subcommand `{name}` exists"));
        }
        sub.render_long_help().to_string()
    }

    /// (M40 F9) `jigc migrate --help` carries **no** hardcoded doctype enumeration —
    /// the migratable set is derived at runtime from the shipped `migrate-<doctype>`
    /// workflows (12 post-M40), so compile-time clap text enumerating it goes stale
    /// on every pack addition. Both former sites (the verb doc-comment and the `--as`
    /// arg doc) now point at the live set instead, and the verb cross-points `ingest`
    /// (the conformant-adoption sibling).
    #[test]
    fn migrate_help_is_de_enumerated_and_cross_points_ingest() {
        let help = long_help(&["migrate"]);
        for stale in ["`adr`", "`spec`", "`prd`", "arch-doc"] {
            assert!(
                !help.contains(stale),
                "migrate --help must not enumerate doctypes (found {stale}): {help}"
            );
        }
        assert_eq!(
            help.matches("any doctype with a shipped `migrate-<doctype>` workflow")
                .count(),
            2,
            "both former enumeration sites route to the live set: {help}"
        );
        assert!(
            help.contains("see the error message for the live set"),
            "migrate --help points at the runtime-derived set: {help}"
        );
        assert!(
            help.contains("jigc ingest"),
            "migrate --help cross-points ingest: {help}"
        );
    }

    /// (M40 F9) `jigc ingest --help` cross-points `jigc migrate` — the two verbs are
    /// the adopt/rewrite halves of one foreign-doc surface, and neither help text
    /// naming the other was the RC-adoption routing gap.
    #[test]
    fn ingest_help_cross_points_migrate() {
        let help = long_help(&["ingest"]);
        assert!(
            help.contains("jigc migrate"),
            "ingest --help cross-points migrate: {help}"
        );
    }

    /// (M43 inc-5 T2, supersedes the M40 F12 pin) `jigc doc show --help` names the
    /// staged read: the committed read is the **default**, not the only read (the R7
    /// revision) — a doc staged in an open task is read with `--task <id>` — and the
    /// retired `jigc task diff` route is gone from the help.
    #[test]
    fn doc_show_help_names_the_staged_read() {
        let help = long_help(&["doc", "show"]);
        assert!(
            help.contains("--task <id>") && help.contains("staged"),
            "doc show --help names the staged read: {help}"
        );
        assert!(
            !help.contains("jigc task diff"),
            "the retired task-diff route is gone from the help: {help}"
        );
    }

    /// (M47 inc-10 T5 — C3) The **anchor-is-the-item-id** fact is stated on a **read**
    /// surface. It shipped at M42 and was stated only on the *write* verbs' long help
    /// (`add-item` / `retitle-item` / `remove-item`) — nowhere the reader who has just
    /// rendered an item heading stands — so two trial workers inferred it and one
    /// distrusted the inference. `doc show --help` now says it: the `{#id}` anchor is
    /// the `<id>` an address takes, minted from the title rather than equal to it, and
    /// frozen across a retitle.
    #[test]
    fn doc_show_help_names_the_anchor_as_the_item_id() {
        let help = long_help(&["doc", "show"]);
        assert!(
            help.contains("{#id}"),
            "doc show --help names the rendered anchor: {help}"
        );
        for fact in ["id", "address", "frozen"] {
            assert!(
                help.contains(fact),
                "doc show --help connects the anchor to the address id ({fact}): {help}"
            );
        }
    }

    /// (M47 inc-10 triage — the C3 sibling) A **worked example** of the mint is only
    /// as true as the mint rule that ships. The T5 statement above illustrated
    /// *minted-from-the-title-never-equal-to-it* with a release titled `1.0.0`
    /// carrying the id `100` — the **generation-1** id the M42 slug-rule fork retired
    /// (since `SLUG_RULE_VERSION` 2 a `.` is a separator, so the mint is `1-0-0`), and
    /// the word-presence assertions above could not catch it. So the example is no
    /// longer a literal anyone re-checks by eye: every surface that states it is
    /// asserted against `engine::slug::slugify` — **the mint site's own function**
    /// ([`engine::write`]'s `add-item` path calls it) — so the next slug-rule
    /// generation reddens here instead of shipping a fresh lie. The axis is *the
    /// surfaces that state this example*: the read verb's long help and the dev pack's
    /// `record-change` step text (the emitted bytes an agent follows).
    #[test]
    fn the_release_id_worked_example_states_the_shipped_mint() {
        let minted = engine::slug::slugify("1.0.0");
        // The generation-1 id this example carried before slug-rule-version 2.
        let retired = "100";

        for (surface, text) in [
            ("jigc doc show --help", long_help(&["doc", "show"])),
            (
                "the dev pack's `author-change` step",
                include_str!("../pack/steps/author-change.yaml").to_owned(),
            ),
        ] {
            assert!(
                text.contains(&minted),
                "{surface} states the release-id example as the shipped mint \
                 (`slugify(\"1.0.0\") == {minted}`): {text}"
            );
            assert!(
                !text.contains(retired),
                "{surface} states no retired generation-1 id (`{retired}`): {text}"
            );
        }
    }

    /// (M47 inc-10 T9 — N20, half 1) Every id-source-taking **mint** verb's own
    /// usage line names the form that verb takes its id-source in — the mechanical
    /// half of the argument convention (`design/write-commands.md` → The argument
    /// convention). The axis is the six mint verbs: the three doc verbs take a
    /// `--title` flag, the three work-unit verbs take a positional. A positional
    /// verb's usage must additionally carry **no** `--title`, so a later "helpful"
    /// alias cannot make the two forms interchangeable behind the convention's back.
    #[test]
    fn every_mint_verb_names_its_own_id_source_form() {
        let usage_of = |path: &[&str]| -> String {
            let help = long_help(path);
            help.lines()
                .find(|l| l.starts_with("Usage:"))
                .unwrap_or_else(|| panic!("`jigc {}` --help renders a usage line", path.join(" ")))
                .to_string()
        };

        for path in [
            &["doc", "create"][..],
            &["doc", "add-item"][..],
            &["doc", "retitle-item"][..],
        ] {
            let usage = usage_of(path);
            assert!(
                usage.contains("--title <TITLE>"),
                "`jigc {}` is a doc verb — its usage names the `--title` flag: {usage}",
                path.join(" "),
            );
        }

        for (path, positional) in [
            (&["start"][..], "[INTENT]"),
            (&["milestone", "create"][..], "<TITLE>"),
            (&["milestone", "add-task"][..], "<INTENT>"),
        ] {
            let usage = usage_of(path);
            assert!(
                usage.contains(positional),
                "`jigc {}` is a work-unit verb — its usage names the `{positional}` \
                 positional: {usage}",
                path.join(" "),
            );
            assert!(
                !usage.contains("--title"),
                "`jigc {}` must not also offer `--title` — one form per verb is the \
                 convention: {usage}",
                path.join(" "),
            );
        }
    }

    /// (M47 inc-10 T9 — N20, half 2) The two `create` verbs **state** the convention
    /// their forms differ under. `jigc milestone create <TITLE>` beside `jigc doc
    /// create --title` reads as an inconsistency to an agent that has met only one of
    /// them — the rule has been stated in `design/write-commands.md` since M42 and on
    /// neither help, so the surface left the reader to guess (`surface-contract.md` →
    /// law 2, nothing hides). Both now carry the one shared statement, so they cannot
    /// drift into describing two rules.
    #[test]
    fn both_create_verbs_state_the_argument_convention() {
        for path in [&["doc", "create"][..], &["milestone", "create"][..]] {
            let help = long_help(path);
            assert!(
                help.contains(ARGUMENT_CONVENTION),
                "`jigc {}` --help states the argument convention verbatim: {help}",
                path.join(" "),
            );
        }
    }

    /// (M47 inc-10 T5 — C6) The **stdout-purity** guarantee is stated, not merely
    /// observed. A trial worker round-tripped a leaf through a shell file
    /// (`doc show > f` → edit → `set-slot --from-file f`) and had to verify with
    /// `tail -c` that no routing footer rode along, because the behaviour was
    /// documented on no surface they are sanctioned to consult. `doc show --help` now
    /// carries it, with the stream split named.
    #[test]
    fn doc_show_help_states_the_stdout_purity_guarantee() {
        let help = long_help(&["doc", "show"]);
        assert!(
            help.contains("Stdout carries the addressed content and nothing else"),
            "doc show --help states the stdout-purity guarantee: {help}"
        );
        assert!(
            help.contains("no routing footer") && help.contains("stderr"),
            "the guarantee names the footer it excludes and the stream diagnostics take: \
             {help}"
        );
    }
}
