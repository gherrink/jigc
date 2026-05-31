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
pub mod finding;
pub mod index;
pub mod parse;
pub mod registry;
pub mod schema;
pub mod slug;
pub mod state;
pub mod validate;

pub mod packsource;
pub mod probe;
pub mod result;
