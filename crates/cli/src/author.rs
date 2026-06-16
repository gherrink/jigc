//! Declarative whole-doc batch-authoring payload + parser — the `jigc doc author
//! <doctype> --from <payload>` front of the M24 batch verb (`design/write-commands.md`
//! → Batch authoring; `design/auto-migration.md` → Hardening #1).
//!
//! This module owns **only the parse** (`design/write-commands.md` → Batch authoring,
//! review B1): it turns the agent-authored declarative payload into the **ordered
//! leaf-write plan** the verb applies — a `create` id-source plus an ordered sequence
//! of `add-item` / `set-field` / `set-slot` leaves, each carrying the **fragment**
//! (the `#…`-less address tail) relative to the about-to-be-created instance. The verb
//! dispatch (the next increment task) prepends `<doctype>:<slug>#` to each fragment and
//! chains the existing engine splice primitives over a single in-memory buffer.
//!
//! Parsing runs **before any persist**, so a structurally-malformed payload is the
//! "rejected whole" case here, at parse time — nothing is staged. Exposed as a library
//! item (like `invoke`) so it is driven by unit tests independent of the binary's
//! command dispatch, which wires it in the next task.
//!
//! **Boundary:** the agent authors the payload (the prose + which-content-goes-where);
//! the parser only flattens its declared structure into leaf coordinates the CLI
//! places. The slot/field split is read straight off the value syntax — a value
//! wrapped in `<<…>>` is slot prose (the delimiters stripped), any other scalar is an
//! inline field value (the pinned `<<slot>>`/scalar convention).

use anyhow::{Context, Result};
use engine::slug::slugify;
use serde::Deserialize;
use std::collections::BTreeMap;

/// The parsed batch payload, lowered to a flat ordered leaf-write plan. The `title`
/// is the `create` id-source (the `--title` equivalent); `leaves` is the ordered
/// `add-item` / `set-*` sequence applied over the single buffer after the create.
#[derive(Debug, PartialEq, Eq)]
pub struct AuthorPlan {
    /// The create id-source — slugged + minted by the shared `create_gated` path.
    pub title: String,
    /// The ordered leaf-write sequence (document order: each section in turn, its
    /// section-level leaves then its items; each item's `add-item` precedes that
    /// item's own leaves and its nested items).
    pub leaves: Vec<Leaf>,
}

/// One lowered leaf-write. The `fragment` is the address tail relative to the
/// created instance (`#…` minus the `<doctype>:<slug>` head) — the same fragment the
/// existing `doc` resolvers parse, so the verb dispatch reuses them verbatim.
#[derive(Debug, PartialEq, Eq)]
pub enum Leaf {
    /// Mint a repeatable item into the section the `fragment` addresses, id-slugged
    /// from `title` (top-level `#section`, or nested `#section/parent/.../nested`).
    AddItem { fragment: String, title: String },
    /// Splice an inline scalar field value at the `fragment` leaf.
    SetField { fragment: String, value: String },
    /// Splice slot prose at the `fragment` leaf (the `<<…>>` delimiters stripped).
    SetSlot { fragment: String, prose: String },
}

/// The declarative whole-doc payload, as authored. A `title` (the create id-source)
/// plus an ordered list of sections; each section carries doc-level leaves (`set`)
/// and/or repeatable `items`, and each item carries its own leaves and nested
/// sections — mirroring the document's structure. `deny_unknown_fields` makes a
/// typo'd key a parse-time reject (the "rejected whole" path), not a silent drop.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorPayload {
    title: String,
    #[serde(default)]
    sections: Vec<PayloadSection>,
}

/// A section in the payload: its `id`, optional doc-level `set` leaves (for a simple
/// section — fields and the section's slot), and optional repeatable `items`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PayloadSection {
    id: String,
    #[serde(default)]
    set: BTreeMap<String, String>,
    #[serde(default)]
    items: Vec<PayloadItem>,
}

/// A repeatable item: its `title` (the item id-source), optional `set` leaves
/// (per-item fields + slots), and optional nested `sections` (the next repeatable
/// level — recursed exactly like a top-level section, parented by this item).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PayloadItem {
    title: String,
    #[serde(default)]
    set: BTreeMap<String, String>,
    #[serde(default)]
    sections: Vec<PayloadSection>,
}

