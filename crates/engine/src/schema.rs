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

    /// Optional doctype-level **display text for the H1**, orthogonal to
    /// `id-from` (display-only, never an id-source — the name avoids colliding
    /// with the `id-from: title` token). When present, it overrides a singleton's
    /// H1 display text (e.g. `vision` declares `display-title: Vision`, so the
    /// managed doc reads `# Vision`, not `# vision`); absent leaves today's
    /// behavior (H1 = id-source / slug). Skip-on-absent (mirrors `location` /
    /// `id-from`): a schema omitting it serializes **nothing**, so no frozen
    /// doctype's `schema-hash` changes — the freeze-safe additive-key pattern
    /// (`design/design-altitude-doctypes.md` → §4 The vision surface; §7 arm 5).
    #[serde(
        rename = "display-title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_title: Option<String>,

    /// Optional doctype-level **literal-file placement**: the managed doc lives at
    /// one exact `placement.file` repo-root-relative path (case-preserved, bypassing
    /// the lowercase slug), *not* at `<docs-root>/<location>/<slug>.md`. A placement
    /// doctype therefore sets **no `location`** (`location: None`, like transient
    /// `commit`) and carries its fixed slug (= type id) independently, so a root
    /// `VISION.md` / `CHANGELOG.md` or a direct `docs/roadmap.md` is reachable exactly
    /// as written and `docs-root` never applies (the home is composition-invariant).
    /// Absent → today's folder behavior. Skip-on-absent (mirrors `location` /
    /// `id-from` / `display-title`): a schema omitting it serializes
    /// **nothing**, so no frozen doctype's `schema-hash` changes — the freeze-safe
    /// additive-key pattern (`design/storage.md` → Placement — direct-file and
    /// root-located homes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,

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

/// A doctype's **literal-file placement**: the one exact repo-root-relative path
/// its single managed instance lives at.
///
/// The `file` path is a **literal** (case-preserved, bypassing the lowercase
/// `[a-z0-9-]` slug), so `VISION.md` / `CHANGELOG.md` / `docs/roadmap.md` are each
/// reachable exactly as written — the `docs/` prefix (or its absence) is encoded
/// in the literal, so there is **no `root:` flag** and `docs-root` never applies.
/// A doctype carrying `placement` sets no `location` (`design/storage.md` →
/// Placement — direct-file and root-located homes).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    /// The repo-root-relative literal path this doctype's one instance lives at.
    pub file: String,
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

/// A leaf inside a repeatable item block: a typed field, an LLM-filled slot, or
/// a **nested repeatable** (a repeatable-inside-a-repeatable — the M22 lift).
///
/// Distinguished on disk by which key is present (`slot:` vs `repeatable:` vs
/// the field's flat `{id, type, …}` form).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Leaf {
    /// An LLM-authored prose slot (south of the determinism boundary).
    Slot {
        /// The leaf id (and on-disk sub-label).
        id: String,
        slot: Slot,
    },
    /// A nested repeatable: an ID'd sub-list of item blocks within a parent
    /// item's block (e.g. a release's `changes` change-groups). Its items
    /// render one heading level deeper than the parent's, bounded by the H6
    /// cap (`design/changelog.md` → engine work #1; `design/structural-grammar.md`
    /// → Repetition). The leaf `id` is the section's stable id; the inner
    /// `repeatable` carries the sub-list's `id-from` + block.
    Repeatable {
        /// The leaf id (and the source of the nested section's heading).
        id: String,
        /// The nested item-template: its id-source field plus the block's
        /// leaves (which may themselves nest, up to the depth cap).
        repeatable: Repeatable,
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

    /// `true` for an **optional** slot: its absence does not block finalize
    /// (`required-slot-present` is skipped), while a required slot still blocks.
    /// Skip-on-false (mirrors `singleton`): a schema omitting it leaves every
    /// existing golden byte-unchanged. See `design/changelog.md` → engine work #3.
    #[serde(default, skip_serializing_if = "is_false")]
    pub optional: bool,
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

    /// `true` for an **optional** field: its absence does not block finalize
    /// (`required-field-present` is skipped), while a required field still blocks.
    /// Skip-on-false (mirrors `singleton`): a schema omitting it leaves every
    /// existing golden byte-unchanged. See `design/changelog.md` → engine work #3.
    #[serde(default, skip_serializing_if = "is_false")]
    pub optional: bool,

    /// `true` for a `code-anchor` whose sibling repeatable-item **title must
    /// contain the symbol it names**. Arch-doc components are titled after their
    /// implementing symbol, so a title left naming a renamed/removed symbol is
    /// doc↔code drift the `symbol-exists` anchor check alone misses (the
    /// long-horizon study's Opus prose blind spot: the agent fixes the anchor to
    /// clear the gate and leaves the heading stale). Pack-declared so the engine
    /// stays generic — spec criteria, whose titles are prose, do not opt in.
    /// Skip-on-false keeps every existing golden byte-unchanged.
    #[serde(
        rename = "title-names-symbol",
        default,
        skip_serializing_if = "is_false"
    )]
    pub title_names_symbol: bool,
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
    /// A repo-relative path under an owner-assigned artifact home. Recognition
    /// only at this layer (the field parses + is exempt from author-required /
    /// value-conformant, mirroring [`FieldType::Pack`]); its real adjudication —
    /// path-safety + durable presence — is the intrinsic finalize-time #5
    /// owner-artifact gate (`design/methodology-docs.md` → The engine work, item
    /// 3). Engine-**native**, not a `Pack` type, because the gate keying on it is
    /// intrinsic (not a tunable pack probe like `code-anchor` → `doc-code`).
    OwnedLocation,
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
        ("owned-location", FieldType::OwnedLocation),
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
            FieldType::OwnedLocation => "owned-location",
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

    /// An `include:` directive named a fragment **absent** from the schema's
    /// top-level `fragments:` map (or whose value is not a block sequence). The
    /// schema-fragment de-dup mechanism rejects a dangling include loudly here,
    /// never silently dropping the block. See `design/document-type-schema.md`
    /// → On-disk definition format (a genuinely-shared section pulled in with
    /// `include`) and `design/corpus-migration.md` → schema-fragment `include`.
    #[error("`include` names undefined fragment `{name}`")]
    UnknownFragment {
        /// The fragment name the include directive named.
        name: String,
    },

    /// An `include:` directive forms a **cycle** — a fragment that includes
    /// itself directly (`a → a`) or transitively (`a → b → a`). Fragment
    /// expansion runs *before* deserialization, so a cycle would recurse without
    /// bound (the `MAX_NESTING_DEPTH` cap is checked on the deserialized model,
    /// which a non-terminating expansion never reaches). The expansion path is
    /// guarded so the re-entered fragment is rejected loudly here, naming it,
    /// rather than overflowing the stack. See `design/corpus-migration.md` →
    /// schema-fragment `include`.
    #[error("`include` forms a cycle through fragment `{name}`")]
    CyclicFragment {
        /// The fragment name re-entered along the current expansion path.
        name: String,
    },

    /// A repeatable nests deeper than the heading-depth cap. Items render at
    /// heading level `2 + nesting-depth`; the cap is **H6 → 4 nesting levels**
    /// ([`MAX_NESTING_DEPTH`]). A 5th level would render at `H7`, which Markdown
    /// has no heading for — so it is rejected loudly at load (a **documented
    /// cap, not silent truncation**), naming the offending depth.
    #[error(
        "repeatable nests to depth {depth}, deeper than the H6 cap (max {} levels)",
        MAX_NESTING_DEPTH
    )]
    NestingTooDeep {
        /// The (1-based) nesting depth that breached the cap.
        depth: usize,
    },

    /// A single block declares **more than one** nested repeatable. The parse
    /// model carries one undifferentiated nested item list per item
    /// (`ParsedItem::items` — heading depth alone cannot attribute an `H(n+1)`
    /// item to one of two sibling nested sections), so a second nested group in
    /// the same block is unrepresentable: its items would be silently unioned
    /// with the first group's under **every** declared block id on the read
    /// surfaces. Rejected loudly at load, naming the block and both nested ids
    /// (the freeze-assert sibling pattern). Nesting stays **depth**-general
    /// (the H6 cap); breadth per block is capped at 1
    /// (`design/structural-grammar.md` → Repetition).
    #[error(
        "block `{block}` declares more than one nested repeatable (`{first}`, `{second}`): \
         at most one nested repeatable per block is supported"
    )]
    MultipleNestedRepeatables {
        /// The id of the block (section or nested-repeatable leaf) declaring both.
        block: String,
        /// The first declared nested repeatable's id.
        first: String,
        /// The second declared nested repeatable's id.
        second: String,
    },

    /// A repeatable block declares the **reserved** item key `id` — as a block
    /// leaf, or as the block's `id-from`. The pinned `jigc doc show --format
    /// json` item object keys the item's minted, frozen id under `id` (the
    /// handle every address into the item takes —
    /// `design/doc-read-surface.md` → The item `id` closes the json contract),
    /// so a declared `id` leaf would silently collide with it and hand a driver
    /// a value it cannot address the item back with. Rejected loudly at
    /// pack-load, naming the block and the declaring site (the freeze-assert
    /// sibling pattern).
    #[error(
        "block `{block}` declares the reserved item key `id` (as its `{site}`): `id` is \
         reserved for the item's minted id on the pinned `jigc doc show --format json` \
         item object"
    )]
    ReservedItemIdKey {
        /// The id of the block (section or nested-repeatable leaf) declaring it.
        block: String,
        /// The declaring site — `leaf` (a block leaf) or `id-from`.
        site: String,
    },
}

