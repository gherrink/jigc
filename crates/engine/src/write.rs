//! Schema-driven Markdown **write path**: the canonical writer (parser-inverse).
//!
//! The #1 technical risk: lossless, diff-clean round-trip. This module owns the
//! *generate* half — the canonical writer that is the **inverse of the parser**
//! ([`crate::parse`]): schema (+ id-source title, field values, slot prose, items)
//! → canonical Markdown bytes, in the **exact frozen byte form**
//! (`DECISIONS.md` 2026-05-31 → Canonical byte form; `design/storage.md` → Anatomy;
//! `implementation/parsing.md` → The write pipeline). It serves `create`,
//! `add-item`, the `--template` blank-instance view, and the **string sink**
//! (rendering the `commit` doc into the git commit message at finalize).
//!
//! ## The canonical byte form (frozen)
//!
//! - **Front-matter** (header section, if the schema declares one): a `---\n`-fenced
//!   flat `key: value` block (one field per line, schema order, no blank line between
//!   the fences and the fields), then a closing `---`, then a **single blank line**.
//! - **`# H1` title** — the id-source field's value, rendered once.
//! - **Body sections** — `## <Heading>\n`, then a blank line, then the slot prose;
//!   the heading text is the section id (the MVP schemas use heading == id slugged
//!   to a title; the parser matches case-insensitively, so we Title-Case the id).
//! - **A body/item field group** — `<slot prose>\n\n<!-- fields -->\n- key: value` —
//!   exactly one blank line before the sentinel, the sentinel on its own line, then
//!   a contiguous bullet list (`- ` prefix, single space), one field per line in
//!   schema order, **no** blank line between the sentinel and the list or between
//!   bullets. The sentinel is emitted **only** when ≥1 field has a value.
//! - **Repeatable items** — `### <title>  {#id}\n` (two spaces before `{#id}`), then a
//!   blank line, then the item body (slot prose + optional field group).
//! - **EOF** — exactly one trailing newline (LF); LF line endings throughout.
//!
//! ## The determinism boundary, applied literally
//!
//! The writer owns **block structure** (the `---` fences, the `##`/`###` headings,
//! the `<!-- fields -->` sentinel + bullets, the `{#id}` anchors) and treats **slot
//! prose as opaque bytes** it emits verbatim. It never reflows, re-wraps, or
//! reformats prose — it places the bytes the caller hands it between the structural
//! markers it owns.
//!
//! ## The round-trip guarantee (the #1 risk)
//!
//! The writer is the parser's inverse on canonical content: `write → parse → write`
//! is **byte-identical**, and the writer emits only parser-accepted forms (so
//! generated and conformant-human content converge on one form). This is
//! golden-tested (exact bytes for a `commit` and an `adr` instance) and
//! property-tested (`write → parse → write` byte-identical over arbitrary
//! schema-valid instances).

use std::fmt::Write as _;
use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::field_block::{self, Field, FieldBlock, Value};
use crate::parse::{self, Block};
use crate::schema::{Schema, Section, SectionBody};

/// A full in-memory document instance, ready to render — the writer's input and the
/// natural inverse of the parser's [`crate::parse::Document`].
///
/// `title` is the H1 / id-source value (rendered once as `# <title>`); `sections`
/// supply each section's content, matched to the schema by id. Slot prose is owned
/// opaque bytes (not a span), since the writer *generates* — it has no source buffer
/// to slice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Instance {
    /// The H1 title — the id-source field's value, rendered once.
    pub title: String,
    /// The section content, matched to the schema sections by id.
    pub sections: Vec<SectionContent>,
}

/// One section's content: its slot prose (if any), its field values (if any), and —
/// for a repeatable section — its items. The inverse of [`crate::parse::ParsedSection`].
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SectionContent {
    /// The section id (matched against the schema).
    pub id: String,
    /// The slot prose, opaque bytes emitted verbatim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    /// The section's field values, in schema order. The writer emits the front-matter
    /// (header section) or a sentinelled trailing bullet group (body section).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
    /// A repeatable section's items, in physical order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ItemContent>,
}

/// One repeatable item's content: its frozen `{#id}` anchor, its mutable title (the
/// `###` heading text), its slot prose, and its per-item field values. The inverse of
/// [`crate::parse::ParsedItem`].
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ItemContent {
    /// The frozen `{#id}` anchor.
    pub id: String,
    /// The item's title (the `###` heading text).
    pub title: String,
    /// The item body's slot prose, opaque bytes emitted verbatim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    /// The item's per-item field values, in schema order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
}

/// The reserved field-group boundary marker (`parsing.md` → Field-group delineation).
const FIELD_SENTINEL: &str = "<!-- fields -->";

