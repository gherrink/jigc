//! The deterministic transform **driver** — step 2 of the schema-diff → byte-stable
//! splice pipeline (`design/corpus-migration.md` → The deterministic transform).
//!
//! Consumes the classified diff produced by [`crate::schema_diff::schema_diff`] and
//! emits the ordered sequence of **byte-stable splices** over the proven `write.rs`
//! primitives that carries a v1-shaped instance onto its v2 shape — **no LLM in the
//! structural path** (the determinism boundary: the CLI owns structure, the LLM owns
//! only prose). Same classified diff in → same bytes out.
//!
//! This increment (M34 Inc-2 T3) builds the two structural branches with **no live
//! case**, provable only synthetically:
//!
//! - **`fixed-slot→repeatable-with-default`** — promote a simple `<<slot>>` section
//!   into its repeatable shape, carrying the old slot prose as the default first item,
//!   via the net-new [`crate::write::promote_slot_to_repeatable`] primitive (T2). The
//!   default item's **title is the section id** — a deterministic, domain-empty,
//!   always-slug-able source (the engine invents no prose); a human may rename the
//!   heading afterward as an ordinary edit.
//! - **`widened-cardinality`** — an instance-byte **identity** transform: the existing
//!   value stays valid under the widened cardinality, so the bytes are unchanged.
//!
//! The remaining two classified kinds are deferred per the branch-split
//! ([DECISIONS.md](../../../DECISIONS.md) → 2026-06-25): `prose-needing` (the
//! Framing-A escape hatch) lands in **T4**, and `added-optional-field` in **M34
//! Inc-3** (proven live by the schema-version-stamp dogfood). The driver surfaces them
//! as [`TransformError::Unsupported`] rather than silently skipping — an un-built
//! branch must block, never drop a change.

use crate::schema::Schema;
use crate::schema_diff::SchemaChange;
use crate::write::{self, GenerateError};

/// A failure applying a classified diff to one instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransformError {
    /// A structural splice primitive failed (an absent/non-conformant target, an
    /// unslug-able promotion title, …). Carries the underlying [`GenerateError`].
    Generate(GenerateError),
    /// A classified change kind whose driver branch is **not built in this increment**
    /// — `prose-needing` (T4) and `added-optional-field` (M34 Inc-3). Surfaced, never
    /// silently skipped, so an un-built branch blocks the migration rather than
    /// dropping a change.
    Unsupported {
        /// The classified kind's wire name.
        kind: &'static str,
        /// The section the change concerns.
        section: String,
    },
}

impl From<GenerateError> for TransformError {
    fn from(err: GenerateError) -> Self {
        TransformError::Generate(err)
    }
}

