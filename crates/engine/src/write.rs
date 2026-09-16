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
/// driving case (no slot, a `date` field, then nested `#### change-group` items).
///
/// **The slot and nested paths DO coincide (M49 T2).** They are written here as
/// independent stages — sub-labels, then field group, then children — and always were;
/// what stood in the way was the *reader*, which bounded an item's leaves at the first
/// deeper heading and so read a multi-slot item's own sub-labels as anchor-less nested
/// items. With that boundary schema-keyed ([`parse::is_item_slot_sub_label`]) the
/// combination is legal rather than banned, and this renderer needs no arm for it: a
/// `{multi-slot, nested}` item emits its `#### <Leaf-Title>` leaves at `depth + 1` and
/// its children's `#### <title>  {#id}` headings at the same depth, told apart by the
/// anchor. The sentence this replaces claimed the two paths never coincide, and was
/// false the moment the reader admitted the shape.
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
///
/// Crate-visible so the store-read path renders a **body** field-group slice through the
/// writer's own emitter (M42 — `design/doc-read-surface.md` → a fields-only section slice
/// serves its fields: a canonical re-emit, never a second field format).
pub(crate) fn append_field_group(out: &mut String, fields: &[Field]) {
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
///
/// Crate-visible for the store-read path's **header** field-group slice (see
/// [`append_field_group`]).
pub(crate) fn emit_bare_fields(fields: &[Field]) -> String {
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
///
/// Crate-visible so the *recognition* side ([`crate::parse`]'s `heading_matches`,
/// which renormalizes a heading back to its id) can be fenced against the rendering
/// rule it inverts, over the real shipped section-id population.
pub(crate) fn heading_text(id: &str) -> String {
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
                    id: "options".to_string(),
                    slot: Some("Alternatives were weighed and rejected.".to_string()),
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
                    id: "options".to_string(),
                    slot: Some("Alternatives were weighed and rejected.".to_string()),
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
                    id: "options".to_string(),
                    slot: Some("Alternatives were weighed and rejected.".to_string()),
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
/// the source (so the caller must route to the *generation* path), the source does not
/// conform to the schema (so no target can be located at all — the failed parse's own
/// findings ride along), or the schema declares no such section at all.
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
    ///
    /// Carries the **parse findings** the failed parse produced (M45 Inc 2 T4): they
    /// are what say *where* and *why* the buffer broke, and a reject that drops them
    /// leaves the agent holding only the class of the failure. Every construction site
    /// has them in hand at the `map_err` boundary.
    NotConformant {
        /// The located conformance diagnostics from the failed parse, in the parser's
        /// own order — the first is the surfaced break.
        findings: Vec<Finding>,
    },
    /// The **schema** declares no section by this id, so the write addresses a shape
    /// that does not exist. Distinct from [`SpliceError::NotConformant`]: the buffer is
    /// not the broken thing and there are no parse findings to name — which is exactly
    /// why this cannot share the conformance sentence (it would render an empty
    /// diagnosis). The [`GenerateError::UnknownSection`] sibling on the generation path.
    UndeclaredSection {
        /// The section id the caller named — **one hop of the address, as typed**, never
        /// a path this walk assembled (M51 — EC-25).
        section: String,
        /// For a **nested** repeatable, the top-level section the address entered
        /// through; `None` when `section` is that top-level hop itself.
        ///
        /// It exists so the nested reject can name *which* hop is undeclared without
        /// echoing back a truncated address. Naming the bare id alone was rejected at
        /// M49 for a good reason — "no section `bogus`" reads as a claim about a
        /// top-level `bogus` the schema has none of either — and the answer then was to
        /// render the address path down to the failing hop, which made the sentence a
        /// *proper prefix of what the caller typed*: an address in no argv, that the
        /// reader must diff against their own to place. The locus carries M49's
        /// disambiguation and quotes only hops the caller typed.
        under: Option<String>,
    },
    /// The addressed **slot leaf is not declared** where the write addressed it — on the
    /// item block the address bottoms out in, or (the CLI's section arm) on a simple
    /// section, whose prose slot is the section itself and carries no leaf hop.
    ///
    /// The slot sibling of [`GenerateError::UnknownField`], and it carries the **same**
    /// `write.unknown-field` code: an undeclared address is one contract member whichever
    /// leaf kind it names, and the schema can answer it, so the code's
    /// `jigc doc schema <doctype>` route is the followable repair for both. Only the
    /// sentence differs — a `set-slot` reject says *slot*, because that is what the agent
    /// addressed (M47 — the undeclared-address table; `DECISIONS.md` → 2026-07-26 M47
    /// Settle, Decision 9).
    UnknownLeaf {
        /// The undeclared slot leaf id.
        leaf: String,
        /// Where the write addressed it (`item "1-3-0/changes" in section "releases"`, or
        /// `section "..."` for the section arm) — free-form, so the section arm can name
        /// the leaf-less address form in the same breath. The item chain is rendered by
        /// [`hop_path`], in the caller's own address syntax.
        at: String,
    },
}

/// Render an address hop chain **in the caller's own address syntax** — `/`-joined, the
/// separator [`crate::address`] parses — for every reject sentence that echoes one back.
///
/// The doors hold these chains as `&[&str]`, and a `{chain:?}` render reaches the reader
/// as `["1-3-0", "changes", "no-such-group"]`: a Rust notation that is in no argv and is
/// not the address grammar at all, while the sibling door one rank away already echoed
/// the typed `"1-3-0/changes/no-such-group"`. One helper, so the same miss cannot come
/// back in two notations again (M51 — EC-25; the echo column of
/// `crates/cli/tests/write_miss_shape_axis.rs`).
fn hop_path(ids: &[&str]) -> String {
    ids.join("/")
}

impl std::fmt::Display for SpliceError {
    /// The reject's sentence, one rendering for every consumer — the write-path
    /// [`splice_error_finding`] and the milestone record's flip/self-ref findings alike,
    /// so a carried parse break reads the same wherever it surfaces.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpliceError::NotPresent { what } => write!(f, "{what} is not present"),
            SpliceError::NotConformant { findings } => match findings.first() {
                Some(first) => write!(
                    f,
                    "the source does not conform to the schema ({})",
                    first.message
                ),
                None => write!(f, "the source does not conform to the schema"),
            },
            SpliceError::UndeclaredSection { section, under } => match under {
                Some(top) => write!(
                    f,
                    "no nested section {section:?} declared under section {top:?} in the schema"
                ),
                None => write!(f, "no section {section:?} declared in the schema"),
            },
            SpliceError::UnknownLeaf { leaf, at } => {
                write!(f, "no slot {leaf:?} declared on {at}")
            }
        }
    }
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
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
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
        .ok_or_else(|| SpliceError::UndeclaredSection {
            section: section_id.to_string(),
            under: None,
        })?;
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

/// `set-title` (H1 present): rewrite the document's `# H1` line to `# {title}`, leaving
/// every other byte intact — the sole splice the `display-title-changed` transform kind
/// needs (`design/corpus-migration.md` → Relocation: display-title-changed rewrites only
/// the H1; `# changelog` → `# Changelog`).
///
/// Locates the first `# ` (H1) heading via the block parse and replaces its **whole line**
/// with the canonical `# {title}` form (`title` trimmed, mirroring [`render`]'s
/// `# {title}` emission). Re-rendering the H1 line — rather than a bare text-span splice
/// inside the heading — is the byte-stable path (the [`set_slot`] re-render-the-region
/// discipline), so `render(parse(out)) == out` holds. An absent H1 →
/// [`SpliceError::NotPresent`].
pub fn set_title(source: &str, title: &str) -> Result<String, SpliceError> {
    let blocks = parse::scan_blocks(source);
    let h1 = blocks
        .iter()
        .find_map(|b| match b {
            Block::Heading {
                level: HeadingLevel::H1,
                range,
                ..
            } => Some(range.start),
            _ => None,
        })
        .ok_or_else(|| SpliceError::NotPresent {
            what: "H1 title".to_string(),
        })?;
    // The H1 sits at a line start in canonical form; replace from there to end-of-line
    // (exclusive of the `\n`) with the canonical `# {title}` render, so the surrounding
    // blank line and every other byte are untouched.
    let line_end = source[h1..]
        .find('\n')
        .map(|n| h1 + n)
        .unwrap_or(source.len());
    Ok(splice(source, h1..line_end, &format!("# {}", title.trim())))
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
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
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

/// `rename` referrer-repoint: rewrite the `ref`-typed field `relation` in `source`
/// from `old` to `new`, leaving every other byte intact — the per-referrer half of
/// the `jigc rename` transaction ([write-commands.md](../../../design/write-commands.md)
/// → `jigc rename`, step 3).
///
/// Locates the schema section declaring the `ref` field `relation`, reads its current
/// value, and swaps the single `old` occurrence for `new`:
/// - a **scalar** ref (`derived-from: prd:old`) is replaced whole → `prd:new`;
/// - a **list-valued** ref (`cites: [adr:a, adr:old, adr:c]`) is parsed, the one
///   matching element swapped, and the **canonical whole-list re-emitted** via
///   [`Value::render`](crate::field_block::Value::render) (`[adr:a, adr:new, adr:c]`,
///   `", "`-separated) — siblings, order, and separator preserved byte-for-byte.
///
/// The new value bytes are spliced via [`set_field`], so the round-trip stays exact
/// (`render(parse(out)) == out`). `relation` not declared as a `ref` on any section,
/// the field absent, or `old` not among its value(s) → [`SpliceError::NotPresent`].
pub fn repoint_ref(
    schema: &Schema,
    source: &str,
    relation: &str,
    old: &str,
    new: &str,
) -> Result<String, SpliceError> {
    let section_id =
        ref_field_section(schema, relation).ok_or_else(|| SpliceError::NotPresent {
            what: format!("ref field {relation:?} in schema"),
        })?;

    // Read the field's current value to choose scalar-replace vs whole-list re-emit.
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
    let section = doc
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("section {section_id:?}"),
        })?;
    let field = section
        .fields
        .iter()
        .find(|f| f.key == relation)
        .ok_or_else(|| SpliceError::NotPresent {
            what: format!("field {relation:?} in section {section_id:?}"),
        })?;

    let new_value = match &field.value {
        Value::Scalar(v) if v == old => Value::Scalar(new.to_string()),
        Value::List(elems) if elems.iter().any(|e| e == old) => Value::List(
            elems
                .iter()
                .map(|e| if e == old { new.to_string() } else { e.clone() })
                .collect(),
        ),
        // The field is present but does not carry `old` — nothing to repoint here.
        _ => {
            return Err(SpliceError::NotPresent {
                what: format!("value {old:?} in field {relation:?}"),
            });
        }
    };

    set_field(schema, source, &section_id, relation, &new_value.render())
}

/// The id of the schema section declaring `relation` as a `ref`-typed field on a
/// **simple** (header / body) section — the section [`set_field`] addresses. The three
/// persisted ref-fields all live on a header section (`adr.supersedes`, `arch-doc.cites`,
/// `spec.derived-from`); a repeatable-item ref is not a `rename` repoint target in v1.
/// `None` if no simple section declares `relation` as a `ref`.
fn ref_field_section(schema: &Schema, relation: &str) -> Option<String> {
    schema.sections.iter().find_map(|s| match &s.body {
        SectionBody::Simple { fields, .. }
            if fields
                .iter()
                .any(|f| f.id == relation && f.ty == crate::schema::FieldType::Ref) =>
        {
            Some(s.id.clone())
        }
        _ => None,
    })
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
    // Rank 1 — the schema-declaredness question, ahead of the parsed-doc lookup below.
    if let Some(err) = undeclared_section_splice(schema, section_id) {
        return Err(err);
    }
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
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
        .ok_or_else(|| SpliceError::UndeclaredSection {
            section: section_id.to_string(),
            under: None,
        })?;
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

/// `set-field --unset` (header / simple-body field present): remove the field's line
/// from `section_id`, leaving every other byte intact — the byte-stable **splice-remove**
/// primitive (no prior one existed: [`remove_item`] deletes whole item blocks,
/// [`set_field`] splices/inserts a value). Two shapes, mirroring [`set_field_validated`]'s
/// header-vs-body split:
/// - a **header** field is a front-matter `key: value` line → the whole physical line is
///   removed (the surviving front-matter stays in schema order);
/// - a **simple body** field is a `- key: value` bullet in the trailing `<!-- fields -->`
///   group → [`unset_group_field`] removes just that bullet, **dropping the whole group**
///   (sentinel + its leading blank line) when it was the group's only field.
///
/// The eligibility guard (author-required / defaulted / `set:`-stamped fields are refused)
/// lives in [`unset_field_validated`]; this raw primitive only performs the byte edit. An
/// absent section / field, or a non-conformant source → [`SpliceError::NotPresent`] /
/// [`SpliceError::NotConformant`].
pub fn unset_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    field_key: &str,
) -> Result<String, SpliceError> {
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
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
    let schema_section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| SpliceError::UndeclaredSection {
            section: section_id.to_string(),
            under: None,
        })?;
    if schema_section.header {
        // Front-matter: remove the whole `key: value` physical line.
        let content = front_matter_content(source).ok_or_else(|| SpliceError::NotPresent {
            what: format!("front-matter for section {section_id:?}"),
        })?;
        let line = locate_field_line(source, content, field_key, false).ok_or_else(|| {
            SpliceError::NotPresent {
                what: format!("field {field_key:?} line"),
            }
        })?;
        Ok(splice(source, line, ""))
    } else {
        // Simple body section: remove the bullet from its trailing field group.
        let blocks = parse::scan_blocks(source);
        let present = present_body_sections(schema, source);
        let region = section_region(&blocks, source, section_id, &present).ok_or_else(|| {
            SpliceError::NotPresent {
                what: format!("section {section_id:?} body"),
            }
        })?;
        unset_group_field(source, &blocks, region, field_key)
    }
}

/// `set-field --unset` (repeatable item field present): remove `field_key`'s bullet from
/// the item addressed by `item_ids` (its parent-scoped id chain — `["1-2-0"]` for a
/// top-level item, `["1-2-0", "added"]` for a nested one), scoped to that item's OWN leaf
/// region ([`item_own_leaf_region`]) so a sibling's or a nested child's identically-keyed
/// bullet is out of range (the wrong-item / swallow-child guards [`set_item_field`] uses).
/// Drops the whole `<!-- fields -->` group when it clears the item's only field, else
/// splices out just that bullet. The eligibility guard lives in
/// [`unset_item_field_validated`]. An absent item / field, or a non-conformant source →
/// [`SpliceError::NotPresent`] / [`SpliceError::NotConformant`].
pub fn unset_item_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field_key: &str,
) -> Result<String, SpliceError> {
    // Re-parse for conformance (the surgical splice only runs over a conformant buffer).
    parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
    let blocks = parse::scan_blocks(source);
    let region = locate_item_path(schema, source, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {:?} in section {section_id:?}", hop_path(item_ids)),
        }
    })?;
    let leaf_region = item_own_leaf_region(schema, source, &blocks, section_id, item_ids, region);
    unset_group_field(source, &blocks, leaf_region, field_key)
}

/// Remove the `field_key` bullet from the sentinelled field group inside `region` (a
/// section body region or an item's own-leaf region), byte-stable against the canonical
/// writer form. When `field_key` is the group's **only** field, the whole group is
/// dropped — the blank line before the sentinel, the `<!-- fields -->` sentinel line, and
/// the bullet — leaving the body terminated by a single `\n` (the no-field canonical
/// form); otherwise just that bullet's physical line is spliced out, the sentinel and
/// sibling bullets untouched. No field group in `region`, or no such bullet →
/// [`SpliceError::NotPresent`].
fn unset_group_field(
    source: &str,
    blocks: &[Block],
    region: Range<usize>,
    field_key: &str,
) -> Result<String, SpliceError> {
    let (list_range, items) =
        field_group_list(blocks, region.clone()).ok_or_else(|| SpliceError::NotPresent {
            what: format!("field group for field {field_key:?}"),
        })?;
    if !field_key_in_list(blocks, source, &items, field_key) {
        return Err(SpliceError::NotPresent {
            what: format!("field {field_key:?} bullet"),
        });
    }
    if items.len() == 1 {
        // The group's only field: drop the whole group. `append_field_group` always
        // emits exactly `\n\n<!-- fields -->` before the sentinel, so removing one of the
        // two leading `\n` (at `sentinel.start - 1`) through the bullet's own newline
        // leaves the body terminated by a single `\n` — byte-identical to the canonical
        // no-field form.
        //
        // The end of that removal is **not** `list_range.end`: a `Block::List` range runs
        // to the end of the blank line that separates the list from whatever follows it,
        // and that separator is not the write's to take. Where the group is the last thing
        // in the region the two coincide, which is why every shipped corpus looked right;
        // where something follows it inside the region — a multi-slot item's nested
        // repeatable, most sharply — taking it too glues the next `#### <heading>` onto the
        // preceding prose, and clause 2 of the round-trip guarantees ("only that target's
        // bytes differ", `implementation/parsing.md`) is broken by a splice path that is
        // not the sanctioned re-render exception. So end at the last bullet's own newline,
        // computed by trimming the newlines the list block reaches past.
        let sentinel =
            field_sentinel_in(blocks, region).ok_or_else(|| SpliceError::NotPresent {
                what: format!("field-group sentinel for field {field_key:?}"),
            })?;
        let last_bullet_end =
            (source[..list_range.end].trim_end_matches('\n').len() + 1).min(source.len());
        Ok(splice(source, (sentinel.start - 1)..last_bullet_end, ""))
    } else {
        // A surviving sibling keeps the group: remove just this bullet's physical line
        // (scoped to the bullet list so a `- …:`-looking prose line is never matched).
        let line = locate_field_line(source, list_range, field_key, true).ok_or_else(|| {
            SpliceError::NotPresent {
                what: format!("field {field_key:?} bullet line"),
            }
        })?;
        Ok(splice(source, line, ""))
    }
}

/// The **whole physical line** span (line start through its terminating `\n`, inclusive)
/// of the `key: value` line whose key matches `key`, within byte `region` — the removal
/// analogue of [`locate_field_value`] (which returns only the value span). `bullet` strips
/// a leading `- ` marker before reading the key. `None` if no such line exists in `region`.
fn locate_field_line(
    source: &str,
    region: Range<usize>,
    key: &str,
    bullet: bool,
) -> Option<Range<usize>> {
    let text = &source[region.clone()];
    let mut line_start = region.start;
    for line in text.split_inclusive('\n') {
        let mut rest = line.trim_end_matches('\n');
        if bullet {
            rest = rest
                .strip_prefix("- ")
                .or_else(|| rest.strip_prefix('-'))
                .unwrap_or(rest);
        }
        if let Some((line_key, _)) = rest.trim_start().split_once(':')
            && line_key.trim() == key
        {
            return Some(line_start..line_start + line.len());
        }
        line_start += line.len();
    }
    None
}

/// The `<!-- fields -->` sentinel block range inside `region`, if present.
fn field_sentinel_in(blocks: &[Block], region: Range<usize>) -> Option<Range<usize>> {
    blocks.iter().find_map(|b| match b {
        Block::FieldSentinel { range }
            if range.start >= region.start && range.start < region.end =>
        {
            Some(range.clone())
        }
        _ => None,
    })
}

/// `set-item-field` (item field present): replace the **value** bytes of `field_key`
/// on the repeatable item addressed by `item_ids` — its section-qualified id chain
/// (`["1-2-0"]` for a top-level item, `["1-2-0", "changes", "added"]` for a nested one) —
/// scoped to that item's OWN byte region, so neither a sibling item's identically-keyed
/// field (the wrong-item write bug) nor a nested child's field block (the
/// parent-region-swallows-child bug) is matched. Not a wrapper over [`set_field`]: that
/// scans globally and would hit the first matching key. Re-parses for conformance, asserts
/// the item and its `field_key` are present, then locates the value span *within* the
/// item's own leaf region ([`locate_item_path`] narrowed by [`item_own_leaf_region`]) via
/// [`field_value_in_lines`] over the item's sentinelled `- key: value` bullets
/// (`bullet = true`). An absent item / field → [`SpliceError::NotPresent`].
///
/// **One function, the flat form as the single-segment chain** (M50 Increment 7 / T5, the
/// shape [`insert_item_slot`] took at T4). This splices the **value span**, so what is
/// around it cannot move at any depth — which is why [`crate::transform`]'s value-remap arm
/// splices through **this** at every locus, and why both `…_or_insert` entry points now
/// reach it first. The alternative a depth-aware dual used to take — re-rendering the
/// item's **whole committed region** from what the parse modelled — destroyed the bytes the
/// parse does not model (a paragraph hand-appended after the `<!-- fields -->` group), and
/// that re-render is retired.
pub(crate) fn set_item_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field_key: &str,
    new_value: &str,
) -> Result<String, SpliceError> {
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
    if !doc.sections.iter().any(|s| s.id == section_id) {
        return Err(SpliceError::NotPresent {
            what: format!("section {section_id:?}"),
        });
    }
    // The chain rendered as the **address path** a reader follows (`1-0-0/changes/added`),
    // never `Debug` of the slice: at the flat locus that is the single id these messages
    // always named, so widening the parameter did not widen the message.
    let at = item_ids.join("/");
    let item = nested_parsed_item(schema, &doc, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {at:?} in section {section_id:?}"),
        }
    })?;
    if !item.fields.iter().any(|f| f.key == field_key) {
        return Err(SpliceError::NotPresent {
            what: format!("field {field_key:?} on item {at:?}"),
        });
    }

    // The item's byte region — resolved through the parent-scoped path locator (which walks
    // the **physical** half of the chain, the segments that own a heading), then narrowed to
    // the item's OWN leaf region so a nested child's identically-keyed bullet is out of range
    // (the parent's field swallowed by its child's field block — the corruption bug) just as
    // a sibling item's is. The narrowing is schema-keyed and takes the **logical** chain: the
    // item's own multi-slot sub-labels are deeper headings too, and stopping at one would put
    // the item's own field group out of range ([`item_own_leaf_region`]).
    let blocks = parse::scan_blocks(source);
    let physical = physical_item_chain(schema, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {at:?} in section {section_id:?} not addressable"),
        }
    })?;
    let region = locate_item_path(schema, source, section_id, &physical).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {at:?} block"),
        }
    })?;
    let region = item_own_leaf_region(schema, source, &blocks, section_id, item_ids, region);
    let value_span = field_value_in_lines(source, region, field_key, true).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("field {field_key:?} value line on item {at:?}"),
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
///
/// **Private by design (M45).** The raw splice is reachable only through
/// [`set_slot_validated`]'s item arm, so no caller can write item-slot prose past the
/// address's heading-depth ceiling or skip [`validate_after`] — the gate is structural,
/// not a convention a caller has to remember (`parsing.md` → Slot heading-depth ceiling).
fn set_item_slot(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_id: &str,
    leaf_id: &str,
    new_prose: &str,
) -> Result<String, SpliceError> {
    // Rank 1 — an **undeclared** section outranks source presence
    // ([`undeclared_section_splice`]): the section lookup below reads the *parsed doc*, so
    // without this an address the schema never declared came back "section not present".
    if let Some(err) = undeclared_section_splice(schema, section_id) {
        return Err(err);
    }
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
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
    // The addressed leaf must be **declared** by the item's template before it is looked
    // for in the bytes ([`undeclared_slot_reject`] — the item is already resolved here, so
    // presence keeps its own `write.not-present` above and only a *present* item reaches
    // the declaredness question, the ordering the field sibling takes).
    if let Some(err) = undeclared_slot_reject(schema, section_id, &[item_id], leaf_id) {
        return Err(err);
    }
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
///
/// `pub(crate)` so the store read path ([`crate::store`]) resolves the *same*
/// section-qualified grammar on `doc show` — one canonical address, no second grammar
/// (M40, `design/doc-read-surface.md` → Nested repeatables join the pin).
pub(crate) fn physical_item_chain<'a>(
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
/// An absent item / leaf / non-conformant source → [`SpliceError`]. **Private by
/// design (M45)** — the same structural gate [`set_item_slot`] carries.
fn set_nested_item_slot(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    leaf_id: &str,
    new_prose: &str,
) -> Result<String, SpliceError> {
    // Rank 1, at every nesting depth (the top-level dual's ordering).
    if let Some(err) = undeclared_section_splice(schema, section_id) {
        return Err(err);
    }
    // … and at every hop past the first: a *nested*-section segment the schema never
    // declared is the same declaredness miss one level down.
    if let Some(err) = undeclared_nested_section_splice(schema, section_id, item_ids) {
        return Err(err);
    }
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
    let item = nested_parsed_item(schema, &doc, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {:?} in section {section_id:?}", hop_path(item_ids)),
        }
    })?;
    // The declaredness guard the top-level dual carries, at every nesting depth: the
    // chain's own template answers it (`undeclared_slot_reject`).
    if let Some(err) = undeclared_slot_reject(schema, section_id, item_ids, leaf_id) {
        return Err(err);
    }
    if item.slot_span(leaf_id).is_none() {
        return Err(SpliceError::NotPresent {
            what: format!("slot {leaf_id:?} in item {:?}", item.id),
        });
    }

    let physical = physical_item_chain(schema, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {:?} in section {section_id:?}", hop_path(item_ids)),
        }
    })?;
    let region = locate_item_path(schema, source, section_id, &physical).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {:?} block", hop_path(item_ids)),
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
/// [`set_item_field_or_insert`], and since M50 Increment 7 the *same* pair of splices: the
/// present bullet's **value span** ([`set_item_field`], depth-aware since T5), else the
/// in-group bullet insert ([`insert_item_field`], depth-aware since this fix) — never the
/// whole-item re-render, which destroyed the bytes the parse does not model. Like its
/// top-level dual it **adjudicates the value's declared type before touching bytes**
/// (closing the 2026-06-07 item-field parity gap) — a malformed value is rejected as
/// [`GenerateError::MalformedValue`], an **undeclared** field leaf as
/// [`GenerateError::UnknownField`]. A genuinely absent item / non-repeatable section →
/// [`GenerateError`].
pub(crate) fn set_nested_item_field_or_insert(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field_key: &str,
    new_value: &str,
) -> Result<String, GenerateError> {
    // Rank 1 — the undeclared section, as the top-level dual.
    if let Some(err) = undeclared_section_generate(schema, section_id) {
        return Err(err);
    }
    if let Some(err) = undeclared_nested_section_generate(schema, section_id, item_ids) {
        return Err(err);
    }
    // The depth-aware twin of [`set_item_field_or_insert`]'s pre-byte adjudication: the
    // undeclared **nested** item-field address is rejected here, not spliced (M47 — the
    // undeclared-address table; `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 9).
    match item_field_schema(schema, section_id, item_ids, field_key) {
        Some(field) => {
            if let Err(why) = check_value(field, &Value::Scalar(new_value.to_string())) {
                return Err(GenerateError::MalformedValue { why });
            }
        }
        None => {
            if let Some(err) =
                undeclared_field_reject(schema, source, section_id, item_ids, field_key)
            {
                return Err(err);
            }
        }
    }
    // **The same splice-or-insert the flat dual makes** ([`set_item_field_or_insert`]),
    // which is what this arm owes at depth: the present bullet takes the surgical
    // **value-span** splice and the absent one the in-group bullet insert, so the bytes
    // around the write cannot move. The whole-item re-render this used to perform destroyed
    // them — a paragraph hand-appended after a nested item's `<!-- fields -->` group is
    // unmodelled by the parse (clean under `parse_sections`, no `jigc validate` finding),
    // and re-rendering the region from what the parse modelled deleted it at `Ok`
    // (M50 Increment 7 fix — **No-data-loss**, the property [`insert_item_slot`]'s
    // pre-image guard defends for the same bytes one leaf-kind over).
    match set_item_field(schema, source, section_id, item_ids, field_key, new_value) {
        Ok(edited) => Ok(edited),
        // The field bullet is absent on a present item ⇒ generate it. (`set_item_field`
        // returns `NotPresent` both for an absent field *and* an absent item;
        // [`insert_item_field`] re-locates the item and routes a truly-absent item to a
        // `GenerateError::NotPresent`, so the item miss reaches the agent as the item miss
        // it is — the flat dual's split, verbatim.)
        Err(SpliceError::NotPresent { .. }) => {
            let new_field = Field {
                key: field_key.to_string(),
                value: Value::Scalar(new_value.to_string()),
            };
            insert_item_field(schema, source, section_id, item_ids, &new_field)
        }
        // The buffer is broken: carry the parse break's own diagnosis through, so the
        // shape reject names *what* broke rather than only its class.
        Err(e @ SpliceError::NotConformant { .. }) => Err(GenerateError::WrongShape {
            what: format!("{e}, for section {section_id:?}"),
        }),
        // An undeclared section is a shape miss, not a broken buffer.
        Err(SpliceError::UndeclaredSection { section, under }) => {
            Err(GenerateError::UnknownSection { id: section, under })
        }
        // The undeclared-leaf reject is the slot writers' ([`undeclared_slot_reject`]);
        // [`set_item_field`] addresses field bullets and constructs none. The mapping is the
        // total one the two error types already agree on — same class, same
        // `write.unknown-field` code.
        Err(SpliceError::UnknownLeaf { leaf, at }) => {
            Err(GenerateError::UnknownField { key: leaf, at })
        }
    }
}

/// `add-item` into a (possibly nested) repeatable, addressed by the **parent** item id
/// chain `parent_item_ids` plus the `nested_section_id` naming which nested repeatable
/// receives the item (`["1-2-0"]`, `"changes"` → a new change-group under release
/// `1-2-0`). The depth-aware dual of [`add_item`]: it re-derives the parent item,
/// appends a freshly-minted nested [`ItemContent`] (mint-empty / multi-slot skeleton per
/// the nested block's template), re-renders the parent at its depth, and splices it back
/// canonically. A minted id colliding with a present nested item under that parent →
/// [`GenerateError::AlreadyPresent`]; an unslugable title → [`GenerateError::UnslugableTitle`];
/// an **undeclared section** → [`GenerateError::UnknownSection`]
/// ([`undeclared_section_generate`], rank 1, ahead of the parse), a declared-but-wrong
/// shape → [`GenerateError::WrongShape`]. `slug_override` drives the minted `{#id}`
/// verbatim exactly as at top level ([`mint_item_id`]).
#[allow(clippy::too_many_arguments)]
pub fn add_nested_item(
    schema: &Schema,
    source: &str,
    section_id: &str,
    parent_item_ids: &[&str],
    nested_section_id: &str,
    title: &str,
    slug_override: Option<&str>,
    slot: Option<&str>,
    fields: &[Field],
) -> Result<String, GenerateError> {
    // Rank 1 — the undeclared section, answered from the schema alone and so **ahead of
    // the parse**: the address is wrong whatever state the source is in, and every sibling
    // door opens with the same check.
    if let Some(err) = undeclared_section_generate(schema, section_id) {
        return Err(err);
    }
    // … and at every hop past the first — including the **destination** nested section
    // itself, which is why the chain walked here is the parent items plus
    // `nested_section_id`: `#releases/1-3-0/bogus` addresses a nested section the schema
    // never declared, exactly as `#no-such-section` does one level up.
    let destination: Vec<&str> = parent_item_ids
        .iter()
        .copied()
        .chain(std::iter::once(nested_section_id))
        .collect();
    if let Some(err) = undeclared_nested_section_generate(schema, section_id, &destination) {
        return Err(err);
    }
    let doc = parse::parse_sections(schema, source).map_err(|_| GenerateError::WrongShape {
        what: format!("source does not conform to schema for section {section_id:?}"),
    })?;
    // Shape before presence (see [`set_nested_item_field_or_insert`]): the chain resolves
    // against the schema first, so a section/nested-repeatable miss stays a shape question
    // and only a genuinely absent **parent item** is an item-id miss.
    let parent_physical =
        physical_item_chain(schema, section_id, parent_item_ids).ok_or_else(|| {
            GenerateError::WrongShape {
                what: format!(
                    "parent item {:?} in section {section_id:?} not addressable",
                    hop_path(parent_item_ids)
                ),
            }
        })?;
    let parent =
        nested_parsed_item(schema, &doc, section_id, parent_item_ids).ok_or_else(|| {
            GenerateError::NotPresent {
                what: format!(
                    "parent item {:?} in section {section_id:?} not present",
                    hop_path(parent_item_ids)
                ),
            }
        })?;

    // The nested repeatable named by `nested_section_id` (the leaf id of a
    // `Leaf::Repeatable` in the parent's block), resolved through the schema by walking
    // the section-qualified parent chain in the section tree.
    let nested = nested_repeatable(schema, section_id, parent_item_ids, nested_section_id)
        .ok_or_else(|| GenerateError::WrongShape {
            what: format!(
                "nested repeatable {nested_section_id:?} not declared under {:?}",
                hop_path(parent_item_ids)
            ),
        })?;

    // The mint, through the same shared derivation top-level [`add_item`] uses —
    // anchor-injection reject, then `slugify(title)` or the verbatim `slug_override`.
    let id = mint_item_id(title, slug_override)?;
    if parent.items.iter().any(|i| i.id == id) {
        return Err(GenerateError::AlreadyPresent {
            what: format!("nested item {id:?} under {:?}", hop_path(parent_item_ids)),
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
                what: format!(
                    "parent item {:?} block not locatable",
                    hop_path(parent_item_ids)
                ),
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
    // Rank 1, as the top-level dual ([`remove_item`]).
    if let Some(err) = undeclared_section_splice(schema, section_id) {
        return Err(err);
    }
    if let Some(err) = undeclared_nested_section_splice(schema, section_id, item_ids) {
        return Err(err);
    }
    let doc = parse::parse_sections(schema, source)
        .map_err(|findings| SpliceError::NotConformant { findings })?;
    if nested_parsed_item(schema, &doc, section_id, item_ids).is_none() {
        return Err(SpliceError::NotPresent {
            what: format!("item {:?} in section {section_id:?}", hop_path(item_ids)),
        });
    }
    let physical = physical_item_chain(schema, section_id, item_ids).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {:?} in section {section_id:?}", hop_path(item_ids)),
        }
    })?;
    let region = locate_item_path(schema, source, section_id, &physical).ok_or_else(|| {
        SpliceError::NotPresent {
            what: format!("item {:?} block", hop_path(item_ids)),
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

/// `retitle-item` for a (possibly nested) repeatable item, addressed by its
/// section-qualified id chain `item_ids`: rewrite the item's heading line to the
/// canonical `{hashes} {title}  {{#id}}` form with the **`{#id}` anchor frozen** —
/// the retitle-without-reslug invariant's verb at item level
/// (`design/write-commands.md` → `jigc doc retitle-item`; `design/storage.md` →
/// Identity). [`set_title`]'s heading-line splice, carried over the parent-scoped
/// nested locator: only the heading line's bytes change, so the anchor, the item
/// body, and every other byte stay identical and `render(parse(out)) == out` holds.
///
/// Because the heading **is** the item's `id-from` field's value, the new title
/// **re-validates against that field's declared type** when the block declares it
/// (the [`set_nested_item_field_or_insert`] `check_value` guard shape) — a
/// non-member title against an enum id-source is rejected as
/// [`GenerateError::MalformedValue`] — but only **once the addressed item is known
/// to exist** (M47 — the write-verb × item-id-miss axis): the question is about
/// *that item's* id-source, so at an item that was never minted the item-id miss
/// answers first. (The *unconditional* enum-id-from refusal — a member-to-member
/// change is an identity change, not a retitle — is the CLI guard's, mirroring
/// `add-item`'s, and is presence-ranked the same way.) The anchor is read from the
/// located heading via [`anchor_of`], never re-derived from the new title. An
/// empty/whitespace title, a title carrying a control character (which would split
/// the heading line) or embedding the `{#` anchor pattern (which would out-shadow
/// the frozen anchor on re-parse) — all [`reject_malformed_title`], all checks on
/// the caller's own argument, so all ahead of presence. An **undeclared section** →
/// [`GenerateError::UnknownSection`] ([`undeclared_section_generate`], rank 1, ahead of
/// the parse); a declared but **non-repeatable** section, an unknown nested hop, or a
/// non-conformant source → [`GenerateError::WrongShape`]; a declared chain naming an item
/// that is simply **absent** → [`GenerateError::NotPresent`], routed to the containing
/// section's live ids.
pub fn retitle_item(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    new_title: &str,
) -> Result<String, GenerateError> {
    let title = new_title.trim();
    if title.is_empty() {
        return Err(GenerateError::WrongShape {
            what: "retitle title is empty".to_string(),
        });
    }
    // The anchor-injection reject: a title embedding `{#…}` would out-shadow the
    // frozen `{#id}` on re-parse — the reslug-hijack this verb's contract forbids.
    reject_malformed_title(title)?;

    // Rank 1 — within shape, the **undeclared** section is its own answer
    // ([`undeclared_section_generate`]): `write.unknown-section`, not the
    // `write.wrong-shape` that means "declared, but the wrong shape". Answered from the
    // schema alone, so it sits ahead of the parse, as every sibling door.
    if let Some(err) = undeclared_section_generate(schema, section_id) {
        return Err(err);
    }
    if let Some(err) = undeclared_nested_section_generate(schema, section_id, item_ids) {
        return Err(err);
    }
    let doc = parse::parse_sections(schema, source).map_err(|_| GenerateError::WrongShape {
        what: format!("source does not conform to schema for section {section_id:?}"),
    })?;
    // Shape before presence (see [`set_nested_item_field_or_insert`]): an undeclared or
    // non-repeatable section is a shape question the schema can answer; only a genuinely
    // absent item is an item-id miss.
    let physical = physical_item_chain(schema, section_id, item_ids).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!(
                "item {:?} in section {section_id:?} not addressable",
                hop_path(item_ids)
            ),
        }
    })?;
    if nested_parsed_item(schema, &doc, section_id, item_ids).is_none() {
        return Err(GenerateError::NotPresent {
            what: format!(
                "item {:?} in section {section_id:?} not present",
                hop_path(item_ids)
            ),
        });
    }
    // The id-from re-validation (see doc comment): resolved against the repeatable block
    // the chain bottoms out in, before any bytes move — but **after** presence (M47 — the
    // write-verb × item-id-miss axis). It adjudicates the new title against the field the
    // *addressed item's* id derives from and routes to `set-field` on that item, so at an
    // item that was never minted it would answer a question about a nonexistent item with
    // a route that blocks on the same absence. The two rejects above it are argument-shape
    // checks on the caller's own title (empty, anchor-injecting) — they claim nothing about
    // the item, so they keep their place.
    if let Some(repeatable) = chain_repeatable(schema, section_id, item_ids)
        && let Some(field) = repeatable.block.iter().find_map(|leaf| match leaf {
            crate::schema::Leaf::Field(field) if field.id == repeatable.id_from => Some(&**field),
            _ => None,
        })
        && let Err(why) = check_value(field, &Value::Scalar(title.to_string()))
    {
        return Err(GenerateError::MalformedValue { why });
    }
    let region = locate_item_path(schema, source, section_id, &physical).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!("item {:?} block not locatable", hop_path(item_ids)),
        }
    })?;

    // The item's heading is the located region's FIRST line; its anchor is read from
    // those bytes ([`anchor_of`]) — never re-derived from the new title — so the id is
    // frozen by construction. Replace the heading line (exclusive of its `\n`) with
    // the canonical `{hashes} {title}  {{#id}}` render ([`render_item_at`]'s frozen
    // heading form at this nesting depth), leaving every other byte untouched — the
    // [`set_title`] discipline at item level.
    let line_end = source[region.start..]
        .find('\n')
        .map(|n| region.start + n)
        .unwrap_or(source.len());
    let anchor =
        anchor_of(&source[region.start..line_end]).ok_or_else(|| GenerateError::WrongShape {
            what: format!(
                "item {:?} heading carries no {{#id}} anchor",
                hop_path(item_ids)
            ),
        })?;
    let hashes = "#".repeat(item_heading_level(physical.len()));
    let heading = format!("{hashes} {title}  {{#{anchor}}}");
    Ok(splice(source, region.start..line_end, &heading))
}

