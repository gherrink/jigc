//! The pack's closed, typed **scalar-knob surface** — loaded from `config/knobs.yaml`.
//!
//! A knob is a pack-declared, typed field (`design/overrides.md` → Scalar knobs
//! are config-level fields). The on-disk declaration is one entry per settable
//! key, reusing the document-type [`FieldType`](crate::schema) vocabulary so a
//! `scalar-set` is adjudicated by the **same** [`crate::write::check_value`] the
//! doc write path uses — no second type system.
//!
//! The loader builds the pack-default layer's scalar surface from this file: the
//! **closed key set** (what `scalar-set` may target) plus each knob's
//! **materialized default**. The default is materialized by *this loader* seeding
//! the base scalar map — independent of the doc-instance `Field.default` (an
//! orthogonal, still-unfixed write-path defect, `DECISIONS.md` 2026-06-03). A
//! knob whose declaration carries no `default` cannot seed the closed surface
//! deterministically, so it is rejected at load.

use crate::schema::Field;
use std::collections::BTreeMap;
use thiserror::Error;

/// The on-disk shape of one knob entry: the [`FieldType`](crate::schema)-reusing
/// `{type, of?, default}` triple, keyed in the file by the knob's settable key.
/// Deserialized then folded into a [`crate::schema::Field`] (id = the map key) so
/// adjudication reuses `check_value`.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct KnobDecl {
    #[serde(rename = "type")]
    ty: crate::schema::FieldType,
    #[serde(default)]
    of: Option<Vec<String>>,
    #[serde(default)]
    default: Option<String>,
    /// The demotion-lock floor: the value may be set to this severity or
    /// stricter, never below (`overrides.md` → Locked keys). Intrinsic checks
    /// carry `floor: blocking`; tunable checks carry none.
    #[serde(default)]
    floor: Option<String>,
}

/// The engine-owned set of intrinsic check cascade keys — the checks whose
/// demotion would break a load-bearing invariant of the system itself
/// (`validation.md` → What "intrinsic" means mechanically: the eleven
/// `workflow-refs.*` checks, the five `schema-conformance.*` checks — the four
/// structural ones plus `schema-version-current`, intrinsic on the trustworthiness
/// axis (M42): a corpus below its manifest version means every other check in the
/// sweep is adjudicating docs against a schema they were never written to — the three
/// `pack-probe-integrity.*` meta-findings, and the `owner-artifact.present` #5 gate).
/// The
/// engine asserts at pack-load that every one of these knobs is **declared** and
/// floored at `blocking`, so a mis-declared pack cannot silently un-lock the
/// determinism boundary by leaving an intrinsic check absent or demotable. This
/// is *assertion-only* — it never assigns severity nor ships pack content, so the
/// engine-empty invariant holds.
pub const INTRINSIC_CHECK_KEYS: &[&str] = &[
    "validation.workflow-refs.placeholder-resolves.severity",
    "validation.workflow-refs.include-resolves.severity",
    "validation.workflow-refs.command-ref-resolves.severity",
    "validation.workflow-refs.include-cycle-absent.severity",
    "validation.workflow-refs.at-marker-on-non-scalar.severity",
    "validation.workflow-refs.run-marker-not-shadowed.severity",
    "validation.workflow-refs.spawn-marker-not-shadowed.severity",
    "validation.workflow-refs.checkpoint-marker-not-shadowed.severity",
    "validation.workflow-refs.fan-out-join-paired.severity",
    "validation.workflow-refs.body-include-only.severity",
    "validation.workflow-refs.schema-ref-resolves.severity",
    "validation.schema-conformance.ref-resolves.severity",
    "validation.schema-conformance.required-slot-present.severity",
    "validation.schema-conformance.required-field-present.severity",
    "validation.schema-conformance.field-value-conformant.severity",
    "validation.schema-conformance.schema-version-current.severity",
    "validation.pack-probe-integrity.timeout.severity",
    "validation.pack-probe-integrity.crash.severity",
    "validation.pack-probe-integrity.malformed-output.severity",
    "validation.owner-artifact.present.severity",
];

/// The parsed knob surface: the declared knobs as `(key, Field)` in sorted key
/// order, each carrying its materialized default. This is what seeds the
/// pack-default layer (closed key set + base scalar values).
#[derive(Debug)]
pub struct KnobSet {
    /// One [`Field`] per declared knob, id = the knob key, in sorted key order.
    fields: Vec<Field>,
    /// Each knob's materialized default value, keyed by knob key — the base
    /// scalar map the resolver starts from.
    defaults: BTreeMap<String, String>,
    /// The demotion-lock floor for each *floored* knob, keyed by knob key. Only
    /// floored (intrinsic) knobs appear; a tunable knob is absent. The cascade
    /// resolver reads this to soft-reject a below-floor `scalar-set`.
    floors: BTreeMap<String, String>,
}

impl KnobSet {
    /// The materialized base scalar map — each declared key → its default value.
    /// This is fed verbatim to [`crate::cascade::PackDefaultLayer::new`] as the
    /// closed, pre-seeded scalar surface.
    pub fn base_scalars(&self) -> BTreeMap<String, String> {
        self.defaults.clone()
    }

