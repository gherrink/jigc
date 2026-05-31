//! `jigc` — the CLI frontend over the `engine` core.
//!
//! Owns argument parsing, command dispatch, the renderers, adapter generation,
//! and cascade-layer *location* (CLI locates, engine resolves). See
//! `implementation/module-layout.md` → The I/O boundary.

// `adapter` (bootstrap line + allowlist generation) is consumed by `jigc setup`
// in increment 6; it is a `//!`-only stub until then.
mod adapter;
mod cli;
mod locate;
mod orient;
mod pack;
mod render;
mod start;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    cli::Cli::parse().dispatch()
}
