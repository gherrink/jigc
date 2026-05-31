//! Schema-driven Markdown **read path**: block structure → sections + slot spans.
//!
//! The #1 technical risk: lossless, diff-clean round-trip. See
//! `implementation/parsing.md` (offset-splice, never re-stringify, re-parse to
//! validate) and `design/storage.md` for the on-disk format. This module owns the
//! *read* half: a full CommonMark **block** parse, mapped onto a [`Schema`], that
//! locates each section and records each slot's prose as an **opaque byte span**.
//! The surgical-splice *writer* is a later task; nothing here re-stringifies.
//!
//! ## The parse model (`parsing.md` → The parse model)
//!
//! Raw bytes become section/slot structure through a real block parse, **not** a
//! line scanner — a line scanner misreads a `## …` line *inside a fenced code
//! block* as a heading, and slot prose is arbitrary Markdown that will contain
//! such cases. `pulldown-cmark`'s offset iterator hands trustworthy byte ranges:
//!
//! - the **metadata block** (`---`-fenced front-matter) bounds the header section;
//! - a schema-fixed `##` ATX **heading** opens a section; its text matches the
//!   schema section in document order;
//! - the bytes **between a section heading and the next section boundary** (the
//!   next `##`, a `<!-- fields -->` field-group sentinel, or EOF) are the slot's
//!   **opaque** content span — the parser never interprets them.
//!
//! ## The determinism boundary, applied literally
//!
//! The parser owns **block structure** (north of the line — which section, where
//! it begins/ends) and treats **slot prose as an opaque byte span** it never reads
//! (south of the line). The one structural constraint it *does* enforce inside a
//! slot is the **heading-depth ceiling** (`parsing.md` → Slot heading-depth
//! ceiling): no ATX `##`/`###` and no Setext heading in slot prose, because those
//! are CLI-owned structural markers — a `## Decision` authored in a slot would
//! otherwise force the parser to content-sniff. A violation is a located
//! [`Severity::Blocking`] conformance [`Finding`], never an instance.
//!
//! ## Conformance diagnostics (`parsing.md` → Conformance diagnostics)
//!
//! Conformance falls out of the parse — no second pass. A successful schema mapping
//! *is* the conformance check; each point it can't proceed is a located diagnostic.
//! The parser returns **either** a parsed [`Document`] **or** a non-empty set of
//! [`Finding`]s (collect-all in one pass), all carrying the one
//! [`crate::finding`] envelope.
//!
//! ## Scope (read path)
//!
//! The full read path's *block + field + item* structure:
//!
//! - **Sections + slot spans** — the header (front-matter) section and body slot
//!   sections, each slot recorded as an opaque byte span.
//! - **Fields** — the `---` front-matter block (the header section's fields) and a
//!   simple section's trailing `<!-- fields -->`-sentinelled bullet group both read
//!   through the one flat [`crate::field_block`] grammar; each value is opaque text
//!   (the schema, not this layer, types it), each key validated against the
//!   section's declared fields.
//! - **Repeatable `{#id}` items** — a `repeatable` section's `### …{#id}` items,
//!   each with its frozen anchor, mutable title, opaque slot span, and sentinelled
//!   per-item fields.
//!
//! Each mismatch — a missing/renamed section, an orphaned sentinel, an unknown
//! field key, a missing/duplicate/malformed `{#id}`, a slot heading-depth-ceiling
//! violation — is a located [`Severity::Blocking`] conformance [`Finding`], never a
//! panic and never a mis-parse. The surgical-splice *writer* is a later task;
//! nothing here re-stringifies.

use std::ops::Range;

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};

use crate::finding::{Finding, Location};
use crate::schema::{Schema, SectionBody};

/// A parsed document instance: the schema's sections, in document order, each with
/// its located slot span (if it has one).
///
/// Produced only when the source maps cleanly onto the schema; any structural
/// mismatch yields [`Finding`]s instead (see [`parse_sections`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    /// The sections, in schema document order.
    pub sections: Vec<ParsedSection>,
}

/// One mapped section: its schema `id`, its slot span (if any), its trailing
/// field group (if any), and — for a repeatable section — its items.
///
/// The header (front-matter) section has no slot (`slot: None`) but carries the
/// front-matter `fields`; a body slot section carries `Some(span)` and any
/// sentinelled trailing `fields`; a repeatable section carries `items` instead.
/// A span is recorded, not interpreted: re-slicing the source over it yields
/// exactly the slot's prose bytes (the round-trip boundary-integrity property).
/// Field values are likewise opaque text — the schema, not the parser, types them.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedSection {
    /// The schema section id this maps to.
    pub id: String,
    /// The opaque slot prose span, when the section has a slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<Span>,
    /// The section's fields, in physical order — front-matter fields for the
    /// header section, the sentinelled trailing bullet group for a body section.
    /// Each value is opaque text (the [`crate::field_block`] grammar).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<crate::field_block::Field>,
    /// The repeatable section's items, in physical order, each with its frozen
    /// `{#id}` anchor. Empty for a simple section.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ParsedItem>,
}

/// One repeatable item: its frozen `{#id}` anchor, its mutable title (the `###`
/// heading text, the id-source field's value), its opaque slot span (if the item
/// template declares a slot), and its sentinelled per-item fields.
///
/// The anchor is the only instance-minted in-body identity ([`parsing.md`] →
/// `{#id}` anchors); it is read as an opaque frozen token, mapped to the address
/// fragment, and survives a title rename. The title is the heading text with the
/// `### ` marker and the `{#id}` anchor stripped.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedItem {
    /// The frozen `{#id}` anchor — the item's stable id fragment.
    pub id: String,
    /// The item's mutable title (the `###` heading text, anchor stripped).
    pub title: String,
    /// The item body's opaque slot span, when the item template declares a slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<Span>,
    /// The item's sentinelled per-item fields, in physical order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<crate::field_block::Field>,
}

