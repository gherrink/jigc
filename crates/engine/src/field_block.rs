//! The shared flat `key: value` field-line reader — one grammar, two locations.
//!
//! Both front-matter (between `---` fences) and body/item field groups (a
//! `<!-- fields -->`-sentinelled bullet list) carry the *same* field grammar
//! ([design/storage.md](../../../design/storage.md) → "One field grammar, two
//! structural frames"; [implementation/parsing.md](../../../implementation/parsing.md)
//! → Front-matter / Field-group delineation). This module is that one grammar; the
//! two structural frames (fences vs. sentinel + `- ` bullets) are the *callers'*
//! concern — they hand this reader a sequence of bare `key: value` lines.
//!
//! ## The determinism boundary, applied to a value
//!
//! A field value is read as the **literal text after `key:`, trimmed** — never
//! coerced. YAML's dynamic typing fights schema-typed fields (the *Norway problem*
//! `no`→`false`, a bare date→a date object, `1.0`→a float), so this reader keeps
//! every value as a string and lets the schema adjudicate the type
//! ([parsing.md](../../../implementation/parsing.md) → Front-matter). The one
//! structural exception is **inline-flow list values** (`relates-to: [a, b]`) — a
//! forward relation with cardinality > 1 — which parse to an ordered list of
//! trimmed string elements (each element is still opaque text).
//!
//! ## Round-trip: the #1-risk diff-clean guarantee
//!
//! The reader is paired with [`emit`], the byte-exact inverse over the *parsed*
//! field set: `parse → emit` is byte-identical, one line per field, order
//! preserved. This is the flat-line diff-clean guarantee
//! ([parsing.md](../../../implementation/parsing.md) → Round-trip guarantees clause
//! 1: idempotent on canonical content) at the field-block granularity, and the
//! foundation the surgical-splice writer builds on.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A field value as read from disk: an opaque scalar string, or an inline-flow
/// list of opaque scalar strings.
///
/// Both forms hold **trimmed literal text** — the schema, not this reader,
/// adjudicates whether `"no"` is a bool, `"2026-05-23"` a date, or `"1.0"` an int
/// (it is none of those to *us*). The distinction the reader *does* draw is
/// structural: a value wrapped in `[ … ]` at the top level is a [`Value::List`]
/// (cardinality > 1, inline flow), everything else is a [`Value::Scalar`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Value {
    /// A single opaque value — the literal text after `key:`, trimmed.
    Scalar(String),
    /// An inline-flow list `[a, b, c]` — each element trimmed, opaque, ordered.
    List(Vec<String>),
}

impl Value {
    /// Render this value to its canonical on-disk bytes — the text after `key: ` in
    /// the flat field grammar: a [`Value::Scalar`] verbatim, a [`Value::List`] as the
    /// inline-flow `[a, b, c]` form (`", "`-separated). The **single source** of the
    /// value byte form, shared by [`emit`] and the `rename` referrer-repoint
    /// ([`crate::write::repoint_ref`]) so the two cannot drift — the canonical bytes
    /// are defined in exactly one place (`DECISIONS.md` 2026-05-31 → Canonical byte
    /// form).
    pub fn render(&self) -> String {
        match self {
            Value::Scalar(s) => s.clone(),
            Value::List(elems) => format!("[{}]", elems.join(", ")),
        }
    }
}

/// One parsed field: its `key` and its raw [`Value`], in the order it appeared.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    /// The field key (the text before the first `:`), trimmed.
    pub key: String,
    /// The field's raw value, opaque to this reader.
    pub value: Value,
}

/// A parsed flat field block: an ordered list of [`Field`]s.
///
/// Order is physical order — the order the lines appeared — and is preserved on
/// [`emit`], so the round-trip is byte-identical line-for-line.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FieldBlock {
    /// The fields, in physical order.
    pub fields: Vec<Field>,
}

/// A located field-block parse error.
///
/// Carries the 1-based line within the block (the caller offsets it to the file)
/// and a human message. The MVP conformance surface promotes these to
/// [`crate::finding::Finding`]s at the call site; at this granularity the reader
/// reports the structural defect (a line with no `:` separator).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldBlockError {
    /// 1-based line within the block where the defect was found.
    pub line: usize,
    /// Human-readable description of the defect.
    pub message: String,
}

impl fmt::Display for FieldBlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "field block line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for FieldBlockError {}

