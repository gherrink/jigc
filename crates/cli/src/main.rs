//! `jigc` — the CLI frontend over the `engine` core.
//!
//! Owns argument parsing, command dispatch, the renderers, adapter generation,
//! and cascade-layer *location* (CLI locates, engine resolves). See
//! `implementation/module-layout.md` → The I/O boundary.

// `adapter` ships the embedded profiles + the typed profile model/loader; its
// generation surface (`jigc setup` host-file injection) wires in a later inc-6
// task, so the model/loader is `#[allow(dead_code)]` until then (the
// established pre-wire pattern).
#[allow(dead_code)]
mod adapter;
mod cli;
mod doc;
mod locate;
mod orient;
mod pack;
mod render;
mod start;
mod task;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    cli::Cli::parse().dispatch()
}