    /// The declared [`Field`] for `key`, or `None` if it is not a knob. The
    /// write path adjudicates a `scalar-set` against this with `check_value`.
    pub fn field(&self, key: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.id == key)
    }

    /// The declared knob keys (the closed key set), in sorted key order. The
    /// upgrade-time `override-default` classifier reads these to ask whether a
    /// recorded `scalar-set`'s key is still a declared knob (`overrides.md` →
    /// Upgrade reconciliation: `scalar-set | a key's existence | clean / orphaned`).
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.fields.iter().map(|f| f.id.as_str())
    }

    /// The demotion-lock floors, keyed by knob key — only floored knobs appear
    /// (a tunable knob declaring no `floor` is absent). The cascade resolver
    /// reads this to soft-reject a `scalar-set` that would set a key below its
    /// floor (`overrides.md` → Soft-rejection).
    pub fn floors(&self) -> &BTreeMap<String, String> {
        &self.floors
    }
}

/// Why loading the knob surface failed.
#[derive(Debug, Error)]
pub enum KnobError {
    /// The bytes were not valid UTF-8 (declarations are text).
    #[error("knobs.yaml is not valid UTF-8")]
    NotUtf8,

    /// The YAML did not match the knob-declaration model (unknown key, bad type).
    #[error("malformed knobs.yaml: {0}")]
    Malformed(#[from] serde_yaml_ng::Error),

    /// A knob carried no `default` — it cannot seed the closed scalar surface.
    #[error("knob `{0}` declares no `default` (a knob must materialize a base value)")]
    MissingDefault(String),

    /// An engine-known intrinsic check's knob is not floored at `blocking` — a
    /// mis-declared pack that would silently un-lock the determinism boundary.
    /// The engine asserts its known intrinsic id set is each floored at load
    /// (`overrides.md` → Locked keys; assertion-only).
    #[error("intrinsic check `{0}` must declare `floor: blocking` but does not")]
    UnflooredIntrinsic(String),

    /// An engine-known intrinsic check is not declared in the knob surface at
    /// all. Today a `scalar-set` on an undeclared key hard-aborts, so an absent
    /// intrinsic key is not *directly* demotable — but requiring every intrinsic
    /// check be declared (and floored) keeps the determinism boundary locked by
    /// the floor itself, not resting solely on the closed-surface rule (which a
    /// future change could loosen). Defense-in-depth; assertion-only
    /// (`overrides.md` → Locked keys).
    #[error("intrinsic check `{0}` must be declared (with `floor: blocking`) but is absent")]
    UndeclaredIntrinsic(String),

    /// A knob declared a non-engine-native `type` (a pack-declared field type).
    /// Knobs reuse only the engine-native field-type vocabulary; a pack type is
    /// never a knob, so it is rejected at load rather than folded in as an
    /// opaque value. (Before M10 opened the type vocabulary this was a serde
    /// error against the closed enum; now it is this typed error.)
    #[error("knob `{0}` declares a non-native type `{1}` (knobs reuse only engine-native types)")]
    NonNativeType(String, String),
}

/// Parse the closed knob surface from raw `config/knobs.yaml` bytes.
///
/// Each top-level key is a settable knob; its `{type, of?, default, floor?}` body
/// folds into a [`Field`] (id = the key) reusing the document-type field model.
/// Every knob must declare a `default` (the value the resolver seeds the base map
/// with); an absent default is a [`KnobError::MissingDefault`]. A knob may also
/// declare a `floor` (the demotion-lock; see [`KnobSet::floors`]). The engine
/// asserts at load that every [`INTRINSIC_CHECK_KEYS`] knob is **declared** and
/// floored at `blocking` — an absent intrinsic is a
/// [`KnobError::UndeclaredIntrinsic`], a declared-but-unfloored one a
/// [`KnobError::UnflooredIntrinsic`].
pub fn load_knobs(bytes: &[u8]) -> Result<KnobSet, KnobError> {
    let text = std::str::from_utf8(bytes).map_err(|_| KnobError::NotUtf8)?;
    let decls: BTreeMap<String, KnobDecl> = serde_yaml_ng::from_str(text)?;

    let mut fields = Vec::with_capacity(decls.len());
    let mut defaults = BTreeMap::new();
    let mut floors = BTreeMap::new();
    for (key, decl) in decls {
        if let crate::schema::FieldType::Pack(pack) = &decl.ty {
            return Err(KnobError::NonNativeType(key, pack.name.clone()));
        }
        let default = decl
            .default
            .ok_or_else(|| KnobError::MissingDefault(key.clone()))?;
        defaults.insert(key.clone(), default);
        if let Some(floor) = decl.floor {
            floors.insert(key.clone(), floor);
        }
        fields.push(Field {
            id: key,
            ty: decl.ty,
            of: decl.of,
            default: None,
            set: None,
            to: None,
            card: None,
            inverse: None,
            inverse_card: None,
            check: None,
            optional: false,
            title_names_symbol: false,
        });
    }

    // Assertion-only: every engine-known intrinsic check must be *declared* and
    // floored at `blocking`. An undeclared intrinsic key is not *directly*
    // demotable today (a `scalar-set` on an undeclared key hard-aborts
    // `UndeclaredScalar`), but requiring declaration too keeps the determinism
    // boundary locked by the floor itself — defense-in-depth, not resting solely
    // on the closed-surface rule (which a future change could loosen). A
    // declared-but-unfloored intrinsic is the un-lock a mis-declared pack would
    // otherwise sneak in. This never assigns a severity nor ships pack content,
    // so the engine-empty invariant holds.
    for key in INTRINSIC_CHECK_KEYS {
        if !defaults.contains_key(*key) {
            return Err(KnobError::UndeclaredIntrinsic((*key).to_owned()));
        }
        if floors.get(*key).map(String::as_str) != Some("blocking") {
            return Err(KnobError::UnflooredIntrinsic((*key).to_owned()));
        }
    }

    Ok(KnobSet {
        fields,
        defaults,
        floors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cascade::{self, PackDefaultLayer};
    use crate::schema::FieldType;
    use crate::write::check_value;

    /// The shipped pack knob surface, loaded from the embedded source tree so the
    /// test pins exactly the bytes that ship.
    const KNOBS_YAML: &[u8] = include_bytes!("../../cli/pack/config/knobs.yaml");

    /// Loading the embedded `knobs.yaml`, building the pack-default layer from its
    /// base scalars, and resolving yields each key's declared default — and the
    /// resolved key set is **exactly** the declared knob keys (the closed surface):
    /// the full per-check severity surface (the inventory rows live this
    /// increment, per `validation.md` → MVP check inventory — including the three
    /// `pack-probe-integrity.*` meta-findings) plus the two retained M4 per-probe
    /// keys (additive defaults, never a rename) and `default-workflow`. No
    /// team/project layer is present, so resolution returns the materialized base
    /// values verbatim.
    #[test]
    fn loaded_knobs_seed_the_pack_default_scalar_surface() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");

        let pack = PackDefaultLayer::new("dev", "0.1.0", knobs.base_scalars(), Vec::new());
        let resolved = cascade::resolve(&pack, None, None).expect("resolves");

        // Each declared key resolves to its inventory default through the cascade.
        // `(key, default)` in sorted-key order — the closed surface is exactly
        // this list, no more, no less.
        let expected: Vec<(&str, &str)> = vec![
            ("default-workflow", "router"),
            // docs-root — the managed-doc parent dir (default `docs/`); string,
            // tunable, no floor.
            ("docs-root", "docs/"),
            // finalize.fan-out.squash — the milestone commit-shaping knob (M8),
            // bool, default true (the M7 single-aggregate form).
            ("finalize.fan-out.squash", "true"),
            // invocation-log — the opt-in in-repo invocation log (M36), bool,
            // default false (OFF); tunable, no floor.
            ("invocation-log", "false"),
            // changelog-recording (1, tunable; M42 — the granted-and-unused changelog
            // gate: advisory by default, promotable to `blocking` with one cascade
            // line, validation.md → The changelog-gate advisory).
            (
                "validation.changelog-recording.gate-granted-unused.severity",
                "advisory",
            ),
            // commit-rendering (2, advisory-by-default convention checks).
            (
                "validation.commit-rendering.line-limit-body.severity",
                "advisory",
            ),
            (
                "validation.commit-rendering.line-limit-subject.severity",
                "advisory",
            ),
            // doc-code (3, tunable; M10 ×2 — the pack-provided doc↔code probe,
            // blocking-by-default — + the M40 stale-heading guard, advisory-by-
            // default: the brand-name false-positive class has no valid remedy
            // under blocking. NOT floored).
            (
                "validation.doc-code.criterion-maps-to-test.severity",
                "blocking",
            ),
            ("validation.doc-code.symbol-exists.severity", "blocking"),
            (
                "validation.doc-code.title-names-symbol.severity",
                "advisory",
            ),
            // file-state: the M4 per-probe default (retained, additive) + the
            // hash-matches per-check key (tunable).
            ("validation.file-state.hash-matches.severity", "blocking"),
            ("validation.file-state.severity", "blocking"),
            // override-default (3, blocking-by-default, tunable from M6).
            (
                "validation.override-default.basis-recorded.severity",
                "blocking",
            ),
            (
                "validation.override-default.target-exists.severity",
                "blocking",
            ),
            (
                "validation.override-default.target-unchanged.severity",
                "blocking",
            ),
            // owner-artifact (1, intrinsic — the M16 #5 completion-half gate).
            ("validation.owner-artifact.present.severity", "blocking"),
            // pack-probe-integrity (3, intrinsic — the enforced meta-findings).
            ("validation.pack-probe-integrity.crash.severity", "blocking"),
            (
                "validation.pack-probe-integrity.malformed-output.severity",
                "blocking",
            ),
            (
                "validation.pack-probe-integrity.timeout.severity",
                "blocking",
            ),
            // schema-completeness (1, advisory at task scope by design).
            (
                "validation.schema-completeness.inverse-cardinality.severity",
                "advisory",
            ),
            // schema-conformance (4 intrinsic + 3 tunable store-scope — M33
            // mention-resolves, M40 repeatable-populated + its `.exempt` string
            // sibling (the pinned `doctype#section` steady-state tokens) + M40
            // surplus-sections-absent, the trailing-surplus raw-block scan).
            (
                "validation.schema-conformance.field-value-conformant.severity",
                "blocking",
            ),
            (
                "validation.schema-conformance.mention-resolves.severity",
                "advisory",
            ),
            (
                "validation.schema-conformance.ref-resolves.severity",
                "blocking",
            ),
            (
                "validation.schema-conformance.repeatable-populated.exempt",
                "changelog#unreleased-changes milestone-record#tasks completion-record#findings",
            ),
            (
                "validation.schema-conformance.repeatable-populated.severity",
                "advisory",
            ),
            (
                "validation.schema-conformance.required-field-present.severity",
                "blocking",
            ),
            (
                "validation.schema-conformance.required-slot-present.severity",
                "blocking",
            ),
            (
                "validation.schema-conformance.schema-version-current.severity",
                "blocking",
            ),
            (
                "validation.schema-conformance.surplus-sections-absent.severity",
                "advisory",
            ),
            // workflow-refs (11, intrinsic) + the M4 per-probe default (retained).
            (
                "validation.workflow-refs.at-marker-on-non-scalar.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.body-include-only.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.checkpoint-marker-not-shadowed.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.command-ref-resolves.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.fan-out-join-paired.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.include-cycle-absent.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.include-resolves.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.placeholder-resolves.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.run-marker-not-shadowed.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.schema-ref-resolves.severity",
                "blocking",
            ),
            ("validation.workflow-refs.severity", "blocking"),
            (
                "validation.workflow-refs.spawn-marker-not-shadowed.severity",
                "blocking",
            ),
        ];

        // The closed surface is exactly the declared keys — no more, no less.
        let base = knobs.base_scalars();
        let keys: Vec<&str> = base.keys().map(String::as_str).collect();
        let expected_keys: Vec<&str> = expected.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, expected_keys);

        // Each key resolves to its inventory default through the cascade read.
        for (key, default) in &expected {
            assert_eq!(resolved.scalar(key), Some(*default), "key `{key}`");
        }

        // `pack-id` is pack identity, never a knob — it is not in the surface.
        assert!(knobs.field("pack-id").is_none());
    }

    /// Each declared knob folds into a [`Field`] adjudicated by the same
    /// `check_value` the doc write path uses: a declared enum member passes, a
    /// non-member is rejected — proving no second type system.
    #[test]
    fn knob_fields_adjudicate_via_check_value() {
        use crate::field_block::Value;

        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");
        let field = knobs.field("default-workflow").expect("declared knob");
        assert_eq!(field.ty, FieldType::Enum);

        check_value(field, &Value::Scalar("single-task".to_owned()))
            .expect("a declared enum member passes");
        check_value(field, &Value::Scalar("not-a-workflow".to_owned()))
            .expect_err("a non-member is rejected");
    }

    /// The `finalize.fan-out.squash` knob (M8) is a `bool` that resolves `true` by
    /// default through the cascade and is **settable to `false`** by a project
    /// `scalar-set` — the read-side knob the milestone commit boundary reads to
    /// choose one aggregate commit (`true`) vs per-sub-task commits (`false`). A
    /// tunable knob (no floor), adjudicated by the same `check_value` (a bool
    /// accepts `true`/`false`, rejects anything else).
    #[test]
    fn squash_knob_defaults_true_and_is_settable_to_false() {
        use crate::cascade::OverrideLayer;
        use crate::field_block::Value;

        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");

        // It is a declared `bool` knob with no floor (tunable).
        let field = knobs
            .field("finalize.fan-out.squash")
            .expect("squash is a declared knob");
        assert_eq!(field.ty, FieldType::Bool);
        assert!(
            knobs.floors().get("finalize.fan-out.squash").is_none(),
            "squash is tunable — it carries no demotion-lock floor",
        );

        // Default: resolves `true` with no override layer (the M7 aggregate form).
        let pack = PackDefaultLayer::new("dev", "0.1.0", knobs.base_scalars(), Vec::new());
        let default = cascade::resolve(&pack, None, None).expect("resolves");
        assert_eq!(
            default.scalar("finalize.fan-out.squash"),
            Some("true"),
            "squash resolves true by default",
        );

        // Settable: a project `scalar-set finalize.fan-out.squash false` resolves false.
        let project = OverrideLayer::empty().scalar_set("finalize.fan-out.squash", "false");
        let overridden =
            cascade::resolve(&pack, None, Some(&project)).expect("a settable knob resolves");
        assert_eq!(
            overridden.scalar("finalize.fan-out.squash"),
            Some("false"),
            "squash is settable to false via a project scalar-set",
        );

        // The bool type adjudicates via the same `check_value`: true/false pass, else reject.
        check_value(field, &Value::Scalar("false".to_owned())).expect("`false` is a valid bool");
        check_value(field, &Value::Scalar("maybe".to_owned())).expect_err("a non-bool is rejected");
    }

    /// A knob declaration with no `default` cannot seed the base surface, so it is
    /// a typed load error (not a silent empty seed that would break the read-side
    /// determinism invariant).
    #[test]
    fn knob_without_default_is_a_typed_error() {
        let yaml = b"some-knob:\n  type: string\n";
        let err = load_knobs(yaml).expect_err("a defaultless knob errors");
        assert!(
            matches!(err, KnobError::MissingDefault(ref k) if k == "some-knob"),
            "expected MissingDefault, got {err:?}",
        );
    }

    /// An unknown key in a knob body is rejected (the `deny_unknown_fields` guard),
    /// so a typo'd declaration surfaces as a typed error, never a silent drop.
    #[test]
    fn unknown_knob_body_key_is_a_typed_error() {
        let yaml = b"some-knob:\n  type: string\n  defualt: x\n";
        let err = load_knobs(yaml).expect_err("unknown body key errors");
        assert!(matches!(err, KnobError::Malformed(_)), "got {err:?}");
    }

    /// A knob declaring a non-engine-native type is rejected with a typed error.
    /// Knobs reuse only the native field-type vocabulary; once M10 opened the
    /// vocabulary, a non-native `type:` deserializes to a pack-declared type
    /// rather than a serde error, so the knob loader rejects it explicitly — a
    /// pack field type is never a knob.
    #[test]
    fn non_native_knob_type_is_a_typed_error() {
        let yaml = b"some-knob:\n  type: code-anchor\n  default: x\n";
        let err = load_knobs(yaml).expect_err("non-native knob type errors");
        assert!(
            matches!(&err, KnobError::NonNativeType(key, ty)
                if key == "some-knob" && ty == "code-anchor"),
            "expected NonNativeType, got {err:?}",
        );
    }

    /// The embedded pack floors **exactly** the 16 intrinsic checks at `blocking`
    /// — the `floors` accessor exposes them, and the set is precisely the
    /// engine-owned intrinsic id set (`validation.md` → What 'intrinsic' means
    /// mechanically). The assertion-only load-time guard passes for the shipped
    /// pack because every intrinsic id is floored.
    #[test]
    fn embedded_knobs_floor_the_intrinsic_checks() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");
        let floors = knobs.floors();

        let floored: Vec<&str> = floors.keys().map(String::as_str).collect();
        let mut expected: Vec<&str> = INTRINSIC_CHECK_KEYS.to_vec();
        expected.sort_unstable();
        assert_eq!(
            floored, expected,
            "floored set is exactly the intrinsic ids"
        );

        for key in INTRINSIC_CHECK_KEYS {
            assert_eq!(floors.get(*key).map(String::as_str), Some("blocking"));
        }
    }

