//! Schema-driven Markdown parse + the surgical-splice write pipeline.
//!
//! The #1 technical risk: lossless, diff-clean round-trip. See
//! `implementation/parsing.md` (offset-splice, never re-stringify, re-parse to
//! validate) and `design/storage.md` for the on-disk format.
