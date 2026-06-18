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
use crate::migrate;
use crate::milestone::MilestoneCommand;
use crate::orient;
use crate::render;
use crate::setup;
use crate::start;
use crate::task::TaskCommand;
use crate::unmanage;
use crate::upgrade;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

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
    /// mints a task and composes the default workflow; `--task <id>` resumes an
    /// existing task and re-composes it.
    Start {
        /// Optional task intent. Absent → orient (read-only); present →
        /// compose the default workflow with `{{task.intent}}` = `<intent>`.
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
    },

    /// Re-enter a milestone sub-task as a fanned sub-agent — compose the named
    /// sub-workflow `<W>` for sub-task `<id>`. `<W>` must equal the sub-task's
    /// recorded mint workflow, else the command is rejected. Distinct from
    /// `jigc start --task`, which recomposes a top-level task's own workflow.
    Workflow {
        /// The sub-workflow to compose — the fan-out step's `run:` workflow. Must
        /// equal the sub-task's recorded mint workflow, else rejected.
        workflow: String,

        /// The milestone sub-task to enter (required — re-entry always names its
        /// sub-task).
        #[arg(long)]
        task: String,
    },

    /// Write managed docs — `jigc doc <verb> <addr>` over the active task's
    /// working area.
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

    /// The cascade-authoring surface — `jigc config <verb>` records deltas into
    /// the project layer (`.jigc/config/`).
    Config {
        #[command(subcommand)]
        verb: ConfigCommand,
    },

    /// The milestone work-unit surface — `jigc milestone <verb>` mints a
    /// milestone, builds its sub-task list, and executes, joins, and finalizes it.
    Milestone {
        #[command(subcommand)]
        verb: MilestoneCommand,
    },

    /// Install the Claude Code adapter — writes the `.jigc/AGENT.md` bootstrap and
    /// a reference into `CLAUDE.md`, initializes the project layer (`.jigc/config/`),
    /// allowlists `jigc *`, and installs the `SessionStart` and warn-only git
    /// `pre-commit` hooks. Idempotent.
    Setup,

    /// Reverse this project's jigc install — removes `.jigc/`, unwires the
    /// `CLAUDE.md` reference, and drops the `jigc *` permit from
    /// `.claude/settings.json`. Leaves the machine-global `doc-code` probe (shared
    /// across repos) in place. Idempotent and non-destructive: a second run is a
    /// clean no-op, and your own file content is preserved byte-for-byte.
    Uninstall,

    /// Re-check every recorded config delta against the current pack and report
    /// what needs attention. Report-and-route only — it changes nothing, and exits
    /// non-zero if any finding blocks.
    Upgrade,

    /// Scan the project for existing markdown docs, adopt every conformant one
    /// (register-only — never moving or rewriting a file), and print a triage
    /// report. Misplaced or non-conformant files are flagged for a human; exits 0.
    Ingest,

    /// Rewrite a foreign, non-conformant document into managed shape — `jigc
    /// migrate <path> --as <doctype>`. Mints a migration task and composes the
    /// `migrate-<doctype>` workflow so the agent re-authors the content through the
    /// write verbs (`changelog`, `adr`, `spec`, `prd`, and `arch-doc` all migrate).
    Migrate {
        /// The repo-relative path of the foreign document to migrate (e.g.
        /// `CHANGELOG.md`).
        path: String,

        /// The target managed doctype the foreign document is rewritten into — one of
        /// `changelog`, `adr`, `spec`, `prd`, `arch-doc`.
        #[arg(long = "as")]
        r#as: String,
    },

    /// Drop a single managed doc from jigc's index and state — `jigc unmanage
    /// <path>` removes its edges and file-state baseline, leaving the file bytes on
    /// disk (the inverse of `ingest`'s adopt). Idempotent: a re-run is a clean no-op.
    Unmanage {
        /// The repo-relative path of the managed doc to un-manage (e.g.
        /// `decisions/single-node-cache.md`).
        path: String,
    },

    /// Print a prose tour of what's available — every workflow and doc-type with
    /// their descriptions, plus command-ref hints — reflecting the resolved
    /// cascade. The output is a human menu, not a stable API; don't parse it.
    Describe,

    /// Re-check every committed doc's code anchors against the codebase and report
    /// drift — `jigc validate` is the store-wide, read-only sweep (distinct from
    /// `jigc task validate <id>`, which gates one task). Detect-and-report.
    Validate,
}

