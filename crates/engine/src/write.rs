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
use pulldown_cmark::HeadingLevel;

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

/// First-touch canonicalization — the **no-op write** of the round-trip contract.
///
/// When the CLI first touches an existing on-disk doc (copies it into the task
/// working area), it applies the *only* permitted first-touch canonicalizations and
/// leaves every other byte intact (`parsing.md` → Round-trip guarantees;
/// `DECISIONS.md` 2026-05-31 → Round-trip canonicalization ledger):
///
/// - **BOM** — a leading `\u{feff}` is stripped (it breaks front-matter detection;
///   never re-emitted).
/// - **Trailing newline** — normalized to exactly one, **EOL-preserving** (a CRLF
///   file ends in one `\r\n`, an LF file in one `\n`).
/// - **Everything else preserved byte-for-byte** — EOL is *not* normalized (matched
///   locally, never globally rewritten); interior whitespace and prose are never
///   reflowed.
///
/// This is the oracle the no-op-write fuzz asserts against: `canonicalize(doc)` is
/// `doc` modulo exactly these two changes — nothing line-spanning is rewritten.
pub fn first_touch_canonicalize(source: &str) -> String {
    // BOM: strip a single leading byte-order mark.
    let body = source.strip_prefix('\u{feff}').unwrap_or(source);

    // EOL: preserve the file's existing ending. A file containing any `\r\n` is a
    // CRLF file; otherwise LF (and a new/empty file defaults to LF).
    let eol = if body.contains("\r\n") { "\r\n" } else { "\n" };

    // Trailing newline: strip all trailing `\r`/`\n`, then re-append exactly one EOL
    // unit — EOL-preserving, no global normalization. An empty body stays empty.
    let trimmed = body.trim_end_matches(['\r', '\n']);
    if trimmed.is_empty() {
        return String::new();
    }
    let mut out = String::with_capacity(trimmed.len() + eol.len());
    out.push_str(trimmed);
    out.push_str(eol);
    out
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
    /// matter (`type`), a single blank line, the `# H1`, the `## Summary` and
    /// `## Body` slot sections with their prose, the empty `## Trailers` repeatable
    /// section, exactly one trailing newline. Pins the no-field-group case (the slot
    /// sections have no fields, so no sentinel).
    #[test]
    fn commit_instance_renders_canonical_bytes() {
        let instance = Instance {
            title: "Add the rate limiter".to_string(),
            sections: vec![
                SectionContent {
                    id: "header".to_string(),
                    fields: vec![scalar("type", "feat")],
                    ..Default::default()
                },
                SectionContent {
                    id: "summary".to_string(),
                    slot: Some("Add a per-client rate limit at the gateway.".to_string()),
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

// ============================================================================
// The commit string sink — render a `commit` instance to a git-message string.
// The canonical writer's sixth caller (`parsing.md` → The write pipeline): not a
// file, a git-commit message. Schema-driven (the commit schema's section roles
// drive the mapping) and deterministic — a pure function of `(schema, instance)`,
// no disk side effects. See `design/finalize.md` → Commit-doc rendering.
// ============================================================================

/// Render a `commit` [`Instance`] against its `schema` into a git-message string.
///
/// The mapping (`design/finalize.md` → Commit-doc rendering, the pack-default
/// Conventional-Commits shape):
///
/// - header `type` field → subject `<type>`; header `scope` field →
///   subject `(<scope>)`, **omitted entirely when empty** (no empty parens);
/// - the `summary` slot → the subject text after `: `;
/// - the `body` slot → body paragraph(s), preceded by one blank line, skipped
///   when empty;
/// - the `trailers` repeatable section → footer `key: value` lines, preceded by
///   one blank line, skipped when empty.
///
/// Schema-driven: the section roles are read from `schema` by their well-known
/// ids (the pack-default commit schema declares `header`/`summary`/`body`/
/// `trailers`); the renderer reads only sections the schema declares. Returns a
/// string with no trailing whitespace and no trailing newline (the git-message
/// is consumed via `-F`, not appended to a file). Deterministic — same
/// `(schema, instance)` in → identical string out.
pub fn render_commit_message(schema: &Schema, instance: &Instance) -> String {
    let header_id = schema
        .sections
        .iter()
        .find(|s| s.header)
        .map(|s| s.id.as_str());
    let header = header_id.and_then(|id| section_content(instance, id));

    let type_text = header
        .and_then(|c| commit_field(c, "type"))
        .unwrap_or_default();
    let scope_text = header
        .and_then(|c| commit_field(c, "scope"))
        .unwrap_or_default();
    let summary_text = section_content(instance, "summary")
        .and_then(|c| c.slot.as_deref())
        .unwrap_or("")
        .trim();

    // Subject: `<type>(<scope>): <summary>`, scope-parens omitted when empty.
    let mut out = String::new();
    out.push_str(&type_text);
    if !scope_text.is_empty() {
        let _ = write!(out, "({scope_text})");
    }
    let _ = write!(out, ": {summary_text}");

    // Body: one blank-line gap, skipped when empty.
    if let Some(body) = section_content(instance, "body").and_then(|c| c.slot.as_deref()) {
        let body = body.trim();
        if !body.is_empty() {
            let _ = write!(out, "\n\n{body}");
        }
    }

    // Trailers footer: one blank-line gap, `key: value` per item, skipped when empty.
    let trailers = trailer_lines(instance);
    if !trailers.is_empty() {
        let _ = write!(out, "\n\n{}", trailers.join("\n"));
    }

    out
}

/// The scalar text of the named header field of a commit instance, if present and
/// non-empty after trimming.
fn commit_field(content: &SectionContent, key: &str) -> Option<String> {
    content
        .fields
        .iter()
        .find(|f| f.key == key)
        .map(|f| value_text(&f.value).trim().to_string())
        .filter(|s| !s.is_empty())
}

/// The trailer footer lines (`key: value`) of a commit instance, in item order.
/// Each repeatable `trailers` item carries `key` and `value` fields; an item
/// missing either contributes nothing.
fn trailer_lines(instance: &Instance) -> Vec<String> {
    let Some(section) = section_content(instance, "trailers") else {
        return Vec::new();
    };
    section
        .items
        .iter()
        .filter_map(|item| {
            let key = item_field(item, "key")?;
            let value = item_field(item, "value")?;
            Some(format!("{key}: {value}"))
        })
        .collect()
}

/// The scalar text of a repeatable item's named field, if present and non-empty.
fn item_field(item: &ItemContent, key: &str) -> Option<String> {
    item.fields
        .iter()
        .find(|f| f.key == key)
        .map(|f| value_text(&f.value).trim().to_string())
        .filter(|s| !s.is_empty())
}

// ============================================================================
// Generation-and-insert — the other half of the write path. When the structural
// home a write targets is **absent** (an optional section not yet in the file, an
// item being added, the first field of a fieldless section), there is no span to
// splice. We *generate* the canonical bytes for the new structural home (heading,
// item block, or sentinel + bullet) and **insert** them at the deterministic
// schema-document-order position (`parsing.md` → The write pipeline → "Absent
// structural homes generate at their schema-ordered position"). Generation scope is
// the structural home only — the heading plus the one leaf being written;
// sibling slots / a field group materialize incrementally when *their* writes land.
// ============================================================================

/// A located generation-and-insert failure: the target's structural home is already
/// present (so the caller should have routed to the *surgical splice* path), or the
/// source does not conform enough to locate the schema-ordered insertion point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenerateError {
    /// The section / item the caller asked to generate is **already present** — a
    /// present target is a surgical splice ([`set_slot`] / [`add_item`] of a new id),
    /// not a generation. Routes the caller back to the edit path.
    AlreadyPresent {
        /// A human-readable description of what was already present.
        what: String,
    },
    /// The named section is not declared by the schema, so there is no schema-ordered
    /// position to insert at.
    UnknownSection {
        /// The unknown section id.
        id: String,
    },
    /// The section's shape does not match the requested generation (e.g. `add_item`
    /// into a simple section, or `generate_section` for the header).
    WrongShape {
        /// A human-readable description of the mismatch.
        what: String,
    },
}

/// `set-slot` (section **absent**): materialize the absent body section `section_id`'s
/// structural home at its schema-document-order position and write `slot` prose (plus
/// any `fields`) into it. The generated `## Heading` is inserted **after the nearest
/// preceding present section, before the nearest following one**, so the present
/// sections stay in schema order (`parsing.md` → Absent structural homes). A section
/// already present → [`GenerateError::AlreadyPresent`] (route to [`set_slot`]).
pub fn generate_section(
    schema: &Schema,
    source: &str,
    section_id: &str,
    slot: Option<&str>,
    fields: &[Field],
) -> Result<String, GenerateError> {
    let section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
        })?;
    if section.header {
        return Err(GenerateError::WrongShape {
            what: format!(
                "section {section_id:?} is the header (front-matter), not a body section"
            ),
        });
    }

    let present = present_body_sections(schema, source);
    if present.iter().any(|(id, _)| id == section_id) {
        return Err(GenerateError::AlreadyPresent {
            what: format!("section {section_id:?}"),
        });
    }

    // The generated structural home: the canonical section block (heading + prose +
    // optional field group), no surrounding blank lines (the insertion adds those).
    let block = render_generated_section(section, slot, fields);
    let at = insertion_offset(schema, source, section_id, &present);
    Ok(insert_block(source, at, &block))
}

/// `add-item` (repeatable section): mint a `{#id}` from `title` via [`crate::slug`],
/// generate the `### <title>  {#id}` item block (slot prose + optional fields), and
/// insert it at the **end of the section's existing items** (append order; `reorder`
/// is a separate verb). If the repeatable section's own `##` home is absent, it is
/// materialized first at its schema-ordered position, then the item generated into it.
/// A minted id colliding with a present item → [`GenerateError::AlreadyPresent`].
pub fn add_item(
    schema: &Schema,
    source: &str,
    section_id: &str,
    title: &str,
    slot: Option<&str>,
    fields: &[Field],
) -> Result<String, GenerateError> {
    let section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
        })?;
    if !matches!(section.body, SectionBody::Repeatable { .. }) {
        return Err(GenerateError::WrongShape {
            what: format!("section {section_id:?} is not repeatable"),
        });
    }

    // Mint the item anchor from the id-source (the title) via slugify.
    let id = crate::slug::slugify(title);

    let item = render_item(&ItemContent {
        id: id.clone(),
        title: title.to_string(),
        slot: slot.map(str::to_string),
        fields: fields.to_vec(),
    });

    let present = present_body_sections(schema, source);
    if present.iter().any(|(sid, _)| sid == section_id) {
        // The section is present: append the item after its last present item (or
        // right after the heading if it has none yet).
        let blocks = parse::scan_blocks(source);
        let region = section_region(&blocks, source, section_id, &present).ok_or_else(|| {
            GenerateError::WrongShape {
                what: format!("section {section_id:?} region not locatable"),
            }
        })?;
        // Reject a minted id that collides with a present item in this section.
        if item_anchor_present(&blocks, source, region.clone(), &id) {
            return Err(GenerateError::AlreadyPresent {
                what: format!("item {id:?} in section {section_id:?}"),
            });
        }
        let at = last_item_end(source, region);
        Ok(insert_block(source, at, &item))
    } else {
        // The section's home is absent: generate the `## Heading` with the first item
        // as its body, inserted at the section's schema-ordered position.
        let block = format!("## {}\n\n{}", heading_text(&section.id), item);
        let at = insertion_offset(schema, source, section_id, &present);
        Ok(insert_block(source, at, &block))
    }
}