/// Render `instance` against `schema` to canonical Markdown bytes.
///
/// Walks the schema sections in document order, matching each to the instance's
/// content by id, and emits the frozen canonical byte form (front-matter, `# H1`,
/// `## …` sections with slot prose, `<!-- fields -->`-sentinelled field groups,
/// `### …{#id}` repeatable items). Slot prose is emitted verbatim (opaque). The
/// output is the parser's inverse on canonical content (`write → parse → write`
/// byte-identical), terminated by exactly one trailing `\n`.
pub fn render(schema: &Schema, instance: &Instance) -> String {
    let mut out = String::new();

    // The header (front-matter) section, if the schema declares one, renders as a
    // `---`-fenced flat field block at the top.
    let header = schema.sections.iter().find(|s| s.header);
    if let Some(header) = header {
        let content = section_content(instance, &header.id);
        let fields = content.map(|c| c.fields.as_slice()).unwrap_or(&[]);
        out.push_str("---\n");
        out.push_str(&emit_bare_fields(fields));
        out.push_str("---\n\n");
    }

    // The `# H1` title — the id-source value.
    let _ = writeln!(out, "# {}\n", instance.title.trim());

    // Body sections, in schema document order.
    let body_sections = schema.sections.iter().filter(|s| !s.header);
    let mut rendered_body = Vec::new();
    for section in body_sections {
        let content = section_content(instance, &section.id);
        rendered_body.push(render_section(section, content));
    }
    out.push_str(&rendered_body.join("\n"));

    // Exactly one trailing newline at EOF.
    ensure_single_trailing_newline(&mut out);
    out
}

/// Render one body section: its `## Heading`, a blank line, then either its slot
/// prose + optional trailing field group (simple) or its `### …{#id}` items
/// (repeatable). Returns the section block *without* a trailing blank line; the
/// caller joins blocks with a single blank line.
fn render_section(section: &Section, content: Option<&SectionContent>) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "## {}", heading_text(&section.id));

    // The schema `body` discriminates the section *shape*; the per-leaf detail
    // (which slot, which fields) is the caller's content concern — the writer places
    // the bytes it is handed between the structural markers it owns.
    match &section.body {
        SectionBody::Simple { .. } => {
            let prose = content.and_then(|c| c.slot.as_deref()).unwrap_or("");
            let field_values = content.map(|c| c.fields.as_slice()).unwrap_or(&[]);
            out.push('\n');
            out.push_str(prose.trim_end());
            append_field_group(&mut out, field_values);
        }
        SectionBody::Repeatable { .. } => {
            let items = content.map(|c| c.items.as_slice()).unwrap_or(&[]);
            out.push('\n');
            let blocks: Vec<String> = items.iter().map(render_item).collect();
            out.push_str(&blocks.join("\n"));
        }
    }
    out
}

/// Render one repeatable item: `### <title>  {#id}`, a blank line, the slot prose,
/// then an optional trailing field group. Two spaces precede the `{#id}` anchor.
fn render_item(item: &ItemContent) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "### {}  {{#{}}}", item.title.trim(), item.id);
    out.push('\n');
    let prose = item.slot.as_deref().unwrap_or("");
    out.push_str(prose.trim_end());
    append_field_group(&mut out, &item.fields);
    out
}

/// Append a sentinelled trailing field group to a section/item body, *only* when at
/// least one field is present. Emits `\n\n<!-- fields -->\n` then the bullet list (one
/// `- key: value` per field, no blank line between bullets), and terminates the body
/// with a single `\n`. With no fields, terminates the prose with a single `\n`.
fn append_field_group(out: &mut String, fields: &[Field]) {
    if fields.is_empty() {
        out.push('\n');
        return;
    }
    out.push_str("\n\n");
    out.push_str(FIELD_SENTINEL);
    out.push('\n');
    for field in fields {
        out.push_str("- ");
        out.push_str(&emit_one_field(field));
        out.push('\n');
    }
}

/// Emit a bare flat field block (header front-matter): one `key: value` line per
/// field, schema order, no bullets, each terminated by `\n`.
fn emit_bare_fields(fields: &[Field]) -> String {
    field_block::emit(&FieldBlock {
        fields: fields.to_vec(),
    })
}

/// Emit one field as its canonical `key: value` text (no trailing newline, no
/// bullet) — reuses the field-block emitter over a single-field block.
fn emit_one_field(field: &Field) -> String {
    let line = field_block::emit(&FieldBlock {
        fields: vec![field.clone()],
    });
    line.trim_end_matches('\n').to_string()
}

/// Find the instance's content for a schema section id.
fn section_content<'a>(instance: &'a Instance, id: &str) -> Option<&'a SectionContent> {
    instance.sections.iter().find(|s| s.id == id)
}