/// The [`crate::schema::Repeatable`] whose items a section-qualified id chain bottoms
/// out in: the section's own repeatable for a top-level chain (`["1-2-0"]`), descending
/// one declared nested repeatable per nested-section segment for a deeper chain
/// (`["1-2-0", "changes", "added"]` → the `changes` repeatable). The schema-side walk
/// [`physical_item_chain`] / [`item_field_schema`] share. `None` if the section is not
/// repeatable or a nested-section segment names no declared nested repeatable.
///
/// `pub(crate)` so the store read path ([`crate::store`]) resolves a trailing leaf hop
/// against the item template the chain bottoms out in (its `id-from` + declared slots)
/// — the same schema walk as the writer, no second grammar (M40).
pub(crate) fn chain_repeatable<'a>(
    schema: &'a Schema,
    section_id: &str,
    item_ids: &[&str],
) -> Option<&'a crate::schema::Repeatable> {
    let section = schema.sections.iter().find(|s| s.id == section_id)?;
    let SectionBody::Repeatable { repeatable } = &section.body else {
        return None;
    };
    let mut current = repeatable;
    let mut expect_item = true;
    for segment in item_ids {
        if expect_item {
            expect_item = false;
        } else {
            current = current.block.iter().find_map(|leaf| match leaf {
                crate::schema::Leaf::Repeatable { id, repeatable } if id == segment => {
                    Some(repeatable)
                }
                _ => None,
            })?;
            expect_item = true;
        }
    }
    Some(current)
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

/// The item's **own leaf region** — its full sub-tree `region` (from
/// [`locate_item_path`]) narrowed to `[region.start .. first nested child heading)`,
/// i.e. the bytes the item owns *before* its first nested `####`+ child. A field-group
/// scan/insert for this item's own field must be bounded to it, not to the full
/// sub-tree: the sub-tree spans the item's nested children, whose field blocks would
/// otherwise be matched (the parent's field appended INTO a child's group — the
/// corruption this guards). An item with no nested child keeps its whole region.
///
/// **The boundary is the schema-reserved depth, not "the first deeper heading" (M49
/// D3(A), completed at the M49 completion audit).** Two things are wrong with "deeper":
///
/// * a **multi-slot** item's own `#### <Leaf-Title>` sub-labels *are* deeper headings, so
///   on any block declaring ≥2 slots the region stopped at the item's own first
///   sub-label, putting the item's own trailing field group outside it — [`set_item_field`]
///   found no bullet, [`set_item_field_or_insert`] fell through to the insert half, and
///   [`insert_item_field`]'s cold-fill arm appended a **second bullet for the same key**
///   at exit 0, invisible to `jigc validate`, with the pinned `doc show --format json`
///   read returning the stale first value;
/// * a heading the address's own **ceiling declares free** ([`slot_ceiling`] —
///   `reserved_max` is a *level*, `first_allowed` one deeper) is likewise a deeper
///   heading, so ordinary author prose carrying one truncated the region the same way and
///   produced the same duplicate bullet — this time refused by the validate-after guard
///   as a **false** `write.non-reparseable`. The item's own region cannot end at a depth
///   the tool's own message told the author to use.
///
/// The rule the fix keys on — the context taken as a fence input rather than sniffed
/// from bytes (`design/surface-contract.md`): the region ends at the first heading **at
/// `own_level + 1`**, the one depth a nested child can render at, that is not a
/// **declared slot sub-label of this item's template**, and only where the template
/// declares a nested repeatable at all. An `{#id}` anchor is what identifies such a
/// heading as a nested item rather than a defect, so an anchored heading ends the region
/// even when its text matches a declared leaf title — without which a parent's leaf
/// region would swallow its children's field groups, which is the same corruption in the
/// other direction.
///
/// The predicate is the **parser's own** ([`parse::opens_item_nested_region`], M49 T2),
/// because the reader asks the identical question one seam over — where does this item's
/// nested region begin — and two implementations of one question is how this class
/// opened.
///
/// `item_ids` is the **section-qualified** chain (the one [`chain_repeatable`] walks), so
/// the template resolved is the one the addressed item is an instance of; a chain the
/// schema cannot walk yields no declared sub-labels, and the predicate then answers on
/// depth alone — the conservative narrower region rather than a wider one.
fn item_own_leaf_region(
    schema: &Schema,
    source: &str,
    blocks: &[Block],
    section_id: &str,
    item_ids: &[&str],
    region: Range<usize>,
) -> Range<usize> {
    let own_level = blocks.iter().find_map(|b| match b {
        Block::Heading { level, range, .. } if range.start == region.start => {
            Some(level_num_of(*level))
        }
        _ => None,
    });
    let Some(own_level) = own_level else {
        return region;
    };
    let template = chain_repeatable(schema, section_id, item_ids).map(parse::ItemTemplate::from);
    let first_child = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level, range, text, ..
            } if range.start > region.start
                && range.start < region.end
                && parse::opens_item_nested_region(
                    &source[range.clone()],
                    level_num_of(*level),
                    own_level,
                    text,
                    template.as_ref(),
                ) =>
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

/// The title-shape floor: reject an item title that would corrupt the heading it
/// splices into, as [`GenerateError::WrongShape`]. Two input classes fail here:
/// a **control character** (newline/tab/…), which would split the single heading
/// line and strand the `{#id}` anchor as prose; and the **`{#` anchor pattern** —
/// item identity is CLI-minted and frozen, and the parser reads a heading's
/// **first** `{#…}` as the item's anchor ([`anchor_of`] mirrors it), so a title
/// embedding `{#…}` would, once spliced/rendered, silently *replace* the item's
/// identity with the injected slug: the frozen anchor dies, every inbound address
/// dangles, and `render(parse(out)) == out` breaks. Titles are LLM-owned prose
/// (the determinism boundary), so both input classes are agent-reachable — every
/// title-taking write primitive fails closed here before any bytes move.
fn reject_malformed_title(title: &str) -> Result<(), GenerateError> {
    // The single-line floor: an item heading renders as `{hashes} {title}  {{#id}}`
    // on ONE line, so a control character (newline/tab/…) in the title would splice
    // a multi-line heading that strands the frozen/minted `{#id}` anchor on a prose
    // line — identity lost, `render(parse(out)) == out` broken. Rejected here
    // unconditionally because the id-from `check_value` re-validation only fires
    // when the repeatable's block *declares* its id-from leaf, and the loader never
    // requires that declaration (M40 completion finding).
    if title.chars().any(|c| c.is_control()) {
        return Err(GenerateError::WrongShape {
            what: format!(
                "title {title:?} contains a control character — a title renders on \
                 the single heading line, so a newline/tab would split the heading \
                 and strand its {{#id}} anchor"
            ),
        });
    }
    if title.contains("{#") {
        return Err(GenerateError::WrongShape {
            what: format!(
                "title {title:?} contains the anchor pattern \"{{#\" — a title cannot \
                 embed a {{#id}} anchor (item anchors are CLI-minted and frozen; an \
                 identity change goes through remove-item + add-item)"
            ),
        });
    }
    Ok(())
}

/// The `{#id}` an `add-item` mints, the ONE derivation both mint doors share
/// ([`add_item`] and [`add_nested_item`]): `slugify(title)` unless the caller supplies
/// an explicit `slug_override`, in which case that value **drives the id verbatim**
/// (`design/write-commands.md` → `jigc rename`'s `--slug` precedent — the same
/// override discipline [`crate::state::mint_task`] carries for a work-unit id).
///
/// [`reject_malformed_title`] runs **either way**: the title still becomes the
/// heading's visible text, so an embedded `{#…}` would out-shadow the minted anchor
/// whether the anchor was slugged or supplied. The unslugable-title reject is the
/// **derivation's** floor and so fires only on the derived arm — an override is
/// exactly what lets a title with no slug-able content mint at all.
///
/// The override is validated at the CLI boundary (via [`crate::slug::is_slug`]) and
/// used as-is here, so a malformed one never reaches the splice and a colliding one
/// rejects through the same [`GenerateError::AlreadyPresent`] route a colliding
/// slugged title takes.
fn mint_item_id(title: &str, slug_override: Option<&str>) -> Result<String, GenerateError> {
    reject_malformed_title(title)?;
    if let Some(slug) = slug_override {
        return Ok(slug.to_owned());
    }
    let id = crate::slug::slugify(title);
    if id.is_empty() {
        return Err(GenerateError::UnslugableTitle {
            title: title.to_string(),
        });
    }
    Ok(id)
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
pub const COMMIT_SECTION_SUMMARY: &str = "summary";
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
/// The `trailers` block is `id-from: key`, so each item's **heading** (`item.title`)
/// *is* the trailer key — never a `- key:` field bullet (a CLI-authored trailer via
/// `add-item …#trailers --title <Key>` carries no such bullet). The `value` comes from
/// the item's `value` field; an item with an empty title or a missing/empty value
/// contributes nothing.
fn trailer_lines(instance: &Instance) -> Vec<String> {
    let Some(section) = section_content(instance, COMMIT_SECTION_TRAILERS) else {
        return Vec::new();
    };
    section
        .items
        .iter()
        .filter_map(|item| {
            let key = item.title.trim();
            if key.is_empty() {
                return None;
            }
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
        /// The unknown section id — **one hop of the address, as typed**.
        id: String,
        /// The top-level section a **nested** miss was addressed under; `None` at the
        /// top level. The [`SpliceError::UndeclaredSection`] sibling's field, and it
        /// carries that field's whole rationale.
        under: Option<String>,
    },
    /// The section's shape does not match the requested generation (e.g. `add_item`
    /// into a simple section, or `generate_section` for the header).
    WrongShape {
        /// A human-readable description of the mismatch.
        what: String,
    },
    /// The **addressed item is absent** — the id was never minted (or names no item
    /// under the addressed parent). This is *not* a shape question: the schema names
    /// the declared shape, never the corpus's real item ids, so routing such a miss to
    /// `jigc doc schema <doctype>` is a dead end. It carries the [`SpliceError::NotPresent`]
    /// sibling's `write.not-present` code, whose route names the followable recovery —
    /// show the containing section and read its live item ids (M47 — the write-verb ×
    /// miss-shape axis; `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8).
    NotPresent {
        /// A human-readable description of the absent item.
        what: String,
    },
    /// The addressed **field leaf is not declared** on the item block the address bottoms
    /// out in. An undeclared address is not a value defect and not a corpus question: the
    /// schema *can* answer it, so it carries the `write.unknown-field` code — and with it
    /// the route — the `set-field --unset` sibling ([`unset_item_field_validated`]) has
    /// always emitted for the identical miss, rather than minting a second contract member
    /// for one half of one verb (M47 — the undeclared-address table;
    /// `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 9).
    UnknownField {
        /// The undeclared field key.
        key: String,
        /// Where the write addressed it (`item "1-3-0/changes" in section "releases"`) —
        /// the `--unset` sibling's own phrasing, so both doors emit the identical
        /// sentence, with the item chain rendered by [`hop_path`].
        at: String,
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
            under: None,
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
///
/// `slug_override` drives the minted `{#id}` **verbatim** when `Some`, decoupling the
/// item's identity from its heading text ([`mint_item_id`]); `None` mints from the
/// title, today's behaviour.
pub fn add_item(
    schema: &Schema,
    source: &str,
    section_id: &str,
    title: &str,
    slug_override: Option<&str>,
    slot: Option<&str>,
    fields: &[Field],
) -> Result<String, GenerateError> {
    let section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
            under: None,
        })?;
    if !matches!(section.body, SectionBody::Repeatable { .. }) {
        return Err(GenerateError::WrongShape {
            what: format!("section {section_id:?} is not repeatable"),
        });
    }

    // Mint the item anchor through the one shared derivation ([`mint_item_id`]):
    // `slugify(title)` — total, so a title with no slug-able content is rejected there
    // rather than emitting a malformed empty `{#}` anchor — or `slug_override`
    // verbatim. Either way the title itself is anchor-injection-rejected first.
    let id = mint_item_id(title, slug_override)?;

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

/// `fixed-slot→repeatable-with-default` — the M34 net-new migration primitive.
///
/// Promote a section that is a **simple `<<slot>>`** under `old_schema` into the
/// **repeatable** item-block it is under `new_schema`, carrying the old slot prose
/// **verbatim** as the **default first item** (no data loss). This is the one
/// structural transform with no existing write primitive: [`add_item`] assumes the
/// section is *already* repeatable (it returns [`GenerateError::WrongShape`] otherwise),
/// so it cannot perform the promotion itself (`design/corpus-migration.md` → The
/// deterministic transform → "the one net-new primitive").
///
/// The promoted item is a **single-slot** item: the old prose becomes the bare-prose
/// body of the new repeatable's single slot leaf, titled `title` (the `{#id}` anchor is
/// minted from it via [`crate::slug`], exactly as [`add_item`] mints). The change is a
/// **byte-stable splice**: only the target section's body region is rewritten (its slot
/// prose replaced by the canonical first-item block via [`render_item`]), every other
/// byte preserved — so `render(parse(out)) == out` holds over the proven splice
/// machinery.
///
/// Errors (typed, never a panic): the target is not declared / not repeatable under
/// `new_schema`, or its new item-template is not exactly single-slot (a multi-slot or
/// slot-less target has no deterministic prose mapping — a separate classified change);
/// the source section is not a simple slot under `old_schema`; the source does not
/// conform to `old_schema`; or `title` slugs to empty.
pub fn promote_slot_to_repeatable(
    old_schema: &Schema,
    new_schema: &Schema,
    source: &str,
    section_id: &str,
    title: &str,
) -> Result<String, GenerateError> {
    // The target's NEW shape must be a single-slot repeatable (the promotion target).
    let new_section = new_schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
            under: None,
        })?;
    let SectionBody::Repeatable { repeatable } = &new_section.body else {
        return Err(GenerateError::WrongShape {
            what: format!("section {section_id:?} is not repeatable in the new schema"),
        });
    };
    // The old single prose maps deterministically only to a **single-slot** item; a
    // multi-slot or slot-less target has no unambiguous home for it (a separate
    // classified change, out of scope for this primitive).
    if parse::ItemTemplate::from(repeatable).slot_ids.len() != 1 {
        return Err(GenerateError::WrongShape {
            what: format!(
                "section {section_id:?} new item-template is not single-slot; no \
                 deterministic prose mapping"
            ),
        });
    }

    // The target's OLD shape must be a simple slot — the thing being promoted.
    let old_section = old_schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
            under: None,
        })?;
    if !matches!(old_section.body, SectionBody::Simple { .. }) {
        return Err(GenerateError::WrongShape {
            what: format!("section {section_id:?} is not a simple slot in the old schema"),
        });
    }

    // Mint the default item's `{#id}` anchor from the title (total slugify; empty →
    // reject, mirroring [`add_item`]'s mint-site guard against a malformed empty `{#}`).
    let id = crate::slug::slugify(title);
    if id.is_empty() {
        return Err(GenerateError::UnslugableTitle {
            title: title.to_string(),
        });
    }

    // The old slot prose, verbatim — re-derived through the old schema (which the source
    // conforms to), so a non-conformant source is a typed error, never a panic.
    let prose = instance_from_source(old_schema, source)
        .map_err(|_| GenerateError::WrongShape {
            what: "source does not conform to the old schema".to_string(),
        })?
        .sections
        .into_iter()
        .find(|s| s.id == section_id)
        .and_then(|s| s.slot);

    // The canonical default first item carrying the old prose (single-slot bare-prose
    // form). [`render_item`] is the one source of item bytes, so the result is byte-stable.
    let item = render_item(&ItemContent {
        id,
        title: title.to_string(),
        slot: prose,
        slots: Vec::new(),
        fields: Vec::new(),
        items: Vec::new(),
    });

    // Replace the present section's body region (its old slot prose) with the first-item
    // block, re-attaching the region's tail separator — the same byte-stable
    // body-replacement [`add_item`] uses for a present-but-item-less section.
    let present = present_body_sections(new_schema, source);
    let blocks = parse::scan_blocks(source);
    let region = section_region(&blocks, source, section_id, &present).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!("section {section_id:?} region not locatable"),
        }
    })?;
    // The canonical separator after the promoted item: a single blank line before a
    // following `## ` (`\n\n`), or one terminating `\n` at EOF. Derived from whether a
    // section follows — *not* re-attached from the region tail, because a **simple**
    // empty-slot body carries one more trailing `\n` than the canonical repeatable form
    // (`## H\n\n\n` vs `## H\n\n`), and that debris would survive into `out` (the empty
    // case `add_item` never meets, since it only ever inserts into already-canonical
    // repeatable bodies).
    let separator = if region.end >= source.trim_end().len() {
        "\n"
    } else {
        "\n\n"
    };
    let replacement = format!("\n{}{}", item.trim_end_matches('\n'), separator);
    Ok(splice(source, region.start..region.end, &replacement))
}

/// The refusal [`insert_item_slot`] raises **beside** the shared [`GenerateError`]s: the
/// committed item region carries bytes the parse does not model, so the reshape's whole-item
/// re-render would destroy them.
///
/// It is its own type rather than a [`GenerateError`] variant because it is not a write-verb
/// reject and answers none of the `write.*` route split's questions
/// (`design/validation.md` → The `write.*` route split): no address missed, no declared shape
/// was violated — the doc conforms, and the defect is that *conformance does not model every
/// committed byte*. Its home is the migration report, which decides what to say about it
/// (`crates/cli/src/migrate_corpus.rs` → `halt_finding`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemSlotError {
    /// A shared generation failure (undeclared/non-repeatable section, an undeclared leaf).
    Generate(GenerateError),
    /// **The byte-fidelity refusal.** Re-rendering the item under the OLD template does not
    /// reproduce its committed region, so the region holds bytes the parse does not carry and
    /// the re-render would silently drop them. **No-data-loss** is a declared property of this
    /// pair — [`crate::transform`] invokes it by name to refuse `RemovedItemSlot` for exactly
    /// these bytes — so the fold refuses instead.
    UnmodelledContent {
        /// The repeatable section the item sits in.
        section: String,
        /// The item's **addressing chain from the section root**, rendered as the address
        /// body a reader follows: a bare `{#id}` anchor at the item locus, and the item id,
        /// the nested-section hop and the nested item id (`1-0-0/changes/added`) one level
        /// down. It is the chain rather than the anchor because same-anchor nested items
        /// under different parents are legal — two releases may each carry a `#### added`,
        /// and a refusal naming only `added` would not say which one to repair.
        item: String,
    },
}

impl From<GenerateError> for ItemSlotError {
    fn from(err: GenerateError) -> Self {
        ItemSlotError::Generate(err)
    }
}

/// `added-item-slot` — the M49 migration primitive that **reshapes a repeatable item
/// block's slot layout** from `old_schema`'s to `new_schema`'s, carrying every committed
/// slot's prose verbatim and minting the newly declared leaf `leaf_id` **empty** at its
/// schema-ordered offset.
///
/// **It takes a locus, not a section (M50 Increment 7 / T4).** `nested` is the chain of
/// nested repeatable leaf ids the change sits under
/// ([`crate::schema_diff::Locus::nested`]) — empty for the section's own item block, one
/// entry for a nested one — and the flat form is that empty case, not a separate function.
/// This is the `set_item_slot → set_nested_item_slot` generalization the write path already
/// makes, arriving one call site later: without it a nested slot add refused, and
/// `changelog.releases/changes` — the one nested repeatable the shipped doctype set carries
/// — was frozen at its birth shape with a route into this workspace.
///
/// The reshape is a per-item **re-render + splice**, not a new renderer: [`render_item_at`]
/// already emits a multi-slot item's sub-labels **one level deeper than the item** in the
/// caller's `slots` order (`####` under a top-level item, `#####` under a nested one), so
/// the primitive's whole job is to hand it the new order with the old prose in it.
/// [`set_item_slot`] cannot do this — it is **update-only**, refusing when
/// `slot_span(leaf_id)` is absent, which is every committed item by definition of this
/// change (the `set_item_field_or_insert` split, one leaf-kind over).
///
/// **The read goes through `old_schema`.** A v1-shaped instance does **not** parse under a
/// v2 multi-slot template — every declared sub-label is missing, which is a blocking
/// `conformance.item-slot-label-missing` — so the committed prose is re-derived through the
/// shape the doc actually conforms to, exactly as [`promote_slot_to_repeatable`] does.
///
/// **Below two declared slots this writes zero bytes, by construction.** A single-slot item
/// renders its prose *bare*, under no sub-heading ([`render_item_at`]), and an item whose one
/// slot is empty renders identically to a slot-less one — so a 0→1 add needs no splice at
/// all, and inventing one would only re-canonicalize bytes the change does not touch.
///
/// **The idempotency guard is per item**, because one doc can hold an item that already
/// carries the added sub-label beside one that does not (a re-run after a refused commit, a
/// hand-authored entry): such an item is left byte-identical
/// ([`carries_leaf_sub_label`]). *Declared bound:* an item whose committed bare prose
/// happens to carry a `#### <Added-Leaf-Title>` heading of its own reads as already
/// migrated; that ambiguity is inherent to the shape being migrated *from* (bare prose is
/// opaque), and the conservative answer — leave the bytes alone — is the one that cannot
/// destroy committed prose.
///
/// Errors (typed, never a panic): the section is not declared / not repeatable under either
/// schema, `leaf_id` is not a declared slot leaf of the new item template, or the source
/// does not conform to `old_schema`.
pub fn insert_item_slot(
    old_schema: &Schema,
    new_schema: &Schema,
    source: &str,
    section_id: &str,
    nested: &[String],
    leaf_id: &str,
) -> Result<String, ItemSlotError> {
    let new_template = parse::ItemTemplate::from(item_block(new_schema, section_id, nested)?);
    if !new_template.slot_ids.iter().any(|id| id == leaf_id) {
        return Err(GenerateError::WrongShape {
            what: format!(
                "section {section_id:?} new item-template declares no slot leaf {leaf_id:?}"
            ),
        }
        .into());
    }
    // One declared slot renders bare — there are no sub-label bytes at this arity, so the
    // reshape is the identity.
    if !new_template.is_multi_slot() {
        return Ok(source.to_string());
    }
    let old_template = parse::ItemTemplate::from(item_block(old_schema, section_id, nested)?);

    let doc = parse::parse_sections(old_schema, source).map_err(|_| {
        ItemSlotError::Generate(GenerateError::WrongShape {
            what: "source does not conform to the old schema".to_string(),
        })
    })?;
    let Some(parsed) = doc.sections.iter().find(|s| s.id == section_id) else {
        // The instance omits the section — no items, nothing to reshape.
        return Ok(source.to_string());
    };

    // THE ITEMS AT THE CHANGE'S OWN LOCUS, each carrying its addressing chain from the
    // section root. The descent is **per parent**, which is what makes same-anchor nested
    // items under different parents legal: two releases may each nest a `#### added`, and a
    // walk keyed on the anchor alone would reshape one of them twice and the other never.
    // The chain is the write path's own address shape — item id, then one nested-section hop
    // per level, then the nested item's id (`["1-0-0", "changes", "added"]`).
    let mut level: Vec<(Vec<String>, &parse::ParsedItem)> = parsed
        .items
        .iter()
        .map(|item| (vec![item.id.clone()], item))
        .collect();
    for hop in nested {
        level = level
            .into_iter()
            .flat_map(|(chain, item)| {
                item.items.iter().map(move |child| {
                    let mut chain = chain.clone();
                    chain.push(hop.clone());
                    chain.push(child.id.clone());
                    (chain, child)
                })
            })
            .collect();
    }

    let blocks = parse::scan_blocks(source);
    let mut edits: Vec<(Range<usize>, String)> = Vec::new();
    for (chain, item) in level {
        // The chain alternates item id and nested-section hop by construction above, so its
        // **physical** half — the ids that own a heading, the only ones the byte locator
        // walks — is every other segment ([`physical_item_chain`]'s reduction, without a
        // second schema walk to disagree with).
        let physical: Vec<&str> = chain.iter().step_by(2).map(String::as_str).collect();
        let Some(region) = locate_item_path(old_schema, source, section_id, &physical) else {
            continue;
        };
        if carries_leaf_sub_label(source, &blocks, &region, &new_template, leaf_id) {
            continue;
        }
        let mut content = item_content_from_parsed(item, source);
        // **The byte-fidelity guard, asked before a single byte moves.** The reshape below
        // replaces the item's **whole committed region** with a re-render of what the parse
        // modelled, so any committed byte the parse does *not* model is destroyed by it —
        // and one such byte is reachable by hand: jigc's canonical item order is
        // slots-then-fields, so "append a sentence to the end of this entry" lands **after**
        // the `<!-- fields -->` group, where the parse carries nothing, `doc show` renders
        // the slot without it and `jigc validate` reports no finding about it.
        //
        // So the pre-image is checked: re-render the item under the OLD template (the shape
        // the committed doc conforms to) and compare it to the committed region, modulo the
        // separator discipline the splice re-applies below. Equal means the parse models
        // every byte in the region and the re-render is faithful; unequal means it does not,
        // and the doc is **refused** — **No-data-loss** is a declared property of this pair
        // ([`crate::transform`] invokes it by name to refuse `RemovedItemSlot` for exactly
        // these bytes, and a silent whole-item re-render is the larger exception).
        //
        // **The pre-image renders at the item's own nesting depth**, which the physical
        // chain's length *is*. At the item locus that is 1 and the bytes are the flat form's,
        // unchanged; one level down it is 2, and rendering at 1 would emit `###` against a
        // committed `####` — a mismatch on **every** conformant nested item, refusing the
        // whole corpus with a message that reads like a doc problem and is a code one.
        //
        // *Declared bound:* the check is byte equality, so a **conformant but non-canonical**
        // item region (an extra blank line, a hand-spaced field bullet) is refused rather
        // than re-canonicalized. That is the safe side of a check that cannot tell the two
        // apart, and the refusal names the item, so the repair is a followable one-item edit.
        if source[region.clone()].trim_end_matches('\n')
            != render_item_at(&content, physical.len()).trim_end_matches('\n')
        {
            return Err(ItemSlotError::UnmodelledContent {
                section: section_id.to_string(),
                item: chain.join("/"),
            });
        }
        content.slots =
            reshaped_item_slots(&content, &old_template.slot_ids, &new_template.slot_ids);
        content.slot = None;
        let rendered = render_item_at(&content, physical.len());
        // The one separator discipline every item-bytes splice shares
        // ([`nested_replacement`], which [`set_nested_item_slot`] and
        // [`insert_item_field`]'s cold-fill arm also splice under): one blank line before a
        // following heading, exactly one terminating `\n` at EOF.
        let replacement = nested_replacement(source, &region, &rendered);
        edits.push((region, replacement));
    }

    // Applied **back to front** so every region — all located against the one parse of
    // `source` — stays valid as earlier items are rewritten.
    let mut out = source.to_string();
    for (region, replacement) in edits.into_iter().rev() {
        out = splice(&out, region, &replacement);
    }
    Ok(out)
}

/// The declared item block of `schema`'s repeatable section `section_id` **at the locus
/// `nested` names** — the one lookup [`insert_item_slot`] performs against both schemas.
///
/// `nested` is the locus's chain of nested repeatable leaf ids, outermost first
/// ([`crate::schema_diff::Locus::nested`]): empty for the section's own item block, one entry
/// for a nested one. Resolving the block **at the change's own locus** rather than by section
/// id alone is what keeps a nested leaf whose id also exists in the outer block from
/// answering with the outer declaration.
fn item_block<'a>(
    schema: &'a Schema,
    section_id: &str,
    nested: &[String],
) -> Result<&'a crate::schema::Repeatable, GenerateError> {
    let section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
            under: None,
        })?;
    let mut block = match &section.body {
        SectionBody::Repeatable { repeatable } => repeatable,
        SectionBody::Simple { .. } => {
            return Err(GenerateError::WrongShape {
                what: format!("section {section_id:?} is not repeatable"),
            });
        }
    };
    for hop in nested {
        block = block
            .block
            .iter()
            .find_map(|leaf| match leaf {
                crate::schema::Leaf::Repeatable { id, repeatable } if id == hop => Some(repeatable),
                _ => None,
            })
            .ok_or_else(|| GenerateError::WrongShape {
                what: format!(
                    "section {section_id:?} declares no nested repeatable {hop:?} at that locus"
                ),
            })?;
    }
    Ok(block)
}

/// Whether the item at `region` already carries the sub-label of the **added** leaf
/// `leaf_id` — the per-item idempotency guard [`insert_item_slot`] skips on.
///
/// **Keyed on the added leaf, never on "any declared sub-label."** At a 2→3 add the item
/// already carries both of the old shape's sub-labels while still needing the new one, so an
/// any-sub-label guard skips exactly the items the change exists to reshape and the fold's
/// output then fails to parse. The added leaf is the only heading whose presence means *this
/// change already landed here*.
///
/// The two compares are the parser's own — [`parse::is_item_slot_sub_label`] for *is this
/// deeper heading a sub-label at all* (multi-slot template · one level deeper · unanchored ·
/// a declared leaf title) and [`parse::heading_matches_label`] for *which leaf* — so the
/// reader's nested-region boundary, the writer's leaf-region boundary and this guard cannot
/// disagree about what a `#### <Leaf-Title>` heading is.
fn carries_leaf_sub_label(
    source: &str,
    blocks: &[Block],
    region: &Range<usize>,
    template: &parse::ItemTemplate,
    leaf_id: &str,
) -> bool {
    let Some(own_level) = blocks.iter().find_map(|b| match b {
        Block::Heading { level, range, .. } if range.start == region.start => {
            Some(level_num_of(*level))
        }
        _ => None,
    }) else {
        return false;
    };
    blocks.iter().any(|b| match b {
        Block::Heading {
            level, range, text, ..
        } => {
            range.start > region.start
                && range.start < region.end
                && parse::is_item_slot_sub_label(
                    &source[range.clone()],
                    level_num_of(*level),
                    own_level,
                    text,
                    template,
                )
                && parse::heading_matches_label(text, leaf_id)
        }
        _ => false,
    })
}

/// Re-key one item's committed slot prose onto the **new** template's leaf order: each new
/// leaf carries the prose the old shape held for it, and a leaf the old shape did not
/// declare mints **empty**.
///
/// The old shape's prose lives in one of two places, which is the arity split the renderer
/// itself makes: a **single-slot** item carries it bare in `slot` (owned by the old
/// template's one leaf id), a **multi-slot** item in the named `slots` entries. A
/// **slot-less** old template carries none, so every new leaf mints empty.
fn reshaped_item_slots(
    item: &ItemContent,
    old_ids: &[String],
    new_ids: &[String],
) -> Vec<(String, String)> {
    let carried: Vec<(&str, &str)> = if item.slots.is_empty() {
        match (old_ids.first(), item.slot.as_deref()) {
            (Some(id), Some(prose)) => vec![(id.as_str(), prose)],
            _ => Vec::new(),
        }
    } else {
        item.slots
            .iter()
            .map(|(id, prose)| (id.as_str(), prose.as_str()))
            .collect()
    };
    new_ids
        .iter()
        .map(|id| {
            let prose = carried
                .iter()
                .find(|(carried_id, _)| carried_id == id)
                .map(|(_, prose)| (*prose).to_string())
                .unwrap_or_default();
            (id.clone(), prose)
        })
        .collect()
}

