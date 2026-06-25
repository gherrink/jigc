//! The deterministic transform **driver** — step 2 of the schema-diff → byte-stable
//! splice pipeline (`design/corpus-migration.md` → The deterministic transform).
//!
//! Consumes the classified diff produced by [`crate::schema_diff::schema_diff`] and
//! emits the ordered sequence of **byte-stable splices** over the proven `write.rs`
//! primitives that carries a v1-shaped instance onto its v2 shape — **no LLM in the
//! structural path** (the determinism boundary: the CLI owns structure, the LLM owns
//! only prose). Same classified diff in → same bytes out.
//!
//! This increment (M34 Inc-2 T3) builds the two structural branches with **no live
//! case**, provable only synthetically:
//!
//! - **`fixed-slot→repeatable-with-default`** — promote a simple `<<slot>>` section
//!   into its repeatable shape, carrying the old slot prose as the default first item,
//!   via the net-new [`crate::write::promote_slot_to_repeatable`] primitive (T2). The
//!   default item's **title is the section id** — a deterministic, domain-empty,
//!   always-slug-able source (the engine invents no prose); a human may rename the
//!   heading afterward as an ordinary edit.
//! - **`widened-cardinality`** — an instance-byte **identity** transform: the existing
//!   value stays valid under the widened cardinality, so the bytes are unchanged.
//!
//! T4 adds the **Framing-A prose-routing branch** — a `prose-needing` change that is a
//! new **required section slot**: the CLI mints the slot **empty** at its schema-ordered
//! home (via [`crate::write::generate_section`]) and stops, so the reused conformance
//! gate blocks on the empty slot until the agent authors the prose (the CLI never authors
//! it — the determinism boundary).
//!
//! M34 Inc-3 T1 builds the **`added-optional-field` branch**: a defaulted field is
//! spliced at its schema-ordered home — into an existing `---` header, or **introducing
//! the fence** for a header-less doctype (the schema-version stamp's `prd`/`changelog`
//! v0→v1 shape change). An `optional:` field with no default is a byte no-op (its
//! absence conforms); a `set`-derived field with no default needs the caller-supplied
//! value the T4 dogfood threads in (unbuilt here).
//!
//! Two branches stay deferred, surfaced as [`TransformError::Unsupported`] rather than
//! silently skipped — an un-built branch must block, never drop a change: the
//! `prose-needing` **field** sub-case (a new required *field*, not a slot — the transform
//! mints slots, not fields) and the `set`-derived add-field value (M34 Inc-3 T4).
//!
//! # The per-doc-gated corpus fold (M34 Inc-2 T5)
//!
//! [`migrate_corpus`] lifts the single-instance [`transform`] to an **N-doc corpus**
//! while upholding the *writes-are-transactional* invariant (`CLAUDE.md`;
//! `corpus-migration.md` → census *Migration atomicity / WIP-safety* row;
//! `finalize.md` → the transactional boundary). The corpus is **heterogeneous**: each
//! [`CorpusDoc`] carries its own schema pair + classified diff, mirroring the real
//! corpus the M34 Inc-3 verb migrates (mixed doctypes, each with its own v1→v2 diff).
//!
//! **Per-doc transaction granularity.** The transaction unit is **one doc**, never the
//! whole corpus. Each doc is migrated in a *scratch buffer* and gated on conformance:
//!
//! 1. **apply** — `transform` folds the doc's classified diff into scratch bytes;
//! 2. **gate** — the scratch re-parses under the doc's v2 schema **and** carries **zero**
//!    [`crate::validate::schema_conformance`] findings;
//! 3. **commit** — only a clean gate records the doc as [`DocOutcome::Migrated`] (v2).
//!    Anything else — a [`TransformError`] (an un-built branch), a parse failure, or a
//!    non-empty gate (the `prose-needing` mint-empty leaves a `required-slot-present`
//!    break that **blocks until the agent authors it**) — **rolls the scratch back**:
//!    the doc stays [`DocOutcome::Untouched`], byte-identical v0, and the corpus run
//!    **halts cleanly** at that doc. Docs after the halt are never touched.
//!
//! So an interrupted corpus (the Framing-A prose-authoring handoff) leaves the docs
//! before the halt fully transformed-and-conformant (v2) and the doc-at-halt plus every
//! doc after it byte-identical v0 — both halves **independently re-detectable** by the
//! Inc-1 fifth-family detector (conformant-v2 → done; non-conformant-v0 → routed
//! `migrate`). **No doc is ever left half-transformed**: a doc's bytes only ever change
//! as one all-or-nothing commit.
//!
//! **The rollback inventory** (the M35-rename rollback-inventory discipline, pinned here
//! per the charter at *the increment that builds the transform*). A per-doc commit, when
//! the **M34 Inc-3 on-disk verb** consumes this model, touches exactly — and rolls back
//! exactly — three things, **all keyed by the one doc**:
//!
//! - **path** — the doc's own file, rewritten in place (at most one path per doc; a
//!   transform never spans files). *Rollback:* the scratch is discarded before any write,
//!   so an un-committed doc leaves the on-disk file byte-identical v0 (zero residue).
//! - **file-state** — the doc's [`crate::file_state::FileStateRecord`] entry, re-hashed
//!   after the rewrite, **and** its schema-version stamp — which **flips last**, only
//!   inside a clean commit (`corpus-migration.md` → the stamp-flips-last rule; the stamp
//!   itself lands in Inc-3). *Rollback:* no re-hash, no stamp flip — the record still
//!   reads the v0 hash + v0 stamp, so the detector re-routes the doc as `migrate`.
//! - **index** — the edge-index entries the doc contributes (re-extracted from its v2
//!   refs). *Rollback:* the working-overlay edges for the doc are dropped; the committed
//!   index is untouched (the verb overlays per-doc, commits per-doc).
//!
//! Inc-2 builds the **in-memory** fold only (the scratch is a `String`; rollback is
//! discarding it). The on-disk file/git write-back + the live stamp flip + the actual
//! rollback **orchestration** land in the Inc-3 verb that consumes [`migrate_corpus`];
//! this doc-comment is where their per-doc granularity + inventory are **pinned**.

