//! Declarative whole-doc batch-authoring payload + parser — the `jigc doc author
//! <doctype> --from-file <payload>` front of the M24 batch verb (`design/write-commands.md`
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

use engine::finding::{Finding, Severity};
use engine::schema::{Field, Leaf as SchemaLeaf, Schema, Section, SectionBody, Slot};
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
    /// The create id-source. Optional at the serde layer so a **singleton** payload
    /// (whose slug is fixed to the type id) may omit it — the required-for-non-singletons
    /// check moves into [`parse_author_payload`], where the schema tells singleton from
    /// not, so the reject is an enriched message naming `title:`, not a raw serde error.
    #[serde(default)]
    title: Option<String>,
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

/// Whether a `set` value carries the `<<…>>` slot marker (after trimming surrounding
/// whitespace) — the single syntactic predicate the slot/field split rests on.
fn is_slot_wrapped(raw: &str) -> bool {
    let trimmed = raw.trim();
    trimmed.len() >= 4 && trimmed.starts_with("<<") && trimmed.ends_with(">>")
}

/// Classify a `set` value by the pinned convention: a value that (after trimming
/// surrounding whitespace) opens with `<<` and closes with `>>` is **slot prose**
/// with those delimiters stripped; anything else is an **inline field** value
/// (carried unchanged — the splice path trims its own ends).
fn classify(raw: &str) -> LeafValue {
    if is_slot_wrapped(raw) {
        let trimmed = raw.trim();
        LeafValue::Slot(trimmed[2..trimmed.len() - 2].to_string())
    } else {
        LeafValue::Field(raw.to_string())
    }
}

/// **Every way [`parse_author_payload`] can refuse** — the class's defining case-set, and
/// the set `crates/cli/tests/author_payload_floor.rs` iterates.
///
/// **Why it exists** (M51 Increment 6, T3; the rc.14 trial's F-11). The payload parse used
/// to refuse with a bare `anyhow` sentence: no severity, no code, no `at:`, no route, and
/// `{"error": …}` on `--format json` — while the *same door* answers a declared-address miss
/// with `blocking · write.unknown-field` + `at:` + a route. M50 Increment 9 closed the
/// write-miss floor at the four target resolvers, and this parse sits **upstream of every
/// one of them**, so a driver keying on the stable `(code, target)` pair got nothing at the
/// door it reaches first. Naming the refusals as a set is what makes the repair checkable
/// over the class rather than over the reported repro.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayloadReject {
    /// The payload is not the declared YAML grammar at all — unparseable input, an unknown
    /// key (the `deny_unknown_fields` typo guard), or an item missing its `title:`
    /// id-source. Every serde-layer refusal is this one exit.
    Ungrammatical,
    /// A **non-singleton** payload carries no top-level `title:` — the create id-source. A
    /// singleton's slug is fixed to its type id, so a title-less singleton payload is not a
    /// refusal at all (it defaults).
    MissingTitle,
    /// A value for a schema-declared **slot** leaf is not `<<…>>`-wrapped — the
    /// silent-misroute guard's first direction.
    BareSlotValue,
    /// A value for a schema-declared **field** leaf *is* `<<…>>`-wrapped — the guard's
    /// second direction.
    WrappedFieldValue,
}

impl PayloadReject {
    /// Every refusal exit, in declaration order.
    pub const ALL: &'static [PayloadReject] = &[
        PayloadReject::Ungrammatical,
        PayloadReject::MissingTitle,
        PayloadReject::BareSlotValue,
        PayloadReject::WrappedFieldValue,
    ];

    /// The shipped **write-family** code this refusal earns, under one stated
    /// discriminator: `write.wrong-shape` for a payload **the grammar does not admit**, and
    /// `write.malformed-value` for a **value** whose `<<…>>` form contradicts its declared
    /// leaf kind. No code is minted here — both are the engine's own
    /// (`design/validation.md` → the `write.*` route split), which is what keeps this door
    /// speaking the same taxonomy as the five addressed write verbs its payload lowers onto.
    pub fn code(self) -> &'static str {
        match self {
            PayloadReject::Ungrammatical | PayloadReject::MissingTitle => "write.wrong-shape",
            PayloadReject::BareSlotValue | PayloadReject::WrappedFieldValue => {
                "write.malformed-value"
            }
        }
    }

    /// The variant's own identifier, as the source spells it — the handle the completeness
    /// arm scans this module's production source for, so a refusal exit added outside
    /// [`reject`] cannot land silently.
    pub fn variant(self) -> &'static str {
        match self {
            PayloadReject::Ungrammatical => "Ungrammatical",
            PayloadReject::MissingTitle => "MissingTitle",
            PayloadReject::BareSlotValue => "BareSlotValue",
            PayloadReject::WrappedFieldValue => "WrappedFieldValue",
        }
    }
}