    /// A project `scalar-set validation.pack-probe-integrity.timeout.severity
    /// advisory` is **soft-rejected at cascade resolution** over the embedded knob
    /// surface: the meta-finding's `floor: blocking` drops the below-floor demotion,
    /// the key resolves from the remaining layers (the pack-default base `blocking`),
    /// and the dropped delta surfaces as a `RejectedDemotion` line with `(attempted,
    /// floor, layer)`. A misbehaving probe's meta-finding cannot be demoted below
    /// `blocking` (`validation.md` → What 'intrinsic' means mechanically;
    /// `overrides.md` → Soft-rejection). Driven over the *embedded* bytes through
    /// the production floor-wiring path (`with_floors`), so the proof is against the
    /// shipped surface, not a synthetic one.
    #[test]
    fn pack_probe_integrity_timeout_demotion_is_soft_rejected() {
        use crate::cascade::{LayerKind, OverrideLayer};

        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");
        let key = "validation.pack-probe-integrity.timeout.severity";

        // The meta-finding is a declared, blocking-floored knob (intrinsic).
        assert_eq!(
            knobs.floors().get(key).map(String::as_str),
            Some("blocking"),
            "the timeout meta-finding is floored at blocking",
        );

        // Production floor-wiring: seed the base from the embedded knobs and attach
        // the embedded floors — the path `cli::start` takes.
        let pack = PackDefaultLayer::new("dev", "0.1.0", knobs.base_scalars(), Vec::new())
            .with_floors(knobs.floors().clone());

        // A project tries to demote the timeout meta-finding to advisory.
        let project = OverrideLayer::empty().scalar_set(key, "advisory");
        let resolved = cascade::resolve(&pack, None, Some(&project)).expect("resolution continues");

        // The meta-finding stays blocking — the below-floor demotion was dropped.
        assert_eq!(
            resolved.scalar(key),
            Some("blocking"),
            "the meta-finding stays blocking; the demotion was not applied",
        );
        // And it was *not* recorded as an applied override.
        assert!(
            resolved.overridden_scalar(key).is_none(),
            "a soft-rejected demotion is not an applied override",
        );

        // The rejection surfaces as a RejectedDemotion line with attempted/floor/layer.
        let rejected: Vec<_> = resolved.rejected_scalar_sets().collect();
        assert_eq!(
            rejected,
            vec![(key, "advisory", "blocking", LayerKind::Project)],
            "the dropped demotion surfaces as a RejectedDemotion",
        );
    }

