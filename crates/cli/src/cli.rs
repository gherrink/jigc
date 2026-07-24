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
use crate::setup;
use crate::start;
use crate::task::TaskCommand;
use crate::unmanage;
use crate::upgrade;
use anyhow::{Context, Result};
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

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

    /// The cascade-authoring surface — `jigc config <verb>` records deltas into
    /// the project layer (`.jigc/config/`).
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
    /// `pre-commit` hooks. Idempotent.
    Setup,

    /// Reverse this project's jigc install — removes `.jigc/`, unwires the
    /// `CLAUDE.md` reference, and drops the `Bash(jigc:*)` permit from
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

    /// Migrate the committed managed corpus onto the current schema — `jigc
    /// migrate-corpus` re-parses every committed doc of a frozen persisted doctype,
    /// applies the deterministic v0→v1 transform (the schema-version stamp + any
    /// structural splice) byte-stable, and writes each doc back only on a clean
    /// conformance gate (the stamp flips last). A prose-needing change is routed to
    /// the agent to author, then re-run. Detect-with `jigc validate`; this migrates.
    /// Disjoint from `jigc migrate` (foreign adoption) and `jigc upgrade` (config).
    /// The verb **lands its own migration** in a pathspec-limited commit; `--no-commit`
    /// leaves the writes unstaged, `--dry-run` writes nothing at all.
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

        /// Override the derived slug (only valid alongside `--to`).
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
    /// cascade. The output is a human menu, not a stable API; don't parse it.
    Describe,

    /// Re-check every committed doc's code anchors against the codebase and report
    /// drift — `jigc validate` is the store-wide, read-only sweep (distinct from
    /// `jigc task validate <id>`, which gates one task). Detect-and-report.
    Validate,
}

impl Cli {
    /// Dispatch the parsed command: bare `start` (no `intent`) runs the
    /// read-only orientation end-to-end; an `<intent>` composes the cascade's
    /// default workflow, minting a task iff that workflow declares
    /// `creates-task: true` (`design/write-commands.md` → Task origination).
    pub fn dispatch(self) -> Outcome {
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
            Command::Setup => run_setup(self.format),
            Command::Uninstall => run_uninstall(self.format),
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
fn run_describe(format: Format) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match describe::run(&cwd) {
        Ok(description) => {
            println!("{}", render::describe(format, &description));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
        }
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
fn run_setup(format: Format) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match setup::run(&cwd) {
        Ok(summary) => {
            println!("{}", render::setup_success(format, &summary));
            Outcome::success()
        }
        Err(finding) => {
            eprintln!("{}", render::setup_block(format, &finding));
            Outcome::failure()
        }
    }
}

/// Run `jigc uninstall` (the repo-local teardown) against the current working
/// directory: locate the repo root, reverse the enumerated repo-local install
/// (remove `.jigc/`, unwire the `CLAUDE.md` reference, drop the `Bash(jigc:*)` allowlist
/// permit — never the machine-global `doc-code` probe), render the outcome through the
/// selected `format`, and map it to the exit code. Success prints a summary on stdout
/// and exits 0; a write failure prints a blocking `uninstall.*` finding (with its
/// route) on stderr and exits non-zero (`design/project-setup.md` → Flow 2 hardening →
/// Teardown / cleanup (G5), bullet (b)).
fn run_uninstall(format: Format) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match setup::run_uninstall(&cwd) {
        Ok(summary) => {
            println!("{}", render::uninstall_success(format, &summary));
            Outcome::success()
        }
        Err(finding) => {
            eprintln!("{}", render::setup_block(format, &finding));
            Outcome::failure()
        }
    }
}

/// Dispatch a `jigc task <verb> <id>` lifecycle verb against the current working
/// directory. The selected `--format` flows through to the finding renderer for
/// `validate`; `diff` / `discard` produce plain output.
fn run_task(format: Format, verb: TaskCommand) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
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
fn run_config(format: Format, verb: ConfigCommand) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    verb.dispatch(&cwd, format)
}