/// The **one** finding-minting seam of the payload parse: a blocking [`Finding`] carrying
/// the refusal's declared code and its sentence.
///
/// It deliberately carries **neither a location nor a route**, and both absences are filled
/// one layer up at [`crate::doc`]'s shared write-path block seam, which is the only place
/// that holds what they need: the **doctype id** — this door's declared `key.target` form,
/// since nothing is staged when a payload is refused, so no instance exists to address
/// (`design/command-output-contract.md` → the `create.*` doctype-scoped target form) — and
/// the verb the recovery re-runs. The route that seam supplies is the declared one for a
/// **payload** defect: the write persisted nothing, so re-running the same verb with a
/// corrected payload is the recovery (`engine::write`'s per-code route map states exactly
/// that for this family).
///
/// The error *type* is what closes the escape: `Result<_, Finding>` is a type a bare
/// `anyhow` cannot inhabit, so a future refusal exit cannot slip back out of the envelope
/// by construction rather than by a later grep (M50 Increment 9's shape).
fn reject(kind: PayloadReject, message: String) -> Finding {
    Finding::graded(Severity::Blocking, kind.code(), message, None, None)
}

/// The leaf-kind the **schema** declares for a `set` key — the source of truth the
/// syntactic `<<…>>` marker is cross-checked against.
#[derive(Clone, Copy, PartialEq, Eq)]
enum DeclaredKind {
    Slot,
    Field,
}

/// Cross-check a `set` value's syntactic form against the schema-declared leaf-kind,
/// rejecting the whole payload (at parse, before any persist) on a mismatch — the
/// silent-misroute guard (`design/auto-migration.md` → Hardening; `DECISIONS.md`
/// 2026-06-16 batch-payload serialization). The `<<…>>` marker stays the *settled*
/// payload format: this does **not** re-route by schema, it makes a deviation **loud
/// and located** so a bare value for a slot leaf (silently misrouted into a trailing
/// `<!-- fields -->` block today) and a `<<…>>`-wrapped value for a field leaf are
/// both hard parse-time rejects naming the offending address.
fn check_kind(declared: DeclaredKind, raw: &str, addr: &str) -> Result<(), Finding> {
    match (declared, is_slot_wrapped(raw)) {
        (DeclaredKind::Slot, false) => Err(reject(
            PayloadReject::BareSlotValue,
            format!(
                "write rejected: the value for slot `{addr}` must be wrapped in <<…>> \
                 (the literal `<<`/`>>` markers are required syntax, not a placeholder to delete) — got: {raw}"
            ),
        )),
        (DeclaredKind::Field, true) => Err(reject(
            PayloadReject::WrappedFieldValue,
            format!(
                "write rejected: the value for field `{addr}` must not be wrapped in <<…>> \
                 — `<<…>>` marks slot prose, and `{addr}` is a declared inline field"
            ),
        )),
        _ => Ok(()),
    }
}

/// The schema view at one section level: the leaf-kind lookups a payload section's
/// `set` keys and items are cross-checked against. Derived from a top-level
/// [`Section`] or a nested [`SchemaLeaf::Repeatable`]; [`SchemaCtx::None`] when the
/// payload addresses a section the schema does not declare — left to the downstream
/// `write.unknown-section` reject the lowered leaf's target resolver raises from the
/// engine's own rank-1 predicate. (True since M49 Increment 11 / T3: before it, that
/// resolver refused with a bare code-less error, so this sentence named a reject the
/// batch door did not produce.) The cross-check is purely additive.
enum SchemaCtx<'a> {
    None,
    Simple {
        slot: &'a Option<Slot>,
        fields: &'a [Field],
    },
    Repeatable {
        block: &'a [SchemaLeaf],
    },
}

impl<'a> SchemaCtx<'a> {
    /// The context for a top-level payload section, resolved against the schema by id.
    fn from_section(section: Option<&'a Section>) -> Self {
        match section.map(|s| &s.body) {
            Some(SectionBody::Simple { slot, fields }) => SchemaCtx::Simple { slot, fields },
            Some(SectionBody::Repeatable { repeatable }) => SchemaCtx::Repeatable {
                block: &repeatable.block,
            },
            None => SchemaCtx::None,
        }
    }