/// `set-field` (header field **absent**): materialize the front-matter `key: value`
/// line for `field` at its **schema-ordered position** inside the `---` fence (after
/// the nearest preceding present header field, before the nearest following one), so
/// the present front-matter fields stay in schema order (`parsing.md` → Absent
/// structural homes generate at their schema-ordered position). The optional ref the
/// fillable form omits (`commit#header/implements`) is the driving case. When the
/// source carries **no `---` block at all** (a header-less doctype gaining a header in
/// a migration — the schema-version stamp's `prd`/`changelog` case), the fence is
/// **introduced** carrying this first field, byte-identical to [`render`]'s header
/// emission. A header field already present → [`GenerateError::AlreadyPresent`] (route
/// to [`set_field`]); a section that is not the header, or a key the schema does not
/// declare for it → [`GenerateError::WrongShape`] / [`GenerateError::UnknownSection`].
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
            under: None,
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
    // in physical order, located from the same block parse the reader uses. When the
    // source carries **no `---` block at all** — a header-less doctype gaining a header
    // in a corpus migration (the `prd`/`changelog` v0→v1 stamp case) — introduce the
    // fence carrying this first field ahead of the body. The bytes are byte-identical to
    // [`render`]'s header emission (`---\n<bare-fields>---\n\n`), so `render(parse(out))
    // == out` holds; subsequent fields then take the present-block path below.
    let content = match front_matter_content(source) {
        Some(content) => content,
        None => {
            let block = format!(
                "---\n{}---\n\n",
                emit_bare_fields(std::slice::from_ref(field))
            );
            return Ok(format!("{block}{source}"));
        }
    };
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
            under: None,
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
/// `field` into the repeatable item addressed by `item_ids`'s trailing field group,
/// materializing the `<!-- fields -->` sentinel **once** if the item has no field group yet
/// (the "first field into a freshly-minted empty item" case — [`add_item`] mints items
/// empty, so the item-leaf write path needs this insert half just as the header path
/// has [`insert_front_matter_field`]). The field-group scan is confined to the item's
/// OWN leaf region ([`locate_item_path`] narrowed by [`item_own_leaf_region`]), so
/// neither a sibling item's identically-keyed bullet (the wrong-item write bug) nor a
/// nested child's field group (the parent-region-swallows-child bug) is in range. The
/// bullet is appended
/// after the item's present bullets, mirroring [`insert_field`]. A field key already
/// present on the item → [`GenerateError::AlreadyPresent`] (route to [`set_item_field`]).
///
/// **One function, the flat form as the single-segment chain** (M50 Increment 7 fix — the
/// shape [`set_item_field`] took at T5 and [`insert_item_slot`] at T4). `item_ids` is the
/// section-qualified id chain (`["1-2-0"]` for a top-level item, `["1-2-0", "changes",
/// "added"]` for a nested one), and the cold-fill re-render below renders the item at the
/// depth its **physical** chain length names, so a nested item's `####` heading is not
/// re-emitted as `###`. Generalizing this — rather than leaving the nested seam on a
/// whole-item re-render — is what makes the nested field write **No-data-loss**: the
/// present-group arm inserts one line and moves nothing else, at any depth.
pub(crate) fn insert_item_field(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field: &Field,
) -> Result<String, GenerateError> {
    let section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .ok_or_else(|| GenerateError::UnknownSection {
            id: section_id.to_string(),
            under: None,
        })?;
    if !matches!(section.body, SectionBody::Repeatable { .. }) {
        return Err(GenerateError::WrongShape {
            what: format!("section {section_id:?} is not repeatable"),
        });
    }

    // The chain rendered as the **address path** a reader follows (`1-0-0/changes/added`),
    // never `Debug` of the slice: at the flat locus that is the single id these messages
    // always named, so widening the parameter did not widen the message.
    let at = item_ids.join("/");
    // The item's byte region, resolved through the parent-scoped path locator over the
    // chain's **physical** half (the ids that own a heading). The field-group scan is
    // bounded to the item's OWN leaf region (before its first nested item heading — NOT
    // before its first deeper heading, which on a multi-slot template is the item's own
    // `#### <Leaf-Title>` sub-label; [`item_own_leaf_region`]) so a nested child's field
    // group is never matched — without this the parent's field is appended INTO a child's
    // group (the corruption) or the child's same key triggers a false `AlreadyPresent`. The
    // cold-fill re-render below uses the FULL sub-tree region (children preserved).
    let blocks = parse::scan_blocks(source);
    let physical = physical_item_chain(schema, section_id, item_ids).ok_or_else(|| {
        GenerateError::WrongShape {
            what: format!("item {at:?} in section {section_id:?} not addressable"),
        }
    })?;
    let region = locate_item_path(schema, source, section_id, &physical).ok_or_else(|| {
        GenerateError::NotPresent {
            what: format!("item {at:?} in section {section_id:?} not present"),
        }
    })?;
    let leaf_region = item_own_leaf_region(
        schema,
        source,
        &blocks,
        section_id,
        item_ids,
        region.clone(),
    );

    let bullet = format!("- {}", emit_one_field(field));

    // Is there an existing field group (a sentinel + list) inside the item's OWN region?
    match field_group_list(&blocks, leaf_region.clone()) {
        Some((list_range, items)) => {
            // A present key is a surgical set-item-field, not a generation.
            if field_key_in_list(&blocks, source, &items, &field.key) {
                return Err(GenerateError::AlreadyPresent {
                    what: format!("field {:?} on item {at:?}", field.key),
                });
            }
            // Append the bullet right after the last present bullet (the list end).
            Ok(insert_after_line(source, list_range.end, &bullet))
        }
        None => {
            // No field group yet (the freshly-minted empty item): re-render the whole
            // item via [`render_item_at`] with the new field appended, then splice it over
            // the item's located block. Re-rendering (not a bullet-group splice at the
            // slot-prose end) is what makes this **byte-stable** on a mint-empty item:
            // an empty slot's canonical form (`### …{#id}\n\n<!-- fields -->`, the M26
            // one-blank form) is the writer's, and `render_item_at` is that writer — so
            // `render(parse(out)) == out` holds, where a slot-prose-end bullet insert would
            // mis-space the blank lines.
            //
            // **This arm cannot destroy unmodelled bytes, by construction of the shape it
            // fires on**: the one spot the canonical item form leaves for a hand-append is
            // *after* the `<!-- fields -->` group, and an item with no group has no such
            // spot — trailing prose is inside the slot span, which the parse carries. The
            // present-group arm above, which is where a hand-append can sit, moves nothing.
            let doc =
                parse::parse_sections(schema, source).map_err(|_| GenerateError::NotPresent {
                    what: format!("item {at:?} in section {section_id:?} not present"),
                })?;
            let item = nested_parsed_item(schema, &doc, section_id, item_ids).ok_or_else(|| {
                GenerateError::NotPresent {
                    what: format!("item {at:?} in section {section_id:?} not present"),
                }
            })?;
            let mut content = item_content_from_parsed(item, source);
            content.fields.push(field.clone());
            // Re-render the item **at its own nesting depth**, then re-attach the
            // **canonical** inter-block separator (not the recorded one): the located region
            // may carry non-canonical trailing blank-line debris, so the gap is recomputed
            // from the canonical form — one blank line before a following heading, exactly
            // one terminating `\n` at EOF ([`nested_replacement`], the one separator
            // discipline every item-bytes splice shares).
            let rendered = render_item_at(&content, physical.len());
            Ok(splice(
                source,
                region.clone(),
                &nested_replacement(source, &region, &rendered),
            ))
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
/// rejected as [`GenerateError::MalformedValue`], and an **undeclared** field leaf on a
/// present item as [`GenerateError::UnknownField`] — the address is adjudicated before the
/// bytes, so no write lands at an address the schema does not declare. A genuinely absent
/// item surfaces as [`GenerateError::NotPresent`]; a non-repeatable section as
/// [`GenerateError::WrongShape`].
pub(crate) fn set_item_field_or_insert(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_id: &str,
    field_key: &str,
    new_value: &str,
) -> Result<String, GenerateError> {
    // Rank 1 — the undeclared section, stated at the door rather than left to whichever
    // deeper lookup happens to notice it first ([`undeclared_section_generate`]).
    if let Some(err) = undeclared_section_generate(schema, section_id) {
        return Err(err);
    }
    // Shape/value adjudication **before any bytes move** — including the undeclared
    // address itself ([`undeclared_field_reject`]): the `None` arm used to fall straight
    // through and splice the bullet anyway, leaving the doc unreadable at exit 0 (M47 —
    // the undeclared-address table; `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 9).
    match item_field_schema(schema, section_id, &[item_id], field_key) {
        Some(field) => {
            if let Err(why) = check_value(field, &Value::Scalar(new_value.to_string())) {
                return Err(GenerateError::MalformedValue { why });
            }
        }
        None => {
            if let Some(err) =
                undeclared_field_reject(schema, source, section_id, &[item_id], field_key)
            {
                return Err(err);
            }
        }
    }
    match set_item_field(schema, source, section_id, &[item_id], field_key, new_value) {
        Ok(edited) => Ok(edited),
        // The field bullet is absent on a present item ⇒ generate it. (`set_item_field`
        // returns `NotPresent` both for an absent field *and* an absent item;
        // `insert_item_field` re-locates the item and routes a truly-absent item to a
        // `GenerateError::NotPresent`, so the item miss reaches the agent as the item
        // miss it is — the two absence cases stay distinguishable in the code, and both
        // now emit the code whose route names the containing section.)
        Err(SpliceError::NotPresent { .. }) => {
            let new_field = Field {
                key: field_key.to_string(),
                value: Value::Scalar(new_value.to_string()),
            };
            insert_item_field(schema, source, section_id, &[item_id], &new_field)
        }
        // The buffer is broken: carry the parse break's own diagnosis through, so the
        // shape reject names *what* broke rather than only its class.
        Err(e @ SpliceError::NotConformant { .. }) => Err(GenerateError::WrongShape {
            what: format!("{e}, for section {section_id:?}"),
        }),
        // An undeclared section is a shape miss, not a broken buffer — it lands on the
        // generation path's own code for exactly that.
        Err(SpliceError::UndeclaredSection { section, under }) => {
            Err(GenerateError::UnknownSection { id: section, under })
        }
        // The undeclared-leaf reject is the slot writers' ([`undeclared_slot_reject`]);
        // [`set_item_field`] addresses field bullets and constructs none. The mapping is
        // the total one the two error types already agree on — same class, same
        // `write.unknown-field` code — so the translation stays faithful if a future
        // field-side caller ever does raise it.
        Err(SpliceError::UnknownLeaf { leaf, at }) => {
            Err(GenerateError::UnknownField { key: leaf, at })
        }
    }
}

/// Render a body section's canonical block (`## Heading` + slot prose + optional
/// field group), with **no** surrounding blank lines — the caller's insertion adds
/// the separating blanks. Mirrors [`render_section`]'s body **per section shape** — a
/// simple section's slot/field body (taken directly, since the generation caller supplies
/// them rather than an `Instance`), or a **repeatable** section's **zero-item** body, whose
/// canonical form is one `\n` shorter (no empty-prose terminator, no field group): a
/// zero-item repeatable conforms, so an `AddedRepeatableSection` migration mints exactly
/// this (`design/corpus-migration.md` → The classifier's holes). Rendering a repeatable's
/// home as a *simple* body would over-pad it by one newline and leave the spliced doc
/// byte-**un**stable against the canonical writer form.
fn render_generated_section(section: &Section, slot: Option<&str>, fields: &[Field]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "## {}", heading_text(&section.id));
    out.push('\n');
    if !matches!(section.body, SectionBody::Repeatable { .. }) {
        out.push_str(slot.unwrap_or("").trim_end());
        append_field_group(&mut out, fields);
    }
    // A repeatable's zero-item body is exactly what `render_section`'s `Repeatable` arm
    // emits over an empty item list: the heading, the blank line, and nothing else (the
    // caller mints no items — the CLI invents none).
    // Drop the **single** trailing `\n` `append_field_group` leaves so the block is bare
    // and `insert_block` owns the surrounding blank lines. Exactly one — never a greedy
    // strip: a section whose slot is **empty** (`slot: Some("")`, an added *optional*
    // section's canonical form) renders `## Heading\n\n\n` (heading, blank, empty-prose
    // terminator), and dropping more than the one terminator would collapse the blank
    // line that `render_section`'s join preserves, leaving the spliced section
    // byte-**un**stable against the canonical writer form. A filled/field-carrying block
    // ends in exactly one `\n`, so `if` and the old greedy `while` are identical for it.
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

/// The present body sections, as `(schema id, heading start offset)` pairs in source
/// order. A present heading is an `## H2` whose text **renormalizes** to a schema
/// **body** section id ([`crate::slug::renormalize`] — the recognition rule, never the
/// mint rule: `slugify` caps and drops edge stopwords, so it would read a present
/// `## In Scope` as absent and duplicate the `in-scope` section). Built without
/// requiring a clean conformant parse — the target section is, by construction, absent.
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
                .find(|id| crate::slug::renormalize(text) == **id)
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

/// Insert `bullet` as a new line at the end of the bullet list ending at `at`, keeping the
/// bullet block **contiguous** — the canonical field-group form (`render`'s writer emits the
/// bullets as adjacent lines, then one blank before the next block).
///
/// `at` is the caller's list-block end, which for a **non-trailing** group runs *past* the last
/// bullet's terminating newline, over the blank line that separates the group from the following
/// `##`/`###` heading. Landing there splits the group (`- a\n\n- b\n## Next`) — a doc that
/// **re-parses fine but fails `render(parse(x)) == x`**, the retired byte-stability invariant, on
/// **both** insert callers ([`insert_field`] and [`insert_item_field`], i.e. the shipped
/// `set-field` insert half at both loci; `DECISIONS.md` → 2026-07-13 M42 Inc-5 T7). So the landing
/// offset **retreats over the trailing blank** to just past the last *content* line's newline. A
/// trailing group (`at` already just past the last bullet) is unaffected — which is exactly why
/// every trailing-item/trailing-section fixture passed while the shape was broken (the T6 lesson,
/// again).
fn insert_after_line(source: &str, at: usize, bullet: &str) -> String {
    let mut pos = source[..at.min(source.len())].trim_end().len();
    // Land just past the terminating newline of that last content line.
    if let Some(nl) = source[pos..].find('\n') {
        pos += nl + 1;
    } else {
        return format!("{source}\n{bullet}");
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

## Options

Alternatives were weighed and rejected.

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

## Options

Alternatives were weighed and rejected.

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
        // The decision prose line is line index 17 (the `## Options` section shifted it down 4).
        assert_only_lines_differ(CANONICAL_ADR, &out, &[17]);
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
            &["burst-allowance"],
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
            &["rate-limit"],
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
            &["ghost"],
            "implemented-by",
            "x",
        )
        .expect_err("no such item");
        assert!(matches!(missing_item, SpliceError::NotPresent { .. }));
        let missing_field = set_item_field(
            &schema,
            TWO_ITEM_SPEC,
            "criteria",
            &["rate-limit"],
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
             ## Context\n\n{context}\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\n{decision}\n\n\
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

## Options

Alternatives were weighed and rejected.

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
        assert_eq!(
            ids,
            ["status", "context", "options", "decision", "consequences"]
        );
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

## Options

Alternatives were weighed and rejected.

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
        assert_eq!(
            ids,
            ["status", "context", "options", "decision", "consequences"]
        );
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

## Options

Alternatives were weighed and rejected.

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
            None,
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
            None,
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
            let err = add_item(&spec_schema(), src, "criteria", title, None, Some("x"), &[])
                .expect_err("an unslugable title must not mint an empty anchor");
            assert!(
                matches!(err, GenerateError::UnslugableTitle { .. }),
                "title {title:?} should be UnslugableTitle, got {err:?}"
            );
        }
    }

    /// The `--slug` override's engine seam ([`mint_item_id`]): the minted `{#id}` is
    /// the caller's value **verbatim**, the heading keeps the title's own bytes, and
    /// the result still round-trips (`render(parse(out)) == out`). Two titles that
    /// slug alike therefore coexist — the limit the override exists to remove.
    #[test]
    fn add_item_slug_override_drives_the_anchor_verbatim() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        let schema = spec_schema();
        let one = add_item(
            &schema,
            src,
            "criteria",
            "Retry policy (v2)",
            None,
            None,
            &[],
        )
        .expect("the slugged mint");
        let two = add_item(
            &schema,
            &one,
            "criteria",
            "Retry policy v2",
            Some("retry-policy-v2-plain"),
            None,
            &[],
        )
        .expect("the overridden mint of a title that slugs identically");
        assert!(
            two.contains("### Retry policy (v2)  {#retry-policy-v2}"),
            "the slugged item is untouched; got:\n{two}",
        );
        assert!(
            two.contains("### Retry policy v2  {#retry-policy-v2-plain}"),
            "the override drives the anchor while the heading keeps its own text; got:\n{two}",
        );
        let reparsed = instance_from_source(&schema, &two).expect("the result conforms");
        assert_eq!(
            render(&schema, &reparsed),
            two,
            "the override is byte-stable"
        );
    }

    /// A title with **no slug-able content** mints under an override — the reject is
    /// the *derivation's* floor, and an override skips the derivation. (Without one it
    /// is still [`GenerateError::UnslugableTitle`]; see the test above.)
    #[test]
    fn add_item_slug_override_admits_an_unslugable_title() {
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
            "!!!___---",
            Some("the-punctuation-one"),
            None,
            &[],
        )
        .expect("an override supplies the id the title cannot");
        assert!(
            out.contains("### !!!___---  {#the-punctuation-one}"),
            "got:\n{out}"
        );
    }

    /// The anchor-injection reject fires **with** an override too: the title is still
    /// the heading's visible text, so an embedded `{#…}` would out-shadow the supplied
    /// anchor on re-parse exactly as it out-shadows a slugged one.
    #[test]
    fn add_item_slug_override_still_rejects_an_anchor_injecting_title() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        let err = add_item(
            &spec_schema(),
            src,
            "criteria",
            "Evil {#other-anchor} title",
            Some("supplied-anchor"),
            None,
            &[],
        )
        .expect_err("an anchor-carrying title must not mint, override or not");
        assert!(matches!(err, GenerateError::WrongShape { .. }));
    }

    /// `add-item` with a title carrying the `{#` anchor pattern is rejected as
    /// [`GenerateError::WrongShape`]: rendered, `### Evil {#other} title  {#minted}`
    /// would be re-read by the parser with `{#other}` (the FIRST `{#…}`) as the item's
    /// identity — not the minted anchor — so the emitted item address dangles and
    /// `render(parse(out)) == out` breaks (the retitle-item anchor-injection hole's
    /// mint-side twin; M40 blocking finding).
    #[test]
    fn add_item_anchor_syntax_title_is_rejected() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        let err = add_item(
            &spec_schema(),
            src,
            "criteria",
            "Evil {#other-anchor} title",
            None,
            Some("x"),
            &[],
        )
        .expect_err("an anchor-carrying title must not mint");
        assert!(matches!(err, GenerateError::WrongShape { .. }));
    }

    /// `add-item` with a title carrying a control character (newline/tab) is rejected
    /// as [`GenerateError::WrongShape`] before any bytes move: the item heading renders
    /// as `### <title>  {#id}` on ONE line, so `"A\nB"` would splice a two-line heading
    /// that pushes the minted `{#id}` anchor onto a prose line — identity lost,
    /// `render(parse(out)) == out` broken. No `check_value` runs on the mint-side title
    /// (only `reject_malformed_title` + slugify), so this reject is the only defense
    /// (M40 completion finding: the control-char-title hole, mint side).
    #[test]
    fn add_item_control_char_title_is_rejected() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria
