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
    /// The item body's slot prose (the **single-slot** bare-prose form), opaque
    /// bytes emitted verbatim. A multi-slot item carries `slots` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    /// The per-leaf slot prose, in schema block order, for a **multi-slot** item
    /// (M16). Each `(leaf_id, prose)` renders under its `#### <Leaf-Title>`
    /// sub-heading. Empty for a single-slot / slot-less item (the bare-prose `slot`
    /// carries the single-slot case). The inverse of [`crate::parse::ParsedItem::slots`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<(String, String)>,
    /// The item's per-item field values, in schema order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
    /// The item's **nested** repeatable items, in physical order — the M22
    /// multi-level lift (a release's nested change-groups). Each renders one heading
    /// level deeper than this item (`2 + nesting-depth`, capped at H6) and is itself
    /// an [`ItemContent`], mirroring [`crate::parse::ParsedItem::items`]. Empty (and
    /// skip-on-serialize) for a flat single-level item, so every shipped
    /// single-level golden's bytes are unchanged (the additive guard).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ItemContent>,
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

/// Render a top-level repeatable item (nesting depth 1). The thin entry point the
/// section renderer + the splice helpers call; [`render_item_at`] carries the
/// depth-aware recursion.
fn render_item(item: &ItemContent) -> String {
    render_item_at(item, 1)
}

/// Render one repeatable item at nesting `depth`: `<#…> <title>  {#id}` at heading
/// level `2 + depth` (depth 1 → `###`, depth 2 → `####`, … per the H6 cap), a blank
/// line, the item body, an optional trailing field group, then — recursively — its
/// **nested** items one level deeper. Two spaces precede the `{#id}` anchor.
///
/// The body is either the **single-slot** bare prose (`slots` empty — the form every
/// existing single-slot doctype keeps, byte-identical backward-compat) or, for a
/// **multi-slot** item (`slots` non-empty), each slot under its `<#…> <Leaf-Title>`
/// sub-heading (one level deeper than the item) in schema block order (M16). A
/// **nested** item (`items` non-empty — the M22 multi-level lift) renders its
/// children at `depth + 1` after the field group; the changelog's release item is the
/// driving case (no slot, a `date` field, then nested `#### change-group` items). A
/// slot/multi-slot AND nested children in one item is a deferred combination the
/// changelog avoids, so the slot and nested paths never coincide here.
fn render_item_at(item: &ItemContent, depth: usize) -> String {
    let item_hashes = "#".repeat(item_heading_level(depth));
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} {}  {{#{}}}",
        item_hashes,
        item.title.trim(),
        item.id
    );
    // A **slotless** item (no slot leaf, no multi-slot leaves — the M22 nested-parent
    // / fields-only case, e.g. a changelog release: `date` field then nested
    // `#### change-group` items) has no body prose, so the heading is followed directly
    // by its field group / nested items at a single-blank-line gap. We leave `out`
    // ending at the heading's `\n` (no extra blank line) so `append_field_group`'s
    // leading `\n\n` yields `### …\n\n<!-- fields -->`. An item whose single slot is
    // **present but empty** (`slot: Some("")` — what a re-parse yields for a
    // minted-but-unfilled slot) canonicalizes to this same form (M26 fork C4), so it
    // renders byte-identically to the absent-slot mint and `render(parse(x)) == x`
    // holds. A slot-bearing item with actual prose keeps the existing
    // `### …\n\n<prose>` form (the post-heading blank, byte-identical backward-compat).
    // The `trim()` is intentional (not `is_empty()`): a whitespace-only slot canonicalizes
    // to the same empty form — equivalent to `Some("")` on a parsed instance (the parser
    // trims slot prose) and the preferable normalization for a programmatically-built one,
    // consistent with the writer's `trim_end` discipline below.
    let slot_empty = item.slot.as_deref().is_none_or(|s| s.trim().is_empty());
    let slotless = slot_empty && item.slots.is_empty();
    if slotless {
        out.truncate(out.trim_end_matches('\n').len());
    } else {
        out.push('\n');
    }
    if item.slots.is_empty() {
        let prose = item.slot.as_deref().unwrap_or("");
        out.push_str(prose.trim_end());
    } else {
        // Multi-slot: each leaf renders under its sub-heading one level deeper than the
        // item, the slots joined by a single blank line (the `\n\n` discipline sections
        // use).
        let leaf_hashes = "#".repeat(item_heading_level(depth) + 1);
        let blocks: Vec<String> = item
            .slots
            .iter()
            .map(|(leaf_id, prose)| {
                let mut block = format!("{} {}\n", leaf_hashes, heading_text(leaf_id));
                block.push('\n');
                block.push_str(prose.trim_end());
                // Trim a slot whose prose is empty back to just its heading line so an
                // empty-skeleton slot is `#### Proves` (no trailing blanks); a filled
                // one is `#### Proves\n\n<prose>`.
                block.trim_end().to_string()
            })
            .collect();
        out.push_str(&blocks.join("\n\n"));
    }
    append_field_group(&mut out, &item.fields);
    // Nested items (the M22 multi-level lift): each child renders one level deeper,
    // after the parent's leaves. `append_field_group` left `out` ending in exactly one
    // `\n` (the parent body's terminator); re-attach the canonical one-blank-line gap
    // before the first child and join children with the same single-blank discipline
    // `render_section` uses, then EOF-normalize to one trailing `\n`.
    if !item.items.is_empty() {
        let children: Vec<String> = item
            .items
            .iter()
            .map(|child| render_item_at(child, depth + 1))
            .collect();
        let joined = children.join("\n");
        let body = out.trim_end_matches('\n');
        out = format!("{body}\n\n{}\n", joined.trim_end_matches('\n'));
    }
    out
}

/// The ATX heading level a repeatable item at nesting `depth` renders at: a section
/// is `##` (H2), so a depth-`d` item is at level `2 + d` (depth 1 → `###`, depth 2 →
/// `####`, …), the writer-side dual of the parser's `item_level_num`. The schema
/// loader caps nesting at H6, so the level never exceeds 6.
fn item_heading_level(depth: usize) -> usize {
    2 + depth
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

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }
    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
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
        let schema =
            crate::schema::load_schema_with_types(yaml, &crate::schema::dev_pack_field_types())
                .expect("spec schema loads");
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
                            slots: Vec::new(),
                            fields: vec![scalar("maps-to-test", "`test/rate_limit_spec.rb#burst`")],
                            items: Vec::new(),
                        },
                        ItemContent {
                            id: "burst-allowance".to_string(),
                            title: "Burst allowance".to_string(),
                            slot: Some(
                                "A short burst above the limit is tolerated for 2s.".to_string(),
                            ),
                            slots: Vec::new(),
                            fields: vec![],
                            items: Vec::new(),
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

    /// M10 inc-1 round-trip clause: a hand-authored `adr` carrying the optional
    /// pack-declared `cites-code: <path>#<symbol>` anchor in its `status` header
    /// **canonical-writes then re-parses idempotently** — `write → parse → write`
    /// byte-identical — and the parsed `cites-code` value is the exact `path#symbol`
    /// text (the writer places a `code-anchor` field on the splice path like any
    /// header scalar; the `#`/`/` are opaque to the writer/parser, the bound
    /// `doc-code` probe adjudicates them later). The schema loads only because the
    /// `code-anchor → doc-code` set is threaded in (`adr_schema`).
    #[test]
    fn adr_with_a_cites_code_anchor_round_trips_byte_identical() {
        let schema = adr_schema();
        let anchor = "crates/engine/src/schema.rs#FieldType";
        let instance = Instance {
            title: "Rate-limit at the gateway".to_string(),
            sections: vec![
                SectionContent {
                    id: "status".to_string(),
                    fields: vec![
                        scalar("status", "accepted"),
                        scalar("date", "2026-05-23"),
                        scalar("cites-code", anchor),
                    ],
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
        let doc = parse_sections(&schema, &first).expect("rendered adr parses");
        // The parsed `cites-code` value is the verbatim `path#symbol` anchor.
        let status = doc
            .sections
            .iter()
            .find(|s| s.id == "status")
            .expect("status header present");
        let cites = status
            .fields
            .iter()
            .find(|f| f.key == "cites-code")
            .expect("cites-code parsed");
        assert_eq!(
            cites.value,
            Value::Scalar(anchor.to_string()),
            "the code-anchor value round-trips verbatim",
        );
        let second = render(&schema, &reparse_to_instance(&schema, &first));
        assert_eq!(
            first, second,
            "an adr carrying cites-code is byte-identical across write → parse → write",
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
                    .map(|it| item_content_from_parsed(it, source))
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
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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
            let schema = crate::schema::load_schema_with_types(
                yaml,
                &crate::schema::dev_pack_field_types(),
            )
            .expect("spec schema loads");
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
                        slots: Vec::new(),
                        fields,
                        items: Vec::new(),
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
    if section.slot.is_none() {
        return Err(SpliceError::NotPresent {
            what: format!("slot in section {section_id:?}"),
        });
    }

    // Re-render the whole section body via [`render_section`] with the new slot prose
    // (the section's existing fields preserved), then splice it over the section's body
    // region — the same canonicalization trick [`set_item_slot`] (3afc98a) uses for an
    // item slot. A bare splice of the recorded slot span is **not** byte-stable on a
    // **leading** Simple slot (a section followed by another section whose empty form
    // carries surrounding blanks): the parser records an empty/leading slot's span
    // *without* the canonical surrounding blank lines, so splicing prose into it yields
    // `## Goal\nprose\n\n\n## Context` (no blank after the heading, a double blank before
    // the next section) while [`render_section`] emits one blank line on each side — so
    // `render(parse(out)) != out`. Re-rendering the body (one canonical section-bytes
    // path) is the fix; `set_field`'s surgical span-splice is untouched (a field-value
    // edit must stay byte-surgical on non-canonical input).
    let schema_section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or(SpliceError::NotConformant)?;
    let content = SectionContent {
        id: section.id.clone(),
        slot: Some(new_prose.to_string()),
        fields: section.fields.clone(),
        items: Vec::new(),
    };
    // [`render_section`] emits `## Heading\n` then the body; strip that heading line to
    // get the canonical body bytes (`\n` + prose + field-group / `\n`).
    let rendered = render_section(schema_section, Some(&content));
    let body = rendered
        .split_once('\n')
        .map(|(_, rest)| rest)
        .unwrap_or("");
    let body = body.trim_end_matches('\n');

    let blocks = parse::scan_blocks(source);
    let present = present_body_sections(schema, source);
    let region = section_region(&blocks, source, section_id, &present).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("section {section_id:?} body region"),
        }
    })?;
    // Re-attach the **canonical** inter-section separator (not the recorded one): a
    // following `##` heading gets `body\n\n` (one blank line — [`render`] joins section
    // blocks with a single `\n` and each block already ends in one `\n`); a trailing
    // section (region runs to EOF) gets `body\n` (the single EOF newline).
    let replacement = if region.end < source.len() {
        format!("{body}\n\n")
    } else {
        format!("{body}\n")
    };
    Ok(splice(source, region, &replacement))
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
    // LAST-block edge: a span ending at EOF carries no trailing separator, so consume
    // the one `\n` of the preceding blank-line separator to avoid a dangling blank line
    // (the same edge `remove_nested_item` handles — keeping `render(parse(out)) == out`).
    let start = if span.end == source.len() && source[..span.start].ends_with('\n') {
        span.start - 1
    } else {
        span.start
    };
    Ok(splice(source, start..span.end, ""))
}

/// `set-item-field` (item field present): replace the **value** bytes of `field_key`
/// on the repeatable item `item_id` in `section_id`, scoped to that item's OWN byte
/// region — so neither a sibling item's identically-keyed field (the wrong-item write
/// bug) nor a nested child's field block (the parent-region-swallows-child bug) is
/// matched. Not a wrapper over [`set_field`]: that scans globally and would hit the
/// first matching key. Re-parses for conformance, asserts the item and its `field_key`
/// are present, then locates the value span *within* the item's own leaf region
/// ([`locate_item_path`] narrowed by [`item_own_leaf_region`]) via
/// [`field_value_in_lines`] over the item's sentinelled `- key: value` bullets
/// (`bullet = true`). An absent item / field → [`SpliceError::NotPresent`].
pub fn set_item_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_id: &str,
    field_key: &str,
    new_value: &str,
) -> Result<String, SpliceError> {
    let doc = parse::parse_sections(schema, source).map_err(|_| SpliceError::NotConformant)?;
    let section = doc
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("section {section_id:?}"),
        })?;
    let item = section
        .items
        .iter()
        .find(|i| i.id == item_id)
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("item {item_id:?} in section {section_id:?}"),
        })?;
    if !item.fields.iter().any(|f| f.key == field_key) {
        return Err(SpliceError::NotPresent {
            what: format!("field {field_key:?} on item {item_id:?}"),
        });
    }

    // The item's byte region — resolved through the parent-scoped path locator (a
    // single-element chain), then narrowed to the item's OWN leaf region so a nested
    // child's identically-keyed bullet is out of range (the parent's field swallowed by
    // its child's field block — the corruption bug) just as a sibling item's is.
    let blocks = parse::scan_blocks(source);
    let region = locate_item_path(schema, source, section_id, &[item_id]).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {item_id:?} block"),
        }
    })?;
    let region = item_own_leaf_region(&blocks, region);
    let value_span = field_value_in_lines(source, region, field_key, true).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("field {field_key:?} value line on item {item_id:?}"),
        }
    })?;
    Ok(splice(source, value_span, new_value))
}

/// `set-item-slot` (item slot present): replace the per-item slot prose of the
/// repeatable item `item_id` in `section_id` with `new_prose`, leaving every other
/// byte intact. Scoped to the addressed item by reading **that item's** recorded slot
/// span — the dual of [`set_slot`]/`locate_slot_span` (which read only a section-level
/// slot, the wrong-item bug for per-item slots). The parser already records each
/// item's slot as [`crate::parse::ParsedItem::slot`], so we read the addressed item's
/// span directly rather than re-scanning bytes. An absent item, a non-repeatable
/// section, or an item whose template declares no slot → [`SpliceError::NotPresent`].
pub fn set_item_slot(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_id: &str,
    leaf_id: &str,
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
    let item = section
        .items
        .iter()
        .find(|i| i.id == item_id)
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("item {item_id:?} in section {section_id:?}"),
        })?;
    // The addressed leaf's slot span must be present — single-slot via the bare
    // `slot`, multi-slot via the named `slots` entry (`slot_span` resolves either).
    if item.slot_span(leaf_id).is_none() {
        return Err(SpliceError::NotPresent {
            what: format!("slot {leaf_id:?} in item {item_id:?}"),
        });
    }

    // Re-render the whole item via [`render_item`] with the new slot prose, then splice
    // it over the item's located block — the same canonicalization trick
    // [`insert_item_field`] uses. A bare splice of the recorded slot span is **not**
    // byte-stable on a **mint-empty** item: [`add_item`] records its empty slot span
    // *without* the canonical surrounding blank lines, so splicing prose in yields
    // `### H  {#h}\nprose\n### Next` while [`render_item`] emits the blanks — so
    // `render(parse(out)) != out`. Mint-empty is the only state an item-leaf set-slot
    // runs in, so re-rendering (one canonical item-bytes path) is the fix.
    //
    // The located block spans `[item.start .. next-heading | EOF)`, so it *includes* the
    // inter-item gap, which on non-canonical input may be wider than a *filled* item's
    // canonical one-blank-line gap. So we re-attach the **canonical** separator, not the
    // recorded one: [`render_section`] joins full
    // `render_item`s with a single `\n`, and a filled item's render already ends in one
    // `\n`, so a following item / section gets `render_item(item)` + `\n` (one blank
    // line); a trailing item (block runs to EOF) gets the full render, EOF-normalized to
    // one `\n`.
    // The item's full sub-tree region, resolved through the parent-scoped path locator
    // (a single-element chain) — unambiguous when another parent carries a same-anchor
    // item. The whole sub-tree is re-rendered (slot prose changes, children preserved by
    // [`render_item`]'s recursion), so this is the full region, not the leaf region.
    let region = locate_item_path(schema, source, section_id, &[item_id]).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {item_id:?} block"),
        }
    })?;
    // Re-derive the item's content (preserving sibling slots + fields) and overwrite
    // the addressed leaf's prose — single-slot updates the bare `slot`, multi-slot
    // updates the named `slots` entry, sibling slots untouched.
    let mut content = item_content_from_parsed(item, source);
    if content.slots.is_empty() {
        content.slot = Some(new_prose.to_string());
    } else if let Some(entry) = content.slots.iter_mut().find(|(id, _)| id == leaf_id) {
        entry.1 = new_prose.to_string();
    }
    let rendered = render_item(&content);
    let body = rendered.trim_end_matches('\n');
    let replacement = if region.end < source.len() {
        // A following `###`/`##` heading: the canonical inter-block gap is one blank
        // line — `render_section` joins full `render_item`s (each ending in one `\n`)
        // with a single `\n`, i.e. `body\n\n` before the next heading.
        format!("{body}\n\n")
    } else {
        // Trailing item: the full render normalized to exactly one trailing `\n` (EOF).
        format!("{body}\n")
    };
    Ok(splice(source, region, &replacement))
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

/// Strip the **nested-section** segments from a *section-qualified* address chain,
/// yielding the **physical item-id chain** — the segments that have a `{#id}` heading
/// in the document (the only ones the byte locator and parsed tree walk).
///
/// The canonical nested address is section-qualified (review finding S1,
/// `design/changelog.md` → engine work #1): `#releases/1-2-0/changes/added/notes` names
/// the nested-section `changes` between the release item `1-2-0` and the change-group
/// item `added`. But a nested section is a purely *logical* schema hop — its items
/// render directly one heading-level under the parent (`#### added`, not a `#### changes`
/// wrapper), so the document tree has no `changes` node. The chain therefore alternates
/// `item, nested-section, item, nested-section, …` starting at an item; this walks it
/// against the schema, keeping each item id and **dropping** each nested-section id after
/// confirming it names a declared [`Leaf::Repeatable`] at that level (so a mistyped
/// nested-section segment is rejected, not silently treated as an item id). Returns the
/// item-only chain (`["1-2-0", "added"]`), or `None` if a nested-section segment names no
/// declared nested repeatable.
fn physical_item_chain<'a>(
    schema: &Schema,
    section_id: &str,
    chain: &[&'a str],
) -> Option<Vec<&'a str>> {
    let section = schema.sections.iter().find(|s| s.id == section_id)?;
    let SectionBody::Repeatable { repeatable } = &section.body else {
        return None;
    };
    let mut block = &repeatable.block;
    let mut physical: Vec<&str> = Vec::new();
    let mut expect_item = true;
    for segment in chain {
        if expect_item {
            // An item id at this repeatable level — kept (it has a heading), and its
            // block becomes the scope for any following nested-section segment.
            physical.push(segment);
            expect_item = false;
        } else {
            // A nested-section id — must name a `Leaf::Repeatable` declared in the
            // current block. Dropped from the physical chain (no heading of its own);
            // its inner block becomes the scope for the next item id.
            let nested = block.iter().find_map(|leaf| match leaf {
                crate::schema::Leaf::Repeatable { id, repeatable } if id == segment => {
                    Some(&repeatable.block)
                }
                _ => None,
            })?;
            block = nested;
            expect_item = true;
        }
    }
    Some(physical)
}

/// Descend a parsed document to the nested [`parse::ParsedItem`] addressed by the
/// section-qualified id chain `item_ids` under `section_id` (`["1-2-0", "changes",
/// "added"]` → release `1-2-0`'s nested `#added`). The chain is first reduced to its
/// **physical** item-id chain ([`physical_item_chain`], dropping the logical
/// nested-section hops), then each item is matched **within its parent's items**
/// (parent-scoped, the model the byte locator [`locate_item_path`] enforces on disk), so
/// a same-anchor item under a different parent is never returned. `None` if any segment
/// is absent or a nested-section segment is unknown.
fn nested_parsed_item<'a>(
    schema: &Schema,
    doc: &'a parse::Document,
    section_id: &str,
    item_ids: &[&str],
) -> Option<&'a parse::ParsedItem> {
    let physical = physical_item_chain(schema, section_id, item_ids)?;
    let section = doc.sections.iter().find(|s| s.id == section_id)?;
    let mut items = &section.items;
    let mut found: Option<&parse::ParsedItem> = None;
    for id in &physical {
        let item = items.iter().find(|i| &i.id == id)?;
        found = Some(item);
        items = &item.items;
    }
    found
}