/// `set-field` (field **absent**, section present): insert the field bullet for
/// `field` into `section_id`'s trailing field group, materializing the
/// `<!-- fields -->` sentinel **once** if the section has no field group yet (the
/// "first field into a fieldless section" case). The bullet is appended after the
/// section's present bullets, in physical order. A field key already present →
/// [`GenerateError::AlreadyPresent`] (route to [`set_field`]).
pub fn insert_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    field: &Field,
) -> Result<String, GenerateError> {
    let _section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
        })?;

    let present = present_body_sections(schema, source);
    let blocks = parse::scan_blocks(source);
    let region = section_region(&blocks, source, section_id, &present).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!("section {section_id:?} not present"),
        }
    })?;

    let bullet = format!("- {}", emit_one_field(field));

    // Is there an existing field group (a sentinel + list) in the section's region?
    match field_group_list(&blocks, region.clone()) {
        Some((list_range, items)) => {
            // The field key must not already be present (that is a surgical set-field).
            if field_key_in_list(&blocks, source, &items, &field.key) {
                return Err(GenerateError::AlreadyPresent {
                    what: format!("field {:?} in section {section_id:?}", field.key),
                });
            }
            // Append the bullet right after the last present bullet (the list end).
            let at = list_range.end;
            Ok(insert_after_line(source, at, &bullet))
        }
        None => {
            // No field group yet: materialize the sentinel + this first bullet at the
            // end of the section's slot prose (the region's trimmed end).
            let at = slot_prose_end(source, region);
            let group = format!("\n\n{FIELD_SENTINEL}\n{bullet}");
            Ok(insert_at(source, at, &group))
        }
    }
}

/// Render a body section's canonical block (`## Heading` + slot prose + optional
/// field group), with **no** surrounding blank lines — the caller's insertion adds
/// the separating blanks. Mirrors [`render_section`]'s simple-section body but takes
/// the slot/fields directly (the generation caller supplies them, not an `Instance`).
fn render_generated_section(section: &Section, slot: Option<&str>, fields: &[Field]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "## {}", heading_text(&section.id));
    out.push('\n');
    out.push_str(slot.unwrap_or("").trim_end());
    append_field_group(&mut out, fields);
    // Drop the single trailing `\n` `append_field_group` leaves so the block is bare;
    // `insert_block` owns the surrounding blank lines.
    while out.ends_with('\n') {
        out.pop();
    }
    out
}

/// The present body sections, as `(schema id, heading start offset)` pairs in source
/// order. A present heading is an `## H2` whose text matches a schema **body** section
/// id (case-insensitive). Built without requiring a clean conformant parse — the
/// target section is, by construction, absent.
fn present_body_sections(schema: &Schema, source: &str) -> Vec<(String, usize)> {
    let body_ids: Vec<&str> = schema
        .sections
        .iter()
        .filter(|s| !s.header)
        .map(|s| s.id.as_str())
        .collect();
    let mut present = Vec::new();
    for block in parse::scan_blocks(source) {
        if let Block::Heading {
            level: pulldown_cmark::HeadingLevel::H2,
            text,
            range,
            ..
        } = &block
            && let Some(id) = body_ids
                .iter()
                .find(|id| text.trim().eq_ignore_ascii_case(id.trim()))
        {
            present.push((id.to_string(), range.start));
        }
    }
    present
}

/// The byte offset at which to insert an absent section's generated home so the
/// present sections stay in **schema document order**: the heading start of the
/// nearest *following* present section (the first present section whose schema index
/// is greater than the target's), else end-of-document.
fn insertion_offset(
    schema: &Schema,
    source: &str,
    section_id: &str,
    present: &[(String, usize)],
) -> usize {
    let order = |id: &str| schema.sections.iter().position(|s| s.id == id);
    let target_idx = order(section_id);
    present
        .iter()
        .filter(|(id, _)| order(id) > target_idx)
        .map(|(_, start)| *start)
        .min()
        .unwrap_or(source.len())
}

/// Insert a bare section/item `block` at `at` (a section-heading boundary or EOF),
/// surrounding it with exactly one blank line on each adjoining side so the result is
/// canonically spaced. At a following heading's start, the block precedes it
/// (`block\n\n`); at EOF, the block follows the last section (`\nblock\n`).
fn insert_block(source: &str, at: usize, block: &str) -> String {
    if at >= source.trim_end().len() {
        // Append at end-of-document: one blank line, the block, one trailing newline.
        let head = source.trim_end();
        return format!("{head}\n\n{block}\n");
    }
    // Insert before a following heading: the block then one blank line then the head.
    splice(source, at..at, &format!("{block}\n\n"))
}

/// The byte region `[content_start, region_end)` of a present body section
/// `section_id`: from just past its `##` heading to the next `##`/EOF.
fn section_region(
    blocks: &[Block],
    source: &str,
    section_id: &str,
    present: &[(String, usize)],
) -> Option<Range<usize>> {
    let &(_, head_start) = present.iter().find(|(id, _)| id == section_id)?;
    let content_start = blocks.iter().find_map(|b| match b {
        Block::Heading {
            level: pulldown_cmark::HeadingLevel::H2,
            range,
            content_start,
            ..
        } if range.start == head_start => Some(*content_start),
        _ => None,
    })?;
    let end = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level: pulldown_cmark::HeadingLevel::H2,
                range,
                ..
            } if range.start > head_start => Some(range.start),
            _ => None,
        })
        .min()
        .unwrap_or(source.len());
    Some(content_start..end)
}

/// The byte offset at the end of a repeatable section's items: the section region's
/// content, back-trimmed to just past the last item's last non-blank byte — where a
/// new `### …` item block appends. With no items yet, the section's content start
/// (just past its heading), back-trimmed.
fn last_item_end(source: &str, region: Range<usize>) -> usize {
    region.start + source[region.clone()].trim_end().len()
}

/// Whether a `### …{#id}` item with anchor `id` is already present in `region`.
fn item_anchor_present(blocks: &[Block], source: &str, region: Range<usize>, id: &str) -> bool {
    blocks.iter().any(|b| match b {
        Block::Heading {
            level: pulldown_cmark::HeadingLevel::H3,
            range,
            ..
        } if range.start >= region.start && range.start < region.end => {
            anchor_of(&source[range.clone()]) == Some(id)
        }
        _ => false,
    })
}

/// The trailing-field-group bullet list in a section `region`, if present: a
/// `<!-- fields -->` sentinel immediately followed by a top-level list. Returns the
/// list's byte range and its per-item ranges.
fn field_group_list(
    blocks: &[Block],
    region: Range<usize>,
) -> Option<(Range<usize>, Vec<Range<usize>>)> {
    let sentinel = blocks.iter().find_map(|b| match b {
        Block::FieldSentinel { range }
            if range.start >= region.start && range.start < region.end =>
        {
            Some(range.clone())
        }
        _ => None,
    })?;
    blocks.iter().find_map(|b| match b {
        Block::List { range, items } if range.start >= sentinel.end && range.start < region.end => {
            Some((range.clone(), items.clone()))
        }
        _ => None,
    })
}

/// Whether `key` is already a bullet key in the field-group `items`.
fn field_key_in_list(_blocks: &[Block], source: &str, items: &[Range<usize>], key: &str) -> bool {
    items.iter().any(|item| {
        let raw = source[item.clone()].trim_end();
        let bare = raw
            .strip_prefix("- ")
            .or_else(|| raw.strip_prefix('-'))
            .unwrap_or(raw)
            .trim_start();
        bare.split_once(':')
            .map(|(k, _)| k.trim() == key)
            .unwrap_or(false)
    })
}

/// The byte offset where a simple section's slot prose ends (its region content,
/// back-trimmed) — where a first `<!-- fields -->` field group materializes.
fn slot_prose_end(source: &str, region: Range<usize>) -> usize {
    region.start + source[region.clone()].trim_end().len()
}

/// Insert `text` verbatim at byte offset `at` (no spacing added — the caller owns it).
fn insert_at(source: &str, at: usize, text: &str) -> String {
    splice(source, at..at, text)
}