impl Cli {
    /// Dispatch the parsed command: bare `start` (no `intent`) runs the
    /// read-only orientation end-to-end; an `<intent>` mints a task and composes
    /// the cascade's default workflow (`design/write-commands.md` → Task
    /// origination).
    pub fn dispatch(self) -> ExitCode {
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
            } => run_explain(self.format, intent.as_deref(), workflow.as_deref()),
            Command::Start {
                intent: None,
                workflow: _,
                task: None,
                explain: false,
            } => run_orient(self.format),
            Command::Start {
                intent: Some(intent),
                workflow: None,
                task: None,
                explain: false,
            } => run_compose(self.format, &intent),
            Command::Start {
                intent: Some(intent),
                workflow: Some(workflow),
                task: None,
                explain: false,
            } => run_compose_named(self.format, &intent, &workflow),
            Command::Start {
                intent: None,
                workflow: _,
                task: Some(id),
                explain: false,
            } => run_resume(self.format, &id),
            // `conflicts_with` makes clap reject `<intent>` + `--task` and
            // `--explain` + `--task` together, so these arms are unreachable.
            Command::Start {
                intent: Some(_),
                workflow: _,
                task: Some(_),
                explain: _,
            }
            | Command::Start {
                task: Some(_),
                explain: true,
                ..
            } => unreachable!("clap rejects `<intent>`/`--explain` together with `--task`"),
            Command::Workflow { workflow, task } => run_reenter(self.format, &workflow, &task),
            Command::Doc { verb } => run_doc(self.format, verb),
            Command::Task { verb } => run_task(self.format, verb),
            Command::Config { verb } => run_config(self.format, verb),
            Command::Milestone { verb } => run_milestone(self.format, verb),
            Command::Setup => run_setup(self.format),
            Command::Uninstall => run_uninstall(self.format),
            Command::Upgrade => run_upgrade(self.format),
            Command::Ingest => run_ingest(self.format),
            Command::Migrate { path, r#as } => run_migrate(self.format, &path, &r#as),
            Command::Unmanage { path } => run_unmanage(self.format, &path),
            Command::Describe => run_describe(self.format),
            Command::Validate => run_validate_store(self.format),
        }
    }
}

/// Run `jigc describe` against the current working directory: locate the repo +
/// project layer, build the **pack-only** resolved definitions (the unfiltered
/// workflow set + the full doctype set + the command catalog), assemble the
/// whole-menu projection, render it through the selected `format`, and print it. A
/// clean run exits 0; a locator error (no repo / no project layer) routes to stderr
/// and exits non-zero (`design/introspection.md` → Command surface). Reads-only —
/// it composes nothing and writes nothing.
fn run_describe(format: Format) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match describe::run(&cwd) {
        Ok(description) => {
            println!("{}", render::describe(format, &description));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
    }
}

/// Run `jigc setup` (the adapter install) against the current working directory:
/// locate the repo root, install the Claude Code adapter (bootstrap reference +
/// project-layer init + `jigc *` allowlist), render the outcome through the
/// selected `format`, and map
/// it to the exit code. Success prints a summary on stdout and exits 0; a write
/// failure prints a blocking `setup.*` finding (with its route) on stderr and
/// exits non-zero (`design/assistant-adapter.md` → Generated, minimal,
/// regenerated; the block-payload envelope, `DECISIONS.md` 2026-05-31).
fn run_setup(format: Format) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match setup::run(&cwd) {
        Ok(summary) => {
            println!("{}", render::setup_success(format, &summary));
            ExitCode::SUCCESS
        }
        Err(finding) => {
            eprintln!("{}", render::setup_block(format, &finding));
            ExitCode::FAILURE
        }
    }
}