/// The schema id, ignoring any per-leaf detail, is also the source of the section
/// heading text. The MVP schemas use lowercase ids (`context`, `body`); the parser
/// matches headings case-insensitively, so we render a Title-Cased heading
/// (`Context`, `Body`) and the round-trip still parses. A hyphenated id Title-Cases
/// each word.
fn heading_text(id: &str) -> String {
    id.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Ensure the rendered document ends in exactly one `\n` (the frozen EOF rule).
fn ensure_single_trailing_newline(out: &mut String) {
    while out.ends_with('\n') {
        out.pop();
    }
    out.push('\n');
}

#[cfg(test)]
mod canonical {
    //! The canonical writer's round-trip contract: golden (exact bytes) + property
    //! (`write → parse → write` byte-identical). This is the #1-risk contract on the
    //! write side (`parsing.md` → Round-trip guarantees clause 1: idempotent on
    //! canonical content).

    use super::*;
    use crate::field_block::Value;
    use crate::parse::parse_sections;
    use crate::schema::load_schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }
    fn commit_schema() -> Schema {
        load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    fn scalar(key: &str, value: &str) -> Field {
        Field {
            key: key.to_string(),
            value: Value::Scalar(value.to_string()),
        }
    }

    /// Golden: a fixed `commit` instance renders to the exact frozen bytes — front
    /// matter (`subject`), a single blank line, the `# H1`, the `## Body` section
    /// with its slot prose, exactly one trailing newline. Pins the no-field-group
    /// case (the body slot has no fields, so no sentinel).
    #[test]
    fn commit_instance_renders_canonical_bytes() {
        let instance = Instance {
            title: "Add the rate limiter".to_string(),
            sections: vec![
                SectionContent {
                    id: "header".to_string(),
                    fields: vec![scalar("subject", "Add the rate limiter")],
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    slot: Some("Centralize limiting at the gateway.".to_string()),
                    ..Default::default()
                },
            ],
        };
        let out = render(&commit_schema(), &instance);
        insta::assert_snapshot!("commit_instance", out);
    }

    /// Golden: a fixed `adr` instance renders to the exact frozen bytes — front
    /// matter with `status` + `date` + the `supersedes` **ref**, the `# H1` title,
    /// and the three prose slot sections (`## Context` / `## Decision` /
    /// `## Consequences`), each with its opaque prose, exactly one trailing newline.
    #[test]
    fn adr_instance_renders_canonical_bytes() {
        let instance = Instance {
            title: "Rate-limit at the gateway".to_string(),
            sections: vec![
                SectionContent {
                    id: "status".to_string(),
                    fields: vec![
                        scalar("status", "accepted"),
                        scalar("date", "2026-05-23"),
                        scalar("supersedes", "adr:single-node-cache"),
                    ],
                    ..Default::default()
                },
                SectionContent {
                    id: "context".to_string(),
                    slot: Some("Per-client limits were enforced ad hoc.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "decision".to_string(),
                    slot: Some("Centralize rate limiting at the gateway.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "consequences".to_string(),
                    slot: Some("Each service drops its local limiter.".to_string()),
                    ..Default::default()
                },
            ],
        };
        let out = render(&adr_schema(), &instance);
        insta::assert_snapshot!("adr_instance", out);
    }

    /// Golden: a repeatable-section instance (a `spec`-shaped schema) renders the
    /// `### <title>  {#id}` items with the two-space anchor gap, per-item slot prose,
    /// and a per-item sentinelled field group (`maps-to-test`). Pins the item byte
    /// form (two spaces before `{#id}`, the sentinel + bullet, blank lines between
    /// items).
    #[test]
    fn repeatable_items_render_canonical_bytes() {
        let yaml = b"\
type: spec
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"x\" } }
        - { id: maps-to-test, type: code-anchor }
";
        let schema = load_schema(yaml).expect("spec schema loads");
        let instance = Instance {
            title: "Auth flow".to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![scalar("title", "Auth flow")],
                    ..Default::default()
                },
                SectionContent {
                    id: "criteria".to_string(),
                    items: vec![
                        ItemContent {
                            id: "rate-limit".to_string(),
                            title: "Rate limit holds at 100/min".to_string(),
                            slot: Some(
                                "The gateway rejects the 101st request in a 60s window."
                                    .to_string(),
                            ),
                            fields: vec![scalar("maps-to-test", "`test/rate_limit_spec.rb#burst`")],
                        },
                        ItemContent {
                            id: "burst-allowance".to_string(),
                            title: "Burst allowance".to_string(),
                            slot: Some(
                                "A short burst above the limit is tolerated for 2s.".to_string(),
                            ),
                            fields: vec![],
                        },
                    ],
                    ..Default::default()
                },
            ],
        };
        let out = render(&schema, &instance);
        insta::assert_snapshot!("repeatable_items_instance", out);
    }

    /// Idempotence (golden side): the rendered ADR bytes parse, and re-rendering the
    /// parsed instance is byte-identical — `write → parse → write` on canonical
    /// content. Built explicitly (not via the property) so the exact bytes are
    /// asserted equal, the concrete #1-risk round-trip on the persisted doc-type.
    #[test]
    fn adr_render_parse_render_is_byte_identical() {
        let schema = adr_schema();
        let instance = Instance {
            title: "Rate-limit at the gateway".to_string(),
            sections: vec![
                SectionContent {
                    id: "status".to_string(),
                    fields: vec![scalar("status", "accepted"), scalar("date", "2026-05-23")],
                    ..Default::default()
                },
                SectionContent {
                    id: "context".to_string(),
                    slot: Some("Forces at play.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "decision".to_string(),
                    slot: Some("We centralize.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "consequences".to_string(),
                    slot: Some("Local limiters retired.".to_string()),
                    ..Default::default()
                },
            ],
        };
        let first = render(&schema, &instance);
        let reparsed = reparse_to_instance(&schema, &first);
        let second = render(&schema, &reparsed);
        assert_eq!(
            first, second,
            "write → parse → write must be byte-identical"
        );
    }

    /// Re-derive an [`Instance`] from a parsed [`crate::parse::Document`] over
    /// `source`, owning the slot prose (re-slicing the spans). The bridge that lets
    /// the property assert `write → parse → write`.
    fn reparse_to_instance(schema: &Schema, source: &str) -> Instance {
        let doc = parse_sections(schema, source).expect("rendered bytes parse");
        // The title is the H1 — re-scan it from the source (the id-source value).
        let title = source
            .lines()
            .find_map(|l| l.strip_prefix("# "))
            .unwrap_or("")
            .to_string();
        let sections = doc
            .sections
            .iter()
            .map(|s| SectionContent {
                id: s.id.clone(),
                slot: s.slot.as_ref().map(|sp| sp.slice(source).to_string()),
                fields: s.fields.clone(),
                items: s
                    .items
                    .iter()
                    .map(|it| ItemContent {
                        id: it.id.clone(),
                        title: it.title.clone(),
                        slot: it.slot.as_ref().map(|sp| sp.slice(source).to_string()),
                        fields: it.fields.clone(),
                    })
                    .collect(),
            })
            .collect();
        Instance { title, sections }
    }
}

#[cfg(test)]
mod prop_tests {
    //! The #1-risk property: for arbitrary schema-valid in-memory instances,
    //! `write → parse → write` is byte-identical (idempotent on canonical content),
    //! and the field-group sentinel is present iff ≥1 field has a value.

    use super::*;
    use crate::field_block::Value;
    use crate::parse::parse_sections;
    use crate::schema::load_schema;
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    /// Opaque slot prose: arbitrary heading-free, sentinel-free paragraphs joined by
    /// blank lines (so they survive the parser's block split and the canonical form
    /// places them contiguously). No leading/trailing whitespace lines.
    fn prose_strategy() -> impl Strategy<Value = String> {
        let line = "[a-zA-Z][a-zA-Z0-9 .,]{0,40}";
        prop::collection::vec(line, 1..4).prop_map(|lines| lines.join("\n\n"))
    }

    /// A canonical enum/scalar field value (no newline, no `:`-ambiguity beyond the
    /// first, not list-shaped) — trimmed opaque text.
    fn value_strategy() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9][a-zA-Z0-9 ._:/-]{0,20}[a-zA-Z0-9]|[a-zA-Z0-9]"
            .prop_filter("not list-shaped", |s: &String| {
                !(s.starts_with('[') && s.ends_with(']'))
            })
    }

    proptest! {
        /// Idempotent on canonical content: an arbitrary schema-valid ADR instance
        /// renders, parses, and re-renders **byte-identical**. The #1-risk round-trip
        /// on the write side (`parsing.md` → Round-trip guarantees clause 1).
        #[test]
        fn adr_write_parse_write_is_byte_identical(
            status in prop::sample::select(vec!["proposed", "accepted", "superseded"]),
            context in prose_strategy(),
            decision in prose_strategy(),
            consequences in prose_strategy(),
            supersedes in proptest::option::of("[a-z][a-z0-9-]{0,20}"),
        ) {
            let schema = adr_schema();
            let mut fields = vec![
                Field { key: "status".into(), value: Value::Scalar(status.to_string()) },
                Field { key: "date".into(), value: Value::Scalar("2026-05-31".into()) },
            ];
            if let Some(target) = &supersedes {
                fields.push(Field {
                    key: "supersedes".into(),
                    value: Value::Scalar(format!("adr:{target}")),
                });
            }
            let instance = Instance {
                title: "A decision".into(),
                sections: vec![
                    SectionContent { id: "status".into(), fields, ..Default::default() },
                    SectionContent { id: "context".into(), slot: Some(context), ..Default::default() },
                    SectionContent { id: "decision".into(), slot: Some(decision), ..Default::default() },
                    SectionContent { id: "consequences".into(), slot: Some(consequences), ..Default::default() },
                ],
            };
            let first = render(&schema, &instance);
            let doc = parse_sections(&schema, &first).expect("rendered ADR parses");
            let title = first.lines().find_map(|l| l.strip_prefix("# ")).unwrap_or("").to_string();
            let reparsed = Instance {
                title,
                sections: doc.sections.iter().map(|s| SectionContent {
                    id: s.id.clone(),
                    slot: s.slot.as_ref().map(|sp| sp.slice(&first).to_string()),
                    fields: s.fields.clone(),
                    items: Vec::new(),
                }).collect(),
            };
            let second = render(&schema, &reparsed);
            prop_assert_eq!(&first, &second);
        }

        /// The sentinel is present iff ≥1 field has a value. A spec item with a
        /// `maps-to-test` value emits the `<!-- fields -->` sentinel; an item with no
        /// fields emits none.
        #[test]
        fn sentinel_present_iff_fields_present(
            anchor in "[a-z][a-z0-9-]{0,12}".prop_map(|s| crate::slug::slugify(&s))
                .prop_filter("non-empty", |s: &String| !s.is_empty()),
            test_value in proptest::option::of(value_strategy()),
        ) {
            let yaml = b"\
type: spec
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"x\" } }
        - { id: maps-to-test, type: code-anchor }
";
            let schema = load_schema(yaml).expect("spec schema loads");
            let fields = match &test_value {
                Some(v) => vec![Field { key: "maps-to-test".into(), value: Value::Scalar(v.clone()) }],
                None => vec![],
            };
            let instance = Instance {
                title: "T".into(),
                sections: vec![
                    SectionContent { id: "meta".into(), fields: vec![
                        Field { key: "title".into(), value: Value::Scalar("T".into()) },
                    ], ..Default::default() },
                    SectionContent { id: "criteria".into(), items: vec![ItemContent {
                        id: anchor,
                        title: "Crit".into(),
                        slot: Some("A statement.".into()),
                        fields,
                    }], ..Default::default() },
                ],
            };
            let out = render(&schema, &instance);
            prop_assert_eq!(out.contains(FIELD_SENTINEL), test_value.is_some());
        }
    }
}

// ============================================================================
// Surgical splice — the edit half of the write path (see module doc above; the
// generate half is the canonical writer). Present-target edits splice the located
// byte span; absent targets route to generation.
// ============================================================================

/// A located surgical-splice failure: the target the caller named is not present in
/// the source (so the caller must route to the *generation* path), or the source
/// does not conform to the schema (so no target can be located at all).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpliceError {
    /// The named section / field / item is **absent** from the source. A present
    /// target is a surgical splice; an absent one is a generation concern, so the
    /// caller routes to [`crate::write`] + insertion, not here.
    NotPresent {
        /// A human-readable description of what was sought (e.g. `field "status"`).
        what: String,
    },
    /// The source does not parse against the schema, so no span can be located. The
    /// surgical splice only runs over a conformant buffer.
    NotConformant,
}

