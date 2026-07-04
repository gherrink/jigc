//! The schema-diff classifier — a v1→v2 delta over two [`Schema`] values.
//!
//! Step 1 of the deterministic transform (`design/corpus-migration.md` → The
//! deterministic transform): classify each change between an old and a new
//! doc-type [`Schema`] into one of the four kinds the transform driver knows how
//! to apply. The driver (M34 Inc-3 for `added-optional-field`; Inc-2 T3 for the
//! structural kinds) consumes these classifications and emits byte-stable splices
//! over the `write.rs` primitives — **no LLM in the structural path** (the
//! determinism boundary: the CLI owns structure).
//!
//! This module is *classification only* — it compares two already-loaded schemas
//! and never touches an instance. It is presentation-free and domain-empty (the
//! engine invariant). Changes outside the four supported kinds (a removed
//! section/leaf, a type change, a narrowed cardinality — all breaking or
//! data-losing, none a supported transform) are **not** classified here; the
//! conformance detector and the transform gate adjudicate those.

use crate::schema::{Field, Schema, SectionBody};
use std::collections::HashMap;

/// One classified change between an old and a new [`Schema`], by kind.
///
/// Each variant names the section (and, where applicable, the leaf) it concerns —
/// the address the transform driver splices at. The four kinds mirror
/// `design/corpus-migration.md` → The deterministic transform (1. Schema-diff).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaChange {
    /// A field the transform can place **deterministically**: it is either
    /// `optional` (its absence never blocks) or carries a deterministic value
    /// source (`default`/`set`, e.g. the schema-version stamp's `set` deriver).
    /// The driver branch + the "current active schema version" deriver land in
    /// M34 Inc-3 (the live stamp dogfood — the branch-split); only the
    /// *classification* lives here.
    AddedOptionalField {
        /// The section the field was added to.
        section: String,
        /// The added field's id.
        field: String,
    },

    /// A field present in both schemas whose `card` (forward cardinality) changed
    /// (e.g. `0..1` → `0..*`). The driver widens the stored instance's edge set.
    /// Narrowing is data-losing and unsupported — not a separate kind; a `card`
    /// delta is classified as a widening (the only supported direction).
    WidenedCardinality {
        /// The section carrying the field.
        section: String,
        /// The field whose cardinality changed.
        field: String,
    },

    /// A wholly-new **optional** slot section in `v2` — an added `## Heading` whose
    /// body is a simple `slot: { optional: true }` (the adr `options` shape). The
    /// driver mints the empty `## Heading` at its schema-ordered offset; an empty
    /// optional slot conforms, so no prose is needed (unlike [`Self::ProseNeeding`]).
    /// It is **not** a byte no-op: the v2 writer emits the heading unconditionally, so
    /// a historical doc lacking it is non-canonical until the heading is spliced in
    /// (`design/corpus-migration.md` → The deterministic transform, 2. added-optional-
    /// section).
    AddedOptionalSection {
        /// The added optional slot section's id.
        section: String,
    },

    /// A section that was a simple `<<slot>>` becoming a repeatable item-block —
    /// the riskiest net-new transform (the old slot content becomes the default
    /// first item; `design/corpus-migration.md`). Only emitted when the old
    /// section actually carried a slot (the physical prose to promote).
    FixedSlotToRepeatable {
        /// The section promoted from simple-slot to repeatable.
        section: String,
    },

    /// A new **required** slot or field with **no deterministic default** — the
    /// Framing-A escape hatch: the transform mints it empty and routes it to the
    /// agent, which authors the prose; the CLI conformance-gates the result. A
    /// section-level slot is anonymous, so `leaf` is `None` for it and
    /// `Some(<field-id>)` for a required-without-default field.
    ProseNeeding {
        /// The section the new required prose belongs to.
        section: String,
        /// The leaf id for a required field; `None` for a section's own slot.
        leaf: Option<String>,
    },

    /// A **doctype-level home change**: the committed instances' resolved home moved
    /// (`design/storage.md` → Placement; `design/corpus-migration.md` → Relocation). A
    /// **file move, not a content edit** — the bytes are byte-identical at the new home,
    /// so the transform fold is a content no-op (the [`Self::WidenedCardinality`]
    /// sibling); the CLI migrate-corpus arm performs the git-free `fs::rename`. `from` /
    /// `to` are the two schemas' declared homes (`placement.file` else `location`),
    /// auto-derived from the [`Schema`] values alone — engine-pure, no cascade/docs-root
    /// (the determinism boundary: no hand-written move recipe). Classified **outside**
    /// the per-section loop, because `location` / `placement` are `Schema`-level fields
    /// the section-diff never inspects (a pure relocation would otherwise diff to `[]`).
    Relocated {
        /// The v1 home (its declared `placement.file` else `location`).
        from: String,
        /// The v2 home (its declared `placement.file` else `location`).
        to: String,
    },

    /// A **doctype-level `display-title` add/change**: the doc's `# H1` line is rewritten
    /// to the new display text (`# changelog` → `# Changelog`; `design/corpus-migration.md`
    /// → Relocation). A one-line H1 splice, byte-stable otherwise. Classified **outside**
    /// the per-section loop, because `display-title` is a `Schema`-level field the
    /// section-diff never inspects.
    DisplayTitleChanged {
        /// The new H1 display text.
        to: String,
    },
}

