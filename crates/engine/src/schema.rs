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

    /// Authored-prose: what this doc-type *is* (its identity, one or two
    /// sentences). Top-level, optional, human-authored at definition time; the
    /// `describe` self-description surface projects it. Skip-on-absent: a schema
    /// that omits it leaves the golden untouched
    /// (`design/document-type-schema.md` → Authored metadata fields).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Authored-prose: when and why you'd reach for this doc-type. Top-level,
    /// optional, human-authored at definition time; projected by `describe`.
    /// Usage, never mechanism (`design/introspection.md`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,

    /// `true` for a **singleton** doctype: a running doc with a **fixed slug = the
    /// type id** (e.g. `roadmap/roadmap.md`), *not* an `id-from: title` slug. A
    /// re-`create` then deterministically targets the same committed file — the
    /// premise idempotent-create rests on (`design/methodology-docs.md` → The four
    /// doctypes, review finding B-2). Skip-on-false (mirrors `header`): a schema
    /// omitting it leaves every existing golden byte-unchanged.
    #[serde(default, skip_serializing_if = "is_false")]
    pub singleton: bool,

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
    /// A CLI-adjudicated typed field. Boxed because [`Field`] is much larger
    /// than the slot variant (the `large_enum_variant` lint).
    Field(Box<Field>),
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
/// `id` is the on-disk key; `ty` selects the engine-native or pack-declared
/// type. `ref`-specific keys (`to` / `card` / `inverse` / `inverse-card`) and
/// type-specific keys (`of` / `default` / `set`) ride alongside, per
/// `design/document-type-schema.md` → Field / Cross-references.
///
/// The `type` key deserializes into a *raw* [`RawFieldType`] string (a closed
/// native name, or any other string left unresolved); [`load_schema`] then
/// resolves it against the supplied pack-declared type set into the typed
/// [`Field::ty`] — promoting a declared name to a [`FieldType::Pack`] carrying
/// its adjudicator binding, and rejecting an undeclared name with a typed
/// [`SchemaError::UnknownFieldType`] (never a panic). This two-step resolve is
/// why the engine ships **no** pack field type yet admits one a pack declares
/// (the engine-empty invariant).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    /// The field id (and the on-disk key).
    pub id: String,

    /// The field's type, resolved against the pack-declared type set at load.
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

    /// Per-field override of the resolved field-type's `check:` predicate (the
    /// M13 selector). When present, the `doc-code` probe runs this predicate for
    /// this field instead of the type's declared default — so a repeatable
    /// `code-anchor` may carry `check: criterion-maps-to-test`. Absent leaves the
    /// type's check in force. See `design/architecture-documentation.md` → The
    /// per-field-type predicate selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<String>,
}

/// The field type vocabulary — **engine-native variants + a pack-declared
/// variant** (the M10 extension axis). Native types are a closed set the engine
/// owns; [`FieldType::Pack`] carries a type a *pack* declares (its spelling +
/// the adjudicator probe bound to it). The engine ships **no** pack type itself
/// (the engine-empty invariant): `code-anchor` lives in the dev pack, declared
/// as a `(name, adjudicator)` pair (`design/document-type-schema.md` →
/// Pack-declared field types).
///
/// **Deserialization is two-step.** Serde maps a known kebab string to its
/// native variant; **any other string** deserializes to an *unresolved*
/// [`FieldType::Pack`] (`adjudicator: None`). [`load_schema`] then resolves each
/// unresolved `Pack` against the supplied pack-declared type set — promoting a
/// **declared** name to carry its adjudicator binding, and rejecting an
/// **undeclared** name with a typed [`SchemaError::UnknownFieldType`], never a
/// panic. This is why the engine admits a pack-supplied type while shipping
/// none: the *name→adjudicator* binding rides in from the pack at load.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// A pack-declared type: its declared spelling plus the adjudicator probe
    /// bound to it. `adjudicator` is `None` until [`load_schema`] resolves the
    /// name against the supplied pack-declared set; an unresolved `Pack`
    /// surviving to resolution with an undeclared name is the
    /// [`SchemaError::UnknownFieldType`] case.
    Pack(PackFieldType),
}

