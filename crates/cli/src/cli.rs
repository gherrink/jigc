//! The `jigc` clap command tree + format selection.
//!
//! Argument parsing only: the command tree (`start`, …), the global
//! `--format=agent|json|human` selector, and dispatch to a handler stub. The
//! renderers themselves live in `render`; format *selection* is here, the
//! mapping `Format → renderer` is presentation. See
//! `implementation/module-layout.md` → Renderers (format selection) and
//! `design/write-commands.md` → Task origination (bare `jigc start`).

use crate::orient;
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
    /// composes the cascade's default workflow.
    Start {
        /// Optional task intent. Absent → orient (read-only); present →
        /// compose the default workflow with `{{task.intent}}` = `<intent>`.
        intent: Option<String>,
    },
}

impl Cli {
    /// Dispatch the parsed command. Increment-1 scope: bare `start` (no
    /// `intent`) runs the read-only orientation end-to-end; an `<intent>` is
    /// composition, which lands in increment 3.
    pub fn dispatch(self) -> ExitCode {
        match self.command {
            Command::Start { intent: None } => run_orient(),
            Command::Start { intent: Some(_) } => {
                // Composition (`jigc start "<intent>"`) lands in increment 3.
                eprintln!("composition is not yet implemented (increment 3)");
                ExitCode::FAILURE
            }
        }
    }
}

/// Run the bare-`start` orientation against the current working directory and
/// print it, mapping success/failure to the process exit code.
fn run_orient() -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("cannot determine the current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match orient::orient(&cwd) {
        Ok(text) => {
            println!("{text}");
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
        assert_eq!(cli.command, Command::Start { intent: None });
    }

    #[test]
    fn start_with_intent_parses_the_positional() {
        let cli =
            Cli::try_parse_from(["jigc", "start", "add rate limiter"]).expect("intent parses");
        assert_eq!(
            cli.command,
            Command::Start {
                intent: Some("add rate limiter".to_string()),
            }
        );
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
    fn start_help_runs_cleanly() {
        let err = Cli::try_parse_from(["jigc", "start", "--help"])
            .expect_err("--help short-circuits parsing");
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }
}