use crate::field_block::{Field, Value};
use crate::parse::parse_sections;
use crate::schema::{Schema, SectionBody};
use crate::schema_diff::SchemaChange;
use crate::validate::schema_conformance;
use crate::write::{self, GenerateError};

/// A failure applying a classified diff to one instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransformError {
    /// A structural splice primitive failed (an absent/non-conformant target, an
    /// unslug-able promotion title, …). Carries the underlying [`GenerateError`].
    Generate(GenerateError),
    /// A classified change kind whose driver branch is **not built** — the
    /// `prose-needing` **field** sub-case (a new required field; T4 mints only slots) and
    /// a `set`-derived `added-optional-field` with no static default (its value is the
    /// deriver's, threaded in by M34 Inc-3 T4). Surfaced, never silently skipped, so an
    /// un-built branch blocks the migration rather than dropping a change.
    Unsupported {
        /// The classified kind's wire name.
        kind: &'static str,
        /// The section the change concerns.
        section: String,
    },
}

impl From<GenerateError> for TransformError {
    fn from(err: GenerateError) -> Self {
        TransformError::Generate(err)
    }
}

/// Apply a classified diff to one v1-shaped `source`, returning its v2-shaped bytes.
///
/// Folds the `changes` in classifier (document) order, each emitting a byte-stable
/// splice over the prior result. `old_schema` / `new_schema` are the schema pair the
/// diff was classified from (the promotion primitive re-derives the old slot prose
/// through `old_schema` and places it under `new_schema`'s repeatable shape).
pub fn transform(
    old_schema: &Schema,
    new_schema: &Schema,
    source: &str,
    changes: &[SchemaChange],
) -> Result<String, TransformError> {
    let mut out = source.to_string();
    for change in changes {
        match change {
            SchemaChange::FixedSlotToRepeatable { section } => {
                // The default first item's title is the section id (deterministic,
                // domain-empty, always slug-able). The primitive carries the old slot
                // prose verbatim as that item's body.
                out = write::promote_slot_to_repeatable(
                    old_schema, new_schema, &out, section, section,
                )?;
            }
            SchemaChange::WidenedCardinality { .. } => {
                // Instance-byte identity: the existing value is still valid under the
                // widened cardinality, so no splice is needed.
            }
            SchemaChange::ProseNeeding {
                section,
                leaf: None,
            } => {
                // Framing A — the prose-routing branch (T4). A new **required section
                // slot** has no deterministic default, so the CLI mints it **empty** at
                // its schema-ordered home and stops: the reused conformance gate then
                // blocks on the empty slot (`required-slot-present`) until the agent
                // authors the prose. The CLI owns only placement; it never authors the
                // prose — the determinism boundary (`corpus-migration.md` → Prose
                // routing). The minted block is byte-stable by construction
                // (`generate_section`), and `set_slot` fills it byte-stably once authored.
                out = write::generate_section(new_schema, &out, section, Some(""), &[])?;
            }
            SchemaChange::ProseNeeding {
                section,
                leaf: Some(_),
            } => {
                // A new required **field** with no default is also prose-needing, but the
                // transform's prose routing mints a **slot**, not a field
                // (`corpus-migration.md` → Prose routing: "mints the empty slot"). No
                // shipped or reconstructed migration needs the field sub-case, so its
                // driver branch is unbuilt — surfaced, never silently dropped (an un-built
                // branch must block the migration rather than drop the change).
                return Err(TransformError::Unsupported {
                    kind: "prose-needing",
                    section: section.clone(),
                });
            }
            SchemaChange::AddedOptionalField { section, field } => {
                out = apply_added_field(new_schema, &out, section, field)?;
            }
        }
    }
    Ok(out)
}

