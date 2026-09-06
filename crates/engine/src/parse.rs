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

use crate::finding::{Finding, Location, Severity};
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
    /// The item body's opaque slot span, when the item template declares **exactly
    /// one** slot leaf (the bare-prose form — the whole item body is the slot). A
    /// multi-slot template (`slots` non-empty) carries `None` here; a slot-less
    /// template carries `None` too.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<Span>,
    /// The per-leaf slot spans, in schema block order, when the item template
    /// declares **>1** slot leaf (M16 multi-slot, `methodology-docs.md` → the
    /// `roadmap` two-slot entry). Each `(leaf_id, span)` is the opaque prose under
    /// that leaf's `#### <Leaf-Title>` sub-heading. Empty for a single-slot or
    /// slot-less template (the bare-prose `slot` carries the single-slot case).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<(String, Span)>,
    /// The item's sentinelled per-item fields, in physical order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<crate::field_block::Field>,
    /// The item's **nested** repeatable items, in physical order, when the item
    /// template declares a [`crate::schema::Leaf::Repeatable`] (the M22 multi-level
    /// lift — a release's nested change-groups). Each nested item renders one
    /// heading level deeper than this item (`2 + nesting-depth`, capped at H6) and
    /// is itself a [`ParsedItem`], so nesting recurses to the depth cap. Empty —
    /// and skip-on-serialize — for a flat single-level item, so every shipped
    /// single-level golden's bytes are unchanged (the additive guard).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ParsedItem>,
}

impl ParsedItem {
    /// The opaque span of the slot leaf `leaf_id`: the multi-slot `slots` entry when
    /// the template declares >1 slot, else the single bare-prose `slot` (whose leaf
    /// id is the item template's one slot id). Returns `None` when no such slot leaf
    /// is present. The lookup the conformance + item-slot-splice paths address by.
    pub fn slot_span(&self, leaf_id: &str) -> Option<&Span> {
        if self.slots.is_empty() {
            // Single-slot (bare-prose) template: the one slot leaf is the body.
            return self.slot.as_ref();
        }
        self.slots
            .iter()
            .find(|(id, _)| id == leaf_id)
            .map(|(_, span)| span)
    }
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
    // One structural scan per parse — blocks, newline index, block order — shared by
    // every helper below. See [`Scan`] for why the two whole-document rescans it
    // replaces made this function quadratic in its own input.
    let scan = Scan::new(source);
    let blocks = scan.blocks();
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
        // validating each key against the header section's declared fields. The
        // block's findings are collected apart so the **header section hop** can be
        // prefixed onto each ([`prefix_hop`] — the outward assembly).
        let header = &schema.sections[idx];
        let declared = declared_field_keys(header);
        let mut header_findings = Vec::new();
        let header_fields = match blocks.first() {
            Some(Block::Metadata { content }) => {
                block_cursor = 1;
                read_field_block(
                    source,
                    &scan,
                    content.clone(),
                    &declared,
                    &mut header_findings,
                )
            }
            // Absent front-matter is a finalize/field concern, not a block-structure
            // conformance error at this layer.
            _ => Vec::new(),
        };
        prefix_hop(&mut header_findings, &header.id);
        findings.append(&mut header_findings);
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
    // every deeper heading is resolved *within* its section — both are excluded from
    // the section-heading list so index-based matching aligns sections to `##`s.
    //
    // Deeper is not a synonym for prose (M49 D4): a `###` opens an item, and below that
    // a heading may be a declared slot sub-label, a nested item head, or a reserved-depth
    // defect. Which one it is is a **schema** question, answered per item against its
    // template ([`is_item_slot_sub_label`]) — never a depth this filter could read off.
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
        // This section's findings, collected apart so the **section hop** can be prefixed
        // onto each one on the way out ([`prefix_hop`] — the outward assembly; this loop
        // is the only place that holds `section.id`).
        let mut section_findings: Vec<Finding> = Vec::new();

        let Some(Block::Heading {
            text,
            line,
            content_start,
            ..
        }) = headings.get(i).copied()
        else {
            section_findings.push(Finding::blocking(
                "conformance.section-missing",
                format!("required section heading `## {}` is missing", section.id),
                Location::at(1, 1),
            ));
            prefix_hop(&mut section_findings, &section.id);
            findings.append(&mut section_findings);
            continue;
        };

        // The schema-fixed heading text must match the section id (case-folded,
        // trimmed) — sections match by their schema-fixed headings, not position
        // alone, so a renamed heading is a located error.
        if !heading_matches(text, &section.id) {
            section_findings.push(Finding::blocking(
                "conformance.section-renamed",
                format!(
                    "section heading {text:?} does not match required section `{}`",
                    section.id
                ),
                Location::at(*line, 1),
            ));
        }

        // The section's region runs from after its heading to the next `##` (or EOF).
        let region_end = next_section_heading(&scan, *content_start).unwrap_or(source.len());

