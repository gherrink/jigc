//! `jigc` — the CLI frontend over the `engine` core.
//!
//! Owns argument parsing, command dispatch, the renderers, adapter generation,
//! and cascade-layer *location* (CLI locates, engine resolves). See
//! `implementation/module-layout.md` → The I/O boundary.

mod adapter;
// `locate` is consumed by command dispatch (`start`, later in inc 1); until
// `main` wires it in, its public API is exercised only by its own tests.
#[allow(dead_code)]
mod locate;
// `EmbeddedPack` is consumed by command dispatch (later in inc 1); until `main`
// wires it in, its public API is exercised only by its own tests.
#[allow(dead_code)]
mod pack;
mod render;

fn main() {
    // Scaffold entry point. Command dispatch (`start`, `doc`, `task`) lands in
    // the first implementation increment.
}