impl FieldType {
    /// The closed set of engine-native type spellings (kebab on disk). A
    /// `type:` string outside this set is a pack-declared candidate.
    const NATIVE: &'static [(&'static str, FieldType)] = &[
        ("enum", FieldType::Enum),
        ("string", FieldType::String),
        ("date", FieldType::Date),
        ("bool", FieldType::Bool),
        ("int", FieldType::Int),
        ("ref", FieldType::Ref),
    ];

    /// The on-disk spelling of this type (the inverse of [`Self::NATIVE`]).
    fn as_str(&self) -> &str {
        match self {
            FieldType::Enum => "enum",
            FieldType::String => "string",
            FieldType::Date => "date",
            FieldType::Bool => "bool",
            FieldType::Int => "int",
            FieldType::Ref => "ref",
            FieldType::Pack(p) => &p.name,
        }
    }
}

// On disk a field type is a bare string. A native spelling maps to its variant;
// any other string becomes an *unresolved* `Pack` (`adjudicator: None`) that
// `load_schema` then resolves against the pack-declared set (or rejects with a
// typed `UnknownFieldType`). Serializing emits the bare spelling back.
impl Serialize for FieldType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for FieldType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = std::string::String::deserialize(deserializer)?;
        Ok(Self::NATIVE
            .iter()
            .find(|(spelling, _)| *spelling == name)
            .map(|(_, ty)| ty.clone())
            .unwrap_or(FieldType::Pack(PackFieldType {
                name,
                adjudicator: None,
                check: None,
            })))
    }
}

/// A pack-declared field type: a `(name, adjudicator-probe)` pair.
///
/// The pack declares the type's spelling (`code-anchor`) and the **probe** that
/// adjudicates it (`doc-code`); the binding falls out of the type — a leaf of
/// this type *means* its bound probe applies, exactly as a `ref` leaf means
/// `ref-resolves` applies. `adjudicator` is `None` for a freshly-deserialized
/// (still-unresolved) type and `Some(probe)` once [`load_schema`] has resolved
/// the name against the supplied pack-declared set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackFieldType {
    /// The type's declared spelling (the on-disk `type:` value, e.g.
    /// `code-anchor`).
    pub name: String,

    /// The probe bound to adjudicate this type (e.g. `doc-code`). `None` until
    /// resolved against the pack-declared set at load.
    pub adjudicator: Option<String>,

    /// The predicate the bound adjudicator runs for this type (e.g.
    /// `symbol-exists`), resolved from the pack declaration. `None` until
    /// [`load_schema`] resolves the name; a schema field's own `check:` overrides
    /// it per-field. See `design/architecture-documentation.md` → The
    /// per-field-type predicate selector.
    pub check: Option<String>,
}

/// A pack's field-type declaration: the `(name, adjudicator-probe)` pair a pack
/// supplies so a schema field may name it. The engine ships none; the dev pack
/// declares `code-anchor` → `doc-code`. Threaded into [`load_schema_with_types`]
/// as the set against which an unresolved [`FieldType::Pack`] is resolved.
///
/// Deserializes directly from the pack's config-family declaration file (a YAML
/// sequence of `{ name, adjudicator }` entries) — the on-disk form the dev pack
/// supplies at `config/field-types.yaml`, the CLI reads, and threads in here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackTypeDecl {
    /// The type's spelling (e.g. `code-anchor`).
    pub name: String,
    /// The probe bound to adjudicate it (e.g. `doc-code`).
    pub adjudicator: String,
    /// The predicate the adjudicator runs for this type (e.g. `symbol-exists`).
    /// **Required — no implicit engine default.** A pack declaring a type must
    /// spell its predicate; defaulting it would silently strip `spec.criteria`'s
    /// `criterion-maps-to-test` (the VISION headline check). A schema field may
    /// override it per-field. See `design/architecture-documentation.md` → The
    /// per-field-type predicate selector (the absent-default trap).
    pub check: String,
}