/// Insert `bullet` as a new line immediately after the list ending at `at` (the list
/// range's end sits just past the last bullet's newline), preserving the contiguous
/// bullet block.
fn insert_after_line(source: &str, at: usize, bullet: &str) -> String {
    // The list range ends after the last bullet's content; ensure we land just past
    // its terminating newline so the new bullet is its own line.
    let mut pos = at;
    if !source[..pos].ends_with('\n') {
        // Advance to the end of the current line.
        if let Some(nl) = source[pos..].find('\n') {
            pos += nl + 1;
        } else {
            return format!("{source}\n{bullet}");
        }
    }
    splice(source, pos..pos, &format!("{bullet}\n"))
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

#[cfg(test)]
mod generate {
    //! The generation-and-insert contract — the write path for **absent** structural
    //! homes (`parsing.md` → The write pipeline → "Absent structural homes generate at
    //! their schema-ordered position"). Goldens (insta) pin the exact generated +
    //! inserted bytes: a `set-slot` into an absent optional section materializes its
    //! `##` home in schema document-order; an `add-item` mints a `{#id}` from the
    //! id-source via [`crate::slug`] and inserts the item block; a first `set-field`
    //! into a fieldless section emits the sentinel + bullet once.

    use super::*;
    use crate::field_block::Value;
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

    /// A schema with a body section (`detail`) that declares both a slot and a typed
    /// field — the fixture for the "first set-field into a fieldless section" case
    /// (the MVP `adr`/`commit` body sections declare no body fields).
    fn fielded_schema() -> Schema {
        let yaml = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: detail
    slot: { hint: \"x\" }
    fields:
      - { id: owner, type: string }
";
        load_schema(yaml).expect("fielded schema loads")
    }

    fn scalar(key: &str, value: &str) -> Field {
        Field {
            key: key.to_string(),
            value: Value::Scalar(value.to_string()),
        }
    }

    /// Golden: a `set-slot` into an **absent** optional ADR section (`decision`
    /// missing from the file) materializes the `## Decision` home at its
    /// schema-document-order position — between the present `## Context` and
    /// `## Consequences` — and writes the slot prose. The present sections stay in
    /// schema order; the snapshot pins the exact inserted bytes (canonical spacing).
    #[test]
    fn set_slot_into_absent_section_materializes_home_in_schema_order() {
        let src = "\
---
status: proposed
date: 2026-05-23
---

# Rate-limit at the gateway

## Context

Per-client limits were enforced ad hoc.

## Consequences

Each service drops its local limiter.
";
        let out = generate_section(
            &adr_schema(),
            src,
            "decision",
            Some("Adopt a token bucket per client."),
            &[],
        )
        .expect("decision section is absent ⇒ generated");
        insta::assert_snapshot!("set_slot_absent_section", out);

        // The result is conformant and the sections are in schema document-order.
        let doc = parse_sections(&adr_schema(), &out).expect("generated result conforms");
        let ids: Vec<&str> = doc.sections.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, ["status", "context", "decision", "consequences"]);
    }

    /// Golden: a `set-slot` into an absent section that is the **last** in schema
    /// order appends its `##` home at EOF (no following present section to insert
    /// before). Pins the EOF-append spacing (one blank line before, one trailing LF).
    #[test]
    fn set_slot_into_absent_trailing_section_appends_at_eof() {
        let src = "\
---
status: proposed
date: 2026-05-23
---

# A decision

## Context

Forces at play.

## Decision

We centralize.
";
        let out = generate_section(
            &adr_schema(),
            src,
            "consequences",
            Some("Local limiters retired."),
            &[],
        )
        .expect("consequences absent ⇒ generated");
        insta::assert_snapshot!("set_slot_absent_trailing", out);
        let doc = parse_sections(&adr_schema(), &out).expect("conforms");
        let ids: Vec<&str> = doc.sections.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, ["status", "context", "decision", "consequences"]);
    }

    /// A section already present is **not** a generation — it routes to the surgical
    /// `set_slot` edit path via [`GenerateError::AlreadyPresent`].
    #[test]
    fn set_slot_into_present_section_is_already_present() {
        let src = "\
---
status: proposed
date: 2026-05-23
---

# A decision

## Context

Forces.

## Decision

We decided.

## Consequences

Fine.
";
        let err = generate_section(&adr_schema(), src, "context", Some("x"), &[])
            .expect_err("context is present");
        assert!(matches!(err, GenerateError::AlreadyPresent { .. }));
    }

    /// Golden: `add-item` mints a `{#id}` from the id-source title via slugify and
    /// inserts the generated `### <title>  {#id}` item block (two-space anchor gap),
    /// with its slot prose and a sentinelled per-item field, appended after the
    /// section's existing item. The snapshot pins the slugified anchor
    /// (`Add a rate limiter` → `add-a-rate-limiter`) and the exact item bytes.
    #[test]
    fn add_item_mints_anchor_and_inserts_block() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Burst allowance  {#burst-allowance}

A short burst is tolerated.
";
        let out = add_item(
            &spec_schema(),
            src,
            "criteria",
            "Add a rate limiter",
            Some("The gateway rejects the 101st request."),
            &[scalar("maps-to-test", "`test/rate_limit_spec.rb#burst`")],
        )
        .expect("item generated");
        insta::assert_snapshot!("add_item_block", out);

        // The minted anchor is the slugified title, and the result parses to two items.
        assert!(out.contains("### Add a rate limiter  {#add-a-rate-limiter}"));
        let doc = parse_sections(&spec_schema(), &out).expect("result conforms");
        let criteria = doc.sections.iter().find(|s| s.id == "criteria").unwrap();
        let ids: Vec<&str> = criteria.items.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(ids, ["burst-allowance", "add-a-rate-limiter"]);
    }

    /// `add-item` whose minted id collides with a present item is
    /// [`GenerateError::AlreadyPresent`] — serial collisions reject (MVP;
    /// `write-commands.md`), the numeric suffix is the post-MVP parallel case.
    #[test]
    fn add_item_colliding_anchor_is_already_present() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Rate limit  {#rate-limit}

Holds at 100/min.
";
        let err = add_item(
            &spec_schema(),
            src,
            "criteria",
            "Rate limit",
            Some("x"),
            &[],
        )
        .expect_err("rate-limit anchor collides");
        assert!(matches!(err, GenerateError::AlreadyPresent { .. }));
    }

    /// Golden: a first `set-field` into a **fieldless** section (a `detail` section
    /// whose declared `owner` field has no value on disk yet) materializes the
    /// `<!-- fields -->` sentinel **once** plus the first `- owner: …` bullet, at the
    /// end of the section's slot prose. The snapshot pins the exact sentinel + bullet
    /// bytes (one blank line before the sentinel, the bullet contiguous beneath it).
    #[test]
    fn first_set_field_emits_sentinel_and_bullet_once() {
        let src = "\
---
title: A note
---

# A note

## Detail

Some opaque prose here.
";
        let out = insert_field(
            &fielded_schema(),
            src,
            "detail",
            &scalar("owner", "platform-team"),
        )
        .expect("owner field absent ⇒ generated");
        insta::assert_snapshot!("first_set_field", out);

        // Exactly one sentinel, and the result parses with the field present.
        assert_eq!(out.matches(FIELD_SENTINEL).count(), 1);
        let doc = parse_sections(&fielded_schema(), &out).expect("result conforms");
        let detail = doc.sections.iter().find(|s| s.id == "detail").unwrap();
        assert_eq!(detail.fields.len(), 1);
        assert_eq!(detail.fields[0].key, "owner");
    }

    /// A `set-field` of an absent field into a section that **already has a field
    /// group** appends a new bullet beneath the present ones (the sentinel is reused,
    /// not re-emitted), so the contiguous bullet block grows by one line.
    #[test]
    fn set_field_absent_into_existing_group_appends_a_bullet() {
        // A `note` schema whose `detail` section declares two fields; only `owner`
        // is present on disk, so adding `area` appends a second bullet.
        let yaml = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: detail
    slot: { hint: \"x\" }
    fields:
      - { id: owner, type: string }
      - { id: area, type: string }
";
        let schema = load_schema(yaml).expect("schema loads");
        let src = "\
---
title: A note
---

# A note

## Detail

Prose.

<!-- fields -->
- owner: platform-team
";
        let out = insert_field(&schema, src, "detail", &scalar("area", "gateway"))
            .expect("area field absent ⇒ generated");
        // The sentinel is reused (still exactly one) and both bullets are present,
        // contiguous beneath it.
        assert_eq!(out.matches(FIELD_SENTINEL).count(), 1);
        assert!(out.contains("- owner: platform-team\n- area: gateway"));
        let doc = parse_sections(&schema, &out).expect("result conforms");
        let detail = doc.sections.iter().find(|s| s.id == "detail").unwrap();
        let keys: Vec<&str> = detail.fields.iter().map(|f| f.key.as_str()).collect();
        assert_eq!(keys, ["owner", "area"]);
    }

    /// A `set-field` whose key is already present is **not** a generation — it routes
    /// to the surgical `set_field` edit path via [`GenerateError::AlreadyPresent`].
    #[test]
    fn set_field_already_present_routes_to_splice() {
        let src = "\
---
title: A note
---

# A note

## Detail

Prose.

<!-- fields -->
- owner: platform-team
";
        let err = insert_field(
            &fielded_schema(),
            src,
            "detail",
            &scalar("owner", "other-team"),
        )
        .expect_err("owner already present");
        assert!(matches!(err, GenerateError::AlreadyPresent { .. }));
    }
}

// ============================================================================
// Validate-after-write — the local safety gate (`parsing.md` → Validate-after-write).
// After every splice/generation, re-parse the result and assert (a) it parses, (b)
// only the intended target changed, (c) for `set-field`, the new value passes its
// declared type. On any anomaly, abort: return a blocking `Finding`, never a buffer
// to persist. This is the write-time local adjudication of `write-commands.md`,
// distinct from the `finalize` validation engine (cross-doc/ref integrity).
// ============================================================================

use crate::finding::{Finding, Location};
use crate::schema::{Field as SchemaField, FieldType};

/// Type-check a `set-field`'s new value against its declared schema [`FieldType`] —
/// the *write-time local adjudication* of [`design/write-commands.md`]: a malformed
/// date or a non-member enum is rejected **now**, with fast local feedback.
///
/// Engine-native checks: an `enum` value must be a declared member (`of`); a `date`
/// must be ISO `YYYY-MM-DD`; a `bool` must be `true`/`false`; an `int` must parse as
/// a signed integer. `string`, `ref`, and `code-anchor` accept any non-empty opaque
/// value — full `ref` resolution is a `finalize` (edge-index) concern and the
/// `code-anchor` adjudicator ships in a pack, so neither is typed here. A
/// [`Value::List`] is checked element-wise. Returns a human-readable description of
/// the malformation on failure (the [`Finding`] message the caller surfaces).
pub fn check_value(field: &SchemaField, value: &Value) -> Result<(), String> {
    match value {
        Value::Scalar(s) => check_scalar(field, s),
        Value::List(items) => {
            for item in items {
                check_scalar(field, item)?;
            }
            Ok(())
        }
    }
}

/// Type-check a single scalar value against `field`'s declared type.
fn check_scalar(field: &SchemaField, value: &str) -> Result<(), String> {
    match field.ty {
        FieldType::Enum => {
            let members = field.of.as_deref().unwrap_or(&[]);
            if members.iter().any(|m| m == value) {
                Ok(())
            } else {
                Err(format!(
                    "{value:?} is not a member of enum {:?} (allowed: {})",
                    field.id,
                    members.join(", ")
                ))
            }
        }
        FieldType::Date => {
            if is_iso_date(value) {
                Ok(())
            } else {
                Err(format!(
                    "{value:?} is not an ISO date (expected `YYYY-MM-DD`)"
                ))
            }
        }
        FieldType::Bool => {
            if value == "true" || value == "false" {
                Ok(())
            } else {
                Err(format!(
                    "{value:?} is not a bool (expected `true` or `false`)"
                ))
            }
        }
        FieldType::Int => {
            if value.parse::<i64>().is_ok() {
                Ok(())
            } else {
                Err(format!("{value:?} is not an integer"))
            }
        }
        // Non-empty opaque value; deeper adjudication is a finalize / pack concern.
        FieldType::String | FieldType::Ref | FieldType::CodeAnchor => {
            if value.is_empty() {
                Err(format!("{:?} must not be empty", field.id))
            } else {
                Ok(())
            }
        }
    }
}

/// Whether `s` is an ISO calendar date `YYYY-MM-DD` with an in-range month/day. A
/// minimal hand-rolled check (no chrono dependency) — month `01..=12`, day
/// `01..=31`; the local gate rejects obvious malformations, the canonical-date
/// authority is the CLI `set: on-create` deriver, not free user input.
fn is_iso_date(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let digits = |range: std::ops::Range<usize>| s[range].bytes().all(|b| b.is_ascii_digit());
    if !(digits(0..4) && digits(5..7) && digits(8..10)) {
        return false;
    }
    let month: u8 = s[5..7].parse().unwrap_or(0);
    let day: u8 = s[8..10].parse().unwrap_or(0);
    (1..=12).contains(&month) && (1..=31).contains(&day)
}

/// The validate-after-write gate over an already-edited buffer: re-parse `edited`
/// against the schema and assert (a) it parses, and (b) the byte-diff from `source`
/// is confined to exactly `target` (the located target span). Returns a blocking
/// [`Finding`] — and the caller persists **nothing** — on either anomaly.
///
/// (b) is checked structurally on the bytes: the prefix `source[..target.start]` and
/// the suffix `source[target.end..]` must both survive intact at the head and tail of
/// `edited` (every byte outside the target span unchanged); the replacement occupies
/// the bytes between. This is the surgical-on-edit contract, *re-verified* after the
/// write rather than trusted from the splice primitive.
pub fn validate_after(
    schema: &Schema,
    source: &str,
    edited: &str,
    target: Range<usize>,
) -> Result<(), Finding> {
    // (a) the result still parses against the schema.
    if let Err(findings) = parse::parse_sections(schema, edited) {
        let first = findings.into_iter().next();
        let detail = first
            .map(|f| f.message)
            .unwrap_or_else(|| "the edited buffer no longer conforms".to_string());
        return Err(Finding::blocking(
            "write.non-reparseable",
            format!("write rejected: result does not re-parse ({detail})"),
            Location::at(1, 1),
        ));
    }

    // (b) the byte-diff is confined to exactly the target span.
    let prefix = &source[..target.start];
    let suffix = &source[target.end..];
    let confined = edited.starts_with(prefix)
        && edited.ends_with(suffix)
        && edited.len() >= prefix.len() + suffix.len();
    if !confined {
        return Err(Finding::blocking(
            "write.target-escape",
            "write rejected: the change touched bytes outside the intended target",
            Location::at(1, 1),
        ));
    }

    Ok(())
}

/// The gated `set-field`: the full write-time local adjudication for a present-field
/// value edit. Type-checks `new_value` against the field's declared schema type (c),
/// performs the surgical splice, then runs [`validate_after`] (a + b). Returns the
/// new buffer to persist, or a blocking [`Finding`] (and **no** buffer) on any
/// anomaly — a malformed value, a non-reparseable result, or a target escape.
///
/// An absent field / section (a [`SpliceError`]) is reported as a blocking finding
/// too: at this gate the caller asked to *edit* a present field, so absence is an
/// abort (routing an absent target to the generation path is the caller's concern,
/// not the gate's).
pub fn set_field_validated(
    schema: &Schema,
    source: &str,
    section_id: &str,
    field_key: &str,
    new_value: &Value,
) -> Result<String, Finding> {
    // (c) the new value passes its declared type — *before* touching bytes.
    let field = field_schema(schema, section_id, field_key).ok_or_else(|| {
        Finding::blocking(
            "write.unknown-field",
            format!("no field {field_key:?} declared in section {section_id:?}"),
            Location::at(1, 1),
        )
    })?;
    if let Err(why) = check_value(field, new_value) {
        return Err(Finding::blocking(
            "write.malformed-value",
            format!("write rejected: {why}"),
            Location::at(1, 1),
        ));
    }

    // Locate the target span (the field's value bytes) before the splice, so the
    // validate-after diff-confinement check has the intended target to assert against.
    let target = locate_field_value(source, field_key).ok_or_else(|| {
        Finding::blocking(
            "write.not-present",
            format!("field {field_key:?} value line is not present in section {section_id:?}"),
            Location::at(1, 1),
        )
    })?;

    let edited = set_field(
        schema,
        source,
        section_id,
        field_key,
        &value_text(new_value),
    )
    .map_err(|e| splice_error_finding(&e))?;

    // (a) + (b): re-parse and only-target-changed. Abort (no buffer) on any anomaly.
    validate_after(schema, source, &edited, target)?;
    Ok(edited)
}

/// The gated `set-slot`: the full write-time local adjudication for a slot prose
/// write — the slot sibling to [`set_field_validated`]. Enforces the **slot
/// heading-depth ceiling** on `new_prose` *before* touching bytes (a `##`/`###` ATX
/// heading or any Setext heading is rejected with a located `write.slot-heading-depth`
/// / `write.slot-setext-heading` finding naming the offending line), then surgically
/// splices the present section's slot — or **generates** the section's structural home
/// when the section is absent — and runs [`validate_after`] (re-parse + only the
/// intended target changed). Returns the new buffer to persist, or a blocking
/// [`Finding`] (and **no** buffer) on any anomaly.
///
/// The ceiling check is the **write-time** enforcement site of the heading-depth
/// ceiling (`parsing.md` → Slot heading-depth ceiling, enforcement site 1): the
/// parser is the read-time site, but scanning the agent's content here gives precise,
/// fast local feedback pointing at the offending line *within the prose* (1-based),
/// so the agent retries with non-conflicting prose before any byte is written.
pub fn set_slot_validated(
    schema: &Schema,
    source: &str,
    section_id: &str,
    new_prose: &str,
) -> Result<String, Finding> {
    // The heading-depth ceiling — scanned on the standalone prose so the located line
    // is relative to the agent's content (the first violation is the surfaced block).
    if let Some(finding) = slot_ceiling_finding(new_prose) {
        return Err(finding);
    }

    // Present section ⇒ surgical splice of the located slot span; absent section ⇒
    // generate the section's structural home at its schema-ordered position.
    match set_slot(schema, source, section_id, new_prose) {
        Ok(edited) => {
            // Locate the (pre-edit) slot span as the validate-after target.
            let target = locate_slot_span(schema, source, section_id).ok_or_else(|| {
                Finding::blocking(
                    "write.not-present",
                    format!("slot in section {section_id:?} is not present"),
                    Location::at(1, 1),
                )
            })?;
            validate_after(schema, source, &edited, target)?;
            Ok(edited)
        }
        Err(SpliceError::NotPresent { .. }) => {
            // The section (or its slot) is absent: materialize its structural home and
            // write the prose. Generation is not a single-span splice, so the only
            // post-write gate is re-parse (the surgical-span check does not apply).
            let edited = generate_section(schema, source, section_id, Some(new_prose), &[])
                .map_err(|e| generate_error_finding(&e))?;
            if let Err(findings) = parse::parse_sections(schema, &edited) {
                let detail = findings
                    .into_iter()
                    .next()
                    .map(|f| f.message)
                    .unwrap_or_else(|| "the generated buffer no longer conforms".to_string());
                return Err(Finding::blocking(
                    "write.non-reparseable",
                    format!("write rejected: result does not re-parse ({detail})"),
                    Location::at(1, 1),
                ));
            }
            Ok(edited)
        }
        Err(e @ SpliceError::NotConformant) => Err(splice_error_finding(&e)),
    }
}

/// The first heading-depth-ceiling violation in standalone slot `prose`, if any — a
/// `##`/`###` ATX heading or a Setext underline, located by its **1-based line within
/// the prose**. Reuses the same block parse the read-time parser uses (never a line
/// scanner — a `## …` inside a fenced code block is correctly *not* a heading), so the
/// write-time and read-time ceiling agree. Emits a `write.*` finding (the write-path
/// envelope), distinct from the parser's `conformance.*` producer.
fn slot_ceiling_finding(prose: &str) -> Option<Finding> {
    parse::scan_blocks(prose).into_iter().find_map(|b| match b {
        Block::Heading { is_atx, line, .. } if !is_atx => Some(Finding::blocking(
            "write.slot-setext-heading",
            format!(
                "Setext heading in slot prose at line {line}; use `####` ATX depth or rephrase"
            ),
            Location::at(line, 1),
        )),
        Block::Heading { level, line, .. }
            if matches!(level, HeadingLevel::H2 | HeadingLevel::H3) =>
        {
            let depth = if level == HeadingLevel::H2 {
                "##"
            } else {
                "###"
            };
            Some(Finding::blocking(
                "write.slot-heading-depth",
                format!(
                    "heading at schema-reserved depth `{depth}` in slot prose at line {line}; \
                     use `####` or rephrase"
                ),
                Location::at(line, 1),
            ))
        }
        _ => None,
    })
}

/// Locate the byte span of `section_id`'s slot prose in `source` (the parser's
/// recorded opaque span), the validate-after target for a present-section set-slot.
fn locate_slot_span(schema: &Schema, source: &str, section_id: &str) -> Option<Range<usize>> {
    let doc = parse::parse_sections(schema, source).ok()?;
    let span = doc
        .sections
        .iter()
        .find(|s| s.id == section_id)?
        .slot
        .as_ref()?;
    Some(span.start..span.end)
}

/// Render a [`GenerateError`] as the gate's blocking [`Finding`].
fn generate_error_finding(err: &GenerateError) -> Finding {
    let (code, message) = match err {
        GenerateError::AlreadyPresent { what } => (
            "write.already-present",
            format!("write rejected: {what} is already present"),
        ),
        GenerateError::UnknownSection { id } => (
            "write.unknown-section",
            format!("write rejected: no section {id:?} declared in the schema"),
        ),
        GenerateError::WrongShape { what } => {
            ("write.wrong-shape", format!("write rejected: {what}"))
        }
    };
    Finding::blocking(code, message, Location::at(1, 1))
}

/// Find the schema [`SchemaField`] declared for `field_key` in `section_id`, across a
/// header/simple section's `fields`. Returns `None` when the field is not declared.
fn field_schema<'a>(
    schema: &'a Schema,
    section_id: &str,
    field_key: &str,
) -> Option<&'a SchemaField> {
    let section = schema.sections.iter().find(|s| s.id == section_id)?;
    let fields = match &section.body {
        SectionBody::Simple { fields, .. } => fields,
        SectionBody::Repeatable { .. } => return None,
    };
    fields.iter().find(|f| f.id == field_key)
}

