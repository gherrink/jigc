//! The document/workflow schema model — sections, slots, fields, relations.
//!
//! The deserializable in-memory model of a **doc-type definition**: the
//! config-family YAML a pack ships at `schemas/<type>.yaml`
//! (`design/document-type-schema.md` → On-disk definition format). The engine
//! parses an instance *against* this model (`design/storage.md` → Schema-driven
//! parse); the model knows every section (id, heading order, header-vs-body) and
//! every leaf (slot / typed field / repeatable block), which is exactly what the
//! parser and writer consume.
//!
//! This module is the *model + loader* only — it builds no instances and parses
//! no `.md`. It is presentation-free and domain-empty (the engine invariant):
//! the actual `commit` / `adr` definitions are pack bytes, fed in as raw YAML.
//!
//! See `design/structural-grammar.md` (the dialect-neutral skeleton) and
//! `design/document-type-schema.md` (the document dialect: the leaf kinds and
//! the `ref` relation metadata).

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A parsed doc-type definition: an ordered list of [`Section`]s plus the
/// document-level identity metadata (`type`, `location`, `id-from`).
///
/// Deserialized from the config-family YAML at `schemas/<type>.yaml`. The
/// `sections` list is in **document order** (top-to-bottom as the instance
/// renders), the property the parser and writer both depend on.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    /// The doc-type name (e.g. `adr`, `commit`).
    #[serde(rename = "type")]
    pub ty: String,

    /// The repo-relative directory persisted instances live in (e.g.
    /// `decisions/`). Absent for a transient type whose sink is not a file
    /// (the `commit` type's sink is the git message).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// The id of the field whose value is slugged into the document's frozen id
    /// (the H1 title field). `commit` has no persisted identity but still names
    /// its id-source field for symmetry.
    #[serde(rename = "id-from", default, skip_serializing_if = "Option::is_none")]
    pub id_from: Option<String>,

    /// The document's sections, in document order.
    pub sections: Vec<Section>,
}

/// One section of a document: a heading the CLI owns, carrying leaves.
///
/// A section is either **simple** (a slot and/or a trailing field group) or
/// **repeatable** (an ID'd list of item blocks). The `header` flag marks the
/// single front-matter section (rendered between `---` fences, not as a `##`
/// heading) — `design/storage.md` → Anatomy.
// NOTE: no `deny_unknown_fields` here — serde forbids it alongside the
// `#[serde(flatten)]` of `body` below (the flattened untagged enum must be free
// to consume the section's shape keys). Unknown *value* errors (bad field type,
// bad section shape) are still caught by the leaf structs' own guards.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Section {
    /// The section's stable id (and the source of its schema-fixed heading).
    pub id: String,

    /// `true` for the front-matter header section; `false`/absent for a body
    /// section.
    #[serde(default, skip_serializing_if = "is_false")]
    pub header: bool,

    /// Whether the section is simple or repeatable, with its leaves.
    #[serde(flatten)]
    pub body: SectionBody,
}

/// The two section shapes, distinguished on disk by which key is present.
///
/// A **simple** section carries an optional `slot` and an optional `fields`
/// list (a header section is the `fields`-only case; a body section is the
/// `slot`(+optional `fields`) case). A **repeatable** section carries a
/// `repeatable` block instead.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SectionBody {
    /// A repeatable section: an ID'd list of item blocks.
    Repeatable {
        /// The item-template: its id-source field plus the block's leaves.
        repeatable: Repeatable,
    },
    /// A simple section: an optional slot and an optional trailing field group.
    Simple {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        slot: Option<Slot>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fields: Vec<Field>,
    },
}

/// A repeatable section's item-template.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repeatable {
    /// The id of the block field slugged into each item's `{#id}` anchor.
    #[serde(rename = "id-from")]
    pub id_from: String,

    /// The item's leaves, in document order.
    pub block: Vec<Leaf>,
}