/// A half-open byte range `[start, end)` into the source, plus the 1-based source
/// line the span starts at (for diagnostics and human-facing addressing).
///
/// `start..end` is exactly what the writer will splice against; `start_line` is the
/// located coordinate the [`crate::finding`] envelope reports. Slicing the source
/// `&src[start..end]` returns the opaque slot bytes verbatim.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Byte offset of the first byte (inclusive).
    pub start: usize,
    /// Byte offset one past the last byte (exclusive).
    pub end: usize,
    /// 1-based source line the span starts on.
    pub start_line: usize,
}

impl Span {
    /// Re-slice the source over this span — the opaque slot bytes verbatim.
    pub fn slice<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}

/// Parse `source` against `schema`, returning the located [`Document`] on a clean
/// schema mapping, or the set of conformance [`Finding`]s otherwise.
///
/// The mapping *is* the conformance check (`parsing.md` → Conformance falls out of
/// the parser): the parser walks the block structure once, matches each schema
/// section heading in document order, records each slot's opaque span, reads each
/// section's fields (front-matter or sentinelled bullet group) and a repeatable
/// section's `{#id}` items, and enforces the heading-depth ceiling inside every
/// slot. Any point it can't proceed — a missing/renamed required heading, a
/// forbidden heading inside slot prose, an orphaned sentinel, an unknown field key,
/// a missing/duplicate/malformed `{#id}` — is a located [`Severity::Blocking`]
/// finding; findings are collected, not fail-fast. A non-empty finding set means
/// *no* instance is returned.
pub fn parse_sections(schema: &Schema, source: &str) -> Result<Document, Vec<Finding>> {
    let blocks = scan_blocks(source);
    let mut findings = Vec::new();
    let mut parsed = Vec::new();

    // The header (front-matter) section, if the schema declares one, is consumed by
    // the metadata block; it has no slot, but carries the front-matter fields.
    let mut block_cursor = 0;
    let header_idx = schema.sections.iter().position(is_header_section);
    if let Some(idx) = header_idx {
        // The header must be the first schema section (front-matter is at the top).
        if idx != 0 {
            findings.push(Finding::blocking(
                "conformance.header-not-first",
                "the header section must be the document's first section",
                Location::at(1, 1),
            ));
        }
        // Read the front-matter fields from the metadata block (if present),
        // validating each key against the header section's declared fields.
        let header = &schema.sections[idx];
        let declared = declared_field_keys(header);
        let header_fields = match blocks.first() {
            Some(Block::Metadata { content }) => {
                block_cursor = 1;
                read_field_block(source, content.clone(), &declared, &mut findings)
            }
            // Absent front-matter is a finalize/field concern, not a block-structure
            // conformance error at this layer.
            _ => Vec::new(),
        };
        parsed.push(ParsedSection {
            id: header.id.clone(),
            slot: None,
            fields: header_fields,
            items: Vec::new(),
        });
    }

    // Body sections: each schema body section matches the next `##` heading, in
    // document order. The slot span runs from after the heading to the next
    // section boundary.
    let body_sections: Vec<&crate::schema::Section> = schema
        .sections
        .iter()
        .filter(|s| !is_header_section(s))
        .collect();

    // Body sections map to `##` (H2) section headings, in document order. The `# H1`
    // is the document *title* (rendered from `id-from`, `storage.md` → Identity), and
    // any `####`+ heading is allowed slot-internal structure — both are excluded from
    // the section-heading list so index-based matching aligns sections to `##`s.
    let headings: Vec<&Block> = blocks[block_cursor..]
        .iter()
        .filter(|b| {
            matches!(
                b,
                Block::Heading {
                    level: HeadingLevel::H2,
                    ..
                }
            )
        })
        .collect();

    for (i, section) in body_sections.iter().enumerate() {
        let Some(Block::Heading {
            text,
            line,
            content_start,
            ..
        }) = headings.get(i).copied()
        else {
            findings.push(Finding::blocking(
                "conformance.section-missing",
                format!("required section heading `## {}` is missing", section.id),
                Location::at(1, 1),
            ));
            continue;
        };

        // The schema-fixed heading text must match the section id (case-folded,
        // trimmed) — sections match by their schema-fixed headings, not position
        // alone, so a renamed heading is a located error.
        if !heading_matches(text, &section.id) {
            findings.push(Finding::blocking(
                "conformance.section-renamed",
                format!(
                    "section heading {text:?} does not match required section `{}`",
                    section.id
                ),
                Location::at(*line, 1),
            ));
        }

        // The section's region runs from after its heading to the next `##` (or EOF).
        let region_end = next_section_heading(&blocks, *content_start).unwrap_or(source.len());

        match &section.body {
            // A repeatable section: parse its `### …{#id}` items in the region.
            SectionBody::Repeatable { repeatable } => {
                let items = parse_items(
                    source,
                    &blocks,
                    *content_start,
                    region_end,
                    repeatable,
                    &mut findings,
                );
                parsed.push(ParsedSection {
                    id: section.id.clone(),
                    slot: None,
                    fields: Vec::new(),
                    items,
                });
            }
            // A simple section: a slot span plus an optional sentinelled field group.
            SectionBody::Simple { slot, fields: _ } => {
                // The slot span: from the byte after the heading to the next
                // boundary (next `##`, a field-group sentinel, or EOF).
                let slot_end = body_boundary(&blocks, *content_start, region_end);
                let span = trim_span(source, *content_start, slot_end);

                // Heading-depth ceiling inside the located slot span.
                for v in ceiling_violations(&blocks, span.start, span.end) {
                    findings.push(v);
                }

                // Read a trailing field group (sentinel + following list), if any,
                // against the section's declared fields.
                let declared = declared_field_keys(section);
                let section_fields = read_field_group(
                    source,
                    &blocks,
                    *content_start,
                    region_end,
                    &declared,
                    &mut findings,
                );

                let slot = slot.as_ref().map(|_| span);

                parsed.push(ParsedSection {
                    id: section.id.clone(),
                    slot,
                    fields: section_fields,
                    items: Vec::new(),
                });
            }
        }
    }

    if findings.is_empty() {
        Ok(Document { sections: parsed })
    } else {
        Err(findings)
    }
}