/// The maximum repeatable nesting depth: a section is `##`, so a repeatable at
/// nesting-depth `d` renders its items at heading level `2 + d`. The deepest
/// Markdown heading is `H6`, so `d` caps at **4** (top-level items `###` = depth
/// 1, the changelog's nested change-groups `####` = depth 2, …, `######` = depth
/// 4). A documented cap, enforced at load (`design/changelog.md` → engine work
/// #1; `design/structural-grammar.md` → Repetition).
pub const MAX_NESTING_DEPTH: usize = 4;

/// The item key **reserved** on every repeatable block: the pinned `jigc doc show
/// --format json` item object keys the item's minted, frozen id under `id` — the
/// handle every address into the item takes (`design/doc-read-surface.md` → The
/// item `id` closes the json contract under its own address grammar). A block
/// declaring an `id` leaf (or naming `id` as its `id-from`) would collide with
/// that key, so it is a typed [`SchemaError::ReservedItemIdKey`] at load.
pub const RESERVED_ITEM_ID_KEY: &str = "id";

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

/// The id of the engine-declared per-doc **schema-version stamp** field — the
/// front-matter leaf recording which schema version an instance was authored
/// against (`design/corpus-migration.md` → The schema-version stamp). The field is
/// **engine-declared** (shape only) and **pack-loader-injected** uniformly into
/// every persisted frozen doctype, so the N doctype YAMLs are not hand-edited; its
/// value is supplied at create time by the CLI's "current active schema version"
/// deriver (the `status`/`date` model — in-schema field, engine/CLI-set value).
pub const SCHEMA_VERSION_FIELD: &str = "schema-version";

/// The `set:` deriver marker naming the [`SCHEMA_VERSION_FIELD`]'s value source —
/// the CLI's "current active schema version" deriver fills it at create, the version
/// analog of the `set: on-create` clock deriver for dates. A field carrying it is
/// CLI-derived (never author-required) just like a `set: on-create` date.
pub const SCHEMA_VERSION_SET: &str = "schema-version";

/// The `set:` deriver naming a **milestone transition** as a field's value source —
/// the milestone verbs stamp `base` / `status` / `intent` at create/join
/// (`crates/engine/src/milestone.rs`; `design/team-ready-state.md`). A field carrying
/// it is a machine-maintained absolute the author never overwrites, exactly like the
/// [`SCHEMA_VERSION_SET`] stamp — the second member of the "the CLI is the sole author"
/// `set:` kind (`design/write-commands.md` → The set-field machine-maintained guard).
pub const SET_ON_TRANSITION: &str = "on-transition";

/// Whether `field` is a **machine-maintained absolute**: its value is CLI-derived and
/// the author may **never** overwrite it through a `jigc doc` write — the `set:`-kind
/// split at the heart of the set-field machine-maintained guard. **True** for the freeze
/// stamp ([`SCHEMA_VERSION_SET`]) and a milestone transition ([`SET_ON_TRANSITION`]);
/// **false** for `set: on-create`, which the CLI merely *defaults* at mint and the
/// changelog-migration historical-date path legitimately overwrites
/// (`design/auto-migration.md` → no false history), and false for any non-`set:` field.
///
/// This is the *single authority* the write-path guard (`crates/cli/src/doc.rs` →
/// `apply_field_target`) and the `doc schema` settability projection
/// (`design/doc-read-surface.md` → the settability states) share, so *advertised-set*
/// and *accepted-set* cannot drift — neither re-implements the walk. It is the strict
/// **write** counterpart of [`crate::validate::is_author_required`]'s `set:` arm and the
/// `--unset` eligibility guard: *cannot be unset* (every `set:`-derived field) is a
/// strictly wider set than *may not be overwritten* (the absolutes only).
pub fn is_machine_maintained_absolute(field: &Field) -> bool {
    matches!(
        field.set.as_deref(),
        Some(SCHEMA_VERSION_SET | SET_ON_TRANSITION)
    )
}