/// Classify every supported change from `v1` to `v2` into a deterministic,
/// document-order list of [`SchemaChange`]s. An identical pair yields an empty
/// diff. Output order follows `v2`'s section/leaf document order (a stable
/// property of the [`Schema`] model), so the same pair always diffs identically.
pub fn schema_diff(v1: &Schema, v2: &Schema) -> Vec<SchemaChange> {
    let mut out = Vec::new();

    // Doctype-level changes first, classified **outside** the per-section loop —
    // `location` / `placement` / `display-title` are `Schema`-level fields the
    // section-diff never inspects, so a pure relocation or display-title change would
    // otherwise diff to `[]` and silently no-op (`design/corpus-migration.md` → 1.
    // Schema-diff: the two M38 doctype-level kinds).
    if let (Some(from), Some(to)) = (resolved_home(v1), resolved_home(v2))
        && from != to
    {
        out.push(SchemaChange::Relocated { from, to });
    }
    if let Some(to) = &v2.display_title
        && v1.display_title.as_deref() != Some(to.as_str())
    {
        out.push(SchemaChange::DisplayTitleChanged { to: to.clone() });
    }

    let old_sections: HashMap<&str, &SectionBody> = v1
        .sections
        .iter()
        .map(|s| (s.id.as_str(), &s.body))
        .collect();

    for section in &v2.sections {
        match old_sections.get(section.id.as_str()) {
            Some(old) => diff_section(&section.id, old, &section.body, &mut out),
            None => added_section(&section.id, &section.body, &mut out),
        }
    }
    out
}

/// A doctype's declared home: its `placement.file` literal (a placement doctype) else
/// its `location` directory. `None` for a transient doctype with neither (e.g. `commit`,
/// whose sink is the git message). Engine-pure — the raw schema-declared home, with **no**
/// `docs-root` resolution (that is the CLI's concern; the determinism boundary keeps the
/// engine domain-empty). Representation (dir vs full path) is executor latitude the
/// migrate-corpus move-arm resolves.
fn resolved_home(schema: &Schema) -> Option<String> {
    schema
        .placement
        .as_ref()
        .map(|p| p.file.clone())
        .or_else(|| schema.location.clone())
}

/// Diff a section present in both schemas.
fn diff_section(id: &str, old: &SectionBody, new: &SectionBody, out: &mut Vec<SchemaChange>) {
    match (old, new) {
        // simple → repeatable: only a *promotion of an existing slot* is the
        // supported `fixed-slot→repeatable-with-default` transform.
        (SectionBody::Simple { slot: Some(_), .. }, SectionBody::Repeatable { .. }) => {
            out.push(SchemaChange::FixedSlotToRepeatable {
                section: id.to_owned(),
            });
        }
        (
            SectionBody::Simple {
                slot: old_slot,
                fields: old_fields,
            },
            SectionBody::Simple {
                slot: new_slot,
                fields: new_fields,
            },
        ) => {
            // A newly-introduced required slot needs prose (the slot is
            // anonymous → `leaf: None`). An added *optional* slot leaves existing
            // docs conformant, so it needs no transform.
            if old_slot.is_none()
                && let Some(s) = new_slot
                && !s.optional
            {
                out.push(SchemaChange::ProseNeeding {
                    section: id.to_owned(),
                    leaf: None,
                });
            }
            diff_fields(id, old_fields, new_fields, out);
        }
        // Any other shape change (repeatable→simple, slotless simple→repeatable,
        // removed leaves) is breaking/unsupported — not classified here.
        _ => {}
    }
}