/// A schema section is the header (front-matter) section iff its `header` flag is set.
fn is_header_section(section: &crate::schema::Section) -> bool {
    section.header
}

/// Case-insensitive, whitespace-trimmed match of a heading's text against a
/// section id. The schema id is the heading's slug-ish source; the MVP schemas use
/// ids equal to the heading text lowercased (`context`, `decision`), so a
/// lowercased compare is the conformance check.
fn heading_matches(text: &str, section_id: &str) -> bool {
    text.trim().eq_ignore_ascii_case(section_id.trim())
}

/// A coarse block: the structural events the schema mapping cares about, each with
/// its byte range and 1-based start line. Inline content and paragraph internals
/// are *not* modelled — slot prose is opaque.
#[derive(Debug)]
enum Block {
    /// A `---`-fenced metadata (front-matter) block, carrying the byte range of its
    /// **inner content** (the bare `key: value` lines between the fences). The
    /// header section's fields are read from this range via [`crate::field_block`].
    Metadata { content: Range<usize> },
    /// An ATX or Setext heading. `is_atx` distinguishes `##`-style from Setext
    /// underline-style (the latter forbidden in slots at any depth). `raw` is the
    /// heading's source range (so the `{#id}` anchor can be re-scanned for items).
    Heading {
        level: HeadingLevel,
        is_atx: bool,
        text: String,
        range: Range<usize>,
        /// Byte offset just past the heading (where slot prose may begin).
        content_start: usize,
        line: usize,
    },
    /// The `<!-- fields -->` field-group sentinel (an HTML block).
    FieldSentinel { range: Range<usize> },
    /// A top-level bullet list, carrying each item's byte range. A list immediately
    /// following a [`Block::FieldSentinel`] is a field group; the per-item ranges
    /// (stripped of their `- ` marker) feed the flat field-block reader.
    List {
        range: Range<usize>,
        items: Vec<Range<usize>>,
    },
}

/// Walk the CommonMark block events once, projecting the structural blocks the
/// schema mapping needs (headings, the metadata block, the field-group sentinel,
/// top-level bullet lists), each with its byte range and start line.
fn scan_blocks(source: &str) -> Vec<Block> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let mut blocks = Vec::new();
    // Track the current top-level list (depth-1) and its item ranges; nested lists
    // (depth > 1) are slot-internal prose, never a field group, so we ignore them.
    let mut list_depth = 0usize;
    let mut cur_list: Option<(Range<usize>, Vec<Range<usize>>)> = None;
    // The metadata block's inner content range (the `Text` event between fences).
    let mut meta_content: Option<Range<usize>> = None;
    let mut in_metadata = false;

    for (event, range) in Parser::new_ext(source, opts).into_offset_iter() {
        match event {
            Event::Start(Tag::MetadataBlock(_)) => {
                in_metadata = true;
                meta_content = None;
            }
            Event::Text(_) if in_metadata && meta_content.is_none() => {
                meta_content = Some(range.clone());
            }
            Event::End(TagEnd::MetadataBlock(_)) => {
                in_metadata = false;
                blocks.push(Block::Metadata {
                    content: meta_content.take().unwrap_or(range.start..range.start),
                });
            }
            Event::Start(Tag::Heading { level, .. }) => {
                let raw = &source[range.clone()];
                let is_atx = raw.trim_start().starts_with('#');
                let text = heading_text(raw, is_atx);
                let line = line_of(source, range.start);
                blocks.push(Block::Heading {
                    level,
                    is_atx,
                    text,
                    content_start: range.end,
                    range,
                    line,
                });
            }
            Event::Start(Tag::HtmlBlock) if source[range.clone()].trim_end() == FIELD_SENTINEL => {
                blocks.push(Block::FieldSentinel { range });
            }
            Event::Start(Tag::List(_)) => {
                list_depth += 1;
                if list_depth == 1 {
                    cur_list = Some((range, Vec::new()));
                }
            }
            Event::Start(Tag::Item) if list_depth == 1 => {
                if let Some((_, items)) = cur_list.as_mut() {
                    items.push(range);
                }
            }
            Event::End(TagEnd::List(_)) => {
                if list_depth == 1
                    && let Some((range, items)) = cur_list.take()
                {
                    blocks.push(Block::List { range, items });
                }
                list_depth = list_depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    blocks
}

/// The reserved field-group boundary marker (`parsing.md` → Field-group delineation).
const FIELD_SENTINEL: &str = "<!-- fields -->";

/// Extract a heading's text from its raw source range. For an ATX heading, strip
/// the leading `#`s, a trailing `{#id}` anchor and trailing `#`s; for a Setext
/// heading, drop the underline line. The id-anchor is consumed by pulldown-cmark's
/// heading-attributes extension separately; here we only need clean title text.
fn heading_text(raw: &str, is_atx: bool) -> String {
    let first_line = raw.lines().next().unwrap_or("");
    if is_atx {
        let after_hashes = first_line.trim_start().trim_start_matches('#');
        let no_anchor = match after_hashes.split_once("{#") {
            Some((before, _)) => before,
            None => after_hashes,
        };
        no_anchor.trim().trim_end_matches('#').trim().to_string()
    } else {
        first_line.trim().to_string()
    }
}

/// The 1-based source line a byte offset falls on.
fn line_of(source: &str, offset: usize) -> usize {
    source[..offset].bytes().filter(|&b| b == b'\n').count() + 1
}

/// The start offset of the next **`##` section heading** at or after `from`, if any.
/// Deeper (`###`/`####`+) headings are *not* section boundaries — `###` opens a
/// repeatable item (handled within the section), `####`+ is slot-internal structure.
fn next_section_heading(blocks: &[Block], from: usize) -> Option<usize> {
    blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level: HeadingLevel::H2,
                range,
                ..
            } if range.start >= from => Some(range.start),
            _ => None,
        })
        .min()
}

/// The byte offset where a simple section's slot prose ends: the first field-group
/// sentinel in `[from, region_end)`, else `region_end`. The sentinel marks where
/// the trailing field group begins, so the slot prose stops there.
fn body_boundary(blocks: &[Block], from: usize, region_end: usize) -> usize {
    blocks
        .iter()
        .filter_map(|b| match b {
            Block::FieldSentinel { range } if range.start >= from && range.start < region_end => {
                Some(range.start)
            }
            _ => None,
        })
        .min()
        .unwrap_or(region_end)
}