/// Why loading a schema from raw YAML failed.
#[derive(Debug, Error)]
pub enum SchemaError {
    /// The bytes were not valid UTF-8 (definitions are text).
    #[error("schema is not valid UTF-8")]
    NotUtf8,

    /// The YAML did not match the schema model: an unknown key, a missing
    /// required key, or a malformed value.
    #[error("malformed schema YAML: {0}")]
    Malformed(#[from] serde_yaml_ng::Error),

    /// A field named a type that is **neither engine-native nor pack-declared**.
    /// The `type:` string parsed fine but resolves to nothing — the M10
    /// extension axis rejects an undeclared name loudly here, never a panic.
    #[error("field `{field}` names undeclared type `{ty}` (not engine-native, not pack-declared)")]
    UnknownFieldType {
        /// The field id whose type is undeclared.
        field: String,
        /// The undeclared type spelling the field named.
        ty: String,
    },
}

/// Parse a doc-type [`Schema`] from raw config-family YAML bytes, with **no**
/// pack-declared types in scope — engine-native field types only.
///
/// A schema field naming a non-native type (e.g. `code-anchor`) is therefore a
/// typed [`SchemaError::UnknownFieldType`] here: a pack type loads **only** when
/// its declaration is threaded in via [`load_schema_with_types`]. The engine
/// compiles in no schema content nor any pack type (the engine-empty invariant);
/// the `commit` / `adr` / `spec` definitions and the `code-anchor` declaration
/// ride in the pack.
pub fn load_schema(bytes: &[u8]) -> Result<Schema, SchemaError> {
    load_schema_with_types(bytes, &[])
}

/// Parse a doc-type [`Schema`], resolving each field's type against the supplied
/// **pack-declared** type set (the `(name, adjudicator-probe)` pairs the pack
/// declares — the M10 extension axis).
///
/// The whole model deserializes through serde: an unknown key, a missing
/// required key, or a malformed value is a typed [`SchemaError::Malformed`]. A
/// field whose `type:` is not engine-native deserializes to an unresolved
/// [`FieldType::Pack`]; this pass then resolves it:
///
/// - a name **present** in `pack_types` is promoted to carry its adjudicator
///   binding (so a `code-anchor` leaf *means* its `doc-code` probe applies),
/// - a name **absent** from both the native set and `pack_types` is a typed
///   [`SchemaError::UnknownFieldType`], never a panic.
///
/// The engine ships no pack type itself; `pack_types` comes from the pack.
pub fn load_schema_with_types(
    bytes: &[u8],
    pack_types: &[PackTypeDecl],
) -> Result<Schema, SchemaError> {
    let text = std::str::from_utf8(bytes).map_err(|_| SchemaError::NotUtf8)?;
    let mut schema: Schema = serde_yaml_ng::from_str(text)?;
    for section in &mut schema.sections {
        match &mut section.body {
            SectionBody::Simple { fields, .. } => {
                for field in fields {
                    resolve_field_type(field, pack_types)?;
                }
            }
            SectionBody::Repeatable { repeatable } => {
                for leaf in &mut repeatable.block {
                    if let Leaf::Field(field) = leaf {
                        resolve_field_type(field, pack_types)?;
                    }
                }
            }
        }
    }
    Ok(schema)
}

/// Resolve one field's (possibly unresolved) [`FieldType::Pack`] against the
/// pack-declared set: bind the adjudicator if declared, else
/// [`SchemaError::UnknownFieldType`]. Native types are already resolved.
fn resolve_field_type(field: &mut Field, pack_types: &[PackTypeDecl]) -> Result<(), SchemaError> {
    if let FieldType::Pack(pack) = &mut field.ty {
        match pack_types.iter().find(|d| d.name == pack.name) {
            Some(decl) => {
                pack.adjudicator = Some(decl.adjudicator.clone());
                pack.check = Some(decl.check.clone());
            }
            None => {
                return Err(SchemaError::UnknownFieldType {
                    field: field.id.clone(),
                    ty: pack.name.clone(),
                });
            }
        }
    }
    Ok(())
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// Test-only: the dev pack's single M10 field-type declaration (`code-anchor` →
/// `doc-code`). The engine ships none; cross-module test fixtures that load an
/// inline schema carrying a `code-anchor` leaf thread this in to resolve it.
#[cfg(test)]
pub(crate) fn dev_pack_field_types() -> Vec<PackTypeDecl> {
    vec![PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
    }]
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
    /// the `supersedes` ref's `to: adr` / `card` / `inverse`, the optional
    /// `cites-code` pack-declared `code-anchor` field (M10 inc-1, resolved with its
    /// `doc-code` adjudicator), and the three prose slot sections in order. The
    /// `cites-code` row is the justified, intended output-adding snapshot diff.
    #[test]
    fn schema_adr_golden() {
        let schema =
            load_schema_with_types(ADR_YAML, &dev_pack_field_types()).expect("adr.yaml loads");
        let json = serde_json::to_string_pretty(&schema).expect("serializes");
        insta::assert_snapshot!("schema_adr", json);
    }

    /// The shipped `adr` schema's structure is reachable through the model (not
    /// just a snapshot string): the `supersedes` ref carries `to: adr` and its
    /// inverse, and `status` is an enum over the three members.
    #[test]
    fn adr_status_section_models_the_ref_relation() {
        let schema =
            load_schema_with_types(ADR_YAML, &dev_pack_field_types()).expect("adr.yaml loads");
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
        let schema =
            load_schema_with_types(SPEC_YAML, &dev_pack_field_types()).expect("spec.yaml loads");
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
        let schema =
            load_schema_with_types(SPEC_YAML, &dev_pack_field_types()).expect("spec.yaml loads");
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

    use super::dev_pack_field_types as code_anchor_decl;

    /// A repeatable section round-trips through the model: id-source field +
    /// block leaves (slot and field). Exercises the `repeatable` shape the MVP
    /// schemas don't yet use, so the model is proven against the design's
    /// SPEC-criteria example — including a pack-declared `code-anchor` leaf,
    /// which loads only when the type is threaded in.
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
        let schema =
            load_schema_with_types(yaml, &code_anchor_decl()).expect("repeatable schema loads");
        let SectionBody::Repeatable { repeatable } = &schema.sections[0].body else {
            panic!("criteria is repeatable");
        };
        assert_eq!(repeatable.id_from, "title");
        assert_eq!(repeatable.block.len(), 3);
        assert!(matches!(&repeatable.block[1], Leaf::Slot { id, .. } if id == "statement"));
        let Leaf::Field(anchor) = &repeatable.block[2] else {
            panic!("maps-to-test is a field");
        };
        // The leaf in a repeatable block resolves against the declared set too:
        // it carries the pack name AND its bound adjudicator through the model.
        assert_eq!(
            anchor.ty,
            FieldType::Pack(PackFieldType {
                name: "code-anchor".to_owned(),
                adjudicator: Some("doc-code".to_owned()),
                check: Some("symbol-exists".to_owned()),
            })
        );
    }

    /// (i) A field typed `code-anchor` loads **only when** `code-anchor` is in
    /// the supplied pack-declared set, and the resolved type carries its
    /// `doc-code` adjudicator binding through the model — the M10 extension-axis
    /// done-criterion. The engine itself ships no such type (the set is fed in).
    #[test]
    fn pack_declared_field_type_loads_with_its_adjudicator_binding() {
        let yaml = b"\
type: adr
sections:
  - id: status
    header: true
    fields:
      - { id: cites-code, type: code-anchor }
";
        let schema =
            load_schema_with_types(yaml, &code_anchor_decl()).expect("declared pack type loads");
        let SectionBody::Simple { fields, .. } = &schema.sections[0].body else {
            panic!("status is a simple header section");
        };
        let cites = fields.iter().find(|f| f.id == "cites-code").unwrap();
        let FieldType::Pack(pack) = &cites.ty else {
            panic!(
                "cites-code resolves to a pack-declared type, got {:?}",
                cites.ty
            );
        };
        assert_eq!(pack.name, "code-anchor");
        assert_eq!(
            pack.adjudicator.as_deref(),
            Some("doc-code"),
            "the resolved pack type carries its bound adjudicator probe",
        );
    }

    /// A `code-anchor` field is undeclared when **no** pack type set is supplied
    /// — the engine-empty invariant in action: the engine knows no `code-anchor`
    /// of its own, so the bare loader rejects it loudly with the typed
    /// `UnknownFieldType` (never a panic, never a silent native fallback).
    #[test]
    fn pack_type_absent_from_set_is_unknown_field_type() {
        let yaml = b"\
type: adr
sections:
  - id: status
    header: true
    fields:
      - { id: cites-code, type: code-anchor }
";
        let err = load_schema(yaml).expect_err("undeclared code-anchor errors");
        assert!(
            matches!(&err, SchemaError::UnknownFieldType { field, ty }
                if field == "cites-code" && ty == "code-anchor"),
            "expected UnknownFieldType for cites-code/code-anchor, got {err:?}",
        );
    }

    /// (ii) A field typed with an **undeclared** name fails loudly with a typed
    /// `SchemaError::UnknownFieldType` — never a panic, never a generic serde
    /// `Malformed` (the `type:` string parses fine; it resolves to nothing). The
    /// error names the offending field and type for diagnosis.
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
        let err =
            load_schema_with_types(yaml, &code_anchor_decl()).expect_err("undeclared type errors");
        assert!(
            matches!(&err, SchemaError::UnknownFieldType { field, ty }
                if field == "x" && ty == "wormhole"),
            "expected a typed UnknownFieldType error, got {err:?}",
        );
    }