/// Render a [`SpliceError`] as the gate's blocking [`Finding`].
fn splice_error_finding(err: &SpliceError) -> Finding {
    let (code, message) = match err {
        SpliceError::NotPresent { what } => (
            "write.not-present",
            format!("write rejected: {what} is not present"),
        ),
        SpliceError::NotConformant => (
            "write.non-reparseable",
            "write rejected: the source does not conform to the schema".to_string(),
        ),
    };
    Finding::blocking(code, message, Location::at(1, 1))
}

#[cfg(test)]
mod validate_after {
    //! The write-time local safety gate (`parsing.md` → Validate-after-write). After
    //! every splice/generation we **re-parse** the result and assert (a) it parses,
    //! (b) only the intended target changed, (c) for `set-field`, the new value passes
    //! its declared type — aborting (a [`Finding`], never a persisted buffer) on any
    //! anomaly. This is the *write-time local adjudication* of
    //! [`design/write-commands.md`], distinct from the `finalize` validation engine.

    use super::*;
    use crate::field_block::Value;
    use crate::schema::{Schema, load_schema};

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    /// A canonical ADR with a typed `status` enum and a `date` field present.
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

    fn scalar(v: &str) -> Value {
        Value::Scalar(v.to_string())
    }

    /// (c) Type-check, then write: a `status` enum value that is a declared member
    /// (`accepted`) passes the gate — the gate returns the new buffer, which re-parses
    /// with only the `status:` value changed.
    #[test]
    fn valid_set_field_returns_buffer_and_reparses() {
        let out = set_field_validated(
            &adr_schema(),
            CANONICAL_ADR,
            "status",
            "status",
            &scalar("accepted"),
        )
        .expect("a declared enum member passes the gate");
        // The buffer re-parses against the schema (a).
        parse::parse_sections(&adr_schema(), &out).expect("gated result re-parses");
        assert!(out.contains("status: accepted"));
        // (b) only the status value line changed.
        assert!(out.contains("date: 2026-05-23"));
        assert!(out.contains("Centralize rate limiting at the gateway."));
    }