";
        for title in ["A\nB", "A\tB"] {
            let err = add_item(&spec_schema(), src, "criteria", title, None, Some("x"), &[])
                .expect_err("a control-char title must not splice a multi-line heading");
            assert!(
                matches!(err, GenerateError::WrongShape { .. }),
                "title {title:?} should be WrongShape, got {err:?}"
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
        let out = add_item(&schema, src, "unreleased-changes", "Fixed", None, None, &[])
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

    /// M42 inc 10, the splice path's arm of the same rule: `add-item` into a present
    /// section whose id carries a **leading edge stopword** (`## In Scope`, id
    /// `in-scope`) appends under the existing heading rather than generating a
    /// duplicate home. `present_body_sections` recognizes a heading by
    /// **renormalizing** it (`slug::renormalize`), never by re-running the *mint*
    /// rule — `slugify("In Scope") == "scope"`, so a `slugify`-based compare reports
    /// the section absent and duplicates it.
    #[test]
    fn add_item_into_present_edge_stopword_section_does_not_duplicate_the_heading() {
        let yaml = b"\
type: brief
id-from: title
sections:
  - id: in-scope
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: note, slot: {} }
";
        let schema = crate::schema::load_schema(yaml).expect("schema loads");
        let src = "\
# Brief

## In Scope
";
        let out = add_item(&schema, src, "in-scope", "Read path", None, None, &[])
            .expect("item appends under the present edge-stopword section");
        assert_eq!(
            out.matches("## In Scope").count(),
            1,
            "the edge-stopword section heading must NOT be duplicated; got:\n{out}",
        );
        assert!(
            out.contains("### Read path  {#read-path}"),
            "the item is appended under the existing section; got:\n{out}",
        );
        let reparsed = instance_from_source(&schema, &out).expect("result re-parses");
        assert_eq!(
            render(&schema, &reparsed),
            out,
            "add_item under an edge-stopword section is byte-stable",
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
        let one = add_item(
            &schema,
            base,
            "criteria",
            "Rate limit holds",
            None,
            None,
            &[],
        )
        .expect("first item minted empty");
        let two = add_item(
            &schema,
            &one,
            "criteria",
            "Burst allowance",
            None,
            None,
            &[],
        )
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

    /// **The non-trailing insert-into-an-existing-group defect** (M42 Inc-5 T7 — found by the
    /// `AddedItemField` migration, live on the shipped `doc set-field` item-leaf path). Inserting
    /// a **second** bullet into an item that (a) already has a field group and (b) is **followed
    /// by another item** landed the bullet at the list block's end — which for a non-trailing
    /// group runs *past* the blank line separating it from the next `###` heading. The result
    /// (`- a\n\n- b\n### Next`) re-parses fine but **breaks `render(parse(x)) == x`**, the retired
    /// byte-stability invariant. Every prior fixture inserted into a *trailing* item or a
    /// mint-empty one (the re-render branch), so the shape was invisible — the T6 lesson again.
    #[test]
    fn insert_a_second_item_bullet_on_a_non_trailing_item_is_byte_stable() {
        let schema = crate::schema::load_schema_with_types(
            b"\
type: spec
id-from: title
sections:
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"x\" } }
        - { id: implemented-by, type: string }
        - { id: owner, type: string }
",
            &crate::schema::dev_pack_field_types(),
        )
        .expect("two-item-field spec schema loads");

        // Two items; the FIRST (non-trailing) already carries one bullet.
        let src = "\
# Auth flow

## Criteria

### Rate limit holds  {#rate-limit-holds}

The limiter holds under burst.

<!-- fields -->
- implemented-by: src/gateway.rs

### Burst allowance  {#burst-allowance}

Bursts are allowed briefly.

<!-- fields -->
- implemented-by: src/burst.rs
";
        let canonical = instance_from_source(&schema, src).expect("fixture parses");
        assert_eq!(render(&schema, &canonical), src, "the fixture is canonical");

        let out = set_item_field_or_insert(
            &schema,
            src,
            "criteria",
            "rate-limit-holds",
            "owner",
            "platform",
        )
        .expect("the absent second bullet is inserted");

        // The bullets stay CONTIGUOUS and the blank before the next item survives.
        assert!(
            out.contains("- implemented-by: src/gateway.rs\n- owner: platform\n\n### Burst"),
            "the group stays contiguous and the item separator is preserved; got:\n{out}"
        );
        // The invariant itself, on the emitted bytes.
        let reparsed = instance_from_source(&schema, &out).expect("result re-parses");
        assert_eq!(
            render(&schema, &reparsed),
            out,
            "byte-stable insert on a non-trailing item"
        );
        // The sibling item is untouched (no value bleed).
        assert!(out.contains("- implemented-by: src/burst.rs"));
        assert_eq!(out.matches("- owner: platform").count(), 1);
    }

    /// The **same defect at the simple-section locus** — the shared
    /// [`insert_after_line`] landing offset, exercised through [`insert_field`]: a first-bullet
    /// group in a **non-trailing** section (followed by a `##` heading) grows a second bullet
    /// contiguously and stays byte-stable. The one fix covers both callers; the sibling is swept,
    /// not assumed.
    #[test]
    fn insert_a_second_section_bullet_before_a_following_section_is_byte_stable() {
        let schema = crate::schema::load_schema_with_types(
            b"\
type: note
id-from: title
sections:
  - id: meta
    fields:
      - { id: owner, type: string }
      - { id: link, type: string, optional: true }
    slot: { hint: \"m\" }
  - id: tail
    slot: { hint: \"t\" }
",
            &crate::schema::dev_pack_field_types(),
        )
        .expect("note schema loads");
        let src = "\
# A note

## Meta

The meta prose.

<!-- fields -->
- owner: platform

## Tail

The tail prose.
";
        let canonical = instance_from_source(&schema, src).expect("fixture parses");
        assert_eq!(render(&schema, &canonical), src, "the fixture is canonical");

        let out = insert_field(
            &schema,
            src,
            "meta",
            &Field {
                key: "link".to_string(),
                value: Value::Scalar("docs/x.md".to_string()),
            },
        )
        .expect("the absent second bullet is inserted");

        assert!(
            out.contains("- owner: platform\n- link: docs/x.md\n\n## Tail"),
            "the group stays contiguous and the section separator is preserved; got:\n{out}"
        );
        let reparsed = instance_from_source(&schema, &out).expect("result re-parses");
        assert_eq!(
            render(&schema, &reparsed),
            out,
            "byte-stable insert into a non-trailing section's group"
        );
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
        let one = add_item(
            &schema,
            base,
            "criteria",
            "Rate limit holds",
            None,
            None,
            &[],
        )
        .expect("first mint");
        let two = add_item(
            &schema,
            &one,
            "criteria",
            "Burst allowance",
            None,
            None,
            &[],
        )
        .expect("second mint");

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

    /// The **nested** schema pair the locus-3 arms of [`insert_item_slot`] run against: a
    /// `releases` block whose items nest a `changes` block. The prior shape declares the one
    /// `notes` slot (so a committed nested item carries its prose **bare**), the current one
    /// adds an optional `impact` — the 1→2 arity, one locus down. `extra` joins the nested
    /// block in **both** shapes, so it is committed structure rather than part of the delta.
    fn nested_slot_pair(extra: &str) -> (crate::schema::Schema, crate::schema::Schema) {
        let yaml = |slots: &str| {
            format!(
                "\
type: log
id-from: title
sections:
  - id: releases
    repeatable:
      id-from: title
      block:
        - {{ id: title, type: string }}
        - id: changes
          repeatable:
            id-from: title
            block:
              - {{ id: title, type: string }}
{extra}{slots}"
            )
            .into_bytes()
        };
        let notes = "              - { id: notes, slot: { hint: \"The notes.\" } }\n";
        let impact =
            "              - { id: impact, slot: { optional: true, hint: \"The impact.\" } }\n";
        (
            crate::schema::load_schema(&yaml(notes)).expect("the prior nested schema loads"),
            crate::schema::load_schema(&yaml(&format!("{notes}{impact}")))
                .expect("the current nested schema loads"),
        )
    }

    /// **The depth swap, asserted directly (M50 Increment 7 / T4).** The reshape checks its
    /// own pre-image — it re-renders the committed item under the OLD template and refuses
    /// when the bytes differ, because a whole-item re-render would then destroy what the
    /// parse does not model. That render must happen at the item's **own nesting depth**: a
    /// nested item's heading is `####`, and rendering it at depth 1 emits `###`, so the
    /// compare fails on **every** conformant nested item and the whole corpus is refused
    /// with `UnmodelledContent` — a message that reads like a doc problem over bytes that
    /// are perfectly conformant.
    ///
    /// So the claim is stated as the absence of that refusal, not merely as a successful
    /// reshape: the item below models every byte of its region.
    #[test]
    fn a_conformant_nested_item_is_not_refused_as_unmodelled() {
        let (old, new) = nested_slot_pair("");
        let src = "\
# Log

## Releases

### 1.1.0  {#1-1-0}

#### Added  {#added}

- the group-count axis
";
        let out = insert_item_slot(
            &old,
            &new,
            src,
            "releases",
            &["changes".to_string()],
            "impact",
        );
        assert!(
            !matches!(out, Err(ItemSlotError::UnmodelledContent { .. })),
            "a conformant nested item models every byte of its region — the pre-image must \
             render at the item's own depth; got: {out:?}",
        );
        let out = out.expect("the nested reshape lands");
        assert_eq!(
            out,
            "\
# Log

## Releases

### 1.1.0  {#1-1-0}

#### Added  {#added}

##### Notes

- the group-count axis

##### Impact
",
            "the committed bare prose rides under `##### Notes` — the sub-labels render one \
             level deeper than the nested item, not one level deeper than a top-level one",
        );
    }

    /// **The refusal names the whole chain, because the anchor names two legal items.**
    /// Same-anchor nested items under different parents are legal — two releases may each
    /// carry a `#### Added` — so a cause that named only `added` would leave the operator
    /// with two candidate items and no way to tell which holds the stray bytes. The chain is
    /// the address a reader follows: item id, nested-section hop, nested item id.
    #[test]
    fn the_unmodelled_refusal_names_the_nested_item_by_its_chain() {
        let (old, new) =
            nested_slot_pair("              - { id: ticket, type: string, optional: true }\n");
        let src = "\
# Log

## Releases

### 1.1.0  {#1-1-0}

#### Added  {#added}

- the group-count axis

<!-- fields -->
- ticket: T-1

### 1.0.0  {#1-0-0}

#### Added  {#added}

- the trial-shaped fixture builder

<!-- fields -->
- ticket: T-2

THE COMMITTED PROSE THAT MUST SURVIVE.
";
        let err = insert_item_slot(
            &old,
            &new,
            src,
            "releases",
            &["changes".to_string()],
            "impact",
        )
        .expect_err("the unmodelled bytes are refused, never rewritten");
        assert_eq!(
            err,
            ItemSlotError::UnmodelledContent {
                section: "releases".to_string(),
                item: "1-0-0/changes/added".to_string(),
            },
            "the refusal locates the nested item by its whole chain, not by the anchor two \
             items share",
        );
    }

    /// **A nested item-field write keeps the committed bytes the parse does not model.**
    /// (M50 Increment 7 fix — the write seam of the `set_nested_item_field_or_insert`
    /// class.)
    ///
    /// jigc's canonical item order is slots-then-fields, so *"append a sentence to this
    /// entry"* lands **after** the `<!-- fields -->` group, where the parse carries nothing:
    /// it is unmodelled, not malformed — `parse_sections` is clean over it and `jigc
    /// validate` raises no finding. The **top-level** field write splices the value span or
    /// inserts the bullet after the present group, so those bytes cannot move; the nested
    /// dual re-rendered the item's whole region from what the parse modelled and deleted them
    /// at `Ok`. Both the update half (a bullet already present) and the insert half (an
    /// absent bullet) are driven, because they are two different splices.
    #[test]
    fn a_nested_item_field_write_keeps_committed_bytes_the_parse_does_not_model() {
        let (schema, _) = nested_slot_pair(
            "              - { id: ticket, type: string, optional: true }\n              - { id: severity, type: string, optional: true }\n",
        );
        let src = "\
# Log

## Releases

### 1.0.0  {#1-0-0}

#### Added  {#added}

- the group-count axis

<!-- fields -->
- ticket: T-2

An aside a human appended by hand.
";
        assert!(
            parse::parse_sections(&schema, src).is_ok(),
            "the aside parses — it is unmodelled, not malformed; got {:?}",
            parse::parse_sections(&schema, src).err(),
        );

        // ---- the update half: the bullet is present, only its value may move.
        let out = set_item_field_validated(
            &schema,
            src,
            "releases",
            &["1-0-0", "changes", "added"],
            "ticket",
            "T-9",
        )
        .expect("the nested field update lands");
        assert!(
            out.contains("An aside a human appended by hand."),
            "the hand-appended aside must survive a nested field update; got:\n{out}",
        );
        assert_eq!(
            out.replace("ticket: T-9", "ticket: T-2"),
            src,
            "the value bytes are the only bytes a present-bullet write may move",
        );

        // ---- the insert half: the bullet is absent and is generated into the group.
        let out = set_item_field_validated(
            &schema,
            src,
            "releases",
            &["1-0-0", "changes", "added"],
            "severity",
            "minor",
        )
        .expect("the nested field insert lands");
        assert!(
            out.contains("An aside a human appended by hand."),
            "the hand-appended aside must survive a nested field insert; got:\n{out}",
        );
        assert_eq!(
            out.replace("- severity: minor\n", ""),
            src,
            "the generated bullet is the only byte run a nested insert may add",
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
        let one = add_item(
            &schema,
            base,
            "criteria",
            "Rate limit holds",
            None,
            None,
            &[],
        )
        .expect("first mint");
        let out = add_item(
            &schema,
            &one,
            "criteria",
            "Burst allowance",
            None,
            None,
            &[],
        )
        .expect("second mint");

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

use crate::finding::{Finding, Location, Route};
use crate::schema::{Field as SchemaField, FieldType};

/// Build a blocking `write.*` reject [`Finding`], **routed** at its repair
/// ([`write_route`]) — the route floor (M43, `design/surface-contract.md` → The route
/// fence) widened over the write gate: a rejected write names its recovery. The write
/// verbs are the doc surface a driver reads with `--format json`, so a route-less
/// block here would be refused at the serialization seam anyway; this names the
/// recovery at the source.
fn blocking_write(code: &str, message: impl Into<String>, location: Location) -> Finding {
    Finding::graded(
        crate::finding::Severity::Blocking,
        code,
        message,
        Some(location),
        Some(write_route(code)),
    )
}

/// The per-code repair route of a write-time reject — one declared map, so every call
/// site minting a code routes it identically (the [`crate::validate::conformance_route`]
/// sibling). Shape questions route the mechanical `jigc doc schema <doctype>` read
/// (`<doctype>` is a declared placeholder of the CLI-seam dummy table); payload defects
/// route the human fix-and-re-run direction — the write persisted nothing, so re-running
/// the same verb with a corrected payload is the recovery *for a payload defect*. Where a
/// **broken staged source** can be the cause instead, re-running is a dead end and the
/// route names the escape hatch as well ([`discard_span`]).
fn write_route(code: &str) -> Route {
    match code {
        "write.malformed-value" => Route::mechanical(
            ["jigc", "doc", "schema", "<doctype>"],
            " to see the field's declared type and members, then re-run the write with a \
             conformant value",
        ),
        "write.unknown-field" | "write.unknown-section" | "write.wrong-shape" => Route::mechanical(
            ["jigc", "doc", "schema", "<doctype>"],
            " to see the declared shape, then re-run the write at a declared address",
        ),
        // A not-present is an **item-id** miss (the addressed item was never minted), not a
        // shape question: the schema names the shape, never the corpus's real item ids. The
        // followable recovery is to *show the containing section* and read its live item ids
        // — so the CLI seam ([`crate::doc`] `enrich_not_present_route`) overrides this at the
        // write-verb dispatch, where the doc's `<type>:<slug>#<section>` and the resolved
        // task id are in hand. This is the defensive fallback for any un-enriched producer:
        // the `<address>` / `<task-id>` placeholders are declared in the CLI-seam dummy table
        // (`design/validation.md` → the `write.*` route split; `surface-contract.md` law 2).
        "write.not-present" => Route::mechanical(
            ["jigc", "doc", "show", "<address>", "--task", "<task-id>"],
            " to see the section's current item ids, then re-run the write at an existing item",
        ),
        "write.already-present" => Route::human(
            "the target already exists — edit it in place (`set-field`/`set-slot`) instead \
             of re-creating it",
        ),
        "write.unslugable-title" => Route::human(
            "re-run the write with a title carrying at least one word character (the item \
             id is slugged from it)",
        ),
        "write.list-overwrite" => Route::human(
            "re-run `jigc doc set-field` with the inline-list form shown in the message, \
             carrying every value to keep",
        ),
        // The two **dead-end** rejects (M45): "re-run the same write" is the recovery only
        // when the *payload* is the broken thing — when the **staged source** is, no payload
        // revision can succeed and the agent is left circling (the rc.8 trial's costliest
        // finding). Both name the escape hatch too, keeping re-run as the first branch where
        // it genuinely applies. The exit is the shipped whole-task discard (no per-doc
        // discard verb exists — the M43 ghost-verb repair), and the engine holds no task id
        // at this seam, so the span carries the declared `<task-id>` placeholder the agent
        // fills — the [`crate::file_state`] `reconciliation.conflict-block` shape, and it
        // rides the checked [`Route::mechanical`] constructor so a verb that does not parse
        // cannot be taught here (`surface-contract.md` → The route fence).
        "write.non-reparseable" => Route::human(format!(
            "nothing was persisted — revise the payload so the result still conforms (the \
             message names the break), then re-run the same write; if the staged source \
             itself is what no longer parses, {} and start the task over",
            discard_span(),
        )),
        "write.target-escape" => Route::human(format!(
            "nothing was persisted — re-run the write with a revised payload; the same \
             payload over the same staged source escapes the same way, so if it recurs, \
             {} and start the task over (a recurring escape is a write-path defect to \
             report)",
            discard_span(),
        )),
        // The freed depth is **per-address** (M45 — a plain item frees `####`, a
        // multi-slot or nested-bearing one only `#####`), and the message names the
        // one that applies; a route naming a depth of its own would contradict it at
        // every address the global constant is wrong for.
        "write.slot-heading-depth" => Route::human(
            "demote the heading to the depth the message names (or deeper), or rephrase it \
             as plain prose, then re-run the same write",
        ),
        "write.slot-setext-heading" => Route::human(
            "rewrite the Setext heading as an ATX heading at the depth the message names, or \
             as plain prose, then re-run the same write",
        ),
        // The item-field value reject deliberately emits finalize's conformance code
        // (`generate_error_finding` → Two check times), so it carries that code's route.
        "schema-conformance.field-value-conformant" => crate::validate::conformance_route(code),
        other => panic!(
            "blocking_write mints `{other}` with no declared route — add it to write_route \
             (the route floor, design/surface-contract.md → The route fence)"
        ),
    }
}

/// The `` `jigc task discard <task-id>` `` span the two dead-end write rejects embed in
/// their prose. Built through the **checked** [`Route::mechanical`] constructor so the
/// CLI-seam parse fence adjudicates the argv (`design/surface-contract.md` → The route
/// fence) — a verb that does not parse cannot be taught here — and rendered to its text
/// so the route can put the re-run branch first. `<task-id>` is a declared placeholder of
/// the fence's dummy-substitution table; the agent fills it with the task it is in.
fn discard_span() -> String {
    Route::mechanical(["jigc", "task", "discard", "<task-id>", "--force"], "")
        .as_str()
        .to_owned()
}

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

/// The writable-scalar floor every scalar value must clear regardless of type:
/// non-empty and single-line (no control char — a newline/tab/… would inject a second
/// field line when spliced onto its `- key: value` line). Shared by the opaque floor
/// and by `check_ref` (which strips the bracket-list wrapper *after* this floor but
/// declines the opaque floor's non-ref bracket reject — a `ref` is exactly the type
/// `[…]` is a valid form for).
fn check_writable_scalar(field: &SchemaField, value: &str) -> Result<(), String> {
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

/// The opaque-scalar floor shared by `string` / `owned-location` (and any pack type
/// lacking a shape check): the writable-scalar floor, **plus** the N2 non-ref bracket
/// reject. A `[…]`-wrapped value is the inline-list / `[]` ref-clear idiom, valid only
/// on a `ref` field (which recognizes and strips the wrapper before reaching here). On
/// a non-ref scalar it is the ref-clear footgun misapplied — splicing it verbatim would
/// write literal `[]` garbage — so it is rejected, routed to the real clear verb.
/// Deeper adjudication is a `finalize` / pack concern.
fn check_opaque_scalar(field: &SchemaField, value: &str) -> Result<(), String> {
    check_writable_scalar(field, value)?;
    // N2: a `[…]` wrapper on a non-ref scalar is not a clear (only a list-cardinality
    // ref clears via `[]`). Route to the verb that actually clears a field.
    if value.starts_with('[') && value.ends_with(']') {
        return Err(format!(
            "{:?} is not a list-cardinality ref, so a `[…]` value is not a clear; to \
             clear an optional field, use `jigc doc set-field <addr> --unset`",
            field.id
        ));
    }
    Ok(())
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
    // Floor first: non-empty + single-line (control chars rejected). The *writable*
    // floor, not the opaque floor — a `ref` is exactly the type the `[…]` bracket form
    // is valid for, so it declines the opaque floor's N2 non-ref bracket reject and
    // recognizes the wrapper below instead (the in-task reorder N2 depends on).
    check_writable_scalar(field, value)?;
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
    Some(blocking_write(
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
        return Err(blocking_write(
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
        return Err(blocking_write(
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
        blocking_write(
            "write.unknown-field",
            format!("no field {field_key:?} declared in section {section_id:?}"),
            Location::at(1, 1),
        )
    })?;
    if let Err(why) = check_value(field, new_value) {
        return Err(blocking_write(
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
                return Err(blocking_write(
                    "write.non-reparseable",
                    format!("write rejected: result does not re-parse ({detail})"),
                    Location::at(1, 1),
                ));
            }
            Ok(edited)
        }
    }
}

/// What a gated `--unset` did — the two conformant ends of a removal, kept apart so the
/// ack can state which one it reached (`design/surface-contract.md` law 1: an ack that
/// says "unset" distinguishes *removed* from *was never there*, exactly as `create`'s
/// `existed` distinguishes minted from copied-in).
///
/// `AlreadyAbsent` carries **no buffer** on purpose: a no-op has nothing to persist, and
/// handing back a copy of `source` would let a caller write bytes for a write that did
/// not happen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsetOutcome {
    /// The field's line/bullet was removed; the buffer to persist.
    Removed(String),
    /// The field was **already absent** — the state the caller asked for already held,
    /// so nothing was edited (M49 Increment 1 T4).
    AlreadyAbsent,
}

/// Is the addressed field **already absent** from `source`? The presence question
/// [`item_chain_absent`] asks of an item, asked of a leaf — and the whole of `--unset`'s
/// no-op rule, at every container depth: an empty `item_ids` addresses the section's own
/// field group, a non-empty one the (possibly nested) item's.
///
/// A **missing container** answers the section arm's question `true`: the field is absent
/// because its optional section is, and "is this field present?" is the only question
/// `--unset` asks. The item arm answers `false` instead, because an absent *item* is a
/// real miss that [`unset_item_field_validated`] has already adjudicated a rank above —
/// answering `true` here would swallow it into a silent no-op.
///
/// A source that no longer parses answers `false`: "cannot tell" is not a yes, so the
/// splice path below keeps its `NotConformant` diagnosis.
fn addressed_field_absent(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field_key: &str,
) -> bool {
    let Ok(doc) = parse::parse_sections(schema, source) else {
        return false;
    };
    let fields = if item_ids.is_empty() {
        match doc.sections.iter().find(|s| s.id == section_id) {
            Some(section) => &section.fields,
            None => return true,
        }
    } else {
        match nested_parsed_item(schema, &doc, section_id, item_ids) {
            Some(item) => &item.fields,
            None => return false,
        }
    };
    !fields.iter().any(|f| f.key == field_key)
}

/// The gated `set-field --unset` (header / simple-body field): apply the eligibility
/// guard, answer the presence question, then the byte-stable [`unset_field`]
/// splice-remove and a re-parse of the result. Returns the [`UnsetOutcome`], or a
/// blocking [`Finding`] (and **no** buffer) — an ineligible field, an unknown field, or
/// a non-reparseable result.
///
/// **An eligible field that is already absent is a no-op, not a miss** (M49 Increment 1
/// T4): the state the caller asked for already holds. Before, it fell through to the
/// splice's `NotPresent` and the CLI's item-miss enrichment rewrote its route to *"show
/// the containing section to see its current item ids"* — a route that, followed
/// verbatim, showed a section that was fine and left the write reproducing its own error
/// (`design/surface-contract.md` → nothing dead-ends).
///
/// The determinism boundary is unmoved: the agent decides *which* optional field to
/// clear; the CLI/engine owns the byte removal.
pub fn unset_field_validated(
    schema: &Schema,
    source: &str,
    section_id: &str,
    field_key: &str,
) -> Result<UnsetOutcome, Finding> {
    let field = field_schema(schema, section_id, field_key).ok_or_else(|| {
        blocking_write(
            "write.unknown-field",
            format!("no field {field_key:?} declared in section {section_id:?}"),
            Location::at(1, 1),
        )
    })?;
    if let Some(finding) = unset_eligibility_finding(field) {
        return Err(finding);
    }
    if addressed_field_absent(schema, source, section_id, &[], field_key) {
        return Ok(UnsetOutcome::AlreadyAbsent);
    }
    let edited =
        unset_field(schema, source, section_id, field_key).map_err(|e| splice_error_finding(&e))?;
    reparse_or_reject(schema, &edited)?;
    Ok(UnsetOutcome::Removed(edited))
}

/// Does the addressed item chain name an item that is **not present** in `source`? The
/// shared presence question every write door asks *before* it adjudicates a leaf, a
/// title, or a field declaration — the `shape → presence → leaf` order (M47 — the
/// write-verb × miss-shape axis).
///
/// Three answers are deliberately **not** "absent":
/// - a **shape** miss (an undeclared or non-repeatable section, an unknown nested-section
///   hop — [`physical_item_chain`] `None`): shape outranks presence, and it is named a rank
///   above — the *undeclared* section by each door's opening [`section_undeclared`] check
///   (`write.unknown-section`), the declared-but-wrong shape by the splice path below
///   (`write.wrong-shape`);
/// - a **source that no longer parses**: "cannot tell" is not a yes, so today's
///   parse-break diagnosis is left standing;
/// - a chain that **resolves** to a live item.
///
/// Public because the presence question also has to outrank two **CLI-side, schema-only**
/// pre-checks that never touch the source — `write.id-from-field` and
/// `write.identity-change` (`crates/cli/src/doc.rs`) — which otherwise assert properties
/// of an item that does not exist and hand back a route whose first verb blocks.
pub fn item_chain_absent(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
) -> bool {
    physical_item_chain(schema, section_id, item_ids).is_some()
        && parse::parse_sections(schema, source)
            .is_ok_and(|doc| nested_parsed_item(schema, &doc, section_id, item_ids).is_none())
}

/// The gated `set-field --unset` (repeatable item field): the item-addressed sibling of
/// [`unset_field_validated`] — same eligibility guard + re-parse, over [`unset_item_field`].
/// `item_ids` is the parent-scoped id chain (single-level or nested).
///
/// **Item presence is adjudicated before field declaration** (M47 — the write-verb ×
/// miss-shape axis): an undeclared field *on an item that does not exist* is an item-id
/// miss, not a field question, and answering it with `write.unknown-field` sends the agent
/// to the schema for a defect the schema cannot show. The item miss outranks. **Shape
/// outranks both**: the chain is resolved against the schema first ([`physical_item_chain`]),
/// so an undeclared or non-repeatable section keeps today's shape-question diagnosis. A
/// source that no longer parses is left to the splice path below, which diagnoses the break.
///
/// **And the leaf's own presence is adjudicated last** (M49 Increment 1 T4): on a present
/// item, an eligible field with no bullet is [`UnsetOutcome::AlreadyAbsent`] — a no-op —
/// never the `NotPresent` whose CLI-side route told the agent to go read the section's
/// item ids for an item that was right there. The rank order matters both ways: the miss
/// still outranks, so an absent item never degrades into a silent no-op.
pub fn unset_item_field_validated(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field_key: &str,
) -> Result<UnsetOutcome, Finding> {
    // Rank 1 — the undeclared section outranks both presence and the field question
    // ([`undeclared_section_generate`]): `item_chain_absent` answers "not absent" for a
    // shape miss, so without this the door fell through to `write.unknown-field` and
    // asserted a field question about an item in a section that does not exist.
    if let Some(err) = undeclared_section_generate(schema, section_id) {
        return Err(generate_error_finding(&err));
    }
    if let Some(err) = undeclared_nested_section_generate(schema, section_id, item_ids) {
        return Err(generate_error_finding(&err));
    }
    if item_chain_absent(schema, source, section_id, item_ids) {
        return Err(generate_error_finding(&GenerateError::NotPresent {
            what: format!(
                "item {:?} in section {section_id:?} not present",
                hop_path(item_ids)
            ),
        }));
    }
    let field = item_field_schema(schema, section_id, item_ids, field_key).ok_or_else(|| {
        blocking_write(
            "write.unknown-field",
            format!(
                "no field {field_key:?} declared on item {:?} in section {section_id:?}",
                hop_path(item_ids)
            ),
            Location::at(1, 1),
        )
    })?;
    if let Some(finding) = unset_eligibility_finding(field) {
        return Err(finding);
    }
    // Rank 4 — the leaf's own presence. The item is present (rank 2) and the field is
    // declared and eligible (rank 3), so an absent bullet is the requested end state
    // rather than a miss: the no-op ([`addressed_field_absent`]).
    if addressed_field_absent(schema, source, section_id, item_ids, field_key) {
        return Ok(UnsetOutcome::AlreadyAbsent);
    }
    let edited = unset_item_field(schema, source, section_id, item_ids, field_key)
        .map_err(|e| splice_error_finding(&e))?;
    reparse_or_reject(schema, &edited)?;
    Ok(UnsetOutcome::Removed(edited))
}

/// The `--unset` eligibility guard: a field may be cleared only when its **absence is a
/// conformant state** — i.e. it is neither author-required, nor defaulted, nor
/// `set:`-derived. Returns a routed blocking [`Finding`] for an ineligible field, `None`
/// when the unset is allowed.
///
/// The roadmap shorthand "author-required / set-stamped" is insufficient: `status` is
/// **not** author-required ([`crate::validate::is_author_required`] returns `false` for a
/// defaulted field), yet clearing it drops a value the schema guarantees present — so the
/// guard keys on `default.is_some()` too. The **`set:`** arm covers the engine-injected
/// `schema-version` stamp (unsetting it would corrupt the freeze gate) and the
/// `set: on-create` date deriver alike.
fn unset_eligibility_finding(field: &SchemaField) -> Option<Finding> {
    if let Some(set) = field.set.as_deref() {
        let why = if set == crate::schema::SCHEMA_VERSION_SET {
            "the engine-injected schema-version stamp (unsetting it would corrupt the freeze gate)"
                .to_string()
        } else {
            format!("CLI-derived (`set: {set}`)")
        };
        return Some(Finding::block(
            "write.unset-ineligible",
            format!(
                "write rejected: field {:?} is {why} and cannot be unset",
                field.id
            ),
            "leave it in place — the CLI owns its value".to_string(),
        ));
    }
    if crate::validate::is_author_required(field) || field.default.is_some() {
        return Some(Finding::block(
            "write.unset-ineligible",
            format!(
                "write rejected: field {:?} is required (or defaulted) and cannot be unset",
                field.id
            ),
            "to change it, set a new value with `jigc doc set-field <addr> --value <value>`"
                .to_string(),
        ));
    }
    None
}

/// Re-parse `edited` against `schema`, mapping a parse failure to the blocking
/// `write.non-reparseable` finding — the (a) half of [`validate_after`], shared by the
/// generation paths whose edit is not a single located span (the `--unset` splice-remove,
/// the absent-field generate arms).
fn reparse_or_reject(schema: &Schema, edited: &str) -> Result<(), Finding> {
    if let Err(findings) = parse::parse_sections(schema, edited) {
        let detail = findings
            .into_iter()
            .next()
            .map(|f| f.message)
            .unwrap_or_else(|| "the edited buffer no longer conforms".to_string());
        return Err(blocking_write(
            "write.non-reparseable",
            format!("write rejected: result does not re-parse ({detail})"),
            Location::at(1, 1),
        ));
    }
    Ok(())
}

/// **Where a slot write lands** — the address [`set_slot_validated`] gates. The two
/// arms are the two structural homes a slot has: a section's own slot, or one leaf
/// slot on a (possibly nested) repeatable item, addressed by its section-qualified id
/// chain. One address type because there is **one** gated entry point: both the
/// reserved-depth ceiling and the validate-after confinement target are derived from
/// the address, so no slot write can reach bytes down a path that skipped them
/// (`implementation/parsing.md` → Slot heading-depth ceiling, enforcement site 1 —
/// before M45 the item arms were separate ungated verbs).
#[derive(Clone, Copy, Debug)]
pub enum SlotAddress<'a> {
    /// The section's own slot — `#<section>`.
    Section { section: &'a str },
    /// A leaf slot on a repeatable item — `#<section>/<item>[/<nested>/<item>…]/<leaf>`.
    /// `chain` is the **section-qualified** item chain ([`physical_item_chain`]'s
    /// input grammar); `leaf` names the slot inside the item's block (the bare `slot`
    /// for a single-slot item, a named `slots` entry for a multi-slot one).
    Item {
        section: &'a str,
        chain: &'a [&'a str],
        leaf: &'a str,
    },
}

impl<'a> SlotAddress<'a> {
    /// The `(section_id, item-chain)` pair the ceiling derivation keys on — the
    /// section arm's chain is empty, which is [`slot_ceiling`]'s section-slot arm.
    fn ceiling_key(&self) -> (&'a str, &'a [&'a str]) {
        match *self {
            SlotAddress::Section { section } => (section, &[]),
            SlotAddress::Item { section, chain, .. } => (section, chain),
        }
    }
}

/// The gated `set-slot`: the full write-time local adjudication for a slot prose
/// write — the slot sibling to [`set_field_validated`], and **the one seam every slot
/// write path passes through** (section slots and item slots, at every nesting depth;
/// the raw splices are private to this module so they cannot be reached ungated).
///
/// Enforces the **slot heading-depth ceiling** on `new_prose` *before* touching bytes,
/// with the reserved set derived from the address ([`slot_ceiling`]) rather than from
/// global constants: an ATX heading at or shallower than *this address's* reserved
/// depth, or any Setext heading, is rejected with a located
/// `write.slot-heading-depth` / `write.slot-setext-heading` finding naming the
/// offending line **and the shallowest depth that is free here**. Then it splices —
/// the present section's slot surgically, the addressed item by re-render
/// ([`set_item_slot`] / [`set_nested_item_slot`]) — or **generates** the section's
/// structural home when the section is absent, and runs [`validate_after`] (re-parse +
/// only the intended target changed). Returns the new buffer to persist, or a blocking
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
    addr: SlotAddress<'_>,
    new_prose: &str,
) -> Result<String, Finding> {
    // The heading-depth ceiling **for this address** — scanned on the standalone prose
    // so the located line is relative to the agent's content (the first violation is
    // the surfaced block). An address that does not resolve against the schema derives
    // no ceiling and reaches no bytes: the splice below rejects it as not-present.
    if let Some(ceiling) = {
        let (section_id, chain) = addr.ceiling_key();
        slot_ceiling(schema, section_id, chain)
    } && let Some(finding) = slot_ceiling_finding(new_prose, ceiling)
    {
        return Err(finding);
    }

    match addr {
        SlotAddress::Section { section } => set_section_slot(schema, source, section, new_prose),
        SlotAddress::Item {
            section,
            chain,
            leaf,
        } => set_gated_item_slot(schema, source, section, chain, leaf, new_prose),
    }
}

/// The **item-slot** arm of [`set_slot_validated`]: re-render-and-splice the addressed
/// item, then [`validate_after`] confined to the **item region**.
///
/// Clause (b)'s target is the whole located item block — the bytes the item paths
/// re-render ([`locate_item_path`]) — never the addressed leaf's span: the item paths
/// deliberately do not splice the bare leaf (that is not byte-stable on a mint-empty
/// item, `parsing.md` → The write pipeline), so a leaf-narrowed target would trip
/// `write.target-escape` on every item-slot write — the same shape as the section
/// arm's section-body-region target (the M13 audit HIGH).
fn set_gated_item_slot(
    schema: &Schema,
    source: &str,
    section_id: &str,
    chain: &[&str],
    leaf_id: &str,
    new_prose: &str,
) -> Result<String, Finding> {
    let edited = match chain {
        // A single-hop chain is a top-level item; deeper chains alternate item /
        // nested-section ids and take the depth-aware path.
        [item] => set_item_slot(schema, source, section_id, item, leaf_id, new_prose),
        _ => set_nested_item_slot(schema, source, section_id, chain, leaf_id, new_prose),
    }
    .map_err(|e| splice_error_finding(&e))?;
    let target = locate_item_region(schema, source, section_id, chain).ok_or_else(|| {
        blocking_write(
            "write.not-present",
            format!(
                "item {:?} in section {section_id:?} is not present",
                hop_path(chain)
            ),
            Location::at(1, 1),
        )
    })?;
    validate_after(schema, source, &edited, target)?;
    Ok(edited)
}

/// The **item region** of a section-qualified item chain — the whole located block the
/// item write paths re-render, and so the validate-after confinement target for an
/// item-slot write. `None` when the chain does not resolve (schema-side or in bytes).
fn locate_item_region(
    schema: &Schema,
    source: &str,
    section_id: &str,
    chain: &[&str],
) -> Option<Range<usize>> {
    let physical = physical_item_chain(schema, section_id, chain)?;
    locate_item_path(schema, source, section_id, &physical)
}

/// The gated **item** `set-field` — the item-addressed sibling of [`set_field_validated`],
/// and the seam every CLI item-field write passes through (top-level and nested, at every
/// container depth). It runs the splice-or-generate primitive and then [`validate_after`]
/// with the confinement target **derived from the pre-write source**, so the check asks
/// where the write was *supposed* to land rather than accepting wherever it did.
///
/// **Why the item-field paths needed their own domain entry (M49 Increment 1, T5).** M45
/// unified the item **slot** paths onto `validate_after`; the item **field** paths were
/// never in it — they were called raw, with no re-parse and no confinement clause. T1 then
/// widened an item's own leaf region to span its `#### <Leaf-Title>` prose *and* its
/// trailing field group, and [`set_item_field`] locates its bullet by a **line scan** over
/// that region: a slot-prose line that merely looks like a field bullet (`- status: …`,
/// ordinary Markdown the determinism boundary puts on the agent's side) is matched first
/// and the value spliced into the prose — acked at exit 0, with the pinned
/// `doc show --format json` then returning the stale field-group value the ack had just
/// contradicted. `dev-workflow.md` → *widen a guard's trigger, re-derive its response*.
///
/// **Clause (b)'s target, re-derived per path** ([`item_field_write_target`]) — the
/// per-path rule `parsing.md`'s write-pipeline table already draws, applied to the field
/// row it had never reached:
///
/// * a **present** bullet is a *surgical splice*, so the target is the item's own
///   **field-group list** — the bytes a bullet edit belongs to. A splice into slot prose
///   lands outside it and is rejected `write.target-escape`, nothing persisted;
/// * an **absent** bullet with a field group present appends at that list's end — the same
///   target, and the insert is confined by the same check;
/// * an **absent** bullet with **no** field group yet re-renders the whole item
///   ([`insert_item_field`]'s cold-fill arm), so the target is the **item region** — the
///   shape `parsing.md` warns must never be leaf-narrowed;
/// * the **nested** path asks the identical question — it takes the same splice-or-insert
///   as the flat one, so it takes the same target.
///
/// Clause (a) — the re-parse — rides every arm, and it is load-bearing only because T3
/// landed first: a duplicate declared bullet is now a parse-level
/// `conformance.duplicate-field`, so the T1 corruption shape can no longer re-parse clean.
///
/// **The gate is structural for the CLI, a convention inside the engine.** All four
/// item-field write primitives — [`set_item_field_or_insert`],
/// [`set_nested_item_field_or_insert`] and the [`set_item_field`] / [`insert_item_field`]
/// halves they dispatch to — are `pub(crate)`, so no CLI caller can reach one ungated (the
/// item-**slot** precedent, where the raw splices went private for the same reason). Three
/// engine-internal callers still reach them directly and are named rather than glossed:
/// [`crate::transform`]'s corpus-migration splices, under its own whole-document fidelity
/// adjudication, and [`crate::milestone`]'s record writes.
pub fn set_item_field_validated(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field_key: &str,
    new_value: &str,
) -> Result<String, Finding> {
    // Derived BEFORE the write: after it, the bytes the target is meant to bound may have
    // moved, and a target read off the edited buffer would confirm whatever happened.
    let target = item_field_write_target(schema, source, section_id, item_ids);
    let edited = match item_ids {
        // A single-hop chain is a top-level item; deeper chains alternate item /
        // nested-section ids and take the depth-aware path (the [`set_gated_item_slot`]
        // dispatch, so the field and slot seams cannot disagree about what "nested" means).
        [item] => set_item_field_or_insert(schema, source, section_id, item, field_key, new_value),
        _ => set_nested_item_field_or_insert(
            schema, source, section_id, item_ids, field_key, new_value,
        ),
    }
    .map_err(|e| generate_error_finding(&e))?;
    match target {
        Some(target) => validate_after(schema, source, &edited, target)?,
        // The address did not resolve against the schema or the bytes. The primitive
        // above adjudicates that with its own diagnosis (an unknown section, a wrong
        // shape, an absent item), so this arm is only reached if it somehow succeeded
        // anyway — keep the (a) half rather than skipping the gate entirely.
        None => reparse_or_reject(schema, &edited)?,
    }
    Ok(edited)
}

/// The validate-after confinement target of one item-**field** write, derived from the
/// pre-write `source` — the per-path rule documented on [`set_item_field_validated`].
/// `None` when the addressed item does not resolve (schema-side or in bytes).
fn item_field_write_target(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
) -> Option<Range<usize>> {
    let region = locate_item_region(schema, source, section_id, item_ids)?;
    // **One rule at every depth** (M50 Increment 7 fix). A write edits a bullet **inside
    // the item's own field group** — the same group [`insert_item_field`] scans for, over
    // the same schema-keyed leaf region, so the guard and the writer cannot disagree about
    // which group is the item's own. With no group yet, the write is the cold-fill
    // re-render and the region is the target. The nested path used to be carved out here
    // because it re-rendered the addressed item whole; it no longer does — it takes the
    // flat dual's splice-or-insert — so the carve-out is gone with the re-render, and the
    // confinement is the tighter one at both loci.
    let blocks = parse::scan_blocks(source);
    let leaf = item_own_leaf_region(
        schema,
        source,
        &blocks,
        section_id,
        item_ids,
        region.clone(),
    );
    match field_group_list(&blocks, leaf) {
        Some((list, _)) => Some(list),
        None => Some(region),
    }
}

/// The **section-slot** arm of [`set_slot_validated`] — splice the present section's
/// slot (validate-after confined to the section body region) or generate the section's
/// structural home when it is absent (re-parse only; generation is not a single-span
/// splice).
fn set_section_slot(
    schema: &Schema,
    source: &str,
    section_id: &str,
    new_prose: &str,
) -> Result<String, Finding> {
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
                blocking_write(
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
                return Err(blocking_write(
                    "write.non-reparseable",
                    format!("write rejected: result does not re-parse ({detail})"),
                    Location::at(1, 1),
                ));
            }
            Ok(edited)
        }
        // Anything else — a broken buffer, an undeclared section — renders through the
        // one splice→finding mapping, which carries the parse break when there is one.
        Err(e) => Err(splice_error_finding(&e)),
    }
}

/// The heading-depth ceiling governing **one slot address**: the deepest ATX level
/// the CLI reserves for structure there, and the shallowest level the slot's prose
/// may use. Depths are ATX level numbers (`2` = `##`), never rendered hashes — the
/// rendering is the statement's concern, not the rule's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotCeiling {
    /// The deepest reserved level: an ATX heading at or shallower than this is a
    /// CLI-owned structural marker (a section start, an item start, or a
    /// multi-slot sub-label), never slot prose.
    pub reserved_max: usize,
    /// The shallowest heading level slot prose may use — `reserved_max + 1`.
    pub first_allowed: usize,
}

impl SlotCeiling {
    /// The ceiling reserving through `reserved_max`, first-allowed one deeper.
    fn reserving(reserved_max: usize) -> Self {
        SlotCeiling {
            reserved_max,
            first_allowed: reserved_max + 1,
        }
    }
}

/// **The reserved-depth set, derived from `(Schema, section_id, item-chain)`** — the
/// single origin of both the enforced ceiling and the rendered statement, replacing
/// the context-blind global depths (`implementation/parsing.md` → Slot heading-depth
/// ceiling, the four-row table):
///
/// | slot context | reserved through | first allowed |
/// |---|---|---|
/// | a **section** slot | `3` (`##` section, `###` item) | `4` |
/// | a **plain item** (single-slot, no nested repeatable) at nesting depth `d` | `2+d` | `2+d+1` |
/// | a **multi-slot** item at depth `d` (its `#### <Leaf-Title>` sub-labels) | `2+d+1` | `2+d+2` |
/// | an item whose block carries a **nested repeatable**, at depth `d` | `2+d+1` | `2+d+2` |
///
/// The derivation keys on **`multi_slot || has_nested`**, never multi-slot alone: the
/// parser bounds a nested-bearing item's leaf region at the first heading **at `2+d+1`**
/// that is not one of the item's own declared slot sub-labels
/// ([`parse::opens_item_nested_region`], M49 T2), so an *authored* heading at that level
/// inside the item's prose reads as a nested item start exactly as a sub-label would —
/// and, symmetrically, `first_allowed` is a depth the boundary must let through, since
/// the ceiling is the sentence the write path prints to the author. No shipped doctype is single-slot-with-nested today — which is
/// precisely why the member is derived rather than enumerated.
///
/// `item_chain` is the **section-qualified** address chain ([`physical_item_chain`]):
/// it alternates item id / nested-section id, so the item ids only contribute depth
/// (their values are never matched against a document here — this is a schema-side
/// derivation). An empty chain is the section-slot arm, whose depths are fixed by the
/// grammar (sections are `##`, items `###`) and so need no section lookup. `None` when
/// the chain does not resolve against the schema — the section is absent or not
/// repeatable, or a nested-section segment names no declared nested repeatable.
pub fn slot_ceiling(schema: &Schema, section_id: &str, item_chain: &[&str]) -> Option<SlotCeiling> {
    if item_chain.is_empty() {
        // The section-slot arm: `##` is the section and `###` its items, both
        // CLI-owned wherever the section lives.
        return Some(SlotCeiling::reserving(item_heading_level(1)));
    }
    // The physical chain drops the logical nested-section hops (they render no
    // heading), so its length *is* the addressed item's nesting depth.
    let depth = physical_item_chain(schema, section_id, item_chain)?.len();
    let template = parse::ItemTemplate::from(chain_repeatable(schema, section_id, item_chain)?);
    let item_level = item_heading_level(depth);
    let reserved_max = if template.is_multi_slot() || template.has_nested() {
        // One level deeper is structure too — a `#### <Leaf-Title>` sub-label, or a
        // nested item's own heading.
        item_level + 1
    } else {
        item_level
    };
    Some(SlotCeiling::reserving(reserved_max))
}

/// Every slot address `schema` declares, each with the [`SlotCeiling`] the write
/// path will enforce there — the **fence input** the stated-at statement takes
/// (`design/surface-contract.md` → The stated-at fence, the M45 revision). The
/// label is the slot's dotted schema path (`milestones.proves`,
/// `releases.changes.notes`, or a bare section id for a section slot), in
/// declaration order.
///
/// The item-id segments are placeholders: [`slot_ceiling`] is a **schema-side**
/// derivation and never matches an item id against a document, so a synthetic
/// chain resolves the same ceiling the real address will. A context whose chain
/// does not resolve is skipped rather than guessed at (unreachable for a loaded
/// schema — the chain is built from the schema's own declarations).
pub fn schema_slot_ceilings(schema: &Schema) -> Vec<(String, SlotCeiling)> {
    /// The stand-in for an item id in a synthetic address chain.
    const ANY_ITEM: &str = "*";

    fn walk<'a>(
        schema: &'a Schema,
        section_id: &str,
        repeatable: &'a crate::schema::Repeatable,
        chain: &mut Vec<&'a str>,
        label: &str,
        out: &mut Vec<(String, SlotCeiling)>,
    ) {
        chain.push(ANY_ITEM);
        for leaf in &repeatable.block {
            match leaf {
                crate::schema::Leaf::Slot { id, .. } => {
                    if let Some(ceiling) = slot_ceiling(schema, section_id, chain) {
                        out.push((format!("{label}.{id}"), ceiling));
                    }
                }
                crate::schema::Leaf::Repeatable { id, repeatable } => {
                    // The nested-section hop: it renders no heading of its own,
                    // but it IS a chain segment the resolver walks by name.
                    let nested_label = format!("{label}.{id}");
                    chain.push(id.as_str());
                    walk(schema, section_id, repeatable, chain, &nested_label, out);
                    chain.pop();
                }
                crate::schema::Leaf::Field(_) => {}
            }
        }
        chain.pop();
    }

    let mut out = Vec::new();
    for section in &schema.sections {
        match &section.body {
            SectionBody::Simple { slot: Some(_), .. } => {
                if let Some(ceiling) = slot_ceiling(schema, &section.id, &[]) {
                    out.push((section.id.clone(), ceiling));
                }
            }
            SectionBody::Simple { .. } => {}
            SectionBody::Repeatable { repeatable } => {
                let mut chain = Vec::new();
                walk(
                    schema,
                    &section.id,
                    repeatable,
                    &mut chain,
                    &section.id,
                    &mut out,
                );
            }
        }
    }
    out
}

/// The ATX hashes for one depth (`3` → `` `###` ``), and the schema-reserved set
/// of a ceiling as the statement spells it (`` `##`/`###`/`####` ``) — the depth
/// vocabulary shared by the enforcing check ([`slot_ceiling_finding`]) and the
/// stated-at statement ([`slot_ceiling_statement`]), so the sentence a projection
/// renders and the rule the write path enforces cannot drift.
fn ceiling_reserved_set(ceiling: SlotCeiling) -> String {
    (2..=ceiling.reserved_max)
        .map(|level| format!("`{}`", "#".repeat(level)))
        .collect::<Vec<_>>()
        .join("/")
}

/// The **stated-at statement** of the slot heading-depth ceiling for the slot
/// addresses `contexts` names — the sentence the `{{schema:<doctype>}}` projection
/// renders beside the slot-prose skeleton, so the ceiling is stated where it binds,
/// before it can fail (law 3).
///
/// **Parameterized by address at M45** (`design/surface-contract.md` → The stated-at
/// fence, the M45 revision): the reserved set is a function of the target, so a
/// statement built from global constants proved *statement == constant* while the
/// constant was context-blind — and rendered "`####` is safe" into the workflow whose
/// primary writes are `roadmap` milestone slots, where `####` is the corrupting depth.
/// Feed [`schema_slot_ceilings`] — the same derivation the write path enforces with.
///
/// All contexts sharing one ceiling render the single-depth sentence; a schema that
/// mixes them (`changelog` — a depth-1 single-slot item beside a depth-2 nested one)
/// renders each group against the slots it governs. Empty ⇒ empty string: a slot-less
/// schema solicits no prose, so the statement stays inert.
pub fn slot_ceiling_statement(contexts: &[(String, SlotCeiling)]) -> String {
    // Group by ceiling, in first-appearance order — the projection's own
    // declaration order, so the sentence is deterministic.
    let mut groups: Vec<(SlotCeiling, Vec<&str>)> = Vec::new();
    for (label, ceiling) in contexts {
        match groups.iter_mut().find(|(c, _)| c == ceiling) {
            Some((_, labels)) => labels.push(label),
            None => groups.push((*ceiling, vec![label])),
        }
    }
    match groups.as_slice() {
        [] => String::new(),
        [(ceiling, _)] => format!(
            "Inside slot prose, headings must sit at `{}` depth or deeper — {} are \
             schema-reserved, and Setext headings are rejected.",
            "#".repeat(ceiling.first_allowed),
            ceiling_reserved_set(*ceiling),
        ),
        many => {
            let per_group = many
                .iter()
                .map(|(ceiling, labels)| {
                    format!(
                        "at `{}` or deeper in {}",
                        "#".repeat(ceiling.first_allowed),
                        labels
                            .iter()
                            .map(|l| format!("`{l}`"))
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "Inside slot prose, the reserved depths differ by slot — headings must \
                 sit {per_group}. Setext headings are rejected."
            )
        }
    }
}

/// The **address-independent** form of the ceiling rule, for the surfaces composed
/// before any address exists: `jigc doc set-slot --help` (clap renders help with no
/// target in hand) and the pack steps' hand-prose solicits. It states the rule and
/// names no depth — naming one would re-mint the very lie parameterization removed
/// (`design/surface-contract.md` → The stated-at fence, the M45 census table).
///
/// `pub` for the CLI's help text, which carries it verbatim.
pub fn slot_ceiling_rule_statement() -> &'static str {
    "Inside slot prose, the reserved heading depths are schema-relative to the address \
     you write — the CLI owns the section, item, and sub-label heading levels there, so \
     your headings sit below them. Setext headings are rejected at every depth; a \
     rejected write names the shallowest depth free at that address."
}

/// The first heading-depth-ceiling violation in standalone slot `prose` against
/// `ceiling`, if any — an ATX heading at or shallower than the address's reserved
/// depth, or a Setext underline, located by its **1-based line within the prose**.
/// Reuses the same block parse the read-time parser uses (never a line scanner — a
/// `## …` inside a fenced code block is correctly *not* a heading), so the write-time
/// and read-time ceiling agree: `parse::is_reserved_depth` tests `level <=
/// reserved_max`, and so does this. Emits a `write.*` finding (the write-path
/// envelope), distinct from the parser's `conformance.*` producer.
///
/// The message names the depth that is free **at this address** — derived from the
/// passed [`SlotCeiling`], never a global `####`, which is true for a plain depth-1
/// item and false for a multi-slot or nested-bearing one.
fn slot_ceiling_finding(prose: &str, ceiling: SlotCeiling) -> Option<Finding> {
    let allowed = "#".repeat(ceiling.first_allowed);
    parse::scan_blocks(prose).into_iter().find_map(|b| match b {
        Block::Heading { is_atx, line, .. } if !is_atx => Some(blocking_write(
            "write.slot-setext-heading",
            format!(
                "Setext heading in slot prose at line {line}; use `{allowed}` ATX depth or rephrase"
            ),
            Location::at(line, 1),
        )),
        Block::Heading { level, line, .. } if level_num_of(level) <= ceiling.reserved_max => {
            let depth = "#".repeat(level_num_of(level));
            Some(blocking_write(
                "write.slot-heading-depth",
                format!(
                    "heading at schema-reserved depth `{depth}` in slot prose at line {line}; \
                     `{allowed}` is the shallowest depth free at this address — use it or rephrase"
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
        GenerateError::UnknownSection { id, under } => (
            "write.unknown-section",
            match under {
                Some(top) => format!(
                    "write rejected: no nested section {id:?} declared under section {top:?} \
                     in the schema"
                ),
                None => format!("write rejected: no section {id:?} declared in the schema"),
            },
        ),
        GenerateError::WrongShape { what } => {
            ("write.wrong-shape", format!("write rejected: {what}"))
        }
        // The item-id miss emits the splice path's `write.not-present` (the same literal
        // [`splice_error_finding`] uses for [`SpliceError::NotPresent`]), so the same miss
        // carries the same code whichever half of the write path adjudicated it — and with
        // it the route that names the containing section rather than the schema.
        GenerateError::NotPresent { what } => {
            ("write.not-present", format!("write rejected: {what}"))
        }
        // The undeclared item-field leaf emits the `--unset` sibling's own code AND its
        // own sentence (`unset_item_field_validated`), so the same miss reads identically
        // whichever half of the write path adjudicated it — and routes identically too,
        // since [`write_route`] keys on the code.
        GenerateError::UnknownField { key, at } => (
            "write.unknown-field",
            format!("no field {key:?} declared on {at}"),
        ),
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
    blocking_write(code, message, Location::at(1, 1))
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

/// **Rank 1 of the write address adjudication order** — shape → item presence → leaf
/// (`design/write-commands.md` → Every write resolves its address before it moves bytes):
/// does the address name a section the schema **does not declare**?
///
/// Asked *first* by every item-addressing write door, because neither lower rank can
/// answer it honestly: an undeclared section holds no items whose presence could be
/// adjudicated, and no block on which a leaf could be declared. Answering it lower down is
/// what let one miss emit four different codes across the six item-addressing verbs —
/// `write.not-present` at [`set_item_slot`] / [`remove_item`] (whose enriched route is a
/// `jigc doc show <doc>#<undeclared-section>` that **exits 1**, a recovery that does not
/// answer), `write.wrong-shape` at [`retitle_item`] / [`add_nested_item`], and
/// `write.unknown-field` at [`unset_item_field_validated`], which asserted a field
/// question about an item in a section that does not exist (M47 — the write-verb ×
/// miss-shape axis, the undeclared-**section** column).
///
/// Deliberately narrower than "is this a shape miss": a **declared but non-repeatable**
/// section is a genuine declared-shape defect each door keeps diagnosing itself
/// (`write.wrong-shape`), and only the *undeclared* section is re-ranked here.
fn section_undeclared(schema: &Schema, section_id: &str) -> bool {
    !schema.sections.iter().any(|s| s.id == section_id)
}

/// [`section_undeclared`] as the **splice** path's reject — the ranked-first `?`-able form
/// the [`SpliceError`]-returning item doors open with.
///
/// `pub` since M49 Increment 11 / T3: the **section-level** address forms (`set-slot` at
/// `#<section>`, `set-field` at `#<section>/<leaf>`, and both of their `doc author` batch
/// arms) never reach one of those doors — the CLI's own target resolvers refuse first,
/// because a section that is not declared holds neither the slot nor the field they are
/// looking for. They ask the same question here rather than re-deriving it, so *undeclared
/// section* is one predicate and one reject sentence at every write verb, whichever side of
/// the seam adjudicates it (`design/validation.md` → The `write.*` route split).
pub fn undeclared_section_splice(schema: &Schema, section_id: &str) -> Option<SpliceError> {
    section_undeclared(schema, section_id).then(|| SpliceError::UndeclaredSection {
        section: section_id.to_string(),
        under: None,
    })
}

/// [`section_undeclared`] as the **generation** path's reject — the [`GenerateError`]
/// sibling of [`undeclared_section_splice`]. Both render `write.unknown-section`, whose
/// route is the `jigc doc schema <doctype>` read that genuinely answers the miss.
fn undeclared_section_generate(schema: &Schema, section_id: &str) -> Option<GenerateError> {
    section_undeclared(schema, section_id).then(|| GenerateError::UnknownSection {
        id: section_id.to_string(),
        under: None,
    })
}

/// [`section_undeclared`] **at every hop past the first** — the nested-section sibling,
/// and rank 1 at that depth for the same reason: an address whose *later* segment names a
/// nested section the schema never declared is wrong whatever the corpus holds, so no
/// lower rank can answer it honestly.
///
/// Returns **the first undeclared segment's own id** (`"bogus"`), which the reject names
/// together with the top-level section the address entered through
/// ([`SpliceError::UndeclaredSection::under`]). Until M51 it returned the assembled
/// address path down to that segment (`"releases/1-3-0/bogus"`) — M49's answer to the
/// objection that the id alone would say "no section `bogus`" about a schema that has no
/// top-level `bogus` either, since the agent's mistake is the hop and not the name. The
/// objection stands and the locus answers it; the path did not, because it was a *proper
/// prefix of the address the caller typed*, rendered in the exact shape of an address —
/// so the reader was handed a string that is in no argv and had to diff it against their
/// own to find the failing hop (M51 — EC-25; driven at the traversal form
/// `#milestones/../../../etc` → `no section "milestones/../.."`).
///
/// Answering it lower down is what let **one** miss emit four different codes across the
/// six item-addressing verbs (M49 — the nested arm of the undeclared-section column;
/// `design/command-output-contract.md` → Evolution posture, the M49 paragraph):
/// `write.not-present` at [`set_nested_item_slot`] / [`remove_nested_item`] (routing a
/// `jigc doc show` of the *top* section, which exits 0 listing item ids that are not what
/// the address got wrong), `write.wrong-shape` at [`set_nested_item_field_or_insert`] /
/// [`retitle_item`] / [`add_nested_item`], and `write.unknown-field` at
/// [`unset_item_field_validated`] — a field question about an item in a nested section
/// that does not exist.
///
/// **Deliberately as narrow as its top-level dual.** A segment the block declares as
/// something *other* than a nested repeatable (a field, a slot) is a genuine declared-shape
/// defect each door keeps diagnosing itself (`write.wrong-shape`), exactly as a declared
/// but non-repeatable **section** is; only the segment declared *nowhere* is re-ranked
/// here. And the chain alternates `item, nested-section, …` starting at an item, so a
/// trailing leaf name — `#releases/1-3-0/bogus` at `set-slot` / `set-field`, where `bogus`
/// is the leaf and never reaches this walk — stays the `write.unknown-field` it correctly
/// is.
fn nested_section_undeclared(
    schema: &Schema,
    section_id: &str,
    item_ids: &[&str],
) -> Option<String> {
    let section = schema.sections.iter().find(|s| s.id == section_id)?;
    let SectionBody::Repeatable { repeatable } = &section.body else {
        return None;
    };
    let mut current = repeatable;
    let mut expect_item = true;
    for segment in item_ids {
        if expect_item {
            expect_item = false;
            continue;
        }
        expect_item = true;
        let declared = current.block.iter().find(|leaf| match leaf {
            crate::schema::Leaf::Field(field) => field.id == *segment,
            crate::schema::Leaf::Slot { id, .. } | crate::schema::Leaf::Repeatable { id, .. } => {
                id == segment
            }
        });
        match declared {
            Some(crate::schema::Leaf::Repeatable { repeatable, .. }) => current = repeatable,
            // Declared, but not a nested repeatable: a shape question, not a declaredness
            // one — left to the door, as the non-repeatable **section** is.
            Some(_) => return None,
            None => return Some((*segment).to_string()),
        }
    }
    None
}

/// [`nested_section_undeclared`] as the **splice** path's reject — the nested sibling of
/// [`undeclared_section_splice`], and the same `write.unknown-section` it renders.
fn undeclared_nested_section_splice(
    schema: &Schema,
    section_id: &str,
    item_ids: &[&str],
) -> Option<SpliceError> {
    nested_section_undeclared(schema, section_id, item_ids).map(|section| {
        SpliceError::UndeclaredSection {
            section,
            under: Some(section_id.to_string()),
        }
    })
}

/// [`nested_section_undeclared`] as the **generation** path's reject — the nested sibling
/// of [`undeclared_section_generate`].
fn undeclared_nested_section_generate(
    schema: &Schema,
    section_id: &str,
    item_ids: &[&str],
) -> Option<GenerateError> {
    nested_section_undeclared(schema, section_id, item_ids).map(|id| {
        GenerateError::UnknownSection {
            id,
            under: Some(section_id.to_string()),
        }
    })
}

/// The **undeclared-address guard** the two insert-capable item-field writers consult
/// once [`item_field_schema`] has come back `None`: `Some(GenerateError::UnknownField)`
/// when the write must be rejected **before any bytes move**, `None` when the miss is
/// really something else and the splice path below is the better diagnostician.
///
/// Two ranks outrank the field question, so both fall through here:
/// - **Shape.** An undeclared or non-repeatable section ([`chain_repeatable`] `None`) is a
///   section-shaped miss named a rank above — the undeclared section by the door's own
///   [`undeclared_section_generate`] check (`write.unknown-section`), the
///   declared-but-not-repeatable one by the splice path (`write.wrong-shape`).
/// - **Item presence.** An undeclared field *on an item that does not exist* is an item-id
///   miss, not a field question — the [`unset_item_field_validated`] sibling's settled
///   ordering (M47 — the write-verb × miss-shape axis). Falling through lets the splice
///   path emit its own `write.not-present`, so the two doors agree without a second
///   construction site.
///
/// A source that no longer **parses** answers "is the item present?" with "cannot tell",
/// which is not a yes: it too falls through, leaving today's parse-break diagnosis intact
/// (and leaving the migration transform caller, which splices over intermediate buffers,
/// byte-identical).
fn undeclared_field_reject(
    schema: &Schema,
    source: &str,
    section_id: &str,
    item_ids: &[&str],
    field_key: &str,
) -> Option<GenerateError> {
    chain_repeatable(schema, section_id, item_ids)?;
    let doc = parse::parse_sections(schema, source).ok()?;
    nested_parsed_item(schema, &doc, section_id, item_ids)?;
    Some(GenerateError::UnknownField {
        key: field_key.to_string(),
        at: format!("item {:?} in section {section_id:?}", hop_path(item_ids)),
    })
}

/// The **slot** sibling of [`undeclared_field_reject`], consulted by the two item-slot
/// splices ([`set_item_slot`] / [`set_nested_item_slot`]) once the addressed item is in
/// hand and **before** [`crate::parse::ParsedItem::slot_span`] is asked for its bytes:
/// `Some(SpliceError::UnknownLeaf)` when the chain's own template declares no slot by
/// that id, `None` when the leaf is declared (or the chain is a shape miss the splice
/// path below names better).
///
/// This is where Settle Decision 9 puts the tightening — **at the write callers, not in
/// `slot_span`**, whose second consumer is conformance adjudication and needs the
/// single-slot fallback it applies. That fallback is exactly the hole: `slot_span`
/// ignores the leaf id whenever the item carries no sub-labelled `slots`, so **any** leaf
/// name resolved to the item's one real slot and the write landed there at exit 0, acking
/// the leaf it never wrote (`baseline.md` §3a, N4). Asking the *template* first also
/// re-diagnoses the slot-less block arm, which used to surface as a
/// [`SpliceError::NotPresent`] about a slot the schema never had.
fn undeclared_slot_reject(
    schema: &Schema,
    section_id: &str,
    item_ids: &[&str],
    leaf_id: &str,
) -> Option<SpliceError> {
    let template = chain_repeatable(schema, section_id, item_ids)?;
    let declared = template
        .block
        .iter()
        .any(|leaf| matches!(leaf, crate::schema::Leaf::Slot { id, .. } if id == leaf_id));
    (!declared).then(|| SpliceError::UnknownLeaf {
        leaf: leaf_id.to_string(),
        at: format!("item {:?} in section {section_id:?}", hop_path(item_ids)),
    })
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
    let repeatable = chain_repeatable(schema, section_id, item_chain)?;
    repeatable.block.iter().find_map(|leaf| match leaf {
        crate::schema::Leaf::Field(field) if field.id == field_key => Some(&**field),
        _ => None,
    })
}

/// Render a [`SpliceError`] as the gate's blocking [`Finding`].
///
/// The message is the error's own [`Display`](std::fmt::Display) sentence — so a
/// [`SpliceError::NotConformant`] names the parse break it carries — and a carried
/// break also **locates** the finding at the offending line, rather than the 1:1
/// fallback every address-less write reject otherwise takes.
///
/// **The carried break contributes its coordinate, never its address** (M49). A parse
/// finding's [`Location::address`] is a bare *fragment* — `header/bogus`,
/// `criteria/ghost/statement` — assembled outward by [`parse::prefix_hop`] and completed
/// into URI normal form only by the producer that holds the doc's identity
/// ([`crate::finding::readdress_to_uri`], whose stated precondition is exactly that its
/// input is raw per-instance parse output). This seam is **not** that producer: the write
/// path's target is the *write's own address*, stamped outward by the CLI, which holds it
/// ([command-output-contract.md](../../../design/command-output-contract.md) → the
/// `write.*` row). Cloning the whole location shipped the fragment as the finding's
/// `key.target` and, being non-null, it survived that stamp untouched — so a
/// `write.non-reparseable` over any break **inside a section** (every break `prefix_hop`
/// reaches, which is nearly all of them) emitted `target: "header/bogus"`: a key no read
/// verb resolves, and a different target shape from the same code's doc-level branch.
/// Keeping `line`/`col` keeps the whole reason the carry exists.
pub fn splice_error_finding(err: &SpliceError) -> Finding {
    let (code, location) = match err {
        SpliceError::NotPresent { .. } => ("write.not-present", None),
        SpliceError::NotConformant { findings } => (
            "write.non-reparseable",
            findings
                .first()
                .and_then(|f| f.location.as_ref())
                .map(|l| Location::at(l.line, l.col)),
        ),
        SpliceError::UndeclaredSection { .. } => ("write.unknown-section", None),
        // The undeclared **slot** leaf carries the `GenerateError::UnknownField`
        // sibling's code — one contract member for "the address names no declared leaf",
        // whichever leaf kind it named — so both doors route the caller at the schema.
        SpliceError::UnknownLeaf { .. } => ("write.unknown-field", None),
    };
    let message = format!("write rejected: {err}");
    blocking_write(
        code,
        message,
        location.unwrap_or_else(|| Location::at(1, 1)),
    )
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

## Options

Alternatives were weighed and rejected.

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
        // The **outward target stamp** the CLI applies before any write finding is projected
        // (`cli::doc::stamp_target`): the engine's write constructors hold a section + a field,
        // never a `type:slug`, so they emit `Location::at(line, col)` with **no** address, and
        // the CLI — which holds the parsed write address — stamps it on the way out. A raw
        // engine write finding is therefore not projectable at all (the presence half of the
        // membership test rides `Finding`'s own `Serialize`), so the golden pins what a driver
        // actually receives, not an intermediate the product never emits.
        let mut finding = finding;
        crate::finding::readdress_to_uri(
            std::slice::from_mut(&mut finding),
            "adr:rate-limit-at-the-gateway#status",
        );
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
                hint: None,
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

    /// N2: a `[…]`-wrapped value is the inline-list / `[]` ref-clear idiom — valid ONLY
    /// on a `ref` field (which recognizes and strips the bracket wrapper before the
    /// opaque floor). On a **non-ref** scalar (`string` / `code-anchor` / other pack
    /// type / `owned-location`) it is the ref-clear footgun misapplied: splicing it
    /// verbatim would write literal `[]` garbage. It is rejected at the opaque floor,
    /// and the reject routes to the real clear verb (`--unset`). The `ref` accept-path
    /// is unaffected (the in-task `check_ref` reorder) — the coupling this test guards.
    #[test]
    fn bracket_value_rejects_on_non_ref_scalar() {
        use crate::schema::{Field as SField, FieldType};
        let field = |ty: FieldType| SField {
            id: "f".into(),
            ty,
            of: None,
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

        // A `string` field: both the empty `[]` clear and a `[a, b]` list form are the
        // misapplied idiom — rejected, routing to `--unset`.
        let name = field(FieldType::String);
        let empty = check_value(&name, &scalar("[]")).expect_err("`[]` on a string rejects");
        assert!(
            empty.contains("--unset"),
            "the reject routes to the clear verb; got: {empty}"
        );
        assert!(check_value(&name, &scalar("[adr:a, adr:b]")).is_err());
        // A normal non-bracket string still passes — no over-rejection.
        assert!(check_value(&name, &scalar("anything goes")).is_ok());
        // A string that merely *contains* brackets (not a `[…]` wrapper) still passes.
        assert!(check_value(&name, &scalar("a [note] here")).is_ok());

        // `owned-location` (engine-native, opaque floor): `[]` rejects; a path passes.
        let loc = field(FieldType::OwnedLocation);
        assert!(check_value(&loc, &scalar("[]")).is_err());
        assert!(check_value(&loc, &scalar("docs/roadmap.md")).is_ok());

        // `code-anchor` (a pack-declared type on the opaque floor): `[]` rejects; a
        // real anchor still passes.
        let anchor = field(FieldType::Pack(crate::schema::PackFieldType {
            name: "code-anchor".into(),
            adjudicator: Some("doc-code".into()),
            check: Some("symbol-exists".into()),
            hint: None,
        }));
        assert!(check_value(&anchor, &scalar("[]")).is_err());
        assert!(check_value(&anchor, &scalar("src/foo.rs#bar")).is_ok());

        // The `ref` accept-path is unaffected: `[]` clears, `[a, b]` list-replaces.
        let r = SField {
            to: Some("adr".into()),
            ..field(FieldType::Ref)
        };
        assert!(check_value(&r, &scalar("[]")).is_ok());
        assert!(check_value(&r, &scalar("[adr:a, adr:b]")).is_ok());
        // A bracket ref form carrying a control char still rejects (the floor guard
        // survives the reorder — a trailing-newline element must not splice a 2nd line).
        assert!(check_value(&r, &scalar("[adr:a\nadr:b]")).is_err());
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

    /// (b), the **length-preserving** escape shape (2026-07-24 mutation audit, finding
    /// #8): the sibling test above tampers with a *shrinking* replacement, so its kill
    /// rides an accident of fixture arithmetic — a mutant that weakens the confinement
    /// conjunction to `(prefix && suffix) || len_ok` still rejects it (`len_ok` is
    /// false). A **same-length** byte flip outside the target span satisfies `len_ok`,
    /// so only the genuine prefix/suffix conjuncts can catch it. This is the safety net
    /// the item-slot corruption class rides; the escape must block for every diff shape,
    /// not only shrinking ones.
    #[test]
    fn length_preserving_target_escape_is_blocking_finding() {
        let span = locate_field_value(CANONICAL_ADR, "status").expect("status located");
        let original = "Each service drops its local limiter.";
        let replacement = "Every service keeps its local limiter"; // same byte length
        assert_eq!(
            original.len(),
            replacement.len(),
            "the tamper must be length-preserving — that is the untested shape"
        );
        let spliced = splice(CANONICAL_ADR, span.clone(), "accepted");
        let tampered = spliced.replace(original, replacement);
        assert_ne!(
            tampered, spliced,
            "the stray edit must actually change bytes"
        );
        assert_eq!(
            tampered.len(),
            spliced.len(),
            "the tampered buffer keeps the spliced buffer's exact length"
        );
        let finding = validate_after(&adr_schema(), CANONICAL_ADR, &tampered, span)
            .expect_err("a length-preserving diff outside the target span ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.target-escape");
    }

    /// The [`reparse_or_reject`] tripwire's own red path (2026-07-24 mutation audit,
    /// finding #9): the gate shared by the two `--unset` splice-remove arms is
    /// defense-in-depth — no reachable input through its gated callers is known to
    /// produce a non-reparseable buffer — but a stub (`Ok(())`) would silently disarm
    /// it, and the unreachability of the empty-sentinel-after-last-optional-field state
    /// is unproven. Pin the tripwire directly: a hand-corrupted buffer that no longer
    /// parses must return the blocking `write.non-reparseable`, and the canonical buffer
    /// must pass.
    #[test]
    fn reparse_or_reject_rejects_a_corrupted_buffer() {
        // A renamed section heading fails the strict positional parse.
        let corrupted = CANONICAL_ADR.replace("## Context", "## Bogus");
        assert_ne!(corrupted, CANONICAL_ADR, "the corruption must change bytes");
        let finding = reparse_or_reject(&adr_schema(), &corrupted)
            .expect_err("a non-reparseable buffer trips the gate");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.non-reparseable");

        // The green half: the canonical buffer passes untouched.
        reparse_or_reject(&adr_schema(), CANONICAL_ADR)
            .expect("the canonical buffer re-parses clean");
    }

    /// M45 Inc 2, T5 — **the two dead-end routes name their exit.** `write.non-reparseable`
    /// and `write.target-escape` are the pair whose stated recovery was "re-run the same
    /// write": true when the *payload* is the broken thing, false when the **staged source**
    /// is — and then the agent has nothing left to try. Both now name the shipped whole-task
    /// discard as well, with the re-run branch kept first where it genuinely applies.
    ///
    /// Both routes are pinned here at the map. `non-reparseable` is proven end-to-end over
    /// a broken staged source in `crates/cli/tests/write_not_present_route.rs` (arm (e)),
    /// where the CLI-installed parse fence adjudicates the identical
    /// `jigc task discard <task-id>` argv live. `target-escape` was recorded at M45 as
    /// *unreachable through the real binary without an injected defect*; **that stopped
    /// being true at M49 Increment 1 T5** — [`set_item_field_validated`] gives the
    /// item-field paths a confinement target, and a `set-field` whose located bullet is a
    /// slot-prose line escapes it through the real binary
    /// (`crates/cli/tests/item_region_boundary.rs`, the T5 arms).
    #[test]
    fn the_dead_end_write_routes_name_the_discard_escape_hatch() {
        for code in ["write.non-reparseable", "write.target-escape"] {
            let route = write_route(code);
            assert!(
                route
                    .as_str()
                    .contains("`jigc task discard <task-id> --force`"),
                "`{code}` routes the escape hatch beyond re-running; got: {route}",
            );
            assert!(
                route.as_str().contains("re-run"),
                "`{code}` keeps the re-run branch where it genuinely applies; got: {route}",
            );
        }
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

## Options

## Decision

## Consequences
";
        // `context` is a non-terminal slot (Options + Decision + Consequences follow it).
        // The prose deliberately lacks a trailing newline.
        let out = set_slot_validated(
            &adr_schema(),
            empty,
            SlotAddress::Section { section: "context" },
            "Per-client limits.",
        )
        .expect("filling an empty non-terminal slot must round-trip, trailing LF or not");
        // The buffer re-parses and the prose landed in `context`, not fused to the next
        // heading.
        let doc = parse::parse_sections(&adr_schema(), &out).expect("result re-parses");
        let ids: Vec<&str> = doc.sections.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(
            ids,
            ["status", "context", "options", "decision", "consequences"]
        );
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

## Options

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

## Options

Alternatives were weighed and rejected.

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
        let out = set_slot_validated(
            &schema,
            CANONICAL_ADR,
            SlotAddress::Section {
                section: "decision",
            },
            "We centralize.",
        )
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
            SlotAddress::Section { section: "decision" },
            "Intro.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nMore.",
        )
        .expect_err("a `##` heading in slot prose ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.slot-heading-depth");
        // The located finding names the offending line (line 3 within the prose).
        assert_eq!(finding.location.as_ref().unwrap().line, 3);
        // The **outward target stamp** the CLI applies before any write finding is projected
        // (`cli::doc::stamp_target`): the engine's write constructors hold a section + a field,
        // never a `type:slug`, so they emit `Location::at(line, col)` with **no** address, and
        // the CLI — which holds the parsed write address — stamps it on the way out. A raw
        // engine write finding is therefore not projectable at all (the presence half of the
        // membership test rides `Finding`'s own `Serialize`), so the golden pins what a driver
        // actually receives, not an intermediate the product never emits.
        let mut finding = finding;
        crate::finding::readdress_to_uri(
            std::slice::from_mut(&mut finding),
            "adr:rate-limit-at-the-gateway#decision",
        );
        insta::assert_snapshot!(
            "set_slot_ceiling_finding",
            serde_json::to_string_pretty(&finding).unwrap()
        );

        // --- Ceiling: a `### x` ATX heading (item depth) is rejected. ---
        let finding = set_slot_validated(
            &schema,
            CANONICAL_ADR,
            SlotAddress::Section {
                section: "decision",
            },
            "### Sneaky",
        )
        .expect_err("a `###` heading in slot prose ⇒ abort");
        assert_eq!(finding.code, "write.slot-heading-depth");
        assert_eq!(finding.location.as_ref().unwrap().line, 1);

        // --- Ceiling: a Setext underline heading is rejected at any depth. ---
        let finding = set_slot_validated(
            &schema,
            CANONICAL_ADR,
            SlotAddress::Section {
                section: "decision",
            },
            "A title\n=======",
        )
        .expect_err("a Setext heading in slot prose ⇒ abort");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.code, "write.slot-setext-heading");
        assert_eq!(finding.location.as_ref().unwrap().line, 1);
    }

    /// M43 inc-3 T3, **revised at M45 inc-2 T6** — the stated-at statement and the
    /// enforcing check share one depth vocabulary, and it is now the *derived*
    /// one: for the same `(Schema, section, item-chain)`, the sentence names
    /// exactly the depths the check rejects and the depth its findings route to
    /// (`surface-contract.md` → The stated-at fence, the M45 revision).
    #[test]
    fn ceiling_statement_and_check_share_the_derived_depth_vocabulary() {
        for (label, ceiling) in [
            ("a section slot", SlotCeiling::reserving(3)),
            ("a multi-slot depth-1 item", SlotCeiling::reserving(4)),
        ] {
            let statement = slot_ceiling_statement(&[(label.to_owned(), ceiling)]);
            let allowed = "#".repeat(ceiling.first_allowed);
            assert!(
                statement.contains(&format!("`{allowed}` depth or deeper")),
                "{label}: the statement offers `{allowed}`; got: {statement}"
            );
            for level in 2..=ceiling.reserved_max {
                let reserved = "#".repeat(level);
                assert!(
                    statement.contains(&format!("`{reserved}`")),
                    "{label}: the statement names the reserved `{reserved}`; got: {statement}"
                );
            }
            assert!(
                !statement.contains(&format!("`{}`", "#".repeat(ceiling.first_allowed + 1))),
                "{label}: the statement stops at the reserved set; got: {statement}"
            );
            // The enforcing finding routes to the SAME allowed depth the statement
            // states, and names the reserved depth it rejects.
            let deepest_reserved = "#".repeat(ceiling.reserved_max);
            let hit = slot_ceiling_finding(&format!("{deepest_reserved} nope\n"), ceiling)
                .expect("a heading at the deepest reserved depth trips the ceiling");
            assert!(hit.message.contains(&format!("`{deepest_reserved}`")));
            assert!(hit.message.contains(&format!("`{allowed}`")));
            let setext = slot_ceiling_finding("A title\n=======\n", ceiling)
                .expect("Setext trips the ceiling");
            assert!(setext.message.contains(&format!("`{allowed}`")));
        }
    }

    /// The fence input itself (M45 inc-2 T6): the reserved set the statement
    /// renders is derived per slot address, so a **multi-slot** item deepens it
    /// while a plain single-slot item does not — and a schema that mixes contexts
    /// (`changelog`'s depth-1 staging group beside its depth-2 nested one) renders
    /// each group against the slots it governs, never one blanket depth.
    #[test]
    fn schema_slot_ceilings_derive_the_reserved_set_per_address() {
        // Multi-slot at depth 1 — the `roadmap` shape: `####` is the sub-label
        // depth the CLI owns, so the statement must offer `#####`.
        let roadmap = crate::schema::load_schema(
            br#"
type: roadmap
placement: { file: docs/roadmap.md }
singleton: true
id-from: title
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: proves, slot: { hint: "What it proves." } }
        - { id: decomposition, slot: { hint: "The increments." } }
"#,
        )
        .expect("the roadmap fixture schema loads");
        let ceilings = schema_slot_ceilings(&roadmap);
        assert_eq!(
            ceilings,
            vec![
                ("milestones.proves".to_owned(), SlotCeiling::reserving(4)),
                (
                    "milestones.decomposition".to_owned(),
                    SlotCeiling::reserving(4)
                ),
            ]
        );
        let statement = slot_ceiling_statement(&ceilings);
        assert_eq!(
            statement,
            "Inside slot prose, headings must sit at `#####` depth or deeper — \
             `##`/`###`/`####` are schema-reserved, and Setext headings are rejected."
        );

        // The omitting context — single-slot at depth 1 leaves `####` free.
        let log = crate::schema::load_schema(
            br#"
type: log
placement: { file: docs/log.md }
singleton: true
id-from: title
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: why, slot: { hint: "Why." } }
"#,
        )
        .expect("the log fixture schema loads");
        assert_eq!(
            schema_slot_ceilings(&log),
            vec![("entries.why".to_owned(), SlotCeiling::reserving(3))]
        );

        // Mixed — a section slot, a depth-1 single-slot item, and a depth-2
        // nested one (the `changelog` shape).
        let mixed = crate::schema::load_schema(
            br#"
type: mixed
placement: { file: docs/mixed.md }
singleton: true
id-from: title
sections:
  - id: overview
    slot: { hint: "The overview." }
  - id: releases
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - id: changes
          repeatable:
            id-from: title
            block:
              - { id: title, type: string }
              - { id: notes, slot: { hint: "The notes." } }
"#,
        )
        .expect("the mixed fixture schema loads");
        let mixed_ceilings = schema_slot_ceilings(&mixed);
        assert_eq!(
            mixed_ceilings,
            vec![
                ("overview".to_owned(), SlotCeiling::reserving(3)),
                (
                    "releases.changes.notes".to_owned(),
                    SlotCeiling::reserving(4)
                ),
            ]
        );
        assert_eq!(
            slot_ceiling_statement(&mixed_ceilings),
            "Inside slot prose, the reserved depths differ by slot — headings must sit \
             at `####` or deeper in `overview`, at `#####` or deeper in \
             `releases.changes.notes`. Setext headings are rejected."
        );

        // Slot-less — inert, never boilerplate.
        assert_eq!(slot_ceiling_statement(&[]), "");
    }

    /// The address-independent form names no depth (M45 inc-2 T6): the surfaces
    /// composed before an address exists — `doc set-slot --help`, the pack steps'
    /// hand-prose solicits — must not bless one, or the parameterization's whole
    /// point is undone by the sentence beside it.
    #[test]
    fn the_address_free_rule_statement_names_no_depth() {
        let rule = slot_ceiling_rule_statement();
        assert!(!rule.contains('#'), "the rule names no depth; got: {rule}");
        for fragment in ["schema-relative", "Setext", "shallowest depth free"] {
            assert!(
                rule.contains(fragment),
                "the rule states `{fragment}`; got: {rule}"
            );
        }
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
             ## Context\n\n{context}\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\n{decision}\n\n\
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
    /// subset of `[context, options, decision, consequences]`, kept in schema order),
    /// each with a one-line opaque slot prose.
    fn build_partial_adr(present: &[&str]) -> String {
        let mut out =
            String::from("---\nstatus: proposed\ndate: 2026-05-31\n---\n\n# A decision\n");
        for id in present {
            let heading = match *id {
                "context" => "Context",
                "options" => "Options",
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
            mask in prop::collection::vec(any::<bool>(), 4..=4)
                .prop_filter("at least one absent", |m| !m.iter().all(|&b| b)),
            target_pick in 0usize..4,
        ) {
            let body = ["context", "options", "decision", "consequences"];
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
            // The fully-generated case (all four now present) must also conform.
            if got_in_source_order.len() == 4 {
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
    const CHANGELOG_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/changelog.yaml");

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
    /// The shipped `changelog` schema — the riskiest fuzzed shape. It is the first
    /// fuzzed doctype with a **TWO-LEVEL repeatable** (`releases` → nested `changes`
    /// change-groups, the `Leaf::Repeatable` path), a **multi-word section id**
    /// (`unreleased-changes` → `## Unreleased Changes`), and an **optional field**
    /// (a release's `link`, present *and* absent). It declares only engine-native
    /// field types (`enum`/`date`/`string`), so it loads bare (no pack types). It has
    /// NO header/front-matter (`id-from: title`, the H1), so it carries no settable
    /// scalar — the surgical-edit clause self-skips on `edit: None`.
    fn changelog_schema() -> Schema {
        crate::schema::load_schema(CHANGELOG_YAML).expect("changelog.yaml loads")
    }

    /// One generated `changelog` release: `(version, date, optional link, nested
    /// change-groups)` — the `releases` two-level item, named to keep the generator's
    /// release vectors readable (and below the type-complexity ceiling).
    type GenRelease = (String, String, Option<String>, Vec<(String, String)>);

    /// One generated arbitrary conformant document plus its known-canonical LF form
    /// and the metadata the surgical-edit clause needs.
    #[derive(Clone, Debug)]
    struct GenDoc {
        /// `"commit"`, `"adr"`, `"prd"`, `"spec"`, `"arch-doc"`, or `"changelog"` —
        /// selects the schema.
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

    /// Opaque change-group `notes` prose for a `changelog`. Like [`edgy_prose`] but its
    /// deeper-heading fragment is at **`#####` (H5)**, never `####` (H4): a change-group
    /// nests at `####` (a release's `Leaf::Repeatable`), so a `####` heading inside its
    /// slot would collide with a *sibling* change-group heading and end the slot
    /// (verified: a `####`-in-notes fixture fails the conformance gate with
    /// `item-heading-unanchored`). H5 is deeper than both the unreleased depth (`###`) and
    /// the nested depth (`####`), so it stays prose at either — the safe deeper-heading
    /// stress for this doctype. The fenced-`##`, `- x:` prose line, and interior-blank
    /// edges (all proven safe at `####`) are kept.
    fn notes_prose() -> impl Strategy<Value = String> {
        let para = "[a-zA-Z][a-zA-Z0-9 .,]{0,30}";
        let fragment = prop_oneof![
            para,
            Just("- one bullet per change".to_string()),
            Just("```\n## not a heading\n```".to_string()),
            Just("- x: this is prose, not a field".to_string()),
            Just("##### a deeper heading is allowed".to_string()),
            "[a-zA-Z][a-zA-Z0-9 .,]{0,30}".prop_map(|s: String| format!("{s}\n\n\n{s}")),
        ];
        prop::collection::vec(fragment, 1..4).prop_map(|frags| frags.join("\n\n"))
    }

    /// A `changelog` change-group set: a `size`-bounded **distinct** subsequence of the
    /// six `category` enum members, each paired with arbitrary `notes` prose. The
    /// distinctness is load-bearing — a repeatable keyed `id-from: category` rejects
    /// duplicate `{#id}` anchors, so two same-category groups in one section would be
    /// non-conformant. Drawing a subsequence of the fixed enum guarantees both the
    /// distinct ids and that every category renders as its exact lowercase enum value
    /// (the heading text == the enum value, e.g. `#### added  {#added}`).
    fn change_groups(
        size: impl Into<proptest::collection::SizeRange>,
    ) -> impl Strategy<Value = Vec<(String, String)>> {
        let categories = vec![
            "added",
            "changed",
            "deprecated",
            "removed",
            "fixed",
            "security",
        ];
        prop::sample::subsequence(categories, size)
            .prop_flat_map(|cats| {
                let owned: Vec<String> = cats.into_iter().map(str::to_string).collect();
                let n = owned.len();
                (Just(owned), prop::collection::vec(notes_prose(), n))
            })
            .prop_map(|(cats, notes)| cats.into_iter().zip(notes).collect())
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
            "---\n{fm}---\n\n# {title}\n\n## Context\n\n{context}\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\n{decision}\n\n## Consequences\n\n{consequences}\n",
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
                SectionContent {
                    id: "options".to_string(),
                    slot: Some("Alternatives were weighed and rejected.".to_string()),
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
                SectionContent {
                    id: "options".to_string(),
                    slot: Some("Alternatives were weighed and rejected.".to_string()),
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

    /// One `changelog` change-group [`ItemContent`]: the `category` enum value is the
    /// `id-from` heading (consumed as both the heading text and the `{#id}` anchor,
    /// never a field bullet — like `prd.requirements`'s `title`), and `notes` is the
    /// bare-prose slot. Shared by the single-level `unreleased-changes` groups and the
    /// nested-under-a-release `changes` groups (the identical block, duplicated by the
    /// schema deliberately).
    fn change_group_item(category: &str, notes: &str) -> ItemContent {
        ItemContent {
            id: category.to_string(),
            title: category.to_string(),
            slot: Some(notes.to_string()),
            slots: Vec::new(),
            fields: Vec::new(),
            items: Vec::new(),
        }
    }

    /// A canonical `date` field value (`YYYY-MM-DD`, the form the `date` type and the
    /// `set: on-create` stamp emit). Day capped at 28 so every `(y, m, d)` is a real
    /// date.
    fn date_value() -> impl Strategy<Value = String> {
        (2020u32..2030, 1u32..=12, 1u32..=28).prop_map(|(y, m, d)| format!("{y:04}-{m:02}-{d:02}"))
    }

    /// A canonical `link` field value (a KaC diff URL). Opaque single-line scalar with
    /// no spaces, so it round-trips verbatim; modelling the real URL shape drives a
    /// realistic optional-field value.
    fn link_value() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9-]{0,12}".prop_map(|s: String| format!("https://example.com/compare/{s}"))
    }

    /// Build a canonical-LF `changelog` document from generated parts by constructing an
    /// [`Instance`] and **rendering it** — the riskiest fuzzed shape. `changelog` has NO
    /// front-matter; its two sections are the multi-word `## Unreleased Changes`
    /// (`unreleased-changes`, a single-level change-group repeatable, possibly EMPTY —
    /// the post-cut KaC state, which renders the bare `## Unreleased Changes` heading and
    /// round-trips byte-stable) and `## Releases` (a TWO-LEVEL repeatable). Each release
    /// is a slotless item carrying a `date` field, an OPTIONAL `link` field (present →
    /// `[date, link]`, absent → `[date]` — the optional-field byte risk), then a NESTED
    /// `changes` change-group repeatable (the `Leaf::Repeatable` path, each group at
    /// `####`). Building through `render` guarantees the bytes match the canonical form.
    fn build_changelog(
        title: &str,
        unreleased: &[(String, String)],
        releases: &[GenRelease],
    ) -> String {
        let unreleased_items: Vec<ItemContent> = unreleased
            .iter()
            .map(|(category, notes)| change_group_item(category, notes))
            .collect();
        let release_items: Vec<ItemContent> = releases
            .iter()
            .map(|(version, date, link, changes)| {
                // Field order follows the schema block order: `date` then the optional
                // `link` (omitted when absent).
                let mut fields = vec![Field {
                    key: "date".to_string(),
                    value: Value::Scalar(date.clone()),
                }];
                if let Some(link) = link {
                    fields.push(Field {
                        key: "link".to_string(),
                        value: Value::Scalar(link.clone()),
                    });
                }
                ItemContent {
                    id: crate::slug::slugify(version),
                    title: version.clone(),
                    slot: None,
                    slots: Vec::new(),
                    fields,
                    items: changes
                        .iter()
                        .map(|(category, notes)| change_group_item(category, notes))
                        .collect(),
                }
            })
            .collect();
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "unreleased-changes".to_string(),
                    items: unreleased_items,
                    ..Default::default()
                },
                SectionContent {
                    id: "releases".to_string(),
                    items: release_items,
                    ..Default::default()
                },
            ],
        };
        render(&changelog_schema(), &instance)
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

        let changelog = (
            scalar_value(),
            // `unreleased-changes`: 0..=6 distinct change-groups. The `0` lower bound
            // generates the EMPTY-unreleased case (the post-cut KaC state → a bare
            // `## Unreleased Changes` heading) as well as the populated one, so a single
            // run covers both — and the multi-word section heading renders either way.
            change_groups(0..=6),
            // `releases`: 1..4 releases, EACH with 1..=6 nested change-groups (the
            // done-criterion's ">=1 nested release with >=1 nested change-group") and an
            // OPTIONAL `link` — `option::of` makes a single run cover BOTH present and
            // absent.
            prop::collection::vec(
                (
                    scalar_value(),
                    date_value(),
                    proptest::option::of(link_value()),
                    change_groups(1..=6),
                ),
                1..4,
            ),
        )
            .prop_map(|(title, unreleased, releases)| {
                // Index-suffix each release version so the slugified `{#id}` anchors are
                // distinct — a repeatable rejects duplicate anchors, and two arbitrary
                // version strings can slug-collide (the version slugger drops dots, so
                // `1.0.0` and `1.0.0` both mint `100`). The suffix also exercises the
                // multi-word-heading parse on every release title.
                let releases: Vec<GenRelease> = releases
                    .into_iter()
                    .enumerate()
                    .map(|(i, (version, date, link, changes))| {
                        (format!("{} {i}", version.trim()), date, link, changes)
                    })
                    .collect();
                (
                    "changelog".to_string(),
                    build_changelog(&title, &unreleased, &releases),
                    // `changelog` has no front-matter, so there is no settable field; the
                    // surgical-edit clause self-skips on `edit: None`.
                    None,
                )
            });

        (
            prop_oneof![commit, adr, prd, spec, arch_doc, changelog],
            eol,
        )
            .prop_map(|((ty, canonical_lf, edit), eol)| GenDoc {
                ty,
                canonical_lf,
                eol,
                edit,
            })
    }

    fn schema_for(ty: &str) -> Schema {
        match ty {
            "commit" => commit_schema(),
            "prd" => prd_schema(),
            "spec" => spec_schema(),
            "arch-doc" => arch_doc_schema(),
            "changelog" => changelog_schema(),
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

## Options

Alternatives were weighed and rejected.

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

// ============================================================================
// The M40 methodology byte-stability census — the M33 `roundtrip` pattern over
// the NINE persisted methodology doctypes (`roadmap`, `deferral-ledger`,
// `decisions-log`, `completion-record`, `dogfood-record`, `vision`, `research`,
// `idea`, `milestone-record`). Discharges the recorded freeze precondition
// (`implementation/doctype-map.md` → the revised scope pin; `DECISIONS.md`
// 2026-07-10 M40 Settle item 1): the methodology pack's manifest (T4) may only
// hash-freeze shapes proven byte-stable, and M33's census deliberately scoped
// the methodology doctypes out. Same clauses as `mod roundtrip`:
//
//   1. **render∘parse == id** on the canonical LF form (built THROUGH `render`
//      from the shipped schema YAML), plus the no-op write idempotent modulo the
//      canonicalization ledger across LF/CRLF.
//   2. **Surgical on edits** where a settable `meta` front-matter scalar exists
//      (`completion-record`/`dogfood-record` `verdict`, `research` `date`,
//      `idea` `trigger`, `milestone-record` `base`) — a single `set_field`
//      changes only that field's value bytes.
//
// The schema bytes are loaded test-only via `include_bytes!` from
// `packs/methodology/schemas/` (the M33 pattern) — the engine-empty invariant
// is intact: nothing ships in the engine binary.
// ============================================================================
#[cfg(test)]
mod methodology_roundtrip {
    //! The methodology byte-stability census (M40 inc-5 T2). One named generator
    //! arm per persisted methodology doctype; every instance is built through
    //! [`render`] over the **shipped** schema YAML, so the fuzz drives the real
    //! pack shapes — the multi-slot repeatable (`roadmap`), slot+field items
    //! (`deferral-ledger`/`decisions-log`), fields-only items
    //! (`completion-record` findings, `milestone-record` tasks), the 14-field
    //! header (`dogfood-record`), and the optional front-matter ref-list
    //! (`vision.grounded-in`).

    use super::*;
    use crate::parse::parse_sections;
    use crate::schema::Schema;
    use proptest::prelude::*;

    const ROADMAP_YAML: &[u8] = include_bytes!("../../../packs/methodology/schemas/roadmap.yaml");
    const DEFERRAL_LEDGER_YAML: &[u8] =
        include_bytes!("../../../packs/methodology/schemas/deferral-ledger.yaml");
    const DECISIONS_LOG_YAML: &[u8] =
        include_bytes!("../../../packs/methodology/schemas/decisions-log.yaml");
    const COMPLETION_RECORD_YAML: &[u8] =
        include_bytes!("../../../packs/methodology/schemas/completion-record.yaml");
    const DOGFOOD_RECORD_YAML: &[u8] =
        include_bytes!("../../../packs/methodology/schemas/dogfood-record.yaml");
    const VISION_YAML: &[u8] = include_bytes!("../../../packs/methodology/schemas/vision.yaml");
    const RESEARCH_YAML: &[u8] = include_bytes!("../../../packs/methodology/schemas/research.yaml");
    const IDEA_YAML: &[u8] = include_bytes!("../../../packs/methodology/schemas/idea.yaml");
    const MILESTONE_RECORD_YAML: &[u8] =
        include_bytes!("../../../packs/methodology/schemas/milestone-record.yaml");
    const PLANNING_RECORD_YAML: &[u8] =
        include_bytes!("../../../packs/methodology/schemas/planning-record.yaml");

    /// Load the shipped schema for one methodology doctype. Every methodology
    /// doctype declares **engine-native** field types only (`string`/`enum`/
    /// `date`/`int`/`ref`/`owned-location`), so all ten load bare — no pack
    /// field-type registration (unlike the dev pack's `code-anchor`).
    fn schema_for(ty: &str) -> Schema {
        let bytes: &[u8] = match ty {
            "roadmap" => ROADMAP_YAML,
            "deferral-ledger" => DEFERRAL_LEDGER_YAML,
            "decisions-log" => DECISIONS_LOG_YAML,
            "completion-record" => COMPLETION_RECORD_YAML,
            "dogfood-record" => DOGFOOD_RECORD_YAML,
            "vision" => VISION_YAML,
            "research" => RESEARCH_YAML,
            "idea" => IDEA_YAML,
            "milestone-record" => MILESTONE_RECORD_YAML,
            "planning-record" => PLANNING_RECORD_YAML,
            other => panic!("unknown methodology doctype {other:?}"),
        };
        crate::schema::load_schema(bytes)
            .unwrap_or_else(|e| panic!("shipped {ty} schema loads: {e:?}"))
    }

    /// One generated `deferral-ledger` entry: `(title, kind, trigger, date, body)`.
    type GenLedgerEntry = (String, String, String, String, String);
    /// One generated `completion-record` finding:
    /// `(title, severity, disposition, evidence)`.
    type GenFinding = (String, String, String, String);

    /// One generated arbitrary conformant document plus its known-canonical LF
    /// form and the metadata the surgical-edit clause needs (the M33 `GenDoc`).
    #[derive(Clone, Debug)]
    struct GenDoc {
        /// The methodology doctype id — selects the schema.
        ty: String,
        /// The canonical LF document text (no BOM, exactly one trailing `\n`).
        canonical_lf: String,
        /// The EOL the perturbed variant uses (`"\n"` or `"\r\n"`).
        eol: String,
        /// A present `meta` front-matter field key + a fresh canonical value to
        /// set it to (drives clause 2). `None` where the doctype has no settable
        /// header scalar (`roadmap`/`deferral-ledger`/`decisions-log` are
        /// header-less; `vision`'s only header field is a ref-list).
        edit: Option<(String, String)>,
    }

    /// A canonical front-matter/field scalar value (trimmed, single-line, not
    /// list-shaped) — the M33 `scalar_value`.
    fn scalar_value() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9][a-zA-Z0-9 ._/-]{0,18}[a-zA-Z0-9]|[a-zA-Z0-9]"
            .prop_filter("not list-shaped", |s: &String| {
                !(s.starts_with('[') && s.ends_with(']'))
            })
    }

    /// A canonical `date` field value (`YYYY-MM-DD`). Day capped at 28 so every
    /// `(y, m, d)` is a real date. Years below 2031, so the clause-2 fresh date
    /// (`2031-01-15`) always differs from a generated one.
    fn date_value() -> impl Strategy<Value = String> {
        (2020u32..2030, 1u32..=12, 1u32..=28).prop_map(|(y, m, d)| format!("{y:04}-{m:02}-{d:02}"))
    }

    /// A canonical `owned-location` value — a run-artifact directory path (the
    /// shipped `owner-artifact` shape, e.g. `completions/artifacts/M16/`).
    /// Opaque to the reader, so it round-trips verbatim; modelling the real path
    /// drives a realistic value.
    fn owner_artifact_value() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9-]{0,12}".prop_map(|s: String| format!("completions/artifacts/{s}/"))
    }

    /// Opaque slot prose for a **section-level** slot or a `###`-item slot — the
    /// M33 `edgy_prose`: a fenced block whose body is `## not a heading`, a
    /// `- x:` line that is prose (no sentinel), a `####` deeper heading (safe at
    /// section/`###`-item depth), and extra interior blank lines.
    fn section_prose() -> impl Strategy<Value = String> {
        let para = "[a-zA-Z][a-zA-Z0-9 .,]{0,30}";
        let fragment = prop_oneof![
            para,
            Just("```\n## not a heading\n```".to_string()),
            Just("- x: this is prose, not a field".to_string()),
            Just("#### a deeper heading is allowed".to_string()),
            "[a-zA-Z][a-zA-Z0-9 .,]{0,30}".prop_map(|s: String| format!("{s}\n\n\n{s}")),
        ];
        prop::collection::vec(fragment, 1..4).prop_map(|frags| frags.join("\n\n"))
    }

    /// Opaque prose for a **multi-slot leaf** (the roadmap's `proves`/
    /// `decomposition`, rendered under a `####` sub-heading): like
    /// [`section_prose`] but its deeper-heading fragment is at **`#####` (H5)**,
    /// never `####` — a `####` heading inside a leaf's prose would collide with a
    /// *sibling* leaf sub-heading level (the M33 `notes_prose` reasoning, applied
    /// to the multi-slot shape).
    fn leaf_prose() -> impl Strategy<Value = String> {
        let para = "[a-zA-Z][a-zA-Z0-9 .,]{0,30}";
        let fragment = prop_oneof![
            para,
            Just("```\n## not a heading\n```".to_string()),
            Just("- x: this is prose, not a field".to_string()),
            Just("##### a deeper heading is allowed".to_string()),
            "[a-zA-Z][a-zA-Z0-9 .,]{0,30}".prop_map(|s: String| format!("{s}\n\n\n{s}")),
        ];
        prop::collection::vec(fragment, 1..4).prop_map(|frags| frags.join("\n\n"))
    }

    /// Build a canonical-LF `roadmap` by constructing an [`Instance`] and
    /// **rendering it** — the census's first fuzz of the **multi-slot repeatable
    /// item** (M16): each milestone renders its `proves`/`decomposition` prose
    /// under `#### Proves` / `#### Decomposition` sub-headings. `0..` milestones
    /// covers the fresh-singleton EMPTY repeatable (a bare `## Milestones`).
    fn build_roadmap(title: &str, milestones: &[(String, String, String)]) -> String {
        let items: Vec<ItemContent> = milestones
            .iter()
            .map(|(m_title, proves, decomposition)| ItemContent {
                id: crate::slug::slugify(m_title),
                title: m_title.clone(),
                slot: None,
                slots: vec![
                    ("proves".to_string(), proves.clone()),
                    ("decomposition".to_string(), decomposition.clone()),
                ],
                fields: Vec::new(),
                items: Vec::new(),
            })
            .collect();
        let instance = Instance {
            title: title.to_string(),
            sections: vec![SectionContent {
                id: "milestones".to_string(),
                items,
                ..Default::default()
            }],
        };
        render(&schema_for("roadmap"), &instance)
    }

    /// Build a canonical-LF `deferral-ledger`: header-less, one `entries`
    /// repeatable whose items carry a `body` slot **plus** a three-field group
    /// (`kind` enum / `trigger` string / `date`) — the slot-then-fields item
    /// shape (`spec.criteria` precedent, with three fields).
    fn build_deferral_ledger(title: &str, entries: &[GenLedgerEntry]) -> String {
        let items: Vec<ItemContent> = entries
            .iter()
            .map(|(e_title, kind, trigger, date, body)| ItemContent {
                id: crate::slug::slugify(e_title),
                title: e_title.clone(),
                slot: Some(body.clone()),
                slots: Vec::new(),
                fields: vec![
                    Field {
                        key: "kind".to_string(),
                        value: Value::Scalar(kind.clone()),
                    },
                    Field {
                        key: "trigger".to_string(),
                        value: Value::Scalar(trigger.clone()),
                    },
                    Field {
                        key: "date".to_string(),
                        value: Value::Scalar(date.clone()),
                    },
                ],
                items: Vec::new(),
            })
            .collect();
        let instance = Instance {
            title: title.to_string(),
            sections: vec![SectionContent {
                id: "entries".to_string(),
                items,
                ..Default::default()
            }],
        };
        render(&schema_for("deferral-ledger"), &instance)
    }

    /// Build a canonical-LF `decisions-log`: header-less, one `entries`
    /// repeatable whose items carry a `why` slot plus a single `date` field.
    fn build_decisions_log(title: &str, entries: &[(String, String, String)]) -> String {
        let items: Vec<ItemContent> = entries
            .iter()
            .map(|(e_title, date, why)| ItemContent {
                id: crate::slug::slugify(e_title),
                title: e_title.clone(),
                slot: Some(why.clone()),
                slots: Vec::new(),
                fields: vec![Field {
                    key: "date".to_string(),
                    value: Value::Scalar(date.clone()),
                }],
                items: Vec::new(),
            })
            .collect();
        let instance = Instance {
            title: title.to_string(),
            sections: vec![SectionContent {
                id: "entries".to_string(),
                items,
                ..Default::default()
            }],
        };
        render(&schema_for("decisions-log"), &instance)
    }

    /// Build a canonical-LF `completion-record`: a `meta` header (a `verdict`
    /// enum and an `owner-artifact` owned-location) and a `findings` repeatable
    /// of **fields-only** (slotless) items — `severity`/`disposition`/
    /// `evidence`, the changelog-release shape without nesting. `0..` findings
    /// covers the clean-audit EMPTY repeatable.
    fn build_completion_record(
        title: &str,
        verdict: &str,
        owner: &str,
        findings: &[GenFinding],
    ) -> String {
        let items: Vec<ItemContent> = findings
            .iter()
            .map(|(f_title, severity, disposition, evidence)| ItemContent {
                id: crate::slug::slugify(f_title),
                title: f_title.clone(),
                slot: None,
                slots: Vec::new(),
                fields: vec![
                    Field {
                        key: "severity".to_string(),
                        value: Value::Scalar(severity.clone()),
                    },
                    Field {
                        key: "disposition".to_string(),
                        value: Value::Scalar(disposition.clone()),
                    },
                    Field {
                        key: "evidence".to_string(),
                        value: Value::Scalar(evidence.clone()),
                    },
                ],
                items: Vec::new(),
            })
            .collect();
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![
                        Field {
                            key: "verdict".to_string(),
                            value: Value::Scalar(verdict.to_string()),
                        },
                        Field {
                            key: "owner-artifact".to_string(),
                            value: Value::Scalar(owner.to_string()),
                        },
                    ],
                    ..Default::default()
                },
                SectionContent {
                    id: "findings".to_string(),
                    items,
                    ..Default::default()
                },
            ],
        };
        render(&schema_for("completion-record"), &instance)
    }

    /// Field keys of the ten transcribed `dogfood-record` int facts, schema order.
    const DOGFOOD_INT_FIELDS: [&str; 10] = [
        "adapter-writes",
        "oob-edits",
        "drift-caught",
        "validate-blocks",
        "halts-expected",
        "halts-unplanned",
        "fix-rounds",
        "audit-findings",
        "seeded-oob",
        "seeded-blocks",
    ];

    /// Build a canonical-LF `dogfood-record`: the census's widest header — a
    /// `meta` front-matter of **14 fields** (`case` enum, `binary-sha`, the ten
    /// `int` facts — negatives included, the stated stores-verbatim dialect
    /// bound — `verdict` enum, `owner-artifact`) plus the `judgment` prose slot.
    fn build_dogfood_record(
        title: &str,
        case: &str,
        sha: &str,
        ints: &[i64],
        verdict: &str,
        owner: &str,
        judgment: &str,
    ) -> String {
        let mut fields = vec![
            Field {
                key: "case".to_string(),
                value: Value::Scalar(case.to_string()),
            },
            Field {
                key: "binary-sha".to_string(),
                value: Value::Scalar(sha.to_string()),
            },
        ];
        for (key, n) in DOGFOOD_INT_FIELDS.iter().zip(ints) {
            fields.push(Field {
                key: (*key).to_string(),
                value: Value::Scalar(n.to_string()),
            });
        }
        fields.push(Field {
            key: "verdict".to_string(),
            value: Value::Scalar(verdict.to_string()),
        });
        fields.push(Field {
            key: "owner-artifact".to_string(),
            value: Value::Scalar(owner.to_string()),
        });
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields,
                    ..Default::default()
                },
                SectionContent {
                    id: "judgment".to_string(),
                    slot: Some(judgment.to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&schema_for("dogfood-record"), &instance)
    }

    /// Build a canonical-LF `vision`: a `meta` header whose only field is the
    /// `grounded-in` ref-list (`card: 0..*` — 0 refs → the field is OMITTED, an
    /// empty `---\n---` header; ≥1 → an inline-flow `grounded-in: [research:…]`
    /// list, the `arch-doc.cites` shape), then the `thesis`/`invariants`/
    /// `open-questions` prose slots (`open-questions` exercises the multi-word
    /// section heading `## Open Questions`).
    fn build_vision(
        title: &str,
        grounded: &[String],
        thesis: &str,
        invariants: &str,
        open_questions: &str,
    ) -> String {
        let meta_fields = if grounded.is_empty() {
            Vec::new()
        } else {
            vec![Field {
                key: "grounded-in".to_string(),
                value: Value::List(grounded.to_vec()),
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
                    id: "thesis".to_string(),
                    slot: Some(thesis.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "invariants".to_string(),
                    slot: Some(invariants.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "open-questions".to_string(),
                    slot: Some(open_questions.to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&schema_for("vision"), &instance)
    }

    /// Build a canonical-LF `planning-record`: header-less, one prose slot per
    /// planning gate. **The section ids come from the shipped schema**, never a hand
    /// list — the gate set is settled in `design/methodology-docs.md` and a gate added
    /// there (and to the schema) joins this generator by construction rather than
    /// silently escaping the census.
    fn build_planning_record(title: &str, prose: &[String]) -> String {
        let schema = schema_for("planning-record");
        assert_eq!(
            prose.len(),
            schema.sections.len(),
            "one prose body per declared gate slot",
        );
        let instance = Instance {
            title: title.to_string(),
            sections: schema
                .sections
                .iter()
                .zip(prose)
                .map(|(section, body)| SectionContent {
                    id: section.id.clone(),
                    slot: Some(body.clone()),
                    ..Default::default()
                })
                .collect(),
        };
        render(&schema, &instance)
    }

    /// Build a canonical-LF `research`: a `meta` header with the one `date`
    /// field, then the `question`/`findings`/`sources` prose slots.
    fn build_research(
        title: &str,
        date: &str,
        question: &str,
        findings: &str,
        sources: &str,
    ) -> String {
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![Field {
                        key: "date".to_string(),
                        value: Value::Scalar(date.to_string()),
                    }],
                    ..Default::default()
                },
                SectionContent {
                    id: "question".to_string(),
                    slot: Some(question.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "findings".to_string(),
                    slot: Some(findings.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "sources".to_string(),
                    slot: Some(sources.to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&schema_for("research"), &instance)
    }

    /// Build a canonical-LF `idea`: a `meta` header with `trigger` + `date`,
    /// then the `description` prose slot.
    fn build_idea(title: &str, trigger: &str, date: &str, description: &str) -> String {
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![
                        Field {
                            key: "trigger".to_string(),
                            value: Value::Scalar(trigger.to_string()),
                        },
                        Field {
                            key: "date".to_string(),
                            value: Value::Scalar(date.to_string()),
                        },
                    ],
                    ..Default::default()
                },
                SectionContent {
                    id: "description".to_string(),
                    slot: Some(description.to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&schema_for("idea"), &instance)
    }

    /// Build a canonical-LF `milestone-record`: a `meta` header (`base` SHA pin +
    /// `status` enum) and a `tasks` repeatable whose items are keyed
    /// `id-from: task-id` — the task-id IS both the heading text and the `{#id}`
    /// anchor (a slug, not a slugified title), with `intent`/`status` as the
    /// fields-only body (the shipped `render_fresh_record`/`append_task_item`
    /// byte form). `0..` tasks covers the fresh-record EMPTY repeatable.
    fn build_milestone_record(
        title: &str,
        base: &str,
        status: &str,
        tasks: &[(String, String, String)],
    ) -> String {
        let items: Vec<ItemContent> = tasks
            .iter()
            .map(|(task_id, intent, task_status)| ItemContent {
                id: task_id.clone(),
                title: task_id.clone(),
                slot: None,
                slots: Vec::new(),
                fields: vec![
                    Field {
                        key: "intent".to_string(),
                        value: Value::Scalar(intent.clone()),
                    },
                    Field {
                        key: "status".to_string(),
                        value: Value::Scalar(task_status.clone()),
                    },
                ],
                items: Vec::new(),
            })
            .collect();
        let instance = Instance {
            title: title.to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![
                        Field {
                            key: "base".to_string(),
                            value: Value::Scalar(base.to_string()),
                        },
                        Field {
                            key: "status".to_string(),
                            value: Value::Scalar(status.to_string()),
                        },
                    ],
                    ..Default::default()
                },
                SectionContent {
                    id: "tasks".to_string(),
                    items,
                    ..Default::default()
                },
            ],
        };
        render(&schema_for("milestone-record"), &instance)
    }

    /// The generator: an arbitrary conformant doc of one of the NINE persisted
    /// methodology doctypes — one named arm per doctype — with a chosen EOL.
    /// Item titles are index-suffixed so slugified `{#id}` anchors stay distinct
    /// (a repeatable rejects duplicate anchors; the M33 discipline).
    fn arb_doc() -> impl Strategy<Value = GenDoc> {
        let eol = prop_oneof![Just("\n".to_string()), Just("\r\n".to_string())];

        let roadmap = (
            scalar_value(),
            prop::collection::vec((scalar_value(), leaf_prose(), leaf_prose()), 0..3),
        )
            .prop_map(|(title, raw)| {
                let milestones: Vec<(String, String, String)> = raw
                    .into_iter()
                    .enumerate()
                    .map(|(i, (m_title, proves, decomposition))| {
                        (format!("{} {i}", m_title.trim()), proves, decomposition)
                    })
                    .collect();
                (
                    "roadmap".to_string(),
                    build_roadmap(&title, &milestones),
                    // Header-less (no front-matter section): no settable scalar;
                    // the surgical-edit clause self-skips on `edit: None`.
                    None,
                )
            });

        let deferral_ledger = (
            scalar_value(),
            prop::collection::vec(
                (
                    scalar_value(),
                    prop::sample::select(vec!["Decision", "Idea"]),
                    scalar_value(),
                    date_value(),
                    section_prose(),
                ),
                1..3,
            ),
        )
            .prop_map(|(title, raw)| {
                let entries: Vec<GenLedgerEntry> = raw
                    .into_iter()
                    .enumerate()
                    .map(|(i, (e_title, kind, trigger, date, body))| {
                        (
                            format!("{} {i}", e_title.trim()),
                            kind.to_string(),
                            trigger,
                            date,
                            body,
                        )
                    })
                    .collect();
                (
                    "deferral-ledger".to_string(),
                    build_deferral_ledger(&title, &entries),
                    None,
                )
            });

        let decisions_log = (
            scalar_value(),
            prop::collection::vec((scalar_value(), date_value(), section_prose()), 1..3),
        )
            .prop_map(|(title, raw)| {
                let entries: Vec<(String, String, String)> = raw
                    .into_iter()
                    .enumerate()
                    .map(|(i, (e_title, date, why))| (format!("{} {i}", e_title.trim()), date, why))
                    .collect();
                (
                    "decisions-log".to_string(),
                    build_decisions_log(&title, &entries),
                    None,
                )
            });

        let completion_record = (
            scalar_value(),
            prop::sample::select(vec!["green", "red"]),
            owner_artifact_value(),
            // `0..` findings covers the clean-audit EMPTY repeatable as well as
            // the populated fields-only items.
            prop::collection::vec(
                (
                    scalar_value(),
                    prop::sample::select(vec!["blocking", "advisory"]),
                    prop::sample::select(vec!["fixed", "deferred", "contested"]),
                    scalar_value(),
                ),
                0..3,
            ),
        )
            .prop_map(|(title, verdict, owner, raw)| {
                let findings: Vec<GenFinding> = raw
                    .into_iter()
                    .enumerate()
                    .map(|(i, (f_title, severity, disposition, evidence))| {
                        (
                            format!("{} {i}", f_title.trim()),
                            severity.to_string(),
                            disposition.to_string(),
                            evidence,
                        )
                    })
                    .collect();
                // Re-set `verdict` to the *other* enum member so the edit always
                // changes a byte (clause 2 asserts exactly one line differs).
                let flipped = if verdict == "green" { "red" } else { "green" };
                (
                    "completion-record".to_string(),
                    build_completion_record(&title, verdict, &owner, &findings),
                    Some(("verdict".to_string(), flipped.to_string())),
                )
            });

        let dogfood_record = (
            scalar_value(),
            prop::sample::select(vec!["pilot", "existing-docs", "greenfield"]),
            "[0-9a-f]{7,40}",
            // The ten transcribed int facts — negatives included (the
            // stores-verbatim dialect bound).
            prop::collection::vec(-3i64..1000, 10),
            prop::sample::select(vec!["green", "red"]),
            owner_artifact_value(),
            section_prose(),
        )
            .prop_map(|(title, case, sha, ints, verdict, owner, judgment)| {
                let flipped = if verdict == "green" { "red" } else { "green" };
                (
                    "dogfood-record".to_string(),
                    build_dogfood_record(&title, case, &sha, &ints, verdict, &owner, &judgment),
                    Some(("verdict".to_string(), flipped.to_string())),
                )
            });

        let vision = (
            scalar_value(),
            // `grounded-in`: 0..3 research refs — the `0..` lower bound covers
            // BOTH the omitted-field empty header and the inline-flow list.
            prop::collection::vec("[a-z][a-z0-9]{0,12}", 0..3),
            section_prose(),
            section_prose(),
            section_prose(),
        )
            .prop_map(|(title, slugs, thesis, invariants, open_questions)| {
                let grounded: Vec<String> = slugs
                    .into_iter()
                    .enumerate()
                    .map(|(i, slug)| format!("research:{slug}-{i}"))
                    .collect();
                (
                    "vision".to_string(),
                    build_vision(&title, &grounded, &thesis, &invariants, &open_questions),
                    // The only header field is a ref-list (not a scalar
                    // `set_field` target): the surgical-edit clause self-skips.
                    None,
                )
            });

        let research = (
            scalar_value(),
            date_value(),
            section_prose(),
            section_prose(),
            section_prose(),
        )
            .prop_map(|(title, date, question, findings, sources)| {
                (
                    "research".to_string(),
                    build_research(&title, &date, &question, &findings, &sources),
                    // Re-set `date` to a year the generator never mints (< 2030),
                    // so the edit always changes a byte.
                    Some(("date".to_string(), "2031-01-15".to_string())),
                )
            });

        let idea = (
            scalar_value(),
            scalar_value(),
            date_value(),
            section_prose(),
        )
            .prop_map(|(title, trigger, date, description)| {
                (
                    "idea".to_string(),
                    build_idea(&title, &trigger, &date, &description),
                    Some(("trigger".to_string(), "a fresh de-park trigger".to_string())),
                )
            });

        let milestone_record = (
            "[a-z][a-z0-9-]{0,15}",
            ("[0-9a-f]{40}", "[0-9a-f]{7}"),
            prop::sample::select(vec!["active", "joined"]),
            // `0..` tasks covers the fresh-record EMPTY repeatable. The task-id
            // stem is hyphen-free: the `-{i}` suffix supplies the only hyphen,
            // since the anchor grammar rejects a non-canonical slug (`a--0`,
            // a trailing `-`).
            prop::collection::vec(
                (
                    "[a-z][a-z0-9]{0,11}",
                    scalar_value(),
                    prop::sample::select(vec!["active", "joined"]),
                ),
                0..3,
            ),
        )
            .prop_map(|(title, (sha, short), status, raw)| {
                // Index-suffix each task id so the `{#id}` anchors are distinct
                // (the task-id IS the anchor — no slugification).
                let tasks: Vec<(String, String, String)> = raw
                    .into_iter()
                    .enumerate()
                    .map(|(i, (task_id, intent, task_status))| {
                        (format!("{task_id}-{i}"), intent, task_status.to_string())
                    })
                    .collect();
                (
                    "milestone-record".to_string(),
                    build_milestone_record(&title, &format!("{sha} {short}"), status, &tasks),
                    // `base` is a plain string scalar — set it to a fresh pin
                    // (all-`a` SHA + short, never minted by the hex generator in
                    // practice; the clause-2 no-op guard covers a collision).
                    Some((
                        "base".to_string(),
                        format!("{} {}", "a".repeat(40), "a".repeat(7)),
                    )),
                )
            });

        // `planning-record` — header-less, fourteen prose slots. The slot COUNT is
        // read from the shipped schema, so a gate added to the doctype widens the
        // generated document rather than leaving the new slot un-sampled.
        let gate_count = schema_for("planning-record").sections.len();
        let planning_record = (
            scalar_value(),
            prop::collection::vec(section_prose(), gate_count..=gate_count),
        )
            .prop_map(|(title, prose)| {
                (
                    "planning-record".to_string(),
                    build_planning_record(&title, &prose),
                    // Header-less (the stamp is injected by the CLI loader, not the
                    // bare engine one): no settable scalar, so the surgical-edit
                    // clause self-skips — the `roadmap` precedent.
                    None,
                )
            });

        (
            prop_oneof![
                roadmap,
                deferral_ledger,
                decisions_log,
                completion_record,
                dogfood_record,
                vision,
                research,
                idea,
                milestone_record,
                planning_record,
            ],
            eol,
        )
            .prop_map(|((ty, canonical_lf, edit), eol)| GenDoc {
                ty,
                canonical_lf,
                eol,
                edit,
            })
    }

    /// Project a canonical-LF doc onto the generated EOL (EOL is preserved
    /// through the round-trip, never normalized).
    fn with_eol(canonical_lf: &str, eol: &str) -> String {
        if eol == "\n" {
            canonical_lf.to_string()
        } else {
            canonical_lf.replace('\n', "\r\n")
        }
    }

    /// Project a parsed document to its structural shape — per section: the id,
    /// the field keys, and the item `{#id}` anchors (recursively flattened one
    /// level; no methodology doctype nests deeper). Drives the clause-1
    /// LF-vs-CRLF structure-equality check.
    fn parsed_shape(doc: &crate::parse::Document) -> Vec<(String, Vec<String>, Vec<String>)> {
        doc.sections
            .iter()
            .map(|s| {
                (
                    s.id.clone(),
                    s.fields.iter().map(|f| f.key.clone()).collect(),
                    s.items.iter().map(|it| it.id.clone()).collect(),
                )
            })
            .collect()
    }

    proptest! {
        /// Clause 1 — **render∘parse == id** on the canonical LF form, and the
        /// **no-op write idempotent modulo the canonicalization ledger** across
        /// LF/CRLF. The generated doc (built through `render` from the shipped
        /// schema) must parse; re-deriving the [`Instance`] from the rendered
        /// bytes and re-rendering must be byte-identical; and the no-op write
        /// over the EOL-projected + BOM/trailing-newline-perturbed variant must
        /// canonicalize exactly the ledger deviations, EOL preserved.
        #[test]
        fn methodology_no_op_write_is_byte_identical_modulo_ledger(doc in arb_doc()) {
            let schema = schema_for(&doc.ty);

            // render∘parse == id: the parse→Instance→render inverse reproduces
            // the canonical LF bytes exactly.
            let reparsed = match instance_from_source(&schema, &doc.canonical_lf) {
                Ok(instance) => instance,
                Err(findings) => {
                    return Err(TestCaseError::fail(format!(
                        "generated {} doc must parse: {findings:?}\n--- doc ---\n{}",
                        doc.ty, doc.canonical_lf
                    )));
                }
            };
            prop_assert_eq!(
                render(&schema, &reparsed),
                doc.canonical_lf.clone(),
                "render∘parse == id for {}",
                doc.ty.clone()
            );

            // The EOL-projected form still parses (CRLF is a first-class input) —
            // and to the SAME structure: same sections, same field keys, same
            // item anchors. Parse-ok alone is not enough: the CRLF metadata-scan
            // defect this census surfaced DROPPED every front-matter field after
            // the first while still parsing "ok".
            let canonical = with_eol(&doc.canonical_lf, &doc.eol);
            let eol_doc = match parse_sections(&schema, &canonical) {
                Ok(parsed) => parsed,
                Err(findings) => {
                    return Err(TestCaseError::fail(format!(
                        "generated {} doc must parse under {:?} EOL: {findings:?}",
                        doc.ty, doc.eol
                    )));
                }
            };
            let lf_doc = parse_sections(&schema, &doc.canonical_lf)
                .expect("the canonical LF form parsed above");
            prop_assert_eq!(
                parsed_shape(&lf_doc),
                parsed_shape(&eol_doc),
                "the EOL projection must not change the parsed structure of a {} doc",
                doc.ty.clone()
            );

            // The no-op write of the already-canonical doc is byte-identical.
            prop_assert_eq!(first_touch_canonicalize(&canonical), canonical.clone());

            // Perturb with the two ledger-covered deviations: a leading BOM and
            // extra trailing newlines. The no-op write must canonicalize *only*
            // those back — nothing line-spanning touched.
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

        /// Clause 2 — **surgical on edits.** Where the doctype carries a settable
        /// `meta` front-matter scalar, a single `set_field` changes *only* that
        /// field's value bytes: the prefix and suffix survive byte-identical, the
        /// EOL is preserved, and the new value is in place.
        #[test]
        fn methodology_single_field_edit_changes_only_that_field(doc in arb_doc()) {
            let Some((key, new_value)) = doc.edit.clone() else { return Ok(()); };
            let schema = schema_for(&doc.ty);
            let source = with_eol(&doc.canonical_lf, &doc.eol);
            // Skip the no-op edit case (new value equals the existing one): there
            // is no byte to change, so "exactly one line differs" would not hold.
            if source.contains(&format!("{key}: {new_value}")) {
                return Ok(());
            }

            // Every methodology header section is `meta`.
            let edited = set_field(&schema, &source, "meta", &key, &new_value)
                .expect("present front-matter field is settable");

            // EOL preserved.
            if doc.eol == "\r\n" {
                prop_assert!(edited.contains("\r\n"), "EOL must survive the edit");
            }

            // Only the target field's value line differs.
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

    // ========================================================================
    // The DETERMINISTIC fence (the proptests above are the finder).
    //
    // `cargo test` runs the two `proptest!` blocks over RANDOM inputs, so a green
    // gate means "no counterexample was sampled this run" — not "the invariant
    // holds" (dev-workflow → Gate: *A proptest is not a gate*; the M41 `slugify`
    // idempotency break). The #1-risk round-trip over the NINE shipped methodology
    // schemas was proptest-only here, with deterministic backup for just four
    // doctypes and only externally (`crates/cli/tests/migrate_methodology.rs`).
    //
    // The table below is that fence, uniform across all nine and sitting beside the
    // invariant it guards. One row per canonical fixture, hand-authored in the
    // frozen byte form the writer emits; a future doctype is one row. Per row:
    //   1. `render(parse(src)) == src` — byte-identical, plus an insta golden of
    //      the rendered bytes so a canonical-form drift reads as a reviewable diff.
    //   2. The **absent** arm of every optional shape — `vision.grounded-in` at
    //      zero refs (the empty `---\n---` fence pair) and the EMPTY repeatable
    //      (the fresh-mint shape) for all five repeatable-bearing doctypes. The
    //      un-sampled shapes.
    //   3. ≥2 items in every populated repeatable, so item ordering and `{#id}`
    //      anchor emission are pinned (both `roadmap` leaves filled; both
    //      `deferral-ledger.kind` enum members exercised).
    //   4. The canonicalization ledger (BOM + doubled trailing newline) and the
    //      LF/CRLF **parsed-shape equality** — the only guard on the front-matter
    //      field-drop class for these nine types (`parse.rs`'s deterministic
    //      `crlf_front_matter_parses_every_field` pins `adr` alone).
    // ========================================================================

    /// One canonical fixture: a doctype, a snapshot-stable name, and the exact
    /// frozen bytes.
    struct Fixture {
        /// Names the insta golden and the failing row.
        name: &'static str,
        /// The methodology doctype id — selects the shipped schema.
        ty: &'static str,
        /// The canonical document bytes, hand-authored in the writer's frozen form
        /// (`---` fences, `# H1`, `## Section`, `### Item  {#id}` with the two-space
        /// anchor gap, `<!-- fields -->`-sentinelled field groups, exactly one
        /// trailing `\n`).
        src: &'static str,
    }

    /// The fixture table — every shipped persisted methodology doctype, each in its
    /// populated form and (where it has one) its absent-optional / empty-repeatable
    /// form.
    fn methodology_fixtures() -> Vec<Fixture> {
        vec![
            // `roadmap` — the multi-slot repeatable item: each milestone renders
            // BOTH leaves (`#### Proves` / `#### Decomposition`) as sub-headings.
            Fixture {
                name: "roadmap_two_milestones",
                ty: "roadmap",
                src: "\
# Roadmap

## Milestones

### Milestone 41  {#milestone-41}

#### Proves

The rc.5 wave closes the holes the adoption trial surfaced.

#### Decomposition

Nine increments: fold-safe templates, the composed task id, the driver contract.

### Milestone 42  {#milestone-42}

#### Proves

The rc.6 wave closes the store-validate and abandon-path holes.

#### Decomposition

Six clusters, one increment each, risk-first.
",
            },
            // The fresh-singleton EMPTY repeatable: a bare `## Milestones`.
            Fixture {
                name: "roadmap_empty",
                ty: "roadmap",
                src: "\
# Roadmap

## Milestones
",
            },
            // `deferral-ledger` — slot-then-fields items; both `kind` enum members
            // (`Decision` / `Idea`, the M41 F4 value-remap targets) exercised.
            Fixture {
                name: "deferral_ledger_two_entries",
                ty: "deferral-ledger",
                src: "\
# Deferral Ledger

## Entries

### Abandon path  {#abandon-path}

The milestone abandon path was never designed, so a committed record can lie.

<!-- fields -->
- kind: Decision
- trigger: M42
- date: 2026-07-12

### Store validate  {#store-validate}

A schema-version-aware store validate for placement doctypes, parked until asked for.

<!-- fields -->
- kind: Idea
- trigger: M43
- date: 2026-07-12
",
            },
            Fixture {
                name: "deferral_ledger_empty",
                ty: "deferral-ledger",
                src: "\
# Deferral Ledger

## Entries
",
            },
            // `decisions-log` — slot-then-one-field items.
            Fixture {
                name: "decisions_log_two_entries",
                ty: "decisions-log",
                src: "\
# Decisions Log

## Entries

### Deterministic fences  {#deterministic-fences}

A proptest samples; a point test enforces. Both ship, and the point test is the gate.

<!-- fields -->
- date: 2026-07-11

### Frozen schemas  {#frozen-schemas}

The methodology pack is manifest-governed, version-gated exactly like the dev pack.

<!-- fields -->
- date: 2026-07-12
",
            },
            Fixture {
                name: "decisions_log_empty",
                ty: "decisions-log",
                src: "\
# Decisions Log

## Entries
",
            },
            // `completion-record` — a two-field header plus FIELDS-ONLY (slotless)
            // items: the heading is followed directly by the field group.
            Fixture {
                name: "completion_record_two_findings",
                ty: "completion-record",
                src: "\
---
verdict: green
owner-artifact: completions/artifacts/M41/
---

# M41

## Findings

### Ref key  {#ref-key}

<!-- fields -->
- severity: advisory
- disposition: fixed
- evidence: crates/engine/src/validate.rs

### Slot fidelity  {#slot-fidelity}

<!-- fields -->
- severity: blocking
- disposition: contested
- evidence: crates/cli/tests/migrate_methodology.rs
",
            },
            // The clean-audit EMPTY repeatable, header still populated.
            Fixture {
                name: "completion_record_clean_audit",
                ty: "completion-record",
                src: "\
---
verdict: green
owner-artifact: completions/artifacts/M40/
---

# M40

## Findings
",
            },
            // `dogfood-record` — the census's widest header (14 front-matter fields),
            // then a prose slot. The field-drop class's loudest CRLF canary.
            Fixture {
                name: "dogfood_record_greenfield",
                ty: "dogfood-record",
                src: "\
---
case: greenfield
binary-sha: 1492317
adapter-writes: 412
oob-edits: 7
drift-caught: 5
validate-blocks: 3
halts-expected: 2
halts-unplanned: 1
fix-rounds: 4
audit-findings: 6
seeded-oob: 1
seeded-blocks: 1
verdict: green
owner-artifact: dogfood/artifacts/greenfield-one/
---

# Greenfield run one

## Judgment

The loop held: every seeded out-of-band edit was caught and every seeded block fired.

The one unplanned halt was a route-less advisory, not a determinism break.
",
            },
            // `vision` — the ref-list header PRESENT: an inline-flow `[a, b]` value.
            Fixture {
                name: "vision_grounded",
                ty: "vision",
                src: "\
---
grounded-in: [research:context-compilers, research:agent-drift]
---

# Vision

## Thesis

Take every structural operation away from the LLM and give it to the CLI.

Leave the LLM only the prose.

## Invariants

The CLI core makes no LLM calls, and the LLM writes only through the CLI.

## Open Questions

Whether a third-party pack-authoring API ever earns its keep.
",
            },
            // The ABSENT-optional arm: `grounded-in` is `card: 0..*`, so zero refs
            // OMIT the field entirely — an empty `---\n---` fence pair. The classic
            // un-sampled shape, and the reason this arm is spelled out.
            Fixture {
                name: "vision_ungrounded",
                ty: "vision",
                src: "\
---
---

# Vision

## Thesis

Take every structural operation away from the LLM and give it to the CLI.

## Invariants

The CLI core makes no LLM calls.

## Open Questions

Whether a third-party pack-authoring API ever earns its keep.
",
            },
            // `research` — one header field, three prose slots.
            Fixture {
                name: "research_context_compilers",
                ty: "research",
                src: "\
---
date: 2026-07-05
---

# Context compilers

## Question

What do the existing context tools actually assemble, and when?

## Findings

None assemble just-in-time from a managed corpus; all of them ship static rules.

## Sources

The GSD harness, Spec-Kit, and the Cursor rules format.
",
            },
            // `idea` — two header fields, one prose slot.
            Fixture {
                name: "idea_multi_language_doc_code",
                ty: "idea",
                src: "\
---
trigger: M43
date: 2026-07-12
---

# Multi language doc code

## Description

The anchor gate is symbol-blind outside Rust. The tier-one slice is the Vue SFC arm.
",
            },
            // `milestone-record` — `id-from: task-id`, so the item heading text IS
            // the task-id slug (not a slugified prose title) and the id-source field
            // is elided from the field group, exactly as `title` is elsewhere.
            Fixture {
                name: "milestone_record_two_tasks",
                ty: "milestone-record",
                src: "\
---
base: 1492317c0ffee1234567890abcdef1234567890
status: active
---

# M42

## Tasks

### fence-roundtrip  {#fence-roundtrip}

<!-- fields -->
- intent: Fence the methodology round-trip with deterministic cases.
- status: joined

### fence-slug  {#fence-slug}

<!-- fields -->
- intent: Pin the slug composition deterministically.
- status: active
",
            },
            // The fresh-record EMPTY repeatable (a milestone minted before its first
            // sub-task is spawned).
            Fixture {
                name: "milestone_record_fresh",
                ty: "milestone-record",
                src: "\
---
base: a3c26dcfeedfacefeedfacefeedfacefeedface0
status: active
---

# M43

## Tasks
",
            },
            // `planning-record` — fourteen prose slots, one per planning gate, and no
            // header at all (the schema-version stamp is CLI-injected, not engine-side).
            // The gate ids are hyphenated, so this row also pins the multi-word
            // `## <Heading Text>` rendering for a slot section.
            Fixture {
                name: "planning_record_m49",
                ty: "planning-record",
                src: "\
# M49

## Reuse Exercised

Driven: `grep -rn \"FileStateRecord\"` over the cache seam, then the cold-start form on a fresh clone.

## Cheap Vs Robust

The cheap cut leaves a hole in a declared surface, so the robust path is taken; the advocate's case is in the settle record.

## Foreclosed By Doc

`team-ready-state.md`'s exclusion rests on a predicate, and the predicate was driven rather than obeyed.

## Prior Art Reconciled

`grep -rn \"planning gate\" design/` — 4 hits, all one home, no contradiction.

## Census

`grep -rn \"required-slot-present\" crates/` — 11 hits; enforced at the conformance seam, not on a list.

## Integration Seam

The projection feeds `doc schema`, whose input precondition is a loaded schema; the new sections satisfy it.

## Check Scope Pinned

`schema-conformance.required-slot-present`, task and finalize scope.

## Design Complete

Behaviour, finding id, severity and acceptance are all settled in the record.

## Acceptance Spiked

The composed projection and the `--format json` read were both spiked on the real binary.

## Value Flow Exercised

N/A — this milestone builds no new end-to-end flow; the baseline walk stands.

## Deliverable Reachable

Reachable at T6, when the planning workflow's create-gate names the doctype.

## Strategic Claim Fresh

The justification was re-read against the latest trial record, not the charter.

## Quote Attributed

`heading_text` at `crates/engine/src/write.rs` is the producer of every rendered heading quoted here.

## Claim Driven

Driven by the orchestrator: every count in this record came from a command whose output is quoted.
",
            },
        ]
    }

    /// **The fence.** For every canonical fixture: `render(parse(src)) == src`,
    /// byte-identical — the #1-risk round-trip, deterministically, over the shipped
    /// methodology schemas. The insta golden pins the rendered bytes so a
    /// canonical-form drift reads as a diff before the equality assert names it.
    #[test]
    fn methodology_canonical_fixtures_round_trip_byte_identically() {
        for fx in methodology_fixtures() {
            let schema = schema_for(fx.ty);
            let instance = instance_from_source(&schema, fx.src).unwrap_or_else(|findings| {
                panic!(
                    "the canonical {} fixture {:?} must parse: {findings:?}",
                    fx.ty, fx.name
                )
            });
            let rendered = render(&schema, &instance);
            insta::assert_snapshot!(format!("methodology_canonical_{}", fx.name), rendered);
            assert_eq!(
                rendered, fx.src,
                "render(parse(x)) == x must hold byte-identically for the {} fixture {:?}",
                fx.ty, fx.name
            );
        }
    }

    /// **The ledger + CRLF fence.** For every canonical fixture, under BOTH line
    /// endings: the no-op write of the already-canonical doc is byte-identical, and
    /// a BOM + doubled-trailing-newline perturbation canonicalizes back to exactly
    /// the canonical form (EOL preserved — a CRLF doc never normalizes to LF).
    ///
    /// Then the clause a parse-ok check cannot make: the LF and CRLF forms parse to
    /// the SAME structure — same sections, same field keys, same item anchors. This
    /// is the only deterministic guard on the front-matter field-drop class for
    /// these nine doctypes (the CRLF metadata-scan defect the M40 census surfaced
    /// dropped every front-matter field after the first while still parsing "ok" —
    /// `dogfood_record_greenfield`'s 14-field header is the loud canary).
    #[test]
    fn methodology_canonical_fixtures_canonicalize_from_the_ledger_deviations() {
        for fx in methodology_fixtures() {
            let schema = schema_for(fx.ty);

            for eol in ["\n", "\r\n"] {
                let canonical = with_eol(fx.src, eol);

                assert_eq!(
                    first_touch_canonicalize(&canonical),
                    canonical,
                    "the no-op write of the canonical {} fixture {:?} must be byte-identical under {eol:?}",
                    fx.ty,
                    fx.name
                );

                let perturbed = format!("\u{feff}{canonical}{eol}{eol}");
                assert_eq!(
                    first_touch_canonicalize(&perturbed),
                    canonical,
                    "the ledger deviations (BOM, doubled trailing newline) must canonicalize back to the canonical {} fixture {:?} under {eol:?}",
                    fx.ty,
                    fx.name
                );
            }

            let lf = parse_sections(&schema, fx.src).unwrap_or_else(|f| {
                panic!("the {} fixture {:?} parses under LF: {f:?}", fx.ty, fx.name)
            });
            let crlf_src = with_eol(fx.src, "\r\n");
            let crlf = parse_sections(&schema, &crlf_src).unwrap_or_else(|f| {
                panic!(
                    "the {} fixture {:?} parses under CRLF: {f:?}",
                    fx.ty, fx.name
                )
            });
            assert_eq!(
                parsed_shape(&lf),
                parsed_shape(&crlf),
                "the EOL projection must not change the parsed structure of the {} fixture {:?} \
                 (the front-matter field-drop class)",
                fx.ty,
                fx.name
            );
        }
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

    /// A canonical `spec` instance over the shipped schema: the empty `---\n---`
    /// header (the `meta` section carries an optional `derived-from → prd` ref, card
    /// `0..1`; a PRD-less spec omits it → an empty fence pair, the production shape),
    /// the `# H1` title (the id-source — `spec` carries NO `title` field, like `adr`),
    /// `goal` + `context` prose slots, and two `criteria` items each with a frozen
    /// `{#id}` anchor and a `statement` slot. Authored in the exact frozen byte form
    /// the canonical writer emits (`---\n---`, `# H1`, `## …` slots, `### …  {#id}`
    /// items with the two-space anchor gap).
    fn canonical_spec_source() -> &'static str {
        "\
---
---

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
---
---

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
                fields: vec![scalar("value", v)],
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
        let minted = add_item(
            &schema,
            &empty,
            "milestones",
            "M16 self-hosting",
            None,
            None,
            &[],
        )
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
        let minted = add_item(&schema, &empty, "milestones", "Alpha", None, None, &[])
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

    /// A nested `add-item` with a title carrying the `{#` anchor pattern is rejected
    /// as [`GenerateError::WrongShape`] — the same anchor-injection reject as
    /// top-level [`add_item`], through the nested mint path (M40 blocking finding).
    #[test]
    fn nested_add_item_anchor_syntax_title_is_rejected() {
        let schema = changelog_schema();
        let err = add_nested_item(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-2-0"],
            "changes",
            "Evil {#added} group",
            None,
            None,
            &[],
        )
        .expect_err("an anchor-carrying nested title must not mint");
        assert!(matches!(err, GenerateError::WrongShape { .. }));
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

    /// The `--slug` override reaches the **nested** mint too, through the same shared
    /// derivation: the supplied anchor lands at the nested heading depth, scoped to the
    /// addressed parent, and the result round-trips.
    #[test]
    fn nested_add_item_slug_override_drives_the_anchor_verbatim() {
        let schema = changelog_schema();
        let out = add_nested_item(
            &schema,
            TWO_PARENT_TWO_LEVEL,
            "releases",
            &["1-2-0"],
            "changes",
            "Changed",
            Some("changed-under-1-2-0"),
            None,
            &[],
        )
        .expect("nested add-item with an override");
        assert!(
            out.contains("#### Changed  {#changed-under-1-2-0}"),
            "the nested anchor is the override; got:\n{out}",
        );
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
            None,
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

#[cfg(test)]
mod retitle_item_tests {
    //! M40 Increment 2, T1 — [`retitle_item`]: the retitle-without-reslug verb at item
    //! level. The heading-title bytes change; the `{#id}` anchor and EVERY other byte
    //! stay identical (the expected buffer is computed by replacing exactly the heading
    //! line, so the assertion is over the whole document — never a scoped subtree).

    use super::*;
    use crate::schema::Schema;

    const ARCH_DOC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/arch-doc.yaml");

    fn arch_doc_schema() -> Schema {
        crate::schema::load_schema_with_types(ARCH_DOC_YAML, &crate::schema::dev_pack_field_types())
            .expect("arch-doc.yaml loads")
    }

    /// The `changelog`-shaped two-level schema with a **string** nested id-source
    /// (`category`, free text) — the nested happy path.
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

    /// The same shape with an **enum** nested id-source — the id-from type-check arm.
    fn enum_changelog_schema() -> Schema {
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
              - { id: category, type: enum, of: [added, fixed] }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";
        crate::schema::load_schema(yaml).expect("enum changelog schema loads")
    }

    /// A canonical committed arch-doc with two components — the shipped doctype the
    /// retitle verb is minted for (a component heading gone stale against its symbol).
    const ARCH_DOC: &str = "\
---
cites: adr:0001-storage-layout
---

# Storage layer

## Overview

The storage layer owns the on-disk task working areas.

## Components

### Working area  {#working-area}

Owns the per-task scratch tree.

<!-- fields -->
- implemented-by: src/storage.rs#WorkingArea

### Join merge  {#join-merge}

Merges fan-out results by task id.

<!-- fields -->
- implemented-by: src/join.rs#merge
";

    /// Two releases, EACH carrying a `#added` nested change-group — the parent-scoped
    /// same-anchor fixture (`1-3-0` first, `1-2-0` second).
    const TWO_PARENT: &str = "\
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

    /// Round-trip helper: `render(instance_from_source(x)) == x` over the WHOLE doc.
    fn assert_byte_stable(schema: &Schema, src: &str) {
        let instance = instance_from_source(schema, src)
            .unwrap_or_else(|f| panic!("source parses: {f:?}\n--- src ---\n{src}"));
        assert_eq!(
            render(schema, &instance),
            src,
            "render(parse(x)) must equal x"
        );
    }

    /// The fixtures are themselves canonical, so whole-document byte comparisons hold.
    #[test]
    fn fixtures_are_canonical() {
        assert_byte_stable(&arch_doc_schema(), ARCH_DOC);
        assert_byte_stable(&changelog_schema(), TWO_PARENT);
    }

    /// Done-criterion (1): a TOP-LEVEL arch-doc component retitle changes exactly the
    /// heading-title bytes — the `{#working-area}` anchor and every other byte
    /// identical (whole-document equality against a replace-only expected buffer) —
    /// and the result round-trips byte-stable.
    #[test]
    fn top_level_component_retitle_changes_only_the_heading_title_bytes() {
        let schema = arch_doc_schema();
        let out = retitle_item(
            &schema,
            ARCH_DOC,
            "components",
            &["working-area"],
            "WorkingArea store",
        )
        .expect("top-level retitle succeeds");

        // The expected buffer differs from the source in exactly the heading line.
        let expected = ARCH_DOC.replace(
            "### Working area  {#working-area}",
            "### WorkingArea store  {#working-area}",
        );
        assert_ne!(out, ARCH_DOC, "the retitle must change bytes");
        assert_eq!(
            out, expected,
            "only the heading-title bytes change; anchor + all other bytes identical"
        );

        // The frozen anchor still resolves the item.
        locate_item_path(&schema, &out, "components", &["working-area"])
            .expect("the item is still addressable by its frozen anchor");

        // render(parse(out)) == out.
        assert_byte_stable(&schema, &out);
    }

    /// Done-criterion (2): a NESTED chain-addressed item retitles the same way — the
    /// section-qualified chain (`1-2-0/changes/added`) lands on exactly the addressed
    /// item, the same-anchor `#added` under the OTHER parent stays byte-untouched, and
    /// the result round-trips byte-stable.
    #[test]
    fn nested_chain_addressed_retitle_freezes_the_anchor() {
        let schema = changelog_schema();
        let out = retitle_item(
            &schema,
            TWO_PARENT,
            "releases",
            &["1-2-0", "changes", "added"],
            "Added (auth)",
        )
        .expect("nested retitle succeeds");

        // Expected: exactly the SECOND `#### Added  {#added}` heading (1-2-0's — the
        // later one in the fixture) rewritten; 1-3-0's same-anchor heading untouched.
        let heading = "#### Added  {#added}";
        let pos = TWO_PARENT.rfind(heading).expect("1-2-0's heading present");
        let mut expected = String::new();
        expected.push_str(&TWO_PARENT[..pos]);
        expected.push_str("#### Added (auth)  {#added}");
        expected.push_str(&TWO_PARENT[pos + heading.len()..]);
        assert_eq!(
            out, expected,
            "only the addressed nested heading's title bytes change"
        );

        assert_byte_stable(&schema, &out);
    }

    /// Done-criterion (3): an absent item — top-level, nested, or under a mis-named
    /// parent — and an unknown section both error, never a wrong-item write.
    ///
    /// **M47 — the three carry *different* errors, and that is the point.** An absent
    /// item is an item-**id** miss ([`GenerateError::NotPresent`], routed to the
    /// containing section's live ids); an **undeclared section** is
    /// [`GenerateError::UnknownSection`] → `write.unknown-section`, the schema read that
    /// answers it; [`GenerateError::WrongShape`] is reserved for a *declared* section of
    /// the wrong shape. Before the flip all three were `WrongShape`, which is why this
    /// test could not tell them apart; the undeclared-section arm was **revised from
    /// `WrongShape` to `UnknownSection`** when the undeclared-**section** column of the
    /// verb × miss-shape matrix was swept ([`section_undeclared`], rank 1) — the code the
    /// `write.*` route split has always specified for it
    /// (`design/validation.md`; `crates/cli/tests/write_miss_shape_axis.rs`).
    #[test]
    fn absent_item_or_section_errors_with_the_existing_shapes() {
        let schema = changelog_schema();
        // Absent top-level item — an item-id miss.
        let err = retitle_item(&schema, TWO_PARENT, "releases", &["9-9-9"], "New")
            .expect_err("no such release");
        assert!(matches!(err, GenerateError::NotPresent { .. }));
        // Absent nested item under a present parent — also an item-id miss.
        let err = retitle_item(
            &schema,
            TWO_PARENT,
            "releases",
            &["1-2-0", "changes", "removed"],
            "New",
        )
        .expect_err("no such nested group");
        assert!(matches!(err, GenerateError::NotPresent { .. }));
        // Undeclared section — the schema can answer it, so it is its own code.
        let err = retitle_item(&schema, TWO_PARENT, "nope", &["1-2-0"], "New")
            .expect_err("no such section");
        assert!(matches!(err, GenerateError::UnknownSection { .. }));
        // An **undeclared nested hop** under a declared section is the same declaredness
        // miss one level down, so it earns the same code (M49 — the nested arm of the
        // undeclared-section column; [`nested_section_undeclared`]). This assertion read
        // `WrongShape` until M49, which is exactly the four-codes-for-one-miss state the
        // sweep closed — moved as a basis-has-changed rebuttal, not an override.
        let err = retitle_item(
            &schema,
            TWO_PARENT,
            "releases",
            &["1-2-0", "nope", "x"],
            "New",
        )
        .expect_err("no such nested repeatable");
        assert!(matches!(err, GenerateError::UnknownSection { .. }));
        // The bound of that arm, driven rather than asserted in prose: a nested segment
        // the block **does** declare — as a field, not a repeatable — is a genuine
        // declared-shape defect and keeps `WrongShape`, exactly as a declared but
        // non-repeatable *section* does one level up.
        let err = retitle_item(
            &schema,
            TWO_PARENT,
            "releases",
            &["1-2-0", "date", "x"],
            "New",
        )
        .expect_err("`date` is a field, not a nested repeatable");
        assert!(matches!(err, GenerateError::WrongShape { .. }));
    }

    /// **Item presence outranks the id-from re-validation** (M47 — the write-verb ×
    /// item-id-miss axis). The deterministic pin for the ordering: the sibling above runs
    /// over the **string**-id-from fixture, whose `check_value` accepts any title, so it
    /// cannot witness the rank at all — under the **enum** fixture (the shape the shipped
    /// dev pack's `changelog` actually declares) the re-validation used to answer first,
    /// telling the caller what a nonexistent item's id derives from and routing at a
    /// `set-field` that blocks on the same absence. Both an absent top-level item and an
    /// absent nested one under a real parent must reach [`GenerateError::NotPresent`],
    /// with a member title *and* a non-member one — the value question is not asked until
    /// there is an item to ask it of.
    #[test]
    fn an_absent_item_outranks_the_enum_id_from_revalidation() {
        let schema = enum_changelog_schema();
        for (item_ids, title) in [
            (["9-9-9"].as_slice(), "Fixed"),
            (["9-9-9"].as_slice(), "Rewritten"),
            (["1-2-0", "changes", "removed"].as_slice(), "Fixed"),
            (["1-2-0", "changes", "removed"].as_slice(), "Rewritten"),
        ] {
            let err = retitle_item(&schema, TWO_PARENT, "releases", item_ids, title)
                .expect_err("no such item");
            assert!(
                matches!(err, GenerateError::NotPresent { .. }),
                "{item_ids:?} + {title:?} is an item-id miss, got {err:?}"
            );
        }
    }

    /// The id-from re-validation: the heading IS the id-source field's value, so a new
    /// title failing the declared type — a non-member against an ENUM id-from — is
    /// rejected as [`GenerateError::MalformedValue`] before any bytes move (the
    /// [`set_nested_item_field_or_insert`] guard shape).
    #[test]
    fn enum_id_from_rejects_a_non_member_title_as_malformed() {
        let schema = enum_changelog_schema();
        let err = retitle_item(
            &schema,
            TWO_PARENT,
            "releases",
            &["1-2-0", "changes", "added"],
            "Rewritten",
        )
        .expect_err("a non-member title against an enum id-source is malformed");
        assert!(matches!(err, GenerateError::MalformedValue { .. }));
    }

    /// An empty / whitespace-only title is rejected (it would emit a titleless heading
    /// whose round-trip is undefined), with the existing [`GenerateError::WrongShape`].
    #[test]
    fn empty_title_is_rejected() {
        let schema = arch_doc_schema();
        let err = retitle_item(&schema, ARCH_DOC, "components", &["working-area"], "  ")
            .expect_err("an empty title is rejected");
        assert!(matches!(err, GenerateError::WrongShape { .. }));
    }

    /// A new title carrying the `{#` anchor pattern is rejected as
    /// [`GenerateError::WrongShape`] before any bytes move — spliced, the parser would
    /// re-read the heading's FIRST `{#…}` as the item's identity, so
    /// `### Evil {#other} title  {#frozen}` silently reslug-hijacks the frozen anchor
    /// (every inbound address to `{#frozen}` dangles) and breaks
    /// `render(parse(out)) == out`. Titles are LLM-owned prose, so this input class is
    /// agent-reachable (M40 blocking finding: the anchor-injection hole).
    #[test]
    fn anchor_syntax_title_is_rejected() {
        let schema = arch_doc_schema();
        let err = retitle_item(
            &schema,
            ARCH_DOC,
            "components",
            &["working-area"],
            "Evil {#other-anchor} title",
        )
        .expect_err("an anchor-carrying title must not splice");
        assert!(matches!(err, GenerateError::WrongShape { .. }));
    }

    /// A new title carrying a control character (newline/tab) is rejected as
    /// [`GenerateError::WrongShape`] before any bytes move — even on a schema whose
    /// repeatable declares NO block leaf matching its `id-from` (legal: the loader
    /// never cross-checks `Repeatable.id_from` against the block), where the id-from
    /// `check_value` re-validation has no field to fire against. Without the shared
    /// [`reject_malformed_title`] floor, `"A\nB"` would splice a TWO-LINE heading whose
    /// second line carries the frozen `{#id}` anchor as prose — identity lost,
    /// `render(parse(out)) == out` broken (M40 completion finding: the
    /// control-char-title hole, retitle side).
    #[test]
    fn control_char_title_is_rejected_without_declared_id_from() {
        let yaml = b"\
type: notes
id-from: title
sections:
  - id: entries
    repeatable:
      id-from: heading
      block:
        - { id: body, slot: { hint: \"The note body.\" } }
";
        let schema = crate::schema::load_schema(yaml).expect("undeclared-id-from schema loads");
        let src = "\
# Notes

## Entries

### First note  {#first-note}

Body text.
";
        for title in ["A\nB", "A\tB"] {
            let err = retitle_item(&schema, src, "entries", &["first-note"], title)
                .expect_err("a control-char title must not splice a multi-line heading");
            assert!(
                matches!(err, GenerateError::WrongShape { .. }),
                "title {title:?} should be WrongShape, got {err:?}"
            );
        }
    }
}

/// T2 (M34 Inc-2): the net-new `fixed-slot→repeatable-with-default` primitive
/// ([`promote_slot_to_repeatable`]). Exercised **only** here (no existing test is
/// touched). The two contract properties the done-criterion names: (a) the promoted
/// output round-trips **byte-identical** (`render(parse(out)) == out`), and (b) the old
/// slot prose is preserved **verbatim** as the default first item (no data loss).
#[cfg(test)]
mod promote_slot_to_repeatable_tests {
    use super::*;
    use proptest::prelude::*;

    /// v1: `requirements` is a **simple slot**, sitting between two other slot sections
    /// (so the splice is exercised on a NON-trailing section — a following `## ` boundary
    /// to re-attach the separator past, not the EOF shortcut).
    fn v1_schema() -> Schema {
        crate::schema::load_schema(
            b"\
type: t
sections:
  - id: overview
    slot: { hint: \"the overview\" }
  - id: requirements
    slot: { hint: \"the requirements prose\" }
  - id: notes
    slot: { hint: \"trailing notes\" }
",
        )
        .expect("v1 schema loads")
    }

    /// v2: `requirements` is now a **single-slot repeatable** (`title` id-from heading +
    /// one `statement` slot — the reconstructed M25 `prd.requirements` reshape). The two
    /// neighbour sections are byte-identical to v1, so the only diff the splice may
    /// introduce is in the `requirements` region.
    fn v2_schema() -> Schema {
        crate::schema::load_schema(
            b"\
type: t
sections:
  - id: overview
    slot: { hint: \"the overview\" }
  - id: requirements
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one requirement\" } }
  - id: notes
    slot: { hint: \"trailing notes\" }
",
        )
        .expect("v2 schema loads")
    }

    /// A canonical v1-shaped document whose `requirements` section carries `prose`.
    /// Built through [`render`] so the input is exactly the byte-stable canonical form a
    /// first-touch-canonicalized corpus doc has.
    fn v1_doc(prose: &str) -> String {
        let inst = Instance {
            title: "Doc".to_string(),
            sections: vec![
                SectionContent {
                    id: "overview".to_string(),
                    slot: Some("An overview paragraph.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "requirements".to_string(),
                    slot: Some(prose.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "notes".to_string(),
                    slot: Some("Some trailing notes.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&v1_schema(), &inst)
    }

    /// Conformant, opaque slot prose: paragraphs with interior blank lines and prose
    /// that *looks* structural but is not at this depth (a fenced `## not a heading`, a
    /// `- x:` non-field bullet line, a `#### deeper` heading) — the stress that makes the
    /// round-trip assertion meaningful rather than trivial.
    fn prose() -> impl Strategy<Value = String> {
        let para = "[a-zA-Z][a-zA-Z0-9 .,]{0,30}";
        let fragment = prop_oneof![
            para,
            Just("```\n## not a heading\n```".to_string()),
            Just("- x: this is prose, not a field".to_string()),
            Just("#### a deeper heading is allowed".to_string()),
            "[a-zA-Z][a-zA-Z0-9 .,]{0,30}".prop_map(|s: String| format!("{s}\n\n\n{s}")),
        ];
        prop::collection::vec(fragment, 1..4).prop_map(|frags| frags.join("\n\n"))
    }

    /// A title that slugs to a non-empty `{#id}` anchor. The regex alone can emit a
    /// single edge-stopword ("The"), which the M41 edge-stopword drop slugs to the
    /// empty string — `promote_slot_to_repeatable` then refuses with the typed
    /// `UnslugableTitle`, and the property's unconditional `.expect` reddens
    /// nondeterministically (observed 2026-07-24 as a false mutant catch). The filter
    /// makes the generator match its own contract: promotion-success is the invariant
    /// under test, and it holds for sluggable titles; the unslugable refusal is pinned
    /// separately (`add_item_unslugable_title_is_rejected`).
    fn title() -> impl Strategy<Value = String> {
        "[A-Z][a-z]{2,8}( [A-Z][a-z]{2,8}){0,2}"
            .prop_filter("title must slug to a non-empty {#id} anchor", |t| {
                !crate::slug::slugify(t).is_empty()
            })
    }

    proptest! {
        /// (a) byte-stable round-trip + (b) verbatim no-data-loss, over arbitrary prose
        /// and titles. The promoted doc must (a) satisfy `render(parse(out)) == out`
        /// under the v2 schema, and (b) re-parse to a `requirements` section that is now
        /// a single item whose slot prose is **identical** to the v1 slot prose and whose
        /// title is the supplied one — and v1's section-level slot is gone.
        #[test]
        fn promotion_is_byte_stable_and_lossless(prose in prose(), title in title()) {
            let v1 = v1_schema();
            let v2 = v2_schema();
            let src = v1_doc(&prose);

            // The canonical v1 slot prose (the parser's verbatim slice) — the exact bytes
            // the promotion must carry into the default first item.
            let v1_inst = instance_from_source(&v1, &src).expect("v1 doc conforms");
            let v1_slot = v1_inst
                .sections
                .iter()
                .find(|s| s.id == "requirements")
                .and_then(|s| s.slot.clone());

            let out = promote_slot_to_repeatable(&v1, &v2, &src, "requirements", &title)
                .expect("promotion succeeds");

            // (a) Byte-stable round-trip under the NEW schema.
            let out_inst = instance_from_source(&v2, &out).expect("promoted doc conforms to v2");
            prop_assert_eq!(render(&v2, &out_inst), out.clone());

            // (b) No data loss: requirements is now exactly one item carrying the v1 slot
            // prose verbatim as its slot, titled as supplied; the section-level slot is gone.
            let req = out_inst
                .sections
                .iter()
                .find(|s| s.id == "requirements")
                .expect("requirements section present");
            prop_assert!(req.slot.is_none(), "section-level slot must be gone post-promotion");
            prop_assert_eq!(req.items.len(), 1, "exactly one default item");
            prop_assert_eq!(&req.items[0].title, title.trim());
            prop_assert_eq!(req.items[0].slot.clone(), v1_slot);

            // The neighbour sections are untouched (the splice is confined).
            let untouched = |id: &str| {
                let a = v1_inst.sections.iter().find(|s| s.id == id).unwrap();
                let b = out_inst.sections.iter().find(|s| s.id == id).unwrap();
                a.slot == b.slot
            };
            prop_assert!(untouched("overview"));
            prop_assert!(untouched("notes"));
        }
    }

    /// An **empty** old slot promotes to a slot-less default item (`### Title  {#id}`
    /// with no body), still byte-stable and lossless — the boundary the proptest's
    /// non-empty prose does not reach.
    #[test]
    fn empty_slot_promotes_to_slotless_default_item() {
        let v1 = v1_schema();
        let v2 = v2_schema();
        let src = v1_doc("");

        let out = promote_slot_to_repeatable(&v1, &v2, &src, "requirements", "First")
            .expect("promotion succeeds");

        let out_inst = instance_from_source(&v2, &out).expect("promoted doc conforms to v2");
        assert_eq!(render(&v2, &out_inst), out, "byte-stable round-trip");

        let req = out_inst
            .sections
            .iter()
            .find(|s| s.id == "requirements")
            .expect("requirements present");
        assert_eq!(req.items.len(), 1);
        assert_eq!(req.items[0].title, "First");
        // A minted/empty slot canonicalizes to the slot-less item form (M26 fork C4).
        assert!(req.items[0].slot.as_deref().unwrap_or("").trim().is_empty());
    }

    /// Promoting the **last** body section (no following `## `) exercises the EOF
    /// separator branch — still byte-stable and lossless.
    #[test]
    fn promotion_of_trailing_section_is_byte_stable() {
        let v1 = crate::schema::load_schema(
            b"\
type: t
sections:
  - id: overview
    slot: { hint: \"the overview\" }
  - id: requirements
    slot: { hint: \"the requirements prose\" }
",
        )
        .expect("v1 (trailing) loads");
        let v2 = crate::schema::load_schema(
            b"\
type: t
sections:
  - id: overview
    slot: { hint: \"the overview\" }
  - id: requirements
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one requirement\" } }
",
        )
        .expect("v2 (trailing) loads");
        let inst = Instance {
            title: "Doc".to_string(),
            sections: vec![
                SectionContent {
                    id: "overview".to_string(),
                    slot: Some("An overview paragraph.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "requirements".to_string(),
                    slot: Some("The single requirement prose.".to_string()),
                    ..Default::default()
                },
            ],
        };
        let src = render(&v1, &inst);

        let out = promote_slot_to_repeatable(&v1, &v2, &src, "requirements", "First")
            .expect("promotion succeeds");

        let out_inst = instance_from_source(&v2, &out).expect("conforms to v2");
        assert_eq!(render(&v2, &out_inst), out, "byte-stable round-trip at EOF");
        let req = out_inst
            .sections
            .iter()
            .find(|s| s.id == "requirements")
            .expect("requirements present");
        assert_eq!(req.items.len(), 1);
        assert_eq!(
            req.items[0].slot.as_deref(),
            Some("The single requirement prose.")
        );
    }

    /// A title with no slug-able content is rejected (no malformed empty `{#}` anchor),
    /// mirroring [`add_item`]'s mint-site guard.
    #[test]
    fn unslugable_title_is_rejected() {
        let v1 = v1_schema();
        let v2 = v2_schema();
        let src = v1_doc("some prose");
        let err = promote_slot_to_repeatable(&v1, &v2, &src, "requirements", "   ")
            .expect_err("blank title rejected");
        assert!(matches!(err, GenerateError::UnslugableTitle { .. }));
    }
}

#[cfg(test)]
mod repoint_ref_tests {
    //! The `jigc rename` referrer-repoint primitive ([`repoint_ref`]): rewrite one
    //! referrer's `ref` field old→new — a scalar replaced whole, a list-valued ref
    //! re-emitting the **canonical whole-list** with siblings/order/`", "`-separator
    //! preserved byte-for-byte, every other byte intact (`render(parse(out)) == out`).

    use super::*;
    use crate::parse::parse_sections;

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

    /// A committed `spec` whose `meta.derived-from` is the **scalar** `ref → prd`. In
    /// the frozen canonical byte form (every required section present so it round-trips).
    const SPEC_SCALAR: &str = "\
---
derived-from: prd:old
---

# Auth flow

## Goal

Deliver the auth flow.

## Context

It is needed for login.

## Criteria

### Rate limit holds  {#rate-limit}

The gateway rejects the 101st request.
";

    /// A committed `arch-doc` whose `meta.cites` is a **3-element list** `ref → adr`
    /// (`card: 0..*`), with `adr:old` flanked by siblings — the byte-exact list-rewrite
    /// fixture (acceptance #8). In the frozen canonical byte form (required sections
    /// present so it round-trips).
    const ARCH_DOC_LIST: &str = "\
---
cites: [adr:a, adr:old, adr:c]
---

# Payment subsystem

## Overview

The payment subsystem handles charges.

## Components

### The charger  {#charger}

Captures and settles a charge.

<!-- fields -->
- implemented-by: crates/pay/src/charge.rs#capture
";

    /// Scalar repoint: `derived-from: prd:old` → `prd:new`, every other byte intact,
    /// and the result round-trips byte-stable (`render(parse(out)) == out`).
    #[test]
    fn repoint_scalar_ref() {
        let schema = spec_schema();
        let out = repoint_ref(&schema, SPEC_SCALAR, "derived-from", "prd:old", "prd:new")
            .expect("the derived-from ref carries prd:old");

        // Only the one token changed — the whole doc is byte-identical otherwise.
        assert_eq!(out, SPEC_SCALAR.replace("prd:old", "prd:new"));
        assert!(out.contains("derived-from: prd:new"));
        assert!(!out.contains("prd:old"));

        // Round-trip: render(parse(out)) == out.
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(render(&schema, &reparsed), out);
    }

    /// RED (acceptance #8): a real 3-element list `cites: [adr:a, adr:old, adr:c]`
    /// rewrites to the EXACT bytes `[adr:a, adr:new, adr:c]` — siblings, order, and the
    /// `", "` separator preserved. **Proven, not asserted**: the test executes the
    /// emitted output (golden-pins the full bytes, re-parses to confirm the list value
    /// and element order, and checks the doc is byte-identical save the one swap).
    #[test]
    fn repoint_list_ref_preserves_siblings_order_separator() {
        let schema = arch_doc_schema();
        let out = repoint_ref(&schema, ARCH_DOC_LIST, "cites", "adr:old", "adr:new")
            .expect("the cites list carries adr:old");

        // The full emitted bytes (serialization → golden).
        insta::assert_snapshot!(out, @r"
        ---
        cites: [adr:a, adr:new, adr:c]
        ---

        # Payment subsystem

        ## Overview

        The payment subsystem handles charges.

        ## Components

        ### The charger  {#charger}

        Captures and settles a charge.

        <!-- fields -->
        - implemented-by: crates/pay/src/charge.rs#capture
        ");

        // The one swapped token is the ONLY change — siblings/order/separator are
        // byte-identical because the rest of the doc is byte-identical.
        assert_eq!(out, ARCH_DOC_LIST.replace("adr:old", "adr:new"));

        // Re-parse the EMITTED bytes: the list value + element order is exactly what we
        // drove (not a hand-built equivalent).
        let doc = parse_sections(&schema, &out).expect("result re-parses");
        let meta = doc.sections.iter().find(|s| s.id == "meta").unwrap();
        let cites = meta.fields.iter().find(|f| f.key == "cites").unwrap();
        assert_eq!(
            cites.value,
            crate::field_block::Value::List(vec!["adr:a".into(), "adr:new".into(), "adr:c".into()]),
            "the middle element swapped, siblings + order preserved"
        );

        // Round-trip: render(parse(out)) == out.
        let reparsed = instance_from_source(&schema, &out).expect("result conforms");
        assert_eq!(render(&schema, &reparsed), out);
    }

    /// An unknown relation, an absent field-value, or a value the field does not carry
    /// each routes to [`SpliceError::NotPresent`] (never a silent no-op or a panic).
    #[test]
    fn repoint_absent_is_not_present() {
        let schema = arch_doc_schema();
        // `old` not among the list's elements.
        assert!(matches!(
            repoint_ref(&schema, ARCH_DOC_LIST, "cites", "adr:missing", "adr:new"),
            Err(SpliceError::NotPresent { .. })
        ));
        // A relation not declared as a ref on the schema.
        assert!(matches!(
            repoint_ref(
                &schema,
                ARCH_DOC_LIST,
                "not-a-relation",
                "adr:old",
                "adr:new"
            ),
            Err(SpliceError::NotPresent { .. })
        ));
    }
}

#[cfg(test)]
mod unset {
    //! `set-field --unset` — the byte-stable field-line splice-remove (V5). Removing an
    //! optional header line / a body-or-item field bullet must round-trip byte-stable
    //! (`render(parse(x)) == x`); the last field of a group drops the `<!-- fields -->`
    //! sentinel, a surviving sibling keeps it; author-required / defaulted / `set:`-stamped
    //! fields are refused by the eligibility guard.

    use super::*;
    use crate::schema::Schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }

    /// An ADR whose optional `cites-code` header field is present (last front-matter line).
    const ADR_WITH_CITES: &str = "\
---
status: proposed
date: 2026-05-23
cites-code: src/gateway.rs#RateLimiter
---

# Rate-limit at the gateway

## Context

Per-client limits were enforced ad hoc.

## Options

Alternatives were weighed and rejected.

## Decision

Centralize rate limiting at the gateway.

## Consequences

Each service drops its local limiter.
";

    /// A spec-shaped schema whose repeatable item template carries two optional item
    /// fields (`weight`, `owner`) — the field-group last-vs-non-last surface at item level.
    fn item_field_schema() -> Schema {
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
        - { id: weight, type: int, optional: true }
        - { id: owner, type: string, optional: true }
";
        crate::schema::load_schema_with_types(yaml, &crate::schema::dev_pack_field_types())
            .expect("item-field spec schema loads")
    }

    /// A canonical single-item spec whose item carries both optional fields.
    const ITEM_TWO_FIELDS: &str = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Rate limit holds  {#rate-limit}

The gateway rejects the 101st request.

<!-- fields -->
- weight: 3
- owner: platform
";

    /// Assert `out` round-trips byte-stable through the canonical writer.
    fn assert_byte_stable(schema: &Schema, out: &str) {
        let reparsed = instance_from_source(schema, out).expect("result conforms");
        assert_eq!(
            render(schema, &reparsed),
            out,
            "the unset result must be byte-stable under the canonical writer"
        );
    }

    #[test]
    fn unset_header_field_is_byte_stable_and_reparses() {
        // The input is canonical.
        assert_byte_stable(&adr_schema(), ADR_WITH_CITES);
        let out = unset_field(&adr_schema(), ADR_WITH_CITES, "status", "cites-code")
            .expect("cites-code present");
        assert!(!out.contains("cites-code"), "the cleared line is gone");
        assert!(out.contains("status: proposed"), "siblings survive");
        assert!(out.contains("date: 2026-05-23"), "siblings survive");
        assert_byte_stable(&adr_schema(), &out);
        // The cleared field is absent from the re-parsed instance.
        let reparsed = instance_from_source(&adr_schema(), &out).expect("conforms");
        let header = reparsed.sections.iter().find(|s| s.id == "status").unwrap();
        assert!(header.fields.iter().all(|f| f.key != "cites-code"));
    }

    #[test]
    fn unset_middle_header_field_keeps_surrounding_lines() {
        // Removing a *middle* front-matter line (raw primitive; no eligibility guard)
        // leaves the lines above and below byte-intact.
        let out =
            unset_field(&adr_schema(), ADR_WITH_CITES, "status", "date").expect("date present");
        assert!(!out.contains("date:"), "the middle line is gone");
        assert!(out.contains("status: proposed"));
        assert!(out.contains("cites-code: src/gateway.rs#RateLimiter"));
        assert_byte_stable(&adr_schema(), &out);
    }

    #[test]
    fn unset_last_item_field_drops_the_sentinel_byte_stable() {
        let schema = item_field_schema();
        assert_byte_stable(&schema, ITEM_TWO_FIELDS);
        // Clear `weight` (a surviving sibling remains → sentinel kept).
        let one = unset_item_field(
            &schema,
            ITEM_TWO_FIELDS,
            "criteria",
            &["rate-limit"],
            "weight",
        )
        .expect("weight present");
        assert!(
            one.contains("<!-- fields -->"),
            "sentinel kept with a sibling"
        );
        assert!(one.contains("- owner: platform"), "sibling kept");
        assert!(!one.contains("- weight: 3"), "cleared bullet gone");
        assert_byte_stable(&schema, &one);
        // Clear the now-only field `owner` → the whole group (sentinel + leading blank)
        // drops, leaving the slot prose terminated by a single newline.
        let none = unset_item_field(&schema, &one, "criteria", &["rate-limit"], "owner")
            .expect("owner present");
        assert!(
            !none.contains("<!-- fields -->"),
            "sentinel dropped with the last field"
        );
        assert!(!none.contains("- owner"), "cleared bullet gone");
        assert!(
            none.contains("The gateway rejects the 101st request.\n"),
            "slot prose survives, one trailing newline"
        );
        assert_byte_stable(&schema, &none);
    }

    #[test]
    fn unset_absent_field_is_not_present() {
        // `supersedes` is declared but absent from the fixture → NotPresent, never a
        // corruption.
        assert!(matches!(
            unset_field(&adr_schema(), ADR_WITH_CITES, "status", "supersedes"),
            Err(SpliceError::NotPresent { .. })
        ));
    }

    #[test]
    fn validated_unset_clears_an_optional_header_scalar() {
        let UnsetOutcome::Removed(out) =
            unset_field_validated(&adr_schema(), ADR_WITH_CITES, "status", "cites-code")
                .expect("cites-code is eligible + present")
        else {
            panic!("a present field is removed, not already absent");
        };
        assert!(!out.contains("cites-code"));
        assert_byte_stable(&adr_schema(), &out);
    }

    #[test]
    fn validated_unset_of_an_already_absent_field_is_a_no_op() {
        // `supersedes` is declared, eligible and absent from the fixture. The raw splice
        // calls that `NotPresent` (above); the gated door calls it what it is — the state
        // the caller asked for, already held (M49 Inc 1 T4).
        let outcome = unset_field_validated(&adr_schema(), ADR_WITH_CITES, "status", "supersedes")
            .expect("an already-absent eligible field is a no-op, not a miss");
        assert_eq!(outcome, UnsetOutcome::AlreadyAbsent);
    }

    #[test]
    fn validated_unset_of_an_absent_field_never_outranks_the_eligibility_guard() {
        // `date` (`set: on-create`) is absent from a doc that never carried it, and the
        // refusal still fires: the no-op is the *last* rank, never a way past the guard.
        let source = ADR_WITH_CITES.replace("date: 2026-05-23\n", "");
        assert!(!source.contains("date:"), "the fixture drops the date line");
        let finding = unset_field_validated(&adr_schema(), &source, "status", "date")
            .expect_err("a set-derived field cannot be unset, present or not");
        assert_eq!(finding.code, "write.unset-ineligible");
    }

    #[test]
    fn validated_unset_rejects_a_defaulted_field() {
        // `status` carries `default: proposed` — clearing it drops a guaranteed value.
        let finding = unset_field_validated(&adr_schema(), ADR_WITH_CITES, "status", "status")
            .expect_err("a defaulted field cannot be unset");
        assert_eq!(finding.code, "write.unset-ineligible");
        assert!(finding.route.is_some(), "the guard reject carries a route");
    }

    #[test]
    fn validated_unset_rejects_a_set_derived_field() {
        // `date` carries `set: on-create` — a CLI-derived value, refused (the same arm the
        // engine-injected schema-version stamp is refused by).
        let finding = unset_field_validated(&adr_schema(), ADR_WITH_CITES, "status", "date")
            .expect_err("a set-derived field cannot be unset");
        assert_eq!(finding.code, "write.unset-ineligible");
        assert!(finding.route.is_some());
    }
}

#[cfg(test)]
mod slot_ceiling_derivation {
    //! The reserved-depth set derived from `(Schema, section_id, item-chain)`
    //! ([`slot_ceiling`]; `implementation/parsing.md` → Slot heading-depth ceiling).
    //! The shipped doctypes' contexts are swept from the real registry in
    //! `crates/cli/tests/item_slot_ceiling_axis.rs`; what lives here is the pair no
    //! registry can supply — the **single-slot-with-nested** row, which no shipped
    //! doctype expresses — plus the section-slot arm and the unresolvable chain.

    use super::*;

    /// A synthetic doctype hitting the fourth row: one item block with **exactly
    /// one** slot *and* a nested repeatable. `multi_slot` is false here, so a
    /// derivation keyed on multi-slot alone would reserve only `###` — while the
    /// parser bounds this item's leaf region at the first `####` (the nested item's
    /// own depth), which is precisely what makes `####` reserved.
    const SINGLE_SLOT_WITH_NESTED: &[u8] = br#"
type: rollup
location: rollups/
id-from: title
sections:
  - id: groups
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: summary, slot: { hint: "The group's summary." } }
        - id: entries
          repeatable:
            id-from: title
            block:
              - { id: title, type: string }
              - { id: note, slot: { hint: "The entry." } }
"#;

    fn rollup_schema() -> Schema {
        crate::schema::load_schema(SINGLE_SLOT_WITH_NESTED).expect("the synthetic schema loads")
    }

    #[test]
    fn a_section_slot_reserves_through_h3() {
        // Sections are `##` and their items `###`, so a section slot's prose starts
        // at `####` — grammar-fixed, independent of the doctype.
        let schema = rollup_schema();
        let ceiling = slot_ceiling(&schema, "groups", &[]).expect("the section arm derives");
        assert_eq!(ceiling.reserved_max, 3);
        assert_eq!(ceiling.first_allowed, 4);
    }

    #[test]
    fn a_single_slot_item_carrying_a_nested_repeatable_reserves_the_nested_depth() {
        // The row no shipped doctype hits: single-slot (so `multi_slot` is false) but
        // nested-bearing, at depth 1 → reserved through `####`, first allowed `#####`.
        let schema = rollup_schema();
        let ceiling = slot_ceiling(&schema, "groups", &["a-group"]).expect("the item arm derives");
        assert_eq!(
            (ceiling.reserved_max, ceiling.first_allowed),
            (4, 5),
            "`has_nested` reserves `2+d+1` exactly as multi-slot does"
        );
    }

    #[test]
    fn the_nested_level_reserves_its_own_depth() {
        // The nested item is a plain single-slot item at depth 2 → reserved through
        // `####` (its own heading level), first allowed `#####`.
        let schema = rollup_schema();
        let ceiling = slot_ceiling(&schema, "groups", &["a-group", "entries", "an-entry"])
            .expect("the nested arm derives");
        assert_eq!((ceiling.reserved_max, ceiling.first_allowed), (4, 5));
    }

    #[test]
    fn an_unresolvable_chain_derives_nothing() {
        // A chain that does not walk the schema has no ceiling to derive: an absent
        // section, a non-repeatable one, and an undeclared nested-section segment.
        let schema = rollup_schema();
        assert!(slot_ceiling(&schema, "absent", &["x"]).is_none());
        assert!(slot_ceiling(&schema, "groups", &["a-group", "undeclared", "x"]).is_none());
    }
}

#[cfg(test)]
mod item_slot_gate {
    //! **The item-slot arms of the one gated slot-write seam** (M45 Inc 2 T2;
    //! `implementation/parsing.md` → Slot heading-depth ceiling, enforcement site 1,
    //! and Validate-after-write).
    //!
    //! Before M45 the item paths were **public, ungated verbs**: prose carrying a
    //! heading at a CLI-reserved depth was spliced verbatim, so `### Ghost  {#ghost}`
    //! authored into a `spec` criterion's slot **minted a real repeatable item**
    //! through jigc's own write verb (reproduced live before this task: the section's
    //! item-count went 1 → 2, and the prose beneath the ghost heading was reattributed
    //! to it). They are private now — every slot write, section or item, at every
    //! nesting depth, enters through [`set_slot_validated`], which derives the reserved
    //! set from the address ([`slot_ceiling`]), rejects **before** touching bytes, and
    //! confines [`validate_after`] to the **item region**.
    //!
    //! *Nothing is persisted* on a reject is **structural, not asserted**: the gate
    //! returns a `Finding` and **no buffer**, and `cli::doc::apply_slot_target` — the
    //! CLI's only slot-write door — persists exactly the `Ok` buffer. The
    //! through-the-binary staged-bytes proof over the whole axis is the increment's
    //! acceptance (T7).
    //!
    //! Every rejection arm is paired with the **first allowed depth at that address**,
    //! so what is pinned here is a per-address ceiling, never a global one: `####` is
    //! free prose in a plain `spec` criterion and reserved structure in a `roadmap`
    //! milestone and a `changelog` change-group.

    use super::*;
    use crate::finding::Severity;
    use crate::schema::Schema;

    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");
    const CHANGELOG_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/changelog.yaml");
    const ROADMAP_YAML: &[u8] = include_bytes!("../../../packs/methodology/schemas/roadmap.yaml");

    /// The shipped `spec` — a **plain single-slot** item at depth 1 (`criteria`),
    /// reserved through `###`. Loads with the dev-pack field types (its `maps-to-test`
    /// is a `code-anchor`).
    fn spec_schema() -> Schema {
        crate::schema::load_schema_with_types(SPEC_YAML, &crate::schema::dev_pack_field_types())
            .expect("spec.yaml loads")
    }

    /// The shipped `roadmap` — a **multi-slot** item at depth 1 (`milestones`, the
    /// `proves`/`decomposition` pair under `#### <Leaf-Title>` sub-labels), so `####`
    /// is reserved and prose starts at `#####`.
    fn roadmap_schema() -> Schema {
        crate::schema::load_schema(ROADMAP_YAML).expect("roadmap.yaml loads")
    }

    /// The shipped `changelog` — the **nested** pair: a release at depth 1 carrying a
    /// nested `changes` repeatable, whose change-groups sit at depth 2 (`####`), so a
    /// change-group's `notes` prose starts at `#####`.
    fn changelog_schema() -> Schema {
        crate::schema::load_schema(CHANGELOG_YAML).expect("changelog.yaml loads")
    }

    const SPEC: &str = "\
---
---

# Rate limiting

## Goal

Limit requests.

## Context

Bursts happen.

## Criteria

### Rejects the 101st  {#rejects-the-101st}

The gateway rejects the 101st request.
";

    const ROADMAP: &str = "\
# Roadmap

## Milestones

### Alpha  {#alpha}

#### Proves

The loop closes.

#### Decomposition

Inc 1: the seam.
";

    const CHANGELOG: &str = "\
# Changelog

## Unreleased Changes


## Releases

### 1.2.0  {#1-2-0}

<!-- fields -->
- date: 2026-06-14

#### Added  {#added}

OAuth device-code flow.
";

    /// The fixtures are canonical, so a byte-identity assertion over an accepted write
    /// means what it says (`render(parse(x)) == x` on the untouched fixture).
    #[test]
    fn the_fixtures_are_canonical() {
        for (schema, src) in [
            (spec_schema(), SPEC),
            (roadmap_schema(), ROADMAP),
            (changelog_schema(), CHANGELOG),
        ] {
            let instance = instance_from_source(&schema, src)
                .unwrap_or_else(|f| panic!("fixture parses: {f:?}\n{src}"));
            assert_eq!(render(&schema, &instance), src, "fixture is canonical");
        }
    }

    /// The addressed slot write is **refused**, blocking and located, and yields no
    /// buffer at all — the shape that makes "nothing is persisted" structural.
    fn reject(schema: &Schema, source: &str, addr: SlotAddress<'_>, prose: &str) -> Finding {
        let finding = set_slot_validated(schema, source, addr, prose)
            .expect_err("reserved-depth prose must be refused");
        assert_eq!(finding.severity, Severity::Blocking);
        assert!(
            finding.location.is_some(),
            "a ceiling reject is located: {finding:?}"
        );
        finding
    }

    /// The addressed slot write is **accepted**, re-parses, and round-trips
    /// byte-stable — the first-allowed-depth half of each arm.
    fn accept(schema: &Schema, source: &str, addr: SlotAddress<'_>, prose: &str) -> String {
        let out = set_slot_validated(schema, source, addr, prose)
            .expect("prose below the address's ceiling is accepted");
        let instance = instance_from_source(schema, &out)
            .unwrap_or_else(|f| panic!("accepted result parses: {f:?}\n{out}"));
        assert_eq!(render(schema, &instance), out, "render(parse(out)) == out");
        out
    }

    /// The number of items the `section_id` repeatable parses to — the ghost-item
    /// witness (the live defect took this from 1 to 2 through the write verb).
    fn item_count(schema: &Schema, source: &str, section_id: &str) -> usize {
        parse::parse_sections(schema, source)
            .expect("source parses")
            .sections
            .iter()
            .find(|s| s.id == section_id)
            .map(|s| s.items.len())
            .expect("the section is present")
    }

    fn criterion(leaf: &str) -> SlotAddress<'_> {
        SlotAddress::Item {
            section: "criteria",
            chain: &["rejects-the-101st"],
            leaf,
        }
    }

    /// **The headline arm.** `### Ghost  {#ghost}` into a `spec` criterion's slot is
    /// refused with a located blocking `write.slot-heading-depth` naming `####` — the
    /// depth free *here* — and no buffer is produced, so the ghost item is never
    /// minted. The paired write at that first-allowed depth is accepted and leaves the
    /// item count at 1.
    #[test]
    fn a_ghost_item_heading_never_reaches_a_spec_criterion_slot() {
        let schema = spec_schema();
        assert_eq!(item_count(&schema, SPEC, "criteria"), 1);

        let finding = reject(
            &schema,
            SPEC,
            criterion("statement"),
            "The gateway rejects it.\n\n### Ghost  {#ghost}\n\nHijacked prose.",
        );
        assert_eq!(finding.code, "write.slot-heading-depth");
        // Located at the offending line **within the agent's prose** (1-based).
        assert_eq!(finding.location.as_ref().unwrap().line, 3);
        assert!(
            finding.message.contains("`###`") && finding.message.contains("`####`"),
            "the reject names the reserved depth and the one free here: {}",
            finding.message
        );

        // The first ALLOWED depth at this address is accepted — and mints nothing.
        let out = accept(
            &schema,
            SPEC,
            criterion("statement"),
            "The gateway rejects it.\n\n#### Detail\n\nWithin the window.",
        );
        assert_eq!(item_count(&schema, &out, "criteria"), 1);
        assert!(out.contains("#### Detail"));
    }

    /// The **section-hijack** variant on the same address: `## Ghost Section` is
    /// shallower still — reserved by the section grammar, not just the item one.
    #[test]
    fn a_section_heading_never_reaches_an_item_slot() {
        let schema = spec_schema();
        let finding = reject(
            &schema,
            SPEC,
            criterion("statement"),
            "Rejected.\n\n## Ghost Section\n\nDownstream prose.",
        );
        assert_eq!(finding.code, "write.slot-heading-depth");
        assert!(finding.message.contains("`##`"), "{}", finding.message);

        // And the shallowest reserved depth of all — the document's own `#` title.
        let h1 = reject(&schema, SPEC, criterion("statement"), "# Another doc");
        assert_eq!(h1.code, "write.slot-heading-depth");
    }

    /// The **Setext** variant: an underline heading is refused at any depth, on the
    /// item path exactly as on the section path.
    #[test]
    fn a_setext_heading_never_reaches_an_item_slot() {
        let schema = spec_schema();
        let finding = reject(
            &schema,
            SPEC,
            criterion("statement"),
            "Intro.\n\nA title\n=======\n",
        );
        assert_eq!(finding.code, "write.slot-setext-heading");
        assert_eq!(finding.location.as_ref().unwrap().line, 3);
    }

    /// **The multi-slot arm.** A `roadmap` milestone reserves one level deeper than a
    /// plain item — `#### Proves` is a sub-label, so writing it into `decomposition`
    /// would silently reattribute the prose beneath it to the sibling leaf. Refused;
    /// `#####` is what is free here, and it is accepted.
    #[test]
    fn a_sub_label_depth_heading_never_reaches_a_multi_slot_item() {
        let schema = roadmap_schema();
        let addr = |leaf| SlotAddress::Item {
            section: "milestones",
            chain: &["alpha"],
            leaf,
        };

        let finding = reject(
            &schema,
            ROADMAP,
            addr("decomposition"),
            "Inc 1: the seam.\n\n#### Proves\n\nSmuggled into the sibling leaf.",
        );
        assert_eq!(finding.code, "write.slot-heading-depth");
        assert_eq!(finding.location.as_ref().unwrap().line, 3);
        assert!(
            finding.message.contains("`####`") && finding.message.contains("`#####`"),
            "the reject names `#####` as free here, not the global `####`: {}",
            finding.message
        );

        let out = accept(
            &schema,
            ROADMAP,
            addr("decomposition"),
            "Inc 1: the seam.\n\n##### Tasks\n\nT1 then T2.",
        );
        assert!(out.contains("##### Tasks"));
        // The sibling leaf's prose survives, and the milestone is still one item.
        assert!(out.contains("The loop closes."));
        assert_eq!(item_count(&schema, &out, "milestones"), 1);
    }

    /// **The nested arm.** A `changelog` change-group sits at depth 2, so `####` is
    /// its own item depth — writing one into its `notes` would mint a sibling
    /// change-group under the same release. Refused; `#####` is accepted.
    #[test]
    fn an_item_depth_heading_never_reaches_a_nested_item_slot() {
        let schema = changelog_schema();
        let addr = |leaf| SlotAddress::Item {
            section: "releases",
            // The section-qualified chain: release → the nested `changes` section →
            // the change-group item.
            chain: &["1-2-0", "changes", "added"],
            leaf,
        };

        let finding = reject(
            &schema,
            CHANGELOG,
            addr("notes"),
            "OAuth device-code flow.\n\n#### Fixed  {#fixed}\n\nA smuggled group.",
        );
        assert_eq!(finding.code, "write.slot-heading-depth");
        assert_eq!(finding.location.as_ref().unwrap().line, 3);
        assert!(
            finding.message.contains("`#####`"),
            "the reject names the depth free at THIS address: {}",
            finding.message
        );

        let out = accept(
            &schema,
            CHANGELOG,
            addr("notes"),
            "OAuth device-code flow.\n\n##### Caveat\n\nDesktop only.",
        );
        assert!(out.contains("##### Caveat"));
        // Still exactly one release, carrying exactly one change-group.
        let doc = parse::parse_sections(&schema, &out).expect("result parses");
        let releases = doc
            .sections
            .iter()
            .find(|s| s.id == "releases")
            .expect("releases present");
        assert_eq!(releases.items.len(), 1);
        assert_eq!(releases.items[0].items.len(), 1);
    }

    /// Validate-after's clause (b) is confined to the **item region**, never the
    /// addressed leaf's span: the item paths re-render the whole item block, so a
    /// leaf-narrowed target would trip `write.target-escape` on every accepted
    /// item-slot write — including one that changes the item's canonical shape (here
    /// the multi-slot fill of a **mint-empty** leaf, which grows blank lines the leaf
    /// span never covered).
    #[test]
    fn the_confinement_target_is_the_item_region() {
        let schema = roadmap_schema();
        let empty = render(
            &schema,
            &instance_from_source(&schema, "# Roadmap\n\n## Milestones\n").expect("empty parses"),
        );
        let minted = add_item(&schema, &empty, "milestones", "Alpha", None, None, &[])
            .expect("the mint-empty skeleton");
        let filled = accept(
            &schema,
            &minted,
            SlotAddress::Item {
                section: "milestones",
                chain: &["alpha"],
                leaf: "proves",
            },
            "The loop closes.",
        );
        assert!(filled.contains("The loop closes."));
    }

    /// An address that resolves against **no** schema shape derives no ceiling — and
    /// reaches no bytes either: the splice refuses it as not-present rather than
    /// writing unchecked prose. Same for an item that is simply absent from the
    /// document.
    #[test]
    fn an_unresolvable_item_address_is_refused_not_written() {
        let schema = spec_schema();
        // A **non-repeatable** section addressed as an item: `slot_ceiling` derives
        // nothing, and the reserved-depth prose still never lands.
        let unresolvable = set_slot_validated(
            &schema,
            SPEC,
            SlotAddress::Item {
                section: "goal",
                chain: &["ghost"],
                leaf: "statement",
            },
            "### Ghost  {#ghost}",
        )
        .expect_err("a non-repeatable section carries no item slot");
        assert_eq!(unresolvable.code, "write.not-present");
        assert_eq!(unresolvable.severity, Severity::Blocking);

        // An absent item under a real repeatable, with clean prose.
        let absent = set_slot_validated(
            &schema,
            SPEC,
            SlotAddress::Item {
                section: "criteria",
                chain: &["ghost"],
                leaf: "statement",
            },
            "Clean prose.",
        )
        .expect_err("an absent item is refused");
        assert_eq!(absent.code, "write.not-present");
    }
}

#[cfg(test)]
mod splice_error_payload {
    //! **The rejected write names the break it hit** (M45 Inc 2 T4). A slot write over
    //! a staged doc that no longer parses used to report one context-free sentence —
    //! *"write rejected: the source does not conform to the schema"* — while the parse
    //! findings that say **where** and **why** were discarded at the `map_err` boundary
    //! of all twelve construction sites. [`SpliceError::NotConformant`] carries them
    //! now, so the reject is located and names the offending line's diagnosis.
    //!
    //! The corollary is [`SpliceError::UndeclaredSection`]: once the variant *means*
    //! "the buffer is broken, here is the break", a schema-lookup miss — which holds no
    //! findings at all — can no longer borrow that sentence.

    use super::*;
    use crate::finding::Severity;
    use crate::schema::Schema;

    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");

    fn spec_schema() -> Schema {
        crate::schema::load_schema_with_types(SPEC_YAML, &crate::schema::dev_pack_field_types())
            .expect("spec.yaml loads")
    }

    /// A staged `spec` carrying the trial's real corruption shape: a second criterion
    /// heading at the reserved item depth with **no** `{#id}` anchor (line 20), so the
    /// buffer does not parse and no span can be located in it.
    const BROKEN_SPEC: &str = "\
---
---

# Rate limiting

## Goal

Limit requests.

## Context

Bursts happen.

## Criteria

### Rejects the 101st  {#rejects-the-101st}

The gateway rejects the 101st request.

### Ghost

Smuggled prose.
";

    /// The line `### Ghost` sits on in [`BROKEN_SPEC`] — the break the reject must point at.
    const BREAK_LINE: usize = 20;

    /// The **item arm**: a criterion-slot write over the broken buffer is refused with
    /// the parse break's own diagnosis and located at the offending line, not at 1:1.
    #[test]
    fn an_item_slot_write_over_a_broken_source_names_the_parse_break() {
        let finding = set_slot_validated(
            &spec_schema(),
            BROKEN_SPEC,
            SlotAddress::Item {
                section: "criteria",
                chain: &["rejects-the-101st"],
                leaf: "statement",
            },
            "Clean prose.",
        )
        .expect_err("a non-conformant source cannot be spliced");
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(finding.code, "write.non-reparseable");
        assert!(
            finding.message.contains("Ghost"),
            "the reject must name the break it hit, not just its class: {}",
            finding.message
        );
        assert_eq!(
            finding.location.as_ref().map(|l| l.line),
            Some(BREAK_LINE),
            "the reject must point at the offending line: {finding:?}"
        );
    }

    /// The **section arm** carries the same payload — one rendering, both paths.
    #[test]
    fn a_section_slot_write_over_a_broken_source_names_the_parse_break() {
        let finding = set_slot_validated(
            &spec_schema(),
            BROKEN_SPEC,
            SlotAddress::Section { section: "goal" },
            "Clean prose.",
        )
        .expect_err("a non-conformant source cannot be spliced");
        assert_eq!(finding.code, "write.non-reparseable");
        assert!(
            finding.message.contains("Ghost"),
            "the reject must name the break it hit, not just its class: {}",
            finding.message
        );
        assert_eq!(finding.location.as_ref().map(|l| l.line), Some(BREAK_LINE));
    }

    /// A **schema-lookup miss** is not a malformed buffer: it reports the undeclared
    /// section under the shape code its [`GenerateError::UnknownSection`] sibling
    /// already uses, and never the conformance sentence. (The three sites that mint it
    /// are defensive — the parser only ever names schema-declared sections
    /// (`implementation/parsing.md` → Surplus-section tolerance) — which is exactly why
    /// they must not fall back onto a findings-less `NotConformant`.)
    #[test]
    fn an_undeclared_section_reports_the_section_not_a_malformed_buffer() {
        let finding = splice_error_finding(&SpliceError::UndeclaredSection {
            section: "nonsuch".to_string(),
            under: None,
        });
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(finding.code, "write.unknown-section");
        assert!(
            finding.message.contains("nonsuch"),
            "the reject must name the undeclared section: {}",
            finding.message
        );
        assert!(
            !finding.message.contains("does not conform"),
            "an undeclared section is not a malformed buffer: {}",
            finding.message
        );
    }
}

#[cfg(test)]
mod not_present_route_followability {
    //! **P6 route-followability over the write gate's own routes** (M47 Inc 6 T3;
    //! `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8). [`write_route`] mints two
    //! placeholder-carrying mechanical routes — the `jigc doc schema <doctype>` shape
    //! question and the `write.not-present` **defensive fallback** — and both are
    //! substituted from the finding's own target the moment one is established.
    //!
    //! The fallback is exercised **here** rather than through the binary, and the reason is
    //! recorded rather than glossed: M47 Inc 6 T2 wired
    //! [`crate::finding::Finding`]-enrichment onto all six write verbs, and the batch
    //! `jigc doc author` path lowers every item hop with the same `slugify` the engine
    //! mints ids with (`cli::author::flatten_section`), so it cannot address an item it did
    //! not just add. The fallback therefore has **no binary-reachable producer today** — it
    //! is the declared defence for a future un-wired one, and its substitution is pinned at
    //! the level where it is reachable.

    use super::*;
    use crate::finding::Location;

    /// The shape question: `<doctype>` is the `<type>` head of the finding's own target,
    /// so the emitted route names the doctype whose schema the agent must read.
    #[test]
    fn the_shape_question_route_names_the_real_doctype() {
        let mut finding = blocking_write(
            "write.wrong-shape",
            "write rejected: section \"summary\" is not repeatable",
            Location::at(1, 1),
        );
        crate::finding::readdress_to_uri(
            std::slice::from_mut(&mut finding),
            "commit:log-it#summary",
        );

        assert_eq!(
            finding.route.as_ref().map(|r| r.as_str()),
            Some(
                "`jigc doc schema commit` to see the declared shape, then re-run the write \
                 at a declared address"
            ),
        );
    }

    /// The **un-enriched `write.not-present` fallback**: once a target is established the
    /// route names the real, *showable* containing section — byte-identical to the CLI
    /// enrichment's own address half — never the `<address>` placeholder, and never the
    /// absent item's own address (which no read verb can resolve).
    #[test]
    fn the_not_present_fallback_route_names_the_real_showable_address() {
        let mut finding = blocking_write(
            "write.not-present",
            "write rejected: item \"9-9-9\" in section \"releases\" not present",
            Location::at(1, 1),
        );
        crate::finding::readdress_to_uri(
            std::slice::from_mut(&mut finding),
            "changelog:changelog#releases/9-9-9/summary",
        );

        assert_eq!(
            finding.route.as_ref().map(|r| r.as_str()),
            Some(
                "`jigc doc show changelog:changelog#releases --task <task-id>` to see the \
                 section's current item ids, then re-run the write at an existing item"
            ),
        );
    }
}
