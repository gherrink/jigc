//! `jigc` — the CLI frontend over the `engine` core, as a **library**.
//!
//! Owns argument parsing, command dispatch, the renderers, adapter generation,
//! and cascade-layer *location* (CLI locates, engine resolves). See
//! `implementation/module-layout.md` → The I/O boundary.
//!
//! The whole module tree lives here rather than in `main.rs` so `tests/*.rs` can
//! reach the **enumeration seam** ([pinning.md](../../../implementation/pinning.md)
//! §1/§2): the pack registry ([`pack`]) for the workflow/doctype sweeps, the clap
//! tree ([`cli`]) for the `CommandFactory` verb sweep, and the exit-code taxonomy
//! table ([`task::EXIT_CODES`]) the taxonomy suite asserts against. `main.rs` keeps
//! only `fn main` and the fd-level output tee, reaching everything through `cli::…`.
//!
//! **Not an API, no semver promise.** This lib is published inside the `jigc` package
//! only because the `jigc` binary is built from it; the binary is the product. Nothing
//! outside this workspace may depend on it, and any release may change it
//! (`implementation/release.md` → Packages and names).

#![doc(hidden)]

// `adapter` ships the embedded profiles + the typed profile model/loader and the
// host-file injectors; `setup` orchestrates them into the `jigc setup` install.
pub mod adapter;
pub mod author;
pub mod cascade_util;
pub mod cli;
pub mod combine;
pub mod config;
pub mod describe;
pub mod doc;
// The gate-coverage table: one source for what `jigc task validate` previews and
// what only `finalize` decides — generated into the composed line, fenced per token
// everywhere else.
pub mod gate_coverage;
pub mod gitignore;
pub mod ingest;
pub mod invocation_log;
pub mod invoke;
pub mod locate;
pub mod migrate;
pub mod migrate_corpus;
pub mod milestone;
pub mod orient;
pub mod orphan;
pub mod pack;
pub mod pack_builtin;
pub mod relocate;
pub mod rename;
pub mod render;
pub mod repo;
pub mod rollback;
pub mod route_fence;
pub mod setup;
pub mod start;
pub mod task;
pub mod trackable;
pub mod unmanage;
pub mod upgrade;
