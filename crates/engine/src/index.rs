//! The edge index — a rebuildable map of cross-reference edges (forward, with
//! inverses derived) for cheap forward-ref integrity checks.
//!
//! See `design/storage.md` (edge-index lifecycle) and
//! `design/document-type-schema.md` (bidirectional, inverse derived not stored).