/// Parse a flat field block — a sequence of bare `key: value` lines — into an
/// ordered [`FieldBlock`].
///
/// Input is the block content with its structural frame already stripped by the
/// caller: front-matter has had its `---` fences removed; a body field group has
/// had its `<!-- fields -->` sentinel and `- ` bullet prefixes removed. Each
/// non-empty line is `key: value`; the value is the literal text after the first
/// `:`, trimmed. A value of the form `[ … ]` is an inline-flow [`Value::List`].
///
/// A line with no `:` is a conformance defect ([`FieldBlockError`]). Empty / blank
/// lines are not expected in a canonical block (the writer emits a contiguous
/// list, no blank lines between fields — `DECISIONS.md` 2026-05-31 canonical byte
/// form) and are rejected so the round-trip stays exact; the caller frames the
/// block precisely.
pub fn parse(block: &str) -> Result<FieldBlock, FieldBlockError> {
    let mut fields = Vec::new();
    for (idx, line) in block.lines().enumerate() {
        let lineno = idx + 1;
        if line.trim().is_empty() {
            return Err(FieldBlockError {
                line: lineno,
                message: "unexpected blank line in field block".to_string(),
            });
        }
        let Some((key, raw)) = line.split_once(':') else {
            return Err(FieldBlockError {
                line: lineno,
                message: format!("field line has no `:` separator: {line:?}"),
            });
        };
        fields.push(Field {
            key: key.trim().to_string(),
            value: parse_value(raw.trim()),
        });
    }
    Ok(FieldBlock { fields })
}

/// Classify a trimmed raw value: an inline-flow list `[ … ]`, else a scalar. Public so
/// a write-ack can project a just-written value through the same scalar/list grammar the
/// read path parses it back with (the `doc show` field shape — `design/command-output-
/// contract.md` §2), without re-reading the persisted doc.
pub fn parse_value(raw: &str) -> Value {
    if let Some(inner) = raw.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let inner = inner.trim();
        if inner.is_empty() {
            return Value::List(Vec::new());
        }
        let elems = inner.split(',').map(|e| e.trim().to_string()).collect();
        return Value::List(elems);
    }
    Value::Scalar(raw.to_string())
}