/// The id of the header section [`inject_schema_version_stamp`] creates for a
/// header-less doctype (`prd`/`changelog`) so the stamp has a front-matter home —
/// matching the `meta` header the other persisted doctypes already carry.
const STAMP_HEADER_SECTION: &str = "meta";

/// The engine-declared schema-version stamp [`Field`] — its **shape only**
/// (`type: int, set: schema-version`); the version VALUE is per-instance, supplied
/// by the CLI deriver at create. Version-independent by design: the stamp's shape is
/// identical across every schema version, so the doctype's `schema_hash` stays a
/// pure fingerprint of the doctype's *own* shape, and a v→v+1 bump changes the hash
/// only via the real shape change (never via the stamp).
pub fn schema_version_stamp_field() -> Field {
    Field {
        id: SCHEMA_VERSION_FIELD.to_owned(),
        ty: FieldType::Int,
        of: None,
        default: None,
        set: Some(SCHEMA_VERSION_SET.to_owned()),
        to: None,
        card: None,
        inverse: None,
        inverse_card: None,
        check: None,
        optional: false,
        title_names_symbol: false,
    }
}

/// Inject the engine-declared [`schema_version_stamp_field`] into `schema`'s header
/// section, **uniformly** — appended to the existing header's `fields` for a doctype
/// that already declares one (`adr`/`spec`/`arch-doc`), or carried by a freshly
/// created header section inserted **first** for a header-less doctype
/// (`prd`/`changelog`, which thereby gain a `---` block; the header must be the
/// document's first section — `parse.rs` → header-not-first). Idempotent: a schema
/// that already carries a [`SCHEMA_VERSION_FIELD`] anywhere is left unchanged.
///
/// The caller (the CLI pack loader) decides *which* schemas are eligible (the frozen
/// persisted set); this helper is the engine-declared placement, so the doctype
/// YAMLs are never hand-edited (`design/corpus-migration.md` → The schema-version
/// stamp: the engine injects the declaration uniformly).
pub fn inject_schema_version_stamp(schema: &mut Schema) {
    let already = schema.sections.iter().any(|s| match &s.body {
        SectionBody::Simple { fields, .. } => fields.iter().any(|f| f.id == SCHEMA_VERSION_FIELD),
        SectionBody::Repeatable { .. } => false,
    });
    if already {
        return;
    }
    if let Some(header) = schema.sections.iter_mut().find(|s| s.header)
        && let SectionBody::Simple { fields, .. } = &mut header.body
    {
        fields.push(schema_version_stamp_field());
        return;
    }
    // No header section (a header-less doctype): create one, carrying the stamp, and
    // insert it first so the front-matter `---` block renders at the top.
    schema.sections.insert(
        0,
        Section {
            id: STAMP_HEADER_SECTION.to_owned(),
            header: true,
            body: SectionBody::Simple {
                slot: None,
                fields: vec![schema_version_stamp_field()],
            },
        },
    );
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
    // Pre-deserialize pass: expand any schema-fragment `include` directives,
    // then deserialize the expanded value into the model (a schema with no
    // `fragments:` map passes through structurally unchanged).
    let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(text)?;
    let expanded = expand_fragment_includes(raw)?;
    let mut schema: Schema = serde_yaml_ng::from_value(expanded)?;
    for section in &mut schema.sections {
        match &mut section.body {
            SectionBody::Simple { fields, .. } => {
                for field in fields {
                    resolve_field_type(field, pack_types)?;
                }
            }
            // A repeatable section's items render at `###` (nesting-depth 1);
            // its block walks recursively, resolving nested field types and
            // enforcing the H6 depth cap.
            SectionBody::Repeatable { repeatable } => {
                let owner = section.id.clone();
                resolve_block(repeatable, &owner, 1, pack_types)?;
            }
        }
    }
    Ok(schema)
}

/// Recursively resolve every field type in a repeatable block and enforce the
/// two structural caps: the [`MAX_NESTING_DEPTH`] **depth** cap and the
/// one-nested-repeatable-per-block **breadth** cap
/// ([`SchemaError::MultipleNestedRepeatables`]). `owner` is the id of the
/// block's declaring unit (the section, or the nested-repeatable leaf), used to
/// name the offender. `depth` is the (1-based) nesting depth of the items this
/// block templates — top-level repeatable items are depth 1, a nested
/// repeatable's items depth 2, and so on. A block whose own depth exceeds the
/// cap is a typed [`SchemaError::NestingTooDeep`].
fn resolve_block(
    repeatable: &mut Repeatable,
    owner: &str,
    depth: usize,
    pack_types: &[PackTypeDecl],
) -> Result<(), SchemaError> {
    if depth > MAX_NESTING_DEPTH {
        return Err(SchemaError::NestingTooDeep { depth });
    }
    // Breadth guard: at most ONE nested repeatable per block. The parsed item
    // carries a single undifferentiated nested list, so a second sibling group
    // would be silently unioned with the first — reject it loudly at load.
    let mut nested_ids = repeatable.block.iter().filter_map(|leaf| match leaf {
        Leaf::Repeatable { id, .. } => Some(id.as_str()),
        _ => None,
    });
    if let (Some(first), Some(second)) = (nested_ids.next(), nested_ids.next()) {
        return Err(SchemaError::MultipleNestedRepeatables {
            block: owner.to_owned(),
            first: first.to_owned(),
            second: second.to_owned(),
        });
    }
    // Reserved-key guard: `id` is the pinned json item object's key for the item's
    // minted id, so no block may declare it — as a leaf or as its `id-from`.
    if repeatable.id_from == RESERVED_ITEM_ID_KEY {
        return Err(SchemaError::ReservedItemIdKey {
            block: owner.to_owned(),
            site: "id-from".to_owned(),
        });
    }
    if repeatable
        .block
        .iter()
        .any(|leaf| leaf_id(leaf) == RESERVED_ITEM_ID_KEY)
    {
        return Err(SchemaError::ReservedItemIdKey {
            block: owner.to_owned(),
            site: "leaf".to_owned(),
        });
    }
    for leaf in &mut repeatable.block {
        match leaf {
            Leaf::Field(field) => resolve_field_type(field, pack_types)?,
            Leaf::Repeatable { id, repeatable } => {
                let owner = id.clone();
                resolve_block(repeatable, &owner, depth + 1, pack_types)?;
            }
            Leaf::Slot { .. } => {}
        }
    }
    Ok(())
}