    /// (c) A `set-field` whose new value is **not** a declared enum member is rejected
    /// with a Blocking [`Finding`] and **no** buffer — the malformed value never
    /// reaches disk. The located finding is snapshotted.
    #[test]
    fn malformed_enum_value_is_blocking_finding() {
        let finding = set_field_validated(
            &adr_schema(),
            CANONICAL_ADR,
            "status",
            "status",
            &scalar("rejected"),
        )
        .expect_err("`rejected` is not a declared status member ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        insta::assert_snapshot!(
            "malformed_enum_finding",
            serde_json::to_string_pretty(&finding).unwrap()
        );
    }

    /// (c) A `set-field` whose new `date` value is not an ISO `YYYY-MM-DD` date is
    /// rejected with a Blocking [`Finding`] and no buffer.
    #[test]
    fn malformed_date_value_is_blocking_finding() {
        let finding = set_field_validated(
            &adr_schema(),
            CANONICAL_ADR,
            "status",
            "date",
            &scalar("23rd of May"),
        )
        .expect_err("a non-ISO date ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.malformed-value");
    }

    /// The type-check primitive accepts well-formed values and rejects malformed ones,
    /// per declared [`crate::schema::FieldType`]: enum membership, ISO date, bool, int.
    /// `string`/`ref`/`code-anchor` accept any non-empty opaque value (full ref /
    /// code-anchor adjudication is a `finalize` / pack concern).
    #[test]
    fn check_value_types_each_native_kind() {
        use crate::schema::{Field as SField, FieldType};
        let field = |ty: FieldType, of: Option<Vec<String>>| SField {
            id: "f".into(),
            ty,
            of,
            default: None,
            set: None,
            to: None,
            card: None,
            inverse: None,
            inverse_card: None,
        };

        // enum
        let status = field(
            FieldType::Enum,
            Some(vec!["proposed".into(), "accepted".into()]),
        );
        assert!(check_value(&status, &scalar("accepted")).is_ok());
        assert!(check_value(&status, &scalar("rejected")).is_err());

        // date — ISO YYYY-MM-DD only
        let date = field(FieldType::Date, None);
        assert!(check_value(&date, &scalar("2026-05-31")).is_ok());
        assert!(check_value(&date, &scalar("2026-13-01")).is_err());
        assert!(check_value(&date, &scalar("not a date")).is_err());

        // bool
        let flag = field(FieldType::Bool, None);
        assert!(check_value(&flag, &scalar("true")).is_ok());
        assert!(check_value(&flag, &scalar("false")).is_ok());
        assert!(check_value(&flag, &scalar("yes")).is_err());

        // int
        let count = field(FieldType::Int, None);
        assert!(check_value(&count, &scalar("42")).is_ok());
        assert!(check_value(&count, &scalar("-7")).is_ok());
        assert!(check_value(&count, &scalar("4.2")).is_err());

        // string / ref / code-anchor: any non-empty value
        let name = field(FieldType::String, None);
        assert!(check_value(&name, &scalar("anything goes")).is_ok());
        assert!(check_value(&name, &scalar("")).is_err());
        let r = field(FieldType::Ref, None);
        assert!(check_value(&r, &scalar("adr:single-node-cache")).is_ok());
    }

    /// (b) The only-intended-target gate rejects a buffer whose byte-diff escapes the
    /// declared target span — a write that touched more than the target aborts with a
    /// Blocking [`Finding`], never persists.
    #[test]
    fn target_escape_is_blocking_finding() {
        // The located `status` value span in the canonical ADR.
        let span = locate_field_value(CANONICAL_ADR, "status").expect("status located");
        // A tampered buffer: a valid in-span replacement PLUS a stray edit to slot
        // prose elsewhere (still conformant — so the re-parse passes and the
        // only-target-changed check is what must catch the escape).
        let tampered = splice(CANONICAL_ADR, span.clone(), "accepted")
            .replace("Each service drops its local limiter.", "Tampered prose.");
        assert_ne!(
            tampered,
            splice(CANONICAL_ADR, span.clone(), "accepted"),
            "the stray edit must actually change bytes"
        );
        let finding = validate_after(&adr_schema(), CANONICAL_ADR, &tampered, span)
            .expect_err("a diff outside the target span ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.target-escape");
    }

    /// (a) The re-parse gate rejects a buffer that no longer parses — a write that
    /// produced a non-conformant result aborts with a Blocking [`Finding`].
    #[test]
    fn nonreparseable_result_is_blocking_finding() {
        // The `decision` slot span; replace its prose with a forbidden `###` heading,
        // which the parser rejects inside a slot (heading-depth ceiling).
        let doc = parse::parse_sections(&adr_schema(), CANONICAL_ADR).unwrap();
        let span = doc
            .sections
            .iter()
            .find(|s| s.id == "decision")
            .unwrap()
            .slot
            .clone()
            .unwrap();
        let target = span.start..span.end;
        let broken = splice(CANONICAL_ADR, target.clone(), "### Sneaky heading");
        let finding = validate_after(&adr_schema(), CANONICAL_ADR, &broken, target)
            .expect_err("a non-reparseable result ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.non-reparseable");
    }
}