        match &section.body {
            // A repeatable section: parse its `### …{#id}` items in the region.
            // Top-level items are nesting-depth 1 (`###`); a nested repeatable
            // recurses one level deeper (`####`, …).
            SectionBody::Repeatable { repeatable } => {
                let items = parse_items(
                    source,
                    &scan,
                    *content_start,
                    region_end,
                    repeatable,
                    1,
                    &mut section_findings,
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
                let slot_end = body_boundary(&scan, *content_start, region_end);
                let span = trim_span(source, &scan, *content_start, slot_end);

                // Heading-depth ceiling inside the located slot span. A section slot
                // reserves `##` (sections) + `###` (items) — `reserved_max = 3`. The
                // violation's subject is the section's own slot prose, so it takes no hop
                // below the section (`command-output-contract.md` → the owning slot's
                // address; declared non-unique, per slot).
                for v in ceiling_violations(&scan, span.start, span.end, 3) {
                    section_findings.push(v);
                }

                // Read a trailing field group (sentinel + following list), if any,
                // against the section's declared fields.
                let declared = declared_field_keys(section);
                let section_fields = read_field_group(
                    source,
                    &scan,
                    *content_start,
                    region_end,
                    &declared,
                    &mut section_findings,
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

        prefix_hop(&mut section_findings, &section.id);
        findings.append(&mut section_findings);
    }

    if findings.is_empty() {
        Ok(Document { sections: parsed })
    } else {
        Err(findings)
    }
}

/// Prefix one address `hop` onto every finding in `findings` — the outward-assembly step
/// that gives each parse-`conformance.*` finding its **discriminating fragment**
/// ([command-output-contract.md](../../../design/command-output-contract.md) → the
/// parse-conformance sub-table).
///
/// No parse helper receives the address it sits under, so the fragment is assembled
/// **outward**, at the boundary that holds each hop: the innermost helper sets the
/// leaf/item hop it holds ([`read_field_block_str`]'s field key, [`parse_item_slots`]'s
/// slot leaf, [`parse_items`]'s item id), [`parse_sections`] prefixes the section hop (it
/// is the only fn holding `section.id`), and [`crate::validate`]'s `attribute_to_doc`
/// prefixes the `<type>:<slug>` doc hop. Three stages, one address, no signature churn.
///
/// A finding with no address yet becomes addressed at the bare `hop`; one that already
/// carries an inner hop is extended to `<hop>/<inner>`.
fn prefix_hop(findings: &mut [Finding], hop: &str) {
    for finding in findings {
        let Some(location) = finding.location.as_mut() else {
            continue; // a parse finding is always located; nothing to hang an address on.
        };
        location.address = Some(match location.address.as_deref() {
            Some(inner) => format!("{hop}/{inner}"),
            None => hop.to_string(),
        });
    }
}

/// A schema section is the header (front-matter) section iff its `header` flag is set.
fn is_header_section(section: &crate::schema::Section) -> bool {
    section.header
}

/// Whether a heading's rendered text is the schema-fixed heading for `section_id`.
///
/// The section id is a frozen slug; the writer renders it as a Title-Cased heading
/// (hyphen-split words joined by a space — [`crate::write`] `heading_text`). The
/// match is the **true inverse**: [`crate::slug::renormalize`] the heading text (the
/// separator map alone) and compare to the (already-slugged) id, so a multi-word id
/// like `unreleased-changes` ↔ heading `Unreleased Changes` round-trips. For a
/// single-word id (`context`) this reduces to a lowercased compare, and an
/// out-of-band upper-case heading (`## CONTEXT`) stays tolerated.
///
/// **Recognition, never the mint rule** (M42 inc 10): `slugify` also caps and drops
/// edge stopwords, so it is not the inverse of `heading_text` — a frozen id like
/// `in-scope` renders `## In Scope`, which `slugify` maps to `scope`, and this
/// compare would report the section *renamed* and abort the whole-doc parse.
fn heading_matches(text: &str, section_id: &str) -> bool {
    crate::slug::renormalize(text) == section_id
}

/// A coarse block: the structural events the schema mapping cares about, each with
/// its byte range and 1-based start line. Inline content and paragraph internals
/// are *not* modelled — slot prose is opaque.
///
/// Crate-visible so the surgical-splice edit path ([`crate::write`]) can locate target
/// spans over the *same* trustworthy block parse (never a line scanner — the
/// determinism boundary).
#[derive(Debug)]
pub(crate) enum Block {
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
///
/// Crate-visible so the surgical-splice edit path ([`crate::write`]) locates target
/// spans over this same block parse rather than a fragile line scanner.
/// Strip a single leading UTF-8 BOM in place — read-tolerance per `parsing.md` →
/// Round-trip guarantees ("BOM — tolerate on read (skip — it would break
/// front-matter detection)"). No-op when absent.
///
/// Applied at the committed-store read boundary ([`crate::store`], [`crate::index`])
/// *before* both parse and slot-slice, so spans stay aligned with the source the
/// caller holds. The working-area copy-in canonicalizes separately
/// ([`crate::write::first_touch_canonicalize`]); this covers the read-only paths
/// that parse committed bytes directly.
pub(crate) fn strip_leading_bom(source: &mut String) {
    if source.starts_with('\u{feff}') {
        source.replace_range(..'\u{feff}'.len_utf8(), "");
    }
}

pub(crate) fn scan_blocks(source: &str) -> Vec<Block> {
    scan_blocks_with(source, &LineIndex::new(source))
}

/// [`scan_blocks`] over a [`LineIndex`] the caller already built — the form
/// [`parse_sections`] uses, so one document is newline-indexed once per parse rather
/// than once here and again there.
fn scan_blocks_with(source: &str, lines: &LineIndex) -> Vec<Block> {
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
            Event::Text(_) if in_metadata => {
                // Accumulate across Text events: under LF the parser emits the
                // whole metadata content as ONE Text event, but under CRLF it
                // emits one PER LINE — keeping only the first silently dropped
                // every front-matter field after the first (the defect the M40
                // methodology byte-stability census surfaced). The content range
                // spans the first event's start to the last event's end.
                meta_content = Some(match meta_content.take() {
                    None => range.clone(),
                    Some(existing) => existing.start..range.end,
                });
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
                let line = lines.line_of(range.start);
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

/// Every `\n` offset in a source, ascending — so "which line is this byte on?" is a
/// binary search instead of a rescan of the whole prefix.
///
/// **Why it exists (M49 Increment 5, T6).** The predecessor was a free
/// `line_of(source, offset)` that counted the newlines in `source[..offset]`: O(offset)
/// per call, and both hot loops call it **per structural element** — [`scan_blocks`]
/// once per heading, [`parse_items`] once per item head and again per slot span. Over a
/// document of n items that is O(n²) *inside a single parse*. It compounds at the
/// [`crate::write`] batch seam, where `jigc doc author` re-parses the growing buffer
/// three or four times per lowered leaf (`design/write-commands.md` → Batch authoring):
/// the whole-document author every `migrate-*` workflow drives came out **cubic**, and a
/// `sample(1)` profile of an 800-item run attributed **98 % of it to `line_of`**.
///
/// The answers are identical by construction, which is why this is a speedup and not a
/// behaviour change: the count of newlines strictly before `offset` is exactly the number
/// of recorded offsets less than it, which is [`slice::partition_point`]'s answer.
struct LineIndex {
    /// Byte offset of every `\n`, ascending (`source.bytes()` yields byte offsets, and
    /// `\n` is never a UTF-8 continuation byte, so no char-boundary care is needed).
    newlines: Vec<usize>,
}

impl LineIndex {
    fn new(source: &str) -> Self {
        LineIndex {
            newlines: source
                .bytes()
                .enumerate()
                .filter_map(|(at, b)| (b == b'\n').then_some(at))
                .collect(),
        }
    }

    /// The 1-based source line a byte offset falls on.
    fn line_of(&self, offset: usize) -> usize {
        self.newlines.partition_point(|&nl| nl < offset) + 1
    }
}

/// The offset a block starts at — the key both [`Scan`] and its consumers order by.
///
/// A [`Block::Metadata`] carries only its **inner content** range; it is never matched by
/// a range-scoped scan (they all key on a heading, a sentinel or a list), so using the
/// content start keeps one total order without inventing a fence offset.
fn block_start(block: &Block) -> usize {
    match block {
        Block::Metadata { content } => content.start,
        Block::Heading { range, .. } => range.start,
        Block::FieldSentinel { range } => range.start,
        Block::List { range, .. } => range.start,
    }
}

/// One source's structural **scan**: the [`Block`] projection plus the two indexes that
/// make the parser's repeated questions cheap — newline offsets behind [`Scan::line_of`],
/// and a start-ordered permutation behind [`Scan::blocks_in`], which turns *"which blocks
/// lie in `[from, end)`?"* into a binary search.
///
/// **Why the order is a separate permutation and not a sort of `blocks` itself.** Every
/// range-scoped scan in this module — the item heads, the field-group sentinel, the slot
/// ceiling, the nested-region boundary — was a `blocks.iter().filter(range)` over the
/// **whole** block list, and each runs **once per item**. Over a document of n items that
/// is a second O(n²) inside one parse, sitting behind the [`LineIndex`] one; with the
/// batch author re-parsing per lowered leaf, it is the term that keeps the whole-document
/// author cubic once the newline rescan is gone. Sorting `blocks` in place would fix it
/// too, but ~20 [`crate::write`] call sites consume [`scan_blocks`]' output in **document
/// order**, so the order lives here instead, where only the parser sees it.
///
/// It cannot simply binary-search `blocks` as-is: [`scan_blocks`] pushes a
/// [`Block::List`] at its **end** event, so a heading nested inside a list item is pushed
/// *before* the list containing it. The vector is almost sorted, never guaranteed sorted
/// — and "almost" is exactly the case a binary search answers wrongly and silently.
struct Scan {
    blocks: Vec<Block>,
    lines: LineIndex,
    /// Indices into `blocks`, ascending by [`block_start`].
    order: Vec<usize>,
}

impl Scan {
    fn new(source: &str) -> Self {
        let lines = LineIndex::new(source);
        let blocks = scan_blocks_with(source, &lines);
        let mut order: Vec<usize> = (0..blocks.len()).collect();
        order.sort_by_key(|&i| block_start(&blocks[i]));
        Scan {
            blocks,
            lines,
            order,
        }
    }

    /// The block projection, in **document order** — what [`scan_blocks`] returns.
    fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// The 1-based source line a byte offset falls on.
    fn line_of(&self, offset: usize) -> usize {
        self.lines.line_of(offset)
    }

    /// The blocks starting in `[from, end)`, **in document order** — the same sequence
    /// the whole-list filter it replaces yielded, reached in `O(log n + hits)`. Callers
    /// that took `.min()` over the filtered starts take the first hit instead: ordered,
    /// the first *is* the minimum.
    fn blocks_in(&self, from: usize, end: usize) -> impl Iterator<Item = &Block> + '_ {
        let at = self
            .order
            .partition_point(|&i| block_start(&self.blocks[i]) < from);
        self.order[at..]
            .iter()
            .map(move |&i| &self.blocks[i])
            .take_while(move |b| block_start(b) < end)
    }
}

/// The 1-based heading-level number (`H1` → 1, …, `H6` → 6) — the depth arithmetic
/// the multi-level item scanner does (`2 + nesting-depth`).
fn level_num(level: HeadingLevel) -> usize {
    level as usize
}

/// The ATX heading level a repeatable's items render at for a given 1-based
/// nesting `depth`: a section is `##` (H2), so depth-`d` items are at level
/// `2 + d` (depth 1 → `###`, depth 2 → `####`, …). Caller is the recursive item
/// scanner; the schema loader already capped `depth` at [`MAX_NESTING_DEPTH`]
/// (H6), so `2 + depth` never exceeds 6.
fn item_level_num(depth: usize) -> usize {
    2 + depth
}

/// The start offset of the next **`##` section heading** at or after `from`, if any.
/// Deeper (`###`/`####`+) headings are *not* section boundaries: `###` opens a
/// repeatable item, and anything below that is resolved **inside** the item against its
/// template — a declared slot sub-label, a nested item head, or a defect — never by depth
/// alone ([`first_nested_heading`] / [`is_item_slot_sub_label`], M49 D4).
fn next_section_heading(scan: &Scan, from: usize) -> Option<usize> {
    scan.blocks_in(from, usize::MAX).find_map(|b| match b {
        Block::Heading {
            level: HeadingLevel::H2,
            range,
            ..
        } => Some(range.start),
        _ => None,
    })
}

/// Whether a heading **opens an item's nested region** — the one boundary the reader
/// ([`first_nested_heading`]) and the writer ([`crate::write::item_own_leaf_region`])
/// both ask for, so the byte at which an item stops owning its own leaves is one answer
/// rather than two implementations of one question.
///
/// **The boundary is the schema-reserved depth, not "deeper" (M49 D4, completed at the
/// M49 completion audit).** Two narrowings live here, and each closed a class:
///
/// 1. **Not a declared slot sub-label.** On a template that is both multi-slot and
///    nested the item's own `#### <Leaf-Title>` sub-label is a deeper heading too: the
///    leaf region ended at the item's first sub-label, so every declared leaf parsed as
///    missing (`conformance.item-slot-label-missing`) and the sub-labels were re-read as
///    anchor-less nested item heads — over bytes jigc's own writer had emitted at exit 0.
/// 2. **At exactly `item_level + 1`, and only where the template nests.** A nested item
///    head renders at exactly one level deeper, never further; anything below that depth
///    is opaque slot prose, which is precisely what the address's own ceiling declares
///    ([`crate::write::slot_ceiling`] — `reserved_max` is a *level*, and `first_allowed`
///    is one deeper). Keying on *"the first heading deeper than the item"* therefore
///    contradicted the ceiling: a heading at the very depth `write.slot-heading-depth`
///    prescribes truncated the item's own region, and the truncation surfaced three
///    ways — a multi-slot item's next sub-label falling outside the region (a **false**
///    `write.non-reparseable` naming a `####` sub-heading the payload never contained), a
///    single-slot nested item's prose silently losing everything past the heading on the
///    pinned read at exit 0, and, on the writer's side, the item's own trailing field
///    group falling outside its leaf region so a second bullet was appended for one key.
///    A template that nests nothing has no nested region at all, so nothing inside it
///    ends the item's own leaves.
///
/// `template` is `None` where the writer's address chain does not resolve against the
/// schema: the declared sub-labels are then unknown, so the depth alone answers — still
/// bounded to `item_level + 1`, the only depth a nested child can render at.
///
/// The combination is **fixed rather than fenced**
/// (`completions/artifacts/M49/settle-record.md` → D4): a pack-load fence would be inert
/// for exactly the manifest-less adopter packs it would need to bind.
pub(crate) fn opens_item_nested_region(
    raw_heading: &str,
    level: usize,
    item_level: usize,
    text: &str,
    template: Option<&ItemTemplate>,
) -> bool {
    if level != item_level + 1 {
        return false;
    }
    match template {
        None => true,
        Some(template) => {
            template.has_nested()
                && !is_item_slot_sub_label(raw_heading, level, item_level, text, template)
        }
    }
}

/// The start offset where this item's **nested region** begins in `[from, region_end)`,
/// if any — the byte its own leaves (slot prose, field group) stop at.
///
/// The rule is [`opens_item_nested_region`]'s, which the writer's boundary asks too;
/// `None` for a template that nests nothing (the item owns its whole region).
///
/// The `has_nested` early-out is a **fast path, not a second statement of the rule**:
/// the predicate answers `false` for every heading under a template that nests nothing,
/// so returning here is the same answer without the range scan. It is kept because this
/// runs once per item per parse and the batch path re-parses per leaf
/// (`tests/author_batch_scaling.rs` — the growth-ratio bound).
fn first_nested_heading(
    source: &str,
    scan: &Scan,
    from: usize,
    region_end: usize,
    item_level: usize,
    template: &ItemTemplate,
) -> Option<usize> {
    if !template.has_nested() {
        return None;
    }
    scan.blocks_in(from, region_end).find_map(|b| match b {
        Block::Heading {
            level, range, text, ..
        } if opens_item_nested_region(
            &source[range.clone()],
            level_num(*level),
            item_level,
            text,
            Some(template),
        ) =>
        {
            Some(range.start)
        }
        _ => None,
    })
}

/// The byte offset where a simple section's slot prose ends: the first field-group
/// sentinel in `[from, region_end)`, else `region_end`. The sentinel marks where
/// the trailing field group begins, so the slot prose stops there.
fn body_boundary(scan: &Scan, from: usize, region_end: usize) -> usize {
    scan.blocks_in(from, region_end)
        .find_map(|b| match b {
            Block::FieldSentinel { range } => Some(range.start),
            _ => None,
        })
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
    scan: &Scan,
    content: Range<usize>,
    declared: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<crate::field_block::Field> {
    let base_line = scan.line_of(content.start);
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

/// The **repeated declared key** message (M49 Inc 1 T3). A declared field carries
/// exactly one value, so a second line for one key is a defect the reader cannot
/// adjudicate: the schema says which key, never which of the two values.
///
/// Its sibling one function up reports a repeated *undeclared* key; a repeated
/// **declared** one had no check at all — `read_field_block_str` pushed every repeat into
/// the parsed field list unchecked, so three values on a `0..1` enum were conformant to
/// the tool and the pinned `doc show --format json` returned whichever `find` reached
/// first, silently (`DECISIONS.md` → 2026-08-28 M49 Increment 1 planning: decomposition,
/// T3).
fn duplicate_field_message(key: &str) -> String {
    format!(
        "field `{key}` appears more than once in this field group — a declared field \
         carries exactly one value, and which one this is cannot be read from the schema"
    )
}

/// The repeated-key **route**. `conformance.*` is route-**exempt**, not route-forbidden
/// ([`crate::finding::is_route_exempt`]) — the exemption covers diagnostics for which any
/// route would be a guess — and this one is not a guess: the repair is to delete the
/// stray line, and the sanction that makes hand-editing a managed file legitimate is
/// already written and already shipped, so it is **lifted, never retyped**
/// ([`crate::file_state::OUT_OF_BAND_SANCTION`], the M46 inc-5 / T2 lesson applied to its
/// own const: two producers agreeing is not two producers sharing a source).
fn duplicate_field_route(key: &str) -> String {
    format!(
        "delete the repeated `{key}:` line, keeping the one value you intend — this is \
         the one case a managed file is yours to hand-edit: {}",
        crate::file_state::OUT_OF_BAND_SANCTION,
    )
}

/// A cheap nearness check: equal ignoring case, or a one-char length difference
/// with a shared prefix. Just enough for a typo hint, never load-bearing.
fn is_near(a: &str, b: &str) -> bool {
    if a.eq_ignore_ascii_case(b) {
        return true;
    }
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    // Char-boundary-safe prefix: slicing `short[..3]` would panic mid-codepoint on
    // a non-ASCII key (the OOB-edit input class this parser must turn into findings,
    // never a panic). Take up to 3 *chars*.
    let prefix: String = short.chars().take(3).collect();
    long.len() - short.len() <= 1 && long.starts_with(&prefix)
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
    scan: &Scan,
    from: usize,
    end: usize,
    declared: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<crate::field_block::Field> {
    let Some(sentinel) = scan.blocks_in(from, end).find_map(|b| match b {
        Block::FieldSentinel { range } => Some(range.clone()),
        _ => None,
    }) else {
        return Vec::new();
    };
    read_marked_field_group(source, scan, &sentinel, declared, findings)
}

/// Read the bullet list immediately following a `<!-- fields -->` `sentinel` as a
/// flat field block. The list must start at the sentinel's end (canonical form:
/// sentinel on its own line, then a contiguous bullet list, no blank line between).
/// No following list → orphaned-sentinel finding.
fn read_marked_field_group(
    source: &str,
    scan: &Scan,
    sentinel: &Range<usize>,
    declared: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<crate::field_block::Field> {
    let list = scan
        .blocks_in(sentinel.end, usize::MAX)
        .find_map(|b| match b {
            Block::List { range, items } => Some((range.clone(), items)),
            _ => None,
        });
    // The list must immediately follow the sentinel (only blank-line whitespace
    // between), else the sentinel is orphaned.
    let following = list.filter(|(range, _)| source[sentinel.end..range.start].trim().is_empty());
    let Some((_, items)) = following else {
        findings.push(Finding::blocking(
            "conformance.orphaned-sentinel",
            "`<!-- fields -->` sentinel without a following bullet list".to_string(),
            Location::at(scan.line_of(sentinel.start), 1),
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
    read_field_block_str(&block, scan.line_of(sentinel.end), declared, findings)
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
            let mut out: Vec<crate::field_block::Field> = Vec::new();
            // The undeclared keys already *reported* in this block. A **repeated** undeclared
            // key is **one** defect and **one** repair (delete the stray line), so it is
            // reported once — however many lines carry it — exactly as a duplicated `{#id}`
            // anchor is ([`parse_items`]'s `reported_duplicate`). Reporting per *line* would
            // emit N byte-identical `(code, target)` keys into one slice: two lines carrying
            // one key address at one `#<section>/<field-key>` **by construction**, so the
            // fragment cannot discriminate them. That is a degenerate key, which the
            // membership test forbids ([`crate::finding::debug_assert_targets_declared`]) —
            // and the contract's declared granularity for this code is per `(section, key)`,
            // not per line (`command-output-contract.md` → the parse-conformance sub-table).
            let mut reported_unknown: Vec<String> = Vec::new();
            // The **declared** keys already reported as repeated, on the same
            // once-per-key rule and for the same reason as `reported_unknown` below.
            let mut reported_duplicate: Vec<String> = Vec::new();
            for field in fb.fields {
                if declared.iter().any(|d| d == &field.key) {
                    if out.iter().any(|f| f.key == field.key)
                        && !reported_duplicate.iter().any(|k| k == &field.key)
                    {
                        findings.push(Finding::graded(
                            Severity::Blocking,
                            "conformance.duplicate-field",
                            duplicate_field_message(&field.key),
                            // The **field-key hop**, as the unknown-key sibling below —
                            // `#<section>/<field-key>` after the outward assembly, the
                            // granularity the contract declares for this locus.
                            Some(Location::addressed(field.key.clone(), base_line, 1)),
                            Some(duplicate_field_route(&field.key).into()),
                        ));
                        reported_duplicate.push(field.key.clone());
                    }
                    // The repeat stays in the parsed set: `parse → emit` is byte-exact
                    // over the fields it read ([`crate::field_block`] → Round-trip), and
                    // dropping a line here would make the reader lossy over exactly the
                    // doc it is refusing. Every consumer resolves a key by first match
                    // (`validate::check_field`), so the extra line changes no verdict but
                    // this one.
                    out.push(field);
                } else if !reported_unknown.iter().any(|k| k == &field.key) {
                    findings.push(Finding::blocking(
                        "conformance.unknown-field",
                        unknown_field_message(&field.key, declared),
                        // The **field-key hop** — the innermost hop this helper holds. Two
                        // undeclared keys in one field group would otherwise carry one key
                        // (`command-output-contract.md` → the parse-conformance sub-table:
                        // `unknown-field` → `#<section>/<field-key>`); the outer hops are
                        // prefixed by [`parse_items`] / [`parse_sections`].
                        Location::addressed(field.key.clone(), base_line, 1),
                    ));
                    reported_unknown.push(field.key);
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

/// Parse the `{#id}` items of a repeatable in `[from, region_end)` at nesting
/// `depth` (1-based; depth-1 items are `###`, depth-2 `####`, … per
/// [`item_level_num`]).
///
/// Each item heading at this depth's level opens an item: its frozen `{#id}`
/// anchor (re-scanned from the raw heading source and validated as a slug), its
/// title (heading text), its opaque slot span(s), its sentinelled per-item fields,
/// and — when the item template carries a nested [`crate::schema::Leaf::Repeatable`]
/// — its **nested items**, parsed recursively one level deeper within this item's
/// byte region (each nested item heading is a sub-item boundary inside the parent).
/// Missing / malformed / duplicate `{#id}` is a located [`Severity::Blocking`]
/// finding — the missing case naming the reserved-depth **cause**
/// ([`unanchored_heading_message`]), the two readings being byte-identical; the
/// duplicate-`seen` set is **scoped to this call** (one parent's
/// items), so the same anchor across two parents is legitimate while two within one
/// parent block — review finding C1).
fn parse_items(
    source: &str,
    scan: &Scan,
    from: usize,
    region_end: usize,
    repeatable: &crate::schema::Repeatable,
    depth: usize,
    findings: &mut Vec<Finding>,
) -> Vec<ParsedItem> {
    let item_level = item_level_num(depth);
    // The item headings at this nesting depth's level, in document order. Deeper
    // headings are sub-items (handled within their parent's region by recursion);
    // shallower ones are excluded by `region_end` (the parent/section bound).
    let item_heads: Vec<(usize, usize, &str)> = scan
        .blocks_in(from, region_end)
        .filter_map(|b| match b {
            Block::Heading {
                level,
                range,
                content_start,
                ..
            } if level_num(*level) == item_level => {
                Some((range.start, *content_start, &source[range.clone()]))
            }
            _ => None,
        })
        .collect();

    let item_template = ItemTemplate::from(repeatable);
    let mut items = Vec::new();
    // The `seen` set is per-parent (this call only) — anchor uniqueness is
    // parent-scoped (review finding C1), so it resets on each recursion.
    let mut seen: Vec<String> = Vec::new();
    // The ids already *reported* as duplicated, per-parent alongside `seen`. A duplicated id
    // is **one** defect and **one** repair, so it is reported once — on the first re-occurrence
    // — however many extra heads carry it (`command-output-contract.md` → the
    // parse-`conformance.*` sub-table). Reporting per extra occurrence would emit N-1
    // byte-identical `(code, target)` keys into one slice: a degenerate key, which the
    // membership test forbids.
    let mut reported_duplicate: Vec<String> = Vec::new();

    for (idx, (head_start, content_start, raw)) in item_heads.iter().enumerate() {
        let head_line = scan.line_of(*head_start);
        // The item body runs to the next same-level item or the region end.
        let item_end = item_heads
            .get(idx + 1)
            .map(|(s, _, _)| *s)
            .unwrap_or(region_end);

        // The frozen `{#id}` anchor, re-scanned from the raw heading source. The three
        // anchor breaks carry **no item hop** — they are raised *before* an item identity
        // exists — so they are pushed straight to the caller's set, where the section hop
        // (and, under nesting, the parent item's) is prefixed. Each sets what it holds:
        // the malformed anchor text / the duplicated id, and nothing for the unanchored
        // heading (declared non-unique — the discriminator is the identity that is
        // missing; `command-output-contract.md` → the parse-conformance sub-table).
        let id = match extract_anchor(raw) {
            AnchorRead::Missing => {
                findings.push(Finding::blocking(
                    "conformance.item-heading-unanchored",
                    unanchored_heading_message(
                        &heading_text(raw, true),
                        item_level,
                        // Before any item at this level has parsed, the heading sits in
                        // the enclosing section's / parent item's region, whose ceiling
                        // reserves exactly this level; inside a preceding sibling's body
                        // that sibling's own ceiling binds, one deeper again when its
                        // template carries sub-labels or a nested repeatable.
                        !items.is_empty()
                            && (item_template.is_multi_slot() || item_template.has_nested()),
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
                    Location::addressed(found, head_line, 1),
                ));
                continue;
            }
            AnchorRead::Ok(id) => id,
        };
        if seen.iter().any(|s| s == &id) {
            if !reported_duplicate.iter().any(|s| s == &id) {
                findings.push(Finding::blocking(
                    "conformance.item-anchor-duplicate",
                    format!("duplicate `{{#id}}` anchor `{{#{id}}}` in repeatable section"),
                    Location::addressed(id.clone(), head_line, 1),
                ));
                reported_duplicate.push(id.clone());
            }
            continue;
        }
        seen.push(id.clone());

        // From here the item HAS an identity, so everything its body raises — its slots,
        // its ceiling violations, its field group, its nested items — is collected apart
        // and carries this item's hop out ([`prefix_hop`]).
        let mut item_findings: Vec<Finding> = Vec::new();

        // When the item nests a repeatable, the item's *own* leaves (slot/fields)
        // occupy only the region before its nested content begins. Bounding the parent's
        // leaf region here keeps a nested `####` group out of the parent's slot span and
        // out of its field-group scan; the nested region `[leaf_end, item_end)` is then
        // parsed recursively below. The boundary is the schema-reserved depth, NOT "the
        // first deeper heading": a multi-slot template's own `#### <Leaf-Title>`
        // sub-labels are deeper headings too, and a heading BELOW the reserved depth is
        // the author's own prose at the depth the ceiling declares free
        // ([`opens_item_nested_region`], M49 D4). The predicate answers `false` for a
        // template that nests nothing, so a flat item keeps its whole region without a
        // second statement of that fact here.
        let leaf_end = first_nested_heading(
            source,
            scan,
            *content_start,
            item_end,
            item_level,
            &item_template,
        )
        .unwrap_or(item_end);

        // The item's slot prose. A **multi-slot** template splits the body by its
        // `<Leaf-Title>` sub-headings one level deeper (one span per slot leaf,
        // schema order); a single-slot template keeps the whole leaf region as one
        // bare-prose span.
        let body_end = body_boundary(scan, *content_start, leaf_end);
        let (single_slot, multi_slots) = if item_template.is_multi_slot() {
            let slots = parse_item_slots(
                source,
                scan,
                *content_start,
                body_end,
                item_level,
                &item_template.slot_ids,
                &mut item_findings,
            );
            (None, slots)
        } else if item_template.has_slot() {
            let span = trim_span(source, scan, *content_start, body_end);
            // A single-slot item's prose belongs to its one slot leaf, so a ceiling
            // violation carries that **leaf hop** — the owning slot's address
            // (`command-output-contract.md` → the parse-conformance sub-table).
            let mut ceiling = ceiling_violations(scan, span.start, span.end, item_level);
            prefix_hop(&mut ceiling, &item_template.slot_ids[0]);
            item_findings.append(&mut ceiling);
            (Some(span), Vec::new())
        } else {
            (None, Vec::new())
        };

        // The item's per-item fields (sentinelled bullet group), against its block.
        // Read unconditionally — a template with NO declared bullet fields (e.g. the
        // change-group: id-source heading + slot) still scans a stray sentinel, so an
        // undeclared key spliced into such an item is an unknown-field break, not
        // silently committed bytes (M40 triage; the simple-section path's discipline).
        let item_fields = read_field_group(
            source,
            scan,
            *content_start,
            leaf_end,
            &item_template.field_keys,
            &mut item_findings,
        );

        // Nested repeatables: each declared nested leaf parses its own items one
        // level deeper, within this item's **nested** region `[leaf_end, item_end)` —
        // never from `content_start`, which on a multi-slot template would read the
        // item's own unanchored `#### <Leaf-Title>` sub-labels as anchor-less nested
        // items. For a template whose leaves render no heading the two starts coincide,
        // so the shipped nested corpora parse byte-identically. The schema loader caps
        // the depth at H6, so the recursion terminates. Nested items render in document
        // order (the recursive scan), each carrying its own parent-scoped `seen` set.
        let mut nested_items = Vec::new();
        for (nested_id, nested) in &item_template.nested {
            // The nested repeatable's findings are collected APART so its own leaf id can
            // be prefixed before they join the parent's set. That hop is what makes the
            // composed address the canonical `<section>/<item>/<nested>/<child>/<leaf>` the
            // write verbs accept and `validate::check_item_leaves` already builds on the
            // adjudication side (`command-output-contract.md` → the parse-conformance
            // sub-table). Without it a break inside a change-group composed
            // `#releases/1-0-0/added/bogus`, which parses as a grammar but names nothing —
            // the address the tool printed could not be pasted back into the tool (N29).
            // Prefixing HERE, at the recursion site, is what makes the repair hold at every
            // nesting depth and for every declared nested leaf rather than for the one
            // nesting doctype that ships.
            let mut nested_findings: Vec<Finding> = Vec::new();
            nested_items.extend(parse_items(
                source,
                scan,
                leaf_end,
                item_end,
                nested,
                depth + 1,
                &mut nested_findings,
            ));
            prefix_hop(&mut nested_findings, nested_id);
            item_findings.append(&mut nested_findings);
        }

        // This item's hop, prefixed onto everything its body raised (a nested item's own
        // hop and its nested section's are already inside, so the address nests:
        // `<item>/<nested>/<child>/<leaf>`).
        prefix_hop(&mut item_findings, &id);
        findings.append(&mut item_findings);

        items.push(ParsedItem {
            id,
            title: heading_text(raw, true),
            slot: single_slot,
            slots: multi_slots,
            fields: item_fields,
            items: nested_items,
        });
    }
    items
}

/// Split a **multi-slot** item body `[from, body_end)` into per-leaf slot spans by
/// its `<Leaf-Title>` sub-headings, in schema block order.
///
/// Each declared slot leaf's sub-heading (the leaf id title-cased, matching the writer)
/// opens its slot at the **leaf-label level** — one deeper than the item
/// (`item_level + 1`: `####` for a `###` item, `#####` for a nested `####` one); the
/// slot's opaque prose runs from after that heading to the next sub-heading at that
/// level (or `body_end`). A heading **deeper than the leaf-label level** stays opaque
/// slot-internal content; only one *at* it delimits a slot — the same discipline by
/// which the item level delimits items, one level down. A declared leaf whose
/// sub-heading is absent is a located [`Severity::Blocking`] conformance finding (the
/// skeleton the writer mints always carries every sub-heading, so an absent one is an
/// out-of-band malformation).
fn parse_item_slots(
    source: &str,
    scan: &Scan,
    from: usize,
    body_end: usize,
    item_level: usize,
    slot_ids: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<(String, Span)> {
    // The per-leaf sub-headings sit one level deeper than the item (`item_level + 1`
    // — `####` for a `###` item), in document order: `(start, content_start, label)`.
    let label_level = item_level + 1;
    let sub_heads: Vec<(usize, usize, String)> = scan
        .blocks_in(from, body_end)
        .filter_map(|b| match b {
            Block::Heading {
                level,
                range,
                content_start,
                text,
                ..
            } if level_num(*level) == label_level => {
                Some((range.start, *content_start, text.clone()))
            }
            _ => None,
        })
        .collect();

    let mut slots = Vec::new();
    for leaf_id in slot_ids {
        // The slot's `#### <Leaf-Title>` heading — matched case-insensitively against
        // the title-cased leaf id (the writer emits `#### Proves` for `proves`).
        let Some(pos) = sub_heads
            .iter()
            .position(|(_, _, label)| heading_matches_label(label, leaf_id))
        else {
            findings.push(Finding::blocking(
                "conformance.item-slot-label-missing",
                // The heading is spelled at **`label_level`** — the depth this matcher
                // binds and `write::render_item_at` emits its `leaf_hashes` at — never a
                // global `####`. At a nested multi-slot item the writer emits `#####`, so
                // a message naming `####` instructed the author to write a heading at the
                // schema-reserved item depth, which `conformance.item-heading-unanchored`
                // then blocks and `write::slot_ceiling` refuses on the write path: the
                // instruction left this finding standing AND minted a second one (N30, the
                // un-swept sibling of the shadow guard's depth repair below).
                format!(
                    "multi-slot item is missing its `{} {}` sub-heading",
                    "#".repeat(label_level),
                    title_case(leaf_id)
                ),
                // The **leaf hop** — several declared leaves can be missing from one item,
                // so each keys at its own `(item, leaf)` (`command-output-contract.md` →
                // the parse-conformance sub-table). [`parse_items`] prefixes the item hop.
                Location::addressed(leaf_id.clone(), scan.line_of(from), 1),
            ));
            continue;
        };
        let content_start = sub_heads[pos].1;
        // The slot's prose ends at the next `####` sub-heading or the body end.
        let slot_end = sub_heads
            .get(pos + 1)
            .map(|(s, _, _)| *s)
            .unwrap_or(body_end);
        let span = trim_span(source, scan, content_start, slot_end);
        // Heading-depth ceiling inside this leaf's slot prose. A multi-slot item's
        // `#### <Leaf-Title>` sub-labels sit one level deeper than the item, so its
        // slot prose reserves through `item_level + 1` — the same context-derived
        // ceiling the write side computes (`write::slot_ceiling`, the `multi_slot`
        // arm). This catches an OOB reserved-depth or Setext heading a jigc-authored
        // write would have rejected (the read/parse-side sibling of the single-slot
        // scan). A `#### ` line-start is the delimiter itself (never inside a span);
        // the shadow guard below handles a stray one.
        let mut ceiling = ceiling_violations(scan, span.start, span.end, item_level + 1);
        prefix_hop(&mut ceiling, leaf_id);
        findings.append(&mut ceiling);
        slots.push((leaf_id.clone(), span));
    }

    // Shadow guard (M16): in a multi-slot item every `#### ` line-start is the writer's
    // per-leaf slot delimiter, so any `#### ` sub-heading whose label does **not** match
    // a declared leaf is authored slot prose that shadows the delimiter — the parser
    // would (mis)read it as a slot boundary and silently corrupt the round-trip
    // (`render(parse(x)) != x`). Detect each such stray `#### ` and emit a located
    // Blocking conformance finding, the same shadow-conformance discipline as the
    // reserved-marker checks (`run-marker-not-shadowed` &c.) — turning silent
    // corruption into a clear, routed block. (A heading deeper than the leaf-label
    // level is opaque slot-internal content and is never collected here, so it is
    // unaffected.)
    //
    // The message names the two depths **at this address** — the delimiter shadowed
    // (`label_level`) and the shallowest free depth (`label_level + 1`) — the same
    // context-derived repair [`ceiling_violations`] renders, and for the same reason it
    // records there: *never a global `####`*. At a nested multi-slot item the delimiter
    // is `#####`, so a message naming `####`/`#####` would send the author to demote a
    // stray into a *second* shadowing delimiter (M49 Increment 1 T7 — the un-swept
    // sibling of that fix).
    let delimiter = "#".repeat(label_level);
    let free = "#".repeat(label_level + 1);
    for (start, _, label) in &sub_heads {
        if !slot_ids
            .iter()
            .any(|leaf_id| heading_matches_label(label, leaf_id))
        {
            findings.push(Finding::blocking(
                "conformance.item-slot-delimiter-shadowed",
                format!(
                    "`{delimiter} {}` in multi-slot item prose shadows the item-slot \
                     delimiter; slot prose must not start a line with `{delimiter} ` \
                     (use `{free}`+ or rephrase)",
                    label.trim()
                ),
                Location::at(scan.line_of(*start), 1),
            ));
        }
    }

    slots
}

/// Case-insensitive, whitespace-trimmed match of a `#### <label>` sub-heading's text
/// against a slot leaf id — the multi-slot dual of [`heading_matches`] for sections.
/// The writer title-cases the leaf id (`proves` → `Proves`); the read compare folds
/// case so the title-cased heading maps back to the leaf id.
///
/// `pub(crate)` since M49: the **writer's** item-leaf boundary
/// ([`crate::write::item_own_leaf_region`]) asks the same question the reader does —
/// *is this deeper heading one of this template's declared slot sub-labels?* — and the
/// two answers must be one function, or a heading the parser reads as slot structure is
/// a region boundary to the writer (the multi-slot field-duplication class, M49 D3(A)).
pub(crate) fn heading_matches_label(label: &str, leaf_id: &str) -> bool {
    label.trim().eq_ignore_ascii_case(leaf_id.trim())
}

/// Whether a heading inside an item's sub-tree is that item's own **declared slot
/// sub-label** — the one deeper-heading kind that does *not* end the item's own leaf
/// region, on either seam.
///
/// **One function, both seams (M49 D3(A)/D4).** The reader asks it to find where an
/// item's nested region starts ([`first_nested_heading`]); the writer asks it to find
/// where an item's field group may live ([`crate::write::item_own_leaf_region`]). Two
/// implementations of one question is how the class opened: a heading the parser reads
/// as slot structure was a region boundary to the writer.
///
/// Four conjuncts, each load-bearing:
/// - **the template is multi-slot** — a single-slot item renders bare prose under no
///   sub-heading, so nothing at any depth inside it is a sub-label; exempting its one
///   leaf id would silently absorb a heading at a depth the write path reserves
///   ([`crate::write::slot_ceiling`]'s `multi_slot || has_nested` arm),
/// - **exactly one level deeper** — the depth `render_item_at` emits sub-labels at
///   (`item_level + 1`), which is also the depth a nested item renders at; a heading
///   deeper still is slot-internal prose structure, and nothing at any other depth
///   competes with a nested item,
/// - **unanchored** — an `{#id}` anchor is what makes the heading a nested *item*, whose
///   block the parent's leaf region must stop before. **The declared bound:** on a
///   malformed corpus the discriminator is undefined for exactly the documents
///   `conformance.item-heading-unanchored` / `conformance.item-anchor-malformed` exist to
///   report. jigc's own writer always anchors, so the write path never meets it; the read
///   path answers with a **conformance finding rather than a guess** — an unanchored
///   stray ends the region and is reported at its line, and a *malformed* anchor is still
///   an anchor, so the heading is a broken nested item rather than a re-read sub-label.
/// - **a declared leaf title** — matched through [`heading_matches_label`], the same
///   compare [`parse_item_slots`] splits the body with.
pub(crate) fn is_item_slot_sub_label(
    raw_heading: &str,
    level: usize,
    item_level: usize,
    text: &str,
    template: &ItemTemplate,
) -> bool {
    template.is_multi_slot()
        && level == item_level + 1
        && matches!(extract_anchor(raw_heading), AnchorRead::Missing)
        && template
            .slot_ids
            .iter()
            .any(|leaf_id| heading_matches_label(text, leaf_id))
}

/// Title-case a single-word leaf id for the `#### <Leaf-Title>` sub-heading (mirrors
/// the writer's `heading_text` for a single-word id). Leaf ids are single words
/// (the same constraint section ids follow), so a single capitalization suffices.
fn title_case(id: &str) -> String {
    let mut chars = id.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// The leaf shape of a repeatable item template: its declared slot leaf ids (in
/// schema block order), its declared field keys (in schema order, excluding the
/// id-source field, which is rendered as the heading not a bullet).
///
/// **Single vs multi-slot** is the count of `slot_ids`: exactly one → the
/// bare-prose form (the whole item body is the slot, byte-identical backward-compat
/// with every existing single-slot doctype); more than one → each slot renders
/// under its own `#### <Leaf-Title>` sub-heading (M16).
pub(crate) struct ItemTemplate {
    pub(crate) slot_ids: Vec<String>,
    field_keys: Vec<String>,
    /// The block's nested repeatables, in schema block order (the M22 multi-level
    /// lift), each paired with **its own leaf id** — the nested section's stable id,
    /// which is a hop in every address that reaches into it. Each parses its own items
    /// one level deeper within the parent item's region; empty for a flat single-level
    /// template.
    nested: Vec<(String, crate::schema::Repeatable)>,
}

impl ItemTemplate {
    pub(crate) fn from(repeatable: &crate::schema::Repeatable) -> Self {
        let mut slot_ids = Vec::new();
        let mut field_keys = Vec::new();
        let mut nested = Vec::new();
        for leaf in &repeatable.block {
            match leaf {
                crate::schema::Leaf::Slot { id, .. } => slot_ids.push(id.clone()),
                crate::schema::Leaf::Field(f) => {
                    // The id-source field is the heading, not a trailing bullet.
                    if f.id != repeatable.id_from {
                        field_keys.push(f.id.clone());
                    }
                }
                // A nested repeatable: its items parse one level deeper, within the
                // parent item's region (the depth-aware parse lift, M22 inc-1 T2).
                crate::schema::Leaf::Repeatable {
                    id,
                    repeatable: inner,
                } => nested.push((id.clone(), inner.clone())),
            }
        }
        ItemTemplate {
            slot_ids,
            field_keys,
            nested,
        }
    }

    /// Whether the template declares at least one slot leaf.
    fn has_slot(&self) -> bool {
        !self.slot_ids.is_empty()
    }

    /// Whether the template declares more than one slot leaf (the multi-slot form,
    /// rendered with `#### <Leaf-Title>` sub-headings).
    pub(crate) fn is_multi_slot(&self) -> bool {
        self.slot_ids.len() > 1
    }

    /// Whether the template declares a nested repeatable (the M22 multi-level form).
    ///
    /// `pub(crate)` since M45: it is the second discriminator of the slot
    /// heading-depth ceiling ([`crate::write::slot_ceiling`]) — a nested-bearing
    /// item's leaves are bounded at the first heading **at `2+d+1`** that is not one of
    /// the item's own declared sub-labels ([`opens_item_nested_region`], M49 T2), so
    /// that level is reserved there exactly as a multi-slot sub-label reserves it, and
    /// everything deeper stays the author's prose.
    pub(crate) fn has_nested(&self) -> bool {
        !self.nested.is_empty()
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
/// silently truncated. A well-formed anchor is any valid slug — [`crate::slug::is_slug`],
/// the *recognition* predicate. (It must **not** re-use `slugify(id) == id`: since the
/// F5 edge-stopword drop, `slugify` no longer round-trips every valid slug — a frozen id
/// like `a-0` or `a-plugin-surface` is a valid slug but not a `slugify` fixed point — and
/// an anchor is a frozen identity to be *recognized*, never re-normalized.)
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
    if crate::slug::is_slug(inner) {
        AnchorRead::Ok(inner.to_string())
    } else {
        AnchorRead::Malformed(inner.to_string())
    }
}

/// Trim surrounding blank-line whitespace off a raw `[start, end)` slot region so
/// the recorded span is exactly the prose bytes (boundary integrity). Leading and
/// trailing ASCII whitespace produced by the canonical blank line after a heading
/// and before the next boundary is excluded; interior bytes are untouched.
fn trim_span(source: &str, scan: &Scan, start: usize, end: usize) -> Span {
    let slice = &source[start..end];
    let leading = slice.len() - slice.trim_start().len();
    let trimmed_len = slice.trim_end().len();
    if trimmed_len == 0 {
        // An empty slot (the whole region is the blank line(s) after the heading): the
        // insertion point sits at the **leading** edge, so the canonical separating
        // whitespace before the next boundary stays *after* the span. This makes the
        // writer the parser's inverse — filling the slot yields `<prose>\n\n## Next`
        // (reparseable) rather than gluing prose onto the following heading.
        let new_start = start;
        return Span {
            start: new_start,
            end: new_start,
            start_line: scan.line_of(new_start),
        };
    }
    let new_start = start + leading;
    let new_end = start + trimmed_len;
    Span {
        start: new_start,
        end: new_end,
        start_line: scan.line_of(new_start),
    }
}

/// The diagnosis for an item heading carrying no `{#id}` anchor
/// (`conformance.item-heading-unanchored`) — it names the **cause** at the offending
/// line: the heading sits at a depth the schema reserves for item structure, so the
/// parser reads it as an item boundary and everything after it is re-attributed
/// (`design/validation.md` → The M45 registrations, row 2).
///
/// The two readings are **byte-identical** — stray slot prose that broke out, or a
/// genuinely anchor-less new item — so the message carries **both** repairs rather
/// than picking one.
///
/// **Both repairs are edits at the named line, and neither is a CLI write** (M46
/// Increment 5). The corruption is precisely what stops the parse, so every write
/// door refuses the doc: `jigc doc add-item` at the containing section — the verb
/// this message named until M46 — answers an unrelated `write.wrong-shape` ("last
/// item not present"), which is a law-1 lie on a blocking path. So the new-item
/// reading names the `{#id}` anchor **to add at that line**, not a verb to run;
/// driven, the anchored heading is a conformant out-of-band edit the reconciler
/// absorbs. That is the same hand repair every carrier of this message already
/// sanctions ([`crate::file_state`]'s hand-repair sanction on both
/// `reconciliation.conformance-block` arms; `store.unparseable`'s *"fix the
/// committed file so it conforms"*).
///
/// `item_level` is the reserved depth (the item heading level at
/// this nesting); `prose_reserves_deeper` says the enclosing item's own ceiling sits
/// one level deeper still (a multi-slot template's `#### <Leaf-Title>` sub-labels, or
/// a nested repeatable's item headings — [`crate::write::slot_ceiling`]'s
/// `multi_slot || has_nested` derivation, read from where the enclosing prose lives),
/// so the demote repair names a depth that is actually free at this address.
fn unanchored_heading_message(
    heading: &str,
    item_level: usize,
    prose_reserves_deeper: bool,
) -> String {
    let depth = "#".repeat(item_level);
    let allowed = "#".repeat(item_level + if prose_reserves_deeper { 2 } else { 1 });
    format!(
        "`{depth} {heading}` sits at `{depth}`, the schema-reserved item depth here, so the \
         parser reads it as an item boundary — and it carries no `{{#id}}` anchor; if that \
         line is slot prose, demote it to `{allowed}` or deeper; if it is a new item, anchor \
         it in place — `{depth} {heading}  {{#<id>}}` — with `<id>` a lowercase-kebab slug \
         unique among this section's items"
    )
}

/// Whether a heading `level` is a CLI-owned structural depth forbidden inside a slot
/// at item heading level `reserved_max`: any level **at or shallower than** the
/// enclosing item's level (`<= reserved_max`) is a section/item structural marker,
/// while a deeper level is allowed slot-internal structure. A section slot passes
/// `reserved_max = 3` (sections `##` + items `###` are reserved); an item slot at
/// level `L` passes `reserved_max = L` (so a depth-2 `####` item's slot forbids
/// `<= ####` but admits `#####`+).
fn is_reserved_depth(level: HeadingLevel, reserved_max: usize) -> bool {
    level_num(level) <= reserved_max
}

/// Collect heading-depth-ceiling violations inside a slot span `[start, end)`: an
/// ATX heading at or shallower than `reserved_max`, or any Setext heading, located
/// by source line. `reserved_max` is the enclosing item's heading level (or `3` for
/// a section slot) — depth-aware so a nested item's slot has a deeper ceiling.
fn ceiling_violations(scan: &Scan, start: usize, end: usize, reserved_max: usize) -> Vec<Finding> {
    scan.blocks_in(start, end)
        .filter_map(|b| match b {
            Block::Heading {
                level,
                is_atx,
                line,
                ..
            } => {
                if !is_atx {
                    // Name the depth that is free **at this address** — `reserved_max +
                    // 1`, the same context-derived first-allowed the write twin renders
                    // (`write::slot_ceiling_finding`), never a global `####`: at a
                    // nested change-group leaf `####` is itself the reserved (corrupting)
                    // depth, so the read-side repair must name `#####` there.
                    let allowed = "#".repeat(reserved_max + 1);
                    Some(Finding::blocking(
                        "conformance.slot-setext-heading",
                        format!(
                            "Setext heading in slot prose at line {line}; \
                             use `{allowed}` ATX depth or rephrase"
                        ),
                        Location::at(*line, 1),
                    ))
                } else if is_reserved_depth(*level, reserved_max) {
                    let depth = "#".repeat(level_num(*level));
                    Some(Finding::blocking(
                        "conformance.slot-heading-depth",
                        format!(
                            "heading at schema-reserved depth `{depth}` in slot prose \
                             at line {line}; use a deeper level or rephrase"
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
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }
    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
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

## Options
Alternatives were weighed and rejected.

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

    /// Regression (M40 methodology byte-stability census): a **CRLF** doc's
    /// multi-field front-matter parses EVERY field, not just the first. Under
    /// CRLF, pulldown-cmark emits the metadata block's content as one `Text`
    /// event **per line** (LF emits a single event for the whole block);
    /// `scan_blocks` kept only the first event's range, so a CRLF ADR silently
    /// dropped `date`/`supersedes` — every field after the first. The scan now
    /// accumulates the Text ranges across the block.
    #[test]
    fn crlf_front_matter_parses_every_field() {
        let lf = "\
---
status: accepted
date: 2026-05-23
supersedes: adr:old-call
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
        let crlf = lf.replace('\n', "\r\n");
        for (label, src) in [("LF", lf.to_string()), ("CRLF", crlf)] {
            let doc = parse_sections(&adr_schema(), &src).expect("conformant ADR parses");
            let header = doc.sections.iter().find(|s| s.id == "status").unwrap();
            let fields: Vec<(&str, String)> = header
                .fields
                .iter()
                .map(|f| (f.key.as_str(), f.value.render()))
                .collect();
            assert_eq!(
                fields,
                vec![
                    ("status", "accepted".to_string()),
                    ("date", "2026-05-23".to_string()),
                    ("supersedes", "adr:old-call".to_string()),
                ],
                "{label}: every front-matter field parses with its value"
            );
        }
    }

    /// Golden: a `commit` fixture (header section + `summary`/`body` slots + an
    /// empty `trailers` repeatable section) parses to four sections; the slot spans
    /// re-slice to the opaque subject + message prose.
    #[test]
    fn sections_commit_fixture() {
        let src = "\
---
type: feat
---

# Add the rate limiter

## Summary
Add the rate limiter

## Body
Why this change: centralize limiting.

## Trailers
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

    /// The ceiling sweep's **slot-span boundary** (2026-07-24 mutation audit, finding
    /// #5): a heading can start exactly at `span.end` only in the zero-gap OOB shape —
    /// no blank line and no content between a section heading and the next structural
    /// heading (`## Context\n## Options`), where `trim_span` collapses the empty slot to
    /// `(content_start, content_start)` and the next heading's `range.start` equals it.
    /// The parser accepts that non-canonical shape, and the next section's own `##` must
    /// NOT be reported as a ceiling violation *inside* the empty slot — the `<` (not
    /// `<=`) half-open upper bound. Under `<=` this doc would false-block.
    #[test]
    fn zero_gap_empty_slot_does_not_flag_the_next_heading() {
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Context
## Options
Alternatives were weighed and rejected.

## Decision
We decided.

## Consequences
Fine.
";
        // The zero-gap empty slot parses clean: no `conformance.slot-heading-depth`
        // (or any other) finding against the next section's own `## Options` heading.
        let doc = parse_sections(&adr_schema(), src)
            .expect("the zero-gap empty-slot shape parses without ceiling findings");
        let context = doc
            .sections
            .iter()
            .find(|s| s.id == "context")
            .expect("context section present");
        let prose = context.slot.as_ref().map(|s| s.slice(src)).unwrap_or("");
        assert!(
            prose.is_empty(),
            "the zero-gap context slot is empty, not swallowing `## Options`; got {prose:?}",
        );
    }

    /// A throwaway single-slot doctype whose one body section carries a
    /// **multi-word (hyphenated) section id** — the `## Unreleased Changes`
    /// (`unreleased-changes`) shape the changelog earns. Loaded through the real
    /// YAML path so the test drives the production parser, not a hand-built Schema.
    fn multiword_section_schema() -> Schema {
        let yaml = b"\
type: multiword
location: notes/
id-from: title
sections:
  - id: title
    header: true
    fields:
      - { id: title, type: string }
  - id: unreleased-changes
    slot: { hint: \"Staging area for unreleased changes.\" }
";
        crate::schema::load_schema(yaml).expect("multiword schema loads")
    }

    /// Multi-word section-id round-trip (M22 increment 2 / engine work #2).
    ///
    /// (i) **Regression red** — pre-fix, `heading_matches` flat-compared the
    ///     rendered heading text against the raw section id, so `"Unreleased
    ///     Changes"` never matched `unreleased-changes` and the whole-doc parse
    ///     aborted with `conformance.section-renamed`. The fix re-slugs the
    ///     heading before comparing (`slugify(text) == section_id`), the true
    ///     inverse of the title-casing writer.
    /// (ii) **The fix** — the same multi-word section parses clean (no
    ///     `section-renamed` finding) and round-trips byte-stable.
    #[test]
    fn multiword_section_id_round_trips() {
        // (i) Regression red, pinned as the precise pre-fix behaviour: the OLD
        // flat `eq_ignore_ascii_case` compare returned false for this pair, which
        // is exactly the bug. The NEW `heading_matches` must return true.
        assert!(
            !"Unreleased Changes".eq_ignore_ascii_case("unreleased-changes"),
            "the pre-fix flat compare rejected the multi-word heading (the regression)"
        );
        assert!(
            heading_matches("Unreleased Changes", "unreleased-changes"),
            "the fix: re-slugging the heading matches the hyphenated section id"
        );

        // (ii) The fix end-to-end: a `## Unreleased Changes` doc parses clean and
        // round-trips byte-stable through the real render path.
        let src = "\
---
title: Release tracking
---

# Release tracking

## Unreleased Changes

Pending work not yet cut into a version.
";
        let schema = multiword_section_schema();
        let doc = parse_sections(&schema, src).expect("multi-word section parses clean");
        // No section-renamed finding could have fired (parse returned Ok), and the
        // body section mapped under its hyphenated id.
        assert!(
            doc.sections.iter().any(|s| s.id == "unreleased-changes"),
            "the hyphenated section mapped: {:?}",
            doc.sections.iter().map(|s| &s.id).collect::<Vec<_>>()
        );

        let instance = crate::write::instance_from_source(&schema, src)
            .expect("conformant doc reads to an instance");
        let rendered = crate::write::render(&schema, &instance);
        assert_eq!(rendered, src, "multi-word section round-trips byte-stable");
    }

    /// Additive guard: an existing **single-word** doctype (`adr`) is unaffected.
    /// Its sections still match by their lowercase ids (`slugify(id) == id`), an
    /// out-of-band `## CONTEXT` heading stays case-tolerant, and the conformant
    /// doc parses to the same three body sections it does today.
    #[test]
    fn single_word_sections_unaffected_by_reslug() {
        // slugify(id) == id for every shipped single-word section id, so re-slugging
        // the heading is a true superset of the old flat compare.
        for id in ["context", "decision", "consequences"] {
            assert_eq!(
                crate::slug::slugify(id),
                id,
                "single-word id is its own slug"
            );
            // Title-cased heading still matches.
            let titled = id[..1].to_uppercase() + &id[1..];
            assert!(heading_matches(&titled, id), "{titled:?} matches {id:?}");
        }
        // Case-tolerance for an OOB upper-case heading is preserved.
        assert!(
            heading_matches("CONTEXT", "context"),
            "an OOB `## CONTEXT` heading stays case-tolerant"
        );

        let src = "\
---
status: accepted
date: 2026-05-23
---

# Rate-limit at the gateway

## CONTEXT
Per-client limits were enforced ad hoc.

## Options
Alternatives were weighed and rejected.

## Decision
Centralize rate limiting at the gateway.

## Consequences
Each service drops its local limiter.
";
        let doc = parse_sections(&adr_schema(), src).expect("adr with OOB-case heading parses");
        let body_ids: Vec<&str> = doc
            .sections
            .iter()
            .map(|s| s.id.as_str())
            .filter(|id| *id != "status")
            .collect();
        assert_eq!(body_ids, ["context", "options", "decision", "consequences"]);
    }

    /// A throwaway single-slot doctype whose one body section id carries a
    /// **leading edge stopword** (`in-scope`) — the shape the *mint* rule
    /// mangles. Loaded through the real YAML path so the test drives the
    /// production parser, not a hand-built `Schema`.
    fn edge_stopword_section_schema() -> Schema {
        let yaml = b"\
type: brief
location: notes/
id-from: title
sections:
  - id: title
    header: true
    fields:
      - { id: title, type: string }
  - id: in-scope
    slot: { hint: \"What this brief covers.\" }
";
        crate::schema::load_schema(yaml).expect("edge-stopword schema loads")
    }

    /// M42 inc 10: a **frozen section id** is recognized by *renormalizing* its
    /// rendered heading — never by re-running the **mint** rule over it.
    ///
    /// The mint rule drops a leading edge stopword (M41 F5), so re-slugging the
    /// heading of a section id like `in-scope` does **not** recover the id:
    /// `in-scope` renders `## In Scope`, which `slugify` maps to `scope` — and a
    /// heading↔id compare built on `slugify` therefore reports the section
    /// *renamed* and aborts the whole-doc parse (blocking
    /// `conformance.section-renamed`). Latent today only because no shipped id
    /// trips it — luck, not a fence. Recognition uses `slug::renormalize` (the
    /// separator map, no cap, no stopword drop), which is the true inverse of the
    /// writer's `heading_text`.
    #[test]
    fn edge_stopword_section_id_round_trips() {
        let src = "\
---
title: Cache rewrite
---

# Cache rewrite

## In Scope

The read path only.
";
        let schema = edge_stopword_section_schema();
        let doc = parse_sections(&schema, src)
            .expect("an edge-stopword section id parses clean (no `conformance.section-renamed`)");
        assert!(
            doc.sections.iter().any(|s| s.id == "in-scope"),
            "the edge-stopword section mapped: {:?}",
            doc.sections.iter().map(|s| &s.id).collect::<Vec<_>>()
        );

        let instance = crate::write::instance_from_source(&schema, src)
            .expect("conformant doc reads to an instance");
        assert_eq!(
            crate::write::render(&schema, &instance),
            src,
            "an edge-stopword section id round-trips byte-stable"
        );

        // The hazard the parse above would otherwise hit, pinned at the unit: the
        // *mint* rule is not the inverse of `heading_text` — the *recognition* rule is.
        assert_eq!(
            crate::slug::slugify("In Scope"),
            "scope",
            "the mint rule drops the leading edge stopword — it cannot recognize `in-scope`"
        );
        assert!(
            heading_matches("In Scope", "in-scope"),
            "the rendered heading of a frozen `in-scope` section must match its id"
        );
    }

    /// The **real population**, not a sample: every section id shipped by **both**
    /// packs (the dev pack + the methodology pack) must be recognized from its own
    /// rendered heading — `heading_matches(heading_text(id), id)`, the writer's
    /// rule composed with the parser's. Read off the shipped schema YAML on disk,
    /// so a *new* doctype (or a renamed section) joins the population automatically
    /// rather than waiting for someone to remember this list.
    #[test]
    fn every_shipped_section_id_is_recognized_from_its_rendered_heading() {
        let engine_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let pack_dirs = [
            engine_dir.join("../cli/pack/schemas"),
            engine_dir.join("../../packs/methodology/schemas"),
        ];

        let mut checked = 0usize;
        for dir in &pack_dirs {
            let entries = std::fs::read_dir(dir).unwrap_or_else(|e| panic!("read {dir:?}: {e}"));
            let mut saw_schema = false;
            for entry in entries {
                let path = entry.expect("dir entry").path();
                if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
                    continue;
                }
                saw_schema = true;
                let bytes = std::fs::read(&path).expect("read schema");
                let schema = crate::schema::load_schema_with_types(
                    &bytes,
                    &crate::schema::dev_pack_field_types(),
                )
                .unwrap_or_else(|e| panic!("{path:?} loads: {e:?}"));
                for section in &schema.sections {
                    let heading = crate::write::heading_text(&section.id);
                    assert!(
                        heading_matches(&heading, &section.id),
                        "{path:?}: section `{}` renders `## {heading}`, which the parser \
                         does not recognize as `{}` — the doc would abort with \
                         `conformance.section-renamed`",
                        section.id,
                        section.id,
                    );
                    checked += 1;
                }
            }
            assert!(saw_schema, "no schema YAML found under {dir:?}");
        }
        assert!(
            checked >= 29,
            "the shipped section-id population shrank to {checked} — did a pack move?"
        );
    }

    proptest! {
        /// The recognition round-trip holds **by construction** for every
        /// well-formed slug (`is_slug`): `heading_text` splits on `-`, capitalizes,
        /// and joins with a space; the recognizer lowercases and maps the space
        /// back — no cap, no stopword drop, nothing lossy in between. (The
        /// exhaustive fence over the *shipped* ids is the arm above; this ranges
        /// over the ids a pack could ship.)
        #[test]
        fn recognition_round_trips_every_well_formed_slug(
            id in "[a-z0-9]{1,9}(-[a-z0-9]{1,9}){0,5}"
        ) {
            prop_assert!(crate::slug::is_slug(&id), "generator produced a non-slug: {:?}", id);
            let heading = crate::write::heading_text(&id);
            prop_assert!(
                heading_matches(&heading, &id),
                "`## {}` is not recognized as `{}`", heading, id
            );
        }
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

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
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
        crate::schema::load_schema_with_types(yaml, &crate::schema::dev_pack_field_types())
            .expect("spec schema loads")
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

## Options
Alternatives were weighed and rejected.

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

    /// A `roadmap`-shaped schema: a `milestones` repeatable whose item block carries
    /// TWO prose slots (`proves`, `decomposition`) — the M16 multi-slot shape.
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
        - { id: proves, slot: { hint: \"x\" } }
        - { id: decomposition, slot: { hint: \"y\" } }
";
        crate::schema::load_schema(yaml).expect("two-slot roadmap schema loads")
    }

    /// Golden (M16 multi-slot read): a two-slot item is split by its `#### Proves` /
    /// `#### Decomposition` sub-headings into two per-leaf spans (schema block order),
    /// each re-slicing to exactly its opaque prose. `item.slot` stays `None` (the
    /// single-slot bare-prose carrier); the per-leaf spans live in `item.slots`. A
    /// `#####`-deep heading inside a slot stays opaque slot-internal content.
    #[test]
    fn multi_slot_item_splits_by_sub_label_headings() {
        let src = "\
# Roadmap

## Milestones

### M16 self-hosting  {#m16-self-hosting}

#### Proves

Closes the self-hosting loop.

##### A deeper heading stays opaque

#### Decomposition

Inc 1, Inc 2, Inc 3.
";
        let doc = parse_sections(&two_slot_schema(), src).expect("conformant roadmap parses");
        let milestones = doc.sections.iter().find(|s| s.id == "milestones").unwrap();
        let item = &milestones.items[0];

        // The single-slot bare carrier is unused; the per-leaf spans carry the prose.
        assert!(
            item.slot.is_none(),
            "multi-slot item uses `slots`, not bare `slot`"
        );
        let leaf_ids: Vec<&str> = item.slots.iter().map(|(id, _)| id.as_str()).collect();
        assert_eq!(leaf_ids, ["proves", "decomposition"], "schema block order");

        // Each leaf span re-slices to its opaque prose (the `#####` stays inside proves).
        let proves = item.slot_span("proves").unwrap().slice(src);
        assert!(proves.contains("Closes the self-hosting loop."));
        assert!(
            proves.contains("##### A deeper heading stays opaque"),
            "`#####` stays opaque slot-internal content: {proves:?}",
        );
        let decomp = item.slot_span("decomposition").unwrap().slice(src);
        assert_eq!(decomp.trim(), "Inc 1, Inc 2, Inc 3.");

        // Project for the golden: (id, [(leaf, prose)]).
        type SlotView<'a> = (&'a str, Vec<(&'a str, &'a str)>);
        let view: Vec<SlotView> = milestones
            .items
            .iter()
            .map(|it| {
                (
                    it.id.as_str(),
                    it.slots
                        .iter()
                        .map(|(leaf, sp)| (leaf.as_str(), sp.slice(src)))
                        .collect(),
                )
            })
            .collect();
        insta::assert_debug_snapshot!("multi_slot_items", view);
    }

    /// Conformance golden (M16 multi-slot shadow): a slot's prose that itself starts
    /// a line with `#### Something` shadows the **item-slot delimiter** — in a
    /// multi-slot item, a `#### ` line-start is the writer's per-leaf delimiter, so
    /// authored prose carrying one would be re-parsed as a slot boundary and silently
    /// corrupt the round-trip (`render(parse(x)) != x`). The parser detects the extra
    /// `#### ` (one not matching a declared leaf label) and emits a located Blocking
    /// finding — the same shadow-conformance discipline as the reserved-marker checks
    /// (`run-marker-not-shadowed` &c.), turning silent corruption into a routed block.
    /// A single-slot item is unaffected: there `####` is opaque whole-body prose.
    #[test]
    fn multi_slot_prose_h4_shadows_slot_delimiter() {
        // The `proves` slot's prose carries a `#### Note` line. Today that line is
        // collected as a slot delimiter and ends `proves` early (dropping the rest);
        // the guard must flag it instead.
        let src = "\
# Roadmap

## Milestones

### M16 self-hosting  {#m16-self-hosting}

#### Proves

Closes the self-hosting loop.

#### Note

This stray heading is prose, not a slot delimiter.

#### Decomposition

Inc 1, Inc 2, Inc 3.
";
        let findings =
            parse_sections(&two_slot_schema(), src).expect_err("a shadowing `#### ` blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.item-slot-delimiter-shadowed")
            .expect("an item-slot-delimiter-shadowed finding");
        insta::assert_debug_snapshot!("item_slot_delimiter_shadowed", f);
    }

    /// The shadow guard's **message** names the depth *at this address*, not a global
    /// `####` (M49 Increment 1 T7 — the un-swept sibling of the ceiling message's own
    /// fix, `ceiling_violations`' *"never a global `####`"*). A **nested** multi-slot
    /// item sits at `####`, so its per-leaf delimiter is `#####` and the shallowest
    /// free depth is `######`: a message naming `####`/`#####` there points the author
    /// at the wrong two depths — and following it (demoting the stray to `#####`) would
    /// mint a *second* shadowing delimiter rather than repair the first.
    #[test]
    fn nested_multi_slot_shadow_message_names_this_depth() {
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
              - { id: impact, slot: { hint: \"Who is affected.\" } }
";
        let schema =
            crate::schema::load_schema(yaml).expect("nested multi-slot changelog schema loads");
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

##### Notes

OAuth device-code flow.

##### Stray

This heading is prose, not a slot delimiter.

##### Impact

Everyone.
";
        let findings = parse_sections(&schema, src).expect_err("a shadowing `##### ` blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.item-slot-delimiter-shadowed")
            .expect("an item-slot-delimiter-shadowed finding at the nested level");
        assert!(
            f.message.contains("`##### Stray`"),
            "the message names the heading at ITS depth (`#####` for a `####` item): {:?}",
            f.message
        );
        assert!(
            f.message.contains("must not start a line with `##### `"),
            "the delimiter it shadows is `#####` here, not `####`: {:?}",
            f.message
        );
        assert!(
            f.message.contains("(use `######`+"),
            "the shallowest free depth here is `######`, and `#####` is the corrupting \
             one — naming it would mint a second shadow: {:?}",
            f.message
        );
    }

    /// A `#### `-line-start in a **single-slot** item's prose is *not* a delimiter
    /// (single-slot bodies are opaque whole-body prose), so it must parse cleanly with
    /// no shadow finding and the `####` retained verbatim in the slot span.
    #[test]
    fn single_slot_prose_h4_is_opaque_not_shadowed() {
        let yaml = b"\
type: roadmap
id-from: title
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: notes, slot: { hint: \"x\" } }
";
        let schema = crate::schema::load_schema(yaml).expect("single-slot roadmap schema loads");
        let src = "\
# Roadmap

## Milestones

### M16 self-hosting  {#m16-self-hosting}

Opening prose.

#### A stray heading stays opaque

Closing prose.
";
        let doc = parse_sections(&schema, src).expect("single-slot prose `####` parses cleanly");
        let item = &doc
            .sections
            .iter()
            .find(|s| s.id == "milestones")
            .unwrap()
            .items[0];
        let notes = item
            .slot
            .as_ref()
            .expect("single-slot bare carrier")
            .slice(src);
        assert!(
            notes.contains("#### A stray heading stays opaque"),
            "single-slot `####` is opaque whole-body prose: {notes:?}",
        );
    }

    /// (M45 read-side item-slot ceiling, Face A) The Setext-ceiling message in a
    /// **single-slot** leaf must name the **context-derived** first-allowed depth, not
    /// a hard-coded `####`. A nested change-group `notes` leaf sits at depth 2
    /// (`####` item), so its slot reserves through `####` and the shallowest free depth
    /// is `#####` — the message must say `#####`, because `####` is itself the
    /// corrupting (reserved) depth here (the write twin already parameterizes this).
    #[test]
    fn setext_ceiling_message_names_context_depth_in_nested_leaf() {
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

Broke out
=========
";
        let findings = parse_sections(&changelog_schema(), src)
            .expect_err("a setext heading in a nested change-group leaf blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.slot-setext-heading")
            .expect("a slot-setext-heading finding");
        assert!(
            f.message.contains("`#####`"),
            "the nested leaf reserves through `####`, so the message must name `#####` \
             (the shallowest free depth), never the corrupting `####`: {:?}",
            f.message,
        );
    }

    /// (M45 read-side item-slot ceiling, Face B) A Setext heading OOB-introduced into a
    /// **multi-slot** item leaf (`roadmap.milestones`' `proves`/`decomposition`) must be
    /// flagged at parse time — the multi-slot branch runs no ceiling scan today, so an
    /// out-of-band reserved-depth/Setext heading slips through the read path (the write
    /// path is closed by validate-after). The finding keys at the owning `(item, leaf)`.
    #[test]
    fn setext_ceiling_flagged_in_multi_slot_leaf() {
        let src = "\
# Roadmap

## Milestones

### M16 self-hosting  {#m16-self-hosting}

#### Proves

Broke out
=========

#### Decomposition

Inc 1, Inc 2, Inc 3.
";
        let findings = parse_sections(&two_slot_schema(), src)
            .expect_err("a setext heading in a multi-slot leaf blocks");
        assert_eq!(
            fragments(&findings, "conformance.slot-setext-heading"),
            [Some("milestones/m16-self-hosting/proves")],
            "a multi-slot leaf's setext heading keys at the owning `(item, leaf)`: \
             {findings:#?}",
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

## Options
Alternatives were weighed and rejected.

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

## Options
Alternatives were weighed and rejected.

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

    /// Regression: the typo-hint prefix must never slice through a multi-byte UTF-8
    /// char. `"stéu"` (5 bytes) is the shorter operand and its 2-byte `é` straddles
    /// byte index 3 — the exact input that panicked the old `short[..3]` slice.
    #[test]
    fn is_near_tolerates_non_ascii_keys() {
        assert!(!is_near("stéu", "status"));
        assert!(!is_near("status", "stéu"));
        // ASCII typo hints still resolve.
        assert!(is_near("statuss", "status"));
    }

    /// Regression (end-to-end): an out-of-band edit introducing a non-ASCII unknown
    /// field key re-parses to a located finding, never a panic — the parser's
    /// "every mismatch is a finding, never a panic" contract over OOB human edits.
    #[test]
    fn non_ascii_unknown_field_key_yields_finding_not_panic() {
        let src = "\
---
status: accepted
stéu: oops
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
        let findings = parse_sections(&adr_schema(), src)
            .expect_err("non-ASCII unknown field key blocks, not panics");
        assert!(
            findings
                .iter()
                .any(|f| f.code == "conformance.unknown-field"),
            "expected an unknown-field finding, got: {findings:?}"
        );
    }

    /// Conformance golden: a repeatable `###` item with no `{#id}` anchor → a located
    /// Blocking finding that names the **cause** at its source coordinate — the heading
    /// sits at the section's schema-reserved item depth, so the parser reads it as an
    /// item boundary and the prose that follows is re-attributed. The two readings are
    /// byte-identical (stray prose vs. a genuinely anchor-less new item), so the message
    /// carries **both** repairs (M45 Increment 2 / T3; `design/validation.md` → The M45
    /// registrations, row 2).
    #[test]
    fn unanchored_item_heading_names_the_reserved_depth_cause() {
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
            .find(|f| f.code == "conformance.item-heading-unanchored")
            .expect("an item-heading-unanchored finding");
        assert!(
            f.message.contains("`###`"),
            "the cause is the reserved depth, named: {:?}",
            f.message
        );
        assert!(
            f.message.contains("`####`"),
            "the demote repair names the depth free at this address: {:?}",
            f.message
        );
        assert!(
            f.message
                .contains("`### A criterion with no anchor  {#<id>}`"),
            "the new-item repair is the anchor to write at that line — never a verb, \
             which this state refuses (M46 Increment 5 / T1): {:?}",
            f.message
        );
        insta::assert_debug_snapshot!("unanchored_item_heading", f);
    }

    /// The **single-slot arm** end to end: a `### Ghost` inside a criterion's slot prose
    /// is the corruption the write gate now refuses — read back out-of-band, the parser
    /// mints a ghost item, and the diagnosis must name the depth, not the anchor.
    #[test]
    fn a_ghost_heading_in_single_slot_item_prose_names_the_depth_not_the_anchor() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### It works  {#it-works}

Some prose.

### Ghost

more prose.
";
        let findings = parse_sections(&spec_schema(), src).expect_err("the ghost heading blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.item-heading-unanchored")
            .expect("an item-heading-unanchored finding");
        assert!(
            f.message.contains("Ghost") && f.message.contains("`###`"),
            "the offending heading and its reserved depth: {:?}",
            f.message
        );
        assert_eq!(
            f.location.as_ref().expect("located").line,
            13,
            "located at the offending line, not the item's: {f:?}"
        );
        // The anchor-blaming diagnosis is replaced, not doubled — asserted as *one*
        // conformance finding for this heading rather than as the absence of the M45-retired
        // `item-anchor-missing` code, which no producer has emitted since the rename and
        // which the disposal arm in `finding.rs` now refuses to see named at all.
        assert_eq!(
            findings
                .iter()
                .filter(|f| f.code.starts_with("conformance."))
                .count(),
            1,
            "one diagnosis for the ghost heading, not two: {findings:#?}"
        );
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

    /// The **dominant real shape** — a copy-pasted item, i.e. three heads on one id — and the
    /// half the single-occurrence fixtures never fed: the finding collapses **per duplicated
    /// id**, not per extra occurrence (`command-output-contract.md` → the parse-`conformance.*`
    /// sub-table: *"several items sharing one id collapse, correctly — one duplicated id is one
    /// defect, one repair"*). Red before the producer collapsed: three heads emitted **two**
    /// byte-identical `(code, target)` keys into one slice — a degenerate key in release, and a
    /// panic at the seam in debug — falsifying the closure claim over ordinary corpus content.
    #[test]
    fn three_items_on_one_anchor_collapse_to_one_finding() {
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

### Third  {#dup}
Body three.
";
        let findings = parse_sections(&spec_schema(), src).expect_err("duplicate anchor blocks");
        assert_eq!(
            fragments(&findings, "conformance.item-anchor-duplicate"),
            [Some("criteria/dup")],
            "three heads on one id are ONE defect and ONE repair — one finding, not two: \
             {findings:#?}",
        );
        // The seam, run over the real multi-occurrence slice (not a hand-built pair) and
        // through the **real projection** — serializing the producer's own output as a
        // `Findings` is what a driver receives, and both halves of the membership test ride
        // that serialization (presence on each `Finding`, uniqueness on the collection).
        assert_seam_passes(&findings);
    }

    /// The nested-parent variant of the same collapse: three `#### Added {#added}` groups within
    /// **one** release. The per-parent `seen` scope (C1) is unchanged — what collapses is the
    /// *reporting*, once per duplicated id per parent.
    #[test]
    fn three_nested_groups_on_one_anchor_collapse_to_one_finding() {
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

- OAuth device-code flow

#### Added  {#added}

- a second added group, same anchor, same parent

#### Added  {#added}

- a third added group, same anchor, same parent
";
        let findings = parse_sections(&changelog_schema(), src)
            .expect_err("three #added within one release blocks");
        assert_eq!(
            fragments(&findings, "conformance.item-anchor-duplicate"),
            [Some("releases/1-2-0/changes/added")],
            "the nested duplicate collapses per (parent, id) too, at the nested \
             repeatable's own hop (N29): {findings:#?}",
        );
        assert_seam_passes(&findings);
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

    /// The **second member of the degenerate-key class**, and the half the single-occurrence
    /// golden above never fed: two items carrying the **same** malformed anchor text. Unlike a
    /// duplicated *id* (one shared defect, one repair — collapsed at the producer), these are
    /// **two** items and **two** repairs, so both findings must be emitted — and they key
    /// byte-identically, because the fragment is the malformed text and the malformed text is
    /// the defect, not an identity. Red before the declaration: two byte-identical
    /// `(conformance.item-anchor-malformed, spec:…#criteria/Bad Id)` keys in one slice — a
    /// degenerate key in release, a panic at the seam in debug — so the code is a **declared
    /// non-unique exception**, alongside `item-heading-unanchored`, and the seam must pass it
    /// (`command-output-contract.md` → The declared non-unique exceptions).
    #[test]
    fn two_items_on_one_malformed_anchor_both_report_and_pass_the_seam() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### A criterion  {#Bad Id}
Body one.

### Another criterion  {#Bad Id}
Body two.
";
        let findings = parse_sections(&spec_schema(), src).expect_err("malformed anchors block");
        assert_eq!(
            fragments(&findings, "conformance.item-anchor-malformed"),
            [Some("criteria/Bad Id"), Some("criteria/Bad Id")],
            "two broken anchors are two defects and two repairs — both reported, collapsed on \
             one key by declaration: {findings:#?}",
        );
        // The seam, over the real emitted slice: the collision is the pin, not a defect.
        assert_seam_passes(&findings);
    }

    /// Regression golden (F5): a valid slug that is **not** a `slugify` fixed
    /// point — `{#a-0}`, a leading-article id — is a well-formed frozen anchor and
    /// must be *recognized*, never flagged malformed. Before the F5 edge-stopword
    /// drop, `extract_anchor` used `slugify(id) == id`; F5 broke that equivalence,
    /// so the recognizer is `is_slug`. A committed doc carrying such an anchor must
    /// keep parsing (identity is frozen; it is never re-normalized on read).
    #[test]
    fn stopword_edge_anchor_is_recognized_not_malformed() {
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### A criterion  {#a-0}
Body.
";
        let inst =
            parse_sections(&spec_schema(), src).expect("a valid stopword-edge anchor must parse");
        let criteria = inst
            .sections
            .iter()
            .find(|s| s.id == "criteria")
            .expect("criteria section");
        assert_eq!(
            criteria
                .items
                .iter()
                .map(|i| i.id.as_str())
                .collect::<Vec<_>>(),
            vec!["a-0"],
            "the leading-article anchor is read verbatim as the frozen id"
        );
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

## Options
Alternatives were weighed and rejected.

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

    /// A `changelog`-shaped schema: a `releases` repeatable (depth 1, items `###`)
    /// whose block carries a scalar `version` (id-source) field then a **nested**
    /// `changes` repeatable (depth 2, items `####`), each change-group an `category`
    /// (id-source) field + a `notes` prose slot. The `Leaf::Repeatable` target shape
    /// (`design/changelog.md` → engine work #1) — no leading prose slot on the
    /// release item (review finding B1), so the nested `####` groups are
    /// unambiguous.
    fn changelog_schema() -> Schema {
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
        crate::schema::load_schema(yaml).expect("changelog schema loads")
    }

    /// (T2, done-criterion (i)) A two-level fixture parses: a release item is reached
    /// at depth 1 (`###`) with its frozen `{#id}`, and its nested change-group items
    /// are reachable at the nested level (`####`) — each with its frozen `{#id}`
    /// anchor in order, each carrying its own `notes` slot prose. The nested items
    /// live on the new `ParsedItem.items` field.
    #[test]
    fn nested_repeatable_items_reachable_at_the_nested_level() {
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

- OAuth device-code flow

#### Fixed  {#fixed}

- session fixation on logout
";
        let doc = parse_sections(&changelog_schema(), src).expect("conformant changelog parses");
        let releases = doc.sections.iter().find(|s| s.id == "releases").unwrap();
        assert_eq!(releases.items.len(), 1, "one release at depth 1");
        let release = &releases.items[0];
        assert_eq!(release.id, "1-2-0", "the release's frozen anchor");

        // The nested change-groups are reachable at the nested level, in order, each
        // with its frozen anchor and its own `notes` slot prose.
        let nested_ids: Vec<&str> = release.items.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(nested_ids, ["added", "fixed"], "nested anchors, in order");

        let added = &release.items[0];
        assert_eq!(added.title, "Added");
        let added_notes = added.slot.as_ref().expect("nested notes slot").slice(src);
        assert!(
            added_notes.contains("- OAuth device-code flow"),
            "nested slot re-slices to its prose: {added_notes:?}"
        );
        let fixed = &release.items[1];
        let fixed_notes = fixed.slot.as_ref().expect("nested notes slot").slice(src);
        assert!(
            fixed_notes.contains("- session fixation on logout"),
            "nested slot re-slices to its prose: {fixed_notes:?}"
        );
    }

    /// (T2) A release item carrying a **scalar field** (`date`) *then* its nested
    /// change-groups: the parent's own `<!-- fields -->` group is bounded before the
    /// first nested `####` sub-item, so the field reads cleanly and the nested groups
    /// are still reached (the fields-before-nested boundary, `design/changelog.md` →
    /// illustrative render). This is the changelog's exact release shape.
    #[test]
    fn release_fields_bounded_before_nested_groups() {
        let yaml = b"\
type: changelog
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: date, type: date }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"x\" } }
";
        let schema = crate::schema::load_schema(yaml).expect("changelog-with-date schema loads");
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

<!-- fields -->
- date: 2026-06-14

#### Added  {#added}

- OAuth device-code flow
";
        let doc = parse_sections(&schema, src).expect("conformant changelog parses");
        let release = &doc
            .sections
            .iter()
            .find(|s| s.id == "releases")
            .unwrap()
            .items[0];
        // The release's own `date` field reads — the field group ends before `####`.
        assert_eq!(
            release.fields.len(),
            1,
            "the date field is read, nothing else"
        );
        assert_eq!(release.fields[0].key, "date");
        assert_eq!(release.fields[0].value, Value::Scalar("2026-06-14".into()));
        // The nested change-group is still reached.
        let nested_ids: Vec<&str> = release.items.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(nested_ids, ["added"]);
    }

    /// (M40 triage) A stray `<!-- fields -->` group on a **field-less** nested item
    /// (the change-group template: `category` is the heading-derived id-source,
    /// `notes` is a slot — zero declared bullet fields) is an unknown-field break,
    /// not silence. Before the fix the item's field-group scan was gated on the
    /// template *declaring* fields, so an undeclared bullet written into such an
    /// item (`set-field …/changes/added/title`) parsed clean and the corruption
    /// committed; the scan now runs unconditionally — mirroring the simple-section
    /// path, which already reads a stray sentinel against its (possibly empty)
    /// declared keys.
    #[test]
    fn stray_field_group_on_field_less_nested_item_is_unknown_field() {
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

- OAuth device-code flow

<!-- fields -->
- title: Changed
";
        let findings = parse_sections(&changelog_schema(), src)
            .expect_err("a stray field group on a field-less nested item must not parse clean");
        let finding = findings
            .iter()
            .find(|f| f.code == "conformance.unknown-field")
            .unwrap_or_else(|| panic!("an unknown-field finding, got: {findings:?}"));
        assert!(
            finding.message.contains("`title`"),
            "the finding names the undeclared key: {}",
            finding.message
        );
    }

    /// The **nested arm** (M45 Increment 2 / T3): a `#### Ghost` inside a nested-bearing
    /// release's region — here in a change-group's `notes` prose — is read as a nested
    /// item boundary, so the diagnosis must name the depth reserved *at this nesting*
    /// (`####`) and the depth free there (`#####`), never the shallower section-level
    /// pair. The empirically-verified case behind the changelog generator's H5 rule
    /// (`write.rs` → `notes_prose`).
    #[test]
    fn a_ghost_heading_in_nested_item_prose_names_the_deeper_reserved_depth() {
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

- OAuth device-code flow

#### Ghost

more prose.
";
        let findings =
            parse_sections(&changelog_schema(), src).expect_err("the nested ghost heading blocks");
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.item-heading-unanchored")
            .unwrap_or_else(|| panic!("an item-heading-unanchored finding, got: {findings:#?}"));
        assert!(
            f.message.contains("`####`"),
            "the reserved depth at this nesting: {:?}",
            f.message
        );
        assert!(
            f.message.contains("`#####`"),
            "the depth free at this nesting, not the section-level `####`: {:?}",
            f.message
        );
        assert_eq!(
            f.location.as_ref().expect("located").line,
            11,
            "located at the offending line: {f:?}"
        );
    }

    /// (T2, done-criterion (ii) — within one parent) Two `#added` change-groups
    /// **within one release** are a duplicate anchor (per-parent uniqueness) → a
    /// located Blocking `conformance.item-anchor-duplicate` finding.
    #[test]
    fn two_same_anchor_groups_within_one_parent_block() {
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

- OAuth device-code flow

#### Added  {#added}

- a second added group, same anchor, same parent
";
        let findings = parse_sections(&changelog_schema(), src)
            .expect_err("two #added within one release blocks");
        assert!(
            findings
                .iter()
                .any(|f| f.code == "conformance.item-anchor-duplicate"),
            "expected a duplicate-anchor finding, got: {findings:?}"
        );
    }

    /// (T2, done-criterion (ii) — across two parents) The same `#added` anchor across
    /// **two different releases** is legitimate (the `seen` set resets per parent) →
    /// parses clean, both nested groups present under their respective parents.
    #[test]
    fn same_anchor_across_two_parents_parses_clean() {
        let src = "\
# Changelog

## Releases

### 1.2.0  {#1-2-0}

#### Added  {#added}

- OAuth device-code flow

### 1.1.0  {#1-1-0}

#### Added  {#added}

- initial login
";
        let doc = parse_sections(&changelog_schema(), src)
            .expect("the same anchor across two parents parses clean");
        let releases = doc.sections.iter().find(|s| s.id == "releases").unwrap();
        assert_eq!(releases.items.len(), 2, "two releases");
        // Each release carries its own `#added` group — the `seen` reset per parent.
        for rel in &releases.items {
            let ids: Vec<&str> = rel.items.iter().map(|i| i.id.as_str()).collect();
            assert_eq!(ids, ["added"], "each parent has its own #added");
        }
    }

    /// (T2, done-criterion (iii) — additive guard) A single-level repeatable doc
    /// parses **byte-identically to today**: same item count, frozen anchors, slot
    /// prose, and per-item fields — and no nested `items` leak onto a flat item. The
    /// depth-aware scanner must not perturb the long-shipped single-level path.
    #[test]
    fn single_level_doc_parses_unchanged() {
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
        let ids: Vec<&str> = criteria.items.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(ids, ["rate-limit", "burst-allowance"]);
        // A flat item carries no nested items (the additive-guard skip-on-empty).
        for it in &criteria.items {
            assert!(it.items.is_empty(), "flat item carries no nested items");
        }
        // Slot prose + field value unchanged from the long-shipped single-level path.
        let rl = &criteria.items[0];
        assert_eq!(
            rl.slot.as_ref().unwrap().slice(src),
            "The gateway rejects the 101st request in a 60s window."
        );
        assert_eq!(rl.fields.len(), 1);
        assert_eq!(rl.fields[0].key, "maps-to-test");
        assert_eq!(
            rl.fields[0].value,
            Value::Scalar("`test/rate_limit_spec.rb#burst`".into())
        );
    }

    /// Drive a producer's real emitted slice through the **real projection** — the membership
    /// test rides the serialization itself (the presence half on [`Finding`]'s `Serialize`, the
    /// uniqueness half on [`crate::finding::Findings`]'s), so serializing is exactly what a
    /// driver does and exactly what the seam checks. Debug builds panic on a violation, which
    /// is the assertion.
    ///
    /// These parse findings are **pre-flip** (fragment-only addresses — the owning doc's
    /// identity is threaded in downstream by `validate::attribute_to_doc`), so what the seam
    /// exercises here is the *discriminating* half: distinct fragments where the contract owes
    /// them, a declared collapse where it does not.
    fn assert_seam_passes(findings: &[Finding]) {
        serde_json::to_string(&crate::finding::Findings::from(findings.to_vec()))
            .expect("the emitted slice projects through the findings seam");
    }

    /// The address each finding of `code` carries (the fragment the outward assembly
    /// installed), for the fragment sub-table below. `None` when the code carries no
    /// address at all (the bare-doc row).
    fn fragments<'a>(findings: &'a [Finding], code: &str) -> Vec<Option<&'a str>> {
        findings
            .iter()
            .filter(|f| f.code == code)
            .map(|f| f.location.as_ref().and_then(|l| l.address.as_deref()))
            .collect()
    }

    /// (M42 Inc 9 T5 — the collision the family was named for) **Two unknown fields in
    /// one section carry DISTINCT fragments.** Before the outward assembly, every
    /// `conformance.unknown-field` in a doc set **no** address, so the store-scope
    /// `path→URI` flip addressed them all at the bare `type:slug` and a driver saw one
    /// key for both (`command-output-contract.md` → the parse-conformance sub-table:
    /// `unknown-field` → `#<section>/<field-key>`).
    #[test]
    fn two_unknown_fields_in_one_section_carry_distinct_fragments() {
        let src = "\
---
status: proposed
date: 2026-05-31
alpha: 1
beta: 2
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
        let findings = parse_sections(&adr_schema(), src).expect_err("two unknown keys block");
        assert_eq!(
            fragments(&findings, "conformance.unknown-field"),
            [Some("status/alpha"), Some("status/beta")],
            "each unknown key keys at its own `#<section>/<field-key>`, so the two \
             findings do not collide on one `(code, target)`: {findings:#?}",
        );
    }

    /// (M42 Inc 9 — the repeated key) **One repeated unknown key is ONE finding.** The
    /// sibling of the test above, and the case it did not cover: two unknown keys that are
    /// *the same key* address at the same `#<section>/<field-key>` **by construction**, so a
    /// finding per *line* emitted two byte-identical `(code, target)` keys into one JSON
    /// array — the exact collision the contract pins as impossible for the code it calls
    /// "the collision the family was named for". Collapse at the producer, as
    /// `item-anchor-duplicate` already does one row over: a repeated key is **one defect and
    /// one repair** (delete the stray line), and the contract's declared granularity for
    /// this code is per `(section, key)` — not per line.
    #[test]
    fn a_repeated_unknown_field_key_reports_once() {
        let src = "\
---
status: proposed
date: 2026-05-31
owner: alice
owner: bob
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
        let findings = parse_sections(&adr_schema(), src).expect_err("an unknown key blocks");
        assert_eq!(
            fragments(&findings, "conformance.unknown-field"),
            [Some("status/owner")],
            "the repeated key `owner` is one defect and one repair, so it is reported ONCE — \
             reporting per line emits two byte-identical `(code, target)` keys into one \
             emitted slice, which the membership test forbids: {findings:#?}",
        );
    }

    /// (M49 Inc 1 T3 — the read-side half of the item-region class) **A repeated
    /// DECLARED field key is a blocking finding, reported once.** The sibling above
    /// collapses a repeated *undeclared* key; a repeated *declared* one was pushed into
    /// the parsed field list unchecked, so a field group carrying two `date:` lines — or
    /// three values on a `0..1` enum — was **conformant to the tool**: `jigc task
    /// validate` raised nothing at all, and the pinned `doc show --format json` returned
    /// whichever value `find` reached first, with no diagnostic. That is the shape T1
    /// stopped jigc's own writer from creating; this is the one that already exists on
    /// disk (`DECISIONS.md` → 2026-08-28 M49 Increment 1 planning: decomposition, T3).
    ///
    /// Keyed at `#<section>/<field-key>` — the granularity
    /// `conformance.unknown-field` and the duplicate-`{#id}` guard already use, and the
    /// only one [`crate::finding::debug_assert_targets_declared`] admits: two lines
    /// carrying one key address at one fragment **by construction**, so a per-line
    /// finding would be a degenerate key.
    #[test]
    fn a_repeated_declared_field_key_blocks_once() {
        let src = "\
---
status: proposed
date: 2026-05-31
date: 2026-06-01
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
        let findings =
            parse_sections(&adr_schema(), src).expect_err("a repeated declared key blocks");
        assert_eq!(
            fragments(&findings, "conformance.duplicate-field"),
            [Some("status/date")],
            "the repeated declared key `date` is ONE defect and ONE repair (delete the \
             stray line), keyed at its `#<section>/<field-key>`: {findings:#?}",
        );
        let f = findings
            .iter()
            .find(|f| f.code == "conformance.duplicate-field")
            .expect("a duplicate-field finding");
        assert_eq!(
            f.severity,
            crate::finding::Severity::Blocking,
            "a field carrying two values is not a heads-up: {f:#?}",
        );
        assert!(
            f.message.contains("date"),
            "the message names the repeated key: {f:#?}",
        );
        // The route floor: `conformance.*` is route-EXEMPT, not route-forbidden, and
        // this diagnostic does know a direction — the hand-repair sanction, the shipped
        // route for a managed doc whose repair is a human edit (M48 Inc 4).
        let route = f.route.as_ref().expect("a routed diagnostic").to_string();
        assert!(
            route.contains("date") && route.contains("hand-edit"),
            "the route names the line to delete and sanctions the hand repair: {route}",
        );
        // And the emitted slice projects through the real key seam.
        assert_seam_passes(&findings);
    }

    /// (M49 Inc 1 T3) The same defect at the **item** locus keys at
    /// `#<section>/<item>/<field-key>` — the outward assembly's item hop — so a repeat
    /// in one item does not collide with a repeat in its sibling.
    #[test]
    fn a_repeated_declared_item_field_key_keys_at_its_item() {
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
- maps-to-test: `test/rate_limit_spec.rb#steady`

### Burst allowance  {#burst-allowance}
A short burst above the limit is tolerated for 2s.

<!-- fields -->
- maps-to-test: `test/burst_spec.rb#one`
- maps-to-test: `test/burst_spec.rb#two`
";
        let findings =
            parse_sections(&spec_schema(), src).expect_err("a repeated item field blocks");
        assert_eq!(
            fragments(&findings, "conformance.duplicate-field"),
            [
                Some("criteria/rate-limit/maps-to-test"),
                Some("criteria/burst-allowance/maps-to-test")
            ],
            "each item's repeat keys under its OWN item hop, so two sibling items \
             repeating one key do not collide on a single `(code, target)`: {findings:#?}",
        );
        assert_seam_passes(&findings);
    }

    /// (M42 Inc 9 T5) The parse-`conformance.*` sub-table, code by code
    /// (`command-output-contract.md` → the parse-conformance sub-table). Each fixture
    /// trips one code; the assertion is the **fragment the finding carries** — the half
    /// of the stable key that discriminates two instances inside one doc. The doc hop
    /// (`type:slug`) is prefixed one stage further out, by
    /// [`crate::validate`]'s `attribute_to_doc`.
    #[test]
    fn conformance_findings_carry_the_pinned_fragment() {
        // `section-missing` → `#<section>`; `section-renamed` → `#<section>`.
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Contextt
Forces.

## Options
Alternatives were weighed and rejected.

## Decision
We decided.
";
        let findings = parse_sections(&adr_schema(), src).expect_err("renamed + missing block");
        assert_eq!(
            fragments(&findings, "conformance.section-renamed"),
            [Some("context")],
            "the renamed heading keys at its section: {findings:#?}",
        );
        assert_eq!(
            fragments(&findings, "conformance.section-missing"),
            [Some("consequences")],
            "the missing section keys at its section: {findings:#?}",
        );

        // `orphaned-sentinel` → `#<section>`.
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Context
Forces.

<!-- fields -->

## Options
Alternatives were weighed and rejected.

## Decision
We decided.

## Consequences
Fine.
";
        let findings = parse_sections(&adr_schema(), src).expect_err("an orphaned sentinel blocks");
        assert_eq!(
            fragments(&findings, "conformance.orphaned-sentinel"),
            [Some("context")],
            "the orphaned sentinel keys at its section's field group: {findings:#?}",
        );

        // `malformed-field-block` → `#<section>`.
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Context
Forces.

<!-- fields -->
- no separator here

## Options
Alternatives were weighed and rejected.

## Decision
We decided.

## Consequences
Fine.
";
        let findings = parse_sections(&adr_schema(), src).expect_err("a malformed block blocks");
        assert_eq!(
            fragments(&findings, "conformance.malformed-field-block"),
            [Some("context")],
            "the malformed field block keys at its section: {findings:#?}",
        );

        // `slot-setext-heading` / `slot-heading-depth` in a **section** slot → the
        // owning slot's address, `#<section>` (declared non-unique, per slot).
        let src = "\
---
status: proposed
date: 2026-05-31
---

# A decision

## Context
Setext
======

### Too shallow

## Options
Alternatives were weighed and rejected.

## Decision
We decided.

## Consequences
Fine.
";
        let findings = parse_sections(&adr_schema(), src).expect_err("slot headings block");
        assert_eq!(
            fragments(&findings, "conformance.slot-setext-heading"),
            [Some("context")],
            "a section slot's setext heading keys at the owning slot: {findings:#?}",
        );
        assert_eq!(
            fragments(&findings, "conformance.slot-heading-depth"),
            [Some("context")],
            "a section slot's reserved-depth heading keys at the owning slot: {findings:#?}",
        );

        // `item-heading-unanchored` → `#<section>` (declared non-unique: the item has
        // no identity — that IS the finding).
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### A criterion with no anchor
Body.
";
        let findings = parse_sections(&spec_schema(), src).expect_err("a missing anchor blocks");
        assert_eq!(
            fragments(&findings, "conformance.item-heading-unanchored"),
            [Some("criteria")],
            "an anchor-less item keys at its section — the discriminator is the thing \
             that is missing: {findings:#?}",
        );

        // `item-anchor-malformed` → `#<section>/<found>` (the malformed anchor text).
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### A criterion  {#Bad Id}
Body.
";
        let findings = parse_sections(&spec_schema(), src).expect_err("a malformed anchor blocks");
        assert_eq!(
            fragments(&findings, "conformance.item-anchor-malformed"),
            [Some("criteria/Bad Id")],
            "the malformed anchor keys at the text it found: {findings:#?}",
        );

        // `item-anchor-duplicate` → `#<section>/<id>` (the duplicated id).
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
        let findings = parse_sections(&spec_schema(), src).expect_err("a duplicate anchor blocks");
        assert_eq!(
            fragments(&findings, "conformance.item-anchor-duplicate"),
            [Some("criteria/dup")],
            "the duplicate keys at the duplicated id — one duplicated id is one defect: \
             {findings:#?}",
        );

        // A **single-slot item**'s slot prose: the ceiling violation keys at the owning
        // slot, `#<section>/<item>/<leaf>`.
        let src = "\
---
title: Auth flow
---

# Auth flow

## Criteria

### Rate limit holds  {#rate-limit}

Setext
======
";
        let findings =
            parse_sections(&spec_schema(), src).expect_err("a setext heading in item prose blocks");
        assert_eq!(
            fragments(&findings, "conformance.slot-setext-heading"),
            [Some("criteria/rate-limit/statement")],
            "an item slot's setext heading keys at the owning slot — item hop from \
             `parse_items`, leaf hop from the item's slot leaf: {findings:#?}",
        );

        // `item-slot-label-missing` → `#<section>/<item>/<leaf>`;
        // `item-slot-delimiter-shadowed` → `#<section>/<item>` (declared non-unique).
        let src = "\
# Roadmap

## Milestones

### M16 self-hosting  {#m16-self-hosting}

#### Proves

Closes the self-hosting loop.

#### Note

This stray heading is prose, not a slot delimiter.
";
        let findings =
            parse_sections(&two_slot_schema(), src).expect_err("a shadowed delimiter blocks");
        assert_eq!(
            fragments(&findings, "conformance.item-slot-label-missing"),
            [Some("milestones/m16-self-hosting/decomposition")],
            "the missing slot sub-heading keys at its `(item, leaf)`: {findings:#?}",
        );
        assert_eq!(
            fragments(&findings, "conformance.item-slot-delimiter-shadowed"),
            [Some("milestones/m16-self-hosting")],
            "the shadowing prose keys at the item that owns it: {findings:#?}",
        );
    }

    /// (M42 Inc 9 T5) `conformance.header-not-first` is the sub-table's one **bare-doc**
    /// row: the subject is the doc's section order, one per doc by construction, so it
    /// carries **no fragment** and keys at the bare `type:slug`.
    #[test]
    fn header_not_first_carries_no_fragment() {
        let yaml = b"\
type: odd
id-from: title
sections:
  - id: body
    slot: { hint: \"x\" }
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
";
        let schema = crate::schema::load_schema(yaml).expect("a header-second schema loads");
        let src = "\
# Odd

## Body
Prose.
";
        let findings = parse_sections(&schema, src).expect_err("a header-second schema blocks");
        assert_eq!(
            fragments(&findings, "conformance.header-not-first"),
            [None],
            "the whole doc's section order is the subject — one per doc, no discriminator \
             needed: {findings:#?}",
        );
    }
}

#[cfg(test)]
mod item_prop_tests {
    use super::*;
    use crate::field_block::Value;
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
        crate::schema::load_schema(yaml).expect("spec schema loads")
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
            let schema = crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types()).expect("adr.yaml loads");
            let src = format!(
                "---\nstatus: proposed\ndate: 2026-05-31\n---\n\n# Title\n\n\
                 ## Context\n{context}\n\n## Options\nAlternatives were weighed and rejected.\n\n## Decision\n{decision}\n\n## Consequences\n{consequences}\n"
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

    proptest! {
        /// The equivalence [`LineIndex`] rests on, asserted rather than asserted in
        /// prose: for **every** byte offset in an arbitrary source, the binary search
        /// returns exactly what the counting definition it replaced returned.
        ///
        /// The M49 Increment 5 speedup is a speedup only if the answers do not move,
        /// and the answers are line numbers on located findings — user-visible bytes.
        /// The predecessor is written out inline below, so this test carries its own
        /// specification and does not lean on a function that no longer exists.
        #[test]
        fn the_line_index_agrees_with_counting_the_prefix(
            source in "(?s)[a-z \n]{0,200}",
        ) {
            let index = LineIndex::new(&source);
            for offset in 0..=source.len() {
                let counted = source[..offset].bytes().filter(|&b| b == b'\n').count() + 1;
                prop_assert_eq!(index.line_of(offset), counted, "at offset {}", offset);
            }
        }
    }
}