/// A leaf inside a repeatable item block: a typed field or an LLM-filled slot.
///
/// Distinguished on disk by which key is present (`slot:` vs the field's flat
/// `{id, type, …}` form).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Leaf {
    /// An LLM-authored prose slot (south of the determinism boundary).
    Slot {
        /// The leaf id (and on-disk sub-label).
        id: String,
        slot: Slot,
    },
    /// A CLI-adjudicated typed field.
    Field(Field),
}

/// An LLM-authored prose slot: the only thing south of the determinism boundary.
///
/// Carries only an optional authoring hint; presence/required is the writer's
/// concern, surfaced at `finalize`. (`design/document-type-schema.md` → Slot.)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slot {
    /// One-line guidance surfaced to the LLM when it fills this slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// A CLI-adjudicated typed field, with the relation metadata a `ref` carries.
///
/// `id` is the on-disk key; `ty` selects the engine-native or pack-provided
/// type. `ref`-specific keys (`to` / `card` / `inverse` / `inverse-card`) and
/// type-specific keys (`of` / `default` / `set`) ride alongside, per
/// `design/document-type-schema.md` → Field / Cross-references.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    /// The field id (and the on-disk key).
    pub id: String,

    /// The field's type.
    #[serde(rename = "type")]
    pub ty: FieldType,

    /// `enum` members (required for `type: enum`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub of: Option<Vec<String>>,

    /// A literal default value (e.g. an `enum`'s default member).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// When the CLI derives the value (e.g. `on-create` for a `date`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,

    /// A `ref`'s target type (required for `type: ref`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// A `ref`'s forward cardinality (default `"0..1"` when omitted).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,

    /// The name of a `ref`'s derived back-edge in the target type's read view.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inverse: Option<String>,

    /// A `ref`'s inverse-side cardinality (completeness obligation).
    #[serde(
        rename = "inverse-card",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inverse_card: Option<String>,
}

/// The field type vocabulary. Engine-native types are enumerated; `code-anchor`
/// is the one MVP pack-provided type whose adjudicator ships in a pack.
///
/// An undeclared type name is a typed [`SchemaError::UnknownFieldType`] at load,
/// never a panic — `deny_unknown_fields` rejects the *shape*; this rejects the
/// *value*.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FieldType {
    /// A controlled-choice member set (`of:` + optional `default:`).
    Enum,
    /// A constrained free value (maxlen / pattern; the slug id-source).
    String,
    /// A CLI-derived date.
    Date,
    /// A boolean.
    Bool,
    /// An integer.
    Int,
    /// A cross-reference carrying relation metadata.
    Ref,
    /// A pack-provided pointer at a module / symbol / test.
    CodeAnchor,
}

/// Why loading a schema from raw YAML failed.
#[derive(Debug, Error)]
pub enum SchemaError {
    /// The bytes were not valid UTF-8 (definitions are text).
    #[error("schema is not valid UTF-8")]
    NotUtf8,

