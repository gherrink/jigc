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
use crate::result::SCHEMA_VERSION;
use crate::target_surface::TargetAnchor;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The **request** envelope the engine sends a subprocess pack probe — JSON on the
/// probe's stdin ([validation.md](../../../design/validation.md) → The wire contract,
/// line 106: `{ probe_id, target, effective_state: { snapshot_path }, config,
/// schema_version }`). It is the **concrete Rust projection of the locked envelope**;
/// the struct shapes + serde form are a doc-elaboration pin within it
/// ([DECISIONS.md](../../../DECISIONS.md) 2026-06-06, M10 inc-3 / T1).
///
/// The snapshot is carried **by path-ref** ([`ProbeEffectiveState::snapshot_path`]),
/// never inlined — the engine materialized the [`EffectiveStateSnapshot`] in the
/// probe's read scope (inc 2) and the request names its path (`validation.md`:101:
/// "the request carries a path-ref to it"). The CLI invoker (T2) writes this to the
/// probe's stdin; the probe never calls back.
///
/// **`target`** is the one address grammar ([structural-grammar.md](../../../design/structural-grammar.md#addressing))
/// in its **string form** — the wire carries the flat URI a probe reads, not the
/// engine's internal [`crate::address::Address`] enum (the snapshot already addresses
/// each anchor as a string, [`TargetAnchor::address`]). **`schema_version`** is present
/// **as a field** so the contract can evolve without silently breaking a probe built
/// against an older engine; the *versioning policy itself stays deferred*
/// (`validation.md`:112; [module-layout.md](../../../implementation/module-layout.md)
/// → Open questions).
///
/// **Field order is pinned** (`probe_id`, `target`, `effective_state`, `config`,
/// `schema_version`) — the serialized form is the contract a probe is built against, so
/// a reorder is a breaking change the golden ([`tests::request_json_projection_is_the_pinned_envelope`])
/// catches.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeRequest {
    /// The probe being invoked (e.g. `doc-code`) — the `probe` half of the `(probe,
    /// check)` handle the engine keys severity on.
    pub probe_id: String,
    /// The address the probe adjudicates, in the one address grammar's string form.
    pub target: String,
    /// The read-only effective-state the probe reads — carried **by path-ref**.
    pub effective_state: ProbeEffectiveState,
    /// The cascade-resolved probe config (an opaque JSON object the probe interprets);
    /// empty when the cascade resolves none.
    pub config: serde_json::Map<String, serde_json::Value>,
    /// The wire-contract schema version (the contract-evolution marker; policy deferred).
    pub schema_version: u32,
}

/// The `effective_state` member of a [`ProbeRequest`]: the **path-ref** to the
/// materialized [`EffectiveStateSnapshot`] (`validation.md`:101 — a subprocess holds no
/// live graph handle and reaches the engine over no socket, so the engine materializes
/// a read-only snapshot in the probe's read scope and the request names its path).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeEffectiveState {
    /// The filesystem path of the materialized snapshot the probe reads.
    pub snapshot_path: PathBuf,
}

/// The **response** envelope a subprocess pack probe returns — JSON on the probe's
/// stdout ([validation.md](../../../design/validation.md) → The wire contract, line
/// 107: `{ findings: [<Finding>], schema_version }`). The engine deserializes this from
/// the probe's stdout (T3 ingests `findings` into the [`crate::result::ValidationReport`]).
///
/// **`findings`** are the engine's **one [`Finding`] shape** ([finding.rs](finding.rs))
/// — a probe emits findings; the engine assigns the final severity downstream
/// (severity is engine-owned, never baked into a probe — `validation.md`:111). A
/// response **round-trips** serialize → deserialize → equal
/// ([`tests::response_round_trips_through_serde`]). **`schema_version`** mirrors the
/// request's contract-evolution marker.
///
/// **Field order is pinned** (`findings`, `schema_version`) for the same contract reason
/// as [`ProbeRequest`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeResponse {
    /// The findings the probe emits — the engine's one [`Finding`] shape.
    pub findings: Vec<Finding>,
    /// The wire-contract schema version (mirrors the request's marker).
    pub schema_version: u32,
}

impl ProbeRequest {
    /// Build a request for `probe_id` over `target`, carrying the snapshot by
    /// `snapshot_path` and the cascade-resolved `config`, stamping the current
    /// [`SCHEMA_VERSION`]. The engine's request-construction step (the CLI invoker, T2,
    /// serializes this to the probe's stdin).
    pub fn new(
        probe_id: impl Into<String>,
        target: impl Into<String>,
        snapshot_path: PathBuf,
        config: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        Self {
            probe_id: probe_id.into(),
            target: target.into(),
            effective_state: ProbeEffectiveState { snapshot_path },
            config,
            schema_version: SCHEMA_VERSION,
        }
    }
}

