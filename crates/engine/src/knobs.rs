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
}

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
}

/// Parse the closed knob surface from raw `config/knobs.yaml` bytes.
///
/// Each top-level key is a settable knob; its `{type, of?, default}` body folds
/// into a [`Field`] (id = the key) reusing the document-type field model. Every
/// knob must declare a `default` (the value the resolver seeds the base map with);
/// an absent default is a [`KnobError::MissingDefault`].
pub fn load_knobs(bytes: &[u8]) -> Result<KnobSet, KnobError> {
    let text = std::str::from_utf8(bytes).map_err(|_| KnobError::NotUtf8)?;
    let decls: BTreeMap<String, KnobDecl> = serde_yaml_ng::from_str(text)?;

    let mut fields = Vec::with_capacity(decls.len());
    let mut defaults = BTreeMap::new();
    for (key, decl) in decls {
        let default = decl
            .default
            .ok_or_else(|| KnobError::MissingDefault(key.clone()))?;
        defaults.insert(key.clone(), default);
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
        });
    }

    Ok(KnobSet { fields, defaults })
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
    /// the full per-check severity surface (17 inventory rows, per
    /// `validation.md` → MVP check inventory) plus the two retained M4 per-probe
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
            // commit-rendering (2, advisory-by-default convention checks).
            (
                "validation.commit-rendering.line-limit-body.severity",
                "advisory",
            ),
            (
                "validation.commit-rendering.line-limit-subject.severity",
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
            // schema-completeness (1, advisory at task scope by design).
            (
                "validation.schema-completeness.inverse-cardinality.severity",
                "advisory",
            ),
            // schema-conformance (4, intrinsic).
            (
                "validation.schema-conformance.field-value-conformant.severity",
                "blocking",
            ),
            (
                "validation.schema-conformance.ref-resolves.severity",
                "blocking",
            ),
            (
                "validation.schema-conformance.required-field-present.severity",
                "blocking",
            ),
            (
                "validation.schema-conformance.required-slot-present.severity",
                "blocking",
            ),
            // workflow-refs (7, intrinsic) + the M4 per-probe default (retained).
            (
                "validation.workflow-refs.at-marker-on-non-scalar.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.body-include-only.severity",
                "blocking",
            ),
            (
                "validation.workflow-refs.command-ref-resolves.severity",
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
            ("validation.workflow-refs.severity", "blocking"),
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
}