/// Run `jigc uninstall` (the repo-local teardown) against the current working
/// directory: locate the repo root, reverse the enumerated repo-local install
/// (remove `.jigc/`, unwire the `CLAUDE.md` reference, drop the `jigc *` allowlist
/// permit — never the machine-global `doc-code` probe), render the outcome through the
/// selected `format`, and map it to the exit code. Success prints a summary on stdout
/// and exits 0; a write failure prints a blocking `uninstall.*` finding (with its
/// route) on stderr and exits non-zero (`design/project-setup.md` → Flow 2 hardening →
/// Teardown / cleanup (G5), bullet (b)).
fn run_uninstall(format: Format) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match setup::run_uninstall(&cwd) {
        Ok(summary) => {
            println!("{}", render::uninstall_success(format, &summary));
            ExitCode::SUCCESS
        }
        Err(finding) => {
            eprintln!("{}", render::setup_block(format, &finding));
            ExitCode::FAILURE
        }
    }
}

/// Dispatch a `jigc task <verb> <id>` lifecycle verb against the current working
/// directory. The selected `--format` flows through to the finding renderer for
/// `validate`; `diff` / `discard` produce plain output.
fn run_task(format: Format, verb: TaskCommand) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
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
fn run_config(format: Format, verb: ConfigCommand) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    verb.dispatch(&cwd, format)
}