/// The declared field keys of a simple section, in schema order (for unknown-key
/// "did you mean" and presence checks). A repeatable or slot-only section returns
/// an empty set.
fn declared_field_keys(section: &crate::schema::Section) -> Vec<String> {
    match &section.body {
        SectionBody::Simple { fields, .. } => fields.iter().map(|f| f.id.clone()).collect(),
        SectionBody::Repeatable { .. } => Vec::new(),
    }
}

/// Parse a flat field block over `content` (front-matter inner content or a body
/// field-group's bullet text already de-bulleted), validating every key against
/// `declared`. A bare grammar defect (no `:` separator) or an unknown key is a
/// located [`Severity::Blocking`] finding; valid fields are returned.
fn read_field_block(
    source: &str,
    content: Range<usize>,
    declared: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<crate::field_block::Field> {
    let base_line = line_of(source, content.start);
    read_field_block_str(&source[content], base_line, declared, findings)
}

/// Build the unknown-field message with a cheap "did you mean" when a declared key
/// is one edit away (parsing.md → Conformance: unknown field key with a cheap hint).
fn unknown_field_message(key: &str, declared: &[String]) -> String {
    let suggestion = declared
        .iter()
        .find(|d| is_near(key, d))
        .map(|d| format!(" — did you mean `{d}`?"))
        .unwrap_or_default();
    format!("unknown field key `{key}`{suggestion}")
}

/// A cheap nearness check: equal ignoring case, or a one-char length difference
/// with a shared prefix. Just enough for a typo hint, never load-bearing.
fn is_near(a: &str, b: &str) -> bool {
    if a.eq_ignore_ascii_case(b) {
        return true;
    }
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    long.len() - short.len() <= 1 && long.starts_with(&short[..short.len().min(3)])
}

/// Read a trailing field group in `[from, end)`, if present: a `<!-- fields -->`
/// sentinel immediately followed by a bullet list. The sentinel's per-bullet
/// content (each `- ` stripped) is read as a flat field block against `declared`. A
/// sentinel with no following list is an orphaned-sentinel finding; an unknown
/// bullet key is an unknown-field finding. One rule, two callers (a simple section
/// scoped to its region, a repeatable item scoped to its body — parsing.md →
/// Field-group delineation: "one rule, both places").
fn read_field_group(
    source: &str,
    blocks: &[Block],
    from: usize,
    end: usize,
    declared: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<crate::field_block::Field> {
    let Some(sentinel) = blocks.iter().find_map(|b| match b {
        Block::FieldSentinel { range } if range.start >= from && range.start < end => {
            Some(range.clone())
        }
        _ => None,
    }) else {
        return Vec::new();
    };
    read_marked_field_group(source, blocks, &sentinel, declared, findings)
}

/// Read the bullet list immediately following a `<!-- fields -->` `sentinel` as a
/// flat field block. The list must start at the sentinel's end (canonical form:
/// sentinel on its own line, then a contiguous bullet list, no blank line between).
/// No following list → orphaned-sentinel finding.
fn read_marked_field_group(
    source: &str,
    blocks: &[Block],
    sentinel: &Range<usize>,
    declared: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<crate::field_block::Field> {
    let list = blocks.iter().find_map(|b| match b {
        Block::List { range, items } if range.start >= sentinel.end => Some((range.clone(), items)),
        _ => None,
    });
    // The list must immediately follow the sentinel (only blank-line whitespace
    // between), else the sentinel is orphaned.
    let following = list.filter(|(range, _)| source[sentinel.end..range.start].trim().is_empty());
    let Some((_, items)) = following else {
        findings.push(Finding::blocking(
            "conformance.orphaned-sentinel",
            "`<!-- fields -->` sentinel without a following bullet list".to_string(),
            Location::at(line_of(source, sentinel.start), 1),
        ));
        return Vec::new();
    };

    // De-bullet each item (`- key: value` → `key: value`) and join into a flat
    // field block, then read it against the declared keys.
    let mut block = String::new();
    for item in items {
        let raw = source[item.clone()].trim_end();
        let bare = raw
            .strip_prefix("- ")
            .or_else(|| raw.strip_prefix('-'))
            .unwrap_or(raw);
        block.push_str(bare.trim_start());
        block.push('\n');
    }
    read_field_block_str(&block, line_of(source, sentinel.end), declared, findings)
}

/// [`read_field_block`] over an owned, already-de-framed string (used for body /
/// item field groups, whose bullets the caller de-bulleted). `base_line` is the
/// source line the block's first line falls on, for located findings.
fn read_field_block_str(
    block: &str,
    base_line: usize,
    declared: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<crate::field_block::Field> {
    match crate::field_block::parse(block) {
        Ok(fb) => {
            let mut out = Vec::new();
            for field in fb.fields {
                if declared.iter().any(|d| d == &field.key) {
                    out.push(field);
                } else {
                    findings.push(Finding::blocking(
                        "conformance.unknown-field",
                        unknown_field_message(&field.key, declared),
                        Location::at(base_line, 1),
                    ));
                }
            }
            out
        }
        Err(e) => {
            findings.push(Finding::blocking(
                "conformance.malformed-field-block",
                e.message,
                Location::at(base_line + e.line - 1, 1),
            ));
            Vec::new()
        }
    }
}

/// Parse the `### …{#id}` items of a repeatable section in `[from, region_end)`.
///
/// Each `###` heading opens an item: its frozen `{#id}` anchor (re-scanned from the
/// raw heading source and validated as a slug), its title (heading text), its
/// opaque slot span (heading end → the item's field-group sentinel or the next
/// `###`/section end), and its sentinelled per-item fields. Missing / malformed /
/// duplicate `{#id}` is a located [`Severity::Blocking`] finding.
fn parse_items(
    source: &str,
    blocks: &[Block],
    from: usize,
    region_end: usize,
    repeatable: &crate::schema::Repeatable,
    findings: &mut Vec<Finding>,
) -> Vec<ParsedItem> {
    // The `###` item headings in the section's region, in document order.
    let item_heads: Vec<(usize, usize, &str)> = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level: HeadingLevel::H3,
                range,
                content_start,
                ..
            } if range.start >= from && range.start < region_end => {
                Some((range.start, *content_start, &source[range.clone()]))
            }
            _ => None,
        })
        .collect();

    let item_template = ItemTemplate::from(repeatable);
    let mut items = Vec::new();
    let mut seen: Vec<String> = Vec::new();

    for (idx, (head_start, content_start, raw)) in item_heads.iter().enumerate() {
        let head_line = line_of(source, *head_start);
        // The item body runs to the next `###` item or the section's region end.
        let item_end = item_heads
            .get(idx + 1)
            .map(|(s, _, _)| *s)
            .unwrap_or(region_end);

        // The frozen `{#id}` anchor, re-scanned from the raw heading source.
        let id = match extract_anchor(raw) {
            AnchorRead::Missing => {
                findings.push(Finding::blocking(
                    "conformance.item-anchor-missing",
                    format!(
                        "repeatable item `{}` has no `{{#id}}` anchor",
                        heading_text(raw, true)
                    ),
                    Location::at(head_line, 1),
                ));
                continue;
            }
            AnchorRead::Malformed(found) => {
                findings.push(Finding::blocking(
                    "conformance.item-anchor-malformed",
                    format!(
                        "malformed `{{#id}}` anchor `{{#{found}}}` (must be a slug `[a-z0-9-]`)"
                    ),
                    Location::at(head_line, 1),
                ));
                continue;
            }
            AnchorRead::Ok(id) => id,
        };
        if seen.iter().any(|s| s == &id) {
            findings.push(Finding::blocking(
                "conformance.item-anchor-duplicate",
                format!("duplicate `{{#id}}` anchor `{{#{id}}}` in repeatable section"),
                Location::at(head_line, 1),
            ));
            continue;
        }
        seen.push(id.clone());

        // The item's slot prose: heading end → its field-group sentinel or item end.
        let slot_end = body_boundary(blocks, *content_start, item_end);
        let span = trim_span(source, *content_start, slot_end);
        for v in ceiling_violations(blocks, span.start, span.end) {
            findings.push(v);
        }

        // The item's per-item fields (sentinelled bullet group), against its block.
        let item_fields = if item_template.has_fields {
            read_field_group(
                source,
                blocks,
                *content_start,
                item_end,
                &item_template.field_keys,
                findings,
            )
        } else {
            Vec::new()
        };

        items.push(ParsedItem {
            id,
            title: heading_text(raw, true),
            slot: if item_template.has_slot {
                Some(span)
            } else {
                None
            },
            fields: item_fields,
        });
    }
    items
}

/// The leaf shape of a repeatable item template: whether it has a slot, its
/// declared field keys (in schema order, excluding the id-source field, which is
/// rendered as the heading not a bullet).
struct ItemTemplate {
    has_slot: bool,
    has_fields: bool,
    field_keys: Vec<String>,
}

impl ItemTemplate {
    fn from(repeatable: &crate::schema::Repeatable) -> Self {
        let mut has_slot = false;
        let mut field_keys = Vec::new();
        for leaf in &repeatable.block {
            match leaf {
                crate::schema::Leaf::Slot { .. } => has_slot = true,
                crate::schema::Leaf::Field(f) => {
                    // The id-source field is the heading, not a trailing bullet.
                    if f.id != repeatable.id_from {
                        field_keys.push(f.id.clone());
                    }
                }
            }
        }
        let has_fields = !field_keys.is_empty();
        ItemTemplate {
            has_slot,
            has_fields,
            field_keys,
        }
    }
}

/// The outcome of re-scanning a `### …` heading's raw source for its `{#id}` anchor.
enum AnchorRead {
    /// No `{#…}` anchor present (or an empty `{#}`).
    Missing,
    /// An anchor present but not a well-formed slug; carries the literal inner text.
    Malformed(String),
    /// A well-formed frozen slug id.
    Ok(String),
}

/// Re-scan a heading's raw source for a trailing `{#id}` anchor and validate it as a
/// slug. We re-scan the literal `{#…}` (rather than trust pulldown's lenient anchor
/// recovery) so a malformed anchor like `{#Bad Id}` is detected as malformed, not
/// silently truncated. A well-formed anchor matches `slug::slugify(id) == id`.
fn extract_anchor(raw: &str) -> AnchorRead {
    let first_line = raw.lines().next().unwrap_or("");
    let Some((_, after)) = first_line.split_once("{#") else {
        return AnchorRead::Missing;
    };
    let Some((inner, _)) = after.split_once('}') else {
        return AnchorRead::Missing;
    };
    if inner.is_empty() {
        return AnchorRead::Missing;
    }
    if crate::slug::slugify(inner) == inner {
        AnchorRead::Ok(inner.to_string())
    } else {
        AnchorRead::Malformed(inner.to_string())
    }
}

/// Trim surrounding blank-line whitespace off a raw `[start, end)` slot region so
/// the recorded span is exactly the prose bytes (boundary integrity). Leading and
/// trailing ASCII whitespace produced by the canonical blank line after a heading
/// and before the next boundary is excluded; interior bytes are untouched.
fn trim_span(source: &str, start: usize, end: usize) -> Span {
    let slice = &source[start..end];
    let leading = slice.len() - slice.trim_start().len();
    let trimmed_len = slice.trim_end().len();
    let new_start = start + leading;
    let new_end = start + trimmed_len.max(leading);
    Span {
        start: new_start,
        end: new_end,
        start_line: line_of(source, new_start),
    }
}

/// Heading levels that are CLI-owned structural depths, forbidden in slot prose.
fn is_reserved_depth(level: HeadingLevel) -> bool {
    matches!(level, HeadingLevel::H2 | HeadingLevel::H3)
}

/// Collect heading-depth-ceiling violations inside a slot span `[start, end)`:
/// an ATX `##`/`###` heading, or any Setext heading, located by source line.
fn ceiling_violations(blocks: &[Block], start: usize, end: usize) -> Vec<Finding> {
    blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level,
                is_atx,
                range,
                line,
                ..
            } if range.start >= start && range.start < end => {
                if !is_atx {
                    Some(Finding::blocking(
                        "conformance.slot-setext-heading",
                        format!(
                            "Setext heading in slot prose at line {line}; \
                             use `####` ATX depth or rephrase"
                        ),
                        Location::at(*line, 1),
                    ))
                } else if is_reserved_depth(*level) {
                    let depth = if *level == HeadingLevel::H2 {
                        "##"
                    } else {
                        "###"
                    };
                    Some(Finding::blocking(
                        "conformance.slot-heading-depth",
                        format!(
                            "heading at schema-reserved depth `{depth}` in slot prose \
                             at line {line}; use `####` or rephrase"
                        ),
                        Location::at(*line, 1),
                    ))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::load_schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }
    fn commit_schema() -> Schema {
        load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    /// Project each parsed section to `(id, Option<(start, end, opaque prose)>)` for
    /// the goldens — the done-criterion's "section id → slot byte-range + extracted
    /// opaque prose". The tuple is aliased so it reads cleanly and dodges the
    /// type-complexity lint without an `allow`.
    type SectionView<'a> = (&'a str, Option<(usize, usize, &'a str)>);

    fn project<'a>(doc: &'a Document, source: &'a str) -> Vec<SectionView<'a>> {
        doc.sections
            .iter()
            .map(|s| {
                (
                    s.id.as_str(),
                    s.slot
                        .as_ref()
                        .map(|sp| (sp.start, sp.end, sp.slice(source))),
                )
            })
            .collect()
    }

    /// Golden: an ADR fixture whose `context` slot contains a **fenced** `## x`
    /// line and inline `## text`. Parsing against the schema yields the section
    /// list with correct slot byte-ranges, and the fenced `## …` stays inside the
    /// opaque slot span — NOT mis-read as a section heading (the determinism
    /// boundary, proven by a real block parse, not a line scanner). The snapshot
    /// pins each section id → slot span + the re-sliced opaque prose.
    #[test]
    fn sections_adr_fixture_fenced_heading_stays_prose() {
        let src = "\
---
status: accepted
date: 2026-05-23
---

# Rate-limit at the gateway

## Context
Per-client limits were enforced ad hoc.

```
## fenced not a heading
```

A trailing line.

## Decision
Centralize rate limiting at the gateway.

## Consequences
Each service drops its local limiter.
";
        let doc = parse_sections(&adr_schema(), src).expect("conformant ADR parses");

        insta::assert_debug_snapshot!("sections_adr_fixture", project(&doc, src));

        // Boundary integrity: the Context slot re-slices to exactly its prose,
        // including the fenced `## fenced not a heading` line, verbatim.
        let context = doc.sections.iter().find(|s| s.id == "context").unwrap();
        let prose = context.slot.as_ref().unwrap().slice(src);
        assert!(
            prose.contains("## fenced not a heading"),
            "fenced heading kept: {prose:?}"
        );
        assert!(
            prose.starts_with("Per-client limits"),
            "starts at prose: {prose:?}"
        );
        assert!(
            prose.ends_with("A trailing line."),
            "ends before next ##: {prose:?}"
        );
    }

    /// Golden: a `commit` fixture (header section + one body slot) parses to two
    /// sections; the `body` slot span re-slices to the opaque message prose.
    #[test]
    fn sections_commit_fixture() {
        let src = "\
---
subject: Add the rate limiter
---

# Add the rate limiter

## Body
Why this change: centralize limiting.
";
        let doc = parse_sections(&commit_schema(), src).expect("conformant commit parses");
        insta::assert_debug_snapshot!("sections_commit_fixture", project(&doc, src));
    }

    /// Conformance golden: a slot containing a real `## Decision` line (a
    /// schema-reserved `##` depth inside the `context` slot's prose) yields a
    /// located Blocking finding — no instance. The snapshot pins the finding's
    /// code, severity, and located line.
    #[test]
    fn slot_forbidden_heading_yields_located_finding() {
        // The `context` slot's prose illegally contains a `## Decision` heading.
        // Because it parses as a real H2, it is NOT inside the context slot span
        // (it opens a new section region); the conformance failure surfaces as a
        // section-mapping mismatch: the schema expects `decision`/`consequences`
        // headings but finds an extra/renamed one. To target the *ceiling* rule
        // precisely, put a forbidden `### x` (item depth, unused by ADR) inside a
        // slot — it parses as an H3 within the slot span.
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Context
Forces at play.

### Sneaky item heading
This should not be here.

## Decision
We decided.

## Consequences
Fine.
";
        let findings =
            parse_sections(&adr_schema(), src).expect_err("forbidden slot heading blocks");
        // The located ceiling finding must be present.
        let ceiling = findings
            .iter()
            .find(|f| f.code == "conformance.slot-heading-depth")
            .expect("a heading-depth ceiling finding");
        insta::assert_debug_snapshot!("slot_forbidden_heading", ceiling);
    }
}

#[cfg(test)]
mod fields {
    //! Read path for front-matter / body field-groups / repeatable `{#id}` items.
    //!
    //! Goldens pin the parsed front-matter fields, the sentinelled body field
    //! group, and repeatable items (frozen `{#id}` + per-item fields). Conformance
    //! goldens pin one located [`Finding`] per outcomes-table blocking case
    //! (`parsing.md` → Conformance diagnostics). A proptest property pins the
    //! round-trip read invariant: N conformant items → item count + frozen `{#id}`s
    //! preserved, field values verbatim.

    use super::*;
    use crate::field_block::Value;
    use crate::schema::load_schema;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        load_schema(ADR_YAML).expect("adr.yaml loads")
    }

    /// A `spec`-shaped schema with one header section and one repeatable
    /// `criteria` section whose item template is `{ title (id-source), statement
    /// (slot), maps-to-test (code-anchor field) }` — the design's SPEC-criteria
    /// example (`document-type-schema.md` → Sections and repetition).
    fn spec_schema() -> Schema {
        let yaml = b"\
type: spec
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
        - { id: statement, slot: { hint: \"The criterion, testably phrased.\" } }
        - { id: maps-to-test, type: code-anchor }
";
        load_schema(yaml).expect("spec schema loads")
    }

    /// Golden: an ADR fixture parses to its front-matter fields — `status` enum,
    /// `date`, and the `supersedes` **ref** — each read as opaque text mapped to
    /// the header section, plus the three prose slots. Pins the section→fields
    /// projection (the done-criterion's "status front-matter + supersedes ref").
    #[test]
    fn adr_front_matter_fields_map_to_header_section() {
        let src = "\
---
status: accepted
date: 2026-05-23
supersedes: adr:single-node-cache
---

# Rate-limit at the gateway

## Context
Per-client limits were enforced ad hoc.

## Decision
Centralize rate limiting at the gateway.

## Consequences
Each service drops its local limiter.
";
        let doc = parse_sections(&adr_schema(), src).expect("conformant ADR parses");
        let status = doc.sections.iter().find(|s| s.id == "status").unwrap();
        insta::assert_debug_snapshot!("adr_front_matter_fields", &status.fields);
    }

    /// Golden: a repeatable-section fixture parses to its items — each with its
    /// frozen `{#id}`, its mutable title (the `###` heading text), and its
    /// sentinelled per-item fields read verbatim. Pins the item projection (id +
    /// title + slot prose + fields), the done-criterion's "criteria-style items
    /// with their `{#id}` and per-item fields".
    #[test]
    fn repeatable_items_carry_frozen_anchors_and_fields() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Rate limit holds at 100/min  {#rate-limit}
The gateway rejects the 101st request in a 60s window.

<!-- fields -->
- maps-to-test: `test/rate_limit_spec.rb#burst`

### Burst allowance  {#burst-allowance}
A short burst above the limit is tolerated for 2s.
";
        let doc = parse_sections(&spec_schema(), src).expect("conformant spec parses");
        let criteria = doc.sections.iter().find(|s| s.id == "criteria").unwrap();

        // Project items to (id, title, slot prose, fields) for the golden.
        type ItemView<'a> = (
            &'a str,
            &'a str,
            Option<&'a str>,
            &'a [crate::field_block::Field],
        );
        let view: Vec<ItemView> = criteria
            .items
            .iter()
            .map(|it| {
                (
                    it.id.as_str(),
                    it.title.as_str(),
                    it.slot.as_ref().map(|sp| sp.slice(src)),
                    it.fields.as_slice(),
                )
            })
            .collect();
        insta::assert_debug_snapshot!("repeatable_items", view);

        // The frozen anchors are the literal `{#id}` tokens, in order.
        let ids: Vec<&str> = criteria.items.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(ids, ["rate-limit", "burst-allowance"]);
        // The first item's field value is the code-anchor text, verbatim (backticks
        // kept — the schema, not this layer, strips them).
        assert_eq!(criteria.items[0].fields.len(), 1);
        assert_eq!(criteria.items[0].fields[0].key, "maps-to-test");
        assert_eq!(
            criteria.items[0].fields[0].value,
            Value::Scalar("`test/rate_limit_spec.rb#burst`".into())
        );
    }

    /// Conformance golden: a `<!-- fields -->` sentinel with no following bullet
    /// list (orphaned sentinel) → a located Blocking finding.
    #[test]
    fn orphaned_sentinel_yields_located_finding() {
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Context
Forces at play.

<!-- fields -->

## Decision
We decided.

## Consequences
Fine.
";
        let findings = parse_sections(&adr_schema(), src).expect_err("orphaned sentinel blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.orphaned-sentinel")
            .expect("an orphaned-sentinel finding");
        insta::assert_debug_snapshot!("orphaned_sentinel", f);
    }

    /// Conformance golden: a body field group whose bullet key is not a declared
    /// field of its section (unknown field key) → a located Blocking finding.
    #[test]
    fn unknown_field_key_yields_located_finding() {
        // ADR `context` is a slot-only section with no declared fields; any field
        // key under it is unknown.
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Context
Forces at play.

<!-- fields -->
- nonsense: value

## Decision
We decided.

## Consequences
Fine.
";
        let findings = parse_sections(&adr_schema(), src).expect_err("unknown field key blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.unknown-field")
            .expect("an unknown-field finding");
        insta::assert_debug_snapshot!("unknown_field_key", f);
    }

    /// Conformance golden: a repeatable `###` item with no `{#id}` anchor → a
    /// located Blocking finding (missing anchor).
    #[test]
    fn missing_anchor_yields_located_finding() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### A criterion with no anchor
Body.
";
        let findings = parse_sections(&spec_schema(), src).expect_err("missing anchor blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.item-anchor-missing")
            .expect("a missing-anchor finding");
        insta::assert_debug_snapshot!("missing_anchor", f);
    }

    /// Conformance golden: two repeatable items sharing one `{#id}` → a located
    /// Blocking finding (duplicate anchor).
    #[test]
    fn duplicate_anchor_yields_located_finding() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### First  {#dup}
Body one.

### Second  {#dup}
Body two.
";
        let findings = parse_sections(&spec_schema(), src).expect_err("duplicate anchor blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.item-anchor-duplicate")
            .expect("a duplicate-anchor finding");
        insta::assert_debug_snapshot!("duplicate_anchor", f);
    }

    /// Conformance golden: a repeatable item whose `{#id}` is not a well-formed
    /// slug (`{#Bad Id}`) → a located Blocking finding (malformed anchor).
    #[test]
    fn malformed_anchor_yields_located_finding() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### A criterion  {#Bad Id}
Body.
";
        let findings = parse_sections(&spec_schema(), src).expect_err("malformed anchor blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.item-anchor-malformed")
            .expect("a malformed-anchor finding");
        insta::assert_debug_snapshot!("malformed_anchor", f);
    }

    /// Conformance golden: a trailing bullet *field group* with no `<!-- fields -->`
    /// sentinel is unambiguously prose by the marker rule — so a section whose
    /// declared fields are supplied as a bare bullet list (no sentinel) leaves the
    /// fields unread, and a *required* presence is a finalize concern. But the
    /// inverse blocking case the outcomes table names is **a field group without a
    /// sentinel**: the header section's front-matter is fenced, but a body field
    /// group authored without the sentinel cannot be a field group. We assert the
    /// positive marker contract: bullets without a sentinel stay inside the slot
    /// span (prose), never silently promoted to fields.
    #[test]
    fn body_bullets_without_sentinel_stay_prose() {
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Context
Forces at play.

- status: TBD
- note: still prose

## Decision
We decided.

## Consequences
Fine.
";
        let doc = parse_sections(&adr_schema(), src).expect("bare bullets are prose, conformant");
        let context = doc.sections.iter().find(|s| s.id == "context").unwrap();
        // No fields read (no sentinel), and the bullets are inside the slot prose.
        assert!(context.fields.is_empty(), "no sentinel ⇒ no fields");
        let prose = context.slot.as_ref().unwrap().slice(src);
        assert!(
            prose.contains("- status: TBD"),
            "bullets kept as prose: {prose:?}"
        );
    }
}

