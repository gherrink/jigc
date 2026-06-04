//! Cascade helpers shared across the CLI dispatch surface.
//!
//! [`no_delta_resolved`] is the **M6 increment-1 (T2) placeholder** the validate /
//! finalize / upgrade paths pass into the engine's severity post-pass
//! ([`engine::result::ValidationReport::new`]) before those paths resolve a *real*
//! project cascade. A no-delta cascade carries no `scalar-set`, so the post-pass is
//! a no-op and the no-override path stays byte-identical — exactly the determinism
//! guard this increment retires (`design/validation.md` → Severity assignment — the
//! M6 post-pass: the no-override path is byte-identical). T3 (Thread a real
//! `Resolved` into `task.rs`/`finalize`/`jigc upgrade`) replaces these call sites
//! with the project-resolved cascade (`start::resolve_cascade`); until then the
//! post-pass is correctly inert here.

use anyhow::Result;
use engine::cascade::{self, PackDefaultLayer, Resolved};
use std::collections::BTreeMap;

/// A no-delta resolved cascade: an empty pack-default layer, no `team` / `project`
/// layer. The engine severity post-pass reads no override from it, so every emitted
/// finding keeps its severity. The T2 placeholder feeding paths that have not yet
/// resolved their real project cascade (T3).
pub(crate) fn no_delta_resolved() -> Result<Resolved> {
    let pack = PackDefaultLayer::new("", "", BTreeMap::new(), Vec::new());
    Ok(cascade::resolve(&pack, None, None)?)
}