    /// The loop with T1, closed over the **shipped** bytes: the real `adr.yaml` /
    /// `spec.yaml` (which now carry the pack-declared `code-anchor` anchors)
    /// **load** when the `code-anchor → doc-code` set is supplied, and **fail to
    /// load** with the bare loader (no pack types) — the engine ships no
    /// `code-anchor` of its own (the engine-empty invariant), so the bare path
    /// rejects the shipped anchor loudly with `UnknownFieldType`. This is the M10
    /// inc-1 done-criterion over the bytes that ship, not an inline fixture.
    #[test]
    fn shipped_adr_and_spec_load_with_the_pack_set_and_fail_without_it() {
        for (bytes, field) in [(ADR_YAML, "cites-code"), (SPEC_YAML, "maps-to-test")] {
            load_schema_with_types(bytes, &dev_pack_field_types())
                .expect("loads when code-anchor is declared");

            let err = load_schema(bytes).expect_err("the bare loader rejects code-anchor");
            assert!(
                matches!(&err, SchemaError::UnknownFieldType { field: f, ty }
                    if f == field && ty == "code-anchor"),
                "expected UnknownFieldType for {field}/code-anchor, got {err:?}",
            );
        }
    }

    /// (M13) The per-field-type `check:` predicate selector. The pack-declared
    /// type carries its check explicitly (`code-anchor → symbol-exists`); a bare
    /// field of that type **inherits** the type's check (resolved to
    /// `Some("symbol-exists")`), while a field declaring an explicit `check:`
    /// **overrides** it for that one field — both reachable through the model so
    /// `target_surface.rs` can read `field.check ?? field_type.check`. See
    /// `design/architecture-documentation.md` → The per-field-type predicate
    /// selector.
    #[test]
    fn pack_field_type_resolves_its_check_and_a_field_check_overrides_it() {
        let yaml = b"\
type: adr
sections:
  - id: status
    header: true
    fields:
      - { id: bare, type: code-anchor }
      - { id: overridden, type: code-anchor, check: criterion-maps-to-test }
";
        let schema = load_schema_with_types(yaml, &dev_pack_field_types())
            .expect("declared pack type loads");
        let SectionBody::Simple { fields, .. } = &schema.sections[0].body else {
            panic!("status is a simple header section");
        };

        // A bare `code-anchor` field inherits the declared type's check, resolved
        // onto the `PackFieldType` — `Some("symbol-exists")` — and carries no
        // field-level override.
        let bare = fields.iter().find(|f| f.id == "bare").unwrap();
        let FieldType::Pack(pack) = &bare.ty else {
            panic!("bare resolves to a pack-declared type, got {:?}", bare.ty);
        };
        assert_eq!(pack.check.as_deref(), Some("symbol-exists"));
        assert_eq!(bare.check, None);

        // The field declaring an explicit `check:` carries that field-level value
        // in the model — the per-field override the resolver reads first. The
        // type's own resolved check is untouched.
        let overridden = fields.iter().find(|f| f.id == "overridden").unwrap();
        assert_eq!(overridden.check.as_deref(), Some("criterion-maps-to-test"));
        let FieldType::Pack(pack) = &overridden.ty else {
            panic!("overridden resolves to a pack-declared type");
        };
        assert_eq!(pack.check.as_deref(), Some("symbol-exists"));
    }

