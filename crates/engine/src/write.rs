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

use serde::{Deserialize, Serialize};

use crate::field_block::{self, Field, FieldBlock};
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