/// `set-item-slot` for a (possibly nested) repeatable item, addressed by its parent
/// -scoped id chain `item_ids` (`["1-2-0", "added"]`). The depth-aware dual of
/// [`set_item_slot`]: it locates the nested item's byte region via [`locate_item_path`]
/// (so a same-anchor sibling under another parent is out of range), re-renders that item
/// at its nesting depth via [`render_item_at`], and splices it back with the canonical
/// inter-block separator — the one item-bytes path, keeping `render(parse(out)) == out`.
/// An absent item / leaf / non-conformant source → [`SpliceError`].
pub fn set_nested_item_slot(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    leaf_id: &str,
    new_prose: &str,
) -> Result<String, SpliceError> {
    let doc = parse::parse_sections(schema, source).map_err(|_| SpliceError::NotConformant)?;
    let item = nested_parsed_item(schema, &doc, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {item_ids:?} in section {section_id:?}"),
        }
    })?;
    if item.slot_span(leaf_id).is_none() {
        return Err(SpliceError::NotPresent {
            what: format!("slot {leaf_id:?} in item {:?}", item.id),
        });
    }

    let physical = physical_item_chain(schema, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {item_ids:?} in section {section_id:?}"),
        }
    })?;
    let region = locate_item_path(schema, source, section_id, &physical).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {item_ids:?} block"),
        }
    })?;
    let mut content = item_content_from_parsed(item, source);
    if content.slots.is_empty() {
        content.slot = Some(new_prose.to_string());
    } else if let Some(entry) = content.slots.iter_mut().find(|(id, _)| id == leaf_id) {
        entry.1 = new_prose.to_string();
    }
    let rendered = render_item_at(&content, physical.len());
    Ok(splice(
        source,
        region.clone(),
        &nested_replacement(source, &region, &rendered),
    ))
}

/// `set-item-field`-or-insert for a (possibly nested) repeatable item, addressed by its
/// parent-scoped id chain `item_ids`. The depth-aware dual of
/// [`set_item_field_or_insert`]: it re-derives the nested item, sets/overwrites the
/// field, re-renders at the item's nesting depth, and splices canonically. Like its
/// top-level dual it **adjudicates the value's declared type before touching bytes**
/// (closing the 2026-06-07 item-field parity gap) — a malformed value is rejected as
/// [`GenerateError::MalformedValue`]. A genuinely absent item / non-repeatable section →
/// [`GenerateError`].
pub fn set_nested_item_field_or_insert(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field_key: &str,
    new_value: &str,
) -> Result<String, GenerateError> {
    if let Some(field) = item_field_schema(schema, section_id, item_ids, field_key)
        && let Err(why) = check_value(field, &Value::Scalar(new_value.to_string()))
    {
        return Err(GenerateError::MalformedValue { why });
    }
    let doc = parse::parse_sections(schema, source).map_err(|_| GenerateError::WrongShape {
        what: format!("source does not conform to schema for section {section_id:?}"),
    })?;
    let item = nested_parsed_item(schema, &doc, section_id, item_ids).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!("item {item_ids:?} in section {section_id:?} not present"),
        }
    })?;
    let physical = physical_item_chain(schema, section_id, item_ids).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!("item {item_ids:?} in section {section_id:?} not present"),
        }
    })?;
    let region = locate_item_path(schema, source, section_id, &physical).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!("item {item_ids:?} block not locatable"),
        }
    })?;

    let mut content = item_content_from_parsed(item, source);
    if let Some(existing) = content.fields.iter_mut().find(|f| f.key == field_key) {
        existing.value = Value::Scalar(new_value.to_string());
    } else {
        content.fields.push(Field {
            key: field_key.to_string(),
            value: Value::Scalar(new_value.to_string()),
        });
    }
    let rendered = render_item_at(&content, physical.len());
    Ok(splice(
        source,
        region.clone(),
        &nested_replacement(source, &region, &rendered),
    ))
}

/// `add-item` into a (possibly nested) repeatable, addressed by the **parent** item id
/// chain `parent_item_ids` plus the `nested_section_id` naming which nested repeatable
/// receives the item (`["1-2-0"]`, `"changes"` → a new change-group under release
/// `1-2-0`). The depth-aware dual of [`add_item`]: it re-derives the parent item,
/// appends a freshly-minted nested [`ItemContent`] (mint-empty / multi-slot skeleton per
/// the nested block's template), re-renders the parent at its depth, and splices it back
/// canonically. A minted id colliding with a present nested item under that parent →
/// [`GenerateError::AlreadyPresent`]; an unslugable title → [`GenerateError::UnslugableTitle`].
#[allow(clippy::too_many_arguments)]
pub fn add_nested_item(
    schema: &Schema,
    source: &str,
    section_id: &str,
    parent_item_ids: &[&str],
    nested_section_id: &str,
    title: &str,
    slot: Option<&str>,
    fields: &[Field],
) -> Result<String, GenerateError> {
    let doc = parse::parse_sections(schema, source).map_err(|_| GenerateError::WrongShape {
        what: format!("source does not conform to schema for section {section_id:?}"),
    })?;
    let parent =
        nested_parsed_item(schema, &doc, section_id, parent_item_ids).ok_or_else(|| {
            GenerateError::WrongShape {
                what: format!(
                    "parent item {parent_item_ids:?} in section {section_id:?} not present"
                ),
            }
        })?;
    let parent_physical =
        physical_item_chain(schema, section_id, parent_item_ids).ok_or_else(|| {
            GenerateError::WrongShape {
                what: format!(
                    "parent item {parent_item_ids:?} in section {section_id:?} not present"
                ),
            }
        })?;

    // The nested repeatable named by `nested_section_id` (the leaf id of a
    // `Leaf::Repeatable` in the parent's block), resolved through the schema by walking
    // the section-qualified parent chain in the section tree.
    let nested = nested_repeatable(schema, section_id, parent_item_ids, nested_section_id)
        .ok_or_else(|| GenerateError::WrongShape {
            what: format!(
                "nested repeatable {nested_section_id:?} not declared under {parent_item_ids:?}"
            ),
        })?;

    let id = crate::slug::slugify(title);
    if id.is_empty() {
        return Err(GenerateError::UnslugableTitle {
            title: title.to_string(),
        });
    }
    if parent.items.iter().any(|i| i.id == id) {
        return Err(GenerateError::AlreadyPresent {
            what: format!("nested item {id:?} under {parent_item_ids:?}"),
        });
    }

    let template = parse::ItemTemplate::from(&nested);
    let new_item = if template.is_multi_slot() {
        ItemContent {
            id: id.clone(),
            title: title.to_string(),
            slot: None,
            slots: template
                .slot_ids
                .iter()
                .map(|leaf| (leaf.clone(), String::new()))
                .collect(),
            fields: fields.to_vec(),
            items: Vec::new(),
        }
    } else {
        ItemContent {
            id: id.clone(),
            title: title.to_string(),
            slot: slot.map(str::to_string),
            slots: Vec::new(),
            fields: fields.to_vec(),
            items: Vec::new(),
        }
    };

    let region =
        locate_item_path(schema, source, section_id, &parent_physical).ok_or_else(|| {
            GenerateError::WrongShape {
                what: format!("parent item {parent_item_ids:?} block not locatable"),
            }
        })?;
    let mut content = item_content_from_parsed(parent, source);
    content.items.push(new_item);
    let rendered = render_item_at(&content, parent_physical.len());
    Ok(splice(
        source,
        region.clone(),
        &nested_replacement(source, &region, &rendered),
    ))
}

/// `remove-item` for a (possibly nested) repeatable item, addressed by its parent-scoped
/// id chain `item_ids` (`["1-2-0", "changes", "added"]`). The depth-aware dual of
/// [`remove_item`]: it re-parses for conformance, asserts the nested item is present via
/// [`nested_parsed_item`] (so a same-anchor sibling under a *different* parent is never
/// the target), reduces the chain to its physical form, locates the item's OWN byte block
/// via [`locate_item_path`], and splices that span to empty — the nested analogue of
/// top-level [`remove_item`]'s `splice(src, span, "")`.
///
/// A non-last nested sibling's region already absorbs its trailing blank-line separator
/// (it ends at the next heading start within the parent), so splice-to-empty leaves the
/// survivors canonically spaced. The genuinely LAST block of the document has no trailing
/// separator to absorb (its region ends at EOF), so the splice extends back over the one
/// `\n` of the *leading* separator — otherwise that blank line would dangle and break
/// `render(parse(out)) == out`. An absent item / non-conformant source → [`SpliceError`].
pub fn remove_nested_item(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
) -> Result<String, SpliceError> {
    let doc = parse::parse_sections(schema, source).map_err(|_| SpliceError::NotConformant)?;
    if nested_parsed_item(schema, &doc, section_id, item_ids).is_none() {
        return Err(SpliceError::NotPresent {
            what: format!("item {item_ids:?} in section {section_id:?}"),
        });
    }
    let physical = physical_item_chain(schema, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {item_ids:?} in section {section_id:?}"),
        }
    })?;
    let region = locate_item_path(schema, source, section_id, &physical).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {item_ids:?} block"),
        }
    })?;
    // LAST-block edge: a region ending at EOF carries no trailing separator, so consume
    // the one `\n` of the preceding blank-line separator to avoid a dangling blank line.
    let start = if region.end == source.len() && source[..region.start].ends_with('\n') {
        region.start - 1
    } else {
        region.start
    };
    Ok(splice(source, start..region.end, ""))
}

/// The nested [`crate::schema::Repeatable`] named `nested_section_id` declared in the
/// item block reached by walking the **section-qualified** `parent_item_ids` chain from
/// `section_id` — the schema dual of [`nested_parsed_item`]. The chain
/// (`parent_item_ids` ++ `nested_section_id`) alternates `item, nested-section, …`
/// starting at an item; we descend through the schema by the **named** nested-section id
/// each time (review LOW finding #5 — *not* "the first `Leaf::Repeatable`", which would
/// silently pick the wrong nested section if a block ever declared two). `None` if any
/// nested-section segment names no declared nested repeatable at its level.
///
/// `pub` so the CLI's `add-item` write-time id-from-enum pre-check can resolve the
/// destination nested repeatable the same way the mint path does (review S3 — one
/// navigation, not a CLI copy).
pub fn nested_repeatable(
    schema: &Schema,
    section_id: &str,
    parent_item_ids: &[&str],
    nested_section_id: &str,
) -> Option<crate::schema::Repeatable> {
    let section = schema.sections.iter().find(|s| s.id == section_id)?;
    let SectionBody::Repeatable { repeatable } = &section.body else {
        return None;
    };
    // The section-qualified chain to the nested section: the parent items + the named
    // nested section. It begins with the top item (the section's own repeatable, already
    // in hand) and thereafter alternates nested-section, item, …, ending at the named
    // nested section. We descend by each **named** nested-section segment — the segments
    // at odd indices (0-indexed) of the full chain — and skip the item-id segments.
    let mut chain = parent_item_ids.to_vec();
    chain.push(nested_section_id);
    let mut current = repeatable;
    for segment in chain.iter().skip(1).step_by(2) {
        current = current.block.iter().find_map(|leaf| match leaf {
            crate::schema::Leaf::Repeatable { id, repeatable } if id == *segment => {
                Some(repeatable)
            }
            _ => None,
        })?;
    }
    Some(current.clone())
}

/// Build the splice replacement for a re-rendered nested item over its located
/// `region` in `source`: the rendered bytes (trailing-newline-trimmed) plus the
/// **canonical** inter-block separator — `\n\n` before a following heading, `\n` at EOF
/// — mirroring [`set_item_slot`]'s separator discipline so surrounding items stay
/// canonically spaced and the whole doc round-trips.
fn nested_replacement(source: &str, region: &Range<usize>, rendered: &str) -> String {
    let body = rendered.trim_end_matches('\n');
    if region.end < source.len() {
        format!("{body}\n\n")
    } else {
        format!("{body}\n")
    }
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

/// The **parent-scoped path locator** (review finding S1): resolve the byte region of
/// a (possibly nested) repeatable item addressed by its id chain, walking each segment
/// **within its parent's region** rather than matching by anchor globally.
///
/// `item_ids` is the item id chain from the section root: `["1-2-0"]` for a top-level
/// release, `["1-2-0", "added"]` for the `#added` change-group nested inside it. The
/// walk starts at the section's body region, then for each id finds the item heading at
/// that nesting depth's level (depth-1 items are `###`, depth-2 `####`, … —
/// [`item_heading_level`]) whose `{#id}` anchor matches, **bounded to the current
/// parent's region**, and narrows to that item's own block (its heading start to the
/// next heading at the same-or-shallower level within the parent, or the parent's end).
/// The deepest segment's region is returned.
///
/// This replaces the global single-level `locate_item_block` (which matched only `###`,
/// by anchor alone, over the whole source — ambiguous when two parents each carry a
/// same-anchor nested group, and blind to `####`+ items). A chain segment that names no
/// present item within its parent's region yields `None` (so a mis-named parent never
/// misfires onto a same-anchor item under a *different* parent).
pub fn locate_item_path(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
) -> Option<Range<usize>> {
    if item_ids.is_empty() {
        return None;
    }
    let blocks = parse::scan_blocks(source);
    let present = present_body_sections(schema, source);
    // The section's body region is the depth-1 search scope.
    let mut region = section_region(&blocks, source, section_id, &present)?;

    for (depth0, id) in item_ids.iter().enumerate() {
        let level = item_heading_level(depth0 + 1);
        region = item_block_within(&blocks, source, region.clone(), level, id)?;
    }
    Some(region)
}

/// The byte block of the item at heading `level` whose `{#id}` anchor is `id`, located
/// **within** the parent byte `region`: from its heading start to the next heading at a
/// **same-or-shallower** level inside `region` (the item's sub-tree boundary), or
/// `region.end`. Only headings *inside* `region` are considered, so a same-anchor item
/// under a different parent is out of range. `None` if no such item heading is present.
fn item_block_within(
    blocks: &[Block],
    source: &str,
    region: Range<usize>,
    level: usize,
    id: &str,
) -> Option<Range<usize>> {
    // The matching item heading at `level` within the parent region, by anchor.
    let start = blocks.iter().find_map(|b| match b {
        Block::Heading {
            level: hl, range, ..
        } if level_num_of(*hl) == level
            && range.start >= region.start
            && range.start < region.end
            && anchor_of(&source[range.clone()]) == Some(id) =>
        {
            Some(range.start)
        }
        _ => None,
    })?;
    // The item's block ends at the next heading at the same-or-shallower level *within*
    // the parent region (a sibling item, or a structure that closes this item's
    // sub-tree), else the parent region's end.
    let end = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level: hl, range, ..
            } if range.start > start && range.start < region.end && level_num_of(*hl) <= level => {
                Some(range.start)
            }
            _ => None,
        })
        .min()
        .unwrap_or(region.end);
    Some(start..end)
}

/// The single-level item's **own leaf region** — its full sub-tree `region` (from
/// [`locate_item_path`]) narrowed to `[region.start .. first nested child heading)`,
/// i.e. the bytes the item owns *before* its first nested `####`+ child. A field-group
/// scan/insert for a top-level field must be bounded to this, not the full sub-tree:
/// the sub-tree spans the item's nested children, whose field blocks would otherwise be
/// matched (the parent's field appended INTO a child's group — the corruption this
/// guards). An item with no nested child keeps its whole region (no deeper heading
/// inside it).
///
/// "First nested child" = the first heading inside `region` deeper than the item's own
/// heading (the heading at `region.start`); a same-or-shallower heading would already be
/// outside the item's sub-tree, so [`locate_item_path`] never includes one.
fn item_own_leaf_region(blocks: &[Block], region: Range<usize>) -> Range<usize> {
    let own_level = blocks.iter().find_map(|b| match b {
        Block::Heading { level, range, .. } if range.start == region.start => {
            Some(level_num_of(*level))
        }
        _ => None,
    });
    let Some(own_level) = own_level else {
        return region;
    };
    let first_child = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading { level, range, .. }
                if range.start > region.start
                    && range.start < region.end
                    && level_num_of(*level) > own_level =>
            {
                Some(range.start)
            }
            _ => None,
        })
        .min()
        .unwrap_or(region.end);
    region.start..first_child
}