/// Classify the leaves of a section that exists only in `v2` (every leaf is new).
fn added_section(id: &str, new: &SectionBody, out: &mut Vec<SchemaChange>) {
    if let SectionBody::Simple { slot, fields } = new {
        match slot {
            // An added **optional** slot section is deterministically mintable (empty
            // slot conforms) — the added-optional-section transform.
            Some(s) if s.optional => out.push(SchemaChange::AddedOptionalSection {
                section: id.to_owned(),
            }),
            // A new **required** slot needs prose (the slot is anonymous → `leaf: None`).
            Some(_) => out.push(SchemaChange::ProseNeeding {
                section: id.to_owned(),
                leaf: None,
            }),
            None => {}
        }
        diff_fields(id, &[], fields, out);
    }
    // A wholly-new repeatable section is not one of the transform kinds.
}

/// Diff a simple section's field list: classify added fields and cardinality
/// widenings (matching by field id; field document order is `v2`'s).
fn diff_fields(section: &str, old: &[Field], new: &[Field], out: &mut Vec<SchemaChange>) {
    let old_by_id: HashMap<&str, &Field> = old.iter().map(|f| (f.id.as_str(), f)).collect();
    for field in new {
        match old_by_id.get(field.id.as_str()) {
            Some(prev) => {
                if prev.card != field.card {
                    out.push(SchemaChange::WidenedCardinality {
                        section: section.to_owned(),
                        field: field.id.clone(),
                    });
                }
            }
            None => out.push(classify_added_field(section, field)),
        }
    }
}

