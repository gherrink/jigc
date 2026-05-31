//! Cascade-layer location: pack-default (embedded), team (`~/.config/jigc/`),
//! project (in-repo `.jigc/config/`), plus repo-root discovery.
//!
//! The CLI locates layers and hands the engine its run context; the engine
//! resolves. See `implementation/module-layout.md` → The I/O boundary and
//! `design/overrides.md`.