/// The numeric ATX level of a heading (`H1`→1 … `H6`→6) — the writer-side dual of the
/// parser's `level_num`, used by the parent-scoped locator to compare item depths.
fn level_num_of(level: HeadingLevel) -> usize {
    level as usize
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

/// The `commit` doctype's well-known section/field ids — the Conventional-Commits
/// vocabulary [`render_commit_message`] encodes by design. Centralized + named so
/// the one engine↔pack coupling is explicit, not scattered string literals.
const COMMIT_FIELD_TYPE: &str = "type";
const COMMIT_FIELD_SCOPE: &str = "scope";
const COMMIT_SECTION_SUMMARY: &str = "summary";
const COMMIT_SECTION_BODY: &str = "body";
const COMMIT_SECTION_TRAILERS: &str = "trailers";

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
/// This is the **engine-adjacent projection of the one doctype whose sink is the
/// VCS message** (`CLAUDE.md` → MVP scope): it necessarily encodes the
/// Conventional-Commits shape, so it references the commit doctype's well-known
/// ids by name (the `COMMIT_*` constants above) — a deliberate, documented
/// coupling for the single VCS-sink type, *not* generic schema-driven rendering
/// (`DECISIONS.md` 2026-05-31 → commit-message projection; generalizing to a
/// schema-declared message template is post-MVP). The header section is still
/// located via the schema's `header` flag, never by id. Returns a string with no
/// trailing whitespace and no trailing newline (consumed via `-F`, not appended
/// to a file). Deterministic — same `(schema, instance)` in → identical string out.
pub fn render_commit_message(schema: &Schema, instance: &Instance) -> String {
    let header_id = schema
        .sections
        .iter()
        .find(|s| s.header)
        .map(|s| s.id.as_str());
    let header = header_id.and_then(|id| section_content(instance, id));

    let type_text = header
        .and_then(|c| commit_field(c, COMMIT_FIELD_TYPE))
        .unwrap_or_default();
    let scope_text = header
        .and_then(|c| commit_field(c, COMMIT_FIELD_SCOPE))
        .unwrap_or_default();
    let summary_text = section_content(instance, COMMIT_SECTION_SUMMARY)
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
    if let Some(body) =
        section_content(instance, COMMIT_SECTION_BODY).and_then(|c| c.slot.as_deref())
    {
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
    let Some(section) = section_content(instance, COMMIT_SECTION_TRAILERS) else {
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

/// Reconstruct the in-memory [`Instance`] from a staged doc's `source` bytes — the
/// **parse-to-instance** inverse of [`render`], so a caller (e.g. the `finalize`
/// planner rendering the staged `commit` doc) can recover the writer's input from
/// the file on disk.
///
/// Parses `source` against `schema` ([`parse::parse_sections`]), recovers the H1
/// title from the source, and slices each section's recorded slot span back to its
/// opaque prose (fields and items are carried verbatim). A parse-level conformance
/// failure surfaces the parser's [`Finding`]s. The result is a faithful round-trip
/// pre-image: `render(schema, &instance_from_source(schema, source)?)` reproduces
/// the canonical bytes of a canonical `source`.
pub fn instance_from_source(schema: &Schema, source: &str) -> Result<Instance, Vec<Finding>> {
    let doc = parse::parse_sections(schema, source)?;
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
                .map(|it| item_content_from_parsed(it, source))
                .collect(),
        })
        .collect();
    Ok(Instance { title, sections })
}

/// Re-derive an [`ItemContent`] from a parsed [`parse::ParsedItem`] over `source`,
/// owning each slot's prose (re-slicing the recorded spans). Carries the single-slot
/// bare-prose `slot` or the multi-slot per-leaf `slots`, whichever the parser
/// populated — the inverse the round-trip rests on, used everywhere a parsed item is
/// re-rendered (`instance_from_source`, `set_item_slot`, the `add_item` re-render).
fn item_content_from_parsed(it: &parse::ParsedItem, source: &str) -> ItemContent {
    ItemContent {
        id: it.id.clone(),
        title: it.title.clone(),
        slot: it.slot.as_ref().map(|sp| sp.slice(source).to_string()),
        slots: it
            .slots
            .iter()
            .map(|(leaf, sp)| (leaf.clone(), sp.slice(source).to_string()))
            .collect(),
        fields: it.fields.clone(),
        // The item's nested repeatable items, re-derived one level deeper — the M22
        // recursion that closes the parse→render inverse for a two-level repeatable.
        items: it
            .items
            .iter()
            .map(|nested| item_content_from_parsed(nested, source))
            .collect(),
    }
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
    /// The `add_item` title has no slug-able content, so it would mint an **empty**
    /// `{#}` anchor. `slugify` is total (maps such input to `""`), so the mint site
    /// rejects it rather than emit a malformed item.
    UnslugableTitle {
        /// The rejected title, verbatim.
        title: String,
    },
    /// A repeatable item-field `set-field` value fails its declared schema type (a
    /// non-ISO `date`, a non-member enum, …). The item-field write-time parity of the
    /// header path's [`set_field_validated`] type check, closing the 2026-06-07 gap
    /// (`design/write-commands.md` → Two check times). Mapped to finalize's item-field
    /// code `schema-conformance.field-value-conformant`, so the two check times emit the
    /// identical code.
    MalformedValue {
        /// The type-check failure message from [`check_value`].
        why: String,
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

    // Mint the item anchor from the id-source (the title) via slugify. `slugify` is
    // total: a title with no slug-able content maps to `""`, which would emit a
    // malformed empty `{#}` anchor — reject it here (the only place it can be caught).
    let id = crate::slug::slugify(title);
    if id.is_empty() {
        return Err(GenerateError::UnslugableTitle {
            title: title.to_string(),
        });
    }

    // A **multi-slot** template mints the FULL ordered `#### <Leaf-Title>` skeleton
    // (every slot leaf, empty body, schema order) — the M13 create→fill seam: a later
    // `set-slot .../<leaf>` has an ordered, named target to splice into. A single-slot
    // template keeps the bare-prose form (the `slot` pre-fill, the existing shape).
    let template = match &section.body {
        SectionBody::Repeatable { repeatable } => parse::ItemTemplate::from(repeatable),
        SectionBody::Simple { .. } => unreachable!("repeatability checked above"),
    };
    let item_content = if template.is_multi_slot() {
        ItemContent {
            id: id.clone(),
            title: title.to_string(),
            slot: None,
            slots: template
                .slot_ids
                .iter()
                .map(|leaf| (leaf.clone(), String::new()))
                .collect(),
            fields: fields.to_vec(),
            // A freshly-minted item has no nested items yet (they are added later via
            // their own `add-item` into the nested path — the T5 addressing lift).
            items: Vec::new(),
        }
    } else {
        ItemContent {
            id: id.clone(),
            title: title.to_string(),
            slot: slot.map(str::to_string),
            slots: Vec::new(),
            fields: fields.to_vec(),
            items: Vec::new(),
        }
    };
    let item = render_item(&item_content);

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
        // If the section already has a last item, re-render **it** canonically next to
        // the new item, so the inter-item spacing is exactly [`render_item`]'s — the one
        // source of truth for item bytes. A fixed `\n\n` join is **not** guaranteed
        // byte-stable when the preceding item carries non-canonical trailing blank-line
        // debris (a hand-widened skeleton): the canonical join (`render_section` joins
        // full `render_item`s with `\n`) is the one source of truth, where a fixed-blank
        // insert could mis-space and yield
        // `render(parse(out)) != out`. We splice `[last_item_start .. region.end]` with
        // `render_item(prev)` + `\n` + the new item, re-attaching the region's tail
        // separator (the blank line / `##` boundary / EOF newline) the located region
        // carried.
        if let Some(last) = locate_last_item_block(&blocks, region.clone()) {
            let prev = parse::parse_sections(schema, source)
                .ok()
                .and_then(|doc| {
                    doc.sections
                        .into_iter()
                        .find(|s| s.id == section_id)
                        .and_then(|s| s.items.into_iter().last())
                })
                .ok_or_else(|| GenerateError::WrongShape {
                    what: format!("section {section_id:?} last item not present"),
                })?;
            let prev_content = item_content_from_parsed(&prev, source);
            let tail = &source[last.clone()];
            let separator = &tail[tail.trim_end().len()..];
            // The canonical join `render_section` uses: full `render_item`s joined by a
            // single `\n` (an empty prev item renders its one-blank slotless form).
            // Trim the joined block and re-attach the region's tail separator so the
            // surrounding sections / EOF stay canonically spaced.
            let joined = format!("{}\n{}", render_item(&prev_content), item);
            let replacement = format!("{}{}", joined.trim_end(), separator);
            return Ok(splice(source, last.start..region.end, &replacement));
        }
        // The section is present but has **no items yet**: its body region (just past
        // the `## Heading\n`) is all whitespace up to the next `##`/EOF. Replace that
        // whole empty body with the canonical first item — one blank line under the
        // heading, the item, then the region's original tail separator (the blank
        // before a following `##`, or the single EOF `\n`). This consumes the
        // mint-empty section's blank-line debris so the result is byte-stable whether
        // the section is trailing (EOF) or followed by another `##` (the M22
        // empty-non-last-section append: `## Unreleased Changes` before `## Releases`).
        let tail = &source[region.clone()];
        let separator = &tail[tail.trim_end().len()..];
        // A trailing (EOF) empty section's body carries no separator of its own; the
        // canonical EOF rule is exactly one terminating `\n`.
        let separator = if separator.is_empty() {
            "\n"
        } else {
            separator
        };
        let replacement = format!("\n{}{}", item.trim_end_matches('\n'), separator);
        Ok(splice(source, region.start..region.end, &replacement))
    } else {
        // The section's home is absent: generate the `## Heading` with the first item
        // as its body, inserted at the section's schema-ordered position.
        let block = format!("## {}\n\n{}", heading_text(&section.id), item);
        let at = insertion_offset(schema, source, section_id, &present);
        Ok(insert_block(source, at, &block))
    }
}

/// `set-field` (header field **absent**): materialize the front-matter `key: value`
/// line for `field` at its **schema-ordered position** inside the `---` fence (after
/// the nearest preceding present header field, before the nearest following one), so
/// the present front-matter fields stay in schema order (`parsing.md` → Absent
/// structural homes generate at their schema-ordered position). The optional ref the
/// fillable form omits (`commit#header/implements`) is the driving case. A header
/// field already present → [`GenerateError::AlreadyPresent`] (route to [`set_field`]);
/// a section that is not the header, or a key the schema does not declare for it →
/// [`GenerateError::WrongShape`] / [`GenerateError::UnknownSection`].
pub fn insert_front_matter_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    field: &Field,
) -> Result<String, GenerateError> {
    let section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
        })?;
    if !section.header {
        return Err(GenerateError::WrongShape {
            what: format!("section {section_id:?} is not the header (front-matter)"),
        });
    }

    // The header's declared field order — the schema-ordered key sequence the inserted
    // line slots into.
    let schema_keys: Vec<&str> = match &section.body {
        SectionBody::Simple { fields, .. } => fields.iter().map(|f| f.id.as_str()).collect(),
        SectionBody::Repeatable { .. } => {
            return Err(GenerateError::WrongShape {
                what: format!("section {section_id:?} is repeatable, not a header"),
            });
        }
    };

    // The front-matter content range (between the `---` fences) and the present keys
    // in physical order, located from the same block parse the reader uses.
    let content = front_matter_content(source).ok_or_else(|| GenerateError::WrongShape {
        what: format!("section {section_id:?} has no front-matter block to insert into"),
    })?;
    if field_value_in_lines(source, content.clone(), &field.key, false).is_some() {
        return Err(GenerateError::AlreadyPresent {
            what: format!("field {:?} in section {section_id:?}", field.key),
        });
    }

    let line = format!("{}\n", emit_one_field(field));
    let at = front_matter_insertion_offset(source, content, &schema_keys, &field.key);
    Ok(insert_at(source, at, &line))
}

/// The inner content range of the front-matter `---` block, if present.
///
/// A populated front matter parses to a [`Block::Metadata`] whose content is the
/// `key: value` lines between the fences. An **empty** fence pair (`---\n---`) carries
/// no inner text, so pulldown-cmark emits no metadata block — yet the structural home
/// for a header field *does* exist (the create flow renders this fence pair before any
/// header field is materialized; `parsing.md` → Absent structural homes). We detect that
/// degenerate case directly and return the **zero-width** content range just inside the
/// opening fence, so a first header field inserts between the fences rather than failing
/// with "no front-matter block".
fn front_matter_content(source: &str) -> Option<Range<usize>> {
    if let Some(content) = parse::scan_blocks(source)
        .into_iter()
        .find_map(|b| match b {
            Block::Metadata { content } => Some(content),
            _ => None,
        })
    {
        return Some(content);
    }
    empty_front_matter_content(source)
}

/// The zero-width inner-content range of an **empty** front-matter fence pair — a
/// leading `---` fence immediately followed by a closing `---` fence with nothing
/// between them. Returns the offset just past the opening fence's newline (where a
/// first header field line slots in), or `None` when the source has no such empty
/// fence pair at its head.
fn empty_front_matter_content(source: &str) -> Option<Range<usize>> {
    // The opening fence must be the very first line (front matter is top-of-file).
    let mut lines = source.split_inclusive('\n');
    let first = lines.next()?;
    if first.trim_end_matches(['\r', '\n']) != "---" {
        return None;
    }
    let second = lines.next()?;
    if second.trim_end_matches(['\r', '\n']) != "---" {
        return None;
    }
    // Inner content is the empty span between the fences: the offset just after the
    // opening fence line (== the closing fence's start).
    let at = first.len();
    Some(at..at)
}

/// The byte offset at which to insert a new front-matter field line so the present
/// header fields stay in **schema order**: the line-start of the nearest *following*
/// present field (the first present field whose schema index exceeds the target's),
/// else the end of the front-matter content (just before the closing `---`).
fn front_matter_insertion_offset(
    source: &str,
    content: Range<usize>,
    schema_keys: &[&str],
    target_key: &str,
) -> usize {
    let order = |key: &str| schema_keys.iter().position(|k| *k == key);
    let target_idx = order(target_key);
    let text = &source[content.clone()];
    let mut line_start = content.start;
    for line in text.split_inclusive('\n') {
        let bare = line.trim_end_matches('\n').trim_start();
        if let Some((key, _)) = bare.split_once(':')
            && order(key.trim()) > target_idx
        {
            return line_start;
        }
        line_start += line.len();
    }
    // No following present field: append at the content's end (the metadata content
    // range ends just before the closing `---` fence line).
    content.end
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

/// `set-item-field` (field **absent**, item present): insert the field bullet for
/// `field` into the repeatable item `item_id`'s trailing field group, materializing
/// the `<!-- fields -->` sentinel **once** if the item has no field group yet (the
/// "first field into a freshly-minted empty item" case — [`add_item`] mints items
/// empty, so the item-leaf write path needs this insert half just as the header path
/// has [`insert_front_matter_field`]). The field-group scan is confined to the item's
/// OWN leaf region ([`locate_item_path`] narrowed by [`item_own_leaf_region`]), so
/// neither a sibling item's identically-keyed bullet (the wrong-item write bug) nor a
/// nested child's field group (the parent-region-swallows-child bug) is in range. The
/// bullet is appended
/// after the item's present bullets, mirroring [`insert_field`]. A field key already
/// present on the item → [`GenerateError::AlreadyPresent`] (route to [`set_item_field`]).
pub fn insert_item_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_id: &str,
    field: &Field,
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

    // The item's byte region, resolved through the parent-scoped path locator (a
    // single-element chain). The field-group scan is bounded to the item's OWN leaf
    // region (before its first nested `####` child) so a nested child's field group is
    // never matched — without this the parent's field is appended INTO a child's group
    // (the corruption) or the child's same key triggers a false `AlreadyPresent`. The
    // cold-fill re-render below uses the FULL sub-tree region (children preserved).
    let blocks = parse::scan_blocks(source);
    let region = locate_item_path(schema, source, section_id, &[item_id]).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!("item {item_id:?} in section {section_id:?} not present"),
        }
    })?;
    let leaf_region = item_own_leaf_region(&blocks, region.clone());

    let bullet = format!("- {}", emit_one_field(field));

    // Is there an existing field group (a sentinel + list) inside the item's OWN region?
    match field_group_list(&blocks, leaf_region.clone()) {
        Some((list_range, items)) => {
            // A present key is a surgical set-item-field, not a generation.
            if field_key_in_list(&blocks, source, &items, &field.key) {
                return Err(GenerateError::AlreadyPresent {
                    what: format!("field {:?} on item {item_id:?}", field.key),
                });
            }
            // Append the bullet right after the last present bullet (the list end).
            Ok(insert_after_line(source, list_range.end, &bullet))
        }
        None => {
            // No field group yet (the freshly-minted empty item): re-render the whole
            // item via [`render_item`] with the new field appended, then splice it over
            // the item's located block. Re-rendering (not a bullet-group splice at the
            // slot-prose end) is what makes this **byte-stable** on a mint-empty item:
            // an empty slot's canonical form (`### …{#id}\n\n<!-- fields -->`, the M26
            // one-blank form) is the writer's, and `render_item` is that writer — so `render(parse(out)) == out`
            // holds, where a slot-prose-end bullet insert would mis-space the blank lines.
            let item = parse::parse_sections(schema, source)
                .ok()
                .and_then(|doc| {
                    doc.sections
                        .into_iter()
                        .find(|s| s.id == section_id)
                        .and_then(|s| s.items.into_iter().find(|i| i.id == item_id))
                })
                .ok_or_else(|| GenerateError::WrongShape {
                    what: format!("item {item_id:?} in section {section_id:?} not present"),
                })?;
            let mut content = item_content_from_parsed(&item, source);
            content.fields.push(field.clone());
            // Re-render the item, then re-attach the **canonical** inter-block separator
            // (not the recorded one): the located region may carry non-canonical trailing
            // blank-line debris, so we recompute the gap from the canonical form — one
            // blank line. [`render_section`] joins full
            // `render_item`s (each ending in one `\n`) with a single `\n`, so a following
            // `###`/`##` heading gets `body\n\n`; a trailing item gets `body\n` (EOF).
            let rendered = render_item(&content);
            let body = rendered.trim_end_matches('\n');
            let replacement = if region.end < source.len() {
                format!("{body}\n\n")
            } else {
                format!("{body}\n")
            };
            Ok(splice(source, region, &replacement))
        }
    }
}