    /// (M13, the no-default pin) A `PackTypeDecl` YAML **missing** `check` fails
    /// to load: `check` is a required field with no implicit engine default, so a
    /// pack declaring a type must spell its predicate — picking the default wrong
    /// would silently strip `spec.criteria`'s `criterion-maps-to-test` (the
    /// VISION headline check). `deny_unknown_fields` + a required `check` together
    /// enforce it. See `design/architecture-documentation.md` (no implicit engine
    /// default).
    #[test]
    fn pack_type_decl_missing_check_fails_to_load() {
        let yaml = b"- { name: code-anchor, adjudicator: doc-code }\n";
        let err =
            serde_yaml_ng::from_str::<Vec<PackTypeDecl>>(std::str::from_utf8(yaml).expect("utf8"))
                .expect_err("a PackTypeDecl missing `check` must fail to load");
        assert!(
            err.to_string().contains("check"),
            "the load error must name the missing `check` field; got {err}",
        );
    }

    /// (M16 inc-2 T1) The `singleton: true` schema flag serde-roundtrips: a
    /// fixture schema declaring it parses `singleton == true` and **re-serializes**
    /// the flag (it survives a load→serialize→reload cycle). A schema that **omits**
    /// it defaults to `false` and, mirroring the `header: bool` skip-on-false
    /// discipline, serializes **nothing** — the additive-field guard that leaves
    /// every existing non-singleton golden byte-unchanged. See
    /// `design/methodology-docs.md` → The four doctypes (review finding B-2).
    #[test]
    fn singleton_flag_serde_roundtrips_and_is_skipped_when_false() {
        let with_singleton = b"\
type: roadmap
singleton: true
sections: []
";
        let schema = load_schema(with_singleton).expect("singleton schema loads");
        assert!(schema.singleton, "the singleton flag parses as true");

        // Re-serializes the flag, and a reload preserves it (roundtrip).
        let json = serde_json::to_string(&schema).expect("serializes");
        assert!(
            json.contains("\"singleton\":true"),
            "the singleton flag re-serializes; got {json}",
        );
        let reloaded: Schema = serde_json::from_str(&json).expect("reloads");
        assert_eq!(reloaded, schema, "singleton survives a serde roundtrip");

        // A schema omitting it defaults to false and serializes nothing for it —
        // the skip-on-false additive guard (no existing golden gains a key).
        let without = b"\
type: commit
sections: []
";
        let plain = load_schema(without).expect("non-singleton schema loads");
        assert!(!plain.singleton, "an omitted flag defaults to false");
        let plain_json = serde_json::to_string(&plain).expect("serializes");
        assert!(
            !plain_json.contains("singleton"),
            "a false singleton flag serializes nothing (skip-on-false); got {plain_json}",
        );
    }

