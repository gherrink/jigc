//! `jigc` — the CLI frontend over the `engine` core.
//!
//! Owns argument parsing, command dispatch, the renderers, adapter generation,
//! and cascade-layer *location* (CLI locates, engine resolves). See
//! `implementation/module-layout.md` → The I/O boundary.

mod adapter;
mod locate;
// `EmbeddedPack` is consumed by the cascade-layer location task (later in inc 1);
// until `main` wires it in, its public API is exercised only by its own tests.
#[allow(dead_code)]
mod pack;
mod render;

fn main() {
    // Scaffold entry point. Command dispatch (`start`, `doc`, `task`) lands in
    // the first implementation increment.
}