/// Dispatch a `jigc milestone <verb>` action against the current working
/// directory. `create` reads HEAD and mints the milestone area; `add-task` mints a
/// sub-task pinned to the milestone's shared base and appends it. A blocking finding
/// (serial collision, unknown milestone) surfaces on stderr with its route and
/// exits non-zero (`design/write-commands.md` → Minting a milestone).
fn run_milestone(format: Format, verb: MilestoneCommand) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
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
fn run_upgrade(format: Format) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match upgrade::upgrade_in_repo(&cwd) {
        Ok(report) => {
            // The clean line names what the sweep checked (round-2 D4): the recorded
            // config deltas re-applied against the current pack — a separate count
            // read so the report seam stays report-and-route only.
            let checked = upgrade::recorded_delta_count(&cwd).unwrap_or(0);
            println!("{}", render::validation_upgrade(format, &report, checked));
            let code = if report.has_blocking() { 1 } else { 0 };
            Outcome::with_findings(code, &report.findings)
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
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
fn run_ingest(format: Format) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match ingest::run(&cwd) {
        Ok(report) => {
            println!("{}", render::ingest(format, &report));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
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
fn run_migrate(format: Format, path: &str, doctype: &str, slug_override: Option<&str>) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
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
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
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
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
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
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match unmanage::run(&cwd, path) {
        Ok(report) => {
            println!("{}", render::unmanage(format, &report));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
        }
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
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match rename::run(&cwd, old_slug, to, slug) {
        Ok(report) => {
            println!("{}", render::rename(format, &report));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
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
/// The exit code follows the report-only rule with **three exit-flipping exceptions**
/// (`design/validation.md` → Exit semantics): a content-only run (stale-anchor findings,
/// or none) exits **0** (the `jigc ingest` precedent — a blocking *content* finding still
/// exits 0); but a `pack-probe-integrity.*` meta-finding (the probe didn't run, so the
/// sweep can't claim a trustworthy result), a `reconciliation.rename` finding (an
/// out-of-band `git mv` — a structural-identity event this commit introduced, M35
/// Component B), **or** an `engine::validate::SCHEMA_VERSION_CURRENT_CODE` break (M42 — a
/// managed committed instance below its manifest version, i.e. an **unmigrated corpus**,
/// where every other family adjudicated docs against a schema they were never written to;
/// paired since 2026-07-24 with its above-current sibling
/// `engine::validate::SCHEMA_VERSION_AHEAD_CODE` — a future/foreign stamp this build has no
/// schema for, the same untrustworthy-sweep criterion)
/// exits **non-zero** (`Outcome::failure()`, *not* the task-gate `EXIT_VALIDATION_BLOCKED`
/// — this is not a transaction gate). The decision is the shared
/// [`render::validation_store_exit_flips`] (keyed on the probe id / check id **directly**,
/// never on `report.has_blocking()` and never on a route string), so the exit code, the JSON
/// `report_only` field, and the trailer stay truthful in lockstep — a deliberate divergence
/// from the `run_upgrade` / `task validate` idiom.
///
/// The probe **pre-flight** runs before the sweep: an unresolvable `doc-code` probe bails
/// with one operational error here. A locator error (no repo / no project layer) likewise
/// routes to stderr and exits non-zero.
fn run_validate_store(format: Format) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
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
            // The exit rule honors the **three** exit-flipping exceptions (`validation.md` →
            // Exit semantics — report-only, with three exit-flipping exceptions): (1) a
            // `pack-probe-integrity.*` meta-finding means the probe could not be trusted (it
            // crashed / timed out / emitted malformed output), so the sweep cannot claim a
            // result; (2) a `reconciliation.rename` finding (M35, Component B) is an
            // out-of-band `git mv` — a structural-identity event *this commit* introduced,
            // not pre-existing content rot — so it joins the same exit-flipping class; (3) a
            // `schema-conformance.schema-version-current` break (M42) means the corpus is
            // **unmigrated**, so every other family in this very report adjudicated docs
            // against a schema they were never written to — the same untrustworthy-sweep
            // criterion as (1); since 2026-07-24 the exception is a code *pair* with its
            // above-current sibling `schema-conformance.schema-version-ahead` (a
            // future/foreign stamp this build has no schema for — same criterion, inverse
            // direction). Otherwise (content-only or clean) detect-and-report exits 0,
            // even when a content finding blocks. Keyed on the probe id / check id directly,
            // mirrored by [`render::validation_store`]'s `report_only` field + trailer.
            let code = if render::validation_store_exit_flips(&report) {
                1
            } else {
                0
            };
            Outcome::with_findings(code, &report.findings)
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
        }
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

    // docs-root orphan advisory (M36, `design/validation.md` → Orphan detection): a
    // committed `.md` stranded outside the resolved doctype roots after a `docs-root`
    // re-point. The engine's file-state twin only sees docs it has a `FileStateRecord`
    // for; this fills the coverage hole by enumerating `git ls-files` (committed truth
    // that survives a fresh clone) matched on the location-directory basename — never a
    // bare `.md` match (`README.md` stays unflagged).
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
    for (rel, doctype) in crate::orphan::orphaned_docs(&jigc_home, schemas.values()) {
        let finding = if record.get(&rel).is_some() {
            engine::finding::Finding::graded(
                engine::finding::Severity::Advisory,
                "file-state.orphaned-doc",
                format!(
                    "committed doc `{rel}` sits outside the resolved doctype roots — a \
                     `docs-root` change likely stranded it (it looks managed but resolves \
                     under no doctype location)"
                ),
                Some(engine::finding::Location::addressed(rel, 1, 1)),
                Some(
                    "move it under the current resolved root (re-point `docs-root` to cover \
                     it) or drop it with `jigc unmanage`"
                        .into(),
                ),
            )
        } else {
            let migrate_workflow = format!("migrate-{doctype}");
            let migratable = pack
                .list(engine::packsource::PackResourceKind::Workflows)
                .iter()
                .any(|id| *id == engine::packsource::ResourceId::from(migrate_workflow.as_str()));
            let route = crate::orphan::unregistered_route(&rel, &doctype, migratable);
            engine::finding::Finding::graded(
                engine::finding::Severity::Advisory,
                "file-state.unregistered-doc",
                format!(
                    "committed doc `{rel}` looks managed (it sits under a `{doctype}`-style \
                     directory) but was never adopted — a basename coincidence or an \
                     un-ingested foreign doc, not a tracked strand"
                ),
                Some(engine::finding::Location::addressed(rel, 1, 1)),
                Some(route.into()),
            )
        };
        report.findings.push(finding);
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
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    verb.dispatch(&cwd, format)
}

/// Mint a task from `intent` and compose the cascade's default workflow against
/// the current working directory, render the composed view through the selected
/// `format`, and print it — mapping success/failure to the process exit code. A
/// blocking gate / minting finding surfaces on stderr (with its route) and exits
/// non-zero; nothing is emitted past a block.
fn run_compose(format: Format, intent: &str, slug: Option<&str>) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match start::compose_in_repo(&cwd, intent, slug) {
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
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match start::compose_named_in_repo(&cwd, intent, workflow, slug) {
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

/// Compose the explicitly-named `workflow` with **no `<intent>`** against the
/// current working directory — the `jigc start --workflow <X>` form with no
/// positional. A `creates-task: false` `<X>` composes (no mint); a
/// `creates-task: true` `<X>` is rejected with an actionable message (it slugs its
/// task id from the intent), rather than silently falling through to orientation
/// (`design/write-commands.md` → Task origination). Renders the composed view on
/// success (exit 0); a rejection or blocking finding surfaces on stderr (with its
/// route) and exits non-zero.
fn run_compose_named_no_intent(format: Format, workflow: &str) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match start::compose_named_no_intent_in_repo(&cwd, workflow) {
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

/// Render the `--explain` resolution tree against the current working directory:
/// resolve the cascade, build the tree over the resolved default workflow (or the
/// `--workflow <X>`-named one) — WITHOUT minting (the tree is task-independent) —
/// render it through the selected `format`, and print it. A blocking resolution
/// finding (an orphaned anchor, an unknown workflow) surfaces on stderr (with its
/// route) and exits non-zero (`design/workflow-dialect.md` → `--explain` output
/// contract; `design/worked-examples.md` → 3a).
fn run_explain(format: Format, intent: Option<&str>, workflow: Option<&str>) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match start::compose_explain_in_repo(&cwd, intent, workflow) {
        Ok((tree, pack_label)) => {
            println!("{}", render::explain(format, &tree, &pack_label));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
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
fn run_resume(format: Format, id: &str) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match start::resume_in_repo(&cwd, id) {
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

/// Re-enter a milestone sub-task as a fanned sub-agent: compose the
/// explicitly-named `workflow` for sub-task `task` against the current working
/// directory, render the composed view through the selected `format`, and print
/// it. The CLI asserts `workflow` equals the sub-task's recorded mint workflow
/// before composing — a mismatch (a stale launch template) is a blocking finding
/// on stderr (with its route) and exits non-zero; an unknown sub-task or a
/// blocking gate finding likewise surfaces on stderr and exits non-zero
/// (`design/write-commands.md` → Sub-agent re-entry).
fn run_reenter(format: Format, workflow: &str, task: &str) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match start::reenter_in_repo(&cwd, workflow, task) {
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

/// Run `jigc workflow <W> --preview` against the current working directory: compose the
/// `creates-task: true` workflow `<W>`'s step text **without minting a task**, and
/// render it through the mint-first [`render::composed_preview`] surface. A
/// `creates-task: false` `<W>` (the router and its kind) mints nothing to begin with,
/// so [`start::preview_in_repo`] rejects it with the route to run it directly.
fn run_preview(format: Format, workflow: &str) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match start::preview_in_repo(&cwd, workflow) {
        Ok(view) => {
            println!("{}", render::composed_preview(format, &view, workflow));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
        }
    }
}

/// Run the bare-`start` orientation against the current working directory,
/// render the structured result through the selected `format`, and print it —
/// mapping success/failure to the process exit code.
fn run_orient(format: Format) -> Outcome {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return Outcome::failure();
        }
    };
    match orient::orient(&cwd) {
        Ok(view) => {
            println!("{}", render::orientation(format, &view));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
        }
    }
}

/// The honest sibling tip for an unknown-subcommand **semantic guess** — never a
/// silent alias (M43 law 2, `DECISIONS.md` 2026-07-16 Settle, cross-cutting; trial
/// provenance: A1 papercut, log rec 254).
///
/// A curated, code-side map keys on `(parent, guessed verb)` — the guesses agents
/// actually reached for in the trials — and supplies a tip naming what the real
/// sibling **does** (its effect), because clap's bare did-you-mean can steer wrong:
/// a `task discard-write` guesser suggested toward `discard` would destroy the whole
/// task. The tip only *appends* — the guess stays a genuine clap usage error (exit 2,
/// clap's own error + usage output untouched, printed by `main`'s clap-error arm).
///
/// Every command span rides the checked [`Route::mechanical`](engine::finding::Route)
/// constructor, so a tip naming a verb that stops parsing fails the T2 parse fence at
/// construction (`design/surface-contract.md` → The route fence) — this map cannot
/// grow a ghost verb. An uncurated guess (or a curated verb under the wrong parent)
/// returns `None`: inert, never an error.
///
/// `argv` is the raw process argv; the parent is the token immediately preceding the
/// guessed verb, so the map fires only where the guess actually sat (a global flag
/// between parent and guess misses the tip — best-effort by design, the clap error
/// still prints).
pub fn unknown_subcommand_tip(err: &clap::Error, argv: &[String]) -> Option<String> {
    use clap::error::{ContextKind, ContextValue};
    use engine::finding::Route;

    if err.kind() != clap::error::ErrorKind::InvalidSubcommand {
        return None;
    }
    let guess = match err.get(ContextKind::InvalidSubcommand)? {
        ContextValue::String(guess) => guess.as_str(),
        _ => return None,
    };
    let parent = argv
        .windows(2)
        .find(|pair| pair[1] == guess)
        .map(|pair| pair[0].as_str())?;
    let tip = match (parent, guess) {
        ("task", "discard-write") => format!(
            "tip: no per-write discard exists — {}",
            Route::mechanical(
                ["jigc", "task", "discard", "<task-id>"],
                " abandons the WHOLE task (removes its working area and every staged \
                 write); to back out a single external edit, revert that file on disk \
                 instead",
            )
            .as_str()
        ),
        ("task", "status") => format!(
            "tip: {}; {}",
            Route::mechanical(
                ["jigc", "task", "list"],
                " enumerates the active tasks (id + minting workflow + intent)",
            )
            .as_str(),
            Route::mechanical(
                ["jigc", "task", "validate", "<task-id>"],
                " previews the finalize gate for one task — what still blocks",
            )
            .as_str(),
        ),
        _ => return None,
    };
    Some(tip)
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
    #[test]
    fn every_curated_sibling_tip_passes_the_route_parse_fence() {
        crate::route_fence::install();
        for (parent, guess) in [("task", "discard-write"), ("task", "status")] {
            let err = Cli::try_parse_from(["jigc", parent, guess])
                .expect_err("a curated guess is an unknown subcommand");
            let argv: Vec<String> = ["jigc", parent, guess].map(String::from).into();
            let tip = unknown_subcommand_tip(&err, &argv)
                .unwrap_or_else(|| panic!("the curated guess `{parent} {guess}` must yield a tip"));
            assert!(
                tip.starts_with("tip: "),
                "the tip is identifiable as a tip; got: {tip}"
            );
        }
    }

    /// A clap error that is not `InvalidSubcommand` never yields a tip — the map
    /// intercepts semantic guesses only, not ordinary usage errors.
    #[test]
    fn a_non_subcommand_usage_error_yields_no_tip() {
        let err = Cli::try_parse_from(["jigc", "task", "validate"])
            .expect_err("`task validate` with no id is a usage error");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
        let argv: Vec<String> = ["jigc", "task", "validate"].map(String::from).into();
        assert_eq!(unknown_subcommand_tip(&err, &argv), None);
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
    fn milestone_provision_parses_the_id() {
        let cli = Cli::try_parse_from(["jigc", "milestone", "provision", "cache-rework"])
            .expect("`jigc milestone provision <id>` parses");
        assert_eq!(
            cli.command,
            Command::Milestone {
                verb: MilestoneCommand::Provision {
                    milestone_id: "cache-rework".to_string(),
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
        // `jigc doc list [<doctype>]` — the fourth read surface (M42 T6). The doctype is
        // optional (omit → every persisted doctype); the global `--format json` selects the
        // pinned listing shape.
        let all = Cli::try_parse_from(["jigc", "doc", "list", "--format", "json"])
            .expect("`doc list --format json` parses");
        assert_eq!(all.format, Format::Json);
        assert_eq!(
            all.command,
            Command::Doc {
                verb: DocCommand::List { doctype: None },
            }
        );
        let one = Cli::try_parse_from(["jigc", "doc", "list", "adr"]).expect("`doc list adr`");
        assert_eq!(
            one.command,
            Command::Doc {
                verb: DocCommand::List {
                    doctype: Some("adr".to_string()),
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
}
