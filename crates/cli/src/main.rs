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
mod ingest;
mod locate;
mod migrate;
mod migrate_corpus;
mod milestone;
mod orient;
mod pack;
mod render;
mod setup;
mod start;
mod task;
mod unmanage;
mod upgrade;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    cli::Cli::parse().dispatch()
}