    /// The shipped per-check severity surface reconciles to the
    /// [`design/validation.md`] Severity inventory (the single source of truth):
    /// **34 per-check `validation.<probe>.<check>.severity` keys — 20 intrinsic
    /// (floored at `blocking`) + 14 tunable (no floor)** (`validation.md` → Severity
    /// inventory). The M15 `checkpoint-marker-not-shadowed` row joined the intrinsic
    /// set (16 → 17); the M16 `owner-artifact.present` #5 gate joins it next (17 → 18);
    /// the M42 `schema-conformance.schema-version-current` row follows (18 → 19 —
    /// the version-currency break, minted with its own check id so a machine consumer
    /// can act on it; intrinsic on the trustworthiness axis, see
    /// [`schema_version_current_key_is_declared_intrinsic_blocking`]); the M43
    /// `workflow-refs.schema-ref-resolves` row completes it (19 → 20 — the
    /// `{{schema:<doctype>}}` generation-seam membership check, resolved against the
    /// composed cascade's doctype set, see
    /// [`schema_ref_resolves_key_is_declared_intrinsic_blocking`]); the M33
    /// `schema-conformance.mention-resolves` row joins the **tunable** set
    /// (9 → 10 — advisory, store-scope only, unfloored); the M40
    /// `doc-code.title-names-symbol` row joins it too (10 → 11 — the stale-heading
    /// guard, live since the long-horizon-study fix but shipped un-keyed, keyed +
    /// demoted to advisory here); the M40 `schema-conformance.repeatable-populated`
    /// row follows (11 → 12 — the hollow-adoption advisory; its sibling `….exempt`
    /// **string** knob is not a severity key and stays outside this count); its M40
    /// sibling `schema-conformance.surplus-sections-absent` completes the wave
    /// (12 → 13 — the trailing-surplus advisory); the M42
    /// `changelog-recording.gate-granted-unused` row closes it (13 → 14 — the
    /// granted-and-unused changelog gate: advisory by default, keyed precisely so a
    /// project that means it promotes the skip to `blocking` with one cascade line,
    /// `validation.md` → The changelog-gate advisory). The 20
    /// intrinsic are exactly [`INTRINSIC_CHECK_KEYS`]; the tunable remainder is every
    /// other per-check key, including the three `doc-code.*` rows. Counted over the
    /// *embedded* bytes, so the count is the shipped surface — not a synthetic one.
    ///
    /// [`design/validation.md`]: ../../../design/validation.md
    #[test]
    fn per_check_severity_surface_reconciles_to_the_34_20_14_inventory() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");