#[cfg(test)]
mod set_slot_validated {
    //! The gated `set-slot` (`parsing.md` → Slot heading-depth ceiling: set-slot
    //! write-time enforcement site; → Validate-after-write). The slot sibling to
    //! `set_field_validated`: enforce the heading-depth ceiling on the agent's prose
    //! at write-time (a precise per-line `write.*` blocking finding naming the
    //! offending line), splice the located slot span, run [`validate_after`], and
    //! return the new buffer or a blocking [`Finding`] — persisting nothing on a
    //! block.

    use super::*;
    use crate::schema::{Schema, load_schema};

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

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

    /// The done-criterion. A clean set-slot returns a buffer differing **only** in the
    /// slot span (surgical-on-edit) and re-parses; ceiling-violating prose (a `##`
    /// ATX heading, a `###` ATX heading, a Setext underline) is each rejected with a
    /// located `write.*` blocking finding naming the offending line and persists
    /// nothing.
    #[test]
    fn set_slot_validated_gates_ceiling_and_target() {
        let schema = adr_schema();

        // --- Clean set-slot: surgical on the slot span. ---
        let target = set_slot_span(&schema, CANONICAL_ADR, "decision");
        let out = set_slot_validated(&schema, CANONICAL_ADR, "decision", "We centralize.")
            .expect("clean prose passes the gate");
        // Re-parses (a).
        parse::parse_sections(&schema, &out).expect("gated result re-parses");
        // Surgical (b): only the slot span's bytes differ — the prefix and suffix
        // around the located target survive byte-for-byte.
        assert!(out.starts_with(&CANONICAL_ADR[..target.start]));
        assert!(out.ends_with(&CANONICAL_ADR[target.end..]));
        assert_eq!(
            &out[target.start..target.start + "We centralize.".len()],
            "We centralize."
        );

        // --- Ceiling: a `## Decision` ATX heading in slot prose is rejected. ---
        let finding = set_slot_validated(
            &schema,
            CANONICAL_ADR,
            "decision",
            "Intro.\n\n## Decision\n\nMore.",
        )
        .expect_err("a `##` heading in slot prose ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.slot-heading-depth");
        // The located finding names the offending line (line 3 within the prose).
        assert_eq!(finding.location.as_ref().unwrap().line, 3);
        insta::assert_snapshot!(
            "set_slot_ceiling_finding",
            serde_json::to_string_pretty(&finding).unwrap()
        );

        // --- Ceiling: a `### x` ATX heading (item depth) is rejected. ---
        let finding = set_slot_validated(&schema, CANONICAL_ADR, "decision", "### Sneaky")
            .expect_err("a `###` heading in slot prose ⇒ abort");
        assert_eq!(finding.code, "write.slot-heading-depth");
        assert_eq!(finding.location.as_ref().unwrap().line, 1);

        // --- Ceiling: a Setext underline heading is rejected at any depth. ---
        let finding = set_slot_validated(&schema, CANONICAL_ADR, "decision", "A title\n=======")
            .expect_err("a Setext heading in slot prose ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.slot-setext-heading");
        assert_eq!(finding.location.as_ref().unwrap().line, 1);
    }

    /// Locate the `decision` slot's byte span in `source` (the parser's recorded
    /// opaque span), for the surgical-on-edit assertion.
    fn set_slot_span(schema: &Schema, source: &str, section_id: &str) -> Range<usize> {
        let doc = parse::parse_sections(schema, source).expect("source parses");
        let span = doc
            .sections
            .iter()
            .find(|s| s.id == section_id)
            .and_then(|s| s.slot.clone())
            .expect("slot located");
        span.start..span.end
    }
}

#[cfg(test)]
mod validate_after_prop_tests {
    //! The #1-risk property on validate-after: for arbitrary single `set-field` edits
    //! over conformant ADRs, the gate's accepted buffer re-parses and changed exactly
    //! the one target — validate-after never passes a write it can't re-read.

    use super::*;
    use crate::field_block::Value;
    use crate::schema::{Schema, load_schema};
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    fn prose() -> impl Strategy<Value = String> {
        let line = "[a-zA-Z][a-zA-Z0-9 .,]{0,30}";
        prop::collection::vec(line, 1..3).prop_map(|ls| ls.join("\n\n"))
    }

    fn build_adr(status: &str, context: &str, decision: &str, consequences: &str) -> String {
        format!(
            "---\nstatus: {status}\ndate: 2026-05-23\n---\n\n# A decision\n\n\
             ## Context\n\n{context}\n\n## Decision\n\n{decision}\n\n\
             ## Consequences\n\n{consequences}\n"
        )
    }

    proptest! {
        /// Validate-after never passes a write it can't re-read: an arbitrary
        /// conformant ADR, a `set-field` of `status` to a declared member, runs the
        /// gate; the accepted buffer re-parses **and** the only byte-diff is the
        /// target's value span.
        #[test]
        fn gated_set_field_reparses_and_changes_one_target(
            from in prop::sample::select(vec!["proposed", "accepted", "superseded"]),
            to in prop::sample::select(vec!["proposed", "accepted", "superseded"]),
            context in prose(),
            decision in prose(),
            consequences in prose(),
        ) {
            let src = build_adr(from, &context, &decision, &consequences);
            let schema = adr_schema();
            let span = locate_field_value(&src, "status").expect("status located");
            let out = set_field_validated(&schema, &src, "status", "status", &Value::Scalar(to.into()))
                .expect("a declared member passes the gate");
            // (a) the accepted buffer re-parses.
            parse::parse_sections(&schema, &out).expect("gated buffer re-parses");
            // (b) the byte-diff is confined to exactly the target span.
            prop_assert!(out.starts_with(&src[..span.start]));
            prop_assert!(out.ends_with(&src[span.end..]));
            prop_assert_eq!(&out[span.start..span.start + to.len()], to);
        }

        /// A `set-field` of `status` to a value that is **not** a declared member is
        /// always rejected with a Blocking [`Finding`] and never returns a buffer.
        #[test]
        fn gated_set_field_rejects_non_member(
            bad in "[a-z]{1,10}".prop_filter("not a status member", |s: &String| {
                !["proposed", "accepted", "superseded"].contains(&s.as_str())
            }),
            context in prose(),
            decision in prose(),
            consequences in prose(),
        ) {
            let src = build_adr("proposed", &context, &decision, &consequences);
            let schema = adr_schema();
            let finding = set_field_validated(&schema, &src, "status", "status", &Value::Scalar(bad))
                .expect_err("a non-member is rejected");
            prop_assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        }
    }
}

#[cfg(test)]
mod generate_prop_tests {
    //! The #1-risk property on the generation path: inserting an absent section's home
    //! then parsing always yields a document whose **present sections are in schema
    //! document-order** — regardless of which subset of sections was already present
    //! and which one was generated.

    use super::*;
    use crate::parse::parse_sections;
    use crate::schema::{Schema, load_schema};
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    /// Build a conformant ADR containing exactly the body sections in `present` (a
    /// subset of `[context, decision, consequences]`, kept in schema order), each with
    /// a one-line opaque slot prose.
    fn build_partial_adr(present: &[&str]) -> String {
        let mut out =
            String::from("---\nstatus: proposed\ndate: 2026-05-31\n---\n\n# A decision\n");
        for id in present {
            let heading = match *id {
                "context" => "Context",
                "decision" => "Decision",
                "consequences" => "Consequences",
                _ => unreachable!(),
            };
            out.push_str(&format!("\n## {heading}\n\n{id} prose.\n"));
        }
        out
    }

    proptest! {
        /// Insert-then-parse keeps present sections in schema document-order: for an
        /// arbitrary subset of the three ADR body sections present, generating one of
        /// the absent ones yields a document whose section ids are a subsequence of the
        /// schema order (so the generated `##` landed in its schema-ordered slot).
        #[test]
        fn generated_section_lands_in_schema_order(
            mask in prop::collection::vec(any::<bool>(), 3..=3)
                .prop_filter("at least one absent", |m| !m.iter().all(|&b| b)),
            target_pick in 0usize..3,
        ) {
            let body = ["context", "decision", "consequences"];
            let present: Vec<&str> = body.iter().enumerate()
                .filter(|(i, _)| mask[*i]).map(|(_, s)| *s).collect();
            // Pick a target among the *absent* sections.
            let absent: Vec<&str> = body.iter().enumerate()
                .filter(|(i, _)| !mask[*i]).map(|(_, s)| *s).collect();
            let target = absent[target_pick % absent.len()];

            let src = build_partial_adr(&present);
            let schema = adr_schema();
            let out = generate_section(&schema, &src, target, Some("generated prose."), &[])
                .expect("absent section generates");

            // Generation materializes only the *one* leaf's structural home; other
            // required-but-absent sections stay a `finalize` concern (parsing.md → Absent
            // structural homes), so the result need not fully parse. The contract is:
            // the **present** sections, in source order, are in schema document-order.
            // We read present `##` homes directly (not a conformant parse).
            let got_in_source_order: Vec<String> = present_body_sections(&schema, &out)
                .into_iter().map(|(id, _)| id).collect();
            let schema_order: Vec<&str> = body.to_vec();
            let mut last: isize = -1;
            for id in &got_in_source_order {
                let pos = schema_order.iter().position(|s| s == id).unwrap() as isize;
                prop_assert!(pos > last, "section {id} out of schema order in {got_in_source_order:?}");
                last = pos;
            }
            // And the generated target is now present.
            prop_assert!(
                got_in_source_order.iter().any(|s| s == target),
                "target {target} present after generation"
            );
            // The fully-generated case (all three now present) must also conform.
            if got_in_source_order.len() == 3 {
                parse_sections(&schema, &out)
                    .map_err(|f| TestCaseError::fail(format!("complete doc must conform: {f:?}")))?;
            }
        }
    }
}