impl ProbeResponse {
    /// A response carrying `findings`, stamping the current [`SCHEMA_VERSION`] — the
    /// shape a well-behaved probe emits (T3 deserializes the probe's stdout into this).
    pub fn new(findings: Vec<Finding>) -> Self {
        Self {
            findings,
            schema_version: SCHEMA_VERSION,
        }
    }
}

/// The read-only **effective-state snapshot** the engine materializes for a doc-code
/// probe — the serializable ctx the wire contract carries **by path-ref**
/// (`validation.md` → The wire contract; line 103: "What the snapshot must carry for
/// `doc-code`"). It is built from the [`crate::target_surface`] enumeration: the
/// `(target-address, anchor-value, check-id)` pairs to adjudicate plus the
/// **working-tree root** the anchors resolve against.
///
/// The snapshot carries **no code** — by rule 4 of the determinism contract the
/// probe reads the cited code directly from the repo at [`working_tree_root`](Self::working_tree_root)
/// (`validation.md`:103: "the code is read directly from the repo per rule 4, not
/// copied into the snapshot"). So an in-process and a subprocess probe consume the
/// *same logical input*: the in-process probe holds the live struct, the subprocess
/// probe reads its serialized form from the path-ref (inc 3) — the serde form is the
/// shared contract, unit-proven by [`tests::snapshot_round_trips_through_serde`].
///
/// **Field order is pinned** (`anchors` then `working_tree_root`) — a doc-elaboration
/// pin within the locked snapshot model ([DECISIONS.md](../../../DECISIONS.md)
/// 2026-06-06, M10 inc-2 / T3): the serialized form is a stable contract a probe is
/// built against, so a reorder is a breaking change a golden must catch.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveStateSnapshot {
    /// The enumerated target surface — every `code-anchor` leaf over the task's
    /// effective-state docs, address-sorted ([`crate::target_surface::enumerate_target_surface`]).
    pub anchors: Vec<TargetAnchor>,
    /// The working-tree root the anchors' `<path>#<symbol>` values resolve against
    /// (the code is read from here directly, never copied into the snapshot).
    pub working_tree_root: PathBuf,
}