/// A classified `set` value: slot prose (the `<<…>>`-wrapped form, delimiters
/// stripped) versus an inline scalar field value.
enum LeafValue {
    Field(String),
    Slot(String),
}

/// Classify a `set` value by the pinned convention: a value that (after trimming
/// surrounding whitespace) opens with `<<` and closes with `>>` is **slot prose**
/// with those delimiters stripped; anything else is an **inline field** value
/// (carried unchanged — the splice path trims its own ends).
fn classify(raw: &str) -> LeafValue {
    let trimmed = raw.trim();
    if trimmed.len() >= 4 && trimmed.starts_with("<<") && trimmed.ends_with(">>") {
        LeafValue::Slot(trimmed[2..trimmed.len() - 2].to_string())
    } else {
        LeafValue::Field(raw.to_string())
    }
}

/// Parse a declarative batch payload (YAML) into the ordered leaf-write [`AuthorPlan`].
/// A structurally-malformed payload (bad YAML, a missing required `title`/item-`title`,
/// or an unknown key) is rejected **whole**, here, before anything could persist.
pub fn parse_author_payload(payload: &str) -> Result<AuthorPlan> {
    let parsed: AuthorPayload =
        serde_yaml_ng::from_str(payload).context("malformed `doc author` payload")?;
    let mut leaves = Vec::new();
    for section in &parsed.sections {
        flatten_section(section, &[], &mut leaves);
    }
    Ok(AuthorPlan {
        title: parsed.title,
        leaves,
    })
}

/// Flatten one section (and its items, recursively) onto `leaves`, in document order.
/// `parent` is the item id-chain hops above this section (empty at the document root,
/// the enclosing item's chain when this is a nested section). Item ids are minted with
/// the same [`slugify`] the engine `add_item` uses, so the chain the parser builds for
/// a nested leaf matches the id the engine mints — by construction.
fn flatten_section(section: &PayloadSection, parent: &[String], leaves: &mut Vec<Leaf>) {
    // Doc-level (simple-section) leaves: a scalar field is addressed `…/<section>/<key>`;
    // the section's slot is the section itself (`…/<section>`, no key hop).
    for (key, raw) in &section.set {
        match classify(raw) {
            LeafValue::Field(value) => leaves.push(Leaf::SetField {
                fragment: join(parent, [section.id.as_str(), key.as_str()]),
                value,
            }),
            LeafValue::Slot(prose) => leaves.push(Leaf::SetSlot {
                fragment: join(parent, [section.id.as_str()]),
                prose,
            }),
        }
    }

    let mut section_hops = parent.to_vec();
    section_hops.push(section.id.clone());
    for item in &section.items {
        leaves.push(Leaf::AddItem {
            fragment: section_hops.join("/"),
            title: item.title.clone(),
        });

        let mut item_hops = section_hops.clone();
        item_hops.push(slugify(&item.title));
        for (key, raw) in &item.set {
            match classify(raw) {
                LeafValue::Field(value) => leaves.push(Leaf::SetField {
                    fragment: join(&item_hops, [key.as_str()]),
                    value,
                }),
                LeafValue::Slot(prose) => leaves.push(Leaf::SetSlot {
                    fragment: join(&item_hops, [key.as_str()]),
                    prose,
                }),
            }
        }
        for nested in &item.sections {
            flatten_section(nested, &item_hops, leaves);
        }
    }
}