/// Apply a classified diff to one v1-shaped `source`, returning its v2-shaped bytes.
///
/// Folds the `changes` in classifier (document) order, each emitting a byte-stable
/// splice over the prior result. `old_schema` / `new_schema` are the schema pair the
/// diff was classified from (the promotion primitive re-derives the old slot prose
/// through `old_schema` and places it under `new_schema`'s repeatable shape).
pub fn transform(
    old_schema: &Schema,
    new_schema: &Schema,
    source: &str,
    changes: &[SchemaChange],
) -> Result<String, TransformError> {
    let mut out = source.to_string();
    for change in changes {
        match change {
            SchemaChange::FixedSlotToRepeatable { section } => {
                // The default first item's title is the section id (deterministic,
                // domain-empty, always slug-able). The primitive carries the old slot
                // prose verbatim as that item's body.
                out = write::promote_slot_to_repeatable(
                    old_schema, new_schema, &out, section, section,
                )?;
            }
            SchemaChange::WidenedCardinality { .. } => {
                // Instance-byte identity: the existing value is still valid under the
                // widened cardinality, so no splice is needed.
            }
            SchemaChange::ProseNeeding { section, .. } => {
                return Err(TransformError::Unsupported {
                    kind: "prose-needing",
                    section: section.clone(),
                });
            }
            SchemaChange::AddedOptionalField { section, .. } => {
                return Err(TransformError::Unsupported {
                    kind: "added-optional-field",
                    section: section.clone(),
                });
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    //! The reconstructed-shape e2e: replay two historical v1→v2 shape changes on
    //! v0-shaped instances and assert each transformed instance (a) round-trips
    //! byte-identical (`render(parse(out)) == out`), (b) preserves every prior
    //! slot/field value (no-data-loss), (c) conforms against the v2 schema, and (d)
    //! is deterministic — all with **no LLM** in the path (the driver is pure
    //! `schema_diff` + `write` splices).

    use super::*;
    use crate::field_block::{Field, Value};
    use crate::parse::parse_sections;
    use crate::schema::load_schema;
    use crate::schema_diff::schema_diff;
    use crate::validate::schema_conformance;
    use crate::write::{Instance, SectionContent, instance_from_source, render};

    /// Parse `out` against `schema` and assert it carries **zero** conformance
    /// findings — the `conformance_for` clean half of the done-criterion.
    fn assert_conforms(schema: &Schema, out: &str) {
        let doc = parse_sections(schema, out).expect("transformed doc parses under v2");
        let findings = schema_conformance(schema, out, &doc);
        assert!(
            findings.is_empty(),
            "transformed doc must conform to v2; got {findings:?}"
        );
    }

    /// `render(parse(out)) == out` under the v2 schema — the byte-stability half.
    fn assert_byte_stable(schema: &Schema, out: &str) {
        let inst = instance_from_source(schema, out).expect("transformed doc re-parses under v2");
        assert_eq!(
            render(schema, &inst),
            out,
            "transformed doc must round-trip byte-identical"
        );
    }

    // ---- (a) the reconstructed M25 `prd.requirements` fixed-slot→repeatable reshape ----

    /// v0/v1: `prd.requirements` is a **simple slot** (sitting between a leading
    /// `vision` slot and a trailing `success` slot, so the splice is on a non-trailing
    /// section).
    fn prd_v1() -> Schema {
        load_schema(
            b"\
type: prd
sections:
  - id: vision
    slot: { hint: \"the product vision\" }
  - id: requirements
    slot: { hint: \"the requirements prose\" }
  - id: success
    slot: { hint: \"success criteria\" }
",
        )
        .expect("prd v1 loads")
    }

    /// v2: `prd.requirements` is now a **single-slot repeatable** (`title` id-from +
    /// one `statement` slot — the reconstructed M25 reshape). The neighbour sections
    /// are byte-identical to v1.
    fn prd_v2() -> Schema {
        load_schema(
            b"\
type: prd
sections:
  - id: vision
    slot: { hint: \"the product vision\" }
  - id: requirements
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one requirement\" } }
  - id: success
    slot: { hint: \"success criteria\" }
",
        )
        .expect("prd v2 loads")
    }

    /// A canonical v0-shaped PRD whose `requirements` slot carries multi-paragraph
    /// prose (built through [`render`] so the input is the exact byte-stable form a
    /// first-touch-canonicalized corpus doc has).
    fn prd_v0_doc() -> (String, String) {
        let prose = "The system must accept orders.\n\nIt must reject duplicate \
                     submissions within a 60s window.";
        let inst = Instance {
            title: "Checkout PRD".to_string(),
            sections: vec![
                SectionContent {
                    id: "vision".to_string(),
                    slot: Some("A frictionless checkout.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "requirements".to_string(),
                    slot: Some(prose.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "success".to_string(),
                    slot: Some("Cart abandonment drops 20%.".to_string()),
                    ..Default::default()
                },
            ],
        };
        (render(&prd_v1(), &inst), prose.to_string())
    }

    #[test]
    fn fixed_slot_to_repeatable_e2e_replays_the_prd_requirements_reshape() {
        let v1 = prd_v1();
        let v2 = prd_v2();
        let (src, original_prose) = prd_v0_doc();

        // The diff is computed by the real classifier (T1), then fed to the driver —
        // the driver is exercised on the emitted classification, not a hand-built list.
        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::FixedSlotToRepeatable {
                section: "requirements".to_string()
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("transform succeeds");

        // (c) conforms against the v2 schema, and (a) round-trips byte-identical.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // (b) no-data-loss: requirements is now exactly one item carrying the v0 slot
        // prose verbatim; vision/success slots are unchanged.
        let out_inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let req = out_inst
            .sections
            .iter()
            .find(|s| s.id == "requirements")
            .expect("requirements present");
        assert!(
            req.slot.is_none(),
            "the section-level slot is gone post-promotion"
        );
        assert_eq!(req.items.len(), 1, "exactly one default item");
        assert_eq!(req.items[0].slot.as_deref(), Some(original_prose.as_str()));
        let slot_of = |id: &str| {
            out_inst
                .sections
                .iter()
                .find(|s| s.id == id)
                .and_then(|s| s.slot.clone())
        };
        assert_eq!(
            slot_of("vision").as_deref(),
            Some("A frictionless checkout.")
        );
        assert_eq!(
            slot_of("success").as_deref(),
            Some("Cart abandonment drops 20%.")
        );

        // (d) determinism: the same diff over the same source yields identical bytes.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(again, out, "transform is deterministic");
    }

    // ---- (b) a synthetic `widened-cardinality` reshape (instance-byte identity) ----

    /// v1: a `spec` whose `derived-from` ref carries `card: 0..1`.
    fn spec_v1() -> Schema {
        load_schema(
            b"\
type: spec
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: prd, card: \"0..1\" }
  - id: body
    slot: { hint: \"the spec body\" }
",
        )
        .expect("spec v1 loads")
    }

    /// v2: identical but `derived-from` is widened to `card: 0..*`.
    fn spec_v2() -> Schema {
        load_schema(
            b"\
type: spec
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: prd, card: \"0..*\" }
  - id: body
    slot: { hint: \"the spec body\" }
",
        )
        .expect("spec v2 loads")
    }

    /// A canonical v0-shaped spec carrying a single `derived-from` ref value.
    fn spec_v0_doc() -> String {
        let inst = Instance {
            title: "Checkout spec".to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![Field {
                        key: "derived-from".to_string(),
                        value: Value::Scalar("prd:checkout".to_string()),
                    }],
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    slot: Some("The checkout flow rejects duplicate orders.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&spec_v1(), &inst)
    }

    #[test]
    fn widened_cardinality_e2e_is_a_byte_identity_transform() {
        let v1 = spec_v1();
        let v2 = spec_v2();
        let src = spec_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::WidenedCardinality {
                section: "meta".to_string(),
                field: "derived-from".to_string()
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("transform succeeds");

        // Instance-byte identity: the v0 value stays valid under the widened card.
        assert_eq!(
            out, src,
            "widened-cardinality leaves the instance bytes unchanged"
        );

        // (c) conforms against v2, (a) round-trips byte-identical, (b) the value survives.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);
        let out_inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let derived = out_inst
            .sections
            .iter()
            .find(|s| s.id == "meta")
            .and_then(|s| s.fields.iter().find(|f| f.key == "derived-from"))
            .map(|f| f.value.clone());
        assert_eq!(derived, Some(Value::Scalar("prd:checkout".to_string())));

        // (d) determinism.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(again, out, "transform is deterministic");
    }

    // ---- the deferred branches block, never silently drop ----

    /// A `prose-needing` change (T4's branch) and an `added-optional-field` change
    /// (M34 Inc-3's branch) are surfaced as [`TransformError::Unsupported`], not
    /// silently skipped — an un-built branch must block the migration.
    #[test]
    fn deferred_kinds_surface_as_unsupported() {
        let v1 = prd_v1();
        let (src, _) = prd_v0_doc();
        let prose_needing = vec![SchemaChange::ProseNeeding {
            section: "vision".to_string(),
            leaf: None,
        }];
        assert_eq!(
            transform(&v1, &v1, &src, &prose_needing),
            Err(TransformError::Unsupported {
                kind: "prose-needing",
                section: "vision".to_string()
            })
        );
        let added = vec![SchemaChange::AddedOptionalField {
            section: "vision".to_string(),
            field: "owner".to_string(),
        }];
        assert_eq!(
            transform(&v1, &v1, &src, &added),
            Err(TransformError::Unsupported {
                kind: "added-optional-field",
                section: "vision".to_string()
            })
        );
    }
}