/// Replace the bytes of `span` in `source` with `replacement`, copying every other
/// byte through verbatim. The pure splice primitive: `source[..start]` then
/// `replacement` then `source[end..]`. Surgical by construction — nothing outside the
/// span is touched, so an unedited, differently-spaced buffer stays byte-for-byte
/// intact.
pub fn splice(source: &str, span: Range<usize>, replacement: &str) -> String {
    let mut out = String::with_capacity(source.len() - (span.end - span.start) + replacement.len());
    out.push_str(&source[..span.start]);
    out.push_str(replacement);
    out.push_str(&source[span.end..]);
    out
}

/// `set-slot` (section present): replace the located slot prose span of `section_id`
/// with `new_prose`, leaving every other byte intact. The slot span is the parser's
/// recorded opaque span; the new prose is placed verbatim (the determinism boundary).
/// An absent section / a section with no slot → [`SpliceError::NotPresent`].
pub fn set_slot(
    schema: &Schema,
    source: &str,
    section_id: &str,
    new_prose: &str,
) -> Result<String, SpliceError> {
    let doc = parse::parse_sections(schema, source).map_err(|_| SpliceError::NotConformant)?;
    let section = doc
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("section {section_id:?}"),
        })?;
    let span = section
        .slot
        .as_ref()
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("slot in section {section_id:?}"),
        })?;
    Ok(splice(source, span.start..span.end, new_prose))
}