    /// The context for a payload section nested under an item — the [`SchemaLeaf::
    /// Repeatable`] this level's block declares with the matching id, if any.
    fn nested(&self, id: &str) -> Self {
        let SchemaCtx::Repeatable { block } = self else {
            return SchemaCtx::None;
        };
        block
            .iter()
            .find_map(|leaf| match leaf {
                SchemaLeaf::Repeatable {
                    id: leaf_id,
                    repeatable,
                } if leaf_id == id => Some(SchemaCtx::Repeatable {
                    block: &repeatable.block,
                }),
                _ => None,
            })
            .unwrap_or(SchemaCtx::None)
    }

    /// The declared leaf-kind of a **section-level** `set` key (a simple section): a
    /// declared field id is a field; the **section's own id** is its slot, when it has
    /// one (the documented key — `doc author --help`, and what the `{{schema:}}` payload
    /// skeleton generates). Any other key names a leaf the section does not declare, so
    /// it yields no kind: the cross-check stands down and the lowering carries the key
    /// into the address, where the write caller rejects it (M47 — the undeclared-address
    /// table). Keying the slot on *any* non-field id would make the `<<…>>` cross-check
    /// demand the wrapper for a key that has no home either way.
    fn section_set_kind(&self, key: &str, section_id: &str) -> Option<DeclaredKind> {
        match self {
            SchemaCtx::Simple { slot, fields } => {
                if fields.iter().any(|f| f.id == key) {
                    Some(DeclaredKind::Field)
                } else if slot.is_some() && key == section_id {
                    Some(DeclaredKind::Slot)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// The declared leaf-kind of an **item-level** `set` key — the matching block leaf
    /// (a nested repeatable is not a `set` key, so it yields no kind).
    fn item_set_kind(&self, key: &str) -> Option<DeclaredKind> {
        let SchemaCtx::Repeatable { block } = self else {
            return None;
        };
        block.iter().find_map(|leaf| match leaf {
            SchemaLeaf::Slot { id, .. } if id == key => Some(DeclaredKind::Slot),
            SchemaLeaf::Field(field) if field.id == key => Some(DeclaredKind::Field),
            _ => None,
        })
    }
}

/// Parse a declarative batch payload (YAML) into the ordered leaf-write [`AuthorPlan`].
/// A structurally-malformed payload (bad YAML, a missing item-`title`, or an unknown
/// key) is rejected **whole**, here, before anything could persist. A missing top-level
/// `title:` is rejected too **for a non-singleton** (with an enriched message naming
/// `title:`), but **defaults to the type id for a singleton** — whose slug is fixed to
/// the type id, so the id-source is ignored downstream (M45 Inc 10 T10).
///
/// The `schema` is the doctype the payload authors (`None` when the doctype is unknown
/// — that case is rejected by the create-gate downstream): it is threaded through the
/// lowering only to **cross-check** each `set` value's `<<…>>` form against the declared
/// leaf-kind (see [`check_kind`]) — a kind mismatch is a parse-time reject, so a bare
/// value for a slot leaf can no longer be silently misrouted into a `<!-- fields -->`
/// block. Where the payload addresses a section/leaf the schema does not declare, the
/// cross-check stands down and the engine's downstream reject handles it.
pub fn parse_author_payload(schema: Option<&Schema>, payload: &str) -> Result<AuthorPlan, Finding> {
    let parsed: AuthorPayload = serde_yaml_ng::from_str(payload).map_err(|err| {
        reject(
            PayloadReject::Ungrammatical,
            format!("write rejected: the `doc author` payload is not the declared grammar — {err}"),
        )
    })?;
    // A singleton's slug is fixed to the type id — `create_gated` ignores the id-source
    // — so a title-less singleton payload defaults its plan title to the title the
    // schema fixes ([`Schema::fixed_title`], the same value the `{{schema:}}` skeleton
    // renders literally and the mint writes as the `# H1`) and succeeds. A title-less
    // non-singleton (or unknown-schema) payload is rejected with an enriched message
    // naming `title:`, never the raw `missing field` serde error (M45 Inc 10 T10 —
    // findings §95 E).
    let title = match parsed.title {
        Some(title) => title,
        None => match schema.and_then(|schema| schema.fixed_title()) {
            Some(fixed) => fixed,
            _ => {
                return Err(reject(
                    PayloadReject::MissingTitle,
                    "write rejected: the `doc author` payload declares no `title:` — the \
                     create id-source (the top-level `title:` line naming the instance); only \
                     a singleton doctype, whose slug is fixed to its type id, may omit it"
                        .to_string(),
                ));
            }
        },
    };
    let mut leaves = Vec::new();
    for section in &parsed.sections {
        let ctx = SchemaCtx::from_section(
            schema.and_then(|s| s.sections.iter().find(|sec| sec.id == section.id)),
        );
        flatten_section(&ctx, section, &[], &mut leaves)?;
    }
    Ok(AuthorPlan { title, leaves })
}

/// Flatten one section (and its items, recursively) onto `leaves`, in document order.
/// `parent` is the item id-chain hops above this section (empty at the document root,
/// the enclosing item's chain when this is a nested section). Item ids are minted with
/// the same [`slugify`] the engine `add_item` uses, so the chain the parser builds for
/// a nested leaf matches the id the engine mints — by construction. `ctx` is this
/// section's schema view, used to cross-check the slot/field form of every `set` value.
fn flatten_section(
    ctx: &SchemaCtx,
    section: &PayloadSection,
    parent: &[String],
    leaves: &mut Vec<Leaf>,
) -> Result<(), Finding> {
    // Doc-level (simple-section) leaves: a scalar field is addressed `…/<section>/<key>`;
    // the section's slot is the section itself (`…/<section>`, no key hop) — and that
    // key-less form is reached **only** by the section's own id, the documented key.
    // Any other slot-shaped key keeps its hop (`…/<section>/<key>`), so the write caller
    // adjudicates it exactly as the per-leaf `set-slot` verb does rather than the
    // lowering silently dropping it onto the section's real slot (M47 — the
    // undeclared-address table; `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 9).
    for (key, raw) in &section.set {
        if let Some(declared) = ctx.section_set_kind(key, &section.id) {
            check_kind(
                declared,
                raw,
                &join(parent, [section.id.as_str(), key.as_str()]),
            )?;
        }
        match classify(raw) {
            LeafValue::Field(value) => leaves.push(Leaf::SetField {
                fragment: join(parent, [section.id.as_str(), key.as_str()]),
                value,
            }),
            LeafValue::Slot(prose) if key == &section.id => leaves.push(Leaf::SetSlot {
                fragment: join(parent, [section.id.as_str()]),
                prose,
            }),
            LeafValue::Slot(prose) => leaves.push(Leaf::SetSlot {
                fragment: join(parent, [section.id.as_str(), key.as_str()]),
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
            if let Some(declared) = ctx.item_set_kind(key) {
                check_kind(declared, raw, &join(&item_hops, [key.as_str()]))?;
            }
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
            flatten_section(&ctx.nested(&nested.id), nested, &item_hops, leaves)?;
        }
    }
    Ok(())
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
    use engine::schema::load_schema;

    /// The shipped `changelog` schema (two-level repeatable: releases → changes), the
    /// cross-check target for the changelog-shaped payloads.
    fn changelog_schema() -> Schema {
        load_schema(include_bytes!(crate::pack_path!(
            dev,
            "schemas/changelog.yaml"
        )))
        .expect("shipped changelog schema loads")
    }

    /// The shipped `commit` schema (`header` fields + a `summary`/`body` slot), the
    /// cross-check target for the doc-level simple-section payload.
    fn commit_schema() -> Schema {
        load_schema(include_bytes!(crate::pack_path!(
            dev,
            "schemas/commit.yaml"
        )))
        .expect("shipped commit schema loads")
    }

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

        let plan = parse_author_payload(Some(&changelog_schema()), payload)
            .expect("the changelog payload parses");
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
        let plan =
            parse_author_payload(Some(&commit_schema()), payload).expect("the payload parses");
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
            parse_author_payload(None, "sections: []\n").is_err(),
            "a payload missing the required `title` create id-source is rejected",
        );
        assert!(
            parse_author_payload(None, "title: X\nsectons: []\n").is_err(),
            "an unknown top-level key (a typo) is rejected, never silently dropped",
        );
        assert!(
            parse_author_payload(
                None,
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
            parse_author_payload(None, "title: X\n  : :\n").is_err(),
            "input that is not valid YAML is rejected",
        );
    }

    /// A `set` value whose `<<…>>` form contradicts the schema-declared leaf-kind is a
    /// **parse-time reject** naming the offending address — the silent-misroute guard.
    /// Two directions, both over the shipped changelog schema: a bare value for the
    /// `notes` **slot** (silently misrouted into a `<!-- fields -->` block before this
    /// guard), and a `<<…>>`-wrapped value for the `link` **field**.
    #[test]
    fn leaf_kind_mismatch_is_rejected_whole_at_parse() {
        let schema = changelog_schema();

        let bare_slot = "\
title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: \"- A bare slot value.\"
";
        let err = parse_author_payload(Some(&schema), bare_slot)
            .expect_err("a bare value for the `notes` slot is rejected, never misrouted");
        let msg = &err.message;
        assert!(
            msg.contains("slot") && msg.contains("notes") && msg.contains("<<"),
            "the slot reject names the leaf-kind, the address, and the expected form: {msg}",
        );
        // The message must teach that the markers are required syntax (not a
        // placeholder to delete) and echo the offending value (#7).
        assert!(
            msg.contains("required syntax, not a placeholder to delete")
                && msg.contains("- A bare slot value."),
            "the slot reject must say the markers are required syntax and echo the value: {msg}",
        );

        let wrapped_field = "\
title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        set:
          link: \"<<https://example.com/x>>\"
";
        let err = parse_author_payload(Some(&schema), wrapped_field)
            .expect_err("a `<<…>>`-wrapped value for the `link` field is rejected");
        let msg = &err.message;
        assert!(
            msg.contains("field") && msg.contains("link") && msg.contains("<<"),
            "the field reject names the leaf-kind, the address, and the form: {msg}",
        );
    }

    /// A **singleton** doctype's `title:` is optional — its slug is fixed to the type
    /// id, so the create id-source is ignored (`create_gated` fixes the slug regardless).
    /// A title-less singleton payload therefore **succeeds**, defaulting the plan title
    /// to the type id (M45 Inc 10 T10 — findings §95 E: the required-`title` serde error
    /// on a singleton author). A title-less **non-singleton** (or unknown-schema) payload
    /// still fails, but with an **enriched** message naming `title:` — never the raw
    /// `missing field \`title\`` serde error.
    #[test]
    fn title_less_singleton_defaults_non_singleton_error_is_enriched() {
        // `changelog` is a shipped singleton (type id `changelog`): a title-less payload
        // succeeds, the plan title defaulting to the title the schema **fixes** —
        // `Schema::fixed_title`, i.e. the `display-title:` when declared, else the type
        // id. The default must be that value, not the raw type id, because the create
        // path's title pre-check compares the plan title against the `# H1` the mint will
        // write (M48; `design/write-commands.md` → The four-way write), and a title-less
        // payload must land rather than fail a comparison against a title it never chose.
        let singleton_payload = "\
sections:
  - id: releases
    items:
      - title: 1.0.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: \"<<- The first release.>>\"
";
        let plan = parse_author_payload(Some(&changelog_schema()), singleton_payload)
            .expect("a title-less singleton payload succeeds against the fixed slug");
        assert_eq!(
            plan.title,
            changelog_schema()
                .fixed_title()
                .expect("the shipped changelog is a singleton"),
            "the plan title defaults to the title the singleton's schema fixes",
        );
        assert_eq!(
            plan.title, "Changelog",
            "and that title is the declared `display-title:`, not the type id — the value \
             the `{{schema:}}` skeleton renders and the mint writes as the `# H1`",
        );

        // `commit` is non-singleton: a title-less payload fails with an enriched message
        // that names `title:` and is NOT the raw serde `missing field` error.
        let err = parse_author_payload(Some(&commit_schema()), "sections: []\n")
            .expect_err("a title-less non-singleton payload is rejected");
        let msg = &err.message;
        assert!(
            msg.contains("title:"),
            "the enriched reject names the `title:` create id-source: {msg}",
        );
        assert!(
            !msg.contains("missing field"),
            "the reject is enriched, not the raw serde `missing field` error: {msg}",
        );
    }

    /// The cross-check stands down when the doctype schema is unknown (`None`) or the
    /// payload addresses a section the schema does not declare — the lowering stays
    /// purely structural so the engine's downstream reject handles the unknown target.
    #[test]
    fn unknown_schema_target_skips_the_cross_check() {
        let payload = "\
title: X
sections:
  - id: nonsuch
    items:
      - title: Boom
        set:
          whatever: \"a bare value for an undeclared leaf\"
";
        assert!(
            parse_author_payload(None, payload).is_ok(),
            "no schema ⇒ no cross-check; the structural lowering still succeeds",
        );
        assert!(
            parse_author_payload(Some(&changelog_schema()), payload).is_ok(),
            "an undeclared section ⇒ no cross-check; the engine rejects it downstream",
        );
    }
}