/// Emit a [`FieldBlock`] back to its canonical bare-line form — the byte-exact
/// inverse of [`parse`] over a parsed block.
///
/// One line per field, in physical order, `key: value` with a single space after
/// the colon; an inline-flow list renders `[a, b, c]` (comma-space separated). A
/// trailing newline terminates each line (so the block is a clean run of lines the
/// caller frames with fences or bullets). This is the canonical form
/// (`DECISIONS.md` 2026-05-31 → Canonical byte form); a block already in canonical
/// form round-trips `parse → emit` byte-identical.
pub fn emit(block: &FieldBlock) -> String {
    let mut out = String::new();
    for field in &block.fields {
        out.push_str(&field.key);
        out.push_str(": ");
        out.push_str(&field.value.render());
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Golden: a flat field block — including values that would trip YAML's
    /// dynamic typing — parses to **raw string values, verbatim**. The Norway
    /// problem (`no`), a bare date, and `1.0` all stay literal text; the schema,
    /// not this reader, decides their type. An inline-flow list parses to an
    /// ordered list of trimmed opaque elements. Pinning the parsed shape locks the
    /// "values are opaque, schema adjudicates" contract.
    #[test]
    fn field_block_parses_values_as_opaque_text() {
        let block = "\
status: no
date: 2026-05-23
ratio: 1.0
title: Rate-limit at the gateway
relates-to: [adr:a, adr:b]
empty-list: []
";
        let parsed = parse(block).expect("canonical block parses");
        insta::assert_debug_snapshot!(parsed.fields, @r#"
        [
            Field {
                key: "status",
                value: Scalar(
                    "no",
                ),
            },
            Field {
                key: "date",
                value: Scalar(
                    "2026-05-23",
                ),
            },
            Field {
                key: "ratio",
                value: Scalar(
                    "1.0",
                ),
            },
            Field {
                key: "title",
                value: Scalar(
                    "Rate-limit at the gateway",
                ),
            },
            Field {
                key: "relates-to",
                value: List(
                    [
                        "adr:a",
                        "adr:b",
                    ],
                ),
            },
            Field {
                key: "empty-list",
                value: List(
                    [],
                ),
            },
        ]
        "#);
    }

    /// Golden: the canonical emitted byte form — one `key: value` line per field
    /// (single space after the colon), inline-flow list comma-space separated, a
    /// trailing newline per line. Pins the exact bytes the round-trip is measured
    /// against (`DECISIONS.md` 2026-05-31 → Canonical byte form).
    #[test]
    fn field_block_emits_canonical_bytes() {
        let fb = FieldBlock {
            fields: vec![
                Field {
                    key: "status".into(),
                    value: Value::Scalar("accepted".into()),
                },
                Field {
                    key: "relates-to".into(),
                    value: Value::List(vec!["adr:a".into(), "adr:b".into()]),
                },
            ],
        };
        insta::assert_snapshot!(emit(&fb), @r"
        status: accepted
        relates-to: [adr:a, adr:b]
        ");
    }

    /// [`Value::render`] is the single source of the value byte form: a scalar is
    /// verbatim; an inline-flow list is `[a, b, c]` (`", "`-separated, order
    /// preserved). This locks the bytes the `rename` referrer-repoint splices.
    #[test]
    fn value_render_is_canonical_bytes() {
        assert_eq!(
            Value::Scalar("prd:auth-flow".into()).render(),
            "prd:auth-flow"
        );
        assert_eq!(
            Value::List(vec!["adr:a".into(), "adr:b".into(), "adr:c".into()]).render(),
            "[adr:a, adr:b, adr:c]"
        );
        assert_eq!(Value::List(Vec::new()).render(), "[]");
    }

    /// A field line with no `:` separator is a located conformance defect, not a
    /// panic — the reader reports the structural problem with its block-relative
    /// line.
    #[test]
    fn field_line_without_colon_is_located_error() {
        let err = parse("status: ok\nbroken line\n").expect_err("missing colon rejected");
        assert_eq!(err.line, 2);
        assert!(err.message.contains("no `:`"), "{}", err.message);
    }

    /// A value containing a colon keeps everything after the *first* colon as its
    /// opaque text — `code-anchor`/`ref` values legitimately carry `:`.
    #[test]
    fn only_first_colon_splits_key_from_value() {
        let parsed = parse("supersedes: adr:single-node-cache\n").expect("parses");
        assert_eq!(parsed.fields.len(), 1);
        assert_eq!(parsed.fields[0].key, "supersedes");
        assert_eq!(
            parsed.fields[0].value,
            Value::Scalar("adr:single-node-cache".into())
        );
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    /// A key: trimmed, no `:` (would re-split), no leading `[` ambiguity at value
    /// start is irrelevant here. Non-empty, no newline, no leading/trailing space
    /// (trimming is idempotent so canonical keys are pre-trimmed), no `:`.
    fn key_strategy() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9-]{0,20}"
    }

    /// A scalar value in *canonical* form: trimmed, single line, not shaped like an
    /// inline-flow list (no leading `[` with trailing `]`), so `parse → emit`
    /// reproduces it verbatim. Opaque text otherwise — colons, dots, the Norway
    /// word, digits all allowed.
    fn scalar_strategy() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9][a-zA-Z0-9 ._:/-]{0,30}[a-zA-Z0-9]|[a-zA-Z0-9]"
            .prop_map(|s| s.trim().to_string())
            .prop_filter("not list-shaped", |s| {
                !(s.starts_with('[') && s.ends_with(']'))
            })
    }

    /// A list element: trimmed, no comma (the inline-flow separator), no surrounding
    /// space, non-empty.
    fn elem_strategy() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9][a-zA-Z0-9._:/-]{0,15}"
    }

    fn value_strategy() -> impl Strategy<Value = Value> {
        prop_oneof![
            scalar_strategy().prop_map(Value::Scalar),
            prop::collection::vec(elem_strategy(), 1..4).prop_map(Value::List),
        ]
    }

    fn field_strategy() -> impl Strategy<Value = Field> {
        (key_strategy(), value_strategy()).prop_map(|(key, value)| Field { key, value })
    }

    proptest! {
        /// The diff-clean flat-line guarantee: for an arbitrary canonical field set,
        /// `emit → parse → emit` is byte-identical — one line per field, order
        /// preserved, values opaque. This is the round-trip contract at field-block
        /// granularity (`parsing.md` → Round-trip guarantees clause 1).
        #[test]
        fn emit_parse_emit_is_byte_identical(
            fields in prop::collection::vec(field_strategy(), 0..8)
        ) {
            let original = FieldBlock { fields };
            let bytes = emit(&original);
            let reparsed = parse(&bytes).expect("emitted block re-parses");
            prop_assert_eq!(&reparsed, &original);
            prop_assert_eq!(emit(&reparsed), bytes);
        }
    }
}