/// `set-field` (field present): replace the **value** bytes of `field_key` in
/// `section_id` with `new_value`, leaving the key, the `:`, the leading space, and
/// every other byte intact. Locates the field's physical line via the block parse
/// (front-matter or a sentinelled body bullet) and splices only the value text after
/// `key:`. An absent field / section → [`SpliceError::NotPresent`].
pub fn set_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    field_key: &str,
    new_value: &str,
) -> Result<String, SpliceError> {
    // Re-parse to assert conformance and that the field is present (read path types
    // the value; here we only need to confirm presence before locating bytes).
    let doc = parse::parse_sections(schema, source).map_err(|_| SpliceError::NotConformant)?;
    let section = doc
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("section {section_id:?}"),
        })?;
    if !section.fields.iter().any(|f| f.key == field_key) {
        return Err(SpliceError::NotPresent {
            what: format!("field {field_key:?} in section {section_id:?}"),
        });
    }

    let value_span =
        locate_field_value(source, field_key).ok_or_else(|| SpliceError::NotPresent {
            what: format!("field {field_key:?} value line"),
        })?;
    Ok(splice(source, value_span, new_value))
}

/// `remove-item` (item present): delete the whole block of the repeatable item
/// `item_id` in `section_id` — its `### …{#id}` heading through the start of the
/// next item / section boundary, including the trailing blank-line separator — so the
/// surrounding items stay canonically spaced. An absent item / non-repeatable section
/// → [`SpliceError::NotPresent`].
pub fn remove_item(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_id: &str,
) -> Result<String, SpliceError> {
    let doc = parse::parse_sections(schema, source).map_err(|_| SpliceError::NotConformant)?;
    let section = doc
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("section {section_id:?}"),
        })?;
    // The section must be repeatable and the item must be present.
    let schema_section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or(SpliceError::NotConformant)?;
    if !matches!(schema_section.body, SectionBody::Repeatable { .. }) {
        return Err(SpliceError::NotPresent {
            what: format!("repeatable item in non-repeatable section {section_id:?}"),
        });
    }
    if !section.items.iter().any(|i| i.id == item_id) {
        return Err(SpliceError::NotPresent {
            what: format!("item {item_id:?} in section {section_id:?}"),
        });
    }

    let span = locate_item_block(source, item_id).ok_or_else(|| SpliceError::NotPresent {
        what: format!("item {item_id:?} block"),
    })?;
    Ok(splice(source, span, ""))
}