/// Splice an `added-optional-field` change: materialize the new field `field` of
/// `section` at its schema-ordered home, carrying a **deterministic** value.
///
/// The value comes from the field's schema `default` (`corpus-migration.md` → the
/// add-field branch). A header field is placed inside the `---` fence (introducing the
/// fence for a header-less doctype) via [`write::insert_front_matter_field`]; a body
/// field via [`write::insert_field`].
///
/// Two sub-cases carry **no** static default:
/// - **`optional:` with no default** — its absence already conforms, so there is
///   nothing deterministic to place: a byte **no-op** (the widened-cardinality sibling).
/// - **a `set`-derived field with no default** (the schema-version stamp) — its value is
///   caller-supplied by the deriver M34 Inc-3 T4 threads in; unbuilt here, so it is
///   surfaced as [`TransformError::Unsupported`] rather than silently dropped.
fn apply_added_field(
    new_schema: &Schema,
    source: &str,
    section: &str,
    field: &str,
) -> Result<String, TransformError> {
    let unsupported = || TransformError::Unsupported {
        kind: "added-optional-field",
        section: section.to_string(),
    };
    let sec = new_schema
        .sections
        .iter()
        .find(|s| s.id == section)
        .ok_or_else(unsupported)?;
    let decl = match &sec.body {
        SectionBody::Simple { fields, .. } => fields.iter().find(|f| f.id == field),
        SectionBody::Repeatable { .. } => None,
    }
    .ok_or_else(unsupported)?;

    let value = match &decl.default {
        Some(default) => default.clone(),
        // No deterministic value: an optional field's absence stays conformant (no-op);
        // a `set`-derived field needs the caller-supplied value (T4).
        None if decl.optional => return Ok(source.to_string()),
        None => return Err(unsupported()),
    };

    let new_field = Field {
        key: field.to_string(),
        value: Value::Scalar(value),
    };
    if sec.header {
        Ok(write::insert_front_matter_field(
            new_schema, source, section, &new_field,
        )?)
    } else {
        Ok(write::insert_field(
            new_schema, source, section, &new_field,
        )?)
    }
}

/// One doc's migration job in a corpus batch — its bytes plus the schema pair and
/// classified diff that carry it v1→v2. Heterogeneous by design: each doc owns its
/// schema pair, mirroring the mixed-doctype corpus the Inc-3 verb migrates.
pub struct CorpusDoc<'a> {
    /// A stable identifier for the doc (its path stand-in — the rollback inventory + the
    /// halt point are keyed by it; the Inc-3 verb supplies the real repo-relative path).
    pub id: &'a str,
    /// The schema the doc was authored against (its v1/v0 shape).
    pub old_schema: &'a Schema,
    /// The schema the doc is being carried onto (its v2 shape).
    pub new_schema: &'a Schema,
    /// The doc's current (v0-shaped) bytes.
    pub source: &'a str,
    /// The classified diff (from [`crate::schema_diff::schema_diff`]) for this doc's pair.
    pub changes: &'a [SchemaChange],
}

/// The outcome of one doc's per-doc transaction — all-or-nothing (never half).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocOutcome {
    /// The doc transformed-and-conformant against its v2 schema. Carries the v2 bytes.
    Migrated {
        /// The doc's stable id.
        id: String,
        /// The committed v2 bytes (byte-stable, conformant).
        v2: String,
    },
    /// The doc is **byte-identical v0**: either it triggered the halt (its transform was
    /// blocked / non-conformant) or it sits **after** the halt point. Carries the
    /// unchanged original bytes.
    Untouched {
        /// The doc's stable id.
        id: String,
        /// The original v0 bytes, unchanged.
        v0: String,
    },
}

/// The result of a per-doc-gated corpus migration: each doc's outcome (aligned to input
/// order) plus the halt point, if any.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusMigration {
    /// Per-doc outcomes, in input order (the caller supplies a deterministic order — the
    /// Inc-3 verb feeds docs path-sorted; the fold faithfully preserves whatever order it
    /// is given, since the halt point is inherently positional).
    pub docs: Vec<DocOutcome>,
    /// The index of the doc that halted the run (a blocked / non-conformant doc), or
    /// `None` if every doc migrated cleanly.
    pub halted_at: Option<usize>,
}

