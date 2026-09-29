//! `engine` — the neutral, domain-empty core of jigc.
//!
//! Owns cascade resolution, the document/schema model, parsing & serialization,
//! the doc registry, workflow composition, the validation engine, task/staging
//! state, and the edge index. Depends on no frontend and no domain content, and
//! makes no LLM calls. See `implementation/module-layout.md` for the topology and
//! `VISION.md` for the determinism boundary these modules enforce.
//!
//! Each module below is a stub home for an increment to fill; the names are taken
//! verbatim from the module-layout dependency graph.

pub mod address;
pub mod cascade;
pub mod catalog;
pub mod compose;
pub mod data_value;
pub mod field_block;
pub mod file_state;
pub mod finalize;
pub mod finding;
pub mod index;
pub mod ingest;
pub mod introspect;
pub mod knobs;
pub mod manifest;
pub mod milestone;
pub mod override_default;
pub mod parse;
pub mod path;
pub mod registry;
pub mod schema;
pub mod schema_diff;
pub mod slug;
pub mod state;
pub mod store;
pub mod target_surface;
pub mod tempname;
pub mod transform;
pub mod validate;
pub mod write;

pub mod packsource;
pub mod probe;
pub mod result;

#[cfg(test)]
mod root_walk;
