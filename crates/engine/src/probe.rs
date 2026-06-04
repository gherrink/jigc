//! The non-task `Probe` seam — a first-class entry point for a probe whose ctx is
//! `(recorded deltas, pack)` rather than a task working area
//! ([validation.md](../../../design/validation.md) → The non-task `Probe` seam).
//!
//! M6 stands this up **minimally** — shaped to fit `override-default`, not as a
//! universal trait forced over every existing probe. `validate_task`'s proven inline
//! probes (`file-state`, `schema-conformance`, `ref-resolves`) stay wired as they are;
//! they become severity-tunable purely via the engine post-pass at
//! [`crate::result::ValidationReport::new`], with no rewrite of the byte-stable task
//! path. This seam's job is narrower: give a **non-task-scoped** probe a first-class
//! entry point so it participates in aggregation + that post-pass.
//!
//! **Two ctx shapes, not one trait over both** (`validation.md` → The non-task `Probe`
//! seam). Task-scoped and override-scoped probes consume structurally different inputs;
//! this seam admits the **non-task** shape ([`OverrideCtx`] — `(deltas, pack)`) rather
//! than coercing both into a single generic ctx (the over-generalization trap M3 paid
//! for). A future store-scoped probe extends the seam then, when a real consumer exists.
//!
//! **Severity stays engine-owned, assigned once at the post-pass.** A [`Probe`] only
//! *emits* findings (each carrying its `(probe, check)` handle); the resolved-cascade
//! severity lookup (`validation.override-default.<check>.severity`) is applied
//! downstream by [`crate::result::ValidationReport::new`], never threaded into the
//! probe site (`validation.md` → severity is assigned in one engine-owned pass). So the
//! seam carries no `Resolved`: routing through it changes nothing about how `upgrade`'s
//! findings already tune.
//!
//! The subprocess (pack-probe) seam is post-MVP; this trait must admit it with zero
//! engine change. See `design/validation.md` → the engine/probe boundary and the
//! pack-probe determinism contract.

use crate::finding::Finding;
use crate::override_default::{RecordedDeltas, classify};
use crate::packsource::PackSource;

/// The read-only context an override-scoped [`Probe`] checks: the recorded deltas to
/// reconcile and the current (env-selected) pack to reconcile them against — the
/// `(recorded deltas, pack)` shape `override-default` consumes (`validation.md` → one
/// whose ctx is `(recorded deltas, pack)`). This is the **non-task** ctx; the seam
/// admits it without coercing the task-scoped shape into the same generic.
pub struct OverrideCtx<'a> {
    /// The project layer's recorded deltas, borrowed for the duration of the check.
    pub deltas: RecordedDeltas<'a>,
    /// The current pack the deltas are reconciled against (pack-direct reads).
    pub pack: &'a dyn PackSource,
}

/// A non-task-scoped probe: `check(ctx) -> Vec<Finding>`, read-only. The seam's single
/// method, shaped to the [`OverrideCtx`] non-task ctx — the engine assigns final
/// severity downstream (the post-pass), so a probe only *suggests* by emitting findings
/// carrying their `(probe, check)` handle ("the engine assigns, the probe suggests").
pub trait Probe {
    /// Check the context, returning one [`Finding`] per problem found (none for a clean
    /// context). Read-only: a probe reads its ctx and writes nothing.
    fn check(&self, ctx: OverrideCtx<'_>) -> Vec<Finding>;
}

/// The `override-default` probe routed onto the seam — the seam's first (and M6's only)
/// real consumer, the proof it is non-hollow. It delegates to the M5 [`classify`]
/// logic: routing changes the *call shape* (a `Probe` entry point that participates in
/// aggregation + the post-pass), not the reconciliation, so the byte-stable upgrade
/// path is unaffected (`validation.md` → `override-default` is the retrofit).
pub struct OverrideDefaultProbe;

impl Probe for OverrideDefaultProbe {
    fn check(&self, ctx: OverrideCtx<'_>) -> Vec<Finding> {
        classify(ctx.deltas, ctx.pack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cascade::{Anchor, StructuralDelta, StructuralTarget};
    use crate::packsource::{PackError, PackResourceKind, ResourceId};
    use std::collections::HashMap;

    /// A trivial in-memory `PackSource` carrying step bodies — the engine test-double
    /// pattern, here seeded as the "current pack" the seam classifies deltas against.
    struct FakePack(HashMap<(PackResourceKind, ResourceId), Vec<u8>>);

    impl PackSource for FakePack {
        fn pack_version(&self) -> String {
            "v2".to_owned()
        }
        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            self.0
                .keys()
                .filter(|(k, _)| *k == kind)
                .map(|(_, id)| id.clone())
                .collect()
        }
        fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
            self.0
                .get(&(kind, id.clone()))
                .cloned()
                .ok_or_else(|| PackError::NotFound {
                    kind,
                    id: id.clone(),
                })
        }
    }

    /// The seam delegates faithfully: `OverrideDefaultProbe::check` over an
    /// [`OverrideCtx`] returns **exactly** what [`classify`] returns over the same
    /// `(deltas, pack)`. A `remove` delta over a step the pack omits orphans, so the
    /// probe must surface that one finding — proving the seam routes the real
    /// reconciliation, not a reconstruction. Hardening #5 — the pack OMITS the target.
    #[test]
    fn override_default_probe_delegates_to_classify() {
        let pack = FakePack(HashMap::from([(
            (PackResourceKind::Steps, ResourceId::from("present")),
            b"present body\n".to_vec(),
        )]));

        let structural = vec![StructuralDelta::Remove {
            target: StructuralTarget {
                workflow_id: "single-task".to_owned(),
                anchor: Anchor::At("gone".to_owned()),
            },
        }];
        let mk = || RecordedDeltas {
            structural: &structural,
            forks: &[],
            bases: &[],
            slot_fills: &[],
            scalars: &[],
        };

        let via_seam = OverrideDefaultProbe.check(OverrideCtx {
            deltas: mk(),
            pack: &pack,
        });
        let direct = classify(mk(), &pack);

        assert_eq!(
            via_seam, direct,
            "the seam returns exactly what classify returns over the same ctx",
        );
        assert_eq!(
            via_seam.len(),
            1,
            "the omitted target orphans: {via_seam:?}"
        );
        assert_eq!(via_seam[0].code, "override-default.target-exists");
    }
}