/// Apply a transform across an **N-doc corpus**, per-doc gated and WIP-safe.
///
/// Each doc is migrated in a scratch buffer and committed **only** when its transform
/// conforms to its v2 schema; the first doc that cannot commit (a [`TransformError`], a
/// parse failure, or a non-empty conformance gate — e.g. the `prose-needing` mint-empty
/// that blocks until authored) **halts the run cleanly**: that doc and every doc after it
/// stay byte-identical v0. See the module doc-comment for the per-doc transaction
/// granularity + the rollback inventory.
pub fn migrate_corpus(docs: &[CorpusDoc<'_>]) -> CorpusMigration {
    let mut out = Vec::with_capacity(docs.len());
    let mut halted_at: Option<usize> = None;
    for (i, doc) in docs.iter().enumerate() {
        let migrated = if halted_at.is_some() {
            // After the halt: never touched — byte-identical v0.
            None
        } else {
            try_migrate_doc(doc)
        };
        match migrated {
            Some(v2) => out.push(DocOutcome::Migrated {
                id: doc.id.to_string(),
                v2,
            }),
            None => {
                if halted_at.is_none() {
                    halted_at = Some(i);
                }
                out.push(DocOutcome::Untouched {
                    id: doc.id.to_string(),
                    v0: doc.source.to_string(),
                });
            }
        }
    }
    CorpusMigration {
        docs: out,
        halted_at,
    }
}

/// One doc's per-doc transaction: transform into a scratch buffer, then gate on
/// conformance against the v2 schema. Returns the committed v2 bytes on a clean gate, or
/// `None` (rollback — the doc stays v0) on any failure: a transform error, a parse
/// failure under v2, or a non-empty conformance gate (the `prose-needing` mint-empty
/// blocks here).
fn try_migrate_doc(doc: &CorpusDoc<'_>) -> Option<String> {
    let scratch = transform(doc.old_schema, doc.new_schema, doc.source, doc.changes).ok()?;
    let parsed = parse_sections(doc.new_schema, &scratch).ok()?;
    if schema_conformance(doc.new_schema, &scratch, &parsed).is_empty() {
        Some(scratch)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    //! The reconstructed-shape e2e: replay two historical v1→v2 shape changes on
    //! v0-shaped instances and assert each transformed instance (a) round-trips
    //! byte-identical (`render(parse(out)) == out`), (b) preserves every prior
    //! slot/field value (no-data-loss), (c) conforms against the v2 schema, and (d)
    //! is deterministic — all with **no LLM** in the path (the driver is pure
    //! `schema_diff` + `write` splices).

    use super::*;
    use crate::field_block::{Field, Value};
    use crate::parse::parse_sections;
    use crate::schema::load_schema;
    use crate::schema_diff::schema_diff;
    use crate::validate::schema_conformance;
    use crate::write::{Instance, SectionContent, instance_from_source, render};

    /// Parse `out` against `schema` and assert it carries **zero** conformance
    /// findings — the `conformance_for` clean half of the done-criterion.
    fn assert_conforms(schema: &Schema, out: &str) {
        let doc = parse_sections(schema, out).expect("transformed doc parses under v2");
        let findings = schema_conformance(schema, out, &doc);
        assert!(
            findings.is_empty(),
            "transformed doc must conform to v2; got {findings:?}"
        );
    }

    /// `render(parse(out)) == out` under the v2 schema — the byte-stability half.
    fn assert_byte_stable(schema: &Schema, out: &str) {
        let inst = instance_from_source(schema, out).expect("transformed doc re-parses under v2");
        assert_eq!(
            render(schema, &inst),
            out,
            "transformed doc must round-trip byte-identical"
        );
    }

    // ---- (a) the reconstructed M25 `prd.requirements` fixed-slot→repeatable reshape ----

    /// v0/v1: `prd.requirements` is a **simple slot** (sitting between a leading
    /// `vision` slot and a trailing `success` slot, so the splice is on a non-trailing
    /// section).
    fn prd_v1() -> Schema {
        load_schema(
            b"\
type: prd
sections:
  - id: vision
    slot: { hint: \"the product vision\" }
  - id: requirements
    slot: { hint: \"the requirements prose\" }
  - id: success
    slot: { hint: \"success criteria\" }
",
        )
        .expect("prd v1 loads")
    }

    /// v2: `prd.requirements` is now a **single-slot repeatable** (`title` id-from +
    /// one `statement` slot — the reconstructed M25 reshape). The neighbour sections
    /// are byte-identical to v1.
    fn prd_v2() -> Schema {
        load_schema(
            b"\
type: prd
sections:
  - id: vision
    slot: { hint: \"the product vision\" }
  - id: requirements
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one requirement\" } }
  - id: success
    slot: { hint: \"success criteria\" }
",
        )
        .expect("prd v2 loads")
    }

    /// A canonical v0-shaped PRD whose `requirements` slot carries multi-paragraph
    /// prose (built through [`render`] so the input is the exact byte-stable form a
    /// first-touch-canonicalized corpus doc has).
    fn prd_v0_doc() -> (String, String) {
        let prose = "The system must accept orders.\n\nIt must reject duplicate \
                     submissions within a 60s window.";
        let inst = Instance {
            title: "Checkout PRD".to_string(),
            sections: vec![
                SectionContent {
                    id: "vision".to_string(),
                    slot: Some("A frictionless checkout.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "requirements".to_string(),
                    slot: Some(prose.to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "success".to_string(),
                    slot: Some("Cart abandonment drops 20%.".to_string()),
                    ..Default::default()
                },
            ],
        };
        (render(&prd_v1(), &inst), prose.to_string())
    }

    #[test]
    fn fixed_slot_to_repeatable_e2e_replays_the_prd_requirements_reshape() {
        let v1 = prd_v1();
        let v2 = prd_v2();
        let (src, original_prose) = prd_v0_doc();

        // The diff is computed by the real classifier (T1), then fed to the driver —
        // the driver is exercised on the emitted classification, not a hand-built list.
        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::FixedSlotToRepeatable {
                section: "requirements".to_string()
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("transform succeeds");

        // (c) conforms against the v2 schema, and (a) round-trips byte-identical.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // (b) no-data-loss: requirements is now exactly one item carrying the v0 slot
        // prose verbatim; vision/success slots are unchanged.
        let out_inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let req = out_inst
            .sections
            .iter()
            .find(|s| s.id == "requirements")
            .expect("requirements present");
        assert!(
            req.slot.is_none(),
            "the section-level slot is gone post-promotion"
        );
        assert_eq!(req.items.len(), 1, "exactly one default item");
        assert_eq!(req.items[0].slot.as_deref(), Some(original_prose.as_str()));
        let slot_of = |id: &str| {
            out_inst
                .sections
                .iter()
                .find(|s| s.id == id)
                .and_then(|s| s.slot.clone())
        };
        assert_eq!(
            slot_of("vision").as_deref(),
            Some("A frictionless checkout.")
        );
        assert_eq!(
            slot_of("success").as_deref(),
            Some("Cart abandonment drops 20%.")
        );

        // (d) determinism: the same diff over the same source yields identical bytes.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(again, out, "transform is deterministic");
    }

    // ---- (b) a synthetic `widened-cardinality` reshape (instance-byte identity) ----

    /// v1: a `spec` whose `derived-from` ref carries `card: 0..1`.
    fn spec_v1() -> Schema {
        load_schema(
            b"\
type: spec
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: prd, card: \"0..1\" }
  - id: body
    slot: { hint: \"the spec body\" }
",
        )
        .expect("spec v1 loads")
    }

    /// v2: identical but `derived-from` is widened to `card: 0..*`.
    fn spec_v2() -> Schema {
        load_schema(
            b"\
type: spec
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: prd, card: \"0..*\" }
  - id: body
    slot: { hint: \"the spec body\" }
",
        )
        .expect("spec v2 loads")
    }

    /// A canonical v0-shaped spec carrying a single `derived-from` ref value.
    fn spec_v0_doc() -> String {
        let inst = Instance {
            title: "Checkout spec".to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![Field {
                        key: "derived-from".to_string(),
                        value: Value::Scalar("prd:checkout".to_string()),
                    }],
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    slot: Some("The checkout flow rejects duplicate orders.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&spec_v1(), &inst)
    }

    #[test]
    fn widened_cardinality_e2e_is_a_byte_identity_transform() {
        let v1 = spec_v1();
        let v2 = spec_v2();
        let src = spec_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::WidenedCardinality {
                section: "meta".to_string(),
                field: "derived-from".to_string()
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("transform succeeds");

        // Instance-byte identity: the v0 value stays valid under the widened card.
        assert_eq!(
            out, src,
            "widened-cardinality leaves the instance bytes unchanged"
        );

        // (c) conforms against v2, (a) round-trips byte-identical, (b) the value survives.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);
        let out_inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let derived = out_inst
            .sections
            .iter()
            .find(|s| s.id == "meta")
            .and_then(|s| s.fields.iter().find(|f| f.key == "derived-from"))
            .map(|f| f.value.clone());
        assert_eq!(derived, Some(Value::Scalar("prd:checkout".to_string())));

        // (d) determinism.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(again, out, "transform is deterministic");
    }

    // ---- (c) the Framing-A prose-routing branch (a new required section slot) ----

    /// v1: a `note` with a leading `vision` slot and a trailing `success` slot.
    fn note_v1() -> Schema {
        load_schema(
            b"\
type: note
sections:
  - id: vision
    slot: { hint: \"the vision\" }
  - id: success
    slot: { hint: \"success criteria\" }
",
        )
        .expect("note v1 loads")
    }

    /// v2: a **new required `rationale` slot** is introduced between the two — the
    /// prose-needing change (no deterministic default, so the CLI cannot fill it).
    fn note_v2() -> Schema {
        load_schema(
            b"\
type: note
sections:
  - id: vision
    slot: { hint: \"the vision\" }
  - id: rationale
    slot: { hint: \"why this decision\" }
  - id: success
    slot: { hint: \"success criteria\" }
",
        )
        .expect("note v2 loads")
    }

    /// A canonical v0/v1-shaped note (built through [`render`] so the input is the exact
    /// byte-stable form a first-touch-canonicalized corpus doc has).
    fn note_v0_doc() -> String {
        let inst = Instance {
            title: "Checkout note".to_string(),
            sections: vec![
                SectionContent {
                    id: "vision".to_string(),
                    slot: Some("A frictionless checkout.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "success".to_string(),
                    slot: Some("Cart abandonment drops 20%.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&note_v1(), &inst)
    }

    #[test]
    fn prose_needing_required_slot_mints_empty_then_blocks_until_authored() {
        let v1 = note_v1();
        let v2 = note_v2();
        let src = note_v0_doc();

        // The real classifier emits the prose-needing change; the driver is exercised on
        // the emitted classification, not a hand-built list.
        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::ProseNeeding {
                section: "rationale".to_string(),
                leaf: None,
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("transform mints the empty slot");

        // Framing A: the output carries the new required slot **empty**, and the reused
        // conformance gate (`schema_conformance`) blocks on it — the only finding is the
        // `required-slot-present` break on the minted slot (blocks until authored).
        let doc = parse_sections(&v2, &out).expect("minted doc parses under v2");
        let findings = schema_conformance(&v2, &out, &doc);
        let codes: Vec<&str> = findings.iter().map(|f| f.code.as_str()).collect();
        assert_eq!(
            codes,
            vec!["schema-conformance.required-slot-present"],
            "the only block is the empty minted rationale slot; got {findings:?}"
        );

        // No-data-loss: the pre-existing vision/success slots survive verbatim.
        let minted = instance_from_source(&v2, &out).expect("v2 re-parse");
        let slot_of = |inst: &Instance, id: &str| {
            inst.sections
                .iter()
                .find(|s| s.id == id)
                .and_then(|s| s.slot.clone())
        };
        assert_eq!(
            slot_of(&minted, "vision").as_deref(),
            Some("A frictionless checkout.")
        );
        assert_eq!(
            slot_of(&minted, "success").as_deref(),
            Some("Cart abandonment drops 20%.")
        );

        // Determinism: the same prose-needing diff over the same source mints identically.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run mints");
        assert_eq!(again, out, "prose-routing mint is deterministic");

        // The agent authors the slot via `set_slot` — the CLI places the prose it is
        // handed (the determinism boundary: the CLI never invents it).
        let authored = crate::write::set_slot(
            &v2,
            &out,
            "rationale",
            "Single-click checkout: the fewest steps win.",
        )
        .expect("set_slot fills the minted slot");

        // The **same** gate now passes clean, and the authored doc round-trips byte-stable.
        assert_conforms(&v2, &authored);
        assert_byte_stable(&v2, &authored);
        let authored_inst = instance_from_source(&v2, &authored).expect("authored re-parse");
        assert_eq!(
            slot_of(&authored_inst, "rationale").as_deref(),
            Some("Single-click checkout: the fewest steps win.")
        );
    }

    // ---- (d) the per-doc-gated corpus fold: atomicity / WIP-safety ----

    /// `true` iff `source` independently re-detects as **conformant** against `schema`
    /// (the Inc-1 fifth-family detector's predicate: a parse-level failure *or* any
    /// conformance finding ⇒ non-conformant ⇒ routed `migrate`).
    fn detect_conformant(schema: &Schema, source: &str) -> bool {
        match parse_sections(schema, source) {
            Ok(doc) => schema_conformance(schema, source, &doc).is_empty(),
            Err(_) => false,
        }
    }

    #[test]
    fn corpus_halts_cleanly_at_the_prose_needing_doc_no_half_transform() {
        let prd_v1 = prd_v1();
        let prd_v2 = prd_v2();
        let spec_v1 = spec_v1();
        let spec_v2 = spec_v2();
        let note_v1 = note_v1();
        let note_v2 = note_v2();

        // Heterogeneous corpus: each doc owns its schema pair + the real classifier's diff.
        let prd_diff = schema_diff(&prd_v1, &prd_v2);
        let spec_diff = schema_diff(&spec_v1, &spec_v2);
        let note_diff = schema_diff(&note_v1, &note_v2);
        // The note diff really is the prose-needing change that blocks (not assumed).
        assert_eq!(
            note_diff,
            vec![SchemaChange::ProseNeeding {
                section: "rationale".to_string(),
                leaf: None,
            }]
        );

        let (prd_a_src, _) = prd_v0_doc();
        let spec_a_src = spec_v0_doc();
        let note_a_src = note_v0_doc();
        let (prd_b_src, _) = prd_v0_doc();

        // N = 4; instance k = 2 (the note) is prose-needing (blocked). prd-b after it is a
        // perfectly migratable structural doc — it must STILL be left untouched (the run
        // halts at the first blocker; it does not skip-and-continue).
        let corpus = [
            CorpusDoc {
                id: "prd-a",
                old_schema: &prd_v1,
                new_schema: &prd_v2,
                source: &prd_a_src,
                changes: &prd_diff,
            },
            CorpusDoc {
                id: "spec-a",
                old_schema: &spec_v1,
                new_schema: &spec_v2,
                source: &spec_a_src,
                changes: &spec_diff,
            },
            CorpusDoc {
                id: "note-a",
                old_schema: &note_v1,
                new_schema: &note_v2,
                source: &note_a_src,
                changes: &note_diff,
            },
            CorpusDoc {
                id: "prd-b",
                old_schema: &prd_v1,
                new_schema: &prd_v2,
                source: &prd_b_src,
                changes: &prd_diff,
            },
        ];

        let result = migrate_corpus(&corpus);

        // The run halts at the prose-needing doc (k = 2).
        assert_eq!(result.halted_at, Some(2));
        assert_eq!(result.docs.len(), 4);

        // Docs BEFORE k are fully transformed-and-conformant (v2): each is Migrated, its
        // bytes conform under its v2 schema, round-trip byte-stable, and re-detect as
        // conformant-v2 (so no doc is left half-transformed).
        match &result.docs[0] {
            DocOutcome::Migrated { id, v2 } => {
                assert_eq!(id, "prd-a");
                assert_conforms(&prd_v2, v2);
                assert_byte_stable(&prd_v2, v2);
                assert!(detect_conformant(&prd_v2, v2));
                assert_ne!(v2, &prd_a_src, "prd-a actually changed (slot→repeatable)");
            }
            other => panic!("prd-a must be Migrated; got {other:?}"),
        }
        match &result.docs[1] {
            DocOutcome::Migrated { id, v2 } => {
                assert_eq!(id, "spec-a");
                assert_conforms(&spec_v2, v2);
                assert_byte_stable(&spec_v2, v2);
                assert!(detect_conformant(&spec_v2, v2));
                // widened-cardinality is a byte-identity transform, so the bytes match v0
                // yet the doc is now conformant against v2 — it is genuinely "migrated".
                assert_eq!(v2, &spec_a_src);
            }
            other => panic!("spec-a must be Migrated; got {other:?}"),
        }

        // Docs AT/AFTER k are byte-identical v0 (untouched) and re-detect as un-migrated
        // (non-conformant against v2 ⇒ the detector routes them `migrate`).
        match &result.docs[2] {
            DocOutcome::Untouched { id, v0 } => {
                assert_eq!(id, "note-a");
                assert_eq!(v0, &note_a_src, "the blocked doc is byte-identical v0");
                assert!(
                    !detect_conformant(&note_v2, v0),
                    "the blocked doc re-detects as un-migrated-v0"
                );
            }
            other => panic!("note-a must be Untouched; got {other:?}"),
        }
        match &result.docs[3] {
            DocOutcome::Untouched { id, v0 } => {
                assert_eq!(id, "prd-b");
                assert_eq!(
                    v0, &prd_b_src,
                    "a migratable doc AFTER the halt is still untouched v0"
                );
                // Atomicity proven by **bytes**: prd-b was independently migratable, yet
                // it is byte-identical to its un-migrated form — it differs from what the
                // transform WOULD have produced. (A fixed-slot→repeatable v0 happens to
                // also satisfy the v2 repeatable shape, so conformance alone cannot flag
                // it — exactly the Inc-2 limitation the Inc-3 schema-version stamp
                // resolves; here the halt-and-stop atomicity rests on the byte-identity.)
                let would_be = try_migrate_doc(&corpus[3]).expect("prd-b is migratable");
                assert_ne!(v0, &would_be, "prd-b was left un-migrated, not transformed");
                // It still re-detects cleanly under v2 (a definite verdict, never a
                // half-parsed state) — the "independently re-detectable" property.
                assert!(parse_sections(&prd_v2, v0).is_ok());
            }
            other => panic!("prd-b must be Untouched; got {other:?}"),
        }

        // Determinism: the same corpus folds to byte-identical outcomes.
        let again = migrate_corpus(&corpus);
        assert_eq!(again, result, "the corpus fold is deterministic");
    }

    #[test]
    fn corpus_with_no_blocker_migrates_every_doc() {
        let prd_v1 = prd_v1();
        let prd_v2 = prd_v2();
        let spec_v1 = spec_v1();
        let spec_v2 = spec_v2();
        let prd_diff = schema_diff(&prd_v1, &prd_v2);
        let spec_diff = schema_diff(&spec_v1, &spec_v2);
        let (prd_src, _) = prd_v0_doc();
        let spec_src = spec_v0_doc();

        let corpus = [
            CorpusDoc {
                id: "prd-a",
                old_schema: &prd_v1,
                new_schema: &prd_v2,
                source: &prd_src,
                changes: &prd_diff,
            },
            CorpusDoc {
                id: "spec-a",
                old_schema: &spec_v1,
                new_schema: &spec_v2,
                source: &spec_src,
                changes: &spec_diff,
            },
        ];

        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, None, "no blocker ⇒ no halt");
        assert!(
            result
                .docs
                .iter()
                .all(|d| matches!(d, DocOutcome::Migrated { .. })),
            "every doc migrates when none blocks"
        );
    }

    // ---- (e) the added-optional-field branch: a defaulted header field, both shapes ----

    /// Shape A — a doctype that **already carries** a `---` header (a `meta` header
    /// with a `derived-from` ref, the spec shape M33 shipped). v1 has only the ref.
    fn doca_v1() -> Schema {
        load_schema(
            b"\
type: doca
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: doca, card: \"0..1\" }
  - id: body
    slot: { hint: \"the body\" }
",
        )
        .expect("doca v1 loads")
    }

    /// v2 adds a **defaulted** `schema-version` field after `derived-from` — the
    /// stamp-shaped add-field whose value comes from the schema `default` (the
    /// caller-supplied `set`-derived value is M34 Inc-3 T4's concern).
    fn doca_v2() -> Schema {
        load_schema(
            b"\
type: doca
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: doca, card: \"0..1\" }
      - { id: schema-version, type: string, default: \"1\" }
  - id: body
    slot: { hint: \"the body\" }
",
        )
        .expect("doca v2 loads")
    }

    /// A canonical v0-shaped `doca` carrying a `derived-from` ref + body prose.
    fn doca_v0_doc() -> String {
        let inst = Instance {
            title: "A doc".to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![Field {
                        key: "derived-from".to_string(),
                        value: Value::Scalar("doca:other".to_string()),
                    }],
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    slot: Some("The body prose, unchanged by the add-field.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&doca_v1(), &inst)
    }

    #[test]
    fn added_field_into_an_existing_header_is_byte_stable_and_preserves_values() {
        let v1 = doca_v1();
        let v2 = doca_v2();
        let src = doca_v0_doc();

        // The real classifier emits the add-field; the driver is exercised on the
        // emitted classification, not a hand-built list.
        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedOptionalField {
                section: "meta".to_string(),
                field: "schema-version".to_string(),
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("add-field transform succeeds");

        // (c) conforms against v2, (a) round-trips byte-identical.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // (b) the new field is present with its default value, at its schema-ordered
        // home (after `derived-from`); every prior field/slot value survives.
        let inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let meta = inst
            .sections
            .iter()
            .find(|s| s.id == "meta")
            .expect("meta present");
        assert_eq!(
            meta.fields
                .iter()
                .map(|f| f.key.as_str())
                .collect::<Vec<_>>(),
            vec!["derived-from", "schema-version"],
            "the new field slots in at its schema-ordered position"
        );
        let field_val = |id: &str| {
            meta.fields
                .iter()
                .find(|f| f.key == id)
                .map(|f| f.value.clone())
        };
        assert_eq!(
            field_val("schema-version"),
            Some(Value::Scalar("1".to_string()))
        );
        assert_eq!(
            field_val("derived-from"),
            Some(Value::Scalar("doca:other".to_string())),
            "the prior ref value survives"
        );
        let body = inst
            .sections
            .iter()
            .find(|s| s.id == "body")
            .and_then(|s| s.slot.clone());
        assert_eq!(
            body.as_deref(),
            Some("The body prose, unchanged by the add-field.")
        );

        // (d) determinism.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(again, out, "add-field transform is deterministic");
    }

    /// Shape B — a **header-less** doctype: v1 renders zero front-matter (the `prd` /
    /// `changelog` shape), so the stamp **introduces a `---` block that never existed**.
    fn docb_v1() -> Schema {
        load_schema(
            b"\
type: docb
sections:
  - id: vision
    slot: { hint: \"the vision\" }
  - id: success
    slot: { hint: \"success criteria\" }
",
        )
        .expect("docb v1 loads")
    }

    /// v2 prepends a `meta` header carrying the defaulted `schema-version` field — a
    /// real v0→v1 shape change that brings the `---` block into being.
    fn docb_v2() -> Schema {
        load_schema(
            b"\
type: docb
sections:
  - id: meta
    header: true
    fields:
      - { id: schema-version, type: string, default: \"1\" }
  - id: vision
    slot: { hint: \"the vision\" }
  - id: success
    slot: { hint: \"success criteria\" }
",
        )
        .expect("docb v2 loads")
    }

    /// A canonical v0-shaped (header-less) `docb`.
    fn docb_v0_doc() -> String {
        let inst = Instance {
            title: "B doc".to_string(),
            sections: vec![
                SectionContent {
                    id: "vision".to_string(),
                    slot: Some("A frictionless flow.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "success".to_string(),
                    slot: Some("Abandonment drops 20%.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&docb_v1(), &inst)
    }

    #[test]
    fn added_field_into_a_header_less_doc_introduces_the_fence_byte_stably() {
        let v1 = docb_v1();
        let v2 = docb_v2();
        let src = docb_v0_doc();

        // Precondition: the v0 doc carries no front-matter fence at all.
        assert!(
            !src.starts_with("---\n"),
            "the v0 header-less doc has no `---` block"
        );

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedOptionalField {
                section: "meta".to_string(),
                field: "schema-version".to_string(),
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("add-field transform succeeds");

        // The `---` block is introduced, carrying the defaulted field.
        assert!(
            out.starts_with("---\nschema-version: 1\n---\n\n"),
            "the fence is introduced ahead of the body; got {out:?}"
        );

        // (c) conforms against v2, (a) round-trips byte-identical.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // (b) the body slots survive verbatim; the new field carries its default.
        let inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let slot_of = |id: &str| {
            inst.sections
                .iter()
                .find(|s| s.id == id)
                .and_then(|s| s.slot.clone())
        };
        assert_eq!(slot_of("vision").as_deref(), Some("A frictionless flow."));
        assert_eq!(
            slot_of("success").as_deref(),
            Some("Abandonment drops 20%.")
        );
        let stamp = inst
            .sections
            .iter()
            .find(|s| s.id == "meta")
            .and_then(|s| s.fields.iter().find(|f| f.key == "schema-version"))
            .map(|f| f.value.clone());
        assert_eq!(stamp, Some(Value::Scalar("1".to_string())));

        // (d) determinism.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(again, out, "fence-introducing add-field is deterministic");
    }

    /// An added field that is **`optional:` with no default** has no deterministic
    /// value to place: its absence stays conformant, so the transform is a byte
    /// **no-op** (the design table's "add optional field" with nothing to add — the
    /// widened-cardinality sibling). It must not error, and must not invent a value.
    #[test]
    fn added_optional_field_with_no_default_is_a_byte_no_op() {
        let v1 = doca_v1();
        let v2 = load_schema(
            b"\
type: doca
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: doca, card: \"0..1\" }
      - { id: link, type: string, optional: true }
  - id: body
    slot: { hint: \"the body\" }
",
        )
        .expect("doca v2-optional loads");
        let src = doca_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedOptionalField {
                section: "meta".to_string(),
                field: "link".to_string(),
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("optional add-field is a no-op");
        assert_eq!(out, src, "an optional field with no default adds no bytes");
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);
    }

    // ---- the deferred branches block, never silently drop ----

    /// The **prose-needing field** sub-case (a new required *field*, not a slot — T4
    /// mints only slots) is surfaced as [`TransformError::Unsupported`], not silently
    /// skipped — an un-built branch must block the migration. The `added-optional-field`
    /// branch is now built (M34 Inc-3 T1; see the e2e tests above), so it no longer
    /// asserts Unsupported here.
    #[test]
    fn deferred_kinds_surface_as_unsupported() {
        let v1 = prd_v1();
        let (src, _) = prd_v0_doc();
        let prose_needing_field = vec![SchemaChange::ProseNeeding {
            section: "vision".to_string(),
            leaf: Some("owner".to_string()),
        }];
        assert_eq!(
            transform(&v1, &v1, &src, &prose_needing_field),
            Err(TransformError::Unsupported {
                kind: "prose-needing",
                section: "vision".to_string()
            })
        );
    }
}