/// Classify a field present only in `v2`. A field the driver can place without
/// new prose — `optional`, or carrying a deterministic value (`default`/`set`) —
/// is [`SchemaChange::AddedOptionalField`]; a *required* field with no such
/// default is [`SchemaChange::ProseNeeding`] (the design's "no deterministic
/// default" boundary of the prose-needing kind).
fn classify_added_field(section: &str, field: &Field) -> SchemaChange {
    if field.optional || field.default.is_some() || field.set.is_some() {
        SchemaChange::AddedOptionalField {
            section: section.to_owned(),
            field: field.id.clone(),
        }
    } else {
        SchemaChange::ProseNeeding {
            section: section.to_owned(),
            leaf: Some(field.id.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::load_schema;

    fn load(yaml: &[u8]) -> Schema {
        load_schema(yaml).expect("fixture schema loads")
    }

    /// An identical pair (a non-trivial schema with a header field group and a
    /// ref) yields an **empty** diff — proof the classifier diffs rather than
    /// always emitting.
    #[test]
    fn identical_schemas_yield_an_empty_diff() {
        let yaml = b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: rel, type: ref, to: t, card: \"0..1\" }
  - id: rationale
    slot: { hint: \"why\" }
";
        let schema = load(yaml);
        assert_eq!(schema_diff(&schema, &schema), vec![]);
    }

    /// `added-optional-field`: a new `optional: true` field in an existing
    /// section classifies to [`SchemaChange::AddedOptionalField`] naming the
    /// section + field.
    #[test]
    fn added_optional_field_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: link, type: string, optional: true }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedOptionalField {
                section: "meta".to_owned(),
                field: "link".to_owned(),
            }]
        );
    }

    /// A required field that carries a **deterministic default** is *not*
    /// prose-needing (the design's "no deterministic default" exclusion): it
    /// classifies to [`SchemaChange::AddedOptionalField`] — the deterministic-add
    /// branch (the schema-version stamp's `set` deriver rides this path).
    #[test]
    fn added_required_field_with_a_default_is_a_deterministic_add() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: kind, type: enum, of: [a, b], default: a }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedOptionalField {
                section: "meta".to_owned(),
                field: "kind".to_owned(),
            }]
        );
    }

    /// `widened-cardinality`: a field present in both whose `card` changed
    /// classifies to [`SchemaChange::WidenedCardinality`] naming the section +
    /// field.
    #[test]
    fn widened_cardinality_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: rel, type: ref, to: t, card: \"0..1\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: rel, type: ref, to: t, card: \"0..*\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::WidenedCardinality {
                section: "meta".to_owned(),
                field: "rel".to_owned(),
            }]
        );
    }

    /// `added-optional-section`: a wholly-new section whose body is an **optional**
    /// slot classifies to exactly [`SchemaChange::AddedOptionalSection`] naming the
    /// section — never `WidenedCardinality`, never the empty diff (the adr `options`
    /// v1→v2 shape, synthetically). The neighbours are unchanged, so it is the *only*
    /// change.
    #[test]
    fn added_optional_section_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: consequences
    slot: { hint: \"the consequences\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: options
    slot: { hint: \"options considered\", optional: true }
  - id: consequences
    slot: { hint: \"the consequences\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedOptionalSection {
                section: "options".to_owned(),
            }]
        );
    }

    /// `fixed-slot→repeatable-with-default`: a section that was a simple slot
    /// becoming a repeatable item-block classifies to
    /// [`SchemaChange::FixedSlotToRepeatable`] naming the section.
    #[test]
    fn fixed_slot_to_repeatable_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: requirements
    slot: { hint: \"the requirements prose\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: requirements
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one requirement\" } }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::FixedSlotToRepeatable {
                section: "requirements".to_owned(),
            }]
        );
    }

    /// `prose-needing`: a new **required** slot (here a wholly-new section whose
    /// body is a required slot) classifies to [`SchemaChange::ProseNeeding`] with
    /// `leaf: None` (a section's slot is anonymous), naming the section.
    #[test]
    fn prose_needing_required_slot_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: rationale
    slot: { hint: \"why this decision\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ProseNeeding {
                section: "rationale".to_owned(),
                leaf: None,
            }]
        );
    }

    /// A required field with **no** default/set is prose-needing, named by its
    /// field id (the `Some(leaf)` arm of the prose-needing kind) — the
    /// counterpart to the deterministic-add case above.
    #[test]
    fn prose_needing_required_field_names_the_leaf() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: owner, type: string }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ProseNeeding {
                section: "meta".to_owned(),
                leaf: Some("owner".to_owned()),
            }]
        );
    }

    // ---- the two M38 doctype-level kinds (classified outside the per-section loop) ----

    /// `relocated`: a **pure home change** (v1 `location:` → v2 `placement:`) with
    /// identical sections classifies to **exactly** `[Relocated{from,to}]`, **not** the
    /// empty diff — proof the doctype-level fields are diffed *outside* the per-section
    /// loop (which never inspects `location`/`placement`), so a relocation cannot
    /// silently no-op. `from`/`to` are the schemas' declared homes, engine-pure.
    #[test]
    fn relocated_pure_home_change_classifies_to_exactly_relocated() {
        let v1 = load(
            b"\
type: changelog
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        let v2 = load(
            b"\
type: changelog
placement: { file: CHANGELOG.md }
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::Relocated {
                from: "changelog/".to_owned(),
                to: "CHANGELOG.md".to_owned(),
            }]
        );
    }

    /// `display-title-changed`: a `display-title:` add (sections + home unchanged)
    /// classifies to **exactly** `[DisplayTitleChanged{to}]` naming the new H1 text.
    #[test]
    fn display_title_add_classifies_to_exactly_display_title_changed() {
        let v1 = load(
            b"\
type: changelog
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        let v2 = load(
            b"\
type: changelog
location: changelog/
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::DisplayTitleChanged {
                to: "Changelog".to_owned(),
            }]
        );
    }

    /// A combined `location:` → `placement:` **and** `display-title:` add classifies to
    /// **both** doctype-level kinds, in the fixed doctype-level order (relocation, then
    /// the H1 re-title) — the real `changelog` v1→v2 relocation, synthetically.
    #[test]
    fn combined_relocation_and_display_title_add_classifies_both_kinds() {
        let v1 = load(
            b"\
type: changelog
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        let v2 = load(
            b"\
type: changelog
placement: { file: CHANGELOG.md }
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![
                SchemaChange::Relocated {
                    from: "changelog/".to_owned(),
                    to: "CHANGELOG.md".to_owned(),
                },
                SchemaChange::DisplayTitleChanged {
                    to: "Changelog".to_owned(),
                },
            ]
        );
    }

    /// The classifier **diffs**, never emits on mere presence: a schema carrying both a
    /// `location:` and a `display-title:`, diffed against itself, yields the empty diff
    /// (neither doctype-level kind fires when nothing changed) — the inert-when-unchanged
    /// guard for the omitting context.
    #[test]
    fn unchanged_home_and_display_title_emit_no_doctype_level_change() {
        let s = load(
            b"\
type: changelog
location: changelog/
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        assert_eq!(schema_diff(&s, &s), vec![]);
    }
}