/// One block leaf's declared id, whichever kind it is.
fn leaf_id(leaf: &Leaf) -> &str {
    match leaf {
        Leaf::Field(field) => &field.id,
        Leaf::Slot { id, .. } | Leaf::Repeatable { id, .. } => id,
    }
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

/// Pre-deserialize expansion of the schema-definition `include` directive.
///
/// A top-level `fragments:` map names genuinely-shared blocks; wherever a
/// `block:` sequence carries an `{ include: <name> }` item, the named
/// fragment's items are spliced in place — de-duplicating a section repeated
/// across sites (the `changelog` change-group block at both the staging area
/// and each cut release; `design/document-type-schema.md` → On-disk definition
/// format, `design/corpus-migration.md` → schema-fragment `include`). A schema
/// with **no** `fragments:` map passes through structurally unchanged, so no
/// existing doctype's loaded model shifts. An include naming an undefined
/// fragment is a typed [`SchemaError::UnknownFragment`], never a silent drop.
fn expand_fragment_includes(
    value: serde_yaml_ng::Value,
) -> Result<serde_yaml_ng::Value, SchemaError> {
    use serde_yaml_ng::{Mapping, Value};
    let Value::Mapping(mut map) = value else {
        return Ok(value);
    };
    // Lift the top-level `fragments:` map out before deserialization (the model
    // is `deny_unknown_fields`); absent or non-mapping leaves no fragments.
    let fragments = match map.remove("fragments") {
        Some(Value::Mapping(f)) => f,
        _ => Mapping::new(),
    };
    splice_includes(Value::Mapping(map), &fragments, &[])
}

/// Recursively rewrite `value`, splicing each `{ include: <name> }` sequence
/// item with the named fragment's items (looked up in `fragments`).
///
/// `path` is the stack of fragment names currently being expanded along this
/// branch. A fragment whose expansion re-enters a name already on `path` is a
/// **cycle** (`a → a` or `a → b → a`): since this pass runs *before*
/// deserialization, an unguarded cycle would recurse without bound (the
/// downstream [`MAX_NESTING_DEPTH`] cap is checked on the deserialized model,
/// which a non-terminating expansion never reaches). The guard rejects it as a
/// typed [`SchemaError::CyclicFragment`] instead of overflowing the stack.
fn splice_includes(
    value: serde_yaml_ng::Value,
    fragments: &serde_yaml_ng::Mapping,
    path: &[&str],
) -> Result<serde_yaml_ng::Value, SchemaError> {
    use serde_yaml_ng::{Mapping, Value};
    match value {
        Value::Mapping(m) => {
            let mut out = Mapping::new();
            for (k, v) in m {
                out.insert(k, splice_includes(v, fragments, path)?);
            }
            Ok(Value::Mapping(out))
        }
        Value::Sequence(seq) => {
            let mut out = Vec::with_capacity(seq.len());
            for item in seq {
                match include_target(&item) {
                    Some(name) => {
                        if path.contains(&name) {
                            return Err(SchemaError::CyclicFragment {
                                name: name.to_owned(),
                            });
                        }
                        let frag = fragments
                            .get(name)
                            .and_then(Value::as_sequence)
                            .ok_or_else(|| SchemaError::UnknownFragment {
                                name: name.to_owned(),
                            })?;
                        // Expand the fragment's items too, so a fragment may
                        // itself include another — re-running include detection
                        // over them with `name` pushed onto the path, so a cycle
                        // is caught above rather than recursing unbounded.
                        let mut child_path: Vec<&str> = path.to_vec();
                        child_path.push(name);
                        let expanded =
                            splice_includes(Value::Sequence(frag.clone()), fragments, &child_path)?;
                        if let Value::Sequence(items) = expanded {
                            out.extend(items);
                        }
                    }
                    None => out.push(splice_includes(item, fragments, path)?),
                }
            }
            Ok(Value::Sequence(out))
        }
        other => Ok(other),
    }
}

/// The fragment name of an `{ include: <name> }` directive item, or `None` for
/// any other sequence item (a leaf field/slot/nested-repeatable).
fn include_target(item: &serde_yaml_ng::Value) -> Option<&str> {
    item.as_mapping()
        .and_then(|m| m.get("include"))
        .and_then(serde_yaml_ng::Value::as_str)
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
    const CHANGELOG_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/changelog.yaml");

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
        assert_eq!(supersedes.card.as_deref(), Some("0..*"));
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
    /// `meta` header carries `derived-from` as a `ref → prd` (card `0..1`,
    /// `inverse: has-specs`, `inverse-card: "1..*"`), the `criteria` repeatable block
    /// carries `statement` as a `Leaf::Slot`, the spec carries NO document-level
    /// `title` field (the slug derives from the H1, like `adr`), and the cut fields
    /// (`status`/`date`/`decided-by`) are genuinely absent.
    #[test]
    fn spec_criteria_block_carries_statement_as_a_slot() {
        let schema =
            load_schema_with_types(SPEC_YAML, &dev_pack_field_types()).expect("spec.yaml loads");
        assert_eq!(schema.ty, "spec");
        assert_eq!(schema.location.as_deref(), Some("specs/"));
        assert_eq!(schema.id_from.as_deref(), Some("title"));

        // `meta` is the header section — it carries the `derived-from → prd` edge of
        // the frozen doctype graph (modeled spec-side; the PRD's `has-specs` inverse
        // is derived, never stored), `card: "0..1"` the deadlock guard.
        let meta = &schema.sections[0];
        assert_eq!(meta.id, "meta");
        assert!(meta.header);
        let SectionBody::Simple { fields, .. } = &meta.body else {
            panic!("meta is a header field section");
        };
        let derived_from = fields
            .iter()
            .find(|f| f.id == "derived-from")
            .expect("meta carries derived-from");
        assert_eq!(derived_from.ty, FieldType::Ref);
        assert_eq!(derived_from.to.as_deref(), Some("prd"));
        assert_eq!(derived_from.card.as_deref(), Some("0..1"));
        assert_eq!(derived_from.inverse.as_deref(), Some("has-specs"));
        assert_eq!(derived_from.inverse_card.as_deref(), Some("1..*"));

        // `goal` follows the header — a prose slot, no fields.
        let goal = &schema.sections[1];
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
        let criteria = &schema.sections[3];
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

    /// (M37 inc-1 T1) The doctype-level `display-title:` knob serde-roundtrips: a
    /// fixture schema declaring `display-title: Vision` parses
    /// `display_title == Some("Vision")`, **re-serializes** the key, and survives a
    /// load→serialize→reload cycle. A schema that **omits** it serializes to JSON
    /// with **no** `display-title` key — so its `schema_hash` (`blake3` over that
    /// JSON) is byte-identical to the pre-field value, the freeze-safe additive-key
    /// proof (`design/design-altitude-doctypes.md` → §4; §7 arm 5): the field's mere
    /// existence on the struct changes no frozen doctype's hash. (b) proven, not
    /// assumed — the real frozen six are covered by the pack-load freeze regressions.
    #[test]
    fn display_title_serde_roundtrips_and_is_skipped_when_absent() {
        let with_display_title = b"\
type: vision
display-title: Vision
sections: []
";
        let schema = load_schema(with_display_title).expect("display-title schema loads");
        assert_eq!(
            schema.display_title.as_deref(),
            Some("Vision"),
            "the display-title knob parses its string",
        );

        // Re-serializes the key, and a reload preserves it (roundtrip).
        let json = serde_json::to_string(&schema).expect("serializes");
        assert!(
            json.contains("\"display-title\":\"Vision\""),
            "the display-title knob re-serializes under its on-disk name; got {json}",
        );
        let reloaded: Schema = serde_json::from_str(&json).expect("reloads");
        assert_eq!(reloaded, schema, "display-title survives a serde roundtrip");

        // A schema omitting it defaults to None and serializes nothing for it — the
        // skip-on-absent additive guard. No `display-title` in the JSON means the
        // bytes `schema_hash` digests are identical to the pre-field value, so no
        // frozen doctype's hash moves (the §7-arm-5 freeze-safety claim, proven).
        let without = b"\
type: commit
sections: []
";
        let plain = load_schema(without).expect("no-display-title schema loads");
        assert!(
            plain.display_title.is_none(),
            "an omitted display-title defaults to None",
        );
        let plain_json = serde_json::to_string(&plain).expect("serializes");
        assert!(
            !plain_json.contains("display-title"),
            "an absent display-title serializes nothing (skip-on-absent); got {plain_json}",
        );
    }

    /// (M38 inc-1 T1) The doctype-level `placement:` key serde-roundtrips: a fixture
    /// schema declaring `placement: { file: FOO.md }` parses
    /// `placement == Some(Placement { file: "FOO.md" })` (case-preserved literal),
    /// **re-serializes** the key, and survives a load→serialize→reload cycle. A
    /// schema that **omits** it serializes to JSON with **no** `placement` key — so
    /// its `schema_hash` (`blake3` over that JSON, `manifest.rs`) is byte-identical to
    /// the pre-field value, the freeze-safe additive-key proof (mirrors
    /// `display-title`): the field's mere existence on the struct
    /// changes no frozen doctype's hash. (b) proven, not assumed — the real frozen six
    /// are covered by the pack-load freeze regressions (`pack.rs` +
    /// `crates/cli/tests/freeze_enforcement.rs`). See `design/storage.md` → Placement.
    #[test]
    fn placement_serde_roundtrips_and_is_skipped_when_absent() {
        let with_placement = b"\
type: foo
placement: { file: FOO.md }
sections: []
";
        let schema = load_schema(with_placement).expect("placement schema loads");
        assert_eq!(
            schema.placement,
            Some(Placement {
                file: "FOO.md".to_owned()
            }),
            "the placement key parses its case-preserved literal file path",
        );

        // Re-serializes the key, and a reload preserves it (roundtrip).
        let json = serde_json::to_string(&schema).expect("serializes");
        assert!(
            json.contains("\"placement\":{\"file\":\"FOO.md\"}"),
            "the placement key re-serializes under its on-disk shape; got {json}",
        );
        let reloaded: Schema = serde_json::from_str(&json).expect("reloads");
        assert_eq!(reloaded, schema, "placement survives a serde roundtrip");

        // A schema omitting it defaults to None and serializes nothing for it — the
        // skip-on-absent additive guard. No `placement` in the JSON means the bytes
        // `schema_hash` digests are identical to the pre-field value, so no frozen
        // doctype's hash moves (the freeze-safety claim, proven).
        let without = b"\
type: commit
sections: []
";
        let plain = load_schema(without).expect("no-placement schema loads");
        assert!(
            plain.placement.is_none(),
            "an omitted placement defaults to None",
        );
        let plain_json = serde_json::to_string(&plain).expect("serializes");
        assert!(
            !plain_json.contains("placement"),
            "an absent placement serializes nothing (skip-on-absent); got {plain_json}",
        );

        // `deny_unknown_fields`: a placement carrying a stray `root:` flag (the
        // rebutted `{file, root}` sketch) is rejected loudly, never silently absorbed.
        let with_root = b"\
type: foo
placement: { file: FOO.md, root: true }
sections: []
";
        assert!(
            load_schema(with_root).is_err(),
            "a placement with an unknown `root` key must fail to load",
        );
    }

    /// (M22 inc-3 T1, done-criterion (iv)) The `optional: true` flag serde-round-trips
    /// on **both** a `Slot` and a `Field`: a fixture schema declaring each parses
    /// `optional == true`, re-serializes the flag, and survives a reload. A schema
    /// **omitting** it defaults to `false` and, mirroring the `singleton` skip-on-false
    /// discipline, serializes **nothing** — the additive guard that leaves every
    /// existing golden byte-unchanged. See `design/changelog.md` → engine work #3.
    #[test]
    fn optional_flag_serde_roundtrips_and_is_skipped_when_false() {
        let with_optional = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: link, type: string, optional: true }
  - id: body
    slot: { hint: \"The body.\", optional: true }
";
        let schema = load_schema(with_optional).expect("optional schema loads");

        // The field carries optional == true.
        let SectionBody::Simple { slot, fields } = &schema.sections[0].body else {
            panic!("meta is a simple header section");
        };
        let link = fields.iter().find(|f| f.id == "link").unwrap();
        assert!(link.optional, "the optional flag parses as true on a field");
        assert!(slot.is_none());

        // The slot carries optional == true.
        let SectionBody::Simple { slot, .. } = &schema.sections[1].body else {
            panic!("body is a simple slot section");
        };
        assert!(
            slot.as_ref().expect("body has a slot").optional,
            "the optional flag parses as true on a slot",
        );

        // Re-serializes the flag on both, and a reload preserves them (roundtrip).
        let json = serde_json::to_string(&schema).expect("serializes");
        assert!(
            json.matches("\"optional\":true").count() == 2,
            "the optional flag re-serializes on both slot and field; got {json}",
        );
        let reloaded: Schema = serde_json::from_str(&json).expect("reloads");
        assert_eq!(reloaded, schema, "optional survives a serde roundtrip");

        // A schema omitting it defaults to false on both and serializes nothing —
        // the skip-on-false additive guard (no existing golden gains a key).
        let without = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: body
    slot: { hint: \"The body.\" }
";
        let plain = load_schema(without).expect("non-optional schema loads");
        let SectionBody::Simple { fields, .. } = &plain.sections[0].body else {
            panic!("meta is a simple header section");
        };
        assert!(
            !fields[0].optional,
            "an omitted field flag defaults to false"
        );
        let SectionBody::Simple { slot, .. } = &plain.sections[1].body else {
            panic!("body is a simple slot section");
        };
        assert!(
            !slot.as_ref().expect("body has a slot").optional,
            "an omitted slot flag defaults to false",
        );
        let plain_json = serde_json::to_string(&plain).expect("serializes");
        assert!(
            !plain_json.contains("optional"),
            "a false optional flag serializes nothing (skip-on-false); got {plain_json}",
        );
    }

    /// (M36 inc-3) The shipped `adr` schema declares the **optional** `options` slot
    /// section — the v1→v2 shape change (`design/document-type-schema.md` → the options
    /// slot + the optional-slot exemption). It sits after `context` and before `decision`,
    /// and its slot is flagged `optional: true` (the first optional slot on a persisted
    /// doctype). Proven over the bytes that ship.
    #[test]
    fn shipped_adr_declares_the_optional_options_section() {
        let schema =
            load_schema_with_types(ADR_YAML, &dev_pack_field_types()).expect("adr.yaml loads");
        let options = schema
            .sections
            .iter()
            .find(|s| s.id == "options")
            .expect("the shipped adr declares an `options` section");
        let SectionBody::Simple { slot, .. } = &options.body else {
            panic!("options is a simple slot section");
        };
        assert!(
            slot.as_ref().expect("options carries a slot").optional,
            "the shipped adr `options` slot is optional",
        );
        let ids: Vec<&str> = schema.sections.iter().map(|s| s.id.as_str()).collect();
        let ctx = ids.iter().position(|&s| s == "context").expect("context");
        let opt = ids.iter().position(|&s| s == "options").expect("options");
        let dec = ids.iter().position(|&s| s == "decision").expect("decision");
        assert!(
            ctx < opt && opt < dec,
            "the options section sits between context and decision: {ids:?}",
        );
    }

    /// (M16 inc-3 T1) The engine-native `owned-location` field type round-trips
    /// through serde: a fixture schema declaring a field of `type: owned-location`
    /// parses to the new [`FieldType::OwnedLocation`] native variant and
    /// **re-serializes to the bare `owned-location` spelling** (not an unresolved
    /// `Pack`). It loads with the **engine-empty** type set (no pack supplied) —
    /// the proof it is engine-native, not pack-declared. The additive guard: every
    /// other native spelling still round-trips byte-unchanged, so no existing
    /// golden gains or loses a key. See `design/methodology-docs.md` → The engine
    /// work (item 3, the owned-location recognition surface).
    #[test]
    fn owned_location_native_type_serde_roundtrips() {
        let yaml = b"\
type: completion-record
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: owner-artifact, type: owned-location }
";
        let schema = load_schema(yaml).expect("owned-location schema loads engine-native");
        let SectionBody::Simple { fields, .. } = &schema.sections[0].body else {
            panic!("meta is a simple header section");
        };
        let owner = fields.iter().find(|f| f.id == "owner-artifact").unwrap();
        assert_eq!(
            owner.ty,
            FieldType::OwnedLocation,
            "`owned-location` parses to the native variant, not an unresolved Pack",
        );

        // Re-serializes to the bare `owned-location` spelling and survives a reload.
        let json = serde_json::to_string(&schema).expect("serializes");
        assert!(
            json.contains("\"owned-location\""),
            "the type re-serializes to the bare `owned-location` spelling; got {json}",
        );
        let reloaded: Schema = serde_json::from_str(&json).expect("reloads");
        assert_eq!(
            reloaded, schema,
            "owned-location survives a serde roundtrip"
        );

        // The additive guard: every pre-existing native spelling still round-trips
        // to its own bare string (no native golden's bytes shift under the new arm).
        for (spelling, ty) in [
            ("enum", FieldType::Enum),
            ("string", FieldType::String),
            ("date", FieldType::Date),
            ("bool", FieldType::Bool),
            ("int", FieldType::Int),
            ("ref", FieldType::Ref),
        ] {
            let s = serde_json::to_string(&ty).expect("native type serializes");
            assert_eq!(
                s,
                format!("\"{spelling}\""),
                "the `{spelling}` native type still serializes to its bare spelling",
            );
        }
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

    /// (M22 inc-1 T1, done-criterion (i)) A schema with a repeatable nested
    /// inside a repeatable **loads**: the outer repeatable's block carries a
    /// `Leaf::Repeatable`, reachable with its own inner `id-from` + block leaves,
    /// and a nested `code-anchor` field resolves its `doc-code` adjudicator when
    /// the pack-declared set is threaded in. This is the `changelog`'s
    /// `release → change-group` shape (the `Leaf::Repeatable` target).
    #[test]
    fn nested_repeatable_loads_with_inner_id_from_and_resolved_field_types() {
        let yaml = b"\
type: changelog
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
              - { id: maps-to-test, type: code-anchor }
";
        let schema =
            load_schema_with_types(yaml, &code_anchor_decl()).expect("nested repeatable loads");

        let SectionBody::Repeatable { repeatable } = &schema.sections[0].body else {
            panic!("releases is repeatable");
        };
        assert_eq!(repeatable.id_from, "version");
        assert_eq!(repeatable.block.len(), 2);
        assert!(matches!(&repeatable.block[0], Leaf::Field(f) if f.id == "version"));

        // The second leaf is itself a repeatable, reachable with its inner
        // id-source and block leaves.
        let Leaf::Repeatable {
            id,
            repeatable: changes,
        } = &repeatable.block[1]
        else {
            panic!("changes is a nested repeatable leaf");
        };
        assert_eq!(id, "changes");
        assert_eq!(changes.id_from, "category");
        assert_eq!(changes.block.len(), 3);
        assert!(matches!(&changes.block[1], Leaf::Slot { id, .. } if id == "notes"));

        // The nested `code-anchor` field resolves its adjudicator + check —
        // the recursive loader walks nested blocks, not just top-level ones.
        let Leaf::Field(anchor) = &changes.block[2] else {
            panic!("maps-to-test is a nested field leaf");
        };
        assert_eq!(
            anchor.ty,
            FieldType::Pack(PackFieldType {
                name: "code-anchor".to_owned(),
                adjudicator: Some("doc-code".to_owned()),
                check: Some("symbol-exists".to_owned()),
            }),
            "the nested code-anchor resolves through the recursive loader",
        );
    }

    /// (M22 inc-1 T1, done-criterion (ii)) A nested-repeatable schema serde
    /// round-trips (load → serialize → reload equal). The `Leaf::Repeatable`
    /// arm of the untagged `Leaf` enum survives a full cycle, nested field
    /// resolution included.
    #[test]
    fn nested_repeatable_serde_roundtrips() {
        let yaml = b"\
type: changelog
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";
        let schema = load_schema(yaml).expect("nested repeatable loads");
        let json = serde_json::to_string(&schema).expect("serializes");
        let reloaded: Schema = serde_json::from_str(&json).expect("reloads");
        assert_eq!(
            reloaded, schema,
            "nested repeatable survives a serde roundtrip"
        );
    }

    /// (M22 inc-1 T1, done-criterion (ii)) The additive guard: a schema with
    /// **no** nesting is byte-unchanged by this lift. A single-level repeatable
    /// (the shipped `spec`/`arch-doc` shape) still loads and re-serializes
    /// exactly as before — no nested arm leaks into a flat schema's output.
    #[test]
    fn single_level_repeatable_is_byte_unchanged() {
        // A flat single-level repeatable (the shipped `spec`/`arch-doc` shape),
        // using only native field types so the JSON round-trip is lossless (a
        // resolved `Pack` adjudicator is not serialized — that is by design).
        let yaml = b"\
type: spec
sections:
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"The criterion.\" } }
";
        let schema = load_schema(yaml).expect("flat repeatable loads");
        let json = serde_json::to_string(&schema).expect("serializes");
        let reloaded: Schema = serde_json::from_str(&json).expect("reloads");
        assert_eq!(reloaded, schema, "a flat schema round-trips unchanged");
        assert!(
            !json.contains("\"id-from\":\"category\""),
            "no nested change-group arm appears in a flat schema's output",
        );
    }

    /// (M22 inc-1 T1, done-criterion (iii)) Nesting deeper than the **H6 / 4
    /// levels** cap fails to load with a typed `SchemaError::NestingTooDeep`
    /// naming the offending **depth** (a documented cap, not silent
    /// truncation). A top-level repeatable's items are `###` (depth 1), so a
    /// 5th nested level would render at `H7` — rejected loudly.
    #[test]
    fn nesting_deeper_than_h6_is_a_typed_depth_error() {
        // Five repeatable levels: depths 1..=5; the 5th (H7) breaches the cap.
        let yaml = b"\
type: deep
sections:
  - id: l1
    repeatable:
      id-from: a
      block:
        - { id: a, type: string }
        - id: l2
          repeatable:
            id-from: b
            block:
              - { id: b, type: string }
              - id: l3
                repeatable:
                  id-from: c
                  block:
                    - { id: c, type: string }
                    - id: l4
                      repeatable:
                        id-from: d
                        block:
                          - { id: d, type: string }
                          - id: l5
                            repeatable:
                              id-from: e
                              block:
                                - { id: e, type: string }
";
        let err = load_schema(yaml).expect_err("over-deep nesting errors");
        assert!(
            matches!(&err, SchemaError::NestingTooDeep { depth } if *depth == 5),
            "expected NestingTooDeep naming depth 5, got {err:?}",
        );
    }

    /// (M22 inc-1 T1) The boundary holds: nesting **at** the cap (4 levels, the
    /// deepest items rendering at `H6`) loads cleanly — the cap is on the 5th
    /// level, not the 4th (a documented cap, exercised at its edge).
    #[test]
    fn nesting_at_the_h6_cap_loads() {
        let yaml = b"\
type: deep
sections:
  - id: l1
    repeatable:
      id-from: a
      block:
        - { id: a, type: string }
        - id: l2
          repeatable:
            id-from: b
            block:
              - { id: b, type: string }
              - id: l3
                repeatable:
                  id-from: c
                  block:
                    - { id: c, type: string }
                    - id: l4
                      repeatable:
                        id-from: d
                        block:
                          - { id: d, type: string }
";
        load_schema(yaml).expect("4-level nesting (deepest items at H6) loads");
    }

    /// (M40 audit fix) A block declaring **two** nested repeatables is rejected
    /// at load with a typed [`SchemaError::MultipleNestedRepeatables`] naming
    /// the block and both nested ids. The parse model carries one
    /// undifferentiated nested item list per item (`ParsedItem::items`), so a
    /// second sibling nested group is physically unrepresentable — its items
    /// would be silently unioned with the first group's under every declared
    /// block id. Depth stays general (the H6 cap); breadth per block is 1.
    #[test]
    fn two_nested_repeatables_in_one_block_is_a_typed_breadth_error() {
        let yaml = b"\
type: twin
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - id: added
          repeatable:
            id-from: title
            block:
              - { id: title, type: string }
        - id: fixed
          repeatable:
            id-from: title
            block:
              - { id: title, type: string }
";
        let err = load_schema(yaml).expect_err("two nested repeatables per block errors");
        assert!(
            matches!(
                &err,
                SchemaError::MultipleNestedRepeatables { block, first, second }
                    if block == "releases" && first == "added" && second == "fixed"
            ),
            "expected MultipleNestedRepeatables naming `releases`/`added`/`fixed`, got {err:?}",
        );
    }

    /// (M42 inc-8 T3) The `id` key is **reserved** on a repeatable block: the
    /// pinned `doc show --format json` item object keys the item's minted id
    /// under `id` (`design/doc-read-surface.md` → The item `id` closes the json
    /// contract), so a pack declaring an `id` leaf — or naming `id` as the
    /// block's `id-from` — would silently collide with it. Rejected loudly at
    /// pack-load with a typed [`SchemaError::ReservedItemIdKey`], at every
    /// declaring site: a top-level block's leaf, a block's `id-from`, and a
    /// **nested** block's leaf (the guard rides the recursion).
    #[test]
    fn the_reserved_item_id_key_is_rejected_at_every_declaring_site() {
        let leaf = b"\
type: collide
sections:
  - id: releases
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: id, type: string }
";
        let err = load_schema(leaf).expect_err("an `id` leaf in a repeatable block errors");
        assert!(
            matches!(
                &err,
                SchemaError::ReservedItemIdKey { block, site }
                    if block == "releases" && site == "leaf"
            ),
            "expected ReservedItemIdKey naming the `releases` block's leaf, got {err:?}",
        );

        let id_from = b"\
type: collide
sections:
  - id: releases
    repeatable:
      id-from: id
      block:
        - { id: title, type: string }
";
        let err = load_schema(id_from).expect_err("`id-from: id` errors");
        assert!(
            matches!(
                &err,
                SchemaError::ReservedItemIdKey { block, site }
                    if block == "releases" && site == "id-from"
            ),
            "expected ReservedItemIdKey naming the `releases` block's id-from, got {err:?}",
        );

        let nested = b"\
type: collide
sections:
  - id: releases
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: id, type: string }
";
        let err = load_schema(nested).expect_err("an `id` leaf in a NESTED block errors");
        assert!(
            matches!(
                &err,
                SchemaError::ReservedItemIdKey { block, site }
                    if block == "changes" && site == "leaf"
            ),
            "expected ReservedItemIdKey naming the nested `changes` block, got {err:?}",
        );
    }

    /// The exact pre-`include` (inline, duplicated) form of the `changelog`
    /// schema — the change-group block spelled out verbatim at **both** the
    /// `unreleased-changes` site and the nested `releases.changes` site. The
    /// shipped `changelog.yaml` now de-duplicates this via `include`; this
    /// fixture is the byte-identity reference (a managed mention-free,
    /// engine-native-types-only schema, so the bare loader resolves it).
    const INLINE_CHANGELOG: &str = r#"type: changelog
placement: { file: CHANGELOG.md }
display-title: Changelog
singleton: true
id-from: title
description: A Keep-a-Changelog singleton — staged unreleased changes plus the cut releases, each grouped by category, maintained over the life of the project.
usage: a user-facing change lands and the project keeps a human-readable record of what changed, staged now and cut into versioned releases over time.

sections:
  - id: unreleased-changes
    repeatable:
      id-from: category
      block:
        - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
        - { id: notes, slot: { hint: "One bullet per change in this category." } }
  - id: releases
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: date, type: date, set: on-create }
        - { id: link, type: string, optional: true }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
              - { id: notes, slot: { hint: "One bullet per change in this category." } }
"#;

    /// Assert a block resolves to the change-group shape `[category enum, notes
    /// slot]` — the fragment the `include` de-dup pulls in at both sites.
    fn assert_change_group(block: &[Leaf]) {
        assert_eq!(block.len(), 2, "the change-group is [category, notes]");
        let Leaf::Field(category) = &block[0] else {
            panic!("the first change-group leaf is the category field");
        };
        assert_eq!(category.id, "category");
        assert_eq!(category.ty, FieldType::Enum);
        assert_eq!(
            category.of.as_deref(),
            Some(
                [
                    "added",
                    "changed",
                    "deprecated",
                    "removed",
                    "fixed",
                    "security"
                ]
                .map(String::from)
                .as_slice()
            )
        );
        assert!(matches!(&block[1], Leaf::Slot { id, .. } if id == "notes"));
    }

    /// (M33 inc-3 T2, done-criterion) The shipped `include`-form `changelog`
    /// schema loads to a [`Schema`] model **byte-identical** to the inline
    /// (duplicated) form: the change-group block resolves to `[category enum,
    /// notes slot]` at **both** the `unreleased-changes` repeatable and the
    /// nested `releases.changes` repeatable. The `include` directive
    /// de-duplicates the genuinely-shared section with no instance-byte change.
    /// See `design/document-type-schema.md` → On-disk definition format and
    /// `design/corpus-migration.md` → schema-fragment `include`.
    #[test]
    fn changelog_include_form_loads_byte_identically_to_inline() {
        let inline = load_schema(INLINE_CHANGELOG.as_bytes()).expect("inline changelog loads");
        let shipped = load_schema(CHANGELOG_YAML).expect("shipped (include-form) changelog loads");
        assert_eq!(
            inline, shipped,
            "the include-form changelog loads to a model byte-identical to the inline form",
        );

        // The change-group block resolves at the staging site.
        let SectionBody::Repeatable {
            repeatable: unreleased,
        } = &shipped.sections[0].body
        else {
            panic!("unreleased-changes is repeatable");
        };
        assert_eq!(unreleased.id_from, "category");
        assert_change_group(&unreleased.block);

        // ...and again at the nested per-release `changes` site.
        let SectionBody::Repeatable {
            repeatable: releases,
        } = &shipped.sections[1].body
        else {
            panic!("releases is repeatable");
        };
        let Leaf::Repeatable {
            id,
            repeatable: changes,
        } = releases
            .block
            .last()
            .expect("a release's last leaf is the nested changes repeatable")
        else {
            panic!("the nested changes leaf is a repeatable");
        };
        assert_eq!(id, "changes");
        assert_eq!(changes.id_from, "category");
        assert_change_group(&changes.block);
    }

    /// (M33 inc-3 T2) An `include:` directive naming a fragment **absent** from
    /// the top-level `fragments:` map fails to load with a typed
    /// [`SchemaError::UnknownFragment`] naming the offending fragment — a
    /// dangling include is rejected loudly, never silently dropped.
    #[test]
    fn include_naming_an_undefined_fragment_is_a_typed_error() {
        let yaml = b"\
type: x
fragments:
  defined:
    - { id: a, type: string }
sections:
  - id: s
    repeatable:
      id-from: a
      block:
        - include: missing
";
        let err = load_schema(yaml).expect_err("an include of an undefined fragment errors");
        assert!(
            matches!(&err, SchemaError::UnknownFragment { name } if name == "missing"),
            "expected UnknownFragment for `missing`, got {err:?}",
        );
    }

    /// (M33 completion fix) A **self-referential** fragment (`a` includes `a`)
    /// must surface as a typed [`SchemaError::CyclicFragment`] — never an
    /// unbounded recursion that aborts the process. The cycle guard catches the
    /// fragment name re-entered along the current expansion path before the
    /// recursion can overflow the stack.
    #[test]
    fn self_referential_fragment_is_a_typed_error_not_a_stack_overflow() {
        let yaml = b"\
type: x
fragments:
  a:
    - include: a
sections:
  - id: s
    repeatable:
      id-from: f
      block:
        - include: a
";
        let err = load_schema(yaml).expect_err("a self-referential fragment errors");
        assert!(
            matches!(&err, SchemaError::CyclicFragment { name } if name == "a"),
            "expected CyclicFragment for `a`, got {err:?}",
        );
    }

    /// (M33 completion fix) A **mutually-referential** fragment pair
    /// (`a → b → a`) is likewise a typed [`SchemaError::CyclicFragment`], naming
    /// the fragment re-entered along the path — proving the guard tracks the
    /// whole expansion path, not just direct self-reference.
    #[test]
    fn mutually_referential_fragments_are_a_typed_error() {
        let yaml = b"\
type: x
fragments:
  a:
    - include: b
  b:
    - include: a
sections:
  - id: s
    repeatable:
      id-from: f
      block:
        - include: a
";
        let err = load_schema(yaml).expect_err("a mutually-referential fragment cycle errors");
        assert!(
            matches!(&err, SchemaError::CyclicFragment { name } if name == "a"),
            "expected CyclicFragment for `a`, got {err:?}",
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