/// Locate the **value** byte span of the field `key` — the bytes after `key:` (and
/// its single separating space) to the end of that physical line. Reuses the block
/// parse to find the line: a front-matter `key: value` line or a body `- key: value`
/// bullet. Returns `None` if no such field line exists (block parse, never a naive
/// scan, so a `key:`-looking line inside fenced prose is not matched).
fn locate_field_value(source: &str, key: &str) -> Option<Range<usize>> {
    for block in parse::scan_blocks(source) {
        match block {
            // Front-matter: each `key: value` line inside the metadata content range.
            Block::Metadata { content } => {
                if let Some(span) = field_value_in_lines(source, content.clone(), key, false) {
                    return Some(span);
                }
            }
            // A body field group: each list item is a `- key: value` bullet.
            Block::List { items, .. } => {
                for item in &items {
                    if let Some(span) = field_value_in_lines(source, item.clone(), key, true) {
                        return Some(span);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Within the byte range `region` of `source`, find the `key: value` line whose key
/// matches `key` and return the **value** span (after `key:` + one space, to the
/// line's end, trailing whitespace excluded). `bullet` strips a leading `- ` marker
/// before reading the key.
fn field_value_in_lines(
    source: &str,
    region: Range<usize>,
    key: &str,
    bullet: bool,
) -> Option<Range<usize>> {
    let text = &source[region.clone()];
    let mut line_start = region.start;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_end_matches('\n');
        // The byte offset of the line's content within the source.
        let mut cursor = line_start;
        let mut rest = trimmed;
        if bullet {
            // Strip a leading `- ` (or `-`) bullet marker, advancing the cursor.
            if let Some(stripped) = rest.strip_prefix("- ") {
                cursor += rest.len() - stripped.len();
                rest = stripped;
            } else if let Some(stripped) = rest.strip_prefix('-') {
                cursor += rest.len() - stripped.len();
                rest = stripped;
            }
        }
        // Leading whitespace before the key.
        let key_start_trim = rest.len() - rest.trim_start().len();
        cursor += key_start_trim;
        let bare = rest.trim_start();
        if let Some((line_key, after_colon)) = bare.split_once(':')
            && line_key.trim() == key
        {
            // The value starts after the colon and one optional separating space,
            // matching the canonical `key: value` and the trimmed read value.
            let colon_off = line_key.len() + 1; // up to and including the ':'
            let mut value_start = cursor + colon_off;
            let leading = after_colon.len() - after_colon.trim_start().len();
            value_start += leading;
            let value_end = value_start + after_colon.trim().len();
            return Some(value_start..value_end);
        }
        line_start += line.len();
    }
    None
}

/// Locate the whole byte span of the repeatable item whose `{#id}` anchor is `id` —
/// from its `### …{#id}` heading start to the start of the next `###` item heading or
/// the next `##` section heading (or EOF), **including** the trailing blank-line
/// separator so the deletion leaves the surrounding items canonically spaced.
fn locate_item_block(source: &str, id: &str) -> Option<Range<usize>> {
    let blocks = parse::scan_blocks(source);
    // The target item's `### …{#id}` heading start, located by its anchor.
    let mut target_start: Option<usize> = None;
    for block in &blocks {
        if let Block::Heading {
            level: pulldown_cmark::HeadingLevel::H3,
            range,
            ..
        } = block
            && anchor_of(&source[range.clone()]) == Some(id)
        {
            target_start = Some(range.start);
        }
    }
    let start = target_start?;

    // The block end: the next `###` item or `##` section heading after `start`, else
    // EOF. We extend through the trailing blank line(s) up to that next boundary so
    // the remaining items stay separated by exactly one blank line.
    let next_boundary = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level: pulldown_cmark::HeadingLevel::H3 | pulldown_cmark::HeadingLevel::H2,
                range,
                ..
            } if range.start > start => Some(range.start),
            _ => None,
        })
        .min()
        .unwrap_or(source.len());

    Some(start..next_boundary)
}

/// Re-scan a `### …` heading's raw source for its `{#id}` anchor, returning the inner
/// slug if present and well-formed (mirrors the parser's anchor read). Used only to
/// match an item by id; malformed anchors won't match any present item.
fn anchor_of(raw: &str) -> Option<&str> {
    let first_line = raw.lines().next()?;
    let (_, after) = first_line.split_once("{#")?;
    let (inner, _) = after.split_once('}')?;
    if inner.is_empty() { None } else { Some(inner) }
}

/// A scalar field value as the canonical `key: value` form's value text (used by
/// callers building a replacement). A [`Value::List`] renders inline-flow; a scalar
/// is its literal text. Kept here so `set-field` callers can pass a [`Value`] without
/// reaching into [`crate::field_block`].
pub fn value_text(value: &Value) -> String {
    match value {
        Value::Scalar(s) => s.clone(),
        Value::List(items) => format!("[{}]", items.join(", ")),
    }
}

#[cfg(test)]
mod splice {
    //! The surgical-splice contract — the #1-risk round-trip on the **edit** side
    //! (`parsing.md` → Round-trip guarantees clause 2: surgical on edits). Golden
    //! (insta): change one field/slot, assert only that target's bytes differ.
    //! Property (proptest): parse an arbitrary conformant doc → splice one target →
    //! assert the byte-diff is confined to exactly the located span.

    use super::*;
    use crate::parse::parse_sections;
    use crate::schema::{Schema, load_schema};

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    fn spec_schema() -> Schema {
        let yaml = b"\
type: spec
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"x\" } }
        - { id: maps-to-test, type: code-anchor }
";
        load_schema(yaml).expect("spec schema loads")
    }

    /// A canonical ADR fixture (the frozen byte form).
    const CANONICAL_ADR: &str = "\
---
status: proposed
date: 2026-05-23
---

# Rate-limit at the gateway

## Context

Per-client limits were enforced ad hoc.

## Decision

Centralize rate limiting at the gateway.

## Consequences

Each service drops its local limiter.
";

    /// Assert that exactly the lines `changed` differ between `before` and `after`,
    /// every other line byte-identical, and the two have the same line count. The
    /// surgical-on-edit contract at line granularity.
    fn assert_only_lines_differ(before: &str, after: &str, changed: &[usize]) {
        let b: Vec<&str> = before.split_inclusive('\n').collect();
        let a: Vec<&str> = after.split_inclusive('\n').collect();
        assert_eq!(b.len(), a.len(), "line count must be unchanged");
        for (i, (bl, al)) in b.iter().zip(a.iter()).enumerate() {
            if changed.contains(&i) {
                assert_ne!(bl, al, "line {i} was expected to change");
            } else {
                assert_eq!(bl, al, "line {i} must be byte-identical");
            }
        }
    }

    /// Golden + surgical: `set-field` of `status` on a canonical ADR changes only
    /// the `status:` value bytes. The snapshot pins the full result; the line check
    /// asserts only line 1 (`status: …`) differs.
    #[test]
    fn set_field_changes_only_the_field_value() {
        let out = set_field(
            &adr_schema(),
            CANONICAL_ADR,
            "status",
            "status",
            "superseded",
        )
        .expect("status field present");
        insta::assert_snapshot!("set_field_status", out);
        // Line index 1 is `status: proposed` (line 0 is the `---` fence).
        assert_only_lines_differ(CANONICAL_ADR, &out, &[1]);
        assert!(out.contains("status: superseded"));
    }

    /// Surgical on a **non-canonically-spaced** but conformant input: a file with
    /// extra blank lines and two spaces after a field colon keeps every unedited
    /// byte identical when one field changes. The concrete "files are truth" claim.
    #[test]
    fn set_field_preserves_noncanonical_spacing() {
        // Two spaces after `date:`, an extra blank line before `## Decision`.
        let noncanon = "\
---
status: proposed
date:  2026-05-23
---

# A decision


## Context

Forces.

## Decision

We decided.

## Consequences

Fine.
";
        let out = set_field(&adr_schema(), noncanon, "status", "status", "accepted")
            .expect("status present");
        // Only the status value changed; the odd `date:  ` spacing and the double
        // blank line are preserved verbatim everywhere else.
        assert_only_lines_differ(noncanon, &out, &[1]);
        assert!(out.contains("date:  2026-05-23"), "odd spacing preserved");
        assert!(out.contains("\n\n\n## Context"), "double blank preserved");
    }

    /// `set-slot`: replacing the `decision` slot prose changes only that slot's
    /// bytes; the front-matter, headings, and the other two slots are untouched.
    #[test]
    fn set_slot_changes_only_the_slot_prose() {
        let out = set_slot(
            &adr_schema(),
            CANONICAL_ADR,
            "decision",
            "Adopt a token bucket per client.",
        )
        .expect("decision slot present");
        insta::assert_snapshot!("set_slot_decision", out);
        // The decision prose line is line index 13.
        assert_only_lines_differ(CANONICAL_ADR, &out, &[13]);
        assert!(out.contains("Adopt a token bucket per client."));
        assert!(!out.contains("Centralize rate limiting"));
    }

    /// `remove-item`: deleting the first of two repeatable items removes exactly its
    /// block (heading → next item) and leaves the surviving item byte-intact and
    /// canonically spaced (no leading blank, no double blank).
    #[test]
    fn remove_item_deletes_only_the_item_block() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Rate limit holds  {#rate-limit}

The gateway rejects the 101st request.

### Burst allowance  {#burst-allowance}

A short burst is tolerated.
";
        let out = remove_item(&spec_schema(), src, "criteria", "rate-limit").expect("item present");
        insta::assert_snapshot!("remove_item_first", out);
        // The surviving item parses and is the only one left.
        let doc = parse_sections(&spec_schema(), &out).expect("result still conforms");
        let criteria = doc.sections.iter().find(|s| s.id == "criteria").unwrap();
        assert_eq!(criteria.items.len(), 1);
        assert_eq!(criteria.items[0].id, "burst-allowance");
        // The surviving item's heading + prose survive verbatim.
        assert!(out.contains("### Burst allowance  {#burst-allowance}"));
        assert!(out.contains("A short burst is tolerated."));
        assert!(!out.contains("rate-limit"));
    }

    /// An absent target routes to generation: a `set-field` for a field not present
    /// (no `supersedes` in the fixture) is [`SpliceError::NotPresent`], not an edit.
    #[test]
    fn absent_field_is_not_present() {
        let err = set_field(
            &adr_schema(),
            CANONICAL_ADR,
            "status",
            "supersedes",
            "adr:x",
        )
        .expect_err("supersedes is absent");
        assert!(matches!(err, SpliceError::NotPresent { .. }));
    }

    /// The pure primitive: `splice` replaces exactly `[start,end)` and copies the
    /// rest verbatim.
    #[test]
    fn splice_primitive_replaces_only_the_span() {
        let s = "abcXXXdef";
        assert_eq!(splice(s, 3..6, "Y"), "abcYdef");
        assert_eq!(splice(s, 3..6, ""), "abcdef");
    }
}

#[cfg(test)]
mod splice_prop_tests {
    //! The #1-risk surgical-on-edit property: parse an arbitrary conformant ADR →
    //! splice a single field/slot → the byte-diff is confined to **exactly** the
    //! located target's recorded span; every other byte is unchanged.

    use super::*;
    use crate::parse::parse_sections;
    use crate::schema::{Schema, load_schema};
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    /// Heading-free, sentinel-free slot prose paragraphs (so the canonical fixture
    /// parses cleanly), joined by blank lines.
    fn prose() -> impl Strategy<Value = String> {
        let line = "[a-zA-Z][a-zA-Z0-9 .,]{0,30}";
        prop::collection::vec(line, 1..3).prop_map(|ls| ls.join("\n\n"))
    }

    /// Build a conformant canonical ADR from generated prose + status.
    fn build_adr(status: &str, context: &str, decision: &str, consequences: &str) -> String {
        format!(
            "---\nstatus: {status}\ndate: 2026-05-23\n---\n\n# A decision\n\n\
             ## Context\n\n{context}\n\n## Decision\n\n{decision}\n\n\
             ## Consequences\n\n{consequences}\n"
        )
    }

    /// Assert the byte-diff between `before` and `after` is confined to exactly
    /// `span` in `before`. The prefix `[..span.start)` is byte-identical at the head
    /// of `after`, and the suffix `[span.end..)` is byte-identical at its tail — which
    /// together prove every byte *outside* the span is untouched, leaving only the
    /// span's bytes free to differ (the surgical-on-edit contract). The replacement
    /// occupies exactly `after.len() - prefix.len() - suffix.len()` bytes between them.
    fn assert_diff_confined_to(before: &str, after: &str, span: std::ops::Range<usize>) {
        let prefix = &before[..span.start];
        let suffix = &before[span.end..];
        assert!(after.starts_with(prefix), "prefix bytes must be intact");
        assert!(after.ends_with(suffix), "suffix bytes must be intact");
        assert!(
            after.len() >= prefix.len() + suffix.len(),
            "prefix and suffix must not overlap (the diff stays inside the span)"
        );
    }

    proptest! {
        /// Surgical on a `set-field`: changing `status` over an arbitrary conformant
        /// ADR confines the diff to exactly the `status:` value bytes.
        #[test]
        fn set_field_diff_confined_to_value(
            from in prop::sample::select(vec!["proposed", "accepted", "superseded"]),
            to in prop::sample::select(vec!["proposed", "accepted", "superseded"]),
            context in prose(),
            decision in prose(),
            consequences in prose(),
        ) {
            let src = build_adr(from, &context, &decision, &consequences);
            let schema = adr_schema();
            let value_span = locate_field_value(&src, "status").expect("status line located");
            let out = set_field(&schema, &src, "status", "status", to).expect("status present");
            assert_diff_confined_to(&src, &out, value_span.clone());
            prop_assert_eq!(&src[value_span.clone()], from);
            prop_assert_eq!(&out[value_span.start..value_span.start + to.len()], to);
        }

        /// Surgical on a `set-slot`: changing the `decision` slot over an arbitrary
        /// conformant ADR confines the diff to exactly the recorded slot span.
        #[test]
        fn set_slot_diff_confined_to_span(
            context in prose(),
            decision in prose(),
            consequences in prose(),
            new_prose in prose(),
        ) {
            let src = build_adr("proposed", &context, &decision, &consequences);
            let schema = adr_schema();
            let doc = parse_sections(&schema, &src).expect("conformant ADR parses");
            let span = doc.sections.iter().find(|s| s.id == "decision")
                .unwrap().slot.as_ref().unwrap().clone();
            let out = set_slot(&schema, &src, "decision", &new_prose).expect("decision present");
            assert_diff_confined_to(&src, &out, span.start..span.end);
            prop_assert_eq!(&out[span.start..span.start + new_prose.len()], new_prose.as_str());
        }
    }

    /// A non-prop scalar/list value-text rendering check (kept with the property
    /// module so its helper is exercised).
    #[test]
    fn value_text_renders_scalar_and_list() {
        assert_eq!(value_text(&Value::Scalar("x".into())), "x");
        assert_eq!(
            value_text(&Value::List(vec!["a".into(), "b".into()])),
            "[a, b]"
        );
    }
}