    /// The YAML did not match the schema model: an unknown key, a missing
    /// required key, a malformed value, or an **undeclared field type**.
    #[error("malformed schema YAML: {0}")]
    Malformed(#[from] serde_yaml_ng::Error),
}

/// Parse a doc-type [`Schema`] from raw config-family YAML bytes.
///
/// The whole model deserializes through serde: an unknown key, a missing
/// required key, or an undeclared field type is a typed [`SchemaError`], never a
/// panic. The engine compiles in no schema content — the bytes are fed in (the
/// `commit` / `adr` definitions ride in the pack).
pub fn load_schema(bytes: &[u8]) -> Result<Schema, SchemaError> {
    let text = std::str::from_utf8(bytes).map_err(|_| SchemaError::NotUtf8)?;
    let schema = serde_yaml_ng::from_str(text)?;
    Ok(schema)
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two shipped MVP schemas, loaded from the embedded pack source tree so
    /// the test pins exactly the bytes that ship.
    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");
    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");

    /// Golden: the parsed `commit` schema projection. Pins section ids in
    /// document order, the header flag, the `subject` string field, and the
    /// `body` slot — the structure the parser/writer consume.
    #[test]
    fn schema_commit_golden() {
        let schema = load_schema(COMMIT_YAML).expect("commit.yaml loads");
        let json = serde_json::to_string_pretty(&schema).expect("serializes");
        insta::assert_snapshot!("schema_commit", json);
    }

    /// Golden: the parsed `adr` schema projection. Pins `location: decisions/`,
    /// `id-from: title`, the `status` enum members + default, the `date` field,
    /// the `supersedes` ref's `to: adr` / `card` / `inverse`, and the three
    /// prose slot sections in order.
    #[test]
    fn schema_adr_golden() {
        let schema = load_schema(ADR_YAML).expect("adr.yaml loads");
        let json = serde_json::to_string_pretty(&schema).expect("serializes");
        insta::assert_snapshot!("schema_adr", json);
    }

    /// The shipped `adr` schema's structure is reachable through the model (not
    /// just a snapshot string): the `supersedes` ref carries `to: adr` and its
    /// inverse, and `status` is an enum over the three members.
    #[test]
    fn adr_status_section_models_the_ref_relation() {
        let schema = load_schema(ADR_YAML).expect("adr.yaml loads");
        assert_eq!(schema.ty, "adr");
        assert_eq!(schema.location.as_deref(), Some("decisions/"));
        assert_eq!(schema.id_from.as_deref(), Some("title"));

        let status = &schema.sections[0];
        assert_eq!(status.id, "status");
        assert!(status.header);
        let SectionBody::Simple { slot, fields } = &status.body else {
            panic!("status is a simple header section");
        };
        assert!(slot.is_none());

        let status_field = fields.iter().find(|f| f.id == "status").unwrap();
        assert_eq!(status_field.ty, FieldType::Enum);
        assert_eq!(
            status_field.of.as_deref(),
            Some(
                ["proposed", "accepted", "superseded"]
                    .map(String::from)
                    .as_slice()
            )
        );
        assert_eq!(status_field.default.as_deref(), Some("proposed"));

        let supersedes = fields.iter().find(|f| f.id == "supersedes").unwrap();
        assert_eq!(supersedes.ty, FieldType::Ref);
        assert_eq!(supersedes.to.as_deref(), Some("adr"));
        assert_eq!(supersedes.card.as_deref(), Some("0..1"));
        assert_eq!(supersedes.inverse.as_deref(), Some("superseded-by"));
    }

    /// Golden: the parsed `spec` schema projection. Pins `location: specs/`,
    /// `id-from: title` (the slug derives from the document H1 — the spec carries
    /// NO `title` field, exactly like `adr`), the `goal`/`context` prose slots in
    /// order, and the repeatable `criteria` section (`id-from: title`, block =
    /// `title` field + `statement` slot). No `status`/`date`/`decided-by` — those
    /// are cut from the MVP spec.
    #[test]
    fn schema_spec_golden() {
        let schema = load_schema(SPEC_YAML).expect("spec.yaml loads");
        let json = serde_json::to_string_pretty(&schema).expect("serializes");
        insta::assert_snapshot!("schema_spec", json);
    }

    /// The shipped `spec` schema's structure is reachable through the model: the
    /// `criteria` repeatable block carries `statement` as a `Leaf::Slot`, the spec
    /// carries NO document-level `title` field (the slug derives from the H1, like
    /// `adr`), and the cut fields (`status`/`date`/`decided-by`) are genuinely
    /// absent.
    #[test]
    fn spec_criteria_block_carries_statement_as_a_slot() {
        let schema = load_schema(SPEC_YAML).expect("spec.yaml loads");
        assert_eq!(schema.ty, "spec");
        assert_eq!(schema.location.as_deref(), Some("specs/"));
        assert_eq!(schema.id_from.as_deref(), Some("title"));

        // `goal` is the first section — a prose slot, no fields (the spec has no
        // header/field section; its title is the document H1).
        let goal = &schema.sections[0];
        assert_eq!(goal.id, "goal");
        let SectionBody::Simple { slot, fields } = &goal.body else {
            panic!("goal is a simple slot section");
        };
        assert!(slot.is_some());
        assert!(fields.is_empty());

        // No document-level title field, and no status/date/decided-by, ship
        // anywhere in the simple sections (the criteria block's own `title`
        // id-source field is a repeatable-block leaf, not a document field).
        let all_field_ids: Vec<&str> = schema
            .sections
            .iter()
            .filter_map(|s| match &s.body {
                SectionBody::Simple { fields, .. } => Some(fields),
                SectionBody::Repeatable { .. } => None,
            })
            .flatten()
            .map(|f| f.id.as_str())
            .collect();
        assert!(!all_field_ids.contains(&"title"));
        assert!(!all_field_ids.contains(&"status"));
        assert!(!all_field_ids.contains(&"date"));
        assert!(!all_field_ids.contains(&"decided-by"));

        // The criteria repeatable block carries `statement` as a prose slot.
        let criteria = &schema.sections[2];
        assert_eq!(criteria.id, "criteria");
        let SectionBody::Repeatable { repeatable } = &criteria.body else {
            panic!("criteria is repeatable");
        };
        assert_eq!(repeatable.id_from, "title");
        assert!(matches!(&repeatable.block[0], Leaf::Field(f) if f.id == "title"));
        assert!(matches!(&repeatable.block[1], Leaf::Slot { id, .. } if id == "statement"));
    }

    /// A repeatable section round-trips through the model: id-source field +
    /// block leaves (slot and field). Exercises the `repeatable` shape the MVP
    /// schemas don't yet use, so the model is proven against the design's
    /// SPEC-criteria example.
    #[test]
    fn repeatable_section_models_an_item_block() {
        let yaml = b"\
type: spec
sections:
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"The criterion, testably phrased.\" } }
        - { id: maps-to-test, type: code-anchor }
";
        let schema = load_schema(yaml).expect("repeatable schema loads");
        let SectionBody::Repeatable { repeatable } = &schema.sections[0].body else {
            panic!("criteria is repeatable");
        };
        assert_eq!(repeatable.id_from, "title");
        assert_eq!(repeatable.block.len(), 3);
        assert!(matches!(&repeatable.block[1], Leaf::Slot { id, .. } if id == "statement"));
        let Leaf::Field(anchor) = &repeatable.block[2] else {
            panic!("maps-to-test is a field");
        };
        assert_eq!(anchor.ty, FieldType::CodeAnchor);
    }

    /// An undeclared field type is a typed error, not a panic — the core
    /// malformed-schema done-criterion.
    #[test]
    fn unknown_field_type_is_a_typed_error() {
        let yaml = b"\
type: bad
sections:
  - id: header
    header: true
    fields:
      - { id: x, type: wormhole }
";
        let err = load_schema(yaml).expect_err("undeclared field type errors");
        assert!(
            matches!(err, SchemaError::Malformed(_)),
            "expected a typed Malformed error, got {err:?}",
        );
    }

    /// An unknown top-level key is rejected (the `deny_unknown_fields` guard),
    /// so schema typos surface as typed errors instead of silent drops.
    #[test]
    fn unknown_top_level_key_is_a_typed_error() {
        let yaml = b"\
type: bad
locaiton: decisions/
sections: []
";
        let err = load_schema(yaml).expect_err("unknown key errors");
        assert!(matches!(err, SchemaError::Malformed(_)), "got {err:?}");
    }

    /// Non-UTF-8 bytes are a typed error, never a panic.
    #[test]
    fn non_utf8_is_a_typed_error() {
        let err = load_schema(&[0xff, 0xfe]).expect_err("non-utf8 errors");
        assert!(matches!(err, SchemaError::NotUtf8), "got {err:?}");
    }
}