/// Join a hop prefix with trailing hops into a `/`-separated fragment.
fn join<'a>(prefix: &[String], tail: impl IntoIterator<Item = &'a str>) -> String {
    let mut hops: Vec<String> = prefix.to_vec();
    hops.extend(tail.into_iter().map(str::to_string));
    hops.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A multi-release **dated-changelog-shaped** payload lowers to the expected
    /// ordered plan: the `create` id-source, then — in document order — each release's
    /// `add-item`, its inline `link` field, the nested-`changes` `add-item`, and the
    /// change-group's `<<…>>` slot prose. Item ids in the nested fragments are the
    /// `slugify`'d titles (`Added` → `added`), matching what the engine mints. The
    /// `set` maps deterministically order their leaves by key (`BTreeMap`).
    #[test]
    fn multi_release_changelog_lowers_to_ordered_plan() {
        let payload = r#"
title: Changelog
sections:
  - id: releases
    items:
      - title: 1-1-0
        set:
          link: "https://example.com/compare/1.0.0...1.1.0"
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- The `jigc doc author` batch verb.>>"
      - title: 1-0-0
        sections:
          - id: changes
            items:
              - title: Fixed
                set:
                  notes: "<<- A first bug fix.>>"
"#;

        let plan = parse_author_payload(payload).expect("the changelog payload parses");
        assert_eq!(plan.title, "Changelog");
        assert_eq!(
            plan.leaves,
            vec![
                Leaf::AddItem {
                    fragment: "releases".into(),
                    title: "1-1-0".into(),
                },
                Leaf::SetField {
                    fragment: "releases/1-1-0/link".into(),
                    value: "https://example.com/compare/1.0.0...1.1.0".into(),
                },
                Leaf::AddItem {
                    fragment: "releases/1-1-0/changes".into(),
                    title: "Added".into(),
                },
                Leaf::SetSlot {
                    fragment: "releases/1-1-0/changes/added/notes".into(),
                    prose: "- The `jigc doc author` batch verb.".into(),
                },
                Leaf::AddItem {
                    fragment: "releases".into(),
                    title: "1-0-0".into(),
                },
                Leaf::AddItem {
                    fragment: "releases/1-0-0/changes".into(),
                    title: "Fixed".into(),
                },
                Leaf::SetSlot {
                    fragment: "releases/1-0-0/changes/fixed/notes".into(),
                    prose: "- A first bug fix.".into(),
                },
            ],
        );
    }

    /// A doc-level simple section lowers its `set` map per the slot/field split: a
    /// scalar field is addressed `<section>/<key>`, the section's `<<…>>` slot is the
    /// section itself (no key hop) — the doctype-general (non-repeatable) shape.
    #[test]
    fn doc_level_section_leaves_split_field_and_slot_addresses() {
        let payload = "\
title: Add rate limiter
sections:
  - id: header
    set:
      scope: api
      type: feat
  - id: summary
    set:
      summary: \"<<Add a token-bucket rate limiter.>>\"
";
        let plan = parse_author_payload(payload).expect("the payload parses");
        assert_eq!(plan.title, "Add rate limiter");
        assert_eq!(
            plan.leaves,
            vec![
                // `header.set` ordered by key (BTreeMap): scope, then type.
                Leaf::SetField {
                    fragment: "header/scope".into(),
                    value: "api".into(),
                },
                Leaf::SetField {
                    fragment: "header/type".into(),
                    value: "feat".into(),
                },
                // The slot is the section itself — no key hop.
                Leaf::SetSlot {
                    fragment: "summary".into(),
                    prose: "Add a token-bucket rate limiter.".into(),
                },
            ],
        );
    }

    /// A structurally-malformed payload is rejected **whole** at parse — before any
    /// persist could run. Four shapes: a missing required `title`, an unknown top-level
    /// key (the `deny_unknown_fields` typo guard), an item missing its `title`
    /// id-source, and input that is not valid YAML.
    #[test]
    fn malformed_payload_is_rejected_whole_at_parse() {
        assert!(
            parse_author_payload("sections: []\n").is_err(),
            "a payload missing the required `title` create id-source is rejected",
        );
        assert!(
            parse_author_payload("title: X\nsectons: []\n").is_err(),
            "an unknown top-level key (a typo) is rejected, never silently dropped",
        );
        assert!(
            parse_author_payload(
                "\
title: X
sections:
  - id: releases
    items:
      - set: { link: y }
"
            )
            .is_err(),
            "an item missing its `title` id-source is rejected",
        );
        assert!(
            parse_author_payload("title: X\n  : :\n").is_err(),
            "input that is not valid YAML is rejected",
        );
    }
}