        // The per-check keys are the inventory rows: keyed by check, never by
        // probe — a per-probe `validation.<probe>.severity` default is not a row.
        let per_check: Vec<&str> = knobs
            .keys()
            .filter(|k| {
                k.starts_with("validation.")
                    && k.ends_with(".severity")
                    && k.matches('.').count() == 3
            })
            .collect();
        assert_eq!(
            per_check.len(),
            34,
            "the inventory totals 34 checks (validation.md → Severity inventory); got:\n{per_check:#?}",
        );

        // 20 are floored at `blocking` (intrinsic) — exactly INTRINSIC_CHECK_KEYS.
        let intrinsic = per_check
            .iter()
            .filter(|k| knobs.floors().get(**k).map(String::as_str) == Some("blocking"))
            .count();
        assert_eq!(intrinsic, 20, "20 intrinsic checks (floored at blocking)");
        assert_eq!(INTRINSIC_CHECK_KEYS.len(), 20);

        // The remaining 14 are tunable (no floor) — 34 - 20.
        let tunable = per_check
            .iter()
            .filter(|k| knobs.floors().get(**k).is_none())
            .count();
        assert_eq!(tunable, 14, "14 tunable checks (unfloored)");
    }

    /// (M42 Increment 4 / T1) The version-currency break joins the **keyed** severity
    /// surface under its own minted check id: `validation.schema-conformance.
    /// schema-version-current.severity` is **declared** and **floored at `blocking`**
    /// — an [`INTRINSIC_CHECK_KEYS`] member (`validation.md` → MVP check inventory,
    /// the `schema-version-current` row; → Version-currency is itself a surfaced
    /// break, the retraction: *intrinsic + keyed*).
    ///
    /// Two facts this pins. **Keyed:** without the declaration a
    /// `scalar-set` on the key hard-aborts (`UndeclaredScalar`) and the finding is
    /// exempt from the severity post-pass — the check would be un-tunable *and*
    /// un-re-gradable, alone among the conformance family. **Floored:** a project
    /// must not be able to demote it, because an unmigrated corpus means every other
    /// check in the sweep is adjudicating docs against a schema they were never
    /// written to — a demotion would let a project silently opt out of knowing its
    /// own validation results are meaningless (and would silently disarm the
    /// store-scope exit flip that keys on this code).
    #[test]
    fn schema_version_current_key_is_declared_intrinsic_blocking() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");

        let key = "validation.schema-conformance.schema-version-current.severity";
        assert!(knobs.field(key).is_some(), "{key} is declared");
        assert_eq!(
            knobs.base_scalars().get(key).map(String::as_str),
            Some("blocking"),
            "the version-currency break is blocking by default",
        );
        assert_eq!(
            knobs.floors().get(key).map(String::as_str),
            Some("blocking"),
            "intrinsic — floored at blocking, never demotable",
        );
        assert!(
            INTRINSIC_CHECK_KEYS.contains(&key),
            "the version-currency key is an INTRINSIC_CHECK_KEYS member",
        );
    }

    /// (M43 T2) The `{{schema:<doctype>}}` generation-seam membership check joins the
    /// **keyed** severity surface as a floored-blocking intrinsic:
    /// `validation.workflow-refs.schema-ref-resolves.severity` is **declared**,
    /// **defaulted `blocking`**, and **floored at `blocking`** — an
    /// [`INTRINSIC_CHECK_KEYS`] member (`validation.md` → MVP check inventory;
    /// `surface-contract.md` → The schema projection: intrinsic-floor class, severity
    /// blocking, un-tunable). A demotion would let a soliciting template silently
    /// render a dangling projection ref as prose noise — the exact "template
    /// understates the schema" lie the seam exists to make impossible.
    #[test]
    fn schema_ref_resolves_key_is_declared_intrinsic_blocking() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");

        let key = "validation.workflow-refs.schema-ref-resolves.severity";
        assert!(knobs.field(key).is_some(), "{key} is declared");
        assert_eq!(
            knobs.base_scalars().get(key).map(String::as_str),
            Some("blocking"),
            "the schema-ref break is blocking by default",
        );
        assert_eq!(
            knobs.floors().get(key).map(String::as_str),
            Some("blocking"),
            "intrinsic — floored at blocking, never demotable",
        );
        assert!(
            INTRINSIC_CHECK_KEYS.contains(&key),
            "the schema-ref key is an INTRINSIC_CHECK_KEYS member",
        );
    }

    /// (M40 F4 half 2) The `surplus-sections-absent` severity key is declared as
    /// designed (`validation.md` → Hollow and surplus adoption): a **tunable
    /// advisory** (no floor) — visibility over trailing surplus H2s the positional
    /// parse never visits, never a gate.
    #[test]
    fn surplus_sections_absent_key_is_declared_tunable_advisory() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");

        let key = "validation.schema-conformance.surplus-sections-absent.severity";
        assert!(knobs.field(key).is_some(), "{key} is declared");
        assert!(
            knobs.floors().get(key).is_none(),
            "the surplus-adoption advisory is tunable — no floor",
        );
        assert_eq!(
            knobs.base_scalars().get(key).map(String::as_str),
            Some("advisory"),
            "advisory-by-default — visibility, never a gate",
        );
    }

    /// (M40 F4 half 1) The `repeatable-populated` pair is declared as designed
    /// (`validation.md` → Hollow and surplus adoption): the severity key is a
    /// **tunable advisory** (no floor), and the sibling `….exempt` knob is a plain
    /// **string** carrying the pinned pack-default `doctype#section` token list —
    /// declared, unfloored, and (by not ending in `.severity`) outside the per-check
    /// severity count above.
    #[test]
    fn repeatable_populated_pair_is_declared_advisory_with_the_pinned_exempt_default() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");

        let severity = "validation.schema-conformance.repeatable-populated.severity";
        assert!(knobs.field(severity).is_some(), "{severity} is declared");
        assert!(
            knobs.floors().get(severity).is_none(),
            "the hollow-adoption advisory is tunable — no floor",
        );
        assert_eq!(
            knobs.base_scalars().get(severity).map(String::as_str),
            Some("advisory"),
            "advisory-by-default — visibility, never a gate",
        );

        let exempt = "validation.schema-conformance.repeatable-populated.exempt";
        assert!(knobs.field(exempt).is_some(), "{exempt} is declared");
        assert!(
            knobs.floors().get(exempt).is_none(),
            "no floor on a string knob"
        );
        assert_eq!(
            knobs.base_scalars().get(exempt).map(String::as_str),
            Some("changelog#unreleased-changes milestone-record#tasks completion-record#findings"),
            "the pack-default exempt token list is the pinned three steady-state sections",
        );
    }

    /// Done-criterion (c): the new `checkpoint-marker-not-shadowed` severity key is
    /// registered in **exactly** the lists its M8 sibling `spawn-marker-not-shadowed`
    /// belongs to — proved, not trusted. The sibling lives in `INTRINSIC_CHECK_KEYS`
    /// and the `knobs.yaml` enum (both floored intrinsic), and NOT in
    /// `CHECK_INVENTORY` (the VERIFIED-FALSE roadmap claim — that gates
    /// task-validate/finalize, not compose-time). This asserts list membership is
    /// byte-for-byte the sibling's set over `INTRINSIC_CHECK_KEYS` (the engine-owned
    /// list) and the embedded knob keys.
    #[test]
    fn checkpoint_severity_key_membership_matches_spawn_sibling() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");
        let checkpoint = "validation.workflow-refs.checkpoint-marker-not-shadowed.severity";
        let spawn = "validation.workflow-refs.spawn-marker-not-shadowed.severity";

        // Both are in INTRINSIC_CHECK_KEYS.
        assert!(
            INTRINSIC_CHECK_KEYS.contains(&spawn),
            "spawn sibling is intrinsic"
        );
        assert!(
            INTRINSIC_CHECK_KEYS.contains(&checkpoint),
            "checkpoint key joins the intrinsic list (the spawn sibling's set)"
        );

        // Both are declared, floored-blocking knobs in the embedded knobs.yaml enum.
        for key in [spawn, checkpoint] {
            assert!(knobs.field(key).is_some(), "{key} is a declared knob");
            assert_eq!(
                knobs.floors().get(key).map(String::as_str),
                Some("blocking"),
                "{key} is floored intrinsic in the knobs enum",
            );
        }
    }

    /// Done-criterion (c): the M16 #5 `owner-artifact.present` gate key is registered
    /// as a **floored-blocking intrinsic** — in [`INTRINSIC_CHECK_KEYS`] (the
    /// engine-owned list) AND declared in the embedded `knobs.yaml` enum with
    /// `floor: blocking`. (Its `CHECK_INVENTORY` membership — the divergence from the
    /// checkpoint sibling — is proved in `result.rs`.) Proved, not trusted.
    #[test]
    fn owner_artifact_present_severity_key_is_floored_intrinsic() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");
        let key = "validation.owner-artifact.present.severity";

        assert!(
            INTRINSIC_CHECK_KEYS.contains(&key),
            "the #5 owner-artifact presence gate joins the intrinsic list",
        );
        assert!(knobs.field(key).is_some(), "{key} is a declared knob");
        assert_eq!(
            knobs.floors().get(key).map(String::as_str),
            Some("blocking"),
            "{key} is floored intrinsic in the knobs enum",
        );
    }

    /// A project `scalar-set validation.doc-code.criterion-maps-to-test.severity
    /// warning` **resolves applied** over the embedded surface — the demotion is
    /// honored, *not* floor-rejected — because the `doc-code.*` checks are tunable
    /// (no floor), unlike the floor-locked `pack-probe-integrity.*` meta-findings
    /// (the tunable counterpart to `pack_probe_integrity_timeout_demotion_is_soft_rejected`).
    /// Driven through the production floor-wiring (`with_floors`) over the shipped
    /// bytes.
    #[test]
    fn doc_code_criterion_demotion_resolves_applied() {
        use crate::cascade::OverrideLayer;

        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");
        let key = "validation.doc-code.criterion-maps-to-test.severity";

        // It is a declared, *tunable* knob — no floor.
        assert!(
            knobs.field(key).is_some(),
            "doc-code check is a declared knob"
        );
        assert!(
            knobs.floors().get(key).is_none(),
            "the doc-code check is tunable — it carries no demotion-lock floor",
        );

        // Production floor-wiring over the embedded bytes.
        let pack = PackDefaultLayer::new("dev", "0.1.0", knobs.base_scalars(), Vec::new())
            .with_floors(knobs.floors().clone());

        // A project demotes the check to warning — honored, not floor-rejected.
        let project = OverrideLayer::empty().scalar_set(key, "warning");
        let resolved = cascade::resolve(&pack, None, Some(&project)).expect("resolves");

        assert_eq!(
            resolved.scalar(key),
            Some("warning"),
            "the tunable doc-code demotion is applied",
        );
        assert_eq!(
            resolved.overridden_scalar(key),
            Some("warning"),
            "the demotion is recorded as an applied override (not soft-rejected)",
        );
        assert_eq!(
            resolved.rejected_scalar_sets().count(),
            0,
            "a tunable demotion is never soft-rejected",
        );
    }

    /// A tunable check carries no floor — the demotion-lock is intrinsic-only, so
    /// `file-state.hash-matches` (a tunable check) is absent from the `floors`
    /// surface even though it is a declared knob.
    #[test]
    fn tunable_check_carries_no_floor() {
        let knobs = load_knobs(KNOBS_YAML).expect("knobs.yaml loads");
        assert!(
            knobs
                .field("validation.file-state.hash-matches.severity")
                .is_some()
        );
        assert!(
            knobs
                .floors()
                .get("validation.file-state.hash-matches.severity")
                .is_none()
        );
    }

    /// A `knobs.yaml` that omits the floor on one intrinsic key fails to load with
    /// the typed [`KnobError::UnflooredIntrinsic`] — a mis-declared pack cannot
    /// silently un-lock the determinism boundary (`overrides.md` → Locked keys;
    /// review M1 hybrid, assertion-only).
    #[test]
    fn unfloored_intrinsic_is_a_typed_error() {
        // The embedded surface with the floor stripped from one intrinsic key.
        let text = std::str::from_utf8(KNOBS_YAML).unwrap();
        let unfloored = text.replacen(
            "validation.workflow-refs.placeholder-resolves.severity:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n  floor: blocking\n",
            "validation.workflow-refs.placeholder-resolves.severity:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n",
            1,
        );
        assert_ne!(unfloored, text, "the strip must actually change the bytes");

        let err = load_knobs(unfloored.as_bytes()).expect_err("an unfloored intrinsic errors");
        assert!(
            matches!(err, KnobError::UnflooredIntrinsic(ref k)
                if k == "validation.workflow-refs.placeholder-resolves.severity"),
            "expected UnflooredIntrinsic, got {err:?}",
        );
    }

    /// A `knobs.yaml` that omits an intrinsic key entirely fails to load with the
    /// typed [`KnobError::UndeclaredIntrinsic`]. Defense-in-depth: the floor must
    /// be present to lock the check, never resting solely on the closed-surface
    /// (undeclared-key-hard-aborts) rule (`overrides.md` → Locked keys).
    #[test]
    fn undeclared_intrinsic_is_a_typed_error() {
        // Drop the whole declaration block for one intrinsic key.
        let text = std::str::from_utf8(KNOBS_YAML).unwrap();
        let stripped = text.replacen(
            "validation.workflow-refs.placeholder-resolves.severity:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n  floor: blocking\n",
            "",
            1,
        );
        assert_ne!(stripped, text, "the strip must actually remove the block");

        let err = load_knobs(stripped.as_bytes()).expect_err("an undeclared intrinsic errors");
        assert!(
            matches!(err, KnobError::UndeclaredIntrinsic(ref k)
                if k == "validation.workflow-refs.placeholder-resolves.severity"),
            "expected UndeclaredIntrinsic, got {err:?}",
        );
    }
}