/// The **one entry point** the CLI's `UnitItemLeaf` `set-field` arm calls: write a
/// repeatable item's field value, **splicing if the bullet is present** (the surgical
/// [`set_item_field`] edit) **else inserting it absent** (the new [`insert_item_field`]
/// generation). [`add_item`] mints items *empty* (heading + empty slot span, no field
/// bullets), so the first write of any declared item field always lands on the
/// insert-absent half — exactly the header path's splice-or-generate shape
/// ([`set_field`] ↔ [`insert_front_matter_field`]). It **adjudicates the value's
/// declared type before touching bytes** (the item-field write-time parity of the
/// header path's [`set_field_validated`], closing the 2026-06-07 gap —
/// `design/write-commands.md` → Two check times): a value failing its declared type is
/// rejected as [`GenerateError::MalformedValue`]. A genuinely absent item or a
/// non-repeatable section surfaces as [`GenerateError::WrongShape`].
pub fn set_item_field_or_insert(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_id: &str,
    field_key: &str,
    new_value: &str,
) -> Result<String, GenerateError> {
    if let Some(field) = item_field_schema(schema, section_id, &[item_id], field_key)
        && let Err(why) = check_value(field, &Value::Scalar(new_value.to_string()))
    {
        return Err(GenerateError::MalformedValue { why });
    }
    match set_item_field(schema, source, section_id, item_id, field_key, new_value) {
        Ok(edited) => Ok(edited),
        // The field bullet is absent on a present item ⇒ generate it. (`set_item_field`
        // returns `NotPresent` both for an absent field *and* an absent item;
        // `insert_item_field` re-locates the item and routes a truly-absent item to a
        // `WrongShape`, so the two absence cases stay distinguishable.)
        Err(SpliceError::NotPresent { .. }) => {
            let new_field = Field {
                key: field_key.to_string(),
                value: Value::Scalar(new_value.to_string()),
            };
            insert_item_field(schema, source, section_id, item_id, &new_field)
        }
        Err(SpliceError::NotConformant) => Err(GenerateError::WrongShape {
            what: format!("source does not conform to schema for section {section_id:?}"),
        }),
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
                .find(|id| crate::slug::slugify(text) == **id)
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
        // Append at end-of-document: one blank line, the block, and **exactly one**
        // trailing newline (the canonical EOF rule). The block is back-trimmed first so
        // a block that already carries its own trailing newline(s) — an empty repeatable
        // item renders to its one-blank slotless form (`### …{#id}\n`) — does not leave
        // trailing blank-line debris, keeping the EOF-append byte-stable (`render∘parse == id`).
        let head = source.trim_end();
        let block = block.trim_end_matches('\n');
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

/// The byte block of the **last** `### …` item in a repeatable section `region`:
/// `[last_item_heading_start, region.end)`, or `None` when the section has no items
/// yet. Used by [`add_item`] to re-render the preceding item canonically next to a new
/// one, so the inter-item spacing is [`render_item`]'s (one source of item bytes).
fn locate_last_item_block(blocks: &[Block], region: Range<usize>) -> Option<Range<usize>> {
    let last_start = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level: pulldown_cmark::HeadingLevel::H3,
                range,
                ..
            } if range.start >= region.start && range.start < region.end => Some(range.start),
            _ => None,
        })
        .max()?;
    Some(last_start..region.end)
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
    use crate::schema::Schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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
        crate::schema::load_schema_with_types(yaml, &crate::schema::dev_pack_field_types())
            .expect("spec schema loads")
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

    /// Removing the **last** item must round-trip byte-stable — the last item's
    /// block ends at EOF, so splicing it to empty leaves the preceding blank-line
    /// separator (`\n\n`) dangling unless the last-block edge is consumed (the same
    /// edge `remove_nested_item` already handles). Regression guard for the M24
    /// Increment 3 halt: `remove_item` was byte-stable only for non-last items.
    #[test]
    fn remove_last_item_round_trips_byte_stable() {
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
        let out =
            remove_item(&spec_schema(), src, "criteria", "burst-allowance").expect("item present");
        // The surviving doc round-trips byte-stable (no dangling trailing blank line).
        let reparsed = instance_from_source(&spec_schema(), &out).expect("result still conforms");
        assert_eq!(
            render(&spec_schema(), &reparsed),
            out,
            "removing the last item must leave a byte-stable document"
        );
        assert!(out.contains("### Rate limit holds  {#rate-limit}"));
        assert!(!out.contains("burst-allowance"));
    }

    /// A repeatable-block schema whose item template carries a plain `string` field
    /// (`implemented-by`) shared across items — the disambiguation surface for
    /// item-scoped field writes (no code-anchor validation to obscure the byte test).
    fn linked_spec_schema() -> Schema {
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
        - { id: implemented-by, type: string }
";
        crate::schema::load_schema_with_types(yaml, &crate::schema::dev_pack_field_types())
            .expect("linked spec schema loads")
    }

    /// A two-item repeatable fixture where both items A and B carry an
    /// identically-keyed `- implemented-by:` field with distinct values — the
    /// wrong-item disambiguation fixture.
    const TWO_ITEM_SPEC: &str = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Rate limit holds  {#rate-limit}

The gateway rejects the 101st request.

<!-- fields -->
- implemented-by: src/gateway.rs

### Burst allowance  {#burst-allowance}

A short burst is tolerated.

<!-- fields -->
- implemented-by: src/burst.rs
";

    /// `set_item_field` on item B sets **B's** `implemented-by` to the new value and
    /// leaves item A's identically-keyed field byte-for-byte untouched — the
    /// wrong-item write bug retired. Golden pins the full result; the line check
    /// asserts only B's field-value line differs; the inverse (set on A) proves the
    /// test is not order-trivial.
    #[test]
    fn set_item_field_targets_the_addressed_item() {
        let schema = linked_spec_schema();
        let out = set_item_field(
            &schema,
            TWO_ITEM_SPEC,
            "criteria",
            "burst-allowance",
            "implemented-by",
            "src/burst_v2.rs",
        )
        .expect("item B field present");
        insta::assert_snapshot!("set_item_field_b", out);
        // B's value updated; A's identical-keyed field unchanged.
        assert!(out.contains("- implemented-by: src/burst_v2.rs"));
        assert!(out.contains("- implemented-by: src/gateway.rs"));
        // Surgical: only B's `implemented-by` value-line index 22 differs.
        let b: Vec<&str> = TWO_ITEM_SPEC.lines().collect();
        let a: Vec<&str> = out.lines().collect();
        assert_eq!(b.len(), a.len(), "line count unchanged");
        let b_idx = b
            .iter()
            .position(|l| *l == "- implemented-by: src/burst.rs")
            .unwrap();
        for (i, (bl, al)) in b.iter().zip(a.iter()).enumerate() {
            if i == b_idx {
                assert_ne!(bl, al, "B's value line must change");
            } else {
                assert_eq!(bl, al, "line {i} must be byte-identical");
            }
        }
        // The result still round-trips byte-identical: render(parse(out)) == out.
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(render(&schema, &reparsed), out);

        // Inverse: setting A leaves B untouched — not order-trivial.
        let out_a = set_item_field(
            &schema,
            TWO_ITEM_SPEC,
            "criteria",
            "rate-limit",
            "implemented-by",
            "src/gateway_v2.rs",
        )
        .expect("item A field present");
        assert!(out_a.contains("- implemented-by: src/gateway_v2.rs"));
        assert!(out_a.contains("- implemented-by: src/burst.rs"));
        let a_idx = b
            .iter()
            .position(|l| *l == "- implemented-by: src/gateway.rs")
            .unwrap();
        let aa: Vec<&str> = out_a.lines().collect();
        for (i, (bl, al)) in b.iter().zip(aa.iter()).enumerate() {
            if i == a_idx {
                assert_ne!(bl, al, "A's value line must change");
            } else {
                assert_eq!(bl, al, "line {i} must be byte-identical");
            }
        }
    }

    /// An absent item / absent field on an item routes to [`SpliceError::NotPresent`].
    #[test]
    fn set_item_field_absent_is_not_present() {
        let schema = linked_spec_schema();
        let missing_item = set_item_field(
            &schema,
            TWO_ITEM_SPEC,
            "criteria",
            "ghost",
            "implemented-by",
            "x",
        )
        .expect_err("no such item");
        assert!(matches!(missing_item, SpliceError::NotPresent { .. }));
        let missing_field = set_item_field(
            &schema,
            TWO_ITEM_SPEC,
            "criteria",
            "rate-limit",
            "nonesuch",
            "x",
        )
        .expect_err("no such field on item");
        assert!(matches!(missing_field, SpliceError::NotPresent { .. }));
    }

    /// `set_item_slot` on item B replaces **B's** per-item slot prose and leaves item
    /// A's identically-shaped slot prose byte-for-byte untouched — the wrong-item slot
    /// write bug retired. Golden pins the full result; the inverse (set on A) proves
    /// the test is not order-trivial; `render(parse(out)) == out` confirms round-trip.
    #[test]
    fn set_item_slot_targets_the_addressed_item() {
        let schema = linked_spec_schema();
        let out = set_item_slot(
            &schema,
            TWO_ITEM_SPEC,
            "criteria",
            "burst-allowance",
            "statement",
            "A short burst is tolerated for two seconds.",
        )
        .expect("item B slot present");
        insta::assert_snapshot!("set_item_slot_b", out);
        // (a) B's slot prose is the new text.
        assert!(out.contains("A short burst is tolerated for two seconds."));
        // (b) A's slot prose is byte-for-byte untouched.
        assert!(out.contains("The gateway rejects the 101st request."));
        // The old B prose is gone.
        assert!(!out.contains("A short burst is tolerated.\n"));
        // (c) The result round-trips byte-identical: render(parse(out)) == out.
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(render(&schema, &reparsed), out);

        // Inverse: setting A leaves B untouched — not order-trivial.
        let out_a = set_item_slot(
            &schema,
            TWO_ITEM_SPEC,
            "criteria",
            "rate-limit",
            "statement",
            "The gateway rejects the 101st request in a 60s window.",
        )
        .expect("item A slot present");
        assert!(out_a.contains("The gateway rejects the 101st request in a 60s window."));
        // B's original slot prose untouched.
        assert!(out_a.contains("A short burst is tolerated."));
        let reparsed_a = instance_from_source(&schema, &out_a).expect("result conforms");
        assert_eq!(render(&schema, &reparsed_a), out_a);
    }

    /// An absent item / a section with no per-item slot routes to
    /// [`SpliceError::NotPresent`].
    #[test]
    fn set_item_slot_absent_is_not_present() {
        let schema = linked_spec_schema();
        let missing_item = set_item_slot(
            &schema,
            TWO_ITEM_SPEC,
            "criteria",
            "ghost",
            "statement",
            "x",
        )
        .expect_err("no item");
        assert!(matches!(missing_item, SpliceError::NotPresent { .. }));
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
    use crate::schema::Schema;
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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

        /// Byte-stable on a `set-slot`: changing the `decision` slot over an arbitrary
        /// conformant ADR yields a **canonical** buffer (`render(parse(out)) == out`)
        /// carrying the new prose, with every byte *outside* this section's body region
        /// untouched. Re-pinned (M13 audit HIGH): `set_slot` now re-renders the section
        /// canonically instead of splicing the bare recorded slot span — so a prior
        /// `assert_diff_confined_to(recorded-span)` + verbatim-placement assertion no
        /// longer holds (the canonical render trims trailing slot whitespace and
        /// canonicalizes surrounding blanks, which the bare-span splice did **not** — the
        /// defect). Byte-stability is the stronger, correct invariant here.
        #[test]
        fn set_slot_into_decision_is_byte_stable(
            context in prose(),
            decision in prose(),
            consequences in prose(),
            new_prose in prose(),
        ) {
            // The fixture must be canonical so whole-doc byte-stability is a valid
            // assertion: the `prose()` strategy can emit trailing whitespace, which the
            // canonical writer trims — so a fixture section `set_slot` does *not* touch
            // (context / consequences) would otherwise make `render(parse(out)) != out`
            // for reasons unrelated to the slot under test. Trim each to its canonical form.
            let context = context.trim_end();
            let decision = decision.trim_end();
            let consequences = consequences.trim_end();
            let src = build_adr("proposed", context, decision, consequences);
            let schema = adr_schema();
            let out = set_slot(&schema, &src, "decision", &new_prose).expect("decision present");
            // The new prose (trimmed, as the canonical writer emits it) is present.
            let want_prose = new_prose.trim_end();
            prop_assert!(out.contains(want_prose));
            // Every byte outside the `## Decision` body region is untouched: the
            // front-matter, the `## Context` body, and the `## Consequences` heading +
            // body all survive verbatim.
            let want_context = format!("## Context\n\n{context}");
            let want_consequences = format!("## Consequences\n\n{consequences}");
            prop_assert!(out.contains(&want_context));
            prop_assert!(out.contains(&want_consequences));
            // Round-trips byte-identical: render(parse(out)) == out.
            let reparsed = instance_from_source(&schema, &out).expect("result conforms");
            prop_assert_eq!(render(&schema, &reparsed), out);
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
    use crate::schema::Schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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
        crate::schema::load_schema_with_types(yaml, &crate::schema::dev_pack_field_types())
            .expect("spec schema loads")
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
        crate::schema::load_schema(yaml).expect("fielded schema loads")
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

    /// `add-item` with a title that slugs to `""` (no slug-able content) must
    /// reject with [`GenerateError::UnslugableTitle`] rather than minting an empty
    /// `{#}` anchor — `slugify` is total, so the mint site is the only place this
    /// can be caught. Covers an empty title and a punctuation-only one (M13 audit
    /// LOW).
    #[test]
    fn add_item_unslugable_title_is_rejected() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        for title in ["", "###", "!!!___---"] {
            let err = add_item(&spec_schema(), src, "criteria", title, Some("x"), &[])
                .expect_err("an unslugable title must not mint an empty anchor");
            assert!(
                matches!(err, GenerateError::UnslugableTitle { .. }),
                "title {title:?} should be UnslugableTitle, got {err:?}"
            );
        }
    }

    /// `add-item` into an **empty trailing** repeatable section (the `criteria`
    /// section is last and has no items yet — the exact shape `jigc doc create spec`
    /// leaves on disk before the first `add-item`) mint-empty (`--title` only) must be
    /// **byte-stable**: `render(parse(out)) == out`. The EOF-append path must not leave
    /// the trailing blank-line debris an un-normalized splice produces — the canonical
    /// EOF rule (exactly one trailing `\n`) holds across the insert. This is the #1-risk
    /// round-trip on the `add_item` insert path, which the populated golden masked
    /// (insta trims trailing whitespace; a raw byte compare does not).
    #[test]
    fn add_item_into_empty_trailing_section_is_byte_stable() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        let out = add_item(
            &spec_schema(),
            src,
            "criteria",
            "Rate limit holds",
            None,
            &[],
        )
        .expect("item generated into the empty trailing section");
        assert!(out.contains("### Rate limit holds  {#rate-limit-holds}"));
        let doc = parse_sections(&spec_schema(), &out).expect("result conforms");
        let title = out
            .lines()
            .find_map(|l| l.strip_prefix("# "))
            .unwrap_or("")
            .to_string();
        let reparsed = Instance {
            title,
            sections: doc
                .sections
                .iter()
                .map(|s| SectionContent {
                    id: s.id.clone(),
                    slot: s.slot.as_ref().map(|sp| sp.slice(&out).to_string()),
                    fields: s.fields.clone(),
                    items: s
                        .items
                        .iter()
                        .map(|it| item_content_from_parsed(it, &out))
                        .collect(),
                })
                .collect(),
        };
        let rerendered = render(&spec_schema(), &reparsed);
        assert_eq!(
            rerendered, out,
            "add_item into an empty trailing section is byte-stable (render∘parse == id)",
        );
    }

    /// `add-item` into a present **multi-word** section id (`## Unreleased Changes`,
    /// id `unreleased-changes`) appends under the existing heading rather than
    /// generating a duplicate `## Unreleased Changes` home. The `present_body_sections`
    /// heading↔id match must re-slug the heading (`slugify(text) == id`), the same
    /// convention `parse::heading_matches` adopted for the multi-word-section-id fix —
    /// a case-insensitive flat compare reports the hyphenated id absent and duplicates
    /// the section (the M22 inc-5 cold-create defect). The result round-trips
    /// byte-stable. See `design/changelog.md` → engine work #2.
    #[test]
    fn add_item_into_present_multi_word_section_does_not_duplicate_the_heading() {
        let yaml = b"\
type: log
id-from: title
sections:
  - id: unreleased-changes
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: note, slot: {} }
";
        let schema = crate::schema::load_schema(yaml).expect("schema loads");
        let src = "\
# Log

## Unreleased Changes
";
        let out = add_item(&schema, src, "unreleased-changes", "Fixed", None, &[])
            .expect("item appends under the present multi-word section");
        assert_eq!(
            out.matches("## Unreleased Changes").count(),
            1,
            "the multi-word section heading must NOT be duplicated; got:\n{out}",
        );
        assert!(
            out.contains("### Fixed  {#fixed}"),
            "the item is appended under the existing section; got:\n{out}",
        );
        // Byte-stable: render(parse(out)) == out.
        let reparsed = instance_from_source(&schema, &out).expect("result re-parses");
        assert_eq!(
            render(&schema, &reparsed),
            out,
            "add_item under a multi-word section is byte-stable",
        );
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
        let schema = crate::schema::load_schema(yaml).expect("schema loads");
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

    /// A repeatable schema whose item template declares a plain-`string` field
    /// (`implemented-by`) — the fixture for filling a **freshly-minted empty item**'s
    /// field through the insert-absent path (no code-anchor validation to obscure the
    /// byte test).
    fn linked_spec_schema() -> Schema {
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
        - { id: implemented-by, type: string }
";
        crate::schema::load_schema_with_types(yaml, &crate::schema::dev_pack_field_types())
            .expect("linked spec schema loads")
    }

    /// The end-to-end reachability the M13 halt named: `add_item` mints an **empty**
    /// item (heading + empty slot span, no field bullets), then
    /// [`set_item_field_or_insert`] **generates** the absent `- implemented-by:` bullet
    /// (the `<!-- fields -->` sentinel emitted **once**) at the item's field-group
    /// position with exactly the set value — while a *second* minted item's
    /// identically-keyed field stays absent (multi-item disambiguation through the
    /// insert path) — and the result round-trips byte-identical (`render(parse(out)) ==
    /// out`). This is the insert-absent half that `set_item_field` (splice-only) could
    /// not reach.
    #[test]
    fn set_item_field_generates_absent_bullet_on_minted_item() {
        let schema = linked_spec_schema();
        let base = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        // Mint two empty items (mint-empty: heading + empty slot span, no bullets).
        let one = add_item(&schema, base, "criteria", "Rate limit holds", None, &[])
            .expect("first item minted empty");
        let two = add_item(&schema, &one, "criteria", "Burst allowance", None, &[])
            .expect("second item minted empty");
        assert!(
            !two.contains(FIELD_SENTINEL),
            "minted items carry no field group",
        );

        // Fill the first item's declared field through the single entry point — the
        // bullet is absent, so this exercises the insert-absent (generation) half.
        let out = set_item_field_or_insert(
            &schema,
            &two,
            "criteria",
            "rate-limit-holds",
            "implemented-by",
            "src/gateway.rs",
        )
        .expect("absent item field generated");

        // The sentinel is emitted exactly once, the bullet carries the exact value.
        assert_eq!(out.matches(FIELD_SENTINEL).count(), 1, "one sentinel");
        assert!(out.contains("- implemented-by: src/gateway.rs"));

        // Multi-item disambiguation: the second item's identically-keyed field is
        // still absent (untouched by the first item's insert).
        let doc = parse_sections(&schema, &out).expect("result conforms");
        let criteria = doc.sections.iter().find(|s| s.id == "criteria").unwrap();
        let first = criteria
            .items
            .iter()
            .find(|i| i.id == "rate-limit-holds")
            .unwrap();
        let second = criteria
            .items
            .iter()
            .find(|i| i.id == "burst-allowance")
            .unwrap();
        assert_eq!(first.fields.len(), 1, "first item has the generated field");
        assert_eq!(first.fields[0].key, "implemented-by");
        assert_eq!(
            value_text(&first.fields[0].value),
            "src/gateway.rs",
            "the value is exactly as set",
        );
        assert!(second.fields.is_empty(), "second item's field stays absent");

        // Round-trips byte-identical: render(parse(out)) == out.
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(render(&schema, &reparsed), out, "byte-stable insert");
    }

    /// Regression (the increment-2 behavior): when the item field bullet is **already
    /// present**, [`set_item_field_or_insert`] splices it (the surgical edit) rather
    /// than inserting a duplicate — the present half of splice-or-insert. The sentinel
    /// count stays one and the value is replaced in place.
    #[test]
    fn set_item_field_or_insert_splices_a_present_bullet() {
        let schema = linked_spec_schema();
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Rate limit holds  {#rate-limit-holds}

The gateway rejects the 101st request.

<!-- fields -->
- implemented-by: src/gateway.rs
";
        let out = set_item_field_or_insert(
            &schema,
            src,
            "criteria",
            "rate-limit-holds",
            "implemented-by",
            "src/gateway_v2.rs",
        )
        .expect("present bullet spliced");
        assert_eq!(out.matches(FIELD_SENTINEL).count(), 1, "no duplicate group");
        assert_eq!(
            out.matches("- implemented-by:").count(),
            1,
            "no duplicate bullet",
        );
        assert!(out.contains("- implemented-by: src/gateway_v2.rs"));
        assert!(!out.contains("src/gateway.rs"));
        // Byte-stable.
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(render(&schema, &reparsed), out);
    }

    /// Regression (M13 Increment 3): [`set_item_slot`] on a **mint-empty** item must be
    /// byte-stable. `add_item` mints the empty item whose slot span the parser records
    /// *bare* (no canonical surrounding blank lines); splicing prose into that bare span
    /// produced `### H  {#h}\nprose\n### Next` while [`render_item`] emits the blanks —
    /// so `render(parse(out)) != out`. Mint-empty is the *only* state an item-leaf
    /// set-slot runs in. Two empty items are minted and the **first** (non-trailing) item
    /// is filled, so the result exercises a filled non-trailing item against an empty
    /// trailing one.
    #[test]
    fn set_item_slot_on_mint_empty_item_is_byte_stable() {
        let schema = linked_spec_schema();
        let base = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        // Mint two empty items (heading + empty slot span, no bullets).
        let one =
            add_item(&schema, base, "criteria", "Rate limit holds", None, &[]).expect("first mint");
        let two =
            add_item(&schema, &one, "criteria", "Burst allowance", None, &[]).expect("second mint");

        // Fill the FIRST (non-trailing) mint-empty item's slot.
        let out = set_item_slot(
            &schema,
            &two,
            "criteria",
            "rate-limit-holds",
            "statement",
            "The gateway rejects the 101st request.",
        )
        .expect("first item slot filled");
        assert!(out.contains("The gateway rejects the 101st request."));

        // Round-trips byte-identical: render(parse(out)) == out.
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(
            render(&schema, &reparsed),
            out,
            "set_item_slot on a mint-empty item is byte-stable",
        );
    }

    /// Regression (M13 Increment 3): two **empty** `add_item`s with no fill must be
    /// byte-stable. `add_item`'s inter-item join rendered the pair `### A\n\n### B`, but
    /// [`render_item`]'s empty-item form then re-rendered the (now non-trailing) empty item A
    /// as `### A\n\n\n\n### B` (the pre-M26 three-blank form) — so `render(parse(out)) != out`. Existing tests only ever
    /// left the *trailing* item empty, so a non-trailing empty item was never
    /// round-trip-checked.
    #[test]
    fn two_empty_add_items_are_byte_stable() {
        let schema = linked_spec_schema();
        let base = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        let one =
            add_item(&schema, base, "criteria", "Rate limit holds", None, &[]).expect("first mint");
        let out =
            add_item(&schema, &one, "criteria", "Burst allowance", None, &[]).expect("second mint");

        // Round-trips byte-identical: render(parse(out)) == out.
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(
            render(&schema, &reparsed),
            out,
            "two empty add_items are byte-stable",
        );
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
/// a signed integer. `string` and `ref` accept any non-empty single-line opaque value
/// — full `ref` resolution is a `finalize` (edge-index) concern. A **pack-declared**
/// type ([`FieldType::Pack`]) runs its optional *write-time shape check* (for
/// `code-anchor`: non-empty, single-line, parses as `path#symbol`); its real
/// adjudication is the bound finalize-time probe (`design/document-type-schema.md` →
/// Pack-declared field types). A [`Value::List`] is checked element-wise. Returns a
/// human-readable description of the malformation on failure (the [`Finding`] message
/// the caller surfaces).
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
    match &field.ty {
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
        // Non-empty, single-line opaque value; deeper adjudication is a finalize /
        // pack concern. Control chars (newline/tab/…) are rejected so a value can
        // never inject a second field line when spliced onto its `- key: value` line.
        // `owned-location` joins the opaque-scalar floor here: recognition only at
        // the write/conformance layer (its real adjudication — path-safety + durable
        // presence — is the intrinsic finalize-time #5 owner-artifact gate). So
        // `field-value-conformant` never fires for it beyond the shared floor (any
        // non-empty single-line value passes), regardless of the value's safety.
        FieldType::String | FieldType::OwnedLocation => check_opaque_scalar(field, value),
        // A `ref` carries a *shape* over the opaque floor: `<type>:<slug>` with the
        // type matching the field's `to:` (when declared) and a well-formed slug
        // body. Split out of the opaque floor so a malformed migrated edge — a bare
        // slug, a wrong type, the unbracketed comma form — is caught at the write
        // verb, not deferred to a misleading finalize dangle. The real adjudication
        // (target *resolves*) is still the finalize-time edge-index probe.
        FieldType::Ref => check_ref(field, value),
        // A pack-declared type. Its **real** adjudicator is the bound finalize-time
        // probe (built in a later increment); here we run only the type's optional,
        // cheap *write-time shape check* — the `String | Ref` arm above split out so
        // a pack type can add constraints without touching the engine-native arms
        // (`design/document-type-schema.md` → Pack-declared field types: Adjudication
        // splits write-time vs finalize-time).
        FieldType::Pack(pack) => match pack.name.as_str() {
            "code-anchor" => check_code_anchor(field, value),
            // A pack-declared type with no write-time shape check defers entirely to
            // its finalize-time probe; the opaque non-empty/single-line floor still
            // applies so a value can never inject a second field line on splice.
            _ => check_opaque_scalar(field, value),
        },
    }
}

/// The opaque-scalar floor shared by `string` / `ref` (and any pack type lacking a
/// shape check): non-empty and single-line (no control char — newline/tab/… would
/// inject a second field line when spliced onto its `- key: value` line). Deeper
/// adjudication is a `finalize` / pack concern.
fn check_opaque_scalar(field: &SchemaField, value: &str) -> Result<(), String> {
    if value.is_empty() {
        Err(format!("{:?} must not be empty", field.id))
    } else if value.chars().any(|c| c.is_control()) {
        Err(format!(
            "{:?} must not contain control characters",
            field.id
        ))
    } else {
        Ok(())
    }
}

/// The `ref` write-time *shape* check: the opaque floor (non-empty + single-line)
/// then the `<type>:<slug>` form — `<type>` must equal the field's declared `to:`
/// (when `to` is present), and `<slug>` must be a well-formed slug
/// ([`crate::slug::is_slug`]). Validating the slug *body* catches the unbracketed
/// comma form (`adr:a, adr:b` arrives as one scalar whose body `a, adr:b` is not a
/// slug), closing that footgun at write (review S3). A **list-cardinality** ref
/// authors as an inline-flow bracket-list `[e1, e2, …]` — its canonical on-disk
/// form, which the writer splices verbatim and [`crate::field_block`] re-parses to a
/// [`Value::List`]. The production write path passes that scalar as one
/// [`Value::Scalar`] (never a `Value::List`), so this shape check recognizes the
/// bracket wrapper and validates each element through the same `<type>:<slug>` rule;
/// the unbracketed comma form is *not* bracketed and stays one scalar, so the
/// slug-body check still rejects it. The real adjudication — that the target
/// *resolves* against the edge index, and forward cardinality — is the finalize-time
/// probe (`design/auto-migration.md` → Write-time ref-shape check).
fn check_ref(field: &SchemaField, value: &str) -> Result<(), String> {
    // Floor first: non-empty + single-line (control chars rejected).
    check_opaque_scalar(field, value)?;
    // Inline-flow bracket-list `[e1, e2, …]`: validate each element's ref shape. An
    // empty list `[]` carries no edges (clean). The bracket wrapper is what
    // distinguishes the list form from the rejected unbracketed comma form.
    if let Some(inner) = value.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let inner = inner.trim();
        if inner.is_empty() {
            return Ok(());
        }
        for elem in inner.split(',') {
            check_ref_shape(field, elem.trim())?;
        }
        return Ok(());
    }
    check_ref_shape(field, value)
}

/// Whether `field` is a **list-cardinality** ref — a `ref` whose forward `card:` has
/// an unbounded (`*`) upper bound (`0..*`, `1..*`). Such a field carries an ordered
/// list of edges; a single-value `set-field` onto a populated one replaces the whole
/// list rather than accumulating.
fn is_list_ref(field: &SchemaField) -> bool {
    field.ty == FieldType::Ref && field.card.as_deref().is_some_and(|c| c.contains('*'))
}

/// The list-cardinality overwrite guard for a present-field `set-field`. Returns a
/// blocking [`Finding`] when the write would silently drop prior value(s): the field
/// is a list-cardinality ref, its `existing_value` carries ≥1 element, and the
/// `new_value` is a single (non-bracket) element. The bracket-list form `[a, b]` is an
/// explicit whole-list replace and passes (it names the full list — no silent loss).
fn check_list_overwrite(
    field: &SchemaField,
    existing_value: &str,
    new_value: &Value,
    field_key: &str,
) -> Option<Finding> {
    if !is_list_ref(field) {
        return None;
    }
    let new_text = value_text(new_value);
    // An explicit bracket-list replace is allowed (the whole list is named).
    if new_text.trim_start().starts_with('[') {
        return None;
    }
    // Count the existing edges: a bracket-list's comma-separated elements, else (a bare
    // scalar) one. An empty / `[]` value carries nothing to drop.
    let existing = existing_value.trim();
    let count = match existing.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        Some(inner) if inner.trim().is_empty() => 0,
        Some(inner) => inner.split(',').count(),
        None if existing.is_empty() => 0,
        None => 1,
    };
    if count == 0 {
        return None;
    }
    // Suggest the inline-list form combining the existing value(s) with the new one —
    // the actionable fix, the only way to set multiple values in one call.
    let existing_inner = existing
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or(existing)
        .trim();
    let suggested = format!("[{existing_inner}, {}]", new_text.trim());
    Some(Finding::blocking(
        "write.list-overwrite",
        format!(
            "write rejected: field {field_key:?} already has {count} value(s); `set-field` \
             replaces the whole list and would silently drop them. To set multiple values, \
             pass them all in one call: --value {suggested:?}"
        ),
        Location::at(1, 1),
    ))
}