    /// Authored-prose metadata (M11): a schema carrying top-level
    /// `description:`/`usage:` parses them to `Some(..)` (they are siblings of
    /// `type`/`location`/`id-from`, NOT inside `sections`), and a schema that
    /// omits them leaves both `None` with no error — even though `Schema` is
    /// `deny_unknown_fields`, the struct field admits the keys. This is the
    /// substrate `describe` projects.
    #[test]
    fn description_and_usage_load_as_top_level_fields() {
        let with_prose = b"\
type: adr
description: A dated architectural decision record.
usage: Reach for it when a choice is worth preserving with its rationale.
sections: []
";
        let schema = load_schema(with_prose).expect("schema with prose loads");
        assert_eq!(
            schema.description.as_deref(),
            Some("A dated architectural decision record.")
        );
        assert_eq!(
            schema.usage.as_deref(),
            Some("Reach for it when a choice is worth preserving with its rationale.")
        );

        let without_prose = b"\
type: commit
sections: []
";
        let schema = load_schema(without_prose).expect("schema without prose loads");
        assert_eq!(schema.description, None);
        assert_eq!(schema.usage, None);
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

#[cfg(test)]
mod arch_doc {
    //! The shipped `arch-doc` doctype (M13 Increment 4 / T1): the schema loads with
    //! the `meta` header `cites → adr` (0..*), an `overview` slot, and a repeatable
    //! `components` block whose bare `implemented-by` resolves to a `code-anchor`
    //! inheriting the type's `symbol-exists` check (no field override). The shipped
    //! instance round-trips byte-stably — the #1-risk round-trip over this doctype's
    //! highest-risk shape (a header list-ref + repeatable items each carrying a
    //! per-item code-anchor field). See `design/architecture-documentation.md` → The
    //! schema.

    use super::*;

    const ARCH_DOC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/arch-doc.yaml");

    fn arch_doc_schema() -> Schema {
        load_schema_with_types(ARCH_DOC_YAML, &dev_pack_field_types()).expect("arch-doc.yaml loads")
    }

    /// The schema's identity and the `meta`/`overview`/`components` shape are
    /// reachable through the model, and the repeatable `implemented-by` resolves to a
    /// `FieldType::Pack` whose **inherited** check is `Some("symbol-exists")` (no
    /// per-field override — `field.check == None`). This is what `target_surface.rs`
    /// reads to select the `doc-code` predicate for the anchor.
    #[test]
    fn schema_loads_with_cites_ref_and_a_symbol_exists_anchor() {
        let schema = arch_doc_schema();
        assert_eq!(schema.ty, "arch-doc");
        assert_eq!(schema.location.as_deref(), Some("architecture/"));
        assert_eq!(schema.id_from.as_deref(), Some("title"));

        // `meta` is the header section carrying the n→n `cites → adr` ref.
        let meta = &schema.sections[0];
        assert_eq!(meta.id, "meta");
        assert!(meta.header);
        let SectionBody::Simple { slot, fields } = &meta.body else {
            panic!("meta is a simple header section");
        };
        assert!(slot.is_none());
        let cites = fields.iter().find(|f| f.id == "cites").unwrap();
        assert_eq!(cites.ty, FieldType::Ref);
        assert_eq!(cites.to.as_deref(), Some("adr"));
        assert_eq!(cites.card.as_deref(), Some("0..*"));
        assert_eq!(cites.inverse.as_deref(), Some("cited-by"));

        // `components` is a repeatable section; its bare `implemented-by` resolves to
        // a `code-anchor` inheriting the type's `symbol-exists` check, with NO
        // field-level override.
        let components = &schema.sections[2];
        assert_eq!(components.id, "components");
        let SectionBody::Repeatable { repeatable } = &components.body else {
            panic!("components is repeatable");
        };
        assert_eq!(repeatable.id_from, "title");
        let Leaf::Field(anchor) = repeatable
            .block
            .iter()
            .find(|l| matches!(l, Leaf::Field(f) if f.id == "implemented-by"))
            .expect("implemented-by is a field leaf")
        else {
            unreachable!();
        };
        let FieldType::Pack(pack) = &anchor.ty else {
            panic!(
                "implemented-by resolves to a pack-declared type, got {:?}",
                anchor.ty
            );
        };
        assert_eq!(pack.name, "code-anchor");
        assert_eq!(pack.adjudicator.as_deref(), Some("doc-code"));
        assert_eq!(
            pack.check.as_deref(),
            Some("symbol-exists"),
            "the bare anchor inherits the type's symbol-exists check",
        );
        assert_eq!(
            anchor.check, None,
            "no per-field check override on the bare anchor",
        );
    }

    /// A canonical `arch-doc` instance over the shipped schema: the `meta` header
    /// carrying `cites` over **two** adr targets, the `# H1` title, the `overview`
    /// slot, and **two** `components` items each with a frozen `{#id}` anchor and an
    /// `implemented-by` code-anchor. The exact frozen byte form the canonical writer
    /// emits.
    fn canonical_arch_doc_source() -> &'static str {
        "\
---
cites: [adr:single-node-cache, adr:distributed-cache]
---

# The edge index

## Overview

The edge index is a rebuildable map of forward cross-reference edges.

## Components

### The rebuild  {#rebuild}

Walks the committed docs and emits one forward edge per present ref field.

<!-- fields -->
- implemented-by: crates/engine/src/index.rs#rebuild_committed

### The overlay  {#overlay}

Layers the active task's working-area edges over the committed index in memory.

<!-- fields -->
- implemented-by: crates/engine/src/index.rs#overlay_working
"
    }

    /// The #1-risk round-trip over `arch-doc`: `render(parse(src)) == src` on the
    /// canonical fixture (a header `cites` list of two adr targets + two `components`
    /// each carrying an `implemented-by` anchor). The two frozen item anchors survive
    /// the parse in order; the writer reproduces the source byte-for-byte.
    #[test]
    fn arch_doc_render_parse_render_equals_source() {
        let schema = arch_doc_schema();
        let src = canonical_arch_doc_source();

        let instance =
            crate::write::instance_from_source(&schema, src).expect("rendered arch-doc parses");

        // The two cites targets survive on the header, in order.
        let meta = instance
            .sections
            .iter()
            .find(|s| s.id == "meta")
            .expect("meta section present");
        let cites = meta.fields.iter().find(|f| f.key == "cites").unwrap();
        assert_eq!(
            cites.value,
            crate::field_block::Value::List(vec![
                "adr:single-node-cache".to_string(),
                "adr:distributed-cache".to_string(),
            ]),
            "the cites list parses, in order",
        );

        // The two frozen component anchors survive the parse, in order.
        let components = instance
            .sections
            .iter()
            .find(|s| s.id == "components")
            .expect("components section present");
        let ids: Vec<&str> = components.items.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(ids, ["rebuild", "overlay"], "frozen anchors preserved");

        let rendered = crate::write::render(&schema, &instance);
        assert_eq!(
            rendered, src,
            "render(parse(src)) must equal the source bytes",
        );
    }

    /// `write → parse → write` is byte-identical (idempotent on canonical content)
    /// over the shipped `arch-doc` schema — the repeatable-item + header-list path
    /// included.
    #[test]
    fn arch_doc_write_parse_write_is_byte_identical() {
        let schema = arch_doc_schema();
        let src = canonical_arch_doc_source();

        let first = crate::write::render(
            &schema,
            &crate::write::instance_from_source(&schema, src).expect("parses"),
        );
        let second = crate::write::render(
            &schema,
            &crate::write::instance_from_source(&schema, &first).expect("parses"),
        );
        assert_eq!(
            first, second,
            "write → parse → write must be byte-identical"
        );
    }
}