// ============================================================================
// Round-trip risk-spike capstone — the #1-technical-risk property/fuzz suite.
//
// This is the artifact that *retires* the risk: it proves the full round-trip
// contract end-to-end over **generated arbitrary conformant documents** for both
// MVP doc-types (`commit`, `adr`), with the edge cases real LLM/human prose carries
// (fenced `##`, trailing `- x:` lines, `####` headings, extra interior blank lines,
// BOM, CRLF-vs-LF). The two clauses (`parsing.md` → Round-trip guarantees):
//
//   1. **No-op write is idempotent modulo the canonicalization ledger** — parse an
//      arbitrary conformant doc → `first_touch_canonicalize` (the no-op write) →
//      byte-identical to the input *modulo* exactly BOM-strip + single-trailing-
//      newline (EOL preserved, prose never reflowed).
//   2. **Surgical on edits** — a single `set_field` → only that field's value bytes
//      differ; the EOL and every other byte survive intact.
//
// Plus a small insta golden pinning one tricky fixture (BOM + CRLF + fenced `##` +
// a `- x:` prose line + interior blank lines) for regression.
// ============================================================================
#[cfg(test)]
mod roundtrip {
    //! The dedicated #1-risk property/fuzz suite (done-criterion `cargo test -p
    //! engine roundtrip`). A generator for arbitrary conformant `commit`/`adr` docs
    //! drives both round-trip clauses; the canonicalization ledger is the only
    //! permitted modulo.

    use super::*;
    use crate::parse::parse_sections;
    use crate::schema::{Schema, load_schema};
    use proptest::prelude::*;

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");
    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn commit_schema() -> Schema {
        load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }
    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    /// One generated arbitrary conformant document plus its known-canonical LF form
    /// and the metadata the surgical-edit clause needs.
    #[derive(Clone, Debug)]
    struct GenDoc {
        /// `"commit"` or `"adr"` — selects the schema.
        ty: String,
        /// The canonical LF document text (no BOM, exactly one trailing `\n`).
        canonical_lf: String,
        /// The EOL the perturbed variant uses (`"\n"` or `"\r\n"`).
        eol: String,
        /// A present front-matter field key + a fresh canonical value to set it to
        /// (drives clause 2). `None` only if the schema had no settable field.
        edit: Option<(String, String)>,
    }

    /// Opaque slot prose with the edge cases real prose carries — each piece is
    /// heading-free at `##`/`###` depth (the ceiling) but stresses the parser:
    /// a fenced code block whose body is `## not a heading`, a `- x:` line that is
    /// prose (no sentinel), a `#### deeper` heading, and extra interior blank lines.
    /// The pieces are joined so the result is conformant slot prose.
    fn edgy_prose() -> impl Strategy<Value = String> {
        // A plain paragraph line (no markers that would end the slot).
        let para = "[a-zA-Z][a-zA-Z0-9 .,]{0,30}";
        // A grab-bag of edge fragments; each is conformant *inside* a slot span.
        let fragment = prop_oneof![
            para,
            Just("```\n## not a heading\n```".to_string()),
            Just("- x: this is prose, not a field".to_string()),
            Just("#### a deeper heading is allowed".to_string()),
            "[a-zA-Z][a-zA-Z0-9 .,]{0,30}".prop_map(|s: String| format!("{s}\n\n\n{s}")), // extra interior blanks
        ];
        prop::collection::vec(fragment, 1..4).prop_map(|frags| frags.join("\n\n"))
    }

    /// A canonical front-matter scalar value (trimmed, single-line, not list-shaped).
    fn scalar_value() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9][a-zA-Z0-9 ._/-]{0,18}[a-zA-Z0-9]|[a-zA-Z0-9]"
            .prop_filter("not list-shaped", |s: &String| {
                !(s.starts_with('[') && s.ends_with(']'))
            })
    }

    /// Build a canonical-LF `commit` document from generated parts: a `type`
    /// front-matter field, the `## Summary` + `## Body` slot sections, and the empty
    /// `## Trailers` repeatable section (matches the `render` byte form exactly).
    fn build_commit(ty: &str, summary: &str, body: &str) -> String {
        format!(
            "---\ntype: {ty}\n---\n\n# {summary}\n\n## Summary\n\n{summary}\n\n## Body\n\n{body}\n\n## Trailers\n",
            ty = ty,
            summary = summary.trim(),
            body = body.trim_end(),
        )
    }

    /// Build a canonical-LF `adr` document from generated parts.
    fn build_adr(
        title: &str,
        status: &str,
        date: &str,
        supersedes: Option<&str>,
        context: &str,
        decision: &str,
        consequences: &str,
    ) -> String {
        let mut fm = format!("status: {status}\ndate: {date}\n");
        if let Some(target) = supersedes {
            fm.push_str(&format!("supersedes: adr:{target}\n"));
        }
        format!(
            "---\n{fm}---\n\n# {title}\n\n## Context\n\n{context}\n\n## Decision\n\n{decision}\n\n## Consequences\n\n{consequences}\n",
            context = context.trim_end(),
            decision = decision.trim_end(),
            consequences = consequences.trim_end(),
        )
    }

    /// The generator: an arbitrary conformant `commit` or `adr` doc, with a chosen
    /// EOL, returning the canonical-LF text + the EOL + a settable front-matter field.
    fn arb_doc() -> impl Strategy<Value = GenDoc> {
        let eol = prop_oneof![Just("\n".to_string()), Just("\r\n".to_string())];

        let commit = (
            prop::sample::select(vec!["feat", "fix", "docs", "chore", "refactor"]),
            scalar_value(),
            edgy_prose(),
        )
            .prop_map(|(ty, summary, body)| {
                // Re-set `type` to a *different* enum member so the edit always
                // changes a byte (the surgical clause asserts exactly one line differs).
                let new_type = if ty == "feat" { "fix" } else { "feat" };
                (
                    "commit".to_string(),
                    build_commit(ty, &summary, &body),
                    Some(("type".to_string(), new_type.to_string())),
                )
            });

        let adr = (
            scalar_value(),
            prop::sample::select(vec!["proposed", "accepted", "superseded"]),
            proptest::option::of("[a-z][a-z0-9-]{0,18}"),
            edgy_prose(),
            edgy_prose(),
            edgy_prose(),
        )
            .prop_map(|(title, status, supersedes, ctx, dec, con)| {
                // Re-set `status` to a *different* enum member so the edit always
                // changes a byte (clause 2 asserts exactly one line differs).
                let new_status = if status == "proposed" {
                    "accepted"
                } else {
                    "proposed"
                };
                (
                    "adr".to_string(),
                    build_adr(
                        &title,
                        status,
                        "2026-05-31",
                        supersedes.as_deref(),
                        &ctx,
                        &dec,
                        &con,
                    ),
                    Some(("status".to_string(), new_status.to_string())),
                )
            });

        (prop_oneof![commit, adr], eol).prop_map(|((ty, canonical_lf, edit), eol)| GenDoc {
            ty,
            canonical_lf,
            eol,
            edit,
        })
    }

    fn schema_for(ty: &str) -> Schema {
        match ty {
            "commit" => commit_schema(),
            _ => adr_schema(),
        }
    }

    /// Project a canonical-LF doc onto the generated EOL (EOL is preserved through
    /// the round-trip, never normalized).
    fn with_eol(canonical_lf: &str, eol: &str) -> String {
        if eol == "\n" {
            canonical_lf.to_string()
        } else {
            canonical_lf.replace('\n', "\r\n")
        }
    }

    proptest! {
        /// Clause 1 — **no-op write idempotent modulo the canonicalization ledger.**
        /// An arbitrary conformant doc (with a chosen EOL) is first confirmed to
        /// parse; then a BOM and extra trailing newlines are added; the no-op write
        /// (`first_touch_canonicalize`) must reproduce the canonical EOL doc exactly —
        /// byte-identical modulo *only* BOM-strip + single-trailing-newline. EOL is
        /// preserved; interior prose is never reflowed.
        #[test]
        fn no_op_write_is_byte_identical_modulo_ledger(doc in arb_doc()) {
            let schema = schema_for(&doc.ty);
            let canonical = with_eol(&doc.canonical_lf, &doc.eol);

            // The generated doc must be conformant in the first place (the generator's
            // own invariant — a non-conformant fixture would be a generator bug).
            prop_assert!(
                parse_sections(&schema, &canonical).is_ok(),
                "generated {} doc must parse: {:?}",
                doc.ty,
                parse_sections(&schema, &canonical).err()
            );

            // The no-op write of the already-canonical doc is byte-identical (the
            // idempotent-on-canonical clause).
            prop_assert_eq!(first_touch_canonicalize(&canonical), canonical.clone());

            // Perturb with the two ledger-covered deviations: a leading BOM and extra
            // trailing newlines (EOL units). The no-op write must canonicalize *only*
            // those back to the canonical form — nothing line-spanning touched.
            let perturbed = format!("\u{feff}{canonical}{eol}{eol}", eol = doc.eol);
            prop_assert_eq!(first_touch_canonicalize(&perturbed), canonical.clone());

            // EOL is preserved: a CRLF doc canonicalizes to CRLF, never LF.
            if doc.eol == "\r\n" {
                prop_assert!(
                    first_touch_canonicalize(&perturbed).contains("\r\n"),
                    "CRLF must be preserved, never normalized to LF"
                );
            }
            // And the canonicalized form still parses (re-parse stability).
            let canon = first_touch_canonicalize(&perturbed);
            prop_assert!(
                parse_sections(&schema, &canon).is_ok(),
                "canonicalized doc must still parse"
            );
        }

        /// Clause 2 — **surgical on edits.** Over an arbitrary conformant doc, a single
        /// `set_field` of a present front-matter field changes *only* that field's
        /// value bytes: the prefix before the value and the suffix after it survive
        /// byte-identical, the EOL is preserved, and the new value is in place.
        #[test]
        fn single_field_edit_changes_only_that_field(doc in arb_doc()) {
            let Some((key, new_value)) = doc.edit.clone() else { return Ok(()); };
            let schema = schema_for(&doc.ty);
            let source = with_eol(&doc.canonical_lf, &doc.eol);
            // The header is the front-matter section: "header" (commit) / "status" (adr).
            let section_id = if doc.ty == "commit" { "header" } else { "status" };
            // Skip the no-op edit case (new value equals the existing one): there is
            // no byte to change, so "exactly one line differs" would not hold. This is
            // the surgical clause over a *real* change, the case worth proving.
            if source.contains(&format!("{key}: {new_value}")) {
                return Ok(());
            }

            let edited = set_field(&schema, &source, section_id, &key, &new_value)
                .expect("present front-matter field is settable");

            // EOL preserved.
            if doc.eol == "\r\n" {
                prop_assert!(edited.contains("\r\n"), "EOL must survive the edit");
            }

            // Only the target field's value line differs: every *other* line is
            // byte-identical and in the same position. We compare line-by-line on the
            // raw EOL-split so the surgical-on-edit clause is checked at field
            // granularity (`parsing.md` → Round-trip guarantees clause 2).
            let src_lines: Vec<&str> = source.split(doc.eol.as_str()).collect();
            let edt_lines: Vec<&str> = edited.split(doc.eol.as_str()).collect();
            prop_assert_eq!(src_lines.len(), edt_lines.len(), "no lines added/removed");
            let mut differing = Vec::new();
            for (i, (a, b)) in src_lines.iter().zip(edt_lines.iter()).enumerate() {
                if a != b {
                    differing.push(i);
                }
            }
            prop_assert_eq!(differing.len(), 1, "exactly one line differs");
            let changed = edt_lines[differing[0]];
            prop_assert!(
                changed.starts_with(&format!("{key}: ")),
                "the differing line is the target field {key:?}: {changed:?}"
            );
            prop_assert_eq!(changed, format!("{key}: {new_value}"));
        }
    }

    /// Golden — one tricky fixture pinning the no-op-write canonicalization end to
    /// end: a CRLF ADR with a leading BOM, a fenced ` ``` ` block whose body is
    /// `## not a heading`, a `- x:` prose line (no sentinel → stays prose), a `####`
    /// deeper heading, extra interior blank lines, and a doubled trailing CRLF. The
    /// no-op write strips the BOM, collapses the trailing newlines to one CRLF, and
    /// preserves *everything else* byte-for-byte (EOL, the fenced `##`, the `- x:`
    /// line, the interior blanks). Pins the exact canonical bytes for regression.
    #[test]
    fn golden_tricky_fixture_no_op_write() {
        let body = "\
---
status: accepted
date: 2026-05-31
---

# A tricky decision

## Context

Some context here.

```
## not a heading
```

## Decision

- x: this looks like a field but has no sentinel, so it is prose

#### a deeper heading is fine


An interior blank-line gap above is preserved.

## Consequences

Done.
";
        // Build the on-disk variant: BOM + CRLF + doubled trailing newline.
        let on_disk = format!("\u{feff}{}\r\n\r\n", body.replace('\n', "\r\n"));
        let canon = first_touch_canonicalize(&on_disk);

        // It still parses (the canonicalization didn't break structure).
        let schema = adr_schema();
        assert!(
            parse_sections(&schema, &canon).is_ok(),
            "canonicalized tricky fixture must parse: {:?}",
            parse_sections(&schema, &canon).err()
        );
        // No BOM, single trailing CRLF, everything else intact.
        assert!(!canon.starts_with('\u{feff}'), "BOM stripped");
        assert!(canon.ends_with("\r\n"), "single trailing CRLF");
        assert!(!canon.ends_with("\r\n\r\n"), "exactly one trailing CRLF");
        insta::assert_snapshot!("roundtrip_tricky_fixture", canon);
    }
}