/// Dispatch a `jigc milestone <verb>` action against the current working
/// directory. `create` reads HEAD and mints the milestone area; `add-task` mints a
/// sub-task pinned to the milestone's shared base and appends it. A blocking finding
/// (serial collision, unknown milestone) surfaces on stderr with its route and
/// exits non-zero (`design/write-commands.md` → Minting a milestone).
fn run_milestone(format: Format, verb: MilestoneCommand) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
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
fn run_upgrade(format: Format) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match upgrade::upgrade_in_repo(&cwd) {
        Ok(report) => {
            println!("{}", render::validation(format, &report));
            if report.has_blocking() {
                ExitCode::FAILURE
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

/// Run `jigc ingest` against the current working directory: locate the repo root,
/// discover candidate markdown, classify each against the persisted schemas, adopt
/// every conformant `adoptable` candidate **register-only** (record it into the edge
/// index + file-state baseline — never moving or rewriting a candidate file), and
/// render the triage report through the selected `format`. A clean scan exits 0; a
/// locator error (no repo / no project layer) routes to stderr and exits non-zero
/// (`design/project-setup.md` → Flow 2; `design/worked-examples.md` → flow 12).
fn run_ingest(format: Format) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match ingest::run(&cwd) {
        Ok(report) => {
            println!("{}", render::ingest(format, &report));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
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
fn run_migrate(format: Format, path: &str, doctype: &str) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    migrate::run(&cwd, path, doctype, format)
}

/// Run `jigc unmanage <rel_path>` against the current working directory: locate the
/// repo + project layer, load the index + file-state record, drop the doc's forward
/// edges + its file-state baseline (register-only — the file bytes are left on disk),
/// and render the outcome through the selected `format`. A clean run (a real drop *or*
/// an idempotent no-op) exits 0; a locator error (no repo / no project layer) routes to
/// stderr and exits non-zero (`design/project-setup.md` → Flow 2 hardening → Teardown /
/// cleanup (G5)).
fn run_unmanage(format: Format, path: &str) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match unmanage::run(&cwd, path) {
        Ok(report) => {
            println!("{}", render::unmanage(format, &report));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
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
/// The exit code follows the **two-class rule** (`design/validation.md` → Severity → the
/// one exit-code exception; review B1): a content-only run (stale-anchor findings, or
/// none) exits **0** (the `jigc ingest` precedent — a blocking *content* finding still
/// exits 0); **any** `pack-probe-integrity.*` meta-finding present exits **non-zero**
/// (`ExitCode::FAILURE`, *not* the task-gate `EXIT_VALIDATION_BLOCKED` — this is not a
/// transaction gate, it is the command honestly reporting the probe didn't run, so the
/// sweep can't claim a trustworthy result). The rule keys on the `pack-probe-integrity`
/// probe id **directly**, never on `report.has_blocking()` — a deliberate divergence from
/// the `run_upgrade` / `task validate` idiom.
///
/// The probe **pre-flight** runs before the sweep: an unresolvable `doc-code` probe bails
/// with one operational error here. A locator error (no repo / no project layer) likewise
/// routes to stderr and exits non-zero.
fn run_validate_store(format: Format) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match validate_store_in_repo(&cwd) {
        Ok(report) => {
            println!("{}", render::validation_store(format, &report));
            // The two-class exit: a `pack-probe-integrity.*` meta-finding means the probe
            // could not be trusted (it crashed / timed out / emitted malformed output), so
            // the sweep cannot claim a result — exit non-zero. Otherwise (content-only or
            // clean) detect-and-report exits 0, even when a content finding blocks.
            let probe_unreliable = report
                .findings
                .iter()
                .any(|f| f.probe == "pack-probe-integrity");
            if probe_unreliable {
                ExitCode::FAILURE
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

/// Locate the repo + project layer from `cwd`, **pre-flight the `doc-code` probe**, then
/// build the resolved schemas (keyed by doctype) + severity cascade and run the
/// committed-store sweep against the production `doc-code` invoker. The pre-flight bails
/// before any sweep work when the probe is unresolvable, so a missing probe is one
/// operational error, never N per-anchor crash meta-findings. Mirrors `run_ingest`'s
/// locate preamble + `task.rs`'s `schemas()` / `resolve_severity_cascade` idiom; the
/// engine stays domain-empty (the CLI feeds the pack in).
fn validate_store_in_repo(cwd: &Path) -> Result<engine::result::ValidationReport> {
    let repo_root = require_project_layer(cwd)?;
    require_doc_code_probe()?;
    let pack = crate::pack::make_pack();
    let pack = pack.as_ref();
    let project_config = repo_root.join(".jigc").join("config");
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
    let jigc_root = repo_root.join(".jigc");
    let record = engine::file_state::FileStateRecord::load(&jigc_root)
        .with_context(|| format!("loading the file-state record at {jigc_root:?}"))?;

    engine::validate::validate_store_families(
        &repo_root,
        &schemas,
        &resolved,
        &crate::task::doc_code_invoker,
        &workflows,
        &workflow_source,
        &record,
    )
    .with_context(|| format!("validating the committed store at {repo_root:?}"))
}

/// The store sweep's **probe pre-flight**: resolve the `doc-code` probe program (the
/// `JIGC_DOC_CODE_PROBE` override else the `<bin-dir>/doc-code` sibling) and require it
/// to be an existing file before the sweep runs. A missing probe is a single
/// misconfiguration to report once — not a `crash` meta-finding per anchor — so this
/// bails with **one** operational error naming the resolved path + the override knob,
/// and the handler routes it to stderr with a non-zero exit (`design/validation.md` →
/// Distribution bound; review S2).
fn require_doc_code_probe() -> Result<()> {
    let program = ::cli::invoke::doc_code_program();
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
        anyhow::bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }
    Ok(ctx.repo_root)
}

/// Dispatch a `jigc doc <verb>` write against the active task in the current
/// working directory. The `doc` surface is agent-facing structured I/O (the
/// staged buffer + the route on a block); a **blocking finding** renders through
/// the global `--format` (a JSON envelope under `--format json`), so an agent on
/// `--format json` gets a parseable block (`design/write-commands.md` → The verbs).
fn run_doc(format: Format, verb: DocCommand) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    verb.dispatch(&cwd, format)
}

/// Mint a task from `intent` and compose the cascade's default workflow against
/// the current working directory, render the composed view through the selected
/// `format`, and print it — mapping success/failure to the process exit code. A
/// blocking gate / minting finding surfaces on stderr (with its route) and exits
/// non-zero; nothing is emitted past a block.
fn run_compose(format: Format, intent: &str) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match start::compose_in_repo(&cwd, intent) {
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

/// Compose the explicitly-named `workflow` from `intent` against the current
/// working directory — the `jigc start --workflow <X> "<intent>"` front door
/// (Form D, `design/write-commands.md` → Task origination). Bypasses the cascade
/// default and composes the *named* workflow, minting iff it declares
/// `creates-task: true`. Renders the composed view through the selected `format`
/// and maps success/failure to the exit code: a clean compose prints on stdout
/// and exits 0; an unknown `<X>` or a blocking gate finding surfaces on stderr
/// (with its route) and exits non-zero — nothing is emitted past a block, and an
/// unknown id is rejected before any mint.
fn run_compose_named(format: Format, intent: &str, workflow: &str) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match start::compose_named_in_repo(&cwd, intent, workflow) {
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

/// Render the `--explain` resolution tree against the current working directory:
/// resolve the cascade, build the tree over the resolved default workflow (or the
/// `--workflow <X>`-named one) — WITHOUT minting (the tree is task-independent) —
/// render it through the selected `format`, and print it. A blocking resolution
/// finding (an orphaned anchor, an unknown workflow) surfaces on stderr (with its
/// route) and exits non-zero (`design/workflow-dialect.md` → `--explain` output
/// contract; `design/worked-examples.md` → 3a).
fn run_explain(format: Format, intent: Option<&str>, workflow: Option<&str>) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match start::compose_explain_in_repo(&cwd, intent, workflow) {
        Ok((tree, pack_label)) => {
            println!("{}", render::explain(format, &tree, &pack_label));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
    }
}

/// Resume an existing task by id and re-compose its default workflow against the
/// current working directory, render the composed view through the selected
/// `format`, and print it. The re-compose reads the task's persisted bound
/// context roles (`.jigc/tasks/<id>/roles.json`), so a role bound since the task
/// was minted (e.g. an in-task ADR) resolves in the composed view. A missing task
/// or a blocking gate finding surfaces on stderr (with its route) and exits
/// non-zero.
fn run_resume(format: Format, id: &str) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match start::resume_in_repo(&cwd, id) {
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

/// Re-enter a milestone sub-task as a fanned sub-agent: compose the
/// explicitly-named `workflow` for sub-task `task` against the current working
/// directory, render the composed view through the selected `format`, and print
/// it. The CLI asserts `workflow` equals the sub-task's recorded mint workflow
/// before composing — a mismatch (a stale launch template) is a blocking finding
/// on stderr (with its route) and exits non-zero; an unknown sub-task or a
/// blocking gate finding likewise surfaces on stderr and exits non-zero
/// (`design/write-commands.md` → Sub-agent re-entry).
fn run_reenter(format: Format, workflow: &str, task: &str) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match start::reenter_in_repo(&cwd, workflow, task) {
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

/// Run the bare-`start` orientation against the current working directory,
/// render the structured result through the selected `format`, and print it —
/// mapping success/failure to the process exit code.
fn run_orient(format: Format) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match orient::orient(&cwd) {
        Ok(view) => {
            println!("{}", render::orientation(format, &view));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod cli_parse {
    use super::*;
    use clap::Parser;

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
        assert_eq!(cli.command, Command::Setup);
    }

    #[test]
    fn uninstall_parses() {
        let cli = Cli::try_parse_from(["jigc", "uninstall"]).expect("`jigc uninstall` parses");
        assert_eq!(cli.command, Command::Uninstall);
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
    fn describe_parses() {
        let cli = Cli::try_parse_from(["jigc", "describe"]).expect("`jigc describe` parses");
        assert_eq!(cli.command, Command::Describe);
    }

    #[test]
    fn describe_format_json_is_selected() {
        let cli = Cli::try_parse_from(["jigc", "describe", "--format", "json"])
            .expect("`jigc describe --format json` parses");
        assert_eq!(cli.format, Format::Json);
        assert_eq!(cli.command, Command::Describe);
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
    fn workflow_parses_the_id_and_required_task() {
        let cli = Cli::try_parse_from(["jigc", "workflow", "single-task", "--task", "move-cache"])
            .expect("`jigc workflow <W> --task <id>` parses");
        assert_eq!(
            cli.command,
            Command::Workflow {
                workflow: "single-task".to_string(),
                task: "move-cache".to_string(),
            }
        );
    }

    #[test]
    fn workflow_requires_the_task_flag() {
        let err = Cli::try_parse_from(["jigc", "workflow", "single-task"])
            .expect_err("`jigc workflow <W>` with no `--task` must be rejected");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
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
                    value: "feat".to_string(),
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
                    value: "feat".to_string(),
                    task: None,
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
}
