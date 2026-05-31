//! The `jigc` clap command tree + format selection.
//!
//! Argument parsing only: the command tree (`start`, …), the global
//! `--format=agent|json|human` selector, and dispatch to a handler stub. The
//! renderers themselves live in `render`; format *selection* is here, the
//! mapping `Format → renderer` is presentation. See
//! `implementation/module-layout.md` → Renderers (format selection) and
//! `design/write-commands.md` → Task origination (bare `jigc start`).

use crate::doc::DocCommand;
use crate::orient;
use crate::render;
use crate::setup;
use crate::start;
use crate::task::TaskCommand;
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

        /// Resume an existing task by id: re-compose its default workflow over
        /// the task's persisted state (its base + bound context roles), so the
        /// agent picks up context bound since the task was minted (e.g. an ADR
        /// created in-task, now reachable as `task.<role>`). Mutually exclusive
        /// with `<intent>`.
        #[arg(long, conflicts_with = "intent")]
        task: Option<String>,
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

    /// The adapter install. Generates the Claude Code adapter from the embedded
    /// profile: writes the managed `.jigc/AGENT.md` bootstrap and a bare
    /// `@.jigc/AGENT.md` reference into `CLAUDE.md`, initializes the project layer
    /// (`.jigc/config/`), and allowlists `jigc *` in `.claude/settings.json`
    /// (`design/assistant-adapter.md` → Generated, minimal, regenerated).
    /// Idempotent; the install the unset-project orientation routes the agent to.
    Setup,
}

impl Cli {
    /// Dispatch the parsed command: bare `start` (no `intent`) runs the
    /// read-only orientation end-to-end; an `<intent>` mints a task and composes
    /// the cascade's default workflow (`design/write-commands.md` → Task
    /// origination).
    pub fn dispatch(self) -> ExitCode {
        match self.command {
            Command::Start {
                intent: None,
                task: None,
            } => run_orient(self.format),
            Command::Start {
                intent: Some(intent),
                task: None,
            } => run_compose(self.format, &intent),
            Command::Start {
                intent: None,
                task: Some(id),
            } => run_resume(self.format, &id),
            // `conflicts_with` makes clap reject `<intent>` + `--task` together,
            // so this arm is unreachable in practice.
            Command::Start {
                intent: Some(_),
                task: Some(_),
            } => unreachable!("clap rejects `<intent>` together with `--task`"),
            Command::Doc { verb } => run_doc(self.format, verb),
            Command::Task { verb } => run_task(self.format, verb),
            Command::Setup => run_setup(self.format),
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
                task: None,
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
                task: None,
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
                task: Some("add-rate-limiter".to_string()),
            }
        );
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
    fn start_help_runs_cleanly() {
        let err = Cli::try_parse_from(["jigc", "start", "--help"])
            .expect_err("--help short-circuits parsing");
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }
}