impl EffectiveStateSnapshot {
    /// Build the snapshot from the enumerated `anchors` and the `working_tree_root`
    /// they resolve against — the engine's materialization step (the wire carries it
    /// by path-ref out-of-process; inc 3 writes the path-ref hand-off).
    pub fn new(anchors: Vec<TargetAnchor>, working_tree_root: PathBuf) -> Self {
        Self {
            anchors,
            working_tree_root,
        }
    }
}

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
    use crate::address::Address;
    use crate::cascade::{Anchor, StructuralDelta, StructuralTarget};
    use crate::finding::{Location, Severity};
    use crate::packsource::{PackError, PackResourceKind, ResourceId};
    use std::collections::HashMap;

    /// The **request** envelope is the pinned wire contract (the T1 done-criterion): a
    /// request projects to exactly `{ probe_id, target, effective_state: {
    /// snapshot_path }, config, schema_version }` — in that field order, the snapshot
    /// carried **by path-ref** (never inlined), `config` an object, `schema_version`
    /// present. The golden pins the serialized *string* (not a key-sorted value), so it
    /// also locks field order; a rename, a reorder, an inlined snapshot, or a serde slip
    /// breaks it. That is the contract both sides bind to.
    #[test]
    fn request_json_projection_is_the_pinned_envelope() {
        let config = serde_json::Map::from_iter([(
            "is-a-test".to_string(),
            serde_json::Value::String("rust-first".to_string()),
        )]);
        let request = ProbeRequest::new(
            "doc-code",
            "adr:single-node-cache#status/cites-code",
            PathBuf::from("/scratch/snapshot.json"),
            config,
        );

        let json = serde_json::to_string_pretty(&request).expect("serializes");

        insta::assert_snapshot!(json, @r#"
        {
          "probe_id": "doc-code",
          "target": "adr:single-node-cache#status/cites-code",
          "effective_state": {
            "snapshot_path": "/scratch/snapshot.json"
          },
          "config": {
            "is-a-test": "rust-first"
          },
          "schema_version": 2
        }
        "#);
    }

    /// A request with **no resolved config** projects `config` as an empty object `{}`
    /// (the cascade resolved none) — the wire member is always present, never absent,
    /// so a probe reads a stable shape. And the request **round-trips** serialize →
    /// deserialize → equal.
    #[test]
    fn request_round_trips_with_empty_config() {
        let request = ProbeRequest::new(
            "doc-code",
            "spec:rate-limiting#criteria/rate-limit/maps-to-test",
            PathBuf::from("/scratch/snapshot.json"),
            serde_json::Map::new(),
        );

        let json = serde_json::to_value(&request).expect("serializes");
        assert_eq!(
            json["config"],
            serde_json::json!({}),
            "an unresolved config is an empty object, not absent",
        );

        let back: ProbeRequest = serde_json::from_value(json).expect("deserializes");
        assert_eq!(
            back, request,
            "the request round-trips serialize -> deserialize -> equal"
        );
    }

    /// The **response** envelope (the T1 done-criterion): a response carrying the
    /// engine's one [`Finding`] shape projects to exactly `{ findings: [<Finding>],
    /// schema_version }` in that field order, and **round-trips** serialize →
    /// deserialize → equal. The `findings` member deserializes back into the engine's
    /// one `Finding` shape — proving a probe's emitted findings ingest unchanged (T3).
    #[test]
    fn response_round_trips_through_serde() {
        let response = ProbeResponse::new(vec![Finding::graded(
            Severity::Blocking,
            "doc-code.symbol-exists",
            "anchor `crates/engine/src/missing.rs#nope` resolves to no symbol",
            Some(Location::addressed(
                "adr:single-node-cache#status/cites-code",
                1,
                1,
            )),
            None,
        )]);

        let json = serde_json::to_string_pretty(&response).expect("serializes");
        let back: ProbeResponse = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(
            back, response,
            "the response round-trips serialize -> deserialize -> equal",
        );

        insta::assert_snapshot!(json, @r#"
        {
          "findings": [
            {
              "severity": "blocking",
              "probe": "doc-code",
              "check": "symbol-exists",
              "code": "doc-code.symbol-exists",
              "message": "anchor `crates/engine/src/missing.rs#nope` resolves to no symbol",
              "location": {
                "address": "adr:single-node-cache#status/cites-code",
                "line": 1,
                "col": 1
              },
              "route": null
            }
          ],
          "schema_version": 2
        }
        "#);
    }

    /// The response's `findings` deserialize into the engine's one [`Finding`] shape
    /// from a **probe-authored** JSON document (the bytes a subprocess writes to stdout,
    /// not an engine-built struct) — proving an external probe's output binds to the
    /// engine's `Finding` envelope. A probe may omit `probe`/`check` (`#[serde(default)]`),
    /// so this fixture does; they default empty and the engine derives them downstream.
    #[test]
    fn response_findings_deserialize_into_the_one_finding_shape() {
        let wire = r#"{
          "findings": [
            {
              "severity": "blocking",
              "code": "doc-code.symbol-exists",
              "message": "dangling anchor",
              "location": { "address": "adr:cache#status/cites-code", "line": 1, "col": 1 },
              "route": null
            }
          ],
          "schema_version": 2
        }"#;

        let response: ProbeResponse =
            serde_json::from_str(wire).expect("probe-authored JSON deserializes");

        assert_eq!(response.findings.len(), 1);
        let finding = &response.findings[0];
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(finding.code, "doc-code.symbol-exists");
        assert_eq!(
            finding.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("adr:cache#status/cites-code"),
        );
    }

    /// Two enumerated anchors — a header `cites-code` (`symbol-exists`) and a criterion
    /// `maps-to-test` (`criterion-maps-to-test`) — the same two shapes T2 enumerates.
    fn sample_anchors() -> Vec<TargetAnchor> {
        vec![
            TargetAnchor {
                address: "adr:single-node-cache#status/cites-code".to_string(),
                anchor_value: "crates/engine/src/validate.rs#validate_task".to_string(),
                check_id: "symbol-exists".to_string(),
            },
            TargetAnchor {
                address: "spec:rate-limiting#criteria/rate-limit/maps-to-test".to_string(),
                anchor_value: "crates/engine/src/missing.rs#nope".to_string(),
                check_id: "criterion-maps-to-test".to_string(),
            },
        ]
    }

    /// Serialization is unit-proven (the T3 done-criterion): the snapshot serialises and
    /// **deserialises back equal** (serialize → deserialize → equal), and its JSON pins
    /// the field order (`anchors` then `working_tree_root`) the wire contract carries —
    /// a reorder or a serde-attribute slip breaks the golden. This is the shared form an
    /// in-process and a subprocess probe both consume.
    #[test]
    fn snapshot_round_trips_through_serde() {
        let snapshot = EffectiveStateSnapshot::new(sample_anchors(), PathBuf::from("/repo/root"));

        let json = serde_json::to_string_pretty(&snapshot).expect("serialises");
        let back: EffectiveStateSnapshot = serde_json::from_str(&json).expect("deserialises");

        assert_eq!(
            back, snapshot,
            "the snapshot round-trips serialize -> deserialize -> equal",
        );

        insta::assert_snapshot!(json, @r#"
        {
          "anchors": [
            {
              "address": "adr:single-node-cache#status/cites-code",
              "anchor_value": "crates/engine/src/validate.rs#validate_task",
              "check_id": "symbol-exists"
            },
            {
              "address": "spec:rate-limiting#criteria/rate-limit/maps-to-test",
              "anchor_value": "crates/engine/src/missing.rs#nope",
              "check_id": "criterion-maps-to-test"
            }
          ],
          "working_tree_root": "/repo/root"
        }
        "#);
    }

    /// An **in-process** doc-code probe test double (an `impl Probe`, NOT a subprocess):
    /// it declares `Target = Address` (the doc-code target shape) and `Ctx` a borrowed
    /// [`EffectiveStateSnapshot`], and emits one finding per snapshot anchor whose
    /// `anchor_value` names a file the working tree lacks — a stand-in for the real
    /// tree-sitter resolve (inc 4). It reads **only** the snapshot's enumerated pairs;
    /// it never re-parses a doc.
    struct InProcessDocCodeProbe;

    impl Probe for InProcessDocCodeProbe {
        type Target = Address;
        type Ctx<'a> = &'a EffectiveStateSnapshot;

        fn check(&self, _target: &Address, ctx: &EffectiveStateSnapshot) -> Vec<Finding> {
            ctx.anchors
                .iter()
                .filter(|anchor| {
                    // Resolve the anchor's `<path>#<symbol>` file against the snapshot's
                    // working-tree root; a missing file is a dangling anchor.
                    let path = anchor.anchor_value.split('#').next().unwrap_or("");
                    !ctx.working_tree_root.join(path).exists()
                })
                .map(|anchor| {
                    Finding::graded(
                        crate::finding::Severity::Blocking,
                        format!("doc-code.{}", anchor.check_id),
                        format!("anchor `{}` resolves to no file", anchor.anchor_value),
                        Some(crate::finding::Location::addressed(
                            anchor.address.clone(),
                            1,
                            1,
                        )),
                        None,
                    )
                })
                .collect()
        }
    }

    /// The reshaped seam admits a probe consuming the **serialized** effective-state ctx
    /// end-to-end (the T3 done-criterion): the engine materializes the snapshot, it is
    /// round-tripped through serialization (serialize -> deserialize — the exact bytes
    /// the wire carries by path-ref), and the in-process probe is handed a `(target,
    /// ctx)` over the *deserialized* snapshot. It returns findings derived from the pairs
    /// it reads — one for the anchor naming a file the working tree lacks, none for the
    /// anchor naming a present file — proving the probe consumes the serialized snapshot,
    /// not a live handle.
    #[test]
    fn in_process_probe_consumes_serialized_snapshot_end_to_end() {
        // A working tree where the first anchor's file EXISTS and the second's does not.
        let root = std::env::temp_dir().join(format!(
            "jigc-snapshot-probe-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        std::fs::create_dir_all(root.join("crates/engine/src")).expect("mk tree");
        std::fs::write(
            root.join("crates/engine/src/validate.rs"),
            b"fn validate_task() {}\n",
        )
        .expect("present file");

        let snapshot = EffectiveStateSnapshot::new(sample_anchors(), root.clone());

        // Cross the wire: materialized -> serialized -> deserialized (the bytes the
        // path-ref carries). The probe consumes the DESERIALIZED snapshot, never the live one.
        let bytes = serde_json::to_vec(&snapshot).expect("serialises");
        let over_wire: EffectiveStateSnapshot =
            serde_json::from_slice(&bytes).expect("deserialises");

        let target: Address = "adr:single-node-cache#status/cites-code"
            .parse()
            .expect("address parses");
        let findings = InProcessDocCodeProbe.check(&target, &over_wire);

        assert_eq!(
            findings.len(),
            1,
            "exactly the dangling anchor surfaces (the present one is clean): {findings:?}",
        );
        assert_eq!(findings[0].code, "doc-code.criterion-maps-to-test");
        assert_eq!(
            findings[0]
                .location
                .as_ref()
                .and_then(|l| l.address.as_deref()),
            Some("spec:rate-limiting#criteria/rate-limit/maps-to-test"),
            "the finding is derived from the snapshot pair the probe read",
        );

        let _ = std::fs::remove_dir_all(&root);
    }

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
