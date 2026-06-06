//! The `Probe` seam — the read-only `check(target, ctx) -> [finding]` interface every
//! probe rides ([validation.md](../../../design/validation.md) → The engine / probe
//! boundary, The wire contract).
//!
//! **M10 reshapes the seam.** The M6 seam shipped `check(&self, ctx)` with **no
//! `target`** and a single, live, non-serializable ctx ([`OverrideCtx`] — a borrowed
//! `(deltas, pack)` handle), sized to its one consumer (`override-default`). The wire
//! contract the subprocess (pack-probe) seam rides requires a **`target`** and an
//! **effective-state ctx the engine can serialize** (a live graph in-process, its
//! serialized read-only projection + a path-ref out-of-process — `validation.md` → The
//! wire contract). M10 reshapes the seam to `check(target, ctx)` so the doc-code shape
//! is admissible, **without** coercing `override-default`'s live non-task ctx into a
//! serializable form it has no need to be ([DECISIONS.md](../../../DECISIONS.md)
//! 2026-06-06, M10 inc-2 / T1).
//!
//! **The carrier: a `target` parameter + two associated types** (`Target`, `Ctx<'_>`).
//! Each probe declares *exactly* its own target shape and ctx shape, so the seam admits
//! both shapes with no coercion (the over-generalization trap M3 paid for is avoided by
//! per-probe associated types, never one generic ctx forced over both):
//! - `override-default` is **non-task**: `Target = ()` (target-less) and `Ctx<'a> =
//!   OverrideCtx<'a>` (a live, non-serializable handle) — its M5 reconciliation is
//!   unchanged; the reshape moves only the *call shape*.
//! - the doc-code probe (M10 inc 3+) declares `Target` an [`crate::address::Address`]
//!   and `Ctx` a **serializable** effective-state snapshot (built in inc 3), so an
//!   in-process and a subprocess impl consume the *same logical input* (`validation.md`
//!   → The wire contract). `Ctx<'_>` is a GAT so a borrowing ctx (`OverrideCtx<'a>`) and
//!   a borrowed-snapshot ctx are both expressible.
//!
//! **Severity stays engine-owned, assigned once at the post-pass.** A [`Probe`] only
//! *emits* findings (each carrying its `(probe, check)` handle); the resolved-cascade
//! severity lookup (`validation.override-default.<check>.severity`) is applied
//! downstream by [`crate::result::ValidationReport::new`], never threaded into the
//! probe site (`validation.md` → severity is assigned in one engine-owned pass). So the
//! seam carries no `Resolved`: routing through it changes nothing about how `upgrade`'s
//! findings already tune.

use crate::finding::Finding;
use crate::override_default::{RecordedDeltas, classify};
use crate::packsource::PackSource;

/// The read-only context an override-scoped [`Probe`] checks: the recorded deltas to
/// reconcile and the current (env-selected) pack to reconcile them against — the
/// `(recorded deltas, pack)` shape `override-default` consumes (`validation.md` → one
/// whose ctx is `(recorded deltas, pack)`). This is the **non-task** ctx; the seam
/// admits it as `OverrideDefaultProbe`'s associated `Ctx` without coercing it into the
/// serializable effective-state shape the doc-code probe declares.
pub struct OverrideCtx<'a> {
    /// The project layer's recorded deltas, borrowed for the duration of the check.
    pub deltas: RecordedDeltas<'a>,
    /// The current pack the deltas are reconciled against (pack-direct reads).
    pub pack: &'a dyn PackSource,
}

/// A probe: a read-only `check(target, ctx) -> [finding]` (`validation.md` → The engine
/// / probe boundary). Each probe declares its own [`Target`](Probe::Target) shape and
/// [`Ctx`](Probe::Ctx) shape via associated types, so the seam admits both the non-task
/// `(deltas, pack)` ctx and a serializable effective-state ctx without forcing one
/// generic over both. The engine assigns final severity downstream (the post-pass), so a
/// probe only *suggests* by emitting findings carrying their `(probe, check)` handle
/// ("the engine assigns, the probe suggests").
pub trait Probe {
    /// The address surface this probe checks — an [`crate::address::Address`] for a
    /// targeted probe (doc-code), or `()` for a non-task probe (`override-default`)
    /// whose ctx already names what it reconciles.
    type Target;
    /// The read-only context this probe checks. A GAT so a borrowing ctx
    /// ([`OverrideCtx`]) and a borrowed serializable-snapshot ctx are both expressible.
    type Ctx<'a>;

    /// Check `target` against `ctx`, returning one [`Finding`] per problem found (none
    /// for a clean context). Read-only: a probe reads its inputs and writes nothing.
    fn check<'a>(&self, target: &Self::Target, ctx: Self::Ctx<'a>) -> Vec<Finding>;
}

/// The `override-default` probe routed onto the seam — the seam's first (and, pre-M10,
/// only) real consumer. It delegates to the M5 [`classify`] logic: the reshape changes
/// the *call shape* (a `target` it ignores — its ctx already names what it reconciles),
/// not the reconciliation, so the byte-stable upgrade path is unaffected (`validation.md`
/// → `override-default` is the retrofit).
pub struct OverrideDefaultProbe;

impl Probe for OverrideDefaultProbe {
    /// Target-less: `override-default`'s ctx (`deltas`) already names the override
    /// surface it reconciles, so the seam's `target` is the unit type.
    type Target = ();
    type Ctx<'a> = OverrideCtx<'a>;

    fn check<'a>(&self, _target: &(), ctx: OverrideCtx<'a>) -> Vec<Finding> {
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

        let via_seam = OverrideDefaultProbe.check(
            &(),
            OverrideCtx {
                deltas: mk(),
                pack: &pack,
            },
        );
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
