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
//! ## Scope (this task, read path)
//!
//! Simple sections — the header (front-matter) section and body **slot** sections,
//! which is exactly what the MVP `commit` / `adr` schemas use. A `repeatable`
//! section (`###` items + `{#id}` anchors) is surfaced as an explicit
//! [`Severity::Blocking`] "unsupported" finding rather than mis-parsed; item
//! parsing, front-matter field parsing, and the writer are separate later tasks.

use std::ops::Range;

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};
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

/// One mapped section: its schema `id` and — for a body slot section — the byte
/// span of its **opaque** slot prose.
///
/// The header (front-matter) section has no slot (`slot: None`); a body slot
/// section carries `Some(span)`. The span is recorded, not interpreted: re-slicing
/// the source over it yields exactly the slot's prose bytes (the round-trip
/// boundary-integrity property).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedSection {
    /// The schema section id this maps to.
    pub id: String,
    /// The opaque slot prose span, when the section has a slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<Span>,
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
/// section heading in document order, records each slot's opaque span, and enforces
/// the heading-depth ceiling inside every slot. Any point it can't proceed —
/// a missing/renamed/reordered required heading, a forbidden heading inside slot
/// prose — is a located [`Severity::Blocking`] finding; findings are collected, not
/// fail-fast. A non-empty finding set means *no* instance is returned.
///
/// Scope: simple sections (header + body slot). A `repeatable` section is reported
/// as an unsupported-shape finding (item parsing is a later task).
pub fn parse_sections(schema: &Schema, source: &str) -> Result<Document, Vec<Finding>> {
    let blocks = scan_blocks(source);
    let mut findings = Vec::new();
    let mut parsed = Vec::new();

    // The header (front-matter) section, if the schema declares one, is consumed by
    // the metadata block; it has no slot. We don't parse its fields here.
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
        parsed.push(ParsedSection {
            id: schema.sections[idx].id.clone(),
            slot: None,
        });
        // Skip a leading metadata block if present; its absence is a finalize/field
        // concern, not a block-structure conformance error at this layer.
        if matches!(blocks.first(), Some(Block::Metadata)) {
            block_cursor = 1;
        }
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
        // Repeatable sections are out of this task's scope — surface, don't mis-parse.
        if let SectionBody::Repeatable { .. } = section.body {
            findings.push(Finding::blocking(
                "conformance.repeatable-unsupported",
                format!(
                    "repeatable section `{}` is not yet supported by the read path",
                    section.id
                ),
                Location::at(1, 1),
            ));
            continue;
        }

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

        // The slot span: from the byte after the heading to the next section
        // boundary (next `##`, a field-group sentinel, or EOF).
        let next_boundary = section_boundary(source, &blocks, *content_start);
        let span = trim_span(source, *content_start, next_boundary);

        // Enforce the heading-depth ceiling inside the located slot span: an ATX
        // `##`/`###` or any Setext heading is a CLI-owned structural marker and
        // must not appear in slot prose.
        for v in ceiling_violations(&blocks, span.start, span.end) {
            findings.push(v);
        }

        let slot = match &section.body {
            SectionBody::Simple { slot: Some(_), .. } => Some(span),
            // A simple section with fields-only and no slot still owns the region,
            // but records no slot. (MVP body sections all declare a slot.)
            _ => None,
        };

        parsed.push(ParsedSection {
            id: section.id.clone(),
            slot,
        });
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
    /// A `---`-fenced metadata (front-matter) block. Its span is not needed by the
    /// read path (front-matter *field* parsing is a later task); only its presence
    /// at the top of the block stream matters here.
    Metadata,
    /// An ATX or Setext heading. `is_atx` distinguishes `##`-style from Setext
    /// underline-style (the latter forbidden in slots at any depth).
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
}

/// Walk the CommonMark block events once, projecting the structural blocks the
/// schema mapping needs (headings, the metadata block, the field-group sentinel),
/// each with its byte range and start line.
fn scan_blocks(source: &str) -> Vec<Block> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let mut blocks = Vec::new();
    for (event, range) in Parser::new_ext(source, opts).into_offset_iter() {
        match event {
            Event::Start(Tag::MetadataBlock(_)) => {
                blocks.push(Block::Metadata);
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

/// The byte offset where the slot region opened at `content_start` ends: the start
/// of the next **`##` section heading**, the next field-group sentinel, or EOF —
/// whichever comes first. Deeper (`####`+) headings are *allowed* slot-internal
/// structure and never end a slot; only the section-depth `##` does.
fn section_boundary(source: &str, blocks: &[Block], content_start: usize) -> usize {
    blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading {
                level: HeadingLevel::H2,
                range,
                ..
            } if range.start >= content_start => Some(range.start),
            Block::FieldSentinel { range } if range.start >= content_start => Some(range.start),
            _ => None,
        })
        .min()
        .unwrap_or(source.len())
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
