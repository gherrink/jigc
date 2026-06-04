//! The `jigc` clap command tree + format selection.
//!
//! Argument parsing only: the command tree (`start`, …), the global
//! `--format=agent|json|human` selector, and dispatch to a handler stub. The
//! renderers themselves live in `render`; format *selection* is here, the
//! mapping `Format → renderer` is presentation. See
//! `implementation/module-layout.md` → Renderers (format selection) and
//! `design/write-commands.md` → Task origination (bare `jigc start`).

use crate::config::ConfigCommand;
use crate::doc::DocCommand;
use crate::milestone::MilestoneCommand;
use crate::orient;
use crate::render;
use crate::setup;
use crate::start;
use crate::task::TaskCommand;
use crate::upgrade;
use clap::{Parser, Subcommand, ValueEnum};
use std::process::ExitCode;

/// The `jigc` CLI — a context compiler for coding agents.
#[derive(Debug, Parser, PartialEq, Eq)]
#[command(name = "jigc", version, about)]
pub struct Cli {
    /// Output format. Defaults to `agent` — the primary consumer is an agent
    /// reading piped, non-TTY stdout (`implementation/module-layout.md` →
    /// Renderers).
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

/// The `jigc` command tree. Only `start` exists in increment 1 (the front door,
/// `design/write-commands.md` → Task origination); `doc` / `task` land later.
#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Command {
    /// The front door. Bare `jigc start` orients (read-only); an `<intent>`
    /// composes the cascade's default workflow; `--task <id>` resumes an
    /// existing task and re-composes it (`design/write-commands.md` → Task-id
    /// collision & resume; `DECISIONS.md` 2026-05-31 → Four `jigc start` forms).
    Start {
        /// Optional task intent. Absent → orient (read-only); present →
        /// compose the default workflow with `{{task.intent}}` = `<intent>`.
        intent: Option<String>,

        /// Compose a named workflow explicitly, bypassing the cascade default
        /// (`design/write-commands.md` → Task origination, `jigc start
        /// --workflow <X>`). Mints iff the workflow declares `creates-task:
        /// true`. Combines with the `<intent>` positional; mutually exclusive
        /// with `--task`.
        #[arg(long, conflicts_with = "task")]
        workflow: Option<String>,

        /// Resume an existing task by id: re-compose its default workflow over
        /// the task's persisted state (its base + bound context roles), so the
        /// agent picks up context bound since the task was minted (e.g. an ADR
        /// created in-task, now reachable as `task.<role>`). Mutually exclusive
        /// with `<intent>`.
        #[arg(long, conflicts_with = "intent")]
        task: Option<String>,

        /// Show the **resolution tree** instead of composing — the cascade
        /// provenance (`overrides applied: N`), the resolved include list with
        /// each step's winning layer, and the `← replaces … at position`
        /// annotation — WITHOUT minting (the tree is task-independent; `design/
        /// workflow-dialect.md` → `--explain` output contract; `design/worked-
        /// examples.md` → 3a). Combines with the `<intent>` positional and
        /// `--workflow <X>`; mutually exclusive with `--task` (resume re-composes
        /// a minted task, not a task-independent tree).
        #[arg(long, conflicts_with = "task")]
        explain: bool,
    },

    /// The write-path surface — `jigc doc <verb> <addr>` over the active task's
    /// working area (`design/write-commands.md` → The verbs).
    Doc {
        #[command(subcommand)]
        verb: DocCommand,
    },

    /// The task lifecycle surface — `jigc task <verb> <id>` (`diff` · `validate` ·
    /// `discard` · `finalize`) over a named task's working area
    /// (`design/write-commands.md` → Lifecycle; `design/finalize.md` → the commit
    /// boundary).
    Task {
        #[command(subcommand)]
        verb: TaskCommand,
    },

    /// The cascade-authoring surface — `jigc config <verb>` records deltas into the
    /// project layer's `.jigc/config/manifest.yaml` (`design/overrides.md` →
    /// Authoring deltas): `set <key> <value>` (a `scalar-set`, write-time
    /// `check_value`-adjudicated) + the `structural-op` trio `insert-step` /
    /// `replace-step` / `remove-step` (a delta + — for insert/replace — a native
    /// step file, write-time collision/anchor-adjudicated).
    Config {
        #[command(subcommand)]
        verb: ConfigCommand,
    },

    /// The `milestone` work-unit surface — `jigc milestone <verb>` mints a
    /// milestone (`create "<title>"`) and populates its task list (`add-task
    /// <milestone-id> "<intent>"`), the substrate the by-task-id join consumes
    /// (`design/write-commands.md` → Minting a milestone + its task list;
    /// `design/storage.md` → The by-task-id join).
    Milestone {
        #[command(subcommand)]
        verb: MilestoneCommand,
    },

    /// The adapter install. Generates the Claude Code adapter from the embedded
    /// profile: writes the managed `.jigc/AGENT.md` bootstrap and a bare
    /// `@.jigc/AGENT.md` reference into `CLAUDE.md`, initializes the project layer
    /// (`.jigc/config/`), and allowlists `jigc *` in `.claude/settings.json`
    /// (`design/assistant-adapter.md` → Generated, minimal, regenerated).
    /// Idempotent; the install the unset-project orientation routes the agent to.
    Setup,

    /// The upgrade-reconciliation surface. Runs the `override-default` classifier
    /// over every recorded delta against the **current** pack, renders the findings
    /// and routes through the global `--format`, and **blocks** (exits non-zero) on
    /// any blocking finding. **Report-and-route only** — it mutates nothing; the
    /// human resolves each route by re-running the `jigc config` verbs
    /// (`design/overrides.md` → The `jigc upgrade` command).
    Upgrade,
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
            Command::Doc { verb } => run_doc(self.format, verb),
            Command::Task { verb } => run_task(self.format, verb),
            Command::Config { verb } => run_config(verb),
            Command::Milestone { verb } => run_milestone(self.format, verb),
            Command::Setup => run_setup(self.format),
            Command::Upgrade => run_upgrade(self.format),
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
/// finding surfaces on stderr (with its route) and exits non-zero
/// (`design/overrides.md` → Authoring deltas).
fn run_config(verb: ConfigCommand) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    verb.dispatch(&cwd)
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
            eprintln!("{err:#}");
            ExitCode::FAILURE
        }
    }
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
            eprintln!("{err:#}");
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
            eprintln!("{err:#}");
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
            eprintln!("{err:#}");
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
            eprintln!("{err:#}");
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
            eprintln!("{err:#}");
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
    fn start_help_runs_cleanly() {
        let err = Cli::try_parse_from(["jigc", "start", "--help"])
            .expect_err("--help short-circuits parsing");
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }
}
