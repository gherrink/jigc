//! Cascade helpers shared across the CLI dispatch surface.
//!
//! [`no_delta_resolved`] is a no-delta resolved cascade for the finding-emitting
//! paths whose findings are **never inventory checks** — so the engine's severity
//! post-pass ([`engine::result::ValidationReport::new`]) is a guaranteed no-op over
//! them regardless of any recorded override. The `jigc doc` write-time block is the
//! one such path: its block findings are exempt (no inventory row), so a no-delta
//! cascade is the honest input. The cascade-resolving paths (`task validate` /
//! `finalize`, `jigc upgrade`) thread a *real* project-resolved cascade instead
//! (T3 — `start::resolve_severity_cascade{,_resilient}`), since their findings can be
//! tunable inventory checks (`design/validation.md` → Every finding-emitting entry
//! point must resolve the cascade; Severity assignment — the M6 post-pass).

use anyhow::Result;
use engine::cascade::{self, PackDefaultLayer, Resolved};
use std::collections::BTreeMap;

/// A no-delta resolved cascade: an empty pack-default layer, no `team` / `project`
/// layer. The engine severity post-pass reads no override from it, so every emitted
/// finding keeps its severity — the honest input for a path whose findings are exempt
/// from the post-pass (the `jigc doc` write-time block).
pub(crate) fn no_delta_resolved() -> Result<Resolved> {
    let pack = PackDefaultLayer::new("", "", BTreeMap::new(), Vec::new());
    Ok(cascade::resolve(&pack, None, None)?)
}
