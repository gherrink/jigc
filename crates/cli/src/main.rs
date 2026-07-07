//! `jigc` — the CLI frontend over the `engine` core.
//!
//! Owns argument parsing, command dispatch, the renderers, adapter generation,
//! and cascade-layer *location* (CLI locates, engine resolves). See
//! `implementation/module-layout.md` → The I/O boundary.

// `adapter` ships the embedded profiles + the typed profile model/loader and the
// host-file injectors; `setup` orchestrates them into the `jigc setup` install.
mod adapter;
mod cascade_util;
mod cli;
mod combine;
mod config;
mod describe;
mod doc;
mod gitignore;
mod ingest;
mod invocation_log;
mod locate;
mod migrate;
mod migrate_corpus;
mod milestone;
mod orient;
mod orphan;
mod pack;
mod rename;
mod render;
mod setup;
mod start;
mod task;
mod unmanage;
mod upgrade;

use clap::Parser;
use invocation_log::Outcome;
use std::process::ExitCode;
use std::time::Instant;

/// Parse and dispatch, then log the invocation (opt-in) and convert to a process exit code.
///
/// The log wrapper **straddles `Cli::try_parse()`**: a clap-rejected usage error exits the
/// dispatch path before it ever runs, so the parse must be fallible and the wrapper must
/// capture that exit-2 outcome too (`design/measurement.md` → The in-repo invocation log,
/// mechanism note). Timing spans the whole run; the knob is resolved from the project cascade
/// independent of argv and no-ops outside a jigc repo (both inside [`invocation_log`]).
fn main() -> ExitCode {
    let started = Instant::now();
    let outcome = match cli::Cli::try_parse() {
        Ok(cli) => cli.dispatch(),
        Err(err) => {
            // Reproduce clap's own behavior: `--help`/`--version` print to stdout and exit 0
            // (`use_stderr()` is false); a genuine usage error prints to stderr and exits 2.
            let code = if err.use_stderr() { 2 } else { 0 };
            let outcome = Outcome::code(code);
            invocation_log::log_invocation(started.elapsed(), &outcome);
            let _ = err.print();
            return outcome.exit_code();
        }
    };
    invocation_log::log_invocation(started.elapsed(), &outcome);
    outcome.exit_code()
}