#[cfg(test)]
mod item_prop_tests {
    use super::*;
    use crate::field_block::Value;
    use crate::schema::load_schema;
    use proptest::prelude::*;

    fn spec_schema() -> Schema {
        let yaml = b"\
type: spec
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
        - { id: weight, type: int }
";
        load_schema(yaml).expect("spec schema loads")
    }

    /// One generated conformant item: a **well-formed** slug id (run through
    /// `slugify` so it is canonical — no leading/trailing/doubled `-`, matching the
    /// frozen-anchor invariant the parser enforces), a title, and a single `weight`
    /// field value (opaque text).
    fn item_strategy() -> impl Strategy<Value = (String, String, String)> {
        (
            "[a-z][a-z0-9-]{0,12}"
                .prop_map(|s| crate::slug::slugify(&s))
                .prop_filter("non-empty canonical slug", |s| !s.is_empty()),
            "[A-Z][a-zA-Z0-9 ]{0,20}",
            "[0-9]{1,4}",
        )
    }

    proptest! {
        /// Round-trip read invariant: a conformant spec built from N generated
        /// items parses to exactly N items whose **frozen `{#id}`s are preserved**
        /// (in order) and whose field values are returned **verbatim**. The #1-risk
        /// read property at item granularity (`parsing.md` → Round-trip guarantees).
        #[test]
        fn n_items_preserve_anchors_and_field_values(
            items in prop::collection::vec(item_strategy(), 1..6)
                .prop_filter("unique anchors", |v| {
                    let ids: Vec<&String> = v.iter().map(|(id, _, _)| id).collect();
                    let mut sorted = ids.clone();
                    sorted.sort();
                    sorted.dedup();
                    sorted.len() == ids.len()
                }),
        ) {
            let mut body = String::from("---\ntitle: T\n---\n\n# T\n\n## Criteria\n\n");
            for (id, title, weight) in &items {
                body.push_str(&format!(
                    "### {title}  {{#{id}}}\nThe statement.\n\n<!-- fields -->\n- weight: {weight}\n\n"
                ));
            }
            let schema = spec_schema();
            let doc = parse_sections(&schema, &body).expect("conformant spec parses");
            let criteria = doc.sections.iter().find(|s| s.id == "criteria").unwrap();

            prop_assert_eq!(criteria.items.len(), items.len());
            for (parsed, (id, _title, weight)) in criteria.items.iter().zip(items.iter()) {
                prop_assert_eq!(&parsed.id, id);
                prop_assert_eq!(parsed.fields.len(), 1);
                prop_assert_eq!(&parsed.fields[0].key, "weight");
                prop_assert_eq!(&parsed.fields[0].value, &Value::Scalar(weight.clone()));
            }
        }
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use crate::schema::load_schema;
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    /// Heading-free slot prose: arbitrary paragraphs and `####`-deep headings, but
    /// never a `##`/`###` ATX heading or a Setext underline (the ceiling), and never
    /// a `<!-- fields -->` sentinel (a section boundary). Lines are joined with
    /// blank-line separators so they parse as distinct blocks.
    fn slot_prose_strategy() -> impl Strategy<Value = String> {
        let line = prop_oneof![
            "[a-zA-Z][a-zA-Z0-9 .,]{0,40}",
            "#### [a-zA-Z][a-zA-Z0-9 ]{0,20}",
        ];
        prop::collection::vec(line, 1..5).prop_map(|lines| lines.join("\n\n"))
    }

    proptest! {
        /// Boundary integrity (the #1-risk round-trip property, read side): for a
        /// conformant ADR built from arbitrary heading-free slot prose, every
        /// recorded slot span re-slices to **exactly** the source prose bytes the
        /// fixture placed there. The parser records opaque spans; the bytes it hands
        /// back are byte-for-byte the bytes that went in.
        #[test]
        fn slot_spans_reslice_to_source_prose(
            context in slot_prose_strategy(),
            decision in slot_prose_strategy(),
            consequences in slot_prose_strategy(),
        ) {
            let schema = load_schema(ADR_YAML).expect("adr.yaml loads");
            let src = format!(
                "---\nstatus: proposed\ndate: 2026-05-31\n---\n\n# Title\n\n\
                 ## Context\n{context}\n\n## Decision\n{decision}\n\n## Consequences\n{consequences}\n"
            );
            let doc = parse_sections(&schema, &src).expect("conformant ADR parses");

            for (id, expected) in [
                ("context", &context),
                ("decision", &decision),
                ("consequences", &consequences),
            ] {
                let sec = doc.sections.iter().find(|s| s.id == id).expect("section present");
                let span = sec.slot.as_ref().expect("slot present");
                let got = span.slice(&src);
                // The span re-slices to the trimmed prose; the canonical fixture
                // places the prose contiguously, so trimmed == the prose itself.
                prop_assert_eq!(got, expected.trim());
            }
        }
    }
}