#[cfg(test)]
mod commit_render {
    //! The commit string sink (`design/finalize.md` → Commit-doc rendering): a
    //! `commit` instance → a git-message string. Golden (insta) over each rendered
    //! variant pins the subject/body/trailer shape — empty-scope omits the parens,
    //! sections are blank-line-separated, no trailing whitespace. A proptest asserts
    //! the renderer is a pure function of `(schema, instance)`.

    use super::*;
    use crate::field_block::Value;
    use crate::schema::{Schema, load_schema};

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn commit_schema() -> Schema {
        load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    fn scalar(key: &str, value: &str) -> Field {
        Field {
            key: key.to_string(),
            value: Value::Scalar(value.to_string()),
        }
    }

    /// Build a commit instance from its parts: a `type`, an optional `scope`, a
    /// `summary`, an optional `body`, and an ordered list of `(key, value)` trailers.
    fn commit_instance(
        ty: &str,
        scope: Option<&str>,
        summary: &str,
        body: Option<&str>,
        trailers: &[(&str, &str)],
    ) -> Instance {
        let mut header_fields = vec![scalar("type", ty)];
        if let Some(scope) = scope {
            header_fields.push(scalar("scope", scope));
        }
        let items: Vec<ItemContent> = trailers
            .iter()
            .map(|(k, v)| ItemContent {
                id: crate::slug::slugify(k),
                title: (*k).to_string(),
                slot: None,
                fields: vec![scalar("key", k), scalar("value", v)],
            })
            .collect();
        Instance {
            title: summary.to_string(),
            sections: vec![
                SectionContent {
                    id: "header".to_string(),
                    fields: header_fields,
                    ..Default::default()
                },
                SectionContent {
                    id: "summary".to_string(),
                    slot: Some(summary.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    slot: body.map(str::to_string),
                    ..Default::default()
                },
                SectionContent {
                    id: "trailers".to_string(),
                    items,
                    ..Default::default()
                },
            ],
        }
    }

    /// (a) type + summary only: the subject line `<type>: <summary>` with **no**
    /// scope parens, no body, no trailers — and no trailing whitespace/newline.
    #[test]
    fn commit_renders_type_and_summary() {
        let instance = commit_instance("feat", None, "add a per-client rate limiter", None, &[]);
        let out = render_commit_message(&commit_schema(), &instance);
        assert_eq!(out, "feat: add a per-client rate limiter");
        insta::assert_snapshot!("commit_type_summary", out);
    }

    /// (b) type + scope + summary + body: subject carries `(<scope>)`, the body
    /// follows after exactly one blank line.
    #[test]
    fn commit_renders_type_scope_summary_body() {
        let instance = commit_instance(
            "feat",
            Some("gateway"),
            "add a per-client rate limiter",
            Some(
                "Centralize rate limiting at the gateway so each service\ndrops its local limiter.",
            ),
            &[],
        );
        let out = render_commit_message(&commit_schema(), &instance);
        assert_eq!(
            out,
            "feat(gateway): add a per-client rate limiter\n\n\
             Centralize rate limiting at the gateway so each service\n\
             drops its local limiter."
        );
        insta::assert_snapshot!("commit_type_scope_summary_body", out);
    }

    /// (c) + trailers: the footer follows the body after one blank line, one
    /// `key: value` line per trailer in item order.
    #[test]
    fn commit_renders_with_trailers() {
        let instance = commit_instance(
            "fix",
            Some("api"),
            "reject the 101st request in a 60s window",
            Some("The burst allowance was off by one."),
            &[
                ("Refs", "#1242"),
                ("Co-Authored-By", "Ada <ada@example.com>"),
            ],
        );
        let out = render_commit_message(&commit_schema(), &instance);
        assert_eq!(
            out,
            "fix(api): reject the 101st request in a 60s window\n\n\
             The burst allowance was off by one.\n\n\
             Refs: #1242\n\
             Co-Authored-By: Ada <ada@example.com>"
        );
        insta::assert_snapshot!("commit_with_trailers", out);
    }

    /// The done-criterion test name: the three variants share one assertion that
    /// empty-scope omits parens, sections are blank-line-separated, and nothing
    /// trails. Each rendered message has no trailing whitespace and no `()`.
    #[test]
    fn commit_renders_to_git_message() {
        let no_scope = commit_instance("docs", None, "update the readme", None, &[]);
        let rendered_no_scope = render_commit_message(&commit_schema(), &no_scope);
        assert!(
            !rendered_no_scope.contains('('),
            "empty scope must omit the parens entirely"
        );

        let full = commit_instance(
            "feat",
            Some("gateway"),
            "rate-limit",
            Some("Body."),
            &[("Refs", "#1")],
        );
        let rendered_full = render_commit_message(&commit_schema(), &full);
        // Blank-line separation between subject, body, and footer.
        assert!(rendered_full.contains("rate-limit\n\nBody."));
        assert!(rendered_full.contains("Body.\n\nRefs: #1"));

        for rendered in [&rendered_no_scope, &rendered_full] {
            assert_eq!(
                rendered.as_str(),
                rendered.trim_end(),
                "no trailing whitespace/newline"
            );
            for line in rendered.lines() {
                assert_eq!(line, line.trim_end(), "no trailing whitespace on any line");
            }
        }
    }

    use proptest::prelude::*;

    proptest! {
        /// The renderer is a **pure function of (schema, instance)**: the same
        /// instance rendered twice yields an identical string (no hidden state, no
        /// nondeterminism). The determinism contract of the string sink.
        #[test]
        fn render_is_pure(
            ty in prop::sample::select(vec!["feat", "fix", "docs", "chore"]),
            scope in proptest::option::of("[a-z][a-z0-9-]{0,10}"),
            summary in "[a-zA-Z][a-zA-Z0-9 ]{0,40}",
            body in proptest::option::of("[a-zA-Z][a-zA-Z0-9 .]{0,40}"),
        ) {
            let instance = commit_instance(ty, scope.as_deref(), &summary, body.as_deref(), &[]);
            let schema = commit_schema();
            let first = render_commit_message(&schema, &instance);
            let second = render_commit_message(&schema, &instance);
            prop_assert_eq!(first, second);
        }
    }
}