/// The `<type>:<slug>` shape check for a single ref element — `<type>` must equal the
/// field's declared `to:` (when present) and `<slug>` must be a well-formed slug.
fn check_ref_shape(field: &SchemaField, value: &str) -> Result<(), String> {
    let (ty, slug) = value
        .split_once(':')
        .ok_or_else(|| format!("{value:?} is not a ref (expected `<type>:<slug>`)"))?;
    // Type-equality only when `to` is declared; the `<type>:<slug>` shape + slug
    // grammar are always validated (no shipped ref carries `to: None`).
    if let Some(to) = &field.to
        && ty != to.as_str()
    {
        return Err(format!(
            "{value:?} targets type {ty:?} but field {:?} references type {to:?}",
            field.id
        ));
    }
    if !crate::slug::is_slug(slug) {
        return Err(format!(
            "{value:?} is not a ref (expected `<type>:<slug>` with a well-formed slug)"
        ));
    }
    Ok(())
}

/// The `code-anchor` write-time *shape* check: non-empty, single-line, and parses
/// as `path#symbol` — a bare `path` (no `#`) is the file-existence form and also
/// passes. This is the cheap local gate; the real adjudication is the bound
/// `doc-code` probe at finalize (`design/validation.md` → The anchor grammar).
fn check_code_anchor(field: &SchemaField, value: &str) -> Result<(), String> {
    // Floor first: non-empty + single-line (control chars rejected).
    check_opaque_scalar(field, value)?;
    // At most one `#`; if present, both `path` and `symbol` are non-empty.
    match value.split_once('#') {
        None => Ok(()), // bare `path` — file-existence form.
        Some((path, symbol)) => {
            if path.is_empty() || symbol.is_empty() || symbol.contains('#') {
                Err(format!(
                    "{:?} is not a code-anchor (expected `path#symbol` or a bare `path`)",
                    field.id
                ))
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
    // Per-month day ceiling; February allows 29 unconditionally (no leap-year
    // computation in this minimal check — the canonical date comes from the
    // `set: on-create` deriver, this only rejects obvious malformations).
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 29,
        _ => return false,
    };
    (1..=max_day).contains(&day)
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

    // Present field ⇒ surgical splice of the located value span; absent optional field
    // ⇒ generate-or-insert the field line at its schema-ordered position (the absent
    // half of the write path; `parsing.md` → Absent structural homes). The driving
    // case is an optional header ref the fillable form omits (`commit#header/implements`).
    match locate_field_value(source, field_key) {
        Some(target) => {
            // List-cardinality (`0..*`) ref overwrite guard. A single-value `set-field`
            // onto a ref that already carries value(s) would surgically replace the
            // *whole* list — silently dropping the prior entries (the NGT-surfaced
            // last-write-wins footgun). Reject it, naming the field and showing the
            // inline-list form (the only multi-value path). The explicit bracket-list
            // form is allowed: naming the whole list is an intentional replace, not a
            // silent loss — that is the blessed re-point/replace idiom.
            if let Some(finding) =
                check_list_overwrite(field, &source[target.clone()], new_value, field_key)
            {
                return Err(finding);
            }
            let edited = set_field(
                schema,
                source,
                section_id,
                field_key,
                &value_text(new_value),
            )
            .map_err(|e| splice_error_finding(&e))?;
            // (a) + (b): re-parse and only-target-changed. Abort on any anomaly.
            validate_after(schema, source, &edited, target)?;
            Ok(edited)
        }
        None => {
            // The field's home is absent: materialize its line in schema order. The
            // header (front-matter) is the in-scope case; a body field group reuses
            // [`insert_field`]. Generation is not a single-span splice, so the only
            // post-write gate is re-parse (the surgical-span check does not apply).
            let new_field = Field {
                key: field_key.to_string(),
                value: new_value.clone(),
            };
            let is_header = schema
                .sections
                .iter()
                .find(|s| s.id == section_id)
                .is_some_and(|s| s.header);
            let edited = if is_header {
                insert_front_matter_field(schema, source, section_id, &new_field)
            } else {
                insert_field(schema, source, section_id, &new_field)
            }
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
    }
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
            // The validate-after target is the **section body region** — the bytes
            // [`set_slot`] re-renders canonically — not the bare recorded slot span:
            // `set_slot` no longer splices the bare span (that was the leading-slot
            // byte-stability defect, M13 audit HIGH), so the confinement check (b) must
            // bound the same region the canonical re-render touches, or a legitimate
            // blank-line canonicalization would trip `write.target-escape`.
            let target = locate_slot_region(schema, source, section_id).ok_or_else(|| {
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

/// Locate the **body region** of `section_id` in `source` — `[content_start, next-`
/// `##`/EOF)`, the bytes [`set_slot`] re-renders canonically — the validate-after
/// confinement target for a present-section set-slot. Returns `None` when the section
/// is absent or declares no slot (the surgical-splice precondition `set_slot` enforces).
fn locate_slot_region(schema: &Schema, source: &str, section_id: &str) -> Option<Range<usize>> {
    let doc = parse::parse_sections(schema, source).ok()?;
    let section = doc.sections.iter().find(|s| s.id == section_id)?;
    section.slot.as_ref()?;
    let blocks = parse::scan_blocks(source);
    let present = present_body_sections(schema, source);
    section_region(&blocks, source, section_id, &present)
}

/// Render a [`GenerateError`] as the gate's blocking [`Finding`] — the shared
/// engine→CLI mapping the create-gate and the `add-item` CLI verb both route their
/// generation failures through (the expose-vs-replicate pin: one engine-owned
/// finding shape, never re-derived in the CLI).
pub fn generate_error_finding(err: &GenerateError) -> Finding {
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
        GenerateError::UnslugableTitle { title } => (
            "write.unslugable-title",
            format!("write rejected: title {title:?} has no slug-able content for an item id"),
        ),
        // The item-field value-type reject emits finalize's item-field code (the same
        // `field-value-conformant` literal `crate::validate::check_field_value` uses) so
        // the two check times are byte-identical in their code; only the address leaf
        // differs by call site (`design/write-commands.md` → Two check times).
        GenerateError::MalformedValue { why } => (
            "schema-conformance.field-value-conformant",
            format!("write rejected: {why}"),
        ),
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

/// Find the [`SchemaField`] declared for `field_key` in the repeatable block a
/// (possibly nested) item id chain bottoms out in: the section's own repeatable for a
/// top-level chain (`["1-0-0"]`), descending one nested repeatable per nested-section
/// segment for a deeper chain (`["1-0-0", "changes", "added"]`). The chain alternates
/// item-id / nested-section-id segments (the shape [`physical_item_chain`] walks), so the
/// returned field is declared exactly where the addressed item lives. `None` if the
/// section is not repeatable, a nested-section segment names no declared repeatable, or
/// the field is not declared at that level (an unknown item field stays unadjudicated
/// here — the engine does not invent a type to check against).
fn item_field_schema<'a>(
    schema: &'a Schema,
    section_id: &str,
    item_chain: &[&str],
    field_key: &str,
) -> Option<&'a SchemaField> {
    let section = schema.sections.iter().find(|s| s.id == section_id)?;
    let SectionBody::Repeatable { repeatable } = &section.body else {
        return None;
    };
    let mut block = &repeatable.block;
    let mut expect_item = true;
    for segment in item_chain {
        if expect_item {
            expect_item = false;
        } else {
            block = block.iter().find_map(|leaf| match leaf {
                crate::schema::Leaf::Repeatable { id, repeatable } if id == segment => {
                    Some(&repeatable.block)
                }
                _ => None,
            })?;
            expect_item = true;
        }
    }
    block.iter().find_map(|leaf| match leaf {
        crate::schema::Leaf::Field(field) if field.id == field_key => Some(&**field),
        _ => None,
    })
}

/// Render a [`SpliceError`] as the gate's blocking [`Finding`].
pub fn splice_error_finding(err: &SpliceError) -> Finding {
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
    use crate::schema::Schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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

    /// (c) A `set-field` whose new `ref` value is malformed (a bare slug — no
    /// `<type>:` prefix) is rejected with a Blocking [`Finding`] at the write verb,
    /// `write.malformed-value` — not deferred to a misleading finalize dangle.
    #[test]
    fn malformed_ref_value_is_blocking_finding() {
        let finding = set_field_validated(
            &adr_schema(),
            CANONICAL_ADR,
            "status",
            "supersedes",
            &scalar("use-postgres"),
        )
        .expect_err("a bare-slug ref ⇒ abort");
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
            check: None,
            optional: false,
            title_names_symbol: false,
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
        // impossible calendar dates are rejected, not merely out-of-range months/days
        assert!(check_value(&date, &scalar("2026-02-31")).is_err());
        assert!(check_value(&date, &scalar("2026-04-31")).is_err());
        assert!(check_value(&date, &scalar("2026-01-31")).is_ok());
        assert!(check_value(&date, &scalar("2026-02-29")).is_ok());

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

        // string / ref: any non-empty value (byte-identical pre/post the arm split)
        let name = field(FieldType::String, None);
        assert!(check_value(&name, &scalar("anything goes")).is_ok());
        assert!(check_value(&name, &scalar("")).is_err());
        // ref: floor (non-empty + single-line) PLUS the `<type>:<slug>` shape —
        // the slug body must be a well-formed slug; with no `to:` declared the
        // type element is unconstrained. The real adjudication (target resolves)
        // is the finalize-time edge-index probe.
        let r = field(FieldType::Ref, None);
        assert!(check_value(&r, &scalar("adr:single-node-cache")).is_ok());
        // a bare slug (no `<type>:` prefix) is rejected — the migration footgun.
        assert!(check_value(&r, &scalar("use-postgres")).is_err());
        // the unbracketed comma form arrives as one scalar whose slug body
        // (`a, adr:b`) is not a slug — rejected at write (review S3).
        assert!(check_value(&r, &scalar("adr:a, adr:b")).is_err());
        // a `#`-fragment / `/` slug body is not a well-formed slug — rejected.
        // (The `#`-bearing forms are read-path placeholder addresses; they never
        // reach `check_value` on the write path.)
        assert!(check_value(&r, &scalar("adr:auth#criteria/rate-limit")).is_err());
        // scalar values are single-line: a control char (newline/tab) is rejected,
        // so a value can never inject a second field line on splice.
        assert!(check_value(&name, &scalar("line one\nline two")).is_err());
        assert!(check_value(&r, &scalar("adr:a\nadr:b")).is_err());
        assert!(check_value(&r, &scalar("adr:a\tb")).is_err());

        // ref with a declared `to:` — the type element must equal `to`.
        let r_to_adr = SField {
            to: Some("adr".into()),
            ..field(FieldType::Ref, None)
        };
        assert!(check_value(&r_to_adr, &scalar("adr:use-postgres")).is_ok());
        // wrong type for a `to: adr` field is rejected.
        assert!(check_value(&r_to_adr, &scalar("spec:foo")).is_err());
        // a 2-element bracket list authors clean (each element checked
        // element-wise) — the `arch-doc.cites` / multi-supersede shape.
        assert!(
            check_value(
                &r_to_adr,
                &Value::List(vec!["adr:a".into(), "adr:b".into()])
            )
            .is_ok()
        );
        // a list whose element carries the wrong type is rejected element-wise.
        assert!(
            check_value(
                &r_to_adr,
                &Value::List(vec!["adr:a".into(), "spec:b".into()])
            )
            .is_err()
        );
        // The *production* write path passes the inline-flow bracket-list as one
        // `Value::Scalar` (the writer splices it verbatim; the reader re-parses it as
        // a list). So the bracket scalar `[adr:a, adr:b]` — the shipped migration
        // guidance form — must author clean, validated element-wise.
        assert!(check_value(&r_to_adr, &scalar("[adr:a, adr:b]")).is_ok());
        // an empty bracket-list carries no edges — clean.
        assert!(check_value(&r_to_adr, &scalar("[]")).is_ok());
        // a bracket-list whose element is the wrong type is rejected element-wise.
        assert!(check_value(&r_to_adr, &scalar("[adr:a, spec:b]")).is_err());
        // the UNbracketed comma form stays rejected: it is one scalar whose slug
        // body (`a, adr:b`) is not a well-formed slug (review S3 footgun).
        assert!(check_value(&r_to_adr, &scalar("adr:a, adr:b")).is_err());

        // code-anchor (a pack-declared type): non-empty + single-line + parses as
        // `path#symbol` (a bare path is the file-existence form and also passes).
        // The real adjudication is the bound `doc-code` finalize-time probe; this
        // is only the cheap write-time *shape* check.
        let anchor = field(
            FieldType::Pack(crate::schema::PackFieldType {
                name: "code-anchor".into(),
                adjudicator: Some("doc-code".into()),
                check: Some("symbol-exists".into()),
            }),
            None,
        );
        // `path#symbol` with `#` and `/` passes.
        assert!(
            check_value(
                &anchor,
                &scalar("crates/engine/src/validate.rs#validate_task")
            )
            .is_ok()
        );
        // a bare path (no `#`) is the file-existence form — passes.
        assert!(check_value(&anchor, &scalar("crates/engine/src/validate.rs")).is_ok());
        // empty is rejected.
        assert!(check_value(&anchor, &scalar("")).is_err());
        // multi-line is rejected (the newly-added single-line constraint).
        assert!(check_value(&anchor, &scalar("a.rs#one\nb.rs#two")).is_err());
        // an interior control char (tab) is rejected.
        assert!(check_value(&anchor, &scalar("a.rs#sym\tbol")).is_err());
        // a value that does not parse as `path#symbol` is rejected: a trailing `#`
        // with an empty symbol, an empty path before `#`, and more than one `#`.
        assert!(check_value(&anchor, &scalar("crates/engine/src/validate.rs#")).is_err());
        assert!(check_value(&anchor, &scalar("#validate_task")).is_err());
        assert!(check_value(&anchor, &scalar("a.rs#one#two")).is_err());
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

    /// Round-trip: filling a **non-terminal** slot whose body is currently empty (a
    /// freshly-created doc — every slot a zero-width span sitting at the next `##`
    /// heading) with prose that lacks a trailing newline must produce a **reparseable**
    /// buffer. The canonical writer is the parser's inverse: valid slot prose round-trips
    /// regardless of a trailing newline. Reproduces the pre-existing defect where the
    /// prose was glued directly onto the following heading (`Prose.## Decision`), so the
    /// result misaligned on reparse and `set_slot_validated` wrongly rejected it with
    /// `write.non-reparseable`.
    #[test]
    fn fill_empty_nonterminal_slot_without_trailing_newline_reparses() {
        // The shape `doc create adr` emits: a header plus every body slot empty (the
        // blank line after each heading is the whole slot region).
        let empty = "\
---
status: proposed
date: 2026-05-23
---

# Rate-limit at the gateway

## Context

## Decision

## Consequences
";
        // `context` is a non-terminal slot (Decision + Consequences follow it). The
        // prose deliberately lacks a trailing newline.
        let out = set_slot_validated(&adr_schema(), empty, "context", "Per-client limits.")
            .expect("filling an empty non-terminal slot must round-trip, trailing LF or not");
        // The buffer re-parses and the prose landed in `context`, not fused to the next
        // heading.
        let doc = parse::parse_sections(&adr_schema(), &out).expect("result re-parses");
        let ids: Vec<&str> = doc.sections.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, ["status", "context", "decision", "consequences"]);
        assert!(
            out.contains("\n## Decision\n"),
            "the `## Decision` heading must survive as a heading, not be glued to prose:\n{out}"
        );
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
mod set_field_generate {
    //! The gated `set-field` over an **absent optional front-matter ref** — the
    //! generate-or-insert half of the write path lifted to the header section
    //! (`parsing.md` → Absent structural homes generate at their schema-ordered
    //! position; `worked-examples.md` flow 6 → `commit#header/implements`). A `commit`
    //! whose fillable form omits the optional `implements` ref must have the
    //! `implements:` line **materialized** at its schema-ordered position; a commit that
    //! already carries `implements` still routes to the surgical splice.

    use super::*;
    use crate::field_block::Value;
    use crate::schema::Schema;

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    /// A staged `commit` whose fillable form OMITS the optional `implements` ref — only
    /// the required `type` is in the front matter.
    const COMMIT_NO_IMPLEMENTS: &str = "\
---
type: feat
---

# Implement gateway rate limiting

## Summary

Add a per-client rate limit at the gateway.

## Body

Centralize limiting at the gateway.

## Trailers
";

    fn scalar(v: &str) -> Value {
        Value::Scalar(v.to_string())
    }

    /// Absent optional ref: `set-field` on `commit#header/implements` materializes the
    /// `implements:` line at its schema-ordered front-matter position (after the present
    /// `type:`), the buffer re-parses, and the diff is confined to the inserted line.
    #[test]
    fn absent_front_matter_ref_materializes_in_schema_order() {
        let out = set_field_validated(
            &commit_schema(),
            COMMIT_NO_IMPLEMENTS,
            "header",
            "implements",
            &scalar("spec:gateway-rate-limiting"),
        )
        .expect("an absent optional ref materializes its line");

        // The line landed in schema order: after `type:`, still inside the `---` fence.
        assert!(out.contains("type: feat\nimplements: spec:gateway-rate-limiting\n---"));
        // The buffer re-parses against the schema.
        parse::parse_sections(&commit_schema(), &out).expect("the materialized buffer re-parses");
        // The diff is confined to the one inserted line: every other byte is verbatim,
        // so removing exactly the new line restores the original.
        assert_eq!(
            out.replace("implements: spec:gateway-rate-limiting\n", ""),
            COMMIT_NO_IMPLEMENTS,
            "only the implements line was inserted"
        );
    }

    /// The `adr` schema, loaded with the dev-pack field types so `cites-code`'s
    /// `code-anchor` type resolves (the empty-fence absent-insert is the M10 inc-1 T4
    /// path; the `create` flow renders the adr's front matter as an EMPTY `---\n---`
    /// fence pair before any header field is materialized).
    fn adr_schema() -> Schema {
        const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }

    /// A freshly-`create`d `adr`: its header fields are all absent, so the front
    /// matter is an EMPTY `---\n---` fence pair (the create flow does not materialize
    /// defaults). pulldown-cmark emits no metadata block for this, so the absent-insert
    /// must detect the empty fence directly.
    const ADR_EMPTY_FRONT_MATTER: &str = "\
---
---

# Anchor decision

## Context

## Decision

## Consequences
";

    /// Absent optional `code-anchor` into an EMPTY front-matter fence pair: `set-field`
    /// on `adr#status/cites-code` materializes the `cites-code:` line *inside* the
    /// previously-empty fence, the buffer re-parses, and the diff is confined to the
    /// one inserted line (the M10 inc-1 T4 empty-fence absent-insert).
    #[test]
    fn absent_field_materializes_into_empty_front_matter_fence() {
        let out = set_field_validated(
            &adr_schema(),
            ADR_EMPTY_FRONT_MATTER,
            "status",
            "cites-code",
            &scalar("src/engine/write.rs#set_field_validated"),
        )
        .expect("an absent field materializes into an empty front-matter fence");

        // The line landed inside the fence (between the opening and closing `---`).
        assert!(
            out.contains("---\ncites-code: src/engine/write.rs#set_field_validated\n---"),
            "the field line lands inside the previously-empty fence; got:\n{out}"
        );
        // The buffer re-parses against the schema.
        parse::parse_sections(&adr_schema(), &out).expect("the materialized buffer re-parses");
        // The diff is confined to the one inserted line.
        assert_eq!(
            out.replace("cites-code: src/engine/write.rs#set_field_validated\n", ""),
            ADR_EMPTY_FRONT_MATTER,
            "only the cites-code line was inserted"
        );
    }

    /// A `commit` that already carries `implements` routes to the **surgical splice**:
    /// the value bytes change in place, no second `implements:` line is generated.
    #[test]
    fn present_front_matter_ref_routes_to_splice() {
        let with_field = COMMIT_NO_IMPLEMENTS
            .replace("type: feat\n", "type: feat\nimplements: spec:old-target\n");
        let out = set_field_validated(
            &commit_schema(),
            &with_field,
            "header",
            "implements",
            &scalar("spec:new-target"),
        )
        .expect("a present ref edits surgically");
        assert!(out.contains("implements: spec:new-target"));
        assert!(!out.contains("spec:old-target"));
        // Exactly one `implements:` line — the splice did not generate a second.
        assert_eq!(out.matches("implements:").count(), 1);
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
    use crate::schema::Schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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
    use crate::schema::Schema;
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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
    use crate::schema::Schema;
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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
    use crate::schema::Schema;
    use proptest::prelude::*;

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");
    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const PRD_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/prd.yaml");
    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");
    const ARCH_DOC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/arch-doc.yaml");

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }
    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }
    /// The shipped `prd` schema. `prd` carries NO pack `code-anchor` type (its
    /// `requirements` block is just a `title` field + a `statement` slot, dropping the
    /// `maps-to-test` anchor `spec.criteria` carries) and no front-matter, so it loads bare.
    fn prd_schema() -> Schema {
        crate::schema::load_schema(PRD_YAML).expect("prd.yaml loads")
    }
    /// The shipped `spec` schema. Unlike `prd`, its `criteria` block carries a
    /// per-item `maps-to-test` **`code-anchor` field** (alongside the `title` field +
    /// `statement` slot), so it loads with the dev-pack field types registered (the
    /// `code-anchor` type — exactly as `adr` does for its `cites-code`). `spec` has no
    /// front-matter (`id-from: title`, the title is the H1), so it carries no settable
    /// field either — the surgical-edit clause self-skips.
    fn spec_schema() -> Schema {
        crate::schema::load_schema_with_types(SPEC_YAML, &crate::schema::dev_pack_field_types())
            .expect("spec.yaml loads")
    }
    /// The shipped `arch-doc` schema. It is the first fuzzed doctype with a
    /// **`header: true` section carrying a list-shaped front-matter field**: `meta`'s
    /// `cites` is a `ref → adr` with `card: 0..*`, rendered as an inline-flow list in
    /// the `---`-fenced header (`cites: [adr:a, adr:b]`). Its `components` repeatable
    /// mirrors `spec.criteria` (a `title` `id-from` heading + a `description` slot + a
    /// per-item `implemented-by` **`code-anchor` field**), so it loads with the dev-pack
    /// field types registered (both `ref` and `code-anchor`, exactly as `adr` does).
    /// `id-from: title` (the H1) → no settable header *scalar*, and `cites` is a ref-list
    /// (not a `set_field` target), so the surgical-edit clause self-skips on `edit: None`.
    fn arch_doc_schema() -> Schema {
        crate::schema::load_schema_with_types(ARCH_DOC_YAML, &crate::schema::dev_pack_field_types())
            .expect("arch-doc.yaml loads")
    }

    /// One generated arbitrary conformant document plus its known-canonical LF form
    /// and the metadata the surgical-edit clause needs.
    #[derive(Clone, Debug)]
    struct GenDoc {
        /// `"commit"`, `"adr"`, `"prd"`, `"spec"`, or `"arch-doc"` — selects the schema.
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

    /// Build a canonical-LF `prd` document from generated parts by constructing an
    /// [`Instance`] and **rendering it** — the first fuzz of the populated
    /// repeatable-item render path (`### <title>  {#id}` heading + per-item `statement`
    /// slot). `prd` carries the `vision`/`context` fixed prose slots and a POPULATED
    /// `requirements` section (one item per `(title, statement)`, the `title` consumed
    /// as the `id-from` heading — never a field bullet, like `spec.criteria`). Building
    /// through `render` guarantees the bytes match the writer's canonical form exactly.
    fn build_prd(
        title: &str,
        vision: &str,
        requirements: &[(String, String)],
        context: &str,
    ) -> String {
        let items: Vec<ItemContent> = requirements
            .iter()
            .map(|(req_title, statement)| ItemContent {
                id: crate::slug::slugify(req_title),
                title: req_title.clone(),
                slot: Some(statement.clone()),
                slots: Vec::new(),
                fields: Vec::new(),
                items: Vec::new(),
            })
            .collect();
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "vision".to_string(),
                    slot: Some(vision.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "requirements".to_string(),
                    items,
                    ..Default::default()
                },
                SectionContent {
                    id: "context".to_string(),
                    slot: Some(context.to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&prd_schema(), &instance)
    }

    /// A canonical `code-anchor` field value: a backtick-wrapped `path#symbol` (the
    /// shipped form `spec.criteria` writes — `parse.rs` golden + `flow13`). The reader
    /// is opaque, so any single-line non-list-shaped scalar round-trips; this models
    /// the real anchor (path with a `#symbol` fragment) so the fuzz drives a realistic
    /// per-item field value, not a degenerate one.
    fn code_anchor_value() -> impl Strategy<Value = String> {
        (
            "[a-z][a-z0-9_/-]{0,18}\\.(rs|rb|py)",
            "[a-z][a-z0-9_]{0,15}",
        )
            .prop_map(|(path, sym): (String, String)| format!("`{path}#{sym}`"))
    }

    /// Build a canonical-LF `spec` document from generated parts by constructing an
    /// [`Instance`] and **rendering it**. `spec` carries the `goal`/`context` fixed
    /// prose slots and a POPULATED `criteria` repeatable whose item block is a `title`
    /// field (consumed as the `id-from` heading, never a field bullet — like
    /// `prd.requirements`), a `statement` slot, **and** a per-item `maps-to-test`
    /// **`code-anchor` field**. The code-anchor field is the new shape this arm fuzzes:
    /// a sentinelled `<!-- fields -->` group *inside* a repeatable item must
    /// render/round-trip byte-stable. Building through `render` guarantees the bytes
    /// match the writer's canonical form exactly.
    fn build_spec(
        title: &str,
        goal: &str,
        criteria: &[(String, String, String)],
        context: &str,
    ) -> String {
        let items: Vec<ItemContent> = criteria
            .iter()
            .map(|(crit_title, statement, anchor)| ItemContent {
                id: crate::slug::slugify(crit_title),
                title: crit_title.clone(),
                slot: Some(statement.clone()),
                slots: Vec::new(),
                fields: vec![Field {
                    key: "maps-to-test".to_string(),
                    value: Value::Scalar(anchor.clone()),
                }],
                items: Vec::new(),
            })
            .collect();
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "goal".to_string(),
                    slot: Some(goal.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "criteria".to_string(),
                    items,
                    ..Default::default()
                },
                SectionContent {
                    id: "context".to_string(),
                    slot: Some(context.to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&spec_schema(), &instance)
    }

    /// A canonical `arch-doc` `implemented-by` value: a **bare** `path#symbol` (no
    /// backticks — the shipped form `arch-doc` writes, per `migrate_arch_doc.rs` /
    /// `arch_doc_acceptance.rs`, distinct from `spec.criteria`'s backtick-wrapped
    /// anchor). Opaque to the reader, so it round-trips verbatim; modelling the real
    /// `path#symbol` shape drives a realistic per-item field value.
    fn arch_anchor_value() -> impl Strategy<Value = String> {
        (
            "[a-z][a-z0-9_/-]{0,18}\\.(rs|rb|py)",
            "[a-z][a-z0-9_]{0,15}",
        )
            .prop_map(|(path, sym): (String, String)| format!("{path}#{sym}"))
    }

    /// Build a canonical-LF `arch-doc` document from generated parts by constructing an
    /// [`Instance`] and **rendering it**. `arch-doc` is the first fuzzed doctype with a
    /// **`header: true` section carrying a list-shaped front-matter field**: the `meta`
    /// header's `cites` (a `ref → adr`, `card: 0..*`). When `cites` is empty the field is
    /// **omitted** (the production form a 0-ref doc renders — an empty `---\n---` header,
    /// exactly what the `migrate-arch-doc` smoke authors); when populated it renders as an
    /// inline-flow list (`cites: [adr:a, adr:b]`) — the new front-matter-list shape this
    /// arm fuzzes byte-stable. The body is the `overview` slot + a POPULATED `components`
    /// repeatable whose item block mirrors `spec.criteria`: a `title` `id-from` heading, a
    /// `description` slot, and a per-item `implemented-by` **`code-anchor` field**.
    /// Building through `render` guarantees the bytes match the writer's canonical form.
    fn build_arch_doc(
        title: &str,
        cites: &[String],
        overview: &str,
        components: &[(String, String, String)],
    ) -> String {
        let items: Vec<ItemContent> = components
            .iter()
            .map(|(comp_title, description, anchor)| ItemContent {
                id: crate::slug::slugify(comp_title),
                title: comp_title.clone(),
                slot: Some(description.clone()),
                slots: Vec::new(),
                fields: vec![Field {
                    key: "implemented-by".to_string(),
                    value: Value::Scalar(anchor.clone()),
                }],
                items: Vec::new(),
            })
            .collect();
        // 0 cites → omit the field (empty header, the production form); ≥1 → a
        // list-shaped front-matter field (`cites: [adr:…, …]`).
        let meta_fields = if cites.is_empty() {
            Vec::new()
        } else {
            vec![Field {
                key: "cites".to_string(),
                value: Value::List(cites.to_vec()),
            }]
        };
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: meta_fields,
                    ..Default::default()
                },
                SectionContent {
                    id: "overview".to_string(),
                    slot: Some(overview.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "components".to_string(),
                    items,
                    ..Default::default()
                },
            ],
        };
        render(&arch_doc_schema(), &instance)
    }

    /// The generator: an arbitrary conformant `commit`, `adr`, `prd`, or `spec` doc,
    /// with a chosen EOL, returning the canonical-LF text + the EOL + a settable
    /// front-matter field (`None` for `prd`/`spec`, which carry no front-matter — the
    /// surgical clause self-skips).
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

        let prd = (
            scalar_value(),
            edgy_prose(),
            prop::collection::vec((scalar_value(), edgy_prose()), 1..4),
            edgy_prose(),
        )
            .prop_map(|(title, vision, reqs, context)| {
                // Index-suffix each requirement title so the slugified `{#id}` anchors
                // are distinct — a repeatable section rejects duplicate anchors, and
                // two arbitrary titles can collide. The suffix also exercises the
                // multi-word-heading parse on every item.
                let requirements: Vec<(String, String)> = reqs
                    .into_iter()
                    .enumerate()
                    .map(|(i, (req_title, statement))| {
                        (format!("{} {i}", req_title.trim()), statement)
                    })
                    .collect();
                (
                    "prd".to_string(),
                    build_prd(&title, &vision, &requirements, &context),
                    // `prd` has no front-matter, so there is no settable field; the
                    // surgical-edit clause self-skips on `edit: None`.
                    None,
                )
            });

        let spec = (
            scalar_value(),
            edgy_prose(),
            prop::collection::vec((scalar_value(), edgy_prose(), code_anchor_value()), 1..4),
            edgy_prose(),
        )
            .prop_map(|(title, goal, crits, context)| {
                // Index-suffix each criterion title so the slugified `{#id}` anchors are
                // distinct (a repeatable section rejects duplicate anchors, and two
                // arbitrary titles can collide). The suffix also exercises the
                // multi-word-heading parse on every item.
                let criteria: Vec<(String, String, String)> = crits
                    .into_iter()
                    .enumerate()
                    .map(|(i, (crit_title, statement, anchor))| {
                        (format!("{} {i}", crit_title.trim()), statement, anchor)
                    })
                    .collect();
                (
                    "spec".to_string(),
                    build_spec(&title, &goal, &criteria, &context),
                    // `spec` has no front-matter, so there is no settable field; the
                    // surgical-edit clause self-skips on `edit: None`.
                    None,
                )
            });

        let arch_doc = (
            scalar_value(),
            // `cites`: 0..3 adr refs. The `0..` lower bound makes proptest generate
            // BOTH the empty-cites case (an omitted field → empty `---\n---` header) and
            // the populated case (an inline-flow `cites: [adr:…, …]` list), so a single
            // run covers both front-matter shapes (the done-criterion).
            prop::collection::vec("[a-z][a-z0-9]{0,12}", 0..3),
            edgy_prose(),
            prop::collection::vec((scalar_value(), edgy_prose(), arch_anchor_value()), 1..4),
        )
            .prop_map(|(title, cite_slugs, overview, comps)| {
                // Index-suffix each cite slug so the `adr:` refs are distinct, and each
                // component title so the slugified `{#id}` item anchors are distinct (a
                // repeatable section rejects duplicate anchors, and two arbitrary titles
                // can collide). The suffix also exercises the multi-word-heading parse on
                // every component.
                let cites: Vec<String> = cite_slugs
                    .into_iter()
                    .enumerate()
                    .map(|(i, slug)| format!("adr:{slug}-{i}"))
                    .collect();
                let components: Vec<(String, String, String)> = comps
                    .into_iter()
                    .enumerate()
                    .map(|(i, (comp_title, description, anchor))| {
                        (format!("{} {i}", comp_title.trim()), description, anchor)
                    })
                    .collect();
                (
                    "arch-doc".to_string(),
                    build_arch_doc(&title, &cites, &overview, &components),
                    // `cites` is a ref-list (not a scalar `set_field` target) and the H1
                    // is the id source, so there is no settable header scalar; the
                    // surgical-edit clause self-skips on `edit: None`.
                    None,
                )
            });

        (prop_oneof![commit, adr, prd, spec, arch_doc], eol).prop_map(
            |((ty, canonical_lf, edit), eol)| GenDoc {
                ty,
                canonical_lf,
                eol,
                edit,
            },
        )
    }

    fn schema_for(ty: &str) -> Schema {
        match ty {
            "commit" => commit_schema(),
            "prd" => prd_schema(),
            "spec" => spec_schema(),
            "arch-doc" => arch_doc_schema(),
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
mod spec_roundtrip {
    //! The shipped `spec` doctype round-trips byte-stably (M3 Increment 1, T2).
    //!
    //! Unlike the in-test `spec`-shaped fixtures elsewhere in this crate, this suite
    //! loads the **shipped** `crates/cli/pack/schemas/spec.yaml` bytes and proves the
    //! #1-risk round-trip over the new doctype's highest-risk shape — the repeatable
    //! `criteria` items, each carrying a frozen `{#id}` anchor and a `statement`
    //! slot. Both round-trip clauses (`parsing.md` → Round-trip guarantees):
    //! (a) `render(parse(src)) == src` golden bytes (idempotent on canonical
    //! content), and (b) `write → parse → write` byte-identical.

    use super::*;
    use crate::schema::Schema;

    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");
    const ARCH_DOC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/arch-doc.yaml");

    fn spec_schema() -> Schema {
        crate::schema::load_schema_with_types(SPEC_YAML, &crate::schema::dev_pack_field_types())
            .expect("spec.yaml loads")
    }

    fn arch_doc_schema() -> Schema {
        crate::schema::load_schema_with_types(ARCH_DOC_YAML, &crate::schema::dev_pack_field_types())
            .expect("arch-doc.yaml loads")
    }

    /// A canonical `spec` instance over the shipped schema: the `# H1` title (the
    /// id-source — `spec` carries NO `title` field and NO header section, like
    /// `adr`, so it renders no front-matter block), `goal` + `context` prose slots,
    /// and two `criteria` items each with a frozen `{#id}` anchor and a `statement`
    /// slot. Authored in the exact frozen byte form the canonical writer emits
    /// (`# H1`, `## …` slots, `### …  {#id}` items with the two-space anchor gap).
    fn canonical_spec_source() -> &'static str {
        "\
# Gateway rate limiting

## Goal

Bound per-client request volume at the gateway.

## Context

Downstream services were each enforcing limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Recovers after the window  {#recovers}

The next window admits requests again.
"
    }

    /// Re-derive an [`Instance`] from a parsed [`crate::parse::Document`] over
    /// `source`, owning the slot prose (re-slicing the spans). Mirrors the canonical
    /// module's bridge so this suite asserts `write → parse → write`.
    fn reparse_to_instance(schema: &Schema, source: &str) -> Instance {
        instance_from_source(schema, source).expect("rendered spec parses")
    }

    /// Clause (a): `render(parse(src)) == src` golden bytes — the shipped spec schema
    /// parses the canonical fixture (`# H1` title, `goal`/`context` slots, ≥2
    /// `criteria` items each with a frozen `{#id}` + `statement` slot) and the writer
    /// reproduces the source byte-for-byte. The golden pins the canonical bytes.
    #[test]
    fn spec_render_parse_render_equals_source() {
        let schema = spec_schema();
        let src = canonical_spec_source();

        let instance = reparse_to_instance(&schema, src);
        // The two frozen criteria anchors survive the parse, in order.
        let criteria = instance
            .sections
            .iter()
            .find(|s| s.id == "criteria")
            .expect("criteria section present");
        let ids: Vec<&str> = criteria.items.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(
            ids,
            ["rejects-burst", "recovers"],
            "frozen anchors preserved"
        );

        let rendered = render(&schema, &instance);
        assert_eq!(
            rendered, src,
            "render(parse(src)) must equal the source bytes"
        );
        insta::assert_snapshot!("spec_instance", rendered);
    }

    /// A canonical **empty** `spec` instance: the `# H1` title and the three section
    /// homes (`## Goal`, `## Context`, `## Criteria`) all present but **unfilled** —
    /// exactly what the writer emits before any slot is authored. `goal` is a *leading*
    /// Simple slot whose section is followed by `## Context`, so its empty-slot canonical
    /// form carries the surrounding blank lines `set_slot` must preserve.
    fn empty_spec_source() -> String {
        let schema = spec_schema();
        let instance = instance_from_source(
            &schema,
            "\
# Gateway rate limiting

## Goal

## Context

## Criteria
",
        )
        .expect("empty spec parses");
        render(&schema, &instance)
    }

    /// Regression (M13 audit HIGH): `set_slot` into the shipped `spec`'s **leading**
    /// `goal` slot (the section is followed by `## Context`) must produce **canonical**
    /// bytes, so `render(parse(out)) == out`. The parser records a leading empty slot's
    /// span *bare* (no surrounding blank lines); a bare-span splice yielded
    /// `## Goal\nLimit…\n\n\n## Context` — no blank after the heading, a double blank
    /// before the next section — while [`render_section`] emits one blank on each side.
    /// `set_slot` now re-renders the section canonically (mirroring 3afc98a's item-slot
    /// fix), so the present-splice path is byte-stable.
    #[test]
    fn set_slot_into_leading_goal_slot_is_byte_stable() {
        let schema = spec_schema();
        let src = empty_spec_source();
        let out = set_slot(
            &schema,
            &src,
            "goal",
            "Bound per-client request volume at the gateway.",
        )
        .expect("goal slot present");
        assert!(out.contains("Bound per-client request volume at the gateway."));
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(
            render(&schema, &reparsed),
            out,
            "set_slot into the leading `goal` slot is byte-stable",
        );
    }

    /// Regression (M13 audit HIGH): `set_slot` into the shipped `arch-doc`'s **leading**
    /// `overview` slot (the section is followed by `## Components`) must produce
    /// **canonical** bytes, so `render(parse(out)) == out`. This is M13's production
    /// authoring path — `author-arch-doc.yaml` drives `set-slot arch-doc:<slug>#overview`
    /// — and the same leading-Simple-slot defect the `spec.goal` case hits.
    #[test]
    fn set_slot_into_leading_overview_slot_is_byte_stable() {
        let schema = arch_doc_schema();
        let src = render(
            &schema,
            &instance_from_source(
                &schema,
                "\
---
cites:
---

# Storage layer

## Overview

## Components
",
            )
            .expect("empty arch-doc parses"),
        );
        let out = set_slot(
            &schema,
            &src,
            "overview",
            "The storage layer owns the on-disk task working areas.",
        )
        .expect("overview slot present");
        assert!(out.contains("The storage layer owns the on-disk task working areas."));
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(
            render(&schema, &reparsed),
            out,
            "set_slot into the leading `overview` slot is byte-stable",
        );
    }

    /// Clause (b): `write → parse → write` is byte-identical (idempotent on canonical
    /// content) over the shipped spec schema — the repeatable-item path included.
    #[test]
    fn spec_write_parse_write_is_byte_identical() {
        let schema = spec_schema();
        let src = canonical_spec_source();

        let first = render(&schema, &reparse_to_instance(&schema, src));
        let second = render(&schema, &reparse_to_instance(&schema, &first));
        assert_eq!(
            first, second,
            "write → parse → write must be byte-identical"
        );
    }

    /// M26 fork C4: the empty-slot byte-stability fix. An arch-doc **component** carries
    /// both a `description` slot and an `implemented-by` field — the slot-plus-field-group
    /// shape. A minted-but-unfilled component (empty `description`, filled `implemented-by`)
    /// is the canonical **one-blank** form below; re-parsing it yields `description:
    /// Some("")`, which the writer must canonicalize back to the same one-blank form, or
    /// `render(parse(x)) != x` (the M22→M26-keyed #1-risk violation — three blank lines on
    /// reparse before the fix). The assertion is `render(parse(x)) == x` over the WHOLE
    /// document.
    #[test]
    fn arch_doc_empty_description_component_round_trips_byte_stable() {
        let schema = arch_doc_schema();
        // Build the minted-but-unfilled component: parse a seed doc, then drop the
        // `description` prose to `None` — the state a freshly-minted slot+field-group item
        // holds. The writer renders it in the canonical **one-blank** form; `x` is those
        // bytes (so `x` is pinned to one-blank regardless of the empty-slot defect).
        let seed = "\
---
cites:
---

# Storage layer

## Overview

The storage layer owns the on-disk task working areas.

## Components

### Working area  {#working-area}

seed prose

<!-- fields -->
- implemented-by: src/storage.rs#WorkingArea
";
        let mut instance = instance_from_source(&schema, seed).expect("seed arch-doc parses");
        let component = instance
            .sections
            .iter_mut()
            .find(|s| s.id == "components")
            .and_then(|s| s.items.iter_mut().find(|i| i.id == "working-area"))
            .expect("component present");
        component.slot = None; // the minted-but-unfilled description.
        let x = render(&schema, &instance);

        // Re-parsing those bytes yields `description: Some("")` (a present-but-empty slot),
        // the exact intermediate the fix canonicalizes back to the one-blank form.
        let reparsed = instance_from_source(&schema, &x).expect("minted arch-doc parses");
        let component = reparsed
            .sections
            .iter()
            .find(|s| s.id == "components")
            .and_then(|s| s.items.iter().find(|i| i.id == "working-area"))
            .expect("component present");
        assert_eq!(
            component.slot.as_deref(),
            Some(""),
            "the unfilled description slot reparses as present-but-empty",
        );
        assert_eq!(
            render(&schema, &reparsed),
            x,
            "render(parse(x)) must equal the minted one-blank bytes (M26 C4)",
        );
    }

    /// Masking guard for the M26 fix: a **genuinely slotless** item (`slot: None`, no slot
    /// leaf in its block — a changelog-shaped release carrying only a `date` field) must
    /// stay byte-identical. The fix moves only items that reparse to `Some("")`; an item
    /// that was never slot-bearing is untouched.
    #[test]
    fn genuinely_slotless_item_stays_byte_identical() {
        let schema = crate::schema::load_schema(
            b"\
type: log
id-from: title
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: date, type: string }
",
        )
        .expect("slotless schema loads");
        let x = "\
# Project

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-18
";
        let instance = instance_from_source(&schema, x).expect("slotless doc parses");
        let release = instance
            .sections
            .iter()
            .find(|s| s.id == "releases")
            .and_then(|s| s.items.iter().find(|i| i.id == "1-0-0"))
            .expect("release present");
        assert_eq!(release.slot, None, "the release is genuinely slotless");
        assert_eq!(
            render(&schema, &instance),
            x,
            "a genuinely-slotless item stays byte-identical",
        );
    }

    /// Boundary guard for the M26 fix: an item whose single slot is **present and filled
    /// with real prose** must NOT be collapsed to the slotless one-blank form — its prose
    /// keeps the `### …  {#id}\n\n<prose>\n\n<!-- fields -->` shape. This is the assertion
    /// the `genuinely_slotless` baseline cannot make (that item was slotless before AND
    /// after the predicate change, so it passes either way). If the canonicalization
    /// predicate were ever broadened to `let slotless = item.slots.is_empty();` — dropping
    /// the `slot_empty` guard so a FILLED slot collapses too — the post-heading blank line
    /// would vanish and the substring assertion below goes red.
    #[test]
    fn filled_slot_item_is_not_collapsed_by_the_empty_slot_fix() {
        let schema = arch_doc_schema();
        // An arch-doc component carries a `description` slot plus an `implemented-by`
        // field — the same slot-plus-field-group shape the empty-slot fix canonicalizes,
        // here with the slot actually FILLED.
        let seed = "\
---
cites:
---

# Storage layer

## Overview

The storage layer owns the on-disk task working areas.

## Components

### Working area  {#working-area}

Some real responsibility.

<!-- fields -->
- implemented-by: src/storage.rs#WorkingArea
";
        let instance = instance_from_source(&schema, seed).expect("seed arch-doc parses");
        let component = instance
            .sections
            .iter()
            .find(|s| s.id == "components")
            .and_then(|s| s.items.iter().find(|i| i.id == "working-area"))
            .expect("component present");
        assert_eq!(
            component.slot.as_deref(),
            Some("Some real responsibility."),
            "the description slot is present and filled with real prose",
        );
        let x = render(&schema, &instance);
        // The prose survives between the heading and the field group, framed by the
        // canonical post-heading and pre-field blank lines (NOT collapsed flush against
        // the heading). This substring goes red if a broadened predicate collapses the
        // filled slot.
        assert!(
            x.contains(
                "### Working area  {#working-area}\n\nSome real responsibility.\n\n<!-- fields -->"
            ),
            "the filled slot keeps its blank-line-framed prose, not the slotless form:\n{x}",
        );
        let reparsed = instance_from_source(&schema, &x).expect("filled arch-doc parses");
        assert_eq!(
            render(&schema, &reparsed),
            x,
            "render(parse(x)) must equal the filled bytes — a filled slot is not collapsed",
        );
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
    use crate::schema::Schema;

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
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
                slots: Vec::new(),
                fields: vec![scalar("key", k), scalar("value", v)],
                items: Vec::new(),
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

#[cfg(test)]
mod multi_slot {
    //! M16 Increment 1b — multi-slot-per-repeatable-item (the build-halt resolution).
    //!
    //! A repeatable item template with **>1 slot leaf** renders each slot under a
    //! `#### <Leaf-Title>` sub-heading (leaf id title-cased, schema block order); an
    //! item with **exactly one** slot keeps the bare-prose form (its backward-compat
    //! is asserted by every other suite staying green). These tests drive the
    //! **emitted bytes** end-to-end — `add_item` mints the skeleton, `set_item_slot`
    //! fills each leaf — and assert `render(parse(x)) == x` at every state.

    use super::*;
    use crate::parse::parse_sections;
    use crate::schema::Schema;

    /// A `roadmap`-shaped schema: one header-less `# H1` title (id-from), one
    /// repeatable `milestones` section whose item block carries TWO prose slots
    /// (`proves`, `decomposition`) in that order — the exact shape the M16 roadmap
    /// doctype ships.
    fn two_slot_schema() -> Schema {
        let yaml = b"\
type: roadmap
id-from: title
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: proves, slot: { hint: \"what it proves\" } }
        - { id: decomposition, slot: { hint: \"the increments\" } }
";
        crate::schema::load_schema(yaml).expect("two-slot roadmap schema loads")
    }

    /// Round-trip helper: parse `src`, re-derive its instance, re-render — assert
    /// byte-identical (`render(parse(x)) == x`).
    fn assert_byte_stable(schema: &Schema, src: &str) {
        let instance = instance_from_source(schema, src)
            .unwrap_or_else(|f| panic!("source parses: {f:?}\n--- src ---\n{src}"));
        let rendered = render(schema, &instance);
        assert_eq!(rendered, src, "render(parse(x)) must equal x");
    }

    /// Done-criterion 1 — the failing case made to pass: a two-slot item, both
    /// slots authored through the real write path (`add_item` then `set_item_slot`
    /// per leaf), and BOTH slots' prose are preserved (today the first is dropped).
    /// The assertion runs over the **emitted bytes**, not a hand-built equivalent.
    #[test]
    fn two_slot_item_preserves_both_slots() {
        let schema = two_slot_schema();
        // An empty roadmap with the section home present.
        let empty = render(
            &schema,
            &instance_from_source(&schema, "# Roadmap\n\n## Milestones\n")
                .expect("empty roadmap parses"),
        );

        // Mint the milestone item (mint-empty skeleton).
        let minted = add_item(&schema, &empty, "milestones", "M16 self-hosting", None, &[])
            .expect("add_item mints a two-slot item");
        assert_byte_stable(&schema, &minted);

        // Fill the first slot leaf (`proves`).
        let filled_a = set_item_slot(
            &schema,
            &minted,
            "milestones",
            "m16-self-hosting",
            "proves",
            "Closes the self-hosting loop — the pack composes with dev.",
        )
        .expect("set proves slot");
        assert_byte_stable(&schema, &filled_a);

        // Fill the second slot leaf (`decomposition`).
        let filled_b = set_item_slot(
            &schema,
            &filled_a,
            "milestones",
            "m16-self-hosting",
            "decomposition",
            "Inc 1: conformance. Inc 2: singletons. Inc 3: owner gate.",
        )
        .expect("set decomposition slot");
        assert_byte_stable(&schema, &filled_b);

        // BOTH slots' prose survive (the bug dropped `proves`).
        assert!(
            filled_b.contains("Closes the self-hosting loop"),
            "the first slot's prose must survive: {filled_b:?}",
        );
        assert!(
            filled_b.contains("Inc 1: conformance"),
            "the second slot's prose must survive: {filled_b:?}",
        );
        // Each slot renders under its `#### <Leaf-Title>` sub-heading, schema order.
        let proves_at = filled_b.find("#### Proves").expect("Proves sub-heading");
        let decomp_at = filled_b
            .find("#### Decomposition")
            .expect("Decomposition sub-heading");
        assert!(
            proves_at < decomp_at,
            "slots render in schema block order (proves before decomposition)",
        );
    }

    /// Done-criterion 2 — the M13 cold-start seam: `add_item` mints the FULL
    /// skeleton (all `#### <Label>` sub-headings, empty bodies, schema order), which
    /// round-trips byte-stable; then each leaf is filled, each intermediate state
    /// byte-stable. set_item_slot splices into the named, pre-minted sub-heading.
    #[test]
    fn cold_start_mints_full_skeleton_then_fills_each_leaf() {
        let schema = two_slot_schema();
        let empty = render(
            &schema,
            &instance_from_source(&schema, "# Roadmap\n\n## Milestones\n")
                .expect("empty roadmap parses"),
        );
        let minted = add_item(&schema, &empty, "milestones", "Alpha", None, &[])
            .expect("add_item mints the skeleton");

        // The minted skeleton carries BOTH empty sub-headings in schema order.
        assert!(
            minted.contains("#### Proves"),
            "skeleton has Proves: {minted:?}"
        );
        assert!(
            minted.contains("#### Decomposition"),
            "skeleton has Decomposition: {minted:?}",
        );
        assert_byte_stable(&schema, &minted);

        // Fill decomposition FIRST (out of block order) — set_item_slot must target
        // the named sub-heading, not insert in order.
        let f1 = set_item_slot(
            &schema,
            &minted,
            "milestones",
            "alpha",
            "decomposition",
            "D-prose",
        )
        .expect("fill decomposition");
        assert_byte_stable(&schema, &f1);
        let f2 = set_item_slot(&schema, &f1, "milestones", "alpha", "proves", "P-prose")
            .expect("fill proves");
        assert_byte_stable(&schema, &f2);

        // Both filled; still in schema order regardless of fill order.
        let proves_at = f2.find("#### Proves").unwrap();
        let decomp_at = f2.find("#### Decomposition").unwrap();
        assert!(
            proves_at < decomp_at,
            "schema order preserved across fill order"
        );
        let doc = parse_sections(&schema, &f2).expect("filled doc parses");
        let item = &doc
            .sections
            .iter()
            .find(|s| s.id == "milestones")
            .unwrap()
            .items[0];
        let proves = item.slot_span("proves").expect("proves span").slice(&f2);
        let decomp = item
            .slot_span("decomposition")
            .expect("decomposition span")
            .slice(&f2);
        assert_eq!(proves.trim(), "P-prose");
        assert_eq!(decomp.trim(), "D-prose");
    }

    /// Done-criterion 4 — conformance per-leaf: a multi-slot item with one empty
    /// required slot leaf produces exactly one blocking
    /// `schema-conformance.required-slot-present` addressed at `#section/item/leaf`;
    /// a fully-authored item yields zero findings (the `#### Proves` id-from heading
    /// not spuriously flagged).
    #[test]
    fn per_leaf_conformance_blocks_empty_required_slot() {
        let schema = two_slot_schema();
        // Author only `proves`; leave `decomposition` empty.
        let src = "\
# Roadmap

## Milestones

### Alpha  {#alpha}

#### Proves

Proven.

#### Decomposition
";
        let doc = parse_sections(&schema, src).expect("half-authored item still parses");
        let findings = crate::validate::schema_conformance(&schema, src, &doc);
        let slot_findings: Vec<_> = findings
            .iter()
            .filter(|f| f.code == "schema-conformance.required-slot-present")
            .collect();
        assert_eq!(
            slot_findings.len(),
            1,
            "exactly one empty-slot finding (decomposition); got: {findings:?}",
        );
        let f = slot_findings[0];
        assert_eq!(
            f.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("milestones/alpha/decomposition"),
            "addressed at the empty leaf",
        );

        // Fully authored: zero slot findings.
        let full = "\
# Roadmap

## Milestones

### Alpha  {#alpha}

#### Proves

Proven.

#### Decomposition

Decomposed.
";
        let full_doc = parse_sections(&schema, full).expect("full item parses");
        let none = crate::validate::schema_conformance(&schema, full, &full_doc);
        assert!(
            none.iter()
                .all(|f| f.code != "schema-conformance.required-slot-present"),
            "a fully-authored multi-slot item yields no empty-slot findings: {none:?}",
        );
    }
}

#[cfg(test)]
mod nested_roundtrip {
    //! M22 Increment 1, T3 — the recursive-render byte-stable round-trip, cold + warm.
    //!
    //! The render half closes the parse→render inverse for a **two-level repeatable**
    //! (a `changelog` release whose nested `changes` change-groups render one heading
    //! level deeper — `### release` / `#### change-group`). The oracle is the same the
    //! whole crate rests on: `render(instance_from_source(src)) == src` over the WHOLE
    //! document (never a filled subtree — the M13 masking-test ban).

    use super::*;
    use crate::field_block::Value;
    use crate::schema::Schema;

    /// A `changelog`-shaped schema: one `# H1` title (id-from), one repeatable
    /// `releases` section whose item block carries a scalar `date` field then a
    /// **nested** `changes` repeatable (id-from `category`, a bare-prose `notes`
    /// slot) — the `release → change-group` two-level shape (`design/changelog.md`).
    fn changelog_schema() -> Schema {
        let yaml = b"\
type: changelog
id-from: title
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: date, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";
        crate::schema::load_schema(yaml).expect("changelog schema loads")
    }

    /// Round-trip helper: parse `src`, re-derive its instance, re-render — assert
    /// byte-identical over the WHOLE document (`render(parse(x)) == x`).
    fn assert_byte_stable(schema: &Schema, src: &str) {
        let instance = instance_from_source(schema, src)
            .unwrap_or_else(|f| panic!("source parses: {f:?}\n--- src ---\n{src}"));
        let rendered = render(schema, &instance);
        assert_eq!(rendered, src, "render(parse(x)) must equal x");
    }

    /// A canonical two-level instance: one release (`1.2.0`) carrying a `date` field
    /// and >=2 nested change-groups (`#added`, `#fixed`), each with bare-prose notes.
    /// The exact frozen byte form the recursive writer emits — the parent at `###`,
    /// its field group, then the nested change-groups at `####` (`2 + nesting-depth`).
    const CANONICAL_TWO_LEVEL: &str = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

<!-- fields -->
- date: 2026-06-14

#### Added  {#added}

OAuth device-code flow.

#### Fixed  {#fixed}

Session fixation on logout.
";

    /// Done-criterion (i): `render(parse(src)) == src` byte-for-byte on a canonical
    /// two-level instance (a release with >=2 nested change-groups), and
    /// `write → parse → write` byte-identical. The nested `####` items survive the
    /// round-trip in order, each carrying its bare-prose notes.
    #[test]
    fn canonical_two_level_round_trips_byte_stable() {
        let schema = changelog_schema();
        let src = CANONICAL_TWO_LEVEL;

        // render(parse(src)) == src.
        assert_byte_stable(&schema, src);

        // The nested change-groups survive the parse, in order, under the release.
        let instance = instance_from_source(&schema, src).expect("canonical two-level parses");
        let releases = instance
            .sections
            .iter()
            .find(|s| s.id == "releases")
            .expect("releases section present");
        assert_eq!(releases.items.len(), 1, "one release");
        let release = &releases.items[0];
        assert_eq!(release.id, "1-2-0");
        let nested_ids: Vec<&str> = release.items.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(
            nested_ids,
            ["added", "fixed"],
            "both nested change-groups present, in order",
        );

        // write → parse → write byte-identical.
        let first = render(&schema, &instance);
        let second = render(
            &schema,
            &instance_from_source(&schema, &first).expect("re-parses"),
        );
        assert_eq!(
            first, second,
            "write → parse → write must be byte-identical"
        );
    }

    /// Done-criterion (ii) COLD: an **empty** two-level structure round-trips
    /// byte-stable, then filling one nested leaf at a time keeps EACH intermediate
    /// whole-document state byte-stable. The assertion is over the WHOLE document at
    /// every step — never scoped to a filled subtree (the M13 masking-test ban).
    #[test]
    fn cold_fill_one_nested_leaf_at_a_time_is_byte_stable() {
        let schema = changelog_schema();

        // The empty two-level structure: a release with its `date` field and two
        // empty change-group homes (no notes prose yet) — the canonical empty-skeleton
        // form, each empty nested change-group carrying the empty-single-slot body in its
        // M26 one-blank canonical form (`#### …{#id}` then a single blank line, the shape
        // `add_item` mints + the parser round-trips).
        let empty = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

<!-- fields -->
- date: 2026-06-14

#### Added  {#added}

#### Fixed  {#fixed}
";
        assert_byte_stable(&schema, empty);

        // Fill the FIRST nested leaf (`#added`'s notes); `#fixed` stays empty.
        let one_filled = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

<!-- fields -->
- date: 2026-06-14

#### Added  {#added}

OAuth device-code flow.

#### Fixed  {#fixed}
";
        assert_byte_stable(&schema, one_filled);

        // Fill the SECOND nested leaf (`#fixed`'s notes) too — both authored.
        assert_byte_stable(&schema, CANONICAL_TWO_LEVEL);
    }

    /// Done-criterion (iii) WARM: re-parsing a doc with one release and appending a
    /// **second** release (each with its own nested change-groups) re-renders
    /// byte-stable with both present. The two same-anchor nested groups (`#added` in
    /// each release) coexist across two parents — parent-scoped anchor uniqueness.
    #[test]
    fn warm_append_a_second_release_is_byte_stable() {
        let schema = changelog_schema();

        // Re-parse the one-release doc, append a second release in memory, re-render,
        // and assert the re-render round-trips byte-stable with both releases present.
        let mut instance =
            instance_from_source(&schema, CANONICAL_TWO_LEVEL).expect("one-release doc parses");
        let releases = instance
            .sections
            .iter_mut()
            .find(|s| s.id == "releases")
            .expect("releases section");
        releases.items.push(ItemContent {
            id: "1-3-0".to_string(),
            title: "1.3.0".to_string(),
            slot: None,
            slots: Vec::new(),
            fields: vec![Field {
                key: "date".to_string(),
                value: Value::Scalar("2026-07-01".to_string()),
            }],
            items: vec![ItemContent {
                id: "added".to_string(),
                title: "Added".to_string(),
                slot: Some("Audit log export.".to_string()),
                slots: Vec::new(),
                fields: Vec::new(),
                items: Vec::new(),
            }],
        });

        let rendered = render(&schema, &instance);
        // Both releases present, in append order.
        assert!(
            rendered.contains("### 1.2.0  {#1-2-0}"),
            "first release: {rendered}"
        );
        assert!(
            rendered.contains("### 1.3.0  {#1-3-0}"),
            "second release: {rendered}"
        );
        // The same `#added` anchor appears in BOTH releases (parent-scoped).
        assert_eq!(
            rendered.matches("#### Added  {#added}").count(),
            2,
            "the #added anchor coexists across two parents: {rendered}",
        );
        // The re-rendered two-release doc round-trips byte-stable.
        assert_byte_stable(&schema, &rendered);
    }
}

#[cfg(test)]
mod nested_locator {
    //! M22 Increment 1, T5 — the parent-scoped path locator + nested splice setters
    //! (review finding S1/C1). The global single-level `locate_item_block` matched an
    //! item only at `###`, by anchor alone, **globally** — so it could not reach a
    //! nested `####` item and would be ambiguous when two parents each carry a same
    //! -anchor nested group (two releases each with `#added`). The locator here walks
    //! each path segment **within its parent's byte region**, so a nested write lands
    //! on exactly the addressed item and leaves its same-anchor sibling under the other
    //! parent byte-untouched.

    use super::*;
    use crate::schema::Schema;

    /// The `changelog`-shaped two-level schema (a release → nested change-groups).
    fn changelog_schema() -> Schema {
        let yaml = b"\
type: changelog
id-from: title
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: date, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
              - { id: ticket, type: string }
";
        crate::schema::load_schema(yaml).expect("changelog schema loads")
    }

    /// Two releases, **each** carrying a `#added` nested change-group with distinct
    /// notes — the parent-scoped uniqueness fixture (same anchor across two parents is
    /// legitimate). The locator must disambiguate them by parent.
    const TWO_PARENT_TWO_LEVEL: &str = "\
# Changelog

## Releases

### 1.3.0  {#1-3-0}

<!-- fields -->
- date: 2026-07-01

#### Added  {#added}

Audit log export.

### 1.2.0  {#1-2-0}

<!-- fields -->
- date: 2026-06-14

#### Added  {#added}

OAuth device-code flow.
";

    /// Round-trip helper: `render(instance_from_source(src)) == src` over the WHOLE doc.
    fn assert_byte_stable(schema: &Schema, src: &str) {
        let instance = instance_from_source(schema, src)
            .unwrap_or_else(|f| panic!("source parses: {f:?}\n--- src ---\n{src}"));
        assert_eq!(
            render(schema, &instance),
            src,
            "render(parse(x)) must equal x"
        );
    }

    /// The fixture is itself canonical (so whole-doc byte-stability assertions hold).
    #[test]
    fn two_parent_fixture_is_canonical() {
        assert_byte_stable(&changelog_schema(), TWO_PARENT_TWO_LEVEL);
    }

    /// The parent-scoped locator resolves the nested `#added` under release `1-2-0`
    /// UNAMBIGUOUSLY — its region is the second `#### Added` block, not the first
    /// release's same-anchor group.
    #[test]
    fn locator_resolves_nested_item_within_its_parent() {
        let schema = changelog_schema();
        let region = locate_item_path(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-2-0", "added"],
        )
        .expect("nested #added under 1-2-0 located");
        let block = &TWO_PARENT_TWO_LEVEL[region.clone()];
        assert!(
            block.contains("OAuth device-code flow."),
            "the located block is 1-2-0's #added: {block:?}",
        );
        assert!(
            !block.contains("Audit log export."),
            "the located block must NOT be 1-3-0's #added: {block:?}",
        );

        // And the OTHER parent's #added resolves to its own distinct region.
        let other = locate_item_path(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-3-0", "added"],
        )
        .expect("nested #added under 1-3-0 located");
        assert_ne!(
            region, other,
            "the two same-anchor groups locate distinctly"
        );
        assert!(TWO_PARENT_TWO_LEVEL[other].contains("Audit log export."));
    }

    /// A nested `set-slot` on `1-2-0/added/notes` edits exactly that nested item's
    /// prose and leaves the same-anchor `#added` under `1-3-0` byte-untouched; the
    /// result round-trips byte-stable.
    #[test]
    fn nested_set_slot_targets_the_addressed_nested_item() {
        let schema = changelog_schema();
        let out = set_nested_item_slot(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            // Section-qualified chain: release `1-2-0` → nested section `changes` → group
            // `added` (review finding S1 — the canonical form the CLI passes through).
            &["1-2-0", "changes", "added"],
            "notes",
            "OAuth device-code flow and PKCE.",
        )
        .expect("nested slot present");
        // (a) 1-2-0's #added prose updated.
        assert!(out.contains("OAuth device-code flow and PKCE."));
        assert!(!out.contains("OAuth device-code flow.\n"));
        // (b) 1-3-0's same-anchor #added prose byte-untouched.
        assert!(out.contains("Audit log export."));
        // (c) round-trips byte-stable.
        assert_byte_stable(&schema, &out);

        // Inverse: setting 1-3-0/added leaves 1-2-0 untouched — not order-trivial.
        let out_other = set_nested_item_slot(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-3-0", "changes", "added"],
            "notes",
            "Audit log export and rotation.",
        )
        .expect("nested slot present");
        assert!(out_other.contains("Audit log export and rotation."));
        assert!(out_other.contains("OAuth device-code flow."));
        assert_byte_stable(&schema, &out_other);
    }

    /// A nested `set-field` (insert-absent) on `1-2-0/added/ticket` lands on exactly
    /// that nested item's field group and leaves `1-3-0/added` byte-untouched; the
    /// result round-trips byte-stable.
    #[test]
    fn nested_set_field_targets_the_addressed_nested_item() {
        let schema = changelog_schema();
        let out = set_nested_item_field_or_insert(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-2-0", "changes", "added"],
            "ticket",
            "JIRA-42",
        )
        .expect("nested field write");
        // 1-2-0's #added now carries the ticket; 1-3-0's #added does NOT.
        let idx_120 = out.find("OAuth device-code flow.").unwrap();
        let idx_130 = out.find("Audit log export.").unwrap();
        assert!(out.contains("- ticket: JIRA-42"));
        assert_eq!(
            out.matches("- ticket: JIRA-42").count(),
            1,
            "exactly one ticket"
        );
        // The ticket bullet sits in 1-2-0's region (after its notes prose), not 1-3-0's.
        let ticket_at = out.find("- ticket: JIRA-42").unwrap();
        assert!(ticket_at > idx_120, "ticket is under 1-2-0");
        assert!(idx_130 < idx_120, "1-3-0 precedes 1-2-0 in the fixture");
        // round-trips byte-stable.
        assert_byte_stable(&schema, &out);
    }

    /// A nested `add-item` mints a change-group into the addressed release's `changes`
    /// nested repeatable, scoped to that parent; the result round-trips byte-stable and
    /// the new group is reachable by the parent-scoped locator.
    #[test]
    fn nested_add_item_mints_into_the_addressed_parent() {
        let schema = changelog_schema();
        let out = add_nested_item(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-2-0"],
            "changes",
            "Changed",
            None,
            &[],
        )
        .expect("nested add-item");
        // The new `#### Changed  {#changed}` group sits under 1-2-0, not 1-3-0.
        assert!(out.contains("#### Changed  {#changed}"));
        assert_byte_stable(&schema, &out);
        let region = locate_item_path(&schema, &out, "releases", &["1-2-0", "changed"])
            .expect("the minted nested group is locatable under 1-2-0");
        // It is inside 1-2-0's region, after 1-2-0's existing #added.
        let block = &out[region];
        assert!(block.contains("#### Changed  {#changed}"));
        // 1-3-0 still carries only its original #added.
        assert!(out.contains("Audit log export."));
    }

    /// Done-criterion (b) "mis-naming its parent": a nested set against an address whose
    /// parent item id does not exist does NOT misfire onto the same-anchor nested item
    /// under a *different* parent — it returns `NotPresent`, never a wrong-item write.
    #[test]
    fn nested_set_against_a_wrong_parent_does_not_misfire() {
        let schema = changelog_schema();
        let err = set_nested_item_slot(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["9-9-9", "changes", "added"], // no such release
            "notes",
            "should not land anywhere",
        )
        .expect_err("the parent release does not exist");
        assert!(matches!(err, SpliceError::NotPresent { .. }));
    }

    /// `remove_nested_item` on the LAST block of the document — release `1-2-0`'s nested
    /// `#added`, whose region ends at EOF (no trailing separator to absorb). Removing it
    /// must leave the release's surviving `date` block canonically spaced (no dangling
    /// blank line), the same-anchor `#added` under `1-3-0` byte-untouched, and the result
    /// round-trips byte-stable. This is the edge the splice's leading-separator trim
    /// guards — asserted, not assumed.
    #[test]
    fn remove_nested_item_last_sibling_round_trips() {
        let schema = changelog_schema();
        let out = remove_nested_item(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-2-0", "changes", "added"],
        )
        .expect("nested #added under 1-2-0 present");
        // (a) 1-2-0's #added group is gone.
        assert!(!out.contains("OAuth device-code flow."));
        assert!(
            out.matches("#### Added  {#added}").count() == 1,
            "only 1-3-0's #added survives: {out:?}"
        );
        // (b) the same-anchor #added under 1-3-0 is byte-untouched.
        assert!(out.contains("Audit log export."));
        let added_130 = locate_item_path(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-3-0", "added"],
        )
        .expect("1-3-0/added located in source");
        assert!(
            out.contains(&TWO_PARENT_TWO_LEVEL[added_130]),
            "1-3-0's #added block survives byte-for-byte"
        );
        // (c) the surviving release ends canonically — no dangling blank line.
        assert!(out.ends_with("- date: 2026-06-14\n"));
        // (d) round-trips byte-stable.
        assert_byte_stable(&schema, &out);
    }

    /// `remove_nested_item` on a NON-last nested sibling — release `1-2-0` is first given a
    /// second change-group (`#### Changed`) so its `#added` precedes a sibling within the
    /// same parent; removing `#added` must leave `#### Changed` byte-intact and the result
    /// byte-stable (the region absorbs its own trailing separator).
    #[test]
    fn remove_nested_item_non_last_sibling_round_trips() {
        let schema = changelog_schema();
        // Build a 1-2-0 carrying both #added (first) and #changed (second).
        let two_groups = add_nested_item(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-2-0"],
            "changes",
            "Changed",
            Some("Token refresh window widened."),
            &[],
        )
        .expect("nested add-item mints #changed under 1-2-0");
        assert_byte_stable(&schema, &two_groups); // the derived fixture is itself canonical

        let out = remove_nested_item(
            &schema,
            &two_groups,
            "releases",
            &["1-2-0", "changes", "added"],
        )
        .expect("1-2-0/added present");
        // The non-last sibling is gone; the following #### Changed survives byte-intact.
        assert!(!out.contains("OAuth device-code flow."));
        assert!(out.contains("#### Changed  {#changed}"));
        assert!(out.contains("Token refresh window widened."));
        // 1-3-0's same-anchor #added is untouched.
        assert!(out.contains("Audit log export."));
        assert_byte_stable(&schema, &out);
    }

    /// `remove_nested_item` against a mis-named nested item id routes a clean block —
    /// `SpliceError::NotPresent`, source unchanged — never a wrong-item delete onto the
    /// same-anchor sibling under a different parent.
    #[test]
    fn remove_nested_item_mis_named_blocks_unchanged() {
        let schema = changelog_schema();
        let err = remove_nested_item(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-2-0", "changes", "removed"], // no #removed group under 1-2-0
        )
        .expect_err("no such nested group");
        assert!(matches!(err, SpliceError::NotPresent { .. }));
    }

    /// Regression (review finding): a **top-level** item field-write must land in the
    /// parent's OWN leaf region — before its first nested `####` child — even when that
    /// child already carries a field block. The single-level setter `locate_item_block`
    /// bounded the release's region at the next `###`/`##`, *spanning* the nested
    /// `#### Added`'s field group, so the release's `date` was appended INTO the nested
    /// group (corrupting the doc) or the nested group's same/other key produced a false
    /// reject. The fix routes the single-level setters through the parent-scoped path and
    /// bounds the field scan to the parent's own leaf region.
    ///
    /// Fixture: one release whose nested `#### Added` was authored *first* with a `ticket`
    /// field block; then the top-level `date` is set on the release. `date` must land in
    /// the release's own region (between its heading and the `#### Added`), and the result
    /// must re-parse conformant + byte-stable.
    #[test]
    fn top_level_field_write_after_nested_child_lands_in_parent_region() {
        let schema = changelog_schema();
        // A release carrying ONLY a nested `#### Added` (with a `ticket` field block) —
        // the release has no own field group yet (the cold-fill-after-nesting case).
        let src = "\
# Changelog

## Releases

### 1.0.0  {#1-0-0}

#### Added  {#added}

<!-- fields -->
- ticket: JIRA-9
";
        assert_byte_stable(&schema, src);

        // Set the TOP-LEVEL release `date` field (a single-level item address).
        let out = set_item_field_or_insert(&schema, src, "releases", "1-0-0", "date", "2026-06-14")
            .expect("top-level date field write");

        // (a) The result re-parses conformant and is byte-stable.
        assert_byte_stable(&schema, &out);

        // (b) `date` belongs to the RELEASE, not the nested `#### Added` change-group.
        let instance = instance_from_source(&schema, &out).expect("conforms");
        let release = instance
            .sections
            .iter()
            .find(|s| s.id == "releases")
            .and_then(|s| s.items.iter().find(|i| i.id == "1-0-0"))
            .expect("release present");
        assert!(
            release
                .fields
                .iter()
                .any(|f| f.key == "date"
                    && matches!(&f.value, Value::Scalar(v) if v == "2026-06-14")),
            "the release itself carries the date field; out:\n{out}",
        );
        let added = release
            .items
            .iter()
            .find(|i| i.id == "added")
            .expect("nested #added present");
        assert!(
            !added.fields.iter().any(|f| f.key == "date"),
            "the nested #added must NOT have absorbed the release's date; out:\n{out}",
        );
        // The nested child keeps its own ticket field intact.
        assert!(
            added.fields.iter().any(|f| f.key == "ticket"),
            "the nested #added keeps its ticket field; out:\n{out}",
        );

        // (c) Byte placement: the release's `date` bullet precedes its `#### Added`.
        let date_at = out.find("- date: 2026-06-14").expect("date bullet present");
        let added_at = out.find("#### Added").expect("nested heading present");
        assert!(
            date_at < added_at,
            "the release's date bullet must sit in the release's OWN region, before its \
             nested #### Added; out:\n{out}",
        );
    }
}
